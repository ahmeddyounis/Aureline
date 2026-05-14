#!/usr/bin/env bash
set -euo pipefail

python3 - "$@" <<'PY'
from __future__ import annotations

import argparse
import dataclasses
import datetime as dt
import json
import re
import sys
from collections import Counter
from pathlib import Path
from typing import Any

import yaml


DEFAULT_PACKET = "artifacts/release/protected_fitness_packet_alpha.yaml"
DEFAULT_REVIEW = "docs/release/protected_fitness_review_alpha.md"

RESULT_RANK = {
    "passing": 0,
    "warning": 1,
    "waived": 2,
    "provisional": 3,
    "evidence_stale": 3,
    "waiver_expired": 4,
    "blocked": 5,
}

ALLOWED_RESULTS = set(RESULT_RANK)
ALLOWED_WAIVER_STATES = {
    "no_active_waiver",
    "active_waiver",
    "expired_waiver",
    "threshold_provisional_pending_council",
}


@dataclasses.dataclass
class Finding:
    severity: str
    check_id: str
    message: str
    ref: str | None = None
    remediation: str | None = None

    def format(self) -> str:
        prefix = f"{self.severity.upper()} {self.check_id}: {self.message}"
        if self.ref:
            prefix += f" [{self.ref}]"
        if self.remediation:
            prefix += f"\n  remediation: {self.remediation}"
        return prefix


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser(description="Validate and render the protected fitness review packet.")
    parser.add_argument("--repo-root", default=".")
    parser.add_argument("--packet", default=DEFAULT_PACKET)
    parser.add_argument("--review", default=DEFAULT_REVIEW)
    parser.add_argument("--render-review", action="store_true", help="Rewrite the Markdown review packet from the YAML packet.")
    parser.add_argument("--skip-review-check", action="store_true", help="Do not compare the checked-in Markdown review packet.")
    return parser.parse_args()


def load_yaml(path: Path) -> Any:
    with path.open("r", encoding="utf-8") as handle:
        return yaml.safe_load(handle)


def load_json(path: Path) -> Any:
    with path.open("r", encoding="utf-8") as handle:
        return json.load(handle)


def strip_fragment(ref: str) -> str:
    return ref.split("#", 1)[0]


def resolve(repo_root: Path, ref: str) -> Path:
    return repo_root / strip_fragment(ref)


def ensure_path(repo_root: Path, ref: str, findings: list[Finding], check_id: str) -> None:
    if not ref or not resolve(repo_root, ref).exists():
        findings.append(
            Finding(
                "error",
                check_id,
                f"referenced artifact does not exist: {ref}",
                ref=ref,
                remediation="Seed the artifact or correct the packet reference.",
            )
        )


def parse_datetime(value: str, label: str, findings: list[Finding], ref: str | None = None) -> dt.datetime | None:
    if not isinstance(value, str) or not value:
        findings.append(Finding("error", f"{label}.missing", f"{label} must be a non-empty timestamp", ref=ref))
        return None
    try:
        parsed = dt.datetime.fromisoformat(value.replace("Z", "+00:00"))
    except ValueError:
        findings.append(Finding("error", f"{label}.invalid", f"{label} must be ISO-8601 UTC: {value}", ref=ref))
        return None
    if parsed.tzinfo is None:
        parsed = parsed.replace(tzinfo=dt.timezone.utc)
    return parsed.astimezone(dt.timezone.utc)


def parse_duration(value: str, label: str, findings: list[Finding], ref: str | None = None) -> dt.timedelta | None:
    if not isinstance(value, str):
        findings.append(Finding("error", f"{label}.missing", f"{label} must be an ISO duration", ref=ref))
        return None
    match = re.fullmatch(r"P(\d+)D", value)
    if not match:
        findings.append(
            Finding(
                "error",
                f"{label}.unsupported",
                f"{label} currently supports day durations like P14D, got {value}",
                ref=ref,
            )
        )
        return None
    return dt.timedelta(days=int(match.group(1)))


def iso_z(value: dt.datetime) -> str:
    return value.astimezone(dt.timezone.utc).replace(tzinfo=None).isoformat(timespec="seconds") + "Z"


def collect_catalog_ids(catalog: dict[str, Any]) -> set[str]:
    return {row["id"] for row in catalog.get("rows", []) if isinstance(row, dict) and row.get("id")}


def collect_gate_ids(gate: dict[str, Any]) -> set[str]:
    return {row["gate_row_id"] for row in gate.get("protected_metric_rows", []) if isinstance(row, dict) and row.get("gate_row_id")}


def dashboard_rows_by_gate(dashboard: dict[str, Any]) -> dict[str, list[dict[str, Any]]]:
    rows: dict[str, list[dict[str, Any]]] = {}
    for row in dashboard.get("rows", []):
        rows.setdefault(row.get("gate_row_id"), []).append(row)
    return rows


def derive_hot_path_result(statuses: list[str], waiver: dict[str, Any], as_of: dt.datetime) -> str:
    waiver_state = waiver.get("waiver_state")
    expiry_at = waiver.get("expiry_at")
    expiry = parse_datetime(expiry_at, "waiver.expiry_at", [], None) if expiry_at else None

    if not statuses:
        return "evidence_stale"
    if "fail" in statuses:
        if waiver_state == "active_waiver" and expiry and expiry > as_of:
            return "waived"
        if waiver_state in {"active_waiver", "expired_waiver"} and expiry and expiry <= as_of:
            return "waiver_expired"
        return "blocked"
    if "waived" in statuses:
        return "waived"
    if "expired_waiver" in statuses:
        return "waiver_expired"
    if "missing_observation" in statuses:
        return "evidence_stale"
    if "pending_scenario_seed" in statuses:
        return "provisional"
    if "warn" in statuses:
        return "warning"
    if all(status == "pass" for status in statuses):
        return "passing"
    return "evidence_stale"


def expected_result_for_row(
    row: dict[str, Any],
    dashboard_by_gate: dict[str, list[dict[str, Any]]],
    support_scorecard: dict[str, Any],
    as_of: dt.datetime,
    findings: list[Finding],
) -> str:
    source = row.get("result_source", {})
    kind = source.get("kind")
    if kind == "hot_path_dashboard":
        statuses: list[str] = []
        for gate_ref in source.get("gate_row_refs", []):
            gate_rows = dashboard_by_gate.get(gate_ref, [])
            if not gate_rows:
                findings.append(
                    Finding(
                        "error",
                        "protected_function_rows.gate_row_missing_dashboard",
                        f"dashboard has no rows for gate row {gate_ref}",
                        ref=row.get("protected_function_ref"),
                    )
                )
            statuses.extend(str(dash_row.get("regression_status")) for dash_row in gate_rows)
        return derive_hot_path_result(statuses, row.get("waiver", {}), as_of)

    if kind == "support_scorecard":
        expected_status = source.get("expected_scorecard_status")
        actual_status = support_scorecard.get("status")
        if expected_status and actual_status != expected_status:
            findings.append(
                Finding(
                    "error",
                    "support_scorecard.status_mismatch",
                    f"packet expected support scorecard status {expected_status}, got {actual_status}",
                    ref=row.get("protected_function_ref"),
                )
            )
        scenario_rows = support_scorecard.get("scenario_rows", [])
        expected_count = source.get("scenario_family_count")
        if expected_count is not None and len(scenario_rows) != expected_count:
            findings.append(
                Finding(
                    "error",
                    "support_scorecard.scenario_count_mismatch",
                    f"packet expected {expected_count} support scenarios, got {len(scenario_rows)}",
                    ref=row.get("protected_function_ref"),
                )
            )
        return "evidence_stale" if actual_status == "seeded_pending_measurement" else "passing"

    findings.append(
        Finding(
            "error",
            "protected_function_rows.result_source.unknown_kind",
            f"unknown result source kind: {kind}",
            ref=row.get("protected_function_ref"),
        )
    )
    return "blocked"


def validate_evidence_dates(row: dict[str, Any], as_of: dt.datetime, findings: list[Finding]) -> None:
    ref = row.get("protected_function_ref")
    evidence = row.get("evidence", {})
    captured_at = parse_datetime(evidence.get("captured_at"), "evidence.captured_at", findings, ref)
    stale_after = parse_duration(evidence.get("stale_after"), "evidence.stale_after", findings, ref)
    expires_at = parse_datetime(evidence.get("expires_at"), "evidence.expires_at", findings, ref)
    if captured_at and stale_after and expires_at:
        computed = captured_at + stale_after
        if abs((computed - expires_at).total_seconds()) > 1:
            findings.append(
                Finding(
                    "error",
                    "protected_function_rows.evidence.expires_at_mismatch",
                    f"expires_at should be {iso_z(computed)} based on captured_at + stale_after",
                    ref=ref,
                )
            )
        if as_of > expires_at and row.get("current_result") == "passing":
            findings.append(
                Finding(
                    "error",
                    "protected_function_rows.stale_evidence_overclaims",
                    "row renders passing even though evidence is past expires_at",
                    ref=ref,
                    remediation="Move the row to evidence_stale or refresh the evidence.",
                )
            )


def validate_waiver(row: dict[str, Any], as_of: dt.datetime, findings: list[Finding]) -> None:
    ref = row.get("protected_function_ref")
    waiver = row.get("waiver", {})
    state = waiver.get("waiver_state")
    if state not in ALLOWED_WAIVER_STATES:
        findings.append(Finding("error", "protected_function_rows.waiver.unknown_state", f"unknown waiver state {state}", ref=ref))
        return
    expiry_at = waiver.get("expiry_at")
    expiry = parse_datetime(expiry_at, "waiver.expiry_at", findings, ref) if expiry_at else None
    result = row.get("current_result")

    if state == "no_active_waiver":
        if waiver.get("waiver_record_ref") is not None or expiry_at is not None:
            findings.append(
                Finding(
                    "error",
                    "protected_function_rows.waiver.no_active_has_ref_or_expiry",
                    "no_active_waiver rows must keep waiver_record_ref and expiry_at null",
                    ref=ref,
                )
            )
    elif state == "active_waiver":
        if not waiver.get("waiver_record_ref") or not expiry:
            findings.append(
                Finding(
                    "error",
                    "protected_function_rows.waiver.active_missing_fields",
                    "active waivers must carry waiver_record_ref and future expiry_at",
                    ref=ref,
                )
            )
        elif expiry <= as_of:
            findings.append(
                Finding(
                    "error",
                    "protected_function_rows.waiver.active_expired",
                    "active waiver expiry_at is not in the future",
                    ref=ref,
                    remediation="Move the row to waiver_expired or renew the waiver with a new expiry.",
                )
            )
        if result == "passing":
            findings.append(
                Finding(
                    "error",
                    "protected_function_rows.waiver.active_overclaims",
                    "a row with an active waiver must not render as passing",
                    ref=ref,
                )
            )
    elif state == "expired_waiver":
        if not waiver.get("waiver_record_ref") or not expiry:
            findings.append(
                Finding(
                    "error",
                    "protected_function_rows.waiver.expired_missing_fields",
                    "expired waivers must carry waiver_record_ref and expiry_at",
                    ref=ref,
                )
            )
        elif expiry > as_of:
            findings.append(
                Finding(
                    "error",
                    "protected_function_rows.waiver.expired_in_future",
                    "expired_waiver expiry_at is still in the future",
                    ref=ref,
                )
            )
        if result in {"passing", "warning", "waived"}:
            findings.append(
                Finding(
                    "error",
                    "protected_function_rows.waiver.expired_overclaims",
                    "expired waivers must degrade instead of rendering passing, warning, or waived",
                    ref=ref,
                )
            )


def validate_rows(
    packet: dict[str, Any],
    catalog_ids: set[str],
    gate_ids: set[str],
    dashboard: dict[str, Any],
    support_scorecard: dict[str, Any],
    as_of: dt.datetime,
    findings: list[Finding],
) -> None:
    rows = packet.get("protected_function_rows", [])
    if not rows:
        findings.append(Finding("error", "protected_function_rows.empty", "packet must include protected function rows"))
        return

    dashboard_by_gate = dashboard_rows_by_gate(dashboard)
    seen: set[str] = set()
    for row in rows:
        ref = row.get("protected_function_ref")
        if not ref:
            findings.append(Finding("error", "protected_function_rows.missing_ref", "row is missing protected_function_ref"))
            continue
        if ref in seen:
            findings.append(Finding("error", "protected_function_rows.duplicate_ref", f"duplicate row {ref}", ref=ref))
        seen.add(ref)

        result = row.get("current_result")
        if result not in ALLOWED_RESULTS:
            findings.append(Finding("error", "protected_function_rows.unknown_result", f"unknown current_result {result}", ref=ref))

        catalog_ref = row.get("catalog_row_ref")
        if catalog_ref is not None and catalog_ref not in catalog_ids:
            findings.append(
                Finding(
                    "error",
                    "protected_function_rows.catalog_ref_missing",
                    f"catalog row does not exist: {catalog_ref}",
                    ref=ref,
                )
            )

        for field_name in ("owner_dri", "owning_lane", "waiver_authority_ref", "current_result_reason", "last_pass_summary"):
            if not row.get(field_name):
                findings.append(Finding("error", f"protected_function_rows.{field_name}.missing", f"{field_name} is required", ref=ref))

        if result == "passing" and not row.get("last_passed_at"):
            findings.append(
                Finding(
                    "error",
                    "protected_function_rows.last_passed_at.missing_for_pass",
                    "passing rows must include last_passed_at",
                    ref=ref,
                )
            )
        if row.get("last_passed_at"):
            parse_datetime(row.get("last_passed_at"), "protected_function_rows.last_passed_at", findings, ref)

        source = row.get("result_source", {})
        if source.get("kind") == "hot_path_dashboard":
            for gate_ref in source.get("gate_row_refs", []):
                if gate_ref not in gate_ids:
                    findings.append(
                        Finding(
                            "error",
                            "protected_function_rows.gate_ref_missing",
                            f"gate row does not exist in ci/perf/nightly_hot_path.yml: {gate_ref}",
                            ref=ref,
                        )
                    )

        expected = expected_result_for_row(row, dashboard_by_gate, support_scorecard, as_of, findings)
        explicit_expected = source.get("expected_ci_result") or source.get("expected_packet_result")
        if explicit_expected and explicit_expected != expected:
            findings.append(
                Finding(
                    "error",
                    "protected_function_rows.expected_result_mismatch",
                    f"packet expected {explicit_expected}, but checked-in inputs derive {expected}",
                    ref=ref,
                )
            )
        if result in ALLOWED_RESULTS and RESULT_RANK[result] < RESULT_RANK[expected]:
            findings.append(
                Finding(
                    "error",
                    "protected_function_rows.result_overclaims_inputs",
                    f"row renders {result}, but checked-in inputs require at least {expected}",
                    ref=ref,
                    remediation="Refresh the upstream evidence or degrade the packet row.",
                )
            )

        validate_evidence_dates(row, as_of, findings)
        validate_waiver(row, as_of, findings)


def validate_fixture_coverage(repo_root: Path, packet: dict[str, Any], findings: list[Finding]) -> None:
    required_states = {"current_pass", "stale_evidence_degrades", "active_waiver_visible", "expired_waiver_degrades"}
    seen_states: set[str] = set()
    for fixture in packet.get("protected_fixture_coverage", []):
        fixture_ref = fixture.get("fixture_ref")
        expected = fixture.get("expected_current_result")
        exercises = fixture.get("exercises_state")
        seen_states.add(exercises)
        ensure_path(repo_root, fixture_ref, findings, "protected_fixture_coverage.fixture_ref_missing")
        path = resolve(repo_root, fixture_ref)
        if path.exists():
            payload = load_yaml(path)
            actual = payload.get("tile_state")
            if actual != expected:
                findings.append(
                    Finding(
                        "error",
                        "protected_fixture_coverage.expected_result_mismatch",
                        f"fixture {fixture_ref} has tile_state {actual}, expected {expected}",
                        ref=fixture_ref,
                    )
                )
    missing = required_states - seen_states
    if missing:
        findings.append(
            Finding(
                "error",
                "protected_fixture_coverage.missing_required_states",
                f"missing fixture coverage states: {', '.join(sorted(missing))}",
                remediation="Add fixture rows for current pass, stale evidence, active waiver, and expired waiver behavior.",
            )
        )


def validate_refs(repo_root: Path, packet: dict[str, Any], findings: list[Finding]) -> None:
    for block_name in ("source_contract_refs",):
        block = packet.get(block_name, {})
        for key, ref in block.items():
            ensure_path(repo_root, ref, findings, f"{block_name}.{key}.missing")
    generation = packet.get("generation", {})
    for key in ("validator_command",):
        ref = generation.get(key)
        if ref:
            ensure_path(repo_root, ref.split()[0], findings, f"generation.{key}.missing")


def render_review(packet: dict[str, Any]) -> str:
    rows = packet.get("protected_function_rows", [])
    lines: list[str] = []
    lines.append("# Protected Fitness Review Packet")
    lines.append("")
    lines.append("<!-- Generated by ci/release/protected_fitness_packet_check.sh --render-review. -->")
    lines.append("")
    lines.append("## Header")
    lines.append("")
    lines.append(f"- Packet: `{packet['packet_id']}`")
    lines.append(f"- Packet state: `{packet['packet_state']}`")
    lines.append(f"- As of: `{packet['as_of']}`")
    lines.append(f"- Owner: `{packet['owner_dri']}`")
    lines.append(f"- Overall result: `{packet['overall_result']}`")
    lines.append(f"- Exact-build identity: `{packet['review_context']['exact_build_identity_ref']}`")
    lines.append(f"- Validator: `{packet['generation']['validator_command']}`")
    lines.append("")
    lines.append("## Summary")
    lines.append("")
    lines.append(packet["overall_summary"].strip())
    lines.append("")
    lines.append("## Protected Function Rows")
    lines.append("")
    lines.append("| Protected function | Result | Owner | Last pass | Waiver | Evidence expiry | Source |")
    lines.append("|---|---|---|---|---|---|---|")
    for row in rows:
        waiver = row["waiver"]
        evidence = row["evidence"]
        source = row["result_source"]
        source_ref = source.get("source_ref", "")
        last_pass = row.get("last_passed_at") or "none"
        waiver_label = waiver["waiver_state"]
        if waiver.get("expiry_at"):
            waiver_label += f" until {waiver['expiry_at']}"
        lines.append(
            "| `{}` | `{}` | `{}` | `{}` | `{}` | `{}` | `{}` |".format(
                row["protected_function_ref"],
                row["current_result"],
                row["owner_dri"],
                last_pass,
                waiver_label,
                evidence["expires_at"],
                source_ref,
            )
        )
    lines.append("")
    lines.append("## Degrade Notes")
    lines.append("")
    for row in rows:
        if row["current_result"] != "passing":
            lines.append(f"- `{row['protected_function_ref']}`: {row['current_result_reason'].strip()}")
    lines.append("")
    lines.append("## Waiver And Expiry Review")
    lines.append("")
    lines.append("| Protected function | Waiver state | Waiver ref | Waiver expiry | Summary |")
    lines.append("|---|---|---|---|---|")
    for row in rows:
        waiver = row["waiver"]
        lines.append(
            "| `{}` | `{}` | `{}` | `{}` | {} |".format(
                row["protected_function_ref"],
                waiver["waiver_state"],
                waiver.get("waiver_record_ref") or "none",
                waiver.get("expiry_at") or "none",
                waiver["summary"].strip(),
            )
        )
    lines.append("")
    lines.append("## Regression History")
    lines.append("")
    lines.append("| Protected function | Status counts | History source | Prior baseline summary |")
    lines.append("|---|---|---|---|")
    for row in rows:
        history = row["regression_history"]
        counts = ", ".join(f"{key}: {value}" for key, value in history.get("status_counts", {}).items())
        lines.append(
            "| `{}` | `{}` | `{}` | {} |".format(
                row["protected_function_ref"],
                counts,
                history["history_source_ref"],
                history["prior_baseline_summary"].strip(),
            )
        )
    lines.append("")
    lines.append("## Verification")
    lines.append("")
    lines.append("```sh")
    lines.append(packet["generation"]["validator_command"])
    lines.append("```")
    lines.append("")
    return "\n".join(lines)


def validate_review(repo_root: Path, review_rel: str, expected: str, render: bool, skip: bool, findings: list[Finding]) -> None:
    review_path = repo_root / review_rel
    if render:
        review_path.parent.mkdir(parents=True, exist_ok=True)
        review_path.write_text(expected, encoding="utf-8")
        return
    if skip:
        return
    if not review_path.exists():
        findings.append(
            Finding(
                "error",
                "review_packet.missing",
                f"review packet is missing: {review_rel}",
                remediation="Run ci/release/protected_fitness_packet_check.sh --render-review.",
            )
        )
        return
    actual = review_path.read_text(encoding="utf-8")
    if actual != expected:
        findings.append(
            Finding(
                "error",
                "review_packet.out_of_date",
                "review packet does not match the generated packet projection",
                ref=review_rel,
                remediation="Run ci/release/protected_fitness_packet_check.sh --render-review.",
            )
        )


def validate_overall(packet: dict[str, Any], findings: list[Finding]) -> None:
    overall = packet.get("overall_result")
    if overall not in ALLOWED_RESULTS:
        findings.append(Finding("error", "overall_result.unknown", f"unknown overall result {overall}"))
        return
    non_passing = [row["protected_function_ref"] for row in packet.get("protected_function_rows", []) if row.get("current_result") != "passing"]
    if non_passing and overall == "passing":
        findings.append(
            Finding(
                "error",
                "overall_result.overclaims_rows",
                "overall_result is passing while one or more protected function rows are degraded",
                remediation="Keep overall_result degraded until every protected row is passing or explicitly waived.",
            )
        )


def main() -> int:
    args = parse_args()
    repo_root = Path(args.repo_root).resolve()
    packet_path = repo_root / args.packet
    findings: list[Finding] = []

    if not packet_path.exists():
        print(f"missing packet: {args.packet}", file=sys.stderr)
        return 2

    packet = load_yaml(packet_path)
    as_of = parse_datetime(packet.get("as_of"), "packet.as_of", findings, args.packet) or dt.datetime.now(dt.timezone.utc)

    refs = packet.get("source_contract_refs", {})
    catalog = load_yaml(resolve(repo_root, refs["fitness_function_catalog_ref"]))
    gate = load_yaml(resolve(repo_root, refs["nightly_hot_path_gate_ref"]))
    dashboard = load_json(resolve(repo_root, refs["nightly_hot_path_dashboard_ref"]))
    support_scorecard = load_yaml(resolve(repo_root, refs["support_scorecard_ref"]))

    validate_refs(repo_root, packet, findings)
    validate_rows(
        packet,
        collect_catalog_ids(catalog),
        collect_gate_ids(gate),
        dashboard,
        support_scorecard,
        as_of,
        findings,
    )
    validate_fixture_coverage(repo_root, packet, findings)
    validate_overall(packet, findings)

    expected_review = render_review(packet)
    validate_review(repo_root, args.review, expected_review, args.render_review, args.skip_review_check, findings)

    errors = [finding for finding in findings if finding.severity == "error"]
    if errors:
        for finding in findings:
            print(finding.format(), file=sys.stderr)
        return 1

    row_counts = Counter(row["current_result"] for row in packet.get("protected_function_rows", []))
    print("protected fitness packet check: PASS")
    print("packet:", args.packet)
    print("review:", args.review)
    print("rows:", ", ".join(f"{key}={value}" for key, value in sorted(row_counts.items())))
    if args.render_review:
        print("rendered review packet")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
PY
