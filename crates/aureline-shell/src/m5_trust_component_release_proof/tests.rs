//! Inline unit tests for the M5 trust-component release proof.

use super::*;

#[test]
fn seeded_packet_covers_every_family_and_is_clean() {
    let packet = seeded_m5_trust_component_release_proof_packet();
    validate_m5_trust_component_release_proof_packet(&packet)
        .expect("seeded packet must validate clean");

    assert_eq!(packet.rows.len(), M5TrustComponentFamily::ALL.len());
    for family in M5TrustComponentFamily::ALL {
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
    let packet = seeded_m5_trust_component_release_proof_packet();
    assert_eq!(packet.green_row_count, 3);
    assert_eq!(packet.yellow_row_count, 3);
    assert_eq!(packet.red_row_count, 0);

    for family in [
        M5TrustComponentFamily::TimelineGroup,
        M5TrustComponentFamily::NarrativeSummaryCard,
        M5TrustComponentFamily::ChronologyExportPreview,
    ] {
        assert_eq!(
            packet.row(family).unwrap().derived_status,
            TrustReleaseProofStatus::Yellow,
            "{} should auto-narrow to yellow",
            family.as_str()
        );
    }
    for family in [
        M5TrustComponentFamily::SettingsRow,
        M5TrustComponentFamily::CapabilitySheet,
        M5TrustComponentFamily::EventHistoryRow,
    ] {
        assert_eq!(
            packet.row(family).unwrap().derived_status,
            TrustReleaseProofStatus::Green,
            "{} should stay green",
            family.as_str()
        );
    }
}

#[test]
fn every_row_status_is_the_derived_value() {
    let packet = seeded_m5_trust_component_release_proof_packet();
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
    let packet = seeded_m5_trust_component_release_proof_packet();
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
            row.never_drops_audit_or_support_truth,
            "row {} drops audit / support truth",
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
    let packet = seeded_m5_trust_component_release_proof_packet();
    let mut covered: Vec<String> = M5TrustComponentTruthPillar::ALL
        .iter()
        .map(|p| p.as_str().to_owned())
        .collect();
    covered.sort();
    assert_eq!(packet.covered_truth_pillars, covered);
    assert_eq!(
        packet.required_truth_pillars,
        M5TrustComponentTruthPillar::ALL.to_vec()
    );
}

#[test]
fn each_family_pulls_its_own_matrix_bindings() {
    let packet = seeded_m5_trust_component_release_proof_packet();

    let settings = packet.row(M5TrustComponentFamily::SettingsRow).unwrap();
    assert!(!settings.certified_settings_row_states.is_empty());
    assert!(!settings.certified_source_pills.is_empty());
    assert_eq!(settings.shell_zone_slot, M5ShellZoneSlot::MainWorkspace);
    assert_eq!(
        settings.certified_truth_pillars,
        vec![M5TrustComponentTruthPillar::EffectiveValueSourceAndLock]
    );

    let capability = packet.row(M5TrustComponentFamily::CapabilitySheet).unwrap();
    assert!(!capability.certified_consequence_classes.is_empty());
    assert!(!capability.certified_capability_scope_states.is_empty());
    assert_eq!(
        capability.shell_zone_slot,
        M5ShellZoneSlot::TransientOverlay
    );
    assert_eq!(
        capability.certified_truth_pillars,
        vec![M5TrustComponentTruthPillar::ConsequenceScopeAndReconsent]
    );

    let events = packet.row(M5TrustComponentFamily::EventHistoryRow).unwrap();
    assert!(!events.certified_chronology_verbs.is_empty());
    assert!(!events.certified_provenance_badges.is_empty());
    assert_eq!(
        events.certified_truth_pillars,
        vec![M5TrustComponentTruthPillar::ChronologyVerbProvenanceAndExport]
    );

    let export = packet
        .row(M5TrustComponentFamily::ChronologyExportPreview)
        .unwrap();
    assert!(!export.certified_chronology_export_fields.is_empty());

    for row in &packet.rows {
        assert!(!row.accessibility_routes.is_empty());
        assert!(!row.required_labels.is_empty());
        assert!(!row.consumer_surfaces.is_empty());
        assert!(!row.applicable_downgrade_triggers.is_empty());
        assert!(row
            .applicable_downgrade_triggers
            .contains(&M5TrustComponentDowngradeTrigger::ProofStale));
        assert!(row
            .applicable_downgrade_triggers
            .contains(&primary_contract_trigger(row.component_family)));
    }
}

#[test]
fn narrowed_rows_disclose_a_reason() {
    let packet = seeded_m5_trust_component_release_proof_packet();
    for row in &packet.rows {
        if !matches!(row.derived_status, TrustReleaseProofStatus::Green) {
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
fn reduced_surface_projection_carries_an_active_waiver() {
    let packet = seeded_m5_trust_component_release_proof_packet();
    let timeline = packet.row(M5TrustComponentFamily::TimelineGroup).unwrap();
    assert!(matches!(
        timeline.cross_surface_parity,
        CrossSurfaceParityState::DisclosedReducedSurfaceProjection
    ));
    assert!(timeline.requires_waiver());
    assert!(timeline.has_active_waiver());
    assert_eq!(packet.active_waivers.len(), 1);
    assert!(packet.active_waivers[0].is_active_at(&packet.generated_at));
    assert!(timeline
        .certification_causes
        .iter()
        .any(|cause| cause.disclosed
            && matches!(
                cause.trigger,
                M5TrustComponentDowngradeTrigger::AuditTruthLostOffPrimarySurface
            )));
}

#[test]
fn partial_capture_narrows_but_does_not_block() {
    let packet = seeded_m5_trust_component_release_proof_packet();
    let card = packet
        .row(M5TrustComponentFamily::NarrativeSummaryCard)
        .unwrap();
    assert!(matches!(
        card.support_export_proof,
        SupportExportProofState::DisclosedPartialCapture
    ));
    assert_eq!(card.derived_status, TrustReleaseProofStatus::Yellow);
    assert!(card.certification_causes.iter().any(|cause| cause.disclosed
        && matches!(cause.trigger, M5TrustComponentDowngradeTrigger::ProofStale)));
}

#[test]
fn collapsed_contract_truth_blocks_the_settings_row_family() {
    let packet =
        seeded_m5_trust_component_release_proof_packet_settings_row_contract_truth_collapsed_blocked();
    let row = packet.row(M5TrustComponentFamily::SettingsRow).unwrap();
    assert_eq!(row.derived_status, TrustReleaseProofStatus::Red);
    assert!(!packet.report_clean);
    assert!(!packet.all_rows_publishable);
    assert!(packet.red_row_count >= 1);
    assert!(packet.blocking_findings.iter().any(|finding| matches!(
        finding,
        TrustReleaseProofFinding::ContractTruthCollapsedOrDrifted { .. }
    )));
    assert!(row.certification_causes.iter().any(|cause| !cause.disclosed
        && matches!(
            cause.trigger,
            M5TrustComponentDowngradeTrigger::EffectiveConfiguredConflated
        )));
    assert!(validate_m5_trust_component_release_proof_packet(&packet).is_err());
}

#[test]
fn diverged_row_grammar_blocks_the_capability_sheet_family() {
    let packet =
        seeded_m5_trust_component_release_proof_packet_capability_sheet_row_grammar_diverged_blocked();
    let row = packet.row(M5TrustComponentFamily::CapabilitySheet).unwrap();
    assert_eq!(row.derived_status, TrustReleaseProofStatus::Red);
    assert!(!packet.report_clean);
    assert!(packet.blocking_findings.iter().any(|finding| matches!(
        finding,
        TrustReleaseProofFinding::RowGrammarDivergedOffPrimarySurface { .. }
    )));
    assert!(validate_m5_trust_component_release_proof_packet(&packet).is_err());
}

#[test]
fn absent_capture_blocks_the_event_history_row_family() {
    let packet =
        seeded_m5_trust_component_release_proof_packet_event_history_row_capture_absent_blocked();
    let row = packet.row(M5TrustComponentFamily::EventHistoryRow).unwrap();
    assert_eq!(row.derived_status, TrustReleaseProofStatus::Red);
    assert!(!packet.report_clean);
    assert!(packet.blocking_findings.iter().any(|finding| matches!(
        finding,
        TrustReleaseProofFinding::ComponentTruthAbsentFromCapture { .. }
    )));
    assert!(validate_m5_trust_component_release_proof_packet(&packet).is_err());
}

#[test]
fn stale_proof_blocks_the_timeline_group_family() {
    let packet =
        seeded_m5_trust_component_release_proof_packet_timeline_group_proof_stale_blocked();
    let row = packet.row(M5TrustComponentFamily::TimelineGroup).unwrap();
    assert_eq!(row.derived_status, TrustReleaseProofStatus::Red);
    assert!(!packet.report_clean);
    assert!(packet.blocking_findings.iter().any(|finding| matches!(
        finding,
        TrustReleaseProofFinding::ExportedProofStaleOrDivergent { .. }
    )));
    assert!(validate_m5_trust_component_release_proof_packet(&packet).is_err());
}

#[test]
fn dropped_audit_truth_blocks_the_narrative_summary_card_family() {
    let packet =
        seeded_m5_trust_component_release_proof_packet_narrative_summary_card_audit_truth_dropped_blocked();
    let row = packet
        .row(M5TrustComponentFamily::NarrativeSummaryCard)
        .unwrap();
    assert_eq!(row.derived_status, TrustReleaseProofStatus::Red);
    assert!(!packet.report_clean);
    assert!(packet.blocking_findings.iter().any(|finding| matches!(
        finding,
        TrustReleaseProofFinding::AuditOrSupportTruthDropped { .. }
    )));
    assert!(row.certification_causes.iter().any(|cause| !cause.disclosed
        && matches!(
            cause.trigger,
            M5TrustComponentDowngradeTrigger::AuditTruthLostOffPrimarySurface
        )));
    assert!(validate_m5_trust_component_release_proof_packet(&packet).is_err());
}

#[test]
fn dashboard_projects_every_row_and_counts() {
    let packet = seeded_m5_trust_component_release_proof_packet();
    let dashboard = packet.dashboard();
    assert_eq!(dashboard.rows.len(), packet.rows.len());
    assert_eq!(dashboard.green_row_count, packet.green_row_count);
    assert_eq!(dashboard.yellow_row_count, packet.yellow_row_count);
    assert_eq!(dashboard.red_row_count, packet.red_row_count);
    assert_eq!(dashboard.all_rows_publishable, packet.all_rows_publishable);
    assert_eq!(dashboard.source_packet_ref, packet.packet_id);
    assert!(!dashboard.shell_automation_refs.is_empty());

    let timeline = dashboard
        .rows
        .iter()
        .find(|row| row.component_family == M5TrustComponentFamily::TimelineGroup)
        .unwrap();
    assert_eq!(timeline.status, TrustReleaseProofStatus::Yellow);
    assert!(timeline.has_active_waiver);
    assert!(timeline.cause_tokens.contains(
        &M5TrustComponentDowngradeTrigger::AuditTruthLostOffPrimarySurface
            .as_str()
            .to_owned()
    ));
}

#[test]
fn support_export_quotes_packet_matrix_and_waiver_refs() {
    let packet = seeded_m5_trust_component_release_proof_packet();
    let export = TrustReleaseProofSupportExport::from_packet(
        M5_TRUST_RELEASE_PROOF_SUPPORT_EXPORT_ID,
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
    let packet = seeded_m5_trust_component_release_proof_packet();
    let markdown = packet.render_markdown();
    let csv = packet.render_matrix_csv();
    for family in M5TrustComponentFamily::ALL {
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
    assert!(markdown.contains("m5_trust_component_release_proof_fixtures"));
    assert!(markdown.contains("waiver:reduced-surface-projection:0001"));
}

#[test]
fn export_carries_no_forbidden_material() {
    let packet = seeded_m5_trust_component_release_proof_packet();
    let json = packet.export_safe_json().to_lowercase();
    assert!(!json.contains("api_key"));
    assert!(!json.contains("password"));
    assert!(!json.contains("secret"));
    assert!(!json.contains("bearer "));
    assert!(!json.contains("://"));
}

#[test]
fn expired_waiver_is_flagged() {
    let waiver = TrustReleaseProofWaiver {
        waiver_id: "waiver:expired:0001".to_owned(),
        component_family: M5TrustComponentFamily::TimelineGroup,
        reason: "test".to_owned(),
        owner_role: "owner".to_owned(),
        expires_at: "2026-01-01T00:00:00Z".to_owned(),
    };
    assert!(!waiver.is_active_at("2026-06-30T00:00:00Z"));
    assert!(waiver.is_active_at("2025-01-01T00:00:00Z"));
}
