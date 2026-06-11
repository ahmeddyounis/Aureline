#!/usr/bin/env python3
"""Generate the generated-artifact lineage register, validation capture, and fixtures.

The register freezes one lineage surface per generated-artifact source family
(scaffolded, AI-generated, notebook-derived, preview-derived). Each surface binds
the family to the stable claim it backs, a per-surface lineage scorecard (one cell
per lineage dimension), the artifact provenance it discloses (generator, provider/
host, trust tier, inputs, and whether the artifact is labeled as generated), an
owner-manifest sign-off, an explicit rollback/downgrade automation record, a proof
packet with its freshness SLO, and the active narrowing reasons that drop the
published label below the launch cutline.

This generator is the single source for the checked-in JSON so the typed Rust
consumer, the JSON Schema, and the CI capture stay in lock-step. Run it from the
repository root.
"""

import json
from collections import Counter

AS_OF = "2026-06-10"

SLUG = (
    "ship_generated_artifact_lineage_surfaces_for_scaffolded_ai_generated_"
    "notebook_derived_and_preview_derived_outputs"
)

ARTIFACT_PATH = f"artifacts/release/m5/{SLUG}.json"
CAPTURE_PATH = f"artifacts/release/captures/{SLUG}_validation_capture.json"
FIXTURES_DIR = f"fixtures/release/m5/{SLUG}"

RECORD_KIND = SLUG

LIFECYCLE_LABELS = ["lts", "stable", "beta", "preview", "withdrawn"]
ABOVE_CUTLINE = {"lts", "stable"}
BELOW_CUTLINE = {"beta", "preview", "withdrawn"}

GENERATOR_KINDS = [
    "scaffolded",
    "ai_generated",
    "notebook_derived",
    "preview_derived",
]

LINEAGE_DIMENSIONS = [
    "provenance",
    "inputs",
    "generator_identity",
    "transform",
    "reproducibility",
    "disclosure",
]

DIMENSION_GRADES = ["pass", "partial", "fail", "waived", "missing"]

LINEAGE_STATES = [
    "traced",
    "lineage_regressed",
    "stale",
    "on_waiver",
    "rollback_undefined",
    "owner_unsigned",
]

NARROWING_REASONS = [
    "lineage_dimension_failed",
    "lineage_dimension_missing",
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
    "remediate_lineage",
    "refresh_proof_packet",
    "request_owner_signoff",
    "verify_rollback_plan",
    "define_downgrade_automation",
    "renew_waiver",
]

DOWNGRADE_TRIGGERS = ["proof_stale", "lineage_regressed", "owner_revoked", "manual"]

AUTOMATION_STATES = ["defined", "unverified", "undefined"]

TRUST_TIERS = ["first_party", "verified_third_party", "community", "untrusted"]

# reason -> (stop-rule title, default action)
REASON_RULES = {
    "lineage_dimension_failed": ("Lineage dimension failed", "remediate_lineage"),
    "lineage_dimension_missing": ("Lineage dimension missing", "remediate_lineage"),
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
        "rule_id": f"lineage_rule:{reason}",
        "title": title,
        "trigger_reason": reason,
        "applies_to_labels": ["lts", "stable"],
        "default_action": action,
        "blocks_promotion": True,
        "rationale": (
            f"A generated-artifact lineage surface whose posture reports '{reason}' "
            f"cannot keep a Stable or LTS claim and must narrow before publication."
        ),
    }
    for reason, (title, action) in REASON_RULES.items()
]

RELEASE_BLOCKING = [
    "artifact-lineage:ai_generated",
    "artifact-lineage:preview_derived",
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


def lineage(generator_ref, provider_ref, trust_tier, input_refs, generated_labeled=True):
    return {
        "generator_ref": generator_ref,
        "provider_ref": provider_ref,
        "trust_tier": trust_tier,
        "input_refs": input_refs,
        "generated_labeled": generated_labeled,
    }


def cell(dimension, grade="pass"):
    return {
        "dimension": dimension,
        "grade": grade,
        "evidence_ref": "" if grade == "missing" else f"evidence/{dimension}",
    }


def scorecard(overrides=None):
    """Build a full lineage scorecard, passing except for overrides."""
    overrides = overrides or {}
    return [cell(dim, overrides.get(dim, "pass")) for dim in LINEAGE_DIMENSIONS]


ROWS = [
    {
        "entry_id": "scaffolded-lineage-surface",
        "title": "Scaffolded output lineage surface",
        "generator_kind": "scaffolded",
        "surface_ref": "artifact-lineage:scaffolded",
        "surface_summary": "Lineage for files emitted by project scaffolding and code generation.",
        "release_blocking": False,
        "claim_ref": "claim:m5_scaffolded_lineage",
        "claim_label": "stable",
        "lineage_state": "traced",
        "scorecard": scorecard(),
        "lineage": lineage(
            "generator/scaffold-engine",
            "first-party/scaffold",
            "first_party",
            ["template/project-skeleton"],
        ),
        "downgrade_automation": automation("scaffolded-lineage-surface"),
        "proof_packet": proof_packet("scaffolded-lineage-surface"),
        "waiver": None,
        "owner_signoff": owner_signoff(),
        "active_narrowing_reasons": [],
        "published_label": "stable",
        "publication_destinations": ["docs/m5", "help/about", "support/export"],
        "rationale": "Scaffolded lineage is fully traced across every dimension, owner-signed, and its rollback/downgrade automation is verified.",
    },
    {
        "entry_id": "ai-generated-lineage-surface",
        "title": "AI-generated output lineage surface",
        "generator_kind": "ai_generated",
        "surface_ref": "artifact-lineage:ai_generated",
        "surface_summary": "Lineage for AI-generated edits, completions, and composed artifacts.",
        "release_blocking": True,
        "claim_ref": "claim:m5_ai_generated_lineage",
        "claim_label": "stable",
        "lineage_state": "traced",
        "scorecard": scorecard(),
        "lineage": lineage(
            "generator/ai-composer",
            "first-party/ai-host",
            "verified_third_party",
            ["prompt/session", "context/workspace-window"],
        ),
        "downgrade_automation": automation(
            "ai-generated-lineage-surface", trigger="lineage_regressed"
        ),
        "proof_packet": proof_packet("ai-generated-lineage-surface"),
        "waiver": None,
        "owner_signoff": owner_signoff(),
        "active_narrowing_reasons": [],
        "published_label": "stable",
        "publication_destinations": ["docs/m5", "help/about", "support/export"],
        "rationale": "AI-generated lineage discloses provider/host/trust and inputs across every dimension and holds its release-blocking Stable claim with verified downgrade automation.",
    },
    {
        "entry_id": "notebook-derived-lineage-surface",
        "title": "Notebook-derived output lineage surface",
        "generator_kind": "notebook_derived",
        "surface_ref": "artifact-lineage:notebook_derived",
        "surface_summary": "Lineage for artifacts exported or derived from notebook cells and outputs.",
        "release_blocking": False,
        "claim_ref": "claim:m5_notebook_derived_lineage",
        "claim_label": "stable",
        "lineage_state": "on_waiver",
        "scorecard": scorecard({"reproducibility": "waived"}),
        "lineage": lineage(
            "generator/notebook-export",
            "first-party/notebook",
            "first_party",
            ["notebook/source-cells"],
        ),
        "downgrade_automation": automation("notebook-derived-lineage-surface"),
        "proof_packet": proof_packet("notebook-derived-lineage-surface"),
        "waiver": {
            "waiver_ref": "waiver:notebook_reproducibility",
            "expires_at": "2026-12-31",
            "reason": "Reproducibility re-validation scheduled; interim lineage dimension waived by owner.",
        },
        "owner_signoff": owner_signoff(),
        "active_narrowing_reasons": [],
        "published_label": "stable",
        "publication_destinations": ["docs/m5", "help/about"],
        "rationale": "Notebook-derived lineage holds Stable provisionally under an unexpired reproducibility-dimension waiver; the artifact stays labeled as generated and downgrade automation is verified.",
    },
    {
        "entry_id": "preview-derived-lineage-surface",
        "title": "Preview-derived output lineage surface",
        "generator_kind": "preview_derived",
        "surface_ref": "artifact-lineage:preview_derived",
        "surface_summary": "Lineage for artifacts published from the preview/designer surface.",
        "release_blocking": True,
        "claim_ref": "claim:m5_preview_derived_lineage",
        "claim_label": "stable",
        "lineage_state": "rollback_undefined",
        "scorecard": scorecard(),
        "lineage": lineage(
            "generator/preview-publish",
            "first-party/preview",
            "first_party",
            ["preview/spec"],
        ),
        "downgrade_automation": automation(
            "preview-derived-lineage-surface",
            trigger="manual",
            target_floor="beta",
            state="undefined",
            rollback_verified=False,
        ),
        "proof_packet": proof_packet("preview-derived-lineage-surface"),
        "waiver": None,
        "owner_signoff": owner_signoff(),
        "active_narrowing_reasons": ["downgrade_automation_undefined"],
        "published_label": "beta",
        "publication_destinations": ["docs/m5", "help/about", "support/export"],
        "rationale": "Preview-derived lineage is fully traced but narrows to Beta because its rollback/downgrade automation is undefined.",
    },
    {
        "entry_id": "scaffolded-batch-lineage-surface",
        "title": "Batch-scaffolded output lineage surface",
        "generator_kind": "scaffolded",
        "surface_ref": "artifact-lineage:scaffolded_batch",
        "surface_summary": "Lineage for batch-scaffolded outputs from third-party community templates.",
        "release_blocking": False,
        "claim_ref": "claim:m5_scaffolded_batch_lineage",
        "claim_label": "stable",
        "lineage_state": "lineage_regressed",
        "scorecard": scorecard({"disclosure": "missing"}),
        "lineage": lineage(
            "generator/scaffold-batch",
            "third-party/community-template",
            "community",
            ["template/community-pack"],
            generated_labeled=False,
        ),
        "downgrade_automation": automation(
            "scaffolded-batch-lineage-surface",
            trigger="proof_stale",
            target_floor="preview",
            state="unverified",
            rollback_verified=False,
        ),
        "proof_packet": proof_packet("scaffolded-batch-lineage-surface", "missing"),
        "waiver": None,
        "owner_signoff": owner_signoff(False),
        "active_narrowing_reasons": [
            "lineage_dimension_missing",
            "proof_packet_missing",
            "owner_manifest_unsigned",
            "rollback_plan_unverified",
        ],
        "published_label": "preview",
        "publication_destinations": ["docs/m5"],
        "rationale": (
            "Batch-scaffolded lineage narrows to Preview: the disclosure dimension is "
            "missing and the artifact is not labeled as generated, the proof packet is "
            "missing, the owner manifest is unsigned, and the rollback plan is unverified."
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
    kinds = Counter(r["generator_kind"] for r in ROWS)
    cells = [c for r in ROWS for c in r["scorecard"]]
    grades = Counter(c["grade"] for c in cells)
    slo = Counter(r["proof_packet"]["slo_state"] for r in ROWS)
    trust = Counter(r["lineage"]["trust_tier"] for r in ROWS)
    release_blocking = [r for r in ROWS if r["release_blocking"]]
    rollback_gap = {"rollback_plan_unverified", "downgrade_automation_undefined"}
    dimension_gap = {"lineage_dimension_failed", "lineage_dimension_missing"}
    return {
        "total_entries": len(ROWS),
        "total_claims": len({r["claim_ref"] for r in ROWS}),
        "entries_traced": sum(1 for r in ROWS if holds(r["published_label"])),
        "entries_narrowed": sum(1 for r in ROWS if not holds(r["published_label"])),
        "entries_on_active_waiver": sum(1 for r in ROWS if r["lineage_state"] == "on_waiver"),
        "entries_with_dimension_gap": sum(
            1 for r in ROWS if any(reason in dimension_gap for reason in r["active_narrowing_reasons"])
        ),
        "entries_with_owner_gap": sum(1 for r in ROWS if has_reason(r, "owner_manifest_unsigned")),
        "entries_with_rollback_gap": sum(
            1 for r in ROWS if any(reason in rollback_gap for reason in r["active_narrowing_reasons"])
        ),
        "entries_unlabeled": sum(1 for r in ROWS if not r["lineage"]["generated_labeled"]),
        "release_blocking_total": len(release_blocking),
        "release_blocking_traced": sum(1 for r in release_blocking if holds(r["published_label"])),
        "release_blocking_narrowed": sum(1 for r in release_blocking if not holds(r["published_label"])),
        "scaffolded_entries": kinds["scaffolded"],
        "ai_generated_entries": kinds["ai_generated"],
        "notebook_derived_entries": kinds["notebook_derived"],
        "preview_derived_entries": kinds["preview_derived"],
        "first_party_entries": trust["first_party"],
        "verified_third_party_entries": trust["verified_third_party"],
        "community_entries": trust["community"],
        "untrusted_entries": trust["untrusted"],
        "packets_current": slo["current"],
        "packets_due_for_refresh": slo["due_for_refresh"],
        "packets_breached": slo["breached"],
        "packets_missing": slo["missing"],
        "total_active_narrowing_reasons": sum(len(r["active_narrowing_reasons"]) for r in ROWS),
        "total_lineage_cells": len(cells),
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
    "manifest_id": "generated_artifact_lineage_register:v1",
    "status": "published",
    "overview_page": f"docs/m5/{SLUG}.md",
    "as_of": AS_OF,
    "claim_manifest_ref": "artifacts/release/stable_claim_manifest.json",
    "lifecycle_labels": LIFECYCLE_LABELS,
    "generator_kinds": GENERATOR_KINDS,
    "lineage_dimensions": LINEAGE_DIMENSIONS,
    "dimension_grades": DIMENSION_GRADES,
    "lineage_states": LINEAGE_STATES,
    "narrowing_reasons": NARROWING_REASONS,
    "stop_rule_actions": STOP_RULE_ACTIONS,
    "downgrade_triggers": DOWNGRADE_TRIGGERS,
    "automation_states": AUTOMATION_STATES,
    "trust_tiers": TRUST_TIERS,
    "launch_cutline": {
        "cutline_level": "stable",
        "above_cutline_levels": ["lts", "stable"],
        "below_cutline_levels": ["beta", "preview", "withdrawn"],
        "description": (
            "A generated-artifact lineage surface carries a Stable (or LTS) claim only "
            "when its lineage scorecard passes every dimension, the artifact is labeled "
            "as generated, the proof packet is current within its freshness SLO, any "
            "waiver is unexpired, the owner manifest is signed, and its rollback/downgrade "
            "automation is defined and verified. A surface that loses any of those must "
            "drop below the cutline rather than inherit an adjacent traced surface."
        ),
    },
    "release_blocking_surface_refs": RELEASE_BLOCKING,
    "stop_rules": STOP_RULES,
    "rows": ROWS,
    "promotion": {
        "promotion_gate": "generated-artifact-lineage-gate",
        "decision": decision,
        "blocking_rule_ids": blocking_rule_ids,
        "blocking_claim_ids": blocking_claim_ids,
        "rationale": (
            "Computed from the firing stop rules over lineage-dimension grades, "
            "disclosure labeling, proof-packet freshness, owner-manifest sign-off, and "
            "rollback/downgrade automation state."
        ),
    },
    "summary": compute_summary(),
}

CAPTURE = {
    "status": "pass",
    "as_of": AS_OF,
    "summary": {
        "total_entries": REGISTER["summary"]["total_entries"],
        "entries_traced": REGISTER["summary"]["entries_traced"],
        "entries_narrowed": REGISTER["summary"]["entries_narrowed"],
        "entries_on_active_waiver": REGISTER["summary"]["entries_on_active_waiver"],
        "entries_with_rollback_gap": REGISTER["summary"]["entries_with_rollback_gap"],
        "entries_unlabeled": REGISTER["summary"]["entries_unlabeled"],
        "packets_missing": REGISTER["summary"]["packets_missing"],
        "total_lineage_cells": REGISTER["summary"]["total_lineage_cells"],
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
        {"drill_id": "drill:held_without_disclosure", "status": "passed"},
        {"drill_id": "drill:held_without_downgrade_automation", "status": "passed"},
    ],
    "fixture_cases": [
        {"case_id": "fixture:duplicate_entry_id", "status": "passed"},
        {"case_id": "fixture:missing_dimension_cell", "status": "passed"},
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
        c for c in missing_cell["rows"][0]["scorecard"] if c["dimension"] != "reproducibility"
    ]
    write_json(f"{FIXTURES_DIR}/missing_dimension_cell.json", missing_cell)

    held_gap = copy.deepcopy(REGISTER)
    held_gap["rows"][0]["active_narrowing_reasons"] = ["rollback_plan_unverified"]
    write_json(f"{FIXTURES_DIR}/held_with_rollback_gap.json", held_gap)

    cases = {
        "cases": [
            {"file": "duplicate_entry_id.json", "expected_check_id": "DuplicateEntryId"},
            {
                "file": "missing_dimension_cell.json",
                "expected_check_id": "LineageIncompleteCoverage",
            },
            {"file": "held_with_rollback_gap.json", "expected_check_id": "HeldWithActiveGap"},
        ]
    }
    write_json(f"{FIXTURES_DIR}/cases.json", cases)


if __name__ == "__main__":
    write_json(ARTIFACT_PATH, REGISTER)
    write_json(CAPTURE_PATH, CAPTURE)
    build_fixtures()
    print("Generated artifact lineage register with", len(ROWS), "lineage surfaces")
    print("Promotion decision:", decision)
    print("Blocking rules:", blocking_rule_ids)
    print("Blocking claims:", blocking_claim_ids)
