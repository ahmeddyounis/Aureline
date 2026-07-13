use super::*;

fn clean_scrim_input() -> M5ScrimEntryResolutionInput {
    M5ScrimEntryResolutionInput {
        entry_id: "scrim:test".to_owned(),
        token_name: "scrim.blocking.modal".to_owned(),
        semantic_role: M5VisualInteractionRole::Overlay,
        scrim_role: M5OpacityScrimRole::ScrimLayer,
        depth_class: M5OverlayDepthClass::BlockingModalDialog,
        contrast_treatment: M5ScrimContrastTreatment::DimBackdropReadableText,
        surface_context: M5OverlaySurfaceContext::Dialog,
        clamp_coverage: M5OverlayRuntimeClamp::ALL.to_vec(),
        preserves_orientation: true,
        preserves_text_contrast: true,
        references_canonical_token: true,
        proof_fresh: true,
    }
}

fn clean_depth_input() -> M5OverlayDepthEntryResolutionInput {
    M5OverlayDepthEntryResolutionInput {
        entry_id: "depth:test".to_owned(),
        token_name: "layer.dialog.modal".to_owned(),
        layer_order_role: M5LayerOrderRole::DialogTier,
        semantic_role: M5VisualInteractionRole::Layer,
        depth_class: M5OverlayDepthClass::BlockingModalDialog,
        surface_context: M5OverlaySurfaceContext::Dialog,
        clamp_coverage: M5OverlayRuntimeClamp::ALL.to_vec(),
        references_canonical_token: true,
        stacks_under_shared_model: true,
        proof_fresh: true,
    }
}

#[test]
fn seeded_registries_validates() {
    let packet = seeded_m5_opacity_scrim_overlay_depth_registries();
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
    assert_eq!(packet.packet_id, M5_OVERLAY_REGISTRIES_PACKET_ID);
}

#[test]
fn scrim_clean_names_meaning_and_is_safe() {
    let resolved = resolve_scrim_entry(clean_scrim_input()).unwrap();
    assert!(resolved.is_clean());
    assert!(resolved.overlay_orientation_preserved);
    assert!(resolved.covers_all_clamps);
    assert!(resolved.contrast_treatment_present);
    assert!(resolved.depth_class_is_classified);
    assert!(resolved.depth_class_is_blocking);
    assert!(!resolved.scrim_role_erases_orientation_or_contrast);
    assert_eq!(resolved.semantic_role, "overlay");
    assert_eq!(resolved.depth_class, "blocking_modal_dialog");
    assert_eq!(resolved.surface_context, "dialog");
    assert_eq!(
        resolved.next_action,
        M5OverlayRegistryNextAction::ExpandOverlayMeaning
    );
}

#[test]
fn scrim_token_unstated_degrades() {
    let mut input = clean_scrim_input();
    input.token_name = "   ".to_owned();
    assert_eq!(
        resolve_scrim_entry(input).unwrap().degrade_reason,
        Some(M5ScrimEntryDegradeReason::TokenNameUnstated)
    );
}

#[test]
fn scrim_orientation_and_contrast_cue_degrade() {
    let mut input = clean_scrim_input();
    input.preserves_orientation = false;
    assert_eq!(
        resolve_scrim_entry(input).unwrap().degrade_reason,
        Some(M5ScrimEntryDegradeReason::OrientationErasedByScrim)
    );

    let mut input = clean_scrim_input();
    input.scrim_role = M5OpacityScrimRole::ScrimErasesOrientationOrContrastDisallowed;
    let resolved = resolve_scrim_entry(input).unwrap();
    assert!(resolved.scrim_role_erases_orientation_or_contrast);
    assert_eq!(
        resolved.degrade_reason,
        Some(M5ScrimEntryDegradeReason::OrientationErasedByScrim)
    );

    let mut input = clean_scrim_input();
    input.contrast_treatment = M5ScrimContrastTreatment::NoneDisallowed;
    assert_eq!(
        resolve_scrim_entry(input).unwrap().degrade_reason,
        Some(M5ScrimEntryDegradeReason::ContrastCueMissing)
    );
}

#[test]
fn scrim_raw_opacity_and_unclassified_degrade() {
    let mut input = clean_scrim_input();
    input.references_canonical_token = false;
    assert_eq!(
        resolve_scrim_entry(input).unwrap().degrade_reason,
        Some(M5ScrimEntryDegradeReason::RawOpacityValueInlined)
    );

    let mut input = clean_scrim_input();
    input.depth_class = M5OverlayDepthClass::DepthClassUnclassified;
    assert_eq!(
        resolve_scrim_entry(input).unwrap().degrade_reason,
        Some(M5ScrimEntryDegradeReason::DepthClassUnclassified)
    );
}

#[test]
fn scrim_clamp_and_text_contrast_degrade() {
    let mut input = clean_scrim_input();
    input.clamp_coverage = vec![
        M5OverlayRuntimeClamp::ReducedMotion,
        M5OverlayRuntimeClamp::PowerSaver,
    ];
    assert_eq!(
        resolve_scrim_entry(input).unwrap().degrade_reason,
        Some(M5ScrimEntryDegradeReason::ClampCoverageIncomplete)
    );

    let mut input = clean_scrim_input();
    input.preserves_text_contrast = false;
    assert_eq!(
        resolve_scrim_entry(input).unwrap().degrade_reason,
        Some(M5ScrimEntryDegradeReason::TextContrastLost)
    );

    let mut input = clean_scrim_input();
    input.surface_context = M5OverlaySurfaceContext::ContextUnknown;
    assert_eq!(
        resolve_scrim_entry(input).unwrap().degrade_reason,
        Some(M5ScrimEntryDegradeReason::SurfaceContextUnresolved)
    );
}

#[test]
fn scrim_empty_id_and_forbidden_material_error() {
    let mut input = clean_scrim_input();
    input.entry_id = "".to_owned();
    assert_eq!(
        resolve_scrim_entry(input).unwrap_err(),
        M5OverlayResolutionError::EmptyScrimEntryId
    );

    let mut input = clean_scrim_input();
    input.token_name = "https://relay.internal/leak".to_owned();
    assert_eq!(
        resolve_scrim_entry(input).unwrap_err(),
        M5OverlayResolutionError::ForbiddenMaterial
    );
}

#[test]
fn depth_clean_stays_safe_under_shared_model() {
    let resolved = resolve_overlay_depth_entry(clean_depth_input()).unwrap();
    assert!(resolved.is_clean());
    assert!(resolved.depth_truth_holds_across_clamps);
    assert!(resolved.covers_all_clamps);
    assert!(resolved.stacks_under_shared_model);
    assert!(!resolved.layer_order_role_is_private_bypass);
    assert!(resolved.depth_class_is_blocking);
    assert_eq!(resolved.layer_order_role, "dialog_tier");
    assert_eq!(resolved.surface_context, "dialog");
}

#[test]
fn depth_private_bypass_and_unclassified_degrade() {
    let mut input = clean_depth_input();
    input.layer_order_role = M5LayerOrderRole::PrivateLayerBypassDisallowed;
    let resolved = resolve_overlay_depth_entry(input).unwrap();
    assert!(resolved.layer_order_role_is_private_bypass);
    assert_eq!(
        resolved.degrade_reason,
        Some(M5OverlayDepthEntryDegradeReason::PrivateLayerBypassWithoutSharedModel)
    );

    let mut input = clean_depth_input();
    input.references_canonical_token = false;
    assert_eq!(
        resolve_overlay_depth_entry(input).unwrap().degrade_reason,
        Some(M5OverlayDepthEntryDegradeReason::PrivateLayerBypassWithoutSharedModel)
    );

    let mut input = clean_depth_input();
    input.depth_class = M5OverlayDepthClass::DepthClassUnclassified;
    assert_eq!(
        resolve_overlay_depth_entry(input).unwrap().degrade_reason,
        Some(M5OverlayDepthEntryDegradeReason::DepthClassUnclassified)
    );
}

#[test]
fn depth_clamp_not_stacked_surface_id_material() {
    let mut input = clean_depth_input();
    input.clamp_coverage = vec![M5OverlayRuntimeClamp::ReducedMotion];
    assert_eq!(
        resolve_overlay_depth_entry(input).unwrap().degrade_reason,
        Some(M5OverlayDepthEntryDegradeReason::ClampCoverageIncomplete)
    );

    let mut input = clean_depth_input();
    input.stacks_under_shared_model = false;
    assert_eq!(
        resolve_overlay_depth_entry(input).unwrap().degrade_reason,
        Some(M5OverlayDepthEntryDegradeReason::NotStackedUnderSharedModel)
    );

    let mut input = clean_depth_input();
    input.surface_context = M5OverlaySurfaceContext::ContextUnknown;
    assert_eq!(
        resolve_overlay_depth_entry(input).unwrap().degrade_reason,
        Some(M5OverlayDepthEntryDegradeReason::SurfaceContextUnresolved)
    );

    let mut input = clean_depth_input();
    input.entry_id = "  ".to_owned();
    assert_eq!(
        resolve_overlay_depth_entry(input).unwrap_err(),
        M5OverlayResolutionError::EmptyOverlayDepthEntryId
    );

    let mut input = clean_depth_input();
    input.token_name = "see internal://notes".to_owned();
    assert_eq!(
        resolve_overlay_depth_entry(input).unwrap_err(),
        M5OverlayResolutionError::ForbiddenMaterial
    );
}

#[test]
fn vocabulary_set_is_canonical() {
    assert!(seeded_m5_opacity_scrim_overlay_depth_registries()
        .vocabulary_set
        .matches_canonical());
}

#[test]
fn vocabulary_set_drift_fails() {
    let mut packet = seeded_m5_opacity_scrim_overlay_depth_registries();
    packet.vocabulary_set.overlay_depth_classes.pop();
    assert!(packet
        .validate()
        .contains(&M5OverlayRegistriesViolation::VocabularySetDrift));
}

#[test]
fn missing_source_contracts_fails() {
    let mut packet = seeded_m5_opacity_scrim_overlay_depth_registries();
    packet.source_contract_refs.clear();
    assert!(packet
        .validate()
        .contains(&M5OverlayRegistriesViolation::MissingSourceContracts));
}

#[test]
fn domain_schema_ref_missing_fails() {
    let mut packet = seeded_m5_opacity_scrim_overlay_depth_registries();
    packet.registry_rows[0]
        .source_contract_refs
        .retain(|r| r != M5_LAYER_ORDER_AND_PORTAL_SCHEMA_REF);
    assert!(packet
        .validate()
        .contains(&M5OverlayRegistriesViolation::DomainSchemaRefMissing));
}

#[test]
fn mandatory_anatomy_missing_fails() {
    let mut packet = seeded_m5_opacity_scrim_overlay_depth_registries();
    packet.registry_rows[0]
        .anatomy_parts
        .retain(|p| *p != M5OverlayRegistryAnatomyPart::Identity);
    assert!(packet
        .validate()
        .contains(&M5OverlayRegistriesViolation::MandatoryAnatomyMissing));
}

#[test]
fn mandatory_export_field_missing_fails() {
    let mut packet = seeded_m5_opacity_scrim_overlay_depth_registries();
    packet.registry_rows[0]
        .export_fields
        .retain(|f| *f != M5OverlayRegistryExportField::DepthClasses);
    assert!(packet
        .validate()
        .contains(&M5OverlayRegistriesViolation::MandatoryExportFieldMissing));
}

#[test]
fn examples_missing_fails() {
    let mut packet = seeded_m5_opacity_scrim_overlay_depth_registries();
    packet.registry_rows[0].overlay_depth_entries.clear();
    assert!(packet
        .validate()
        .contains(&M5OverlayRegistriesViolation::ExamplesMissing));
}

#[test]
fn dishonest_clean_example_fails() {
    let mut packet = seeded_m5_opacity_scrim_overlay_depth_registries();
    // Force a clean scrim entry to also read as orientation-erasing — the packet must reject it.
    let row = &mut packet.registry_rows[0];
    row.scrim_entries[0].degrade_reason = None;
    row.scrim_entries[0].preserves_orientation = false;
    assert!(packet
        .validate()
        .contains(&M5OverlayRegistriesViolation::DishonestExample));
}

#[test]
fn row_invariant_violation_fails() {
    for mutate in 0u8..4 {
        let mut packet = seeded_m5_opacity_scrim_overlay_depth_registries();
        let row = &mut packet.registry_rows[0];
        match mutate {
            0 => row.scrim_erases_orientation_or_contrast = true,
            1 => row.raw_opacity_value_inlined_instead_of_token = true,
            2 => row.overlay_bypasses_shared_z_order = true,
            _ => row.runtime_clamp_coverage_incomplete = true,
        }
        assert!(packet
            .validate()
            .contains(&M5OverlayRegistriesViolation::RowInvariantViolated));
    }
}

#[test]
fn first_consumers_not_proven_when_raw_opacity_example_removed() {
    let mut packet = seeded_m5_opacity_scrim_overlay_depth_registries();
    for row in &mut packet.registry_rows {
        row.scrim_entries.retain(|ex| {
            ex.degrade_reason != Some(M5ScrimEntryDegradeReason::RawOpacityValueInlined)
        });
    }
    assert!(packet.validate().contains(
        &M5OverlayRegistriesViolation::FirstConsumersUseCanonicalOverlayGrammarNotProven
    ));
}

#[test]
fn first_consumers_not_proven_when_semantic_family_collapses() {
    let mut packet = seeded_m5_opacity_scrim_overlay_depth_registries();
    // Drop every clean attention scrim entry so the semantic-role grammar no longer covers "attention".
    for row in &mut packet.registry_rows {
        row.scrim_entries
            .retain(|ex| !(ex.is_clean() && ex.semantic_role == "attention"));
    }
    assert!(packet.validate().contains(
        &M5OverlayRegistriesViolation::FirstConsumersUseCanonicalOverlayGrammarNotProven
    ));
}

#[test]
fn scrims_preserve_contrast_not_proven_when_clamp_incomplete_example_removed() {
    let mut packet = seeded_m5_opacity_scrim_overlay_depth_registries();
    for row in &mut packet.registry_rows {
        row.scrim_entries.retain(|ex| {
            ex.degrade_reason != Some(M5ScrimEntryDegradeReason::ClampCoverageIncomplete)
        });
    }
    assert!(packet
        .validate()
        .contains(&M5OverlayRegistriesViolation::ScrimsPreserveContrastAndOrientationNotProven));
}

#[test]
fn scrims_preserve_contrast_not_proven_when_blocking_class_dropped() {
    let mut packet = seeded_m5_opacity_scrim_overlay_depth_registries();
    // Drop every clean confirm-scrim entry so blocking coverage no longer includes it.
    for row in &mut packet.registry_rows {
        row.scrim_entries
            .retain(|ex| !(ex.is_clean() && ex.depth_class == "blocking_confirm_scrim"));
    }
    assert!(packet
        .validate()
        .contains(&M5OverlayRegistriesViolation::ScrimsPreserveContrastAndOrientationNotProven));
}

#[test]
fn depth_truth_not_proven_when_private_bypass_example_removed() {
    let mut packet = seeded_m5_opacity_scrim_overlay_depth_registries();
    for row in &mut packet.registry_rows {
        row.overlay_depth_entries.retain(|ex| {
            ex.degrade_reason
                != Some(M5OverlayDepthEntryDegradeReason::PrivateLayerBypassWithoutSharedModel)
        });
    }
    assert!(packet
        .validate()
        .contains(&M5OverlayRegistriesViolation::BlockingVersusNonBlockingDepthTruthNotProven));
}

#[test]
fn depth_truth_not_proven_when_blocking_depth_dropped() {
    let mut packet = seeded_m5_opacity_scrim_overlay_depth_registries();
    // Drop every clean modal-dialog overlay-depth entry so blocking depth coverage no longer includes it.
    for row in &mut packet.registry_rows {
        row.overlay_depth_entries
            .retain(|ex| !(ex.is_clean() && ex.depth_class == "blocking_modal_dialog"));
    }
    assert!(packet
        .validate()
        .contains(&M5OverlayRegistriesViolation::BlockingVersusNonBlockingDepthTruthNotProven));
}

#[test]
fn governance_review_incomplete_fails() {
    let mut packet = seeded_m5_opacity_scrim_overlay_depth_registries();
    packet.governance_review.scrim_never_erases_orientation = false;
    assert!(packet
        .validate()
        .contains(&M5OverlayRegistriesViolation::GovernanceReviewIncomplete));
}

#[test]
fn consumer_projection_incomplete_fails() {
    let mut packet = seeded_m5_opacity_scrim_overlay_depth_registries();
    packet
        .consumer_projection
        .support_export_reads_single_registry_source = false;
    assert!(packet
        .validate()
        .contains(&M5OverlayRegistriesViolation::ConsumerProjectionIncomplete));
}

#[test]
fn proof_freshness_incomplete_fails() {
    let mut packet = seeded_m5_opacity_scrim_overlay_depth_registries();
    packet.proof_freshness.proof_freshness_slo_hours = 0;
    assert!(packet
        .validate()
        .contains(&M5OverlayRegistriesViolation::ProofFreshnessIncomplete));
}

#[test]
fn release_posture_incomplete_fails() {
    let mut packet = seeded_m5_opacity_scrim_overlay_depth_registries();
    packet.release_posture.accessibility_parity_required = false;
    assert!(packet
        .validate()
        .contains(&M5OverlayRegistriesViolation::ReleasePostureIncomplete));
}

#[test]
fn injected_raw_material_is_rejected() {
    let mut packet = seeded_m5_opacity_scrim_overlay_depth_registries();
    packet.registry_rows[0].scope_summary =
        "raw endpoint https://relay.internal.example/session leaked".to_owned();
    assert!(packet
        .validate()
        .contains(&M5OverlayRegistriesViolation::RawMaterialInExport));
}

#[test]
fn export_carries_no_forbidden_raw_material() {
    let json = seeded_m5_opacity_scrim_overlay_depth_registries().export_safe_json();
    let lower = json.to_lowercase();
    assert!(!lower.contains("password"));
    assert!(!lower.contains("passphrase"));
    assert!(!lower.contains("bearer "));
    assert!(!lower.contains("://"));
    assert!(!lower.contains("-----begin"));
}

#[test]
fn csv_has_a_row_per_consumer_surface() {
    let packet = seeded_m5_opacity_scrim_overlay_depth_registries();
    let csv = packet.render_matrix_csv();
    let lines: Vec<&str> = csv.lines().collect();
    assert_eq!(lines.len(), 1 + packet.registry_rows.len());
    assert!(lines[0].starts_with("consumer_surface,qualification,owner,"));
}

#[test]
fn markdown_summary_lists_every_consumer_surface() {
    let packet = seeded_m5_opacity_scrim_overlay_depth_registries();
    let summary = packet.render_markdown_summary();
    for row in &packet.registry_rows {
        assert!(summary.contains(row.consumer_surface.as_str()));
    }
}

#[test]
fn checked_support_export_validates_and_matches_seed() {
    let from_disk = current_stable_m5_opacity_scrim_overlay_depth_registries_export()
        .expect("checked M5 opacity / scrim and overlay-depth registries export validates");
    assert_eq!(from_disk.packet_id, M5_OVERLAY_REGISTRIES_PACKET_ID);
    assert_eq!(
        from_disk,
        seeded_m5_opacity_scrim_overlay_depth_registries(),
        "checked support export drifted from the seed builder"
    );
}

#[test]
fn narrowed_variants_validate_and_keep_rows_visible() {
    let beta = seeded_m5_opacity_scrim_overlay_depth_registries_shell_ui_beta_narrowed();
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

    let preview = seeded_m5_opacity_scrim_overlay_depth_registries_onboarding_ui_preview_narrowed();
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
    let beta: M5OverlayRegistriesPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/ui/m5-opacity-scrim-and-overlay-depth-registries/shell_ui_beta_narrowed.json"
    )))
    .expect("shell-ui fixture parses");
    assert!(beta.validate().is_empty());
    assert_eq!(
        beta,
        seeded_m5_opacity_scrim_overlay_depth_registries_shell_ui_beta_narrowed()
    );

    let preview: M5OverlayRegistriesPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/ui/m5-opacity-scrim-and-overlay-depth-registries/onboarding_ui_preview_narrowed.json"
    )))
    .expect("onboarding-ui fixture parses");
    assert!(preview.validate().is_empty());
    assert_eq!(
        preview,
        seeded_m5_opacity_scrim_overlay_depth_registries_onboarding_ui_preview_narrowed()
    );
}

#[test]
fn implemented_families_are_opacity_scrim_and_layer_order() {
    assert_eq!(
        IMPLEMENTED_FAMILIES,
        [
            M5VisualInteractionFamily::OpacityScrim,
            M5VisualInteractionFamily::LayerOrder
        ]
    );
}
