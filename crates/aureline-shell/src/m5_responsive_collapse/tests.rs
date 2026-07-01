//! Inline unit tests for the M5 responsive-collapse proof.

use super::*;

#[test]
fn seeded_packet_covers_every_family_and_is_clean() {
    let packet = seeded_m5_responsive_collapse_packet();
    validate_m5_responsive_collapse_packet(&packet).expect("seeded packet must validate clean");

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
    let packet = seeded_m5_responsive_collapse_packet();
    assert_eq!(packet.green_row_count, 6);
    assert_eq!(packet.yellow_row_count, 4);
    assert_eq!(packet.red_row_count, 0);

    for family in [
        M5ShellSurfaceFamily::Profiler,
        M5ShellSurfaceFamily::Incident,
        M5ShellSurfaceFamily::Companion,
        M5ShellSurfaceFamily::Operator,
    ] {
        assert_eq!(
            packet.row(family).unwrap().derived_status,
            ResponsiveCollapseStatus::Yellow,
            "{} should auto-narrow to yellow",
            family.as_str()
        );
    }
}

#[test]
fn every_row_status_is_the_derived_value() {
    let packet = seeded_m5_responsive_collapse_packet();
    for row in &packet.rows {
        assert_eq!(
            row.derived_status,
            row.recompute_status(),
            "row {} declares a stale status",
            row.family.as_str()
        );
        assert_eq!(row.collapse_causes, row.recompute_causes());
    }
}

#[test]
fn every_row_ladder_is_ordered_and_terminates_in_placeholder() {
    let packet = seeded_m5_responsive_collapse_packet();
    for row in &packet.rows {
        assert!(
            row.ladder_is_ordered(),
            "row {} has an unordered collapse ladder",
            row.family.as_str()
        );
        assert!(
            row.ladder_terminates_in_placeholder(),
            "row {} does not terminate in a placeholder",
            row.family.as_str()
        );
    }
}

#[test]
fn every_row_presentations_are_declared_covered_and_monotonic() {
    let packet = seeded_m5_responsive_collapse_packet();
    for row in &packet.rows {
        assert!(
            row.presentations_cover_declared_classes(),
            "row {} does not cover its declared responsive classes",
            row.family.as_str()
        );
        assert!(
            row.presentations_placements_declared(),
            "row {} lands in an undeclared placement",
            row.family.as_str()
        );
        assert!(
            row.presentations_monotonic(),
            "row {} presentations are not monotonic",
            row.family.as_str()
        );
        assert!(
            row.presentations_stable(),
            "row {} loses identity or an action at some class",
            row.family.as_str()
        );
        // Every family declares all three responsive classes.
        assert_eq!(row.class_presentations.len(), M5ResponsiveClass::ALL.len());
    }
}

#[test]
fn narrowed_rows_disclose_a_reason() {
    let packet = seeded_m5_responsive_collapse_packet();
    for row in &packet.rows {
        if !matches!(row.derived_status, ResponsiveCollapseStatus::Green) {
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
fn disclosed_state_rehydration_carries_an_active_waiver() {
    let packet = seeded_m5_responsive_collapse_packet();
    let companion = packet.row(M5ShellSurfaceFamily::Companion).unwrap();
    assert!(matches!(
        companion.identity_continuity,
        IdentityContinuityState::DisclosedStateRehydration
    ));
    assert!(companion.requires_waiver());
    assert!(companion.has_active_waiver());
    assert_eq!(packet.active_waivers.len(), 1);
    assert!(packet.active_waivers[0].is_active_at(&packet.generated_at));
}

#[test]
fn disclosed_overflow_reach_keeps_the_profiler_yellow() {
    let packet = seeded_m5_responsive_collapse_packet();
    let profiler = packet.row(M5ShellSurfaceFamily::Profiler).unwrap();
    assert!(matches!(
        profiler.critical_action_reach,
        CriticalActionReachState::DisclosedOverflowReach
    ));
    assert_eq!(profiler.derived_status, ResponsiveCollapseStatus::Yellow);
    // A Stable family narrowed purely by disclosed responsive behavior.
    assert!(profiler.is_stable_qualified());
}

#[test]
fn collapse_identity_change_blocks_the_notebook() {
    let packet = seeded_m5_responsive_collapse_packet_notebook_collapse_identity_blocked();
    let row = packet.row(M5ShellSurfaceFamily::Notebook).unwrap();
    assert_eq!(row.derived_status, ResponsiveCollapseStatus::Red);
    assert!(!packet.report_clean);
    assert!(!packet.all_rows_publishable);
    assert!(packet.red_row_count >= 1);
    assert!(packet.blocking_findings.iter().any(|finding| matches!(
        finding,
        ResponsiveCollapseFinding::LadderChangesIdentity { .. }
    )));
    assert!(row
        .collapse_causes
        .iter()
        .any(|cause| matches!(cause.trigger, M5ShellDowngradeTrigger::CollapseChangedTaskIdentity)));
    assert!(validate_m5_responsive_collapse_packet(&packet).is_err());
}

#[test]
fn hidden_critical_state_blocks_the_profiler() {
    let packet = seeded_m5_responsive_collapse_packet_profiler_critical_state_hidden_blocked();
    let row = packet.row(M5ShellSurfaceFamily::Profiler).unwrap();
    assert_eq!(row.derived_status, ResponsiveCollapseStatus::Red);
    assert!(!packet.report_clean);
    assert!(packet.blocking_findings.iter().any(|finding| matches!(
        finding,
        ResponsiveCollapseFinding::CriticalStateHidden { .. }
    )));
    assert!(row.collapse_causes.iter().any(|cause| matches!(
        cause.trigger,
        M5ShellDowngradeTrigger::CriticalStateHiddenOnCollapse
    )));
    assert!(validate_m5_responsive_collapse_packet(&packet).is_err());
}

#[test]
fn zoom_route_divergence_blocks_the_docs_surface() {
    let packet = seeded_m5_responsive_collapse_packet_docs_zoom_route_divergence_blocked();
    let row = packet.row(M5ShellSurfaceFamily::Docs).unwrap();
    assert_eq!(row.derived_status, ResponsiveCollapseStatus::Red);
    assert!(!packet.report_clean);
    assert!(packet.blocking_findings.iter().any(|finding| matches!(
        finding,
        ResponsiveCollapseFinding::RouteSemanticsDivergeAtZoom { .. }
    )));
    assert!(row.collapse_causes.iter().any(|cause| matches!(
        cause.trigger,
        M5ShellDowngradeTrigger::CollapseChangedTaskIdentity
    )));
    assert!(validate_m5_responsive_collapse_packet(&packet).is_err());
}

#[test]
fn missing_placeholder_terminal_blocks_the_companion() {
    let packet =
        seeded_m5_responsive_collapse_packet_companion_ladder_missing_placeholder_blocked();
    let row = packet.row(M5ShellSurfaceFamily::Companion).unwrap();
    assert_eq!(row.derived_status, ResponsiveCollapseStatus::Red);
    assert!(!row.ladder_terminates_in_placeholder());
    assert!(!packet.report_clean);
    assert!(packet.blocking_findings.iter().any(|finding| matches!(
        finding,
        ResponsiveCollapseFinding::LadderMissingPlaceholderTerminal { .. }
    )));
    assert!(row.collapse_causes.iter().any(|cause| matches!(
        cause.trigger,
        M5ShellDowngradeTrigger::PlaceholderLostIdentityOrReopen
    )));
    assert!(validate_m5_responsive_collapse_packet(&packet).is_err());
}

#[test]
fn presentation_landing_outside_the_declared_ladder_blocks() {
    // Hand-mutate a green row so its compact presentation lands in a placement the
    // family never declared — the presentation lint must block it.
    let mut packet = seeded_m5_responsive_collapse_packet();
    let row = packet
        .rows
        .iter_mut()
        .find(|row| row.family == M5ShellSurfaceFamily::Review)
        .unwrap();
    // Review's ladder is docked→overflow→placeholder; a sheet landing is undeclared.
    for presentation in &mut row.class_presentations {
        if presentation.responsive_class == M5ResponsiveClass::CompactDesktop {
            presentation.placement = M5FallbackPlacement::Sheet;
        }
    }
    assert!(!row.presentations_placements_declared());
    assert_eq!(row.recompute_status(), ResponsiveCollapseStatus::Red);
    let findings = row.compute_findings(&packet.generated_at);
    assert!(findings.iter().any(|finding| matches!(
        finding,
        ResponsiveCollapseFinding::PresentationPlacementUndeclared { .. }
    )));
}

#[test]
fn dashboard_projects_every_row_and_counts() {
    let packet = seeded_m5_responsive_collapse_packet();
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
    assert_eq!(companion.status, ResponsiveCollapseStatus::Yellow);
    assert!(companion.has_active_waiver);
    assert_eq!(
        companion.compact_placement,
        Some(M5FallbackPlacement::Overflow)
    );
}

#[test]
fn support_export_quotes_packet_matrix_and_waiver_refs() {
    let packet = seeded_m5_responsive_collapse_packet();
    let export = ResponsiveCollapseSupportExport::from_packet(
        M5_RESPONSIVE_COLLAPSE_SUPPORT_EXPORT_ID,
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
    let packet = seeded_m5_responsive_collapse_packet();
    let markdown = packet.render_markdown();
    let csv = packet.render_matrix_csv();
    for family in REQUIRED_FAMILIES {
        assert!(
            markdown.contains(surface_label(family)),
            "markdown omits {}",
            family.as_str()
        );
        assert!(
            csv.contains(family.as_str()),
            "csv omits {}",
            family.as_str()
        );
    }
    assert!(markdown.contains("m5_responsive_collapse_fixtures"));
    assert!(markdown.contains("waiver:companion-sheet-rehydration:0001"));
}

#[test]
fn export_carries_no_forbidden_material() {
    let packet = seeded_m5_responsive_collapse_packet();
    let json = packet.export_safe_json().to_lowercase();
    assert!(!json.contains("api_key"));
    assert!(!json.contains("password"));
    assert!(!json.contains("bearer "));
    assert!(!json.contains("://"));
}

#[test]
fn expired_waiver_is_flagged() {
    let waiver = ResponsiveCollapseWaiver {
        waiver_id: "waiver:expired:0001".to_owned(),
        family: M5ShellSurfaceFamily::Companion,
        reason: "test".to_owned(),
        owner_role: "owner".to_owned(),
        expires_at: "2026-01-01T00:00:00Z".to_owned(),
    };
    assert!(!waiver.is_active_at("2026-06-30T00:00:00Z"));
    assert!(waiver.is_active_at("2025-01-01T00:00:00Z"));
}
