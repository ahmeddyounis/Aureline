//! Inline unit tests for the M5 accessibility parity proof.

use super::*;

#[test]
fn seeded_packet_covers_every_condition_and_is_clean() {
    let packet = seeded_m5_accessibility_parity_packet();
    validate_m5_accessibility_parity_packet(&packet).expect("seeded packet must validate clean");

    assert_eq!(packet.rows.len(), M5AccessibilityCondition::ALL.len());
    for condition in M5AccessibilityCondition::ALL {
        assert!(
            packet.row(condition).is_some(),
            "missing row for {}",
            condition.as_str()
        );
    }
    assert!(packet.report_clean);
    assert!(packet.blocking_findings.is_empty());
    assert_eq!(packet.red_row_count, 0);
    assert!(packet.all_rows_publishable);
}

#[test]
fn seeded_packet_has_three_green_and_four_yellow_rows() {
    let packet = seeded_m5_accessibility_parity_packet();
    assert_eq!(packet.green_row_count, 3);
    assert_eq!(packet.yellow_row_count, 4);
    assert_eq!(packet.red_row_count, 0);

    for condition in [
        M5AccessibilityCondition::ScreenReaderNarration,
        M5AccessibilityCondition::HighZoom,
        M5AccessibilityCondition::ReducedMotion,
        M5AccessibilityCondition::HighContrast,
    ] {
        assert_eq!(
            packet.row(condition).unwrap().derived_status,
            AccessibilityParityStatus::Yellow,
            "{} should auto-narrow to yellow",
            condition.as_str()
        );
    }
    for condition in [
        M5AccessibilityCondition::KeyboardReach,
        M5AccessibilityCondition::FocusReturn,
        M5AccessibilityCondition::TouchContextAction,
    ] {
        assert_eq!(
            packet.row(condition).unwrap().derived_status,
            AccessibilityParityStatus::Green,
            "{} should stay green",
            condition.as_str()
        );
    }
}

#[test]
fn every_row_status_is_the_derived_value() {
    let packet = seeded_m5_accessibility_parity_packet();
    for row in &packet.rows {
        assert_eq!(
            row.derived_status,
            row.recompute_status(),
            "row {} declares a stale status",
            row.condition.as_str()
        );
        assert_eq!(row.certification_causes, row.recompute_causes());
    }
}

#[test]
fn every_row_keeps_complete_routes_labels_and_invariant() {
    let packet = seeded_m5_accessibility_parity_packet();
    for row in &packet.rows {
        assert!(
            row.accessibility_routes_complete(),
            "row {} does not certify every accessibility route",
            row.condition.as_str()
        );
        assert!(
            row.required_labels_complete(),
            "row {} does not certify every required label",
            row.condition.as_str()
        );
        assert!(
            row.never_pointer_or_hover_only,
            "row {} keeps a critical truth pointer-/hover-only",
            row.condition.as_str()
        );
    }
}

#[test]
fn matrix_union_covers_full_route_and_label_set_and_all_families() {
    // The union across the ten primitive families must cover the full frozen accessibility-route
    // and required-label vocabularies, else no condition could be certified green. The lane
    // certifies all ten families and the full status/representation/pane/progress vocabularies.
    let packet = seeded_m5_accessibility_parity_packet();
    let row = packet.row(M5AccessibilityCondition::KeyboardReach).unwrap();
    assert_eq!(
        row.accessibility_routes,
        ACCESSIBILITY_PARITY_REQUIRED_ROUTES.to_vec()
    );
    assert_eq!(
        row.required_labels,
        ACCESSIBILITY_PARITY_REQUIRED_LABELS.to_vec()
    );
    assert_eq!(
        row.driven_primitive_families,
        M5ShellPrimitiveFamily::ALL.to_vec()
    );
    assert_eq!(
        row.certified_status_item_classes,
        M5StatusItemClass::ALL.to_vec()
    );
    assert_eq!(
        row.certified_overflow_behaviors,
        M5OverflowBehavior::ALL.to_vec()
    );
    assert_eq!(
        row.certified_representation_classes,
        M5RepresentationClass::ALL.to_vec()
    );
    assert_eq!(
        row.certified_promotion_states,
        M5PromotionState::ALL.to_vec()
    );
    assert_eq!(
        row.certified_pane_resize_states,
        M5PaneResizeState::ALL.to_vec()
    );
    assert_eq!(row.certified_progress_states, M5ProgressState::ALL.to_vec());
    assert_eq!(
        row.certified_source_freshness_labels,
        M5SourceFreshnessLabel::ALL.to_vec()
    );
    // The lane spans every shell zone the ten primitives attach to.
    assert!(row
        .certified_shell_zone_slots
        .contains(&M5ShellZoneSlot::StatusBar));
    assert!(row
        .certified_shell_zone_slots
        .contains(&M5ShellZoneSlot::MainWorkspace));
    assert!(row
        .certified_shell_zone_slots
        .contains(&M5ShellZoneSlot::BottomPanel));
    assert!(row
        .certified_shell_zone_slots
        .contains(&M5ShellZoneSlot::TransientOverlay));
}

#[test]
fn narrowed_rows_disclose_a_reason() {
    let packet = seeded_m5_accessibility_parity_packet();
    for row in &packet.rows {
        if !matches!(row.derived_status, AccessibilityParityStatus::Green) {
            assert!(
                row.narrowing_reason
                    .as_deref()
                    .map(|reason| !reason.trim().is_empty())
                    .unwrap_or(false),
                "narrowed row {} hides its reason",
                row.condition.as_str()
            );
        }
    }
}

#[test]
fn reduced_motion_alternative_carries_an_active_waiver() {
    let packet = seeded_m5_accessibility_parity_packet();
    let reduced = packet.row(M5AccessibilityCondition::ReducedMotion).unwrap();
    assert!(matches!(
        reduced.motion_touch_alternative,
        MotionTouchAlternativeState::DisclosedReducedAlternativeDetail
    ));
    assert!(reduced.requires_waiver());
    assert!(reduced.has_active_waiver());
    assert_eq!(packet.active_waivers.len(), 1);
    assert!(packet.active_waivers[0].is_active_at(&packet.generated_at));
}

#[test]
fn reduced_reach_detail_narrows_but_does_not_block() {
    let packet = seeded_m5_accessibility_parity_packet();
    let narration = packet
        .row(M5AccessibilityCondition::ScreenReaderNarration)
        .unwrap();
    assert!(matches!(
        narration.non_visual_reach,
        NonVisualReachState::DisclosedReducedReachDetail
    ));
    assert_eq!(narration.derived_status, AccessibilityParityStatus::Yellow);
    assert!(narration
        .certification_causes
        .iter()
        .any(|cause| cause.disclosed
            && matches!(
                cause.trigger,
                M5ShellPrimitiveDowngradeTrigger::HoverOnlyCriticalTruth
            )));
}

#[test]
fn pointer_or_hover_only_reach_blocks_the_keyboard_reach_condition() {
    let packet =
        seeded_m5_accessibility_parity_packet_keyboard_reach_pointer_or_hover_only_blocked();
    let row = packet.row(M5AccessibilityCondition::KeyboardReach).unwrap();
    assert_eq!(row.derived_status, AccessibilityParityStatus::Red);
    assert!(!packet.report_clean);
    assert!(!packet.all_rows_publishable);
    assert!(packet.red_row_count >= 1);
    assert!(packet.blocking_findings.iter().any(|finding| matches!(
        finding,
        AccessibilityParityFinding::ReachPointerOrHoverOnly { .. }
    )));
    assert!(row.certification_causes.iter().any(|cause| !cause.disclosed
        && matches!(
            cause.trigger,
            M5ShellPrimitiveDowngradeTrigger::HoverOnlyCriticalTruth
        )));
    assert!(validate_m5_accessibility_parity_packet(&packet).is_err());
}

#[test]
fn unreadable_zoom_blocks_the_high_zoom_condition() {
    let packet = seeded_m5_accessibility_parity_packet_high_zoom_unreadable_blocked();
    let row = packet.row(M5AccessibilityCondition::HighZoom).unwrap();
    assert_eq!(row.derived_status, AccessibilityParityStatus::Red);
    assert!(!packet.report_clean);
    assert!(packet.blocking_findings.iter().any(|finding| matches!(
        finding,
        AccessibilityParityFinding::ZoomContrastUnreadable { .. }
    )));
    assert!(row.certification_causes.iter().any(|cause| !cause.disclosed
        && matches!(
            cause.trigger,
            M5ShellPrimitiveDowngradeTrigger::VanityItemReflow
        )));
    assert!(validate_m5_accessibility_parity_packet(&packet).is_err());
}

#[test]
fn motion_only_affordance_blocks_the_reduced_motion_condition() {
    let packet = seeded_m5_accessibility_parity_packet_reduced_motion_motion_only_blocked();
    let row = packet.row(M5AccessibilityCondition::ReducedMotion).unwrap();
    assert_eq!(row.derived_status, AccessibilityParityStatus::Red);
    assert!(!packet.report_clean);
    assert!(packet.blocking_findings.iter().any(|finding| matches!(
        finding,
        AccessibilityParityFinding::MotionOrPointerOnlyAffordance { .. }
    )));
    assert!(row.certification_causes.iter().any(|cause| !cause.disclosed
        && matches!(
            cause.trigger,
            M5ShellPrimitiveDowngradeTrigger::SpinnerOnlyState
        )));
    assert!(validate_m5_accessibility_parity_packet(&packet).is_err());
}

#[test]
fn absent_export_blocks_the_touch_condition() {
    let packet = seeded_m5_accessibility_parity_packet_touch_export_absent_blocked();
    let row = packet
        .row(M5AccessibilityCondition::TouchContextAction)
        .unwrap();
    assert_eq!(row.derived_status, AccessibilityParityStatus::Red);
    assert!(!packet.report_clean);
    assert!(packet.blocking_findings.iter().any(|finding| matches!(
        finding,
        AccessibilityParityFinding::AccessibilityStateAbsentFromCapture { .. }
    )));
    assert!(validate_m5_accessibility_parity_packet(&packet).is_err());
}

#[test]
fn pointer_or_hover_only_invariant_blocks_the_focus_return_condition() {
    let packet = seeded_m5_accessibility_parity_packet_focus_return_pointer_or_hover_only_blocked();
    let row = packet.row(M5AccessibilityCondition::FocusReturn).unwrap();
    assert_eq!(row.derived_status, AccessibilityParityStatus::Red);
    assert!(!packet.report_clean);
    assert!(packet.blocking_findings.iter().any(|finding| matches!(
        finding,
        AccessibilityParityFinding::CriticalTruthPointerOrHoverOnly { .. }
    )));
    assert!(row.certification_causes.iter().any(|cause| !cause.disclosed
        && matches!(
            cause.trigger,
            M5ShellPrimitiveDowngradeTrigger::HoverOnlyCriticalTruth
        )));
    assert!(validate_m5_accessibility_parity_packet(&packet).is_err());
}

#[test]
fn every_row_pulls_primitive_bindings_from_the_matrix() {
    let packet = seeded_m5_accessibility_parity_packet();
    for row in &packet.rows {
        assert!(!row.certified_status_item_classes.is_empty());
        assert!(!row.certified_overflow_behaviors.is_empty());
        assert!(!row.certified_representation_classes.is_empty());
        assert!(!row.certified_promotion_states.is_empty());
        assert!(!row.certified_pane_resize_states.is_empty());
        assert!(!row.certified_progress_states.is_empty());
        assert!(!row.certified_source_freshness_labels.is_empty());
        assert!(!row.accessibility_routes.is_empty());
        assert!(!row.required_labels.is_empty());
        assert!(!row.consumer_surfaces.is_empty());
        assert!(!row.applicable_downgrade_triggers.is_empty());
        assert_eq!(row.driven_primitive_families.len(), 10);
        assert!(row
            .applicable_downgrade_triggers
            .contains(&M5ShellPrimitiveDowngradeTrigger::HoverOnlyCriticalTruth));
        assert!(row
            .applicable_downgrade_triggers
            .contains(&M5ShellPrimitiveDowngradeTrigger::PointerOnlyResize));
        assert!(row
            .applicable_downgrade_triggers
            .contains(&M5ShellPrimitiveDowngradeTrigger::SpinnerOnlyState));
    }
}

#[test]
fn dashboard_projects_every_row_and_counts() {
    let packet = seeded_m5_accessibility_parity_packet();
    let dashboard = packet.dashboard();
    assert_eq!(dashboard.rows.len(), packet.rows.len());
    assert_eq!(dashboard.green_row_count, packet.green_row_count);
    assert_eq!(dashboard.yellow_row_count, packet.yellow_row_count);
    assert_eq!(dashboard.red_row_count, packet.red_row_count);
    assert_eq!(dashboard.all_rows_publishable, packet.all_rows_publishable);
    assert_eq!(dashboard.source_packet_ref, packet.packet_id);
    assert!(!dashboard.shell_automation_refs.is_empty());

    let reduced = dashboard
        .rows
        .iter()
        .find(|row| row.condition == M5AccessibilityCondition::ReducedMotion)
        .unwrap();
    assert_eq!(reduced.status, AccessibilityParityStatus::Yellow);
    assert!(reduced.has_active_waiver);
    assert!(reduced.cause_tokens.contains(
        &M5ShellPrimitiveDowngradeTrigger::SpinnerOnlyState
            .as_str()
            .to_owned()
    ));
}

#[test]
fn support_export_quotes_packet_matrix_and_waiver_refs() {
    let packet = seeded_m5_accessibility_parity_packet();
    let export = AccessibilityParitySupportExport::from_packet(
        M5_ACCESSIBILITY_PARITY_SUPPORT_EXPORT_ID,
        packet.clone(),
    );
    assert!(export.case_ids.contains(&packet.packet_id));
    assert!(export.case_ids.contains(&packet.matrix_packet_ref));
    assert!(export.case_ids.contains(&packet.build_identity_ref));
    for row in &packet.rows {
        assert!(export.case_ids.contains(&row.condition.as_str().to_owned()));
    }
    for waiver in &packet.active_waivers {
        assert!(export.case_ids.contains(&waiver.waiver_id));
    }
    assert_eq!(export.dashboard, packet.dashboard());
}

#[test]
fn markdown_and_csv_name_every_condition() {
    let packet = seeded_m5_accessibility_parity_packet();
    let markdown = packet.render_markdown();
    let csv = packet.render_matrix_csv();
    for condition in M5AccessibilityCondition::ALL {
        assert!(
            markdown.contains(condition.label()),
            "markdown omits {}",
            condition.as_str()
        );
        assert!(
            csv.contains(condition.as_str()),
            "csv omits {}",
            condition.as_str()
        );
    }
    assert!(markdown.contains("m5_accessibility_parity_fixtures"));
    assert!(markdown.contains("waiver:reduced-motion-alternative:0001"));
}

#[test]
fn export_carries_no_forbidden_material() {
    let packet = seeded_m5_accessibility_parity_packet();
    let json = packet.export_safe_json().to_lowercase();
    assert!(!json.contains("api_key"));
    assert!(!json.contains("password"));
    assert!(!json.contains("secret"));
    assert!(!json.contains("bearer "));
    assert!(!json.contains("://"));
}

#[test]
fn expired_waiver_is_flagged() {
    let waiver = AccessibilityParityWaiver {
        waiver_id: "waiver:expired:0001".to_owned(),
        condition: M5AccessibilityCondition::ReducedMotion,
        reason: "test".to_owned(),
        owner_role: "owner".to_owned(),
        expires_at: "2026-01-01T00:00:00Z".to_owned(),
    };
    assert!(!waiver.is_active_at("2026-06-30T00:00:00Z"));
    assert!(waiver.is_active_at("2025-01-01T00:00:00Z"));
}
