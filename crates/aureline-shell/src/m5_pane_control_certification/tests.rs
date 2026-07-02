//! Inline unit tests for the M5 pane-control certification proof.

use super::*;

#[test]
fn seeded_packet_covers_every_layout_and_is_clean() {
    let packet = seeded_m5_pane_control_certification_packet();
    validate_m5_pane_control_certification_packet(&packet)
        .expect("seeded packet must validate clean");

    assert_eq!(packet.rows.len(), M5PaneLayout::ALL.len());
    for layout in M5PaneLayout::ALL {
        assert!(
            packet.row(layout).is_some(),
            "missing row for {}",
            layout.as_str()
        );
    }
    assert!(packet.report_clean);
    assert!(packet.blocking_findings.is_empty());
    assert_eq!(packet.red_row_count, 0);
    assert!(packet.all_rows_publishable);
}

#[test]
fn seeded_packet_has_two_green_and_four_yellow_rows() {
    let packet = seeded_m5_pane_control_certification_packet();
    assert_eq!(packet.green_row_count, 2);
    assert_eq!(packet.yellow_row_count, 4);
    assert_eq!(packet.red_row_count, 0);

    for layout in [
        M5PaneLayout::Review,
        M5PaneLayout::Docs,
        M5PaneLayout::Profiler,
        M5PaneLayout::Incident,
    ] {
        assert_eq!(
            packet.row(layout).unwrap().derived_status,
            PaneControlCertificationStatus::Yellow,
            "{} should auto-narrow to yellow",
            layout.as_str()
        );
    }
}

#[test]
fn every_row_status_is_the_derived_value() {
    let packet = seeded_m5_pane_control_certification_packet();
    for row in &packet.rows {
        assert_eq!(
            row.derived_status,
            row.recompute_status(),
            "row {} declares a stale status",
            row.layout.as_str()
        );
        assert_eq!(row.certification_causes, row.recompute_causes());
    }
}

#[test]
fn every_row_keeps_complete_pane_resize_states_and_labels() {
    let packet = seeded_m5_pane_control_certification_packet();
    for row in &packet.rows {
        assert!(
            row.pane_resize_states_complete(),
            "row {} does not certify every pane-resize state",
            row.layout.as_str()
        );
        assert!(
            row.required_labels_complete(),
            "row {} does not certify every pane-control required label",
            row.layout.as_str()
        );
        assert!(
            row.pane_never_pointer_only_resizable,
            "row {} allows a pointer-only resizable pane",
            row.layout.as_str()
        );
    }
}

#[test]
fn pane_matrix_union_covers_all_resize_states_and_pane_labels() {
    // The union across the two pane families must cover the full frozen pane-resize
    // state vocabulary, else no layout could be certified green. Pane controls carry no
    // source/provider or freshness label, so the required-label set is the four
    // pane-control labels — not the full six.
    let packet = seeded_m5_pane_control_certification_packet();
    let row = packet.row(M5PaneLayout::Notebook).unwrap();
    assert_eq!(
        row.certified_pane_resize_states,
        M5PaneResizeState::ALL.to_vec()
    );
    assert_eq!(row.required_labels, PANE_CONTROL_REQUIRED_LABELS.to_vec());
    assert!(!row
        .required_labels
        .contains(&M5PrimitiveRequiredLabel::Freshness));
    assert!(!row
        .required_labels
        .contains(&M5PrimitiveRequiredLabel::SourceProvider));
}

#[test]
fn narrowed_rows_disclose_a_reason() {
    let packet = seeded_m5_pane_control_certification_packet();
    for row in &packet.rows {
        if !matches!(row.derived_status, PaneControlCertificationStatus::Green) {
            assert!(
                row.narrowing_reason
                    .as_deref()
                    .map(|reason| !reason.trim().is_empty())
                    .unwrap_or(false),
                "narrowed row {} hides its reason",
                row.layout.as_str()
            );
        }
    }
}

#[test]
fn reduced_restore_fidelity_carries_an_active_waiver() {
    let packet = seeded_m5_pane_control_certification_packet();
    let profiler = packet.row(M5PaneLayout::Profiler).unwrap();
    assert!(matches!(
        profiler.reset_restore,
        ResetRestoreState::DisclosedReducedRestoreFidelity
    ));
    assert!(profiler.requires_waiver());
    assert!(profiler.has_active_waiver());
    assert_eq!(packet.active_waivers.len(), 1);
    assert!(packet.active_waivers[0].is_active_at(&packet.generated_at));
}

#[test]
fn reduced_hit_target_narrows_but_does_not_block() {
    let packet = seeded_m5_pane_control_certification_packet();
    let docs = packet.row(M5PaneLayout::Docs).unwrap();
    assert!(matches!(
        docs.resize_control_precision,
        ResizeControlPrecisionState::DisclosedReducedHitTargetOrStep
    ));
    assert_eq!(docs.derived_status, PaneControlCertificationStatus::Yellow);
    assert!(docs.certification_causes.iter().any(|cause| cause.disclosed
        && matches!(
            cause.trigger,
            M5ShellPrimitiveDowngradeTrigger::PointerOnlyResize
        )));
}

#[test]
fn pointer_only_resize_blocks_the_notebook_lane() {
    let packet = seeded_m5_pane_control_certification_packet_notebook_pointer_only_resize_blocked();
    let row = packet.row(M5PaneLayout::Notebook).unwrap();
    assert_eq!(row.derived_status, PaneControlCertificationStatus::Red);
    assert!(!packet.report_clean);
    assert!(!packet.all_rows_publishable);
    assert!(packet.red_row_count >= 1);
    assert!(packet.blocking_findings.iter().any(|finding| matches!(
        finding,
        PaneControlCertificationFinding::ResizeControlNotPrecise { .. }
    )));
    assert!(row.certification_causes.iter().any(|cause| !cause.disclosed
        && matches!(
            cause.trigger,
            M5ShellPrimitiveDowngradeTrigger::PointerOnlyResize
        )));
    assert!(validate_m5_pane_control_certification_packet(&packet).is_err());
}

#[test]
fn pixel_only_persistence_blocks_the_data_lane() {
    let packet = seeded_m5_pane_control_certification_packet_data_pixel_only_persistence_blocked();
    let row = packet.row(M5PaneLayout::Data).unwrap();
    assert_eq!(row.derived_status, PaneControlCertificationStatus::Red);
    assert!(!packet.report_clean);
    assert!(packet.blocking_findings.iter().any(|finding| matches!(
        finding,
        PaneControlCertificationFinding::PersistenceBrittlePixelOnly { .. }
    )));
    assert!(row.certification_causes.iter().any(|cause| !cause.disclosed
        && matches!(
            cause.trigger,
            M5ShellPrimitiveDowngradeTrigger::ResizeStateNotSerializable
        )));
    assert!(validate_m5_pane_control_certification_packet(&packet).is_err());
}

#[test]
fn destructive_restore_blocks_the_review_lane() {
    let packet = seeded_m5_pane_control_certification_packet_review_restore_destructive_blocked();
    let row = packet.row(M5PaneLayout::Review).unwrap();
    assert_eq!(row.derived_status, PaneControlCertificationStatus::Red);
    assert!(!packet.report_clean);
    assert!(packet.blocking_findings.iter().any(|finding| matches!(
        finding,
        PaneControlCertificationFinding::RestoreLostOrDestructive { .. }
    )));
    assert!(validate_m5_pane_control_certification_packet(&packet).is_err());
}

#[test]
fn absent_export_blocks_the_docs_lane() {
    let packet =
        seeded_m5_pane_control_certification_packet_docs_resize_absent_from_capture_blocked();
    let row = packet.row(M5PaneLayout::Docs).unwrap();
    assert_eq!(row.derived_status, PaneControlCertificationStatus::Red);
    assert!(!packet.report_clean);
    assert!(packet.blocking_findings.iter().any(|finding| matches!(
        finding,
        PaneControlCertificationFinding::ResizeStateAbsentFromCapture { .. }
    )));
    assert!(validate_m5_pane_control_certification_packet(&packet).is_err());
}

#[test]
fn pointer_only_resizable_invariant_blocks_the_incident_lane() {
    let packet =
        seeded_m5_pane_control_certification_packet_incident_pointer_only_resizable_blocked();
    let row = packet.row(M5PaneLayout::Incident).unwrap();
    assert_eq!(row.derived_status, PaneControlCertificationStatus::Red);
    assert!(!packet.report_clean);
    assert!(packet.blocking_findings.iter().any(|finding| matches!(
        finding,
        PaneControlCertificationFinding::PanePointerOnlyResizable { .. }
    )));
    assert!(row.certification_causes.iter().any(|cause| !cause.disclosed
        && matches!(
            cause.trigger,
            M5ShellPrimitiveDowngradeTrigger::PointerOnlyResize
        )));
    assert!(validate_m5_pane_control_certification_packet(&packet).is_err());
}

#[test]
fn every_row_pulls_pane_bindings_from_the_matrix() {
    let packet = seeded_m5_pane_control_certification_packet();
    for row in &packet.rows {
        assert!(!row.certified_pane_resize_states.is_empty());
        assert!(!row.accessibility_routes.is_empty());
        assert!(!row.required_labels.is_empty());
        assert!(!row.consumer_surfaces.is_empty());
        assert!(!row.applicable_downgrade_triggers.is_empty());
        assert_eq!(row.shell_zone_slot, M5ShellZoneSlot::MainWorkspace);
        assert_eq!(
            row.driven_primitive_families,
            vec![
                M5ShellPrimitiveFamily::SplitterHandle,
                M5ShellPrimitiveFamily::PaneResizePreset,
            ]
        );
        assert!(row
            .applicable_downgrade_triggers
            .contains(&M5ShellPrimitiveDowngradeTrigger::PointerOnlyResize));
        assert!(row
            .applicable_downgrade_triggers
            .contains(&M5ShellPrimitiveDowngradeTrigger::ResizeStateNotSerializable));
    }
}

#[test]
fn dashboard_projects_every_row_and_counts() {
    let packet = seeded_m5_pane_control_certification_packet();
    let dashboard = packet.dashboard();
    assert_eq!(dashboard.rows.len(), packet.rows.len());
    assert_eq!(dashboard.green_row_count, packet.green_row_count);
    assert_eq!(dashboard.yellow_row_count, packet.yellow_row_count);
    assert_eq!(dashboard.red_row_count, packet.red_row_count);
    assert_eq!(dashboard.all_rows_publishable, packet.all_rows_publishable);
    assert_eq!(dashboard.source_packet_ref, packet.packet_id);
    assert!(!dashboard.shell_automation_refs.is_empty());

    let profiler = dashboard
        .rows
        .iter()
        .find(|row| row.layout == M5PaneLayout::Profiler)
        .unwrap();
    assert_eq!(profiler.status, PaneControlCertificationStatus::Yellow);
    assert!(profiler.has_active_waiver);
    assert!(profiler.cause_tokens.contains(
        &M5ShellPrimitiveDowngradeTrigger::ResizeStateNotSerializable
            .as_str()
            .to_owned()
    ));
}

#[test]
fn support_export_quotes_packet_matrix_and_waiver_refs() {
    let packet = seeded_m5_pane_control_certification_packet();
    let export = PaneControlCertificationSupportExport::from_packet(
        M5_PANE_CONTROL_CERTIFICATION_SUPPORT_EXPORT_ID,
        packet.clone(),
    );
    assert!(export.case_ids.contains(&packet.packet_id));
    assert!(export.case_ids.contains(&packet.matrix_packet_ref));
    assert!(export.case_ids.contains(&packet.build_identity_ref));
    for row in &packet.rows {
        assert!(export.case_ids.contains(&row.layout.as_str().to_owned()));
    }
    for waiver in &packet.active_waivers {
        assert!(export.case_ids.contains(&waiver.waiver_id));
    }
    assert_eq!(export.dashboard, packet.dashboard());
}

#[test]
fn markdown_and_csv_name_every_layout() {
    let packet = seeded_m5_pane_control_certification_packet();
    let markdown = packet.render_markdown();
    let csv = packet.render_matrix_csv();
    for layout in M5PaneLayout::ALL {
        assert!(
            markdown.contains(layout.label()),
            "markdown omits {}",
            layout.as_str()
        );
        assert!(
            csv.contains(layout.as_str()),
            "csv omits {}",
            layout.as_str()
        );
    }
    assert!(markdown.contains("m5_pane_control_certification_fixtures"));
    assert!(markdown.contains("waiver:profiler-reduced-restore-fidelity:0001"));
}

#[test]
fn export_carries_no_forbidden_material() {
    let packet = seeded_m5_pane_control_certification_packet();
    let json = packet.export_safe_json().to_lowercase();
    assert!(!json.contains("api_key"));
    assert!(!json.contains("password"));
    assert!(!json.contains("secret"));
    assert!(!json.contains("bearer "));
    assert!(!json.contains("://"));
}

#[test]
fn expired_waiver_is_flagged() {
    let waiver = PaneControlCertificationWaiver {
        waiver_id: "waiver:expired:0001".to_owned(),
        layout: M5PaneLayout::Profiler,
        reason: "test".to_owned(),
        owner_role: "owner".to_owned(),
        expires_at: "2026-01-01T00:00:00Z".to_owned(),
    };
    assert!(!waiver.is_active_at("2026-06-30T00:00:00Z"));
    assert!(waiver.is_active_at("2025-01-01T00:00:00Z"));
}
