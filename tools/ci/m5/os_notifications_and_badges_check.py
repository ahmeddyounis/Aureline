#!/usr/bin/env python3
"""M5 OS notification, badge, progress, and reopen parity CI gate.

This gate enforces that the checked-in M5 OS-attention audit stays fresh and
clean across the five OS-attention parity guarantees the M5 durable job families
must pass: privacy_safe_summary, badge_durable_class, progress_named_job_class,
suppression_parity, and exact_reopen_parity. It reads:

- the audit fixture at ``fixtures/ux/m5_os_notifications_and_badges/report.json``;
- the support-export fixture at
  ``fixtures/ux/m5_os_notifications_and_badges/support_export.json``;
- the boundary schema at
  ``schemas/ux/m5-os-notification-envelope.schema.json``; and
- (when present) the published markdown at
  ``artifacts/ux/m5/os-notification-and-reopen.md`` and the companion doc at
  ``docs/m5/os-notifications-badges-and-progress.md``.

For the audit the gate verifies that:

- the audit covers all five required guarantees and at least one surface
  qualifies each guarantee;
- every registered surface has a binding for every required guarantee;
- every surface carries a canonical exact-target reopen anchor, a durable job
  ref, a non-empty support note, a declared privacy class, a source-object
  label, a safe reopen action label, ``derived_from_durable_object = true``, and
  an envelope whose durable job ref and reopen anchor match the descriptor;
- every qualified guarantee carries its required captured evidence (an envelope
  ref, a declared privacy class, and an evidence-freshness stamp for every
  guarantee; a lock-screen and payload disclosure for the privacy guarantee; a
  badge basis and count class for the badge guarantee; a progress basis for the
  progress guarantee; a suppression parity, decision, and visible audit for the
  suppression guarantee; a reopen outcome for the reopen guarantee) and a
  present reopen outcome on every high-stakes surface;
- no qualified guarantee carries a red result (a lock-screen leak, a protected
  payload body, a raw-event badge counter, a generic progress spinner, a
  diverging suppression decision, a missing suppression audit, or a lost reopen
  target);
- no surface paints an OS affordance from a synthesized desktop-only state, no
  marketed guarantee is claimed with no evidence, and no marketed guarantee
  carries stale evidence;
- no surface carries any blocking finding;
- the support-export wrapper quotes every surface id and descriptor revision the
  audit exposes; and
- the published markdown audit and the companion doc are present and back-link
  the canonical schema, fixtures, and CLI gate.

Exit codes:

- ``0`` -- audit is clean (all five guarantees qualified, no blockers).
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

REPORT_REL = Path("fixtures/ux/m5_os_notifications_and_badges/report.json")
SUPPORT_EXPORT_REL = Path("fixtures/ux/m5_os_notifications_and_badges/support_export.json")
COMPACT_REL = Path("fixtures/ux/m5_os_notifications_and_badges/compact.txt")
SCHEMA_REL = Path("schemas/ux/m5-os-notification-envelope.schema.json")
MARKDOWN_REL = Path("artifacts/ux/m5/os-notification-and-reopen.md")
DOC_REL = Path("docs/m5/os-notifications-badges-and-progress.md")

REQUIRED_GUARANTEES = (
    "privacy_safe_summary",
    "badge_durable_class",
    "progress_named_job_class",
    "suppression_parity",
    "exact_reopen_parity",
)

EXPECTED_RECORD_KIND_REPORT = "shell_m5_os_attention_report_record"
EXPECTED_RECORD_KIND_ROW = "shell_m5_os_attention_row_record"
EXPECTED_RECORD_KIND_ENVELOPE = "shell_m5_os_notification_envelope_record"
EXPECTED_RECORD_KIND_SUPPORT = "shell_m5_os_attention_support_export_record"
EXPECTED_SHARED_CONTRACT_REF = "shell:m5_os_notifications_and_badges:v1"
EXPECTED_SCHEMA_VERSION = 1

HIGH_STAKES_CLASSES = {
    "security_critical",
    "managed_sensitive",
}

DOC_BACKLINKS = (
    "artifacts/ux/m5/os-notification-and-reopen.md",
    "fixtures/ux/m5_os_notifications_and_badges/report.json",
    "schemas/ux/m5-os-notification-envelope.schema.json",
    "tools/ci/m5/os_notifications_and_badges_check.py",
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
    return descriptor.get("privacy_class") in HIGH_STAKES_CLASSES


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
        "projected_envelope_ref",
        "projected_privacy_class",
        "evidence_freshness",
    ]
    if guarantee == "privacy_safe_summary":
        required_fields.append("projected_lock_screen")
        required_fields.append("projected_payload_disclosure")
    if guarantee == "badge_durable_class":
        required_fields.append("projected_badge_basis")
        required_fields.append("projected_badge_count_class")
    if guarantee == "progress_named_job_class":
        required_fields.append("projected_progress_basis")
    if guarantee == "suppression_parity":
        required_fields.append("projected_suppression_parity")
        required_fields.append("projected_suppression_decision")
        required_fields.append("projected_suppression_audit_visible")
    if guarantee == "exact_reopen_parity":
        required_fields.append("projected_reopen_outcome")
    if high_stakes:
        required_fields.append("projected_reopen_outcome")
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
    if binding.get("projected_lock_screen") == "leaks_protected_detail":
        findings.append(
            Finding("lock_screen_leak", "lock-screen copy leaks protected detail", surface_id, guarantee)
        )
    if binding.get("projected_payload_disclosure") == "carries_protected_body":
        findings.append(
            Finding(
                "protected_payload_body",
                "OS notification packet carries a protected payload body",
                surface_id,
                guarantee,
            )
        )
    if binding.get("projected_badge_basis") == "raw_event_fanout":
        findings.append(
            Finding(
                "badge_raw_event_fanout",
                "badge derived from raw event fanout instead of a durable count class",
                surface_id,
                guarantee,
            )
        )
    if binding.get("projected_progress_basis") == "generic_spinner":
        findings.append(
            Finding(
                "progress_generic_spinner",
                "taskbar/dock progress mapped to a generic spinner instead of a named job class",
                surface_id,
                guarantee,
            )
        )
    if binding.get("projected_suppression_parity") == "diverges_from_in_app":
        findings.append(
            Finding(
                "suppression_divergence",
                "OS suppression decision diverges from the in-app decision",
                surface_id,
                guarantee,
            )
        )
    if guarantee == "suppression_parity" and binding.get(
        "projected_suppression_audit_visible"
    ) is False:
        findings.append(
            Finding(
                "suppression_audit_missing",
                "suppressed guarantee keeps no visible suppression audit",
                surface_id,
                guarantee,
            )
        )
    if binding.get("projected_reopen_outcome") == "target_lost":
        findings.append(
            Finding("reopen_target_lost", "exact-target reopen affordance lost", surface_id, guarantee)
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

    for ref_field, code in (
        ("reopen_anchor_ref", "descriptor_missing_reopen_anchor"),
        ("durable_job_id_ref", "missing_durable_job_ref"),
        ("support_note", "missing_support_note"),
        ("source_object_label_ref", "missing_source_object_label"),
        ("safe_reopen_action_label_ref", "missing_safe_reopen_action"),
    ):
        value = descriptor.get(ref_field)
        if not isinstance(value, str) or not value.strip():
            findings.append(
                Finding(code, f"descriptor.{ref_field} must be non-empty", surface_id=surface_id)
            )

    if not isinstance(descriptor.get("privacy_class"), str) or not descriptor.get(
        "privacy_class"
    ):
        findings.append(
            Finding(
                "missing_privacy_class",
                "descriptor.privacy_class must be declared",
                surface_id=surface_id,
            )
        )

    if descriptor.get("derived_from_durable_object") is not True:
        findings.append(
            Finding(
                "surface_not_derived_from_durable_object",
                "descriptor.derived_from_durable_object must be true",
                surface_id=surface_id,
            )
        )

    high_stakes = descriptor_high_stakes(descriptor)

    if high_stakes and not ensure_list(
        descriptor.get("suppression_controls", []), "descriptor.suppression_controls"
    ):
        findings.append(
            Finding(
                "missing_suppression_controls",
                "high-stakes surface must expose suppression controls",
                surface_id=surface_id,
            )
        )

    # Envelope / descriptor consistency.
    envelope = ensure_dict(surface.get("envelope", {}), "surface.envelope")
    if envelope.get("record_kind") != EXPECTED_RECORD_KIND_ENVELOPE:
        findings.append(
            Finding(
                "envelope_record_kind_mismatch",
                f"envelope.record_kind must be {EXPECTED_RECORD_KIND_ENVELOPE}",
                surface_id=surface_id,
                detail={"record_kind": envelope.get("record_kind")},
            )
        )
    if envelope.get("durable_job_id_ref") != descriptor.get("durable_job_id_ref"):
        findings.append(
            Finding(
                "envelope_descriptor_mismatch",
                "envelope.durable_job_id_ref must match the descriptor",
                surface_id=surface_id,
                detail={"field": "durable_job_id_ref"},
            )
        )
    reopen = ensure_dict(envelope.get("reopen_linkage", {}), "envelope.reopen_linkage")
    if reopen.get("reopen_anchor_ref") != descriptor.get("reopen_anchor_ref"):
        findings.append(
            Finding(
                "envelope_descriptor_mismatch",
                "envelope.reopen_linkage.reopen_anchor_ref must match the descriptor",
                surface_id=surface_id,
                detail={"field": "reopen_anchor_ref"},
            )
        )
    if reopen.get("must_resolve_through_in_product_surface") is not True:
        findings.append(
            Finding(
                "reopen_not_in_product",
                "envelope.reopen_linkage.must_resolve_through_in_product_surface must be true",
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
                    "surface is missing a required parity guarantee binding",
                    surface_id=surface_id,
                    guarantee=required,
                )
            )

    for binding in bindings:
        guarantee = binding.get("guarantee")
        status = binding.get("qualification_status")
        if status == "unqualified_desktop_only_state":
            findings.append(
                Finding(
                    "unqualified_desktop_only_state",
                    "surface paints an OS affordance from a synthesized desktop-only state",
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
        elif status in ("explicitly_narrowed", "not_applicable", "platform_omitted"):
            reason = binding.get("narrowing_reason")
            if not isinstance(reason, str) or not reason.strip():
                findings.append(
                    Finding(
                        "missing_narrowing_reason",
                        "narrowed guarantee must carry a narrowing reason",
                        surface_id=surface_id,
                        guarantee=guarantee,
                    )
                )

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
                    "companion doc must quote every required parity guarantee",
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
            print("m5 os-notifications-and-badges audit: clean")
        else:
            for finding in findings:
                location = finding.surface_id or "report"
                if finding.guarantee:
                    location = f"{location} / {finding.guarantee}"
                print(f"FAIL [{finding.code}] {location}: {finding.message}")

    return 1 if findings else 0


if __name__ == "__main__":
    sys.exit(main())
