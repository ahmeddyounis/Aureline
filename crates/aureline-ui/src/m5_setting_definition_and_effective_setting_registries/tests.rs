use super::*;

fn clean_definition_input() -> M5SettingDefinitionEntryResolutionInput {
    M5SettingDefinitionEntryResolutionInput {
        entry_id: "definition:test".to_owned(),
        setting_binding_id: "settings.acme.editor.format-on-save".to_owned(),
        token_name: "setting.definition.editor.format_on_save".to_owned(),
        semantic_role: M5SettingsGovernanceRole::SettingDefinition,
        setting_definition_type: M5SettingDefinitionKind::BooleanSetting,
        surface_context: M5SettingSurfaceContext::SettingsSurface,
        resolution_form_coverage: M5SettingResolutionForm::ALL.to_vec(),
        stable_setting_id: "editor.format_on_save".to_owned(),
        allowed_scopes: "scopes.machine-user-workspace".to_owned(),
        declared_default: "default.false".to_owned(),
        migration_aliases: "alias.editor.formatOnSave.v1".to_owned(),
        restart_posture: "restart.none".to_owned(),
        sensitivity_class: "sensitivity.public".to_owned(),
        capability_dependencies: "capability.editor.core".to_owned(),
        bound_to_registry: true,
        setting_id_preserved: true,
        is_sensitive_setting: false,
        sensitivity_disclosed: true,
        proof_fresh: true,
    }
}

fn clean_record_input() -> M5EffectiveSettingEntryResolutionInput {
    M5EffectiveSettingEntryResolutionInput {
        entry_id: "record:test".to_owned(),
        setting_ref: "editor.format_on_save".to_owned(),
        token_name: "effective.editor.format_on_save".to_owned(),
        semantic_role: M5SettingsGovernanceRole::EffectiveResolution,
        winning_scope: M5EffectiveSettingScope::MachineScope,
        surface_context: M5SettingSurfaceContext::SettingsSurface,
        resolution_form_coverage: M5SettingResolutionForm::ALL.to_vec(),
        resolved_value_summary: "value.true".to_owned(),
        shadow_chain: "shadow.user-default-lost".to_owned(),
        lock_or_constraint_state: "lock.none".to_owned(),
        validation_status: "validation.ok".to_owned(),
        restart_state: "restart.none".to_owned(),
        capability_availability: "capability.available".to_owned(),
        last_applied_revision: "revision.0007".to_owned(),
        keeps_shadow_chain_visible: true,
        resolution_is_truthful: true,
        lock_present: false,
        lock_source_disclosed: false,
        machine_only_value_present: false,
        machine_only_flagged_not_portable: false,
        proof_fresh: true,
    }
}

#[test]
fn seeded_registries_validates() {
    let packet = seeded_m5_setting_definition_and_effective_setting_registries();
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
    assert_eq!(
        packet.packet_id,
        M5_SETTING_DEFINITION_EFFECTIVE_SETTING_REGISTRIES_PACKET_ID
    );
}

#[test]
fn definition_clean_names_meaning_and_is_bound() {
    let resolved = resolve_setting_definition_entry(clean_definition_input()).unwrap();
    assert!(resolved.is_clean());
    assert!(resolved.definition_resolves_across_settings);
    assert!(resolved.covers_all_resolution_forms);
    assert!(resolved.setting_definition_object_complete);
    assert!(resolved.bound_to_registry);
    assert!(resolved.setting_definition_type_is_classified);
    assert!(resolved.setting_id_preserved);
    assert_eq!(resolved.semantic_role, "setting_definition");
    assert_eq!(resolved.setting_definition_type, "boolean_setting");
    assert_eq!(resolved.canonical_type_mode, "boolean_setting_type");
    assert_eq!(resolved.surface_context, "settings_surface");
    assert_eq!(
        resolved.next_action,
        M5SettingNextAction::ExpandSettingMeaning
    );
}

#[test]
fn definition_token_unstated_degrades() {
    let mut input = clean_definition_input();
    input.token_name = "   ".to_owned();
    assert_eq!(
        resolve_setting_definition_entry(input)
            .unwrap()
            .degrade_reason,
        Some(M5SettingDefinitionEntryDegradeReason::DefinitionTokenUnstated)
    );
}

#[test]
fn definition_unbound_and_unclassified_degrade() {
    let mut input = clean_definition_input();
    input.bound_to_registry = false;
    assert_eq!(
        resolve_setting_definition_entry(input)
            .unwrap()
            .degrade_reason,
        Some(M5SettingDefinitionEntryDegradeReason::DefinitionNotBoundToRegistry)
    );

    let mut input = clean_definition_input();
    input.setting_definition_type = M5SettingDefinitionKind::TypeUnclassified;
    assert_eq!(
        resolve_setting_definition_entry(input)
            .unwrap()
            .degrade_reason,
        Some(M5SettingDefinitionEntryDegradeReason::SettingDefinitionTypeUnclassified)
    );
}

#[test]
fn definition_object_incomplete_and_recycle_and_form_degrade() {
    // An unstated declared default leaves the resolved object incomplete.
    let mut input = clean_definition_input();
    input.declared_default = "  ".to_owned();
    let resolved = resolve_setting_definition_entry(input).unwrap();
    assert!(!resolved.setting_definition_object_complete);
    assert_eq!(
        resolved.degrade_reason,
        Some(M5SettingDefinitionEntryDegradeReason::SettingDefinitionObjectIncomplete)
    );

    // A recycled stable setting ID degrades.
    let mut input = clean_definition_input();
    input.setting_id_preserved = false;
    assert_eq!(
        resolve_setting_definition_entry(input)
            .unwrap()
            .degrade_reason,
        Some(M5SettingDefinitionEntryDegradeReason::SettingDefinitionRecyclesIdOrHidesSensitivity)
    );

    let mut input = clean_definition_input();
    input.resolution_form_coverage = vec![M5SettingResolutionForm::CanonicalObject];
    assert_eq!(
        resolve_setting_definition_entry(input)
            .unwrap()
            .degrade_reason,
        Some(M5SettingDefinitionEntryDegradeReason::ResolutionFormCoverageIncomplete)
    );
}

#[test]
fn definition_sensitivity_and_surface_and_proof_degrade() {
    let mut input = clean_definition_input();
    input.setting_definition_type = M5SettingDefinitionKind::SecretReferenceSetting;
    input.is_sensitive_setting = true;
    input.sensitivity_disclosed = false;
    // A sensitive setting hiding its sensitivity posture first fails non-recycled disclosure.
    assert_eq!(
        resolve_setting_definition_entry(input)
            .unwrap()
            .degrade_reason,
        Some(M5SettingDefinitionEntryDegradeReason::SettingDefinitionRecyclesIdOrHidesSensitivity)
    );

    let mut input = clean_definition_input();
    input.surface_context = M5SettingSurfaceContext::ContextUnknown;
    assert_eq!(
        resolve_setting_definition_entry(input)
            .unwrap()
            .degrade_reason,
        Some(M5SettingDefinitionEntryDegradeReason::SurfaceContextUnresolved)
    );

    let mut input = clean_definition_input();
    input.proof_fresh = false;
    assert_eq!(
        resolve_setting_definition_entry(input)
            .unwrap()
            .degrade_reason,
        Some(M5SettingDefinitionEntryDegradeReason::ProofStale)
    );
}

#[test]
fn definition_empty_id_and_forbidden_material_error() {
    let mut input = clean_definition_input();
    input.entry_id = "".to_owned();
    assert_eq!(
        resolve_setting_definition_entry(input).unwrap_err(),
        M5SettingResolutionError::EmptySettingDefinitionEntryId
    );

    let mut input = clean_definition_input();
    input.sensitivity_class = "see https://relay.internal/leak".to_owned();
    assert_eq!(
        resolve_setting_definition_entry(input).unwrap_err(),
        M5SettingResolutionError::ForbiddenMaterial
    );
}

#[test]
fn stable_setting_id_stays_non_recycled_rejects_recycle() {
    assert!(stable_setting_id_stays_non_recycled(
        M5SettingDefinitionKind::BooleanSetting,
        true,
        false,
        true
    ));
    assert!(!stable_setting_id_stays_non_recycled(
        M5SettingDefinitionKind::BooleanSetting,
        false,
        false,
        true
    ));
    assert!(stable_setting_id_stays_non_recycled(
        M5SettingDefinitionKind::SecretReferenceSetting,
        true,
        true,
        true
    ));
    assert!(!stable_setting_id_stays_non_recycled(
        M5SettingDefinitionKind::SecretReferenceSetting,
        true,
        true,
        false
    ));
    assert!(!stable_setting_id_stays_non_recycled(
        M5SettingDefinitionKind::TypeUnclassified,
        true,
        false,
        true
    ));
}

#[test]
fn setting_definition_object_is_complete_requires_all_fields() {
    assert!(setting_definition_object_is_complete(
        M5SettingDefinitionKind::BooleanSetting,
        "editor.format_on_save",
        "scopes.machine-user-workspace",
        "default.false",
        "alias.editor.formatOnSave.v1",
        "restart.none",
        "sensitivity.public",
        "capability.editor.core",
    ));
    assert!(!setting_definition_object_is_complete(
        M5SettingDefinitionKind::BooleanSetting,
        "editor.format_on_save",
        "  ",
        "default.false",
        "alias.editor.formatOnSave.v1",
        "restart.none",
        "sensitivity.public",
        "capability.editor.core",
    ));
    assert!(!setting_definition_object_is_complete(
        M5SettingDefinitionKind::TypeUnclassified,
        "editor.format_on_save",
        "scopes.machine-user-workspace",
        "default.false",
        "alias.editor.formatOnSave.v1",
        "restart.none",
        "sensitivity.public",
        "capability.editor.core",
    ));
}

#[test]
fn record_clean_stays_honest() {
    let resolved = resolve_effective_setting_entry(clean_record_input()).unwrap();
    assert!(resolved.is_clean());
    assert!(resolved.record_safe_on_every_setting);
    assert!(resolved.covers_all_resolution_forms);
    assert!(resolved.provides_complete_effective_setting);
    assert!(resolved.effective_setting_stays_honest);
    assert_eq!(resolved.winning_scope, "machine_scope");
    assert_eq!(resolved.surface_context, "settings_surface");
}

#[test]
fn record_hidden_shadow_and_unclassified_degrade() {
    // A masked locked value that hides its lock source is a hidden shadow chain.
    let mut input = clean_record_input();
    input.lock_present = true;
    input.lock_source_disclosed = false;
    let resolved = resolve_effective_setting_entry(input).unwrap();
    assert!(!resolved.provides_complete_effective_setting);
    assert_eq!(
        resolved.degrade_reason,
        Some(M5EffectiveSettingEntryDegradeReason::EffectiveSettingHidesShadowChainOrMasksLockOrMachineState)
    );

    // A record that hides the shadow chain of scopes that lost is also a hidden shadow chain.
    let mut input = clean_record_input();
    input.keeps_shadow_chain_visible = false;
    assert_eq!(
        resolve_effective_setting_entry(input).unwrap().degrade_reason,
        Some(M5EffectiveSettingEntryDegradeReason::EffectiveSettingHidesShadowChainOrMasksLockOrMachineState)
    );

    // A machine-only value masquerading as portable is also a hidden shadow chain.
    let mut input = clean_record_input();
    input.machine_only_value_present = true;
    input.machine_only_flagged_not_portable = false;
    assert_eq!(
        resolve_effective_setting_entry(input).unwrap().degrade_reason,
        Some(M5EffectiveSettingEntryDegradeReason::EffectiveSettingHidesShadowChainOrMasksLockOrMachineState)
    );

    let mut input = clean_record_input();
    input.winning_scope = M5EffectiveSettingScope::ScopeUnclassified;
    assert_eq!(
        resolve_effective_setting_entry(input)
            .unwrap()
            .degrade_reason,
        Some(M5EffectiveSettingEntryDegradeReason::WinningScopeUnclassified)
    );
}

#[test]
fn record_form_and_surface_and_id_and_material() {
    let mut input = clean_record_input();
    input.resolution_form_coverage = vec![M5SettingResolutionForm::CanonicalObject];
    assert_eq!(
        resolve_effective_setting_entry(input)
            .unwrap()
            .degrade_reason,
        Some(M5EffectiveSettingEntryDegradeReason::RecordFormCoverageIncomplete)
    );

    let mut input = clean_record_input();
    input.surface_context = M5SettingSurfaceContext::ContextUnknown;
    assert_eq!(
        resolve_effective_setting_entry(input)
            .unwrap()
            .degrade_reason,
        Some(M5EffectiveSettingEntryDegradeReason::SurfaceContextUnresolved)
    );

    let mut input = clean_record_input();
    input.entry_id = "  ".to_owned();
    assert_eq!(
        resolve_effective_setting_entry(input).unwrap_err(),
        M5SettingResolutionError::EmptyEffectiveSettingEntryId
    );

    let mut input = clean_record_input();
    input.resolved_value_summary = "see internal://notes".to_owned();
    assert_eq!(
        resolve_effective_setting_entry(input).unwrap_err(),
        M5SettingResolutionError::ForbiddenMaterial
    );
}

#[test]
fn record_disclosed_lock_and_flagged_machine_only_stay_clean() {
    // A locked value that discloses its lock source stays honest.
    let mut input = clean_record_input();
    input.lock_present = true;
    input.lock_source_disclosed = true;
    assert!(resolve_effective_setting_entry(input).unwrap().is_clean());

    // A machine-only value flagged non-portable stays honest.
    let mut input = clean_record_input();
    input.machine_only_value_present = true;
    input.machine_only_flagged_not_portable = true;
    assert!(resolve_effective_setting_entry(input).unwrap().is_clean());
}

#[test]
fn vocabulary_set_is_canonical() {
    assert!(
        seeded_m5_setting_definition_and_effective_setting_registries()
            .vocabulary_set
            .matches_canonical()
    );
}

#[test]
fn vocabulary_set_drift_fails() {
    let mut packet = seeded_m5_setting_definition_and_effective_setting_registries();
    packet.vocabulary_set.setting_definition_types.pop();
    assert!(packet
        .validate()
        .contains(&M5SettingDefinitionEffectiveSettingRegistriesViolation::VocabularySetDrift));
}

#[test]
fn missing_source_contracts_fails() {
    let mut packet = seeded_m5_setting_definition_and_effective_setting_registries();
    packet.source_contract_refs.clear();
    assert!(packet
        .validate()
        .contains(&M5SettingDefinitionEffectiveSettingRegistriesViolation::MissingSourceContracts));
}

#[test]
fn domain_schema_ref_missing_fails() {
    let mut packet = seeded_m5_setting_definition_and_effective_setting_registries();
    packet.registry_rows[0]
        .source_contract_refs
        .retain(|r| r != M5_SETTING_DEFINITION_DOMAIN_SCHEMA_REF);
    assert!(packet
        .validate()
        .contains(&M5SettingDefinitionEffectiveSettingRegistriesViolation::DomainSchemaRefMissing));

    let mut packet = seeded_m5_setting_definition_and_effective_setting_registries();
    packet.registry_rows[0]
        .source_contract_refs
        .retain(|r| r != M5_EFFECTIVE_SETTING_SCHEMA_REF);
    assert!(packet
        .validate()
        .contains(&M5SettingDefinitionEffectiveSettingRegistriesViolation::DomainSchemaRefMissing));
}

#[test]
fn mandatory_anatomy_missing_fails() {
    let mut packet = seeded_m5_setting_definition_and_effective_setting_registries();
    packet.registry_rows[0]
        .anatomy_parts
        .retain(|p| *p != M5SettingAnatomyPart::Identity);
    assert!(packet.validate().contains(
        &M5SettingDefinitionEffectiveSettingRegistriesViolation::MandatoryAnatomyMissing
    ));
}

#[test]
fn mandatory_export_field_missing_fails() {
    let mut packet = seeded_m5_setting_definition_and_effective_setting_registries();
    packet.registry_rows[0]
        .export_fields
        .retain(|f| *f != M5SettingExportField::SettingDefinitionTypes);
    assert!(packet.validate().contains(
        &M5SettingDefinitionEffectiveSettingRegistriesViolation::MandatoryExportFieldMissing
    ));
}

#[test]
fn examples_missing_fails() {
    let mut packet = seeded_m5_setting_definition_and_effective_setting_registries();
    packet.registry_rows[0].effective_setting_entries.clear();
    assert!(packet
        .validate()
        .contains(&M5SettingDefinitionEffectiveSettingRegistriesViolation::ExamplesMissing));
}

#[test]
fn dishonest_clean_example_fails() {
    let mut packet = seeded_m5_setting_definition_and_effective_setting_registries();
    // Force a clean definition entry to also read as object-incomplete — the packet must reject it.
    let row = &mut packet.registry_rows[0];
    row.setting_definition_entries[0].degrade_reason = None;
    row.setting_definition_entries[0].setting_definition_object_complete = false;
    assert!(packet
        .validate()
        .contains(&M5SettingDefinitionEffectiveSettingRegistriesViolation::DishonestExample));
}

#[test]
fn row_invariant_violation_fails() {
    for mutate in 0u8..4 {
        let mut packet = seeded_m5_setting_definition_and_effective_setting_registries();
        let row = &mut packet.registry_rows[0];
        match mutate {
            0 => row.recycles_a_retired_setting_id = true,
            1 => row.resolves_an_effective_value_without_an_inspectable_shadow_chain = true,
            2 => row.hides_restart_posture_lock_source_or_sensitivity_before_resolution = true,
            _ => row.collapses_distinct_settings_scopes_into_one_resolution_path = true,
        }
        assert!(packet.validate().contains(
            &M5SettingDefinitionEffectiveSettingRegistriesViolation::RowInvariantViolated
        ));
    }
}

#[test]
fn setting_definition_not_proven_when_incomplete_example_removed() {
    let mut packet = seeded_m5_setting_definition_and_effective_setting_registries();
    for row in &mut packet.registry_rows {
        row.setting_definition_entries.retain(|ex| {
            ex.degrade_reason
                != Some(M5SettingDefinitionEntryDegradeReason::SettingDefinitionObjectIncomplete)
        });
    }
    assert!(packet.validate().contains(
        &M5SettingDefinitionEffectiveSettingRegistriesViolation::SettingDefinitionResolutionNotProven
    ));
}

#[test]
fn setting_definition_not_proven_when_surface_collapses() {
    let mut packet = seeded_m5_setting_definition_and_effective_setting_registries();
    // Drop every clean admin-surface definition so the first-consumer surfaces no longer include it.
    for row in &mut packet.registry_rows {
        row.setting_definition_entries
            .retain(|ex| !(ex.is_clean() && ex.surface_context == "admin_surface"));
    }
    assert!(packet.validate().contains(
        &M5SettingDefinitionEffectiveSettingRegistriesViolation::SettingDefinitionResolutionNotProven
    ));
}

#[test]
fn stable_setting_id_preservation_not_proven_when_recycle_example_removed() {
    let mut packet = seeded_m5_setting_definition_and_effective_setting_registries();
    for row in &mut packet.registry_rows {
        row.setting_definition_entries.retain(|ex| {
            ex.degrade_reason
                != Some(
                    M5SettingDefinitionEntryDegradeReason::SettingDefinitionRecyclesIdOrHidesSensitivity,
                )
        });
    }
    assert!(packet.validate().contains(
        &M5SettingDefinitionEffectiveSettingRegistriesViolation::StableSettingIdPreservationNotProven
    ));
}

#[test]
fn stable_setting_id_preservation_not_proven_when_unbound_example_removed() {
    let mut packet = seeded_m5_setting_definition_and_effective_setting_registries();
    for row in &mut packet.registry_rows {
        row.setting_definition_entries.retain(|ex| {
            ex.degrade_reason
                != Some(M5SettingDefinitionEntryDegradeReason::DefinitionNotBoundToRegistry)
        });
    }
    assert!(packet.validate().contains(
        &M5SettingDefinitionEffectiveSettingRegistriesViolation::StableSettingIdPreservationNotProven
    ));
}

#[test]
fn effective_setting_integrity_not_proven_when_hidden_shadow_example_removed() {
    let mut packet = seeded_m5_setting_definition_and_effective_setting_registries();
    for row in &mut packet.registry_rows {
        row.effective_setting_entries.retain(|ex| {
            ex.degrade_reason
                != Some(
                    M5EffectiveSettingEntryDegradeReason::EffectiveSettingHidesShadowChainOrMasksLockOrMachineState,
                )
        });
    }
    assert!(packet.validate().contains(
        &M5SettingDefinitionEffectiveSettingRegistriesViolation::EffectiveSettingIntegrityNotProven
    ));
}

#[test]
fn effective_setting_integrity_not_proven_when_scope_dropped() {
    let mut packet = seeded_m5_setting_definition_and_effective_setting_registries();
    // Drop every clean workspace-scope effective setting so the coverage no longer includes it.
    for row in &mut packet.registry_rows {
        row.effective_setting_entries
            .retain(|ex| !(ex.is_clean() && ex.winning_scope == "workspace_scope"));
    }
    assert!(packet.validate().contains(
        &M5SettingDefinitionEffectiveSettingRegistriesViolation::EffectiveSettingIntegrityNotProven
    ));
}

#[test]
fn governance_review_incomplete_fails() {
    let mut packet = seeded_m5_setting_definition_and_effective_setting_registries();
    packet
        .governance_review
        .stable_setting_ids_stay_non_recycled = false;
    assert!(packet.validate().contains(
        &M5SettingDefinitionEffectiveSettingRegistriesViolation::GovernanceReviewIncomplete
    ));
}

#[test]
fn consumer_projection_incomplete_fails() {
    let mut packet = seeded_m5_setting_definition_and_effective_setting_registries();
    packet
        .consumer_projection
        .support_export_reads_single_registry_source = false;
    assert!(packet.validate().contains(
        &M5SettingDefinitionEffectiveSettingRegistriesViolation::ConsumerProjectionIncomplete
    ));
}

#[test]
fn proof_freshness_incomplete_fails() {
    let mut packet = seeded_m5_setting_definition_and_effective_setting_registries();
    packet.proof_freshness.proof_freshness_slo_hours = 0;
    assert!(packet.validate().contains(
        &M5SettingDefinitionEffectiveSettingRegistriesViolation::ProofFreshnessIncomplete
    ));
}

#[test]
fn release_posture_incomplete_fails() {
    let mut packet = seeded_m5_setting_definition_and_effective_setting_registries();
    packet.release_posture.accessibility_parity_required = false;
    assert!(packet.validate().contains(
        &M5SettingDefinitionEffectiveSettingRegistriesViolation::ReleasePostureIncomplete
    ));
}

#[test]
fn injected_raw_material_is_rejected() {
    let mut packet = seeded_m5_setting_definition_and_effective_setting_registries();
    packet.registry_rows[0].scope_summary =
        "raw endpoint https://sync.example/scope leaked".to_owned();
    assert!(packet
        .validate()
        .contains(&M5SettingDefinitionEffectiveSettingRegistriesViolation::RawMaterialInExport));
}

#[test]
fn export_carries_no_forbidden_raw_material() {
    let json = seeded_m5_setting_definition_and_effective_setting_registries().export_safe_json();
    let lower = json.to_lowercase();
    assert!(!lower.contains("password"));
    assert!(!lower.contains("passphrase"));
    assert!(!lower.contains("bearer "));
    assert!(!lower.contains("://"));
    assert!(!lower.contains("-----begin"));
}

#[test]
fn csv_has_a_row_per_consumer_surface() {
    let packet = seeded_m5_setting_definition_and_effective_setting_registries();
    let csv = packet.render_matrix_csv();
    let lines: Vec<&str> = csv.lines().collect();
    assert_eq!(lines.len(), 1 + packet.registry_rows.len());
    assert!(lines[0].starts_with("consumer_surface,qualification,owner,"));
}

#[test]
fn markdown_summary_lists_every_consumer_surface() {
    let packet = seeded_m5_setting_definition_and_effective_setting_registries();
    let summary = packet.render_markdown_summary();
    for row in &packet.registry_rows {
        assert!(summary.contains(row.consumer_surface.as_str()));
    }
}

#[test]
fn setting_definition_table_lists_only_clean_definitions() {
    let packet = seeded_m5_setting_definition_and_effective_setting_registries();
    let table = packet.render_setting_definition_table();
    // The clean boolean and enum definitions are rendered from the registry.
    assert!(table.contains("boolean_setting_type"));
    assert!(table.contains("enum_setting_type"));
    // A degraded, incomplete entry never leaks into the generated table.
    assert!(!table.contains("incomplete"));
}

#[test]
fn checked_support_export_validates_and_matches_seed() {
    let from_disk = current_stable_m5_setting_definition_and_effective_setting_registries_export()
        .expect("checked M5 setting-definition / effective-setting registries export validates");
    assert_eq!(
        from_disk.packet_id,
        M5_SETTING_DEFINITION_EFFECTIVE_SETTING_REGISTRIES_PACKET_ID
    );
    assert_eq!(
        from_disk,
        seeded_m5_setting_definition_and_effective_setting_registries(),
        "checked support export drifted from the seed builder"
    );
}

#[test]
fn narrowed_variants_validate_and_keep_rows_visible() {
    let beta =
        seeded_m5_setting_definition_and_effective_setting_registries_setting_definition_beta_narrowed();
    assert!(beta.validate().is_empty(), "{:?}", beta.validate());
    assert_eq!(beta.registry_rows.len(), 6);
    let row = beta
        .registry_rows
        .iter()
        .find(|r| r.consumer_surface == M5SettingsGovernanceConsumerSurface::SettingsResolver)
        .unwrap();
    assert_eq!(
        row.qualification,
        M5SettingsGovernanceQualificationClass::Beta
    );

    let preview =
        seeded_m5_setting_definition_and_effective_setting_registries_effective_setting_preview_narrowed();
    assert!(preview.validate().is_empty(), "{:?}", preview.validate());
    assert_eq!(preview.registry_rows.len(), 6);
    let row = preview
        .registry_rows
        .iter()
        .find(|r| r.consumer_surface == M5SettingsGovernanceConsumerSurface::SyncService)
        .unwrap();
    assert_eq!(
        row.qualification,
        M5SettingsGovernanceQualificationClass::Preview
    );
}

#[test]
fn checked_narrowed_fixtures_validate_and_match_seed_builders() {
    let beta: M5SettingDefinitionEffectiveSettingRegistriesPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/config/m5-setting-definition-and-effective-setting-registries/setting_definition_beta_narrowed.json"
    )))
    .expect("setting-definition fixture parses");
    assert!(beta.validate().is_empty());
    assert_eq!(
        beta,
        seeded_m5_setting_definition_and_effective_setting_registries_setting_definition_beta_narrowed()
    );

    let preview: M5SettingDefinitionEffectiveSettingRegistriesPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/config/m5-setting-definition-and-effective-setting-registries/effective_setting_preview_narrowed.json"
    )))
    .expect("effective-setting fixture parses");
    assert!(preview.validate().is_empty());
    assert_eq!(
        preview,
        seeded_m5_setting_definition_and_effective_setting_registries_effective_setting_preview_narrowed()
    );
}

#[test]
fn implemented_families_is_resolve_setting_and_migrate_schema() {
    assert_eq!(
        IMPLEMENTED_FAMILIES,
        [
            M5SettingsGovernanceFamily::ResolveSetting,
            M5SettingsGovernanceFamily::MigrateSchema,
        ]
    );
}
