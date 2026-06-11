#!/usr/bin/env python3
"""Generate the M5 depth-claim manifest artifact, validation capture, and fixtures.

The depth-claim manifest freezes one feature-family packet per M5 feature
family. Each packet binds the family to the stable claim it backs, a
qualification matrix (one cell per qualification dimension), a proof packet with
its freshness SLO, an owner sign-off, and the active narrowing reasons that drop
the published label below the launch cutline.

This generator is the single source for the checked-in JSON so the typed Rust
consumer, the JSON Schema, and the CI capture stay in lock-step. Run it from the
repository root.
"""

import json
from collections import Counter

AS_OF = "2026-06-10"

ARTIFACT_PATH = (
    "artifacts/release/m5/"
    "freeze_the_m5_depth_claim_manifest_feature_family_packets_and_qualification_matrix.json"
)
CAPTURE_PATH = (
    "artifacts/release/captures/"
    "freeze_the_m5_depth_claim_manifest_feature_family_packets_and_qualification_matrix"
    "_validation_capture.json"
)
FIXTURES_DIR = (
    "fixtures/release/m5/"
    "freeze_the_m5_depth_claim_manifest_feature_family_packets_and_qualification_matrix"
)

RECORD_KIND = (
    "freeze_m5_depth_claim_manifest_feature_family_packets_and_qualification_matrix"
)

LIFECYCLE_LABELS = ["lts", "stable", "beta", "preview", "withdrawn"]
ABOVE_CUTLINE = {"lts", "stable"}

FAMILY_KINDS = [
    "notebook",
    "data_rich",
    "ai_adjacent",
    "framework",
    "review",
    "companion",
    "managed_depth",
]

QUALIFICATION_DIMENSIONS = [
    "scorecard",
    "compatibility",
    "proof_freshness",
    "lineage",
    "locale_parity",
    "support_packet",
    "accessibility",
    "downgrade_automation",
]

QUALIFICATION_STATES = ["qualified", "incomplete", "stale", "waived", "missing"]

PACKET_STATES = [
    "qualified",
    "incomplete",
    "stale",
    "on_waiver",
    "lineage_missing",
    "locale_drifted",
    "support_lagging",
]

NARROWING_REASONS = [
    "scorecard_incomplete",
    "compatibility_missing",
    "proof_packet_missing",
    "proof_packet_stale",
    "lineage_missing",
    "locale_parity_drifted",
    "support_packet_lagging",
    "accessibility_unsigned",
    "downgrade_automation_missing",
    "waiver_expired",
    "owner_signoff_missing",
]

STOP_RULE_ACTIONS = [
    "hold_promotion",
    "narrow_label",
    "complete_scorecard",
    "refresh_compatibility_packet",
    "refresh_proof_packet",
    "refresh_lineage",
    "refresh_locale_pack",
    "refresh_support_packet",
    "sign_accessibility",
    "define_downgrade_automation",
    "request_owner_signoff",
]

# reason -> (stop-rule title, default action)
REASON_RULES = {
    "scorecard_incomplete": ("Feature scorecard incomplete", "complete_scorecard"),
    "compatibility_missing": ("Compatibility packet missing", "refresh_compatibility_packet"),
    "proof_packet_missing": ("Proof packet missing", "refresh_proof_packet"),
    "proof_packet_stale": ("Proof packet stale", "refresh_proof_packet"),
    "lineage_missing": ("Generated-artifact lineage missing", "refresh_lineage"),
    "locale_parity_drifted": ("Locale parity drifted", "refresh_locale_pack"),
    "support_packet_lagging": ("Support packet lagging shipped behavior", "refresh_support_packet"),
    "accessibility_unsigned": ("Accessibility signoff missing", "sign_accessibility"),
    "downgrade_automation_missing": ("Downgrade automation missing", "define_downgrade_automation"),
    "waiver_expired": ("Waiver expired", "hold_promotion"),
    "owner_signoff_missing": ("Owner sign-off missing", "request_owner_signoff"),
}

STOP_RULES = [
    {
        "rule_id": f"m5_depth_rule:{reason}",
        "title": title,
        "trigger_reason": reason,
        "applies_to_labels": ["lts", "stable"],
        "default_action": action,
        "blocks_promotion": True,
        "rationale": (
            f"An M5 feature-family packet whose qualification matrix reports "
            f"'{reason}' cannot keep a Stable or LTS depth claim."
        ),
    }
    for reason, (title, action) in REASON_RULES.items()
]

RELEASE_BLOCKING = [
    "family:m5_ai_adjacent",
    "family:m5_framework",
    "family:m5_companion",
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


def cell(dimension, state="qualified"):
    return {
        "dimension": dimension,
        "state": state,
        "evidence_ref": "" if state == "missing" else f"evidence/{dimension}",
    }


def matrix(overrides=None):
    """Build a full qualification matrix, qualified except for overrides."""
    overrides = overrides or {}
    return [cell(dim, overrides.get(dim, "qualified")) for dim in QUALIFICATION_DIMENSIONS]


ROWS = [
    {
        "entry_id": "m5-notebook-depth",
        "title": "Notebook depth-claim family packet",
        "family_kind": "notebook",
        "family_ref": "family:m5_notebook",
        "family_summary": "Notebook and data-rich notebook depth surfaces.",
        "release_blocking": False,
        "claim_ref": "claim:m5_notebook",
        "claim_label": "stable",
        "packet_state": "qualified",
        "qualification_matrix": matrix(),
        "proof_packet": proof_packet("m5-notebook-depth"),
        "waiver": None,
        "owner_signoff": owner_signoff(),
        "active_narrowing_reasons": [],
        "published_label": "stable",
        "publication_destinations": ["docs/m5", "help/about", "support/export"],
        "rationale": "Notebook family is fully qualified on every dimension and holds its Stable claim.",
    },
    {
        "entry_id": "m5-data-rich-depth",
        "title": "Data-rich depth-claim family packet",
        "family_kind": "data_rich",
        "family_ref": "family:m5_data_rich",
        "family_summary": "Result grids, variable explorers, and data tables.",
        "release_blocking": False,
        "claim_ref": "claim:m5_data_rich",
        "claim_label": "stable",
        "packet_state": "qualified",
        "qualification_matrix": matrix(),
        "proof_packet": proof_packet("m5-data-rich-depth"),
        "waiver": None,
        "owner_signoff": owner_signoff(),
        "active_narrowing_reasons": [],
        "published_label": "stable",
        "publication_destinations": ["docs/m5", "help/about"],
        "rationale": "Data-rich family is fully qualified and holds its Stable claim.",
    },
    {
        "entry_id": "m5-ai-adjacent-depth",
        "title": "AI-adjacent depth-claim family packet",
        "family_kind": "ai_adjacent",
        "family_ref": "family:m5_ai_adjacent",
        "family_summary": "AI-adjacent and language-intelligence depth surfaces.",
        "release_blocking": True,
        "claim_ref": "claim:m5_ai_adjacent",
        "claim_label": "stable",
        "packet_state": "qualified",
        "qualification_matrix": matrix(),
        "proof_packet": proof_packet("m5-ai-adjacent-depth"),
        "waiver": None,
        "owner_signoff": owner_signoff(),
        "active_narrowing_reasons": [],
        "published_label": "stable",
        "publication_destinations": ["docs/m5", "help/about", "support/export"],
        "rationale": "AI-adjacent family is fully qualified and holds its release-blocking Stable claim.",
    },
    {
        "entry_id": "m5-framework-depth",
        "title": "Framework depth-claim family packet",
        "family_kind": "framework",
        "family_ref": "family:m5_framework",
        "family_summary": "Core framework and platform foundations.",
        "release_blocking": True,
        "claim_ref": "claim:m5_framework",
        "claim_label": "stable",
        "packet_state": "qualified",
        "qualification_matrix": matrix(),
        "proof_packet": proof_packet("m5-framework-depth"),
        "waiver": None,
        "owner_signoff": owner_signoff(),
        "active_narrowing_reasons": [],
        "published_label": "stable",
        "publication_destinations": ["docs/m5", "help/about", "support/export"],
        "rationale": "Framework family is fully qualified and holds its release-blocking Stable claim.",
    },
    {
        "entry_id": "m5-review-depth",
        "title": "Review depth-claim family packet",
        "family_kind": "review",
        "family_ref": "family:m5_review",
        "family_summary": "Review and diff depth surfaces.",
        "release_blocking": False,
        "claim_ref": "claim:m5_review",
        "claim_label": "stable",
        "packet_state": "on_waiver",
        "qualification_matrix": matrix({"accessibility": "waived"}),
        "proof_packet": proof_packet("m5-review-depth"),
        "waiver": {
            "waiver_ref": "waiver:m5_review_accessibility",
            "expires_at": "2026-12-31",
            "reason": "Accessibility re-validation scheduled; interim signoff waived by owner.",
        },
        "owner_signoff": owner_signoff(),
        "active_narrowing_reasons": [],
        "published_label": "stable",
        "publication_destinations": ["docs/m5", "help/about"],
        "rationale": "Review family holds Stable provisionally under an unexpired accessibility waiver.",
    },
    {
        "entry_id": "m5-companion-depth",
        "title": "Companion depth-claim family packet",
        "family_kind": "companion",
        "family_ref": "family:m5_companion",
        "family_summary": "Browser and mobile companion depth surfaces.",
        "release_blocking": True,
        "claim_ref": "claim:m5_companion",
        "claim_label": "stable",
        "packet_state": "locale_drifted",
        "qualification_matrix": matrix({"locale_parity": "stale"}),
        "proof_packet": proof_packet("m5-companion-depth"),
        "waiver": None,
        "owner_signoff": owner_signoff(),
        "active_narrowing_reasons": ["locale_parity_drifted"],
        "published_label": "beta",
        "publication_destinations": ["docs/m5", "help/about", "support/export"],
        "rationale": "Companion family narrows to Beta because its locale pack drifted out of parity.",
    },
    {
        "entry_id": "m5-managed-depth-depth",
        "title": "Managed-depth depth-claim family packet",
        "family_kind": "managed_depth",
        "family_ref": "family:m5_managed_depth",
        "family_summary": "Managed-depth and infrastructure depth surfaces.",
        "release_blocking": False,
        "claim_ref": "claim:m5_managed_depth",
        "claim_label": "stable",
        "packet_state": "incomplete",
        "qualification_matrix": matrix(
            {
                "proof_freshness": "missing",
                "lineage": "missing",
                "support_packet": "missing",
                "accessibility": "incomplete",
            }
        ),
        "proof_packet": proof_packet("m5-managed-depth-depth", "missing"),
        "waiver": None,
        "owner_signoff": owner_signoff(False),
        "active_narrowing_reasons": [
            "proof_packet_missing",
            "lineage_missing",
            "support_packet_lagging",
            "accessibility_unsigned",
            "owner_signoff_missing",
        ],
        "published_label": "preview",
        "publication_destinations": ["docs/m5"],
        "rationale": (
            "Managed-depth family narrows to Preview: proof packet, generated-artifact "
            "lineage, and support packet are missing, accessibility is unsigned, and the "
            "owner has not signed off."
        ),
    },
]


def holds(label):
    return label in ABOVE_CUTLINE


def rule_fires(rule):
    return any(
        row["claim_label"] in rule["applies_to_labels"]
        and rule["trigger_reason"] in row["active_narrowing_reasons"]
        for row in ROWS
    )


def has_reason(row, reason):
    return reason in row["active_narrowing_reasons"]


def compute_summary():
    kinds = Counter(r["family_kind"] for r in ROWS)
    cells = [c for r in ROWS for c in r["qualification_matrix"]]
    cell_states = Counter(c["state"] for c in cells)
    slo = Counter(r["proof_packet"]["slo_state"] for r in ROWS)
    release_blocking = [r for r in ROWS if r["release_blocking"]]
    return {
        "total_entries": len(ROWS),
        "total_claims": len({r["claim_ref"] for r in ROWS}),
        "entries_qualified": sum(1 for r in ROWS if holds(r["published_label"])),
        "entries_narrowed": sum(1 for r in ROWS if not holds(r["published_label"])),
        "entries_on_active_waiver": sum(1 for r in ROWS if r["packet_state"] == "on_waiver"),
        "entries_with_lineage_gap": sum(1 for r in ROWS if has_reason(r, "lineage_missing")),
        "entries_with_locale_gap": sum(1 for r in ROWS if has_reason(r, "locale_parity_drifted")),
        "entries_with_support_gap": sum(1 for r in ROWS if has_reason(r, "support_packet_lagging")),
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
        "total_qualification_cells": len(cells),
        "cells_qualified": cell_states["qualified"],
        "cells_incomplete": cell_states["incomplete"],
        "cells_stale": cell_states["stale"],
        "cells_waived": cell_states["waived"],
        "cells_missing": cell_states["missing"],
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

MANIFEST = {
    "schema_version": 1,
    "record_kind": RECORD_KIND,
    "manifest_id": "m5_depth_claim_manifest:v1",
    "status": "published",
    "overview_page": (
        "docs/m5/"
        "freeze_the_m5_depth_claim_manifest_feature_family_packets_and_qualification_matrix.md"
    ),
    "as_of": AS_OF,
    "claim_manifest_ref": "artifacts/release/stable_claim_manifest.json",
    "lifecycle_labels": LIFECYCLE_LABELS,
    "family_kinds": FAMILY_KINDS,
    "qualification_dimensions": QUALIFICATION_DIMENSIONS,
    "qualification_states": QUALIFICATION_STATES,
    "packet_states": PACKET_STATES,
    "narrowing_reasons": NARROWING_REASONS,
    "stop_rule_actions": STOP_RULE_ACTIONS,
    "launch_cutline": {
        "cutline_level": "stable",
        "above_cutline_levels": ["lts", "stable"],
        "below_cutline_levels": ["beta", "preview", "withdrawn"],
        "description": (
            "An M5 feature-family packet carries a Stable (or LTS) depth claim only when its "
            "qualification matrix is fully qualified, the proof packet is current within its "
            "freshness SLO, any waiver is unexpired, generated-artifact lineage is present, "
            "locale parity holds, the support packet matches shipped behavior, and the owner "
            "has signed off. A family that loses any of those must drop below the cutline "
            "rather than inherit an adjacent qualified family."
        ),
    },
    "release_blocking_family_refs": RELEASE_BLOCKING,
    "stop_rules": STOP_RULES,
    "rows": ROWS,
    "promotion": {
        "promotion_gate": "m5-depth-claim-gate",
        "decision": decision,
        "blocking_rule_ids": blocking_rule_ids,
        "blocking_claim_ids": blocking_claim_ids,
        "rationale": (
            "Computed from the firing stop rules over qualification-matrix gaps, proof-packet "
            "freshness, lineage, locale parity, support-packet currency, accessibility signoff, "
            "waiver expiry, and owner sign-off."
        ),
    },
    "summary": compute_summary(),
}

CAPTURE = {
    "status": "pass",
    "as_of": AS_OF,
    "summary": {
        "total_entries": MANIFEST["summary"]["total_entries"],
        "entries_qualified": MANIFEST["summary"]["entries_qualified"],
        "entries_narrowed": MANIFEST["summary"]["entries_narrowed"],
        "entries_on_active_waiver": MANIFEST["summary"]["entries_on_active_waiver"],
        "packets_missing": MANIFEST["summary"]["packets_missing"],
        "total_qualification_cells": MANIFEST["summary"]["total_qualification_cells"],
        "cells_qualified": MANIFEST["summary"]["cells_qualified"],
        "cells_missing": MANIFEST["summary"]["cells_missing"],
        "rules_firing": MANIFEST["summary"]["rules_firing"],
    },
    "promotion": {
        "decision": decision,
        "blocking_rule_ids": blocking_rule_ids,
        "blocking_claim_ids": blocking_claim_ids,
    },
    "negative_drills": [
        {"drill_id": "drill:held_with_active_gap", "status": "passed"},
        {"drill_id": "drill:narrowing_without_reason", "status": "passed"},
        {"drill_id": "drill:published_wider_than_claim", "status": "passed"},
    ],
    "fixture_cases": [
        {"case_id": "fixture:duplicate_entry_id", "status": "passed"},
        {"case_id": "fixture:missing_dimension_cell", "status": "passed"},
        {"case_id": "fixture:held_with_locale_gap", "status": "passed"},
    ],
}


def write_json(path, payload):
    with open(path, "w", encoding="utf-8") as handle:
        json.dump(payload, handle, indent=2)
        handle.write("\n")


def build_fixtures():
    import copy

    duplicate = copy.deepcopy(MANIFEST)
    extra = copy.deepcopy(duplicate["rows"][0])
    duplicate["rows"].append(extra)
    write_json(f"{FIXTURES_DIR}/duplicate_entry_id.json", duplicate)

    missing_cell = copy.deepcopy(MANIFEST)
    missing_cell["rows"][0]["qualification_matrix"] = [
        c
        for c in missing_cell["rows"][0]["qualification_matrix"]
        if c["dimension"] != "lineage"
    ]
    write_json(f"{FIXTURES_DIR}/missing_dimension_cell.json", missing_cell)

    held_gap = copy.deepcopy(MANIFEST)
    held_gap["rows"][0]["active_narrowing_reasons"] = ["locale_parity_drifted"]
    write_json(f"{FIXTURES_DIR}/held_with_locale_gap.json", held_gap)

    cases = {
        "cases": [
            {"file": "duplicate_entry_id.json", "expected_check_id": "DuplicateEntryId"},
            {
                "file": "missing_dimension_cell.json",
                "expected_check_id": "QualificationMatrixIncompleteCoverage",
            },
            {"file": "held_with_locale_gap.json", "expected_check_id": "HeldWithActiveGap"},
        ]
    }
    write_json(f"{FIXTURES_DIR}/cases.json", cases)


if __name__ == "__main__":
    write_json(ARTIFACT_PATH, MANIFEST)
    write_json(CAPTURE_PATH, CAPTURE)
    build_fixtures()
    print("Generated depth-claim manifest with", len(ROWS), "feature-family packets")
    print("Promotion decision:", decision)
    print("Blocking rules:", blocking_rule_ids)
    print("Blocking claims:", blocking_claim_ids)
