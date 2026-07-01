//! Inline unit tests for the M5 min-width-guard proof.

use super::*;

#[test]
fn seeded_packet_covers_every_family_and_is_clean() {
    let packet = seeded_m5_min_width_guards_packet();
    validate_m5_min_width_guards_packet(&packet).expect("seeded packet must validate clean");

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
    let packet = seeded_m5_min_width_guards_packet();
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
            MinWidthGuardStatus::Yellow,
            "{} should auto-narrow to yellow",
            family.as_str()
        );
    }
}

#[test]
fn every_row_status_is_the_derived_value() {
    let packet = seeded_m5_min_width_guards_packet();
    for row in &packet.rows {
        assert_eq!(
            row.derived_status,
            row.recompute_status(),
            "row {} declares a stale status",
            row.family.as_str()
        );
        assert_eq!(row.guard_causes, row.recompute_causes());
    }
}

#[test]
fn every_row_strategy_set_is_ordered_and_has_safe_terminal() {
    let packet = seeded_m5_min_width_guards_packet();
    for row in &packet.rows {
        assert!(
            row.strategy_set_is_ordered(),
            "row {} has an unordered strategy set",
            row.family.as_str()
        );
        assert!(
            row.strategy_set_has_safe_terminal(),
            "row {} has no universal safe terminal",
            row.family.as_str()
        );
        assert!(
            row.primary_strategy_declared(),
            "row {} uses an undeclared primary strategy",
            row.family.as_str()
        );
    }
}

#[test]
fn every_row_meets_its_min_size_floor() {
    let packet = seeded_m5_min_width_guards_packet();
    for row in &packet.rows {
        assert!(
            !row.min_size_below_floor(),
            "row {} declares a minimum below its enforcement floor",
            row.family.as_str()
        );
    }
}

#[test]
fn every_row_plans_are_declared_covered_and_monotonic() {
    let packet = seeded_m5_min_width_guards_packet();
    for row in &packet.rows {
        assert!(
            row.plans_cover_declared_classes(),
            "row {} does not cover its declared responsive classes",
            row.family.as_str()
        );
        assert!(
            row.plans_strategies_declared(),
            "row {} lands in an undeclared strategy",
            row.family.as_str()
        );
        assert!(
            row.plans_monotonic(),
            "row {} plans are not monotonic",
            row.family.as_str()
        );
        assert!(
            row.plans_stable(),
            "row {} loses min size or status at some class",
            row.family.as_str()
        );
        // Every family declares all three responsive classes.
        assert_eq!(row.class_plans.len(), M5ResponsiveClass::ALL.len());
    }
}

#[test]
fn narrowed_rows_disclose_a_reason() {
    let packet = seeded_m5_min_width_guards_packet();
    for row in &packet.rows {
        if !matches!(row.derived_status, MinWidthGuardStatus::Green) {
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
fn disclosed_status_relocation_carries_an_active_waiver() {
    let packet = seeded_m5_min_width_guards_packet();
    let companion = packet.row(M5ShellSurfaceFamily::Companion).unwrap();
    assert!(matches!(
        companion.status_continuity,
        StatusContinuityState::DisclosedStatusRelocation
    ));
    assert!(companion.requires_waiver());
    assert!(companion.has_active_waiver());
    assert_eq!(packet.active_waivers.len(), 1);
    assert!(packet.active_waivers[0].is_active_at(&packet.generated_at));
}

#[test]
fn disclosed_reduced_minimum_keeps_the_profiler_yellow() {
    let packet = seeded_m5_min_width_guards_packet();
    let profiler = packet.row(M5ShellSurfaceFamily::Profiler).unwrap();
    assert!(matches!(
        profiler.min_size_enforcement,
        MinSizeEnforcementState::DisclosedReducedMinimum
    ));
    assert_eq!(profiler.derived_status, MinWidthGuardStatus::Yellow);
    // A Stable family narrowed purely by a disclosed reduced minimum.
    assert!(profiler.is_stable_qualified());
    // The reduced minimum stays above the absolute floor.
    assert!(profiler.min_useful_width_px >= ABSOLUTE_MIN_USEFUL_WIDTH_PX);
    assert!(profiler.min_useful_height_px >= ABSOLUTE_MIN_USEFUL_HEIGHT_PX);
}

#[test]
fn pane_forced_below_minimum_blocks_the_notebook() {
    let packet = seeded_m5_min_width_guards_packet_notebook_pane_below_minimum_blocked();
    let row = packet.row(M5ShellSurfaceFamily::Notebook).unwrap();
    assert_eq!(row.derived_status, MinWidthGuardStatus::Red);
    assert!(!packet.report_clean);
    assert!(!packet.all_rows_publishable);
    assert!(packet.red_row_count >= 1);
    assert!(packet.blocking_findings.iter().any(|finding| matches!(
        finding,
        MinWidthGuardFinding::PaneForcedBelowUsableMinimum { .. }
    )));
    assert!(row.guard_causes.iter().any(|cause| matches!(
        cause.trigger,
        M5ShellDowngradeTrigger::CriticalStateHiddenOnCollapse
    )));
    assert!(validate_m5_min_width_guards_packet(&packet).is_err());
}

#[test]
fn silent_unusable_split_blocks_the_preview() {
    let packet = seeded_m5_min_width_guards_packet_preview_silent_unusable_split_blocked();
    let row = packet.row(M5ShellSurfaceFamily::Preview).unwrap();
    assert_eq!(row.derived_status, MinWidthGuardStatus::Red);
    assert!(!packet.report_clean);
    assert!(packet.blocking_findings.iter().any(|finding| matches!(
        finding,
        MinWidthGuardFinding::SilentUnusableSplit { .. }
    )));
    assert!(row.guard_causes.iter().any(|cause| matches!(
        cause.trigger,
        M5ShellDowngradeTrigger::CriticalStateHiddenOnCollapse
    )));
    assert!(validate_m5_min_width_guards_packet(&packet).is_err());
}

#[test]
fn lost_status_blocks_the_data_grid() {
    let packet = seeded_m5_min_width_guards_packet_datagrid_status_lost_blocked();
    let row = packet.row(M5ShellSurfaceFamily::DataGrid).unwrap();
    assert_eq!(row.derived_status, MinWidthGuardStatus::Red);
    assert!(!packet.report_clean);
    assert!(packet.blocking_findings.iter().any(|finding| matches!(
        finding,
        MinWidthGuardFinding::StatusOrIdentityLostUnderFallback { .. }
    )));
    assert!(row.guard_causes.iter().any(|cause| matches!(
        cause.trigger,
        M5ShellDowngradeTrigger::CollapseChangedTaskIdentity
    )));
    assert!(validate_m5_min_width_guards_packet(&packet).is_err());
}

#[test]
fn missing_safe_terminal_blocks_the_companion() {
    let packet = seeded_m5_min_width_guards_packet_companion_strategy_set_no_terminal_blocked();
    let row = packet.row(M5ShellSurfaceFamily::Companion).unwrap();
    assert_eq!(row.derived_status, MinWidthGuardStatus::Red);
    assert!(!row.strategy_set_has_safe_terminal());
    assert!(!packet.report_clean);
    assert!(packet.blocking_findings.iter().any(|finding| matches!(
        finding,
        MinWidthGuardFinding::StrategySetMissingSafeTerminal { .. }
    )));
    assert!(row.guard_causes.iter().any(|cause| matches!(
        cause.trigger,
        M5ShellDowngradeTrigger::PlaceholderLostIdentityOrReopen
    )));
    assert!(validate_m5_min_width_guards_packet(&packet).is_err());
}

#[test]
fn plan_landing_outside_the_declared_set_blocks() {
    // Hand-mutate a green row so its compact plan lands in a strategy the family never
    // declared — the plan lint must block it.
    let mut packet = seeded_m5_min_width_guards_packet();
    let row = packet
        .rows
        .iter_mut()
        .find(|row| row.family == M5ShellSurfaceFamily::Companion)
        .unwrap();
    // Companion never declares a side-by-side split; a side-by-side plan is undeclared.
    for plan in &mut row.class_plans {
        if plan.responsive_class == M5ResponsiveClass::CompactDesktop {
            plan.strategy = M5CompareFallbackStrategy::SideBySideSplit;
        }
    }
    assert!(!row.plans_strategies_declared());
    assert_eq!(row.recompute_status(), MinWidthGuardStatus::Red);
    let findings = row.compute_findings(&packet.generated_at);
    assert!(findings.iter().any(|finding| matches!(
        finding,
        MinWidthGuardFinding::PlanStrategyUndeclared { .. }
    )));
}

#[test]
fn dashboard_projects_every_row_and_counts() {
    let packet = seeded_m5_min_width_guards_packet();
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
    assert_eq!(companion.status, MinWidthGuardStatus::Yellow);
    assert!(companion.has_active_waiver);
    assert_eq!(
        companion.compact_strategy,
        Some(M5CompareFallbackStrategy::SequentialDisclosure)
    );
}

#[test]
fn support_export_quotes_packet_matrix_and_waiver_refs() {
    let packet = seeded_m5_min_width_guards_packet();
    let export = MinWidthGuardSupportExport::from_packet(
        M5_MIN_WIDTH_GUARDS_SUPPORT_EXPORT_ID,
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
    let packet = seeded_m5_min_width_guards_packet();
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
    assert!(markdown.contains("m5_min_width_guards_fixtures"));
    assert!(markdown.contains("waiver:companion-status-relocation:0001"));
}

#[test]
fn export_carries_no_forbidden_material() {
    let packet = seeded_m5_min_width_guards_packet();
    let json = packet.export_safe_json().to_lowercase();
    assert!(!json.contains("api_key"));
    assert!(!json.contains("password"));
    assert!(!json.contains("bearer "));
    assert!(!json.contains("://"));
}

#[test]
fn expired_waiver_is_flagged() {
    let waiver = MinWidthGuardWaiver {
        waiver_id: "waiver:expired:0001".to_owned(),
        family: M5ShellSurfaceFamily::Companion,
        reason: "test".to_owned(),
        owner_role: "owner".to_owned(),
        expires_at: "2026-01-01T00:00:00Z".to_owned(),
    };
    assert!(!waiver.is_active_at("2026-06-30T00:00:00Z"));
    assert!(waiver.is_active_at("2025-01-01T00:00:00Z"));
}
