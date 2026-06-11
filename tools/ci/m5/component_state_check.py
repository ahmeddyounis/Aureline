#!/usr/bin/env python3
"""M5 component-state and design-token inheritance audit CI gate.

This gate enforces that the checked-in M5 component-state audit stays
fresh and clean across the nine normalized component states the M5 depth
lanes introduce: loading, cached, stale, partial, policy_blocked,
degraded, preview_only, sync_pending, and boundary_handoff. It reads:

- the audit fixture at ``fixtures/ux/m5/theme-token-consumers/report.json``;
- the support-export fixture at
  ``fixtures/ux/m5/theme-token-consumers/support_export.json``;
- the boundary schema at ``schemas/ux/m5-component-state.schema.json``; and
- (when present) the published markdown at
  ``artifacts/ux/m5/component-state-audit/m5_component_state_audit.md``
  and the companion doc at ``docs/m5/component-state-parity.md``.

For the audit the gate verifies that:

- the audit covers all nine required states and at least one surface
  inherits each state;
- every registered surface has a binding for every required state;
- every surface carries a canonical registry anchor, a non-empty
  accessibility note, and ``registered_in_shared_registry = true``;
- every high-salience surface (one that conveys lifecycle, trust, or
  severity meaning) requires a non-color cue policy and has no
  ``unknown_token_gap`` binding;
- no binding paints a hard-coded theme value or an unresolved token
  fallback, and no surface renders a state through ad-hoc local semantics;
- no row carries any blocking finding (so token, provenance, cue-policy,
  override, and registry-anchor drift are all caught);
- the support-export wrapper quotes every surface id and descriptor
  revision the audit exposes; and
- the published markdown audit and the companion doc are present and
  back-link the canonical schema, fixtures, and CLI gate.

Exit codes:

- ``0`` -- audit is clean (all nine states inherited, no blockers).
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

REPORT_REL = Path("fixtures/ux/m5/theme-token-consumers/report.json")
SUPPORT_EXPORT_REL = Path("fixtures/ux/m5/theme-token-consumers/support_export.json")
COMPACT_REL = Path("fixtures/ux/m5/theme-token-consumers/compact.txt")
SCHEMA_REL = Path("schemas/ux/m5-component-state.schema.json")
MARKDOWN_REL = Path("artifacts/ux/m5/component-state-audit/m5_component_state_audit.md")
DOC_REL = Path("docs/m5/component-state-parity.md")

REQUIRED_STATES = (
    "loading",
    "cached",
    "stale",
    "partial",
    "policy_blocked",
    "degraded",
    "preview_only",
    "sync_pending",
    "boundary_handoff",
)

EXPECTED_RECORD_KIND_REPORT = "shell_m5_component_state_audit_report_record"
EXPECTED_RECORD_KIND_ROW = "shell_m5_component_state_row_record"
EXPECTED_RECORD_KIND_SUPPORT = "shell_m5_component_state_support_export_record"
EXPECTED_SHARED_CONTRACT_REF = "shell:m5_component_state:v1"
EXPECTED_SCHEMA_VERSION = 1

HIGH_SALIENCE_CLASSES = {
    "lifecycle_bearing",
    "trust_bearing",
    "severity_bearing",
}

DOC_BACKLINKS = (
    "artifacts/ux/m5/component-state-audit/m5_component_state_audit.md",
    "fixtures/ux/m5/theme-token-consumers/report.json",
    "schemas/ux/m5-component-state.schema.json",
    "tools/ci/m5/component_state_check.py",
)


@dataclass
class Finding:
    """One blocking finding emitted by the gate."""

    code: str
    message: str
    surface_id: str | None = None
    state: str | None = None
    detail: dict[str, Any] = field(default_factory=dict)

    def as_dict(self) -> dict[str, Any]:
        out: dict[str, Any] = {"code": self.code, "message": self.message}
        if self.surface_id is not None:
            out["surface_id"] = self.surface_id
        if self.state is not None:
            out["state"] = self.state
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
    declared = report.get("required_states")
    if declared != list(REQUIRED_STATES):
        findings.append(
            Finding(
                "required_states_mismatch",
                "required_states must equal the canonical state list",
                detail={"required": list(REQUIRED_STATES), "declared": declared},
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


def check_required_states_inherited(report: dict[str, Any], findings: list[Finding]) -> None:
    rows = ensure_list(report.get("rows", []), "report.rows")
    for required in REQUIRED_STATES:
        any_inherited = False
        for row in rows:
            for binding in ensure_list(row.get("bindings", []), "row.bindings"):
                if (
                    binding.get("state") == required
                    and binding.get("binding_status") == "inherited"
                ):
                    any_inherited = True
                    break
            if any_inherited:
                break
        if not any_inherited:
            findings.append(
                Finding(
                    "required_state_not_inherited",
                    "no inherited row for required state",
                    state=required,
                )
            )


def check_row(row: dict[str, Any], findings: list[Finding]) -> None:
    descriptor = ensure_dict(row.get("descriptor", {}), "row.descriptor")
    surface_id = descriptor.get("surface_id")
    if not isinstance(surface_id, str) or not surface_id.strip():
        findings.append(Finding("missing_surface_id", "descriptor.surface_id must be non-empty"))
        return

    if row.get("record_kind") != EXPECTED_RECORD_KIND_ROW:
        findings.append(
            Finding(
                "row_record_kind_mismatch",
                f"row.record_kind must be {EXPECTED_RECORD_KIND_ROW}",
                surface_id=surface_id,
                detail={"record_kind": row.get("record_kind")},
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

    anchor = descriptor.get("registry_anchor_ref")
    if not isinstance(anchor, str) or not anchor.strip():
        findings.append(
            Finding(
                "descriptor_missing_registry_anchor",
                "descriptor.registry_anchor_ref must be non-empty",
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

    if descriptor.get("registered_in_shared_registry") is not True:
        findings.append(
            Finding(
                "surface_not_registered",
                "descriptor.registered_in_shared_registry must be true",
                surface_id=surface_id,
            )
        )

    high_salience = descriptor_high_salience(descriptor)
    if high_salience and descriptor.get("cue_policy") != "non_color_cue_required":
        findings.append(
            Finding(
                "missing_non_color_cue_policy",
                "high-salience surface must require a non-color cue policy",
                surface_id=surface_id,
            )
        )

    # Every required state must be bound.
    bindings = ensure_list(row.get("bindings", []), "row.bindings")
    present = {binding.get("state") for binding in bindings}
    for required in REQUIRED_STATES:
        if required not in present:
            findings.append(
                Finding(
                    "missing_required_state",
                    "surface is missing a required state binding",
                    surface_id=surface_id,
                    state=required,
                )
            )

    for binding in bindings:
        state = binding.get("state")
        status = binding.get("binding_status")
        if binding.get("hardcoded_value"):
            findings.append(
                Finding(
                    "hardcoded_theme_value",
                    "binding paints a hard-coded theme value",
                    surface_id=surface_id,
                    state=state,
                )
            )
        if binding.get("unresolved_token_fallback"):
            findings.append(
                Finding(
                    "unresolved_token_fallback",
                    "binding falls back to an unresolved token fallback",
                    surface_id=surface_id,
                    state=state,
                )
            )
        if status == "unregistered_local_state":
            findings.append(
                Finding(
                    "unregistered_local_state",
                    "surface renders a state through ad-hoc local semantics",
                    surface_id=surface_id,
                    state=state,
                )
            )
        if status == "unknown_token_gap" and high_salience:
            findings.append(
                Finding(
                    "unknown_token_gap",
                    "high-salience surface has an unknown token binding for a required state",
                    surface_id=surface_id,
                    state=state,
                )
            )

    # Any blocking finding the Rust validator emitted is a gate failure.
    for blocker in ensure_list(row.get("blocking_findings", []), "row.blocking_findings"):
        findings.append(
            Finding(
                "blocking_finding_present",
                "row carries a blocking finding",
                surface_id=surface_id,
                state=blocker.get("state"),
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
    for row in ensure_list(report.get("rows", []), "report.rows"):
        descriptor = ensure_dict(row.get("descriptor", {}), "row.descriptor")
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
    for state in REQUIRED_STATES:
        if state not in body:
            findings.append(
                Finding(
                    "doc_missing_state",
                    "companion doc must quote every required state",
                    state=state,
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
    check_required_states_inherited(report, findings)
    for row in ensure_list(report.get("rows", []), "report.rows"):
        check_row(ensure_dict(row, "row"), findings)
    check_support_export(report, export, findings)
    check_publications(repo_root, findings)

    if args.format == "json":
        print(json.dumps({"findings": [f.as_dict() for f in findings]}, indent=2))
    else:
        if not findings:
            print("m5 component-state audit: clean")
        else:
            for finding in findings:
                location = finding.surface_id or "report"
                if finding.state:
                    location = f"{location} / {finding.state}"
                print(f"FAIL [{finding.code}] {location}: {finding.message}")

    return 1 if findings else 0


if __name__ == "__main__":
    sys.exit(main())
