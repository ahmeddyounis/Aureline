//! Fixture-replay and invariant tests for the stable migration-center diff /
//! rollback / unsupported-gap taxonomy corpus.
//!
//! The records live under
//! `fixtures/ux/m4/finish-the-migration-center-diff-rollback-and-unsupported/`
//! and are minted by the `aureline_shell_migration_center_stable` emitter so the
//! checked-in JSON stays a literal projection of the in-code corpus.
//!
//! What this guards (the acceptance criteria for this lane):
//!
//! - Each scenario's record on disk matches the in-code projection bit-for-bit.
//!   Regenerate with:
//!
//!   ```sh
//!   cargo run -q -p aureline-shell \
//!     --bin aureline_shell_migration_center_stable -- emit-fixtures \
//!     fixtures/ux/m4/finish-the-migration-center-diff-rollback-and-unsupported
//!   ```
//!
//! - Every flow shows a before/after diff before apply, with every row carrying
//!   both sides and citing one checkpoint.
//! - No flow over-claims: the diff/rollback/gap/full-fidelity ceiling is bound to
//!   the real evidence.
//! - A flow missing live rollback evidence is narrowed below Stable with a named
//!   reason rather than inheriting an adjacent green row.
//! - Every Unsupported / Shimmed gap is visible before apply and keeps a
//!   Review-gaps recovery route.
//! - The migration center, settings import history, and command palette share one
//!   model; the same flow opens from all four surfaces, keyboard-first.
//! - Tab order, narration (which discloses the ecosystem), action labels, and
//!   recovery affordances stay reachable in normal, high-contrast, and zoomed
//!   layouts.
//! - Every row stays available without an account or managed services.

use aureline_shell::migration_center_stable::{
    is_canonical_object_ref, migration_flow_disclosure_corpus, required_recovery_actions,
    LayoutMode, MigrationFlowDisclosureRecord, MigrationRouteSurface, StableClaimClass,
};

const FIXTURE_DIR: &str = concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../fixtures/ux/m4/finish-the-migration-center-diff-rollback-and-unsupported",
);

fn load_record(filename: &str) -> MigrationFlowDisclosureRecord {
    let path = format!("{FIXTURE_DIR}/{filename}");
    let body =
        std::fs::read_to_string(&path).unwrap_or_else(|err| panic!("failed to read {path}: {err}"));
    serde_json::from_str(&body).unwrap_or_else(|err| panic!("failed to parse {path}: {err}"))
}

#[test]
fn every_scenario_fixture_matches_in_code_projection() {
    for scenario in migration_flow_disclosure_corpus() {
        let on_disk = load_record(scenario.fixture_filename);
        let in_code = scenario.record();
        assert_eq!(
            on_disk, in_code,
            "{} fixture drifted; re-emit with `cargo run -q -p aureline-shell --bin aureline_shell_migration_center_stable -- emit-fixtures fixtures/ux/m4/finish-the-migration-center-diff-rollback-and-unsupported`",
            scenario.fixture_filename,
        );
    }
}

#[test]
fn pinned_rollups_match_each_scenario() {
    for scenario in migration_flow_disclosure_corpus() {
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
            record.taxonomy.unsupported, scenario.expected_unsupported,
            "{} unsupported count",
            scenario.scenario_id
        );
        assert_eq!(
            record.taxonomy.shimmed, scenario.expected_shimmed,
            "{} shimmed count",
            scenario.scenario_id
        );
        assert_eq!(
            record.rollback.is_live_for_flow(), scenario.expected_rollback_live,
            "{} rollback_live",
            scenario.scenario_id
        );
    }
}

#[test]
fn diff_is_reviewable_before_apply() {
    for scenario in migration_flow_disclosure_corpus() {
        let record = load_record(scenario.fixture_filename);
        assert!(
            record.diff.is_reviewable_before_apply(),
            "{} diff is not reviewable before apply",
            scenario.scenario_id,
        );
        assert!(
            record.diff.reviewed_before_apply,
            "{} diff not shown before apply",
            scenario.scenario_id,
        );
        assert!(
            record.diff.every_row_has_before_after && record.diff.every_row_uses_one_checkpoint,
            "{} diff rows incomplete",
            scenario.scenario_id,
        );
    }
}

#[test]
fn claim_ceiling_never_overclaims() {
    for scenario in migration_flow_disclosure_corpus() {
        let record = load_record(scenario.fixture_filename);
        let ceiling = record.claim_ceiling;
        if ceiling.asserts_diff_reviewed_before_apply {
            assert!(
                record.diff.is_reviewable_before_apply(),
                "{} claims diff reviewed it cannot prove",
                scenario.scenario_id,
            );
        }
        if ceiling.asserts_rollback_available {
            assert!(
                record.rollback.is_live_for_flow(),
                "{} claims rollback available it cannot prove",
                scenario.scenario_id,
            );
        }
        if ceiling.asserts_no_unsupported_gaps {
            assert!(
                record.taxonomy.gaps.is_empty(),
                "{} claims no unsupported gaps but has some",
                scenario.scenario_id,
            );
        }
        if ceiling.asserts_full_fidelity_import {
            assert!(
                record.taxonomy.is_full_fidelity(),
                "{} claims full-fidelity import it cannot prove",
                scenario.scenario_id,
            );
        }
    }
}

#[test]
fn matrix_spans_stable_and_narrowed_flows() {
    let corpus = migration_flow_disclosure_corpus();
    let stable = corpus
        .iter()
        .filter(|s| s.expected_claim_class == StableClaimClass::Stable)
        .count();
    let narrowed = corpus.len() - stable;
    assert!(stable >= 1, "matrix must include a Stable flow");
    assert!(narrowed >= 1, "matrix must include a narrowed flow");
}

#[test]
fn narrowed_flows_drop_below_cutline_and_name_a_reason() {
    for scenario in migration_flow_disclosure_corpus() {
        let record = load_record(scenario.fixture_filename);
        let qualification = &record.stable_qualification;
        if qualification.claim_class == StableClaimClass::Stable {
            assert!(
                qualification.qualifies_stable && qualification.narrowing_reasons.is_empty(),
                "{} Stable flow carries a narrowing reason",
                scenario.scenario_id,
            );
        } else {
            assert!(
                !qualification.qualifies_stable,
                "{} narrowed flow still qualifies",
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
            assert!(
                !record.claim_ceiling.asserts_rollback_available,
                "{} narrowed flow over-claims rollback",
                scenario.scenario_id,
            );
        }
    }
}

#[test]
fn unsupported_gaps_are_visible_before_apply() {
    for scenario in migration_flow_disclosure_corpus() {
        let record = load_record(scenario.fixture_filename);
        assert!(
            record.taxonomy.unsupported_gaps_visible_before_apply,
            "{} hides the gap lane",
            scenario.scenario_id,
        );
        for gap in &record.taxonomy.gaps {
            assert!(
                gap.visible_before_apply,
                "{} gap {} hidden before apply",
                scenario.scenario_id,
                gap.gap_id,
            );
        }
        // Gap counts match the taxonomy counts.
        let gap_unsupported = record
            .taxonomy
            .gaps
            .iter()
            .filter(|g| g.classification.as_str() == "unsupported")
            .count() as u32;
        let gap_shimmed = record
            .taxonomy
            .gaps
            .iter()
            .filter(|g| g.classification.as_str() == "shimmed")
            .count() as u32;
        assert_eq!(gap_unsupported, record.taxonomy.unsupported, "{} unsupported gap count", scenario.scenario_id);
        assert_eq!(gap_shimmed, record.taxonomy.shimmed, "{} shimmed gap count", scenario.scenario_id);
    }
}

#[test]
fn recovery_routes_are_complete_and_keyboard_reachable() {
    for scenario in migration_flow_disclosure_corpus() {
        let record = load_record(scenario.fixture_filename);
        let has_gaps = !record.taxonomy.gaps.is_empty();
        let route_ids: Vec<&str> = record
            .recovery_routes
            .iter()
            .map(|route| route.action_id.as_str())
            .collect();
        for required in required_recovery_actions(record.rollback.is_live_for_flow(), has_gaps) {
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
                scenario.scenario_id,
                route.action_id,
            );
        }
    }
}

#[test]
fn surfaces_share_one_model() {
    for scenario in migration_flow_disclosure_corpus() {
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
        for required in ["settings", "help", "support_export"] {
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
    for scenario in migration_flow_disclosure_corpus() {
        let record = load_record(scenario.fixture_filename);
        for required in MigrationRouteSurface::REQUIRED {
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
                route.activates_same_flow,
                "{} route {} activates a different flow",
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
    for scenario in migration_flow_disclosure_corpus() {
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
    for scenario in migration_flow_disclosure_corpus() {
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
    for scenario in migration_flow_disclosure_corpus() {
        let record = load_record(scenario.fixture_filename);
        for (label, value) in [
            ("migration_session_ref", &record.migration_session_ref),
            ("diff.diff_preview_ref", &record.diff.diff_preview_ref),
            ("rollback.checkpoint_ref", &record.rollback.checkpoint_ref),
            (
                "rollback.restore_record_ref",
                &record.rollback.restore_record_ref,
            ),
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
        if let Some(undo) = &record.rollback.undo_action_ref {
            assert!(is_canonical_object_ref(undo), "{} undo ref", scenario.scenario_id);
        }
        if let Some(compare) = &record.rollback.compare_action_ref {
            assert!(
                is_canonical_object_ref(compare),
                "{} compare ref",
                scenario.scenario_id
            );
        }
    }
}
