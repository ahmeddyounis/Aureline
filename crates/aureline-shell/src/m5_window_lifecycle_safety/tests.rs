//! Inline unit tests for the M5 window-lifecycle-safety proof.

use super::*;

#[test]
fn seeded_packet_covers_every_family_and_is_clean() {
    let packet = seeded_m5_window_lifecycle_safety_packet();
    validate_m5_window_lifecycle_safety_packet(&packet).expect("seeded packet must validate clean");

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
    let packet = seeded_m5_window_lifecycle_safety_packet();
    assert_eq!(packet.green_row_count, 6);
    assert_eq!(packet.yellow_row_count, 4);
    assert_eq!(packet.red_row_count, 0);

    for family in [
        M5ShellSurfaceFamily::Docs,
        M5ShellSurfaceFamily::Incident,
        M5ShellSurfaceFamily::Companion,
        M5ShellSurfaceFamily::Operator,
    ] {
        assert_eq!(
            packet.row(family).unwrap().derived_status,
            WindowLifecycleStatus::Yellow,
            "{} should auto-narrow to yellow",
            family.as_str()
        );
    }
}

#[test]
fn every_row_status_is_the_derived_value() {
    let packet = seeded_m5_window_lifecycle_safety_packet();
    for row in &packet.rows {
        assert_eq!(
            row.derived_status,
            row.recompute_status(),
            "row {} declares a stale status",
            row.family.as_str()
        );
        assert_eq!(row.lifecycle_causes, row.recompute_causes());
    }
}

#[test]
fn every_row_declares_all_protected_resources() {
    let packet = seeded_m5_window_lifecycle_safety_packet();
    for row in &packet.rows {
        assert!(
            row.protected_resources_complete(),
            "row {} does not declare all four protected close resources",
            row.family.as_str()
        );
    }
}

#[test]
fn every_row_plans_cover_and_disclose_verbs() {
    let packet = seeded_m5_window_lifecycle_safety_packet();
    for row in &packet.rows {
        assert!(
            row.plans_cover_required_verbs(),
            "row {} does not cover the canonical drag verbs",
            row.family.as_str()
        );
        assert!(
            row.plans_disclose_before_drop(),
            "row {} hides a drag verb before the drop",
            row.family.as_str()
        );
        assert!(
            row.plans_keyboard_reachable(),
            "row {} loses keyboard parity on a drag verb",
            row.family.as_str()
        );
        for plan in &row.drag_plans {
            assert!(plan.is_fully_disclosed());
        }
    }
}

#[test]
fn narrowed_rows_disclose_a_reason() {
    let packet = seeded_m5_window_lifecycle_safety_packet();
    for row in &packet.rows {
        if !matches!(row.derived_status, WindowLifecycleStatus::Green) {
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
fn disclosed_close_guard_relocation_carries_an_active_waiver() {
    let packet = seeded_m5_window_lifecycle_safety_packet();
    let companion = packet.row(M5ShellSurfaceFamily::Companion).unwrap();
    assert!(matches!(
        companion.close_orphan_guard,
        CloseOrphanGuardState::DisclosedDeferredGuardRelocation
    ));
    assert!(companion.requires_waiver());
    assert!(companion.has_active_waiver());
    assert_eq!(packet.active_waivers.len(), 1);
    assert!(packet.active_waivers[0].is_active_at(&packet.generated_at));
}

#[test]
fn disclosed_drag_verb_reach_keeps_docs_yellow() {
    let packet = seeded_m5_window_lifecycle_safety_packet();
    let docs = packet.row(M5ShellSurfaceFamily::Docs).unwrap();
    assert!(matches!(
        docs.drag_verb_disclosure,
        DragVerbDisclosureState::DisclosedVerbReachNarrowing
    ));
    assert_eq!(docs.derived_status, WindowLifecycleStatus::Yellow);
    // A Stable family narrowed purely by a disclosed drag-verb command-palette reach.
    assert!(docs.is_stable_qualified());
}

#[test]
fn silent_close_orphan_blocks_the_notebook() {
    let packet = seeded_m5_window_lifecycle_safety_packet_notebook_close_silent_orphan_blocked();
    let row = packet.row(M5ShellSurfaceFamily::Notebook).unwrap();
    assert_eq!(row.derived_status, WindowLifecycleStatus::Red);
    assert!(!packet.report_clean);
    assert!(!packet.all_rows_publishable);
    assert!(packet.red_row_count >= 1);
    assert!(packet
        .blocking_findings
        .iter()
        .any(|finding| matches!(finding, WindowLifecycleFinding::CloseSilentOrphan { .. })));
    assert!(row.lifecycle_causes.iter().any(|cause| matches!(
        cause.trigger,
        M5ShellDowngradeTrigger::OwningWindowRoutingLost
    )));
    assert!(validate_m5_window_lifecycle_safety_packet(&packet).is_err());
}

#[test]
fn hidden_drag_verb_blocks_the_preview() {
    let packet = seeded_m5_window_lifecycle_safety_packet_preview_drag_verb_hidden_blocked();
    let row = packet.row(M5ShellSurfaceFamily::Preview).unwrap();
    assert_eq!(row.derived_status, WindowLifecycleStatus::Red);
    assert!(!packet.report_clean);
    assert!(packet.blocking_findings.iter().any(|finding| matches!(
        finding,
        WindowLifecycleFinding::DragVerbHiddenOrKeyboardLost { .. }
    )));
    assert!(row.lifecycle_causes.iter().any(|cause| matches!(
        cause.trigger,
        M5ShellDowngradeTrigger::OwningWindowRoutingLost
    )));
    assert!(validate_m5_window_lifecycle_safety_packet(&packet).is_err());
}

#[test]
fn wrong_surface_reopen_blocks_the_data_grid() {
    let packet = seeded_m5_window_lifecycle_safety_packet_datagrid_reopen_wrong_surface_blocked();
    let row = packet.row(M5ShellSurfaceFamily::DataGrid).unwrap();
    assert_eq!(row.derived_status, WindowLifecycleStatus::Red);
    assert!(!packet.report_clean);
    assert!(packet.blocking_findings.iter().any(|finding| matches!(
        finding,
        WindowLifecycleFinding::ReopenOrphanedOrWrongSurface { .. }
    )));
    assert!(row.lifecycle_causes.iter().any(|cause| matches!(
        cause.trigger,
        M5ShellDowngradeTrigger::PlaceholderLostIdentityOrReopen
    )));
    assert!(validate_m5_window_lifecycle_safety_packet(&packet).is_err());
}

#[test]
fn incomplete_protected_resources_block_the_review() {
    let packet =
        seeded_m5_window_lifecycle_safety_packet_review_protected_resource_orphan_blocked();
    let row = packet.row(M5ShellSurfaceFamily::Review).unwrap();
    assert_eq!(row.derived_status, WindowLifecycleStatus::Red);
    assert!(!packet.report_clean);
    assert!(!row.protected_resources_complete());
    assert!(packet.blocking_findings.iter().any(|finding| matches!(
        finding,
        WindowLifecycleFinding::ProtectedResourcesIncomplete { .. }
    )));
    assert!(row.lifecycle_causes.iter().any(|cause| matches!(
        cause.trigger,
        M5ShellDowngradeTrigger::OwningWindowRoutingLost
    )));
    assert!(validate_m5_window_lifecycle_safety_packet(&packet).is_err());
}

#[test]
fn plan_dropping_pre_drop_disclosure_blocks() {
    // Hand-mutate a green row so one drag plan drops pre-drop disclosure — the plan lint
    // must block it.
    let mut packet = seeded_m5_window_lifecycle_safety_packet();
    let row = packet
        .rows
        .iter_mut()
        .find(|row| row.family == M5ShellSurfaceFamily::Notebook)
        .unwrap();
    row.drag_plans[0].disclosed_before_drop = false;
    assert!(!row.plans_disclose_before_drop());
    assert_eq!(row.recompute_status(), WindowLifecycleStatus::Red);
    let findings = row.compute_findings(&packet.generated_at);
    assert!(findings.iter().any(|finding| matches!(
        finding,
        WindowLifecycleFinding::DragPlanNotDisclosedBeforeDrop { .. }
    )));
}

#[test]
fn dashboard_projects_every_row_and_counts() {
    let packet = seeded_m5_window_lifecycle_safety_packet();
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
    assert_eq!(companion.status, WindowLifecycleStatus::Yellow);
    assert!(companion.has_active_waiver);
    assert!(matches!(
        companion.close_orphan_guard,
        CloseOrphanGuardState::DisclosedDeferredGuardRelocation
    ));
}

#[test]
fn support_export_quotes_packet_matrix_and_waiver_refs() {
    let packet = seeded_m5_window_lifecycle_safety_packet();
    let export = WindowLifecycleSupportExport::from_packet(
        M5_WINDOW_LIFECYCLE_SAFETY_SUPPORT_EXPORT_ID,
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
    let packet = seeded_m5_window_lifecycle_safety_packet();
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
    assert!(markdown.contains("m5_window_lifecycle_safety_fixtures"));
    assert!(markdown.contains("waiver:companion-close-guard-relocation:0001"));
}

#[test]
fn export_carries_no_forbidden_material() {
    let packet = seeded_m5_window_lifecycle_safety_packet();
    let json = packet.export_safe_json().to_lowercase();
    assert!(!json.contains("api_key"));
    assert!(!json.contains("password"));
    assert!(!json.contains("bearer "));
    assert!(!json.contains("://"));
}

#[test]
fn expired_waiver_is_flagged() {
    let waiver = WindowLifecycleWaiver {
        waiver_id: "waiver:expired:0001".to_owned(),
        family: M5ShellSurfaceFamily::Companion,
        reason: "test".to_owned(),
        owner_role: "owner".to_owned(),
        expires_at: "2026-01-01T00:00:00Z".to_owned(),
    };
    assert!(!waiver.is_active_at("2026-06-30T00:00:00Z"));
    assert!(waiver.is_active_at("2025-01-01T00:00:00Z"));
}
