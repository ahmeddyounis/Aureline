use super::*;

fn clean_summary_input() -> M5PermissionManifestSummaryResolutionInput {
    M5PermissionManifestSummaryResolutionInput {
        summary_id: "perm-summary:test".to_owned(),
        artifact_identity: "test-artifact".to_owned(),
        posture: M5PermissionPostureState::Standard,
        host_runtime_model: M5HostRuntimeModel::Sandboxed,
        required_capabilities: vec!["read workspace files".to_owned()],
        optional_capabilities: vec!["write settings".to_owned()],
        inherited_capabilities: Vec::new(),
        data_boundary: "reads workspace files".to_owned(),
        network_boundary: "no network access".to_owned(),
        manifest_digest: "sha256-manifest-test".to_owned(),
        flattens_into_full_access: false,
        proof_fresh: true,
    }
}

fn clean_drawer_input() -> M5TransitiveCapabilityDrawerResolutionInput {
    M5TransitiveCapabilityDrawerResolutionInput {
        drawer_id: "transitive-drawer:test".to_owned(),
        artifact_identity: "test-artifact".to_owned(),
        posture: M5PermissionPostureState::WidenedTransitive,
        transitive_widening_disclosed: true,
        dependency_contributed_capabilities: vec!["network read".to_owned()],
        dependency_attributions: vec!["network read contributed by telemetry-sdk".to_owned()],
        manifest_digest: "sha256-manifest-test".to_owned(),
        flattens_into_full_access: false,
        proof_fresh: true,
    }
}

#[test]
fn seeded_controls_validates() {
    let packet = seeded_m5_permission_manifest_controls();
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
    assert_eq!(packet.packet_id, M5_PERMISSION_MANIFEST_CONTROLS_PACKET_ID);
}

#[test]
fn summary_clean_names_posture_classes_and_is_legible() {
    let resolved = resolve_permission_manifest_summary(clean_summary_input()).unwrap();
    assert!(resolved.is_clean());
    assert!(resolved.fully_legible);
    assert!(resolved.requests_capabilities);
    assert!(!resolved.widens_transitively);
    assert!(!resolved.flattens_into_full_access);
    assert_eq!(resolved.posture, "standard");
    assert_eq!(resolved.required_capabilities, vec!["read workspace files"]);
    assert_eq!(
        resolved.next_action,
        M5PermissionManifestNextAction::NoActionNeeded
    );
}

#[test]
fn summary_posture_unknown_degrades() {
    let mut input = clean_summary_input();
    input.posture = M5PermissionPostureState::PostureUnknown;
    let resolved = resolve_permission_manifest_summary(input).unwrap();
    assert_eq!(
        resolved.degrade_reason,
        Some(M5PermissionManifestSummaryDegradeReason::PermissionPostureUnresolved)
    );
    assert_eq!(
        resolved.next_action,
        M5PermissionManifestNextAction::ReviewPermissionPosture
    );
}

#[test]
fn summary_host_unknown_degrades() {
    let mut input = clean_summary_input();
    input.host_runtime_model = M5HostRuntimeModel::HostUnknown;
    let resolved = resolve_permission_manifest_summary(input).unwrap();
    assert_eq!(
        resolved.degrade_reason,
        Some(M5PermissionManifestSummaryDegradeReason::HostModelUnresolved)
    );
}

#[test]
fn summary_capability_grouping_unstated_degrades() {
    let mut input = clean_summary_input();
    input.required_capabilities = Vec::new();
    let resolved = resolve_permission_manifest_summary(input).unwrap();
    assert_eq!(
        resolved.degrade_reason,
        Some(M5PermissionManifestSummaryDegradeReason::CapabilityGroupingUnstated)
    );
    assert_eq!(
        resolved.next_action,
        M5PermissionManifestNextAction::ReviewCapabilityClasses
    );
}

#[test]
fn summary_boundary_unstated_degrades() {
    let mut input = clean_summary_input();
    input.network_boundary = "  ".to_owned();
    let resolved = resolve_permission_manifest_summary(input).unwrap();
    assert_eq!(
        resolved.degrade_reason,
        Some(M5PermissionManifestSummaryDegradeReason::DataNetworkBoundaryUnstated)
    );
    assert_eq!(
        resolved.next_action,
        M5PermissionManifestNextAction::ReviewDataNetworkBoundary
    );
}

#[test]
fn summary_flattened_into_full_access_degrades() {
    let mut input = clean_summary_input();
    input.flattens_into_full_access = true;
    let resolved = resolve_permission_manifest_summary(input).unwrap();
    assert!(resolved.flattens_into_full_access);
    assert_eq!(
        resolved.degrade_reason,
        Some(M5PermissionManifestSummaryDegradeReason::FlattenedIntoFullAccess)
    );
    assert_eq!(
        resolved.next_action,
        M5PermissionManifestNextAction::ReviewCapabilityClasses
    );
}

#[test]
fn summary_digest_unstated_degrades() {
    let mut input = clean_summary_input();
    input.manifest_digest = "".to_owned();
    let resolved = resolve_permission_manifest_summary(input).unwrap();
    assert_eq!(
        resolved.degrade_reason,
        Some(M5PermissionManifestSummaryDegradeReason::ManifestDigestUnstated)
    );
    assert_eq!(
        resolved.next_action,
        M5PermissionManifestNextAction::ReviewManifestDigest
    );
}

#[test]
fn summary_policy_restricted_needs_no_capability_grouping() {
    let mut input = clean_summary_input();
    input.posture = M5PermissionPostureState::PolicyRestricted;
    input.required_capabilities = Vec::new();
    let resolved = resolve_permission_manifest_summary(input).unwrap();
    assert!(!resolved.requests_capabilities);
    assert!(resolved.is_clean());
}

#[test]
fn summary_empty_id_and_forbidden_material_error() {
    let mut input = clean_summary_input();
    input.summary_id = "".to_owned();
    assert_eq!(
        resolve_permission_manifest_summary(input).unwrap_err(),
        M5PermissionManifestResolutionError::EmptySummaryId
    );

    let mut input = clean_summary_input();
    input.network_boundary = "outbound https://relay.internal/leak".to_owned();
    assert_eq!(
        resolve_permission_manifest_summary(input).unwrap_err(),
        M5PermissionManifestResolutionError::ForbiddenMaterial
    );

    let mut input = clean_summary_input();
    input.required_capabilities = vec!["bearer secret".to_owned()];
    assert_eq!(
        resolve_permission_manifest_summary(input).unwrap_err(),
        M5PermissionManifestResolutionError::ForbiddenMaterial
    );
}

#[test]
fn drawer_clean_discloses_widening_and_is_legible() {
    let resolved = resolve_transitive_capability_drawer(clean_drawer_input()).unwrap();
    assert!(resolved.is_clean());
    assert!(resolved.fully_legible);
    assert!(resolved.widens_transitively);
    assert!(resolved.transitive_widening_disclosed);
    assert!(!resolved.hides_transitive_widening);
    assert!(!resolved.flattens_into_full_access);
}

#[test]
fn drawer_widening_hidden_degrades() {
    let mut input = clean_drawer_input();
    input.transitive_widening_disclosed = false;
    let resolved = resolve_transitive_capability_drawer(input).unwrap();
    assert!(resolved.hides_transitive_widening);
    assert_eq!(
        resolved.degrade_reason,
        Some(M5TransitiveCapabilityDrawerDegradeReason::TransitiveWideningHidden)
    );
    assert_eq!(
        resolved.next_action,
        M5PermissionManifestNextAction::ReviewTransitiveWidening
    );
}

#[test]
fn drawer_attribution_missing_degrades() {
    let mut input = clean_drawer_input();
    input.posture = M5PermissionPostureState::Standard;
    input.dependency_contributed_capabilities = vec!["write workspace files".to_owned()];
    input.dependency_attributions = Vec::new();
    let resolved = resolve_transitive_capability_drawer(input).unwrap();
    assert!(!resolved.widens_transitively);
    assert_eq!(
        resolved.degrade_reason,
        Some(M5TransitiveCapabilityDrawerDegradeReason::DependencyAttributionMissing)
    );
}

#[test]
fn drawer_flattened_into_full_access_degrades() {
    let mut input = clean_drawer_input();
    input.posture = M5PermissionPostureState::Elevated;
    input.dependency_contributed_capabilities = Vec::new();
    input.dependency_attributions = Vec::new();
    input.flattens_into_full_access = true;
    let resolved = resolve_transitive_capability_drawer(input).unwrap();
    assert_eq!(
        resolved.degrade_reason,
        Some(M5TransitiveCapabilityDrawerDegradeReason::FlattenedIntoFullAccess)
    );
}

#[test]
fn drawer_digest_unstated_degrades() {
    let mut input = clean_drawer_input();
    input.manifest_digest = "".to_owned();
    let resolved = resolve_transitive_capability_drawer(input).unwrap();
    assert_eq!(
        resolved.degrade_reason,
        Some(M5TransitiveCapabilityDrawerDegradeReason::ManifestDigestUnstated)
    );
}

#[test]
fn drawer_non_widening_posture_is_clean_without_disclosure() {
    let mut input = clean_drawer_input();
    input.posture = M5PermissionPostureState::Minimal;
    input.transitive_widening_disclosed = false;
    input.dependency_contributed_capabilities = Vec::new();
    input.dependency_attributions = Vec::new();
    let resolved = resolve_transitive_capability_drawer(input).unwrap();
    assert!(!resolved.widens_transitively);
    assert!(!resolved.hides_transitive_widening);
    assert!(resolved.is_clean());
}

#[test]
fn drawer_empty_id_and_forbidden_material_error() {
    let mut input = clean_drawer_input();
    input.drawer_id = "   ".to_owned();
    assert_eq!(
        resolve_transitive_capability_drawer(input).unwrap_err(),
        M5PermissionManifestResolutionError::EmptyDrawerId
    );

    let mut input = clean_drawer_input();
    input.dependency_attributions = vec!["see internal://notes".to_owned()];
    assert_eq!(
        resolve_transitive_capability_drawer(input).unwrap_err(),
        M5PermissionManifestResolutionError::ForbiddenMaterial
    );
}

#[test]
fn vocabulary_set_is_canonical() {
    assert!(seeded_m5_permission_manifest_controls()
        .vocabulary_set
        .matches_canonical());
}

#[test]
fn vocabulary_set_drift_fails() {
    let mut packet = seeded_m5_permission_manifest_controls();
    packet.vocabulary_set.capability_classes.pop();
    assert!(packet
        .validate()
        .contains(&M5PermissionManifestControlsViolation::VocabularySetDrift));
}

#[test]
fn missing_source_contracts_fails() {
    let mut packet = seeded_m5_permission_manifest_controls();
    packet.source_contract_refs.clear();
    assert!(packet
        .validate()
        .contains(&M5PermissionManifestControlsViolation::MissingSourceContracts));
}

#[test]
fn component_schema_ref_missing_fails() {
    let mut packet = seeded_m5_permission_manifest_controls();
    packet.controls_rows[0]
        .source_contract_refs
        .retain(|r| r != M5_PERMISSION_MANIFEST_SUMMARY_SCHEMA_REF);
    assert!(packet
        .validate()
        .contains(&M5PermissionManifestControlsViolation::ComponentSchemaRefMissing));
}

#[test]
fn mandatory_anatomy_missing_fails() {
    let mut packet = seeded_m5_permission_manifest_controls();
    packet.controls_rows[0]
        .anatomy_parts
        .retain(|p| *p != M5PermissionManifestAnatomyPart::Identity);
    assert!(packet
        .validate()
        .contains(&M5PermissionManifestControlsViolation::MandatoryAnatomyMissing));
}

#[test]
fn mandatory_export_field_missing_fails() {
    let mut packet = seeded_m5_permission_manifest_controls();
    packet.controls_rows[0]
        .export_fields
        .retain(|f| *f != M5PermissionManifestExportField::Dispositions);
    assert!(packet
        .validate()
        .contains(&M5PermissionManifestControlsViolation::MandatoryExportFieldMissing));
}

#[test]
fn examples_missing_fails() {
    let mut packet = seeded_m5_permission_manifest_controls();
    packet.controls_rows[0]
        .transitive_capability_drawer_examples
        .clear();
    assert!(packet
        .validate()
        .contains(&M5PermissionManifestControlsViolation::ExamplesMissing));
}

#[test]
fn dishonest_clean_summary_example_fails() {
    let mut packet = seeded_m5_permission_manifest_controls();
    let row = &mut packet.controls_rows[0];
    row.permission_manifest_summary_examples[0].degrade_reason = None;
    row.permission_manifest_summary_examples[0].flattens_into_full_access = true;
    assert!(packet
        .validate()
        .contains(&M5PermissionManifestControlsViolation::DishonestExample));
}

#[test]
fn dishonest_clean_drawer_example_fails() {
    let mut packet = seeded_m5_permission_manifest_controls();
    let row = &mut packet.controls_rows[0];
    row.transitive_capability_drawer_examples[0].degrade_reason = None;
    row.transitive_capability_drawer_examples[0].hides_transitive_widening = true;
    assert!(packet
        .validate()
        .contains(&M5PermissionManifestControlsViolation::DishonestExample));
}

#[test]
fn row_invariant_violation_fails() {
    for mutate in 0u8..4 {
        let mut packet = seeded_m5_permission_manifest_controls();
        let row = &mut packet.controls_rows[0];
        match mutate {
            0 => row.flattens_permissions_into_vague_full_access = true,
            1 => row.hides_transitive_or_dependency_contributed_widening = true,
            2 => row.hides_data_network_or_runtime_boundary = true,
            _ => row.severs_summary_from_canonical_manifest_digest = true,
        }
        assert!(packet
            .validate()
            .contains(&M5PermissionManifestControlsViolation::RowInvariantViolated));
    }
}

#[test]
fn permission_posture_explicit_not_proven_when_boundary_example_removed() {
    let mut packet = seeded_m5_permission_manifest_controls();
    for row in &mut packet.controls_rows {
        row.permission_manifest_summary_examples.retain(|ex| {
            ex.degrade_reason
                != Some(M5PermissionManifestSummaryDegradeReason::DataNetworkBoundaryUnstated)
        });
    }
    assert!(packet
        .validate()
        .contains(&M5PermissionManifestControlsViolation::PermissionPostureExplicitNotProven));
}

#[test]
fn permission_posture_explicit_not_proven_when_no_clean_summary_shown() {
    let mut packet = seeded_m5_permission_manifest_controls();
    for row in &mut packet.controls_rows {
        row.permission_manifest_summary_examples.retain(|ex| {
            !ex.is_clean()
                || !ex.requests_capabilities
                || ex.required_capabilities.is_empty()
                || ex.manifest_digest.trim().is_empty()
        });
    }
    assert!(packet
        .validate()
        .contains(&M5PermissionManifestControlsViolation::PermissionPostureExplicitNotProven));
}

#[test]
fn transitive_widening_attributable_not_proven_when_hidden_example_removed() {
    let mut packet = seeded_m5_permission_manifest_controls();
    for row in &mut packet.controls_rows {
        row.transitive_capability_drawer_examples.retain(|ex| {
            ex.degrade_reason
                != Some(M5TransitiveCapabilityDrawerDegradeReason::TransitiveWideningHidden)
        });
    }
    assert!(packet
        .validate()
        .contains(&M5PermissionManifestControlsViolation::TransitiveWideningAttributableNotProven));
}

#[test]
fn transitive_widening_attributable_not_proven_when_attribution_example_removed() {
    let mut packet = seeded_m5_permission_manifest_controls();
    for row in &mut packet.controls_rows {
        row.transitive_capability_drawer_examples.retain(|ex| {
            ex.degrade_reason
                != Some(M5TransitiveCapabilityDrawerDegradeReason::DependencyAttributionMissing)
        });
    }
    assert!(packet
        .validate()
        .contains(&M5PermissionManifestControlsViolation::TransitiveWideningAttributableNotProven));
}

#[test]
fn governance_review_incomplete_fails() {
    let mut packet = seeded_m5_permission_manifest_controls();
    packet
        .governance_review
        .transitive_widening_always_visible_and_attributable = false;
    assert!(packet
        .validate()
        .contains(&M5PermissionManifestControlsViolation::GovernanceReviewIncomplete));
}

#[test]
fn consumer_projection_incomplete_fails() {
    let mut packet = seeded_m5_permission_manifest_controls();
    packet
        .consumer_projection
        .support_export_reads_single_source = false;
    assert!(packet
        .validate()
        .contains(&M5PermissionManifestControlsViolation::ConsumerProjectionIncomplete));
}

#[test]
fn proof_freshness_incomplete_fails() {
    let mut packet = seeded_m5_permission_manifest_controls();
    packet.proof_freshness.proof_freshness_slo_hours = 0;
    assert!(packet
        .validate()
        .contains(&M5PermissionManifestControlsViolation::ProofFreshnessIncomplete));
}

#[test]
fn release_posture_incomplete_fails() {
    let mut packet = seeded_m5_permission_manifest_controls();
    packet.release_posture.accessibility_parity_required = false;
    assert!(packet
        .validate()
        .contains(&M5PermissionManifestControlsViolation::ReleasePostureIncomplete));
}

#[test]
fn injected_raw_material_is_rejected() {
    let mut packet = seeded_m5_permission_manifest_controls();
    packet.controls_rows[0].scope_summary =
        "raw endpoint https://relay.internal.example/session leaked".to_owned();
    assert!(packet
        .validate()
        .contains(&M5PermissionManifestControlsViolation::RawMaterialInExport));
}

#[test]
fn export_carries_no_forbidden_raw_material() {
    let json = seeded_m5_permission_manifest_controls().export_safe_json();
    let lower = json.to_lowercase();
    assert!(!lower.contains("password"));
    assert!(!lower.contains("passphrase"));
    assert!(!lower.contains("bearer "));
    assert!(!lower.contains("://"));
    assert!(!lower.contains("-----begin"));
}

#[test]
fn csv_has_a_row_per_consumer_surface() {
    let packet = seeded_m5_permission_manifest_controls();
    let csv = packet.render_matrix_csv();
    let lines: Vec<&str> = csv.lines().collect();
    assert_eq!(lines.len(), 1 + packet.controls_rows.len());
    assert!(lines[0].starts_with("consumer_surface,qualification,owner,"));
}

#[test]
fn markdown_summary_lists_every_consumer_surface() {
    let packet = seeded_m5_permission_manifest_controls();
    let summary = packet.render_markdown_summary();
    for row in &packet.controls_rows {
        assert!(summary.contains(row.consumer_surface.as_str()));
    }
}

#[test]
fn checked_support_export_validates_and_matches_seed() {
    let from_disk = current_stable_m5_permission_manifest_controls_export()
        .expect("checked M5 permission-manifest controls export validates");
    assert_eq!(
        from_disk.packet_id,
        M5_PERMISSION_MANIFEST_CONTROLS_PACKET_ID
    );
    assert_eq!(
        from_disk,
        seeded_m5_permission_manifest_controls(),
        "checked support export drifted from the seed builder"
    );
}

#[test]
fn narrowed_variants_validate_and_keep_rows_visible() {
    let beta = seeded_m5_permission_manifest_controls_marketplace_ui_beta_narrowed();
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

    let preview = seeded_m5_permission_manifest_controls_install_review_ui_preview_narrowed();
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
    let beta: M5PermissionManifestControlsPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/ui/m5-permission-manifest-summary-transitive-capability-drawer-controls/marketplace_ui_beta_narrowed.json"
    )))
    .expect("marketplace-ui fixture parses");
    assert!(beta.validate().is_empty());
    assert_eq!(
        beta,
        seeded_m5_permission_manifest_controls_marketplace_ui_beta_narrowed()
    );

    let preview: M5PermissionManifestControlsPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/ui/m5-permission-manifest-summary-transitive-capability-drawer-controls/install_review_ui_preview_narrowed.json"
    )))
    .expect("install-review fixture parses");
    assert!(preview.validate().is_empty());
    assert_eq!(
        preview,
        seeded_m5_permission_manifest_controls_install_review_ui_preview_narrowed()
    );
}

#[test]
fn implemented_families_is_the_permission_manifest_component() {
    assert_eq!(
        IMPLEMENTED_FAMILIES,
        [M5MarketplaceInstallComponentFamily::PermissionManifestSummary]
    );
}
