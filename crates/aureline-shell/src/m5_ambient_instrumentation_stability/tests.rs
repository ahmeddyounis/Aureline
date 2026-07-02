//! Inline unit tests for the M5 ambient-instrumentation stability proof.

use super::*;

#[test]
fn seeded_packet_covers_every_profile_and_is_clean() {
    let packet = seeded_m5_ambient_instrumentation_stability_packet();
    validate_m5_ambient_instrumentation_stability_packet(&packet)
        .expect("seeded packet must validate clean");

    assert_eq!(packet.rows.len(), M5AmbientStabilityProfile::ALL.len());
    for profile in M5AmbientStabilityProfile::ALL {
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
fn seeded_packet_has_four_green_and_four_yellow_rows() {
    let packet = seeded_m5_ambient_instrumentation_stability_packet();
    assert_eq!(packet.green_row_count, 4);
    assert_eq!(packet.yellow_row_count, 4);
    assert_eq!(packet.red_row_count, 0);

    for profile in [
        M5AmbientStabilityProfile::Compact,
        M5AmbientStabilityProfile::HighZoom,
        M5AmbientStabilityProfile::ReducedMotion,
        M5AmbientStabilityProfile::DegradedNetwork,
    ] {
        assert_eq!(
            packet.row(profile).unwrap().derived_status,
            AmbientStabilityStatus::Yellow,
            "{} should auto-narrow to yellow",
            profile.as_str()
        );
    }
}

#[test]
fn every_row_status_is_the_derived_value() {
    let packet = seeded_m5_ambient_instrumentation_stability_packet();
    for row in &packet.rows {
        assert_eq!(
            row.derived_status,
            row.recompute_status(),
            "row {} declares a stale status",
            row.profile.as_str()
        );
        assert_eq!(row.certification_causes, row.recompute_causes());
    }
}

#[test]
fn every_row_keeps_complete_classes_behaviors_and_labels() {
    let packet = seeded_m5_ambient_instrumentation_stability_packet();
    for row in &packet.rows {
        assert!(
            row.status_item_classes_complete(),
            "row {} does not certify every status-item class",
            row.profile.as_str()
        );
        assert!(
            row.overflow_behaviors_complete(),
            "row {} does not certify every overflow behavior",
            row.profile.as_str()
        );
        assert!(
            row.required_labels_complete(),
            "row {} does not certify every ambient-stability required label",
            row.profile.as_str()
        );
        assert!(
            row.never_reflows_around_vanity_items,
            "row {} allows a vanity-item reflow",
            row.profile.as_str()
        );
    }
}

#[test]
fn ambient_matrix_union_covers_all_classes_behaviors_and_full_label_set() {
    // The union across the three ambient families must cover the full frozen status-item and
    // overflow-behavior vocabularies, else no profile could be certified green. Ambient items
    // carry source/provider and freshness truth, so the required-label set is the full six.
    let packet = seeded_m5_ambient_instrumentation_stability_packet();
    let row = packet.row(M5AmbientStabilityProfile::Standard).unwrap();
    assert_eq!(
        row.certified_status_item_classes,
        M5StatusItemClass::ALL.to_vec()
    );
    assert_eq!(
        row.certified_overflow_behaviors,
        M5OverflowBehavior::ALL.to_vec()
    );
    assert_eq!(
        row.required_labels,
        AMBIENT_STABILITY_REQUIRED_LABELS.to_vec()
    );
    assert!(row
        .required_labels
        .contains(&M5PrimitiveRequiredLabel::Freshness));
    assert!(row
        .required_labels
        .contains(&M5PrimitiveRequiredLabel::SourceProvider));
    assert!(!row.certified_source_freshness_labels.is_empty());
}

#[test]
fn narrowed_rows_disclose_a_reason() {
    let packet = seeded_m5_ambient_instrumentation_stability_packet();
    for row in &packet.rows {
        if !matches!(row.derived_status, AmbientStabilityStatus::Green) {
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
fn coarse_grouping_carries_an_active_waiver() {
    let packet = seeded_m5_ambient_instrumentation_stability_packet();
    let reduced = packet
        .row(M5AmbientStabilityProfile::ReducedMotion)
        .unwrap();
    assert!(matches!(
        reduced.grouped_summary,
        GroupedSummaryState::DisclosedCoarseGrouping
    ));
    assert!(reduced.requires_waiver());
    assert!(reduced.has_active_waiver());
    assert_eq!(packet.active_waivers.len(), 1);
    assert!(packet.active_waivers[0].is_active_at(&packet.generated_at));
}

#[test]
fn reduced_counter_detail_narrows_but_does_not_block() {
    let packet = seeded_m5_ambient_instrumentation_stability_packet();
    let compact = packet.row(M5AmbientStabilityProfile::Compact).unwrap();
    assert!(matches!(
        compact.counter_stability,
        CounterSpinnerStabilityState::DisclosedReducedCounterDetail
    ));
    assert_eq!(compact.derived_status, AmbientStabilityStatus::Yellow);
    assert!(compact
        .certification_causes
        .iter()
        .any(|cause| cause.disclosed
            && matches!(
                cause.trigger,
                M5ShellPrimitiveDowngradeTrigger::VanityItemReflow
            )));
}

#[test]
fn status_reflow_blocks_the_compact_profile() {
    let packet = seeded_m5_ambient_instrumentation_stability_packet_compact_status_reflow_blocked();
    let row = packet.row(M5AmbientStabilityProfile::Compact).unwrap();
    assert_eq!(row.derived_status, AmbientStabilityStatus::Red);
    assert!(!packet.report_clean);
    assert!(!packet.all_rows_publishable);
    assert!(packet.red_row_count >= 1);
    assert!(packet
        .blocking_findings
        .iter()
        .any(|finding| matches!(finding, AmbientStabilityFinding::CountersReflow { .. })));
    assert!(row.certification_causes.iter().any(|cause| !cause.disclosed
        && matches!(
            cause.trigger,
            M5ShellPrimitiveDowngradeTrigger::VanityItemReflow
        )));
    assert!(validate_m5_ambient_instrumentation_stability_packet(&packet).is_err());
}

#[test]
fn undiscoverable_overflow_blocks_the_expanded_profile() {
    let packet =
        seeded_m5_ambient_instrumentation_stability_packet_expanded_overflow_undiscoverable_blocked(
        );
    let row = packet.row(M5AmbientStabilityProfile::Expanded).unwrap();
    assert_eq!(row.derived_status, AmbientStabilityStatus::Red);
    assert!(!packet.report_clean);
    assert!(packet.blocking_findings.iter().any(|finding| matches!(
        finding,
        AmbientStabilityFinding::OverflowUndiscoverable { .. }
    )));
    assert!(row.certification_causes.iter().any(|cause| !cause.disclosed
        && matches!(
            cause.trigger,
            M5ShellPrimitiveDowngradeTrigger::HoverOnlyCriticalTruth
        )));
    assert!(validate_m5_ambient_instrumentation_stability_packet(&packet).is_err());
}

#[test]
fn flickering_primitives_block_the_multi_window_profile() {
    let packet =
        seeded_m5_ambient_instrumentation_stability_packet_multi_window_flickering_primitives_blocked();
    let row = packet.row(M5AmbientStabilityProfile::MultiWindow).unwrap();
    assert_eq!(row.derived_status, AmbientStabilityStatus::Red);
    assert!(!packet.report_clean);
    assert!(packet.blocking_findings.iter().any(|finding| matches!(
        finding,
        AmbientStabilityFinding::FlickeringPrimitives { .. }
    )));
    assert!(row.certification_causes.iter().any(|cause| !cause.disclosed
        && matches!(
            cause.trigger,
            M5ShellPrimitiveDowngradeTrigger::SpinnerOnlyState
        )));
    assert!(validate_m5_ambient_instrumentation_stability_packet(&packet).is_err());
}

#[test]
fn absent_export_blocks_the_degraded_network_profile() {
    let packet =
        seeded_m5_ambient_instrumentation_stability_packet_degraded_network_export_absent_blocked();
    let row = packet
        .row(M5AmbientStabilityProfile::DegradedNetwork)
        .unwrap();
    assert_eq!(row.derived_status, AmbientStabilityStatus::Red);
    assert!(!packet.report_clean);
    assert!(packet.blocking_findings.iter().any(|finding| matches!(
        finding,
        AmbientStabilityFinding::StabilityStateAbsentFromCapture { .. }
    )));
    assert!(validate_m5_ambient_instrumentation_stability_packet(&packet).is_err());
}

#[test]
fn vanity_reflow_invariant_blocks_the_high_zoom_profile() {
    let packet =
        seeded_m5_ambient_instrumentation_stability_packet_high_zoom_vanity_reflow_blocked();
    let row = packet.row(M5AmbientStabilityProfile::HighZoom).unwrap();
    assert_eq!(row.derived_status, AmbientStabilityStatus::Red);
    assert!(!packet.report_clean);
    assert!(packet.blocking_findings.iter().any(|finding| matches!(
        finding,
        AmbientStabilityFinding::StatusReflowsAroundVanityItems { .. }
    )));
    assert!(row.certification_causes.iter().any(|cause| !cause.disclosed
        && matches!(
            cause.trigger,
            M5ShellPrimitiveDowngradeTrigger::VanityItemReflow
        )));
    assert!(validate_m5_ambient_instrumentation_stability_packet(&packet).is_err());
}

#[test]
fn every_row_pulls_ambient_bindings_from_the_matrix() {
    let packet = seeded_m5_ambient_instrumentation_stability_packet();
    for row in &packet.rows {
        assert!(!row.certified_status_item_classes.is_empty());
        assert!(!row.certified_overflow_behaviors.is_empty());
        assert!(!row.certified_source_freshness_labels.is_empty());
        assert!(!row.accessibility_routes.is_empty());
        assert!(!row.required_labels.is_empty());
        assert!(!row.consumer_surfaces.is_empty());
        assert!(!row.applicable_downgrade_triggers.is_empty());
        assert_eq!(row.shell_zone_slot, M5ShellZoneSlot::StatusBar);
        assert_eq!(
            row.driven_primitive_families,
            vec![
                M5ShellPrimitiveFamily::StatusBarItem,
                M5ShellPrimitiveFamily::StatusOverflowMenu,
                M5ShellPrimitiveFamily::ProgressIndicator,
            ]
        );
        assert!(row
            .applicable_downgrade_triggers
            .contains(&M5ShellPrimitiveDowngradeTrigger::VanityItemReflow));
        assert!(row
            .applicable_downgrade_triggers
            .contains(&M5ShellPrimitiveDowngradeTrigger::HoverOnlyCriticalTruth));
        assert!(row
            .applicable_downgrade_triggers
            .contains(&M5ShellPrimitiveDowngradeTrigger::SpinnerOnlyState));
    }
}

#[test]
fn dashboard_projects_every_row_and_counts() {
    let packet = seeded_m5_ambient_instrumentation_stability_packet();
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
        .find(|row| row.profile == M5AmbientStabilityProfile::ReducedMotion)
        .unwrap();
    assert_eq!(reduced.status, AmbientStabilityStatus::Yellow);
    assert!(reduced.has_active_waiver);
    assert!(reduced.cause_tokens.contains(
        &M5ShellPrimitiveDowngradeTrigger::SpinnerOnlyState
            .as_str()
            .to_owned()
    ));
}

#[test]
fn support_export_quotes_packet_matrix_and_waiver_refs() {
    let packet = seeded_m5_ambient_instrumentation_stability_packet();
    let export = AmbientStabilitySupportExport::from_packet(
        M5_AMBIENT_INSTRUMENTATION_STABILITY_SUPPORT_EXPORT_ID,
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
    let packet = seeded_m5_ambient_instrumentation_stability_packet();
    let markdown = packet.render_markdown();
    let csv = packet.render_matrix_csv();
    for profile in M5AmbientStabilityProfile::ALL {
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
    assert!(markdown.contains("m5_ambient_instrumentation_stability_fixtures"));
    assert!(markdown.contains("waiver:reduced-motion-coarse-grouping:0001"));
}

#[test]
fn export_carries_no_forbidden_material() {
    let packet = seeded_m5_ambient_instrumentation_stability_packet();
    let json = packet.export_safe_json().to_lowercase();
    assert!(!json.contains("api_key"));
    assert!(!json.contains("password"));
    assert!(!json.contains("secret"));
    assert!(!json.contains("bearer "));
    assert!(!json.contains("://"));
}

#[test]
fn expired_waiver_is_flagged() {
    let waiver = AmbientStabilityWaiver {
        waiver_id: "waiver:expired:0001".to_owned(),
        profile: M5AmbientStabilityProfile::ReducedMotion,
        reason: "test".to_owned(),
        owner_role: "owner".to_owned(),
        expires_at: "2026-01-01T00:00:00Z".to_owned(),
    };
    assert!(!waiver.is_active_at("2026-06-30T00:00:00Z"));
    assert!(waiver.is_active_at("2025-01-01T00:00:00Z"));
}
