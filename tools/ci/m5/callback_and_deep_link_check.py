#!/usr/bin/env python3
"""Auth-callback and deep-link review CI gate.

This gate enforces that the checked-in callback-review report stays fresh and
clean across the six required callback/deep-link entry kinds (auth_provider_
callback, protocol_deep_link, review_handoff_link, collaboration_join_link,
managed_resume_link, remote_mutation_link). It reads:

- the report fixture at ``fixtures/platform/m5-callback-and-deep-link/report.json``;
- the support-export fixture at
  ``fixtures/platform/m5-callback-and-deep-link/support_export.json``;
- the four per-incident case exports under
  ``fixtures/platform/m5-callback-and-deep-link/cases/``;
- the boundary schema at ``schemas/platform/m5-deep-link-review.schema.json``; and
- (when present) the published markdown at
  ``artifacts/platform/m5-auth-callback-and-deep-link.md`` and the companion doc
  at ``docs/m5/auth-callback-and-protocol-handlers.md``.

For the report the gate verifies that:

- the report covers every required entry kind;
- at least one entry provably reuses the in-product authority path;
- every entry discloses an origin, names a target identity, carries a pending-
  correlation alias, an expiry, an active-profile owner, a trust checkpoint, the
  canonical in-product command, a non-empty continuity note, a non-empty
  degraded-state vocabulary, at least one platform, a downgrade rule,
  ``redaction_safe = true``, and ``registered_on_callback_review_harness = true``;
- any authority wider than a plain local open is gated behind a confirm/reject
  sheet that names a sheet ref;
- an admitted return carries a verified origin (an admitted origin_unverified
  return is rejected);
- any denied return offers at least one recovery action;
- no marketed entry carries stale evidence;
- no entry carries any blocking finding, so the distinct failure classes (a
  silent authority widen, a silent remote mutation, a bypassed origin
  verification, a wrong-origin auth-failure look-alike, an expired silent
  no-op, an unsurfaced stale state, a policy dead-end, a lost local continuity,
  and a raw-target leak) are all caught;
- the report cross-links the browser-handoff, embedded-boundary, provider-
  origin, auth-recovery, system-entry, and entry-interstitial packets;
- the support-export wrapper quotes the report id, every entry id, and every
  descriptor revision; and
- the four incident case exports (wrong_origin, expired, stale, denied) exist,
  each carries a denied outcome with at least one recovery action, and the
  published markdown and companion doc back-link the canonical schema, fixtures,
  and CLI gate.

Exit codes:

- ``0`` -- report is clean (all kinds covered, parity present, no blockers).
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

REPORT_REL = Path("fixtures/platform/m5-callback-and-deep-link/report.json")
SUPPORT_EXPORT_REL = Path("fixtures/platform/m5-callback-and-deep-link/support_export.json")
CASES_DIR_REL = Path("fixtures/platform/m5-callback-and-deep-link/cases")
SCHEMA_REL = Path("schemas/platform/m5-deep-link-review.schema.json")
MARKDOWN_REL = Path("artifacts/platform/m5-auth-callback-and-deep-link.md")
DOC_REL = Path("docs/m5/auth-callback-and-protocol-handlers.md")

REQUIRED_KINDS = (
    "auth_provider_callback",
    "protocol_deep_link",
    "review_handoff_link",
    "collaboration_join_link",
    "managed_resume_link",
    "remote_mutation_link",
)

CROSS_LINK_FIELDS = (
    "browser_handoff_ref",
    "embedded_boundary_ref",
    "provider_origin_ref",
    "auth_recovery_ref",
    "system_entry_ref",
    "entry_interstitial_ref",
)

# (entry_id, case_label) for the four required incident exports.
REQUIRED_CASES = (
    ("callback:case.wrong_origin", "wrong_origin"),
    ("callback:case.expired", "expired"),
    ("callback:case.stale", "stale"),
    ("callback:case.denied", "denied"),
)

EXPECTED_RECORD_KIND_REPORT = "auth_m5_callback_and_deep_link_review_report_record"
EXPECTED_RECORD_KIND_ROW = "auth_m5_callback_and_deep_link_review_entry_record"
EXPECTED_RECORD_KIND_SUPPORT = "auth_m5_callback_and_deep_link_review_support_export_record"
EXPECTED_RECORD_KIND_CASE = "auth_m5_callback_and_deep_link_review_case_export_record"
EXPECTED_SHARED_CONTRACT_REF = "auth:m5_callback_and_deep_link_review:v1"
EXPECTED_SCHEMA_VERSION = 1

DENIED_OUTCOMES = (
    "denied_wrong_origin",
    "denied_expired",
    "denied_stale",
    "denied_by_policy",
)

DOC_BACKLINKS = (
    "artifacts/platform/m5-auth-callback-and-deep-link.md",
    "fixtures/platform/m5-callback-and-deep-link/report.json",
    "schemas/platform/m5-deep-link-review.schema.json",
    "tools/ci/m5/callback_and_deep_link_check.py",
)


@dataclass
class Finding:
    """One blocking finding emitted by the gate."""

    code: str
    message: str
    entry_id: str | None = None
    detail: dict[str, Any] = field(default_factory=dict)

    def as_dict(self) -> dict[str, Any]:
        out: dict[str, Any] = {"code": self.code, "message": self.message}
        if self.entry_id is not None:
            out["entry_id"] = self.entry_id
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


def is_nonempty_str(value: Any) -> bool:
    return isinstance(value, str) and value.strip() != ""


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
    if report.get("report_clean") is not True:
        findings.append(Finding("report_not_clean", "report.report_clean must be true"))


def check_cross_links(report: dict[str, Any], findings: list[Finding]) -> None:
    cross_links = report.get("cross_links")
    if not isinstance(cross_links, dict):
        findings.append(Finding("cross_links_missing", "report.cross_links must be an object"))
        return
    for field_name in CROSS_LINK_FIELDS:
        if not is_nonempty_str(cross_links.get(field_name)):
            findings.append(
                Finding("cross_link_missing", "report.cross_links field is empty", detail={"field": field_name})
            )


def check_required_coverage(report: dict[str, Any], findings: list[Finding]) -> None:
    entries = ensure_list(report.get("entries", []), "report.entries")
    present_kinds = {
        ensure_dict(entry.get("descriptor", {}), "entry.descriptor").get("entry_kind")
        for entry in entries
    }
    for kind in REQUIRED_KINDS:
        if kind not in present_kinds:
            findings.append(
                Finding("missing_required_kind", "report is missing a required entry kind", detail={"kind": kind})
            )
    any_parity = any(
        entry.get("confirm_reject_outcome", {}).get("reuses_in_product_authority_path") is True
        for entry in entries
    )
    if not any_parity:
        findings.append(
            Finding("no_confirm_reject_parity", "no entry provably reuses the in-product authority path")
        )


def check_entry(entry: dict[str, Any], findings: list[Finding]) -> None:
    if entry.get("record_kind") != EXPECTED_RECORD_KIND_ROW:
        findings.append(
            Finding(
                "row_record_kind_mismatch",
                f"entry.record_kind must be {EXPECTED_RECORD_KIND_ROW}",
                detail={"record_kind": entry.get("record_kind")},
            )
        )
    descriptor = ensure_dict(entry.get("descriptor", {}), "entry.descriptor")
    entry_id = descriptor.get("entry_id")

    for field_name in (
        "disclosed_origin_ref",
        "target_identity_ref",
        "pending_correlation_ref",
        "expiry_at",
        "active_profile_owner_ref",
        "trust_checkpoint_ref",
        "canonical_command_ref",
        "continuity_note",
        "downgrade_rule_ref",
        "descriptor_revision_ref",
    ):
        if not is_nonempty_str(descriptor.get(field_name)):
            findings.append(
                Finding("descriptor_field_missing", "descriptor field is empty", entry_id=entry_id, detail={"field": field_name})
            )

    vocab = descriptor.get("degraded_state_vocabulary")
    if not isinstance(vocab, list) or not any(is_nonempty_str(v) for v in vocab):
        findings.append(Finding("missing_degraded_state_vocabulary", "descriptor.degraded_state_vocabulary must be non-empty", entry_id=entry_id))

    platforms = descriptor.get("claimed_platforms")
    if not isinstance(platforms, list) or not platforms:
        findings.append(Finding("missing_claimed_platforms", "descriptor.claimed_platforms must be non-empty", entry_id=entry_id))

    if descriptor.get("redaction_safe") is not True:
        findings.append(
            Finding("raw_target_leak", "descriptor.redaction_safe must be true", entry_id=entry_id)
        )

    if descriptor.get("registered_on_callback_review_harness") is not True:
        findings.append(
            Finding("entry_not_on_harness", "descriptor.registered_on_callback_review_harness must be true", entry_id=entry_id)
        )

    if descriptor.get("marketed") and descriptor.get("evidence_freshness") == "stale":
        findings.append(
            Finding("stale_evidence_on_marketed_entry", "marketed entry carries stale evidence", entry_id=entry_id)
        )

    # Spoof resistance: an admitted return must carry a verified origin.
    if descriptor.get("outcome") == "admitted" and descriptor.get("origin_assurance") == "origin_unverified":
        findings.append(
            Finding("origin_verification_bypassed", "an admitted return must carry a verified origin", entry_id=entry_id)
        )

    # Confirm/reject discipline: anything wider than a plain local open must be
    # gated behind a confirm/reject sheet.
    scope = descriptor.get("authority_scope")
    if scope and scope != "plain_local_open":
        if descriptor.get("requires_confirm_reject") is not True:
            findings.append(
                Finding("authority_widen_not_gated", "authority wider than plain_local_open must require a confirm/reject sheet", entry_id=entry_id, detail={"authority_scope": scope})
            )
        if not is_nonempty_str(descriptor.get("confirm_reject_sheet_ref")):
            findings.append(
                Finding("missing_confirm_reject_sheet", "a gated entry must name a confirm/reject sheet ref", entry_id=entry_id, detail={"authority_scope": scope})
            )

    # Recovery: a denied return must offer a recovery action.
    outcome = descriptor.get("outcome")
    if outcome in DENIED_OUTCOMES:
        recovery = descriptor.get("recovery_actions")
        if not isinstance(recovery, list) or not recovery:
            findings.append(
                Finding("missing_recovery_action", "denied return must offer a recovery action", entry_id=entry_id, detail={"outcome": outcome})
            )

    for blocker in ensure_list(entry.get("blocking_findings", []), "entry.blocking_findings"):
        findings.append(
            Finding(
                "blocking_finding_present",
                "entry carries a blocking finding",
                entry_id=entry_id,
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
    if report.get("report_id") not in case_set:
        findings.append(
            Finding("support_missing_report_id", "support_export.case_ids must quote the report id", detail={"report_id": report.get("report_id")})
        )
    for entry in ensure_list(report.get("entries", []), "report.entries"):
        descriptor = ensure_dict(entry.get("descriptor", {}), "entry.descriptor")
        entry_id = descriptor.get("entry_id")
        revision = descriptor.get("descriptor_revision_ref")
        if entry_id not in case_set:
            findings.append(Finding("support_missing_entry_id", "support_export.case_ids must quote every entry id", entry_id=entry_id))
        if revision not in case_set:
            findings.append(
                Finding("support_missing_descriptor_revision", "support_export.case_ids must quote every descriptor revision", entry_id=entry_id, detail={"descriptor_revision_ref": revision})
            )


def check_case_exports(repo_root: Path, findings: list[Finding]) -> None:
    for entry_id, label in REQUIRED_CASES:
        path = repo_root / CASES_DIR_REL / f"{label}.json"
        if not path.exists():
            findings.append(Finding("case_export_missing", "missing required incident case export", detail={"case": label}))
            continue
        export = ensure_dict(load_json(path), f"case[{label}]")
        if export.get("record_kind") != EXPECTED_RECORD_KIND_CASE:
            findings.append(Finding("case_record_kind_mismatch", "case export record_kind mismatch", detail={"case": label, "record_kind": export.get("record_kind")}))
        if export.get("case_label") != label:
            findings.append(Finding("case_label_mismatch", "case export label mismatch", detail={"case": label, "case_label": export.get("case_label")}))
        if export.get("outcome") not in DENIED_OUTCOMES:
            findings.append(Finding("case_outcome_not_denied", "case export must carry a denied outcome", detail={"case": label, "outcome": export.get("outcome")}))
        recovery = export.get("recovery_actions")
        if not isinstance(recovery, list) or not recovery:
            findings.append(Finding("case_missing_recovery", "case export must offer a recovery action", detail={"case": label}))
        entry = ensure_dict(export.get("entry", {}), f"case[{label}].entry")
        descriptor = ensure_dict(entry.get("descriptor", {}), f"case[{label}].entry.descriptor")
        if descriptor.get("entry_id") != entry_id:
            findings.append(Finding("case_entry_id_mismatch", "case export entry id mismatch", detail={"case": label, "entry_id": descriptor.get("entry_id")}))


def check_publications(repo_root: Path, findings: list[Finding]) -> None:
    markdown = repo_root / MARKDOWN_REL
    if not markdown.exists():
        findings.append(Finding("published_markdown_missing", f"missing published markdown: {MARKDOWN_REL}"))
    doc = repo_root / DOC_REL
    if not doc.exists():
        findings.append(Finding("published_doc_missing", f"missing companion doc: {DOC_REL}"))
        return
    body = doc.read_text(encoding="utf-8")
    for kind in REQUIRED_KINDS:
        if kind not in body:
            findings.append(Finding("doc_missing_kind", "companion doc must quote every required entry kind", detail={"kind": kind}))
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

    findings: list[Finding] = []
    check_report_envelope(report, findings)
    check_cross_links(report, findings)
    check_required_coverage(report, findings)
    for entry in ensure_list(report.get("entries", []), "report.entries"):
        check_entry(ensure_dict(entry, "entry"), findings)
    check_support_export(report, export, findings)
    check_case_exports(repo_root, findings)
    check_publications(repo_root, findings)

    if args.format == "json":
        print(json.dumps({"findings": [f.as_dict() for f in findings]}, indent=2))
    else:
        if not findings:
            print("m5 callback and deep-link review: clean")
        else:
            for finding in findings:
                location = finding.entry_id or "report"
                print(f"FAIL [{finding.code}] {location}: {finding.message}")

    return 1 if findings else 0


if __name__ == "__main__":
    sys.exit(main())
