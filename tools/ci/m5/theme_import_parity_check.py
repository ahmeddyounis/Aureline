#!/usr/bin/env python3
"""M5 theme-package, appearance-session, token-overlay, and import-parity gate.

This gate enforces that the checked-in M5 theme-import-parity audit stays fresh,
honest, and clean across the five parity rows every M5 surface that consumes an
appearance object must report: theme_package_compatibility,
appearance_session_integrity, token_overlay_validation, imported_theme_parity,
and extension_surface_inheritance. It reads:

- the audit fixture at ``fixtures/ux/m5/theme-package-interop/report.json``;
- the support-export fixture at
  ``fixtures/ux/m5/theme-package-interop/support_export.json``;
- the boundary schema at ``schemas/ux/m5-theme-import-parity.schema.json``; and
- the published markdown at
  ``artifacts/ux/m5/theme-import-parity/m5_theme_import_parity_audit.md`` and the
  companion doc at ``docs/m5/theme-package-and-appearance-objects.md``.

For the audit the gate verifies that:

- the frozen object-model index names all five canonical appearance-object
  families, and each family's canonical schema file actually exists on disk so
  the freeze can never index a schema that was renamed or removed;
- the audit covers all five required rows and at least one surface qualifies
  each row;
- every registered surface has a binding for every required row, carries a
  canonical appearance anchor, a non-empty accessibility note, and
  ``registered_on_appearance_session = true``;
- every qualified row projects its disclosed downgrade truth: a compatibility
  state, an object ref, and fresh-or-disclosed evidence; a non-current
  compatibility state (stale evidence, an unsupported slot, partial
  inheritance, or a restart-or-reload-required change) must be disclosed in
  product, export, and diagnostics;
- no row hides a downgrade (a hidden downgrade, a silently dropped overlay
  token, a hidden unresolved import mapping, a hidden inheritance gap, a
  full-fidelity parity claim with no report, a missing rollback path, an
  undisclosed restart-or-reload change, or stale evidence on a marketed row is
  a blocker);
- no surface carries a blocking finding, the support-export wrapper quotes
  every surface id and descriptor revision, and the published markdown audit
  and the companion doc back-link the canonical schema, fixtures, and gate.

Exit codes:

- ``0`` -- audit is clean (all five rows qualified, no blockers).
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

REPORT_REL = Path("fixtures/ux/m5/theme-package-interop/report.json")
SUPPORT_EXPORT_REL = Path("fixtures/ux/m5/theme-package-interop/support_export.json")
COMPACT_REL = Path("fixtures/ux/m5/theme-package-interop/compact.txt")
SCHEMA_REL = Path("schemas/ux/m5-theme-import-parity.schema.json")
MARKDOWN_REL = Path("artifacts/ux/m5/theme-import-parity/m5_theme_import_parity_audit.md")
DOC_REL = Path("docs/m5/theme-package-and-appearance-objects.md")

REQUIRED_ROWS = (
    "theme_package_compatibility",
    "appearance_session_integrity",
    "token_overlay_validation",
    "imported_theme_parity",
    "extension_surface_inheritance",
)

ROW_TO_DIMENSION = {
    "theme_package_compatibility": "theme_package",
    "appearance_session_integrity": "appearance_session",
    "token_overlay_validation": "token_overlay",
    "imported_theme_parity": "import_report",
    "extension_surface_inheritance": "extension_descriptor",
}

ROW_TO_FAMILY = {
    "theme_package_compatibility": "theme_package",
    "appearance_session_integrity": "appearance_session",
    "token_overlay_validation": "token_overlay",
    "imported_theme_parity": "theme_import_report",
    "extension_surface_inheritance": "extension_appearance_descriptor",
}

REQUIRED_OBJECT_FAMILIES = (
    "theme_package",
    "appearance_session",
    "token_overlay",
    "theme_import_report",
    "extension_appearance_descriptor",
)

NON_CURRENT_COMPAT = {
    "stale_evidence",
    "unsupported_slot",
    "partial_inheritance",
    "restart_or_reload_required",
}

EXPECTED_RECORD_KIND_REPORT = "shell_m5_theme_import_parity_report_record"
EXPECTED_RECORD_KIND_ROW = "shell_m5_theme_import_parity_row_record"
EXPECTED_RECORD_KIND_SUPPORT = "shell_m5_theme_import_parity_support_export_record"
EXPECTED_SHARED_CONTRACT_REF = "shell:m5_theme_import_parity:v1"
EXPECTED_SCHEMA_VERSION = 1

HIGH_SALIENCE_CLASSES = {
    "lifecycle_bearing",
    "trust_bearing",
    "severity_bearing",
}

DOC_BACKLINKS = (
    "artifacts/ux/m5/theme-import-parity/m5_theme_import_parity_audit.md",
    "fixtures/ux/m5/theme-package-interop/report.json",
    "schemas/ux/m5-theme-import-parity.schema.json",
    "tools/ci/m5/theme_import_parity_check.py",
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


def check_object_model_index(repo_root: Path, report: dict[str, Any], findings: list[Finding]) -> None:
    index = ensure_list(report.get("object_model_index", []), "report.object_model_index")
    present = {entry.get("family") for entry in index if isinstance(entry, dict)}
    for family in REQUIRED_OBJECT_FAMILIES:
        if family not in present:
            findings.append(
                Finding(
                    "object_model_index_drift",
                    "frozen object-model index is missing a canonical appearance-object family",
                    detail={"family": family},
                )
            )
    for entry in index:
        if not isinstance(entry, dict):
            continue
        family = entry.get("family")
        schema_ref = entry.get("canonical_schema_ref")
        if not isinstance(schema_ref, str) or not schema_ref.strip():
            findings.append(
                Finding(
                    "object_model_index_drift",
                    "object-model index entry has no canonical schema ref",
                    detail={"family": family},
                )
            )
            continue
        if not (repo_root / schema_ref).exists():
            findings.append(
                Finding(
                    "object_model_index_drift",
                    "object-model index points at a schema that does not exist",
                    detail={"family": family, "canonical_schema_ref": schema_ref},
                )
            )


def check_required_rows_qualified(report: dict[str, Any], findings: list[Finding]) -> None:
    rows = ensure_list(report.get("rows", []), "report.rows")
    for required in REQUIRED_ROWS:
        any_qualified = False
        for surface in rows:
            for binding in ensure_list(surface.get("bindings", []), "surface.bindings"):
                if binding.get("row") == required and binding.get("qualification_status") == "qualified":
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
    binding: dict[str, Any],
    findings: list[Finding],
) -> None:
    row = binding.get("row")

    # Disclosed downgrade truth: every qualified row projects a compatibility
    # state, an object ref, and evidence.
    for field_name in ("projected_compatibility_state", "object_ref", "evidence_freshness", "evidence_captured_at"):
        if binding.get(field_name) is None:
            findings.append(
                Finding(
                    "missing_projection",
                    "qualified row is missing required disclosed evidence",
                    surface_id=surface_id,
                    row=row,
                    detail={"field": field_name},
                )
            )

    compat = binding.get("projected_compatibility_state")
    disclosed = (
        binding.get("downgrade_disclosed_in_product") is True
        and binding.get("downgrade_disclosed_in_export") is True
        and binding.get("downgrade_disclosed_in_diagnostics") is True
    )
    if compat in NON_CURRENT_COMPAT and not disclosed:
        findings.append(
            Finding(
                "hidden_downgrade",
                "qualified row downgrades without disclosing it in product, export, and diagnostics",
                surface_id=surface_id,
                row=row,
                detail={"compatibility_state": compat},
            )
        )
    if compat == "restart_or_reload_required" and not disclosed:
        findings.append(
            Finding(
                "restart_reload_undisclosed",
                "a restart-or-reload-required change is not disclosed",
                surface_id=surface_id,
                row=row,
            )
        )

    # Token overlays must never silently drop tokens.
    if row == "token_overlay_validation" and binding.get("projected_overlay_validation_state") == "rolled_back":
        if binding.get("projected_rollback_path") is None:
            findings.append(
                Finding(
                    "rollback_path_missing",
                    "a rolled-back overlay has no rollback path",
                    surface_id=surface_id,
                    row=row,
                )
            )

    # Imported themes: an applied import with unresolved/blocked mapping needs a rollback path,
    # and a full-fidelity parity claim needs a report.
    if row == "imported_theme_parity":
        outcome = binding.get("projected_import_outcome")
        if outcome in ("applied", "applied_with_warnings", "rolled_back") and binding.get("projected_rollback_path") is None:
            findings.append(
                Finding(
                    "rollback_path_missing",
                    "an applied/rolled-back import has no rollback path",
                    surface_id=surface_id,
                    row=row,
                )
            )
        claim = binding.get("projected_parity_claim_state")
        if claim in ("claimed_with_report", "partial_claim_with_gaps") and not binding.get("report_ref"):
            findings.append(
                Finding(
                    "parity_claim_without_report",
                    "an imported-theme parity claim cites no report",
                    surface_id=surface_id,
                    row=row,
                )
            )

    # Partial / non-inheriting extension surfaces must cite a descriptor.
    if row == "extension_surface_inheritance":
        inh = binding.get("projected_inheritance_state")
        if inh in ("partial", "does_not_inherit") and not binding.get("report_ref"):
            findings.append(
                Finding(
                    "inheritance_gap_hidden",
                    "an extension inheritance gap cites no descriptor report",
                    surface_id=surface_id,
                    row=row,
                )
            )

    if binding.get("marketed_on_row") and binding.get("evidence_freshness") == "stale":
        findings.append(
            Finding(
                "stale_evidence_on_marketed_row",
                "marketed row carries stale appearance evidence",
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

    anchor = descriptor.get("appearance_anchor_ref")
    if not isinstance(anchor, str) or not anchor.strip():
        findings.append(
            Finding(
                "descriptor_missing_appearance_anchor",
                "descriptor.appearance_anchor_ref must be non-empty",
                surface_id=surface_id,
            )
        )

    note = descriptor.get("accessibility_note")
    if not isinstance(note, str) or not note.strip():
        findings.append(
            Finding(
                "missing_accessibility_note",
                "descriptor.accessibility_note must be non-empty",
                surface_id=surface_id,
            )
        )

    if descriptor.get("registered_on_appearance_session") is not True:
        findings.append(
            Finding(
                "surface_not_on_appearance_session",
                "descriptor.registered_on_appearance_session must be true",
                surface_id=surface_id,
            )
        )

    # Every required row must be bound.
    bindings = ensure_list(surface.get("bindings", []), "surface.bindings")
    present = {binding.get("row") for binding in bindings}
    for required in REQUIRED_ROWS:
        if required not in present:
            findings.append(
                Finding(
                    "missing_required_row",
                    "surface is missing a required parity row binding",
                    surface_id=surface_id,
                    row=required,
                )
            )

    for binding in bindings:
        row = binding.get("row")
        dimension = binding.get("dimension")
        expected_dimension = ROW_TO_DIMENSION.get(row)
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
        expected_family = ROW_TO_FAMILY.get(row)
        if expected_family is not None and binding.get("object_family") != expected_family:
            findings.append(
                Finding(
                    "dimension_drift",
                    "binding object_family disagrees with its row's canonical family",
                    surface_id=surface_id,
                    row=row,
                    detail={"object_family": binding.get("object_family"), "expected": expected_family},
                )
            )
        status = binding.get("qualification_status")
        if status == "hidden_downgrade":
            findings.append(
                Finding(
                    "hidden_downgrade",
                    "surface hides an appearance-object downgrade",
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
        elif status in ("explicitly_narrowed", "not_applicable", "platform_omitted", "declared_capture_gap"):
            if not binding.get("narrowing_reason"):
                findings.append(
                    Finding(
                        "missing_narrowing_reason",
                        "a narrowed / not-applicable / declared-gap row must name a reason",
                        surface_id=surface_id,
                        row=row,
                    )
                )
        elif status == "qualified":
            check_qualified_binding(surface_id, binding, findings)

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
        findings.append(Finding("published_markdown_missing", f"missing published markdown: {MARKDOWN_REL}"))
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
                    "companion doc must quote every required parity row",
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
    if not (repo_root / SCHEMA_REL).exists():
        raise SystemExit(f"missing required input: {SCHEMA_REL}")

    findings: list[Finding] = []
    check_report_envelope(report, findings)
    check_object_model_index(repo_root, report, findings)
    check_required_rows_qualified(report, findings)
    for surface in ensure_list(report.get("rows", []), "report.rows"):
        check_surface(ensure_dict(surface, "surface"), findings)
    check_support_export(report, export, findings)
    check_publications(repo_root, findings)

    if args.format == "json":
        print(json.dumps({"findings": [f.as_dict() for f in findings]}, indent=2))
    else:
        if not findings:
            print("m5 theme-import-parity audit: clean")
        else:
            for finding in findings:
                location = finding.surface_id or "report"
                if finding.row:
                    location = f"{location} / {finding.row}"
                print(f"FAIL [{finding.code}] {location}: {finding.message}")

    return 1 if findings else 0


if __name__ == "__main__":
    sys.exit(main())
