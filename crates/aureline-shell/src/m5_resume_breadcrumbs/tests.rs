//! Inline unit tests for the M5 resume-breadcrumb proof.

use super::*;

#[test]
fn seeded_packet_covers_every_object_family_and_is_clean() {
    let packet = seeded_m5_resume_breadcrumbs_packet();
    validate_m5_resume_breadcrumbs_packet(&packet).expect("seeded packet must validate clean");

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
    let packet = seeded_m5_resume_breadcrumbs_packet();
    assert_eq!(packet.green_row_count, 9);
    assert_eq!(packet.yellow_row_count, 4);
    assert_eq!(packet.red_row_count, 0);

    for family in [
        M5LifecycleObjectFamily::CompanionSession,
        M5LifecycleObjectFamily::PreviewSession,
        M5LifecycleObjectFamily::ProfilerCapture,
        M5LifecycleObjectFamily::CollaborationSession,
    ] {
        assert_eq!(
            packet.row(family).unwrap().derived_status,
            ResumeBreadcrumbStatus::Yellow,
            "{} should auto-narrow to yellow",
            family.as_str()
        );
    }
}

#[test]
fn every_row_status_is_the_derived_value() {
    let packet = seeded_m5_resume_breadcrumbs_packet();
    for row in &packet.rows {
        assert_eq!(
            row.derived_status,
            row.recompute_status(),
            "row {} declares a stale status",
            row.object_family.as_str()
        );
        assert_eq!(row.breadcrumb_causes, row.recompute_causes());
    }
}

#[test]
fn every_row_distinguishes_all_provenance_classes_and_lineage_facets() {
    let packet = seeded_m5_resume_breadcrumbs_packet();
    for row in &packet.rows {
        assert!(
            row.provenance_classes_complete(),
            "row {} does not distinguish all four provenance classes",
            row.object_family.as_str()
        );
        assert!(
            row.lineage_facets_complete(),
            "row {} does not preserve all four lineage facets",
            row.object_family.as_str()
        );
        assert_eq!(row.distinguished_provenance_classes.len(), 4);
        assert_eq!(row.preserved_lineage_facets.len(), 4);
    }
}

#[test]
fn every_row_certifies_all_declared_consumer_surfaces() {
    let packet = seeded_m5_resume_breadcrumbs_packet();
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
    let packet = seeded_m5_resume_breadcrumbs_packet();
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
    let packet = seeded_m5_resume_breadcrumbs_packet();
    for row in &packet.rows {
        if !matches!(row.derived_status, ResumeBreadcrumbStatus::Green) {
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
fn disclosed_grouped_not_resumed_summary_carries_an_active_waiver() {
    let packet = seeded_m5_resume_breadcrumbs_packet();
    let collaboration = packet
        .row(M5LifecycleObjectFamily::CollaborationSession)
        .unwrap();
    assert!(matches!(
        collaboration.not_resumed_disclosure,
        NotResumedDisclosureState::DisclosedGroupedNotResumedSummary
    ));
    assert!(collaboration.requires_waiver());
    assert!(collaboration.has_active_waiver());
    assert_eq!(packet.active_waivers.len(), 1);
    assert!(packet.active_waivers[0].is_active_at(&packet.generated_at));
}

#[test]
fn disclosed_partial_lineage_keeps_preview_yellow_without_a_waiver() {
    let packet = seeded_m5_resume_breadcrumbs_packet();
    let preview = packet.row(M5LifecycleObjectFamily::PreviewSession).unwrap();
    assert!(matches!(
        preview.lineage_breadcrumb,
        LineageBreadcrumbState::DisclosedPartialLineageBreadcrumb
    ));
    assert_eq!(preview.derived_status, ResumeBreadcrumbStatus::Yellow);
    assert!(preview.not_resumed_disclosure.is_full());
    assert!(!preview.requires_waiver());
}

#[test]
fn ambiguous_provenance_blocks_the_notebook_runtime() {
    let packet = seeded_m5_resume_breadcrumbs_packet_notebook_provenance_ambiguous_blocked();
    let row = packet
        .row(M5LifecycleObjectFamily::NotebookRuntime)
        .unwrap();
    assert_eq!(row.derived_status, ResumeBreadcrumbStatus::Red);
    assert!(!packet.report_clean);
    assert!(!packet.all_rows_publishable);
    assert!(packet.red_row_count >= 1);
    assert!(packet
        .blocking_findings
        .iter()
        .any(|finding| matches!(finding, ResumeBreadcrumbFinding::ProvenanceAmbiguous { .. })));
    assert!(row.breadcrumb_causes.iter().any(|cause| matches!(
        cause.trigger,
        M5LifecycleDowngradeTrigger::StateVocabularyDrift
    )));
    assert!(validate_m5_resume_breadcrumbs_packet(&packet).is_err());
}

#[test]
fn generic_recovered_wording_blocks_the_remote_session() {
    let packet = seeded_m5_resume_breadcrumbs_packet_remote_generic_recovered_blocked();
    let row = packet.row(M5LifecycleObjectFamily::RemoteSession).unwrap();
    assert_eq!(row.derived_status, ResumeBreadcrumbStatus::Red);
    assert!(!packet.report_clean);
    assert!(packet.blocking_findings.iter().any(|finding| matches!(
        finding,
        ResumeBreadcrumbFinding::LineageGenericRecoveredOnly { .. }
    )));
    assert!(row.breadcrumb_causes.iter().any(|cause| matches!(
        cause.trigger,
        M5LifecycleDowngradeTrigger::LastFailureReasonMissing
    )));
    assert!(validate_m5_resume_breadcrumbs_packet(&packet).is_err());
}

#[test]
fn silently_absent_not_resumed_blocks_the_data_session() {
    let packet = seeded_m5_resume_breadcrumbs_packet_data_not_resumed_silent_blocked();
    let row = packet.row(M5LifecycleObjectFamily::DataSession).unwrap();
    assert_eq!(row.derived_status, ResumeBreadcrumbStatus::Red);
    assert!(!packet.report_clean);
    assert!(packet.blocking_findings.iter().any(|finding| matches!(
        finding,
        ResumeBreadcrumbFinding::NotResumedActionsSilentlyAbsent { .. }
    )));
    assert!(row.breadcrumb_causes.iter().any(|cause| matches!(
        cause.trigger,
        M5LifecycleDowngradeTrigger::RecoveryAffordanceMissing
    )));
    assert!(validate_m5_resume_breadcrumbs_packet(&packet).is_err());
}

#[test]
fn breadcrumbs_absent_from_capture_blocks_the_ai_action() {
    let packet = seeded_m5_resume_breadcrumbs_packet_ai_capture_absent_blocked();
    let row = packet.row(M5LifecycleObjectFamily::AiAction).unwrap();
    assert_eq!(row.derived_status, ResumeBreadcrumbStatus::Red);
    assert!(!packet.report_clean);
    assert!(packet.blocking_findings.iter().any(|finding| matches!(
        finding,
        ResumeBreadcrumbFinding::BreadcrumbsAbsentFromCapture { .. }
    )));
    assert!(row.breadcrumb_causes.iter().any(|cause| matches!(
        cause.trigger,
        M5LifecycleDowngradeTrigger::StatusCodeUnexportable
    )));
    assert!(validate_m5_resume_breadcrumbs_packet(&packet).is_err());
}

#[test]
fn headless_parity_loss_blocks_the_extension() {
    let packet = seeded_m5_resume_breadcrumbs_packet_extension_headless_parity_lost_blocked();
    let row = packet.row(M5LifecycleObjectFamily::Extension).unwrap();
    assert_eq!(row.derived_status, ResumeBreadcrumbStatus::Red);
    assert!(!row.headless_parity_preserved);
    assert!(!packet.report_clean);
    assert!(packet
        .blocking_findings
        .iter()
        .any(|finding| matches!(finding, ResumeBreadcrumbFinding::HeadlessParityLost { .. })));
    assert!(row.breadcrumb_causes.iter().any(|cause| matches!(
        cause.trigger,
        M5LifecycleDowngradeTrigger::StateVocabularyDrift
    )));
    assert!(validate_m5_resume_breadcrumbs_packet(&packet).is_err());
}

#[test]
fn incomplete_provenance_class_set_blocks() {
    // Hand-mutate a green row so it distinguishes fewer than all four provenance classes — the
    // completeness lint must block it.
    let mut packet = seeded_m5_resume_breadcrumbs_packet();
    let row = packet
        .rows
        .iter_mut()
        .find(|row| row.object_family == M5LifecycleObjectFamily::Workspace)
        .unwrap();
    row.distinguished_provenance_classes.pop();
    assert!(!row.provenance_classes_complete());
    assert_eq!(row.recompute_status(), ResumeBreadcrumbStatus::Red);
    let findings = row.compute_findings(&packet.generated_at);
    assert!(findings.iter().any(|finding| matches!(
        finding,
        ResumeBreadcrumbFinding::ProvenanceClassesIncomplete { .. }
    )));
}

#[test]
fn incomplete_lineage_facet_set_blocks() {
    let mut packet = seeded_m5_resume_breadcrumbs_packet();
    let row = packet
        .rows
        .iter_mut()
        .find(|row| row.object_family == M5LifecycleObjectFamily::Workspace)
        .unwrap();
    row.preserved_lineage_facets.pop();
    assert!(!row.lineage_facets_complete());
    assert_eq!(row.recompute_status(), ResumeBreadcrumbStatus::Red);
    let findings = row.compute_findings(&packet.generated_at);
    assert!(findings.iter().any(|finding| matches!(
        finding,
        ResumeBreadcrumbFinding::LineageFacetsIncomplete { .. }
    )));
}

#[test]
fn incomplete_consumer_surface_certification_blocks() {
    let mut packet = seeded_m5_resume_breadcrumbs_packet();
    let row = packet
        .rows
        .iter_mut()
        .find(|row| row.object_family == M5LifecycleObjectFamily::Workspace)
        .unwrap();
    row.evaluated_consumer_surfaces.pop();
    assert!(!row.consumer_surfaces_complete());
    assert_eq!(row.recompute_status(), ResumeBreadcrumbStatus::Red);
    let findings = row.compute_findings(&packet.generated_at);
    assert!(findings.iter().any(|finding| matches!(
        finding,
        ResumeBreadcrumbFinding::ConsumerSurfacesIncomplete { .. }
    )));
}

#[test]
fn dashboard_projects_every_row_and_counts() {
    let packet = seeded_m5_resume_breadcrumbs_packet();
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
    assert_eq!(collaboration.status, ResumeBreadcrumbStatus::Yellow);
    assert!(collaboration.has_active_waiver);
    assert!(matches!(
        collaboration.not_resumed_disclosure,
        NotResumedDisclosureState::DisclosedGroupedNotResumedSummary
    ));
}

#[test]
fn support_export_quotes_packet_matrix_and_waiver_refs() {
    let packet = seeded_m5_resume_breadcrumbs_packet();
    let export = ResumeBreadcrumbSupportExport::from_packet(
        M5_RESUME_BREADCRUMBS_SUPPORT_EXPORT_ID,
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
    let packet = seeded_m5_resume_breadcrumbs_packet();
    let markdown = packet.render_markdown();
    let csv = packet.render_matrix_csv();
    for family in REQUIRED_OBJECT_FAMILIES {
        assert!(
            csv.contains(family.as_str()),
            "csv omits {}",
            family.as_str()
        );
    }
    assert!(markdown.contains("m5_resume_breadcrumbs_fixtures"));
    assert!(markdown.contains("waiver:collaboration-grouped-not-resumed:0001"));
}

#[test]
fn export_carries_no_forbidden_material() {
    let packet = seeded_m5_resume_breadcrumbs_packet();
    let json = packet.export_safe_json().to_lowercase();
    assert!(!json.contains("api_key"));
    assert!(!json.contains("password"));
    assert!(!json.contains("bearer "));
    assert!(!json.contains("://"));
}

#[test]
fn expired_waiver_is_flagged() {
    let waiver = ResumeBreadcrumbWaiver {
        waiver_id: "waiver:expired:0001".to_owned(),
        object_family: M5LifecycleObjectFamily::CollaborationSession,
        reason: "test".to_owned(),
        owner_role: "owner".to_owned(),
        expires_at: "2026-01-01T00:00:00Z".to_owned(),
    };
    assert!(!waiver.is_active_at("2026-06-30T00:00:00Z"));
    assert!(waiver.is_active_at("2025-01-01T00:00:00Z"));
}
