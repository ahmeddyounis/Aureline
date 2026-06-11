#!/usr/bin/env python3
"""M5 embedded-boundary owner/origin, auth, and handoff qualification CI gate.

This gate enforces that the checked-in M5 embedded-boundary audit stays fresh
and clean across the eight boundary guarantees the M5 depth lanes must pass:
owner_origin_disclosure, freshness_disclosure, trust_boundary_chrome,
system_browser_auth_default, no_embedded_high_risk_approval, return_anchor_present,
handoff_reason_preserved, and support_export_parity. It reads:

- the audit fixture at ``fixtures/ux/m5/webview-auth-handoff/report.json``;
- the support-export fixture at
  ``fixtures/ux/m5/webview-auth-handoff/support_export.json``;
- the boundary schema at
  ``schemas/help/m5-destination-descriptor-diff.schema.json``; and
- (when present) the published markdown at
  ``artifacts/ux/m5/embedded-boundary-audits/m5_embedded_boundaries_audit.md``
  and the companion doc at ``docs/m5/embedded-boundaries-and-auth.md``.

For the audit the gate verifies that:

- the audit covers all eight required guarantees and at least one surface
  qualifies each guarantee;
- every registered surface has a binding for every required guarantee;
- every surface carries a canonical return anchor, a non-empty support note, a
  declared boundary class, at least one declared handoff target, and
  ``routed_through_governed_boundary = true``;
- every qualified guarantee carries its required captured evidence (a
  destination-descriptor ref, a declared boundary class, an owner/origin
  disclosure, and an evidence-freshness stamp for every guarantee; a freshness
  disclosure for the freshness guarantee; a trust-chrome outcome for the trust
  guarantee; an auth-channel outcome for the system-browser guarantee; a
  high-risk-handling outcome for the no-embedded-approval guarantee; a
  return-anchor outcome for the return guarantee; a handoff-reason outcome for
  the handoff guarantee; a support-parity outcome for the support guarantee) and
  a present return-anchor outcome on every high-stakes surface;
- no qualified guarantee carries a red result (a hidden owner/origin, a hidden
  freshness stamp, a surface that pretends to be first-party, an embedded primary
  auth approval, a hidden high-risk embedded approval, a lost return anchor, a
  dropped handoff reason, or a divergent support clone);
- no surface invents a feature-local boundary rule, no marketed guarantee is
  claimed with no evidence, and no marketed guarantee carries stale evidence;
- no surface carries any blocking finding (so aspect, narrowing, and projection
  drift are all caught);
- the support-export wrapper quotes every surface id and descriptor revision the
  audit exposes; and
- the published markdown audit and the companion doc are present and back-link
  the canonical schema, fixtures, and CLI gate.

Exit codes:

- ``0`` -- audit is clean (all eight guarantees qualified, no blockers).
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

REPORT_REL = Path("fixtures/ux/m5/webview-auth-handoff/report.json")
SUPPORT_EXPORT_REL = Path("fixtures/ux/m5/webview-auth-handoff/support_export.json")
COMPACT_REL = Path("fixtures/ux/m5/webview-auth-handoff/compact.txt")
SCHEMA_REL = Path("schemas/help/m5-destination-descriptor-diff.schema.json")
MARKDOWN_REL = Path(
    "artifacts/ux/m5/embedded-boundary-audits/m5_embedded_boundaries_audit.md"
)
DOC_REL = Path("docs/m5/embedded-boundaries-and-auth.md")

REQUIRED_GUARANTEES = (
    "owner_origin_disclosure",
    "freshness_disclosure",
    "trust_boundary_chrome",
    "system_browser_auth_default",
    "no_embedded_high_risk_approval",
    "return_anchor_present",
    "handoff_reason_preserved",
    "support_export_parity",
)

EXPECTED_RECORD_KIND_REPORT = "shell_m5_embedded_boundary_report_record"
EXPECTED_RECORD_KIND_ROW = "shell_m5_embedded_boundary_row_record"
EXPECTED_RECORD_KIND_SUPPORT = "shell_m5_embedded_boundary_support_export_record"
EXPECTED_SHARED_CONTRACT_REF = "shell:m5_embedded_boundaries:v1"
EXPECTED_SCHEMA_VERSION = 1

HIGH_STAKES_CLASSES = {
    "provider_owned",
    "external_handoff",
}

DOC_BACKLINKS = (
    "artifacts/ux/m5/embedded-boundary-audits/m5_embedded_boundaries_audit.md",
    "fixtures/ux/m5/webview-auth-handoff/report.json",
    "schemas/help/m5-destination-descriptor-diff.schema.json",
    "tools/ci/m5/embedded_boundaries_check.py",
)


@dataclass
class Finding:
    """One blocking finding emitted by the gate."""

    code: str
    message: str
    surface_id: str | None = None
    guarantee: str | None = None
    detail: dict[str, Any] = field(default_factory=dict)

    def as_dict(self) -> dict[str, Any]:
        out: dict[str, Any] = {"code": self.code, "message": self.message}
        if self.surface_id is not None:
            out["surface_id"] = self.surface_id
        if self.guarantee is not None:
            out["guarantee"] = self.guarantee
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


def descriptor_high_stakes(descriptor: dict[str, Any]) -> bool:
    return descriptor.get("boundary_class") in HIGH_STAKES_CLASSES


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
    declared = report.get("required_guarantees")
    if declared != list(REQUIRED_GUARANTEES):
        findings.append(
            Finding(
                "required_guarantees_mismatch",
                "required_guarantees must equal the canonical guarantee list",
                detail={"required": list(REQUIRED_GUARANTEES), "declared": declared},
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


def check_required_guarantees_qualified(
    report: dict[str, Any], findings: list[Finding]
) -> None:
    rows = ensure_list(report.get("rows", []), "report.rows")
    for required in REQUIRED_GUARANTEES:
        any_qualified = False
        for surface in rows:
            for binding in ensure_list(surface.get("bindings", []), "surface.bindings"):
                if (
                    binding.get("guarantee") == required
                    and binding.get("qualification_status") == "qualified"
                ):
                    any_qualified = True
                    break
            if any_qualified:
                break
        if not any_qualified:
            findings.append(
                Finding(
                    "required_guarantee_not_qualified",
                    "no qualified surface for required guarantee",
                    guarantee=required,
                )
            )


def check_qualified_binding(
    surface_id: str,
    high_stakes: bool,
    binding: dict[str, Any],
    findings: list[Finding],
) -> None:
    guarantee = binding.get("guarantee")

    required_fields = [
        "projected_descriptor_ref",
        "projected_boundary_class",
        "projected_owner_origin",
        "evidence_freshness",
    ]
    if guarantee == "freshness_disclosure":
        required_fields.append("projected_freshness")
    if guarantee == "trust_boundary_chrome":
        required_fields.append("projected_trust_chrome")
    if guarantee == "system_browser_auth_default":
        required_fields.append("projected_auth_channel")
    if guarantee == "no_embedded_high_risk_approval":
        required_fields.append("projected_high_risk_handling")
    if guarantee == "return_anchor_present":
        required_fields.append("projected_return_anchor")
    if guarantee == "handoff_reason_preserved":
        required_fields.append("projected_handoff_reason")
    if guarantee == "support_export_parity":
        required_fields.append("projected_support_parity")
    if high_stakes:
        required_fields.append("projected_return_anchor")
    for field_name in dict.fromkeys(required_fields):
        if binding.get(field_name) is None:
            findings.append(
                Finding(
                    "missing_projection",
                    "qualified guarantee is missing required captured evidence",
                    surface_id=surface_id,
                    guarantee=guarantee,
                    detail={"field": field_name},
                )
            )

    # Red captured results.
    if binding.get("projected_owner_origin") == "owner_origin_hidden":
        findings.append(
            Finding("owner_origin_hidden", "owner/origin chrome hidden", surface_id, guarantee)
        )
    if binding.get("projected_freshness") == "freshness_hidden":
        findings.append(
            Finding("freshness_hidden", "content freshness hidden", surface_id, guarantee)
        )
    if binding.get("projected_trust_chrome") == "pretends_first_party":
        findings.append(
            Finding(
                "pretends_first_party",
                "surface pretends to be a first-party local surface",
                surface_id,
                guarantee,
            )
        )
    if binding.get("projected_auth_channel") == "embedded_primary_approval":
        findings.append(
            Finding(
                "embedded_primary_auth",
                "embedded pane is the primary auth approval channel",
                surface_id,
                guarantee,
            )
        )
    if binding.get("projected_high_risk_handling") == "embedded_approval_hidden":
        findings.append(
            Finding(
                "embedded_high_risk_approval",
                "high-risk approval hidden inside the embedded pane",
                surface_id,
                guarantee,
            )
        )
    if binding.get("projected_return_anchor") == "return_lost":
        findings.append(
            Finding("return_anchor_lost", "return anchor lost", surface_id, guarantee)
        )
    if binding.get("projected_handoff_reason") == "reason_dropped":
        findings.append(
            Finding(
                "handoff_reason_dropped",
                "handoff happened with no preserved reason",
                surface_id,
                guarantee,
            )
        )
    if binding.get("projected_support_parity") == "divergent_clone":
        findings.append(
            Finding(
                "support_parity_divergent",
                "support surface clones divergent text instead of reusing the descriptor",
                surface_id,
                guarantee,
            )
        )
    if binding.get("marketed_on_guarantee") and binding.get("evidence_freshness") == "stale":
        findings.append(
            Finding(
                "stale_evidence_on_marketed_row",
                "marketed guarantee carries stale evidence",
                surface_id,
                guarantee,
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

    anchor = descriptor.get("return_anchor_ref")
    if not isinstance(anchor, str) or not anchor.strip():
        findings.append(
            Finding(
                "descriptor_missing_return_anchor",
                "descriptor.return_anchor_ref must be non-empty",
                surface_id=surface_id,
            )
        )

    note = descriptor.get("support_note")
    if not isinstance(note, str) or not note.strip():
        findings.append(
            Finding(
                "missing_support_note",
                "descriptor.support_note must be non-empty",
                surface_id=surface_id,
            )
        )

    if not isinstance(descriptor.get("boundary_class"), str) or not descriptor.get(
        "boundary_class"
    ):
        findings.append(
            Finding(
                "missing_boundary_class",
                "descriptor.boundary_class must be declared",
                surface_id=surface_id,
            )
        )

    if descriptor.get("routed_through_governed_boundary") is not True:
        findings.append(
            Finding(
                "surface_not_on_governed_boundary",
                "descriptor.routed_through_governed_boundary must be true",
                surface_id=surface_id,
            )
        )

    high_stakes = descriptor_high_stakes(descriptor)

    if high_stakes and not ensure_list(
        descriptor.get("boundary_chrome", []), "descriptor.boundary_chrome"
    ):
        findings.append(
            Finding(
                "missing_boundary_chrome",
                "high-stakes surface must expose boundary chrome",
                surface_id=surface_id,
            )
        )

    if descriptor.get("marketed_on_desktop") and not ensure_list(
        descriptor.get("handoff_targets", []), "descriptor.handoff_targets"
    ):
        findings.append(
            Finding(
                "no_declared_handoff_target",
                "marketed surface must declare a handoff target",
                surface_id=surface_id,
            )
        )

    # Every required guarantee must be bound.
    bindings = ensure_list(surface.get("bindings", []), "surface.bindings")
    present = {binding.get("guarantee") for binding in bindings}
    for required in REQUIRED_GUARANTEES:
        if required not in present:
            findings.append(
                Finding(
                    "missing_required_guarantee",
                    "surface is missing a required boundary guarantee binding",
                    surface_id=surface_id,
                    guarantee=required,
                )
            )

    for binding in bindings:
        guarantee = binding.get("guarantee")
        aspect = binding.get("aspect")
        expected_aspect = canonical_aspect(guarantee)
        if expected_aspect is not None and aspect != expected_aspect:
            findings.append(
                Finding(
                    "aspect_drift",
                    "binding aspect disagrees with its guarantee's canonical aspect",
                    surface_id=surface_id,
                    guarantee=guarantee,
                    detail={"aspect": aspect, "expected": expected_aspect},
                )
            )
        status = binding.get("qualification_status")
        if status == "unqualified_local_surface":
            findings.append(
                Finding(
                    "unqualified_local_surface",
                    "surface paints its own boundary chrome outside the governed model",
                    surface_id=surface_id,
                    guarantee=guarantee,
                )
            )
        elif status == "missing_evidence":
            findings.append(
                Finding(
                    "missing_evidence",
                    "marketed guarantee claimed with no captured evidence",
                    surface_id=surface_id,
                    guarantee=guarantee,
                )
            )
        elif status == "qualified":
            check_qualified_binding(surface_id, high_stakes, binding, findings)

    # Any blocking finding the Rust validator emitted is a gate failure.
    for blocker in ensure_list(
        surface.get("blocking_findings", []), "surface.blocking_findings"
    ):
        findings.append(
            Finding(
                "blocking_finding_present",
                "surface carries a blocking finding",
                surface_id=surface_id,
                guarantee=blocker.get("guarantee"),
                detail={"class": blocker.get("class")},
            )
        )


def canonical_aspect(guarantee: Any) -> str | None:
    if guarantee in (
        "owner_origin_disclosure",
        "freshness_disclosure",
        "trust_boundary_chrome",
    ):
        return "attribution"
    if guarantee in ("system_browser_auth_default", "no_embedded_high_risk_approval"):
        return "auth"
    if guarantee in ("return_anchor_present", "handoff_reason_preserved"):
        return "handoff"
    if guarantee == "support_export_parity":
        return "export"
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
    for guarantee in REQUIRED_GUARANTEES:
        if guarantee not in body:
            findings.append(
                Finding(
                    "doc_missing_guarantee",
                    "companion doc must quote every required boundary guarantee",
                    guarantee=guarantee,
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
    check_required_guarantees_qualified(report, findings)
    for surface in ensure_list(report.get("rows", []), "report.rows"):
        check_surface(ensure_dict(surface, "surface"), findings)
    check_support_export(report, export, findings)
    check_publications(repo_root, findings)

    if args.format == "json":
        print(json.dumps({"findings": [f.as_dict() for f in findings]}, indent=2))
    else:
        if not findings:
            print("m5 embedded-boundaries audit: clean")
        else:
            for finding in findings:
                location = finding.surface_id or "report"
                if finding.guarantee:
                    location = f"{location} / {finding.guarantee}"
                print(f"FAIL [{finding.code}] {location}: {finding.message}")

    return 1 if findings else 0


if __name__ == "__main__":
    sys.exit(main())
