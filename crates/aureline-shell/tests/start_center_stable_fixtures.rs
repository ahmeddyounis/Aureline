//! Fixture-replay and invariant tests for the stable Start Center / recent-work
//! / workspace-switcher target-kind disclosure corpus.
//!
//! The records live under
//! `fixtures/ux/m4/stabilize-the-start-center-recent-work-list-workspace/` and are
//! minted by the `aureline_shell_start_center_stable` emitter so the checked-in
//! JSON stays a literal projection of the in-code corpus.
//!
//! What this guards (the acceptance criteria for this lane):
//!
//! - Each scenario's record on disk matches the in-code projection bit-for-bit.
//!   Regenerate with:
//!
//!   ```sh
//!   cargo run -q -p aureline-shell \
//!     --bin aureline_shell_start_center_stable -- emit-fixtures \
//!     fixtures/ux/m4/stabilize-the-start-center-recent-work-list-workspace
//!   ```
//!
//! - Target-kind truth is consistent across local, remote, and managed examples,
//!   and the surface label matches the canonical workspace vocabulary so docs and
//!   Help/About ingest one vocabulary.
//! - No row claims a live open, remote availability, restore fidelity, or trust
//!   the product cannot prove.
//! - A failed open keeps the entry and its Locate / Reconnect / open-minimal /
//!   Remove routes; Remove is scoped to recent-work metadata only.
//! - The Start Center and switcher share one model, the switcher keeps a return
//!   path, and the same target opens from all four surfaces, keyboard-first.
//! - Tab order, row narration, action labels, and recovery affordances stay
//!   reachable in normal, high-contrast, and zoomed layouts.
//! - Every row stays available without an account or managed services.

use aureline_shell::restore::placeholders::WorkspaceSwitchRecoveryAction;
use aureline_shell::start_center_stable::{
    entry_target_disclosure_corpus, is_canonical_object_ref, required_recovery_actions,
    EntryRouteSurface, EntryTargetDisclosureRecord, LayoutMode, TargetClass,
};
use aureline_workspace::{RecentWorkFailureState, RestoreAvailability, TrustState};

const FIXTURE_DIR: &str = concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../fixtures/ux/m4/stabilize-the-start-center-recent-work-list-workspace",
);

fn load_record(filename: &str) -> EntryTargetDisclosureRecord {
    let path = format!("{FIXTURE_DIR}/{filename}");
    let body =
        std::fs::read_to_string(&path).unwrap_or_else(|err| panic!("failed to read {path}: {err}"));
    serde_json::from_str(&body).unwrap_or_else(|err| panic!("failed to parse {path}: {err}"))
}

#[test]
fn every_scenario_fixture_matches_in_code_projection() {
    for scenario in entry_target_disclosure_corpus() {
        let on_disk = load_record(scenario.fixture_filename);
        let in_code = scenario.record();
        assert_eq!(
            on_disk, in_code,
            "{} fixture drifted; re-emit with `cargo run -q -p aureline-shell --bin aureline_shell_start_center_stable -- emit-fixtures fixtures/ux/m4/stabilize-the-start-center-recent-work-list-workspace`",
            scenario.fixture_filename,
        );
    }
}

#[test]
fn pinned_rollups_match_each_scenario() {
    for scenario in entry_target_disclosure_corpus() {
        let record = load_record(scenario.fixture_filename);
        assert_eq!(
            record.target_kind, scenario.expected_target_kind,
            "{} target_kind",
            scenario.scenario_id
        );
        assert_eq!(
            record.target_class, scenario.expected_target_class,
            "{} target_class",
            scenario.scenario_id
        );
        assert_eq!(
            record.failure_state, scenario.expected_failure_state,
            "{} failure_state",
            scenario.scenario_id
        );
        assert_eq!(
            record.trust_state, scenario.expected_trust_state,
            "{} trust_state",
            scenario.scenario_id
        );
        assert_eq!(
            record.restore_availability, scenario.expected_restore_availability,
            "{} restore_availability",
            scenario.scenario_id
        );
        assert_eq!(
            record.honesty_marker_present, scenario.expected_honesty_marker_present,
            "{} honesty_marker_present",
            scenario.scenario_id
        );
    }
}

#[test]
fn target_kind_vocabulary_matches_canonical_surface_labels() {
    // The label the product renders is the same vocabulary docs and Help/About
    // ingest: it is the canonical workspace surface label, not a hand-authored
    // variant.
    for scenario in entry_target_disclosure_corpus() {
        let record = load_record(scenario.fixture_filename);
        assert_eq!(
            record.target_kind_label,
            record.target_kind.surface_label(),
            "{} target_kind_label drifted from canonical surface label",
            scenario.scenario_id,
        );
    }
}

#[test]
fn target_kind_truth_spans_local_remote_and_managed() {
    let corpus = entry_target_disclosure_corpus();
    let classes: Vec<TargetClass> = corpus.iter().map(|s| s.expected_target_class).collect();
    for required in [
        TargetClass::Local,
        TargetClass::RemoteBacked,
        TargetClass::Managed,
    ] {
        assert!(
            classes.contains(&required),
            "claimed stable matrix is missing a {required:?} example",
        );
    }
}

#[test]
fn claim_ceiling_never_overclaims() {
    for scenario in entry_target_disclosure_corpus() {
        let record = load_record(scenario.fixture_filename);
        let ceiling = record.claim_ceiling;
        if record.failure_state != RecentWorkFailureState::Ready {
            assert!(
                !ceiling.asserts_live_open,
                "{} claims a live open for an unavailable target",
                scenario.scenario_id,
            );
        }
        if record.failure_state == RecentWorkFailureState::ReconnectRequired
            || record.target_class == TargetClass::Local
        {
            assert!(
                !ceiling.asserts_remote_available,
                "{} claims remote availability it cannot prove",
                scenario.scenario_id,
            );
        }
        if record.restore_availability != RestoreAvailability::Exact {
            assert!(
                !ceiling.asserts_full_restore_fidelity,
                "{} claims full restore fidelity it cannot prove",
                scenario.scenario_id,
            );
        }
        if record.trust_state != TrustState::Trusted {
            assert!(
                !ceiling.asserts_trusted_without_evaluation,
                "{} claims trust it cannot prove",
                scenario.scenario_id,
            );
        }
    }
}

#[test]
fn failed_opens_keep_entry_and_recovery_routes() {
    for scenario in entry_target_disclosure_corpus() {
        let record = load_record(scenario.fixture_filename);
        assert!(
            !record.discards_stale_entry_on_failure,
            "{} silently discards a stale entry",
            scenario.scenario_id,
        );
        let route_ids: Vec<&str> = record
            .recovery_routes
            .iter()
            .map(|route| route.action_id.as_str())
            .collect();
        for required in required_recovery_actions(record.failure_state) {
            assert!(
                route_ids.contains(&required.as_str()),
                "{} missing required recovery route {}",
                scenario.scenario_id,
                required.as_str(),
            );
        }
        for route in &record.recovery_routes {
            assert!(
                route.preserves_unrelated_state,
                "{} recovery route {} drops unrelated state",
                scenario.scenario_id, route.action_id,
            );
            if route.action_id == "remove_from_recents" {
                assert!(
                    route.metadata_only_cleanup,
                    "{} Remove is not metadata-only",
                    scenario.scenario_id,
                );
            }
        }
    }
}

#[test]
fn surfaces_share_one_model_with_a_return_path() {
    for scenario in entry_target_disclosure_corpus() {
        let record = load_record(scenario.fixture_filename);
        assert!(
            record.surfaces.parity_holds,
            "{} Start Center and switcher disagree",
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
        for required in [
            WorkspaceSwitchRecoveryAction::CancelSwitch,
            WorkspaceSwitchRecoveryAction::ReopenPreviousWorkspace,
        ] {
            assert!(
                record
                    .surfaces
                    .switch_failure_actions
                    .iter()
                    .any(|action| action == required.as_str()),
                "{} switcher dropped return path {}",
                scenario.scenario_id,
                required.as_str(),
            );
        }
    }
}

#[test]
fn routes_reach_every_surface_keyboard_first() {
    for scenario in entry_target_disclosure_corpus() {
        let record = load_record(scenario.fixture_filename);
        for required in EntryRouteSurface::REQUIRED {
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
                route.activates_same_target,
                "{} route {} activates a different target",
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
    for scenario in entry_target_disclosure_corpus() {
        let record = load_record(scenario.fixture_filename);
        assert!(
            record
                .accessibility
                .row_narration
                .contains(&record.target_kind_label),
            "{} narration omits the target kind",
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
    for scenario in entry_target_disclosure_corpus() {
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
fn refs_are_canonical_durable_objects() {
    for scenario in entry_target_disclosure_corpus() {
        let record = load_record(scenario.fixture_filename);
        for (label, value) in [
            ("recent_work_ref", &record.recent_work_ref),
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
