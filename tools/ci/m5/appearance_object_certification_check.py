#!/usr/bin/env python3
"""M5 appearance-object certification gate.

This gate enforces that the checked-in appearance-object certification report
stays fresh, honest, and clean: the canonical object-model index registers all
five appearance-object families (theme package, appearance session, token
overlay, imported-theme report, extension appearance descriptor), every claimed
M5 surface is certified across all five, each family certification is backed by
its canonical source report, the certified claim scope is the derived
auto-narrowed value (never an asserted one), and no surface keeps a full claim
while an underlying object is missing, stale, or hiding a downgrade. It reads:

- the report fixture at
  ``fixtures/ux/m5/appearance-object-certification/report.json``;
- the support-export fixture at
  ``fixtures/ux/m5/appearance-object-certification/support_export.json``;
- the boundary schema at
  ``schemas/ux/m5-appearance-object-certification.schema.json``; and
- the published report at
  ``artifacts/ux/m5/theme-package-certification/m5_appearance_object_certification.md``
  and the companion doc at
  ``docs/m5/appearance-object-certification.md``.

The typed Rust consumer mints the same report, so
``cargo test -p aureline-shell`` enforces the same structural invariants and
that the fixtures are bit-for-bit equal to the seed.

Exit codes:

- ``0`` -- report is clean.
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

REPORT_REL = Path("fixtures/ux/m5/appearance-object-certification/report.json")
SUPPORT_EXPORT_REL = Path("fixtures/ux/m5/appearance-object-certification/support_export.json")
SCHEMA_REL = Path("schemas/ux/m5-appearance-object-certification.schema.json")
MARKDOWN_REL = Path("artifacts/ux/m5/theme-package-certification/m5_appearance_object_certification.md")
DOC_REL = Path("docs/m5/appearance-object-certification.md")

EXPECTED_RECORD_KIND_REPORT = "shell_m5_appearance_object_certification_report_record"
EXPECTED_RECORD_KIND_SURFACE = "shell_m5_appearance_object_certification_surface_record"
EXPECTED_RECORD_KIND_SUPPORT = "shell_m5_appearance_object_certification_support_export_record"
EXPECTED_SHARED_CONTRACT_REF = "shell:m5_appearance_object_certification:v1"
EXPECTED_SCHEMA_VERSION = 1

# The five canonical appearance-object families, in canonical order, each bound
# to the source report id the certification must be backed by. These mirror the
# constants the Rust family modules export.
FAMILY_SOURCE_REPORT_ID = {
    "theme_package": "shell:m5_theme_packages:audit:v1",
    "appearance_session": "shell:m5_appearance_session:runtime:v1",
    "token_overlay": "shell:m5_token_overlays:portability:v1",
    "theme_import_report": "shell:m5_theme_import_report:v1:default",
    "extension_appearance_descriptor": "extensions:m5_appearance_descriptor:audit:v1",
}
OBJECT_FAMILIES = list(FAMILY_SOURCE_REPORT_ID.keys())

REQUIRED_SURFACE_FAMILIES = {
    "notebook_cell_chrome",
    "result_grid_row",
    "profiler_panel",
    "trace_panel",
    "pipeline_card",
    "preview_route_badge",
    "docs_browser_pane",
    "companion_surface",
    "sync_status_surface",
    "offboarding_surface",
}

CERTIFIED_STATUS = "qualified"
BLOCKING_STATUSES = {"missing_evidence", "unqualified_local_appearance"}
# Statuses (other than a disclosed downgrade) that narrow a surface's claim.
NARROWING_STATUSES = {"explicitly_narrowed", "platform_omitted", "declared_capture_gap"}
# Statuses that require a narrowing reason.
REASON_REQUIRED_STATUSES = NARROWING_STATUSES | {"not_applicable"}

ROUTING_REF_FIELDS = (
    "release_evidence_refs",
    "extension_inspection_refs",
    "sync_refs",
    "claim_publication_refs",
)

DOC_BACKLINKS = (
    "artifacts/ux/m5/theme-package-certification/m5_appearance_object_certification.md",
    "fixtures/ux/m5/appearance-object-certification/report.json",
    "schemas/ux/m5-appearance-object-certification.schema.json",
    "tools/ci/m5/appearance_object_certification_check.py",
)


@dataclass
class Finding:
    """One blocking finding emitted by the gate."""

    code: str
    message: str
    subject: str | None = None
    detail: dict[str, Any] = field(default_factory=dict)

    def as_dict(self) -> dict[str, Any]:
        out: dict[str, Any] = {"code": self.code, "message": self.message}
        if self.subject is not None:
            out["subject"] = self.subject
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


def is_certified(certification: dict[str, Any]) -> bool:
    return certification.get("certification_status") == CERTIFIED_STATUS


def reduces_claim_scope(certification: dict[str, Any]) -> bool:
    narrowed_status = certification.get("certification_status") in NARROWING_STATUSES
    disclosed_downgrade = is_certified(certification) and certification.get("compatibility_state") != "current"
    return narrowed_status or disclosed_downgrade


def requires_reason(certification: dict[str, Any]) -> bool:
    status_requires = certification.get("certification_status") in REASON_REQUIRED_STATUSES
    disclosed_downgrade = is_certified(certification) and certification.get("compatibility_state") != "current"
    return status_requires or disclosed_downgrade


def has_reason(certification: dict[str, Any]) -> bool:
    return bool(str(certification.get("narrowing_reason") or "").strip())


def recompute_scope(surface: dict[str, Any]) -> str:
    blocked = False
    narrowed = False
    for certification in surface.get("family_certifications", []):
        if certification.get("certification_status") in BLOCKING_STATUSES:
            blocked = True
        if certification.get("compatibility_state") != "current" and certification.get("downgrade_disclosed") is not True:
            blocked = True
        if is_certified(certification) and certification.get("evidence_freshness") == "stale":
            blocked = True
        if reduces_claim_scope(certification):
            narrowed = True
    if blocked:
        return "blocked"
    if narrowed:
        return "certified_narrowed"
    return "certified_full"


def check_report_envelope(repo_root: Path, report: dict[str, Any], findings: list[Finding]) -> None:
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
            Finding("report_schema_version_mismatch", f"report.schema_version must be {EXPECTED_SCHEMA_VERSION}")
        )
    if report.get("shared_contract_ref") != EXPECTED_SHARED_CONTRACT_REF:
        findings.append(
            Finding("report_shared_contract_ref_mismatch", f"report.shared_contract_ref must be {EXPECTED_SHARED_CONTRACT_REF}")
        )
    source_schema_ref = report.get("source_schema_ref")
    if not isinstance(source_schema_ref, str) or not (repo_root / source_schema_ref).exists():
        findings.append(
            Finding(
                "source_schema_ref_missing",
                "report.source_schema_ref must point at an existing schema",
                detail={"source_schema_ref": source_schema_ref},
            )
        )
    if not str(report.get("build_identity_ref", "")).strip():
        findings.append(Finding("build_identity_ref_missing", "report.build_identity_ref must be non-empty"))
    if not str(report.get("release_channel_class", "")).strip():
        findings.append(Finding("release_channel_class_missing", "report.release_channel_class must be non-empty"))
    if report.get("report_clean") is not True:
        findings.append(Finding("report_not_clean", "report.report_clean must be true"))
    if report.get("all_surfaces_publishable") is not True:
        findings.append(Finding("surfaces_not_publishable", "report.all_surfaces_publishable must be true"))
    if report.get("blocking_findings"):
        findings.append(
            Finding(
                "blocking_findings_present",
                "report.blocking_findings must be empty",
                detail={"count": len(report.get("blocking_findings", []))},
            )
        )
    for ref_field in ROUTING_REF_FIELDS:
        refs = report.get(ref_field)
        if not isinstance(refs, list) or not refs:
            findings.append(
                Finding("routing_ref_missing", f"report.{ref_field} must be a non-empty array", detail={"field": ref_field})
            )


def check_object_model_index(report: dict[str, Any], findings: list[Finding]) -> None:
    index = report.get("object_model_index")
    if not isinstance(index, list):
        findings.append(Finding("index_missing", "report.object_model_index must be an array"))
        return
    seen = {}
    for entry in index:
        if not isinstance(entry, dict):
            continue
        family = entry.get("object_family")
        seen[family] = entry
        canonical = FAMILY_SOURCE_REPORT_ID.get(family)
        if canonical is not None and entry.get("source_report_id") != canonical:
            findings.append(
                Finding(
                    "index_source_report_id_mismatch",
                    "object-model index source report id must match the family's canonical report",
                    subject=family,
                    detail={"expected": canonical, "declared": entry.get("source_report_id")},
                )
            )
        schema_ref = entry.get("canonical_schema_ref")
        if not str(schema_ref or "").strip():
            findings.append(Finding("index_schema_ref_missing", "object-model index entry must name a canonical schema", subject=family))
    for family in OBJECT_FAMILIES:
        if family not in seen:
            findings.append(Finding("index_family_missing", "object-model index must register every family", subject=family))


def check_surface(surface: dict[str, Any], findings: list[Finding]) -> None:
    certification_id = surface.get("certification_id") or "<unknown>"

    if surface.get("record_kind") != EXPECTED_RECORD_KIND_SURFACE:
        findings.append(
            Finding("surface_record_kind_mismatch", f"surface.record_kind must be {EXPECTED_RECORD_KIND_SURFACE}", subject=certification_id)
        )
    if not surface.get("docs_help_refs"):
        findings.append(Finding("surface_docs_help_ref_missing", "surface must carry a docs/help ref", subject=certification_id))

    certifications = surface.get("family_certifications", [])
    present_families = {c.get("object_family") for c in certifications if isinstance(c, dict)}
    for family in OBJECT_FAMILIES:
        if family not in present_families:
            findings.append(Finding("surface_family_missing", "a surface certification must cover every family", subject=certification_id, detail={"family": family}))

    for certification in certifications:
        check_family_certification(surface, certification_id, certification, findings)

    declared_scope = surface.get("certified_claim_scope")
    derived_scope = recompute_scope(surface)
    if declared_scope != derived_scope:
        findings.append(
            Finding(
                "claim_scope_stale",
                "surface.certified_claim_scope must match the derived auto-narrowed scope",
                subject=certification_id,
                detail={"declared": declared_scope, "derived": derived_scope},
            )
        )
    if derived_scope != "certified_full" and not str(surface.get("narrowing_reason") or "").strip():
        findings.append(Finding("surface_narrowed_without_reason", "a narrowed/blocked surface must disclose why", subject=certification_id))

    # all_families_current must match the certified families' compatibility.
    expected_current = all(
        c.get("compatibility_state") == "current" for c in certifications if is_certified(c)
    )
    if surface.get("all_families_current") != expected_current:
        findings.append(
            Finding(
                "all_families_current_stale",
                "surface.all_families_current does not match the certified families",
                subject=certification_id,
                detail={"expected": expected_current, "declared": surface.get("all_families_current")},
            )
        )


def check_family_certification(surface: dict[str, Any], certification_id: str, certification: dict[str, Any], findings: list[Finding]) -> None:
    family = certification.get("object_family")
    detail = {"family": family}

    if certification.get("certification_status") in BLOCKING_STATUSES:
        findings.append(Finding("family_missing_evidence", "a family must not claim appearance with no backing evidence", subject=certification_id, detail=detail))

    if certification.get("compatibility_state") != "current" and certification.get("downgrade_disclosed") is not True:
        findings.append(Finding("hidden_downgrade", "a non-current compatibility state must be disclosed up front", subject=certification_id, detail=detail))

    if is_certified(certification) and certification.get("evidence_freshness") == "stale":
        findings.append(Finding("stale_evidence_on_certified_family", "a certified family must carry fresh evidence", subject=certification_id, detail=detail))

    if requires_reason(certification) and not has_reason(certification):
        findings.append(Finding("missing_family_narrowing_reason", "a narrowed or downgraded family must carry a reason", subject=certification_id, detail=detail))

    canonical = FAMILY_SOURCE_REPORT_ID.get(family)
    if canonical is not None and certification.get("source_report_id") != canonical:
        findings.append(
            Finding(
                "unbacked_family_source",
                "a family certification must cite its canonical source report",
                subject=certification_id,
                detail={"family": family, "expected": canonical, "declared": certification.get("source_report_id")},
            )
        )


def check_coverage(report: dict[str, Any], surfaces: list[dict[str, Any]], findings: list[Finding]) -> None:
    covered = sorted({surface.get("surface_family") for surface in surfaces})
    if report.get("covered_surface_families") != covered:
        findings.append(
            Finding(
                "surface_coverage_stale",
                "report.covered_surface_families does not match the surfaces",
                detail={"expected": covered, "declared": report.get("covered_surface_families")},
            )
        )
    for surface_family in sorted(REQUIRED_SURFACE_FAMILIES - set(covered)):
        findings.append(Finding("uncertified_required_surface", "a claimed M5 surface has no certification", subject=surface_family))

    scopes = [recompute_scope(surface) for surface in surfaces]
    expected_counts = {
        "surface_count": len(surfaces),
        "certified_full_surface_count": scopes.count("certified_full"),
        "narrowed_surface_count": scopes.count("certified_narrowed"),
        "blocked_surface_count": scopes.count("blocked"),
    }
    for key, value in expected_counts.items():
        if report.get(key) != value:
            findings.append(
                Finding("summary_count_stale", f"report.{key} does not match the surfaces", detail={"expected": value, "declared": report.get(key)})
            )


def check_support_export(report: dict[str, Any], export: dict[str, Any], findings: list[Finding]) -> None:
    if export.get("record_kind") != EXPECTED_RECORD_KIND_SUPPORT:
        findings.append(
            Finding("support_record_kind_mismatch", f"support_export.record_kind must be {EXPECTED_RECORD_KIND_SUPPORT}")
        )
    case_ids = export.get("case_ids")
    if not isinstance(case_ids, list):
        findings.append(Finding("support_case_ids_missing", "support_export.case_ids must be an array"))
        return
    case_set = set(case_ids)
    if report.get("report_id") not in case_set:
        findings.append(Finding("support_missing_report_id", "case_ids must quote the report id"))
    if report.get("build_identity_ref") not in case_set:
        findings.append(Finding("support_missing_build_ref", "case_ids must quote the exact-build ref"))
    for entry in report.get("object_model_index", []):
        if entry.get("source_report_id") not in case_set:
            findings.append(Finding("support_missing_index_report", "case_ids must quote every index source report id", subject=entry.get("object_family")))
        if entry.get("canonical_schema_ref") not in case_set:
            findings.append(Finding("support_missing_index_schema", "case_ids must quote every index canonical schema ref", subject=entry.get("object_family")))
    for surface in report.get("surfaces", []):
        certification_id = surface.get("certification_id")
        if certification_id not in case_set:
            findings.append(Finding("support_missing_surface_id", "case_ids must quote every certification id", subject=certification_id))
        for certification in surface.get("family_certifications", []):
            for ref in certification.get("evidence_refs", []):
                if ref not in case_set:
                    findings.append(Finding("support_missing_evidence_ref", "case_ids must quote every family evidence ref", subject=certification_id))


def check_publications(repo_root: Path, findings: list[Finding]) -> None:
    markdown = repo_root / MARKDOWN_REL
    if not markdown.exists():
        findings.append(Finding("published_markdown_missing", f"missing published markdown: {MARKDOWN_REL}"))
    doc = repo_root / DOC_REL
    if not doc.exists():
        findings.append(Finding("published_doc_missing", f"missing companion doc: {DOC_REL}"))
        return
    body = doc.read_text(encoding="utf-8")
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

    surfaces = ensure_list(report.get("surfaces", []), "report.surfaces")

    findings: list[Finding] = []
    check_report_envelope(repo_root, report, findings)
    check_object_model_index(report, findings)
    if not surfaces:
        findings.append(Finding("surfaces_empty", "report.surfaces must be non-empty"))
    check_coverage(report, surfaces, findings)
    for surface in surfaces:
        check_surface(ensure_dict(surface, "surface"), findings)
    check_support_export(report, export, findings)
    check_publications(repo_root, findings)

    if args.format == "json":
        print(json.dumps({"findings": [f.as_dict() for f in findings]}, indent=2))
    else:
        if not findings:
            print("m5 appearance-object certification: clean")
        else:
            for finding in findings:
                location = finding.subject or "report"
                print(f"FAIL [{finding.code}] {location}: {finding.message}")

    return 1 if findings else 0


if __name__ == "__main__":
    sys.exit(main())
