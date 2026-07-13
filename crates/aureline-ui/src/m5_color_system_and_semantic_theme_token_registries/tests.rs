use super::*;

fn clean_color_input() -> M5ColorEntryResolutionInput {
    M5ColorEntryResolutionInput {
        entry_id: "color:test".to_owned(),
        token_name: "color.status.danger".to_owned(),
        semantic_role: M5VisualSemanticRole::Status,
        color_role: M5ColorRoleFamily::StatusPalette,
        operational_state: M5OperationalStateFamily::Danger,
        non_color_cue: M5NonColorCue::IconGlyph,
        surface_context: M5ColorRegistrySurfaceContext::Review,
        defined_modes: M5ThemeMode::ALL.to_vec(),
        meaning_stated_non_color_only: true,
        distinguishable_in_all_modes: true,
        references_canonical_token: true,
        proof_fresh: true,
    }
}

fn clean_theme_input() -> M5ThemeTokenEntryResolutionInput {
    M5ThemeTokenEntryResolutionInput {
        entry_id: "theme:test".to_owned(),
        token_name: "theme.surface.base".to_owned(),
        theme_token_role: M5ThemeTokenRole::SurfaceRole,
        semantic_role: M5VisualSemanticRole::Neutral,
        surface_context: M5ColorRegistrySurfaceContext::Shell,
        defined_modes: M5ThemeMode::ALL.to_vec(),
        references_canonical_token: true,
        role_stable_across_surfaces: true,
        proof_fresh: true,
    }
}

#[test]
fn seeded_registries_validates() {
    let packet = seeded_m5_color_theme_registries();
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
    assert_eq!(packet.packet_id, M5_COLOR_THEME_REGISTRIES_PACKET_ID);
}

#[test]
fn color_clean_names_meaning_and_is_distinct() {
    let resolved = resolve_color_entry(clean_color_input()).unwrap();
    assert!(resolved.is_clean());
    assert!(resolved.meaning_distinct_across_modes);
    assert!(resolved.covers_all_modes);
    assert!(resolved.non_color_cue_present);
    assert!(resolved.operational_state_is_classified);
    assert!(!resolved.color_role_is_hue_alone);
    assert_eq!(resolved.semantic_role, "status");
    assert_eq!(resolved.operational_state, "danger");
    assert_eq!(resolved.surface_context, "review");
    assert_eq!(
        resolved.next_action,
        M5ColorRegistryNextAction::ExpandColorMeaning
    );
}

#[test]
fn color_token_unstated_degrades() {
    let mut input = clean_color_input();
    input.token_name = "   ".to_owned();
    assert_eq!(
        resolve_color_entry(input).unwrap().degrade_reason,
        Some(M5ColorEntryDegradeReason::TokenNameUnstated)
    );
}

#[test]
fn color_only_and_cue_missing_degrade() {
    let mut input = clean_color_input();
    input.meaning_stated_non_color_only = false;
    assert_eq!(
        resolve_color_entry(input).unwrap().degrade_reason,
        Some(M5ColorEntryDegradeReason::MeaningEncodedByColorAlone)
    );

    let mut input = clean_color_input();
    input.color_role = M5ColorRoleFamily::HueAloneMeaningDisallowed;
    let resolved = resolve_color_entry(input).unwrap();
    assert!(resolved.color_role_is_hue_alone);
    assert_eq!(
        resolved.degrade_reason,
        Some(M5ColorEntryDegradeReason::MeaningEncodedByColorAlone)
    );

    let mut input = clean_color_input();
    input.non_color_cue = M5NonColorCue::NoneDisallowed;
    assert_eq!(
        resolve_color_entry(input).unwrap().degrade_reason,
        Some(M5ColorEntryDegradeReason::NonColorCueMissing)
    );
}

#[test]
fn color_raw_inlined_and_unclassified_degrade() {
    let mut input = clean_color_input();
    input.references_canonical_token = false;
    assert_eq!(
        resolve_color_entry(input).unwrap().degrade_reason,
        Some(M5ColorEntryDegradeReason::RawColorValueInlined)
    );

    let mut input = clean_color_input();
    input.operational_state = M5OperationalStateFamily::StateUnclassified;
    assert_eq!(
        resolve_color_entry(input).unwrap().degrade_reason,
        Some(M5ColorEntryDegradeReason::OperationalStateUnclassified)
    );
}

#[test]
fn color_mode_parity_and_distinguishability_degrade() {
    let mut input = clean_color_input();
    input.defined_modes = vec![M5ThemeMode::Dark, M5ThemeMode::Light];
    assert_eq!(
        resolve_color_entry(input).unwrap().degrade_reason,
        Some(M5ColorEntryDegradeReason::ThemeModeParityIncomplete)
    );

    let mut input = clean_color_input();
    input.distinguishable_in_all_modes = false;
    assert_eq!(
        resolve_color_entry(input).unwrap().degrade_reason,
        Some(M5ColorEntryDegradeReason::StateIndistinguishableAcrossModes)
    );

    let mut input = clean_color_input();
    input.surface_context = M5ColorRegistrySurfaceContext::ContextUnknown;
    assert_eq!(
        resolve_color_entry(input).unwrap().degrade_reason,
        Some(M5ColorEntryDegradeReason::SurfaceContextUnresolved)
    );
}

#[test]
fn color_empty_id_and_forbidden_material_error() {
    let mut input = clean_color_input();
    input.entry_id = "".to_owned();
    assert_eq!(
        resolve_color_entry(input).unwrap_err(),
        M5ColorThemeResolutionError::EmptyColorEntryId
    );

    let mut input = clean_color_input();
    input.token_name = "https://relay.internal/leak".to_owned();
    assert_eq!(
        resolve_color_entry(input).unwrap_err(),
        M5ColorThemeResolutionError::ForbiddenMaterial
    );
}

#[test]
fn theme_clean_stays_stable_across_pair() {
    let resolved = resolve_theme_token_entry(clean_theme_input()).unwrap();
    assert!(resolved.is_clean());
    assert!(resolved.token_stable_across_theme_pair);
    assert!(resolved.covers_all_modes);
    assert!(!resolved.theme_token_role_is_raw_hex);
    assert_eq!(resolved.theme_token_role, "surface_role");
    assert_eq!(resolved.surface_context, "shell");
}

#[test]
fn theme_raw_hex_and_pair_incomplete_degrade() {
    let mut input = clean_theme_input();
    input.theme_token_role = M5ThemeTokenRole::RawHexInSurfaceDisallowed;
    let resolved = resolve_theme_token_entry(input).unwrap();
    assert!(resolved.theme_token_role_is_raw_hex);
    assert_eq!(
        resolved.degrade_reason,
        Some(M5ThemeTokenEntryDegradeReason::RawHexInlinedInSurface)
    );

    let mut input = clean_theme_input();
    input.references_canonical_token = false;
    assert_eq!(
        resolve_theme_token_entry(input).unwrap().degrade_reason,
        Some(M5ThemeTokenEntryDegradeReason::RawHexInlinedInSurface)
    );

    let mut input = clean_theme_input();
    input.defined_modes = vec![M5ThemeMode::Dark];
    assert_eq!(
        resolve_theme_token_entry(input).unwrap().degrade_reason,
        Some(M5ThemeTokenEntryDegradeReason::ThemePairIncomplete)
    );
}

#[test]
fn theme_role_drift_and_surface_and_id_and_material() {
    let mut input = clean_theme_input();
    input.role_stable_across_surfaces = false;
    assert_eq!(
        resolve_theme_token_entry(input).unwrap().degrade_reason,
        Some(M5ThemeTokenEntryDegradeReason::ThemeRoleDriftedAcrossSurface)
    );

    let mut input = clean_theme_input();
    input.surface_context = M5ColorRegistrySurfaceContext::ContextUnknown;
    assert_eq!(
        resolve_theme_token_entry(input).unwrap().degrade_reason,
        Some(M5ThemeTokenEntryDegradeReason::SurfaceContextUnresolved)
    );

    let mut input = clean_theme_input();
    input.entry_id = "  ".to_owned();
    assert_eq!(
        resolve_theme_token_entry(input).unwrap_err(),
        M5ColorThemeResolutionError::EmptyThemeTokenEntryId
    );

    let mut input = clean_theme_input();
    input.token_name = "see internal://notes".to_owned();
    assert_eq!(
        resolve_theme_token_entry(input).unwrap_err(),
        M5ColorThemeResolutionError::ForbiddenMaterial
    );
}

#[test]
fn vocabulary_set_is_canonical() {
    assert!(seeded_m5_color_theme_registries()
        .vocabulary_set
        .matches_canonical());
}

#[test]
fn vocabulary_set_drift_fails() {
    let mut packet = seeded_m5_color_theme_registries();
    packet.vocabulary_set.operational_states.pop();
    assert!(packet
        .validate()
        .contains(&M5ColorThemeRegistriesViolation::VocabularySetDrift));
}

#[test]
fn missing_source_contracts_fails() {
    let mut packet = seeded_m5_color_theme_registries();
    packet.source_contract_refs.clear();
    assert!(packet
        .validate()
        .contains(&M5ColorThemeRegistriesViolation::MissingSourceContracts));
}

#[test]
fn domain_schema_ref_missing_fails() {
    let mut packet = seeded_m5_color_theme_registries();
    packet.registry_rows[0]
        .source_contract_refs
        .retain(|r| r != M5_COLOR_SYSTEM_SCHEMA_REF);
    assert!(packet
        .validate()
        .contains(&M5ColorThemeRegistriesViolation::DomainSchemaRefMissing));
}

#[test]
fn mandatory_anatomy_missing_fails() {
    let mut packet = seeded_m5_color_theme_registries();
    packet.registry_rows[0]
        .anatomy_parts
        .retain(|p| *p != M5ColorRegistryAnatomyPart::Identity);
    assert!(packet
        .validate()
        .contains(&M5ColorThemeRegistriesViolation::MandatoryAnatomyMissing));
}

#[test]
fn mandatory_export_field_missing_fails() {
    let mut packet = seeded_m5_color_theme_registries();
    packet.registry_rows[0]
        .export_fields
        .retain(|f| *f != M5ColorRegistryExportField::OperationalStates);
    assert!(packet
        .validate()
        .contains(&M5ColorThemeRegistriesViolation::MandatoryExportFieldMissing));
}

#[test]
fn examples_missing_fails() {
    let mut packet = seeded_m5_color_theme_registries();
    packet.registry_rows[0].theme_token_entries.clear();
    assert!(packet
        .validate()
        .contains(&M5ColorThemeRegistriesViolation::ExamplesMissing));
}

#[test]
fn dishonest_clean_example_fails() {
    let mut packet = seeded_m5_color_theme_registries();
    // Force a clean color entry to also read as color-only meaning — the packet must reject it.
    let row = &mut packet.registry_rows[0];
    row.color_entries[0].degrade_reason = None;
    row.color_entries[0].meaning_stated_non_color_only = false;
    assert!(packet
        .validate()
        .contains(&M5ColorThemeRegistriesViolation::DishonestExample));
}

#[test]
fn row_invariant_violation_fails() {
    for mutate in 0u8..4 {
        let mut packet = seeded_m5_color_theme_registries();
        let row = &mut packet.registry_rows[0];
        match mutate {
            0 => row.status_meaning_relies_on_color_alone = true,
            1 => row.raw_color_value_inlined_instead_of_token = true,
            2 => row.operational_state_indistinguishable_across_modes = true,
            _ => row.theme_mode_parity_incomplete = true,
        }
        assert!(packet
            .validate()
            .contains(&M5ColorThemeRegistriesViolation::RowInvariantViolated));
    }
}

#[test]
fn first_consumers_not_proven_when_raw_color_example_removed() {
    let mut packet = seeded_m5_color_theme_registries();
    for row in &mut packet.registry_rows {
        row.color_entries.retain(|ex| {
            ex.degrade_reason != Some(M5ColorEntryDegradeReason::RawColorValueInlined)
        });
    }
    assert!(packet
        .validate()
        .contains(&M5ColorThemeRegistriesViolation::FirstConsumersUseCanonicalFamiliesNotProven));
}

#[test]
fn first_consumers_not_proven_when_semantic_family_collapses() {
    let mut packet = seeded_m5_color_theme_registries();
    // Drop every clean brand color so the semantic-role grammar no longer covers "brand".
    for row in &mut packet.registry_rows {
        row.color_entries
            .retain(|ex| !(ex.is_clean() && ex.semantic_role == "brand"));
    }
    assert!(packet
        .validate()
        .contains(&M5ColorThemeRegistriesViolation::FirstConsumersUseCanonicalFamiliesNotProven));
}

#[test]
fn state_distinguishability_not_proven_when_mode_incomplete_example_removed() {
    let mut packet = seeded_m5_color_theme_registries();
    for row in &mut packet.registry_rows {
        row.color_entries.retain(|ex| {
            ex.degrade_reason != Some(M5ColorEntryDegradeReason::ThemeModeParityIncomplete)
        });
    }
    assert!(packet
        .validate()
        .contains(&M5ColorThemeRegistriesViolation::StateDistinguishabilityAcrossModesNotProven));
}

#[test]
fn state_distinguishability_not_proven_when_trust_state_dropped() {
    let mut packet = seeded_m5_color_theme_registries();
    // Drop every clean AI color so the trust-sensitive coverage no longer includes "ai".
    for row in &mut packet.registry_rows {
        row.color_entries
            .retain(|ex| !(ex.is_clean() && ex.operational_state == "ai"));
    }
    assert!(packet
        .validate()
        .contains(&M5ColorThemeRegistriesViolation::StateDistinguishabilityAcrossModesNotProven));
}

#[test]
fn raw_color_drift_not_proven_when_raw_hex_example_removed() {
    let mut packet = seeded_m5_color_theme_registries();
    for row in &mut packet.registry_rows {
        row.theme_token_entries.retain(|ex| {
            ex.degrade_reason != Some(M5ThemeTokenEntryDegradeReason::RawHexInlinedInSurface)
        });
    }
    assert!(packet
        .validate()
        .contains(&M5ColorThemeRegistriesViolation::RawColorDriftNotDetectableNotProven));
}

#[test]
fn governance_review_incomplete_fails() {
    let mut packet = seeded_m5_color_theme_registries();
    packet
        .governance_review
        .trust_sensitive_states_distinguishable_in_every_mode = false;
    assert!(packet
        .validate()
        .contains(&M5ColorThemeRegistriesViolation::GovernanceReviewIncomplete));
}

#[test]
fn consumer_projection_incomplete_fails() {
    let mut packet = seeded_m5_color_theme_registries();
    packet
        .consumer_projection
        .support_export_reads_single_registry_source = false;
    assert!(packet
        .validate()
        .contains(&M5ColorThemeRegistriesViolation::ConsumerProjectionIncomplete));
}

#[test]
fn proof_freshness_incomplete_fails() {
    let mut packet = seeded_m5_color_theme_registries();
    packet.proof_freshness.proof_freshness_slo_hours = 0;
    assert!(packet
        .validate()
        .contains(&M5ColorThemeRegistriesViolation::ProofFreshnessIncomplete));
}

#[test]
fn release_posture_incomplete_fails() {
    let mut packet = seeded_m5_color_theme_registries();
    packet.release_posture.accessibility_parity_required = false;
    assert!(packet
        .validate()
        .contains(&M5ColorThemeRegistriesViolation::ReleasePostureIncomplete));
}

#[test]
fn injected_raw_material_is_rejected() {
    let mut packet = seeded_m5_color_theme_registries();
    packet.registry_rows[0].scope_summary =
        "raw endpoint https://relay.internal.example/session leaked".to_owned();
    assert!(packet
        .validate()
        .contains(&M5ColorThemeRegistriesViolation::RawMaterialInExport));
}

#[test]
fn export_carries_no_forbidden_raw_material() {
    let json = seeded_m5_color_theme_registries().export_safe_json();
    let lower = json.to_lowercase();
    assert!(!lower.contains("password"));
    assert!(!lower.contains("passphrase"));
    assert!(!lower.contains("bearer "));
    assert!(!lower.contains("://"));
    assert!(!lower.contains("-----begin"));
}

#[test]
fn csv_has_a_row_per_consumer_surface() {
    let packet = seeded_m5_color_theme_registries();
    let csv = packet.render_matrix_csv();
    let lines: Vec<&str> = csv.lines().collect();
    assert_eq!(lines.len(), 1 + packet.registry_rows.len());
    assert!(lines[0].starts_with("consumer_surface,qualification,owner,"));
}

#[test]
fn markdown_summary_lists_every_consumer_surface() {
    let packet = seeded_m5_color_theme_registries();
    let summary = packet.render_markdown_summary();
    for row in &packet.registry_rows {
        assert!(summary.contains(row.consumer_surface.as_str()));
    }
}

#[test]
fn checked_support_export_validates_and_matches_seed() {
    let from_disk = current_stable_m5_color_theme_registries_export()
        .expect("checked M5 color / theme registries export validates");
    assert_eq!(from_disk.packet_id, M5_COLOR_THEME_REGISTRIES_PACKET_ID);
    assert_eq!(
        from_disk,
        seeded_m5_color_theme_registries(),
        "checked support export drifted from the seed builder"
    );
}

#[test]
fn narrowed_variants_validate_and_keep_rows_visible() {
    let beta = seeded_m5_color_theme_registries_shell_ui_beta_narrowed();
    assert!(beta.validate().is_empty(), "{:?}", beta.validate());
    assert_eq!(beta.registry_rows.len(), 6);
    let row = beta
        .registry_rows
        .iter()
        .find(|r| r.consumer_surface == M5VisualFoundationConsumerSurface::ShellUi)
        .unwrap();
    assert_eq!(
        row.qualification,
        M5VisualFoundationQualificationClass::Beta
    );

    let preview = seeded_m5_color_theme_registries_data_ui_preview_narrowed();
    assert!(preview.validate().is_empty(), "{:?}", preview.validate());
    assert_eq!(preview.registry_rows.len(), 6);
    let row = preview
        .registry_rows
        .iter()
        .find(|r| r.consumer_surface == M5VisualFoundationConsumerSurface::DataUi)
        .unwrap();
    assert_eq!(
        row.qualification,
        M5VisualFoundationQualificationClass::Preview
    );
}

#[test]
fn checked_narrowed_fixtures_validate_and_match_seed_builders() {
    let beta: M5ColorThemeRegistriesPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/ui/m5-color-system-and-semantic-theme-token-registries/shell_ui_beta_narrowed.json"
    )))
    .expect("shell-ui fixture parses");
    assert!(beta.validate().is_empty());
    assert_eq!(
        beta,
        seeded_m5_color_theme_registries_shell_ui_beta_narrowed()
    );

    let preview: M5ColorThemeRegistriesPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/ui/m5-color-system-and-semantic-theme-token-registries/data_ui_preview_narrowed.json"
    )))
    .expect("data-ui fixture parses");
    assert!(preview.validate().is_empty());
    assert_eq!(
        preview,
        seeded_m5_color_theme_registries_data_ui_preview_narrowed()
    );
}

#[test]
fn implemented_families_are_color_system_and_semantic_theme_token() {
    assert_eq!(
        IMPLEMENTED_FAMILIES,
        [
            M5VisualFoundationFamily::ColorSystem,
            M5VisualFoundationFamily::SemanticThemeToken
        ]
    );
}
