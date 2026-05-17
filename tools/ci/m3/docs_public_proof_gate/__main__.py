#!/usr/bin/env python3
"""Block beta promotion when docs, examples, proof packets, or badges drift."""

from __future__ import annotations

import argparse
import dataclasses
import datetime as dt
import json
import re
import subprocess
import sys
from pathlib import Path
from typing import Any


DEFAULT_SOURCE_MAP_REL = "artifacts/ci/m3_docs_truth_source_map.yaml"
DEFAULT_CLAIM_MANIFEST_REL = "artifacts/release/m3/claim_manifest.json"
DEFAULT_PUBLIC_PROOF_INDEX_REL = "artifacts/milestones/m3/public_proof_index.md"
DEFAULT_PUBLIC_PROOF_CAPTURE_REL = (
    "artifacts/milestones/m3/captures/public_proof_index_validation_capture.json"
)
DEFAULT_DOCS_FRESHNESS_CAPTURE_REL = (
    "artifacts/docs/m3/captures/m3_docs_freshness_validation_capture.json"
)
DEFAULT_STALE_EXAMPLE_CAPTURE_REL = (
    "artifacts/docs/m3/captures/m3_stale_example_validation_capture.json"
)
DEFAULT_HELP_BADGE_VOCAB_REL = "artifacts/docs/help_badge_vocabulary.yaml"
DEFAULT_HELP_ABOUT_DOC_REL = "docs/help/help_about_truth_source.md"
DEFAULT_REPORT_REL = "artifacts/docs/m3/public_proof_parity_report.md"
DEFAULT_CAPTURE_REL = (
    "artifacts/docs/m3/captures/m3_docs_public_proof_parity_capture.json"
)

PUBLIC_PROOF_BEGIN = "<!-- BEGIN canonical:public_proof_index -->"
PUBLIC_PROOF_END = "<!-- END canonical:public_proof_index -->"

MARKETED_ROW_KINDS = {"beta_surface_binding", "beta_archetype_binding"}
MARKETED_CHANNELS = {
    "docs_site",
    "release_notes",
    "public_proof_packet",
    "support_export",
    "help_about",
    "cli_help",
}
BLOCKING_FRESHNESS = {"stale", "unverified"}
FRESHNESS_RANK = {
    "unverified": 0,
    "stale": 1,
    "degraded_cached": 2,
    "warm_cached": 3,
    "authoritative_live": 4,
}
STALE_AFTER_RE = re.compile(r"^P(\d+)D$")
VOLATILE_RE = re.compile(
    r'"captured_at":\s*"[^"]*"|"generated_at":\s*"[^"]*"|'
    r"\*\*Generated at:\*\*\s*`[^`]*`"
)


@dataclasses.dataclass
class Finding:
    severity: str
    check_id: str
    message: str
    remediation: str
    row_id: str | None = None
    artifact_ref: str | None = None
    details: dict[str, Any] = dataclasses.field(default_factory=dict)

    def as_report(self) -> dict[str, Any]:
        payload = dataclasses.asdict(self)
        if self.row_id is None:
            payload.pop("row_id")
        if self.artifact_ref is None:
            payload.pop("artifact_ref")
        if not self.details:
            payload.pop("details")
        return payload


@dataclasses.dataclass
class MarketedRowResult:
    row_id: str
    row_kind: str
    claim_family: str
    lifecycle_label: str
    support_effective: str
    claim_posture_effective: str
    manifest_freshness_badge: str | None
    public_proof_row_id: str | None = None
    public_proof_freshness_class: str | None = None
    public_proof_capture_age_days: int | None = None
    stale_example_status: str = "not_applicable"
    status: str = "pass"

    def as_report(self) -> dict[str, Any]:
        return dataclasses.asdict(self)


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--repo-root", default=".")
    parser.add_argument("--source-map", default=DEFAULT_SOURCE_MAP_REL)
    parser.add_argument("--claim-manifest", default=DEFAULT_CLAIM_MANIFEST_REL)
    parser.add_argument("--public-proof-index", default=DEFAULT_PUBLIC_PROOF_INDEX_REL)
    parser.add_argument("--public-proof-capture", default=DEFAULT_PUBLIC_PROOF_CAPTURE_REL)
    parser.add_argument("--docs-freshness-capture", default=DEFAULT_DOCS_FRESHNESS_CAPTURE_REL)
    parser.add_argument("--stale-example-capture", default=DEFAULT_STALE_EXAMPLE_CAPTURE_REL)
    parser.add_argument("--help-badge-vocabulary", default=DEFAULT_HELP_BADGE_VOCAB_REL)
    parser.add_argument("--help-about-doc", default=DEFAULT_HELP_ABOUT_DOC_REL)
    parser.add_argument("--report", default=DEFAULT_REPORT_REL)
    parser.add_argument("--capture", default=DEFAULT_CAPTURE_REL)
    parser.add_argument(
        "--today",
        default=None,
        help="Override today's ISO date for reproducible freshness checks.",
    )
    parser.add_argument(
        "--check",
        action="store_true",
        help="Fail if generated report or capture artifacts would change.",
    )
    return parser.parse_args()


def render_yaml_as_json(text: str, label: str) -> Any:
    ruby = subprocess.run(
        [
            "ruby",
            "-rjson",
            "-ryaml",
            "-rdate",
            "-rtime",
            "-e",
            (
                "payload = YAML.safe_load(STDIN.read, "
                "permitted_classes: [Time, Date, DateTime], aliases: false); "
                "STDOUT.write(JSON.generate(payload))"
            ),
        ],
        input=text,
        capture_output=True,
        text=True,
    )
    if ruby.returncode != 0:
        stderr = ruby.stderr.strip() or "unknown Ruby/Psych failure"
        raise SystemExit(f"failed to parse YAML for {label}: {stderr}")
    try:
        return json.loads(ruby.stdout)
    except json.JSONDecodeError as exc:
        raise SystemExit(f"Ruby/Psych emitted invalid JSON for {label}: {exc}") from exc


def load_yaml_file(path: Path) -> Any:
    if not path.exists():
        raise SystemExit(f"missing YAML file: {path}")
    return render_yaml_as_json(path.read_text(encoding="utf-8"), str(path))


def load_json(path: Path) -> Any:
    if not path.exists():
        raise SystemExit(f"missing JSON file: {path}")
    return json.loads(path.read_text(encoding="utf-8"))


def ensure_dict(value: Any, label: str) -> dict[str, Any]:
    if not isinstance(value, dict):
        raise SystemExit(f"{label} must be an object")
    return value


def ensure_list(value: Any, label: str) -> list[Any]:
    if not isinstance(value, list):
        raise SystemExit(f"{label} must be a list")
    return value


def ensure_str(value: Any, label: str) -> str:
    if not isinstance(value, str) or not value.strip():
        raise SystemExit(f"{label} must be a non-empty string")
    return value.strip()


def parse_today(value: str | None) -> dt.date:
    if value:
        try:
            return dt.date.fromisoformat(value)
        except ValueError as exc:
            raise SystemExit(f"--today must be an ISO date: {value!r}") from exc
    return dt.datetime.now(dt.timezone.utc).date()


def parse_instant_date(value: str, label: str) -> dt.date | None:
    try:
        return dt.datetime.fromisoformat(value.replace("Z", "+00:00")).date()
    except ValueError:
        try:
            return dt.date.fromisoformat(value)
        except ValueError:
            return None


def stale_after_days(value: str) -> int | None:
    match = STALE_AFTER_RE.match(value)
    if not match:
        return None
    return int(match.group(1))


def strip_fragment(ref: str) -> str:
    return ref.split("#", 1)[0].strip()


def ref_exists(repo_root: Path, ref: str) -> bool:
    target = strip_fragment(ref)
    return bool(target) and (repo_root / target).exists()


def extract_public_proof_index(index_text: str) -> dict[str, Any]:
    if PUBLIC_PROOF_BEGIN not in index_text or PUBLIC_PROOF_END not in index_text:
        raise SystemExit("public-proof index canonical sentinels are missing")
    block = index_text.split(PUBLIC_PROOF_BEGIN, 1)[1].split(PUBLIC_PROOF_END, 1)[0]
    if "```yaml" not in block:
        raise SystemExit("public-proof index canonical block must contain a YAML fence")
    yaml_body = block.split("```yaml", 1)[1].split("```", 1)[0]
    return ensure_dict(render_yaml_as_json(yaml_body, "public-proof index"), "public-proof index")


def now_iso_z() -> str:
    return (
        dt.datetime.now(dt.timezone.utc)
        .replace(microsecond=0)
        .isoformat()
        .replace("+00:00", "Z")
    )


def add_finding(
    findings: list[Finding],
    check_id: str,
    message: str,
    remediation: str,
    *,
    row_id: str | None = None,
    artifact_ref: str | None = None,
    details: dict[str, Any] | None = None,
    severity: str = "error",
) -> None:
    findings.append(
        Finding(
            severity=severity,
            check_id=check_id,
            message=message,
            remediation=remediation,
            row_id=row_id,
            artifact_ref=artifact_ref,
            details=details or {},
        )
    )


def manifest_vocabulary(manifest: dict[str, Any], name: str) -> set[str]:
    vocab = ensure_dict(manifest.get("vocabularies"), "claim_manifest.vocabularies")
    values = ensure_list(vocab.get(name), f"claim_manifest.vocabularies.{name}")
    return {ensure_str(value, f"{name}[]") for value in values}


def help_badge_freshness_values(help_badges: dict[str, Any]) -> set[str]:
    axes = ensure_dict(help_badges.get("axes"), "help_badge_vocabulary.axes")
    freshness = ensure_dict(axes.get("freshness_class"), "axes.freshness_class")
    values = ensure_list(freshness.get("values"), "axes.freshness_class.values")
    return {ensure_str(value, "axes.freshness_class.values[]") for value in values}


def row_channel_projection(row: dict[str, Any], channel_id: str) -> dict[str, Any] | None:
    for raw in ensure_list(row.get("channel_projections"), f"{row.get('row_id')}.channel_projections"):
        projection = ensure_dict(raw, "channel_projection")
        if projection.get("channel_id") == channel_id:
            return projection
    return None


def is_marketed_beta_row(row: dict[str, Any]) -> bool:
    if row.get("row_kind") not in MARKETED_ROW_KINDS:
        return False
    projections = ensure_list(row.get("channel_projections"), f"{row.get('row_id')}.channel_projections")
    for raw in projections:
        projection = ensure_dict(raw, "channel_projection")
        if (
            projection.get("channel_id") in MARKETED_CHANNELS
            and projection.get("binding_status") == "required"
        ):
            return True
    return False


def indexed_rows(rows: list[Any], key: str, label: str) -> dict[str, dict[str, Any]]:
    out: dict[str, dict[str, Any]] = {}
    for raw in rows:
        row = ensure_dict(raw, f"{label}[]")
        row_id = ensure_str(row.get(key), f"{label}[].{key}")
        out[row_id] = row
    return out


def public_proof_rows(index: dict[str, Any]) -> dict[str, dict[str, Any]]:
    return {
        ensure_str(row.get("claim_family"), "public_proof.rows[].claim_family"): row
        for row in ensure_list(index.get("rows"), "public_proof.rows")
        if isinstance(row, dict)
    }


def capture_downgrade_rows(capture: dict[str, Any]) -> dict[str, dict[str, Any]]:
    return {
        ensure_str(row.get("claim_family"), "downgrade_matrix[].claim_family"): row
        for row in ensure_list(capture.get("downgrade_matrix"), "public_proof_capture.downgrade_matrix")
        if isinstance(row, dict)
    }


def check_capture_status(
    capture: dict[str, Any],
    capture_ref: str,
    check_prefix: str,
    findings: list[Finding],
) -> None:
    if capture.get("status") != "pass":
        add_finding(
            findings,
            f"{check_prefix}.capture_failed",
            f"{capture_ref} status is {capture.get('status')!r}",
            "Run the producing gate, fix its findings, and commit the refreshed capture.",
            artifact_ref=capture_ref,
            details={"finding_counts": capture.get("finding_counts", {})},
        )
    for raw in capture.get("findings", []):
        if not isinstance(raw, dict):
            continue
        if raw.get("severity") == "error":
            add_finding(
                findings,
                f"{check_prefix}.upstream_error",
                ensure_str(raw.get("message", "upstream error"), f"{check_prefix}.finding.message"),
                ensure_str(raw.get("remediation", "Fix the upstream gate."), f"{check_prefix}.finding.remediation"),
                artifact_ref=raw.get("ref") if isinstance(raw.get("ref"), str) else capture_ref,
                details={"upstream_check_id": raw.get("check_id")},
            )


def check_vocabulary_parity(
    manifest_values: set[str],
    help_values: set[str],
    proof_index: dict[str, Any],
    findings: list[Finding],
) -> dict[str, Any]:
    missing_from_help = sorted(manifest_values - help_values)
    missing_from_manifest = sorted(help_values - manifest_values)
    if missing_from_help or missing_from_manifest:
        add_finding(
            findings,
            "freshness_vocabulary.shared_semantics_mismatch",
            "claim manifest and Help/About badge vocabulary do not publish the same freshness tokens",
            "Update the manifest vocabulary and help badge vocabulary in the same change set.",
            details={
                "missing_from_help_badges": missing_from_help,
                "missing_from_manifest": missing_from_manifest,
            },
        )

    proof_tokens: set[str] = set()
    for row in ensure_list(proof_index.get("rows"), "public_proof.rows"):
        if not isinstance(row, dict):
            continue
        freshness = ensure_dict(row.get("freshness"), "public_proof.row.freshness")
        token = ensure_str(freshness.get("freshness_class"), "public_proof.freshness_class")
        proof_tokens.add(token)
        if token not in manifest_values:
            add_finding(
                findings,
                "public_proof.freshness_class_not_shared",
                f"public-proof row uses freshness token {token!r} outside the shared vocabulary",
                "Use a freshness_class value also published by claim manifest and Help/About badges.",
                row_id=row.get("row_id") if isinstance(row.get("row_id"), str) else None,
                details={"freshness_class": token},
            )

    return {
        "manifest_freshness_tokens": sorted(manifest_values),
        "help_about_freshness_tokens": sorted(help_values),
        "public_proof_freshness_tokens": sorted(proof_tokens),
        "shared": not missing_from_help and not missing_from_manifest,
    }


def check_public_proof_capture_parity(
    proof_by_family: dict[str, dict[str, Any]],
    capture_by_family: dict[str, dict[str, Any]],
    findings: list[Finding],
) -> None:
    for family, proof_row in proof_by_family.items():
        capture_row = capture_by_family.get(family)
        row_id = ensure_str(proof_row.get("row_id"), "public_proof.row_id")
        if capture_row is None:
            add_finding(
                findings,
                "public_proof_capture.family_missing",
                f"public-proof validation capture has no downgrade row for claim family {family!r}",
                "Run the public-proof index validator and commit the refreshed capture.",
                row_id=row_id,
            )
            continue
        for field_name in (
            "canonical_packet_ref",
            "proof_class_id",
            "stale_after",
            "stale_propagation_profile",
        ):
            expected: Any
            if field_name in {"stale_after", "stale_propagation_profile"}:
                expected = ensure_dict(proof_row.get("freshness"), f"{row_id}.freshness").get(field_name)
            else:
                expected = proof_row.get(field_name)
            if capture_row.get(field_name) != expected:
                add_finding(
                    findings,
                    f"public_proof_capture.{field_name}_mismatch",
                    (
                        f"public-proof validation capture for {family!r} has "
                        f"{field_name}={capture_row.get(field_name)!r}, expected {expected!r}"
                    ),
                    "Re-run the public-proof index validator so the downgrade matrix matches the canonical index.",
                    row_id=row_id,
                    details={"claim_family": family, "expected": expected, "actual": capture_row.get(field_name)},
                )
        expected_outputs = list(ensure_list(proof_row.get("current_outputs"), f"{row_id}.current_outputs"))
        if capture_row.get("current_outputs") != expected_outputs:
            add_finding(
                findings,
                "public_proof_capture.current_outputs_mismatch",
                f"public-proof validation capture current outputs drifted for {family!r}",
                "Re-run the public-proof index validator after editing current_outputs.",
                row_id=row_id,
                details={
                    "expected": expected_outputs,
                    "actual": capture_row.get("current_outputs"),
                },
            )


def check_required_docs_outputs(
    docs_row: dict[str, Any] | None,
    required_outputs: list[str],
    repo_root: Path,
    findings: list[Finding],
) -> None:
    if docs_row is None:
        add_finding(
            findings,
            "docs_public_proof.docs_family_missing",
            "public-proof index has no docs_freshness row",
            "Add the docs_freshness row to the public-proof index.",
        )
        return
    row_id = ensure_str(docs_row.get("row_id"), "docs public-proof row_id")
    outputs = {
        ensure_str(value, f"{row_id}.current_outputs[]")
        for value in ensure_list(docs_row.get("current_outputs"), f"{row_id}.current_outputs")
    }
    for required in required_outputs:
        if required not in outputs:
            add_finding(
                findings,
                "docs_public_proof.required_output_missing",
                f"docs public-proof row does not list required current output {required}",
                "Add the docs gate output to current_outputs so stale examples and parity reports are proof-bearing.",
                row_id=row_id,
                artifact_ref=required,
            )
        if not ref_exists(repo_root, required):
            add_finding(
                findings,
                "docs_public_proof.required_output_not_on_disk",
                f"required docs gate output is missing on disk: {required}",
                "Run the docs/public-proof gate and commit the generated output.",
                row_id=row_id,
                artifact_ref=required,
            )


def freshness_age_from_latest_capture(
    proof_row: dict[str, Any],
    today: dt.date,
    findings: list[Finding],
) -> int | None:
    row_id = ensure_str(proof_row.get("row_id"), "public_proof.row_id")
    latest = ensure_dict(proof_row.get("latest_capture"), f"{row_id}.latest_capture")
    captured_at = ensure_str(latest.get("captured_at"), f"{row_id}.latest_capture.captured_at")
    captured_date = parse_instant_date(captured_at, f"{row_id}.latest_capture.captured_at")
    if captured_date is None:
        add_finding(
            findings,
            "public_proof.latest_capture.captured_at_invalid",
            f"latest_capture.captured_at is not parseable: {captured_at!r}",
            "Use an ISO-8601 timestamp on latest_capture.captured_at.",
            row_id=row_id,
        )
        return None
    freshness = ensure_dict(proof_row.get("freshness"), f"{row_id}.freshness")
    stale_after = ensure_str(freshness.get("stale_after"), f"{row_id}.freshness.stale_after")
    window_days = stale_after_days(stale_after)
    if window_days is None:
        add_finding(
            findings,
            "public_proof.freshness.stale_after_invalid",
            f"stale_after must be of the form P<N>D, got {stale_after!r}",
            "Use a duration like P14D.",
            row_id=row_id,
        )
        return None
    age = (today - captured_date).days
    if age > window_days:
        add_finding(
            findings,
            "public_proof.latest_capture_expired",
            f"public-proof row latest capture is {age} days old, beyond {stale_after}",
            "Refresh the row's proof packet and validation capture before promoting the marketed rows.",
            row_id=row_id,
            details={"age_days": age, "stale_after_days": window_days},
        )
    return age


def check_marketed_rows(
    manifest: dict[str, Any],
    proof_by_family: dict[str, dict[str, Any]],
    freshness_vocab: set[str],
    today: dt.date,
    findings: list[Finding],
    stale_example_status_by_family: dict[str, str],
    stale_example_capture_ref: str,
) -> list[MarketedRowResult]:
    results: list[MarketedRowResult] = []
    for row in ensure_list(manifest.get("rows"), "claim_manifest.rows"):
        row = ensure_dict(row, "claim_manifest.rows[]")
        if not is_marketed_beta_row(row):
            continue
        row_id = ensure_str(row.get("row_id"), "claim_manifest.row_id")
        row_kind = ensure_str(row.get("row_kind"), f"{row_id}.row_kind")
        claim_family = ensure_str(row.get("claim_family"), f"{row_id}.claim_family")
        lifecycle = ensure_dict(row.get("lifecycle"), f"{row_id}.lifecycle")
        support = ensure_dict(row.get("support"), f"{row_id}.support")
        posture = ensure_dict(row.get("claim_posture"), f"{row_id}.claim_posture")
        freshness = row.get("freshness")
        badge: str | None = None
        if not isinstance(freshness, dict):
            add_finding(
                findings,
                "marketed_row.freshness_block_missing",
                "marketed beta row has no freshness block",
                "Regenerate the claim manifest with freshness.badge_class, evidence_date, and review_window_days.",
                row_id=row_id,
            )
        else:
            raw_badge = freshness.get("badge_class")
            if not isinstance(raw_badge, str) or not raw_badge.strip():
                add_finding(
                    findings,
                    "marketed_row.freshness_badge_missing",
                    "marketed beta row has no freshness.badge_class",
                    "Regenerate the claim manifest with a shared freshness badge on every marketed row.",
                    row_id=row_id,
                )
            else:
                badge = raw_badge.strip()
                if badge not in freshness_vocab:
                    add_finding(
                        findings,
                        "marketed_row.freshness_badge_unknown",
                        f"marketed beta row uses unknown freshness badge {badge!r}",
                        "Use a badge token from the shared freshness vocabulary.",
                        row_id=row_id,
                        details={"badge_class": badge},
                    )
                if badge in BLOCKING_FRESHNESS:
                    add_finding(
                        findings,
                        "marketed_row.freshness_badge_below_floor",
                        f"marketed beta row uses blocking freshness badge {badge!r}",
                        "Refresh the proof or narrow/withdraw the row before promotion.",
                        row_id=row_id,
                        details={"badge_class": badge},
                    )

        proof_row = proof_by_family.get(claim_family)
        proof_row_id: str | None = None
        proof_freshness: str | None = None
        proof_age: int | None = None
        if proof_row is None:
            add_finding(
                findings,
                "marketed_row.public_proof_family_missing",
                f"marketed beta row claim family {claim_family!r} has no public-proof row",
                "Add the claim family to the public-proof index before promotion.",
                row_id=row_id,
                details={"claim_family": claim_family},
            )
        else:
            proof_row_id = ensure_str(proof_row.get("row_id"), "public_proof.row_id")
            proof_freshness = ensure_str(
                ensure_dict(proof_row.get("freshness"), f"{proof_row_id}.freshness").get("freshness_class"),
                f"{proof_row_id}.freshness.freshness_class",
            )
            proof_age = freshness_age_from_latest_capture(proof_row, today, findings)
            if proof_freshness in BLOCKING_FRESHNESS:
                add_finding(
                    findings,
                    "marketed_row.public_proof_freshness_below_floor",
                    f"public-proof row uses blocking freshness class {proof_freshness!r}",
                    "Refresh the public-proof row or narrow the marketed rows that depend on it.",
                    row_id=row_id,
                    artifact_ref=proof_row_id,
                )
            if badge in FRESHNESS_RANK and proof_freshness in FRESHNESS_RANK:
                if FRESHNESS_RANK[badge] > FRESHNESS_RANK[proof_freshness]:
                    add_finding(
                        findings,
                        "marketed_row.badge_wider_than_public_proof",
                        (
                            f"manifest badge {badge!r} is fresher than public-proof "
                            f"freshness {proof_freshness!r}"
                        ),
                        "Downgrade the manifest badge or refresh the proof row so the product does not overstate freshness.",
                        row_id=row_id,
                        artifact_ref=proof_row_id,
                        details={"manifest_badge": badge, "public_proof_freshness": proof_freshness},
                    )
            proof_dependency_errors = sorted(
                {
                    finding.check_id
                    for finding in findings
                    if finding.row_id == proof_row_id and finding.severity == "error"
                }
            )
            if proof_dependency_errors:
                add_finding(
                    findings,
                    "marketed_row.public_proof_dependency_failed",
                    f"marketed beta row depends on public-proof row {proof_row_id!r} with blocking findings",
                    "Fix the public-proof row or its validation capture before promoting the marketed row.",
                    row_id=row_id,
                    artifact_ref=proof_row_id,
                    details={"upstream_check_ids": proof_dependency_errors},
                )

        stale_status = stale_example_status_by_family.get(claim_family, "not_applicable")
        if stale_status == "fail":
            add_finding(
                findings,
                "marketed_row.stale_examples_failed",
                "marketed beta row is backed by protected examples with stale-example failures",
                "Refresh the protected examples or narrow the marketed row before promotion.",
                row_id=row_id,
                artifact_ref=stale_example_capture_ref,
                details={"claim_family": claim_family},
            )

        result = MarketedRowResult(
            row_id=row_id,
            row_kind=row_kind,
            claim_family=claim_family,
            lifecycle_label=ensure_str(
                lifecycle.get("display_lifecycle_label"),
                f"{row_id}.lifecycle.display_lifecycle_label",
            ),
            support_effective=ensure_str(support.get("effective"), f"{row_id}.support.effective"),
            claim_posture_effective=ensure_str(
                posture.get("effective"),
                f"{row_id}.claim_posture.effective",
            ),
            manifest_freshness_badge=badge,
            public_proof_row_id=proof_row_id,
            public_proof_freshness_class=proof_freshness,
            public_proof_capture_age_days=proof_age,
            stale_example_status=stale_status,
        )
        row_errors = [finding for finding in findings if finding.row_id == row_id]
        if row_errors:
            result.status = "fail"
        results.append(result)
    if not results:
        add_finding(
            findings,
            "marketed_rows.none_detected",
            "no marketed beta rows were detected in the claim manifest",
            "Check row_kind and required channel projections in the claim manifest.",
        )
    return results


def source_map_example_families(source_map: dict[str, Any]) -> dict[str, str]:
    families: dict[str, str] = {}
    for raw in ensure_list(source_map.get("protected_examples"), "source_map.protected_examples"):
        example = ensure_dict(raw, "source_map.protected_examples[]")
        example_id = ensure_str(example.get("example_id"), "protected_examples[].example_id")
        family = example.get("claim_family") or example.get("blocking_claim_family") or "docs_freshness"
        families[example_id] = ensure_str(family, f"{example_id}.claim_family")
    return families


def stale_example_status_by_family(
    source_map: dict[str, Any],
    stale_capture: dict[str, Any],
    findings: list[Finding],
) -> dict[str, str]:
    examples = ensure_list(source_map.get("protected_examples"), "source_map.protected_examples")
    if not examples:
        add_finding(
            findings,
            "stale_examples.no_protected_examples",
            "source map has no protected examples",
            "Add at least one protected example so docs examples can block promotion.",
        )
        return {}
    family_by_example = source_map_example_families(source_map)
    capture_examples = indexed_rows(
        ensure_list(stale_capture.get("examples"), "stale_example_capture.examples"),
        "example_id",
        "stale_example_capture.examples",
    )
    status_by_family: dict[str, str] = {}
    for example_id, family in family_by_example.items():
        result = capture_examples.get(example_id)
        if result is None:
            add_finding(
                findings,
                "stale_examples.capture_example_missing",
                f"stale-example capture has no result for protected example {example_id}",
                "Run the stale-example checker and commit the refreshed capture.",
                artifact_ref=example_id,
                details={"claim_family": family},
            )
            status_by_family[family] = "fail"
            continue
        failed = ensure_list(result.get("failed_checks"), f"{example_id}.failed_checks")
        if failed:
            status_by_family[family] = "fail"
        else:
            status_by_family.setdefault(family, "pass")
    return status_by_family


def validate_help_about_doc(repo_root: Path, help_about_rel: str, findings: list[Finding]) -> None:
    path = repo_root / help_about_rel
    if not path.exists():
        add_finding(
            findings,
            "help_about.surface_missing",
            f"Help/About truth-source doc is missing: {help_about_rel}",
            "Restore the Help/About truth-source surface so in-product badge semantics are inspectable.",
            artifact_ref=help_about_rel,
        )
        return
    body = path.read_text(encoding="utf-8")
    for token in ("HelpAboutReleaseTruthCard", "claim-manifest", "freshness"):
        if token not in body:
            add_finding(
                findings,
                "help_about.required_token_missing",
                f"Help/About truth-source doc does not cite required token {token!r}",
                "Update the Help/About truth-source doc so the in-product badge consumer is discoverable.",
                artifact_ref=help_about_rel,
                details={"missing_token": token},
            )


def render_report(
    *,
    generated_at: str,
    today: dt.date,
    findings: list[Finding],
    marketed_rows: list[MarketedRowResult],
    vocabulary_parity: dict[str, Any],
    captures: dict[str, str],
    args: argparse.Namespace,
) -> str:
    status = "PASS" if not any(f.severity == "error" for f in findings) else "FAIL"
    lines: list[str] = []
    lines.append("# M3 docs / public-proof parity report")
    lines.append("")
    lines.append(
        "This file is generated by `python3 -m tools.ci.m3.docs_public_proof_gate` "
        "from the claim manifest, docs-truth captures, stale-example capture, "
        "public-proof index, and Help/About badge vocabulary. Do not hand-edit."
    )
    lines.append("")
    lines.append("## Run metadata")
    lines.append("")
    lines.append(f"- **Generated at:** `{generated_at}`")
    lines.append(f"- **Today:** `{today.isoformat()}`")
    lines.append(f"- **Gate status:** `{status}`")
    lines.append(f"- **Error findings:** `{sum(1 for f in findings if f.severity == 'error')}`")
    lines.append(f"- **Warning findings:** `{sum(1 for f in findings if f.severity == 'warning')}`")
    lines.append(f"- **Claim manifest:** `{args.claim_manifest}`")
    lines.append(f"- **Public-proof index:** `{args.public_proof_index}`")
    lines.append(f"- **Source map:** `{args.source_map}`")
    lines.append("")
    lines.append("## Input captures")
    lines.append("")
    lines.append("| Capture | Status |")
    lines.append("|---|---|")
    for ref, capture_status in captures.items():
        lines.append(f"| `{ref}` | `{capture_status}` |")
    lines.append("")
    lines.append("## Shared freshness vocabulary")
    lines.append("")
    lines.append("| Source | Tokens |")
    lines.append("|---|---|")
    lines.append(
        "| Claim manifest | "
        + ", ".join(f"`{v}`" for v in vocabulary_parity["manifest_freshness_tokens"])
        + " |"
    )
    lines.append(
        "| Help/About badges | "
        + ", ".join(f"`{v}`" for v in vocabulary_parity["help_about_freshness_tokens"])
        + " |"
    )
    lines.append(
        "| Public proof | "
        + ", ".join(f"`{v}`" for v in vocabulary_parity["public_proof_freshness_tokens"])
        + " |"
    )
    lines.append("")
    lines.append("## Marketed beta rows")
    lines.append("")
    lines.append(
        "| Row | Family | Lifecycle | Support | Claim posture | Manifest badge | Proof row | Proof freshness | Proof age | Examples | Status |"
    )
    lines.append("|---|---|---|---|---|---|---|---|---:|---|---|")
    for row in marketed_rows:
        lines.append(
            f"| `{row.row_id}` | `{row.claim_family}` | `{row.lifecycle_label}` | "
            f"`{row.support_effective}` | `{row.claim_posture_effective}` | "
            f"`{row.manifest_freshness_badge or 'missing'}` | "
            f"`{row.public_proof_row_id or 'missing'}` | "
            f"`{row.public_proof_freshness_class or 'missing'}` | "
            f"{row.public_proof_capture_age_days if row.public_proof_capture_age_days is not None else ''} | "
            f"`{row.stale_example_status}` | `{row.status}` |"
        )
    lines.append("")
    lines.append("## Findings")
    lines.append("")
    if not findings:
        lines.append("_All parity checks pass; marketed rows are not wider than their docs/public-proof evidence._")
    else:
        for finding in findings:
            loc = finding.row_id or finding.artifact_ref or "global"
            lines.append(f"- `{finding.check_id}` (`{loc}`): {finding.message}")
            lines.append(f"  - **Remediation:** {finding.remediation}")
    lines.append("")
    lines.append("## How to refresh")
    lines.append("")
    lines.append("```sh")
    lines.append("bash ci/check_m3_docs_truth.sh")
    lines.append("python3 ci/check_m3_public_proof_index.py --repo-root .")
    lines.append("python3 -m tools.ci.m3.docs_public_proof_gate --repo-root .")
    lines.append("```")
    lines.append("")
    lines.append("Use `--check` in CI to fail when this report or its capture would drift.")
    lines.append("")
    return "\n".join(lines)


def normalize_generated(text: str) -> str:
    return VOLATILE_RE.sub("__volatile_timestamp__", text)


def write_text_if_changed(path: Path, text: str, check_only: bool) -> bool:
    path.parent.mkdir(parents=True, exist_ok=True)
    old = path.read_text(encoding="utf-8") if path.exists() else None
    changed = old is None or normalize_generated(old) != normalize_generated(text)
    if not check_only:
        path.write_text(text, encoding="utf-8")
    return changed


def render_capture(
    *,
    generated_at: str,
    today: dt.date,
    findings: list[Finding],
    marketed_rows: list[MarketedRowResult],
    vocabulary_parity: dict[str, Any],
    captures: dict[str, str],
    args: argparse.Namespace,
) -> str:
    payload = {
        "schema_version": 1,
        "capture_kind": "m3_docs_public_proof_parity_capture",
        "captured_at": generated_at,
        "today": today.isoformat(),
        "status": "pass" if not any(f.severity == "error" for f in findings) else "fail",
        "finding_counts": {
            "error": sum(1 for f in findings if f.severity == "error"),
            "warning": sum(1 for f in findings if f.severity == "warning"),
        },
        "source_map_ref": args.source_map,
        "claim_manifest_ref": args.claim_manifest,
        "public_proof_index_ref": args.public_proof_index,
        "public_proof_capture_ref": args.public_proof_capture,
        "docs_freshness_capture_ref": args.docs_freshness_capture,
        "stale_example_capture_ref": args.stale_example_capture,
        "help_badge_vocabulary_ref": args.help_badge_vocabulary,
        "report_ref": args.report,
        "captures": captures,
        "vocabulary_parity": vocabulary_parity,
        "marketed_rows": [row.as_report() for row in marketed_rows],
        "findings": [finding.as_report() for finding in findings],
    }
    return json.dumps(payload, indent=2, sort_keys=True) + "\n"


def main() -> int:
    args = parse_args()
    repo_root = Path(args.repo_root).resolve()
    if not (repo_root / ".git").exists():
        raise SystemExit(f"--repo-root does not look like a repository root: {repo_root}")

    today = parse_today(args.today)
    source_map = ensure_dict(load_yaml_file(repo_root / args.source_map), "source_map")
    manifest = ensure_dict(load_json(repo_root / args.claim_manifest), "claim_manifest")
    proof_index = extract_public_proof_index(
        (repo_root / args.public_proof_index).read_text(encoding="utf-8")
    )
    proof_capture = ensure_dict(load_json(repo_root / args.public_proof_capture), "public_proof_capture")
    docs_capture = ensure_dict(load_json(repo_root / args.docs_freshness_capture), "docs_freshness_capture")
    stale_capture = ensure_dict(load_json(repo_root / args.stale_example_capture), "stale_example_capture")
    help_badges = ensure_dict(load_yaml_file(repo_root / args.help_badge_vocabulary), "help_badge_vocabulary")

    findings: list[Finding] = []
    captures = {
        args.public_proof_capture: str(proof_capture.get("status", "unknown")),
        args.docs_freshness_capture: str(docs_capture.get("status", "unknown")),
        args.stale_example_capture: str(stale_capture.get("status", "unknown")),
    }

    check_capture_status(proof_capture, args.public_proof_capture, "public_proof", findings)
    check_capture_status(docs_capture, args.docs_freshness_capture, "docs_freshness", findings)
    check_capture_status(stale_capture, args.stale_example_capture, "stale_examples", findings)

    manifest_freshness = manifest_vocabulary(manifest, "freshness_badge_class")
    help_freshness = help_badge_freshness_values(help_badges)
    vocabulary_parity = check_vocabulary_parity(
        manifest_freshness,
        help_freshness,
        proof_index,
        findings,
    )

    proof_by_family = public_proof_rows(proof_index)
    capture_by_family = capture_downgrade_rows(proof_capture)
    check_public_proof_capture_parity(proof_by_family, capture_by_family, findings)
    check_required_docs_outputs(
        proof_by_family.get("docs_freshness"),
        [
            args.docs_freshness_capture,
            args.stale_example_capture,
            args.report,
            args.capture,
        ],
        repo_root,
        findings,
    )
    validate_help_about_doc(repo_root, args.help_about_doc, findings)

    stale_status = stale_example_status_by_family(source_map, stale_capture, findings)
    marketed_rows = check_marketed_rows(
        manifest,
        proof_by_family,
        manifest_freshness & help_freshness,
        today,
        findings,
        stale_status,
        args.stale_example_capture,
    )

    generated_at = now_iso_z()
    report_text = render_report(
        generated_at=generated_at,
        today=today,
        findings=findings,
        marketed_rows=marketed_rows,
        vocabulary_parity=vocabulary_parity,
        captures=captures,
        args=args,
    )
    capture_text = render_capture(
        generated_at=generated_at,
        today=today,
        findings=findings,
        marketed_rows=marketed_rows,
        vocabulary_parity=vocabulary_parity,
        captures=captures,
        args=args,
    )

    report_changed = write_text_if_changed(repo_root / args.report, report_text, args.check)
    capture_changed = write_text_if_changed(repo_root / args.capture, capture_text, args.check)

    if args.check and (report_changed or capture_changed):
        if report_changed:
            add_finding(
                findings,
                "parity_report.stale",
                "checked-in parity report is stale",
                "Run `python3 -m tools.ci.m3.docs_public_proof_gate --repo-root .` and commit the report.",
                artifact_ref=args.report,
            )
        if capture_changed:
            add_finding(
                findings,
                "parity_capture.stale",
                "checked-in parity capture is stale",
                "Run `python3 -m tools.ci.m3.docs_public_proof_gate --repo-root .` and commit the capture.",
                artifact_ref=args.capture,
            )
        report_text = render_report(
            generated_at=generated_at,
            today=today,
            findings=findings,
            marketed_rows=marketed_rows,
            vocabulary_parity=vocabulary_parity,
            captures=captures,
            args=args,
        )
        capture_text = render_capture(
            generated_at=generated_at,
            today=today,
            findings=findings,
            marketed_rows=marketed_rows,
            vocabulary_parity=vocabulary_parity,
            captures=captures,
            args=args,
        )
        write_text_if_changed(repo_root / args.report, report_text, False)
        write_text_if_changed(repo_root / args.capture, capture_text, False)

    errors = [finding for finding in findings if finding.severity == "error"]
    warnings = [finding for finding in findings if finding.severity == "warning"]
    status = "PASS" if not errors else "FAIL"
    print(
        f"[m3-docs-public-proof] {status} ({len(errors)} errors, {len(warnings)} warnings) "
        f"-- report: {args.report}"
    )
    for finding in findings:
        prefix = "ERROR" if finding.severity == "error" else "WARN"
        loc = finding.row_id or finding.artifact_ref or "global"
        print(f"[m3-docs-public-proof] {prefix} {finding.check_id}: {finding.message} [{loc}]")
        print(f"[m3-docs-public-proof]   remediation: {finding.remediation}")
    return 0 if not errors else 1


if __name__ == "__main__":
    try:
        sys.exit(main())
    except KeyboardInterrupt:
        print("[m3-docs-public-proof] interrupted", file=sys.stderr)
        sys.exit(130)
