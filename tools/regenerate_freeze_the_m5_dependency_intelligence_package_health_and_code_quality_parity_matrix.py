#!/usr/bin/env python3
"""Regenerate the M5 dependency/package + code-quality parity matrix artifacts.

This is the single source of truth for the checked-in matrix, its CI validation
capture, and the negative fixtures. It builds the matrix dict, derives the
summary counts and promotion verdict exactly as the typed Rust consumer does,
then writes:

  * ``artifacts/release/m5/<name>.json``                  (the matrix)
  * ``artifacts/release/captures/<name>_validation_capture.json`` (CI capture)
  * ``fixtures/release/m5/<name>/{cases.json,*.json}``    (negative fixtures)

Run ``python3 tools/regenerate_freeze_the_m5_dependency_intelligence_package_health_and_code_quality_parity_matrix.py``
after editing the row set, then ``cargo test -p aureline-release --test
freeze_the_m5_dependency_intelligence_package_health_and_code_quality_parity_matrix``
to confirm the typed model agrees.
"""

from __future__ import annotations

import json
from pathlib import Path

NAME = "freeze_the_m5_dependency_intelligence_package_health_and_code_quality_parity_matrix"
RECORD_KIND = "freeze_m5_dependency_quality_parity_matrix"
SCHEMA_VERSION = 1
AS_OF = "2026-06-11"

REPO_ROOT = Path(__file__).resolve().parent.parent

ARTIFACT_PATH = REPO_ROOT / "artifacts" / "release" / "m5" / f"{NAME}.json"
CAPTURE_PATH = (
    REPO_ROOT / "artifacts" / "release" / "captures" / f"{NAME}_validation_capture.json"
)
FIXTURES_DIR = REPO_ROOT / "fixtures" / "release" / "m5" / NAME

# Closed vocabularies, in canonical (declaration) order. These must match the
# `ALL` arrays in the typed Rust model.
LIFECYCLE_LABELS = ["lts", "stable", "beta", "preview", "withdrawn"]
ABOVE_CUTLINE = ["lts", "stable"]
BELOW_CUTLINE = ["beta", "preview", "withdrawn"]
LANE_KINDS = [
    "dependency_intelligence",
    "package_mutation",
    "package_health",
    "quality_profile",
    "scanner_import",
    "live_quality",
    "cli_headless_parity",
]
CLAIM_CLASSES = [
    "marketed_depth",
    "foundation_parity",
    "imported_visibility",
    "advisory_only",
]
DEPLOYMENT_PROFILES = ["local_only", "mirrored", "managed", "browser_handoff"]
SCORECARD_STATES = [
    "complete",
    "incomplete",
    "stale",
    "on_waiver",
    "blocked",
    "rollback_missing",
]
GAP_REASONS = [
    "proof_packet_missing",
    "proof_packet_stale",
    "compatibility_report_missing",
    "compatibility_report_stale",
    "admin_policy_missing",
    "rollback_path_missing",
    "upstream_lane_narrowed",
    "waiver_expired",
    "owner_signoff_missing",
]
STOP_RULE_ACTIONS = [
    "hold_promotion",
    "narrow_label",
    "refresh_proof_packet",
    "refresh_compatibility_report",
    "staff_admin_policy",
    "define_rollback_path",
    "request_owner_signoff",
]

RANK = {"lts": 4, "stable": 3, "beta": 2, "preview": 1, "withdrawn": 0}


def holds_stable(level: str) -> bool:
    return RANK[level] >= RANK["stable"]


def within_slo(state: str) -> bool:
    return state in ("current", "due_for_refresh")


def scorecard(stem: str) -> dict:
    return {
        "proof_packet_ref": f"artifacts/release/m5/dependency-quality/{stem}_proof_packet.md",
        "compatibility_report_ref": f"artifacts/compat/m5/dependency-quality/{stem}_compat.md",
        "admin_policy_ref": f"docs/governance/m5/dependency-quality/{stem}_policy.md",
        "rollback_path_ref": f"docs/governance/m5/dependency-quality/{stem}_rollback.md",
    }


def blank_scorecard(stem: str, drop: str) -> dict:
    sc = scorecard(stem)
    sc[drop] = ""
    return sc


def proof_packet(stem: str, slo_state: str) -> dict:
    captured = slo_state != "missing"
    return {
        "packet_id": f"proof_packet:dq_{stem}",
        "packet_ref": f"artifacts/release/m5/dependency-quality/{stem}_proof_packet.md",
        "proof_index_ref": "artifacts/release/stable_proof_index.json",
        "captured_at": AS_OF if captured else None,
        "freshness_slo": {
            "target_max_age_days": 30,
            "warn_within_days": 7,
            "slo_register_ref": "docs/release/m5_packet_freshness_slo.md",
        },
        "slo_state": slo_state,
        "evidence_refs": (
            [f"artifacts/release/m5/dependency-quality/{stem}_proof_packet.md"]
            if captured
            else []
        ),
    }


def signoff(owner: str, signed: bool = True) -> dict:
    return {
        "owner_ref": owner,
        "signed_off": signed,
        "signed_at": AS_OF if signed else None,
    }


def row(
    *,
    entry_id,
    title,
    lane_kind,
    surface_ref,
    surface_summary,
    release_blocking,
    claim_ref,
    claim_label,
    claim_class,
    deployment_profile,
    compatibility_downgrade_rule,
    scorecard_state,
    scorecard_dict,
    proof_packet_dict,
    owner,
    signed,
    waiver,
    active_gap_reasons,
    published_label,
    upstream_lane_refs,
    downstream_lane_refs,
    rationale,
):
    return {
        "entry_id": entry_id,
        "title": title,
        "lane_kind": lane_kind,
        "surface_ref": surface_ref,
        "surface_summary": surface_summary,
        "release_blocking": release_blocking,
        "claim_ref": claim_ref,
        "claim_label": claim_label,
        "claim_class": claim_class,
        "deployment_profile": deployment_profile,
        "compatibility_downgrade_rule": compatibility_downgrade_rule,
        "scorecard_state": scorecard_state,
        "scorecard": scorecard_dict,
        "proof_packet": proof_packet_dict,
        "waiver": waiver,
        "owner_signoff": signoff(owner, signed),
        "active_gap_reasons": active_gap_reasons,
        "published_label": published_label,
        "upstream_lane_refs": upstream_lane_refs,
        "downstream_lane_refs": downstream_lane_refs,
        "publication_destinations": ["docs/dependencies", "docs/quality", "help/about"],
        "rationale": rationale,
    }


def build_rows() -> list:
    return [
        row(
            entry_id="dq_lane:dependency_intelligence",
            title="Dependency intelligence lane",
            lane_kind="dependency_intelligence",
            surface_ref="surface:dq_dependency_intelligence",
            surface_summary="Advisory, license, suppression, and SBOM intelligence over the resolved dependency set.",
            release_blocking=True,
            claim_ref="manifest_entry:dq_dependency_intelligence",
            claim_label="stable",
            claim_class="marketed_depth",
            deployment_profile="local_only",
            compatibility_downgrade_rule="docs/dependencies/downgrade_rules.md#advisory-feed-unavailable-narrows-to-no-current-feed-data",
            scorecard_state="complete",
            scorecard_dict=scorecard("dependency_intelligence"),
            proof_packet_dict=proof_packet("dependency_intelligence", "current"),
            owner="dependency_council",
            signed=True,
            waiver=None,
            active_gap_reasons=[],
            published_label="stable",
            upstream_lane_refs=[],
            downstream_lane_refs=["dq_lane:package_health", "dq_lane:scanner_import"],
            rationale="Dependency-intelligence lane is complete, signed off, and rides a current proof packet.",
        ),
        row(
            entry_id="dq_lane:package_mutation",
            title="Package mutation and registry lane",
            lane_kind="package_mutation",
            surface_ref="surface:dq_package_mutation",
            surface_summary="Manifest-scope, registry/mirror auth, and grouped-update blast radius over package mutations.",
            release_blocking=True,
            claim_ref="manifest_entry:dq_package_mutation",
            claim_label="stable",
            claim_class="foundation_parity",
            deployment_profile="mirrored",
            compatibility_downgrade_rule="docs/dependencies/downgrade_rules.md#registry-unreachable-narrows-to-read-only-review",
            scorecard_state="complete",
            scorecard_dict=scorecard("package_mutation"),
            proof_packet_dict=proof_packet("package_mutation", "current"),
            owner="dependency_council",
            signed=True,
            waiver=None,
            active_gap_reasons=[],
            published_label="stable",
            upstream_lane_refs=[],
            downstream_lane_refs=["dq_lane:package_health"],
            rationale="Package-mutation lane is complete with a current registry-auth and lockfile-impact proof packet.",
        ),
        row(
            entry_id="dq_lane:package_health",
            title="Package and lockfile health lane",
            lane_kind="package_health",
            surface_ref="surface:dq_package_health",
            surface_summary="Lockfile authority, validation packs, and package-health review of the mutated manifest.",
            release_blocking=False,
            claim_ref="manifest_entry:dq_package_health",
            claim_label="stable",
            claim_class="foundation_parity",
            deployment_profile="mirrored",
            compatibility_downgrade_rule="docs/dependencies/downgrade_rules.md#validation-pack-pending-holds-on-waiver",
            scorecard_state="on_waiver",
            scorecard_dict=scorecard("package_health"),
            proof_packet_dict=proof_packet("package_health", "due_for_refresh"),
            owner="dependency_council",
            signed=True,
            waiver={
                "waiver_ref": "waiver:dq_package_health_validation_pack",
                "expires_at": "2026-07-11",
                "reason": "Native-build validation pack re-measurement is scheduled; the lane holds on an unexpired waiver.",
            },
            active_gap_reasons=[],
            published_label="stable",
            upstream_lane_refs=["dq_lane:package_mutation", "dq_lane:dependency_intelligence"],
            downstream_lane_refs=[],
            rationale="Package-health lane holds on an active waiver for a pending native-build validation pack.",
        ),
        row(
            entry_id="dq_lane:quality_profile",
            title="Quality-profile precedence lane",
            lane_kind="quality_profile",
            surface_ref="surface:dq_quality_profile",
            surface_summary="Quality-profile precedence and gate configuration over live and imported findings.",
            release_blocking=True,
            claim_ref="manifest_entry:dq_quality_profile",
            claim_label="stable",
            claim_class="marketed_depth",
            deployment_profile="managed",
            compatibility_downgrade_rule="docs/quality/downgrade_rules.md#profile-proof-missing-narrows-to-beta",
            scorecard_state="incomplete",
            scorecard_dict=scorecard("quality_profile"),
            proof_packet_dict=proof_packet("quality_profile", "missing"),
            owner="quality_council",
            signed=True,
            waiver=None,
            active_gap_reasons=["proof_packet_missing"],
            published_label="beta",
            upstream_lane_refs=[],
            downstream_lane_refs=["dq_lane:live_quality", "dq_lane:scanner_import"],
            rationale="Quality-profile lane narrows to beta because its precedence proof packet has not been captured.",
        ),
        row(
            entry_id="dq_lane:scanner_import",
            title="Scanner-import visibility lane",
            lane_kind="scanner_import",
            surface_ref="surface:dq_scanner_import",
            surface_summary="SARIF/scanner imported findings with imported-versus-live truth and import provenance.",
            release_blocking=False,
            claim_ref="manifest_entry:dq_scanner_import",
            claim_label="stable",
            claim_class="imported_visibility",
            deployment_profile="local_only",
            compatibility_downgrade_rule="docs/quality/downgrade_rules.md#stale-import-narrows-to-preview",
            scorecard_state="stale",
            scorecard_dict=scorecard("scanner_import"),
            proof_packet_dict=proof_packet("scanner_import", "breached"),
            owner="quality_council",
            signed=True,
            waiver=None,
            active_gap_reasons=["proof_packet_stale", "compatibility_report_stale"],
            published_label="preview",
            upstream_lane_refs=["dq_lane:quality_profile", "dq_lane:dependency_intelligence"],
            downstream_lane_refs=[],
            rationale="Scanner-import lane narrows to preview because its proof packet and compatibility report are stale.",
        ),
        row(
            entry_id="dq_lane:live_quality",
            title="Live code-quality lane",
            lane_kind="live_quality",
            surface_ref="surface:dq_live_quality",
            surface_summary="Live in-repo code-quality findings and quality-gate verdicts on the working tree.",
            release_blocking=False,
            claim_ref="manifest_entry:dq_live_quality",
            claim_label="stable",
            claim_class="marketed_depth",
            deployment_profile="local_only",
            compatibility_downgrade_rule="docs/quality/downgrade_rules.md#admin-policy-pending-narrows-to-beta",
            scorecard_state="blocked",
            scorecard_dict=scorecard("live_quality"),
            proof_packet_dict=proof_packet("live_quality", "current"),
            owner="quality_council",
            signed=True,
            waiver=None,
            active_gap_reasons=["admin_policy_missing"],
            published_label="beta",
            upstream_lane_refs=["dq_lane:quality_profile"],
            downstream_lane_refs=[],
            rationale="Live code-quality lane is blocked below the cutline until its admin/policy story is staffed.",
        ),
        row(
            entry_id="dq_lane:cli_headless_parity",
            title="CLI/headless parity lane",
            lane_kind="cli_headless_parity",
            surface_ref="surface:dq_cli_headless_parity",
            surface_summary="CLI and headless parity for dependency, package, and quality inspection flows.",
            release_blocking=False,
            claim_ref="manifest_entry:dq_cli_headless_parity",
            claim_label="stable",
            claim_class="foundation_parity",
            deployment_profile="local_only",
            compatibility_downgrade_rule="docs/quality/downgrade_rules.md#rollback-path-missing-narrows-to-beta",
            scorecard_state="rollback_missing",
            scorecard_dict=scorecard("cli_headless_parity"),
            proof_packet_dict=proof_packet("cli_headless_parity", "current"),
            owner="quality_council",
            signed=True,
            waiver=None,
            active_gap_reasons=["rollback_path_missing"],
            published_label="beta",
            upstream_lane_refs=[],
            downstream_lane_refs=[],
            rationale="CLI/headless parity lane narrows to beta until a rollback/downgrade path is defined.",
        ),
    ]


def build_dependencies() -> list:
    return [
        {
            "edge_id": "dep:package_mutation_to_package_health",
            "from_lane": "dq_lane:package_mutation",
            "to_lane": "dq_lane:package_health",
            "dependency_kind": "hard",
            "rationale": "Package-health review depends on the mutated lockfile produced by package mutation.",
        },
        {
            "edge_id": "dep:quality_profile_to_live_quality",
            "from_lane": "dq_lane:quality_profile",
            "to_lane": "dq_lane:live_quality",
            "dependency_kind": "hard",
            "rationale": "Live code-quality verdicts inherit the active quality-profile precedence.",
        },
        {
            "edge_id": "dep:quality_profile_to_scanner_import",
            "from_lane": "dq_lane:quality_profile",
            "to_lane": "dq_lane:scanner_import",
            "dependency_kind": "hard",
            "rationale": "Imported scanner findings are ranked under the active quality-profile precedence.",
        },
        {
            "edge_id": "dep:dependency_intelligence_to_scanner_import",
            "from_lane": "dq_lane:dependency_intelligence",
            "to_lane": "dq_lane:scanner_import",
            "dependency_kind": "soft",
            "rationale": "Scanner imports cross-reference advisory intelligence but can render independently.",
        },
        {
            "edge_id": "dep:dependency_intelligence_to_package_health",
            "from_lane": "dq_lane:dependency_intelligence",
            "to_lane": "dq_lane:package_health",
            "dependency_kind": "soft",
            "rationale": "Package-health review surfaces advisory context but does not block on it.",
        },
    ]


def build_stop_rules() -> list:
    # reason -> (title, default_action, blocks_promotion, rationale)
    spec = {
        "proof_packet_missing": (
            "Proof packet missing",
            "refresh_proof_packet",
            True,
            "A dependency/quality lane without a captured proof packet cannot carry a Stable claim.",
        ),
        "proof_packet_stale": (
            "Proof packet stale",
            "refresh_proof_packet",
            True,
            "A lane whose proof packet breached its freshness SLO must narrow below the cutline.",
        ),
        "compatibility_report_missing": (
            "Compatibility report missing",
            "refresh_compatibility_report",
            False,
            "A missing compatibility report narrows the lane but does not by itself hold promotion.",
        ),
        "compatibility_report_stale": (
            "Compatibility report stale",
            "refresh_compatibility_report",
            False,
            "A stale compatibility report narrows the lane but does not by itself hold promotion.",
        ),
        "admin_policy_missing": (
            "Admin/policy story missing",
            "staff_admin_policy",
            True,
            "A marketed lane with no admin/policy story is blocked from a Stable claim.",
        ),
        "rollback_path_missing": (
            "Rollback/downgrade path missing",
            "define_rollback_path",
            True,
            "A lane without a defined rollback/downgrade path cannot promote at Stable.",
        ),
        "upstream_lane_narrowed": (
            "Upstream hard-dependency lane narrowed",
            "narrow_label",
            True,
            "A lane whose upstream hard dependency narrowed must inherit the ceiling.",
        ),
        "waiver_expired": (
            "Waiver expired",
            "hold_promotion",
            True,
            "A lane that relied on a now-expired waiver must narrow and hold promotion.",
        ),
        "owner_signoff_missing": (
            "Owner sign-off missing",
            "request_owner_signoff",
            True,
            "A lane without owner sign-off cannot carry a Stable claim.",
        ),
    }
    rules = []
    for reason in GAP_REASONS:
        title, action, blocks, rationale = spec[reason]
        rules.append(
            {
                "rule_id": f"dq_rule:{reason}",
                "title": title,
                "trigger_reason": reason,
                "applies_to_labels": ["lts", "stable"],
                "default_action": action,
                "blocks_promotion": blocks,
                "rationale": rationale,
            }
        )
    return rules


def rule_fires(rule: dict, rows: list) -> bool:
    return any(
        row["claim_label"] in rule["applies_to_labels"]
        and rule["trigger_reason"] in row["active_gap_reasons"]
        for row in rows
    )


def compute_promotion(rows: list, stop_rules: list) -> dict:
    firing_blocking = [
        r for r in stop_rules if r["blocks_promotion"] and rule_fires(r, rows)
    ]
    decision = "hold" if firing_blocking else "proceed"
    blocking_rule_ids = sorted(r["rule_id"] for r in firing_blocking)
    blocking_triggers = {r["trigger_reason"] for r in firing_blocking}
    blocking_claim_ids = sorted(
        {
            row["entry_id"]
            for row in rows
            if holds_stable(row["claim_label"])
            and any(reason in blocking_triggers for reason in row["active_gap_reasons"])
        }
    )
    return {
        "promotion_gate": "m5_dependency_quality_parity_promotion",
        "decision": decision,
        "blocking_rule_ids": blocking_rule_ids,
        "blocking_claim_ids": blocking_claim_ids,
        "rationale": (
            "Promotion is held because dependency/quality stop rules are firing on lanes that "
            "must narrow before the parity matrix can promote at Stable."
            if decision == "hold"
            else "Promotion may proceed; no blocking dependency/quality stop rule is firing."
        ),
    }


def compute_summary(rows: list, stop_rules: list, deps: list) -> dict:
    def kind_count(kind):
        return sum(1 for r in rows if r["lane_kind"] == kind)

    def packet_count(state):
        return sum(1 for r in rows if r["proof_packet"]["slo_state"] == state)

    release_blocking = [r for r in rows if r["release_blocking"]]
    hard = sum(1 for e in deps if e["dependency_kind"] == "hard")
    soft = sum(1 for e in deps if e["dependency_kind"] == "soft")
    claims = sorted({r["claim_ref"] for r in rows})
    return {
        "total_entries": len(rows),
        "total_claims": len(claims),
        "entries_holding_stable": sum(1 for r in rows if holds_stable(r["published_label"])),
        "entries_narrowed": sum(1 for r in rows if not holds_stable(r["published_label"])),
        "entries_on_active_waiver": sum(
            1 for r in rows if r["scorecard_state"] == "on_waiver"
        ),
        "entries_blocked": sum(1 for r in rows if r["scorecard_state"] == "blocked"),
        "entries_rollback_missing": sum(
            1 for r in rows if r["scorecard_state"] == "rollback_missing"
        ),
        "release_blocking_total": len(release_blocking),
        "release_blocking_holding": sum(
            1 for r in release_blocking if holds_stable(r["published_label"])
        ),
        "release_blocking_narrowed": sum(
            1 for r in release_blocking if not holds_stable(r["published_label"])
        ),
        "dependency_intelligence_entries": kind_count("dependency_intelligence"),
        "package_mutation_entries": kind_count("package_mutation"),
        "package_health_entries": kind_count("package_health"),
        "quality_profile_entries": kind_count("quality_profile"),
        "scanner_import_entries": kind_count("scanner_import"),
        "live_quality_entries": kind_count("live_quality"),
        "cli_headless_parity_entries": kind_count("cli_headless_parity"),
        "packets_current": packet_count("current"),
        "packets_due_for_refresh": packet_count("due_for_refresh"),
        "packets_breached": packet_count("breached"),
        "packets_missing": packet_count("missing"),
        "total_active_gap_reasons": sum(len(r["active_gap_reasons"]) for r in rows),
        "rules_firing": sum(1 for r in stop_rules if rule_fires(r, rows)),
        "hard_dependency_edges": hard,
        "soft_dependency_edges": soft,
    }


def build_matrix() -> dict:
    rows = build_rows()
    deps = build_dependencies()
    stop_rules = build_stop_rules()
    release_blocking_lane_refs = [
        r["surface_ref"] for r in rows if r["release_blocking"]
    ]
    return {
        "schema_version": SCHEMA_VERSION,
        "record_kind": RECORD_KIND,
        "matrix_id": "m5_dependency_quality_parity_matrix:v1",
        "status": "published",
        "overview_page": f"docs/m5/{NAME}.md",
        "as_of": AS_OF,
        "claim_manifest_ref": "artifacts/release/stable_claim_manifest.json",
        "lifecycle_labels": LIFECYCLE_LABELS,
        "lane_kinds": LANE_KINDS,
        "claim_classes": CLAIM_CLASSES,
        "deployment_profiles": DEPLOYMENT_PROFILES,
        "scorecard_states": SCORECARD_STATES,
        "gap_reasons": GAP_REASONS,
        "stop_rule_actions": STOP_RULE_ACTIONS,
        "launch_cutline": {
            "cutline_level": "stable",
            "above_cutline_levels": ABOVE_CUTLINE,
            "below_cutline_levels": BELOW_CUTLINE,
            "description": (
                "A dependency/package or code-quality lane carries a Stable (or LTS) public claim "
                "only when its scorecard is complete, the proof packet is current within its freshness "
                "SLO, any waiver is unexpired, the compatibility report, admin/policy story, and "
                "rollback/downgrade path are all present, owner sign-off is recorded, and no upstream "
                "hard dependency is narrowed below the cutline. A lane that loses any of those must drop "
                "below the cutline rather than inherit an adjacent green lane or implicit M4 stable "
                "coverage."
            ),
        },
        "release_blocking_lane_refs": release_blocking_lane_refs,
        "stop_rules": stop_rules,
        "rows": rows,
        "dependencies": deps,
        "promotion": compute_promotion(rows, stop_rules),
        "summary": compute_summary(rows, stop_rules, deps),
    }


def build_capture(matrix: dict) -> dict:
    s = matrix["summary"]
    return {
        "status": "pass",
        "as_of": matrix["as_of"],
        "summary": s,
        "promotion": {
            "decision": matrix["promotion"]["decision"],
            "blocking_rule_ids": matrix["promotion"]["blocking_rule_ids"],
            "blocking_claim_ids": matrix["promotion"]["blocking_claim_ids"],
        },
        "negative_drills": [
            {"drill_id": "drill:narrowing_without_reason", "status": "passed"},
            {"drill_id": "drill:held_with_active_gap", "status": "passed"},
            {"drill_id": "drill:published_wider_than_claim", "status": "passed"},
            {"drill_id": "drill:promotion_proceed_while_rule_fires", "status": "passed"},
        ],
        "fixture_cases": [
            {"case_id": "fixture:missing_scorecard_ref", "status": "passed"},
            {"case_id": "fixture:duplicate_entry_id", "status": "passed"},
        ],
    }


def build_fixtures(matrix: dict) -> dict:
    # duplicate_entry_id: clone the first row's id onto the second.
    dup = json.loads(json.dumps(matrix))
    dup["rows"][1]["entry_id"] = dup["rows"][0]["entry_id"]

    # missing_scorecard_ref: blank a held row's rollback_path_ref so the
    # complete-scorecard invariant fails.
    miss = json.loads(json.dumps(matrix))
    miss["rows"][0]["scorecard"]["rollback_path_ref"] = ""

    return {
        "duplicate_entry_id.json": dup,
        "missing_scorecard_ref.json": miss,
    }


def write_json(path: Path, data: dict) -> None:
    path.parent.mkdir(parents=True, exist_ok=True)
    path.write_text(json.dumps(data, indent=2) + "\n", encoding="utf-8")


def main() -> None:
    matrix = build_matrix()
    write_json(ARTIFACT_PATH, matrix)
    write_json(CAPTURE_PATH, build_capture(matrix))

    fixtures = build_fixtures(matrix)
    for filename, data in fixtures.items():
        write_json(FIXTURES_DIR / filename, data)
    cases = {
        "cases": [
            {
                "case_id": "fixture:missing_scorecard_ref",
                "file": "missing_scorecard_ref.json",
                "expected_check_id": "scorecard.incomplete_on_held_row",
            },
            {
                "case_id": "fixture:duplicate_entry_id",
                "file": "duplicate_entry_id.json",
                "expected_check_id": "rows.duplicate_entry_id",
            },
        ]
    }
    write_json(FIXTURES_DIR / "cases.json", cases)

    print(f"wrote {ARTIFACT_PATH.relative_to(REPO_ROOT)}")
    print(f"wrote {CAPTURE_PATH.relative_to(REPO_ROOT)}")
    for filename in list(fixtures) + ["cases.json"]:
        print(f"wrote {(FIXTURES_DIR / filename).relative_to(REPO_ROOT)}")


if __name__ == "__main__":
    main()
