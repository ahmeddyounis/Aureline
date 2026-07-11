use super::*;

fn clean_strip_input() -> M5CompatibilityLabelStripResolutionInput {
    M5CompatibilityLabelStripResolutionInput {
        strip_id: "compat-strip:test".to_owned(),
        artifact_identity: "test-artifact".to_owned(),
        compatibility: M5CompatibilityState::Compatible,
        host_runtime_model: M5HostRuntimeModel::Sandboxed,
        host_version_range: ">=1.0.0, <2.0.0".to_owned(),
        manifest_schema_version: "manifest-schema v3".to_owned(),
        lifecycle: M5CompatibilityLifecycleState::Active,
        replacement_path: "".to_owned(),
        certified_or_supported_claimed: true,
        evidence_fresh: true,
        reads_incompatible_as_ready: false,
        proof_fresh: true,
    }
}

fn clean_row_input() -> M5PublisherContinuityRowResolutionInput {
    M5PublisherContinuityRowResolutionInput {
        row_id: "publisher-row:test".to_owned(),
        artifact_identity: "test-artifact".to_owned(),
        continuity: M5PublisherContinuityState::Continuous,
        registry_source: M5RegistrySourceClass::PublicRegistry,
        continuity_language: "".to_owned(),
        transfer_history_available: false,
        transfer_history_stated: false,
        certified_or_supported_claimed: true,
        evidence_fresh: true,
        collapses_source_class: false,
        proof_fresh: true,
    }
}

#[test]
fn seeded_controls_validates() {
    let packet = seeded_m5_compatibility_continuity_controls();
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
    assert_eq!(
        packet.packet_id,
        M5_COMPATIBILITY_CONTINUITY_CONTROLS_PACKET_ID
    );
}

#[test]
fn strip_clean_names_ranges_and_is_legible() {
    let resolved = resolve_compatibility_label_strip(clean_strip_input()).unwrap();
    assert!(resolved.is_clean());
    assert!(resolved.fully_legible);
    assert!(resolved.is_installable);
    assert!(!resolved.presents_incompatible_as_ready);
    assert!(!resolved.leaves_stale_certified_overclaim);
    assert_eq!(resolved.host_version_range, ">=1.0.0, <2.0.0");
    assert_eq!(resolved.manifest_schema_version, "manifest-schema v3");
    assert_eq!(
        resolved.next_action,
        M5CompatibilityContinuityNextAction::NoActionNeeded
    );
}

#[test]
fn strip_host_version_unstated_degrades() {
    let mut input = clean_strip_input();
    input.host_version_range = "  ".to_owned();
    let resolved = resolve_compatibility_label_strip(input).unwrap();
    assert_eq!(
        resolved.degrade_reason,
        Some(M5CompatibilityLabelStripDegradeReason::HostVersionRangeUnstated)
    );
}

#[test]
fn strip_manifest_schema_unstated_degrades() {
    let mut input = clean_strip_input();
    input.manifest_schema_version = "".to_owned();
    let resolved = resolve_compatibility_label_strip(input).unwrap();
    assert_eq!(
        resolved.degrade_reason,
        Some(M5CompatibilityLabelStripDegradeReason::ManifestSchemaVersionUnstated)
    );
}

#[test]
fn strip_incompatible_shown_ready_degrades() {
    let mut input = clean_strip_input();
    input.compatibility = M5CompatibilityState::Incompatible;
    input.reads_incompatible_as_ready = true;
    let resolved = resolve_compatibility_label_strip(input).unwrap();
    assert!(resolved.presents_incompatible_as_ready);
    assert!(!resolved.is_installable);
    assert_eq!(
        resolved.degrade_reason,
        Some(M5CompatibilityLabelStripDegradeReason::IncompatibleShownAsReady)
    );
    assert_eq!(
        resolved.next_action,
        M5CompatibilityContinuityNextAction::ReviewCompatibility
    );
}

#[test]
fn strip_lifecycle_unstated_degrades() {
    let mut input = clean_strip_input();
    input.lifecycle = M5CompatibilityLifecycleState::LifecycleUnknown;
    let resolved = resolve_compatibility_label_strip(input).unwrap();
    assert_eq!(
        resolved.degrade_reason,
        Some(M5CompatibilityLabelStripDegradeReason::LifecycleStateUnstated)
    );
}

#[test]
fn strip_deprecated_without_replacement_degrades() {
    let mut input = clean_strip_input();
    input.certified_or_supported_claimed = false;
    input.lifecycle = M5CompatibilityLifecycleState::Deprecated;
    input.replacement_path = "".to_owned();
    let resolved = resolve_compatibility_label_strip(input).unwrap();
    assert!(resolved.requires_replacement_path);
    assert_eq!(
        resolved.degrade_reason,
        Some(M5CompatibilityLabelStripDegradeReason::ReplacementPathMissing)
    );
    assert_eq!(
        resolved.next_action,
        M5CompatibilityContinuityNextAction::ReviewReplacementPath
    );
}

#[test]
fn strip_deprecated_with_replacement_is_clean() {
    let mut input = clean_strip_input();
    input.certified_or_supported_claimed = false;
    input.lifecycle = M5CompatibilityLifecycleState::Deprecated;
    input.replacement_path = "use modern-tool >=2.0".to_owned();
    let resolved = resolve_compatibility_label_strip(input).unwrap();
    assert!(resolved.is_clean());
    assert!(resolved.requires_replacement_path);
}

#[test]
fn strip_stale_certified_overclaim_degrades() {
    let mut input = clean_strip_input();
    input.certified_or_supported_claimed = true;
    input.evidence_fresh = false;
    let resolved = resolve_compatibility_label_strip(input).unwrap();
    assert!(resolved.leaves_stale_certified_overclaim);
    assert_eq!(
        resolved.degrade_reason,
        Some(M5CompatibilityLabelStripDegradeReason::StaleEvidenceCertifiedOverclaim)
    );
    assert_eq!(
        resolved.next_action,
        M5CompatibilityContinuityNextAction::ReviewEvidenceFreshness
    );
}

#[test]
fn strip_empty_id_and_forbidden_material_error() {
    let mut input = clean_strip_input();
    input.strip_id = "".to_owned();
    assert_eq!(
        resolve_compatibility_label_strip(input).unwrap_err(),
        M5CompatibilityContinuityResolutionError::EmptyStripId
    );

    let mut input = clean_strip_input();
    input.replacement_path = "see internal://notes".to_owned();
    assert_eq!(
        resolve_compatibility_label_strip(input).unwrap_err(),
        M5CompatibilityContinuityResolutionError::ForbiddenMaterial
    );
}

#[test]
fn row_clean_names_continuity_and_source() {
    let resolved = resolve_publisher_continuity_row(clean_row_input()).unwrap();
    assert!(resolved.is_clean());
    assert!(resolved.fully_legible);
    assert!(!resolved.collapses_source_class);
    assert!(!resolved.hides_continuity_language);
    assert!(!resolved.leaves_stale_certified_overclaim);
    assert_eq!(resolved.presentation, "continuous");
    assert_eq!(
        resolved.source_disposition,
        Some(M5MarketplaceInstallDisposition::Public)
    );
}

#[test]
fn row_mirror_source_presents_as_mirrored() {
    let mut input = clean_row_input();
    input.certified_or_supported_claimed = false;
    input.registry_source = M5RegistrySourceClass::MirroredRegistry;
    let resolved = resolve_publisher_continuity_row(input).unwrap();
    assert_eq!(resolved.presentation, "mirrored");
    assert_eq!(
        resolved.source_disposition,
        Some(M5MarketplaceInstallDisposition::Mirrored)
    );
}

#[test]
fn row_source_unknown_degrades_and_has_no_disposition() {
    let mut input = clean_row_input();
    input.certified_or_supported_claimed = false;
    input.registry_source = M5RegistrySourceClass::SourceUnknown;
    let resolved = resolve_publisher_continuity_row(input).unwrap();
    assert_eq!(
        resolved.degrade_reason,
        Some(M5PublisherContinuityRowDegradeReason::RegistrySourceUnresolved)
    );
    assert_eq!(resolved.source_disposition, None);
}

#[test]
fn row_source_collapsed_degrades() {
    let mut input = clean_row_input();
    input.certified_or_supported_claimed = false;
    input.collapses_source_class = true;
    let resolved = resolve_publisher_continuity_row(input).unwrap();
    assert!(resolved.collapses_source_class);
    assert_eq!(
        resolved.degrade_reason,
        Some(M5PublisherContinuityRowDegradeReason::SourceClassCollapsedIntoAmbiguousOrigin)
    );
}

#[test]
fn row_transferred_without_continuity_language_degrades() {
    let mut input = clean_row_input();
    input.certified_or_supported_claimed = false;
    input.continuity = M5PublisherContinuityState::Transferred;
    input.continuity_language = "".to_owned();
    let resolved = resolve_publisher_continuity_row(input).unwrap();
    assert!(resolved.publisher_changed);
    assert!(resolved.hides_continuity_language);
    assert_eq!(
        resolved.presentation,
        M5PublisherContinuityPresentation::Transferred.as_str()
    );
    assert_eq!(
        resolved.degrade_reason,
        Some(M5PublisherContinuityRowDegradeReason::ContinuityLanguageHidden)
    );
}

#[test]
fn row_transferred_with_continuity_language_is_clean() {
    let mut input = clean_row_input();
    input.certified_or_supported_claimed = false;
    input.continuity = M5PublisherContinuityState::Transferred;
    input.continuity_language = "transferred to new-owner".to_owned();
    let resolved = resolve_publisher_continuity_row(input).unwrap();
    assert!(resolved.is_clean());
    assert!(resolved.publisher_changed);
    assert!(!resolved.hides_continuity_language);
}

#[test]
fn row_transfer_history_hidden_degrades() {
    let mut input = clean_row_input();
    input.certified_or_supported_claimed = false;
    input.continuity = M5PublisherContinuityState::Transferred;
    input.continuity_language = "transferred to new-owner".to_owned();
    input.transfer_history_available = true;
    input.transfer_history_stated = false;
    let resolved = resolve_publisher_continuity_row(input).unwrap();
    assert_eq!(
        resolved.degrade_reason,
        Some(M5PublisherContinuityRowDegradeReason::TransferHistoryHidden)
    );
    assert_eq!(
        resolved.next_action,
        M5CompatibilityContinuityNextAction::ReviewTransferHistory
    );
}

#[test]
fn row_stale_certified_overclaim_degrades() {
    let mut input = clean_row_input();
    input.certified_or_supported_claimed = true;
    input.evidence_fresh = false;
    let resolved = resolve_publisher_continuity_row(input).unwrap();
    assert!(resolved.leaves_stale_certified_overclaim);
    assert_eq!(
        resolved.degrade_reason,
        Some(M5PublisherContinuityRowDegradeReason::StaleOrUnverifiableCertifiedOverclaim)
    );
}

#[test]
fn row_unverifiable_certified_overclaim_degrades() {
    let mut input = clean_row_input();
    input.continuity = M5PublisherContinuityState::ContinuityUnknown;
    input.certified_or_supported_claimed = true;
    input.evidence_fresh = true;
    let resolved = resolve_publisher_continuity_row(input).unwrap();
    assert_eq!(resolved.presentation, "unverifiable");
    assert!(resolved.leaves_stale_certified_overclaim);
    assert_eq!(
        resolved.degrade_reason,
        Some(M5PublisherContinuityRowDegradeReason::StaleOrUnverifiableCertifiedOverclaim)
    );
}

#[test]
fn row_unverifiable_without_certified_claim_is_clean() {
    let mut input = clean_row_input();
    input.continuity = M5PublisherContinuityState::ContinuityUnknown;
    input.certified_or_supported_claimed = false;
    let resolved = resolve_publisher_continuity_row(input).unwrap();
    assert!(resolved.is_clean());
    assert_eq!(resolved.presentation, "unverifiable");
}

#[test]
fn row_empty_id_and_forbidden_material_error() {
    let mut input = clean_row_input();
    input.row_id = "   ".to_owned();
    assert_eq!(
        resolve_publisher_continuity_row(input).unwrap_err(),
        M5CompatibilityContinuityResolutionError::EmptyRowId
    );

    let mut input = clean_row_input();
    input.continuity_language = "https://relay.internal/leak".to_owned();
    assert_eq!(
        resolve_publisher_continuity_row(input).unwrap_err(),
        M5CompatibilityContinuityResolutionError::ForbiddenMaterial
    );
}

#[test]
fn vocabulary_set_is_canonical() {
    assert!(seeded_m5_compatibility_continuity_controls()
        .vocabulary_set
        .matches_canonical());
}

#[test]
fn vocabulary_set_drift_fails() {
    let mut packet = seeded_m5_compatibility_continuity_controls();
    packet.vocabulary_set.lifecycle_states.pop();
    assert!(packet
        .validate()
        .contains(&M5CompatibilityContinuityControlsViolation::VocabularySetDrift));
}

#[test]
fn missing_source_contracts_fails() {
    let mut packet = seeded_m5_compatibility_continuity_controls();
    packet.source_contract_refs.clear();
    assert!(packet
        .validate()
        .contains(&M5CompatibilityContinuityControlsViolation::MissingSourceContracts));
}

#[test]
fn component_schema_ref_missing_fails() {
    let mut packet = seeded_m5_compatibility_continuity_controls();
    packet.controls_rows[0]
        .source_contract_refs
        .retain(|r| r != M5_PUBLISHER_CONTINUITY_ROW_SCHEMA_REF);
    assert!(packet
        .validate()
        .contains(&M5CompatibilityContinuityControlsViolation::ComponentSchemaRefMissing));
}

#[test]
fn mandatory_anatomy_missing_fails() {
    let mut packet = seeded_m5_compatibility_continuity_controls();
    packet.controls_rows[0]
        .anatomy_parts
        .retain(|p| *p != M5CompatibilityContinuityAnatomyPart::Identity);
    assert!(packet
        .validate()
        .contains(&M5CompatibilityContinuityControlsViolation::MandatoryAnatomyMissing));
}

#[test]
fn mandatory_export_field_missing_fails() {
    let mut packet = seeded_m5_compatibility_continuity_controls();
    packet.controls_rows[0]
        .export_fields
        .retain(|f| *f != M5CompatibilityContinuityExportField::Dispositions);
    assert!(packet
        .validate()
        .contains(&M5CompatibilityContinuityControlsViolation::MandatoryExportFieldMissing));
}

#[test]
fn examples_missing_fails() {
    let mut packet = seeded_m5_compatibility_continuity_controls();
    packet.controls_rows[0]
        .publisher_continuity_row_examples
        .clear();
    assert!(packet
        .validate()
        .contains(&M5CompatibilityContinuityControlsViolation::ExamplesMissing));
}

#[test]
fn dishonest_clean_strip_example_fails() {
    let mut packet = seeded_m5_compatibility_continuity_controls();
    // Force a clean strip to also read as leaving a stale-certified overclaim — the packet must
    // reject it.
    let row = &mut packet.controls_rows[0];
    row.compatibility_label_strip_examples[0].degrade_reason = None;
    row.compatibility_label_strip_examples[0].leaves_stale_certified_overclaim = true;
    assert!(packet
        .validate()
        .contains(&M5CompatibilityContinuityControlsViolation::DishonestExample));
}

#[test]
fn dishonest_clean_row_example_fails() {
    let mut packet = seeded_m5_compatibility_continuity_controls();
    let row = &mut packet.controls_rows[0];
    row.publisher_continuity_row_examples[0].degrade_reason = None;
    row.publisher_continuity_row_examples[0].hides_continuity_language = true;
    assert!(packet
        .validate()
        .contains(&M5CompatibilityContinuityControlsViolation::DishonestExample));
}

#[test]
fn row_invariant_violation_fails() {
    for mutate in 0u8..4 {
        let mut packet = seeded_m5_compatibility_continuity_controls();
        let row = &mut packet.controls_rows[0];
        match mutate {
            0 => row.collapses_registry_source_class_across_public_mirrored_enterprise = true,
            1 => row.hides_replacement_path_or_lifecycle_state = true,
            2 => row.hides_publisher_transfer_or_continuity_language = true,
            _ => row.leaves_stale_evidence_certified_or_supported = true,
        }
        assert!(packet
            .validate()
            .contains(&M5CompatibilityContinuityControlsViolation::RowInvariantViolated));
    }
}

#[test]
fn replacement_continuity_honesty_not_proven_when_missing_example_removed() {
    let mut packet = seeded_m5_compatibility_continuity_controls();
    for row in &mut packet.controls_rows {
        row.compatibility_label_strip_examples.retain(|ex| {
            ex.degrade_reason
                != Some(M5CompatibilityLabelStripDegradeReason::ReplacementPathMissing)
        });
    }
    assert!(packet.validate().contains(
        &M5CompatibilityContinuityControlsViolation::ReplacementContinuityHonestyNotProven
    ));
}

#[test]
fn replacement_continuity_honesty_not_proven_when_no_clean_replacement_shown() {
    let mut packet = seeded_m5_compatibility_continuity_controls();
    // Drop every clean strip that shows a replacement path so the required coverage breaks.
    for row in &mut packet.controls_rows {
        row.compatibility_label_strip_examples.retain(|ex| {
            !(ex.is_clean()
                && ex.requires_replacement_path
                && !ex.replacement_path.trim().is_empty())
        });
    }
    assert!(packet.validate().contains(
        &M5CompatibilityContinuityControlsViolation::ReplacementContinuityHonestyNotProven
    ));
}

#[test]
fn stale_certified_overclaim_not_proven_when_stale_example_removed() {
    let mut packet = seeded_m5_compatibility_continuity_controls();
    for row in &mut packet.controls_rows {
        row.compatibility_label_strip_examples.retain(|ex| {
            ex.degrade_reason
                != Some(M5CompatibilityLabelStripDegradeReason::StaleEvidenceCertifiedOverclaim)
        });
    }
    assert!(packet
        .validate()
        .contains(&M5CompatibilityContinuityControlsViolation::StaleCertifiedOverclaimNotProven));
}

#[test]
fn stale_certified_overclaim_not_proven_when_row_stale_example_removed() {
    let mut packet = seeded_m5_compatibility_continuity_controls();
    for row in &mut packet.controls_rows {
        row.publisher_continuity_row_examples.retain(|ex| {
            ex.degrade_reason
                != Some(
                    M5PublisherContinuityRowDegradeReason::StaleOrUnverifiableCertifiedOverclaim,
                )
        });
    }
    assert!(packet
        .validate()
        .contains(&M5CompatibilityContinuityControlsViolation::StaleCertifiedOverclaimNotProven));
}

#[test]
fn governance_review_incomplete_fails() {
    let mut packet = seeded_m5_compatibility_continuity_controls();
    packet
        .governance_review
        .stale_evidence_never_leaves_certified_language = false;
    assert!(packet
        .validate()
        .contains(&M5CompatibilityContinuityControlsViolation::GovernanceReviewIncomplete));
}

#[test]
fn consumer_projection_incomplete_fails() {
    let mut packet = seeded_m5_compatibility_continuity_controls();
    packet
        .consumer_projection
        .support_export_reads_single_source = false;
    assert!(packet
        .validate()
        .contains(&M5CompatibilityContinuityControlsViolation::ConsumerProjectionIncomplete));
}

#[test]
fn proof_freshness_incomplete_fails() {
    let mut packet = seeded_m5_compatibility_continuity_controls();
    packet.proof_freshness.proof_freshness_slo_hours = 0;
    assert!(packet
        .validate()
        .contains(&M5CompatibilityContinuityControlsViolation::ProofFreshnessIncomplete));
}

#[test]
fn release_posture_incomplete_fails() {
    let mut packet = seeded_m5_compatibility_continuity_controls();
    packet.release_posture.accessibility_parity_required = false;
    assert!(packet
        .validate()
        .contains(&M5CompatibilityContinuityControlsViolation::ReleasePostureIncomplete));
}

#[test]
fn injected_raw_material_is_rejected() {
    let mut packet = seeded_m5_compatibility_continuity_controls();
    packet.controls_rows[0].scope_summary =
        "raw endpoint https://relay.internal.example/session leaked".to_owned();
    assert!(packet
        .validate()
        .contains(&M5CompatibilityContinuityControlsViolation::RawMaterialInExport));
}

#[test]
fn export_carries_no_forbidden_raw_material() {
    let json = seeded_m5_compatibility_continuity_controls().export_safe_json();
    let lower = json.to_lowercase();
    assert!(!lower.contains("password"));
    assert!(!lower.contains("passphrase"));
    assert!(!lower.contains("bearer "));
    assert!(!lower.contains("://"));
    assert!(!lower.contains("-----begin"));
}

#[test]
fn csv_has_a_row_per_consumer_surface() {
    let packet = seeded_m5_compatibility_continuity_controls();
    let csv = packet.render_matrix_csv();
    let lines: Vec<&str> = csv.lines().collect();
    assert_eq!(lines.len(), 1 + packet.controls_rows.len());
    assert!(lines[0].starts_with("consumer_surface,qualification,owner,"));
}

#[test]
fn markdown_summary_lists_every_consumer_surface() {
    let packet = seeded_m5_compatibility_continuity_controls();
    let summary = packet.render_markdown_summary();
    for row in &packet.controls_rows {
        assert!(summary.contains(row.consumer_surface.as_str()));
    }
}

#[test]
fn checked_support_export_validates_and_matches_seed() {
    let from_disk = current_stable_m5_compatibility_continuity_controls_export()
        .expect("checked M5 compatibility-continuity controls export validates");
    assert_eq!(
        from_disk.packet_id,
        M5_COMPATIBILITY_CONTINUITY_CONTROLS_PACKET_ID
    );
    assert_eq!(
        from_disk,
        seeded_m5_compatibility_continuity_controls(),
        "checked support export drifted from the seed builder"
    );
}

#[test]
fn narrowed_variants_validate_and_keep_rows_visible() {
    let beta = seeded_m5_compatibility_continuity_controls_marketplace_ui_beta_narrowed();
    assert!(beta.validate().is_empty(), "{:?}", beta.validate());
    assert_eq!(beta.controls_rows.len(), 5);
    let row = beta
        .controls_rows
        .iter()
        .find(|r| r.consumer_surface == M5MarketplaceInstallConsumerSurface::MarketplaceUi)
        .unwrap();
    assert_eq!(
        row.qualification,
        M5MarketplaceInstallQualificationClass::Beta
    );

    let preview = seeded_m5_compatibility_continuity_controls_registry_ui_preview_narrowed();
    assert!(preview.validate().is_empty(), "{:?}", preview.validate());
    assert_eq!(preview.controls_rows.len(), 5);
    let row = preview
        .controls_rows
        .iter()
        .find(|r| r.consumer_surface == M5MarketplaceInstallConsumerSurface::InstallReviewUi)
        .unwrap();
    assert_eq!(
        row.qualification,
        M5MarketplaceInstallQualificationClass::Preview
    );
}

#[test]
fn checked_narrowed_fixtures_validate_and_match_seed_builders() {
    let beta: M5CompatibilityContinuityControlsPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/ui/m5-compatibility-label-strip-publisher-continuity-row-controls/marketplace_ui_beta_narrowed.json"
    )))
    .expect("marketplace-ui fixture parses");
    assert!(beta.validate().is_empty());
    assert_eq!(
        beta,
        seeded_m5_compatibility_continuity_controls_marketplace_ui_beta_narrowed()
    );

    let preview: M5CompatibilityContinuityControlsPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/ui/m5-compatibility-label-strip-publisher-continuity-row-controls/registry_ui_preview_narrowed.json"
    )))
    .expect("install-review fixture parses");
    assert!(preview.validate().is_empty());
    assert_eq!(
        preview,
        seeded_m5_compatibility_continuity_controls_registry_ui_preview_narrowed()
    );
}

#[test]
fn implemented_families_are_the_two_lifecycle_components() {
    assert_eq!(
        IMPLEMENTED_FAMILIES,
        [
            M5MarketplaceInstallComponentFamily::CompatibilityLabelStrip,
            M5MarketplaceInstallComponentFamily::PublisherContinuityRow,
        ]
    );
}
