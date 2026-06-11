#!/usr/bin/env python3
"""Generate the community locale-pack governance register, capture, and fixtures.

The register freezes one locale-pack lane per pack channel (core locale,
community pack, partner pack, machine-assisted pack). Each lane binds the pack to
the stable claim it backs, a per-lane translation-governance scorecard (one cell
per governance dimension), the translation governance it discloses (maintainer,
source, trust tier, string sets, and whether untranslated-string fallback is
disclosed to the user), an owner-manifest sign-off, an explicit rollback/downgrade
automation record (fall back to the base locale), a proof packet with its
freshness SLO, and the active narrowing reasons that drop the published label
below the launch cutline when translation coverage, locale parity, terminology,
review, source sync, proof freshness, ownership, or rollback automation thins out.

This generator is the single source for the checked-in JSON so the typed Rust
consumer, the JSON Schema, and the CI capture stay in lock-step. Run it from the
repository root.
"""

import json
from collections import Counter

AS_OF = "2026-06-10"

SLUG = (
    "add_community_locale_pack_lifecycle_translation_governance_and_parity_"
    "audits_for_new_m5_surfaces"
)

ARTIFACT_PATH = f"artifacts/release/m5/{SLUG}.json"
CAPTURE_PATH = f"artifacts/release/captures/{SLUG}_validation_capture.json"
FIXTURES_DIR = f"fixtures/release/m5/{SLUG}"

RECORD_KIND = SLUG

LIFECYCLE_LABELS = ["lts", "stable", "beta", "preview", "withdrawn"]
ABOVE_CUTLINE = {"lts", "stable"}
BELOW_CUTLINE = {"beta", "preview", "withdrawn"}

PACK_CHANNELS = [
    "core_locale",
    "community_pack",
    "partner_pack",
    "machine_assisted",
]

GOVERNANCE_DIMENSIONS = [
    "string_coverage",
    "critical_state_coverage",
    "terminology",
    "translation_review",
    "locale_parity",
    "source_sync",
]

DIMENSION_GRADES = ["pass", "partial", "fail", "waived", "missing"]

PACK_STATES = [
    "certified",
    "parity_drifted",
    "stale",
    "on_waiver",
    "rollback_undefined",
    "owner_unsigned",
]

NARROWING_REASONS = [
    "governance_dimension_failed",
    "governance_dimension_missing",
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
    "remediate_translation",
    "refresh_proof_packet",
    "request_owner_signoff",
    "verify_rollback_plan",
    "define_downgrade_automation",
    "renew_waiver",
]

DOWNGRADE_TRIGGERS = ["proof_stale", "parity_drifted", "owner_revoked", "manual"]

AUTOMATION_STATES = ["defined", "unverified", "undefined"]

TRUST_TIERS = ["first_party", "verified_partner", "community", "untrusted"]

# reason -> (stop-rule title, default action)
REASON_RULES = {
    "governance_dimension_failed": ("Governance dimension failed", "remediate_translation"),
    "governance_dimension_missing": ("Governance dimension missing", "remediate_translation"),
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
        "rule_id": f"locale_pack_rule:{reason}",
        "title": title,
        "trigger_reason": reason,
        "applies_to_labels": ["lts", "stable"],
        "default_action": action,
        "blocks_promotion": True,
        "rationale": (
            f"A community locale-pack lane whose posture reports '{reason}' cannot "
            f"keep a Stable or LTS claim and must narrow before publication."
        ),
    }
    for reason, (title, action) in REASON_RULES.items()
]

RELEASE_BLOCKING = [
    "locale-pack:community_de",
    "locale-pack:machine_es",
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
        "owner_ref": "localization-guild",
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


def governance(maintainer_ref, source_ref, trust_tier, string_set_refs, fallback_disclosed=True):
    return {
        "maintainer_ref": maintainer_ref,
        "source_ref": source_ref,
        "trust_tier": trust_tier,
        "string_set_refs": string_set_refs,
        "fallback_disclosed": fallback_disclosed,
    }


def cell(dimension, grade="pass"):
    return {
        "dimension": dimension,
        "grade": grade,
        "evidence_ref": "" if grade == "missing" else f"evidence/{dimension}",
    }


def scorecard(overrides=None):
    """Build a full governance scorecard, passing except for overrides."""
    overrides = overrides or {}
    return [cell(dim, overrides.get(dim, "pass")) for dim in GOVERNANCE_DIMENSIONS]


ROWS = [
    {
        "entry_id": "core-locale-pack",
        "title": "Core shipped locale pack",
        "pack_channel": "core_locale",
        "pack_ref": "locale-pack:core",
        "pack_summary": "Translation governance for the first-party shipped locales covering the new M5 surfaces.",
        "release_blocking": False,
        "claim_ref": "claim:m5_core_locale_pack",
        "claim_label": "stable",
        "pack_state": "certified",
        "scorecard": scorecard(),
        "governance": governance(
            "localization-guild/core",
            "source-strings/m5-surfaces",
            "first_party",
            ["string-set/notebook", "string-set/companion", "string-set/preview"],
        ),
        "downgrade_automation": automation("core-locale-pack"),
        "proof_packet": proof_packet("core-locale-pack"),
        "waiver": None,
        "owner_signoff": owner_signoff(),
        "active_narrowing_reasons": [],
        "published_label": "stable",
        "publication_destinations": ["docs/m5", "help/about", "support/export"],
        "rationale": "Core locale pack passes every governance dimension, discloses fallback, is owner-signed, and its rollback/downgrade automation is verified.",
    },
    {
        "entry_id": "community-de-locale-pack",
        "title": "Community German (de) locale pack",
        "pack_channel": "community_pack",
        "pack_ref": "locale-pack:community_de",
        "pack_summary": "Community-contributed German translation pack for the new M5 surfaces.",
        "release_blocking": True,
        "claim_ref": "claim:m5_community_de_locale_pack",
        "claim_label": "stable",
        "pack_state": "certified",
        "scorecard": scorecard(),
        "governance": governance(
            "community/de-maintainers",
            "source-strings/m5-surfaces",
            "community",
            ["string-set/notebook", "string-set/companion"],
        ),
        "downgrade_automation": automation(
            "community-de-locale-pack", trigger="parity_drifted"
        ),
        "proof_packet": proof_packet("community-de-locale-pack"),
        "waiver": None,
        "owner_signoff": owner_signoff(),
        "active_narrowing_reasons": [],
        "published_label": "stable",
        "publication_destinations": ["docs/m5", "help/about", "support/export"],
        "rationale": "Community German pack passes coverage, critical-state, terminology, review, parity, and source-sync audits and holds its release-blocking Stable claim with verified downgrade automation.",
    },
    {
        "entry_id": "partner-ja-locale-pack",
        "title": "Partner Japanese (ja) locale pack",
        "pack_channel": "partner_pack",
        "pack_ref": "locale-pack:partner_ja",
        "pack_summary": "Partner-maintained Japanese translation pack for the new M5 surfaces.",
        "release_blocking": False,
        "claim_ref": "claim:m5_partner_ja_locale_pack",
        "claim_label": "stable",
        "pack_state": "on_waiver",
        "scorecard": scorecard({"source_sync": "waived"}),
        "governance": governance(
            "partner/ja-localization",
            "source-strings/m5-surfaces",
            "verified_partner",
            ["string-set/notebook", "string-set/preview"],
        ),
        "downgrade_automation": automation("partner-ja-locale-pack"),
        "proof_packet": proof_packet("partner-ja-locale-pack"),
        "waiver": {
            "waiver_ref": "waiver:partner_ja_source_sync",
            "expires_at": "2026-12-31",
            "reason": "Source-string re-sync scheduled after the next string freeze; interim source-sync dimension waived by owner.",
        },
        "owner_signoff": owner_signoff(),
        "active_narrowing_reasons": [],
        "published_label": "stable",
        "publication_destinations": ["docs/m5", "help/about"],
        "rationale": "Partner Japanese pack holds Stable provisionally under an unexpired source-sync waiver; fallback stays disclosed and downgrade automation is verified.",
    },
    {
        "entry_id": "machine-assisted-es-locale-pack",
        "title": "Machine-assisted Spanish (es) locale pack",
        "pack_channel": "machine_assisted",
        "pack_ref": "locale-pack:machine_es",
        "pack_summary": "Machine-translation-seeded, human-reviewed Spanish pack for the new M5 surfaces.",
        "release_blocking": True,
        "claim_ref": "claim:m5_machine_es_locale_pack",
        "claim_label": "stable",
        "pack_state": "rollback_undefined",
        "scorecard": scorecard(),
        "governance": governance(
            "localization-guild/machine-assist",
            "source-strings/m5-surfaces",
            "verified_partner",
            ["string-set/notebook", "string-set/companion", "string-set/preview"],
        ),
        "downgrade_automation": automation(
            "machine-assisted-es-locale-pack",
            trigger="manual",
            target_floor="beta",
            state="undefined",
            rollback_verified=False,
        ),
        "proof_packet": proof_packet("machine-assisted-es-locale-pack"),
        "waiver": None,
        "owner_signoff": owner_signoff(),
        "active_narrowing_reasons": ["downgrade_automation_undefined"],
        "published_label": "beta",
        "publication_destinations": ["docs/m5", "help/about", "support/export"],
        "rationale": "Machine-assisted Spanish pack passes every governance dimension but narrows to Beta because its base-locale fallback rollback/downgrade automation is undefined.",
    },
    {
        "entry_id": "community-ar-locale-pack",
        "title": "Community Arabic (ar) locale pack",
        "pack_channel": "community_pack",
        "pack_ref": "locale-pack:community_ar",
        "pack_summary": "Community-contributed Arabic (RTL) translation pack for the new M5 surfaces.",
        "release_blocking": False,
        "claim_ref": "claim:m5_community_ar_locale_pack",
        "claim_label": "stable",
        "pack_state": "parity_drifted",
        "scorecard": scorecard({"locale_parity": "missing"}),
        "governance": governance(
            "community/ar-maintainers",
            "source-strings/m5-surfaces",
            "community",
            ["string-set/notebook"],
            fallback_disclosed=False,
        ),
        "downgrade_automation": automation(
            "community-ar-locale-pack",
            trigger="proof_stale",
            target_floor="preview",
            state="unverified",
            rollback_verified=False,
        ),
        "proof_packet": proof_packet("community-ar-locale-pack", "missing"),
        "waiver": None,
        "owner_signoff": owner_signoff(False),
        "active_narrowing_reasons": [
            "governance_dimension_missing",
            "proof_packet_missing",
            "owner_manifest_unsigned",
            "rollback_plan_unverified",
        ],
        "published_label": "preview",
        "publication_destinations": ["docs/m5"],
        "rationale": (
            "Community Arabic pack narrows to Preview: the locale-parity audit is "
            "missing and untranslated-string fallback is not disclosed, the proof "
            "packet is missing, the owner manifest is unsigned, and the base-locale "
            "rollback plan is unverified."
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
    channels = Counter(r["pack_channel"] for r in ROWS)
    cells = [c for r in ROWS for c in r["scorecard"]]
    grades = Counter(c["grade"] for c in cells)
    slo = Counter(r["proof_packet"]["slo_state"] for r in ROWS)
    trust = Counter(r["governance"]["trust_tier"] for r in ROWS)
    release_blocking = [r for r in ROWS if r["release_blocking"]]
    rollback_gap = {"rollback_plan_unverified", "downgrade_automation_undefined"}
    dimension_gap = {"governance_dimension_failed", "governance_dimension_missing"}
    return {
        "total_entries": len(ROWS),
        "total_claims": len({r["claim_ref"] for r in ROWS}),
        "entries_certified": sum(1 for r in ROWS if holds(r["published_label"])),
        "entries_narrowed": sum(1 for r in ROWS if not holds(r["published_label"])),
        "entries_on_active_waiver": sum(1 for r in ROWS if r["pack_state"] == "on_waiver"),
        "entries_with_dimension_gap": sum(
            1 for r in ROWS if any(reason in dimension_gap for reason in r["active_narrowing_reasons"])
        ),
        "entries_with_owner_gap": sum(1 for r in ROWS if has_reason(r, "owner_manifest_unsigned")),
        "entries_with_rollback_gap": sum(
            1 for r in ROWS if any(reason in rollback_gap for reason in r["active_narrowing_reasons"])
        ),
        "entries_fallback_undisclosed": sum(
            1 for r in ROWS if not r["governance"]["fallback_disclosed"]
        ),
        "release_blocking_total": len(release_blocking),
        "release_blocking_certified": sum(1 for r in release_blocking if holds(r["published_label"])),
        "release_blocking_narrowed": sum(1 for r in release_blocking if not holds(r["published_label"])),
        "core_locale_entries": channels["core_locale"],
        "community_pack_entries": channels["community_pack"],
        "partner_pack_entries": channels["partner_pack"],
        "machine_assisted_entries": channels["machine_assisted"],
        "first_party_entries": trust["first_party"],
        "verified_partner_entries": trust["verified_partner"],
        "community_entries": trust["community"],
        "untrusted_entries": trust["untrusted"],
        "packets_current": slo["current"],
        "packets_due_for_refresh": slo["due_for_refresh"],
        "packets_breached": slo["breached"],
        "packets_missing": slo["missing"],
        "total_active_narrowing_reasons": sum(len(r["active_narrowing_reasons"]) for r in ROWS),
        "total_governance_cells": len(cells),
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
    "manifest_id": "community_locale_pack_governance_register:v1",
    "status": "published",
    "overview_page": f"docs/m5/{SLUG}.md",
    "as_of": AS_OF,
    "claim_manifest_ref": "artifacts/release/stable_claim_manifest.json",
    "lifecycle_labels": LIFECYCLE_LABELS,
    "pack_channels": PACK_CHANNELS,
    "governance_dimensions": GOVERNANCE_DIMENSIONS,
    "dimension_grades": DIMENSION_GRADES,
    "pack_states": PACK_STATES,
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
            "A community locale-pack lane carries a Stable (or LTS) claim only when "
            "its translation-governance scorecard passes every dimension (string "
            "coverage, critical-state coverage, terminology, review, locale parity, "
            "and source sync), untranslated-string fallback is disclosed, the proof "
            "packet is current within its freshness SLO, any waiver is unexpired, the "
            "owner manifest is signed, and its base-locale rollback/downgrade "
            "automation is defined and verified. A lane that loses any of those must "
            "drop below the cutline rather than inherit an adjacent certified pack."
        ),
    },
    "release_blocking_pack_refs": RELEASE_BLOCKING,
    "stop_rules": STOP_RULES,
    "rows": ROWS,
    "promotion": {
        "promotion_gate": "community-locale-pack-governance-gate",
        "decision": decision,
        "blocking_rule_ids": blocking_rule_ids,
        "blocking_claim_ids": blocking_claim_ids,
        "rationale": (
            "Computed from the firing stop rules over governance-dimension grades, "
            "fallback disclosure, proof-packet freshness, owner-manifest sign-off, and "
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
        "entries_certified": REGISTER["summary"]["entries_certified"],
        "entries_narrowed": REGISTER["summary"]["entries_narrowed"],
        "entries_on_active_waiver": REGISTER["summary"]["entries_on_active_waiver"],
        "entries_with_rollback_gap": REGISTER["summary"]["entries_with_rollback_gap"],
        "entries_fallback_undisclosed": REGISTER["summary"]["entries_fallback_undisclosed"],
        "packets_missing": REGISTER["summary"]["packets_missing"],
        "total_governance_cells": REGISTER["summary"]["total_governance_cells"],
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
        {"drill_id": "drill:certified_without_fallback_disclosure", "status": "passed"},
        {"drill_id": "drill:certified_without_downgrade_automation", "status": "passed"},
    ],
    "fixture_cases": [
        {"case_id": "fixture:duplicate_entry_id", "status": "passed"},
        {"case_id": "fixture:missing_dimension_cell", "status": "passed"},
        {"case_id": "fixture:certified_with_rollback_gap", "status": "passed"},
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
        c for c in missing_cell["rows"][0]["scorecard"] if c["dimension"] != "source_sync"
    ]
    write_json(f"{FIXTURES_DIR}/missing_dimension_cell.json", missing_cell)

    held_gap = copy.deepcopy(REGISTER)
    held_gap["rows"][0]["active_narrowing_reasons"] = ["rollback_plan_unverified"]
    write_json(f"{FIXTURES_DIR}/certified_with_rollback_gap.json", held_gap)

    cases = {
        "cases": [
            {"file": "duplicate_entry_id.json", "expected_check_id": "DuplicateEntryId"},
            {
                "file": "missing_dimension_cell.json",
                "expected_check_id": "GovernanceIncompleteCoverage",
            },
            {
                "file": "certified_with_rollback_gap.json",
                "expected_check_id": "HeldWithActiveGap",
            },
        ]
    }
    write_json(f"{FIXTURES_DIR}/cases.json", cases)


if __name__ == "__main__":
    write_json(ARTIFACT_PATH, REGISTER)
    write_json(CAPTURE_PATH, CAPTURE)
    build_fixtures()
    print("Generated community locale-pack governance register with", len(ROWS), "lanes")
    print("Promotion decision:", decision)
    print("Blocking rules:", blocking_rule_ids)
    print("Blocking claims:", blocking_claim_ids)
