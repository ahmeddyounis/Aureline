//! Fixture-replay and invariant tests for the first-run onboarding drill corpus.
//!
//! The records live under
//! `fixtures/ux/m4/finalize-first-run-onboarding-with-no-account-local/` and are
//! minted by the `aureline_shell_first_run_onboarding` emitter so the checked-in
//! JSON stays a literal projection of the in-code corpus.
//!
//! What this guards (the acceptance criteria for this lane):
//!
//! - Each scenario's record on disk matches the in-code projection bit-for-bit.
//!   Regenerate with:
//!
//!   ```sh
//!   cargo run -q -p aureline-shell \
//!     --bin aureline_shell_first_run_onboarding -- emit-fixtures \
//!     fixtures/ux/m4/finalize-first-run-onboarding-with-no-account-local
//!   ```
//!
//! - No-account local entry: local work is available with no account and no
//!   managed service, and at least one account-free entry verb is offered.
//! - Setup-later posture: no setup step blocks first-useful-work, and an
//!   outstanding (offered / deferred) step never widens trust, installs packages,
//!   applies a bundle, or suppresses a checkpoint; each step keeps a resume route.
//! - Repair-safe recovery: every cue preserves user work, never dead-ends, never
//!   silently resets, routes through metadata_safe_default, and carries an
//!   export-safe chain of custody (doctor.finding.* code, repair_transaction:*
//!   id, opaque checkpoint ref). Healthy records have no cues; non-healthy
//!   records have at least one.
//! - Durable, accessible truth: keyboard-complete, focus-ordered, high-contrast
//!   and zoom reachable, with the Start Center entry surface present and every
//!   surface keyboard-reachable.
//! - The first-useful-work landing is keyboard-reachable, non-destructive, and
//!   account-free.

use aureline_shell::first_run_onboarding::{
    first_run_onboarding_corpus, is_canonical_object_ref, is_doctor_finding_code, is_opaque_id,
    is_repair_transaction_ref, EntrySurfaceClass, EntryVerbClass, FirstRunHealthClass,
    FirstRunOnboardingRecord, FirstRunResourceClass, FirstRunScenarioClass,
    FirstUsefulWorkLandingClass, SetupStepClass, SetupStepPosture, REQUIRED_REDACTION_CLASS,
};

const FIXTURE_DIR: &str = concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../fixtures/ux/m4/finalize-first-run-onboarding-with-no-account-local",
);

fn load_record(filename: &str) -> FirstRunOnboardingRecord {
    let path = format!("{FIXTURE_DIR}/{filename}");
    let body =
        std::fs::read_to_string(&path).unwrap_or_else(|err| panic!("failed to read {path}: {err}"));
    serde_json::from_str(&body).unwrap_or_else(|err| panic!("failed to parse {path}: {err}"))
}

#[test]
fn every_scenario_fixture_matches_in_code_projection() {
    for scenario in first_run_onboarding_corpus() {
        let on_disk = load_record(scenario.fixture_filename);
        let in_code = scenario.record();
        assert_eq!(
            on_disk, in_code,
            "{} fixture drifted; re-emit with `cargo run -q -p aureline-shell --bin aureline_shell_first_run_onboarding -- emit-fixtures fixtures/ux/m4/finalize-first-run-onboarding-with-no-account-local`",
            scenario.fixture_filename,
        );
    }
}

#[test]
fn pinned_rollups_match_each_scenario() {
    for scenario in first_run_onboarding_corpus() {
        let record = load_record(scenario.fixture_filename);
        assert_eq!(
            record.scenario, scenario.expected_scenario,
            "{} scenario",
            scenario.scenario_id
        );
        assert_eq!(
            record.health, scenario.expected_health,
            "{} health",
            scenario.scenario_id
        );
        assert_eq!(
            record.landing.landing, scenario.expected_landing,
            "{} landing",
            scenario.scenario_id,
        );
        assert_eq!(
            record.honesty_marker_present, scenario.expected_honesty_marker_present,
            "{} honesty_marker_present",
            scenario.scenario_id,
        );
        assert_eq!(
            record.summary_counts.deferred_step_count, scenario.expected_deferred_step_count,
            "{} deferred_step_count",
            scenario.scenario_id,
        );
        assert_eq!(
            record.summary_counts.repair_cue_count, scenario.expected_repair_cue_count,
            "{} repair_cue_count",
            scenario.scenario_id,
        );
    }
}

#[test]
fn local_entry_is_account_free() {
    for scenario in first_run_onboarding_corpus() {
        let record = load_record(scenario.fixture_filename);
        assert!(
            !record.entry.account_required_for_local_work,
            "{}: local work claimed to need an account",
            scenario.scenario_id,
        );
        assert!(
            !record.entry.managed_services_required_for_local_work,
            "{}: local work claimed to need a managed service",
            scenario.scenario_id,
        );
        assert!(
            record.entry.local_work_available,
            "{}: local work unavailable",
            scenario.scenario_id
        );
        assert!(
            !record.entry.entry_verbs.is_empty(),
            "{}: no account-free entry verb",
            scenario.scenario_id,
        );
        assert!(
            !record.display_copy.account_implied_for_local_work,
            "{}",
            scenario.scenario_id
        );
        assert!(
            !record.display_copy.managed_services_implied_for_local_work,
            "{}",
            scenario.scenario_id,
        );
    }
}

#[test]
fn setup_steps_are_deferrable_and_bounded() {
    for scenario in first_run_onboarding_corpus() {
        let record = load_record(scenario.fixture_filename);
        for step in &record.setup_steps {
            assert!(
                !step.blocks_first_useful_work,
                "{}: step {} blocked first-useful-work",
                scenario.scenario_id,
                step.step.as_str(),
            );
            assert!(
                is_canonical_object_ref(&step.resume_route_ref),
                "{}: step {} resume route {:?} not canonical",
                scenario.scenario_id,
                step.step.as_str(),
                step.resume_route_ref,
            );
            if step.posture.is_outstanding() {
                assert!(
                    !step.widens_trust_on_defer,
                    "{}: step widened trust on defer",
                    scenario.scenario_id
                );
                assert!(
                    !step.installs_packages_on_defer,
                    "{}: step installed packages on defer",
                    scenario.scenario_id
                );
                assert!(
                    !step.applies_workflow_bundle_on_defer,
                    "{}: step applied a bundle on defer",
                    scenario.scenario_id
                );
                assert!(
                    !step.suppresses_required_checkpoint_on_defer,
                    "{}: step suppressed a checkpoint on defer",
                    scenario.scenario_id,
                );
            }
        }
        assert!(
            !record.display_copy.setup_blocked_first_useful_work,
            "{}",
            scenario.scenario_id
        );
        assert!(
            !record.display_copy.setup_deferral_widened_trust,
            "{}",
            scenario.scenario_id
        );
    }
}

#[test]
fn repair_cues_are_repair_safe_with_chain_of_custody() {
    for scenario in first_run_onboarding_corpus() {
        let record = load_record(scenario.fixture_filename);
        match record.health {
            FirstRunHealthClass::Healthy => assert!(
                record.repair_cues.is_empty(),
                "{}: healthy record surfaced a repair cue",
                scenario.scenario_id,
            ),
            FirstRunHealthClass::Degraded | FirstRunHealthClass::NeedsRepair => assert!(
                !record.repair_cues.is_empty(),
                "{}: non-healthy record surfaced no repair cue",
                scenario.scenario_id,
            ),
        }
        for cue in &record.repair_cues {
            assert!(
                cue.preserves_user_work,
                "{}: repair destroyed user work",
                scenario.scenario_id
            );
            assert!(!cue.dead_end, "{}: repair dead-ended", scenario.scenario_id);
            assert!(
                !cue.silent_reset,
                "{}: repair silently reset",
                scenario.scenario_id
            );
            assert_eq!(
                cue.redaction_class, REQUIRED_REDACTION_CLASS,
                "{}: repair redaction not metadata_safe_default",
                scenario.scenario_id,
            );
            // Export-safe chain of custody.
            assert!(
                is_doctor_finding_code(&cue.finding_code),
                "{}: finding code {:?} not canonical",
                scenario.scenario_id,
                cue.finding_code,
            );
            assert!(
                is_repair_transaction_ref(&cue.repair_transaction_ref),
                "{}: repair transaction ref {:?} not canonical",
                scenario.scenario_id,
                cue.repair_transaction_ref,
            );
            assert!(
                is_opaque_id(&cue.checkpoint_ref),
                "{}: checkpoint ref {:?} not opaque/export-safe",
                scenario.scenario_id,
                cue.checkpoint_ref,
            );
            assert!(
                is_canonical_object_ref(&cue.repair_route_ref),
                "{}: repair route {:?} not canonical",
                scenario.scenario_id,
                cue.repair_route_ref,
            );
        }
        assert_eq!(
            record.repair_cues.len() as u32,
            record.summary_counts.repair_cue_count,
            "{}: repair cue count drift",
            scenario.scenario_id,
        );
        assert!(
            !record.display_copy.repair_destroyed_user_work,
            "{}",
            scenario.scenario_id
        );
        assert!(
            !record.display_copy.repair_dead_ended,
            "{}",
            scenario.scenario_id
        );
        assert!(
            !record.display_copy.repair_silently_reset,
            "{}",
            scenario.scenario_id
        );
    }
}

#[test]
fn truth_is_durable_and_accessible_not_toast_or_theme_only() {
    for scenario in first_run_onboarding_corpus() {
        let record = load_record(scenario.fixture_filename);
        let a11y = &record.accessibility;
        assert!(
            a11y.keyboard_complete,
            "{}: not keyboard complete",
            scenario.scenario_id
        );
        assert!(
            a11y.focus_order_defined,
            "{}: no focus order",
            scenario.scenario_id
        );
        assert!(
            a11y.high_contrast_reachable,
            "{}: not high-contrast reachable",
            scenario.scenario_id
        );
        assert!(
            a11y.zoom_reachable,
            "{}: not zoom reachable",
            scenario.scenario_id
        );
        assert!(
            a11y.entry_surfaces
                .iter()
                .any(|s| s.surface == EntrySurfaceClass::StartCenter),
            "{}: missing Start Center entry surface",
            scenario.scenario_id,
        );
        for surface in &a11y.entry_surfaces {
            assert!(
                surface.keyboard_reachable,
                "{}: entry surface not keyboard reachable",
                scenario.scenario_id
            );
            assert!(
                is_canonical_object_ref(&surface.route_ref),
                "{}: entry surface route {:?} not canonical",
                scenario.scenario_id,
                surface.route_ref,
            );
        }
        assert!(
            !record.display_copy.toast_only_truth,
            "{}",
            scenario.scenario_id
        );
        assert!(
            !record.display_copy.theme_only_semantics,
            "{}",
            scenario.scenario_id
        );
    }
}

#[test]
fn first_useful_work_landing_is_account_free_and_reachable() {
    for scenario in first_run_onboarding_corpus() {
        let record = load_record(scenario.fixture_filename);
        let landing = &record.landing;
        assert!(
            landing.keyboard_reachable,
            "{}: landing not keyboard reachable",
            scenario.scenario_id
        );
        assert!(
            !landing.destructive,
            "{}: landing is destructive",
            scenario.scenario_id
        );
        assert!(
            !landing.requires_account,
            "{}: landing requires an account",
            scenario.scenario_id
        );
        assert!(
            is_canonical_object_ref(&landing.target_ref),
            "{}: landing target {:?} not canonical",
            scenario.scenario_id,
            landing.target_ref,
        );
    }
}

#[test]
fn first_run_truth_is_inspectable_in_support_export() {
    for scenario in first_run_onboarding_corpus() {
        let record = load_record(scenario.fixture_filename);
        let lines = record.support_export_lines().join("\n");
        assert!(
            lines.contains(&format!("health: {}", record.health.as_str())),
            "{}: support export omits health",
            scenario.scenario_id,
        );
        assert!(
            lines.contains("account_required=false"),
            "{}: support export omits the no-account claim",
            scenario.scenario_id,
        );
        for cue in &record.repair_cues {
            assert!(
                lines.contains(&cue.finding_code),
                "{}: support export omits finding {}",
                scenario.scenario_id,
                cue.finding_code,
            );
            assert!(
                lines.contains(&cue.repair_transaction_ref),
                "{}: support export omits repair transaction {}",
                scenario.scenario_id,
                cue.repair_transaction_ref,
            );
        }
        // Export is redaction-safe: no raw scheme leakage beyond canonical refs.
        assert!(
            !lines.contains("http://") && !lines.contains("https://"),
            "{}",
            scenario.scenario_id
        );
    }
}

#[test]
fn corpus_covers_required_cases_and_vocabularies() {
    let mut scenarios = std::collections::BTreeSet::new();
    let mut healths = std::collections::BTreeSet::new();
    let mut verbs = std::collections::BTreeSet::new();
    let mut steps = std::collections::BTreeSet::new();
    let mut postures = std::collections::BTreeSet::new();
    let mut resources = std::collections::BTreeSet::new();
    let mut surfaces = std::collections::BTreeSet::new();
    let mut landings = std::collections::BTreeSet::new();
    for scenario in first_run_onboarding_corpus() {
        let record = load_record(scenario.fixture_filename);
        scenarios.insert(record.scenario.as_str());
        healths.insert(record.health.as_str());
        for v in &record.entry.entry_verbs {
            verbs.insert(v.as_str());
        }
        for s in &record.setup_steps {
            steps.insert(s.step.as_str());
            postures.insert(s.posture.as_str());
        }
        for c in &record.repair_cues {
            resources.insert(c.resource.as_str());
        }
        for s in &record.accessibility.entry_surfaces {
            surfaces.insert(s.surface.as_str());
        }
        landings.insert(record.landing.landing.as_str());
    }
    for expected in [
        FirstRunScenarioClass::CleanFirstRun.as_str(),
        FirstRunScenarioClass::SetupDeferredLocalOnly.as_str(),
        FirstRunScenarioClass::SetupCompletedWithImport.as_str(),
        FirstRunScenarioClass::DegradedSettingsStore.as_str(),
        FirstRunScenarioClass::NeedsRepairPartialMigration.as_str(),
        FirstRunScenarioClass::MissingLocalePack.as_str(),
        FirstRunScenarioClass::NewerProfileIncompatible.as_str(),
    ] {
        assert!(
            scenarios.contains(expected),
            "scenario {expected} not exercised"
        );
    }
    for expected in [
        FirstRunHealthClass::Healthy.as_str(),
        FirstRunHealthClass::Degraded.as_str(),
        FirstRunHealthClass::NeedsRepair.as_str(),
    ] {
        assert!(
            healths.contains(expected),
            "health {expected} not exercised"
        );
    }
    for expected in EntryVerbClass::ALL {
        assert!(
            verbs.contains(expected.as_str()),
            "entry verb {} not exercised",
            expected.as_str()
        );
    }
    for expected in SetupStepClass::ALL {
        assert!(
            steps.contains(expected.as_str()),
            "setup step {} not exercised",
            expected.as_str()
        );
    }
    for expected in [
        SetupStepPosture::OfferedDeferrable.as_str(),
        SetupStepPosture::Deferred.as_str(),
        SetupStepPosture::CompletedDuringFirstRun.as_str(),
    ] {
        assert!(
            postures.contains(expected),
            "setup posture {expected} not exercised"
        );
    }
    for expected in FirstRunResourceClass::ALL {
        assert!(
            resources.contains(expected.as_str()),
            "repair resource {} not exercised",
            expected.as_str(),
        );
    }
    for expected in EntrySurfaceClass::ALL {
        assert!(
            surfaces.contains(expected.as_str()),
            "entry surface {} not exercised",
            expected.as_str(),
        );
    }
    // Every landing class except start_center is exercised by a scenario;
    // start_center stays wired through the vocabulary.
    for expected in [
        FirstUsefulWorkLandingClass::LocalWorkspace.as_str(),
        FirstUsefulWorkLandingClass::Readme.as_str(),
        FirstUsefulWorkLandingClass::SampleProject.as_str(),
        FirstUsefulWorkLandingClass::EmptyEditor.as_str(),
    ] {
        assert!(
            landings.contains(expected),
            "landing {expected} not exercised"
        );
    }
    let _ = FirstUsefulWorkLandingClass::StartCenter.as_str();
}

#[test]
fn fixture_directory_has_no_unexpected_files() {
    let scenario_files: std::collections::BTreeSet<String> = first_run_onboarding_corpus()
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
