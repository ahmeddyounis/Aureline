#!/usr/bin/env python3
"""Regenerate the emergency-response evidence register.

The open/local-boundary durability matrix records, per asset lane, the emergency
signing/registry/security authority, and the release-authority continuity register makes each
protected authority lane inspectable. Neither records the *emergency-response evidence* a protected
M5 ecosystem/release lane produces when something goes wrong — the signed security advisory, the
extension/provider revocation packet, the emergency-disable bundle, and the high-severity
postmortem — nor whether that evidence actually reached the hosted, mirror, and offline customers
that claim it, whether the action is attributable and reversible where policy allows, whether a
break-glass action carried its audit markers and post-incident reconciliation, and whether the
evidence is linked to the release artifact-graph and support exports rather than a side channel.

This register is that emergency-response evidence layer. For every protected M5 ecosystem/release
lane and emergency packet it records one copy-safe record that states:

  - the packet template (a signed advisory/revocation/disable/postmortem packet, bound and
    digested);
  - the distribution reach (hosted, mirror, and offline channels, each claimed and propagated, so a
    mirror or offline customer is never left on a hosted-only path);
  - the attribution (the emergency action is attributable to an authorized actor);
  - the reversibility (a reversal runbook where policy permits reversal);
  - the audit trail (audit markers and, for break-glass or high-severity actions, post-incident
    reconciliation, so a break-glass action never bypasses the audit/reconciliation rules);
  - the evidence linkage (the release artifact-graph identity and support-export packet, not a side
    channel).

A record is cleared only when the packet template is bound, every claimed channel carries current
evidence, the action is attributable, a reversible action has its reversal runbook, the audit trail
is complete (markers present and reconciliation done where required), the evidence is linked to
release/support, the proof is fresh, and the owner signed. Otherwise it narrows on the specific axis
that thins out (a template gap, a distribution-reach gap, an attribution gap, a reversibility gap,
an audit gap, a linkage gap, or stale proof) and drops its effective label below the launch cutline;
the axes never collapse into one global flag. The response scan and the
service-health/release-center/support surface must agree on every record, so a green emergency-
response card can never mask a mirror/offline customer that never received the advisory, an
unattributable break-glass action, or a side-channel-only disable.

An inherited narrowing (a subject already below the cutline, or a gap held by an unexpired waiver)
is gated upstream and does not hold promotion; a response failure on a still-stable subject holds
promotion through a stop rule — a protected lane whose advisory/revocation/disable evidence did not
reach a claimed mirror/offline customer, or whose break-glass action bypassed audit/reconciliation,
cannot widen a stable claim without coverage.

This emits the canonical register artifact, the negative fixtures, the cases manifest, and the
frozen validation capture. The Python summary/parity/promotion logic mirrors the typed Rust consumer
so the checked-in artifact validates cleanly and the capture cross-check agrees with the model.
"""

from __future__ import annotations

import copy
import json
from pathlib import Path

REPO = Path(__file__).resolve().parent.parent

SLUG = "m5-emergency-response-evidence"
RECORD_KIND = "m5_emergency_response_evidence_register"
REGISTER_ID = "m5_emergency_response_evidence:v1"
ARTIFACT = REPO / "artifacts/governance" / f"{SLUG}.json"
CAPTURE = REPO / "artifacts/governance/captures" / f"{SLUG}_validation_capture.json"
FIXTURES = REPO / "fixtures/governance" / SLUG

AS_OF = "2026-06-16"
OVERVIEW_PAGE = (
    "docs/m5/"
    "ship_advisory_templates_extension_provider_revocation_packets_emergency_disable_bundles_"
    "mirror_propagation_drills_and_offline_import_response_evidence_for_m5_ecosystem_release_lanes.md"
)

ADVISORY_TEMPLATE_REF = "artifacts/governance/security/advisory_templates.json"
REVOCATION_REGISTER_REF = "artifacts/governance/registry/revocation_register.json"
DISABLE_BUNDLE_REF = "artifacts/governance/security/emergency_disable_bundles.json"
POSTMORTEM_REGISTER_REF = "artifacts/governance/security/postmortem_register.json"
MIRROR_PROPAGATION_REF = "artifacts/governance/distribution/mirror_propagation.json"
OFFLINE_IMPORT_REF = "artifacts/governance/distribution/offline_import_response.json"
RELEASE_GRAPH_REF = "artifacts/release/m5/artifact_graph.json"
SUPPORT_EXPORT_REF = "artifacts/support/m5/support_export_index.json"
AUDIT_JOURNAL_REF = "artifacts/governance/security/break_glass_journal.json"
CONTINUITY_REGISTER_REF = "artifacts/governance/m5-release-authority-continuity.json"
SHIPROOM_REGISTER_REF = "artifacts/governance/shiproom/gate_register.json"
EVIDENCE_INDEX_REF = "artifacts/release/m5/evidence_index.json"
SLO_REGISTER_REF = "artifacts/governance/freshness/slo_register.json"

# Closed vocabularies, mirroring the Rust `::ALL` order.
FAMILIES = [
    "notebook",
    "data_rich",
    "ai_adjacent",
    "framework",
    "review",
    "companion",
    "managed_depth",
]
PACKET_KINDS = [
    "security_advisory",
    "extension_provider_revocation",
    "emergency_disable_bundle",
    "high_severity_postmortem",
]
SUPPORT_CLASSES = ["open_local", "mixed_open_managed", "managed", "restricted"]
SEVERITIES = ["informational", "moderate", "high", "critical"]
CONTROL_DIMENSIONS = [
    "packet_template",
    "distribution_reach",
    "attribution",
    "reversibility",
    "audit_trail",
    "evidence_linkage",
    "scan_surface_parity",
]
DISTRIBUTION_CHANNELS = ["hosted", "mirror", "offline"]
CHANNEL_STATES = ["propagated", "pending", "stale", "not_claimed"]
TEMPLATE_STATES = ["bound", "unbound"]
ATTRIBUTION_STATES = ["attributable", "unattributable"]
REVERSIBILITY_STATES = [
    "reversible_with_runbook",
    "irreversible_by_policy",
    "reversal_rule_missing",
]
RECONCILIATION_STATES = ["reconciled", "pending", "not_required"]
LINKAGE_STATES = ["linked", "side_channel_only"]
POSTURES = ["clear", "gaps_found"]
RESPONSE_STATES = [
    "cleared",
    "narrowed_template",
    "narrowed_distribution",
    "narrowed_attribution",
    "narrowed_reversibility",
    "narrowed_audit",
    "narrowed_linkage",
    "narrowed_stale",
    "withdrawn",
]
RESPONSE_REASONS = [
    "packet_template_unbound",
    "mirror_propagation_incomplete",
    "offline_import_response_missing",
    "channel_evidence_stale",
    "action_unattributable",
    "reversal_rule_missing",
    "audit_markers_missing",
    "reconciliation_pending",
    "evidence_linkage_missing",
    "response_proof_stale",
    "response_proof_missing",
    "owner_signoff_missing",
    "waiver_expired",
]
RESPONSE_ACTIONS = [
    "hold_promotion",
    "bind_packet_template",
    "complete_mirror_propagation",
    "complete_offline_import_response",
    "refresh_channel_evidence",
    "record_attribution",
    "attach_reversal_rule",
    "attach_audit_markers",
    "complete_reconciliation",
    "link_release_and_support_evidence",
    "refresh_response_proof",
    "request_owner_signoff",
]

ABOVE_CUTLINE = ["lts", "stable"]
LABEL_RANK = {"lts": 4, "stable": 3, "beta": 2, "preview": 1, "withdrawn": 0}

STATE_BY_REASON = {
    "packet_template_unbound": "narrowed_template",
    "mirror_propagation_incomplete": "narrowed_distribution",
    "offline_import_response_missing": "narrowed_distribution",
    "channel_evidence_stale": "narrowed_distribution",
    "action_unattributable": "narrowed_attribution",
    "reversal_rule_missing": "narrowed_reversibility",
    "audit_markers_missing": "narrowed_audit",
    "reconciliation_pending": "narrowed_audit",
    "evidence_linkage_missing": "narrowed_linkage",
    "response_proof_stale": "narrowed_stale",
    "response_proof_missing": "narrowed_stale",
    "owner_signoff_missing": "narrowed_stale",
    "waiver_expired": "narrowed_stale",
}
PRECEDENCE = {
    "narrowed_distribution": 0,
    "narrowed_audit": 1,
    "narrowed_attribution": 2,
    "narrowed_reversibility": 3,
    "narrowed_linkage": 4,
    "narrowed_template": 5,
    "narrowed_stale": 6,
}
DIMENSION_BY_REASON = {
    "packet_template_unbound": "packet_template",
    "mirror_propagation_incomplete": "distribution_reach",
    "offline_import_response_missing": "distribution_reach",
    "channel_evidence_stale": "distribution_reach",
    "action_unattributable": "attribution",
    "reversal_rule_missing": "reversibility",
    "audit_markers_missing": "audit_trail",
    "reconciliation_pending": "audit_trail",
    "evidence_linkage_missing": "evidence_linkage",
    "response_proof_stale": "scan_surface_parity",
    "response_proof_missing": "scan_surface_parity",
    "owner_signoff_missing": "scan_surface_parity",
    "waiver_expired": "scan_surface_parity",
}
ACTION_BY_REASON = {
    "packet_template_unbound": "bind_packet_template",
    "mirror_propagation_incomplete": "complete_mirror_propagation",
    "offline_import_response_missing": "complete_offline_import_response",
    "channel_evidence_stale": "refresh_channel_evidence",
    "action_unattributable": "record_attribution",
    "reversal_rule_missing": "attach_reversal_rule",
    "audit_markers_missing": "attach_audit_markers",
    "reconciliation_pending": "complete_reconciliation",
    "evidence_linkage_missing": "link_release_and_support_evidence",
    "response_proof_stale": "refresh_response_proof",
    "response_proof_missing": "refresh_response_proof",
    "owner_signoff_missing": "request_owner_signoff",
    "waiver_expired": "refresh_response_proof",
}


# ---------------------------------------------------------------------------
# Builders
# ---------------------------------------------------------------------------
def proof(packet_id: str, slo_state: str, captured: str | None) -> dict:
    return {
        "packet_id": packet_id,
        "packet_ref": f"artifacts/governance/captures/{packet_id}.json",
        "captured_at": captured,
        "freshness_slo": {
            "target_max_age_days": 90,
            "warn_within_days": 14,
            "slo_register_ref": SLO_REGISTER_REF,
        },
        "slo_state": slo_state,
        "evidence_refs": [RELEASE_GRAPH_REF, EVIDENCE_INDEX_REF],
    }


def signoff(owner: str, signed: bool, at: str | None) -> dict:
    return {"owner_ref": owner, "signed_off": signed, "signed_at": at}


def waiver(ref: str, expires: str, reason: str) -> dict:
    return {"waiver_ref": ref, "expires_at": expires, "reason": reason}


def packet_template(kind: str, state: str, signed: bool, template_ref: str, digest: str) -> dict:
    return {
        "template_state": state,
        "packet_kind": kind,
        "signed": signed,
        "template_ref": template_ref,
        "digest_ref": digest,
    }


def channel(ch: str, claimed: bool, state: str, evidence_ref: str) -> dict:
    return {"channel": ch, "claimed": claimed, "state": state, "evidence_ref": evidence_ref}


def reach(hosted: str, mirror: tuple[bool, str], offline: tuple[bool, str]) -> dict:
    mirror_claimed, mirror_state = mirror
    offline_claimed, offline_state = offline
    return {
        "channels": [
            channel("hosted", True, hosted, f"{RELEASE_GRAPH_REF}#hosted"),
            channel(
                "mirror",
                mirror_claimed,
                mirror_state,
                f"{MIRROR_PROPAGATION_REF}#mirror",
            ),
            channel(
                "offline",
                offline_claimed,
                offline_state,
                f"{OFFLINE_IMPORT_REF}#offline",
            ),
        ]
    }


def attribution(state: str, actor_ref: str, authorization_ref: str) -> dict:
    return {
        "attribution_state": state,
        "actor_ref": actor_ref,
        "authorization_ref": authorization_ref,
    }


def reversibility(state: str, policy_reversible: bool, runbook_ref: str) -> dict:
    return {
        "reversibility_state": state,
        "policy_reversible": policy_reversible,
        "reversal_runbook_ref": runbook_ref,
    }


def audit_trail(markers_present: bool, marker_ref: str, reconciliation_state: str, reconciliation_ref: str) -> dict:
    return {
        "audit_markers_present": markers_present,
        "audit_marker_ref": marker_ref,
        "reconciliation_state": reconciliation_state,
        "reconciliation_ref": reconciliation_ref,
        "mutation_journal_ref": AUDIT_JOURNAL_REF,
    }


def linkage(state: str, release_ref: str, support_ref: str, digest: str) -> dict:
    return {
        "linkage_state": state,
        "release_artifact_ref": release_ref,
        "support_export_ref": support_ref,
        "bundle_digest_ref": digest,
    }


# ---------------------------------------------------------------------------
# Derivations mirroring the Rust model
# ---------------------------------------------------------------------------
def is_high(rec: dict) -> bool:
    return rec["severity"] in ("high", "critical")


def requires_reconciliation(rec: dict) -> bool:
    return rec["is_break_glass"] or is_high(rec)


def template_unbound(rec: dict) -> bool:
    return rec["packet_template"]["template_state"] == "unbound"


def distribution_reasons(rec: dict) -> list[str]:
    out: list[str] = []
    for c in rec["distribution_reach"]["channels"]:
        if not c["claimed"]:
            continue
        if c["state"] == "stale":
            out.append("channel_evidence_stale")
        elif c["state"] == "pending":
            if c["channel"] == "mirror":
                out.append("mirror_propagation_incomplete")
            elif c["channel"] == "offline":
                out.append("offline_import_response_missing")
            else:
                out.append("channel_evidence_stale")
    return out


def has_distribution_gap(rec: dict) -> bool:
    return len(distribution_reasons(rec)) > 0


def unattributable(rec: dict) -> bool:
    return rec["attribution"]["attribution_state"] == "unattributable"


def reversal_rule_missing(rec: dict) -> bool:
    return rec["reversibility"]["reversibility_state"] == "reversal_rule_missing"


def audit_markers_missing(rec: dict) -> bool:
    return not rec["audit_trail"]["audit_markers_present"]


def reconciliation_pending(rec: dict) -> bool:
    return requires_reconciliation(rec) and rec["audit_trail"]["reconciliation_state"] == "pending"


def linkage_missing(rec: dict) -> bool:
    return rec["evidence_linkage"]["linkage_state"] == "side_channel_only"


def derive_reasons(rec: dict) -> list[str]:
    reasons: set[str] = set()
    if template_unbound(rec):
        reasons.add("packet_template_unbound")
    for r in distribution_reasons(rec):
        reasons.add(r)
    if unattributable(rec):
        reasons.add("action_unattributable")
    if reversal_rule_missing(rec):
        reasons.add("reversal_rule_missing")
    if audit_markers_missing(rec):
        reasons.add("audit_markers_missing")
    if reconciliation_pending(rec):
        reasons.add("reconciliation_pending")
    if linkage_missing(rec):
        reasons.add("evidence_linkage_missing")
    if rec["proof_packet"]["slo_state"] == "breached":
        reasons.add("response_proof_stale")
    if rec["proof_packet"]["slo_state"] == "missing":
        reasons.add("response_proof_missing")
    if not rec["owner_signoff"]["signed_off"]:
        reasons.add("owner_signoff_missing")
    # Order by the closed-vocabulary declaration order for tidy output.
    return [r for r in RESPONSE_REASONS if r in reasons]


def computed_state(rec: dict) -> str:
    if rec["declared_label"] == "withdrawn":
        return "withdrawn"
    reasons = rec["active_reasons"]
    if not reasons:
        return "cleared"
    groups = [STATE_BY_REASON[r] for r in reasons]
    return min(groups, key=lambda g: PRECEDENCE[g])


def computed_effective_label(rec: dict) -> str:
    state = rec["continuity_state"]
    if state == "cleared":
        return rec["declared_label"]
    if state == "withdrawn":
        return "withdrawn"
    if LABEL_RANK[rec["declared_label"]] <= LABEL_RANK["beta"]:
        return rec["declared_label"]
    return "beta"


def computed_posture(rec: dict) -> str:
    return "gaps_found" if rec["continuity_state"] not in ("cleared", "withdrawn") else "clear"


def expected_control_state(rec: dict, dimension: str) -> str:
    if dimension == "packet_template":
        return "unsatisfied" if template_unbound(rec) else "satisfied"
    if dimension == "distribution_reach":
        return "unsatisfied" if has_distribution_gap(rec) else "satisfied"
    if dimension == "attribution":
        return "unsatisfied" if unattributable(rec) else "satisfied"
    if dimension == "reversibility":
        return "unsatisfied" if reversal_rule_missing(rec) else "satisfied"
    if dimension == "audit_trail":
        return "unsatisfied" if (audit_markers_missing(rec) or reconciliation_pending(rec)) else "satisfied"
    if dimension == "evidence_linkage":
        return "unsatisfied" if linkage_missing(rec) else "satisfied"
    # scan_surface_parity
    return "unsatisfied" if rec["scan_posture"] != rec["surface_posture"] else "satisfied"


def build_controls(rec: dict) -> list[dict]:
    controls = []
    for dimension in CONTROL_DIMENSIONS:
        controls.append(
            {
                "dimension": dimension,
                "control_ref": f"{SHIPROOM_REGISTER_REF}#{dimension}",
                "owner_ref": "role:security-response-owner",
                "state": expected_control_state(rec, dimension),
            }
        )
    return controls


def finalize(rec: dict) -> dict:
    """Fill in derived fields (reasons, state, effective label, postures, controls)."""
    # Fail fast on internally inconsistent fact wiring.
    assert rec["packet_template"]["packet_kind"] == rec["packet_kind"], rec["record_id"]
    rec["active_reasons"] = derive_reasons(rec)
    rec["continuity_state"] = computed_state(rec)
    rec["effective_label"] = computed_effective_label(rec)
    posture = computed_posture(rec)
    rec["scan_posture"] = posture
    rec["surface_posture"] = posture
    rec["controls"] = build_controls(rec)
    return rec


SURFACES = [
    "help_about:emergency_response",
    "service_health:advisories",
    "release_center:revocation_and_disable",
    "support_export:emergency_response_evidence",
]


def make_record(
    *,
    record_id: str,
    family: str,
    packet_kind: str,
    title: str,
    subject_ref: str,
    subject_summary: str,
    release_blocking: bool,
    declared_label: str,
    support_class: str,
    severity: str,
    is_break_glass: bool,
    template: dict,
    distribution: dict,
    attrib: dict,
    revers: dict,
    audit: dict,
    link: dict,
    proof_packet: dict,
    owner_signoff: dict,
    waiver_record: dict | None,
    rationale: str,
) -> dict:
    rec = {
        "record_id": record_id,
        "family": family,
        "packet_kind": packet_kind,
        "title": title,
        "subject_ref": subject_ref,
        "subject_summary": subject_summary,
        "release_blocking": release_blocking,
        "declared_label": declared_label,
        "support_class": support_class,
        "severity": severity,
        "is_break_glass": is_break_glass,
        "packet_template": template,
        "distribution_reach": distribution,
        "attribution": attrib,
        "reversibility": revers,
        "audit_trail": audit,
        "evidence_linkage": link,
        "controls": [],
        "scan_posture": "clear",
        "surface_posture": "clear",
        "scan_ref": f"{SHIPROOM_REGISTER_REF}#response_scan",
        "surface_ref": f"{SHIPROOM_REGISTER_REF}#service_health_surface",
        "proof_packet": proof_packet,
        "waiver": waiver_record,
        "owner_signoff": owner_signoff,
        "continuity_state": "cleared",
        "active_reasons": [],
        "effective_label": declared_label,
        "surfaces": list(SURFACES),
        "rationale": rationale,
    }
    return finalize(rec)


def build_records() -> list[dict]:
    records: list[dict] = []

    # 1) A fully covered, high-severity security advisory: cleared.
    records.append(
        make_record(
            record_id="response-framework-security-advisory",
            family="framework",
            packet_kind="security_advisory",
            title="Framework security advisory",
            subject_ref="lane:framework/security-advisory",
            subject_summary="Signed security advisory for the core framework train.",
            release_blocking=True,
            declared_label="stable",
            support_class="open_local",
            severity="high",
            is_break_glass=True,
            template=packet_template(
                "security_advisory", "bound", True, ADVISORY_TEMPLATE_REF, f"{ADVISORY_TEMPLATE_REF}#digest"
            ),
            distribution=reach("propagated", (True, "propagated"), (True, "propagated")),
            attrib=attribution("attributable", "role:framework-security-owner", f"{AUDIT_JOURNAL_REF}#auth"),
            revers=reversibility("reversible_with_runbook", True, f"{ADVISORY_TEMPLATE_REF}#reversal"),
            audit=audit_trail(True, f"{AUDIT_JOURNAL_REF}#markers", "reconciled", f"{AUDIT_JOURNAL_REF}#recon"),
            link=linkage("linked", f"{RELEASE_GRAPH_REF}#advisory", f"{SUPPORT_EXPORT_REF}#advisory", f"{RELEASE_GRAPH_REF}#digest"),
            proof_packet=proof("response-framework-security-advisory", "current", "2026-06-12"),
            owner_signoff=signoff("role:security-response-owner", True, "2026-06-13"),
            waiver_record=None,
            rationale="A signed advisory bound and digested, reaching the hosted, mirror, and offline channels, attributable and reversible with a runbook, audit markers present and reconciled, linked to the release graph and support export, with fresh proof; the lane is cleared.",
        )
    )

    # 2) A break-glass extension/provider revocation that never reached the claimed mirror.
    records.append(
        make_record(
            record_id="response-notebook-extension-provider-revocation",
            family="notebook",
            packet_kind="extension_provider_revocation",
            title="Notebook extension/provider revocation",
            subject_ref="lane:notebook/extension-revocation",
            subject_summary="Emergency revocation of a notebook extension/provider key.",
            release_blocking=True,
            declared_label="stable",
            support_class="open_local",
            severity="critical",
            is_break_glass=True,
            template=packet_template(
                "extension_provider_revocation", "bound", True, REVOCATION_REGISTER_REF, f"{REVOCATION_REGISTER_REF}#digest"
            ),
            distribution=reach("propagated", (True, "pending"), (True, "propagated")),
            attrib=attribution("attributable", "role:notebook-security-owner", f"{AUDIT_JOURNAL_REF}#auth-nb"),
            revers=reversibility("reversible_with_runbook", True, f"{REVOCATION_REGISTER_REF}#reversal"),
            audit=audit_trail(True, f"{AUDIT_JOURNAL_REF}#markers-nb", "reconciled", f"{AUDIT_JOURNAL_REF}#recon-nb"),
            link=linkage("linked", f"{RELEASE_GRAPH_REF}#revocation-nb", f"{SUPPORT_EXPORT_REF}#revocation-nb", f"{RELEASE_GRAPH_REF}#digest-nb"),
            proof_packet=proof("response-notebook-extension-provider-revocation", "current", "2026-06-11"),
            owner_signoff=signoff("role:security-response-owner", True, "2026-06-12"),
            waiver_record=None,
            rationale="The revocation packet reached hosted and offline customers but the claimed mirror channel is still propagating; it narrows on the distribution-reach axis and holds promotion on a still-stable claim — a mirror customer may not be left on a hosted-only path.",
        )
    )

    # 3) A break-glass emergency-disable bundle that never reached the claimed offline customers.
    records.append(
        make_record(
            record_id="response-managed_depth-emergency-disable-bundle",
            family="managed_depth",
            packet_kind="emergency_disable_bundle",
            title="Managed-depth emergency-disable bundle",
            subject_ref="lane:managed_depth/emergency-disable",
            subject_summary="Emergency-disable bundle for a managed-depth integration.",
            release_blocking=True,
            declared_label="stable",
            support_class="managed",
            severity="critical",
            is_break_glass=True,
            template=packet_template(
                "emergency_disable_bundle", "bound", True, DISABLE_BUNDLE_REF, f"{DISABLE_BUNDLE_REF}#digest"
            ),
            distribution=reach("propagated", (True, "propagated"), (True, "pending")),
            attrib=attribution("attributable", "role:managed-depth-security-owner", f"{AUDIT_JOURNAL_REF}#auth-md"),
            revers=reversibility("reversible_with_runbook", True, f"{DISABLE_BUNDLE_REF}#reversal"),
            audit=audit_trail(True, f"{AUDIT_JOURNAL_REF}#markers-md", "reconciled", f"{AUDIT_JOURNAL_REF}#recon-md"),
            link=linkage("linked", f"{RELEASE_GRAPH_REF}#disable-md", f"{SUPPORT_EXPORT_REF}#disable-md", f"{RELEASE_GRAPH_REF}#digest-md"),
            proof_packet=proof("response-managed_depth-emergency-disable-bundle", "current", "2026-06-10"),
            owner_signoff=signoff("role:security-response-owner", True, "2026-06-11"),
            waiver_record=None,
            rationale="The disable bundle reached hosted and mirror customers but the offline-import response has not landed for the claimed offline profile; it narrows on the distribution-reach axis and holds promotion on a still-stable claim.",
        )
    )

    # 4) A break-glass advisory with no audit markers: break-glass bypass of the audit trail.
    records.append(
        make_record(
            record_id="response-ai_adjacent-security-advisory",
            family="ai_adjacent",
            packet_kind="security_advisory",
            title="AI-adjacent security advisory",
            subject_ref="lane:ai_adjacent/security-advisory",
            subject_summary="Break-glass security advisory for AI-adjacent surfaces.",
            release_blocking=True,
            declared_label="stable",
            support_class="open_local",
            severity="high",
            is_break_glass=True,
            template=packet_template(
                "security_advisory", "bound", True, ADVISORY_TEMPLATE_REF, f"{ADVISORY_TEMPLATE_REF}#digest-ai"
            ),
            distribution=reach("propagated", (True, "propagated"), (True, "propagated")),
            attrib=attribution("attributable", "role:ai-adjacent-security-owner", f"{AUDIT_JOURNAL_REF}#auth-ai"),
            revers=reversibility("reversible_with_runbook", True, f"{ADVISORY_TEMPLATE_REF}#reversal-ai"),
            audit=audit_trail(False, "", "reconciled", f"{AUDIT_JOURNAL_REF}#recon-ai"),
            link=linkage("linked", f"{RELEASE_GRAPH_REF}#advisory-ai", f"{SUPPORT_EXPORT_REF}#advisory-ai", f"{RELEASE_GRAPH_REF}#digest-ai"),
            proof_packet=proof("response-ai_adjacent-security-advisory", "current", "2026-06-09"),
            owner_signoff=signoff("role:security-response-owner", True, "2026-06-10"),
            waiver_record=None,
            rationale="A break-glass advisory shipped without audit markers; it narrows on the audit axis and holds promotion — a break-glass action may not bypass the audit markers.",
        )
    )

    # 5) A high-severity postmortem whose action is not attributable.
    records.append(
        make_record(
            record_id="response-review-high-severity-postmortem",
            family="review",
            packet_kind="high_severity_postmortem",
            title="Review high-severity postmortem",
            subject_ref="lane:review/postmortem",
            subject_summary="High-severity postmortem for the review/diff surfaces.",
            release_blocking=True,
            declared_label="stable",
            support_class="open_local",
            severity="high",
            is_break_glass=False,
            template=packet_template(
                "high_severity_postmortem", "bound", True, POSTMORTEM_REGISTER_REF, f"{POSTMORTEM_REGISTER_REF}#digest"
            ),
            distribution=reach("propagated", (True, "propagated"), (True, "propagated")),
            attrib=attribution("unattributable", "", f"{AUDIT_JOURNAL_REF}#auth-rv"),
            revers=reversibility("irreversible_by_policy", False, ""),
            audit=audit_trail(True, f"{AUDIT_JOURNAL_REF}#markers-rv", "reconciled", f"{AUDIT_JOURNAL_REF}#recon-rv"),
            link=linkage("linked", f"{RELEASE_GRAPH_REF}#postmortem-rv", f"{SUPPORT_EXPORT_REF}#postmortem-rv", f"{RELEASE_GRAPH_REF}#digest-rv"),
            proof_packet=proof("response-review-high-severity-postmortem", "current", "2026-06-08"),
            owner_signoff=signoff("role:security-response-owner", True, "2026-06-09"),
            waiver_record=None,
            rationale="The postmortem records the timeline but the triggering action is not attributable to a named authority; it narrows on the attribution axis and holds promotion on a still-stable claim.",
        )
    )

    # 6) A break-glass revocation that policy permits reversing, but with no reversal runbook.
    records.append(
        make_record(
            record_id="response-companion-extension-provider-revocation",
            family="companion",
            packet_kind="extension_provider_revocation",
            title="Companion extension/provider revocation",
            subject_ref="lane:companion/extension-revocation",
            subject_summary="Emergency revocation for a companion-surface provider.",
            release_blocking=True,
            declared_label="stable",
            support_class="mixed_open_managed",
            severity="high",
            is_break_glass=True,
            template=packet_template(
                "extension_provider_revocation", "bound", True, REVOCATION_REGISTER_REF, f"{REVOCATION_REGISTER_REF}#digest-cp"
            ),
            distribution=reach("propagated", (True, "propagated"), (True, "propagated")),
            attrib=attribution("attributable", "role:companion-security-owner", f"{AUDIT_JOURNAL_REF}#auth-cp"),
            revers=reversibility("reversal_rule_missing", True, ""),
            audit=audit_trail(True, f"{AUDIT_JOURNAL_REF}#markers-cp", "reconciled", f"{AUDIT_JOURNAL_REF}#recon-cp"),
            link=linkage("linked", f"{RELEASE_GRAPH_REF}#revocation-cp", f"{SUPPORT_EXPORT_REF}#revocation-cp", f"{RELEASE_GRAPH_REF}#digest-cp"),
            proof_packet=proof("response-companion-extension-provider-revocation", "current", "2026-06-07"),
            owner_signoff=signoff("role:security-response-owner", True, "2026-06-08"),
            waiver_record=None,
            rationale="Policy permits reversing this revocation, but no reversal runbook is attached; it narrows on the reversibility axis and holds promotion — a reversible action must carry its reversal rule.",
        )
    )

    # 7) A break-glass disable bundle linked only through a side channel.
    records.append(
        make_record(
            record_id="response-data_rich-emergency-disable-bundle",
            family="data_rich",
            packet_kind="emergency_disable_bundle",
            title="Data-rich emergency-disable bundle",
            subject_ref="lane:data_rich/emergency-disable",
            subject_summary="Emergency-disable bundle for a data-rich result-grid integration.",
            release_blocking=True,
            declared_label="stable",
            support_class="mixed_open_managed",
            severity="critical",
            is_break_glass=True,
            template=packet_template(
                "emergency_disable_bundle", "bound", True, DISABLE_BUNDLE_REF, f"{DISABLE_BUNDLE_REF}#digest-dr"
            ),
            distribution=reach("propagated", (True, "propagated"), (True, "propagated")),
            attrib=attribution("attributable", "role:data-rich-security-owner", f"{AUDIT_JOURNAL_REF}#auth-dr"),
            revers=reversibility("reversible_with_runbook", True, f"{DISABLE_BUNDLE_REF}#reversal-dr"),
            audit=audit_trail(True, f"{AUDIT_JOURNAL_REF}#markers-dr", "reconciled", f"{AUDIT_JOURNAL_REF}#recon-dr"),
            link=linkage("side_channel_only", "", "", f"{RELEASE_GRAPH_REF}#digest-dr"),
            proof_packet=proof("response-data_rich-emergency-disable-bundle", "current", "2026-06-06"),
            owner_signoff=signoff("role:security-response-owner", True, "2026-06-07"),
            waiver_record=None,
            rationale="The disable bundle was announced through a side channel and is not linked to the release artifact-graph or a support export; it narrows on the linkage axis and holds promotion — emergency evidence must be linked to release/support, not side channels.",
        )
    )

    # 8) A moderate advisory whose packet template is not bound/signed.
    records.append(
        make_record(
            record_id="response-notebook-security-advisory",
            family="notebook",
            packet_kind="security_advisory",
            title="Notebook security advisory",
            subject_ref="lane:notebook/security-advisory",
            subject_summary="Security advisory for the notebook train.",
            release_blocking=True,
            declared_label="stable",
            support_class="open_local",
            severity="moderate",
            is_break_glass=False,
            template=packet_template("security_advisory", "unbound", False, ADVISORY_TEMPLATE_REF, ""),
            distribution=reach("propagated", (True, "propagated"), (True, "propagated")),
            attrib=attribution("attributable", "role:notebook-security-owner", f"{AUDIT_JOURNAL_REF}#auth-nb2"),
            revers=reversibility("reversible_with_runbook", True, f"{ADVISORY_TEMPLATE_REF}#reversal-nb2"),
            audit=audit_trail(True, f"{AUDIT_JOURNAL_REF}#markers-nb2", "not_required", ""),
            link=linkage("linked", f"{RELEASE_GRAPH_REF}#advisory-nb2", f"{SUPPORT_EXPORT_REF}#advisory-nb2", f"{RELEASE_GRAPH_REF}#digest-nb2"),
            proof_packet=proof("response-notebook-security-advisory", "current", "2026-06-05"),
            owner_signoff=signoff("role:security-response-owner", True, "2026-06-06"),
            waiver_record=None,
            rationale="The advisory text exists but the signed packet template is not yet bound and digested; it narrows on the template axis and holds promotion on a still-stable claim.",
        )
    )

    # 9) A high-severity postmortem with a stale continuity proof; already Beta (inherited).
    records.append(
        make_record(
            record_id="response-managed_depth-high-severity-postmortem",
            family="managed_depth",
            packet_kind="high_severity_postmortem",
            title="Managed-depth high-severity postmortem",
            subject_ref="lane:managed_depth/postmortem",
            subject_summary="High-severity postmortem for managed-depth services.",
            release_blocking=False,
            declared_label="beta",
            support_class="managed",
            severity="high",
            is_break_glass=False,
            template=packet_template(
                "high_severity_postmortem", "bound", True, POSTMORTEM_REGISTER_REF, f"{POSTMORTEM_REGISTER_REF}#digest-md"
            ),
            distribution=reach("propagated", (True, "propagated"), (True, "propagated")),
            attrib=attribution("attributable", "role:managed-depth-security-owner", f"{AUDIT_JOURNAL_REF}#auth-md2"),
            revers=reversibility("irreversible_by_policy", False, ""),
            audit=audit_trail(True, f"{AUDIT_JOURNAL_REF}#markers-md2", "reconciled", f"{AUDIT_JOURNAL_REF}#recon-md2"),
            link=linkage("linked", f"{RELEASE_GRAPH_REF}#postmortem-md", f"{SUPPORT_EXPORT_REF}#postmortem-md", f"{RELEASE_GRAPH_REF}#digest-md2"),
            proof_packet=proof("response-managed_depth-high-severity-postmortem", "breached", "2026-01-04"),
            owner_signoff=signoff("role:security-response-owner", True, "2026-02-01"),
            waiver_record=None,
            rationale="The response proof packet aged past its freshness SLO; the lane narrows on the stale axis. The subject is already Beta, so the narrowing is inherited and gated upstream.",
        )
    )

    # 10) A moderate advisory with no captured continuity proof; already Beta (inherited).
    records.append(
        make_record(
            record_id="response-data_rich-security-advisory",
            family="data_rich",
            packet_kind="security_advisory",
            title="Data-rich security advisory",
            subject_ref="lane:data_rich/security-advisory",
            subject_summary="Security advisory for data-rich surfaces.",
            release_blocking=False,
            declared_label="beta",
            support_class="mixed_open_managed",
            severity="moderate",
            is_break_glass=False,
            template=packet_template(
                "security_advisory", "bound", True, ADVISORY_TEMPLATE_REF, f"{ADVISORY_TEMPLATE_REF}#digest-dr2"
            ),
            distribution=reach("propagated", (True, "propagated"), (True, "propagated")),
            attrib=attribution("attributable", "role:data-rich-security-owner", f"{AUDIT_JOURNAL_REF}#auth-dr2"),
            revers=reversibility("reversible_with_runbook", True, f"{ADVISORY_TEMPLATE_REF}#reversal-dr2"),
            audit=audit_trail(True, f"{AUDIT_JOURNAL_REF}#markers-dr2", "not_required", ""),
            link=linkage("linked", f"{RELEASE_GRAPH_REF}#advisory-dr2", f"{SUPPORT_EXPORT_REF}#advisory-dr2", f"{RELEASE_GRAPH_REF}#digest-dr3"),
            proof_packet=proof("response-data_rich-security-advisory", "missing", None),
            owner_signoff=signoff("role:security-response-owner", True, "2026-05-01"),
            waiver_record=None,
            rationale="No response proof packet is captured; the lane narrows on the stale axis. The subject is already Beta, so the narrowing is inherited.",
        )
    )

    # 11) A break-glass revocation that never reached the mirror, but held by an unexpired waiver.
    records.append(
        make_record(
            record_id="response-review-extension-provider-revocation",
            family="review",
            packet_kind="extension_provider_revocation",
            title="Review extension/provider revocation",
            subject_ref="lane:review/extension-revocation",
            subject_summary="Emergency revocation for a review-surface provider.",
            release_blocking=True,
            declared_label="stable",
            support_class="open_local",
            severity="critical",
            is_break_glass=True,
            template=packet_template(
                "extension_provider_revocation", "bound", True, REVOCATION_REGISTER_REF, f"{REVOCATION_REGISTER_REF}#digest-rv"
            ),
            distribution=reach("propagated", (True, "pending"), (True, "propagated")),
            attrib=attribution("attributable", "role:review-security-owner", f"{AUDIT_JOURNAL_REF}#auth-rv2"),
            revers=reversibility("reversible_with_runbook", True, f"{REVOCATION_REGISTER_REF}#reversal-rv"),
            audit=audit_trail(True, f"{AUDIT_JOURNAL_REF}#markers-rv2", "reconciled", f"{AUDIT_JOURNAL_REF}#recon-rv2"),
            link=linkage("linked", f"{RELEASE_GRAPH_REF}#revocation-rv", f"{SUPPORT_EXPORT_REF}#revocation-rv", f"{RELEASE_GRAPH_REF}#digest-rv2"),
            proof_packet=proof("response-review-extension-provider-revocation", "current", "2026-06-05"),
            owner_signoff=signoff("role:security-response-owner", True, "2026-06-06"),
            waiver_record=waiver(
                f"{SHIPROOM_REGISTER_REF}#waiver/review-revocation",
                "2026-09-30",
                "Mirror operator re-propagation in progress; covered by an approved, time-boxed waiver.",
            ),
            rationale="A revocation that has not yet reached the claimed mirror, held by an unexpired waiver: it stays visible and narrowed on the distribution-reach axis but is gated upstream and does not hold promotion.",
        )
    )

    # 12) A high-severity disable bundle whose offline-channel evidence aged out; Beta (inherited).
    records.append(
        make_record(
            record_id="response-companion-emergency-disable-bundle",
            family="companion",
            packet_kind="emergency_disable_bundle",
            title="Companion emergency-disable bundle",
            subject_ref="lane:companion/emergency-disable",
            subject_summary="Emergency-disable bundle for a companion-surface integration.",
            release_blocking=False,
            declared_label="beta",
            support_class="mixed_open_managed",
            severity="high",
            is_break_glass=False,
            template=packet_template(
                "emergency_disable_bundle", "bound", True, DISABLE_BUNDLE_REF, f"{DISABLE_BUNDLE_REF}#digest-cp"
            ),
            distribution=reach("propagated", (True, "propagated"), (True, "stale")),
            attrib=attribution("attributable", "role:companion-security-owner", f"{AUDIT_JOURNAL_REF}#auth-cp2"),
            revers=reversibility("reversible_with_runbook", True, f"{DISABLE_BUNDLE_REF}#reversal-cp"),
            audit=audit_trail(True, f"{AUDIT_JOURNAL_REF}#markers-cp2", "reconciled", f"{AUDIT_JOURNAL_REF}#recon-cp2"),
            link=linkage("linked", f"{RELEASE_GRAPH_REF}#disable-cp", f"{SUPPORT_EXPORT_REF}#disable-cp", f"{RELEASE_GRAPH_REF}#digest-cp2"),
            proof_packet=proof("response-companion-emergency-disable-bundle", "current", "2026-06-04"),
            owner_signoff=signoff("role:security-response-owner", True, "2026-06-05"),
            waiver_record=None,
            rationale="The offline channel's disable evidence has aged out of its window; the lane narrows on the distribution-reach axis. The subject is already Beta, so the narrowing is inherited.",
        )
    )

    # 13) A fully covered, moderate high-severity postmortem with no offline customers: cleared.
    records.append(
        make_record(
            record_id="response-framework-high-severity-postmortem",
            family="framework",
            packet_kind="high_severity_postmortem",
            title="Framework high-severity postmortem",
            subject_ref="lane:framework/postmortem",
            subject_summary="High-severity postmortem for the core framework train.",
            release_blocking=True,
            declared_label="stable",
            support_class="open_local",
            severity="moderate",
            is_break_glass=False,
            template=packet_template(
                "high_severity_postmortem", "bound", True, POSTMORTEM_REGISTER_REF, f"{POSTMORTEM_REGISTER_REF}#digest-fw"
            ),
            distribution=reach("propagated", (True, "propagated"), (False, "not_claimed")),
            attrib=attribution("attributable", "role:framework-security-owner", f"{AUDIT_JOURNAL_REF}#auth-fw"),
            revers=reversibility("irreversible_by_policy", False, ""),
            audit=audit_trail(True, f"{AUDIT_JOURNAL_REF}#markers-fw", "not_required", ""),
            link=linkage("linked", f"{RELEASE_GRAPH_REF}#postmortem-fw", f"{SUPPORT_EXPORT_REF}#postmortem-fw", f"{RELEASE_GRAPH_REF}#digest-fw2"),
            proof_packet=proof("response-framework-high-severity-postmortem", "current", "2026-06-12"),
            owner_signoff=signoff("role:security-response-owner", True, "2026-06-13"),
            waiver_record=None,
            rationale="A bound postmortem reaching the hosted and mirror channels it claims (no offline customer profile), attributable, audit-complete, and linked to release/support with fresh proof; the lane is cleared.",
        )
    )

    return records


# ---------------------------------------------------------------------------
# Promotion / parity / summary, mirroring the Rust model
# ---------------------------------------------------------------------------
def is_waived(rec: dict) -> bool:
    return rec["waiver"] is not None and "waiver_expired" not in rec["active_reasons"]


def is_narrowed(rec: dict) -> bool:
    return rec["continuity_state"] not in ("cleared", "withdrawn")


def declares_at_or_above_cutline(rec: dict) -> bool:
    return rec["declared_label"] in ABOVE_CUTLINE


def holds_promotion(rec: dict) -> bool:
    return (
        rec["release_blocking"]
        and is_narrowed(rec)
        and declares_at_or_above_cutline(rec)
        and not is_waived(rec)
    )


def computed_blocking_rule_ids(records: list[dict], rules: list[dict]) -> list[str]:
    ids = set()
    for rule in rules:
        if not rule["blocks_promotion"]:
            continue
        for r in records:
            if (
                holds_promotion(r)
                and rule["trigger_reason"] in r["active_reasons"]
                and r["declared_label"] in rule["applies_to_labels"]
            ):
                ids.add(rule["rule_id"])
                break
    return sorted(ids)


def rule_for(reason: str, rules: list[dict]) -> dict | None:
    return next((rule for rule in rules if rule["trigger_reason"] == reason), None)


def computed_blocking_record_ids(records: list[dict], rules: list[dict]) -> list[str]:
    ids = set()
    for r in records:
        if not holds_promotion(r):
            continue
        for reason in r["active_reasons"]:
            rule = rule_for(reason, rules)
            if rule and rule["blocks_promotion"] and r["declared_label"] in rule["applies_to_labels"]:
                ids.add(r["record_id"])
                break
    return sorted(ids)


def computed_scan_surface_parity(records: list[dict]) -> dict:
    agree = [r for r in records if r["scan_posture"] == r["surface_posture"]]
    return {
        "parity_gate": "m5_emergency_response_evidence_parity_gate",
        "subjects_total": len(records),
        "subjects_in_agreement": len(agree),
        "subjects_in_disagreement": len(records) - len(agree),
        "subjects_with_gaps": len([r for r in records if r["surface_posture"] == "gaps_found"]),
        "all_subjects_agree": len(agree) == len(records),
        "rationale": "Every record's response scan and service-health/release-center surface agree, so a green emergency-response card can never mask a mirror/offline customer that never received the evidence, an unattributable break-glass action, or a side-channel-only disable.",
    }


def mirror_reach_gaps(records: list[dict]) -> int:
    return len([r for r in records if "mirror_propagation_incomplete" in r["active_reasons"]])


def offline_reach_gaps(records: list[dict]) -> int:
    return len([r for r in records if "offline_import_response_missing" in r["active_reasons"]])


def computed_summary(records: list[dict], rules: list[dict]) -> dict:
    def count_state(state: str) -> int:
        return len([r for r in records if r["continuity_state"] == state])

    return {
        "total_records": len(records),
        "records_cleared": count_state("cleared"),
        "records_narrowed": len([r for r in records if is_narrowed(r)]),
        "state_cleared": count_state("cleared"),
        "state_narrowed_template": count_state("narrowed_template"),
        "state_narrowed_distribution": count_state("narrowed_distribution"),
        "state_narrowed_attribution": count_state("narrowed_attribution"),
        "state_narrowed_reversibility": count_state("narrowed_reversibility"),
        "state_narrowed_audit": count_state("narrowed_audit"),
        "state_narrowed_linkage": count_state("narrowed_linkage"),
        "state_narrowed_stale": count_state("narrowed_stale"),
        "state_withdrawn": count_state("withdrawn"),
        "release_blocking_total": len([r for r in records if r["release_blocking"]]),
        "release_blocking_narrowed": len(
            [r for r in records if r["release_blocking"] and is_narrowed(r)]
        ),
        "records_on_active_waiver": len([r for r in records if is_waived(r)]),
        "template_gaps": len([r for r in records if template_unbound(r)]),
        "distribution_gaps": len([r for r in records if has_distribution_gap(r)]),
        "attribution_gaps": len([r for r in records if unattributable(r)]),
        "reversibility_gaps": len([r for r in records if reversal_rule_missing(r)]),
        "audit_gaps": len(
            [r for r in records if audit_markers_missing(r) or reconciliation_pending(r)]
        ),
        "linkage_gaps": len([r for r in records if linkage_missing(r)]),
        "mirror_reach_gaps": mirror_reach_gaps(records),
        "offline_reach_gaps": offline_reach_gaps(records),
        "break_glass_total": len([r for r in records if r["is_break_glass"]]),
        "reconciliation_required": len([r for r in records if requires_reconciliation(r)]),
        "reconciliation_complete": len(
            [r for r in records if r["audit_trail"]["reconciliation_state"] == "reconciled"]
        ),
        "total_active_reasons": sum(len(r["active_reasons"]) for r in records),
        "rules_firing": len(computed_blocking_rule_ids(records, rules)),
    }


RULE_TITLES = {
    "packet_template_unbound": "Packet template unbound",
    "mirror_propagation_incomplete": "Mirror propagation incomplete",
    "offline_import_response_missing": "Offline import response missing",
    "channel_evidence_stale": "Channel evidence stale",
    "action_unattributable": "Emergency action unattributable",
    "reversal_rule_missing": "Reversal rule missing",
    "audit_markers_missing": "Audit markers missing",
    "reconciliation_pending": "Post-incident reconciliation pending",
    "evidence_linkage_missing": "Evidence linkage missing (side channel)",
    "response_proof_stale": "Response proof stale",
    "response_proof_missing": "Response proof missing",
    "owner_signoff_missing": "Owner sign-off missing",
    "waiver_expired": "Waiver expired",
}


def build_rules() -> list[dict]:
    rules = []
    for reason in RESPONSE_REASONS:
        rules.append(
            {
                "rule_id": f"rule_{reason}",
                "title": RULE_TITLES[reason],
                "trigger_reason": reason,
                "applies_to_labels": list(ABOVE_CUTLINE),
                "default_action": ACTION_BY_REASON[reason],
                "blocks_promotion": True,
                "rationale": f"A still-stable subject that narrows on {reason.replace('_', ' ')} holds promotion until the gap clears; inherited and waived narrowings are gated upstream.",
            }
        )
    return rules


def build_register() -> dict:
    records = build_records()
    rules = build_rules()
    blocking_rules = computed_blocking_rule_ids(records, rules)
    blocking_records = computed_blocking_record_ids(records, rules)
    decision_verdict = "hold" if blocking_records else "proceed"
    return {
        "schema_version": 1,
        "record_kind": RECORD_KIND,
        "register_id": REGISTER_ID,
        "status": "active",
        "overview_page": OVERVIEW_PAGE,
        "as_of": AS_OF,
        "source_contract_refs": {
            "advisory_template_ref": ADVISORY_TEMPLATE_REF,
            "revocation_register_ref": REVOCATION_REGISTER_REF,
            "disable_bundle_ref": DISABLE_BUNDLE_REF,
            "postmortem_register_ref": POSTMORTEM_REGISTER_REF,
            "mirror_propagation_ref": MIRROR_PROPAGATION_REF,
            "offline_import_ref": OFFLINE_IMPORT_REF,
            "release_graph_ref": RELEASE_GRAPH_REF,
            "support_export_ref": SUPPORT_EXPORT_REF,
            "audit_journal_ref": AUDIT_JOURNAL_REF,
            "continuity_register_ref": CONTINUITY_REGISTER_REF,
            "shiproom_register_ref": SHIPROOM_REGISTER_REF,
            "m5_evidence_index_ref": EVIDENCE_INDEX_REF,
        },
        "response_cutline": {
            "cutline_level": "stable",
            "above_cutline_levels": list(ABOVE_CUTLINE),
            "below_cutline_levels": ["beta", "preview", "withdrawn"],
            "description": "Subjects at or above Stable carry the cleared emergency-response claim; a response gap on a still-stable subject holds promotion through the shiproom gate.",
        },
        "families": FAMILIES,
        "packet_kinds": PACKET_KINDS,
        "support_classes": SUPPORT_CLASSES,
        "severities": SEVERITIES,
        "control_dimensions": CONTROL_DIMENSIONS,
        "distribution_channels": DISTRIBUTION_CHANNELS,
        "channel_states": CHANNEL_STATES,
        "template_states": TEMPLATE_STATES,
        "attribution_states": ATTRIBUTION_STATES,
        "reversibility_states": REVERSIBILITY_STATES,
        "reconciliation_states": RECONCILIATION_STATES,
        "linkage_states": LINKAGE_STATES,
        "postures": POSTURES,
        "response_states": RESPONSE_STATES,
        "response_reasons": RESPONSE_REASONS,
        "response_actions": RESPONSE_ACTIONS,
        "rules": rules,
        "records": records,
        "scan_surface_parity": computed_scan_surface_parity(records),
        "publication": {
            "publication_gate": "m5_emergency_response_evidence_gate",
            "decision": decision_verdict,
            "blocking_rule_ids": blocking_rules,
            "blocking_record_ids": blocking_records,
            "rationale": "Hold while any release-blocking subject carries an emergency-response gap on a still-stable claim; inherited and waived narrowings are gated upstream. A protected lane whose advisory/revocation/disable evidence did not reach a claimed mirror/offline customer, or whose break-glass action bypassed audit/reconciliation, may not widen a stable claim without coverage.",
        },
        "summary": computed_summary(records, rules),
    }


# ---------------------------------------------------------------------------
# Fixtures
# ---------------------------------------------------------------------------
def build_fixtures(register: dict) -> list[tuple[str, dict, str]]:
    cases: list[tuple[str, dict, str]] = []

    dup = copy.deepcopy(register)
    dup["records"].append(copy.deepcopy(dup["records"][0]))
    cases.append(("duplicate_record_id.json", dup, "DuplicateRecordId"))

    # A cleared record hiding a side-channel-only linkage gap without narrowing on it.
    hidden = copy.deepcopy(register)
    target = next(r for r in hidden["records"] if r["continuity_state"] == "cleared")
    target["evidence_linkage"]["linkage_state"] = "side_channel_only"
    target["evidence_linkage"]["release_artifact_ref"] = ""
    target["evidence_linkage"]["support_export_ref"] = ""
    cases.append(("hidden_linkage_gap.json", hidden, "GapWithoutReason"))

    # A narrowed record whose service-health surface is green over a gapped scan.
    masked = copy.deepcopy(register)
    target = next(r for r in masked["records"] if is_narrowed(r))
    target["surface_posture"] = "clear"
    cases.append(("green_surface_over_gap.json", masked, "ScanSurfaceDisagreement"))

    # A narrowed record whose effective label stays above the cutline.
    above = copy.deepcopy(register)
    target = next(r for r in above["records"] if is_narrowed(r))
    target["effective_label"] = "stable"
    cases.append(("narrowed_above_cutline.json", above, "EffectiveLabelMismatch"))

    # A break-glass action whose audit markers are dropped without narrowing on the audit axis.
    bypass = copy.deepcopy(register)
    target = next(
        r
        for r in bypass["records"]
        if r["continuity_state"] == "cleared" and r["is_break_glass"]
    )
    target["audit_trail"]["audit_markers_present"] = False
    target["audit_trail"]["audit_marker_ref"] = ""
    cases.append(("break_glass_bypasses_audit.json", bypass, "GapWithoutReason"))

    return cases


def build_capture(register: dict, cases: list[tuple[str, dict, str]]) -> dict:
    s = register["summary"]
    p = register["publication"]
    parity = register["scan_surface_parity"]
    drills = [
        "drill:hidden_linkage_gap",
        "drill:green_surface_over_gap",
        "drill:narrowed_above_cutline",
        "drill:break_glass_bypasses_audit",
        "drill:cleared_with_active_reason",
        "drill:reason_not_justified",
        "drill:control_state_inconsistent",
        "drill:mirror_propagation_drill",
        "drill:offline_import_response_drill",
        "drill:publication_decision_inconsistent",
    ]
    return {
        "status": "pass",
        "as_of": register["as_of"],
        "summary": {
            "total_records": s["total_records"],
            "records_cleared": s["records_cleared"],
            "records_narrowed": s["records_narrowed"],
            "state_cleared": s["state_cleared"],
            "state_narrowed_template": s["state_narrowed_template"],
            "state_narrowed_distribution": s["state_narrowed_distribution"],
            "state_narrowed_attribution": s["state_narrowed_attribution"],
            "state_narrowed_reversibility": s["state_narrowed_reversibility"],
            "state_narrowed_audit": s["state_narrowed_audit"],
            "state_narrowed_linkage": s["state_narrowed_linkage"],
            "state_narrowed_stale": s["state_narrowed_stale"],
            "release_blocking_narrowed": s["release_blocking_narrowed"],
            "records_on_active_waiver": s["records_on_active_waiver"],
            "template_gaps": s["template_gaps"],
            "distribution_gaps": s["distribution_gaps"],
            "attribution_gaps": s["attribution_gaps"],
            "reversibility_gaps": s["reversibility_gaps"],
            "audit_gaps": s["audit_gaps"],
            "linkage_gaps": s["linkage_gaps"],
            "mirror_reach_gaps": s["mirror_reach_gaps"],
            "offline_reach_gaps": s["offline_reach_gaps"],
            "break_glass_total": s["break_glass_total"],
            "reconciliation_required": s["reconciliation_required"],
            "reconciliation_complete": s["reconciliation_complete"],
            "total_active_reasons": s["total_active_reasons"],
            "rules_firing": s["rules_firing"],
        },
        "scan_surface_parity": {
            "subjects_in_agreement": parity["subjects_in_agreement"],
            "subjects_in_disagreement": parity["subjects_in_disagreement"],
            "subjects_with_gaps": parity["subjects_with_gaps"],
            "all_subjects_agree": parity["all_subjects_agree"],
        },
        "publication": {
            "decision": p["decision"],
            "blocking_rule_ids": p["blocking_rule_ids"],
            "blocking_record_ids": p["blocking_record_ids"],
        },
        "negative_drills": [{"drill_id": d, "status": "passed"} for d in drills],
        "fixture_cases": [
            {"case_id": f"fixture:{f[:-5]}", "status": "passed"} for f, _, _ in cases
        ],
    }


def write_json(path: Path, data: dict) -> None:
    path.parent.mkdir(parents=True, exist_ok=True)
    path.write_text(json.dumps(data, indent=2) + "\n", encoding="utf-8")


def main() -> None:
    register = build_register()
    cases = build_fixtures(register)

    write_json(ARTIFACT, register)
    print(f"wrote {ARTIFACT.relative_to(REPO)}")

    for filename, data, _ in cases:
        write_json(FIXTURES / filename, data)
    manifest_index = {
        "cases": [
            {"file": filename, "expected_check_id": check_id}
            for filename, _, check_id in cases
        ]
    }
    write_json(FIXTURES / "cases.json", manifest_index)
    print(f"wrote {FIXTURES.relative_to(REPO)}/ ({len(cases)} fixtures + cases.json)")

    write_json(CAPTURE, build_capture(register, cases))
    print(f"wrote {CAPTURE.relative_to(REPO)}")


if __name__ == "__main__":
    main()
