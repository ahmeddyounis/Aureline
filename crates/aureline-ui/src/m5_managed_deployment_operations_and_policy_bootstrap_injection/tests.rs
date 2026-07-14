use super::*;

fn clean_operation_input() -> M5ManagedOperationEntryResolutionInput {
    M5ManagedOperationEntryResolutionInput {
        entry_id: "operation:test".to_owned(),
        profile_id: "profile.per_machine_managed".to_owned(),
        token_name: "managed.operation.silent_install".to_owned(),
        semantic_role: M5InstallTopologyRole::InstallMode,
        operation: M5ManagedOperation::SilentInstall,
        surface_context: M5ManagedSurfaceContext::InstallerFlow,
        presentation_form_coverage: M5ManagedPresentationForm::ALL.to_vec(),
        operation_target_root: r"%ProgramFiles%\Aureline".to_owned(),
        receipt_root: r"%ProgramData%\Aureline\receipts".to_owned(),
        failure_diagnostics_root: r"%ProgramData%\Aureline\logs".to_owned(),
        receipt_fields_covered: M5ManagedReceiptField::ALL.to_vec(),
        ownership: M5ManagedOwnership::AdminOwned,
        bound_to_registry: true,
        ownership_misrepresented_used: false,
        ownership_disclosure_enforced: true,
        proof_fresh: true,
    }
}

fn clean_injection_input() -> M5PolicyInjectionEntryResolutionInput {
    M5PolicyInjectionEntryResolutionInput {
        entry_id: "injection:test".to_owned(),
        profile_id: "profile.per_machine_managed".to_owned(),
        token_name: "managed.injection.channel".to_owned(),
        semantic_role: M5InstallTopologyRole::PolicyRoots,
        injection_surface: M5PolicyInjectionSurface::ManagedPolicyChannel,
        surface_context: M5ManagedSurfaceContext::InstallerFlow,
        presentation_form_coverage: M5ManagedPresentationForm::ALL.to_vec(),
        policy_bundle_source: r"%ProgramData%\Aureline\policy\bootstrap.json".to_owned(),
        bootstrap_target: r"%ProgramData%\Aureline\policy\applied".to_owned(),
        disclosed_fields: M5PolicyInjectionField::ALL.to_vec(),
        pin_deferral_posture: M5ChannelDeferralPosture::ChannelPinned,
        pin_and_deferral_continuity_documented: true,
        admin_control_disclosed: true,
        proof_fresh: true,
    }
}

#[test]
fn seeded_registries_validates() {
    let packet = seeded_m5_managed_deployment_operations_and_policy_bootstrap_injection();
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
    assert_eq!(
        packet.packet_id,
        M5_MANAGED_DEPLOYMENT_OPERATIONS_AND_POLICY_BOOTSTRAP_INJECTION_PACKET_ID
    );
}

#[test]
fn operation_clean_names_meaning_and_is_bound() {
    let resolved = resolve_managed_operation_entry(clean_operation_input()).unwrap();
    assert!(resolved.is_clean());
    assert!(resolved.operation_resolves_across_profiles);
    assert!(resolved.covers_all_presentation_forms);
    assert!(resolved.managed_receipt_complete);
    assert!(resolved.operation_is_accountable);
    assert!(resolved.bound_to_registry);
    assert!(resolved.operation_is_supported);
    assert!(resolved.ownership_is_disclosed);
    assert_eq!(resolved.semantic_role, "install_mode");
    assert_eq!(resolved.operation, "silent_install");
    assert_eq!(resolved.ownership, "admin_owned");
    assert_eq!(resolved.surface_context, "installer_flow");
    assert_eq!(
        resolved.next_action,
        M5ManagedNextAction::ExpandManagedMeaning
    );
}

#[test]
fn operation_token_unstated_degrades() {
    let mut input = clean_operation_input();
    input.token_name = "   ".to_owned();
    assert_eq!(
        resolve_managed_operation_entry(input)
            .unwrap()
            .degrade_reason,
        Some(M5ManagedOperationEntryDegradeReason::OperationTokenUnstated)
    );
}

#[test]
fn operation_unbound_and_unclassified_degrade() {
    let mut input = clean_operation_input();
    input.bound_to_registry = false;
    assert_eq!(
        resolve_managed_operation_entry(input)
            .unwrap()
            .degrade_reason,
        Some(M5ManagedOperationEntryDegradeReason::OperationNotBoundToRegistry)
    );

    let mut input = clean_operation_input();
    input.operation = M5ManagedOperation::OperationUnclassified;
    assert_eq!(
        resolve_managed_operation_entry(input)
            .unwrap()
            .degrade_reason,
        Some(M5ManagedOperationEntryDegradeReason::OperationUnclassified)
    );
}

#[test]
fn operation_receipt_and_misrepresentation_and_ownership_and_form_degrade() {
    // A dropped mandatory receipt field leaves the inventory incomplete.
    let mut input = clean_operation_input();
    input.receipt_fields_covered = vec![
        M5ManagedReceiptField::InstallId,
        M5ManagedReceiptField::Timestamp,
        M5ManagedReceiptField::FailureSummary,
    ];
    let resolved = resolve_managed_operation_entry(input).unwrap();
    assert!(!resolved.managed_receipt_complete);
    assert_eq!(
        resolved.degrade_reason,
        Some(M5ManagedOperationEntryDegradeReason::ManagedReceiptInventoryIncomplete)
    );

    // A managed installer presented as user-controlled degrades.
    let mut input = clean_operation_input();
    input.ownership_misrepresented_used = true;
    input.ownership_disclosure_enforced = false;
    assert_eq!(
        resolve_managed_operation_entry(input)
            .unwrap()
            .degrade_reason,
        Some(M5ManagedOperationEntryDegradeReason::ManagedInstallerPresentedAsUserControlled)
    );

    // An unenforced (unproven) disclosure posture degrades even without an actual misrepresentation.
    let mut input = clean_operation_input();
    input.ownership_disclosure_enforced = false;
    assert_eq!(
        resolve_managed_operation_entry(input)
            .unwrap()
            .degrade_reason,
        Some(M5ManagedOperationEntryDegradeReason::ManagedInstallerPresentedAsUserControlled)
    );

    // An ambiguous ownership degrades.
    let mut input = clean_operation_input();
    input.ownership = M5ManagedOwnership::OwnershipAmbiguous;
    assert_eq!(
        resolve_managed_operation_entry(input)
            .unwrap()
            .degrade_reason,
        Some(M5ManagedOperationEntryDegradeReason::OwnershipAmbiguous)
    );

    let mut input = clean_operation_input();
    input.presentation_form_coverage = vec![M5ManagedPresentationForm::CanonicalObject];
    assert_eq!(
        resolve_managed_operation_entry(input)
            .unwrap()
            .degrade_reason,
        Some(M5ManagedOperationEntryDegradeReason::PresentationFormCoverageIncomplete)
    );
}

#[test]
fn operation_surface_and_proof_degrade() {
    let mut input = clean_operation_input();
    input.surface_context = M5ManagedSurfaceContext::ContextUnknown;
    assert_eq!(
        resolve_managed_operation_entry(input)
            .unwrap()
            .degrade_reason,
        Some(M5ManagedOperationEntryDegradeReason::SurfaceContextUnresolved)
    );

    let mut input = clean_operation_input();
    input.proof_fresh = false;
    assert_eq!(
        resolve_managed_operation_entry(input)
            .unwrap()
            .degrade_reason,
        Some(M5ManagedOperationEntryDegradeReason::ProofStale)
    );
}

#[test]
fn operation_empty_id_and_forbidden_material_error() {
    let mut input = clean_operation_input();
    input.entry_id = "".to_owned();
    assert_eq!(
        resolve_managed_operation_entry(input).unwrap_err(),
        M5ManagedResolutionError::EmptyOperationEntryId
    );

    let mut input = clean_operation_input();
    input.operation_target_root = "see https://relay.internal/leak".to_owned();
    assert_eq!(
        resolve_managed_operation_entry(input).unwrap_err(),
        M5ManagedResolutionError::ForbiddenMaterial
    );
}

#[test]
fn managed_operation_is_accountable_requires_no_misrepresentation_and_full_receipt() {
    assert!(managed_operation_is_accountable(
        M5ManagedOperation::SilentInstall,
        r"%ProgramFiles%\Aureline",
        r"%ProgramData%\Aureline\receipts",
        r"%ProgramData%\Aureline\logs",
        &M5ManagedReceiptField::ALL,
        false,
        true,
    ));
    // A misrepresented installer breaks accountability.
    assert!(!managed_operation_is_accountable(
        M5ManagedOperation::SilentInstall,
        r"%ProgramFiles%\Aureline",
        r"%ProgramData%\Aureline\receipts",
        r"%ProgramData%\Aureline\logs",
        &M5ManagedReceiptField::ALL,
        true,
        true,
    ));
    // An unenforced disclosure posture is not proven accountable.
    assert!(!managed_operation_is_accountable(
        M5ManagedOperation::SilentInstall,
        r"%ProgramFiles%\Aureline",
        r"%ProgramData%\Aureline\receipts",
        r"%ProgramData%\Aureline\logs",
        &M5ManagedReceiptField::ALL,
        false,
        false,
    ));
    // An unclassified operation is never accountable.
    assert!(!managed_operation_is_accountable(
        M5ManagedOperation::OperationUnclassified,
        r"%ProgramFiles%\Aureline",
        r"%ProgramData%\Aureline\receipts",
        r"%ProgramData%\Aureline\logs",
        &M5ManagedReceiptField::ALL,
        false,
        true,
    ));
}

#[test]
fn injection_clean_is_disclosed_and_continuous() {
    let resolved = resolve_policy_bootstrap_injection_entry(clean_injection_input()).unwrap();
    assert!(resolved.is_clean());
    assert!(resolved.injection_discoverable_on_every_profile);
    assert!(resolved.covers_all_presentation_forms);
    assert!(resolved.injection_is_disclosed);
    assert!(resolved.pin_and_deferral_is_continuous);
    assert_eq!(resolved.injection_surface, "managed_policy_channel");
    assert_eq!(resolved.pin_deferral_posture, "channel_pinned");
}

#[test]
fn injection_disclosure_and_continuity_and_surface_degrade() {
    // A dropped mandatory field leaves the disclosure incomplete.
    let mut input = clean_injection_input();
    input.disclosed_fields = vec![
        M5PolicyInjectionField::PolicyBundleSource,
        M5PolicyInjectionField::BootstrapTarget,
        M5PolicyInjectionField::AppliedSettings,
        M5PolicyInjectionField::AdminOwner,
    ];
    let resolved = resolve_policy_bootstrap_injection_entry(input).unwrap();
    assert!(!resolved.injection_is_disclosed);
    assert_eq!(
        resolved.degrade_reason,
        Some(M5PolicyInjectionEntryDegradeReason::PolicyInjectionDisclosureIncomplete)
    );

    // An undisclosed admin ownership is also an incomplete disclosure.
    let mut input = clean_injection_input();
    input.admin_control_disclosed = false;
    assert_eq!(
        resolve_policy_bootstrap_injection_entry(input)
            .unwrap()
            .degrade_reason,
        Some(M5PolicyInjectionEntryDegradeReason::PolicyInjectionDisclosureIncomplete)
    );

    // An undocumented continuity note degrades.
    let mut input = clean_injection_input();
    input.pin_and_deferral_continuity_documented = false;
    assert_eq!(
        resolve_policy_bootstrap_injection_entry(input)
            .unwrap()
            .degrade_reason,
        Some(M5PolicyInjectionEntryDegradeReason::PinAndDeferralContinuityUndocumented)
    );

    // An unclassified posture also breaks continuity.
    let mut input = clean_injection_input();
    input.pin_deferral_posture = M5ChannelDeferralPosture::PostureUnclassified;
    assert_eq!(
        resolve_policy_bootstrap_injection_entry(input)
            .unwrap()
            .degrade_reason,
        Some(M5PolicyInjectionEntryDegradeReason::PinAndDeferralContinuityUndocumented)
    );

    let mut input = clean_injection_input();
    input.injection_surface = M5PolicyInjectionSurface::SurfaceUnclassified;
    assert_eq!(
        resolve_policy_bootstrap_injection_entry(input)
            .unwrap()
            .degrade_reason,
        Some(M5PolicyInjectionEntryDegradeReason::InjectionSurfaceUnclassified)
    );
}

#[test]
fn injection_form_and_id_and_material() {
    let mut input = clean_injection_input();
    input.presentation_form_coverage = vec![M5ManagedPresentationForm::CanonicalObject];
    assert_eq!(
        resolve_policy_bootstrap_injection_entry(input)
            .unwrap()
            .degrade_reason,
        Some(M5PolicyInjectionEntryDegradeReason::InjectionFormCoverageIncomplete)
    );

    let mut input = clean_injection_input();
    input.entry_id = "  ".to_owned();
    assert_eq!(
        resolve_policy_bootstrap_injection_entry(input).unwrap_err(),
        M5ManagedResolutionError::EmptyInjectionEntryId
    );

    let mut input = clean_injection_input();
    input.bootstrap_target = "see internal://notes".to_owned();
    assert_eq!(
        resolve_policy_bootstrap_injection_entry(input).unwrap_err(),
        M5ManagedResolutionError::ForbiddenMaterial
    );
}

#[test]
fn vocabulary_set_is_canonical() {
    assert!(
        seeded_m5_managed_deployment_operations_and_policy_bootstrap_injection()
            .vocabulary_set
            .matches_canonical()
    );
}

#[test]
fn vocabulary_set_drift_fails() {
    let mut packet = seeded_m5_managed_deployment_operations_and_policy_bootstrap_injection();
    packet.vocabulary_set.managed_operations.pop();
    assert!(packet.validate().contains(
        &M5ManagedDeploymentOperationsAndPolicyBootstrapInjectionViolation::VocabularySetDrift
    ));
}

#[test]
fn missing_source_contracts_fails() {
    let mut packet = seeded_m5_managed_deployment_operations_and_policy_bootstrap_injection();
    packet.source_contract_refs.clear();
    assert!(packet.validate().contains(
        &M5ManagedDeploymentOperationsAndPolicyBootstrapInjectionViolation::MissingSourceContracts
    ));
}

#[test]
fn domain_schema_ref_missing_fails() {
    let mut packet = seeded_m5_managed_deployment_operations_and_policy_bootstrap_injection();
    packet.registry_rows[0]
        .source_contract_refs
        .retain(|r| r != M5_INSTALL_TOPOLOGY_DOMAIN_SCHEMA_REF);
    assert!(packet.validate().contains(
        &M5ManagedDeploymentOperationsAndPolicyBootstrapInjectionViolation::DomainSchemaRefMissing
    ));
}

#[test]
fn mandatory_anatomy_missing_fails() {
    let mut packet = seeded_m5_managed_deployment_operations_and_policy_bootstrap_injection();
    packet.registry_rows[0]
        .anatomy_parts
        .retain(|p| *p != M5ManagedAnatomyPart::Identity);
    assert!(packet.validate().contains(
        &M5ManagedDeploymentOperationsAndPolicyBootstrapInjectionViolation::MandatoryAnatomyMissing
    ));
}

#[test]
fn mandatory_export_field_missing_fails() {
    let mut packet = seeded_m5_managed_deployment_operations_and_policy_bootstrap_injection();
    packet.registry_rows[0]
        .export_fields
        .retain(|f| *f != M5ManagedExportField::Ownerships);
    assert!(packet.validate().contains(
        &M5ManagedDeploymentOperationsAndPolicyBootstrapInjectionViolation::MandatoryExportFieldMissing
    ));
}

#[test]
fn examples_missing_fails() {
    let mut packet = seeded_m5_managed_deployment_operations_and_policy_bootstrap_injection();
    packet.registry_rows[0].policy_injection_entries.clear();
    assert!(packet.validate().contains(
        &M5ManagedDeploymentOperationsAndPolicyBootstrapInjectionViolation::ExamplesMissing
    ));
}

#[test]
fn dishonest_clean_example_fails() {
    let mut packet = seeded_m5_managed_deployment_operations_and_policy_bootstrap_injection();
    // Force a clean operation entry to also read as receipt-incomplete — the packet must reject it.
    let row = &mut packet.registry_rows[0];
    row.managed_operation_entries[0].degrade_reason = None;
    row.managed_operation_entries[0].managed_receipt_complete = false;
    assert!(packet.validate().contains(
        &M5ManagedDeploymentOperationsAndPolicyBootstrapInjectionViolation::DishonestExample
    ));
}

#[test]
fn row_invariant_violation_fails() {
    for mutate in 0u8..4 {
        let mut packet = seeded_m5_managed_deployment_operations_and_policy_bootstrap_injection();
        let row = &mut packet.registry_rows[0];
        match mutate {
            0 => row.managed_installer_presented_as_user_controlled = true,
            1 => row.managed_failure_stranded_user_without_diagnostics = true,
            2 => row.channel_pinning_or_repair_verify_drifted_from_matrix = true,
            _ => row.policy_bootstrap_injection_ownership_left_undisclosed = true,
        }
        assert!(packet.validate().contains(
            &M5ManagedDeploymentOperationsAndPolicyBootstrapInjectionViolation::RowInvariantViolated
        ));
    }
}

#[test]
fn managed_operation_contract_not_proven_when_incomplete_example_removed() {
    let mut packet = seeded_m5_managed_deployment_operations_and_policy_bootstrap_injection();
    for row in &mut packet.registry_rows {
        row.managed_operation_entries.retain(|ex| {
            ex.degrade_reason
                != Some(M5ManagedOperationEntryDegradeReason::ManagedReceiptInventoryIncomplete)
        });
    }
    assert!(packet.validate().contains(
        &M5ManagedDeploymentOperationsAndPolicyBootstrapInjectionViolation::ManagedOperationContractNotProven
    ));
}

#[test]
fn managed_operation_contract_not_proven_when_surface_collapses() {
    let mut packet = seeded_m5_managed_deployment_operations_and_policy_bootstrap_injection();
    // Drop every clean admin operation so the first-consumer surfaces no longer include it.
    for row in &mut packet.registry_rows {
        row.managed_operation_entries
            .retain(|ex| !(ex.is_clean() && ex.surface_context == "admin_surface"));
    }
    assert!(packet.validate().contains(
        &M5ManagedDeploymentOperationsAndPolicyBootstrapInjectionViolation::ManagedOperationContractNotProven
    ));
}

#[test]
fn ownership_disclosure_not_proven_when_ambiguous_example_removed() {
    let mut packet = seeded_m5_managed_deployment_operations_and_policy_bootstrap_injection();
    for row in &mut packet.registry_rows {
        row.managed_operation_entries.retain(|ex| {
            ex.degrade_reason != Some(M5ManagedOperationEntryDegradeReason::OwnershipAmbiguous)
        });
    }
    assert!(packet.validate().contains(
        &M5ManagedDeploymentOperationsAndPolicyBootstrapInjectionViolation::OwnershipDisclosureNotProven
    ));
}

#[test]
fn ownership_disclosure_not_proven_when_injection_surface_dropped() {
    let mut packet = seeded_m5_managed_deployment_operations_and_policy_bootstrap_injection();
    // Drop every clean docs-help injection so the canonical surface coverage no longer includes it.
    for row in &mut packet.registry_rows {
        row.policy_injection_entries
            .retain(|ex| !(ex.is_clean() && ex.injection_surface == "docs_help_injection"));
    }
    assert!(packet.validate().contains(
        &M5ManagedDeploymentOperationsAndPolicyBootstrapInjectionViolation::OwnershipDisclosureNotProven
    ));
}

#[test]
fn drift_detection_not_proven_when_misrepresentation_example_removed() {
    let mut packet = seeded_m5_managed_deployment_operations_and_policy_bootstrap_injection();
    for row in &mut packet.registry_rows {
        row.managed_operation_entries.retain(|ex| {
            ex.degrade_reason
                != Some(
                    M5ManagedOperationEntryDegradeReason::ManagedInstallerPresentedAsUserControlled,
                )
        });
    }
    assert!(packet.validate().contains(
        &M5ManagedDeploymentOperationsAndPolicyBootstrapInjectionViolation::DriftDetectionNotProven
    ));
}

#[test]
fn drift_detection_not_proven_when_continuity_example_removed() {
    let mut packet = seeded_m5_managed_deployment_operations_and_policy_bootstrap_injection();
    for row in &mut packet.registry_rows {
        row.policy_injection_entries.retain(|ex| {
            ex.degrade_reason
                != Some(M5PolicyInjectionEntryDegradeReason::PinAndDeferralContinuityUndocumented)
        });
    }
    assert!(packet.validate().contains(
        &M5ManagedDeploymentOperationsAndPolicyBootstrapInjectionViolation::DriftDetectionNotProven
    ));
}

#[test]
fn governance_review_incomplete_fails() {
    let mut packet = seeded_m5_managed_deployment_operations_and_policy_bootstrap_injection();
    packet
        .governance_review
        .managed_installer_never_presented_as_user_controlled = false;
    assert!(packet.validate().contains(
        &M5ManagedDeploymentOperationsAndPolicyBootstrapInjectionViolation::GovernanceReviewIncomplete
    ));
}

#[test]
fn consumer_projection_incomplete_fails() {
    let mut packet = seeded_m5_managed_deployment_operations_and_policy_bootstrap_injection();
    packet
        .consumer_projection
        .support_export_reads_single_registry_source = false;
    assert!(packet.validate().contains(
        &M5ManagedDeploymentOperationsAndPolicyBootstrapInjectionViolation::ConsumerProjectionIncomplete
    ));
}

#[test]
fn proof_freshness_incomplete_fails() {
    let mut packet = seeded_m5_managed_deployment_operations_and_policy_bootstrap_injection();
    packet.proof_freshness.proof_freshness_slo_hours = 0;
    assert!(packet.validate().contains(
        &M5ManagedDeploymentOperationsAndPolicyBootstrapInjectionViolation::ProofFreshnessIncomplete
    ));
}

#[test]
fn release_posture_incomplete_fails() {
    let mut packet = seeded_m5_managed_deployment_operations_and_policy_bootstrap_injection();
    packet.release_posture.accessibility_parity_required = false;
    assert!(packet.validate().contains(
        &M5ManagedDeploymentOperationsAndPolicyBootstrapInjectionViolation::ReleasePostureIncomplete
    ));
}

#[test]
fn injected_raw_material_is_rejected() {
    let mut packet = seeded_m5_managed_deployment_operations_and_policy_bootstrap_injection();
    packet.registry_rows[0].scope_summary =
        "raw endpoint https://relay.internal.example/session leaked".to_owned();
    assert!(packet.validate().contains(
        &M5ManagedDeploymentOperationsAndPolicyBootstrapInjectionViolation::RawMaterialInExport
    ));
}

#[test]
fn export_carries_no_forbidden_raw_material() {
    let json =
        seeded_m5_managed_deployment_operations_and_policy_bootstrap_injection().export_safe_json();
    let lower = json.to_lowercase();
    assert!(!lower.contains("password"));
    assert!(!lower.contains("passphrase"));
    assert!(!lower.contains("bearer "));
    assert!(!lower.contains("://"));
    assert!(!lower.contains("-----begin"));
}

#[test]
fn csv_has_a_row_per_consumer_surface() {
    let packet = seeded_m5_managed_deployment_operations_and_policy_bootstrap_injection();
    let csv = packet.render_matrix_csv();
    let lines: Vec<&str> = csv.lines().collect();
    assert_eq!(lines.len(), 1 + packet.registry_rows.len());
    assert!(lines[0].starts_with("consumer_surface,qualification,owner,"));
}

#[test]
fn markdown_summary_lists_every_consumer_surface() {
    let packet = seeded_m5_managed_deployment_operations_and_policy_bootstrap_injection();
    let summary = packet.render_markdown_summary();
    for row in &packet.registry_rows {
        assert!(summary.contains(row.consumer_surface.as_str()));
    }
}

#[test]
fn receipt_table_lists_only_clean_operations() {
    let packet = seeded_m5_managed_deployment_operations_and_policy_bootstrap_injection();
    let table = packet.render_managed_operation_receipt_table();
    // The clean silent-install and channel-pin operations are rendered from the registry.
    assert!(table.contains("silent_install"));
    assert!(table.contains("channel_pin"));
    // A degraded, incomplete entry never leaks into the generated table.
    assert!(!table.contains("receipt-incomplete"));
    // An ambiguous ownership never leaks into the generated table.
    assert!(!table.contains("ownership_ambiguous"));
}

#[test]
fn checked_support_export_validates_and_matches_seed() {
    let from_disk =
        current_stable_m5_managed_deployment_operations_and_policy_bootstrap_injection_export()
            .expect("checked M5 managed-deployment operations / injection export validates");
    assert_eq!(
        from_disk.packet_id,
        M5_MANAGED_DEPLOYMENT_OPERATIONS_AND_POLICY_BOOTSTRAP_INJECTION_PACKET_ID
    );
    assert_eq!(
        from_disk,
        seeded_m5_managed_deployment_operations_and_policy_bootstrap_injection(),
        "checked support export drifted from the seed builder"
    );
}

#[test]
fn narrowed_variants_validate_and_keep_rows_visible() {
    let beta =
        seeded_m5_managed_deployment_operations_and_policy_bootstrap_injection_per_machine_managed_beta_narrowed();
    assert!(beta.validate().is_empty(), "{:?}", beta.validate());
    assert_eq!(beta.registry_rows.len(), 6);
    let row = beta
        .registry_rows
        .iter()
        .find(|r| r.consumer_surface == M5InstallTopologyConsumerSurface::Admin)
        .unwrap();
    assert_eq!(row.qualification, M5InstallTopologyQualificationClass::Beta);

    let preview =
        seeded_m5_managed_deployment_operations_and_policy_bootstrap_injection_offline_airgap_bundle_preview_narrowed();
    assert!(preview.validate().is_empty(), "{:?}", preview.validate());
    assert_eq!(preview.registry_rows.len(), 6);
    let row = preview
        .registry_rows
        .iter()
        .find(|r| r.consumer_surface == M5InstallTopologyConsumerSurface::UpdaterService)
        .unwrap();
    assert_eq!(
        row.qualification,
        M5InstallTopologyQualificationClass::Preview
    );
}

#[test]
fn checked_narrowed_fixtures_validate_and_match_seed_builders() {
    let beta: M5ManagedDeploymentOperationsAndPolicyBootstrapInjectionPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/install/m5-managed-deployment-operations-and-policy-bootstrap-injection/per_machine_managed_beta_narrowed.json"
    )))
    .expect("per-machine-managed fixture parses");
    assert!(beta.validate().is_empty());
    assert_eq!(
        beta,
        seeded_m5_managed_deployment_operations_and_policy_bootstrap_injection_per_machine_managed_beta_narrowed()
    );

    let preview: M5ManagedDeploymentOperationsAndPolicyBootstrapInjectionPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/install/m5-managed-deployment-operations-and-policy-bootstrap-injection/offline_airgap_bundle_preview_narrowed.json"
    )))
    .expect("offline fixture parses");
    assert!(preview.validate().is_empty());
    assert_eq!(
        preview,
        seeded_m5_managed_deployment_operations_and_policy_bootstrap_injection_offline_airgap_bundle_preview_narrowed()
    );
}

#[test]
fn implemented_families_is_managed() {
    assert_eq!(
        IMPLEMENTED_FAMILIES,
        [
            M5InstallTopologyFamily::PerUserManaged,
            M5InstallTopologyFamily::PerMachineManaged
        ]
    );
}
