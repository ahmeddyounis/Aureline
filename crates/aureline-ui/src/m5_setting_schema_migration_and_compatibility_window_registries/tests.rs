use super::*;

fn clean_migration_input() -> M5SchemaMigrationRecordEntryResolutionInput {
    M5SchemaMigrationRecordEntryResolutionInput {
        entry_id: "migration:test".to_owned(),
        migration_ref: "settings.acme.editor.font-size@v1-to-v2".to_owned(),
        token_name: "migration.editor.font_size".to_owned(),
        semantic_role: M5SettingsGovernanceRole::SchemaMigration,
        fidelity_class: M5SchemaMigrationFidelityClass::ExactMigration,
        surface_context: M5ConfigMigrationSurfaceContext::UpgradeFlow,
        resolution_form_coverage: M5ConfigMigrationResolutionForm::ALL.to_vec(),
        old_key_or_alias: "old.editor.fontSize".to_owned(),
        new_key: "new.editor.font-size".to_owned(),
        transform: "transform.rename-key-verbatim".to_owned(),
        compatibility_window: "window.v1-through-v3".to_owned(),
        rollback_note: "rollback.restore-v1-key".to_owned(),
        compare_before_apply_reference: "compare.before-apply-0007".to_owned(),
        migration_provenance_reference: "provenance.migration-record-0007".to_owned(),
        bound_to_registry: true,
        fidelity_label_honest: true,
        is_lossy_or_manual_review: false,
        compare_surface_materialized: true,
        proof_fresh: true,
    }
}

fn clean_window_input() -> M5CompatibilityWindowEntryResolutionInput {
    M5CompatibilityWindowEntryResolutionInput {
        entry_id: "window:test".to_owned(),
        window_ref: "editor.font_size".to_owned(),
        token_name: "window.editor.font_size".to_owned(),
        semantic_role: M5SettingsGovernanceRole::SchemaMigration,
        window_class: M5CompatibilityWindowClass::WithinCompatibilityWindow,
        surface_context: M5ConfigMigrationSurfaceContext::UpgradeFlow,
        resolution_form_coverage: M5ConfigMigrationResolutionForm::ALL.to_vec(),
        window_source: "window.schema-version-registry".to_owned(),
        supported_version_range: "range.v1-to-v3".to_owned(),
        deprecation_review: "review.deprecates-2026-12-31".to_owned(),
        validation_status: "validation.ok".to_owned(),
        review_state: "review.current".to_owned(),
        docs_pointer: "docs.migration-compatibility".to_owned(),
        last_review_revision: "revision.0007".to_owned(),
        keeps_window_source_visible: true,
        window_is_truthful: true,
        deprecation_present: false,
        deprecation_source_disclosed: false,
        unsupported_present: false,
        downgrade_guidance_disclosed: false,
        proof_fresh: true,
    }
}

#[test]
fn seeded_registries_validates() {
    let packet = seeded_m5_setting_schema_migration_and_compatibility_window_registries();
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
    assert_eq!(
        packet.packet_id,
        M5_SETTING_SCHEMA_MIGRATION_COMPATIBILITY_WINDOW_REGISTRIES_PACKET_ID
    );
}

#[test]
fn migration_clean_names_meaning_and_is_bound() {
    let resolved = resolve_schema_migration_record_entry(clean_migration_input()).unwrap();
    assert!(resolved.is_clean());
    assert!(resolved.migration_resolves_across_routes);
    assert!(resolved.covers_all_resolution_forms);
    assert!(resolved.schema_migration_record_complete);
    assert!(resolved.bound_to_registry);
    assert!(resolved.fidelity_class_is_classified);
    assert!(resolved.fidelity_label_honest);
    assert_eq!(resolved.semantic_role, "schema_migration");
    assert_eq!(resolved.fidelity_class, "exact_migration");
    assert_eq!(resolved.canonical_class_mode, "exact_migration_label");
    assert_eq!(resolved.surface_context, "upgrade_flow");
    assert_eq!(
        resolved.next_action,
        M5ConfigMigrationNextAction::ExpandMigrationMeaning
    );
}

#[test]
fn migration_token_unstated_degrades() {
    let mut input = clean_migration_input();
    input.token_name = "   ".to_owned();
    assert_eq!(
        resolve_schema_migration_record_entry(input)
            .unwrap()
            .degrade_reason,
        Some(M5SchemaMigrationRecordEntryDegradeReason::SchemaMigrationTokenUnstated)
    );
}

#[test]
fn migration_unbound_and_unclassified_degrade() {
    let mut input = clean_migration_input();
    input.bound_to_registry = false;
    assert_eq!(
        resolve_schema_migration_record_entry(input)
            .unwrap()
            .degrade_reason,
        Some(M5SchemaMigrationRecordEntryDegradeReason::SchemaMigrationNotBoundToRegistry)
    );

    let mut input = clean_migration_input();
    input.fidelity_class = M5SchemaMigrationFidelityClass::FidelityClassUnclassified;
    assert_eq!(
        resolve_schema_migration_record_entry(input)
            .unwrap()
            .degrade_reason,
        Some(M5SchemaMigrationRecordEntryDegradeReason::FidelityClassUnclassified)
    );
}

#[test]
fn migration_record_incomplete_and_overstate_and_form_degrade() {
    // An unstated compare-before-apply reference leaves the resolved record incomplete.
    let mut input = clean_migration_input();
    input.compare_before_apply_reference = "  ".to_owned();
    let resolved = resolve_schema_migration_record_entry(input).unwrap();
    assert!(!resolved.schema_migration_record_complete);
    assert_eq!(
        resolved.degrade_reason,
        Some(M5SchemaMigrationRecordEntryDegradeReason::SchemaMigrationRecordIncomplete)
    );

    // An overstated fidelity label degrades.
    let mut input = clean_migration_input();
    input.fidelity_label_honest = false;
    assert_eq!(
        resolve_schema_migration_record_entry(input)
            .unwrap()
            .degrade_reason,
        Some(M5SchemaMigrationRecordEntryDegradeReason::MigrationOverstatesFidelityOrHidesCompareSurface)
    );

    let mut input = clean_migration_input();
    input.resolution_form_coverage = vec![M5ConfigMigrationResolutionForm::CanonicalObject];
    assert_eq!(
        resolve_schema_migration_record_entry(input)
            .unwrap()
            .degrade_reason,
        Some(M5SchemaMigrationRecordEntryDegradeReason::ResolutionFormCoverageIncomplete)
    );
}

#[test]
fn migration_compare_and_surface_and_proof_degrade() {
    let mut input = clean_migration_input();
    input.fidelity_class = M5SchemaMigrationFidelityClass::LossyMigration;
    input.is_lossy_or_manual_review = true;
    input.compare_surface_materialized = false;
    // A lossy migration hiding its compare surface first fails the fidelity / compare fold.
    assert_eq!(
        resolve_schema_migration_record_entry(input)
            .unwrap()
            .degrade_reason,
        Some(M5SchemaMigrationRecordEntryDegradeReason::MigrationOverstatesFidelityOrHidesCompareSurface)
    );

    let mut input = clean_migration_input();
    input.surface_context = M5ConfigMigrationSurfaceContext::ContextUnknown;
    assert_eq!(
        resolve_schema_migration_record_entry(input)
            .unwrap()
            .degrade_reason,
        Some(M5SchemaMigrationRecordEntryDegradeReason::SurfaceContextUnresolved)
    );

    let mut input = clean_migration_input();
    input.proof_fresh = false;
    assert_eq!(
        resolve_schema_migration_record_entry(input)
            .unwrap()
            .degrade_reason,
        Some(M5SchemaMigrationRecordEntryDegradeReason::ProofStale)
    );
}

#[test]
fn migration_empty_id_and_forbidden_material_error() {
    let mut input = clean_migration_input();
    input.entry_id = "".to_owned();
    assert_eq!(
        resolve_schema_migration_record_entry(input).unwrap_err(),
        M5ConfigMigrationResolutionError::EmptySchemaMigrationEntryId
    );

    let mut input = clean_migration_input();
    input.rollback_note = "see https://relay.internal/leak".to_owned();
    assert_eq!(
        resolve_schema_migration_record_entry(input).unwrap_err(),
        M5ConfigMigrationResolutionError::ForbiddenMaterial
    );
}

#[test]
fn migration_does_not_overstate_fidelity_rejects_lossy_without_compare() {
    assert!(migration_does_not_overstate_fidelity(
        M5SchemaMigrationFidelityClass::ExactMigration,
        true,
        false,
        true
    ));
    assert!(!migration_does_not_overstate_fidelity(
        M5SchemaMigrationFidelityClass::ExactMigration,
        false,
        false,
        true
    ));
    assert!(migration_does_not_overstate_fidelity(
        M5SchemaMigrationFidelityClass::LossyMigration,
        true,
        true,
        true
    ));
    assert!(!migration_does_not_overstate_fidelity(
        M5SchemaMigrationFidelityClass::LossyMigration,
        true,
        true,
        false
    ));
    assert!(!migration_does_not_overstate_fidelity(
        M5SchemaMigrationFidelityClass::FidelityClassUnclassified,
        true,
        false,
        true
    ));
}

#[test]
fn schema_migration_record_is_complete_requires_all_fields() {
    assert!(schema_migration_record_is_complete(
        M5SchemaMigrationFidelityClass::ExactMigration,
        "old.editor.fontSize",
        "new.editor.font-size",
        "transform.rename-key-verbatim",
        "window.v1-through-v3",
        "rollback.restore-v1-key",
        "compare.before-apply-0007",
        "provenance.migration-record-0007",
    ));
    assert!(!schema_migration_record_is_complete(
        M5SchemaMigrationFidelityClass::ExactMigration,
        "old.editor.fontSize",
        "  ",
        "transform.rename-key-verbatim",
        "window.v1-through-v3",
        "rollback.restore-v1-key",
        "compare.before-apply-0007",
        "provenance.migration-record-0007",
    ));
    assert!(!schema_migration_record_is_complete(
        M5SchemaMigrationFidelityClass::FidelityClassUnclassified,
        "old.editor.fontSize",
        "new.editor.font-size",
        "transform.rename-key-verbatim",
        "window.v1-through-v3",
        "rollback.restore-v1-key",
        "compare.before-apply-0007",
        "provenance.migration-record-0007",
    ));
}

#[test]
fn window_clean_stays_honest() {
    let resolved = resolve_compatibility_window_entry(clean_window_input()).unwrap();
    assert!(resolved.is_clean());
    assert!(resolved.window_safe_on_every_route);
    assert!(resolved.covers_all_resolution_forms);
    assert!(resolved.provides_complete_compatibility_window);
    assert!(resolved.compatibility_window_stays_honest);
    assert_eq!(resolved.window_class, "within_compatibility_window");
    assert_eq!(resolved.surface_context, "upgrade_flow");
}

#[test]
fn window_masked_and_unclassified_degrade() {
    // A deprecated window that hides its window source is a masked window.
    let mut input = clean_window_input();
    input.window_class = M5CompatibilityWindowClass::DeprecatedButSupported;
    input.deprecation_present = true;
    input.deprecation_source_disclosed = false;
    let resolved = resolve_compatibility_window_entry(input).unwrap();
    assert!(!resolved.provides_complete_compatibility_window);
    assert_eq!(
        resolved.degrade_reason,
        Some(M5CompatibilityWindowEntryDegradeReason::CompatibilityWindowMasksWindowSourceOrHidesDowngradeGuidance)
    );

    // A record that drops the window source visibility is also a masked window.
    let mut input = clean_window_input();
    input.keeps_window_source_visible = false;
    assert_eq!(
        resolve_compatibility_window_entry(input)
            .unwrap()
            .degrade_reason,
        Some(M5CompatibilityWindowEntryDegradeReason::CompatibilityWindowMasksWindowSourceOrHidesDowngradeGuidance)
    );

    // An outside-window migration that hides its downgrade guidance is also a masked window / hidden guidance.
    let mut input = clean_window_input();
    input.window_class = M5CompatibilityWindowClass::OutsideCompatibilityWindow;
    input.unsupported_present = true;
    input.downgrade_guidance_disclosed = false;
    assert_eq!(
        resolve_compatibility_window_entry(input)
            .unwrap()
            .degrade_reason,
        Some(M5CompatibilityWindowEntryDegradeReason::CompatibilityWindowMasksWindowSourceOrHidesDowngradeGuidance)
    );

    let mut input = clean_window_input();
    input.window_class = M5CompatibilityWindowClass::WindowClassUnclassified;
    assert_eq!(
        resolve_compatibility_window_entry(input)
            .unwrap()
            .degrade_reason,
        Some(M5CompatibilityWindowEntryDegradeReason::WindowClassUnclassified)
    );
}

#[test]
fn window_form_and_surface_and_id_and_material() {
    let mut input = clean_window_input();
    input.resolution_form_coverage = vec![M5ConfigMigrationResolutionForm::CanonicalObject];
    assert_eq!(
        resolve_compatibility_window_entry(input)
            .unwrap()
            .degrade_reason,
        Some(M5CompatibilityWindowEntryDegradeReason::WindowFormCoverageIncomplete)
    );

    let mut input = clean_window_input();
    input.surface_context = M5ConfigMigrationSurfaceContext::ContextUnknown;
    assert_eq!(
        resolve_compatibility_window_entry(input)
            .unwrap()
            .degrade_reason,
        Some(M5CompatibilityWindowEntryDegradeReason::SurfaceContextUnresolved)
    );

    let mut input = clean_window_input();
    input.entry_id = "  ".to_owned();
    assert_eq!(
        resolve_compatibility_window_entry(input).unwrap_err(),
        M5ConfigMigrationResolutionError::EmptyCompatibilityWindowEntryId
    );

    let mut input = clean_window_input();
    input.window_source = "see internal://notes".to_owned();
    assert_eq!(
        resolve_compatibility_window_entry(input).unwrap_err(),
        M5ConfigMigrationResolutionError::ForbiddenMaterial
    );
}

#[test]
fn window_disclosed_deprecation_and_disclosed_guidance_stay_clean() {
    // A deprecated window that discloses its window source stays honest.
    let mut input = clean_window_input();
    input.window_class = M5CompatibilityWindowClass::DeprecatedButSupported;
    input.deprecation_present = true;
    input.deprecation_source_disclosed = true;
    assert!(resolve_compatibility_window_entry(input)
        .unwrap()
        .is_clean());

    // An outside-window migration that discloses its downgrade guidance stays honest.
    let mut input = clean_window_input();
    input.window_class = M5CompatibilityWindowClass::OutsideCompatibilityWindow;
    input.unsupported_present = true;
    input.downgrade_guidance_disclosed = true;
    assert!(resolve_compatibility_window_entry(input)
        .unwrap()
        .is_clean());
}

#[test]
fn vocabulary_set_is_canonical() {
    assert!(
        seeded_m5_setting_schema_migration_and_compatibility_window_registries()
            .vocabulary_set
            .matches_canonical()
    );
}

#[test]
fn vocabulary_set_drift_fails() {
    let mut packet = seeded_m5_setting_schema_migration_and_compatibility_window_registries();
    packet.vocabulary_set.migration_fidelity_classes.pop();
    assert!(packet.validate().contains(
        &M5SettingSchemaMigrationCompatibilityWindowRegistriesViolation::VocabularySetDrift
    ));
}

#[test]
fn missing_source_contracts_fails() {
    let mut packet = seeded_m5_setting_schema_migration_and_compatibility_window_registries();
    packet.source_contract_refs.clear();
    assert!(packet.validate().contains(
        &M5SettingSchemaMigrationCompatibilityWindowRegistriesViolation::MissingSourceContracts
    ));
}

#[test]
fn domain_schema_ref_missing_fails() {
    let mut packet = seeded_m5_setting_schema_migration_and_compatibility_window_registries();
    packet.registry_rows[0]
        .source_contract_refs
        .retain(|r| r != M5_SETTING_DEFINITION_DOMAIN_SCHEMA_REF);
    assert!(packet.validate().contains(
        &M5SettingSchemaMigrationCompatibilityWindowRegistriesViolation::DomainSchemaRefMissing
    ));

    let mut packet = seeded_m5_setting_schema_migration_and_compatibility_window_registries();
    packet.registry_rows[0]
        .source_contract_refs
        .retain(|r| r != M5_SCHEMA_MIGRATION_LANDED_SCHEMA_REF);
    assert!(packet.validate().contains(
        &M5SettingSchemaMigrationCompatibilityWindowRegistriesViolation::DomainSchemaRefMissing
    ));
}

#[test]
fn mandatory_anatomy_missing_fails() {
    let mut packet = seeded_m5_setting_schema_migration_and_compatibility_window_registries();
    packet.registry_rows[0]
        .anatomy_parts
        .retain(|p| *p != M5ConfigMigrationAnatomyPart::Identity);
    assert!(packet.validate().contains(
        &M5SettingSchemaMigrationCompatibilityWindowRegistriesViolation::MandatoryAnatomyMissing
    ));
}

#[test]
fn mandatory_export_field_missing_fails() {
    let mut packet = seeded_m5_setting_schema_migration_and_compatibility_window_registries();
    packet.registry_rows[0]
        .export_fields
        .retain(|f| *f != M5ConfigMigrationExportField::MigrationFidelityLabels);
    assert!(packet.validate().contains(
        &M5SettingSchemaMigrationCompatibilityWindowRegistriesViolation::MandatoryExportFieldMissing
    ));
}

#[test]
fn examples_missing_fails() {
    let mut packet = seeded_m5_setting_schema_migration_and_compatibility_window_registries();
    packet.registry_rows[0].compatibility_window_entries.clear();
    assert!(packet.validate().contains(
        &M5SettingSchemaMigrationCompatibilityWindowRegistriesViolation::ExamplesMissing
    ));
}

#[test]
fn dishonest_clean_example_fails() {
    let mut packet = seeded_m5_setting_schema_migration_and_compatibility_window_registries();
    // Force a clean migration entry to also read as record-incomplete — the packet must reject it.
    let row = &mut packet.registry_rows[0];
    row.schema_migration_entries[0].degrade_reason = None;
    row.schema_migration_entries[0].schema_migration_record_complete = false;
    assert!(packet.validate().contains(
        &M5SettingSchemaMigrationCompatibilityWindowRegistriesViolation::DishonestExample
    ));
}

#[test]
fn row_invariant_violation_fails() {
    for mutate in 0u8..4 {
        let mut packet = seeded_m5_setting_schema_migration_and_compatibility_window_registries();
        let row = &mut packet.registry_rows[0];
        match mutate {
            0 => row.implies_full_fidelity_when_migration_is_lossy = true,
            1 => row.alters_stored_meaning_without_a_checked_in_migration_record = true,
            2 => row.applies_a_lossy_migration_without_a_compare_before_apply_surface = true,
            _ => row.hides_the_compatibility_window_or_downgrade_cause_behind_generic_copy = true,
        }
        assert!(packet.validate().contains(
            &M5SettingSchemaMigrationCompatibilityWindowRegistriesViolation::RowInvariantViolated
        ));
    }
}

#[test]
fn schema_migration_not_proven_when_incomplete_example_removed() {
    let mut packet = seeded_m5_setting_schema_migration_and_compatibility_window_registries();
    for row in &mut packet.registry_rows {
        row.schema_migration_entries.retain(|ex| {
            ex.degrade_reason
                != Some(M5SchemaMigrationRecordEntryDegradeReason::SchemaMigrationRecordIncomplete)
        });
    }
    assert!(packet.validate().contains(
        &M5SettingSchemaMigrationCompatibilityWindowRegistriesViolation::SchemaMigrationResolutionNotProven
    ));
}

#[test]
fn schema_migration_not_proven_when_surface_collapses() {
    let mut packet = seeded_m5_setting_schema_migration_and_compatibility_window_registries();
    // Drop every clean downgrade-flow migration so the first-consumer flows no longer include it.
    for row in &mut packet.registry_rows {
        row.schema_migration_entries
            .retain(|ex| !(ex.is_clean() && ex.surface_context == "downgrade_flow"));
    }
    assert!(packet.validate().contains(
        &M5SettingSchemaMigrationCompatibilityWindowRegistriesViolation::SchemaMigrationResolutionNotProven
    ));
}

#[test]
fn migration_fidelity_not_proven_when_overstate_example_removed() {
    let mut packet = seeded_m5_setting_schema_migration_and_compatibility_window_registries();
    for row in &mut packet.registry_rows {
        row.schema_migration_entries.retain(|ex| {
            ex.degrade_reason
                != Some(
                    M5SchemaMigrationRecordEntryDegradeReason::MigrationOverstatesFidelityOrHidesCompareSurface,
                )
        });
    }
    assert!(packet.validate().contains(
        &M5SettingSchemaMigrationCompatibilityWindowRegistriesViolation::MigrationFidelityHonestyNotProven
    ));
}

#[test]
fn migration_fidelity_not_proven_when_unbound_example_removed() {
    let mut packet = seeded_m5_setting_schema_migration_and_compatibility_window_registries();
    for row in &mut packet.registry_rows {
        row.schema_migration_entries.retain(|ex| {
            ex.degrade_reason
                != Some(
                    M5SchemaMigrationRecordEntryDegradeReason::SchemaMigrationNotBoundToRegistry,
                )
        });
    }
    assert!(packet.validate().contains(
        &M5SettingSchemaMigrationCompatibilityWindowRegistriesViolation::MigrationFidelityHonestyNotProven
    ));
}

#[test]
fn compatibility_window_integrity_not_proven_when_masked_window_example_removed() {
    let mut packet = seeded_m5_setting_schema_migration_and_compatibility_window_registries();
    for row in &mut packet.registry_rows {
        row.compatibility_window_entries.retain(|ex| {
            ex.degrade_reason
                != Some(
                    M5CompatibilityWindowEntryDegradeReason::CompatibilityWindowMasksWindowSourceOrHidesDowngradeGuidance,
                )
        });
    }
    assert!(packet.validate().contains(
        &M5SettingSchemaMigrationCompatibilityWindowRegistriesViolation::CompatibilityWindowIntegrityNotProven
    ));
}

#[test]
fn compatibility_window_integrity_not_proven_when_class_dropped() {
    let mut packet = seeded_m5_setting_schema_migration_and_compatibility_window_registries();
    // Drop every clean outside-window record so the coverage no longer includes it.
    for row in &mut packet.registry_rows {
        row.compatibility_window_entries
            .retain(|ex| !(ex.is_clean() && ex.window_class == "outside_compatibility_window"));
    }
    assert!(packet.validate().contains(
        &M5SettingSchemaMigrationCompatibilityWindowRegistriesViolation::CompatibilityWindowIntegrityNotProven
    ));
}

#[test]
fn governance_review_incomplete_fails() {
    let mut packet = seeded_m5_setting_schema_migration_and_compatibility_window_registries();
    packet
        .governance_review
        .migration_labels_never_overstate_fidelity = false;
    assert!(packet.validate().contains(
        &M5SettingSchemaMigrationCompatibilityWindowRegistriesViolation::GovernanceReviewIncomplete
    ));
}

#[test]
fn consumer_projection_incomplete_fails() {
    let mut packet = seeded_m5_setting_schema_migration_and_compatibility_window_registries();
    packet
        .consumer_projection
        .support_export_reads_single_registry_source = false;
    assert!(packet.validate().contains(
        &M5SettingSchemaMigrationCompatibilityWindowRegistriesViolation::ConsumerProjectionIncomplete
    ));
}

#[test]
fn proof_freshness_incomplete_fails() {
    let mut packet = seeded_m5_setting_schema_migration_and_compatibility_window_registries();
    packet.proof_freshness.proof_freshness_slo_hours = 0;
    assert!(packet.validate().contains(
        &M5SettingSchemaMigrationCompatibilityWindowRegistriesViolation::ProofFreshnessIncomplete
    ));
}

#[test]
fn release_posture_incomplete_fails() {
    let mut packet = seeded_m5_setting_schema_migration_and_compatibility_window_registries();
    packet.release_posture.accessibility_parity_required = false;
    assert!(packet.validate().contains(
        &M5SettingSchemaMigrationCompatibilityWindowRegistriesViolation::ReleasePostureIncomplete
    ));
}

#[test]
fn injected_raw_material_is_rejected() {
    let mut packet = seeded_m5_setting_schema_migration_and_compatibility_window_registries();
    packet.registry_rows[0].scope_summary =
        "raw endpoint https://sync.example/scope leaked".to_owned();
    assert!(packet.validate().contains(
        &M5SettingSchemaMigrationCompatibilityWindowRegistriesViolation::RawMaterialInExport
    ));
}

#[test]
fn export_carries_no_forbidden_raw_material() {
    let json =
        seeded_m5_setting_schema_migration_and_compatibility_window_registries().export_safe_json();
    let lower = json.to_lowercase();
    assert!(!lower.contains("password"));
    assert!(!lower.contains("passphrase"));
    assert!(!lower.contains("bearer "));
    assert!(!lower.contains("://"));
    assert!(!lower.contains("-----begin"));
}

#[test]
fn csv_has_a_row_per_consumer_surface() {
    let packet = seeded_m5_setting_schema_migration_and_compatibility_window_registries();
    let csv = packet.render_matrix_csv();
    let lines: Vec<&str> = csv.lines().collect();
    assert_eq!(lines.len(), 1 + packet.registry_rows.len());
    assert!(lines[0].starts_with("consumer_surface,qualification,owner,"));
}

#[test]
fn markdown_summary_lists_every_consumer_surface() {
    let packet = seeded_m5_setting_schema_migration_and_compatibility_window_registries();
    let summary = packet.render_markdown_summary();
    for row in &packet.registry_rows {
        assert!(summary.contains(row.consumer_surface.as_str()));
    }
}

#[test]
fn migration_table_lists_only_clean_migrations() {
    let packet = seeded_m5_setting_schema_migration_and_compatibility_window_registries();
    let table = packet.render_migration_table();
    // The clean exact and compatible migrations are rendered from the registry.
    assert!(table.contains("exact_migration_label"));
    assert!(table.contains("compatible_migration_label"));
    // A degraded, incomplete entry never leaks into the generated table.
    assert!(!table.contains("incomplete"));
}

#[test]
fn checked_support_export_validates_and_matches_seed() {
    let from_disk =
        current_stable_m5_setting_schema_migration_and_compatibility_window_registries_export()
            .expect(
                "checked M5 schema-migration / compatibility-window registries export validates",
            );
    assert_eq!(
        from_disk.packet_id,
        M5_SETTING_SCHEMA_MIGRATION_COMPATIBILITY_WINDOW_REGISTRIES_PACKET_ID
    );
    assert_eq!(
        from_disk,
        seeded_m5_setting_schema_migration_and_compatibility_window_registries(),
        "checked support export drifted from the seed builder"
    );
}

#[test]
fn narrowed_variants_validate_and_keep_rows_visible() {
    let beta =
        seeded_m5_setting_schema_migration_and_compatibility_window_registries_schema_migration_beta_narrowed();
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
        seeded_m5_setting_schema_migration_and_compatibility_window_registries_compatibility_window_preview_narrowed();
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
    let beta: M5SettingSchemaMigrationCompatibilityWindowRegistriesPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/config/m5-setting-schema-migration-and-compatibility-window-registries/schema_migration_beta_narrowed.json"
    )))
    .expect("schema-migration fixture parses");
    assert!(beta.validate().is_empty());
    assert_eq!(
        beta,
        seeded_m5_setting_schema_migration_and_compatibility_window_registries_schema_migration_beta_narrowed()
    );

    let preview: M5SettingSchemaMigrationCompatibilityWindowRegistriesPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/config/m5-setting-schema-migration-and-compatibility-window-registries/compatibility_window_preview_narrowed.json"
    )))
    .expect("compatibility-window fixture parses");
    assert!(preview.validate().is_empty());
    assert_eq!(
        preview,
        seeded_m5_setting_schema_migration_and_compatibility_window_registries_compatibility_window_preview_narrowed()
    );
}

#[test]
fn implemented_families_is_migrate_schema() {
    assert_eq!(
        IMPLEMENTED_FAMILIES,
        [M5SettingsGovernanceFamily::MigrateSchema]
    );
}
