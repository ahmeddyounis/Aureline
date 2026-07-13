use super::*;

fn clean_notation_input() -> M5ShortcutNotationEntryResolutionInput {
    M5ShortcutNotationEntryResolutionInput {
        entry_id: "notation:test".to_owned(),
        command_id: "command.file.save".to_owned(),
        token_name: "shortcut.file.save.macos".to_owned(),
        semantic_role: M5PlatformFitRole::Shortcut,
        notation_role: M5ShortcutNotationRole::ModifierGlyphNotation,
        host_platform: M5HostPlatform::Macos,
        surface_context: M5ShortcutSurfaceContext::MenuBar,
        notation_form_coverage: M5ShortcutNotationForm::ALL.to_vec(),
        rendered_notation: "⌘S".to_owned(),
        bound_to_registry: true,
        preserves_command_id: true,
        reserved_by_os: false,
        fallback_explained: true,
        proof_fresh: true,
    }
}

fn clean_label_input() -> M5CommandLabelMappingResolutionInput {
    M5CommandLabelMappingResolutionInput {
        entry_id: "label:test".to_owned(),
        command_id: "command.file.save".to_owned(),
        token_name: "label.file.save.menu".to_owned(),
        mapping_role: M5ShortcutNotationRole::AcceleratorLabel,
        semantic_role: M5PlatformFitRole::CommandStability,
        label_kind: M5CommandLabelKind::MenuLabel,
        surface_context: M5ShortcutSurfaceContext::MenuBar,
        notation_form_coverage: M5ShortcutNotationForm::ALL.to_vec(),
        human_label: "Save".to_owned(),
        shortcut_text: "Ctrl+S".to_owned(),
        discoverable_by_id_label_and_shortcut: true,
        proof_fresh: true,
    }
}

#[test]
fn seeded_registries_validates() {
    let packet = seeded_m5_shortcut_notation_command_label_registries();
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
    assert_eq!(packet.packet_id, M5_SHORTCUT_NOTATION_REGISTRIES_PACKET_ID);
}

#[test]
fn notation_clean_names_meaning_and_is_bound() {
    let resolved = resolve_shortcut_notation_entry(clean_notation_input()).unwrap();
    assert!(resolved.is_clean());
    assert!(resolved.notation_holds_across_surfaces_and_platforms);
    assert!(resolved.covers_all_notation_forms);
    assert!(resolved.notation_matches_host);
    assert!(resolved.bound_to_registry);
    assert!(resolved.host_platform_is_classified);
    assert!(resolved.host_uses_glyph_notation);
    assert!(!resolved.notation_role_hardcoded);
    assert!(resolved.preserves_command_id);
    assert_eq!(resolved.semantic_role, "shortcut");
    assert_eq!(resolved.host_platform, "macos");
    assert_eq!(resolved.surface_context, "menu_bar");
    assert_eq!(
        resolved.next_action,
        M5ShortcutRegistryNextAction::ExpandNotationMeaning
    );
}

#[test]
fn notation_token_unstated_degrades() {
    let mut input = clean_notation_input();
    input.token_name = "   ".to_owned();
    assert_eq!(
        resolve_shortcut_notation_entry(input)
            .unwrap()
            .degrade_reason,
        Some(M5ShortcutNotationEntryDegradeReason::CommandTokenUnstated)
    );
}

#[test]
fn notation_hand_copied_and_unclassified_degrade() {
    let mut input = clean_notation_input();
    input.notation_role = M5ShortcutNotationRole::HardcodedPlatformNotationDisallowed;
    let resolved = resolve_shortcut_notation_entry(input).unwrap();
    assert!(resolved.notation_role_hardcoded);
    assert_eq!(
        resolved.degrade_reason,
        Some(M5ShortcutNotationEntryDegradeReason::NotationNotBoundToRegistry)
    );

    let mut input = clean_notation_input();
    input.bound_to_registry = false;
    assert_eq!(
        resolve_shortcut_notation_entry(input)
            .unwrap()
            .degrade_reason,
        Some(M5ShortcutNotationEntryDegradeReason::NotationNotBoundToRegistry)
    );

    let mut input = clean_notation_input();
    input.host_platform = M5HostPlatform::PlatformUnknown;
    assert_eq!(
        resolve_shortcut_notation_entry(input)
            .unwrap()
            .degrade_reason,
        Some(M5ShortcutNotationEntryDegradeReason::HostPlatformUnclassified)
    );
}

#[test]
fn notation_mislabeled_and_identity_and_form_degrade() {
    // A macOS entry rendered with a Windows modifier name is mislabeled for its host.
    let mut input = clean_notation_input();
    input.rendered_notation = "Ctrl+S".to_owned();
    let resolved = resolve_shortcut_notation_entry(input).unwrap();
    assert!(!resolved.notation_matches_host);
    assert_eq!(
        resolved.degrade_reason,
        Some(M5ShortcutNotationEntryDegradeReason::NotationMislabeledForHost)
    );

    let mut input = clean_notation_input();
    input.preserves_command_id = false;
    assert_eq!(
        resolve_shortcut_notation_entry(input)
            .unwrap()
            .degrade_reason,
        Some(M5ShortcutNotationEntryDegradeReason::CommandIdentityNotStable)
    );

    let mut input = clean_notation_input();
    input.notation_form_coverage = vec![M5ShortcutNotationForm::VisualNotation];
    assert_eq!(
        resolve_shortcut_notation_entry(input)
            .unwrap()
            .degrade_reason,
        Some(M5ShortcutNotationEntryDegradeReason::NotationFormCoverageIncomplete)
    );
}

#[test]
fn notation_reserved_and_surface_and_proof_degrade() {
    let mut input = clean_notation_input();
    input.reserved_by_os = true;
    input.fallback_explained = false;
    assert_eq!(
        resolve_shortcut_notation_entry(input)
            .unwrap()
            .degrade_reason,
        Some(M5ShortcutNotationEntryDegradeReason::ReservedKeyWithoutFallback)
    );

    let mut input = clean_notation_input();
    input.surface_context = M5ShortcutSurfaceContext::ContextUnknown;
    assert_eq!(
        resolve_shortcut_notation_entry(input)
            .unwrap()
            .degrade_reason,
        Some(M5ShortcutNotationEntryDegradeReason::SurfaceContextUnresolved)
    );

    let mut input = clean_notation_input();
    input.proof_fresh = false;
    assert_eq!(
        resolve_shortcut_notation_entry(input)
            .unwrap()
            .degrade_reason,
        Some(M5ShortcutNotationEntryDegradeReason::ProofStale)
    );
}

#[test]
fn notation_empty_id_and_forbidden_material_error() {
    let mut input = clean_notation_input();
    input.entry_id = "".to_owned();
    assert_eq!(
        resolve_shortcut_notation_entry(input).unwrap_err(),
        M5ShortcutNotationResolutionError::EmptyShortcutNotationEntryId
    );

    let mut input = clean_notation_input();
    input.token_name = "https://relay.internal/leak".to_owned();
    assert_eq!(
        resolve_shortcut_notation_entry(input).unwrap_err(),
        M5ShortcutNotationResolutionError::ForbiddenMaterial
    );
}

#[test]
fn notation_matches_host_rejects_cross_platform_notation() {
    assert!(notation_matches_host(M5HostPlatform::Macos, "⌘S"));
    assert!(!notation_matches_host(M5HostPlatform::Macos, "Ctrl+S"));
    assert!(notation_matches_host(M5HostPlatform::Windows, "Ctrl+S"));
    assert!(!notation_matches_host(M5HostPlatform::Windows, "⌘S"));
    assert!(notation_matches_host(M5HostPlatform::Linux, "Ctrl+Alt+T"));
    assert!(!notation_matches_host(
        M5HostPlatform::PlatformUnknown,
        "⌘S"
    ));
    assert!(!notation_matches_host(M5HostPlatform::Macos, "   "));
}

#[test]
fn label_clean_stays_discoverable() {
    let resolved = resolve_command_label_mapping_entry(clean_label_input()).unwrap();
    assert!(resolved.is_clean());
    assert!(resolved.command_discoverable_on_every_profile);
    assert!(resolved.covers_all_notation_forms);
    assert!(resolved.provides_complete_discovery_triple);
    assert!(!resolved.mapping_role_hardcoded);
    assert_eq!(resolved.label_kind, "menu_label");
    assert_eq!(resolved.human_label, "Save");
    assert_eq!(resolved.surface_context, "menu_bar");
}

#[test]
fn label_discovery_incomplete_and_unclassified_degrade() {
    let mut input = clean_label_input();
    input.discoverable_by_id_label_and_shortcut = false;
    let resolved = resolve_command_label_mapping_entry(input).unwrap();
    assert!(!resolved.provides_complete_discovery_triple);
    assert_eq!(
        resolved.degrade_reason,
        Some(M5CommandLabelMappingDegradeReason::DiscoveryTripleIncomplete)
    );

    let mut input = clean_label_input();
    input.shortcut_text = "   ".to_owned();
    assert_eq!(
        resolve_command_label_mapping_entry(input)
            .unwrap()
            .degrade_reason,
        Some(M5CommandLabelMappingDegradeReason::DiscoveryTripleIncomplete)
    );

    let mut input = clean_label_input();
    input.mapping_role = M5ShortcutNotationRole::HardcodedPlatformNotationDisallowed;
    assert_eq!(
        resolve_command_label_mapping_entry(input)
            .unwrap()
            .degrade_reason,
        Some(M5CommandLabelMappingDegradeReason::DiscoveryTripleIncomplete)
    );

    let mut input = clean_label_input();
    input.label_kind = M5CommandLabelKind::LabelUnclassified;
    assert_eq!(
        resolve_command_label_mapping_entry(input)
            .unwrap()
            .degrade_reason,
        Some(M5CommandLabelMappingDegradeReason::LabelKindUnclassified)
    );
}

#[test]
fn label_form_and_surface_and_id_and_material() {
    let mut input = clean_label_input();
    input.notation_form_coverage = vec![M5ShortcutNotationForm::VisualNotation];
    assert_eq!(
        resolve_command_label_mapping_entry(input)
            .unwrap()
            .degrade_reason,
        Some(M5CommandLabelMappingDegradeReason::NotationFormCoverageIncomplete)
    );

    let mut input = clean_label_input();
    input.surface_context = M5ShortcutSurfaceContext::ContextUnknown;
    assert_eq!(
        resolve_command_label_mapping_entry(input)
            .unwrap()
            .degrade_reason,
        Some(M5CommandLabelMappingDegradeReason::SurfaceContextUnresolved)
    );

    let mut input = clean_label_input();
    input.entry_id = "  ".to_owned();
    assert_eq!(
        resolve_command_label_mapping_entry(input).unwrap_err(),
        M5ShortcutNotationResolutionError::EmptyCommandLabelEntryId
    );

    let mut input = clean_label_input();
    input.human_label = "see internal://notes".to_owned();
    assert_eq!(
        resolve_command_label_mapping_entry(input).unwrap_err(),
        M5ShortcutNotationResolutionError::ForbiddenMaterial
    );
}

#[test]
fn vocabulary_set_is_canonical() {
    assert!(seeded_m5_shortcut_notation_command_label_registries()
        .vocabulary_set
        .matches_canonical());
}

#[test]
fn vocabulary_set_drift_fails() {
    let mut packet = seeded_m5_shortcut_notation_command_label_registries();
    packet.vocabulary_set.host_platforms.pop();
    assert!(packet
        .validate()
        .contains(&M5ShortcutNotationRegistriesViolation::VocabularySetDrift));
}

#[test]
fn missing_source_contracts_fails() {
    let mut packet = seeded_m5_shortcut_notation_command_label_registries();
    packet.source_contract_refs.clear();
    assert!(packet
        .validate()
        .contains(&M5ShortcutNotationRegistriesViolation::MissingSourceContracts));
}

#[test]
fn domain_schema_ref_missing_fails() {
    let mut packet = seeded_m5_shortcut_notation_command_label_registries();
    packet.registry_rows[0]
        .source_contract_refs
        .retain(|r| r != M5_SHORTCUT_NOTATION_SCHEMA_REF);
    assert!(packet
        .validate()
        .contains(&M5ShortcutNotationRegistriesViolation::DomainSchemaRefMissing));
}

#[test]
fn mandatory_anatomy_missing_fails() {
    let mut packet = seeded_m5_shortcut_notation_command_label_registries();
    packet.registry_rows[0]
        .anatomy_parts
        .retain(|p| *p != M5ShortcutRegistryAnatomyPart::Identity);
    assert!(packet
        .validate()
        .contains(&M5ShortcutNotationRegistriesViolation::MandatoryAnatomyMissing));
}

#[test]
fn mandatory_export_field_missing_fails() {
    let mut packet = seeded_m5_shortcut_notation_command_label_registries();
    packet.registry_rows[0]
        .export_fields
        .retain(|f| *f != M5ShortcutRegistryExportField::HostPlatforms);
    assert!(packet
        .validate()
        .contains(&M5ShortcutNotationRegistriesViolation::MandatoryExportFieldMissing));
}

#[test]
fn examples_missing_fails() {
    let mut packet = seeded_m5_shortcut_notation_command_label_registries();
    packet.registry_rows[0].command_label_entries.clear();
    assert!(packet
        .validate()
        .contains(&M5ShortcutNotationRegistriesViolation::ExamplesMissing));
}

#[test]
fn dishonest_clean_example_fails() {
    let mut packet = seeded_m5_shortcut_notation_command_label_registries();
    // Force a clean shortcut-notation entry to also read as mislabeled — the packet must reject it.
    let row = &mut packet.registry_rows[0];
    row.shortcut_notation_entries[0].degrade_reason = None;
    row.shortcut_notation_entries[0].notation_matches_host = false;
    assert!(packet
        .validate()
        .contains(&M5ShortcutNotationRegistriesViolation::DishonestExample));
}

#[test]
fn row_invariant_violation_fails() {
    for mutate in 0u8..4 {
        let mut packet = seeded_m5_shortcut_notation_command_label_registries();
        let row = &mut packet.registry_rows[0];
        match mutate {
            0 => row.notation_changes_command_or_permission_meaning = true,
            1 => row.primary_command_hidden_only_in_os_chrome = true,
            2 => row.notation_hardcoded_instead_of_registry = true,
            _ => row.screenshot_or_docs_mislabels_shortcut = true,
        }
        assert!(packet
            .validate()
            .contains(&M5ShortcutNotationRegistriesViolation::RowInvariantViolated));
    }
}

#[test]
fn same_command_not_proven_when_hand_copied_example_removed() {
    let mut packet = seeded_m5_shortcut_notation_command_label_registries();
    for row in &mut packet.registry_rows {
        row.shortcut_notation_entries.retain(|ex| {
            ex.degrade_reason
                != Some(M5ShortcutNotationEntryDegradeReason::NotationNotBoundToRegistry)
        });
    }
    assert!(packet.validate().contains(
        &M5ShortcutNotationRegistriesViolation::SameCommandDiscoverableAcrossSurfacesNotProven
    ));
}

#[test]
fn same_command_not_proven_when_surface_collapses() {
    let mut packet = seeded_m5_shortcut_notation_command_label_registries();
    // Drop every clean onboarding notation so the first-consumer surfaces no longer include it.
    for row in &mut packet.registry_rows {
        row.shortcut_notation_entries
            .retain(|ex| !(ex.is_clean() && ex.surface_context == "onboarding"));
    }
    assert!(packet.validate().contains(
        &M5ShortcutNotationRegistriesViolation::SameCommandDiscoverableAcrossSurfacesNotProven
    ));
}

#[test]
fn command_discoverable_not_proven_when_discovery_example_removed() {
    let mut packet = seeded_m5_shortcut_notation_command_label_registries();
    for row in &mut packet.registry_rows {
        row.command_label_entries.retain(|ex| {
            ex.degrade_reason != Some(M5CommandLabelMappingDegradeReason::DiscoveryTripleIncomplete)
        });
    }
    assert!(packet.validate().contains(
        &M5ShortcutNotationRegistriesViolation::CommandDiscoverableOnEveryProfileNotProven
    ));
}

#[test]
fn command_discoverable_not_proven_when_label_kind_dropped() {
    let mut packet = seeded_m5_shortcut_notation_command_label_registries();
    // Drop every clean help-label mapping so the coverage no longer includes it.
    for row in &mut packet.registry_rows {
        row.command_label_entries
            .retain(|ex| !(ex.is_clean() && ex.label_kind == "help_label"));
    }
    assert!(packet.validate().contains(
        &M5ShortcutNotationRegistriesViolation::CommandDiscoverableOnEveryProfileNotProven
    ));
}

#[test]
fn wrong_notation_not_proven_when_mislabeled_example_removed() {
    let mut packet = seeded_m5_shortcut_notation_command_label_registries();
    for row in &mut packet.registry_rows {
        row.shortcut_notation_entries.retain(|ex| {
            ex.degrade_reason
                != Some(M5ShortcutNotationEntryDegradeReason::NotationMislabeledForHost)
        });
    }
    assert!(packet
        .validate()
        .contains(&M5ShortcutNotationRegistriesViolation::WrongNotationOrLabelDetectableNotProven));
}

#[test]
fn governance_review_incomplete_fails() {
    let mut packet = seeded_m5_shortcut_notation_command_label_registries();
    packet
        .governance_review
        .command_discoverable_by_id_label_and_shortcut = false;
    assert!(packet
        .validate()
        .contains(&M5ShortcutNotationRegistriesViolation::GovernanceReviewIncomplete));
}

#[test]
fn consumer_projection_incomplete_fails() {
    let mut packet = seeded_m5_shortcut_notation_command_label_registries();
    packet
        .consumer_projection
        .support_export_reads_single_registry_source = false;
    assert!(packet
        .validate()
        .contains(&M5ShortcutNotationRegistriesViolation::ConsumerProjectionIncomplete));
}

#[test]
fn proof_freshness_incomplete_fails() {
    let mut packet = seeded_m5_shortcut_notation_command_label_registries();
    packet.proof_freshness.proof_freshness_slo_hours = 0;
    assert!(packet
        .validate()
        .contains(&M5ShortcutNotationRegistriesViolation::ProofFreshnessIncomplete));
}

#[test]
fn release_posture_incomplete_fails() {
    let mut packet = seeded_m5_shortcut_notation_command_label_registries();
    packet.release_posture.accessibility_parity_required = false;
    assert!(packet
        .validate()
        .contains(&M5ShortcutNotationRegistriesViolation::ReleasePostureIncomplete));
}

#[test]
fn injected_raw_material_is_rejected() {
    let mut packet = seeded_m5_shortcut_notation_command_label_registries();
    packet.registry_rows[0].scope_summary =
        "raw endpoint https://relay.internal.example/session leaked".to_owned();
    assert!(packet
        .validate()
        .contains(&M5ShortcutNotationRegistriesViolation::RawMaterialInExport));
}

#[test]
fn export_carries_no_forbidden_raw_material() {
    let json = seeded_m5_shortcut_notation_command_label_registries().export_safe_json();
    let lower = json.to_lowercase();
    assert!(!lower.contains("password"));
    assert!(!lower.contains("passphrase"));
    assert!(!lower.contains("bearer "));
    assert!(!lower.contains("://"));
    assert!(!lower.contains("-----begin"));
}

#[test]
fn csv_has_a_row_per_consumer_surface() {
    let packet = seeded_m5_shortcut_notation_command_label_registries();
    let csv = packet.render_matrix_csv();
    let lines: Vec<&str> = csv.lines().collect();
    assert_eq!(lines.len(), 1 + packet.registry_rows.len());
    assert!(lines[0].starts_with("consumer_surface,qualification,owner,"));
}

#[test]
fn markdown_summary_lists_every_consumer_surface() {
    let packet = seeded_m5_shortcut_notation_command_label_registries();
    let summary = packet.render_markdown_summary();
    for row in &packet.registry_rows {
        assert!(summary.contains(row.consumer_surface.as_str()));
    }
}

#[test]
fn help_notation_table_lists_only_clean_notation() {
    let packet = seeded_m5_shortcut_notation_command_label_registries();
    let table = packet.render_platform_help_notation_table();
    // The clean macOS Save notation is rendered from the registry.
    assert!(table.contains("`⌘S`"));
    assert!(table.contains("command.file.save"));
    // The clean Windows palette accelerator is rendered too.
    assert!(table.contains("`Ctrl+Shift+P`"));
    // A degraded, mislabeled entry never leaks into the generated help table.
    assert!(!table.contains("mislabeled"));
}

#[test]
fn checked_support_export_validates_and_matches_seed() {
    let from_disk = current_stable_m5_shortcut_notation_command_label_registries_export()
        .expect("checked M5 shortcut-notation / command-label registries export validates");
    assert_eq!(
        from_disk.packet_id,
        M5_SHORTCUT_NOTATION_REGISTRIES_PACKET_ID
    );
    assert_eq!(
        from_disk,
        seeded_m5_shortcut_notation_command_label_registries(),
        "checked support export drifted from the seed builder"
    );
}

#[test]
fn narrowed_variants_validate_and_keep_rows_visible() {
    let beta = seeded_m5_shortcut_notation_command_label_registries_docs_help_beta_narrowed();
    assert!(beta.validate().is_empty(), "{:?}", beta.validate());
    assert_eq!(beta.registry_rows.len(), 6);
    let row = beta
        .registry_rows
        .iter()
        .find(|r| r.consumer_surface == M5PlatformFitConsumerSurface::DocsHelp)
        .unwrap();
    assert_eq!(row.qualification, M5PlatformFitQualificationClass::Beta);

    let preview =
        seeded_m5_shortcut_notation_command_label_registries_onboarding_preview_narrowed();
    assert!(preview.validate().is_empty(), "{:?}", preview.validate());
    assert_eq!(preview.registry_rows.len(), 6);
    let row = preview
        .registry_rows
        .iter()
        .find(|r| r.consumer_surface == M5PlatformFitConsumerSurface::Onboarding)
        .unwrap();
    assert_eq!(row.qualification, M5PlatformFitQualificationClass::Preview);
}

#[test]
fn checked_narrowed_fixtures_validate_and_match_seed_builders() {
    let beta: M5ShortcutNotationRegistriesPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/platform/m5-shortcut-notation-and-command-label-registries/docs_help_beta_narrowed.json"
    )))
    .expect("docs-help fixture parses");
    assert!(beta.validate().is_empty());
    assert_eq!(
        beta,
        seeded_m5_shortcut_notation_command_label_registries_docs_help_beta_narrowed()
    );

    let preview: M5ShortcutNotationRegistriesPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/platform/m5-shortcut-notation-and-command-label-registries/onboarding_preview_narrowed.json"
    )))
    .expect("onboarding fixture parses");
    assert!(preview.validate().is_empty());
    assert_eq!(
        preview,
        seeded_m5_shortcut_notation_command_label_registries_onboarding_preview_narrowed()
    );
}

#[test]
fn implemented_families_is_shortcut_notation() {
    assert_eq!(
        IMPLEMENTED_FAMILIES,
        [M5PlatformFitFamily::ShortcutNotation]
    );
}
