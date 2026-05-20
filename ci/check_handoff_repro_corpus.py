#!/usr/bin/env python3
"""Validate the handoff & reproduction-packet corpus.

This is the release-engineering / public-proof audit lane that proves Aureline
cannot quietly leak context, lose reproduction data, or mislabel
support/community/security boundaries on the marketed beta handoff rows. It is
the proof packet a release, support, or security team attaches to a claimed
handoff lane: one current drill per claimed beta row that a reviewer can read
and explain without rebuilding live shell state.

It composes — it does not fork — the `HandoffReviewSheet` model the desktop
shell uses to gate every issue / security-disclosure / docs-feedback /
RFC-discussion / community-support handoff before a report leaves the product
(see crates/aureline-shell/src/handoff_review/mod.rs and the boundary schemas
schemas/public/handoff_target_review.schema.json and
schemas/public/repro_packet_preview.schema.json). On top of the model it adds
the public-proof drills this lane requires:

- **Target identity & visibility.** Every drill pins a typed handoff route and
  one of the five visibility classes (official public, official private,
  security disclosure, community, third-party / vendor). A route may only target
  a visibility from its allowed set, so a security report can never be coerced
  onto a public tracker and the lanes never blur together.

- **Preview before share.** A handoff only opens the system browser after the
  reproduction-packet preview is confirmed. A drill that shares without a
  confirmed preview fails the lane.

- **Redaction posture safe for the target.** The previewed redaction posture
  must be safe for the chosen visibility — a world-readable target may not carry
  a support- or security-scoped payload.

- **Exact-anchor / build-context continuity.** Each drill carries a continuity
  flow (prepare → preview → block → retry → export → reopen). The redaction
  posture, target visibility, exact anchor / object identity, versioned
  build-context export blocks, and the previewed-shareable diagnostic / attachment
  set are derived from the review sheet and must survive every stage unchanged.
  An export stage that carries a diagnostic the preview did not show as included
  (a field omitted from the review sheet but still exported), an anchor that
  drifts on reopen, or a blocked handoff that drops the preserved draft fails the
  lane.

- **Browser-blocked / offline preservation.** Browser-blocked, offline, and
  policy-denied outcomes preserve the draft text, attachments, target class, and
  redaction posture with export / save actions instead of silent loss.

- **Support-boundary copy honesty.** The reviewer-facing support-boundary copy
  is derived from the target review and must match it: it can never describe a
  private / security route as world-readable, nor name a different data exit or
  network requirement than the one the handoff actually uses.

For every **accept** drill the validator schema-validates the two constituent
records against their boundary schemas, re-runs an independent Python port of
`HandoffReviewSheet::validate()` (a second implementation, so a regression in
either the Rust model or a fixture fails the lane), re-derives the continuity
carried-identity and the support-boundary copy from the sheet and drift-checks
the stored values, then proves export parity (support-bundle plaintext and CLI /
headless index preserve the record semantics) and that every claimed beta row
maps to exactly one packet. For every **reject** drill it proves the documented
drift is actually rejected, with the expected typed reason.

Run via scripts/ci/run_handoff_repro_corpus.sh. Use --write to (re)mint the
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

TARGET_SCHEMA_REL = "schemas/public/handoff_target_review.schema.json"
PACKET_SCHEMA_REL = "schemas/public/repro_packet_preview.schema.json"
CORPUS_DIR_REL = "fixtures/public/m3/handoff_repro_corpus"
CORPUS_MATRIX_REL = f"{CORPUS_DIR_REL}/corpus_matrix.json"
PARITY_PACKET_REL = f"{CORPUS_DIR_REL}/export_parity_packet.json"
README_REL = f"{CORPUS_DIR_REL}/README.md"
REPORT_REL = "artifacts/public/m3/handoff_repro_report.md"
VALIDATOR_REL = "ci/check_handoff_repro_corpus.py"
SCRIPT_REL = "scripts/ci/run_handoff_repro_corpus.sh"
CONTRACT_DOC_REL = "docs/public/m3/handoff_and_repro_boundary.md"

AUDIT_CONTRACT_REF = "public:handoff_repro_corpus:v1"
DRILL_RECORD_KIND = "handoff_repro_drill_record"
MATRIX_RECORD_KIND = "handoff_repro_corpus_matrix"
PARITY_RECORD_KIND = "handoff_repro_export_parity_packet"
DRILL_SCHEMA_VERSION = 1

NON_DRILL_FILES = {"corpus_matrix.json", "export_parity_packet.json", "README.md"}

# Frozen contract-doc ref every constituent record must cite.
CONTRACT_DOC_REF = "docs/public/m3/handoff_and_repro_boundary.md"

# --------------------------------------------------------------------------- #
# Closed vocabularies — a deliberate second copy of the Rust enums in
# crates/aureline-shell/src/handoff_review/mod.rs. Keep in lockstep.
# --------------------------------------------------------------------------- #

TARGET_VISIBILITY_LABEL = {
    "official_public": "Official public",
    "official_private": "Official private",
    "security_disclosure": "Security disclosure",
    "community": "Community",
    "third_party_vendor": "Third-party / vendor",
}
PUBLIC_VISIBILITIES = {"official_public", "community"}

ROUTE_ALLOWED_VISIBILITY = {
    "public_issue": {"official_public", "community"},
    "security_disclosure": {"security_disclosure", "official_private"},
    "docs_feedback": {"official_public", "community"},
    "rfc_discussion": {"community", "official_public"},
    "community_support": {"community", "official_private"},
}

NETWORK_REQUIREMENTS = {
    "offline_capture_preview",
    "system_browser_public_browse",
    "system_browser_authenticated_plane",
    "encrypted_security_channel",
    "vendor_or_third_party_call",
}

DATA_EXIT_CLASSES = {
    "no_payload_leaves_product",
    "metadata_safe_object_refs",
    "proposal_refs_only",
    "redacted_support_packet",
    "security_payloads_only",
    "external_public_browse",
    "vendor_or_third_party_outbound",
}

REDACTION_POSTURES = {
    "fully_redacted_public_safe",
    "redacted_support_scoped",
    "security_channel_only",
    "metadata_refs_only",
}
PUBLIC_SAFE_POSTURES = {"fully_redacted_public_safe", "metadata_refs_only"}

HANDOFF_OUTCOMES = {
    "preview_pending_confirmation",
    "opened_in_system_browser",
    "browser_blocked",
    "offline",
    "policy_denied",
    "target_permission_denied",
}
BLOCKED_OUTCOMES = {
    "browser_blocked",
    "offline",
    "policy_denied",
    "target_permission_denied",
}

PRESERVATION_ACTIONS = {
    "export_packet",
    "save_draft_local",
    "retry_when_online",
    "copy_refs_to_clipboard",
    "discard",
}

# Continuity-flow stage vocabulary.
STAGE_CLASSES = {
    "prepared",
    "preview_confirmed",
    "browser_blocked",
    "offline_captured",
    "policy_denied",
    "retry",
    "export",
    "reopen",
}
# Stages that come after the handoff has been attempted / blocked: the preserved
# draft must still be intact at every one of them.
POST_ATTEMPT_STAGES = {
    "browser_blocked",
    "offline_captured",
    "policy_denied",
    "retry",
    "export",
    "reopen",
}

EXPECTATIONS = {"accept", "reject"}


def visibility_is_public(visibility: str) -> bool:
    return visibility in PUBLIC_VISIBILITIES


def posture_is_public_safe(posture: str) -> bool:
    return posture in PUBLIC_SAFE_POSTURES


def posture_allowed_for_visibility(posture: str, visibility: str) -> bool:
    if visibility in {"official_public", "community", "third_party_vendor"}:
        return posture_is_public_safe(posture)
    if visibility == "official_private":
        return posture in {
            "redacted_support_scoped",
            "metadata_refs_only",
            "fully_redacted_public_safe",
        }
    if visibility == "security_disclosure":
        return posture == "security_channel_only"
    return False


def visibility_allows_data_exit(visibility: str, data_exit: str) -> bool:
    if visibility in {"official_public", "community"}:
        return data_exit in {
            "no_payload_leaves_product",
            "metadata_safe_object_refs",
            "proposal_refs_only",
            "external_public_browse",
        }
    if visibility == "official_private":
        return data_exit in {
            "redacted_support_packet",
            "metadata_safe_object_refs",
            "no_payload_leaves_product",
        }
    if visibility == "security_disclosure":
        return data_exit == "security_payloads_only"
    if visibility == "third_party_vendor":
        return data_exit in {"external_public_browse", "vendor_or_third_party_outbound"}
    return False


def visibility_allows_network(visibility: str, network: str) -> bool:
    if visibility in {"official_public", "community"}:
        return network in {"offline_capture_preview", "system_browser_public_browse"}
    if visibility == "official_private":
        return network in {
            "offline_capture_preview",
            "system_browser_authenticated_plane",
        }
    if visibility == "security_disclosure":
        return network in {"offline_capture_preview", "encrypted_security_channel"}
    if visibility == "third_party_vendor":
        return network in {"offline_capture_preview", "vendor_or_third_party_call"}
    return False


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
# Independent port of HandoffReviewSheet::validate(). Returns the full set of
# typed rejection codes (snake_case of the Rust error variants). An empty list
# means the sheet validates.
# --------------------------------------------------------------------------- #


def validate_review_sheet(sheet: dict[str, Any]) -> list[str]:
    codes: list[str] = []

    if sheet.get("handoff_review_sheet_schema_version") != 1:
        codes.append("wrong_sheet_schema_version")
    if sheet.get("record_kind") != "handoff_review_sheet_record":
        codes.append("wrong_sheet_record_kind")
    sheet_id = sheet.get("sheet_id", "")
    if not (isinstance(sheet_id, str) and sheet_id.startswith("handoff_review_sheet:")):
        codes.append("malformed_sheet_id")
    if sheet.get("contract_doc_ref") != CONTRACT_DOC_REF:
        codes.append("wrong_contract_doc_ref")
    if not non_empty(sheet.get("sheet_summary")):
        codes.append("empty_required_field")

    target = sheet.get("target_review", {})
    packet = sheet.get("repro_packet_preview", {})
    draft = sheet.get("draft_continuity", {})

    codes.extend(_validate_target(target))
    codes.extend(_validate_packet(packet))
    codes.extend(_validate_draft(draft))

    visibility = target.get("visibility_class")
    posture = packet.get("redaction_posture_class")

    # Cross-record: redaction posture must be safe for the chosen visibility.
    if visibility in TARGET_VISIBILITY_LABEL and posture in REDACTION_POSTURES:
        if not posture_allowed_for_visibility(posture, visibility):
            codes.append("redaction_posture_unsafe_for_visibility")

    # Cross-record: the preserved draft mirrors target class and redaction.
    if draft.get("preserved_visibility_class") != visibility:
        codes.append("preserved_visibility_mismatch")
    if draft.get("preserved_redaction_posture_class") != posture:
        codes.append("preserved_redaction_mismatch")

    # Cross-record: the browser only opens after the preview is confirmed.
    if (
        draft.get("handoff_outcome_class") == "opened_in_system_browser"
        and not packet.get("preview_confirmed_before_share")
    ):
        codes.append("shared_without_preview_confirmation")

    # Cross-record: a selected fallback is one the target actually offered.
    selected = draft.get("selected_fallback_ref")
    if selected is not None and selected not in (target.get("safe_fallback_refs") or []):
        codes.append("selected_fallback_not_offered")

    return codes


def _validate_target(target: dict[str, Any]) -> list[str]:
    codes: list[str] = []
    if target.get("handoff_target_review_schema_version") != 1:
        codes.append("wrong_target_schema_version")
    if target.get("record_kind") != "handoff_target_review_record":
        codes.append("wrong_target_record_kind")
    target_id = target.get("target_id", "")
    if not (isinstance(target_id, str) and target_id.startswith("handoff_target:")):
        codes.append("malformed_target_id")
    if target.get("contract_doc_ref") != CONTRACT_DOC_REF:
        codes.append("wrong_contract_doc_ref")
    for value in (
        target.get("headline_label"),
        target.get("target_summary"),
        target.get("destination_label"),
    ):
        if not non_empty(value):
            codes.append("empty_required_field")
    if not ref_is_opaque(target.get("destination_identity_ref")):
        codes.append("raw_ref_leak")

    fallbacks = target.get("safe_fallback_refs") or []
    if not fallbacks:
        codes.append("missing_safe_fallback")
    for fallback in fallbacks:
        if not ref_is_opaque(fallback):
            codes.append("raw_ref_leak")

    route = target.get("route_class")
    visibility = target.get("visibility_class")
    if route in ROUTE_ALLOWED_VISIBILITY and visibility in TARGET_VISIBILITY_LABEL:
        if visibility not in ROUTE_ALLOWED_VISIBILITY[route]:
            codes.append("route_visibility_mismatch")
    data_exit = target.get("data_exit_boundary_class")
    network = target.get("network_browser_requirement_class")
    if visibility in TARGET_VISIBILITY_LABEL and data_exit in DATA_EXIT_CLASSES:
        if not visibility_allows_data_exit(visibility, data_exit):
            codes.append("visibility_data_exit_mismatch")
    if visibility in TARGET_VISIBILITY_LABEL and network in NETWORK_REQUIREMENTS:
        if not visibility_allows_network(visibility, network):
            codes.append("visibility_network_mismatch")

    exports = target.get("build_context_exports") or []
    if not exports:
        codes.append("missing_build_context_export")
    for export in exports:
        if not isinstance(export, dict):
            codes.append("build_context_export_field_empty")
            continue
        if int(export.get("export_block_schema_version", 0)) < 1:
            codes.append("build_context_export_schema_version_invalid")
        if not (export.get("raw_screenshots_excluded") and export.get("raw_secrets_excluded")):
            codes.append("build_context_export_not_redaction_safe")
        if not (
            non_empty(export.get("export_block_ref"))
            and non_empty(export.get("export_summary"))
        ):
            codes.append("build_context_export_field_empty")
    return codes


def _validate_packet(packet: dict[str, Any]) -> list[str]:
    codes: list[str] = []
    if packet.get("repro_packet_preview_schema_version") != 1:
        codes.append("wrong_packet_schema_version")
    if packet.get("record_kind") != "repro_packet_preview_record":
        codes.append("wrong_packet_record_kind")
    packet_id = packet.get("packet_id", "")
    if not (isinstance(packet_id, str) and packet_id.startswith("repro_packet_preview:")):
        codes.append("malformed_packet_id")
    if packet.get("contract_doc_ref") != CONTRACT_DOC_REF:
        codes.append("wrong_contract_doc_ref")
    for value in (packet.get("headline_label"), packet.get("packet_summary")):
        if not non_empty(value):
            codes.append("empty_required_field")
    if not (packet.get("raw_secrets_excluded") and packet.get("raw_screenshots_excluded")):
        codes.append("raw_payload_not_excluded")

    diagnostics = packet.get("selected_diagnostics") or []
    if not diagnostics or not any(d.get("included") for d in diagnostics):
        codes.append("no_diagnostics_selected")
    for diagnostic in diagnostics:
        if not non_empty(diagnostic.get("summary")):
            codes.append("empty_required_field")

    for attachment in packet.get("attachments") or []:
        if not ref_is_opaque(attachment.get("attachment_ref")):
            codes.append("raw_ref_leak")
        if not non_empty(attachment.get("summary")):
            codes.append("empty_required_field")
        if not attachment.get("redaction_applied"):
            codes.append("attachment_not_redacted")

    anchor = packet.get("anchor_identity", {})
    if not (ref_is_opaque(anchor.get("anchor_ref")) and ref_is_opaque(anchor.get("object_ref"))):
        codes.append("raw_ref_leak")
    if not non_empty(anchor.get("anchor_label")):
        codes.append("empty_required_field")
    return codes


def _validate_draft(draft: dict[str, Any]) -> list[str]:
    codes: list[str] = []
    if not non_empty(draft.get("continuity_summary")):
        codes.append("empty_required_field")
    if draft.get("silent_loss"):
        codes.append("silent_loss_not_allowed")
    actions = draft.get("available_actions") or []
    if not actions:
        codes.append("no_preservation_actions")
    for ref in draft.get("preserved_attachment_refs") or []:
        if not ref_is_opaque(ref):
            codes.append("raw_ref_leak")
    text_ref = draft.get("preserved_draft_text_ref")
    if text_ref is not None and not ref_is_opaque(text_ref):
        codes.append("raw_ref_leak")
    selected = draft.get("selected_fallback_ref")
    if selected is not None and not ref_is_opaque(selected):
        codes.append("raw_ref_leak")

    outcome = draft.get("handoff_outcome_class")
    if outcome in BLOCKED_OUTCOMES:
        if not draft.get("intent_preserved"):
            codes.append("blocked_handoff_dropped_intent")
        if draft.get("preserved_draft_text_ref") is None:
            codes.append("blocked_handoff_missing_draft_text")
        if not ("export_packet" in actions and "save_draft_local" in actions):
            codes.append("blocked_handoff_missing_preservation_actions")
    return codes


# --------------------------------------------------------------------------- #
# Derivation: continuity carried-identity and support-boundary copy are pure
# functions of the review sheet. The stored values in every drill are drift-
# checked against these, so a fixture can never quietly diverge from the sheet.
# --------------------------------------------------------------------------- #


def derive_carried_identity(sheet: dict[str, Any]) -> dict[str, Any]:
    target = sheet["target_review"]
    packet = sheet["repro_packet_preview"]
    anchor = packet["anchor_identity"]
    return {
        "redaction_posture_class": packet["redaction_posture_class"],
        "visibility_class": target["visibility_class"],
        "anchor_ref": anchor["anchor_ref"],
        "object_ref": anchor["object_ref"],
        "build_context_export_refs": sorted(
            e["export_block_ref"] for e in target["build_context_exports"]
        ),
        "shareable_diagnostic_kinds": sorted(
            d["kind_class"] for d in packet["selected_diagnostics"] if d.get("included")
        ),
        "attachment_refs": sorted(a["attachment_ref"] for a in packet["attachments"]),
    }


def derive_support_copy(sheet: dict[str, Any]) -> dict[str, Any]:
    target = sheet["target_review"]
    visibility = target["visibility_class"]
    return {
        "claimed_visibility_class": visibility,
        "support_scope_label": TARGET_VISIBILITY_LABEL[visibility],
        "claimed_data_exit_boundary_class": target["data_exit_boundary_class"],
        "claimed_network_browser_requirement_class": target[
            "network_browser_requirement_class"
        ],
        "names_world_readable": visibility_is_public(visibility),
        "copy_text": support_copy_text(target),
    }


def support_copy_text(target: dict[str, Any]) -> str:
    visibility = target["visibility_class"]
    label = TARGET_VISIBILITY_LABEL[visibility]
    reach = "world-readable once shared" if visibility_is_public(visibility) else "not world-readable"
    return (
        f"This handoff is {label} ({reach}); it exits as "
        f"{target['data_exit_boundary_class']} over "
        f"{target['network_browser_requirement_class']}."
    )


def build_continuity_flow(sheet: dict[str, Any], stage_classes: list[str]) -> dict[str, Any]:
    identity = derive_carried_identity(sheet)
    stages = []
    for stage_class in stage_classes:
        stages.append(
            {
                "stage_class": stage_class,
                "redaction_posture_class": identity["redaction_posture_class"],
                "visibility_class": identity["visibility_class"],
                "anchor_ref": identity["anchor_ref"],
                "object_ref": identity["object_ref"],
                "build_context_export_refs": list(identity["build_context_export_refs"]),
                "carried_diagnostic_kinds": list(identity["shareable_diagnostic_kinds"]),
                "attachment_refs": list(identity["attachment_refs"]),
                "draft_preserved": True,
                "summary": STAGE_SUMMARY[stage_class],
            }
        )
    return {"carried_identity": identity, "stages": stages}


STAGE_SUMMARY = {
    "prepared": "Packet assembled locally; nothing has left the product yet.",
    "preview_confirmed": "User confirmed the reproduction-packet preview before any share.",
    "browser_blocked": "System browser could not open; the prepared context is held.",
    "offline_captured": "Captured offline; the packet is built and previewed without leaving the product.",
    "policy_denied": "Managed policy denied the handoff; the prepared context is held.",
    "retry": "Retry after the block; the same context is re-presented unchanged.",
    "export": "User exported the packet; only the previewed-shareable fields are written.",
    "reopen": "User reopened the saved draft; the full identity is restored unchanged.",
}


# --------------------------------------------------------------------------- #
# Continuity-flow + support-copy checks (the corpus-specific drills).
# --------------------------------------------------------------------------- #


def check_continuity(label: str, sheet: dict[str, Any], flow: dict[str, Any]) -> list[str]:
    codes: list[str] = []
    expected_identity = derive_carried_identity(sheet)
    stored_identity = flow.get("carried_identity", {})
    if stored_identity != expected_identity:
        codes.append("continuity_carried_identity_drift")
        # Use the stored identity for per-stage comparison anyway, so a stage that
        # also drifts is still reported.
    identity = stored_identity or expected_identity

    stages = flow.get("stages") or []
    if not stages:
        codes.append("continuity_no_stages")
        return codes

    seen_classes = [s.get("stage_class") for s in stages]
    for stage_class in seen_classes:
        if stage_class not in STAGE_CLASSES:
            codes.append("continuity_unknown_stage")

    shareable = set(identity.get("shareable_diagnostic_kinds", []))
    attachments = set(identity.get("attachment_refs", []))
    for stage in stages:
        if stage.get("redaction_posture_class") != identity.get("redaction_posture_class"):
            codes.append("continuity_redaction_drift")
        if stage.get("visibility_class") != identity.get("visibility_class"):
            codes.append("continuity_visibility_drift")
        if (
            stage.get("anchor_ref") != identity.get("anchor_ref")
            or stage.get("object_ref") != identity.get("object_ref")
        ):
            codes.append("continuity_anchor_drift")
        if sorted(stage.get("build_context_export_refs", [])) != sorted(
            identity.get("build_context_export_refs", [])
        ):
            codes.append("continuity_build_context_drift")
        # A stage may carry a subset of the previewed-shareable diagnostics, never
        # a diagnostic the preview did not show as included.
        carried = set(stage.get("carried_diagnostic_kinds", []))
        if not carried <= shareable:
            codes.append("continuity_preview_export_mismatch")
        if not set(stage.get("attachment_refs", [])) <= attachments:
            codes.append("continuity_attachment_export_mismatch")
        # Once the handoff has been attempted / blocked, the preserved draft must
        # survive every later stage.
        if stage.get("stage_class") in POST_ATTEMPT_STAGES and not stage.get("draft_preserved"):
            codes.append("continuity_blocked_context_loss")
    return sorted(set(codes))


def check_support_copy(label: str, sheet: dict[str, Any], copy: dict[str, Any]) -> list[str]:
    codes: list[str] = []
    expected = derive_support_copy(sheet)
    target = sheet["target_review"]
    if copy.get("claimed_visibility_class") != target["visibility_class"]:
        codes.append("support_copy_visibility_drift")
    if copy.get("claimed_data_exit_boundary_class") != target["data_exit_boundary_class"]:
        codes.append("support_copy_data_exit_drift")
    if (
        copy.get("claimed_network_browser_requirement_class")
        != target["network_browser_requirement_class"]
    ):
        codes.append("support_copy_network_drift")
    if bool(copy.get("names_world_readable")) != visibility_is_public(target["visibility_class"]):
        codes.append("support_copy_world_readable_drift")
    if copy.get("support_scope_label") != expected["support_scope_label"]:
        codes.append("support_copy_label_drift")
    if not non_empty(copy.get("copy_text")):
        codes.append("support_copy_empty")
    return sorted(set(codes))


# --------------------------------------------------------------------------- #
# Drill definitions — six accept drills (one per claimed beta row) and six
# reject drills (one per documented drift axis).
# --------------------------------------------------------------------------- #


def build_context_export(export_class: str, ref: str, summary: str) -> dict[str, Any]:
    return {
        "export_class": export_class,
        "export_block_ref": ref,
        "export_block_schema_version": 1,
        "redacted_for_audience": export_class,
        "raw_screenshots_excluded": True,
        "raw_secrets_excluded": True,
        "export_summary": summary,
    }


def diagnostic(kind: str, included: bool, summary: str) -> dict[str, Any]:
    return {"kind_class": kind, "included": included, "summary": summary}


def attachment(kind: str, ref: str, summary: str) -> dict[str, Any]:
    return {"kind_class": kind, "attachment_ref": ref, "redaction_applied": True, "summary": summary}


def make_sheet(
    *,
    slug: str,
    sheet_summary: str,
    route_class: str,
    visibility_class: str,
    destination_identity_ref: str,
    destination_label: str,
    network: str,
    data_exit: str,
    safe_fallback_refs: list[str],
    exports: list[dict[str, Any]],
    target_headline: str,
    target_summary: str,
    posture: str,
    diagnostics: list[dict[str, Any]],
    attachments: list[dict[str, Any]],
    anchor_ref: str,
    object_ref: str,
    anchor_label: str,
    preview_confirmed: bool,
    packet_headline: str,
    packet_summary: str,
    outcome: str,
    intent_preserved: bool,
    preserved_draft_text_ref: str | None,
    available_actions: list[str],
    selected_fallback_ref: str | None,
    continuity_summary: str,
) -> dict[str, Any]:
    return {
        "handoff_review_sheet_schema_version": 1,
        "record_kind": "handoff_review_sheet_record",
        "sheet_id": f"handoff_review_sheet:{slug}",
        "sheet_summary": sheet_summary,
        "target_review": {
            "handoff_target_review_schema_version": 1,
            "record_kind": "handoff_target_review_record",
            "target_id": f"handoff_target:{slug}",
            "route_class": route_class,
            "visibility_class": visibility_class,
            "destination_identity_ref": destination_identity_ref,
            "destination_label": destination_label,
            "network_browser_requirement_class": network,
            "data_exit_boundary_class": data_exit,
            "safe_fallback_refs": safe_fallback_refs,
            "build_context_exports": exports,
            "headline_label": target_headline,
            "target_summary": target_summary,
            "contract_doc_ref": CONTRACT_DOC_REF,
            "notes": None,
        },
        "repro_packet_preview": {
            "repro_packet_preview_schema_version": 1,
            "record_kind": "repro_packet_preview_record",
            "packet_id": f"repro_packet_preview:{slug}",
            "redaction_posture_class": posture,
            "selected_diagnostics": diagnostics,
            "attachments": attachments,
            "anchor_identity": {
                "anchor_ref": anchor_ref,
                "object_ref": object_ref,
                "anchor_label": anchor_label,
            },
            "preview_confirmed_before_share": preview_confirmed,
            "raw_secrets_excluded": True,
            "raw_screenshots_excluded": True,
            "headline_label": packet_headline,
            "packet_summary": packet_summary,
            "contract_doc_ref": CONTRACT_DOC_REF,
            "notes": None,
        },
        "draft_continuity": {
            "handoff_outcome_class": outcome,
            "intent_preserved": intent_preserved,
            "silent_loss": False,
            "preserved_draft_text_ref": preserved_draft_text_ref,
            "preserved_attachment_refs": sorted(a["attachment_ref"] for a in attachments),
            "preserved_visibility_class": visibility_class,
            "preserved_redaction_posture_class": posture,
            "available_actions": available_actions,
            "selected_fallback_ref": selected_fallback_ref,
            "continuity_summary": continuity_summary,
        },
        "contract_doc_ref": CONTRACT_DOC_REF,
        "notes": None,
    }


@dataclass
class Drill:
    scenario_id: str
    fixture_filename: str
    claimed_beta_row: str | None
    expectation: str
    title: str
    summary: str
    sheet: dict[str, Any]
    stage_classes: list[str]
    expected_rejection_codes: list[str] = field(default_factory=list)
    # Optional post-build mutators applied to inject documented drift.
    flow_override: dict[str, Any] | None = None
    support_override: dict[str, Any] | None = None

    def record(self) -> dict[str, Any]:
        flow = self.flow_override if self.flow_override is not None else build_continuity_flow(
            self.sheet, self.stage_classes
        )
        support = (
            self.support_override
            if self.support_override is not None
            else derive_support_copy(self.sheet)
        )
        rec = {
            "record_kind": DRILL_RECORD_KIND,
            "schema_version": DRILL_SCHEMA_VERSION,
            "drill_id": f"handoff_repro_drill:{self.scenario_id}",
            "scenario_id": self.scenario_id,
            "claimed_beta_row": self.claimed_beta_row,
            "expectation": self.expectation,
            "expected_rejection_codes": sorted(self.expected_rejection_codes),
            "title": self.title,
            "summary": self.summary,
            "review_sheet": self.sheet,
            "continuity_flow": flow,
            "support_boundary_copy": support,
            "contract_doc_ref": CONTRACT_DOC_REF,
            "narrative_refs": [CONTRACT_DOC_REL],
        }
        return rec


def accept_drills() -> list[Drill]:
    public_issue = make_sheet(
        slug="public_issue_after_preview",
        sheet_summary="Public issue handoff opened after the reproduction-packet preview was confirmed.",
        route_class="public_issue",
        visibility_class="official_public",
        destination_identity_ref="about_destination:issue.tracker.public",
        destination_label="Public issue tracker",
        network="system_browser_public_browse",
        data_exit="metadata_safe_object_refs",
        safe_fallback_refs=[
            "about_destination:local.fallback.docs_pack",
            "handoff_fallback:save_draft_local",
        ],
        exports=[
            build_context_export(
                "public_issue_template_block",
                "build_context_export:public_issue.v1",
                "Build identity refs only; no raw screenshots or secrets.",
            )
        ],
        target_headline="File a public issue",
        target_summary="Opens the public issue tracker in the system browser; browsing is anonymous, commenting needs a community account.",
        posture="fully_redacted_public_safe",
        diagnostics=[
            diagnostic("build_identity", True, "Build channel, version, and commit refs."),
            diagnostic("repro_steps_text", True, "Reproduction steps the user wrote."),
            diagnostic("redacted_log_tail", False, "Tail of the redacted session log (excluded by the user)."),
        ],
        attachments=[
            attachment(
                "build_context_export_block",
                "build_context_export:public_issue.v1",
                "Redacted build-context export block.",
            )
        ],
        anchor_ref="anchor:docs.page.boundary_truth.section.3",
        object_ref="object:docs.page.boundary_truth",
        anchor_label="Boundary-truth docs page, section 3",
        preview_confirmed=True,
        packet_headline="Reproduction packet preview",
        packet_summary="Build identity and repro steps with one redacted export block; the redacted log tail is excluded.",
        outcome="opened_in_system_browser",
        intent_preserved=True,
        preserved_draft_text_ref="draft_text:public_issue.body",
        available_actions=["export_packet", "save_draft_local", "copy_refs_to_clipboard"],
        selected_fallback_ref=None,
        continuity_summary="Opened in the system browser after the preview was confirmed; the draft and export block stay saved locally.",
    )

    docs_feedback = make_sheet(
        slug="docs_feedback_browser_blocked",
        sheet_summary="Docs-feedback handoff whose system browser is blocked; the prepared context survives across retry, export, and reopen.",
        route_class="docs_feedback",
        visibility_class="official_public",
        destination_identity_ref="about_destination:docs.feedback.public",
        destination_label="Docs feedback channel",
        network="system_browser_public_browse",
        data_exit="metadata_safe_object_refs",
        safe_fallback_refs=[
            "about_destination:local.fallback.docs_pack",
            "handoff_fallback:save_draft_local",
        ],
        exports=[
            build_context_export(
                "public_issue_template_block",
                "build_context_export:docs_feedback.v1",
                "Docs feedback block carries build identity refs only.",
            )
        ],
        target_headline="Send docs feedback",
        target_summary="Opens the public docs-feedback channel in the system browser.",
        posture="fully_redacted_public_safe",
        diagnostics=[
            diagnostic("build_identity", True, "Build channel and version refs."),
            diagnostic("anchor_object_ref", True, "Anchor / object refs for the docs page."),
        ],
        attachments=[
            attachment(
                "build_context_export_block",
                "build_context_export:docs_feedback.v1",
                "Redacted docs-feedback export block.",
            )
        ],
        anchor_ref="anchor:docs.page.install.step.2",
        object_ref="object:docs.page.install",
        anchor_label="Install docs page, step 2",
        preview_confirmed=True,
        packet_headline="Docs feedback reproduction packet",
        packet_summary="Build identity and the exact docs anchor; nothing leaves the metadata boundary.",
        outcome="browser_blocked",
        intent_preserved=True,
        preserved_draft_text_ref="draft_text:docs_feedback.body",
        available_actions=["export_packet", "save_draft_local", "retry_when_online", "discard"],
        selected_fallback_ref="handoff_fallback:save_draft_local",
        continuity_summary="The system browser could not open. The draft, the export block, and the exact anchor are preserved with export, save, retry, and discard.",
    )

    community_offline = make_sheet(
        slug="community_support_offline_capture",
        sheet_summary="Community-support handoff captured offline: the packet is built and previewed without leaving the product.",
        route_class="community_support",
        visibility_class="community",
        destination_identity_ref="about_destination:community.forum.public",
        destination_label="Community support forum",
        network="offline_capture_preview",
        data_exit="metadata_safe_object_refs",
        safe_fallback_refs=[
            "about_destination:local.fallback.docs_pack",
            "handoff_fallback:save_draft_local",
        ],
        exports=[
            build_context_export(
                "community_discussion_block",
                "build_context_export:community_discussion.v1",
                "Community discussion block carries build identity refs only.",
            )
        ],
        target_headline="Ask the community",
        target_summary="Builds the community-support packet offline and previews it before any browser opens.",
        posture="metadata_refs_only",
        diagnostics=[
            diagnostic("build_identity", True, "Build channel and version refs."),
            diagnostic("environment_capsule", True, "Sanitized environment capsule refs."),
        ],
        attachments=[
            attachment(
                "anchor_object_snapshot",
                "object:workspace.setting.theme",
                "Anchor object snapshot, metadata refs only.",
            )
        ],
        anchor_ref="anchor:settings.theme.density",
        object_ref="object:workspace.setting.theme",
        anchor_label="Theme density setting",
        preview_confirmed=True,
        packet_headline="Community reproduction packet",
        packet_summary="Build identity and environment capsule refs only; nothing leaves the metadata boundary.",
        outcome="offline",
        intent_preserved=True,
        preserved_draft_text_ref="draft_text:community_support.body",
        available_actions=["export_packet", "save_draft_local", "retry_when_online", "discard"],
        selected_fallback_ref="handoff_fallback:save_draft_local",
        continuity_summary="Captured offline: the draft, attachments, target class, and redaction posture are preserved; export, save, retry, and discard are all offered.",
    )

    private_support = make_sheet(
        slug="private_support_intake_authenticated",
        sheet_summary="Private support-intake handoff routed to the authenticated support plane, never a public target.",
        route_class="community_support",
        visibility_class="official_private",
        destination_identity_ref="about_destination:support.intake.private",
        destination_label="Private support intake",
        network="system_browser_authenticated_plane",
        data_exit="redacted_support_packet",
        safe_fallback_refs=[
            "about_destination:local.fallback.docs_pack",
            "handoff_fallback:save_draft_local",
        ],
        exports=[
            build_context_export(
                "private_support_intake_block",
                "build_context_export:support_intake.v1",
                "Support intake block carries build identity refs; the redacted packet stays on the authenticated plane.",
            )
        ],
        target_headline="Open a private support request",
        target_summary="Routes to the authenticated private support intake; the report is not world-readable.",
        posture="redacted_support_scoped",
        diagnostics=[
            diagnostic("build_identity", True, "Build channel and version refs."),
            diagnostic("redacted_log_tail", True, "Tail of the redacted session log."),
            diagnostic("sanitized_config_snapshot", True, "Sanitized config snapshot refs."),
        ],
        attachments=[
            attachment(
                "redacted_log_bundle",
                "object:support.log.bundle.redacted",
                "Redacted log bundle, support scoped.",
            )
        ],
        anchor_ref="anchor:editor.session.crash",
        object_ref="object:editor.session",
        anchor_label="Editor session crash path",
        preview_confirmed=True,
        packet_headline="Support reproduction packet",
        packet_summary="Build identity, a redacted log tail, and a sanitized config snapshot scoped to the private support plane.",
        outcome="opened_in_system_browser",
        intent_preserved=True,
        preserved_draft_text_ref="draft_text:support_intake.body",
        available_actions=["export_packet", "save_draft_local"],
        selected_fallback_ref=None,
        continuity_summary="Opened on the authenticated support plane after preview confirmation; the redacted bundle stays saved locally.",
    )

    security = make_sheet(
        slug="security_disclosure_encrypted_channel",
        sheet_summary="Security disclosure routed to the encrypted private channel, never the public tracker, with security-only payloads.",
        route_class="security_disclosure",
        visibility_class="security_disclosure",
        destination_identity_ref="about_destination:security.intake.private",
        destination_label="Private security intake",
        network="encrypted_security_channel",
        data_exit="security_payloads_only",
        safe_fallback_refs=[
            "about_destination:local.fallback.docs_pack",
            "handoff_fallback:save_draft_local",
        ],
        exports=[
            build_context_export(
                "private_security_intake_block",
                "build_context_export:security_intake.v1",
                "Security intake block carries build identity refs; raw payloads stay on the encrypted channel.",
            )
        ],
        target_headline="Report a security issue privately",
        target_summary="Routes to the private security intake under the published key; public advisory follows fix-and-disclosure cadence.",
        posture="security_channel_only",
        diagnostics=[
            diagnostic("build_identity", True, "Build channel and version refs."),
            diagnostic("repro_steps_text", True, "Steps to reproduce the vulnerability."),
        ],
        attachments=[
            attachment(
                "minimal_repro_project",
                "object:repro.project.minimal",
                "Minimal reproduction project, secrets scrubbed.",
            )
        ],
        anchor_ref="anchor:auth.token.exchange",
        object_ref="object:crate.auth.token_exchange",
        anchor_label="Auth token exchange path",
        preview_confirmed=True,
        packet_headline="Security reproduction packet",
        packet_summary="Build identity and reproduction steps with a scrubbed minimal repro project, scoped to the encrypted channel.",
        outcome="opened_in_system_browser",
        intent_preserved=True,
        preserved_draft_text_ref="draft_text:security_disclosure.body",
        available_actions=["export_packet", "save_draft_local"],
        selected_fallback_ref=None,
        continuity_summary="Opened on the encrypted channel after preview confirmation; the scrubbed repro stays saved locally.",
    )

    rfc_denied = make_sheet(
        slug="rfc_discussion_policy_denied",
        sheet_summary="RFC/discussion handoff denied by managed policy; the prepared proposal context is preserved with export and save.",
        route_class="rfc_discussion",
        visibility_class="community",
        destination_identity_ref="about_destination:rfc.forum.public",
        destination_label="RFC / discussion forum",
        network="system_browser_public_browse",
        data_exit="proposal_refs_only",
        safe_fallback_refs=[
            "about_destination:local.fallback.docs_pack",
            "handoff_fallback:save_draft_local",
        ],
        exports=[
            build_context_export(
                "community_discussion_block",
                "build_context_export:rfc_discussion.v1",
                "RFC discussion block carries proposal refs only.",
            )
        ],
        target_headline="Open an RFC discussion",
        target_summary="Opens the public RFC/discussion forum in the system browser to propose a change.",
        posture="metadata_refs_only",
        diagnostics=[
            diagnostic("build_identity", True, "Build channel and version refs."),
            diagnostic("anchor_object_ref", True, "Anchor / object refs for the proposal target."),
        ],
        attachments=[],
        anchor_ref="anchor:rfc.proposal.command_palette",
        object_ref="object:rfc.proposal.command_palette",
        anchor_label="Command-palette RFC proposal",
        preview_confirmed=True,
        packet_headline="RFC reproduction packet",
        packet_summary="Build identity and the exact proposal anchor; proposal refs only.",
        outcome="policy_denied",
        intent_preserved=True,
        preserved_draft_text_ref="draft_text:rfc_discussion.body",
        available_actions=["export_packet", "save_draft_local", "copy_refs_to_clipboard", "discard"],
        selected_fallback_ref="handoff_fallback:save_draft_local",
        continuity_summary="Managed policy denied the handoff. The proposal draft, target class, and anchor are preserved with export, save, copy-refs, and discard.",
    )

    return [
        Drill(
            scenario_id="public_issue_after_preview",
            fixture_filename="accept_public_issue_after_preview.json",
            claimed_beta_row="beta.row.public.issue_handoff",
            expectation="accept",
            title="Public issue opened after preview confirmation",
            summary="A world-readable public issue is filed only after the reproduction-packet preview is confirmed; the exact anchor and redacted export survive export and reopen.",
            sheet=public_issue,
            stage_classes=["prepared", "preview_confirmed", "export", "reopen"],
        ),
        Drill(
            scenario_id="docs_feedback_browser_blocked",
            fixture_filename="accept_docs_feedback_browser_blocked.json",
            claimed_beta_row="beta.row.public.docs_feedback",
            expectation="accept",
            title="Browser-blocked docs feedback preserves prepared context",
            summary="A blocked system browser never loses the prepared docs-feedback context; the draft, export block, and anchor survive retry, export, and reopen.",
            sheet=docs_feedback,
            stage_classes=["prepared", "preview_confirmed", "browser_blocked", "retry", "export", "reopen"],
        ),
        Drill(
            scenario_id="community_support_offline_capture",
            fixture_filename="accept_community_support_offline_capture.json",
            claimed_beta_row="beta.row.community.support_offline",
            expectation="accept",
            title="Offline community-support capture and preview",
            summary="The community-support packet is captured and previewed offline without leaving the product; everything is preserved for later publish.",
            sheet=community_offline,
            stage_classes=["prepared", "offline_captured", "preview_confirmed", "export", "reopen"],
        ),
        Drill(
            scenario_id="private_support_intake_authenticated",
            fixture_filename="accept_private_support_intake_authenticated.json",
            claimed_beta_row="beta.row.private.support_intake",
            expectation="accept",
            title="Private support intake stays off public targets",
            summary="A private support request routes to the authenticated plane with a support-scoped redacted bundle and is never world-readable.",
            sheet=private_support,
            stage_classes=["prepared", "preview_confirmed", "export", "reopen"],
        ),
        Drill(
            scenario_id="security_disclosure_encrypted_channel",
            fixture_filename="accept_security_disclosure_encrypted_channel.json",
            claimed_beta_row="beta.row.security.disclosure",
            expectation="accept",
            title="Security disclosure stays on the encrypted channel",
            summary="A security report routes only to the encrypted private channel with security-scoped redaction; it can never be coerced to a public target.",
            sheet=security,
            stage_classes=["prepared", "preview_confirmed", "export", "reopen"],
        ),
        Drill(
            scenario_id="rfc_discussion_policy_denied",
            fixture_filename="accept_rfc_discussion_policy_denied.json",
            claimed_beta_row="beta.row.community.rfc_discussion",
            expectation="accept",
            title="Policy-denied RFC discussion preserves the proposal",
            summary="A managed-policy denial preserves the prepared RFC proposal context with export and save instead of dropping it.",
            sheet=rfc_denied,
            stage_classes=["prepared", "preview_confirmed", "policy_denied", "export", "reopen"],
        ),
    ]


def reject_drills() -> list[Drill]:
    drills: list[Drill] = []

    # 1. Mislabeled target: a security route pointed at a public visibility.
    coerced = make_sheet(
        slug="reject_security_route_coerced_to_public",
        sheet_summary="A security route is pointed at a public visibility — the lane must refuse to coerce a disclosure onto a public target.",
        route_class="security_disclosure",
        visibility_class="official_public",
        destination_identity_ref="about_destination:issue.tracker.public",
        destination_label="Public issue tracker",
        network="system_browser_public_browse",
        data_exit="metadata_safe_object_refs",
        safe_fallback_refs=["handoff_fallback:save_draft_local"],
        exports=[
            build_context_export(
                "public_issue_template_block",
                "build_context_export:coerced.v1",
                "Build identity refs only.",
            )
        ],
        target_headline="Report a security issue",
        target_summary="Incorrectly routes a security report to the public tracker.",
        posture="fully_redacted_public_safe",
        diagnostics=[diagnostic("build_identity", True, "Build channel and version refs.")],
        attachments=[],
        anchor_ref="anchor:auth.token.exchange",
        object_ref="object:crate.auth.token_exchange",
        anchor_label="Auth token exchange path",
        preview_confirmed=True,
        packet_headline="Coerced reproduction packet",
        packet_summary="Build identity refs only.",
        outcome="preview_pending_confirmation",
        intent_preserved=True,
        preserved_draft_text_ref="draft_text:coerced.body",
        available_actions=["export_packet", "save_draft_local"],
        selected_fallback_ref=None,
        continuity_summary="A security report incorrectly aimed at a public target.",
    )
    drills.append(
        Drill(
            scenario_id="security_route_coerced_to_public",
            fixture_filename="reject_security_route_coerced_to_public.json",
            claimed_beta_row=None,
            expectation="reject",
            title="Security route coerced to a public target is rejected",
            summary="A security_disclosure route may only target security or private visibility; pointing it at a public target is rejected so a disclosure is never coerced onto a world-readable tracker.",
            sheet=coerced,
            stage_classes=["prepared", "preview_confirmed"],
            expected_rejection_codes=["route_visibility_mismatch"],
        )
    )

    # 2. Redaction-preview mismatch: public target carrying a security-scoped posture.
    mismatch = make_sheet(
        slug="reject_public_target_with_security_redaction",
        sheet_summary="A public issue carries a security-channel-only redaction posture — a world-readable target must never carry a security-scoped payload.",
        route_class="public_issue",
        visibility_class="official_public",
        destination_identity_ref="about_destination:issue.tracker.public",
        destination_label="Public issue tracker",
        network="system_browser_public_browse",
        data_exit="metadata_safe_object_refs",
        safe_fallback_refs=["handoff_fallback:save_draft_local"],
        exports=[
            build_context_export(
                "public_issue_template_block",
                "build_context_export:mismatch.v1",
                "Build identity refs only.",
            )
        ],
        target_headline="File a public issue",
        target_summary="Opens the public issue tracker in the system browser.",
        posture="security_channel_only",
        diagnostics=[diagnostic("build_identity", True, "Build channel and version refs.")],
        attachments=[],
        anchor_ref="anchor:docs.page.boundary_truth.section.3",
        object_ref="object:docs.page.boundary_truth",
        anchor_label="Boundary-truth docs page, section 3",
        preview_confirmed=True,
        packet_headline="Mismatched reproduction packet",
        packet_summary="Security-scoped redaction on a world-readable target.",
        outcome="preview_pending_confirmation",
        intent_preserved=True,
        preserved_draft_text_ref="draft_text:mismatch.body",
        available_actions=["export_packet", "save_draft_local"],
        selected_fallback_ref=None,
        continuity_summary="A public target carrying a security-scoped redaction posture.",
    )
    # Keep the preserved redaction mirroring the (wrong) packet posture, so the
    # only violation is the posture-vs-visibility mismatch.
    drills.append(
        Drill(
            scenario_id="public_target_with_security_redaction",
            fixture_filename="reject_public_target_with_security_redaction.json",
            claimed_beta_row=None,
            expectation="reject",
            title="Public target with a security-scoped redaction is rejected",
            summary="A world-readable target may not carry a support- or security-scoped redaction posture; the redaction-preview mismatch is rejected.",
            sheet=mismatch,
            stage_classes=["prepared", "preview_confirmed"],
            expected_rejection_codes=["redaction_posture_unsafe_for_visibility"],
        )
    )

    # 3. Preview omits a field but the export still carries it.
    omit_sheet = make_sheet(
        slug="reject_preview_omits_but_exports",
        sheet_summary="The preview marks the redacted log tail excluded, but the export stage still carries it.",
        route_class="public_issue",
        visibility_class="official_public",
        destination_identity_ref="about_destination:issue.tracker.public",
        destination_label="Public issue tracker",
        network="system_browser_public_browse",
        data_exit="metadata_safe_object_refs",
        safe_fallback_refs=["handoff_fallback:save_draft_local"],
        exports=[
            build_context_export(
                "public_issue_template_block",
                "build_context_export:omit.v1",
                "Build identity refs only.",
            )
        ],
        target_headline="File a public issue",
        target_summary="Opens the public issue tracker in the system browser.",
        posture="fully_redacted_public_safe",
        diagnostics=[
            diagnostic("build_identity", True, "Build channel and version refs."),
            diagnostic("redacted_log_tail", False, "Redacted log tail the user chose to exclude."),
        ],
        attachments=[
            attachment(
                "build_context_export_block",
                "build_context_export:omit.v1",
                "Redacted build-context export block.",
            )
        ],
        anchor_ref="anchor:docs.page.boundary_truth.section.3",
        object_ref="object:docs.page.boundary_truth",
        anchor_label="Boundary-truth docs page, section 3",
        preview_confirmed=True,
        packet_headline="Omission reproduction packet",
        packet_summary="Build identity only; the redacted log tail is excluded from the preview.",
        outcome="opened_in_system_browser",
        intent_preserved=True,
        preserved_draft_text_ref="draft_text:omit.body",
        available_actions=["export_packet", "save_draft_local"],
        selected_fallback_ref=None,
        continuity_summary="A field hidden from the review sheet must not be exported.",
    )
    omit_flow = build_continuity_flow(
        omit_sheet, ["prepared", "preview_confirmed", "export", "reopen"]
    )
    # Inject the drift: the export stage carries the excluded diagnostic.
    for stage in omit_flow["stages"]:
        if stage["stage_class"] == "export":
            stage["carried_diagnostic_kinds"] = sorted(
                set(stage["carried_diagnostic_kinds"]) | {"redacted_log_tail"}
            )
    drills.append(
        Drill(
            scenario_id="preview_omits_but_exports",
            fixture_filename="reject_preview_omits_but_exports.json",
            claimed_beta_row=None,
            expectation="reject",
            title="Preview omits a field but the export carries it — rejected",
            summary="The reproduction-packet preview marked a diagnostic excluded, yet the export stage still wrote it; exporting a field the review sheet hid is rejected.",
            sheet=omit_sheet,
            stage_classes=["prepared", "preview_confirmed", "export", "reopen"],
            expected_rejection_codes=["continuity_preview_export_mismatch"],
            flow_override=omit_flow,
        )
    )

    # 4. Exact-anchor loss on reopen.
    anchor_sheet = make_sheet(
        slug="reject_exact_anchor_loss_on_reopen",
        sheet_summary="The reopened draft points at a different anchor than the one the user reviewed.",
        route_class="docs_feedback",
        visibility_class="official_public",
        destination_identity_ref="about_destination:docs.feedback.public",
        destination_label="Docs feedback channel",
        network="system_browser_public_browse",
        data_exit="metadata_safe_object_refs",
        safe_fallback_refs=["handoff_fallback:save_draft_local"],
        exports=[
            build_context_export(
                "public_issue_template_block",
                "build_context_export:anchor.v1",
                "Build identity refs only.",
            )
        ],
        target_headline="Send docs feedback",
        target_summary="Opens the public docs-feedback channel in the system browser.",
        posture="fully_redacted_public_safe",
        diagnostics=[
            diagnostic("build_identity", True, "Build channel and version refs."),
            diagnostic("anchor_object_ref", True, "Anchor / object refs for the docs page."),
        ],
        attachments=[],
        anchor_ref="anchor:docs.page.install.step.2",
        object_ref="object:docs.page.install",
        anchor_label="Install docs page, step 2",
        preview_confirmed=True,
        packet_headline="Anchor reproduction packet",
        packet_summary="Build identity and the exact docs anchor.",
        outcome="browser_blocked",
        intent_preserved=True,
        preserved_draft_text_ref="draft_text:anchor.body",
        available_actions=["export_packet", "save_draft_local", "retry_when_online"],
        selected_fallback_ref="handoff_fallback:save_draft_local",
        continuity_summary="The exact anchor must survive reopen unchanged.",
    )
    anchor_flow = build_continuity_flow(
        anchor_sheet, ["prepared", "preview_confirmed", "browser_blocked", "reopen"]
    )
    for stage in anchor_flow["stages"]:
        if stage["stage_class"] == "reopen":
            stage["anchor_ref"] = "anchor:docs.page.install.step.9"
            stage["object_ref"] = "object:docs.page.changelog"
    drills.append(
        Drill(
            scenario_id="exact_anchor_loss_on_reopen",
            fixture_filename="reject_exact_anchor_loss_on_reopen.json",
            claimed_beta_row=None,
            expectation="reject",
            title="Exact-anchor loss on reopen is rejected",
            summary="A reopened draft whose anchor / object identity drifts from the reviewed packet is rejected so a report cannot silently change which object it is about.",
            sheet=anchor_sheet,
            stage_classes=["prepared", "preview_confirmed", "browser_blocked", "reopen"],
            expected_rejection_codes=["continuity_anchor_drift"],
            flow_override=anchor_flow,
        )
    )

    # 5. Blocked handoff loses the preserved context across reopen.
    loss_sheet = make_sheet(
        slug="reject_blocked_handoff_context_loss",
        sheet_summary="A blocked offline handoff drops the preserved draft when the user reopens it.",
        route_class="community_support",
        visibility_class="community",
        destination_identity_ref="about_destination:community.forum.public",
        destination_label="Community support forum",
        network="offline_capture_preview",
        data_exit="metadata_safe_object_refs",
        safe_fallback_refs=["handoff_fallback:save_draft_local"],
        exports=[
            build_context_export(
                "community_discussion_block",
                "build_context_export:loss.v1",
                "Community discussion block carries build identity refs only.",
            )
        ],
        target_headline="Ask the community",
        target_summary="Builds the community-support packet offline and previews it before any browser opens.",
        posture="metadata_refs_only",
        diagnostics=[diagnostic("build_identity", True, "Build channel and version refs.")],
        attachments=[],
        anchor_ref="anchor:settings.theme.density",
        object_ref="object:workspace.setting.theme",
        anchor_label="Theme density setting",
        preview_confirmed=True,
        packet_headline="Context-loss reproduction packet",
        packet_summary="Build identity refs only.",
        outcome="offline",
        intent_preserved=True,
        preserved_draft_text_ref="draft_text:loss.body",
        available_actions=["export_packet", "save_draft_local", "retry_when_online"],
        selected_fallback_ref="handoff_fallback:save_draft_local",
        continuity_summary="A blocked handoff must keep the prepared context across reopen.",
    )
    loss_flow = build_continuity_flow(
        loss_sheet, ["prepared", "offline_captured", "preview_confirmed", "reopen"]
    )
    for stage in loss_flow["stages"]:
        if stage["stage_class"] == "reopen":
            stage["draft_preserved"] = False
    drills.append(
        Drill(
            scenario_id="blocked_handoff_context_loss",
            fixture_filename="reject_blocked_handoff_context_loss.json",
            claimed_beta_row=None,
            expectation="reject",
            title="Blocked handoff that loses prepared context is rejected",
            summary="A blocked / offline handoff that drops the preserved draft on reopen is rejected; prepared context must survive every post-block stage.",
            sheet=loss_sheet,
            stage_classes=["prepared", "offline_captured", "preview_confirmed", "reopen"],
            expected_rejection_codes=["continuity_blocked_context_loss"],
            flow_override=loss_flow,
        )
    )

    # 6. Support-boundary copy drift: private/security route described as public.
    copy_sheet = make_sheet(
        slug="reject_support_boundary_copy_drift",
        sheet_summary="A security disclosure whose support-boundary copy calls it world-readable.",
        route_class="security_disclosure",
        visibility_class="security_disclosure",
        destination_identity_ref="about_destination:security.intake.private",
        destination_label="Private security intake",
        network="encrypted_security_channel",
        data_exit="security_payloads_only",
        safe_fallback_refs=["handoff_fallback:save_draft_local"],
        exports=[
            build_context_export(
                "private_security_intake_block",
                "build_context_export:copydrift.v1",
                "Security intake block carries build identity refs.",
            )
        ],
        target_headline="Report a security issue privately",
        target_summary="Routes to the private security intake under the published key.",
        posture="security_channel_only",
        diagnostics=[diagnostic("build_identity", True, "Build channel and version refs.")],
        attachments=[],
        anchor_ref="anchor:auth.token.exchange",
        object_ref="object:crate.auth.token_exchange",
        anchor_label="Auth token exchange path",
        preview_confirmed=True,
        packet_headline="Copy-drift reproduction packet",
        packet_summary="Build identity refs only, scoped to the encrypted channel.",
        outcome="opened_in_system_browser",
        intent_preserved=True,
        preserved_draft_text_ref="draft_text:copydrift.body",
        available_actions=["export_packet", "save_draft_local"],
        selected_fallback_ref=None,
        continuity_summary="Support-boundary copy must match the actual target boundary.",
    )
    copy_override = derive_support_copy(copy_sheet)
    copy_override["names_world_readable"] = True
    copy_override["claimed_visibility_class"] = "official_public"
    copy_override["support_scope_label"] = TARGET_VISIBILITY_LABEL["official_public"]
    copy_override["copy_text"] = (
        "This handoff is Official public (world-readable once shared); it exits as "
        "security_payloads_only over encrypted_security_channel."
    )
    drills.append(
        Drill(
            scenario_id="support_boundary_copy_drift",
            fixture_filename="reject_support_boundary_copy_drift.json",
            claimed_beta_row=None,
            expectation="reject",
            title="Support-boundary copy that mislabels a security route is rejected",
            summary="Support-boundary copy that describes a security / private route as world-readable is rejected so support and disclosure copy cannot drift from actual product behavior.",
            sheet=copy_sheet,
            stage_classes=["prepared", "preview_confirmed"],
            expected_rejection_codes=[
                "support_copy_visibility_drift",
                "support_copy_world_readable_drift",
                "support_copy_label_drift",
            ],
            support_override=copy_override,
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


def check_drill(label: str, drill: dict[str, Any], target_validator: Draft202012Validator,
                packet_validator: Draft202012Validator) -> list[Finding]:
    findings: list[Finding] = []

    if drill.get("record_kind") != DRILL_RECORD_KIND:
        findings.append(
            err(
                "handoff_repro.drill.record_kind",
                f"{label}: record_kind must be {DRILL_RECORD_KIND}",
                "Re-mint the drill with --write.",
                label,
            )
        )
        return findings
    if drill.get("schema_version") != DRILL_SCHEMA_VERSION:
        findings.append(
            err(
                "handoff_repro.drill.schema_version",
                f"{label}: schema_version must be {DRILL_SCHEMA_VERSION}",
                "Bump the validator together with any drill schema change.",
                label,
            )
        )
    expectation = drill.get("expectation")
    if expectation not in EXPECTATIONS:
        findings.append(
            err(
                "handoff_repro.drill.expectation",
                f"{label}: expectation must be one of {sorted(EXPECTATIONS)}",
                "Set expectation to accept or reject.",
                label,
            )
        )
        return findings
    if drill.get("contract_doc_ref") != CONTRACT_DOC_REF:
        findings.append(
            err(
                "handoff_repro.drill.contract_doc",
                f"{label}: drill cites the wrong contract doc",
                f"Cite {CONTRACT_DOC_REF}.",
                label,
            )
        )

    sheet = drill.get("review_sheet", {})
    flow = drill.get("continuity_flow", {})
    support = drill.get("support_boundary_copy", {})

    # Schema-validate the two constituent records.
    target = sheet.get("target_review", {})
    packet = sheet.get("repro_packet_preview", {})
    schema_codes: list[str] = []
    target_schema_errs = list(target_validator.iter_errors(target))
    packet_schema_errs = list(packet_validator.iter_errors(packet))
    if target_schema_errs:
        schema_codes.append("target_schema")
    if packet_schema_errs:
        schema_codes.append("packet_schema")

    model_codes = validate_review_sheet(sheet)
    continuity_codes = check_continuity(label, sheet, flow) if isinstance(flow, dict) else ["continuity_no_stages"]
    support_codes = check_support_copy(label, sheet, support) if isinstance(support, dict) else ["support_copy_empty"]

    all_codes = sorted(set(schema_codes + model_codes + continuity_codes + support_codes))

    if expectation == "accept":
        # Schema must pass.
        for e in sorted(target_schema_errs, key=lambda x: list(x.path)):
            path = ".".join(str(p) for p in e.path) or "<root>"
            findings.append(
                err(
                    "handoff_repro.schema.target",
                    f"{label}: target_review.{path}: {e.message}",
                    f"Fix the record so it validates against {TARGET_SCHEMA_REL}.",
                    label,
                )
            )
        for e in sorted(packet_schema_errs, key=lambda x: list(x.path)):
            path = ".".join(str(p) for p in e.path) or "<root>"
            findings.append(
                err(
                    "handoff_repro.schema.packet",
                    f"{label}: repro_packet_preview.{path}: {e.message}",
                    f"Fix the record so it validates against {PACKET_SCHEMA_REL}.",
                    label,
                )
            )
        if model_codes:
            findings.append(
                err(
                    "handoff_repro.model.unexpected_rejection",
                    f"{label}: accept drill is rejected by the model port: {sorted(set(model_codes))}",
                    "Fix the review sheet so it validates, or re-classify the drill as reject.",
                    label,
                    codes=sorted(set(model_codes)),
                )
            )
        if continuity_codes:
            findings.append(
                err(
                    "handoff_repro.continuity.unexpected_drift",
                    f"{label}: accept drill's continuity flow drifts from the review sheet: {continuity_codes}",
                    "Re-mint the drill with --write so the flow mirrors the sheet.",
                    label,
                    codes=continuity_codes,
                )
            )
        if support_codes:
            findings.append(
                err(
                    "handoff_repro.support_copy.unexpected_drift",
                    f"{label}: accept drill's support-boundary copy drifts from the target: {support_codes}",
                    "Re-mint the drill with --write so the copy mirrors the target.",
                    label,
                    codes=support_codes,
                )
            )
        if drill.get("expected_rejection_codes"):
            findings.append(
                err(
                    "handoff_repro.accept.has_rejection_codes",
                    f"{label}: accept drill must not declare expected_rejection_codes",
                    "Leave expected_rejection_codes empty on accept drills.",
                    label,
                )
            )
        if drill.get("claimed_beta_row") is None:
            findings.append(
                err(
                    "handoff_repro.accept.no_claimed_row",
                    f"{label}: accept drill must map to a claimed beta row",
                    "Bind each accept drill to exactly one claimed beta row.",
                    label,
                )
            )
    else:  # reject
        if not all_codes:
            findings.append(
                err(
                    "handoff_repro.reject.not_rejected",
                    f"{label}: reject drill validates clean — the documented drift is not caught",
                    "Make the drill actually exercise the drift, or re-classify it as accept.",
                    label,
                )
            )
        expected = drill.get("expected_rejection_codes") or []
        if not expected:
            findings.append(
                err(
                    "handoff_repro.reject.no_expected_codes",
                    f"{label}: reject drill must declare at least one expected rejection code",
                    "Declare the typed rejection code(s) the drill proves.",
                    label,
                )
            )
        for code in expected:
            if code not in all_codes:
                findings.append(
                    err(
                        "handoff_repro.reject.missing_expected_code",
                        f"{label}: expected rejection code {code!r} not produced; got {all_codes}",
                        "Align the drill with its expected typed rejection reason.",
                        label,
                        produced=all_codes,
                    )
                )
        if drill.get("claimed_beta_row") is not None:
            findings.append(
                err(
                    "handoff_repro.reject.has_claimed_row",
                    f"{label}: reject drill must not claim a beta row",
                    "Only accept drills map to claimed beta rows.",
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

    visibilities = {
        v["review_sheet"]["target_review"]["visibility_class"] for v in accept.values()
    }
    outcomes = {
        v["review_sheet"]["draft_continuity"]["handoff_outcome_class"] for v in accept.values()
    }
    networks = {
        v["review_sheet"]["target_review"]["network_browser_requirement_class"]
        for v in accept.values()
    }
    reject_codes: set[str] = set()
    for v in reject.values():
        reject_codes.update(v.get("expected_rejection_codes") or [])

    required = [
        ("visibility.official_public", "official_public" in visibilities,
         "Add an accept drill with an official_public target."),
        ("visibility.official_private", "official_private" in visibilities,
         "Add an accept drill with an official_private target."),
        ("visibility.security_disclosure", "security_disclosure" in visibilities,
         "Add an accept drill with a security_disclosure target."),
        ("visibility.community", "community" in visibilities,
         "Add an accept drill with a community target."),
        ("outcome.browser_blocked", "browser_blocked" in outcomes,
         "Add an accept drill whose browser handoff is blocked."),
        ("outcome.offline", "offline" in outcomes,
         "Add an accept drill captured offline."),
        ("outcome.policy_denied", "policy_denied" in outcomes,
         "Add an accept drill denied by managed policy."),
        ("network.offline_capture_preview", "offline_capture_preview" in networks,
         "Add an accept drill that builds and previews the packet offline."),
        ("reject.route_visibility_mismatch", "route_visibility_mismatch" in reject_codes,
         "Add a reject drill where a route is mislabeled onto a disallowed visibility."),
        ("reject.redaction_posture_unsafe_for_visibility",
         "redaction_posture_unsafe_for_visibility" in reject_codes,
         "Add a reject drill with a redaction-preview mismatch."),
        ("reject.continuity_preview_export_mismatch",
         "continuity_preview_export_mismatch" in reject_codes,
         "Add a reject drill where the export carries a field the preview omitted."),
        ("reject.continuity_anchor_drift", "continuity_anchor_drift" in reject_codes,
         "Add a reject drill that loses the exact anchor on reopen."),
        ("reject.continuity_blocked_context_loss",
         "continuity_blocked_context_loss" in reject_codes,
         "Add a reject drill where a blocked handoff loses prepared context."),
        ("reject.support_copy_world_readable_drift",
         "support_copy_world_readable_drift" in reject_codes,
         "Add a reject drill where support-boundary copy mislabels the visibility."),
    ]
    for name, ok, remedy in required:
        if not ok:
            findings.append(
                err(
                    f"handoff_repro.coverage.{name}",
                    f"corpus does not prove the required axis: {name}",
                    remedy,
                )
            )
    return findings


# --------------------------------------------------------------------------- #
# Export parity — support-bundle plaintext and CLI / headless index preserve the
# semantics of the accept-drill record.
# --------------------------------------------------------------------------- #


def _join(values: list[str]) -> str:
    return ",".join(values) if values else "-"


def _split(value: str) -> list[str]:
    return [] if value == "-" else value.split(",")


def _b(value: bool) -> str:
    return "true" if value else "false"


def digest_from_drill(drill: dict[str, Any]) -> dict[str, Any]:
    sheet = drill["review_sheet"]
    target = sheet["target_review"]
    packet = sheet["repro_packet_preview"]
    draft = sheet["draft_continuity"]
    identity = derive_carried_identity(sheet)
    return {
        "scenario_id": drill["scenario_id"],
        "claimed_beta_row": drill["claimed_beta_row"],
        "route_class": target["route_class"],
        "visibility_class": target["visibility_class"],
        "world_readable": visibility_is_public(target["visibility_class"]),
        "network_browser_requirement_class": target["network_browser_requirement_class"],
        "data_exit_boundary_class": target["data_exit_boundary_class"],
        "redaction_posture_class": packet["redaction_posture_class"],
        "preview_confirmed_before_share": bool(packet["preview_confirmed_before_share"]),
        "handoff_outcome_class": draft["handoff_outcome_class"],
        "intent_preserved": bool(draft["intent_preserved"]),
        "anchor_ref": identity["anchor_ref"],
        "object_ref": identity["object_ref"],
        "build_context_export_refs": list(identity["build_context_export_refs"]),
        "shareable_diagnostic_kinds": list(identity["shareable_diagnostic_kinds"]),
        "attachment_refs": list(identity["attachment_refs"]),
        "safe_fallback_refs": sorted(target["safe_fallback_refs"]),
        "available_actions": list(draft["available_actions"]),
    }


def coarse_digest(full: dict[str, Any]) -> dict[str, Any]:
    return {
        "visibility_class": full["visibility_class"],
        "world_readable": full["world_readable"],
        "redaction_posture_class": full["redaction_posture_class"],
        "handoff_outcome_class": full["handoff_outcome_class"],
        "preview_confirmed_before_share": full["preview_confirmed_before_share"],
        "intent_preserved": full["intent_preserved"],
    }


def render_support_export(drill: dict[str, Any]) -> str:
    d = digest_from_drill(drill)
    out: list[str] = []
    out.append("Handoff & reproduction-packet review")
    out.append(f"scenario_id: {d['scenario_id']}")
    out.append(f"claimed_beta_row: {d['claimed_beta_row']}")
    out.append(f"route_class: {d['route_class']}")
    out.append(f"visibility_class: {d['visibility_class']}")
    out.append(f"world_readable: {_b(d['world_readable'])}")
    out.append(f"network_browser_requirement_class: {d['network_browser_requirement_class']}")
    out.append(f"data_exit_boundary_class: {d['data_exit_boundary_class']}")
    out.append(f"redaction_posture_class: {d['redaction_posture_class']}")
    out.append(f"preview_confirmed_before_share: {_b(d['preview_confirmed_before_share'])}")
    out.append(f"handoff_outcome_class: {d['handoff_outcome_class']}")
    out.append(f"intent_preserved: {_b(d['intent_preserved'])}")
    out.append(f"anchor_ref: {d['anchor_ref']}")
    out.append(f"object_ref: {d['object_ref']}")
    out.append(f"build_context_export_refs: {_join(d['build_context_export_refs'])}")
    out.append(f"shareable_diagnostic_kinds: {_join(d['shareable_diagnostic_kinds'])}")
    out.append(f"attachment_refs: {_join(d['attachment_refs'])}")
    out.append(f"safe_fallback_refs: {_join(d['safe_fallback_refs'])}")
    out.append(f"available_actions: {_join(d['available_actions'])}")
    return "\n".join(out) + "\n"


def digest_from_support_export(text: str) -> dict[str, Any]:
    fields: dict[str, str] = {}
    for line in text.split("\n"):
        if ": " in line:
            key, value = line.split(": ", 1)
            fields[key] = value
    return {
        "scenario_id": fields["scenario_id"],
        "claimed_beta_row": None if fields["claimed_beta_row"] == "None" else fields["claimed_beta_row"],
        "route_class": fields["route_class"],
        "visibility_class": fields["visibility_class"],
        "world_readable": fields["world_readable"] == "true",
        "network_browser_requirement_class": fields["network_browser_requirement_class"],
        "data_exit_boundary_class": fields["data_exit_boundary_class"],
        "redaction_posture_class": fields["redaction_posture_class"],
        "preview_confirmed_before_share": fields["preview_confirmed_before_share"] == "true",
        "handoff_outcome_class": fields["handoff_outcome_class"],
        "intent_preserved": fields["intent_preserved"] == "true",
        "anchor_ref": fields["anchor_ref"],
        "object_ref": fields["object_ref"],
        "build_context_export_refs": _split(fields["build_context_export_refs"]),
        "shareable_diagnostic_kinds": _split(fields["shareable_diagnostic_kinds"]),
        "attachment_refs": _split(fields["attachment_refs"]),
        "safe_fallback_refs": _split(fields["safe_fallback_refs"]),
        "available_actions": _split(fields["available_actions"]),
    }


def render_cli_index(drill: dict[str, Any], fixture_filename: str) -> str:
    d = digest_from_drill(drill)
    return "\t".join(
        [
            d["scenario_id"],
            d["route_class"],
            d["visibility_class"],
            f"world_readable={_b(d['world_readable'])}",
            d["redaction_posture_class"],
            d["handoff_outcome_class"],
            f"preview_confirmed={_b(d['preview_confirmed_before_share'])}",
            f"preserved={_b(d['intent_preserved'])}",
            fixture_filename,
        ]
    )


def digest_from_cli_index(line: str) -> dict[str, Any]:
    parts = line.split("\t")
    return {
        "visibility_class": parts[2],
        "world_readable": parts[3].split("=", 1)[1] == "true",
        "redaction_posture_class": parts[4],
        "handoff_outcome_class": parts[5],
        "preview_confirmed_before_share": parts[6].split("=", 1)[1] == "true",
        "intent_preserved": parts[7].split("=", 1)[1] == "true",
    }


def check_parity(accept: dict[str, tuple[str, dict[str, Any]]]) -> list[Finding]:
    findings: list[Finding] = []
    for scenario_id, (source_fixture, drill) in sorted(accept.items()):
        full = digest_from_drill(drill)
        support_text = render_support_export(drill)
        cli_line = render_cli_index(drill, Path(source_fixture).name)
        if digest_from_support_export(support_text) != full:
            findings.append(
                err(
                    "handoff_repro.parity.support_export",
                    f"{scenario_id}: support-export plaintext loses semantics from the record",
                    "Keep the support-export projection faithful to the record.",
                    source_fixture,
                )
            )
        if digest_from_cli_index(cli_line) != coarse_digest(full):
            findings.append(
                err(
                    "handoff_repro.parity.cli_index",
                    f"{scenario_id}: CLI / headless index loses semantics from the record",
                    "Keep the CLI / headless index faithful to the record.",
                    source_fixture,
                )
            )
    return findings


# --------------------------------------------------------------------------- #
# Matrix + parity packet (build / drift-check)
# --------------------------------------------------------------------------- #


def lane_properties(drill: dict[str, Any]) -> list[str]:
    sheet = drill["review_sheet"]
    target = sheet["target_review"]
    packet = sheet["repro_packet_preview"]
    draft = sheet["draft_continuity"]
    props = [
        "exact_target_identity",
        "exact_anchor_continuity",
        "support_boundary_copy_honest",
        "export_parity",
    ]
    if packet["preview_confirmed_before_share"]:
        props.append("preview_before_share")
    if draft["handoff_outcome_class"] in BLOCKED_OUTCOMES:
        props.append("blocked_preservation")
    if target["network_browser_requirement_class"] == "offline_capture_preview":
        props.append("offline_capture")
    if not visibility_is_public(target["visibility_class"]):
        props.append("private_target")
    return sorted(set(props))


def build_matrix(drills: dict[str, tuple[str, dict[str, Any]]]) -> dict[str, Any]:
    accept_cases = []
    reject_cases = []
    claimed_rows = []
    for scenario_id, (source_fixture, drill) in sorted(drills.items()):
        sheet = drill["review_sheet"]
        target = sheet["target_review"]
        packet = sheet["repro_packet_preview"]
        draft = sheet["draft_continuity"]
        if drill["expectation"] == "accept":
            accept_cases.append(
                {
                    "scenario_id": scenario_id,
                    "claimed_beta_row": drill["claimed_beta_row"],
                    "source_fixture": source_fixture,
                    "route_class": target["route_class"],
                    "visibility_class": target["visibility_class"],
                    "world_readable": visibility_is_public(target["visibility_class"]),
                    "network_browser_requirement_class": target["network_browser_requirement_class"],
                    "data_exit_boundary_class": target["data_exit_boundary_class"],
                    "redaction_posture_class": packet["redaction_posture_class"],
                    "preview_confirmed_before_share": packet["preview_confirmed_before_share"],
                    "handoff_outcome_class": draft["handoff_outcome_class"],
                    "intent_preserved": draft["intent_preserved"],
                    "stage_classes": [s["stage_class"] for s in drill["continuity_flow"]["stages"]],
                    "lane_properties": lane_properties(drill),
                }
            )
            claimed_rows.append(
                {"claimed_beta_row": drill["claimed_beta_row"], "scenario_id": scenario_id}
            )
        else:
            reject_cases.append(
                {
                    "scenario_id": scenario_id,
                    "source_fixture": source_fixture,
                    "expected_rejection_codes": sorted(drill.get("expected_rejection_codes") or []),
                    "route_class": target["route_class"],
                    "visibility_class": target["visibility_class"],
                }
            )
    claimed_rows.sort(key=lambda r: r["claimed_beta_row"])
    return {
        "record_kind": MATRIX_RECORD_KIND,
        "schema_version": 1,
        "shared_contract_ref": AUDIT_CONTRACT_REF,
        "target_schema_ref": TARGET_SCHEMA_REL,
        "packet_schema_ref": PACKET_SCHEMA_REL,
        "corpus_dir": CORPUS_DIR_REL,
        "validator_ref": VALIDATOR_REL,
        "contract_doc_ref": CONTRACT_DOC_REL,
        "claimed_beta_rows": claimed_rows,
        "accept_cases": accept_cases,
        "reject_cases": reject_cases,
    }


def build_parity_packet(accept: dict[str, tuple[str, dict[str, Any]]]) -> dict[str, Any]:
    entries = []
    for scenario_id, (source_fixture, drill) in sorted(accept.items()):
        full = digest_from_drill(drill)
        support_text = render_support_export(drill)
        cli_line = render_cli_index(drill, Path(source_fixture).name)
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
            "of the handoff-review record, so support, docs, and security teams can "
            "explain a handoff boundary from the exported packet alone. Regenerate "
            "with `ci/check_handoff_repro_corpus.py --write`."
        ),
        "scenarios": entries,
    }


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
                "handoff_repro.scorecard.missing_packet",
                f"claimed beta row maps to scenario {scenario_id} but no accept drill is present",
                "Mint the missing drill with --write.",
            )
        )
    for scenario_id in sorted(scenarios - expected):
        findings.append(
            err(
                "handoff_repro.scorecard.unclaimed_packet",
                f"accept drill {scenario_id} has no claimed beta row",
                "Map every accept drill to exactly one claimed beta row.",
            )
        )
    rows = [drill["claimed_beta_row"] for _, drill in accept.values()]
    dupes = sorted({r for r in rows if rows.count(r) > 1})
    for row in dupes:
        findings.append(
            err(
                "handoff_repro.scorecard.duplicate_row",
                f"claimed beta row {row} maps to more than one accept drill",
                "Each claimed row gets exactly one current handoff-repro packet.",
            )
        )
    return findings


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
                f"handoff_repro.{kind}.missing",
                f"{kind} is missing: {path}",
                "Generate it with --write.",
                str(path),
            )
        ]
    actual = load_json(path)
    if actual != expected:
        return [
            err(
                f"handoff_repro.{kind}.drift",
                f"{kind} has drifted from the corpus; regenerate with --write",
                "Run ci/check_handoff_repro_corpus.py --write and commit the result.",
                str(path),
                diff=_first_diff(actual, expected),
            )
        ]
    return []


def validate_report_and_readme(
    report_path: Path, readme_path: Path, scenario_ids: list[str]
) -> list[Finding]:
    findings: list[Finding] = []
    required_refs = [TARGET_SCHEMA_REL, PACKET_SCHEMA_REL, CORPUS_DIR_REL, VALIDATOR_REL, SCRIPT_REL]
    for kind, path, refs in (
        ("report", report_path, required_refs),
        ("readme", readme_path, [VALIDATOR_REL, TARGET_SCHEMA_REL, PACKET_SCHEMA_REL]),
    ):
        if not path.exists():
            findings.append(
                err(
                    f"handoff_repro.{kind}.missing",
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
                        f"handoff_repro.{kind}.missing_ref",
                        f"{kind} does not mention {ref}",
                        "Keep the report / README pointing at the schemas, corpus, validator, and script.",
                        str(path),
                    )
                )
        for scenario_id in scenario_ids:
            if scenario_id not in text:
                findings.append(
                    err(
                        f"handoff_repro.{kind}.missing_scenario",
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

    for rel in (TARGET_SCHEMA_REL, PACKET_SCHEMA_REL):
        if not (repo_root / rel).exists():
            print(f"[handoff-repro] missing schema: {repo_root / rel}", file=sys.stderr)
            return 2

    target_validator = make_validator(repo_root, TARGET_SCHEMA_REL)
    packet_validator = make_validator(repo_root, PACKET_SCHEMA_REL)

    drills = collect_drills(repo_root)
    if not drills:
        print("[handoff-repro] no handoff-repro drills found", file=sys.stderr)
        return 2

    findings: list[Finding] = []
    flat: dict[str, dict[str, Any]] = {}
    for scenario_id, (source_fixture, drill) in sorted(drills.items()):
        flat[scenario_id] = drill
        findings.extend(check_drill(source_fixture, drill, target_validator, packet_validator))

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
        "lane": "handoff_and_reproduction_packet_corpus",
        "target_schema": TARGET_SCHEMA_REL,
        "packet_schema": PACKET_SCHEMA_REL,
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
            f"[handoff-repro] FAIL: {len(errors)} error(s) across {len(drills)} drill(s)",
            file=sys.stderr,
        )
        for f in errors:
            print(f"  - {f.check_id}: {f.message}", file=sys.stderr)
        return 1

    print(
        f"[handoff-repro] PASS: {len(drills)} drill(s) validated "
        f"({len(accept)} accept, {len(drills) - len(accept)} reject), "
        "re-derived, parity- and scorecard-checked"
    )
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
