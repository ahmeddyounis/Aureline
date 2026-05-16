#!/usr/bin/env python3
"""Validate the alpha change-object schema, fixtures, and consumer wiring."""

from __future__ import annotations

import argparse
import json
import sys
from dataclasses import asdict, dataclass, field
from pathlib import Path
from typing import Any

from jsonschema import Draft202012Validator, FormatChecker


DEFAULT_SCHEMA_REL = "schemas/workspace/change_object.schema.json"
DEFAULT_FIXTURE_DIR_REL = "fixtures/workspace/m3/change_objects"
DEFAULT_DOC_REL = "docs/review/m3/change_objects_alpha.md"
DEFAULT_CONSUMER_REL = "crates/aureline-shell/src/change_object_inspector/mod.rs"
DEFAULT_CRATE_MODULE_REL = "crates/aureline-git/src/change_objects/mod.rs"

REQUIRED_KINDS = {"branch", "worktree", "patch_stack"}
REQUIRED_LANDING_STATES = {
    "local_only_no_remote_yet",
    "pending_publish_to_remote",
    "pending_merge_into_base",
    "pending_patch_apply",
    "landed_publicly",
}
REQUIRED_CONSUMER_INSPECTOR = "change_object_inspector"


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
    parser.add_argument("--report", default=None)
    parser.add_argument(
        "--render-gallery",
        action="store_true",
        help="Print the deterministic change-object inspector projection after validation.",
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
                check_id="change_object_alpha.schema.validation_failed",
                message=f"{label}: {path}: {error.message}",
                remediation="Fix the change-object record to validate against schemas/workspace/change_object.schema.json.",
                ref=label,
                details={"schema_path": list(error.schema_path)},
            )
        )
    return findings


def cross_check_record(label: str, record: dict[str, Any]) -> list[Finding]:
    findings: list[Finding] = []
    landing = record.get("landing_state", {})
    landing_state_class = landing.get("landing_state_class")
    landing_action_class = landing.get("landing_action_class")
    remote_visibility = landing.get("remote_visibility_class")
    egress = landing.get("required_network_egress_class")
    mutation_authority = landing.get("mutation_authority_class")

    if landing_state_class == "pending_publish_to_remote":
        if remote_visibility == "no_remote_attached":
            findings.append(
                Finding(
                    severity="error",
                    check_id="change_object.publish.remote_missing",
                    message=f"{label} pending_publish_to_remote must declare a remote-attached visibility class",
                    remediation="Pair pending_publish_to_remote with a remote_attached_* visibility class.",
                    ref=label,
                )
            )
        if egress == "no_network_egress_required":
            findings.append(
                Finding(
                    severity="error",
                    check_id="change_object.publish.egress_closed",
                    message=f"{label} pending_publish_to_remote must declare a non-zero network-egress class",
                    remediation="Pending publish requires a network-egress envelope; declare it.",
                    ref=label,
                )
            )
        if landing_action_class not in {"publish", "action_class_unknown_requires_review"}:
            findings.append(
                Finding(
                    severity="error",
                    check_id="change_object.publish.action_mismatch",
                    message=f"{label} pending_publish_to_remote must declare landing_action_class=publish",
                    remediation="Set landing_action_class to publish.",
                    ref=label,
                )
            )
    elif landing_state_class == "local_only_no_remote_yet":
        if remote_visibility not in {
            "no_remote_attached",
            "remote_visibility_unknown_requires_review",
        }:
            findings.append(
                Finding(
                    severity="error",
                    check_id="change_object.local.remote_widened",
                    message=f"{label} local_only_no_remote_yet must keep remote_visibility_class detached",
                    remediation="Keep local_only_no_remote_yet records without remote visibility.",
                    ref=label,
                )
            )
        if egress not in {
            "no_network_egress_required",
            "egress_envelope_unknown_requires_review",
        }:
            findings.append(
                Finding(
                    severity="error",
                    check_id="change_object.local.egress_widened",
                    message=f"{label} local_only_no_remote_yet must keep required_network_egress_class closed",
                    remediation="Local-only change objects do not require remote egress.",
                    ref=label,
                )
            )
        if mutation_authority not in {
            "local_only",
            "mutation_authority_unknown_requires_review",
        }:
            findings.append(
                Finding(
                    severity="error",
                    check_id="change_object.local.authority_widened",
                    message=f"{label} local_only_no_remote_yet must keep mutation_authority_class local-only",
                    remediation="Local-only change objects must keep mutation authority local.",
                    ref=label,
                )
            )
    elif (
        landing_state_class == "pending_merge_into_base"
        and landing_action_class not in {"merge", "action_class_unknown_requires_review"}
    ):
        findings.append(
            Finding(
                severity="error",
                check_id="change_object.merge.action_mismatch",
                message=f"{label} pending_merge_into_base must declare landing_action_class=merge",
                remediation="Set landing_action_class to merge.",
                ref=label,
            )
        )
    elif (
        landing_state_class == "pending_patch_apply"
        and landing_action_class not in {"apply", "action_class_unknown_requires_review"}
    ):
        findings.append(
            Finding(
                severity="error",
                check_id="change_object.apply.action_mismatch",
                message=f"{label} pending_patch_apply must declare landing_action_class=apply",
                remediation="Set landing_action_class to apply.",
                ref=label,
            )
        )

    kind = record.get("change_object_kind")
    branch = record.get("branch")
    worktree = record.get("worktree")
    patch_stack = record.get("patch_stack")
    if kind == "branch" and (worktree is not None or patch_stack is not None):
        findings.append(
            Finding(
                severity="error",
                check_id="change_object.variant.branch_exclusive",
                message=f"{label} branch change-object must not carry worktree or patch_stack variant blocks",
                remediation="Drop the non-matching variant block.",
                ref=label,
            )
        )
    elif kind == "worktree" and (branch is not None or patch_stack is not None):
        findings.append(
            Finding(
                severity="error",
                check_id="change_object.variant.worktree_exclusive",
                message=f"{label} worktree change-object must not carry branch or patch_stack variant blocks",
                remediation="Drop the non-matching variant block.",
                ref=label,
            )
        )
    elif kind == "patch_stack" and (branch is not None or worktree is not None):
        findings.append(
            Finding(
                severity="error",
                check_id="change_object.variant.patch_stack_exclusive",
                message=f"{label} patch_stack change-object must not carry branch or worktree variant blocks",
                remediation="Drop the non-matching variant block.",
                ref=label,
            )
        )

    consumers = set(record.get("consumer_surfaces", []))
    if REQUIRED_CONSUMER_INSPECTOR not in consumers:
        findings.append(
            Finding(
                severity="error",
                check_id="change_object.consumer.inspector_missing",
                message=f"{label} consumer_surfaces does not include {REQUIRED_CONSUMER_INSPECTOR}",
                remediation="Wire change_object_inspector as a consumer surface.",
                ref=label,
            )
        )

    invariants = record.get("review_invariants", {})
    for key in (
        "inspectable_before_publish",
        "inspectable_before_merge",
        "inspectable_before_apply",
        "no_hidden_target_mutation",
    ):
        if invariants.get(key) is not True:
            findings.append(
                Finding(
                    severity="error",
                    check_id=f"change_object.review.{key}_missing",
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
                    check_id=f"change_object.support_export.{key}_widened",
                    message=f"{label} support_export.{key} must be false",
                    remediation="Keep raw path, branch-name, remote URL, and diff-body export closed on the alpha record.",
                    ref=label,
                )
            )

    lineage = record.get("lineage", {})
    chain = lineage.get("ancestor_chain", [])
    seen: set[str] = set()
    for entry in chain:
        ancestor_ref = entry.get("ancestor_ref")
        if ancestor_ref in seen:
            findings.append(
                Finding(
                    severity="error",
                    check_id="change_object.lineage.duplicate_ancestor",
                    message=f"{label} lineage.ancestor_chain duplicates ancestor_ref {ancestor_ref}",
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
                remediation="Seed a fixture for each change-object kind.",
                details={"missing": sorted(missing_kinds)},
            )
        )
    states = {
        record.get("landing_state", {}).get("landing_state_class")
        for record in records.values()
    }
    missing_states = REQUIRED_LANDING_STATES - states
    if missing_states:
        findings.append(
            Finding(
                severity="error",
                check_id="fixtures.coverage.landing_state_missing",
                message="fixtures must cover every required landing-state class",
                remediation="Seed a fixture for every required landing-state class.",
                details={"missing": sorted(missing_states)},
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
                check_id="docs.change_object.missing",
                message=f"change-object reviewer doc is missing: {doc_path}",
                remediation="Author docs/review/m3/change_objects_alpha.md.",
                ref=str(doc_path),
            )
        )
    else:
        for ref in [
            DEFAULT_SCHEMA_REL,
            DEFAULT_FIXTURE_DIR_REL,
            "ci/check_change_object_alpha.py",
            DEFAULT_CONSUMER_REL,
            DEFAULT_CRATE_MODULE_REL,
        ]:
            if ref not in doc:
                findings.append(
                    Finding(
                        severity="error",
                        check_id="docs.change_object.missing_ref",
                        message=f"change-object reviewer doc does not mention {ref}",
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
                check_id="consumer.change_object.missing",
                message=f"shell change-object inspector consumer is missing: {consumer_path}",
                remediation="Wire the first shell consumer at crates/aureline-shell/src/change_object_inspector/mod.rs.",
                ref=str(consumer_path),
            )
        )
    else:
        for token in [
            "build_alpha_change_object_rows",
            "render_alpha_change_object_plaintext",
            "project_change_object",
        ]:
            if token not in consumer:
                findings.append(
                    Finding(
                        severity="error",
                        check_id="consumer.change_object.missing_token",
                        message=f"shell change-object inspector does not contain {token}",
                        remediation="Keep the alpha change-object family wired into the first shell consumer.",
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
                check_id="crate.change_object.missing",
                message=f"aureline-git change_objects module is missing: {crate_module_path}",
                remediation="Add the change_objects module to aureline-git.",
                ref=str(crate_module_path),
            )
        )
    else:
        for token in [
            "ChangeObjectRecord",
            "project_change_object",
            "CHANGE_OBJECT_ALPHA_RECORD_KIND",
            "CHANGE_OBJECT_LANDING_STATE_CLASSES",
        ]:
            if token not in crate_module:
                findings.append(
                    Finding(
                        severity="error",
                        check_id="crate.change_object.missing_token",
                        message=f"aureline-git change_objects module does not export {token}",
                        remediation="Keep the alpha change-object vocabulary exported from aureline-git.",
                        ref=str(crate_module_path),
                    )
                )
    return findings


def render_gallery(records: dict[str, dict[str, Any]]) -> str:
    lines = [
        "Change-object inspector alpha gallery",
        "change_object_id | kind | landing_state/action | target_ref | mutation_authority | remote_visibility | egress",
    ]
    for name in sorted(records):
        record = records[name]
        landing = record.get("landing_state", {})
        lines.append(
            " | ".join(
                [
                    record.get("change_object_id", name),
                    record.get("change_object_kind", "?"),
                    f"{landing.get('landing_state_class', '?')}/{landing.get('landing_action_class', '?')}",
                    landing.get("target_ref", "?"),
                    landing.get("mutation_authority_class", "?"),
                    landing.get("remote_visibility_class", "?"),
                    landing.get("required_network_egress_class", "?"),
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
