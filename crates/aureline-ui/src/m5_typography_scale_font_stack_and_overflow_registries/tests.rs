use super::*;

fn clean_type_scale_input() -> M5TypeScaleEntryResolutionInput {
    M5TypeScaleEntryResolutionInput {
        entry_id: "type:test".to_owned(),
        token_name: "type.body".to_owned(),
        semantic_role: M5VisualSemanticRole::Neutral,
        typography_role: M5TypographyRole::BodyScale,
        text_role: M5TextRole::Body,
        font_stack: M5FontStackSelection::UiSansStack,
        case_rule: M5TextCaseRule::SentenceCase,
        surface_context: M5TextSurfaceContext::Editor,
        line_height_guarded: true,
        tabular_numerals_enabled: false,
        references_canonical_token: true,
        proof_fresh: true,
    }
}

fn clean_overflow_input() -> M5OverflowEntryResolutionInput {
    M5OverflowEntryResolutionInput {
        entry_id: "overflow:test".to_owned(),
        token_name: "overflow.row".to_owned(),
        semantic_role: M5VisualSemanticRole::Neutral,
        surface_element: M5TextSurfaceElement::Row,
        overflow_treatment: M5OverflowTreatment::TruncateWithTooltip,
        density_context: M5DensityContext::Compact,
        surface_context: M5TextSurfaceContext::Data,
        full_meaning_reachable: true,
        survives_zoom: true,
        survives_density: true,
        references_canonical_token: true,
        proof_fresh: true,
    }
}

#[test]
fn seeded_registries_validates() {
    let packet = seeded_m5_typography_overflow_registries();
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
    assert_eq!(
        packet.packet_id,
        M5_TYPOGRAPHY_OVERFLOW_REGISTRIES_PACKET_ID
    );
}

#[test]
fn type_scale_clean_reads_readable() {
    let resolved = resolve_type_scale_entry(clean_type_scale_input()).unwrap();
    assert!(resolved.is_clean());
    assert!(resolved.type_hierarchy_is_readable);
    assert!(resolved.font_stack_stable);
    assert!(resolved.line_height_guarded);
    assert!(resolved.case_rule_stated);
    assert!(resolved.references_canonical_token);
    assert_eq!(resolved.text_role, "body");
    assert_eq!(resolved.surface_context, "editor");
    assert_eq!(resolved.next_action, M5TextNextAction::InspectTypeScale);
}

#[test]
fn type_scale_font_stack_mismatch_degrades() {
    // A code role must select the monospace stack; selecting the UI stack is unstable.
    let mut input = clean_type_scale_input();
    input.text_role = M5TextRole::Code;
    input.font_stack = M5FontStackSelection::UiSansStack;
    let resolved = resolve_type_scale_entry(input).unwrap();
    assert!(!resolved.font_stack_stable);
    assert_eq!(
        resolved.degrade_reason,
        Some(M5TypeScaleDegradeReason::FontStackUnstable)
    );

    // A local font fork is never canonical.
    let mut input = clean_type_scale_input();
    input.font_stack = M5FontStackSelection::LocalFontStackDisallowed;
    assert_eq!(
        resolve_type_scale_entry(input).unwrap().degrade_reason,
        Some(M5TypeScaleDegradeReason::FontStackUnstable)
    );
}

#[test]
fn type_scale_line_height_role_and_case_degrade() {
    let mut input = clean_type_scale_input();
    input.line_height_guarded = false;
    assert_eq!(
        resolve_type_scale_entry(input).unwrap().degrade_reason,
        Some(M5TypeScaleDegradeReason::LineHeightDrifted)
    );

    let mut input = clean_type_scale_input();
    input.text_role = M5TextRole::RoleUnknown;
    assert_eq!(
        resolve_type_scale_entry(input).unwrap().degrade_reason,
        Some(M5TypeScaleDegradeReason::TypeRoleUnstated)
    );

    let mut input = clean_type_scale_input();
    input.case_rule = M5TextCaseRule::CaseRuleUnknown;
    assert_eq!(
        resolve_type_scale_entry(input).unwrap().degrade_reason,
        Some(M5TypeScaleDegradeReason::CaseRuleUnstated)
    );
}

#[test]
fn type_scale_tabular_numerals_required_for_numeric_data() {
    let mut input = clean_type_scale_input();
    input.text_role = M5TextRole::NumericData;
    input.tabular_numerals_enabled = false;
    let resolved = resolve_type_scale_entry(input).unwrap();
    assert!(resolved.text_role_demands_tabular_numerals);
    assert_eq!(
        resolved.degrade_reason,
        Some(M5TypeScaleDegradeReason::TabularNumeralsMissing)
    );

    let mut input = clean_type_scale_input();
    input.text_role = M5TextRole::NumericData;
    input.tabular_numerals_enabled = true;
    assert!(resolve_type_scale_entry(input).unwrap().is_clean());
}

#[test]
fn type_scale_raw_token_and_forbidden_degrade() {
    let mut input = clean_type_scale_input();
    input.references_canonical_token = false;
    assert_eq!(
        resolve_type_scale_entry(input).unwrap().degrade_reason,
        Some(M5TypeScaleDegradeReason::RawTypeValueInlined)
    );

    let mut input = clean_type_scale_input();
    input.token_name = "  ".to_owned();
    assert_eq!(
        resolve_type_scale_entry(input).unwrap().degrade_reason,
        Some(M5TypeScaleDegradeReason::TokenNameUnstated)
    );

    let mut input = clean_type_scale_input();
    input.surface_context = M5TextSurfaceContext::ContextUnknown;
    assert_eq!(
        resolve_type_scale_entry(input).unwrap().degrade_reason,
        Some(M5TypeScaleDegradeReason::SurfaceContextUnresolved)
    );

    let mut input = clean_type_scale_input();
    input.entry_id = "".to_owned();
    assert_eq!(
        resolve_type_scale_entry(input).unwrap_err(),
        M5TypographyOverflowResolutionError::EmptyTypeScaleEntryId
    );

    let mut input = clean_type_scale_input();
    input.token_name = "https://relay.internal/leak".to_owned();
    assert_eq!(
        resolve_type_scale_entry(input).unwrap_err(),
        M5TypographyOverflowResolutionError::ForbiddenMaterial
    );
}

#[test]
fn overflow_clean_preserves_meaning_and_survives() {
    let resolved = resolve_overflow_entry(clean_overflow_input()).unwrap();
    assert!(resolved.is_clean());
    assert!(resolved.meaning_survives_zoom_and_density);
    assert!(resolved.overflow_preserves_meaning);
    assert!(resolved.full_meaning_reachable);
    assert!(resolved.survives_zoom);
    assert!(resolved.survives_density);
    assert_eq!(resolved.surface_element, "row");
    assert_eq!(resolved.surface_context, "data");
    assert_eq!(
        resolved.next_action,
        M5TextNextAction::AdjustOverflowBehavior
    );
}

#[test]
fn overflow_silent_clip_and_unreachable_degrade() {
    let mut input = clean_overflow_input();
    input.overflow_treatment = M5OverflowTreatment::SilentClipDisallowed;
    let resolved = resolve_overflow_entry(input).unwrap();
    assert!(!resolved.overflow_preserves_meaning);
    assert_eq!(
        resolved.degrade_reason,
        Some(M5OverflowDegradeReason::MeaningSilentlyDestroyed)
    );

    let mut input = clean_overflow_input();
    input.full_meaning_reachable = false;
    assert_eq!(
        resolve_overflow_entry(input).unwrap().degrade_reason,
        Some(M5OverflowDegradeReason::FullMeaningUnreachable)
    );

    let mut input = clean_overflow_input();
    input.surface_element = M5TextSurfaceElement::ElementUnknown;
    assert_eq!(
        resolve_overflow_entry(input).unwrap().degrade_reason,
        Some(M5OverflowDegradeReason::SurfaceElementUnresolved)
    );
}

#[test]
fn overflow_zoom_density_and_raw_degrade() {
    let mut input = clean_overflow_input();
    input.survives_zoom = false;
    assert_eq!(
        resolve_overflow_entry(input).unwrap().degrade_reason,
        Some(M5OverflowDegradeReason::ZoomRegression)
    );

    let mut input = clean_overflow_input();
    input.survives_density = false;
    assert_eq!(
        resolve_overflow_entry(input).unwrap().degrade_reason,
        Some(M5OverflowDegradeReason::DensityRegression)
    );

    let mut input = clean_overflow_input();
    input.references_canonical_token = false;
    assert_eq!(
        resolve_overflow_entry(input).unwrap().degrade_reason,
        Some(M5OverflowDegradeReason::RawLayoutValueInlined)
    );

    let mut input = clean_overflow_input();
    input.entry_id = "  ".to_owned();
    assert_eq!(
        resolve_overflow_entry(input).unwrap_err(),
        M5TypographyOverflowResolutionError::EmptyOverflowEntryId
    );

    let mut input = clean_overflow_input();
    input.token_name = "bearer abc".to_owned();
    assert_eq!(
        resolve_overflow_entry(input).unwrap_err(),
        M5TypographyOverflowResolutionError::ForbiddenMaterial
    );
}

#[test]
fn vocabulary_set_is_canonical() {
    assert!(seeded_m5_typography_overflow_registries()
        .vocabulary_set
        .matches_canonical());
}

#[test]
fn vocabulary_set_drift_fails() {
    let mut packet = seeded_m5_typography_overflow_registries();
    packet.vocabulary_set.text_roles.pop();
    assert!(packet
        .validate()
        .contains(&M5TypographyOverflowRegistriesViolation::VocabularySetDrift));
}

#[test]
fn missing_source_contracts_fails() {
    let mut packet = seeded_m5_typography_overflow_registries();
    packet.source_contract_refs.clear();
    assert!(packet
        .validate()
        .contains(&M5TypographyOverflowRegistriesViolation::MissingSourceContracts));
}

#[test]
fn domain_schema_ref_missing_fails() {
    let mut packet = seeded_m5_typography_overflow_registries();
    packet.registry_rows[0]
        .source_contract_refs
        .retain(|r| r != M5_TYPOGRAPHY_AND_GEOMETRY_SCHEMA_REF);
    assert!(packet
        .validate()
        .contains(&M5TypographyOverflowRegistriesViolation::DomainSchemaRefMissing));
}

#[test]
fn mandatory_anatomy_missing_fails() {
    let mut packet = seeded_m5_typography_overflow_registries();
    packet.registry_rows[0]
        .anatomy_parts
        .retain(|p| *p != M5TextAnatomyPart::Identity);
    assert!(packet
        .validate()
        .contains(&M5TypographyOverflowRegistriesViolation::MandatoryAnatomyMissing));
}

#[test]
fn mandatory_export_field_missing_fails() {
    let mut packet = seeded_m5_typography_overflow_registries();
    packet.registry_rows[0]
        .export_fields
        .retain(|f| *f != M5TextExportField::SemanticRoles);
    assert!(packet
        .validate()
        .contains(&M5TypographyOverflowRegistriesViolation::MandatoryExportFieldMissing));
}

#[test]
fn examples_missing_fails() {
    let mut packet = seeded_m5_typography_overflow_registries();
    let row = &mut packet.registry_rows[0];
    row.type_scale_entries.clear();
    row.overflow_entries.clear();
    assert!(packet
        .validate()
        .contains(&M5TypographyOverflowRegistriesViolation::ExamplesMissing));
}

#[test]
fn dishonest_clean_example_fails() {
    let mut packet = seeded_m5_typography_overflow_registries();
    // Force a clean type-scale entry to also read as an unstable font stack — the packet must reject it.
    let row = &mut packet.registry_rows[0];
    row.type_scale_entries[0].degrade_reason = None;
    row.type_scale_entries[0].font_stack_stable = false;
    assert!(packet
        .validate()
        .contains(&M5TypographyOverflowRegistriesViolation::DishonestExample));
}

#[test]
fn row_invariant_violation_fails() {
    for mutate in 0u8..4 {
        let mut packet = seeded_m5_typography_overflow_registries();
        let row = &mut packet.registry_rows[0];
        match mutate {
            0 => row.typography_scale_or_font_stack_drifted = true,
            1 => row.overflow_silently_destroyed_meaning = true,
            2 => row.zoom_or_density_regression_uncaught = true,
            _ => row.raw_type_value_inlined_instead_of_token = true,
        }
        assert!(packet
            .validate()
            .contains(&M5TypographyOverflowRegistriesViolation::RowInvariantViolated));
    }
}

#[test]
fn shared_hierarchy_not_proven_when_raw_type_example_removed() {
    let mut packet = seeded_m5_typography_overflow_registries();
    for row in &mut packet.registry_rows {
        row.type_scale_entries
            .retain(|ex| ex.degrade_reason != Some(M5TypeScaleDegradeReason::RawTypeValueInlined));
    }
    assert!(packet
        .validate()
        .contains(&M5TypographyOverflowRegistriesViolation::SharedTypeHierarchyNotProven));
}

#[test]
fn shared_hierarchy_not_proven_when_code_role_dropped() {
    let mut packet = seeded_m5_typography_overflow_registries();
    // Drop every clean code-role entry so the hierarchy no longer covers "code".
    for row in &mut packet.registry_rows {
        row.type_scale_entries
            .retain(|ex| !(ex.is_clean() && ex.text_role == "code"));
    }
    assert!(packet
        .validate()
        .contains(&M5TypographyOverflowRegistriesViolation::SharedTypeHierarchyNotProven));
}

#[test]
fn tabular_or_overflow_not_proven_when_tabular_missing_removed() {
    let mut packet = seeded_m5_typography_overflow_registries();
    for row in &mut packet.registry_rows {
        row.type_scale_entries.retain(|ex| {
            ex.degrade_reason != Some(M5TypeScaleDegradeReason::TabularNumeralsMissing)
        });
    }
    assert!(packet.validate().contains(
        &M5TypographyOverflowRegistriesViolation::TabularNumeralsOrOverflowSafetyNotProven
    ));
}

#[test]
fn tabular_or_overflow_not_proven_when_meaning_destroyed_removed() {
    let mut packet = seeded_m5_typography_overflow_registries();
    for row in &mut packet.registry_rows {
        row.overflow_entries.retain(|ex| {
            ex.degrade_reason != Some(M5OverflowDegradeReason::MeaningSilentlyDestroyed)
        });
    }
    assert!(packet.validate().contains(
        &M5TypographyOverflowRegistriesViolation::TabularNumeralsOrOverflowSafetyNotProven
    ));
}

#[test]
fn zoom_density_not_caught_when_zoom_regression_removed() {
    let mut packet = seeded_m5_typography_overflow_registries();
    for row in &mut packet.registry_rows {
        row.overflow_entries
            .retain(|ex| ex.degrade_reason != Some(M5OverflowDegradeReason::ZoomRegression));
    }
    assert!(packet
        .validate()
        .contains(&M5TypographyOverflowRegistriesViolation::ZoomDensityRegressionNotCaught));
}

#[test]
fn zoom_density_not_caught_when_line_height_drift_removed() {
    let mut packet = seeded_m5_typography_overflow_registries();
    for row in &mut packet.registry_rows {
        row.type_scale_entries
            .retain(|ex| ex.degrade_reason != Some(M5TypeScaleDegradeReason::LineHeightDrifted));
    }
    assert!(packet
        .validate()
        .contains(&M5TypographyOverflowRegistriesViolation::ZoomDensityRegressionNotCaught));
}

#[test]
fn governance_review_incomplete_fails() {
    let mut packet = seeded_m5_typography_overflow_registries();
    packet
        .governance_review
        .overflow_never_silently_destroys_meaning = false;
    assert!(packet
        .validate()
        .contains(&M5TypographyOverflowRegistriesViolation::GovernanceReviewIncomplete));
}

#[test]
fn consumer_projection_incomplete_fails() {
    let mut packet = seeded_m5_typography_overflow_registries();
    packet
        .consumer_projection
        .support_export_reads_single_typography_source = false;
    assert!(packet
        .validate()
        .contains(&M5TypographyOverflowRegistriesViolation::ConsumerProjectionIncomplete));
}

#[test]
fn proof_freshness_incomplete_fails() {
    let mut packet = seeded_m5_typography_overflow_registries();
    packet.proof_freshness.proof_freshness_slo_hours = 0;
    assert!(packet
        .validate()
        .contains(&M5TypographyOverflowRegistriesViolation::ProofFreshnessIncomplete));
}

#[test]
fn release_posture_incomplete_fails() {
    let mut packet = seeded_m5_typography_overflow_registries();
    packet.release_posture.accessibility_parity_required = false;
    assert!(packet
        .validate()
        .contains(&M5TypographyOverflowRegistriesViolation::ReleasePostureIncomplete));
}

#[test]
fn injected_raw_material_is_rejected() {
    let mut packet = seeded_m5_typography_overflow_registries();
    packet.registry_rows[0].scope_summary =
        "raw endpoint https://relay.internal.example/session leaked".to_owned();
    assert!(packet
        .validate()
        .contains(&M5TypographyOverflowRegistriesViolation::RawMaterialInExport));
}

#[test]
fn export_carries_no_forbidden_raw_material() {
    let json = seeded_m5_typography_overflow_registries().export_safe_json();
    let lower = json.to_lowercase();
    assert!(!lower.contains("password"));
    assert!(!lower.contains("passphrase"));
    assert!(!lower.contains("bearer "));
    assert!(!lower.contains("://"));
    assert!(!lower.contains("-----begin"));
}

#[test]
fn csv_has_a_row_per_consumer_surface() {
    let packet = seeded_m5_typography_overflow_registries();
    let csv = packet.render_matrix_csv();
    let lines: Vec<&str> = csv.lines().collect();
    assert_eq!(lines.len(), 1 + packet.registry_rows.len());
    assert!(lines[0].starts_with("consumer_surface,qualification,owner,"));
}

#[test]
fn markdown_summary_lists_every_consumer_surface() {
    let packet = seeded_m5_typography_overflow_registries();
    let summary = packet.render_markdown_summary();
    for row in &packet.registry_rows {
        assert!(summary.contains(row.consumer_surface.as_str()));
    }
}

#[test]
fn checked_support_export_validates_and_matches_seed() {
    let from_disk = current_stable_m5_typography_overflow_registries_export()
        .expect("checked M5 typography / overflow registries export validates");
    assert_eq!(
        from_disk.packet_id,
        M5_TYPOGRAPHY_OVERFLOW_REGISTRIES_PACKET_ID
    );
    assert_eq!(
        from_disk,
        seeded_m5_typography_overflow_registries(),
        "checked support export drifted from the seed builder"
    );
}

#[test]
fn narrowed_variants_validate_and_keep_rows_visible() {
    let beta = seeded_m5_typography_overflow_registries_editor_ui_beta_narrowed();
    assert!(beta.validate().is_empty(), "{:?}", beta.validate());
    assert_eq!(beta.registry_rows.len(), 6);
    let row = beta
        .registry_rows
        .iter()
        .find(|r| r.consumer_surface == M5VisualFoundationConsumerSurface::EditorUi)
        .unwrap();
    assert_eq!(
        row.qualification,
        M5VisualFoundationQualificationClass::Beta
    );

    let preview = seeded_m5_typography_overflow_registries_data_ui_preview_narrowed();
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
    let beta: M5TypographyOverflowRegistriesPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/ui/m5-typography-scale-font-stack-and-overflow-registries/editor_ui_beta_narrowed.json"
    )))
    .expect("editor-ui fixture parses");
    assert!(beta.validate().is_empty());
    assert_eq!(
        beta,
        seeded_m5_typography_overflow_registries_editor_ui_beta_narrowed()
    );

    let preview: M5TypographyOverflowRegistriesPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/ui/m5-typography-scale-font-stack-and-overflow-registries/data_ui_preview_narrowed.json"
    )))
    .expect("data-ui fixture parses");
    assert!(preview.validate().is_empty());
    assert_eq!(
        preview,
        seeded_m5_typography_overflow_registries_data_ui_preview_narrowed()
    );
}

#[test]
fn implemented_families_are_typography() {
    assert_eq!(IMPLEMENTED_FAMILIES, [M5VisualFoundationFamily::Typography]);
}
