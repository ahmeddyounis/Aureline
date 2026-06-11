#!/usr/bin/env python3
"""M5 desktop-and-handoff qualification audit CI gate.

This gate enforces that the checked-in M5 desktop-conformance audit stays
fresh and clean across the nine desktop scenario rows the M5 depth lanes must
pass: multi_window, multi_monitor, mixed_dpi, suspend_resume, battery_saver,
thermal_pressure, deep_link, file_association, and system_open_return. It
reads:

- the audit fixture at ``fixtures/platform/m5_depth_surfaces/report.json``;
- the support-export fixture at
  ``fixtures/platform/m5_depth_surfaces/support_export.json``;
- the boundary schema at
  ``schemas/platform/m5-surface-desktop-qualification.schema.json``; and
- (when present) the published markdown at
  ``artifacts/ux/m5/desktop-conformance/m5_desktop_conformance_audit.md`` and
  the companion doc at ``docs/m5/desktop-and-handoff-parity.md``.

For the audit the gate verifies that:

- the audit covers all nine required rows and at least one surface qualifies
  each row;
- every registered surface has a binding for every required row;
- every surface carries a canonical reopen anchor, a non-empty continuity
  note, at least one claimed desktop profile, and
  ``registered_on_platform_conformance = true``;
- every qualified row carries its required captured evidence (an evidence
  pack, reopen fidelity, layout continuity, interruption safety, and
  placeholder honesty for every row; a background-adaptation result on the
  power rows; a handoff-reason and authority-context result on the handoff
  rows) and a present high-risk boundary cue on a high-salience surface;
- no qualified row carries a red result (a lost reopen target, broken layout
  continuity, corrupted interruption safety, a misleading placeholder, a lost
  authority context, un-throttled background work, a dropped handoff reason,
  or a hidden boundary cue);
- no surface drives a row through an ad-hoc local platform path, no marketed
  row is claimed with no evidence, and no marketed row carries stale
  evidence;
- no surface carries any blocking finding (so dimension, narrowing, and
  projection drift are all caught);
- the support-export wrapper quotes every surface id and descriptor revision
  the audit exposes; and
- the published markdown audit and the companion doc are present and
  back-link the canonical schema, fixtures, and CLI gate.

Exit codes:

- ``0`` -- audit is clean (all nine rows qualified, no blockers).
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

REPORT_REL = Path("fixtures/platform/m5_depth_surfaces/report.json")
SUPPORT_EXPORT_REL = Path("fixtures/platform/m5_depth_surfaces/support_export.json")
COMPACT_REL = Path("fixtures/platform/m5_depth_surfaces/compact.txt")
SCHEMA_REL = Path("schemas/platform/m5-surface-desktop-qualification.schema.json")
MARKDOWN_REL = Path("artifacts/ux/m5/desktop-conformance/m5_desktop_conformance_audit.md")
DOC_REL = Path("docs/m5/desktop-and-handoff-parity.md")

REQUIRED_ROWS = (
    "multi_window",
    "multi_monitor",
    "mixed_dpi",
    "suspend_resume",
    "battery_saver",
    "thermal_pressure",
    "deep_link",
    "file_association",
    "system_open_return",
)

WINDOW_TOPOLOGY_ROWS = ("multi_window", "multi_monitor", "mixed_dpi")
POWER_STATE_ROWS = ("suspend_resume", "battery_saver", "thermal_pressure")
HANDOFF_ROWS = ("deep_link", "file_association", "system_open_return")
BACKGROUND_ADAPTATION_ROWS = ("battery_saver", "thermal_pressure")

EXPECTED_RECORD_KIND_REPORT = "shell_m5_desktop_qualification_report_record"
EXPECTED_RECORD_KIND_ROW = "shell_m5_desktop_qualification_row_record"
EXPECTED_RECORD_KIND_SUPPORT = "shell_m5_desktop_qualification_support_export_record"
EXPECTED_SHARED_CONTRACT_REF = "shell:m5_desktop_conformance:v1"
EXPECTED_SCHEMA_VERSION = 1

HIGH_SALIENCE_CLASSES = {
    "lifecycle_bearing",
    "trust_bearing",
    "severity_bearing",
}

DOC_BACKLINKS = (
    "artifacts/ux/m5/desktop-conformance/m5_desktop_conformance_audit.md",
    "fixtures/platform/m5_depth_surfaces/report.json",
    "schemas/platform/m5-surface-desktop-qualification.schema.json",
    "tools/ci/m5/desktop_conformance_check.py",
)


@dataclass
class Finding:
    """One blocking finding emitted by the gate."""

    code: str
    message: str
    surface_id: str | None = None
    row: str | None = None
    detail: dict[str, Any] = field(default_factory=dict)

    def as_dict(self) -> dict[str, Any]:
        out: dict[str, Any] = {"code": self.code, "message": self.message}
        if self.surface_id is not None:
            out["surface_id"] = self.surface_id
        if self.row is not None:
            out["row"] = self.row
        if self.detail:
            out["detail"] = self.detail
        return out


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument(
        "--repo-root",
        default=".",
        help="Path to the repository root (default: cwd).",
    )
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


def descriptor_high_salience(descriptor: dict[str, Any]) -> bool:
    return descriptor.get("semantic_salience") in HIGH_SALIENCE_CLASSES


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
    declared = report.get("required_rows")
    if declared != list(REQUIRED_ROWS):
        findings.append(
            Finding(
                "required_rows_mismatch",
                "required_rows must equal the canonical row list",
                detail={"required": list(REQUIRED_ROWS), "declared": declared},
            )
        )
    profiles = report.get("claimed_desktop_profiles")
    if not isinstance(profiles, list) or not profiles:
        findings.append(
            Finding(
                "claimed_desktop_profiles_missing",
                "report.claimed_desktop_profiles must be a non-empty array",
                detail={"claimed_desktop_profiles": profiles},
            )
        )
    for ref_field in ("published_report_ref", "published_doc_ref"):
        ref = report.get(ref_field)
        if not isinstance(ref, str) or not ref.strip():
            findings.append(
                Finding(
                    "publication_ref_missing",
                    f"report.{ref_field} must be a non-empty string",
                    detail={ref_field: ref},
                )
            )
    if report.get("report_clean") is not True:
        findings.append(
            Finding(
                "report_not_clean",
                "report.report_clean must be true",
                detail={"report_clean": report.get("report_clean")},
            )
        )


def check_required_rows_qualified(report: dict[str, Any], findings: list[Finding]) -> None:
    rows = ensure_list(report.get("rows", []), "report.rows")
    for required in REQUIRED_ROWS:
        any_qualified = False
        for surface in rows:
            for binding in ensure_list(surface.get("bindings", []), "surface.bindings"):
                if (
                    binding.get("row") == required
                    and binding.get("qualification_status") == "qualified"
                ):
                    any_qualified = True
                    break
            if any_qualified:
                break
        if not any_qualified:
            findings.append(
                Finding(
                    "required_row_not_qualified",
                    "no qualified surface for required row",
                    row=required,
                )
            )


def check_qualified_binding(
    surface_id: str,
    high_salience: bool,
    binding: dict[str, Any],
    findings: list[Finding],
) -> None:
    row = binding.get("row")

    required_fields = [
        "projected_evidence_pack_ref",
        "projected_reopen_fidelity",
        "projected_layout_continuity",
        "projected_interruption_safety",
        "projected_placeholder_honesty",
        "evidence_freshness",
    ]
    if row in BACKGROUND_ADAPTATION_ROWS:
        required_fields.append("projected_background_adaptation")
    if row in HANDOFF_ROWS:
        required_fields.append("projected_handoff_reason")
        required_fields.append("projected_authority_context")
    if high_salience:
        required_fields.append("projected_boundary_cue")
    for field_name in required_fields:
        if binding.get(field_name) is None:
            findings.append(
                Finding(
                    "missing_projection",
                    "qualified row is missing required captured evidence",
                    surface_id=surface_id,
                    row=row,
                    detail={"field": field_name},
                )
            )

    # Red captured results.
    if binding.get("projected_reopen_fidelity") == "lost":
        findings.append(
            Finding("reopen_target_lost", "reopen target lost", surface_id, row)
        )
    if binding.get("projected_layout_continuity") == "broken":
        findings.append(
            Finding("layout_continuity_broken", "layout continuity broken", surface_id, row)
        )
    if binding.get("projected_interruption_safety") == "corrupted":
        findings.append(
            Finding("interruption_unsafe", "interruption corrupts foreground work", surface_id, row)
        )
    if binding.get("projected_placeholder_honesty") == "misleading":
        findings.append(
            Finding("placeholder_misleading", "placeholder pretends a missing target is live", surface_id, row)
        )
    if binding.get("projected_authority_context") == "lost":
        findings.append(
            Finding("authority_context_lost", "authority context lost across handoff", surface_id, row)
        )
    if (
        row in BACKGROUND_ADAPTATION_ROWS
        and binding.get("projected_background_adaptation") == "not_throttled"
    ):
        findings.append(
            Finding("background_not_throttled", "background work not throttled before corruption", surface_id, row)
        )
    if row in HANDOFF_ROWS and binding.get("projected_handoff_reason") == "dropped":
        findings.append(
            Finding("handoff_reason_dropped", "handoff reason dropped", surface_id, row)
        )
    if binding.get("projected_boundary_cue") == "hidden":
        findings.append(
            Finding("boundary_cue_hidden", "high-risk boundary cue hidden", surface_id, row)
        )
    if binding.get("marketed_on_row") and binding.get("evidence_freshness") == "stale":
        findings.append(
            Finding(
                "stale_evidence_on_marketed_row",
                "marketed row carries stale desktop evidence",
                surface_id,
                row,
            )
        )


def check_surface(surface: dict[str, Any], findings: list[Finding]) -> None:
    descriptor = ensure_dict(surface.get("descriptor", {}), "surface.descriptor")
    surface_id = descriptor.get("surface_id")
    if not isinstance(surface_id, str) or not surface_id.strip():
        findings.append(Finding("missing_surface_id", "descriptor.surface_id must be non-empty"))
        return

    if surface.get("record_kind") != EXPECTED_RECORD_KIND_ROW:
        findings.append(
            Finding(
                "surface_record_kind_mismatch",
                f"surface.record_kind must be {EXPECTED_RECORD_KIND_ROW}",
                surface_id=surface_id,
                detail={"record_kind": surface.get("record_kind")},
            )
        )

    revision = descriptor.get("descriptor_revision_ref")
    if not isinstance(revision, str) or not revision.strip():
        findings.append(
            Finding(
                "missing_descriptor_revision_ref",
                "descriptor.descriptor_revision_ref must be non-empty",
                surface_id=surface_id,
            )
        )

    anchor = descriptor.get("reopen_anchor_ref")
    if not isinstance(anchor, str) or not anchor.strip():
        findings.append(
            Finding(
                "descriptor_missing_reopen_anchor",
                "descriptor.reopen_anchor_ref must be non-empty",
                surface_id=surface_id,
            )
        )

    note = descriptor.get("continuity_note")
    if not isinstance(note, str) or not note.strip():
        findings.append(
            Finding(
                "missing_continuity_note",
                "descriptor.continuity_note must be non-empty",
                surface_id=surface_id,
            )
        )

    profiles = descriptor.get("claimed_desktop_profiles")
    if not isinstance(profiles, list) or not profiles:
        findings.append(
            Finding(
                "missing_claimed_profiles",
                "descriptor.claimed_desktop_profiles must be non-empty",
                surface_id=surface_id,
            )
        )

    if descriptor.get("registered_on_platform_conformance") is not True:
        findings.append(
            Finding(
                "surface_not_on_platform_conformance",
                "descriptor.registered_on_platform_conformance must be true",
                surface_id=surface_id,
            )
        )

    high_salience = descriptor_high_salience(descriptor)

    # Every required row must be bound.
    bindings = ensure_list(surface.get("bindings", []), "surface.bindings")
    present = {binding.get("row") for binding in bindings}
    for required in REQUIRED_ROWS:
        if required not in present:
            findings.append(
                Finding(
                    "missing_required_row",
                    "surface is missing a required scenario row binding",
                    surface_id=surface_id,
                    row=required,
                )
            )

    for binding in bindings:
        row = binding.get("row")
        dimension = binding.get("dimension")
        expected_dimension = canonical_dimension(row)
        if expected_dimension is not None and dimension != expected_dimension:
            findings.append(
                Finding(
                    "dimension_drift",
                    "binding dimension disagrees with its row's canonical dimension",
                    surface_id=surface_id,
                    row=row,
                    detail={"dimension": dimension, "expected": expected_dimension},
                )
            )
        status = binding.get("qualification_status")
        if status == "unqualified_local_platform_path":
            findings.append(
                Finding(
                    "unqualified_local_platform_path",
                    "surface drives a row through an ad-hoc local window/restore path",
                    surface_id=surface_id,
                    row=row,
                )
            )
        elif status == "missing_evidence":
            findings.append(
                Finding(
                    "missing_evidence",
                    "marketed row claimed with no captured evidence",
                    surface_id=surface_id,
                    row=row,
                )
            )
        elif status == "qualified":
            check_qualified_binding(surface_id, high_salience, binding, findings)

    # Any blocking finding the Rust validator emitted is a gate failure.
    for blocker in ensure_list(surface.get("blocking_findings", []), "surface.blocking_findings"):
        findings.append(
            Finding(
                "blocking_finding_present",
                "surface carries a blocking finding",
                surface_id=surface_id,
                row=blocker.get("row"),
                detail={"class": blocker.get("class")},
            )
        )


def canonical_dimension(row: Any) -> str | None:
    if row in WINDOW_TOPOLOGY_ROWS:
        return "window_topology"
    if row in POWER_STATE_ROWS:
        return "power_state"
    if row in HANDOFF_ROWS:
        return "handoff"
    return None


def check_support_export(
    report: dict[str, Any], export: dict[str, Any], findings: list[Finding]
) -> None:
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
        findings.append(
            Finding("support_case_ids_missing", "support_export.case_ids must be an array")
        )
        return
    case_set = set(case_ids)
    report_id = report.get("report_id")
    if report_id not in case_set:
        findings.append(
            Finding(
                "support_missing_report_id",
                "support_export.case_ids must quote the report id",
                detail={"report_id": report_id},
            )
        )
    for surface in ensure_list(report.get("rows", []), "report.rows"):
        descriptor = ensure_dict(surface.get("descriptor", {}), "surface.descriptor")
        surface_id = descriptor.get("surface_id")
        revision = descriptor.get("descriptor_revision_ref")
        if surface_id not in case_set:
            findings.append(
                Finding(
                    "support_missing_surface_id",
                    "support_export.case_ids must quote every surface id",
                    surface_id=surface_id,
                )
            )
        if revision not in case_set:
            findings.append(
                Finding(
                    "support_missing_descriptor_revision",
                    "support_export.case_ids must quote every descriptor revision",
                    surface_id=surface_id,
                    detail={"descriptor_revision_ref": revision},
                )
            )


def check_publications(repo_root: Path, findings: list[Finding]) -> None:
    markdown = repo_root / MARKDOWN_REL
    if not markdown.exists():
        findings.append(
            Finding("published_markdown_missing", f"missing published markdown: {MARKDOWN_REL}")
        )
    doc = repo_root / DOC_REL
    if not doc.exists():
        findings.append(Finding("published_doc_missing", f"missing companion doc: {DOC_REL}"))
        return
    body = doc.read_text(encoding="utf-8")
    for row in REQUIRED_ROWS:
        if row not in body:
            findings.append(
                Finding(
                    "doc_missing_row",
                    "companion doc must quote every required scenario row",
                    row=row,
                )
            )
    for backlink in DOC_BACKLINKS:
        if backlink not in body:
            findings.append(
                Finding(
                    "doc_missing_backlink",
                    "companion doc must back-link the canonical artifacts and gate",
                    detail={"backlink": backlink},
                )
            )


def main() -> int:
    args = parse_args()
    repo_root = Path(args.repo_root).resolve()

    report = ensure_dict(load_json(repo_root / REPORT_REL), "report")
    export = ensure_dict(load_json(repo_root / SUPPORT_EXPORT_REL), "support_export")
    # The schema is required to exist so the contract stays discoverable.
    if not (repo_root / SCHEMA_REL).exists():
        raise SystemExit(f"missing required input: {SCHEMA_REL}")

    findings: list[Finding] = []
    check_report_envelope(report, findings)
    check_required_rows_qualified(report, findings)
    for surface in ensure_list(report.get("rows", []), "report.rows"):
        check_surface(ensure_dict(surface, "surface"), findings)
    check_support_export(report, export, findings)
    check_publications(repo_root, findings)

    if args.format == "json":
        print(json.dumps({"findings": [f.as_dict() for f in findings]}, indent=2))
    else:
        if not findings:
            print("m5 desktop-conformance audit: clean")
        else:
            for finding in findings:
                location = finding.surface_id or "report"
                if finding.row:
                    location = f"{location} / {finding.row}"
                print(f"FAIL [{finding.code}] {location}: {finding.message}")

    return 1 if findings else 0


if __name__ == "__main__":
    sys.exit(main())
