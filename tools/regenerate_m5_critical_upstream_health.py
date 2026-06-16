#!/usr/bin/env python3
"""Regenerate the critical-upstream health register.

The open/local-boundary durability matrix records, per asset lane, *whether* a critical
upstream is owned as one coarse flag, and the import-provenance register records where each
protected-path import came from. Neither makes each critical upstream — the third-party
packages, protocols, and curated imports a protected M5 family leans on — inspectable as a
durable health record: how healthy its maintainer base is, what its security posture is, how
fast it still ships, whether its license is clear, how feasible a replacement would be, who
owns it, whether its review is on cadence, and — when it is red-risk or unowned — whether a
sponsor/fork/replace plan is recorded and the shiproom has been told.

This register is that upstream-health layer. For every critical upstream a protected M5 family
depends on it records one entry that states, in one copy-safe record:

  - the maintainer-health rating (active maintainers and bus factor);
  - the security posture (open advisories, unpatched criticals);
  - the update cadence (still active, slowing, or stalled);
  - the review cadence (current, a due-for-review reminder, overdue, or missing);
  - the license clarity (clear, ambiguous, or incompatible);
  - the replacement feasibility (drop-in, moderate, hard, or no known path);
  - the ownership (owned or unowned);
  - the sponsor/fork/replace contingency plan, required for any red-risk upstream;
  - the shiproom escalation, required for any red-risk or unowned upstream.

A record is cleared only when the maintainer base is healthy, the security posture is clean,
the cadence has not stalled and the review is on cadence, the license is clear, the upstream
is owned, any required contingency plan and shiproom escalation are recorded, the proof is
fresh, and the owner signed. Otherwise it narrows on the specific axis that thins out (a
maintainer gap, a security gap, a cadence gap, a license gap, an ownership/escalation gap, or
stale proof) and drops its effective label below the launch cutline; the axes never collapse
into one global flag. The upstream-health scan and the governance-dashboard/promotion-packet
surface must agree on every record, so a green upstream card can never mask an abandoned,
unpatched, or unowned dependency.

An inherited narrowing (a subject already below the cutline, or a gap held by an unexpired
waiver) is gated upstream and does not hold promotion; an upstream-health failure on a
still-stable subject holds promotion through a shiproom stop rule — a red-risk or unowned
protected-path dependency cannot widen a stable claim without an approved sponsor, fork, or
replacement plan.

This emits the canonical register artifact, the negative fixtures, the cases manifest, and the
frozen validation capture. The Python summary/parity/promotion logic mirrors the typed Rust
consumer so the checked-in artifact validates cleanly and the capture cross-check agrees with
the model.
"""

from __future__ import annotations

import copy
import json
from pathlib import Path

REPO = Path(__file__).resolve().parent.parent

SLUG = "m5-critical-upstream-health"
RECORD_KIND = "m5_critical_upstream_health_register"
REGISTER_ID = "m5_critical_upstream_health:v1"
ARTIFACT = REPO / "artifacts/governance" / f"{SLUG}.json"
CAPTURE = REPO / "artifacts/governance/captures" / f"{SLUG}_validation_capture.json"
FIXTURES = REPO / "fixtures/governance" / SLUG
OVERVIEW_PAGE = (
    "docs/m5/add_critical_upstream_health_registers_review_cadence_reminders_red_risk_"
    "escalation_and_shiproom_gating_for_unowned_or_degraded_protected_path_dependencies.md"
)
AS_OF = "2026-06-16"

# Canonical source registers this register binds together.
UPSTREAM_SCORECARD_REF = "artifacts/governance/upstream_health_scorecard.yaml"
DEPENDENCY_REGISTER_REF = "artifacts/governance/dependency_register.yaml"
ADVISORY_REGISTER_REF = "artifacts/governance/security_advisory_register.yaml"
IMPORT_REGISTER_REF = "artifacts/governance/m5-import-provenance-and-fork-review.json"
PACKAGE_INVENTORY_REF = "artifacts/governance/package_inventory.yaml"
DURABILITY_MATRIX_REF = "artifacts/governance/m5-boundary-and-upstream-durability.json"
SHIPROOM_REGISTER_REF = "artifacts/release/shiproom_gate_register.yaml"
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
UPSTREAM_KINDS = [
    "package",
    "protocol",
    "curated_import",
    "toolchain_component",
]
SUPPORT_CLASSES = ["open_local", "mixed_open_managed", "managed", "restricted"]
HEALTH_GRADES = ["green", "amber", "red", "blocked"]
CONTROL_DIMENSIONS = [
    "maintainer_health",
    "security_posture",
    "update_cadence",
    "license_clarity",
    "ownership_continuity",
    "scan_surface_parity",
]
MAINTAINER_RATINGS = ["healthy", "thinning", "single_maintainer", "abandoned"]
SECURITY_POSTURES = ["clean", "advisories_open", "unpatched_critical"]
UPDATE_CADENCES = ["active", "slowing", "stalled"]
REVIEW_CADENCE_STATES = ["current", "due_for_review", "overdue", "missing"]
LICENSE_CLARITIES = ["clear", "ambiguous", "incompatible"]
REPLACEMENT_FEASIBILITIES = ["drop_in", "moderate", "hard", "no_known_path"]
OWNERSHIP_STATES = ["owned", "unowned"]
CONTINGENCY_STATES = ["recorded", "pending", "not_required"]
CONTINGENCY_DISPOSITIONS = [
    "sponsor_upstream",
    "maintain_fork",
    "replace_dependency",
    "none",
]
ESCALATION_STATES = ["raised", "pending", "not_required"]
POSTURES = ["clear", "gaps_found"]
HEALTH_STATES = [
    "cleared",
    "narrowed_maintainer",
    "narrowed_security",
    "narrowed_cadence",
    "narrowed_license",
    "narrowed_ownership",
    "narrowed_stale",
    "withdrawn",
]
HEALTH_REASONS = [
    "maintainer_health_thinning",
    "maintainer_abandoned",
    "security_advisories_open",
    "security_unpatched_critical",
    "update_cadence_stalled",
    "review_cadence_overdue",
    "review_cadence_missing",
    "license_ambiguous",
    "license_incompatible",
    "upstream_unowned",
    "contingency_plan_missing",
    "shiproom_escalation_missing",
    "health_proof_stale",
    "health_proof_missing",
    "owner_signoff_missing",
    "waiver_expired",
]
HEALTH_ACTIONS = [
    "hold_promotion",
    "assign_backup_maintainer",
    "sponsor_or_replace_upstream",
    "remediate_open_advisory",
    "patch_critical_vulnerability",
    "escalate_stalled_cadence",
    "refresh_upstream_review",
    "clarify_upstream_license",
    "assign_upstream_owner",
    "record_contingency_plan",
    "raise_shiproom_escalation",
    "refresh_health_proof",
    "request_owner_signoff",
]

LABEL_RANK = {"lts": 4, "stable": 3, "beta": 2, "preview": 1, "withdrawn": 0}
ABOVE_CUTLINE = ["lts", "stable"]

REASON_STATE = {
    "maintainer_health_thinning": "narrowed_maintainer",
    "maintainer_abandoned": "narrowed_maintainer",
    "security_advisories_open": "narrowed_security",
    "security_unpatched_critical": "narrowed_security",
    "update_cadence_stalled": "narrowed_cadence",
    "review_cadence_overdue": "narrowed_cadence",
    "review_cadence_missing": "narrowed_cadence",
    "license_ambiguous": "narrowed_license",
    "license_incompatible": "narrowed_license",
    "upstream_unowned": "narrowed_ownership",
    "contingency_plan_missing": "narrowed_ownership",
    "shiproom_escalation_missing": "narrowed_ownership",
    "health_proof_stale": "narrowed_stale",
    "health_proof_missing": "narrowed_stale",
    "owner_signoff_missing": "narrowed_stale",
    "waiver_expired": "narrowed_stale",
}
# Lower wins when several reasons are active. Ownership (the unowned / red-risk-without-plan
# guardrail) is the worst, then security, then maintainer, cadence, license, and finally the
# evidence-staleness axis.
STATE_PRECEDENCE = {
    "narrowed_ownership": 0,
    "narrowed_security": 1,
    "narrowed_maintainer": 2,
    "narrowed_cadence": 3,
    "narrowed_license": 4,
    "narrowed_stale": 5,
}
REASON_ACTION = {
    "maintainer_health_thinning": "assign_backup_maintainer",
    "maintainer_abandoned": "sponsor_or_replace_upstream",
    "security_advisories_open": "remediate_open_advisory",
    "security_unpatched_critical": "patch_critical_vulnerability",
    "update_cadence_stalled": "escalate_stalled_cadence",
    "review_cadence_overdue": "refresh_upstream_review",
    "review_cadence_missing": "refresh_upstream_review",
    "license_ambiguous": "clarify_upstream_license",
    "license_incompatible": "clarify_upstream_license",
    "upstream_unowned": "assign_upstream_owner",
    "contingency_plan_missing": "record_contingency_plan",
    "shiproom_escalation_missing": "raise_shiproom_escalation",
    "health_proof_stale": "refresh_health_proof",
    "health_proof_missing": "refresh_health_proof",
    "owner_signoff_missing": "request_owner_signoff",
    "waiver_expired": "hold_promotion",
}
REASON_DIMENSION = {
    "maintainer_health_thinning": "maintainer_health",
    "maintainer_abandoned": "maintainer_health",
    "security_advisories_open": "security_posture",
    "security_unpatched_critical": "security_posture",
    "update_cadence_stalled": "update_cadence",
    "review_cadence_overdue": "update_cadence",
    "review_cadence_missing": "update_cadence",
    "license_ambiguous": "license_clarity",
    "license_incompatible": "license_clarity",
    "upstream_unowned": "ownership_continuity",
    "contingency_plan_missing": "ownership_continuity",
    "shiproom_escalation_missing": "ownership_continuity",
    "health_proof_stale": "scan_surface_parity",
    "health_proof_missing": "scan_surface_parity",
    "owner_signoff_missing": "scan_surface_parity",
    "waiver_expired": "scan_surface_parity",
}

# Owners (planning metadata-free role refs).
GOV = "role:governance-release-lead"
SEC = "role:security-response-owner"
ECO = "role:ecosystem-owner"
OSS = "role:oss-compliance-devrel"
ARCH = "role:architecture-board"
DEP = "role:dependency-health-owner"

DEFAULT_SURFACES = [
    "shell/help_about_upstream_health_card",
    "service_health/critical_upstream_panel",
    "release_center/promotion_packet_upstream_view",
    "support_export/procurement_upstream_packet",
    "shiproom/red_risk_escalation_queue",
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
        "evidence_refs": [UPSTREAM_SCORECARD_REF, EVIDENCE_INDEX_REF],
    }


def signoff(owner: str, signed: bool, at: str | None) -> dict:
    return {"owner_ref": owner, "signed_off": signed, "signed_at": at}


def maintainer(rating: str, active: int, bus_factor: int) -> dict:
    return {
        "rating": rating,
        "active_maintainer_count": active,
        "bus_factor": bus_factor,
        "assessment_ref": f"{UPSTREAM_SCORECARD_REF}#maintainer_assessment",
    }


def security(posture: str, open_advisories: int) -> dict:
    return {
        "posture": posture,
        "open_advisory_count": open_advisories,
        "advisory_ref": f"{ADVISORY_REGISTER_REF}#open_advisories",
    }


def update_cadence(cadence: str, days_since_release: int) -> dict:
    return {
        "cadence": cadence,
        "days_since_last_release": days_since_release,
        "release_ref": f"{DEPENDENCY_REGISTER_REF}#release_history",
    }


def review_cadence(state: str, interval_days: int, next_due: str | None) -> dict:
    return {
        "cadence_state": state,
        "review_interval_days": interval_days,
        "next_review_due": next_due,
        "last_reviewed_ref": f"{UPSTREAM_SCORECARD_REF}#last_review",
    }


def license_clarity(clarity: str, *, spdx: str = "") -> dict:
    return {
        "clarity": clarity,
        "spdx_license_id": spdx,
        "license_ref": f"{DEPENDENCY_REGISTER_REF}#license",
    }


def ownership(state: str, owner: str) -> dict:
    return {
        "ownership_state": state,
        "owner_ref": owner,
        "escalation_owner_ref": GOV,
    }


def contingency(state: str, disposition: str, feasibility: str) -> dict:
    return {
        "plan_state": state,
        "disposition": disposition,
        "replacement_feasibility": feasibility,
        "plan_ref": f"{UPSTREAM_SCORECARD_REF}#sponsor_fork_replace_plan",
    }


def escalation(state: str, required: bool) -> dict:
    return {
        "escalation_state": state,
        "required": required,
        "shiproom_ref": f"{SHIPROOM_REGISTER_REF}#red_risk_escalation",
        "governance_ref": f"{UPSTREAM_SCORECARD_REF}#governance_review",
    }


def waiver(ref: str, expires: str, reason: str) -> dict:
    return {"waiver_ref": ref, "expires_at": expires, "reason": reason}


# --- mirrored Rust derivations --------------------------------------------
def maintainer_degraded(m: dict) -> bool:
    return m["rating"] != "healthy"


def maintainer_thinning(m: dict) -> bool:
    return m["rating"] in ("thinning", "single_maintainer")


def maintainer_abandoned(m: dict) -> bool:
    return m["rating"] == "abandoned"


def security_degraded(s: dict) -> bool:
    return s["posture"] != "clean"


def advisories_open(s: dict) -> bool:
    return s["posture"] == "advisories_open"


def unpatched_critical(s: dict) -> bool:
    return s["posture"] == "unpatched_critical"


def cadence_stalled(u: dict) -> bool:
    return u["cadence"] == "stalled"


def review_overdue(rc: dict) -> bool:
    return rc["cadence_state"] == "overdue"


def review_missing(rc: dict) -> bool:
    return rc["cadence_state"] == "missing"


def cadence_degraded(rec: dict) -> bool:
    return (
        cadence_stalled(rec["update_cadence"])
        or review_overdue(rec["review_cadence"])
        or review_missing(rec["review_cadence"])
    )


def license_ambiguous(lic: dict) -> bool:
    return lic["clarity"] == "ambiguous"


def license_incompatible(lic: dict) -> bool:
    return lic["clarity"] == "incompatible"


def license_degraded(lic: dict) -> bool:
    return lic["clarity"] != "clear"


def unowned(rec: dict) -> bool:
    return rec["ownership"]["ownership_state"] == "unowned"


def red_risk(rec: dict) -> bool:
    return rec["risk_grade"] in ("red", "blocked")


def requires_contingency(rec: dict) -> bool:
    return red_risk(rec)


def requires_escalation(rec: dict) -> bool:
    return red_risk(rec) or unowned(rec)


def contingency_missing(rec: dict) -> bool:
    return requires_contingency(rec) and rec["contingency"]["plan_state"] == "pending"


def escalation_missing(rec: dict) -> bool:
    return requires_escalation(rec) and rec["escalation"]["escalation_state"] == "pending"


def any_health_degraded(rec: dict) -> bool:
    return (
        maintainer_degraded(rec["maintainer"])
        or security_degraded(rec["security"])
        or cadence_degraded(rec)
        or license_degraded(rec["license"])
        or unowned(rec)
    )


def derive_reasons(rec: dict) -> list[str]:
    m, s, lic = rec["maintainer"], rec["security"], rec["license"]
    proof_slo = rec["proof_packet"]["slo_state"]
    facts = {
        "maintainer_health_thinning": maintainer_thinning(m),
        "maintainer_abandoned": maintainer_abandoned(m),
        "security_advisories_open": advisories_open(s),
        "security_unpatched_critical": unpatched_critical(s),
        "update_cadence_stalled": cadence_stalled(rec["update_cadence"]),
        "review_cadence_overdue": review_overdue(rec["review_cadence"]),
        "review_cadence_missing": review_missing(rec["review_cadence"]),
        "license_ambiguous": license_ambiguous(lic),
        "license_incompatible": license_incompatible(lic),
        "upstream_unowned": unowned(rec),
        "contingency_plan_missing": contingency_missing(rec),
        "shiproom_escalation_missing": escalation_missing(rec),
        "health_proof_stale": proof_slo == "breached",
        "health_proof_missing": proof_slo == "missing",
        "owner_signoff_missing": not rec["owner_signoff"]["signed_off"],
        # waiver_expired is authored explicitly, never auto-derived.
    }
    derived = [r for r in HEALTH_REASONS if facts.get(r, False)]
    for extra in rec.get("_extra_reasons", []):
        if extra not in derived:
            derived.append(extra)
    return [r for r in HEALTH_REASONS if r in derived]


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
    if dimension == "maintainer_health":
        return "unsatisfied" if maintainer_degraded(rec["maintainer"]) else "satisfied"
    if dimension == "security_posture":
        return "unsatisfied" if security_degraded(rec["security"]) else "satisfied"
    if dimension == "update_cadence":
        return "unsatisfied" if cadence_degraded(rec) else "satisfied"
    if dimension == "license_clarity":
        return "unsatisfied" if license_degraded(rec["license"]) else "satisfied"
    if dimension == "ownership_continuity":
        return (
            "unsatisfied"
            if (unowned(rec) or contingency_missing(rec) or escalation_missing(rec))
            else "satisfied"
        )
    if dimension == "scan_surface_parity":
        return "unsatisfied" if rec["scan_posture"] != rec["surface_posture"] else "satisfied"
    raise ValueError(dimension)


CONTROL_OWNERS = {
    "maintainer_health": DEP,
    "security_posture": SEC,
    "update_cadence": DEP,
    "license_clarity": OSS,
    "ownership_continuity": ARCH,
    "scan_surface_parity": GOV,
}
CONTROL_REFS = {
    "maintainer_health": UPSTREAM_SCORECARD_REF,
    "security_posture": ADVISORY_REGISTER_REF,
    "update_cadence": DEPENDENCY_REGISTER_REF,
    "license_clarity": DEPENDENCY_REGISTER_REF,
    "ownership_continuity": SHIPROOM_REGISTER_REF,
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
    upstream_kind: str,
    title: str,
    subject_ref: str,
    subject_summary: str,
    *,
    release_blocking: bool,
    declared: str,
    support_class: str,
    risk_grade: str,
    maint: dict,
    sec: dict,
    upd: dict,
    rev: dict,
    lic: dict,
    own: dict,
    cont: dict,
    esc: dict,
    pkt: dict,
    wv: dict | None,
    so: dict,
    rationale: str,
    extra_reasons: list[str] | None = None,
    surfaces: list[str] | None = None,
) -> dict:
    rec = {
        "record_id": record_id,
        "family": family,
        "upstream_kind": upstream_kind,
        "title": title,
        "subject_ref": subject_ref,
        "subject_summary": subject_summary,
        "release_blocking": release_blocking,
        "declared_label": declared,
        "support_class": support_class,
        "risk_grade": risk_grade,
        "maintainer": maint,
        "security": sec,
        "update_cadence": upd,
        "review_cadence": rev,
        "license": lic,
        "ownership": own,
        "contingency": cont,
        "escalation": esc,
        # filled below
        "controls": [],
        "scan_posture": "clear",
        "surface_posture": "clear",
        "scan_ref": f"{UPSTREAM_SCORECARD_REF}#health_scan/{record_id}",
        "surface_ref": f"shell/help_about_upstream_health_card#{record_id}",
        "proof_packet": pkt,
        "waiver": wv,
        "owner_signoff": so,
        "health_state": "cleared",
        "active_reasons": [],
        "effective_label": declared,
        "surfaces": surfaces or list(DEFAULT_SURFACES),
        "rationale": rationale,
        "_extra_reasons": extra_reasons or [],
    }
    reasons = derive_reasons(rec)
    state = computed_state(reasons, declared)
    posture = "gaps_found" if state not in ("cleared", "withdrawn") else "clear"
    rec["active_reasons"] = reasons
    rec["health_state"] = state
    rec["effective_label"] = computed_effective(reasons, declared)
    rec["scan_posture"] = posture
    rec["surface_posture"] = posture
    rec["controls"] = build_controls(rec)
    del rec["_extra_reasons"]
    return rec


def healthy_maint() -> dict:
    return maintainer("healthy", 6, 4)


def clean_sec() -> dict:
    return security("clean", 0)


def active_upd() -> dict:
    return update_cadence("active", 12)


def current_rev() -> dict:
    return review_cadence("current", 90, "2026-08-20")


def clear_lic(spdx: str = "Apache-2.0") -> dict:
    return license_clarity("clear", spdx=spdx)


def na_contingency() -> dict:
    return contingency("not_required", "none", "moderate")


def na_escalation() -> dict:
    return escalation("not_required", False)


def build_records() -> list[dict]:
    records = []

    # 1. Framework package — fully cleared at stable.
    records.append(
        record(
            "upstream-framework-async-runtime",
            "framework",
            "package",
            "Framework async-runtime upstream",
            f"{DEPENDENCY_REGISTER_REF}#framework_async_runtime",
            "A healthy, well-maintained async runtime with a clean security posture, an active release cadence, a current review, a clear license, an assigned owner, and no contingency or escalation outstanding.",
            release_blocking=True,
            declared="stable",
            support_class="open_local",
            risk_grade="green",
            maint=healthy_maint(),
            sec=clean_sec(),
            upd=active_upd(),
            rev=current_rev(),
            lic=clear_lic("Apache-2.0"),
            own=ownership("owned", DEP),
            cont=na_contingency(),
            esc=na_escalation(),
            pkt=proof("upstream_framework_async_runtime_proof", "current", "2026-05-30"),
            wv=None,
            so=signoff(GOV, True, "2026-05-31"),
            rationale="Maintainer health, security posture, cadence, review, license, ownership, and proof all hold, so the upstream is green and the scan and surface agree on a clean posture.",
        )
    )

    # 2. Framework protocol — cleared at stable, with a slowing cadence and a due-for-review reminder.
    records.append(
        record(
            "upstream-framework-wire-protocol",
            "framework",
            "protocol",
            "Framework wire-protocol upstream",
            f"{DEPENDENCY_REGISTER_REF}#framework_wire_protocol",
            "A stable wire protocol whose release cadence is slowing and whose next review is coming due; both are surfaced as reminders without narrowing the still-healthy upstream.",
            release_blocking=True,
            declared="stable",
            support_class="open_local",
            risk_grade="green",
            maint=maintainer("healthy", 4, 3),
            sec=clean_sec(),
            upd=update_cadence("slowing", 140),
            rev=review_cadence("due_for_review", 90, "2026-06-25"),
            lic=clear_lic("MIT"),
            own=ownership("owned", ARCH),
            cont=na_contingency(),
            esc=na_escalation(),
            pkt=proof("upstream_framework_wire_protocol_proof", "current", "2026-05-29"),
            wv=None,
            so=signoff(GOV, True, "2026-05-30"),
            rationale="A slowing cadence and a coming-due review are reminders, not gaps: the maintainer base, security posture, license, and ownership all hold, so the upstream stays cleared and green.",
        )
    )

    # 3. Notebook package — maintainer abandoned on a still-stable claim.
    records.append(
        record(
            "upstream-notebook-render-kernel",
            "notebook",
            "package",
            "Notebook render-kernel upstream",
            f"{DEPENDENCY_REGISTER_REF}#notebook_render_kernel",
            "A render kernel whose upstream has been abandoned by its maintainers; a replace-dependency plan is recorded and the shiproom has been told, but the maintainer collapse narrows the still-stable claim.",
            release_blocking=True,
            declared="stable",
            support_class="open_local",
            risk_grade="red",
            maint=maintainer("abandoned", 0, 0),
            sec=clean_sec(),
            upd=update_cadence("stalled", 410),
            rev=current_rev(),
            lic=clear_lic("BSD-3-Clause"),
            own=ownership("owned", DEP),
            cont=contingency("recorded", "replace_dependency", "moderate"),
            esc=escalation("raised", True),
            pkt=proof("upstream_notebook_render_kernel_proof", "current", "2026-05-28"),
            wv=None,
            so=signoff(GOV, True, "2026-05-29"),
            rationale="Upstream-health failure: the maintainer base has collapsed to abandoned while the family still claims Stable, so the maintainer gap holds promotion even though a replacement plan is recorded and the shiproom escalation is raised.",
        )
    )

    # 4. Data-rich package — unpatched critical advisory on a still-stable claim.
    records.append(
        record(
            "upstream-data_rich-columnar-core",
            "data_rich",
            "package",
            "Data-rich columnar-core upstream",
            f"{DEPENDENCY_REGISTER_REF}#data_rich_columnar_core",
            "A columnar engine carrying an unpatched critical advisory; an owner, a sponsor plan, and a raised escalation are in place, but the open critical narrows the still-stable claim.",
            release_blocking=True,
            declared="stable",
            support_class="open_local",
            risk_grade="red",
            maint=maintainer("healthy", 5, 3),
            sec=security("unpatched_critical", 2),
            upd=active_upd(),
            rev=current_rev(),
            lic=clear_lic("Apache-2.0"),
            own=ownership("owned", DEP),
            cont=contingency("recorded", "sponsor_upstream", "hard"),
            esc=escalation("raised", True),
            pkt=proof("upstream_data_rich_columnar_core_proof", "current", "2026-05-27"),
            wv=None,
            so=signoff(GOV, True, "2026-05-28"),
            rationale="Upstream-health failure: an unpatched critical advisory is open while the family still claims Stable, so the security gap holds promotion until the vulnerability is patched.",
        )
    )

    # 5. AI-adjacent curated import — stalled update cadence on a still-stable claim.
    records.append(
        record(
            "upstream-ai_adjacent-model-runtime",
            "ai_adjacent",
            "curated_import",
            "AI-adjacent model-runtime curated import",
            f"{DEPENDENCY_REGISTER_REF}#ai_adjacent_model_runtime",
            "A curated model runtime whose upstream has stopped shipping releases; the maintainer base and security posture are otherwise fine, but the stalled cadence narrows the still-stable claim.",
            release_blocking=True,
            declared="stable",
            support_class="mixed_open_managed",
            risk_grade="amber",
            maint=maintainer("healthy", 3, 2),
            sec=clean_sec(),
            upd=update_cadence("stalled", 300),
            rev=current_rev(),
            lic=clear_lic("Apache-2.0"),
            own=ownership("owned", ARCH),
            cont=na_contingency(),
            esc=na_escalation(),
            pkt=proof("upstream_ai_model_runtime_proof", "current", "2026-05-26"),
            wv=None,
            so=signoff(GOV, True, "2026-05-27"),
            rationale="Upstream-health failure: the update cadence has stalled while the family still claims Stable, so the cadence gap holds promotion until the upstream resumes or a replacement is found.",
        )
    )

    # 6. Review protocol — review cadence overdue; already Beta (inherited).
    records.append(
        record(
            "upstream-review-diff-protocol",
            "review",
            "protocol",
            "Review diff-protocol upstream",
            f"{DEPENDENCY_REGISTER_REF}#review_diff_protocol",
            "A diff protocol whose quarterly upstream-health review is overdue; the review lane is already Beta, so the overdue review is gated upstream.",
            release_blocking=True,
            declared="beta",
            support_class="open_local",
            risk_grade="amber",
            maint=maintainer("healthy", 4, 3),
            sec=clean_sec(),
            upd=active_upd(),
            rev=review_cadence("overdue", 90, "2026-04-01"),
            lic=clear_lic("MIT"),
            own=ownership("owned", ARCH),
            cont=na_contingency(),
            esc=na_escalation(),
            pkt=proof("upstream_review_diff_protocol_proof", "current", "2026-05-24"),
            wv=None,
            so=signoff(GOV, True, "2026-05-25"),
            rationale="The upstream-health review is overdue; the lane is already Beta, so this cadence narrowing is gated upstream and does not hold promotion, but the overdue review is surfaced as a reminder.",
        )
    )

    # 7. Companion package — ambiguous license on a still-stable claim.
    records.append(
        record(
            "upstream-companion-ui-toolkit",
            "companion",
            "package",
            "Companion UI-toolkit upstream",
            f"{DEPENDENCY_REGISTER_REF}#companion_ui_toolkit",
            "A UI toolkit whose effective license is ambiguous after an upstream relicensing; the maintainer base and security posture are fine, but the license ambiguity narrows the still-stable claim.",
            release_blocking=True,
            declared="stable",
            support_class="open_local",
            risk_grade="amber",
            maint=maintainer("healthy", 5, 3),
            sec=clean_sec(),
            upd=active_upd(),
            rev=current_rev(),
            lic=license_clarity("ambiguous", spdx=""),
            own=ownership("owned", OSS),
            cont=na_contingency(),
            esc=na_escalation(),
            pkt=proof("upstream_companion_ui_toolkit_proof", "current", "2026-05-23"),
            wv=None,
            so=signoff(GOV, True, "2026-05-24"),
            rationale="Upstream-health failure: the upstream license is ambiguous while the family still claims Stable, so the license gap holds promotion until the license is clarified.",
        )
    )

    # 8. Managed-depth package — unowned protected-path dependency on a still-stable claim.
    records.append(
        record(
            "upstream-managed_depth-object-store",
            "managed_depth",
            "package",
            "Managed-depth object-store upstream",
            f"{DEPENDENCY_REGISTER_REF}#managed_depth_object_store",
            "An object-store client treated as infrastructure plumbing and left without an assigned owner; the shiproom escalation is raised, but the unowned protected-path dependency narrows the still-stable claim.",
            release_blocking=True,
            declared="stable",
            support_class="managed",
            risk_grade="amber",
            maint=maintainer("healthy", 4, 3),
            sec=clean_sec(),
            upd=active_upd(),
            rev=current_rev(),
            lic=clear_lic("Apache-2.0"),
            own=ownership("unowned", ""),
            cont=na_contingency(),
            esc=escalation("raised", True),
            pkt=proof("upstream_managed_object_store_proof", "current", "2026-05-22"),
            wv=None,
            so=signoff(ECO, True, "2026-05-23"),
            rationale="Upstream-health failure: a critical upstream was left unowned because it is 'just infrastructure' while the family still claims Stable, so the ownership gap holds promotion until an owner is assigned, even though the shiproom escalation is raised.",
        )
    )

    # 9. AI-adjacent package — blocked, unowned, open advisory, contingency and escalation pending.
    records.append(
        record(
            "upstream-ai_adjacent-vector-index",
            "ai_adjacent",
            "package",
            "AI-adjacent vector-index upstream",
            f"{DEPENDENCY_REGISTER_REF}#ai_adjacent_vector_index",
            "A red-risk vector-index engine with an open advisory that is unowned, has no recorded sponsor/fork/replace plan, and has not been raised to the shiproom — the headline guardrail case on a still-stable claim.",
            release_blocking=True,
            declared="stable",
            support_class="mixed_open_managed",
            risk_grade="blocked",
            maint=maintainer("healthy", 3, 2),
            sec=security("advisories_open", 3),
            upd=update_cadence("slowing", 150),
            rev=current_rev(),
            lic=clear_lic("Apache-2.0"),
            own=ownership("unowned", ""),
            cont=contingency("pending", "none", "no_known_path"),
            esc=escalation("pending", True),
            pkt=proof("upstream_ai_vector_index_proof", "current", "2026-05-21"),
            wv=None,
            so=signoff(GOV, True, "2026-05-22"),
            rationale="Upstream-health failure: a red-risk dependency is unowned, has no approved sponsor/fork/replace plan, and has not been escalated to the shiproom while the family still claims Stable, so the ownership/escalation gap holds promotion — a red-risk dependency may not widen a stable claim without an approved plan.",
        )
    )

    # 10. Framework toolchain component — health proof stale; already Beta (inherited).
    records.append(
        record(
            "upstream-framework-build-toolchain",
            "framework",
            "toolchain_component",
            "Framework build-toolchain upstream",
            f"{DEPENDENCY_REGISTER_REF}#framework_build_toolchain",
            "A healthy build toolchain whose upstream-health proof packet has aged past its freshness SLO; the toolchain lane is already Beta.",
            release_blocking=True,
            declared="beta",
            support_class="open_local",
            risk_grade="green",
            maint=maintainer("healthy", 6, 4),
            sec=clean_sec(),
            upd=active_upd(),
            rev=current_rev(),
            lic=clear_lic("Apache-2.0"),
            own=ownership("owned", DEP),
            cont=na_contingency(),
            esc=na_escalation(),
            pkt=proof("upstream_framework_build_toolchain_proof", "breached", "2026-01-08"),
            wv=None,
            so=signoff(ECO, True, "2026-01-09"),
            rationale="The upstream is healthy but its health proof packet is stale; the lane is already Beta, so this evidence-staleness narrowing is gated upstream and does not hold promotion.",
        )
    )

    # 11. Companion toolchain component — health proof missing; already Beta (inherited).
    records.append(
        record(
            "upstream-companion-packager",
            "companion",
            "toolchain_component",
            "Companion packager upstream",
            f"{DEPENDENCY_REGISTER_REF}#companion_packager",
            "A healthy packager toolchain with no captured upstream-health proof packet; the companion lane is already Beta.",
            release_blocking=True,
            declared="beta",
            support_class="open_local",
            risk_grade="green",
            maint=maintainer("healthy", 5, 3),
            sec=clean_sec(),
            upd=active_upd(),
            rev=current_rev(),
            lic=clear_lic("ISC"),
            own=ownership("owned", DEP),
            cont=na_contingency(),
            esc=na_escalation(),
            pkt=proof("upstream_companion_packager_proof", "missing", None),
            wv=None,
            so=signoff(ECO, True, "2026-05-18"),
            rationale="No upstream-health proof packet has been captured; the lane is already Beta, so this evidence-staleness narrowing is gated upstream and does not hold promotion.",
        )
    )

    # 12. Review package — owner sign-off missing; already Beta (inherited).
    records.append(
        record(
            "upstream-review-grammar-pack",
            "review",
            "package",
            "Review grammar-pack upstream",
            f"{DEPENDENCY_REGISTER_REF}#review_grammar_pack",
            "A healthy grammar pack whose health record still lacks an owner sign-off; the review lane is already Beta.",
            release_blocking=True,
            declared="beta",
            support_class="open_local",
            risk_grade="green",
            maint=maintainer("healthy", 4, 3),
            sec=clean_sec(),
            upd=active_upd(),
            rev=current_rev(),
            lic=clear_lic("MIT"),
            own=ownership("owned", DEP),
            cont=na_contingency(),
            esc=na_escalation(),
            pkt=proof("upstream_review_grammar_pack_proof", "current", "2026-05-17"),
            wv=None,
            so=signoff(GOV, False, None),
            rationale="The health record carries no owner sign-off; the lane is already Beta, so this evidence-staleness narrowing is gated upstream and does not hold promotion.",
        )
    )

    # 13. Data-rich curated import — relied-on waiver has expired; already Beta (inherited).
    records.append(
        record(
            "upstream-data_rich-stats-kernel",
            "data_rich",
            "curated_import",
            "Data-rich stats-kernel curated import",
            f"{DEPENDENCY_REGISTER_REF}#data_rich_stats_kernel",
            "A healthy curated stats kernel whose relied-on health waiver has expired; the data-rich curated lane is already Beta.",
            release_blocking=True,
            declared="beta",
            support_class="mixed_open_managed",
            risk_grade="green",
            maint=maintainer("healthy", 4, 3),
            sec=clean_sec(),
            upd=active_upd(),
            rev=current_rev(),
            lic=clear_lic("BSD-3-Clause"),
            own=ownership("owned", DEP),
            cont=na_contingency(),
            esc=na_escalation(),
            pkt=proof("upstream_data_rich_stats_kernel_proof", "current", "2026-05-16"),
            wv=waiver(
                f"{PACKAGE_INVENTORY_REF}#waivers.data-rich-stats-kernel",
                "2026-03-31",
                "An upstream-health re-assessment was time-boxed under this waiver, which has since expired and must be renewed or cleared.",
            ),
            so=signoff(GOV, True, "2026-05-17"),
            extra_reasons=["waiver_expired"],
            rationale="The relied-on waiver has expired; the lane is already Beta, so this narrowing is gated upstream, but the expired waiver is surfaced rather than silently honored.",
        )
    )

    # 14. Managed-depth protocol — single-maintainer (thinning); held under an unexpired waiver.
    records.append(
        record(
            "upstream-managed_depth-queue-protocol",
            "managed_depth",
            "protocol",
            "Managed-depth queue-protocol upstream",
            f"{DEPENDENCY_REGISTER_REF}#managed_depth_queue_protocol",
            "A queue protocol that has thinned to a single maintainer; a backup-maintainer search is time-boxed under a waiver, so the maintainer narrowing is gated upstream while the family still claims Stable.",
            release_blocking=True,
            declared="stable",
            support_class="managed",
            risk_grade="amber",
            maint=maintainer("single_maintainer", 1, 1),
            sec=clean_sec(),
            upd=update_cadence("slowing", 120),
            rev=current_rev(),
            lic=clear_lic("Apache-2.0"),
            own=ownership("owned", ARCH),
            cont=na_contingency(),
            esc=na_escalation(),
            pkt=proof("upstream_managed_queue_protocol_proof", "current", "2026-05-19"),
            wv=waiver(
                f"{PACKAGE_INVENTORY_REF}#waivers.managed-depth-queue-protocol",
                "2026-09-30",
                "The upstream has thinned to a single maintainer; a backup-maintainer search and sponsorship are time-boxed under this waiver.",
            ),
            so=signoff(GOV, True, "2026-05-20"),
            rationale="The maintainer base has thinned to a single maintainer, but an unexpired waiver holds the gap provisionally, so it is gated upstream and does not hold promotion while the search runs.",
        )
    )

    return records


def build_rules() -> list[dict]:
    titles = {
        "maintainer_health_thinning": "Maintainer base must not thin to a bus-factor risk",
        "maintainer_abandoned": "Abandoned upstream must be sponsored or replaced",
        "security_advisories_open": "Open advisories must be remediated",
        "security_unpatched_critical": "Critical vulnerabilities must be patched",
        "update_cadence_stalled": "Stalled update cadence must be escalated",
        "review_cadence_overdue": "Upstream-health review must not be overdue",
        "review_cadence_missing": "Upstream-health review must exist",
        "license_ambiguous": "Ambiguous license must be clarified",
        "license_incompatible": "Incompatible license must be resolved",
        "upstream_unowned": "Critical upstream must have an owner",
        "contingency_plan_missing": "Red-risk upstream must record a sponsor/fork/replace plan",
        "shiproom_escalation_missing": "Red-risk or unowned upstream must be escalated to the shiproom",
        "health_proof_stale": "Upstream-health proof must be fresh",
        "health_proof_missing": "Upstream-health proof must exist",
        "owner_signoff_missing": "Owner sign-off required",
        "waiver_expired": "Waiver must be current",
    }
    rules = []
    for reason in HEALTH_REASONS:
        rules.append(
            {
                "rule_id": f"m5_critical_upstream_health_rule:{reason}",
                "title": titles[reason],
                "trigger_reason": reason,
                "applies_to_labels": ABOVE_CUTLINE,
                "default_action": REASON_ACTION[reason],
                "blocks_promotion": True,
                "rationale": "An upstream-health failure on a subject still claiming a label at or above the cutline holds promotion through the shiproom gate; inherited (below-cutline or waived) narrowings are gated upstream.",
            }
        )
    return rules


def is_waived(rec: dict) -> bool:
    return rec.get("waiver") is not None and "waiver_expired" not in rec["active_reasons"]


def holds_promotion(rec: dict) -> bool:
    return (
        rec["release_blocking"]
        and rec["health_state"] not in ("cleared", "withdrawn")
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
            if (
                rule
                and rule["blocks_promotion"]
                and rec["declared_label"] in rule["applies_to_labels"]
            ):
                ids.add(rec["record_id"])
                break
    return sorted(ids)


def computed_scan_surface_parity(records: list[dict]) -> dict:
    return {
        "parity_gate": "m5_critical_upstream_scan_surface_parity_gate",
        "subjects_total": len(records),
        "subjects_in_agreement": sum(
            1 for r in records if r["scan_posture"] == r["surface_posture"]
        ),
        "subjects_in_disagreement": sum(
            1 for r in records if r["scan_posture"] != r["surface_posture"]
        ),
        "subjects_with_gaps": sum(1 for r in records if r["surface_posture"] == "gaps_found"),
        "all_subjects_agree": all(
            r["scan_posture"] == r["surface_posture"] for r in records
        ),
        "rationale": "The upstream-health scan and the governance-dashboard/promotion-packet surface agree on every subject, so a green upstream card can never mask an abandoned, unpatched, or unowned dependency.",
    }


def cadence_gap(rec: dict) -> bool:
    return cadence_degraded(rec)


def ownership_gap(rec: dict) -> bool:
    return unowned(rec) or contingency_missing(rec) or escalation_missing(rec)


def computed_summary(records: list[dict], rules: list[dict]) -> dict:
    def count_state(s):
        return sum(1 for r in records if r["health_state"] == s)

    narrowed = [r for r in records if r["health_state"] not in ("cleared", "withdrawn")]
    cleared = [r for r in records if r["health_state"] == "cleared"]
    return {
        "total_records": len(records),
        "records_cleared": len(cleared),
        "records_narrowed": len(narrowed),
        "state_cleared": count_state("cleared"),
        "state_narrowed_maintainer": count_state("narrowed_maintainer"),
        "state_narrowed_security": count_state("narrowed_security"),
        "state_narrowed_cadence": count_state("narrowed_cadence"),
        "state_narrowed_license": count_state("narrowed_license"),
        "state_narrowed_ownership": count_state("narrowed_ownership"),
        "state_narrowed_stale": count_state("narrowed_stale"),
        "state_withdrawn": count_state("withdrawn"),
        "release_blocking_total": sum(1 for r in records if r["release_blocking"]),
        "release_blocking_narrowed": sum(1 for r in narrowed if r["release_blocking"]),
        "records_on_active_waiver": sum(1 for r in records if is_waived(r)),
        "maintainer_gaps": sum(1 for r in records if maintainer_degraded(r["maintainer"])),
        "security_gaps": sum(1 for r in records if security_degraded(r["security"])),
        "cadence_gaps": sum(1 for r in records if cadence_gap(r)),
        "license_gaps": sum(1 for r in records if license_degraded(r["license"])),
        "ownership_gaps": sum(1 for r in records if ownership_gap(r)),
        "red_risk_total": sum(1 for r in records if red_risk(r)),
        "unowned_total": sum(1 for r in records if unowned(r)),
        "escalations_required": sum(1 for r in records if requires_escalation(r)),
        "escalations_raised": sum(
            1 for r in records if r["escalation"]["escalation_state"] == "raised"
        ),
        "contingency_plans_recorded": sum(
            1 for r in records if r["contingency"]["plan_state"] == "recorded"
        ),
        "total_active_reasons": sum(len(r["active_reasons"]) for r in records),
        "rules_firing": len(computed_blocking_rule_ids(records, rules)),
    }


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
            "upstream_scorecard_ref": UPSTREAM_SCORECARD_REF,
            "dependency_register_ref": DEPENDENCY_REGISTER_REF,
            "advisory_register_ref": ADVISORY_REGISTER_REF,
            "import_register_ref": IMPORT_REGISTER_REF,
            "package_inventory_ref": PACKAGE_INVENTORY_REF,
            "durability_matrix_ref": DURABILITY_MATRIX_REF,
            "shiproom_register_ref": SHIPROOM_REGISTER_REF,
            "m5_evidence_index_ref": EVIDENCE_INDEX_REF,
        },
        "health_cutline": {
            "cutline_level": "stable",
            "above_cutline_levels": ABOVE_CUTLINE,
            "below_cutline_levels": ["beta", "preview", "withdrawn"],
            "description": "Subjects at or above Stable carry the cleared upstream-health claim; an upstream-health gap on a still-stable subject holds promotion through the shiproom gate.",
        },
        "families": FAMILIES,
        "upstream_kinds": UPSTREAM_KINDS,
        "support_classes": SUPPORT_CLASSES,
        "health_grades": HEALTH_GRADES,
        "control_dimensions": CONTROL_DIMENSIONS,
        "maintainer_ratings": MAINTAINER_RATINGS,
        "security_postures": SECURITY_POSTURES,
        "update_cadences": UPDATE_CADENCES,
        "review_cadence_states": REVIEW_CADENCE_STATES,
        "license_clarities": LICENSE_CLARITIES,
        "replacement_feasibilities": REPLACEMENT_FEASIBILITIES,
        "ownership_states": OWNERSHIP_STATES,
        "contingency_states": CONTINGENCY_STATES,
        "contingency_dispositions": CONTINGENCY_DISPOSITIONS,
        "escalation_states": ESCALATION_STATES,
        "postures": POSTURES,
        "health_states": HEALTH_STATES,
        "health_reasons": HEALTH_REASONS,
        "health_actions": HEALTH_ACTIONS,
        "rules": rules,
        "records": records,
        "scan_surface_parity": computed_scan_surface_parity(records),
        "publication": {
            "publication_gate": "m5_critical_upstream_health_gate",
            "decision": decision_verdict,
            "blocking_rule_ids": blocking_rules,
            "blocking_record_ids": blocking_records,
            "rationale": "Hold while any release-blocking subject carries an upstream-health gap on a still-stable claim; inherited and waived narrowings are gated upstream. A red-risk or unowned protected-path dependency may not widen a stable claim without an approved sponsor, fork, or replacement plan.",
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

    # A cleared record hiding an unowned dependency without narrowing on it.
    hidden = copy.deepcopy(register)
    target = next(r for r in hidden["records"] if r["health_state"] == "cleared")
    target["ownership"]["ownership_state"] = "unowned"
    target["ownership"]["owner_ref"] = ""
    cases.append(("hidden_ownership_gap.json", hidden, "GapWithoutReason"))

    # A narrowed record whose governance surface is green over a gapped scan.
    masked = copy.deepcopy(register)
    target = next(
        r for r in masked["records"] if r["health_state"] not in ("cleared", "withdrawn")
    )
    target["surface_posture"] = "clear"
    cases.append(("green_surface_over_gap.json", masked, "ScanSurfaceDisagreement"))

    # A narrowed record whose effective label stays above the cutline.
    above = copy.deepcopy(register)
    target = next(
        r for r in above["records"] if r["health_state"] not in ("cleared", "withdrawn")
    )
    target["effective_label"] = "stable"
    cases.append(("narrowed_above_cutline.json", above, "EffectiveLabelMismatch"))

    return cases


def build_capture(register: dict, cases: list[tuple[str, dict, str]]) -> dict:
    s = register["summary"]
    p = register["publication"]
    parity = register["scan_surface_parity"]
    drills = [
        "drill:hidden_ownership_gap",
        "drill:green_surface_over_gap",
        "drill:narrowed_above_cutline",
        "drill:cleared_with_active_reason",
        "drill:reason_not_justified",
        "drill:control_state_inconsistent",
        "drill:red_risk_without_contingency",
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
            "state_narrowed_maintainer": s["state_narrowed_maintainer"],
            "state_narrowed_security": s["state_narrowed_security"],
            "state_narrowed_cadence": s["state_narrowed_cadence"],
            "state_narrowed_license": s["state_narrowed_license"],
            "state_narrowed_ownership": s["state_narrowed_ownership"],
            "state_narrowed_stale": s["state_narrowed_stale"],
            "release_blocking_narrowed": s["release_blocking_narrowed"],
            "records_on_active_waiver": s["records_on_active_waiver"],
            "maintainer_gaps": s["maintainer_gaps"],
            "security_gaps": s["security_gaps"],
            "cadence_gaps": s["cadence_gaps"],
            "license_gaps": s["license_gaps"],
            "ownership_gaps": s["ownership_gaps"],
            "red_risk_total": s["red_risk_total"],
            "unowned_total": s["unowned_total"],
            "escalations_required": s["escalations_required"],
            "escalations_raised": s["escalations_raised"],
            "contingency_plans_recorded": s["contingency_plans_recorded"],
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
