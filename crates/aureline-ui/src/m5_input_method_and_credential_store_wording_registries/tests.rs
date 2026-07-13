use super::*;

fn clean_input_input() -> M5InputCompositionEntryResolutionInput {
    M5InputCompositionEntryResolutionInput {
        entry_id: "input:test".to_owned(),
        command_id: "command.editor.insert".to_owned(),
        token_name: "input.ime.editor.macos".to_owned(),
        semantic_role: M5PlatformFitRole::InputFidelity,
        input_role: M5InputMethodRole::ImeCompositionFidelity,
        input_stack: M5InputMethodStack::MacosInputMethods,
        surface_context: M5InputSurfaceContext::EditorBuffer,
        presentation_form_coverage: M5InputCredentialPresentationForm::ALL.to_vec(),
        committed_text: "日本語".to_owned(),
        expected_text: "日本語".to_owned(),
        bound_to_registry: true,
        preserves_command_and_trust_fidelity: true,
        composition_unsupported_on_surface: false,
        fallback_input_path_explained: true,
        proof_fresh: true,
    }
}

fn clean_credential_input() -> M5CredentialStoreWordingEntryResolutionInput {
    M5CredentialStoreWordingEntryResolutionInput {
        entry_id: "cred:test".to_owned(),
        command_id: "command.auth.store.inspect".to_owned(),
        token_name: "cred.store.settings".to_owned(),
        wording_role: M5CredentialStoreWordingRole::TruthfulStorageClaim,
        semantic_role: M5PlatformFitRole::CredentialWording,
        wording_surface: M5CredentialWordingSurface::SettingsCredentialPanel,
        surface_context: M5InputSurfaceContext::SettingsField,
        presentation_form_coverage: M5InputCredentialPresentationForm::ALL.to_vec(),
        generic_wording: "Your sign-in credentials are kept in the system secure store.".to_owned(),
        disclosure_route: "settings.credentials.help".to_owned(),
        storage_is_truthful: true,
        non_leaky: true,
        plaintext_fallback_used: false,
        plaintext_fallback_disclosed: false,
        platform_detail_disclosed: false,
        platform_detail_justified: false,
        proof_fresh: true,
    }
}

#[test]
fn seeded_registries_validates() {
    let packet = seeded_m5_input_method_and_credential_store_wording_registries();
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
    assert_eq!(packet.packet_id, M5_INPUT_CREDENTIAL_REGISTRIES_PACKET_ID);
}

#[test]
fn input_clean_names_meaning_and_is_bound() {
    let resolved = resolve_input_composition_entry(clean_input_input()).unwrap();
    assert!(resolved.is_clean());
    assert!(resolved.text_fidelity_holds_across_surfaces_and_profiles);
    assert!(resolved.covers_all_presentation_forms);
    assert!(resolved.composed_text_intact);
    assert!(resolved.bound_to_registry);
    assert!(resolved.input_stack_is_classified);
    assert!(!resolved.input_role_names_corruption);
    assert!(resolved.preserves_command_and_trust_fidelity);
    assert_eq!(resolved.semantic_role, "input_fidelity");
    assert_eq!(resolved.input_stack, "macos_input_methods");
    assert_eq!(
        resolved.canonical_composition_model,
        "marked-text composition"
    );
    assert_eq!(resolved.surface_context, "editor_buffer");
    assert_eq!(
        resolved.next_action,
        M5InputCredentialNextAction::ExpandInputMeaning
    );
}

#[test]
fn input_token_unstated_degrades() {
    let mut input = clean_input_input();
    input.token_name = "   ".to_owned();
    assert_eq!(
        resolve_input_composition_entry(input)
            .unwrap()
            .degrade_reason,
        Some(M5InputCompositionEntryDegradeReason::InputTokenUnstated)
    );
}

#[test]
fn input_unbound_and_unclassified_degrade() {
    let mut input = clean_input_input();
    input.input_role = M5InputMethodRole::TextOrTrustCorruptionDisallowed;
    let resolved = resolve_input_composition_entry(input).unwrap();
    assert!(resolved.input_role_names_corruption);
    assert_eq!(
        resolved.degrade_reason,
        Some(M5InputCompositionEntryDegradeReason::BehaviorNotBoundToRegistry)
    );

    let mut input = clean_input_input();
    input.bound_to_registry = false;
    assert_eq!(
        resolve_input_composition_entry(input)
            .unwrap()
            .degrade_reason,
        Some(M5InputCompositionEntryDegradeReason::BehaviorNotBoundToRegistry)
    );

    let mut input = clean_input_input();
    input.input_stack = M5InputMethodStack::StackUnclassified;
    assert_eq!(
        resolve_input_composition_entry(input)
            .unwrap()
            .degrade_reason,
        Some(M5InputCompositionEntryDegradeReason::InputStackUnclassified)
    );
}

#[test]
fn input_corrupted_and_fidelity_and_form_degrade() {
    // A committed text that drifts from the expected text is corrupted for its stack.
    let mut input = clean_input_input();
    input.committed_text = "日木語".to_owned();
    let resolved = resolve_input_composition_entry(input).unwrap();
    assert!(!resolved.composed_text_intact);
    assert_eq!(
        resolved.degrade_reason,
        Some(M5InputCompositionEntryDegradeReason::ComposedTextCorruptedForStack)
    );

    let mut input = clean_input_input();
    input.preserves_command_and_trust_fidelity = false;
    assert_eq!(
        resolve_input_composition_entry(input)
            .unwrap()
            .degrade_reason,
        Some(M5InputCompositionEntryDegradeReason::CommandOrTrustFidelityNotPreserved)
    );

    let mut input = clean_input_input();
    input.presentation_form_coverage = vec![M5InputCredentialPresentationForm::LiteralRendering];
    assert_eq!(
        resolve_input_composition_entry(input)
            .unwrap()
            .degrade_reason,
        Some(M5InputCompositionEntryDegradeReason::FidelityFormCoverageIncomplete)
    );
}

#[test]
fn input_composition_unhandled_and_surface_and_proof_degrade() {
    let mut input = clean_input_input();
    input.composition_unsupported_on_surface = true;
    input.fallback_input_path_explained = false;
    assert_eq!(
        resolve_input_composition_entry(input)
            .unwrap()
            .degrade_reason,
        Some(M5InputCompositionEntryDegradeReason::CompositionUnsupportedWithoutFallback)
    );

    let mut input = clean_input_input();
    input.surface_context = M5InputSurfaceContext::ContextUnknown;
    assert_eq!(
        resolve_input_composition_entry(input)
            .unwrap()
            .degrade_reason,
        Some(M5InputCompositionEntryDegradeReason::SurfaceContextUnresolved)
    );

    let mut input = clean_input_input();
    input.proof_fresh = false;
    assert_eq!(
        resolve_input_composition_entry(input)
            .unwrap()
            .degrade_reason,
        Some(M5InputCompositionEntryDegradeReason::ProofStale)
    );
}

#[test]
fn input_empty_id_and_forbidden_material_error() {
    let mut input = clean_input_input();
    input.entry_id = "".to_owned();
    assert_eq!(
        resolve_input_composition_entry(input).unwrap_err(),
        M5InputCredentialResolutionError::EmptyInputCompositionEntryId
    );

    let mut input = clean_input_input();
    input.committed_text = "see https://relay.internal/leak".to_owned();
    assert_eq!(
        resolve_input_composition_entry(input).unwrap_err(),
        M5InputCredentialResolutionError::ForbiddenMaterial
    );
}

#[test]
fn input_composition_matches_stack_rejects_drift() {
    assert!(input_composition_matches_stack(
        M5InputMethodStack::MacosInputMethods,
        "café",
        "café"
    ));
    assert!(!input_composition_matches_stack(
        M5InputMethodStack::MacosInputMethods,
        "cafe",
        "café"
    ));
    assert!(input_composition_matches_stack(
        M5InputMethodStack::WindowsImeTsf,
        "€uro",
        "€uro"
    ));
    assert!(!input_composition_matches_stack(
        M5InputMethodStack::StackUnclassified,
        "café",
        "café"
    ));
    assert!(!input_composition_matches_stack(
        M5InputMethodStack::LinuxImeIbusFcitx,
        "   ",
        "café"
    ));
}

#[test]
fn credential_clean_stays_truthful() {
    let resolved = resolve_credential_store_wording_entry(clean_credential_input()).unwrap();
    assert!(resolved.is_clean());
    assert!(resolved.wording_truthful_on_every_profile);
    assert!(resolved.covers_all_presentation_forms);
    assert!(resolved.provides_complete_disclosure_triple);
    assert!(resolved.wording_stays_truthful);
    assert!(!resolved.wording_role_hides_plaintext);
    assert_eq!(resolved.wording_surface, "settings_credential_panel");
    assert_eq!(resolved.surface_context, "settings_field");
}

#[test]
fn credential_untruthful_and_unclassified_degrade() {
    // A used but undisclosed plaintext fallback hides a downgrade — untruthful.
    let mut input = clean_credential_input();
    input.plaintext_fallback_used = true;
    input.plaintext_fallback_disclosed = false;
    let resolved = resolve_credential_store_wording_entry(input).unwrap();
    assert!(!resolved.provides_complete_disclosure_triple);
    assert_eq!(
        resolved.degrade_reason,
        Some(M5CredentialStoreWordingEntryDegradeReason::StorageClaimUntruthfulOrLeaky)
    );

    // Unjustified platform detail is also untruthful.
    let mut input = clean_credential_input();
    input.platform_detail_disclosed = true;
    input.platform_detail_justified = false;
    assert_eq!(
        resolve_credential_store_wording_entry(input)
            .unwrap()
            .degrade_reason,
        Some(M5CredentialStoreWordingEntryDegradeReason::StorageClaimUntruthfulOrLeaky)
    );

    // The disallowed hidden-plaintext-fallback role degrades.
    let mut input = clean_credential_input();
    input.wording_role = M5CredentialStoreWordingRole::PlaintextFallbackHiddenDisallowed;
    assert_eq!(
        resolve_credential_store_wording_entry(input)
            .unwrap()
            .degrade_reason,
        Some(M5CredentialStoreWordingEntryDegradeReason::StorageClaimUntruthfulOrLeaky)
    );

    let mut input = clean_credential_input();
    input.wording_surface = M5CredentialWordingSurface::SurfaceUnclassified;
    assert_eq!(
        resolve_credential_store_wording_entry(input)
            .unwrap()
            .degrade_reason,
        Some(M5CredentialStoreWordingEntryDegradeReason::CredentialSurfaceUnclassified)
    );
}

#[test]
fn credential_form_and_surface_and_id_and_material() {
    let mut input = clean_credential_input();
    input.presentation_form_coverage = vec![M5InputCredentialPresentationForm::LiteralRendering];
    assert_eq!(
        resolve_credential_store_wording_entry(input)
            .unwrap()
            .degrade_reason,
        Some(M5CredentialStoreWordingEntryDegradeReason::WordingPhrasingCoverageIncomplete)
    );

    let mut input = clean_credential_input();
    input.surface_context = M5InputSurfaceContext::ContextUnknown;
    assert_eq!(
        resolve_credential_store_wording_entry(input)
            .unwrap()
            .degrade_reason,
        Some(M5CredentialStoreWordingEntryDegradeReason::SurfaceContextUnresolved)
    );

    let mut input = clean_credential_input();
    input.entry_id = "  ".to_owned();
    assert_eq!(
        resolve_credential_store_wording_entry(input).unwrap_err(),
        M5InputCredentialResolutionError::EmptyCredentialStoreWordingEntryId
    );

    let mut input = clean_credential_input();
    input.generic_wording = "see internal://notes".to_owned();
    assert_eq!(
        resolve_credential_store_wording_entry(input).unwrap_err(),
        M5InputCredentialResolutionError::ForbiddenMaterial
    );
}

#[test]
fn credential_disclosed_fallback_and_justified_detail_stay_clean() {
    // A disclosed plaintext fallback stays truthful.
    let mut input = clean_credential_input();
    input.plaintext_fallback_used = true;
    input.plaintext_fallback_disclosed = true;
    assert!(resolve_credential_store_wording_entry(input)
        .unwrap()
        .is_clean());

    // Justified platform detail stays truthful.
    let mut input = clean_credential_input();
    input.platform_detail_disclosed = true;
    input.platform_detail_justified = true;
    assert!(resolve_credential_store_wording_entry(input)
        .unwrap()
        .is_clean());
}

#[test]
fn vocabulary_set_is_canonical() {
    assert!(
        seeded_m5_input_method_and_credential_store_wording_registries()
            .vocabulary_set
            .matches_canonical()
    );
}

#[test]
fn vocabulary_set_drift_fails() {
    let mut packet = seeded_m5_input_method_and_credential_store_wording_registries();
    packet.vocabulary_set.input_stacks.pop();
    assert!(packet
        .validate()
        .contains(&M5InputCredentialRegistriesViolation::VocabularySetDrift));
}

#[test]
fn missing_source_contracts_fails() {
    let mut packet = seeded_m5_input_method_and_credential_store_wording_registries();
    packet.source_contract_refs.clear();
    assert!(packet
        .validate()
        .contains(&M5InputCredentialRegistriesViolation::MissingSourceContracts));
}

#[test]
fn domain_schema_ref_missing_fails() {
    let mut packet = seeded_m5_input_method_and_credential_store_wording_registries();
    packet.registry_rows[0]
        .source_contract_refs
        .retain(|r| r != M5_INPUT_METHOD_BEHAVIOR_SCHEMA_REF);
    assert!(packet
        .validate()
        .contains(&M5InputCredentialRegistriesViolation::DomainSchemaRefMissing));

    let mut packet = seeded_m5_input_method_and_credential_store_wording_registries();
    packet.registry_rows[0]
        .source_contract_refs
        .retain(|r| r != M5_FILE_PATH_AND_REVEAL_SCHEMA_REF);
    assert!(packet
        .validate()
        .contains(&M5InputCredentialRegistriesViolation::DomainSchemaRefMissing));
}

#[test]
fn mandatory_anatomy_missing_fails() {
    let mut packet = seeded_m5_input_method_and_credential_store_wording_registries();
    packet.registry_rows[0]
        .anatomy_parts
        .retain(|p| *p != M5InputCredentialAnatomyPart::Identity);
    assert!(packet
        .validate()
        .contains(&M5InputCredentialRegistriesViolation::MandatoryAnatomyMissing));
}

#[test]
fn mandatory_export_field_missing_fails() {
    let mut packet = seeded_m5_input_method_and_credential_store_wording_registries();
    packet.registry_rows[0]
        .export_fields
        .retain(|f| *f != M5InputCredentialExportField::InputStacks);
    assert!(packet
        .validate()
        .contains(&M5InputCredentialRegistriesViolation::MandatoryExportFieldMissing));
}

#[test]
fn examples_missing_fails() {
    let mut packet = seeded_m5_input_method_and_credential_store_wording_registries();
    packet.registry_rows[0]
        .credential_store_wording_entries
        .clear();
    assert!(packet
        .validate()
        .contains(&M5InputCredentialRegistriesViolation::ExamplesMissing));
}

#[test]
fn dishonest_clean_example_fails() {
    let mut packet = seeded_m5_input_method_and_credential_store_wording_registries();
    // Force a clean input entry to also read as text-corrupted — the packet must reject it.
    let row = &mut packet.registry_rows[0];
    row.input_composition_entries[0].degrade_reason = None;
    row.input_composition_entries[0].composed_text_intact = false;
    assert!(packet
        .validate()
        .contains(&M5InputCredentialRegistriesViolation::DishonestExample));
}

#[test]
fn row_invariant_violation_fails() {
    for mutate in 0u8..4 {
        let mut packet = seeded_m5_input_method_and_credential_store_wording_registries();
        let row = &mut packet.registry_rows[0];
        match mutate {
            0 => row.input_method_corrupts_text_command_or_trust = true,
            1 => row.shortcut_routing_and_composition_fight = true,
            2 => row.credential_wording_hides_plaintext_downgrade_or_leaks = true,
            _ => row.input_or_credential_wording_hardcoded_instead_of_registry = true,
        }
        assert!(packet
            .validate()
            .contains(&M5InputCredentialRegistriesViolation::RowInvariantViolated));
    }
}

#[test]
fn text_intact_not_proven_when_corrupted_example_removed() {
    let mut packet = seeded_m5_input_method_and_credential_store_wording_registries();
    for row in &mut packet.registry_rows {
        row.input_composition_entries.retain(|ex| {
            ex.degrade_reason
                != Some(M5InputCompositionEntryDegradeReason::ComposedTextCorruptedForStack)
        });
    }
    assert!(packet
        .validate()
        .contains(&M5InputCredentialRegistriesViolation::TextIntactAcrossProfilesNotProven));
}

#[test]
fn text_intact_not_proven_when_surface_collapses() {
    let mut packet = seeded_m5_input_method_and_credential_store_wording_registries();
    // Drop every clean terminal input so the first-consumer surfaces no longer include it.
    for row in &mut packet.registry_rows {
        row.input_composition_entries
            .retain(|ex| !(ex.is_clean() && ex.surface_context == "terminal_input"));
    }
    assert!(packet
        .validate()
        .contains(&M5InputCredentialRegistriesViolation::TextIntactAcrossProfilesNotProven));
}

#[test]
fn composition_and_shortcuts_not_proven_when_fidelity_example_removed() {
    let mut packet = seeded_m5_input_method_and_credential_store_wording_registries();
    for row in &mut packet.registry_rows {
        row.input_composition_entries.retain(|ex| {
            ex.degrade_reason
                != Some(M5InputCompositionEntryDegradeReason::CommandOrTrustFidelityNotPreserved)
        });
    }
    assert!(packet.validate().contains(
        &M5InputCredentialRegistriesViolation::CompositionAndShortcutsDoNotFightNotProven
    ));
}

#[test]
fn composition_and_shortcuts_not_proven_when_unbound_example_removed() {
    let mut packet = seeded_m5_input_method_and_credential_store_wording_registries();
    for row in &mut packet.registry_rows {
        row.input_composition_entries.retain(|ex| {
            ex.degrade_reason
                != Some(M5InputCompositionEntryDegradeReason::BehaviorNotBoundToRegistry)
        });
    }
    assert!(packet.validate().contains(
        &M5InputCredentialRegistriesViolation::CompositionAndShortcutsDoNotFightNotProven
    ));
}

#[test]
fn credential_copy_not_proven_when_untruthful_example_removed() {
    let mut packet = seeded_m5_input_method_and_credential_store_wording_registries();
    for row in &mut packet.registry_rows {
        row.credential_store_wording_entries.retain(|ex| {
            ex.degrade_reason
                != Some(M5CredentialStoreWordingEntryDegradeReason::StorageClaimUntruthfulOrLeaky)
        });
    }
    assert!(packet.validate().contains(
        &M5InputCredentialRegistriesViolation::CredentialCopyTruthfulAndPrivacySafeNotProven
    ));
}

#[test]
fn credential_copy_not_proven_when_surface_dropped() {
    let mut packet = seeded_m5_input_method_and_credential_store_wording_registries();
    // Drop every clean support-diagnostics credential so the coverage no longer includes it.
    for row in &mut packet.registry_rows {
        row.credential_store_wording_entries
            .retain(|ex| !(ex.is_clean() && ex.wording_surface == "support_diagnostics"));
    }
    assert!(packet.validate().contains(
        &M5InputCredentialRegistriesViolation::CredentialCopyTruthfulAndPrivacySafeNotProven
    ));
}

#[test]
fn governance_review_incomplete_fails() {
    let mut packet = seeded_m5_input_method_and_credential_store_wording_registries();
    packet
        .governance_review
        .command_shortcut_and_trust_fidelity_preserved = false;
    assert!(packet
        .validate()
        .contains(&M5InputCredentialRegistriesViolation::GovernanceReviewIncomplete));
}

#[test]
fn consumer_projection_incomplete_fails() {
    let mut packet = seeded_m5_input_method_and_credential_store_wording_registries();
    packet
        .consumer_projection
        .support_export_reads_single_registry_source = false;
    assert!(packet
        .validate()
        .contains(&M5InputCredentialRegistriesViolation::ConsumerProjectionIncomplete));
}

#[test]
fn proof_freshness_incomplete_fails() {
    let mut packet = seeded_m5_input_method_and_credential_store_wording_registries();
    packet.proof_freshness.proof_freshness_slo_hours = 0;
    assert!(packet
        .validate()
        .contains(&M5InputCredentialRegistriesViolation::ProofFreshnessIncomplete));
}

#[test]
fn release_posture_incomplete_fails() {
    let mut packet = seeded_m5_input_method_and_credential_store_wording_registries();
    packet.release_posture.accessibility_parity_required = false;
    assert!(packet
        .validate()
        .contains(&M5InputCredentialRegistriesViolation::ReleasePostureIncomplete));
}

#[test]
fn injected_raw_material_is_rejected() {
    let mut packet = seeded_m5_input_method_and_credential_store_wording_registries();
    packet.registry_rows[0].scope_summary =
        "raw endpoint https://relay.internal.example/session leaked".to_owned();
    assert!(packet
        .validate()
        .contains(&M5InputCredentialRegistriesViolation::RawMaterialInExport));
}

#[test]
fn export_carries_no_forbidden_raw_material() {
    let json = seeded_m5_input_method_and_credential_store_wording_registries().export_safe_json();
    let lower = json.to_lowercase();
    assert!(!lower.contains("password"));
    assert!(!lower.contains("passphrase"));
    assert!(!lower.contains("bearer "));
    assert!(!lower.contains("://"));
    assert!(!lower.contains("-----begin"));
}

#[test]
fn csv_has_a_row_per_consumer_surface() {
    let packet = seeded_m5_input_method_and_credential_store_wording_registries();
    let csv = packet.render_matrix_csv();
    let lines: Vec<&str> = csv.lines().collect();
    assert_eq!(lines.len(), 1 + packet.registry_rows.len());
    assert!(lines[0].starts_with("consumer_surface,qualification,owner,"));
}

#[test]
fn markdown_summary_lists_every_consumer_surface() {
    let packet = seeded_m5_input_method_and_credential_store_wording_registries();
    let summary = packet.render_markdown_summary();
    for row in &packet.registry_rows {
        assert!(summary.contains(row.consumer_surface.as_str()));
    }
}

#[test]
fn input_composition_table_lists_only_clean_inputs() {
    let packet = seeded_m5_input_method_and_credential_store_wording_registries();
    let table = packet.render_input_composition_table();
    // The clean macOS editor composition is rendered from the registry.
    assert!(table.contains("`日本語`"));
    assert!(table.contains("marked-text composition"));
    // The clean Linux terminal composition is rendered too.
    assert!(table.contains("preedit composition"));
    // A degraded, corrupted entry never leaks into the generated table.
    assert!(!table.contains("corrupted"));
}

#[test]
fn checked_support_export_validates_and_matches_seed() {
    let from_disk = current_stable_m5_input_method_and_credential_store_wording_registries_export()
        .expect("checked M5 input-method / credential-store-wording registries export validates");
    assert_eq!(
        from_disk.packet_id,
        M5_INPUT_CREDENTIAL_REGISTRIES_PACKET_ID
    );
    assert_eq!(
        from_disk,
        seeded_m5_input_method_and_credential_store_wording_registries(),
        "checked support export drifted from the seed builder"
    );
}

#[test]
fn narrowed_variants_validate_and_keep_rows_visible() {
    let beta =
        seeded_m5_input_method_and_credential_store_wording_registries_composition_beta_narrowed();
    assert!(beta.validate().is_empty(), "{:?}", beta.validate());
    assert_eq!(beta.registry_rows.len(), 6);
    let row = beta
        .registry_rows
        .iter()
        .find(|r| r.consumer_surface == M5PlatformFitConsumerSurface::DocsHelp)
        .unwrap();
    assert_eq!(row.qualification, M5PlatformFitQualificationClass::Beta);

    let preview =
        seeded_m5_input_method_and_credential_store_wording_registries_credential_preview_narrowed(
        );
    assert!(preview.validate().is_empty(), "{:?}", preview.validate());
    assert_eq!(preview.registry_rows.len(), 6);
    let row = preview
        .registry_rows
        .iter()
        .find(|r| r.consumer_surface == M5PlatformFitConsumerSurface::CliExport)
        .unwrap();
    assert_eq!(row.qualification, M5PlatformFitQualificationClass::Preview);
}

#[test]
fn checked_narrowed_fixtures_validate_and_match_seed_builders() {
    let beta: M5InputCredentialRegistriesPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/platform/m5-input-method-and-credential-store-wording-registries/composition_beta_narrowed.json"
    )))
    .expect("composition fixture parses");
    assert!(beta.validate().is_empty());
    assert_eq!(
        beta,
        seeded_m5_input_method_and_credential_store_wording_registries_composition_beta_narrowed()
    );

    let preview: M5InputCredentialRegistriesPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/platform/m5-input-method-and-credential-store-wording-registries/credential_preview_narrowed.json"
    )))
    .expect("credential fixture parses");
    assert!(preview.validate().is_empty());
    assert_eq!(
        preview,
        seeded_m5_input_method_and_credential_store_wording_registries_credential_preview_narrowed(
        )
    );
}

#[test]
fn implemented_families_is_input_method_and_credential_store_wording() {
    assert_eq!(
        IMPLEMENTED_FAMILIES,
        [
            M5PlatformFitFamily::InputMethod,
            M5PlatformFitFamily::CredentialStoreWording
        ]
    );
}
