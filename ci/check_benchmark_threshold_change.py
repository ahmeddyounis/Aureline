#!/usr/bin/env python3
"""Enforce the typed threshold-change workflow for protected benchmark metrics.

This gate makes protected-threshold easings reviewable, evidence-backed,
time-bounded, and promotion-blocking instead of implicit. It validates the
checked-in threshold-change ledger against its boundary schema, resolves every
change record to a protected metric in the governance matrix, requires the
performance-owner and architecture-board approvals before a protected bar may
loosen, requires a granting authority's approval and an expiry on every
non-default waiver, fails promotion when an open in-force waiver is past its
expiry, keeps the in-force change record consistent with the matrix's threshold
state and waiver binding, surfaces the active waivers and expiry dates a shiproom
or release packet must show, and replays the workflow fixtures that prove each
fail-closed path.

Exit code is 0 when every check passes and 1 when any finding is at severity
``error``.
"""

from __future__ import annotations

import argparse
import datetime as dt
import json
import subprocess
import sys
from dataclasses import asdict, dataclass, field
from pathlib import Path
from typing import Any

try:
    from jsonschema import Draft202012Validator
except Exception as exc:  # pragma: no cover - dependency guard
    raise SystemExit(
        "python jsonschema is required: pip install jsonschema"
    ) from exc


LEDGER_REL = "artifacts/benchmarks/threshold-change-ledger.json"
SCHEMA_REL = "schemas/benchmarks/threshold-change-record.schema.json"
MATRIX_REL = "artifacts/benchmarks/m5-benchmark-governance.json"
FIXTURE_REGISTER_REL = "fixtures/benchmarks/threshold-change/manifest.yaml"

# Change kind -> the only resulting threshold state it may carry.
CHANGE_KIND_TO_STATE = {
    "set_calibrated": "frozen_calibrated",
    "tightened": "tightened",
    "eased_with_evidence": "eased_with_evidence",
    "provisional_hold": "provisional_uncalibrated",
    "recalibration_reset": "stale_recalibration_pending",
}

# An easing is the only change kind that may loosen the protected bar.
EASING_KIND = "eased_with_evidence"

# Approval groups required before a protected bar may loosen: one authority from
# each group must approve the easing.
EASING_APPROVAL_GROUPS = [
    {"performance_owner", "performance_council"},
    {"architecture_board", "architecture_council"},
]

# Approval groups required to grant each non-default waiver class; one authority
# from each group must appear in the change's approvals.
WAIVER_APPROVAL_GROUPS = {
    "performance_council_time_boxed": [{"performance_owner", "performance_council"}],
    "architecture_council_protected_path": [
        {"architecture_council", "architecture_board"},
        {"performance_council", "performance_owner"},
    ],
    "release_council_launch_scope": [{"release_council"}],
    "shiproom_executive_scope": [{"shiproom_executive_scope_review"}],
}

# Only an in-force record carries a live promotion obligation.
IN_FORCE_STATUS = "active"


@dataclass
class Finding:
    severity: str
    check_id: str
    message: str
    remediation: str
    ref: str | None = None
    details: dict[str, Any] = field(default_factory=dict)

    def as_report(self) -> dict[str, Any]:
        payload = asdict(self)
        if payload["ref"] is None:
            payload.pop("ref")
        if not payload["details"]:
            payload.pop("details")
        return payload


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--repo-root", default=".")
    parser.add_argument(
        "--today",
        default=None,
        help=(
            "Override the evaluation date (YYYY-MM-DD) for waiver-expiry "
            "narrowing. Defaults to the ledger generated_at date so the gate is "
            "deterministic."
        ),
    )
    parser.add_argument(
        "--report",
        default=None,
        help="Write the machine-readable JSON report to this repo-relative path.",
    )
    return parser.parse_args()


def add_finding(
    findings: list[Finding],
    check_id: str,
    message: str,
    remediation: str,
    *,
    ref: str | None = None,
    severity: str = "error",
    details: dict[str, Any] | None = None,
) -> None:
    findings.append(
        Finding(
            severity=severity,
            check_id=check_id,
            message=message,
            remediation=remediation,
            ref=ref,
            details=details or {},
        )
    )


def load_json(path: Path) -> Any:
    if not path.exists():
        raise SystemExit(f"missing JSON file: {path}")
    try:
        return json.loads(path.read_text(encoding="utf-8"))
    except json.JSONDecodeError as exc:
        raise SystemExit(f"invalid JSON at {path}: {exc}") from exc


def render_yaml_as_json(path: Path) -> Any:
    if not path.exists():
        raise SystemExit(f"missing YAML file: {path}")
    ruby = subprocess.run(
        [
            "ruby",
            "-rjson",
            "-ryaml",
            "-rdate",
            "-rtime",
            "-e",
            (
                "payload = YAML.safe_load(File.read(ARGV[0]), "
                "permitted_classes: [Date, Time, DateTime], aliases: false); "
                "STDOUT.write(JSON.generate(payload))"
            ),
            str(path),
        ],
        capture_output=True,
        text=True,
    )
    if ruby.returncode != 0:
        stderr = ruby.stderr.strip() or "unknown Ruby/Psych failure"
        raise SystemExit(f"failed to parse YAML at {path}: {stderr}")
    try:
        return json.loads(ruby.stdout)
    except json.JSONDecodeError as exc:
        raise SystemExit(f"Ruby/Psych emitted invalid JSON for {path}: {exc}") from exc


def parse_date(value: Any) -> dt.date | None:
    if not isinstance(value, str) or not value:
        return None
    try:
        return dt.date.fromisoformat(value)
    except ValueError:
        return None


# --------------------------------------------------------------------------- #
# Per-record admission rules (shared by the ledger and the fixture replay).
# --------------------------------------------------------------------------- #


def _approvals_authorities(record: dict[str, Any]) -> set[str]:
    return {
        row.get("authority")
        for row in record.get("approvals", [])
        if isinstance(row, dict) and isinstance(row.get("authority"), str)
    }


def _groups_satisfied(authorities: set[str], groups: list[set[str]]) -> bool:
    return all(bool(group & authorities) for group in groups)


def reject_change_record(
    record: dict[str, Any], metric_ids: set[str], today: dt.date
) -> str | None:
    """Return the rule id that rejects a single change record, or None.

    These are the per-record obligations that hold for any threshold-change
    record in isolation. Ledger-wide consistency (one in-force record per
    metric, agreement with the matrix) is checked separately.
    """
    metric_ref = record.get("metric_ref")
    if metric_ref not in metric_ids:
        return "metric_unresolved"

    change_kind = record.get("change_kind")
    expected_state = CHANGE_KIND_TO_STATE.get(change_kind)
    if expected_state is None:
        return "unknown_change_kind"
    if record.get("resulting_threshold_state") != expected_state:
        return "change_kind_state_mismatch"

    loosens = bool(record.get("loosens_protected_bar"))
    if loosens != (change_kind == EASING_KIND):
        return "loosen_flag_mismatch"

    authorities = _approvals_authorities(record)

    if change_kind == EASING_KIND:
        if not _groups_satisfied(authorities, EASING_APPROVAL_GROUPS):
            return "easing_missing_required_approval"
        if not record.get("release_evidence_ref"):
            return "easing_missing_release_evidence"

    # Before/after evidence ordering, when both sides are dated.
    evidence = record.get("before_after_evidence") or {}
    before_on = parse_date((evidence.get("before") or {}).get("captured_on"))
    after_on = parse_date((evidence.get("after") or {}).get("captured_on"))
    if before_on is not None and after_on is not None and before_on > after_on:
        return "before_after_evidence_inconsistent"

    waiver = record.get("waiver") or {}
    waiver_class = waiver.get("class")
    if waiver_class in (None, "none"):
        if (
            waiver.get("waiver_ref") is not None
            or waiver.get("granted_on") is not None
            or waiver.get("expires_on") is not None
        ):
            return "default_waiver_carries_grant"
    else:
        if not waiver.get("waiver_ref"):
            return "waiver_missing_ref"
        if not waiver.get("expires_on"):
            return "waiver_missing_expiry"
        groups = WAIVER_APPROVAL_GROUPS.get(waiver_class, [])
        if not _groups_satisfied(authorities, groups):
            return "waiver_missing_required_approval"
        # An open, in-force waiver past its expiry blocks promotion.
        expires = parse_date(waiver.get("expires_on"))
        if (
            record.get("status") == IN_FORCE_STATUS
            and expires is not None
            and expires < today
        ):
            return "expired_open_waiver_blocks_promotion"

    return None


# --------------------------------------------------------------------------- #
# Ledger validation.
# --------------------------------------------------------------------------- #


def validate_schema(
    repo_root: Path, ledger: dict[str, Any], findings: list[Finding]
) -> Draft202012Validator:
    validator = Draft202012Validator(load_json(repo_root / SCHEMA_REL))
    for error in sorted(validator.iter_errors(ledger), key=lambda e: list(e.path)):
        location = "/".join(str(p) for p in error.path) or "<root>"
        add_finding(
            findings,
            "ledger.schema",
            f"threshold-change ledger fails its schema at {location}: {error.message}",
            "Bring the ledger back into conformance with its boundary schema.",
            ref=LEDGER_REL,
        )
    return validator


def validate_source_refs(
    repo_root: Path, ledger: dict[str, Any], findings: list[Finding]
) -> None:
    source_refs = ledger.get("source_refs", [])
    for required in (SCHEMA_REL, MATRIX_REL):
        if required not in source_refs:
            add_finding(
                findings,
                "ledger.source_refs.missing_required",
                f"ledger source_refs must cite {required}",
                f"Add {required} to source_refs so the binding is explicit.",
                ref=LEDGER_REL,
            )
    for ref in source_refs:
        if isinstance(ref, str) and "/" in ref and not (repo_root / ref).exists():
            add_finding(
                findings,
                "ledger.source_refs.unresolved",
                f"ledger cites a missing source artifact: {ref}",
                "Seed the referenced artifact or correct the source ref.",
                ref=ref,
            )
    matrix_ref = ledger.get("matrix_ref")
    if isinstance(matrix_ref, str) and not (repo_root / matrix_ref).exists():
        add_finding(
            findings,
            "ledger.matrix_ref.unresolved",
            f"ledger cites a missing governance matrix: {matrix_ref}",
            "Point matrix_ref at the canonical benchmark-governance matrix.",
            ref=matrix_ref,
        )


def load_matrix_metrics(repo_root: Path) -> dict[str, dict[str, Any]]:
    matrix = load_json(repo_root / MATRIX_REL)
    return {
        row["metric_ref"]: row
        for row in matrix.get("protected_metrics", [])
        if isinstance(row, dict) and isinstance(row.get("metric_ref"), str)
    }


def validate_records(
    ledger: dict[str, Any],
    metrics: dict[str, dict[str, Any]],
    today: dt.date,
    findings: list[Finding],
) -> None:
    metric_ids = set(metrics)
    active_by_metric: dict[str, list[dict[str, Any]]] = {}

    for record in ledger.get("changes", []):
        ref = record.get("change_ref", "<unknown>")
        rejected_by = reject_change_record(record, metric_ids, today)
        if rejected_by is not None:
            add_finding(
                findings,
                f"change.{rejected_by}",
                f"threshold-change record {ref} is rejected by {rejected_by}",
                _remediation_for(rejected_by),
                ref=LEDGER_REL,
                details={"metric_ref": record.get("metric_ref")},
            )

        # Owner agreement with the matrix is advisory: surface drift, do not fail.
        metric = metrics.get(record.get("metric_ref"))
        if (
            metric is not None
            and record.get("owner_ref")
            and metric.get("owner_ref")
            and record["owner_ref"] != metric["owner_ref"]
        ):
            add_finding(
                findings,
                "change.owner_drift",
                (
                    f"threshold-change record {ref} owner {record['owner_ref']} "
                    f"differs from the matrix owner {metric['owner_ref']}"
                ),
                "Record the change under the metric's governing owner.",
                ref=LEDGER_REL,
                severity="warning",
            )

        if record.get("status") == IN_FORCE_STATUS:
            active_by_metric.setdefault(record.get("metric_ref"), []).append(record)

    _validate_active_consistency(metrics, active_by_metric, findings)


def _remediation_for(rejected_by: str) -> str:
    return {
        "metric_unresolved": "Bind every change record to a protected metric declared in the governance matrix.",
        "unknown_change_kind": "Use a declared change kind from the schema.",
        "change_kind_state_mismatch": "Set the resulting threshold state to the one the change kind implies.",
        "loosen_flag_mismatch": "Mark loosens_protected_bar true only for an easing, false otherwise.",
        "easing_missing_required_approval": "An easing needs both a performance-owner and an architecture-board approval before it lands.",
        "easing_missing_release_evidence": "Link the release-evidence packet that carries the easing.",
        "before_after_evidence_inconsistent": "The after capture must be no earlier than the before capture.",
        "default_waiver_carries_grant": "A class-none waiver carries no ref, grant date, or expiry.",
        "waiver_missing_ref": "A non-default waiver needs an owner-resolvable waiver_ref.",
        "waiver_missing_expiry": "Every non-default waiver must be time-boxed with an expiry.",
        "waiver_missing_required_approval": "Grant the waiver only with its granting authority's approval.",
        "expired_open_waiver_blocks_promotion": "Renew, close, or remediate the expired in-force waiver; promotion is blocked while it stays open.",
    }.get(rejected_by, "Correct the rejected change record.")


def _validate_active_consistency(
    metrics: dict[str, dict[str, Any]],
    active_by_metric: dict[str, list[dict[str, Any]]],
    findings: list[Finding],
) -> None:
    for metric_ref, metric in metrics.items():
        active = active_by_metric.get(metric_ref, [])
        if not active:
            add_finding(
                findings,
                "ledger.metric_missing_active_change",
                f"protected metric {metric_ref} has no in-force threshold-change record",
                "Record exactly one active threshold-change record per protected metric.",
                ref=LEDGER_REL,
            )
            continue
        if len(active) > 1:
            add_finding(
                findings,
                "ledger.multiple_active_changes",
                f"protected metric {metric_ref} has more than one in-force record",
                "Supersede prior records so exactly one stays active.",
                ref=LEDGER_REL,
                details={"change_refs": [r.get("change_ref") for r in active]},
            )
            continue

        record = active[0]
        if record.get("resulting_threshold_state") != metric.get("threshold_state"):
            add_finding(
                findings,
                "ledger.active_state_disagrees_with_matrix",
                (
                    f"metric {metric_ref} in-force state "
                    f"{record.get('resulting_threshold_state')} disagrees with the "
                    f"matrix state {metric.get('threshold_state')}"
                ),
                "Keep the in-force change record and the matrix threshold state in lockstep.",
                ref=LEDGER_REL,
            )
        ledger_waiver = record.get("waiver") or {}
        matrix_waiver = metric.get("waiver") or {}
        if ledger_waiver.get("class") != matrix_waiver.get("class") or ledger_waiver.get(
            "expires_on"
        ) != matrix_waiver.get("expires_on"):
            add_finding(
                findings,
                "ledger.active_waiver_disagrees_with_matrix",
                (
                    f"metric {metric_ref} in-force waiver "
                    f"({ledger_waiver.get('class')}, expires {ledger_waiver.get('expires_on')}) "
                    f"disagrees with the matrix waiver "
                    f"({matrix_waiver.get('class')}, expires {matrix_waiver.get('expires_on')})"
                ),
                "Keep the in-force waiver and the matrix waiver binding in lockstep.",
                ref=LEDGER_REL,
            )


# --------------------------------------------------------------------------- #
# Shiproom / release projection.
# --------------------------------------------------------------------------- #


def build_shiproom_projection(
    ledger: dict[str, Any], today: dt.date
) -> dict[str, Any]:
    """Project the active waivers and expiry a shiproom/release packet must show."""
    active_waivers: list[dict[str, Any]] = []
    for record in ledger.get("changes", []):
        if record.get("status") != IN_FORCE_STATUS:
            continue
        waiver = record.get("waiver") or {}
        if waiver.get("class") in (None, "none"):
            continue
        expires = parse_date(waiver.get("expires_on"))
        active_waivers.append(
            {
                "metric_ref": record.get("metric_ref"),
                "change_ref": record.get("change_ref"),
                "waiver_class": waiver.get("class"),
                "waiver_ref": waiver.get("waiver_ref"),
                "expires_on": waiver.get("expires_on"),
                "days_to_expiry": (expires - today).days if expires else None,
                "expired_open": bool(expires and expires < today),
                "loosens_protected_bar": bool(record.get("loosens_protected_bar")),
            }
        )
    active_waivers.sort(key=lambda row: (row["expires_on"] or "", row["metric_ref"] or ""))
    return {
        "evaluated_on": today.isoformat(),
        "active_waiver_count": len(active_waivers),
        "expired_open_waiver_count": sum(1 for w in active_waivers if w["expired_open"]),
        "active_waivers": active_waivers,
    }


# --------------------------------------------------------------------------- #
# Fixture replay.
# --------------------------------------------------------------------------- #


def replay_fixtures(
    repo_root: Path,
    validator: Draft202012Validator,
    metric_ids: set[str],
    today: dt.date,
    findings: list[Finding],
) -> int:
    register = render_yaml_as_json(repo_root / FIXTURE_REGISTER_REL)
    count = 0
    for entry in register.get("fixtures", []):
        rel = entry.get("file") if isinstance(entry, dict) else entry
        fixture = load_json(repo_root / rel)
        expect = fixture.get("__fixture__", {})
        count += 1

        record = {
            key: value
            for key, value in fixture.items()
            if key not in {"__fixture__", "$schema"}
        }
        schema_valid = not list(validator.iter_errors(record))
        if schema_valid != bool(expect.get("expect_schema_valid")):
            add_finding(
                findings,
                "fixture.schema_expectation",
                (
                    f"fixture {expect.get('fixture_id')} schema validity "
                    f"{schema_valid} != expected {expect.get('expect_schema_valid')}"
                ),
                "Align the fixture payload or its expectation.",
                ref=rel,
            )

        if not schema_valid:
            rejected_by = "schema_required_field"
        else:
            rejected_by = reject_change_record(record, metric_ids, today)
        admitted = rejected_by is None

        if admitted != bool(expect.get("expect_admitted")):
            add_finding(
                findings,
                "fixture.admission_expectation",
                (
                    f"fixture {expect.get('fixture_id')} admitted={admitted} "
                    f"!= expected {expect.get('expect_admitted')}"
                ),
                "Align the fixture payload or its expectation.",
                ref=rel,
            )
        if not admitted and rejected_by != expect.get("rejected_by"):
            add_finding(
                findings,
                "fixture.reason_expectation",
                (
                    f"fixture {expect.get('fixture_id')} rejected_by "
                    f"{rejected_by} != expected {expect.get('rejected_by')}"
                ),
                "Align the fixture's rejected_by with the rule that fires.",
                ref=rel,
            )
    return count


# --------------------------------------------------------------------------- #
# Entry point.
# --------------------------------------------------------------------------- #


def resolve_today(ledger: dict[str, Any], override: str | None) -> dt.date:
    if override:
        try:
            return dt.date.fromisoformat(override)
        except ValueError as exc:
            raise SystemExit(f"--today must be an ISO date: {override!r}") from exc
    generated = ledger.get("generated_at", "")
    parsed = parse_date(generated[:10] if isinstance(generated, str) else None)
    if parsed is None:
        raise SystemExit("ledger generated_at is not a parseable date; pass --today")
    return parsed


def main() -> int:
    args = parse_args()
    repo_root = Path(args.repo_root).resolve()
    if not (repo_root / ".git").exists():
        raise SystemExit(f"--repo-root does not look like a repository root: {repo_root}")

    findings: list[Finding] = []
    ledger = load_json(repo_root / LEDGER_REL)
    if not isinstance(ledger, dict):
        raise SystemExit("ledger must be a JSON object")

    today = resolve_today(ledger, args.today)
    metrics = load_matrix_metrics(repo_root)
    metric_ids = set(metrics)

    validator = validate_schema(repo_root, ledger, findings)
    validate_source_refs(repo_root, ledger, findings)
    validate_records(ledger, metrics, today, findings)
    projection = build_shiproom_projection(ledger, today)
    fixture_count = replay_fixtures(repo_root, validator, metric_ids, today, findings)

    errors = [f for f in findings if f.severity == "error"]
    warnings = [f for f in findings if f.severity == "warning"]
    status = "PASS" if not errors else "FAIL"
    print(
        f"[threshold-change] {status} "
        f"({len(errors)} errors, {len(warnings)} warnings) -- "
        f"changes: {len(ledger.get('changes', []))}, "
        f"metrics: {len(metrics)}, "
        f"active waivers: {projection['active_waiver_count']}, "
        f"expired-open waivers: {projection['expired_open_waiver_count']}, "
        f"fixtures: {fixture_count}, evaluated_on: {today.isoformat()}"
    )
    for row in projection["active_waivers"]:
        flag = " EXPIRED-OPEN" if row["expired_open"] else ""
        print(
            f"[threshold-change]   active waiver: {row['metric_ref']} "
            f"{row['waiver_class']} expires {row['expires_on']} "
            f"({row['days_to_expiry']} days){flag}"
        )
    for finding in findings:
        prefix = "ERROR" if finding.severity == "error" else "WARN"
        suffix = f" [{finding.ref}]" if finding.ref else ""
        print(f"[threshold-change] {prefix} {finding.check_id}: {finding.message}{suffix}")
        print(f"[threshold-change]   remediation: {finding.remediation}")

    if args.report:
        report = {
            "schema_version": 1,
            "check_id": "benchmark_threshold_change",
            "evaluated_on": today.isoformat(),
            "status": "pass" if not errors else "fail",
            "ledger_ref": LEDGER_REL,
            "matrix_ref": MATRIX_REL,
            "change_count": len(ledger.get("changes", [])),
            "metric_count": len(metrics),
            "fixture_count": fixture_count,
            "shiproom_projection": projection,
            "finding_counts": {"error": len(errors), "warning": len(warnings)},
            "findings": [f.as_report() for f in findings],
        }
        report_path = repo_root / args.report
        report_path.parent.mkdir(parents=True, exist_ok=True)
        report_path.write_text(
            json.dumps(report, indent=2, sort_keys=True) + "\n", encoding="utf-8"
        )

    return 0 if not errors else 1


if __name__ == "__main__":
    try:
        sys.exit(main())
    except KeyboardInterrupt:
        print("[threshold-change] interrupted", file=sys.stderr)
        sys.exit(130)
