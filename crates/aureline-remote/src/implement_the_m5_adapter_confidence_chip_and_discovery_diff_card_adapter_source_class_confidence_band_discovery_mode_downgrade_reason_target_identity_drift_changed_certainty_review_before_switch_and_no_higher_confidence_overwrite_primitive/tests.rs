use super::*;

fn clean_chip_input() -> M5AdapterConfidenceChipResolutionInput {
    M5AdapterConfidenceChipResolutionInput {
        chip_id: "adapter-chip:test".to_owned(),
        adapter_source_class: TargetDiscoveryClass::DeclaredManifest,
        source_class_disclosed: true,
        adapter_confidence: AdapterConfidence::Verified,
        confidence_band_disclosed: true,
        discovery_mode: DiscoveryConfidence::Exact,
        discovery_mode_disclosed: true,
        stale: false,
        current_downgrade_reason: None,
        proof_fresh: true,
    }
}

fn clean_card_input() -> M5DiscoveryDiffCardResolutionInput {
    M5DiscoveryDiffCardResolutionInput {
        card_id: "discovery-diff:test".to_owned(),
        previous_target_identity: "web-frontend@service-alpha".to_owned(),
        current_target_identity: "web-frontend@service-beta".to_owned(),
        previous_confidence: DiscoveryConfidence::Structured,
        current_confidence: DiscoveryConfidence::Structured,
        target_identity_disclosed: true,
        material_change: true,
        changed_certainty_disclosed: true,
        review_before_switch_available: true,
        attributed_review_state: true,
        proof_fresh: true,
    }
}

#[test]
fn seeded_controls_validates() {
    let packet = seeded_m5_adapter_discovery_controls();
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
    assert_eq!(packet.packet_id, M5_ADAPTER_DISCOVERY_CONTROLS_PACKET_ID);
}

#[test]
fn chip_exact_names_full_confidence_basis() {
    let resolved = resolve_adapter_confidence_chip(clean_chip_input()).unwrap();
    assert!(resolved.is_clean());
    assert!(resolved.source_class_disclosed);
    assert!(resolved.confidence_band_disclosed);
    assert!(resolved.discovery_mode_disclosed);
    assert!(!resolved.hides_confidence_basis());
    assert_eq!(resolved.certainty, M5AdapterDiscoveryCertainty::Exact);
    assert_eq!(resolved.adapter_source_class, "declared_manifest");
    assert_eq!(resolved.adapter_confidence, "verified");
    assert_eq!(resolved.discovery_mode, "exact");
    assert_eq!(resolved.claim_ceiling, "authoritative");
    assert_eq!(
        resolved.next_action,
        M5AdapterDiscoveryNextAction::NoActionNeeded
    );
}

#[test]
fn chip_certainties_cover_every_state() {
    // Compatible
    let mut input = clean_chip_input();
    input.discovery_mode = DiscoveryConfidence::Structured;
    assert_eq!(
        resolve_adapter_confidence_chip(input).unwrap().certainty,
        M5AdapterDiscoveryCertainty::Compatible
    );
    // Heuristic
    let mut input = clean_chip_input();
    input.discovery_mode = DiscoveryConfidence::Heuristic;
    assert_eq!(
        resolve_adapter_confidence_chip(input).unwrap().certainty,
        M5AdapterDiscoveryCertainty::Heuristic
    );
    // Imported
    let mut input = clean_chip_input();
    input.discovery_mode = DiscoveryConfidence::Imported;
    assert_eq!(
        resolve_adapter_confidence_chip(input).unwrap().certainty,
        M5AdapterDiscoveryCertainty::Imported
    );
    // Downgraded (low confidence)
    let mut input = clean_chip_input();
    input.adapter_confidence = AdapterConfidence::Unverified;
    input.current_downgrade_reason = Some(NarrowingReason::AdapterConfidenceLow);
    assert_eq!(
        resolve_adapter_confidence_chip(input).unwrap().certainty,
        M5AdapterDiscoveryCertainty::Downgraded
    );
    // Stale
    let mut input = clean_chip_input();
    input.stale = true;
    input.current_downgrade_reason = Some(NarrowingReason::EvidenceStale);
    assert_eq!(
        resolve_adapter_confidence_chip(input).unwrap().certainty,
        M5AdapterDiscoveryCertainty::Stale
    );
}

#[test]
fn chip_source_class_undisclosed_degrades_ac1() {
    let mut input = clean_chip_input();
    input.source_class_disclosed = false;
    let resolved = resolve_adapter_confidence_chip(input).unwrap();
    assert!(!resolved.is_clean());
    assert!(resolved.hides_confidence_basis());
    assert_eq!(
        resolved.degrade_reason,
        Some(M5AdapterConfidenceChipDegradeReason::SourceClassUnstated)
    );
}

#[test]
fn chip_undiscovered_source_degrades() {
    let mut input = clean_chip_input();
    input.adapter_source_class = TargetDiscoveryClass::Undiscovered;
    let resolved = resolve_adapter_confidence_chip(input).unwrap();
    assert_eq!(
        resolved.degrade_reason,
        Some(M5AdapterConfidenceChipDegradeReason::SourceClassUnstated)
    );
}

#[test]
fn chip_confidence_band_undisclosed_degrades_ac1() {
    let mut input = clean_chip_input();
    input.confidence_band_disclosed = false;
    let resolved = resolve_adapter_confidence_chip(input).unwrap();
    assert_eq!(
        resolved.degrade_reason,
        Some(M5AdapterConfidenceChipDegradeReason::ConfidenceBandUnstated)
    );
}

#[test]
fn chip_discovery_mode_undisclosed_degrades_ac1() {
    let mut input = clean_chip_input();
    input.discovery_mode_disclosed = false;
    let resolved = resolve_adapter_confidence_chip(input).unwrap();
    assert_eq!(
        resolved.degrade_reason,
        Some(M5AdapterConfidenceChipDegradeReason::DiscoveryModeUnstated)
    );
    assert_eq!(
        resolved.next_action,
        M5AdapterDiscoveryNextAction::ViewConfidenceBasis
    );
}

#[test]
fn chip_downgraded_without_reason_degrades() {
    let mut input = clean_chip_input();
    input.adapter_confidence = AdapterConfidence::Unverified;
    input.current_downgrade_reason = None;
    let resolved = resolve_adapter_confidence_chip(input).unwrap();
    assert_eq!(resolved.certainty, M5AdapterDiscoveryCertainty::Downgraded);
    assert_eq!(
        resolved.degrade_reason,
        Some(M5AdapterConfidenceChipDegradeReason::DowngradeReasonUnattributed)
    );
}

#[test]
fn chip_downgraded_with_reason_is_clean() {
    let mut input = clean_chip_input();
    input.adapter_confidence = AdapterConfidence::Unverified;
    input.current_downgrade_reason = Some(NarrowingReason::AdapterConfidenceLow);
    let resolved = resolve_adapter_confidence_chip(input).unwrap();
    assert!(resolved.is_clean());
    assert_eq!(
        resolved.current_downgrade_reason.as_deref(),
        Some("adapter_confidence_low")
    );
}

#[test]
fn chip_empty_id_and_forbidden_material_error() {
    let mut input = clean_chip_input();
    input.chip_id = "".to_owned();
    assert_eq!(
        resolve_adapter_confidence_chip(input).unwrap_err(),
        M5AdapterDiscoveryResolutionError::EmptyChipId
    );

    let mut input = clean_chip_input();
    input.chip_id = "chip://leak".to_owned();
    assert_eq!(
        resolve_adapter_confidence_chip(input).unwrap_err(),
        M5AdapterDiscoveryResolutionError::ForbiddenMaterial
    );
}

#[test]
fn card_reviewed_change_is_clean() {
    let resolved = resolve_discovery_diff_card(clean_card_input()).unwrap();
    assert!(resolved.is_clean());
    assert!(!resolved.renders_silent_relabel);
    assert!(!resolved.overwrites_higher_confidence_without_review);
    assert_eq!(
        resolved.changed_certainty,
        M5AdapterDiscoveryCertainty::Compatible
    );
    assert_eq!(
        resolved.next_action,
        M5AdapterDiscoveryNextAction::SwitchAfterReview
    );
}

#[test]
fn card_silent_relabel_degrades_ac2() {
    let mut input = clean_card_input();
    input.attributed_review_state = false;
    let resolved = resolve_discovery_diff_card(input).unwrap();
    assert!(!resolved.is_clean());
    assert!(resolved.renders_silent_relabel);
    assert_eq!(
        resolved.degrade_reason,
        Some(M5DiscoveryDiffCardDegradeReason::SilentRelabelWithoutReview)
    );
    assert_eq!(
        resolved.next_action,
        M5AdapterDiscoveryNextAction::ReviewDiscoveryDrift
    );
}

#[test]
fn card_lower_confidence_overwrite_degrades() {
    let mut input = clean_card_input();
    input.previous_confidence = DiscoveryConfidence::Exact;
    input.current_confidence = DiscoveryConfidence::Heuristic;
    input.review_before_switch_available = false;
    let resolved = resolve_discovery_diff_card(input).unwrap();
    assert!(!resolved.is_clean());
    assert!(resolved.overwrites_higher_confidence_without_review);
    assert_eq!(
        resolved.changed_certainty,
        M5AdapterDiscoveryCertainty::Downgraded
    );
    assert_eq!(
        resolved.degrade_reason,
        Some(M5DiscoveryDiffCardDegradeReason::LowerConfidenceOverwroteResolved)
    );
    assert_eq!(
        resolved.next_action,
        M5AdapterDiscoveryNextAction::KeepResolvedTarget
    );
}

#[test]
fn card_lower_confidence_with_review_is_clean() {
    let mut input = clean_card_input();
    input.previous_confidence = DiscoveryConfidence::Exact;
    input.current_confidence = DiscoveryConfidence::Heuristic;
    input.review_before_switch_available = true;
    let resolved = resolve_discovery_diff_card(input).unwrap();
    assert!(resolved.is_clean());
    assert!(!resolved.overwrites_higher_confidence_without_review);
}

#[test]
fn card_changed_certainty_unstated_degrades() {
    let mut input = clean_card_input();
    input.changed_certainty_disclosed = false;
    let resolved = resolve_discovery_diff_card(input).unwrap();
    assert_eq!(
        resolved.degrade_reason,
        Some(M5DiscoveryDiffCardDegradeReason::ChangedCertaintyUnstated)
    );
}

#[test]
fn card_identity_unstated_degrades() {
    let mut input = clean_card_input();
    input.target_identity_disclosed = false;
    let resolved = resolve_discovery_diff_card(input).unwrap();
    assert_eq!(
        resolved.degrade_reason,
        Some(M5DiscoveryDiffCardDegradeReason::TargetIdentityUnstated)
    );
}

#[test]
fn card_no_change_is_clean() {
    let mut input = clean_card_input();
    input.material_change = false;
    let resolved = resolve_discovery_diff_card(input).unwrap();
    assert!(resolved.is_clean());
    assert_eq!(
        resolved.next_action,
        M5AdapterDiscoveryNextAction::NoActionNeeded
    );
}

#[test]
fn card_empty_id_and_forbidden_material_error() {
    let mut input = clean_card_input();
    input.card_id = "   ".to_owned();
    assert_eq!(
        resolve_discovery_diff_card(input).unwrap_err(),
        M5AdapterDiscoveryResolutionError::EmptyCardId
    );

    let mut input = clean_card_input();
    input.current_target_identity = "https://relay.internal/leak".to_owned();
    assert_eq!(
        resolve_discovery_diff_card(input).unwrap_err(),
        M5AdapterDiscoveryResolutionError::ForbiddenMaterial
    );
}

#[test]
fn vocabulary_set_is_canonical() {
    assert!(seeded_m5_adapter_discovery_controls()
        .vocabulary_set
        .matches_canonical());
}

#[test]
fn vocabulary_set_drift_fails() {
    let mut packet = seeded_m5_adapter_discovery_controls();
    packet.vocabulary_set.certainties.pop();
    assert!(packet
        .validate()
        .contains(&M5AdapterDiscoveryControlsViolation::VocabularySetDrift));
}

#[test]
fn missing_source_contracts_fails() {
    let mut packet = seeded_m5_adapter_discovery_controls();
    packet.source_contract_refs.clear();
    assert!(packet
        .validate()
        .contains(&M5AdapterDiscoveryControlsViolation::MissingSourceContracts));
}

#[test]
fn component_schema_ref_missing_fails() {
    let mut packet = seeded_m5_adapter_discovery_controls();
    packet.controls_rows[0]
        .source_contract_refs
        .retain(|r| r != M5_ADAPTER_CONFIDENCE_CHIP_SCHEMA_REF);
    assert!(packet
        .validate()
        .contains(&M5AdapterDiscoveryControlsViolation::ComponentSchemaRefMissing));
}

#[test]
fn mandatory_anatomy_missing_fails() {
    let mut packet = seeded_m5_adapter_discovery_controls();
    packet.controls_rows[0]
        .anatomy_parts
        .retain(|p| *p != M5AdapterDiscoveryAnatomyPart::Identity);
    assert!(packet
        .validate()
        .contains(&M5AdapterDiscoveryControlsViolation::MandatoryAnatomyMissing));
}

#[test]
fn mandatory_export_field_missing_fails() {
    let mut packet = seeded_m5_adapter_discovery_controls();
    packet.controls_rows[0]
        .export_fields
        .retain(|f| *f != M5AdapterDiscoveryExportField::Certainties);
    assert!(packet
        .validate()
        .contains(&M5AdapterDiscoveryControlsViolation::MandatoryExportFieldMissing));
}

#[test]
fn examples_missing_fails() {
    let mut packet = seeded_m5_adapter_discovery_controls();
    packet.controls_rows[0].discovery_diff_card_examples.clear();
    assert!(packet
        .validate()
        .contains(&M5AdapterDiscoveryControlsViolation::ExamplesMissing));
}

#[test]
fn dishonest_clean_example_fails() {
    let mut packet = seeded_m5_adapter_discovery_controls();
    // Force a clean card to also read as a silent relabel — the packet must reject it.
    let row = &mut packet.controls_rows[0];
    row.discovery_diff_card_examples[0].degrade_reason = None;
    row.discovery_diff_card_examples[0].renders_silent_relabel = true;
    assert!(packet
        .validate()
        .contains(&M5AdapterDiscoveryControlsViolation::DishonestExample));
}

#[test]
fn row_invariant_violation_fails() {
    for mutate in 0u8..4 {
        let mut packet = seeded_m5_adapter_discovery_controls();
        let row = &mut packet.controls_rows[0];
        match mutate {
            0 => row.relabels_target_without_attributable_review = true,
            1 => row.lower_confidence_overwrites_resolved_without_review = true,
            2 => row.hides_adapter_confidence_or_discovery_mode = true,
            _ => row.conceals_downgrade_or_drift_in_generic_status_wording = true,
        }
        assert!(packet
            .validate()
            .contains(&M5AdapterDiscoveryControlsViolation::RowInvariantViolated));
    }
}

#[test]
fn ac1_not_proven_when_a_certainty_uncovered() {
    let mut packet = seeded_m5_adapter_discovery_controls();
    // Drop every clean stale chip so the required certainty coverage breaks.
    for row in &mut packet.controls_rows {
        row.adapter_confidence_chip_examples
            .retain(|ex| !(ex.is_clean() && ex.certainty == M5AdapterDiscoveryCertainty::Stale));
    }
    assert!(packet
        .validate()
        .contains(&M5AdapterDiscoveryControlsViolation::Ac1NotProven));
}

#[test]
fn ac1_not_proven_when_mode_unstated_example_removed() {
    let mut packet = seeded_m5_adapter_discovery_controls();
    for row in &mut packet.controls_rows {
        row.adapter_confidence_chip_examples.retain(|ex| {
            ex.degrade_reason != Some(M5AdapterConfidenceChipDegradeReason::DiscoveryModeUnstated)
        });
    }
    assert!(packet
        .validate()
        .contains(&M5AdapterDiscoveryControlsViolation::Ac1NotProven));
}

#[test]
fn ac2_not_proven_when_silent_relabel_example_removed() {
    let mut packet = seeded_m5_adapter_discovery_controls();
    for row in &mut packet.controls_rows {
        row.discovery_diff_card_examples.retain(|ex| {
            ex.degrade_reason != Some(M5DiscoveryDiffCardDegradeReason::SilentRelabelWithoutReview)
        });
    }
    assert!(packet
        .validate()
        .contains(&M5AdapterDiscoveryControlsViolation::Ac2NotProven));
}

#[test]
fn ac2_not_proven_when_overwrite_example_removed() {
    let mut packet = seeded_m5_adapter_discovery_controls();
    for row in &mut packet.controls_rows {
        row.discovery_diff_card_examples.retain(|ex| {
            ex.degrade_reason
                != Some(M5DiscoveryDiffCardDegradeReason::LowerConfidenceOverwroteResolved)
        });
    }
    assert!(packet
        .validate()
        .contains(&M5AdapterDiscoveryControlsViolation::Ac2NotProven));
}

#[test]
fn governance_review_incomplete_fails() {
    let mut packet = seeded_m5_adapter_discovery_controls();
    packet
        .governance_review
        .lower_confidence_never_overwrites_resolved_without_review = false;
    assert!(packet
        .validate()
        .contains(&M5AdapterDiscoveryControlsViolation::GovernanceReviewIncomplete));
}

#[test]
fn consumer_projection_incomplete_fails() {
    let mut packet = seeded_m5_adapter_discovery_controls();
    packet
        .consumer_projection
        .discovery_language_consistent_across_surfaces = false;
    assert!(packet
        .validate()
        .contains(&M5AdapterDiscoveryControlsViolation::ConsumerProjectionIncomplete));
}

#[test]
fn proof_freshness_incomplete_fails() {
    let mut packet = seeded_m5_adapter_discovery_controls();
    packet.proof_freshness.proof_freshness_slo_hours = 0;
    assert!(packet
        .validate()
        .contains(&M5AdapterDiscoveryControlsViolation::ProofFreshnessIncomplete));
}

#[test]
fn release_posture_incomplete_fails() {
    let mut packet = seeded_m5_adapter_discovery_controls();
    packet.release_posture.accessibility_parity_required = false;
    assert!(packet
        .validate()
        .contains(&M5AdapterDiscoveryControlsViolation::ReleasePostureIncomplete));
}

#[test]
fn injected_raw_material_is_rejected() {
    let mut packet = seeded_m5_adapter_discovery_controls();
    packet.controls_rows[0].scope_summary =
        "raw endpoint https://relay.internal.example/session leaked".to_owned();
    assert!(packet
        .validate()
        .contains(&M5AdapterDiscoveryControlsViolation::RawMaterialInExport));
}

#[test]
fn export_carries_no_forbidden_raw_material() {
    let json = seeded_m5_adapter_discovery_controls().export_safe_json();
    let lower = json.to_lowercase();
    assert!(!lower.contains("password"));
    assert!(!lower.contains("passphrase"));
    assert!(!lower.contains("bearer "));
    assert!(!lower.contains("://"));
    assert!(!lower.contains("-----begin"));
}

#[test]
fn csv_has_a_row_per_consumer_surface() {
    let packet = seeded_m5_adapter_discovery_controls();
    let csv = packet.render_matrix_csv();
    let lines: Vec<&str> = csv.lines().collect();
    assert_eq!(lines.len(), 1 + packet.controls_rows.len());
    assert!(lines[0].starts_with("consumer_surface,qualification,owner,"));
}

#[test]
fn markdown_summary_lists_every_consumer_surface() {
    let packet = seeded_m5_adapter_discovery_controls();
    let summary = packet.render_markdown_summary();
    for row in &packet.controls_rows {
        assert!(summary.contains(row.consumer_surface.as_str()));
    }
}

#[test]
fn checked_support_export_validates_and_matches_seed() {
    let from_disk = current_stable_m5_adapter_discovery_controls_export()
        .expect("checked M5 adapter-discovery controls export validates");
    assert_eq!(from_disk.packet_id, M5_ADAPTER_DISCOVERY_CONTROLS_PACKET_ID);
    assert_eq!(
        from_disk,
        seeded_m5_adapter_discovery_controls(),
        "checked support export drifted from the seed builder"
    );
}

#[test]
fn narrowed_variants_validate_and_keep_rows_visible() {
    let beta = seeded_m5_adapter_discovery_controls_run_test_debug_beta_narrowed();
    assert!(beta.validate().is_empty(), "{:?}", beta.validate());
    assert_eq!(beta.controls_rows.len(), 5);
    let row = beta
        .controls_rows
        .iter()
        .find(|r| r.consumer_surface == M5BuildRemoteConsumerSurface::RunTestDebugUi)
        .unwrap();
    assert_eq!(row.qualification, M5BuildRemoteQualificationClass::Beta);

    let preview = seeded_m5_adapter_discovery_controls_preview_row_preview_narrowed();
    assert!(preview.validate().is_empty(), "{:?}", preview.validate());
    assert_eq!(preview.controls_rows.len(), 5);
    let row = preview
        .controls_rows
        .iter()
        .find(|r| r.consumer_surface == M5BuildRemoteConsumerSurface::PreviewUi)
        .unwrap();
    assert_eq!(row.qualification, M5BuildRemoteQualificationClass::Preview);
}

#[test]
fn checked_narrowed_fixtures_validate_and_match_seed_builders() {
    let beta: M5AdapterDiscoveryControlsPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/ui/m5-adapter-confidence-chip-discovery-diff-card-controls/run_test_debug_beta_narrowed.json"
    )))
    .expect("run/test/debug fixture parses");
    assert!(beta.validate().is_empty());
    assert_eq!(
        beta,
        seeded_m5_adapter_discovery_controls_run_test_debug_beta_narrowed()
    );

    let preview: M5AdapterDiscoveryControlsPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/ui/m5-adapter-confidence-chip-discovery-diff-card-controls/preview_preview_narrowed.json"
    )))
    .expect("preview fixture parses");
    assert!(preview.validate().is_empty());
    assert_eq!(
        preview,
        seeded_m5_adapter_discovery_controls_preview_row_preview_narrowed()
    );
}
