//! Inline unit tests for the M5 status-bar certification proof.

use super::*;

#[test]
fn seeded_packet_covers_every_context_and_is_clean() {
    let packet = seeded_m5_status_bar_certification_packet();
    validate_m5_status_bar_certification_packet(&packet)
        .expect("seeded packet must validate clean");

    assert_eq!(packet.rows.len(), M5StatusContext::ALL.len());
    for context in M5StatusContext::ALL {
        assert!(
            packet.row(context).is_some(),
            "missing row for {}",
            context.as_str()
        );
    }
    assert!(packet.report_clean);
    assert!(packet.blocking_findings.is_empty());
    assert_eq!(packet.red_row_count, 0);
    assert!(packet.all_rows_publishable);
}

#[test]
fn seeded_packet_has_four_green_and_four_yellow_rows() {
    let packet = seeded_m5_status_bar_certification_packet();
    assert_eq!(packet.green_row_count, 4);
    assert_eq!(packet.yellow_row_count, 4);
    assert_eq!(packet.red_row_count, 0);

    for context in [
        M5StatusContext::RemoteLane,
        M5StatusContext::PreviewLane,
        M5StatusContext::ProfilerLane,
        M5StatusContext::IncidentLane,
    ] {
        assert_eq!(
            packet.row(context).unwrap().derived_status,
            StatusBarCertificationStatus::Yellow,
            "{} should auto-narrow to yellow",
            context.as_str()
        );
    }
}

#[test]
fn every_row_status_is_the_derived_value() {
    let packet = seeded_m5_status_bar_certification_packet();
    for row in &packet.rows {
        assert_eq!(
            row.derived_status,
            row.recompute_status(),
            "row {} declares a stale status",
            row.context.as_str()
        );
        assert_eq!(row.certification_causes, row.recompute_causes());
    }
}

#[test]
fn every_row_keeps_priority_order_reach_routes_and_status_classes() {
    let packet = seeded_m5_status_bar_certification_packet();
    for row in &packet.rows {
        assert!(
            row.priority_order_well_formed(),
            "row {} lost its canonical priority order",
            row.context.as_str()
        );
        assert!(
            row.reach_routes_complete(),
            "row {} does not certify every reach route",
            row.context.as_str()
        );
        assert!(
            row.status_item_classes_complete(),
            "row {} does not certify every status-item class",
            row.context.as_str()
        );
        assert!(
            row.keyboard_reachable_without_hover,
            "row {} keeps critical truth hover-only",
            row.context.as_str()
        );
    }
}

#[test]
fn priority_classes_are_recovery_critical_first() {
    assert_eq!(
        M5StatusPriorityClass::ALL[0],
        M5StatusPriorityClass::RecoveryCritical
    );
    // priority_rank must be a strictly increasing (lower = higher priority) sequence.
    for pair in M5StatusPriorityClass::ALL.windows(2) {
        assert!(
            pair[0].priority_rank() < pair[1].priority_rank(),
            "priority ranks must strictly increase from recovery-critical to ambient"
        );
    }
}

#[test]
fn narrowed_rows_disclose_a_reason() {
    let packet = seeded_m5_status_bar_certification_packet();
    for row in &packet.rows {
        if !matches!(row.derived_status, StatusBarCertificationStatus::Green) {
            assert!(
                row.narrowing_reason
                    .as_deref()
                    .map(|reason| !reason.trim().is_empty())
                    .unwrap_or(false),
                "narrowed row {} hides its reason",
                row.context.as_str()
            );
        }
    }
}

#[test]
fn disclosed_compact_compaction_carries_an_active_waiver() {
    let packet = seeded_m5_status_bar_certification_packet();
    let incident = packet.row(M5StatusContext::IncidentLane).unwrap();
    assert!(matches!(
        incident.placement_stability,
        PlacementStabilityState::DisclosedCompactPriorityCompaction
    ));
    assert!(incident.requires_waiver());
    assert!(incident.has_active_waiver());
    assert_eq!(packet.active_waivers.len(), 1);
    assert!(packet.active_waivers[0].is_active_at(&packet.generated_at));
}

#[test]
fn reduced_overflow_route_narrows_but_does_not_block() {
    let packet = seeded_m5_status_bar_certification_packet();
    let remote = packet.row(M5StatusContext::RemoteLane).unwrap();
    assert!(matches!(
        remote.overflow_discoverability,
        OverflowDiscoverabilityState::DisclosedReducedOverflowRoute
    ));
    assert_eq!(remote.derived_status, StatusBarCertificationStatus::Yellow);
    assert!(remote
        .certification_causes
        .iter()
        .any(|cause| cause.disclosed
            && matches!(
                cause.trigger,
                M5ShellPrimitiveDowngradeTrigger::HoverOnlyCriticalTruth
            )));
}

#[test]
fn vanity_reflow_blocks_the_notebook_lane() {
    let packet = seeded_m5_status_bar_certification_packet_notebook_vanity_reflow_blocked();
    let row = packet.row(M5StatusContext::NotebookLane).unwrap();
    assert_eq!(row.derived_status, StatusBarCertificationStatus::Red);
    assert!(!packet.report_clean);
    assert!(!packet.all_rows_publishable);
    assert!(packet.red_row_count >= 1);
    assert!(packet.blocking_findings.iter().any(|finding| matches!(
        finding,
        StatusBarCertificationFinding::UnstablePlacement { .. }
    )));
    assert!(row.certification_causes.iter().any(|cause| !cause.disclosed
        && matches!(
            cause.trigger,
            M5ShellPrimitiveDowngradeTrigger::SevereStateDisplacedTruth
        )));
    assert!(validate_m5_status_bar_certification_packet(&packet).is_err());
}

#[test]
fn hover_only_overflow_blocks_the_data_api_lane() {
    let packet = seeded_m5_status_bar_certification_packet_data_api_overflow_hover_only_blocked();
    let row = packet.row(M5StatusContext::DataApiLane).unwrap();
    assert_eq!(row.derived_status, StatusBarCertificationStatus::Red);
    assert!(!packet.report_clean);
    assert!(packet.blocking_findings.iter().any(|finding| matches!(
        finding,
        StatusBarCertificationFinding::OverflowNotKeyboardReachable { .. }
    )));
    assert!(validate_m5_status_bar_certification_packet(&packet).is_err());
}

#[test]
fn missing_backlink_blocks_the_review_lane() {
    let packet = seeded_m5_status_bar_certification_packet_review_backlink_missing_blocked();
    let row = packet.row(M5StatusContext::ReviewLane).unwrap();
    assert_eq!(row.derived_status, StatusBarCertificationStatus::Red);
    assert!(!packet.report_clean);
    assert!(packet.blocking_findings.iter().any(|finding| matches!(
        finding,
        StatusBarCertificationFinding::InspectorBacklinkMissing { .. }
    )));
    assert!(validate_m5_status_bar_certification_packet(&packet).is_err());
}

#[test]
fn absent_capture_blocks_the_preview_lane() {
    let packet = seeded_m5_status_bar_certification_packet_preview_capture_absent_blocked();
    let row = packet.row(M5StatusContext::PreviewLane).unwrap();
    assert_eq!(row.derived_status, StatusBarCertificationStatus::Red);
    assert!(!packet.report_clean);
    assert!(packet.blocking_findings.iter().any(|finding| matches!(
        finding,
        StatusBarCertificationFinding::CriticalDisplacementAbsentFromCapture { .. }
    )));
    assert!(validate_m5_status_bar_certification_packet(&packet).is_err());
}

#[test]
fn hover_only_item_blocks_the_desktop_base_lane() {
    let packet = seeded_m5_status_bar_certification_packet_desktop_base_hover_only_blocked();
    let row = packet.row(M5StatusContext::DesktopBaseLane).unwrap();
    assert_eq!(row.derived_status, StatusBarCertificationStatus::Red);
    assert!(!packet.report_clean);
    assert!(packet.blocking_findings.iter().any(|finding| matches!(
        finding,
        StatusBarCertificationFinding::KeyboardReachabilityLost { .. }
    )));
    assert!(row.certification_causes.iter().any(|cause| !cause.disclosed
        && matches!(
            cause.trigger,
            M5ShellPrimitiveDowngradeTrigger::HoverOnlyCriticalTruth
        )));
    assert!(validate_m5_status_bar_certification_packet(&packet).is_err());
}

#[test]
fn every_row_pulls_ambient_bindings_from_the_matrix() {
    let packet = seeded_m5_status_bar_certification_packet();
    for row in &packet.rows {
        assert!(!row.certified_status_item_classes.is_empty());
        assert!(!row.overflow_behaviors.is_empty());
        assert!(!row.source_freshness_labels.is_empty());
        assert!(!row.accessibility_routes.is_empty());
        assert!(!row.consumer_surfaces.is_empty());
        assert!(!row.applicable_downgrade_triggers.is_empty());
        assert_eq!(row.shell_zone_slot, M5ShellZoneSlot::StatusBar);
        assert_eq!(
            row.driven_primitive_families,
            vec![
                M5ShellPrimitiveFamily::StatusBarItem,
                M5ShellPrimitiveFamily::StatusOverflowMenu
            ]
        );
    }
}

#[test]
fn dashboard_projects_every_row_and_counts() {
    let packet = seeded_m5_status_bar_certification_packet();
    let dashboard = packet.dashboard();
    assert_eq!(dashboard.rows.len(), packet.rows.len());
    assert_eq!(dashboard.green_row_count, packet.green_row_count);
    assert_eq!(dashboard.yellow_row_count, packet.yellow_row_count);
    assert_eq!(dashboard.red_row_count, packet.red_row_count);
    assert_eq!(dashboard.all_rows_publishable, packet.all_rows_publishable);
    assert_eq!(dashboard.source_packet_ref, packet.packet_id);
    assert!(!dashboard.shell_automation_refs.is_empty());

    let incident = dashboard
        .rows
        .iter()
        .find(|row| row.context == M5StatusContext::IncidentLane)
        .unwrap();
    assert_eq!(incident.status, StatusBarCertificationStatus::Yellow);
    assert!(incident.has_active_waiver);
    assert!(incident.cause_tokens.contains(
        &M5ShellPrimitiveDowngradeTrigger::VanityItemReflow
            .as_str()
            .to_owned()
    ));
}

#[test]
fn support_export_quotes_packet_matrix_and_waiver_refs() {
    let packet = seeded_m5_status_bar_certification_packet();
    let export = StatusBarCertificationSupportExport::from_packet(
        M5_STATUS_BAR_CERTIFICATION_SUPPORT_EXPORT_ID,
        packet.clone(),
    );
    assert!(export.case_ids.contains(&packet.packet_id));
    assert!(export.case_ids.contains(&packet.matrix_packet_ref));
    assert!(export.case_ids.contains(&packet.build_identity_ref));
    for row in &packet.rows {
        assert!(export.case_ids.contains(&row.context.as_str().to_owned()));
    }
    for waiver in &packet.active_waivers {
        assert!(export.case_ids.contains(&waiver.waiver_id));
    }
    assert_eq!(export.dashboard, packet.dashboard());
}

#[test]
fn markdown_and_csv_name_every_context() {
    let packet = seeded_m5_status_bar_certification_packet();
    let markdown = packet.render_markdown();
    let csv = packet.render_matrix_csv();
    for context in M5StatusContext::ALL {
        assert!(
            markdown.contains(context.label()),
            "markdown omits {}",
            context.as_str()
        );
        assert!(
            csv.contains(context.as_str()),
            "csv omits {}",
            context.as_str()
        );
    }
    assert!(markdown.contains("m5_status_bar_certification_fixtures"));
    assert!(markdown.contains("waiver:incident-compact-priority-compaction:0001"));
}

#[test]
fn export_carries_no_forbidden_material() {
    let packet = seeded_m5_status_bar_certification_packet();
    let json = packet.export_safe_json().to_lowercase();
    assert!(!json.contains("api_key"));
    assert!(!json.contains("password"));
    assert!(!json.contains("secret"));
    assert!(!json.contains("bearer "));
    assert!(!json.contains("://"));
}

#[test]
fn expired_waiver_is_flagged() {
    let waiver = StatusBarCertificationWaiver {
        waiver_id: "waiver:expired:0001".to_owned(),
        context: M5StatusContext::IncidentLane,
        reason: "test".to_owned(),
        owner_role: "owner".to_owned(),
        expires_at: "2026-01-01T00:00:00Z".to_owned(),
    };
    assert!(!waiver.is_active_at("2026-06-30T00:00:00Z"));
    assert!(waiver.is_active_at("2025-01-01T00:00:00Z"));
}
