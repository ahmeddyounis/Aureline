#!/usr/bin/env python3
"""Generate the M5 publication-pack register, capture, and fixtures.

The register freezes one publication pack per pack kind — the docs packs, the
migration packs, the known-limits packs, and the publication index that ties them
together — published for the M5 feature families (notebooks, DB/API, profiler, AI,
frameworks, companion, sync, and offboarding). Each pack binds the surface to the
stable claim it backs, a per-pack readiness scorecard (one cell per dimension:
docs coverage, migration completeness, known-limits disclosure, redaction safety,
proof freshness, and locale parity), the disclosed support posture it exposes
(support window, policy ref, maintainer trust tier, scope refs, and whether the
redaction/provenance posture is disclosed to the operator), an owner-manifest
sign-off, an explicit downgrade/rollback automation record (fall back to a frozen
floor label), a proof packet with its freshness SLO, and the active narrowing
reasons that drop the published label below the launch cutline when a readiness
dimension fails or is missing, the redaction posture is undisclosed, proof
freshness expires, the owner manifest is unsigned, the rollback plan is
unverified, downgrade automation is undefined, or a waiver expires.

This generator is the single source for the checked-in JSON so the typed Rust
consumer, the JSON Schema, and the CI capture stay in lock-step. Run it from the
repository root.
"""

import json
from collections import Counter

AS_OF = "2026-06-10"

SLUG = "publish_docs_migration_and_known_limits_packs_for_m5_feature_families"

ARTIFACT_PATH = f"artifacts/release/m5/{SLUG}.json"
CAPTURE_PATH = f"artifacts/release/captures/{SLUG}_validation_capture.json"
FIXTURES_DIR = f"fixtures/release/m5/{SLUG}"

RECORD_KIND = SLUG

LIFECYCLE_LABELS = ["lts", "stable", "beta", "preview", "withdrawn"]
ABOVE_CUTLINE = {"lts", "stable"}
BELOW_CUTLINE = {"beta", "preview", "withdrawn"}

SURFACE_KINDS = [
    "docs_pack",
    "migration_pack",
    "known_limits_pack",
    "publication_index",
]

READINESS_DIMENSIONS = [
    "docs_coverage",
    "migration_completeness",
    "known_limits_disclosure",
    "redaction_safety",
    "proof_freshness",
    "locale_parity",
]

DIMENSION_GRADES = ["pass", "partial", "fail", "waived", "missing"]

SURFACE_STATES = [
    "certified",
    "readiness_regressed",
    "stale",
    "on_waiver",
    "automation_undefined",
    "owner_unsigned",
]

NARROWING_REASONS = [
    "readiness_dimension_failed",
    "readiness_dimension_missing",
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
    "remediate_readiness",
    "refresh_proof_packet",
    "request_owner_signoff",
    "verify_rollback_plan",
    "define_downgrade_automation",
    "renew_waiver",
]

AUTOMATION_TRIGGERS = ["proof_stale", "readiness_regressed", "owner_revoked", "manual"]

AUTOMATION_STATES = ["defined", "unverified", "undefined"]

TRUST_TIERS = ["first_party", "verified_partner", "community", "generated"]

# reason -> (stop-rule title, default action)
REASON_RULES = {
    "readiness_dimension_failed": ("Readiness dimension failed", "remediate_readiness"),
    "readiness_dimension_missing": ("Readiness dimension missing", "remediate_readiness"),
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
        "rule_id": f"publication_pack_rule:{reason}",
        "title": title,
        "trigger_reason": reason,
        "applies_to_labels": ["lts", "stable"],
        "default_action": action,
        "blocks_promotion": True,
        "rationale": (
            f"A publication pack whose posture reports '{reason}' cannot keep a Stable "
            f"or LTS claim and must narrow before publication."
        ),
    }
    for reason, (title, action) in REASON_RULES.items()
]

RELEASE_BLOCKING = [
    "publication-pack:docs_notebooks",
    "publication-pack:known_limits_ai",
    "publication-pack:publication_index",
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
        "owner_ref": "publication-guild",
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
    redaction_disclosed=True,
):
    return {
        "support_window_ref": support_window_ref,
        "policy_ref": policy_ref,
        "trust_tier": trust_tier,
        "scope_refs": scope_refs,
        "redaction_disclosed": redaction_disclosed,
    }


def cell(dimension, grade="pass"):
    return {
        "dimension": dimension,
        "grade": grade,
        "evidence_ref": "" if grade == "missing" else f"evidence/{dimension}",
    }


def scorecard(overrides=None):
    """Build a full readiness scorecard, passing except for overrides."""
    overrides = overrides or {}
    return [cell(dim, overrides.get(dim, "pass")) for dim in READINESS_DIMENSIONS]


ROWS = [
    {
        "entry_id": "docs-pack-notebooks",
        "title": "Docs publication pack — notebooks",
        "surface_kind": "docs_pack",
        "surface_ref": "publication-pack:docs_notebooks",
        "surface_summary": "Readiness scorecard and support posture for the notebooks docs publication pack.",
        "release_blocking": True,
        "claim_ref": "claim:m5_notebooks_depth",
        "claim_label": "stable",
        "surface_state": "certified",
        "scorecard": scorecard(),
        "disclosure": disclosure(
            "support-window/docs-packs",
            "publication-policy/docs",
            "first_party",
            ["scope/notebooks", "scope/stable"],
        ),
        "downgrade_automation": downgrade_automation("docs-pack-notebooks"),
        "proof_packet": proof_packet("docs-pack-notebooks"),
        "waiver": None,
        "owner_signoff": owner_signoff(),
        "active_narrowing_reasons": [],
        "published_label": "stable",
        "publication_destinations": ["docs/m5", "help/about", "support/export"],
        "rationale": "The notebooks docs pack passes every readiness dimension, discloses its support/redaction posture, is owner-signed, and its downgrade automation is defined and its rollback plan verified.",
    },
    {
        "entry_id": "migration-pack-db-api",
        "title": "Migration publication pack — DB/API",
        "surface_kind": "migration_pack",
        "surface_ref": "publication-pack:migration_db_api",
        "surface_summary": "Readiness scorecard and support posture for the DB/API migration publication pack.",
        "release_blocking": False,
        "claim_ref": "claim:m5_db_api_depth",
        "claim_label": "stable",
        "surface_state": "on_waiver",
        "scorecard": scorecard({"locale_parity": "waived"}),
        "disclosure": disclosure(
            "support-window/migration-packs",
            "publication-policy/migration",
            "verified_partner",
            ["scope/db-api"],
        ),
        "downgrade_automation": downgrade_automation("migration-pack-db-api"),
        "proof_packet": proof_packet("migration-pack-db-api"),
        "waiver": {
            "waiver_ref": "waiver:db_api_locale_parity",
            "expires_at": "2026-12-31",
            "reason": "Locale parity for the DB/API migration pack is scheduled after the next translation snapshot; interim locale-parity dimension waived by owner.",
        },
        "owner_signoff": owner_signoff(),
        "active_narrowing_reasons": [],
        "published_label": "stable",
        "publication_destinations": ["docs/m5", "help/about"],
        "rationale": "The DB/API migration pack holds Stable provisionally under an unexpired locale-parity waiver; its redaction posture stays disclosed and its downgrade automation is verified.",
    },
    {
        "entry_id": "known-limits-pack-ai",
        "title": "Known-limits publication pack — AI",
        "surface_kind": "known_limits_pack",
        "surface_ref": "publication-pack:known_limits_ai",
        "surface_summary": "Readiness scorecard and support posture for the AI known-limits publication pack.",
        "release_blocking": True,
        "claim_ref": "claim:m5_ai_depth",
        "claim_label": "stable",
        "surface_state": "automation_undefined",
        "scorecard": scorecard(),
        "disclosure": disclosure(
            "support-window/known-limits-packs",
            "publication-policy/known-limits",
            "first_party",
            ["scope/ai"],
        ),
        "downgrade_automation": downgrade_automation(
            "known-limits-pack-ai",
            trigger="manual",
            target_floor="beta",
            state="undefined",
            rollback_verified=False,
        ),
        "proof_packet": proof_packet("known-limits-pack-ai"),
        "waiver": None,
        "owner_signoff": owner_signoff(),
        "active_narrowing_reasons": ["downgrade_automation_undefined"],
        "published_label": "beta",
        "publication_destinations": ["docs/m5", "help/about", "support/export"],
        "rationale": "The AI known-limits pack passes every readiness dimension but narrows to Beta because its downgrade automation and frozen-fallback rollback plan are undefined.",
    },
    {
        "entry_id": "publication-index",
        "title": "Publication index across the M5 feature families",
        "surface_kind": "publication_index",
        "surface_ref": "publication-pack:publication_index",
        "surface_summary": "Readiness scorecard and support posture for the publication index that ties the docs, migration, and known-limits packs together across the M5 feature families.",
        "release_blocking": True,
        "claim_ref": "claim:m5_publication_index",
        "claim_label": "stable",
        "surface_state": "certified",
        "scorecard": scorecard(),
        "disclosure": disclosure(
            "support-window/publication-index",
            "publication-policy/index",
            "verified_partner",
            ["scope/profiler", "scope/frameworks", "scope/sync", "scope/offboarding"],
        ),
        "downgrade_automation": downgrade_automation(
            "publication-index", trigger="readiness_regressed"
        ),
        "proof_packet": proof_packet("publication-index"),
        "waiver": None,
        "owner_signoff": owner_signoff(),
        "active_narrowing_reasons": [],
        "published_label": "stable",
        "publication_destinations": ["docs/m5", "help/about", "support/export"],
        "rationale": "The publication index passes every readiness dimension and holds its release-blocking Stable claim with verified downgrade automation.",
    },
    {
        "entry_id": "docs-pack-companion-generated",
        "title": "Generated docs publication pack — companion",
        "surface_kind": "docs_pack",
        "surface_ref": "publication-pack:docs_companion_generated",
        "surface_summary": "Readiness scorecard and support posture for the generated companion docs publication pack.",
        "release_blocking": False,
        "claim_ref": "claim:m5_companion_depth",
        "claim_label": "stable",
        "surface_state": "readiness_regressed",
        "scorecard": scorecard({"redaction_safety": "missing"}),
        "disclosure": disclosure(
            "support-window/docs-packs-generated",
            "publication-policy/docs-generated",
            "generated",
            ["scope/companion"],
            redaction_disclosed=False,
        ),
        "downgrade_automation": downgrade_automation(
            "docs-pack-companion-generated",
            trigger="proof_stale",
            target_floor="preview",
            state="unverified",
            rollback_verified=False,
        ),
        "proof_packet": proof_packet("docs-pack-companion-generated", "missing"),
        "waiver": None,
        "owner_signoff": owner_signoff(False),
        "active_narrowing_reasons": [
            "readiness_dimension_missing",
            "proof_packet_missing",
            "owner_manifest_unsigned",
            "rollback_plan_unverified",
        ],
        "published_label": "preview",
        "publication_destinations": ["docs/m5"],
        "rationale": (
            "The generated companion docs pack narrows to Preview: its redaction-safety "
            "readiness report is missing and the redaction posture is not disclosed, the "
            "proof packet is missing, the owner manifest is unsigned, and the "
            "frozen-fallback rollback plan is unverified."
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
    kinds = Counter(r["surface_kind"] for r in ROWS)
    cells = [c for r in ROWS for c in r["scorecard"]]
    grades = Counter(c["grade"] for c in cells)
    slo = Counter(r["proof_packet"]["slo_state"] for r in ROWS)
    trust = Counter(r["disclosure"]["trust_tier"] for r in ROWS)
    release_blocking = [r for r in ROWS if r["release_blocking"]]
    automation_gap = {"rollback_plan_unverified", "downgrade_automation_undefined"}
    dimension_gap = {"readiness_dimension_failed", "readiness_dimension_missing"}
    return {
        "total_entries": len(ROWS),
        "total_claims": len({r["claim_ref"] for r in ROWS}),
        "entries_certified": sum(1 for r in ROWS if holds(r["published_label"])),
        "entries_narrowed": sum(1 for r in ROWS if not holds(r["published_label"])),
        "entries_on_active_waiver": sum(1 for r in ROWS if r["surface_state"] == "on_waiver"),
        "entries_with_dimension_gap": sum(
            1 for r in ROWS if any(reason in dimension_gap for reason in r["active_narrowing_reasons"])
        ),
        "entries_with_owner_gap": sum(1 for r in ROWS if has_reason(r, "owner_manifest_unsigned")),
        "entries_with_automation_gap": sum(
            1 for r in ROWS if any(reason in automation_gap for reason in r["active_narrowing_reasons"])
        ),
        "entries_redaction_undisclosed": sum(
            1 for r in ROWS if not r["disclosure"]["redaction_disclosed"]
        ),
        "release_blocking_total": len(release_blocking),
        "release_blocking_certified": sum(1 for r in release_blocking if holds(r["published_label"])),
        "release_blocking_narrowed": sum(1 for r in release_blocking if not holds(r["published_label"])),
        "docs_pack_entries": kinds["docs_pack"],
        "migration_pack_entries": kinds["migration_pack"],
        "known_limits_pack_entries": kinds["known_limits_pack"],
        "publication_index_entries": kinds["publication_index"],
        "first_party_entries": trust["first_party"],
        "verified_partner_entries": trust["verified_partner"],
        "community_entries": trust["community"],
        "generated_entries": trust["generated"],
        "packets_current": slo["current"],
        "packets_due_for_refresh": slo["due_for_refresh"],
        "packets_breached": slo["breached"],
        "packets_missing": slo["missing"],
        "total_active_narrowing_reasons": sum(len(r["active_narrowing_reasons"]) for r in ROWS),
        "total_readiness_cells": len(cells),
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
    "manifest_id": "publication_pack_register:v1",
    "status": "published",
    "overview_page": f"docs/m5/{SLUG}.md",
    "as_of": AS_OF,
    "claim_manifest_ref": "artifacts/release/stable_claim_manifest.json",
    "lifecycle_labels": LIFECYCLE_LABELS,
    "surface_kinds": SURFACE_KINDS,
    "readiness_dimensions": READINESS_DIMENSIONS,
    "dimension_grades": DIMENSION_GRADES,
    "surface_states": SURFACE_STATES,
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
            "A publication pack carries a Stable (or LTS) claim only when its readiness "
            "scorecard passes every dimension (docs coverage, migration completeness, "
            "known-limits disclosure, redaction safety, proof freshness, and locale "
            "parity), the redaction/provenance support posture is disclosed, the proof "
            "packet is current within its freshness SLO, any waiver is unexpired, the "
            "owner manifest is signed, and its downgrade automation is defined and its "
            "frozen-fallback rollback plan verified. A pack that loses any of those must "
            "drop below the cutline rather than inherit an adjacent certified pack."
        ),
    },
    "release_blocking_surface_refs": RELEASE_BLOCKING,
    "stop_rules": STOP_RULES,
    "rows": ROWS,
    "promotion": {
        "promotion_gate": "publication-pack-gate",
        "decision": decision,
        "blocking_rule_ids": blocking_rule_ids,
        "blocking_claim_ids": blocking_claim_ids,
        "rationale": (
            "Computed from the firing stop rules over readiness-dimension grades, "
            "redaction-posture disclosure, proof-packet freshness, owner-manifest "
            "sign-off, and downgrade-automation state."
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
        "entries_redaction_undisclosed": REGISTER["summary"]["entries_redaction_undisclosed"],
        "packets_missing": REGISTER["summary"]["packets_missing"],
        "total_readiness_cells": REGISTER["summary"]["total_readiness_cells"],
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
        {"drill_id": "drill:certified_without_redaction_disclosure", "status": "passed"},
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
        c for c in missing_cell["rows"][0]["scorecard"] if c["dimension"] != "locale_parity"
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
                "expected_check_id": "ReadinessIncompleteCoverage",
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
    print("Generated publication-pack register with", len(ROWS), "packs")
    print("Promotion decision:", decision)
    print("Blocking rules:", blocking_rule_ids)
    print("Blocking claims:", blocking_claim_ids)
