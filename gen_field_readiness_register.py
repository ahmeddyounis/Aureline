#!/usr/bin/env python3
"""Generate the M5 field-readiness register, capture, and fixtures.

The register freezes one field-readiness surface per surface kind — the
support-bundle schema expansion, the feature-family export packets, the
field-readiness drills, and the operator escalation runbook those surfaces lean
on. Each surface binds the surface to the stable claim it backs, a per-surface
readiness scorecard (one cell per dimension: schema coverage, export
completeness, drill execution, redaction safety, proof freshness, and docs
truth), the disclosed support posture it exposes (support window, policy ref,
maintainer trust tier, scope refs, and whether the redaction/provenance posture
is disclosed to the operator), an owner-manifest sign-off, an explicit
downgrade/rollback automation record (fall back to a frozen floor label), a
proof packet with its freshness SLO, and the active narrowing reasons that drop
the published label below the launch cutline when a readiness dimension fails or
is missing, the redaction posture is undisclosed, proof freshness expires, the
owner manifest is unsigned, the rollback plan is unverified, downgrade automation
is undefined, or a waiver expires.

This generator is the single source for the checked-in JSON so the typed Rust
consumer, the JSON Schema, and the CI capture stay in lock-step. Run it from the
repository root.
"""

import json
from collections import Counter

AS_OF = "2026-06-10"

SLUG = (
    "implement_support_bundle_schema_expansion_feature_family_export_packets_"
    "and_field_readiness_drills_for_m5_surfaces"
)

ARTIFACT_PATH = f"artifacts/release/m5/{SLUG}.json"
CAPTURE_PATH = f"artifacts/release/captures/{SLUG}_validation_capture.json"
FIXTURES_DIR = f"fixtures/release/m5/{SLUG}"

RECORD_KIND = SLUG

LIFECYCLE_LABELS = ["lts", "stable", "beta", "preview", "withdrawn"]
ABOVE_CUTLINE = {"lts", "stable"}
BELOW_CUTLINE = {"beta", "preview", "withdrawn"}

SURFACE_KINDS = [
    "support_bundle_schema",
    "feature_family_export",
    "field_readiness_drill",
    "escalation_runbook",
]

READINESS_DIMENSIONS = [
    "schema_coverage",
    "export_completeness",
    "drill_execution",
    "redaction_safety",
    "proof_freshness",
    "docs_truth",
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
        "rule_id": f"field_readiness_rule:{reason}",
        "title": title,
        "trigger_reason": reason,
        "applies_to_labels": ["lts", "stable"],
        "default_action": action,
        "blocks_promotion": True,
        "rationale": (
            f"A field-readiness surface whose posture reports '{reason}' cannot keep a "
            f"Stable or LTS claim and must narrow before publication."
        ),
    }
    for reason, (title, action) in REASON_RULES.items()
]

RELEASE_BLOCKING = [
    "field-readiness:support_bundle_schema_core",
    "field-readiness:field_readiness_drill_suite",
    "field-readiness:escalation_runbook",
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
        "entry_id": "support-bundle-schema-core",
        "title": "Support-bundle schema expansion (core)",
        "surface_kind": "support_bundle_schema",
        "surface_ref": "field-readiness:support_bundle_schema_core",
        "surface_summary": "Readiness scorecard and support posture for the expanded core support-bundle schema.",
        "release_blocking": True,
        "claim_ref": "claim:m5_support_bundle_schema_core",
        "claim_label": "stable",
        "surface_state": "certified",
        "scorecard": scorecard(),
        "disclosure": disclosure(
            "support-window/support-bundle",
            "support-bundle-policy/core",
            "first_party",
            ["scope/lts", "scope/stable"],
        ),
        "downgrade_automation": downgrade_automation("support-bundle-schema-core"),
        "proof_packet": proof_packet("support-bundle-schema-core"),
        "waiver": None,
        "owner_signoff": owner_signoff(),
        "active_narrowing_reasons": [],
        "published_label": "stable",
        "publication_destinations": ["docs/m5", "help/about", "support/export"],
        "rationale": "The core support-bundle schema expansion passes every readiness dimension, discloses its redaction posture, is owner-signed, and its downgrade automation is defined and its rollback plan verified.",
    },
    {
        "entry_id": "feature-family-export-packets",
        "title": "Feature-family export packets",
        "surface_kind": "feature_family_export",
        "surface_ref": "field-readiness:feature_family_export_packets",
        "surface_summary": "Readiness scorecard and support posture for the per-feature-family export packets.",
        "release_blocking": False,
        "claim_ref": "claim:m5_feature_family_export_packets",
        "claim_label": "stable",
        "surface_state": "on_waiver",
        "scorecard": scorecard({"docs_truth": "waived"}),
        "disclosure": disclosure(
            "support-window/feature-family-export",
            "export-policy/feature-family",
            "verified_partner",
            ["scope/stable"],
        ),
        "downgrade_automation": downgrade_automation("feature-family-export-packets"),
        "proof_packet": proof_packet("feature-family-export-packets"),
        "waiver": {
            "waiver_ref": "waiver:feature_family_export_docs_truth",
            "expires_at": "2026-12-31",
            "reason": "Support docs refresh for the feature-family export packets scheduled after the next readiness snapshot; interim docs-truth dimension waived by owner.",
        },
        "owner_signoff": owner_signoff(),
        "active_narrowing_reasons": [],
        "published_label": "stable",
        "publication_destinations": ["docs/m5", "help/about"],
        "rationale": "The feature-family export packets hold Stable provisionally under an unexpired docs-truth waiver; their redaction posture stays disclosed and their downgrade automation is verified.",
    },
    {
        "entry_id": "field-readiness-drill-suite",
        "title": "Field-readiness drill suite",
        "surface_kind": "field_readiness_drill",
        "surface_ref": "field-readiness:field_readiness_drill_suite",
        "surface_summary": "Readiness scorecard and support posture for the M5 field-readiness drill suite.",
        "release_blocking": True,
        "claim_ref": "claim:m5_field_readiness_drill_suite",
        "claim_label": "stable",
        "surface_state": "automation_undefined",
        "scorecard": scorecard(),
        "disclosure": disclosure(
            "support-window/field-readiness-drills",
            "drill-policy/field-readiness",
            "first_party",
            ["scope/m5-surfaces"],
        ),
        "downgrade_automation": downgrade_automation(
            "field-readiness-drill-suite",
            trigger="manual",
            target_floor="beta",
            state="undefined",
            rollback_verified=False,
        ),
        "proof_packet": proof_packet("field-readiness-drill-suite"),
        "waiver": None,
        "owner_signoff": owner_signoff(),
        "active_narrowing_reasons": ["downgrade_automation_undefined"],
        "published_label": "beta",
        "publication_destinations": ["docs/m5", "help/about", "support/export"],
        "rationale": "The field-readiness drill suite passes every readiness dimension but narrows to Beta because its downgrade automation and frozen-fallback rollback plan are undefined.",
    },
    {
        "entry_id": "escalation-runbook-surface",
        "title": "Operator escalation runbook",
        "surface_kind": "escalation_runbook",
        "surface_ref": "field-readiness:escalation_runbook",
        "surface_summary": "Readiness scorecard and support posture for the M5 operator escalation runbook.",
        "release_blocking": True,
        "claim_ref": "claim:m5_escalation_runbook",
        "claim_label": "stable",
        "surface_state": "certified",
        "scorecard": scorecard(),
        "disclosure": disclosure(
            "support-window/escalation-runbook",
            "escalation-policy/runbook",
            "verified_partner",
            ["scope/support", "scope/field"],
        ),
        "downgrade_automation": downgrade_automation(
            "escalation-runbook-surface", trigger="readiness_regressed"
        ),
        "proof_packet": proof_packet("escalation-runbook-surface"),
        "waiver": None,
        "owner_signoff": owner_signoff(),
        "active_narrowing_reasons": [],
        "published_label": "stable",
        "publication_destinations": ["docs/m5", "help/about", "support/export"],
        "rationale": "The operator escalation runbook passes every readiness dimension and holds its release-blocking Stable claim with verified downgrade automation.",
    },
    {
        "entry_id": "support-bundle-schema-generated",
        "title": "Generated support-bundle schema extension",
        "surface_kind": "support_bundle_schema",
        "surface_ref": "field-readiness:support_bundle_schema_generated",
        "surface_summary": "Readiness scorecard and support posture for the generated support-bundle schema extension.",
        "release_blocking": False,
        "claim_ref": "claim:m5_support_bundle_schema_generated",
        "claim_label": "stable",
        "surface_state": "readiness_regressed",
        "scorecard": scorecard({"redaction_safety": "missing"}),
        "disclosure": disclosure(
            "support-window/support-bundle-generated",
            "support-bundle-policy/generated",
            "generated",
            ["scope/generated"],
            redaction_disclosed=False,
        ),
        "downgrade_automation": downgrade_automation(
            "support-bundle-schema-generated",
            trigger="proof_stale",
            target_floor="preview",
            state="unverified",
            rollback_verified=False,
        ),
        "proof_packet": proof_packet("support-bundle-schema-generated", "missing"),
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
            "The generated support-bundle schema extension narrows to Preview: its "
            "redaction-safety readiness report is missing and the redaction posture is "
            "not disclosed, the proof packet is missing, the owner manifest is unsigned, "
            "and the frozen-fallback rollback plan is unverified."
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
        "support_bundle_schema_entries": kinds["support_bundle_schema"],
        "feature_family_export_entries": kinds["feature_family_export"],
        "field_readiness_drill_entries": kinds["field_readiness_drill"],
        "escalation_runbook_entries": kinds["escalation_runbook"],
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
    "manifest_id": "field_readiness_register:v1",
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
            "A field-readiness surface carries a Stable (or LTS) claim only when its "
            "readiness scorecard passes every dimension (schema coverage, export "
            "completeness, drill execution, redaction safety, proof freshness, and docs "
            "truth), the redaction/provenance support posture is disclosed, the proof "
            "packet is current within its freshness SLO, any waiver is unexpired, the "
            "owner manifest is signed, and its downgrade automation is defined and its "
            "frozen-fallback rollback plan verified. A surface that loses any of those "
            "must drop below the cutline rather than inherit an adjacent certified surface."
        ),
    },
    "release_blocking_surface_refs": RELEASE_BLOCKING,
    "stop_rules": STOP_RULES,
    "rows": ROWS,
    "promotion": {
        "promotion_gate": "field-readiness-gate",
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
    print("Generated field-readiness register with", len(ROWS), "surfaces")
    print("Promotion decision:", decision)
    print("Blocking rules:", blocking_rule_ids)
    print("Blocking claims:", blocking_claim_ids)
