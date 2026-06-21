//! Unit tests for presentation-session restore.

use aureline_recovery::session_restore::records::{DowngradeTriggerClass, RestoreClass};

use super::corpus::{
    presentation_restore_support_export, seeded_presentation_restore_corpus,
    validate_presentation_restore_corpus, PresentationRestoreCorpus,
};
use super::restore::{
    project_evidence_only_report, project_no_restore_report, project_restore_report,
    PresentationRestoreClass, PresentationRestoreLifecycle, PresentationRestoreTrigger,
    PresentationRestoreViolation, RestoreDegradeTrigger, RestoreProjectionInputs,
    WaypointAvailability, WaypointDegrade,
};
use crate::presentation_mode::{
    AudienceScope, BoundaryLabel, FollowWaypoint, LeaderFollowState, PresentationSession,
    PresentationSessionBuilder, RestoreCheckpoint, WalkthroughSurfaceKind, WaypointCompletionState,
};

fn checkpoint() -> RestoreCheckpoint {
    RestoreCheckpoint {
        checkpoint_id: "presentation:checkpoint:restore-unit".into(),
        prior_layout_ref: "window-topology:restore-unit:prior".into(),
        prior_focus_ref: "focus-chain:restore-unit:prior".into(),
        prior_panel_visibility_ref: "panel-visibility:restore-unit:prior".into(),
        accessibility_posture_ref: "a11y-posture:restore-unit:prior".into(),
        captured_at: "2026-06-20T09:00:00Z".into(),
    }
}

fn waypoint(id: &str) -> FollowWaypoint {
    FollowWaypoint {
        waypoint_id: id.into(),
        ordinal: 1,
        step_title: "Anchor".into(),
        surface_kind: WalkthroughSurfaceKind::Editor,
        target_object_ref: format!("obj:{id}"),
        file_path_ref: Some("src/lib.rs".into()),
        symbol_anchor_ref: Some("fn main".into()),
        branch_workspace_ref: "branch:main@workspace:local".into(),
        boundary_label: BoundaryLabel::Shared,
        zoom_layout_hint_ref: None,
        reveal_action_ref: None,
        completion_state: WaypointCompletionState::Current,
        speaker_note: None,
        reuses_existing_surface: true,
        creates_parallel_artifact: false,
    }
}

fn session() -> PresentationSession {
    PresentationSessionBuilder::new(
        "presentation:session:restore-unit",
        LeaderFollowState::Presenting,
        AudienceScope::SharedWorkspace,
        checkpoint(),
    )
    .focus("wp:restore-unit:1")
    .waypoint(waypoint("wp:restore-unit:1"))
    .waypoint(waypoint("wp:restore-unit:2"))
    .build()
}

#[test]
fn trigger_maps_to_its_terminal_lifecycle() {
    assert_eq!(
        PresentationRestoreTrigger::Exit.restored_lifecycle(),
        PresentationRestoreLifecycle::ExitedRestored
    );
    assert_eq!(
        PresentationRestoreTrigger::Cancel.restored_lifecycle(),
        PresentationRestoreLifecycle::CancelledRestored
    );
    assert_eq!(
        PresentationRestoreTrigger::CrashRecovery.restored_lifecycle(),
        PresentationRestoreLifecycle::CrashRecoveredRestored
    );
    assert_eq!(
        PresentationRestoreTrigger::InterruptedResume.restored_lifecycle(),
        PresentationRestoreLifecycle::ResumedRestored
    );
}

#[test]
fn restore_class_maps_onto_the_durable_shell_vocabulary() {
    assert_eq!(
        PresentationRestoreClass::ExactRestore.to_durable_restore_class(),
        RestoreClass::ExactRestore
    );
    assert_eq!(
        PresentationRestoreClass::CompatibleRestore.to_durable_restore_class(),
        RestoreClass::CompatibleRestore
    );
    assert_eq!(
        PresentationRestoreClass::LayoutOnly.to_durable_restore_class(),
        RestoreClass::LayoutOnly
    );
    assert_eq!(
        PresentationRestoreClass::EvidenceOnly.to_durable_restore_class(),
        RestoreClass::EvidenceOnly
    );
    assert_eq!(
        PresentationRestoreClass::NoRestore.to_durable_restore_class(),
        RestoreClass::NoRestore
    );
}

#[test]
fn degrade_trigger_maps_to_durable_downgrade_and_honest_availability() {
    assert_eq!(
        RestoreDegradeTrigger::MissingDependency.to_durable_downgrade_trigger(),
        DowngradeTriggerClass::MissingExtensionDependency
    );
    assert_eq!(
        RestoreDegradeTrigger::UnavailableRemoteTarget.to_durable_downgrade_trigger(),
        DowngradeTriggerClass::MissingRemoteSession
    );
    // A missing dependency degrades to a placeholder; the rest to disconnected.
    assert_eq!(
        RestoreDegradeTrigger::MissingDependency.degraded_availability(),
        Some(WaypointAvailability::Placeholder)
    );
    assert_eq!(
        RestoreDegradeTrigger::RevokedSharingGrant.degraded_availability(),
        Some(WaypointAvailability::Disconnected)
    );
    // Session-scoped triggers degrade the whole session, not a waypoint.
    assert!(!RestoreDegradeTrigger::LiveSessionUnavailable.is_waypoint_scoped());
    assert_eq!(
        RestoreDegradeTrigger::LiveSessionUnavailable.degraded_availability(),
        None
    );
}

#[test]
fn exact_restore_brings_back_the_checkpoint_and_reruns_nothing() {
    let session = session();
    let report = project_restore_report(
        &session,
        &RestoreProjectionInputs::exact(PresentationRestoreTrigger::Exit),
    );
    assert!(report.validate().is_empty(), "{:?}", report.validate());
    assert_eq!(report.restore_class, PresentationRestoreClass::ExactRestore);
    assert!(report.matches_checkpoint);
    assert_eq!(
        report.restored_layout_ref,
        "window-topology:restore-unit:prior"
    );
    assert_eq!(report.restored_waypoint_count(), 2);
    assert!(report.degrade_triggers.is_empty());
    assert!(!report.replayed_any_mutating_action);
    assert!(!report.reacquired_any_authority);
    assert!(!report.left_in_improvised_shell);
}

#[test]
fn compatible_restore_is_labeled_not_claimed_exact() {
    let session = session();
    let report = project_restore_report(
        &session,
        &RestoreProjectionInputs::compatible(PresentationRestoreTrigger::CrashRecovery),
    );
    assert!(report.validate().is_empty(), "{:?}", report.validate());
    assert_eq!(
        report.restore_class,
        PresentationRestoreClass::CompatibleRestore
    );
    assert!(!report.matches_checkpoint);
    assert!(report.compatible_translation_applied);
    // The layout still came back, so the user is not stranded.
    assert!(!report.restored_layout_ref.is_empty());
}

#[test]
fn layout_only_degrades_waypoints_honestly_without_rerun() {
    let session = session();
    let degrades = vec![
        WaypointDegrade::new(
            "wp:restore-unit:1",
            RestoreDegradeTrigger::MissingDependency,
            "placeholder: surface dependency missing",
        ),
        WaypointDegrade::new(
            "wp:restore-unit:2",
            RestoreDegradeTrigger::RevokedSharingGrant,
            "disconnected: sharing grant revoked",
        ),
    ];
    let report = project_restore_report(
        &session,
        &RestoreProjectionInputs::with_degrades(
            PresentationRestoreTrigger::InterruptedResume,
            degrades,
        ),
    );
    assert!(report.validate().is_empty(), "{:?}", report.validate());
    assert_eq!(report.restore_class, PresentationRestoreClass::LayoutOnly);
    assert_eq!(report.placeholder_waypoint_count(), 1);
    assert_eq!(report.disconnected_waypoint_count(), 1);
    assert_eq!(
        report.degrade_triggers,
        vec![
            RestoreDegradeTrigger::MissingDependency,
            RestoreDegradeTrigger::RevokedSharingGrant
        ]
    );
    // The layout still came back — never an improvised shell.
    assert!(!report.left_in_improvised_shell);
    assert!(!report.restored_layout_ref.is_empty());
    // No waypoint re-ran or re-acquired authority.
    for waypoint in &report.waypoint_restores {
        assert!(!waypoint.replayed_mutating_action);
        assert!(!waypoint.reacquired_authority);
        if waypoint.availability.is_degraded() {
            assert!(waypoint.placeholder_label.is_some());
            assert!(waypoint.degrade_trigger.is_some());
        }
    }
}

#[test]
fn evidence_only_keeps_layout_but_not_the_live_walkthrough() {
    let session = session();
    let report = project_evidence_only_report(
        &session,
        PresentationRestoreTrigger::CrashRecovery,
        RestoreDegradeTrigger::LiveSessionUnavailable,
    );
    assert!(report.validate().is_empty(), "{:?}", report.validate());
    assert_eq!(report.restore_class, PresentationRestoreClass::EvidenceOnly);
    assert!(report.waypoint_restores.is_empty());
    assert!(!report.live_session_rehydrated);
    assert_eq!(
        report.session_degrade,
        Some(RestoreDegradeTrigger::LiveSessionUnavailable)
    );
    // The layout still came back from the checkpoint.
    assert!(!report.restored_layout_ref.is_empty());
}

#[test]
fn no_restore_is_honest_and_does_not_strand_the_user() {
    let report = project_no_restore_report(
        "presentation:session:no-restore-unit",
        PresentationRestoreTrigger::InterruptedResume,
    );
    assert!(report.validate().is_empty(), "{:?}", report.validate());
    assert_eq!(report.restore_class, PresentationRestoreClass::NoRestore);
    assert!(report.restored_layout_ref.is_empty());
    assert!(!report.left_in_improvised_shell);
    assert_eq!(
        report.session_degrade,
        Some(RestoreDegradeTrigger::CheckpointUnavailable)
    );
}

#[test]
fn a_report_claiming_exact_with_a_degraded_waypoint_fails_validation() {
    let session = session();
    let mut report = project_restore_report(
        &session,
        &RestoreProjectionInputs::with_degrades(
            PresentationRestoreTrigger::Exit,
            vec![WaypointDegrade::new(
                "wp:restore-unit:1",
                RestoreDegradeTrigger::MissingDependency,
                "placeholder",
            )],
        ),
    );
    // Tamper: claim an exact restore despite a degraded waypoint.
    report.restore_class = PresentationRestoreClass::ExactRestore;
    report.matches_checkpoint = true;
    let violations = report.validate();
    assert!(violations
        .iter()
        .any(|v| matches!(v, PresentationRestoreViolation::RestoreClassMismatch { .. })));
}

#[test]
fn a_replayed_mutating_action_fails_validation() {
    let session = session();
    let mut report = project_restore_report(
        &session,
        &RestoreProjectionInputs::exact(PresentationRestoreTrigger::Exit),
    );
    // Tamper: pretend a waypoint replayed a mutating action during restore.
    report.waypoint_restores[0].replayed_mutating_action = true;
    let violations = report.validate();
    assert!(violations.iter().any(|v| matches!(
        v,
        PresentationRestoreViolation::ReplayedMutatingAction { .. }
    )));
}

#[test]
fn a_degrade_hidden_behind_success_fails_validation() {
    let session = session();
    let mut report = project_restore_report(
        &session,
        &RestoreProjectionInputs::with_degrades(
            PresentationRestoreTrigger::Cancel,
            vec![WaypointDegrade::new(
                "wp:restore-unit:1",
                RestoreDegradeTrigger::RevokedSharingGrant,
                "disconnected: grant revoked",
            )],
        ),
    );
    // Tamper: strip the honest cause so the degrade is hidden behind success.
    report.waypoint_restores[0].degrade_trigger = None;
    report.waypoint_restores[0].placeholder_label = None;
    let violations = report.validate();
    assert!(violations.iter().any(|v| matches!(
        v,
        PresentationRestoreViolation::WaypointInconsistent { .. }
    ) || matches!(
        v,
        PresentationRestoreViolation::DegradeHiddenBehindSuccess { .. }
    ) || matches!(
        v,
        PresentationRestoreViolation::RestoreClassMismatch { .. }
    )));
}

#[test]
fn seeded_corpus_validates_and_round_trips() {
    let corpus = seeded_presentation_restore_corpus();
    validate_presentation_restore_corpus(&corpus).expect("seeded corpus must validate");

    let json = serde_json::to_string(&corpus).unwrap();
    let parsed: PresentationRestoreCorpus = serde_json::from_str(&json).unwrap();
    assert_eq!(parsed, corpus);

    assert!(corpus.summary.exact_demonstrated);
    assert!(corpus.summary.compatible_demonstrated);
    assert!(corpus.summary.layout_only_demonstrated);
    assert!(corpus.summary.evidence_only_demonstrated);
    assert!(corpus.summary.no_restore_demonstrated);
    assert!(corpus.summary.placeholder_demonstrated);
    assert!(corpus.summary.disconnected_demonstrated);
    assert!(corpus.summary.no_mutating_replay);
    assert!(corpus.summary.no_authority_reacquired);
    assert!(corpus.summary.no_improvised_shell);
    assert!(corpus.summary.no_hidden_degrade);
}

#[test]
fn checked_in_fixtures_match_the_seed_projection() {
    let corpus = seeded_presentation_restore_corpus();
    let fixture = include_str!(
        "../../../../../fixtures/presentation/restore-no-rerun/restore-report-corpus.json"
    );
    let parsed: PresentationRestoreCorpus = serde_json::from_str(fixture).expect("fixture parses");
    assert_eq!(
        parsed, corpus,
        "fixtures/presentation/restore-no-rerun drifted from the seed corpus; \
         regenerate with the dump_presentation_restore example"
    );
}

#[test]
fn support_export_excludes_refs_and_placeholder_labels() {
    let corpus = seeded_presentation_restore_corpus();
    let export = presentation_restore_support_export(
        "support-export:presentation-restore:001",
        "2026-06-20T00:00:00Z",
        &corpus,
    );
    assert!(export.raw_private_material_excluded);
    assert_eq!(export.rows.len(), corpus.cases.len());

    let export_json = serde_json::to_string(&export).unwrap();
    for case in &corpus.cases {
        assert!(!export_json.contains(&case.scenario_label));
        let report = &case.report;
        if !report.restored_layout_ref.is_empty() {
            assert!(
                !export_json.contains(&report.restored_layout_ref),
                "support export leaked a layout ref for {}",
                case.case_id
            );
            assert!(
                !export_json.contains(&report.checkpoint_id),
                "support export leaked a checkpoint id for {}",
                case.case_id
            );
        }
        for waypoint in &report.waypoint_restores {
            if let Some(label) = &waypoint.placeholder_label {
                assert!(
                    !export_json.contains(label),
                    "support export leaked a placeholder label for {}",
                    case.case_id
                );
            }
            assert!(
                !export_json.contains(&waypoint.target_object_ref),
                "support export leaked a target ref for {}",
                case.case_id
            );
        }
    }
}
