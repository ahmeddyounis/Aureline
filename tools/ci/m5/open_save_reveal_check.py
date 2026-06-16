#!/usr/bin/env python3
"""Open/save/reveal path-truth CI gate.

This gate enforces that the checked-in open/save/reveal path-truth report stays
fresh and clean across the five required system-dialog/reveal flow kinds (open,
save, save_as, reveal_in_system_shell, open_in_default_browser). It reads:

- the report fixture at ``fixtures/platform/m5-open-save-reveal/report.json``;
- the support-export fixture at
  ``fixtures/platform/m5-open-save-reveal/support_export.json``;
- the four per-incident case exports under
  ``fixtures/platform/m5-open-save-reveal/cases/``;
- the boundary schema at ``schemas/platform/m5-path-boundary.schema.json``; and
- (when present) the published markdown at
  ``artifacts/platform/m5-open-save-reveal.md`` and the companion doc at
  ``docs/m5/open-save-reveal-path-truth.md``.

For the report the gate verifies that:

- the report covers every required flow kind;
- every flow carries a literal target, a canonical target, a boundary-label
  ref, the filesystem-identity and save-coordination refs it reuses, an
  active-profile owner, a trust checkpoint, the canonical in-product command, a
  shared overwrite-review ref, a non-empty continuity note, a non-empty
  degraded-state vocabulary, at least one platform, a downgrade rule, and
  ``registered_on_path_truth_harness = true``;
- an in-place overwrite (``overwrite_with_checkpoint``) pins an available
  checkpoint and names a checkpoint ref;
- a writing posture against a read-only boundary/destination, and a generated
  artifact saved in place, are not present (they are distinct caught failures);
- a reveal-in-system-shell or open-in-default-browser flow discloses its
  external side effect and a stable action label;
- any non-exact path condition offers at least one recovery action;
- no marketed flow carries stale evidence;
- no flow carries any blocking finding, so the distinct failure classes (a
  wrong-target save, an alias-path confusion, an unrecoverable generated output
  or read-only destination, an overwrite without checkpoint review, a read-only
  write attempt, a generated in-place save, a hidden reveal side effect, a
  hidden canonical path, a divergent checkpoint vocabulary, and a bypassed trust
  evaluation) are all caught;
- the report cross-links the filesystem-identity, save-coordination,
  restore-continuity, native-desktop matrix, system-entry intake, and Help/About
  surfaces;
- the support-export wrapper quotes the report id, every flow id, and every
  descriptor revision; and
- the four incident case exports (missing_canonical_target, network_share_alias,
  generated_output, read_only_destination) exist, each carries a non-exact path
  condition with at least one recovery action, and the published markdown and
  companion doc back-link the canonical schema, fixtures, and CLI gate.

Exit codes:

- ``0`` -- report is clean (all kinds covered, no blockers).
- ``1`` -- one or more findings.
- ``2`` -- usage error or missing input file.
"""

from __future__ import annotations

import argparse
import json
import sys
from dataclasses import dataclass, field
from pathlib import Path
from typing import Any

REPORT_REL = Path("fixtures/platform/m5-open-save-reveal/report.json")
SUPPORT_EXPORT_REL = Path("fixtures/platform/m5-open-save-reveal/support_export.json")
CASES_DIR_REL = Path("fixtures/platform/m5-open-save-reveal/cases")
SCHEMA_REL = Path("schemas/platform/m5-path-boundary.schema.json")
MARKDOWN_REL = Path("artifacts/platform/m5-open-save-reveal.md")
DOC_REL = Path("docs/m5/open-save-reveal-path-truth.md")

REQUIRED_KINDS = (
    "open",
    "save",
    "save_as",
    "reveal_in_system_shell",
    "open_in_default_browser",
)

CROSS_LINK_FIELDS = (
    "filesystem_identity_ref",
    "save_coordination_ref",
    "restore_continuity_ref",
    "native_desktop_matrix_ref",
    "system_entry_intake_ref",
    "help_about_ref",
)

# (flow_id, case_label) for the four required incident exports.
REQUIRED_CASES = (
    ("flow:case.missing_canonical_target", "missing_canonical_target"),
    ("flow:case.network_share_alias", "network_share_alias"),
    ("flow:case.generated_output", "generated_output"),
    ("flow:case.read_only_destination", "read_only_destination"),
)

EXPECTED_RECORD_KIND_REPORT = "workspace_m5_open_save_reveal_report_record"
EXPECTED_RECORD_KIND_ROW = "workspace_m5_open_save_reveal_flow_record"
EXPECTED_RECORD_KIND_SUPPORT = "workspace_m5_open_save_reveal_support_export_record"
EXPECTED_RECORD_KIND_CASE = "workspace_m5_open_save_reveal_case_export_record"
EXPECTED_SHARED_CONTRACT_REF = "workspace:m5_open_save_reveal:v1"
EXPECTED_SCHEMA_VERSION = 1

NON_EXACT_CONDITION = (
    "missing_canonical_target",
    "network_share_alias",
    "generated_output",
    "read_only_destination",
)

WRITE_FLOW_KINDS = ("save", "save_as")
REVEAL_FLOW_KINDS = ("reveal_in_system_shell", "open_in_default_browser")
EXPECTED_SIDE_EFFECT = {
    "open": "no_external_side_effect",
    "save": "no_external_side_effect",
    "save_as": "no_external_side_effect",
    "reveal_in_system_shell": "selects_target_in_file_manager",
    "open_in_default_browser": "opens_default_browser",
}
IN_PLACE_WRITE_POSTURES = ("overwrite_with_checkpoint", "overwrite_review_required")

DOC_BACKLINKS = (
    "artifacts/platform/m5-open-save-reveal.md",
    "fixtures/platform/m5-open-save-reveal/report.json",
    "schemas/platform/m5-path-boundary.schema.json",
    "tools/ci/m5/open_save_reveal_check.py",
)


@dataclass
class Finding:
    """One blocking finding emitted by the gate."""

    code: str
    message: str
    flow_id: str | None = None
    detail: dict[str, Any] = field(default_factory=dict)

    def as_dict(self) -> dict[str, Any]:
        out: dict[str, Any] = {"code": self.code, "message": self.message}
        if self.flow_id is not None:
            out["flow_id"] = self.flow_id
        if self.detail:
            out["detail"] = self.detail
        return out


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--repo-root", default=".", help="Path to the repository root (default: cwd).")
    parser.add_argument(
        "--format",
        choices=("text", "json"),
        default="text",
        help="Output format for the findings report.",
    )
    return parser.parse_args()


def load_json(path: Path) -> Any:
    if not path.exists():
        raise SystemExit(f"missing required input: {path}")
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


def is_nonempty_str(value: Any) -> bool:
    return isinstance(value, str) and value.strip() != ""


def check_report_envelope(report: dict[str, Any], findings: list[Finding]) -> None:
    if report.get("record_kind") != EXPECTED_RECORD_KIND_REPORT:
        findings.append(
            Finding(
                "report_record_kind_mismatch",
                f"report.record_kind must be {EXPECTED_RECORD_KIND_REPORT}",
                detail={"record_kind": report.get("record_kind")},
            )
        )
    if report.get("schema_version") != EXPECTED_SCHEMA_VERSION:
        findings.append(
            Finding(
                "report_schema_version_mismatch",
                f"report.schema_version must be {EXPECTED_SCHEMA_VERSION}",
                detail={"schema_version": report.get("schema_version")},
            )
        )
    if report.get("shared_contract_ref") != EXPECTED_SHARED_CONTRACT_REF:
        findings.append(
            Finding(
                "report_shared_contract_ref_mismatch",
                f"report.shared_contract_ref must be {EXPECTED_SHARED_CONTRACT_REF}",
                detail={"shared_contract_ref": report.get("shared_contract_ref")},
            )
        )
    if report.get("report_clean") is not True:
        findings.append(Finding("report_not_clean", "report.report_clean must be true"))


def check_cross_links(report: dict[str, Any], findings: list[Finding]) -> None:
    cross_links = report.get("cross_links")
    if not isinstance(cross_links, dict):
        findings.append(Finding("cross_links_missing", "report.cross_links must be an object"))
        return
    for field_name in CROSS_LINK_FIELDS:
        if not is_nonempty_str(cross_links.get(field_name)):
            findings.append(
                Finding("cross_link_missing", "report.cross_links field is empty", detail={"field": field_name})
            )


def check_required_coverage(report: dict[str, Any], findings: list[Finding]) -> None:
    entries = ensure_list(report.get("entries", []), "report.entries")
    present_kinds = {
        ensure_dict(entry.get("descriptor", {}), "entry.descriptor").get("flow_kind")
        for entry in entries
    }
    for kind in REQUIRED_KINDS:
        if kind not in present_kinds:
            findings.append(
                Finding("missing_required_kind", "report is missing a required flow kind", detail={"kind": kind})
            )


def check_entry(entry: dict[str, Any], findings: list[Finding]) -> None:
    if entry.get("record_kind") != EXPECTED_RECORD_KIND_ROW:
        findings.append(
            Finding(
                "row_record_kind_mismatch",
                f"entry.record_kind must be {EXPECTED_RECORD_KIND_ROW}",
                detail={"record_kind": entry.get("record_kind")},
            )
        )
    descriptor = ensure_dict(entry.get("descriptor", {}), "entry.descriptor")
    flow_id = descriptor.get("flow_id")
    flow_kind = descriptor.get("flow_kind")

    for field_name in (
        "literal_target_ref",
        "canonical_target_ref",
        "boundary_label_ref",
        "overwrite_review_ref",
        "filesystem_identity_ref",
        "save_coordination_ref",
        "active_profile_owner_ref",
        "trust_checkpoint_ref",
        "canonical_command_ref",
        "continuity_note",
        "downgrade_rule_ref",
        "descriptor_revision_ref",
    ):
        if not is_nonempty_str(descriptor.get(field_name)):
            findings.append(
                Finding("descriptor_field_missing", "descriptor field is empty", flow_id=flow_id, detail={"field": field_name})
            )

    vocab = descriptor.get("degraded_state_vocabulary")
    if not isinstance(vocab, list) or not any(is_nonempty_str(v) for v in vocab):
        findings.append(Finding("missing_degraded_state_vocabulary", "descriptor.degraded_state_vocabulary must be non-empty", flow_id=flow_id))

    platforms = descriptor.get("claimed_platforms")
    if not isinstance(platforms, list) or not platforms:
        findings.append(Finding("missing_claimed_platforms", "descriptor.claimed_platforms must be non-empty", flow_id=flow_id))

    if descriptor.get("registered_on_path_truth_harness") is not True:
        findings.append(
            Finding("flow_not_on_harness", "descriptor.registered_on_path_truth_harness must be true", flow_id=flow_id)
        )

    if descriptor.get("marketed") and descriptor.get("evidence_freshness") == "stale":
        findings.append(
            Finding("stale_evidence_on_marketed_flow", "marketed flow carries stale evidence", flow_id=flow_id)
        )

    boundary = descriptor.get("boundary_label")
    posture = descriptor.get("write_posture")
    condition = descriptor.get("path_condition")
    is_write_flow = flow_kind in WRITE_FLOW_KINDS

    # Overwrite / checkpoint discipline.
    if posture == "overwrite_with_checkpoint":
        if descriptor.get("checkpoint_availability") != "pinned" or not is_nonempty_str(descriptor.get("checkpoint_ref")):
            findings.append(
                Finding("overwrite_without_checkpoint_review", "in-place overwrite must pin an available checkpoint", flow_id=flow_id)
            )

    # Read-only discipline.
    is_read_only = boundary == "read_only" or condition == "read_only_destination"
    if is_write_flow and is_read_only and posture in ("create_new_file", "overwrite_with_checkpoint", "overwrite_review_required"):
        findings.append(
            Finding("read_only_write_attempt", "a writing posture targeted a read-only boundary", flow_id=flow_id, detail={"write_posture": posture})
        )

    # Generated discipline.
    is_generated = boundary == "generated" or condition == "generated_output"
    if is_write_flow and is_generated and posture in IN_PLACE_WRITE_POSTURES:
        findings.append(
            Finding("generated_treated_as_in_place_save", "a generated artifact was saved in place instead of exported", flow_id=flow_id, detail={"write_posture": posture})
        )

    # Reveal / browser side-effect disclosure.
    expected_side_effect = EXPECTED_SIDE_EFFECT.get(flow_kind)
    if expected_side_effect is not None and descriptor.get("reveal_side_effect") != expected_side_effect:
        findings.append(
            Finding("reveal_side_effect_hidden", "flow does not disclose its expected external side effect", flow_id=flow_id, detail={"flow_kind": flow_kind})
        )
    if flow_kind in REVEAL_FLOW_KINDS and not is_nonempty_str(descriptor.get("reveal_action_label_ref")):
        findings.append(
            Finding("reveal_side_effect_hidden", "reveal/browser flow must disclose a stable action label", flow_id=flow_id, detail={"flow_kind": flow_kind})
        )

    # Recovery: a non-exact path condition must offer a recovery action.
    if condition in NON_EXACT_CONDITION:
        recovery = descriptor.get("recovery_actions")
        if not isinstance(recovery, list) or not recovery:
            findings.append(
                Finding("missing_recovery_action", "non-exact path condition must offer a recovery action", flow_id=flow_id, detail={"path_condition": condition})
            )

    for blocker in ensure_list(entry.get("blocking_findings", []), "entry.blocking_findings"):
        findings.append(
            Finding(
                "blocking_finding_present",
                "flow carries a blocking finding",
                flow_id=flow_id,
                detail={"class": blocker.get("class")},
            )
        )


def check_support_export(report: dict[str, Any], export: dict[str, Any], findings: list[Finding]) -> None:
    if export.get("record_kind") != EXPECTED_RECORD_KIND_SUPPORT:
        findings.append(
            Finding(
                "support_record_kind_mismatch",
                f"support_export.record_kind must be {EXPECTED_RECORD_KIND_SUPPORT}",
                detail={"record_kind": export.get("record_kind")},
            )
        )
    case_ids = export.get("case_ids")
    if not isinstance(case_ids, list):
        findings.append(Finding("support_case_ids_missing", "support_export.case_ids must be an array"))
        return
    case_set = set(case_ids)
    if report.get("report_id") not in case_set:
        findings.append(
            Finding("support_missing_report_id", "support_export.case_ids must quote the report id", detail={"report_id": report.get("report_id")})
        )
    for entry in ensure_list(report.get("entries", []), "report.entries"):
        descriptor = ensure_dict(entry.get("descriptor", {}), "entry.descriptor")
        flow_id = descriptor.get("flow_id")
        revision = descriptor.get("descriptor_revision_ref")
        if flow_id not in case_set:
            findings.append(Finding("support_missing_flow_id", "support_export.case_ids must quote every flow id", flow_id=flow_id))
        if revision not in case_set:
            findings.append(
                Finding("support_missing_descriptor_revision", "support_export.case_ids must quote every descriptor revision", flow_id=flow_id, detail={"descriptor_revision_ref": revision})
            )


def check_case_exports(repo_root: Path, findings: list[Finding]) -> None:
    for flow_id, label in REQUIRED_CASES:
        path = repo_root / CASES_DIR_REL / f"{label}.json"
        if not path.exists():
            findings.append(Finding("case_export_missing", "missing required incident case export", detail={"case": label}))
            continue
        export = ensure_dict(load_json(path), f"case[{label}]")
        if export.get("record_kind") != EXPECTED_RECORD_KIND_CASE:
            findings.append(Finding("case_record_kind_mismatch", "case export record_kind mismatch", detail={"case": label, "record_kind": export.get("record_kind")}))
        if export.get("case_label") != label:
            findings.append(Finding("case_label_mismatch", "case export label mismatch", detail={"case": label, "case_label": export.get("case_label")}))
        if export.get("path_condition") not in NON_EXACT_CONDITION:
            findings.append(Finding("case_condition_not_degraded", "case export must carry a non-exact path condition", detail={"case": label, "path_condition": export.get("path_condition")}))
        recovery = export.get("recovery_actions")
        if not isinstance(recovery, list) or not recovery:
            findings.append(Finding("case_missing_recovery", "case export must offer a recovery action", detail={"case": label}))
        flow = ensure_dict(export.get("flow", {}), f"case[{label}].flow")
        descriptor = ensure_dict(flow.get("descriptor", {}), f"case[{label}].flow.descriptor")
        if descriptor.get("flow_id") != flow_id:
            findings.append(Finding("case_flow_id_mismatch", "case export flow id mismatch", detail={"case": label, "flow_id": descriptor.get("flow_id")}))


def check_publications(repo_root: Path, findings: list[Finding]) -> None:
    markdown = repo_root / MARKDOWN_REL
    if not markdown.exists():
        findings.append(Finding("published_markdown_missing", f"missing published markdown: {MARKDOWN_REL}"))
    doc = repo_root / DOC_REL
    if not doc.exists():
        findings.append(Finding("published_doc_missing", f"missing companion doc: {DOC_REL}"))
        return
    body = doc.read_text(encoding="utf-8")
    for kind in REQUIRED_KINDS:
        if kind not in body:
            findings.append(Finding("doc_missing_kind", "companion doc must quote every required flow kind", detail={"kind": kind}))
    for backlink in DOC_BACKLINKS:
        if backlink not in body:
            findings.append(Finding("doc_missing_backlink", "companion doc must back-link the canonical artifacts and gate", detail={"backlink": backlink}))


def main() -> int:
    args = parse_args()
    repo_root = Path(args.repo_root).resolve()

    report = ensure_dict(load_json(repo_root / REPORT_REL), "report")
    export = ensure_dict(load_json(repo_root / SUPPORT_EXPORT_REL), "support_export")
    if not (repo_root / SCHEMA_REL).exists():
        raise SystemExit(f"missing required input: {SCHEMA_REL}")

    findings: list[Finding] = []
    check_report_envelope(report, findings)
    check_cross_links(report, findings)
    check_required_coverage(report, findings)
    for entry in ensure_list(report.get("entries", []), "report.entries"):
        check_entry(ensure_dict(entry, "entry"), findings)
    check_support_export(report, export, findings)
    check_case_exports(repo_root, findings)
    check_publications(repo_root, findings)

    if args.format == "json":
        print(json.dumps({"findings": [f.as_dict() for f in findings]}, indent=2))
    else:
        if not findings:
            print("m5 open/save/reveal path truth: clean")
        else:
            for finding in findings:
                location = finding.flow_id or "report"
                print(f"FAIL [{finding.code}] {location}: {finding.message}")

    return 1 if findings else 0


if __name__ == "__main__":
    sys.exit(main())
