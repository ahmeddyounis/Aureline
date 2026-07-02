//! Inline unit tests for the M5 discoverability-access-parity certification.

use super::*;

#[test]
fn seeded_packet_covers_every_surface_family_and_is_clean() {
    let packet = seeded_m5_discoverability_access_parity_packet();
    validate_m5_access_parity_packet(&packet).expect("seeded packet must validate clean");

    assert_eq!(packet.rows.len(), REQUIRED_SURFACE_FAMILIES.len());
    for family in REQUIRED_SURFACE_FAMILIES {
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
    let packet = seeded_m5_discoverability_access_parity_packet();
    assert_eq!(packet.green_row_count, 6);
    assert_eq!(packet.yellow_row_count, 4);
    assert_eq!(packet.red_row_count, 0);

    for family in [
        M5CommandSurfaceFamily::CommandBar,
        M5CommandSurfaceFamily::ImportBridgeRow,
        M5CommandSurfaceFamily::LeaderSequenceHelp,
        M5CommandSurfaceFamily::CommandDocumentationSurface,
    ] {
        assert_eq!(
            packet.row(family).unwrap().derived_status,
            AccessParityStatus::Yellow,
            "{} should auto-narrow to yellow",
            family.as_str()
        );
    }
}

#[test]
fn every_row_status_is_the_derived_value() {
    let packet = seeded_m5_discoverability_access_parity_packet();
    for row in &packet.rows {
        assert_eq!(
            row.derived_status,
            row.recompute_status(),
            "row {} declares a stale status",
            row.surface_family.as_str()
        );
        assert_eq!(row.conformance_causes, row.recompute_causes());
    }
}

#[test]
fn every_row_certifies_all_channels_fields_and_profiles() {
    let packet = seeded_m5_discoverability_access_parity_packet();
    for row in &packet.rows {
        assert!(
            row.reach_channels_complete(),
            "row {} does not certify all five non-pointer reach channels",
            row.surface_family.as_str()
        );
        assert!(
            row.incident_fields_complete(),
            "row {} does not capture all five accessibility-incident fields",
            row.surface_family.as_str()
        );
        assert!(
            row.access_profiles_complete(),
            "row {} does not stay stable across all four desktop access profiles",
            row.surface_family.as_str()
        );
        assert_eq!(
            row.certified_reach_channels.len(),
            REQUIRED_REACH_CHANNELS.len()
        );
        assert_eq!(
            row.certified_incident_fields.len(),
            REQUIRED_INCIDENT_FIELDS.len()
        );
        assert_eq!(
            row.certified_access_profiles.len(),
            REQUIRED_ACCESS_PROFILES.len()
        );
    }
}

#[test]
fn every_row_certifies_all_declared_consumer_surfaces() {
    let packet = seeded_m5_discoverability_access_parity_packet();
    for row in &packet.rows {
        assert!(
            row.consumer_surfaces_complete(),
            "row {} does not certify all declared consumer surfaces",
            row.surface_family.as_str()
        );
        assert!(row.headless_parity_preserved);
    }
}

#[test]
fn every_binding_is_pulled_from_the_frozen_matrix() {
    use crate::freeze_the_m5_menu_keybinding_resolver_and_command_documentation_matrix::seeded_m5_discoverability_matrix;

    let matrix = seeded_m5_discoverability_matrix();
    let packet = seeded_m5_discoverability_access_parity_packet();
    for row in &packet.rows {
        let surface = matrix
            .surface_rows
            .iter()
            .find(|surface| surface.surface_family == row.surface_family)
            .expect("surface family is frozen by the matrix");
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
fn narrowed_rows_disclose_a_reason() {
    let packet = seeded_m5_discoverability_access_parity_packet();
    for row in &packet.rows {
        if !matches!(row.derived_status, AccessParityStatus::Green) {
            assert!(
                row.narrowing_reason
                    .as_deref()
                    .map(|reason| !reason.trim().is_empty())
                    .unwrap_or(false),
                "narrowed row {} hides its reason",
                row.surface_family.as_str()
            );
        }
    }
}

#[test]
fn disclosed_reduced_touch_fallback_carries_an_active_waiver() {
    let packet = seeded_m5_discoverability_access_parity_packet();
    let bar = packet.row(M5CommandSurfaceFamily::CommandBar).unwrap();
    assert!(matches!(
        bar.non_pointer_reach,
        NonPointerReachState::DisclosedReducedTouchFallback
    ));
    assert!(bar.requires_waiver());
    assert!(bar.has_active_waiver());
    assert_eq!(packet.active_waivers.len(), 1);
    assert!(packet.active_waivers[0].is_active_at(&packet.generated_at));
}

#[test]
fn disclosed_partial_capture_keeps_import_bridge_yellow_without_a_waiver() {
    let packet = seeded_m5_discoverability_access_parity_packet();
    let row = packet.row(M5CommandSurfaceFamily::ImportBridgeRow).unwrap();
    assert!(matches!(
        row.support_export_evidence,
        SupportExportEvidenceState::DisclosedPartialCapture
    ));
    assert_eq!(row.derived_status, AccessParityStatus::Yellow);
    assert!(row.non_pointer_reach.is_full());
    assert!(!row.requires_waiver());
}

#[test]
fn hover_only_blocks_the_menu_item() {
    let packet = seeded_m5_discoverability_access_parity_packet_menu_item_hover_only_blocked();
    let row = packet.row(M5CommandSurfaceFamily::MenuItem).unwrap();
    assert_eq!(row.derived_status, AccessParityStatus::Red);
    assert!(!packet.report_clean);
    assert!(!packet.all_rows_publishable);
    assert!(packet.red_row_count >= 1);
    assert!(packet
        .blocking_findings
        .iter()
        .any(|finding| matches!(finding, AccessParityFinding::NonPointerReachBroken { .. })));
    assert!(row.conformance_causes.iter().any(|cause| matches!(
        cause.trigger,
        M5DiscoverabilityDowngradeTrigger::ParitySurfaceDropped
    )));
    assert!(validate_m5_access_parity_packet(&packet).is_err());
}

#[test]
fn absent_evidence_blocks_the_context_menu() {
    let packet =
        seeded_m5_discoverability_access_parity_packet_context_menu_evidence_absent_blocked();
    let row = packet.row(M5CommandSurfaceFamily::ContextMenu).unwrap();
    assert_eq!(row.derived_status, AccessParityStatus::Red);
    assert!(!packet.report_clean);
    assert!(packet.blocking_findings.iter().any(|finding| matches!(
        finding,
        AccessParityFinding::SupportExportEvidenceBroken { .. }
    )));
    assert!(row
        .conformance_causes
        .iter()
        .any(|cause| matches!(cause.trigger, M5DiscoverabilityDowngradeTrigger::ProofStale)));
    assert!(validate_m5_access_parity_packet(&packet).is_err());
}

#[test]
fn unstable_profile_blocks_the_resolver_layer() {
    let packet = seeded_m5_discoverability_access_parity_packet_resolver_profile_unstable_blocked();
    let row = packet
        .row(M5CommandSurfaceFamily::KeybindingResolverLayer)
        .unwrap();
    assert_eq!(row.derived_status, AccessParityStatus::Red);
    assert!(!packet.report_clean);
    assert!(packet
        .blocking_findings
        .iter()
        .any(|finding| matches!(finding, AccessParityFinding::ProfileStabilityBroken { .. })));
    assert!(row.conformance_causes.iter().any(|cause| matches!(
        cause.trigger,
        M5DiscoverabilityDowngradeTrigger::ParitySurfaceDropped
    )));
    assert!(validate_m5_access_parity_packet(&packet).is_err());
}

#[test]
fn stale_anchor_blocks_the_documentation_surface() {
    let packet = seeded_m5_discoverability_access_parity_packet_doc_stale_anchor_blocked();
    let row = packet
        .row(M5CommandSurfaceFamily::CommandDocumentationSurface)
        .unwrap();
    assert_eq!(row.derived_status, AccessParityStatus::Red);
    assert!(!packet.report_clean);
    assert!(packet
        .blocking_findings
        .iter()
        .any(|finding| matches!(finding, AccessParityFinding::ReleaseEvidenceBroken { .. })));
    assert!(row
        .conformance_causes
        .iter()
        .any(|cause| matches!(cause.trigger, M5DiscoverabilityDowngradeTrigger::ProofStale)));
    assert!(validate_m5_access_parity_packet(&packet).is_err());
}

#[test]
fn headless_parity_loss_blocks_the_explainer() {
    let packet =
        seeded_m5_discoverability_access_parity_packet_explainer_headless_parity_lost_blocked();
    let row = packet
        .row(M5CommandSurfaceFamily::DisabledCommandExplainer)
        .unwrap();
    assert_eq!(row.derived_status, AccessParityStatus::Red);
    assert!(!row.headless_parity_preserved);
    assert!(!packet.report_clean);
    assert!(packet
        .blocking_findings
        .iter()
        .any(|finding| matches!(finding, AccessParityFinding::HeadlessParityLost { .. })));
    assert!(row.conformance_causes.iter().any(|cause| matches!(
        cause.trigger,
        M5DiscoverabilityDowngradeTrigger::ParitySurfaceDropped
    )));
    assert!(validate_m5_access_parity_packet(&packet).is_err());
}

#[test]
fn incomplete_reach_channel_set_blocks() {
    let mut packet = seeded_m5_discoverability_access_parity_packet();
    let row = packet
        .rows
        .iter_mut()
        .find(|row| row.surface_family == M5CommandSurfaceFamily::MenuItem)
        .unwrap();
    row.certified_reach_channels.pop();
    assert!(!row.reach_channels_complete());
    assert_eq!(row.recompute_status(), AccessParityStatus::Red);
    let findings = row.compute_findings(&packet.generated_at);
    assert!(findings
        .iter()
        .any(|finding| matches!(finding, AccessParityFinding::ReachChannelsIncomplete { .. })));
}

#[test]
fn incomplete_incident_field_set_blocks() {
    let mut packet = seeded_m5_discoverability_access_parity_packet();
    let row = packet
        .rows
        .iter_mut()
        .find(|row| row.surface_family == M5CommandSurfaceFamily::MenuItem)
        .unwrap();
    row.certified_incident_fields.pop();
    assert!(!row.incident_fields_complete());
    assert_eq!(row.recompute_status(), AccessParityStatus::Red);
    let findings = row.compute_findings(&packet.generated_at);
    assert!(findings.iter().any(|finding| matches!(
        finding,
        AccessParityFinding::IncidentFieldsIncomplete { .. }
    )));
}

#[test]
fn incomplete_access_profile_set_blocks() {
    let mut packet = seeded_m5_discoverability_access_parity_packet();
    let row = packet
        .rows
        .iter_mut()
        .find(|row| row.surface_family == M5CommandSurfaceFamily::MenuItem)
        .unwrap();
    row.certified_access_profiles.pop();
    assert!(!row.access_profiles_complete());
    assert_eq!(row.recompute_status(), AccessParityStatus::Red);
    let findings = row.compute_findings(&packet.generated_at);
    assert!(findings.iter().any(|finding| matches!(
        finding,
        AccessParityFinding::AccessProfilesIncomplete { .. }
    )));
}

#[test]
fn incomplete_consumer_surface_certification_blocks() {
    let mut packet = seeded_m5_discoverability_access_parity_packet();
    let row = packet
        .rows
        .iter_mut()
        .find(|row| row.surface_family == M5CommandSurfaceFamily::MenuItem)
        .unwrap();
    row.evaluated_consumer_surfaces.pop();
    assert!(!row.consumer_surfaces_complete());
    assert_eq!(row.recompute_status(), AccessParityStatus::Red);
    let findings = row.compute_findings(&packet.generated_at);
    assert!(findings.iter().any(|finding| matches!(
        finding,
        AccessParityFinding::ConsumerSurfacesIncomplete { .. }
    )));
}

#[test]
fn dashboard_projects_every_row_and_counts() {
    let packet = seeded_m5_discoverability_access_parity_packet();
    let dashboard = packet.dashboard();
    assert_eq!(dashboard.rows.len(), packet.rows.len());
    assert_eq!(dashboard.green_row_count, packet.green_row_count);
    assert_eq!(dashboard.yellow_row_count, packet.yellow_row_count);
    assert_eq!(dashboard.red_row_count, packet.red_row_count);
    assert_eq!(dashboard.all_rows_publishable, packet.all_rows_publishable);
    assert_eq!(dashboard.source_packet_ref, packet.packet_id);
    assert!(!dashboard.command_automation_refs.is_empty());

    let bar = dashboard
        .rows
        .iter()
        .find(|row| row.surface_family == M5CommandSurfaceFamily::CommandBar)
        .unwrap();
    assert_eq!(bar.status, AccessParityStatus::Yellow);
    assert!(bar.has_active_waiver);
    assert!(matches!(
        bar.non_pointer_reach,
        NonPointerReachState::DisclosedReducedTouchFallback
    ));
}

#[test]
fn support_export_quotes_packet_matrix_and_waiver_refs() {
    let packet = seeded_m5_discoverability_access_parity_packet();
    let export =
        AccessParitySupportExport::from_packet(M5_ACCESS_PARITY_SUPPORT_EXPORT_ID, packet.clone());
    assert!(export.case_ids.contains(&packet.packet_id));
    assert!(export.case_ids.contains(&packet.matrix_packet_ref));
    assert!(export.case_ids.contains(&packet.build_identity_ref));
    for row in &packet.rows {
        assert!(export
            .case_ids
            .contains(&row.surface_family.as_str().to_owned()));
    }
    for waiver in &packet.active_waivers {
        assert!(export.case_ids.contains(&waiver.waiver_id));
    }
    assert_eq!(export.dashboard, packet.dashboard());
}

#[test]
fn markdown_and_csv_name_every_surface_family() {
    let packet = seeded_m5_discoverability_access_parity_packet();
    let markdown = packet.render_markdown();
    let csv = packet.render_matrix_csv();
    for family in REQUIRED_SURFACE_FAMILIES {
        assert!(
            csv.contains(family.as_str()),
            "csv omits {}",
            family.as_str()
        );
    }
    assert!(markdown.contains("m5_discoverability_access_parity_fixtures"));
    assert!(markdown.contains("waiver:access-parity-reduced-touch:0001"));
}

#[test]
fn export_carries_no_forbidden_material() {
    let packet = seeded_m5_discoverability_access_parity_packet();
    let json = packet.export_safe_json().to_lowercase();
    assert!(!json.contains("api_key"));
    assert!(!json.contains("password"));
    assert!(!json.contains("bearer "));
    assert!(!json.contains("://"));
}

#[test]
fn expired_waiver_is_flagged() {
    let waiver = AccessParityWaiver {
        waiver_id: "waiver:expired:0001".to_owned(),
        surface_family: M5CommandSurfaceFamily::CommandBar,
        reason: "test".to_owned(),
        owner_role: "owner".to_owned(),
        expires_at: "2026-01-01T00:00:00Z".to_owned(),
    };
    assert!(!waiver.is_active_at("2026-06-30T00:00:00Z"));
    assert!(waiver.is_active_at("2025-01-01T00:00:00Z"));
}
