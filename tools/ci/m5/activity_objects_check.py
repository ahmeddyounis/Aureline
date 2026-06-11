#!/usr/bin/env python3
"""M5 durable activity-object qualification audit CI gate.

This gate enforces that the checked-in M5 durable activity-object audit stays
fresh and clean across the eight durable-attention guarantees the M5 depth
lanes must pass: activity_center_landing, exact_target_reopen,
reopen_after_focus_loss, reopen_after_restart, reopen_after_degraded_provider,
lifecycle_action_semantics, support_export_identity, and
companion_fanout_honesty. It reads:

- the audit fixture at ``fixtures/ux/m5/activity-center/report.json``;
- the support-export fixture at
  ``fixtures/ux/m5/activity-center/support_export.json``;
- the boundary schema at ``schemas/ux/m5-activity-object.schema.json``; and
- (when present) the published markdown at
  ``artifacts/ux/m5/durable-attention-packets/m5_activity_objects_audit.md``
  and the companion doc at ``docs/m5/durable-progress-and-reopen.md``.

For the audit the gate verifies that:

- the audit covers all eight required guarantees and at least one family
  qualifies each guarantee;
- every registered family has a binding for every required guarantee;
- every family carries a canonical exact-target reopen anchor, a non-empty
  support note, and ``registered_on_activity_center = true``;
- every qualified guarantee carries its required captured evidence (a durable
  attention packet, durable not toast-only truth, and an evidence-freshness
  stamp for every guarantee; a reopen outcome for the reopen guarantees; a
  survival outcome for the focus-loss, restart, and degraded-provider
  guarantees; differentiated action semantics for the lifecycle guarantee; a
  stable export identity for the support-export guarantee; an honest fanout
  label for the companion guarantee) and a present reopen outcome on every
  high-salience family;
- no qualified guarantee carries a red result (a lost reopen target,
  toast-only truth, a lost reopen after focus loss/restart/degraded provider,
  collapsed lifecycle actions, a reconstructed export identity, or a silent
  fanout failure);
- no family invents an ad-hoc parallel history model, no marketed guarantee is
  claimed with no evidence, and no marketed guarantee carries stale evidence;
- no family carries any blocking finding (so aspect, narrowing, and projection
  drift are all caught);
- the support-export wrapper quotes every family id and descriptor revision
  the audit exposes; and
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

REPORT_REL = Path("fixtures/ux/m5/activity-center/report.json")
SUPPORT_EXPORT_REL = Path("fixtures/ux/m5/activity-center/support_export.json")
COMPACT_REL = Path("fixtures/ux/m5/activity-center/compact.txt")
SCHEMA_REL = Path("schemas/ux/m5-activity-object.schema.json")
MARKDOWN_REL = Path(
    "artifacts/ux/m5/durable-attention-packets/m5_activity_objects_audit.md"
)
DOC_REL = Path("docs/m5/durable-progress-and-reopen.md")

REQUIRED_GUARANTEES = (
    "activity_center_landing",
    "exact_target_reopen",
    "reopen_after_focus_loss",
    "reopen_after_restart",
    "reopen_after_degraded_provider",
    "lifecycle_action_semantics",
    "support_export_identity",
    "companion_fanout_honesty",
)

REOPEN_GUARANTEES = (
    "exact_target_reopen",
    "reopen_after_focus_loss",
    "reopen_after_restart",
    "reopen_after_degraded_provider",
)

SURVIVAL_GUARANTEES = (
    "reopen_after_focus_loss",
    "reopen_after_restart",
    "reopen_after_degraded_provider",
)

EXPECTED_RECORD_KIND_REPORT = "shell_m5_activity_object_report_record"
EXPECTED_RECORD_KIND_ROW = "shell_m5_activity_object_row_record"
EXPECTED_RECORD_KIND_SUPPORT = "shell_m5_activity_object_support_export_record"
EXPECTED_SHARED_CONTRACT_REF = "shell:m5_activity_objects:v1"
EXPECTED_SCHEMA_VERSION = 1

HIGH_SALIENCE_CLASSES = {
    "lifecycle_bearing",
    "review_bearing",
    "risk_bearing",
}

DOC_BACKLINKS = (
    "artifacts/ux/m5/durable-attention-packets/m5_activity_objects_audit.md",
    "fixtures/ux/m5/activity-center/report.json",
    "schemas/ux/m5-activity-object.schema.json",
    "tools/ci/m5/activity_objects_check.py",
)


@dataclass
class Finding:
    """One blocking finding emitted by the gate."""

    code: str
    message: str
    family_id: str | None = None
    guarantee: str | None = None
    detail: dict[str, Any] = field(default_factory=dict)

    def as_dict(self) -> dict[str, Any]:
        out: dict[str, Any] = {"code": self.code, "message": self.message}
        if self.family_id is not None:
            out["family_id"] = self.family_id
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
        for family in rows:
            for binding in ensure_list(family.get("bindings", []), "family.bindings"):
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
                    "no qualified family for required guarantee",
                    guarantee=required,
                )
            )


def check_qualified_binding(
    family_id: str,
    high_salience: bool,
    binding: dict[str, Any],
    findings: list[Finding],
) -> None:
    guarantee = binding.get("guarantee")

    required_fields = [
        "projected_durable_packet_ref",
        "projected_toast_independence",
        "evidence_freshness",
    ]
    if guarantee in REOPEN_GUARANTEES:
        required_fields.append("projected_reopen_outcome")
    if guarantee in SURVIVAL_GUARANTEES:
        required_fields.append("projected_survival")
    if guarantee == "lifecycle_action_semantics":
        required_fields.append("projected_action_semantics")
    if guarantee == "support_export_identity":
        required_fields.append("projected_export_identity")
    if guarantee == "companion_fanout_honesty":
        required_fields.append("projected_fanout_honesty")
    if high_salience:
        required_fields.append("projected_reopen_outcome")
    for field_name in dict.fromkeys(required_fields):
        if binding.get(field_name) is None:
            findings.append(
                Finding(
                    "missing_projection",
                    "qualified guarantee is missing required captured evidence",
                    family_id=family_id,
                    guarantee=guarantee,
                    detail={"field": field_name},
                )
            )

    # Red captured results.
    if binding.get("projected_toast_independence") == "toast_only":
        findings.append(
            Finding("toast_only_truth", "guarantee relies on toast-only truth", family_id, guarantee)
        )
    if binding.get("projected_reopen_outcome") == "target_lost":
        findings.append(
            Finding("reopen_target_lost", "exact-target reopen affordance lost", family_id, guarantee)
        )
    if binding.get("projected_survival") == "lost":
        if guarantee == "reopen_after_focus_loss":
            code = "reopen_lost_after_focus_loss"
        elif guarantee == "reopen_after_restart":
            code = "reopen_lost_after_restart"
        else:
            code = "reopen_lost_under_degraded_provider"
        findings.append(Finding(code, "reopen target lost after event", family_id, guarantee))
    if binding.get("projected_action_semantics") == "collapsed":
        findings.append(
            Finding(
                "lifecycle_actions_collapsed",
                "reviewable work collapsed into one generic close action",
                family_id,
                guarantee,
            )
        )
    if binding.get("projected_export_identity") == "reconstructed":
        findings.append(
            Finding(
                "export_identity_reconstructed",
                "object identity reconstructed from logs",
                family_id,
                guarantee,
            )
        )
    if binding.get("projected_fanout_honesty") == "silent_failure":
        findings.append(
            Finding("fanout_failure_silent", "companion fanout failure hidden", family_id, guarantee)
        )
    if binding.get("marketed_on_guarantee") and binding.get("evidence_freshness") == "stale":
        findings.append(
            Finding(
                "stale_evidence_on_marketed_row",
                "marketed guarantee carries stale durable evidence",
                family_id,
                guarantee,
            )
        )


def check_family(family: dict[str, Any], findings: list[Finding]) -> None:
    descriptor = ensure_dict(family.get("descriptor", {}), "family.descriptor")
    family_id = descriptor.get("family_id")
    if not isinstance(family_id, str) or not family_id.strip():
        findings.append(Finding("missing_family_id", "descriptor.family_id must be non-empty"))
        return

    if family.get("record_kind") != EXPECTED_RECORD_KIND_ROW:
        findings.append(
            Finding(
                "family_record_kind_mismatch",
                f"family.record_kind must be {EXPECTED_RECORD_KIND_ROW}",
                family_id=family_id,
                detail={"record_kind": family.get("record_kind")},
            )
        )

    revision = descriptor.get("descriptor_revision_ref")
    if not isinstance(revision, str) or not revision.strip():
        findings.append(
            Finding(
                "missing_descriptor_revision_ref",
                "descriptor.descriptor_revision_ref must be non-empty",
                family_id=family_id,
            )
        )

    anchor = descriptor.get("reopen_anchor_ref")
    if not isinstance(anchor, str) or not anchor.strip():
        findings.append(
            Finding(
                "descriptor_missing_reopen_anchor",
                "descriptor.reopen_anchor_ref must be non-empty",
                family_id=family_id,
            )
        )

    note = descriptor.get("support_note")
    if not isinstance(note, str) or not note.strip():
        findings.append(
            Finding(
                "missing_support_note",
                "descriptor.support_note must be non-empty",
                family_id=family_id,
            )
        )

    if descriptor.get("registered_on_activity_center") is not True:
        findings.append(
            Finding(
                "family_not_on_activity_center",
                "descriptor.registered_on_activity_center must be true",
                family_id=family_id,
            )
        )

    high_salience = descriptor_high_salience(descriptor)

    if high_salience and not ensure_list(
        descriptor.get("supported_actions", []), "descriptor.supported_actions"
    ):
        findings.append(
            Finding(
                "missing_lifecycle_actions",
                "high-salience family must expose differentiated lifecycle actions",
                family_id=family_id,
            )
        )

    # Every required guarantee must be bound.
    bindings = ensure_list(family.get("bindings", []), "family.bindings")
    present = {binding.get("guarantee") for binding in bindings}
    for required in REQUIRED_GUARANTEES:
        if required not in present:
            findings.append(
                Finding(
                    "missing_required_guarantee",
                    "family is missing a required durable guarantee binding",
                    family_id=family_id,
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
                    family_id=family_id,
                    guarantee=guarantee,
                    detail={"aspect": aspect, "expected": expected_aspect},
                )
            )
        status = binding.get("qualification_status")
        if status == "unqualified_local_history":
            findings.append(
                Finding(
                    "unqualified_local_history",
                    "family tracks work in an ad-hoc parallel history model",
                    family_id=family_id,
                    guarantee=guarantee,
                )
            )
        elif status == "missing_evidence":
            findings.append(
                Finding(
                    "missing_evidence",
                    "marketed guarantee claimed with no captured evidence",
                    family_id=family_id,
                    guarantee=guarantee,
                )
            )
        elif status == "qualified":
            check_qualified_binding(family_id, high_salience, binding, findings)

    # Any blocking finding the Rust validator emitted is a gate failure.
    for blocker in ensure_list(
        family.get("blocking_findings", []), "family.blocking_findings"
    ):
        findings.append(
            Finding(
                "blocking_finding_present",
                "family carries a blocking finding",
                family_id=family_id,
                guarantee=blocker.get("guarantee"),
                detail={"class": blocker.get("class")},
            )
        )


def canonical_aspect(guarantee: Any) -> str | None:
    if guarantee == "activity_center_landing":
        return "landing"
    if guarantee in REOPEN_GUARANTEES:
        return "reopen"
    if guarantee == "lifecycle_action_semantics":
        return "lifecycle"
    if guarantee in ("support_export_identity", "companion_fanout_honesty"):
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
    for family in ensure_list(report.get("rows", []), "report.rows"):
        descriptor = ensure_dict(family.get("descriptor", {}), "family.descriptor")
        family_id = descriptor.get("family_id")
        revision = descriptor.get("descriptor_revision_ref")
        if family_id not in case_set:
            findings.append(
                Finding(
                    "support_missing_family_id",
                    "support_export.case_ids must quote every family id",
                    family_id=family_id,
                )
            )
        if revision not in case_set:
            findings.append(
                Finding(
                    "support_missing_descriptor_revision",
                    "support_export.case_ids must quote every descriptor revision",
                    family_id=family_id,
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
                    "companion doc must quote every required durable guarantee",
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
    for family in ensure_list(report.get("rows", []), "report.rows"):
        check_family(ensure_dict(family, "family"), findings)
    check_support_export(report, export, findings)
    check_publications(repo_root, findings)

    if args.format == "json":
        print(json.dumps({"findings": [f.as_dict() for f in findings]}, indent=2))
    else:
        if not findings:
            print("m5 activity-objects audit: clean")
        else:
            for finding in findings:
                location = finding.family_id or "report"
                if finding.guarantee:
                    location = f"{location} / {finding.guarantee}"
                print(f"FAIL [{finding.code}] {location}: {finding.message}")

    return 1 if findings else 0


if __name__ == "__main__":
    sys.exit(main())
