use super::*;

fn clean_authority_input() -> M5WorkspaceAuthorityEntryResolutionInput {
    M5WorkspaceAuthorityEntryResolutionInput {
        entry_id: "authority:test".to_owned(),
        workspace_id: "workspace.acme.single".to_owned(),
        token_name: "workspace.authority.single_window".to_owned(),
        semantic_role: M5WindowRestoreRole::WorkspaceAuthority,
        authority_scope: M5WorkspaceAuthorityScope::SingleWindowAuthorityScope,
        surface_context: M5WindowRestoreSurfaceContext::ShellSurface,
        resolution_form_coverage: M5WindowStateResolutionForm::ALL.to_vec(),
        backing_window_ids: "window.main".to_owned(),
        stable_pane_tree_ids: "pane-tree.main.v3".to_owned(),
        shared_dirty_buffer_state: "dirty-buffer.shared.0007".to_owned(),
        shared_save_checkpoint_state: "checkpoint.shared.0007".to_owned(),
        authority_state_root: "workspace-authority.acme/single".to_owned(),
        profile_defaults_ref: "profile-defaults.machine-hints".to_owned(),
        bound_to_registry: true,
        window_local_state_isolated: true,
        shares_authority_across_windows: false,
        window_local_history_preserved: true,
        proof_fresh: true,
    }
}

fn clean_topology_input() -> M5WindowTopologyEntryResolutionInput {
    M5WindowTopologyEntryResolutionInput {
        entry_id: "topology:test".to_owned(),
        window_id: "window.main".to_owned(),
        token_name: "window.topology.primary".to_owned(),
        semantic_role: M5WindowRestoreRole::WindowTopology,
        topology_surface: M5WindowTopologySurface::PrimaryWindowTopology,
        surface_context: M5WindowRestoreSurfaceContext::ShellSurface,
        resolution_form_coverage: M5WindowStateResolutionForm::ALL.to_vec(),
        window_local_pane_tree: "pane-tree.main.v4".to_owned(),
        window_local_focus_history: "focus-history.window.main".to_owned(),
        display_affinity_hint: "display-affinity.monitor-1".to_owned(),
        keeps_authority_distinct: true,
        topology_is_truthful: true,
        authority_copied_into_window_used: false,
        authority_copy_disclosed: false,
        profile_default_override_asserted: false,
        profile_default_override_explained: false,
        proof_fresh: true,
    }
}

#[test]
fn seeded_registries_validates() {
    let packet = seeded_m5_workspace_authority_and_window_topology_registries();
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
    assert_eq!(
        packet.packet_id,
        M5_WORKSPACE_AUTHORITY_WINDOW_TOPOLOGY_REGISTRIES_PACKET_ID
    );
}

#[test]
fn authority_clean_names_meaning_and_is_bound() {
    let resolved = resolve_workspace_authority_entry(clean_authority_input()).unwrap();
    assert!(resolved.is_clean());
    assert!(resolved.authority_resolves_across_workspaces);
    assert!(resolved.covers_all_resolution_forms);
    assert!(resolved.workspace_authority_object_complete);
    assert!(resolved.bound_to_registry);
    assert!(resolved.authority_scope_is_classified);
    assert!(resolved.window_local_state_isolated);
    assert_eq!(resolved.semantic_role, "workspace_authority");
    assert_eq!(resolved.authority_scope, "single_window_authority_scope");
    assert_eq!(resolved.canonical_authority_mode, "single_window_authority");
    assert_eq!(resolved.surface_context, "shell_surface");
    assert_eq!(
        resolved.next_action,
        M5WindowStateNextAction::ExpandOwnershipMeaning
    );
}

#[test]
fn authority_token_unstated_degrades() {
    let mut input = clean_authority_input();
    input.token_name = "   ".to_owned();
    assert_eq!(
        resolve_workspace_authority_entry(input)
            .unwrap()
            .degrade_reason,
        Some(M5WorkspaceAuthorityEntryDegradeReason::AuthorityTokenUnstated)
    );
}

#[test]
fn authority_unbound_and_unclassified_degrade() {
    let mut input = clean_authority_input();
    input.bound_to_registry = false;
    assert_eq!(
        resolve_workspace_authority_entry(input)
            .unwrap()
            .degrade_reason,
        Some(M5WorkspaceAuthorityEntryDegradeReason::AuthorityNotBoundToRegistry)
    );

    let mut input = clean_authority_input();
    input.authority_scope = M5WorkspaceAuthorityScope::ScopeUnclassified;
    assert_eq!(
        resolve_workspace_authority_entry(input)
            .unwrap()
            .degrade_reason,
        Some(M5WorkspaceAuthorityEntryDegradeReason::AuthorityScopeUnclassified)
    );
}

#[test]
fn authority_object_incomplete_and_overwrite_and_form_degrade() {
    // An unstated pane-tree ID leaves the resolved object incomplete.
    let mut input = clean_authority_input();
    input.stable_pane_tree_ids = "  ".to_owned();
    let resolved = resolve_workspace_authority_entry(input).unwrap();
    assert!(!resolved.workspace_authority_object_complete);
    assert_eq!(
        resolved.degrade_reason,
        Some(M5WorkspaceAuthorityEntryDegradeReason::WorkspaceAuthorityObjectIncomplete)
    );

    // A window-local overwrite of the shared authority degrades.
    let mut input = clean_authority_input();
    input.window_local_state_isolated = false;
    assert_eq!(
        resolve_workspace_authority_entry(input)
            .unwrap()
            .degrade_reason,
        Some(M5WorkspaceAuthorityEntryDegradeReason::WindowLocalStateOverwritesSharedAuthority)
    );

    let mut input = clean_authority_input();
    input.resolution_form_coverage = vec![M5WindowStateResolutionForm::CanonicalObject];
    assert_eq!(
        resolve_workspace_authority_entry(input)
            .unwrap()
            .degrade_reason,
        Some(M5WorkspaceAuthorityEntryDegradeReason::ResolutionFormCoverageIncomplete)
    );
}

#[test]
fn authority_shared_history_and_surface_and_proof_degrade() {
    let mut input = clean_authority_input();
    input.authority_scope = M5WorkspaceAuthorityScope::MultiWindowSharedAuthorityScope;
    input.shares_authority_across_windows = true;
    input.window_local_history_preserved = false;
    // A multi-window authority losing its window-local history first fails window-local isolation.
    assert_eq!(
        resolve_workspace_authority_entry(input)
            .unwrap()
            .degrade_reason,
        Some(M5WorkspaceAuthorityEntryDegradeReason::WindowLocalStateOverwritesSharedAuthority)
    );

    let mut input = clean_authority_input();
    input.surface_context = M5WindowRestoreSurfaceContext::ContextUnknown;
    assert_eq!(
        resolve_workspace_authority_entry(input)
            .unwrap()
            .degrade_reason,
        Some(M5WorkspaceAuthorityEntryDegradeReason::SurfaceContextUnresolved)
    );

    let mut input = clean_authority_input();
    input.proof_fresh = false;
    assert_eq!(
        resolve_workspace_authority_entry(input)
            .unwrap()
            .degrade_reason,
        Some(M5WorkspaceAuthorityEntryDegradeReason::ProofStale)
    );
}

#[test]
fn authority_empty_id_and_forbidden_material_error() {
    let mut input = clean_authority_input();
    input.entry_id = "".to_owned();
    assert_eq!(
        resolve_workspace_authority_entry(input).unwrap_err(),
        M5WindowStateResolutionError::EmptyWorkspaceAuthorityEntryId
    );

    let mut input = clean_authority_input();
    input.authority_state_root = "see https://relay.internal/leak".to_owned();
    assert_eq!(
        resolve_workspace_authority_entry(input).unwrap_err(),
        M5WindowStateResolutionError::ForbiddenMaterial
    );
}

#[test]
fn window_local_state_stays_window_local_rejects_overwrite() {
    assert!(window_local_state_stays_window_local(
        M5WorkspaceAuthorityScope::SingleWindowAuthorityScope,
        true,
        false,
        true
    ));
    assert!(!window_local_state_stays_window_local(
        M5WorkspaceAuthorityScope::SingleWindowAuthorityScope,
        false,
        false,
        true
    ));
    assert!(window_local_state_stays_window_local(
        M5WorkspaceAuthorityScope::MultiWindowSharedAuthorityScope,
        true,
        true,
        true
    ));
    assert!(!window_local_state_stays_window_local(
        M5WorkspaceAuthorityScope::MultiWindowSharedAuthorityScope,
        true,
        true,
        false
    ));
    assert!(!window_local_state_stays_window_local(
        M5WorkspaceAuthorityScope::ScopeUnclassified,
        true,
        false,
        true
    ));
}

#[test]
fn workspace_authority_object_is_complete_requires_all_fields() {
    assert!(workspace_authority_object_is_complete(
        M5WorkspaceAuthorityScope::SingleWindowAuthorityScope,
        "window.main",
        "pane-tree.main.v3",
        "dirty-buffer.shared.0007",
        "checkpoint.shared.0007",
        "workspace-authority.acme/single",
        "profile-defaults.machine-hints",
    ));
    assert!(!workspace_authority_object_is_complete(
        M5WorkspaceAuthorityScope::SingleWindowAuthorityScope,
        "window.main",
        "  ",
        "dirty-buffer.shared.0007",
        "checkpoint.shared.0007",
        "workspace-authority.acme/single",
        "profile-defaults.machine-hints",
    ));
    assert!(!workspace_authority_object_is_complete(
        M5WorkspaceAuthorityScope::ScopeUnclassified,
        "window.main",
        "pane-tree.main.v3",
        "dirty-buffer.shared.0007",
        "checkpoint.shared.0007",
        "workspace-authority.acme/single",
        "profile-defaults.machine-hints",
    ));
}

#[test]
fn topology_clean_stays_distinct() {
    let resolved = resolve_window_topology_entry(clean_topology_input()).unwrap();
    assert!(resolved.is_clean());
    assert!(resolved.topology_distinct_on_every_window);
    assert!(resolved.covers_all_resolution_forms);
    assert!(resolved.provides_complete_disclosure_triple);
    assert!(resolved.topology_stays_distinct);
    assert_eq!(resolved.topology_surface, "primary_window_topology");
    assert_eq!(resolved.surface_context, "shell_surface");
}

#[test]
fn topology_leaked_and_unclassified_degrade() {
    // A used but undisclosed authority copy leaks shared authority into private window state.
    let mut input = clean_topology_input();
    input.authority_copied_into_window_used = true;
    input.authority_copy_disclosed = false;
    let resolved = resolve_window_topology_entry(input).unwrap();
    assert!(!resolved.provides_complete_disclosure_triple);
    assert_eq!(
        resolved.degrade_reason,
        Some(M5WindowTopologyEntryDegradeReason::WindowTopologyMergesOrLeaksSharedAuthority)
    );

    // A topology that no longer keeps shared authority distinct is also a merge / leak.
    let mut input = clean_topology_input();
    input.keeps_authority_distinct = false;
    assert_eq!(
        resolve_window_topology_entry(input).unwrap().degrade_reason,
        Some(M5WindowTopologyEntryDegradeReason::WindowTopologyMergesOrLeaksSharedAuthority)
    );

    // An unexplained profile-default override is also a merge / leak.
    let mut input = clean_topology_input();
    input.profile_default_override_asserted = true;
    input.profile_default_override_explained = false;
    assert_eq!(
        resolve_window_topology_entry(input).unwrap().degrade_reason,
        Some(M5WindowTopologyEntryDegradeReason::WindowTopologyMergesOrLeaksSharedAuthority)
    );

    let mut input = clean_topology_input();
    input.topology_surface = M5WindowTopologySurface::SurfaceUnclassified;
    assert_eq!(
        resolve_window_topology_entry(input).unwrap().degrade_reason,
        Some(M5WindowTopologyEntryDegradeReason::WindowTopologySurfaceUnclassified)
    );
}

#[test]
fn topology_form_and_surface_and_id_and_material() {
    let mut input = clean_topology_input();
    input.resolution_form_coverage = vec![M5WindowStateResolutionForm::CanonicalObject];
    assert_eq!(
        resolve_window_topology_entry(input).unwrap().degrade_reason,
        Some(M5WindowTopologyEntryDegradeReason::TopologyFormCoverageIncomplete)
    );

    let mut input = clean_topology_input();
    input.surface_context = M5WindowRestoreSurfaceContext::ContextUnknown;
    assert_eq!(
        resolve_window_topology_entry(input).unwrap().degrade_reason,
        Some(M5WindowTopologyEntryDegradeReason::SurfaceContextUnresolved)
    );

    let mut input = clean_topology_input();
    input.entry_id = "  ".to_owned();
    assert_eq!(
        resolve_window_topology_entry(input).unwrap_err(),
        M5WindowStateResolutionError::EmptyWindowTopologyEntryId
    );

    let mut input = clean_topology_input();
    input.window_local_pane_tree = "see internal://notes".to_owned();
    assert_eq!(
        resolve_window_topology_entry(input).unwrap_err(),
        M5WindowStateResolutionError::ForbiddenMaterial
    );
}

#[test]
fn topology_disclosed_copy_and_explained_override_stay_clean() {
    // A disclosed read-only copy of shared authority state stays distinct.
    let mut input = clean_topology_input();
    input.authority_copied_into_window_used = true;
    input.authority_copy_disclosed = true;
    assert!(resolve_window_topology_entry(input).unwrap().is_clean());

    // An explained profile-default override stays distinct.
    let mut input = clean_topology_input();
    input.profile_default_override_asserted = true;
    input.profile_default_override_explained = true;
    assert!(resolve_window_topology_entry(input).unwrap().is_clean());
}

#[test]
fn vocabulary_set_is_canonical() {
    assert!(
        seeded_m5_workspace_authority_and_window_topology_registries()
            .vocabulary_set
            .matches_canonical()
    );
}

#[test]
fn vocabulary_set_drift_fails() {
    let mut packet = seeded_m5_workspace_authority_and_window_topology_registries();
    packet.vocabulary_set.authority_scopes.pop();
    assert!(packet
        .validate()
        .contains(&M5WorkspaceAuthorityWindowTopologyRegistriesViolation::VocabularySetDrift));
}

#[test]
fn missing_source_contracts_fails() {
    let mut packet = seeded_m5_workspace_authority_and_window_topology_registries();
    packet.source_contract_refs.clear();
    assert!(packet
        .validate()
        .contains(&M5WorkspaceAuthorityWindowTopologyRegistriesViolation::MissingSourceContracts));
}

#[test]
fn domain_schema_ref_missing_fails() {
    let mut packet = seeded_m5_workspace_authority_and_window_topology_registries();
    packet.registry_rows[0]
        .source_contract_refs
        .retain(|r| r != M5_WINDOW_TOPOLOGY_DOMAIN_SCHEMA_REF);
    assert!(packet
        .validate()
        .contains(&M5WorkspaceAuthorityWindowTopologyRegistriesViolation::DomainSchemaRefMissing));

    let mut packet = seeded_m5_workspace_authority_and_window_topology_registries();
    packet.registry_rows[0]
        .source_contract_refs
        .retain(|r| r != M5_RESTORE_FIDELITY_SCHEMA_REF);
    assert!(packet
        .validate()
        .contains(&M5WorkspaceAuthorityWindowTopologyRegistriesViolation::DomainSchemaRefMissing));
}

#[test]
fn mandatory_anatomy_missing_fails() {
    let mut packet = seeded_m5_workspace_authority_and_window_topology_registries();
    packet.registry_rows[0]
        .anatomy_parts
        .retain(|p| *p != M5WindowStateAnatomyPart::Identity);
    assert!(packet
        .validate()
        .contains(&M5WorkspaceAuthorityWindowTopologyRegistriesViolation::MandatoryAnatomyMissing));
}

#[test]
fn mandatory_export_field_missing_fails() {
    let mut packet = seeded_m5_workspace_authority_and_window_topology_registries();
    packet.registry_rows[0]
        .export_fields
        .retain(|f| *f != M5WindowStateExportField::WorkspaceAuthorityScopes);
    assert!(packet.validate().contains(
        &M5WorkspaceAuthorityWindowTopologyRegistriesViolation::MandatoryExportFieldMissing
    ));
}

#[test]
fn examples_missing_fails() {
    let mut packet = seeded_m5_workspace_authority_and_window_topology_registries();
    packet.registry_rows[0].window_topology_entries.clear();
    assert!(packet
        .validate()
        .contains(&M5WorkspaceAuthorityWindowTopologyRegistriesViolation::ExamplesMissing));
}

#[test]
fn dishonest_clean_example_fails() {
    let mut packet = seeded_m5_workspace_authority_and_window_topology_registries();
    // Force a clean authority entry to also read as object-incomplete — the packet must reject it.
    let row = &mut packet.registry_rows[0];
    row.workspace_authority_entries[0].degrade_reason = None;
    row.workspace_authority_entries[0].workspace_authority_object_complete = false;
    assert!(packet
        .validate()
        .contains(&M5WorkspaceAuthorityWindowTopologyRegistriesViolation::DishonestExample));
}

#[test]
fn row_invariant_violation_fails() {
    for mutate in 0u8..4 {
        let mut packet = seeded_m5_workspace_authority_and_window_topology_registries();
        let row = &mut packet.registry_rows[0];
        match mutate {
            0 => row.window_local_state_overwrites_shared_workspace_authority = true,
            1 => row.shared_workspace_authority_becomes_private_window_state = true,
            2 => row.merges_workspace_authority_and_window_topology_into_one_opaque_blob = true,
            _ => row.dirty_buffer_state_drifts_across_windows_sharing_one_authority = true,
        }
        assert!(packet.validate().contains(
            &M5WorkspaceAuthorityWindowTopologyRegistriesViolation::RowInvariantViolated
        ));
    }
}

#[test]
fn stable_object_not_proven_when_incomplete_example_removed() {
    let mut packet = seeded_m5_workspace_authority_and_window_topology_registries();
    for row in &mut packet.registry_rows {
        row.workspace_authority_entries.retain(|ex| {
            ex.degrade_reason
                != Some(M5WorkspaceAuthorityEntryDegradeReason::WorkspaceAuthorityObjectIncomplete)
        });
    }
    assert!(packet.validate().contains(
        &M5WorkspaceAuthorityWindowTopologyRegistriesViolation::StableObjectResolutionNotProven
    ));
}

#[test]
fn stable_object_not_proven_when_surface_collapses() {
    let mut packet = seeded_m5_workspace_authority_and_window_topology_registries();
    // Drop every clean admin authority so the first-consumer surfaces no longer include it.
    for row in &mut packet.registry_rows {
        row.workspace_authority_entries
            .retain(|ex| !(ex.is_clean() && ex.surface_context == "admin_surface"));
    }
    assert!(packet.validate().contains(
        &M5WorkspaceAuthorityWindowTopologyRegistriesViolation::StableObjectResolutionNotProven
    ));
}

#[test]
fn window_local_isolation_not_proven_when_overwrite_example_removed() {
    let mut packet = seeded_m5_workspace_authority_and_window_topology_registries();
    for row in &mut packet.registry_rows {
        row.workspace_authority_entries.retain(|ex| {
            ex.degrade_reason
                != Some(
                    M5WorkspaceAuthorityEntryDegradeReason::WindowLocalStateOverwritesSharedAuthority,
                )
        });
    }
    assert!(packet.validate().contains(
        &M5WorkspaceAuthorityWindowTopologyRegistriesViolation::WindowLocalIsolationNotProven
    ));
}

#[test]
fn window_local_isolation_not_proven_when_unbound_example_removed() {
    let mut packet = seeded_m5_workspace_authority_and_window_topology_registries();
    for row in &mut packet.registry_rows {
        row.workspace_authority_entries.retain(|ex| {
            ex.degrade_reason
                != Some(M5WorkspaceAuthorityEntryDegradeReason::AuthorityNotBoundToRegistry)
        });
    }
    assert!(packet.validate().contains(
        &M5WorkspaceAuthorityWindowTopologyRegistriesViolation::WindowLocalIsolationNotProven
    ));
}

#[test]
fn topology_distinctness_not_proven_when_leaked_example_removed() {
    let mut packet = seeded_m5_workspace_authority_and_window_topology_registries();
    for row in &mut packet.registry_rows {
        row.window_topology_entries.retain(|ex| {
            ex.degrade_reason
                != Some(
                    M5WindowTopologyEntryDegradeReason::WindowTopologyMergesOrLeaksSharedAuthority,
                )
        });
    }
    assert!(packet.validate().contains(
        &M5WorkspaceAuthorityWindowTopologyRegistriesViolation::TopologyDistinctnessNotProven
    ));
}

#[test]
fn topology_distinctness_not_proven_when_surface_dropped() {
    let mut packet = seeded_m5_workspace_authority_and_window_topology_registries();
    // Drop every clean diagnostics-inspection topology so the coverage no longer includes it.
    for row in &mut packet.registry_rows {
        row.window_topology_entries.retain(|ex| {
            !(ex.is_clean() && ex.topology_surface == "diagnostics_inspection_topology")
        });
    }
    assert!(packet.validate().contains(
        &M5WorkspaceAuthorityWindowTopologyRegistriesViolation::TopologyDistinctnessNotProven
    ));
}

#[test]
fn governance_review_incomplete_fails() {
    let mut packet = seeded_m5_workspace_authority_and_window_topology_registries();
    packet
        .governance_review
        .window_local_selection_and_focus_stay_window_local = false;
    assert!(packet.validate().contains(
        &M5WorkspaceAuthorityWindowTopologyRegistriesViolation::GovernanceReviewIncomplete
    ));
}

#[test]
fn consumer_projection_incomplete_fails() {
    let mut packet = seeded_m5_workspace_authority_and_window_topology_registries();
    packet
        .consumer_projection
        .support_export_reads_single_registry_source = false;
    assert!(packet.validate().contains(
        &M5WorkspaceAuthorityWindowTopologyRegistriesViolation::ConsumerProjectionIncomplete
    ));
}

#[test]
fn proof_freshness_incomplete_fails() {
    let mut packet = seeded_m5_workspace_authority_and_window_topology_registries();
    packet.proof_freshness.proof_freshness_slo_hours = 0;
    assert!(packet.validate().contains(
        &M5WorkspaceAuthorityWindowTopologyRegistriesViolation::ProofFreshnessIncomplete
    ));
}

#[test]
fn release_posture_incomplete_fails() {
    let mut packet = seeded_m5_workspace_authority_and_window_topology_registries();
    packet.release_posture.accessibility_parity_required = false;
    assert!(packet.validate().contains(
        &M5WorkspaceAuthorityWindowTopologyRegistriesViolation::ReleasePostureIncomplete
    ));
}

#[test]
fn injected_raw_material_is_rejected() {
    let mut packet = seeded_m5_workspace_authority_and_window_topology_registries();
    packet.registry_rows[0].scope_summary =
        "raw endpoint https://relay.internal.example/session leaked".to_owned();
    assert!(packet
        .validate()
        .contains(&M5WorkspaceAuthorityWindowTopologyRegistriesViolation::RawMaterialInExport));
}

#[test]
fn export_carries_no_forbidden_raw_material() {
    let json = seeded_m5_workspace_authority_and_window_topology_registries().export_safe_json();
    let lower = json.to_lowercase();
    assert!(!lower.contains("password"));
    assert!(!lower.contains("passphrase"));
    assert!(!lower.contains("bearer "));
    assert!(!lower.contains("://"));
    assert!(!lower.contains("-----begin"));
}

#[test]
fn csv_has_a_row_per_consumer_surface() {
    let packet = seeded_m5_workspace_authority_and_window_topology_registries();
    let csv = packet.render_matrix_csv();
    let lines: Vec<&str> = csv.lines().collect();
    assert_eq!(lines.len(), 1 + packet.registry_rows.len());
    assert!(lines[0].starts_with("consumer_surface,qualification,owner,"));
}

#[test]
fn markdown_summary_lists_every_consumer_surface() {
    let packet = seeded_m5_workspace_authority_and_window_topology_registries();
    let summary = packet.render_markdown_summary();
    for row in &packet.registry_rows {
        assert!(summary.contains(row.consumer_surface.as_str()));
    }
}

#[test]
fn workspace_ownership_table_lists_only_clean_authorities() {
    let packet = seeded_m5_workspace_authority_and_window_topology_registries();
    let table = packet.render_workspace_ownership_table();
    // The clean single-window and multi-window authorities are rendered from the registry.
    assert!(table.contains("single_window_authority"));
    assert!(table.contains("multi_window_shared_authority"));
    // A degraded, incomplete entry never leaks into the generated table.
    assert!(!table.contains("incomplete"));
}

#[test]
fn checked_support_export_validates_and_matches_seed() {
    let from_disk = current_stable_m5_workspace_authority_and_window_topology_registries_export()
        .expect("checked M5 workspace-authority / window-topology registries export validates");
    assert_eq!(
        from_disk.packet_id,
        M5_WORKSPACE_AUTHORITY_WINDOW_TOPOLOGY_REGISTRIES_PACKET_ID
    );
    assert_eq!(
        from_disk,
        seeded_m5_workspace_authority_and_window_topology_registries(),
        "checked support export drifted from the seed builder"
    );
}

#[test]
fn narrowed_variants_validate_and_keep_rows_visible() {
    let beta =
        seeded_m5_workspace_authority_and_window_topology_registries_multi_window_shared_authority_beta_narrowed();
    assert!(beta.validate().is_empty(), "{:?}", beta.validate());
    assert_eq!(beta.registry_rows.len(), 6);
    let row = beta
        .registry_rows
        .iter()
        .find(|r| r.consumer_surface == M5WindowRestoreConsumerSurface::RestoreCoordinator)
        .unwrap();
    assert_eq!(row.qualification, M5WindowRestoreQualificationClass::Beta);

    let preview =
        seeded_m5_workspace_authority_and_window_topology_registries_auxiliary_window_topology_preview_narrowed();
    assert!(preview.validate().is_empty(), "{:?}", preview.validate());
    assert_eq!(preview.registry_rows.len(), 6);
    let row = preview
        .registry_rows
        .iter()
        .find(|r| r.consumer_surface == M5WindowRestoreConsumerSurface::Diagnostics)
        .unwrap();
    assert_eq!(
        row.qualification,
        M5WindowRestoreQualificationClass::Preview
    );
}

#[test]
fn checked_narrowed_fixtures_validate_and_match_seed_builders() {
    let beta: M5WorkspaceAuthorityWindowTopologyRegistriesPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/ui/m5-workspace-authority-and-window-topology-registries/multi_window_shared_authority_beta_narrowed.json"
    )))
    .expect("multi-window fixture parses");
    assert!(beta.validate().is_empty());
    assert_eq!(
        beta,
        seeded_m5_workspace_authority_and_window_topology_registries_multi_window_shared_authority_beta_narrowed()
    );

    let preview: M5WorkspaceAuthorityWindowTopologyRegistriesPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/ui/m5-workspace-authority-and-window-topology-registries/auxiliary_window_topology_preview_narrowed.json"
    )))
    .expect("auxiliary fixture parses");
    assert!(preview.validate().is_empty());
    assert_eq!(
        preview,
        seeded_m5_workspace_authority_and_window_topology_registries_auxiliary_window_topology_preview_narrowed()
    );
}

#[test]
fn implemented_families_is_shared_authority_and_window_local_topology() {
    assert_eq!(
        IMPLEMENTED_FAMILIES,
        [
            M5WindowRestoreFamily::SharedWorkspaceAuthority,
            M5WindowRestoreFamily::WindowLocalTopology,
        ]
    );
}
