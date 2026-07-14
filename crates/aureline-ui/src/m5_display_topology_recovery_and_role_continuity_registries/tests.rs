use super::*;

fn clean_bounds_input() -> M5BoundsRecoveryEntryResolutionInput {
    M5BoundsRecoveryEntryResolutionInput {
        entry_id: "bounds:test".to_owned(),
        remap_target_id: "window.acme.editor-main".to_owned(),
        token_name: "bounds.recovery.affinity_monitor_restored".to_owned(),
        semantic_role: M5WindowRestoreRole::DisplayAffinity,
        bounds_recovery_state: M5BoundsRecoveryState::AffinityMonitorRestored,
        surface_context: M5DisplayTopologyOrchestrationSurfaceContext::ShellSurface,
        resolution_form_coverage: M5DisplayTopologyOrchestrationResolutionForm::ALL.to_vec(),
        window_surface_id: "window-surface.editor.main".to_owned(),
        affinity_monitor_hint: "affinity.monitor.primary".to_owned(),
        resolved_visible_bounds: "bounds.visible.primary-1440p".to_owned(),
        layout_intent: "layout-intent.split-editor".to_owned(),
        provenance_class: "provenance.live-layout".to_owned(),
        keyboard_reach_plan: "keyboard-reach.focus-cycle".to_owned(),
        bound_to_registry: true,
        bounds_resolved_before_present: true,
        is_material_topology_adjustment: false,
        topology_adjustment_recorded_when_material: true,
        proof_fresh: true,
    }
}

fn clean_fence_input() -> M5RoleContinuityFenceEntryResolutionInput {
    M5RoleContinuityFenceEntryResolutionInput {
        entry_id: "fence:test".to_owned(),
        guarded_window_id: "window.presentation.main".to_owned(),
        token_name: "fence.follow.no_reset".to_owned(),
        semantic_role: M5WindowRestoreRole::DisplayAffinity,
        role_class: M5RoleContinuityClass::FollowOrPresentationState,
        surface_context: M5DisplayTopologyOrchestrationSurfaceContext::ShellSurface,
        resolution_form_coverage: M5DisplayTopologyOrchestrationResolutionForm::ALL.to_vec(),
        preserved_role_label: "role-label.presenter-following".to_owned(),
        boundary_label: "boundary.presentation-window".to_owned(),
        provenance_hint: "provenance.live-layout".to_owned(),
        preserves_role_and_boundary: true,
        fence_is_truthful: true,
        role_was_present_used: false,
        role_preserved_after_remap: false,
        fidelity_reduced: false,
        fidelity_reduction_disclosed: false,
        proof_fresh: true,
    }
}

#[test]
fn seeded_registries_validates() {
    let packet = seeded_m5_display_topology_recovery_and_role_continuity_registries();
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
    assert_eq!(
        packet.packet_id,
        M5_DISPLAY_TOPOLOGY_RECOVERY_AND_ROLE_CONTINUITY_REGISTRIES_PACKET_ID
    );
}

#[test]
fn bounds_clean_names_meaning_and_is_bound() {
    let resolved = resolve_bounds_recovery_entry(clean_bounds_input()).unwrap();
    assert!(resolved.is_clean());
    assert!(resolved.bounds_resolve_across_remaps);
    assert!(resolved.covers_all_resolution_forms);
    assert!(resolved.bounds_recovery_object_complete);
    assert!(resolved.bound_to_registry);
    assert!(resolved.bounds_recovery_state_is_classified);
    assert!(resolved.bounds_resolved_before_present);
    assert_eq!(resolved.semantic_role, "display_affinity");
    assert_eq!(resolved.bounds_recovery_state, "affinity_monitor_restored");
    assert_eq!(
        resolved.canonical_bounds_recovery_mode,
        "affinity_monitor_restored"
    );
    assert_eq!(resolved.surface_context, "shell_surface");
    assert_eq!(
        resolved.next_action,
        M5DisplayTopologyOrchestrationNextAction::ExpandLayoutMeaning
    );
}

#[test]
fn bounds_token_unstated_degrades() {
    let mut input = clean_bounds_input();
    input.token_name = "   ".to_owned();
    assert_eq!(
        resolve_bounds_recovery_entry(input).unwrap().degrade_reason,
        Some(M5BoundsRecoveryEntryDegradeReason::BoundsTokenUnstated)
    );
}

#[test]
fn bounds_unbound_and_unclassified_degrade() {
    let mut input = clean_bounds_input();
    input.bound_to_registry = false;
    assert_eq!(
        resolve_bounds_recovery_entry(input).unwrap().degrade_reason,
        Some(M5BoundsRecoveryEntryDegradeReason::BoundsNotBoundToRegistry)
    );

    let mut input = clean_bounds_input();
    input.bounds_recovery_state = M5BoundsRecoveryState::BoundsUnclassified;
    assert_eq!(
        resolve_bounds_recovery_entry(input).unwrap().degrade_reason,
        Some(M5BoundsRecoveryEntryDegradeReason::BoundsRecoveryStateUnclassified)
    );
}

#[test]
fn bounds_object_incomplete_and_present_first_and_form_degrade() {
    // An unstated layout intent leaves the resolved object incomplete.
    let mut input = clean_bounds_input();
    input.layout_intent = "  ".to_owned();
    let resolved = resolve_bounds_recovery_entry(input).unwrap();
    assert!(!resolved.bounds_recovery_object_complete);
    assert_eq!(
        resolved.degrade_reason,
        Some(M5BoundsRecoveryEntryDegradeReason::BoundsRecoveryObjectIncomplete)
    );

    // A window that was presented before its bounds were resolved onto visible bounds degrades.
    let mut input = clean_bounds_input();
    input.bounds_resolved_before_present = false;
    assert_eq!(
        resolve_bounds_recovery_entry(input).unwrap().degrade_reason,
        Some(M5BoundsRecoveryEntryDegradeReason::PresentPrecededBounds)
    );

    let mut input = clean_bounds_input();
    input.resolution_form_coverage =
        vec![M5DisplayTopologyOrchestrationResolutionForm::CanonicalObject];
    assert_eq!(
        resolve_bounds_recovery_entry(input).unwrap().degrade_reason,
        Some(M5BoundsRecoveryEntryDegradeReason::ResolutionFormCoverageIncomplete)
    );
}

#[test]
fn bounds_unrecorded_adjustment_and_surface_and_proof_degrade() {
    let mut input = clean_bounds_input();
    input.bounds_recovery_state = M5BoundsRecoveryState::ClampedOntoVisibleBounds;
    input.is_material_topology_adjustment = true;
    input.topology_adjustment_recorded_when_material = false;
    // A material adjustment that hides the topology change first fails bounds-precede-present.
    assert_eq!(
        resolve_bounds_recovery_entry(input).unwrap().degrade_reason,
        Some(M5BoundsRecoveryEntryDegradeReason::PresentPrecededBounds)
    );

    let mut input = clean_bounds_input();
    input.surface_context = M5DisplayTopologyOrchestrationSurfaceContext::ContextUnknown;
    assert_eq!(
        resolve_bounds_recovery_entry(input).unwrap().degrade_reason,
        Some(M5BoundsRecoveryEntryDegradeReason::SurfaceContextUnresolved)
    );

    let mut input = clean_bounds_input();
    input.proof_fresh = false;
    assert_eq!(
        resolve_bounds_recovery_entry(input).unwrap().degrade_reason,
        Some(M5BoundsRecoveryEntryDegradeReason::ProofStale)
    );
}

#[test]
fn bounds_empty_id_and_forbidden_material_error() {
    let mut input = clean_bounds_input();
    input.entry_id = "".to_owned();
    assert_eq!(
        resolve_bounds_recovery_entry(input).unwrap_err(),
        M5DisplayTopologyOrchestrationResolutionError::EmptyBoundsRecoveryEntryId
    );

    let mut input = clean_bounds_input();
    input.provenance_class = "see https://relay.internal/leak".to_owned();
    assert_eq!(
        resolve_bounds_recovery_entry(input).unwrap_err(),
        M5DisplayTopologyOrchestrationResolutionError::ForbiddenMaterial
    );
}

#[test]
fn bounds_precede_present_rejects_present_first() {
    assert!(bounds_precede_present(
        M5BoundsRecoveryState::AffinityMonitorRestored,
        true,
        false,
        true
    ));
    assert!(!bounds_precede_present(
        M5BoundsRecoveryState::AffinityMonitorRestored,
        false,
        false,
        true
    ));
    assert!(bounds_precede_present(
        M5BoundsRecoveryState::ClampedOntoVisibleBounds,
        true,
        true,
        true
    ));
    assert!(!bounds_precede_present(
        M5BoundsRecoveryState::ClampedOntoVisibleBounds,
        true,
        true,
        false
    ));
    assert!(!bounds_precede_present(
        M5BoundsRecoveryState::BoundsUnclassified,
        true,
        false,
        true
    ));
}

#[test]
fn bounds_recovery_object_is_complete_requires_all_fields() {
    assert!(bounds_recovery_object_is_complete(
        M5BoundsRecoveryState::AffinityMonitorRestored,
        "window-surface.editor.main",
        "affinity.monitor.primary",
        "bounds.visible.primary-1440p",
        "layout-intent.split-editor",
        "provenance.live-layout",
        "keyboard-reach.focus-cycle",
    ));
    assert!(!bounds_recovery_object_is_complete(
        M5BoundsRecoveryState::AffinityMonitorRestored,
        "window-surface.editor.main",
        "  ",
        "bounds.visible.primary-1440p",
        "layout-intent.split-editor",
        "provenance.live-layout",
        "keyboard-reach.focus-cycle",
    ));
    assert!(!bounds_recovery_object_is_complete(
        M5BoundsRecoveryState::BoundsUnclassified,
        "window-surface.editor.main",
        "affinity.monitor.primary",
        "bounds.visible.primary-1440p",
        "layout-intent.split-editor",
        "provenance.live-layout",
        "keyboard-reach.focus-cycle",
    ));
}

#[test]
fn fence_clean_holds_no_reset() {
    let resolved = resolve_role_continuity_fence_entry(clean_fence_input()).unwrap();
    assert!(resolved.is_clean());
    assert!(resolved.fence_holds_on_every_surface);
    assert!(resolved.covers_all_resolution_forms);
    assert!(resolved.provides_complete_disclosure_triple);
    assert!(resolved.fence_holds_no_reset);
    assert_eq!(resolved.role_class, "follow_or_presentation_state");
    assert_eq!(resolved.surface_context, "shell_surface");
}

#[test]
fn fence_resets_and_unclassified_degrade() {
    // A role that was present before the remap and not preserved is a reset into a generic window.
    let mut input = clean_fence_input();
    input.role_was_present_used = true;
    input.role_preserved_after_remap = false;
    let resolved = resolve_role_continuity_fence_entry(input).unwrap();
    assert!(!resolved.provides_complete_disclosure_triple);
    assert_eq!(
        resolved.degrade_reason,
        Some(M5RoleContinuityEntryDegradeReason::RoleContinuityResetsOrOverclaims)
    );

    // A fence that no longer preserves the role label and boundary is also a reset / overclaim.
    let mut input = clean_fence_input();
    input.preserves_role_and_boundary = false;
    assert_eq!(
        resolve_role_continuity_fence_entry(input)
            .unwrap()
            .degrade_reason,
        Some(M5RoleContinuityEntryDegradeReason::RoleContinuityResetsOrOverclaims)
    );

    // A hidden reduced fidelity is also a reset / overclaim.
    let mut input = clean_fence_input();
    input.fidelity_reduced = true;
    input.fidelity_reduction_disclosed = false;
    assert_eq!(
        resolve_role_continuity_fence_entry(input)
            .unwrap()
            .degrade_reason,
        Some(M5RoleContinuityEntryDegradeReason::RoleContinuityResetsOrOverclaims)
    );

    let mut input = clean_fence_input();
    input.role_class = M5RoleContinuityClass::RoleClassUnclassified;
    assert_eq!(
        resolve_role_continuity_fence_entry(input)
            .unwrap()
            .degrade_reason,
        Some(M5RoleContinuityEntryDegradeReason::RoleContinuityClassUnclassified)
    );
}

#[test]
fn fence_form_and_surface_and_id_and_material() {
    let mut input = clean_fence_input();
    input.resolution_form_coverage =
        vec![M5DisplayTopologyOrchestrationResolutionForm::CanonicalObject];
    assert_eq!(
        resolve_role_continuity_fence_entry(input)
            .unwrap()
            .degrade_reason,
        Some(M5RoleContinuityEntryDegradeReason::RoleFormCoverageIncomplete)
    );

    let mut input = clean_fence_input();
    input.surface_context = M5DisplayTopologyOrchestrationSurfaceContext::ContextUnknown;
    assert_eq!(
        resolve_role_continuity_fence_entry(input)
            .unwrap()
            .degrade_reason,
        Some(M5RoleContinuityEntryDegradeReason::SurfaceContextUnresolved)
    );

    let mut input = clean_fence_input();
    input.entry_id = "  ".to_owned();
    assert_eq!(
        resolve_role_continuity_fence_entry(input).unwrap_err(),
        M5DisplayTopologyOrchestrationResolutionError::EmptyRoleContinuityFenceEntryId
    );

    let mut input = clean_fence_input();
    input.preserved_role_label = "see internal://notes".to_owned();
    assert_eq!(
        resolve_role_continuity_fence_entry(input).unwrap_err(),
        M5DisplayTopologyOrchestrationResolutionError::ForbiddenMaterial
    );
}

#[test]
fn fence_preserved_role_and_disclosed_fidelity_stay_clean() {
    // A preserved present role holds no-reset.
    let mut input = clean_fence_input();
    input.role_was_present_used = true;
    input.role_preserved_after_remap = true;
    assert!(resolve_role_continuity_fence_entry(input)
        .unwrap()
        .is_clean());

    // A disclosed reduced fidelity holds no-reset.
    let mut input = clean_fence_input();
    input.fidelity_reduced = true;
    input.fidelity_reduction_disclosed = true;
    assert!(resolve_role_continuity_fence_entry(input)
        .unwrap()
        .is_clean());
}

#[test]
fn vocabulary_set_is_canonical() {
    assert!(
        seeded_m5_display_topology_recovery_and_role_continuity_registries()
            .vocabulary_set
            .matches_canonical()
    );
}

#[test]
fn vocabulary_set_drift_fails() {
    let mut packet = seeded_m5_display_topology_recovery_and_role_continuity_registries();
    packet.vocabulary_set.bounds_recovery_states.pop();
    assert!(packet.validate().contains(
        &M5DisplayTopologyRecoveryAndRoleContinuityRegistriesViolation::VocabularySetDrift
    ));
}

#[test]
fn missing_source_contracts_fails() {
    let mut packet = seeded_m5_display_topology_recovery_and_role_continuity_registries();
    packet.source_contract_refs.clear();
    assert!(packet.validate().contains(
        &M5DisplayTopologyRecoveryAndRoleContinuityRegistriesViolation::MissingSourceContracts
    ));
}

#[test]
fn domain_schema_ref_missing_fails() {
    let mut packet = seeded_m5_display_topology_recovery_and_role_continuity_registries();
    packet.registry_rows[0]
        .source_contract_refs
        .retain(|r| r != M5_RESTORE_FIDELITY_SCHEMA_REF);
    assert!(packet.validate().contains(
        &M5DisplayTopologyRecoveryAndRoleContinuityRegistriesViolation::DomainSchemaRefMissing
    ));

    let mut packet = seeded_m5_display_topology_recovery_and_role_continuity_registries();
    packet.registry_rows[0]
        .source_contract_refs
        .retain(|r| r != M5_WINDOW_TOPOLOGY_DOMAIN_SCHEMA_REF);
    assert!(packet.validate().contains(
        &M5DisplayTopologyRecoveryAndRoleContinuityRegistriesViolation::DomainSchemaRefMissing
    ));
}

#[test]
fn mandatory_anatomy_missing_fails() {
    let mut packet = seeded_m5_display_topology_recovery_and_role_continuity_registries();
    packet.registry_rows[0]
        .anatomy_parts
        .retain(|p| *p != M5DisplayTopologyOrchestrationAnatomyPart::Identity);
    assert!(packet.validate().contains(
        &M5DisplayTopologyRecoveryAndRoleContinuityRegistriesViolation::MandatoryAnatomyMissing
    ));
}

#[test]
fn mandatory_export_field_missing_fails() {
    let mut packet = seeded_m5_display_topology_recovery_and_role_continuity_registries();
    packet.registry_rows[0]
        .export_fields
        .retain(|f| *f != M5DisplayTopologyOrchestrationExportField::BoundsRecoveryStates);
    assert!(packet.validate().contains(
        &M5DisplayTopologyRecoveryAndRoleContinuityRegistriesViolation::MandatoryExportFieldMissing
    ));
}

#[test]
fn examples_missing_fails() {
    let mut packet = seeded_m5_display_topology_recovery_and_role_continuity_registries();
    packet.registry_rows[0]
        .role_continuity_fence_entries
        .clear();
    assert!(packet
        .validate()
        .contains(&M5DisplayTopologyRecoveryAndRoleContinuityRegistriesViolation::ExamplesMissing));
}

#[test]
fn dishonest_clean_example_fails() {
    let mut packet = seeded_m5_display_topology_recovery_and_role_continuity_registries();
    // Force a clean bounds entry to also read as object-incomplete — the packet must reject it.
    let row = &mut packet.registry_rows[0];
    row.bounds_recovery_entries[0].degrade_reason = None;
    row.bounds_recovery_entries[0].bounds_recovery_object_complete = false;
    assert!(packet.validate().contains(
        &M5DisplayTopologyRecoveryAndRoleContinuityRegistriesViolation::DishonestExample
    ));
}

#[test]
fn row_invariant_violation_fails() {
    for mutate in 0u8..4 {
        let mut packet = seeded_m5_display_topology_recovery_and_role_continuity_registries();
        let row = &mut packet.registry_rows[0];
        match mutate {
            0 => row.strands_window_or_dialog_offscreen_after_remap = true,
            1 => row.resets_auxiliary_window_into_generic_window = true,
            2 => row.merges_bounds_recovery_and_role_continuity_into_one_opaque_blob = true,
            _ => row.overclaims_layout_fidelity_when_only_bounds_or_context_recovered = true,
        }
        assert!(packet.validate().contains(
            &M5DisplayTopologyRecoveryAndRoleContinuityRegistriesViolation::RowInvariantViolated
        ));
    }
}

#[test]
fn bounds_recovery_not_proven_when_incomplete_example_removed() {
    let mut packet = seeded_m5_display_topology_recovery_and_role_continuity_registries();
    for row in &mut packet.registry_rows {
        row.bounds_recovery_entries.retain(|ex| {
            ex.degrade_reason
                != Some(M5BoundsRecoveryEntryDegradeReason::BoundsRecoveryObjectIncomplete)
        });
    }
    assert!(packet.validate().contains(
        &M5DisplayTopologyRecoveryAndRoleContinuityRegistriesViolation::BoundsRecoveryResolutionNotProven
    ));
}

#[test]
fn bounds_recovery_not_proven_when_surface_collapses() {
    let mut packet = seeded_m5_display_topology_recovery_and_role_continuity_registries();
    // Drop every clean admin bounds entry so the first-consumer surfaces no longer include it.
    for row in &mut packet.registry_rows {
        row.bounds_recovery_entries
            .retain(|ex| !(ex.is_clean() && ex.surface_context == "admin_surface"));
    }
    assert!(packet.validate().contains(
        &M5DisplayTopologyRecoveryAndRoleContinuityRegistriesViolation::BoundsRecoveryResolutionNotProven
    ));
}

#[test]
fn bounds_before_present_not_proven_when_present_first_example_removed() {
    let mut packet = seeded_m5_display_topology_recovery_and_role_continuity_registries();
    for row in &mut packet.registry_rows {
        row.bounds_recovery_entries.retain(|ex| {
            ex.degrade_reason != Some(M5BoundsRecoveryEntryDegradeReason::PresentPrecededBounds)
        });
    }
    assert!(packet.validate().contains(
        &M5DisplayTopologyRecoveryAndRoleContinuityRegistriesViolation::BoundsBeforePresentNotProven
    ));
}

#[test]
fn bounds_before_present_not_proven_when_unbound_example_removed() {
    let mut packet = seeded_m5_display_topology_recovery_and_role_continuity_registries();
    for row in &mut packet.registry_rows {
        row.bounds_recovery_entries.retain(|ex| {
            ex.degrade_reason != Some(M5BoundsRecoveryEntryDegradeReason::BoundsNotBoundToRegistry)
        });
    }
    assert!(packet.validate().contains(
        &M5DisplayTopologyRecoveryAndRoleContinuityRegistriesViolation::BoundsBeforePresentNotProven
    ));
}

#[test]
fn role_continuity_not_proven_when_resets_example_removed() {
    let mut packet = seeded_m5_display_topology_recovery_and_role_continuity_registries();
    for row in &mut packet.registry_rows {
        row.role_continuity_fence_entries.retain(|ex| {
            ex.degrade_reason
                != Some(M5RoleContinuityEntryDegradeReason::RoleContinuityResetsOrOverclaims)
        });
    }
    assert!(packet.validate().contains(
        &M5DisplayTopologyRecoveryAndRoleContinuityRegistriesViolation::RoleContinuityNotProven
    ));
}

#[test]
fn role_continuity_not_proven_when_class_dropped() {
    let mut packet = seeded_m5_display_topology_recovery_and_role_continuity_registries();
    // Drop every clean auxiliary-purpose fence so the coverage no longer includes it.
    for row in &mut packet.registry_rows {
        row.role_continuity_fence_entries
            .retain(|ex| !(ex.is_clean() && ex.role_class == "auxiliary_window_purpose"));
    }
    assert!(packet.validate().contains(
        &M5DisplayTopologyRecoveryAndRoleContinuityRegistriesViolation::RoleContinuityNotProven
    ));
}

#[test]
fn governance_review_incomplete_fails() {
    let mut packet = seeded_m5_display_topology_recovery_and_role_continuity_registries();
    packet
        .governance_review
        .bounds_resolved_before_surface_presented = false;
    assert!(packet.validate().contains(
        &M5DisplayTopologyRecoveryAndRoleContinuityRegistriesViolation::GovernanceReviewIncomplete
    ));
}

#[test]
fn consumer_projection_incomplete_fails() {
    let mut packet = seeded_m5_display_topology_recovery_and_role_continuity_registries();
    packet
        .consumer_projection
        .support_export_reads_single_registry_source = false;
    assert!(packet.validate().contains(
        &M5DisplayTopologyRecoveryAndRoleContinuityRegistriesViolation::ConsumerProjectionIncomplete
    ));
}

#[test]
fn proof_freshness_incomplete_fails() {
    let mut packet = seeded_m5_display_topology_recovery_and_role_continuity_registries();
    packet.proof_freshness.proof_freshness_slo_hours = 0;
    assert!(packet.validate().contains(
        &M5DisplayTopologyRecoveryAndRoleContinuityRegistriesViolation::ProofFreshnessIncomplete
    ));
}

#[test]
fn release_posture_incomplete_fails() {
    let mut packet = seeded_m5_display_topology_recovery_and_role_continuity_registries();
    packet.release_posture.accessibility_parity_required = false;
    assert!(packet.validate().contains(
        &M5DisplayTopologyRecoveryAndRoleContinuityRegistriesViolation::ReleasePostureIncomplete
    ));
}

#[test]
fn injected_raw_material_is_rejected() {
    let mut packet = seeded_m5_display_topology_recovery_and_role_continuity_registries();
    packet.registry_rows[0].scope_summary =
        "raw endpoint https://relay.internal.example/session leaked".to_owned();
    assert!(packet.validate().contains(
        &M5DisplayTopologyRecoveryAndRoleContinuityRegistriesViolation::RawMaterialInExport
    ));
}

#[test]
fn export_carries_no_forbidden_raw_material() {
    let json =
        seeded_m5_display_topology_recovery_and_role_continuity_registries().export_safe_json();
    let lower = json.to_lowercase();
    assert!(!lower.contains("password"));
    assert!(!lower.contains("passphrase"));
    assert!(!lower.contains("bearer "));
    assert!(!lower.contains("://"));
    assert!(!lower.contains("-----begin"));
}

#[test]
fn csv_has_a_row_per_consumer_surface() {
    let packet = seeded_m5_display_topology_recovery_and_role_continuity_registries();
    let csv = packet.render_matrix_csv();
    let lines: Vec<&str> = csv.lines().collect();
    assert_eq!(lines.len(), 1 + packet.registry_rows.len());
    assert!(lines[0].starts_with("consumer_surface,qualification,owner,"));
}

#[test]
fn markdown_summary_lists_every_consumer_surface() {
    let packet = seeded_m5_display_topology_recovery_and_role_continuity_registries();
    let summary = packet.render_markdown_summary();
    for row in &packet.registry_rows {
        assert!(summary.contains(row.consumer_surface.as_str()));
    }
}

#[test]
fn bounds_recovery_table_lists_only_clean_bounds() {
    let packet = seeded_m5_display_topology_recovery_and_role_continuity_registries();
    let table = packet.render_bounds_recovery_table();
    // The clean affinity and clamped recoveries are rendered from the registry.
    assert!(table.contains("affinity_monitor_restored"));
    assert!(table.contains("clamped_onto_visible_bounds"));
    // A degraded, incomplete entry never leaks into the generated table.
    assert!(!table.contains("incomplete"));
}

#[test]
fn checked_support_export_validates_and_matches_seed() {
    let from_disk =
        current_stable_m5_display_topology_recovery_and_role_continuity_registries_export()
            .expect("checked M5 bounds-recovery / role-continuity registries export validates");
    assert_eq!(
        from_disk.packet_id,
        M5_DISPLAY_TOPOLOGY_RECOVERY_AND_ROLE_CONTINUITY_REGISTRIES_PACKET_ID
    );
    assert_eq!(
        from_disk,
        seeded_m5_display_topology_recovery_and_role_continuity_registries(),
        "checked support export drifted from the seed builder"
    );
}

#[test]
fn narrowed_variants_validate_and_keep_rows_visible() {
    let beta =
        seeded_m5_display_topology_recovery_and_role_continuity_registries_dpi_rescale_beta_narrowed();
    assert!(beta.validate().is_empty(), "{:?}", beta.validate());
    assert_eq!(beta.registry_rows.len(), 6);
    let row = beta
        .registry_rows
        .iter()
        .find(|r| r.consumer_surface == M5WindowRestoreConsumerSurface::RestoreCoordinator)
        .unwrap();
    assert_eq!(row.qualification, M5WindowRestoreQualificationClass::Beta);

    let preview =
        seeded_m5_display_topology_recovery_and_role_continuity_registries_reduced_fidelity_preview_narrowed();
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
    let beta: M5DisplayTopologyRecoveryAndRoleContinuityRegistriesPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/ui/m5-display-topology-recovery-and-role-continuity-registries/dpi_rescale_beta_narrowed.json"
    )))
    .expect("dpi-rescale fixture parses");
    assert!(beta.validate().is_empty());
    assert_eq!(
        beta,
        seeded_m5_display_topology_recovery_and_role_continuity_registries_dpi_rescale_beta_narrowed()
    );

    let preview: M5DisplayTopologyRecoveryAndRoleContinuityRegistriesPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/ui/m5-display-topology-recovery-and-role-continuity-registries/reduced_fidelity_preview_narrowed.json"
    )))
    .expect("reduced-fidelity fixture parses");
    assert!(preview.validate().is_empty());
    assert_eq!(
        preview,
        seeded_m5_display_topology_recovery_and_role_continuity_registries_reduced_fidelity_preview_narrowed()
    );
}

#[test]
fn implemented_families_is_display_topology_recovery() {
    assert_eq!(
        IMPLEMENTED_FAMILIES,
        [M5WindowRestoreFamily::DisplayTopologyRecovery]
    );
}
