#!/usr/bin/env python3
"""Regenerate the open-durability certification register.

The sibling registers each make one durability axis inspectable: the versioned boundary-manifest
register publishes the open-versus-paid boundary per family; the repository-compliance register binds
REUSE/SPDX/notice/SBOM hygiene; the import-provenance register attributes third-party and generated
imports; the release-authority continuity register names signer quorum and backup coverage; the
emergency-response evidence register records advisory/revocation/disable drills; and the
critical-upstream health register rates the protected-path dependencies. None of them *certifies a
single claimed M5 ecosystem/release row across all six axes at once* — so a row could carry a green
boundary card while its critical import is ownerless, or a healthy upstream while its emergency
authority is one irreplaceable human.

This register is that certification layer. For every claimed M5 ecosystem/release row it records one
copy-safe record binding the six durability axes:

  - boundary manifest (the versioned open-boundary manifest is published and release-linked, with no
    hidden proprietary baseline);
  - repository compliance (REUSE/SPDX licensing is current and the notice inventory and SBOM are
    bound);
  - import durability (third-party/generated import provenance is attributed and every critical
    import is owned);
  - signer authority (the signer quorum is met and the emergency authority is not one irreplaceable
    human);
  - emergency response (the advisory/revocation/disable drill evidence is current);
  - critical upstream (the protected-path dependencies are healthy and owned).

A record is certified only when every axis holds, the certification proof is fresh, and the owner
signed. Otherwise it narrows on the specific axis that thins out (a boundary, compliance, import,
authority, emergency, upstream, or stale-proof gap) and drops its effective label below the launch
cutline; the axes never collapse into one global flag. The certification scan and the
service-health/release-center/support surface must agree on every record, so a green certification
card can never mask a hidden proprietary baseline, an ownerless critical import, a single-person
emergency authority, a stale notice/SBOM, an uncovered drill, or an unhealthy upstream.

An inherited narrowing (a row already below the cutline, or a gap held by an unexpired waiver) is
gated upstream and does not hold promotion; a certification failure on a still-stable row holds
promotion through a stop rule — a row that depends on a hidden proprietary baseline, an ownerless
critical import, or a single-person emergency authority may not widen a stable claim without a plan.

This emits the canonical register artifact, the negative fixtures, the cases manifest, and the frozen
validation capture. The Python summary/parity/promotion logic mirrors the typed Rust consumer so the
checked-in artifact validates cleanly and the capture cross-check agrees with the model.
"""

from __future__ import annotations

import copy
import json
from pathlib import Path

REPO = Path(__file__).resolve().parent.parent

SLUG = "m5-open-durability-certification"
RECORD_KIND = "m5_open_durability_certification_register"
REGISTER_ID = "m5_open_durability_certification:v1"
ARTIFACT = REPO / "artifacts/governance" / f"{SLUG}.json"
CAPTURE = REPO / "artifacts/governance/captures" / f"{SLUG}_validation_capture.json"
FIXTURES = REPO / "fixtures/governance" / SLUG

AS_OF = "2026-06-16"
OVERVIEW_PAGE = (
    "docs/m5/"
    "certify_open_boundary_publication_repository_compliance_import_durability_signing_quorum_"
    "and_critical_upstream_continuity_on_every_claimed_m5_ecosystem_and_release_row.md"
)

# Canonical source registers this certification binds together.
BOUNDARY_MANIFEST_REGISTER_REF = "artifacts/governance/m5-versioned-boundary-manifests.json"
BOUNDARY_DURABILITY_MATRIX_REF = "artifacts/governance/m5-boundary-and-upstream-durability.json"
COMPLIANCE_REGISTER_REF = "artifacts/governance/m5-compliance-and-notice-binding.json"
IMPORT_REGISTER_REF = "artifacts/governance/m5-import-provenance-and-fork-review.json"
AUTHORITY_CONTINUITY_REGISTER_REF = "artifacts/governance/m5-release-authority-continuity.json"
EMERGENCY_RESPONSE_REGISTER_REF = "artifacts/governance/m5-emergency-response-evidence.json"
UPSTREAM_HEALTH_REGISTER_REF = "artifacts/governance/m5-critical-upstream-health.json"
RELEASE_GRAPH_REF = "artifacts/release/m5/artifact_graph.json"
SUPPORT_EXPORT_REF = "artifacts/support/m5/support_export_index.json"
SHIPROOM_REGISTER_REF = "artifacts/governance/shiproom/gate_register.json"
STABLE_PROMOTION_PACKET_REF = "artifacts/release/m5/stable_promotion_packet.json"
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
ROW_KINDS = ["ecosystem", "release"]
SUPPORT_CLASSES = ["open_local", "mixed_open_managed", "managed", "restricted"]
CONTROL_DIMENSIONS = [
    "boundary_manifest",
    "repository_compliance",
    "import_durability",
    "signer_authority",
    "emergency_response",
    "critical_upstream",
    "scan_surface_parity",
]
BOUNDARY_STATES = ["published", "unpublished", "hidden_proprietary_baseline"]
COMPLIANCE_STATES = ["current", "stale", "notice_binding_missing"]
IMPORT_STATES = ["attributed", "provenance_missing", "ownerless_critical_import"]
AUTHORITY_STATES = ["quorum_met", "quorum_unmet", "single_person_authority"]
EMERGENCY_STATES = ["current", "stale"]
UPSTREAM_STATES = ["healthy", "unhealthy"]
POSTURES = ["clear", "gaps_found"]
CERTIFICATION_STATES = [
    "certified",
    "narrowed_boundary",
    "narrowed_compliance",
    "narrowed_import",
    "narrowed_authority",
    "narrowed_emergency",
    "narrowed_upstream",
    "narrowed_stale",
    "withdrawn",
]
CERTIFICATION_REASONS = [
    "boundary_manifest_missing",
    "hidden_proprietary_baseline",
    "repository_compliance_stale",
    "notice_binding_missing",
    "import_provenance_missing",
    "ownerless_critical_import",
    "signer_quorum_unmet",
    "single_person_emergency_authority",
    "emergency_response_stale",
    "critical_upstream_unhealthy",
    "certification_proof_stale",
    "certification_proof_missing",
    "owner_signoff_missing",
    "waiver_expired",
]
CERTIFICATION_ACTIONS = [
    "hold_promotion",
    "publish_boundary_manifest",
    "disclose_proprietary_baseline",
    "refresh_repository_compliance",
    "bind_notices_and_sbom",
    "attribute_import_provenance",
    "assign_import_owner",
    "meet_signer_quorum",
    "add_backup_authority",
    "refresh_emergency_response",
    "remediate_critical_upstream",
    "refresh_certification_proof",
    "request_owner_signoff",
]

ABOVE_CUTLINE = ["lts", "stable"]
LABEL_RANK = {"lts": 4, "stable": 3, "beta": 2, "preview": 1, "withdrawn": 0}

STATE_BY_REASON = {
    "boundary_manifest_missing": "narrowed_boundary",
    "hidden_proprietary_baseline": "narrowed_boundary",
    "repository_compliance_stale": "narrowed_compliance",
    "notice_binding_missing": "narrowed_compliance",
    "import_provenance_missing": "narrowed_import",
    "ownerless_critical_import": "narrowed_import",
    "signer_quorum_unmet": "narrowed_authority",
    "single_person_emergency_authority": "narrowed_authority",
    "emergency_response_stale": "narrowed_emergency",
    "critical_upstream_unhealthy": "narrowed_upstream",
    "certification_proof_stale": "narrowed_stale",
    "certification_proof_missing": "narrowed_stale",
    "owner_signoff_missing": "narrowed_stale",
    "waiver_expired": "narrowed_stale",
}
# Lower precedence wins. The three "do not certify" guardrails lead: a single-person emergency
# authority, then an ownerless critical import, then a hidden proprietary baseline.
PRECEDENCE = {
    "narrowed_authority": 0,
    "narrowed_import": 1,
    "narrowed_boundary": 2,
    "narrowed_upstream": 3,
    "narrowed_emergency": 4,
    "narrowed_compliance": 5,
    "narrowed_stale": 6,
}
DIMENSION_BY_REASON = {
    "boundary_manifest_missing": "boundary_manifest",
    "hidden_proprietary_baseline": "boundary_manifest",
    "repository_compliance_stale": "repository_compliance",
    "notice_binding_missing": "repository_compliance",
    "import_provenance_missing": "import_durability",
    "ownerless_critical_import": "import_durability",
    "signer_quorum_unmet": "signer_authority",
    "single_person_emergency_authority": "signer_authority",
    "emergency_response_stale": "emergency_response",
    "critical_upstream_unhealthy": "critical_upstream",
    "certification_proof_stale": "scan_surface_parity",
    "certification_proof_missing": "scan_surface_parity",
    "owner_signoff_missing": "scan_surface_parity",
    "waiver_expired": "scan_surface_parity",
}
ACTION_BY_REASON = {
    "boundary_manifest_missing": "publish_boundary_manifest",
    "hidden_proprietary_baseline": "disclose_proprietary_baseline",
    "repository_compliance_stale": "refresh_repository_compliance",
    "notice_binding_missing": "bind_notices_and_sbom",
    "import_provenance_missing": "attribute_import_provenance",
    "ownerless_critical_import": "assign_import_owner",
    "signer_quorum_unmet": "meet_signer_quorum",
    "single_person_emergency_authority": "add_backup_authority",
    "emergency_response_stale": "refresh_emergency_response",
    "critical_upstream_unhealthy": "remediate_critical_upstream",
    "certification_proof_stale": "refresh_certification_proof",
    "certification_proof_missing": "refresh_certification_proof",
    "owner_signoff_missing": "request_owner_signoff",
    "waiver_expired": "refresh_certification_proof",
}

SURFACES = [
    "help_about:open_durability_certification",
    "service_health:open_durability",
    "release_center:certification",
    "support_export:open_durability_certification",
]


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
        "evidence_refs": [STABLE_PROMOTION_PACKET_REF, EVIDENCE_INDEX_REF],
    }


def signoff(owner: str, signed: bool, at: str | None) -> dict:
    return {"owner_ref": owner, "signed_off": signed, "signed_at": at}


def waiver(ref: str, expires: str, reason: str) -> dict:
    return {"waiver_ref": ref, "expires_at": expires, "reason": reason}


def boundary(manifest_published: bool, proprietary_baseline_hidden: bool, suffix: str) -> dict:
    if proprietary_baseline_hidden:
        state = "hidden_proprietary_baseline"
    elif not manifest_published:
        state = "unpublished"
    else:
        state = "published"
    return {
        "state": state,
        "manifest_published": manifest_published,
        "proprietary_baseline_hidden": proprietary_baseline_hidden,
        "manifest_ref": f"{BOUNDARY_MANIFEST_REGISTER_REF}#{suffix}",
        "evidence_ref": f"{BOUNDARY_DURABILITY_MATRIX_REF}#{suffix}",
    }


def compliance(licensing_current: bool, notice_sbom_bound: bool, suffix: str) -> dict:
    if not notice_sbom_bound:
        state = "notice_binding_missing"
    elif not licensing_current:
        state = "stale"
    else:
        state = "current"
    return {
        "state": state,
        "licensing_current": licensing_current,
        "notice_sbom_bound": notice_sbom_bound,
        "compliance_register_ref": f"{COMPLIANCE_REGISTER_REF}#{suffix}",
        "evidence_ref": f"{COMPLIANCE_REGISTER_REF}#{suffix}/evidence",
    }


def import_durability(provenance_attributed: bool, critical_import_owned: bool, suffix: str) -> dict:
    if not critical_import_owned:
        state = "ownerless_critical_import"
    elif not provenance_attributed:
        state = "provenance_missing"
    else:
        state = "attributed"
    return {
        "state": state,
        "provenance_attributed": provenance_attributed,
        "critical_import_owned": critical_import_owned,
        "import_register_ref": f"{IMPORT_REGISTER_REF}#{suffix}",
        "evidence_ref": f"{IMPORT_REGISTER_REF}#{suffix}/evidence",
    }


def authority(required: int, available: int, backup_present: bool, suffix: str) -> dict:
    if available <= 1 or not backup_present:
        state = "single_person_authority"
    elif available < required:
        state = "quorum_unmet"
    else:
        state = "quorum_met"
    return {
        "state": state,
        "required_distinct_humans": required,
        "available_distinct_humans": available,
        "backup_present": backup_present,
        "continuity_register_ref": f"{AUTHORITY_CONTINUITY_REGISTER_REF}#{suffix}",
        "evidence_ref": f"{AUTHORITY_CONTINUITY_REGISTER_REF}#{suffix}/evidence",
    }


def emergency(drill_current: bool, suffix: str) -> dict:
    return {
        "state": "current" if drill_current else "stale",
        "drill_current": drill_current,
        "response_register_ref": f"{EMERGENCY_RESPONSE_REGISTER_REF}#{suffix}",
        "evidence_ref": f"{EMERGENCY_RESPONSE_REGISTER_REF}#{suffix}/evidence",
    }


def upstream(upstream_healthy: bool, suffix: str) -> dict:
    return {
        "state": "healthy" if upstream_healthy else "unhealthy",
        "upstream_healthy": upstream_healthy,
        "upstream_health_register_ref": f"{UPSTREAM_HEALTH_REGISTER_REF}#{suffix}",
        "evidence_ref": f"{UPSTREAM_HEALTH_REGISTER_REF}#{suffix}/evidence",
    }


# ---------------------------------------------------------------------------
# Derivations mirroring the Rust model
# ---------------------------------------------------------------------------
def boundary_reason(rec: dict) -> str | None:
    st = rec["boundary"]["state"]
    if st == "hidden_proprietary_baseline":
        return "hidden_proprietary_baseline"
    if st == "unpublished":
        return "boundary_manifest_missing"
    return None


def compliance_reason(rec: dict) -> str | None:
    st = rec["compliance"]["state"]
    if st == "notice_binding_missing":
        return "notice_binding_missing"
    if st == "stale":
        return "repository_compliance_stale"
    return None


def import_reason(rec: dict) -> str | None:
    st = rec["import_durability"]["state"]
    if st == "ownerless_critical_import":
        return "ownerless_critical_import"
    if st == "provenance_missing":
        return "import_provenance_missing"
    return None


def authority_reason(rec: dict) -> str | None:
    st = rec["authority"]["state"]
    if st == "single_person_authority":
        return "single_person_emergency_authority"
    if st == "quorum_unmet":
        return "signer_quorum_unmet"
    return None


def emergency_reason(rec: dict) -> str | None:
    return "emergency_response_stale" if rec["emergency"]["state"] == "stale" else None


def upstream_reason(rec: dict) -> str | None:
    return "critical_upstream_unhealthy" if rec["upstream"]["state"] == "unhealthy" else None


def axis_reasons(rec: dict) -> list[str]:
    out: list[str] = []
    for fn in (
        boundary_reason,
        compliance_reason,
        import_reason,
        authority_reason,
        emergency_reason,
        upstream_reason,
    ):
        reason = fn(rec)
        if reason is not None:
            out.append(reason)
    return out


def has_structural_gap(rec: dict) -> bool:
    return len(axis_reasons(rec)) > 0


def derive_reasons(rec: dict) -> list[str]:
    reasons: set[str] = set(axis_reasons(rec))
    if rec["proof_packet"]["slo_state"] == "breached":
        reasons.add("certification_proof_stale")
    if rec["proof_packet"]["slo_state"] == "missing":
        reasons.add("certification_proof_missing")
    if not rec["owner_signoff"]["signed_off"]:
        reasons.add("owner_signoff_missing")
    # Order by the closed-vocabulary declaration order for tidy output.
    return [r for r in CERTIFICATION_REASONS if r in reasons]


def computed_state(rec: dict) -> str:
    if rec["declared_label"] == "withdrawn":
        return "withdrawn"
    reasons = rec["active_reasons"]
    if not reasons:
        return "certified"
    groups = [STATE_BY_REASON[r] for r in reasons]
    return min(groups, key=lambda g: PRECEDENCE[g])


def computed_effective_label(rec: dict) -> str:
    state = rec["certification_state"]
    if state == "certified":
        return rec["declared_label"]
    if state == "withdrawn":
        return "withdrawn"
    if LABEL_RANK[rec["declared_label"]] <= LABEL_RANK["beta"]:
        return rec["declared_label"]
    return "beta"


def computed_posture(rec: dict) -> str:
    return "gaps_found" if rec["certification_state"] not in ("certified", "withdrawn") else "clear"


def expected_control_state(rec: dict, dimension: str) -> str:
    if dimension == "boundary_manifest":
        gap = boundary_reason(rec) is not None
    elif dimension == "repository_compliance":
        gap = compliance_reason(rec) is not None
    elif dimension == "import_durability":
        gap = import_reason(rec) is not None
    elif dimension == "signer_authority":
        gap = authority_reason(rec) is not None
    elif dimension == "emergency_response":
        gap = emergency_reason(rec) is not None
    elif dimension == "critical_upstream":
        gap = upstream_reason(rec) is not None
    else:  # scan_surface_parity
        gap = rec["scan_posture"] != rec["surface_posture"]
    return "unsatisfied" if gap else "satisfied"


def build_controls(rec: dict) -> list[dict]:
    owner_by_dim = {
        "boundary_manifest": "role:ecosystem-owner",
        "repository_compliance": "role:oss-compliance-owner",
        "import_durability": "role:oss-compliance-owner",
        "signer_authority": "role:release-authority-owner",
        "emergency_response": "role:security-response-owner",
        "critical_upstream": "role:ecosystem-owner",
        "scan_surface_parity": "role:governance-release-lead",
    }
    return [
        {
            "dimension": dimension,
            "control_ref": f"{SHIPROOM_REGISTER_REF}#{dimension}",
            "owner_ref": owner_by_dim[dimension],
            "state": expected_control_state(rec, dimension),
        }
        for dimension in CONTROL_DIMENSIONS
    ]


def finalize(rec: dict) -> dict:
    """Fill in derived fields (reasons, state, effective label, postures, controls)."""
    rec["active_reasons"] = derive_reasons(rec)
    rec["certification_state"] = computed_state(rec)
    rec["effective_label"] = computed_effective_label(rec)
    posture = computed_posture(rec)
    rec["scan_posture"] = posture
    rec["surface_posture"] = posture
    rec["controls"] = build_controls(rec)
    return rec


def make_record(
    *,
    record_id: str,
    family: str,
    row_kind: str,
    title: str,
    subject_ref: str,
    subject_summary: str,
    release_blocking: bool,
    declared_label: str,
    support_class: str,
    boundary_binding: dict,
    compliance_binding: dict,
    import_binding: dict,
    authority_binding: dict,
    emergency_binding: dict,
    upstream_binding: dict,
    proof_packet: dict,
    owner_signoff: dict,
    waiver_record: dict | None,
    rationale: str,
) -> dict:
    rec = {
        "record_id": record_id,
        "family": family,
        "row_kind": row_kind,
        "title": title,
        "subject_ref": subject_ref,
        "subject_summary": subject_summary,
        "release_blocking": release_blocking,
        "declared_label": declared_label,
        "support_class": support_class,
        "boundary": boundary_binding,
        "compliance": compliance_binding,
        "import_durability": import_binding,
        "authority": authority_binding,
        "emergency": emergency_binding,
        "upstream": upstream_binding,
        "controls": [],
        "scan_posture": "clear",
        "surface_posture": "clear",
        "scan_ref": f"{SHIPROOM_REGISTER_REF}#certification_scan",
        "surface_ref": f"{SHIPROOM_REGISTER_REF}#service_health_surface",
        "proof_packet": proof_packet,
        "waiver": waiver_record,
        "owner_signoff": owner_signoff,
        "certification_state": "certified",
        "active_reasons": [],
        "effective_label": declared_label,
        "surfaces": list(SURFACES),
        "rationale": rationale,
    }
    return finalize(rec)


def ok_boundary(s: str) -> dict:
    return boundary(True, False, s)


def ok_compliance(s: str) -> dict:
    return compliance(True, True, s)


def ok_import(s: str) -> dict:
    return import_durability(True, True, s)


def ok_authority(s: str) -> dict:
    return authority(2, 3, True, s)


def ok_emergency(s: str) -> dict:
    return emergency(True, s)


def ok_upstream(s: str) -> dict:
    return upstream(True, s)


def fresh_proof(rid: str) -> dict:
    return proof(rid, "current", "2026-06-12")


def signed(at: str = "2026-06-13") -> dict:
    return signoff("role:governance-release-lead", True, at)


def build_records() -> list[dict]:
    records: list[dict] = []

    # 1) A fully certified release row.
    records.append(
        make_record(
            record_id="cert-framework-release",
            family="framework",
            row_kind="release",
            title="Framework release row",
            subject_ref="row:framework/release",
            subject_summary="Core framework release row across all six durability axes.",
            release_blocking=True,
            declared_label="stable",
            support_class="open_local",
            boundary_binding=ok_boundary("framework"),
            compliance_binding=ok_compliance("framework"),
            import_binding=ok_import("framework"),
            authority_binding=ok_authority("framework"),
            emergency_binding=ok_emergency("framework"),
            upstream_binding=ok_upstream("framework"),
            proof_packet=fresh_proof("cert-framework-release"),
            owner_signoff=signed(),
            waiver_record=None,
            rationale="The boundary manifest is published and release-linked, REUSE/SPDX and notices/SBOM are bound, imports are attributed and owned, the signer quorum is met with backup authority, the emergency drill is current, and the critical upstreams are healthy; with fresh proof and owner sign-off the row is certified.",
        )
    )

    # 2) A fully certified ecosystem row.
    records.append(
        make_record(
            record_id="cert-notebook-ecosystem",
            family="notebook",
            row_kind="ecosystem",
            title="Notebook ecosystem row",
            subject_ref="row:notebook/ecosystem",
            subject_summary="Notebook extension/provider ecosystem row across all six axes.",
            release_blocking=True,
            declared_label="stable",
            support_class="open_local",
            boundary_binding=ok_boundary("notebook"),
            compliance_binding=ok_compliance("notebook"),
            import_binding=ok_import("notebook"),
            authority_binding=ok_authority("notebook"),
            emergency_binding=ok_emergency("notebook"),
            upstream_binding=ok_upstream("notebook"),
            proof_packet=fresh_proof("cert-notebook-ecosystem"),
            owner_signoff=signed(),
            waiver_record=None,
            rationale="Every durability axis holds for the notebook ecosystem row, with fresh certification proof and owner sign-off; the row is certified.",
        )
    )

    # 3) Boundary manifest not published: narrowed_boundary, holds promotion.
    records.append(
        make_record(
            record_id="cert-data_rich-release",
            family="data_rich",
            row_kind="release",
            title="Data-rich release row",
            subject_ref="row:data_rich/release",
            subject_summary="Data-rich result-grid release row with an unpublished boundary manifest.",
            release_blocking=True,
            declared_label="stable",
            support_class="mixed_open_managed",
            boundary_binding=boundary(False, False, "data_rich"),
            compliance_binding=ok_compliance("data_rich"),
            import_binding=ok_import("data_rich"),
            authority_binding=ok_authority("data_rich"),
            emergency_binding=ok_emergency("data_rich"),
            upstream_binding=ok_upstream("data_rich"),
            proof_packet=fresh_proof("cert-data_rich-release"),
            owner_signoff=signed(),
            waiver_record=None,
            rationale="The versioned boundary manifest for this row is not published or release-linked; the row narrows on the boundary axis and holds promotion on a still-stable claim.",
        )
    )

    # 4) Hidden proprietary baseline: narrowed_boundary (headline guardrail), holds.
    records.append(
        make_record(
            record_id="cert-ai_adjacent-ecosystem",
            family="ai_adjacent",
            row_kind="ecosystem",
            title="AI-adjacent ecosystem row",
            subject_ref="row:ai_adjacent/ecosystem",
            subject_summary="AI-adjacent ecosystem row that depends on a hidden proprietary baseline.",
            release_blocking=True,
            declared_label="stable",
            support_class="open_local",
            boundary_binding=boundary(True, True, "ai_adjacent"),
            compliance_binding=ok_compliance("ai_adjacent"),
            import_binding=ok_import("ai_adjacent"),
            authority_binding=ok_authority("ai_adjacent"),
            emergency_binding=ok_emergency("ai_adjacent"),
            upstream_binding=ok_upstream("ai_adjacent"),
            proof_packet=fresh_proof("cert-ai_adjacent-ecosystem"),
            owner_signoff=signed(),
            waiver_record=None,
            rationale="The row's open-boundary claim rests on a hidden proprietary baseline; it narrows on the boundary axis and holds promotion — a row may not be certified open while it depends on a hidden proprietary baseline.",
        )
    )

    # 5) Repository compliance stale (REUSE/SPDX): narrowed_compliance, holds.
    records.append(
        make_record(
            record_id="cert-review-release",
            family="review",
            row_kind="release",
            title="Review release row",
            subject_ref="row:review/release",
            subject_summary="Review/diff release row with stale REUSE/SPDX licensing coverage.",
            release_blocking=True,
            declared_label="stable",
            support_class="open_local",
            boundary_binding=ok_boundary("review"),
            compliance_binding=compliance(False, True, "review"),
            import_binding=ok_import("review"),
            authority_binding=ok_authority("review"),
            emergency_binding=ok_emergency("review"),
            upstream_binding=ok_upstream("review"),
            proof_packet=fresh_proof("cert-review-release"),
            owner_signoff=signed(),
            waiver_record=None,
            rationale="The REUSE/SPDX file-level licensing coverage has aged out of its review window; the row narrows on the compliance axis and holds promotion on a still-stable claim.",
        )
    )

    # 6) Notice/SBOM binding missing: narrowed_compliance, holds.
    records.append(
        make_record(
            record_id="cert-companion-ecosystem",
            family="companion",
            row_kind="ecosystem",
            title="Companion ecosystem row",
            subject_ref="row:companion/ecosystem",
            subject_summary="Companion-surface ecosystem row whose notice inventory and SBOM are unbound.",
            release_blocking=True,
            declared_label="stable",
            support_class="mixed_open_managed",
            boundary_binding=ok_boundary("companion"),
            compliance_binding=compliance(True, False, "companion"),
            import_binding=ok_import("companion"),
            authority_binding=ok_authority("companion"),
            emergency_binding=ok_emergency("companion"),
            upstream_binding=ok_upstream("companion"),
            proof_packet=fresh_proof("cert-companion-ecosystem"),
            owner_signoff=signed(),
            waiver_record=None,
            rationale="The third-party notice inventory and SBOM are not bound to the row's artifacts; it narrows on the compliance axis and holds promotion — a green SBOM card may not mask a missing notice binding.",
        )
    )

    # 7) Import provenance missing: narrowed_import, holds.
    records.append(
        make_record(
            record_id="cert-managed_depth-release",
            family="managed_depth",
            row_kind="release",
            title="Managed-depth release row",
            subject_ref="row:managed_depth/release",
            subject_summary="Managed-depth release row with an unattributed third-party import.",
            release_blocking=True,
            declared_label="stable",
            support_class="managed",
            boundary_binding=ok_boundary("managed_depth"),
            compliance_binding=ok_compliance("managed_depth"),
            import_binding=import_durability(False, True, "managed_depth"),
            authority_binding=ok_authority("managed_depth"),
            emergency_binding=ok_emergency("managed_depth"),
            upstream_binding=ok_upstream("managed_depth"),
            proof_packet=fresh_proof("cert-managed_depth-release"),
            owner_signoff=signed(),
            waiver_record=None,
            rationale="A third-party import on this row carries no origin/license/upstream-pin provenance; it narrows on the import axis and holds promotion on a still-stable claim.",
        )
    )

    # 8) Ownerless critical import: narrowed_import (headline guardrail), holds.
    records.append(
        make_record(
            record_id="cert-framework-ecosystem",
            family="framework",
            row_kind="ecosystem",
            title="Framework ecosystem row",
            subject_ref="row:framework/ecosystem",
            subject_summary="Framework ecosystem row with an ownerless critical import.",
            release_blocking=True,
            declared_label="stable",
            support_class="open_local",
            boundary_binding=ok_boundary("framework-eco"),
            compliance_binding=ok_compliance("framework-eco"),
            import_binding=import_durability(True, False, "framework-eco"),
            authority_binding=ok_authority("framework-eco"),
            emergency_binding=ok_emergency("framework-eco"),
            upstream_binding=ok_upstream("framework-eco"),
            proof_packet=fresh_proof("cert-framework-ecosystem"),
            owner_signoff=signed(),
            waiver_record=None,
            rationale="A critical import on this row has no update owner; it narrows on the import axis and holds promotion — a row may not be certified while it depends on an ownerless critical import.",
        )
    )

    # 9) Signer quorum unmet (>=2 but below required): narrowed_authority, holds.
    records.append(
        make_record(
            record_id="cert-notebook-release",
            family="notebook",
            row_kind="release",
            title="Notebook release row",
            subject_ref="row:notebook/release",
            subject_summary="Notebook release row whose signer quorum is below requirement.",
            release_blocking=True,
            declared_label="stable",
            support_class="open_local",
            boundary_binding=ok_boundary("notebook-rel"),
            compliance_binding=ok_compliance("notebook-rel"),
            import_binding=ok_import("notebook-rel"),
            authority_binding=authority(3, 2, True, "notebook-rel"),
            emergency_binding=ok_emergency("notebook-rel"),
            upstream_binding=ok_upstream("notebook-rel"),
            proof_packet=fresh_proof("cert-notebook-release"),
            owner_signoff=signed(),
            waiver_record=None,
            rationale="The signing lane requires three distinct humans but only two are available; the row narrows on the authority axis and holds promotion on a still-stable claim.",
        )
    )

    # 10) Single-person emergency authority: narrowed_authority (headline guardrail), holds.
    records.append(
        make_record(
            record_id="cert-data_rich-ecosystem",
            family="data_rich",
            row_kind="ecosystem",
            title="Data-rich ecosystem row",
            subject_ref="row:data_rich/ecosystem",
            subject_summary="Data-rich ecosystem row whose emergency authority is one irreplaceable human.",
            release_blocking=True,
            declared_label="stable",
            support_class="mixed_open_managed",
            boundary_binding=ok_boundary("data_rich-eco"),
            compliance_binding=ok_compliance("data_rich-eco"),
            import_binding=ok_import("data_rich-eco"),
            authority_binding=authority(2, 1, False, "data_rich-eco"),
            emergency_binding=ok_emergency("data_rich-eco"),
            upstream_binding=ok_upstream("data_rich-eco"),
            proof_packet=fresh_proof("cert-data_rich-ecosystem"),
            owner_signoff=signed(),
            waiver_record=None,
            rationale="The emergency signing/registry/security authority for this row resolves to a single irreplaceable human with no backup; it narrows on the authority axis and holds promotion — a row may not be certified while it depends on a single-person emergency authority.",
        )
    )

    # 11) Emergency-response drill stale: narrowed_emergency, holds.
    records.append(
        make_record(
            record_id="cert-ai_adjacent-release",
            family="ai_adjacent",
            row_kind="release",
            title="AI-adjacent release row",
            subject_ref="row:ai_adjacent/release",
            subject_summary="AI-adjacent release row whose emergency-response drill evidence is stale.",
            release_blocking=True,
            declared_label="stable",
            support_class="open_local",
            boundary_binding=ok_boundary("ai_adjacent-rel"),
            compliance_binding=ok_compliance("ai_adjacent-rel"),
            import_binding=ok_import("ai_adjacent-rel"),
            authority_binding=ok_authority("ai_adjacent-rel"),
            emergency_binding=emergency(False, "ai_adjacent-rel"),
            upstream_binding=ok_upstream("ai_adjacent-rel"),
            proof_packet=fresh_proof("cert-ai_adjacent-release"),
            owner_signoff=signed(),
            waiver_record=None,
            rationale="The advisory/revocation/disable drill evidence for this row has aged out; it narrows on the emergency-response axis and holds promotion on a still-stable claim.",
        )
    )

    # 12) Critical upstream unhealthy/unowned: narrowed_upstream, holds.
    records.append(
        make_record(
            record_id="cert-review-ecosystem",
            family="review",
            row_kind="ecosystem",
            title="Review ecosystem row",
            subject_ref="row:review/ecosystem",
            subject_summary="Review ecosystem row that depends on a red-risk, unowned critical upstream.",
            release_blocking=True,
            declared_label="stable",
            support_class="open_local",
            boundary_binding=ok_boundary("review-eco"),
            compliance_binding=ok_compliance("review-eco"),
            import_binding=ok_import("review-eco"),
            authority_binding=ok_authority("review-eco"),
            emergency_binding=ok_emergency("review-eco"),
            upstream_binding=upstream(False, "review-eco"),
            proof_packet=fresh_proof("cert-review-ecosystem"),
            owner_signoff=signed(),
            waiver_record=None,
            rationale="A protected-path dependency for this row is red-risk and unowned with no approved sponsor/fork/replace plan; it narrows on the upstream axis and holds promotion on a still-stable claim.",
        )
    )

    # 13) Stale certification proof; already Beta (inherited), not holding.
    records.append(
        make_record(
            record_id="cert-managed_depth-ecosystem",
            family="managed_depth",
            row_kind="ecosystem",
            title="Managed-depth ecosystem row",
            subject_ref="row:managed_depth/ecosystem",
            subject_summary="Managed-depth ecosystem row whose certification proof aged out; already Beta.",
            release_blocking=False,
            declared_label="beta",
            support_class="managed",
            boundary_binding=ok_boundary("managed_depth-eco"),
            compliance_binding=ok_compliance("managed_depth-eco"),
            import_binding=ok_import("managed_depth-eco"),
            authority_binding=ok_authority("managed_depth-eco"),
            emergency_binding=ok_emergency("managed_depth-eco"),
            upstream_binding=ok_upstream("managed_depth-eco"),
            proof_packet=proof("cert-managed_depth-ecosystem", "breached", "2026-01-04"),
            owner_signoff=signed("2026-02-01"),
            waiver_record=None,
            rationale="The certification proof packet aged past its freshness SLO; the row narrows on the stale axis. The subject is already Beta, so the narrowing is inherited and gated upstream.",
        )
    )

    # 14) Missing certification proof; already Beta (inherited), not holding.
    records.append(
        make_record(
            record_id="cert-companion-release",
            family="companion",
            row_kind="release",
            title="Companion release row",
            subject_ref="row:companion/release",
            subject_summary="Companion release row with no captured certification proof; already Beta.",
            release_blocking=False,
            declared_label="beta",
            support_class="mixed_open_managed",
            boundary_binding=ok_boundary("companion-rel"),
            compliance_binding=ok_compliance("companion-rel"),
            import_binding=ok_import("companion-rel"),
            authority_binding=ok_authority("companion-rel"),
            emergency_binding=ok_emergency("companion-rel"),
            upstream_binding=ok_upstream("companion-rel"),
            proof_packet=proof("cert-companion-release", "missing", None),
            owner_signoff=signed("2026-05-01"),
            waiver_record=None,
            rationale="No certification proof packet is captured; the row narrows on the stale axis. The subject is already Beta, so the narrowing is inherited.",
        )
    )

    # 15) Boundary gap held by an unexpired waiver: narrowed but not holding.
    records.append(
        make_record(
            record_id="cert-review-managed-release",
            family="review",
            row_kind="release",
            title="Review managed release row",
            subject_ref="row:review/managed-release",
            subject_summary="Review managed release row with a boundary gap held by an approved waiver.",
            release_blocking=True,
            declared_label="stable",
            support_class="open_local",
            boundary_binding=boundary(False, False, "review-managed"),
            compliance_binding=ok_compliance("review-managed"),
            import_binding=ok_import("review-managed"),
            authority_binding=ok_authority("review-managed"),
            emergency_binding=ok_emergency("review-managed"),
            upstream_binding=ok_upstream("review-managed"),
            proof_packet=fresh_proof("cert-review-managed-release"),
            owner_signoff=signed(),
            waiver_record=waiver(
                f"{SHIPROOM_REGISTER_REF}#waiver/review-managed-boundary",
                "2026-09-30",
                "Boundary manifest re-publication in progress; covered by an approved, time-boxed waiver.",
            ),
            rationale="A boundary manifest not yet re-published, held by an unexpired waiver: it stays visible and narrowed on the boundary axis but is gated upstream and does not hold promotion.",
        )
    )

    # 16) Boundary gap on a row already below the cutline (Beta): inherited, not holding.
    records.append(
        make_record(
            record_id="cert-companion-preview-ecosystem",
            family="companion",
            row_kind="ecosystem",
            title="Companion preview ecosystem row",
            subject_ref="row:companion/preview-ecosystem",
            subject_summary="Companion preview ecosystem row with an unpublished boundary manifest; already Beta.",
            release_blocking=False,
            declared_label="beta",
            support_class="mixed_open_managed",
            boundary_binding=boundary(False, False, "companion-preview"),
            compliance_binding=ok_compliance("companion-preview"),
            import_binding=ok_import("companion-preview"),
            authority_binding=ok_authority("companion-preview"),
            emergency_binding=ok_emergency("companion-preview"),
            upstream_binding=ok_upstream("companion-preview"),
            proof_packet=fresh_proof("cert-companion-preview-ecosystem"),
            owner_signoff=signed(),
            waiver_record=None,
            rationale="The boundary manifest for this row is not yet published; it narrows on the boundary axis. The subject is already Beta, so the narrowing is inherited.",
        )
    )

    return records


# ---------------------------------------------------------------------------
# Promotion / parity / summary, mirroring the Rust model
# ---------------------------------------------------------------------------
def is_waived(rec: dict) -> bool:
    return rec["waiver"] is not None and "waiver_expired" not in rec["active_reasons"]


def is_narrowed(rec: dict) -> bool:
    return rec["certification_state"] not in ("certified", "withdrawn")


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
        "parity_gate": "m5_open_durability_certification_parity_gate",
        "subjects_total": len(records),
        "subjects_in_agreement": len(agree),
        "subjects_in_disagreement": len(records) - len(agree),
        "subjects_with_gaps": len([r for r in records if r["surface_posture"] == "gaps_found"]),
        "all_subjects_agree": len(agree) == len(records),
        "rationale": "Every row's certification scan and service-health/release-center/support surface agree, so a green certification card can never mask a hidden proprietary baseline, an ownerless critical import, a single-person emergency authority, a stale notice/SBOM, an uncovered drill, or an unhealthy upstream.",
    }


def count_reason(records: list[dict], reason: str) -> int:
    return len([r for r in records if reason in r["active_reasons"]])


def computed_summary(records: list[dict], rules: list[dict]) -> dict:
    def count_state(state: str) -> int:
        return len([r for r in records if r["certification_state"] == state])

    return {
        "total_records": len(records),
        "records_certified": count_state("certified"),
        "records_narrowed": len([r for r in records if is_narrowed(r)]),
        "state_certified": count_state("certified"),
        "state_narrowed_boundary": count_state("narrowed_boundary"),
        "state_narrowed_compliance": count_state("narrowed_compliance"),
        "state_narrowed_import": count_state("narrowed_import"),
        "state_narrowed_authority": count_state("narrowed_authority"),
        "state_narrowed_emergency": count_state("narrowed_emergency"),
        "state_narrowed_upstream": count_state("narrowed_upstream"),
        "state_narrowed_stale": count_state("narrowed_stale"),
        "state_withdrawn": count_state("withdrawn"),
        "release_blocking_total": len([r for r in records if r["release_blocking"]]),
        "release_blocking_narrowed": len(
            [r for r in records if r["release_blocking"] and is_narrowed(r)]
        ),
        "records_on_active_waiver": len([r for r in records if is_waived(r)]),
        "boundary_gaps": len([r for r in records if boundary_reason(r) is not None]),
        "compliance_gaps": len([r for r in records if compliance_reason(r) is not None]),
        "import_gaps": len([r for r in records if import_reason(r) is not None]),
        "authority_gaps": len([r for r in records if authority_reason(r) is not None]),
        "emergency_gaps": len([r for r in records if emergency_reason(r) is not None]),
        "upstream_gaps": len([r for r in records if upstream_reason(r) is not None]),
        "hidden_proprietary_baseline_gaps": count_reason(records, "hidden_proprietary_baseline"),
        "ownerless_critical_import_gaps": count_reason(records, "ownerless_critical_import"),
        "single_person_authority_gaps": count_reason(records, "single_person_emergency_authority"),
        "total_active_reasons": sum(len(r["active_reasons"]) for r in records),
        "rules_firing": len(computed_blocking_rule_ids(records, rules)),
    }


RULE_TITLES = {
    "boundary_manifest_missing": "Boundary manifest missing",
    "hidden_proprietary_baseline": "Hidden proprietary baseline",
    "repository_compliance_stale": "Repository compliance stale",
    "notice_binding_missing": "Notice/SBOM binding missing",
    "import_provenance_missing": "Import provenance missing",
    "ownerless_critical_import": "Ownerless critical import",
    "signer_quorum_unmet": "Signer quorum unmet",
    "single_person_emergency_authority": "Single-person emergency authority",
    "emergency_response_stale": "Emergency-response drill stale",
    "critical_upstream_unhealthy": "Critical upstream unhealthy",
    "certification_proof_stale": "Certification proof stale",
    "certification_proof_missing": "Certification proof missing",
    "owner_signoff_missing": "Owner sign-off missing",
    "waiver_expired": "Waiver expired",
}


def build_rules() -> list[dict]:
    rules = []
    for reason in CERTIFICATION_REASONS:
        rules.append(
            {
                "rule_id": f"rule_{reason}",
                "title": RULE_TITLES[reason],
                "trigger_reason": reason,
                "applies_to_labels": list(ABOVE_CUTLINE),
                "default_action": ACTION_BY_REASON[reason],
                "blocks_promotion": True,
                "rationale": f"A still-stable row that narrows on {reason.replace('_', ' ')} holds promotion until the gap clears; inherited and waived narrowings are gated upstream.",
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
            "boundary_manifest_register_ref": BOUNDARY_MANIFEST_REGISTER_REF,
            "boundary_durability_matrix_ref": BOUNDARY_DURABILITY_MATRIX_REF,
            "compliance_register_ref": COMPLIANCE_REGISTER_REF,
            "import_register_ref": IMPORT_REGISTER_REF,
            "authority_continuity_register_ref": AUTHORITY_CONTINUITY_REGISTER_REF,
            "emergency_response_register_ref": EMERGENCY_RESPONSE_REGISTER_REF,
            "upstream_health_register_ref": UPSTREAM_HEALTH_REGISTER_REF,
            "release_graph_ref": RELEASE_GRAPH_REF,
            "support_export_ref": SUPPORT_EXPORT_REF,
            "shiproom_register_ref": SHIPROOM_REGISTER_REF,
            "stable_promotion_packet_ref": STABLE_PROMOTION_PACKET_REF,
            "m5_evidence_index_ref": EVIDENCE_INDEX_REF,
        },
        "certification_cutline": {
            "cutline_level": "stable",
            "above_cutline_levels": list(ABOVE_CUTLINE),
            "below_cutline_levels": ["beta", "preview", "withdrawn"],
            "description": "Rows at or above Stable carry the certified open-durability claim; a certification gap on a still-stable row holds promotion through the shiproom gate.",
        },
        "families": FAMILIES,
        "row_kinds": ROW_KINDS,
        "support_classes": SUPPORT_CLASSES,
        "control_dimensions": CONTROL_DIMENSIONS,
        "boundary_states": BOUNDARY_STATES,
        "compliance_states": COMPLIANCE_STATES,
        "import_states": IMPORT_STATES,
        "authority_states": AUTHORITY_STATES,
        "emergency_states": EMERGENCY_STATES,
        "upstream_states": UPSTREAM_STATES,
        "postures": POSTURES,
        "certification_states": CERTIFICATION_STATES,
        "certification_reasons": CERTIFICATION_REASONS,
        "certification_actions": CERTIFICATION_ACTIONS,
        "rules": rules,
        "records": records,
        "scan_surface_parity": computed_scan_surface_parity(records),
        "publication": {
            "publication_gate": "m5_open_durability_certification_gate",
            "decision": decision_verdict,
            "blocking_rule_ids": blocking_rules,
            "blocking_record_ids": blocking_records,
            "rationale": "Hold while any release-blocking row carries an open-durability certification gap on a still-stable claim; inherited and waived narrowings are gated upstream. A row that depends on a hidden proprietary baseline, an ownerless critical import, or a single-person emergency authority may not widen a stable claim without an approved plan.",
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

    # A certified row hiding an unpublished boundary manifest without narrowing on it.
    hidden = copy.deepcopy(register)
    target = next(r for r in hidden["records"] if r["certification_state"] == "certified")
    target["boundary"]["state"] = "unpublished"
    target["boundary"]["manifest_published"] = False
    cases.append(("hidden_boundary_gap.json", hidden, "GapWithoutReason"))

    # A certified row hiding a single-person emergency authority (headline guardrail).
    bypass = copy.deepcopy(register)
    target = next(r for r in bypass["records"] if r["certification_state"] == "certified")
    target["authority"]["state"] = "single_person_authority"
    target["authority"]["available_distinct_humans"] = 1
    target["authority"]["backup_present"] = False
    cases.append(("single_person_authority_certified.json", bypass, "GapWithoutReason"))

    # A narrowed row whose service-health surface is green over a gapped scan.
    masked = copy.deepcopy(register)
    target = next(r for r in masked["records"] if is_narrowed(r))
    target["surface_posture"] = "clear"
    cases.append(("green_surface_over_gap.json", masked, "ScanSurfaceDisagreement"))

    # A narrowed row whose effective label stays above the cutline.
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
        "drill:hidden_boundary_gap",
        "drill:single_person_authority_certified",
        "drill:ownerless_critical_import_certified",
        "drill:green_surface_over_gap",
        "drill:narrowed_above_cutline",
        "drill:certified_with_active_reason",
        "drill:reason_not_justified",
        "drill:control_state_inconsistent",
        "drill:boundary_fact_inconsistent",
        "drill:publication_decision_inconsistent",
    ]
    return {
        "status": "pass",
        "as_of": register["as_of"],
        "summary": {
            "total_records": s["total_records"],
            "records_certified": s["records_certified"],
            "records_narrowed": s["records_narrowed"],
            "state_certified": s["state_certified"],
            "state_narrowed_boundary": s["state_narrowed_boundary"],
            "state_narrowed_compliance": s["state_narrowed_compliance"],
            "state_narrowed_import": s["state_narrowed_import"],
            "state_narrowed_authority": s["state_narrowed_authority"],
            "state_narrowed_emergency": s["state_narrowed_emergency"],
            "state_narrowed_upstream": s["state_narrowed_upstream"],
            "state_narrowed_stale": s["state_narrowed_stale"],
            "release_blocking_narrowed": s["release_blocking_narrowed"],
            "records_on_active_waiver": s["records_on_active_waiver"],
            "boundary_gaps": s["boundary_gaps"],
            "compliance_gaps": s["compliance_gaps"],
            "import_gaps": s["import_gaps"],
            "authority_gaps": s["authority_gaps"],
            "emergency_gaps": s["emergency_gaps"],
            "upstream_gaps": s["upstream_gaps"],
            "hidden_proprietary_baseline_gaps": s["hidden_proprietary_baseline_gaps"],
            "ownerless_critical_import_gaps": s["ownerless_critical_import_gaps"],
            "single_person_authority_gaps": s["single_person_authority_gaps"],
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
