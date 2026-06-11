#!/usr/bin/env python3
"""M5 notification privacy, quiet-hours, and badge qualification audit CI gate.

This gate enforces that the checked-in M5 notification-route audit stays fresh
and clean across the nine notification guarantees the M5 depth lanes must pass:
privacy_classification, lock_screen_privacy, payload_minimization,
quiet_hours_policy, admin_suppression, root_cause_dedupe, badge_semantics,
exact_target_reopen, and companion_fanout_honesty. It reads:

- the audit fixture at ``fixtures/ux/m5/notification-dedupe/report.json``;
- the support-export fixture at
  ``fixtures/ux/m5/notification-dedupe/support_export.json``;
- the boundary schema at
  ``schemas/ux/m5-notification-envelope-diff.schema.json``; and
- (when present) the published markdown at
  ``artifacts/ux/m5/quiet-hours-and-privacy/m5_notification_routes_audit.md``
  and the companion doc at ``docs/m5/notification-privacy-and-badges.md``.

For the audit the gate verifies that:

- the audit covers all nine required guarantees and at least one source
  qualifies each guarantee;
- every registered source has a binding for every required guarantee;
- every source carries a canonical exact-target reopen anchor, a non-empty
  support note, a declared privacy class, at least one declared fanout channel,
  and ``routed_through_governed_router = true``;
- every qualified guarantee carries its required captured evidence (a
  notification-envelope ref, a declared privacy class, a lock-screen
  disclosure, and an evidence-freshness stamp for every guarantee; a
  payload-minimization outcome for the payload guarantee; a quiet-hours outcome
  for the quiet-hours guarantee; an admin-suppression outcome for the admin
  guarantee; a dedupe outcome for the dedupe guarantee; a badge outcome for the
  badge guarantee; a reopen outcome for the reopen guarantee; an honest fanout
  label for the companion guarantee) and a present reopen outcome on every
  high-stakes source;
- no qualified guarantee carries a red result (a lock-screen leak, a
  secret-bearing payload, a bypassed quiet-hours window, an overridden admin
  suppression, a duplicate flood, a raw-event badge counter, a lost reopen
  target, or a silent fanout failure);
- no source invents a feature-local notification rule, no marketed guarantee is
  claimed with no evidence, and no marketed guarantee carries stale evidence;
- no source carries any blocking finding (so aspect, narrowing, and projection
  drift are all caught);
- the support-export wrapper quotes every source id and descriptor revision the
  audit exposes; and
- the published markdown audit and the companion doc are present and back-link
  the canonical schema, fixtures, and CLI gate.

Exit codes:

- ``0`` -- audit is clean (all nine guarantees qualified, no blockers).
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

REPORT_REL = Path("fixtures/ux/m5/notification-dedupe/report.json")
SUPPORT_EXPORT_REL = Path("fixtures/ux/m5/notification-dedupe/support_export.json")
COMPACT_REL = Path("fixtures/ux/m5/notification-dedupe/compact.txt")
SCHEMA_REL = Path("schemas/ux/m5-notification-envelope-diff.schema.json")
MARKDOWN_REL = Path(
    "artifacts/ux/m5/quiet-hours-and-privacy/m5_notification_routes_audit.md"
)
DOC_REL = Path("docs/m5/notification-privacy-and-badges.md")

REQUIRED_GUARANTEES = (
    "privacy_classification",
    "lock_screen_privacy",
    "payload_minimization",
    "quiet_hours_policy",
    "admin_suppression",
    "root_cause_dedupe",
    "badge_semantics",
    "exact_target_reopen",
    "companion_fanout_honesty",
)

EXPECTED_RECORD_KIND_REPORT = "shell_m5_notification_route_report_record"
EXPECTED_RECORD_KIND_ROW = "shell_m5_notification_route_row_record"
EXPECTED_RECORD_KIND_SUPPORT = "shell_m5_notification_route_support_export_record"
EXPECTED_SHARED_CONTRACT_REF = "shell:m5_notification_routes:v1"
EXPECTED_SCHEMA_VERSION = 1

HIGH_STAKES_CLASSES = {
    "security_critical",
    "managed_sensitive",
}

DOC_BACKLINKS = (
    "artifacts/ux/m5/quiet-hours-and-privacy/m5_notification_routes_audit.md",
    "fixtures/ux/m5/notification-dedupe/report.json",
    "schemas/ux/m5-notification-envelope-diff.schema.json",
    "tools/ci/m5/notification_routes_check.py",
)


@dataclass
class Finding:
    """One blocking finding emitted by the gate."""

    code: str
    message: str
    source_id: str | None = None
    guarantee: str | None = None
    detail: dict[str, Any] = field(default_factory=dict)

    def as_dict(self) -> dict[str, Any]:
        out: dict[str, Any] = {"code": self.code, "message": self.message}
        if self.source_id is not None:
            out["source_id"] = self.source_id
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
        for source in rows:
            for binding in ensure_list(source.get("bindings", []), "source.bindings"):
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
                    "no qualified source for required guarantee",
                    guarantee=required,
                )
            )


def check_qualified_binding(
    source_id: str,
    high_stakes: bool,
    binding: dict[str, Any],
    findings: list[Finding],
) -> None:
    guarantee = binding.get("guarantee")

    required_fields = [
        "projected_envelope_ref",
        "projected_privacy_class",
        "projected_lock_screen",
        "evidence_freshness",
    ]
    if guarantee == "payload_minimization":
        required_fields.append("projected_payload_disclosure")
    if guarantee == "quiet_hours_policy":
        required_fields.append("projected_quiet_hours")
    if guarantee == "admin_suppression":
        required_fields.append("projected_admin_suppression")
    if guarantee == "root_cause_dedupe":
        required_fields.append("projected_dedupe")
    if guarantee == "badge_semantics":
        required_fields.append("projected_badge")
    if guarantee == "exact_target_reopen":
        required_fields.append("projected_reopen_outcome")
    if guarantee == "companion_fanout_honesty":
        required_fields.append("projected_fanout_honesty")
    if high_stakes:
        required_fields.append("projected_reopen_outcome")
    for field_name in dict.fromkeys(required_fields):
        if binding.get(field_name) is None:
            findings.append(
                Finding(
                    "missing_projection",
                    "qualified guarantee is missing required captured evidence",
                    source_id=source_id,
                    guarantee=guarantee,
                    detail={"field": field_name},
                )
            )

    # Red captured results.
    if binding.get("projected_lock_screen") == "leaks_detail":
        findings.append(
            Finding("lock_screen_leak", "lock-screen copy leaks detail", source_id, guarantee)
        )
    if binding.get("projected_payload_disclosure") == "carries_secret_body":
        findings.append(
            Finding(
                "secret_bearing_payload",
                "notification packet carries a secret-bearing payload",
                source_id,
                guarantee,
            )
        )
    if binding.get("projected_quiet_hours") == "bypassed":
        findings.append(
            Finding("quiet_hours_bypassed", "quiet-hours window bypassed", source_id, guarantee)
        )
    if binding.get("projected_admin_suppression") == "overridden":
        findings.append(
            Finding(
                "admin_suppression_overridden",
                "admin suppression overridden",
                source_id,
                guarantee,
            )
        )
    if binding.get("projected_dedupe") == "floods_duplicates":
        findings.append(
            Finding("duplicate_flood", "semantically identical alerts flood the user", source_id, guarantee)
        )
    if binding.get("projected_badge") == "raw_event_fanout":
        findings.append(
            Finding(
                "badge_raw_event_fanout",
                "badge derived from raw event fanout instead of durable item state",
                source_id,
                guarantee,
            )
        )
    if binding.get("projected_reopen_outcome") == "target_lost":
        findings.append(
            Finding("reopen_target_lost", "exact-target reopen affordance lost", source_id, guarantee)
        )
    if binding.get("projected_fanout_honesty") == "silent_failure":
        findings.append(
            Finding("fanout_failure_silent", "companion fanout failure hidden", source_id, guarantee)
        )
    if binding.get("marketed_on_guarantee") and binding.get("evidence_freshness") == "stale":
        findings.append(
            Finding(
                "stale_evidence_on_marketed_row",
                "marketed guarantee carries stale evidence",
                source_id,
                guarantee,
            )
        )


def check_source(source: dict[str, Any], findings: list[Finding]) -> None:
    descriptor = ensure_dict(source.get("descriptor", {}), "source.descriptor")
    source_id = descriptor.get("source_id")
    if not isinstance(source_id, str) or not source_id.strip():
        findings.append(Finding("missing_source_id", "descriptor.source_id must be non-empty"))
        return

    if source.get("record_kind") != EXPECTED_RECORD_KIND_ROW:
        findings.append(
            Finding(
                "source_record_kind_mismatch",
                f"source.record_kind must be {EXPECTED_RECORD_KIND_ROW}",
                source_id=source_id,
                detail={"record_kind": source.get("record_kind")},
            )
        )

    revision = descriptor.get("descriptor_revision_ref")
    if not isinstance(revision, str) or not revision.strip():
        findings.append(
            Finding(
                "missing_descriptor_revision_ref",
                "descriptor.descriptor_revision_ref must be non-empty",
                source_id=source_id,
            )
        )

    anchor = descriptor.get("reopen_anchor_ref")
    if not isinstance(anchor, str) or not anchor.strip():
        findings.append(
            Finding(
                "descriptor_missing_reopen_anchor",
                "descriptor.reopen_anchor_ref must be non-empty",
                source_id=source_id,
            )
        )

    note = descriptor.get("support_note")
    if not isinstance(note, str) or not note.strip():
        findings.append(
            Finding(
                "missing_support_note",
                "descriptor.support_note must be non-empty",
                source_id=source_id,
            )
        )

    if not isinstance(descriptor.get("privacy_class"), str) or not descriptor.get(
        "privacy_class"
    ):
        findings.append(
            Finding(
                "missing_privacy_class",
                "descriptor.privacy_class must be declared",
                source_id=source_id,
            )
        )

    if descriptor.get("routed_through_governed_router") is not True:
        findings.append(
            Finding(
                "source_not_on_governed_router",
                "descriptor.routed_through_governed_router must be true",
                source_id=source_id,
            )
        )

    high_stakes = descriptor_high_stakes(descriptor)

    if high_stakes and not ensure_list(
        descriptor.get("suppression_controls", []), "descriptor.suppression_controls"
    ):
        findings.append(
            Finding(
                "missing_suppression_controls",
                "high-stakes source must expose suppression controls",
                source_id=source_id,
            )
        )

    if descriptor.get("marketed_on_desktop") and not ensure_list(
        descriptor.get("fanout_channels", []), "descriptor.fanout_channels"
    ):
        findings.append(
            Finding(
                "no_declared_channel",
                "marketed source must declare a fanout channel",
                source_id=source_id,
            )
        )

    # Every required guarantee must be bound.
    bindings = ensure_list(source.get("bindings", []), "source.bindings")
    present = {binding.get("guarantee") for binding in bindings}
    for required in REQUIRED_GUARANTEES:
        if required not in present:
            findings.append(
                Finding(
                    "missing_required_guarantee",
                    "source is missing a required notification guarantee binding",
                    source_id=source_id,
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
                    source_id=source_id,
                    guarantee=guarantee,
                    detail={"aspect": aspect, "expected": expected_aspect},
                )
            )
        status = binding.get("qualification_status")
        if status == "unqualified_local_rule":
            findings.append(
                Finding(
                    "unqualified_local_rule",
                    "source emits through a feature-local rule outside the governed router",
                    source_id=source_id,
                    guarantee=guarantee,
                )
            )
        elif status == "missing_evidence":
            findings.append(
                Finding(
                    "missing_evidence",
                    "marketed guarantee claimed with no captured evidence",
                    source_id=source_id,
                    guarantee=guarantee,
                )
            )
        elif status == "qualified":
            check_qualified_binding(source_id, high_stakes, binding, findings)

    # Any blocking finding the Rust validator emitted is a gate failure.
    for blocker in ensure_list(
        source.get("blocking_findings", []), "source.blocking_findings"
    ):
        findings.append(
            Finding(
                "blocking_finding_present",
                "source carries a blocking finding",
                source_id=source_id,
                guarantee=blocker.get("guarantee"),
                detail={"class": blocker.get("class")},
            )
        )


def canonical_aspect(guarantee: Any) -> str | None:
    if guarantee in ("privacy_classification", "lock_screen_privacy", "payload_minimization"):
        return "privacy"
    if guarantee in ("quiet_hours_policy", "admin_suppression"):
        return "suppression"
    if guarantee in ("root_cause_dedupe", "badge_semantics"):
        return "dedupe"
    if guarantee in ("exact_target_reopen", "companion_fanout_honesty"):
        return "routing"
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
    for source in ensure_list(report.get("rows", []), "report.rows"):
        descriptor = ensure_dict(source.get("descriptor", {}), "source.descriptor")
        source_id = descriptor.get("source_id")
        revision = descriptor.get("descriptor_revision_ref")
        if source_id not in case_set:
            findings.append(
                Finding(
                    "support_missing_source_id",
                    "support_export.case_ids must quote every source id",
                    source_id=source_id,
                )
            )
        if revision not in case_set:
            findings.append(
                Finding(
                    "support_missing_descriptor_revision",
                    "support_export.case_ids must quote every descriptor revision",
                    source_id=source_id,
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
                    "companion doc must quote every required notification guarantee",
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
    for source in ensure_list(report.get("rows", []), "report.rows"):
        check_source(ensure_dict(source, "source"), findings)
    check_support_export(report, export, findings)
    check_publications(repo_root, findings)

    if args.format == "json":
        print(json.dumps({"findings": [f.as_dict() for f in findings]}, indent=2))
    else:
        if not findings:
            print("m5 notification-routes audit: clean")
        else:
            for finding in findings:
                location = finding.source_id or "report"
                if finding.guarantee:
                    location = f"{location} / {finding.guarantee}"
                print(f"FAIL [{finding.code}] {location}: {finding.message}")

    return 1 if findings else 0


if __name__ == "__main__":
    sys.exit(main())
