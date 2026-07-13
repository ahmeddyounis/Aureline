use super::*;

fn clean_syntax_input() -> M5SyntaxEntryResolutionInput {
    M5SyntaxEntryResolutionInput {
        entry_id: "syntax:test".to_owned(),
        token_name: "syntax.keyword".to_owned(),
        semantic_role: M5VisualSemanticRole::Syntax,
        syntax_role: M5SyntaxTokenRole::Keyword,
        diagnostics_posture: M5SyntaxDiagnosticsPosture::DiagnosticsOutrankSyntax,
        surface_context: M5CodeDataSurfaceContext::Editor,
        export_channels: M5MeaningExportChannel::REQUIRED.to_vec(),
        references_canonical_token: true,
        proof_fresh: true,
    }
}

fn clean_diff_input() -> M5DiffEntryResolutionInput {
    M5DiffEntryResolutionInput {
        entry_id: "diff:test".to_owned(),
        token_name: "diff.moved".to_owned(),
        semantic_role: M5VisualSemanticRole::Diff,
        diff_role: M5DiffTokenRole::Moved,
        moved_confidence: M5DiffMovedConfidence::HighConfidenceMove,
        historical_emphasis: M5DiffEmphasis::CurrentEmphasis,
        non_color_cue: M5CodeDataNonColorCue::FillPattern,
        surface_context: M5CodeDataSurfaceContext::Review,
        export_channels: M5MeaningExportChannel::REQUIRED.to_vec(),
        distinct_from_diagnostics: true,
        references_canonical_token: true,
        proof_fresh: true,
    }
}

fn clean_chart_input() -> M5ChartEntryResolutionInput {
    M5ChartEntryResolutionInput {
        entry_id: "chart:test".to_owned(),
        token_name: "chart.categorical_series".to_owned(),
        semantic_role: M5VisualSemanticRole::Chart,
        chart_role: M5ChartTokenRole::CategoricalSeries,
        non_color_cue: M5CodeDataNonColorCue::Legend,
        surface_context: M5CodeDataSurfaceContext::Data,
        export_channels: M5MeaningExportChannel::REQUIRED.to_vec(),
        legend_or_pattern_present: true,
        accessible_contrast: true,
        references_canonical_token: true,
        proof_fresh: true,
    }
}

#[test]
fn seeded_registries_validates() {
    let packet = seeded_m5_syntax_diff_chart_registries();
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
    assert_eq!(packet.packet_id, M5_SYNTAX_DIFF_CHART_REGISTRIES_PACKET_ID);
}

#[test]
fn syntax_clean_honors_precedence() {
    let resolved = resolve_syntax_entry(clean_syntax_input()).unwrap();
    assert!(resolved.is_clean());
    assert!(resolved.diagnostics_outrank_syntax);
    assert!(resolved.diagnostics_precedence_honored);
    assert!(resolved.survives_required_export_channels);
    assert!(!resolved.syntax_role_collides_with_diagnostics);
    assert_eq!(resolved.semantic_role, "syntax");
    assert_eq!(resolved.surface_context, "editor");
    assert_eq!(
        resolved.next_action,
        M5CodeDataNextAction::InspectDiagnosticsPrecedence
    );
}

#[test]
fn syntax_collision_and_precedence_degrade() {
    let mut input = clean_syntax_input();
    input.syntax_role = M5SyntaxTokenRole::SyntaxDiagnosticCollisionDisallowed;
    let resolved = resolve_syntax_entry(input).unwrap();
    assert!(resolved.syntax_role_collides_with_diagnostics);
    assert_eq!(
        resolved.degrade_reason,
        Some(M5SyntaxEntryDegradeReason::SyntaxCollidesWithDiagnostics)
    );

    let mut input = clean_syntax_input();
    input.diagnostics_posture = M5SyntaxDiagnosticsPosture::SyntaxOutranksDiagnosticsDisallowed;
    assert_eq!(
        resolve_syntax_entry(input).unwrap().degrade_reason,
        Some(M5SyntaxEntryDegradeReason::DiagnosticsPrecedenceMissing)
    );
}

#[test]
fn syntax_raw_export_and_token_degrade() {
    let mut input = clean_syntax_input();
    input.references_canonical_token = false;
    assert_eq!(
        resolve_syntax_entry(input).unwrap().degrade_reason,
        Some(M5SyntaxEntryDegradeReason::RawColorValueInlined)
    );

    let mut input = clean_syntax_input();
    input.export_channels = vec![M5MeaningExportChannel::Screenshot];
    assert_eq!(
        resolve_syntax_entry(input).unwrap().degrade_reason,
        Some(M5SyntaxEntryDegradeReason::ExportMeaningLost)
    );

    let mut input = clean_syntax_input();
    input.token_name = "  ".to_owned();
    assert_eq!(
        resolve_syntax_entry(input).unwrap().degrade_reason,
        Some(M5SyntaxEntryDegradeReason::TokenNameUnstated)
    );

    let mut input = clean_syntax_input();
    input.surface_context = M5CodeDataSurfaceContext::ContextUnknown;
    assert_eq!(
        resolve_syntax_entry(input).unwrap().degrade_reason,
        Some(M5SyntaxEntryDegradeReason::SurfaceContextUnresolved)
    );
}

#[test]
fn syntax_empty_id_and_forbidden_material_error() {
    let mut input = clean_syntax_input();
    input.entry_id = "".to_owned();
    assert_eq!(
        resolve_syntax_entry(input).unwrap_err(),
        M5SyntaxDiffChartResolutionError::EmptySyntaxEntryId
    );

    let mut input = clean_syntax_input();
    input.token_name = "https://relay.internal/leak".to_owned();
    assert_eq!(
        resolve_syntax_entry(input).unwrap_err(),
        M5SyntaxDiffChartResolutionError::ForbiddenMaterial
    );
}

#[test]
fn diff_clean_states_notes_and_survives() {
    let resolved = resolve_diff_entry(clean_diff_input()).unwrap();
    assert!(resolved.is_clean());
    assert!(resolved.meaning_survives_export);
    assert!(resolved.moved_confidence_stated);
    assert!(resolved.historical_emphasis_stated);
    assert!(resolved.non_color_cue_present);
    assert!(resolved.distinct_from_diagnostics);
    assert_eq!(resolved.semantic_role, "diff");
    assert_eq!(resolved.moved_confidence, "high_confidence_move");
    assert_eq!(
        resolved.next_action,
        M5CodeDataNextAction::AddLegendOrPattern
    );
}

#[test]
fn diff_collision_cue_and_notes_degrade() {
    let mut input = clean_diff_input();
    input.diff_role = M5DiffTokenRole::DiffDiagnosticCollisionDisallowed;
    assert_eq!(
        resolve_diff_entry(input).unwrap().degrade_reason,
        Some(M5DiffEntryDegradeReason::DiffCollidesWithDiagnostics)
    );

    let mut input = clean_diff_input();
    input.distinct_from_diagnostics = false;
    assert_eq!(
        resolve_diff_entry(input).unwrap().degrade_reason,
        Some(M5DiffEntryDegradeReason::DiffCollidesWithDiagnostics)
    );

    let mut input = clean_diff_input();
    input.non_color_cue = M5CodeDataNonColorCue::NoneDisallowed;
    assert_eq!(
        resolve_diff_entry(input).unwrap().degrade_reason,
        Some(M5DiffEntryDegradeReason::NonColorCueMissing)
    );

    let mut input = clean_diff_input();
    input.moved_confidence = M5DiffMovedConfidence::ConfidenceUnknown;
    assert_eq!(
        resolve_diff_entry(input).unwrap().degrade_reason,
        Some(M5DiffEntryDegradeReason::MovedConfidenceUnstated)
    );

    let mut input = clean_diff_input();
    input.historical_emphasis = M5DiffEmphasis::EmphasisUnknown;
    assert_eq!(
        resolve_diff_entry(input).unwrap().degrade_reason,
        Some(M5DiffEntryDegradeReason::HistoricalEmphasisUnstated)
    );
}

#[test]
fn diff_raw_export_and_id_and_material() {
    let mut input = clean_diff_input();
    input.references_canonical_token = false;
    assert_eq!(
        resolve_diff_entry(input).unwrap().degrade_reason,
        Some(M5DiffEntryDegradeReason::RawColorValueInlined)
    );

    let mut input = clean_diff_input();
    input.export_channels = partial_free();
    assert_eq!(
        resolve_diff_entry(input).unwrap().degrade_reason,
        Some(M5DiffEntryDegradeReason::ExportMeaningLost)
    );

    let mut input = clean_diff_input();
    input.entry_id = "  ".to_owned();
    assert_eq!(
        resolve_diff_entry(input).unwrap_err(),
        M5SyntaxDiffChartResolutionError::EmptyDiffEntryId
    );

    let mut input = clean_diff_input();
    input.token_name = "see internal://notes".to_owned();
    assert_eq!(
        resolve_diff_entry(input).unwrap_err(),
        M5SyntaxDiffChartResolutionError::ForbiddenMaterial
    );
}

fn partial_free() -> Vec<M5MeaningExportChannel> {
    vec![
        M5MeaningExportChannel::Screenshot,
        M5MeaningExportChannel::Pdf,
    ]
}

#[test]
fn chart_clean_pairs_cue_and_survives() {
    let resolved = resolve_chart_entry(clean_chart_input()).unwrap();
    assert!(resolved.is_clean());
    assert!(resolved.meaning_survives_without_color);
    assert!(resolved.non_color_cue_present);
    assert!(resolved.legend_or_pattern_present);
    assert!(resolved.accessible_contrast);
    assert!(!resolved.chart_role_is_color_alone);
    assert_eq!(resolved.semantic_role, "chart");
    assert_eq!(resolved.surface_context, "data");
}

#[test]
fn chart_color_alone_legend_and_contrast_degrade() {
    let mut input = clean_chart_input();
    input.chart_role = M5ChartTokenRole::ChartColorAloneDisallowed;
    let resolved = resolve_chart_entry(input).unwrap();
    assert!(resolved.chart_role_is_color_alone);
    assert_eq!(
        resolved.degrade_reason,
        Some(M5ChartEntryDegradeReason::ChartMeaningColorAlone)
    );

    let mut input = clean_chart_input();
    input.non_color_cue = M5CodeDataNonColorCue::NoneDisallowed;
    assert_eq!(
        resolve_chart_entry(input).unwrap().degrade_reason,
        Some(M5ChartEntryDegradeReason::ChartMeaningColorAlone)
    );

    let mut input = clean_chart_input();
    input.legend_or_pattern_present = false;
    assert_eq!(
        resolve_chart_entry(input).unwrap().degrade_reason,
        Some(M5ChartEntryDegradeReason::LegendOrPatternMissing)
    );

    let mut input = clean_chart_input();
    input.accessible_contrast = false;
    assert_eq!(
        resolve_chart_entry(input).unwrap().degrade_reason,
        Some(M5ChartEntryDegradeReason::ContrastInsufficient)
    );
}

#[test]
fn chart_raw_export_and_id_and_material() {
    let mut input = clean_chart_input();
    input.references_canonical_token = false;
    assert_eq!(
        resolve_chart_entry(input).unwrap().degrade_reason,
        Some(M5ChartEntryDegradeReason::RawColorValueInlined)
    );

    let mut input = clean_chart_input();
    input.export_channels = partial_free();
    assert_eq!(
        resolve_chart_entry(input).unwrap().degrade_reason,
        Some(M5ChartEntryDegradeReason::ExportMeaningLost)
    );

    let mut input = clean_chart_input();
    input.entry_id = "".to_owned();
    assert_eq!(
        resolve_chart_entry(input).unwrap_err(),
        M5SyntaxDiffChartResolutionError::EmptyChartEntryId
    );

    let mut input = clean_chart_input();
    input.token_name = "bearer abc".to_owned();
    assert_eq!(
        resolve_chart_entry(input).unwrap_err(),
        M5SyntaxDiffChartResolutionError::ForbiddenMaterial
    );
}

#[test]
fn vocabulary_set_is_canonical() {
    assert!(seeded_m5_syntax_diff_chart_registries()
        .vocabulary_set
        .matches_canonical());
}

#[test]
fn vocabulary_set_drift_fails() {
    let mut packet = seeded_m5_syntax_diff_chart_registries();
    packet.vocabulary_set.chart_roles.pop();
    assert!(packet
        .validate()
        .contains(&M5SyntaxDiffChartRegistriesViolation::VocabularySetDrift));
}

#[test]
fn missing_source_contracts_fails() {
    let mut packet = seeded_m5_syntax_diff_chart_registries();
    packet.source_contract_refs.clear();
    assert!(packet
        .validate()
        .contains(&M5SyntaxDiffChartRegistriesViolation::MissingSourceContracts));
}

#[test]
fn domain_schema_ref_missing_fails() {
    let mut packet = seeded_m5_syntax_diff_chart_registries();
    packet.registry_rows[0]
        .source_contract_refs
        .retain(|r| r != M5_SYNTAX_DIFF_CHART_TOKENS_SCHEMA_REF);
    assert!(packet
        .validate()
        .contains(&M5SyntaxDiffChartRegistriesViolation::DomainSchemaRefMissing));
}

#[test]
fn mandatory_anatomy_missing_fails() {
    let mut packet = seeded_m5_syntax_diff_chart_registries();
    packet.registry_rows[0]
        .anatomy_parts
        .retain(|p| *p != M5CodeDataAnatomyPart::Identity);
    assert!(packet
        .validate()
        .contains(&M5SyntaxDiffChartRegistriesViolation::MandatoryAnatomyMissing));
}

#[test]
fn mandatory_export_field_missing_fails() {
    let mut packet = seeded_m5_syntax_diff_chart_registries();
    packet.registry_rows[0]
        .export_fields
        .retain(|f| *f != M5CodeDataExportField::SemanticRoles);
    assert!(packet
        .validate()
        .contains(&M5SyntaxDiffChartRegistriesViolation::MandatoryExportFieldMissing));
}

#[test]
fn examples_missing_fails() {
    let mut packet = seeded_m5_syntax_diff_chart_registries();
    let row = &mut packet.registry_rows[0];
    row.syntax_entries.clear();
    row.diff_entries.clear();
    row.chart_entries.clear();
    assert!(packet
        .validate()
        .contains(&M5SyntaxDiffChartRegistriesViolation::ExamplesMissing));
}

#[test]
fn dishonest_clean_example_fails() {
    let mut packet = seeded_m5_syntax_diff_chart_registries();
    // Force a clean syntax entry to also read as a diagnostics collision — the packet must reject it.
    let row = &mut packet.registry_rows[0];
    row.syntax_entries[0].degrade_reason = None;
    row.syntax_entries[0].syntax_role_collides_with_diagnostics = true;
    assert!(packet
        .validate()
        .contains(&M5SyntaxDiffChartRegistriesViolation::DishonestExample));
}

#[test]
fn row_invariant_violation_fails() {
    for mutate in 0u8..4 {
        let mut packet = seeded_m5_syntax_diff_chart_registries();
        let row = &mut packet.registry_rows[0];
        match mutate {
            0 => row.syntax_or_diff_palette_collides_with_diagnostics = true,
            1 => row.chart_meaning_relies_on_color_alone = true,
            2 => row.meaning_lost_under_high_contrast_or_export = true,
            _ => row.raw_color_value_inlined_instead_of_token = true,
        }
        assert!(packet
            .validate()
            .contains(&M5SyntaxDiffChartRegistriesViolation::RowInvariantViolated));
    }
}

#[test]
fn shared_mapping_not_proven_when_raw_color_example_removed() {
    let mut packet = seeded_m5_syntax_diff_chart_registries();
    for row in &mut packet.registry_rows {
        row.syntax_entries.retain(|ex| {
            ex.degrade_reason != Some(M5SyntaxEntryDegradeReason::RawColorValueInlined)
        });
        row.diff_entries
            .retain(|ex| ex.degrade_reason != Some(M5DiffEntryDegradeReason::RawColorValueInlined));
        row.chart_entries.retain(|ex| {
            ex.degrade_reason != Some(M5ChartEntryDegradeReason::RawColorValueInlined)
        });
    }
    assert!(packet
        .validate()
        .contains(&M5SyntaxDiffChartRegistriesViolation::SharedSemanticMappingNotProven));
}

#[test]
fn shared_mapping_not_proven_when_chart_family_collapses() {
    let mut packet = seeded_m5_syntax_diff_chart_registries();
    // Drop every clean chart entry so the semantic-role grammar no longer covers "chart".
    for row in &mut packet.registry_rows {
        row.chart_entries
            .retain(|ex| !(ex.is_clean() && ex.semantic_role == "chart"));
    }
    assert!(packet
        .validate()
        .contains(&M5SyntaxDiffChartRegistriesViolation::SharedSemanticMappingNotProven));
}

#[test]
fn precedence_or_export_not_proven_when_collision_example_removed() {
    let mut packet = seeded_m5_syntax_diff_chart_registries();
    for row in &mut packet.registry_rows {
        row.syntax_entries.retain(|ex| {
            ex.degrade_reason != Some(M5SyntaxEntryDegradeReason::SyntaxCollidesWithDiagnostics)
        });
    }
    assert!(packet.validate().contains(
        &M5SyntaxDiffChartRegistriesViolation::DiagnosticsPrecedenceOrExportSurvivalNotProven
    ));
}

#[test]
fn precedence_or_export_not_proven_when_export_loss_removed() {
    let mut packet = seeded_m5_syntax_diff_chart_registries();
    for row in &mut packet.registry_rows {
        row.syntax_entries
            .retain(|ex| ex.degrade_reason != Some(M5SyntaxEntryDegradeReason::ExportMeaningLost));
        row.diff_entries
            .retain(|ex| ex.degrade_reason != Some(M5DiffEntryDegradeReason::ExportMeaningLost));
        row.chart_entries
            .retain(|ex| ex.degrade_reason != Some(M5ChartEntryDegradeReason::ExportMeaningLost));
    }
    assert!(packet.validate().contains(
        &M5SyntaxDiffChartRegistriesViolation::DiagnosticsPrecedenceOrExportSurvivalNotProven
    ));
}

#[test]
fn legend_parity_not_proven_when_color_alone_example_removed() {
    let mut packet = seeded_m5_syntax_diff_chart_registries();
    for row in &mut packet.registry_rows {
        row.chart_entries.retain(|ex| {
            ex.degrade_reason != Some(M5ChartEntryDegradeReason::ChartMeaningColorAlone)
        });
    }
    assert!(packet
        .validate()
        .contains(&M5SyntaxDiffChartRegistriesViolation::LegendOrPatternParityNotProven));
}

#[test]
fn legend_parity_not_proven_when_diff_cue_missing_removed() {
    let mut packet = seeded_m5_syntax_diff_chart_registries();
    for row in &mut packet.registry_rows {
        row.diff_entries
            .retain(|ex| ex.degrade_reason != Some(M5DiffEntryDegradeReason::NonColorCueMissing));
    }
    assert!(packet
        .validate()
        .contains(&M5SyntaxDiffChartRegistriesViolation::LegendOrPatternParityNotProven));
}

#[test]
fn governance_review_incomplete_fails() {
    let mut packet = seeded_m5_syntax_diff_chart_registries();
    packet
        .governance_review
        .diagnostics_outrank_syntax_where_they_overlap = false;
    assert!(packet
        .validate()
        .contains(&M5SyntaxDiffChartRegistriesViolation::GovernanceReviewIncomplete));
}

#[test]
fn consumer_projection_incomplete_fails() {
    let mut packet = seeded_m5_syntax_diff_chart_registries();
    packet
        .consumer_projection
        .support_export_reads_single_registry_source = false;
    assert!(packet
        .validate()
        .contains(&M5SyntaxDiffChartRegistriesViolation::ConsumerProjectionIncomplete));
}

#[test]
fn proof_freshness_incomplete_fails() {
    let mut packet = seeded_m5_syntax_diff_chart_registries();
    packet.proof_freshness.proof_freshness_slo_hours = 0;
    assert!(packet
        .validate()
        .contains(&M5SyntaxDiffChartRegistriesViolation::ProofFreshnessIncomplete));
}

#[test]
fn release_posture_incomplete_fails() {
    let mut packet = seeded_m5_syntax_diff_chart_registries();
    packet.release_posture.accessibility_parity_required = false;
    assert!(packet
        .validate()
        .contains(&M5SyntaxDiffChartRegistriesViolation::ReleasePostureIncomplete));
}

#[test]
fn injected_raw_material_is_rejected() {
    let mut packet = seeded_m5_syntax_diff_chart_registries();
    packet.registry_rows[0].scope_summary =
        "raw endpoint https://relay.internal.example/session leaked".to_owned();
    assert!(packet
        .validate()
        .contains(&M5SyntaxDiffChartRegistriesViolation::RawMaterialInExport));
}

#[test]
fn export_carries_no_forbidden_raw_material() {
    let json = seeded_m5_syntax_diff_chart_registries().export_safe_json();
    let lower = json.to_lowercase();
    assert!(!lower.contains("password"));
    assert!(!lower.contains("passphrase"));
    assert!(!lower.contains("bearer "));
    assert!(!lower.contains("://"));
    assert!(!lower.contains("-----begin"));
}

#[test]
fn csv_has_a_row_per_consumer_surface() {
    let packet = seeded_m5_syntax_diff_chart_registries();
    let csv = packet.render_matrix_csv();
    let lines: Vec<&str> = csv.lines().collect();
    assert_eq!(lines.len(), 1 + packet.registry_rows.len());
    assert!(lines[0].starts_with("consumer_surface,qualification,owner,"));
}

#[test]
fn markdown_summary_lists_every_consumer_surface() {
    let packet = seeded_m5_syntax_diff_chart_registries();
    let summary = packet.render_markdown_summary();
    for row in &packet.registry_rows {
        assert!(summary.contains(row.consumer_surface.as_str()));
    }
}

#[test]
fn checked_support_export_validates_and_matches_seed() {
    let from_disk = current_stable_m5_syntax_diff_chart_registries_export()
        .expect("checked M5 syntax / diff / chart registries export validates");
    assert_eq!(
        from_disk.packet_id,
        M5_SYNTAX_DIFF_CHART_REGISTRIES_PACKET_ID
    );
    assert_eq!(
        from_disk,
        seeded_m5_syntax_diff_chart_registries(),
        "checked support export drifted from the seed builder"
    );
}

#[test]
fn narrowed_variants_validate_and_keep_rows_visible() {
    let beta = seeded_m5_syntax_diff_chart_registries_editor_ui_beta_narrowed();
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

    let preview = seeded_m5_syntax_diff_chart_registries_data_ui_preview_narrowed();
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
    let beta: M5SyntaxDiffChartRegistriesPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/ui/m5-syntax-diff-and-chart-token-registries/editor_ui_beta_narrowed.json"
    )))
    .expect("editor-ui fixture parses");
    assert!(beta.validate().is_empty());
    assert_eq!(
        beta,
        seeded_m5_syntax_diff_chart_registries_editor_ui_beta_narrowed()
    );

    let preview: M5SyntaxDiffChartRegistriesPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/ui/m5-syntax-diff-and-chart-token-registries/data_ui_preview_narrowed.json"
    )))
    .expect("data-ui fixture parses");
    assert!(preview.validate().is_empty());
    assert_eq!(
        preview,
        seeded_m5_syntax_diff_chart_registries_data_ui_preview_narrowed()
    );
}

#[test]
fn implemented_families_are_syntax_diff_and_chart() {
    assert_eq!(
        IMPLEMENTED_FAMILIES,
        [
            M5VisualFoundationFamily::SyntaxToken,
            M5VisualFoundationFamily::DiffToken,
            M5VisualFoundationFamily::ChartToken
        ]
    );
}
