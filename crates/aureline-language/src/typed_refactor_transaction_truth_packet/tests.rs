use super::*;

fn doc_ref() -> String {
    TYPED_REFACTOR_TRANSACTION_TRUTH_DOC_REF.to_owned()
}

fn fixture_ref() -> String {
    TYPED_REFACTOR_TRANSACTION_TRUTH_FIXTURE_DIR.to_owned()
}

/// Per-lane posture used to seed a fully covered, stable transaction lane.
struct LaneSpec {
    lane: ArtifactFamilyLaneClass,
    prefix: &'static str,
    provider: ProviderFamilyClass,
    refactor: RefactorTransactionClass,
    target_scope: MutationScopeClass,
    missing_scope: u32,
    completeness: CompletenessClass,
    hunks: u32,
    plan: ValidationPlanClass,
    generated: GeneratedArtifactPolicyClass,
    pipeline: ApplyPipelineClass,
    rollback: RollbackPathClass,
    disagreement: DisagreementVisibilityClass,
}

fn lane_specs() -> Vec<LaneSpec> {
    vec![
        LaneSpec {
            lane: ArtifactFamilyLaneClass::FrameworkPackLane,
            prefix: "framework",
            provider: ProviderFamilyClass::FrameworkAnalyzer,
            refactor: RefactorTransactionClass::Move,
            target_scope: MutationScopeClass::MultiFileScope,
            missing_scope: 0,
            completeness: CompletenessClass::Complete,
            hunks: 4,
            plan: ValidationPlanClass::BuildThenTest,
            generated: GeneratedArtifactPolicyClass::NotGenerated,
            pipeline: ApplyPipelineClass::PreviewThenSavePipeline,
            rollback: RollbackPathClass::GroupedMutationJournalRevert,
            disagreement: DisagreementVisibilityClass::WinnerLoserBothInspectable,
        },
        LaneSpec {
            lane: ArtifactFamilyLaneClass::NotebookCellLane,
            prefix: "notebook",
            provider: ProviderFamilyClass::NotebookAdapter,
            refactor: RefactorTransactionClass::NotebookGeneratedEdit,
            target_scope: MutationScopeClass::CrossArtifactScope,
            missing_scope: 2,
            completeness: CompletenessClass::Partial,
            hunks: 3,
            plan: ValidationPlanClass::TestSuitePlan,
            generated: GeneratedArtifactPolicyClass::RegenerateBeforeEdit,
            pipeline: ApplyPipelineClass::PreviewThenSavePipeline,
            rollback: RollbackPathClass::CompensatingRevertViaWorkspaceDiff,
            disagreement: DisagreementVisibilityClass::SingleProviderNoDisagreement,
        },
        LaneSpec {
            lane: ArtifactFamilyLaneClass::DocsArtifactLane,
            prefix: "docs",
            provider: ProviderFamilyClass::TextFallback,
            refactor: RefactorTransactionClass::Rename,
            target_scope: MutationScopeClass::SingleFileScope,
            missing_scope: 0,
            completeness: CompletenessClass::Complete,
            hunks: 1,
            plan: ValidationPlanClass::LintFormatPlan,
            generated: GeneratedArtifactPolicyClass::NotGenerated,
            pipeline: ApplyPipelineClass::SavePipelineWithJournal,
            rollback: RollbackPathClass::ExactUndoViaLocalHistoryCheckpoint,
            disagreement: DisagreementVisibilityClass::SingleProviderNoDisagreement,
        },
        LaneSpec {
            lane: ArtifactFamilyLaneClass::RequestArtifactLane,
            prefix: "request",
            provider: ProviderFamilyClass::LspProvider,
            refactor: RefactorTransactionClass::SchemaCodegenRewrite,
            target_scope: MutationScopeClass::StructuredArtifactScope,
            missing_scope: 0,
            completeness: CompletenessClass::Complete,
            hunks: 2,
            plan: ValidationPlanClass::SchemaValidatePlan,
            generated: GeneratedArtifactPolicyClass::NotGenerated,
            pipeline: ApplyPipelineClass::PreviewThenSavePipeline,
            rollback: RollbackPathClass::GroupedMutationJournalRevert,
            disagreement: DisagreementVisibilityClass::PolicyOverrideRecorded,
        },
        LaneSpec {
            lane: ArtifactFamilyLaneClass::ConfigArtifactLane,
            prefix: "config",
            provider: ProviderFamilyClass::FrameworkAnalyzer,
            refactor: RefactorTransactionClass::OrganizeImports,
            target_scope: MutationScopeClass::MultiFileScope,
            missing_scope: 0,
            completeness: CompletenessClass::Complete,
            hunks: 3,
            plan: ValidationPlanClass::FrameworkCheckPlan,
            generated: GeneratedArtifactPolicyClass::NotGenerated,
            pipeline: ApplyPipelineClass::PreviewThenSavePipeline,
            rollback: RollbackPathClass::ExactUndoViaLocalHistoryCheckpoint,
            disagreement: DisagreementVisibilityClass::WinnerLoserBothInspectable,
        },
        LaneSpec {
            lane: ArtifactFamilyLaneClass::GeneratedSourceLane,
            prefix: "generated",
            provider: ProviderFamilyClass::GeneratedSourceBridge,
            refactor: RefactorTransactionClass::CompareOnlyNoMutation,
            target_scope: MutationScopeClass::GeneratedArtifactScope,
            missing_scope: 0,
            completeness: CompletenessClass::Blocked,
            hunks: 2,
            plan: ValidationPlanClass::ManualReviewPlan,
            generated: GeneratedArtifactPolicyClass::EditBlockedGeneratedSource,
            pipeline: ApplyPipelineClass::BlockedPendingReview,
            rollback: RollbackPathClass::RegenerateFirstThenReplay,
            disagreement: DisagreementVisibilityClass::UnresolvedSurfaced,
        },
    ]
}

fn base_row(
    row_id: &str,
    lane: ArtifactFamilyLaneClass,
    refactor_id: &str,
    row_class: TransactionRowClass,
) -> TransactionRow {
    TransactionRow {
        row_id: row_id.to_owned(),
        lane_class: lane,
        row_class,
        refactor_id: refactor_id.to_owned(),
        support_class: SupportClass::Certified,
        acting_provider_class: ProviderFamilyClass::NotApplicable,
        refactor_class: RefactorTransactionClass::NotApplicable,
        target_scope_class: MutationScopeClass::NotApplicable,
        scope_completeness_class: CompletenessClass::NotApplicable,
        missing_scope_count: 0,
        grouped_hunk_count: 0,
        impact_summary_present: false,
        ownership_hint_present: false,
        validation_plan_class: ValidationPlanClass::NotApplicable,
        generated_asset_policy_class: GeneratedArtifactPolicyClass::NotApplicable,
        apply_pipeline_class: ApplyPipelineClass::NotApplicable,
        reuses_save_pipeline: false,
        reuses_mutation_journal: false,
        source_fidelity_preserved: false,
        privileged_fast_path: false,
        rollback_checkpoint_class: RollbackPathClass::NotApplicable,
        disagreement_visibility_class: DisagreementVisibilityClass::NotApplicable,
        evidence_class: EvidenceClass::FixtureRepoEvidence,
        known_limit_class: KnownLimitClass::NoneDeclared,
        downgrade_automation_class: DowngradeAutomationClass::AutoNarrowOnMissingFixture,
        confidence_class: ConfidenceClass::HighConfidence,
        evidence_refs: vec![fixture_ref()],
        disclosure_ref: Some(format!("{}#auto_narrow_on_missing_fixture", doc_ref())),
        engine_identity_label: None,
        validation_plan_ref: None,
        checkpoint_ref: None,
        raw_source_material_excluded: true,
        secrets_excluded: true,
        ambient_authority_excluded: true,
        captured_at: "2026-06-14T12:00:00Z".to_owned(),
    }
}

fn lane_rows(spec: &LaneSpec) -> Vec<TransactionRow> {
    let refactor_id = format!("refactor:{}:tx", spec.prefix);
    let mut rows = Vec::new();

    let mut quality = base_row(
        &format!("row:{}:quality", spec.prefix),
        spec.lane,
        &refactor_id,
        TransactionRowClass::TransactionLaneQuality,
    );
    quality.acting_provider_class = spec.provider;
    quality.refactor_class = spec.refactor;
    quality.engine_identity_label = Some(format!("{} acting engine", spec.prefix));
    quality.evidence_class = EvidenceClass::ArchetypeRepoEvidence;
    quality.downgrade_automation_class = DowngradeAutomationClass::AutoBlockOnMissingEvidence;
    quality.disclosure_ref = Some(format!("{}#auto_block_on_missing_evidence", doc_ref()));
    quality.evidence_refs = vec![doc_ref(), fixture_ref()];
    rows.push(quality);

    let mut target = base_row(
        &format!("row:{}:target_scope", spec.prefix),
        spec.lane,
        &refactor_id,
        TransactionRowClass::TargetScopeAdmission,
    );
    target.target_scope_class = spec.target_scope;
    target.scope_completeness_class = spec.completeness;
    target.missing_scope_count = spec.missing_scope;
    rows.push(target);

    let mut hunks = base_row(
        &format!("row:{}:grouped_hunks", spec.prefix),
        spec.lane,
        &refactor_id,
        TransactionRowClass::GroupedHunksAdmission,
    );
    hunks.grouped_hunk_count = spec.hunks;
    hunks.impact_summary_present = true;
    hunks.ownership_hint_present = true;
    rows.push(hunks);

    let mut plan = base_row(
        &format!("row:{}:validation_plan", spec.prefix),
        spec.lane,
        &refactor_id,
        TransactionRowClass::ValidationPlanAdmission,
    );
    plan.validation_plan_class = spec.plan;
    plan.validation_plan_ref = Some(format!("validation-plan:{}:01", spec.prefix));
    rows.push(plan);

    let mut generated = base_row(
        &format!("row:{}:generated_policy", spec.prefix),
        spec.lane,
        &refactor_id,
        TransactionRowClass::GeneratedAssetPolicyAdmission,
    );
    generated.generated_asset_policy_class = spec.generated;
    rows.push(generated);

    let mut pipeline = base_row(
        &format!("row:{}:apply_pipeline", spec.prefix),
        spec.lane,
        &refactor_id,
        TransactionRowClass::ApplyPipelineAdmission,
    );
    pipeline.apply_pipeline_class = spec.pipeline;
    pipeline.reuses_save_pipeline = spec.pipeline.applies_mutation();
    pipeline.reuses_mutation_journal = spec.pipeline.applies_mutation();
    pipeline.source_fidelity_preserved = true;
    pipeline.privileged_fast_path = false;
    rows.push(pipeline);

    let mut rollback = base_row(
        &format!("row:{}:rollback", spec.prefix),
        spec.lane,
        &refactor_id,
        TransactionRowClass::RollbackCheckpointAdmission,
    );
    rollback.rollback_checkpoint_class = spec.rollback;
    if rollback_requires_checkpoint_ref(spec.rollback) {
        rollback.checkpoint_ref = Some(format!("checkpoint:{}:01", spec.prefix));
    }
    rows.push(rollback);

    let mut disagreement = base_row(
        &format!("row:{}:disagreement", spec.prefix),
        spec.lane,
        &refactor_id,
        TransactionRowClass::ProviderDisagreementAdmission,
    );
    disagreement.disagreement_visibility_class = spec.disagreement;
    rows.push(disagreement);

    rows
}

fn projection(surface: ConsumerSurface, packet_id: &str) -> TransactionConsumerProjection {
    TransactionConsumerProjection {
        consumer_surface: surface,
        projection_ref: format!("projection:{}", surface.as_str()),
        transaction_packet_id_ref: packet_id.to_owned(),
        rendered_at: "2026-06-14T12:00:01Z".to_owned(),
        preserves_same_packet: true,
        preserves_lane_vocabulary: true,
        preserves_row_class_vocabulary: true,
        preserves_support_class_vocabulary: true,
        preserves_engine_identity_vocabulary: true,
        preserves_refactor_class_vocabulary: true,
        preserves_target_scope_vocabulary: true,
        preserves_scope_completeness_vocabulary: true,
        preserves_validation_plan_vocabulary: true,
        preserves_generated_asset_policy_vocabulary: true,
        preserves_apply_pipeline_vocabulary: true,
        preserves_rollback_checkpoint_vocabulary: true,
        preserves_disagreement_visibility_vocabulary: true,
        preserves_known_limit_vocabulary: true,
        preserves_downgrade_automation_vocabulary: true,
        preserves_evidence_class_vocabulary: true,
        supports_json_export: true,
        raw_private_material_excluded: true,
        ambient_authority_excluded: true,
    }
}

fn sample_input() -> TypedRefactorTransactionTruthPacketInput {
    let packet_id = "packet:m5:typed_refactor_transaction";
    let mut rows = Vec::new();
    for spec in lane_specs() {
        rows.extend(lane_rows(&spec));
    }
    let projections = ConsumerSurface::REQUIRED
        .into_iter()
        .map(|surface| projection(surface, packet_id))
        .collect();
    TypedRefactorTransactionTruthPacketInput {
        packet_id: packet_id.to_owned(),
        workflow_or_surface_id: "workflow.language.typed_refactor_transaction".to_owned(),
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
        TransactionRowClass::TransactionLaneQuality.as_str(),
        "transaction_lane_quality"
    );
    assert_eq!(
        TransactionRowClass::ApplyPipelineAdmission.as_str(),
        "apply_pipeline_admission"
    );
    assert_eq!(
        TransactionRowClass::TargetScopeAdmission.as_str(),
        "target_scope_admission"
    );
    assert_eq!(
        ValidationPlanClass::BuildThenTest.as_str(),
        "build_then_test"
    );
    assert_eq!(
        ValidationPlanClass::ManualReviewPlan.as_str(),
        "manual_review_plan"
    );
    assert_eq!(
        ApplyPipelineClass::SavePipelineWithJournal.as_str(),
        "save_pipeline_with_journal"
    );
    assert_eq!(
        ApplyPipelineClass::BlockedPendingReview.as_str(),
        "blocked_pending_review"
    );
    assert_eq!(
        FindingKind::ScopeCompletenessOverclaimed.as_str(),
        "scope_completeness_overclaimed"
    );
    assert_eq!(
        FindingKind::PrivilegedFastPathNotPermitted.as_str(),
        "privileged_fast_path_not_permitted"
    );
    assert_eq!(
        FindingKind::GeneratedPolicyBypassed.as_str(),
        "generated_policy_bypassed"
    );
    assert_eq!(
        FindingKind::MissingCheckpointRef.as_str(),
        "missing_checkpoint_ref"
    );
}

#[test]
fn baseline_materialization_is_stable() {
    let packet = TypedRefactorTransactionTruthPacket::materialize(sample_input());
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
            "support:m5:typed_refactor_transaction",
            "2026-06-14T12:00:10Z"
        )
        .is_export_safe());
}

#[test]
fn certified_with_unbound_evidence_blocks() {
    let mut input = sample_input();
    input.rows[0].evidence_class = EvidenceClass::EvidenceUnbound;
    let packet = TypedRefactorTransactionTruthPacket::materialize(input);
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
fn missing_target_scope_admission_for_certified_lane_blocks() {
    let mut input = sample_input();
    input.rows.retain(|row| {
        !(row.lane_class == ArtifactFamilyLaneClass::FrameworkPackLane
            && row.row_class == TransactionRowClass::TargetScopeAdmission)
    });
    let packet = TypedRefactorTransactionTruthPacket::materialize(input);
    assert_eq!(packet.promotion_state, PromotionState::BlocksStable);
    assert!(packet
        .validation_findings
        .iter()
        .any(|finding| finding.finding_kind == FindingKind::MissingTargetScopeCoverage));
}

#[test]
fn scope_completeness_overclaimed_blocks() {
    let mut input = sample_input();
    for row in &mut input.rows {
        if row.lane_class == ArtifactFamilyLaneClass::FrameworkPackLane
            && row.row_class == TransactionRowClass::TargetScopeAdmission
        {
            row.scope_completeness_class = CompletenessClass::Complete;
            row.missing_scope_count = 3;
        }
    }
    let packet = TypedRefactorTransactionTruthPacket::materialize(input);
    assert_eq!(packet.promotion_state, PromotionState::BlocksStable);
    assert!(packet
        .validation_findings
        .iter()
        .any(|finding| finding.finding_kind == FindingKind::ScopeCompletenessOverclaimed));
}

#[test]
fn grouped_hunks_without_impact_summary_blocks() {
    let mut input = sample_input();
    for row in &mut input.rows {
        if row.lane_class == ArtifactFamilyLaneClass::FrameworkPackLane
            && row.row_class == TransactionRowClass::GroupedHunksAdmission
        {
            row.impact_summary_present = false;
        }
    }
    let packet = TypedRefactorTransactionTruthPacket::materialize(input);
    assert_eq!(packet.promotion_state, PromotionState::BlocksStable);
    assert!(packet
        .validation_findings
        .iter()
        .any(|finding| finding.finding_kind == FindingKind::MissingImpactSummary));
}

#[test]
fn validation_plan_without_plan_ref_blocks() {
    let mut input = sample_input();
    for row in &mut input.rows {
        if row.lane_class == ArtifactFamilyLaneClass::FrameworkPackLane
            && row.row_class == TransactionRowClass::ValidationPlanAdmission
        {
            row.validation_plan_ref = None;
        }
    }
    let packet = TypedRefactorTransactionTruthPacket::materialize(input);
    assert_eq!(packet.promotion_state, PromotionState::BlocksStable);
    assert!(packet
        .validation_findings
        .iter()
        .any(|finding| finding.finding_kind == FindingKind::MissingValidationPlanRef));
}

#[test]
fn apply_pipeline_bypassing_save_pipeline_blocks() {
    let mut input = sample_input();
    for row in &mut input.rows {
        if row.lane_class == ArtifactFamilyLaneClass::FrameworkPackLane
            && row.row_class == TransactionRowClass::ApplyPipelineAdmission
        {
            row.reuses_save_pipeline = false;
        }
    }
    let packet = TypedRefactorTransactionTruthPacket::materialize(input);
    assert_eq!(packet.promotion_state, PromotionState::BlocksStable);
    assert!(packet
        .validation_findings
        .iter()
        .any(|finding| finding.finding_kind == FindingKind::ApplyPipelineBypassesSavePipeline));
}

#[test]
fn privileged_fast_path_blocks() {
    let mut input = sample_input();
    for row in &mut input.rows {
        if row.lane_class == ArtifactFamilyLaneClass::FrameworkPackLane
            && row.row_class == TransactionRowClass::ApplyPipelineAdmission
        {
            row.privileged_fast_path = true;
        }
    }
    let packet = TypedRefactorTransactionTruthPacket::materialize(input);
    assert_eq!(packet.promotion_state, PromotionState::BlocksStable);
    assert!(packet
        .validation_findings
        .iter()
        .any(|finding| finding.finding_kind == FindingKind::PrivilegedFastPathNotPermitted));
}

#[test]
fn source_fidelity_bypass_blocks() {
    let mut input = sample_input();
    for row in &mut input.rows {
        if row.lane_class == ArtifactFamilyLaneClass::FrameworkPackLane
            && row.row_class == TransactionRowClass::ApplyPipelineAdmission
        {
            row.source_fidelity_preserved = false;
        }
    }
    let packet = TypedRefactorTransactionTruthPacket::materialize(input);
    assert_eq!(packet.promotion_state, PromotionState::BlocksStable);
    assert!(packet
        .validation_findings
        .iter()
        .any(|finding| finding.finding_kind == FindingKind::SourceFidelityBypassed));
}

#[test]
fn mutating_transaction_without_checkpoint_blocks() {
    let mut input = sample_input();
    for row in &mut input.rows {
        if row.lane_class == ArtifactFamilyLaneClass::FrameworkPackLane
            && row.row_class == TransactionRowClass::RollbackCheckpointAdmission
        {
            row.checkpoint_ref = None;
        }
    }
    let packet = TypedRefactorTransactionTruthPacket::materialize(input);
    assert_eq!(packet.promotion_state, PromotionState::BlocksStable);
    assert!(packet
        .validation_findings
        .iter()
        .any(|finding| finding.finding_kind == FindingKind::MissingCheckpointRef));
}

#[test]
fn generated_source_treated_as_text_blocks() {
    let mut input = sample_input();
    for row in &mut input.rows {
        if row.lane_class == ArtifactFamilyLaneClass::GeneratedSourceLane
            && row.row_class == TransactionRowClass::GeneratedAssetPolicyAdmission
        {
            row.generated_asset_policy_class = GeneratedArtifactPolicyClass::NotGenerated;
        }
    }
    let packet = TypedRefactorTransactionTruthPacket::materialize(input);
    assert_eq!(packet.promotion_state, PromotionState::BlocksStable);
    assert!(packet
        .validation_findings
        .iter()
        .any(|finding| finding.finding_kind == FindingKind::GeneratedPolicyBypassed));
}

#[test]
fn disagreement_collapsed_to_ranking_only_blocks() {
    let mut input = sample_input();
    for row in &mut input.rows {
        if row.lane_class == ArtifactFamilyLaneClass::FrameworkPackLane
            && row.row_class == TransactionRowClass::ProviderDisagreementAdmission
        {
            row.disagreement_visibility_class = DisagreementVisibilityClass::RankingOnlyCollapsed;
        }
    }
    let packet = TypedRefactorTransactionTruthPacket::materialize(input);
    assert_eq!(packet.promotion_state, PromotionState::BlocksStable);
    assert!(packet
        .validation_findings
        .iter()
        .any(|finding| finding.finding_kind == FindingKind::DisagreementCollapsedToRankingOnly));
}

#[test]
fn missing_engine_identity_label_blocks() {
    let mut input = sample_input();
    input.rows[0].engine_identity_label = None;
    let packet = TypedRefactorTransactionTruthPacket::materialize(input);
    assert_eq!(packet.promotion_state, PromotionState::BlocksStable);
    assert!(packet
        .validation_findings
        .iter()
        .any(|finding| finding.finding_kind == FindingKind::MissingEngineIdentityLabel));
}

#[test]
fn dimension_bound_on_wrong_row_class_blocks() {
    let mut input = sample_input();
    for row in &mut input.rows {
        if row.lane_class == ArtifactFamilyLaneClass::FrameworkPackLane
            && row.row_class == TransactionRowClass::GroupedHunksAdmission
        {
            row.apply_pipeline_class = ApplyPipelineClass::SavePipelineWithJournal;
        }
    }
    let packet = TypedRefactorTransactionTruthPacket::materialize(input);
    assert_eq!(packet.promotion_state, PromotionState::BlocksStable);
    assert!(packet
        .validation_findings
        .iter()
        .any(|finding| finding.finding_kind == FindingKind::ApplyPipelineNotPermittedOnRowClass));
}

#[test]
fn missing_refactor_id_blocks() {
    let mut input = sample_input();
    input.rows[0].refactor_id = String::new();
    let packet = TypedRefactorTransactionTruthPacket::materialize(input);
    assert_eq!(packet.promotion_state, PromotionState::BlocksStable);
    assert!(packet
        .validation_findings
        .iter()
        .any(|finding| finding.finding_kind == FindingKind::MissingRefactorId));
}

#[test]
fn projection_drop_blocks_promotion() {
    let mut input = sample_input();
    input
        .consumer_projections
        .retain(|p| p.consumer_surface != ConsumerSurface::NotebookSurface);
    let packet = TypedRefactorTransactionTruthPacket::materialize(input);
    assert_eq!(packet.promotion_state, PromotionState::BlocksStable);
    assert!(packet
        .validation_findings
        .iter()
        .any(|finding| finding.finding_kind == FindingKind::MissingConsumerProjection));
}

#[test]
fn collapsed_target_scope_vocabulary_blocks() {
    let mut input = sample_input();
    for projection in &mut input.consumer_projections {
        if projection.consumer_surface == ConsumerSurface::HelpAbout {
            projection.preserves_target_scope_vocabulary = false;
        }
    }
    let packet = TypedRefactorTransactionTruthPacket::materialize(input);
    assert_eq!(packet.promotion_state, PromotionState::BlocksStable);
    assert!(packet
        .validation_findings
        .iter()
        .any(|finding| finding.finding_kind == FindingKind::TargetScopeVocabularyCollapsed));
}

#[test]
fn raw_source_material_blocks_promotion() {
    let mut input = sample_input();
    input.rows[0].raw_source_material_excluded = false;
    let packet = TypedRefactorTransactionTruthPacket::materialize(input);
    assert_eq!(packet.promotion_state, PromotionState::BlocksStable);
    assert!(packet
        .validation_findings
        .iter()
        .any(|finding| finding.finding_kind == FindingKind::RawSourceMaterialPresent));
}
