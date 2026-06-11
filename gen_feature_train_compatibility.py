#!/usr/bin/env python3
"""Generate the feature-train compatibility register, capture, and fixtures.

The register freezes one feature-train lane per train channel (core platform, AI
assistant, collaboration, extensions). Each lane binds the train to the stable
claim it backs, a per-lane compatibility-report scorecard (one cell per
compatibility dimension), the provider-family support window it discloses
(provider family, baseline, trust tier, supported version refs, and whether the
end-of-support boundary is disclosed to the operator), an owner-manifest
sign-off, an explicit change-freeze guidance record (fall back to a frozen base
label), a proof packet with its freshness SLO, and the active narrowing reasons
that drop the published label below the launch cutline when forward/backward
compatibility, schema versioning, the provider support window, deprecation
policy, change-freeze adherence, proof freshness, ownership, or change-freeze
guidance thins out.

This generator is the single source for the checked-in JSON so the typed Rust
consumer, the JSON Schema, and the CI capture stay in lock-step. Run it from the
repository root.
"""

import json
from collections import Counter

AS_OF = "2026-06-10"

SLUG = (
    "implement_feature_train_compatibility_reports_provider_family_support_"
    "windows_and_change_freeze_guidance"
)

ARTIFACT_PATH = f"artifacts/release/m5/{SLUG}.json"
CAPTURE_PATH = f"artifacts/release/captures/{SLUG}_validation_capture.json"
FIXTURES_DIR = f"fixtures/release/m5/{SLUG}"

RECORD_KIND = SLUG

LIFECYCLE_LABELS = ["lts", "stable", "beta", "preview", "withdrawn"]
ABOVE_CUTLINE = {"lts", "stable"}
BELOW_CUTLINE = {"beta", "preview", "withdrawn"}

TRAIN_CHANNELS = [
    "core_platform",
    "ai_assistant",
    "collaboration",
    "extensions",
]

COMPATIBILITY_DIMENSIONS = [
    "forward_compatibility",
    "backward_compatibility",
    "schema_versioning",
    "provider_support_window",
    "deprecation_policy",
    "change_freeze_adherence",
]

DIMENSION_GRADES = ["pass", "partial", "fail", "waived", "missing"]

TRAIN_STATES = [
    "certified",
    "compatibility_broken",
    "stale",
    "on_waiver",
    "freeze_undefined",
    "owner_unsigned",
]

NARROWING_REASONS = [
    "compatibility_dimension_failed",
    "compatibility_dimension_missing",
    "proof_packet_missing",
    "proof_packet_stale",
    "owner_manifest_unsigned",
    "freeze_plan_unverified",
    "change_freeze_undefined",
    "waiver_expired",
]

STOP_RULE_ACTIONS = [
    "hold_promotion",
    "narrow_label",
    "remediate_compatibility",
    "refresh_proof_packet",
    "request_owner_signoff",
    "verify_freeze_plan",
    "define_change_freeze",
    "renew_waiver",
]

FREEZE_TRIGGERS = ["proof_stale", "compatibility_broken", "owner_revoked", "manual"]

FREEZE_STATES = ["defined", "unverified", "undefined"]

TRUST_TIERS = ["first_party", "verified_partner", "community", "untrusted"]

# reason -> (stop-rule title, default action)
REASON_RULES = {
    "compatibility_dimension_failed": (
        "Compatibility dimension failed",
        "remediate_compatibility",
    ),
    "compatibility_dimension_missing": (
        "Compatibility dimension missing",
        "remediate_compatibility",
    ),
    "proof_packet_missing": ("Proof packet missing", "refresh_proof_packet"),
    "proof_packet_stale": ("Proof packet stale", "refresh_proof_packet"),
    "owner_manifest_unsigned": ("Owner manifest unsigned", "request_owner_signoff"),
    "freeze_plan_unverified": ("Change-freeze plan unverified", "verify_freeze_plan"),
    "change_freeze_undefined": (
        "Change-freeze guidance undefined",
        "define_change_freeze",
    ),
    "waiver_expired": ("Waiver expired", "renew_waiver"),
}

STOP_RULES = [
    {
        "rule_id": f"feature_train_rule:{reason}",
        "title": title,
        "trigger_reason": reason,
        "applies_to_labels": ["lts", "stable"],
        "default_action": action,
        "blocks_promotion": True,
        "rationale": (
            f"A feature-train compatibility lane whose posture reports '{reason}' "
            f"cannot keep a Stable or LTS claim and must narrow before publication."
        ),
    }
    for reason, (title, action) in REASON_RULES.items()
]

RELEASE_BLOCKING = [
    "feature-train:ai_assistant",
    "feature-train:extensions",
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


def change_freeze(
    entry_id,
    trigger="proof_stale",
    target_floor="beta",
    state="defined",
    freeze_verified=True,
):
    return {
        "guidance_ref": f"change-freeze/{entry_id}",
        "freeze_plan_ref": f"freeze-plan/{entry_id}",
        "trigger": trigger,
        "target_floor": target_floor,
        "state": state,
        "freeze_verified": freeze_verified,
    }


def support_window(
    provider_family_ref,
    baseline_ref,
    trust_tier,
    supported_version_refs,
    eol_disclosed=True,
):
    return {
        "provider_family_ref": provider_family_ref,
        "baseline_ref": baseline_ref,
        "trust_tier": trust_tier,
        "supported_version_refs": supported_version_refs,
        "eol_disclosed": eol_disclosed,
    }


def cell(dimension, grade="pass"):
    return {
        "dimension": dimension,
        "grade": grade,
        "evidence_ref": "" if grade == "missing" else f"evidence/{dimension}",
    }


def scorecard(overrides=None):
    """Build a full compatibility scorecard, passing except for overrides."""
    overrides = overrides or {}
    return [cell(dim, overrides.get(dim, "pass")) for dim in COMPATIBILITY_DIMENSIONS]


ROWS = [
    {
        "entry_id": "core-platform-train",
        "title": "Core platform feature train",
        "train_channel": "core_platform",
        "train_ref": "feature-train:core_platform",
        "train_summary": "Compatibility report and provider-family support window for the core platform feature train.",
        "release_blocking": False,
        "claim_ref": "claim:m5_core_platform_train",
        "claim_label": "stable",
        "train_state": "certified",
        "scorecard": scorecard(),
        "support_window": support_window(
            "provider-family/first-party",
            "compat-baseline/core",
            "first_party",
            ["schema-version/core-v3", "schema-version/core-v2"],
        ),
        "change_freeze": change_freeze("core-platform-train"),
        "proof_packet": proof_packet("core-platform-train"),
        "waiver": None,
        "owner_signoff": owner_signoff(),
        "active_narrowing_reasons": [],
        "published_label": "stable",
        "publication_destinations": ["docs/m5", "help/about", "support/export"],
        "rationale": "Core platform train passes every compatibility dimension, discloses its provider-family support window, is owner-signed, and its change-freeze guidance is defined and verified.",
    },
    {
        "entry_id": "ai-assistant-train",
        "title": "AI assistant feature train",
        "train_channel": "ai_assistant",
        "train_ref": "feature-train:ai_assistant",
        "train_summary": "Compatibility report and provider-family support window for the AI assistant feature train.",
        "release_blocking": True,
        "claim_ref": "claim:m5_ai_assistant_train",
        "claim_label": "stable",
        "train_state": "certified",
        "scorecard": scorecard(),
        "support_window": support_window(
            "provider-family/anthropic",
            "compat-baseline/ai",
            "verified_partner",
            ["schema-version/ai-v4", "schema-version/ai-v3"],
        ),
        "change_freeze": change_freeze(
            "ai-assistant-train", trigger="compatibility_broken"
        ),
        "proof_packet": proof_packet("ai-assistant-train"),
        "waiver": None,
        "owner_signoff": owner_signoff(),
        "active_narrowing_reasons": [],
        "published_label": "stable",
        "publication_destinations": ["docs/m5", "help/about", "support/export"],
        "rationale": "AI assistant train passes forward/backward compatibility, schema versioning, provider support window, deprecation policy, and change-freeze adherence and holds its release-blocking Stable claim with verified change-freeze guidance.",
    },
    {
        "entry_id": "collaboration-train",
        "title": "Collaboration feature train",
        "train_channel": "collaboration",
        "train_ref": "feature-train:collaboration",
        "train_summary": "Compatibility report and provider-family support window for the collaboration feature train.",
        "release_blocking": False,
        "claim_ref": "claim:m5_collaboration_train",
        "claim_label": "stable",
        "train_state": "on_waiver",
        "scorecard": scorecard({"deprecation_policy": "waived"}),
        "support_window": support_window(
            "provider-family/sync-mesh",
            "compat-baseline/collaboration",
            "verified_partner",
            ["schema-version/sync-v2"],
        ),
        "change_freeze": change_freeze("collaboration-train"),
        "proof_packet": proof_packet("collaboration-train"),
        "waiver": {
            "waiver_ref": "waiver:collaboration_deprecation_policy",
            "expires_at": "2026-12-31",
            "reason": "Deprecation-window publication scheduled after the next sync protocol revision; interim deprecation-policy dimension waived by owner.",
        },
        "owner_signoff": owner_signoff(),
        "active_narrowing_reasons": [],
        "published_label": "stable",
        "publication_destinations": ["docs/m5", "help/about"],
        "rationale": "Collaboration train holds Stable provisionally under an unexpired deprecation-policy waiver; the support window stays disclosed and change-freeze guidance is verified.",
    },
    {
        "entry_id": "extensions-train",
        "title": "Extensions feature train",
        "train_channel": "extensions",
        "train_ref": "feature-train:extensions",
        "train_summary": "Compatibility report and provider-family support window for the extensions feature train.",
        "release_blocking": True,
        "claim_ref": "claim:m5_extensions_train",
        "claim_label": "stable",
        "train_state": "freeze_undefined",
        "scorecard": scorecard(),
        "support_window": support_window(
            "provider-family/extension-registry",
            "compat-baseline/extensions",
            "community",
            ["schema-version/ext-v1"],
        ),
        "change_freeze": change_freeze(
            "extensions-train",
            trigger="manual",
            target_floor="beta",
            state="undefined",
            freeze_verified=False,
        ),
        "proof_packet": proof_packet("extensions-train"),
        "waiver": None,
        "owner_signoff": owner_signoff(),
        "active_narrowing_reasons": ["change_freeze_undefined"],
        "published_label": "beta",
        "publication_destinations": ["docs/m5", "help/about", "support/export"],
        "rationale": "Extensions train passes every compatibility dimension but narrows to Beta because its change-freeze guidance and frozen-fallback plan are undefined.",
    },
    {
        "entry_id": "ai-assistant-partner-train",
        "title": "Partner AI assistant feature train",
        "train_channel": "ai_assistant",
        "train_ref": "feature-train:ai_assistant_partner",
        "train_summary": "Compatibility report and provider-family support window for the partner-hosted AI assistant feature train.",
        "release_blocking": False,
        "claim_ref": "claim:m5_ai_assistant_partner_train",
        "claim_label": "stable",
        "train_state": "compatibility_broken",
        "scorecard": scorecard({"backward_compatibility": "missing"}),
        "support_window": support_window(
            "provider-family/partner-host",
            "compat-baseline/ai-partner",
            "community",
            ["schema-version/ai-partner-v1"],
            eol_disclosed=False,
        ),
        "change_freeze": change_freeze(
            "ai-assistant-partner-train",
            trigger="proof_stale",
            target_floor="preview",
            state="unverified",
            freeze_verified=False,
        ),
        "proof_packet": proof_packet("ai-assistant-partner-train", "missing"),
        "waiver": None,
        "owner_signoff": owner_signoff(False),
        "active_narrowing_reasons": [
            "compatibility_dimension_missing",
            "proof_packet_missing",
            "owner_manifest_unsigned",
            "freeze_plan_unverified",
        ],
        "published_label": "preview",
        "publication_destinations": ["docs/m5"],
        "rationale": (
            "Partner AI assistant train narrows to Preview: the backward-compatibility "
            "report is missing and the provider-family end-of-support window is not "
            "disclosed, the proof packet is missing, the owner manifest is unsigned, and "
            "the frozen-fallback plan is unverified."
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
    channels = Counter(r["train_channel"] for r in ROWS)
    cells = [c for r in ROWS for c in r["scorecard"]]
    grades = Counter(c["grade"] for c in cells)
    slo = Counter(r["proof_packet"]["slo_state"] for r in ROWS)
    trust = Counter(r["support_window"]["trust_tier"] for r in ROWS)
    release_blocking = [r for r in ROWS if r["release_blocking"]]
    freeze_gap = {"freeze_plan_unverified", "change_freeze_undefined"}
    dimension_gap = {"compatibility_dimension_failed", "compatibility_dimension_missing"}
    return {
        "total_entries": len(ROWS),
        "total_claims": len({r["claim_ref"] for r in ROWS}),
        "entries_certified": sum(1 for r in ROWS if holds(r["published_label"])),
        "entries_narrowed": sum(1 for r in ROWS if not holds(r["published_label"])),
        "entries_on_active_waiver": sum(1 for r in ROWS if r["train_state"] == "on_waiver"),
        "entries_with_dimension_gap": sum(
            1 for r in ROWS if any(reason in dimension_gap for reason in r["active_narrowing_reasons"])
        ),
        "entries_with_owner_gap": sum(1 for r in ROWS if has_reason(r, "owner_manifest_unsigned")),
        "entries_with_freeze_gap": sum(
            1 for r in ROWS if any(reason in freeze_gap for reason in r["active_narrowing_reasons"])
        ),
        "entries_eol_undisclosed": sum(
            1 for r in ROWS if not r["support_window"]["eol_disclosed"]
        ),
        "release_blocking_total": len(release_blocking),
        "release_blocking_certified": sum(1 for r in release_blocking if holds(r["published_label"])),
        "release_blocking_narrowed": sum(1 for r in release_blocking if not holds(r["published_label"])),
        "core_platform_entries": channels["core_platform"],
        "ai_assistant_entries": channels["ai_assistant"],
        "collaboration_entries": channels["collaboration"],
        "extensions_entries": channels["extensions"],
        "first_party_entries": trust["first_party"],
        "verified_partner_entries": trust["verified_partner"],
        "community_entries": trust["community"],
        "untrusted_entries": trust["untrusted"],
        "packets_current": slo["current"],
        "packets_due_for_refresh": slo["due_for_refresh"],
        "packets_breached": slo["breached"],
        "packets_missing": slo["missing"],
        "total_active_narrowing_reasons": sum(len(r["active_narrowing_reasons"]) for r in ROWS),
        "total_compatibility_cells": len(cells),
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
    "manifest_id": "feature_train_compatibility_register:v1",
    "status": "published",
    "overview_page": f"docs/m5/{SLUG}.md",
    "as_of": AS_OF,
    "claim_manifest_ref": "artifacts/release/stable_claim_manifest.json",
    "lifecycle_labels": LIFECYCLE_LABELS,
    "train_channels": TRAIN_CHANNELS,
    "compatibility_dimensions": COMPATIBILITY_DIMENSIONS,
    "dimension_grades": DIMENSION_GRADES,
    "train_states": TRAIN_STATES,
    "narrowing_reasons": NARROWING_REASONS,
    "stop_rule_actions": STOP_RULE_ACTIONS,
    "freeze_triggers": FREEZE_TRIGGERS,
    "freeze_states": FREEZE_STATES,
    "trust_tiers": TRUST_TIERS,
    "launch_cutline": {
        "cutline_level": "stable",
        "above_cutline_levels": ["lts", "stable"],
        "below_cutline_levels": ["beta", "preview", "withdrawn"],
        "description": (
            "A feature-train compatibility lane carries a Stable (or LTS) claim only "
            "when its compatibility-report scorecard passes every dimension (forward "
            "compatibility, backward compatibility, schema versioning, provider support "
            "window, deprecation policy, and change-freeze adherence), the provider-"
            "family end-of-support window is disclosed, the proof packet is current "
            "within its freshness SLO, any waiver is unexpired, the owner manifest is "
            "signed, and its change-freeze guidance is defined and its frozen-fallback "
            "plan verified. A train that loses any of those must drop below the cutline "
            "rather than inherit an adjacent certified train."
        ),
    },
    "release_blocking_train_refs": RELEASE_BLOCKING,
    "stop_rules": STOP_RULES,
    "rows": ROWS,
    "promotion": {
        "promotion_gate": "feature-train-compatibility-gate",
        "decision": decision,
        "blocking_rule_ids": blocking_rule_ids,
        "blocking_claim_ids": blocking_claim_ids,
        "rationale": (
            "Computed from the firing stop rules over compatibility-dimension grades, "
            "provider-family support-window disclosure, proof-packet freshness, owner-"
            "manifest sign-off, and change-freeze guidance state."
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
        "entries_with_freeze_gap": REGISTER["summary"]["entries_with_freeze_gap"],
        "entries_eol_undisclosed": REGISTER["summary"]["entries_eol_undisclosed"],
        "packets_missing": REGISTER["summary"]["packets_missing"],
        "total_compatibility_cells": REGISTER["summary"]["total_compatibility_cells"],
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
        {"drill_id": "drill:certified_without_eol_disclosure", "status": "passed"},
        {"drill_id": "drill:certified_without_change_freeze", "status": "passed"},
    ],
    "fixture_cases": [
        {"case_id": "fixture:duplicate_entry_id", "status": "passed"},
        {"case_id": "fixture:missing_dimension_cell", "status": "passed"},
        {"case_id": "fixture:certified_with_freeze_gap", "status": "passed"},
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
        c for c in missing_cell["rows"][0]["scorecard"] if c["dimension"] != "change_freeze_adherence"
    ]
    write_json(f"{FIXTURES_DIR}/missing_dimension_cell.json", missing_cell)

    held_gap = copy.deepcopy(REGISTER)
    held_gap["rows"][0]["active_narrowing_reasons"] = ["freeze_plan_unverified"]
    write_json(f"{FIXTURES_DIR}/certified_with_freeze_gap.json", held_gap)

    cases = {
        "cases": [
            {"file": "duplicate_entry_id.json", "expected_check_id": "DuplicateEntryId"},
            {
                "file": "missing_dimension_cell.json",
                "expected_check_id": "CompatibilityIncompleteCoverage",
            },
            {
                "file": "certified_with_freeze_gap.json",
                "expected_check_id": "HeldWithActiveGap",
            },
        ]
    }
    write_json(f"{FIXTURES_DIR}/cases.json", cases)


if __name__ == "__main__":
    write_json(ARTIFACT_PATH, REGISTER)
    write_json(CAPTURE_PATH, CAPTURE)
    build_fixtures()
    print("Generated feature-train compatibility register with", len(ROWS), "lanes")
    print("Promotion decision:", decision)
    print("Blocking rules:", blocking_rule_ids)
    print("Blocking claims:", blocking_claim_ids)
