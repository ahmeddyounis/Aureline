//! Inline unit tests for the M5 lifecycle-object certification proof.

use super::*;

#[test]
fn seeded_packet_covers_every_object_family_and_is_clean() {
    let packet = seeded_m5_lifecycle_object_certification_packet();
    validate_m5_lifecycle_object_certification_packet(&packet)
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
    let packet = seeded_m5_lifecycle_object_certification_packet();
    assert_eq!(packet.green_row_count, 9);
    assert_eq!(packet.yellow_row_count, 4);
    assert_eq!(packet.red_row_count, 0);

    for object_family in [
        M5LifecycleObjectFamily::CompanionSession,
        M5LifecycleObjectFamily::ProfilerCapture,
        M5LifecycleObjectFamily::AiAction,
        M5LifecycleObjectFamily::PreviewSession,
    ] {
        assert_eq!(
            packet.row(object_family).unwrap().derived_status,
            LifecycleObjectStatus::Yellow,
            "{} should auto-narrow to yellow",
            object_family.as_str()
        );
    }
}

#[test]
fn every_row_status_is_the_derived_value() {
    let packet = seeded_m5_lifecycle_object_certification_packet();
    for row in &packet.rows {
        assert_eq!(
            row.derived_status,
            row.recompute_status(),
            "row {} declares a stale status",
            row.object_family.as_str()
        );
        assert_eq!(row.object_causes, row.recompute_causes());
    }
}

#[test]
fn every_row_certifies_all_declared_consumer_surfaces() {
    let packet = seeded_m5_lifecycle_object_certification_packet();
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
    let packet = seeded_m5_lifecycle_object_certification_packet();
    for matrix_row in &matrix.object_state_rows {
        let row = packet.row(matrix_row.object_family).unwrap();
        assert_eq!(
            row.primary_status_surface,
            matrix_row.primary_status_surface
        );
        assert_eq!(
            row.status_code_export_field,
            matrix_row.status_code_export_field
        );
        assert_eq!(
            row.last_failure_reason_field,
            matrix_row.last_failure_reason_field
        );
        assert_eq!(row.recovery_affordance, matrix_row.recovery_affordance);
        assert_eq!(row.qualification, matrix_row.qualification);
        assert_eq!(row.required_consumer_surfaces, matrix_row.consumer_surfaces);
    }
}

#[test]
fn narrowed_rows_disclose_a_reason() {
    let packet = seeded_m5_lifecycle_object_certification_packet();
    for row in &packet.rows {
        if !matches!(row.derived_status, LifecycleObjectStatus::Green) {
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
fn disclosed_surface_relocation_carries_an_active_waiver() {
    let packet = seeded_m5_lifecycle_object_certification_packet();
    let companion = packet
        .row(M5LifecycleObjectFamily::CompanionSession)
        .unwrap();
    assert!(matches!(
        companion.status_surface_binding,
        StatusSurfaceBindingState::DisclosedSurfaceRelocation
    ));
    assert!(companion.requires_waiver());
    assert!(companion.has_active_waiver());
    assert_eq!(packet.active_waivers.len(), 1);
    assert!(packet.active_waivers[0].is_active_at(&packet.generated_at));
}

#[test]
fn disclosed_partial_export_keeps_profiler_yellow_without_a_waiver() {
    let packet = seeded_m5_lifecycle_object_certification_packet();
    let profiler = packet
        .row(M5LifecycleObjectFamily::ProfilerCapture)
        .unwrap();
    assert!(matches!(
        profiler.status_code_export,
        StatusCodeExportState::DisclosedPartialExport
    ));
    assert_eq!(profiler.derived_status, LifecycleObjectStatus::Yellow);
    assert!(profiler.status_surface_binding.is_full());
    assert!(!profiler.requires_waiver());
}

#[test]
fn lost_status_surface_blocks_the_notebook_runtime() {
    let packet =
        seeded_m5_lifecycle_object_certification_packet_notebook_status_surface_missing_blocked();
    let row = packet
        .row(M5LifecycleObjectFamily::NotebookRuntime)
        .unwrap();
    assert_eq!(row.derived_status, LifecycleObjectStatus::Red);
    assert!(!packet.report_clean);
    assert!(!packet.all_rows_publishable);
    assert!(packet.red_row_count >= 1);
    assert!(packet.blocking_findings.iter().any(|finding| matches!(
        finding,
        LifecycleObjectFinding::StatusSurfaceMissingOrSplit { .. }
    )));
    assert!(row.object_causes.iter().any(|cause| matches!(
        cause.trigger,
        M5LifecycleDowngradeTrigger::StatusSurfaceMissing
    )));
    assert!(validate_m5_lifecycle_object_certification_packet(&packet).is_err());
}

#[test]
fn unexportable_status_code_blocks_the_request_run() {
    let packet =
        seeded_m5_lifecycle_object_certification_packet_request_status_code_unexportable_blocked();
    let row = packet.row(M5LifecycleObjectFamily::RequestApiRun).unwrap();
    assert_eq!(row.derived_status, LifecycleObjectStatus::Red);
    assert!(!packet.report_clean);
    assert!(packet.blocking_findings.iter().any(|finding| matches!(
        finding,
        LifecycleObjectFinding::StatusCodeUnexportable { .. }
    )));
    assert!(row.object_causes.iter().any(|cause| matches!(
        cause.trigger,
        M5LifecycleDowngradeTrigger::StatusCodeUnexportable
    )));
    assert!(validate_m5_lifecycle_object_certification_packet(&packet).is_err());
}

#[test]
fn missing_last_failure_reason_blocks_the_data_session() {
    let packet =
        seeded_m5_lifecycle_object_certification_packet_data_last_failure_missing_blocked();
    let row = packet.row(M5LifecycleObjectFamily::DataSession).unwrap();
    assert_eq!(row.derived_status, LifecycleObjectStatus::Red);
    assert!(!packet.report_clean);
    assert!(packet.blocking_findings.iter().any(|finding| matches!(
        finding,
        LifecycleObjectFinding::LastFailureReasonMissingOrRaw { .. }
    )));
    assert!(row.object_causes.iter().any(|cause| matches!(
        cause.trigger,
        M5LifecycleDowngradeTrigger::LastFailureReasonMissing
    )));
    assert!(validate_m5_lifecycle_object_certification_packet(&packet).is_err());
}

#[test]
fn missing_recovery_affordance_blocks_the_companion_session() {
    let packet =
        seeded_m5_lifecycle_object_certification_packet_companion_recovery_missing_blocked();
    let row = packet
        .row(M5LifecycleObjectFamily::CompanionSession)
        .unwrap();
    assert_eq!(row.derived_status, LifecycleObjectStatus::Red);
    assert!(!packet.report_clean);
    assert!(packet.blocking_findings.iter().any(|finding| matches!(
        finding,
        LifecycleObjectFinding::RecoveryAffordanceMissing { .. }
    )));
    assert!(row.object_causes.iter().any(|cause| matches!(
        cause.trigger,
        M5LifecycleDowngradeTrigger::RecoveryAffordanceMissing
    )));
    assert!(validate_m5_lifecycle_object_certification_packet(&packet).is_err());
}

#[test]
fn headless_parity_loss_blocks_the_extension() {
    let packet =
        seeded_m5_lifecycle_object_certification_packet_extension_headless_parity_lost_blocked();
    let row = packet.row(M5LifecycleObjectFamily::Extension).unwrap();
    assert_eq!(row.derived_status, LifecycleObjectStatus::Red);
    assert!(!row.headless_parity_preserved);
    assert!(!packet.report_clean);
    assert!(packet
        .blocking_findings
        .iter()
        .any(|finding| matches!(finding, LifecycleObjectFinding::HeadlessParityLost { .. })));
    assert!(row.object_causes.iter().any(|cause| matches!(
        cause.trigger,
        M5LifecycleDowngradeTrigger::StateVocabularyDrift
    )));
    assert!(validate_m5_lifecycle_object_certification_packet(&packet).is_err());
}

#[test]
fn incomplete_consumer_surface_certification_blocks() {
    // Hand-mutate a green row so it certifies fewer than all declared consumer surfaces — the
    // completeness lint must block it.
    let mut packet = seeded_m5_lifecycle_object_certification_packet();
    let row = packet
        .rows
        .iter_mut()
        .find(|row| row.object_family == M5LifecycleObjectFamily::Workspace)
        .unwrap();
    row.evaluated_consumer_surfaces.pop();
    assert!(!row.consumer_surfaces_complete());
    assert_eq!(row.recompute_status(), LifecycleObjectStatus::Red);
    let findings = row.compute_findings(&packet.generated_at);
    assert!(findings.iter().any(|finding| matches!(
        finding,
        LifecycleObjectFinding::ConsumerSurfacesIncomplete { .. }
    )));
}

#[test]
fn dashboard_projects_every_row_and_counts() {
    let packet = seeded_m5_lifecycle_object_certification_packet();
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
    assert_eq!(companion.status, LifecycleObjectStatus::Yellow);
    assert!(companion.has_active_waiver);
    assert!(matches!(
        companion.status_surface_binding,
        StatusSurfaceBindingState::DisclosedSurfaceRelocation
    ));
}

#[test]
fn support_export_quotes_packet_matrix_and_waiver_refs() {
    let packet = seeded_m5_lifecycle_object_certification_packet();
    let export = LifecycleObjectSupportExport::from_packet(
        M5_LIFECYCLE_OBJECT_CERTIFICATION_SUPPORT_EXPORT_ID,
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
    let packet = seeded_m5_lifecycle_object_certification_packet();
    let markdown = packet.render_markdown();
    let csv = packet.render_matrix_csv();
    for object_family in REQUIRED_OBJECT_FAMILIES {
        assert!(
            csv.contains(object_family.as_str()),
            "csv omits {}",
            object_family.as_str()
        );
    }
    assert!(markdown.contains("m5_lifecycle_object_certification_fixtures"));
    assert!(markdown.contains("waiver:companion-surface-relocation:0001"));
}

#[test]
fn export_carries_no_forbidden_material() {
    let packet = seeded_m5_lifecycle_object_certification_packet();
    let json = packet.export_safe_json().to_lowercase();
    assert!(!json.contains("api_key"));
    assert!(!json.contains("password"));
    assert!(!json.contains("bearer "));
    assert!(!json.contains("://"));
}

#[test]
fn expired_waiver_is_flagged() {
    let waiver = LifecycleObjectWaiver {
        waiver_id: "waiver:expired:0001".to_owned(),
        object_family: M5LifecycleObjectFamily::CompanionSession,
        reason: "test".to_owned(),
        owner_role: "owner".to_owned(),
        expires_at: "2026-01-01T00:00:00Z".to_owned(),
    };
    assert!(!waiver.is_active_at("2026-06-30T00:00:00Z"));
    assert!(waiver.is_active_at("2025-01-01T00:00:00Z"));
}
