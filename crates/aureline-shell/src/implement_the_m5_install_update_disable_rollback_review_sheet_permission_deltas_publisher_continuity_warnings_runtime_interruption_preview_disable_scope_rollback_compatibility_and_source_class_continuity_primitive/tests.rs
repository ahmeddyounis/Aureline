use super::*;

fn clean_install_input() -> M5InstallReviewSheetResolutionInput {
    M5InstallReviewSheetResolutionInput {
        sheet_id: "review-sheet:test".to_owned(),
        artifact_identity: "test-artifact".to_owned(),
        mutation_flow: M5InstallReviewMutationFlow::Install,
        registry_source: M5RegistrySourceClass::PublicRegistry,
        compatibility: M5CompatibilityState::Compatible,
        permission_delta: M5InstallReviewPermissionDelta::NoChange,
        publisher_continuity: M5PublisherContinuityState::VerifiedPublisher,
        runtime_interruption: M5InstallReviewRuntimeInterruption::ReloadRequired,
        disable_scope: None,
        rollback_compatibility: None,
        review_actions: M5InstallReviewAction::ALL.to_vec(),
        certified_or_supported_claimed: true,
        evidence_fresh: true,
        reads_incompatible_as_ready: false,
        reads_transferred_as_continuous: false,
        collapses_source_class: false,
        reads_rollback_as_clean: false,
        proof_fresh: true,
    }
}

#[test]
fn seeded_controls_validates() {
    let packet = seeded_m5_install_review_sheet_controls();
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
    assert_eq!(packet.packet_id, M5_INSTALL_REVIEW_SHEET_CONTROLS_PACKET_ID);
}

#[test]
fn install_clean_names_facts_and_is_legible() {
    let resolved = resolve_install_review_sheet(clean_install_input()).unwrap();
    assert!(resolved.is_clean());
    assert!(resolved.fully_legible);
    assert!(resolved.has_transaction_grammar);
    assert_eq!(resolved.mutation_flow, "install");
    assert_eq!(resolved.registry_source, "public_registry");
    assert!(!resolved.presents_incompatible_as_ready);
    assert!(!resolved.collapses_source_class);
    assert_eq!(
        resolved.next_action,
        M5InstallReviewNextAction::NoActionNeeded
    );
}

#[test]
fn disable_clean_names_scope() {
    let mut input = clean_install_input();
    input.certified_or_supported_claimed = false;
    input.mutation_flow = M5InstallReviewMutationFlow::Disable;
    input.disable_scope = Some(M5DisableScopeClass::DisableWorkspace);
    let resolved = resolve_install_review_sheet(input).unwrap();
    assert!(resolved.is_clean());
    assert!(resolved.reviews_disable_scope);
    assert!(resolved.names_disable_scope);
    assert_eq!(resolved.disable_scope, Some("disable_workspace".to_owned()));
}

#[test]
fn rollback_clean_discloses_data_loss() {
    let mut input = clean_install_input();
    input.certified_or_supported_claimed = false;
    input.mutation_flow = M5InstallReviewMutationFlow::Rollback;
    input.rollback_compatibility = Some(M5RollbackCompatibilityState::RollbackDataLoss);
    let resolved = resolve_install_review_sheet(input).unwrap();
    assert!(resolved.is_clean());
    assert!(resolved.reviews_rollback);
    assert!(resolved.names_rollback_compatibility);
    assert!(!resolved.hides_rollback_incompatibility);
    assert_eq!(
        resolved.rollback_compatibility,
        Some("rollback_data_loss".to_owned())
    );
}

#[test]
fn identity_unstated_degrades() {
    let mut input = clean_install_input();
    input.artifact_identity = "   ".to_owned();
    let resolved = resolve_install_review_sheet(input).unwrap();
    assert_eq!(
        resolved.degrade_reason,
        Some(M5InstallReviewSheetDegradeReason::ArtifactIdentityUnstated)
    );
    assert_eq!(
        resolved.next_action,
        M5InstallReviewNextAction::ReviewTransaction
    );
}

#[test]
fn source_unresolved_degrades() {
    let mut input = clean_install_input();
    input.certified_or_supported_claimed = false;
    input.registry_source = M5RegistrySourceClass::SourceUnknown;
    let resolved = resolve_install_review_sheet(input).unwrap();
    assert_eq!(
        resolved.degrade_reason,
        Some(M5InstallReviewSheetDegradeReason::RegistrySourceUnresolved)
    );
    assert_eq!(
        resolved.next_action,
        M5InstallReviewNextAction::ReviewSourceContinuity
    );
}

#[test]
fn source_collapsed_degrades() {
    let mut input = clean_install_input();
    input.certified_or_supported_claimed = false;
    input.collapses_source_class = true;
    let resolved = resolve_install_review_sheet(input).unwrap();
    assert!(resolved.collapses_source_class);
    assert_eq!(
        resolved.degrade_reason,
        Some(M5InstallReviewSheetDegradeReason::RegistrySourceClassCollapsed)
    );
}

#[test]
fn grammar_incomplete_degrades() {
    let mut input = clean_install_input();
    input.certified_or_supported_claimed = false;
    input.review_actions = vec![
        M5InstallReviewAction::ReviewTransaction,
        M5InstallReviewAction::ConfirmMutation,
    ];
    let resolved = resolve_install_review_sheet(input).unwrap();
    assert!(!resolved.has_transaction_grammar);
    assert_eq!(
        resolved.degrade_reason,
        Some(M5InstallReviewSheetDegradeReason::TransactionGrammarIncomplete)
    );
}

#[test]
fn permission_delta_unverified_degrades() {
    let mut input = clean_install_input();
    input.certified_or_supported_claimed = false;
    input.permission_delta = M5InstallReviewPermissionDelta::DeltaUnknown;
    let resolved = resolve_install_review_sheet(input).unwrap();
    assert_eq!(
        resolved.degrade_reason,
        Some(M5InstallReviewSheetDegradeReason::PermissionDeltaUnverified)
    );
    assert_eq!(
        resolved.next_action,
        M5InstallReviewNextAction::ReviewPermissionDelta
    );
}

#[test]
fn incompatible_shown_ready_degrades() {
    let mut input = clean_install_input();
    input.certified_or_supported_claimed = false;
    input.compatibility = M5CompatibilityState::Incompatible;
    input.reads_incompatible_as_ready = true;
    let resolved = resolve_install_review_sheet(input).unwrap();
    assert!(resolved.presents_incompatible_as_ready);
    assert_eq!(
        resolved.degrade_reason,
        Some(M5InstallReviewSheetDegradeReason::IncompatibleShownReady)
    );
    assert_eq!(
        resolved.next_action,
        M5InstallReviewNextAction::ReviewRuntimeInterruption
    );
}

#[test]
fn publisher_transfer_hidden_degrades() {
    let mut input = clean_install_input();
    input.certified_or_supported_claimed = false;
    input.publisher_continuity = M5PublisherContinuityState::Transferred;
    input.reads_transferred_as_continuous = true;
    let resolved = resolve_install_review_sheet(input).unwrap();
    assert!(resolved.hides_publisher_transfer);
    assert_eq!(
        resolved.degrade_reason,
        Some(M5InstallReviewSheetDegradeReason::PublisherContinuityWarningMissing)
    );
}

#[test]
fn runtime_interruption_unresolved_degrades() {
    let mut input = clean_install_input();
    input.certified_or_supported_claimed = false;
    input.runtime_interruption = M5InstallReviewRuntimeInterruption::InterruptionUnknown;
    let resolved = resolve_install_review_sheet(input).unwrap();
    assert_eq!(
        resolved.degrade_reason,
        Some(M5InstallReviewSheetDegradeReason::RuntimeInterruptionUnresolved)
    );
    assert_eq!(
        resolved.next_action,
        M5InstallReviewNextAction::ReviewRuntimeInterruption
    );
}

#[test]
fn disable_scope_unstated_degrades() {
    let mut input = clean_install_input();
    input.certified_or_supported_claimed = false;
    input.mutation_flow = M5InstallReviewMutationFlow::Disable;
    input.disable_scope = None;
    let resolved = resolve_install_review_sheet(input).unwrap();
    assert!(resolved.hides_disable_scope);
    assert_eq!(
        resolved.degrade_reason,
        Some(M5InstallReviewSheetDegradeReason::DisableScopeUnstated)
    );
    assert_eq!(
        resolved.next_action,
        M5InstallReviewNextAction::ReviewDisableOrRollbackScope
    );
}

#[test]
fn disable_scope_unknown_degrades() {
    let mut input = clean_install_input();
    input.certified_or_supported_claimed = false;
    input.mutation_flow = M5InstallReviewMutationFlow::Disable;
    input.disable_scope = Some(M5DisableScopeClass::ScopeUnknown);
    let resolved = resolve_install_review_sheet(input).unwrap();
    assert!(resolved.hides_disable_scope);
    assert_eq!(
        resolved.degrade_reason,
        Some(M5InstallReviewSheetDegradeReason::DisableScopeUnstated)
    );
}

#[test]
fn rollback_compat_unresolved_degrades() {
    let mut input = clean_install_input();
    input.certified_or_supported_claimed = false;
    input.mutation_flow = M5InstallReviewMutationFlow::Rollback;
    input.rollback_compatibility = None;
    let resolved = resolve_install_review_sheet(input).unwrap();
    assert_eq!(
        resolved.degrade_reason,
        Some(M5InstallReviewSheetDegradeReason::RollbackCompatibilityUnresolved)
    );
}

#[test]
fn rollback_incompatibility_hidden_degrades() {
    let mut input = clean_install_input();
    input.certified_or_supported_claimed = false;
    input.mutation_flow = M5InstallReviewMutationFlow::Rollback;
    input.rollback_compatibility = Some(M5RollbackCompatibilityState::RollbackIncompatible);
    input.reads_rollback_as_clean = true;
    let resolved = resolve_install_review_sheet(input).unwrap();
    assert!(resolved.hides_rollback_incompatibility);
    assert_eq!(
        resolved.degrade_reason,
        Some(M5InstallReviewSheetDegradeReason::RollbackIncompatibilityHidden)
    );
}

#[test]
fn stale_certified_overclaim_degrades() {
    let mut input = clean_install_input();
    input.certified_or_supported_claimed = true;
    input.evidence_fresh = false;
    let resolved = resolve_install_review_sheet(input).unwrap();
    assert!(resolved.leaves_stale_certified_overclaim);
    assert_eq!(
        resolved.degrade_reason,
        Some(M5InstallReviewSheetDegradeReason::StaleEvidenceCertifiedOverclaim)
    );
    assert_eq!(
        resolved.next_action,
        M5InstallReviewNextAction::ReviewEvidenceFreshness
    );
}

#[test]
fn empty_id_and_forbidden_material_error() {
    let mut input = clean_install_input();
    input.sheet_id = "  ".to_owned();
    assert_eq!(
        resolve_install_review_sheet(input).unwrap_err(),
        M5InstallReviewSheetResolutionError::EmptySheetId
    );

    let mut input = clean_install_input();
    input.artifact_identity = "https://relay.internal/leak".to_owned();
    assert_eq!(
        resolve_install_review_sheet(input).unwrap_err(),
        M5InstallReviewSheetResolutionError::ForbiddenMaterial
    );
}

#[test]
fn vocabulary_set_is_canonical() {
    assert!(seeded_m5_install_review_sheet_controls()
        .vocabulary_set
        .matches_canonical());
}

#[test]
fn vocabulary_set_drift_fails() {
    let mut packet = seeded_m5_install_review_sheet_controls();
    packet.vocabulary_set.mutation_flows.pop();
    assert!(packet
        .validate()
        .contains(&M5InstallReviewSheetControlsViolation::VocabularySetDrift));
}

#[test]
fn missing_source_contracts_fails() {
    let mut packet = seeded_m5_install_review_sheet_controls();
    packet.source_contract_refs.clear();
    assert!(packet
        .validate()
        .contains(&M5InstallReviewSheetControlsViolation::MissingSourceContracts));
}

#[test]
fn component_schema_ref_missing_fails() {
    let mut packet = seeded_m5_install_review_sheet_controls();
    packet.controls_rows[0]
        .source_contract_refs
        .retain(|r| r != M5_INSTALL_UPDATE_DISABLE_ROLLBACK_REVIEW_SHEET_SCHEMA_REF);
    assert!(packet
        .validate()
        .contains(&M5InstallReviewSheetControlsViolation::ComponentSchemaRefMissing));
}

#[test]
fn mandatory_anatomy_missing_fails() {
    let mut packet = seeded_m5_install_review_sheet_controls();
    packet.controls_rows[0]
        .anatomy_parts
        .retain(|p| *p != M5InstallReviewAnatomyPart::Identity);
    assert!(packet
        .validate()
        .contains(&M5InstallReviewSheetControlsViolation::MandatoryAnatomyMissing));
}

#[test]
fn mandatory_export_field_missing_fails() {
    let mut packet = seeded_m5_install_review_sheet_controls();
    packet.controls_rows[0]
        .export_fields
        .retain(|f| *f != M5InstallReviewExportField::Dispositions);
    assert!(packet
        .validate()
        .contains(&M5InstallReviewSheetControlsViolation::MandatoryExportFieldMissing));
}

#[test]
fn examples_missing_fails() {
    let mut packet = seeded_m5_install_review_sheet_controls();
    packet.controls_rows[0].review_sheet_examples.clear();
    assert!(packet
        .validate()
        .contains(&M5InstallReviewSheetControlsViolation::ExamplesMissing));
}

#[test]
fn dishonest_clean_example_fails() {
    let mut packet = seeded_m5_install_review_sheet_controls();
    let row = &mut packet.controls_rows[0];
    row.review_sheet_examples[0].degrade_reason = None;
    row.review_sheet_examples[0].hides_publisher_transfer = true;
    assert!(packet
        .validate()
        .contains(&M5InstallReviewSheetControlsViolation::DishonestExample));
}

#[test]
fn row_invariant_violation_fails() {
    for mutate in 0u8..4 {
        let mut packet = seeded_m5_install_review_sheet_controls();
        let row = &mut packet.controls_rows[0];
        match mutate {
            0 => row.hides_permission_delta_or_runtime_interruption = true,
            1 => row.hides_publisher_transfer_disable_scope_or_rollback_incompatibility = true,
            2 => row.collapses_registry_source_class_across_public_mirrored_enterprise = true,
            _ => row.presents_incompatible_or_over_budget_as_ready = true,
        }
        assert!(packet
            .validate()
            .contains(&M5InstallReviewSheetControlsViolation::RowInvariantViolated));
    }
}

#[test]
fn transaction_grammar_not_proven_when_incomplete_sheet_removed() {
    let mut packet = seeded_m5_install_review_sheet_controls();
    for row in &mut packet.controls_rows {
        row.review_sheet_examples.retain(|ex| {
            ex.degrade_reason
                != Some(M5InstallReviewSheetDegradeReason::TransactionGrammarIncomplete)
        });
    }
    assert!(packet
        .validate()
        .contains(&M5InstallReviewSheetControlsViolation::TransactionGrammarNotProven));
}

#[test]
fn transaction_grammar_not_proven_when_a_flow_has_no_clean_sheet() {
    let mut packet = seeded_m5_install_review_sheet_controls();
    for row in &mut packet.controls_rows {
        row.review_sheet_examples
            .retain(|ex| !(ex.is_clean() && ex.mutation_flow == "disable"));
    }
    assert!(packet
        .validate()
        .contains(&M5InstallReviewSheetControlsViolation::TransactionGrammarNotProven));
}

#[test]
fn disable_scope_and_rollback_truth_not_proven_when_disable_unstated_removed() {
    let mut packet = seeded_m5_install_review_sheet_controls();
    for row in &mut packet.controls_rows {
        row.review_sheet_examples.retain(|ex| {
            ex.degrade_reason != Some(M5InstallReviewSheetDegradeReason::DisableScopeUnstated)
        });
    }
    assert!(packet
        .validate()
        .contains(&M5InstallReviewSheetControlsViolation::DisableScopeAndRollbackTruthNotProven));
}

#[test]
fn disable_scope_and_rollback_truth_not_proven_when_clean_disable_removed() {
    let mut packet = seeded_m5_install_review_sheet_controls();
    for row in &mut packet.controls_rows {
        row.review_sheet_examples
            .retain(|ex| !(ex.is_clean() && ex.reviews_disable_scope && ex.names_disable_scope));
    }
    assert!(packet
        .validate()
        .contains(&M5InstallReviewSheetControlsViolation::DisableScopeAndRollbackTruthNotProven));
}

#[test]
fn source_continuity_not_proven_when_source_collapsed_removed() {
    let mut packet = seeded_m5_install_review_sheet_controls();
    for row in &mut packet.controls_rows {
        row.review_sheet_examples.retain(|ex| {
            ex.degrade_reason
                != Some(M5InstallReviewSheetDegradeReason::RegistrySourceClassCollapsed)
        });
    }
    assert!(packet
        .validate()
        .contains(&M5InstallReviewSheetControlsViolation::SourceContinuityNotProven));
}

#[test]
fn source_continuity_not_proven_when_transfer_hidden_removed() {
    let mut packet = seeded_m5_install_review_sheet_controls();
    for row in &mut packet.controls_rows {
        row.review_sheet_examples.retain(|ex| {
            ex.degrade_reason
                != Some(M5InstallReviewSheetDegradeReason::PublisherContinuityWarningMissing)
        });
    }
    assert!(packet
        .validate()
        .contains(&M5InstallReviewSheetControlsViolation::SourceContinuityNotProven));
}

#[test]
fn governance_review_incomplete_fails() {
    let mut packet = seeded_m5_install_review_sheet_controls();
    packet.governance_review.disable_scope_always_explicit = false;
    assert!(packet
        .validate()
        .contains(&M5InstallReviewSheetControlsViolation::GovernanceReviewIncomplete));
}

#[test]
fn consumer_projection_incomplete_fails() {
    let mut packet = seeded_m5_install_review_sheet_controls();
    packet
        .consumer_projection
        .source_continuity_carried_into_handoff = false;
    assert!(packet
        .validate()
        .contains(&M5InstallReviewSheetControlsViolation::ConsumerProjectionIncomplete));
}

#[test]
fn proof_freshness_incomplete_fails() {
    let mut packet = seeded_m5_install_review_sheet_controls();
    packet.proof_freshness.proof_freshness_slo_hours = 0;
    assert!(packet
        .validate()
        .contains(&M5InstallReviewSheetControlsViolation::ProofFreshnessIncomplete));
}

#[test]
fn release_posture_incomplete_fails() {
    let mut packet = seeded_m5_install_review_sheet_controls();
    packet.release_posture.accessibility_parity_required = false;
    assert!(packet
        .validate()
        .contains(&M5InstallReviewSheetControlsViolation::ReleasePostureIncomplete));
}

#[test]
fn injected_raw_material_is_rejected() {
    let mut packet = seeded_m5_install_review_sheet_controls();
    packet.controls_rows[0].scope_summary =
        "raw endpoint https://relay.internal.example/session leaked".to_owned();
    assert!(packet
        .validate()
        .contains(&M5InstallReviewSheetControlsViolation::RawMaterialInExport));
}

#[test]
fn export_carries_no_forbidden_raw_material() {
    let json = seeded_m5_install_review_sheet_controls().export_safe_json();
    let lower = json.to_lowercase();
    assert!(!lower.contains("password"));
    assert!(!lower.contains("passphrase"));
    assert!(!lower.contains("bearer "));
    assert!(!lower.contains("://"));
    assert!(!lower.contains("-----begin"));
}

#[test]
fn csv_has_a_row_per_consumer_surface() {
    let packet = seeded_m5_install_review_sheet_controls();
    let csv = packet.render_matrix_csv();
    let lines: Vec<&str> = csv.lines().collect();
    assert_eq!(lines.len(), 1 + packet.controls_rows.len());
    assert!(lines[0].starts_with("consumer_surface,qualification,owner,"));
}

#[test]
fn markdown_summary_lists_every_consumer_surface() {
    let packet = seeded_m5_install_review_sheet_controls();
    let summary = packet.render_markdown_summary();
    for row in &packet.controls_rows {
        assert!(summary.contains(row.consumer_surface.as_str()));
    }
}

#[test]
fn checked_support_export_validates_and_matches_seed() {
    let from_disk = current_stable_m5_install_review_sheet_controls_export()
        .expect("checked M5 install-review-sheet controls export validates");
    assert_eq!(
        from_disk.packet_id,
        M5_INSTALL_REVIEW_SHEET_CONTROLS_PACKET_ID
    );
    assert_eq!(
        from_disk,
        seeded_m5_install_review_sheet_controls(),
        "checked support export drifted from the seed builder"
    );
}

#[test]
fn narrowed_variants_validate_and_keep_rows_visible() {
    let beta = seeded_m5_install_review_sheet_controls_install_review_ui_beta_narrowed();
    assert!(beta.validate().is_empty(), "{:?}", beta.validate());
    assert_eq!(beta.controls_rows.len(), 5);
    let row = beta
        .controls_rows
        .iter()
        .find(|r| r.consumer_surface == M5MarketplaceInstallConsumerSurface::InstallReviewUi)
        .unwrap();
    assert_eq!(
        row.qualification,
        M5MarketplaceInstallQualificationClass::Beta
    );

    let preview = seeded_m5_install_review_sheet_controls_marketplace_ui_preview_narrowed();
    assert!(preview.validate().is_empty(), "{:?}", preview.validate());
    assert_eq!(preview.controls_rows.len(), 5);
    let row = preview
        .controls_rows
        .iter()
        .find(|r| r.consumer_surface == M5MarketplaceInstallConsumerSurface::MarketplaceUi)
        .unwrap();
    assert_eq!(
        row.qualification,
        M5MarketplaceInstallQualificationClass::Preview
    );
}

#[test]
fn checked_narrowed_fixtures_validate_and_match_seed_builders() {
    let beta: M5InstallReviewSheetControlsPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/ui/m5-install-update-disable-rollback-review-sheet-controls/install_review_ui_beta_narrowed.json"
    )))
    .expect("install-review fixture parses");
    assert!(beta.validate().is_empty());
    assert_eq!(
        beta,
        seeded_m5_install_review_sheet_controls_install_review_ui_beta_narrowed()
    );

    let preview: M5InstallReviewSheetControlsPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/ui/m5-install-update-disable-rollback-review-sheet-controls/marketplace_ui_preview_narrowed.json"
    )))
    .expect("marketplace fixture parses");
    assert!(preview.validate().is_empty());
    assert_eq!(
        preview,
        seeded_m5_install_review_sheet_controls_marketplace_ui_preview_narrowed()
    );
}

#[test]
fn implemented_families_is_the_review_sheet() {
    assert_eq!(
        IMPLEMENTED_FAMILIES,
        [M5MarketplaceInstallComponentFamily::InstallUpdateDisableRollbackReviewSheet]
    );
}
