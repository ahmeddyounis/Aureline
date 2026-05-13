#!/usr/bin/env python3
"""Validate the activity-center alpha contract, fixtures, and consumers."""

from __future__ import annotations

import argparse
import datetime as dt
import json
import sys
from dataclasses import asdict, dataclass, field
from pathlib import Path
from typing import Any


DEFAULT_SCHEMA_REL = "schemas/events/activity_row.schema.json"
DEFAULT_SNAPSHOT_REL = "fixtures/ux/activity_center_alpha/activity_center_alpha_snapshot.json"
DEFAULT_SUPPORT_EXPORT_REL = "fixtures/ux/activity_center_alpha/support_export_activity_rows.json"
DEFAULT_RUNTIME_REL = "crates/aureline-shell/src/activity_center/alpha.rs"
DEFAULT_SUPPORT_CONSUMER_REL = "crates/aureline-shell/src/support_seed/mod.rs"
DEFAULT_DOC_REL = "docs/ux/activity_center_alpha.md"

REQUIRED_FAMILIES = {"indexing", "restore", "install_update", "task_run", "test_run"}
REQUIRED_STATES = {"running", "queued_waiting", "partially_completed", "failed", "completed"}
REQUIRED_PARTITIONS = {"current_work", "needs_attention", "completed"}
REQUIRED_RUNTIME_MARKERS = {
    "ActivityCenterAlphaRuntime",
    "ActivityCenterAlphaStore",
    "ActivityCenterSupportExport",
    "has_exact_reopen_identity",
    "satisfies_sensitive_detail_rule",
}
REQUIRED_SUPPORT_MARKERS = {
    "activity_center_preview",
    "activity_center_seed",
    "support.item.activity_center_alpha_rows",
}


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
    parser.add_argument("--schema", default=DEFAULT_SCHEMA_REL)
    parser.add_argument("--snapshot", default=DEFAULT_SNAPSHOT_REL)
    parser.add_argument("--support-export", default=DEFAULT_SUPPORT_EXPORT_REL)
    parser.add_argument("--runtime", default=DEFAULT_RUNTIME_REL)
    parser.add_argument("--support-consumer", default=DEFAULT_SUPPORT_CONSUMER_REL)
    parser.add_argument("--docs", default=DEFAULT_DOC_REL)
    parser.add_argument("--report", default=None)
    return parser.parse_args()


def load_json(path: Path) -> dict[str, Any]:
    if not path.exists():
        raise SystemExit(f"missing JSON file: {path}")
    try:
        with path.open("r", encoding="utf-8") as handle:
            payload = json.load(handle)
    except json.JSONDecodeError as exc:
        raise SystemExit(f"invalid JSON at {path}: {exc}") from exc
    if not isinstance(payload, dict):
        raise SystemExit(f"{path} must contain a JSON object")
    return payload


def validate_path(path: Path, label: str, findings: list[Finding]) -> None:
    if not path.exists():
        findings.append(
            Finding(
                severity="error",
                check_id=f"{label}.missing",
                message=f"{label} does not exist: {path}",
                remediation="Create the required activity-center alpha artifact.",
                ref=str(path),
            )
        )


def ensure(condition: bool, finding: Finding, findings: list[Finding]) -> None:
    if not condition:
        findings.append(finding)


def validate_runtime(path: Path, markers: set[str], label: str, findings: list[Finding]) -> None:
    validate_path(path, label, findings)
    if not path.exists():
        return
    text = path.read_text(encoding="utf-8")
    missing = sorted(marker for marker in markers if marker not in text)
    if missing:
        findings.append(
            Finding(
                severity="error",
                check_id=f"{label}.missing_markers",
                message=f"{label} is missing required consumer markers",
                remediation="Wire the runtime/support consumer to the activity-center alpha contract.",
                ref=str(path),
                details={"missing": missing},
            )
        )


def validate_schema(schema: dict[str, Any], findings: list[Finding]) -> None:
    ensure(
        schema.get("$id") == "https://aureline.dev/schemas/events/activity_row.schema.json",
        Finding(
            severity="error",
            check_id="schema.id",
            message="activity row schema id is not the canonical events schema",
            remediation="Restore the canonical $id.",
        ),
        findings,
    )
    defs = schema.get("$defs")
    ensure(
        isinstance(defs, dict)
        and "activity_row_record" in defs
        and "activity_center_alpha_snapshot_record" in defs
        and "activity_center_support_export_record" in defs,
        Finding(
            severity="error",
            check_id="schema.defs",
            message="schema must define row, snapshot, and support-export records",
            remediation="Add the missing record definitions.",
        ),
        findings,
    )


def validate_snapshot(snapshot: dict[str, Any], findings: list[Finding]) -> None:
    ensure(
        snapshot.get("record_kind") == "activity_center_alpha_snapshot_record",
        Finding(
            severity="error",
            check_id="snapshot.record_kind",
            message="snapshot record_kind must be activity_center_alpha_snapshot_record",
            remediation="Regenerate the fixture from the alpha snapshot model.",
        ),
        findings,
    )
    rows = snapshot.get("rows")
    if not isinstance(rows, list) or not rows:
        findings.append(
            Finding(
                severity="error",
                check_id="snapshot.rows.empty",
                message="snapshot must contain activity rows",
                remediation="Add the protected activity-center rows.",
            )
        )
        return

    families = {row.get("job_family") for row in rows if isinstance(row, dict)}
    missing_families = REQUIRED_FAMILIES - families
    if missing_families:
        findings.append(
            Finding(
                severity="error",
                check_id="snapshot.families.missing",
                message="snapshot does not cover all required alpha job families",
                remediation="Add rows for the missing families.",
                details={"missing": sorted(missing_families)},
            )
        )

    states = {row.get("state_class") for row in rows if isinstance(row, dict)}
    missing_states = REQUIRED_STATES - states
    if missing_states:
        findings.append(
            Finding(
                severity="error",
                check_id="snapshot.states.missing",
                message="snapshot misses required state coverage",
                remediation="Add rows covering current, failed/partial, and completed states.",
                details={"missing": sorted(missing_states)},
            )
        )

    partitions = {row.get("activity_partition") for row in rows if isinstance(row, dict)}
    missing_partitions = REQUIRED_PARTITIONS - partitions
    if missing_partitions:
        findings.append(
            Finding(
                severity="error",
                check_id="snapshot.partitions.missing",
                message="snapshot misses required activity partitions",
                remediation="Add current, needs-attention, and completed rows.",
                details={"missing": sorted(missing_partitions)},
            )
        )

    exact_rows = 0
    exportable_rows = 0
    sensitive_rows_with_detail = 0
    retry_rows = 0
    cancel_rows = 0
    stable_ids: set[str] = set()
    for idx, row in enumerate(rows):
        if not isinstance(row, dict):
            findings.append(
                Finding(
                    severity="error",
                    check_id="snapshot.rows.invalid",
                    message=f"row {idx} must be an object",
                    remediation="Regenerate the fixture as activity row records.",
                )
            )
            continue
        row_id = row.get("activity_row_id")
        job_id = row.get("durable_job_id")
        event_id = row.get("canonical_event_id")
        for field_name, value in (
            ("activity_row_id", row_id),
            ("durable_job_id", job_id),
            ("canonical_event_id", event_id),
            ("actor_identity_ref", row.get("actor_identity_ref")),
            ("actor_or_subsystem_label", row.get("actor_or_subsystem_label")),
        ):
            if not isinstance(value, str) or not value:
                findings.append(
                    Finding(
                        severity="error",
                        check_id=f"snapshot.row.{field_name}.missing",
                        message=f"row {idx} must carry {field_name}",
                        remediation="Populate stable row, job, event, and actor fields.",
                        details={"row_index": idx},
                    )
                )
        if isinstance(row_id, str):
            if row_id in stable_ids:
                findings.append(
                    Finding(
                        severity="error",
                        check_id="snapshot.row.duplicate_id",
                        message=f"duplicate activity row id: {row_id}",
                        remediation="Use stable unique row ids.",
                        ref=row_id,
                    )
                )
            stable_ids.add(row_id)
        reopen = row.get("reopen_target")
        exact_ref = row.get("exact_reopen_identity_ref")
        if isinstance(reopen, dict) and reopen.get("exact_target_identity_ref") == exact_ref:
            exact_rows += 1
        actions = row.get("actions") if isinstance(row.get("actions"), list) else []
        action_kinds = {action.get("action_kind") for action in actions if isinstance(action, dict)}
        if "open_details" not in action_kinds:
            findings.append(
                Finding(
                    severity="error",
                    check_id="snapshot.row.open_details.missing",
                    message=f"row {idx} has no open-details action",
                    remediation="Add command-backed open_details to every activity row.",
                    details={"row_id": row_id},
                )
            )
        if "retry_job" in action_kinds:
            retry_rows += 1
            for action in actions:
                if (
                    isinstance(action, dict)
                    and action.get("action_kind") == "retry_job"
                    and action.get("reissues_original_side_effect") is not True
                ):
                    findings.append(
                        Finding(
                            severity="error",
                            check_id="snapshot.row.retry.replay_flag",
                            message="retry actions must declare that they start reviewed new work",
                            remediation="Set reissues_original_side_effect=true on retry actions.",
                            ref=str(row_id),
                        )
                    )
        if "cancel_job" in action_kinds:
            cancel_rows += 1
        progress = row.get("progress") if isinstance(row.get("progress"), dict) else {}
        impact = row.get("impact") if isinstance(row.get("impact"), dict) else {}
        sensitive = any(
            impact.get(flag) is True
            for flag in (
                "affects_cost",
                "affects_policy",
                "affects_network",
                "affects_trust",
                "affects_provider_state",
                "affects_recovery_posture",
            )
        )
        if sensitive:
            if impact.get("detail_or_evidence_required") is True and progress.get("detail_or_evidence_ref"):
                sensitive_rows_with_detail += 1
            else:
                findings.append(
                    Finding(
                        severity="error",
                        check_id="snapshot.row.sensitive_detail.missing",
                        message="sensitive-impact rows must carry detail/evidence refs",
                        remediation="Add detail_or_evidence_required and progress.detail_or_evidence_ref.",
                        ref=str(row_id),
                    )
                )
        support = row.get("support_link") if isinstance(row.get("support_link"), dict) else {}
        if support.get("exportable") is True:
            exportable_rows += 1
            if support.get("raw_private_material_excluded") is not True:
                findings.append(
                    Finding(
                        severity="error",
                        check_id="snapshot.row.support.raw_material",
                        message="support-exportable rows must exclude raw private material",
                        remediation="Set raw_private_material_excluded=true and export only structured refs.",
                        ref=str(row_id),
                    )
                )

    ensure(
        exact_rows == len(rows),
        Finding(
            severity="error",
            check_id="snapshot.exact_reopen.count",
            message="every row must preserve exact reopen identity",
            remediation="Set reopen_target.exact_target_identity_ref to the row's exact reopen identity.",
            details={"exact_rows": exact_rows, "row_count": len(rows)},
        ),
        findings,
    )
    ensure(
        exportable_rows >= 1,
        Finding(
            severity="error",
            check_id="snapshot.support_export.none",
            message="at least one row must be exportable to support artifacts",
            remediation="Mark a meaningful row exportable and provide support item refs.",
        ),
        findings,
    )
    ensure(
        sensitive_rows_with_detail >= 1,
        Finding(
            severity="error",
            check_id="snapshot.sensitive_coverage.none",
            message="snapshot must prove impact flags with evidence detail",
            remediation="Add a policy/network/trust/recovery-affecting row with evidence.",
        ),
        findings,
    )
    ensure(
        retry_rows >= 1 and cancel_rows >= 1,
        Finding(
            severity="error",
            check_id="snapshot.actions.coverage",
            message="snapshot must cover retry and cancel postures",
            remediation="Add at least one retry row and one cancellable row.",
            details={"retry_rows": retry_rows, "cancel_rows": cancel_rows},
        ),
        findings,
    )


def validate_support_export(
    snapshot: dict[str, Any], support_export: dict[str, Any], findings: list[Finding]
) -> None:
    ensure(
        support_export.get("record_kind") == "activity_center_support_export_record",
        Finding(
            severity="error",
            check_id="support_export.record_kind",
            message="support export record_kind must be activity_center_support_export_record",
            remediation="Regenerate the support-export fixture.",
        ),
        findings,
    )
    rows = support_export.get("rows")
    if not isinstance(rows, list) or not rows:
        findings.append(
            Finding(
                severity="error",
                check_id="support_export.rows.empty",
                message="support export must include at least one structured row",
                remediation="Export support-safe activity rows.",
            )
        )
        return
    snapshot_ids = {
        row.get("activity_row_id")
        for row in snapshot.get("rows", [])
        if isinstance(row, dict)
    }
    for idx, row in enumerate(rows):
        if not isinstance(row, dict):
            continue
        row_id = row.get("activity_row_id")
        if row_id not in snapshot_ids:
            findings.append(
                Finding(
                    severity="error",
                    check_id="support_export.row.unknown",
                    message=f"support export row does not map to snapshot row: {row_id}",
                    remediation="Export rows from the canonical activity snapshot only.",
                    ref=str(row_id),
                )
            )
        if row.get("raw_private_material_excluded") is not True:
            findings.append(
                Finding(
                    severity="error",
                    check_id="support_export.row.raw_material",
                    message=f"support export row {idx} does not exclude raw private material",
                    remediation="Keep support export metadata-only or redacted.",
                    ref=str(row_id),
                )
            )
    exported_families = {row.get("job_family") for row in rows if isinstance(row, dict)}
    ensure(
        "test_run" in exported_families or "restore" in exported_families,
        Finding(
            severity="error",
            check_id="support_export.meaningful_family",
            message="support export must include a meaningful task/test or restore family",
            remediation="Export a reviewable failed test or restore row.",
        ),
        findings,
    )


def write_report(path: Path, findings: list[Finding]) -> None:
    payload = {
        "record_kind": "activity_center_alpha_validation_capture",
        "schema_version": 1,
        "captured_at": dt.datetime.now(dt.timezone.utc).replace(microsecond=0).isoformat().replace("+00:00", "Z"),
        "status": "pass" if not findings else "fail",
        "finding_count": len(findings),
        "findings": [finding.as_report() for finding in findings],
    }
    path.parent.mkdir(parents=True, exist_ok=True)
    path.write_text(json.dumps(payload, indent=2) + "\n", encoding="utf-8")


def main() -> int:
    args = parse_args()
    repo_root = Path(args.repo_root).resolve()
    findings: list[Finding] = []

    schema_path = repo_root / args.schema
    snapshot_path = repo_root / args.snapshot
    support_export_path = repo_root / args.support_export
    docs_path = repo_root / args.docs

    for label, path in (
        ("schema", schema_path),
        ("snapshot", snapshot_path),
        ("support_export", support_export_path),
        ("docs", docs_path),
    ):
        validate_path(path, label, findings)

    validate_runtime(repo_root / args.runtime, REQUIRED_RUNTIME_MARKERS, "runtime", findings)
    validate_runtime(
        repo_root / args.support_consumer,
        REQUIRED_SUPPORT_MARKERS,
        "support_consumer",
        findings,
    )

    if schema_path.exists():
        validate_schema(load_json(schema_path), findings)
    snapshot = load_json(snapshot_path) if snapshot_path.exists() else {}
    support_export = load_json(support_export_path) if support_export_path.exists() else {}
    if snapshot:
        validate_snapshot(snapshot, findings)
    if snapshot and support_export:
        validate_support_export(snapshot, support_export, findings)

    if args.report:
        write_report(repo_root / args.report, findings)

    if findings:
        print(f"[activity-center-alpha] FAIL ({len(findings)} findings)")
        for finding in findings:
            print(f"[activity-center-alpha] {finding.severity.upper()} {finding.check_id}: {finding.message}")
            print(f"[activity-center-alpha]   remediation: {finding.remediation}")
        return 1
    print("[activity-center-alpha] PASS")
    return 0


if __name__ == "__main__":
    sys.exit(main())
