//! Unit tests for the macro-recorder session and replay object.

use super::*;

fn notebook_saved() -> MacroRecorderSession {
    seeded_macro_recorder_session(RecipeBuilderEntrypoint::Notebook)
}

#[test]
fn seeded_packet_is_stable_and_binds_every_entrypoint() {
    let packet = seeded_macro_recorder_first_consumers_packet();
    assert!(packet.is_stable());
    assert!(packet.validation_findings.is_empty());
    assert_eq!(
        packet.consumer_bindings.len(),
        RecipeBuilderEntrypoint::ALL.len()
    );
    assert!(validate_macro_recorder_first_consumers_packet(&packet).is_ok());
}

#[test]
fn replay_resolution_declares_scope_and_refuses_unsafe_reuse() {
    let session = seeded_macro_recorder_session(RecipeBuilderEntrypoint::RequestApi);
    let resolution = session.resolve_replay("2026-06-18T00:00:00Z");
    assert!(resolution.declares_target_scope);
    assert!(resolution.refuses_on_context_mismatch);
    assert!(resolution.reresolves_supported_command_set);
    assert!(resolution.is_fail_closed_safe());
    // The request macro's command set changed, so replay fails closed.
    assert!(resolution.fails_closed);
    assert!(!resolution.admissible);
    assert_eq!(
        resolution.replay_action_class,
        MacroReplayActionClass::BlockedSupportedSetChanged
    );
}

#[test]
fn clean_macro_replays_in_declared_scope() {
    let session = notebook_saved();
    assert_eq!(
        session.resolved_replay_class(),
        MacroReplayActionClass::ReplayInDeclaredScope
    );
    assert!(session.replay_admissible());
    assert!(session.replay_consistent());
}

#[test]
fn imported_macro_fails_closed() {
    let mut session = notebook_saved();
    session.imported_from_repository_content = true;
    assert_eq!(
        session.resolved_replay_class(),
        MacroReplayActionClass::BlockedImportedFromRepositoryContent
    );
    assert!(!session.replay_admissible());
}

#[test]
fn fail_closed_dominates_reconcilable() {
    let mut session = notebook_saved();
    session.current_replay_blockers = vec![
        MacroReplayBlocker::ActiveContextReconcilable,
        MacroReplayBlocker::KillSwitchEngaged,
    ];
    // The fail-closed kill-switch blocker dominates the reconcilable one.
    assert_eq!(
        session.resolved_replay_class(),
        MacroReplayActionClass::BlockedKillSwitch
    );
}

#[test]
fn every_blocker_maps_to_its_replay_action() {
    for blocker in MacroReplayBlocker::ALL {
        // Each blocker resolves to its own action when it is the only blocker.
        let class = derive_replay_class(false, &[blocker]);
        assert_eq!(class, blocker.replay_action_class());
        // No-blocker is the only admissible-in-scope pairing.
        if blocker == MacroReplayBlocker::NoBlockerPresent {
            assert_eq!(class, MacroReplayActionClass::ReplayInDeclaredScope);
        }
    }
}

#[test]
fn unsupported_command_is_flagged_and_blocks_save() {
    let session = seeded_unsupported_command_session();
    assert!(session.has_unsupported_command());
    let review = session.captured_command_review();
    assert!(review.has_unsupported_command);
    assert!(!review.save_admissible);
    assert_eq!(review.unsupported_command_count, 1);
    assert_eq!(review.unsupported_command_warnings.len(), 1);
    assert_eq!(
        review.unsupported_command_warnings[0].support_class,
        CapturedCommandSupportClass::UnsupportedRunsProcess
    );
    // A discarded recording mints no macro manifest.
    assert!(session.resulting_macro_manifest_ref.is_none());
    assert!(session.disposition_consistent());
    // Replay fails closed.
    assert!(session.resolved_replay_class().is_fail_closed());
}

#[test]
fn cross_scope_macro_requires_explicit_promotion() {
    let session = seeded_cross_scope_promotion_session();
    assert!(session.declared_target_scope_class.requires_promotion());
    assert!(session.promotion_consistent());
    assert_eq!(
        session.resolved_replay_class(),
        MacroReplayActionClass::BlockedPromotionRequired
    );
    // A macro that crosses scope must not be UI-only-not-promotable.
    assert_ne!(
        session.promotion_affordance_class,
        MacroPromotionAffordanceClass::NotPromotableUiOnly
    );
}

#[test]
fn macros_are_profile_local_and_ui_only() {
    let packet = seeded_macro_recorder_first_consumers_packet();
    for binding in &packet.consumer_bindings {
        for session in &binding.sessions {
            assert!(session.profile_local_default_consistent());
            assert!(session.storage_scope_class.is_local_only());
            assert!(session.safety_labels_constrained());
            for label in &session.projected_safety_labels {
                assert!(matches!(
                    label,
                    AutomationSafetyLabelId::MacroSafe | AutomationSafetyLabelId::UiOnly
                ));
            }
        }
    }
}

#[test]
fn saved_session_projects_a_conforming_macro_record() {
    let session = notebook_saved();
    let record = session.to_session_record();
    assert_eq!(record.record_kind, "macro_session_record");
    assert_eq!(
        record.manifest_target_schema_ref,
        RECIPE_MANIFEST_SCHEMA_REF
    );
    // Only supported commands become capture steps.
    assert_eq!(
        record.captured_steps.len(),
        session
            .captured_commands
            .iter()
            .filter(|command| command.is_supported())
            .count()
    );
    assert!(!record.captured_steps.is_empty());
    // The record carries the resulting macro manifest ref for a saved macro.
    assert!(record.resulting_macro_manifest_ref.is_some());
}

#[test]
fn active_recording_strip_reports_capture_counts() {
    let panel = seeded_consumer_panel(RecipeBuilderEntrypoint::Notebook);
    let live = &panel[1];
    let strip = live.active_recording_strip();
    assert!(strip.is_capturing);
    assert_eq!(
        strip.recorder_state_class,
        MacroRecorderStateClass::Recording
    );
    assert_eq!(strip.unsupported_command_count, 0);
    assert_eq!(
        strip.captured_command_count,
        live.captured_commands.len() as u32
    );
}

#[test]
fn export_round_trips_and_preserves_replay() {
    let session = notebook_saved();
    let export = session.export("export:test", "2026-06-18T00:01:00Z");
    assert_eq!(export.import(), session);
    assert!(export.replay_and_scope_preserved());
    assert_eq!(export.export_digest, session.session_digest());
}

#[test]
fn support_export_carries_resolutions_and_is_safe() {
    let packet = seeded_macro_recorder_first_consumers_packet();
    let export = packet.support_export("support:test", "2026-06-18T00:01:00Z");
    assert!(export.is_export_safe());
    assert_eq!(
        export.consumer_rows.len(),
        RecipeBuilderEntrypoint::ALL.len()
    );
    let total_sessions: usize = packet
        .consumer_bindings
        .iter()
        .map(|binding| binding.sessions.len())
        .sum();
    assert_eq!(export.replay_resolutions.len(), total_sessions);
    assert_eq!(export.packet_digest, packet.packet_digest);
}

#[test]
fn cli_headless_view_explains_every_entrypoint() {
    let packet = seeded_macro_recorder_first_consumers_packet();
    let view = packet.cli_headless_view("cli:test", "2026-06-18T00:01:00Z");
    assert!(view.every_entrypoint_explained());
    assert_eq!(
        view.consumer_lines.len(),
        RecipeBuilderEntrypoint::ALL.len()
    );
}

#[test]
fn raw_secret_reference_is_rejected() {
    let mut session = notebook_saved();
    session.captured_commands[0].command_id = "raw:secret".to_owned();
    assert!(!session.captures_are_opaque());
}

#[test]
fn dropped_entrypoint_blocks_stable() {
    let mut input = current_macro_recorder_first_consumers_input();
    input
        .consumer_bindings
        .retain(|binding| binding.entrypoint != RecipeBuilderEntrypoint::Incident);
    let packet = MacroRecorderFirstConsumersPacket::materialize(input);
    assert!(!packet.is_stable());
    assert!(packet
        .validation_findings
        .iter()
        .any(|finding| finding.finding_kind == MacroRecorderFindingKind::MissingEntrypoint));
}
