//! Unit tests for the governed presentation-mode overlays.

use super::corpus::{
    seeded_presentation_mode_corpus, validate_presentation_mode_corpus,
    PresentationModeSupportExport,
};
use super::overlay::project_overlay;
use super::session::{
    restore_from_checkpoint, AudienceScope, BoundaryLabel, LeaderFollowState, PresentationSession,
    PresentationSessionBuilder, RestoreCheckpoint, RestoreTrigger, SessionLifecycleState,
    SpeakerNote, WalkthroughSurfaceKind, WaypointCompletionState,
};

fn checkpoint() -> RestoreCheckpoint {
    RestoreCheckpoint {
        checkpoint_id: "presentation:checkpoint:unit".into(),
        prior_layout_ref: "window-topology:unit:prior".into(),
        prior_focus_ref: "focus-chain:unit:prior".into(),
        prior_panel_visibility_ref: "panel-visibility:unit:prior".into(),
        accessibility_posture_ref: "a11y-posture:unit:prior".into(),
        captured_at: "2026-05-20T09:00:00Z".into(),
    }
}

fn waypoint(
    id: &str,
    kind: WalkthroughSurfaceKind,
    note: Option<SpeakerNote>,
) -> super::FollowWaypoint {
    super::FollowWaypoint {
        waypoint_id: id.into(),
        ordinal: 1,
        step_title: "Step".into(),
        surface_kind: kind,
        target_object_ref: format!("obj:{id}"),
        file_path_ref: Some("src/lib.rs".into()),
        symbol_anchor_ref: Some("fn main".into()),
        branch_workspace_ref: "branch:main@workspace:local".into(),
        boundary_label: BoundaryLabel::Local,
        zoom_layout_hint_ref: None,
        reveal_action_ref: None,
        completion_state: WaypointCompletionState::Current,
        speaker_note: note,
        reuses_existing_surface: true,
        creates_parallel_artifact: false,
    }
}

fn presenting_session() -> PresentationSession {
    PresentationSessionBuilder::new(
        "presentation:session:unit",
        LeaderFollowState::Presenting,
        AudienceScope::SoloRehearsal,
        checkpoint(),
    )
    .focus("wp:unit:1")
    .waypoint(waypoint(
        "wp:unit:1",
        WalkthroughSurfaceKind::Editor,
        Some(SpeakerNote::local(
            "note:unit:1",
            "wp:unit:1",
            "private prompt",
        )),
    ))
    .build()
}

#[test]
fn session_builder_fixes_guardrail_flags_safe() {
    let s = presenting_session();
    assert!(!s.grants_mutation_authority);
    assert!(!s.grants_control_authority);
    assert!(!s.establishes_private_data_ownership);
    assert!(s.speaker_notes_default_local_only);
    assert!(s.preserves_source_provenance);
    assert!(s.reuses_existing_surfaces_only);
}

#[test]
fn local_notes_do_not_leak_by_default() {
    let s = presenting_session();
    assert!(s.all_notes_local_only());
    assert!(s.shared_notes_are_explicit());
}

#[test]
fn shared_note_sets_explicit_promotion_marker() {
    let note = SpeakerNote::shared("note:s", "wp:s", "shared body");
    assert!(note.shared_promotion_explicit);
    assert!(!note.scope.is_local_only());

    let local = SpeakerNote::local("note:l", "wp:l", "local body");
    assert!(!local.shared_promotion_explicit);
    assert!(local.scope.is_local_only());
}

#[test]
fn overlay_is_keyboard_complete_and_attention_only() {
    let overlay = project_overlay(&presenting_session());
    assert!(overlay.keyboard_complete);
    assert!(!overlay.pointer_only);
    assert!(overlay.screen_reader_reachable);
    for action in overlay.all_actions() {
        assert!(
            !action.command_id.is_empty(),
            "{} missing command",
            action.action_id
        );
        assert!(
            !action.key_binding_ref.is_empty(),
            "{} missing keybinding",
            action.action_id
        );
        assert!(
            !action.accessible_label.is_empty(),
            "{} missing a11y label",
            action.action_id
        );
        assert!(
            !action.is_destructive,
            "{} is destructive",
            action.action_id
        );
        assert!(
            !action.mutates_workspace,
            "{} mutates workspace",
            action.action_id
        );
    }
}

#[test]
fn overlay_preserves_source_provenance() {
    let overlay = project_overlay(&presenting_session());
    let strip = &overlay.provenance_strip;
    assert!(strip.source_identity_preserved);
    assert_eq!(strip.file_path_ref.as_deref(), Some("src/lib.rs"));
    assert_eq!(strip.symbol_anchor_ref.as_deref(), Some("fn main"));
    assert_eq!(strip.branch_workspace_ref, "branch:main@workspace:local");
    assert_eq!(strip.boundary_label, BoundaryLabel::Local);
}

#[test]
fn breakaway_banner_appears_only_when_broken_away() {
    let presenting = project_overlay(&presenting_session());
    assert!(presenting.breakaway_banner.is_none());

    let broken = PresentationSessionBuilder::new(
        "presentation:session:broken",
        LeaderFollowState::BrokenAway,
        AudienceScope::SharedWorkspace,
        checkpoint(),
    )
    .focus("wp:broken:1")
    .waypoint(waypoint("wp:broken:1", WalkthroughSurfaceKind::Diff, None))
    .build();
    let overlay = project_overlay(&broken);
    let banner = overlay.breakaway_banner.expect("breakaway banner present");
    assert_eq!(banner.state_label, "You are browsing independently");
    assert!(!banner.return_to_presenter_action.mutates_workspace);
}

#[test]
fn restore_returns_prior_environment_for_every_trigger() {
    let s = presenting_session();
    for trigger in [
        RestoreTrigger::Exit,
        RestoreTrigger::Cancel,
        RestoreTrigger::CrashRecovery,
    ] {
        let outcome = restore_from_checkpoint(&s, trigger);
        assert!(outcome.matches_checkpoint);
        assert!(!outcome.left_in_improvised_shell);
        assert_eq!(
            outcome.restored_layout_ref,
            s.restore_checkpoint.prior_layout_ref
        );
        assert_eq!(
            outcome.restored_focus_ref,
            s.restore_checkpoint.prior_focus_ref
        );
        assert_eq!(outcome.trigger, trigger);
    }
    // Each trigger lands in its own restored lifecycle state.
    assert_eq!(
        restore_from_checkpoint(&s, RestoreTrigger::Exit).resulting_lifecycle_state,
        SessionLifecycleState::ExitedRestored
    );
    assert_eq!(
        restore_from_checkpoint(&s, RestoreTrigger::CrashRecovery).resulting_lifecycle_state,
        SessionLifecycleState::CrashRecoveredRestored
    );
}

#[test]
fn waypoint_rail_marks_current_step() {
    let overlay = project_overlay(&presenting_session());
    let current: Vec<_> = overlay
        .waypoint_rail
        .rows
        .iter()
        .filter(|r| r.is_current)
        .collect();
    assert_eq!(current.len(), 1);
    assert_eq!(current[0].waypoint_ref, "wp:unit:1");
}

#[test]
fn seeded_corpus_validates_and_round_trips() {
    let corpus = seeded_presentation_mode_corpus();
    validate_presentation_mode_corpus(&corpus).expect("seeded corpus must validate");

    let json = serde_json::to_string(&corpus).unwrap();
    let parsed: super::PresentationModeCorpus = serde_json::from_str(&json).unwrap();
    assert_eq!(parsed, corpus);

    let export = PresentationModeSupportExport::from_corpus(
        "support-export:presentation-mode-beta:001",
        "2026-05-20T00:00:00Z",
        &corpus,
    );
    assert!(export.raw_private_material_excluded);
    assert_eq!(export.rows.len(), corpus.sessions.len());
}

#[test]
fn support_export_excludes_note_bodies_and_step_copy() {
    let corpus = seeded_presentation_mode_corpus();
    let export = PresentationModeSupportExport::from_corpus(
        "support-export:presentation-mode-beta:001",
        "2026-05-20T00:00:00Z",
        &corpus,
    );
    let export_json = serde_json::to_string(&export).unwrap();
    for case in &corpus.sessions {
        assert!(
            !export_json.contains(&case.scenario_label),
            "support export leaked scenario copy for {}",
            case.case_id
        );
        for w in &case.session.waypoints {
            assert!(
                !export_json.contains(&w.step_title),
                "support export leaked step title for {}",
                case.case_id
            );
            if let Some(path) = &w.file_path_ref {
                assert!(
                    !export_json.contains(path),
                    "support export leaked file path for {}",
                    case.case_id
                );
            }
            if let Some(note) = &w.speaker_note {
                if let Some(body) = &note.body_label {
                    assert!(
                        !export_json.contains(body),
                        "support export leaked note body for {}",
                        case.case_id
                    );
                }
            }
        }
    }
}
