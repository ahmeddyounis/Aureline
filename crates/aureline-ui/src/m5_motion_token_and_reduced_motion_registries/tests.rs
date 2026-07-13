use super::*;

fn clean_motion_input() -> M5MotionEntryResolutionInput {
    M5MotionEntryResolutionInput {
        entry_id: "motion:test".to_owned(),
        token_name: "motion.instant.palette".to_owned(),
        semantic_role: M5VisualInteractionRole::Motion,
        motion_role: M5MotionTokenRole::RespectsInputPriority,
        surface_class: M5MotionSurfaceClass::CommandPaletteInput,
        reduced_motion_fallback: M5ReducedMotionFallback::InstantStateChange,
        surface_context: M5MotionSurfaceContext::Shell,
        clamp_coverage: M5MotionClamp::ALL.to_vec(),
        respects_input_priority: true,
        preserves_no_layout_shift: true,
        references_canonical_token: true,
        proof_fresh: true,
    }
}

fn clean_reduced_input() -> M5ReducedMotionEntryResolutionInput {
    M5ReducedMotionEntryResolutionInput {
        entry_id: "reduced:test".to_owned(),
        token_name: "reduced.clamp.reduced_motion".to_owned(),
        reduced_motion_role: M5ReducedMotionRole::ReducedMotionClamp,
        semantic_role: M5VisualInteractionRole::Motion,
        surface_context: M5MotionSurfaceContext::Shell,
        clamp_coverage: M5MotionClamp::ALL.to_vec(),
        references_canonical_token: true,
        static_fallback_preserves_meaning: true,
        proof_fresh: true,
    }
}

#[test]
fn seeded_registries_validates() {
    let packet = seeded_m5_motion_reduced_motion_registries();
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
    assert_eq!(packet.packet_id, M5_MOTION_REGISTRIES_PACKET_ID);
}

#[test]
fn motion_clean_names_meaning_and_is_safe() {
    let resolved = resolve_motion_entry(clean_motion_input()).unwrap();
    assert!(resolved.is_clean());
    assert!(resolved.motion_safe_on_protected_paths);
    assert!(resolved.covers_all_clamps);
    assert!(resolved.reduced_motion_fallback_present);
    assert!(resolved.surface_class_is_classified);
    assert!(resolved.surface_class_is_protected_path);
    assert!(!resolved.motion_role_delays_protected_input);
    assert_eq!(resolved.semantic_role, "motion");
    assert_eq!(resolved.surface_class, "command_palette_input");
    assert_eq!(resolved.surface_context, "shell");
    assert_eq!(
        resolved.next_action,
        M5MotionRegistryNextAction::ExpandMotionMeaning
    );
}

#[test]
fn motion_token_unstated_degrades() {
    let mut input = clean_motion_input();
    input.token_name = "   ".to_owned();
    assert_eq!(
        resolve_motion_entry(input).unwrap().degrade_reason,
        Some(M5MotionEntryDegradeReason::TokenNameUnstated)
    );
}

#[test]
fn motion_delay_and_fallback_missing_degrade() {
    let mut input = clean_motion_input();
    input.respects_input_priority = false;
    assert_eq!(
        resolve_motion_entry(input).unwrap().degrade_reason,
        Some(M5MotionEntryDegradeReason::ProtectedPathDelayedByMotion)
    );

    let mut input = clean_motion_input();
    input.motion_role = M5MotionTokenRole::MotionDelaysProtectedInputDisallowed;
    let resolved = resolve_motion_entry(input).unwrap();
    assert!(resolved.motion_role_delays_protected_input);
    assert_eq!(
        resolved.degrade_reason,
        Some(M5MotionEntryDegradeReason::ProtectedPathDelayedByMotion)
    );

    let mut input = clean_motion_input();
    input.reduced_motion_fallback = M5ReducedMotionFallback::NoneDisallowed;
    assert_eq!(
        resolve_motion_entry(input).unwrap().degrade_reason,
        Some(M5MotionEntryDegradeReason::ReducedMotionFallbackMissing)
    );
}

#[test]
fn motion_raw_inlined_and_unclassified_degrade() {
    let mut input = clean_motion_input();
    input.references_canonical_token = false;
    assert_eq!(
        resolve_motion_entry(input).unwrap().degrade_reason,
        Some(M5MotionEntryDegradeReason::RawDurationValueInlined)
    );

    let mut input = clean_motion_input();
    input.surface_class = M5MotionSurfaceClass::SurfaceClassUnclassified;
    assert_eq!(
        resolve_motion_entry(input).unwrap().degrade_reason,
        Some(M5MotionEntryDegradeReason::SurfaceClassUnclassified)
    );
}

#[test]
fn motion_clamp_and_layout_shift_degrade() {
    let mut input = clean_motion_input();
    input.clamp_coverage = vec![M5MotionClamp::ReducedMotion, M5MotionClamp::PowerSaver];
    assert_eq!(
        resolve_motion_entry(input).unwrap().degrade_reason,
        Some(M5MotionEntryDegradeReason::ClampCoverageIncomplete)
    );

    let mut input = clean_motion_input();
    input.preserves_no_layout_shift = false;
    assert_eq!(
        resolve_motion_entry(input).unwrap().degrade_reason,
        Some(M5MotionEntryDegradeReason::LayoutShiftIntroduced)
    );

    let mut input = clean_motion_input();
    input.surface_context = M5MotionSurfaceContext::ContextUnknown;
    assert_eq!(
        resolve_motion_entry(input).unwrap().degrade_reason,
        Some(M5MotionEntryDegradeReason::SurfaceContextUnresolved)
    );
}

#[test]
fn motion_empty_id_and_forbidden_material_error() {
    let mut input = clean_motion_input();
    input.entry_id = "".to_owned();
    assert_eq!(
        resolve_motion_entry(input).unwrap_err(),
        M5MotionResolutionError::EmptyMotionEntryId
    );

    let mut input = clean_motion_input();
    input.token_name = "https://relay.internal/leak".to_owned();
    assert_eq!(
        resolve_motion_entry(input).unwrap_err(),
        M5MotionResolutionError::ForbiddenMaterial
    );
}

#[test]
fn reduced_clean_stays_safe_across_clamps() {
    let resolved = resolve_reduced_motion_entry(clean_reduced_input()).unwrap();
    assert!(resolved.is_clean());
    assert!(resolved.fallback_preserves_meaning_across_clamps);
    assert!(resolved.covers_all_clamps);
    assert!(!resolved.reduced_motion_role_is_motion_only);
    assert_eq!(resolved.reduced_motion_role, "reduced_motion_clamp");
    assert_eq!(resolved.surface_context, "shell");
}

#[test]
fn reduced_motion_only_and_clamp_incomplete_degrade() {
    let mut input = clean_reduced_input();
    input.reduced_motion_role = M5ReducedMotionRole::MotionOnlyMeaningDisallowed;
    let resolved = resolve_reduced_motion_entry(input).unwrap();
    assert!(resolved.reduced_motion_role_is_motion_only);
    assert_eq!(
        resolved.degrade_reason,
        Some(M5ReducedMotionEntryDegradeReason::MotionOnlyMeaningWithoutFallback)
    );

    let mut input = clean_reduced_input();
    input.references_canonical_token = false;
    assert_eq!(
        resolve_reduced_motion_entry(input).unwrap().degrade_reason,
        Some(M5ReducedMotionEntryDegradeReason::MotionOnlyMeaningWithoutFallback)
    );

    let mut input = clean_reduced_input();
    input.clamp_coverage = vec![M5MotionClamp::ReducedMotion];
    assert_eq!(
        resolve_reduced_motion_entry(input).unwrap().degrade_reason,
        Some(M5ReducedMotionEntryDegradeReason::ClampCoverageIncomplete)
    );
}

#[test]
fn reduced_fallback_and_surface_and_id_and_material() {
    let mut input = clean_reduced_input();
    input.static_fallback_preserves_meaning = false;
    assert_eq!(
        resolve_reduced_motion_entry(input).unwrap().degrade_reason,
        Some(M5ReducedMotionEntryDegradeReason::StaticFallbackNotEquivalent)
    );

    let mut input = clean_reduced_input();
    input.surface_context = M5MotionSurfaceContext::ContextUnknown;
    assert_eq!(
        resolve_reduced_motion_entry(input).unwrap().degrade_reason,
        Some(M5ReducedMotionEntryDegradeReason::SurfaceContextUnresolved)
    );

    let mut input = clean_reduced_input();
    input.entry_id = "  ".to_owned();
    assert_eq!(
        resolve_reduced_motion_entry(input).unwrap_err(),
        M5MotionResolutionError::EmptyReducedMotionEntryId
    );

    let mut input = clean_reduced_input();
    input.token_name = "see internal://notes".to_owned();
    assert_eq!(
        resolve_reduced_motion_entry(input).unwrap_err(),
        M5MotionResolutionError::ForbiddenMaterial
    );
}

#[test]
fn vocabulary_set_is_canonical() {
    assert!(seeded_m5_motion_reduced_motion_registries()
        .vocabulary_set
        .matches_canonical());
}

#[test]
fn vocabulary_set_drift_fails() {
    let mut packet = seeded_m5_motion_reduced_motion_registries();
    packet.vocabulary_set.motion_surface_classes.pop();
    assert!(packet
        .validate()
        .contains(&M5MotionRegistriesViolation::VocabularySetDrift));
}

#[test]
fn missing_source_contracts_fails() {
    let mut packet = seeded_m5_motion_reduced_motion_registries();
    packet.source_contract_refs.clear();
    assert!(packet
        .validate()
        .contains(&M5MotionRegistriesViolation::MissingSourceContracts));
}

#[test]
fn domain_schema_ref_missing_fails() {
    let mut packet = seeded_m5_motion_reduced_motion_registries();
    packet.registry_rows[0]
        .source_contract_refs
        .retain(|r| r != M5_MOTION_AND_REDUCED_MOTION_SCHEMA_REF);
    assert!(packet
        .validate()
        .contains(&M5MotionRegistriesViolation::DomainSchemaRefMissing));
}

#[test]
fn mandatory_anatomy_missing_fails() {
    let mut packet = seeded_m5_motion_reduced_motion_registries();
    packet.registry_rows[0]
        .anatomy_parts
        .retain(|p| *p != M5MotionRegistryAnatomyPart::Identity);
    assert!(packet
        .validate()
        .contains(&M5MotionRegistriesViolation::MandatoryAnatomyMissing));
}

#[test]
fn mandatory_export_field_missing_fails() {
    let mut packet = seeded_m5_motion_reduced_motion_registries();
    packet.registry_rows[0]
        .export_fields
        .retain(|f| *f != M5MotionRegistryExportField::MotionSurfaceClasses);
    assert!(packet
        .validate()
        .contains(&M5MotionRegistriesViolation::MandatoryExportFieldMissing));
}

#[test]
fn examples_missing_fails() {
    let mut packet = seeded_m5_motion_reduced_motion_registries();
    packet.registry_rows[0].reduced_motion_entries.clear();
    assert!(packet
        .validate()
        .contains(&M5MotionRegistriesViolation::ExamplesMissing));
}

#[test]
fn dishonest_clean_example_fails() {
    let mut packet = seeded_m5_motion_reduced_motion_registries();
    // Force a clean motion entry to also read as protected-path-delaying — the packet must reject it.
    let row = &mut packet.registry_rows[0];
    row.motion_entries[0].degrade_reason = None;
    row.motion_entries[0].respects_input_priority = false;
    assert!(packet
        .validate()
        .contains(&M5MotionRegistriesViolation::DishonestExample));
}

#[test]
fn row_invariant_violation_fails() {
    for mutate in 0u8..4 {
        let mut packet = seeded_m5_motion_reduced_motion_registries();
        let row = &mut packet.registry_rows[0];
        match mutate {
            0 => row.motion_delays_protected_input = true,
            1 => row.raw_duration_value_inlined_instead_of_token = true,
            2 => row.layout_shift_on_protected_surface = true,
            _ => row.clamp_coverage_incomplete = true,
        }
        assert!(packet
            .validate()
            .contains(&M5MotionRegistriesViolation::RowInvariantViolated));
    }
}

#[test]
fn first_consumers_not_proven_when_raw_duration_example_removed() {
    let mut packet = seeded_m5_motion_reduced_motion_registries();
    for row in &mut packet.registry_rows {
        row.motion_entries.retain(|ex| {
            ex.degrade_reason != Some(M5MotionEntryDegradeReason::RawDurationValueInlined)
        });
    }
    assert!(packet
        .validate()
        .contains(&M5MotionRegistriesViolation::FirstConsumersUseCanonicalMotionGrammarNotProven));
}

#[test]
fn first_consumers_not_proven_when_semantic_family_collapses() {
    let mut packet = seeded_m5_motion_reduced_motion_registries();
    // Drop every clean attention motion entry so the semantic-role grammar no longer covers "attention".
    for row in &mut packet.registry_rows {
        row.motion_entries
            .retain(|ex| !(ex.is_clean() && ex.semantic_role == "attention"));
    }
    assert!(packet
        .validate()
        .contains(&M5MotionRegistriesViolation::FirstConsumersUseCanonicalMotionGrammarNotProven));
}

#[test]
fn protected_path_safety_not_proven_when_clamp_incomplete_example_removed() {
    let mut packet = seeded_m5_motion_reduced_motion_registries();
    for row in &mut packet.registry_rows {
        row.motion_entries.retain(|ex| {
            ex.degrade_reason != Some(M5MotionEntryDegradeReason::ClampCoverageIncomplete)
        });
    }
    assert!(packet
        .validate()
        .contains(&M5MotionRegistriesViolation::ProtectedPathSafetyAcrossClampsNotProven));
}

#[test]
fn protected_path_safety_not_proven_when_protected_class_dropped() {
    let mut packet = seeded_m5_motion_reduced_motion_registries();
    // Drop every clean typing-caret motion entry so protected coverage no longer includes it.
    for row in &mut packet.registry_rows {
        row.motion_entries
            .retain(|ex| !(ex.is_clean() && ex.surface_class == "typing_caret"));
    }
    assert!(packet
        .validate()
        .contains(&M5MotionRegistriesViolation::ProtectedPathSafetyAcrossClampsNotProven));
}

#[test]
fn animation_drift_not_proven_when_motion_only_example_removed() {
    let mut packet = seeded_m5_motion_reduced_motion_registries();
    for row in &mut packet.registry_rows {
        row.reduced_motion_entries.retain(|ex| {
            ex.degrade_reason
                != Some(M5ReducedMotionEntryDegradeReason::MotionOnlyMeaningWithoutFallback)
        });
    }
    assert!(packet
        .validate()
        .contains(&M5MotionRegistriesViolation::ProtectedPathAnimationDriftNotDetectableNotProven));
}

#[test]
fn governance_review_incomplete_fails() {
    let mut packet = seeded_m5_motion_reduced_motion_registries();
    packet.governance_review.motion_never_delays_protected_input = false;
    assert!(packet
        .validate()
        .contains(&M5MotionRegistriesViolation::GovernanceReviewIncomplete));
}

#[test]
fn consumer_projection_incomplete_fails() {
    let mut packet = seeded_m5_motion_reduced_motion_registries();
    packet
        .consumer_projection
        .support_export_reads_single_registry_source = false;
    assert!(packet
        .validate()
        .contains(&M5MotionRegistriesViolation::ConsumerProjectionIncomplete));
}

#[test]
fn proof_freshness_incomplete_fails() {
    let mut packet = seeded_m5_motion_reduced_motion_registries();
    packet.proof_freshness.proof_freshness_slo_hours = 0;
    assert!(packet
        .validate()
        .contains(&M5MotionRegistriesViolation::ProofFreshnessIncomplete));
}

#[test]
fn release_posture_incomplete_fails() {
    let mut packet = seeded_m5_motion_reduced_motion_registries();
    packet.release_posture.accessibility_parity_required = false;
    assert!(packet
        .validate()
        .contains(&M5MotionRegistriesViolation::ReleasePostureIncomplete));
}

#[test]
fn injected_raw_material_is_rejected() {
    let mut packet = seeded_m5_motion_reduced_motion_registries();
    packet.registry_rows[0].scope_summary =
        "raw endpoint https://relay.internal.example/session leaked".to_owned();
    assert!(packet
        .validate()
        .contains(&M5MotionRegistriesViolation::RawMaterialInExport));
}

#[test]
fn export_carries_no_forbidden_raw_material() {
    let json = seeded_m5_motion_reduced_motion_registries().export_safe_json();
    let lower = json.to_lowercase();
    assert!(!lower.contains("password"));
    assert!(!lower.contains("passphrase"));
    assert!(!lower.contains("bearer "));
    assert!(!lower.contains("://"));
    assert!(!lower.contains("-----begin"));
}

#[test]
fn csv_has_a_row_per_consumer_surface() {
    let packet = seeded_m5_motion_reduced_motion_registries();
    let csv = packet.render_matrix_csv();
    let lines: Vec<&str> = csv.lines().collect();
    assert_eq!(lines.len(), 1 + packet.registry_rows.len());
    assert!(lines[0].starts_with("consumer_surface,qualification,owner,"));
}

#[test]
fn markdown_summary_lists_every_consumer_surface() {
    let packet = seeded_m5_motion_reduced_motion_registries();
    let summary = packet.render_markdown_summary();
    for row in &packet.registry_rows {
        assert!(summary.contains(row.consumer_surface.as_str()));
    }
}

#[test]
fn checked_support_export_validates_and_matches_seed() {
    let from_disk = current_stable_m5_motion_reduced_motion_registries_export()
        .expect("checked M5 motion / reduced-motion registries export validates");
    assert_eq!(from_disk.packet_id, M5_MOTION_REGISTRIES_PACKET_ID);
    assert_eq!(
        from_disk,
        seeded_m5_motion_reduced_motion_registries(),
        "checked support export drifted from the seed builder"
    );
}

#[test]
fn narrowed_variants_validate_and_keep_rows_visible() {
    let beta = seeded_m5_motion_reduced_motion_registries_shell_ui_beta_narrowed();
    assert!(beta.validate().is_empty(), "{:?}", beta.validate());
    assert_eq!(beta.registry_rows.len(), 6);
    let row = beta
        .registry_rows
        .iter()
        .find(|r| r.consumer_surface == M5VisualInteractionConsumerSurface::ShellUi)
        .unwrap();
    assert_eq!(
        row.qualification,
        M5VisualInteractionQualificationClass::Beta
    );

    let preview = seeded_m5_motion_reduced_motion_registries_onboarding_ui_preview_narrowed();
    assert!(preview.validate().is_empty(), "{:?}", preview.validate());
    assert_eq!(preview.registry_rows.len(), 6);
    let row = preview
        .registry_rows
        .iter()
        .find(|r| r.consumer_surface == M5VisualInteractionConsumerSurface::OnboardingUi)
        .unwrap();
    assert_eq!(
        row.qualification,
        M5VisualInteractionQualificationClass::Preview
    );
}

#[test]
fn checked_narrowed_fixtures_validate_and_match_seed_builders() {
    let beta: M5MotionRegistriesPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/ui/m5-motion-token-and-reduced-motion-registries/shell_ui_beta_narrowed.json"
    )))
    .expect("shell-ui fixture parses");
    assert!(beta.validate().is_empty());
    assert_eq!(
        beta,
        seeded_m5_motion_reduced_motion_registries_shell_ui_beta_narrowed()
    );

    let preview: M5MotionRegistriesPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/ui/m5-motion-token-and-reduced-motion-registries/onboarding_ui_preview_narrowed.json"
    )))
    .expect("onboarding-ui fixture parses");
    assert!(preview.validate().is_empty());
    assert_eq!(
        preview,
        seeded_m5_motion_reduced_motion_registries_onboarding_ui_preview_narrowed()
    );
}

#[test]
fn implemented_families_are_motion_token_and_reduced_motion() {
    assert_eq!(
        IMPLEMENTED_FAMILIES,
        [
            M5VisualInteractionFamily::MotionToken,
            M5VisualInteractionFamily::ReducedMotion
        ]
    );
}
