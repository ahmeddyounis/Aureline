#!/usr/bin/env python3
"""Store-lock and external-root recovery CI gate.

This gate enforces that the checked-in store-lock / external-root recovery report
stays fresh and clean across the seven required incident kinds (credential store
locked, credential store unavailable, trust-store drift, removable volume
missing, network share missing, external root missing, root returned). It reads:

- the report fixture at ``fixtures/platform/m5-store-lock-and-missing-root/report.json``;
- the support-export fixture at
  ``fixtures/platform/m5-store-lock-and-missing-root/support_export.json``;
- the four per-incident case exports under
  ``fixtures/platform/m5-store-lock-and-missing-root/cases/``;
- the boundary schema at ``schemas/platform/m5-store-lock-and-missing-root.schema.json``; and
- (when present) the published markdown at
  ``artifacts/platform/m5-store-lock-and-external-root-recovery.md`` and the
  companion doc at ``docs/m5/store-lock-and-external-root-recovery.md``.

For the report the gate verifies that:

- the report covers every required incident kind;
- every state carries a last-seen identity, a truthful placeholder, a
  repair-guidance ref, an active-profile owner, a trust checkpoint, the canonical
  in-product command, a non-empty continuity note, a non-empty degraded-state
  vocabulary, a non-empty local-only disclosure, full desktop/cli_headless/support
  surface parity, at least one platform, a downgrade rule, and
  ``registered_on_recovery_harness = true``;
- no state implies a plaintext-secret fallback
  (``implies_plaintext_fallback`` is false);
- local user-owned work is preserved (``local_continuity_preserved`` is true);
- an active degradation discloses what is paused and offers a recovery action,
  and the store-lock, trust-store-drift, and missing-root unrecoverable failures
  stay distinct;
- nothing resumes silently after recovery: ``resumes_silently_on_recovery`` is
  false, no continuation carries a ``silent_resume`` disposition, and a returned
  root requires explicit resume;
- no marketed state carries stale evidence;
- no state carries any blocking finding;
- the report cross-links the credential-store, trust-store, filesystem-identity,
  deferred-intent, auth-recovery, and Help/About surfaces;
- the support-export wrapper quotes the report id, every state id, and every
  descriptor revision; and
- the four incident case exports (credential_store_locked, trust_store_drift,
  missing_root, root_returned) exist, and the published markdown and companion
  doc back-link the canonical schema, fixtures, and CLI gate.

Exit codes:

- ``0`` -- report is clean (all kinds covered, no blockers).
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

REPORT_REL = Path("fixtures/platform/m5-store-lock-and-missing-root/report.json")
SUPPORT_EXPORT_REL = Path("fixtures/platform/m5-store-lock-and-missing-root/support_export.json")
CASES_DIR_REL = Path("fixtures/platform/m5-store-lock-and-missing-root/cases")
SCHEMA_REL = Path("schemas/platform/m5-store-lock-and-missing-root.schema.json")
MARKDOWN_REL = Path("artifacts/platform/m5-store-lock-and-external-root-recovery.md")
DOC_REL = Path("docs/m5/store-lock-and-external-root-recovery.md")

REQUIRED_KINDS = (
    "credential_store_locked",
    "credential_store_unavailable",
    "trust_store_drift",
    "removable_volume_missing",
    "network_share_missing",
    "external_root_missing",
    "root_returned",
)

CROSS_LINK_FIELDS = (
    "credential_store_ref",
    "trust_store_ref",
    "filesystem_identity_ref",
    "deferred_intent_ref",
    "auth_recovery_ref",
    "help_about_ref",
)

# (state_id, case_label) for the four required incident exports.
REQUIRED_CASES = (
    ("state:credential_store.locked", "credential_store_locked"),
    ("state:trust_store.drift", "trust_store_drift"),
    ("state:external_root.missing", "missing_root"),
    ("state:network_share.returned", "root_returned"),
)

EXPECTED_RECORD_KIND_REPORT = "auth_m5_store_lock_and_external_root_recovery_report_record"
EXPECTED_RECORD_KIND_ROW = "auth_m5_store_lock_and_external_root_recovery_state_record"
EXPECTED_RECORD_KIND_SUPPORT = "auth_m5_store_lock_and_external_root_recovery_support_export_record"
EXPECTED_RECORD_KIND_CASE = "auth_m5_store_lock_and_external_root_recovery_case_export_record"
EXPECTED_SHARED_CONTRACT_REF = "auth:m5_store_lock_and_external_root_recovery:v1"
EXPECTED_SCHEMA_VERSION = 1

ACTIVE_DEGRADATION_STATES = (
    "store_locked",
    "store_unavailable",
    "trust_store_drifted",
    "root_missing",
)
STORE_LOCK_INCIDENTS = ("credential_store_locked", "credential_store_unavailable")
REQUIRED_SURFACES = ("desktop", "cli_headless", "support")

DOC_BACKLINKS = (
    "artifacts/platform/m5-store-lock-and-external-root-recovery.md",
    "fixtures/platform/m5-store-lock-and-missing-root/report.json",
    "schemas/platform/m5-store-lock-and-missing-root.schema.json",
    "tools/ci/m5/store_lock_and_external_root_check.py",
)


@dataclass
class Finding:
    """One blocking finding emitted by the gate."""

    code: str
    message: str
    state_id: str | None = None
    detail: dict[str, Any] = field(default_factory=dict)

    def as_dict(self) -> dict[str, Any]:
        out: dict[str, Any] = {"code": self.code, "message": self.message}
        if self.state_id is not None:
            out["state_id"] = self.state_id
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
        ensure_dict(entry.get("descriptor", {}), "entry.descriptor").get("incident_class")
        for entry in entries
    }
    for kind in REQUIRED_KINDS:
        if kind not in present_kinds:
            findings.append(
                Finding("missing_required_kind", "report is missing a required incident kind", detail={"kind": kind})
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
    state_id = descriptor.get("state_id")
    incident = descriptor.get("incident_class")
    degraded = descriptor.get("degraded_state_class")

    for field_name in (
        "last_seen_identity_ref",
        "placeholder_ref",
        "repair_guidance_ref",
        "active_profile_owner_ref",
        "trust_checkpoint_ref",
        "canonical_command_ref",
        "continuity_note",
        "downgrade_rule_ref",
        "descriptor_revision_ref",
    ):
        if not is_nonempty_str(descriptor.get(field_name)):
            findings.append(
                Finding("descriptor_field_missing", "descriptor field is empty", state_id=state_id, detail={"field": field_name})
            )

    vocab = descriptor.get("degraded_state_vocabulary")
    if not isinstance(vocab, list) or not any(is_nonempty_str(v) for v in vocab):
        findings.append(Finding("missing_degraded_state_vocabulary", "descriptor.degraded_state_vocabulary must be non-empty", state_id=state_id))

    local_only = descriptor.get("local_only_capabilities")
    if not isinstance(local_only, list) or not local_only:
        findings.append(Finding("missing_local_only_disclosure", "descriptor.local_only_capabilities must be non-empty", state_id=state_id))

    platforms = descriptor.get("claimed_platforms")
    if not isinstance(platforms, list) or not platforms:
        findings.append(Finding("missing_claimed_platforms", "descriptor.claimed_platforms must be non-empty", state_id=state_id))

    surface_parity = descriptor.get("surface_parity")
    surface_set = set(surface_parity) if isinstance(surface_parity, list) else set()
    if not all(surface in surface_set for surface in REQUIRED_SURFACES):
        findings.append(Finding("surface_parity_incomplete", "descriptor.surface_parity must include desktop, cli_headless, and support", state_id=state_id))

    if descriptor.get("registered_on_recovery_harness") is not True:
        findings.append(
            Finding("state_not_on_harness", "descriptor.registered_on_recovery_harness must be true", state_id=state_id)
        )

    if descriptor.get("marketed") and descriptor.get("evidence_freshness") == "stale":
        findings.append(
            Finding("stale_evidence_on_marketed_state", "marketed state carries stale evidence", state_id=state_id)
        )

    # Guardrail: never imply a plaintext-secret fallback.
    if descriptor.get("implies_plaintext_fallback") is not False:
        findings.append(
            Finding("plaintext_fallback_implied", "descriptor.implies_plaintext_fallback must be false", state_id=state_id)
        )

    # Local user-owned work must be preserved.
    if descriptor.get("local_continuity_preserved") is not True:
        findings.append(
            Finding("local_work_not_preserved", "descriptor.local_continuity_preserved must be true", state_id=state_id)
        )

    # Active-degradation discipline.
    is_active = degraded in ACTIVE_DEGRADATION_STATES
    if is_active:
        paused = descriptor.get("paused_capabilities")
        if not isinstance(paused, list) or not paused:
            findings.append(Finding("missing_paused_disclosure", "active degradation must disclose what is paused", state_id=state_id))
        recovery = descriptor.get("recovery_actions")
        if not isinstance(recovery, list) or not recovery:
            if incident in STORE_LOCK_INCIDENTS:
                code = "credential_store_lock_unrecoverable"
            elif incident == "trust_store_drift":
                code = "trust_store_drift_unrecoverable"
            else:
                code = "missing_root_unrecoverable"
            findings.append(Finding(code, "active degradation must offer a recovery action", state_id=state_id, detail={"incident_class": incident}))

    # No silent resume on recovery.
    resume_posture = descriptor.get("resume_posture")
    continuations = descriptor.get("protected_continuations")
    continuations = continuations if isinstance(continuations, list) else []
    silent_continuation = any(c.get("resume_disposition") == "silent_resume" for c in continuations)
    returned_without_explicit = degraded == "root_returned" and resume_posture != "explicit_resume_required"
    continuations_without_explicit = bool(continuations) and resume_posture != "explicit_resume_required"
    if (
        descriptor.get("resumes_silently_on_recovery") is not False
        or silent_continuation
        or returned_without_explicit
        or continuations_without_explicit
    ):
        findings.append(Finding("silent_resume_on_recovery", "no session, job, or decision may resume silently after recovery", state_id=state_id))

    for blocker in ensure_list(entry.get("blocking_findings", []), "entry.blocking_findings"):
        findings.append(
            Finding(
                "blocking_finding_present",
                "state carries a blocking finding",
                state_id=state_id,
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
        state_id = descriptor.get("state_id")
        revision = descriptor.get("descriptor_revision_ref")
        if state_id not in case_set:
            findings.append(Finding("support_missing_state_id", "support_export.case_ids must quote every state id", state_id=state_id))
        if revision not in case_set:
            findings.append(
                Finding("support_missing_descriptor_revision", "support_export.case_ids must quote every descriptor revision", state_id=state_id, detail={"descriptor_revision_ref": revision})
            )


def check_case_exports(repo_root: Path, findings: list[Finding]) -> None:
    for state_id, label in REQUIRED_CASES:
        path = repo_root / CASES_DIR_REL / f"{label}.json"
        if not path.exists():
            findings.append(Finding("case_export_missing", "missing required incident case export", detail={"case": label}))
            continue
        export = ensure_dict(load_json(path), f"case[{label}]")
        if export.get("record_kind") != EXPECTED_RECORD_KIND_CASE:
            findings.append(Finding("case_record_kind_mismatch", "case export record_kind mismatch", detail={"case": label, "record_kind": export.get("record_kind")}))
        if export.get("case_label") != label:
            findings.append(Finding("case_label_mismatch", "case export label mismatch", detail={"case": label, "case_label": export.get("case_label")}))
        recovery = export.get("recovery_actions")
        if not isinstance(recovery, list) or not recovery:
            findings.append(Finding("case_missing_recovery", "case export must offer a recovery action", detail={"case": label}))
        state = ensure_dict(export.get("state", {}), f"case[{label}].state")
        descriptor = ensure_dict(state.get("descriptor", {}), f"case[{label}].state.descriptor")
        if descriptor.get("state_id") != state_id:
            findings.append(Finding("case_state_id_mismatch", "case export state id mismatch", detail={"case": label, "state_id": descriptor.get("state_id")}))


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
            findings.append(Finding("doc_missing_kind", "companion doc must quote every required incident kind", detail={"kind": kind}))
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
            print("m5 store-lock and external-root recovery: clean")
        else:
            for finding in findings:
                location = finding.state_id or "report"
                print(f"FAIL [{finding.code}] {location}: {finding.message}")

    return 1 if findings else 0


if __name__ == "__main__":
    sys.exit(main())
