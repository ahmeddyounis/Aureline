//! Fixture-replay and invariant tests for the warm-startup / warm-restore /
//! first-useful-work drill corpus.
//!
//! The drills live under
//! `fixtures/ux/m4/harden_shell_startup_warm_restore_and_first_useful/` and are
//! minted by the `aureline_shell_warm_continuity_corpus` emitter so the checked-
//! in JSON stays a literal projection of the in-code corpus.
//!
//! What this guards:
//!
//! - Each scenario's record on disk matches the in-code projection bit-for-bit.
//!   Regenerate with:
//!
//!   ```sh
//!   cargo run -q -p aureline-shell \
//!     --bin aureline_shell_warm_continuity_corpus -- emit-fixtures \
//!     fixtures/ux/m4/harden_shell_startup_warm_restore_and_first_useful
//!   ```
//!
//! - Useful-chrome-first: shell chrome, command entry, and a stable focus target
//!   are all present and all reached before deep discovery completes.
//! - Restore honesty: each item is exactly / partially / needs-review, narrowed
//!   items name a downgrade trigger, and the live runtime is never implied to
//!   have resumed.
//! - No silent rerun: every side-effectful surface forbids auto rerun, and
//!   remote / authority-bound surfaces are gated behind fresh authorization or
//!   review with an explicit resume route.
//! - Bounded routing: the selected route is an inspectable candidate, is
//!   keyboard-reachable and non-destructive, and any remembered preference can
//!   never widen trust or run setup.
//! - Zone-owned truth stays put, and collapsed surfaces keep an explicit,
//!   keyboard-reachable reopen route and last meaningful state.

use aureline_shell::warm_continuity::{
    is_canonical_object_ref, warm_continuity_corpus, CollapseTargetClass, EntryCauseClass,
    LandingRouteClass, RestoreClassToken, RestoreProvenanceClass, ShellZoneToken,
    SideEffectfulSurfaceClass, StartupMilestoneClass, WarmContinuityRecord, WindowClassToken,
    ZoneOwnedCueClass,
};

const FIXTURE_DIR: &str = concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../fixtures/ux/m4/harden_shell_startup_warm_restore_and_first_useful",
);

fn load_record(filename: &str) -> WarmContinuityRecord {
    let path = format!("{FIXTURE_DIR}/{filename}");
    let body =
        std::fs::read_to_string(&path).unwrap_or_else(|err| panic!("failed to read {path}: {err}"));
    serde_json::from_str(&body).unwrap_or_else(|err| panic!("failed to parse {path}: {err}"))
}

#[test]
fn every_scenario_fixture_matches_in_code_projection() {
    for scenario in warm_continuity_corpus() {
        let on_disk = load_record(scenario.fixture_filename);
        let in_code = scenario.record();
        assert_eq!(
            on_disk, in_code,
            "{} fixture drifted; re-emit with `cargo run -q -p aureline-shell --bin aureline_shell_warm_continuity_corpus -- emit-fixtures fixtures/ux/m4/harden_shell_startup_warm_restore_and_first_useful`",
            scenario.fixture_filename,
        );
    }
}

#[test]
fn pinned_rollups_match_each_scenario() {
    for scenario in warm_continuity_corpus() {
        let record = load_record(scenario.fixture_filename);
        assert_eq!(record.entry_cause, scenario.expected_entry_cause, "{} entry_cause", scenario.scenario_id);
        assert_eq!(
            record.restore.restore_class, scenario.expected_restore_class,
            "{} restore_class",
            scenario.scenario_id,
        );
        assert_eq!(
            record.responsive.window_class, scenario.expected_window_class,
            "{} window_class",
            scenario.scenario_id,
        );
        assert_eq!(
            record.landing.selected_route, scenario.expected_landing_route,
            "{} landing_route",
            scenario.scenario_id,
        );
        assert_eq!(
            record.honesty_marker_present, scenario.expected_honesty_marker_present,
            "{} honesty_marker_present",
            scenario.scenario_id,
        );
        assert_eq!(
            record.summary_counts.restored_exactly_count, scenario.expected_restored_exactly_count,
            "{} restored_exactly_count",
            scenario.scenario_id,
        );
        assert_eq!(
            record.summary_counts.restored_partially_count,
            scenario.expected_restored_partially_count,
            "{} restored_partially_count",
            scenario.scenario_id,
        );
        assert_eq!(
            record.summary_counts.needs_review_count, scenario.expected_needs_review_count,
            "{} needs_review_count",
            scenario.scenario_id,
        );
    }
}

#[test]
fn useful_chrome_is_reached_before_deep_discovery() {
    for scenario in warm_continuity_corpus() {
        let record = load_record(scenario.fixture_filename);
        for required in StartupMilestoneClass::REQUIRED_BEFORE_DEEP_DISCOVERY {
            let row = record
                .startup
                .milestones
                .iter()
                .find(|m| m.milestone == *required)
                .unwrap_or_else(|| {
                    panic!("{}: missing early milestone {}", scenario.scenario_id, required.as_str())
                });
            assert!(
                row.reached_before_deep_discovery,
                "{}: {} must be reached before deep discovery",
                scenario.scenario_id,
                required.as_str(),
            );
            assert!(
                row.keyboard_reachable,
                "{}: {} must be keyboard reachable",
                scenario.scenario_id,
                required.as_str(),
            );
        }
    }
}

#[test]
fn restore_provenance_is_honest() {
    for scenario in warm_continuity_corpus() {
        let record = load_record(scenario.fixture_filename);
        let mut exactly = 0u32;
        let mut partially = 0u32;
        let mut needs_review = 0u32;
        for item in &record.restore.items {
            assert!(
                is_canonical_object_ref(&item.object_ref),
                "{}: restore item ref {:?} not canonical",
                scenario.scenario_id,
                item.object_ref,
            );
            match item.provenance {
                RestoreProvenanceClass::RestoredExactly => exactly += 1,
                RestoreProvenanceClass::RestoredPartially => {
                    partially += 1;
                    assert!(
                        item.downgrade_trigger.is_some(),
                        "{}: partial item {:?} must name a trigger",
                        scenario.scenario_id,
                        item.object_ref,
                    );
                }
                RestoreProvenanceClass::NeedsReview => {
                    needs_review += 1;
                    assert!(
                        item.downgrade_trigger.is_some(),
                        "{}: needs-review item {:?} must name a trigger",
                        scenario.scenario_id,
                        item.object_ref,
                    );
                }
            }
        }
        assert_eq!(exactly, record.summary_counts.restored_exactly_count, "{} exactly", scenario.scenario_id);
        assert_eq!(partially, record.summary_counts.restored_partially_count, "{} partially", scenario.scenario_id);
        assert_eq!(needs_review, record.summary_counts.needs_review_count, "{} needs_review", scenario.scenario_id);
        // The live runtime is never implied to have resumed.
        assert!(!record.display_copy.full_resumption_implied, "{}", scenario.scenario_id);
        assert!(!record.display_copy.side_effect_rerun_implied, "{}", scenario.scenario_id);
    }
}

#[test]
fn side_effectful_surfaces_are_never_silently_rerun() {
    for scenario in warm_continuity_corpus() {
        let record = load_record(scenario.fixture_filename);
        for surface in &record.restore.no_rerun_surfaces {
            assert!(
                surface.auto_rerun_forbidden,
                "{}: surface {} must forbid auto rerun",
                scenario.scenario_id,
                surface.surface_class.as_str(),
            );
            assert!(
                is_canonical_object_ref(&surface.resume_route_ref),
                "{}: surface {} resume route {:?} not canonical",
                scenario.scenario_id,
                surface.surface_class.as_str(),
                surface.resume_route_ref,
            );
            if surface.surface_class.inherently_authority_bound() {
                assert!(
                    surface.requires_fresh_authorization || surface.requires_review,
                    "{}: authority-bound surface {} must require fresh authorization or review",
                    scenario.scenario_id,
                    surface.surface_class.as_str(),
                );
            }
        }
    }
}

#[test]
fn first_useful_work_routing_is_bounded_and_reachable() {
    for scenario in warm_continuity_corpus() {
        let record = load_record(scenario.fixture_filename);
        let landing = &record.landing;
        assert!(landing.keyboard_reachable, "{}: landing not keyboard reachable", scenario.scenario_id);
        assert!(!landing.destructive, "{}: landing is destructive", scenario.scenario_id);
        assert!(
            landing.candidate_routes.contains(&landing.selected_route),
            "{}: selected route is not an inspectable candidate",
            scenario.scenario_id,
        );
        assert!(
            is_canonical_object_ref(&landing.target_ref),
            "{}: landing target {:?} not canonical",
            scenario.scenario_id,
            landing.target_ref,
        );
        if let Some(pref) = &landing.remembered_preference {
            assert!(!pref.widens_workspace_trust, "{}: pref widened trust", scenario.scenario_id);
            assert!(!pref.installs_packages, "{}: pref installed packages", scenario.scenario_id);
            assert!(!pref.applies_workflow_bundle, "{}: pref applied a bundle", scenario.scenario_id);
            assert!(
                !pref.suppresses_required_checkpoint,
                "{}: pref suppressed a checkpoint",
                scenario.scenario_id,
            );
        }
        assert!(!record.display_copy.remembered_preference_widened_trust, "{}", scenario.scenario_id);
    }
}

#[test]
fn first_useful_work_is_inspectable_in_support_export() {
    for scenario in warm_continuity_corpus() {
        let record = load_record(scenario.fixture_filename);
        let lines = record.support_export_lines().join("\n");
        assert!(
            lines.contains(&format!("route={}", record.landing.selected_route.as_str())),
            "{}: support export omits the selected route",
            scenario.scenario_id,
        );
        assert!(
            lines.contains(&format!("reason={}", record.landing.route_reason.as_str())),
            "{}: support export omits the route reason",
            scenario.scenario_id,
        );
        // Export is redaction-safe: no raw scheme leakage beyond canonical refs.
        assert!(!lines.contains("http://") && !lines.contains("https://"), "{}", scenario.scenario_id);
    }
}

#[test]
fn zone_owned_truth_stays_in_its_zone() {
    for scenario in warm_continuity_corpus() {
        let record = load_record(scenario.fixture_filename);
        for cue in &record.zone_identity.cues {
            assert!(
                !cue.relocated(),
                "{}: cue {} relocated from {} to {}",
                scenario.scenario_id,
                cue.cue.as_str(),
                cue.owning_zone.as_str(),
                cue.rendered_zone.as_str(),
            );
        }
        assert!(!record.display_copy.zone_cue_relocated, "{}", scenario.scenario_id);
    }
}

#[test]
fn collapsed_surfaces_stay_reachable() {
    for scenario in warm_continuity_corpus() {
        let record = load_record(scenario.fixture_filename);
        for surface in &record.responsive.collapsed_surfaces {
            assert!(surface.keyboard_reachable, "{}: collapsed surface unreachable", scenario.scenario_id);
            assert!(surface.approved_to_move, "{}: collapsed surface moved without approval", scenario.scenario_id);
            assert!(
                is_canonical_object_ref(&surface.reopen_route_ref),
                "{}: reopen route {:?} not canonical",
                scenario.scenario_id,
                surface.reopen_route_ref,
            );
            assert!(
                is_canonical_object_ref(&surface.last_meaningful_state_ref),
                "{}: last-state ref {:?} not canonical",
                scenario.scenario_id,
                surface.last_meaningful_state_ref,
            );
        }
        assert_eq!(
            record.responsive.collapsed_surfaces.len() as u32,
            record.summary_counts.collapsed_surface_count,
            "{}: collapsed surface count drift",
            scenario.scenario_id,
        );
        assert!(!record.display_copy.collapsed_surface_unreachable, "{}", scenario.scenario_id);
    }
}

#[test]
fn corpus_covers_required_regression_cases_and_vocabularies() {
    let mut causes = std::collections::BTreeSet::new();
    let mut restore_classes = std::collections::BTreeSet::new();
    let mut window_classes = std::collections::BTreeSet::new();
    let mut routes = std::collections::BTreeSet::new();
    let mut surfaces = std::collections::BTreeSet::new();
    let mut collapse_targets = std::collections::BTreeSet::new();
    let mut cues = std::collections::BTreeSet::new();
    let mut provenance = std::collections::BTreeSet::new();
    for scenario in warm_continuity_corpus() {
        let record = load_record(scenario.fixture_filename);
        causes.insert(record.entry_cause.as_str());
        restore_classes.insert(record.restore.restore_class.as_str());
        window_classes.insert(record.responsive.window_class.as_str());
        routes.insert(record.landing.selected_route.as_str());
        for s in &record.restore.no_rerun_surfaces {
            surfaces.insert(s.surface_class.as_str());
        }
        for c in &record.responsive.collapsed_surfaces {
            collapse_targets.insert(c.collapsed_to.as_str());
        }
        for c in &record.zone_identity.cues {
            cues.insert(c.cue.as_str());
        }
        for i in &record.restore.items {
            provenance.insert(i.provenance.as_str());
        }
    }
    // The five required regression cases plus the two baselines.
    for expected in [
        EntryCauseClass::WarmRelaunch.as_str(),
        EntryCauseClass::CrashRecovery.as_str(),
        EntryCauseClass::SleepResume.as_str(),
        EntryCauseClass::DisplayTopologyChange.as_str(),
        EntryCauseClass::MissingExtensionFallback.as_str(),
        EntryCauseClass::ExpiredRemoteSession.as_str(),
        EntryCauseClass::RevokedAuthorization.as_str(),
    ] {
        assert!(causes.contains(expected), "entry cause {expected} not exercised");
    }
    for expected in [
        RestoreClassToken::ExactRestore.as_str(),
        RestoreClassToken::CompatibleRestore.as_str(),
        RestoreClassToken::LayoutOnly.as_str(),
        RestoreClassToken::RecoveredDrafts.as_str(),
        RestoreClassToken::EvidenceOnly.as_str(),
    ] {
        assert!(restore_classes.contains(expected), "restore class {expected} not exercised");
    }
    for expected in [
        WindowClassToken::CompactDesktop.as_str(),
        WindowClassToken::StandardDesktop.as_str(),
        WindowClassToken::ExpandedDesktop.as_str(),
    ] {
        assert!(window_classes.contains(expected), "window class {expected} not exercised");
    }
    for expected in [
        LandingRouteClass::PriorActiveEditor.as_str(),
        LandingRouteClass::ChangedFilesView.as_str(),
        LandingRouteClass::Readme.as_str(),
        LandingRouteClass::ReviewPacket.as_str(),
        LandingRouteClass::PostEntryHandoffCard.as_str(),
    ] {
        assert!(routes.contains(expected), "landing route {expected} not exercised");
    }
    for expected in [
        SideEffectfulSurfaceClass::Terminal.as_str(),
        SideEffectfulSurfaceClass::Task.as_str(),
        SideEffectfulSurfaceClass::DebugSession.as_str(),
        SideEffectfulSurfaceClass::NotebookCell.as_str(),
        SideEffectfulSurfaceClass::ProviderMutation.as_str(),
        SideEffectfulSurfaceClass::CollaborationControl.as_str(),
        SideEffectfulSurfaceClass::RemoteAction.as_str(),
    ] {
        assert!(surfaces.contains(expected), "side-effectful surface {expected} not exercised");
    }
    for expected in [
        CollapseTargetClass::Sheet.as_str(),
        CollapseTargetClass::Overlay.as_str(),
        CollapseTargetClass::Overflow.as_str(),
    ] {
        assert!(collapse_targets.contains(expected), "collapse target {expected} not exercised");
    }
    for expected in [
        ZoneOwnedCueClass::Breadcrumb.as_str(),
        ZoneOwnedCueClass::TrustBadge.as_str(),
        ZoneOwnedCueClass::ExecutionTargetCue.as_str(),
        ZoneOwnedCueClass::WorkspaceIdentity.as_str(),
        ZoneOwnedCueClass::StatusSummary.as_str(),
    ] {
        assert!(cues.contains(expected), "zone-owned cue {expected} not exercised");
    }
    for expected in [
        RestoreProvenanceClass::RestoredExactly.as_str(),
        RestoreProvenanceClass::RestoredPartially.as_str(),
        RestoreProvenanceClass::NeedsReview.as_str(),
    ] {
        assert!(provenance.contains(expected), "provenance class {expected} not exercised");
    }
    // ShellZoneToken is referenced by collapsed-surface source zones; ensure the
    // re-projected zone vocabulary stays wired.
    let _ = ShellZoneToken::MainWorkspace.as_str();
}

#[test]
fn fixture_directory_has_no_unexpected_files() {
    let scenario_files: std::collections::BTreeSet<String> = warm_continuity_corpus()
        .iter()
        .map(|s| s.fixture_filename.to_owned())
        .collect();
    let mut on_disk = std::collections::BTreeSet::new();
    for entry in std::fs::read_dir(FIXTURE_DIR).expect("read fixture dir") {
        let entry = entry.expect("dir entry");
        let name = entry.file_name().to_string_lossy().into_owned();
        if name.ends_with(".json") {
            on_disk.insert(name);
        }
    }
    assert_eq!(
        scenario_files, on_disk,
        "fixture directory drifted from the corpus; re-emit fixtures",
    );
}
