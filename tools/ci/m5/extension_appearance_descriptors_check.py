#!/usr/bin/env python3
"""M5 extension appearance-inheritance descriptor gate.

This gate enforces that the checked-in extension appearance-descriptor audit
stays fresh, honest, and clean: every extension-backed or embedded surface
discloses whether it inherits Aureline's theme, high-contrast, density, and
focus-token semantics; the visible inheritance badge is rendered in extension
details, embedded panes, diagnostics, and support/export packets; no surface
overclaims first-party parity it cannot back; no axis hides behind an
undisclosed posture; and the extension-detail, embedded-pane, diagnostics, and
support/export surfaces share the same audit object. It reads:

- the audit fixture at
  ``fixtures/ux/m5/extension-theme-inheritance/audit.json``;
- the support-export fixture at
  ``fixtures/ux/m5/extension-theme-inheritance/support_export.json``;
- the boundary schema at
  ``schemas/ux/extension-appearance-descriptor.schema.json``; and
- the published report at
  ``artifacts/ux/m5/extension-appearance-audit/extension_appearance_audit.md``
  and the companion doc at ``docs/m5/extension-appearance-inheritance.md``.

For the audit the gate verifies that:

- the record envelope (record kind, schema version, shared contract ref, schema
  ref) is correct and the schema file exists on disk;
- the summary is recomputed from the descriptors and matches;
- every descriptor carries a host id, a package id, the four governed posture
  axes, a derived badge consistent with those axes, and renders the badge on
  every required surface;
- no descriptor leaves an axis undisclosed, and no full-inheritance badge hides a
  disclosed gap;
- a granted first-party-parity decision is backed by full inheritance, zero
  gaps, and at least one accessibility-evidence ref, and a blocked parity claim
  is recorded as a defect;
- the audit declares no defects (a clean audit);
- the support-export wrapper quotes the audit id and every descriptor id, asserts
  raw appearance material is excluded, and its summary matches the audit; and
- the published report and companion doc exist and the doc back-links the
  canonical schema, fixtures, artifact, and gate.

Exit codes:

- ``0`` -- audit is clean.
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

AUDIT_REL = Path("fixtures/ux/m5/extension-theme-inheritance/audit.json")
SUPPORT_EXPORT_REL = Path("fixtures/ux/m5/extension-theme-inheritance/support_export.json")
SCHEMA_REL = Path("schemas/ux/extension-appearance-descriptor.schema.json")
MARKDOWN_REL = Path("artifacts/ux/m5/extension-appearance-audit/extension_appearance_audit.md")
DOC_REL = Path("docs/m5/extension-appearance-inheritance.md")

EXPECTED_RECORD_KIND_AUDIT = "extension_appearance_audit_record"
EXPECTED_RECORD_KIND_DESCRIPTOR = "extension_appearance_descriptor_record"
EXPECTED_RECORD_KIND_SUPPORT = "extension_appearance_support_export_record"
EXPECTED_SHARED_CONTRACT_REF = "extensions:m5_appearance_descriptor:v1"
EXPECTED_SCHEMA_VERSION = 1

AXES = ("theme", "focus", "contrast", "density", "reduced_motion")
POSTURES = ("inherits", "partial", "does_not_inherit", "not_disclosed")
RENDERED_SURFACES = ("extension_detail", "embedded_pane", "diagnostics", "support_export")
BADGES = ("full_inheritance", "partial_inheritance", "does_not_inherit", "undisclosed")
PARITY_STATES = (
    "no_parity_claim",
    "claims_host_parity",
    "partial_claim_with_gaps",
    "denied_claim",
)

DOC_BACKLINKS = (
    "artifacts/ux/m5/extension-appearance-audit/extension_appearance_audit.md",
    "fixtures/ux/m5/extension-theme-inheritance/audit.json",
    "schemas/ux/extension-appearance-descriptor.schema.json",
    "tools/ci/m5/extension_appearance_descriptors_check.py",
)


@dataclass
class Finding:
    """One blocking finding emitted by the gate."""

    code: str
    message: str
    descriptor_id: str | None = None
    detail: dict[str, Any] = field(default_factory=dict)

    def as_dict(self) -> dict[str, Any]:
        out: dict[str, Any] = {"code": self.code, "message": self.message}
        if self.descriptor_id is not None:
            out["descriptor_id"] = self.descriptor_id
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


def derive_badge(postures: list[str]) -> str:
    if any(p == "not_disclosed" for p in postures):
        return "undisclosed"
    if all(p == "inherits" for p in postures):
        return "full_inheritance"
    if all(p == "does_not_inherit" for p in postures):
        return "does_not_inherit"
    return "partial_inheritance"


def check_envelope(repo_root: Path, audit: dict[str, Any], findings: list[Finding]) -> None:
    if audit.get("record_kind") != EXPECTED_RECORD_KIND_AUDIT:
        findings.append(
            Finding(
                "audit_record_kind_mismatch",
                f"audit.record_kind must be {EXPECTED_RECORD_KIND_AUDIT}",
                detail={"record_kind": audit.get("record_kind")},
            )
        )
    if audit.get("schema_version") != EXPECTED_SCHEMA_VERSION:
        findings.append(
            Finding(
                "audit_schema_version_mismatch",
                f"audit.schema_version must be {EXPECTED_SCHEMA_VERSION}",
                detail={"schema_version": audit.get("schema_version")},
            )
        )
    if audit.get("shared_contract_ref") != EXPECTED_SHARED_CONTRACT_REF:
        findings.append(
            Finding(
                "audit_shared_contract_ref_mismatch",
                f"audit.shared_contract_ref must be {EXPECTED_SHARED_CONTRACT_REF}",
                detail={"shared_contract_ref": audit.get("shared_contract_ref")},
            )
        )
    schema_ref = audit.get("schema_ref")
    if not isinstance(schema_ref, str) or not (repo_root / schema_ref).exists():
        findings.append(
            Finding(
                "schema_ref_missing",
                "audit.schema_ref must point at an existing schema",
                detail={"schema_ref": schema_ref},
            )
        )
    rendered = audit.get("rendered_surface_tokens")
    if list(rendered or []) != list(RENDERED_SURFACES):
        findings.append(
            Finding(
                "rendered_surface_tokens_mismatch",
                "audit.rendered_surface_tokens must list every required surface",
                detail={"declared": rendered, "expected": list(RENDERED_SURFACES)},
            )
        )


def check_summary(audit: dict[str, Any], descriptors: list[dict[str, Any]], findings: list[Finding]) -> None:
    counts = {
        "descriptor_count": len(descriptors),
        "full_inheritance_count": 0,
        "partial_inheritance_count": 0,
        "does_not_inherit_count": 0,
        "undisclosed_count": 0,
        "host_parity_claim_count": 0,
        "partial_parity_claim_count": 0,
        "denied_parity_claim_count": 0,
        "defect_count": 0,
    }
    badge_key = {
        "full_inheritance": "full_inheritance_count",
        "partial_inheritance": "partial_inheritance_count",
        "does_not_inherit": "does_not_inherit_count",
        "undisclosed": "undisclosed_count",
    }
    parity_key = {
        "claims_host_parity": "host_parity_claim_count",
        "partial_claim_with_gaps": "partial_parity_claim_count",
        "denied_claim": "denied_parity_claim_count",
    }
    for descriptor in descriptors:
        badge = descriptor.get("badge", {}).get("badge_class")
        if badge in badge_key:
            counts[badge_key[badge]] += 1
        state = descriptor.get("parity_claim_state")
        if state in parity_key:
            counts[parity_key[state]] += 1
        counts["defect_count"] += len(descriptor.get("defect_kind_tokens", []))

    if audit.get("summary") != counts:
        findings.append(
            Finding(
                "summary_stale",
                "audit.summary does not match the descriptors",
                detail={"expected": counts, "declared": audit.get("summary")},
            )
        )


def check_descriptor(descriptor: dict[str, Any], findings: list[Finding]) -> None:
    descriptor_id = descriptor.get("descriptor_id") or "<unknown>"

    if descriptor.get("record_kind") != EXPECTED_RECORD_KIND_DESCRIPTOR:
        findings.append(
            Finding(
                "descriptor_record_kind_mismatch",
                f"descriptor.record_kind must be {EXPECTED_RECORD_KIND_DESCRIPTOR}",
                descriptor_id=descriptor_id,
                detail={"record_kind": descriptor.get("record_kind")},
            )
        )

    for field_name in ("host_id", "package_id"):
        if not str(descriptor.get(field_name, "")).strip():
            findings.append(
                Finding(
                    "descriptor_provenance_missing",
                    "descriptor must carry a host id and a package id",
                    descriptor_id=descriptor_id,
                    detail={"field": field_name},
                )
            )

    axes = descriptor.get("axes", [])
    axis_by_name = {a.get("axis"): a.get("posture") for a in axes if isinstance(a, dict)}
    if sorted(axis_by_name) != sorted(AXES):
        findings.append(
            Finding(
                "axis_coverage_incomplete",
                "descriptor must declare exactly the four governed axes",
                descriptor_id=descriptor_id,
                detail={"declared": sorted(axis_by_name), "expected": sorted(AXES)},
            )
        )
        return

    postures = [axis_by_name[axis] for axis in AXES]
    for axis, posture in zip(AXES, postures):
        if posture not in POSTURES:
            findings.append(
                Finding(
                    "posture_token_invalid",
                    f"axis {axis} carries an unknown posture",
                    descriptor_id=descriptor_id,
                    detail={"axis": axis, "posture": posture},
                )
            )
        if posture == "not_disclosed":
            findings.append(
                Finding(
                    "axis_undisclosed",
                    f"axis {axis} is undisclosed; inheritance cannot be inspected",
                    descriptor_id=descriptor_id,
                    detail={"axis": axis},
                )
            )

    badge = descriptor.get("badge", {})
    badge_class = badge.get("badge_class")
    expected_badge = derive_badge(postures)
    if badge_class != expected_badge:
        findings.append(
            Finding(
                "badge_mismatch",
                "descriptor badge does not match its axis postures",
                descriptor_id=descriptor_id,
                detail={"declared": badge_class, "expected": expected_badge},
            )
        )
    if badge.get("implies_host_parity") != (badge_class == "full_inheritance"):
        findings.append(
            Finding(
                "badge_parity_flag_inconsistent",
                "badge.implies_host_parity must be true only for full inheritance",
                descriptor_id=descriptor_id,
                detail={"badge_class": badge_class, "implies": badge.get("implies_host_parity")},
            )
        )

    gaps = descriptor.get("known_gaps", [])
    if badge_class == "full_inheritance" and gaps:
        findings.append(
            Finding(
                "hidden_inheritance_gap",
                "a full-inheritance badge cannot disclose appearance gaps",
                descriptor_id=descriptor_id,
                detail={"gap_count": len(gaps)},
            )
        )

    rendered = descriptor.get("rendered_on_surfaces", [])
    for surface in RENDERED_SURFACES:
        if surface not in rendered:
            findings.append(
                Finding(
                    "badge_not_rendered",
                    f"inheritance badge is not rendered on the {surface} surface",
                    descriptor_id=descriptor_id,
                    detail={"surface": surface},
                )
            )
    if descriptor.get("host_rendered_appearance_badge") is not True:
        findings.append(
            Finding(
                "host_badge_chrome_hidden",
                "the host appearance badge must never be suppressed",
                descriptor_id=descriptor_id,
            )
        )

    state = descriptor.get("parity_claim_state")
    if state not in PARITY_STATES:
        findings.append(
            Finding(
                "parity_state_invalid",
                "descriptor carries an unknown parity-claim state",
                descriptor_id=descriptor_id,
                detail={"parity_claim_state": state},
            )
        )
    claims = bool(descriptor.get("claims_first_party_parity"))
    has_evidence = bool(descriptor.get("accessibility_evidence_refs"))
    if state == "claims_host_parity":
        if not (claims and badge_class == "full_inheritance" and not gaps and has_evidence):
            findings.append(
                Finding(
                    "parity_overclaimed",
                    "a host-parity claim is not backed by full inheritance, zero gaps, and accessibility evidence",
                    descriptor_id=descriptor_id,
                    detail={
                        "claims": claims,
                        "badge_class": badge_class,
                        "gap_count": len(gaps),
                        "has_accessibility_evidence": has_evidence,
                    },
                )
            )
    if state == "denied_claim":
        if "overclaimed_parity" not in descriptor.get("defect_kind_tokens", []):
            findings.append(
                Finding(
                    "denied_parity_not_flagged",
                    "a denied parity claim must carry an overclaimed_parity defect token",
                    descriptor_id=descriptor_id,
                    detail={"parity_claim_state": state},
                )
            )


def check_clean(audit: dict[str, Any], descriptors: list[dict[str, Any]], findings: list[Finding]) -> None:
    if audit.get("defects"):
        findings.append(
            Finding(
                "audit_not_clean",
                "the published audit must carry no defects",
                detail={"defect_count": len(audit.get("defects", []))},
            )
        )
    for descriptor in descriptors:
        tokens = descriptor.get("defect_kind_tokens", [])
        if tokens:
            findings.append(
                Finding(
                    "descriptor_not_clean",
                    "the published descriptor must carry no defect tokens",
                    descriptor_id=descriptor.get("descriptor_id"),
                    detail={"defect_kind_tokens": tokens},
                )
            )


def check_support_export(audit: dict[str, Any], export: dict[str, Any], findings: list[Finding]) -> None:
    if export.get("record_kind") != EXPECTED_RECORD_KIND_SUPPORT:
        findings.append(
            Finding(
                "support_record_kind_mismatch",
                f"support_export.record_kind must be {EXPECTED_RECORD_KIND_SUPPORT}",
                detail={"record_kind": export.get("record_kind")},
            )
        )
    if export.get("audit_ref") != audit.get("audit_id"):
        findings.append(Finding("support_audit_ref_mismatch", "support_export must quote the audit id"))
    if export.get("summary") != audit.get("summary"):
        findings.append(Finding("support_summary_stale", "support_export.summary drifted from the audit"))
    if export.get("raw_appearance_material_excluded") is not True:
        findings.append(
            Finding(
                "support_raw_material_not_excluded",
                "support_export must assert raw appearance material is excluded",
            )
        )
    case_ids = export.get("case_ids")
    if not isinstance(case_ids, list):
        findings.append(Finding("support_case_ids_missing", "support_export.case_ids must be an array"))
        return
    case_set = set(case_ids)
    if audit.get("audit_id") not in case_set:
        findings.append(Finding("support_missing_audit_id", "case_ids must quote the audit id"))
    for descriptor in ensure_list(audit.get("descriptors", []), "audit.descriptors"):
        descriptor_id = descriptor.get("descriptor_id")
        if descriptor_id not in case_set:
            findings.append(
                Finding(
                    "support_missing_descriptor_id",
                    "case_ids must quote every descriptor id",
                    descriptor_id=descriptor_id,
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

    audit = ensure_dict(load_json(repo_root / AUDIT_REL), "audit")
    export = ensure_dict(load_json(repo_root / SUPPORT_EXPORT_REL), "support_export")
    if not (repo_root / SCHEMA_REL).exists():
        raise SystemExit(f"missing required input: {SCHEMA_REL}")

    descriptors = ensure_list(audit.get("descriptors", []), "audit.descriptors")

    findings: list[Finding] = []
    check_envelope(repo_root, audit, findings)
    if not descriptors:
        findings.append(Finding("descriptors_empty", "audit.descriptors must be non-empty"))
    check_summary(audit, descriptors, findings)
    for descriptor in descriptors:
        check_descriptor(ensure_dict(descriptor, "descriptor"), findings)
    check_clean(audit, descriptors, findings)
    check_support_export(audit, export, findings)
    check_publications(repo_root, findings)

    if args.format == "json":
        print(json.dumps({"findings": [f.as_dict() for f in findings]}, indent=2))
    else:
        if not findings:
            print("m5 extension appearance descriptors: clean")
        else:
            for finding in findings:
                location = finding.descriptor_id or "audit"
                print(f"FAIL [{finding.code}] {location}: {finding.message}")

    return 1 if findings else 0


if __name__ == "__main__":
    sys.exit(main())
