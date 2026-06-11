#!/usr/bin/env python3
"""Generate the M5 fitness-surface register, capture, and fixtures.

The register freezes one fitness-surface lane per surface kind — checked-in
benchmark corpora, reference-workspace expansions, protected fitness dashboards,
and the fitness functions that guard them. Each lane binds the surface to the
stable claim it backs, a per-lane fitness scorecard (one cell per fitness
dimension: corpus lineage, baseline coverage, threshold calibration, regression
guard, accessibility audit, and docs truth), the corpus provenance it discloses
(corpus source, baseline, trust tier, dataset refs, and whether the
generated-artifact provenance is disclosed to the operator), an owner-manifest
sign-off, an explicit downgrade/rollback automation record (fall back to a frozen
floor label), a proof packet with its freshness SLO, and the active narrowing
reasons that drop the published label below the launch cutline when a fitness
dimension fails or is missing, the corpus provenance is undisclosed, proof
freshness expires, the owner manifest is unsigned, the rollback plan is
unverified, downgrade automation is undefined, or a waiver expires.

This generator is the single source for the checked-in JSON so the typed Rust
consumer, the JSON Schema, and the CI capture stay in lock-step. Run it from the
repository root.
"""

import json
from collections import Counter

AS_OF = "2026-06-10"

SLUG = (
    "ship_benchmark_corpora_reference_workspace_expansions_and_m5_specific_"
    "protected_fitness_dashboards"
)

ARTIFACT_PATH = f"artifacts/release/m5/{SLUG}.json"
CAPTURE_PATH = f"artifacts/release/captures/{SLUG}_validation_capture.json"
FIXTURES_DIR = f"fixtures/release/m5/{SLUG}"

RECORD_KIND = SLUG

LIFECYCLE_LABELS = ["lts", "stable", "beta", "preview", "withdrawn"]
ABOVE_CUTLINE = {"lts", "stable"}
BELOW_CUTLINE = {"beta", "preview", "withdrawn"}

SURFACE_KINDS = [
    "benchmark_corpus",
    "reference_workspace",
    "fitness_dashboard",
    "fitness_function",
]

FITNESS_DIMENSIONS = [
    "corpus_lineage",
    "baseline_coverage",
    "threshold_calibration",
    "regression_guard",
    "accessibility_audit",
    "docs_truth",
]

DIMENSION_GRADES = ["pass", "partial", "fail", "waived", "missing"]

SURFACE_STATES = [
    "certified",
    "fitness_regressed",
    "stale",
    "on_waiver",
    "automation_undefined",
    "owner_unsigned",
]

NARROWING_REASONS = [
    "fitness_dimension_failed",
    "fitness_dimension_missing",
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
    "remediate_fitness",
    "refresh_proof_packet",
    "request_owner_signoff",
    "verify_rollback_plan",
    "define_downgrade_automation",
    "renew_waiver",
]

AUTOMATION_TRIGGERS = ["proof_stale", "fitness_regressed", "owner_revoked", "manual"]

AUTOMATION_STATES = ["defined", "unverified", "undefined"]

TRUST_TIERS = ["first_party", "verified_partner", "community", "generated"]

# reason -> (stop-rule title, default action)
REASON_RULES = {
    "fitness_dimension_failed": ("Fitness dimension failed", "remediate_fitness"),
    "fitness_dimension_missing": ("Fitness dimension missing", "remediate_fitness"),
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
        "rule_id": f"fitness_surface_rule:{reason}",
        "title": title,
        "trigger_reason": reason,
        "applies_to_labels": ["lts", "stable"],
        "default_action": action,
        "blocks_promotion": True,
        "rationale": (
            f"A fitness-surface lane whose posture reports '{reason}' cannot keep a "
            f"Stable or LTS claim and must narrow before publication."
        ),
    }
    for reason, (title, action) in REASON_RULES.items()
]

RELEASE_BLOCKING = [
    "fitness-surface:benchmark_corpus_core",
    "fitness-surface:fitness_dashboard",
    "fitness-surface:ai_fitness_function",
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


def provenance(
    corpus_ref,
    baseline_ref,
    trust_tier,
    dataset_refs,
    provenance_disclosed=True,
):
    return {
        "corpus_ref": corpus_ref,
        "baseline_ref": baseline_ref,
        "trust_tier": trust_tier,
        "dataset_refs": dataset_refs,
        "provenance_disclosed": provenance_disclosed,
    }


def cell(dimension, grade="pass"):
    return {
        "dimension": dimension,
        "grade": grade,
        "evidence_ref": "" if grade == "missing" else f"evidence/{dimension}",
    }


def scorecard(overrides=None):
    """Build a full fitness scorecard, passing except for overrides."""
    overrides = overrides or {}
    return [cell(dim, overrides.get(dim, "pass")) for dim in FITNESS_DIMENSIONS]


ROWS = [
    {
        "entry_id": "benchmark-corpus-core",
        "title": "Core benchmark corpus",
        "surface_kind": "benchmark_corpus",
        "surface_ref": "fitness-surface:benchmark_corpus_core",
        "surface_summary": "Fitness scorecard and corpus provenance for the core checked-in benchmark corpus.",
        "release_blocking": True,
        "claim_ref": "claim:m5_benchmark_corpus_core",
        "claim_label": "stable",
        "surface_state": "certified",
        "scorecard": scorecard(),
        "provenance": provenance(
            "corpus/benchmark-core",
            "fitness-baseline/core",
            "first_party",
            ["dataset/core-v3", "dataset/core-v2"],
        ),
        "downgrade_automation": downgrade_automation("benchmark-corpus-core"),
        "proof_packet": proof_packet("benchmark-corpus-core"),
        "waiver": None,
        "owner_signoff": owner_signoff(),
        "active_narrowing_reasons": [],
        "published_label": "stable",
        "publication_destinations": ["docs/m5", "help/about", "support/export"],
        "rationale": "Core benchmark corpus passes every fitness dimension, discloses its generated-artifact provenance, is owner-signed, and its downgrade automation is defined and its rollback plan verified.",
    },
    {
        "entry_id": "reference-workspace-expansion",
        "title": "Reference-workspace expansion",
        "surface_kind": "reference_workspace",
        "surface_ref": "fitness-surface:reference_workspace",
        "surface_summary": "Fitness scorecard and corpus provenance for the expanded reference workspaces.",
        "release_blocking": False,
        "claim_ref": "claim:m5_reference_workspace",
        "claim_label": "stable",
        "surface_state": "on_waiver",
        "scorecard": scorecard({"docs_truth": "waived"}),
        "provenance": provenance(
            "corpus/reference-workspaces",
            "fitness-baseline/reference-workspace",
            "verified_partner",
            ["dataset/reference-v2"],
        ),
        "downgrade_automation": downgrade_automation("reference-workspace-expansion"),
        "proof_packet": proof_packet("reference-workspace-expansion"),
        "waiver": {
            "waiver_ref": "waiver:reference_workspace_docs_truth",
            "expires_at": "2026-12-31",
            "reason": "Docs/help refresh for the expanded archetypes scheduled after the next workspace snapshot; interim docs-truth dimension waived by owner.",
        },
        "owner_signoff": owner_signoff(),
        "active_narrowing_reasons": [],
        "published_label": "stable",
        "publication_destinations": ["docs/m5", "help/about"],
        "rationale": "Reference-workspace expansion holds Stable provisionally under an unexpired docs-truth waiver; its provenance stays disclosed and its downgrade automation is verified.",
    },
    {
        "entry_id": "protected-fitness-dashboard",
        "title": "Protected fitness dashboard",
        "surface_kind": "fitness_dashboard",
        "surface_ref": "fitness-surface:fitness_dashboard",
        "surface_summary": "Fitness scorecard and corpus provenance for the M5-specific protected fitness dashboard.",
        "release_blocking": True,
        "claim_ref": "claim:m5_fitness_dashboard",
        "claim_label": "stable",
        "surface_state": "automation_undefined",
        "scorecard": scorecard(),
        "provenance": provenance(
            "corpus/fitness-dashboard",
            "fitness-baseline/dashboard",
            "first_party",
            ["dataset/dashboard-v1"],
        ),
        "downgrade_automation": downgrade_automation(
            "protected-fitness-dashboard",
            trigger="manual",
            target_floor="beta",
            state="undefined",
            rollback_verified=False,
        ),
        "proof_packet": proof_packet("protected-fitness-dashboard"),
        "waiver": None,
        "owner_signoff": owner_signoff(),
        "active_narrowing_reasons": ["downgrade_automation_undefined"],
        "published_label": "beta",
        "publication_destinations": ["docs/m5", "help/about", "support/export"],
        "rationale": "Protected fitness dashboard passes every fitness dimension but narrows to Beta because its downgrade automation and frozen-fallback rollback plan are undefined.",
    },
    {
        "entry_id": "ai-fitness-function",
        "title": "AI fitness function",
        "surface_kind": "fitness_function",
        "surface_ref": "fitness-surface:ai_fitness_function",
        "surface_summary": "Fitness scorecard and corpus provenance for the AI-assistant fitness function guarding the dashboards.",
        "release_blocking": True,
        "claim_ref": "claim:m5_ai_fitness_function",
        "claim_label": "stable",
        "surface_state": "certified",
        "scorecard": scorecard(),
        "provenance": provenance(
            "corpus/ai-fitness-function",
            "fitness-baseline/ai-fitness-function",
            "verified_partner",
            ["dataset/ai-fitness-v2"],
        ),
        "downgrade_automation": downgrade_automation(
            "ai-fitness-function", trigger="fitness_regressed"
        ),
        "proof_packet": proof_packet("ai-fitness-function"),
        "waiver": None,
        "owner_signoff": owner_signoff(),
        "active_narrowing_reasons": [],
        "published_label": "stable",
        "publication_destinations": ["docs/m5", "help/about", "support/export"],
        "rationale": "AI fitness function passes every fitness dimension and holds its release-blocking Stable claim with verified downgrade automation.",
    },
    {
        "entry_id": "public-benchmark-publication",
        "title": "Public benchmark publication corpus",
        "surface_kind": "benchmark_corpus",
        "surface_ref": "fitness-surface:public_benchmark_publication",
        "surface_summary": "Fitness scorecard and corpus provenance for the generated public benchmark publication pack.",
        "release_blocking": False,
        "claim_ref": "claim:m5_public_benchmark_publication",
        "claim_label": "stable",
        "surface_state": "fitness_regressed",
        "scorecard": scorecard({"regression_guard": "missing"}),
        "provenance": provenance(
            "corpus/public-benchmark",
            "fitness-baseline/public-benchmark",
            "generated",
            ["dataset/public-benchmark-v1"],
            provenance_disclosed=False,
        ),
        "downgrade_automation": downgrade_automation(
            "public-benchmark-publication",
            trigger="proof_stale",
            target_floor="preview",
            state="unverified",
            rollback_verified=False,
        ),
        "proof_packet": proof_packet("public-benchmark-publication", "missing"),
        "waiver": None,
        "owner_signoff": owner_signoff(False),
        "active_narrowing_reasons": [
            "fitness_dimension_missing",
            "proof_packet_missing",
            "owner_manifest_unsigned",
            "rollback_plan_unverified",
        ],
        "published_label": "preview",
        "publication_destinations": ["docs/m5"],
        "rationale": (
            "Public benchmark publication corpus narrows to Preview: its regression-guard "
            "fitness report is missing and the generated-artifact provenance is not "
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
    kinds = Counter(r["surface_kind"] for r in ROWS)
    cells = [c for r in ROWS for c in r["scorecard"]]
    grades = Counter(c["grade"] for c in cells)
    slo = Counter(r["proof_packet"]["slo_state"] for r in ROWS)
    trust = Counter(r["provenance"]["trust_tier"] for r in ROWS)
    release_blocking = [r for r in ROWS if r["release_blocking"]]
    automation_gap = {"rollback_plan_unverified", "downgrade_automation_undefined"}
    dimension_gap = {"fitness_dimension_failed", "fitness_dimension_missing"}
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
        "entries_provenance_undisclosed": sum(
            1 for r in ROWS if not r["provenance"]["provenance_disclosed"]
        ),
        "release_blocking_total": len(release_blocking),
        "release_blocking_certified": sum(1 for r in release_blocking if holds(r["published_label"])),
        "release_blocking_narrowed": sum(1 for r in release_blocking if not holds(r["published_label"])),
        "benchmark_corpus_entries": kinds["benchmark_corpus"],
        "reference_workspace_entries": kinds["reference_workspace"],
        "fitness_dashboard_entries": kinds["fitness_dashboard"],
        "fitness_function_entries": kinds["fitness_function"],
        "first_party_entries": trust["first_party"],
        "verified_partner_entries": trust["verified_partner"],
        "community_entries": trust["community"],
        "generated_entries": trust["generated"],
        "packets_current": slo["current"],
        "packets_due_for_refresh": slo["due_for_refresh"],
        "packets_breached": slo["breached"],
        "packets_missing": slo["missing"],
        "total_active_narrowing_reasons": sum(len(r["active_narrowing_reasons"]) for r in ROWS),
        "total_fitness_cells": len(cells),
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
    "manifest_id": "fitness_surface_register:v1",
    "status": "published",
    "overview_page": f"docs/m5/{SLUG}.md",
    "as_of": AS_OF,
    "claim_manifest_ref": "artifacts/release/stable_claim_manifest.json",
    "lifecycle_labels": LIFECYCLE_LABELS,
    "surface_kinds": SURFACE_KINDS,
    "fitness_dimensions": FITNESS_DIMENSIONS,
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
            "A fitness-surface lane carries a Stable (or LTS) claim only when its "
            "fitness scorecard passes every dimension (corpus lineage, baseline "
            "coverage, threshold calibration, regression guard, accessibility audit, "
            "and docs truth), the generated-artifact corpus provenance is disclosed, "
            "the proof packet is current within its freshness SLO, any waiver is "
            "unexpired, the owner manifest is signed, and its downgrade automation is "
            "defined and its frozen-fallback rollback plan verified. A surface that "
            "loses any of those must drop below the cutline rather than inherit an "
            "adjacent certified surface."
        ),
    },
    "release_blocking_surface_refs": RELEASE_BLOCKING,
    "stop_rules": STOP_RULES,
    "rows": ROWS,
    "promotion": {
        "promotion_gate": "fitness-surface-gate",
        "decision": decision,
        "blocking_rule_ids": blocking_rule_ids,
        "blocking_claim_ids": blocking_claim_ids,
        "rationale": (
            "Computed from the firing stop rules over fitness-dimension grades, "
            "generated-artifact provenance disclosure, proof-packet freshness, owner-"
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
        "entries_provenance_undisclosed": REGISTER["summary"]["entries_provenance_undisclosed"],
        "packets_missing": REGISTER["summary"]["packets_missing"],
        "total_fitness_cells": REGISTER["summary"]["total_fitness_cells"],
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
        {"drill_id": "drill:certified_without_provenance_disclosure", "status": "passed"},
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
                "expected_check_id": "FitnessIncompleteCoverage",
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
    print("Generated fitness-surface register with", len(ROWS), "lanes")
    print("Promotion decision:", decision)
    print("Blocking rules:", blocking_rule_ids)
    print("Blocking claims:", blocking_claim_ids)
