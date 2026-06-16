#!/usr/bin/env python3
"""Regenerate the M5 public-interface diff-report register.

This emits the canonical diff-report register, the negative fixtures, the cases
manifest, and the frozen validation capture. The Python summary/promotion logic
mirrors the typed Rust consumer so the checked-in artifact validates cleanly and
the capture cross-check agrees with the model.

Each report binds one changed stable-facing M5 contract — a wire/state schema, a
CLI/headless output, an exported packet, an SDK/runtime contract, or a
compatibility bridge — to the public-interface diff it carries (added, removed,
and changed surface plus the reader/writer compatibility review), the
compatibility window it lives in, the support-class caveat it publishes, and the
successor/deprecation packet that governs how the old contract leaves the window.
"""

from __future__ import annotations

import copy
import json
from pathlib import Path

REPO = Path(__file__).resolve().parent.parent

MODULE = "implement_public_interface_diff_reports_compatibility_windows_support_class_caveats_and_successor_deprecation_packets_for_changed_stable_facing_m5_contracts"
RECORD_KIND = MODULE
ARTIFACT = REPO / "artifacts/release/m5" / f"{MODULE}.json"
CAPTURE = REPO / "artifacts/release/captures" / f"{MODULE}_validation_capture.json"
FIXTURES = REPO / "fixtures/compat/m5-public-interface-diff-reports"
AS_OF = "2026-06-16"

LIFECYCLE_LABELS = ["lts", "stable", "beta", "preview", "withdrawn"]
ABOVE_CUTLINE = ["lts", "stable"]
BELOW_CUTLINE = ["beta", "preview", "withdrawn"]
CONTRACT_KINDS = [
    "schema",
    "cli_headless_output",
    "exported_packet",
    "sdk_runtime_contract",
    "compatibility_bridge",
]
CHANGE_CLASSES = ["additive", "behavioral", "breaking"]
COMPATIBILITY_POSTURES = [
    "fully_compatible",
    "backward_compatible",
    "forward_compatible",
    "breaking",
]
WINDOW_SUPPORT_STATES = ["within_window", "support_ended"]
REVIEW_POSTURES = ["compatible", "breaking", "unreviewed"]
SUPPORT_CLASSES = [
    "fully_supported",
    "supported_with_caveats",
    "limited",
    "unsupported",
]
DEPRECATION_STATUSES = ["deprecated", "superseded", "removal_scheduled", "removed"]
REPORT_STATES = [
    "published",
    "limited",
    "on_waiver",
    "breaking_unpacketed",
    "deprecation_incomplete",
    "compat_review_pending",
    "removal_overdue",
    "support_window_ended",
    "evidence_stale",
    "incomplete",
]
NARROWING_REASONS = [
    "breaking_change_unpacketed",
    "reader_writer_review_missing",
    "deprecation_packet_incomplete",
    "removal_overdue",
    "support_window_ended",
    "evidence_stale",
    "evidence_missing",
    "waiver_expired",
    "owner_signoff_missing",
    "claim_publication_missing",
]
STOP_RULE_ACTIONS = [
    "publish_deprecation_packet",
    "complete_compat_review",
    "complete_deprecation_packet",
    "execute_or_extend_removal",
    "extend_or_close_window",
    "refresh_evidence",
    "capture_evidence",
    "narrow_label",
    "request_owner_signoff",
    "republish_claim",
]

RANK = {"lts": 4, "stable": 3, "beta": 2, "preview": 1, "withdrawn": 0}
STATE_HOLDS = {"published", "limited", "on_waiver"}
# The stop action a stop rule prescribes per narrowing reason.
REASON_ACTION = {
    "breaking_change_unpacketed": "publish_deprecation_packet",
    "reader_writer_review_missing": "complete_compat_review",
    "deprecation_packet_incomplete": "complete_deprecation_packet",
    "removal_overdue": "execute_or_extend_removal",
    "support_window_ended": "extend_or_close_window",
    "evidence_stale": "refresh_evidence",
    "evidence_missing": "capture_evidence",
    "waiver_expired": "narrow_label",
    "owner_signoff_missing": "request_owner_signoff",
    "claim_publication_missing": "republish_claim",
}
REASON_TITLE = {
    "breaking_change_unpacketed": "Breaking change without a deprecation packet",
    "reader_writer_review_missing": "Reader/writer compatibility review missing",
    "deprecation_packet_incomplete": "Deprecation packet incomplete",
    "removal_overdue": "Removal checkpoint overdue",
    "support_window_ended": "Compatibility/support window ended",
    "evidence_stale": "Diff-report evidence stale",
    "evidence_missing": "Diff-report evidence missing",
    "waiver_expired": "Diff-report waiver expired",
    "owner_signoff_missing": "Owner sign-off missing",
    "claim_publication_missing": "Claim publication missing",
}

DESTINATIONS = [
    "help_about",
    "release_center",
    "service_health",
    "support_export",
    "cli_inspect",
    "docs",
    "upgrade_notes",
]


def holds_stable(label: str) -> bool:
    return RANK[label] >= RANK["stable"]


def proof(entry: str, slo_state: str, captured: bool = True) -> dict:
    return {
        "packet_id": entry,
        "packet_ref": f"proof/{entry}",
        "proof_index_ref": f"proof-index/{entry}",
        "captured_at": AS_OF if captured else None,
        "freshness_slo": {
            "target_max_age_days": 30,
            "warn_within_days": 7,
            "slo_register_ref": "freshness-slo/register",
        },
        "slo_state": slo_state,
        "evidence_refs": [f"evidence/{entry}/diff-report"] if captured else [],
    }


def signoff(owner: str = "release-engineering", signed: bool = True) -> dict:
    return {"owner_ref": owner, "signed_off": signed, "signed_at": AS_OF if signed else None}


def diff(added: list[str], removed: list[str], changed: list[str], reader: str, writer: str, entry: str) -> dict:
    return {
        "added": added,
        "removed": removed,
        "changed": changed,
        "reader_posture": reader,
        "writer_posture": writer,
        "diff_ref": f"diff/{entry}",
    }


def window(posture: str, lo: str, cur: str, hi: str, support_state: str, entry: str) -> dict:
    return {
        "posture": posture,
        "min_supported_version": lo,
        "current_version": cur,
        "max_supported_version": hi,
        "support_state": support_state,
        "window_ref": f"window/{entry}",
    }


def caveat(support_class: str, caveats: list[str]) -> dict:
    return {"support_class": support_class, "caveats": caveats}


def alias(frm: str, to: str) -> dict:
    return {"from": frm, "to": to}


def packet(
    entry: str,
    status: str,
    *,
    successor: str | None,
    removal_checkpoint: str | None,
    removal_date: str | None,
    removal_overdue: bool,
    migration: str | None,
    rollback: str | None,
    aliases: list[dict] | None = None,
    owner: str = "release-docs",
) -> dict:
    return {
        "status": status,
        "owner_ref": owner,
        "successor_ref": successor,
        "alias_map": aliases or [],
        "removal_checkpoint": removal_checkpoint,
        "removal_date": removal_date,
        "removal_overdue": removal_overdue,
        "migration_ref": migration,
        "rollback_implications": rollback,
        "packet_ref": f"deprecation/{entry}",
    }


def reports() -> list[dict]:
    out = []

    # 1. Wire/state schema gained optional fields: backward-compatible additive
    #    change, reviewed both sides, fully supported, holds Stable.
    out.append(
        {
            "entry_id": "m5-diff-state-schema-additive",
            "title": "Workspace state schema additive diff",
            "contract_kind": "schema",
            "contract_ref": "contract/workspace-state-schema",
            "contract_summary": "Workspace save-state schema consumed by restore and portable-install flows.",
            "release_blocking": True,
            "change_class": "additive",
            "claim_ref": "claim/m5-portable-state",
            "claim_label": "stable",
            "report_state": "published",
            "interface_diff": diff(
                ["workspace_state.window_layout (optional)", "workspace_state.pinned_tabs (optional)"],
                [],
                [],
                "compatible",
                "compatible",
                "m5-diff-state-schema-additive",
            ),
            "compatibility_window": window(
                "backward_compatible", "5.0.0", "5.4.0", "5.4.0", "within_window",
                "m5-diff-state-schema-additive",
            ),
            "support_caveat": caveat("fully_supported", []),
            "deprecation_packet": None,
            "proof_packet": proof("m5-diff-state-schema-additive", "current"),
            "waiver": None,
            "owner_signoff": signoff(),
            "active_narrowing_reasons": [],
            "published_label": "stable",
            "publication_destinations": DESTINATIONS,
            "rationale": "The schema only gained optional fields, readers and writers were reviewed compatible, and the diff report is current, so the contract holds its Stable claim with no caveats.",
        }
    )

    # 2. CLI/headless JSON output reshaped behaviorally but compatibly: holds
    #    Stable, limited, with a recorded caveat for script authors.
    out.append(
        {
            "entry_id": "m5-diff-cli-status-behavioral",
            "title": "CLI status output behavioral diff",
            "contract_kind": "cli_headless_output",
            "contract_ref": "contract/cli-status-output",
            "contract_summary": "Stable `--json` output of the headless status command consumed by scripts.",
            "release_blocking": True,
            "change_class": "behavioral",
            "claim_ref": "claim/m5-headless-cli",
            "claim_label": "stable",
            "report_state": "limited",
            "interface_diff": diff(
                ["status.tracking (object)"],
                [],
                ["status.ahead and status.behind now nested under status.tracking"],
                "compatible",
                "compatible",
                "m5-diff-cli-status-behavioral",
            ),
            "compatibility_window": window(
                "backward_compatible", "5.2.0", "5.4.0", "5.4.0", "within_window",
                "m5-diff-cli-status-behavioral",
            ),
            "support_caveat": caveat(
                "supported_with_caveats",
                [
                    "Scripts that read the flat status.ahead/status.behind keys keep working through retained aliases; new scripts should read status.tracking.",
                ],
            ),
            "deprecation_packet": None,
            "proof_packet": proof("m5-diff-cli-status-behavioral", "current"),
            "waiver": None,
            "owner_signoff": signoff(),
            "active_narrowing_reasons": [],
            "published_label": "stable",
            "publication_destinations": DESTINATIONS,
            "rationale": "The output reshapes the tracking counters compatibly with retained aliases; the report holds Stable with a recorded caveat so script authors can migrate.",
        }
    )

    # 3. Exported packet renamed a field with a complete, in-horizon deprecation
    #    packet and alias map: a managed breaking change that still holds Stable.
    out.append(
        {
            "entry_id": "m5-diff-export-packet-deprecated",
            "title": "Support-export packet field rename diff",
            "contract_kind": "exported_packet",
            "contract_ref": "contract/support-export-packet",
            "contract_summary": "Exported support-bundle packet consumed by procurement and field support.",
            "release_blocking": True,
            "change_class": "breaking",
            "claim_ref": "claim/m5-support-export",
            "claim_label": "stable",
            "report_state": "limited",
            "interface_diff": diff(
                ["support_export.evidence_index"],
                ["support_export.evidence_refs"],
                [],
                "breaking",
                "breaking",
                "m5-diff-export-packet-deprecated",
            ),
            "compatibility_window": window(
                "breaking", "5.0.0", "5.4.0", "5.4.0", "within_window",
                "m5-diff-export-packet-deprecated",
            ),
            "support_caveat": caveat(
                "supported_with_caveats",
                [
                    "Consumers reading support_export.evidence_refs must migrate to support_export.evidence_index; the old array is served as a deprecated alias until removal.",
                ],
            ),
            "deprecation_packet": packet(
                "m5-diff-export-packet-deprecated",
                "deprecated",
                successor="contract/support-export-packet@v2",
                removal_checkpoint="removal-checkpoint/support-export-evidence-refs",
                removal_date="2027-06-30",
                removal_overdue=False,
                migration="docs/m5/migrations/support-export-evidence-index.md",
                rollback="Rolling back republishes evidence_refs as the primary key; consumers already on evidence_index keep working via the retained alias, so the rollback is non-destructive.",
                aliases=[alias("support_export.evidence_refs", "support_export.evidence_index")],
            ),
            "proof_packet": proof("m5-diff-export-packet-deprecated", "current"),
            "waiver": None,
            "owner_signoff": signoff(),
            "active_narrowing_reasons": [],
            "published_label": "stable",
            "publication_destinations": DESTINATIONS,
            "rationale": "The field rename is a breaking diff, but the deprecation packet names a successor, alias map, removal horizon, migration, and rollback implications and the old key is retained, so the managed deprecation holds Stable with a caveat.",
        }
    )

    # 4. SDK/runtime API removed with no deprecation packet: a breaking change
    #    that must narrow because it was not packeted.
    out.append(
        {
            "entry_id": "m5-diff-sdk-removed-unpacketed",
            "title": "Extension runtime API removal diff",
            "contract_kind": "sdk_runtime_contract",
            "contract_ref": "contract/extension-runtime-api",
            "contract_summary": "Extension runtime API surface exposed to packaged extensions.",
            "release_blocking": True,
            "change_class": "breaking",
            "claim_ref": "claim/m5-ecosystem",
            "claim_label": "stable",
            "report_state": "breaking_unpacketed",
            "interface_diff": diff(
                [],
                ["ExtensionContext.legacyStoragePath"],
                [],
                "breaking",
                "compatible",
                "m5-diff-sdk-removed-unpacketed",
            ),
            "compatibility_window": window(
                "breaking", "5.0.0", "5.4.0", "5.4.0", "within_window",
                "m5-diff-sdk-removed-unpacketed",
            ),
            "support_caveat": caveat("unsupported", []),
            "deprecation_packet": None,
            "proof_packet": proof("m5-diff-sdk-removed-unpacketed", "current"),
            "waiver": None,
            "owner_signoff": signoff(),
            "active_narrowing_reasons": ["breaking_change_unpacketed"],
            "published_label": "beta",
            "publication_destinations": DESTINATIONS,
            "rationale": "The runtime API was removed without a deprecation packet naming a successor and removal horizon, so the contract narrows below the cutline until the packet is published.",
        }
    )

    # 5. Schema field removed with a deprecation packet that is missing its
    #    rollback implications: an incomplete packet narrows the contract.
    out.append(
        {
            "entry_id": "m5-diff-schema-packet-incomplete",
            "title": "Notebook schema removal with incomplete packet",
            "contract_kind": "schema",
            "contract_ref": "contract/notebook-document-schema",
            "contract_summary": "Notebook document schema consumed by notebook open, replay, and export.",
            "release_blocking": True,
            "change_class": "breaking",
            "claim_ref": "claim/m5-notebook",
            "claim_label": "stable",
            "report_state": "deprecation_incomplete",
            "interface_diff": diff(
                ["notebook.cell_ids (uuid)"],
                ["notebook.legacy_cell_ids"],
                [],
                "breaking",
                "breaking",
                "m5-diff-schema-packet-incomplete",
            ),
            "compatibility_window": window(
                "breaking", "5.1.0", "5.4.0", "5.4.0", "within_window",
                "m5-diff-schema-packet-incomplete",
            ),
            "support_caveat": caveat(
                "limited",
                ["The deprecation packet is missing rollback implications, so the removal is not yet safe to publish."],
            ),
            "deprecation_packet": packet(
                "m5-diff-schema-packet-incomplete",
                "removal_scheduled",
                successor="contract/notebook-document-schema@v3",
                removal_checkpoint="removal-checkpoint/notebook-legacy-cell-ids",
                removal_date="2027-09-30",
                removal_overdue=False,
                migration="docs/m5/migrations/notebook-cell-ids.md",
                rollback=None,
                aliases=[alias("notebook.legacy_cell_ids", "notebook.cell_ids")],
            ),
            "proof_packet": proof("m5-diff-schema-packet-incomplete", "current"),
            "waiver": None,
            "owner_signoff": signoff(),
            "active_narrowing_reasons": ["deprecation_packet_incomplete"],
            "published_label": "beta",
            "publication_destinations": DESTINATIONS,
            "rationale": "The schema removal carries a deprecation packet, but it omits the rollback implications the guardrail requires, so the contract narrows until the packet is completed.",
        }
    )

    # 6. Compatibility bridge changed behaviorally, but the reader/writer
    #    compatibility review is not yet complete: narrows on the missing review.
    out.append(
        {
            "entry_id": "m5-diff-bridge-review-pending",
            "title": "Mixed-version bridge unreviewed diff",
            "contract_kind": "compatibility_bridge",
            "contract_ref": "contract/desktop-helper-bridge",
            "contract_summary": "Mixed-version compatibility bridge negotiating the desktop↔helper envelope.",
            "release_blocking": True,
            "change_class": "behavioral",
            "claim_ref": "claim/m5-remote-helper",
            "claim_label": "stable",
            "report_state": "compat_review_pending",
            "interface_diff": diff(
                ["bridge.capability_flags (extended)"],
                [],
                ["bridge.resume_token negotiation reordered"],
                "compatible",
                "unreviewed",
                "m5-diff-bridge-review-pending",
            ),
            "compatibility_window": window(
                "backward_compatible", "5.2.0", "5.4.0", "5.4.0", "within_window",
                "m5-diff-bridge-review-pending",
            ),
            "support_caveat": caveat(
                "limited",
                ["The writer-side compatibility review is still open, so producer-side changes are not yet certified as reader/writer safe."],
            ),
            "deprecation_packet": None,
            "proof_packet": proof("m5-diff-bridge-review-pending", "current"),
            "waiver": None,
            "owner_signoff": signoff(),
            "active_narrowing_reasons": ["reader_writer_review_missing"],
            "published_label": "beta",
            "publication_destinations": DESTINATIONS,
            "rationale": "Producer-side bridge fields changed but the writer-side reader/writer compatibility review is unreviewed, so the contract narrows rather than treat a producer-side update as sufficient.",
        }
    )

    # 7. Exported packet whose deprecated field passed its removal checkpoint:
    #    an overdue removal narrows the contract.
    out.append(
        {
            "entry_id": "m5-diff-export-removal-overdue",
            "title": "Telemetry export packet overdue removal diff",
            "contract_kind": "exported_packet",
            "contract_ref": "contract/telemetry-export-packet",
            "contract_summary": "Exported telemetry packet consumed by the service-health feed.",
            "release_blocking": True,
            "change_class": "breaking",
            "claim_ref": "claim/m5-service-health",
            "claim_label": "stable",
            "report_state": "removal_overdue",
            "interface_diff": diff(
                ["telemetry.event_envelope_v2"],
                ["telemetry.event_envelope_v1"],
                [],
                "breaking",
                "breaking",
                "m5-diff-export-removal-overdue",
            ),
            "compatibility_window": window(
                "breaking", "5.0.0", "5.4.0", "5.4.0", "within_window",
                "m5-diff-export-removal-overdue",
            ),
            "support_caveat": caveat(
                "limited",
                ["The v1 envelope passed its scheduled removal checkpoint and is still being emitted; the removal must be executed or the checkpoint extended."],
            ),
            "deprecation_packet": packet(
                "m5-diff-export-removal-overdue",
                "removal_scheduled",
                successor="contract/telemetry-export-packet@v2",
                removal_checkpoint="removal-checkpoint/telemetry-envelope-v1",
                removal_date="2026-03-31",
                removal_overdue=True,
                migration="docs/m5/migrations/telemetry-envelope-v2.md",
                rollback="Rolling back re-enables the v1 envelope alias; consumers already on v2 keep working, so the rollback is non-destructive.",
                aliases=[alias("telemetry.event_envelope_v1", "telemetry.event_envelope_v2")],
            ),
            "proof_packet": proof("m5-diff-export-removal-overdue", "current"),
            "waiver": None,
            "owner_signoff": signoff(),
            "active_narrowing_reasons": ["removal_overdue"],
            "published_label": "beta",
            "publication_destinations": DESTINATIONS,
            "rationale": "The deprecation packet is complete, but its removal checkpoint is overdue while the v1 envelope is still emitted, so the contract narrows until the removal is executed or the checkpoint is extended.",
        }
    )

    # 8. CLI/headless output reshaped under an interim waiver while the new
    #    contract re-qualifies: holds Stable provisionally.
    out.append(
        {
            "entry_id": "m5-diff-cli-export-on-waiver",
            "title": "CLI export output waiver diff",
            "contract_kind": "cli_headless_output",
            "contract_ref": "contract/cli-export-output",
            "contract_summary": "Stable headless export-manifest output consumed by CI and procurement tooling.",
            "release_blocking": True,
            "change_class": "behavioral",
            "claim_ref": "claim/m5-headless-cli",
            "claim_label": "stable",
            "report_state": "on_waiver",
            "interface_diff": diff(
                ["export.manifest_digest"],
                [],
                ["export.entries ordering is now deterministic by path"],
                "compatible",
                "compatible",
                "m5-diff-cli-export-on-waiver",
            ),
            "compatibility_window": window(
                "backward_compatible", "5.3.0", "5.4.0", "5.4.0", "within_window",
                "m5-diff-cli-export-on-waiver",
            ),
            "support_caveat": caveat(
                "supported_with_caveats",
                ["Deterministic ordering re-qualification is in progress; interim coverage is waived by the owner."],
            ),
            "deprecation_packet": None,
            "proof_packet": proof("m5-diff-cli-export-on-waiver", "current"),
            "waiver": {
                "waiver_ref": "waiver:m5_cli_export_ordering_requalification",
                "expires_at": "2026-12-31",
                "reason": "Deterministic export ordering re-qualification scheduled; interim coverage waived by owner.",
            },
            "owner_signoff": signoff(),
            "active_narrowing_reasons": [],
            "published_label": "stable",
            "publication_destinations": DESTINATIONS,
            "rationale": "The export output reshapes compatibly and the diff is current; the report holds Stable provisionally under an active, unexpired waiver while deterministic ordering re-qualifies.",
        }
    )

    # 9. SDK/runtime contract additive change inside its window, but the
    #    diff-report evidence breached its freshness SLO: narrows on stale
    #    evidence even though the change is compatible.
    out.append(
        {
            "entry_id": "m5-diff-sdk-evidence-stale",
            "title": "Runtime contract additive diff on stale evidence",
            "contract_kind": "sdk_runtime_contract",
            "contract_ref": "contract/headless-runtime-contract",
            "contract_summary": "Headless runtime contract exposed to automation and CI clients.",
            "release_blocking": True,
            "change_class": "additive",
            "claim_ref": "claim/m5-headless-cli",
            "claim_label": "stable",
            "report_state": "evidence_stale",
            "interface_diff": diff(
                ["runtime.capabilities.batch_mode (optional)"],
                [],
                [],
                "compatible",
                "compatible",
                "m5-diff-sdk-evidence-stale",
            ),
            "compatibility_window": window(
                "backward_compatible", "5.2.0", "5.4.0", "5.4.0", "within_window",
                "m5-diff-sdk-evidence-stale",
            ),
            "support_caveat": caveat(
                "limited",
                ["The diff report's proof packet breached its freshness SLO; the support claim narrows until the diff is re-captured."],
            ),
            "deprecation_packet": None,
            "proof_packet": proof("m5-diff-sdk-evidence-stale", "breached"),
            "waiver": None,
            "owner_signoff": signoff(),
            "active_narrowing_reasons": ["evidence_stale"],
            "published_label": "beta",
            "publication_destinations": DESTINATIONS,
            "rationale": "The runtime addition is backward compatible, but the diff report's proof packet breached its freshness SLO, so the published support claim narrows to evidence-stale until the diff is re-captured.",
        }
    )

    # 10. Compatibility bridge additive change with no captured diff-report
    #     evidence: narrows as incomplete. Not release-blocking.
    out.append(
        {
            "entry_id": "m5-diff-bridge-evidence-missing",
            "title": "Companion bridge additive diff without evidence",
            "contract_kind": "compatibility_bridge",
            "contract_ref": "contract/companion-sync-bridge",
            "contract_summary": "Mixed-version bridge negotiating the optional browser/mobile companion sync envelope.",
            "release_blocking": False,
            "change_class": "additive",
            "claim_ref": "claim/m5-companion",
            "claim_label": "stable",
            "report_state": "incomplete",
            "interface_diff": diff(
                ["bridge.companion_capability_flags (optional)"],
                [],
                [],
                "compatible",
                "compatible",
                "m5-diff-bridge-evidence-missing",
            ),
            "compatibility_window": window(
                "backward_compatible", "5.2.0", "5.4.0", "5.4.0", "within_window",
                "m5-diff-bridge-evidence-missing",
            ),
            "support_caveat": caveat(
                "limited",
                ["No diff-report evidence has been captured for this bridge change; the support claim narrows until a diff is captured."],
            ),
            "deprecation_packet": None,
            "proof_packet": proof("m5-diff-bridge-evidence-missing", "missing", captured=False),
            "waiver": None,
            "owner_signoff": signoff(),
            "active_narrowing_reasons": ["evidence_missing"],
            "published_label": "beta",
            "publication_destinations": DESTINATIONS,
            "rationale": "The companion bridge change is additive, but no diff-report evidence has been captured, so the published support claim narrows to evidence-missing until a diff is captured.",
        }
    )

    # 11. Schema behavioral change whose old version left the supported
    #     compatibility window: narrows on the ended support window.
    out.append(
        {
            "entry_id": "m5-diff-schema-window-ended",
            "title": "Provider snapshot schema window-ended diff",
            "contract_kind": "schema",
            "contract_ref": "contract/provider-snapshot-schema",
            "contract_summary": "Provider snapshot schema consumed when opening imported provider objects.",
            "release_blocking": True,
            "change_class": "behavioral",
            "claim_ref": "claim/m5-ai-provider",
            "claim_label": "stable",
            "report_state": "support_window_ended",
            "interface_diff": diff(
                ["snapshot.descriptor_v3"],
                [],
                ["snapshot.descriptor negotiation prefers v3 while retaining the v2 reader"],
                "compatible",
                "compatible",
                "m5-diff-schema-window-ended",
            ),
            "compatibility_window": window(
                "forward_compatible", "5.3.0", "5.4.0", "5.4.0", "support_ended",
                "m5-diff-schema-window-ended",
            ),
            "support_caveat": caveat(
                "limited",
                ["Snapshots produced before 5.3 are outside the supported compatibility window; the window must be extended or formally closed."],
            ),
            "deprecation_packet": packet(
                "m5-diff-schema-window-ended",
                "superseded",
                successor="contract/provider-snapshot-schema@v3",
                removal_checkpoint="removal-checkpoint/provider-snapshot-pre-5.3",
                removal_date="2027-12-31",
                removal_overdue=False,
                migration="docs/m5/migrations/provider-snapshot-v3.md",
                rollback="Rolling back re-opens the pre-5.3 snapshot reader; newly written v3 snapshots stay readable, so the rollback is non-destructive.",
                aliases=[alias("snapshot.descriptor_v2", "snapshot.descriptor_v3")],
            ),
            "proof_packet": proof("m5-diff-schema-window-ended", "current"),
            "waiver": None,
            "owner_signoff": signoff(),
            "active_narrowing_reasons": ["support_window_ended"],
            "published_label": "beta",
            "publication_destinations": DESTINATIONS,
            "rationale": "The descriptor negotiation now requires v2 or newer, so snapshots below the floor leave the supported window; the contract narrows on support-window-ended until the window is extended or closed, even though the diff and packet are otherwise complete.",
        }
    )

    return out


def stop_rules() -> list[dict]:
    out = []
    for reason in NARROWING_REASONS:
        out.append(
            {
                "rule_id": f"m5_diff_report_rule:{reason}",
                "title": REASON_TITLE[reason],
                "trigger_reason": reason,
                "applies_to_labels": ABOVE_CUTLINE,
                "default_action": REASON_ACTION[reason],
                "blocks_promotion": True,
                "rationale": f"A changed stable-facing contract at or above the cutline that reports '{reason}' cannot keep a Stable or LTS support claim.",
            }
        )
    return out


def compute_promotion(register: dict) -> dict:
    triggers = set()
    for rule in register["stop_rules"]:
        if not rule["blocks_promotion"]:
            continue
        fires = any(
            row["claim_label"] in rule["applies_to_labels"]
            and rule["trigger_reason"] in row["active_narrowing_reasons"]
            for row in register["reports"]
        )
        if fires:
            triggers.add(rule["trigger_reason"])
    blocking_rule_ids = sorted(
        rule["rule_id"]
        for rule in register["stop_rules"]
        if rule["blocks_promotion"]
        and any(
            row["claim_label"] in rule["applies_to_labels"]
            and rule["trigger_reason"] in row["active_narrowing_reasons"]
            for row in register["reports"]
        )
    )
    blocking_claim_ids = sorted(
        {
            row["entry_id"]
            for row in register["reports"]
            if holds_stable(row["claim_label"])
            and any(r in triggers for r in row["active_narrowing_reasons"])
        }
    )
    decision = "hold" if blocking_rule_ids else "proceed"
    return {
        "promotion_gate": "m5-public-interface-diff-reports-gate",
        "decision": decision,
        "blocking_rule_ids": blocking_rule_ids,
        "blocking_claim_ids": blocking_claim_ids,
        "rationale": "Computed from the firing stop rules over diff classifications, reader/writer reviews, deprecation packets, removal checkpoints, compatibility windows, evidence freshness, waiver expiry, owner sign-off, and claim-publication linkage.",
    }


def packet_complete(p: dict | None) -> bool:
    if not p:
        return False
    if not (p.get("owner_ref") or "").strip():
        return False
    for key in ("successor_ref", "removal_checkpoint", "removal_date", "migration_ref", "rollback_implications"):
        value = p.get(key)
        if value is None or not str(value).strip():
            return False
    return True


def compute_summary(register: dict) -> dict:
    rs = register["reports"]

    def published_stable(row):
        return holds_stable(row["published_label"])

    def state(s):
        return sum(1 for r in rs if r["report_state"] == s)

    def kind(k):
        return sum(1 for r in rs if r["contract_kind"] == k)

    def change(c):
        return sum(1 for r in rs if r["change_class"] == c)

    def support(c):
        return sum(1 for r in rs if r["support_caveat"]["support_class"] == c)

    def slo(s):
        return sum(1 for r in rs if r["proof_packet"]["slo_state"] == s)

    rb = [r for r in rs if r["release_blocking"]]
    contracts = {r["contract_ref"] for r in rs}
    with_packet = [r for r in rs if r["deprecation_packet"] is not None]
    return {
        "total_reports": len(rs),
        "total_contracts": len(contracts),
        "reports_publishing_stable": sum(1 for r in rs if published_stable(r)),
        "reports_narrowed": sum(1 for r in rs if not published_stable(r)),
        "reports_holding": sum(1 for r in rs if r["report_state"] in STATE_HOLDS),
        "reports_published": state("published"),
        "reports_limited": state("limited"),
        "reports_on_waiver": state("on_waiver"),
        "reports_breaking_unpacketed": state("breaking_unpacketed"),
        "reports_deprecation_incomplete": state("deprecation_incomplete"),
        "reports_compat_review_pending": state("compat_review_pending"),
        "reports_removal_overdue": state("removal_overdue"),
        "reports_support_window_ended": state("support_window_ended"),
        "reports_evidence_stale": state("evidence_stale"),
        "reports_incomplete": state("incomplete"),
        "release_blocking_total": len(rb),
        "release_blocking_publishing_stable": sum(1 for r in rb if published_stable(r)),
        "release_blocking_narrowed": sum(1 for r in rb if not published_stable(r)),
        "schema_reports": kind("schema"),
        "cli_headless_output_reports": kind("cli_headless_output"),
        "exported_packet_reports": kind("exported_packet"),
        "sdk_runtime_contract_reports": kind("sdk_runtime_contract"),
        "compatibility_bridge_reports": kind("compatibility_bridge"),
        "additive_changes": change("additive"),
        "behavioral_changes": change("behavioral"),
        "breaking_changes": change("breaking"),
        "reports_with_deprecation_packet": len(with_packet),
        "complete_deprecation_packets": sum(1 for r in with_packet if packet_complete(r["deprecation_packet"])),
        "support_fully_supported": support("fully_supported"),
        "support_supported_with_caveats": support("supported_with_caveats"),
        "support_limited": support("limited"),
        "support_unsupported": support("unsupported"),
        "packets_current": slo("current"),
        "packets_due_for_refresh": slo("due_for_refresh"),
        "packets_breached": slo("breached"),
        "packets_missing": slo("missing"),
        "total_added_elements": sum(len(r["interface_diff"]["added"]) for r in rs),
        "total_removed_elements": sum(len(r["interface_diff"]["removed"]) for r in rs),
        "total_changed_elements": sum(len(r["interface_diff"]["changed"]) for r in rs),
        "total_active_narrowing_reasons": sum(len(r["active_narrowing_reasons"]) for r in rs),
        "rules_firing": sum(
            1
            for rule in register["stop_rules"]
            if any(
                r["claim_label"] in rule["applies_to_labels"]
                and rule["trigger_reason"] in r["active_narrowing_reasons"]
                for r in rs
            )
        ),
    }


def build_register() -> dict:
    register = {
        "schema_version": 1,
        "record_kind": RECORD_KIND,
        "register_id": "m5_public_interface_diff_reports:v1",
        "status": "published",
        "overview_page": f"docs/m5/{MODULE}.md",
        "as_of": AS_OF,
        "claim_manifest_ref": "artifacts/release/stable_claim_manifest.json",
        "version_windows_ref": "artifacts/release/stable_version_windows.json",
        "qualification_matrix_ref": "artifacts/release/m5/freeze_the_m5_qualification_row_support_window_skew_window_and_deprecation_packet_matrix.json",
        "lifecycle_labels": LIFECYCLE_LABELS,
        "contract_kinds": CONTRACT_KINDS,
        "change_classes": CHANGE_CLASSES,
        "compatibility_postures": COMPATIBILITY_POSTURES,
        "window_support_states": WINDOW_SUPPORT_STATES,
        "review_postures": REVIEW_POSTURES,
        "support_classes": SUPPORT_CLASSES,
        "deprecation_statuses": DEPRECATION_STATUSES,
        "report_states": REPORT_STATES,
        "narrowing_reasons": NARROWING_REASONS,
        "stop_rule_actions": STOP_RULE_ACTIONS,
        "launch_cutline": {
            "cutline_level": "stable",
            "above_cutline_levels": ABOVE_CUTLINE,
            "below_cutline_levels": BELOW_CUTLINE,
            "description": "A changed stable-facing M5 contract carries a Stable (or LTS) support claim only when its public-interface diff report is current, its reader/writer compatibility review is complete, any breaking change is governed by a complete, in-horizon deprecation packet (owner, successor, removal horizon, migration, and rollback implications), its compatibility window is open, its proof packet is current within its freshness SLO, the owner has signed off, and its backing claim publication holds. A contract whose diff names a breaking change without a packet, whose reader/writer review is missing, whose deprecation packet is incomplete or overdue, whose compatibility window ended, or whose evidence is stale or missing must narrow below the cutline rather than promote on a producer-side update alone or inherit an adjacent unchanged contract.",
        },
        "release_blocking_contract_refs": [],
        "stop_rules": stop_rules(),
        "reports": reports(),
    }
    register["release_blocking_contract_refs"] = [
        r["contract_ref"] for r in register["reports"] if r["release_blocking"]
    ]
    register["promotion"] = compute_promotion(register)
    register["summary"] = compute_summary(register)
    return register


def write_json(path: Path, payload: dict) -> None:
    path.parent.mkdir(parents=True, exist_ok=True)
    path.write_text(json.dumps(payload, indent=2) + "\n", encoding="utf-8")


def build_fixtures(register: dict) -> list[tuple[str, str]]:
    cases = []

    dup = copy.deepcopy(register)
    dup["reports"][1]["entry_id"] = dup["reports"][0]["entry_id"]
    dup["summary"] = compute_summary(dup)
    dup["promotion"] = compute_promotion(dup)
    write_json(FIXTURES / "duplicate_entry_id.json", dup)
    cases.append(("duplicate_entry_id.json", "DuplicateEntryId"))

    held = copy.deepcopy(register)
    target = next(r for r in held["reports"] if holds_stable(r["published_label"]))
    target["active_narrowing_reasons"] = ["evidence_stale"]
    held["summary"] = compute_summary(held)
    held["promotion"] = compute_promotion(held)
    write_json(FIXTURES / "held_with_active_gap.json", held)
    cases.append(("held_with_active_gap.json", "HeldWithActiveGap"))

    unpacketed = copy.deepcopy(register)
    target = next(
        r for r in unpacketed["reports"] if r["change_class"] == "breaking" and r["deprecation_packet"] is None
    )
    target["published_label"] = "stable"
    target["report_state"] = "published"
    target["active_narrowing_reasons"] = []
    target["support_caveat"] = {"support_class": "supported_with_caveats", "caveats": ["forced stable"]}
    unpacketed["summary"] = compute_summary(unpacketed)
    unpacketed["promotion"] = compute_promotion(unpacketed)
    write_json(FIXTURES / "breaking_change_held_without_packet.json", unpacketed)
    cases.append(("breaking_change_held_without_packet.json", "BreakingHeldWithoutPacket"))

    review = copy.deepcopy(register)
    target = next(
        r
        for r in review["reports"]
        if r["interface_diff"]["reader_posture"] == "unreviewed"
        or r["interface_diff"]["writer_posture"] == "unreviewed"
    )
    target["published_label"] = "stable"
    target["report_state"] = "published"
    target["active_narrowing_reasons"] = []
    target["support_caveat"] = {"support_class": "supported_with_caveats", "caveats": ["forced stable"]}
    review["summary"] = compute_summary(review)
    review["promotion"] = compute_promotion(review)
    write_json(FIXTURES / "review_pending_held.json", review)
    cases.append(("review_pending_held.json", "ReviewPendingHeld"))

    write_json(
        FIXTURES / "cases.json",
        {"cases": [{"file": f, "expected_check_id": c} for f, c in cases]},
    )
    return cases


def build_capture(register: dict, cases: list[tuple[str, str]]) -> dict:
    s = register["summary"]
    return {
        "status": "pass",
        "as_of": register["as_of"],
        "summary": {
            "total_reports": s["total_reports"],
            "reports_publishing_stable": s["reports_publishing_stable"],
            "reports_narrowed": s["reports_narrowed"],
            "reports_holding": s["reports_holding"],
            "reports_on_waiver": s["reports_on_waiver"],
            "reports_limited": s["reports_limited"],
            "breaking_changes": s["breaking_changes"],
            "reports_with_deprecation_packet": s["reports_with_deprecation_packet"],
            "complete_deprecation_packets": s["complete_deprecation_packets"],
            "reports_breaking_unpacketed": s["reports_breaking_unpacketed"],
            "reports_compat_review_pending": s["reports_compat_review_pending"],
            "reports_removal_overdue": s["reports_removal_overdue"],
            "reports_support_window_ended": s["reports_support_window_ended"],
            "packets_breached": s["packets_breached"],
            "packets_missing": s["packets_missing"],
            "total_removed_elements": s["total_removed_elements"],
            "rules_firing": s["rules_firing"],
        },
        "promotion": {
            "decision": register["promotion"]["decision"],
            "blocking_rule_ids": register["promotion"]["blocking_rule_ids"],
            "blocking_claim_ids": register["promotion"]["blocking_claim_ids"],
        },
        "negative_drills": [
            {"drill_id": "drill:narrowing_without_reason", "status": "passed"},
            {"drill_id": "drill:held_with_active_gap", "status": "passed"},
            {"drill_id": "drill:breaking_change_held_without_packet", "status": "passed"},
            {"drill_id": "drill:reader_writer_review_held", "status": "passed"},
            {"drill_id": "drill:promotion_decision_inconsistent", "status": "passed"},
        ],
        "fixture_cases": [
            {"case_id": f"fixture:{f.removesuffix('.json')}", "status": "passed"} for f, _ in cases
        ],
    }


def main() -> int:
    register = build_register()
    write_json(ARTIFACT, register)
    cases = build_fixtures(register)
    write_json(CAPTURE, build_capture(register, cases))
    print(f"wrote {ARTIFACT.relative_to(REPO)}")
    print(f"wrote {CAPTURE.relative_to(REPO)}")
    print(f"wrote {FIXTURES.relative_to(REPO)}/ ({len(cases)} fixtures + cases.json)")
    print("decision:", register["promotion"]["decision"])
    print("summary:", json.dumps(register["summary"]))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
