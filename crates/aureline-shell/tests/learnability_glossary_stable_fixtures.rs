//! Fixture-replay and invariant tests for the stable learnability / glossary /
//! contextual docs-help disclosure corpus.
//!
//! The records live under
//! `fixtures/ux/m4/promote-learnability-glossary-and-contextual-docs-help-guidance/`
//! and are minted by the `aureline_shell_learnability_glossary_stable` emitter
//! so the checked-in JSON stays a literal projection of the in-code corpus.
//!
//! What this guards (the acceptance criteria for this lane):
//!
//! - Each scenario's record on disk matches the in-code projection bit-for-bit.
//!   Regenerate with:
//!
//!   ```sh
//!   cargo run -q -p aureline-shell \
//!     --bin aureline_shell_learnability_glossary_stable -- emit-fixtures \
//!     fixtures/ux/m4/promote-learnability-glossary-and-contextual-docs-help-guidance
//!   ```
//!
//! - The learnability layer is stable only when it cites command/file/symbol
//!   truth, preserves exact focus return, and never forces a tutorial funnel
//!   before first useful work.
//! - No row over-claims: each pillar of the claim ceiling is bound to the real
//!   evidence.
//! - A row missing a pillar is narrowed below Stable with a named reason rather
//!   than inheriting an adjacent green row.
//! - Any guided tour / learning / teaching affordance carries its own
//!   Preview/Beta/Stable marker on every surface rather than implying stable
//!   coverage by adjacency.
//! - Dismissals, resume entries, and the learning digest stay user-owned and
//!   local-first — never repo-visible or telemetry-grade.
//! - The switching row, docs/help browser, and command palette share one model;
//!   the same row opens from all four surfaces, keyboard-first.
//! - Tab order, narration (which discloses the ecosystem), action labels, and
//!   recovery affordances stay reachable in normal, high-contrast, and zoomed
//!   layouts.
//! - Every row stays available without an account or managed services.

use aureline_shell::learnability_glossary_stable::{
    is_canonical_object_ref, is_command_file_symbol_anchor, learnability_disclosure_corpus,
    required_recovery_actions, LayoutMode, LearnabilityDisclosureRecord, LearnabilityRouteSurface,
    StableClaimClass,
};

const FIXTURE_DIR: &str = concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../fixtures/ux/m4/promote-learnability-glossary-and-contextual-docs-help-guidance",
);

fn load_record(filename: &str) -> LearnabilityDisclosureRecord {
    let path = format!("{FIXTURE_DIR}/{filename}");
    let body =
        std::fs::read_to_string(&path).unwrap_or_else(|err| panic!("failed to read {path}: {err}"));
    serde_json::from_str(&body).unwrap_or_else(|err| panic!("failed to parse {path}: {err}"))
}

#[test]
fn every_scenario_fixture_matches_in_code_projection() {
    for scenario in learnability_disclosure_corpus() {
        let on_disk = load_record(scenario.fixture_filename);
        let in_code = scenario.record();
        assert_eq!(
            on_disk, in_code,
            "{} fixture drifted; re-emit with `cargo run -q -p aureline-shell --bin aureline_shell_learnability_glossary_stable -- emit-fixtures fixtures/ux/m4/promote-learnability-glossary-and-contextual-docs-help-guidance`",
            scenario.fixture_filename,
        );
    }
}

#[test]
fn pinned_rollups_match_each_scenario() {
    for scenario in learnability_disclosure_corpus() {
        let record = load_record(scenario.fixture_filename);
        assert_eq!(
            record.source_ecosystem, scenario.expected_ecosystem,
            "{} ecosystem",
            scenario.scenario_id
        );
        assert_eq!(
            record.stable_qualification.claim_class, scenario.expected_claim_class,
            "{} claim_class",
            scenario.scenario_id
        );
        assert_eq!(
            record.stable_qualification.qualifies_stable, scenario.expected_qualifies_stable,
            "{} qualifies_stable",
            scenario.scenario_id
        );
        assert_eq!(
            record.glossary_chips.len(),
            scenario.expected_glossary_chip_count,
            "{} glossary chip count",
            scenario.scenario_id
        );
        assert_eq!(
            record.why_now_card.is_grounded_in_truth(),
            scenario.expected_why_now_grounded,
            "{} why-now grounded",
            scenario.scenario_id
        );
    }
}

#[test]
fn glossary_chips_cite_stable_anchors() {
    for scenario in learnability_disclosure_corpus() {
        let record = load_record(scenario.fixture_filename);
        assert!(
            !record.glossary_chips.is_empty(),
            "{} has no glossary chips",
            scenario.scenario_id,
        );
        for chip in &record.glossary_chips {
            assert!(
                chip.has_stable_anchor(),
                "{} chip {} lacks a stable anchor",
                scenario.scenario_id,
                chip.chip_id,
            );
        }
    }
}

#[test]
fn claim_ceiling_never_overclaims() {
    for scenario in learnability_disclosure_corpus() {
        let record = load_record(scenario.fixture_filename);
        let ceiling = record.claim_ceiling;
        if ceiling.asserts_glossary_anchors_stable {
            assert!(
                record
                    .glossary_chips
                    .iter()
                    .all(|chip| chip.has_stable_anchor()),
                "{} claims stable glossary anchors it cannot prove",
                scenario.scenario_id,
            );
        }
        if ceiling.asserts_why_now_grounded {
            assert!(
                is_command_file_symbol_anchor(&record.why_now_card.cited_target),
                "{} claims a grounded why-now card it cannot prove",
                scenario.scenario_id,
            );
        }
        if ceiling.asserts_contextual_docs_cited {
            assert!(
                record.contextual_docs.cites_docs_nodes(),
                "{} claims cited contextual docs it cannot prove",
                scenario.scenario_id,
            );
        }
        if ceiling.asserts_focus_return_preserved {
            assert!(
                record.posture.preserves_exact_focus_return,
                "{} claims preserved focus return it cannot prove",
                scenario.scenario_id,
            );
        }
        if ceiling.asserts_non_blocking {
            assert!(
                record.posture.is_non_blocking() && !record.why_now_card.blocks_first_useful_work,
                "{} claims a non-blocking layer it cannot prove",
                scenario.scenario_id,
            );
        }
    }
}

#[test]
fn matrix_spans_stable_and_narrowed_rows() {
    let corpus = learnability_disclosure_corpus();
    let stable = corpus
        .iter()
        .filter(|s| s.expected_claim_class == StableClaimClass::Stable)
        .count();
    let narrowed = corpus.len() - stable;
    assert!(stable >= 1, "matrix must include a Stable row");
    assert!(narrowed >= 1, "matrix must include a narrowed row");
}

#[test]
fn narrowed_rows_drop_below_cutline_and_name_a_reason() {
    for scenario in learnability_disclosure_corpus() {
        let record = load_record(scenario.fixture_filename);
        let qualification = &record.stable_qualification;
        if qualification.claim_class == StableClaimClass::Stable {
            assert!(
                qualification.qualifies_stable && qualification.narrowing_reasons.is_empty(),
                "{} Stable row carries a narrowing reason",
                scenario.scenario_id,
            );
        } else {
            assert!(
                !qualification.qualifies_stable,
                "{} narrowed row still qualifies",
                scenario.scenario_id,
            );
            assert!(
                !qualification.claim_class.at_or_above_cutline(),
                "{} narrowed claim sits at or above the cutline",
                scenario.scenario_id,
            );
            assert!(
                !qualification.narrowing_reasons.is_empty(),
                "{} narrowed without a named reason",
                scenario.scenario_id,
            );
        }
    }
}

#[test]
fn opt_in_layer_never_blocks_first_useful_work() {
    for scenario in learnability_disclosure_corpus() {
        let record = load_record(scenario.fixture_filename);
        assert!(
            record.posture.opt_in,
            "{} learnability layer is not opt-in",
            scenario.scenario_id,
        );
        assert!(
            !record.posture.blocks_first_useful_work,
            "{} learnability layer blocks first useful work",
            scenario.scenario_id,
        );
        assert!(
            !record.why_now_card.blocks_first_useful_work,
            "{} why-now card blocks first useful work",
            scenario.scenario_id,
        );
        assert!(
            record.why_now_card.dismissible,
            "{} why-now card is not dismissible",
            scenario.scenario_id,
        );
    }
}

#[test]
fn guided_affordances_carry_a_marked_support_boundary() {
    for scenario in learnability_disclosure_corpus() {
        let record = load_record(scenario.fixture_filename);
        for affordance in &record.guided_affordances {
            assert!(
                affordance.marker_fully_disclosed(),
                "{} guided affordance {} hides its marker on a surface",
                scenario.scenario_id,
                affordance.affordance_id,
            );
            assert!(
                !affordance.support_boundary.trim().is_empty(),
                "{} guided affordance {} lacks a support boundary",
                scenario.scenario_id,
                affordance.affordance_id,
            );
        }
        // A below-stable affordance must surface the honesty marker so it never
        // implies full stable coverage by adjacency.
        if record
            .guided_affordances
            .iter()
            .any(|a| a.is_below_stable())
        {
            assert!(
                record.honesty_marker_present,
                "{} hides the honesty marker despite a below-stable affordance",
                scenario.scenario_id,
            );
        }
    }
}

#[test]
fn learning_state_stays_local_first_and_user_owned() {
    for scenario in learnability_disclosure_corpus() {
        let record = load_record(scenario.fixture_filename);
        assert!(
            record.privacy.is_local_first(),
            "{} learning state is not local-first",
            scenario.scenario_id,
        );
        assert!(
            record.privacy.all_user_owned(),
            "{} learning state is not user-owned",
            scenario.scenario_id,
        );
        assert!(
            !record.privacy.repo_visible,
            "{} learning state is repo-visible",
            scenario.scenario_id,
        );
        assert!(
            !record.privacy.telemetry_grade,
            "{} learning state is telemetry-grade",
            scenario.scenario_id,
        );
    }
}

#[test]
fn recovery_routes_are_complete_and_keyboard_reachable() {
    for scenario in learnability_disclosure_corpus() {
        let record = load_record(scenario.fixture_filename);
        let has_guided = !record.guided_affordances.is_empty();
        let route_ids: Vec<&str> = record
            .recovery_routes
            .iter()
            .map(|route| route.action_id.as_str())
            .collect();
        for required in required_recovery_actions(has_guided) {
            assert!(
                route_ids.contains(&required.as_str()),
                "{} missing recovery route {}",
                scenario.scenario_id,
                required.as_str(),
            );
        }
        for route in &record.recovery_routes {
            assert!(
                route.keyboard_reachable,
                "{} recovery route {} not keyboard reachable",
                scenario.scenario_id, route.action_id,
            );
        }
    }
}

#[test]
fn surfaces_share_one_model() {
    for scenario in learnability_disclosure_corpus() {
        let record = load_record(scenario.fixture_filename);
        assert!(
            record.surfaces.parity_holds,
            "{} surfaces disagree",
            scenario.scenario_id,
        );
        let route_ids: Vec<String> = record
            .recovery_routes
            .iter()
            .map(|route| route.action_id.clone())
            .collect();
        assert_eq!(
            record.surfaces.recovery_action_ids, route_ids,
            "{} surface recovery ids drift from recovery routes",
            scenario.scenario_id,
        );
        for required in ["docs_help", "command_palette", "support_export"] {
            assert!(
                record
                    .surfaces
                    .reopen_surfaces
                    .iter()
                    .any(|surface| surface == required),
                "{} dropped reopen surface {}",
                scenario.scenario_id,
                required,
            );
        }
    }
}

#[test]
fn routes_reach_every_surface_keyboard_first() {
    for scenario in learnability_disclosure_corpus() {
        let record = load_record(scenario.fixture_filename);
        for required in LearnabilityRouteSurface::REQUIRED {
            let route = record
                .routes
                .iter()
                .find(|route| route.surface == required)
                .unwrap_or_else(|| {
                    panic!(
                        "{} missing route surface {}",
                        scenario.scenario_id,
                        required.as_str()
                    )
                });
            assert!(
                route.keyboard_reachable,
                "{} route {} not keyboard reachable",
                scenario.scenario_id,
                required.as_str(),
            );
            assert!(
                route.activates_same_row,
                "{} route {} activates a different row",
                scenario.scenario_id,
                required.as_str(),
            );
            assert!(
                is_canonical_object_ref(&route.route_ref),
                "{} route {} ref {:?} not canonical",
                scenario.scenario_id,
                required.as_str(),
                route.route_ref,
            );
        }
    }
}

#[test]
fn accessibility_holds_in_every_layout() {
    for scenario in learnability_disclosure_corpus() {
        let record = load_record(scenario.fixture_filename);
        assert!(
            record
                .accessibility
                .row_narration
                .contains(&record.source_ecosystem_label),
            "{} narration omits the ecosystem",
            scenario.scenario_id,
        );
        assert_eq!(
            record.accessibility.action_labels.len(),
            record.recovery_routes.len(),
            "{} action labels drift from recovery routes",
            scenario.scenario_id,
        );
        for (label, route) in record
            .accessibility
            .action_labels
            .iter()
            .zip(record.recovery_routes.iter())
        {
            assert_eq!(
                label, &route.action_label,
                "{} action label drift",
                scenario.scenario_id
            );
        }
        for required in LayoutMode::REQUIRED {
            let mode = record
                .accessibility
                .layout_modes
                .iter()
                .find(|mode| mode.mode == required)
                .unwrap_or_else(|| {
                    panic!(
                        "{} missing layout mode {}",
                        scenario.scenario_id,
                        required.as_str()
                    )
                });
            assert!(
                mode.row_narration_available && mode.recovery_affordances_reachable,
                "{} layout mode {} unreachable",
                scenario.scenario_id,
                required.as_str(),
            );
        }
    }
}

#[test]
fn rows_stay_available_without_account_or_managed_services() {
    for scenario in learnability_disclosure_corpus() {
        let record = load_record(scenario.fixture_filename);
        assert!(
            record.available_without_account,
            "{} hidden without an account",
            scenario.scenario_id,
        );
        assert!(
            record.available_without_managed_services,
            "{} hidden without managed services",
            scenario.scenario_id,
        );
    }
}

#[test]
fn top_level_refs_are_canonical_durable_objects() {
    for scenario in learnability_disclosure_corpus() {
        let record = load_record(scenario.fixture_filename);
        for (label, value) in [
            (
                "contextual_docs.docs_browser_ref",
                &record.contextual_docs.docs_browser_ref,
            ),
            (
                "posture.focus_return_anchor_ref",
                &record.posture.focus_return_anchor_ref,
            ),
            ("privacy.state_store_ref", &record.privacy.state_store_ref),
            ("diagnostics_export_ref", &record.diagnostics_export_ref),
            ("support_export_ref", &record.support_export_ref),
        ] {
            assert!(
                is_canonical_object_ref(value),
                "{} {label} {value:?} not canonical",
                scenario.scenario_id,
            );
        }
        for value in record
            .evidence_refs
            .iter()
            .chain(record.narrative_refs.iter())
        {
            assert!(
                is_canonical_object_ref(value),
                "{} ref {value:?} not canonical",
                scenario.scenario_id,
            );
        }
    }
}
