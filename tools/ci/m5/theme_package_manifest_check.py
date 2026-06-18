#!/usr/bin/env python3
"""M5 theme-package manifest audit CI gate.

This gate enforces that the checked-in M5 theme-package manifest audit stays
fresh and clean: every claimed M5 surface (notebook, result grid, profiler
timeline, preview/browser pane, docs/help pane, companion surface,
extension-backed surface) declares its active theme package and supported
appearance modes through one shared manifest shape, theme provenance and
supported-mode metadata survive into the support-export wrapper, and no surface
hides an appearance downgrade behind feature-local style code. It reads:

- the audit fixture at ``fixtures/ux/m5/theme-package-modes/report.json``;
- the support-export fixture at
  ``fixtures/ux/m5/theme-package-modes/support_export.json``;
- the boundary schema at
  ``schemas/ux/m5-theme-package-manifest.schema.json``;
- the canonical theme-package manifest schema at
  ``schemas/ux/theme_package_manifest.schema.json`` (referenced by the audit);
  and
- (when present) the published markdown at
  ``artifacts/ux/m5/theme-manifest-audit/m5_theme_package_manifest_audit.md``
  and the companion doc at ``docs/m5/theme-package-manifests.md``.

For the audit the gate verifies that:

- every registered surface resolves its ``active_package_id`` to a manifest;
- every honoured theme / density / motion mode is one the active package
  supports;
- every inheritance axis the active package expects is either inherited or
  disclosed as a gap, and the inheritance posture agrees with the disclosed
  gaps;
- every surface discloses its provenance and rides the shared
  appearance-session model, and carries a canonical appearance anchor and a
  non-empty accessibility note;
- no marketed surface carries stale evidence and no disabled package renders
  without disclosure;
- first-party manifests carry the semantic, component, and syntax token sets
  and cover the dark, light, and reduced-motion modes the product already
  claims, and no signature-failed manifest is registered;
- no surface carries any blocking finding;
- the support-export wrapper quotes the report id, every package id and
  revision, and every surface id and descriptor revision; and
- the published markdown audit and the companion doc are present and back-link
  the canonical schema, fixtures, and CLI gate.

Exit codes:

- ``0`` -- audit is clean (no blockers).
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

REPORT_REL = Path("fixtures/ux/m5/theme-package-modes/report.json")
SUPPORT_EXPORT_REL = Path("fixtures/ux/m5/theme-package-modes/support_export.json")
SCHEMA_REL = Path("schemas/ux/m5-theme-package-manifest.schema.json")
CANONICAL_SCHEMA_REL = Path("schemas/ux/theme_package_manifest.schema.json")
MARKDOWN_REL = Path("artifacts/ux/m5/theme-manifest-audit/m5_theme_package_manifest_audit.md")
DOC_REL = Path("docs/m5/theme-package-manifests.md")

EXPECTED_RECORD_KIND_REPORT = "shell_m5_theme_package_manifest_report_record"
EXPECTED_RECORD_KIND_SURFACE = "shell_m5_theme_package_surface_binding_record"
EXPECTED_RECORD_KIND_MANIFEST = "shell_m5_theme_package_manifest_record"
EXPECTED_RECORD_KIND_SUPPORT = "shell_m5_theme_package_manifest_support_export_record"
EXPECTED_SHARED_CONTRACT_REF = "shell:m5_theme_packages:v1"
EXPECTED_SCHEMA_VERSION = 1

REQUIRED_TOKEN_SET_KINDS = ("semantic", "component", "syntax")
FIRST_PARTY = "built_in_with_product"
REQUIRED_FIRST_PARTY_THEME_MODES = ("dark_reference", "light_parity")
REQUIRED_FIRST_PARTY_MOTION = "motion_reduced"

DOC_BACKLINKS = (
    "artifacts/ux/m5/theme-manifest-audit/m5_theme_package_manifest_audit.md",
    "fixtures/ux/m5/theme-package-modes/report.json",
    "schemas/ux/m5-theme-package-manifest.schema.json",
    "schemas/ux/theme_package_manifest.schema.json",
    "tools/ci/m5/theme_package_manifest_check.py",
)


@dataclass
class Finding:
    """One blocking finding emitted by the gate."""

    code: str
    message: str
    surface_id: str | None = None
    package_id: str | None = None
    detail: dict[str, Any] = field(default_factory=dict)

    def as_dict(self) -> dict[str, Any]:
        out: dict[str, Any] = {"code": self.code, "message": self.message}
        if self.surface_id is not None:
            out["surface_id"] = self.surface_id
        if self.package_id is not None:
            out["package_id"] = self.package_id
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
    if not ensure_list(report.get("manifests", []), "report.manifests"):
        findings.append(Finding("no_manifests", "report.manifests must be non-empty"))
    if not ensure_list(report.get("surfaces", []), "report.surfaces"):
        findings.append(Finding("no_surfaces", "report.surfaces must be non-empty"))


def index_manifests(report: dict[str, Any]) -> dict[str, dict[str, Any]]:
    out: dict[str, dict[str, Any]] = {}
    for manifest in ensure_list(report.get("manifests", []), "report.manifests"):
        manifest = ensure_dict(manifest, "manifest")
        package_id = manifest.get("package_id")
        if isinstance(package_id, str):
            out[package_id] = manifest
    return out


def check_manifest(manifest: dict[str, Any], findings: list[Finding]) -> None:
    package_id = manifest.get("package_id")
    if manifest.get("record_kind") != EXPECTED_RECORD_KIND_MANIFEST:
        findings.append(
            Finding(
                "manifest_record_kind_mismatch",
                f"manifest.record_kind must be {EXPECTED_RECORD_KIND_MANIFEST}",
                package_id=package_id,
                detail={"record_kind": manifest.get("record_kind")},
            )
        )
    for ref_field in ("package_version_label", "package_revision_ref"):
        value = manifest.get(ref_field)
        if not isinstance(value, str) or not value.strip():
            findings.append(
                Finding(
                    "manifest_field_missing",
                    f"manifest.{ref_field} must be a non-empty string",
                    package_id=package_id,
                )
            )

    if manifest.get("signature_state") == "signature_failed_blocked":
        findings.append(
            Finding(
                "manifest_signature_failed_still_registered",
                "a signature-failed manifest must not be registered for rendering",
                package_id=package_id,
            )
        )

    if manifest.get("provenance_class") == FIRST_PARTY:
        kinds = {
            ts.get("kind")
            for ts in ensure_list(manifest.get("token_sets", []), "manifest.token_sets")
        }
        for required in REQUIRED_TOKEN_SET_KINDS:
            if required not in kinds:
                findings.append(
                    Finding(
                        "manifest_token_set_incomplete",
                        "first-party manifest is missing a required token set",
                        package_id=package_id,
                        detail={"token_set_kind": required},
                    )
                )
        modes = set(manifest.get("supported_theme_modes", []))
        for required in REQUIRED_FIRST_PARTY_THEME_MODES:
            if required not in modes:
                findings.append(
                    Finding(
                        "manifest_missing_required_mode",
                        "first-party manifest is missing a required theme mode",
                        package_id=package_id,
                        detail={"mode": required},
                    )
                )
        if REQUIRED_FIRST_PARTY_MOTION not in set(
            manifest.get("supported_motion_postures", [])
        ):
            findings.append(
                Finding(
                    "manifest_missing_required_mode",
                    "first-party manifest is missing the reduced-motion posture",
                    package_id=package_id,
                    detail={"mode": REQUIRED_FIRST_PARTY_MOTION},
                )
            )


def check_surface(
    surface: dict[str, Any],
    manifests: dict[str, dict[str, Any]],
    findings: list[Finding],
) -> None:
    descriptor = ensure_dict(surface.get("descriptor", {}), "surface.descriptor")
    surface_id = descriptor.get("surface_id")
    if not isinstance(surface_id, str) or not surface_id.strip():
        findings.append(Finding("missing_surface_id", "descriptor.surface_id must be non-empty"))
        return

    if surface.get("record_kind") != EXPECTED_RECORD_KIND_SURFACE:
        findings.append(
            Finding(
                "surface_record_kind_mismatch",
                f"surface.record_kind must be {EXPECTED_RECORD_KIND_SURFACE}",
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

    if surface.get("provenance_disclosed") is not True:
        findings.append(
            Finding(
                "provenance_not_disclosed",
                "surface.provenance_disclosed must be true",
                surface_id=surface_id,
            )
        )

    active_package_id = surface.get("active_package_id")
    manifest = manifests.get(active_package_id) if isinstance(active_package_id, str) else None
    if manifest is None:
        findings.append(
            Finding(
                "active_package_unknown",
                "surface names an active package not in the registry",
                surface_id=surface_id,
                package_id=active_package_id,
            )
        )
    else:
        supported_themes = set(manifest.get("supported_theme_modes", []))
        supported_densities = set(manifest.get("supported_density_classes", []))
        supported_motion = set(manifest.get("supported_motion_postures", []))
        for mode in surface.get("honored_theme_modes", []):
            if mode not in supported_themes:
                findings.append(
                    Finding(
                        "unsupported_mode_claimed",
                        "surface honours a theme mode the package does not support",
                        surface_id=surface_id,
                        detail={"mode": mode},
                    )
                )
        for mode in surface.get("honored_density_classes", []):
            if mode not in supported_densities:
                findings.append(
                    Finding(
                        "unsupported_mode_claimed",
                        "surface honours a density the package does not support",
                        surface_id=surface_id,
                        detail={"mode": mode},
                    )
                )
        for mode in surface.get("honored_motion_postures", []):
            if mode not in supported_motion:
                findings.append(
                    Finding(
                        "unsupported_mode_claimed",
                        "surface honours a motion posture the package does not support",
                        surface_id=surface_id,
                        detail={"mode": mode},
                    )
                )

        inherited = set(surface.get("inherited_axes", []))
        disclosed = set(surface.get("disclosed_inheritance_gaps", []))
        for axis in manifest.get("inheritance_expectations", []):
            if axis not in inherited and axis not in disclosed:
                findings.append(
                    Finding(
                        "inheritance_gap_hidden",
                        "surface neither inherits nor discloses an expected axis",
                        surface_id=surface_id,
                        detail={"axis": axis},
                    )
                )

    posture = surface.get("inheritance_posture")
    has_gaps = bool(surface.get("disclosed_inheritance_gaps"))
    if posture == "fully_inherited" and has_gaps:
        findings.append(
            Finding(
                "inheritance_posture_mismatch",
                "fully_inherited posture must carry no disclosed gaps",
                surface_id=surface_id,
            )
        )
    if posture == "partial_inheritance_disclosed" and not has_gaps:
        findings.append(
            Finding(
                "inheritance_posture_mismatch",
                "partial_inheritance_disclosed posture must disclose at least one gap",
                surface_id=surface_id,
            )
        )

    evidence_state = surface.get("evidence_state")
    if evidence_state == "stale_evidence" and surface.get("marketed") is True:
        findings.append(
            Finding(
                "stale_evidence_on_marketed_surface",
                "marketed surface carries stale appearance evidence",
                surface_id=surface_id,
            )
        )
    if evidence_state == "disabled_package" and surface.get("provenance_disclosed") is not True:
        findings.append(
            Finding(
                "disabled_package_rendering_undisclosed",
                "disabled package renders without disclosure",
                surface_id=surface_id,
                package_id=active_package_id,
            )
        )

    for blocker in ensure_list(
        surface.get("blocking_findings", []), "surface.blocking_findings"
    ):
        findings.append(
            Finding(
                "blocking_finding_present",
                "surface carries a blocking finding",
                surface_id=surface_id,
                detail={"class": blocker.get("class")},
            )
        )


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
    for manifest in ensure_list(report.get("manifests", []), "report.manifests"):
        for ref_field in ("package_id", "package_revision_ref"):
            value = manifest.get(ref_field)
            if value not in case_set:
                findings.append(
                    Finding(
                        "support_missing_package_ref",
                        f"support_export.case_ids must quote every manifest {ref_field}",
                        package_id=manifest.get("package_id"),
                        detail={ref_field: value},
                    )
                )
    for surface in ensure_list(report.get("surfaces", []), "report.surfaces"):
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
    # The schemas are required to exist so the contract stays discoverable.
    for schema_rel in (SCHEMA_REL, CANONICAL_SCHEMA_REL):
        if not (repo_root / schema_rel).exists():
            raise SystemExit(f"missing required input: {schema_rel}")

    findings: list[Finding] = []
    check_report_envelope(report, findings)
    manifests = index_manifests(report)
    for manifest in ensure_list(report.get("manifests", []), "report.manifests"):
        check_manifest(ensure_dict(manifest, "manifest"), findings)
    for surface in ensure_list(report.get("surfaces", []), "report.surfaces"):
        check_surface(ensure_dict(surface, "surface"), manifests, findings)
    check_support_export(report, export, findings)
    check_publications(repo_root, findings)

    if args.format == "json":
        print(json.dumps({"findings": [f.as_dict() for f in findings]}, indent=2))
    else:
        if not findings:
            print("m5 theme-package manifest audit: clean")
        else:
            for finding in findings:
                location = finding.surface_id or finding.package_id or "report"
                print(f"FAIL [{finding.code}] {location}: {finding.message}")

    return 1 if findings else 0


if __name__ == "__main__":
    sys.exit(main())
