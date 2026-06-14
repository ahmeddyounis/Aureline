use super::*;

fn doc_ref() -> String {
    CODE_ACTION_QUICK_FIX_PICKER_TRUTH_DOC_REF.to_owned()
}

fn fixture_ref() -> String {
    CODE_ACTION_QUICK_FIX_PICKER_TRUTH_FIXTURE_DIR.to_owned()
}

/// Per-lane posture used to seed a fully covered, stable lane.
struct LaneSpec {
    lane: ArtifactFamilyLaneClass,
    prefix: &'static str,
    provider: ProviderFamilyClass,
    posture: ApplyPostureClass,
    scope: MutationScopeClass,
    hook: ValidationHookClass,
    preview: CompletenessClass,
    needs_preview_hash: bool,
    needs_checkpoint: bool,
    generated: GeneratedArtifactPolicyClass,
    fallback: FallbackPathClass,
    disagreement: DisagreementVisibilityClass,
    rollback: RollbackPathClass,
}

fn lane_specs() -> Vec<LaneSpec> {
    vec![
        LaneSpec {
            lane: ArtifactFamilyLaneClass::FrameworkPackLane,
            prefix: "framework",
            provider: ProviderFamilyClass::FrameworkAnalyzer,
            posture: ApplyPostureClass::PreviewRequired,
            scope: MutationScopeClass::MultiFileScope,
            hook: ValidationHookClass::BuildCheck,
            preview: CompletenessClass::Complete,
            needs_preview_hash: true,
            needs_checkpoint: true,
            generated: GeneratedArtifactPolicyClass::NotGenerated,
            fallback: FallbackPathClass::ManualFixGuidance,
            disagreement: DisagreementVisibilityClass::WinnerLoserBothInspectable,
            rollback: RollbackPathClass::GroupedMutationJournalRevert,
        },
        LaneSpec {
            lane: ArtifactFamilyLaneClass::NotebookCellLane,
            prefix: "notebook",
            provider: ProviderFamilyClass::NotebookAdapter,
            posture: ApplyPostureClass::PreviewRequired,
            scope: MutationScopeClass::CrossArtifactScope,
            hook: ValidationHookClass::TestSuite,
            preview: CompletenessClass::Partial,
            needs_preview_hash: true,
            needs_checkpoint: true,
            generated: GeneratedArtifactPolicyClass::RegenerateBeforeEdit,
            fallback: FallbackPathClass::RegenerateFirstGuidance,
            disagreement: DisagreementVisibilityClass::SingleProviderNoDisagreement,
            rollback: RollbackPathClass::CompensatingRevertViaWorkspaceDiff,
        },
        LaneSpec {
            lane: ArtifactFamilyLaneClass::DocsArtifactLane,
            prefix: "docs",
            provider: ProviderFamilyClass::TextFallback,
            posture: ApplyPostureClass::InlineSafe,
            scope: MutationScopeClass::SingleFileScope,
            hook: ValidationHookClass::LintFormat,
            preview: CompletenessClass::NotApplicable,
            needs_preview_hash: false,
            needs_checkpoint: true,
            generated: GeneratedArtifactPolicyClass::NotGenerated,
            fallback: FallbackPathClass::ManualFixGuidance,
            disagreement: DisagreementVisibilityClass::SingleProviderNoDisagreement,
            rollback: RollbackPathClass::ExactUndoViaLocalHistoryCheckpoint,
        },
        LaneSpec {
            lane: ArtifactFamilyLaneClass::RequestArtifactLane,
            prefix: "request",
            provider: ProviderFamilyClass::LspProvider,
            posture: ApplyPostureClass::PreviewRequired,
            scope: MutationScopeClass::StructuredArtifactScope,
            hook: ValidationHookClass::SchemaValidate,
            preview: CompletenessClass::Complete,
            needs_preview_hash: true,
            needs_checkpoint: true,
            generated: GeneratedArtifactPolicyClass::NotGenerated,
            fallback: FallbackPathClass::ManualFixGuidance,
            disagreement: DisagreementVisibilityClass::PolicyOverrideRecorded,
            rollback: RollbackPathClass::GroupedMutationJournalRevert,
        },
        LaneSpec {
            lane: ArtifactFamilyLaneClass::ConfigArtifactLane,
            prefix: "config",
            provider: ProviderFamilyClass::FrameworkAnalyzer,
            posture: ApplyPostureClass::PreviewRequired,
            scope: MutationScopeClass::MultiFileScope,
            hook: ValidationHookClass::SchemaValidate,
            preview: CompletenessClass::Complete,
            needs_preview_hash: true,
            needs_checkpoint: true,
            generated: GeneratedArtifactPolicyClass::NotGenerated,
            fallback: FallbackPathClass::ManualFixGuidance,
            disagreement: DisagreementVisibilityClass::WinnerLoserBothInspectable,
            rollback: RollbackPathClass::ExactUndoViaLocalHistoryCheckpoint,
        },
        LaneSpec {
            lane: ArtifactFamilyLaneClass::GeneratedSourceLane,
            prefix: "generated",
            provider: ProviderFamilyClass::GeneratedSourceBridge,
            posture: ApplyPostureClass::BlockedPendingReview,
            scope: MutationScopeClass::GeneratedArtifactScope,
            hook: ValidationHookClass::ManualReviewOnly,
            preview: CompletenessClass::NotApplicable,
            needs_preview_hash: false,
            needs_checkpoint: false,
            generated: GeneratedArtifactPolicyClass::EditBlockedGeneratedSource,
            fallback: FallbackPathClass::RegenerateFirstGuidance,
            disagreement: DisagreementVisibilityClass::UnresolvedSurfaced,
            rollback: RollbackPathClass::RegenerateFirstThenReplay,
        },
    ]
}

fn base_row(row_id: &str, lane: ArtifactFamilyLaneClass, row_class: PickerRowClass) -> PickerRow {
    PickerRow {
        row_id: row_id.to_owned(),
        lane_class: lane,
        row_class,
        support_class: SupportClass::Certified,
        acting_provider_class: ProviderFamilyClass::NotApplicable,
        apply_posture_class: ApplyPostureClass::NotApplicable,
        mutation_scope_class: MutationScopeClass::NotApplicable,
        validation_hook_class: ValidationHookClass::NotApplicable,
        generated_asset_policy_class: GeneratedArtifactPolicyClass::NotApplicable,
        fallback_path_class: FallbackPathClass::NotApplicable,
        disagreement_visibility_class: DisagreementVisibilityClass::NotApplicable,
        rollback_checkpoint_class: RollbackPathClass::NotApplicable,
        preview_completeness_class: CompletenessClass::NotApplicable,
        evidence_class: EvidenceClass::FixtureRepoEvidence,
        known_limit_class: KnownLimitClass::NoneDeclared,
        downgrade_automation_class: DowngradeAutomationClass::AutoNarrowOnMissingFixture,
        confidence_class: ConfidenceClass::HighConfidence,
        evidence_refs: vec![fixture_ref()],
        disclosure_ref: Some(format!("{}#auto_narrow_on_missing_fixture", doc_ref())),
        acting_provider_label: None,
        preview_hash_ref: None,
        checkpoint_ref: None,
        raw_source_material_excluded: true,
        secrets_excluded: true,
        ambient_authority_excluded: true,
        captured_at: "2026-06-14T12:00:00Z".to_owned(),
    }
}

fn lane_rows(spec: &LaneSpec) -> Vec<PickerRow> {
    let mut rows = Vec::new();

    let mut quality = base_row(
        &format!("row:{}:quality", spec.prefix),
        spec.lane,
        PickerRowClass::PickerLaneQuality,
    );
    quality.acting_provider_class = spec.provider;
    quality.acting_provider_label = Some(format!("{} acting provider", spec.prefix));
    quality.evidence_class = EvidenceClass::ArchetypeRepoEvidence;
    quality.downgrade_automation_class = DowngradeAutomationClass::AutoBlockOnMissingEvidence;
    quality.disclosure_ref = Some(format!("{}#auto_block_on_missing_evidence", doc_ref()));
    quality.evidence_refs = vec![doc_ref(), fixture_ref()];
    rows.push(quality);

    let mut apply = base_row(
        &format!("row:{}:apply_posture", spec.prefix),
        spec.lane,
        PickerRowClass::ApplyPostureAdmission,
    );
    apply.apply_posture_class = spec.posture;
    apply.mutation_scope_class = spec.scope;
    apply.validation_hook_class = spec.hook;
    apply.preview_completeness_class = spec.preview;
    apply.evidence_class = EvidenceClass::ConformanceSuiteEvidence;
    if spec.needs_preview_hash {
        apply.preview_hash_ref = Some(format!("preview-hash:{}:01", spec.prefix));
    }
    if spec.needs_checkpoint {
        apply.checkpoint_ref = Some(format!("checkpoint:{}:01", spec.prefix));
    }
    rows.push(apply);

    let mut generated = base_row(
        &format!("row:{}:generated_policy", spec.prefix),
        spec.lane,
        PickerRowClass::GeneratedAssetPolicyAdmission,
    );
    generated.generated_asset_policy_class = spec.generated;
    rows.push(generated);

    let mut fallback = base_row(
        &format!("row:{}:fallback", spec.prefix),
        spec.lane,
        PickerRowClass::FallbackPathAdmission,
    );
    fallback.fallback_path_class = spec.fallback;
    rows.push(fallback);

    let mut disagreement = base_row(
        &format!("row:{}:disagreement", spec.prefix),
        spec.lane,
        PickerRowClass::ProviderDisagreementAdmission,
    );
    disagreement.disagreement_visibility_class = spec.disagreement;
    rows.push(disagreement);

    let mut rollback = base_row(
        &format!("row:{}:rollback", spec.prefix),
        spec.lane,
        PickerRowClass::RollbackCheckpointAdmission,
    );
    rollback.rollback_checkpoint_class = spec.rollback;
    rows.push(rollback);

    rows
}

fn projection(surface: ConsumerSurface, packet_id: &str) -> PickerConsumerProjection {
    PickerConsumerProjection {
        consumer_surface: surface,
        projection_ref: format!("projection:{}", surface.as_str()),
        picker_packet_id_ref: packet_id.to_owned(),
        rendered_at: "2026-06-14T12:00:01Z".to_owned(),
        preserves_same_packet: true,
        preserves_lane_vocabulary: true,
        preserves_row_class_vocabulary: true,
        preserves_support_class_vocabulary: true,
        preserves_acting_provider_vocabulary: true,
        preserves_apply_posture_vocabulary: true,
        preserves_mutation_scope_vocabulary: true,
        preserves_validation_hook_vocabulary: true,
        preserves_generated_asset_policy_vocabulary: true,
        preserves_fallback_path_vocabulary: true,
        preserves_disagreement_visibility_vocabulary: true,
        preserves_rollback_checkpoint_vocabulary: true,
        preserves_preview_completeness_vocabulary: true,
        preserves_known_limit_vocabulary: true,
        preserves_downgrade_automation_vocabulary: true,
        preserves_evidence_class_vocabulary: true,
        supports_json_export: true,
        raw_private_material_excluded: true,
        ambient_authority_excluded: true,
    }
}

fn sample_input() -> CodeActionQuickFixPickerTruthPacketInput {
    let packet_id = "packet:m5:code_action_quick_fix_picker";
    let mut rows = Vec::new();
    for spec in lane_specs() {
        rows.extend(lane_rows(&spec));
    }
    let projections = ConsumerSurface::REQUIRED
        .into_iter()
        .map(|surface| projection(surface, packet_id))
        .collect();
    CodeActionQuickFixPickerTruthPacketInput {
        packet_id: packet_id.to_owned(),
        workflow_or_surface_id: "workflow.language.code_action_quick_fix_picker".to_owned(),
        generated_at: "2026-06-14T12:00:00Z".to_owned(),
        covered_lanes: ArtifactFamilyLaneClass::REQUIRED.to_vec(),
        rows,
        consumer_projections: projections,
        source_contract_refs: vec![doc_ref()],
    }
}

#[test]
fn closed_tokens_are_pinned() {
    assert_eq!(
        ArtifactFamilyLaneClass::FrameworkPackLane.as_str(),
        "framework_pack_lane"
    );
    assert_eq!(
        ArtifactFamilyLaneClass::GeneratedSourceLane.as_str(),
        "generated_source_lane"
    );
    assert_eq!(
        PickerRowClass::PickerLaneQuality.as_str(),
        "picker_lane_quality"
    );
    assert_eq!(
        PickerRowClass::ApplyPostureAdmission.as_str(),
        "apply_posture_admission"
    );
    assert_eq!(ApplyPostureClass::InlineSafe.as_str(), "inline_safe");
    assert_eq!(
        ApplyPostureClass::PreviewRequired.as_str(),
        "preview_required"
    );
    assert_eq!(ApplyPostureClass::CompareOnly.as_str(), "compare_only");
    assert_eq!(
        ApplyPostureClass::BlockedPendingReview.as_str(),
        "blocked_pending_review"
    );
    assert_eq!(
        MutationScopeClass::GeneratedArtifactScope.as_str(),
        "generated_artifact_scope"
    );
    assert_eq!(
        MutationScopeClass::StructuredArtifactScope.as_str(),
        "structured_artifact_scope"
    );
    assert_eq!(
        ValidationHookClass::SchemaValidate.as_str(),
        "schema_validate"
    );
    assert_eq!(
        FallbackPathClass::ManualFixGuidance.as_str(),
        "manual_fix_guidance"
    );
    assert_eq!(
        DisagreementVisibilityClass::RankingOnlyCollapsed.as_str(),
        "ranking_only_collapsed"
    );
    assert_eq!(
        FindingKind::InlineApplyWidensScopeWithoutPreview.as_str(),
        "inline_apply_widens_scope_without_preview"
    );
    assert_eq!(
        FindingKind::MissingCheckpointRef.as_str(),
        "missing_checkpoint_ref"
    );
    assert_eq!(
        FindingKind::DisagreementCollapsedToRankingOnly.as_str(),
        "disagreement_collapsed_to_ranking_only"
    );
    assert_eq!(
        FindingKind::ManualFixGuidanceHidden.as_str(),
        "manual_fix_guidance_hidden"
    );
}

#[test]
fn baseline_materialization_is_stable() {
    let packet = CodeActionQuickFixPickerTruthPacket::materialize(sample_input());
    assert_eq!(
        packet.promotion_state,
        PromotionState::Stable,
        "expected stable but got findings: {:?}",
        packet
            .validation_findings
            .iter()
            .map(|f| f.finding_kind.as_str())
            .collect::<Vec<_>>()
    );
    assert!(packet.validation_findings.is_empty());
    assert!(packet.is_stable());
    assert!(packet
        .support_export(
            "support:m5:code_action_quick_fix_picker",
            "2026-06-14T12:00:10Z"
        )
        .is_export_safe());
}

#[test]
fn certified_with_unbound_evidence_blocks() {
    let mut input = sample_input();
    input.rows[0].evidence_class = EvidenceClass::EvidenceUnbound;
    let packet = CodeActionQuickFixPickerTruthPacket::materialize(input);
    assert_eq!(packet.promotion_state, PromotionState::BlocksStable);
    assert!(packet
        .validation_findings
        .iter()
        .any(|finding| finding.finding_kind == FindingKind::MissingEvidenceClass));
    assert!(packet
        .validation_findings
        .iter()
        .any(|finding| finding.finding_kind == FindingKind::CertifiedWithUnboundBinding));
}

#[test]
fn missing_apply_posture_admission_for_certified_lane_blocks() {
    let mut input = sample_input();
    input.rows.retain(|row| {
        !(row.lane_class == ArtifactFamilyLaneClass::FrameworkPackLane
            && row.row_class == PickerRowClass::ApplyPostureAdmission)
    });
    let packet = CodeActionQuickFixPickerTruthPacket::materialize(input);
    assert_eq!(packet.promotion_state, PromotionState::BlocksStable);
    assert!(packet
        .validation_findings
        .iter()
        .any(|finding| finding.finding_kind == FindingKind::MissingApplyPostureCoverage));
}

#[test]
fn inline_apply_widening_scope_without_preview_blocks() {
    let mut input = sample_input();
    for row in &mut input.rows {
        if row.lane_class == ArtifactFamilyLaneClass::FrameworkPackLane
            && row.row_class == PickerRowClass::ApplyPostureAdmission
        {
            row.apply_posture_class = ApplyPostureClass::InlineSafe;
            row.mutation_scope_class = MutationScopeClass::GeneratedArtifactScope;
        }
    }
    let packet = CodeActionQuickFixPickerTruthPacket::materialize(input);
    assert_eq!(packet.promotion_state, PromotionState::BlocksStable);
    assert!(packet
        .validation_findings
        .iter()
        .any(|finding| finding.finding_kind == FindingKind::InlineApplyWidensScopeWithoutPreview));
}

#[test]
fn preview_required_without_preview_hash_blocks() {
    let mut input = sample_input();
    for row in &mut input.rows {
        if row.lane_class == ArtifactFamilyLaneClass::FrameworkPackLane
            && row.row_class == PickerRowClass::ApplyPostureAdmission
        {
            row.preview_hash_ref = None;
        }
    }
    let packet = CodeActionQuickFixPickerTruthPacket::materialize(input);
    assert_eq!(packet.promotion_state, PromotionState::BlocksStable);
    assert!(packet
        .validation_findings
        .iter()
        .any(|finding| finding.finding_kind == FindingKind::MissingPreviewHashRef));
}

#[test]
fn mutating_apply_without_checkpoint_blocks() {
    let mut input = sample_input();
    for row in &mut input.rows {
        if row.lane_class == ArtifactFamilyLaneClass::FrameworkPackLane
            && row.row_class == PickerRowClass::ApplyPostureAdmission
        {
            row.checkpoint_ref = None;
        }
    }
    let packet = CodeActionQuickFixPickerTruthPacket::materialize(input);
    assert_eq!(packet.promotion_state, PromotionState::BlocksStable);
    assert!(packet
        .validation_findings
        .iter()
        .any(|finding| finding.finding_kind == FindingKind::MissingCheckpointRef));
}

#[test]
fn missing_acting_provider_label_blocks() {
    let mut input = sample_input();
    input.rows[0].acting_provider_label = None;
    let packet = CodeActionQuickFixPickerTruthPacket::materialize(input);
    assert_eq!(packet.promotion_state, PromotionState::BlocksStable);
    assert!(packet
        .validation_findings
        .iter()
        .any(|finding| finding.finding_kind == FindingKind::MissingActingProviderLabel));
}

#[test]
fn disagreement_collapsed_to_ranking_only_blocks() {
    let mut input = sample_input();
    for row in &mut input.rows {
        if row.lane_class == ArtifactFamilyLaneClass::FrameworkPackLane
            && row.row_class == PickerRowClass::ProviderDisagreementAdmission
        {
            row.disagreement_visibility_class = DisagreementVisibilityClass::RankingOnlyCollapsed;
        }
    }
    let packet = CodeActionQuickFixPickerTruthPacket::materialize(input);
    assert_eq!(packet.promotion_state, PromotionState::BlocksStable);
    assert!(packet
        .validation_findings
        .iter()
        .any(|finding| finding.finding_kind == FindingKind::DisagreementCollapsedToRankingOnly));
}

#[test]
fn hidden_manual_fix_guidance_on_low_confidence_blocks() {
    let mut input = sample_input();
    for row in &mut input.rows {
        if row.lane_class == ArtifactFamilyLaneClass::FrameworkPackLane
            && row.row_class == PickerRowClass::FallbackPathAdmission
        {
            row.support_class = SupportClass::CertifiedBelow;
            row.confidence_class = ConfidenceClass::LowConfidence;
            row.fallback_path_class = FallbackPathClass::NoneNeeded;
            row.disclosure_ref = Some(format!("{}#auto_narrow_on_missing_fixture", doc_ref()));
        }
    }
    let packet = CodeActionQuickFixPickerTruthPacket::materialize(input);
    assert_eq!(packet.promotion_state, PromotionState::BlocksStable);
    assert!(packet
        .validation_findings
        .iter()
        .any(|finding| finding.finding_kind == FindingKind::ManualFixGuidanceHidden));
}

#[test]
fn dimension_bound_on_wrong_row_class_blocks() {
    let mut input = sample_input();
    for row in &mut input.rows {
        if row.lane_class == ArtifactFamilyLaneClass::FrameworkPackLane
            && row.row_class == PickerRowClass::FallbackPathAdmission
        {
            row.apply_posture_class = ApplyPostureClass::InlineSafe;
        }
    }
    let packet = CodeActionQuickFixPickerTruthPacket::materialize(input);
    assert_eq!(packet.promotion_state, PromotionState::BlocksStable);
    assert!(packet
        .validation_findings
        .iter()
        .any(|finding| finding.finding_kind == FindingKind::ApplyPostureNotPermittedOnRowClass));
}

#[test]
fn projection_drop_blocks_promotion() {
    let mut input = sample_input();
    input
        .consumer_projections
        .retain(|p| p.consumer_surface != ConsumerSurface::NotebookSurface);
    let packet = CodeActionQuickFixPickerTruthPacket::materialize(input);
    assert_eq!(packet.promotion_state, PromotionState::BlocksStable);
    assert!(packet
        .validation_findings
        .iter()
        .any(|finding| finding.finding_kind == FindingKind::MissingConsumerProjection));
}

#[test]
fn collapsed_apply_posture_vocabulary_blocks() {
    let mut input = sample_input();
    for projection in &mut input.consumer_projections {
        if projection.consumer_surface == ConsumerSurface::HelpAbout {
            projection.preserves_apply_posture_vocabulary = false;
        }
    }
    let packet = CodeActionQuickFixPickerTruthPacket::materialize(input);
    assert_eq!(packet.promotion_state, PromotionState::BlocksStable);
    assert!(packet
        .validation_findings
        .iter()
        .any(|finding| finding.finding_kind == FindingKind::ApplyPostureVocabularyCollapsed));
}

#[test]
fn raw_source_material_blocks_promotion() {
    let mut input = sample_input();
    input.rows[0].raw_source_material_excluded = false;
    let packet = CodeActionQuickFixPickerTruthPacket::materialize(input);
    assert_eq!(packet.promotion_state, PromotionState::BlocksStable);
    assert!(packet
        .validation_findings
        .iter()
        .any(|finding| finding.finding_kind == FindingKind::RawSourceMaterialPresent));
}
