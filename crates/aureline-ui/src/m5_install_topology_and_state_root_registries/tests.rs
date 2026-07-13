use super::*;

fn clean_install_input() -> M5InstallTopologyEntryResolutionInput {
    M5InstallTopologyEntryResolutionInput {
        entry_id: "install:test".to_owned(),
        profile_id: "profile.per_user_managed".to_owned(),
        token_name: "install.topology.per_user".to_owned(),
        semantic_role: M5InstallTopologyRole::InstallMode,
        delivery_scope: M5DeliveryScope::PerUserManagedScope,
        surface_context: M5InstallSurfaceContext::AboutSurface,
        resolution_form_coverage: M5InstallStateResolutionForm::ALL.to_vec(),
        channel: "stable".to_owned(),
        updater_owner: "per_user_updater".to_owned(),
        binary_root: r"%LOCALAPPDATA%\Aureline\app".to_owned(),
        writable_state_roots: r"%LOCALAPPDATA%\Aureline\state".to_owned(),
        policy_roots: r"%LOCALAPPDATA%\Aureline\policy".to_owned(),
        rollback_target: "artifact-graph:per-user:stable".to_owned(),
        bound_to_registry: true,
        state_namespaces_isolated: true,
        coexists_with_sibling_channel: false,
        coexistence_handoff_explained: true,
        proof_fresh: true,
    }
}

fn clean_boundary_input() -> M5StateRootBoundaryEntryResolutionInput {
    M5StateRootBoundaryEntryResolutionInput {
        entry_id: "boundary:test".to_owned(),
        profile_id: "profile.portable_mode".to_owned(),
        token_name: "state.root.portable".to_owned(),
        semantic_role: M5InstallTopologyRole::WritableStateRoots,
        state_root_surface: M5StateRootSurface::PortableModeBoundary,
        surface_context: M5InstallSurfaceContext::AboutSurface,
        resolution_form_coverage: M5InstallStateResolutionForm::ALL.to_vec(),
        writable_state_roots: r".\AurelinePortable\state".to_owned(),
        policy_roots: r".\AurelinePortable\policy".to_owned(),
        rollback_target: "artifact-graph:portable:full".to_owned(),
        rollback_targets_full_graph: true,
        boundary_is_truthful: true,
        machine_global_spill_used: false,
        machine_global_spill_disclosed: false,
        narrower_scope_asserted: false,
        narrower_scope_explained: false,
        proof_fresh: true,
    }
}

#[test]
fn seeded_registries_validates() {
    let packet = seeded_m5_install_topology_and_state_root_registries();
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
    assert_eq!(
        packet.packet_id,
        M5_INSTALL_TOPOLOGY_STATE_ROOT_REGISTRIES_PACKET_ID
    );
}

#[test]
fn install_clean_names_meaning_and_is_bound() {
    let resolved = resolve_install_topology_entry(clean_install_input()).unwrap();
    assert!(resolved.is_clean());
    assert!(resolved.topology_resolves_across_profiles);
    assert!(resolved.covers_all_resolution_forms);
    assert!(resolved.install_topology_object_complete);
    assert!(resolved.bound_to_registry);
    assert!(resolved.delivery_scope_is_classified);
    assert!(resolved.state_namespaces_isolated);
    assert_eq!(resolved.semantic_role, "install_mode");
    assert_eq!(resolved.delivery_scope, "per_user_managed_scope");
    assert_eq!(resolved.canonical_install_mode, "per_user_managed_install");
    assert_eq!(resolved.surface_context, "about_surface");
    assert_eq!(
        resolved.next_action,
        M5InstallStateNextAction::ExpandInstallMeaning
    );
}

#[test]
fn install_token_unstated_degrades() {
    let mut input = clean_install_input();
    input.token_name = "   ".to_owned();
    assert_eq!(
        resolve_install_topology_entry(input)
            .unwrap()
            .degrade_reason,
        Some(M5InstallTopologyEntryDegradeReason::InstallTokenUnstated)
    );
}

#[test]
fn install_unbound_and_unclassified_degrade() {
    let mut input = clean_install_input();
    input.bound_to_registry = false;
    assert_eq!(
        resolve_install_topology_entry(input)
            .unwrap()
            .degrade_reason,
        Some(M5InstallTopologyEntryDegradeReason::TopologyNotBoundToRegistry)
    );

    let mut input = clean_install_input();
    input.delivery_scope = M5DeliveryScope::ScopeUnclassified;
    assert_eq!(
        resolve_install_topology_entry(input)
            .unwrap()
            .degrade_reason,
        Some(M5InstallTopologyEntryDegradeReason::DeliveryScopeUnclassified)
    );
}

#[test]
fn install_object_incomplete_and_namespace_and_form_degrade() {
    // An unstated updater owner leaves the resolved object incomplete.
    let mut input = clean_install_input();
    input.updater_owner = "  ".to_owned();
    let resolved = resolve_install_topology_entry(input).unwrap();
    assert!(!resolved.install_topology_object_complete);
    assert_eq!(
        resolved.degrade_reason,
        Some(M5InstallTopologyEntryDegradeReason::InstallTopologyObjectIncomplete)
    );

    // A reused state namespace degrades.
    let mut input = clean_install_input();
    input.state_namespaces_isolated = false;
    assert_eq!(
        resolve_install_topology_entry(input)
            .unwrap()
            .degrade_reason,
        Some(M5InstallTopologyEntryDegradeReason::StateNamespaceReusedWithoutHandoff)
    );

    let mut input = clean_install_input();
    input.resolution_form_coverage = vec![M5InstallStateResolutionForm::CanonicalObject];
    assert_eq!(
        resolve_install_topology_entry(input)
            .unwrap()
            .degrade_reason,
        Some(M5InstallTopologyEntryDegradeReason::ResolutionFormCoverageIncomplete)
    );
}

#[test]
fn install_coexistence_and_surface_and_proof_degrade() {
    let mut input = clean_install_input();
    input.delivery_scope = M5DeliveryScope::SideBySideChannelScope;
    input.coexists_with_sibling_channel = true;
    input.coexistence_handoff_explained = false;
    // A coexisting channel with an unexplained handoff first fails namespace isolation.
    assert_eq!(
        resolve_install_topology_entry(input)
            .unwrap()
            .degrade_reason,
        Some(M5InstallTopologyEntryDegradeReason::StateNamespaceReusedWithoutHandoff)
    );

    let mut input = clean_install_input();
    input.surface_context = M5InstallSurfaceContext::ContextUnknown;
    assert_eq!(
        resolve_install_topology_entry(input)
            .unwrap()
            .degrade_reason,
        Some(M5InstallTopologyEntryDegradeReason::SurfaceContextUnresolved)
    );

    let mut input = clean_install_input();
    input.proof_fresh = false;
    assert_eq!(
        resolve_install_topology_entry(input)
            .unwrap()
            .degrade_reason,
        Some(M5InstallTopologyEntryDegradeReason::ProofStale)
    );
}

#[test]
fn install_empty_id_and_forbidden_material_error() {
    let mut input = clean_install_input();
    input.entry_id = "".to_owned();
    assert_eq!(
        resolve_install_topology_entry(input).unwrap_err(),
        M5InstallStateResolutionError::EmptyInstallTopologyEntryId
    );

    let mut input = clean_install_input();
    input.binary_root = "see https://relay.internal/leak".to_owned();
    assert_eq!(
        resolve_install_topology_entry(input).unwrap_err(),
        M5InstallStateResolutionError::ForbiddenMaterial
    );
}

#[test]
fn state_namespace_is_isolated_rejects_reuse() {
    assert!(state_namespace_is_isolated(
        M5DeliveryScope::PerUserManagedScope,
        true,
        false,
        true
    ));
    assert!(!state_namespace_is_isolated(
        M5DeliveryScope::PerUserManagedScope,
        false,
        false,
        true
    ));
    assert!(state_namespace_is_isolated(
        M5DeliveryScope::SideBySideChannelScope,
        true,
        true,
        true
    ));
    assert!(!state_namespace_is_isolated(
        M5DeliveryScope::SideBySideChannelScope,
        true,
        true,
        false
    ));
    assert!(!state_namespace_is_isolated(
        M5DeliveryScope::ScopeUnclassified,
        true,
        false,
        true
    ));
}

#[test]
fn install_topology_object_is_complete_requires_all_fields() {
    assert!(install_topology_object_is_complete(
        M5DeliveryScope::PerUserManagedScope,
        "stable",
        "per_user_updater",
        r"%LOCALAPPDATA%\Aureline\app",
        r"%LOCALAPPDATA%\Aureline\state",
        r"%LOCALAPPDATA%\Aureline\policy",
        "artifact-graph:per-user:stable",
    ));
    assert!(!install_topology_object_is_complete(
        M5DeliveryScope::PerUserManagedScope,
        "stable",
        "  ",
        r"%LOCALAPPDATA%\Aureline\app",
        r"%LOCALAPPDATA%\Aureline\state",
        r"%LOCALAPPDATA%\Aureline\policy",
        "artifact-graph:per-user:stable",
    ));
    assert!(!install_topology_object_is_complete(
        M5DeliveryScope::ScopeUnclassified,
        "stable",
        "per_user_updater",
        r"%LOCALAPPDATA%\Aureline\app",
        r"%LOCALAPPDATA%\Aureline\state",
        r"%LOCALAPPDATA%\Aureline\policy",
        "artifact-graph:per-user:stable",
    ));
}

#[test]
fn boundary_clean_stays_truthful() {
    let resolved = resolve_state_root_boundary_entry(clean_boundary_input()).unwrap();
    assert!(resolved.is_clean());
    assert!(resolved.boundary_truthful_on_every_profile);
    assert!(resolved.covers_all_resolution_forms);
    assert!(resolved.provides_complete_disclosure_triple);
    assert!(resolved.boundary_stays_truthful);
    assert_eq!(resolved.state_root_surface, "portable_mode_boundary");
    assert_eq!(resolved.surface_context, "about_surface");
}

#[test]
fn boundary_untruthful_and_unclassified_degrade() {
    // A used but undisclosed machine-global spill hides a downgrade — untruthful.
    let mut input = clean_boundary_input();
    input.machine_global_spill_used = true;
    input.machine_global_spill_disclosed = false;
    let resolved = resolve_state_root_boundary_entry(input).unwrap();
    assert!(!resolved.provides_complete_disclosure_triple);
    assert_eq!(
        resolved.degrade_reason,
        Some(M5StateRootBoundaryEntryDegradeReason::StateBoundaryUntruthfulOrIncomplete)
    );

    // A rollback that narrows below the full artifact graph is also untruthful / incomplete.
    let mut input = clean_boundary_input();
    input.rollback_targets_full_graph = false;
    assert_eq!(
        resolve_state_root_boundary_entry(input)
            .unwrap()
            .degrade_reason,
        Some(M5StateRootBoundaryEntryDegradeReason::StateBoundaryUntruthfulOrIncomplete)
    );

    // An unexplained narrower scope is also untruthful / incomplete.
    let mut input = clean_boundary_input();
    input.narrower_scope_asserted = true;
    input.narrower_scope_explained = false;
    assert_eq!(
        resolve_state_root_boundary_entry(input)
            .unwrap()
            .degrade_reason,
        Some(M5StateRootBoundaryEntryDegradeReason::StateBoundaryUntruthfulOrIncomplete)
    );

    let mut input = clean_boundary_input();
    input.state_root_surface = M5StateRootSurface::SurfaceUnclassified;
    assert_eq!(
        resolve_state_root_boundary_entry(input)
            .unwrap()
            .degrade_reason,
        Some(M5StateRootBoundaryEntryDegradeReason::StateRootSurfaceUnclassified)
    );
}

#[test]
fn boundary_form_and_surface_and_id_and_material() {
    let mut input = clean_boundary_input();
    input.resolution_form_coverage = vec![M5InstallStateResolutionForm::CanonicalObject];
    assert_eq!(
        resolve_state_root_boundary_entry(input)
            .unwrap()
            .degrade_reason,
        Some(M5StateRootBoundaryEntryDegradeReason::BoundaryFormCoverageIncomplete)
    );

    let mut input = clean_boundary_input();
    input.surface_context = M5InstallSurfaceContext::ContextUnknown;
    assert_eq!(
        resolve_state_root_boundary_entry(input)
            .unwrap()
            .degrade_reason,
        Some(M5StateRootBoundaryEntryDegradeReason::SurfaceContextUnresolved)
    );

    let mut input = clean_boundary_input();
    input.entry_id = "  ".to_owned();
    assert_eq!(
        resolve_state_root_boundary_entry(input).unwrap_err(),
        M5InstallStateResolutionError::EmptyStateRootBoundaryEntryId
    );

    let mut input = clean_boundary_input();
    input.writable_state_roots = "see internal://notes".to_owned();
    assert_eq!(
        resolve_state_root_boundary_entry(input).unwrap_err(),
        M5InstallStateResolutionError::ForbiddenMaterial
    );
}

#[test]
fn boundary_disclosed_spill_and_explained_scope_stay_clean() {
    // A disclosed machine-global spill stays truthful.
    let mut input = clean_boundary_input();
    input.machine_global_spill_used = true;
    input.machine_global_spill_disclosed = true;
    assert!(resolve_state_root_boundary_entry(input).unwrap().is_clean());

    // An explained narrower scope stays truthful.
    let mut input = clean_boundary_input();
    input.narrower_scope_asserted = true;
    input.narrower_scope_explained = true;
    assert!(resolve_state_root_boundary_entry(input).unwrap().is_clean());
}

#[test]
fn vocabulary_set_is_canonical() {
    assert!(seeded_m5_install_topology_and_state_root_registries()
        .vocabulary_set
        .matches_canonical());
}

#[test]
fn vocabulary_set_drift_fails() {
    let mut packet = seeded_m5_install_topology_and_state_root_registries();
    packet.vocabulary_set.delivery_scopes.pop();
    assert!(packet
        .validate()
        .contains(&M5InstallTopologyStateRootRegistriesViolation::VocabularySetDrift));
}

#[test]
fn missing_source_contracts_fails() {
    let mut packet = seeded_m5_install_topology_and_state_root_registries();
    packet.source_contract_refs.clear();
    assert!(packet
        .validate()
        .contains(&M5InstallTopologyStateRootRegistriesViolation::MissingSourceContracts));
}

#[test]
fn domain_schema_ref_missing_fails() {
    let mut packet = seeded_m5_install_topology_and_state_root_registries();
    packet.registry_rows[0]
        .source_contract_refs
        .retain(|r| r != M5_INSTALL_TOPOLOGY_DOMAIN_SCHEMA_REF);
    assert!(packet
        .validate()
        .contains(&M5InstallTopologyStateRootRegistriesViolation::DomainSchemaRefMissing));

    let mut packet = seeded_m5_install_topology_and_state_root_registries();
    packet.registry_rows[0]
        .source_contract_refs
        .retain(|r| r != M5_STATE_ROOT_BOUNDARIES_SCHEMA_REF);
    assert!(packet
        .validate()
        .contains(&M5InstallTopologyStateRootRegistriesViolation::DomainSchemaRefMissing));
}

#[test]
fn mandatory_anatomy_missing_fails() {
    let mut packet = seeded_m5_install_topology_and_state_root_registries();
    packet.registry_rows[0]
        .anatomy_parts
        .retain(|p| *p != M5InstallStateAnatomyPart::Identity);
    assert!(packet
        .validate()
        .contains(&M5InstallTopologyStateRootRegistriesViolation::MandatoryAnatomyMissing));
}

#[test]
fn mandatory_export_field_missing_fails() {
    let mut packet = seeded_m5_install_topology_and_state_root_registries();
    packet.registry_rows[0]
        .export_fields
        .retain(|f| *f != M5InstallStateExportField::DeliveryScopes);
    assert!(packet
        .validate()
        .contains(&M5InstallTopologyStateRootRegistriesViolation::MandatoryExportFieldMissing));
}

#[test]
fn examples_missing_fails() {
    let mut packet = seeded_m5_install_topology_and_state_root_registries();
    packet.registry_rows[0].state_root_boundary_entries.clear();
    assert!(packet
        .validate()
        .contains(&M5InstallTopologyStateRootRegistriesViolation::ExamplesMissing));
}

#[test]
fn dishonest_clean_example_fails() {
    let mut packet = seeded_m5_install_topology_and_state_root_registries();
    // Force a clean install entry to also read as object-incomplete — the packet must reject it.
    let row = &mut packet.registry_rows[0];
    row.install_topology_entries[0].degrade_reason = None;
    row.install_topology_entries[0].install_topology_object_complete = false;
    assert!(packet
        .validate()
        .contains(&M5InstallTopologyStateRootRegistriesViolation::DishonestExample));
}

#[test]
fn row_invariant_violation_fails() {
    for mutate in 0u8..4 {
        let mut packet = seeded_m5_install_topology_and_state_root_registries();
        let row = &mut packet.registry_rows[0];
        match mutate {
            0 => row.portable_mode_writes_hidden_machine_global_durable_state = true,
            1 => row.preview_channel_reuses_stable_state_namespace_without_handoff = true,
            2 => row.rollback_targets_primary_executable_while_sidecars_drift = true,
            _ => row.hides_updater_ownership_or_admin_control_in_managed_flow = true,
        }
        assert!(packet
            .validate()
            .contains(&M5InstallTopologyStateRootRegistriesViolation::RowInvariantViolated));
    }
}

#[test]
fn stable_object_not_proven_when_incomplete_example_removed() {
    let mut packet = seeded_m5_install_topology_and_state_root_registries();
    for row in &mut packet.registry_rows {
        row.install_topology_entries.retain(|ex| {
            ex.degrade_reason
                != Some(M5InstallTopologyEntryDegradeReason::InstallTopologyObjectIncomplete)
        });
    }
    assert!(packet
        .validate()
        .contains(&M5InstallTopologyStateRootRegistriesViolation::StableObjectResolutionNotProven));
}

#[test]
fn stable_object_not_proven_when_surface_collapses() {
    let mut packet = seeded_m5_install_topology_and_state_root_registries();
    // Drop every clean admin install so the first-consumer surfaces no longer include it.
    for row in &mut packet.registry_rows {
        row.install_topology_entries
            .retain(|ex| !(ex.is_clean() && ex.surface_context == "admin_surface"));
    }
    assert!(packet
        .validate()
        .contains(&M5InstallTopologyStateRootRegistriesViolation::StableObjectResolutionNotProven));
}

#[test]
fn namespace_isolation_not_proven_when_reused_example_removed() {
    let mut packet = seeded_m5_install_topology_and_state_root_registries();
    for row in &mut packet.registry_rows {
        row.install_topology_entries.retain(|ex| {
            ex.degrade_reason
                != Some(M5InstallTopologyEntryDegradeReason::StateNamespaceReusedWithoutHandoff)
        });
    }
    assert!(packet.validate().contains(
        &M5InstallTopologyStateRootRegistriesViolation::StateNamespaceIsolationNotProven
    ));
}

#[test]
fn namespace_isolation_not_proven_when_unbound_example_removed() {
    let mut packet = seeded_m5_install_topology_and_state_root_registries();
    for row in &mut packet.registry_rows {
        row.install_topology_entries.retain(|ex| {
            ex.degrade_reason
                != Some(M5InstallTopologyEntryDegradeReason::TopologyNotBoundToRegistry)
        });
    }
    assert!(packet.validate().contains(
        &M5InstallTopologyStateRootRegistriesViolation::StateNamespaceIsolationNotProven
    ));
}

#[test]
fn state_boundary_not_proven_when_untruthful_example_removed() {
    let mut packet = seeded_m5_install_topology_and_state_root_registries();
    for row in &mut packet.registry_rows {
        row.state_root_boundary_entries.retain(|ex| {
            ex.degrade_reason
                != Some(M5StateRootBoundaryEntryDegradeReason::StateBoundaryUntruthfulOrIncomplete)
        });
    }
    assert!(packet
        .validate()
        .contains(&M5InstallTopologyStateRootRegistriesViolation::StateBoundaryTruthNotProven));
}

#[test]
fn state_boundary_not_proven_when_surface_dropped() {
    let mut packet = seeded_m5_install_topology_and_state_root_registries();
    // Drop every clean diagnostics-inspection boundary so the coverage no longer includes it.
    for row in &mut packet.registry_rows {
        row.state_root_boundary_entries.retain(|ex| {
            !(ex.is_clean() && ex.state_root_surface == "diagnostics_inspection_boundary")
        });
    }
    assert!(packet
        .validate()
        .contains(&M5InstallTopologyStateRootRegistriesViolation::StateBoundaryTruthNotProven));
}

#[test]
fn governance_review_incomplete_fails() {
    let mut packet = seeded_m5_install_topology_and_state_root_registries();
    packet
        .governance_review
        .managed_and_user_scopes_and_channels_isolated = false;
    assert!(packet
        .validate()
        .contains(&M5InstallTopologyStateRootRegistriesViolation::GovernanceReviewIncomplete));
}

#[test]
fn consumer_projection_incomplete_fails() {
    let mut packet = seeded_m5_install_topology_and_state_root_registries();
    packet
        .consumer_projection
        .support_export_reads_single_registry_source = false;
    assert!(packet
        .validate()
        .contains(&M5InstallTopologyStateRootRegistriesViolation::ConsumerProjectionIncomplete));
}

#[test]
fn proof_freshness_incomplete_fails() {
    let mut packet = seeded_m5_install_topology_and_state_root_registries();
    packet.proof_freshness.proof_freshness_slo_hours = 0;
    assert!(packet
        .validate()
        .contains(&M5InstallTopologyStateRootRegistriesViolation::ProofFreshnessIncomplete));
}

#[test]
fn release_posture_incomplete_fails() {
    let mut packet = seeded_m5_install_topology_and_state_root_registries();
    packet.release_posture.accessibility_parity_required = false;
    assert!(packet
        .validate()
        .contains(&M5InstallTopologyStateRootRegistriesViolation::ReleasePostureIncomplete));
}

#[test]
fn injected_raw_material_is_rejected() {
    let mut packet = seeded_m5_install_topology_and_state_root_registries();
    packet.registry_rows[0].scope_summary =
        "raw endpoint https://relay.internal.example/session leaked".to_owned();
    assert!(packet
        .validate()
        .contains(&M5InstallTopologyStateRootRegistriesViolation::RawMaterialInExport));
}

#[test]
fn export_carries_no_forbidden_raw_material() {
    let json = seeded_m5_install_topology_and_state_root_registries().export_safe_json();
    let lower = json.to_lowercase();
    assert!(!lower.contains("password"));
    assert!(!lower.contains("passphrase"));
    assert!(!lower.contains("bearer "));
    assert!(!lower.contains("://"));
    assert!(!lower.contains("-----begin"));
}

#[test]
fn csv_has_a_row_per_consumer_surface() {
    let packet = seeded_m5_install_topology_and_state_root_registries();
    let csv = packet.render_matrix_csv();
    let lines: Vec<&str> = csv.lines().collect();
    assert_eq!(lines.len(), 1 + packet.registry_rows.len());
    assert!(lines[0].starts_with("consumer_surface,qualification,owner,"));
}

#[test]
fn markdown_summary_lists_every_consumer_surface() {
    let packet = seeded_m5_install_topology_and_state_root_registries();
    let summary = packet.render_markdown_summary();
    for row in &packet.registry_rows {
        assert!(summary.contains(row.consumer_surface.as_str()));
    }
}

#[test]
fn state_root_boundary_table_lists_only_clean_installs() {
    let packet = seeded_m5_install_topology_and_state_root_registries();
    let table = packet.render_state_root_boundary_table();
    // The clean per-user managed install is rendered from the registry.
    assert!(table.contains("per_user_managed_install"));
    assert!(table.contains("per_machine_managed_install"));
    // A degraded, incomplete entry never leaks into the generated table.
    assert!(!table.contains("incomplete"));
}

#[test]
fn checked_support_export_validates_and_matches_seed() {
    let from_disk = current_stable_m5_install_topology_and_state_root_registries_export()
        .expect("checked M5 install-topology / state-root registries export validates");
    assert_eq!(
        from_disk.packet_id,
        M5_INSTALL_TOPOLOGY_STATE_ROOT_REGISTRIES_PACKET_ID
    );
    assert_eq!(
        from_disk,
        seeded_m5_install_topology_and_state_root_registries(),
        "checked support export drifted from the seed builder"
    );
}

#[test]
fn narrowed_variants_validate_and_keep_rows_visible() {
    let beta =
        seeded_m5_install_topology_and_state_root_registries_side_by_side_channel_beta_narrowed();
    assert!(beta.validate().is_empty(), "{:?}", beta.validate());
    assert_eq!(beta.registry_rows.len(), 6);
    let row = beta
        .registry_rows
        .iter()
        .find(|r| r.consumer_surface == M5InstallTopologyConsumerSurface::Diagnostics)
        .unwrap();
    assert_eq!(row.qualification, M5InstallTopologyQualificationClass::Beta);

    let preview =
        seeded_m5_install_topology_and_state_root_registries_offline_airgap_bundle_preview_narrowed(
        );
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
    let beta: M5InstallTopologyStateRootRegistriesPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/install/m5-install-topology-and-state-root-registries/side_by_side_channel_beta_narrowed.json"
    )))
    .expect("side-by-side fixture parses");
    assert!(beta.validate().is_empty());
    assert_eq!(
        beta,
        seeded_m5_install_topology_and_state_root_registries_side_by_side_channel_beta_narrowed()
    );

    let preview: M5InstallTopologyStateRootRegistriesPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/install/m5-install-topology-and-state-root-registries/offline_airgap_bundle_preview_narrowed.json"
    )))
    .expect("offline fixture parses");
    assert!(preview.validate().is_empty());
    assert_eq!(
        preview,
        seeded_m5_install_topology_and_state_root_registries_offline_airgap_bundle_preview_narrowed(
        )
    );
}

#[test]
fn implemented_families_is_all_five_delivery_topologies() {
    assert_eq!(
        IMPLEMENTED_FAMILIES,
        [
            M5InstallTopologyFamily::PerUserManaged,
            M5InstallTopologyFamily::PerMachineManaged,
            M5InstallTopologyFamily::SideBySideStablePreview,
            M5InstallTopologyFamily::PortableMode,
            M5InstallTopologyFamily::OfflineAirgapBundle,
        ]
    );
}
