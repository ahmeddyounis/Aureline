//! Inline unit tests for the M5 advisory-component release proof.

use super::*;

#[test]
fn seeded_packet_covers_every_family_and_is_clean() {
    let packet = seeded_m5_advisory_component_release_proof_packet();
    validate_m5_advisory_component_release_proof_packet(&packet)
        .expect("seeded packet must validate clean");

    assert_eq!(packet.rows.len(), M5AdvisoryComponentFamily::ALL.len());
    for family in M5AdvisoryComponentFamily::ALL {
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
fn seeded_packet_has_three_green_and_three_yellow_rows() {
    let packet = seeded_m5_advisory_component_release_proof_packet();
    assert_eq!(packet.green_row_count, 3);
    assert_eq!(packet.yellow_row_count, 3);
    assert_eq!(packet.red_row_count, 0);

    for family in [
        M5AdvisoryComponentFamily::AffectedInstallPanel,
        M5AdvisoryComponentFamily::DisclosureBlock,
        M5AdvisoryComponentFamily::AdvisoryActivityRow,
    ] {
        assert_eq!(
            packet.row(family).unwrap().derived_status,
            AdvisoryReleaseProofStatus::Yellow,
            "{} should auto-narrow to yellow",
            family.as_str()
        );
    }
    for family in [
        M5AdvisoryComponentFamily::AdvisoryCard,
        M5AdvisoryComponentFamily::EmergencyNotice,
        M5AdvisoryComponentFamily::NativeNotificationHandoff,
    ] {
        assert_eq!(
            packet.row(family).unwrap().derived_status,
            AdvisoryReleaseProofStatus::Green,
            "{} should stay green",
            family.as_str()
        );
    }
}

#[test]
fn every_row_status_is_the_derived_value() {
    let packet = seeded_m5_advisory_component_release_proof_packet();
    for row in &packet.rows {
        assert_eq!(
            row.derived_status,
            row.recompute_status(),
            "row {} declares a stale status",
            row.component_family.as_str()
        );
        assert_eq!(row.certification_causes, row.recompute_causes());
    }
}

#[test]
fn every_row_certifies_every_surface_and_declares_pillars_and_invariant() {
    let packet = seeded_m5_advisory_component_release_proof_packet();
    for row in &packet.rows {
        assert!(
            row.surface_families_complete(),
            "row {} does not certify every claimed M5 surface family",
            row.component_family.as_str()
        );
        assert!(
            row.truth_pillars_declared(),
            "row {} does not declare its truth pillars",
            row.component_family.as_str()
        );
        assert!(
            row.never_hides_advisory_truth_off_channel,
            "row {} hides advisory truth off a claimed channel",
            row.component_family.as_str()
        );
        assert_eq!(
            row.certified_surface_families,
            M5ShellSurfaceFamily::ALL.to_vec()
        );
    }
}

#[test]
fn truth_pillars_cover_the_whole_track_invariant() {
    let packet = seeded_m5_advisory_component_release_proof_packet();
    let mut covered: Vec<String> = M5AdvisoryTruthPillar::ALL
        .iter()
        .map(|p| p.as_str().to_owned())
        .collect();
    covered.sort();
    assert_eq!(packet.covered_truth_pillars, covered);
    assert_eq!(
        packet.required_truth_pillars,
        M5AdvisoryTruthPillar::ALL.to_vec()
    );
}

#[test]
fn each_family_pulls_its_own_matrix_bindings() {
    let packet = seeded_m5_advisory_component_release_proof_packet();

    let card = packet.row(M5AdvisoryComponentFamily::AdvisoryCard).unwrap();
    assert!(!card.certified_anatomy_fields.is_empty());
    assert!(!card.certified_action_states.is_empty());
    assert_eq!(card.shell_zone_slot, M5ShellZoneSlot::MainWorkspace);
    assert_eq!(
        card.certified_truth_pillars,
        vec![M5AdvisoryTruthPillar::AffectedScopeExposureAndContinuity]
    );

    let emergency = packet
        .row(M5AdvisoryComponentFamily::EmergencyNotice)
        .unwrap();
    assert!(!emergency.certified_dismissal_states.is_empty());
    assert_eq!(emergency.shell_zone_slot, M5ShellZoneSlot::TitleContextBar);
    assert_eq!(
        emergency.certified_truth_pillars,
        vec![M5AdvisoryTruthPillar::EmergencyBlastRadiusAndForcedDisable]
    );

    let install = packet
        .row(M5AdvisoryComponentFamily::AffectedInstallPanel)
        .unwrap();
    assert!(!install.certified_continuity_claims.is_empty());
    assert!(!install.certified_delivery_profiles.is_empty());
    assert!(!install.certified_freshness_states.is_empty());
    assert_eq!(install.shell_zone_slot, M5ShellZoneSlot::RightInspector);

    let disclosure = packet
        .row(M5AdvisoryComponentFamily::DisclosureBlock)
        .unwrap();
    assert!(!disclosure.certified_disclosure_fields.is_empty());
    assert_eq!(
        disclosure.certified_truth_pillars,
        vec![M5AdvisoryTruthPillar::DisclosureProvenanceAndHistory]
    );

    let activity = packet
        .row(M5AdvisoryComponentFamily::AdvisoryActivityRow)
        .unwrap();
    assert!(!activity.certified_export_fields.is_empty());

    let native = packet
        .row(M5AdvisoryComponentFamily::NativeNotificationHandoff)
        .unwrap();
    assert!(!native.certified_notification_behaviors.is_empty());

    for row in &packet.rows {
        assert!(!row.certified_severity_classes.is_empty());
        assert!(!row.certified_projection_surfaces.is_empty());
        assert!(!row.accessibility_routes.is_empty());
        assert!(!row.required_labels.is_empty());
        assert!(!row.consumer_surfaces.is_empty());
        assert!(!row.applicable_downgrade_triggers.is_empty());
        assert!(row
            .applicable_downgrade_triggers
            .contains(&M5AdvisoryDowngradeTrigger::ProofStale));
        assert!(row
            .applicable_downgrade_triggers
            .contains(&primary_contract_trigger(row.component_family)));
    }
}

#[test]
fn narrowed_rows_disclose_a_reason() {
    let packet = seeded_m5_advisory_component_release_proof_packet();
    for row in &packet.rows {
        if !matches!(row.derived_status, AdvisoryReleaseProofStatus::Green) {
            assert!(
                row.narrowing_reason
                    .as_deref()
                    .map(|reason| !reason.trim().is_empty())
                    .unwrap_or(false),
                "narrowed row {} hides its reason",
                row.component_family.as_str()
            );
        }
    }
}

#[test]
fn reduced_channel_projection_carries_an_active_waiver() {
    let packet = seeded_m5_advisory_component_release_proof_packet();
    let install = packet
        .row(M5AdvisoryComponentFamily::AffectedInstallPanel)
        .unwrap();
    assert!(matches!(
        install.cross_channel_parity,
        CrossChannelParityState::DisclosedReducedChannelProjection
    ));
    assert!(install.requires_waiver());
    assert!(install.has_active_waiver());
    assert_eq!(packet.active_waivers.len(), 1);
    assert!(packet.active_waivers[0].is_active_at(&packet.generated_at));
    assert!(install
        .certification_causes
        .iter()
        .any(|cause| cause.disclosed
            && matches!(
                cause.trigger,
                M5AdvisoryDowngradeTrigger::AffectedScopeHidden
            )));
}

#[test]
fn partial_capture_narrows_but_does_not_block() {
    let packet = seeded_m5_advisory_component_release_proof_packet();
    let disclosure = packet
        .row(M5AdvisoryComponentFamily::DisclosureBlock)
        .unwrap();
    assert!(matches!(
        disclosure.support_export_proof,
        SupportExportProofState::DisclosedPartialCapture
    ));
    assert_eq!(
        disclosure.derived_status,
        AdvisoryReleaseProofStatus::Yellow
    );
    assert!(disclosure
        .certification_causes
        .iter()
        .any(|cause| cause.disclosed
            && matches!(cause.trigger, M5AdvisoryDowngradeTrigger::ProofStale)));
}

#[test]
fn collapsed_advisory_truth_blocks_the_advisory_card_family() {
    let packet =
        seeded_m5_advisory_component_release_proof_packet_advisory_card_contract_truth_collapsed_blocked();
    let row = packet.row(M5AdvisoryComponentFamily::AdvisoryCard).unwrap();
    assert_eq!(row.derived_status, AdvisoryReleaseProofStatus::Red);
    assert!(!packet.report_clean);
    assert!(!packet.all_rows_publishable);
    assert!(packet.red_row_count >= 1);
    assert!(packet.blocking_findings.iter().any(|finding| matches!(
        finding,
        AdvisoryReleaseProofFinding::AdvisoryTruthCollapsedOrDrifted { .. }
    )));
    assert!(row.certification_causes.iter().any(|cause| !cause.disclosed
        && matches!(
            cause.trigger,
            M5AdvisoryDowngradeTrigger::AffectedScopeHidden
        )));
    assert!(validate_m5_advisory_component_release_proof_packet(&packet).is_err());
}

#[test]
fn diverged_row_grammar_blocks_the_affected_install_family() {
    let packet =
        seeded_m5_advisory_component_release_proof_packet_affected_install_channel_diverged_blocked(
        );
    let row = packet
        .row(M5AdvisoryComponentFamily::AffectedInstallPanel)
        .unwrap();
    assert_eq!(row.derived_status, AdvisoryReleaseProofStatus::Red);
    assert!(!packet.report_clean);
    assert!(packet.blocking_findings.iter().any(|finding| matches!(
        finding,
        AdvisoryReleaseProofFinding::ChannelGrammarDivergedOffPrimaryChannel { .. }
    )));
    assert!(validate_m5_advisory_component_release_proof_packet(&packet).is_err());
}

#[test]
fn absent_capture_blocks_the_disclosure_block_family() {
    let packet =
        seeded_m5_advisory_component_release_proof_packet_disclosure_block_capture_absent_blocked();
    let row = packet
        .row(M5AdvisoryComponentFamily::DisclosureBlock)
        .unwrap();
    assert_eq!(row.derived_status, AdvisoryReleaseProofStatus::Red);
    assert!(!packet.report_clean);
    assert!(packet.blocking_findings.iter().any(|finding| matches!(
        finding,
        AdvisoryReleaseProofFinding::AdvisoryTruthAbsentFromCapture { .. }
    )));
    assert!(validate_m5_advisory_component_release_proof_packet(&packet).is_err());
}

#[test]
fn stale_proof_blocks_the_advisory_activity_row_family() {
    let packet =
        seeded_m5_advisory_component_release_proof_packet_advisory_activity_row_proof_stale_blocked(
        );
    let row = packet
        .row(M5AdvisoryComponentFamily::AdvisoryActivityRow)
        .unwrap();
    assert_eq!(row.derived_status, AdvisoryReleaseProofStatus::Red);
    assert!(!packet.report_clean);
    assert!(packet.blocking_findings.iter().any(|finding| matches!(
        finding,
        AdvisoryReleaseProofFinding::ExportedProofStaleOrDivergent { .. }
    )));
    assert!(validate_m5_advisory_component_release_proof_packet(&packet).is_err());
}

#[test]
fn hidden_advisory_truth_blocks_the_emergency_notice_family() {
    let packet =
        seeded_m5_advisory_component_release_proof_packet_emergency_notice_advisory_truth_dropped_blocked();
    let row = packet
        .row(M5AdvisoryComponentFamily::EmergencyNotice)
        .unwrap();
    assert_eq!(row.derived_status, AdvisoryReleaseProofStatus::Red);
    assert!(!packet.report_clean);
    assert!(packet.blocking_findings.iter().any(|finding| matches!(
        finding,
        AdvisoryReleaseProofFinding::AdvisoryTruthHiddenOffChannel { .. }
    )));
    assert!(row.certification_causes.iter().any(|cause| !cause.disclosed
        && matches!(
            cause.trigger,
            M5AdvisoryDowngradeTrigger::AffectedScopeHidden
        )));
    assert!(validate_m5_advisory_component_release_proof_packet(&packet).is_err());
}

#[test]
fn dashboard_projects_every_row_and_counts() {
    let packet = seeded_m5_advisory_component_release_proof_packet();
    let dashboard = packet.dashboard();
    assert_eq!(dashboard.rows.len(), packet.rows.len());
    assert_eq!(dashboard.green_row_count, packet.green_row_count);
    assert_eq!(dashboard.yellow_row_count, packet.yellow_row_count);
    assert_eq!(dashboard.red_row_count, packet.red_row_count);
    assert_eq!(dashboard.all_rows_publishable, packet.all_rows_publishable);
    assert_eq!(dashboard.source_packet_ref, packet.packet_id);
    assert!(!dashboard.claim_automation_refs.is_empty());

    let install = dashboard
        .rows
        .iter()
        .find(|row| row.component_family == M5AdvisoryComponentFamily::AffectedInstallPanel)
        .unwrap();
    assert_eq!(install.status, AdvisoryReleaseProofStatus::Yellow);
    assert!(install.has_active_waiver);
    assert!(install.cause_tokens.contains(
        &M5AdvisoryDowngradeTrigger::AffectedScopeHidden
            .as_str()
            .to_owned()
    ));
}

#[test]
fn support_export_quotes_packet_matrix_and_waiver_refs() {
    let packet = seeded_m5_advisory_component_release_proof_packet();
    let export = AdvisoryReleaseProofSupportExport::from_packet(
        M5_ADVISORY_RELEASE_PROOF_SUPPORT_EXPORT_ID,
        packet.clone(),
    );
    assert!(export.case_ids.contains(&packet.packet_id));
    assert!(export.case_ids.contains(&packet.matrix_packet_ref));
    assert!(export.case_ids.contains(&packet.build_identity_ref));
    for row in &packet.rows {
        assert!(export
            .case_ids
            .contains(&row.component_family.as_str().to_owned()));
    }
    for waiver in &packet.active_waivers {
        assert!(export.case_ids.contains(&waiver.waiver_id));
    }
    assert_eq!(export.dashboard, packet.dashboard());
}

#[test]
fn markdown_and_csv_name_every_family() {
    let packet = seeded_m5_advisory_component_release_proof_packet();
    let markdown = packet.render_markdown();
    let csv = packet.render_matrix_csv();
    for family in M5AdvisoryComponentFamily::ALL {
        assert!(
            markdown.contains(component_family_label(family)),
            "markdown omits {}",
            family.as_str()
        );
        assert!(
            csv.contains(family.as_str()),
            "csv omits {}",
            family.as_str()
        );
    }
    assert!(markdown.contains("m5_advisory_component_release_proof_fixtures"));
    assert!(markdown.contains("waiver:reduced-channel-projection:0001"));
}

#[test]
fn export_carries_no_forbidden_material() {
    let packet = seeded_m5_advisory_component_release_proof_packet();
    let json = packet.export_safe_json().to_lowercase();
    assert!(!json.contains("api_key"));
    assert!(!json.contains("password"));
    assert!(!json.contains("secret"));
    assert!(!json.contains("bearer "));
    assert!(!json.contains("://"));
}

#[test]
fn expired_waiver_is_flagged() {
    let waiver = AdvisoryReleaseProofWaiver {
        waiver_id: "waiver:expired:0001".to_owned(),
        component_family: M5AdvisoryComponentFamily::AffectedInstallPanel,
        reason: "test".to_owned(),
        owner_role: "owner".to_owned(),
        expires_at: "2026-01-01T00:00:00Z".to_owned(),
    };
    assert!(!waiver.is_active_at("2026-07-03T00:00:00Z"));
    assert!(waiver.is_active_at("2025-01-01T00:00:00Z"));
}
