//! Inline unit tests for the M5 discoverability-affordance-parity certification.

use super::*;

#[test]
fn seeded_packet_covers_every_affordance_and_is_clean() {
    let packet = seeded_m5_discoverability_affordance_parity_packet();
    validate_m5_affordance_parity_packet(&packet).expect("seeded packet must validate clean");

    assert_eq!(packet.rows.len(), REQUIRED_AFFORDANCES.len());
    for affordance in REQUIRED_AFFORDANCES {
        assert!(
            packet.row(affordance).is_some(),
            "missing row for {}",
            affordance.as_str()
        );
    }
    assert!(packet.report_clean);
    assert!(packet.blocking_findings.is_empty());
    assert_eq!(packet.red_row_count, 0);
    assert!(packet.all_rows_publishable);
}

#[test]
fn seeded_packet_has_three_green_and_four_yellow_rows() {
    let packet = seeded_m5_discoverability_affordance_parity_packet();
    assert_eq!(packet.green_row_count, 3);
    assert_eq!(packet.yellow_row_count, 4);
    assert_eq!(packet.red_row_count, 0);

    for affordance in [
        M5ConvenienceAffordance::InlineAffordance,
        M5ConvenienceAffordance::Tooltip,
        M5ConvenienceAffordance::VoiceHint,
        M5ConvenienceAffordance::CompanionHandoff,
    ] {
        assert_eq!(
            packet.row(affordance).unwrap().derived_status,
            AffordanceParityStatus::Yellow,
            "{} should auto-narrow to yellow",
            affordance.as_str()
        );
    }
}

#[test]
fn every_row_status_is_the_derived_value() {
    let packet = seeded_m5_discoverability_affordance_parity_packet();
    for row in &packet.rows {
        assert_eq!(
            row.derived_status,
            row.recompute_status(),
            "row {} declares a stale status",
            row.affordance.as_str()
        );
        assert_eq!(row.conformance_causes, row.recompute_causes());
    }
}

#[test]
fn every_row_reuses_all_record_fields_and_reach_modes() {
    let packet = seeded_m5_discoverability_affordance_parity_packet();
    for row in &packet.rows {
        assert!(
            row.record_fields_complete(),
            "row {} does not reuse all six canonical record fields",
            row.affordance.as_str()
        );
        assert!(
            row.reach_modes_complete(),
            "row {} does not stay reachable in all five reach modes",
            row.affordance.as_str()
        );
        assert_eq!(
            row.certified_record_fields.len(),
            REQUIRED_RECORD_FIELDS.len()
        );
        assert_eq!(row.certified_reach_modes.len(), REQUIRED_REACH_MODES.len());
    }
}

#[test]
fn every_row_certifies_all_declared_consumer_surfaces() {
    let packet = seeded_m5_discoverability_affordance_parity_packet();
    for row in &packet.rows {
        assert!(
            row.consumer_surfaces_complete(),
            "row {} does not certify all declared consumer surfaces",
            row.affordance.as_str()
        );
        assert!(row.headless_parity_preserved);
    }
}

#[test]
fn every_binding_is_pulled_from_the_frozen_matrix() {
    use crate::freeze_the_m5_menu_keybinding_resolver_and_command_documentation_matrix::seeded_m5_discoverability_matrix;

    let matrix = seeded_m5_discoverability_matrix();
    let packet = seeded_m5_discoverability_affordance_parity_packet();
    for row in &packet.rows {
        assert_eq!(
            row.driving_surface_family,
            row.affordance.driving_surface_family()
        );
        let surface = matrix
            .surface_rows
            .iter()
            .find(|surface| surface.surface_family == row.driving_surface_family)
            .expect("driving surface family is frozen by the matrix");
        assert_eq!(row.qualification, surface.qualification);
        assert_eq!(row.owner_role, surface.owner_role);
        assert_eq!(
            row.canonical_command_binding,
            surface.canonical_command_binding
        );
        assert_eq!(
            row.lifecycle_label,
            surface.canonical_command_binding.lifecycle_label
        );
        assert_eq!(
            row.preview_class,
            surface.canonical_command_binding.preview_class
        );
        assert_eq!(
            row.disabled_reason_mode,
            surface.canonical_command_binding.disabled_reason_mode
        );
        assert_eq!(row.required_labels, surface.required_labels);
        assert_eq!(row.feature_families, surface.feature_families);
        assert_eq!(row.required_consumer_surfaces, surface.consumer_surfaces);
        assert_eq!(
            row.applicable_downgrade_triggers,
            surface.downgrade_triggers
        );
        assert!(!row
            .canonical_command_binding
            .command_id_field
            .trim()
            .is_empty());
    }
}

#[test]
fn each_affordance_drives_a_distinct_surface_family() {
    use std::collections::BTreeSet;
    let families: BTreeSet<&str> = M5ConvenienceAffordance::ALL
        .iter()
        .map(|affordance| affordance.driving_surface_family().as_str())
        .collect();
    assert_eq!(families.len(), M5ConvenienceAffordance::ALL.len());
}

#[test]
fn narrowed_rows_disclose_a_reason() {
    let packet = seeded_m5_discoverability_affordance_parity_packet();
    for row in &packet.rows {
        if !matches!(row.derived_status, AffordanceParityStatus::Green) {
            assert!(
                row.narrowing_reason
                    .as_deref()
                    .map(|reason| !reason.trim().is_empty())
                    .unwrap_or(false),
                "narrowed row {} hides its reason",
                row.affordance.as_str()
            );
        }
    }
}

#[test]
fn disclosed_reduced_hover_fallback_carries_an_active_waiver() {
    let packet = seeded_m5_discoverability_affordance_parity_packet();
    let companion = packet
        .row(M5ConvenienceAffordance::CompanionHandoff)
        .unwrap();
    assert!(matches!(
        companion.authority_reach,
        AuthorityReachState::DisclosedReducedHoverFallback
    ));
    assert!(companion.requires_waiver());
    assert!(companion.has_active_waiver());
    assert_eq!(packet.active_waivers.len(), 1);
    assert!(packet.active_waivers[0].is_active_at(&packet.generated_at));
}

#[test]
fn disclosed_shortened_label_keeps_inline_affordance_yellow_without_a_waiver() {
    let packet = seeded_m5_discoverability_affordance_parity_packet();
    let inline = packet
        .row(M5ConvenienceAffordance::InlineAffordance)
        .unwrap();
    assert!(matches!(
        inline.label_reuse,
        LabelReuseState::DisclosedShortenedAffordanceLabel
    ));
    assert_eq!(inline.derived_status, AffordanceParityStatus::Yellow);
    assert!(inline.authority_reach.is_full());
    assert!(!inline.requires_waiver());
}

#[test]
fn private_label_blocks_the_button() {
    let packet = seeded_m5_discoverability_affordance_parity_packet_button_private_label_blocked();
    let row = packet.row(M5ConvenienceAffordance::Button).unwrap();
    assert_eq!(row.derived_status, AffordanceParityStatus::Red);
    assert!(!packet.report_clean);
    assert!(!packet.all_rows_publishable);
    assert!(packet.red_row_count >= 1);
    assert!(packet
        .blocking_findings
        .iter()
        .any(|finding| matches!(finding, AffordanceParityFinding::LabelReuseBroken { .. })));
    assert!(row.conformance_causes.iter().any(|cause| matches!(
        cause.trigger,
        M5DiscoverabilityDowngradeTrigger::AlternateLabelInvented
    )));
    assert!(validate_m5_affordance_parity_packet(&packet).is_err());
}

#[test]
fn weakened_side_effect_truth_blocks_the_tooltip() {
    let packet =
        seeded_m5_discoverability_affordance_parity_packet_tooltip_side_effect_weakened_blocked();
    let row = packet.row(M5ConvenienceAffordance::Tooltip).unwrap();
    assert_eq!(row.derived_status, AffordanceParityStatus::Red);
    assert!(!packet.report_clean);
    assert!(packet.blocking_findings.iter().any(|finding| matches!(
        finding,
        AffordanceParityFinding::SideEffectTruthBroken { .. }
    )));
    assert!(row.conformance_causes.iter().any(|cause| matches!(
        cause.trigger,
        M5DiscoverabilityDowngradeTrigger::PreviewApprovalMasked
    )));
    assert!(validate_m5_affordance_parity_packet(&packet).is_err());
}

#[test]
fn authority_overreach_blocks_the_companion_handoff() {
    let packet =
        seeded_m5_discoverability_affordance_parity_packet_companion_authority_overreach_blocked();
    let row = packet
        .row(M5ConvenienceAffordance::CompanionHandoff)
        .unwrap();
    assert_eq!(row.derived_status, AffordanceParityStatus::Red);
    assert!(!row.has_active_waiver());
    assert!(!packet.report_clean);
    assert!(packet.blocking_findings.iter().any(|finding| matches!(
        finding,
        AffordanceParityFinding::AuthorityReachBroken { .. }
    )));
    assert!(row.conformance_causes.iter().any(|cause| matches!(
        cause.trigger,
        M5DiscoverabilityDowngradeTrigger::AuthorityWidened
    )));
    assert!(validate_m5_affordance_parity_packet(&packet).is_err());
}

#[test]
fn absent_origin_capture_blocks_the_voice_hint() {
    let packet =
        seeded_m5_discoverability_affordance_parity_packet_voice_hint_origin_absent_blocked();
    let row = packet.row(M5ConvenienceAffordance::VoiceHint).unwrap();
    assert_eq!(row.derived_status, AffordanceParityStatus::Red);
    assert!(!packet.report_clean);
    assert!(packet
        .blocking_findings
        .iter()
        .any(|finding| matches!(finding, AffordanceParityFinding::OriginExportBroken { .. })));
    assert!(row
        .conformance_causes
        .iter()
        .any(|cause| matches!(cause.trigger, M5DiscoverabilityDowngradeTrigger::ProofStale)));
    assert!(validate_m5_affordance_parity_packet(&packet).is_err());
}

#[test]
fn headless_parity_loss_blocks_the_ai_hint() {
    let packet =
        seeded_m5_discoverability_affordance_parity_packet_ai_hint_headless_parity_lost_blocked();
    let row = packet.row(M5ConvenienceAffordance::AiHint).unwrap();
    assert_eq!(row.derived_status, AffordanceParityStatus::Red);
    assert!(!row.headless_parity_preserved);
    assert!(!packet.report_clean);
    assert!(packet
        .blocking_findings
        .iter()
        .any(|finding| matches!(finding, AffordanceParityFinding::HeadlessParityLost { .. })));
    assert!(row.conformance_causes.iter().any(|cause| matches!(
        cause.trigger,
        M5DiscoverabilityDowngradeTrigger::ParitySurfaceDropped
    )));
    assert!(validate_m5_affordance_parity_packet(&packet).is_err());
}

#[test]
fn incomplete_record_field_set_blocks() {
    let mut packet = seeded_m5_discoverability_affordance_parity_packet();
    let row = packet
        .rows
        .iter_mut()
        .find(|row| row.affordance == M5ConvenienceAffordance::Button)
        .unwrap();
    row.certified_record_fields.pop();
    assert!(!row.record_fields_complete());
    assert_eq!(row.recompute_status(), AffordanceParityStatus::Red);
    let findings = row.compute_findings(&packet.generated_at);
    assert!(findings.iter().any(|finding| matches!(
        finding,
        AffordanceParityFinding::RecordFieldsIncomplete { .. }
    )));
}

#[test]
fn incomplete_reach_mode_set_blocks() {
    let mut packet = seeded_m5_discoverability_affordance_parity_packet();
    let row = packet
        .rows
        .iter_mut()
        .find(|row| row.affordance == M5ConvenienceAffordance::Button)
        .unwrap();
    row.certified_reach_modes.pop();
    assert!(!row.reach_modes_complete());
    assert_eq!(row.recompute_status(), AffordanceParityStatus::Red);
    let findings = row.compute_findings(&packet.generated_at);
    assert!(findings.iter().any(|finding| matches!(
        finding,
        AffordanceParityFinding::ReachModesIncomplete { .. }
    )));
}

#[test]
fn incomplete_consumer_surface_certification_blocks() {
    let mut packet = seeded_m5_discoverability_affordance_parity_packet();
    let row = packet
        .rows
        .iter_mut()
        .find(|row| row.affordance == M5ConvenienceAffordance::Button)
        .unwrap();
    row.evaluated_consumer_surfaces.pop();
    assert!(!row.consumer_surfaces_complete());
    assert_eq!(row.recompute_status(), AffordanceParityStatus::Red);
    let findings = row.compute_findings(&packet.generated_at);
    assert!(findings.iter().any(|finding| matches!(
        finding,
        AffordanceParityFinding::ConsumerSurfacesIncomplete { .. }
    )));
}

#[test]
fn dashboard_projects_every_row_and_counts() {
    let packet = seeded_m5_discoverability_affordance_parity_packet();
    let dashboard = packet.dashboard();
    assert_eq!(dashboard.rows.len(), packet.rows.len());
    assert_eq!(dashboard.green_row_count, packet.green_row_count);
    assert_eq!(dashboard.yellow_row_count, packet.yellow_row_count);
    assert_eq!(dashboard.red_row_count, packet.red_row_count);
    assert_eq!(dashboard.all_rows_publishable, packet.all_rows_publishable);
    assert_eq!(dashboard.source_packet_ref, packet.packet_id);
    assert!(!dashboard.command_automation_refs.is_empty());

    let companion = dashboard
        .rows
        .iter()
        .find(|row| row.affordance == M5ConvenienceAffordance::CompanionHandoff)
        .unwrap();
    assert_eq!(companion.status, AffordanceParityStatus::Yellow);
    assert!(companion.has_active_waiver);
    assert!(matches!(
        companion.authority_reach,
        AuthorityReachState::DisclosedReducedHoverFallback
    ));
}

#[test]
fn support_export_quotes_packet_matrix_and_waiver_refs() {
    let packet = seeded_m5_discoverability_affordance_parity_packet();
    let export = AffordanceParitySupportExport::from_packet(
        M5_AFFORDANCE_PARITY_SUPPORT_EXPORT_ID,
        packet.clone(),
    );
    assert!(export.case_ids.contains(&packet.packet_id));
    assert!(export.case_ids.contains(&packet.matrix_packet_ref));
    assert!(export.case_ids.contains(&packet.build_identity_ref));
    for row in &packet.rows {
        assert!(export
            .case_ids
            .contains(&row.affordance.as_str().to_owned()));
    }
    for waiver in &packet.active_waivers {
        assert!(export.case_ids.contains(&waiver.waiver_id));
    }
    assert_eq!(export.dashboard, packet.dashboard());
}

#[test]
fn markdown_and_csv_name_every_affordance() {
    let packet = seeded_m5_discoverability_affordance_parity_packet();
    let markdown = packet.render_markdown();
    let csv = packet.render_matrix_csv();
    for affordance in REQUIRED_AFFORDANCES {
        assert!(
            csv.contains(affordance.as_str()),
            "csv omits {}",
            affordance.as_str()
        );
    }
    assert!(markdown.contains("m5_discoverability_affordance_parity_fixtures"));
    assert!(markdown.contains("waiver:affordance-parity-reduced-hover:0001"));
}

#[test]
fn export_carries_no_forbidden_material() {
    let packet = seeded_m5_discoverability_affordance_parity_packet();
    let json = packet.export_safe_json().to_lowercase();
    assert!(!json.contains("api_key"));
    assert!(!json.contains("password"));
    assert!(!json.contains("bearer "));
    assert!(!json.contains("://"));
}

#[test]
fn expired_waiver_is_flagged() {
    let waiver = AffordanceParityWaiver {
        waiver_id: "waiver:expired:0001".to_owned(),
        affordance: M5ConvenienceAffordance::CompanionHandoff,
        reason: "test".to_owned(),
        owner_role: "owner".to_owned(),
        expires_at: "2026-01-01T00:00:00Z".to_owned(),
    };
    assert!(!waiver.is_active_at("2026-06-30T00:00:00Z"));
    assert!(waiver.is_active_at("2025-01-01T00:00:00Z"));
}
