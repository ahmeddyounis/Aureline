#!/usr/bin/env python3
"""Regenerate the canonical M5 efficiency-state governance matrix and fixtures.

The matrix's derived fields (per-dimension findings, fired narrowing reasons,
narrowed effective posture, certification state, promotion blocker, release
binding, and the promotion gate) are recomputed with the *same* engine the CI
gate (`ci/check_m5_efficiency_governance.py`) uses, so the checked-in matrix can
never disagree with the validator. Run after editing the row inputs below:

    python3 tools/regenerate_m5_efficiency_governance.py

then re-run the gate:

    python3 ci/check_m5_efficiency_governance.py --repo-root .
"""

from __future__ import annotations

import json
import sys
from pathlib import Path

REPO_ROOT = Path(__file__).resolve().parents[1]
sys.path.insert(0, str(REPO_ROOT / "ci"))

import check_m5_efficiency_governance as gate  # noqa: E402

AS_OF = "2026-06-20"
GENERATED_AT = "2026-06-20T14:00:00Z"

MATRIX_REL = "artifacts/efficiency/m5-efficiency-governance.json"
FIXTURE_DIR_REL = "fixtures/efficiency/m5-efficiency-governance"

EVIDENCE_INDEX_REF = (
    "artifacts/release/m5/"
    "certify_the_full_m5_train_narrow_stale_rows_and_publish_the_canonical_evidence_index.json"
)

CLAIM_LEVELS = [
    {
        "level": "undeclared_badge",
        "rank": 0,
        "claim_bearing": False,
        "label": "Undeclared badge",
        "description": "A low-power badge with no materialized efficiency-state evidence or declared behaviour change. Asserts no claim and is retained only for diagnosis.",
    },
    {
        "level": "state_declared",
        "rank": 1,
        "claim_bearing": False,
        "label": "State declared",
        "description": "The efficiency state and source-of-change are materialized, but hidden-work suppression or protected-path preservation is not yet qualified. Not a publishable low-power claim.",
    },
    {
        "level": "qualified_low_power",
        "rank": 2,
        "claim_bearing": True,
        "label": "Qualified low-power",
        "description": "A claim-bearing posture: the surface materializes its state, names shed work, suppresses hidden-pane render and polling, and preserves protected paths under pressure.",
    },
    {
        "level": "certified_low_power",
        "rank": 3,
        "claim_bearing": True,
        "label": "Certified low-power",
        "description": "The highest claim-bearing posture: qualified low-power plus policy-aware override, staged recovery, and propagation to every required release, support, docs, and help surface.",
    },
]

PILLARS = [
    {
        "pillar": "efficiency_state_evidence",
        "proves": "The surface materializes an inspectable efficiency-state record with a named state and source-of-change.",
    },
    {
        "pillar": "behavior_declaration",
        "proves": "The low-power state declares a concrete behaviour change rather than remaining a vague badge.",
    },
    {
        "pillar": "hidden_work_suppression",
        "proves": "Hidden, occluded, or off-screen panes commit no render work and pause nonessential polling and animation.",
    },
    {
        "pillar": "protected_path_preservation",
        "proves": "Active tasks, debug correctness, local save, navigation, and review authority stay protected under pressure.",
    },
    {
        "pillar": "override_policy_awareness",
        "proves": "A user-overridable posture exposes an explicit, policy-aware override reference.",
    },
    {
        "pillar": "recovery_staging",
        "proves": "When pressure clears, deferred work resumes in staged order rather than thrashing back at once.",
    },
    {
        "pillar": "consumer_propagation",
        "proves": "The posture reaches every required publication surface so later low-power copy derives from one source of truth.",
    },
]

NARROWING_REASONS = [
    {
        "reason": "missing_efficiency_state_evidence",
        "pillar": "efficiency_state_evidence",
        "detects": "The row has no materialized efficiency-state evidence, state, or source-of-change.",
        "narrows_to": "undeclared_badge",
        "auto_detectable": True,
        "stop_rule": "Quarantine the row; it may not assert any low-power claim without efficiency-state evidence.",
    },
    {
        "reason": "vague_low_power_badge",
        "pillar": "behavior_declaration",
        "detects": "The row shows a low-power state with no declared behaviour change.",
        "narrows_to": "undeclared_badge",
        "auto_detectable": True,
        "stop_rule": "Quarantine the row; a 'battery saver' or 'thermal mode' badge must declare behaviour changes.",
    },
    {
        "reason": "unqualified_hidden_work_suppression",
        "pillar": "hidden_work_suppression",
        "detects": "The row binds hidden or off-screen panes but cannot prove qualified render/poll suppression.",
        "narrows_to": "state_declared",
        "auto_detectable": True,
        "stop_rule": "Narrow below a publishable low-power claim until hidden-pane suppression is qualified.",
    },
    {
        "reason": "protected_path_regression_under_pressure",
        "pillar": "protected_path_preservation",
        "detects": "A protected interaction regressed under battery or thermal pressure.",
        "narrows_to": "state_declared",
        "auto_detectable": True,
        "stop_rule": "Narrow below a publishable low-power claim until protected paths are preserved under pressure.",
    },
    {
        "reason": "override_not_policy_aware",
        "pillar": "override_policy_awareness",
        "detects": "A user-overridable posture lacks an explicit, policy-aware override reference.",
        "narrows_to": "qualified_low_power",
        "auto_detectable": True,
        "stop_rule": "Narrow off the certified posture until the override is explicit and policy-aware.",
    },
    {
        "reason": "recovery_not_staged",
        "pillar": "recovery_staging",
        "detects": "Recovery applies but deferred work is not staged.",
        "narrows_to": "qualified_low_power",
        "auto_detectable": True,
        "stop_rule": "Narrow off the certified posture until recovery resumes work in stages.",
    },
    {
        "reason": "missing_consumer_propagation",
        "pillar": "consumer_propagation",
        "detects": "The posture does not reach every required release, support, docs, or help surface.",
        "narrows_to": "qualified_low_power",
        "auto_detectable": True,
        "stop_rule": "Narrow off the certified posture until the posture propagates to every required surface.",
    },
]

CERTIFICATION_STATES = [
    {
        "state": "certified",
        "is_certified": True,
        "blocks_when_claim_bearing": False,
        "description": "Every dimension is clean and the effective posture equals the published ceiling.",
    },
    {
        "state": "narrowed",
        "is_certified": False,
        "blocks_when_claim_bearing": True,
        "description": "At least one narrowing reason fired; the effective posture is below the published ceiling. A claim-bearing narrowed row holds promotion.",
    },
    {
        "state": "quarantined",
        "is_certified": False,
        "blocks_when_claim_bearing": True,
        "description": "The row narrowed to the undeclared-badge floor. It asserts no low-power claim and is retained only for diagnosis.",
    },
]

M5_SURFACES = [
    {"surface": "notebooks", "description": "Notebook cell and output panes."},
    {"surface": "previews", "description": "Preview and embedded browser-runtime panes."},
    {"surface": "docs_browser_panes", "description": "Docs and embedded browser panes."},
    {"surface": "traces", "description": "Trace, profiler, and timeline panes."},
    {"surface": "pipelines", "description": "Pipeline, task, and run panes."},
    {"surface": "remote_sessions", "description": "Remote-session and reconnect panes."},
    {"surface": "support_exports", "description": "Support-export and diagnostics panes."},
    {"surface": "companion_adjacent", "description": "Companion-adjacent assistance views."},
]

REQUIRED_PUBLICATION_SURFACES = ["docs", "help", "support_export", "release"]

CONSUMER_BINDINGS = [
    {
        "consumer": "release_promotion",
        "source_projection": "promotion_gate",
        "ingests": [
            "certification states",
            "fired narrowing reasons",
            "effective postures",
            "the promotion verdict",
        ],
    },
    {
        "consumer": "release_packet",
        "source_projection": "release_binding",
        "ingests": [
            "each row's declared certification state",
            "each row's declared effective posture",
        ],
    },
    {
        "consumer": "support_export",
        "source_projection": "redaction_safe_projection",
        "ingests": [
            "states",
            "fired reasons",
            "labels",
            "bound refs only — never raw logs or provider payloads",
        ],
    },
    {
        "consumer": "docs_help",
        "source_projection": "low_power_vocabulary_projection",
        "ingests": [
            "the closed efficiency vocabulary",
            "each surface's effective posture",
            "each surface's certification label",
        ],
    },
]

SOURCE_REFS = [
    gate.MATRIX_SCHEMA_REL,
    gate.FIXTURE_SCHEMA_REL,
    "crates/aureline-shell/src/efficiency/mod.rs",
    "crates/aureline-shell/src/efficiency/governance/mod.rs",
    EVIDENCE_INDEX_REF,
]

INSPECTION = {
    "how_to_recompute": (
        "For each row, fire every narrowing reason whose condition holds over its inline evidence, "
        "set the effective posture to the lowest-ranked of the published ceiling and each fired reason's "
        "target, derive the certification state, then hold promotion when a claim-bearing row narrows below "
        "the posture it asserts."
    ),
    "promotion_gate": (
        "Promotion holds when any claim-bearing row's effective posture is below the posture it asserts. "
        "The current matrix resolves to proceed: no claim-bearing row is narrowed below its posture."
    ),
    "vocabulary_binding": (
        "The closed vocabularies mirror the shell efficiency runtime tokens and are bound in "
        "crates/aureline-shell/src/efficiency/governance/ so the matrix can never drift from what ships."
    ),
    "evidence_index_ref": EVIDENCE_INDEX_REF,
}


def certified_evidence(surface: str, *, propagated: list[str]) -> dict:
    return {
        "efficiency_state_evidence_present": True,
        "declares_behavior_change": True,
        "binds_hidden_panes": True,
        "hidden_work_suppression_qualified": True,
        "hidden_pane_render_violation_count": 0,
        "protected_paths_preserved": True,
        "override_policy_aware": True,
        "override_policy_ref": f"id:policy:efficiency:override:{surface}",
        "recovery_required": False,
        "recovery_staged": False,
        "propagated_surfaces": list(propagated),
    }


# Row inputs. Derived fields are computed by the engine below.
ROW_INPUTS = [
    {
        "row_id": "eff.notebooks.thermal",
        "m5_surface": "notebooks",
        "title": "Notebook panes under thermal pressure",
        "efficiency_state": "ThermalConstrained",
        "source_of_change": ["thermal_pressure"],
        "throttled_subsystems": ["ai_warmup", "speculative_prefetch", "indexing_refresh"],
        "hidden_pane_bindings": [
            {
                "visibility_state": "hidden_tab",
                "behaviors": ["render_suppressed", "animation_suppressed", "fully_quiescent"],
            }
        ],
        "override_posture": "user_override_session_only",
        "recovery_state": "not_in_recovery",
        "posture": "certified_low_power",
        "published_claim_ceiling": "certified_low_power",
        "required_publication_surfaces": REQUIRED_PUBLICATION_SURFACES,
        "evidence": certified_evidence("notebooks", propagated=REQUIRED_PUBLICATION_SURFACES),
        "evidence_refs": [
            "id:evidence:efficiency:notebooks:state",
            "id:evidence:efficiency:notebooks:hidden_pane_audit",
        ],
        "release_packet_ref": "id:release:efficiency:notebooks",
    },
    {
        "row_id": "eff.previews.battery_saver",
        "m5_surface": "previews",
        "title": "Preview and browser-runtime panes under battery saver",
        "efficiency_state": "EfficiencyAware",
        "source_of_change": ["os_battery_saver", "battery"],
        "throttled_subsystems": ["preview_refresh", "non_essential_animation"],
        "hidden_pane_bindings": [
            {
                "visibility_state": "detached_offscreen",
                "behaviors": ["render_suppressed", "polling_paused", "fully_quiescent"],
            }
        ],
        "override_posture": "user_override_session_only",
        "recovery_state": "not_in_recovery",
        "posture": "certified_low_power",
        "published_claim_ceiling": "certified_low_power",
        "required_publication_surfaces": REQUIRED_PUBLICATION_SURFACES,
        "evidence": certified_evidence("previews", propagated=REQUIRED_PUBLICATION_SURFACES),
        "evidence_refs": [
            "id:evidence:efficiency:previews:state",
            "id:evidence:efficiency:previews:hidden_pane_audit",
        ],
        "release_packet_ref": "id:release:efficiency:previews",
    },
    {
        "row_id": "eff.docs_browser.critical_battery",
        "m5_surface": "docs_browser_panes",
        "title": "Docs and embedded browser panes under critical battery",
        "efficiency_state": "ProtectCore",
        "source_of_change": ["critical_battery"],
        "throttled_subsystems": ["preview_refresh", "extension_polling"],
        "hidden_pane_bindings": [
            {
                "visibility_state": "occluded_window",
                "behaviors": ["render_suppressed", "animation_suppressed", "correctness_poll_only"],
            }
        ],
        "override_posture": "policy_blocked",
        "recovery_state": "not_in_recovery",
        "posture": "qualified_low_power",
        "published_claim_ceiling": "qualified_low_power",
        "required_publication_surfaces": ["docs", "help", "support_export"],
        "evidence": {
            **certified_evidence("docs_browser_panes", propagated=["docs", "help", "support_export"]),
            "override_policy_aware": False,
            "override_policy_ref": None,
        },
        "evidence_refs": [
            "id:evidence:efficiency:docs_browser:state",
            "id:evidence:efficiency:docs_browser:hidden_pane_audit",
        ],
        "release_packet_ref": "id:release:efficiency:docs_browser",
    },
    {
        "row_id": "eff.traces.thermal",
        "m5_surface": "traces",
        "title": "Trace and profiler panes under thermal pressure",
        "efficiency_state": "ThermalConstrained",
        "source_of_change": ["thermal_pressure", "frame_miss_pressure"],
        "throttled_subsystems": ["graph_enrichment", "speculative_prefetch"],
        "hidden_pane_bindings": [
            {
                "visibility_state": "collapsed_split",
                "behaviors": ["render_suppressed", "polling_paused"],
            }
        ],
        "override_posture": "user_override_session_only",
        "recovery_state": "not_in_recovery",
        "posture": "qualified_low_power",
        "published_claim_ceiling": "qualified_low_power",
        "required_publication_surfaces": ["docs", "help", "support_export"],
        "evidence": certified_evidence("traces", propagated=["docs", "help", "support_export"]),
        "evidence_refs": [
            "id:evidence:efficiency:traces:state",
            "id:evidence:efficiency:traces:hidden_pane_audit",
        ],
        "release_packet_ref": "id:release:efficiency:traces",
    },
    {
        "row_id": "eff.pipelines.low_battery",
        "m5_surface": "pipelines",
        "title": "Pipeline and task panes under low battery",
        "efficiency_state": "EfficiencyAware",
        "source_of_change": ["low_battery", "user_low_power_mode"],
        "throttled_subsystems": ["upload_transfer", "indexing_refresh"],
        "hidden_pane_bindings": [
            {
                "visibility_state": "hidden_tab",
                "behaviors": ["render_suppressed", "fully_quiescent"],
            }
        ],
        "override_posture": "admin_controlled",
        "recovery_state": "not_in_recovery",
        "posture": "certified_low_power",
        "published_claim_ceiling": "certified_low_power",
        "required_publication_surfaces": REQUIRED_PUBLICATION_SURFACES,
        "evidence": certified_evidence("pipelines", propagated=REQUIRED_PUBLICATION_SURFACES),
        "evidence_refs": [
            "id:evidence:efficiency:pipelines:state",
            "id:evidence:efficiency:pipelines:hidden_pane_audit",
        ],
        "release_packet_ref": "id:release:efficiency:pipelines",
    },
    {
        "row_id": "eff.remote_sessions.protect_core",
        "m5_surface": "remote_sessions",
        "title": "Remote-session panes under sustained pressure",
        "efficiency_state": "ProtectCore",
        "source_of_change": ["thermal_pressure", "policy_cap"],
        "throttled_subsystems": ["remote_session_helper", "upload_transfer"],
        "hidden_pane_bindings": [
            {
                "visibility_state": "detached_offscreen",
                "behaviors": ["render_suppressed", "correctness_poll_only"],
            }
        ],
        "override_posture": "policy_blocked",
        "recovery_state": "not_in_recovery",
        "posture": "qualified_low_power",
        "published_claim_ceiling": "qualified_low_power",
        "required_publication_surfaces": ["docs", "help", "support_export"],
        "evidence": {
            **certified_evidence("remote_sessions", propagated=["docs", "help", "support_export"]),
            "override_policy_aware": False,
            "override_policy_ref": None,
        },
        "evidence_refs": [
            "id:evidence:efficiency:remote_sessions:state",
            "id:evidence:efficiency:remote_sessions:hidden_pane_audit",
        ],
        "release_packet_ref": "id:release:efficiency:remote_sessions",
    },
    {
        "row_id": "eff.support_exports.recovery",
        "m5_surface": "support_exports",
        "title": "Support-export panes resuming after pressure clears",
        "efficiency_state": "Recovery",
        "source_of_change": ["pressure_cleared"],
        "throttled_subsystems": ["indexing_refresh", "graph_enrichment"],
        "hidden_pane_bindings": [
            {
                "visibility_state": "hidden_tab",
                "behaviors": ["render_suppressed", "fully_quiescent"],
            }
        ],
        "override_posture": "user_override_session_only",
        "recovery_state": "staged_resume",
        "posture": "certified_low_power",
        "published_claim_ceiling": "certified_low_power",
        "required_publication_surfaces": REQUIRED_PUBLICATION_SURFACES,
        "evidence": {
            **certified_evidence("support_exports", propagated=REQUIRED_PUBLICATION_SURFACES),
            "recovery_required": True,
            "recovery_staged": True,
        },
        "evidence_refs": [
            "id:evidence:efficiency:support_exports:state",
            "id:evidence:efficiency:support_exports:recovery",
        ],
        "release_packet_ref": "id:release:efficiency:support_exports",
    },
    {
        "row_id": "eff.companion_adjacent.badge",
        "m5_surface": "companion_adjacent",
        "title": "Companion-adjacent low-power badge (retained for diagnosis)",
        "efficiency_state": "EfficiencyAware",
        "source_of_change": ["os_battery_saver"],
        "throttled_subsystems": [],
        "hidden_pane_bindings": [],
        "override_posture": "policy_blocked",
        "recovery_state": "not_in_recovery",
        "posture": "undeclared_badge",
        "published_claim_ceiling": "undeclared_badge",
        "required_publication_surfaces": ["docs"],
        "evidence": {
            "efficiency_state_evidence_present": False,
            "declares_behavior_change": False,
            "binds_hidden_panes": False,
            "hidden_work_suppression_qualified": False,
            "hidden_pane_render_violation_count": 0,
            "protected_paths_preserved": True,
            "override_policy_aware": False,
            "override_policy_ref": None,
            "recovery_required": False,
            "recovery_staged": False,
            "propagated_surfaces": ["docs"],
        },
        "evidence_refs": ["id:evidence:efficiency:companion_adjacent:badge"],
        "release_packet_ref": "id:release:efficiency:companion_adjacent",
    },
]


def metadata_matrix() -> dict:
    """Minimal matrix carrying the metadata the engine needs to recompute."""
    return {"claim_levels": CLAIM_LEVELS, "narrowing_reasons": NARROWING_REASONS}


def build_dimension_findings(result: dict, evidence_refs: list[str]) -> dict:
    findings = {}
    for pillar in gate.PILLARS:
        reason = result["pillar_gaps"][pillar]
        findings[pillar] = {
            "certification_status": "gap" if reason else "certified",
            "narrowing_reason": reason,
            "bound_refs": list(evidence_refs),
        }
    return findings


def build_row(engine: "gate.GovernanceEngine", row_input: dict) -> dict:
    result = engine.recompute(row_input)
    posture_label = next(
        cl["label"] for cl in CLAIM_LEVELS if cl["level"] == result["effective"]
    )
    return {
        "row_id": row_input["row_id"],
        "subject_kind": "m5_efficiency_governance_row",
        "m5_surface": row_input["m5_surface"],
        "title": row_input["title"],
        "efficiency_state": row_input["efficiency_state"],
        "source_of_change": row_input["source_of_change"],
        "throttled_subsystems": row_input["throttled_subsystems"],
        "hidden_pane_bindings": row_input["hidden_pane_bindings"],
        "override_posture": row_input["override_posture"],
        "recovery_state": row_input["recovery_state"],
        "posture": row_input["posture"],
        "published_claim_ceiling": row_input["published_claim_ceiling"],
        "required_publication_surfaces": row_input["required_publication_surfaces"],
        "evidence": row_input["evidence"],
        "evidence_refs": row_input["evidence_refs"],
        "dimension_findings": build_dimension_findings(result, row_input["evidence_refs"]),
        "fired_narrowing_reasons": result["fired"],
        "effective_posture": result["effective"],
        "certification_state": result["state"],
        "promotion_blocker": {
            "blocks_promotion": result["blocks"],
            "blocker_reasons": result["blocker_reasons"],
            "posture_label": posture_label,
        },
        "release_binding": {
            "release_packet_ref": row_input["release_packet_ref"],
            "declared_certification_state": result["state"],
            "declared_effective_posture": result["effective"],
        },
    }


def build_matrix() -> dict:
    engine = gate.GovernanceEngine(metadata_matrix())
    rows = [build_row(engine, ri) for ri in ROW_INPUTS]

    blocking_row_ids = sorted(
        r["row_id"] for r in rows if r["promotion_blocker"]["blocks_promotion"]
    )
    blocking_reasons = sorted(
        {
            reason
            for r in rows
            for reason in r["promotion_blocker"]["blocker_reasons"]
        }
    )
    decision = "hold" if blocking_row_ids else "proceed"

    summary = {
        "total_rows": len(rows),
        "rows_certified": sum(1 for r in rows if r["certification_state"] == "certified"),
        "rows_narrowed": sum(1 for r in rows if r["certification_state"] == "narrowed"),
        "rows_quarantined": sum(1 for r in rows if r["certification_state"] == "quarantined"),
        "claim_bearing_rows": sum(
            1 for r in rows if r["posture"] in gate.CLAIM_BEARING_LEVELS
        ),
        "rows_blocking_promotion": len(blocking_row_ids),
        "covered_surfaces": sorted({r["m5_surface"] for r in rows}),
    }

    return {
        "record_kind": "efficiency_m5_governance_matrix",
        "schema_version": 1,
        "packet_id": "m5-efficiency-governance:0001",
        "generated_at": GENERATED_AT,
        "as_of": AS_OF,
        "title": "Canonical M5 efficiency-state, battery-or-thermal, and hidden-pane render-suppression matrix",
        "summary": (
            "One typed efficiency-state contract for every M5 surface that adapts under battery or thermal "
            "pressure. It freezes the closed vocabulary, binds each surface to its efficiency-state, "
            "hidden-work-suppression, protected-path, override, recovery, and propagation evidence, and "
            "narrows any surface whose evidence cannot back its low-power claim."
        ),
        "evidence_index_ref": EVIDENCE_INDEX_REF,
        "source_refs": SOURCE_REFS,
        "closed_vocabularies": gate.CANONICAL_VOCAB,
        "claim_levels": CLAIM_LEVELS,
        "pillars": PILLARS,
        "narrowing_reasons": NARROWING_REASONS,
        "certification_states": CERTIFICATION_STATES,
        "m5_surfaces": M5_SURFACES,
        "required_publication_surfaces": REQUIRED_PUBLICATION_SURFACES,
        "rows": rows,
        "promotion_gate": {
            "decision": decision,
            "blocking_row_ids": blocking_row_ids,
            "blocking_reasons": blocking_reasons,
            "rationale": (
                "Computed from the firing narrowing reasons over every row's efficiency-state evidence, "
                "hidden-work suppression, protected-path preservation, override policy-awareness, recovery "
                "staging, and consumer propagation."
            ),
        },
        "consumer_bindings": CONSUMER_BINDINGS,
        "inspection": INSPECTION,
        "summary_counts": summary,
    }


# --------------------------------------------------------------------------- #
# Fixtures.
# --------------------------------------------------------------------------- #


def fixture_certified_full() -> dict:
    surface = "notebooks"
    return {
        "fixture_id": "fixture.m5_efficiency_governance.certified_full",
        "description": "A certified low-power row: clean evidence at every dimension certifies at the certified-low-power ceiling.",
        "row": {
            "row_id": "fixture.certified_full",
            "m5_surface": surface,
            "efficiency_state": "ThermalConstrained",
            "source_of_change": ["thermal_pressure"],
            "throttled_subsystems": ["ai_warmup", "speculative_prefetch"],
            "hidden_pane_bindings": [
                {"visibility_state": "hidden_tab", "behaviors": ["render_suppressed", "fully_quiescent"]}
            ],
            "override_posture": "user_override_session_only",
            "recovery_state": "not_in_recovery",
            "posture": "certified_low_power",
            "published_claim_ceiling": "certified_low_power",
            "required_publication_surfaces": REQUIRED_PUBLICATION_SURFACES,
            "evidence": certified_evidence(surface, propagated=REQUIRED_PUBLICATION_SURFACES),
        },
    }


def fixture_certified_qualified() -> dict:
    surface = "traces"
    return {
        "fixture_id": "fixture.m5_efficiency_governance.certified_qualified",
        "description": "A clean qualified low-power row certifies at the qualified-low-power ceiling.",
        "row": {
            "row_id": "fixture.certified_qualified",
            "m5_surface": surface,
            "efficiency_state": "ThermalConstrained",
            "source_of_change": ["thermal_pressure"],
            "throttled_subsystems": ["graph_enrichment"],
            "hidden_pane_bindings": [
                {"visibility_state": "collapsed_split", "behaviors": ["render_suppressed"]}
            ],
            "override_posture": "policy_blocked",
            "recovery_state": "not_in_recovery",
            "posture": "qualified_low_power",
            "published_claim_ceiling": "qualified_low_power",
            "required_publication_surfaces": ["docs", "help", "support_export"],
            "evidence": certified_evidence(surface, propagated=["docs", "help", "support_export"]),
        },
    }


def _drill_row(row_id, surface, posture, evidence_overrides, *, override_posture="user_override_session_only", required=None):
    required = required or REQUIRED_PUBLICATION_SURFACES
    evidence = certified_evidence(surface, propagated=required)
    evidence.update(evidence_overrides)
    return {
        "row_id": row_id,
        "m5_surface": surface,
        "efficiency_state": "EfficiencyAware",
        "source_of_change": ["os_battery_saver"],
        "throttled_subsystems": ["speculative_prefetch"],
        "hidden_pane_bindings": [
            {"visibility_state": "hidden_tab", "behaviors": ["render_suppressed"]}
        ],
        "override_posture": override_posture,
        "recovery_state": "not_in_recovery",
        "posture": posture,
        "published_claim_ceiling": posture,
        "required_publication_surfaces": required,
        "evidence": evidence,
    }


def negative_fixtures() -> list[dict]:
    return [
        {
            "fixture_id": "fixture.m5_efficiency_governance.missing_efficiency_state_evidence",
            "description": "A row with no materialized efficiency-state evidence quarantines to the undeclared-badge floor and holds promotion.",
            "row": _drill_row(
                "fixture.missing_efficiency_state_evidence",
                "notebooks",
                "certified_low_power",
                {"efficiency_state_evidence_present": False},
            ),
        },
        {
            "fixture_id": "fixture.m5_efficiency_governance.vague_low_power_badge",
            "description": "A low-power state with no declared behaviour change quarantines as a vague badge.",
            "row": _drill_row(
                "fixture.vague_low_power_badge",
                "companion_adjacent",
                "qualified_low_power",
                {"declares_behavior_change": False},
            ),
        },
        {
            "fixture_id": "fixture.m5_efficiency_governance.unqualified_hidden_work_suppression",
            "description": "A hidden-pane binding with a committed render violation narrows to the state-declared floor.",
            "row": _drill_row(
                "fixture.unqualified_hidden_work_suppression",
                "previews",
                "certified_low_power",
                {"hidden_work_suppression_qualified": False, "hidden_pane_render_violation_count": 2},
            ),
        },
        {
            "fixture_id": "fixture.m5_efficiency_governance.protected_path_regression_under_pressure",
            "description": "A protected interaction that regresses under pressure narrows to the state-declared floor.",
            "row": _drill_row(
                "fixture.protected_path_regression_under_pressure",
                "pipelines",
                "certified_low_power",
                {"protected_paths_preserved": False},
            ),
        },
        {
            "fixture_id": "fixture.m5_efficiency_governance.override_not_policy_aware",
            "description": "A user-overridable posture with no policy-aware override reference narrows off the certified posture.",
            "row": _drill_row(
                "fixture.override_not_policy_aware",
                "docs_browser_panes",
                "certified_low_power",
                {"override_policy_aware": False, "override_policy_ref": None},
            ),
        },
        {
            "fixture_id": "fixture.m5_efficiency_governance.recovery_not_staged",
            "description": "A row that requires recovery but does not stage it narrows off the certified posture.",
            "row": _drill_row(
                "fixture.recovery_not_staged",
                "support_exports",
                "certified_low_power",
                {"recovery_required": True, "recovery_staged": False},
            ),
        },
        {
            "fixture_id": "fixture.m5_efficiency_governance.missing_consumer_propagation",
            "description": "A row that does not reach every required publication surface narrows off the certified posture.",
            "row": _drill_row(
                "fixture.missing_consumer_propagation",
                "remote_sessions",
                "certified_low_power",
                {"propagated_surfaces": ["docs", "help", "support_export"]},
            ),
        },
        {
            "fixture_id": "fixture.m5_efficiency_governance.quarantined_badge_does_not_block",
            "description": "An undeclared-badge posture quarantines but does not hold promotion: it is retained for diagnosis only.",
            "row": _drill_row(
                "fixture.quarantined_badge_does_not_block",
                "companion_adjacent",
                "undeclared_badge",
                {
                    "efficiency_state_evidence_present": False,
                    "declares_behavior_change": False,
                    "binds_hidden_panes": False,
                    "hidden_work_suppression_qualified": False,
                },
                override_posture="policy_blocked",
                required=["docs"],
            ),
        },
    ]


def finalize_fixture(engine: "gate.GovernanceEngine", fixture: dict) -> dict:
    result = engine.recompute(fixture["row"])
    fixture = {
        "$schema": gate.FIXTURE_SCHEMA_REL,
        "fixture_id": fixture["fixture_id"],
        "description": fixture["description"],
        "as_of": AS_OF,
        "row": fixture["row"],
        "expected_fired_reasons": result["fired"],
        "expected_effective_posture": result["effective"],
        "expected_certification_state": result["state"],
        "expected_blocks_promotion": result["blocks"],
    }
    return fixture


def fixture_filename(fixture_id: str) -> str:
    return fixture_id.rsplit(".", 1)[-1] + ".json"


def write_json(rel: str, payload) -> None:
    path = REPO_ROOT / rel
    path.parent.mkdir(parents=True, exist_ok=True)
    path.write_text(json.dumps(payload, indent=2) + "\n", encoding="utf-8")
    print(f"wrote {rel}")


def main() -> int:
    matrix = build_matrix()
    write_json(MATRIX_REL, matrix)

    engine = gate.GovernanceEngine(metadata_matrix())
    fixtures = [fixture_certified_full(), fixture_certified_qualified(), *negative_fixtures()]
    manifest_lines = [
        "schema_ref: schemas/efficiency/m5-efficiency-governance-fixture.schema.json",
        "matrix_ref: artifacts/efficiency/m5-efficiency-governance.json",
        "doc_ref: docs/efficiency/m5-efficiency-governance.md",
        "# Each fixture carries one governance row and the minimal evidence the recompute reads,",
        "# replayed through the same engine the matrix rows use, proving every fail-closed",
        "# narrowing path fires and narrows to the expected posture.",
        "fixtures:",
    ]
    for fixture in fixtures:
        finalized = finalize_fixture(engine, fixture)
        fname = fixture_filename(fixture["fixture_id"])
        write_json(f"{FIXTURE_DIR_REL}/{fname}", finalized)
        manifest_lines.append(f"  - {FIXTURE_DIR_REL}/{fname}")

    manifest_path = REPO_ROOT / FIXTURE_DIR_REL / "manifest.yaml"
    manifest_path.write_text("\n".join(manifest_lines) + "\n", encoding="utf-8")
    print(f"wrote {FIXTURE_DIR_REL}/manifest.yaml")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
