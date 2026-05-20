//! Unit tests for the governed teaching/classroom sessions.

use super::affordances::{project_affordances, AffordanceKind};
use super::corpus::{
    seeded_teaching_classroom_corpus, validate_teaching_classroom_corpus,
    TeachingClassroomSupportExport,
};
use super::session::{
    restore_from_checkpoint, ClientClass, DemonstratedAction, DemonstrationKind, DocsPackState,
    ReplayPolicy, RestoreCheckpoint, RestoreTrigger, RetentionClass, SegmentKind, SessionKind,
    SessionLifecycleState, TeachingParticipant, TeachingRole, TeachingSegment, TeachingSession,
    TeachingSessionBuilder,
};

fn checkpoint() -> RestoreCheckpoint {
    RestoreCheckpoint {
        checkpoint_id: "teaching:checkpoint:unit".into(),
        prior_layout_ref: "window-topology:unit:prior".into(),
        prior_focus_ref: "focus-chain:unit:prior".into(),
        prior_panel_visibility_ref: "panel-visibility:unit:prior".into(),
        accessibility_posture_ref: "a11y-posture:unit:prior".into(),
        captured_at: "2026-05-20T09:00:00Z".into(),
    }
}

fn tour_segment(id: &str, state: DocsPackState, action: Option<DemonstratedAction>) -> TeachingSegment {
    TeachingSegment {
        segment_id: id.into(),
        ordinal: 1,
        segment_kind: SegmentKind::Tour,
        title: "Walk the safe-start tour".into(),
        learning_object_ref: "tour:aureline.safe-start.command-backed".into(),
        docs_node_refs: vec!["docs-node:help.guided-tours.safe-start".into()],
        graph_node_refs: vec![],
        citation_refs: vec!["citation:docs-help:guided-tours-beta".into()],
        docs_pack_ref: "docs-pack:aureline-help:guided-tours".into(),
        docs_pack_state: state,
        docs_pack_disclosure_ref: if state.requires_disclosure() {
            Some(format!("disclosure:docs-pack:{id}"))
        } else {
            None
        },
        resume_ref: format!("resume:segment:{id}"),
        resumable_across_restart: true,
        resumable_across_reconnect: true,
        demonstrated_action: action,
        cites_learning_mode_object: true,
    }
}

fn moderator_session() -> TeachingSession {
    TeachingSessionBuilder::new(
        "teaching:session:unit",
        SessionKind::Teaching,
        TeachingRole::Moderator,
        ReplayPolicy::Ephemeral,
        RetentionClass::DiscardOnExit,
        checkpoint(),
    )
    .focus("segment:unit:1")
    .exercise_pack("tour-pack:aureline.safe-start.beta")
    .segment(tour_segment(
        "segment:unit:1",
        DocsPackState::Installed,
        Some(DemonstratedAction::explain("action:unit:explain")),
    ))
    .build()
}

#[test]
fn session_builder_fixes_guardrail_flags_safe() {
    let s = moderator_session();
    assert!(!s.grants_mutation_authority);
    assert!(!s.grants_terminal_or_debug_control);
    assert!(!s.grants_broader_authority_than_workspace);
    assert!(!s.establishes_private_data_ownership);
    assert!(!s.creates_hidden_progress_model);
    assert!(!s.creates_cohort_or_grading_flow);
    assert!(s.demonstrations_non_mutating_by_default);
    assert!(s.preserves_source_citations);
    assert!(s.reuses_learning_mode_objects_only);
    assert!(s.restore_on_exit_guaranteed);
}

#[test]
fn teaching_roles_never_grant_terminal_debug_or_broader_authority() {
    for role in [
        TeachingRole::Moderator,
        TeachingRole::Participant,
        TeachingRole::Observer,
        TeachingRole::Approver,
        TeachingRole::Scribe,
    ] {
        assert!(!role.grants_terminal_or_debug_control(), "{}", role.as_str());
        assert!(!role.implies_broader_authority(), "{}", role.as_str());
    }
    // Only the moderator drives.
    assert!(TeachingRole::Moderator.can_drive_session());
    for role in [
        TeachingRole::Participant,
        TeachingRole::Observer,
        TeachingRole::Approver,
        TeachingRole::Scribe,
    ] {
        assert!(!role.can_drive_session(), "{}", role.as_str());
    }
    // Observers and scribes never see a mutation affordance.
    assert!(!TeachingRole::Observer.may_expose_mutation_affordance());
    assert!(!TeachingRole::Scribe.may_expose_mutation_affordance());
}

#[test]
fn demonstrations_are_non_mutating_by_default_and_only_fenced_mutations_pass() {
    let explain = DemonstratedAction::explain("a");
    assert!(!explain.mutates_workspace);
    assert!(explain.is_properly_fenced());

    let preview = DemonstratedAction::preview_only("a", "cmd:x", "preview:x");
    assert!(!preview.mutates_workspace);
    assert!(preview.is_properly_fenced());

    let mutation = DemonstratedAction::mutation_through_fences(
        "a",
        "cmd:workspace.import_profile",
        "preview:workspace.import_profile",
        "approval:path:workspace.import_profile",
        "rollback:workspace.import_profile.checkpoint-or-undo",
        "evidence-rule:command-preview-approval",
    );
    assert!(mutation.mutates_workspace);
    assert!(mutation.is_properly_fenced());
    assert_eq!(mutation.kind, DemonstrationKind::MutationThroughFences);

    // A mutation that drops a fence is no longer properly fenced.
    let mut broken = mutation.clone();
    broken.approval_path_ref = None;
    assert!(!broken.is_properly_fenced());
}

#[test]
fn degraded_docs_pack_states_require_a_disclosure() {
    for state in [
        DocsPackState::Cached,
        DocsPackState::Mirrored,
        DocsPackState::Offline,
        DocsPackState::NotInstalled,
    ] {
        assert!(state.requires_disclosure(), "{}", state.as_str());
        let with = tour_segment("segment:disc", state, None);
        assert!(with.docs_pack_disclosure_ok());
        let mut without = with.clone();
        without.docs_pack_disclosure_ref = None;
        assert!(!without.docs_pack_disclosure_ok());
    }
    // Installed packs are current and need no disclosure.
    assert!(!DocsPackState::Installed.requires_disclosure());
    assert!(DocsPackState::Offline.requires_reconnect());
    assert!(DocsPackState::NotInstalled.requires_install());
    assert!(!DocsPackState::NotInstalled.is_locally_available());
}

#[test]
fn opt_in_markers_track_replay_and_retention_scope() {
    let local = moderator_session();
    assert!(!local.replay_archive_opt_in_explicit);
    assert!(!local.shared_retention_opt_in_explicit);
    assert!(local.opt_in_markers_consistent());

    let shared = TeachingSessionBuilder::new(
        "teaching:session:shared",
        SessionKind::Classroom,
        TeachingRole::Moderator,
        ReplayPolicy::SharedArchiveOptIn,
        RetentionClass::SharedWorkspaceRetained,
        checkpoint(),
    )
    .segment(tour_segment("segment:shared:1", DocsPackState::Installed, None))
    .build();
    assert!(shared.replay_archive_opt_in_explicit);
    assert!(shared.shared_retention_opt_in_explicit);
    assert!(shared.opt_in_markers_consistent());
}

#[test]
fn observers_and_scribes_get_no_drive_or_mutation_affordance() {
    let session = TeachingSessionBuilder::new(
        "teaching:session:roles",
        SessionKind::Classroom,
        TeachingRole::Moderator,
        ReplayPolicy::Ephemeral,
        RetentionClass::DiscardOnExit,
        checkpoint(),
    )
    .participant(TeachingParticipant {
        participant_id: "p:observer".into(),
        role: TeachingRole::Observer,
        client_class: ClientClass::Full,
        is_external_guest: false,
    })
    .participant(TeachingParticipant {
        participant_id: "p:scribe".into(),
        role: TeachingRole::Scribe,
        client_class: ClientClass::Full,
        is_external_guest: false,
    })
    .participant(TeachingParticipant {
        participant_id: "p:approver".into(),
        role: TeachingRole::Approver,
        client_class: ClientClass::Full,
        is_external_guest: false,
    })
    .segment(tour_segment(
        "segment:roles:1",
        DocsPackState::Installed,
        Some(DemonstratedAction::mutation_through_fences(
            "action:roles:mutation",
            "cmd:workspace.import_profile",
            "preview:workspace.import_profile",
            "approval:path:workspace.import_profile",
            "rollback:workspace.import_profile.checkpoint-or-undo",
            "evidence-rule:command-preview-approval",
        )),
    ))
    .build();

    let aff = project_affordances(&session);
    for view in &aff.participant_views {
        assert!(!view.exposes_terminal_or_debug_control, "{}", view.participant_id);
        match view.role {
            TeachingRole::Moderator => {
                assert!(view.exposes_drive_control);
                assert!(view.exposes_mutation_affordance);
            }
            TeachingRole::Observer | TeachingRole::Scribe => {
                assert!(!view.exposes_drive_control, "{}", view.participant_id);
                assert!(!view.exposes_mutation_affordance, "{}", view.participant_id);
            }
            TeachingRole::Approver => {
                assert!(!view.exposes_drive_control);
                assert!(view.exposes_mutation_affordance);
                let approves = view
                    .affordances
                    .iter()
                    .any(|a| a.kind == AffordanceKind::ApproveMutation);
                assert!(approves, "approver should see an approve affordance");
            }
            TeachingRole::Participant => {}
        }
    }
}

#[test]
fn constrained_clients_join_safe_without_broken_controls() {
    let session = TeachingSessionBuilder::new(
        "teaching:session:constrained",
        SessionKind::Classroom,
        TeachingRole::Moderator,
        ReplayPolicy::Ephemeral,
        RetentionClass::DiscardOnExit,
        checkpoint(),
    )
    .participant(TeachingParticipant {
        participant_id: "p:limited".into(),
        role: TeachingRole::Observer,
        client_class: ClientClass::Limited,
        is_external_guest: true,
    })
    .participant(TeachingParticipant {
        participant_id: "p:low".into(),
        role: TeachingRole::Scribe,
        client_class: ClientClass::LowBandwidth,
        is_external_guest: false,
    })
    .segment(tour_segment(
        "segment:constrained:1",
        DocsPackState::Installed,
        Some(DemonstratedAction::mutation_through_fences(
            "action:constrained:mutation",
            "cmd:workspace.import_profile",
            "preview:workspace.import_profile",
            "approval:path:workspace.import_profile",
            "rollback:workspace.import_profile.checkpoint-or-undo",
            "evidence-rule:command-preview-approval",
        )),
    ))
    .build();

    let aff = project_affordances(&session);
    assert!(aff.all_constrained_clients_join_safe);
    for view in &aff.participant_views {
        if view.client_class.is_constrained() {
            assert!(!view.exposes_drive_control, "{}", view.participant_id);
            assert!(!view.exposes_mutation_affordance, "{}", view.participant_id);
            assert!(view.constrained_client_join_safe, "{}", view.participant_id);
        }
        // Every projected control is actionable — never broken/disabled.
        for a in &view.affordances {
            assert!(a.actionable, "{}", a.affordance_id);
            assert!(!a.is_terminal_or_debug_control, "{}", a.affordance_id);
        }
    }
    // The low-bandwidth scribe can still take notes.
    let scribe = aff
        .participant_views
        .iter()
        .find(|v| v.participant_id == "p:low")
        .unwrap();
    assert!(scribe.can_take_notes);
}

#[test]
fn mutation_affordance_reuses_ordinary_command_path() {
    let session = moderator_session();
    // The unit session demonstrates explain-only, so no mutation affordance.
    let aff = project_affordances(&session);
    assert!(aff
        .all_affordances()
        .all(|a| a.routes_through_ordinary_command_path));
    assert!(aff.all_affordances().all(|a| !a.is_terminal_or_debug_control));
}

#[test]
fn restore_returns_prior_environment_for_every_trigger() {
    let s = moderator_session();
    for trigger in [
        RestoreTrigger::Exit,
        RestoreTrigger::Leave,
        RestoreTrigger::CrashRecovery,
    ] {
        let outcome = restore_from_checkpoint(&s, trigger);
        assert!(outcome.matches_checkpoint);
        assert!(!outcome.left_in_improvised_shell);
        assert_eq!(outcome.restored_layout_ref, s.restore_checkpoint.prior_layout_ref);
        assert_eq!(outcome.restored_focus_ref, s.restore_checkpoint.prior_focus_ref);
        assert_eq!(
            outcome.restored_panel_visibility_ref,
            s.restore_checkpoint.prior_panel_visibility_ref
        );
        assert_eq!(
            outcome.restored_accessibility_posture_ref,
            s.restore_checkpoint.accessibility_posture_ref
        );
        assert_eq!(outcome.trigger, trigger);
    }
    assert_eq!(
        restore_from_checkpoint(&s, RestoreTrigger::Exit).resulting_lifecycle_state,
        SessionLifecycleState::ExitedRestored
    );
    assert_eq!(
        restore_from_checkpoint(&s, RestoreTrigger::Leave).resulting_lifecycle_state,
        SessionLifecycleState::LeftRestored
    );
    assert_eq!(
        restore_from_checkpoint(&s, RestoreTrigger::CrashRecovery).resulting_lifecycle_state,
        SessionLifecycleState::CrashRecoveredRestored
    );
}

#[test]
fn seeded_corpus_validates_and_round_trips() {
    let corpus = seeded_teaching_classroom_corpus();
    validate_teaching_classroom_corpus(&corpus).expect("seeded corpus must validate");

    let json = serde_json::to_string(&corpus).unwrap();
    let parsed: super::TeachingClassroomCorpus = serde_json::from_str(&json).unwrap();
    assert_eq!(parsed, corpus);

    let export = TeachingClassroomSupportExport::from_corpus(
        "support-export:teaching-classroom-beta:001",
        "2026-05-20T00:00:00Z",
        &corpus,
    );
    assert!(export.raw_private_material_excluded);
    assert_eq!(export.rows.len(), corpus.sessions.len());
}

#[test]
fn support_export_excludes_scenario_copy_titles_and_paths() {
    let corpus = seeded_teaching_classroom_corpus();
    let export = TeachingClassroomSupportExport::from_corpus(
        "support-export:teaching-classroom-beta:001",
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
        for seg in &case.session.segments {
            assert!(
                !export_json.contains(&seg.title),
                "support export leaked segment title for {}",
                case.case_id
            );
        }
    }
}
