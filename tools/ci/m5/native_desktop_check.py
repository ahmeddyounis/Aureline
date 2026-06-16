#!/usr/bin/env python3
"""Native-desktop system-entry, handler-ownership, reopen, and OS-notification
matrix CI gate.

This gate enforces that the checked-in native-desktop matrix stays fresh and
clean across the ten required system-entry/reopen surface kinds (system_open,
file_association, protocol_handler, auth_callback, recent_item,
dock_taskbar_jumplist, os_notification, badge_progress, removable_path,
store_lock_state) and the seven canonical controls (trust_policy_evaluation,
channel_build_ownership, wrong_target_recovery, unavailable_path_recovery,
policy_block_recovery, signal_durability, notification_privacy). It reads:

- the matrix fixture at ``fixtures/platform/m5_os_entry_and_reopen/report.json``;
- the support-export fixture at
  ``fixtures/platform/m5_os_entry_and_reopen/support_export.json``;
- the boundary schema at
  ``schemas/platform/m5-native-desktop-matrix.schema.json``; and
- (when present) the published markdown at
  ``artifacts/platform/m5-native-desktop-matrix.md``, the companion doc at
  ``docs/m5/native-desktop-integration-and-reopen.md``, and the shiproom
  review packet.

For the matrix the gate verifies that:

- the matrix covers every required surface kind and every required control is
  satisfied by at least one surface;
- every registered surface declares a binding for every required control;
- every surface carries a channel/build owner, a trust checkpoint, a reopen
  anchor, a non-empty continuity note, a non-empty degraded-state vocabulary,
  a downgrade rule, at least one claimed platform, and
  ``registered_on_native_desktop_harness = true``;
- every satisfied control carries an evidence pack, a recovery path on the
  three recovery controls, and a durable-object ref on signal_durability;
- no control carries a failed status, so the distinct failure classes (a
  bypassed trust evaluation, a hidden handler takeover, a wrong-target reopen,
  a silent loss on an unavailable path, an unsafe policy block, a
  transient-poll signal, and a privacy-unsafe notification) are all caught;
- no narrowed control is missing its narrowing reason;
- no marketed surface carries stale evidence;
- no surface carries any blocking finding;
- the report cross-links the install-topology, embedded-boundary,
  activity-center, and auth-recovery packets;
- the support-export wrapper quotes every entry id and descriptor revision the
  matrix exposes; and
- the published markdown matrix and the companion doc are present and back-link
  the canonical schema, fixtures, and CLI gate.

Exit codes:

- ``0`` -- matrix is clean (all kinds and controls covered, no blockers).
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

REPORT_REL = Path("fixtures/platform/m5_os_entry_and_reopen/report.json")
SUPPORT_EXPORT_REL = Path("fixtures/platform/m5_os_entry_and_reopen/support_export.json")
COMPACT_REL = Path("fixtures/platform/m5_os_entry_and_reopen/compact.txt")
SCHEMA_REL = Path("schemas/platform/m5-native-desktop-matrix.schema.json")
MARKDOWN_REL = Path("artifacts/platform/m5-native-desktop-matrix.md")
DOC_REL = Path("docs/m5/native-desktop-integration-and-reopen.md")

REQUIRED_KINDS = (
    "system_open",
    "file_association",
    "protocol_handler",
    "auth_callback",
    "recent_item",
    "dock_taskbar_jumplist",
    "os_notification",
    "badge_progress",
    "removable_path",
    "store_lock_state",
)

REQUIRED_CONTROLS = (
    "trust_policy_evaluation",
    "channel_build_ownership",
    "wrong_target_recovery",
    "unavailable_path_recovery",
    "policy_block_recovery",
    "signal_durability",
    "notification_privacy",
)

RECOVERY_CONTROLS = (
    "wrong_target_recovery",
    "unavailable_path_recovery",
    "policy_block_recovery",
)
DURABLE_OBJECT_CONTROLS = ("signal_durability",)

CANONICAL_FAILURE_MODE = {
    "trust_policy_evaluation": "trust_evaluation_bypassed",
    "channel_build_ownership": "hidden_handler_takeover",
    "wrong_target_recovery": "wrong_target_no_recovery",
    "unavailable_path_recovery": "unavailable_path_silent_loss",
    "policy_block_recovery": "policy_block_unsafe",
    "signal_durability": "transient_poll_signal",
    "notification_privacy": "privacy_unsafe_notification",
}

CROSS_LINK_FIELDS = (
    "install_topology_ref",
    "embedded_boundary_ref",
    "activity_center_ref",
    "auth_recovery_ref",
    "channel_ownership_ref",
    "protocol_handler_ownership_ref",
    "file_association_ownership_ref",
)

EXPECTED_RECORD_KIND_REPORT = "shell_m5_native_desktop_matrix_report_record"
EXPECTED_RECORD_KIND_ROW = "shell_m5_native_desktop_entry_record"
EXPECTED_RECORD_KIND_SUPPORT = "shell_m5_native_desktop_support_export_record"
EXPECTED_SHARED_CONTRACT_REF = "shell:m5_native_desktop:v1"
EXPECTED_SCHEMA_VERSION = 1

DOC_BACKLINKS = (
    "artifacts/platform/m5-native-desktop-matrix.md",
    "fixtures/platform/m5_os_entry_and_reopen/report.json",
    "schemas/platform/m5-native-desktop-matrix.schema.json",
    "tools/ci/m5/native_desktop_check.py",
)


@dataclass
class Finding:
    """One blocking finding emitted by the gate."""

    code: str
    message: str
    entry_id: str | None = None
    control: str | None = None
    detail: dict[str, Any] = field(default_factory=dict)

    def as_dict(self) -> dict[str, Any]:
        out: dict[str, Any] = {"code": self.code, "message": self.message}
        if self.entry_id is not None:
            out["entry_id"] = self.entry_id
        if self.control is not None:
            out["control"] = self.control
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
    if report.get("required_surface_kinds") != list(REQUIRED_KINDS):
        findings.append(
            Finding(
                "required_surface_kinds_mismatch",
                "report.required_surface_kinds must equal the canonical kind list",
                detail={"declared": report.get("required_surface_kinds")},
            )
        )
    if report.get("required_controls") != list(REQUIRED_CONTROLS):
        findings.append(
            Finding(
                "required_controls_mismatch",
                "report.required_controls must equal the canonical control list",
                detail={"declared": report.get("required_controls")},
            )
        )
    platforms = report.get("claimed_platforms")
    if not isinstance(platforms, list) or not platforms:
        findings.append(
            Finding(
                "claimed_platforms_missing",
                "report.claimed_platforms must be a non-empty array",
                detail={"claimed_platforms": platforms},
            )
        )
    for ref_field in ("published_report_ref", "published_doc_ref", "review_packet_ref"):
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


def check_cross_links(report: dict[str, Any], findings: list[Finding]) -> None:
    cross_links = report.get("cross_links")
    if not isinstance(cross_links, dict):
        findings.append(Finding("cross_links_missing", "report.cross_links must be an object"))
        return
    for field_name in CROSS_LINK_FIELDS:
        value = cross_links.get(field_name)
        if not isinstance(value, str) or not value.strip():
            findings.append(
                Finding(
                    "cross_link_missing",
                    "report.cross_links must carry every upstream packet ref",
                    detail={"field": field_name},
                )
            )


def check_required_coverage(report: dict[str, Any], findings: list[Finding]) -> None:
    entries = ensure_list(report.get("entries", []), "report.entries")
    present_kinds = {
        ensure_dict(entry.get("descriptor", {}), "entry.descriptor").get("surface_kind")
        for entry in entries
    }
    for kind in REQUIRED_KINDS:
        if kind not in present_kinds:
            findings.append(
                Finding("required_kind_missing", "no registered surface for required kind", detail={"kind": kind})
            )
    for control in REQUIRED_CONTROLS:
        any_satisfied = any(
            binding.get("control") == control and binding.get("status") == "satisfied"
            for entry in entries
            for binding in ensure_list(entry.get("bindings", []), "entry.bindings")
        )
        if not any_satisfied:
            findings.append(
                Finding("required_control_not_satisfied", "no satisfied surface for required control", control=control)
            )


def check_binding(entry_id: str, binding: dict[str, Any], findings: list[Finding]) -> None:
    control = binding.get("control")
    status = binding.get("status")

    if status == "failed":
        findings.append(
            Finding(
                "control_failed",
                "control carries a failed status",
                entry_id,
                control,
                detail={"failure_mode": binding.get("failure_mode")},
            )
        )
        if binding.get("failure_mode") != CANONICAL_FAILURE_MODE.get(control):
            findings.append(
                Finding("failure_mode_drift", "failed control declares the wrong failure mode", entry_id, control)
            )
    elif status == "satisfied":
        if binding.get("failure_mode") is not None:
            findings.append(
                Finding("failure_mode_drift", "satisfied control declares a failure mode", entry_id, control)
            )
        if not binding.get("evidence_pack_ref"):
            findings.append(
                Finding("missing_evidence_pack", "satisfied control is missing an evidence pack", entry_id, control)
            )
        if control in RECOVERY_CONTROLS and not binding.get("recovery_path_ref"):
            findings.append(
                Finding("missing_recovery_path", "satisfied recovery control is missing a recovery path", entry_id, control)
            )
        if control in DURABLE_OBJECT_CONTROLS and not binding.get("durable_object_ref"):
            findings.append(
                Finding("missing_durable_object", "satisfied signal control is missing a durable object", entry_id, control)
            )
    elif status in ("not_applicable", "explicitly_narrowed"):
        reason = binding.get("narrowing_reason")
        if not isinstance(reason, str) or not reason.strip():
            findings.append(
                Finding("missing_narrowing_reason", "narrowed control is missing a narrowing reason", entry_id, control)
            )
    else:
        findings.append(
            Finding("unknown_status", "control carries an unknown status", entry_id, control, detail={"status": status})
        )


def check_entry(entry: dict[str, Any], findings: list[Finding]) -> None:
    descriptor = ensure_dict(entry.get("descriptor", {}), "entry.descriptor")
    entry_id = descriptor.get("entry_id")
    if not isinstance(entry_id, str) or not entry_id.strip():
        findings.append(Finding("missing_entry_id", "descriptor.entry_id must be non-empty"))
        return

    if entry.get("record_kind") != EXPECTED_RECORD_KIND_ROW:
        findings.append(
            Finding(
                "entry_record_kind_mismatch",
                f"entry.record_kind must be {EXPECTED_RECORD_KIND_ROW}",
                entry_id=entry_id,
                detail={"record_kind": entry.get("record_kind")},
            )
        )

    required_string_fields = {
        "descriptor_revision_ref": "missing_descriptor_revision",
        "channel_build_owner_ref": "missing_channel_build_owner",
        "trust_checkpoint_ref": "missing_trust_checkpoint",
        "reopen_anchor_ref": "missing_reopen_anchor",
        "continuity_note": "missing_continuity_note",
        "downgrade_rule_ref": "missing_downgrade_rule",
    }
    for field_name, code in required_string_fields.items():
        value = descriptor.get(field_name)
        if not isinstance(value, str) or not value.strip():
            findings.append(Finding(code, f"descriptor.{field_name} must be non-empty", entry_id=entry_id))

    vocab = descriptor.get("degraded_state_vocabulary")
    if not isinstance(vocab, list) or not [phrase for phrase in vocab if isinstance(phrase, str) and phrase.strip()]:
        findings.append(
            Finding("missing_degraded_state_vocabulary", "descriptor.degraded_state_vocabulary must be non-empty", entry_id=entry_id)
        )

    platforms = descriptor.get("claimed_platforms")
    if not isinstance(platforms, list) or not platforms:
        findings.append(Finding("missing_claimed_platforms", "descriptor.claimed_platforms must be non-empty", entry_id=entry_id))

    if descriptor.get("registered_on_native_desktop_harness") is not True:
        findings.append(
            Finding("surface_not_on_harness", "descriptor.registered_on_native_desktop_harness must be true", entry_id=entry_id)
        )

    if descriptor.get("marketed") and descriptor.get("evidence_freshness") == "stale":
        findings.append(
            Finding("stale_evidence_on_marketed_surface", "marketed surface carries stale evidence", entry_id=entry_id)
        )

    bindings = ensure_list(entry.get("bindings", []), "entry.bindings")
    present_controls = {binding.get("control") for binding in bindings}
    for control in REQUIRED_CONTROLS:
        if control not in present_controls:
            findings.append(
                Finding("missing_required_control", "surface is missing a required control binding", entry_id=entry_id, control=control)
            )
    for binding in bindings:
        check_binding(entry_id, binding, findings)

    for blocker in ensure_list(entry.get("blocking_findings", []), "entry.blocking_findings"):
        findings.append(
            Finding(
                "blocking_finding_present",
                "surface carries a blocking finding",
                entry_id=entry_id,
                control=blocker.get("control"),
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
            findings.append(Finding("doc_missing_kind", "companion doc must quote every required surface kind", detail={"kind": kind}))
    for control in REQUIRED_CONTROLS:
        if control not in body:
            findings.append(Finding("doc_missing_control", "companion doc must quote every required control", control=control))
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
    check_publications(repo_root, findings)

    if args.format == "json":
        print(json.dumps({"findings": [f.as_dict() for f in findings]}, indent=2))
    else:
        if not findings:
            print("m5 native-desktop matrix: clean")
        else:
            for finding in findings:
                location = finding.entry_id or "report"
                if finding.control:
                    location = f"{location} / {finding.control}"
                print(f"FAIL [{finding.code}] {location}: {finding.message}")

    return 1 if findings else 0


if __name__ == "__main__":
    sys.exit(main())
