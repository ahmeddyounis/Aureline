#!/usr/bin/env python3
"""Validate the beta install rollback drill without a cargo build.

The beta rollback drill matures the alpha synthetic state-root rollback into a
beta rehearsal that ties the synthetic restore to the governed beta rollback
plan (``artifacts/release/m3/update_rollback/rollback_plan.json``) and the
release-center rollback/revocation record model. This gate reads the same frozen
fixtures the Rust test reads
(``cargo test -p aureline-install --test rollback_drill_beta``) and proves they
agree on one closed verdict for whether the prior known-good build can honestly
back a rollback.

The gate:

  - resolves prior-build availability from the rollback plan, the healthy
    release-center rollback record, and the post-rollback exact-build
    diagnostics, and fails if the happy-path inputs do not agree the prior build
    is available;
  - asserts the honest-failure fixture (a revoked rollback target) resolves to
    *unavailable* rather than a silent pass; and
  - runs negative drills proving a broken artifact graph, a dropped rollback
    manifest, a last-known-good drift, an unverifiable retained artifact, and an
    unresolved post-rollback manifest each flip the verdict to unavailable.

The gate is build-free so it and the Rust test agree on every verdict without a
cargo build in CI.
"""

from __future__ import annotations

import argparse
import copy
import dataclasses
import datetime as dt
import json
import sys
from pathlib import Path
from typing import Any


DEFAULT_PLAN_REL = "artifacts/release/m3/update_rollback/rollback_plan.json"
DEFAULT_FIXTURES_DIR_REL = "fixtures/install/rollback_drill_beta"
DEFAULT_REPORT_REL = "artifacts/release/captures/rollback_drill_beta_validation_capture.json"

HEALTHY_RECORD_NAME = "release_center_rollback_record.json"
MISSING_PRIOR_BUILD_RECORD_NAME = "release_center_rollback_record_missing_prior_build.json"
POST_ROLLBACK_DIAGNOSTICS_NAME = "post_rollback_exact_build_diagnostics.json"

CONSISTENT_GRAPH_CLASSES = ("consistent_full_graph", "consistent_scoped_exception")


@dataclasses.dataclass
class Finding:
    severity: str
    check_id: str
    message: str
    ref: str

    def as_report(self) -> dict[str, str]:
        return dataclasses.asdict(self)


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--repo-root", default=".")
    parser.add_argument("--plan", default=DEFAULT_PLAN_REL)
    parser.add_argument("--fixtures-dir", default=DEFAULT_FIXTURES_DIR_REL)
    parser.add_argument(
        "--report",
        default=None,
        help="Optional path for a JSON validation capture.",
    )
    parser.add_argument(
        "--check",
        action="store_true",
        help="Write the validation capture to the default path when no --report is given.",
    )
    return parser.parse_args()


def load_json(path: Path) -> Any:
    if not path.exists():
        raise SystemExit(f"missing JSON file: {path}")
    try:
        return json.loads(path.read_text(encoding="utf-8"))
    except json.JSONDecodeError as exc:
        raise SystemExit(f"invalid JSON at {path}: {exc}") from exc


def ensure_dict(value: Any, label: str) -> dict[str, Any]:
    if not isinstance(value, dict):
        raise SystemExit(f"{label} must be a JSON object")
    return value


def ensure_list(value: Any, label: str) -> list[Any]:
    if not isinstance(value, list):
        raise SystemExit(f"{label} must be a JSON array")
    return value


def generated_at_now() -> str:
    return (
        dt.datetime.now(dt.UTC)
        .replace(microsecond=0)
        .isoformat()
        .replace("+00:00", "Z")
    )


def json_text(payload: dict[str, Any]) -> str:
    return json.dumps(payload, indent=2, sort_keys=True) + "\n"


def rollback_target_ref(plan: dict[str, Any]) -> str:
    target = ensure_dict(plan.get("rollback_target"), "plan.rollback_target")
    ref = target.get("exact_build_identity_ref")
    if not isinstance(ref, str) or not ref.strip():
        raise SystemExit("plan.rollback_target.exact_build_identity_ref must be a non-empty string")
    return ref


def current_build_ref(plan: dict[str, Any]) -> str:
    current = ensure_dict(plan.get("current_build"), "plan.current_build")
    ref = current.get("exact_build_identity_ref")
    if not isinstance(ref, str) or not ref.strip():
        raise SystemExit("plan.current_build.exact_build_identity_ref must be a non-empty string")
    return ref


def availability_reasons(
    plan: dict[str, Any],
    record: dict[str, Any],
    diagnostics: dict[str, Any],
) -> list[str]:
    """Mirrors the Rust ``prior_build_availability`` cross-check.

    Returns the list of reasons the prior known-good build cannot honestly back
    a rollback. An empty list means the prior build is available.
    """
    reasons: list[str] = []
    target = rollback_target_ref(plan)

    for artifact in ensure_list(
        plan.get("retained_prior_artifacts", []), "plan.retained_prior_artifacts"
    ):
        item = ensure_dict(artifact, "plan.retained_prior_artifacts[]")
        artifact_ref = str(item.get("artifact_ref", "<artifact>"))
        if item.get("retention_state") != "retained_exact_build":
            reasons.append(f"retained prior artifact {artifact_ref} is not exact-build retained")
        if item.get("verification_state") != "verified":
            reasons.append(f"retained prior artifact {artifact_ref} is not verified")

    if record.get("kind") != "rollback":
        reasons.append(f"release-center record kind is {record.get('kind')!r}, not a rollback")
    if not record.get("rollback_manifest_ref"):
        reasons.append("release-center rollback record has no rollback manifest")
    if record.get("last_known_good_ref") != target:
        reasons.append("release-center last-known-good ref does not match the plan rollback target")
    if record.get("artifact_graph_consistency") not in CONSISTENT_GRAPH_CLASSES:
        reasons.append(
            f"release-center artifact graph consistency is {record.get('artifact_graph_consistency')!r}"
        )
    affected = [str(ref) for ref in record.get("affected_artifact_refs", [])]
    if target in affected:
        reasons.append("rollback target exact-build is in the record's affected (revoked) set")

    rows = ensure_list(diagnostics.get("rows", []), "diagnostics.rows")
    if not rows:
        reasons.append("post-rollback diagnostics have no rows")
    for raw_row in rows:
        row = ensure_dict(raw_row, "diagnostics.rows[]")
        row_id = str(row.get("topology_row_id", "<row>"))
        exact_build = ensure_dict(row.get("exact_build"), f"{row_id}.exact_build")
        if exact_build.get("exact_build_identity_ref") != target:
            reasons.append(
                f"post-rollback diagnostics row {row_id} does not resolve to the rollback target build"
            )
        if exact_build.get("manifest_state") != "present":
            reasons.append(
                f"post-rollback diagnostics row {row_id} has no present exact-build manifest"
            )

    return reasons


def validate_release_center_record(record: dict[str, Any]) -> list[Finding]:
    """Checks the release-center rollback record's own well-formedness."""
    findings: list[Finding] = []
    record_id = str(record.get("record_id", "<record>"))
    for field in ("record_id", "kind", "artifact_graph_ref", "last_known_good_ref"):
        if not record.get(field):
            findings.append(
                Finding("error", "record.field_missing", f"{field} must not be empty", record_id)
            )
    if not record.get("affected_artifact_refs"):
        findings.append(
            Finding(
                "error",
                "record.affected_artifact_refs",
                "rollback or revocation must name the affected artifact set",
                record_id,
            )
        )
    if not record.get("known_issue_refs"):
        findings.append(
            Finding(
                "error",
                "record.known_issue_refs",
                "rollback or revocation must preserve known-issue publication refs",
                record_id,
            )
        )
    if not record.get("support_export_refs"):
        findings.append(
            Finding(
                "error",
                "record.support_export_refs",
                "rollback or revocation must preserve support export refs",
                record_id,
            )
        )
    return findings


def validate_happy_path(
    plan: dict[str, Any],
    record: dict[str, Any],
    diagnostics: dict[str, Any],
) -> list[Finding]:
    findings: list[Finding] = []
    target = rollback_target_ref(plan)

    reasons = availability_reasons(plan, record, diagnostics)
    for reason in reasons:
        findings.append(
            Finding("error", "happy_path.prior_build_unavailable", reason, record.get("record_id", "<record>"))
        )

    if diagnostics.get("source_install_diagnostics_ref") != plan.get("source_refs", {}).get(
        "install_diagnostics_ref"
    ):
        findings.append(
            Finding(
                "error",
                "diagnostics.source_ref_mismatch",
                "post-rollback diagnostics must quote the plan's install-diagnostics ref",
                str(diagnostics.get("diagnostics_id", "<diagnostics>")),
            )
        )
    if diagnostics.get("rollback_target_exact_build_identity_ref") != target:
        findings.append(
            Finding(
                "error",
                "diagnostics.target_mismatch",
                "post-rollback diagnostics target must be the plan rollback target build",
                str(diagnostics.get("diagnostics_id", "<diagnostics>")),
            )
        )
    if diagnostics.get("superseded_exact_build_identity_ref") != current_build_ref(plan):
        findings.append(
            Finding(
                "error",
                "diagnostics.superseded_mismatch",
                "post-rollback diagnostics must name the superseded current build",
                str(diagnostics.get("diagnostics_id", "<diagnostics>")),
            )
        )
    return findings


def validate_honest_failure(
    plan: dict[str, Any],
    missing_record: dict[str, Any],
    diagnostics: dict[str, Any],
) -> list[Finding]:
    reasons = availability_reasons(plan, missing_record, diagnostics)
    if reasons:
        return []
    return [
        Finding(
            "error",
            "honest_failure.revoked_target_passed",
            "revoked prior-build fixture must resolve to unavailable, not a silent pass",
            str(missing_record.get("record_id", "<record>")),
        )
    ]


@dataclasses.dataclass
class Drill:
    drill_id: str
    mutate: Any


def drill_broken_consistency(record: dict[str, Any]) -> bool:
    record["artifact_graph_consistency"] = "broken"
    return True


def drill_drop_rollback_manifest(record: dict[str, Any]) -> bool:
    record["rollback_manifest_ref"] = None
    return True


def drill_last_known_good_drift(record: dict[str, Any]) -> bool:
    record["last_known_good_ref"] = "build-id:aureline:stable:0.0.0:drifted:release:deadbeef"
    return True


def run_record_drills(
    plan: dict[str, Any],
    record: dict[str, Any],
    diagnostics: dict[str, Any],
) -> tuple[list[dict[str, str]], list[Finding]]:
    results: list[dict[str, str]] = []
    findings: list[Finding] = []
    drills = [
        Drill("broken_consistency_rejected", drill_broken_consistency),
        Drill("dropped_rollback_manifest_rejected", drill_drop_rollback_manifest),
        Drill("last_known_good_drift_rejected", drill_last_known_good_drift),
    ]
    for drill in drills:
        mutated = copy.deepcopy(record)
        drill.mutate(mutated)
        rejected = bool(availability_reasons(plan, mutated, diagnostics))
        results.append({"drill_id": drill.drill_id, "status": "passed" if rejected else "failed"})
        if not rejected:
            findings.append(
                Finding(
                    "error",
                    "negative_drill.not_rejected",
                    f"negative drill {drill.drill_id} did not flip availability to unavailable",
                    drill.drill_id,
                )
            )

    # Unverifiable retained artifact and unresolved diagnostics manifest drills.
    mutated_plan = copy.deepcopy(plan)
    artifacts = mutated_plan.get("retained_prior_artifacts")
    if artifacts:
        artifacts[0]["retention_state"] = "missing_blocked"
        artifacts[0]["verification_state"] = "missing_blocked"
    rejected = bool(availability_reasons(mutated_plan, record, diagnostics))
    results.append(
        {"drill_id": "unverifiable_retained_artifact_rejected", "status": "passed" if rejected else "failed"}
    )
    if not rejected:
        findings.append(
            Finding(
                "error",
                "negative_drill.not_rejected",
                "negative drill unverifiable_retained_artifact_rejected did not flip availability",
                "unverifiable_retained_artifact_rejected",
            )
        )

    mutated_diag = copy.deepcopy(diagnostics)
    rows = mutated_diag.get("rows")
    if rows:
        rows[0]["exact_build"]["manifest_state"] = "reserved"
    rejected = bool(availability_reasons(plan, record, mutated_diag))
    results.append(
        {"drill_id": "unresolved_post_rollback_manifest_rejected", "status": "passed" if rejected else "failed"}
    )
    if not rejected:
        findings.append(
            Finding(
                "error",
                "negative_drill.not_rejected",
                "negative drill unresolved_post_rollback_manifest_rejected did not flip availability",
                "unresolved_post_rollback_manifest_rejected",
            )
        )
    return results, findings


def write_report(
    path: Path,
    plan: dict[str, Any],
    findings: list[Finding],
    drill_results: list[dict[str, str]],
) -> None:
    path.parent.mkdir(parents=True, exist_ok=True)
    payload = {
        "schema_version": 1,
        "record_kind": "rollback_drill_beta_validation_capture",
        "status": "pass" if not any(f.severity == "error" for f in findings) else "fail",
        "generated_at": generated_at_now(),
        "plan_id": plan.get("plan_id"),
        "rollback_target_exact_build_identity_ref": plan.get("rollback_target", {}).get(
            "exact_build_identity_ref"
        ),
        "negative_drills": drill_results,
        "findings": [f.as_report() for f in findings],
    }
    path.write_text(json_text(payload), encoding="utf-8")


def main() -> int:
    args = parse_args()
    repo_root = Path(args.repo_root).resolve()
    fixtures_dir = repo_root / args.fixtures_dir

    plan = ensure_dict(load_json(repo_root / args.plan), "plan")
    healthy_record = ensure_dict(
        load_json(fixtures_dir / HEALTHY_RECORD_NAME), "healthy_record"
    )
    missing_record = ensure_dict(
        load_json(fixtures_dir / MISSING_PRIOR_BUILD_RECORD_NAME), "missing_prior_build_record"
    )
    diagnostics = ensure_dict(
        load_json(fixtures_dir / POST_ROLLBACK_DIAGNOSTICS_NAME), "post_rollback_diagnostics"
    )

    findings: list[Finding] = []
    findings.extend(validate_release_center_record(healthy_record))
    findings.extend(validate_release_center_record(missing_record))
    findings.extend(validate_happy_path(plan, healthy_record, diagnostics))
    findings.extend(validate_honest_failure(plan, missing_record, diagnostics))
    drill_results, drill_findings = run_record_drills(plan, healthy_record, diagnostics)
    findings.extend(drill_findings)

    report_rel = args.report
    if report_rel is None and args.check:
        report_rel = DEFAULT_REPORT_REL
    if report_rel:
        write_report(repo_root / report_rel, plan, findings, drill_results)

    errors = [f for f in findings if f.severity == "error"]
    if errors:
        for item in errors:
            print(f"ERROR [{item.check_id}] ({item.ref}): {item.message}", file=sys.stderr)
        return 1

    print(
        "beta rollback drill validated "
        f"(prior build available for {plan.get('plan_id')}, "
        f"{len(drill_results)} negative drills rejected)"
    )
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
