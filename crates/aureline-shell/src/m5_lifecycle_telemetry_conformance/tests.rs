//! Inline unit tests for the M5 lifecycle-telemetry-conformance proof.

use super::*;

#[test]
fn seeded_packet_covers_every_object_family_and_is_clean() {
    let packet = seeded_m5_lifecycle_telemetry_conformance_packet();
    validate_m5_lifecycle_telemetry_conformance_packet(&packet)
        .expect("seeded packet must validate clean");

    assert_eq!(packet.rows.len(), REQUIRED_OBJECT_FAMILIES.len());
    for family in REQUIRED_OBJECT_FAMILIES {
        assert!(
            packet.row(family).is_some(),
            "missing row for {}",
            family.as_str()
        );
    }
    assert!(packet.report_clean);
    assert!(packet.blocking_findings.is_empty());
    assert_eq!(packet.red_row_count, 0);
    assert!(packet.all_rows_publishable);
}

#[test]
fn seeded_packet_has_nine_green_and_four_yellow_rows() {
    let packet = seeded_m5_lifecycle_telemetry_conformance_packet();
    assert_eq!(packet.green_row_count, 9);
    assert_eq!(packet.yellow_row_count, 4);
    assert_eq!(packet.red_row_count, 0);

    for family in [
        M5LifecycleObjectFamily::ProfilerCapture,
        M5LifecycleObjectFamily::PipelineRun,
        M5LifecycleObjectFamily::PreviewSession,
        M5LifecycleObjectFamily::CompanionSession,
    ] {
        assert_eq!(
            packet.row(family).unwrap().derived_status,
            TelemetryConformanceStatus::Yellow,
            "{} should auto-narrow to yellow",
            family.as_str()
        );
    }
}

#[test]
fn every_row_status_is_the_derived_value() {
    let packet = seeded_m5_lifecycle_telemetry_conformance_packet();
    for row in &packet.rows {
        assert_eq!(
            row.derived_status,
            row.recompute_status(),
            "row {} declares a stale status",
            row.object_family.as_str()
        );
        assert_eq!(row.conformance_causes, row.recompute_causes());
    }
}

#[test]
fn every_row_emits_all_sinks_and_keeps_all_mandatory_fields() {
    let packet = seeded_m5_lifecycle_telemetry_conformance_packet();
    for row in &packet.rows {
        assert!(
            row.telemetry_sinks_complete(),
            "row {} does not emit into all four telemetry sinks",
            row.object_family.as_str()
        );
        assert!(
            row.mandatory_fields_complete(),
            "row {} does not keep all three mandatory fields conformant",
            row.object_family.as_str()
        );
        assert_eq!(row.emitted_telemetry_sinks.len(), 4);
        assert_eq!(row.conformant_mandatory_fields.len(), 3);
    }
}

#[test]
fn every_row_certifies_all_declared_consumer_surfaces() {
    let packet = seeded_m5_lifecycle_telemetry_conformance_packet();
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
fn every_binding_is_pulled_from_the_frozen_matrix() {
    use crate::freeze_the_m5_lifecycle_state_and_journey_checkpoint_matrix::seeded_m5_lifecycle_matrix;

    let matrix = seeded_m5_lifecycle_matrix();
    let packet = seeded_m5_lifecycle_telemetry_conformance_packet();
    for row in &packet.rows {
        let object = matrix
            .object_state_rows
            .iter()
            .find(|object| object.object_family == row.object_family)
            .expect("driving object family is frozen by the matrix");
        let journey = matrix
            .journey_checkpoint_rows
            .iter()
            .find(|journey| journey.object_family == row.object_family)
            .expect("driving journey is frozen by the matrix");
        assert_eq!(row.admitted_states, object.admitted_states);
        assert_eq!(row.primary_status_surface, object.primary_status_surface);
        assert_eq!(row.status_code_export_field, object.status_code_export_field);
        assert_eq!(row.last_failure_reason_field, object.last_failure_reason_field);
        assert_eq!(row.recovery_affordance, object.recovery_affordance);
        assert_eq!(row.qualification, object.qualification);
        assert_eq!(row.required_consumer_surfaces, object.consumer_surfaces);
        assert_eq!(row.applicable_downgrade_triggers, object.downgrade_triggers);
        assert_eq!(
            row.last_failure_reason_classes,
            object.last_failure_reason_classes
        );
        assert_eq!(row.matrix_journey, journey.journey);
        assert_eq!(row.checkpoint_lineage, journey.checkpoints);
        // The explicit state machine must always admit `ready`.
        assert!(row.admitted_states.contains(&M5LifecycleState::Ready));
    }
}

#[test]
fn narrowed_rows_disclose_a_reason() {
    let packet = seeded_m5_lifecycle_telemetry_conformance_packet();
    for row in &packet.rows {
        if !matches!(row.derived_status, TelemetryConformanceStatus::Green) {
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
fn disclosed_export_field_narrowing_carries_an_active_waiver() {
    let packet = seeded_m5_lifecycle_telemetry_conformance_packet();
    let companion = packet
        .row(M5LifecycleObjectFamily::CompanionSession)
        .unwrap();
    assert!(matches!(
        companion.ui_export_parity,
        UiExportParityState::DisclosedExportFieldNarrowing
    ));
    assert!(companion.requires_waiver());
    assert!(companion.has_active_waiver());
    assert_eq!(packet.active_waivers.len(), 1);
    assert!(packet.active_waivers[0].is_active_at(&packet.generated_at));
}

#[test]
fn disclosed_coarse_transition_events_keep_pipeline_yellow_without_a_waiver() {
    let packet = seeded_m5_lifecycle_telemetry_conformance_packet();
    let pipeline = packet.row(M5LifecycleObjectFamily::PipelineRun).unwrap();
    assert!(matches!(
        pipeline.transition_event,
        TransitionEventEmissionState::DisclosedCoarseTransitionEvents
    ));
    assert_eq!(pipeline.derived_status, TelemetryConformanceStatus::Yellow);
    assert!(pipeline.ui_export_parity.is_full());
    assert!(!pipeline.requires_waiver());
}

#[test]
fn absent_enums_block_the_notebook_runtime() {
    let packet = seeded_m5_lifecycle_telemetry_conformance_packet_notebook_enums_absent_blocked();
    let row = packet
        .row(M5LifecycleObjectFamily::NotebookRuntime)
        .unwrap();
    assert_eq!(row.derived_status, TelemetryConformanceStatus::Red);
    assert!(!packet.report_clean);
    assert!(!packet.all_rows_publishable);
    assert!(packet.red_row_count >= 1);
    assert!(packet.blocking_findings.iter().any(|finding| matches!(
        finding,
        TelemetryConformanceFinding::EnumsAbsentOrLocalProse { .. }
    )));
    assert!(row.conformance_causes.iter().any(|cause| matches!(
        cause.trigger,
        M5LifecycleDowngradeTrigger::StateVocabularyDrift
    )));
    assert!(validate_m5_lifecycle_telemetry_conformance_packet(&packet).is_err());
}

#[test]
fn missing_transition_events_block_the_remote_session() {
    let packet =
        seeded_m5_lifecycle_telemetry_conformance_packet_remote_transition_events_missing_blocked();
    let row = packet.row(M5LifecycleObjectFamily::RemoteSession).unwrap();
    assert_eq!(row.derived_status, TelemetryConformanceStatus::Red);
    assert!(!packet.report_clean);
    assert!(packet.blocking_findings.iter().any(|finding| matches!(
        finding,
        TelemetryConformanceFinding::TransitionEventsMissing { .. }
    )));
    assert!(row.conformance_causes.iter().any(|cause| matches!(
        cause.trigger,
        M5LifecycleDowngradeTrigger::AnonymousCheckpoint
    )));
    assert!(validate_m5_lifecycle_telemetry_conformance_packet(&packet).is_err());
}

#[test]
fn ui_export_drift_blocks_the_data_session() {
    let packet = seeded_m5_lifecycle_telemetry_conformance_packet_data_ui_export_drift_blocked();
    let row = packet.row(M5LifecycleObjectFamily::DataSession).unwrap();
    assert_eq!(row.derived_status, TelemetryConformanceStatus::Red);
    assert!(!packet.report_clean);
    assert!(packet
        .blocking_findings
        .iter()
        .any(|finding| matches!(finding, TelemetryConformanceFinding::UiExportDrift { .. })));
    assert!(row.conformance_causes.iter().any(|cause| matches!(
        cause.trigger,
        M5LifecycleDowngradeTrigger::StatusCodeUnexportable
    )));
    assert!(validate_m5_lifecycle_telemetry_conformance_packet(&packet).is_err());
}

#[test]
fn shared_contract_local_prose_blocks_the_ai_action() {
    let packet =
        seeded_m5_lifecycle_telemetry_conformance_packet_ai_shared_contract_local_prose_blocked();
    let row = packet.row(M5LifecycleObjectFamily::AiAction).unwrap();
    assert_eq!(row.derived_status, TelemetryConformanceStatus::Red);
    assert!(!packet.report_clean);
    assert!(packet.blocking_findings.iter().any(|finding| matches!(
        finding,
        TelemetryConformanceFinding::SharedContractLocalProse { .. }
    )));
    assert!(row.conformance_causes.iter().any(|cause| matches!(
        cause.trigger,
        M5LifecycleDowngradeTrigger::StateVocabularyDrift
    )));
    assert!(validate_m5_lifecycle_telemetry_conformance_packet(&packet).is_err());
}

#[test]
fn headless_parity_loss_blocks_the_extension() {
    let packet =
        seeded_m5_lifecycle_telemetry_conformance_packet_extension_headless_parity_lost_blocked();
    let row = packet.row(M5LifecycleObjectFamily::Extension).unwrap();
    assert_eq!(row.derived_status, TelemetryConformanceStatus::Red);
    assert!(!row.headless_parity_preserved);
    assert!(!packet.report_clean);
    assert!(packet
        .blocking_findings
        .iter()
        .any(|finding| matches!(finding, TelemetryConformanceFinding::HeadlessParityLost { .. })));
    assert!(row.conformance_causes.iter().any(|cause| matches!(
        cause.trigger,
        M5LifecycleDowngradeTrigger::StateVocabularyDrift
    )));
    assert!(validate_m5_lifecycle_telemetry_conformance_packet(&packet).is_err());
}

#[test]
fn incomplete_telemetry_sink_set_blocks() {
    // Hand-mutate a green row so it emits into fewer than all four sinks — the completeness lint must
    // block it.
    let mut packet = seeded_m5_lifecycle_telemetry_conformance_packet();
    let row = packet
        .rows
        .iter_mut()
        .find(|row| row.object_family == M5LifecycleObjectFamily::Workspace)
        .unwrap();
    row.emitted_telemetry_sinks.pop();
    assert!(!row.telemetry_sinks_complete());
    assert_eq!(row.recompute_status(), TelemetryConformanceStatus::Red);
    let findings = row.compute_findings(&packet.generated_at);
    assert!(findings.iter().any(|finding| matches!(
        finding,
        TelemetryConformanceFinding::TelemetrySinksIncomplete { .. }
    )));
}

#[test]
fn incomplete_mandatory_field_set_blocks() {
    let mut packet = seeded_m5_lifecycle_telemetry_conformance_packet();
    let row = packet
        .rows
        .iter_mut()
        .find(|row| row.object_family == M5LifecycleObjectFamily::Workspace)
        .unwrap();
    row.conformant_mandatory_fields.pop();
    assert!(!row.mandatory_fields_complete());
    assert_eq!(row.recompute_status(), TelemetryConformanceStatus::Red);
    let findings = row.compute_findings(&packet.generated_at);
    assert!(findings.iter().any(|finding| matches!(
        finding,
        TelemetryConformanceFinding::MandatoryFieldsIncomplete { .. }
    )));
}

#[test]
fn incomplete_consumer_surface_certification_blocks() {
    let mut packet = seeded_m5_lifecycle_telemetry_conformance_packet();
    let row = packet
        .rows
        .iter_mut()
        .find(|row| row.object_family == M5LifecycleObjectFamily::Workspace)
        .unwrap();
    row.evaluated_consumer_surfaces.pop();
    assert!(!row.consumer_surfaces_complete());
    assert_eq!(row.recompute_status(), TelemetryConformanceStatus::Red);
    let findings = row.compute_findings(&packet.generated_at);
    assert!(findings.iter().any(|finding| matches!(
        finding,
        TelemetryConformanceFinding::ConsumerSurfacesIncomplete { .. }
    )));
}

#[test]
fn dashboard_projects_every_row_and_counts() {
    let packet = seeded_m5_lifecycle_telemetry_conformance_packet();
    let dashboard = packet.dashboard();
    assert_eq!(dashboard.rows.len(), packet.rows.len());
    assert_eq!(dashboard.green_row_count, packet.green_row_count);
    assert_eq!(dashboard.yellow_row_count, packet.yellow_row_count);
    assert_eq!(dashboard.red_row_count, packet.red_row_count);
    assert_eq!(dashboard.all_rows_publishable, packet.all_rows_publishable);
    assert_eq!(dashboard.source_packet_ref, packet.packet_id);
    assert!(!dashboard.lifecycle_automation_refs.is_empty());

    let companion = dashboard
        .rows
        .iter()
        .find(|row| row.object_family == M5LifecycleObjectFamily::CompanionSession)
        .unwrap();
    assert_eq!(companion.status, TelemetryConformanceStatus::Yellow);
    assert!(companion.has_active_waiver);
    assert!(matches!(
        companion.ui_export_parity,
        UiExportParityState::DisclosedExportFieldNarrowing
    ));
}

#[test]
fn support_export_quotes_packet_matrix_and_waiver_refs() {
    let packet = seeded_m5_lifecycle_telemetry_conformance_packet();
    let export = TelemetryConformanceSupportExport::from_packet(
        M5_LIFECYCLE_TELEMETRY_CONFORMANCE_SUPPORT_EXPORT_ID,
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
    let packet = seeded_m5_lifecycle_telemetry_conformance_packet();
    let markdown = packet.render_markdown();
    let csv = packet.render_matrix_csv();
    for family in REQUIRED_OBJECT_FAMILIES {
        assert!(
            csv.contains(family.as_str()),
            "csv omits {}",
            family.as_str()
        );
    }
    assert!(markdown.contains("m5_lifecycle_telemetry_conformance_fixtures"));
    assert!(markdown.contains("waiver:companion-export-field-narrowing:0001"));
}

#[test]
fn export_carries_no_forbidden_material() {
    let packet = seeded_m5_lifecycle_telemetry_conformance_packet();
    let json = packet.export_safe_json().to_lowercase();
    assert!(!json.contains("api_key"));
    assert!(!json.contains("password"));
    assert!(!json.contains("bearer "));
    assert!(!json.contains("://"));
}

#[test]
fn expired_waiver_is_flagged() {
    let waiver = TelemetryConformanceWaiver {
        waiver_id: "waiver:expired:0001".to_owned(),
        object_family: M5LifecycleObjectFamily::CompanionSession,
        reason: "test".to_owned(),
        owner_role: "owner".to_owned(),
        expires_at: "2026-01-01T00:00:00Z".to_owned(),
    };
    assert!(!waiver.is_active_at("2026-06-30T00:00:00Z"));
    assert!(waiver.is_active_at("2025-01-01T00:00:00Z"));
}
