#!/usr/bin/env python3
"""Generate the M05-007 JSON artifact."""

import json
from datetime import datetime, timezone

AS_OF = datetime.now(timezone.utc).strftime("%Y-%m-%d")

LANE_KINDS = [
    "notebook",
    "data_rich",
    "ai_adjacent",
    "framework",
    "review",
    "companion",
    "managed_depth",
]

LIFECYCLE_LABELS = ["lts", "stable", "beta", "preview", "withdrawn"]
ROLLBACK_PATH_STATES = ["defined", "tested", "exercised", "missing"]
DOWNGRADE_KINDS = ["automatic_narrowing", "manual_hold", "emergency_rollback", "staged_reversal"]
PROMOTION_STAGE_KINDS = ["canary", "pilot", "stable"]
STAGE_STATES = ["complete", "in_progress", "not_started", "blocked"]
GAP_REASONS = [
    "rollback_path_missing",
    "rollback_path_untested",
    "rollback_path_unexercised",
    "downgrade_rule_missing",
    "claim_narrowing_rule_missing",
    "staged_promotion_rule_missing",
    "promotion_stage_blocked",
    "proof_packet_missing",
    "proof_packet_stale",
    "waiver_expired",
    "owner_signoff_missing",
]
STOP_RULE_ACTIONS = [
    "hold_promotion",
    "narrow_label",
    "define_rollback_path",
    "test_rollback_path",
    "exercise_rollback_path",
    "define_downgrade_rule",
    "define_claim_narrowing_rule",
    "define_staged_promotion_rule",
    "advance_promotion_stage",
    "refresh_proof_packet",
    "request_owner_signoff",
]

STOP_RULES = [
    {
        "rule_id": "stop-rollback-path-missing",
        "title": "Rollback path must be defined",
        "trigger_reason": "rollback_path_missing",
        "applies_to_labels": ["lts", "stable"],
        "default_action": "define_rollback_path",
        "blocks_promotion": True,
        "rationale": "A lane without a defined rollback path cannot safely ship.",
    },
    {
        "rule_id": "stop-rollback-path-untested",
        "title": "Rollback path must be tested",
        "trigger_reason": "rollback_path_untested",
        "applies_to_labels": ["lts", "stable"],
        "default_action": "test_rollback_path",
        "blocks_promotion": True,
        "rationale": "A defined but untested rollback path is not sufficient for stable promotion.",
    },
    {
        "rule_id": "stop-rollback-path-unexercised",
        "title": "Rollback path should be exercised",
        "trigger_reason": "rollback_path_unexercised",
        "applies_to_labels": ["lts"],
        "default_action": "exercise_rollback_path",
        "blocks_promotion": False,
        "rationale": "LTS lanes should have exercised rollback paths; stable lanes may proceed with tested only.",
    },
    {
        "rule_id": "stop-downgrade-rule-missing",
        "title": "Downgrade rule must be defined",
        "trigger_reason": "downgrade_rule_missing",
        "applies_to_labels": ["lts", "stable"],
        "default_action": "define_downgrade_rule",
        "blocks_promotion": True,
        "rationale": "Every lane must have a defined downgrade rule before stable promotion.",
    },
    {
        "rule_id": "stop-claim-narrowing-rule-missing",
        "title": "Claim-narrowing rule must be defined",
        "trigger_reason": "claim_narrowing_rule_missing",
        "applies_to_labels": ["lts", "stable"],
        "default_action": "define_claim_narrowing_rule",
        "blocks_promotion": True,
        "rationale": "Automatic claim-narrowing rules are required for safe staged promotion.",
    },
    {
        "rule_id": "stop-staged-promotion-rule-missing",
        "title": "Staged-promotion rule must be defined",
        "trigger_reason": "staged_promotion_rule_missing",
        "applies_to_labels": ["lts", "stable"],
        "default_action": "define_staged_promotion_rule",
        "blocks_promotion": True,
        "rationale": "Staged promotion rules govern canary-to-pilot-to-stable progression.",
    },
    {
        "rule_id": "stop-promotion-stage-blocked",
        "title": "Promotion stage must not be blocked",
        "trigger_reason": "promotion_stage_blocked",
        "applies_to_labels": ["lts", "stable"],
        "default_action": "advance_promotion_stage",
        "blocks_promotion": True,
        "rationale": "A blocked promotion stage prevents safe rollout.",
    },
    {
        "rule_id": "stop-proof-packet-missing",
        "title": "Proof packet must be present",
        "trigger_reason": "proof_packet_missing",
        "applies_to_labels": ["lts", "stable"],
        "default_action": "refresh_proof_packet",
        "blocks_promotion": True,
        "rationale": "A lane without a proof packet cannot demonstrate its rollback posture.",
    },
    {
        "rule_id": "stop-proof-packet-stale",
        "title": "Proof packet must be current",
        "trigger_reason": "proof_packet_stale",
        "applies_to_labels": ["lts", "stable"],
        "default_action": "refresh_proof_packet",
        "blocks_promotion": True,
        "rationale": "Stale proof packets may hide regressions in rollback behavior.",
    },
    {
        "rule_id": "stop-waiver-expired",
        "title": "Waiver must not be expired",
        "trigger_reason": "waiver_expired",
        "applies_to_labels": ["lts", "stable"],
        "default_action": "hold_promotion",
        "blocks_promotion": True,
        "rationale": "An expired waiver removes provisional authorization to proceed.",
    },
    {
        "rule_id": "stop-owner-signoff-missing",
        "title": "Owner sign-off must be present",
        "trigger_reason": "owner_signoff_missing",
        "applies_to_labels": ["lts", "stable"],
        "default_action": "request_owner_signoff",
        "blocks_promotion": True,
        "rationale": "Rollback and downgrade rules require owner sign-off before promotion.",
    },
]

RELEASE_BLOCKING = [
    "m5-notebook-rollback",
    "m5-ai-adjacent-rollback",
    "m5-companion-rollback",
]

def make_proof_packet(packet_id, slo_state="current"):
    captured = None if slo_state == "missing" else AS_OF
    evidence_refs = [] if slo_state == "missing" else [f"evidence/{packet_id}"]
    return {
        "packet_id": packet_id,
        "packet_ref": f"proof/{packet_id}",
        "proof_index_ref": f"proof-index/{packet_id}",
        "captured_at": captured,
        "freshness_slo": {
            "target_max_age_days": 30,
            "warn_within_days": 7,
            "slo_register_ref": "freshness-slo/register",
        },
        "slo_state": slo_state,
        "evidence_refs": evidence_refs,
    }

def make_owner_signoff(signed_off=True):
    return {
        "owner_ref": "release-engineering",
        "signed_off": signed_off,
        "signed_at": AS_OF if signed_off else None,
    }

def make_downgrade_rules(entry_id):
    return [
        {
            "rule_id": f"{entry_id}-downgrade-auto",
            "downgrade_kind": "automatic_narrowing",
            "trigger_label": "stable",
            "target_label": "beta",
            "rule_ref": f"rules/{entry_id}/downgrade-auto",
            "rationale": "Automatic narrowing to beta when rollback evidence goes stale.",
        },
        {
            "rule_id": f"{entry_id}-downgrade-emergency",
            "downgrade_kind": "emergency_rollback",
            "trigger_label": "lts",
            "target_label": "stable",
            "rule_ref": f"rules/{entry_id}/downgrade-emergency",
            "rationale": "Emergency rollback from LTS to stable on critical defect.",
        },
    ]

def make_claim_narrowing_rules(entry_id):
    return [
        {
            "rule_id": f"{entry_id}-narrow-rollback-missing",
            "trigger_reason": "rollback_path_missing",
            "target_label": "preview",
            "auto_apply": True,
            "rule_ref": f"rules/{entry_id}/narrow-rollback-missing",
            "rationale": "Narrow to preview when rollback path is missing.",
        },
        {
            "rule_id": f"{entry_id}-narrow-packet-stale",
            "trigger_reason": "proof_packet_stale",
            "target_label": "beta",
            "auto_apply": True,
            "rule_ref": f"rules/{entry_id}/narrow-packet-stale",
            "rationale": "Narrow to beta when proof packet goes stale.",
        },
    ]

def make_promotion_stages(entry_id, states=None):
    if states is None:
        states = ["complete", "complete", "complete"]
    stages = []
    for kind, state in zip(PROMOTION_STAGE_KINDS, states):
        completed = AS_OF if state == "complete" else None
        stages.append({
            "stage_kind": kind,
            "stage_state": state,
            "stage_ref": f"stages/{entry_id}/{kind}",
            "completed_at": completed,
            "rationale": f"{kind} stage is {state}",
        })
    return stages

ROWS = [
    {
        "entry_id": "m5-notebook-rollback",
        "title": "Notebook rollback and downgrade rules",
        "lane_kind": "notebook",
        "surface_ref": "m5-notebook-rollback",
        "surface_summary": "Rollback, downgrade, and staged-promotion rules for notebook surfaces.",
        "release_blocking": True,
        "claim_ref": "claim-m5-notebook",
        "claim_label": "stable",
        "rollback_path_state": "exercised",
        "rollback_path_ref": "rollback/notebook",
        "downgrade_rules": make_downgrade_rules("m5-notebook"),
        "claim_narrowing_rules": make_claim_narrowing_rules("m5-notebook"),
        "promotion_stages": make_promotion_stages("m5-notebook"),
        "proof_packet": make_proof_packet("m5-notebook-rollback"),
        "waiver": None,
        "owner_signoff": make_owner_signoff(),
        "active_gap_reasons": [],
        "published_label": "stable",
        "rationale": "Notebook lane has exercised rollback path, complete downgrade rules, and complete promotion stages.",
    },
    {
        "entry_id": "m5-data-rich-rollback",
        "title": "Data-rich rollback and downgrade rules",
        "lane_kind": "data_rich",
        "surface_ref": "m5-data-rich-rollback",
        "surface_summary": "Rollback, downgrade, and staged-promotion rules for data-rich surfaces.",
        "release_blocking": False,
        "claim_ref": "claim-m5-data-rich",
        "claim_label": "stable",
        "rollback_path_state": "tested",
        "rollback_path_ref": "rollback/data-rich",
        "downgrade_rules": make_downgrade_rules("m5-data-rich"),
        "claim_narrowing_rules": make_claim_narrowing_rules("m5-data-rich"),
        "promotion_stages": make_promotion_stages("m5-data-rich"),
        "proof_packet": make_proof_packet("m5-data-rich-rollback"),
        "waiver": None,
        "owner_signoff": make_owner_signoff(),
        "active_gap_reasons": [],
        "published_label": "stable",
        "rationale": "Data-rich lane has tested rollback path and complete rules.",
    },
    {
        "entry_id": "m5-ai-adjacent-rollback",
        "title": "AI-adjacent rollback and downgrade rules",
        "lane_kind": "ai_adjacent",
        "surface_ref": "m5-ai-adjacent-rollback",
        "surface_summary": "Rollback, downgrade, and staged-promotion rules for AI-adjacent surfaces.",
        "release_blocking": True,
        "claim_ref": "claim-m5-ai-adjacent",
        "claim_label": "stable",
        "rollback_path_state": "exercised",
        "rollback_path_ref": "rollback/ai-adjacent",
        "downgrade_rules": make_downgrade_rules("m5-ai-adjacent"),
        "claim_narrowing_rules": make_claim_narrowing_rules("m5-ai-adjacent"),
        "promotion_stages": make_promotion_stages("m5-ai-adjacent"),
        "proof_packet": make_proof_packet("m5-ai-adjacent-rollback"),
        "waiver": None,
        "owner_signoff": make_owner_signoff(),
        "active_gap_reasons": [],
        "published_label": "stable",
        "rationale": "AI-adjacent lane has exercised rollback path and complete rules.",
    },
    {
        "entry_id": "m5-framework-rollback",
        "title": "Framework rollback and downgrade rules",
        "lane_kind": "framework",
        "surface_ref": "m5-framework-rollback",
        "surface_summary": "Rollback, downgrade, and staged-promotion rules for framework surfaces.",
        "release_blocking": False,
        "claim_ref": "claim-m5-framework",
        "claim_label": "stable",
        "rollback_path_state": "tested",
        "rollback_path_ref": "rollback/framework",
        "downgrade_rules": make_downgrade_rules("m5-framework"),
        "claim_narrowing_rules": make_claim_narrowing_rules("m5-framework"),
        "promotion_stages": make_promotion_stages("m5-framework"),
        "proof_packet": make_proof_packet("m5-framework-rollback"),
        "waiver": None,
        "owner_signoff": make_owner_signoff(),
        "active_gap_reasons": [],
        "published_label": "stable",
        "rationale": "Framework lane has tested rollback path and complete rules.",
    },
    {
        "entry_id": "m5-review-rollback",
        "title": "Review rollback and downgrade rules",
        "lane_kind": "review",
        "surface_ref": "m5-review-rollback",
        "surface_summary": "Rollback, downgrade, and staged-promotion rules for review surfaces.",
        "release_blocking": False,
        "claim_ref": "claim-m5-review",
        "claim_label": "beta",
        "rollback_path_state": "defined",
        "rollback_path_ref": "rollback/review",
        "downgrade_rules": make_downgrade_rules("m5-review"),
        "claim_narrowing_rules": make_claim_narrowing_rules("m5-review"),
        "promotion_stages": make_promotion_stages("m5-review", ["complete", "complete", "not_started"]),
        "proof_packet": make_proof_packet("m5-review-rollback", "due_for_refresh"),
        "waiver": None,
        "owner_signoff": make_owner_signoff(),
        "active_gap_reasons": ["rollback_path_untested", "proof_packet_stale"],
        "published_label": "beta",
        "rationale": "Review lane is narrowed to beta due to untested rollback path and stale proof packet.",
    },
    {
        "entry_id": "m5-companion-rollback",
        "title": "Companion rollback and downgrade rules",
        "lane_kind": "companion",
        "surface_ref": "m5-companion-rollback",
        "surface_summary": "Rollback, downgrade, and staged-promotion rules for companion surfaces.",
        "release_blocking": True,
        "claim_ref": "claim-m5-companion",
        "claim_label": "stable",
        "rollback_path_state": "exercised",
        "rollback_path_ref": "rollback/companion",
        "downgrade_rules": make_downgrade_rules("m5-companion"),
        "claim_narrowing_rules": make_claim_narrowing_rules("m5-companion"),
        "promotion_stages": make_promotion_stages("m5-companion"),
        "proof_packet": make_proof_packet("m5-companion-rollback"),
        "waiver": None,
        "owner_signoff": make_owner_signoff(),
        "active_gap_reasons": [],
        "published_label": "stable",
        "rationale": "Companion lane has exercised rollback path and complete rules.",
    },
    {
        "entry_id": "m5-managed-depth-rollback",
        "title": "Managed-depth rollback and downgrade rules",
        "lane_kind": "managed_depth",
        "surface_ref": "m5-managed-depth-rollback",
        "surface_summary": "Rollback, downgrade, and staged-promotion rules for managed-depth surfaces.",
        "release_blocking": False,
        "claim_ref": "claim-m5-managed-depth",
        "claim_label": "preview",
        "rollback_path_state": "missing",
        "rollback_path_ref": "",
        "downgrade_rules": [],
        "claim_narrowing_rules": [],
        "promotion_stages": make_promotion_stages("m5-managed-depth", ["complete", "blocked", "not_started"]),
        "proof_packet": make_proof_packet("m5-managed-depth-rollback", "missing"),
        "waiver": None,
        "owner_signoff": make_owner_signoff(False),
        "active_gap_reasons": [
            "rollback_path_missing",
            "downgrade_rule_missing",
            "claim_narrowing_rule_missing",
            "staged_promotion_rule_missing",
            "promotion_stage_blocked",
            "proof_packet_missing",
            "owner_signoff_missing",
        ],
        "published_label": "preview",
        "rationale": "Managed-depth lane is narrowed to preview due to missing rollback path, rules, blocked stage, and missing proof packet.",
    },
]

# Compute summary
entries_holding_stable = sum(1 for r in ROWS if r["published_label"] in ("lts", "stable"))
entries_narrowed = sum(1 for r in ROWS if r["published_label"] not in ("lts", "stable"))
entries_on_waiver = sum(1 for r in ROWS if r.get("waiver") is not None)
entries_blocked = sum(1 for r in ROWS if any(g == "promotion_stage_blocked" for g in r["active_gap_reasons"]))
entries_rule_missing = sum(1 for r in ROWS if any(g in ("downgrade_rule_missing", "claim_narrowing_rule_missing", "staged_promotion_rule_missing") for g in r["active_gap_reasons"]))
release_blocking_total = sum(1 for r in ROWS if r["release_blocking"])
release_blocking_holding = sum(1 for r in ROWS if r["release_blocking"] and r["published_label"] in ("lts", "stable"))
release_blocking_narrowed = sum(1 for r in ROWS if r["release_blocking"] and r["published_label"] not in ("lts", "stable"))

from collections import Counter
kind_counts = Counter(r["lane_kind"] for r in ROWS)

packets_current = sum(1 for r in ROWS if r["proof_packet"]["slo_state"] == "current")
packets_due = sum(1 for r in ROWS if r["proof_packet"]["slo_state"] == "due_for_refresh")
packets_breached = sum(1 for r in ROWS if r["proof_packet"]["slo_state"] == "breached")
packets_missing = sum(1 for r in ROWS if r["proof_packet"]["slo_state"] == "missing")

total_gap_reasons = sum(len(r["active_gap_reasons"]) for r in ROWS)

def rule_fires(rule):
    for row in ROWS:
        if row["claim_label"] in rule["applies_to_labels"] and rule["trigger_reason"] in row["active_gap_reasons"]:
            return True
    return False

rules_firing = sum(1 for rule in STOP_RULES if rule_fires(rule))

total_downgrade_rules = sum(len(r["downgrade_rules"]) for r in ROWS)
total_claim_narrowing_rules = sum(len(r["claim_narrowing_rules"]) for r in ROWS)
total_promotion_stages = sum(len(r["promotion_stages"]) for r in ROWS)

rollback_path_missing = sum(1 for r in ROWS if r["rollback_path_state"] == "missing")
rollback_path_untested = sum(1 for r in ROWS if r["rollback_path_state"] == "defined")
rollback_path_unexercised = sum(1 for r in ROWS if r["rollback_path_state"] == "tested")

stages_blocked = sum(1 for r in ROWS if any(s["stage_state"] == "blocked" for s in r["promotion_stages"]))

blocking_rule_ids = sorted(rule["rule_id"] for rule in STOP_RULES if rule["blocks_promotion"] and rule_fires(rule))

blocking_triggers = set(rule["trigger_reason"] for rule in STOP_RULES if rule["blocks_promotion"] and rule_fires(rule))
blocking_claim_ids = sorted(set(
    r["entry_id"] for r in ROWS
    if r["claim_label"] in ("lts", "stable") and any(g in blocking_triggers for g in r["active_gap_reasons"])
))

promotion_decision = "hold" if blocking_rule_ids else "proceed"

SUMMARY = {
    "total_entries": len(ROWS),
    "total_claims": len(set(r["claim_ref"] for r in ROWS)),
    "entries_holding_stable": entries_holding_stable,
    "entries_narrowed": entries_narrowed,
    "entries_on_active_waiver": entries_on_waiver,
    "entries_blocked": entries_blocked,
    "entries_rule_missing": entries_rule_missing,
    "release_blocking_total": release_blocking_total,
    "release_blocking_holding": release_blocking_holding,
    "release_blocking_narrowed": release_blocking_narrowed,
    "notebook_entries": kind_counts["notebook"],
    "data_rich_entries": kind_counts["data_rich"],
    "ai_adjacent_entries": kind_counts["ai_adjacent"],
    "framework_entries": kind_counts["framework"],
    "review_entries": kind_counts["review"],
    "companion_entries": kind_counts["companion"],
    "managed_depth_entries": kind_counts["managed_depth"],
    "packets_current": packets_current,
    "packets_due_for_refresh": packets_due,
    "packets_breached": packets_breached,
    "packets_missing": packets_missing,
    "total_active_gap_reasons": total_gap_reasons,
    "rules_firing": rules_firing,
    "total_downgrade_rules": total_downgrade_rules,
    "total_claim_narrowing_rules": total_claim_narrowing_rules,
    "total_promotion_stages": total_promotion_stages,
    "rollback_path_missing": rollback_path_missing,
    "rollback_path_untested": rollback_path_untested,
    "rollback_path_unexercised": rollback_path_unexercised,
    "stages_blocked": stages_blocked,
}

REGISTER = {
    "schema_version": 1,
    "record_kind": "freeze_the_m5_rollback_downgrade_claim_narrowing_and_staged_promotion_rules",
    "register_id": "m5-rollback-downgrade-register",
    "status": "active",
    "overview_page": "docs/m5/freeze_the_m5_rollback_downgrade_claim_narrowing_and_staged_promotion_rules.md",
    "as_of": AS_OF,
    "claim_manifest_ref": "artifacts/release/stable/stable_claim_manifest.json",
    "feature_train_matrix_ref": "artifacts/release/m5/freeze_the_m5_feature_train_matrix_scorecards_and_dependency_graph.json",
    "lifecycle_labels": LIFECYCLE_LABELS,
    "lane_kinds": LANE_KINDS,
    "rollback_path_states": ROLLBACK_PATH_STATES,
    "downgrade_kinds": DOWNGRADE_KINDS,
    "promotion_stage_kinds": PROMOTION_STAGE_KINDS,
    "stage_states": STAGE_STATES,
    "gap_reasons": GAP_REASONS,
    "stop_rule_actions": STOP_RULE_ACTIONS,
    "launch_cutline": {
        "cutline_level": "stable",
        "above_cutline_levels": ["lts", "stable"],
        "below_cutline_levels": ["beta", "preview", "withdrawn"],
        "description": "The stable cutline for M5 rollback, downgrade, and staged-promotion rules. Only lanes at or above stable may ship broadly.",
    },
    "release_blocking_lane_refs": RELEASE_BLOCKING,
    "stop_rules": STOP_RULES,
    "rows": ROWS,
    "promotion": {
        "promotion_gate": "m5-rollback-downgrade-gate",
        "decision": promotion_decision,
        "blocking_rule_ids": blocking_rule_ids,
        "blocking_claim_ids": blocking_claim_ids,
        "rationale": "Computed from firing stop rules over rollback path, downgrade rule, claim-narrowing rule, staged-promotion rule, promotion stage, proof packet, waiver, and owner sign-off gaps.",
    },
    "summary": SUMMARY,
}

with open("artifacts/release/m5/freeze_the_m5_rollback_downgrade_claim_narrowing_and_staged_promotion_rules.json", "w") as f:
    json.dump(REGISTER, f, indent=2)

print("Generated artifact with", len(ROWS), "rows")
print("Promotion decision:", promotion_decision)
print("Blocking rules:", blocking_rule_ids)
print("Blocking claims:", blocking_claim_ids)
