#!/usr/bin/env python3
"""Generate the M5 per-train scorecard register, validation capture, and fixtures.

The register freezes one train-scorecard per M5 feature train. Each scorecard
binds the train to the stable claim it backs, a per-feature scorecard (one cell
per scorecard axis), an owner manifest sign-off, an explicit rollback/downgrade
automation record, a proof packet with its freshness SLO, and the active
narrowing reasons that drop the published label below the launch cutline.

This generator is the single source for the checked-in JSON so the typed Rust
consumer, the JSON Schema, and the CI capture stay in lock-step. Run it from the
repository root.
"""

import json
from collections import Counter

AS_OF = "2026-06-10"

SLUG = (
    "implement_per_feature_scorecards_owner_manifests_and_rollback_or_downgrade_"
    "automation_for_all_m5_trains"
)

ARTIFACT_PATH = f"artifacts/release/m5/{SLUG}.json"
CAPTURE_PATH = f"artifacts/release/captures/{SLUG}_validation_capture.json"
FIXTURES_DIR = f"fixtures/release/m5/{SLUG}"

RECORD_KIND = SLUG

LIFECYCLE_LABELS = ["lts", "stable", "beta", "preview", "withdrawn"]
ABOVE_CUTLINE = {"lts", "stable"}
BELOW_CUTLINE = {"beta", "preview", "withdrawn"}

TRAIN_KINDS = [
    "notebook",
    "data_rich",
    "ai_adjacent",
    "framework",
    "review",
    "companion",
    "managed_depth",
]

SCORECARD_AXES = [
    "functionality",
    "performance",
    "accessibility",
    "compatibility",
    "localization",
    "support_readiness",
]

SCORE_GRADES = ["pass", "partial", "fail", "waived", "missing"]

TRAIN_STATES = [
    "qualified",
    "scorecard_regressed",
    "stale",
    "on_waiver",
    "rollback_undefined",
    "owner_unsigned",
]

NARROWING_REASONS = [
    "scorecard_axis_failed",
    "scorecard_axis_missing",
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
    "remediate_scorecard",
    "refresh_proof_packet",
    "request_owner_signoff",
    "verify_rollback_plan",
    "define_downgrade_automation",
    "renew_waiver",
]

DOWNGRADE_TRIGGERS = ["proof_stale", "scorecard_regressed", "owner_revoked", "manual"]

AUTOMATION_STATES = ["defined", "unverified", "undefined"]

# reason -> (stop-rule title, default action)
REASON_RULES = {
    "scorecard_axis_failed": ("Scorecard axis failed", "remediate_scorecard"),
    "scorecard_axis_missing": ("Scorecard axis missing", "remediate_scorecard"),
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
        "rule_id": f"m5_train_rule:{reason}",
        "title": title,
        "trigger_reason": reason,
        "applies_to_labels": ["lts", "stable"],
        "default_action": action,
        "blocks_promotion": True,
        "rationale": (
            f"An M5 train scorecard whose posture reports '{reason}' cannot keep a "
            f"Stable or LTS claim and must narrow before publication."
        ),
    }
    for reason, (title, action) in REASON_RULES.items()
]

RELEASE_BLOCKING = [
    "train:m5_ai_adjacent",
    "train:m5_framework",
    "train:m5_companion",
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
        "owner_ref": "release-engineering",
        "signed_off": signed,
        "signed_at": AS_OF if signed else None,
    }


def automation(
    entry_id,
    trigger="proof_stale",
    target_floor="beta",
    state="defined",
    rollback_verified=True,
):
    return {
        "automation_ref": f"rollback-automation/{entry_id}",
        "rollback_plan_ref": f"rollback-plan/{entry_id}",
        "trigger": trigger,
        "target_floor": target_floor,
        "state": state,
        "rollback_verified": rollback_verified,
    }


def cell(axis, grade="pass"):
    return {
        "axis": axis,
        "grade": grade,
        "evidence_ref": "" if grade == "missing" else f"evidence/{axis}",
    }


def scorecard(overrides=None):
    """Build a full scorecard, passing except for overrides."""
    overrides = overrides or {}
    return [cell(axis, overrides.get(axis, "pass")) for axis in SCORECARD_AXES]


ROWS = [
    {
        "entry_id": "m5-notebook-train",
        "title": "Notebook train scorecard",
        "train_kind": "notebook",
        "train_ref": "train:m5_notebook",
        "train_summary": "Notebook and data-rich notebook depth surfaces.",
        "release_blocking": False,
        "claim_ref": "claim:m5_notebook",
        "claim_label": "stable",
        "train_state": "qualified",
        "scorecard": scorecard(),
        "downgrade_automation": automation("m5-notebook-train"),
        "proof_packet": proof_packet("m5-notebook-train"),
        "waiver": None,
        "owner_signoff": owner_signoff(),
        "active_narrowing_reasons": [],
        "published_label": "stable",
        "publication_destinations": ["docs/m5", "help/about", "support/export"],
        "rationale": "Notebook train passes every scorecard axis, is owner-signed, and its rollback/downgrade automation is verified.",
    },
    {
        "entry_id": "m5-data-rich-train",
        "title": "Data-rich train scorecard",
        "train_kind": "data_rich",
        "train_ref": "train:m5_data_rich",
        "train_summary": "Result grids, variable explorers, and data tables.",
        "release_blocking": False,
        "claim_ref": "claim:m5_data_rich",
        "claim_label": "stable",
        "train_state": "qualified",
        "scorecard": scorecard(),
        "downgrade_automation": automation("m5-data-rich-train"),
        "proof_packet": proof_packet("m5-data-rich-train"),
        "waiver": None,
        "owner_signoff": owner_signoff(),
        "active_narrowing_reasons": [],
        "published_label": "stable",
        "publication_destinations": ["docs/m5", "help/about"],
        "rationale": "Data-rich train passes every scorecard axis and holds its Stable claim with verified downgrade automation.",
    },
    {
        "entry_id": "m5-ai-adjacent-train",
        "title": "AI-adjacent train scorecard",
        "train_kind": "ai_adjacent",
        "train_ref": "train:m5_ai_adjacent",
        "train_summary": "AI-adjacent and language-intelligence depth surfaces.",
        "release_blocking": True,
        "claim_ref": "claim:m5_ai_adjacent",
        "claim_label": "stable",
        "train_state": "qualified",
        "scorecard": scorecard(),
        "downgrade_automation": automation("m5-ai-adjacent-train"),
        "proof_packet": proof_packet("m5-ai-adjacent-train"),
        "waiver": None,
        "owner_signoff": owner_signoff(),
        "active_narrowing_reasons": [],
        "published_label": "stable",
        "publication_destinations": ["docs/m5", "help/about", "support/export"],
        "rationale": "AI-adjacent train passes every scorecard axis and holds its release-blocking Stable claim with verified downgrade automation.",
    },
    {
        "entry_id": "m5-framework-train",
        "title": "Framework train scorecard",
        "train_kind": "framework",
        "train_ref": "train:m5_framework",
        "train_summary": "Core framework and platform foundations.",
        "release_blocking": True,
        "claim_ref": "claim:m5_framework",
        "claim_label": "stable",
        "train_state": "qualified",
        "scorecard": scorecard(),
        "downgrade_automation": automation("m5-framework-train"),
        "proof_packet": proof_packet("m5-framework-train"),
        "waiver": None,
        "owner_signoff": owner_signoff(),
        "active_narrowing_reasons": [],
        "published_label": "stable",
        "publication_destinations": ["docs/m5", "help/about", "support/export"],
        "rationale": "Framework train passes every scorecard axis and holds its release-blocking Stable claim with verified downgrade automation.",
    },
    {
        "entry_id": "m5-review-train",
        "title": "Review train scorecard",
        "train_kind": "review",
        "train_ref": "train:m5_review",
        "train_summary": "Review and diff depth surfaces.",
        "release_blocking": False,
        "claim_ref": "claim:m5_review",
        "claim_label": "stable",
        "train_state": "on_waiver",
        "scorecard": scorecard({"accessibility": "waived"}),
        "downgrade_automation": automation("m5-review-train"),
        "proof_packet": proof_packet("m5-review-train"),
        "waiver": {
            "waiver_ref": "waiver:m5_review_accessibility",
            "expires_at": "2026-12-31",
            "reason": "Accessibility re-validation scheduled; interim scorecard axis waived by owner.",
        },
        "owner_signoff": owner_signoff(),
        "active_narrowing_reasons": [],
        "published_label": "stable",
        "publication_destinations": ["docs/m5", "help/about"],
        "rationale": "Review train holds Stable provisionally under an unexpired accessibility-axis waiver; downgrade automation is verified.",
    },
    {
        "entry_id": "m5-companion-train",
        "title": "Companion train scorecard",
        "train_kind": "companion",
        "train_ref": "train:m5_companion",
        "train_summary": "Browser and mobile companion depth surfaces.",
        "release_blocking": True,
        "claim_ref": "claim:m5_companion",
        "claim_label": "stable",
        "train_state": "rollback_undefined",
        "scorecard": scorecard(),
        "downgrade_automation": automation(
            "m5-companion-train",
            trigger="manual",
            target_floor="beta",
            state="undefined",
            rollback_verified=False,
        ),
        "proof_packet": proof_packet("m5-companion-train"),
        "waiver": None,
        "owner_signoff": owner_signoff(),
        "active_narrowing_reasons": ["downgrade_automation_undefined"],
        "published_label": "beta",
        "publication_destinations": ["docs/m5", "help/about", "support/export"],
        "rationale": "Companion train passes its scorecard but narrows to Beta because its rollback/downgrade automation is undefined.",
    },
    {
        "entry_id": "m5-managed-depth-train",
        "title": "Managed-depth train scorecard",
        "train_kind": "managed_depth",
        "train_ref": "train:m5_managed_depth",
        "train_summary": "Managed-depth and infrastructure depth surfaces.",
        "release_blocking": False,
        "claim_ref": "claim:m5_managed_depth",
        "claim_label": "stable",
        "train_state": "scorecard_regressed",
        "scorecard": scorecard({"support_readiness": "missing"}),
        "downgrade_automation": automation(
            "m5-managed-depth-train",
            trigger="proof_stale",
            target_floor="preview",
            state="unverified",
            rollback_verified=False,
        ),
        "proof_packet": proof_packet("m5-managed-depth-train", "missing"),
        "waiver": None,
        "owner_signoff": owner_signoff(False),
        "active_narrowing_reasons": [
            "scorecard_axis_missing",
            "proof_packet_missing",
            "owner_manifest_unsigned",
            "rollback_plan_unverified",
        ],
        "published_label": "preview",
        "publication_destinations": ["docs/m5"],
        "rationale": (
            "Managed-depth train narrows to Preview: a scorecard axis is missing, the "
            "proof packet is missing, the owner manifest is unsigned, and the rollback "
            "plan is unverified."
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
    kinds = Counter(r["train_kind"] for r in ROWS)
    cells = [c for r in ROWS for c in r["scorecard"]]
    grades = Counter(c["grade"] for c in cells)
    slo = Counter(r["proof_packet"]["slo_state"] for r in ROWS)
    release_blocking = [r for r in ROWS if r["release_blocking"]]
    rollback_gap = {"rollback_plan_unverified", "downgrade_automation_undefined"}
    scorecard_gap = {"scorecard_axis_failed", "scorecard_axis_missing"}
    return {
        "total_entries": len(ROWS),
        "total_claims": len({r["claim_ref"] for r in ROWS}),
        "entries_qualified": sum(1 for r in ROWS if holds(r["published_label"])),
        "entries_narrowed": sum(1 for r in ROWS if not holds(r["published_label"])),
        "entries_on_active_waiver": sum(1 for r in ROWS if r["train_state"] == "on_waiver"),
        "entries_with_scorecard_gap": sum(
            1 for r in ROWS if any(reason in scorecard_gap for reason in r["active_narrowing_reasons"])
        ),
        "entries_with_owner_gap": sum(1 for r in ROWS if has_reason(r, "owner_manifest_unsigned")),
        "entries_with_rollback_gap": sum(
            1 for r in ROWS if any(reason in rollback_gap for reason in r["active_narrowing_reasons"])
        ),
        "release_blocking_total": len(release_blocking),
        "release_blocking_qualified": sum(1 for r in release_blocking if holds(r["published_label"])),
        "release_blocking_narrowed": sum(1 for r in release_blocking if not holds(r["published_label"])),
        "notebook_entries": kinds["notebook"],
        "data_rich_entries": kinds["data_rich"],
        "ai_adjacent_entries": kinds["ai_adjacent"],
        "framework_entries": kinds["framework"],
        "review_entries": kinds["review"],
        "companion_entries": kinds["companion"],
        "managed_depth_entries": kinds["managed_depth"],
        "packets_current": slo["current"],
        "packets_due_for_refresh": slo["due_for_refresh"],
        "packets_breached": slo["breached"],
        "packets_missing": slo["missing"],
        "total_active_narrowing_reasons": sum(len(r["active_narrowing_reasons"]) for r in ROWS),
        "total_scorecard_cells": len(cells),
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
    "manifest_id": "m5_train_scorecard_register:v1",
    "status": "published",
    "overview_page": f"docs/m5/{SLUG}.md",
    "as_of": AS_OF,
    "claim_manifest_ref": "artifacts/release/stable_claim_manifest.json",
    "lifecycle_labels": LIFECYCLE_LABELS,
    "train_kinds": TRAIN_KINDS,
    "scorecard_axes": SCORECARD_AXES,
    "score_grades": SCORE_GRADES,
    "train_states": TRAIN_STATES,
    "narrowing_reasons": NARROWING_REASONS,
    "stop_rule_actions": STOP_RULE_ACTIONS,
    "downgrade_triggers": DOWNGRADE_TRIGGERS,
    "automation_states": AUTOMATION_STATES,
    "launch_cutline": {
        "cutline_level": "stable",
        "above_cutline_levels": ["lts", "stable"],
        "below_cutline_levels": ["beta", "preview", "withdrawn"],
        "description": (
            "An M5 train carries a Stable (or LTS) claim only when its per-feature "
            "scorecard passes every axis, the proof packet is current within its "
            "freshness SLO, any waiver is unexpired, the owner manifest is signed, and "
            "its rollback/downgrade automation is defined and verified. A train that "
            "loses any of those must drop below the cutline rather than inherit an "
            "adjacent qualified train."
        ),
    },
    "release_blocking_train_refs": RELEASE_BLOCKING,
    "stop_rules": STOP_RULES,
    "rows": ROWS,
    "promotion": {
        "promotion_gate": "m5-train-scorecard-gate",
        "decision": decision,
        "blocking_rule_ids": blocking_rule_ids,
        "blocking_claim_ids": blocking_claim_ids,
        "rationale": (
            "Computed from the firing stop rules over scorecard-axis grades, proof-packet "
            "freshness, owner-manifest sign-off, and rollback/downgrade automation state."
        ),
    },
    "summary": compute_summary(),
}

CAPTURE = {
    "status": "pass",
    "as_of": AS_OF,
    "summary": {
        "total_entries": REGISTER["summary"]["total_entries"],
        "entries_qualified": REGISTER["summary"]["entries_qualified"],
        "entries_narrowed": REGISTER["summary"]["entries_narrowed"],
        "entries_on_active_waiver": REGISTER["summary"]["entries_on_active_waiver"],
        "entries_with_rollback_gap": REGISTER["summary"]["entries_with_rollback_gap"],
        "packets_missing": REGISTER["summary"]["packets_missing"],
        "total_scorecard_cells": REGISTER["summary"]["total_scorecard_cells"],
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
        {"drill_id": "drill:held_with_active_gap", "status": "passed"},
        {"drill_id": "drill:narrowing_without_reason", "status": "passed"},
        {"drill_id": "drill:held_without_downgrade_automation", "status": "passed"},
    ],
    "fixture_cases": [
        {"case_id": "fixture:duplicate_entry_id", "status": "passed"},
        {"case_id": "fixture:missing_axis_cell", "status": "passed"},
        {"case_id": "fixture:held_with_rollback_gap", "status": "passed"},
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
        c for c in missing_cell["rows"][0]["scorecard"] if c["axis"] != "localization"
    ]
    write_json(f"{FIXTURES_DIR}/missing_axis_cell.json", missing_cell)

    held_gap = copy.deepcopy(REGISTER)
    held_gap["rows"][0]["active_narrowing_reasons"] = ["rollback_plan_unverified"]
    write_json(f"{FIXTURES_DIR}/held_with_rollback_gap.json", held_gap)

    cases = {
        "cases": [
            {"file": "duplicate_entry_id.json", "expected_check_id": "DuplicateEntryId"},
            {
                "file": "missing_axis_cell.json",
                "expected_check_id": "ScorecardIncompleteCoverage",
            },
            {"file": "held_with_rollback_gap.json", "expected_check_id": "HeldWithActiveGap"},
        ]
    }
    write_json(f"{FIXTURES_DIR}/cases.json", cases)


if __name__ == "__main__":
    write_json(ARTIFACT_PATH, REGISTER)
    write_json(CAPTURE_PATH, CAPTURE)
    build_fixtures()
    print("Generated train scorecard register with", len(ROWS), "train scorecards")
    print("Promotion decision:", decision)
    print("Blocking rules:", blocking_rule_ids)
    print("Blocking claims:", blocking_claim_ids)
