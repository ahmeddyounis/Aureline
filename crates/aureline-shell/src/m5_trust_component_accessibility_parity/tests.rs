//! Inline unit tests for the M5 trust-component accessibility parity proof.

use super::*;

#[test]
fn seeded_packet_covers_every_condition_and_is_clean() {
    let packet = seeded_m5_trust_component_accessibility_parity_packet();
    validate_m5_trust_component_accessibility_parity_packet(&packet)
        .expect("seeded packet must validate clean");

    assert_eq!(packet.rows.len(), M5TrustAccessibilityCondition::ALL.len());
    for condition in M5TrustAccessibilityCondition::ALL {
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
    let packet = seeded_m5_trust_component_accessibility_parity_packet();
    assert_eq!(packet.green_row_count, 3);
    assert_eq!(packet.yellow_row_count, 4);
    assert_eq!(packet.red_row_count, 0);

    for condition in [
        M5TrustAccessibilityCondition::ScreenReaderNarration,
        M5TrustAccessibilityCondition::HighZoom,
        M5TrustAccessibilityCondition::ReducedMotion,
        M5TrustAccessibilityCondition::HighContrast,
    ] {
        assert_eq!(
            packet.row(condition).unwrap().derived_status,
            TrustComponentParityStatus::Yellow,
            "{} should auto-narrow to yellow",
            condition.as_str()
        );
    }
    for condition in [
        M5TrustAccessibilityCondition::KeyboardReach,
        M5TrustAccessibilityCondition::FocusOrder,
        M5TrustAccessibilityCondition::DensityCompaction,
    ] {
        assert_eq!(
            packet.row(condition).unwrap().derived_status,
            TrustComponentParityStatus::Green,
            "{} should stay green",
            condition.as_str()
        );
    }
}

#[test]
fn every_row_status_is_the_derived_value() {
    let packet = seeded_m5_trust_component_accessibility_parity_packet();
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
    let packet = seeded_m5_trust_component_accessibility_parity_packet();
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
            row.never_hover_color_only_or_compaction_lost,
            "row {} keeps a critical truth hover-/color-only or compaction-lost",
            row.condition.as_str()
        );
    }
}

#[test]
fn matrix_union_covers_full_vocabularies_and_all_families() {
    // The union across the six component families must cover the full frozen accessibility-route
    // and required-label vocabularies, else no condition could be certified green. The lane
    // certifies all six families and the full per-family vocabularies.
    let packet = seeded_m5_trust_component_accessibility_parity_packet();
    let row = packet
        .row(M5TrustAccessibilityCondition::KeyboardReach)
        .unwrap();
    assert_eq!(
        row.accessibility_routes,
        TRUST_COMPONENT_PARITY_REQUIRED_ROUTES.to_vec()
    );
    assert_eq!(
        row.required_labels,
        TRUST_COMPONENT_PARITY_REQUIRED_LABELS.to_vec()
    );
    assert_eq!(
        row.driven_component_families,
        M5TrustComponentFamily::ALL.to_vec()
    );
    assert_eq!(
        row.certified_settings_row_states,
        M5SettingsRowState::ALL.to_vec()
    );
    assert_eq!(
        row.certified_source_pills,
        M5SettingSourcePill::ALL.to_vec()
    );
    assert_eq!(
        row.certified_consequence_classes,
        M5CapabilityConsequenceClass::ALL.to_vec()
    );
    assert_eq!(
        row.certified_capability_scope_states,
        M5CapabilityScopeState::ALL.to_vec()
    );
    assert_eq!(
        row.certified_chronology_verbs,
        M5ChronologyVerb::ALL.to_vec()
    );
    assert_eq!(
        row.certified_provenance_badges,
        M5ProvenanceBadge::ALL.to_vec()
    );
    assert_eq!(
        row.certified_chronology_detail_states,
        M5ChronologyDetailState::ALL.to_vec()
    );
    assert_eq!(
        row.certified_chronology_export_fields,
        M5ChronologyExportField::ALL.to_vec()
    );
    assert_eq!(
        row.certified_responsive_classes,
        M5ResponsiveClass::ALL.to_vec()
    );
    assert_eq!(row.certified_window_classes, M5WindowClass::ALL.to_vec());
    assert_eq!(
        row.certified_surface_families,
        M5ShellSurfaceFamily::ALL.to_vec()
    );
    // The lane spans every shell zone the six components attach to.
    assert!(row
        .certified_shell_zone_slots
        .contains(&M5ShellZoneSlot::MainWorkspace));
    assert!(row
        .certified_shell_zone_slots
        .contains(&M5ShellZoneSlot::TransientOverlay));
    assert!(row
        .certified_shell_zone_slots
        .contains(&M5ShellZoneSlot::BottomPanel));
    assert!(row
        .certified_shell_zone_slots
        .contains(&M5ShellZoneSlot::RightInspector));
}

#[test]
fn narrowed_rows_disclose_a_reason() {
    let packet = seeded_m5_trust_component_accessibility_parity_packet();
    for row in &packet.rows {
        if !matches!(row.derived_status, TrustComponentParityStatus::Green) {
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
    let packet = seeded_m5_trust_component_accessibility_parity_packet();
    let reduced = packet
        .row(M5TrustAccessibilityCondition::ReducedMotion)
        .unwrap();
    assert!(matches!(
        reduced.motion_alternative,
        MotionAlternativeState::DisclosedReducedAlternativeDetail
    ));
    assert!(reduced.requires_waiver());
    assert!(reduced.has_active_waiver());
    assert_eq!(packet.active_waivers.len(), 1);
    assert!(packet.active_waivers[0].is_active_at(&packet.generated_at));
}

#[test]
fn reduced_reach_detail_narrows_but_does_not_block() {
    let packet = seeded_m5_trust_component_accessibility_parity_packet();
    let narration = packet
        .row(M5TrustAccessibilityCondition::ScreenReaderNarration)
        .unwrap();
    assert!(matches!(
        narration.non_visual_reach,
        NonVisualReachState::DisclosedReducedReachDetail
    ));
    assert_eq!(narration.derived_status, TrustComponentParityStatus::Yellow);
    assert!(narration
        .certification_causes
        .iter()
        .any(|cause| cause.disclosed
            && matches!(
                cause.trigger,
                M5TrustComponentDowngradeTrigger::AuditTruthLostOffPrimarySurface
            )));
}

#[test]
fn pointer_or_hover_only_reach_blocks_the_keyboard_reach_condition() {
    let packet =
        seeded_m5_trust_component_accessibility_parity_packet_keyboard_reach_pointer_or_hover_only_blocked();
    let row = packet
        .row(M5TrustAccessibilityCondition::KeyboardReach)
        .unwrap();
    assert_eq!(row.derived_status, TrustComponentParityStatus::Red);
    assert!(!packet.report_clean);
    assert!(!packet.all_rows_publishable);
    assert!(packet.red_row_count >= 1);
    assert!(packet.blocking_findings.iter().any(|finding| matches!(
        finding,
        TrustComponentParityFinding::ReachPointerOrHoverOnly { .. }
    )));
    assert!(row.certification_causes.iter().any(|cause| !cause.disclosed
        && matches!(
            cause.trigger,
            M5TrustComponentDowngradeTrigger::AuditTruthLostOffPrimarySurface
        )));
    assert!(validate_m5_trust_component_accessibility_parity_packet(&packet).is_err());
}

#[test]
fn unreadable_zoom_blocks_the_high_zoom_condition() {
    let packet =
        seeded_m5_trust_component_accessibility_parity_packet_high_zoom_unreadable_blocked();
    let row = packet.row(M5TrustAccessibilityCondition::HighZoom).unwrap();
    assert_eq!(row.derived_status, TrustComponentParityStatus::Red);
    assert!(!packet.report_clean);
    assert!(packet.blocking_findings.iter().any(|finding| matches!(
        finding,
        TrustComponentParityFinding::ZoomContrastDensityUnreadable { .. }
    )));
    assert!(validate_m5_trust_component_accessibility_parity_packet(&packet).is_err());
}

#[test]
fn motion_only_affordance_blocks_the_reduced_motion_condition() {
    let packet =
        seeded_m5_trust_component_accessibility_parity_packet_reduced_motion_motion_only_blocked();
    let row = packet
        .row(M5TrustAccessibilityCondition::ReducedMotion)
        .unwrap();
    assert_eq!(row.derived_status, TrustComponentParityStatus::Red);
    assert!(!packet.report_clean);
    assert!(packet.blocking_findings.iter().any(|finding| matches!(
        finding,
        TrustComponentParityFinding::MotionOnlyAffordance { .. }
    )));
    assert!(validate_m5_trust_component_accessibility_parity_packet(&packet).is_err());
}

#[test]
fn absent_export_blocks_the_high_contrast_condition() {
    let packet =
        seeded_m5_trust_component_accessibility_parity_packet_high_contrast_export_absent_blocked();
    let row = packet
        .row(M5TrustAccessibilityCondition::HighContrast)
        .unwrap();
    assert_eq!(row.derived_status, TrustComponentParityStatus::Red);
    assert!(!packet.report_clean);
    assert!(packet.blocking_findings.iter().any(|finding| matches!(
        finding,
        TrustComponentParityFinding::ComponentStateAbsentFromCapture { .. }
    )));
    assert!(validate_m5_trust_component_accessibility_parity_packet(&packet).is_err());
}

#[test]
fn hover_color_only_invariant_blocks_the_focus_order_condition() {
    let packet =
        seeded_m5_trust_component_accessibility_parity_packet_focus_order_hover_color_only_blocked(
        );
    let row = packet
        .row(M5TrustAccessibilityCondition::FocusOrder)
        .unwrap();
    assert_eq!(row.derived_status, TrustComponentParityStatus::Red);
    assert!(!packet.report_clean);
    assert!(packet.blocking_findings.iter().any(|finding| matches!(
        finding,
        TrustComponentParityFinding::CriticalTruthHoverColorOnlyOrCompactionLost { .. }
    )));
    assert!(row.certification_causes.iter().any(|cause| !cause.disclosed
        && matches!(
            cause.trigger,
            M5TrustComponentDowngradeTrigger::AuditTruthLostOffPrimarySurface
        )));
    assert!(validate_m5_trust_component_accessibility_parity_packet(&packet).is_err());
}

#[test]
fn every_row_pulls_component_bindings_from_the_matrix() {
    let packet = seeded_m5_trust_component_accessibility_parity_packet();
    for row in &packet.rows {
        assert!(!row.certified_settings_row_states.is_empty());
        assert!(!row.certified_source_pills.is_empty());
        assert!(!row.certified_consequence_classes.is_empty());
        assert!(!row.certified_capability_scope_states.is_empty());
        assert!(!row.certified_chronology_verbs.is_empty());
        assert!(!row.certified_provenance_badges.is_empty());
        assert!(!row.certified_chronology_detail_states.is_empty());
        assert!(!row.certified_chronology_export_fields.is_empty());
        assert!(!row.accessibility_routes.is_empty());
        assert!(!row.required_labels.is_empty());
        assert!(!row.consumer_surfaces.is_empty());
        assert!(!row.applicable_downgrade_triggers.is_empty());
        assert_eq!(row.driven_component_families.len(), 6);
        assert!(row
            .applicable_downgrade_triggers
            .contains(&M5TrustComponentDowngradeTrigger::AuditTruthLostOffPrimarySurface));
        assert!(row
            .applicable_downgrade_triggers
            .contains(&M5TrustComponentDowngradeTrigger::ProofStale));
        assert!(row
            .applicable_downgrade_triggers
            .contains(&M5TrustComponentDowngradeTrigger::EffectiveConfiguredConflated));
    }
}

#[test]
fn dashboard_projects_every_row_and_counts() {
    let packet = seeded_m5_trust_component_accessibility_parity_packet();
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
        .find(|row| row.condition == M5TrustAccessibilityCondition::ReducedMotion)
        .unwrap();
    assert_eq!(reduced.status, TrustComponentParityStatus::Yellow);
    assert!(reduced.has_active_waiver);
    assert!(reduced.cause_tokens.contains(
        &M5TrustComponentDowngradeTrigger::AuditTruthLostOffPrimarySurface
            .as_str()
            .to_owned()
    ));
}

#[test]
fn support_export_quotes_packet_matrix_and_waiver_refs() {
    let packet = seeded_m5_trust_component_accessibility_parity_packet();
    let export = TrustComponentParitySupportExport::from_packet(
        M5_TRUST_COMPONENT_PARITY_SUPPORT_EXPORT_ID,
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
    let packet = seeded_m5_trust_component_accessibility_parity_packet();
    let markdown = packet.render_markdown();
    let csv = packet.render_matrix_csv();
    for condition in M5TrustAccessibilityCondition::ALL {
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
    assert!(markdown.contains("m5_trust_component_accessibility_parity_fixtures"));
    assert!(markdown.contains("waiver:reduced-motion-alternative:0001"));
}

#[test]
fn export_carries_no_forbidden_material() {
    let packet = seeded_m5_trust_component_accessibility_parity_packet();
    let json = packet.export_safe_json().to_lowercase();
    assert!(!json.contains("api_key"));
    assert!(!json.contains("password"));
    assert!(!json.contains("secret"));
    assert!(!json.contains("bearer "));
    assert!(!json.contains("://"));
}

#[test]
fn expired_waiver_is_flagged() {
    let waiver = TrustComponentParityWaiver {
        waiver_id: "waiver:expired:0001".to_owned(),
        condition: M5TrustAccessibilityCondition::ReducedMotion,
        reason: "test".to_owned(),
        owner_role: "owner".to_owned(),
        expires_at: "2026-01-01T00:00:00Z".to_owned(),
    };
    assert!(!waiver.is_active_at("2026-06-30T00:00:00Z"));
    assert!(waiver.is_active_at("2025-01-01T00:00:00Z"));
}
