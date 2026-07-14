use super::*;

fn clean_layout_input() -> M5PortableStateLayoutEntryResolutionInput {
    M5PortableStateLayoutEntryResolutionInput {
        entry_id: "layout:test".to_owned(),
        profile_id: "profile.portable_colocated".to_owned(),
        token_name: "portable.layout.colocated".to_owned(),
        semantic_role: M5InstallTopologyRole::WritableStateRoots,
        containment: M5PortableStateContainment::ColocatedUnderExecutable,
        surface_context: M5PortableSurfaceContext::AboutSurface,
        presentation_form_coverage: M5PortablePresentationForm::ALL.to_vec(),
        executable_root: r".\AurelinePortable\app".to_owned(),
        colocated_state_root: r".\AurelinePortable\state".to_owned(),
        log_and_crash_root: r".\AurelinePortable\logs".to_owned(),
        durable_classes_covered: M5PortableDurableStateClass::ALL.to_vec(),
        state_origin: M5PortableStateOrigin::PortableColocated,
        bound_to_registry: true,
        hidden_machine_global_write_used: false,
        hidden_machine_global_write_blocked: true,
        proof_fresh: true,
    }
}

fn clean_diagnostics_input() -> M5PortableDiagnosticsEntryResolutionInput {
    M5PortableDiagnosticsEntryResolutionInput {
        entry_id: "diagnostics:test".to_owned(),
        profile_id: "profile.portable_colocated".to_owned(),
        token_name: "portable.diagnostics.card".to_owned(),
        semantic_role: M5InstallTopologyRole::WritableStateRoots,
        diagnostics_surface: M5PortableDiagnosticsSurface::PortableDiagnosticsCard,
        surface_context: M5PortableSurfaceContext::AboutSurface,
        presentation_form_coverage: M5PortablePresentationForm::ALL.to_vec(),
        executable_root: r".\AurelinePortable\app".to_owned(),
        state_roots: r".\AurelinePortable\state".to_owned(),
        disclosed_fields: M5PortableDiagnosticsField::ALL.to_vec(),
        update_posture: M5PortableUpdatePosture::ManualReplace,
        update_continuity_documented: true,
        unsupported_shell_paths_disclosed: true,
        proof_fresh: true,
    }
}

#[test]
fn seeded_registries_validates() {
    let packet = seeded_m5_portable_mode_state_containment_and_diagnostics();
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
    assert_eq!(
        packet.packet_id,
        M5_PORTABLE_MODE_STATE_CONTAINMENT_AND_DIAGNOSTICS_PACKET_ID
    );
}

#[test]
fn layout_clean_names_meaning_and_is_bound() {
    let resolved = resolve_portable_state_layout_entry(clean_layout_input()).unwrap();
    assert!(resolved.is_clean());
    assert!(resolved.layout_resolves_across_profiles);
    assert!(resolved.covers_all_presentation_forms);
    assert!(resolved.durable_root_inventory_complete);
    assert!(resolved.layout_is_contained);
    assert!(resolved.bound_to_registry);
    assert!(resolved.containment_is_colocated_or_sibling);
    assert!(resolved.state_origin_is_distinguishable);
    assert_eq!(resolved.semantic_role, "writable_state_roots");
    assert_eq!(resolved.containment, "colocated_under_executable");
    assert_eq!(resolved.state_origin, "portable_colocated");
    assert_eq!(resolved.surface_context, "about_surface");
    assert_eq!(
        resolved.next_action,
        M5PortableNextAction::ExpandPortableMeaning
    );
}

#[test]
fn layout_token_unstated_degrades() {
    let mut input = clean_layout_input();
    input.token_name = "   ".to_owned();
    assert_eq!(
        resolve_portable_state_layout_entry(input)
            .unwrap()
            .degrade_reason,
        Some(M5PortableStateLayoutEntryDegradeReason::LayoutTokenUnstated)
    );
}

#[test]
fn layout_unbound_and_unclassified_degrade() {
    let mut input = clean_layout_input();
    input.bound_to_registry = false;
    assert_eq!(
        resolve_portable_state_layout_entry(input)
            .unwrap()
            .degrade_reason,
        Some(M5PortableStateLayoutEntryDegradeReason::LayoutNotBoundToRegistry)
    );

    let mut input = clean_layout_input();
    input.containment = M5PortableStateContainment::ContainmentUnclassified;
    assert_eq!(
        resolve_portable_state_layout_entry(input)
            .unwrap()
            .degrade_reason,
        Some(M5PortableStateLayoutEntryDegradeReason::ContainmentUnclassified)
    );

    // A hidden-machine-global containment is disallowed too.
    let mut input = clean_layout_input();
    input.containment = M5PortableStateContainment::HiddenMachineGlobal;
    assert_eq!(
        resolve_portable_state_layout_entry(input)
            .unwrap()
            .degrade_reason,
        Some(M5PortableStateLayoutEntryDegradeReason::ContainmentUnclassified)
    );
}

#[test]
fn layout_inventory_and_spill_and_origin_and_form_degrade() {
    // A dropped mandatory durable class leaves the inventory incomplete.
    let mut input = clean_layout_input();
    input.durable_classes_covered = vec![
        M5PortableDurableStateClass::DurableSettings,
        M5PortableDurableStateClass::StoredSecrets,
        M5PortableDurableStateClass::BackgroundServices,
    ];
    let resolved = resolve_portable_state_layout_entry(input).unwrap();
    assert!(!resolved.durable_root_inventory_complete);
    assert_eq!(
        resolved.degrade_reason,
        Some(M5PortableStateLayoutEntryDegradeReason::DurableRootInventoryIncomplete)
    );

    // A hidden machine-global spill degrades.
    let mut input = clean_layout_input();
    input.hidden_machine_global_write_used = true;
    input.hidden_machine_global_write_blocked = false;
    assert_eq!(
        resolve_portable_state_layout_entry(input)
            .unwrap()
            .degrade_reason,
        Some(M5PortableStateLayoutEntryDegradeReason::HiddenMachineGlobalDurableSpill)
    );

    // An unblocked (unproven) spill posture degrades even without an actual write.
    let mut input = clean_layout_input();
    input.hidden_machine_global_write_blocked = false;
    assert_eq!(
        resolve_portable_state_layout_entry(input)
            .unwrap()
            .degrade_reason,
        Some(M5PortableStateLayoutEntryDegradeReason::HiddenMachineGlobalDurableSpill)
    );

    // An ambiguous state origin degrades.
    let mut input = clean_layout_input();
    input.state_origin = M5PortableStateOrigin::OriginAmbiguous;
    assert_eq!(
        resolve_portable_state_layout_entry(input)
            .unwrap()
            .degrade_reason,
        Some(M5PortableStateLayoutEntryDegradeReason::StateOriginAmbiguous)
    );

    let mut input = clean_layout_input();
    input.presentation_form_coverage = vec![M5PortablePresentationForm::CanonicalObject];
    assert_eq!(
        resolve_portable_state_layout_entry(input)
            .unwrap()
            .degrade_reason,
        Some(M5PortableStateLayoutEntryDegradeReason::PresentationFormCoverageIncomplete)
    );
}

#[test]
fn layout_surface_and_proof_degrade() {
    let mut input = clean_layout_input();
    input.surface_context = M5PortableSurfaceContext::ContextUnknown;
    assert_eq!(
        resolve_portable_state_layout_entry(input)
            .unwrap()
            .degrade_reason,
        Some(M5PortableStateLayoutEntryDegradeReason::SurfaceContextUnresolved)
    );

    let mut input = clean_layout_input();
    input.proof_fresh = false;
    assert_eq!(
        resolve_portable_state_layout_entry(input)
            .unwrap()
            .degrade_reason,
        Some(M5PortableStateLayoutEntryDegradeReason::ProofStale)
    );
}

#[test]
fn layout_empty_id_and_forbidden_material_error() {
    let mut input = clean_layout_input();
    input.entry_id = "".to_owned();
    assert_eq!(
        resolve_portable_state_layout_entry(input).unwrap_err(),
        M5PortableResolutionError::EmptyLayoutEntryId
    );

    let mut input = clean_layout_input();
    input.executable_root = "see https://relay.internal/leak".to_owned();
    assert_eq!(
        resolve_portable_state_layout_entry(input).unwrap_err(),
        M5PortableResolutionError::ForbiddenMaterial
    );
}

#[test]
fn portable_layout_is_contained_requires_no_spill_and_full_inventory() {
    assert!(portable_layout_is_contained(
        M5PortableStateContainment::ColocatedUnderExecutable,
        r".\AurelinePortable\app",
        r".\AurelinePortable\state",
        r".\AurelinePortable\logs",
        &M5PortableDurableStateClass::ALL,
        false,
        true,
    ));
    // A hidden machine-global write breaks containment.
    assert!(!portable_layout_is_contained(
        M5PortableStateContainment::ColocatedUnderExecutable,
        r".\AurelinePortable\app",
        r".\AurelinePortable\state",
        r".\AurelinePortable\logs",
        &M5PortableDurableStateClass::ALL,
        true,
        true,
    ));
    // An unblocked write posture is not proven contained.
    assert!(!portable_layout_is_contained(
        M5PortableStateContainment::ColocatedUnderExecutable,
        r".\AurelinePortable\app",
        r".\AurelinePortable\state",
        r".\AurelinePortable\logs",
        &M5PortableDurableStateClass::ALL,
        false,
        false,
    ));
    // A hidden machine-global containment is never contained.
    assert!(!portable_layout_is_contained(
        M5PortableStateContainment::HiddenMachineGlobal,
        r".\AurelinePortable\app",
        r".\AurelinePortable\state",
        r".\AurelinePortable\logs",
        &M5PortableDurableStateClass::ALL,
        false,
        true,
    ));
}

#[test]
fn diagnostics_clean_is_discoverable_and_continuous() {
    let resolved = resolve_portable_diagnostics_entry(clean_diagnostics_input()).unwrap();
    assert!(resolved.is_clean());
    assert!(resolved.diagnostics_discoverable_on_every_profile);
    assert!(resolved.covers_all_presentation_forms);
    assert!(resolved.diagnostics_is_discoverable);
    assert!(resolved.update_is_continuous);
    assert_eq!(resolved.diagnostics_surface, "portable_diagnostics_card");
    assert_eq!(resolved.update_posture, "manual_replace");
}

#[test]
fn diagnostics_disclosure_and_continuity_and_surface_degrade() {
    // A dropped mandatory field leaves the disclosure incomplete.
    let mut input = clean_diagnostics_input();
    input.disclosed_fields = vec![
        M5PortableDiagnosticsField::ExecutableRoot,
        M5PortableDiagnosticsField::StateRoots,
        M5PortableDiagnosticsField::LogAndCrashLocations,
        M5PortableDiagnosticsField::UpdatePosture,
    ];
    let resolved = resolve_portable_diagnostics_entry(input).unwrap();
    assert!(!resolved.diagnostics_is_discoverable);
    assert_eq!(
        resolved.degrade_reason,
        Some(M5PortableDiagnosticsEntryDegradeReason::DiagnosticsDisclosureIncomplete)
    );

    // An undisclosed unsupported shell path is also an incomplete disclosure.
    let mut input = clean_diagnostics_input();
    input.unsupported_shell_paths_disclosed = false;
    assert_eq!(
        resolve_portable_diagnostics_entry(input)
            .unwrap()
            .degrade_reason,
        Some(M5PortableDiagnosticsEntryDegradeReason::DiagnosticsDisclosureIncomplete)
    );

    // An undocumented continuity note degrades.
    let mut input = clean_diagnostics_input();
    input.update_continuity_documented = false;
    assert_eq!(
        resolve_portable_diagnostics_entry(input)
            .unwrap()
            .degrade_reason,
        Some(M5PortableDiagnosticsEntryDegradeReason::UpdateContinuityUndocumented)
    );

    // An unclassified update posture also breaks continuity.
    let mut input = clean_diagnostics_input();
    input.update_posture = M5PortableUpdatePosture::PostureUnclassified;
    assert_eq!(
        resolve_portable_diagnostics_entry(input)
            .unwrap()
            .degrade_reason,
        Some(M5PortableDiagnosticsEntryDegradeReason::UpdateContinuityUndocumented)
    );

    let mut input = clean_diagnostics_input();
    input.diagnostics_surface = M5PortableDiagnosticsSurface::SurfaceUnclassified;
    assert_eq!(
        resolve_portable_diagnostics_entry(input)
            .unwrap()
            .degrade_reason,
        Some(M5PortableDiagnosticsEntryDegradeReason::DiagnosticsSurfaceUnclassified)
    );
}

#[test]
fn diagnostics_form_and_id_and_material() {
    let mut input = clean_diagnostics_input();
    input.presentation_form_coverage = vec![M5PortablePresentationForm::CanonicalObject];
    assert_eq!(
        resolve_portable_diagnostics_entry(input)
            .unwrap()
            .degrade_reason,
        Some(M5PortableDiagnosticsEntryDegradeReason::DiagnosticsFormCoverageIncomplete)
    );

    let mut input = clean_diagnostics_input();
    input.entry_id = "  ".to_owned();
    assert_eq!(
        resolve_portable_diagnostics_entry(input).unwrap_err(),
        M5PortableResolutionError::EmptyDiagnosticsEntryId
    );

    let mut input = clean_diagnostics_input();
    input.state_roots = "see internal://notes".to_owned();
    assert_eq!(
        resolve_portable_diagnostics_entry(input).unwrap_err(),
        M5PortableResolutionError::ForbiddenMaterial
    );
}

#[test]
fn vocabulary_set_is_canonical() {
    assert!(seeded_m5_portable_mode_state_containment_and_diagnostics()
        .vocabulary_set
        .matches_canonical());
}

#[test]
fn vocabulary_set_drift_fails() {
    let mut packet = seeded_m5_portable_mode_state_containment_and_diagnostics();
    packet.vocabulary_set.containments.pop();
    assert!(packet
        .validate()
        .contains(&M5PortableModeStateContainmentAndDiagnosticsViolation::VocabularySetDrift));
}

#[test]
fn missing_source_contracts_fails() {
    let mut packet = seeded_m5_portable_mode_state_containment_and_diagnostics();
    packet.source_contract_refs.clear();
    assert!(packet
        .validate()
        .contains(&M5PortableModeStateContainmentAndDiagnosticsViolation::MissingSourceContracts));
}

#[test]
fn domain_schema_ref_missing_fails() {
    let mut packet = seeded_m5_portable_mode_state_containment_and_diagnostics();
    packet.registry_rows[0]
        .source_contract_refs
        .retain(|r| r != M5_STATE_ROOT_BOUNDARIES_SCHEMA_REF);
    assert!(packet
        .validate()
        .contains(&M5PortableModeStateContainmentAndDiagnosticsViolation::DomainSchemaRefMissing));
}

#[test]
fn mandatory_anatomy_missing_fails() {
    let mut packet = seeded_m5_portable_mode_state_containment_and_diagnostics();
    packet.registry_rows[0]
        .anatomy_parts
        .retain(|p| *p != M5PortableAnatomyPart::Identity);
    assert!(packet
        .validate()
        .contains(&M5PortableModeStateContainmentAndDiagnosticsViolation::MandatoryAnatomyMissing));
}

#[test]
fn mandatory_export_field_missing_fails() {
    let mut packet = seeded_m5_portable_mode_state_containment_and_diagnostics();
    packet.registry_rows[0]
        .export_fields
        .retain(|f| *f != M5PortableExportField::Containments);
    assert!(packet.validate().contains(
        &M5PortableModeStateContainmentAndDiagnosticsViolation::MandatoryExportFieldMissing
    ));
}

#[test]
fn examples_missing_fails() {
    let mut packet = seeded_m5_portable_mode_state_containment_and_diagnostics();
    packet.registry_rows[0].portable_diagnostics_entries.clear();
    assert!(packet
        .validate()
        .contains(&M5PortableModeStateContainmentAndDiagnosticsViolation::ExamplesMissing));
}

#[test]
fn dishonest_clean_example_fails() {
    let mut packet = seeded_m5_portable_mode_state_containment_and_diagnostics();
    // Force a clean layout entry to also read as inventory-incomplete — the packet must reject it.
    let row = &mut packet.registry_rows[0];
    row.portable_state_layout_entries[0].degrade_reason = None;
    row.portable_state_layout_entries[0].durable_root_inventory_complete = false;
    assert!(packet
        .validate()
        .contains(&M5PortableModeStateContainmentAndDiagnosticsViolation::DishonestExample));
}

#[test]
fn row_invariant_violation_fails() {
    for mutate in 0u8..4 {
        let mut packet = seeded_m5_portable_mode_state_containment_and_diagnostics();
        let row = &mut packet.registry_rows[0];
        match mutate {
            0 => row.portable_mode_writes_hidden_machine_global_durable_state = true,
            1 => row.portable_state_indistinguishable_from_installed_state = true,
            2 => row.portable_update_drops_retained_state_without_notice = true,
            _ => row.unsupported_shell_integration_path_left_undisclosed = true,
        }
        assert!(packet.validate().contains(
            &M5PortableModeStateContainmentAndDiagnosticsViolation::RowInvariantViolated
        ));
    }
}

#[test]
fn portable_root_inventory_not_proven_when_incomplete_example_removed() {
    let mut packet = seeded_m5_portable_mode_state_containment_and_diagnostics();
    for row in &mut packet.registry_rows {
        row.portable_state_layout_entries.retain(|ex| {
            ex.degrade_reason
                != Some(M5PortableStateLayoutEntryDegradeReason::DurableRootInventoryIncomplete)
        });
    }
    assert!(packet.validate().contains(
        &M5PortableModeStateContainmentAndDiagnosticsViolation::PortableRootInventoryNotProven
    ));
}

#[test]
fn portable_root_inventory_not_proven_when_surface_collapses() {
    let mut packet = seeded_m5_portable_mode_state_containment_and_diagnostics();
    // Drop every clean admin layout so the first-consumer surfaces no longer include it.
    for row in &mut packet.registry_rows {
        row.portable_state_layout_entries
            .retain(|ex| !(ex.is_clean() && ex.surface_context == "admin_surface"));
    }
    assert!(packet.validate().contains(
        &M5PortableModeStateContainmentAndDiagnosticsViolation::PortableRootInventoryNotProven
    ));
}

#[test]
fn distinguishability_not_proven_when_ambiguous_example_removed() {
    let mut packet = seeded_m5_portable_mode_state_containment_and_diagnostics();
    for row in &mut packet.registry_rows {
        row.portable_state_layout_entries.retain(|ex| {
            ex.degrade_reason != Some(M5PortableStateLayoutEntryDegradeReason::StateOriginAmbiguous)
        });
    }
    assert!(packet.validate().contains(
        &M5PortableModeStateContainmentAndDiagnosticsViolation::PortableStateDistinguishabilityNotProven
    ));
}

#[test]
fn distinguishability_not_proven_when_diagnostics_surface_dropped() {
    let mut packet = seeded_m5_portable_mode_state_containment_and_diagnostics();
    // Drop every clean docs-help diagnostics so the canonical surface coverage no longer includes it.
    for row in &mut packet.registry_rows {
        row.portable_diagnostics_entries
            .retain(|ex| !(ex.is_clean() && ex.diagnostics_surface == "docs_help_diagnostics"));
    }
    assert!(packet.validate().contains(
        &M5PortableModeStateContainmentAndDiagnosticsViolation::PortableStateDistinguishabilityNotProven
    ));
}

#[test]
fn spill_detection_not_proven_when_spill_example_removed() {
    let mut packet = seeded_m5_portable_mode_state_containment_and_diagnostics();
    for row in &mut packet.registry_rows {
        row.portable_state_layout_entries.retain(|ex| {
            ex.degrade_reason
                != Some(M5PortableStateLayoutEntryDegradeReason::HiddenMachineGlobalDurableSpill)
        });
    }
    assert!(packet.validate().contains(
        &M5PortableModeStateContainmentAndDiagnosticsViolation::PortableSpillDetectionNotProven
    ));
}

#[test]
fn spill_detection_not_proven_when_continuity_example_removed() {
    let mut packet = seeded_m5_portable_mode_state_containment_and_diagnostics();
    for row in &mut packet.registry_rows {
        row.portable_diagnostics_entries.retain(|ex| {
            ex.degrade_reason
                != Some(M5PortableDiagnosticsEntryDegradeReason::UpdateContinuityUndocumented)
        });
    }
    assert!(packet.validate().contains(
        &M5PortableModeStateContainmentAndDiagnosticsViolation::PortableSpillDetectionNotProven
    ));
}

#[test]
fn governance_review_incomplete_fails() {
    let mut packet = seeded_m5_portable_mode_state_containment_and_diagnostics();
    packet
        .governance_review
        .hidden_machine_global_mutation_absent_or_blocked = false;
    assert!(packet.validate().contains(
        &M5PortableModeStateContainmentAndDiagnosticsViolation::GovernanceReviewIncomplete
    ));
}

#[test]
fn consumer_projection_incomplete_fails() {
    let mut packet = seeded_m5_portable_mode_state_containment_and_diagnostics();
    packet
        .consumer_projection
        .support_export_reads_single_registry_source = false;
    assert!(packet.validate().contains(
        &M5PortableModeStateContainmentAndDiagnosticsViolation::ConsumerProjectionIncomplete
    ));
}

#[test]
fn proof_freshness_incomplete_fails() {
    let mut packet = seeded_m5_portable_mode_state_containment_and_diagnostics();
    packet.proof_freshness.proof_freshness_slo_hours = 0;
    assert!(packet.validate().contains(
        &M5PortableModeStateContainmentAndDiagnosticsViolation::ProofFreshnessIncomplete
    ));
}

#[test]
fn release_posture_incomplete_fails() {
    let mut packet = seeded_m5_portable_mode_state_containment_and_diagnostics();
    packet.release_posture.accessibility_parity_required = false;
    assert!(packet.validate().contains(
        &M5PortableModeStateContainmentAndDiagnosticsViolation::ReleasePostureIncomplete
    ));
}

#[test]
fn injected_raw_material_is_rejected() {
    let mut packet = seeded_m5_portable_mode_state_containment_and_diagnostics();
    packet.registry_rows[0].scope_summary =
        "raw endpoint https://relay.internal.example/session leaked".to_owned();
    assert!(packet
        .validate()
        .contains(&M5PortableModeStateContainmentAndDiagnosticsViolation::RawMaterialInExport));
}

#[test]
fn export_carries_no_forbidden_raw_material() {
    let json = seeded_m5_portable_mode_state_containment_and_diagnostics().export_safe_json();
    let lower = json.to_lowercase();
    assert!(!lower.contains("password"));
    assert!(!lower.contains("passphrase"));
    assert!(!lower.contains("bearer "));
    assert!(!lower.contains("://"));
    assert!(!lower.contains("-----begin"));
}

#[test]
fn csv_has_a_row_per_consumer_surface() {
    let packet = seeded_m5_portable_mode_state_containment_and_diagnostics();
    let csv = packet.render_matrix_csv();
    let lines: Vec<&str> = csv.lines().collect();
    assert_eq!(lines.len(), 1 + packet.registry_rows.len());
    assert!(lines[0].starts_with("consumer_surface,qualification,owner,"));
}

#[test]
fn markdown_summary_lists_every_consumer_surface() {
    let packet = seeded_m5_portable_mode_state_containment_and_diagnostics();
    let summary = packet.render_markdown_summary();
    for row in &packet.registry_rows {
        assert!(summary.contains(row.consumer_surface.as_str()));
    }
}

#[test]
fn root_inventory_table_lists_only_clean_layouts() {
    let packet = seeded_m5_portable_mode_state_containment_and_diagnostics();
    let table = packet.render_portable_root_inventory_table();
    // The clean colocated and named-sibling layouts are rendered from the registry.
    assert!(table.contains("colocated_under_executable"));
    assert!(table.contains("named_sibling_directory"));
    // A degraded, incomplete entry never leaks into the generated table.
    assert!(!table.contains("inventory-incomplete"));
    // An ambiguous origin never leaks into the generated table.
    assert!(!table.contains("origin_ambiguous"));
}

#[test]
fn checked_support_export_validates_and_matches_seed() {
    let from_disk = current_stable_m5_portable_mode_state_containment_and_diagnostics_export()
        .expect("checked M5 portable-mode state-containment / diagnostics export validates");
    assert_eq!(
        from_disk.packet_id,
        M5_PORTABLE_MODE_STATE_CONTAINMENT_AND_DIAGNOSTICS_PACKET_ID
    );
    assert_eq!(
        from_disk,
        seeded_m5_portable_mode_state_containment_and_diagnostics(),
        "checked support export drifted from the seed builder"
    );
}

#[test]
fn narrowed_variants_validate_and_keep_rows_visible() {
    let beta =
        seeded_m5_portable_mode_state_containment_and_diagnostics_side_by_side_channel_beta_narrowed();
    assert!(beta.validate().is_empty(), "{:?}", beta.validate());
    assert_eq!(beta.registry_rows.len(), 6);
    let row = beta
        .registry_rows
        .iter()
        .find(|r| r.consumer_surface == M5InstallTopologyConsumerSurface::Diagnostics)
        .unwrap();
    assert_eq!(row.qualification, M5InstallTopologyQualificationClass::Beta);

    let preview =
        seeded_m5_portable_mode_state_containment_and_diagnostics_offline_airgap_bundle_preview_narrowed();
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
    let beta: M5PortableModeStateContainmentAndDiagnosticsPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/install/m5-portable-mode-state-containment-and-diagnostics/side_by_side_channel_beta_narrowed.json"
    )))
    .expect("side-by-side fixture parses");
    assert!(beta.validate().is_empty());
    assert_eq!(
        beta,
        seeded_m5_portable_mode_state_containment_and_diagnostics_side_by_side_channel_beta_narrowed()
    );

    let preview: M5PortableModeStateContainmentAndDiagnosticsPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/install/m5-portable-mode-state-containment-and-diagnostics/offline_airgap_bundle_preview_narrowed.json"
    )))
    .expect("offline fixture parses");
    assert!(preview.validate().is_empty());
    assert_eq!(
        preview,
        seeded_m5_portable_mode_state_containment_and_diagnostics_offline_airgap_bundle_preview_narrowed()
    );
}

#[test]
fn implemented_families_is_portable_mode() {
    assert_eq!(
        IMPLEMENTED_FAMILIES,
        [M5InstallTopologyFamily::PortableMode]
    );
}
