use super::*;

#[test]
fn seeded_matrix_validates() {
    let packet = seeded_m5_trust_chronology_component_matrix();
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
    assert_eq!(packet.packet_id, M5_TRUST_COMPONENTS_MATRIX_PACKET_ID);
}

#[test]
fn seeded_matrix_names_every_component_family() {
    let packet = seeded_m5_trust_chronology_component_matrix();
    let present: std::collections::BTreeSet<_> = packet
        .component_rows
        .iter()
        .map(|r| r.component_family)
        .collect();
    for family in M5TrustComponentFamily::ALL {
        assert!(
            present.contains(&family),
            "missing component family {}",
            family.as_str()
        );
    }
    assert_eq!(
        packet.component_rows.len(),
        M5TrustComponentFamily::ALL.len()
    );
}

#[test]
fn every_component_declares_mandatory_labels_and_a_zone() {
    let packet = seeded_m5_trust_chronology_component_matrix();
    for row in &packet.component_rows {
        for label in M5TrustRequiredLabel::MANDATORY {
            assert!(
                row.required_labels.contains(&label),
                "component {} missing mandatory label {}",
                row.component_family.as_str(),
                label.as_str()
            );
        }
        assert!(!row.responsive_classes.is_empty());
        assert!(!row.window_classes.is_empty());
        assert!(!row.surface_families.is_empty());
        assert!(!row.accessibility_routes.is_empty());
        assert!(row
            .accessibility_routes
            .contains(&M5TrustAccessibilityRoute::KeyboardFocusable));
    }
}

#[test]
fn family_specific_vocabularies_are_declared_only_where_applicable() {
    let packet = seeded_m5_trust_chronology_component_matrix();
    for row in &packet.component_rows {
        let family = row.component_family;
        assert_eq!(
            !row.settings_row_states.is_empty(),
            family.is_settings(),
            "settings_row_states presence wrong for {}",
            family.as_str()
        );
        assert_eq!(
            !row.source_pills.is_empty(),
            family.is_settings(),
            "source_pills presence wrong for {}",
            family.as_str()
        );
        assert_eq!(
            !row.consequence_classes.is_empty(),
            family.is_capability(),
            "consequence_classes presence wrong for {}",
            family.as_str()
        );
        assert_eq!(
            !row.capability_scope_states.is_empty(),
            family.is_capability(),
            "capability_scope_states presence wrong for {}",
            family.as_str()
        );
        assert_eq!(
            !row.chronology_verbs.is_empty(),
            family.is_chronology_row(),
            "chronology_verbs presence wrong for {}",
            family.as_str()
        );
        assert_eq!(
            !row.provenance_badges.is_empty(),
            family.is_chronology_row(),
            "provenance_badges presence wrong for {}",
            family.as_str()
        );
        assert_eq!(
            !row.chronology_detail_states.is_empty(),
            family.groups_chronology(),
            "chronology_detail_states presence wrong for {}",
            family.as_str()
        );
        assert_eq!(
            !row.chronology_export_fields.is_empty(),
            family.is_export(),
            "chronology_export_fields presence wrong for {}",
            family.as_str()
        );
    }
}

#[test]
fn every_vocabulary_token_is_declared_by_some_component() {
    let packet = seeded_m5_trust_chronology_component_matrix();
    for state in M5SettingsRowState::ALL {
        assert!(
            packet
                .component_rows
                .iter()
                .any(|row| row.settings_row_states.contains(&state)),
            "no component declares settings-row state {}",
            state.as_str()
        );
    }
    for pill in M5SettingSourcePill::ALL {
        assert!(
            packet
                .component_rows
                .iter()
                .any(|row| row.source_pills.contains(&pill)),
            "no component declares source pill {}",
            pill.as_str()
        );
    }
    for class in M5CapabilityConsequenceClass::ALL {
        assert!(
            packet
                .component_rows
                .iter()
                .any(|row| row.consequence_classes.contains(&class)),
            "no component declares consequence class {}",
            class.as_str()
        );
    }
    for state in M5CapabilityScopeState::ALL {
        assert!(
            packet
                .component_rows
                .iter()
                .any(|row| row.capability_scope_states.contains(&state)),
            "no component declares scope state {}",
            state.as_str()
        );
    }
    for verb in M5ChronologyVerb::ALL {
        assert!(
            packet
                .component_rows
                .iter()
                .any(|row| row.chronology_verbs.contains(&verb)),
            "no component declares chronology verb {}",
            verb.as_str()
        );
    }
    for badge in M5ProvenanceBadge::ALL {
        assert!(
            packet
                .component_rows
                .iter()
                .any(|row| row.provenance_badges.contains(&badge)),
            "no component declares provenance badge {}",
            badge.as_str()
        );
    }
    for state in M5ChronologyDetailState::ALL {
        assert!(
            packet
                .component_rows
                .iter()
                .any(|row| row.chronology_detail_states.contains(&state)),
            "no component declares chronology detail state {}",
            state.as_str()
        );
    }
    for field in M5ChronologyExportField::ALL {
        assert!(
            packet
                .component_rows
                .iter()
                .any(|row| row.chronology_export_fields.contains(&field)),
            "no component declares chronology export field {}",
            field.as_str()
        );
    }
}

#[test]
fn missing_component_family_fails_validation() {
    let mut packet = seeded_m5_trust_chronology_component_matrix();
    packet
        .component_rows
        .retain(|row| row.component_family != M5TrustComponentFamily::TimelineGroup);
    assert!(packet
        .validate()
        .contains(&M5TrustComponentMatrixViolation::RequiredComponentMissing));
}

#[test]
fn vocabulary_set_drift_fails() {
    let mut packet = seeded_m5_trust_chronology_component_matrix();
    packet.vocabulary_set.chronology_verbs.pop();
    assert!(packet
        .validate()
        .contains(&M5TrustComponentMatrixViolation::VocabularySetDrift));
}

#[test]
fn mandatory_label_missing_fails() {
    let mut packet = seeded_m5_trust_chronology_component_matrix();
    packet.component_rows[0]
        .required_labels
        .retain(|label| *label != M5TrustRequiredLabel::Identity);
    assert!(packet
        .validate()
        .contains(&M5TrustComponentMatrixViolation::MandatoryLabelMissing));
}

#[test]
fn settings_row_state_missing_fails_for_settings_row() {
    let mut packet = seeded_m5_trust_chronology_component_matrix();
    let row = packet
        .component_rows
        .iter_mut()
        .find(|row| row.component_family == M5TrustComponentFamily::SettingsRow)
        .expect("settings row present");
    row.settings_row_states.clear();
    assert!(packet
        .validate()
        .contains(&M5TrustComponentMatrixViolation::SettingsRowStateMissing));
}

#[test]
fn source_pill_missing_fails_for_settings_row() {
    let mut packet = seeded_m5_trust_chronology_component_matrix();
    let row = packet
        .component_rows
        .iter_mut()
        .find(|row| row.component_family == M5TrustComponentFamily::SettingsRow)
        .expect("settings row present");
    row.source_pills.clear();
    assert!(packet
        .validate()
        .contains(&M5TrustComponentMatrixViolation::SourcePillMissing));
}

#[test]
fn consequence_class_missing_fails_for_capability_sheet() {
    let mut packet = seeded_m5_trust_chronology_component_matrix();
    let row = packet
        .component_rows
        .iter_mut()
        .find(|row| row.component_family == M5TrustComponentFamily::CapabilitySheet)
        .expect("capability sheet present");
    row.consequence_classes.clear();
    assert!(packet
        .validate()
        .contains(&M5TrustComponentMatrixViolation::ConsequenceClassMissing));
}

#[test]
fn capability_scope_state_missing_fails_for_capability_sheet() {
    let mut packet = seeded_m5_trust_chronology_component_matrix();
    let row = packet
        .component_rows
        .iter_mut()
        .find(|row| row.component_family == M5TrustComponentFamily::CapabilitySheet)
        .expect("capability sheet present");
    row.capability_scope_states.clear();
    assert!(packet
        .validate()
        .contains(&M5TrustComponentMatrixViolation::CapabilityScopeStateMissing));
}

#[test]
fn chronology_verb_missing_fails_for_event_row() {
    let mut packet = seeded_m5_trust_chronology_component_matrix();
    let row = packet
        .component_rows
        .iter_mut()
        .find(|row| row.component_family == M5TrustComponentFamily::EventHistoryRow)
        .expect("event history row present");
    row.chronology_verbs.clear();
    assert!(packet
        .validate()
        .contains(&M5TrustComponentMatrixViolation::ChronologyVerbMissing));
}

#[test]
fn provenance_badge_missing_fails_for_timeline_group() {
    let mut packet = seeded_m5_trust_chronology_component_matrix();
    let row = packet
        .component_rows
        .iter_mut()
        .find(|row| row.component_family == M5TrustComponentFamily::TimelineGroup)
        .expect("timeline group present");
    row.provenance_badges.clear();
    assert!(packet
        .validate()
        .contains(&M5TrustComponentMatrixViolation::ProvenanceBadgeMissing));
}

#[test]
fn chronology_detail_state_missing_fails_for_timeline_group() {
    let mut packet = seeded_m5_trust_chronology_component_matrix();
    let row = packet
        .component_rows
        .iter_mut()
        .find(|row| row.component_family == M5TrustComponentFamily::TimelineGroup)
        .expect("timeline group present");
    row.chronology_detail_states.clear();
    assert!(packet
        .validate()
        .contains(&M5TrustComponentMatrixViolation::ChronologyDetailStateMissing));
}

#[test]
fn export_field_missing_fails_for_export_preview() {
    let mut packet = seeded_m5_trust_chronology_component_matrix();
    let row = packet
        .component_rows
        .iter_mut()
        .find(|row| row.component_family == M5TrustComponentFamily::ChronologyExportPreview)
        .expect("chronology export preview present");
    row.chronology_export_fields.clear();
    assert!(packet
        .validate()
        .contains(&M5TrustComponentMatrixViolation::ExportFieldMissing));
}

#[test]
fn component_invariant_violation_fails() {
    let mut packet = seeded_m5_trust_chronology_component_matrix();
    packet.component_rows[0].conflates_effective_and_configured = true;
    assert!(packet
        .validate()
        .contains(&M5TrustComponentMatrixViolation::ComponentInvariantViolated));

    let mut packet = seeded_m5_trust_chronology_component_matrix();
    packet.component_rows[1].hides_permission_scope = true;
    assert!(packet
        .validate()
        .contains(&M5TrustComponentMatrixViolation::ComponentInvariantViolated));

    let mut packet = seeded_m5_trust_chronology_component_matrix();
    packet.component_rows[2].invents_private_row_grammar = true;
    assert!(packet
        .validate()
        .contains(&M5TrustComponentMatrixViolation::ComponentInvariantViolated));

    let mut packet = seeded_m5_trust_chronology_component_matrix();
    packet.component_rows[3].drops_audit_or_support_truth = true;
    assert!(packet
        .validate()
        .contains(&M5TrustComponentMatrixViolation::ComponentInvariantViolated));
}

#[test]
fn stable_component_missing_proof_fails() {
    let mut packet = seeded_m5_trust_chronology_component_matrix();
    let row = packet
        .component_rows
        .iter_mut()
        .find(|row| row.component_family == M5TrustComponentFamily::SettingsRow)
        .expect("settings row present");
    row.required_proof_packet_refs.clear();
    assert!(packet
        .validate()
        .contains(&M5TrustComponentMatrixViolation::StableComponentMissingProof));
}

#[test]
fn missing_consumer_surfaces_fails() {
    let mut packet = seeded_m5_trust_chronology_component_matrix();
    packet.component_rows[1].consumer_surfaces.clear();
    assert!(packet
        .validate()
        .contains(&M5TrustComponentMatrixViolation::ConsumerSurfacesMissing));
}

#[test]
fn missing_source_contracts_fails() {
    let mut packet = seeded_m5_trust_chronology_component_matrix();
    packet.source_contract_refs.clear();
    assert!(packet
        .validate()
        .contains(&M5TrustComponentMatrixViolation::MissingSourceContracts));
}

#[test]
fn governance_review_incomplete_fails() {
    let mut packet = seeded_m5_trust_chronology_component_matrix();
    packet
        .governance_review
        .no_component_invents_second_row_grammar = false;
    assert!(packet
        .validate()
        .contains(&M5TrustComponentMatrixViolation::GovernanceReviewIncomplete));
}

#[test]
fn consumer_projection_incomplete_fails() {
    let mut packet = seeded_m5_trust_chronology_component_matrix();
    packet
        .consumer_projection
        .chronology_export_reads_single_source = false;
    assert!(packet
        .validate()
        .contains(&M5TrustComponentMatrixViolation::ConsumerProjectionIncomplete));
}

#[test]
fn proof_freshness_incomplete_fails() {
    let mut packet = seeded_m5_trust_chronology_component_matrix();
    packet.proof_freshness.proof_freshness_slo_hours = 0;
    assert!(packet
        .validate()
        .contains(&M5TrustComponentMatrixViolation::ProofFreshnessIncomplete));
}

#[test]
fn release_posture_incomplete_fails() {
    let mut packet = seeded_m5_trust_chronology_component_matrix();
    packet.release_posture.accessibility_parity_required = false;
    assert!(packet
        .validate()
        .contains(&M5TrustComponentMatrixViolation::ReleasePostureIncomplete));
}

#[test]
fn markdown_summary_lists_every_component_family() {
    let summary = seeded_m5_trust_chronology_component_matrix().render_markdown_summary();
    for family in M5TrustComponentFamily::ALL {
        assert!(
            summary.contains(family.as_str()),
            "summary missing component {}",
            family.as_str()
        );
    }
}

#[test]
fn matrix_csv_has_a_row_per_component() {
    let csv = seeded_m5_trust_chronology_component_matrix().render_matrix_csv();
    let lines: Vec<&str> = csv.lines().collect();
    assert_eq!(lines.len(), 1 + M5TrustComponentFamily::ALL.len());
    assert!(lines[0].starts_with("component_family,qualification,owner,"));
    for family in M5TrustComponentFamily::ALL {
        assert!(
            csv.contains(family.as_str()),
            "csv missing component {}",
            family.as_str()
        );
    }
}

#[test]
fn checked_support_export_validates() {
    let packet = current_stable_m5_trust_chronology_component_matrix_export()
        .expect("checked M5 trust component matrix export validates");
    assert_eq!(packet.packet_id, M5_TRUST_COMPONENTS_MATRIX_PACKET_ID);
}

#[test]
fn checked_support_export_matches_seed() {
    let from_disk = current_stable_m5_trust_chronology_component_matrix_export()
        .expect("checked M5 trust component matrix export validates");
    assert_eq!(
        from_disk,
        seeded_m5_trust_chronology_component_matrix(),
        "checked support export drifted from the seed builder"
    );
}

#[test]
fn narrowed_variants_validate_and_keep_components_visible() {
    for packet in [
        seeded_m5_trust_chronology_component_matrix_narrative_summary_card_beta_narrowed(),
        seeded_m5_trust_chronology_component_matrix_chronology_export_preview_preview_narrowed(),
    ] {
        assert!(
            packet.validate().is_empty(),
            "narrowed variant failed validation: {:?}",
            packet.validate()
        );
        assert_eq!(
            packet.component_rows.len(),
            M5TrustComponentFamily::ALL.len()
        );
    }

    let narrative =
        seeded_m5_trust_chronology_component_matrix_narrative_summary_card_beta_narrowed();
    let row = narrative
        .component_rows
        .iter()
        .find(|r| r.component_family == M5TrustComponentFamily::NarrativeSummaryCard)
        .expect("narrative-summary-card row present");
    assert_eq!(row.qualification, M5TrustQualificationClass::Beta);

    let export =
        seeded_m5_trust_chronology_component_matrix_chronology_export_preview_preview_narrowed();
    let row = export
        .component_rows
        .iter()
        .find(|r| r.component_family == M5TrustComponentFamily::ChronologyExportPreview)
        .expect("chronology-export-preview row present");
    assert_eq!(row.qualification, M5TrustQualificationClass::Preview);
}

#[test]
fn checked_narrowed_fixtures_validate_and_match_seed_builders() {
    let narrative: M5TrustComponentMatrixPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/ui/m5-trust-chronology-components/narrative_summary_card_beta_narrowed.json"
    )))
    .expect("narrative fixture parses");
    assert!(narrative.validate().is_empty());
    assert_eq!(
        narrative,
        seeded_m5_trust_chronology_component_matrix_narrative_summary_card_beta_narrowed()
    );

    let export: M5TrustComponentMatrixPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/ui/m5-trust-chronology-components/chronology_export_preview_preview_narrowed.json"
    )))
    .expect("export fixture parses");
    assert!(export.validate().is_empty());
    assert_eq!(
        export,
        seeded_m5_trust_chronology_component_matrix_chronology_export_preview_preview_narrowed()
    );
}

#[test]
fn export_carries_no_forbidden_material() {
    let json = seeded_m5_trust_chronology_component_matrix().export_safe_json();
    let lower = json.to_lowercase();
    assert!(!lower.contains("api_key"));
    assert!(!lower.contains("password"));
    assert!(!lower.contains("bearer "));
    assert!(!lower.contains("://"));
    assert!(!lower.contains("secret"));
}
