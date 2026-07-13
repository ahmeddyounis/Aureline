use super::*;

fn clean_response_input() -> M5AppearanceLiveApplyEntryResolutionInput {
    M5AppearanceLiveApplyEntryResolutionInput {
        entry_id: "response:test".to_owned(),
        command_id: "command.appearance.apply".to_owned(),
        token_name: "appearance.theme.live".to_owned(),
        semantic_role: M5PlatformFitRole::Appearance,
        appearance_role: M5ThemeContrastLiveChangeRole::LiveThemeResponse,
        posture: M5AppearancePosture::LiveApply,
        surface_context: M5AppearanceSurfaceContext::ShellChrome,
        response_form_coverage: M5AppearanceResponseForm::ALL.to_vec(),
        applied_appearance_summary: "dark theme, accent blue".to_owned(),
        posture_label: "applies live".to_owned(),
        bound_to_registry: true,
        preserves_active_context_continuity: true,
        live_reapplied: true,
        fallback_explained: true,
        proof_fresh: true,
    }
}

fn clean_provenance_input() -> M5AppearanceSourceProvenanceEntryResolutionInput {
    M5AppearanceSourceProvenanceEntryResolutionInput {
        entry_id: "provenance:test".to_owned(),
        command_id: "command.appearance.source".to_owned(),
        token_name: "provenance.settings.source".to_owned(),
        provenance_role: M5ThemeContrastLiveChangeRole::BoundToAppearanceRegistry,
        semantic_role: M5PlatformFitRole::Appearance,
        record_surface: M5AppearanceRecordSurface::Settings,
        surface_context: M5AppearanceSurfaceContext::SettingsPreview,
        response_form_coverage: M5AppearanceResponseForm::ALL.to_vec(),
        source_signal_label: "system appearance".to_owned(),
        record_route: "settings.appearance.source".to_owned(),
        posture_recorded: true,
        proof_fresh: true,
    }
}

#[test]
fn seeded_registries_validates() {
    let packet = seeded_m5_system_appearance_live_apply_and_source_provenance_registries();
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
    assert_eq!(packet.packet_id, M5_SYSTEM_APPEARANCE_REGISTRIES_PACKET_ID);
}

#[test]
fn response_clean_names_meaning_and_is_bound() {
    let resolved = resolve_appearance_live_apply_entry(clean_response_input()).unwrap();
    assert!(resolved.is_clean());
    assert!(resolved.appearance_response_honest_across_surfaces_and_channels);
    assert!(resolved.covers_all_response_forms);
    assert!(resolved.posture_matches_support);
    assert!(resolved.bound_to_registry);
    assert!(resolved.posture_is_classified);
    assert!(resolved.posture_applies_live);
    assert!(!resolved.appearance_role_silent_drift);
    assert!(resolved.preserves_active_context_continuity);
    assert_eq!(resolved.semantic_role, "appearance");
    assert_eq!(resolved.posture, "live_apply");
    assert_eq!(resolved.canonical_posture_label, "applies live");
    assert_eq!(resolved.surface_context, "shell_chrome");
    assert_eq!(
        resolved.next_action,
        M5AppearanceRegistryNextAction::ExpandResponseMeaning
    );
}

#[test]
fn response_token_unstated_degrades() {
    let mut input = clean_response_input();
    input.token_name = "   ".to_owned();
    assert_eq!(
        resolve_appearance_live_apply_entry(input)
            .unwrap()
            .degrade_reason,
        Some(M5AppearanceLiveApplyEntryDegradeReason::AppearanceTokenUnstated)
    );
}

#[test]
fn response_hand_copied_and_unclassified_degrade() {
    let mut input = clean_response_input();
    input.appearance_role = M5ThemeContrastLiveChangeRole::SilentThemeDriftDisallowed;
    let resolved = resolve_appearance_live_apply_entry(input).unwrap();
    assert!(resolved.appearance_role_silent_drift);
    assert_eq!(
        resolved.degrade_reason,
        Some(M5AppearanceLiveApplyEntryDegradeReason::ResponseNotBoundToRegistry)
    );

    let mut input = clean_response_input();
    input.bound_to_registry = false;
    assert_eq!(
        resolve_appearance_live_apply_entry(input)
            .unwrap()
            .degrade_reason,
        Some(M5AppearanceLiveApplyEntryDegradeReason::ResponseNotBoundToRegistry)
    );

    let mut input = clean_response_input();
    input.posture = M5AppearancePosture::PostureUnclassified;
    assert_eq!(
        resolve_appearance_live_apply_entry(input)
            .unwrap()
            .degrade_reason,
        Some(M5AppearanceLiveApplyEntryDegradeReason::AppearancePostureUnclassified)
    );
}

#[test]
fn response_mislabeled_and_continuity_and_form_degrade() {
    // A live-apply entry that did not reapply live is mislabeled for its posture.
    let mut input = clean_response_input();
    input.live_reapplied = false;
    let resolved = resolve_appearance_live_apply_entry(input).unwrap();
    assert!(!resolved.posture_matches_support);
    assert_eq!(
        resolved.degrade_reason,
        Some(M5AppearanceLiveApplyEntryDegradeReason::PostureMislabeledForSupport)
    );

    // A live-apply entry rendered with the restart-required label is mislabeled for its posture.
    let mut input = clean_response_input();
    input.posture_label = "restart required".to_owned();
    assert_eq!(
        resolve_appearance_live_apply_entry(input)
            .unwrap()
            .degrade_reason,
        Some(M5AppearanceLiveApplyEntryDegradeReason::PostureMislabeledForSupport)
    );

    let mut input = clean_response_input();
    input.preserves_active_context_continuity = false;
    assert_eq!(
        resolve_appearance_live_apply_entry(input)
            .unwrap()
            .degrade_reason,
        Some(M5AppearanceLiveApplyEntryDegradeReason::ActiveContextContinuityNotPreserved)
    );

    let mut input = clean_response_input();
    input.response_form_coverage = vec![M5AppearanceResponseForm::AppliedVisualReapply];
    assert_eq!(
        resolve_appearance_live_apply_entry(input)
            .unwrap()
            .degrade_reason,
        Some(M5AppearanceLiveApplyEntryDegradeReason::ResponseFormCoverageIncomplete)
    );
}

#[test]
fn response_narrower_and_surface_and_proof_degrade() {
    let mut input = clean_response_input();
    input.posture = M5AppearancePosture::RestartRequired;
    input.posture_label = "restart required".to_owned();
    input.live_reapplied = false;
    input.fallback_explained = false;
    assert_eq!(
        resolve_appearance_live_apply_entry(input)
            .unwrap()
            .degrade_reason,
        Some(M5AppearanceLiveApplyEntryDegradeReason::NarrowerBehaviorNotExplained)
    );

    let mut input = clean_response_input();
    input.surface_context = M5AppearanceSurfaceContext::ContextUnknown;
    assert_eq!(
        resolve_appearance_live_apply_entry(input)
            .unwrap()
            .degrade_reason,
        Some(M5AppearanceLiveApplyEntryDegradeReason::SurfaceContextUnresolved)
    );

    let mut input = clean_response_input();
    input.proof_fresh = false;
    assert_eq!(
        resolve_appearance_live_apply_entry(input)
            .unwrap()
            .degrade_reason,
        Some(M5AppearanceLiveApplyEntryDegradeReason::ProofStale)
    );
}

#[test]
fn restart_required_with_explained_fallback_is_clean() {
    let mut input = clean_response_input();
    input.posture = M5AppearancePosture::RestartRequired;
    input.posture_label = "restart required".to_owned();
    input.live_reapplied = false;
    input.fallback_explained = true;
    let resolved = resolve_appearance_live_apply_entry(input).unwrap();
    assert!(resolved.is_clean());
    assert!(!resolved.posture_applies_live);
    assert!(resolved.posture_matches_support);
}

#[test]
fn response_empty_id_and_forbidden_material_error() {
    let mut input = clean_response_input();
    input.entry_id = "".to_owned();
    assert_eq!(
        resolve_appearance_live_apply_entry(input).unwrap_err(),
        M5SystemAppearanceResolutionError::EmptyAppearanceLiveApplyEntryId
    );

    let mut input = clean_response_input();
    input.token_name = "https://relay.internal/leak".to_owned();
    assert_eq!(
        resolve_appearance_live_apply_entry(input).unwrap_err(),
        M5SystemAppearanceResolutionError::ForbiddenMaterial
    );
}

#[test]
fn appearance_response_matches_posture_rejects_cross_posture_states() {
    assert!(appearance_response_matches_posture(
        M5AppearancePosture::LiveApply,
        true,
        "applies live"
    ));
    assert!(!appearance_response_matches_posture(
        M5AppearancePosture::LiveApply,
        false,
        "applies live"
    ));
    assert!(appearance_response_matches_posture(
        M5AppearancePosture::RestartRequired,
        false,
        "restart required"
    ));
    assert!(!appearance_response_matches_posture(
        M5AppearancePosture::RestartRequired,
        true,
        "restart required"
    ));
    assert!(appearance_response_matches_posture(
        M5AppearancePosture::Unsupported,
        false,
        "not supported on this host"
    ));
    // Right live state but wrong label for the posture still mismatches.
    assert!(!appearance_response_matches_posture(
        M5AppearancePosture::Unsupported,
        false,
        "applies live"
    ));
    assert!(!appearance_response_matches_posture(
        M5AppearancePosture::PostureUnclassified,
        true,
        "applies live"
    ));
    assert!(!appearance_response_matches_posture(
        M5AppearancePosture::LiveApply,
        true,
        "   "
    ));
}

#[test]
fn provenance_clean_stays_recorded() {
    let resolved = resolve_appearance_source_provenance_entry(clean_provenance_input()).unwrap();
    assert!(resolved.is_clean());
    assert!(resolved.source_recorded_on_every_profile);
    assert!(resolved.covers_all_response_forms);
    assert!(resolved.provides_complete_provenance_triple);
    assert!(!resolved.provenance_role_silent_drift);
    assert_eq!(resolved.record_surface, "settings");
    assert_eq!(resolved.source_signal_label, "system appearance");
    assert_eq!(resolved.surface_context, "settings_preview");
}

#[test]
fn provenance_not_recorded_and_unclassified_degrade() {
    let mut input = clean_provenance_input();
    input.posture_recorded = false;
    let resolved = resolve_appearance_source_provenance_entry(input).unwrap();
    assert!(!resolved.provides_complete_provenance_triple);
    assert_eq!(
        resolved.degrade_reason,
        Some(M5AppearanceSourceProvenanceEntryDegradeReason::SourceOrPostureNotRecorded)
    );

    let mut input = clean_provenance_input();
    input.record_route = "   ".to_owned();
    assert_eq!(
        resolve_appearance_source_provenance_entry(input)
            .unwrap()
            .degrade_reason,
        Some(M5AppearanceSourceProvenanceEntryDegradeReason::SourceOrPostureNotRecorded)
    );

    let mut input = clean_provenance_input();
    input.provenance_role = M5ThemeContrastLiveChangeRole::SilentThemeDriftDisallowed;
    assert_eq!(
        resolve_appearance_source_provenance_entry(input)
            .unwrap()
            .degrade_reason,
        Some(M5AppearanceSourceProvenanceEntryDegradeReason::SourceOrPostureNotRecorded)
    );

    let mut input = clean_provenance_input();
    input.record_surface = M5AppearanceRecordSurface::RecordSurfaceUnclassified;
    assert_eq!(
        resolve_appearance_source_provenance_entry(input)
            .unwrap()
            .degrade_reason,
        Some(M5AppearanceSourceProvenanceEntryDegradeReason::RecordSurfaceUnclassified)
    );
}

#[test]
fn provenance_form_and_surface_and_id_and_material() {
    let mut input = clean_provenance_input();
    input.response_form_coverage = vec![M5AppearanceResponseForm::AppliedVisualReapply];
    assert_eq!(
        resolve_appearance_source_provenance_entry(input)
            .unwrap()
            .degrade_reason,
        Some(M5AppearanceSourceProvenanceEntryDegradeReason::ProvenancePhrasingCoverageIncomplete)
    );

    let mut input = clean_provenance_input();
    input.surface_context = M5AppearanceSurfaceContext::ContextUnknown;
    assert_eq!(
        resolve_appearance_source_provenance_entry(input)
            .unwrap()
            .degrade_reason,
        Some(M5AppearanceSourceProvenanceEntryDegradeReason::SurfaceContextUnresolved)
    );

    let mut input = clean_provenance_input();
    input.entry_id = "  ".to_owned();
    assert_eq!(
        resolve_appearance_source_provenance_entry(input).unwrap_err(),
        M5SystemAppearanceResolutionError::EmptyAppearanceSourceProvenanceEntryId
    );

    let mut input = clean_provenance_input();
    input.source_signal_label = "see internal://notes".to_owned();
    assert_eq!(
        resolve_appearance_source_provenance_entry(input).unwrap_err(),
        M5SystemAppearanceResolutionError::ForbiddenMaterial
    );
}

#[test]
fn vocabulary_set_is_canonical() {
    assert!(
        seeded_m5_system_appearance_live_apply_and_source_provenance_registries()
            .vocabulary_set
            .matches_canonical()
    );
}

#[test]
fn vocabulary_set_drift_fails() {
    let mut packet = seeded_m5_system_appearance_live_apply_and_source_provenance_registries();
    packet.vocabulary_set.postures.pop();
    assert!(packet
        .validate()
        .contains(&M5SystemAppearanceRegistriesViolation::VocabularySetDrift));
}

#[test]
fn missing_source_contracts_fails() {
    let mut packet = seeded_m5_system_appearance_live_apply_and_source_provenance_registries();
    packet.source_contract_refs.clear();
    assert!(packet
        .validate()
        .contains(&M5SystemAppearanceRegistriesViolation::MissingSourceContracts));
}

#[test]
fn domain_schema_ref_missing_fails() {
    let mut packet = seeded_m5_system_appearance_live_apply_and_source_provenance_registries();
    packet.registry_rows[0]
        .source_contract_refs
        .retain(|r| r != M5_FILE_PATH_AND_REVEAL_SCHEMA_REF);
    assert!(packet
        .validate()
        .contains(&M5SystemAppearanceRegistriesViolation::DomainSchemaRefMissing));
}

#[test]
fn mandatory_anatomy_missing_fails() {
    let mut packet = seeded_m5_system_appearance_live_apply_and_source_provenance_registries();
    packet.registry_rows[0]
        .anatomy_parts
        .retain(|p| *p != M5AppearanceRegistryAnatomyPart::Identity);
    assert!(packet
        .validate()
        .contains(&M5SystemAppearanceRegistriesViolation::MandatoryAnatomyMissing));
}

#[test]
fn mandatory_export_field_missing_fails() {
    let mut packet = seeded_m5_system_appearance_live_apply_and_source_provenance_registries();
    packet.registry_rows[0]
        .export_fields
        .retain(|f| *f != M5AppearanceRegistryExportField::SupportPostures);
    assert!(packet
        .validate()
        .contains(&M5SystemAppearanceRegistriesViolation::MandatoryExportFieldMissing));
}

#[test]
fn examples_missing_fails() {
    let mut packet = seeded_m5_system_appearance_live_apply_and_source_provenance_registries();
    packet.registry_rows[0]
        .appearance_source_provenance_entries
        .clear();
    assert!(packet
        .validate()
        .contains(&M5SystemAppearanceRegistriesViolation::ExamplesMissing));
}

#[test]
fn dishonest_clean_example_fails() {
    let mut packet = seeded_m5_system_appearance_live_apply_and_source_provenance_registries();
    // Force a clean response entry to also read as mislabeled — the packet must reject it.
    let row = &mut packet.registry_rows[0];
    row.appearance_live_apply_entries[0].degrade_reason = None;
    row.appearance_live_apply_entries[0].posture_matches_support = false;
    assert!(packet
        .validate()
        .contains(&M5SystemAppearanceRegistriesViolation::DishonestExample));
}

#[test]
fn row_invariant_violation_fails() {
    for mutate in 0u8..4 {
        let mut packet = seeded_m5_system_appearance_live_apply_and_source_provenance_registries();
        let row = &mut packet.registry_rows[0];
        match mutate {
            0 => row.appearance_change_corrupts_focus_layout_or_meaning_on_protected_path = true,
            1 => row.live_change_forces_mystery_repaint_or_resets_context = true,
            2 => row.appearance_response_hardcoded_instead_of_registry = true,
            _ => row.diagnostics_or_export_cannot_distinguish_live_from_restart = true,
        }
        assert!(packet
            .validate()
            .contains(&M5SystemAppearanceRegistriesViolation::RowInvariantViolated));
    }
}

#[test]
fn live_or_explained_not_proven_when_hand_copied_example_removed() {
    let mut packet = seeded_m5_system_appearance_live_apply_and_source_provenance_registries();
    for row in &mut packet.registry_rows {
        row.appearance_live_apply_entries.retain(|ex| {
            ex.degrade_reason
                != Some(M5AppearanceLiveApplyEntryDegradeReason::ResponseNotBoundToRegistry)
        });
    }
    assert!(packet
        .validate()
        .contains(&M5SystemAppearanceRegistriesViolation::LiveOrExplainedAcrossSurfacesNotProven));
}

#[test]
fn live_or_explained_not_proven_when_surface_collapses() {
    let mut packet = seeded_m5_system_appearance_live_apply_and_source_provenance_registries();
    // Drop every clean open-dialog response so the first-consumer surfaces no longer include it.
    for row in &mut packet.registry_rows {
        row.appearance_live_apply_entries
            .retain(|ex| !(ex.is_clean() && ex.surface_context == "open_dialog"));
    }
    assert!(packet
        .validate()
        .contains(&M5SystemAppearanceRegistriesViolation::LiveOrExplainedAcrossSurfacesNotProven));
}

#[test]
fn source_recorded_not_proven_when_not_recorded_example_removed() {
    let mut packet = seeded_m5_system_appearance_live_apply_and_source_provenance_registries();
    for row in &mut packet.registry_rows {
        row.appearance_source_provenance_entries.retain(|ex| {
            ex.degrade_reason
                != Some(M5AppearanceSourceProvenanceEntryDegradeReason::SourceOrPostureNotRecorded)
        });
    }
    assert!(packet.validate().contains(
        &M5SystemAppearanceRegistriesViolation::AppearanceSourceRecordedOnEveryProfileNotProven
    ));
}

#[test]
fn source_recorded_not_proven_when_record_surface_dropped() {
    let mut packet = seeded_m5_system_appearance_live_apply_and_source_provenance_registries();
    // Drop every clean support-export provenance so the coverage no longer includes it.
    for row in &mut packet.registry_rows {
        row.appearance_source_provenance_entries
            .retain(|ex| !(ex.is_clean() && ex.record_surface == "support_export"));
    }
    assert!(packet.validate().contains(
        &M5SystemAppearanceRegistriesViolation::AppearanceSourceRecordedOnEveryProfileNotProven
    ));
}

#[test]
fn mislabeled_posture_not_proven_when_mislabeled_example_removed() {
    let mut packet = seeded_m5_system_appearance_live_apply_and_source_provenance_registries();
    for row in &mut packet.registry_rows {
        row.appearance_live_apply_entries.retain(|ex| {
            ex.degrade_reason
                != Some(M5AppearanceLiveApplyEntryDegradeReason::PostureMislabeledForSupport)
        });
    }
    assert!(packet.validate().contains(
        &M5SystemAppearanceRegistriesViolation::MislabeledPostureOrUnrecordedSourceDetectableNotProven
    ));
}

#[test]
fn governance_review_incomplete_fails() {
    let mut packet = seeded_m5_system_appearance_live_apply_and_source_provenance_registries();
    packet
        .governance_review
        .live_versus_fallback_posture_truth_kept_explicit = false;
    assert!(packet
        .validate()
        .contains(&M5SystemAppearanceRegistriesViolation::GovernanceReviewIncomplete));
}

#[test]
fn consumer_projection_incomplete_fails() {
    let mut packet = seeded_m5_system_appearance_live_apply_and_source_provenance_registries();
    packet
        .consumer_projection
        .support_export_reads_single_registry_source = false;
    assert!(packet
        .validate()
        .contains(&M5SystemAppearanceRegistriesViolation::ConsumerProjectionIncomplete));
}

#[test]
fn proof_freshness_incomplete_fails() {
    let mut packet = seeded_m5_system_appearance_live_apply_and_source_provenance_registries();
    packet.proof_freshness.proof_freshness_slo_hours = 0;
    assert!(packet
        .validate()
        .contains(&M5SystemAppearanceRegistriesViolation::ProofFreshnessIncomplete));
}

#[test]
fn release_posture_incomplete_fails() {
    let mut packet = seeded_m5_system_appearance_live_apply_and_source_provenance_registries();
    packet.release_posture.accessibility_parity_required = false;
    assert!(packet
        .validate()
        .contains(&M5SystemAppearanceRegistriesViolation::ReleasePostureIncomplete));
}

#[test]
fn injected_raw_material_is_rejected() {
    let mut packet = seeded_m5_system_appearance_live_apply_and_source_provenance_registries();
    packet.registry_rows[0].scope_summary =
        "raw endpoint https://relay.internal.example/session leaked".to_owned();
    assert!(packet
        .validate()
        .contains(&M5SystemAppearanceRegistriesViolation::RawMaterialInExport));
}

#[test]
fn export_carries_no_forbidden_raw_material() {
    let json = seeded_m5_system_appearance_live_apply_and_source_provenance_registries()
        .export_safe_json();
    let lower = json.to_lowercase();
    assert!(!lower.contains("password"));
    assert!(!lower.contains("passphrase"));
    assert!(!lower.contains("bearer "));
    assert!(!lower.contains("://"));
    assert!(!lower.contains("-----begin"));
}

#[test]
fn csv_has_a_row_per_consumer_surface() {
    let packet = seeded_m5_system_appearance_live_apply_and_source_provenance_registries();
    let csv = packet.render_matrix_csv();
    let lines: Vec<&str> = csv.lines().collect();
    assert_eq!(lines.len(), 1 + packet.registry_rows.len());
    assert!(lines[0].starts_with("consumer_surface,qualification,owner,"));
}

#[test]
fn markdown_summary_lists_every_consumer_surface() {
    let packet = seeded_m5_system_appearance_live_apply_and_source_provenance_registries();
    let summary = packet.render_markdown_summary();
    for row in &packet.registry_rows {
        assert!(summary.contains(row.consumer_surface.as_str()));
    }
}

#[test]
fn posture_table_lists_only_clean_responses() {
    let packet = seeded_m5_system_appearance_live_apply_and_source_provenance_registries();
    let table = packet.render_appearance_posture_table();
    // The clean live-apply response is rendered from the registry.
    assert!(table.contains("live_apply"));
    assert!(table.contains("applies live"));
    // The clean restart-required response is rendered too.
    assert!(table.contains("restart required"));
    // A degraded, mislabeled entry never leaks into the generated posture table.
    assert!(!table.contains("mislabeled"));
}

#[test]
fn checked_support_export_validates_and_matches_seed() {
    let from_disk =
        current_stable_m5_system_appearance_live_apply_and_source_provenance_registries_export()
            .expect("checked M5 system-appearance registries export validates");
    assert_eq!(
        from_disk.packet_id,
        M5_SYSTEM_APPEARANCE_REGISTRIES_PACKET_ID
    );
    assert_eq!(
        from_disk,
        seeded_m5_system_appearance_live_apply_and_source_provenance_registries(),
        "checked support export drifted from the seed builder"
    );
}

#[test]
fn narrowed_variants_validate_and_keep_rows_visible() {
    let beta =
        seeded_m5_system_appearance_live_apply_and_source_provenance_registries_docs_help_beta_narrowed();
    assert!(beta.validate().is_empty(), "{:?}", beta.validate());
    assert_eq!(beta.registry_rows.len(), 6);
    let row = beta
        .registry_rows
        .iter()
        .find(|r| r.consumer_surface == M5PlatformFitConsumerSurface::DocsHelp)
        .unwrap();
    assert_eq!(row.qualification, M5PlatformFitQualificationClass::Beta);

    let preview =
        seeded_m5_system_appearance_live_apply_and_source_provenance_registries_restart_posture_preview_narrowed();
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
    let beta: M5SystemAppearanceRegistriesPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/platform/m5-system-appearance-live-apply-and-source-provenance-registries/docs_help_beta_narrowed.json"
    )))
    .expect("docs-help fixture parses");
    assert!(beta.validate().is_empty());
    assert_eq!(
        beta,
        seeded_m5_system_appearance_live_apply_and_source_provenance_registries_docs_help_beta_narrowed()
    );

    let preview: M5SystemAppearanceRegistriesPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/platform/m5-system-appearance-live-apply-and-source-provenance-registries/restart_posture_preview_narrowed.json"
    )))
    .expect("restart-posture fixture parses");
    assert!(preview.validate().is_empty());
    assert_eq!(
        preview,
        seeded_m5_system_appearance_live_apply_and_source_provenance_registries_restart_posture_preview_narrowed()
    );
}

#[test]
fn implemented_families_is_theme_contrast_live_change() {
    assert_eq!(
        IMPLEMENTED_FAMILIES,
        [M5PlatformFitFamily::ThemeContrastLiveChange]
    );
}
