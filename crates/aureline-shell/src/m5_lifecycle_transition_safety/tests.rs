//! Inline unit tests for the M5 lifecycle transition-safety proof.

use super::*;

#[test]
fn seeded_packet_covers_every_object_family_and_is_clean() {
    let packet = seeded_m5_lifecycle_transition_safety_packet();
    validate_m5_lifecycle_transition_safety_packet(&packet)
        .expect("seeded packet must validate clean");

    assert_eq!(packet.rows.len(), REQUIRED_OBJECT_FAMILIES.len());
    for object_family in REQUIRED_OBJECT_FAMILIES {
        assert!(
            packet.row(object_family).is_some(),
            "missing row for {}",
            object_family.as_str()
        );
    }
    assert!(packet.report_clean);
    assert!(packet.blocking_findings.is_empty());
    assert_eq!(packet.red_row_count, 0);
    assert!(packet.all_rows_publishable);
}

#[test]
fn seeded_packet_has_nine_green_and_four_yellow_rows() {
    let packet = seeded_m5_lifecycle_transition_safety_packet();
    assert_eq!(packet.green_row_count, 9);
    assert_eq!(packet.yellow_row_count, 4);
    assert_eq!(packet.red_row_count, 0);

    for object_family in [
        M5LifecycleObjectFamily::RemoteSession,
        M5LifecycleObjectFamily::PipelineRun,
        M5LifecycleObjectFamily::NotebookRuntime,
        M5LifecycleObjectFamily::CollaborationSession,
    ] {
        assert_eq!(
            packet.row(object_family).unwrap().derived_status,
            TransitionSafetyStatus::Yellow,
            "{} should auto-narrow to yellow",
            object_family.as_str()
        );
    }
}

#[test]
fn every_row_status_is_the_derived_value() {
    let packet = seeded_m5_lifecycle_transition_safety_packet();
    for row in &packet.rows {
        assert_eq!(
            row.derived_status,
            row.recompute_status(),
            "row {} declares a stale status",
            row.object_family.as_str()
        );
        assert_eq!(row.transition_causes, row.recompute_causes());
    }
}

#[test]
fn every_row_certifies_all_declared_consumer_surfaces() {
    let packet = seeded_m5_lifecycle_transition_safety_packet();
    for row in &packet.rows {
        assert!(
            row.consumer_surfaces_complete(),
            "row {} does not certify all declared consumer surfaces",
            row.object_family.as_str()
        );
        assert!(row.headless_parity_preserved);
    }
}

#[test]
fn every_state_machine_and_binding_is_pulled_from_the_frozen_matrix() {
    use crate::freeze_the_m5_lifecycle_state_and_journey_checkpoint_matrix::seeded_m5_lifecycle_matrix;

    let matrix = seeded_m5_lifecycle_matrix();
    let packet = seeded_m5_lifecycle_transition_safety_packet();
    for matrix_row in &matrix.object_state_rows {
        let row = packet.row(matrix_row.object_family).unwrap();
        assert_eq!(row.admitted_states, matrix_row.admitted_states);
        assert_eq!(row.recovery_affordance, matrix_row.recovery_affordance);
        assert_eq!(row.qualification, matrix_row.qualification);
        assert_eq!(row.required_consumer_surfaces, matrix_row.consumer_surfaces);
        assert_eq!(
            row.applicable_downgrade_triggers,
            matrix_row.downgrade_triggers
        );
        // The explicit state machine must always admit `ready`.
        assert!(row.admitted_states.contains(&M5LifecycleState::Ready));
    }
}

#[test]
fn narrowed_rows_disclose_a_reason() {
    let packet = seeded_m5_lifecycle_transition_safety_packet();
    for row in &packet.rows {
        if !matches!(row.derived_status, TransitionSafetyStatus::Green) {
            assert!(
                row.narrowing_reason
                    .as_deref()
                    .map(|reason| !reason.trim().is_empty())
                    .unwrap_or(false),
                "narrowed row {} hides its reason",
                row.object_family.as_str()
            );
        }
    }
}

#[test]
fn disclosed_reduced_fallback_carries_an_active_waiver() {
    let packet = seeded_m5_lifecycle_transition_safety_packet();
    let collaboration = packet
        .row(M5LifecycleObjectFamily::CollaborationSession)
        .unwrap();
    assert!(matches!(
        collaboration.local_fallback,
        LocalFallbackState::DisclosedReducedFallback
    ));
    assert!(collaboration.requires_waiver());
    assert!(collaboration.has_active_waiver());
    assert_eq!(packet.active_waivers.len(), 1);
    assert!(packet.active_waivers[0].is_active_at(&packet.generated_at));
}

#[test]
fn disclosed_reduced_transition_set_keeps_remote_yellow_without_a_waiver() {
    let packet = seeded_m5_lifecycle_transition_safety_packet();
    let remote = packet.row(M5LifecycleObjectFamily::RemoteSession).unwrap();
    assert!(matches!(
        remote.safe_transition,
        SafeTransitionState::DisclosedReducedTransitionSet
    ));
    assert_eq!(remote.derived_status, TransitionSafetyStatus::Yellow);
    assert!(remote.local_fallback.is_full());
    assert!(!remote.requires_waiver());
}

#[test]
fn unsafe_transition_blocks_the_ai_action() {
    let packet = seeded_m5_lifecycle_transition_safety_packet_ai_action_unsafe_transition_blocked();
    let row = packet.row(M5LifecycleObjectFamily::AiAction).unwrap();
    assert_eq!(row.derived_status, TransitionSafetyStatus::Red);
    assert!(!packet.report_clean);
    assert!(!packet.all_rows_publishable);
    assert!(packet.red_row_count >= 1);
    assert!(packet.blocking_findings.iter().any(|finding| matches!(
        finding,
        TransitionSafetyFinding::UnsafeOrMissingTransitionRules { .. }
    )));
    assert!(row.transition_causes.iter().any(|cause| matches!(
        cause.trigger,
        M5LifecycleDowngradeTrigger::StateVocabularyDrift
    )));
    assert!(validate_m5_lifecycle_transition_safety_packet(&packet).is_err());
}

#[test]
fn missing_attribution_blocks_the_request_run() {
    let packet = seeded_m5_lifecycle_transition_safety_packet_request_attribution_missing_blocked();
    let row = packet.row(M5LifecycleObjectFamily::RequestApiRun).unwrap();
    assert_eq!(row.derived_status, TransitionSafetyStatus::Red);
    assert!(!packet.report_clean);
    assert!(packet.blocking_findings.iter().any(|finding| matches!(
        finding,
        TransitionSafetyFinding::AttributionMissingOnTransition { .. }
    )));
    assert!(row.transition_causes.iter().any(|cause| matches!(
        cause.trigger,
        M5LifecycleDowngradeTrigger::LastFailureReasonMissing
    )));
    assert!(validate_m5_lifecycle_transition_safety_packet(&packet).is_err());
}

#[test]
fn skipped_checkpoint_blocks_the_update_rollback() {
    let packet = seeded_m5_lifecycle_transition_safety_packet_update_checkpoint_skipped_blocked();
    let row = packet.row(M5LifecycleObjectFamily::UpdateRollback).unwrap();
    assert_eq!(row.derived_status, TransitionSafetyStatus::Red);
    assert!(!packet.report_clean);
    assert!(packet.blocking_findings.iter().any(|finding| matches!(
        finding,
        TransitionSafetyFinding::RequiredCheckpointSkipped { .. }
    )));
    assert!(row.transition_causes.iter().any(|cause| matches!(
        cause.trigger,
        M5LifecycleDowngradeTrigger::AnonymousCheckpoint
    )));
    assert!(validate_m5_lifecycle_transition_safety_packet(&packet).is_err());
}

#[test]
fn lost_local_fallback_blocks_the_data_session() {
    let packet = seeded_m5_lifecycle_transition_safety_packet_data_local_fallback_lost_blocked();
    let row = packet.row(M5LifecycleObjectFamily::DataSession).unwrap();
    assert_eq!(row.derived_status, TransitionSafetyStatus::Red);
    assert!(!packet.report_clean);
    assert!(packet
        .blocking_findings
        .iter()
        .any(|finding| matches!(finding, TransitionSafetyFinding::LocalFallbackLost { .. })));
    assert!(row.transition_causes.iter().any(|cause| matches!(
        cause.trigger,
        M5LifecycleDowngradeTrigger::RecoveryAffordanceMissing
    )));
    assert!(validate_m5_lifecycle_transition_safety_packet(&packet).is_err());
}

#[test]
fn headless_parity_loss_blocks_the_extension() {
    let packet =
        seeded_m5_lifecycle_transition_safety_packet_extension_headless_parity_lost_blocked();
    let row = packet.row(M5LifecycleObjectFamily::Extension).unwrap();
    assert_eq!(row.derived_status, TransitionSafetyStatus::Red);
    assert!(!row.headless_parity_preserved);
    assert!(!packet.report_clean);
    assert!(packet
        .blocking_findings
        .iter()
        .any(|finding| matches!(finding, TransitionSafetyFinding::HeadlessParityLost { .. })));
    assert!(row.transition_causes.iter().any(|cause| matches!(
        cause.trigger,
        M5LifecycleDowngradeTrigger::StateVocabularyDrift
    )));
    assert!(validate_m5_lifecycle_transition_safety_packet(&packet).is_err());
}

#[test]
fn incomplete_consumer_surface_certification_blocks() {
    // Hand-mutate a green row so it certifies fewer than all declared consumer surfaces — the
    // completeness lint must block it.
    let mut packet = seeded_m5_lifecycle_transition_safety_packet();
    let row = packet
        .rows
        .iter_mut()
        .find(|row| row.object_family == M5LifecycleObjectFamily::Workspace)
        .unwrap();
    row.evaluated_consumer_surfaces.pop();
    assert!(!row.consumer_surfaces_complete());
    assert_eq!(row.recompute_status(), TransitionSafetyStatus::Red);
    let findings = row.compute_findings(&packet.generated_at);
    assert!(findings.iter().any(|finding| matches!(
        finding,
        TransitionSafetyFinding::ConsumerSurfacesIncomplete { .. }
    )));
}

#[test]
fn dashboard_projects_every_row_and_counts() {
    let packet = seeded_m5_lifecycle_transition_safety_packet();
    let dashboard = packet.dashboard();
    assert_eq!(dashboard.rows.len(), packet.rows.len());
    assert_eq!(dashboard.green_row_count, packet.green_row_count);
    assert_eq!(dashboard.yellow_row_count, packet.yellow_row_count);
    assert_eq!(dashboard.red_row_count, packet.red_row_count);
    assert_eq!(dashboard.all_rows_publishable, packet.all_rows_publishable);
    assert_eq!(dashboard.source_packet_ref, packet.packet_id);
    assert!(!dashboard.lifecycle_automation_refs.is_empty());

    let collaboration = dashboard
        .rows
        .iter()
        .find(|row| row.object_family == M5LifecycleObjectFamily::CollaborationSession)
        .unwrap();
    assert_eq!(collaboration.status, TransitionSafetyStatus::Yellow);
    assert!(collaboration.has_active_waiver);
    assert!(matches!(
        collaboration.local_fallback,
        LocalFallbackState::DisclosedReducedFallback
    ));
}

#[test]
fn support_export_quotes_packet_matrix_and_waiver_refs() {
    let packet = seeded_m5_lifecycle_transition_safety_packet();
    let export = TransitionSafetySupportExport::from_packet(
        M5_LIFECYCLE_TRANSITION_SAFETY_SUPPORT_EXPORT_ID,
        packet.clone(),
    );
    assert!(export.case_ids.contains(&packet.packet_id));
    assert!(export.case_ids.contains(&packet.matrix_packet_ref));
    assert!(export.case_ids.contains(&packet.build_identity_ref));
    for row in &packet.rows {
        assert!(export
            .case_ids
            .contains(&row.object_family.as_str().to_owned()));
    }
    for waiver in &packet.active_waivers {
        assert!(export.case_ids.contains(&waiver.waiver_id));
    }
    assert_eq!(export.dashboard, packet.dashboard());
}

#[test]
fn markdown_and_csv_name_every_object_family() {
    let packet = seeded_m5_lifecycle_transition_safety_packet();
    let markdown = packet.render_markdown();
    let csv = packet.render_matrix_csv();
    for object_family in REQUIRED_OBJECT_FAMILIES {
        assert!(
            csv.contains(object_family.as_str()),
            "csv omits {}",
            object_family.as_str()
        );
    }
    assert!(markdown.contains("m5_lifecycle_transition_safety_fixtures"));
    assert!(markdown.contains("waiver:collaboration-reduced-fallback:0001"));
}

#[test]
fn export_carries_no_forbidden_material() {
    let packet = seeded_m5_lifecycle_transition_safety_packet();
    let json = packet.export_safe_json().to_lowercase();
    assert!(!json.contains("api_key"));
    assert!(!json.contains("password"));
    assert!(!json.contains("bearer "));
    assert!(!json.contains("://"));
}

#[test]
fn expired_waiver_is_flagged() {
    let waiver = TransitionSafetyWaiver {
        waiver_id: "waiver:expired:0001".to_owned(),
        object_family: M5LifecycleObjectFamily::CollaborationSession,
        reason: "test".to_owned(),
        owner_role: "owner".to_owned(),
        expires_at: "2026-01-01T00:00:00Z".to_owned(),
    };
    assert!(!waiver.is_active_at("2026-06-30T00:00:00Z"));
    assert!(waiver.is_active_at("2025-01-01T00:00:00Z"));
}
