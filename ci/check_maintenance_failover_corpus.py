#!/usr/bin/env python3
"""Validate the maintenance-window & failover communication corpus.

This is the release-engineering / public-proof audit lane that proves the
marketed managed and hybrid beta rows keep their maintenance, drain / read-only,
failover, and tenant / region migration *communication* honest under real
service change. It is the proof packet a release or support team attaches to a
claimed beta row: one current maintenance/failover packet per row that a support
engineer can read and explain without rehydrating live control-plane state.

It composes — it does not fork — the `continuity_notice_view_record` model the
desktop shell, activity center / durable history, CLI / headless inspect,
diagnostics, and support exports all read (see
crates/aureline-shell/src/continuity_notices/model.rs and
schemas/ops/continuity_notice_view.schema.json), and adds the communication
drills this lane requires:

- **Timezone mismatch.** A scheduled window declared in a non-UTC zone always
  renders its absolute UTC instant alongside the IANA zone and the offset in
  force, so an operator in another zone cannot misread it as a naive local time.
- **Stale maintenance card.** A maintenance notice whose last refresh aged out
  downgrades, names the reason, lights the honesty marker, and carries a stale
  label — it never keeps reading as current.
- **Read-only / drain window.** A drain lets existing sessions finish while new
  writes queue for publish-later or are held as local drafts, kept visibly
  separate from hosted mutations that already landed.
- **Changed tenant / region / endpoint.** A failover that changed a boundary
  axis keeps that change visible behind the recovered messaging, with a
  canonical current ref.
- **Queued publish-later work.** Preserved intent carries a canonical queue /
  intent ref and survives the window without collapsing into successful work.
- **Post-window reconciliation under changed authority.** When a reconciliation
  changed the policy / target identity, queued intent is held for a boundary
  recheck — it never silently replays across the new authority boundary.

For **every** packet the validator schema-validates the record against
`schemas/ops/continuity_notice_view.schema.json`, then rebuilds the record from
its extracted inputs with an independent port of the model and asserts the stored
record matches (a second implementation, so a regression in either the model or a
fixture fails the lane). It then runs the explicit lane-failing rules, proves the
corpus exercises the required drills, and drift-checks the enum-only matrix and
the export-parity packet that prove UI / CLI / support-bundle agreement and that
every claimed beta row maps to exactly one packet.

Run via scripts/ci/run_maintenance_failover_corpus.sh. Use --write to (re)mint
the drill fixtures, the matrix, and the parity packet from the model.
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

SCHEMA_REL = "schemas/ops/continuity_notice_view.schema.json"
CORPUS_DIR_REL = "fixtures/ops/m3/maintenance_failover_corpus"
CORPUS_MATRIX_REL = f"{CORPUS_DIR_REL}/corpus_matrix.json"
PARITY_PACKET_REL = f"{CORPUS_DIR_REL}/export_parity_packet.json"
README_REL = f"{CORPUS_DIR_REL}/README.md"
REPORT_REL = "artifacts/ops/m3/maintenance_failover_report.md"
VALIDATOR_REL = "ci/check_maintenance_failover_corpus.py"
SCRIPT_REL = "scripts/ci/run_maintenance_failover_corpus.sh"
CONTRACT_DOC_REL = "docs/ops/m3/maintenance_failover_truth.md"

AUDIT_CONTRACT_REF = "ops:maintenance_failover_corpus:v1"
MATRIX_RECORD_KIND = "maintenance_failover_corpus_matrix"
PARITY_RECORD_KIND = "maintenance_failover_export_parity_packet"

VIEW_RECORD_KIND = "continuity_notice_view_record"
VIEW_SCHEMA_VERSION = 1

NON_VIEW_FILES = {"corpus_matrix.json", "export_parity_packet.json", "README.md"}

NOTICE = (
    "Maintenance & failover truth: every notice declares whether it is a "
    "maintenance, drain, failover, or tenant-migration window, the exact window "
    "time / timezone / offset, the affected deployment scope and write classes, "
    "which queued publish-later or local-draft work survives (kept separate from "
    "successful hosted mutations), what stays local-safe, and any changed tenant "
    "/ region / endpoint boundary. A notice reads as current only while it is "
    "active and its last refresh is current — otherwise it downgrades and names "
    "why, and never collapses into a generic degraded banner. Shell, activity "
    "history, CLI / headless inspect, diagnostics, and support exports read this "
    "record verbatim."
)

# --------------------------------------------------------------------------- #
# Closed vocabularies + labels — a deliberate second copy of the Rust enums and
# their display labels. Keep in lockstep with
# crates/aureline-shell/src/continuity_notices/model.rs.
# --------------------------------------------------------------------------- #

NOTICE_KIND_LABEL = {
    "scheduled_maintenance_window": "Scheduled maintenance window",
    "read_only_window": "Read-only window",
    "drain_window": "Drain window",
    "scheduled_export_freeze": "Scheduled export freeze",
    "tenant_migration": "Tenant migration",
    "region_migration": "Region migration",
    "regional_failover": "Regional failover",
    "control_plane_failover": "Control-plane failover",
    "post_event_reconciliation": "Post-event reconciliation",
}
CATEGORY_LABEL = {
    "maintenance": "Maintenance",
    "drain": "Drain / read-only",
    "failover": "Failover",
    "tenant_migration": "Tenant migration",
}
CATEGORY_FOR_KIND = {
    "scheduled_maintenance_window": "maintenance",
    "scheduled_export_freeze": "maintenance",
    "post_event_reconciliation": "maintenance",
    "read_only_window": "drain",
    "drain_window": "drain",
    "regional_failover": "failover",
    "control_plane_failover": "failover",
    "tenant_migration": "tenant_migration",
    "region_migration": "tenant_migration",
}
PLAN_FOR_KIND = {
    k: ("emergency" if k in {"regional_failover", "control_plane_failover"} else "planned")
    for k in NOTICE_KIND_LABEL
}
PLAN_LABEL = {"planned": "Planned", "emergency": "Emergency"}

FRESHNESS_LABEL = {
    "active_current": "Active (current)",
    "superseded_stale": "Superseded",
    "completed_historical": "Completed (history)",
    "imported_historical": "Imported (history)",
}
EFFECTIVE_LABEL = {
    "current": "Current",
    "refresh_stale": "Stale — last refresh aged out",
    "superseded_stale": "Superseded — newer notice exists",
    "completed_historical": "Completed — historical record",
    "imported_historical": "Imported — offline history",
}
REFRESH_AGE_LABEL = {
    "fresh": "Just now",
    "recent": "Within the hour",
    "stale": "Hours ago",
    "very_stale": "More than a day ago",
    "never": "No refresh recorded",
}
POSTURE_LABEL = {
    "queued_publish_later": "Queued for publish-later",
    "local_draft_preserved": "Saved as local draft",
    "retryable_when_connected": "Retryable when resumed",
    "draining_existing_only": "Draining existing only",
    "blocked_pending_reconnect": "Blocked pending reconnect",
    "blocked_pending_boundary_recheck": "Blocked pending boundary review",
    "blocked_no_safe_retry": "Blocked — no safe retry",
    "requires_manual_rerun": "Requires manual rerun",
}
GUIDANCE_LABEL = {
    "retry_safe_when_resumed": "Retry is safe once resumed",
    "export_now_safer": "Export now — safer than retrying",
    "postpone_safer": "Postpone — safer than retrying now",
    "manual_rerun_required": "Manual rerun required",
    "no_safe_retry_escalate": "No safe retry — escalate",
}
AXIS_LABEL = {
    "tenant": "Tenant",
    "region": "Region",
    "residency": "Residency",
    "key_ownership": "Key ownership",
    "endpoint_identity": "Endpoint identity",
}
AXIS_STATE_LABEL = {
    "unchanged": "Unchanged",
    "changed": "Changed",
    "unknown_recheck_required": "Unknown — recheck required",
    "not_applicable": "Not applicable",
}
LOCAL_CORE_LABEL = {
    "local_core_unaffected": "Local core unaffected",
    "meaningful_safe_subset_available": "Meaningful local subset available",
    "local_only_available": "Local-only available",
    "no_safe_local_subset": "No safe local subset",
    "unknown_requires_review": "Unknown — requires review",
}

EFFECTIVE = set(EFFECTIVE_LABEL)
PRESERVED_POSTURES = {"queued_publish_later", "local_draft_preserved"}
# Postures that auto-replay queued work without a fresh authority check.
AUTO_REPLAY_POSTURES = {"queued_publish_later", "retryable_when_connected"}
# Resume triggers that imply the work crosses a changed authority boundary and
# therefore demands a boundary recheck / fresh approval before it may resume.
BOUNDARY_RECHECK_TRIGGERS = {"boundary_review_completed", "fresh_approval_issued"}
MEANINGFUL_AXIS_STATES = {"changed", "unknown_recheck_required"}
AUTHORITY_AXES = {"tenant", "region", "residency", "key_ownership", "endpoint_identity"}

CANONICAL_SCHEME = "aureline://"
GENERIC_LANDING_CLASSES = {
    "home", "dashboard", "landing", "index", "overview", "start", "root",
}


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
# Timestamp / refresh-age + freshness derivation (port of model.rs)
# --------------------------------------------------------------------------- #


def days_from_civil(y: int, m: int, d: int) -> int:
    y = y - 1 if m <= 2 else y
    era = (y if y >= 0 else y - 399) // 400
    yoe = y - era * 400
    doy = (153 * (m - 3 if m > 2 else m + 9) + 2) // 5 + d - 1
    doe = yoe * 365 + yoe // 4 - yoe // 100 + doy
    return era * 146_097 + doe - 719_468


def parse_timestamp_minutes(value: str | None) -> int | None:
    if value is None or len(value) < 16:
        return None
    if value[4] != "-" or value[7] != "-":
        return None
    if value[10] not in ("T", " "):
        return None
    if value[13] != ":":
        return None
    try:
        year = int(value[0:4])
        month = int(value[5:7])
        day = int(value[8:10])
        hour = int(value[11:13])
        minute = int(value[14:16])
    except ValueError:
        return None
    if not (1 <= month <= 12) or not (1 <= day <= 31):
        return None
    if hour > 23 or minute > 59:
        return None
    return days_from_civil(year, month, day) * 24 * 60 + hour * 60 + minute


def derive_refresh_age(refresh_at: str | None, as_of: str) -> str:
    last = parse_timestamp_minutes(refresh_at)
    now = parse_timestamp_minutes(as_of)
    if last is None or now is None or now < last:
        return "never"
    delta = now - last
    if delta <= 5:
        return "fresh"
    if delta <= 60:
        return "recent"
    if delta <= 60 * 24:
        return "stale"
    return "very_stale"


def derive_effective_freshness(declared: str, refresh_age: str) -> tuple[str, list[str]]:
    if declared == "active_current":
        if refresh_age in ("fresh", "recent"):
            return "current", []
        return "refresh_stale", ["refresh_expired"]
    if declared == "superseded_stale":
        return "superseded_stale", ["notice_superseded"]
    if declared == "completed_historical":
        return "completed_historical", ["window_completed"]
    if declared == "imported_historical":
        return "imported_historical", ["imported_offline"]
    return declared, []


def is_canonical_ref(ref: str | None) -> bool:
    ref = (ref or "").strip()
    if not ref or len(ref) > 200 or not ref.startswith(CANONICAL_SCHEME):
        return False
    rest = ref[len(CANONICAL_SCHEME):]
    if "/" not in rest:
        return False
    cls, ident = rest.split("/", 1)
    if not cls or not ident:
        return False
    return cls not in GENERIC_LANDING_CLASSES


# --------------------------------------------------------------------------- #
# Drill specification + builder (port of model.rs ContinuityNoticeView::build)
# --------------------------------------------------------------------------- #

AS_OF = "2026-05-20T12:00:00Z"
FRESH = "2026-05-20T11:58:00Z"   # 2 min   -> fresh
RECENT = "2026-05-20T11:20:00Z"  # 40 min  -> recent
HOURS_AGO = "2026-05-20T06:00:00Z"  # 6 h   -> stale


def axis(axis_class, state, current_ref, summary, previous_ref=None):
    return {
        "axis_class": axis_class,
        "axis_state_class": state,
        "previous_ref": previous_ref,
        "current_ref": current_ref,
        "summary": summary,
    }


def blocked(action, block_state, posture, guidance, resume, note,
            queue_or_intent_ref=None, idempotency_key_present=False):
    return {
        "action_class": action,
        "block_state_class": block_state,
        "continuity_posture": posture,
        "safer_guidance": guidance,
        "queue_or_intent_ref": queue_or_intent_ref,
        "idempotency_key_present": idempotency_key_present,
        "resume_trigger": resume,
        "note": note,
    }


def succeeded(action, result_ref, completed_at, note):
    return {
        "action_class": action,
        "result_ref": result_ref,
        "completed_at": completed_at,
        "note": note,
    }


@dataclass
class Drill:
    scenario_id: str
    fixture_filename: str
    claimed_beta_row: str
    notice_id: str
    notice_kind: str
    title: str
    summary: str
    created_at: str
    updated_at: str
    schedule_in: dict[str, Any]
    scope_in: dict[str, Any]
    boundary_in: dict[str, Any]
    blocked_writes_in: list[dict[str, Any]]
    succeeded_in: list[dict[str, Any]]
    local_in: dict[str, Any]
    lifecycle_in: dict[str, Any]
    display_in: dict[str, Any]
    stale_label: str | None
    history_ref: str
    support_export_ref: str
    evidence_refs: list[str]
    as_of: str = AS_OF

    def view(self) -> dict[str, Any]:
        return build_view(self)


def build_view(d: Drill) -> dict[str, Any]:
    kind = d.notice_kind
    category = CATEGORY_FOR_KIND[kind]
    plan = PLAN_FOR_KIND[kind]

    sin = d.schedule_in
    refresh_at = sin.get("latest_refresh_at")
    refresh_age = derive_refresh_age(refresh_at, d.as_of)
    schedule = {
        "time_basis": sin["time_basis"],
        "time_basis_label": sin["time_basis"],
        "starts_at": sin["starts_at"],
        "expected_or_actual_ends_at": sin.get("expected_or_actual_ends_at"),
        "completed_at": sin.get("completed_at"),
        "timezone_id": sin["timezone_id"],
        "utc_offset_at_start": sin["utc_offset_at_start"],
        "latest_refresh_at": refresh_at,
        "refresh_age": refresh_age,
        "refresh_age_label": REFRESH_AGE_LABEL[refresh_age],
    }

    declared = d.lifecycle_in["freshness_class"]
    eff, reasons = derive_effective_freshness(declared, refresh_age)
    downgraded = eff != "current"

    axes = []
    changed = unknown = 0
    for a in d.boundary_in["axes"]:
        st = a["axis_state_class"]
        if st == "changed":
            changed += 1
        elif st == "unknown_recheck_required":
            unknown += 1
        axes.append(
            {
                "axis_class": a["axis_class"],
                "axis_label": AXIS_LABEL[a["axis_class"]],
                "axis_state_class": st,
                "axis_state_label": AXIS_STATE_LABEL[st],
                "previous_ref": a.get("previous_ref"),
                "current_ref": a.get("current_ref"),
                "summary": a["summary"],
            }
        )
    required = bool(d.boundary_in["boundary_change_required"])
    reviewed = bool(d.boundary_in["review_completed"])
    unresolved = required and not reviewed and (changed + unknown > 0)
    boundary_change = {
        "boundary_change_required": required,
        "review_completed": reviewed,
        "boundary_change_unresolved": unresolved,
        "changed_axis_count": changed,
        "unknown_axis_count": unknown,
        "axes": axes,
        "summary": d.boundary_in["summary"],
    }

    blocked_writes = []
    queued = drafts = no_retry = manual = 0
    for w in d.blocked_writes_in:
        posture = w["continuity_posture"]
        preserved = posture in PRESERVED_POSTURES
        if posture == "queued_publish_later":
            queued += 1
        elif posture == "local_draft_preserved":
            drafts += 1
        elif posture == "blocked_no_safe_retry":
            no_retry += 1
        elif posture == "requires_manual_rerun":
            manual += 1
        blocked_writes.append(
            {
                "action_class": w["action_class"],
                "block_state_class": w["block_state_class"],
                "continuity_posture": posture,
                "continuity_posture_label": POSTURE_LABEL[posture],
                "safer_guidance": w["safer_guidance"],
                "safer_guidance_label": GUIDANCE_LABEL[w["safer_guidance"]],
                "queue_or_intent_ref": w.get("queue_or_intent_ref"),
                "idempotency_key_present": bool(w.get("idempotency_key_present")),
                "intent_preserved": preserved,
                "resume_trigger": w["resume_trigger"],
                "note": w["note"],
            }
        )

    summary_counts = {
        "blocked_write_count": len(blocked_writes),
        "queued_publish_later_count": queued,
        "local_draft_preserved_count": drafts,
        "preserved_intent_count": queued + drafts,
        "blocked_no_safe_retry_count": no_retry,
        "requires_manual_rerun_count": manual,
        "succeeded_hosted_mutation_count": len(d.succeeded_in),
        "changed_boundary_axis_count": changed,
        "unknown_boundary_axis_count": unknown,
    }

    honesty = downgraded or unresolved
    stale_label = d.stale_label if downgraded else None
    if downgraded and not stale_label:
        raise ValueError(f"{d.scenario_id}: downgraded drill must author a stale_label")

    di = d.display_in
    display_copy = {
        "primary_status_line": di["primary_status_line"],
        "schedule_line": di["schedule_line"],
        "scope_line": di["scope_line"],
        "blocked_writes_line": di["blocked_writes_line"],
        "queued_preserved_line": di["queued_preserved_line"],
        "succeeded_line": di["succeeded_line"],
        "local_continuity_line": di["local_continuity_line"],
        "boundary_change_line": di["boundary_change_line"],
        "freshness_line": di["freshness_line"],
        "follow_up_line": di["follow_up_line"],
        "stale_label": stale_label,
        "all_work_broken_implied": False,
        "incident_language_for_planned_used": False,
        "generic_degraded_banner_used": False,
        "queued_and_succeeded_collapsed": False,
        "stale_presented_as_current": False,
        "boundary_change_hidden": False,
    }

    return {
        "record_kind": VIEW_RECORD_KIND,
        "schema_version": VIEW_SCHEMA_VERSION,
        "notice": NOTICE,
        "view_id": f"continuity_notice_view:m3.beta.maintenance_failover_corpus.{d.scenario_id}",
        "notice_id": d.notice_id,
        "as_of": d.as_of,
        "notice_kind": kind,
        "notice_kind_label": NOTICE_KIND_LABEL[kind],
        "category": category,
        "category_label": CATEGORY_LABEL[category],
        "plan_class": plan,
        "plan_class_label": PLAN_LABEL[plan],
        "title": d.title,
        "summary": d.summary,
        "created_at": d.created_at,
        "updated_at": d.updated_at,
        "schedule": schedule,
        "affected_scope": {
            "deployment_profiles": list(d.scope_in["deployment_profiles"]),
            "tenant_refs": list(d.scope_in.get("tenant_refs", [])),
            "region_refs": list(d.scope_in.get("region_refs", [])),
            "residency_scope_classes": list(d.scope_in["residency_scope_classes"]),
            "service_classes": list(d.scope_in["service_classes"]),
            "scope_summary": d.scope_in["scope_summary"],
        },
        "boundary_change": boundary_change,
        "blocked_writes": blocked_writes,
        "succeeded_hosted_mutations": [dict(m) for m in d.succeeded_in],
        "local_continuity": {
            "local_core_status": d.local_in["local_core_status"],
            "local_core_status_label": LOCAL_CORE_LABEL[d.local_in["local_core_status"]],
            "retained_local_safe_capabilities": list(
                d.local_in["retained_local_safe_capabilities"]
            ),
            "continue_local_guidance_required": bool(
                d.local_in["continue_local_guidance_required"]
            ),
            "continuity_summary": d.local_in["continuity_summary"],
        },
        "lifecycle": {
            "freshness_class": declared,
            "freshness_label": FRESHNESS_LABEL[declared],
            "supersedes_id": d.lifecycle_in.get("supersedes_id"),
            "superseded_by_id": d.lifecycle_in.get("superseded_by_id"),
            "retained_until_at": d.lifecycle_in.get("retained_until_at"),
            "history_refs": list(d.lifecycle_in.get("history_refs", [])),
        },
        "effective_freshness": eff,
        "effective_freshness_label": EFFECTIVE_LABEL[eff],
        "freshness_downgraded": downgraded,
        "downgrade_reasons": reasons,
        "boundary_change_unresolved": unresolved,
        "honesty_marker_present": honesty,
        "summary_counts": summary_counts,
        "display_copy": display_copy,
        "history_ref": d.history_ref,
        "support_export_ref": d.support_export_ref,
        "evidence_refs": list(d.evidence_refs),
        "narrative_refs": [CONTRACT_DOC_REL],
    }


def extract_inputs(view: dict[str, Any]) -> Drill:
    """Recover the drill spec from a stored record so build can reproduce it."""
    s = view["schedule"]
    bc = view["boundary_change"]
    return Drill(
        scenario_id=view["view_id"].rsplit(".", 1)[-1],
        fixture_filename="",  # unused for rebuild equality
        claimed_beta_row="",  # carried out-of-band in the matrix
        notice_id=view["notice_id"],
        notice_kind=view["notice_kind"],
        title=view["title"],
        summary=view["summary"],
        created_at=view["created_at"],
        updated_at=view["updated_at"],
        as_of=view["as_of"],
        schedule_in={
            "time_basis": s["time_basis"],
            "starts_at": s["starts_at"],
            "expected_or_actual_ends_at": s["expected_or_actual_ends_at"],
            "completed_at": s["completed_at"],
            "timezone_id": s["timezone_id"],
            "utc_offset_at_start": s["utc_offset_at_start"],
            "latest_refresh_at": s["latest_refresh_at"],
        },
        scope_in={
            "deployment_profiles": view["affected_scope"]["deployment_profiles"],
            "tenant_refs": view["affected_scope"]["tenant_refs"],
            "region_refs": view["affected_scope"]["region_refs"],
            "residency_scope_classes": view["affected_scope"]["residency_scope_classes"],
            "service_classes": view["affected_scope"]["service_classes"],
            "scope_summary": view["affected_scope"]["scope_summary"],
        },
        boundary_in={
            "boundary_change_required": bc["boundary_change_required"],
            "review_completed": bc["review_completed"],
            "summary": bc["summary"],
            "axes": [
                {
                    "axis_class": a["axis_class"],
                    "axis_state_class": a["axis_state_class"],
                    "previous_ref": a["previous_ref"],
                    "current_ref": a["current_ref"],
                    "summary": a["summary"],
                }
                for a in bc["axes"]
            ],
        },
        blocked_writes_in=[
            {
                "action_class": w["action_class"],
                "block_state_class": w["block_state_class"],
                "continuity_posture": w["continuity_posture"],
                "safer_guidance": w["safer_guidance"],
                "queue_or_intent_ref": w["queue_or_intent_ref"],
                "idempotency_key_present": w["idempotency_key_present"],
                "resume_trigger": w["resume_trigger"],
                "note": w["note"],
            }
            for w in view["blocked_writes"]
        ],
        succeeded_in=[
            {
                "action_class": m["action_class"],
                "result_ref": m["result_ref"],
                "completed_at": m["completed_at"],
                "note": m["note"],
            }
            for m in view["succeeded_hosted_mutations"]
        ],
        local_in={
            "local_core_status": view["local_continuity"]["local_core_status"],
            "retained_local_safe_capabilities": view["local_continuity"][
                "retained_local_safe_capabilities"
            ],
            "continue_local_guidance_required": view["local_continuity"][
                "continue_local_guidance_required"
            ],
            "continuity_summary": view["local_continuity"]["continuity_summary"],
        },
        lifecycle_in={
            "freshness_class": view["lifecycle"]["freshness_class"],
            "supersedes_id": view["lifecycle"]["supersedes_id"],
            "superseded_by_id": view["lifecycle"]["superseded_by_id"],
            "retained_until_at": view["lifecycle"]["retained_until_at"],
            "history_refs": view["lifecycle"]["history_refs"],
        },
        display_in={
            k: view["display_copy"][k]
            for k in (
                "primary_status_line", "schedule_line", "scope_line",
                "blocked_writes_line", "queued_preserved_line", "succeeded_line",
                "local_continuity_line", "boundary_change_line", "freshness_line",
                "follow_up_line",
            )
        },
        stale_label=view["display_copy"]["stale_label"],
        history_ref=view["history_ref"],
        support_export_ref=view["support_export_ref"],
        evidence_refs=view["evidence_refs"],
    )


# --------------------------------------------------------------------------- #
# Drill definitions — the six communication drills this lane adds.
# --------------------------------------------------------------------------- #


def drills() -> list[Drill]:
    timezone_mismatch = Drill(
        scenario_id="timezone_mismatch_window",
        fixture_filename="timezone_mismatch_window.json",
        claimed_beta_row="beta.row.managed_cloud.eu_review_collab",
        notice_id="notice.maintenance.tz_window",
        notice_kind="scheduled_maintenance_window",
        title="Review collaboration maintenance window",
        summary="Managed review collaboration pauses for a scheduled maintenance window; reads and local work continue.",
        created_at="2026-05-20T08:00:00Z",
        updated_at="2026-05-20T11:58:00Z",
        schedule_in={
            "time_basis": "scheduled_exact",
            "starts_at": "2026-05-20T12:00:00Z",
            "expected_or_actual_ends_at": "2026-05-20T14:00:00Z",
            "completed_at": None,
            # A deliberately easy-to-misread offset: the absolute UTC instant
            # plus the IANA zone and offset must always travel together.
            "timezone_id": "Pacific/Chatham",
            "utc_offset_at_start": "+12:45",
            "latest_refresh_at": FRESH,
        },
        scope_in={
            "deployment_profiles": ["managed_cloud", "enterprise_online"],
            "tenant_refs": ["tenant.ref.acme"],
            "region_refs": ["region.ref.eu-central"],
            "residency_scope_classes": ["customer_region_pinned"],
            "service_classes": ["provider_review_service", "sync_service"],
            "scope_summary": "Managed-cloud and enterprise review collaboration in the EU-central region.",
        },
        boundary_in={
            "boundary_change_required": False,
            "review_completed": False,
            "summary": "No tenant, region, or endpoint boundary change.",
            "axes": [
                axis("tenant", "unchanged", None, "Tenant boundary unchanged."),
                axis("residency", "not_applicable", None, "Residency not affected by this window."),
            ],
        },
        blocked_writes_in=[
            blocked(
                "managed_review_comment_publish", "scheduled_to_block",
                "queued_publish_later", "retry_safe_when_resumed", "window_ends",
                "Review comments queue for publish-later and replay when the window ends.",
                queue_or_intent_ref="aureline://publish_later_queue/tz.window.comments",
                idempotency_key_present=True,
            ),
            blocked(
                "profile_settings_sync_write", "scheduled_to_block",
                "retryable_when_connected", "retry_safe_when_resumed", "window_ends",
                "Settings sync is retryable as soon as the window ends.",
            ),
        ],
        succeeded_in=[
            succeeded(
                "managed_review_approval", "aureline://change_review/cr-7100",
                "2026-05-20T11:50:00Z", "Approval landed before the window opened.",
            ),
        ],
        local_in={
            "local_core_status": "local_core_unaffected",
            "retained_local_safe_capabilities": [
                "Editing, saving, and local search continue.",
                "Local Git commit and branch continue.",
            ],
            "continue_local_guidance_required": True,
            "continuity_summary": "All local-first work continues; only managed review collaboration pauses.",
        },
        lifecycle_in={"freshness_class": "active_current"},
        display_in={
            "primary_status_line": "Scheduled maintenance window: review collaboration pauses; reads and local work continue.",
            "schedule_line": "Scheduled maintenance window starts 2026-05-20T12:00:00Z (ends 2026-05-20T14:00:00Z) Pacific/Chatham +12:45.",
            "scope_line": "Managed-cloud and enterprise review collaboration in the EU-central region.",
            "blocked_writes_line": "2 write classes affected this window.",
            "queued_preserved_line": "1 intents preserved (1 publish-later queued, 0 local drafts); they survive the window.",
            "succeeded_line": "1 hosted mutations already landed and are listed separately from queued work.",
            "local_continuity_line": "All local-first work continues; only managed review collaboration pauses.",
            "boundary_change_line": "No tenant / region / endpoint boundary change.",
            "freshness_line": "This notice is current.",
            "follow_up_line": "Queued work will replay when the window resumes; the window time is shown in UTC with the source timezone and offset.",
        },
        stale_label=None,
        history_ref="aureline://continuity_notice_history/notice.maintenance.tz_window",
        support_export_ref="aureline://support_export/notice.maintenance.tz_window",
        evidence_refs=["evidence.continuity.timezone_mismatch_window"],
    )

    stale_card = Drill(
        scenario_id="stale_maintenance_card_downgraded",
        fixture_filename="stale_maintenance_card_downgraded.json",
        claimed_beta_row="beta.row.managed_cloud.registry_maintenance",
        notice_id="notice.maintenance.stale_card",
        notice_kind="scheduled_maintenance_window",
        title="Registry maintenance window",
        summary="Registry service maintenance window; this notice has not refreshed recently.",
        created_at="2026-05-20T05:30:00Z",
        updated_at="2026-05-20T06:00:00Z",
        schedule_in={
            "time_basis": "scheduled_exact",
            "starts_at": "2026-05-20T06:00:00Z",
            "expected_or_actual_ends_at": "2026-05-20T13:00:00Z",
            "completed_at": None,
            "timezone_id": "Europe/Berlin",
            "utc_offset_at_start": "+02:00",
            # Last refresh aged out (6 h) -> refresh_stale -> downgrade.
            "latest_refresh_at": HOURS_AGO,
        },
        scope_in={
            "deployment_profiles": ["managed_cloud"],
            "tenant_refs": ["tenant.ref.acme"],
            "region_refs": ["region.ref.eu-central"],
            "residency_scope_classes": ["customer_region_pinned"],
            "service_classes": ["registry_service"],
            "scope_summary": "Managed-cloud registry service in the EU-central region.",
        },
        boundary_in={
            "boundary_change_required": False,
            "review_completed": False,
            "summary": "No tenant, region, or endpoint boundary change.",
            "axes": [
                axis("tenant", "unchanged", None, "Tenant boundary unchanged."),
                axis("residency", "not_applicable", None, "Residency not affected by this window."),
            ],
        },
        blocked_writes_in=[
            blocked(
                "extension_registry_publish_or_install", "blocked_read_only",
                "queued_publish_later", "retry_safe_when_resumed", "window_ends",
                "Registry publishes queue for publish-later and replay when the window resumes.",
                queue_or_intent_ref="aureline://publish_later_queue/registry.publishes",
                idempotency_key_present=True,
            ),
        ],
        succeeded_in=[],
        local_in={
            "local_core_status": "local_core_unaffected",
            "retained_local_safe_capabilities": [
                "Editing, saving, and local search continue.",
                "Installed extensions keep running locally.",
            ],
            "continue_local_guidance_required": True,
            "continuity_summary": "Local-first work continues; only managed registry writes pause.",
        },
        lifecycle_in={"freshness_class": "active_current"},
        display_in={
            "primary_status_line": "Scheduled maintenance window: registry writes pause; reads and local work continue.",
            "schedule_line": "Scheduled maintenance window starts 2026-05-20T06:00:00Z (ends 2026-05-20T13:00:00Z) Europe/Berlin +02:00.",
            "scope_line": "Managed-cloud registry service in the EU-central region.",
            "blocked_writes_line": "1 write classes affected this window.",
            "queued_preserved_line": "1 intents preserved (1 publish-later queued, 0 local drafts); they survive the window.",
            "succeeded_line": "No hosted mutations landed during this window.",
            "local_continuity_line": "Local-first work continues; only managed registry writes pause.",
            "boundary_change_line": "No tenant / region / endpoint boundary change.",
            "freshness_line": "This notice has not refreshed recently; treat its status as unconfirmed.",
            "follow_up_line": "Refresh the notice to confirm the window is still in effect.",
        },
        stale_label="Last refresh aged out — this maintenance status is unconfirmed.",
        history_ref="aureline://continuity_notice_history/notice.maintenance.stale_card",
        support_export_ref="aureline://support_export/notice.maintenance.stale_card",
        evidence_refs=["evidence.continuity.stale_maintenance_card_downgraded"],
    )

    drain_window = Drill(
        scenario_id="read_only_drain_window",
        fixture_filename="read_only_drain_window.json",
        claimed_beta_row="beta.row.managed_cloud.merge_queue_drain",
        notice_id="notice.drain.read_only_window",
        notice_kind="drain_window",
        title="Merge-queue drain window",
        summary="Merge queue drains existing work while new managed writes queue or save locally.",
        created_at="2026-05-20T11:30:00Z",
        updated_at="2026-05-20T11:58:00Z",
        schedule_in={
            "time_basis": "in_progress_exact",
            "starts_at": "2026-05-20T11:45:00Z",
            "expected_or_actual_ends_at": "2026-05-20T12:30:00Z",
            "completed_at": None,
            "timezone_id": "America/New_York",
            "utc_offset_at_start": "-04:00",
            "latest_refresh_at": FRESH,
        },
        scope_in={
            "deployment_profiles": ["managed_cloud", "enterprise_online"],
            "tenant_refs": ["tenant.ref.acme"],
            "region_refs": ["region.ref.us-east"],
            "residency_scope_classes": ["vendor_region_default"],
            "service_classes": ["merge_queue_service", "relay_service"],
            "scope_summary": "Managed-cloud and enterprise merge queue and collaboration in the US-east region.",
        },
        boundary_in={
            "boundary_change_required": False,
            "review_completed": False,
            "summary": "No tenant, region, or endpoint boundary change; this is a clean drain.",
            "axes": [
                axis("region", "unchanged", None, "Region boundary unchanged during the drain."),
                axis("tenant", "unchanged", None, "Tenant boundary unchanged during the drain."),
            ],
        },
        blocked_writes_in=[
            blocked(
                "merge_queue_enqueue", "blocked_drain_new_actions",
                "queued_publish_later", "retry_safe_when_resumed", "drain_completes",
                "New merge enqueues queue for publish-later and replay when the drain completes.",
                queue_or_intent_ref="aureline://publish_later_queue/merge.enqueues",
                idempotency_key_present=True,
            ),
            blocked(
                "collaboration_presence_write", "draining_existing_only",
                "draining_existing_only", "retry_safe_when_resumed", "drain_completes",
                "Existing collaboration sessions finish; new presence writes wait for the drain to complete.",
            ),
            blocked(
                "provider_publish_local_draft", "blocked_read_only",
                "local_draft_preserved", "postpone_safer", "window_ends",
                "An in-progress provider publish is saved as a local draft and survives the drain.",
                queue_or_intent_ref="aureline://local_draft/provider.publish.draft",
            ),
        ],
        succeeded_in=[
            succeeded(
                "managed_review_approval", "aureline://change_review/cr-8800",
                "2026-05-20T11:40:00Z", "An approval landed before the drain began.",
            ),
        ],
        local_in={
            "local_core_status": "meaningful_safe_subset_available",
            "retained_local_safe_capabilities": [
                "Editing, saving, and local search continue.",
                "Local Git commit and branch continue.",
            ],
            "continue_local_guidance_required": True,
            "continuity_summary": "Local-first work continues; queued and local-draft work survives the drain.",
        },
        lifecycle_in={"freshness_class": "active_current"},
        display_in={
            "primary_status_line": "Drain window: existing work finishes while new managed writes queue or save locally.",
            "schedule_line": "Drain window starts 2026-05-20T11:45:00Z (ends 2026-05-20T12:30:00Z) America/New_York -04:00.",
            "scope_line": "Managed-cloud and enterprise merge queue and collaboration in the US-east region.",
            "blocked_writes_line": "3 write classes affected this window.",
            "queued_preserved_line": "2 intents preserved (1 publish-later queued, 1 local drafts); they survive the drain.",
            "succeeded_line": "1 hosted mutations already landed and are listed separately from queued work.",
            "local_continuity_line": "Local-first work continues; queued and local-draft work survives the drain.",
            "boundary_change_line": "No tenant / region / endpoint boundary change; this is a clean drain.",
            "freshness_line": "This notice is current.",
            "follow_up_line": "Queued and local-draft work will replay when the drain completes.",
        },
        stale_label=None,
        history_ref="aureline://continuity_notice_history/notice.drain.read_only_window",
        support_export_ref="aureline://support_export/notice.drain.read_only_window",
        evidence_refs=["evidence.continuity.read_only_drain_window"],
    )

    changed_endpoint = Drill(
        scenario_id="changed_endpoint_failover",
        fixture_filename="changed_endpoint_failover.json",
        claimed_beta_row="beta.row.managed_cloud.regional_failover",
        notice_id="notice.failover.changed_endpoint",
        notice_kind="regional_failover",
        title="Regional failover to standby endpoint",
        summary="An emergency regional failover moved traffic to a standby region and endpoint; the boundary change stays visible.",
        created_at="2026-05-20T11:30:00Z",
        updated_at="2026-05-20T11:52:00Z",
        schedule_in={
            "time_basis": "detected_exact",
            "starts_at": "2026-05-20T11:30:00Z",
            "expected_or_actual_ends_at": None,
            "completed_at": None,
            "timezone_id": "Europe/Dublin",
            "utc_offset_at_start": "+01:00",
            "latest_refresh_at": RECENT,
        },
        scope_in={
            "deployment_profiles": ["managed_cloud"],
            "tenant_refs": ["tenant.ref.acme"],
            "region_refs": ["region.ref.eu-west", "region.ref.eu-central"],
            "residency_scope_classes": ["residency_changed_review_required"],
            "service_classes": ["workspace_control_plane_service", "provider_review_service"],
            "scope_summary": "Managed-cloud workspace control plane failed over from EU-west to EU-central.",
        },
        boundary_in={
            "boundary_change_required": True,
            "review_completed": False,
            "summary": "Region and endpoint identity changed in the failover; review the new boundary before publishing.",
            "axes": [
                axis(
                    "region", "changed", "aureline://region_boundary/eu-central",
                    "Region moved from EU-west to EU-central.",
                    previous_ref="region.ref.eu-west",
                ),
                axis(
                    "endpoint_identity", "changed", "aureline://endpoint_boundary/standby-eu-central",
                    "Endpoint identity changed to the standby control-plane endpoint.",
                    previous_ref="endpoint.ref.primary-eu-west",
                ),
                axis("tenant", "unchanged", None, "Tenant boundary unchanged."),
            ],
        },
        blocked_writes_in=[
            blocked(
                "provider_publish_immediate", "blocked_pending_boundary_recheck",
                "blocked_pending_boundary_recheck", "postpone_safer", "boundary_review_completed",
                "Immediate publishes are held until the changed endpoint boundary is reviewed; they do not auto-replay.",
            ),
            blocked(
                "managed_workspace_lifecycle_write", "blocked_pending_boundary_recheck",
                "requires_manual_rerun", "manual_rerun_required", "fresh_approval_issued",
                "An in-flight workspace lifecycle write needs a manual rerun after a fresh approval against the new boundary.",
            ),
        ],
        succeeded_in=[],
        local_in={
            "local_core_status": "meaningful_safe_subset_available",
            "retained_local_safe_capabilities": [
                "Editing, saving, and local search continue.",
                "Local Git commit and branch continue.",
            ],
            "continue_local_guidance_required": True,
            "continuity_summary": "Local-first work continues; managed publishes wait for the new endpoint boundary review.",
        },
        lifecycle_in={"freshness_class": "active_current"},
        display_in={
            "primary_status_line": "Regional failover: traffic moved to a standby region and endpoint; the boundary change is shown.",
            "schedule_line": "Regional failover detected 2026-05-20T11:30:00Z Europe/Dublin +01:00.",
            "scope_line": "Managed-cloud workspace control plane failed over from EU-west to EU-central.",
            "blocked_writes_line": "2 write classes affected by this failover.",
            "queued_preserved_line": "No publish-later or local-draft intents in this failover; affected writes are held for boundary review.",
            "succeeded_line": "No hosted mutations landed during this failover.",
            "local_continuity_line": "Local-first work continues; managed publishes wait for the new endpoint boundary review.",
            "boundary_change_line": "Region and endpoint identity changed; the new boundary is shown and must be reviewed before publishing.",
            "freshness_line": "This notice is current.",
            "follow_up_line": "Review the changed region and endpoint boundary before resuming managed publishes.",
        },
        stale_label=None,
        history_ref="aureline://continuity_notice_history/notice.failover.changed_endpoint",
        support_export_ref="aureline://support_export/notice.failover.changed_endpoint",
        evidence_refs=["evidence.continuity.changed_endpoint_failover"],
    )

    queued_preserved = Drill(
        scenario_id="queued_publish_later_preserved",
        fixture_filename="queued_publish_later_preserved.json",
        claimed_beta_row="beta.row.hybrid.read_only_publish_later",
        notice_id="notice.drain.queued_publish_later",
        notice_kind="read_only_window",
        title="Read-only window with queued publish-later work",
        summary="A read-only window queues a managed publish for later and saves a provider publish as a local draft.",
        created_at="2026-05-20T11:40:00Z",
        updated_at="2026-05-20T11:59:00Z",
        schedule_in={
            "time_basis": "in_progress_exact",
            "starts_at": "2026-05-20T11:50:00Z",
            "expected_or_actual_ends_at": "2026-05-20T12:40:00Z",
            "completed_at": None,
            "timezone_id": "Asia/Kolkata",
            "utc_offset_at_start": "+05:30",
            "latest_refresh_at": FRESH,
        },
        scope_in={
            "deployment_profiles": ["managed_cloud", "self_hosted"],
            "tenant_refs": ["tenant.ref.globex"],
            "region_refs": ["region.ref.ap-south"],
            "residency_scope_classes": ["customer_region_pinned"],
            "service_classes": ["provider_review_service", "sync_service"],
            "scope_summary": "Managed-cloud and self-hosted review publishing in the AP-south region.",
        },
        boundary_in={
            "boundary_change_required": False,
            "review_completed": False,
            "summary": "No tenant, region, or endpoint boundary change.",
            "axes": [
                axis("tenant", "unchanged", None, "Tenant boundary unchanged."),
                axis("region", "unchanged", None, "Region boundary unchanged."),
            ],
        },
        blocked_writes_in=[
            blocked(
                "managed_review_comment_publish", "blocked_read_only",
                "queued_publish_later", "retry_safe_when_resumed", "window_ends",
                "Review comments queue for publish-later with an idempotency key and replay when the window ends.",
                queue_or_intent_ref="aureline://publish_later_queue/review.comments",
                idempotency_key_present=True,
            ),
            blocked(
                "provider_publish_local_draft", "blocked_read_only",
                "local_draft_preserved", "postpone_safer", "window_ends",
                "A provider publish is saved as a local draft and survives the read-only window.",
                queue_or_intent_ref="aureline://local_draft/provider.review.draft",
            ),
        ],
        succeeded_in=[
            succeeded(
                "managed_review_approval", "aureline://change_review/cr-7205",
                "2026-05-20T11:48:00Z",
                "An approval landed before the window and is listed separately from queued work.",
            ),
        ],
        local_in={
            "local_core_status": "local_core_unaffected",
            "retained_local_safe_capabilities": [
                "Editing, saving, and local search continue.",
                "Local Git commit and branch continue.",
            ],
            "continue_local_guidance_required": True,
            "continuity_summary": "Local-first work continues; queued and local-draft publishes survive the window.",
        },
        lifecycle_in={"freshness_class": "active_current"},
        display_in={
            "primary_status_line": "Read-only window: managed writes queue for later or save as local drafts; reads continue.",
            "schedule_line": "Read-only window starts 2026-05-20T11:50:00Z (ends 2026-05-20T12:40:00Z) Asia/Kolkata +05:30.",
            "scope_line": "Managed-cloud and self-hosted review publishing in the AP-south region.",
            "blocked_writes_line": "2 write classes affected this window.",
            "queued_preserved_line": "2 intents preserved (1 publish-later queued, 1 local drafts); they survive the window.",
            "succeeded_line": "1 hosted mutations already landed and are listed separately from queued work.",
            "local_continuity_line": "Local-first work continues; queued and local-draft publishes survive the window.",
            "boundary_change_line": "No tenant / region / endpoint boundary change.",
            "freshness_line": "This notice is current.",
            "follow_up_line": "Queued and local-draft work will replay when the window resumes.",
        },
        stale_label=None,
        history_ref="aureline://continuity_notice_history/notice.drain.queued_publish_later",
        support_export_ref="aureline://support_export/notice.drain.queued_publish_later",
        evidence_refs=["evidence.continuity.queued_publish_later_preserved"],
    )

    reconciliation = Drill(
        scenario_id="post_window_reconciliation_changed_authority",
        fixture_filename="post_window_reconciliation_changed_authority.json",
        claimed_beta_row="beta.row.managed_cloud.tenant_reconciliation",
        notice_id="notice.maintenance.post_window_reconciliation",
        notice_kind="post_event_reconciliation",
        title="Post-window reconciliation under changed authority",
        summary="Reconciliation after a failover changed the tenant and key-ownership authority; queued intent is held for a boundary recheck.",
        created_at="2026-05-20T11:00:00Z",
        updated_at="2026-05-20T11:40:00Z",
        schedule_in={
            "time_basis": "in_progress_exact",
            "starts_at": "2026-05-20T11:00:00Z",
            "expected_or_actual_ends_at": "2026-05-20T12:30:00Z",
            "completed_at": None,
            "timezone_id": "Europe/London",
            "utc_offset_at_start": "+01:00",
            "latest_refresh_at": RECENT,
        },
        scope_in={
            "deployment_profiles": ["managed_cloud"],
            "tenant_refs": ["tenant.ref.acme-successor", "tenant.ref.acme"],
            "region_refs": ["region.ref.eu-central"],
            "residency_scope_classes": ["tenant_residency_recheck_required"],
            "service_classes": ["policy_service", "provider_review_service"],
            "scope_summary": "Managed-cloud policy and review reconciliation after a tenant / key-ownership change.",
        },
        boundary_in={
            "boundary_change_required": True,
            "review_completed": False,
            "summary": "Tenant identity and key ownership changed; queued intent must wait for a boundary review before it can resume.",
            "axes": [
                axis(
                    "tenant", "changed", "aureline://tenant_boundary/acme-successor",
                    "Tenant (target identity) changed to the successor tenant.",
                    previous_ref="tenant.ref.acme",
                ),
                axis(
                    "key_ownership", "changed", "aureline://key_ownership_boundary/acme-successor-kms",
                    "Key ownership (authority) moved to the successor tenant's key set.",
                    previous_ref="key.ref.acme-kms",
                ),
                axis("region", "unchanged", None, "Region boundary unchanged."),
            ],
        },
        blocked_writes_in=[
            blocked(
                "policy_admin_write", "blocked_pending_boundary_recheck",
                "blocked_pending_boundary_recheck", "postpone_safer", "boundary_review_completed",
                "Policy admin writes are held until the changed tenant / key-ownership boundary is reviewed; they do not auto-replay.",
            ),
            blocked(
                "provider_publish_local_draft", "blocked_pending_boundary_recheck",
                "local_draft_preserved", "postpone_safer", "boundary_review_completed",
                "A provider publish is preserved as a local draft and held for boundary review — it never silently replays across the new authority.",
                queue_or_intent_ref="aureline://local_draft/reconciliation.publish.draft",
            ),
        ],
        succeeded_in=[],
        local_in={
            "local_core_status": "meaningful_safe_subset_available",
            "retained_local_safe_capabilities": [
                "Editing, saving, and local search continue.",
                "Local Git commit and branch continue.",
            ],
            "continue_local_guidance_required": True,
            "continuity_summary": "Local-first work continues; managed writes wait for the changed-authority boundary review.",
        },
        lifecycle_in={"freshness_class": "active_current"},
        display_in={
            "primary_status_line": "Post-event reconciliation: tenant and key-ownership authority changed; queued intent waits for boundary review.",
            "schedule_line": "Reconciliation in progress since 2026-05-20T11:00:00Z (expected end 2026-05-20T12:30:00Z) Europe/London +01:00.",
            "scope_line": "Managed-cloud policy and review reconciliation after a tenant / key-ownership change.",
            "blocked_writes_line": "2 write classes held pending boundary review.",
            "queued_preserved_line": "1 intents preserved (0 publish-later queued, 1 local drafts); preserved work is held for boundary review, not auto-replayed.",
            "succeeded_line": "No hosted mutations landed during reconciliation.",
            "local_continuity_line": "Local-first work continues; managed writes wait for the changed-authority boundary review.",
            "boundary_change_line": "Tenant identity and key ownership changed; the new authority is shown and must be reviewed before any queued intent resumes.",
            "freshness_line": "This notice is current.",
            "follow_up_line": "Review the changed tenant / key-ownership boundary; only then will held intent resume against the new authority.",
        },
        stale_label=None,
        history_ref="aureline://continuity_notice_history/notice.maintenance.post_window_reconciliation",
        support_export_ref="aureline://support_export/notice.maintenance.post_window_reconciliation",
        evidence_refs=["evidence.continuity.post_window_reconciliation_changed_authority"],
    )

    return [
        timezone_mismatch,
        stale_card,
        drain_window,
        changed_endpoint,
        queued_preserved,
        reconciliation,
    ]


CLAIMED_BETA_ROWS = {d.scenario_id: d.claimed_beta_row for d in drills()}


# --------------------------------------------------------------------------- #
# Per-view lane-failing rules
# --------------------------------------------------------------------------- #


def check_view(label: str, view: dict[str, Any]) -> list[Finding]:
    findings: list[Finding] = []

    # 1. Independent rebuild from extracted inputs must match the stored record.
    #    This is the structural no-silent guard: a fixture that claims a notice is
    #    current while its lifecycle / refresh cannot back it, or that miscounts
    #    boundary axes / preserved intent, disagrees with the rebuild.
    try:
        rebuilt = build_view(extract_inputs(view))
    except (KeyError, TypeError, ValueError) as exc:
        return [
            err(
                "maintenance_failover.rebuild.failed",
                f"{label}: could not rebuild record from inputs: {exc}",
                "Ensure the record carries every field the model emits.",
                label,
            )
        ]
    if rebuilt != view:
        findings.append(
            err(
                "maintenance_failover.rebuild.mismatch",
                f"{label}: stored record does not match the model projection of its "
                "inputs (freshness / boundary / write-continuity / roll-up drift)",
                "Re-emit with --write, or fix the derived fields so refresh age, "
                "effective freshness, downgrade reasons, boundary counts, honesty, "
                "and summary counts all follow the model.",
                label,
                diff=_first_diff(view, rebuilt),
            )
        )

    eff = view.get("effective_freshness")
    dc = view.get("display_copy", {})

    # 2. No stale-as-current. A downgraded notice cannot read as current.
    downgraded = bool(view.get("freshness_downgraded"))
    if (eff != "current") != downgraded:
        findings.append(
            err(
                "maintenance_failover.stale.flag",
                f"{label}: effective_freshness={eff!r} disagrees with "
                f"freshness_downgraded={downgraded}",
                "A notice is current iff it is not downgraded.",
                label,
            )
        )
    if downgraded:
        if not view.get("downgrade_reasons"):
            findings.append(
                err(
                    "maintenance_failover.stale.no_reason",
                    f"{label}: downgraded notice names no downgrade reason",
                    "Name at least one downgrade reason.",
                    label,
                )
            )
        if not view.get("honesty_marker_present"):
            findings.append(
                err(
                    "maintenance_failover.stale.no_marker",
                    f"{label}: downgraded notice does not light the honesty marker",
                    "Light the honesty marker on every downgraded notice.",
                    label,
                )
            )
        if not dc.get("stale_label"):
            findings.append(
                err(
                    "maintenance_failover.stale.no_label",
                    f"{label}: downgraded notice carries no stale label",
                    "A downgraded notice must carry a stale label.",
                    label,
                )
            )
    if dc.get("stale_presented_as_current") is not False:
        findings.append(
            err(
                "maintenance_failover.stale.presented_current",
                f"{label}: stale_presented_as_current must be false",
                "A stale / superseded / completed notice cannot present as current.",
                label,
            )
        )

    # 3. Changed endpoint (or any changed authority axis) stays visible behind
    #    recovered messaging, with a canonical current ref.
    bc = view.get("boundary_change", {})
    for a in bc.get("axes", []):
        st = a.get("axis_state_class")
        if st in MEANINGFUL_AXIS_STATES and not is_canonical_ref(a.get("current_ref")):
            findings.append(
                err(
                    "maintenance_failover.boundary.current_ref",
                    f"{label}: changed axis {a.get('axis_class')} has a non-canonical "
                    "current_ref",
                    "A changed / unknown axis must carry a canonical current_ref.",
                    label,
                )
            )
    changed_endpoint = any(
        a.get("axis_class") == "endpoint_identity"
        and a.get("axis_state_class") in MEANINGFUL_AXIS_STATES
        for a in bc.get("axes", [])
    )
    if dc.get("boundary_change_hidden") is not False:
        findings.append(
            err(
                "maintenance_failover.boundary.hidden",
                f"{label}: boundary_change_hidden must be false",
                "Never hide a changed boundary behind recovered messaging.",
                label,
            )
        )
    if changed_endpoint and not bc.get("boundary_change_required"):
        findings.append(
            err(
                "maintenance_failover.boundary.endpoint_not_required",
                f"{label}: a changed endpoint does not mark the boundary change required",
                "A changed endpoint identity requires a boundary review.",
                label,
            )
        )

    # 4. No silent replay across a changed authority boundary. An auto-replay
    #    posture must never resume on a boundary-recheck trigger.
    succeeded_actions = {m.get("action_class") for m in view.get("succeeded_hosted_mutations", [])}
    for w in view.get("blocked_writes", []):
        posture = w.get("continuity_posture")
        resume = w.get("resume_trigger")
        if posture in AUTO_REPLAY_POSTURES and resume in BOUNDARY_RECHECK_TRIGGERS:
            findings.append(
                err(
                    "maintenance_failover.replay.across_boundary",
                    f"{label}: write {w.get('action_class')} auto-replays "
                    f"(posture={posture}) on a boundary-recheck trigger "
                    f"({resume}) — queued intent would silently cross the changed "
                    "authority boundary",
                    "Hold a write that needs a boundary recheck as "
                    "blocked_pending_boundary_recheck (or require a manual rerun) "
                    "instead of auto-replaying it.",
                    label,
                )
            )
        if posture in PRESERVED_POSTURES:
            if not w.get("intent_preserved"):
                findings.append(
                    err(
                        "maintenance_failover.write.preserved_flag",
                        f"{label}: preserved write {w.get('action_class')} is not "
                        "marked intent_preserved",
                        "Mark preserved writes intent_preserved.",
                        label,
                    )
                )
            if not is_canonical_ref(w.get("queue_or_intent_ref")):
                findings.append(
                    err(
                        "maintenance_failover.write.queue_ref",
                        f"{label}: preserved write {w.get('action_class')} lacks a "
                        "canonical queue / intent ref",
                        "Preserved writes must carry a canonical queue / intent ref.",
                        label,
                    )
                )
        if w.get("action_class") in succeeded_actions:
            findings.append(
                err(
                    "maintenance_failover.write.collapsed",
                    f"{label}: action {w.get('action_class')} appears both "
                    "blocked / queued and succeeded",
                    "Keep queued / preserved work separate from successful hosted "
                    "mutations.",
                    label,
                )
            )

    for m in view.get("succeeded_hosted_mutations", []):
        if not is_canonical_ref(m.get("result_ref")):
            findings.append(
                err(
                    "maintenance_failover.mutation.result_ref",
                    f"{label}: hosted mutation {m.get('action_class')} has a "
                    "non-canonical result_ref",
                    "Route hosted mutations to a canonical durable object.",
                    label,
                )
            )

    # 5. Timezone is unambiguous: the schedule line carries the UTC instant, the
    #    IANA zone, and the offset in force, so it can never be read as a naive
    #    local time.
    s = view.get("schedule", {})
    schedule_line = dc.get("schedule_line", "")
    for piece, msg in (
        (s.get("starts_at"), "the absolute UTC start instant"),
        (s.get("timezone_id"), "the source timezone id"),
        (s.get("utc_offset_at_start"), "the UTC offset in force"),
    ):
        if not piece or piece not in schedule_line:
            findings.append(
                err(
                    "maintenance_failover.timezone.ambiguous",
                    f"{label}: schedule line does not carry {msg} ({piece!r})",
                    "Render the window with its UTC instant, IANA zone, and offset "
                    "so it cannot be misread as a naive local time.",
                    label,
                )
            )

    # 6. Routing.
    for ref_field in ("history_ref", "support_export_ref"):
        if not is_canonical_ref(view.get(ref_field)):
            findings.append(
                err(
                    "maintenance_failover.route." + ref_field,
                    f"{label}: {ref_field} is not canonical",
                    "Route durable history / support export to a canonical object.",
                    label,
                )
            )

    return findings


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


# --------------------------------------------------------------------------- #
# Coverage — the corpus must prove every required drill
# --------------------------------------------------------------------------- #


def validate_coverage(views: dict[str, dict[str, Any]]) -> list[Finding]:
    findings: list[Finding] = []

    has_downgraded = False
    has_changed_endpoint = False
    has_changed_tenant = False
    has_changed_region = False
    has_drain = False
    has_queued = False
    has_local_draft = False
    has_held_boundary = False
    has_reconciliation_authority = False
    has_non_utc_offset = False

    for view in views.values():
        if view["effective_freshness"] != "current":
            has_downgraded = True
        for a in view["boundary_change"]["axes"]:
            if a["axis_state_class"] in MEANINGFUL_AXIS_STATES:
                if a["axis_class"] == "endpoint_identity":
                    has_changed_endpoint = True
                if a["axis_class"] == "tenant":
                    has_changed_tenant = True
                if a["axis_class"] == "region":
                    has_changed_region = True
        for w in view["blocked_writes"]:
            if w["continuity_posture"] == "draining_existing_only":
                has_drain = True
            if w["continuity_posture"] == "queued_publish_later":
                has_queued = True
            if w["continuity_posture"] == "local_draft_preserved":
                has_local_draft = True
            if (
                w["continuity_posture"] not in AUTO_REPLAY_POSTURES
                and w["resume_trigger"] in BOUNDARY_RECHECK_TRIGGERS
            ):
                has_held_boundary = True
        if view["notice_kind"] == "post_event_reconciliation":
            authority_changed = any(
                a["axis_class"] in {"tenant", "key_ownership", "residency"}
                and a["axis_state_class"] in MEANINGFUL_AXIS_STATES
                for a in view["boundary_change"]["axes"]
            )
            if authority_changed:
                has_reconciliation_authority = True
        if view["schedule"]["utc_offset_at_start"] not in ("Z", "+00:00"):
            has_non_utc_offset = True

    required = [
        ("downgraded_stale_notice", has_downgraded,
         "Add a maintenance card whose refresh aged out so it downgrades."),
        ("changed_endpoint", has_changed_endpoint,
         "Add a failover that changed the endpoint identity."),
        ("changed_tenant", has_changed_tenant,
         "Add a drill with a changed tenant boundary."),
        ("changed_region", has_changed_region,
         "Add a drill with a changed region boundary."),
        ("drain_window", has_drain,
         "Add a drain / read-only window that drains existing sessions."),
        ("queued_publish_later", has_queued,
         "Add a drill that queues a managed write for publish-later."),
        ("local_draft_preserved", has_local_draft,
         "Add a drill that preserves a local-draft intent."),
        ("held_for_boundary_recheck", has_held_boundary,
         "Add a write held for a boundary recheck (not auto-replayed)."),
        ("post_window_reconciliation_changed_authority", has_reconciliation_authority,
         "Add a post-event reconciliation under a changed tenant / key-ownership authority."),
        ("non_utc_timezone", has_non_utc_offset,
         "Add a scheduled window declared in a non-UTC timezone offset."),
    ]
    for name, ok, remedy in required:
        if not ok:
            findings.append(
                err(
                    f"maintenance_failover.coverage.{name}",
                    f"corpus does not prove the required drill: {name}",
                    remedy,
                )
            )
    return findings


# --------------------------------------------------------------------------- #
# Export parity — support bundle (plaintext) and CLI / headless index preserve
# the same semantics as the product UI record.
# --------------------------------------------------------------------------- #


def _join(values: list[str]) -> str:
    return ",".join(values) if values else "-"


def _split(value: str) -> list[str]:
    return [] if value == "-" else value.split(",")


def changed_axes_of(view: dict[str, Any]) -> list[dict[str, Any]]:
    return [
        {
            "axis_class": a["axis_class"],
            "axis_state_class": a["axis_state_class"],
            "current_ref": a["current_ref"],
        }
        for a in view["boundary_change"]["axes"]
        if a["axis_state_class"] in MEANINGFUL_AXIS_STATES
    ]


def digest_from_view(view: dict[str, Any]) -> dict[str, Any]:
    s = view["schedule"]
    sc = view["affected_scope"]
    return {
        "notice_id": view["notice_id"],
        "notice_kind": view["notice_kind"],
        "category": view["category"],
        "plan_class": view["plan_class"],
        "as_of": view["as_of"],
        "effective_freshness": view["effective_freshness"],
        "freshness_downgraded": view["freshness_downgraded"],
        "downgrade_reasons": sorted(view["downgrade_reasons"]),
        "boundary_change_unresolved": view["boundary_change_unresolved"],
        "honesty_marker_present": view["honesty_marker_present"],
        "schedule": {
            "starts_at": s["starts_at"],
            "ends_at": s["expected_or_actual_ends_at"],
            "completed_at": s["completed_at"],
            "timezone_id": s["timezone_id"],
            "utc_offset_at_start": s["utc_offset_at_start"],
            "latest_refresh_at": s["latest_refresh_at"],
            "refresh_age": s["refresh_age"],
        },
        "scope": {
            "deployment_profiles": list(sc["deployment_profiles"]),
            "tenant_refs": list(sc["tenant_refs"]),
            "region_refs": list(sc["region_refs"]),
            "residency_scope_classes": list(sc["residency_scope_classes"]),
            "service_classes": list(sc["service_classes"]),
        },
        "changed_axes": changed_axes_of(view),
        "blocked_writes": [
            {
                "action_class": w["action_class"],
                "block_state_class": w["block_state_class"],
                "continuity_posture": w["continuity_posture"],
                "safer_guidance": w["safer_guidance"],
                "resume_trigger": w["resume_trigger"],
                "intent_preserved": w["intent_preserved"],
                "queue_or_intent_ref": w["queue_or_intent_ref"],
            }
            for w in view["blocked_writes"]
        ],
        "succeeded_hosted_mutations": [
            {"action_class": m["action_class"], "result_ref": m["result_ref"]}
            for m in view["succeeded_hosted_mutations"]
        ],
        "summary_counts": dict(view["summary_counts"]),
        "history_ref": view["history_ref"],
        "support_export_ref": view["support_export_ref"],
    }


def coarse_digest(full: dict[str, Any]) -> dict[str, Any]:
    return {
        "effective_freshness": full["effective_freshness"],
        "honesty_marker_present": full["honesty_marker_present"],
        "boundary_change_unresolved": full["boundary_change_unresolved"],
        "preserved_intent_count": full["summary_counts"]["preserved_intent_count"],
        "changed_boundary_axis_count": full["summary_counts"]["changed_boundary_axis_count"],
        "blocked_write_count": full["summary_counts"]["blocked_write_count"],
    }


def _b(value: bool) -> str:
    return "true" if value else "false"


def render_support_export(view: dict[str, Any]) -> str:
    """Self-contained support-bundle plaintext a support engineer can read.

    It carries the full semantic digest so maintenance / failover outcomes can be
    explained without rehydrating live control-plane state.
    """
    d = digest_from_view(view)
    s = d["schedule"]
    sc = d["scope"]
    out: list[str] = []
    out.append("Maintenance & failover continuity packet")
    out.append(f"view_id: {view['view_id']}")
    out.append(f"notice_id: {d['notice_id']}")
    out.append(f"notice_kind: {d['notice_kind']}")
    out.append(f"category: {d['category']}")
    out.append(f"plan_class: {d['plan_class']}")
    out.append(f"as_of: {d['as_of']}")
    out.append(f"schedule.starts_at: {s['starts_at']}")
    out.append(f"schedule.ends_at: {s['ends_at'] or '-'}")
    out.append(f"schedule.completed_at: {s['completed_at'] or '-'}")
    out.append(f"schedule.timezone_id: {s['timezone_id']}")
    out.append(f"schedule.utc_offset_at_start: {s['utc_offset_at_start']}")
    out.append(f"schedule.latest_refresh_at: {s['latest_refresh_at'] or '-'}")
    out.append(f"schedule.refresh_age: {s['refresh_age']}")
    out.append(f"effective_freshness: {d['effective_freshness']}")
    out.append(f"freshness_downgraded: {_b(d['freshness_downgraded'])}")
    out.append(f"downgrade_reasons: {_join(d['downgrade_reasons'])}")
    out.append(f"boundary_change_unresolved: {_b(d['boundary_change_unresolved'])}")
    out.append(f"honesty_marker: {'present' if d['honesty_marker_present'] else 'none'}")
    out.append(f"scope.deployment_profiles: {_join(sc['deployment_profiles'])}")
    out.append(f"scope.tenant_refs: {_join(sc['tenant_refs'])}")
    out.append(f"scope.region_refs: {_join(sc['region_refs'])}")
    out.append(f"scope.residency_scope_classes: {_join(sc['residency_scope_classes'])}")
    out.append(f"scope.service_classes: {_join(sc['service_classes'])}")
    counts = d["summary_counts"]
    out.append(
        "summary: blocked={blocked_write_count} queued={queued_publish_later_count} "
        "drafts={local_draft_preserved_count} preserved={preserved_intent_count} "
        "no_retry={blocked_no_safe_retry_count} manual={requires_manual_rerun_count} "
        "succeeded={succeeded_hosted_mutation_count} "
        "changed_axes={changed_boundary_axis_count} "
        "unknown_axes={unknown_boundary_axis_count}".format(**counts)
    )
    out.append(f"history_ref: {d['history_ref']}")
    out.append(f"support_export_ref: {d['support_export_ref']}")
    for a in d["changed_axes"]:
        out.append(
            f"- axis {a['axis_class']} state={a['axis_state_class']} "
            f"ref={a['current_ref'] or '-'}"
        )
    for w in d["blocked_writes"]:
        out.append(
            f"- blocked {w['action_class']} block={w['block_state_class']} "
            f"posture={w['continuity_posture']} guidance={w['safer_guidance']} "
            f"resume={w['resume_trigger']} preserved={_b(w['intent_preserved'])} "
            f"ref={w['queue_or_intent_ref'] or '-'}"
        )
    for m in d["succeeded_hosted_mutations"]:
        out.append(f"- succeeded {m['action_class']} ref={m['result_ref']}")
    return "\n".join(out) + "\n"


def digest_from_support_export(text: str) -> dict[str, Any]:
    fields: dict[str, str] = {}
    changed_axes: list[dict[str, Any]] = []
    blocked_writes: list[dict[str, Any]] = []
    succeeded: list[dict[str, Any]] = []
    for line in text.split("\n"):
        if not line:
            continue
        if line.startswith("- axis "):
            parts = dict(p.split("=", 1) for p in line[len("- axis "):].split(" ") if "=" in p)
            axis_class = line[len("- axis "):].split(" ", 1)[0]
            changed_axes.append(
                {
                    "axis_class": axis_class,
                    "axis_state_class": parts["state"],
                    "current_ref": None if parts["ref"] == "-" else parts["ref"],
                }
            )
        elif line.startswith("- blocked "):
            action = line[len("- blocked "):].split(" ", 1)[0]
            parts = dict(p.split("=", 1) for p in line.split(" ") if "=" in p)
            blocked_writes.append(
                {
                    "action_class": action,
                    "block_state_class": parts["block"],
                    "continuity_posture": parts["posture"],
                    "safer_guidance": parts["guidance"],
                    "resume_trigger": parts["resume"],
                    "intent_preserved": parts["preserved"] == "true",
                    "queue_or_intent_ref": None if parts["ref"] == "-" else parts["ref"],
                }
            )
        elif line.startswith("- succeeded "):
            action = line[len("- succeeded "):].split(" ", 1)[0]
            parts = dict(p.split("=", 1) for p in line.split(" ") if "=" in p)
            succeeded.append({"action_class": action, "result_ref": parts["ref"]})
        elif ": " in line:
            key, value = line.split(": ", 1)
            fields[key] = value

    counts_raw = dict(p.split("=", 1) for p in fields["summary"].split(" "))
    summary_counts = {
        "blocked_write_count": int(counts_raw["blocked"]),
        "queued_publish_later_count": int(counts_raw["queued"]),
        "local_draft_preserved_count": int(counts_raw["drafts"]),
        "preserved_intent_count": int(counts_raw["preserved"]),
        "blocked_no_safe_retry_count": int(counts_raw["no_retry"]),
        "requires_manual_rerun_count": int(counts_raw["manual"]),
        "succeeded_hosted_mutation_count": int(counts_raw["succeeded"]),
        "changed_boundary_axis_count": int(counts_raw["changed_axes"]),
        "unknown_boundary_axis_count": int(counts_raw["unknown_axes"]),
    }

    def opt(value: str) -> str | None:
        return None if value == "-" else value

    return {
        "notice_id": fields["notice_id"],
        "notice_kind": fields["notice_kind"],
        "category": fields["category"],
        "plan_class": fields["plan_class"],
        "as_of": fields["as_of"],
        "effective_freshness": fields["effective_freshness"],
        "freshness_downgraded": fields["freshness_downgraded"] == "true",
        "downgrade_reasons": _split(fields["downgrade_reasons"]),
        "boundary_change_unresolved": fields["boundary_change_unresolved"] == "true",
        "honesty_marker_present": fields["honesty_marker"] == "present",
        "schedule": {
            "starts_at": fields["schedule.starts_at"],
            "ends_at": opt(fields["schedule.ends_at"]),
            "completed_at": opt(fields["schedule.completed_at"]),
            "timezone_id": fields["schedule.timezone_id"],
            "utc_offset_at_start": fields["schedule.utc_offset_at_start"],
            "latest_refresh_at": opt(fields["schedule.latest_refresh_at"]),
            "refresh_age": fields["schedule.refresh_age"],
        },
        "scope": {
            "deployment_profiles": _split(fields["scope.deployment_profiles"]),
            "tenant_refs": _split(fields["scope.tenant_refs"]),
            "region_refs": _split(fields["scope.region_refs"]),
            "residency_scope_classes": _split(fields["scope.residency_scope_classes"]),
            "service_classes": _split(fields["scope.service_classes"]),
        },
        "changed_axes": changed_axes,
        "blocked_writes": blocked_writes,
        "succeeded_hosted_mutations": succeeded,
        "summary_counts": summary_counts,
        "history_ref": fields["history_ref"],
        "support_export_ref": fields["support_export_ref"],
    }


def render_cli_index(view: dict[str, Any], fixture_filename: str) -> str:
    scenario_id = view["view_id"].rsplit(".", 1)[-1]
    counts = view["summary_counts"]
    honesty = "honesty=present" if view["honesty_marker_present"] else "honesty=none"
    return "\t".join(
        [
            scenario_id,
            view["notice_kind"],
            view["category"],
            view["effective_freshness"],
            honesty,
            f"unresolved={_b(view['boundary_change_unresolved'])}",
            f"preserved={counts['preserved_intent_count']}",
            f"changed_axes={counts['changed_boundary_axis_count']}",
            f"blocked={counts['blocked_write_count']}",
            fixture_filename,
        ]
    )


def digest_from_cli_index(line: str) -> dict[str, Any]:
    parts = line.split("\t")
    return {
        "effective_freshness": parts[3],
        "honesty_marker_present": parts[4] == "honesty=present",
        "boundary_change_unresolved": parts[5].split("=", 1)[1] == "true",
        "preserved_intent_count": int(parts[6].split("=", 1)[1]),
        "changed_boundary_axis_count": int(parts[7].split("=", 1)[1]),
        "blocked_write_count": int(parts[8].split("=", 1)[1]),
    }


def check_parity(views_by_scenario: dict[str, tuple[str, dict[str, Any]]]) -> list[Finding]:
    findings: list[Finding] = []
    for scenario_id, (source_fixture, view) in sorted(views_by_scenario.items()):
        ui_full = digest_from_view(view)
        support_text = render_support_export(view)
        cli_line = render_cli_index(view, Path(source_fixture).name)
        if digest_from_support_export(support_text) != ui_full:
            findings.append(
                err(
                    "maintenance_failover.parity.support_export",
                    f"{scenario_id}: support-export plaintext loses semantics from the "
                    "UI record",
                    "Keep the support-export projection faithful to the record.",
                    source_fixture,
                    diff=_first_diff(ui_full, digest_from_support_export(support_text)),
                )
            )
        if digest_from_cli_index(cli_line) != coarse_digest(ui_full):
            findings.append(
                err(
                    "maintenance_failover.parity.cli_index",
                    f"{scenario_id}: CLI / headless index loses semantics from the UI "
                    "record",
                    "Keep the CLI / headless index faithful to the record.",
                    source_fixture,
                )
            )
    return findings


# --------------------------------------------------------------------------- #
# Matrix + parity packet (build / drift-check)
# --------------------------------------------------------------------------- #


def lane_properties(view: dict[str, Any]) -> list[str]:
    props = ["timezone_unambiguous", "export_parity", "no_generic_degraded_collapse"]
    if view["effective_freshness"] != "current":
        props.append("stale_downgrade")
    if view["summary_counts"]["preserved_intent_count"] > 0:
        props.append("queued_intent_preserved")
    if any(w["continuity_posture"] == "draining_existing_only" for w in view["blocked_writes"]):
        props.append("drain_window")
    if any(
        a["axis_class"] == "endpoint_identity"
        and a["axis_state_class"] in MEANINGFUL_AXIS_STATES
        for a in view["boundary_change"]["axes"]
    ):
        props.append("changed_endpoint_visible")
    if view["boundary_change"]["changed_axis_count"] + view["boundary_change"]["unknown_axis_count"] > 0:
        props.append("boundary_preserved_after_recovery")
    if any(
        w["continuity_posture"] not in AUTO_REPLAY_POSTURES
        and w["resume_trigger"] in BOUNDARY_RECHECK_TRIGGERS
        for w in view["blocked_writes"]
    ):
        props.append("no_silent_replay_across_boundary")
    return sorted(set(props))


def build_matrix(views_by_scenario: dict[str, tuple[str, dict[str, Any]]]) -> dict[str, Any]:
    cases = []
    claimed_rows = []
    for scenario_id, (source_fixture, view) in sorted(views_by_scenario.items()):
        counts = view["summary_counts"]
        claimed = CLAIMED_BETA_ROWS.get(scenario_id, "")
        cases.append(
            {
                "scenario_id": scenario_id,
                "claimed_beta_row": claimed,
                "notice_kind": view["notice_kind"],
                "category": view["category"],
                "plan_class": view["plan_class"],
                "source_fixture": source_fixture,
                "effective_freshness": view["effective_freshness"],
                "freshness_downgraded": view["freshness_downgraded"],
                "boundary_change_unresolved": view["boundary_change_unresolved"],
                "honesty_marker_present": view["honesty_marker_present"],
                "preserved_intent_count": counts["preserved_intent_count"],
                "changed_boundary_axis_count": counts["changed_boundary_axis_count"],
                "unknown_boundary_axis_count": counts["unknown_boundary_axis_count"],
                "blocked_write_count": counts["blocked_write_count"],
                "timezone_id": view["schedule"]["timezone_id"],
                "utc_offset_at_start": view["schedule"]["utc_offset_at_start"],
                "lane_properties": lane_properties(view),
            }
        )
        claimed_rows.append({"claimed_beta_row": claimed, "scenario_id": scenario_id})
    claimed_rows.sort(key=lambda r: r["claimed_beta_row"])
    return {
        "record_kind": MATRIX_RECORD_KIND,
        "schema_version": 1,
        "shared_contract_ref": AUDIT_CONTRACT_REF,
        "view_schema_ref": SCHEMA_REL,
        "corpus_dir": CORPUS_DIR_REL,
        "validator_ref": VALIDATOR_REL,
        "contract_doc_ref": CONTRACT_DOC_REL,
        "claimed_beta_rows": claimed_rows,
        "drill_cases": cases,
    }


def build_parity_packet(views_by_scenario: dict[str, tuple[str, dict[str, Any]]]) -> dict[str, Any]:
    entries = []
    for scenario_id, (source_fixture, view) in sorted(views_by_scenario.items()):
        support_text = render_support_export(view)
        cli_line = render_cli_index(view, Path(source_fixture).name)
        ui_full = digest_from_view(view)
        entries.append(
            {
                "scenario_id": scenario_id,
                "claimed_beta_row": CLAIMED_BETA_ROWS.get(scenario_id, ""),
                "notice_kind": view["notice_kind"],
                "source_fixture": source_fixture,
                "semantic_digest": ui_full,
                "support_export_text": support_text,
                "cli_index_line": cli_line,
                "parity": {
                    "ui_record_full": True,
                    "support_export_full": digest_from_support_export(support_text) == ui_full,
                    "cli_index_coarse": digest_from_cli_index(cli_line) == coarse_digest(ui_full),
                },
            }
        )
    return {
        "record_kind": PARITY_RECORD_KIND,
        "schema_version": 1,
        "shared_contract_ref": AUDIT_CONTRACT_REF,
        "note": (
            "For every scenario the support-bundle plaintext preserves the full "
            "semantic digest and the CLI / headless index preserves the coarse "
            "digest of the product UI record, so support can explain maintenance / "
            "failover outcomes without rehydrating live control-plane state. "
            "Regenerate with `ci/check_maintenance_failover_corpus.py --write`."
        ),
        "scenarios": entries,
    }


# --------------------------------------------------------------------------- #
# Beta scorecard — every claimed row maps to exactly one packet, and back.
# --------------------------------------------------------------------------- #


def validate_scorecard(views_by_scenario: dict[str, tuple[str, dict[str, Any]]]) -> list[Finding]:
    findings: list[Finding] = []
    scenarios = set(views_by_scenario)
    expected = set(CLAIMED_BETA_ROWS)
    missing = expected - scenarios
    extra = scenarios - expected
    for scenario_id in sorted(missing):
        findings.append(
            err(
                "maintenance_failover.scorecard.missing_packet",
                f"claimed beta row maps to scenario {scenario_id} but no packet is present",
                "Mint the missing packet with --write.",
            )
        )
    for scenario_id in sorted(extra):
        findings.append(
            err(
                "maintenance_failover.scorecard.unclaimed_packet",
                f"packet {scenario_id} has no claimed beta row",
                "Map every packet to exactly one claimed managed / hybrid beta row.",
            )
        )
    rows = list(CLAIMED_BETA_ROWS.values())
    dupes = sorted({r for r in rows if rows.count(r) > 1})
    for row in dupes:
        findings.append(
            err(
                "maintenance_failover.scorecard.duplicate_row",
                f"claimed beta row {row} maps to more than one packet",
                "Each claimed row gets exactly one current maintenance / failover packet.",
            )
        )
    return findings


# --------------------------------------------------------------------------- #
# Loading + schema validation + companions
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


def make_validator(repo_root: Path) -> Draft202012Validator:
    schema = load_json(repo_root / SCHEMA_REL)
    Draft202012Validator.check_schema(schema)
    return Draft202012Validator(schema, format_checker=FormatChecker())


def schema_validate(validator: Draft202012Validator, label: str, view: Any) -> list[Finding]:
    findings: list[Finding] = []
    for error in sorted(validator.iter_errors(view), key=lambda e: list(e.path)):
        path = ".".join(str(p) for p in error.path) or "<root>"
        findings.append(
            err(
                "maintenance_failover.schema.validation_failed",
                f"{label}: {path}: {error.message}",
                f"Fix the record so it validates against {SCHEMA_REL}.",
                label,
            )
        )
    return findings


def collect_views(repo_root: Path) -> dict[str, tuple[str, dict[str, Any]]]:
    """scenario_id -> (repo-relative source fixture path, view record)."""
    result: dict[str, tuple[str, dict[str, Any]]] = {}
    corpus_dir = repo_root / CORPUS_DIR_REL
    if not corpus_dir.is_dir():
        return result
    for path in sorted(corpus_dir.glob("*.json")):
        if path.name in NON_VIEW_FILES:
            continue
        record = load_json(path)
        if record.get("record_kind") != VIEW_RECORD_KIND:
            continue
        scenario_id = record["view_id"].rsplit(".", 1)[-1]
        result[scenario_id] = (f"{CORPUS_DIR_REL}/{path.name}", record)
    return result


def validate_companion(path: Path, kind: str, expected: Any) -> list[Finding]:
    if not path.exists():
        return [
            err(
                f"maintenance_failover.{kind}.missing",
                f"{kind} is missing: {path}",
                "Generate it with --write.",
                str(path),
            )
        ]
    actual = load_json(path)
    if actual != expected:
        return [
            err(
                f"maintenance_failover.{kind}.drift",
                f"{kind} has drifted from the corpus; regenerate with --write",
                "Run ci/check_maintenance_failover_corpus.py --write and commit the result.",
                str(path),
                diff=_first_diff(actual, expected),
            )
        ]
    return []


def validate_report_and_readme(
    report_path: Path, readme_path: Path, scenario_ids: list[str]
) -> list[Finding]:
    findings: list[Finding] = []
    required_refs = [SCHEMA_REL, CORPUS_DIR_REL, VALIDATOR_REL, SCRIPT_REL]
    for kind, path, refs in (
        ("report", report_path, required_refs),
        ("readme", readme_path, [VALIDATOR_REL, SCHEMA_REL]),
    ):
        if not path.exists():
            findings.append(
                err(
                    f"maintenance_failover.{kind}.missing",
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
                        f"maintenance_failover.{kind}.missing_ref",
                        f"{kind} does not mention {ref}",
                        "Keep the report / README pointing at the schema, corpus, "
                        "validator, and script.",
                        str(path),
                    )
                )
        for scenario_id in scenario_ids:
            if scenario_id not in text:
                findings.append(
                    err(
                        f"maintenance_failover.{kind}.missing_scenario",
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
    for drill in drills():
        dump_json(corpus_dir / drill.fixture_filename, drill.view())
        print(f"wrote {CORPUS_DIR_REL}/{drill.fixture_filename}")
    views_by_scenario = collect_views(repo_root)
    dump_json(repo_root / CORPUS_MATRIX_REL, build_matrix(views_by_scenario))
    print(f"wrote {CORPUS_MATRIX_REL}")
    dump_json(repo_root / PARITY_PACKET_REL, build_parity_packet(views_by_scenario))
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

    schema_path = repo_root / SCHEMA_REL
    if not schema_path.exists():
        print(f"[maintenance-failover] missing schema: {schema_path}", file=sys.stderr)
        return 2

    validator = make_validator(repo_root)
    views_by_scenario = collect_views(repo_root)
    if not views_by_scenario:
        print("[maintenance-failover] no continuity-notice packets found", file=sys.stderr)
        return 2

    findings: list[Finding] = []
    flat_views: dict[str, dict[str, Any]] = {}
    schema_findings: list[Finding] = []
    for scenario_id, (source_fixture, view) in sorted(views_by_scenario.items()):
        flat_views[source_fixture] = view
        these = schema_validate(validator, source_fixture, view)
        schema_findings.extend(these)
        findings.extend(these)
        findings.extend(check_view(source_fixture, view))

    if not schema_findings:
        findings.extend(validate_coverage(flat_views))
        findings.extend(check_parity(views_by_scenario))
        findings.extend(validate_scorecard(views_by_scenario))

        findings.extend(
            validate_companion(
                repo_root / CORPUS_MATRIX_REL, "corpus_matrix", build_matrix(views_by_scenario)
            )
        )
        findings.extend(
            validate_companion(
                repo_root / PARITY_PACKET_REL, "parity_packet", build_parity_packet(views_by_scenario)
            )
        )

    scenario_ids = sorted(views_by_scenario)
    findings.extend(
        validate_report_and_readme(repo_root / REPORT_REL, repo_root / README_REL, scenario_ids)
    )

    errors = [f for f in findings if f.severity == "error"]
    report = {
        "lane": "maintenance_window_and_failover_communication_corpus",
        "schema": SCHEMA_REL,
        "corpus_dir": CORPUS_DIR_REL,
        "scenario_count": len(views_by_scenario),
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
            f"[maintenance-failover] FAIL: {len(errors)} error(s) across "
            f"{len(views_by_scenario)} packet(s)",
            file=sys.stderr,
        )
        for f in errors:
            print(f"  - {f.check_id}: {f.message}", file=sys.stderr)
        return 1

    print(
        f"[maintenance-failover] PASS: {len(views_by_scenario)} packet(s) validated, "
        "re-derived, parity- and scorecard-checked"
    )
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
