//! Inline unit tests for the M5 command-explainer certification.

use super::*;

#[test]
fn seeded_packet_covers_every_surface_family_and_is_clean() {
    let packet = seeded_m5_command_explainers_packet();
    validate_m5_command_explainers_packet(&packet).expect("seeded packet must validate clean");

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
    let packet = seeded_m5_command_explainers_packet();
    assert_eq!(packet.green_row_count, 6);
    assert_eq!(packet.yellow_row_count, 4);
    assert_eq!(packet.red_row_count, 0);

    for family in [
        M5CommandSurfaceFamily::LeaderSequenceHelp,
        M5CommandSurfaceFamily::CommandBar,
        M5CommandSurfaceFamily::ContextMenu,
        M5CommandSurfaceFamily::ImportBridgeRow,
    ] {
        assert_eq!(
            packet.row(family).unwrap().derived_status,
            CommandExplainerStatus::Yellow,
            "{} should auto-narrow to yellow",
            family.as_str()
        );
    }
}

#[test]
fn every_row_status_is_the_derived_value() {
    let packet = seeded_m5_command_explainers_packet();
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
fn every_row_certifies_all_fields_classes_actions_and_modes() {
    let packet = seeded_m5_command_explainers_packet();
    for row in &packet.rows {
        assert!(
            row.leader_overlay_fields_complete(),
            "row {} does not narrate all six leader-overlay fields",
            row.surface_family.as_str()
        );
        assert!(
            row.blocker_classes_complete(),
            "row {} does not name all seven blocker classes",
            row.surface_family.as_str()
        );
        assert!(
            row.remediation_actions_complete(),
            "row {} does not offer all three remediation actions",
            row.surface_family.as_str()
        );
        assert!(
            row.reach_modes_complete(),
            "row {} does not stay reachable in all five reach modes",
            row.surface_family.as_str()
        );
        assert_eq!(
            row.certified_leader_overlay_fields.len(),
            REQUIRED_LEADER_OVERLAY_FIELDS.len()
        );
        assert_eq!(
            row.certified_blocker_classes.len(),
            REQUIRED_BLOCKER_CLASSES.len()
        );
        assert_eq!(
            row.certified_remediation_actions.len(),
            REQUIRED_REMEDIATION_ACTIONS.len()
        );
        assert_eq!(row.certified_reach_modes.len(), REQUIRED_REACH_MODES.len());
    }
}

#[test]
fn every_row_certifies_all_declared_consumer_surfaces() {
    let packet = seeded_m5_command_explainers_packet();
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
    let packet = seeded_m5_command_explainers_packet();
    for row in &packet.rows {
        let surface = matrix
            .surface_rows
            .iter()
            .find(|surface| surface.surface_family == row.surface_family)
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
        assert_eq!(row.required_labels, surface.required_labels);
        assert_eq!(row.feature_families, surface.feature_families);
        assert_eq!(row.covered_unavailable_reasons, surface.unavailable_reasons);
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
    let packet = seeded_m5_command_explainers_packet();
    for row in &packet.rows {
        if !matches!(row.derived_status, CommandExplainerStatus::Green) {
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
fn disclosed_reduced_sequence_overlay_carries_an_active_waiver() {
    let packet = seeded_m5_command_explainers_packet();
    let leader = packet
        .row(M5CommandSurfaceFamily::LeaderSequenceHelp)
        .unwrap();
    assert!(matches!(
        leader.leader_overlay,
        LeaderOverlayState::DisclosedReducedSequenceOverlay
    ));
    assert!(leader.requires_waiver());
    assert!(leader.has_active_waiver());
    assert_eq!(packet.active_waivers.len(), 1);
    assert!(packet.active_waivers[0].is_active_at(&packet.generated_at));
}

#[test]
fn disclosed_reduced_explainer_detail_keeps_command_bar_yellow_without_a_waiver() {
    let packet = seeded_m5_command_explainers_packet();
    let bar = packet.row(M5CommandSurfaceFamily::CommandBar).unwrap();
    assert!(matches!(
        bar.blocked_explainer,
        BlockedExplainerState::DisclosedReducedExplainerDetail
    ));
    assert_eq!(bar.derived_status, CommandExplainerStatus::Yellow);
    assert!(bar.leader_overlay.is_full());
    assert!(!bar.requires_waiver());
}

#[test]
fn silent_failure_blocks_the_menu_item() {
    let packet = seeded_m5_command_explainers_packet_menu_item_silent_failure_blocked();
    let row = packet.row(M5CommandSurfaceFamily::MenuItem).unwrap();
    assert_eq!(row.derived_status, CommandExplainerStatus::Red);
    assert!(!packet.report_clean);
    assert!(!packet.all_rows_publishable);
    assert!(packet.red_row_count >= 1);
    assert!(packet.blocking_findings.iter().any(|finding| matches!(
        finding,
        CommandExplainerFinding::BlockedExplainerBroken { .. }
    )));
    assert!(row.conformance_causes.iter().any(|cause| matches!(
        cause.trigger,
        M5DiscoverabilityDowngradeTrigger::DisabledReasonHidden
    )));
    assert!(validate_m5_command_explainers_packet(&packet).is_err());
}

#[test]
fn surface_local_prose_blocks_the_context_menu() {
    let packet = seeded_m5_command_explainers_packet_context_menu_surface_local_prose_blocked();
    let row = packet.row(M5CommandSurfaceFamily::ContextMenu).unwrap();
    assert_eq!(row.derived_status, CommandExplainerStatus::Red);
    assert!(!packet.report_clean);
    assert!(packet.blocking_findings.iter().any(|finding| matches!(
        finding,
        CommandExplainerFinding::RemediationParityBroken { .. }
    )));
    assert!(row.conformance_causes.iter().any(|cause| matches!(
        cause.trigger,
        M5DiscoverabilityDowngradeTrigger::AlternateLabelInvented
    )));
    assert!(validate_m5_command_explainers_packet(&packet).is_err());
}

#[test]
fn hidden_knowledge_blocks_the_leader_overlay() {
    let packet = seeded_m5_command_explainers_packet_leader_hidden_knowledge_blocked();
    let row = packet
        .row(M5CommandSurfaceFamily::LeaderSequenceHelp)
        .unwrap();
    assert_eq!(row.derived_status, CommandExplainerStatus::Red);
    assert!(!row.has_active_waiver());
    assert!(!packet.report_clean);
    assert!(packet
        .blocking_findings
        .iter()
        .any(|finding| matches!(finding, CommandExplainerFinding::LeaderOverlayBroken { .. })));
    assert!(row.conformance_causes.iter().any(|cause| matches!(
        cause.trigger,
        M5DiscoverabilityDowngradeTrigger::ConflictWinnerAmbiguous
    )));
    assert!(validate_m5_command_explainers_packet(&packet).is_err());
}

#[test]
fn absent_capture_blocks_the_import_bridge_row() {
    let packet = seeded_m5_command_explainers_packet_import_bridge_capture_absent_blocked();
    let row = packet.row(M5CommandSurfaceFamily::ImportBridgeRow).unwrap();
    assert_eq!(row.derived_status, CommandExplainerStatus::Red);
    assert!(!packet.report_clean);
    assert!(packet.blocking_findings.iter().any(|finding| matches!(
        finding,
        CommandExplainerFinding::ExplainerExportBroken { .. }
    )));
    assert!(row
        .conformance_causes
        .iter()
        .any(|cause| matches!(cause.trigger, M5DiscoverabilityDowngradeTrigger::ProofStale)));
    assert!(validate_m5_command_explainers_packet(&packet).is_err());
}

#[test]
fn headless_parity_loss_blocks_the_explainer() {
    let packet = seeded_m5_command_explainers_packet_explainer_headless_parity_lost_blocked();
    let row = packet
        .row(M5CommandSurfaceFamily::DisabledCommandExplainer)
        .unwrap();
    assert_eq!(row.derived_status, CommandExplainerStatus::Red);
    assert!(!row.headless_parity_preserved);
    assert!(!packet.report_clean);
    assert!(packet
        .blocking_findings
        .iter()
        .any(|finding| matches!(finding, CommandExplainerFinding::HeadlessParityLost { .. })));
    assert!(row.conformance_causes.iter().any(|cause| matches!(
        cause.trigger,
        M5DiscoverabilityDowngradeTrigger::ParitySurfaceDropped
    )));
    assert!(validate_m5_command_explainers_packet(&packet).is_err());
}

#[test]
fn incomplete_leader_overlay_field_set_blocks() {
    let mut packet = seeded_m5_command_explainers_packet();
    let row = packet
        .rows
        .iter_mut()
        .find(|row| row.surface_family == M5CommandSurfaceFamily::MenuItem)
        .unwrap();
    row.certified_leader_overlay_fields.pop();
    assert!(!row.leader_overlay_fields_complete());
    assert_eq!(row.recompute_status(), CommandExplainerStatus::Red);
    let findings = row.compute_findings(&packet.generated_at);
    assert!(findings.iter().any(|finding| matches!(
        finding,
        CommandExplainerFinding::LeaderOverlayFieldsIncomplete { .. }
    )));
}

#[test]
fn incomplete_blocker_class_set_blocks() {
    let mut packet = seeded_m5_command_explainers_packet();
    let row = packet
        .rows
        .iter_mut()
        .find(|row| row.surface_family == M5CommandSurfaceFamily::MenuItem)
        .unwrap();
    row.certified_blocker_classes.pop();
    assert!(!row.blocker_classes_complete());
    assert_eq!(row.recompute_status(), CommandExplainerStatus::Red);
    let findings = row.compute_findings(&packet.generated_at);
    assert!(findings.iter().any(|finding| matches!(
        finding,
        CommandExplainerFinding::BlockerClassesIncomplete { .. }
    )));
}

#[test]
fn incomplete_remediation_action_set_blocks() {
    let mut packet = seeded_m5_command_explainers_packet();
    let row = packet
        .rows
        .iter_mut()
        .find(|row| row.surface_family == M5CommandSurfaceFamily::MenuItem)
        .unwrap();
    row.certified_remediation_actions.pop();
    assert!(!row.remediation_actions_complete());
    assert_eq!(row.recompute_status(), CommandExplainerStatus::Red);
    let findings = row.compute_findings(&packet.generated_at);
    assert!(findings.iter().any(|finding| matches!(
        finding,
        CommandExplainerFinding::RemediationActionsIncomplete { .. }
    )));
}

#[test]
fn incomplete_reach_mode_set_blocks() {
    let mut packet = seeded_m5_command_explainers_packet();
    let row = packet
        .rows
        .iter_mut()
        .find(|row| row.surface_family == M5CommandSurfaceFamily::MenuItem)
        .unwrap();
    row.certified_reach_modes.pop();
    assert!(!row.reach_modes_complete());
    assert_eq!(row.recompute_status(), CommandExplainerStatus::Red);
    let findings = row.compute_findings(&packet.generated_at);
    assert!(findings.iter().any(|finding| matches!(
        finding,
        CommandExplainerFinding::ReachModesIncomplete { .. }
    )));
}

#[test]
fn incomplete_consumer_surface_certification_blocks() {
    let mut packet = seeded_m5_command_explainers_packet();
    let row = packet
        .rows
        .iter_mut()
        .find(|row| row.surface_family == M5CommandSurfaceFamily::MenuItem)
        .unwrap();
    row.evaluated_consumer_surfaces.pop();
    assert!(!row.consumer_surfaces_complete());
    assert_eq!(row.recompute_status(), CommandExplainerStatus::Red);
    let findings = row.compute_findings(&packet.generated_at);
    assert!(findings.iter().any(|finding| matches!(
        finding,
        CommandExplainerFinding::ConsumerSurfacesIncomplete { .. }
    )));
}

#[test]
fn dashboard_projects_every_row_and_counts() {
    let packet = seeded_m5_command_explainers_packet();
    let dashboard = packet.dashboard();
    assert_eq!(dashboard.rows.len(), packet.rows.len());
    assert_eq!(dashboard.green_row_count, packet.green_row_count);
    assert_eq!(dashboard.yellow_row_count, packet.yellow_row_count);
    assert_eq!(dashboard.red_row_count, packet.red_row_count);
    assert_eq!(dashboard.all_rows_publishable, packet.all_rows_publishable);
    assert_eq!(dashboard.source_packet_ref, packet.packet_id);
    assert!(!dashboard.command_automation_refs.is_empty());

    let leader = dashboard
        .rows
        .iter()
        .find(|row| row.surface_family == M5CommandSurfaceFamily::LeaderSequenceHelp)
        .unwrap();
    assert_eq!(leader.status, CommandExplainerStatus::Yellow);
    assert!(leader.has_active_waiver);
    assert!(matches!(
        leader.leader_overlay,
        LeaderOverlayState::DisclosedReducedSequenceOverlay
    ));
}

#[test]
fn support_export_quotes_packet_matrix_and_waiver_refs() {
    let packet = seeded_m5_command_explainers_packet();
    let export = CommandExplainerSupportExport::from_packet(
        M5_COMMAND_EXPLAINERS_SUPPORT_EXPORT_ID,
        packet.clone(),
    );
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
    let packet = seeded_m5_command_explainers_packet();
    let markdown = packet.render_markdown();
    let csv = packet.render_matrix_csv();
    for family in REQUIRED_SURFACE_FAMILIES {
        assert!(
            csv.contains(family.as_str()),
            "csv omits {}",
            family.as_str()
        );
    }
    assert!(markdown.contains("m5_command_explainers_fixtures"));
    assert!(markdown.contains("waiver:command-explainer-reduced-sequence:0001"));
}

#[test]
fn export_carries_no_forbidden_material() {
    let packet = seeded_m5_command_explainers_packet();
    let json = packet.export_safe_json().to_lowercase();
    assert!(!json.contains("api_key"));
    assert!(!json.contains("password"));
    assert!(!json.contains("bearer "));
    assert!(!json.contains("://"));
}

#[test]
fn expired_waiver_is_flagged() {
    let waiver = CommandExplainerWaiver {
        waiver_id: "waiver:expired:0001".to_owned(),
        surface_family: M5CommandSurfaceFamily::LeaderSequenceHelp,
        reason: "test".to_owned(),
        owner_role: "owner".to_owned(),
        expires_at: "2026-01-01T00:00:00Z".to_owned(),
    };
    assert!(!waiver.is_active_at("2026-06-30T00:00:00Z"));
    assert!(waiver.is_active_at("2025-01-01T00:00:00Z"));
}
