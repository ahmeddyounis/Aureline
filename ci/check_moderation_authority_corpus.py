#!/usr/bin/env python3
"""Validate the collaboration moderation & authority-escrow corpus.

This is the release-engineering / collaboration-preview proof lane that proves
Aureline's claimed shared-control rows (shared terminal, shared debug, and
presentation preview) cannot quietly mishandle a join, a temporary grant, or the
durable audit history on the marketed beta/preview lanes. It is the proof packet
a release, support, or security team attaches to a claimed shared-control row:
one current drill per claimed beta/preview row that a reviewer can read and
explain without rebuilding a live collaboration session.

It composes — it does not fork — the `InviteAuthorityReviewSheet` model the
desktop shell uses to gate every invite / observer-first join / temporary
authority-escrow review before anyone joins a session or takes privileged control
(see crates/aureline-shell/src/invite_review/mod.rs and the boundary schemas
schemas/collab/invite_session_manifest.schema.json and
schemas/collab/authority_escrow_ticket.schema.json). On top of the review sheet
it adds the moderation + audit drills this lane requires:

- **Lobby moderation (admit / deny / defer / decline).** Each drill pins a typed
  lobby decision against a join request and an invite-lifecycle class. A decision
  must agree with the grant state it claims to produce (an "admitted with grant"
  decision must actually carry a live or terminal grant; a "denied entry" or a
  recipient decline must carry no surviving authority).

- **Observer-first join + temporary grant.** The strongest thing an invite can
  confer on accept is the right to *request* control; the review-sheet model
  proves the requested scope is one the capability permits and belongs to the
  invited lane.

- **Moderation continuity (no silent resume, expired invites never look live).**
  Each drill carries a moderation flow (invited → lobby → admit/deny/defer →
  grant request → active → expiry/revoke/freeze → reconnect/handoff). A revoked,
  expired, or denied grant may never be displayed as active or quietly resume to
  active after a reconnect or handoff, and an expired or declined invite may never
  still appear active/joinable. The local shared-lane row stays usable across
  every post-decision stage.

- **Durable, export-safe audit history.** Each drill carries an audit-export
  record. Exported history MUST preserve the stable IDs (invite, ticket, lobby,
  join request, durable event), the actor and owner, the scope, the reason code,
  and the final resolution state — and MUST NOT carry a session secret. The
  audit-export record is re-derived from the review sheet + lobby and drift-
  checked, then proven to survive a support-bundle plaintext and a CLI / headless
  index unchanged, so moderation/export state cannot diverge across the UI and a
  support packet.

For every **accept** drill the validator schema-validates the invite manifest and
authority-escrow ticket against their boundary schemas, re-runs an independent
Python port of `InviteAuthorityReviewSheet::validate()` (a second implementation,
so a regression in either the Rust model or a fixture fails the lane), checks the
lobby decision, re-derives the moderation flow and the audit-export record and
drift-checks the stored values, proves export parity, and proves every claimed
beta/preview row maps to exactly one packet (with its preview-only / beta-scoped /
blocked status surfaced). For every **reject** drill it proves the documented
drift is actually rejected, with the expected typed reason.

Run via scripts/ci/run_collab_authority_corpus.sh. Use --write to (re)mint the
drill fixtures, the matrix, and the parity packet from the model.
"""

from __future__ import annotations

import argparse
import json
import sys
from dataclasses import asdict, dataclass, field
from pathlib import Path
from typing import Any

from jsonschema import Draft202012Validator, FormatChecker

# --------------------------------------------------------------------------- #
# Paths
# --------------------------------------------------------------------------- #

MANIFEST_SCHEMA_REL = "schemas/collab/invite_session_manifest.schema.json"
TICKET_SCHEMA_REL = "schemas/collab/authority_escrow_ticket.schema.json"
CORPUS_DIR_REL = "fixtures/collab/m3/moderation_and_authority_escrow_corpus"
CORPUS_MATRIX_REL = f"{CORPUS_DIR_REL}/corpus_matrix.json"
PARITY_PACKET_REL = f"{CORPUS_DIR_REL}/audit_export_parity.json"
README_REL = f"{CORPUS_DIR_REL}/README.md"
REPORT_REL = "artifacts/collab/m3/moderation_authority_report.md"
VALIDATOR_REL = "ci/check_moderation_authority_corpus.py"
SCRIPT_REL = "scripts/ci/run_collab_authority_corpus.sh"
CONTRACT_DOC_REL = "docs/collab/m3/invite_and_authority_escrow.md"

AUDIT_CONTRACT_REF = "collab:moderation_authority_corpus:v1"
DRILL_RECORD_KIND = "moderation_authority_drill_record"
MATRIX_RECORD_KIND = "moderation_authority_corpus_matrix"
PARITY_RECORD_KIND = "moderation_authority_export_parity_packet"
DRILL_SCHEMA_VERSION = 1

NON_DRILL_FILES = {"corpus_matrix.json", "audit_export_parity.json", "README.md"}

# Frozen contract-doc ref every constituent record must cite.
CONTRACT_DOC_REF = "docs/collab/m3/invite_and_authority_escrow.md"

# --------------------------------------------------------------------------- #
# Closed vocabularies — a deliberate second copy of the Rust enums in
# crates/aureline-shell/src/invite_review/mod.rs (model port) plus the
# corpus-only moderation / audit vocabularies. Keep in lockstep with the model.
# --------------------------------------------------------------------------- #

SESSION_LANE_CLASSES = {"shared_terminal", "shared_debug", "presentation_preview"}
INVITE_CAPABILITY_CLASSES = {
    "observer_only",
    "observer_with_follow",
    "observer_with_control_request",
}
OFFERED_ROLE_CLASSES = {"observer", "follow_viewer", "presenter_candidate", "driver_candidate"}
CLIENT_CLASSES = {"desktop_native", "browser_tab", "mobile_companion", "untrusted_web"}
RETENTION_POSTURE_CLASSES = {
    "live_only_no_retention",
    "no_recording_observer_only",
    "redacted_session_archive",
    "recording_with_consent",
}
EXPIRY_POSTURE_CLASSES = {
    "expires_at_fixed_time",
    "expires_at_session_end",
    "expires_on_context_change",
    "perpetual_no_expiry",
}
AUTHORITY_SCOPE_CLASSES = {
    "terminal_input",
    "debug_step_control",
    "presenter_handoff",
    "follow_handoff",
}
GRANT_STATE_CLASSES = {
    "requested",
    "granted_active",
    "expired",
    "revoked",
    "frozen_pending_reapproval",
    "denied",
}
CONTEXT_CHANGE_CLASSES = {
    "none_steady_state",
    "reconnected",
    "browser_to_desktop_handoff",
    "desktop_to_browser_handoff",
    "app_restart",
    "session_context_changed",
}
REASON_CODE_CLASSES = {
    "invite_offered_observer_first",
    "control_requested_pending",
    "control_granted_temporary",
    "control_expired",
    "control_revoked",
    "grant_frozen_pending_reapproval",
    "reapproval_required_on_context_change",
    "local_lane_preserved",
}

LIVE_GRANT_STATES = {"requested", "granted_active", "frozen_pending_reapproval"}
TERMINATED_GRANT_STATES = {"expired", "revoked", "denied"}

# Corpus-only moderation vocabulary (the lobby/admit/deny/defer layer).
INVITE_LIFECYCLE_CLASSES = {"pending_in_lobby", "accepted", "declined", "expired"}
MODERATION_DECISION_CLASSES = {
    "admitted_observer",
    "admitted_with_grant",
    "deferred_to_lobby",
    "denied_entry",
    "invite_declined_by_recipient",
    "invite_expired_in_lobby",
}
# A decision must agree with the grant state it claims to leave behind.
DECISION_GRANT_STATES = {
    "admitted_observer": {"requested"},
    "admitted_with_grant": {"granted_active", "expired", "revoked", "frozen_pending_reapproval"},
    "deferred_to_lobby": {"requested"},
    "denied_entry": {"denied"},
    "invite_declined_by_recipient": {"denied"},
    "invite_expired_in_lobby": {"denied"},
}

RESOLUTION_STATE_CLASSES = {
    "admitted_observer_only",
    "grant_active",
    "grant_expired",
    "grant_revoked",
    "grant_frozen",
    "denied",
    "held_in_lobby",
    "invite_declined",
    "invite_expired",
}

# Final resolution is a pure function of the invite lifecycle + grant state.
GRANT_STATE_RESOLUTION = {
    "requested": "admitted_observer_only",
    "granted_active": "grant_active",
    "expired": "grant_expired",
    "revoked": "grant_revoked",
    "frozen_pending_reapproval": "grant_frozen",
    "denied": "denied",
}

LANE_STATUS_CLASSES = {"preview_only", "beta_scoped", "blocked_unresolved"}

# Moderation-flow stage vocabulary.
STATIC_STAGE_SPEC = {
    # stage_class: (lobby_state_class, grant_state_class|None, displayed_grant_active, invite_active_default)
    "invited": ("invite_sent", None, False, True),
    "lobby_waiting": ("waiting", None, False, True),
    "admitted": ("admitted", None, False, True),
    "deferred": ("deferred", None, False, True),
    "denied": ("denied", "denied", False, False),
    "declined": ("closed", "denied", False, False),
    "grant_requested": ("admitted", "requested", False, True),
    "grant_active": ("admitted", "granted_active", True, True),
    "grant_expired": ("admitted", "expired", False, True),
    "grant_revoked": ("admitted", "revoked", False, True),
    "grant_frozen": ("admitted", "frozen_pending_reapproval", False, True),
    "closed": ("closed", None, False, True),
}
# Dynamic stages reflect the review sheet's authority-continuity block: the grant
# state after the context change, displayed as active only if it is genuinely an
# active grant *and* reapproval was satisfied.
DYNAMIC_STAGES = {"reconnected", "handed_off", "restarted"}
STAGE_CLASSES = set(STATIC_STAGE_SPEC) | DYNAMIC_STAGES

STAGE_SUMMARY = {
    "invited": "Invite delivered; nothing is shared until the join is admitted.",
    "lobby_waiting": "Join request is held in the lobby pending a moderation decision.",
    "admitted": "Moderator admitted the join as observer-first; no privileged control yet.",
    "deferred": "Moderator deferred the join; it stays in the lobby until decided.",
    "denied": "Moderator denied entry; no session access and no authority is granted.",
    "declined": "Recipient declined the invite; the local lane stays usable.",
    "grant_requested": "Observer requested temporary control; the grant is pending approval.",
    "grant_active": "Temporary control is active under a bounded expiry.",
    "grant_expired": "Temporary control expired; authority reverted to observer.",
    "grant_revoked": "Owner revoked temporary control; the revoked state persists.",
    "grant_frozen": "Grant is frozen pending reapproval; no privileged control runs.",
    "reconnected": "After reconnect the grant is re-presented; it never silently resumes.",
    "handed_off": "After a device handoff the grant is re-presented; it never silently resumes.",
    "restarted": "After restart the grant is re-presented; it never silently resumes.",
    "closed": "Session closed; the durable audit record remains.",
}

EXPECTATIONS = {"accept", "reject"}


# --------------------------------------------------------------------------- #
# Model-port helpers (mirrors of the Rust enum methods).
# --------------------------------------------------------------------------- #


def capability_grants_control_request(capability: Any) -> bool:
    return capability == "observer_with_control_request"


def capability_grants_follow(capability: Any) -> bool:
    return capability in {"observer_with_follow", "observer_with_control_request"}


def role_permitted_by_capability(role: Any, capability: Any) -> bool:
    if role == "observer":
        return True
    if role == "follow_viewer":
        return capability_grants_follow(capability)
    if role in {"presenter_candidate", "driver_candidate"}:
        return capability_grants_control_request(capability)
    return False


def scope_permitted_by_capability(scope: Any, capability: Any) -> bool:
    if scope in {"terminal_input", "debug_step_control", "presenter_handoff"}:
        return capability_grants_control_request(capability)
    if scope == "follow_handoff":
        return capability_grants_follow(capability)
    return False


def scope_required_lane(scope: Any) -> str | None:
    return {
        "terminal_input": "shared_terminal",
        "debug_step_control": "shared_debug",
        "presenter_handoff": "presentation_preview",
        "follow_handoff": "presentation_preview",
    }.get(scope)


def expiry_is_bounded(posture: Any) -> bool:
    return posture in EXPIRY_POSTURE_CLASSES and posture != "perpetual_no_expiry"


def grant_is_live(state: Any) -> bool:
    return state in LIVE_GRANT_STATES


def grant_is_privileged_active(state: Any) -> bool:
    return state == "granted_active"


def context_is_interrupting(change: Any) -> bool:
    return change in CONTEXT_CHANGE_CLASSES and change != "none_steady_state"


def reason_for_state(state: Any) -> str | None:
    return {
        "requested": "control_requested_pending",
        "granted_active": "control_granted_temporary",
        "expired": "control_expired",
        "revoked": "control_revoked",
        "denied": "control_revoked",
        "frozen_pending_reapproval": "grant_frozen_pending_reapproval",
    }.get(state)


def ref_is_opaque(value: Any) -> bool:
    if not isinstance(value, str):
        return False
    trimmed = value.strip()
    return (
        bool(trimmed)
        and trimmed == value
        and "://" not in trimmed
        and "@" not in trimmed
        and not any(c.isspace() for c in trimmed)
    )


def non_empty(value: Any) -> bool:
    return isinstance(value, str) and bool(value.strip())


# --------------------------------------------------------------------------- #
# Findings
# --------------------------------------------------------------------------- #


@dataclass
class Finding:
    severity: str
    check_id: str
    message: str
    remediation: str
    ref: str | None = None
    details: dict[str, Any] = field(default_factory=dict)

    def as_report(self) -> dict[str, Any]:
        payload = asdict(self)
        if payload["ref"] is None:
            payload.pop("ref")
        if not payload["details"]:
            payload.pop("details")
        return payload


def err(check_id: str, message: str, remediation: str, ref: str | None = None,
        **details: Any) -> Finding:
    return Finding("error", check_id, message, remediation, ref, dict(details))


# --------------------------------------------------------------------------- #
# Independent port of InviteAuthorityReviewSheet::validate(). Returns the full
# set of typed rejection codes (snake_case of the Rust error variants). An empty
# list means the review sheet validates.
# --------------------------------------------------------------------------- #


def validate_review_sheet(sheet: dict[str, Any]) -> list[str]:
    codes: list[str] = []

    if sheet.get("invite_authority_review_sheet_schema_version") != 1:
        codes.append("wrong_sheet_schema_version")
    if sheet.get("record_kind") != "invite_authority_review_sheet_record":
        codes.append("wrong_sheet_record_kind")
    sheet_id = sheet.get("sheet_id", "")
    if not (isinstance(sheet_id, str) and sheet_id.startswith("invite_authority_review_sheet:")):
        codes.append("malformed_sheet_id")
    if sheet.get("contract_doc_ref") != CONTRACT_DOC_REF:
        codes.append("wrong_contract_doc_ref")
    if not non_empty(sheet.get("sheet_summary")):
        codes.append("empty_required_field")

    manifest = sheet.get("invite_manifest", {})
    ticket = sheet.get("authority_ticket", {})
    continuity = sheet.get("authority_continuity", {})
    history = sheet.get("history_event", {})

    codes.extend(_validate_manifest(manifest))
    codes.extend(_validate_ticket(ticket))
    codes.extend(_validate_continuity(continuity))
    codes.extend(_validate_history(history))

    # The ticket binds to the bundled invite by stable id.
    if ticket.get("invite_ref") != manifest.get("invite_id"):
        codes.append("ticket_invite_ref_mismatch")

    # Observer-first: the requested scope must be one the capability permits.
    scope = ticket.get("authority_scope_class")
    capability = manifest.get("invite_capability_class")
    if scope in AUTHORITY_SCOPE_CLASSES and capability in INVITE_CAPABILITY_CLASSES:
        if not scope_permitted_by_capability(scope, capability):
            codes.append("control_requested_without_capability")

    # The requested scope must match the invited lane.
    lane = manifest.get("session_lane_class")
    if scope in AUTHORITY_SCOPE_CLASSES and lane in SESSION_LANE_CLASSES:
        if scope_required_lane(scope) != lane:
            codes.append("scope_lane_mismatch")

    # The durable reason code must match the grant state.
    state = ticket.get("grant_state_class")
    if state in GRANT_STATE_CLASSES:
        expected = reason_for_state(state)
        if history.get("reason_code_class") != expected:
            codes.append("reason_code_state_mismatch")

    # No silent privileged resume: an active grant after an interrupting context
    # change requires explicit reapproval.
    if (
        context_is_interrupting(continuity.get("context_change_class"))
        and grant_is_privileged_active(continuity.get("post_change_grant_state_class"))
        and not continuity.get("reapproval_satisfied")
    ):
        codes.append("silent_privileged_resume")

    # A preserved grant ref, when present, must point at this ticket.
    preserved = continuity.get("preserved_grant_ref")
    if preserved is not None and preserved != ticket.get("ticket_id"):
        codes.append("preserved_grant_ref_mismatch")

    return codes


def _validate_expiry(expiry: dict[str, Any]) -> list[str]:
    codes: list[str] = []
    if not non_empty(expiry.get("expiry_label")):
        codes.append("empty_required_field")
    ref = expiry.get("expires_at_ref")
    if ref is not None and not ref_is_opaque(ref):
        codes.append("raw_ref_leak")
    if expiry.get("posture_class") == "expires_at_fixed_time" and expiry.get("expires_at_ref") is None:
        codes.append("fixed_expiry_missing_deadline")
    return codes


def _validate_manifest(manifest: dict[str, Any]) -> list[str]:
    codes: list[str] = []
    if manifest.get("invite_session_manifest_schema_version") != 1:
        codes.append("wrong_manifest_schema_version")
    if manifest.get("record_kind") != "invite_session_manifest_record":
        codes.append("wrong_manifest_record_kind")
    invite_id = manifest.get("invite_id", "")
    if not (isinstance(invite_id, str) and invite_id.startswith("collab_invite:")):
        codes.append("malformed_invite_id")
    if manifest.get("contract_doc_ref") != CONTRACT_DOC_REF:
        codes.append("wrong_contract_doc_ref")
    for value in (
        manifest.get("headline_label"),
        manifest.get("manifest_summary"),
        manifest.get("session_owner_label"),
    ):
        if not non_empty(value):
            codes.append("empty_required_field")
    if not ref_is_opaque(manifest.get("session_owner_ref")):
        codes.append("raw_ref_leak")
    codes.extend(_validate_expiry(manifest.get("invite_expiry", {})))

    role = manifest.get("offered_role_class")
    capability = manifest.get("invite_capability_class")
    if role in OFFERED_ROLE_CLASSES and capability in INVITE_CAPABILITY_CLASSES:
        if not role_permitted_by_capability(role, capability):
            codes.append("role_capability_mismatch")
    posture = manifest.get("invite_expiry", {}).get("posture_class")
    if capability_grants_control_request(capability) and not expiry_is_bounded(posture):
        codes.append("invite_expiry_unbounded")
    client = manifest.get("client_class")
    retention = manifest.get("retention_recording_posture_class")
    if client == "untrusted_web" and retention not in {
        "live_only_no_retention",
        "no_recording_observer_only",
    }:
        codes.append("retention_unsafe_for_client")
    return codes


def _validate_ticket(ticket: dict[str, Any]) -> list[str]:
    codes: list[str] = []
    if ticket.get("authority_escrow_ticket_schema_version") != 1:
        codes.append("wrong_ticket_schema_version")
    if ticket.get("record_kind") != "authority_escrow_ticket_record":
        codes.append("wrong_ticket_record_kind")
    ticket_id = ticket.get("ticket_id", "")
    if not (isinstance(ticket_id, str) and ticket_id.startswith("authority_escrow:")):
        codes.append("malformed_ticket_id")
    if ticket.get("contract_doc_ref") != CONTRACT_DOC_REF:
        codes.append("wrong_contract_doc_ref")
    for value in (
        ticket.get("headline_label"),
        ticket.get("scope_summary"),
        ticket.get("requested_by_label"),
    ):
        if not non_empty(value):
            codes.append("empty_required_field")
    if not ref_is_opaque(ticket.get("invite_ref")):
        codes.append("raw_ref_leak")
    if not ref_is_opaque(ticket.get("requested_by_ref")):
        codes.append("raw_ref_leak")
    codes.extend(_validate_expiry(ticket.get("grant_expiry", {})))

    state = ticket.get("grant_state_class")
    posture = ticket.get("grant_expiry", {}).get("posture_class")
    triggers = ticket.get("reapproval_required_on") or []
    if grant_is_live(state) and not expiry_is_bounded(posture):
        codes.append("grant_expiry_unbounded")
    if grant_is_live(state) and not triggers:
        codes.append("no_reapproval_triggers")
    if grant_is_privileged_active(state):
        required = {"on_reconnect", "on_device_handoff", "on_session_restart"}
        if not required <= set(triggers):
            codes.append("silent_resume_not_guarded")
    return codes


def _validate_continuity(continuity: dict[str, Any]) -> list[str]:
    codes: list[str] = []
    if not non_empty(continuity.get("continuity_summary")):
        codes.append("empty_required_field")
    if continuity.get("resumed_privileged_silently"):
        codes.append("silent_resume_not_allowed")
    if not continuity.get("local_lane_usable"):
        codes.append("local_lane_not_usable")
    if not (continuity.get("available_actions") or []):
        codes.append("no_continuity_actions")
    ref = continuity.get("preserved_grant_ref")
    if ref is not None and not ref_is_opaque(ref):
        codes.append("raw_ref_leak")
    return codes


def _validate_history(event: dict[str, Any]) -> list[str]:
    codes: list[str] = []
    event_id = event.get("event_id", "")
    if not (
        isinstance(event_id, str)
        and event_id.startswith("collab_authority_event:")
        and ref_is_opaque(event_id)
    ):
        codes.append("malformed_event_id")
    if not non_empty(event.get("event_summary")):
        codes.append("empty_required_field")
    if not event.get("durable"):
        codes.append("event_not_durable")
    if not event.get("export_safe"):
        codes.append("event_not_export_safe")
    return codes


# --------------------------------------------------------------------------- #
# Lobby moderation checks (the corpus-specific admit/deny/defer layer).
# --------------------------------------------------------------------------- #


def check_lobby(sheet: dict[str, Any], lobby: dict[str, Any]) -> list[str]:
    codes: list[str] = []
    lobby_id = lobby.get("lobby_id", "")
    if not (isinstance(lobby_id, str) and lobby_id.startswith("collab_lobby:") and ref_is_opaque(lobby_id)):
        codes.append("malformed_lobby_id")
    join_id = lobby.get("join_request_id", "")
    if not (
        isinstance(join_id, str)
        and join_id.startswith("collab_join_request:")
        and ref_is_opaque(join_id)
    ):
        codes.append("malformed_join_request_id")
    if not ref_is_opaque(lobby.get("moderator_ref")):
        codes.append("lobby_raw_ref_leak")
    if not non_empty(lobby.get("moderator_label")):
        codes.append("lobby_empty_required_field")
    if not non_empty(lobby.get("decision_summary")):
        codes.append("lobby_empty_required_field")
    if lobby.get("contract_doc_ref") != CONTRACT_DOC_REF:
        codes.append("lobby_wrong_contract_doc_ref")

    lifecycle = lobby.get("invite_lifecycle_class")
    decision = lobby.get("moderation_decision_class")
    if lifecycle not in INVITE_LIFECYCLE_CLASSES:
        codes.append("lobby_unknown_lifecycle")
    if decision not in MODERATION_DECISION_CLASSES:
        codes.append("lobby_unknown_decision")

    manifest = sheet.get("invite_manifest", {})
    ticket = sheet.get("authority_ticket", {})
    if lobby.get("invite_ref") != manifest.get("invite_id"):
        codes.append("lobby_invite_ref_mismatch")
    if lobby.get("requester_ref") != ticket.get("requested_by_ref"):
        codes.append("lobby_requester_ref_mismatch")

    # The decision must agree with the grant state it claims to leave behind.
    state = ticket.get("grant_state_class")
    if decision in DECISION_GRANT_STATES and state in GRANT_STATE_CLASSES:
        if state not in DECISION_GRANT_STATES[decision]:
            codes.append("lobby_decision_grant_mismatch")
    return sorted(set(codes))


# --------------------------------------------------------------------------- #
# Derivations: the moderation flow, the resolution state, and the audit-export
# record are pure functions of the review sheet + lobby. The stored values in
# every drill are drift-checked against these, so a fixture can never quietly
# diverge from the sheet.
# --------------------------------------------------------------------------- #


def derive_resolution(lobby: dict[str, Any], grant_state: Any) -> str | None:
    lifecycle = lobby.get("invite_lifecycle_class")
    decision = lobby.get("moderation_decision_class")
    if lifecycle == "declined":
        return "invite_declined"
    if lifecycle == "expired":
        return "invite_expired"
    if decision == "denied_entry":
        return "denied"
    if decision == "deferred_to_lobby":
        return "held_in_lobby"
    return GRANT_STATE_RESOLUTION.get(grant_state)


def derive_moderation_identity(sheet: dict[str, Any], lobby: dict[str, Any]) -> dict[str, Any]:
    manifest = sheet["invite_manifest"]
    ticket = sheet["authority_ticket"]
    history = sheet["history_event"]
    return {
        "invite_id": manifest["invite_id"],
        "ticket_id": ticket["ticket_id"],
        "lobby_id": lobby["lobby_id"],
        "join_request_id": lobby["join_request_id"],
        "owner_ref": manifest["session_owner_ref"],
        "moderator_ref": lobby["moderator_ref"],
        "actor_ref": ticket["requested_by_ref"],
        "lane_class": manifest["session_lane_class"],
        "scope_class": ticket["authority_scope_class"],
        "capability_class": manifest["invite_capability_class"],
        "invite_lifecycle_class": lobby["invite_lifecycle_class"],
        "moderation_decision_class": lobby["moderation_decision_class"],
        "grant_state_class": ticket["grant_state_class"],
        "reason_code_class": history["reason_code_class"],
        "resolution_state_class": derive_resolution(lobby, ticket["grant_state_class"]),
    }


def build_moderation_flow(
    sheet: dict[str, Any], lobby: dict[str, Any], stage_classes: list[str]
) -> dict[str, Any]:
    identity = derive_moderation_identity(sheet, lobby)
    continuity = sheet["authority_continuity"]
    lifecycle = lobby["invite_lifecycle_class"]
    draft_ok = bool(continuity["local_lane_usable"])
    stages = []
    for stage_class in stage_classes:
        if stage_class in DYNAMIC_STAGES:
            grant_state = continuity["post_change_grant_state_class"]
            displayed = grant_state == "granted_active" and bool(continuity["reapproval_satisfied"])
            lobby_state = "admitted"
            invite_default = True
        else:
            lobby_state, grant_state, displayed, invite_default = STATIC_STAGE_SPEC[stage_class]
        invite_active = invite_default and lifecycle not in {"expired", "declined"}
        stages.append(
            {
                "stage_class": stage_class,
                "lobby_state_class": lobby_state,
                "grant_state_class": grant_state,
                "invite_displayed_active": invite_active,
                "displayed_grant_active": displayed,
                "draft_preserved": draft_ok,
                "lane_class": identity["lane_class"],
                "scope_class": identity["scope_class"],
                "summary": STAGE_SUMMARY[stage_class],
            }
        )
    return {"carried_identity": identity, "stages": stages}


def audit_export_id_for(sheet: dict[str, Any]) -> str:
    return sheet["invite_manifest"]["invite_id"].replace("collab_invite:", "collab_audit_export:", 1)


def derive_audit_export(sheet: dict[str, Any], lobby: dict[str, Any]) -> dict[str, Any]:
    manifest = sheet["invite_manifest"]
    ticket = sheet["authority_ticket"]
    history = sheet["history_event"]
    resolution = derive_resolution(lobby, ticket["grant_state_class"])
    return {
        "audit_export_id": audit_export_id_for(sheet),
        "contains_session_secret": False,
        "stable_ids": {
            "invite_id": manifest["invite_id"],
            "ticket_id": ticket["ticket_id"],
            "lobby_id": lobby["lobby_id"],
            "join_request_id": lobby["join_request_id"],
            "history_event_id": history["event_id"],
        },
        "owner_ref": manifest["session_owner_ref"],
        "moderator_ref": lobby["moderator_ref"],
        "actor_ref": ticket["requested_by_ref"],
        "lane_class": manifest["session_lane_class"],
        "scope_class": ticket["authority_scope_class"],
        "grant_state_class": ticket["grant_state_class"],
        "reason_code_class": history["reason_code_class"],
        "decision_class": lobby["moderation_decision_class"],
        "invite_lifecycle_class": lobby["invite_lifecycle_class"],
        "resolution_state_class": resolution,
        "export_summary": audit_summary_text(sheet, lobby, resolution),
    }


def audit_summary_text(sheet: dict[str, Any], lobby: dict[str, Any], resolution: str | None) -> str:
    manifest = sheet["invite_manifest"]
    ticket = sheet["authority_ticket"]
    return (
        f"Moderation audit for {manifest['session_lane_class']}: "
        f"decision={lobby['moderation_decision_class']}, resolution={resolution}, "
        f"actor={ticket['requested_by_ref']}, owner={manifest['session_owner_ref']}, "
        f"moderator={lobby['moderator_ref']}; reason={sheet['history_event']['reason_code_class']}. "
        "No session secrets are exported."
    )


# --------------------------------------------------------------------------- #
# Moderation-flow + audit-export checks (the corpus-specific drills).
# --------------------------------------------------------------------------- #


def check_moderation_flow(sheet: dict[str, Any], lobby: dict[str, Any], flow: dict[str, Any]) -> list[str]:
    codes: list[str] = []
    expected_identity = derive_moderation_identity(sheet, lobby)
    stored_identity = flow.get("carried_identity", {})
    if stored_identity != expected_identity:
        codes.append("moderation_identity_drift")
    identity = stored_identity or expected_identity

    stages = flow.get("stages") or []
    if not stages:
        codes.append("moderation_no_stages")
        return sorted(set(codes))

    lifecycle = identity.get("invite_lifecycle_class")
    terminated = False
    for stage in stages:
        stage_class = stage.get("stage_class")
        if stage_class not in STAGE_CLASSES:
            codes.append("moderation_unknown_stage")
            continue
        if stage.get("lane_class") != identity.get("lane_class"):
            codes.append("moderation_lane_drift")
        if stage.get("scope_class") != identity.get("scope_class"):
            codes.append("moderation_scope_drift")

        grant_state = stage.get("grant_state_class")
        displayed = bool(stage.get("displayed_grant_active"))
        # A terminated grant must never be displayed as active, nor reactivate.
        if grant_state in TERMINATED_GRANT_STATES:
            terminated = True
            if displayed:
                codes.append("moderation_revoked_silent_resume")
        if terminated and grant_state == "granted_active":
            codes.append("moderation_revoked_silent_resume")
        # Active display is honest only when the grant is genuinely active.
        if displayed and grant_state != "granted_active":
            codes.append("moderation_displayed_active_without_grant")
        # An expired or declined invite must never still appear active/joinable.
        if lifecycle in {"expired", "declined"} and stage.get("invite_displayed_active"):
            codes.append("expired_invite_shown_active")
        # The local shared-lane row stays usable across every stage.
        if not stage.get("draft_preserved"):
            codes.append("moderation_local_lane_dropped")
    return sorted(set(codes))


AUDIT_REQUIRED_OPAQUE = ("owner_ref", "moderator_ref", "actor_ref")
AUDIT_REQUIRED_CLASSES = (
    "lane_class",
    "scope_class",
    "grant_state_class",
    "reason_code_class",
    "decision_class",
    "invite_lifecycle_class",
    "resolution_state_class",
)


def check_audit_export(sheet: dict[str, Any], lobby: dict[str, Any], audit: dict[str, Any]) -> list[str]:
    codes: list[str] = []
    expected = derive_audit_export(sheet, lobby)

    if audit.get("contains_session_secret"):
        codes.append("audit_export_secret_leak")

    stable_ids = audit.get("stable_ids") or {}
    if not isinstance(stable_ids, dict) or not stable_ids:
        codes.append("audit_export_missing_field")
    for key in ("invite_id", "ticket_id", "lobby_id", "join_request_id", "history_event_id"):
        value = stable_ids.get(key) if isinstance(stable_ids, dict) else None
        if not non_empty(value):
            codes.append("audit_export_missing_field")
        elif not ref_is_opaque(value):
            codes.append("audit_export_raw_ref_leak")
    if not ref_is_opaque(audit.get("audit_export_id")):
        codes.append("audit_export_raw_ref_leak")
    for key in AUDIT_REQUIRED_OPAQUE:
        if not non_empty(audit.get(key)):
            codes.append("audit_export_missing_field")
        elif not ref_is_opaque(audit.get(key)):
            codes.append("audit_export_raw_ref_leak")
    for key in AUDIT_REQUIRED_CLASSES:
        if not non_empty(audit.get(key)):
            codes.append("audit_export_missing_field")
    if not non_empty(audit.get("export_summary")):
        codes.append("audit_export_missing_field")

    # Identity drift: stable IDs, actor/owner, scope, reason code, lane, decision.
    identity_keys = (
        "stable_ids",
        "owner_ref",
        "moderator_ref",
        "actor_ref",
        "lane_class",
        "scope_class",
        "grant_state_class",
        "reason_code_class",
        "decision_class",
        "invite_lifecycle_class",
    )
    if any(audit.get(k) != expected.get(k) for k in identity_keys):
        codes.append("audit_export_identity_drift")
    # Final-resolution drift (called out separately so a divergence is explicit).
    if audit.get("resolution_state_class") != expected.get("resolution_state_class"):
        codes.append("audit_export_resolution_drift")
    return sorted(set(codes))


# --------------------------------------------------------------------------- #
# Export parity — support-bundle plaintext and CLI / headless index preserve the
# semantics of the audit-export record.
# --------------------------------------------------------------------------- #


def audit_digest(audit: dict[str, Any]) -> dict[str, Any]:
    stable = audit.get("stable_ids") or {}
    return {
        "invite_id": stable.get("invite_id"),
        "ticket_id": stable.get("ticket_id"),
        "lobby_id": stable.get("lobby_id"),
        "join_request_id": stable.get("join_request_id"),
        "history_event_id": stable.get("history_event_id"),
        "owner_ref": audit.get("owner_ref"),
        "moderator_ref": audit.get("moderator_ref"),
        "actor_ref": audit.get("actor_ref"),
        "lane_class": audit.get("lane_class"),
        "scope_class": audit.get("scope_class"),
        "grant_state_class": audit.get("grant_state_class"),
        "reason_code_class": audit.get("reason_code_class"),
        "decision_class": audit.get("decision_class"),
        "invite_lifecycle_class": audit.get("invite_lifecycle_class"),
        "resolution_state_class": audit.get("resolution_state_class"),
        "contains_session_secret": bool(audit.get("contains_session_secret")),
    }


def coarse_digest(full: dict[str, Any]) -> dict[str, Any]:
    return {
        "lane_class": full["lane_class"],
        "scope_class": full["scope_class"],
        "grant_state_class": full["grant_state_class"],
        "decision_class": full["decision_class"],
        "resolution_state_class": full["resolution_state_class"],
        "contains_session_secret": full["contains_session_secret"],
    }


def _b(value: bool) -> str:
    return "true" if value else "false"


def render_support_export(audit: dict[str, Any]) -> str:
    d = audit_digest(audit)
    out = ["Collaboration moderation & authority-escrow audit export"]
    for key in (
        "invite_id",
        "ticket_id",
        "lobby_id",
        "join_request_id",
        "history_event_id",
        "owner_ref",
        "moderator_ref",
        "actor_ref",
        "lane_class",
        "scope_class",
        "grant_state_class",
        "reason_code_class",
        "decision_class",
        "invite_lifecycle_class",
        "resolution_state_class",
    ):
        out.append(f"{key}: {d[key]}")
    out.append(f"contains_session_secret: {_b(d['contains_session_secret'])}")
    return "\n".join(out) + "\n"


def digest_from_support_export(text: str) -> dict[str, Any]:
    fields: dict[str, str] = {}
    for line in text.split("\n"):
        if ": " in line:
            key, value = line.split(": ", 1)
            fields[key] = value
    digest = {
        key: fields[key]
        for key in (
            "invite_id",
            "ticket_id",
            "lobby_id",
            "join_request_id",
            "history_event_id",
            "owner_ref",
            "moderator_ref",
            "actor_ref",
            "lane_class",
            "scope_class",
            "grant_state_class",
            "reason_code_class",
            "decision_class",
            "invite_lifecycle_class",
            "resolution_state_class",
        )
    }
    digest["contains_session_secret"] = fields["contains_session_secret"] == "true"
    return digest


def render_cli_index(audit: dict[str, Any], fixture_filename: str) -> str:
    d = audit_digest(audit)
    return "\t".join(
        [
            d["lane_class"],
            d["scope_class"],
            d["grant_state_class"],
            d["decision_class"],
            d["resolution_state_class"],
            f"secret={_b(d['contains_session_secret'])}",
            fixture_filename,
        ]
    )


def digest_from_cli_index(line: str) -> dict[str, Any]:
    parts = line.split("\t")
    return {
        "lane_class": parts[0],
        "scope_class": parts[1],
        "grant_state_class": parts[2],
        "decision_class": parts[3],
        "resolution_state_class": parts[4],
        "contains_session_secret": parts[5].split("=", 1)[1] == "true",
    }


def check_parity(accept: dict[str, tuple[str, dict[str, Any]]]) -> list[Finding]:
    findings: list[Finding] = []
    for scenario_id, (source_fixture, drill) in sorted(accept.items()):
        audit = derive_audit_export(drill["review_sheet"], drill["lobby_moderation"])
        full = audit_digest(audit)
        support_text = render_support_export(audit)
        cli_line = render_cli_index(audit, Path(source_fixture).name)
        if digest_from_support_export(support_text) != full:
            findings.append(
                err(
                    "moderation_authority.parity.support_export",
                    f"{scenario_id}: support-export plaintext loses semantics from the audit record",
                    "Keep the support-export projection faithful to the audit record.",
                    source_fixture,
                )
            )
        if digest_from_cli_index(cli_line) != coarse_digest(full):
            findings.append(
                err(
                    "moderation_authority.parity.cli_index",
                    f"{scenario_id}: CLI / headless index loses semantics from the audit record",
                    "Keep the CLI / headless index faithful to the audit record.",
                    source_fixture,
                )
            )
    return findings


# --------------------------------------------------------------------------- #
# Sheet / lobby builders.
# --------------------------------------------------------------------------- #


def expiry(posture: str, label: str, ref: str | None = None) -> dict[str, Any]:
    return {"posture_class": posture, "expires_at_ref": ref, "expiry_label": label}


def make_sheet(
    *,
    slug: str,
    sheet_summary: str,
    lane: str,
    role: str,
    capability: str,
    client: str,
    retention: str,
    invite_expiry: dict[str, Any],
    manifest_headline: str,
    manifest_summary: str,
    scope: str,
    grant_state: str,
    grant_expiry: dict[str, Any],
    downgrade: str,
    reapproval_required_on: list[str],
    scope_summary: str,
    ticket_headline: str,
    context_change: str,
    post_change_state: str,
    reapproval_satisfied: bool,
    available_actions: list[str],
    continuity_summary: str,
    event_state_suffix: str,
    attention_surface: str,
    event_summary: str,
    owner_ref: str = "collab_principal:owner.alex",
    owner_label: str = "Alex (session owner)",
    requested_by_ref: str = "collab_principal:guest.sam",
    requested_by_label: str = "Sam (guest)",
) -> dict[str, Any]:
    invite_id = f"collab_invite:{slug}"
    ticket_id = f"authority_escrow:{slug}.grant"
    reason = reason_for_state(grant_state)
    return {
        "invite_authority_review_sheet_schema_version": 1,
        "record_kind": "invite_authority_review_sheet_record",
        "sheet_id": f"invite_authority_review_sheet:{slug}",
        "sheet_summary": sheet_summary,
        "invite_manifest": {
            "invite_session_manifest_schema_version": 1,
            "record_kind": "invite_session_manifest_record",
            "invite_id": invite_id,
            "session_owner_ref": owner_ref,
            "session_owner_label": owner_label,
            "session_lane_class": lane,
            "offered_role_class": role,
            "invite_capability_class": capability,
            "client_class": client,
            "retention_recording_posture_class": retention,
            "invite_expiry": invite_expiry,
            "headline_label": manifest_headline,
            "manifest_summary": manifest_summary,
            "contract_doc_ref": CONTRACT_DOC_REF,
            "notes": None,
        },
        "authority_ticket": {
            "authority_escrow_ticket_schema_version": 1,
            "record_kind": "authority_escrow_ticket_record",
            "ticket_id": ticket_id,
            "invite_ref": invite_id,
            "authority_scope_class": scope,
            "grant_state_class": grant_state,
            "requested_by_ref": requested_by_ref,
            "requested_by_label": requested_by_label,
            "grant_expiry": grant_expiry,
            "downgrade_path_class": downgrade,
            "reapproval_required_on": reapproval_required_on,
            "scope_summary": scope_summary,
            "headline_label": ticket_headline,
            "contract_doc_ref": CONTRACT_DOC_REF,
            "notes": None,
        },
        "authority_continuity": {
            "context_change_class": context_change,
            "resumed_privileged_silently": False,
            "post_change_grant_state_class": post_change_state,
            "reapproval_satisfied": reapproval_satisfied,
            "local_lane_usable": True,
            "available_actions": available_actions,
            "preserved_grant_ref": ticket_id,
            "continuity_summary": continuity_summary,
        },
        "history_event": {
            "event_id": f"collab_authority_event:{slug}.{event_state_suffix}",
            "attention_surface_class": attention_surface,
            "reason_code_class": reason,
            "durable": True,
            "export_safe": True,
            "event_summary": event_summary,
        },
        "contract_doc_ref": CONTRACT_DOC_REF,
        "notes": None,
    }


def make_lobby(
    *,
    slug: str,
    moderator_ref: str,
    moderator_label: str,
    requester_ref: str,
    requester_label: str,
    lifecycle: str,
    decision: str,
    decision_summary: str,
) -> dict[str, Any]:
    return {
        "lobby_id": f"collab_lobby:{slug}",
        "join_request_id": f"collab_join_request:{slug}",
        "invite_ref": f"collab_invite:{slug}",
        "moderator_ref": moderator_ref,
        "moderator_label": moderator_label,
        "requester_ref": requester_ref,
        "requester_label": requester_label,
        "invite_lifecycle_class": lifecycle,
        "moderation_decision_class": decision,
        "decision_summary": decision_summary,
        "contract_doc_ref": CONTRACT_DOC_REF,
    }


# --------------------------------------------------------------------------- #
# Drill definition.
# --------------------------------------------------------------------------- #


@dataclass
class Drill:
    scenario_id: str
    fixture_filename: str
    claimed_beta_row: str | None
    expectation: str
    title: str
    summary: str
    sheet: dict[str, Any]
    lobby: dict[str, Any]
    stage_classes: list[str]
    lane_status_class: str | None = None
    expected_rejection_codes: list[str] = field(default_factory=list)
    flow_override: dict[str, Any] | None = None
    audit_override: dict[str, Any] | None = None

    def record(self) -> dict[str, Any]:
        flow = (
            self.flow_override
            if self.flow_override is not None
            else build_moderation_flow(self.sheet, self.lobby, self.stage_classes)
        )
        audit = (
            self.audit_override
            if self.audit_override is not None
            else derive_audit_export(self.sheet, self.lobby)
        )
        return {
            "record_kind": DRILL_RECORD_KIND,
            "schema_version": DRILL_SCHEMA_VERSION,
            "drill_id": f"moderation_authority_drill:{self.scenario_id}",
            "scenario_id": self.scenario_id,
            "claimed_beta_row": self.claimed_beta_row,
            "expectation": self.expectation,
            "expected_rejection_codes": sorted(self.expected_rejection_codes),
            "lane_status_class": self.lane_status_class,
            "title": self.title,
            "summary": self.summary,
            "review_sheet": self.sheet,
            "lobby_moderation": self.lobby,
            "moderation_flow": flow,
            "audit_export": audit,
            "contract_doc_ref": CONTRACT_DOC_REF,
            "narrative_refs": [CONTRACT_DOC_REL],
        }


# --------------------------------------------------------------------------- #
# Accept drills — one per claimed beta/preview row.
# --------------------------------------------------------------------------- #


FULL_TRIGGERS = ["on_reconnect", "on_device_handoff", "on_session_restart"]


def accept_drills() -> list[Drill]:
    drills: list[Drill] = []

    # 1. Observer-first admit into the shared terminal; control still pending.
    drills.append(
        Drill(
            scenario_id="observer_first_admitted_terminal",
            fixture_filename="accept_observer_first_admitted_terminal.json",
            claimed_beta_row="beta.collab.shared_terminal.observer_first_join",
            expectation="accept",
            lane_status_class="beta_scoped",
            title="Observer-first admit into the shared terminal",
            summary="A lobby admit lets the guest join the shared terminal as observer-first; taking input is a separate pending request, and the audit export records the admit honestly.",
            sheet=make_sheet(
                slug="terminal.observer_first",
                sheet_summary="Observer-first shared-terminal admit with a pending temporary-input request; the exact scope is shown before control is approved.",
                lane="shared_terminal",
                role="driver_candidate",
                capability="observer_with_control_request",
                client="desktop_native",
                retention="live_only_no_retention",
                invite_expiry=expiry("expires_at_session_end", "Expires when the session ends"),
                manifest_headline="Join the shared terminal as observer",
                manifest_summary="You join the shared terminal as an observer first. Taking input is a separate request the owner must approve; nothing is recorded.",
                scope="terminal_input",
                grant_state="requested",
                grant_expiry=expiry("expires_on_context_change", "Ends on reconnect, device switch, or restart"),
                downgrade="revert_to_observer",
                reapproval_required_on=FULL_TRIGGERS + ["on_scope_change"],
                scope_summary="Requesting temporary input for the shared terminal pane only; the owner can revoke at any time.",
                ticket_headline="Pending: temporary terminal input",
                context_change="none_steady_state",
                post_change_state="requested",
                reapproval_satisfied=False,
                available_actions=["continue_observer_only", "continue_local_only", "export_authority_record"],
                continuity_summary="Steady state; the control request is pending and the local terminal stays usable while it waits.",
                event_state_suffix="requested",
                attention_surface="activity_center",
                event_summary="Guest admitted observer-first; a temporary terminal-input request is pending owner approval.",
            ),
            lobby=make_lobby(
                slug="terminal.observer_first",
                moderator_ref="collab_principal:owner.alex",
                moderator_label="Alex (session owner)",
                requester_ref="collab_principal:guest.sam",
                requester_label="Sam (guest)",
                lifecycle="accepted",
                decision="admitted_observer",
                decision_summary="Owner admitted the guest as observer-first; control stays a separate request.",
            ),
            stage_classes=["invited", "lobby_waiting", "admitted", "grant_requested"],
        )
    )

    # 2. Admit-with-grant: active, bounded terminal input.
    drills.append(
        Drill(
            scenario_id="terminal_input_granted_active",
            fixture_filename="accept_terminal_input_granted_active.json",
            claimed_beta_row="beta.collab.shared_terminal.temporary_input_grant",
            expectation="accept",
            lane_status_class="beta_scoped",
            title="Temporary terminal input granted and active",
            summary="The owner admits and grants temporary terminal input; the active grant is bounded and guards reconnect, device handoff, and restart so it can never silently resume.",
            sheet=make_sheet(
                slug="terminal.input_active",
                sheet_summary="Active, bounded temporary terminal-input grant that guards reconnect, device handoff, and restart.",
                lane="shared_terminal",
                role="driver_candidate",
                capability="observer_with_control_request",
                client="desktop_native",
                retention="live_only_no_retention",
                invite_expiry=expiry("expires_at_session_end", "Expires when the session ends"),
                manifest_headline="Join the shared terminal",
                manifest_summary="You join the shared terminal; temporary input is granted as a separate, bounded request.",
                scope="terminal_input",
                grant_state="granted_active",
                grant_expiry=expiry("expires_at_fixed_time", "Expires in 15 minutes", "collab_deadline:terminal.input_active.t1"),
                downgrade="revert_to_observer",
                reapproval_required_on=FULL_TRIGGERS + ["on_owner_change"],
                scope_summary="Temporary input for the shared terminal pane only; the owner can revoke at any time.",
                ticket_headline="Active: temporary terminal input",
                context_change="none_steady_state",
                post_change_state="granted_active",
                reapproval_satisfied=False,
                available_actions=["revoke_grant", "continue_local_only", "export_authority_record"],
                continuity_summary="Steady state; the active grant is bounded and will require reapproval if the connection, device, or process changes.",
                event_state_suffix="granted",
                attention_surface="durable_attention",
                event_summary="Temporary terminal input granted with a 15-minute bound; recorded in durable attention.",
            ),
            lobby=make_lobby(
                slug="terminal.input_active",
                moderator_ref="collab_principal:owner.alex",
                moderator_label="Alex (session owner)",
                requester_ref="collab_principal:guest.sam",
                requester_label="Sam (guest)",
                lifecycle="accepted",
                decision="admitted_with_grant",
                decision_summary="Owner admitted the guest and approved a bounded temporary terminal-input grant.",
            ),
            stage_classes=["invited", "admitted", "grant_requested", "grant_active"],
        )
    )

    # 3. Debug step/control grant expiry.
    drills.append(
        Drill(
            scenario_id="debug_grant_expires",
            fixture_filename="accept_debug_grant_expires.json",
            claimed_beta_row="beta.collab.shared_debug.step_control_grant",
            expectation="accept",
            lane_status_class="beta_scoped",
            title="Debug step/control grant expires cleanly",
            summary="A bounded debug step/control grant reaches its expiry; authority reverts to observer and the audit export records expiry as the final resolution.",
            sheet=make_sheet(
                slug="debug.step_expires",
                sheet_summary="A temporary debug step/control grant expired at its bound; the guest reverts to observer and the local debugger stays usable.",
                lane="shared_debug",
                role="driver_candidate",
                capability="observer_with_control_request",
                client="desktop_native",
                retention="redacted_session_archive",
                invite_expiry=expiry("expires_at_session_end", "Expires when the debug session ends"),
                manifest_headline="Join the shared debug session",
                manifest_summary="You join the shared debug session as an observer first; stepping the debugger is a separate temporary request.",
                scope="debug_step_control",
                grant_state="expired",
                grant_expiry=expiry("expires_at_fixed_time", "Expired after 20 minutes", "collab_deadline:debug.step_expires.t1"),
                downgrade="revert_to_observer",
                reapproval_required_on=[],
                scope_summary="Temporary step/continue/breakpoint control for the shared debug session; expired at its bound.",
                ticket_headline="Expired: temporary debug step/control",
                context_change="none_steady_state",
                post_change_state="expired",
                reapproval_satisfied=False,
                available_actions=["continue_observer_only", "continue_local_only", "export_authority_record"],
                continuity_summary="Steady state; the grant expired at its bound and authority reverted to observer.",
                event_state_suffix="expired",
                attention_surface="session_history",
                event_summary="Temporary debug step/control expired at its 20-minute bound; recorded in session history.",
            ),
            lobby=make_lobby(
                slug="debug.step_expires",
                moderator_ref="collab_principal:owner.alex",
                moderator_label="Alex (session owner)",
                requester_ref="collab_principal:guest.sam",
                requester_label="Sam (guest)",
                lifecycle="accepted",
                decision="admitted_with_grant",
                decision_summary="Owner admitted and granted bounded debug step/control; the grant later expired at its bound.",
            ),
            stage_classes=["invited", "admitted", "grant_active", "grant_expired"],
        )
    )

    # 4. Terminal grant revoked; reconnect after revoke stays revoked.
    drills.append(
        Drill(
            scenario_id="terminal_grant_revoked_reconnect",
            fixture_filename="accept_terminal_grant_revoked_reconnect.json",
            claimed_beta_row="beta.collab.shared_terminal.revoke_then_reconnect",
            expectation="accept",
            lane_status_class="beta_scoped",
            title="Revoked terminal grant stays revoked after reconnect",
            summary="The owner revokes temporary terminal input; after a reconnect the grant is still shown revoked, never silently resumed, and the local terminal stays usable.",
            sheet=make_sheet(
                slug="terminal.revoked_reconnect",
                sheet_summary="A revoked terminal grant stays visibly revoked after a reconnect, and the local terminal remains fully usable.",
                lane="shared_terminal",
                role="driver_candidate",
                capability="observer_with_control_request",
                client="desktop_native",
                retention="live_only_no_retention",
                invite_expiry=expiry("expires_at_session_end", "Expires when the session ends"),
                manifest_headline="Shared terminal (control revoked)",
                manifest_summary="The temporary terminal grant was revoked; you remain an observer and your own terminal is unaffected.",
                scope="terminal_input",
                grant_state="revoked",
                grant_expiry=expiry("expires_on_context_change", "Ended when revoked"),
                downgrade="revert_to_local_only",
                reapproval_required_on=[],
                scope_summary="Temporary terminal input was revoked by the owner; no authority remains on this ticket.",
                ticket_headline="Revoked: temporary terminal input",
                context_change="reconnected",
                post_change_state="revoked",
                reapproval_satisfied=False,
                available_actions=["continue_local_only", "continue_observer_only", "export_authority_record"],
                continuity_summary="After reconnect the grant is still shown as revoked rather than quietly resumed; the local terminal is fully usable.",
                event_state_suffix="revoked",
                attention_surface="session_history",
                event_summary="Temporary terminal input revoked; the revoked state persists in session history across the reconnect.",
            ),
            lobby=make_lobby(
                slug="terminal.revoked_reconnect",
                moderator_ref="collab_principal:owner.alex",
                moderator_label="Alex (session owner)",
                requester_ref="collab_principal:guest.sam",
                requester_label="Sam (guest)",
                lifecycle="accepted",
                decision="admitted_with_grant",
                decision_summary="Owner admitted and granted terminal input, then revoked it; the revoked state survives the reconnect.",
            ),
            stage_classes=["invited", "admitted", "grant_active", "grant_revoked", "reconnected"],
        )
    )

    # 5. Presenter handoff frozen pending reapproval after reconnect.
    drills.append(
        Drill(
            scenario_id="presenter_handoff_frozen_after_reconnect",
            fixture_filename="accept_presenter_handoff_frozen_after_reconnect.json",
            claimed_beta_row="beta.collab.presentation.presenter_handoff",
            expectation="accept",
            lane_status_class="preview_only",
            title="Presenter handoff freezes pending reapproval after reconnect",
            summary="After a reconnect the presenter handoff is frozen pending reapproval, never silently resumed; the audit export records the frozen resolution.",
            sheet=make_sheet(
                slug="presentation.handoff_frozen",
                sheet_summary="A presenter handoff is frozen pending reapproval after a reconnect; nobody's view is redirected until it is explicitly reapproved.",
                lane="presentation_preview",
                role="presenter_candidate",
                capability="observer_with_control_request",
                client="desktop_native",
                retention="no_recording_observer_only",
                invite_expiry=expiry("expires_on_context_change", "Expires if the session context changes"),
                manifest_headline="Join the presentation preview",
                manifest_summary="You join the presentation preview as a follower; presenting is a separate temporary handoff the owner must approve.",
                scope="presenter_handoff",
                grant_state="frozen_pending_reapproval",
                grant_expiry=expiry("expires_on_context_change", "Frozen until reapproved"),
                downgrade="freeze_pending_reapproval",
                reapproval_required_on=FULL_TRIGGERS,
                scope_summary="Temporary presenter handoff for the presentation preview; frozen after the reconnect until reapproved.",
                ticket_headline="Frozen: presenter handoff pending reapproval",
                context_change="reconnected",
                post_change_state="frozen_pending_reapproval",
                reapproval_satisfied=False,
                available_actions=["reapprove_control", "continue_observer_only", "export_authority_record"],
                continuity_summary="After reconnect the presenter handoff is frozen pending reapproval; no view is redirected until the owner reapproves.",
                event_state_suffix="frozen",
                attention_surface="durable_attention",
                event_summary="Presenter handoff frozen pending reapproval after the reconnect; recorded in durable attention.",
            ),
            lobby=make_lobby(
                slug="presentation.handoff_frozen",
                moderator_ref="collab_principal:owner.alex",
                moderator_label="Alex (session owner)",
                requester_ref="collab_principal:guest.sam",
                requester_label="Sam (guest)",
                lifecycle="accepted",
                decision="admitted_with_grant",
                decision_summary="Owner admitted and approved a presenter handoff; the reconnect froze it pending reapproval.",
            ),
            stage_classes=["invited", "admitted", "grant_active", "grant_frozen", "reconnected"],
        )
    )

    # 6. Browser-to-desktop handoff reapproved.
    drills.append(
        Drill(
            scenario_id="browser_to_desktop_reapproved",
            fixture_filename="accept_browser_to_desktop_reapproved.json",
            claimed_beta_row="beta.collab.shared_debug.browser_to_desktop_handoff",
            expectation="accept",
            lane_status_class="preview_only",
            title="Browser-to-desktop handoff resumes only after reapproval",
            summary="A debug grant taken in a browser tab is handed off to the desktop app; it resumes active only because it was explicitly reapproved, never silently.",
            sheet=make_sheet(
                slug="debug.browser_handoff",
                sheet_summary="A debug step/control grant carried from a browser tab to the desktop app resumes active only after explicit reapproval.",
                lane="shared_debug",
                role="driver_candidate",
                capability="observer_with_control_request",
                client="browser_tab",
                retention="redacted_session_archive",
                invite_expiry=expiry("expires_at_session_end", "Expires when the debug session ends"),
                manifest_headline="Join the shared debug session",
                manifest_summary="You join the shared debug session from a browser tab; stepping the debugger is a separate temporary request that re-approves on a device handoff.",
                scope="debug_step_control",
                grant_state="granted_active",
                grant_expiry=expiry("expires_at_fixed_time", "Expires in 20 minutes", "collab_deadline:debug.browser_handoff.t1"),
                downgrade="revert_to_observer",
                reapproval_required_on=FULL_TRIGGERS + ["on_owner_change"],
                scope_summary="Temporary step/control for the shared debug session; reapproved after the browser-to-desktop handoff.",
                ticket_headline="Active: temporary debug step/control",
                context_change="browser_to_desktop_handoff",
                post_change_state="granted_active",
                reapproval_satisfied=True,
                available_actions=["revoke_grant", "continue_local_only", "export_authority_record"],
                continuity_summary="The grant was carried from the browser tab to the desktop app and resumes active only because it was explicitly reapproved on the handoff.",
                event_state_suffix="reapproved",
                attention_surface="durable_attention",
                event_summary="Debug step/control reapproved after a browser-to-desktop handoff; recorded in durable attention.",
            ),
            lobby=make_lobby(
                slug="debug.browser_handoff",
                moderator_ref="collab_principal:owner.alex",
                moderator_label="Alex (session owner)",
                requester_ref="collab_principal:guest.sam",
                requester_label="Sam (guest)",
                lifecycle="accepted",
                decision="admitted_with_grant",
                decision_summary="Owner admitted and granted debug step/control; the browser-to-desktop handoff required explicit reapproval.",
            ),
            stage_classes=["invited", "admitted", "grant_active", "handed_off"],
        )
    )

    # 7. Join deferred to the lobby.
    drills.append(
        Drill(
            scenario_id="join_deferred_to_lobby",
            fixture_filename="accept_join_deferred_to_lobby.json",
            claimed_beta_row="beta.collab.lobby.defer_join",
            expectation="accept",
            lane_status_class="beta_scoped",
            title="Join request deferred to the lobby",
            summary="The moderator defers a join request; the requester waits in the lobby with no privileged control, and the audit export records the held-in-lobby resolution.",
            sheet=make_sheet(
                slug="terminal.deferred",
                sheet_summary="A join request is deferred to the lobby; the control request stays pending and the local terminal stays usable while it waits.",
                lane="shared_terminal",
                role="driver_candidate",
                capability="observer_with_control_request",
                client="desktop_native",
                retention="live_only_no_retention",
                invite_expiry=expiry("expires_at_session_end", "Expires when the session ends"),
                manifest_headline="Join the shared terminal (waiting in lobby)",
                manifest_summary="Your join request is held in the lobby; you cannot see or drive the shared terminal until the moderator admits you.",
                scope="terminal_input",
                grant_state="requested",
                grant_expiry=expiry("expires_on_context_change", "Ends on reconnect, device switch, or restart"),
                downgrade="revert_to_observer",
                reapproval_required_on=FULL_TRIGGERS,
                scope_summary="Requesting temporary input for the shared terminal; the request waits while the join is in the lobby.",
                ticket_headline="Pending: join held in lobby",
                context_change="none_steady_state",
                post_change_state="requested",
                reapproval_satisfied=False,
                available_actions=["continue_local_only", "export_authority_record"],
                continuity_summary="Steady state; the join is deferred to the lobby and the local terminal stays usable while it waits.",
                event_state_suffix="deferred",
                attention_surface="notification_inbox",
                event_summary="Join request deferred to the lobby; the control request remains pending owner action.",
            ),
            lobby=make_lobby(
                slug="terminal.deferred",
                moderator_ref="collab_principal:owner.alex",
                moderator_label="Alex (session owner)",
                requester_ref="collab_principal:guest.sam",
                requester_label="Sam (guest)",
                lifecycle="pending_in_lobby",
                decision="deferred_to_lobby",
                decision_summary="Moderator deferred the join; the requester waits in the lobby until a later decision.",
            ),
            stage_classes=["invited", "lobby_waiting", "deferred", "grant_requested"],
        )
    )

    # 8. Entry denied by the moderator.
    drills.append(
        Drill(
            scenario_id="entry_denied_by_moderator",
            fixture_filename="accept_entry_denied_by_moderator.json",
            claimed_beta_row="beta.collab.lobby.deny_entry",
            expectation="accept",
            lane_status_class="beta_scoped",
            title="Lobby entry denied by the moderator",
            summary="The moderator denies a join request; no session access and no authority is granted, the local lane stays usable, and the audit export records the denial.",
            sheet=make_sheet(
                slug="debug.denied",
                sheet_summary="A join request is denied at the lobby; no session access is granted and the local debugger stays usable.",
                lane="shared_debug",
                role="driver_candidate",
                capability="observer_with_control_request",
                client="desktop_native",
                retention="redacted_session_archive",
                invite_expiry=expiry("expires_at_session_end", "Expires when the debug session ends"),
                manifest_headline="Join request denied",
                manifest_summary="The moderator denied this join request; you have no access to the shared debug session and your local debugger is unaffected.",
                scope="debug_step_control",
                grant_state="denied",
                grant_expiry=expiry("expires_on_context_change", "No grant was issued"),
                downgrade="revert_to_local_only",
                reapproval_required_on=[],
                scope_summary="No debug step/control authority was granted; the join request was denied at the lobby.",
                ticket_headline="Denied: no debug authority granted",
                context_change="none_steady_state",
                post_change_state="denied",
                reapproval_satisfied=False,
                available_actions=["continue_local_only", "export_authority_record"],
                continuity_summary="Steady state; the join was denied at the lobby and the local debugger stays fully usable.",
                event_state_suffix="denied",
                attention_surface="session_history",
                event_summary="Join request denied at the lobby; no authority granted, recorded in session history.",
            ),
            lobby=make_lobby(
                slug="debug.denied",
                moderator_ref="collab_principal:owner.alex",
                moderator_label="Alex (session owner)",
                requester_ref="collab_principal:guest.sam",
                requester_label="Sam (guest)",
                lifecycle="pending_in_lobby",
                decision="denied_entry",
                decision_summary="Moderator denied the join request; the requester gets no session access.",
            ),
            stage_classes=["invited", "lobby_waiting", "denied"],
        )
    )

    # 9. Invite declined by the recipient.
    drills.append(
        Drill(
            scenario_id="invite_declined_local_preserved",
            fixture_filename="accept_invite_declined_local_preserved.json",
            claimed_beta_row="beta.collab.presentation.invite_declined",
            expectation="accept",
            lane_status_class="preview_only",
            title="Invite declined by the recipient",
            summary="The recipient declines a presentation invite; the invite is recorded declined, the local lane stays usable, and the audit export records the decline.",
            sheet=make_sheet(
                slug="presentation.declined",
                sheet_summary="A presentation-preview invite is declined by the recipient; no authority is taken and the local lane stays usable.",
                lane="presentation_preview",
                role="presenter_candidate",
                capability="observer_with_control_request",
                client="desktop_native",
                retention="no_recording_observer_only",
                invite_expiry=expiry("expires_on_context_change", "Expires if the session context changes"),
                manifest_headline="Presentation invite (declined)",
                manifest_summary="You declined this presentation-preview invite; you have not joined and nothing about your local session changed.",
                scope="presenter_handoff",
                grant_state="denied",
                grant_expiry=expiry("expires_on_context_change", "No grant was issued"),
                downgrade="revert_to_local_only",
                reapproval_required_on=[],
                scope_summary="No presenter handoff authority was taken; the invite was declined by the recipient.",
                ticket_headline="Declined: no presenter authority taken",
                context_change="none_steady_state",
                post_change_state="denied",
                reapproval_satisfied=False,
                available_actions=["continue_local_only", "export_authority_record"],
                continuity_summary="Steady state; the recipient declined the invite and the local lane is fully usable.",
                event_state_suffix="declined",
                attention_surface="session_history",
                event_summary="Presentation invite declined by the recipient; no authority taken, recorded in session history.",
            ),
            lobby=make_lobby(
                slug="presentation.declined",
                moderator_ref="collab_principal:owner.alex",
                moderator_label="Alex (session owner)",
                requester_ref="collab_principal:guest.sam",
                requester_label="Sam (guest)",
                lifecycle="declined",
                decision="invite_declined_by_recipient",
                decision_summary="Recipient declined the invite; no join and no authority was taken.",
            ),
            stage_classes=["invited", "declined"],
        )
    )

    return drills


# --------------------------------------------------------------------------- #
# Reject drills — one per documented drift axis.
# --------------------------------------------------------------------------- #


def reject_drills() -> list[Drill]:
    drills: list[Drill] = []

    # 1. A revoked grant silently resumes to active after a reconnect.
    revoked = make_sheet(
        slug="terminal.silent_resume",
        sheet_summary="A revoked terminal grant is shown active again after a reconnect — silent privileged resume.",
        lane="shared_terminal",
        role="driver_candidate",
        capability="observer_with_control_request",
        client="desktop_native",
        retention="live_only_no_retention",
        invite_expiry=expiry("expires_at_session_end", "Expires when the session ends"),
        manifest_headline="Shared terminal (control revoked)",
        manifest_summary="The temporary terminal grant was revoked; you remain an observer.",
        scope="terminal_input",
        grant_state="revoked",
        grant_expiry=expiry("expires_on_context_change", "Ended when revoked"),
        downgrade="revert_to_local_only",
        reapproval_required_on=[],
        scope_summary="Temporary terminal input was revoked; no authority remains on this ticket.",
        ticket_headline="Revoked: temporary terminal input",
        context_change="reconnected",
        post_change_state="revoked",
        reapproval_satisfied=False,
        available_actions=["continue_local_only", "export_authority_record"],
        continuity_summary="The revoked grant must stay revoked across the reconnect.",
        event_state_suffix="revoked",
        attention_surface="session_history",
        event_summary="Temporary terminal input revoked.",
    )
    revoked_lobby = make_lobby(
        slug="terminal.silent_resume",
        moderator_ref="collab_principal:owner.alex",
        moderator_label="Alex (session owner)",
        requester_ref="collab_principal:guest.sam",
        requester_label="Sam (guest)",
        lifecycle="accepted",
        decision="admitted_with_grant",
        decision_summary="Owner admitted, granted, then revoked terminal input.",
    )
    revoked_flow = build_moderation_flow(
        revoked, revoked_lobby, ["invited", "admitted", "grant_active", "grant_revoked", "reconnected"]
    )
    for stage in revoked_flow["stages"]:
        if stage["stage_class"] == "reconnected":
            stage["grant_state_class"] = "granted_active"
            stage["displayed_grant_active"] = True
    drills.append(
        Drill(
            scenario_id="revoked_grant_silent_resume",
            fixture_filename="reject_revoked_grant_silent_resume.json",
            claimed_beta_row=None,
            expectation="reject",
            title="Revoked grant that silently resumes is rejected",
            summary="A revoked grant that is shown active again after a reconnect — without a fresh admit and approval — is rejected so privileged control can never silently resume.",
            sheet=revoked,
            lobby=revoked_lobby,
            stage_classes=["invited", "admitted", "grant_active", "grant_revoked", "reconnected"],
            expected_rejection_codes=["moderation_revoked_silent_resume"],
            flow_override=revoked_flow,
        )
    )

    # 2. An expired invite still appears active/joinable.
    expired_sheet = make_sheet(
        slug="terminal.expired_invite",
        sheet_summary="An expired invite is still shown as active/joinable in the lobby.",
        lane="shared_terminal",
        role="driver_candidate",
        capability="observer_with_control_request",
        client="desktop_native",
        retention="live_only_no_retention",
        invite_expiry=expiry("expires_on_context_change", "Expired when the session context changed"),
        manifest_headline="Shared terminal invite (expired)",
        manifest_summary="This invite has expired; it should no longer be joinable.",
        scope="terminal_input",
        grant_state="denied",
        grant_expiry=expiry("expires_on_context_change", "No grant was issued"),
        downgrade="revert_to_local_only",
        reapproval_required_on=[],
        scope_summary="No authority was granted; the invite expired in the lobby.",
        ticket_headline="Expired: invite no longer joinable",
        context_change="none_steady_state",
        post_change_state="denied",
        reapproval_satisfied=False,
        available_actions=["continue_local_only", "export_authority_record"],
        continuity_summary="An expired invite must not still appear active.",
        event_state_suffix="expired",
        attention_surface="session_history",
        event_summary="Invite expired in the lobby.",
    )
    expired_lobby = make_lobby(
        slug="terminal.expired_invite",
        moderator_ref="collab_principal:owner.alex",
        moderator_label="Alex (session owner)",
        requester_ref="collab_principal:guest.sam",
        requester_label="Sam (guest)",
        lifecycle="expired",
        decision="invite_expired_in_lobby",
        decision_summary="The invite expired before the moderator decided.",
    )
    expired_flow = build_moderation_flow(
        expired_sheet, expired_lobby, ["invited", "lobby_waiting", "closed"]
    )
    for stage in expired_flow["stages"]:
        if stage["stage_class"] == "lobby_waiting":
            stage["invite_displayed_active"] = True
    drills.append(
        Drill(
            scenario_id="expired_invite_shown_active",
            fixture_filename="reject_expired_invite_shown_active.json",
            claimed_beta_row=None,
            expectation="reject",
            title="Expired invite shown active is rejected",
            summary="An expired invite that still appears active/joinable in the lobby is rejected so a stale invite can never read as live.",
            sheet=expired_sheet,
            lobby=expired_lobby,
            stage_classes=["invited", "lobby_waiting", "closed"],
            expected_rejection_codes=["expired_invite_shown_active"],
            flow_override=expired_flow,
        )
    )

    # 3. The audit export diverges from the UI / review-sheet truth.
    diverge = make_sheet(
        slug="debug.audit_divergence",
        sheet_summary="The audit export claims a different resolution than the review sheet — moderation/export divergence.",
        lane="shared_debug",
        role="driver_candidate",
        capability="observer_with_control_request",
        client="desktop_native",
        retention="redacted_session_archive",
        invite_expiry=expiry("expires_at_session_end", "Expires when the debug session ends"),
        manifest_headline="Shared debug (control revoked)",
        manifest_summary="The temporary debug grant was revoked; you remain an observer.",
        scope="debug_step_control",
        grant_state="revoked",
        grant_expiry=expiry("expires_on_context_change", "Ended when revoked"),
        downgrade="revert_to_local_only",
        reapproval_required_on=[],
        scope_summary="Temporary debug step/control was revoked; no authority remains.",
        ticket_headline="Revoked: temporary debug step/control",
        context_change="none_steady_state",
        post_change_state="revoked",
        reapproval_satisfied=False,
        available_actions=["continue_local_only", "export_authority_record"],
        continuity_summary="The audit export must match the revoked review-sheet truth.",
        event_state_suffix="revoked",
        attention_surface="session_history",
        event_summary="Temporary debug step/control revoked.",
    )
    diverge_lobby = make_lobby(
        slug="debug.audit_divergence",
        moderator_ref="collab_principal:owner.alex",
        moderator_label="Alex (session owner)",
        requester_ref="collab_principal:guest.sam",
        requester_label="Sam (guest)",
        lifecycle="accepted",
        decision="admitted_with_grant",
        decision_summary="Owner admitted, granted, then revoked debug step/control.",
    )
    diverge_audit = derive_audit_export(diverge, diverge_lobby)
    diverge_audit["resolution_state_class"] = "grant_active"
    drills.append(
        Drill(
            scenario_id="audit_export_diverges_from_ui",
            fixture_filename="reject_audit_export_diverges_from_ui.json",
            claimed_beta_row=None,
            expectation="reject",
            title="Audit export that diverges from the UI is rejected",
            summary="An audit export that records a different final resolution than the review sheet (claims active control while the grant is revoked) is rejected so moderation/export state cannot diverge across the UI and a support packet.",
            sheet=diverge,
            lobby=diverge_lobby,
            stage_classes=["invited", "admitted", "grant_active", "grant_revoked"],
            expected_rejection_codes=["audit_export_resolution_drift"],
            audit_override=diverge_audit,
        )
    )

    # 4. The audit export leaks a session secret.
    leak = make_sheet(
        slug="terminal.audit_secret",
        sheet_summary="The audit export carries a session secret — never allowed.",
        lane="shared_terminal",
        role="driver_candidate",
        capability="observer_with_control_request",
        client="desktop_native",
        retention="live_only_no_retention",
        invite_expiry=expiry("expires_at_session_end", "Expires when the session ends"),
        manifest_headline="Shared terminal (control expired)",
        manifest_summary="The temporary terminal grant expired; you remain an observer.",
        scope="terminal_input",
        grant_state="expired",
        grant_expiry=expiry("expires_at_fixed_time", "Expired after 10 minutes", "collab_deadline:terminal.audit_secret.t1"),
        downgrade="revert_to_observer",
        reapproval_required_on=[],
        scope_summary="Temporary terminal input expired at its bound; no authority remains.",
        ticket_headline="Expired: temporary terminal input",
        context_change="none_steady_state",
        post_change_state="expired",
        reapproval_satisfied=False,
        available_actions=["continue_observer_only", "continue_local_only", "export_authority_record"],
        continuity_summary="The audit export must not carry any session secret.",
        event_state_suffix="expired",
        attention_surface="session_history",
        event_summary="Temporary terminal input expired.",
    )
    leak_lobby = make_lobby(
        slug="terminal.audit_secret",
        moderator_ref="collab_principal:owner.alex",
        moderator_label="Alex (session owner)",
        requester_ref="collab_principal:guest.sam",
        requester_label="Sam (guest)",
        lifecycle="accepted",
        decision="admitted_with_grant",
        decision_summary="Owner admitted and granted terminal input; the grant later expired.",
    )
    leak_audit = derive_audit_export(leak, leak_lobby)
    leak_audit["contains_session_secret"] = True
    drills.append(
        Drill(
            scenario_id="audit_export_leaks_secret",
            fixture_filename="reject_audit_export_leaks_secret.json",
            claimed_beta_row=None,
            expectation="reject",
            title="Audit export that leaks a session secret is rejected",
            summary="An audit export flagged as carrying a session secret is rejected; exported moderation history carries stable IDs, actor/owner, scope, reason code, and resolution — never session secrets.",
            sheet=leak,
            lobby=leak_lobby,
            stage_classes=["invited", "admitted", "grant_active", "grant_expired"],
            expected_rejection_codes=["audit_export_secret_leak"],
            audit_override=leak_audit,
        )
    )

    # 5. Model-level: a perpetual (unbounded) temporary grant.
    perpetual = make_sheet(
        slug="debug.perpetual_grant",
        sheet_summary="An active debug grant carries a perpetual (unbounded) expiry — temporary authority is never perpetual.",
        lane="shared_debug",
        role="driver_candidate",
        capability="observer_with_control_request",
        client="desktop_native",
        retention="redacted_session_archive",
        invite_expiry=expiry("expires_at_session_end", "Expires when the debug session ends"),
        manifest_headline="Join the shared debug session",
        manifest_summary="You join the shared debug session; stepping the debugger is a separate temporary request.",
        scope="debug_step_control",
        grant_state="granted_active",
        grant_expiry=expiry("perpetual_no_expiry", "Never expires"),
        downgrade="revert_to_observer",
        reapproval_required_on=FULL_TRIGGERS,
        scope_summary="Temporary debug step/control with no expiry — not allowed.",
        ticket_headline="Active: debug step/control (no expiry)",
        context_change="none_steady_state",
        post_change_state="granted_active",
        reapproval_satisfied=False,
        available_actions=["revoke_grant", "continue_local_only", "export_authority_record"],
        continuity_summary="A temporary grant must carry a bound.",
        event_state_suffix="granted",
        attention_surface="durable_attention",
        event_summary="Debug step/control granted.",
    )
    perpetual_lobby = make_lobby(
        slug="debug.perpetual_grant",
        moderator_ref="collab_principal:owner.alex",
        moderator_label="Alex (session owner)",
        requester_ref="collab_principal:guest.sam",
        requester_label="Sam (guest)",
        lifecycle="accepted",
        decision="admitted_with_grant",
        decision_summary="Owner admitted and granted debug step/control with no expiry.",
    )
    drills.append(
        Drill(
            scenario_id="perpetual_control_grant",
            fixture_filename="reject_perpetual_control_grant.json",
            claimed_beta_row=None,
            expectation="reject",
            title="Perpetual temporary grant is rejected",
            summary="An active grant with a perpetual (unbounded) expiry is rejected by the review-sheet model port; temporary authority is never perpetual.",
            sheet=perpetual,
            lobby=perpetual_lobby,
            stage_classes=["invited", "admitted", "grant_active"],
            expected_rejection_codes=["grant_expiry_unbounded"],
        )
    )

    # 6. Model-level: an active grant resumes after reconnect without reapproval.
    silent = make_sheet(
        slug="debug.silent_reconnect",
        sheet_summary="An active debug grant resumes after a reconnect without explicit reapproval.",
        lane="shared_debug",
        role="driver_candidate",
        capability="observer_with_control_request",
        client="desktop_native",
        retention="redacted_session_archive",
        invite_expiry=expiry("expires_at_session_end", "Expires when the debug session ends"),
        manifest_headline="Join the shared debug session",
        manifest_summary="You join the shared debug session; stepping the debugger is a separate temporary request.",
        scope="debug_step_control",
        grant_state="granted_active",
        grant_expiry=expiry("expires_at_fixed_time", "Expires in 20 minutes", "collab_deadline:debug.silent_reconnect.t1"),
        downgrade="revert_to_observer",
        reapproval_required_on=FULL_TRIGGERS,
        scope_summary="Temporary step/control for the shared debug session.",
        ticket_headline="Active: temporary debug step/control",
        context_change="reconnected",
        post_change_state="granted_active",
        reapproval_satisfied=False,
        available_actions=["revoke_grant", "continue_local_only", "export_authority_record"],
        continuity_summary="An active grant after reconnect must be explicitly reapproved.",
        event_state_suffix="granted",
        attention_surface="durable_attention",
        event_summary="Debug step/control granted.",
    )
    silent_lobby = make_lobby(
        slug="debug.silent_reconnect",
        moderator_ref="collab_principal:owner.alex",
        moderator_label="Alex (session owner)",
        requester_ref="collab_principal:guest.sam",
        requester_label="Sam (guest)",
        lifecycle="accepted",
        decision="admitted_with_grant",
        decision_summary="Owner admitted and granted debug step/control.",
    )
    drills.append(
        Drill(
            scenario_id="silent_resume_after_reconnect",
            fixture_filename="reject_silent_resume_after_reconnect.json",
            claimed_beta_row=None,
            expectation="reject",
            title="Active grant resuming after reconnect without reapproval is rejected",
            summary="An active grant whose authority-continuity block resumes after a reconnect without explicit reapproval is rejected by the review-sheet model port.",
            sheet=silent,
            lobby=silent_lobby,
            stage_classes=["invited", "admitted", "grant_active", "reconnected"],
            expected_rejection_codes=["silent_privileged_resume"],
        )
    )

    return drills


def all_drills() -> list[Drill]:
    return accept_drills() + reject_drills()


CLAIMED_BETA_ROWS = {
    d.scenario_id: d.claimed_beta_row
    for d in accept_drills()
    if d.claimed_beta_row is not None
}


# --------------------------------------------------------------------------- #
# Per-drill validation
# --------------------------------------------------------------------------- #


def check_drill(label: str, drill: dict[str, Any], manifest_validator: Draft202012Validator,
                ticket_validator: Draft202012Validator) -> list[Finding]:
    findings: list[Finding] = []

    if drill.get("record_kind") != DRILL_RECORD_KIND:
        findings.append(
            err(
                "moderation_authority.drill.record_kind",
                f"{label}: record_kind must be {DRILL_RECORD_KIND}",
                "Re-mint the drill with --write.",
                label,
            )
        )
        return findings
    if drill.get("schema_version") != DRILL_SCHEMA_VERSION:
        findings.append(
            err(
                "moderation_authority.drill.schema_version",
                f"{label}: schema_version must be {DRILL_SCHEMA_VERSION}",
                "Bump the validator together with any drill schema change.",
                label,
            )
        )
    expectation = drill.get("expectation")
    if expectation not in EXPECTATIONS:
        findings.append(
            err(
                "moderation_authority.drill.expectation",
                f"{label}: expectation must be one of {sorted(EXPECTATIONS)}",
                "Set expectation to accept or reject.",
                label,
            )
        )
        return findings
    if drill.get("contract_doc_ref") != CONTRACT_DOC_REF:
        findings.append(
            err(
                "moderation_authority.drill.contract_doc",
                f"{label}: drill cites the wrong contract doc",
                f"Cite {CONTRACT_DOC_REF}.",
                label,
            )
        )

    sheet = drill.get("review_sheet", {})
    lobby = drill.get("lobby_moderation", {})
    flow = drill.get("moderation_flow", {})
    audit = drill.get("audit_export", {})

    manifest = sheet.get("invite_manifest", {})
    ticket = sheet.get("authority_ticket", {})
    schema_codes: list[str] = []
    manifest_schema_errs = list(manifest_validator.iter_errors(manifest))
    ticket_schema_errs = list(ticket_validator.iter_errors(ticket))
    if manifest_schema_errs:
        schema_codes.append("manifest_schema")
    if ticket_schema_errs:
        schema_codes.append("ticket_schema")

    model_codes = validate_review_sheet(sheet)
    lobby_codes = check_lobby(sheet, lobby)
    flow_codes = (
        check_moderation_flow(sheet, lobby, flow)
        if isinstance(flow, dict)
        else ["moderation_no_stages"]
    )
    audit_codes = (
        check_audit_export(sheet, lobby, audit)
        if isinstance(audit, dict)
        else ["audit_export_missing_field"]
    )

    all_codes = sorted(set(schema_codes + model_codes + lobby_codes + flow_codes + audit_codes))

    if expectation == "accept":
        for e in sorted(manifest_schema_errs, key=lambda x: list(x.path)):
            path = ".".join(str(p) for p in e.path) or "<root>"
            findings.append(
                err(
                    "moderation_authority.schema.manifest",
                    f"{label}: invite_manifest.{path}: {e.message}",
                    f"Fix the record so it validates against {MANIFEST_SCHEMA_REL}.",
                    label,
                )
            )
        for e in sorted(ticket_schema_errs, key=lambda x: list(x.path)):
            path = ".".join(str(p) for p in e.path) or "<root>"
            findings.append(
                err(
                    "moderation_authority.schema.ticket",
                    f"{label}: authority_ticket.{path}: {e.message}",
                    f"Fix the record so it validates against {TICKET_SCHEMA_REL}.",
                    label,
                )
            )
        if model_codes:
            findings.append(
                err(
                    "moderation_authority.model.unexpected_rejection",
                    f"{label}: accept drill is rejected by the review-sheet model port: {sorted(set(model_codes))}",
                    "Fix the review sheet so it validates, or re-classify the drill as reject.",
                    label,
                    codes=sorted(set(model_codes)),
                )
            )
        if lobby_codes:
            findings.append(
                err(
                    "moderation_authority.lobby.unexpected_rejection",
                    f"{label}: accept drill's lobby moderation is rejected: {lobby_codes}",
                    "Fix the lobby block, or re-classify the drill as reject.",
                    label,
                    codes=lobby_codes,
                )
            )
        if flow_codes:
            findings.append(
                err(
                    "moderation_authority.flow.unexpected_drift",
                    f"{label}: accept drill's moderation flow drifts from the review sheet: {flow_codes}",
                    "Re-mint the drill with --write so the flow mirrors the sheet.",
                    label,
                    codes=flow_codes,
                )
            )
        if audit_codes:
            findings.append(
                err(
                    "moderation_authority.audit.unexpected_drift",
                    f"{label}: accept drill's audit export drifts from the review sheet: {audit_codes}",
                    "Re-mint the drill with --write so the audit export mirrors the sheet.",
                    label,
                    codes=audit_codes,
                )
            )
        if drill.get("expected_rejection_codes"):
            findings.append(
                err(
                    "moderation_authority.accept.has_rejection_codes",
                    f"{label}: accept drill must not declare expected_rejection_codes",
                    "Leave expected_rejection_codes empty on accept drills.",
                    label,
                )
            )
        if drill.get("claimed_beta_row") is None:
            findings.append(
                err(
                    "moderation_authority.accept.no_claimed_row",
                    f"{label}: accept drill must map to a claimed beta/preview row",
                    "Bind each accept drill to exactly one claimed beta/preview row.",
                    label,
                )
            )
        if drill.get("lane_status_class") not in LANE_STATUS_CLASSES:
            findings.append(
                err(
                    "moderation_authority.accept.lane_status",
                    f"{label}: accept drill must declare a lane_status_class in {sorted(LANE_STATUS_CLASSES)}",
                    "Mark the lane preview_only, beta_scoped, or blocked_unresolved.",
                    label,
                )
            )
    else:  # reject
        if not all_codes:
            findings.append(
                err(
                    "moderation_authority.reject.not_rejected",
                    f"{label}: reject drill validates clean — the documented drift is not caught",
                    "Make the drill actually exercise the drift, or re-classify it as accept.",
                    label,
                )
            )
        expected = drill.get("expected_rejection_codes") or []
        if not expected:
            findings.append(
                err(
                    "moderation_authority.reject.no_expected_codes",
                    f"{label}: reject drill must declare at least one expected rejection code",
                    "Declare the typed rejection code(s) the drill proves.",
                    label,
                )
            )
        for code in expected:
            if code not in all_codes:
                findings.append(
                    err(
                        "moderation_authority.reject.missing_expected_code",
                        f"{label}: expected rejection code {code!r} not produced; got {all_codes}",
                        "Align the drill with its expected typed rejection reason.",
                        label,
                        produced=all_codes,
                    )
                )
        if drill.get("claimed_beta_row") is not None:
            findings.append(
                err(
                    "moderation_authority.reject.has_claimed_row",
                    f"{label}: reject drill must not claim a beta/preview row",
                    "Only accept drills map to claimed beta/preview rows.",
                    label,
                )
            )

    return findings


# --------------------------------------------------------------------------- #
# Coverage — the corpus must prove every required axis.
# --------------------------------------------------------------------------- #


def validate_coverage(drills: dict[str, dict[str, Any]]) -> list[Finding]:
    findings: list[Finding] = []

    accept = {k: v for k, v in drills.items() if v.get("expectation") == "accept"}
    reject = {k: v for k, v in drills.items() if v.get("expectation") == "reject"}

    lanes = {v["review_sheet"]["invite_manifest"]["session_lane_class"] for v in accept.values()}
    decisions = {v["lobby_moderation"]["moderation_decision_class"] for v in accept.values()}
    grant_states = {v["review_sheet"]["authority_ticket"]["grant_state_class"] for v in accept.values()}
    contexts = {
        v["review_sheet"]["authority_continuity"]["context_change_class"] for v in accept.values()
    }
    lifecycles = {v["lobby_moderation"]["invite_lifecycle_class"] for v in accept.values()}
    reject_codes: set[str] = set()
    for v in reject.values():
        reject_codes.update(v.get("expected_rejection_codes") or [])

    required = [
        ("lane.shared_terminal", "shared_terminal" in lanes, "Add an accept drill on the shared terminal lane."),
        ("lane.shared_debug", "shared_debug" in lanes, "Add an accept drill on the shared debug lane."),
        ("lane.presentation_preview", "presentation_preview" in lanes, "Add an accept drill on the presentation preview lane."),
        ("decision.admitted_observer", "admitted_observer" in decisions, "Add an admit-observer-first drill."),
        ("decision.admitted_with_grant", "admitted_with_grant" in decisions, "Add an admit-with-grant drill."),
        ("decision.deferred_to_lobby", "deferred_to_lobby" in decisions, "Add a deferred-to-lobby drill."),
        ("decision.denied_entry", "denied_entry" in decisions, "Add a denied-entry drill."),
        ("decision.invite_declined_by_recipient", "invite_declined_by_recipient" in decisions, "Add an invite-declined drill."),
        ("grant_state.requested", "requested" in grant_states, "Add a drill with a pending (requested) grant."),
        ("grant_state.granted_active", "granted_active" in grant_states, "Add a drill with an active grant."),
        ("grant_state.expired", "expired" in grant_states, "Add a drill with an expired grant."),
        ("grant_state.revoked", "revoked" in grant_states, "Add a drill with a revoked grant."),
        ("grant_state.frozen_pending_reapproval", "frozen_pending_reapproval" in grant_states, "Add a drill with a frozen grant."),
        ("grant_state.denied", "denied" in grant_states, "Add a drill with a denied/declined grant."),
        ("context.reconnected", "reconnected" in contexts, "Add a drill that reconnects after a grant change."),
        ("context.browser_to_desktop_handoff", "browser_to_desktop_handoff" in contexts, "Add a browser-to-desktop handoff drill."),
        ("lifecycle.accepted", "accepted" in lifecycles, "Add a drill where the invite is accepted."),
        ("lifecycle.declined", "declined" in lifecycles, "Add a drill where the invite is declined."),
        ("lifecycle.pending_in_lobby", "pending_in_lobby" in lifecycles, "Add a drill where the join waits in the lobby."),
        ("reject.moderation_revoked_silent_resume", "moderation_revoked_silent_resume" in reject_codes,
         "Add a reject drill where a revoked grant silently resumes."),
        ("reject.expired_invite_shown_active", "expired_invite_shown_active" in reject_codes,
         "Add a reject drill where an expired invite still appears active."),
        ("reject.audit_export_resolution_drift", "audit_export_resolution_drift" in reject_codes,
         "Add a reject drill where the audit export diverges from the UI."),
        ("reject.audit_export_secret_leak", "audit_export_secret_leak" in reject_codes,
         "Add a reject drill where the audit export leaks a session secret."),
        ("reject.grant_expiry_unbounded", "grant_expiry_unbounded" in reject_codes,
         "Add a reject drill with a perpetual temporary grant."),
        ("reject.silent_privileged_resume", "silent_privileged_resume" in reject_codes,
         "Add a reject drill where an active grant resumes after reconnect without reapproval."),
    ]
    for name, ok, remedy in required:
        if not ok:
            findings.append(
                err(
                    f"moderation_authority.coverage.{name}",
                    f"corpus does not prove the required axis: {name}",
                    remedy,
                )
            )
    return findings


# --------------------------------------------------------------------------- #
# Beta scorecard — every claimed row maps to exactly one accept drill, and back.
# --------------------------------------------------------------------------- #


def validate_scorecard(accept: dict[str, tuple[str, dict[str, Any]]]) -> list[Finding]:
    findings: list[Finding] = []
    scenarios = set(accept)
    expected = set(CLAIMED_BETA_ROWS)
    for scenario_id in sorted(expected - scenarios):
        findings.append(
            err(
                "moderation_authority.scorecard.missing_packet",
                f"claimed beta/preview row maps to scenario {scenario_id} but no accept drill is present",
                "Mint the missing drill with --write.",
            )
        )
    for scenario_id in sorted(scenarios - expected):
        findings.append(
            err(
                "moderation_authority.scorecard.unclaimed_packet",
                f"accept drill {scenario_id} has no claimed beta/preview row",
                "Map every accept drill to exactly one claimed beta/preview row.",
            )
        )
    rows = [drill["claimed_beta_row"] for _, drill in accept.values()]
    dupes = sorted({r for r in rows if rows.count(r) > 1})
    for row in dupes:
        findings.append(
            err(
                "moderation_authority.scorecard.duplicate_row",
                f"claimed beta/preview row {row} maps to more than one accept drill",
                "Each claimed row gets exactly one current moderation-authority packet.",
            )
        )
    return findings


# --------------------------------------------------------------------------- #
# Matrix + parity packet (build / drift-check)
# --------------------------------------------------------------------------- #


def build_matrix(drills: dict[str, tuple[str, dict[str, Any]]]) -> dict[str, Any]:
    accept_cases = []
    reject_cases = []
    claimed_rows = []
    lane_status_rollup: dict[str, set[str]] = {}
    for scenario_id, (source_fixture, drill) in sorted(drills.items()):
        sheet = drill["review_sheet"]
        manifest = sheet["invite_manifest"]
        ticket = sheet["authority_ticket"]
        lobby = drill["lobby_moderation"]
        if drill["expectation"] == "accept":
            lane = manifest["session_lane_class"]
            status = drill["lane_status_class"]
            lane_status_rollup.setdefault(lane, set()).add(status)
            accept_cases.append(
                {
                    "scenario_id": scenario_id,
                    "claimed_beta_row": drill["claimed_beta_row"],
                    "source_fixture": source_fixture,
                    "lane_class": lane,
                    "lane_status_class": status,
                    "scope_class": ticket["authority_scope_class"],
                    "capability_class": manifest["invite_capability_class"],
                    "invite_lifecycle_class": lobby["invite_lifecycle_class"],
                    "moderation_decision_class": lobby["moderation_decision_class"],
                    "grant_state_class": ticket["grant_state_class"],
                    "context_change_class": sheet["authority_continuity"]["context_change_class"],
                    "resolution_state_class": drill["audit_export"]["resolution_state_class"],
                    "stage_classes": [s["stage_class"] for s in drill["moderation_flow"]["stages"]],
                }
            )
            claimed_rows.append(
                {
                    "claimed_beta_row": drill["claimed_beta_row"],
                    "scenario_id": scenario_id,
                    "lane_status_class": status,
                }
            )
        else:
            reject_cases.append(
                {
                    "scenario_id": scenario_id,
                    "source_fixture": source_fixture,
                    "expected_rejection_codes": sorted(drill.get("expected_rejection_codes") or []),
                    "lane_class": manifest["session_lane_class"],
                    "grant_state_class": ticket["grant_state_class"],
                }
            )
    claimed_rows.sort(key=lambda r: r["claimed_beta_row"])
    rollup = {
        lane: sorted(statuses) for lane, statuses in sorted(lane_status_rollup.items())
    }
    return {
        "record_kind": MATRIX_RECORD_KIND,
        "schema_version": 1,
        "shared_contract_ref": AUDIT_CONTRACT_REF,
        "manifest_schema_ref": MANIFEST_SCHEMA_REL,
        "ticket_schema_ref": TICKET_SCHEMA_REL,
        "corpus_dir": CORPUS_DIR_REL,
        "validator_ref": VALIDATOR_REL,
        "contract_doc_ref": CONTRACT_DOC_REL,
        "lane_status_rollup": rollup,
        "claimed_beta_rows": claimed_rows,
        "accept_cases": accept_cases,
        "reject_cases": reject_cases,
    }


def build_parity_packet(accept: dict[str, tuple[str, dict[str, Any]]]) -> dict[str, Any]:
    entries = []
    for scenario_id, (source_fixture, drill) in sorted(accept.items()):
        audit = derive_audit_export(drill["review_sheet"], drill["lobby_moderation"])
        full = audit_digest(audit)
        support_text = render_support_export(audit)
        cli_line = render_cli_index(audit, Path(source_fixture).name)
        entries.append(
            {
                "scenario_id": scenario_id,
                "claimed_beta_row": drill["claimed_beta_row"],
                "source_fixture": source_fixture,
                "semantic_digest": full,
                "support_export_text": support_text,
                "cli_index_line": cli_line,
                "parity": {
                    "ui_record_full": True,
                    "support_export_full": digest_from_support_export(support_text) == full,
                    "cli_index_coarse": digest_from_cli_index(cli_line) == coarse_digest(full),
                },
            }
        )
    return {
        "record_kind": PARITY_RECORD_KIND,
        "schema_version": 1,
        "shared_contract_ref": AUDIT_CONTRACT_REF,
        "note": (
            "For every accept drill the support-bundle plaintext preserves the full "
            "semantic digest and the CLI / headless index preserves the coarse digest "
            "of the moderation audit-export record, so support, docs, and security "
            "teams can explain a moderation/authority resolution from the exported "
            "packet alone. Regenerate with "
            "`ci/check_moderation_authority_corpus.py --write`."
        ),
        "scenarios": entries,
    }


# --------------------------------------------------------------------------- #
# Loading + companions
# --------------------------------------------------------------------------- #


def load_json(path: Path) -> Any:
    try:
        return json.loads(path.read_text(encoding="utf-8"))
    except FileNotFoundError as exc:
        raise SystemExit(f"missing JSON file: {path}") from exc
    except json.JSONDecodeError as exc:
        raise SystemExit(f"invalid JSON at {path}: {exc}") from exc


def dump_json(path: Path, payload: Any) -> None:
    path.write_text(json.dumps(payload, indent=2, ensure_ascii=False) + "\n", encoding="utf-8")


def make_validator(repo_root: Path, rel: str) -> Draft202012Validator:
    schema = load_json(repo_root / rel)
    Draft202012Validator.check_schema(schema)
    return Draft202012Validator(schema, format_checker=FormatChecker())


def collect_drills(repo_root: Path) -> dict[str, tuple[str, dict[str, Any]]]:
    """scenario_id -> (repo-relative source fixture path, drill record)."""
    result: dict[str, tuple[str, dict[str, Any]]] = {}
    corpus_dir = repo_root / CORPUS_DIR_REL
    if not corpus_dir.is_dir():
        return result
    for path in sorted(corpus_dir.glob("*.json")):
        if path.name in NON_DRILL_FILES:
            continue
        record = load_json(path)
        if record.get("record_kind") != DRILL_RECORD_KIND:
            continue
        result[record["scenario_id"]] = (f"{CORPUS_DIR_REL}/{path.name}", record)
    return result


def _first_diff(a: Any, b: Any, path: str = "") -> str:
    if isinstance(a, dict) and isinstance(b, dict):
        for key in sorted(set(a) | set(b)):
            if key not in a:
                return f"{path}.{key}: missing in stored"
            if key not in b:
                return f"{path}.{key}: missing in rebuilt"
            sub = _first_diff(a[key], b[key], f"{path}.{key}")
            if sub:
                return sub
        return ""
    if isinstance(a, list) and isinstance(b, list):
        if len(a) != len(b):
            return f"{path}: length {len(a)} != {len(b)}"
        for idx, (x, y) in enumerate(zip(a, b)):
            sub = _first_diff(x, y, f"{path}[{idx}]")
            if sub:
                return sub
        return ""
    if a != b:
        return f"{path}: stored={a!r} rebuilt={b!r}"
    return ""


def validate_companion(path: Path, kind: str, expected: Any) -> list[Finding]:
    if not path.exists():
        return [
            err(
                f"moderation_authority.{kind}.missing",
                f"{kind} is missing: {path}",
                "Generate it with --write.",
                str(path),
            )
        ]
    actual = load_json(path)
    if actual != expected:
        return [
            err(
                f"moderation_authority.{kind}.drift",
                f"{kind} has drifted from the corpus; regenerate with --write",
                "Run ci/check_moderation_authority_corpus.py --write and commit the result.",
                str(path),
                diff=_first_diff(actual, expected),
            )
        ]
    return []


def validate_report_and_readme(
    report_path: Path, readme_path: Path, scenario_ids: list[str]
) -> list[Finding]:
    findings: list[Finding] = []
    required_refs = [MANIFEST_SCHEMA_REL, TICKET_SCHEMA_REL, CORPUS_DIR_REL, VALIDATOR_REL, SCRIPT_REL]
    for kind, path, refs in (
        ("report", report_path, required_refs),
        ("readme", readme_path, [VALIDATOR_REL, MANIFEST_SCHEMA_REL, TICKET_SCHEMA_REL]),
    ):
        if not path.exists():
            findings.append(
                err(
                    f"moderation_authority.{kind}.missing",
                    f"{kind} is missing: {path}",
                    f"Land the {kind} for the audit lane.",
                    str(path),
                )
            )
            continue
        text = path.read_text(encoding="utf-8")
        for ref in refs:
            if ref not in text:
                findings.append(
                    err(
                        f"moderation_authority.{kind}.missing_ref",
                        f"{kind} does not mention {ref}",
                        "Keep the report / README pointing at the schemas, corpus, validator, and script.",
                        str(path),
                    )
                )
        for scenario_id in scenario_ids:
            if scenario_id not in text:
                findings.append(
                    err(
                        f"moderation_authority.{kind}.missing_scenario",
                        f"{kind} does not mention scenario {scenario_id}",
                        "Document every drill scenario.",
                        str(path),
                    )
                )
    return findings


# --------------------------------------------------------------------------- #
# Main
# --------------------------------------------------------------------------- #


def write_corpus(repo_root: Path) -> None:
    corpus_dir = repo_root / CORPUS_DIR_REL
    corpus_dir.mkdir(parents=True, exist_ok=True)
    for drill in all_drills():
        dump_json(corpus_dir / drill.fixture_filename, drill.record())
        print(f"wrote {CORPUS_DIR_REL}/{drill.fixture_filename}")
    drills = collect_drills(repo_root)
    accept = {k: v for k, v in drills.items() if v[1]["expectation"] == "accept"}
    dump_json(repo_root / CORPUS_MATRIX_REL, build_matrix(drills))
    print(f"wrote {CORPUS_MATRIX_REL}")
    dump_json(repo_root / PARITY_PACKET_REL, build_parity_packet(accept))
    print(f"wrote {PARITY_PACKET_REL}")


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--repo-root", default=".")
    parser.add_argument(
        "--write",
        action="store_true",
        help="Regenerate the drill fixtures, corpus matrix, and parity packet.",
    )
    parser.add_argument("--report-json", default=None)
    return parser.parse_args()


def main() -> int:
    args = parse_args()
    repo_root = Path(args.repo_root).resolve()

    if args.write:
        write_corpus(repo_root)
        return 0

    for rel in (MANIFEST_SCHEMA_REL, TICKET_SCHEMA_REL):
        if not (repo_root / rel).exists():
            print(f"[moderation-authority] missing schema: {repo_root / rel}", file=sys.stderr)
            return 2

    manifest_validator = make_validator(repo_root, MANIFEST_SCHEMA_REL)
    ticket_validator = make_validator(repo_root, TICKET_SCHEMA_REL)

    drills = collect_drills(repo_root)
    if not drills:
        print("[moderation-authority] no moderation-authority drills found", file=sys.stderr)
        return 2

    findings: list[Finding] = []
    flat: dict[str, dict[str, Any]] = {}
    for scenario_id, (source_fixture, drill) in sorted(drills.items()):
        flat[scenario_id] = drill
        findings.extend(check_drill(source_fixture, drill, manifest_validator, ticket_validator))

    findings.extend(validate_coverage(flat))

    accept = {k: v for k, v in drills.items() if v[1]["expectation"] == "accept"}
    findings.extend(check_parity(accept))
    findings.extend(validate_scorecard(accept))
    findings.extend(
        validate_companion(repo_root / CORPUS_MATRIX_REL, "corpus_matrix", build_matrix(drills))
    )
    findings.extend(
        validate_companion(repo_root / PARITY_PACKET_REL, "parity_packet", build_parity_packet(accept))
    )

    scenario_ids = sorted(drills)
    findings.extend(
        validate_report_and_readme(repo_root / REPORT_REL, repo_root / README_REL, scenario_ids)
    )

    errors = [f for f in findings if f.severity == "error"]
    report = {
        "lane": "collaboration_moderation_and_authority_escrow_corpus",
        "manifest_schema": MANIFEST_SCHEMA_REL,
        "ticket_schema": TICKET_SCHEMA_REL,
        "corpus_dir": CORPUS_DIR_REL,
        "drill_count": len(drills),
        "accept_count": len(accept),
        "reject_count": len(drills) - len(accept),
        "ok": not errors,
        "error_count": len(errors),
        "findings": [f.as_report() for f in findings],
    }
    if args.report_json:
        out = Path(args.report_json)
        out.parent.mkdir(parents=True, exist_ok=True)
        out.write_text(json.dumps(report, indent=2) + "\n", encoding="utf-8")

    if errors:
        print(
            f"[moderation-authority] FAIL: {len(errors)} error(s) across {len(drills)} drill(s)",
            file=sys.stderr,
        )
        for f in errors:
            print(f"  - {f.check_id}: {f.message}", file=sys.stderr)
        return 1

    print(
        f"[moderation-authority] PASS: {len(drills)} drill(s) validated "
        f"({len(accept)} accept, {len(drills) - len(accept)} reject), "
        "re-derived, parity- and scorecard-checked"
    )
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
