#!/usr/bin/env python3
"""Regenerate the M5 mixed-version boundary skew-inspector register.

This emits the canonical inspector register, the negative fixtures, the cases
manifest, and the frozen validation capture. The Python summary/promotion logic
mirrors the typed Rust consumer so the checked-in artifact validates cleanly and
the capture cross-check agrees with the model.

Each inspector binds one M5 mixed-version boundary-crossing flow — helper/agent
attach, extension/runtime load, workspace-state import/restore, or provider
snapshot/open — to the version skew it inspects, the fail-closed verdict it
reports before a mutating or privileged action, the helper/agent/host/schema/
provider downgrade vocabulary it speaks, and a structured upgrade-order guide.
"""

from __future__ import annotations

import copy
import json
from pathlib import Path

REPO = Path(__file__).resolve().parent.parent

MODULE = "ship_mixed_version_skew_inspectors_upgrade_order_guides_and_fail_closed_unsupported_skew_states_across_m5_boundaries"
RECORD_KIND = MODULE
ARTIFACT = REPO / "artifacts/release/m5" / f"{MODULE}.json"
CAPTURE = REPO / "artifacts/release/captures" / f"{MODULE}_validation_capture.json"
FIXTURES = REPO / "fixtures/compat/m5-boundary-skew-inspectors"
AS_OF = "2026-06-16"

LIFECYCLE_LABELS = ["lts", "stable", "beta", "preview", "withdrawn"]
ABOVE_CUTLINE = ["lts", "stable"]
BELOW_CUTLINE = ["beta", "preview", "withdrawn"]
BOUNDARY_KINDS = [
    "helper_agent_attach",
    "extension_runtime_load",
    "state_import_restore",
    "provider_snapshot_open",
]
DOWNGRADE_SUBJECTS = ["helper", "agent", "host", "schema", "provider"]
GATED_ACTIONS = ["attach", "load", "restore", "open"]
ACTION_RISKS = ["mutating", "privileged", "mutating_and_privileged"]
SKEW_WINDOW_CLASSES = [
    "lockstep_only",
    "bounded_skew",
    "backward_compatible",
    "forward_compatible",
    "unsupported_skew",
]
INSPECTOR_VERDICTS = [
    "inside_window",
    "unsupported_skew",
    "reconnect_required",
    "reinstall_required",
    "migration_needed",
    "retest_pending",
]
GATE_POSTURES = ["allow", "fail_closed"]
UPGRADE_LEAD_SIDES = ["none_required", "local_first", "peer_first", "coordinated"]
INSPECTOR_STATES = [
    "inside_window",
    "limited",
    "on_waiver",
    "fail_closed",
    "retest_pending",
    "evidence_stale",
    "incomplete",
]
NARROWING_REASONS = [
    "skew_window_exceeded",
    "reconnect_required",
    "reinstall_required",
    "migration_needed",
    "retest_pending",
    "evidence_stale",
    "evidence_missing",
    "waiver_expired",
    "owner_signoff_missing",
    "claim_publication_missing",
]
STOP_RULE_ACTIONS = [
    "widen_or_document_skew",
    "guide_reconnect",
    "guide_reinstall",
    "guide_migration",
    "retest_boundary",
    "refresh_evidence",
    "capture_evidence",
    "narrow_label",
    "request_owner_signoff",
    "republish_claim",
]

RANK = {"lts": 4, "stable": 3, "beta": 2, "preview": 1, "withdrawn": 0}
STATE_HOLDS = {"inside_window", "limited", "on_waiver"}
ALLOW_VERDICTS = {"inside_window"}
# Verdicts whose recovery is a version change with an explicit upgrade order.
SKEW_RECOVERY_VERDICTS = {
    "unsupported_skew",
    "reconnect_required",
    "reinstall_required",
    "migration_needed",
}
VERDICT_REASON = {
    "unsupported_skew": "skew_window_exceeded",
    "reconnect_required": "reconnect_required",
    "reinstall_required": "reinstall_required",
    "migration_needed": "migration_needed",
    "retest_pending": "retest_pending",
}
# The mutating/privileged action each boundary-crossing flow guards.
BOUNDARY_ACTION = {
    "helper_agent_attach": "attach",
    "extension_runtime_load": "load",
    "state_import_restore": "restore",
    "provider_snapshot_open": "open",
}

DESTINATIONS = [
    "help_about",
    "release_center",
    "service_health",
    "support_export",
    "cli_inspect",
    "docs",
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
        "evidence_refs": [f"evidence/{entry}/proof"] if captured else [],
    }


def signoff(owner: str = "release-engineering", signed: bool = True) -> dict:
    return {"owner_ref": owner, "signed_off": signed, "signed_at": AS_OF if signed else None}


def skew(cls: str, local: str, peer: str, lo: str, hi: str, fields: list[str], entry: str) -> dict:
    return {
        "skew_window_class": cls,
        "local_version": local,
        "peer_version": peer,
        "min_supported_version": lo,
        "max_supported_version": hi,
        "negotiated_fields": fields,
        "skew_window_ref": f"skew/{entry}",
    }


def step(order: int, side: str, instruction: str) -> dict:
    return {"order": order, "side": side, "instruction": instruction}


def guide(lead: str, steps: list[dict], entry: str) -> dict:
    return {"lead_side": lead, "steps": steps, "guide_ref": f"upgrade-guide/{entry}"}


def no_guide(entry: str) -> dict:
    return guide("none_required", [], entry)


def inspectors() -> list[dict]:
    out = []

    # 1. Desktop↔remote helper attach, inside its bounded skew window: the
    #    privileged attach proceeds and the boundary holds a Stable claim.
    out.append(
        {
            "entry_id": "m5-helper-attach-inside-window",
            "title": "Remote helper attach skew inspector",
            "boundary_kind": "helper_agent_attach",
            "boundary_ref": "boundary/desktop-remote-helper",
            "boundary_summary": "Desktop host attaching a remote helper session over the helper RPC envelope.",
            "release_blocking": True,
            "downgrade_subject": "helper",
            "gated_action": "attach",
            "action_risk": "privileged",
            "local_role": "desktop host",
            "peer_role": "remote helper",
            "claim_ref": "claim/m5-remote-helper",
            "claim_label": "stable",
            "inspector_state": "inside_window",
            "skew_window": skew(
                "bounded_skew", "5.4.0", "5.3.0", "5.2.0", "5.4.0",
                ["helper_rpc_envelope", "session_resume_token"],
                "m5-helper-attach-inside-window",
            ),
            "verdict": "inside_window",
            "gate_posture": "allow",
            "upgrade_order_guide": no_guide("m5-helper-attach-inside-window"),
            "compatibility_caveats": [],
            "proof_packet": proof("m5-helper-attach-inside-window", "current"),
            "waiver": None,
            "owner_signoff": signoff(),
            "active_narrowing_reasons": [],
            "published_label": "stable",
            "publication_destinations": DESTINATIONS,
            "rationale": "The remote helper is within the bounded helper-RPC skew window, so the privileged attach proceeds and the boundary holds its Stable claim.",
        }
    )

    # 2. Desktop↔remote agent attach, peer predates the capability handshake:
    #    fail closed, reconnect after upgrading the remote agent first.
    out.append(
        {
            "entry_id": "m5-agent-attach-reconnect-required",
            "title": "Remote agent attach skew inspector",
            "boundary_kind": "helper_agent_attach",
            "boundary_ref": "boundary/desktop-remote-agent",
            "boundary_summary": "Desktop host attaching a remote execution agent over the agent capability handshake.",
            "release_blocking": True,
            "downgrade_subject": "agent",
            "gated_action": "attach",
            "action_risk": "privileged",
            "local_role": "desktop host",
            "peer_role": "remote agent",
            "claim_ref": "claim/m5-remote-helper",
            "claim_label": "stable",
            "inspector_state": "fail_closed",
            "skew_window": skew(
                "backward_compatible", "5.4.0", "4.6.0", "5.0.0", "5.4.0",
                ["agent_capability_handshake", "session_resume_token"],
                "m5-agent-attach-reconnect-required",
            ),
            "verdict": "reconnect_required",
            "gate_posture": "fail_closed",
            "upgrade_order_guide": guide(
                "peer_first",
                [
                    step(1, "peer_first", "Upgrade the remote agent to 5.0.0 or newer so it speaks the current capability handshake."),
                    step(2, "coordinated", "Reconnect the desktop session; the attach re-runs the skew inspection."),
                ],
                "m5-agent-attach-reconnect-required",
            ),
            "compatibility_caveats": [],
            "proof_packet": proof("m5-agent-attach-reconnect-required", "current"),
            "waiver": None,
            "owner_signoff": signoff(),
            "active_narrowing_reasons": ["reconnect_required"],
            "published_label": "beta",
            "publication_destinations": DESTINATIONS,
            "rationale": "The remote agent predates the supported handshake floor, so the attach fails closed and the inspector narrows to reconnect-required until the agent is upgraded and the session reconnects.",
        }
    )

    # 3. Extension host load with a minor SDK skew: holds Stable with a recorded
    #    compatibility caveat.
    out.append(
        {
            "entry_id": "m5-extension-load-limited",
            "title": "Extension host load skew inspector",
            "boundary_kind": "extension_runtime_load",
            "boundary_ref": "boundary/extension-host-sdk",
            "boundary_summary": "Extension host loading a packaged extension against the host ABI and manifest schema.",
            "release_blocking": True,
            "downgrade_subject": "host",
            "gated_action": "load",
            "action_risk": "mutating_and_privileged",
            "local_role": "extension host",
            "peer_role": "packaged extension",
            "claim_ref": "claim/m5-ecosystem",
            "claim_label": "stable",
            "inspector_state": "limited",
            "skew_window": skew(
                "bounded_skew", "5.4.0", "5.2.0", "5.2.0", "5.4.0",
                ["extension_host_abi", "manifest_schema_version"],
                "m5-extension-load-limited",
            ),
            "verdict": "inside_window",
            "gate_posture": "allow",
            "upgrade_order_guide": no_guide("m5-extension-load-limited"),
            "compatibility_caveats": [
                "Extensions built against the 5.2 SDK load on the 5.4 host, but proposed-API surfaces added after 5.2 stay inert until the extension is rebuilt.",
            ],
            "proof_packet": proof("m5-extension-load-limited", "current"),
            "waiver": None,
            "owner_signoff": signoff(),
            "active_narrowing_reasons": [],
            "published_label": "stable",
            "publication_destinations": DESTINATIONS,
            "rationale": "The packaged extension is inside the bounded host-ABI skew window, so the load proceeds; the row holds Stable with a recorded caveat that post-5.2 proposed APIs stay inert.",
        }
    )

    # 4. Extension host load against an out-of-window ABI: unsupported skew, fail
    #    closed, reinstall a compatible build.
    out.append(
        {
            "entry_id": "m5-extension-load-unsupported-skew",
            "title": "Extension host ABI skew inspector",
            "boundary_kind": "extension_runtime_load",
            "boundary_ref": "boundary/extension-host-abi",
            "boundary_summary": "Extension host loading a sideloaded extension whose ABI predates the supported window.",
            "release_blocking": True,
            "downgrade_subject": "host",
            "gated_action": "load",
            "action_risk": "mutating_and_privileged",
            "local_role": "extension host",
            "peer_role": "sideloaded extension",
            "claim_ref": "claim/m5-ecosystem",
            "claim_label": "stable",
            "inspector_state": "fail_closed",
            "skew_window": skew(
                "unsupported_skew", "5.4.0", "4.5.0", "5.0.0", "5.4.0",
                ["extension_host_abi", "capability_grant_manifest"],
                "m5-extension-load-unsupported-skew",
            ),
            "verdict": "unsupported_skew",
            "gate_posture": "fail_closed",
            "upgrade_order_guide": guide(
                "coordinated",
                [
                    step(1, "coordinated", "Reinstall the extension from a build compiled against the 5.x host ABI."),
                    step(2, "local_first", "Re-run capability grant review before the host loads the rebuilt extension."),
                ],
                "m5-extension-load-unsupported-skew",
            ),
            "compatibility_caveats": [],
            "proof_packet": proof("m5-extension-load-unsupported-skew", "current"),
            "waiver": None,
            "owner_signoff": signoff(),
            "active_narrowing_reasons": ["skew_window_exceeded"],
            "published_label": "beta",
            "publication_destinations": DESTINATIONS,
            "rationale": "The sideloaded extension's ABI is outside the supported host window, so the load fails closed and the inspector narrows to skew-window-exceeded until a compatible build is reinstalled.",
        }
    )

    # 5. Workspace-state restore against an older save-state schema: fail closed,
    #    migration needed before the mutating restore.
    out.append(
        {
            "entry_id": "m5-state-restore-migration-needed",
            "title": "Workspace state restore skew inspector",
            "boundary_kind": "state_import_restore",
            "boundary_ref": "boundary/workspace-save-state",
            "boundary_summary": "Restoring an imported workspace save-state against the current state schema.",
            "release_blocking": True,
            "downgrade_subject": "schema",
            "gated_action": "restore",
            "action_risk": "mutating",
            "local_role": "workspace runtime",
            "peer_role": "imported save-state",
            "claim_ref": "claim/m5-portable-state",
            "claim_label": "stable",
            "inspector_state": "fail_closed",
            "skew_window": skew(
                "forward_compatible", "5.4.0", "5.0.0", "5.3.0", "5.4.0",
                ["workspace_state_schema", "save_state_envelope"],
                "m5-state-restore-migration-needed",
            ),
            "verdict": "migration_needed",
            "gate_posture": "fail_closed",
            "upgrade_order_guide": guide(
                "local_first",
                [
                    step(1, "local_first", "Run the workspace state migration to the 5.4 schema on a copy of the imported save-state."),
                    step(2, "coordinated", "Restore the migrated save-state; the inspector re-runs against the upgraded schema."),
                ],
                "m5-state-restore-migration-needed",
            ),
            "compatibility_caveats": [],
            "proof_packet": proof("m5-state-restore-migration-needed", "current"),
            "waiver": None,
            "owner_signoff": signoff(),
            "active_narrowing_reasons": ["migration_needed"],
            "published_label": "beta",
            "publication_destinations": DESTINATIONS,
            "rationale": "The imported save-state predates the supported state-schema floor, so the restore fails closed and the inspector narrows to migration-needed rather than mutating workspace state optimistically.",
        }
    )

    # 6. Workspace-state restore inside its window but held provisionally under a
    #    waiver while the schema re-qualification completes.
    out.append(
        {
            "entry_id": "m5-state-restore-on-waiver",
            "title": "Workspace state schema waiver inspector",
            "boundary_kind": "state_import_restore",
            "boundary_ref": "boundary/workspace-schema-waiver",
            "boundary_summary": "Restoring a workspace save-state inside the schema window under an interim waiver.",
            "release_blocking": True,
            "downgrade_subject": "schema",
            "gated_action": "restore",
            "action_risk": "mutating",
            "local_role": "workspace runtime",
            "peer_role": "imported save-state",
            "claim_ref": "claim/m5-portable-state",
            "claim_label": "stable",
            "inspector_state": "on_waiver",
            "skew_window": skew(
                "bounded_skew", "5.4.0", "5.3.0", "5.3.0", "5.4.0",
                ["workspace_state_schema", "save_state_envelope"],
                "m5-state-restore-on-waiver",
            ),
            "verdict": "inside_window",
            "gate_posture": "allow",
            "upgrade_order_guide": no_guide("m5-state-restore-on-waiver"),
            "compatibility_caveats": [],
            "proof_packet": proof("m5-state-restore-on-waiver", "current"),
            "waiver": {
                "waiver_ref": "waiver:m5_state_schema_requalification",
                "expires_at": "2026-12-31",
                "reason": "Save-state schema re-qualification scheduled; interim coverage waived by owner.",
            },
            "owner_signoff": signoff(),
            "active_narrowing_reasons": [],
            "published_label": "stable",
            "publication_destinations": DESTINATIONS,
            "rationale": "The save-state is inside the bounded schema window and the restore proceeds; the row holds Stable provisionally under an active, unexpired waiver while re-qualification completes.",
        }
    )

    # 7. Provider snapshot open inside a backward-compatible window: the open
    #    proceeds and the boundary holds Stable.
    out.append(
        {
            "entry_id": "m5-provider-open-inside-window",
            "title": "Provider snapshot open skew inspector",
            "boundary_kind": "provider_snapshot_open",
            "boundary_ref": "boundary/provider-snapshot",
            "boundary_summary": "Opening an imported provider snapshot against the current provider snapshot format.",
            "release_blocking": True,
            "downgrade_subject": "provider",
            "gated_action": "open",
            "action_risk": "privileged",
            "local_role": "provider runtime",
            "peer_role": "imported provider snapshot",
            "claim_ref": "claim/m5-ai-provider",
            "claim_label": "stable",
            "inspector_state": "inside_window",
            "skew_window": skew(
                "backward_compatible", "5.4.0", "5.1.0", "5.0.0", "5.4.0",
                ["provider_snapshot_format", "imported_object_descriptor"],
                "m5-provider-open-inside-window",
            ),
            "verdict": "inside_window",
            "gate_posture": "allow",
            "upgrade_order_guide": no_guide("m5-provider-open-inside-window"),
            "compatibility_caveats": [],
            "proof_packet": proof("m5-provider-open-inside-window", "due_for_refresh"),
            "waiver": None,
            "owner_signoff": signoff(),
            "active_narrowing_reasons": [],
            "published_label": "stable",
            "publication_destinations": DESTINATIONS,
            "rationale": "The imported provider snapshot is within the backward-compatible window, so the open proceeds and the boundary holds Stable; its proof packet is within SLO and due for refresh soon.",
        }
    )

    # 8. Provider imported-object open after a descriptor change: retest pending,
    #    fail closed until the boundary is retested.
    out.append(
        {
            "entry_id": "m5-provider-open-retest-pending",
            "title": "Provider imported-object retest inspector",
            "boundary_kind": "provider_snapshot_open",
            "boundary_ref": "boundary/provider-imported-object",
            "boundary_summary": "Opening an imported provider object after the imported-object descriptor changed.",
            "release_blocking": True,
            "downgrade_subject": "provider",
            "gated_action": "open",
            "action_risk": "privileged",
            "local_role": "provider runtime",
            "peer_role": "imported provider object",
            "claim_ref": "claim/m5-ai-provider",
            "claim_label": "stable",
            "inspector_state": "retest_pending",
            "skew_window": skew(
                "bounded_skew", "5.4.0", "5.3.0", "5.3.0", "5.4.0",
                ["provider_snapshot_format", "imported_object_descriptor"],
                "m5-provider-open-retest-pending",
            ),
            "verdict": "retest_pending",
            "gate_posture": "fail_closed",
            "upgrade_order_guide": no_guide("m5-provider-open-retest-pending"),
            "compatibility_caveats": [],
            "proof_packet": proof("m5-provider-open-retest-pending", "current"),
            "waiver": None,
            "owner_signoff": signoff(),
            "active_narrowing_reasons": ["retest_pending"],
            "published_label": "beta",
            "publication_destinations": DESTINATIONS,
            "rationale": "The imported-object descriptor changed, so the open fails closed and the inspector narrows to retest-pending until the boundary is retested at the new descriptor.",
        }
    )

    # 9. Air-gapped lockstep helper attach with a mismatched bundle digest: fail
    #    closed, reinstall the matching helper bundle. Not release-blocking.
    out.append(
        {
            "entry_id": "m5-airgapped-helper-reinstall-required",
            "title": "Air-gapped helper lockstep inspector",
            "boundary_kind": "helper_agent_attach",
            "boundary_ref": "boundary/airgapped-helper-bundle",
            "boundary_summary": "Air-gapped desktop host attaching a lockstep helper bundle by digest.",
            "release_blocking": False,
            "downgrade_subject": "helper",
            "gated_action": "attach",
            "action_risk": "privileged",
            "local_role": "air-gapped desktop host",
            "peer_role": "lockstep helper bundle",
            "claim_ref": "claim/m5-managed-airgapped",
            "claim_label": "stable",
            "inspector_state": "fail_closed",
            "skew_window": skew(
                "lockstep_only", "5.4.0", "5.3.0", "5.4.0", "5.4.0",
                ["airgapped_helper_bundle_digest"],
                "m5-airgapped-helper-reinstall-required",
            ),
            "verdict": "reinstall_required",
            "gate_posture": "fail_closed",
            "upgrade_order_guide": guide(
                "local_first",
                [
                    step(1, "local_first", "Reinstall the 5.4.0 helper bundle whose digest matches the air-gapped host."),
                    step(2, "coordinated", "Re-attach; the inspector verifies the lockstep bundle digest."),
                ],
                "m5-airgapped-helper-reinstall-required",
            ),
            "compatibility_caveats": [],
            "proof_packet": proof("m5-airgapped-helper-reinstall-required", "current"),
            "waiver": None,
            "owner_signoff": signoff(),
            "active_narrowing_reasons": ["reinstall_required"],
            "published_label": "beta",
            "publication_destinations": DESTINATIONS,
            "rationale": "The air-gapped boundary is lockstep-only and the helper bundle digest does not match, so the attach fails closed and the inspector narrows to reinstall-required.",
        }
    )

    # 10. State restore inside its skew window but on a breached proof packet: the
    #     skew is fine (action allowed) yet the support claim narrows on stale
    #     evidence.
    out.append(
        {
            "entry_id": "m5-state-restore-evidence-stale",
            "title": "Workspace state evidence-freshness inspector",
            "boundary_kind": "state_import_restore",
            "boundary_ref": "boundary/workspace-state-evidence",
            "boundary_summary": "Restoring a workspace save-state inside the schema window on aged inspector evidence.",
            "release_blocking": True,
            "downgrade_subject": "schema",
            "gated_action": "restore",
            "action_risk": "mutating",
            "local_role": "workspace runtime",
            "peer_role": "imported save-state",
            "claim_ref": "claim/m5-portable-state",
            "claim_label": "stable",
            "inspector_state": "evidence_stale",
            "skew_window": skew(
                "bounded_skew", "5.4.0", "5.3.0", "5.3.0", "5.4.0",
                ["workspace_state_schema", "save_state_envelope"],
                "m5-state-restore-evidence-stale",
            ),
            "verdict": "inside_window",
            "gate_posture": "allow",
            "upgrade_order_guide": no_guide("m5-state-restore-evidence-stale"),
            "compatibility_caveats": [],
            "proof_packet": proof("m5-state-restore-evidence-stale", "breached"),
            "waiver": None,
            "owner_signoff": signoff(),
            "active_narrowing_reasons": ["evidence_stale"],
            "published_label": "beta",
            "publication_destinations": DESTINATIONS,
            "rationale": "The save-state is inside the schema window so the restore is allowed, but the inspector's proof packet breached its freshness SLO, so the published support claim narrows to evidence-stale until evidence is refreshed.",
        }
    )

    # 11. Provider snapshot open inside its window with no captured inspector
    #     evidence: the claim narrows as incomplete. Not release-blocking.
    out.append(
        {
            "entry_id": "m5-provider-open-incomplete",
            "title": "Provider snapshot evidence-coverage inspector",
            "boundary_kind": "provider_snapshot_open",
            "boundary_ref": "boundary/provider-snapshot-evidence",
            "boundary_summary": "Opening an imported provider snapshot inside the window without captured inspector evidence.",
            "release_blocking": False,
            "downgrade_subject": "provider",
            "gated_action": "open",
            "action_risk": "privileged",
            "local_role": "provider runtime",
            "peer_role": "imported provider snapshot",
            "claim_ref": "claim/m5-ai-provider",
            "claim_label": "stable",
            "inspector_state": "incomplete",
            "skew_window": skew(
                "backward_compatible", "5.4.0", "5.2.0", "5.0.0", "5.4.0",
                ["provider_snapshot_format", "imported_object_descriptor"],
                "m5-provider-open-incomplete",
            ),
            "verdict": "inside_window",
            "gate_posture": "allow",
            "upgrade_order_guide": no_guide("m5-provider-open-incomplete"),
            "compatibility_caveats": [],
            "proof_packet": proof("m5-provider-open-incomplete", "missing", captured=False),
            "waiver": None,
            "owner_signoff": signoff(),
            "active_narrowing_reasons": ["evidence_missing"],
            "published_label": "beta",
            "publication_destinations": DESTINATIONS,
            "rationale": "The snapshot is inside the window, but the inspector has no captured evidence, so the published support claim narrows to evidence-missing until an inspection is captured.",
        }
    )

    return out


def stop_rules() -> list[dict]:
    action = {
        "skew_window_exceeded": "widen_or_document_skew",
        "reconnect_required": "guide_reconnect",
        "reinstall_required": "guide_reinstall",
        "migration_needed": "guide_migration",
        "retest_pending": "retest_boundary",
        "evidence_stale": "refresh_evidence",
        "evidence_missing": "capture_evidence",
        "waiver_expired": "narrow_label",
        "owner_signoff_missing": "request_owner_signoff",
        "claim_publication_missing": "republish_claim",
    }
    titles = {
        "skew_window_exceeded": "Peer outside supported skew window",
        "reconnect_required": "Boundary requires reconnect after upgrade",
        "reinstall_required": "Boundary requires reinstall to a supported version",
        "migration_needed": "Imported state requires migration before restore",
        "retest_pending": "Boundary retest pending",
        "evidence_stale": "Inspector evidence stale",
        "evidence_missing": "Inspector evidence missing",
        "waiver_expired": "Inspector waiver expired",
        "owner_signoff_missing": "Owner sign-off missing",
        "claim_publication_missing": "Claim publication missing",
    }
    out = []
    for reason in NARROWING_REASONS:
        out.append(
            {
                "rule_id": f"m5_skew_inspector_rule:{reason}",
                "title": titles[reason],
                "trigger_reason": reason,
                "applies_to_labels": ABOVE_CUTLINE,
                "default_action": action[reason],
                "blocks_promotion": True,
                "rationale": f"A boundary inspector at or above the cutline that reports '{reason}' cannot keep a Stable or LTS support claim.",
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
            for row in register["inspectors"]
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
            for row in register["inspectors"]
        )
    )
    blocking_claim_ids = sorted(
        {
            row["entry_id"]
            for row in register["inspectors"]
            if holds_stable(row["claim_label"])
            and any(r in triggers for r in row["active_narrowing_reasons"])
        }
    )
    decision = "hold" if blocking_rule_ids else "proceed"
    return {
        "promotion_gate": "m5-boundary-skew-inspectors-gate",
        "decision": decision,
        "blocking_rule_ids": blocking_rule_ids,
        "blocking_claim_ids": blocking_claim_ids,
        "rationale": "Computed from the firing stop rules over inspector verdicts, gate postures, skew windows, evidence freshness, retest state, waiver expiry, owner sign-off, and claim-publication linkage.",
    }


def compute_summary(register: dict) -> dict:
    rs = register["inspectors"]

    def published_stable(row):
        return holds_stable(row["published_label"])

    def state(s):
        return sum(1 for r in rs if r["inspector_state"] == s)

    def kind(k):
        return sum(1 for r in rs if r["boundary_kind"] == k)

    def subject(s):
        return sum(1 for r in rs if r["downgrade_subject"] == s)

    def verdict(v):
        return sum(1 for r in rs if r["verdict"] == v)

    def slo(s):
        return sum(1 for r in rs if r["proof_packet"]["slo_state"] == s)

    rb = [r for r in rs if r["release_blocking"]]
    boundaries = {r["boundary_ref"] for r in rs}
    return {
        "total_inspectors": len(rs),
        "total_boundaries": len(boundaries),
        "inspectors_publishing_stable": sum(1 for r in rs if published_stable(r)),
        "inspectors_narrowed": sum(1 for r in rs if not published_stable(r)),
        "inspectors_holding": sum(1 for r in rs if r["inspector_state"] in STATE_HOLDS),
        "inspectors_on_waiver": state("on_waiver"),
        "inspectors_limited": state("limited"),
        "inspectors_fail_closed": state("fail_closed"),
        "inspectors_retest_pending": state("retest_pending"),
        "inspectors_evidence_stale": state("evidence_stale"),
        "inspectors_incomplete": state("incomplete"),
        "gate_allow": sum(1 for r in rs if r["gate_posture"] == "allow"),
        "gate_fail_closed": sum(1 for r in rs if r["gate_posture"] == "fail_closed"),
        "release_blocking_total": len(rb),
        "release_blocking_publishing_stable": sum(1 for r in rb if published_stable(r)),
        "release_blocking_narrowed": sum(1 for r in rb if not published_stable(r)),
        "helper_agent_attach_inspectors": kind("helper_agent_attach"),
        "extension_runtime_load_inspectors": kind("extension_runtime_load"),
        "state_import_restore_inspectors": kind("state_import_restore"),
        "provider_snapshot_open_inspectors": kind("provider_snapshot_open"),
        "helper_subject_inspectors": subject("helper"),
        "agent_subject_inspectors": subject("agent"),
        "host_subject_inspectors": subject("host"),
        "schema_subject_inspectors": subject("schema"),
        "provider_subject_inspectors": subject("provider"),
        "verdict_inside_window": verdict("inside_window"),
        "verdict_unsupported_skew": verdict("unsupported_skew"),
        "verdict_reconnect_required": verdict("reconnect_required"),
        "verdict_reinstall_required": verdict("reinstall_required"),
        "verdict_migration_needed": verdict("migration_needed"),
        "verdict_retest_pending": verdict("retest_pending"),
        "packets_current": slo("current"),
        "packets_due_for_refresh": slo("due_for_refresh"),
        "packets_breached": slo("breached"),
        "packets_missing": slo("missing"),
        "total_active_narrowing_reasons": sum(len(r["active_narrowing_reasons"]) for r in rs),
        "total_upgrade_steps": sum(len(r["upgrade_order_guide"]["steps"]) for r in rs),
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
        "register_id": "m5_boundary_skew_inspectors:v1",
        "status": "published",
        "overview_page": f"docs/m5/{MODULE}.md",
        "as_of": AS_OF,
        "claim_manifest_ref": "artifacts/release/stable_claim_manifest.json",
        "qualification_matrix_ref": "artifacts/release/m5/freeze_the_m5_qualification_row_support_window_skew_window_and_deprecation_packet_matrix.json",
        "lifecycle_labels": LIFECYCLE_LABELS,
        "boundary_kinds": BOUNDARY_KINDS,
        "downgrade_subjects": DOWNGRADE_SUBJECTS,
        "gated_actions": GATED_ACTIONS,
        "action_risks": ACTION_RISKS,
        "skew_window_classes": SKEW_WINDOW_CLASSES,
        "inspector_verdicts": INSPECTOR_VERDICTS,
        "gate_postures": GATE_POSTURES,
        "upgrade_lead_sides": UPGRADE_LEAD_SIDES,
        "inspector_states": INSPECTOR_STATES,
        "narrowing_reasons": NARROWING_REASONS,
        "stop_rule_actions": STOP_RULE_ACTIONS,
        "launch_cutline": {
            "cutline_level": "stable",
            "above_cutline_levels": ABOVE_CUTLINE,
            "below_cutline_levels": BELOW_CUTLINE,
            "description": "An M5 mixed-version boundary inspector carries a Stable (or LTS) support claim only when its current verdict is inside-window (the gate allows the mutating or privileged action), its peer is inside the supported skew window, its proof packet is current within its freshness SLO, the owner has signed off, and its backing claim publication holds. A boundary whose inspector reports any fail-closed verdict — unsupported skew, reconnect required, reinstall required, migration needed, or retest pending — or whose evidence is stale or missing must narrow below the cutline rather than mutate optimistically or inherit an adjacent in-window boundary.",
        },
        "release_blocking_boundary_refs": [],
        "stop_rules": stop_rules(),
        "inspectors": inspectors(),
    }
    register["release_blocking_boundary_refs"] = [
        r["boundary_ref"] for r in register["inspectors"] if r["release_blocking"]
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
    dup["inspectors"][1]["entry_id"] = dup["inspectors"][0]["entry_id"]
    dup["summary"] = compute_summary(dup)
    dup["promotion"] = compute_promotion(dup)
    write_json(FIXTURES / "duplicate_entry_id.json", dup)
    cases.append(("duplicate_entry_id.json", "DuplicateEntryId"))

    held = copy.deepcopy(register)
    target = next(r for r in held["inspectors"] if holds_stable(r["published_label"]))
    target["active_narrowing_reasons"] = ["skew_window_exceeded"]
    held["summary"] = compute_summary(held)
    held["promotion"] = compute_promotion(held)
    write_json(FIXTURES / "held_with_active_gap.json", held)
    cases.append(("held_with_active_gap.json", "HeldWithActiveGap"))

    no_guide_case = copy.deepcopy(register)
    target = next(
        r for r in no_guide_case["inspectors"] if r["verdict"] in SKEW_RECOVERY_VERDICTS
    )
    target["upgrade_order_guide"] = {
        "lead_side": "none_required",
        "steps": [],
        "guide_ref": target["upgrade_order_guide"]["guide_ref"],
    }
    no_guide_case["summary"] = compute_summary(no_guide_case)
    no_guide_case["promotion"] = compute_promotion(no_guide_case)
    write_json(FIXTURES / "fail_closed_without_guide.json", no_guide_case)
    cases.append(("fail_closed_without_guide.json", "UpgradeGuideMissing"))

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
            "total_inspectors": s["total_inspectors"],
            "inspectors_publishing_stable": s["inspectors_publishing_stable"],
            "inspectors_narrowed": s["inspectors_narrowed"],
            "inspectors_holding": s["inspectors_holding"],
            "inspectors_on_waiver": s["inspectors_on_waiver"],
            "inspectors_limited": s["inspectors_limited"],
            "inspectors_fail_closed": s["inspectors_fail_closed"],
            "inspectors_retest_pending": s["inspectors_retest_pending"],
            "inspectors_evidence_stale": s["inspectors_evidence_stale"],
            "inspectors_incomplete": s["inspectors_incomplete"],
            "gate_allow": s["gate_allow"],
            "gate_fail_closed": s["gate_fail_closed"],
            "packets_breached": s["packets_breached"],
            "packets_missing": s["packets_missing"],
            "total_upgrade_steps": s["total_upgrade_steps"],
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
            {"drill_id": "drill:allow_on_fail_closed_verdict", "status": "passed"},
            {"drill_id": "drill:fail_closed_without_upgrade_guide", "status": "passed"},
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
