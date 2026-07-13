use super::*;

fn clean_scale_input() -> M5DensityScaleEntryResolutionInput {
    let canonical = M5DensityMode::Standard.canonical_scale();
    M5DensityScaleEntryResolutionInput {
        entry_id: "scale:test".to_owned(),
        token_name: "shell.density.standard.scale".to_owned(),
        semantic_role: M5ShellGeometryRole::Density,
        density_mode_role: M5DensityModeRole::StandardMode,
        density_mode: M5DensityMode::Standard,
        surface_context: M5DensitySurfaceContext::Shell,
        row_height_px: canonical.row_height_px,
        control_height_px: canonical.control_height_px,
        tab_chip_spacing_px: canonical.tab_chip_spacing_px,
        panel_padding_px: canonical.panel_padding_px,
        gutter_spacing_px: canonical.gutter_spacing_px,
        surface_elements: M5DensitySurfaceElement::ALL.to_vec(),
        changes_information_architecture: false,
        preserves_command_focus_and_trust: true,
        proof_fresh: true,
    }
}

fn clean_persistence_input() -> M5DensityPersistenceEntryResolutionInput {
    M5DensityPersistenceEntryResolutionInput {
        entry_id: "persistence:test".to_owned(),
        token_name: "shell.density.persistence.profile".to_owned(),
        density_mode_role: M5DensityModeRole::PreservesInformationArchitecture,
        semantic_role: M5ShellGeometryRole::Density,
        persistence_scope: M5DensityPersistenceScope::ProfileScoped,
        override_reason: M5DensityOverrideReason::NotOverridden,
        surface_context: M5DensitySurfaceContext::Shell,
        switched_silently_by_provider_theme_or_workflow: false,
        proof_fresh: true,
    }
}

#[test]
fn seeded_registries_validates() {
    let packet = seeded_m5_density_mode_registries();
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
    assert_eq!(packet.packet_id, M5_DENSITY_MODE_REGISTRIES_PACKET_ID);
}

#[test]
fn scale_clean_names_meaning_and_matches_canonical() {
    let resolved = resolve_density_scale_entry(clean_scale_input()).unwrap();
    assert!(resolved.is_clean());
    assert!(resolved.density_change_is_presentation_only);
    assert!(resolved.matches_canonical_scale);
    assert!(resolved.covers_all_surface_elements);
    assert!(resolved.meets_hit_target_minimum);
    assert!(resolved.density_mode_is_classified);
    assert!(!resolved.density_mode_role_changes_information_architecture);
    assert!(resolved.semantic_role_preserves_task_identity_under_collapse);
    assert_eq!(resolved.semantic_role, "density");
    assert_eq!(resolved.density_mode, "standard");
    assert_eq!(resolved.row_height_px, 28);
    assert_eq!(resolved.control_height_px, 32);
    assert_eq!(resolved.surface_context, "shell");
    assert_eq!(
        resolved.next_action,
        M5DensityRegistryNextAction::ExpandDensityMeaning
    );
}

#[test]
fn canonical_scales_match_the_contract() {
    assert_eq!(M5DensityMode::Compact.canonical_scale().row_height_px, 24);
    assert_eq!(
        M5DensityMode::Compact.canonical_scale().control_height_px,
        28
    );
    assert_eq!(M5DensityMode::Standard.canonical_scale().row_height_px, 28);
    assert_eq!(
        M5DensityMode::Standard.canonical_scale().control_height_px,
        32
    );
    assert_eq!(
        M5DensityMode::Comfortable.canonical_scale().row_height_px,
        32
    );
    assert_eq!(
        M5DensityMode::Comfortable
            .canonical_scale()
            .control_height_px,
        36
    );
}

#[test]
fn scale_token_unstated_degrades() {
    let mut input = clean_scale_input();
    input.token_name = "   ".to_owned();
    assert_eq!(
        resolve_density_scale_entry(input).unwrap().degrade_reason,
        Some(M5DensityScaleEntryDegradeReason::TokenUnstated)
    );
}

#[test]
fn scale_mode_and_information_architecture_and_command_degrade() {
    let mut input = clean_scale_input();
    input.density_mode = M5DensityMode::ModeUnclassified;
    assert_eq!(
        resolve_density_scale_entry(input).unwrap().degrade_reason,
        Some(M5DensityScaleEntryDegradeReason::ModeUnclassified)
    );

    let mut input = clean_scale_input();
    input.changes_information_architecture = true;
    assert_eq!(
        resolve_density_scale_entry(input).unwrap().degrade_reason,
        Some(M5DensityScaleEntryDegradeReason::ChangesInformationArchitecture)
    );

    let mut input = clean_scale_input();
    input.density_mode_role = M5DensityModeRole::DensityChangesInformationArchitectureDisallowed;
    let resolved = resolve_density_scale_entry(input).unwrap();
    assert!(resolved.density_mode_role_changes_information_architecture);
    assert_eq!(
        resolved.degrade_reason,
        Some(M5DensityScaleEntryDegradeReason::ChangesInformationArchitecture)
    );

    let mut input = clean_scale_input();
    input.preserves_command_focus_and_trust = false;
    assert_eq!(
        resolve_density_scale_entry(input).unwrap().degrade_reason,
        Some(M5DensityScaleEntryDegradeReason::ChangesCommandFocusOrTrust)
    );
}

#[test]
fn scale_below_minimum_and_private_scale_and_element_degrade() {
    let mut input = clean_scale_input();
    input.control_height_px = 20;
    let resolved = resolve_density_scale_entry(input).unwrap();
    assert!(!resolved.meets_hit_target_minimum);
    assert_eq!(
        resolved.degrade_reason,
        Some(M5DensityScaleEntryDegradeReason::HitTargetShrinksBelowMinimum)
    );

    // A scale that stays above the hit-target minimum but does not match the canonical tokens degrades as
    // a private scale.
    let mut input = clean_scale_input();
    input.row_height_px = 30;
    input.control_height_px = 34;
    let resolved = resolve_density_scale_entry(input).unwrap();
    assert!(!resolved.matches_canonical_scale);
    assert!(resolved.meets_hit_target_minimum);
    assert_eq!(
        resolved.degrade_reason,
        Some(M5DensityScaleEntryDegradeReason::ScaleOutsideCanonicalTokens)
    );

    let mut input = clean_scale_input();
    input.surface_elements = vec![M5DensitySurfaceElement::List];
    assert_eq!(
        resolve_density_scale_entry(input).unwrap().degrade_reason,
        Some(M5DensityScaleEntryDegradeReason::SurfaceElementCoverageIncomplete)
    );
}

#[test]
fn scale_surface_and_proof_degrade() {
    let mut input = clean_scale_input();
    input.surface_context = M5DensitySurfaceContext::ContextUnknown;
    assert_eq!(
        resolve_density_scale_entry(input).unwrap().degrade_reason,
        Some(M5DensityScaleEntryDegradeReason::SurfaceContextUnresolved)
    );

    let mut input = clean_scale_input();
    input.proof_fresh = false;
    assert_eq!(
        resolve_density_scale_entry(input).unwrap().degrade_reason,
        Some(M5DensityScaleEntryDegradeReason::ProofStale)
    );
}

#[test]
fn scale_empty_id_and_forbidden_material_error() {
    let mut input = clean_scale_input();
    input.entry_id = "".to_owned();
    assert_eq!(
        resolve_density_scale_entry(input).unwrap_err(),
        M5DensityResolutionError::EmptyDensityScaleEntryId
    );

    let mut input = clean_scale_input();
    input.token_name = "https://relay.internal/leak".to_owned();
    assert_eq!(
        resolve_density_scale_entry(input).unwrap_err(),
        M5DensityResolutionError::ForbiddenMaterial
    );
}

#[test]
fn persistence_clean_holds_at_profile_scope() {
    let resolved = resolve_density_persistence_entry(clean_persistence_input()).unwrap();
    assert!(resolved.is_clean());
    assert!(resolved.persistence_holds_at_profile_scope);
    assert!(resolved.scope_is_classified);
    assert!(resolved.scope_is_profile_scoped);
    assert!(!resolved.switched_silently_by_provider_theme_or_workflow);
    assert_eq!(resolved.persistence_scope, "profile_scoped");
    assert_eq!(resolved.surface_context, "shell");
    assert_eq!(
        resolved.next_action,
        M5DensityRegistryNextAction::TraceCanonicalRegistry
    );
}

#[test]
fn persistence_explained_override_is_clean() {
    let mut input = clean_persistence_input();
    input.persistence_scope = M5DensityPersistenceScope::ExplainedLocalOverride;
    input.override_reason = M5DensityOverrideReason::AccessibilityViewer;
    let resolved = resolve_density_persistence_entry(input).unwrap();
    assert!(resolved.is_clean());
    assert!(resolved.override_is_explained);
    assert!(!resolved.scope_is_profile_scoped);
}

#[test]
fn persistence_silent_switch_and_unexplained_and_scope_degrade() {
    let mut input = clean_persistence_input();
    input.switched_silently_by_provider_theme_or_workflow = true;
    assert_eq!(
        resolve_density_persistence_entry(input)
            .unwrap()
            .degrade_reason,
        Some(M5DensityPersistenceEntryDegradeReason::SilentDensitySwitch)
    );

    let mut input = clean_persistence_input();
    input.persistence_scope = M5DensityPersistenceScope::ExplainedLocalOverride;
    input.override_reason = M5DensityOverrideReason::UnexplainedDisallowed;
    assert_eq!(
        resolve_density_persistence_entry(input)
            .unwrap()
            .degrade_reason,
        Some(M5DensityPersistenceEntryDegradeReason::UnexplainedLocalOverride)
    );

    let mut input = clean_persistence_input();
    input.persistence_scope = M5DensityPersistenceScope::ScopeUnclassified;
    assert_eq!(
        resolve_density_persistence_entry(input)
            .unwrap()
            .degrade_reason,
        Some(M5DensityPersistenceEntryDegradeReason::PersistenceScopeUnclassified)
    );
}

#[test]
fn persistence_surface_and_id_and_material() {
    let mut input = clean_persistence_input();
    input.surface_context = M5DensitySurfaceContext::ContextUnknown;
    assert_eq!(
        resolve_density_persistence_entry(input)
            .unwrap()
            .degrade_reason,
        Some(M5DensityPersistenceEntryDegradeReason::SurfaceContextUnresolved)
    );

    let mut input = clean_persistence_input();
    input.entry_id = "  ".to_owned();
    assert_eq!(
        resolve_density_persistence_entry(input).unwrap_err(),
        M5DensityResolutionError::EmptyDensityPersistenceEntryId
    );

    let mut input = clean_persistence_input();
    input.token_name = "see internal://notes".to_owned();
    assert_eq!(
        resolve_density_persistence_entry(input).unwrap_err(),
        M5DensityResolutionError::ForbiddenMaterial
    );
}

#[test]
fn vocabulary_set_is_canonical() {
    assert!(seeded_m5_density_mode_registries()
        .vocabulary_set
        .matches_canonical());
}

#[test]
fn vocabulary_set_drift_fails() {
    let mut packet = seeded_m5_density_mode_registries();
    packet.vocabulary_set.density_modes.pop();
    assert!(packet
        .validate()
        .contains(&M5DensityModeRegistriesViolation::VocabularySetDrift));
}

#[test]
fn missing_source_contracts_fails() {
    let mut packet = seeded_m5_density_mode_registries();
    packet.source_contract_refs.clear();
    assert!(packet
        .validate()
        .contains(&M5DensityModeRegistriesViolation::MissingSourceContracts));
}

#[test]
fn domain_schema_ref_missing_fails() {
    let mut packet = seeded_m5_density_mode_registries();
    packet.registry_rows[0]
        .source_contract_refs
        .retain(|r| r != M5_DENSITY_MODE_SCHEMA_REF);
    assert!(packet
        .validate()
        .contains(&M5DensityModeRegistriesViolation::DomainSchemaRefMissing));
}

#[test]
fn mandatory_anatomy_missing_fails() {
    let mut packet = seeded_m5_density_mode_registries();
    packet.registry_rows[0]
        .anatomy_parts
        .retain(|p| *p != M5DensityRegistryAnatomyPart::Identity);
    assert!(packet
        .validate()
        .contains(&M5DensityModeRegistriesViolation::MandatoryAnatomyMissing));
}

#[test]
fn mandatory_export_field_missing_fails() {
    let mut packet = seeded_m5_density_mode_registries();
    packet.registry_rows[0]
        .export_fields
        .retain(|f| *f != M5DensityRegistryExportField::DensityModes);
    assert!(packet
        .validate()
        .contains(&M5DensityModeRegistriesViolation::MandatoryExportFieldMissing));
}

#[test]
fn examples_missing_fails() {
    let mut packet = seeded_m5_density_mode_registries();
    packet.registry_rows[0].density_persistence_entries.clear();
    assert!(packet
        .validate()
        .contains(&M5DensityModeRegistriesViolation::ExamplesMissing));
}

#[test]
fn dishonest_clean_example_fails() {
    let mut packet = seeded_m5_density_mode_registries();
    // Force a clean density-scale entry to also read as information-architecture-changing — reject it.
    let row = &mut packet.registry_rows[0];
    row.density_scale_entries[0].degrade_reason = None;
    row.density_scale_entries[0].changes_information_architecture = true;
    assert!(packet
        .validate()
        .contains(&M5DensityModeRegistriesViolation::DishonestExample));
}

#[test]
fn row_invariant_violation_fails() {
    for mutate in 0u8..4 {
        let mut packet = seeded_m5_density_mode_registries();
        let row = &mut packet.registry_rows[0];
        match mutate {
            0 => row.density_change_alters_information_architecture = true,
            1 => row.density_change_alters_command_focus_or_trust = true,
            2 => row.shrinks_hit_target_below_supported_minimum = true,
            _ => row.silently_switches_density_outside_profile_scope = true,
        }
        assert!(packet
            .validate()
            .contains(&M5DensityModeRegistriesViolation::RowInvariantViolated));
    }
}

#[test]
fn tokenized_changes_not_proven_when_private_scale_example_removed() {
    let mut packet = seeded_m5_density_mode_registries();
    for row in &mut packet.registry_rows {
        row.density_scale_entries.retain(|ex| {
            ex.degrade_reason != Some(M5DensityScaleEntryDegradeReason::ScaleOutsideCanonicalTokens)
        });
    }
    assert!(packet.validate().contains(
        &M5DensityModeRegistriesViolation::TokenizedDensityChangesAcrossSurfacesNotProven
    ));
}

#[test]
fn tokenized_changes_not_proven_when_mode_dropped() {
    let mut packet = seeded_m5_density_mode_registries();
    // Drop every clean comfortable-mode scale so the coverage no longer includes it.
    for row in &mut packet.registry_rows {
        row.density_scale_entries
            .retain(|ex| !(ex.is_clean() && ex.density_mode == "comfortable"));
    }
    assert!(packet.validate().contains(
        &M5DensityModeRegistriesViolation::TokenizedDensityChangesAcrossSurfacesNotProven
    ));
}

#[test]
fn density_operable_not_proven_when_below_minimum_example_removed() {
    let mut packet = seeded_m5_density_mode_registries();
    for row in &mut packet.registry_rows {
        row.density_scale_entries.retain(|ex| {
            ex.degrade_reason
                != Some(M5DensityScaleEntryDegradeReason::HitTargetShrinksBelowMinimum)
        });
    }
    assert!(packet.validate().contains(
        &M5DensityModeRegistriesViolation::DensityOperableUnderZoomWithoutShrinkingHitTargetsNotProven
    ));
}

#[test]
fn extension_degradation_not_proven_when_silent_switch_removed() {
    let mut packet = seeded_m5_density_mode_registries();
    for row in &mut packet.registry_rows {
        row.density_persistence_entries.retain(|ex| {
            ex.degrade_reason != Some(M5DensityPersistenceEntryDegradeReason::SilentDensitySwitch)
        });
    }
    assert!(packet.validate().contains(
        &M5DensityModeRegistriesViolation::ExtensionPrivateScaleDegradesHonestlyNotProven
    ));
}

#[test]
fn governance_review_incomplete_fails() {
    let mut packet = seeded_m5_density_mode_registries();
    packet.governance_review.density_changes_presentation_only = false;
    assert!(packet
        .validate()
        .contains(&M5DensityModeRegistriesViolation::GovernanceReviewIncomplete));
}

#[test]
fn consumer_projection_incomplete_fails() {
    let mut packet = seeded_m5_density_mode_registries();
    packet
        .consumer_projection
        .support_export_reads_single_registry_source = false;
    assert!(packet
        .validate()
        .contains(&M5DensityModeRegistriesViolation::ConsumerProjectionIncomplete));
}

#[test]
fn proof_freshness_incomplete_fails() {
    let mut packet = seeded_m5_density_mode_registries();
    packet.proof_freshness.proof_freshness_slo_hours = 0;
    assert!(packet
        .validate()
        .contains(&M5DensityModeRegistriesViolation::ProofFreshnessIncomplete));
}

#[test]
fn release_posture_incomplete_fails() {
    let mut packet = seeded_m5_density_mode_registries();
    packet.release_posture.accessibility_parity_required = false;
    assert!(packet
        .validate()
        .contains(&M5DensityModeRegistriesViolation::ReleasePostureIncomplete));
}

#[test]
fn injected_raw_material_is_rejected() {
    let mut packet = seeded_m5_density_mode_registries();
    packet.registry_rows[0].scope_summary =
        "raw endpoint https://relay.internal.example/session leaked".to_owned();
    assert!(packet
        .validate()
        .contains(&M5DensityModeRegistriesViolation::RawMaterialInExport));
}

#[test]
fn export_carries_no_forbidden_raw_material() {
    let json = seeded_m5_density_mode_registries().export_safe_json();
    let lower = json.to_lowercase();
    assert!(!lower.contains("password"));
    assert!(!lower.contains("passphrase"));
    assert!(!lower.contains("bearer "));
    assert!(!lower.contains("://"));
    assert!(!lower.contains("-----begin"));
}

#[test]
fn csv_has_a_row_per_consumer_surface() {
    let packet = seeded_m5_density_mode_registries();
    let csv = packet.render_matrix_csv();
    let lines: Vec<&str> = csv.lines().collect();
    assert_eq!(lines.len(), 1 + packet.registry_rows.len());
    assert!(lines[0].starts_with("consumer_surface,qualification,owner,"));
}

#[test]
fn markdown_summary_lists_every_consumer_surface() {
    let packet = seeded_m5_density_mode_registries();
    let summary = packet.render_markdown_summary();
    for row in &packet.registry_rows {
        assert!(summary.contains(row.consumer_surface.as_str()));
    }
}

#[test]
fn checked_support_export_validates_and_matches_seed() {
    let from_disk = current_stable_m5_density_mode_registries_export()
        .expect("checked M5 density-mode registries export validates");
    assert_eq!(from_disk.packet_id, M5_DENSITY_MODE_REGISTRIES_PACKET_ID);
    assert_eq!(
        from_disk,
        seeded_m5_density_mode_registries(),
        "checked support export drifted from the seed builder"
    );
}

#[test]
fn narrowed_variants_validate_and_keep_rows_visible() {
    let beta = seeded_m5_density_mode_registries_editor_ui_beta_narrowed();
    assert!(beta.validate().is_empty(), "{:?}", beta.validate());
    assert_eq!(beta.registry_rows.len(), 6);
    let row = beta
        .registry_rows
        .iter()
        .find(|r| r.consumer_surface == M5ShellGeometryConsumerSurface::EditorUi)
        .unwrap();
    assert_eq!(row.qualification, M5ShellGeometryQualificationClass::Beta);

    let preview = seeded_m5_density_mode_registries_settings_ui_preview_narrowed();
    assert!(preview.validate().is_empty(), "{:?}", preview.validate());
    assert_eq!(preview.registry_rows.len(), 6);
    let row = preview
        .registry_rows
        .iter()
        .find(|r| r.consumer_surface == M5ShellGeometryConsumerSurface::SettingsUi)
        .unwrap();
    assert_eq!(
        row.qualification,
        M5ShellGeometryQualificationClass::Preview
    );
}

#[test]
fn checked_narrowed_fixtures_validate_and_match_seed_builders() {
    let beta: M5DensityModeRegistriesPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/ui/m5-density-mode-registries/editor_ui_beta_narrowed.json"
    )))
    .expect("editor-ui fixture parses");
    assert!(beta.validate().is_empty());
    assert_eq!(
        beta,
        seeded_m5_density_mode_registries_editor_ui_beta_narrowed()
    );

    let preview: M5DensityModeRegistriesPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/ui/m5-density-mode-registries/settings_ui_preview_narrowed.json"
    )))
    .expect("settings-ui fixture parses");
    assert!(preview.validate().is_empty());
    assert_eq!(
        preview,
        seeded_m5_density_mode_registries_settings_ui_preview_narrowed()
    );
}

#[test]
fn implemented_families_are_density_mode() {
    assert_eq!(IMPLEMENTED_FAMILIES, [M5ShellGeometryFamily::DensityMode]);
}
