//! Unit tests for the M5 start-center / switcher parity packet.

use super::*;

#[test]
fn seeded_packet_covers_every_surface_class() {
    let packet = seeded_m5_start_center_and_switcher_packet();
    assert!(packet.covers_every_class());
    for class in M5EntrySurfaceClass::required_classes() {
        assert!(
            packet.rows.iter().any(|row| row.surface_class == class),
            "packet must include surface class {}",
            class.as_str()
        );
    }
}

#[test]
fn seeded_packet_validates() {
    let packet = seeded_m5_start_center_and_switcher_packet();
    validate_m5_start_center_and_switcher_packet(&packet).expect("seeded packet must validate");
}

#[test]
fn every_row_holds_full_cross_surface_parity() {
    let packet = seeded_m5_start_center_and_switcher_packet();
    assert!(packet.full_parity);
    for row in &packet.rows {
        assert!(row.present_in_start_center);
        assert!(row.present_in_switcher);
        assert!(
            row.parity_complete(),
            "{} must hold full parity: {:?}",
            row.recent_work_id,
            row.parity
        );
        assert_eq!(row.start_center_trust_state, row.canonical_trust_state);
        assert_eq!(row.switcher_trust_state, row.canonical_trust_state);
        assert_eq!(
            row.start_center_restore_availability,
            row.canonical_restore_availability
        );
        assert_eq!(
            row.switcher_restore_availability,
            row.canonical_restore_availability
        );
    }
}

#[test]
fn no_two_distinct_target_kinds_collapse_into_one_class() {
    let packet = seeded_m5_start_center_and_switcher_packet();
    assert!(packet.no_target_kind_collapsed);
    for row in &packet.rows {
        assert_eq!(
            M5EntrySurfaceClass::from_target_kind(row.target_kind),
            Some(row.surface_class),
            "{} collapsed its target kind",
            row.recent_work_id
        );
    }
}

#[test]
fn failure_rows_never_widen_trust_and_expose_recovery() {
    let packet = seeded_m5_start_center_and_switcher_packet();
    assert!(packet.no_trust_widened);
    for row in &packet.rows {
        assert!(
            row.trust_not_widened(),
            "{} widened trust",
            row.recent_work_id
        );
        if !row.root_resolved() {
            assert!(
                row.reconnect_or_locate_available,
                "{} is unresolved but exposes no reconnect/locate path",
                row.recent_work_id
            );
            assert!(
                row.remove_action_available,
                "{} should let the user remove an unavailable target",
                row.recent_work_id
            );
        }
        assert!(
            row.pin_action_available,
            "{} lost pin/remove",
            row.recent_work_id
        );
        assert!(row.keyboard_complete);
    }
}

#[test]
fn diagnostics_cover_every_required_class_and_stay_export_safe() {
    let packet = seeded_m5_start_center_and_switcher_packet();
    for class in M5DiagnosticClass::required_classes() {
        assert!(
            packet
                .diagnostics
                .iter()
                .any(|diagnostic| diagnostic.diagnostic_class == class),
            "diagnostics must cover {}",
            class.as_str()
        );
    }
    for diagnostic in &packet.diagnostics {
        assert!(diagnostic.export_safe);
        assert_eq!(
            diagnostic.redacted_location,
            diagnostic.target_kind.surface_label(),
            "diagnostic {} leaked a raw location",
            diagnostic.diagnostic_id
        );
        // The redacted location must never echo a raw path or host subtitle.
        assert!(!diagnostic.redacted_location.contains('/'));
        assert!(!diagnostic.redacted_location.contains('~'));
    }
}

#[test]
fn missing_root_row_maps_to_missing_root_diagnostic() {
    let packet = seeded_m5_start_center_and_switcher_packet();
    let row = packet
        .rows
        .iter()
        .find(|row| row.recent_work_id == "recent:m5.missing_root")
        .expect("missing-root row present");
    assert_eq!(row.surface_class, M5EntrySurfaceClass::LocalFolder);
    assert_eq!(row.failure_state, RecentWorkFailureState::MissingPath);
    assert_eq!(row.root_state, M5RootState::MissingRoot);
    assert_eq!(row.diagnostic_class, Some(M5DiagnosticClass::MissingRoot));
    assert!(row
        .start_center_safe_actions
        .contains(&SafeRecoveryAction::LocateMissingTarget));
    assert!(row
        .start_center_safe_actions
        .contains(&SafeRecoveryAction::OpenWithoutRestore));
}

#[test]
fn partial_restore_row_is_reachable_but_layout_only() {
    let packet = seeded_m5_start_center_and_switcher_packet();
    let row = packet
        .rows
        .iter()
        .find(|row| row.recent_work_id == "recent:m5.partial_restore")
        .expect("partial-restore row present");
    assert_eq!(row.surface_class, M5EntrySurfaceClass::ManagedWorkspace);
    assert_eq!(row.root_state, M5RootState::RootResolved);
    assert_eq!(
        row.canonical_restore_availability,
        RestoreAvailability::LayoutOnly
    );
    assert_eq!(
        row.diagnostic_class,
        Some(M5DiagnosticClass::PartialRestore)
    );
}

#[test]
fn packet_round_trips_through_json() {
    let packet = seeded_m5_start_center_and_switcher_packet();
    let json = serde_json::to_string(&packet).expect("serialize");
    let parsed: M5StartCenterSwitcherPacket = serde_json::from_str(&json).expect("deserialize");
    assert_eq!(parsed, packet);
}

#[test]
fn support_export_collects_row_and_diagnostic_ids() {
    let packet = seeded_m5_start_center_and_switcher_packet();
    let export = M5StartCenterSwitcherSupportExport::from_packet(
        "support-export:m5-start-center-and-switcher:001",
        packet.clone(),
    );
    assert_eq!(export.packet, packet);
    assert!(export.case_ids.contains(&packet.packet_id));
    for row in &packet.rows {
        assert!(export.case_ids.contains(&row.recent_work_id));
    }
    for diagnostic in &packet.diagnostics {
        assert!(export.case_ids.contains(&diagnostic.diagnostic_id));
    }
}

#[test]
fn validation_flags_collapsed_surface_class() {
    let mut packet = seeded_m5_start_center_and_switcher_packet();
    packet.rows[0].surface_class = M5EntrySurfaceClass::ManagedWorkspace;
    packet.surface_class_coverage = SurfaceClassCoverageSummary::from_rows(&packet.rows);
    let errors = validate_m5_start_center_and_switcher_packet(&packet)
        .expect_err("collapsed surface class must fail");
    assert!(errors.iter().any(|error| matches!(
        error,
        M5StartCenterSwitcherValidationError::SurfaceClassCollapsed { .. }
    )));
}

#[test]
fn validation_flags_widened_trust() {
    let mut packet = seeded_m5_start_center_and_switcher_packet();
    let row = packet
        .rows
        .iter_mut()
        .find(|row| row.recent_work_id == "recent:m5.managed_workspace")
        .expect("managed row present");
    row.start_center_trust_state = TrustState::Trusted;
    row.parity.trust_parity = false;
    let errors =
        validate_m5_start_center_and_switcher_packet(&packet).expect_err("widened trust must fail");
    assert!(errors.iter().any(|error| matches!(
        error,
        M5StartCenterSwitcherValidationError::TrustWidened { .. }
    )));
}

#[test]
fn validation_flags_diagnostic_leak() {
    let mut packet = seeded_m5_start_center_and_switcher_packet();
    packet.diagnostics[0].redacted_location = "~/Code/payments".to_owned();
    let errors = validate_m5_start_center_and_switcher_packet(&packet)
        .expect_err("leaked location must fail");
    assert!(errors.iter().any(|error| matches!(
        error,
        M5StartCenterSwitcherValidationError::DiagnosticLeaksRawLocation { .. }
    )));
}

#[test]
fn validation_flags_stale_coverage_summary() {
    let mut packet = seeded_m5_start_center_and_switcher_packet();
    packet.surface_class_coverage.covered_classes.pop();
    let errors = validate_m5_start_center_and_switcher_packet(&packet)
        .expect_err("stale coverage must fail");
    assert!(errors.iter().any(|error| matches!(
        error,
        M5StartCenterSwitcherValidationError::CoverageSummaryStale
    )));
}

#[test]
fn compact_lines_and_markdown_render() {
    let packet = seeded_m5_start_center_and_switcher_packet();
    let compact = packet.compact_lines();
    assert!(compact.iter().any(|line| line.contains("packet:")));
    assert!(compact
        .iter()
        .any(|line| line.contains("recent:m5.missing_root")));

    let markdown = packet.render_markdown();
    assert!(markdown.contains("Start Center and workspace-switcher parity"));
    assert!(markdown.contains("Export-safe diagnostics"));
    assert!(markdown.contains("missing_root"));
}
