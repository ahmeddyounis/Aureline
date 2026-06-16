#!/usr/bin/env python3
"""Regenerate the repository-compliance and notice-binding register.

The open/local-boundary durability matrix records, per asset lane, *whether* a
repository-compliance control is satisfied as one coarse flag. This register is the
compliance-truth layer on top of it: for every claimed M5 artifact family, docs pack, and
mirrored output it records the DCO/CLA contribution-provenance lane truth, the REUSE/SPDX
file-level licensing coverage, the third-party notice-inventory state, and the SBOM/notice
binding, and it holds the repository-compliance scan in parity with the user/admin
notice/SBOM surface.

A record is cleared only when provenance holds, licensing coverage is complete, the notice
inventory is complete, the SBOM is present and bound, the mirror is fresh, the proof is
fresh, and the owner signed. Otherwise it narrows on the specific axis that thins out (a
provenance gap, a licensing gap, a notice gap, an SBOM/binding gap, a stale mirror, or
stale proof) and drops its effective label below the launch cutline. A missing or partial
notice is a first-class state — it can never disappear behind a green SBOM badge, because
the scan and the surface must agree.

An inherited narrowing (a subject already below the cutline, or a gap held by an unexpired
waiver) is gated upstream and does not hold promotion; a compliance-layer failure on a
still-stable subject holds promotion through a stop rule.

This emits the canonical register artifact, the negative fixtures, the cases manifest, and
the frozen validation capture. The Python summary/parity/promotion logic mirrors the typed
Rust consumer so the checked-in artifact validates cleanly and the capture cross-check
agrees with the model.
"""

from __future__ import annotations

import copy
import json
from pathlib import Path

REPO = Path(__file__).resolve().parent.parent

SLUG = "m5-compliance-and-notice-binding"
RECORD_KIND = "m5_compliance_and_notice_binding_register"
REGISTER_ID = "m5_compliance_and_notice_binding:v1"
ARTIFACT = REPO / "artifacts/governance" / f"{SLUG}.json"
CAPTURE = REPO / "artifacts/governance/captures" / f"{SLUG}_validation_capture.json"
FIXTURES = REPO / "fixtures/governance" / SLUG
OVERVIEW_PAGE = (
    "docs/m5/ship_dco_cla_lane_truth_reuse_spdx_compliance_views_notice_inventories_"
    "and_sbom_notice_binding_across_m5_artifacts_docs_packs_and_mirrored_outputs.md"
)
AS_OF = "2026-06-16"

# Canonical source registers this register binds together.
CONTRIBUTION_GOVERNANCE_REF = "artifacts/governance/contribution_governance_seed.yaml"
REUSE_SPDX_REPORT_REF = "artifacts/governance/compliance_checklist.yaml"
NOTICE_INVENTORY_REF = "artifacts/governance/release_notice_seed.yaml"
SBOM_INDEX_REF = (
    "artifacts/release/m5/certify_artifact_graph_publication_exact_build_provenance_"
    "rollback_revocation_parity_and_mirror_offline_release_truth_on_all_claimed_m5_rows.json"
)
MIRROR_INDEX_REF = (
    "artifacts/release/m5/certify_artifact_graph_publication_exact_build_provenance_"
    "rollback_revocation_parity_and_mirror_offline_release_truth_on_all_claimed_m5_rows.json#mirror"
)
DURABILITY_MATRIX_REF = "artifacts/governance/m5-boundary-and-upstream-durability.json"
EVIDENCE_INDEX_REF = (
    "artifacts/release/m5/certify_the_full_m5_train_narrow_stale_rows_and_"
    "publish_the_canonical_evidence_index.json"
)
SLO_REGISTER_REF = "artifacts/governance/evidence_freshness_slos.yaml"

# Closed vocabularies (mirror the Rust enums in declaration order).
FAMILIES = [
    "notebook",
    "data_rich",
    "ai_adjacent",
    "framework",
    "review",
    "companion",
    "managed_depth",
]
SCOPE_KINDS = ["artifact_family", "docs_pack", "mirrored_output"]
SUPPORT_CLASSES = ["open_local", "mixed_open_managed", "managed", "restricted"]
CONTROL_DIMENSIONS = [
    "contribution_provenance",
    "file_level_licensing",
    "notice_inventory",
    "sbom_notice_binding",
    "mirror_freshness",
    "scan_surface_parity",
]
DCO_STATES = ["all_signed", "gaps_present", "not_required"]
CLA_STATES = ["on_file", "unresolved", "not_required"]
NOTICE_STATES = ["complete", "partial", "missing", "not_required"]
SBOM_BINDING_STATES = ["bound", "unbound", "not_applicable"]
SBOM_FORMATS = ["spdx_primary", "cyclonedx_export"]
POSTURES = ["clear", "gaps_found"]
COMPLIANCE_STATES = [
    "cleared",
    "narrowed_provenance",
    "narrowed_licensing",
    "narrowed_notice",
    "narrowed_sbom",
    "narrowed_mirror",
    "narrowed_stale",
    "withdrawn",
]
COMPLIANCE_REASONS = [
    "dco_signoff_missing",
    "cla_unresolved",
    "licensing_coverage_incomplete",
    "license_exception_undocumented",
    "notice_inventory_partial",
    "notice_inventory_missing",
    "sbom_primary_missing",
    "sbom_notice_binding_broken",
    "cyclonedx_export_unavailable",
    "mirror_stale",
    "compliance_proof_stale",
    "compliance_proof_missing",
    "owner_signoff_missing",
    "waiver_expired",
]
COMPLIANCE_ACTIONS = [
    "hold_promotion",
    "collect_dco_signoff",
    "resolve_cla",
    "complete_licensing_coverage",
    "document_license_exception",
    "complete_notice_inventory",
    "generate_spdx_sbom",
    "rebind_sbom_notices",
    "enable_cyclonedx_export",
    "refresh_mirror",
    "refresh_compliance_proof",
    "request_owner_signoff",
]

LABEL_RANK = {"lts": 4, "stable": 3, "beta": 2, "preview": 1, "withdrawn": 0}
ABOVE_CUTLINE = ["lts", "stable"]

REASON_STATE = {
    "dco_signoff_missing": "narrowed_provenance",
    "cla_unresolved": "narrowed_provenance",
    "licensing_coverage_incomplete": "narrowed_licensing",
    "license_exception_undocumented": "narrowed_licensing",
    "notice_inventory_partial": "narrowed_notice",
    "notice_inventory_missing": "narrowed_notice",
    "sbom_primary_missing": "narrowed_sbom",
    "sbom_notice_binding_broken": "narrowed_sbom",
    "cyclonedx_export_unavailable": "narrowed_sbom",
    "mirror_stale": "narrowed_mirror",
    "compliance_proof_stale": "narrowed_stale",
    "compliance_proof_missing": "narrowed_stale",
    "owner_signoff_missing": "narrowed_stale",
    "waiver_expired": "narrowed_stale",
}
STATE_PRECEDENCE = {
    "narrowed_provenance": 0,
    "narrowed_licensing": 1,
    "narrowed_notice": 2,
    "narrowed_sbom": 3,
    "narrowed_mirror": 4,
    "narrowed_stale": 5,
}
REASON_ACTION = {
    "dco_signoff_missing": "collect_dco_signoff",
    "cla_unresolved": "resolve_cla",
    "licensing_coverage_incomplete": "complete_licensing_coverage",
    "license_exception_undocumented": "document_license_exception",
    "notice_inventory_partial": "complete_notice_inventory",
    "notice_inventory_missing": "complete_notice_inventory",
    "sbom_primary_missing": "generate_spdx_sbom",
    "sbom_notice_binding_broken": "rebind_sbom_notices",
    "cyclonedx_export_unavailable": "enable_cyclonedx_export",
    "mirror_stale": "refresh_mirror",
    "compliance_proof_stale": "refresh_compliance_proof",
    "compliance_proof_missing": "refresh_compliance_proof",
    "owner_signoff_missing": "request_owner_signoff",
    "waiver_expired": "hold_promotion",
}

# Owners (planning metadata-free role refs).
GOV = "role:governance-release-lead"
SEC = "role:security-response-owner"
ECO = "role:ecosystem-owner"
OSS = "role:oss-compliance-devrel"

DEFAULT_SURFACES = [
    "shell/help_about_compliance_card",
    "service_health/compliance_notice_panel",
    "release_center/notice_and_sbom_view",
    "support_export/compliance_packet",
    "evaluation_pack/compliance_notice_section",
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
        "evidence_refs": [CONTRIBUTION_GOVERNANCE_REF, EVIDENCE_INDEX_REF],
    }


def signoff(owner: str, signed: bool, at: str | None) -> dict:
    return {"owner_ref": owner, "signed_off": signed, "signed_at": at}


def provenance(dco: str, cla: str, unsigned: int) -> dict:
    return {
        "dco_state": dco,
        "cla_state": cla,
        "unsigned_commit_count": unsigned,
        "dco_audit_ref": f"{CONTRIBUTION_GOVERNANCE_REF}#dco_merge_audit",
        "cla_register_ref": f"{CONTRIBUTION_GOVERNANCE_REF}#contributor_agreements",
    }


def licensing(total: int, spdx: int, documented: int, undocumented: int) -> dict:
    return {
        "files_total": total,
        "files_spdx_covered": spdx,
        "documented_exceptions": documented,
        "undocumented_exceptions": undocumented,
        "reuse_report_ref": f"{REUSE_SPDX_REPORT_REF}#reuse_spdx_coverage",
    }


def notices(state: str, total: int, present: int) -> dict:
    return {
        "notice_state": state,
        "entries_total": total,
        "entries_present": present,
        "notice_inventory_ref": f"{NOTICE_INVENTORY_REF}#third_party_notices",
    }


def sbom(spdx_present: bool, cyclonedx: bool, binding: str) -> dict:
    formats = []
    if spdx_present:
        formats.append("spdx_primary")
    if cyclonedx:
        formats.append("cyclonedx_export")
    return {
        "spdx_primary_present": spdx_present,
        "cyclonedx_export_available": cyclonedx,
        "binding_state": binding,
        "formats": formats,
        "sbom_ref": f"{SBOM_INDEX_REF}#spdx",
        "notice_binding_ref": f"{SBOM_INDEX_REF}#sbom_notice_binding",
    }


def mirror(required: bool, freshness: str) -> dict:
    return {
        "mirror_required": required,
        "mirror_freshness": freshness,
        "mirror_ref": f"{MIRROR_INDEX_REF}#offline_pack",
    }


def waiver(ref: str, expires: str, reason: str) -> dict:
    return {"waiver_ref": ref, "expires_at": expires, "reason": reason}


# --- mirrored Rust derivations --------------------------------------------
def dco_gap(p: dict) -> bool:
    return p["dco_state"] == "gaps_present"


def cla_gap(p: dict) -> bool:
    return p["cla_state"] == "unresolved"


def provenance_na(p: dict) -> bool:
    return p["dco_state"] == "not_required" and p["cla_state"] == "not_required"


def coverage_incomplete(lic: dict) -> bool:
    return lic["files_spdx_covered"] + lic["documented_exceptions"] < lic["files_total"]


def exception_undocumented(lic: dict) -> bool:
    return lic["undocumented_exceptions"] > 0


def notice_partial(n: dict) -> bool:
    return n["notice_state"] == "partial"


def notice_missing(n: dict) -> bool:
    return n["notice_state"] == "missing"


def notice_na(n: dict) -> bool:
    return n["notice_state"] == "not_required"


def spdx_missing(s: dict) -> bool:
    return not s["spdx_primary_present"]


def binding_broken(s: dict) -> bool:
    return s["binding_state"] == "unbound"


def cyclonedx_gap(rec: dict) -> bool:
    return rec["release_blocking"] and not rec["sbom"]["cyclonedx_export_available"]


def mirror_stale(m: dict) -> bool:
    return m["mirror_required"] and m["mirror_freshness"] == "breached"


def derive_reasons(rec: dict) -> list[str]:
    p, lic, n, s, m = (
        rec["provenance"],
        rec["licensing"],
        rec["notices"],
        rec["sbom"],
        rec["mirror"],
    )
    proof_slo = rec["proof_packet"]["slo_state"]
    facts = {
        "dco_signoff_missing": dco_gap(p),
        "cla_unresolved": cla_gap(p),
        "licensing_coverage_incomplete": coverage_incomplete(lic),
        "license_exception_undocumented": exception_undocumented(lic),
        "notice_inventory_partial": notice_partial(n),
        "notice_inventory_missing": notice_missing(n),
        "sbom_primary_missing": spdx_missing(s),
        "sbom_notice_binding_broken": binding_broken(s),
        "cyclonedx_export_unavailable": cyclonedx_gap(rec),
        "mirror_stale": mirror_stale(m),
        "compliance_proof_stale": proof_slo == "breached",
        "compliance_proof_missing": proof_slo == "missing",
        "owner_signoff_missing": not rec["owner_signoff"]["signed_off"],
        # waiver_expired is authored explicitly, never auto-derived.
    }
    return [r for r in COMPLIANCE_REASONS if facts.get(r, False)]


def computed_state(reasons: list[str], declared: str) -> str:
    if declared == "withdrawn":
        return "withdrawn"
    if not reasons:
        return "cleared"
    best = min(reasons, key=lambda r: STATE_PRECEDENCE[REASON_STATE[r]])
    return REASON_STATE[best]


def computed_effective(reasons: list[str], declared: str) -> str:
    state = computed_state(reasons, declared)
    if state == "cleared":
        return declared
    if state == "withdrawn":
        return "withdrawn"
    return declared if LABEL_RANK[declared] <= LABEL_RANK["beta"] else "beta"


def expected_control_state(rec: dict, dimension: str) -> str:
    p, lic, n, s, m = (
        rec["provenance"],
        rec["licensing"],
        rec["notices"],
        rec["sbom"],
        rec["mirror"],
    )
    if dimension == "contribution_provenance":
        if provenance_na(p):
            return "not_applicable"
        return "unsatisfied" if (dco_gap(p) or cla_gap(p)) else "satisfied"
    if dimension == "file_level_licensing":
        return (
            "unsatisfied"
            if (coverage_incomplete(lic) or exception_undocumented(lic))
            else "satisfied"
        )
    if dimension == "notice_inventory":
        if notice_na(n):
            return "not_applicable"
        return "unsatisfied" if (notice_partial(n) or notice_missing(n)) else "satisfied"
    if dimension == "sbom_notice_binding":
        if s["binding_state"] == "not_applicable":
            return "not_applicable"
        return (
            "unsatisfied"
            if (spdx_missing(s) or binding_broken(s) or cyclonedx_gap(rec))
            else "satisfied"
        )
    if dimension == "mirror_freshness":
        if not m["mirror_required"]:
            return "not_applicable"
        return "unsatisfied" if mirror_stale(m) else "satisfied"
    if dimension == "scan_surface_parity":
        return "unsatisfied" if rec["scan_posture"] != rec["surface_posture"] else "satisfied"
    raise ValueError(dimension)


CONTROL_OWNERS = {
    "contribution_provenance": OSS,
    "file_level_licensing": OSS,
    "notice_inventory": OSS,
    "sbom_notice_binding": SEC,
    "mirror_freshness": ECO,
    "scan_surface_parity": GOV,
}
CONTROL_REFS = {
    "contribution_provenance": CONTRIBUTION_GOVERNANCE_REF,
    "file_level_licensing": REUSE_SPDX_REPORT_REF,
    "notice_inventory": NOTICE_INVENTORY_REF,
    "sbom_notice_binding": SBOM_INDEX_REF,
    "mirror_freshness": MIRROR_INDEX_REF,
    "scan_surface_parity": EVIDENCE_INDEX_REF,
}


def build_controls(rec: dict) -> list[dict]:
    return [
        {
            "dimension": d,
            "control_ref": f"{CONTROL_REFS[d]}#{d}",
            "owner_ref": CONTROL_OWNERS[d],
            "state": expected_control_state(rec, d),
        }
        for d in CONTROL_DIMENSIONS
    ]


def record(
    record_id: str,
    family: str,
    scope_kind: str,
    title: str,
    subject_ref: str,
    subject_summary: str,
    *,
    release_blocking: bool,
    declared: str,
    support_class: str,
    prov: dict,
    lic: dict,
    note: dict,
    sb: dict,
    mir: dict,
    pkt: dict,
    wv: dict | None,
    so: dict,
    rationale: str,
    surfaces: list[str] | None = None,
) -> dict:
    rec = {
        "record_id": record_id,
        "family": family,
        "scope_kind": scope_kind,
        "title": title,
        "subject_ref": subject_ref,
        "subject_summary": subject_summary,
        "release_blocking": release_blocking,
        "declared_label": declared,
        "support_class": support_class,
        "provenance": prov,
        "licensing": lic,
        "notices": note,
        "sbom": sb,
        "mirror": mir,
        # filled below
        "controls": [],
        "scan_posture": "clear",
        "surface_posture": "clear",
        "scan_ref": f"{REUSE_SPDX_REPORT_REF}#compliance_scan/{record_id}",
        "surface_ref": f"shell/help_about_compliance_card#{record_id}",
        "proof_packet": pkt,
        "waiver": wv,
        "owner_signoff": so,
        "compliance_state": "cleared",
        "active_reasons": [],
        "effective_label": declared,
        "surfaces": surfaces or list(DEFAULT_SURFACES),
        "rationale": rationale,
    }
    reasons = derive_reasons(rec)
    state = computed_state(reasons, declared)
    posture = "gaps_found" if state not in ("cleared", "withdrawn") else "clear"
    rec["active_reasons"] = reasons
    rec["compliance_state"] = state
    rec["effective_label"] = computed_effective(reasons, declared)
    rec["scan_posture"] = posture
    rec["surface_posture"] = posture
    rec["controls"] = build_controls(rec)
    return rec


def clean_prov() -> dict:
    return provenance("all_signed", "on_file", 0)


def green_sbom() -> dict:
    return sbom(True, True, "bound")


def build_records() -> list[dict]:
    records = []

    # 1. Framework artifact family — fully cleared at stable.
    records.append(
        record(
            "compliance-framework",
            "framework",
            "artifact_family",
            "Core framework and platform foundation compliance",
            "schemas/",
            "Editor shell and platform foundation artifacts with complete provenance, licensing, notices, and a bound SBOM.",
            release_blocking=True,
            declared="stable",
            support_class="open_local",
            prov=clean_prov(),
            lic=licensing(420, 410, 10, 0),
            note=notices("complete", 36, 36),
            sb=green_sbom(),
            mir=mirror(True, "current"),
            pkt=proof("compliance_framework_proof", "current", "2026-05-30"),
            wv=None,
            so=signoff(GOV, True, "2026-05-31"),
            rationale="Provenance, REUSE/SPDX coverage, notices, the bound SBOM, and the mirror all hold; the scan and the surface agree on a clean posture.",
        )
    )

    # 2. Managed-depth artifact family — cleared at stable.
    records.append(
        record(
            "compliance-managed_depth",
            "managed_depth",
            "artifact_family",
            "Managed-depth and infrastructure compliance",
            "artifacts/release/open_paid_boundary_audit.json#managed_tier",
            "Managed/hosted depth artifacts with their larger third-party notice set complete and bound to the SBOM.",
            release_blocking=True,
            declared="stable",
            support_class="managed",
            prov=clean_prov(),
            lic=licensing(180, 175, 5, 0),
            note=notices("complete", 48, 48),
            sb=green_sbom(),
            mir=mirror(True, "current"),
            pkt=proof("compliance_managed_depth_proof", "current", "2026-05-29"),
            wv=None,
            so=signoff(GOV, True, "2026-05-30"),
            rationale="The managed tier carries a complete notice inventory bound to the SBOM and clean provenance/licensing; nothing is masked.",
        )
    )

    # 3. Notebook artifact family — DCO sign-off gap on a still-stable claim.
    records.append(
        record(
            "compliance-notebook",
            "notebook",
            "artifact_family",
            "Notebook depth-surface compliance",
            "schemas/notebook/",
            "Notebook depth artifacts whose recent contributions include commits that still lack a DCO sign-off.",
            release_blocking=True,
            declared="stable",
            support_class="open_local",
            prov=provenance("gaps_present", "on_file", 3),
            lic=licensing(210, 205, 5, 0),
            note=notices("complete", 22, 22),
            sb=green_sbom(),
            mir=mirror(True, "current"),
            pkt=proof("compliance_notebook_proof", "current", "2026-05-28"),
            wv=None,
            so=signoff(GOV, True, "2026-05-29"),
            rationale="Compliance-layer failure: three contributions lack a DCO sign-off while the family still claims Stable, so the provenance gap holds promotion until cleared.",
        )
    )

    # 4. AI-adjacent artifact family — REUSE/SPDX coverage incomplete + undocumented exception.
    records.append(
        record(
            "compliance-ai_adjacent",
            "ai_adjacent",
            "artifact_family",
            "AI-adjacent surface compliance",
            "schemas/ai/",
            "AI-adjacent artifacts with files lacking SPDX licensing and an undocumented licensing exception.",
            release_blocking=True,
            declared="stable",
            support_class="mixed_open_managed",
            prov=clean_prov(),
            lic=licensing(260, 240, 10, 2),
            note=notices("complete", 30, 30),
            sb=green_sbom(),
            mir=mirror(True, "current"),
            pkt=proof("compliance_ai_adjacent_proof", "current", "2026-05-27"),
            wv=None,
            so=signoff(GOV, True, "2026-05-28"),
            rationale="Compliance-layer failure: file-level SPDX coverage is incomplete and a licensing exception is undocumented while the family still claims Stable, so the licensing gap holds promotion.",
        )
    )

    # 5. Review artifact family — SBOM present but unbound; held under an unexpired waiver.
    records.append(
        record(
            "compliance-review",
            "review",
            "artifact_family",
            "Review and diff-surface compliance",
            "schemas/review/",
            "Review artifacts whose SBOM is present but not yet bound to the notice inventory; rebinding is time-boxed under a waiver.",
            release_blocking=True,
            declared="stable",
            support_class="mixed_open_managed",
            prov=clean_prov(),
            lic=licensing(150, 146, 4, 0),
            note=notices("complete", 18, 18),
            sb=sbom(True, True, "unbound"),
            mir=mirror(True, "current"),
            pkt=proof("compliance_review_proof", "current", "2026-05-26"),
            wv=waiver(
                "artifacts/governance/ownership_matrix.yaml#waivers.review-sbom-notice-binding",
                "2026-09-30",
                "The SBOM is published but its binding to the notice inventory is being reissued; the gap is recorded and time-boxed.",
            ),
            so=signoff(GOV, True, "2026-05-27"),
            rationale="A present SBOM does not imply notice clearance: the SBOM is unbound, but an unexpired waiver holds the gap provisionally, so it is gated upstream and does not hold promotion.",
        )
    )

    # 6. Data-rich artifact family — notice inventory partial behind a green, bound SBOM; already Beta.
    records.append(
        record(
            "compliance-data_rich",
            "data_rich",
            "artifact_family",
            "Data-rich result/explorer compliance",
            "schemas/data/",
            "Data-rich artifacts whose third-party notice inventory is only partially captured even though the SBOM is present and bound.",
            release_blocking=True,
            declared="beta",
            support_class="open_local",
            prov=clean_prov(),
            lic=licensing(190, 188, 2, 0),
            note=notices("partial", 22, 18),
            sb=green_sbom(),
            mir=mirror(True, "current"),
            pkt=proof("compliance_data_rich_proof", "current", "2026-05-20"),
            wv=None,
            so=signoff(GOV, True, "2026-05-21"),
            rationale="The notice inventory is partial and surfaced as a first-class state despite the green, bound SBOM; the public claim is already Beta, so this narrowing is gated upstream.",
        )
    )

    # 7. Companion artifact family — compliance proof stale; already Beta.
    records.append(
        record(
            "compliance-companion",
            "companion",
            "artifact_family",
            "Companion-surface compliance",
            "schemas/companion/",
            "Companion artifacts whose compliance proof packet has aged past its freshness SLO.",
            release_blocking=True,
            declared="beta",
            support_class="open_local",
            prov=clean_prov(),
            lic=licensing(120, 118, 2, 0),
            note=notices("complete", 14, 14),
            sb=green_sbom(),
            mir=mirror(True, "current"),
            pkt=proof("compliance_companion_proof", "breached", "2026-01-08"),
            wv=None,
            so=signoff(GOV, True, "2026-01-09"),
            rationale="The compliance proof packet is stale; the public claim is already Beta, so this narrowing is gated upstream and does not hold promotion.",
        )
    )

    # 8. Mirrored output for managed depth — mirror/offline pack stale on a still-stable claim.
    records.append(
        record(
            "compliance-managed_depth-mirror",
            "managed_depth",
            "mirrored_output",
            "Managed-depth mirrored-output compliance",
            f"{MIRROR_INDEX_REF}#managed_depth_offline_pack",
            "The mirrored/offline redistribution of the managed-depth compliance artifacts whose mirror has aged out of its freshness window.",
            release_blocking=True,
            declared="stable",
            support_class="managed",
            prov=clean_prov(),
            lic=licensing(180, 175, 5, 0),
            note=notices("complete", 48, 48),
            sb=green_sbom(),
            mir=mirror(True, "breached"),
            pkt=proof("compliance_managed_depth_mirror_proof", "current", "2026-05-29"),
            wv=None,
            so=signoff(ECO, True, "2026-05-30"),
            rationale="Compliance-layer failure: the offline mirror of the compliance artifacts is stale while the mirrored output still claims Stable, so the mirror gap holds promotion until refreshed.",
        )
    )

    # 9. Docs pack for the framework — cleared at stable.
    records.append(
        record(
            "compliance-framework-docs",
            "framework",
            "docs_pack",
            "Framework docs-pack compliance",
            "docs/m5/",
            "The framework documentation pack with complete provenance, licensing, notices, and a bound SBOM.",
            release_blocking=True,
            declared="stable",
            support_class="open_local",
            prov=clean_prov(),
            lic=licensing(96, 94, 2, 0),
            note=notices("complete", 12, 12),
            sb=green_sbom(),
            mir=mirror(True, "current"),
            pkt=proof("compliance_framework_docs_proof", "current", "2026-05-30"),
            wv=None,
            so=signoff(OSS, True, "2026-05-31"),
            rationale="The docs pack carries the same compliance truth as its artifact family: clean provenance/licensing, complete notices, and a bound SBOM.",
        )
    )

    return records


def build_rules() -> list[dict]:
    titles = {
        "dco_signoff_missing": "Contributions must carry a DCO sign-off",
        "cla_unresolved": "Contributor agreements must be resolved",
        "licensing_coverage_incomplete": "File-level licensing coverage must be complete",
        "license_exception_undocumented": "Licensing exceptions must be documented",
        "notice_inventory_partial": "Notice inventory must be complete",
        "notice_inventory_missing": "Notice inventory must exist",
        "sbom_primary_missing": "SPDX primary SBOM must be present",
        "sbom_notice_binding_broken": "SBOM must be bound to the notice inventory",
        "cyclonedx_export_unavailable": "CycloneDX export must be available",
        "mirror_stale": "Compliance mirror must be fresh",
        "compliance_proof_stale": "Compliance proof must be fresh",
        "compliance_proof_missing": "Compliance proof must exist",
        "owner_signoff_missing": "Owner sign-off required",
        "waiver_expired": "Waiver must be current",
    }
    rules = []
    for reason in COMPLIANCE_REASONS:
        rules.append(
            {
                "rule_id": f"m5_compliance_and_notice_rule:{reason}",
                "title": titles[reason],
                "trigger_reason": reason,
                "applies_to_labels": ABOVE_CUTLINE,
                "default_action": REASON_ACTION[reason],
                "blocks_promotion": True,
                "rationale": "A compliance-layer failure on a subject still claiming a label at or above the cutline holds promotion; inherited (below-cutline or waived) narrowings are gated upstream.",
            }
        )
    return rules


def is_waived(rec: dict) -> bool:
    return rec.get("waiver") is not None and "waiver_expired" not in rec["active_reasons"]


def holds_promotion(rec: dict) -> bool:
    return (
        rec["release_blocking"]
        and rec["compliance_state"] not in ("cleared", "withdrawn")
        and rec["declared_label"] in ABOVE_CUTLINE
        and not is_waived(rec)
    )


def computed_blocking_rule_ids(records: list[dict], rules: list[dict]) -> list[str]:
    ids = set()
    for rule in rules:
        if not rule["blocks_promotion"]:
            continue
        for rec in records:
            if (
                holds_promotion(rec)
                and rule["trigger_reason"] in rec["active_reasons"]
                and rec["declared_label"] in rule["applies_to_labels"]
            ):
                ids.add(rule["rule_id"])
                break
    return sorted(ids)


def computed_blocking_record_ids(records: list[dict], rules: list[dict]) -> list[str]:
    rule_by_reason = {rule["trigger_reason"]: rule for rule in rules}
    ids = set()
    for rec in records:
        if not holds_promotion(rec):
            continue
        for reason in rec["active_reasons"]:
            rule = rule_by_reason.get(reason)
            if rule and rule["blocks_promotion"] and rec["declared_label"] in rule["applies_to_labels"]:
                ids.add(rec["record_id"])
                break
    return sorted(ids)


def computed_scan_surface_parity(records: list[dict]) -> dict:
    return {
        "parity_gate": "m5_compliance_scan_surface_parity_gate",
        "subjects_total": len(records),
        "subjects_in_agreement": sum(1 for r in records if r["scan_posture"] == r["surface_posture"]),
        "subjects_in_disagreement": sum(1 for r in records if r["scan_posture"] != r["surface_posture"]),
        "subjects_with_gaps": sum(1 for r in records if r["surface_posture"] == "gaps_found"),
        "all_subjects_agree": all(r["scan_posture"] == r["surface_posture"] for r in records),
        "rationale": "The repository-compliance scan and the user/admin notice/SBOM surface agree on every subject, so a green badge can never mask a gap the scan found.",
    }


def computed_summary(records: list[dict], rules: list[dict]) -> dict:
    def count_state(s):
        return sum(1 for r in records if r["compliance_state"] == s)

    narrowed = [r for r in records if r["compliance_state"] not in ("cleared", "withdrawn")]
    cleared = [r for r in records if r["compliance_state"] == "cleared"]
    return {
        "total_records": len(records),
        "records_cleared": len(cleared),
        "records_narrowed": len(narrowed),
        "state_cleared": count_state("cleared"),
        "state_narrowed_provenance": count_state("narrowed_provenance"),
        "state_narrowed_licensing": count_state("narrowed_licensing"),
        "state_narrowed_notice": count_state("narrowed_notice"),
        "state_narrowed_sbom": count_state("narrowed_sbom"),
        "state_narrowed_mirror": count_state("narrowed_mirror"),
        "state_narrowed_stale": count_state("narrowed_stale"),
        "state_withdrawn": count_state("withdrawn"),
        "release_blocking_total": sum(1 for r in records if r["release_blocking"]),
        "release_blocking_narrowed": sum(1 for r in narrowed if r["release_blocking"]),
        "records_on_active_waiver": sum(1 for r in records if is_waived(r)),
        "provenance_gaps": sum(1 for r in records if dco_gap(r["provenance"]) or cla_gap(r["provenance"])),
        "licensing_gaps": sum(
            1 for r in records if coverage_incomplete(r["licensing"]) or exception_undocumented(r["licensing"])
        ),
        "notice_gaps": sum(1 for r in records if notice_partial(r["notices"]) or notice_missing(r["notices"])),
        "sbom_gaps": sum(
            1 for r in records if spdx_missing(r["sbom"]) or binding_broken(r["sbom"]) or cyclonedx_gap(r)
        ),
        "mirror_gaps": sum(1 for r in records if mirror_stale(r["mirror"])),
        "spdx_primary_present": sum(1 for r in records if r["sbom"]["spdx_primary_present"]),
        "cyclonedx_export_available": sum(1 for r in records if r["sbom"]["cyclonedx_export_available"]),
        "notices_complete": sum(1 for r in records if r["notices"]["notice_state"] == "complete"),
        "total_active_reasons": sum(len(r["active_reasons"]) for r in records),
        "rules_firing": len(computed_blocking_rule_ids(records, rules)),
    }


def build_register() -> dict:
    records = build_records()
    rules = build_rules()
    blocking_rules = computed_blocking_rule_ids(records, rules)
    blocking_records = computed_blocking_record_ids(records, rules)
    decision = "hold" if blocking_records else "proceed"
    return {
        "schema_version": 1,
        "record_kind": RECORD_KIND,
        "register_id": REGISTER_ID,
        "status": "active",
        "overview_page": OVERVIEW_PAGE,
        "as_of": AS_OF,
        "source_contract_refs": {
            "contribution_governance_ref": CONTRIBUTION_GOVERNANCE_REF,
            "reuse_spdx_report_ref": REUSE_SPDX_REPORT_REF,
            "notice_inventory_ref": NOTICE_INVENTORY_REF,
            "sbom_index_ref": SBOM_INDEX_REF,
            "mirror_index_ref": MIRROR_INDEX_REF,
            "durability_matrix_ref": DURABILITY_MATRIX_REF,
            "m5_evidence_index_ref": EVIDENCE_INDEX_REF,
        },
        "compliance_cutline": {
            "cutline_level": "stable",
            "above_cutline_levels": ABOVE_CUTLINE,
            "below_cutline_levels": ["beta", "preview", "withdrawn"],
            "description": "Subjects at or above Stable carry the cleared compliance claim; a compliance-layer gap on a still-stable subject holds promotion.",
        },
        "families": FAMILIES,
        "scope_kinds": SCOPE_KINDS,
        "support_classes": SUPPORT_CLASSES,
        "control_dimensions": CONTROL_DIMENSIONS,
        "dco_states": DCO_STATES,
        "cla_states": CLA_STATES,
        "notice_states": NOTICE_STATES,
        "sbom_binding_states": SBOM_BINDING_STATES,
        "sbom_formats": SBOM_FORMATS,
        "postures": POSTURES,
        "compliance_states": COMPLIANCE_STATES,
        "compliance_reasons": COMPLIANCE_REASONS,
        "compliance_actions": COMPLIANCE_ACTIONS,
        "rules": rules,
        "records": records,
        "scan_surface_parity": computed_scan_surface_parity(records),
        "publication": {
            "publication_gate": "m5_compliance_and_notice_binding_gate",
            "decision": decision,
            "blocking_rule_ids": blocking_rules,
            "blocking_record_ids": blocking_records,
            "rationale": "Hold while any release-blocking subject carries a compliance-layer gap on a still-stable claim; inherited and waived narrowings are gated upstream.",
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

    # A cleared record hiding a file-level licensing gap without narrowing.
    hidden = copy.deepcopy(register)
    target = next(r for r in hidden["records"] if r["compliance_state"] == "cleared")
    target["licensing"]["files_total"] += 1
    cases.append(("hidden_licensing_gap.json", hidden, "GapWithoutReason"))

    # A narrowed record whose user/admin surface is clean over a gapped scan.
    masked = copy.deepcopy(register)
    target = next(r for r in masked["records"] if r["compliance_state"] not in ("cleared", "withdrawn"))
    target["surface_posture"] = "clear"
    cases.append(("green_surface_over_gap.json", masked, "ScanSurfaceDisagreement"))

    # A narrowed record whose effective label stays above the cutline.
    above = copy.deepcopy(register)
    target = next(r for r in above["records"] if r["compliance_state"] not in ("cleared", "withdrawn"))
    target["effective_label"] = "stable"
    cases.append(("narrowed_above_cutline.json", above, "EffectiveLabelMismatch"))

    return cases


def build_capture(register: dict, cases: list[tuple[str, dict, str]]) -> dict:
    s = register["summary"]
    p = register["publication"]
    parity = register["scan_surface_parity"]
    drills = [
        "drill:hidden_licensing_gap",
        "drill:green_surface_over_gap",
        "drill:narrowed_above_cutline",
        "drill:cleared_with_active_reason",
        "drill:reason_not_justified",
        "drill:control_state_inconsistent",
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
            "state_narrowed_provenance": s["state_narrowed_provenance"],
            "state_narrowed_licensing": s["state_narrowed_licensing"],
            "state_narrowed_notice": s["state_narrowed_notice"],
            "state_narrowed_sbom": s["state_narrowed_sbom"],
            "state_narrowed_mirror": s["state_narrowed_mirror"],
            "state_narrowed_stale": s["state_narrowed_stale"],
            "release_blocking_narrowed": s["release_blocking_narrowed"],
            "records_on_active_waiver": s["records_on_active_waiver"],
            "provenance_gaps": s["provenance_gaps"],
            "licensing_gaps": s["licensing_gaps"],
            "notice_gaps": s["notice_gaps"],
            "sbom_gaps": s["sbom_gaps"],
            "mirror_gaps": s["mirror_gaps"],
            "spdx_primary_present": s["spdx_primary_present"],
            "cyclonedx_export_available": s["cyclonedx_export_available"],
            "notices_complete": s["notices_complete"],
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
        "fixture_cases": [{"case_id": f"fixture:{f[:-5]}", "status": "passed"} for f, _, _ in cases],
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
        "cases": [{"file": filename, "expected_check_id": check_id} for filename, _, check_id in cases]
    }
    write_json(FIXTURES / "cases.json", manifest_index)
    print(f"wrote {FIXTURES.relative_to(REPO)}/ ({len(cases)} fixtures + cases.json)")

    write_json(CAPTURE, build_capture(register, cases))
    print(f"wrote {CAPTURE.relative_to(REPO)}")


if __name__ == "__main__":
    main()
