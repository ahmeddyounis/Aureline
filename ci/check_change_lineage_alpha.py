#!/usr/bin/env python3
"""Validate the alpha change-lineage schema, fixtures, and consumer wiring."""

from __future__ import annotations

import argparse
import json
import sys
from dataclasses import asdict, dataclass, field
from pathlib import Path
from typing import Any

from jsonschema import Draft202012Validator, FormatChecker


DEFAULT_SCHEMA_REL = "schemas/review/change_lineage.schema.json"
DEFAULT_FIXTURE_DIR_REL = "fixtures/review/m3/change_lineage"
DEFAULT_DOC_REL = "docs/ux/m3/change_lineage_alpha.md"
DEFAULT_CONSUMER_REL = "crates/aureline-shell/src/review/change_inspector/mod.rs"
DEFAULT_CRATE_MODULE_REL = "crates/aureline-review/src/change_inspector/mod.rs"
DEFAULT_CHANGE_OBJECT_FIXTURE_DIR_REL = "fixtures/workspace/m3/change_objects"

REQUIRED_KINDS = {"branch", "worktree", "patch_stack"}
REQUIRED_SCOPES = {"main_worktree", "side_worktree", "stacked_patch_set"}
REQUIRED_READINESS = {
    "ready_to_publish",
    "blocked_by_conflicts",
    "blocked_by_review_required",
    "not_applicable_inspect_only",
}
REQUIRED_CONSUMER = "change_inspector"


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
    parser.add_argument("--fixture-dir", default=DEFAULT_FIXTURE_DIR_REL)
    parser.add_argument("--doc", default=DEFAULT_DOC_REL)
    parser.add_argument("--consumer", default=DEFAULT_CONSUMER_REL)
    parser.add_argument("--crate-module", default=DEFAULT_CRATE_MODULE_REL)
    parser.add_argument(
        "--change-object-fixture-dir",
        default=DEFAULT_CHANGE_OBJECT_FIXTURE_DIR_REL,
    )
    parser.add_argument("--report", default=None)
    parser.add_argument(
        "--render-gallery",
        action="store_true",
        help="Print the deterministic landing-state inspector projection after validation.",
    )
    return parser.parse_args()


def load_json(path: Path) -> Any:
    try:
        return json.loads(path.read_text(encoding="utf-8"))
    except FileNotFoundError as exc:
        raise SystemExit(f"missing JSON file: {path}") from exc
    except json.JSONDecodeError as exc:
        raise SystemExit(f"invalid JSON at {path}: {exc}") from exc


def schema_validate(
    schema: dict[str, Any], label: str, payload: dict[str, Any]
) -> list[Finding]:
    Draft202012Validator.check_schema(schema)
    validator = Draft202012Validator(schema, format_checker=FormatChecker())
    findings: list[Finding] = []
    for error in sorted(validator.iter_errors(payload), key=lambda item: list(item.path)):
        path = ".".join(str(part) for part in error.path) or "<root>"
        findings.append(
            Finding(
                severity="error",
                check_id="change_lineage_alpha.schema.validation_failed",
                message=f"{label}: {path}: {error.message}",
                remediation=(
                    "Fix the change-lineage record to validate against "
                    "schemas/review/change_lineage.schema.json."
                ),
                ref=label,
                details={"schema_path": list(error.schema_path)},
            )
        )
    return findings


def cross_check_record(label: str, record: dict[str, Any]) -> list[Finding]:
    findings: list[Finding] = []
    kind = record.get("change_object_kind")
    scope = record.get("active_scope_class")
    target = record.get("target_summary", {})
    conflict = record.get("conflict_state", {})
    readiness = record.get("publish_readiness", {})

    if kind == "branch" and scope == "stacked_patch_set":
        findings.append(
            Finding(
                severity="error",
                check_id="change_lineage.scope.branch_stacked_invalid",
                message=f"{label} branch change-object must not open the stacked_patch_set scope",
                remediation="Use main_worktree or side_worktree for branch records.",
                ref=label,
            )
        )
    if kind == "worktree" and scope not in {
        "main_worktree",
        "side_worktree",
        "active_scope_unknown_requires_review",
    }:
        findings.append(
            Finding(
                severity="error",
                check_id="change_lineage.scope.worktree_invalid",
                message=f"{label} worktree change-object must open a main or side worktree scope",
                remediation="Use main_worktree or side_worktree for worktree records.",
                ref=label,
            )
        )
    if kind == "patch_stack" and scope not in {
        "stacked_patch_set",
        "active_scope_unknown_requires_review",
    }:
        findings.append(
            Finding(
                severity="error",
                check_id="change_lineage.scope.patch_stack_invalid",
                message=f"{label} patch_stack change-object must open the stacked_patch_set scope",
                remediation="Use stacked_patch_set for patch-stack records.",
                ref=label,
            )
        )

    readiness_class = readiness.get("publish_readiness_class")
    conflict_class = conflict.get("conflict_state_class")
    landing_action = target.get("landing_action_class")
    if readiness_class in {"ready_to_publish", "ready_to_merge", "ready_to_apply"}:
        if conflict_class != "no_conflicts_detected":
            findings.append(
                Finding(
                    severity="error",
                    check_id="change_lineage.readiness.ready_with_conflicts",
                    message=(
                        f"{label} {readiness_class} must pair with conflict_state_class=no_conflicts_detected"
                    ),
                    remediation="Resolve the declared conflict before claiming a ready-to-* readiness class.",
                    ref=label,
                )
            )
    if readiness_class == "ready_to_publish" and landing_action != "publish":
        findings.append(
            Finding(
                severity="error",
                check_id="change_lineage.readiness.publish_action_mismatch",
                message=f"{label} ready_to_publish must declare landing_action_class=publish",
                remediation="Set target_summary.landing_action_class to publish.",
                ref=label,
            )
        )
    if readiness_class == "ready_to_merge" and landing_action != "merge":
        findings.append(
            Finding(
                severity="error",
                check_id="change_lineage.readiness.merge_action_mismatch",
                message=f"{label} ready_to_merge must declare landing_action_class=merge",
                remediation="Set target_summary.landing_action_class to merge.",
                ref=label,
            )
        )
    if readiness_class == "ready_to_apply" and landing_action != "apply":
        findings.append(
            Finding(
                severity="error",
                check_id="change_lineage.readiness.apply_action_mismatch",
                message=f"{label} ready_to_apply must declare landing_action_class=apply",
                remediation="Set target_summary.landing_action_class to apply.",
                ref=label,
            )
        )
    if readiness_class == "not_applicable_inspect_only" and landing_action != "inspect_only":
        findings.append(
            Finding(
                severity="error",
                check_id="change_lineage.readiness.inspect_only_action_mismatch",
                message=f"{label} not_applicable_inspect_only must declare landing_action_class=inspect_only",
                remediation="Set target_summary.landing_action_class to inspect_only.",
                ref=label,
            )
        )
    if readiness_class == "blocked_by_conflicts" and conflict_class == "no_conflicts_detected":
        findings.append(
            Finding(
                severity="error",
                check_id="change_lineage.readiness.blocked_without_conflict",
                message=f"{label} blocked_by_conflicts must report a non-no_conflicts_detected conflict state",
                remediation="Either change readiness or declare the actual conflict state.",
                ref=label,
            )
        )

    ready_classes = {
        "ready_to_publish",
        "ready_to_merge",
        "ready_to_apply",
        "not_applicable_inspect_only",
    }
    blocker_list = readiness.get("blockers", [])
    if readiness_class in ready_classes and any(b != "no_blockers" for b in blocker_list):
        findings.append(
            Finding(
                severity="error",
                check_id="change_lineage.readiness.ready_with_blocker",
                message=(
                    f"{label} {readiness_class} must list only the no_blockers token"
                ),
                remediation="Carry exactly the no_blockers token for ready-to-* and inspect-only readiness.",
                ref=label,
            )
        )
    blocked_classes = {
        "blocked_by_conflicts",
        "blocked_by_review_required",
        "blocked_by_authority",
        "readiness_unknown_requires_review",
    }
    if readiness_class in blocked_classes and (
        not blocker_list or all(b == "no_blockers" for b in blocker_list)
    ):
        findings.append(
            Finding(
                severity="error",
                check_id="change_lineage.readiness.blocked_without_blocker",
                message=(
                    f"{label} {readiness_class} must declare at least one real readiness blocker"
                ),
                remediation="Declare the actual readiness blocker tokens.",
                ref=label,
            )
        )

    conflict_path_count = conflict.get("conflict_path_count")
    if conflict_class == "no_conflicts_detected" and conflict_path_count not in (None, 0):
        findings.append(
            Finding(
                severity="error",
                check_id="change_lineage.conflict.path_count_inconsistent",
                message=(
                    f"{label} no_conflicts_detected must report conflict_path_count=0"
                ),
                remediation="Set conflict_path_count to 0 when no conflicts are declared.",
                ref=label,
            )
        )
    if (
        conflict_class
        in {
            "merge_conflicts_pending_review",
            "rebase_conflicts_pending_review",
            "apply_conflicts_pending_review",
        }
        and (conflict_path_count is None or conflict_path_count == 0)
    ):
        findings.append(
            Finding(
                severity="error",
                check_id="change_lineage.conflict.path_count_missing",
                message=(
                    f"{label} pending-conflict classes must report conflict_path_count > 0"
                ),
                remediation="Quote the conflict path count when conflicts are pending review.",
                ref=label,
            )
        )

    consumers = set(record.get("consumer_surfaces", []))
    if REQUIRED_CONSUMER not in consumers:
        findings.append(
            Finding(
                severity="error",
                check_id="change_lineage.consumer.inspector_missing",
                message=f"{label} consumer_surfaces does not include {REQUIRED_CONSUMER}",
                remediation="Wire change_inspector as a consumer surface.",
                ref=label,
            )
        )

    invariants = record.get("review_invariants", {})
    for key in (
        "target_ref_pinned",
        "ancestry_pinned",
        "conflict_state_inspectable",
        "publish_readiness_inspectable",
        "no_hidden_target_mutation",
    ):
        if invariants.get(key) is not True:
            findings.append(
                Finding(
                    severity="error",
                    check_id=f"change_lineage.review.{key}_missing",
                    message=f"{label} review_invariants.{key} must be true",
                    remediation="The record is a pre-execution review record; keep every review invariant true.",
                    ref=label,
                )
            )

    support_export = record.get("support_export", {})
    for key in (
        "raw_path_export_allowed",
        "raw_branch_name_export_allowed",
        "raw_remote_url_export_allowed",
        "raw_diff_body_export_allowed",
    ):
        if support_export.get(key) is not False:
            findings.append(
                Finding(
                    severity="error",
                    check_id=f"change_lineage.support_export.{key}_widened",
                    message=f"{label} support_export.{key} must be false",
                    remediation="Keep raw path, branch-name, remote URL, and diff-body export closed.",
                    ref=label,
                )
            )

    ancestry = record.get("ancestry_view", {})
    seen: set[str] = set()
    for entry in ancestry.get("ancestor_chain", []):
        ancestor_ref = entry.get("ancestor_ref")
        if ancestor_ref in seen:
            findings.append(
                Finding(
                    severity="error",
                    check_id="change_lineage.lineage.duplicate_ancestor",
                    message=(
                        f"{label} ancestry_view.ancestor_chain duplicates ancestor_ref {ancestor_ref}"
                    ),
                    remediation="Ancestor refs must be unique within the lineage chain.",
                    ref=label,
                )
            )
        elif ancestor_ref:
            seen.add(ancestor_ref)

    return findings


def collect_records(fixture_dir: Path) -> dict[str, dict[str, Any]]:
    if not fixture_dir.exists():
        raise SystemExit(f"missing fixture dir: {fixture_dir}")
    records: dict[str, dict[str, Any]] = {}
    for path in sorted(fixture_dir.glob("*.json")):
        records[path.name] = load_json(path)
    if not records:
        raise SystemExit(f"no fixtures in {fixture_dir}")
    return records


def validate_coverage(records: dict[str, dict[str, Any]]) -> list[Finding]:
    findings: list[Finding] = []
    kinds = {record.get("change_object_kind") for record in records.values()}
    missing_kinds = REQUIRED_KINDS - kinds
    if missing_kinds:
        findings.append(
            Finding(
                severity="error",
                check_id="fixtures.coverage.kind_missing",
                message="fixtures must cover every change-object kind",
                remediation="Seed a change-lineage fixture for each change-object kind.",
                details={"missing": sorted(missing_kinds)},
            )
        )
    scopes = {record.get("active_scope_class") for record in records.values()}
    missing_scopes = REQUIRED_SCOPES - scopes
    if missing_scopes:
        findings.append(
            Finding(
                severity="error",
                check_id="fixtures.coverage.scope_missing",
                message="fixtures must cover every active_scope_class so users can tell where they are",
                remediation="Seed fixtures covering main_worktree, side_worktree, and stacked_patch_set.",
                details={"missing": sorted(missing_scopes)},
            )
        )
    readiness = {
        record.get("publish_readiness", {}).get("publish_readiness_class")
        for record in records.values()
    }
    missing_readiness = REQUIRED_READINESS - readiness
    if missing_readiness:
        findings.append(
            Finding(
                severity="error",
                check_id="fixtures.coverage.readiness_missing",
                message="fixtures must cover every required publish-readiness class",
                remediation="Seed fixtures covering ready_to_publish, blocked_by_conflicts, blocked_by_review_required, and not_applicable_inspect_only.",
                details={"missing": sorted(missing_readiness)},
            )
        )
    return findings


def validate_doc_and_consumer(
    doc_path: Path, consumer_path: Path, crate_module_path: Path
) -> list[Finding]:
    findings: list[Finding] = []
    doc = doc_path.read_text(encoding="utf-8") if doc_path.exists() else ""
    if not doc:
        findings.append(
            Finding(
                severity="error",
                check_id="docs.change_lineage.missing",
                message=f"change-lineage reviewer doc is missing: {doc_path}",
                remediation="Author docs/ux/m3/change_lineage_alpha.md.",
                ref=str(doc_path),
            )
        )
    else:
        for ref in [
            DEFAULT_SCHEMA_REL,
            DEFAULT_FIXTURE_DIR_REL,
            "ci/check_change_lineage_alpha.py",
            DEFAULT_CONSUMER_REL,
            DEFAULT_CRATE_MODULE_REL,
        ]:
            if ref not in doc:
                findings.append(
                    Finding(
                        severity="error",
                        check_id="docs.change_lineage.missing_ref",
                        message=f"change-lineage reviewer doc does not mention {ref}",
                        remediation="Document the schema, fixtures, validator, crate module, and first consumer together.",
                        ref=str(doc_path),
                    )
                )

    consumer = (
        consumer_path.read_text(encoding="utf-8") if consumer_path.exists() else ""
    )
    if not consumer:
        findings.append(
            Finding(
                severity="error",
                check_id="consumer.change_lineage.missing",
                message=f"shell landing-state inspector consumer is missing: {consumer_path}",
                remediation="Wire the first shell consumer at crates/aureline-shell/src/review/change_inspector/mod.rs.",
                ref=str(consumer_path),
            )
        )
    else:
        for token in [
            "build_alpha_change_lineage_rows",
            "render_alpha_change_lineage_plaintext",
            "project_change_lineage",
        ]:
            if token not in consumer:
                findings.append(
                    Finding(
                        severity="error",
                        check_id="consumer.change_lineage.missing_token",
                        message=f"shell landing-state inspector does not contain {token}",
                        remediation="Keep the alpha change-lineage family wired into the first shell consumer.",
                        ref=str(consumer_path),
                    )
                )

    crate_module = (
        crate_module_path.read_text(encoding="utf-8")
        if crate_module_path.exists()
        else ""
    )
    if not crate_module:
        findings.append(
            Finding(
                severity="error",
                check_id="crate.change_lineage.missing",
                message=f"aureline-review change_inspector module is missing: {crate_module_path}",
                remediation="Add the change_inspector module to aureline-review.",
                ref=str(crate_module_path),
            )
        )
    else:
        for token in [
            "ChangeLineageRecord",
            "project_change_lineage",
            "CHANGE_LINEAGE_ALPHA_RECORD_KIND",
            "CHANGE_LINEAGE_PUBLISH_READINESS_CLASSES",
        ]:
            if token not in crate_module:
                findings.append(
                    Finding(
                        severity="error",
                        check_id="crate.change_lineage.missing_token",
                        message=f"aureline-review change_inspector module does not export {token}",
                        remediation="Keep the alpha change-lineage vocabulary exported from aureline-review.",
                        ref=str(crate_module_path),
                    )
                )
    return findings


def validate_change_object_back_references(
    records: dict[str, dict[str, Any]], change_object_dir: Path
) -> list[Finding]:
    findings: list[Finding] = []
    if not change_object_dir.exists():
        return findings
    known_ids: set[str] = set()
    for path in change_object_dir.glob("*.json"):
        try:
            payload = json.loads(path.read_text(encoding="utf-8"))
        except json.JSONDecodeError:
            continue
        change_object_id = payload.get("change_object_id")
        if isinstance(change_object_id, str):
            known_ids.add(change_object_id)
    for label, record in records.items():
        ref = record.get("change_object_ref")
        if not isinstance(ref, str):
            continue
        if known_ids and ref not in known_ids:
            findings.append(
                Finding(
                    severity="error",
                    check_id="change_lineage.change_object_ref.unknown",
                    message=(
                        f"{label} change_object_ref {ref} does not match any checked-in change-object fixture"
                    ),
                    remediation=(
                        "Quote a change-object id from "
                        "fixtures/workspace/m3/change_objects/."
                    ),
                    ref=label,
                )
            )
    return findings


def render_gallery(records: dict[str, dict[str, Any]]) -> str:
    lines = [
        "Landing-state inspector alpha gallery",
        "change_lineage_id | kind | scope | landing_state/action | conflict_state | publish_readiness | blockers",
    ]
    for name in sorted(records):
        record = records[name]
        target = record.get("target_summary", {})
        conflict = record.get("conflict_state", {})
        readiness = record.get("publish_readiness", {})
        blockers = ",".join(readiness.get("blockers", [])) or "none"
        lines.append(
            " | ".join(
                [
                    record.get("change_lineage_id", name),
                    record.get("change_object_kind", "?"),
                    record.get("active_scope_class", "?"),
                    f"{target.get('landing_state_class', '?')}/{target.get('landing_action_class', '?')}",
                    conflict.get("conflict_state_class", "?"),
                    readiness.get("publish_readiness_class", "?"),
                    blockers,
                ]
            )
        )
    return "\n".join(lines) + "\n"


def main() -> int:
    args = parse_args()
    repo_root = Path(args.repo_root).resolve()
    schema_path = repo_root / args.schema
    fixture_dir = repo_root / args.fixture_dir
    doc_path = repo_root / args.doc
    consumer_path = repo_root / args.consumer
    crate_module_path = repo_root / args.crate_module
    change_object_dir = repo_root / args.change_object_fixture_dir

    schema = load_json(schema_path)
    records = collect_records(fixture_dir)

    findings: list[Finding] = []
    for name, record in records.items():
        validation_payload = {
            key: value for key, value in record.items() if key != "$schema"
        }
        findings.extend(schema_validate(schema, name, validation_payload))
        findings.extend(cross_check_record(name, record))

    findings.extend(validate_coverage(records))
    findings.extend(validate_doc_and_consumer(doc_path, consumer_path, crate_module_path))
    findings.extend(validate_change_object_back_references(records, change_object_dir))

    report = {
        "status": "pass" if not findings else "fail",
        "schema_ref": args.schema,
        "fixture_dir": args.fixture_dir,
        "record_count": len(records),
        "findings": [finding.as_report() for finding in findings],
    }
    if args.report:
        Path(args.report).write_text(
            json.dumps(report, indent=2) + "\n", encoding="utf-8"
        )
    if args.render_gallery:
        print(render_gallery(records), end="")
    if findings:
        print(json.dumps(report, indent=2), file=sys.stderr)
        return 1
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
