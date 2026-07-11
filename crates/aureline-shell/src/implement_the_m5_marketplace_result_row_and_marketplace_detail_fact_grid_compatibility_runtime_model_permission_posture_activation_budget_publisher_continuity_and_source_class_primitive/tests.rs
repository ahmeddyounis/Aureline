use super::*;

fn clean_row_input() -> M5MarketplaceResultRowResolutionInput {
    M5MarketplaceResultRowResolutionInput {
        row_id: "result-row:test".to_owned(),
        artifact_identity: "test-artifact".to_owned(),
        registry_source: M5RegistrySourceClass::PublicRegistry,
        compatibility: M5CompatibilityState::Compatible,
        host_runtime_model: M5HostRuntimeModel::Sandboxed,
        permission_posture: M5PermissionPostureState::Minimal,
        permission_widening_stated: true,
        activation_budget: M5ActivationBudgetBandState::WithinBudget,
        activation_cost_stated: true,
        trust_tier: M5MarketplaceTrustTier::Reviewed,
        publisher_continuity: M5PublisherContinuityState::Continuous,
        publisher_change_stated: true,
        collapses_source_class: false,
        reads_incompatible_or_over_budget_as_ready: false,
        detail_command_available: true,
        proof_fresh: true,
    }
}

fn clean_grid_input() -> M5MarketplaceDetailFactGridResolutionInput {
    M5MarketplaceDetailFactGridResolutionInput {
        grid_id: "detail-grid:test".to_owned(),
        artifact_identity: "test-artifact".to_owned(),
        registry_source: M5RegistrySourceClass::PublicRegistry,
        compatibility: M5CompatibilityState::Compatible,
        host_runtime_model: M5HostRuntimeModel::Sandboxed,
        permission_posture: M5PermissionPostureState::Minimal,
        permission_widening_stated: true,
        activation_budget: M5ActivationBudgetBandState::WithinBudget,
        activation_cost_stated: true,
        trust_tier: M5MarketplaceTrustTier::Reviewed,
        publisher_continuity: M5PublisherContinuityState::Continuous,
        publisher_change_stated: true,
        version_range: ">=1.0.0, <2.0.0".to_owned(),
        lifecycle: M5MarketplaceLifecycleState::Active,
        docs_linked: true,
        changelog_linked: true,
        open_issues_linked: true,
        collapses_source_class: false,
        reads_incompatible_or_over_budget_as_ready: false,
        proof_fresh: true,
    }
}

#[test]
fn seeded_controls_validates() {
    let packet = seeded_m5_marketplace_result_detail_controls();
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
    assert_eq!(
        packet.packet_id,
        M5_MARKETPLACE_RESULT_DETAIL_CONTROLS_PACKET_ID
    );
}

#[test]
fn row_clean_names_source_and_is_comparable() {
    let resolved = resolve_marketplace_result_row(clean_row_input()).unwrap();
    assert!(resolved.is_clean());
    assert!(resolved.comparable_at_a_glance);
    assert!(!resolved.collapses_source_class);
    assert!(!resolved.presents_incompatible_or_over_budget_as_ready);
    assert!(resolved.is_installable);
    assert_eq!(resolved.registry_source, "public_registry");
    assert_eq!(
        resolved.source_disposition,
        Some(M5MarketplaceInstallDisposition::Public)
    );
    assert_eq!(
        resolved.next_action,
        M5MarketplaceResultDetailNextAction::OpenDetail
    );
}

#[test]
fn row_source_unknown_degrades_and_has_no_disposition() {
    let mut input = clean_row_input();
    input.registry_source = M5RegistrySourceClass::SourceUnknown;
    let resolved = resolve_marketplace_result_row(input).unwrap();
    assert_eq!(
        resolved.degrade_reason,
        Some(M5MarketplaceResultRowDegradeReason::RegistrySourceUnresolved)
    );
    assert_eq!(resolved.source_disposition, None);
}

#[test]
fn row_source_collapsed_degrades() {
    let mut input = clean_row_input();
    input.collapses_source_class = true;
    let resolved = resolve_marketplace_result_row(input).unwrap();
    assert!(!resolved.is_clean());
    assert!(resolved.collapses_source_class);
    assert_eq!(
        resolved.degrade_reason,
        Some(M5MarketplaceResultRowDegradeReason::SourceClassCollapsedIntoAmbiguousOrigin)
    );
}

#[test]
fn row_incompatible_shown_ready_degrades() {
    let mut input = clean_row_input();
    input.compatibility = M5CompatibilityState::Incompatible;
    input.reads_incompatible_or_over_budget_as_ready = true;
    let resolved = resolve_marketplace_result_row(input).unwrap();
    assert!(resolved.presents_incompatible_or_over_budget_as_ready);
    assert!(!resolved.is_installable);
    assert_eq!(
        resolved.degrade_reason,
        Some(M5MarketplaceResultRowDegradeReason::IncompatibleOrOverBudgetShownAsReady)
    );
    assert_eq!(
        resolved.next_action,
        M5MarketplaceResultDetailNextAction::ReviewCompatibility
    );
}

#[test]
fn row_permission_widening_hidden_degrades() {
    let mut input = clean_row_input();
    input.permission_posture = M5PermissionPostureState::WidenedTransitive;
    input.permission_widening_stated = false;
    let resolved = resolve_marketplace_result_row(input).unwrap();
    assert!(resolved.permission_widened);
    assert_eq!(
        resolved.degrade_reason,
        Some(M5MarketplaceResultRowDegradeReason::PermissionWideningHidden)
    );
    assert_eq!(
        resolved.next_action,
        M5MarketplaceResultDetailNextAction::ReviewPermissionAndBudget
    );
}

#[test]
fn row_activation_cost_hidden_degrades() {
    let mut input = clean_row_input();
    input.activation_budget = M5ActivationBudgetBandState::OverBudget;
    input.activation_cost_stated = false;
    let resolved = resolve_marketplace_result_row(input).unwrap();
    assert!(!resolved.within_activation_budget);
    assert_eq!(
        resolved.degrade_reason,
        Some(M5MarketplaceResultRowDegradeReason::ActivationCostHidden)
    );
}

#[test]
fn row_support_class_unresolved_degrades() {
    let mut input = clean_row_input();
    input.trust_tier = M5MarketplaceTrustTier::TierUnknown;
    let resolved = resolve_marketplace_result_row(input).unwrap();
    assert_eq!(
        resolved.degrade_reason,
        Some(M5MarketplaceResultRowDegradeReason::SupportClassUnresolved)
    );
}

#[test]
fn row_publisher_transfer_hidden_degrades() {
    let mut input = clean_row_input();
    input.publisher_continuity = M5PublisherContinuityState::Transferred;
    input.publisher_change_stated = false;
    let resolved = resolve_marketplace_result_row(input).unwrap();
    assert!(resolved.publisher_changed);
    assert_eq!(
        resolved.degrade_reason,
        Some(M5MarketplaceResultRowDegradeReason::PublisherTransferHidden)
    );
}

#[test]
fn row_detail_missing_degrades() {
    let mut input = clean_row_input();
    input.detail_command_available = false;
    let resolved = resolve_marketplace_result_row(input).unwrap();
    assert_eq!(
        resolved.degrade_reason,
        Some(M5MarketplaceResultRowDegradeReason::DetailPathMissing)
    );
}

#[test]
fn row_identity_unstated_degrades() {
    let mut input = clean_row_input();
    input.artifact_identity = "  ".to_owned();
    let resolved = resolve_marketplace_result_row(input).unwrap();
    assert_eq!(
        resolved.degrade_reason,
        Some(M5MarketplaceResultRowDegradeReason::ArtifactIdentityUnstated)
    );
}

#[test]
fn row_empty_id_and_forbidden_material_error() {
    let mut input = clean_row_input();
    input.row_id = "".to_owned();
    assert_eq!(
        resolve_marketplace_result_row(input).unwrap_err(),
        M5MarketplaceResultDetailResolutionError::EmptyRowId
    );

    let mut input = clean_row_input();
    input.artifact_identity = "https://relay.internal/leak".to_owned();
    assert_eq!(
        resolve_marketplace_result_row(input).unwrap_err(),
        M5MarketplaceResultDetailResolutionError::ForbiddenMaterial
    );
}

#[test]
fn grid_clean_names_richer_facts() {
    let resolved = resolve_marketplace_detail_fact_grid(clean_grid_input()).unwrap();
    assert!(resolved.is_clean());
    assert!(resolved.fully_legible);
    assert_eq!(resolved.version_range, ">=1.0.0, <2.0.0");
    assert_eq!(resolved.lifecycle, "active");
    assert_eq!(
        resolved.source_disposition,
        Some(M5MarketplaceInstallDisposition::Public)
    );
}

#[test]
fn grid_version_range_unstated_degrades() {
    let mut input = clean_grid_input();
    input.version_range = "".to_owned();
    let resolved = resolve_marketplace_detail_fact_grid(input).unwrap();
    assert_eq!(
        resolved.degrade_reason,
        Some(M5MarketplaceDetailFactGridDegradeReason::VersionRangeUnstated)
    );
}

#[test]
fn grid_lifecycle_unstated_degrades() {
    let mut input = clean_grid_input();
    input.lifecycle = M5MarketplaceLifecycleState::LifecycleUnknown;
    let resolved = resolve_marketplace_detail_fact_grid(input).unwrap();
    assert_eq!(
        resolved.degrade_reason,
        Some(M5MarketplaceDetailFactGridDegradeReason::LifecycleStateUnstated)
    );
}

#[test]
fn grid_docs_unlinked_degrades() {
    let mut input = clean_grid_input();
    input.docs_linked = false;
    input.changelog_linked = false;
    input.open_issues_linked = false;
    let resolved = resolve_marketplace_detail_fact_grid(input).unwrap();
    assert_eq!(
        resolved.degrade_reason,
        Some(M5MarketplaceDetailFactGridDegradeReason::DocsChangelogIssuesUnlinked)
    );
}

#[test]
fn grid_empty_id_and_forbidden_material_error() {
    let mut input = clean_grid_input();
    input.grid_id = "   ".to_owned();
    assert_eq!(
        resolve_marketplace_detail_fact_grid(input).unwrap_err(),
        M5MarketplaceResultDetailResolutionError::EmptyGridId
    );

    let mut input = clean_grid_input();
    input.version_range = "see internal://notes".to_owned();
    assert_eq!(
        resolve_marketplace_detail_fact_grid(input).unwrap_err(),
        M5MarketplaceResultDetailResolutionError::ForbiddenMaterial
    );
}

#[test]
fn row_and_grid_share_facts_for_same_artifact() {
    let row = resolve_marketplace_result_row(clean_row_input()).unwrap();
    let grid = resolve_marketplace_detail_fact_grid(clean_grid_input()).unwrap();
    assert_eq!(row.artifact_identity, grid.artifact_identity);
    assert_eq!(row.registry_source, grid.registry_source);
    assert_eq!(row.compatibility, grid.compatibility);
    assert_eq!(row.permission_posture, grid.permission_posture);
    assert_eq!(row.publisher_continuity, grid.publisher_continuity);
    assert_eq!(row.trust_tier, grid.trust_tier);
}

#[test]
fn vocabulary_set_is_canonical() {
    assert!(seeded_m5_marketplace_result_detail_controls()
        .vocabulary_set
        .matches_canonical());
}

#[test]
fn vocabulary_set_drift_fails() {
    let mut packet = seeded_m5_marketplace_result_detail_controls();
    packet.vocabulary_set.registry_source_classes.pop();
    assert!(packet
        .validate()
        .contains(&M5MarketplaceResultDetailControlsViolation::VocabularySetDrift));
}

#[test]
fn missing_source_contracts_fails() {
    let mut packet = seeded_m5_marketplace_result_detail_controls();
    packet.source_contract_refs.clear();
    assert!(packet
        .validate()
        .contains(&M5MarketplaceResultDetailControlsViolation::MissingSourceContracts));
}

#[test]
fn component_schema_ref_missing_fails() {
    let mut packet = seeded_m5_marketplace_result_detail_controls();
    packet.controls_rows[0]
        .source_contract_refs
        .retain(|r| r != M5_MARKETPLACE_RESULT_ROW_SCHEMA_REF);
    assert!(packet
        .validate()
        .contains(&M5MarketplaceResultDetailControlsViolation::ComponentSchemaRefMissing));
}

#[test]
fn mandatory_anatomy_missing_fails() {
    let mut packet = seeded_m5_marketplace_result_detail_controls();
    packet.controls_rows[0]
        .anatomy_parts
        .retain(|p| *p != M5MarketplaceResultDetailAnatomyPart::Identity);
    assert!(packet
        .validate()
        .contains(&M5MarketplaceResultDetailControlsViolation::MandatoryAnatomyMissing));
}

#[test]
fn mandatory_export_field_missing_fails() {
    let mut packet = seeded_m5_marketplace_result_detail_controls();
    packet.controls_rows[0]
        .export_fields
        .retain(|f| *f != M5MarketplaceResultDetailExportField::Dispositions);
    assert!(packet
        .validate()
        .contains(&M5MarketplaceResultDetailControlsViolation::MandatoryExportFieldMissing));
}

#[test]
fn examples_missing_fails() {
    let mut packet = seeded_m5_marketplace_result_detail_controls();
    packet.controls_rows[0]
        .marketplace_detail_fact_grid_examples
        .clear();
    assert!(packet
        .validate()
        .contains(&M5MarketplaceResultDetailControlsViolation::ExamplesMissing));
}

#[test]
fn dishonest_clean_example_fails() {
    let mut packet = seeded_m5_marketplace_result_detail_controls();
    // Force a clean row to also read as collapsing source class — the packet must reject it.
    let row = &mut packet.controls_rows[0];
    row.marketplace_result_row_examples[0].degrade_reason = None;
    row.marketplace_result_row_examples[0].collapses_source_class = true;
    assert!(packet
        .validate()
        .contains(&M5MarketplaceResultDetailControlsViolation::DishonestExample));
}

#[test]
fn row_invariant_violation_fails() {
    for mutate in 0u8..4 {
        let mut packet = seeded_m5_marketplace_result_detail_controls();
        let row = &mut packet.controls_rows[0];
        match mutate {
            0 => row.collapses_registry_source_class_across_public_mirrored_enterprise = true,
            1 => row.hides_permission_widening_or_activation_cost = true,
            2 => row.hides_publisher_transfer_or_deprecation = true,
            _ => row.presents_incompatible_or_over_budget_as_ready = true,
        }
        assert!(packet
            .validate()
            .contains(&M5MarketplaceResultDetailControlsViolation::RowInvariantViolated));
    }
}

#[test]
fn source_class_honesty_not_proven_when_collapse_example_removed() {
    let mut packet = seeded_m5_marketplace_result_detail_controls();
    for row in &mut packet.controls_rows {
        row.marketplace_result_row_examples.retain(|ex| {
            ex.degrade_reason
                != Some(
                    M5MarketplaceResultRowDegradeReason::SourceClassCollapsedIntoAmbiguousOrigin,
                )
        });
        row.marketplace_detail_fact_grid_examples.retain(|ex| {
            ex.degrade_reason
                != Some(
                    M5MarketplaceDetailFactGridDegradeReason::SourceClassCollapsedIntoAmbiguousOrigin,
                )
        });
    }
    assert!(packet
        .validate()
        .contains(&M5MarketplaceResultDetailControlsViolation::SourceClassHonestyNotProven));
}

#[test]
fn source_class_honesty_not_proven_when_source_uncovered() {
    let mut packet = seeded_m5_marketplace_result_detail_controls();
    // Drop every clean enterprise-source example so the required source coverage breaks.
    for row in &mut packet.controls_rows {
        row.marketplace_result_row_examples.retain(|ex| {
            !(ex.is_clean()
                && ex.source_disposition == Some(M5MarketplaceInstallDisposition::Enterprise))
        });
        row.marketplace_detail_fact_grid_examples.retain(|ex| {
            !(ex.is_clean()
                && ex.source_disposition == Some(M5MarketplaceInstallDisposition::Enterprise))
        });
    }
    assert!(packet
        .validate()
        .contains(&M5MarketplaceResultDetailControlsViolation::SourceClassHonestyNotProven));
}

#[test]
fn list_detail_parity_not_proven_when_grids_dropped() {
    let mut packet = seeded_m5_marketplace_result_detail_controls();
    // Keep grids present (schema requires >=1) but replace every clean grid identity so no clean
    // row/grid pair shares an artifact.
    for row in &mut packet.controls_rows {
        for grid in &mut row.marketplace_detail_fact_grid_examples {
            if grid.is_clean() {
                grid.artifact_identity = format!("orphan-{}", grid.grid_id);
            }
        }
    }
    assert!(packet
        .validate()
        .contains(&M5MarketplaceResultDetailControlsViolation::ListDetailParityNotProven));
}

#[test]
fn list_detail_parity_not_proven_when_facts_contradict() {
    let mut packet = seeded_m5_marketplace_result_detail_controls();
    // Make a shared-identity clean grid contradict its result row's compatibility.
    for row in &mut packet.controls_rows {
        for grid in &mut row.marketplace_detail_fact_grid_examples {
            if grid.is_clean() && grid.artifact_identity == "acme-linter" {
                grid.compatibility = M5CompatibilityState::Incompatible.as_str().to_owned();
            }
        }
    }
    assert!(packet
        .validate()
        .contains(&M5MarketplaceResultDetailControlsViolation::ListDetailParityNotProven));
}

#[test]
fn governance_review_incomplete_fails() {
    let mut packet = seeded_m5_marketplace_result_detail_controls();
    packet
        .governance_review
        .list_and_detail_share_one_fact_grammar = false;
    assert!(packet
        .validate()
        .contains(&M5MarketplaceResultDetailControlsViolation::GovernanceReviewIncomplete));
}

#[test]
fn consumer_projection_incomplete_fails() {
    let mut packet = seeded_m5_marketplace_result_detail_controls();
    packet
        .consumer_projection
        .support_export_reads_single_marketplace_source = false;
    assert!(packet
        .validate()
        .contains(&M5MarketplaceResultDetailControlsViolation::ConsumerProjectionIncomplete));
}

#[test]
fn proof_freshness_incomplete_fails() {
    let mut packet = seeded_m5_marketplace_result_detail_controls();
    packet.proof_freshness.proof_freshness_slo_hours = 0;
    assert!(packet
        .validate()
        .contains(&M5MarketplaceResultDetailControlsViolation::ProofFreshnessIncomplete));
}

#[test]
fn release_posture_incomplete_fails() {
    let mut packet = seeded_m5_marketplace_result_detail_controls();
    packet.release_posture.accessibility_parity_required = false;
    assert!(packet
        .validate()
        .contains(&M5MarketplaceResultDetailControlsViolation::ReleasePostureIncomplete));
}

#[test]
fn injected_raw_material_is_rejected() {
    let mut packet = seeded_m5_marketplace_result_detail_controls();
    packet.controls_rows[0].scope_summary =
        "raw endpoint https://relay.internal.example/session leaked".to_owned();
    assert!(packet
        .validate()
        .contains(&M5MarketplaceResultDetailControlsViolation::RawMaterialInExport));
}

#[test]
fn export_carries_no_forbidden_raw_material() {
    let json = seeded_m5_marketplace_result_detail_controls().export_safe_json();
    let lower = json.to_lowercase();
    assert!(!lower.contains("password"));
    assert!(!lower.contains("passphrase"));
    assert!(!lower.contains("bearer "));
    assert!(!lower.contains("://"));
    assert!(!lower.contains("-----begin"));
}

#[test]
fn csv_has_a_row_per_consumer_surface() {
    let packet = seeded_m5_marketplace_result_detail_controls();
    let csv = packet.render_matrix_csv();
    let lines: Vec<&str> = csv.lines().collect();
    assert_eq!(lines.len(), 1 + packet.controls_rows.len());
    assert!(lines[0].starts_with("consumer_surface,qualification,owner,"));
}

#[test]
fn markdown_summary_lists_every_consumer_surface() {
    let packet = seeded_m5_marketplace_result_detail_controls();
    let summary = packet.render_markdown_summary();
    for row in &packet.controls_rows {
        assert!(summary.contains(row.consumer_surface.as_str()));
    }
}

#[test]
fn checked_support_export_validates_and_matches_seed() {
    let from_disk = current_stable_m5_marketplace_result_detail_controls_export()
        .expect("checked M5 marketplace-result-detail controls export validates");
    assert_eq!(
        from_disk.packet_id,
        M5_MARKETPLACE_RESULT_DETAIL_CONTROLS_PACKET_ID
    );
    assert_eq!(
        from_disk,
        seeded_m5_marketplace_result_detail_controls(),
        "checked support export drifted from the seed builder"
    );
}

#[test]
fn narrowed_variants_validate_and_keep_rows_visible() {
    let beta = seeded_m5_marketplace_result_detail_controls_marketplace_ui_beta_narrowed();
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

    let preview = seeded_m5_marketplace_result_detail_controls_registry_ui_preview_narrowed();
    assert!(preview.validate().is_empty(), "{:?}", preview.validate());
    assert_eq!(preview.controls_rows.len(), 5);
    let row = preview
        .controls_rows
        .iter()
        .find(|r| r.consumer_surface == M5MarketplaceInstallConsumerSurface::RegistryUi)
        .unwrap();
    assert_eq!(
        row.qualification,
        M5MarketplaceInstallQualificationClass::Preview
    );
}

#[test]
fn checked_narrowed_fixtures_validate_and_match_seed_builders() {
    let beta: M5MarketplaceResultDetailControlsPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/ui/m5-marketplace-result-row-detail-fact-grid-controls/marketplace_ui_beta_narrowed.json"
    )))
    .expect("marketplace-ui fixture parses");
    assert!(beta.validate().is_empty());
    assert_eq!(
        beta,
        seeded_m5_marketplace_result_detail_controls_marketplace_ui_beta_narrowed()
    );

    let preview: M5MarketplaceResultDetailControlsPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/ui/m5-marketplace-result-row-detail-fact-grid-controls/registry_ui_preview_narrowed.json"
    )))
    .expect("registry-ui fixture parses");
    assert!(preview.validate().is_empty());
    assert_eq!(
        preview,
        seeded_m5_marketplace_result_detail_controls_registry_ui_preview_narrowed()
    );
}

#[test]
fn implemented_families_are_the_two_marketplace_components() {
    assert_eq!(
        IMPLEMENTED_FAMILIES,
        [
            M5MarketplaceInstallComponentFamily::MarketplaceResultRow,
            M5MarketplaceInstallComponentFamily::MarketplaceDetailFactGrid,
        ]
    );
}
