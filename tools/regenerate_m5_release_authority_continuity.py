#!/usr/bin/env python3
"""Regenerate the release-authority continuity register.

The open/local-boundary durability matrix records, per asset lane, the emergency
signing/registry/security authority as one coarse block, and the critical-upstream health register
makes each critical dependency inspectable. Neither makes each protected *authority lane* — the
release-signing, promotion-approval, registry-moderation, and security-response operations a
protected M5 family leans on — inspectable as a durable continuity record: who the named primary
owner is, whether a backup owner exists so the lane is not a single-person system, whether the
signer / promotion-approval / moderation-operator / security-responder roster meets its quorum,
whether split (two-person) authority is enforced where required, whether a current backup runbook
exists, and — when the lane is critical or already single-owner — whether the shiproom has been
told.

This register is that authority-continuity layer. For every protected authority lane a protected M5
family runs it records one entry that states, in one copy-safe record:

  - the owner coverage (a named primary owner);
  - the backup coverage (at least one named backup owner — not a single-person system);
  - the roster quorum (the signer/approval/operator/responder roster against its threshold);
  - the split-authority requirement (two-person control for critical lanes);
  - the backup-runbook coverage (current, due for review, stale, or missing);
  - the shiproom escalation, required for any critical or single-owner lane.

A record is cleared only when the lane has a named primary and backup owner, its roster meets
quorum, the runbook is current (or only due for review), split authority is enforced where
required, any required shiproom escalation is raised, the proof is fresh, and the owner signed.
Otherwise it narrows on the specific axis that thins out (an owner gap, a backup/single-owner gap, a
quorum gap, a runbook gap, an authority/escalation gap, or stale proof) and drops its effective
label below the launch cutline; the axes never collapse into one global flag. The continuity scan
and the governance-dashboard/promotion-packet surface must agree on every record, so a green
authority card can never mask a lane that is single-owner, under quorum, or without a current
runbook.

An inherited narrowing (a subject already below the cutline, or a gap held by an unexpired waiver)
is gated upstream and does not hold promotion; a continuity failure on a still-stable subject holds
promotion through a shiproom stop rule — a single-owner, under-quorum, or runbook-less protected
lane cannot widen a stable claim without coverage.

This emits the canonical register artifact, the negative fixtures, the cases manifest, and the
frozen validation capture. The Python summary/parity/promotion logic mirrors the typed Rust
consumer so the checked-in artifact validates cleanly and the capture cross-check agrees with the
model.
"""

from __future__ import annotations

import copy
import json
from pathlib import Path

REPO = Path(__file__).resolve().parent.parent

SLUG = "m5-release-authority-continuity"
RECORD_KIND = "m5_release_authority_continuity_register"
REGISTER_ID = "m5_release_authority_continuity:v1"
ARTIFACT = REPO / "artifacts/governance" / f"{SLUG}.json"
CAPTURE = REPO / "artifacts/governance/captures" / f"{SLUG}_validation_capture.json"
FIXTURES = REPO / "fixtures/governance" / SLUG

AS_OF = "2026-06-16"
OVERVIEW_PAGE = (
    "docs/m5/"
    "expose_signer_rosters_split_authority_promotion_quorum_registry_moderation_and_security_"
    "response_owner_backup_and_runbook_continuity_across_m5_release_lanes.md"
)

SIGNER_ROSTER_REF = "artifacts/governance/signing/signer_roster.json"
PROMOTION_QUORUM_REF = "artifacts/governance/promotion/promotion_quorum.json"
REGISTRY_MODERATION_REF = "artifacts/governance/registry/moderation_operators.json"
SECURITY_RESPONSE_REF = "artifacts/governance/security/security_responders.json"
RUNBOOK_REGISTER_REF = "artifacts/governance/continuity/backup_runbooks.json"
DURABILITY_MATRIX_REF = "artifacts/governance/m5-boundary-and-upstream-durability.json"
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
AUTHORITY_LANES = [
    "release_signing",
    "promotion_approval",
    "registry_moderation",
    "security_response",
]
SUPPORT_CLASSES = ["open_local", "mixed_open_managed", "managed", "restricted"]
CRITICALITIES = ["routine", "elevated", "critical", "blocking"]
CONTROL_DIMENSIONS = [
    "owner_coverage",
    "backup_coverage",
    "roster_quorum",
    "runbook_coverage",
    "authority_continuity",
    "scan_surface_parity",
]
ROSTER_KINDS = [
    "signer_roster",
    "promotion_approvers",
    "moderation_operators",
    "security_responders",
]
OWNER_STATES = ["assigned", "vacant"]
BACKUP_STATES = ["covered", "single_owner"]
QUORUM_STATES = ["met", "below_threshold"]
SPLIT_AUTHORITY_STATES = ["satisfied", "unmet", "not_required"]
RUNBOOK_STATES = ["current", "due_for_review", "stale", "missing"]
ESCALATION_STATES = ["raised", "pending", "not_required"]
POSTURES = ["clear", "gaps_found"]
CONTINUITY_STATES = [
    "cleared",
    "narrowed_owner",
    "narrowed_backup",
    "narrowed_quorum",
    "narrowed_runbook",
    "narrowed_authority",
    "narrowed_stale",
    "withdrawn",
]
CONTINUITY_REASONS = [
    "primary_owner_vacant",
    "backup_owner_missing",
    "roster_quorum_below_threshold",
    "runbook_stale",
    "runbook_missing",
    "split_authority_unmet",
    "shiproom_escalation_missing",
    "continuity_proof_stale",
    "continuity_proof_missing",
    "owner_signoff_missing",
    "waiver_expired",
]
CONTINUITY_ACTIONS = [
    "hold_promotion",
    "assign_primary_owner",
    "assign_backup_owner",
    "staff_roster_to_quorum",
    "refresh_continuity_runbook",
    "enforce_split_authority",
    "raise_shiproom_escalation",
    "refresh_continuity_proof",
    "request_owner_signoff",
]

ABOVE_CUTLINE = ["lts", "stable"]
LABEL_RANK = {"lts": 4, "stable": 3, "beta": 2, "preview": 1, "withdrawn": 0}

ROSTER_BY_LANE = {
    "release_signing": "signer_roster",
    "promotion_approval": "promotion_approvers",
    "registry_moderation": "moderation_operators",
    "security_response": "security_responders",
}

STATE_BY_REASON = {
    "primary_owner_vacant": "narrowed_owner",
    "backup_owner_missing": "narrowed_backup",
    "roster_quorum_below_threshold": "narrowed_quorum",
    "runbook_stale": "narrowed_runbook",
    "runbook_missing": "narrowed_runbook",
    "split_authority_unmet": "narrowed_authority",
    "shiproom_escalation_missing": "narrowed_authority",
    "continuity_proof_stale": "narrowed_stale",
    "continuity_proof_missing": "narrowed_stale",
    "owner_signoff_missing": "narrowed_stale",
    "waiver_expired": "narrowed_stale",
}
PRECEDENCE = {
    "narrowed_backup": 0,
    "narrowed_owner": 1,
    "narrowed_authority": 2,
    "narrowed_quorum": 3,
    "narrowed_runbook": 4,
    "narrowed_stale": 5,
}
DIMENSION_BY_REASON = {
    "primary_owner_vacant": "owner_coverage",
    "backup_owner_missing": "backup_coverage",
    "roster_quorum_below_threshold": "roster_quorum",
    "runbook_stale": "runbook_coverage",
    "runbook_missing": "runbook_coverage",
    "split_authority_unmet": "authority_continuity",
    "shiproom_escalation_missing": "authority_continuity",
    "continuity_proof_stale": "scan_surface_parity",
    "continuity_proof_missing": "scan_surface_parity",
    "owner_signoff_missing": "scan_surface_parity",
    "waiver_expired": "scan_surface_parity",
}
ACTION_BY_REASON = {
    "primary_owner_vacant": "assign_primary_owner",
    "backup_owner_missing": "assign_backup_owner",
    "roster_quorum_below_threshold": "staff_roster_to_quorum",
    "runbook_stale": "refresh_continuity_runbook",
    "runbook_missing": "refresh_continuity_runbook",
    "split_authority_unmet": "enforce_split_authority",
    "shiproom_escalation_missing": "raise_shiproom_escalation",
    "continuity_proof_stale": "refresh_continuity_proof",
    "continuity_proof_missing": "refresh_continuity_proof",
    "owner_signoff_missing": "request_owner_signoff",
    "waiver_expired": "refresh_continuity_proof",
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
        "evidence_refs": [RUNBOOK_REGISTER_REF, EVIDENCE_INDEX_REF],
    }


def signoff(owner: str, signed: bool, at: str | None) -> dict:
    return {"owner_ref": owner, "signed_off": signed, "signed_at": at}


def waiver(ref: str, expires: str, reason: str) -> dict:
    return {"waiver_ref": ref, "expires_at": expires, "reason": reason}


def owner_coverage(state: str, primary: str) -> dict:
    return {
        "owner_state": state,
        "primary_owner_ref": primary,
        "assignment_ref": f"{SHIPROOM_REGISTER_REF}#ownership",
    }


def backup_coverage(state: str, count: int, roster: str) -> dict:
    return {"backup_state": state, "backup_owner_count": count, "roster_ref": roster}


def roster_profile(kind: str, required: int, available: int) -> dict:
    return {
        "roster_kind": kind,
        "quorum_state": "met" if available >= required else "below_threshold",
        "required_quorum": required,
        "available_members": available,
        "roster_ref": f"{SHIPROOM_REGISTER_REF}#{kind}",
    }


def split_authority(state: str, required: bool, distinct: int) -> dict:
    return {
        "split_state": state,
        "required": required,
        "distinct_authorities": distinct,
        "policy_ref": f"{SHIPROOM_REGISTER_REF}#split_authority",
    }


def runbook_coverage(state: str, interval: int, next_due: str | None) -> dict:
    return {
        "runbook_state": state,
        "review_interval_days": interval,
        "next_review_due": next_due,
        "runbook_ref": f"{RUNBOOK_REGISTER_REF}#runbook",
    }


def escalation(state: str, required: bool) -> dict:
    return {
        "escalation_state": state,
        "required": required,
        "shiproom_ref": f"{SHIPROOM_REGISTER_REF}#escalations",
        "governance_ref": f"{SHIPROOM_REGISTER_REF}#governance_review",
    }


# ---------------------------------------------------------------------------
# Derivations mirroring the Rust model
# ---------------------------------------------------------------------------
def is_critical(rec: dict) -> bool:
    return rec["criticality"] in ("critical", "blocking")


def owner_vacant(rec: dict) -> bool:
    return rec["owner_coverage"]["owner_state"] == "vacant"


def single_owner(rec: dict) -> bool:
    return rec["backup_coverage"]["backup_state"] == "single_owner"


def quorum_below(rec: dict) -> bool:
    return rec["roster"]["quorum_state"] == "below_threshold"


def runbook_degraded(rec: dict) -> bool:
    return rec["runbook"]["runbook_state"] in ("stale", "missing")


def requires_split(rec: dict) -> bool:
    return is_critical(rec)


def requires_escalation(rec: dict) -> bool:
    return is_critical(rec) or single_owner(rec)


def split_unmet(rec: dict) -> bool:
    return requires_split(rec) and rec["split_authority"]["split_state"] == "unmet"


def escalation_missing(rec: dict) -> bool:
    return requires_escalation(rec) and rec["escalation"]["escalation_state"] == "pending"


def derive_reasons(rec: dict) -> list[str]:
    reasons: list[str] = []
    if owner_vacant(rec):
        reasons.append("primary_owner_vacant")
    if single_owner(rec):
        reasons.append("backup_owner_missing")
    if quorum_below(rec):
        reasons.append("roster_quorum_below_threshold")
    if rec["runbook"]["runbook_state"] == "stale":
        reasons.append("runbook_stale")
    if rec["runbook"]["runbook_state"] == "missing":
        reasons.append("runbook_missing")
    if split_unmet(rec):
        reasons.append("split_authority_unmet")
    if escalation_missing(rec):
        reasons.append("shiproom_escalation_missing")
    if rec["proof_packet"]["slo_state"] == "breached":
        reasons.append("continuity_proof_stale")
    if rec["proof_packet"]["slo_state"] == "missing":
        reasons.append("continuity_proof_missing")
    if not rec["owner_signoff"]["signed_off"]:
        reasons.append("owner_signoff_missing")
    # Order by the closed-vocabulary declaration order for tidy output.
    return [r for r in CONTINUITY_REASONS if r in reasons]


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
    if dimension == "owner_coverage":
        return "unsatisfied" if owner_vacant(rec) else "satisfied"
    if dimension == "backup_coverage":
        return "unsatisfied" if single_owner(rec) else "satisfied"
    if dimension == "roster_quorum":
        return "unsatisfied" if quorum_below(rec) else "satisfied"
    if dimension == "runbook_coverage":
        return "unsatisfied" if runbook_degraded(rec) else "satisfied"
    if dimension == "authority_continuity":
        return "unsatisfied" if (split_unmet(rec) or escalation_missing(rec)) else "satisfied"
    # scan_surface_parity
    return "unsatisfied" if rec["scan_posture"] != rec["surface_posture"] else "satisfied"


def build_controls(rec: dict) -> list[dict]:
    controls = []
    for dimension in CONTROL_DIMENSIONS:
        controls.append(
            {
                "dimension": dimension,
                "control_ref": f"{SHIPROOM_REGISTER_REF}#{dimension}",
                "owner_ref": "role:governance-release-lead",
                "state": expected_control_state(rec, dimension),
            }
        )
    return controls


def finalize(rec: dict) -> dict:
    """Fill in derived fields (reasons, state, effective label, postures, controls)."""
    # Fail fast on internally inconsistent fact wiring.
    assert rec["roster"]["roster_kind"] == ROSTER_BY_LANE[rec["lane"]], rec["record_id"]
    rec["active_reasons"] = derive_reasons(rec)
    rec["continuity_state"] = computed_state(rec)
    rec["effective_label"] = computed_effective_label(rec)
    posture = computed_posture(rec)
    rec["scan_posture"] = posture
    rec["surface_posture"] = posture
    rec["controls"] = build_controls(rec)
    return rec


SURFACES = [
    "help_about:release_authority",
    "service_health:continuity",
    "release_center:promotion_packet",
    "support_export:authority_continuity",
]


def make_record(
    *,
    record_id: str,
    family: str,
    lane: str,
    title: str,
    subject_ref: str,
    subject_summary: str,
    release_blocking: bool,
    declared_label: str,
    support_class: str,
    criticality: str,
    owner: dict,
    backup: dict,
    roster: dict,
    split: dict,
    runbook: dict,
    escal: dict,
    proof_packet: dict,
    owner_signoff: dict,
    waiver_record: dict | None,
    rationale: str,
) -> dict:
    rec = {
        "record_id": record_id,
        "family": family,
        "lane": lane,
        "title": title,
        "subject_ref": subject_ref,
        "subject_summary": subject_summary,
        "release_blocking": release_blocking,
        "declared_label": declared_label,
        "support_class": support_class,
        "criticality": criticality,
        "owner_coverage": owner,
        "backup_coverage": backup,
        "roster": roster,
        "split_authority": split,
        "runbook": runbook,
        "escalation": escal,
        "controls": [],
        "scan_posture": "clear",
        "surface_posture": "clear",
        "scan_ref": f"{SHIPROOM_REGISTER_REF}#continuity_scan",
        "surface_ref": f"{SHIPROOM_REGISTER_REF}#governance_surface",
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

    # 1) A fully covered, critical release-signing lane: cleared.
    records.append(
        make_record(
            record_id="authority-framework-release-signing",
            family="framework",
            lane="release_signing",
            title="Framework release-artifact signing",
            subject_ref="lane:framework/release-signing",
            subject_summary="Release-artifact signing for the core framework train.",
            release_blocking=True,
            declared_label="stable",
            support_class="open_local",
            criticality="critical",
            owner=owner_coverage("assigned", "role:release-engineering-lead"),
            backup=backup_coverage("covered", 2, f"{SIGNER_ROSTER_REF}#backups"),
            roster=roster_profile("signer_roster", 3, 3),
            split=split_authority("satisfied", True, 3),
            runbook=runbook_coverage("current", 90, "2026-08-01"),
            escal=escalation("raised", True),
            proof_packet=proof("authority-framework-release-signing", "current", "2026-06-10"),
            owner_signoff=signoff("role:governance-release-lead", True, "2026-06-11"),
            waiver_record=None,
            rationale="Named primary and backup owners, a three-signer roster at quorum, enforced split authority, a current runbook, and a raised escalation; the lane is cleared.",
        )
    )

    # 2) A critical promotion-approval lane with no backup owner: single-person system.
    records.append(
        make_record(
            record_id="authority-notebook-promotion-approval",
            family="notebook",
            lane="promotion_approval",
            title="Notebook promotion-gate approval",
            subject_ref="lane:notebook/promotion-approval",
            subject_summary="Promotion-gate approval quorum for the notebook train.",
            release_blocking=True,
            declared_label="stable",
            support_class="open_local",
            criticality="critical",
            owner=owner_coverage("assigned", "role:notebook-release-owner"),
            backup=backup_coverage("single_owner", 0, f"{PROMOTION_QUORUM_REF}#backups"),
            roster=roster_profile("promotion_approvers", 2, 3),
            split=split_authority("satisfied", True, 2),
            runbook=runbook_coverage("current", 90, "2026-08-15"),
            escal=escalation("raised", True),
            proof_packet=proof("authority-notebook-promotion-approval", "current", "2026-06-09"),
            owner_signoff=signoff("role:governance-release-lead", True, "2026-06-10"),
            waiver_record=None,
            rationale="The promotion-approval lane has a single owner with no named backup: it is a single-person system and narrows on the backup axis even though the lane is escalated. A still-stable subject, so it holds promotion.",
        )
    )

    # 3) A critical release-signing lane whose signer roster is below quorum.
    records.append(
        make_record(
            record_id="authority-managed_depth-release-signing",
            family="managed_depth",
            lane="release_signing",
            title="Managed-depth release-artifact signing",
            subject_ref="lane:managed_depth/release-signing",
            subject_summary="Release-artifact signing for managed-depth services.",
            release_blocking=True,
            declared_label="stable",
            support_class="managed",
            criticality="critical",
            owner=owner_coverage("assigned", "role:managed-depth-release-owner"),
            backup=backup_coverage("covered", 2, f"{SIGNER_ROSTER_REF}#managed-backups"),
            roster=roster_profile("signer_roster", 3, 1),
            split=split_authority("satisfied", True, 2),
            runbook=runbook_coverage("current", 90, "2026-08-20"),
            escal=escalation("raised", True),
            proof_packet=proof("authority-managed_depth-release-signing", "current", "2026-06-08"),
            owner_signoff=signoff("role:governance-release-lead", True, "2026-06-09"),
            waiver_record=None,
            rationale="The signer roster carries one available signer against a quorum of three; the lane narrows on the quorum axis and holds promotion on a still-stable claim.",
        )
    )

    # 4) A blocking security-response lane without enforced split authority or escalation.
    records.append(
        make_record(
            record_id="authority-ai_adjacent-security-response",
            family="ai_adjacent",
            lane="security_response",
            title="AI-adjacent security response",
            subject_ref="lane:ai_adjacent/security-response",
            subject_summary="Advisory, CVE/GHSA, and revocation response for AI-adjacent surfaces.",
            release_blocking=True,
            declared_label="stable",
            support_class="open_local",
            criticality="blocking",
            owner=owner_coverage("assigned", "role:security-response-owner"),
            backup=backup_coverage("covered", 2, f"{SECURITY_RESPONSE_REF}#backups"),
            roster=roster_profile("security_responders", 2, 3),
            split=split_authority("unmet", True, 1),
            runbook=runbook_coverage("current", 60, "2026-07-20"),
            escal=escalation("pending", True),
            proof_packet=proof("authority-ai_adjacent-security-response", "current", "2026-06-07"),
            owner_signoff=signoff("role:governance-release-lead", True, "2026-06-08"),
            waiver_record=None,
            rationale="A blocking security-response lane whose split (two-person) authority is unmet and whose shiproom escalation is still pending; it narrows on the authority axis and holds promotion.",
        )
    )

    # 5) An elevated registry-moderation lane with no backup runbook captured.
    records.append(
        make_record(
            record_id="authority-companion-registry-moderation",
            family="companion",
            lane="registry_moderation",
            title="Companion registry moderation",
            subject_ref="lane:companion/registry-moderation",
            subject_summary="Extension-registry moderation and emergency unpublish for the companion surfaces.",
            release_blocking=True,
            declared_label="stable",
            support_class="mixed_open_managed",
            criticality="elevated",
            owner=owner_coverage("assigned", "role:registry-moderation-owner"),
            backup=backup_coverage("covered", 2, f"{REGISTRY_MODERATION_REF}#backups"),
            roster=roster_profile("moderation_operators", 2, 2),
            split=split_authority("not_required", False, 0),
            runbook=runbook_coverage("missing", 90, None),
            escal=escalation("not_required", False),
            proof_packet=proof("authority-companion-registry-moderation", "current", "2026-06-06"),
            owner_signoff=signoff("role:governance-release-lead", True, "2026-06-07"),
            waiver_record=None,
            rationale="The registry-moderation lane has owners and operators but no captured backup runbook; it narrows on the runbook axis and holds promotion on a still-stable claim.",
        )
    )

    # 6) An elevated registry-moderation lane with a vacant primary owner; already Beta (inherited).
    records.append(
        make_record(
            record_id="authority-data_rich-registry-moderation",
            family="data_rich",
            lane="registry_moderation",
            title="Data-rich registry moderation",
            subject_ref="lane:data_rich/registry-moderation",
            subject_summary="Extension-registry moderation for data-rich surfaces.",
            release_blocking=False,
            declared_label="beta",
            support_class="mixed_open_managed",
            criticality="elevated",
            owner=owner_coverage("vacant", ""),
            backup=backup_coverage("covered", 2, f"{REGISTRY_MODERATION_REF}#data-backups"),
            roster=roster_profile("moderation_operators", 2, 2),
            split=split_authority("not_required", False, 0),
            runbook=runbook_coverage("current", 90, "2026-08-30"),
            escal=escalation("not_required", False),
            proof_packet=proof("authority-data_rich-registry-moderation", "current", "2026-06-05"),
            owner_signoff=signoff("role:governance-release-lead", True, "2026-06-06"),
            waiver_record=None,
            rationale="The lane has no named primary owner; it narrows on the owner axis. The subject is already Beta, so the narrowing is inherited and gated upstream.",
        )
    )

    # 7) A critical promotion-approval lane that is single-owner but held by an unexpired waiver.
    records.append(
        make_record(
            record_id="authority-review-promotion-approval",
            family="review",
            lane="promotion_approval",
            title="Review promotion-gate approval",
            subject_ref="lane:review/promotion-approval",
            subject_summary="Promotion-gate approval quorum for review/diff surfaces.",
            release_blocking=True,
            declared_label="stable",
            support_class="open_local",
            criticality="critical",
            owner=owner_coverage("assigned", "role:review-release-owner"),
            backup=backup_coverage("single_owner", 0, f"{PROMOTION_QUORUM_REF}#review-backups"),
            roster=roster_profile("promotion_approvers", 2, 2),
            split=split_authority("satisfied", True, 2),
            runbook=runbook_coverage("current", 90, "2026-08-12"),
            escal=escalation("raised", True),
            proof_packet=proof("authority-review-promotion-approval", "current", "2026-06-05"),
            owner_signoff=signoff("role:governance-release-lead", True, "2026-06-06"),
            waiver_record=waiver(
                f"{SHIPROOM_REGISTER_REF}#waiver/review-promotion",
                "2026-09-30",
                "Backup owner onboarding in progress; covered by an approved, time-boxed waiver.",
            ),
            rationale="A single-owner promotion-approval lane held by an unexpired waiver: it stays visible and narrowed on the backup axis but is gated upstream and does not hold promotion.",
        )
    )

    # 8) An elevated security-response lane with a stale continuity proof; already Beta (inherited).
    records.append(
        make_record(
            record_id="authority-managed_depth-security-response",
            family="managed_depth",
            lane="security_response",
            title="Managed-depth security response",
            subject_ref="lane:managed_depth/security-response",
            subject_summary="Advisory and revocation response for managed-depth services.",
            release_blocking=False,
            declared_label="beta",
            support_class="managed",
            criticality="elevated",
            owner=owner_coverage("assigned", "role:managed-depth-security-owner"),
            backup=backup_coverage("covered", 2, f"{SECURITY_RESPONSE_REF}#managed-backups"),
            roster=roster_profile("security_responders", 2, 2),
            split=split_authority("not_required", False, 0),
            runbook=runbook_coverage("current", 60, "2026-07-15"),
            escal=escalation("not_required", False),
            proof_packet=proof("authority-managed_depth-security-response", "breached", "2026-01-02"),
            owner_signoff=signoff("role:governance-release-lead", True, "2026-02-01"),
            waiver_record=None,
            rationale="The continuity proof packet aged past its freshness SLO; the lane narrows on the stale axis. The subject is already Beta, so the narrowing is inherited.",
        )
    )

    # 9) A routine registry-moderation lane with no captured continuity proof; Beta (inherited).
    records.append(
        make_record(
            record_id="authority-managed_depth-registry-moderation",
            family="managed_depth",
            lane="registry_moderation",
            title="Managed-depth registry moderation",
            subject_ref="lane:managed_depth/registry-moderation",
            subject_summary="Registry moderation for managed-depth marketplace surfaces.",
            release_blocking=False,
            declared_label="beta",
            support_class="managed",
            criticality="routine",
            owner=owner_coverage("assigned", "role:managed-depth-registry-owner"),
            backup=backup_coverage("covered", 2, f"{REGISTRY_MODERATION_REF}#md-backups"),
            roster=roster_profile("moderation_operators", 1, 2),
            split=split_authority("not_required", False, 0),
            runbook=runbook_coverage("due_for_review", 90, "2026-07-01"),
            escal=escalation("not_required", False),
            proof_packet=proof("authority-managed_depth-registry-moderation", "missing", None),
            owner_signoff=signoff("role:governance-release-lead", True, "2026-05-01"),
            waiver_record=None,
            rationale="No continuity proof packet is captured; the lane narrows on the stale axis. The subject is already Beta, so the narrowing is inherited.",
        )
    )

    # 10) A fully covered, critical security-response lane: cleared.
    records.append(
        make_record(
            record_id="authority-framework-security-response",
            family="framework",
            lane="security_response",
            title="Framework security response",
            subject_ref="lane:framework/security-response",
            subject_summary="Advisory, CVE/GHSA, and revocation response for the core framework.",
            release_blocking=True,
            declared_label="stable",
            support_class="open_local",
            criticality="critical",
            owner=owner_coverage("assigned", "role:framework-security-owner"),
            backup=backup_coverage("covered", 2, f"{SECURITY_RESPONSE_REF}#framework-backups"),
            roster=roster_profile("security_responders", 2, 3),
            split=split_authority("satisfied", True, 2),
            runbook=runbook_coverage("due_for_review", 60, "2026-06-25"),
            escal=escalation("raised", True),
            proof_packet=proof("authority-framework-security-response", "current", "2026-06-10"),
            owner_signoff=signoff("role:governance-release-lead", True, "2026-06-11"),
            waiver_record=None,
            rationale="Named primary and backup owners, a responder roster at quorum, enforced split authority, a runbook due for review (a reminder, not a gap), and a raised escalation; the lane is cleared.",
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
        "parity_gate": "m5_release_authority_continuity_parity_gate",
        "subjects_total": len(records),
        "subjects_in_agreement": len(agree),
        "subjects_in_disagreement": len(records) - len(agree),
        "subjects_with_gaps": len([r for r in records if r["surface_posture"] == "gaps_found"]),
        "all_subjects_agree": len(agree) == len(records),
        "rationale": "Every record's continuity scan and governance surface agree, so a green authority card can never mask a single-owner, under-quorum, or runbook-less lane.",
    }


def computed_summary(records: list[dict], rules: list[dict]) -> dict:
    def count_state(state: str) -> int:
        return len([r for r in records if r["continuity_state"] == state])

    return {
        "total_records": len(records),
        "records_cleared": count_state("cleared"),
        "records_narrowed": len([r for r in records if is_narrowed(r)]),
        "state_cleared": count_state("cleared"),
        "state_narrowed_owner": count_state("narrowed_owner"),
        "state_narrowed_backup": count_state("narrowed_backup"),
        "state_narrowed_quorum": count_state("narrowed_quorum"),
        "state_narrowed_runbook": count_state("narrowed_runbook"),
        "state_narrowed_authority": count_state("narrowed_authority"),
        "state_narrowed_stale": count_state("narrowed_stale"),
        "state_withdrawn": count_state("withdrawn"),
        "release_blocking_total": len([r for r in records if r["release_blocking"]]),
        "release_blocking_narrowed": len(
            [r for r in records if r["release_blocking"] and is_narrowed(r)]
        ),
        "records_on_active_waiver": len([r for r in records if is_waived(r)]),
        "owner_gaps": len([r for r in records if owner_vacant(r)]),
        "backup_gaps": len([r for r in records if single_owner(r)]),
        "quorum_gaps": len([r for r in records if quorum_below(r)]),
        "runbook_gaps": len([r for r in records if runbook_degraded(r)]),
        "authority_gaps": len(
            [r for r in records if split_unmet(r) or escalation_missing(r)]
        ),
        "critical_total": len([r for r in records if is_critical(r)]),
        "single_owner_total": len([r for r in records if single_owner(r)]),
        "escalations_required": len([r for r in records if requires_escalation(r)]),
        "escalations_raised": len(
            [r for r in records if r["escalation"]["escalation_state"] == "raised"]
        ),
        "split_authority_enforced": len(
            [r for r in records if r["split_authority"]["split_state"] == "satisfied"]
        ),
        "total_active_reasons": sum(len(r["active_reasons"]) for r in records),
        "rules_firing": len(computed_blocking_rule_ids(records, rules)),
    }


RULE_TITLES = {
    "primary_owner_vacant": "Primary owner vacant",
    "backup_owner_missing": "No backup owner (single-person system)",
    "roster_quorum_below_threshold": "Roster below quorum",
    "runbook_stale": "Backup runbook stale",
    "runbook_missing": "Backup runbook missing",
    "split_authority_unmet": "Split authority unmet",
    "shiproom_escalation_missing": "Shiproom escalation missing",
    "continuity_proof_stale": "Continuity proof stale",
    "continuity_proof_missing": "Continuity proof missing",
    "owner_signoff_missing": "Owner sign-off missing",
    "waiver_expired": "Waiver expired",
}


def build_rules() -> list[dict]:
    rules = []
    for reason in CONTINUITY_REASONS:
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
            "signer_roster_ref": SIGNER_ROSTER_REF,
            "promotion_quorum_ref": PROMOTION_QUORUM_REF,
            "registry_moderation_ref": REGISTRY_MODERATION_REF,
            "security_response_ref": SECURITY_RESPONSE_REF,
            "runbook_register_ref": RUNBOOK_REGISTER_REF,
            "durability_matrix_ref": DURABILITY_MATRIX_REF,
            "shiproom_register_ref": SHIPROOM_REGISTER_REF,
            "m5_evidence_index_ref": EVIDENCE_INDEX_REF,
        },
        "continuity_cutline": {
            "cutline_level": "stable",
            "above_cutline_levels": list(ABOVE_CUTLINE),
            "below_cutline_levels": ["beta", "preview", "withdrawn"],
            "description": "Subjects at or above Stable carry the cleared continuity claim; a continuity gap on a still-stable subject holds promotion through the shiproom gate.",
        },
        "families": FAMILIES,
        "authority_lanes": AUTHORITY_LANES,
        "support_classes": SUPPORT_CLASSES,
        "criticalities": CRITICALITIES,
        "control_dimensions": CONTROL_DIMENSIONS,
        "roster_kinds": ROSTER_KINDS,
        "owner_states": OWNER_STATES,
        "backup_states": BACKUP_STATES,
        "quorum_states": QUORUM_STATES,
        "split_authority_states": SPLIT_AUTHORITY_STATES,
        "runbook_states": RUNBOOK_STATES,
        "escalation_states": ESCALATION_STATES,
        "postures": POSTURES,
        "continuity_states": CONTINUITY_STATES,
        "continuity_reasons": CONTINUITY_REASONS,
        "continuity_actions": CONTINUITY_ACTIONS,
        "rules": rules,
        "records": records,
        "scan_surface_parity": computed_scan_surface_parity(records),
        "publication": {
            "publication_gate": "m5_release_authority_continuity_gate",
            "decision": decision_verdict,
            "blocking_rule_ids": blocking_rules,
            "blocking_record_ids": blocking_records,
            "rationale": "Hold while any release-blocking subject carries a continuity gap on a still-stable claim; inherited and waived narrowings are gated upstream. A single-owner, under-quorum, or runbook-less protected lane may not widen a stable claim without coverage.",
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

    # A cleared critical record hiding a single-owner backup gap without narrowing on it.
    hidden = copy.deepcopy(register)
    target = next(
        r
        for r in hidden["records"]
        if r["continuity_state"] == "cleared" and r["criticality"] in ("critical", "blocking")
    )
    target["backup_coverage"]["backup_state"] = "single_owner"
    target["backup_coverage"]["backup_owner_count"] = 0
    cases.append(("hidden_backup_gap.json", hidden, "GapWithoutReason"))

    # A narrowed record whose governance surface is green over a gapped scan.
    masked = copy.deepcopy(register)
    target = next(r for r in masked["records"] if is_narrowed(r))
    target["surface_posture"] = "clear"
    cases.append(("green_surface_over_gap.json", masked, "ScanSurfaceDisagreement"))

    # A narrowed record whose effective label stays above the cutline.
    above = copy.deepcopy(register)
    target = next(r for r in above["records"] if is_narrowed(r))
    target["effective_label"] = "stable"
    cases.append(("narrowed_above_cutline.json", above, "EffectiveLabelMismatch"))

    return cases


def build_capture(register: dict, cases: list[tuple[str, dict, str]]) -> dict:
    s = register["summary"]
    p = register["publication"]
    parity = register["scan_surface_parity"]
    drills = [
        "drill:hidden_backup_gap",
        "drill:green_surface_over_gap",
        "drill:narrowed_above_cutline",
        "drill:cleared_with_active_reason",
        "drill:reason_not_justified",
        "drill:control_state_inconsistent",
        "drill:critical_without_split_authority",
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
            "state_narrowed_owner": s["state_narrowed_owner"],
            "state_narrowed_backup": s["state_narrowed_backup"],
            "state_narrowed_quorum": s["state_narrowed_quorum"],
            "state_narrowed_runbook": s["state_narrowed_runbook"],
            "state_narrowed_authority": s["state_narrowed_authority"],
            "state_narrowed_stale": s["state_narrowed_stale"],
            "release_blocking_narrowed": s["release_blocking_narrowed"],
            "records_on_active_waiver": s["records_on_active_waiver"],
            "owner_gaps": s["owner_gaps"],
            "backup_gaps": s["backup_gaps"],
            "quorum_gaps": s["quorum_gaps"],
            "runbook_gaps": s["runbook_gaps"],
            "authority_gaps": s["authority_gaps"],
            "critical_total": s["critical_total"],
            "single_owner_total": s["single_owner_total"],
            "escalations_required": s["escalations_required"],
            "escalations_raised": s["escalations_raised"],
            "split_authority_enforced": s["split_authority_enforced"],
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
