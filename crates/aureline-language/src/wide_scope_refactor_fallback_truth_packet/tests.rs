use super::*;

fn doc_ref() -> String {
    WIDE_SCOPE_REFACTOR_FALLBACK_TRUTH_DOC_REF.to_owned()
}

fn fixture_ref() -> String {
    WIDE_SCOPE_REFACTOR_FALLBACK_TRUTH_FIXTURE_DIR.to_owned()
}

/// Per-lane posture used to seed a fully covered, stable fallback lane.
struct LaneSpec {
    lane: ArtifactFamilyLaneClass,
    prefix: &'static str,
    provider: ProviderFamilyClass,
    refactor: RefactorTransactionClass,
    posture: ApplyFallbackPostureClass,
    target_scope: MutationScopeClass,
    completeness: CompletenessClass,
    confidence: ConfidenceClass,
    missing_scope: u32,
    impacted_targets: u32,
    impacted_owners: u32,
    reviewer: ReviewerHintClass,
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
            posture: ApplyFallbackPostureClass::SideBranchApply,
            target_scope: MutationScopeClass::MultiFileScope,
            completeness: CompletenessClass::Complete,
            confidence: ConfidenceClass::HighConfidence,
            missing_scope: 0,
            impacted_targets: 6,
            impacted_owners: 2,
            reviewer: ReviewerHintClass::CodeownersReviewer,
            rollback: RollbackPathClass::GroupedMutationJournalRevert,
            disagreement: DisagreementVisibilityClass::WinnerLoserBothInspectable,
        },
        LaneSpec {
            lane: ArtifactFamilyLaneClass::NotebookCellLane,
            prefix: "notebook",
            provider: ProviderFamilyClass::NotebookAdapter,
            refactor: RefactorTransactionClass::NotebookGeneratedEdit,
            posture: ApplyFallbackPostureClass::StagedApply,
            target_scope: MutationScopeClass::CrossArtifactScope,
            completeness: CompletenessClass::Partial,
            confidence: ConfidenceClass::MediumConfidence,
            missing_scope: 2,
            impacted_targets: 4,
            impacted_owners: 1,
            reviewer: ReviewerHintClass::RecentAuthorReviewer,
            rollback: RollbackPathClass::CompensatingRevertViaWorkspaceDiff,
            disagreement: DisagreementVisibilityClass::SingleProviderNoDisagreement,
        },
        LaneSpec {
            lane: ArtifactFamilyLaneClass::DocsArtifactLane,
            prefix: "docs",
            provider: ProviderFamilyClass::TextFallback,
            refactor: RefactorTransactionClass::Rename,
            posture: ApplyFallbackPostureClass::ApplyAllOnLiveWorkspace,
            target_scope: MutationScopeClass::SingleFileScope,
            completeness: CompletenessClass::Complete,
            confidence: ConfidenceClass::HighConfidence,
            missing_scope: 0,
            impacted_targets: 1,
            impacted_owners: 1,
            reviewer: ReviewerHintClass::NoReviewerRequired,
            rollback: RollbackPathClass::ExactUndoViaLocalHistoryCheckpoint,
            disagreement: DisagreementVisibilityClass::SingleProviderNoDisagreement,
        },
        LaneSpec {
            lane: ArtifactFamilyLaneClass::RequestArtifactLane,
            prefix: "request",
            provider: ProviderFamilyClass::LspProvider,
            refactor: RefactorTransactionClass::SchemaCodegenRewrite,
            posture: ApplyFallbackPostureClass::WorktreeApply,
            target_scope: MutationScopeClass::StructuredArtifactScope,
            completeness: CompletenessClass::Complete,
            confidence: ConfidenceClass::HighConfidence,
            missing_scope: 0,
            impacted_targets: 3,
            impacted_owners: 2,
            reviewer: ReviewerHintClass::OwningTeamReviewer,
            rollback: RollbackPathClass::GroupedMutationJournalRevert,
            disagreement: DisagreementVisibilityClass::PolicyOverrideRecorded,
        },
        LaneSpec {
            lane: ArtifactFamilyLaneClass::ConfigArtifactLane,
            prefix: "config",
            provider: ProviderFamilyClass::FrameworkAnalyzer,
            refactor: RefactorTransactionClass::OrganizeImports,
            posture: ApplyFallbackPostureClass::SideBranchApply,
            target_scope: MutationScopeClass::MultiFileScope,
            completeness: CompletenessClass::Complete,
            confidence: ConfidenceClass::MediumConfidence,
            missing_scope: 0,
            impacted_targets: 3,
            impacted_owners: 1,
            reviewer: ReviewerHintClass::CodeownersReviewer,
            rollback: RollbackPathClass::ExactUndoViaLocalHistoryCheckpoint,
            disagreement: DisagreementVisibilityClass::WinnerLoserBothInspectable,
        },
        LaneSpec {
            lane: ArtifactFamilyLaneClass::GeneratedSourceLane,
            prefix: "generated",
            provider: ProviderFamilyClass::GeneratedSourceBridge,
            refactor: RefactorTransactionClass::CompareOnlyNoMutation,
            posture: ApplyFallbackPostureClass::CompareOnlyReview,
            target_scope: MutationScopeClass::GeneratedArtifactScope,
            completeness: CompletenessClass::Blocked,
            confidence: ConfidenceClass::LowConfidence,
            missing_scope: 0,
            impacted_targets: 2,
            impacted_owners: 1,
            reviewer: ReviewerHintClass::ManualAssignmentRequired,
            rollback: RollbackPathClass::RegenerateFirstThenReplay,
            disagreement: DisagreementVisibilityClass::UnresolvedSurfaced,
        },
    ]
}

fn base_row(
    row_id: &str,
    lane: ArtifactFamilyLaneClass,
    refactor_id: &str,
    row_class: FallbackRowClass,
) -> FallbackRow {
    FallbackRow {
        row_id: row_id.to_owned(),
        lane_class: lane,
        row_class,
        refactor_id: refactor_id.to_owned(),
        support_class: SupportClass::Certified,
        acting_provider_class: ProviderFamilyClass::NotApplicable,
        refactor_class: RefactorTransactionClass::NotApplicable,
        apply_posture_class: ApplyFallbackPostureClass::NotApplicable,
        target_scope_class: MutationScopeClass::NotApplicable,
        scope_completeness_class: CompletenessClass::NotApplicable,
        confidence_class: ConfidenceClass::HighConfidence,
        missing_scope_count: 0,
        impacted_target_count: 0,
        impacted_owner_count: 0,
        impact_summary_present: false,
        missing_scope_explanation_present: false,
        reviewer_hint_class: ReviewerHintClass::NotApplicable,
        owner_hint_present: false,
        rollback_path_class: RollbackPathClass::NotApplicable,
        preserves_refactor_lineage: false,
        preserves_missing_scope_explanation: false,
        disagreement_visibility_class: DisagreementVisibilityClass::NotApplicable,
        evidence_class: EvidenceClass::FixtureRepoEvidence,
        known_limit_class: KnownLimitClass::NoneDeclared,
        downgrade_automation_class: DowngradeAutomationClass::AutoNarrowOnMissingFixture,
        evidence_refs: vec![fixture_ref()],
        disclosure_ref: Some(format!("{}#auto_narrow_on_missing_fixture", doc_ref())),
        engine_identity_label: None,
        impact_packet_ref: None,
        review_anchor_ref: None,
        checkpoint_ref: None,
        lineage_ref: None,
        raw_source_material_excluded: true,
        secrets_excluded: true,
        ambient_authority_excluded: true,
        captured_at: "2026-06-14T12:00:00Z".to_owned(),
    }
}

fn lane_rows(spec: &LaneSpec) -> Vec<FallbackRow> {
    let refactor_id = format!("refactor:{}:tx", spec.prefix);
    let mut rows = Vec::new();

    let mut quality = base_row(
        &format!("row:{}:quality", spec.prefix),
        spec.lane,
        &refactor_id,
        FallbackRowClass::FallbackLaneQuality,
    );
    quality.acting_provider_class = spec.provider;
    quality.refactor_class = spec.refactor;
    quality.engine_identity_label = Some(format!("{} acting engine", spec.prefix));
    quality.evidence_class = EvidenceClass::ArchetypeRepoEvidence;
    quality.downgrade_automation_class = DowngradeAutomationClass::AutoBlockOnMissingEvidence;
    quality.disclosure_ref = Some(format!("{}#auto_block_on_missing_evidence", doc_ref()));
    quality.evidence_refs = vec![doc_ref(), fixture_ref()];
    rows.push(quality);

    let mut posture = base_row(
        &format!("row:{}:apply_posture", spec.prefix),
        spec.lane,
        &refactor_id,
        FallbackRowClass::ApplyPostureAdmission,
    );
    posture.apply_posture_class = spec.posture;
    posture.target_scope_class = spec.target_scope;
    posture.scope_completeness_class = spec.completeness;
    posture.missing_scope_count = spec.missing_scope;
    rows.push(posture);

    let mut impact = base_row(
        &format!("row:{}:impact_packet", spec.prefix),
        spec.lane,
        &refactor_id,
        FallbackRowClass::ImpactPacketAdmission,
    );
    impact.impacted_target_count = spec.impacted_targets;
    impact.impacted_owner_count = spec.impacted_owners;
    impact.impact_summary_present = true;
    impact.missing_scope_explanation_present = spec.missing_scope > 0;
    impact.impact_packet_ref = Some(format!("impact:{}:01", spec.prefix));
    rows.push(impact);

    let mut reviewer = base_row(
        &format!("row:{}:reviewer_hint", spec.prefix),
        spec.lane,
        &refactor_id,
        FallbackRowClass::ReviewerHintAdmission,
    );
    reviewer.reviewer_hint_class = spec.reviewer;
    if spec.reviewer.requires_review_anchor() {
        reviewer.owner_hint_present = true;
        reviewer.review_anchor_ref = Some(format!("review-anchor:{}:01", spec.prefix));
    }
    rows.push(reviewer);

    let mut rollback = base_row(
        &format!("row:{}:rollback", spec.prefix),
        spec.lane,
        &refactor_id,
        FallbackRowClass::RollbackPathAdmission,
    );
    rollback.rollback_path_class = spec.rollback;
    if rollback_requires_checkpoint_ref(spec.rollback) {
        rollback.checkpoint_ref = Some(format!("checkpoint:{}:01", spec.prefix));
    }
    rows.push(rollback);

    let mut parity = base_row(
        &format!("row:{}:support_export_parity", spec.prefix),
        spec.lane,
        &refactor_id,
        FallbackRowClass::SupportExportParityAdmission,
    );
    parity.preserves_refactor_lineage = true;
    parity.preserves_missing_scope_explanation = true;
    parity.lineage_ref = Some(format!("lineage:{}:01", spec.prefix));
    rows.push(parity);

    let mut disagreement = base_row(
        &format!("row:{}:disagreement", spec.prefix),
        spec.lane,
        &refactor_id,
        FallbackRowClass::ProviderDisagreementAdmission,
    );
    disagreement.disagreement_visibility_class = spec.disagreement;
    rows.push(disagreement);

    for row in &mut rows {
        row.confidence_class = spec.confidence;
    }

    rows
}

fn projection(surface: ConsumerSurface, packet_id: &str) -> FallbackConsumerProjection {
    FallbackConsumerProjection {
        consumer_surface: surface,
        projection_ref: format!("projection:{}", surface.as_str()),
        fallback_packet_id_ref: packet_id.to_owned(),
        rendered_at: "2026-06-14T12:00:01Z".to_owned(),
        preserves_same_packet: true,
        preserves_lane_vocabulary: true,
        preserves_row_class_vocabulary: true,
        preserves_support_class_vocabulary: true,
        preserves_engine_identity_vocabulary: true,
        preserves_refactor_class_vocabulary: true,
        preserves_target_scope_vocabulary: true,
        preserves_scope_completeness_vocabulary: true,
        preserves_confidence_vocabulary: true,
        preserves_apply_posture_vocabulary: true,
        preserves_reviewer_hint_vocabulary: true,
        preserves_rollback_path_vocabulary: true,
        preserves_disagreement_visibility_vocabulary: true,
        preserves_known_limit_vocabulary: true,
        preserves_downgrade_automation_vocabulary: true,
        preserves_evidence_class_vocabulary: true,
        supports_json_export: true,
        raw_private_material_excluded: true,
        ambient_authority_excluded: true,
    }
}

fn sample_input() -> WideScopeRefactorFallbackTruthPacketInput {
    let packet_id = "packet:m5:wide_scope_refactor_fallback";
    let mut rows = Vec::new();
    for spec in lane_specs() {
        rows.extend(lane_rows(&spec));
    }
    let projections = ConsumerSurface::REQUIRED
        .into_iter()
        .map(|surface| projection(surface, packet_id))
        .collect();
    WideScopeRefactorFallbackTruthPacketInput {
        packet_id: packet_id.to_owned(),
        workflow_or_surface_id: "workflow.language.wide_scope_refactor_fallback".to_owned(),
        generated_at: "2026-06-14T12:00:00Z".to_owned(),
        covered_lanes: ArtifactFamilyLaneClass::REQUIRED.to_vec(),
        rows,
        consumer_projections: projections,
        source_contract_refs: vec![doc_ref()],
    }
}

fn framework_row(
    input: &mut WideScopeRefactorFallbackTruthPacketInput,
    row_class: FallbackRowClass,
) -> &mut FallbackRow {
    input
        .rows
        .iter_mut()
        .find(|row| {
            row.lane_class == ArtifactFamilyLaneClass::FrameworkPackLane
                && row.row_class == row_class
        })
        .expect("framework row exists")
}

#[test]
fn closed_tokens_are_pinned() {
    assert_eq!(
        FallbackRowClass::FallbackLaneQuality.as_str(),
        "fallback_lane_quality"
    );
    assert_eq!(
        FallbackRowClass::ApplyPostureAdmission.as_str(),
        "apply_posture_admission"
    );
    assert_eq!(
        FallbackRowClass::SupportExportParityAdmission.as_str(),
        "support_export_parity_admission"
    );
    assert_eq!(
        ApplyFallbackPostureClass::SideBranchApply.as_str(),
        "side_branch_apply"
    );
    assert_eq!(
        ApplyFallbackPostureClass::WorktreeApply.as_str(),
        "worktree_apply"
    );
    assert_eq!(
        ApplyFallbackPostureClass::ApplyAllOnLiveWorkspace.as_str(),
        "apply_all_on_live_workspace"
    );
    assert_eq!(
        ReviewerHintClass::CodeownersReviewer.as_str(),
        "codeowners_reviewer"
    );
    assert_eq!(
        FindingKind::UnsafeApplyAllBelowThreshold.as_str(),
        "unsafe_apply_all_below_threshold"
    );
    assert_eq!(
        FindingKind::ImpactPacketDropsMissingScope.as_str(),
        "impact_packet_drops_missing_scope"
    );
    assert_eq!(
        FindingKind::WritingFallbackWithoutSafeRollback.as_str(),
        "writing_fallback_without_safe_rollback"
    );
    assert_eq!(
        FindingKind::SupportExportDropsLineage.as_str(),
        "support_export_drops_lineage"
    );
}

#[test]
fn baseline_materialization_is_stable() {
    let packet = WideScopeRefactorFallbackTruthPacket::materialize(sample_input());
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
            "support:m5:wide_scope_refactor_fallback",
            "2026-06-14T12:00:10Z"
        )
        .is_export_safe());
}

#[test]
fn baseline_offers_safe_fallbacks_for_wide_scope_lanes() {
    let packet = WideScopeRefactorFallbackTruthPacket::materialize(sample_input());
    for row in &packet.rows {
        if row.row_class != FallbackRowClass::ApplyPostureAdmission {
            continue;
        }
        if row.apply_posture_class.is_apply_all_on_live() {
            // Apply-all is only permitted for a narrow, complete, high-confidence
            // transform.
            assert!(
                !scope_is_wide(row.target_scope_class)
                    && matches!(row.scope_completeness_class, CompletenessClass::Complete)
                    && matches!(row.confidence_class, ConfidenceClass::HighConfidence),
                "apply-all row {} must be narrow, complete, and high confidence",
                row.row_id
            );
        } else {
            assert!(
                row.apply_posture_class.is_safe_fallback(),
                "wide-scope row {} must offer a safe fallback posture",
                row.row_id
            );
        }
    }
}

#[test]
fn certified_with_unbound_evidence_blocks() {
    let mut input = sample_input();
    framework_row(&mut input, FallbackRowClass::FallbackLaneQuality).evidence_class =
        EvidenceClass::EvidenceUnbound;
    let packet = WideScopeRefactorFallbackTruthPacket::materialize(input);
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
            && row.row_class == FallbackRowClass::ApplyPostureAdmission)
    });
    let packet = WideScopeRefactorFallbackTruthPacket::materialize(input);
    assert_eq!(packet.promotion_state, PromotionState::BlocksStable);
    assert!(packet
        .validation_findings
        .iter()
        .any(|finding| finding.finding_kind == FindingKind::MissingApplyPostureCoverage));
}

#[test]
fn unsafe_apply_all_below_threshold_blocks() {
    let mut input = sample_input();
    // Framework lane is wide-scope (multi-file); apply-all on the live workspace
    // is refused.
    framework_row(&mut input, FallbackRowClass::ApplyPostureAdmission).apply_posture_class =
        ApplyFallbackPostureClass::ApplyAllOnLiveWorkspace;
    let packet = WideScopeRefactorFallbackTruthPacket::materialize(input);
    assert_eq!(packet.promotion_state, PromotionState::BlocksStable);
    assert!(packet
        .validation_findings
        .iter()
        .any(|finding| finding.finding_kind == FindingKind::UnsafeApplyAllBelowThreshold));
}

#[test]
fn apply_all_on_low_confidence_lane_blocks() {
    let mut input = sample_input();
    // Docs lane is narrow + complete but force low confidence under apply-all.
    let docs = input
        .rows
        .iter_mut()
        .find(|row| {
            row.lane_class == ArtifactFamilyLaneClass::DocsArtifactLane
                && row.row_class == FallbackRowClass::ApplyPostureAdmission
        })
        .expect("docs posture exists");
    docs.confidence_class = ConfidenceClass::LowConfidence;
    let packet = WideScopeRefactorFallbackTruthPacket::materialize(input);
    assert_eq!(packet.promotion_state, PromotionState::BlocksStable);
    assert!(packet
        .validation_findings
        .iter()
        .any(|finding| finding.finding_kind == FindingKind::UnsafeApplyAllBelowThreshold));
}

#[test]
fn scope_completeness_overclaimed_blocks() {
    let mut input = sample_input();
    let row = framework_row(&mut input, FallbackRowClass::ApplyPostureAdmission);
    row.scope_completeness_class = CompletenessClass::Complete;
    row.missing_scope_count = 3;
    let packet = WideScopeRefactorFallbackTruthPacket::materialize(input);
    assert_eq!(packet.promotion_state, PromotionState::BlocksStable);
    assert!(packet
        .validation_findings
        .iter()
        .any(|finding| finding.finding_kind == FindingKind::ScopeCompletenessOverclaimed));
}

#[test]
fn impact_packet_missing_summary_blocks() {
    let mut input = sample_input();
    framework_row(&mut input, FallbackRowClass::ImpactPacketAdmission).impact_summary_present =
        false;
    let packet = WideScopeRefactorFallbackTruthPacket::materialize(input);
    assert_eq!(packet.promotion_state, PromotionState::BlocksStable);
    assert!(packet
        .validation_findings
        .iter()
        .any(|finding| finding.finding_kind == FindingKind::MissingImpactSummary));
}

#[test]
fn impact_packet_drops_missing_scope_blocks() {
    let mut input = sample_input();
    // Notebook lane left two targets out of scope; the impact packet must keep
    // the missing-scope explanation.
    let notebook = input
        .rows
        .iter_mut()
        .find(|row| {
            row.lane_class == ArtifactFamilyLaneClass::NotebookCellLane
                && row.row_class == FallbackRowClass::ImpactPacketAdmission
        })
        .expect("notebook impact exists");
    notebook.missing_scope_explanation_present = false;
    let packet = WideScopeRefactorFallbackTruthPacket::materialize(input);
    assert_eq!(packet.promotion_state, PromotionState::BlocksStable);
    assert!(packet
        .validation_findings
        .iter()
        .any(|finding| finding.finding_kind == FindingKind::ImpactPacketDropsMissingScope));
}

#[test]
fn reviewer_hint_missing_anchor_blocks() {
    let mut input = sample_input();
    framework_row(&mut input, FallbackRowClass::ReviewerHintAdmission).review_anchor_ref = None;
    let packet = WideScopeRefactorFallbackTruthPacket::materialize(input);
    assert_eq!(packet.promotion_state, PromotionState::BlocksStable);
    assert!(packet
        .validation_findings
        .iter()
        .any(|finding| finding.finding_kind == FindingKind::MissingReviewAnchorRef));
}

#[test]
fn reviewer_hint_missing_owner_hint_blocks() {
    let mut input = sample_input();
    framework_row(&mut input, FallbackRowClass::ReviewerHintAdmission).owner_hint_present = false;
    let packet = WideScopeRefactorFallbackTruthPacket::materialize(input);
    assert_eq!(packet.promotion_state, PromotionState::BlocksStable);
    assert!(packet
        .validation_findings
        .iter()
        .any(|finding| finding.finding_kind == FindingKind::MissingOwnerHint));
}

#[test]
fn writing_fallback_without_safe_rollback_blocks() {
    let mut input = sample_input();
    // Framework lane writes under side-branch apply; a no-safe-rollback route is
    // refused.
    framework_row(&mut input, FallbackRowClass::RollbackPathAdmission).rollback_path_class =
        RollbackPathClass::NoSafeRollbackAvailable;
    let packet = WideScopeRefactorFallbackTruthPacket::materialize(input);
    assert_eq!(packet.promotion_state, PromotionState::BlocksStable);
    assert!(packet
        .validation_findings
        .iter()
        .any(|finding| finding.finding_kind == FindingKind::WritingFallbackWithoutSafeRollback));
}

#[test]
fn mutating_fallback_without_checkpoint_blocks() {
    let mut input = sample_input();
    framework_row(&mut input, FallbackRowClass::RollbackPathAdmission).checkpoint_ref = None;
    let packet = WideScopeRefactorFallbackTruthPacket::materialize(input);
    assert_eq!(packet.promotion_state, PromotionState::BlocksStable);
    assert!(packet
        .validation_findings
        .iter()
        .any(|finding| finding.finding_kind == FindingKind::MissingCheckpointRef));
}

#[test]
fn support_export_drops_lineage_blocks() {
    let mut input = sample_input();
    framework_row(&mut input, FallbackRowClass::SupportExportParityAdmission)
        .preserves_refactor_lineage = false;
    let packet = WideScopeRefactorFallbackTruthPacket::materialize(input);
    assert_eq!(packet.promotion_state, PromotionState::BlocksStable);
    assert!(packet
        .validation_findings
        .iter()
        .any(|finding| finding.finding_kind == FindingKind::SupportExportDropsLineage));
}

#[test]
fn support_export_missing_lineage_ref_blocks() {
    let mut input = sample_input();
    framework_row(&mut input, FallbackRowClass::SupportExportParityAdmission).lineage_ref = None;
    let packet = WideScopeRefactorFallbackTruthPacket::materialize(input);
    assert_eq!(packet.promotion_state, PromotionState::BlocksStable);
    assert!(packet
        .validation_findings
        .iter()
        .any(|finding| finding.finding_kind == FindingKind::MissingLineageRef));
}

#[test]
fn disagreement_collapsed_to_ranking_only_blocks() {
    let mut input = sample_input();
    framework_row(&mut input, FallbackRowClass::ProviderDisagreementAdmission)
        .disagreement_visibility_class = DisagreementVisibilityClass::RankingOnlyCollapsed;
    let packet = WideScopeRefactorFallbackTruthPacket::materialize(input);
    assert_eq!(packet.promotion_state, PromotionState::BlocksStable);
    assert!(packet
        .validation_findings
        .iter()
        .any(|finding| finding.finding_kind == FindingKind::DisagreementCollapsedToRankingOnly));
}

#[test]
fn missing_engine_identity_label_blocks() {
    let mut input = sample_input();
    framework_row(&mut input, FallbackRowClass::FallbackLaneQuality).engine_identity_label = None;
    let packet = WideScopeRefactorFallbackTruthPacket::materialize(input);
    assert_eq!(packet.promotion_state, PromotionState::BlocksStable);
    assert!(packet
        .validation_findings
        .iter()
        .any(|finding| finding.finding_kind == FindingKind::MissingEngineIdentityLabel));
}

#[test]
fn dimension_bound_on_wrong_row_class_blocks() {
    let mut input = sample_input();
    framework_row(&mut input, FallbackRowClass::ImpactPacketAdmission).apply_posture_class =
        ApplyFallbackPostureClass::SideBranchApply;
    let packet = WideScopeRefactorFallbackTruthPacket::materialize(input);
    assert_eq!(packet.promotion_state, PromotionState::BlocksStable);
    assert!(packet
        .validation_findings
        .iter()
        .any(|finding| finding.finding_kind == FindingKind::ApplyPostureNotPermittedOnRowClass));
}

#[test]
fn missing_refactor_id_blocks() {
    let mut input = sample_input();
    framework_row(&mut input, FallbackRowClass::FallbackLaneQuality).refactor_id = String::new();
    let packet = WideScopeRefactorFallbackTruthPacket::materialize(input);
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
    let packet = WideScopeRefactorFallbackTruthPacket::materialize(input);
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
    let packet = WideScopeRefactorFallbackTruthPacket::materialize(input);
    assert_eq!(packet.promotion_state, PromotionState::BlocksStable);
    assert!(packet
        .validation_findings
        .iter()
        .any(|finding| finding.finding_kind == FindingKind::ApplyPostureVocabularyCollapsed));
}

#[test]
fn narrowed_row_missing_disclosure_ref_blocks() {
    let mut input = sample_input();
    let row = framework_row(&mut input, FallbackRowClass::FallbackLaneQuality);
    row.support_class = SupportClass::CertifiedBelow;
    row.disclosure_ref = None;
    let packet = WideScopeRefactorFallbackTruthPacket::materialize(input);
    assert_eq!(packet.promotion_state, PromotionState::BlocksStable);
    assert!(packet
        .validation_findings
        .iter()
        .any(|finding| finding.finding_kind == FindingKind::NarrowedRowMissingDisclosureRef));
}

#[test]
fn raw_source_material_blocks_promotion() {
    let mut input = sample_input();
    framework_row(&mut input, FallbackRowClass::FallbackLaneQuality).raw_source_material_excluded =
        false;
    let packet = WideScopeRefactorFallbackTruthPacket::materialize(input);
    assert_eq!(packet.promotion_state, PromotionState::BlocksStable);
    assert!(packet
        .validation_findings
        .iter()
        .any(|finding| finding.finding_kind == FindingKind::RawSourceMaterialPresent));
}
