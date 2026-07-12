use super::*;

fn clean_decoration_input() -> M5DiagnosticDecorationResolutionInput {
    M5DiagnosticDecorationResolutionInput {
        decoration_id: "decoration:test".to_owned(),
        message_label: "mismatched types".to_owned(),
        severity: M5DiagnosticSeverity::Error,
        severity_stated: true,
        source_class: M5DiagnosticSourceClass::LanguageServer,
        freshness: M5DiagnosticFreshness::Current,
        stale_disclosed: true,
        anchor_durability: M5AnchorDurability::AnchoredExact,
        anchor_drift_disclosed: true,
        linkage_target: M5DiagnosticLinkageTarget::ProblemsPanel,
        linkage_stable: true,
        imported_certainty_distinguished: true,
        detail_command_available: true,
        proof_fresh: true,
    }
}

fn clean_chip_input() -> M5CodeActionChipResolutionInput {
    M5CodeActionChipResolutionInput {
        chip_id: "chip:test".to_owned(),
        action_label: "Add missing import".to_owned(),
        fix_posture: M5FixPosture::ExactFix,
        posture_stated: true,
        shown_as_exact: true,
        apply_scope: M5CodeActionApplyScope::DirectApply,
        preview_available: true,
        side_effect_class: M5CodeActionSideEffectClass::SingleFile,
        side_effect_disclosed: true,
        block_reason: M5CodeActionBlockReason::NotBlocked,
        detail_command_available: true,
        proof_fresh: true,
    }
}

#[test]
fn seeded_controls_validates() {
    let packet = seeded_m5_diagnostic_chip_controls();
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
    assert_eq!(packet.packet_id, M5_DIAGNOSTIC_CHIP_CONTROLS_PACKET_ID);
}

#[test]
fn decoration_clean_names_severity_source_and_is_legible() {
    let resolved = resolve_diagnostic_decoration(clean_decoration_input()).unwrap();
    assert!(resolved.is_clean());
    assert!(resolved.decoration_legible_at_a_glance);
    assert!(resolved.severity_stated);
    assert_eq!(resolved.severity, "error");
    assert_eq!(resolved.source_class, "language_server");
    assert_eq!(resolved.freshness, "current");
    assert_eq!(resolved.linkage_target, "problems_panel");
    assert!(!resolved.source_is_imported);
    assert_eq!(
        resolved.next_action,
        M5DiagnosticChipNextAction::OpenDiagnosticDetail
    );
}

#[test]
fn decoration_identity_unstated_degrades() {
    let mut input = clean_decoration_input();
    input.message_label = "   ".to_owned();
    assert_eq!(
        resolve_diagnostic_decoration(input).unwrap().degrade_reason,
        Some(M5DiagnosticDecorationDegradeReason::DiagnosticIdentityUnstated)
    );
}

#[test]
fn decoration_severity_unresolved_and_color_only_degrade() {
    let mut input = clean_decoration_input();
    input.severity = M5DiagnosticSeverity::SeverityUnknown;
    assert_eq!(
        resolve_diagnostic_decoration(input).unwrap().degrade_reason,
        Some(M5DiagnosticDecorationDegradeReason::SeverityUnresolved)
    );

    let mut input = clean_decoration_input();
    input.severity_stated = false;
    assert_eq!(
        resolve_diagnostic_decoration(input).unwrap().degrade_reason,
        Some(M5DiagnosticDecorationDegradeReason::SeverityEncodedByColorAlone)
    );
}

#[test]
fn decoration_source_and_freshness_unresolved_degrade() {
    let mut input = clean_decoration_input();
    input.source_class = M5DiagnosticSourceClass::SourceUnknown;
    assert_eq!(
        resolve_diagnostic_decoration(input).unwrap().degrade_reason,
        Some(M5DiagnosticDecorationDegradeReason::SourceProviderUnstated)
    );

    let mut input = clean_decoration_input();
    input.freshness = M5DiagnosticFreshness::FreshnessUnknown;
    assert_eq!(
        resolve_diagnostic_decoration(input).unwrap().degrade_reason,
        Some(M5DiagnosticDecorationDegradeReason::FreshnessUnresolved)
    );
}

#[test]
fn decoration_stale_shown_as_current_degrades_but_disclosed_is_clean() {
    let mut input = clean_decoration_input();
    input.freshness = M5DiagnosticFreshness::Stale;
    input.stale_disclosed = false;
    let hidden = resolve_diagnostic_decoration(input).unwrap();
    assert_eq!(
        hidden.degrade_reason,
        Some(M5DiagnosticDecorationDegradeReason::StaleShownAsCurrent)
    );
    assert!(hidden.freshness_is_stale);

    let mut input = clean_decoration_input();
    input.freshness = M5DiagnosticFreshness::Stale;
    input.stale_disclosed = true;
    let disclosed = resolve_diagnostic_decoration(input).unwrap();
    assert!(disclosed.is_clean());
    assert!(disclosed.freshness_is_stale);
}

#[test]
fn decoration_anchor_unresolved_and_drift_hidden_degrade() {
    let mut input = clean_decoration_input();
    input.anchor_durability = M5AnchorDurability::AnchorUnresolved;
    assert_eq!(
        resolve_diagnostic_decoration(input).unwrap().degrade_reason,
        Some(M5DiagnosticDecorationDegradeReason::AnchorDurabilityUnresolved)
    );

    let mut input = clean_decoration_input();
    input.anchor_durability = M5AnchorDurability::OutdatedAnchor;
    input.anchor_drift_disclosed = false;
    let resolved = resolve_diagnostic_decoration(input).unwrap();
    assert!(resolved.anchor_is_drifted);
    assert_eq!(
        resolved.degrade_reason,
        Some(M5DiagnosticDecorationDegradeReason::AnchorDriftHidden)
    );
}

#[test]
fn decoration_linkage_unresolved_and_broken_degrade() {
    let mut input = clean_decoration_input();
    input.linkage_target = M5DiagnosticLinkageTarget::LinkageUnresolved;
    assert_eq!(
        resolve_diagnostic_decoration(input).unwrap().degrade_reason,
        Some(M5DiagnosticDecorationDegradeReason::LinkageTargetUnresolved)
    );

    let mut input = clean_decoration_input();
    input.linkage_stable = false;
    assert_eq!(
        resolve_diagnostic_decoration(input).unwrap().degrade_reason,
        Some(M5DiagnosticDecorationDegradeReason::ProblemsLinkageBroken)
    );
}

#[test]
fn decoration_imported_certainty_overstated_degrades() {
    let mut input = clean_decoration_input();
    input.source_class = M5DiagnosticSourceClass::ImportedExternal;
    input.imported_certainty_distinguished = false;
    let resolved = resolve_diagnostic_decoration(input).unwrap();
    assert!(resolved.source_is_imported);
    assert_eq!(
        resolved.degrade_reason,
        Some(M5DiagnosticDecorationDegradeReason::ImportedCertaintyOverstated)
    );
}

#[test]
fn decoration_detail_missing_degrades() {
    let mut input = clean_decoration_input();
    input.detail_command_available = false;
    assert_eq!(
        resolve_diagnostic_decoration(input).unwrap().degrade_reason,
        Some(M5DiagnosticDecorationDegradeReason::DiagnosticDetailPathMissing)
    );
}

#[test]
fn decoration_empty_id_and_forbidden_material_error() {
    let mut input = clean_decoration_input();
    input.decoration_id = "".to_owned();
    assert_eq!(
        resolve_diagnostic_decoration(input).unwrap_err(),
        M5DiagnosticChipResolutionError::EmptyDecorationId
    );

    let mut input = clean_decoration_input();
    input.message_label = "see https://relay.internal/leak".to_owned();
    assert_eq!(
        resolve_diagnostic_decoration(input).unwrap_err(),
        M5DiagnosticChipResolutionError::ForbiddenMaterial
    );
}

#[test]
fn chip_clean_names_posture_and_is_legible() {
    let resolved = resolve_code_action_chip(clean_chip_input()).unwrap();
    assert!(resolved.is_clean());
    assert!(resolved.fix_posture_legible_at_a_glance);
    assert!(resolved.fix_is_exact);
    assert!(!resolved.fix_is_inferred);
    assert_eq!(resolved.fix_posture, "exact_fix");
    assert_eq!(resolved.apply_scope, "direct_apply");
    assert_eq!(
        resolved.next_action,
        M5DiagnosticChipNextAction::PreviewFixBeforeApply
    );
}

#[test]
fn chip_inferred_and_heuristic_postures_are_named() {
    let mut input = clean_chip_input();
    input.fix_posture = M5FixPosture::InferredFix;
    input.shown_as_exact = false;
    input.apply_scope = M5CodeActionApplyScope::PreviewRequired;
    let inferred = resolve_code_action_chip(input).unwrap();
    assert!(inferred.is_clean());
    assert!(inferred.fix_is_inferred);
    assert!(inferred.requires_preview);

    let mut input = clean_chip_input();
    input.fix_posture = M5FixPosture::HeuristicSuggestion;
    input.shown_as_exact = false;
    input.apply_scope = M5CodeActionApplyScope::ReviewRequired;
    let heuristic = resolve_code_action_chip(input).unwrap();
    assert!(heuristic.is_clean());
    assert!(heuristic.fix_is_inferred);
    assert!(heuristic.requires_preview);
}

#[test]
fn chip_posture_unresolved_and_color_only_degrade() {
    let mut input = clean_chip_input();
    input.fix_posture = M5FixPosture::PostureUnknown;
    assert_eq!(
        resolve_code_action_chip(input).unwrap().degrade_reason,
        Some(M5CodeActionChipDegradeReason::FixPostureUnresolved)
    );

    let mut input = clean_chip_input();
    input.posture_stated = false;
    assert_eq!(
        resolve_code_action_chip(input).unwrap().degrade_reason,
        Some(M5CodeActionChipDegradeReason::FixPostureEncodedByColorAlone)
    );
}

#[test]
fn chip_inferred_shown_as_exact_degrades() {
    let mut input = clean_chip_input();
    input.fix_posture = M5FixPosture::InferredFix;
    input.shown_as_exact = true;
    let resolved = resolve_code_action_chip(input).unwrap();
    assert!(resolved.claims_exact);
    assert_eq!(
        resolved.degrade_reason,
        Some(M5CodeActionChipDegradeReason::InferredFixShownAsExact)
    );
}

#[test]
fn chip_scope_unresolved_and_preview_bypass_degrade() {
    let mut input = clean_chip_input();
    input.apply_scope = M5CodeActionApplyScope::ScopeUnresolved;
    assert_eq!(
        resolve_code_action_chip(input).unwrap().degrade_reason,
        Some(M5CodeActionChipDegradeReason::ApplyScopeUnresolved)
    );

    let mut input = clean_chip_input();
    input.apply_scope = M5CodeActionApplyScope::PreviewRequired;
    input.preview_available = false;
    assert_eq!(
        resolve_code_action_chip(input).unwrap().degrade_reason,
        Some(M5CodeActionChipDegradeReason::PreviewRequiredButBypassed)
    );
}

#[test]
fn chip_blocked_requires_reason_but_stated_is_clean() {
    let mut input = clean_chip_input();
    input.apply_scope = M5CodeActionApplyScope::Blocked;
    input.block_reason = M5CodeActionBlockReason::BlockReasonUnknown;
    assert_eq!(
        resolve_code_action_chip(input).unwrap().degrade_reason,
        Some(M5CodeActionChipDegradeReason::BlockedReasonHidden)
    );

    let mut input = clean_chip_input();
    input.apply_scope = M5CodeActionApplyScope::Blocked;
    input.block_reason = M5CodeActionBlockReason::PolicyDenied;
    let resolved = resolve_code_action_chip(input).unwrap();
    assert!(resolved.is_clean());
    assert!(resolved.is_blocked);
    assert_eq!(resolved.block_reason, "policy_denied");
}

#[test]
fn chip_side_effect_unresolved_and_hidden_degrade() {
    let mut input = clean_chip_input();
    input.side_effect_class = M5CodeActionSideEffectClass::SideEffectUnknown;
    assert_eq!(
        resolve_code_action_chip(input).unwrap().degrade_reason,
        Some(M5CodeActionChipDegradeReason::SideEffectClassUnresolved)
    );

    let mut input = clean_chip_input();
    input.side_effect_class = M5CodeActionSideEffectClass::MultiFile;
    input.side_effect_disclosed = false;
    let resolved = resolve_code_action_chip(input).unwrap();
    assert!(resolved.touches_multiple_or_external);
    assert_eq!(
        resolved.degrade_reason,
        Some(M5CodeActionChipDegradeReason::SideEffectClassHidden)
    );
}

#[test]
fn chip_detail_missing_degrades() {
    let mut input = clean_chip_input();
    input.detail_command_available = false;
    assert_eq!(
        resolve_code_action_chip(input).unwrap().degrade_reason,
        Some(M5CodeActionChipDegradeReason::ChipDetailPathMissing)
    );
}

#[test]
fn chip_empty_id_and_forbidden_material_error() {
    let mut input = clean_chip_input();
    input.chip_id = "   ".to_owned();
    assert_eq!(
        resolve_code_action_chip(input).unwrap_err(),
        M5DiagnosticChipResolutionError::EmptyChipId
    );

    let mut input = clean_chip_input();
    input.action_label = "connect to internal://host".to_owned();
    assert_eq!(
        resolve_code_action_chip(input).unwrap_err(),
        M5DiagnosticChipResolutionError::ForbiddenMaterial
    );
}

#[test]
fn vocabulary_set_is_canonical() {
    assert!(seeded_m5_diagnostic_chip_controls()
        .vocabulary_set
        .matches_canonical());
}

#[test]
fn vocabulary_set_drift_fails() {
    let mut packet = seeded_m5_diagnostic_chip_controls();
    packet.vocabulary_set.apply_scopes.pop();
    assert!(packet
        .validate()
        .contains(&M5DiagnosticChipControlsViolation::VocabularySetDrift));
}

#[test]
fn missing_source_contracts_fails() {
    let mut packet = seeded_m5_diagnostic_chip_controls();
    packet.source_contract_refs.clear();
    assert!(packet
        .validate()
        .contains(&M5DiagnosticChipControlsViolation::MissingSourceContracts));
}

#[test]
fn component_schema_ref_missing_fails() {
    let mut packet = seeded_m5_diagnostic_chip_controls();
    packet.controls_rows[0]
        .source_contract_refs
        .retain(|r| r != M5_CODE_ACTION_CHIP_SCHEMA_REF);
    assert!(packet
        .validate()
        .contains(&M5DiagnosticChipControlsViolation::ComponentSchemaRefMissing));
}

#[test]
fn mandatory_anatomy_missing_fails() {
    let mut packet = seeded_m5_diagnostic_chip_controls();
    packet.controls_rows[0]
        .anatomy_parts
        .retain(|p| *p != M5DiagnosticChipAnatomyPart::Identity);
    assert!(packet
        .validate()
        .contains(&M5DiagnosticChipControlsViolation::MandatoryAnatomyMissing));
}

#[test]
fn mandatory_export_field_missing_fails() {
    let mut packet = seeded_m5_diagnostic_chip_controls();
    packet.controls_rows[0]
        .export_fields
        .retain(|f| *f != M5DiagnosticChipExportField::Dispositions);
    assert!(packet
        .validate()
        .contains(&M5DiagnosticChipControlsViolation::MandatoryExportFieldMissing));
}

#[test]
fn examples_missing_fails() {
    let mut packet = seeded_m5_diagnostic_chip_controls();
    packet.controls_rows[0].chip_examples.clear();
    assert!(packet
        .validate()
        .contains(&M5DiagnosticChipControlsViolation::ExamplesMissing));
}

#[test]
fn dishonest_clean_example_fails() {
    let mut packet = seeded_m5_diagnostic_chip_controls();
    // Force a clean chip to also read as inferred-shown-as-exact — the packet must reject it.
    let row = &mut packet.controls_rows[0];
    row.chip_examples[0].degrade_reason = None;
    row.chip_examples[0].fix_is_inferred = true;
    row.chip_examples[0].claims_exact = true;
    assert!(packet
        .validate()
        .contains(&M5DiagnosticChipControlsViolation::DishonestExample));
}

#[test]
fn row_invariant_violation_fails() {
    for mutate in 0u8..4 {
        let mut packet = seeded_m5_diagnostic_chip_controls();
        let row = &mut packet.controls_rows[0];
        match mutate {
            0 => row.diagnostic_severity_or_source_encoded_by_color_alone = true,
            1 => row.diagnostic_anchor_or_freshness_silently_drifts = true,
            2 => row.inferred_or_blocked_fix_presented_as_exact_or_ready = true,
            _ => row.code_action_bypasses_preview_or_apply_truth = true,
        }
        assert!(packet
            .validate()
            .contains(&M5DiagnosticChipControlsViolation::RowInvariantViolated));
    }
}

#[test]
fn severity_source_vocabulary_not_proven_when_color_only_example_removed() {
    let mut packet = seeded_m5_diagnostic_chip_controls();
    for row in &mut packet.controls_rows {
        row.diagnostic_examples.retain(|ex| {
            ex.degrade_reason
                != Some(M5DiagnosticDecorationDegradeReason::SeverityEncodedByColorAlone)
        });
    }
    assert!(packet
        .validate()
        .contains(&M5DiagnosticChipControlsViolation::SeveritySourceFreshnessVocabularyNotProven));
}

#[test]
fn severity_source_vocabulary_not_proven_when_sources_collapse() {
    let mut packet = seeded_m5_diagnostic_chip_controls();
    // Drop every clean decoration whose source is not a language server so the source grammar
    // collapses to one class.
    for row in &mut packet.controls_rows {
        row.diagnostic_examples
            .retain(|ex| !(ex.is_clean() && ex.source_class != "language_server"));
    }
    assert!(packet
        .validate()
        .contains(&M5DiagnosticChipControlsViolation::SeveritySourceFreshnessVocabularyNotProven));
}

#[test]
fn fix_posture_legibility_not_proven_when_inferred_as_exact_removed() {
    let mut packet = seeded_m5_diagnostic_chip_controls();
    for row in &mut packet.controls_rows {
        row.chip_examples.retain(|ex| {
            ex.degrade_reason != Some(M5CodeActionChipDegradeReason::InferredFixShownAsExact)
        });
    }
    assert!(packet
        .validate()
        .contains(&M5DiagnosticChipControlsViolation::FixPostureLegibilityNotProven));
}

#[test]
fn preview_apply_truth_not_proven_when_bypass_example_removed() {
    let mut packet = seeded_m5_diagnostic_chip_controls();
    for row in &mut packet.controls_rows {
        row.chip_examples.retain(|ex| {
            ex.degrade_reason != Some(M5CodeActionChipDegradeReason::PreviewRequiredButBypassed)
        });
    }
    assert!(packet
        .validate()
        .contains(&M5DiagnosticChipControlsViolation::PreviewApplyTruthNotProven));
}

#[test]
fn preview_apply_truth_not_proven_when_clean_chips_lose_detail_path() {
    let mut packet = seeded_m5_diagnostic_chip_controls();
    for row in &mut packet.controls_rows {
        for c in &mut row.chip_examples {
            if c.is_clean() {
                c.detail_command_available = false;
            }
        }
    }
    let violations = packet.validate();
    assert!(violations.contains(&M5DiagnosticChipControlsViolation::PreviewApplyTruthNotProven));
}

#[test]
fn governance_review_incomplete_fails() {
    let mut packet = seeded_m5_diagnostic_chip_controls();
    packet
        .governance_review
        .inferred_fixes_never_presented_as_exact = false;
    assert!(packet
        .validate()
        .contains(&M5DiagnosticChipControlsViolation::GovernanceReviewIncomplete));
}

#[test]
fn consumer_projection_incomplete_fails() {
    let mut packet = seeded_m5_diagnostic_chip_controls();
    packet
        .consumer_projection
        .support_export_reads_single_editor_source = false;
    assert!(packet
        .validate()
        .contains(&M5DiagnosticChipControlsViolation::ConsumerProjectionIncomplete));
}

#[test]
fn proof_freshness_incomplete_fails() {
    let mut packet = seeded_m5_diagnostic_chip_controls();
    packet.proof_freshness.proof_freshness_slo_hours = 0;
    assert!(packet
        .validate()
        .contains(&M5DiagnosticChipControlsViolation::ProofFreshnessIncomplete));
}

#[test]
fn release_posture_incomplete_fails() {
    let mut packet = seeded_m5_diagnostic_chip_controls();
    packet.release_posture.accessibility_parity_required = false;
    assert!(packet
        .validate()
        .contains(&M5DiagnosticChipControlsViolation::ReleasePostureIncomplete));
}

#[test]
fn injected_raw_material_is_rejected() {
    let mut packet = seeded_m5_diagnostic_chip_controls();
    packet.controls_rows[0].scope_summary =
        "raw endpoint https://relay.internal.example/session leaked".to_owned();
    assert!(packet
        .validate()
        .contains(&M5DiagnosticChipControlsViolation::RawMaterialInExport));
}

#[test]
fn export_carries_no_forbidden_raw_material() {
    let json = seeded_m5_diagnostic_chip_controls().export_safe_json();
    let lower = json.to_lowercase();
    assert!(!lower.contains("password"));
    assert!(!lower.contains("passphrase"));
    assert!(!lower.contains("bearer "));
    assert!(!lower.contains("://"));
    assert!(!lower.contains("-----begin"));
}

#[test]
fn csv_has_a_row_per_consumer_surface() {
    let packet = seeded_m5_diagnostic_chip_controls();
    let csv = packet.render_matrix_csv();
    let lines: Vec<&str> = csv.lines().collect();
    assert_eq!(lines.len(), 1 + packet.controls_rows.len());
    assert!(lines[0].starts_with("consumer_surface,qualification,owner,"));
}

#[test]
fn markdown_summary_lists_every_consumer_surface() {
    let packet = seeded_m5_diagnostic_chip_controls();
    let summary = packet.render_markdown_summary();
    for row in &packet.controls_rows {
        assert!(summary.contains(row.consumer_surface.as_str()));
    }
}

#[test]
fn checked_support_export_validates_and_matches_seed() {
    let from_disk = current_stable_m5_diagnostic_chip_controls_export()
        .expect("checked M5 diagnostic-decoration / code-action-chip controls export validates");
    assert_eq!(from_disk.packet_id, M5_DIAGNOSTIC_CHIP_CONTROLS_PACKET_ID);
    assert_eq!(
        from_disk,
        seeded_m5_diagnostic_chip_controls(),
        "checked support export drifted from the seed builder"
    );
}

#[test]
fn narrowed_variants_validate_and_keep_rows_visible() {
    let beta = seeded_m5_diagnostic_chip_controls_diagnostics_ui_beta_narrowed();
    assert!(beta.validate().is_empty(), "{:?}", beta.validate());
    assert_eq!(beta.controls_rows.len(), 6);
    let row = beta
        .controls_rows
        .iter()
        .find(|r| r.consumer_surface == M5EditorInlineConsumerSurface::DiagnosticsUi)
        .unwrap();
    assert_eq!(row.qualification, M5EditorInlineQualificationClass::Beta);

    let preview = seeded_m5_diagnostic_chip_controls_ai_ui_preview_narrowed();
    assert!(preview.validate().is_empty(), "{:?}", preview.validate());
    assert_eq!(preview.controls_rows.len(), 6);
    let row = preview
        .controls_rows
        .iter()
        .find(|r| r.consumer_surface == M5EditorInlineConsumerSurface::AiUi)
        .unwrap();
    assert_eq!(row.qualification, M5EditorInlineQualificationClass::Preview);
}

#[test]
fn checked_narrowed_fixtures_validate_and_match_seed_builders() {
    let beta: M5DiagnosticChipControlsPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/ui/m5-diagnostic-decoration-code-action-chip-controls/diagnostics_ui_beta_narrowed.json"
    )))
    .expect("diagnostics-ui fixture parses");
    assert!(beta.validate().is_empty());
    assert_eq!(
        beta,
        seeded_m5_diagnostic_chip_controls_diagnostics_ui_beta_narrowed()
    );

    let preview: M5DiagnosticChipControlsPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/ui/m5-diagnostic-decoration-code-action-chip-controls/ai_ui_preview_narrowed.json"
    )))
    .expect("ai-ui fixture parses");
    assert!(preview.validate().is_empty());
    assert_eq!(
        preview,
        seeded_m5_diagnostic_chip_controls_ai_ui_preview_narrowed()
    );
}

#[test]
fn implemented_families_are_diagnostic_decoration_and_code_action_chip() {
    assert_eq!(
        IMPLEMENTED_FAMILIES,
        [
            M5EditorInlineComponentFamily::DiagnosticDecoration,
            M5EditorInlineComponentFamily::CodeActionChip,
        ]
    );
}
