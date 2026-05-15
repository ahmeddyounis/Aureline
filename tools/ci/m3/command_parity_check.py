#!/usr/bin/env python3
"""M3 beta command-parity diff report CI gate.

This gate enforces that the checked-in command-parity report stays
fresh and clean across the five required surface families: command
palette, menus/buttons, keybinding help, CLI/headless help, and AI
tool surfaces. It reads:

- the parity diff fixture at
  ``fixtures/commands/m3/command_parity/report.json``;
- the support-export fixture at
  ``fixtures/commands/m3/command_parity/support_export.json``;
- the boundary schema at
  ``schemas/commands/command_parity.schema.json``; and
- (when present) the published markdown at
  ``artifacts/ux/m3/command_parity_diff_report.md`` and the companion
  doc at ``docs/ux/m3/command_parity_diff_report.md``.

For each enforced row the gate verifies that:

- the report covers all five required surface families and at least
  one row claims each surface;
- every claimed beta command has a projection for every required
  surface family;
- every high-risk command (non-trivial preview class or non-trivial
  capability scope) has no ``unknown_high_risk_gap`` projection;
- every claimed surface row matches the descriptor for command id,
  primary label, lifecycle label, preview class, disabled-reason
  mode, docs/help anchor, and aliases (no surface alias may sit
  outside the descriptor-owned set);
- every non-claimed row carries a ``narrowing_reason``;
- the support-export wrapper quotes every command id and descriptor
  revision the report exposes; and
- the published markdown report and the companion doc are present and
  back-link the canonical schema, fixtures, and CLI gate.

Exit codes:

- ``0`` -- report is clean (all five surfaces claimed, no blockers).
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


REPORT_REL = Path("fixtures/commands/m3/command_parity/report.json")
SUPPORT_EXPORT_REL = Path("fixtures/commands/m3/command_parity/support_export.json")
COMPACT_REL = Path("fixtures/commands/m3/command_parity/compact.txt")
SCHEMA_REL = Path("schemas/commands/command_parity.schema.json")
MARKDOWN_REL = Path("artifacts/ux/m3/command_parity_diff_report.md")
DOC_REL = Path("docs/ux/m3/command_parity_diff_report.md")

REQUIRED_SURFACES = (
    "command_palette",
    "menu_or_button",
    "keybinding_help",
    "cli_headless",
    "ai_tool_surface",
)

EXPECTED_RECORD_KIND_REPORT = "shell_command_parity_beta_diff_report_record"
EXPECTED_RECORD_KIND_ROW = "shell_command_parity_beta_row_record"
EXPECTED_RECORD_KIND_SUPPORT = "shell_command_parity_beta_support_export_record"
EXPECTED_SHARED_CONTRACT_REF = "shell:command_parity_beta:v1"
EXPECTED_SCHEMA_VERSION = 1

HIGH_RISK_PREVIEW_CLASSES = {
    "structured_diff_preview",
    "destructive_bulk_mutation_preview",
    "policy_authoring_or_waiver_preview",
    "irreversible_publish_preview",
}
HIGH_RISK_CAPABILITY_SCOPES = {
    "recoverable_durable_mutation",
    "destructive_bulk_mutation",
    "irreversible_publish",
}


@dataclass
class Finding:
    """One blocking finding emitted by the gate."""

    code: str
    message: str
    command_id: str | None = None
    surface_family: str | None = None
    detail: dict[str, Any] = field(default_factory=dict)

    def as_dict(self) -> dict[str, Any]:
        out: dict[str, Any] = {"code": self.code, "message": self.message}
        if self.command_id is not None:
            out["command_id"] = self.command_id
        if self.surface_family is not None:
            out["surface_family"] = self.surface_family
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


def descriptor_high_risk(descriptor: dict[str, Any]) -> bool:
    preview = descriptor.get("preview_class")
    capability = descriptor.get("capability_scope_class")
    return (
        preview in HIGH_RISK_PREVIEW_CLASSES
        or capability in HIGH_RISK_CAPABILITY_SCOPES
    )


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
    required = list(REQUIRED_SURFACES)
    declared = report.get("required_surface_families")
    if declared != required:
        findings.append(
            Finding(
                "required_surface_families_mismatch",
                "required_surface_families must equal the canonical surface family list",
                detail={"required": required, "declared": declared},
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


def check_required_surfaces_claimed(
    report: dict[str, Any], findings: list[Finding]
) -> None:
    rows = ensure_list(report.get("rows", []), "report.rows")
    for required in REQUIRED_SURFACES:
        any_claimed = False
        for row in rows:
            for projection in ensure_list(row.get("surfaces", []), "row.surfaces"):
                if (
                    projection.get("surface_family") == required
                    and projection.get("coverage_status") == "claimed"
                ):
                    any_claimed = True
                    break
            if any_claimed:
                break
        if not any_claimed:
            findings.append(
                Finding(
                    "required_surface_not_claimed",
                    "no claimed row for required surface family",
                    surface_family=required,
                )
            )


def check_row(row: dict[str, Any], findings: list[Finding]) -> None:
    descriptor = ensure_dict(row.get("descriptor", {}), "row.descriptor")
    command_id = descriptor.get("command_id")
    if not isinstance(command_id, str) or not command_id.strip():
        findings.append(
            Finding(
                "missing_command_id",
                "descriptor.command_id must be a non-empty string",
            )
        )
        return

    if row.get("record_kind") != EXPECTED_RECORD_KIND_ROW:
        findings.append(
            Finding(
                "row_record_kind_mismatch",
                f"row.record_kind must be {EXPECTED_RECORD_KIND_ROW}",
                command_id=command_id,
                detail={"record_kind": row.get("record_kind")},
            )
        )

    revision = descriptor.get("descriptor_revision_ref")
    if not isinstance(revision, str) or not revision.strip():
        findings.append(
            Finding(
                "missing_descriptor_revision_ref",
                "descriptor.descriptor_revision_ref must be a non-empty string",
                command_id=command_id,
            )
        )

    high_risk = descriptor_high_risk(descriptor)
    declared_high_risk = bool(row.get("high_risk"))
    if declared_high_risk != high_risk:
        findings.append(
            Finding(
                "high_risk_flag_mismatch",
                "row.high_risk must reflect descriptor preview/scope",
                command_id=command_id,
                detail={"declared": declared_high_risk, "expected": high_risk},
            )
        )

    surfaces_present: set[str] = set()
    canonical_aliases = set(descriptor.get("canonical_aliases", []))

    for projection in ensure_list(row.get("surfaces", []), "row.surfaces"):
        family = projection.get("surface_family")
        if family is None:
            findings.append(
                Finding(
                    "missing_surface_family",
                    "surface projection must declare surface_family",
                    command_id=command_id,
                )
            )
            continue
        surfaces_present.add(family)

        coverage = projection.get("coverage_status")
        if coverage == "unknown_high_risk_gap":
            if high_risk:
                findings.append(
                    Finding(
                        "unknown_high_risk_gap",
                        "high-risk command has no claim or explicit narrowing on this surface",
                        command_id=command_id,
                        surface_family=family,
                    )
                )
            continue

        if coverage in (
            "explicitly_narrowed",
            "discoverable_only",
            "browser_handoff_only",
            "voice_addressable",
            "not_surfaced_on_this_client",
        ):
            reason = projection.get("narrowing_reason")
            if not isinstance(reason, str) or not reason.strip():
                findings.append(
                    Finding(
                        "missing_narrowing_reason",
                        "non-claimed projection must declare narrowing_reason",
                        command_id=command_id,
                        surface_family=family,
                        detail={"coverage_status": coverage},
                    )
                )
            continue

        if coverage != "claimed":
            findings.append(
                Finding(
                    "unknown_coverage_status",
                    "coverage_status is not in the supported enum",
                    command_id=command_id,
                    surface_family=family,
                    detail={"coverage_status": coverage},
                )
            )
            continue

        for field_name in (
            "projected_command_id",
            "projected_label_ref",
            "projected_lifecycle_label",
            "projected_preview_class",
            "projected_disabled_reason_mode",
            "projected_docs_help_anchor_ref",
        ):
            if projection.get(field_name) in (None, ""):
                findings.append(
                    Finding(
                        "missing_projection",
                        f"claimed surface must populate {field_name}",
                        command_id=command_id,
                        surface_family=family,
                        detail={"field": field_name},
                    )
                )

        if projection.get("projected_command_id") not in (None, command_id):
            findings.append(
                Finding(
                    "command_id_drift",
                    "claimed surface command id disagrees with descriptor",
                    command_id=command_id,
                    surface_family=family,
                    detail={
                        "projected_command_id": projection.get("projected_command_id")
                    },
                )
            )
        for projected_field, descriptor_field in (
            ("projected_label_ref", "primary_label_ref"),
            ("projected_lifecycle_label", "lifecycle_label"),
            ("projected_preview_class", "preview_class"),
            ("projected_disabled_reason_mode", "disabled_reason_mode"),
            ("projected_docs_help_anchor_ref", "docs_help_anchor_ref"),
        ):
            projected_value = projection.get(projected_field)
            descriptor_value = descriptor.get(descriptor_field)
            if projected_value is None:
                continue
            if projected_value != descriptor_value:
                if projected_field == "projected_docs_help_anchor_ref":
                    code = "missing_docs_help_anchor"
                else:
                    code = projected_field.replace("projected_", "") + "_drift"
                findings.append(
                    Finding(
                        code,
                        f"claimed surface {projected_field} disagrees with descriptor",
                        command_id=command_id,
                        surface_family=family,
                        detail={
                            "projected": projected_value,
                            "descriptor": descriptor_value,
                        },
                    )
                )

        for alias in projection.get("projected_aliases", []) or []:
            if alias not in canonical_aliases:
                findings.append(
                    Finding(
                        "alias_drift",
                        "surface exposes alias outside descriptor canonical alias set",
                        command_id=command_id,
                        surface_family=family,
                        detail={"alias": alias},
                    )
                )

    for required in REQUIRED_SURFACES:
        if required not in surfaces_present:
            findings.append(
                Finding(
                    "missing_required_surface",
                    "row is missing a required surface family projection",
                    command_id=command_id,
                    surface_family=required,
                )
            )


def check_support_export(
    report: dict[str, Any], support_export: dict[str, Any], findings: list[Finding]
) -> None:
    if support_export.get("record_kind") != EXPECTED_RECORD_KIND_SUPPORT:
        findings.append(
            Finding(
                "support_export_record_kind_mismatch",
                f"support_export.record_kind must be {EXPECTED_RECORD_KIND_SUPPORT}",
                detail={"record_kind": support_export.get("record_kind")},
            )
        )
    if support_export.get("shared_contract_ref") != EXPECTED_SHARED_CONTRACT_REF:
        findings.append(
            Finding(
                "support_export_shared_contract_ref_mismatch",
                f"support_export.shared_contract_ref must be {EXPECTED_SHARED_CONTRACT_REF}",
            )
        )
    quoted_report = ensure_dict(
        support_export.get("report", {}), "support_export.report"
    )
    if quoted_report.get("report_id") != report.get("report_id"):
        findings.append(
            Finding(
                "support_export_report_id_mismatch",
                "support_export.report.report_id must match the parity report",
                detail={
                    "support_export": quoted_report.get("report_id"),
                    "report": report.get("report_id"),
                },
            )
        )
    case_ids = set(support_export.get("case_ids", []) or [])
    expected_case_ids = {report.get("report_id")}
    for row in ensure_list(report.get("rows", []), "report.rows"):
        descriptor = ensure_dict(row.get("descriptor", {}), "row.descriptor")
        expected_case_ids.add(descriptor.get("command_id"))
        expected_case_ids.add(descriptor.get("descriptor_revision_ref"))
    expected_case_ids.discard(None)
    missing = expected_case_ids - case_ids
    for case in sorted(missing):
        findings.append(
            Finding(
                "support_export_missing_case_id",
                "support_export.case_ids must quote the report id, every command id, and every descriptor revision",
                detail={"missing_case_id": case},
            )
        )


def check_publications(repo_root: Path, findings: list[Finding]) -> None:
    md_path = repo_root / MARKDOWN_REL
    if not md_path.exists():
        findings.append(
            Finding(
                "published_markdown_missing",
                f"published markdown report must exist at {MARKDOWN_REL}",
            )
        )
    else:
        body = md_path.read_text(encoding="utf-8")
        if "Beta command-parity diff report" not in body:
            findings.append(
                Finding(
                    "published_markdown_unexpected_body",
                    "published markdown does not contain expected title heading",
                )
            )

    doc_path = repo_root / DOC_REL
    if not doc_path.exists():
        findings.append(
            Finding(
                "companion_doc_missing",
                f"companion doc must exist at {DOC_REL}",
            )
        )
    else:
        body = doc_path.read_text(encoding="utf-8")
        for required in REQUIRED_SURFACES:
            if required not in body:
                findings.append(
                    Finding(
                        "companion_doc_missing_surface",
                        "companion doc must quote every required surface family",
                        detail={"surface_family": required},
                    )
                )
        if "/schemas/commands/command_parity.schema.json" not in body:
            findings.append(
                Finding(
                    "companion_doc_missing_schema_link",
                    "companion doc must back-link the parity schema",
                )
            )
        if "tools/ci/m3/command_parity_check.py" not in body:
            findings.append(
                Finding(
                    "companion_doc_missing_gate_link",
                    "companion doc must back-link the CI gate",
                )
            )

    schema_path = repo_root / SCHEMA_REL
    if not schema_path.exists():
        findings.append(
            Finding(
                "schema_missing",
                f"parity schema must exist at {SCHEMA_REL}",
            )
        )


def main() -> int:
    args = parse_args()
    repo_root = Path(args.repo_root).resolve()

    report_path = repo_root / REPORT_REL
    support_export_path = repo_root / SUPPORT_EXPORT_REL

    report = ensure_dict(load_json(report_path), "report")
    support_export = ensure_dict(
        load_json(support_export_path), "support_export"
    )

    findings: list[Finding] = []
    check_report_envelope(report, findings)
    check_required_surfaces_claimed(report, findings)

    rows = ensure_list(report.get("rows", []), "report.rows")
    if not rows:
        findings.append(
            Finding(
                "no_claimed_commands",
                "report.rows must contain at least one claimed command",
            )
        )
    for row in rows:
        check_row(ensure_dict(row, "report.rows[*]"), findings)

    check_support_export(report, support_export, findings)
    check_publications(repo_root, findings)

    declared_clean = bool(report.get("report_clean"))
    if declared_clean and findings:
        findings.append(
            Finding(
                "report_clean_flag_mismatch",
                "report.report_clean is true but findings were detected",
            )
        )

    if args.format == "json":
        payload = {
            "report_id": report.get("report_id"),
            "report_clean": declared_clean,
            "findings": [finding.as_dict() for finding in findings],
        }
        json.dump(payload, sys.stdout, indent=2, sort_keys=False)
        sys.stdout.write("\n")
    else:
        if not findings:
            print(
                f"command_parity_check: ok ({report.get('report_id')}, "
                f"commands={report.get('claimed_command_count')}, "
                f"surface_rows={report.get('surface_rows_checked')})"
            )
        else:
            print(
                f"command_parity_check: {len(findings)} finding(s) for "
                f"{report.get('report_id')}",
                file=sys.stderr,
            )
            for finding in findings:
                location = []
                if finding.command_id:
                    location.append(finding.command_id)
                if finding.surface_family:
                    location.append(finding.surface_family)
                location_str = " ".join(location) or "-"
                print(
                    f"  [{finding.code}] {location_str}: {finding.message}",
                    file=sys.stderr,
                )

    return 1 if findings else 0


if __name__ == "__main__":
    sys.exit(main())
