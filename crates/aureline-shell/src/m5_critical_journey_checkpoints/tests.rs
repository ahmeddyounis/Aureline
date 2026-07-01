//! Inline unit tests for the M5 critical-journey checkpoint proof.

use super::*;

#[test]
fn seeded_packet_covers_every_protected_journey_and_is_clean() {
    let packet = seeded_m5_critical_journey_checkpoints_packet();
    validate_m5_critical_journey_checkpoints_packet(&packet)
        .expect("seeded packet must validate clean");

    assert_eq!(packet.rows.len(), REQUIRED_PROTECTED_JOURNEYS.len());
    for journey in REQUIRED_PROTECTED_JOURNEYS {
        assert!(
            packet.row(journey).is_some(),
            "missing row for {}",
            journey.as_str()
        );
    }
    assert!(packet.report_clean);
    assert!(packet.blocking_findings.is_empty());
    assert_eq!(packet.red_row_count, 0);
    assert!(packet.all_rows_publishable);
}

#[test]
fn seeded_packet_has_two_green_and_three_yellow_rows() {
    let packet = seeded_m5_critical_journey_checkpoints_packet();
    assert_eq!(packet.green_row_count, 2);
    assert_eq!(packet.yellow_row_count, 3);
    assert_eq!(packet.red_row_count, 0);

    for journey in [
        M5ProtectedJourney::LargeRepoOpen,
        M5ProtectedJourney::RemoteAttachRun,
        M5ProtectedJourney::CollaborationJoinFollow,
    ] {
        assert_eq!(
            packet.row(journey).unwrap().derived_status,
            CriticalJourneyStatus::Yellow,
            "{} should auto-narrow to yellow",
            journey.as_str()
        );
    }
    for journey in [
        M5ProtectedJourney::WarmStartup,
        M5ProtectedJourney::AiMultiFileApply,
    ] {
        assert_eq!(
            packet.row(journey).unwrap().derived_status,
            CriticalJourneyStatus::Green,
            "{} should be green",
            journey.as_str()
        );
    }
}

#[test]
fn every_row_status_is_the_derived_value() {
    let packet = seeded_m5_critical_journey_checkpoints_packet();
    for row in &packet.rows {
        assert_eq!(
            row.derived_status,
            row.recompute_status(),
            "row {} declares a stale status",
            row.journey.as_str()
        );
        assert_eq!(row.journey_causes, row.recompute_causes());
    }
}

#[test]
fn every_row_shows_a_well_formed_checkpoint_sequence_replacing_a_spinner() {
    let packet = seeded_m5_critical_journey_checkpoints_packet();
    for row in &packet.rows {
        assert!(
            row.checkpoint_sequence_well_formed(),
            "row {} does not show a well-formed checkpoint sequence",
            row.journey.as_str()
        );
        assert!(
            row.checkpoint_sequence.len() >= 2,
            "row {} shows fewer than two milestones",
            row.journey.as_str()
        );
        assert!(
            row.checkpoint_sequence.last().unwrap().is_terminal(),
            "row {} does not end on a terminal milestone",
            row.journey.as_str()
        );
    }
}

#[test]
fn every_row_certifies_all_declared_consumer_surfaces() {
    let packet = seeded_m5_critical_journey_checkpoints_packet();
    for row in &packet.rows {
        assert!(
            row.consumer_surfaces_complete(),
            "row {} does not certify all declared consumer surfaces",
            row.journey.as_str()
        );
        assert!(row.headless_parity_preserved);
    }
}

#[test]
fn every_binding_is_pulled_from_the_frozen_matrix() {
    use crate::freeze_the_m5_lifecycle_state_and_journey_checkpoint_matrix::seeded_m5_lifecycle_matrix;

    let matrix = seeded_m5_lifecycle_matrix();
    let packet = seeded_m5_critical_journey_checkpoints_packet();
    for row in &packet.rows {
        let source = matrix
            .object_state_rows
            .iter()
            .find(|object| object.object_family == row.object_family)
            .expect("driving object family is frozen by the matrix");
        assert_eq!(row.admitted_states, source.admitted_states);
        assert_eq!(row.recovery_affordance, source.recovery_affordance);
        assert_eq!(row.qualification, source.qualification);
        assert_eq!(row.required_consumer_surfaces, source.consumer_surfaces);
        assert_eq!(row.applicable_downgrade_triggers, source.downgrade_triggers);
        assert_eq!(
            row.last_failure_reason_classes,
            source.last_failure_reason_classes
        );
        // The explicit state machine must always admit `ready`, and the success terminal is ready.
        assert!(row.admitted_states.contains(&M5LifecycleState::Ready));
        assert_eq!(row.success_state, M5LifecycleState::Ready);
    }
}

#[test]
fn matrix_bound_journeys_name_a_frozen_matrix_journey() {
    let packet = seeded_m5_critical_journey_checkpoints_packet();
    assert_eq!(
        packet
            .row(M5ProtectedJourney::WarmStartup)
            .unwrap()
            .matrix_journey,
        Some(M5CriticalJourney::WorkspaceRestore)
    );
    assert_eq!(
        packet
            .row(M5ProtectedJourney::AiMultiFileApply)
            .unwrap()
            .matrix_journey,
        Some(M5CriticalJourney::AiActionRun)
    );
    assert_eq!(
        packet
            .row(M5ProtectedJourney::RemoteAttachRun)
            .unwrap()
            .matrix_journey,
        Some(M5CriticalJourney::RemoteReconnect)
    );
    assert_eq!(
        packet
            .row(M5ProtectedJourney::CollaborationJoinFollow)
            .unwrap()
            .matrix_journey,
        Some(M5CriticalJourney::CollaborationJoin)
    );
    // Large-repo open has no frozen matrix journey; it anchors on the workspace object family.
    assert_eq!(
        packet
            .row(M5ProtectedJourney::LargeRepoOpen)
            .unwrap()
            .matrix_journey,
        None
    );
    assert_eq!(
        packet
            .row(M5ProtectedJourney::LargeRepoOpen)
            .unwrap()
            .object_family,
        M5LifecycleObjectFamily::Workspace
    );
}

#[test]
fn narrowed_rows_disclose_a_reason() {
    let packet = seeded_m5_critical_journey_checkpoints_packet();
    for row in &packet.rows {
        if !matches!(row.derived_status, CriticalJourneyStatus::Green) {
            assert!(
                row.narrowing_reason
                    .as_deref()
                    .map(|reason| !reason.trim().is_empty())
                    .unwrap_or(false),
                "narrowed row {} hides its reason",
                row.journey.as_str()
            );
        }
    }
}

#[test]
fn disclosed_reduced_next_action_carries_an_active_waiver() {
    let packet = seeded_m5_critical_journey_checkpoints_packet();
    let collaboration = packet
        .row(M5ProtectedJourney::CollaborationJoinFollow)
        .unwrap();
    assert!(matches!(
        collaboration.place_continuity,
        PlaceContinuityState::DisclosedReducedNextAction
    ));
    assert!(collaboration.requires_waiver());
    assert!(collaboration.has_active_waiver());
    assert_eq!(packet.active_waivers.len(), 1);
    assert!(packet.active_waivers[0].is_active_at(&packet.generated_at));
}

#[test]
fn disclosed_compacted_milestones_keeps_remote_yellow_without_a_waiver() {
    let packet = seeded_m5_critical_journey_checkpoints_packet();
    let remote = packet.row(M5ProtectedJourney::RemoteAttachRun).unwrap();
    assert!(matches!(
        remote.checkpoint_visibility,
        CheckpointVisibilityState::DisclosedCompactedMilestones
    ));
    assert_eq!(remote.derived_status, CriticalJourneyStatus::Yellow);
    assert!(remote.place_continuity.is_full());
    assert!(!remote.requires_waiver());
}

#[test]
fn anonymous_spinner_blocks_the_warm_startup() {
    let packet =
        seeded_m5_critical_journey_checkpoints_packet_warm_startup_anonymous_spinner_blocked();
    let row = packet.row(M5ProtectedJourney::WarmStartup).unwrap();
    assert_eq!(row.derived_status, CriticalJourneyStatus::Red);
    assert!(!packet.report_clean);
    assert!(!packet.all_rows_publishable);
    assert!(packet.red_row_count >= 1);
    assert!(packet.blocking_findings.iter().any(|finding| matches!(
        finding,
        CriticalJourneyFinding::AnonymousSpinnerShown { .. }
    )));
    assert!(row.journey_causes.iter().any(|cause| matches!(
        cause.trigger,
        M5LifecycleDowngradeTrigger::AnonymousCheckpoint
    )));
    assert!(validate_m5_critical_journey_checkpoints_packet(&packet).is_err());
}

#[test]
fn unlabeled_partial_state_blocks_the_large_repo_open() {
    let packet =
        seeded_m5_critical_journey_checkpoints_packet_large_repo_partial_unlabeled_blocked();
    let row = packet.row(M5ProtectedJourney::LargeRepoOpen).unwrap();
    assert_eq!(row.derived_status, CriticalJourneyStatus::Red);
    assert!(!packet.report_clean);
    assert!(packet.blocking_findings.iter().any(|finding| matches!(
        finding,
        CriticalJourneyFinding::PartialStateUnlabeled { .. }
    )));
    assert!(row.journey_causes.iter().any(|cause| matches!(
        cause.trigger,
        M5LifecycleDowngradeTrigger::LastFailureReasonMissing
    )));
    assert!(validate_m5_critical_journey_checkpoints_packet(&packet).is_err());
}

#[test]
fn lost_place_blocks_the_collaboration_join_follow() {
    let packet = seeded_m5_critical_journey_checkpoints_packet_collaboration_place_lost_blocked();
    let row = packet
        .row(M5ProtectedJourney::CollaborationJoinFollow)
        .unwrap();
    assert_eq!(row.derived_status, CriticalJourneyStatus::Red);
    assert!(!packet.report_clean);
    assert!(packet
        .blocking_findings
        .iter()
        .any(|finding| matches!(finding, CriticalJourneyFinding::PlaceOrRecoveryLost { .. })));
    assert!(row.journey_causes.iter().any(|cause| matches!(
        cause.trigger,
        M5LifecycleDowngradeTrigger::RecoveryAffordanceMissing
    )));
    assert!(validate_m5_critical_journey_checkpoints_packet(&packet).is_err());
}

#[test]
fn checkpoints_absent_from_capture_blocks_the_ai_apply() {
    let packet = seeded_m5_critical_journey_checkpoints_packet_ai_apply_capture_absent_blocked();
    let row = packet.row(M5ProtectedJourney::AiMultiFileApply).unwrap();
    assert_eq!(row.derived_status, CriticalJourneyStatus::Red);
    assert!(!packet.report_clean);
    assert!(packet.blocking_findings.iter().any(|finding| matches!(
        finding,
        CriticalJourneyFinding::CheckpointsAbsentFromCapture { .. }
    )));
    assert!(row.journey_causes.iter().any(|cause| matches!(
        cause.trigger,
        M5LifecycleDowngradeTrigger::StatusCodeUnexportable
    )));
    assert!(validate_m5_critical_journey_checkpoints_packet(&packet).is_err());
}

#[test]
fn headless_parity_loss_blocks_the_remote_attach_run() {
    let packet =
        seeded_m5_critical_journey_checkpoints_packet_remote_headless_parity_lost_blocked();
    let row = packet.row(M5ProtectedJourney::RemoteAttachRun).unwrap();
    assert_eq!(row.derived_status, CriticalJourneyStatus::Red);
    assert!(!row.headless_parity_preserved);
    assert!(!packet.report_clean);
    assert!(packet
        .blocking_findings
        .iter()
        .any(|finding| matches!(finding, CriticalJourneyFinding::HeadlessParityLost { .. })));
    assert!(row.journey_causes.iter().any(|cause| matches!(
        cause.trigger,
        M5LifecycleDowngradeTrigger::StateVocabularyDrift
    )));
    assert!(validate_m5_critical_journey_checkpoints_packet(&packet).is_err());
}

#[test]
fn incomplete_consumer_surface_certification_blocks() {
    // Hand-mutate a green row so it certifies fewer than all declared consumer surfaces — the
    // completeness lint must block it.
    let mut packet = seeded_m5_critical_journey_checkpoints_packet();
    let row = packet
        .rows
        .iter_mut()
        .find(|row| row.journey == M5ProtectedJourney::WarmStartup)
        .unwrap();
    row.evaluated_consumer_surfaces.pop();
    assert!(!row.consumer_surfaces_complete());
    assert_eq!(row.recompute_status(), CriticalJourneyStatus::Red);
    let findings = row.compute_findings(&packet.generated_at);
    assert!(findings.iter().any(|finding| matches!(
        finding,
        CriticalJourneyFinding::ConsumerSurfacesIncomplete { .. }
    )));
}

#[test]
fn malformed_checkpoint_sequence_blocks() {
    // A single-milestone sequence with no terminal cannot prove named milestones — it blocks.
    let mut packet = seeded_m5_critical_journey_checkpoints_packet();
    let row = packet
        .rows
        .iter_mut()
        .find(|row| row.journey == M5ProtectedJourney::WarmStartup)
        .unwrap();
    row.checkpoint_sequence = vec![M5JourneyCheckpoint::Preparing];
    assert!(!row.checkpoint_sequence_well_formed());
    assert_eq!(row.recompute_status(), CriticalJourneyStatus::Red);
    let findings = row.compute_findings(&packet.generated_at);
    assert!(findings.iter().any(|finding| matches!(
        finding,
        CriticalJourneyFinding::CheckpointSequenceMalformed { .. }
    )));
}

#[test]
fn dashboard_projects_every_row_and_counts() {
    let packet = seeded_m5_critical_journey_checkpoints_packet();
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
        .find(|row| row.journey == M5ProtectedJourney::CollaborationJoinFollow)
        .unwrap();
    assert_eq!(collaboration.status, CriticalJourneyStatus::Yellow);
    assert!(collaboration.has_active_waiver);
    assert!(matches!(
        collaboration.place_continuity,
        PlaceContinuityState::DisclosedReducedNextAction
    ));
}

#[test]
fn support_export_quotes_packet_matrix_and_waiver_refs() {
    let packet = seeded_m5_critical_journey_checkpoints_packet();
    let export = CriticalJourneySupportExport::from_packet(
        M5_CRITICAL_JOURNEY_CHECKPOINTS_SUPPORT_EXPORT_ID,
        packet.clone(),
    );
    assert!(export.case_ids.contains(&packet.packet_id));
    assert!(export.case_ids.contains(&packet.matrix_packet_ref));
    assert!(export.case_ids.contains(&packet.build_identity_ref));
    for row in &packet.rows {
        assert!(export.case_ids.contains(&row.journey.as_str().to_owned()));
    }
    for waiver in &packet.active_waivers {
        assert!(export.case_ids.contains(&waiver.waiver_id));
    }
    assert_eq!(export.dashboard, packet.dashboard());
}

#[test]
fn markdown_and_csv_name_every_protected_journey() {
    let packet = seeded_m5_critical_journey_checkpoints_packet();
    let markdown = packet.render_markdown();
    let csv = packet.render_matrix_csv();
    for journey in REQUIRED_PROTECTED_JOURNEYS {
        assert!(
            csv.contains(journey.as_str()),
            "csv omits {}",
            journey.as_str()
        );
    }
    assert!(markdown.contains("m5_critical_journey_checkpoints_fixtures"));
    assert!(markdown.contains("waiver:collaboration-reduced-next-action:0001"));
}

#[test]
fn export_carries_no_forbidden_material() {
    let packet = seeded_m5_critical_journey_checkpoints_packet();
    let json = packet.export_safe_json().to_lowercase();
    assert!(!json.contains("api_key"));
    assert!(!json.contains("password"));
    assert!(!json.contains("bearer "));
    assert!(!json.contains("://"));
}

#[test]
fn expired_waiver_is_flagged() {
    let waiver = CriticalJourneyWaiver {
        waiver_id: "waiver:expired:0001".to_owned(),
        journey: M5ProtectedJourney::CollaborationJoinFollow,
        reason: "test".to_owned(),
        owner_role: "owner".to_owned(),
        expires_at: "2026-01-01T00:00:00Z".to_owned(),
    };
    assert!(!waiver.is_active_at("2026-06-30T00:00:00Z"));
    assert!(waiver.is_active_at("2025-01-01T00:00:00Z"));
}
