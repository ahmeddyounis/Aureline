use super::*;

fn clean_path_input() -> M5FilePathPresentationEntryResolutionInput {
    M5FilePathPresentationEntryResolutionInput {
        entry_id: "path:test".to_owned(),
        command_id: "command.file.reveal".to_owned(),
        token_name: "path.reveal.macos".to_owned(),
        semantic_role: M5PlatformFitRole::PathTerminology,
        path_role: M5FilePathRevealRole::RevealVerb,
        host_platform: M5HostPlatform::Macos,
        surface_context: M5FilePathSurfaceContext::RevealMenu,
        presentation_form_coverage: M5PathPresentationForm::ALL.to_vec(),
        rendered_path: "/Users/ana/Documents".to_owned(),
        reveal_verb: "Reveal in Finder".to_owned(),
        bound_to_registry: true,
        preserves_canonical_path_truth: true,
        reveal_target_unavailable: false,
        fallback_explained: true,
        proof_fresh: true,
    }
}

fn clean_action_input() -> M5WindowMenuActionEntryResolutionInput {
    M5WindowMenuActionEntryResolutionInput {
        entry_id: "action:test".to_owned(),
        command_id: "command.file.reveal".to_owned(),
        token_name: "action.file.reveal.palette".to_owned(),
        action_role: M5PlatformConventionRole::BoundToPlatformRegistry,
        semantic_role: M5PlatformFitRole::WindowMenu,
        action_surface: M5ProductActionSurface::CommandPalette,
        surface_context: M5FilePathSurfaceContext::RevealMenu,
        presentation_form_coverage: M5PathPresentationForm::ALL.to_vec(),
        human_label: "Reveal in Finder".to_owned(),
        in_product_route: "command.file.reveal".to_owned(),
        reachable_by_id_surface_and_command: true,
        proof_fresh: true,
    }
}

#[test]
fn seeded_registries_validates() {
    let packet = seeded_m5_file_path_reveal_and_native_window_menu_registries();
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
    assert_eq!(packet.packet_id, M5_FILE_PATH_REVEAL_REGISTRIES_PACKET_ID);
}

#[test]
fn path_clean_names_meaning_and_is_bound() {
    let resolved = resolve_file_path_presentation_entry(clean_path_input()).unwrap();
    assert!(resolved.is_clean());
    assert!(resolved.path_truth_holds_across_surfaces_and_platforms);
    assert!(resolved.covers_all_presentation_forms);
    assert!(resolved.path_matches_host);
    assert!(resolved.bound_to_registry);
    assert!(resolved.host_platform_is_classified);
    assert!(!resolved.host_uses_backslash_separator);
    assert!(!resolved.path_role_mislabeled);
    assert!(resolved.preserves_canonical_path_truth);
    assert_eq!(resolved.semantic_role, "path_terminology");
    assert_eq!(resolved.host_platform, "macos");
    assert_eq!(resolved.canonical_reveal_verb, "Reveal in Finder");
    assert_eq!(resolved.surface_context, "reveal_menu");
    assert_eq!(
        resolved.next_action,
        M5FilePathRegistryNextAction::ExpandPathMeaning
    );
}

#[test]
fn path_token_unstated_degrades() {
    let mut input = clean_path_input();
    input.token_name = "   ".to_owned();
    assert_eq!(
        resolve_file_path_presentation_entry(input)
            .unwrap()
            .degrade_reason,
        Some(M5FilePathPresentationEntryDegradeReason::PathTokenUnstated)
    );
}

#[test]
fn path_hand_copied_and_unclassified_degrade() {
    let mut input = clean_path_input();
    input.path_role = M5FilePathRevealRole::MislabeledPathVerbDisallowed;
    let resolved = resolve_file_path_presentation_entry(input).unwrap();
    assert!(resolved.path_role_mislabeled);
    assert_eq!(
        resolved.degrade_reason,
        Some(M5FilePathPresentationEntryDegradeReason::TerminologyNotBoundToRegistry)
    );

    let mut input = clean_path_input();
    input.bound_to_registry = false;
    assert_eq!(
        resolve_file_path_presentation_entry(input)
            .unwrap()
            .degrade_reason,
        Some(M5FilePathPresentationEntryDegradeReason::TerminologyNotBoundToRegistry)
    );

    let mut input = clean_path_input();
    input.host_platform = M5HostPlatform::PlatformUnknown;
    assert_eq!(
        resolve_file_path_presentation_entry(input)
            .unwrap()
            .degrade_reason,
        Some(M5FilePathPresentationEntryDegradeReason::HostPlatformUnclassified)
    );
}

#[test]
fn path_mislabeled_and_canonical_and_form_degrade() {
    // A macOS entry rendered with a backslash separator is mislabeled for its host.
    let mut input = clean_path_input();
    input.rendered_path = "\\Users\\ana\\Documents".to_owned();
    let resolved = resolve_file_path_presentation_entry(input).unwrap();
    assert!(!resolved.path_matches_host);
    assert_eq!(
        resolved.degrade_reason,
        Some(M5FilePathPresentationEntryDegradeReason::PathOrRevealMislabeledForHost)
    );

    // A macOS entry rendered with the Windows reveal verb is mislabeled for its host.
    let mut input = clean_path_input();
    input.reveal_verb = "Show in Explorer".to_owned();
    assert_eq!(
        resolve_file_path_presentation_entry(input)
            .unwrap()
            .degrade_reason,
        Some(M5FilePathPresentationEntryDegradeReason::PathOrRevealMislabeledForHost)
    );

    let mut input = clean_path_input();
    input.preserves_canonical_path_truth = false;
    assert_eq!(
        resolve_file_path_presentation_entry(input)
            .unwrap()
            .degrade_reason,
        Some(M5FilePathPresentationEntryDegradeReason::CanonicalPathTruthNotPreserved)
    );

    let mut input = clean_path_input();
    input.presentation_form_coverage = vec![M5PathPresentationForm::HostStyledDisplay];
    assert_eq!(
        resolve_file_path_presentation_entry(input)
            .unwrap()
            .degrade_reason,
        Some(M5FilePathPresentationEntryDegradeReason::PresentationFormCoverageIncomplete)
    );
}

#[test]
fn path_reveal_unavailable_and_surface_and_proof_degrade() {
    let mut input = clean_path_input();
    input.reveal_target_unavailable = true;
    input.fallback_explained = false;
    assert_eq!(
        resolve_file_path_presentation_entry(input)
            .unwrap()
            .degrade_reason,
        Some(M5FilePathPresentationEntryDegradeReason::RevealUnavailableWithoutFallback)
    );

    let mut input = clean_path_input();
    input.surface_context = M5FilePathSurfaceContext::ContextUnknown;
    assert_eq!(
        resolve_file_path_presentation_entry(input)
            .unwrap()
            .degrade_reason,
        Some(M5FilePathPresentationEntryDegradeReason::SurfaceContextUnresolved)
    );

    let mut input = clean_path_input();
    input.proof_fresh = false;
    assert_eq!(
        resolve_file_path_presentation_entry(input)
            .unwrap()
            .degrade_reason,
        Some(M5FilePathPresentationEntryDegradeReason::ProofStale)
    );
}

#[test]
fn path_empty_id_and_forbidden_material_error() {
    let mut input = clean_path_input();
    input.entry_id = "".to_owned();
    assert_eq!(
        resolve_file_path_presentation_entry(input).unwrap_err(),
        M5FilePathRevealResolutionError::EmptyFilePathPresentationEntryId
    );

    let mut input = clean_path_input();
    input.token_name = "https://relay.internal/leak".to_owned();
    assert_eq!(
        resolve_file_path_presentation_entry(input).unwrap_err(),
        M5FilePathRevealResolutionError::ForbiddenMaterial
    );
}

#[test]
fn path_presentation_matches_host_rejects_cross_platform_paths() {
    assert!(path_presentation_matches_host(
        M5HostPlatform::Macos,
        "/Users/ana",
        "Reveal in Finder"
    ));
    assert!(!path_presentation_matches_host(
        M5HostPlatform::Macos,
        "C:\\Users\\ana",
        "Reveal in Finder"
    ));
    assert!(path_presentation_matches_host(
        M5HostPlatform::Windows,
        "C:\\Users\\ana",
        "Show in Explorer"
    ));
    assert!(!path_presentation_matches_host(
        M5HostPlatform::Windows,
        "C:/Users/ana",
        "Show in Explorer"
    ));
    assert!(path_presentation_matches_host(
        M5HostPlatform::Linux,
        "/home/ana",
        "Open Containing Folder"
    ));
    // Right separator but wrong reveal verb for the host still mismatches.
    assert!(!path_presentation_matches_host(
        M5HostPlatform::Linux,
        "/home/ana",
        "Reveal in Finder"
    ));
    assert!(!path_presentation_matches_host(
        M5HostPlatform::PlatformUnknown,
        "/Users/ana",
        "Reveal in Finder"
    ));
    assert!(!path_presentation_matches_host(
        M5HostPlatform::Macos,
        "   ",
        "Reveal in Finder"
    ));
}

#[test]
fn action_clean_stays_reachable() {
    let resolved = resolve_window_menu_action_entry(clean_action_input()).unwrap();
    assert!(resolved.is_clean());
    assert!(resolved.action_reachable_on_every_profile);
    assert!(resolved.covers_all_presentation_forms);
    assert!(resolved.provides_complete_reachability_triple);
    assert!(!resolved.action_role_invented);
    assert_eq!(resolved.action_surface, "command_palette");
    assert_eq!(resolved.human_label, "Reveal in Finder");
    assert_eq!(resolved.surface_context, "reveal_menu");
}

#[test]
fn action_reachable_only_in_os_chrome_and_unclassified_degrade() {
    let mut input = clean_action_input();
    input.reachable_by_id_surface_and_command = false;
    let resolved = resolve_window_menu_action_entry(input).unwrap();
    assert!(!resolved.provides_complete_reachability_triple);
    assert_eq!(
        resolved.degrade_reason,
        Some(M5WindowMenuActionEntryDegradeReason::ReachableOnlyInOsChrome)
    );

    let mut input = clean_action_input();
    input.in_product_route = "   ".to_owned();
    assert_eq!(
        resolve_window_menu_action_entry(input)
            .unwrap()
            .degrade_reason,
        Some(M5WindowMenuActionEntryDegradeReason::ReachableOnlyInOsChrome)
    );

    let mut input = clean_action_input();
    input.action_role = M5PlatformConventionRole::InventedPrivateConventionDisallowed;
    assert_eq!(
        resolve_window_menu_action_entry(input)
            .unwrap()
            .degrade_reason,
        Some(M5WindowMenuActionEntryDegradeReason::ReachableOnlyInOsChrome)
    );

    let mut input = clean_action_input();
    input.action_surface = M5ProductActionSurface::SurfaceUnclassified;
    assert_eq!(
        resolve_window_menu_action_entry(input)
            .unwrap()
            .degrade_reason,
        Some(M5WindowMenuActionEntryDegradeReason::ActionSurfaceUnclassified)
    );
}

#[test]
fn action_form_and_surface_and_id_and_material() {
    let mut input = clean_action_input();
    input.presentation_form_coverage = vec![M5PathPresentationForm::HostStyledDisplay];
    assert_eq!(
        resolve_window_menu_action_entry(input)
            .unwrap()
            .degrade_reason,
        Some(M5WindowMenuActionEntryDegradeReason::WindowMenuPhrasingCoverageIncomplete)
    );

    let mut input = clean_action_input();
    input.surface_context = M5FilePathSurfaceContext::ContextUnknown;
    assert_eq!(
        resolve_window_menu_action_entry(input)
            .unwrap()
            .degrade_reason,
        Some(M5WindowMenuActionEntryDegradeReason::SurfaceContextUnresolved)
    );

    let mut input = clean_action_input();
    input.entry_id = "  ".to_owned();
    assert_eq!(
        resolve_window_menu_action_entry(input).unwrap_err(),
        M5FilePathRevealResolutionError::EmptyWindowMenuActionEntryId
    );

    let mut input = clean_action_input();
    input.human_label = "see internal://notes".to_owned();
    assert_eq!(
        resolve_window_menu_action_entry(input).unwrap_err(),
        M5FilePathRevealResolutionError::ForbiddenMaterial
    );
}

#[test]
fn vocabulary_set_is_canonical() {
    assert!(
        seeded_m5_file_path_reveal_and_native_window_menu_registries()
            .vocabulary_set
            .matches_canonical()
    );
}

#[test]
fn vocabulary_set_drift_fails() {
    let mut packet = seeded_m5_file_path_reveal_and_native_window_menu_registries();
    packet.vocabulary_set.host_platforms.pop();
    assert!(packet
        .validate()
        .contains(&M5FilePathRevealRegistriesViolation::VocabularySetDrift));
}

#[test]
fn missing_source_contracts_fails() {
    let mut packet = seeded_m5_file_path_reveal_and_native_window_menu_registries();
    packet.source_contract_refs.clear();
    assert!(packet
        .validate()
        .contains(&M5FilePathRevealRegistriesViolation::MissingSourceContracts));
}

#[test]
fn domain_schema_ref_missing_fails() {
    let mut packet = seeded_m5_file_path_reveal_and_native_window_menu_registries();
    packet.registry_rows[0]
        .source_contract_refs
        .retain(|r| r != M5_FILE_PATH_AND_REVEAL_SCHEMA_REF);
    assert!(packet
        .validate()
        .contains(&M5FilePathRevealRegistriesViolation::DomainSchemaRefMissing));
}

#[test]
fn mandatory_anatomy_missing_fails() {
    let mut packet = seeded_m5_file_path_reveal_and_native_window_menu_registries();
    packet.registry_rows[0]
        .anatomy_parts
        .retain(|p| *p != M5FilePathRegistryAnatomyPart::Identity);
    assert!(packet
        .validate()
        .contains(&M5FilePathRevealRegistriesViolation::MandatoryAnatomyMissing));
}

#[test]
fn mandatory_export_field_missing_fails() {
    let mut packet = seeded_m5_file_path_reveal_and_native_window_menu_registries();
    packet.registry_rows[0]
        .export_fields
        .retain(|f| *f != M5FilePathRegistryExportField::HostPlatforms);
    assert!(packet
        .validate()
        .contains(&M5FilePathRevealRegistriesViolation::MandatoryExportFieldMissing));
}

#[test]
fn examples_missing_fails() {
    let mut packet = seeded_m5_file_path_reveal_and_native_window_menu_registries();
    packet.registry_rows[0].window_menu_action_entries.clear();
    assert!(packet
        .validate()
        .contains(&M5FilePathRevealRegistriesViolation::ExamplesMissing));
}

#[test]
fn dishonest_clean_example_fails() {
    let mut packet = seeded_m5_file_path_reveal_and_native_window_menu_registries();
    // Force a clean file-path entry to also read as mislabeled — the packet must reject it.
    let row = &mut packet.registry_rows[0];
    row.file_path_presentation_entries[0].degrade_reason = None;
    row.file_path_presentation_entries[0].path_matches_host = false;
    assert!(packet
        .validate()
        .contains(&M5FilePathRevealRegistriesViolation::DishonestExample));
}

#[test]
fn row_invariant_violation_fails() {
    for mutate in 0u8..4 {
        let mut packet = seeded_m5_file_path_reveal_and_native_window_menu_registries();
        let row = &mut packet.registry_rows[0];
        match mutate {
            0 => row.path_terminology_changes_command_or_permission_meaning = true,
            1 => row.primary_action_reachable_only_in_os_chrome = true,
            2 => row.terminology_hardcoded_instead_of_registry = true,
            _ => row.screenshot_or_docs_mislabels_path_verb = true,
        }
        assert!(packet
            .validate()
            .contains(&M5FilePathRevealRegistriesViolation::RowInvariantViolated));
    }
}

#[test]
fn host_correct_terms_not_proven_when_hand_copied_example_removed() {
    let mut packet = seeded_m5_file_path_reveal_and_native_window_menu_registries();
    for row in &mut packet.registry_rows {
        row.file_path_presentation_entries.retain(|ex| {
            ex.degrade_reason
                != Some(M5FilePathPresentationEntryDegradeReason::TerminologyNotBoundToRegistry)
        });
    }
    assert!(packet
        .validate()
        .contains(&M5FilePathRevealRegistriesViolation::HostCorrectTermsAcrossSurfacesNotProven));
}

#[test]
fn host_correct_terms_not_proven_when_surface_collapses() {
    let mut packet = seeded_m5_file_path_reveal_and_native_window_menu_registries();
    // Drop every clean breadcrumb path so the first-consumer surfaces no longer include it.
    for row in &mut packet.registry_rows {
        row.file_path_presentation_entries
            .retain(|ex| !(ex.is_clean() && ex.surface_context == "path_breadcrumb"));
    }
    assert!(packet
        .validate()
        .contains(&M5FilePathRevealRegistriesViolation::HostCorrectTermsAcrossSurfacesNotProven));
}

#[test]
fn action_reachable_not_proven_when_os_chrome_example_removed() {
    let mut packet = seeded_m5_file_path_reveal_and_native_window_menu_registries();
    for row in &mut packet.registry_rows {
        row.window_menu_action_entries.retain(|ex| {
            ex.degrade_reason != Some(M5WindowMenuActionEntryDegradeReason::ReachableOnlyInOsChrome)
        });
    }
    assert!(packet.validate().contains(
        &M5FilePathRevealRegistriesViolation::HostCorrectActionReachableOnEveryProfileNotProven
    ));
}

#[test]
fn action_reachable_not_proven_when_action_surface_dropped() {
    let mut packet = seeded_m5_file_path_reveal_and_native_window_menu_registries();
    // Drop every clean command-list action so the coverage no longer includes it.
    for row in &mut packet.registry_rows {
        row.window_menu_action_entries
            .retain(|ex| !(ex.is_clean() && ex.action_surface == "command_list"));
    }
    assert!(packet.validate().contains(
        &M5FilePathRevealRegistriesViolation::HostCorrectActionReachableOnEveryProfileNotProven
    ));
}

#[test]
fn wrong_path_verb_not_proven_when_mislabeled_example_removed() {
    let mut packet = seeded_m5_file_path_reveal_and_native_window_menu_registries();
    for row in &mut packet.registry_rows {
        row.file_path_presentation_entries.retain(|ex| {
            ex.degrade_reason
                != Some(M5FilePathPresentationEntryDegradeReason::PathOrRevealMislabeledForHost)
        });
    }
    assert!(packet
        .validate()
        .contains(&M5FilePathRevealRegistriesViolation::WrongPathVerbOrChromeDetectableNotProven));
}

#[test]
fn governance_review_incomplete_fails() {
    let mut packet = seeded_m5_file_path_reveal_and_native_window_menu_registries();
    packet
        .governance_review
        .literal_versus_canonical_path_truth_kept_explicit = false;
    assert!(packet
        .validate()
        .contains(&M5FilePathRevealRegistriesViolation::GovernanceReviewIncomplete));
}

#[test]
fn consumer_projection_incomplete_fails() {
    let mut packet = seeded_m5_file_path_reveal_and_native_window_menu_registries();
    packet
        .consumer_projection
        .support_export_reads_single_registry_source = false;
    assert!(packet
        .validate()
        .contains(&M5FilePathRevealRegistriesViolation::ConsumerProjectionIncomplete));
}

#[test]
fn proof_freshness_incomplete_fails() {
    let mut packet = seeded_m5_file_path_reveal_and_native_window_menu_registries();
    packet.proof_freshness.proof_freshness_slo_hours = 0;
    assert!(packet
        .validate()
        .contains(&M5FilePathRevealRegistriesViolation::ProofFreshnessIncomplete));
}

#[test]
fn release_posture_incomplete_fails() {
    let mut packet = seeded_m5_file_path_reveal_and_native_window_menu_registries();
    packet.release_posture.accessibility_parity_required = false;
    assert!(packet
        .validate()
        .contains(&M5FilePathRevealRegistriesViolation::ReleasePostureIncomplete));
}

#[test]
fn injected_raw_material_is_rejected() {
    let mut packet = seeded_m5_file_path_reveal_and_native_window_menu_registries();
    packet.registry_rows[0].scope_summary =
        "raw endpoint https://relay.internal.example/session leaked".to_owned();
    assert!(packet
        .validate()
        .contains(&M5FilePathRevealRegistriesViolation::RawMaterialInExport));
}

#[test]
fn export_carries_no_forbidden_raw_material() {
    let json = seeded_m5_file_path_reveal_and_native_window_menu_registries().export_safe_json();
    let lower = json.to_lowercase();
    assert!(!lower.contains("password"));
    assert!(!lower.contains("passphrase"));
    assert!(!lower.contains("bearer "));
    assert!(!lower.contains("://"));
    assert!(!lower.contains("-----begin"));
}

#[test]
fn csv_has_a_row_per_consumer_surface() {
    let packet = seeded_m5_file_path_reveal_and_native_window_menu_registries();
    let csv = packet.render_matrix_csv();
    let lines: Vec<&str> = csv.lines().collect();
    assert_eq!(lines.len(), 1 + packet.registry_rows.len());
    assert!(lines[0].starts_with("consumer_surface,qualification,owner,"));
}

#[test]
fn markdown_summary_lists_every_consumer_surface() {
    let packet = seeded_m5_file_path_reveal_and_native_window_menu_registries();
    let summary = packet.render_markdown_summary();
    for row in &packet.registry_rows {
        assert!(summary.contains(row.consumer_surface.as_str()));
    }
}

#[test]
fn path_reveal_table_lists_only_clean_paths() {
    let packet = seeded_m5_file_path_reveal_and_native_window_menu_registries();
    let table = packet.render_platform_path_reveal_table();
    // The clean macOS open path is rendered from the registry.
    assert!(table.contains("`/Users/ana/Documents`"));
    assert!(table.contains("Reveal in Finder"));
    // The clean Windows save path is rendered too.
    assert!(table.contains("Show in Explorer"));
    // A degraded, mislabeled entry never leaks into the generated path table.
    assert!(!table.contains("mislabeled"));
}

#[test]
fn checked_support_export_validates_and_matches_seed() {
    let from_disk = current_stable_m5_file_path_reveal_and_native_window_menu_registries_export()
        .expect("checked M5 file-path-reveal / window-menu registries export validates");
    assert_eq!(
        from_disk.packet_id,
        M5_FILE_PATH_REVEAL_REGISTRIES_PACKET_ID
    );
    assert_eq!(
        from_disk,
        seeded_m5_file_path_reveal_and_native_window_menu_registries(),
        "checked support export drifted from the seed builder"
    );
}

#[test]
fn narrowed_variants_validate_and_keep_rows_visible() {
    let beta =
        seeded_m5_file_path_reveal_and_native_window_menu_registries_docs_help_beta_narrowed();
    assert!(beta.validate().is_empty(), "{:?}", beta.validate());
    assert_eq!(beta.registry_rows.len(), 6);
    let row = beta
        .registry_rows
        .iter()
        .find(|r| r.consumer_surface == M5PlatformFitConsumerSurface::DocsHelp)
        .unwrap();
    assert_eq!(row.qualification, M5PlatformFitQualificationClass::Beta);

    let preview =
        seeded_m5_file_path_reveal_and_native_window_menu_registries_reveal_preview_narrowed();
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
    let beta: M5FilePathRevealRegistriesPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/platform/m5-file-path-reveal-and-native-window-menu-registries/docs_help_beta_narrowed.json"
    )))
    .expect("docs-help fixture parses");
    assert!(beta.validate().is_empty());
    assert_eq!(
        beta,
        seeded_m5_file_path_reveal_and_native_window_menu_registries_docs_help_beta_narrowed()
    );

    let preview: M5FilePathRevealRegistriesPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/platform/m5-file-path-reveal-and-native-window-menu-registries/reveal_preview_narrowed.json"
    )))
    .expect("reveal fixture parses");
    assert!(preview.validate().is_empty());
    assert_eq!(
        preview,
        seeded_m5_file_path_reveal_and_native_window_menu_registries_reveal_preview_narrowed()
    );
}

#[test]
fn implemented_families_is_file_path_reveal_and_platform_convention() {
    assert_eq!(
        IMPLEMENTED_FAMILIES,
        [
            M5PlatformFitFamily::FilePathReveal,
            M5PlatformFitFamily::PlatformConvention
        ]
    );
}
