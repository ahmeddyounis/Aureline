//! Inline unit tests for the M5 shell-zone occupancy proof.

use super::*;

#[test]
fn seeded_packet_covers_every_family_and_is_clean() {
    let packet = seeded_m5_shell_occupancy_packet();
    validate_m5_shell_occupancy_packet(&packet).expect("seeded packet must validate clean");

    assert_eq!(packet.rows.len(), REQUIRED_FAMILIES.len());
    for family in REQUIRED_FAMILIES {
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
fn seeded_packet_has_six_green_and_four_yellow_rows() {
    let packet = seeded_m5_shell_occupancy_packet();
    assert_eq!(packet.green_row_count, 6);
    assert_eq!(packet.yellow_row_count, 4);
    assert_eq!(packet.red_row_count, 0);

    for family in [
        M5ShellSurfaceFamily::DataGrid,
        M5ShellSurfaceFamily::Incident,
        M5ShellSurfaceFamily::Companion,
        M5ShellSurfaceFamily::Operator,
    ] {
        assert_eq!(
            packet.row(family).unwrap().derived_status,
            ShellOccupancyStatus::Yellow,
            "{} should auto-narrow to yellow",
            family.as_str()
        );
    }
}

#[test]
fn every_row_status_is_the_derived_value() {
    let packet = seeded_m5_shell_occupancy_packet();
    for row in &packet.rows {
        assert_eq!(
            row.derived_status,
            row.recompute_status(),
            "row {} declares a stale status",
            row.family.as_str()
        );
        assert_eq!(row.occupancy_causes, row.recompute_causes());
    }
}

#[test]
fn every_row_occupies_a_registered_declared_slot() {
    let packet = seeded_m5_shell_occupancy_packet();
    for row in &packet.rows {
        assert!(
            row.slot_attachment.is_declared(),
            "row {} attaches outside a declared slot",
            row.family.as_str()
        );
        assert!(
            row.occupied_slot_is_registered(),
            "row {} occupies an unregistered slot",
            row.family.as_str()
        );
        assert!(
            row.registered_declares_canonical(),
            "row {} does not register its canonical slot",
            row.family.as_str()
        );
    }
}

#[test]
fn narrowed_rows_disclose_a_reason() {
    let packet = seeded_m5_shell_occupancy_packet();
    for row in &packet.rows {
        if !matches!(row.derived_status, ShellOccupancyStatus::Green) {
            assert!(
                row.narrowing_reason
                    .as_deref()
                    .map(|reason| !reason.trim().is_empty())
                    .unwrap_or(false),
                "narrowed row {} hides its reason",
                row.family.as_str()
            );
        }
    }
}

#[test]
fn dependency_missing_and_policy_blocked_placeholders_keep_continuity() {
    let packet = seeded_m5_shell_occupancy_packet();

    let data_grid = packet.row(M5ShellSurfaceFamily::DataGrid).unwrap();
    assert!(matches!(
        data_grid.occupant_availability,
        OccupantAvailabilityState::DependencyMissingPlaceholder
    ));
    assert_eq!(data_grid.derived_status, ShellOccupancyStatus::Yellow);
    // A placeholder keeps the slot occupied — it is registered, not collapsed.
    assert!(data_grid.occupied_slot_is_registered());

    let operator = packet.row(M5ShellSurfaceFamily::Operator).unwrap();
    assert!(matches!(
        operator.occupant_availability,
        OccupantAvailabilityState::PolicyBlockedPlaceholder
    ));
    assert_eq!(operator.derived_status, ShellOccupancyStatus::Yellow);
}

#[test]
fn disclosed_route_fallback_carries_an_active_waiver() {
    let packet = seeded_m5_shell_occupancy_packet();
    let companion = packet.row(M5ShellSurfaceFamily::Companion).unwrap();
    assert!(matches!(
        companion.route_resolution,
        RouteResolutionState::DisclosedRouteFallback
    ));
    assert!(companion.requires_waiver());
    assert!(companion.has_active_waiver());
    assert_eq!(packet.active_waivers.len(), 1);
    assert!(packet.active_waivers[0].is_active_at(&packet.generated_at));
}

#[test]
fn undeclared_slot_attachment_blocks_promotion() {
    let packet = seeded_m5_shell_occupancy_packet_notebook_undeclared_blocked();
    let row = packet.row(M5ShellSurfaceFamily::Notebook).unwrap();
    assert_eq!(row.derived_status, ShellOccupancyStatus::Red);
    assert!(!packet.report_clean);
    assert!(!packet.all_rows_publishable);
    assert!(packet.red_row_count >= 1);
    assert!(packet.blocking_findings.iter().any(|finding| matches!(
        finding,
        ShellOccupancyFinding::UndeclaredSlotAttachment { .. }
    )));
    // The exact cause is recorded against the slot-undeclared trigger.
    assert!(row
        .occupancy_causes
        .iter()
        .any(|cause| matches!(cause.trigger, M5ShellDowngradeTrigger::SlotUndeclared)));
    assert!(validate_m5_shell_occupancy_packet(&packet).is_err());
}

#[test]
fn collapsed_placeholder_blocks_the_data_grid() {
    let packet = seeded_m5_shell_occupancy_packet_data_grid_placeholder_collapsed_blocked();
    let row = packet.row(M5ShellSurfaceFamily::DataGrid).unwrap();
    assert_eq!(row.derived_status, ShellOccupancyStatus::Red);
    assert!(!packet.report_clean);
    assert!(packet.blocking_findings.iter().any(|finding| matches!(
        finding,
        ShellOccupancyFinding::PlaceholderCollapsedLayout { .. }
    )));
    assert!(row.occupancy_causes.iter().any(|cause| matches!(
        cause.trigger,
        M5ShellDowngradeTrigger::PlaceholderLostIdentityOrReopen
    )));
    assert!(validate_m5_shell_occupancy_packet(&packet).is_err());
}

#[test]
fn conflicting_route_blocks_the_review_surface() {
    let packet = seeded_m5_shell_occupancy_packet_review_route_conflict_blocked();
    let row = packet.row(M5ShellSurfaceFamily::Review).unwrap();
    assert_eq!(row.derived_status, ShellOccupancyStatus::Red);
    assert!(!packet.report_clean);
    assert!(packet.blocking_findings.iter().any(|finding| matches!(
        finding,
        ShellOccupancyFinding::ConflictingRouteResolution { .. }
    )));
    assert!(row.occupancy_causes.iter().any(|cause| matches!(
        cause.trigger,
        M5ShellDowngradeTrigger::OwningWindowRoutingLost
    )));
    assert!(validate_m5_shell_occupancy_packet(&packet).is_err());
}

#[test]
fn green_rows_resolve_every_route_channel() {
    let packet = seeded_m5_shell_occupancy_packet();
    for row in &packet.rows {
        if matches!(row.derived_status, ShellOccupancyStatus::Green) {
            for channel in RouteChannel::ALL {
                assert!(
                    row.resolved_route_channels.contains(&channel),
                    "green row {} does not resolve {}",
                    row.family.as_str(),
                    channel.as_str()
                );
            }
        }
    }
}

#[test]
fn dashboard_projects_every_row_and_counts() {
    let packet = seeded_m5_shell_occupancy_packet();
    let dashboard = packet.dashboard();
    assert_eq!(dashboard.rows.len(), packet.rows.len());
    assert_eq!(dashboard.green_row_count, packet.green_row_count);
    assert_eq!(dashboard.yellow_row_count, packet.yellow_row_count);
    assert_eq!(dashboard.red_row_count, packet.red_row_count);
    assert_eq!(dashboard.all_rows_publishable, packet.all_rows_publishable);
    assert_eq!(dashboard.source_packet_ref, packet.packet_id);
    assert!(!dashboard.shell_automation_refs.is_empty());

    let companion = dashboard
        .rows
        .iter()
        .find(|row| row.family == M5ShellSurfaceFamily::Companion)
        .unwrap();
    assert_eq!(companion.status, ShellOccupancyStatus::Yellow);
    assert!(companion.has_active_waiver);
    assert!(companion.cause_tokens.contains(
        &M5ShellDowngradeTrigger::OwningWindowRoutingLost
            .as_str()
            .to_owned()
    ));
}

#[test]
fn support_export_quotes_packet_matrix_and_waiver_refs() {
    let packet = seeded_m5_shell_occupancy_packet();
    let export = ShellOccupancySupportExport::from_packet(
        M5_SHELL_OCCUPANCY_SUPPORT_EXPORT_ID,
        packet.clone(),
    );
    assert!(export.case_ids.contains(&packet.packet_id));
    assert!(export.case_ids.contains(&packet.matrix_packet_ref));
    assert!(export.case_ids.contains(&packet.build_identity_ref));
    for row in &packet.rows {
        assert!(export.case_ids.contains(&row.family.as_str().to_owned()));
    }
    for waiver in &packet.active_waivers {
        assert!(export.case_ids.contains(&waiver.waiver_id));
    }
    assert_eq!(export.dashboard, packet.dashboard());
}

#[test]
fn markdown_and_csv_name_every_family() {
    let packet = seeded_m5_shell_occupancy_packet();
    let markdown = packet.render_markdown();
    let csv = packet.render_matrix_csv();
    for family in REQUIRED_FAMILIES {
        assert!(
            markdown.contains(occupant_label(family)),
            "markdown omits {}",
            family.as_str()
        );
        assert!(
            csv.contains(family.as_str()),
            "csv omits {}",
            family.as_str()
        );
    }
    assert!(markdown.contains("m5_shell_zone_occupancy_fixtures"));
    assert!(markdown.contains("waiver:companion-onboarding-route-sync:0001"));
}

#[test]
fn export_carries_no_forbidden_material() {
    let packet = seeded_m5_shell_occupancy_packet();
    let json = packet.export_safe_json().to_lowercase();
    assert!(!json.contains("api_key"));
    assert!(!json.contains("password"));
    assert!(!json.contains("bearer "));
    assert!(!json.contains("://"));
}

#[test]
fn expired_waiver_is_flagged() {
    let waiver = ShellOccupancyWaiver {
        waiver_id: "waiver:expired:0001".to_owned(),
        family: M5ShellSurfaceFamily::Companion,
        reason: "test".to_owned(),
        owner_role: "owner".to_owned(),
        expires_at: "2026-01-01T00:00:00Z".to_owned(),
    };
    assert!(!waiver.is_active_at("2026-06-30T00:00:00Z"));
    assert!(waiver.is_active_at("2025-01-01T00:00:00Z"));
}
