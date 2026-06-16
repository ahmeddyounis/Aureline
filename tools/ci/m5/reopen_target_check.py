#!/usr/bin/env python3
"""Recent-item, dock/taskbar, and jump-list reopen-fidelity CI gate.

This gate enforces that the checked-in reopen-target report stays fresh and
clean across the four required reopen surfaces (recent_item, dock, taskbar,
jump_list) and the five degraded reopen incidents (moved_target, missing_root,
changed_channel, stale_provider_linked, wrong_target_detected). It reads:

- the report fixture at ``fixtures/platform/m5-reopen-targets/report.json``;
- the support-export fixture at
  ``fixtures/platform/m5-reopen-targets/support_export.json``;
- the five per-incident case exports under
  ``fixtures/platform/m5-reopen-targets/cases/``;
- the boundary schema at ``schemas/platform/m5-reopen-target.schema.json``; and
- (when present) the published markdown at
  ``artifacts/platform/m5-recent-item-and-reopen.md`` and the companion doc at
  ``docs/m5/recent-items-dock-taskbar-jump-list.md``.

For the report the gate verifies that:

- the report covers every required reopen surface and every degraded
  availability class, so each reopen incident is tested and exportable;
- every target carries a literal target, a canonical object, an active-profile
  owner, an originating channel/build owner, a trust checkpoint, the canonical
  in-product command, a non-empty continuity note, a non-empty degraded-state
  vocabulary, a restore-provenance ref, at least one platform, a downgrade rule,
  and ``registered_on_reopen_harness = true``;
- a non-exact target carries a labeled placeholder and at least one recovery
  action, and the wrong-target class stays distinct from the unavailable-path
  class;
- a degraded or stale target never claims an exact restore;
- a privileged/mutating action returns through a reviewed in-product surface;
- no marketed target carries stale evidence;
- no target carries any blocking finding, so the distinct failure classes (a
  wrong-target reopen, a silent loss on an unavailable path, a stale-certainty
  overclaim, a silent mutating action, a hidden channel owner, an unpreserved
  identity, and a bypassed trust evaluation) are all caught;
- the report cross-links the native-desktop matrix, the system-entry intake,
  install-topology, the restore-provenance contract, the Start Center
  recent-work surface, and the entry interstitials;
- the support-export wrapper quotes the report id, every target id, and every
  descriptor revision; and
- the five incident case exports (moved_target, missing_root, changed_channel,
  stale_provider_linked, wrong_target) exist, each carries a non-exact
  availability with at least one recovery action, and the published markdown and
  companion doc back-link the canonical schema, fixtures, and CLI gate.

Exit codes:

- ``0`` -- report is clean (all surfaces and degraded classes covered, no
  blockers).
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

REPORT_REL = Path("fixtures/platform/m5-reopen-targets/report.json")
SUPPORT_EXPORT_REL = Path("fixtures/platform/m5-reopen-targets/support_export.json")
CASES_DIR_REL = Path("fixtures/platform/m5-reopen-targets/cases")
SCHEMA_REL = Path("schemas/platform/m5-reopen-target.schema.json")
MARKDOWN_REL = Path("artifacts/platform/m5-recent-item-and-reopen.md")
DOC_REL = Path("docs/m5/recent-items-dock-taskbar-jump-list.md")

REQUIRED_SURFACES = ("recent_item", "dock", "taskbar", "jump_list")

DEGRADED_AVAILABILITY = (
    "moved_target",
    "missing_root",
    "changed_channel",
    "stale_provider_linked",
    "wrong_target_detected",
)

CROSS_LINK_FIELDS = (
    "native_desktop_matrix_ref",
    "system_entry_intake_ref",
    "install_topology_ref",
    "restore_provenance_ref",
    "start_center_ref",
    "entry_interstitial_ref",
)

# (reopen_target_id, case_label) for the five required incident exports.
REQUIRED_CASES = (
    ("reopen:case.moved_target", "moved_target"),
    ("reopen:case.missing_root", "missing_root"),
    ("reopen:case.changed_channel", "changed_channel"),
    ("reopen:case.stale_provider_linked", "stale_provider_linked"),
    ("reopen:case.wrong_target", "wrong_target"),
)

EXPECTED_RECORD_KIND_REPORT = "shell_m5_reopen_target_report_record"
EXPECTED_RECORD_KIND_ROW = "shell_m5_reopen_target_row_record"
EXPECTED_RECORD_KIND_SUPPORT = "shell_m5_reopen_target_support_export_record"
EXPECTED_RECORD_KIND_CASE = "shell_m5_reopen_target_case_export_record"
EXPECTED_SHARED_CONTRACT_REF = "shell:m5_recent_items_and_reopen:v1"
EXPECTED_SCHEMA_VERSION = 1

DOC_BACKLINKS = (
    "artifacts/platform/m5-recent-item-and-reopen.md",
    "fixtures/platform/m5-reopen-targets/report.json",
    "schemas/platform/m5-reopen-target.schema.json",
    "tools/ci/m5/reopen_target_check.py",
)


@dataclass
class Finding:
    """One blocking finding emitted by the gate."""

    code: str
    message: str
    reopen_target_id: str | None = None
    detail: dict[str, Any] = field(default_factory=dict)

    def as_dict(self) -> dict[str, Any]:
        out: dict[str, Any] = {"code": self.code, "message": self.message}
        if self.reopen_target_id is not None:
            out["reopen_target_id"] = self.reopen_target_id
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
    present_surfaces = {
        ensure_dict(entry.get("descriptor", {}), "entry.descriptor").get("surface_kind")
        for entry in entries
    }
    for surface in REQUIRED_SURFACES:
        if surface not in present_surfaces:
            findings.append(
                Finding("missing_required_surface", "report is missing a required reopen surface", detail={"surface": surface})
            )
    present_availability = {
        ensure_dict(entry.get("descriptor", {}), "entry.descriptor").get("availability")
        for entry in entries
    }
    for availability in DEGRADED_AVAILABILITY:
        if availability not in present_availability:
            findings.append(
                Finding("missing_degraded_class", "report is missing a degraded availability class", detail={"availability": availability})
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
    reopen_target_id = descriptor.get("reopen_target_id")

    for field_name in (
        "literal_target_ref",
        "canonical_object_ref",
        "originating_channel_build_owner_ref",
        "active_profile_owner_ref",
        "trust_checkpoint_ref",
        "canonical_command_ref",
        "continuity_note",
        "restore_provenance_ref",
        "downgrade_rule_ref",
        "descriptor_revision_ref",
    ):
        if not is_nonempty_str(descriptor.get(field_name)):
            findings.append(
                Finding("descriptor_field_missing", "descriptor field is empty", reopen_target_id=reopen_target_id, detail={"field": field_name})
            )

    vocab = descriptor.get("degraded_state_vocabulary")
    if not isinstance(vocab, list) or not any(is_nonempty_str(v) for v in vocab):
        findings.append(Finding("missing_degraded_state_vocabulary", "descriptor.degraded_state_vocabulary must be non-empty", reopen_target_id=reopen_target_id))

    platforms = descriptor.get("claimed_platforms")
    if not isinstance(platforms, list) or not platforms:
        findings.append(Finding("missing_claimed_platforms", "descriptor.claimed_platforms must be non-empty", reopen_target_id=reopen_target_id))

    if descriptor.get("registered_on_reopen_harness") is not True:
        findings.append(
            Finding("target_not_on_harness", "descriptor.registered_on_reopen_harness must be true", reopen_target_id=reopen_target_id)
        )

    if descriptor.get("marketed") and descriptor.get("evidence_freshness") == "stale":
        findings.append(
            Finding("stale_evidence_on_marketed_target", "marketed target carries stale evidence", reopen_target_id=reopen_target_id)
        )

    availability = descriptor.get("availability")
    restore_availability = descriptor.get("restore_availability")
    degraded = availability in DEGRADED_AVAILABILITY
    stale = descriptor.get("target_freshness") == "stale"

    # Restore-certainty binding: a degraded or stale target may not claim exact
    # restore -- external re-entry never looks more certain than internal
    # restore.
    if (degraded or stale) and restore_availability == "exact":
        findings.append(
            Finding("stale_certainty_overclaim", "degraded or stale target claims an exact restore", reopen_target_id=reopen_target_id, detail={"availability": availability})
        )

    # No hidden mutation: a privileged/mutating action must return through a
    # reviewed in-product surface.
    if descriptor.get("action_class") == "privileged_or_mutating":
        routed = descriptor.get("stays_summary_only") is False and is_nonempty_str(descriptor.get("reviewed_return_surface_ref"))
        if not routed:
            findings.append(
                Finding("silent_mutating_action", "privileged/mutating shortcut must return through a reviewed surface", reopen_target_id=reopen_target_id)
            )

    # Recovery + placeholder: a non-exact target lands on a labeled placeholder
    # with at least one recovery action.
    if degraded:
        recovery = descriptor.get("recovery_actions")
        if not isinstance(recovery, list) or not recovery:
            findings.append(
                Finding("missing_recovery_action", "non-exact target must offer a recovery action", reopen_target_id=reopen_target_id, detail={"availability": availability})
            )
        if not is_nonempty_str(descriptor.get("placeholder_label_ref")):
            findings.append(
                Finding("missing_placeholder_label", "non-exact target must name a labeled placeholder", reopen_target_id=reopen_target_id, detail={"availability": availability})
            )

    # Wrong-target detection carries a conflicting object so the incident is
    # concrete and exportable.
    if availability == "wrong_target_detected" and not is_nonempty_str(descriptor.get("conflicting_object_ref")):
        findings.append(
            Finding("wrong_target_no_conflicting_object", "wrong-target reopen must name the conflicting object", reopen_target_id=reopen_target_id)
        )

    for blocker in ensure_list(entry.get("blocking_findings", []), "entry.blocking_findings"):
        findings.append(
            Finding(
                "blocking_finding_present",
                "target carries a blocking finding",
                reopen_target_id=reopen_target_id,
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
        reopen_target_id = descriptor.get("reopen_target_id")
        revision = descriptor.get("descriptor_revision_ref")
        if reopen_target_id not in case_set:
            findings.append(Finding("support_missing_target_id", "support_export.case_ids must quote every target id", reopen_target_id=reopen_target_id))
        if revision not in case_set:
            findings.append(
                Finding("support_missing_descriptor_revision", "support_export.case_ids must quote every descriptor revision", reopen_target_id=reopen_target_id, detail={"descriptor_revision_ref": revision})
            )


def check_case_exports(repo_root: Path, findings: list[Finding]) -> None:
    for reopen_target_id, label in REQUIRED_CASES:
        path = repo_root / CASES_DIR_REL / f"{label}.json"
        if not path.exists():
            findings.append(Finding("case_export_missing", "missing required incident case export", detail={"case": label}))
            continue
        export = ensure_dict(load_json(path), f"case[{label}]")
        if export.get("record_kind") != EXPECTED_RECORD_KIND_CASE:
            findings.append(Finding("case_record_kind_mismatch", "case export record_kind mismatch", detail={"case": label, "record_kind": export.get("record_kind")}))
        if export.get("case_label") != label:
            findings.append(Finding("case_label_mismatch", "case export label mismatch", detail={"case": label, "case_label": export.get("case_label")}))
        if export.get("availability") not in DEGRADED_AVAILABILITY:
            findings.append(Finding("case_availability_not_degraded", "case export must carry a non-exact availability", detail={"case": label, "availability": export.get("availability")}))
        recovery = export.get("recovery_actions")
        if not isinstance(recovery, list) or not recovery:
            findings.append(Finding("case_missing_recovery", "case export must offer a recovery action", detail={"case": label}))
        target = ensure_dict(export.get("target", {}), f"case[{label}].target")
        descriptor = ensure_dict(target.get("descriptor", {}), f"case[{label}].target.descriptor")
        if descriptor.get("reopen_target_id") != reopen_target_id:
            findings.append(Finding("case_target_id_mismatch", "case export target id mismatch", detail={"case": label, "reopen_target_id": descriptor.get("reopen_target_id")}))


def check_publications(repo_root: Path, findings: list[Finding]) -> None:
    markdown = repo_root / MARKDOWN_REL
    if not markdown.exists():
        findings.append(Finding("published_markdown_missing", f"missing published markdown: {MARKDOWN_REL}"))
    doc = repo_root / DOC_REL
    if not doc.exists():
        findings.append(Finding("published_doc_missing", f"missing companion doc: {DOC_REL}"))
        return
    body = doc.read_text(encoding="utf-8")
    for surface in REQUIRED_SURFACES:
        if surface not in body:
            findings.append(Finding("doc_missing_surface", "companion doc must quote every required reopen surface", detail={"surface": surface}))
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
            print("m5 reopen-target fidelity: clean")
        else:
            for finding in findings:
                location = finding.reopen_target_id or "report"
                print(f"FAIL [{finding.code}] {location}: {finding.message}")

    return 1 if findings else 0


if __name__ == "__main__":
    sys.exit(main())
