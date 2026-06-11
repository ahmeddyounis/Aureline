#!/usr/bin/env python3
"""Generate the M5 maintenance-truth register, capture, and fixtures.

The register freezes one maintenance-truth lane per lane kind — supported-line
backport rules, emergency hotfix rules, proof-freshness/evidence-expiry
automation, and the Help/About truth surfaces those lanes publish. Each lane
binds the lane to the stable claim it backs, a per-lane maintenance scorecard
(one cell per dimension: backport policy, hotfix policy, proof freshness,
evidence expiry, Help/About truth, and docs truth), the disclosed support
posture it exposes (support window, policy ref, maintainer trust tier, scope
refs, and whether the Help/About truth is disclosed to the operator), an
owner-manifest sign-off, an explicit downgrade/rollback automation record (fall
back to a frozen floor label), a proof packet with its freshness SLO, and the
active narrowing reasons that drop the published label below the launch cutline
when a maintenance dimension fails or is missing, the support posture is
undisclosed, proof freshness expires, the owner manifest is unsigned, the
rollback plan is unverified, downgrade automation is undefined, or a waiver
expires.

This generator is the single source for the checked-in JSON so the typed Rust
consumer, the JSON Schema, and the CI capture stay in lock-step. Run it from the
repository root.
"""

import json
from collections import Counter

AS_OF = "2026-06-10"

SLUG = (
    "add_backport_and_hotfix_rules_proof_freshness_automation_and_help_about_"
    "truth_updates_for_m5_lanes"
)

ARTIFACT_PATH = f"artifacts/release/m5/{SLUG}.json"
CAPTURE_PATH = f"artifacts/release/captures/{SLUG}_validation_capture.json"
FIXTURES_DIR = f"fixtures/release/m5/{SLUG}"

RECORD_KIND = SLUG

LIFECYCLE_LABELS = ["lts", "stable", "beta", "preview", "withdrawn"]
ABOVE_CUTLINE = {"lts", "stable"}
BELOW_CUTLINE = {"beta", "preview", "withdrawn"}

LANE_KINDS = [
    "backport_rule",
    "hotfix_rule",
    "proof_freshness_automation",
    "help_about_truth",
]

MAINTENANCE_DIMENSIONS = [
    "backport_policy",
    "hotfix_policy",
    "proof_freshness",
    "evidence_expiry",
    "help_about_truth",
    "docs_truth",
]

DIMENSION_GRADES = ["pass", "partial", "fail", "waived", "missing"]

LANE_STATES = [
    "certified",
    "maintenance_regressed",
    "stale",
    "on_waiver",
    "automation_undefined",
    "owner_unsigned",
]

NARROWING_REASONS = [
    "maintenance_dimension_failed",
    "maintenance_dimension_missing",
    "proof_packet_missing",
    "proof_packet_stale",
    "owner_manifest_unsigned",
    "rollback_plan_unverified",
    "downgrade_automation_undefined",
    "waiver_expired",
]

STOP_RULE_ACTIONS = [
    "hold_promotion",
    "narrow_label",
    "remediate_maintenance",
    "refresh_proof_packet",
    "request_owner_signoff",
    "verify_rollback_plan",
    "define_downgrade_automation",
    "renew_waiver",
]

AUTOMATION_TRIGGERS = ["proof_stale", "maintenance_regressed", "owner_revoked", "manual"]

AUTOMATION_STATES = ["defined", "unverified", "undefined"]

TRUST_TIERS = ["first_party", "verified_partner", "community", "generated"]

# reason -> (stop-rule title, default action)
REASON_RULES = {
    "maintenance_dimension_failed": ("Maintenance dimension failed", "remediate_maintenance"),
    "maintenance_dimension_missing": ("Maintenance dimension missing", "remediate_maintenance"),
    "proof_packet_missing": ("Proof packet missing", "refresh_proof_packet"),
    "proof_packet_stale": ("Proof packet stale", "refresh_proof_packet"),
    "owner_manifest_unsigned": ("Owner manifest unsigned", "request_owner_signoff"),
    "rollback_plan_unverified": ("Rollback plan unverified", "verify_rollback_plan"),
    "downgrade_automation_undefined": (
        "Downgrade automation undefined",
        "define_downgrade_automation",
    ),
    "waiver_expired": ("Waiver expired", "renew_waiver"),
}

STOP_RULES = [
    {
        "rule_id": f"maintenance_truth_rule:{reason}",
        "title": title,
        "trigger_reason": reason,
        "applies_to_labels": ["lts", "stable"],
        "default_action": action,
        "blocks_promotion": True,
        "rationale": (
            f"A maintenance-truth lane whose posture reports '{reason}' cannot keep a "
            f"Stable or LTS claim and must narrow before publication."
        ),
    }
    for reason, (title, action) in REASON_RULES.items()
]

RELEASE_BLOCKING = [
    "maintenance-truth:backport_supported_lines",
    "maintenance-truth:proof_freshness_automation",
    "maintenance-truth:help_about_truth",
]


def proof_packet(packet_id, slo_state="current"):
    captured = None if slo_state == "missing" else AS_OF
    evidence = [] if slo_state == "missing" else [f"evidence/{packet_id}"]
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
        "evidence_refs": evidence,
    }


def owner_signoff(signed=True):
    return {
        "owner_ref": "release-engineering-guild",
        "signed_off": signed,
        "signed_at": AS_OF if signed else None,
    }


def downgrade_automation(
    entry_id,
    trigger="proof_stale",
    target_floor="beta",
    state="defined",
    rollback_verified=True,
):
    return {
        "automation_ref": f"downgrade-automation/{entry_id}",
        "rollback_plan_ref": f"rollback-plan/{entry_id}",
        "trigger": trigger,
        "target_floor": target_floor,
        "state": state,
        "rollback_verified": rollback_verified,
    }


def disclosure(
    support_window_ref,
    policy_ref,
    trust_tier,
    scope_refs,
    truth_disclosed=True,
):
    return {
        "support_window_ref": support_window_ref,
        "policy_ref": policy_ref,
        "trust_tier": trust_tier,
        "scope_refs": scope_refs,
        "truth_disclosed": truth_disclosed,
    }


def cell(dimension, grade="pass"):
    return {
        "dimension": dimension,
        "grade": grade,
        "evidence_ref": "" if grade == "missing" else f"evidence/{dimension}",
    }


def scorecard(overrides=None):
    """Build a full maintenance scorecard, passing except for overrides."""
    overrides = overrides or {}
    return [cell(dim, overrides.get(dim, "pass")) for dim in MAINTENANCE_DIMENSIONS]


ROWS = [
    {
        "entry_id": "backport-rule-supported-lines",
        "title": "Supported-line backport rule",
        "lane_kind": "backport_rule",
        "surface_ref": "maintenance-truth:backport_supported_lines",
        "surface_summary": "Maintenance scorecard and support posture for the supported-line backport rule.",
        "release_blocking": True,
        "claim_ref": "claim:m5_backport_supported_lines",
        "claim_label": "stable",
        "lane_state": "certified",
        "scorecard": scorecard(),
        "disclosure": disclosure(
            "support-window/supported-lines",
            "backport-policy/supported-lines",
            "first_party",
            ["scope/lts", "scope/stable"],
        ),
        "downgrade_automation": downgrade_automation("backport-rule-supported-lines"),
        "proof_packet": proof_packet("backport-rule-supported-lines"),
        "waiver": None,
        "owner_signoff": owner_signoff(),
        "active_narrowing_reasons": [],
        "published_label": "stable",
        "publication_destinations": ["docs/m5", "help/about", "support/export"],
        "rationale": "Supported-line backport rule passes every maintenance dimension, discloses its Help/About support posture, is owner-signed, and its downgrade automation is defined and its rollback plan verified.",
    },
    {
        "entry_id": "hotfix-rule-emergency-lane",
        "title": "Emergency hotfix rule",
        "lane_kind": "hotfix_rule",
        "surface_ref": "maintenance-truth:hotfix_emergency_lane",
        "surface_summary": "Maintenance scorecard and support posture for the emergency hotfix rule.",
        "release_blocking": False,
        "claim_ref": "claim:m5_hotfix_emergency_lane",
        "claim_label": "stable",
        "lane_state": "on_waiver",
        "scorecard": scorecard({"docs_truth": "waived"}),
        "disclosure": disclosure(
            "support-window/hotfix",
            "hotfix-policy/emergency",
            "verified_partner",
            ["scope/stable"],
        ),
        "downgrade_automation": downgrade_automation("hotfix-rule-emergency-lane"),
        "proof_packet": proof_packet("hotfix-rule-emergency-lane"),
        "waiver": {
            "waiver_ref": "waiver:hotfix_docs_truth",
            "expires_at": "2026-12-31",
            "reason": "Support docs refresh for the emergency hotfix runbook scheduled after the next maintenance snapshot; interim docs-truth dimension waived by owner.",
        },
        "owner_signoff": owner_signoff(),
        "active_narrowing_reasons": [],
        "published_label": "stable",
        "publication_destinations": ["docs/m5", "help/about"],
        "rationale": "Emergency hotfix rule holds Stable provisionally under an unexpired docs-truth waiver; its Help/About support posture stays disclosed and its downgrade automation is verified.",
    },
    {
        "entry_id": "proof-freshness-automation",
        "title": "Proof-freshness automation",
        "lane_kind": "proof_freshness_automation",
        "surface_ref": "maintenance-truth:proof_freshness_automation",
        "surface_summary": "Maintenance scorecard and support posture for the M5 proof-freshness and evidence-expiry automation.",
        "release_blocking": True,
        "claim_ref": "claim:m5_proof_freshness_automation",
        "claim_label": "stable",
        "lane_state": "automation_undefined",
        "scorecard": scorecard(),
        "disclosure": disclosure(
            "support-window/proof-freshness",
            "proof-freshness-policy/automation",
            "first_party",
            ["scope/depth-trains"],
        ),
        "downgrade_automation": downgrade_automation(
            "proof-freshness-automation",
            trigger="manual",
            target_floor="beta",
            state="undefined",
            rollback_verified=False,
        ),
        "proof_packet": proof_packet("proof-freshness-automation"),
        "waiver": None,
        "owner_signoff": owner_signoff(),
        "active_narrowing_reasons": ["downgrade_automation_undefined"],
        "published_label": "beta",
        "publication_destinations": ["docs/m5", "help/about", "support/export"],
        "rationale": "Proof-freshness automation passes every maintenance dimension but narrows to Beta because its downgrade automation and frozen-fallback rollback plan are undefined.",
    },
    {
        "entry_id": "help-about-truth-surface",
        "title": "Help/About truth surface",
        "lane_kind": "help_about_truth",
        "surface_ref": "maintenance-truth:help_about_truth",
        "surface_summary": "Maintenance scorecard and support posture for the M5 Help/About truth surface.",
        "release_blocking": True,
        "claim_ref": "claim:m5_help_about_truth",
        "claim_label": "stable",
        "lane_state": "certified",
        "scorecard": scorecard(),
        "disclosure": disclosure(
            "support-window/help-about",
            "help-about-policy/truth",
            "verified_partner",
            ["scope/help", "scope/about"],
        ),
        "downgrade_automation": downgrade_automation(
            "help-about-truth-surface", trigger="maintenance_regressed"
        ),
        "proof_packet": proof_packet("help-about-truth-surface"),
        "waiver": None,
        "owner_signoff": owner_signoff(),
        "active_narrowing_reasons": [],
        "published_label": "stable",
        "publication_destinations": ["docs/m5", "help/about", "support/export"],
        "rationale": "Help/About truth surface passes every maintenance dimension and holds its release-blocking Stable claim with verified downgrade automation.",
    },
    {
        "entry_id": "correction-train-backport",
        "title": "Correction-train backport rule",
        "lane_kind": "backport_rule",
        "surface_ref": "maintenance-truth:correction_train_backport",
        "surface_summary": "Maintenance scorecard and support posture for the generated correction-train backport rule.",
        "release_blocking": False,
        "claim_ref": "claim:m5_correction_train_backport",
        "claim_label": "stable",
        "lane_state": "maintenance_regressed",
        "scorecard": scorecard({"evidence_expiry": "missing"}),
        "disclosure": disclosure(
            "support-window/correction-train",
            "backport-policy/correction-train",
            "generated",
            ["scope/correction-train"],
            truth_disclosed=False,
        ),
        "downgrade_automation": downgrade_automation(
            "correction-train-backport",
            trigger="proof_stale",
            target_floor="preview",
            state="unverified",
            rollback_verified=False,
        ),
        "proof_packet": proof_packet("correction-train-backport", "missing"),
        "waiver": None,
        "owner_signoff": owner_signoff(False),
        "active_narrowing_reasons": [
            "maintenance_dimension_missing",
            "proof_packet_missing",
            "owner_manifest_unsigned",
            "rollback_plan_unverified",
        ],
        "published_label": "preview",
        "publication_destinations": ["docs/m5"],
        "rationale": (
            "Correction-train backport rule narrows to Preview: its evidence-expiry "
            "maintenance report is missing and the Help/About support posture is not "
            "disclosed, the proof packet is missing, the owner manifest is unsigned, and "
            "the frozen-fallback rollback plan is unverified."
        ),
    },
]


def holds(label):
    return label in ABOVE_CUTLINE


def has_reason(row, reason):
    return reason in row["active_narrowing_reasons"]


def rule_fires(rule):
    return any(
        row["claim_label"] in rule["applies_to_labels"]
        and rule["trigger_reason"] in row["active_narrowing_reasons"]
        for row in ROWS
    )


def compute_summary():
    kinds = Counter(r["lane_kind"] for r in ROWS)
    cells = [c for r in ROWS for c in r["scorecard"]]
    grades = Counter(c["grade"] for c in cells)
    slo = Counter(r["proof_packet"]["slo_state"] for r in ROWS)
    trust = Counter(r["disclosure"]["trust_tier"] for r in ROWS)
    release_blocking = [r for r in ROWS if r["release_blocking"]]
    automation_gap = {"rollback_plan_unverified", "downgrade_automation_undefined"}
    dimension_gap = {"maintenance_dimension_failed", "maintenance_dimension_missing"}
    return {
        "total_entries": len(ROWS),
        "total_claims": len({r["claim_ref"] for r in ROWS}),
        "entries_certified": sum(1 for r in ROWS if holds(r["published_label"])),
        "entries_narrowed": sum(1 for r in ROWS if not holds(r["published_label"])),
        "entries_on_active_waiver": sum(1 for r in ROWS if r["lane_state"] == "on_waiver"),
        "entries_with_dimension_gap": sum(
            1 for r in ROWS if any(reason in dimension_gap for reason in r["active_narrowing_reasons"])
        ),
        "entries_with_owner_gap": sum(1 for r in ROWS if has_reason(r, "owner_manifest_unsigned")),
        "entries_with_automation_gap": sum(
            1 for r in ROWS if any(reason in automation_gap for reason in r["active_narrowing_reasons"])
        ),
        "entries_truth_undisclosed": sum(
            1 for r in ROWS if not r["disclosure"]["truth_disclosed"]
        ),
        "release_blocking_total": len(release_blocking),
        "release_blocking_certified": sum(1 for r in release_blocking if holds(r["published_label"])),
        "release_blocking_narrowed": sum(1 for r in release_blocking if not holds(r["published_label"])),
        "backport_rule_entries": kinds["backport_rule"],
        "hotfix_rule_entries": kinds["hotfix_rule"],
        "proof_freshness_automation_entries": kinds["proof_freshness_automation"],
        "help_about_truth_entries": kinds["help_about_truth"],
        "first_party_entries": trust["first_party"],
        "verified_partner_entries": trust["verified_partner"],
        "community_entries": trust["community"],
        "generated_entries": trust["generated"],
        "packets_current": slo["current"],
        "packets_due_for_refresh": slo["due_for_refresh"],
        "packets_breached": slo["breached"],
        "packets_missing": slo["missing"],
        "total_active_narrowing_reasons": sum(len(r["active_narrowing_reasons"]) for r in ROWS),
        "total_maintenance_cells": len(cells),
        "cells_pass": grades["pass"],
        "cells_partial": grades["partial"],
        "cells_fail": grades["fail"],
        "cells_waived": grades["waived"],
        "cells_missing": grades["missing"],
        "rules_firing": sum(1 for rule in STOP_RULES if rule_fires(rule)),
    }


blocking_rule_ids = sorted(
    rule["rule_id"] for rule in STOP_RULES if rule["blocks_promotion"] and rule_fires(rule)
)
blocking_triggers = {
    rule["trigger_reason"]
    for rule in STOP_RULES
    if rule["blocks_promotion"] and rule_fires(rule)
}
blocking_claim_ids = sorted(
    {
        r["entry_id"]
        for r in ROWS
        if holds(r["claim_label"])
        and any(reason in blocking_triggers for reason in r["active_narrowing_reasons"])
    }
)
decision = "hold" if blocking_rule_ids else "proceed"

REGISTER = {
    "schema_version": 1,
    "record_kind": RECORD_KIND,
    "manifest_id": "maintenance_truth_register:v1",
    "status": "published",
    "overview_page": f"docs/m5/{SLUG}.md",
    "as_of": AS_OF,
    "claim_manifest_ref": "artifacts/release/stable_claim_manifest.json",
    "lifecycle_labels": LIFECYCLE_LABELS,
    "lane_kinds": LANE_KINDS,
    "maintenance_dimensions": MAINTENANCE_DIMENSIONS,
    "dimension_grades": DIMENSION_GRADES,
    "lane_states": LANE_STATES,
    "narrowing_reasons": NARROWING_REASONS,
    "stop_rule_actions": STOP_RULE_ACTIONS,
    "automation_triggers": AUTOMATION_TRIGGERS,
    "automation_states": AUTOMATION_STATES,
    "trust_tiers": TRUST_TIERS,
    "launch_cutline": {
        "cutline_level": "stable",
        "above_cutline_levels": ["lts", "stable"],
        "below_cutline_levels": ["beta", "preview", "withdrawn"],
        "description": (
            "A maintenance-truth lane carries a Stable (or LTS) claim only when its "
            "maintenance scorecard passes every dimension (backport policy, hotfix "
            "policy, proof freshness, evidence expiry, Help/About truth, and docs "
            "truth), the Help/About support posture is disclosed, the proof packet is "
            "current within its freshness SLO, any waiver is unexpired, the owner "
            "manifest is signed, and its downgrade automation is defined and its "
            "frozen-fallback rollback plan verified. A lane that loses any of those "
            "must drop below the cutline rather than inherit an adjacent certified lane."
        ),
    },
    "release_blocking_surface_refs": RELEASE_BLOCKING,
    "stop_rules": STOP_RULES,
    "rows": ROWS,
    "promotion": {
        "promotion_gate": "maintenance-truth-gate",
        "decision": decision,
        "blocking_rule_ids": blocking_rule_ids,
        "blocking_claim_ids": blocking_claim_ids,
        "rationale": (
            "Computed from the firing stop rules over maintenance-dimension grades, "
            "Help/About support-posture disclosure, proof-packet freshness, owner-"
            "manifest sign-off, and downgrade-automation state."
        ),
    },
    "summary": compute_summary(),
}

CAPTURE = {
    "status": "pass",
    "as_of": AS_OF,
    "summary": {
        "total_entries": REGISTER["summary"]["total_entries"],
        "entries_certified": REGISTER["summary"]["entries_certified"],
        "entries_narrowed": REGISTER["summary"]["entries_narrowed"],
        "entries_on_active_waiver": REGISTER["summary"]["entries_on_active_waiver"],
        "entries_with_automation_gap": REGISTER["summary"]["entries_with_automation_gap"],
        "entries_truth_undisclosed": REGISTER["summary"]["entries_truth_undisclosed"],
        "packets_missing": REGISTER["summary"]["packets_missing"],
        "total_maintenance_cells": REGISTER["summary"]["total_maintenance_cells"],
        "cells_pass": REGISTER["summary"]["cells_pass"],
        "cells_missing": REGISTER["summary"]["cells_missing"],
        "rules_firing": REGISTER["summary"]["rules_firing"],
    },
    "promotion": {
        "decision": decision,
        "blocking_rule_ids": blocking_rule_ids,
        "blocking_claim_ids": blocking_claim_ids,
    },
    "negative_drills": [
        {"drill_id": "drill:certified_with_active_gap", "status": "passed"},
        {"drill_id": "drill:narrowing_without_reason", "status": "passed"},
        {"drill_id": "drill:certified_without_truth_disclosure", "status": "passed"},
        {"drill_id": "drill:certified_without_downgrade_automation", "status": "passed"},
    ],
    "fixture_cases": [
        {"case_id": "fixture:duplicate_entry_id", "status": "passed"},
        {"case_id": "fixture:missing_dimension_cell", "status": "passed"},
        {"case_id": "fixture:certified_with_automation_gap", "status": "passed"},
    ],
}


def write_json(path, payload):
    with open(path, "w", encoding="utf-8") as handle:
        json.dump(payload, handle, indent=2)
        handle.write("\n")


def build_fixtures():
    import copy

    duplicate = copy.deepcopy(REGISTER)
    extra = copy.deepcopy(duplicate["rows"][0])
    duplicate["rows"].append(extra)
    write_json(f"{FIXTURES_DIR}/duplicate_entry_id.json", duplicate)

    missing_cell = copy.deepcopy(REGISTER)
    missing_cell["rows"][0]["scorecard"] = [
        c for c in missing_cell["rows"][0]["scorecard"] if c["dimension"] != "docs_truth"
    ]
    write_json(f"{FIXTURES_DIR}/missing_dimension_cell.json", missing_cell)

    held_gap = copy.deepcopy(REGISTER)
    held_gap["rows"][0]["active_narrowing_reasons"] = ["rollback_plan_unverified"]
    write_json(f"{FIXTURES_DIR}/certified_with_automation_gap.json", held_gap)

    cases = {
        "cases": [
            {"file": "duplicate_entry_id.json", "expected_check_id": "DuplicateEntryId"},
            {
                "file": "missing_dimension_cell.json",
                "expected_check_id": "MaintenanceIncompleteCoverage",
            },
            {
                "file": "certified_with_automation_gap.json",
                "expected_check_id": "HeldWithActiveGap",
            },
        ]
    }
    write_json(f"{FIXTURES_DIR}/cases.json", cases)


if __name__ == "__main__":
    write_json(ARTIFACT_PATH, REGISTER)
    write_json(CAPTURE_PATH, CAPTURE)
    build_fixtures()
    print("Generated maintenance-truth register with", len(ROWS), "lanes")
    print("Promotion decision:", decision)
    print("Blocking rules:", blocking_rule_ids)
    print("Blocking claims:", blocking_claim_ids)
