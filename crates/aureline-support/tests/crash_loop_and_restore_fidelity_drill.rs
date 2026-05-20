//! Evidence-first drill harness for the crash-loop and restore-fidelity corpus.
//!
//! Every drill is replayed through the real crash-loop recovery center or
//! restore hydrator. The harness asserts the observed recovery class, the
//! no-silent-rerun posture for protected lanes, truthful placeholders for
//! missing dependencies, the off-screen remap, and accessibility — then
//! golden-checks the published recovery-choice matrix.

use std::collections::BTreeSet;
use std::path::{Path, PathBuf};

use aureline_support::crash_loop_and_restore_fidelity::{
    current_corpus, load_drill, CrashLoopRestoreFidelityCorpus, DrillConditionClass, DrillKind,
    RecoveryOutcomeClass, CORPUS_DOC_REF, CORPUS_MANIFEST_REF, CORPUS_REPORT_REF,
    RECOVERY_CHOICE_MATRIX_REF,
};
use aureline_support::crash_loop_center::{load_signal, CrashLoopRecoveryCenterBeta, RestoreClass};
use aureline_workspace::{
    PlaceholderActionClass, PlaceholderReasonClass, RestoreHydrationOutcome,
    RestoreHydrationRequest, RestoreLevel, RestoreNoRerunGuardrail, RestoreSurfaceRestorePosture,
};

const MATRIX_ID: &str = "crash_loop_restore.recovery_choice_matrix.v1";
const GENERATED_AT: &str = "2026-05-20T00:00:00Z";

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(Path::parent)
        .expect("derive repo root")
        .to_path_buf()
}

fn corpus() -> CrashLoopRestoreFidelityCorpus {
    current_corpus().expect("corpus parses")
}

/// Stable snake-case token for any unit enum variant.
fn token<T: serde::Serialize>(value: &T) -> String {
    serde_json::to_value(value)
        .expect("serialize token")
        .as_str()
        .expect("token is a string")
        .to_string()
}

fn is_command_bearing(token: &str) -> bool {
    matches!(
        token,
        "terminal"
            | "remote_shell"
            | "task_runner"
            | "pipeline_view"
            | "preview_runtime"
            | "notebook"
            | "debug_session"
    )
}

fn crash_outcome(class: RestoreClass) -> RecoveryOutcomeClass {
    match class {
        RestoreClass::ExactRestore => RecoveryOutcomeClass::ExactRestore,
        RestoreClass::CompatibleRestore => RecoveryOutcomeClass::CompatibleRestore,
        RestoreClass::LayoutOnly => RecoveryOutcomeClass::LayoutOnly,
        RestoreClass::EvidenceOnly => RecoveryOutcomeClass::EvidenceOnly,
        RestoreClass::NoRestoreAttempted => RecoveryOutcomeClass::NoRestore,
    }
}

fn restore_outcome(level: RestoreLevel) -> RecoveryOutcomeClass {
    match level {
        RestoreLevel::ExactRestore => RecoveryOutcomeClass::ExactRestore,
        RestoreLevel::CompatibleRestore => RecoveryOutcomeClass::CompatibleRestore,
        RestoreLevel::LayoutOnly => RecoveryOutcomeClass::LayoutOnly,
        RestoreLevel::RecoveredDrafts | RestoreLevel::EvidenceOnly => {
            RecoveryOutcomeClass::EvidenceOnly
        }
        RestoreLevel::NoRestore => RecoveryOutcomeClass::NoRestore,
    }
}

fn read_request(input_fixture_ref: &str) -> RestoreHydrationRequest {
    let path = repo_root().join(input_fixture_ref);
    let payload = std::fs::read_to_string(&path)
        .unwrap_or_else(|err| panic!("read {}: {err}", path.display()));
    serde_json::from_str(&payload).unwrap_or_else(|err| panic!("parse {}: {err}", path.display()))
}

fn hydrate(input_fixture_ref: &str) -> RestoreHydrationOutcome {
    read_request(input_fixture_ref)
        .hydrate()
        .unwrap_or_else(|err| panic!("hydrate {input_fixture_ref}: {err}"))
}

/// Asserts at least one declared recovery-path token appears in the observed set.
fn assert_path_overlap(declared: &[String], observed: &BTreeSet<String>, ctx: &str) {
    assert!(
        declared.iter().any(|path| observed.contains(path)),
        "{ctx}: declared recovery_paths {declared:?} share nothing with observed {observed:?}"
    );
}

// ---------------------------------------------------------------------------
// Corpus shape and references
// ---------------------------------------------------------------------------

#[test]
fn corpus_validates_and_covers_every_required_condition() {
    let corpus = corpus();
    let violations = corpus.validate();
    assert_eq!(violations, Vec::new(), "{violations:#?}");

    let conditions = corpus
        .drills()
        .map(|drill| drill.condition_class)
        .collect::<BTreeSet<_>>();
    let expected = [
        DrillConditionClass::ExtensionHostCrashLoop,
        DrillConditionClass::BadLayoutRestore,
        DrillConditionClass::MissingExtensionPanes,
        DrillConditionClass::UnavailableRemoteSessions,
        DrillConditionClass::OffScreenWindowRemap,
        DrillConditionClass::CheckpointOnlyRecovery,
        DrillConditionClass::NoSilentRerunProtectedLanes,
    ]
    .into_iter()
    .collect::<BTreeSet<_>>();
    assert_eq!(conditions, expected);
    assert_eq!(corpus.entries.len(), expected.len());
}

#[test]
fn every_drill_pins_repo_relative_refs_that_exist() {
    let corpus = corpus();
    let root = repo_root();
    for entry in &corpus.entries {
        let drill = &entry.drill;
        let refs = &drill.source_refs;
        for (label, value) in [
            ("doc_ref", &refs.doc_ref),
            ("schema_ref", &refs.schema_ref),
            ("crate_consumer", &refs.crate_consumer),
            ("integration_test", &refs.integration_test),
            ("input_fixture_ref", &refs.input_fixture_ref),
        ] {
            assert!(
                root.join(value).is_file(),
                "{}: {label} {value} is missing",
                drill.drill_id
            );
        }
        assert!(
            root.join(&entry.fixture_ref).is_file(),
            "{}: drill fixture {} missing",
            drill.drill_id,
            entry.fixture_ref
        );
        // The declared drill kind matches the input fixture format.
        match drill.drill_kind {
            DrillKind::CrashLoopCenter => {
                assert!(refs.input_fixture_ref.ends_with(".yaml"))
            }
            DrillKind::RestoreFidelity => {
                assert!(refs.input_fixture_ref.ends_with(".json"))
            }
        }
    }
}

#[test]
fn yaml_round_trip_load_matches_corpus_entry() {
    let corpus = corpus();
    let root = repo_root();
    for entry in &corpus.entries {
        let yaml = std::fs::read_to_string(root.join(&entry.fixture_ref))
            .unwrap_or_else(|err| panic!("read {}: {err}", entry.fixture_ref));
        let drill =
            load_drill(&yaml).unwrap_or_else(|err| panic!("parse {}: {err}", entry.fixture_ref));
        assert_eq!(drill, entry.drill);
    }
}

// ---------------------------------------------------------------------------
// Crash-loop center drills (evidence-first replay)
// ---------------------------------------------------------------------------

#[test]
fn crash_loop_drills_route_into_bounded_centers_at_the_declared_class() {
    let corpus = corpus();
    let beta = CrashLoopRecoveryCenterBeta::new();
    let root = repo_root();

    for drill in corpus
        .drills()
        .filter(|drill| drill.drill_kind == DrillKind::CrashLoopCenter)
    {
        let yaml = std::fs::read_to_string(root.join(&drill.source_refs.input_fixture_ref))
            .unwrap_or_else(|err| panic!("read signal: {err}"));
        let signal = load_signal(&yaml).expect("signal parses");
        let center = beta
            .evaluate(&signal)
            .unwrap_or_else(|err| panic!("{} evaluate: {err}", drill.drill_id));

        // Repeated crashes route into a visible, bounded surface.
        assert!(center.is_bounded_recovery_surface(), "{}", drill.drill_id);
        assert!(center.silent_restart_suppressed);
        assert!(!center.crash_id.is_empty());
        assert!(!center.build_id.is_empty());

        // The observed recovery (restore) class matches the claimed class.
        assert_eq!(
            crash_outcome(center.restore_class),
            drill.expected_recovery_outcome,
            "{} recovery class regressed",
            drill.drill_id
        );

        // Accessibility: the center is keyboard-complete and every choice is
        // screen-reader labeled.
        assert!(center.accessibility.keyboard_complete);
        assert!(!center.accessibility.screen_reader_summary.is_empty());
        for choice in &center.recovery_choices {
            assert!(choice.accessibility.keyboard_complete);
            assert!(!choice.accessibility.screen_reader_label.is_empty());
        }

        // No-silent-rerun: every session re-entry choice pins no_silent_rerun.
        for choice in &center.recovery_choices {
            if choice.choice_class.is_session_reentry() {
                assert!(
                    choice.command.no_silent_rerun && choice.command.requires_explicit_confirmation,
                    "{} re-entry choice would replay silently",
                    drill.drill_id
                );
            }
        }

        // The publishable evidence packet is metadata-safe.
        let packet = beta.support_packet(format!("packet:{}", drill.drill_id), &center);
        assert!(
            packet.is_export_safe(),
            "{} packet not export-safe",
            drill.drill_id
        );
        assert!(!packet.render_support_summary().is_empty());

        // Declared recovery paths overlap the live choices and evidence entries.
        let mut observed = BTreeSet::new();
        for choice in &center.recovery_choices {
            observed.insert(choice.choice_class.as_str().to_string());
        }
        for entry in &center.evidence_entry_points {
            observed.insert(entry.entry_class.as_str().to_string());
        }
        assert_path_overlap(&drill.recovery_paths, &observed, &drill.drill_id);
    }
}

// ---------------------------------------------------------------------------
// Restore-fidelity drills (evidence-first replay)
// ---------------------------------------------------------------------------

#[test]
fn restore_drills_recover_at_the_declared_class_without_silent_replay() {
    let corpus = corpus();
    for drill in corpus
        .drills()
        .filter(|drill| drill.drill_kind == DrillKind::RestoreFidelity)
    {
        let outcome = hydrate(&drill.source_refs.input_fixture_ref);

        // Aggregate recovery class matches the claim.
        assert_eq!(
            restore_outcome(outcome.summary.aggregate_restore_level),
            drill.expected_recovery_outcome,
            "{} aggregate recovery class regressed",
            drill.drill_id
        );

        for window in &outcome.windows {
            // Skeleton-first: the shell and every pane slot survive.
            assert!(window.shell_restored, "{}", drill.drill_id);
            assert!(!window.preserved_pane_ids.is_empty());

            for live in &window.provenance.live_surface_outcomes {
                // No protected session is replayed silently.
                assert!(
                    live.no_rerun_guardrails
                        .contains(&RestoreNoRerunGuardrail::ExplicitUserActionRequired),
                    "{} pane {} would resume without explicit action",
                    drill.drill_id,
                    live.pane_id
                );
                let lane = token(&live.live_surface_class);
                if is_command_bearing(&lane) {
                    assert!(
                        live.no_rerun_guardrails
                            .contains(&RestoreNoRerunGuardrail::NoCommandRerun),
                        "{} command lane {lane} lacks no-command-rerun",
                        drill.drill_id
                    );
                }
            }
        }
    }
}

#[test]
fn missing_dependency_surfaces_reopen_as_truthful_placeholders() {
    let corpus = corpus();

    // Missing extension pane.
    let ext = corpus
        .drill_for(DrillConditionClass::MissingExtensionPanes)
        .expect("missing-extension drill");
    let ext_outcome = hydrate(&ext.source_refs.input_fixture_ref);
    let ext_placeholder = ext_outcome
        .windows
        .iter()
        .flat_map(|window| window.provenance.placeholder_results.iter())
        .find(|placeholder| {
            placeholder.placeholder_reason == PlaceholderReasonClass::MissingExtension
        })
        .expect("a missing-extension placeholder reopened");
    assert!(ext_placeholder
        .safe_actions
        .contains(&PlaceholderActionClass::InstallExtension));
    assert!(ext_placeholder.last_known_provenance_label.is_some());
    let ext_actions = ext_placeholder
        .safe_actions
        .iter()
        .map(token)
        .collect::<BTreeSet<_>>();
    assert_path_overlap(&ext.recovery_paths, &ext_actions, &ext.drill_id);

    // Unavailable remote session.
    let remote = corpus
        .drill_for(DrillConditionClass::UnavailableRemoteSessions)
        .expect("unavailable-remote drill");
    let remote_outcome = hydrate(&remote.source_refs.input_fixture_ref);
    let remote_placeholder = remote_outcome
        .windows
        .iter()
        .flat_map(|window| window.provenance.placeholder_results.iter())
        .find(|placeholder| placeholder.placeholder_reason == PlaceholderReasonClass::MissingRemote)
        .expect("a missing-remote placeholder reopened");
    assert!(remote_placeholder
        .safe_actions
        .contains(&PlaceholderActionClass::ReconnectRemote));
    let remote_actions = remote_placeholder
        .safe_actions
        .iter()
        .map(token)
        .collect::<BTreeSet<_>>();
    assert_path_overlap(&remote.recovery_paths, &remote_actions, &remote.drill_id);

    // No placeholder ever masquerades as a ready, live surface.
    for outcome in [&ext_outcome, &remote_outcome] {
        for window in &outcome.windows {
            for live in &window.provenance.live_surface_outcomes {
                assert_ne!(
                    live.restore_posture,
                    RestoreSurfaceRestorePosture::LiveAttachVisible
                );
            }
        }
    }
}

#[test]
fn bad_layout_restore_recovers_the_shell_and_topology() {
    let corpus = corpus();
    let drill = corpus
        .drill_for(DrillConditionClass::BadLayoutRestore)
        .expect("bad-layout drill");
    let outcome = hydrate(&drill.source_refs.input_fixture_ref);
    assert_eq!(
        restore_outcome(outcome.summary.aggregate_restore_level),
        RecoveryOutcomeClass::LayoutOnly
    );
    // The layout shell and all pane slots are preserved even though heavy
    // surfaces degraded to placeholders.
    let window = &outcome.windows[0];
    assert!(window.shell_restored);
    assert!(window.preserved_pane_ids.len() >= 3);
    assert!(!window.provenance.placeholder_results.is_empty());
}

#[test]
fn offscreen_remap_applies_safe_bounds_without_trapping_windows() {
    let corpus = corpus();
    let drill = corpus
        .drill_for(DrillConditionClass::OffScreenWindowRemap)
        .expect("offscreen drill");
    let outcome = hydrate(&drill.source_refs.input_fixture_ref);
    assert_eq!(
        restore_outcome(outcome.summary.aggregate_restore_level),
        RecoveryOutcomeClass::CompatibleRestore
    );

    let display = (0i64, 0i64, 1440i64, 900i64);
    let mut observed_adjustments = BTreeSet::new();
    let mut saw_exact = false;
    let mut saw_compatible = false;
    for window in &outcome.windows {
        let bounds = &window.applied_bounds;
        assert!(bounds.width > 0 && bounds.height > 0);
        assert!(
            bounds.x >= display.0
                && bounds.y >= display.1
                && bounds.x + bounds.width <= display.0 + display.2
                && bounds.y + bounds.height <= display.1 + display.3,
            "{} window {} escaped the display",
            drill.drill_id,
            window.window_id
        );
        match window.provenance.restore_level {
            RestoreLevel::ExactRestore => saw_exact = true,
            RestoreLevel::CompatibleRestore => saw_compatible = true,
            _ => {}
        }
        for adjustment in &window.provenance.display_adjustments {
            assert!(!adjustment.affected_pane_ids.is_empty());
            observed_adjustments.insert(token(&adjustment.adjustment_class));
        }
    }
    // The drill demonstrates both an exact and a compatible window restore.
    assert!(saw_exact, "offscreen aux window should restore exactly");
    assert!(
        saw_compatible,
        "offscreen primary window should restore compatibly"
    );
    assert_path_overlap(
        &drill.recovery_paths,
        &observed_adjustments,
        &drill.drill_id,
    );
}

#[test]
fn protected_lanes_never_replay_silently() {
    let corpus = corpus();
    let drill = corpus
        .drill_for(DrillConditionClass::NoSilentRerunProtectedLanes)
        .expect("no-silent-rerun drill");
    let outcome = hydrate(&drill.source_refs.input_fixture_ref);
    assert_eq!(
        restore_outcome(outcome.summary.aggregate_restore_level),
        RecoveryOutcomeClass::EvidenceOnly
    );

    let observed_lanes = outcome
        .windows
        .iter()
        .flat_map(|window| window.provenance.live_surface_outcomes.iter())
        .map(|live| token(&live.live_surface_class))
        .collect::<BTreeSet<_>>();

    // Every declared protected lane is actually exercised.
    for lane in &drill.protected_lanes {
        assert!(
            observed_lanes.contains(lane.live_surface_class_token()),
            "{} did not exercise protected lane {}",
            drill.drill_id,
            lane.as_str()
        );
    }

    // No live surface re-runs, none masquerade as ready, command lanes never
    // re-run commands, and authority is never reacquired silently.
    let mut live_outcomes = 0;
    for window in &outcome.windows {
        for live in &window.provenance.live_surface_outcomes {
            live_outcomes += 1;
            assert_ne!(
                live.restore_posture,
                RestoreSurfaceRestorePosture::LiveAttachVisible
            );
            assert!(live
                .no_rerun_guardrails
                .contains(&RestoreNoRerunGuardrail::ExplicitUserActionRequired));
            assert!(live
                .no_rerun_guardrails
                .contains(&RestoreNoRerunGuardrail::NoHiddenAuthorityReacquire));
            let lane = token(&live.live_surface_class);
            if is_command_bearing(&lane) {
                assert!(live
                    .no_rerun_guardrails
                    .contains(&RestoreNoRerunGuardrail::NoCommandRerun));
            }
        }
    }
    assert!(live_outcomes >= drill.protected_lanes.len());
}

// ---------------------------------------------------------------------------
// Published recovery-choice matrix (golden artifact)
// ---------------------------------------------------------------------------

#[test]
fn recovery_choice_matrix_matches_checked_in_artifact() {
    let corpus = corpus();
    let matrix = corpus.recovery_choice_matrix(MATRIX_ID, GENERATED_AT);

    // The matrix shows the exact / compatible / layout-only / evidence-only
    // recovery spectrum and stays metadata-safe.
    assert!(matrix.is_export_safe());
    for required in [
        "exact_restore",
        "compatible_restore",
        "layout_only",
        "evidence_only",
    ] {
        assert!(
            matrix
                .observed_recovery_outcome_classes
                .iter()
                .any(|outcome| outcome == required),
            "matrix is missing the {required} recovery outcome"
        );
    }
    assert_eq!(matrix.rows.len(), corpus.entries.len());

    let generated = serde_json::to_value(&matrix).expect("serialize matrix");
    let path = repo_root().join(RECOVERY_CHOICE_MATRIX_REF);
    let on_disk = std::fs::read_to_string(&path)
        .unwrap_or_else(|err| panic!("read {}: {err}", path.display()));
    let parsed: serde_json::Value = serde_json::from_str(&on_disk)
        .unwrap_or_else(|err| panic!("parse {}: {err}", path.display()));
    assert_eq!(
        parsed, generated,
        "checked-in recovery_choice_matrix.json is stale; regenerate it from the corpus"
    );
}

#[test]
fn corpus_doc_report_and_matrix_exist_at_declared_paths() {
    let root = repo_root();
    for required in [
        CORPUS_MANIFEST_REF,
        CORPUS_DOC_REF,
        CORPUS_REPORT_REF,
        RECOVERY_CHOICE_MATRIX_REF,
    ] {
        assert!(root.join(required).is_file(), "{required} missing");
    }
}

// ---------------------------------------------------------------------------
// Negative: the validator refuses a regressed corpus
// ---------------------------------------------------------------------------

#[test]
fn validator_refuses_a_drill_kind_that_does_not_match_its_condition() {
    let mut corpus = corpus();
    for entry in &mut corpus.entries {
        if entry.drill.condition_class == DrillConditionClass::BadLayoutRestore {
            entry.drill.drill_kind = DrillKind::CrashLoopCenter;
        }
    }
    let violations = corpus.validate();
    assert!(violations
        .iter()
        .any(|violation| violation.check_id == "drill.kind_mismatch"));
}

#[test]
fn validator_refuses_dropping_a_required_condition() {
    let mut corpus = corpus();
    corpus
        .entries
        .retain(|entry| entry.drill.condition_class != DrillConditionClass::CheckpointOnlyRecovery);
    let violations = corpus.validate();
    assert!(violations.iter().any(|violation| {
        violation.check_id == "corpus.required_condition_missing"
            && violation.target_ref == "checkpoint_only_recovery"
    }));
}

#[test]
fn validator_refuses_weakening_no_silent_rerun_on_a_protected_lane() {
    let mut corpus = corpus();
    for entry in &mut corpus.entries {
        if entry.drill.condition_class == DrillConditionClass::NoSilentRerunProtectedLanes {
            entry.drill.no_silent_rerun_required = false;
        }
    }
    let violations = corpus.validate();
    assert!(violations
        .iter()
        .any(|violation| violation.check_id == "drill.no_silent_rerun_required"));
}
