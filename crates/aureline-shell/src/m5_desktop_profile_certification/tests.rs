//! Inline unit tests for the M5 desktop-profile certification proof.

use super::*;

#[test]
fn seeded_packet_covers_every_profile_and_is_clean() {
    let packet = seeded_m5_desktop_profile_certification_packet();
    validate_m5_desktop_profile_certification_packet(&packet)
        .expect("seeded packet must validate clean");

    assert_eq!(packet.rows.len(), REQUIRED_PROFILES.len());
    for profile in REQUIRED_PROFILES {
        assert!(
            packet.row(profile).is_some(),
            "missing row for {}",
            profile.as_str()
        );
    }
    assert!(packet.report_clean);
    assert!(packet.blocking_findings.is_empty());
    assert_eq!(packet.red_row_count, 0);
    assert!(packet.all_rows_publishable);
}

#[test]
fn seeded_packet_has_three_green_and_three_yellow_rows() {
    let packet = seeded_m5_desktop_profile_certification_packet();
    assert_eq!(packet.green_row_count, 3);
    assert_eq!(packet.yellow_row_count, 3);
    assert_eq!(packet.red_row_count, 0);

    for profile in [
        M5DesktopProfile::CompactDesktop,
        M5DesktopProfile::MultiMonitor,
        M5DesktopProfile::DependencyMissingRestore,
    ] {
        assert_eq!(
            packet.row(profile).unwrap().derived_status,
            DesktopProfileStatus::Yellow,
            "{} should auto-narrow to yellow",
            profile.as_str()
        );
    }
    for profile in [
        M5DesktopProfile::StandardDesktop,
        M5DesktopProfile::ExpandedDesktop,
        M5DesktopProfile::MixedDpi,
    ] {
        assert_eq!(
            packet.row(profile).unwrap().derived_status,
            DesktopProfileStatus::Green,
            "{} should stay green",
            profile.as_str()
        );
    }
}

#[test]
fn every_row_status_is_the_derived_value() {
    let packet = seeded_m5_desktop_profile_certification_packet();
    for row in &packet.rows {
        assert_eq!(
            row.derived_status,
            row.recompute_status(),
            "row {} declares a stale status",
            row.profile.as_str()
        );
        assert_eq!(row.profile_causes, row.recompute_causes());
    }
}

#[test]
fn every_row_evaluates_all_claimed_families() {
    let packet = seeded_m5_desktop_profile_certification_packet();
    for row in &packet.rows {
        assert!(
            row.families_complete(),
            "row {} does not evaluate all ten claimed surface families",
            row.profile.as_str()
        );
        assert_eq!(row.evaluated_families.len(), REQUIRED_FAMILIES.len());
    }
}

#[test]
fn narrowed_rows_disclose_a_reason() {
    let packet = seeded_m5_desktop_profile_certification_packet();
    for row in &packet.rows {
        if !matches!(row.derived_status, DesktopProfileStatus::Green) {
            assert!(
                row.narrowing_reason
                    .as_deref()
                    .map(|reason| !reason.trim().is_empty())
                    .unwrap_or(false),
                "narrowed row {} hides its reason",
                row.profile.as_str()
            );
        }
    }
}

#[test]
fn disclosed_routing_relocation_carries_an_active_waiver() {
    let packet = seeded_m5_desktop_profile_certification_packet();
    let multi_monitor = packet.row(M5DesktopProfile::MultiMonitor).unwrap();
    assert!(matches!(
        multi_monitor.owning_window_routing,
        OwningWindowRoutingState::DisclosedRoutingRelocation
    ));
    assert!(multi_monitor.requires_waiver());
    assert!(multi_monitor.has_active_waiver());
    assert_eq!(packet.active_waivers.len(), 1);
    assert!(packet.active_waivers[0].is_active_at(&packet.generated_at));
}

#[test]
fn disclosed_collapse_keeps_compact_yellow() {
    let packet = seeded_m5_desktop_profile_certification_packet();
    let compact = packet.row(M5DesktopProfile::CompactDesktop).unwrap();
    assert!(matches!(
        compact.adaptive_layout,
        AdaptiveLayoutState::DisclosedCollapseNarrowing
    ));
    assert_eq!(compact.derived_status, DesktopProfileStatus::Yellow);
    // Compact keeps shell-zone integrity and does not require a waiver for a collapse narrowing.
    assert!(compact.shell_zone_integrity.is_full());
    assert!(!compact.requires_waiver());
}

#[test]
fn private_slot_drift_blocks_the_compact_profile() {
    let packet =
        seeded_m5_desktop_profile_certification_packet_compact_private_slot_drift_blocked();
    let row = packet.row(M5DesktopProfile::CompactDesktop).unwrap();
    assert_eq!(row.derived_status, DesktopProfileStatus::Red);
    assert!(!packet.report_clean);
    assert!(!packet.all_rows_publishable);
    assert!(packet.red_row_count >= 1);
    assert!(packet.blocking_findings.iter().any(|finding| matches!(
        finding,
        DesktopProfileFinding::PrivateSlotDriftDetected { .. }
    )));
    assert!(row.profile_causes.iter().any(|cause| matches!(
        cause.trigger,
        M5ShellDowngradeTrigger::SlotUndeclared
    )));
    assert!(validate_m5_desktop_profile_certification_packet(&packet).is_err());
}

#[test]
fn unusable_pane_blocks_the_compact_profile() {
    let packet = seeded_m5_desktop_profile_certification_packet_compact_unusable_pane_blocked();
    let row = packet.row(M5DesktopProfile::CompactDesktop).unwrap();
    assert_eq!(row.derived_status, DesktopProfileStatus::Red);
    assert!(!packet.report_clean);
    assert!(packet.blocking_findings.iter().any(|finding| matches!(
        finding,
        DesktopProfileFinding::AdaptiveIdentityLostOrUnusablePane { .. }
    )));
    assert!(row.profile_causes.iter().any(|cause| matches!(
        cause.trigger,
        M5ShellDowngradeTrigger::CollapseChangedTaskIdentity
    )));
    assert!(validate_m5_desktop_profile_certification_packet(&packet).is_err());
}

#[test]
fn truth_divergence_blocks_the_multi_monitor_profile() {
    let packet =
        seeded_m5_desktop_profile_certification_packet_multi_monitor_truth_diverged_blocked();
    let row = packet.row(M5DesktopProfile::MultiMonitor).unwrap();
    assert_eq!(row.derived_status, DesktopProfileStatus::Red);
    assert!(!packet.report_clean);
    assert!(packet.blocking_findings.iter().any(|finding| matches!(
        finding,
        DesktopProfileFinding::WorkspaceTruthDivergedAcrossWindows { .. }
    )));
    assert!(row.profile_causes.iter().any(|cause| matches!(
        cause.trigger,
        M5ShellDowngradeTrigger::WorkspaceTruthDivergedAcrossWindows
    )));
    assert!(validate_m5_desktop_profile_certification_packet(&packet).is_err());
}

#[test]
fn lost_routing_blocks_the_dependency_restore_profile() {
    let packet =
        seeded_m5_desktop_profile_certification_packet_dependency_restore_routing_lost_blocked();
    let row = packet.row(M5DesktopProfile::DependencyMissingRestore).unwrap();
    assert_eq!(row.derived_status, DesktopProfileStatus::Red);
    assert!(!packet.report_clean);
    assert!(packet.blocking_findings.iter().any(|finding| matches!(
        finding,
        DesktopProfileFinding::OwningWindowRoutingLost { .. }
    )));
    assert!(row.profile_causes.iter().any(|cause| matches!(
        cause.trigger,
        M5ShellDowngradeTrigger::OwningWindowRoutingLost
    )));
    assert!(validate_m5_desktop_profile_certification_packet(&packet).is_err());
}

#[test]
fn incomplete_family_evaluation_blocks() {
    // Hand-mutate a green row so it evaluates fewer than all ten claimed families — the
    // completeness lint must block it.
    let mut packet = seeded_m5_desktop_profile_certification_packet();
    let row = packet
        .rows
        .iter_mut()
        .find(|row| row.profile == M5DesktopProfile::StandardDesktop)
        .unwrap();
    row.evaluated_families.pop();
    assert!(!row.families_complete());
    assert_eq!(row.recompute_status(), DesktopProfileStatus::Red);
    let findings = row.compute_findings(&packet.generated_at);
    assert!(findings.iter().any(|finding| matches!(
        finding,
        DesktopProfileFinding::EvaluatedFamiliesIncomplete { .. }
    )));
}

#[test]
fn dashboard_projects_every_row_and_counts() {
    let packet = seeded_m5_desktop_profile_certification_packet();
    let dashboard = packet.dashboard();
    assert_eq!(dashboard.rows.len(), packet.rows.len());
    assert_eq!(dashboard.green_row_count, packet.green_row_count);
    assert_eq!(dashboard.yellow_row_count, packet.yellow_row_count);
    assert_eq!(dashboard.red_row_count, packet.red_row_count);
    assert_eq!(dashboard.all_rows_publishable, packet.all_rows_publishable);
    assert_eq!(dashboard.source_packet_ref, packet.packet_id);
    assert!(!dashboard.shell_automation_refs.is_empty());

    let multi_monitor = dashboard
        .rows
        .iter()
        .find(|row| row.profile == M5DesktopProfile::MultiMonitor)
        .unwrap();
    assert_eq!(multi_monitor.status, DesktopProfileStatus::Yellow);
    assert!(multi_monitor.has_active_waiver);
    assert!(matches!(
        multi_monitor.owning_window_routing,
        OwningWindowRoutingState::DisclosedRoutingRelocation
    ));
}

#[test]
fn support_export_quotes_packet_matrix_and_waiver_refs() {
    let packet = seeded_m5_desktop_profile_certification_packet();
    let export = DesktopProfileSupportExport::from_packet(
        M5_DESKTOP_PROFILE_CERTIFICATION_SUPPORT_EXPORT_ID,
        packet.clone(),
    );
    assert!(export.case_ids.contains(&packet.packet_id));
    assert!(export.case_ids.contains(&packet.matrix_packet_ref));
    assert!(export.case_ids.contains(&packet.build_identity_ref));
    for row in &packet.rows {
        assert!(export.case_ids.contains(&row.profile.as_str().to_owned()));
    }
    for waiver in &packet.active_waivers {
        assert!(export.case_ids.contains(&waiver.waiver_id));
    }
    assert_eq!(export.dashboard, packet.dashboard());
}

#[test]
fn markdown_and_csv_name_every_profile() {
    let packet = seeded_m5_desktop_profile_certification_packet();
    let markdown = packet.render_markdown();
    let csv = packet.render_matrix_csv();
    for profile in REQUIRED_PROFILES {
        assert!(
            markdown.contains(profile.label()),
            "markdown omits {}",
            profile.as_str()
        );
        assert!(
            csv.contains(profile.as_str()),
            "csv omits {}",
            profile.as_str()
        );
    }
    assert!(markdown.contains("m5_desktop_profile_certification_fixtures"));
    assert!(markdown.contains("waiver:multi-monitor-routing-relocation:0001"));
}

#[test]
fn export_carries_no_forbidden_material() {
    let packet = seeded_m5_desktop_profile_certification_packet();
    let json = packet.export_safe_json().to_lowercase();
    assert!(!json.contains("api_key"));
    assert!(!json.contains("password"));
    assert!(!json.contains("bearer "));
    assert!(!json.contains("://"));
}

#[test]
fn expired_waiver_is_flagged() {
    let waiver = DesktopProfileWaiver {
        waiver_id: "waiver:expired:0001".to_owned(),
        profile: M5DesktopProfile::MultiMonitor,
        reason: "test".to_owned(),
        owner_role: "owner".to_owned(),
        expires_at: "2026-01-01T00:00:00Z".to_owned(),
    };
    assert!(!waiver.is_active_at("2026-06-30T00:00:00Z"));
    assert!(waiver.is_active_at("2025-01-01T00:00:00Z"));
}
