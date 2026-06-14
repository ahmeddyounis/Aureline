use super::*;

fn doc_ref() -> String {
    PROVIDER_REFACTOR_MATRIX_TRUTH_DOC_REF.to_owned()
}

fn fixture_ref() -> String {
    PROVIDER_REFACTOR_MATRIX_TRUTH_FIXTURE_DIR.to_owned()
}

/// Per-lane posture used to seed a fully covered, stable lane.
struct LaneSpec {
    lane: ArtifactFamilyLaneClass,
    prefix: &'static str,
    provider: ProviderFamilyClass,
    capability: CapabilityNegotiationClass,
    conflict: ConflictClass,
    diagnostic: DiagnosticSourceClass,
    provenance: ResultProvenanceClass,
    mode: SemanticLayerModeClass,
    refactor: RefactorTransactionClass,
    completeness: CompletenessClass,
    rollback: RollbackPathClass,
    generated: GeneratedArtifactPolicyClass,
    downgrade_label: DowngradeLabelClass,
}

fn lane_specs() -> Vec<LaneSpec> {
    vec![
        LaneSpec {
            lane: ArtifactFamilyLaneClass::FrameworkPackLane,
            prefix: "framework",
            provider: ProviderFamilyClass::FrameworkAnalyzer,
            capability: CapabilityNegotiationClass::FullSemanticNegotiated,
            conflict: ConflictClass::ArbitratedWinnerLoserPreserved,
            diagnostic: DiagnosticSourceClass::FrameworkSchema,
            provenance: ResultProvenanceClass::LiveSemantic,
            mode: SemanticLayerModeClass::PreviewableRefactor,
            refactor: RefactorTransactionClass::Extract,
            completeness: CompletenessClass::Complete,
            rollback: RollbackPathClass::GroupedMutationJournalRevert,
            generated: GeneratedArtifactPolicyClass::NotGenerated,
            downgrade_label: DowngradeLabelClass::FullToPartialCompleteness,
        },
        LaneSpec {
            lane: ArtifactFamilyLaneClass::NotebookCellLane,
            prefix: "notebook",
            provider: ProviderFamilyClass::NotebookAdapter,
            capability: CapabilityNegotiationClass::PartialSemanticNegotiated,
            conflict: ConflictClass::SingleProviderNoConflict,
            diagnostic: DiagnosticSourceClass::NotebookKernel,
            provenance: ResultProvenanceClass::CachedSemantic,
            mode: SemanticLayerModeClass::NotebookGeneratedBridge,
            refactor: RefactorTransactionClass::NotebookGeneratedEdit,
            completeness: CompletenessClass::Partial,
            rollback: RollbackPathClass::CompensatingRevertViaWorkspaceDiff,
            generated: GeneratedArtifactPolicyClass::RegenerateBeforeEdit,
            downgrade_label: DowngradeLabelClass::SemanticToTextFallback,
        },
        LaneSpec {
            lane: ArtifactFamilyLaneClass::GeneratedSourceLane,
            prefix: "generated",
            provider: ProviderFamilyClass::GeneratedSourceBridge,
            capability: CapabilityNegotiationClass::TextFallbackNegotiated,
            conflict: ConflictClass::PolicyOverrideRecorded,
            diagnostic: DiagnosticSourceClass::GeneratedArtifactValidation,
            provenance: ResultProvenanceClass::ImportedScan,
            mode: SemanticLayerModeClass::CodeActionMutation,
            refactor: RefactorTransactionClass::SchemaCodegenRewrite,
            completeness: CompletenessClass::Complete,
            rollback: RollbackPathClass::RegenerateFirstThenReplay,
            generated: GeneratedArtifactPolicyClass::EditWithRegenerationReplay,
            downgrade_label: DowngradeLabelClass::GeneratedEditToRegenerateFirst,
        },
        LaneSpec {
            lane: ArtifactFamilyLaneClass::StructuredArtifactLane,
            prefix: "structured",
            provider: ProviderFamilyClass::LspProvider,
            capability: CapabilityNegotiationClass::FullSemanticNegotiated,
            conflict: ConflictClass::SingleProviderNoConflict,
            diagnostic: DiagnosticSourceClass::CompilerBuild,
            provenance: ResultProvenanceClass::LiveSemantic,
            mode: SemanticLayerModeClass::SemanticRename,
            refactor: RefactorTransactionClass::Rename,
            completeness: CompletenessClass::Complete,
            rollback: RollbackPathClass::ExactUndoViaLocalHistoryCheckpoint,
            generated: GeneratedArtifactPolicyClass::NotGenerated,
            downgrade_label: DowngradeLabelClass::PreviewableToCompareOnly,
        },
        LaneSpec {
            lane: ArtifactFamilyLaneClass::CodeUnderstandingGraphLane,
            prefix: "graph",
            provider: ProviderFamilyClass::SemanticGraphLane,
            capability: CapabilityNegotiationClass::PartialSemanticNegotiated,
            conflict: ConflictClass::UnresolvedDisagreementSurfaced,
            diagnostic: DiagnosticSourceClass::Lsp,
            provenance: ResultProvenanceClass::PartialSemantic,
            mode: SemanticLayerModeClass::CompareOnly,
            refactor: RefactorTransactionClass::CompareOnlyNoMutation,
            completeness: CompletenessClass::Unsupported,
            rollback: RollbackPathClass::NoSafeRollbackAvailable,
            generated: GeneratedArtifactPolicyClass::CompareOnlyGenerated,
            downgrade_label: DowngradeLabelClass::ProviderUnavailableTextOnly,
        },
    ]
}

fn base_row(row_id: &str, lane: ArtifactFamilyLaneClass, row_class: MatrixRowClass) -> MatrixRow {
    MatrixRow {
        row_id: row_id.to_owned(),
        lane_class: lane,
        row_class,
        support_class: SupportClass::Certified,
        provider_family_class: ProviderFamilyClass::NotApplicable,
        capability_negotiation_class: CapabilityNegotiationClass::NotApplicable,
        conflict_class: ConflictClass::NotApplicable,
        diagnostic_source_class: DiagnosticSourceClass::NotApplicable,
        result_provenance_class: ResultProvenanceClass::NotApplicable,
        semantic_layer_mode_class: SemanticLayerModeClass::NotApplicable,
        refactor_transaction_class: RefactorTransactionClass::NotApplicable,
        completeness_class: CompletenessClass::NotApplicable,
        generated_artifact_policy_class: GeneratedArtifactPolicyClass::NotApplicable,
        downgrade_label_class: DowngradeLabelClass::NotApplicable,
        rollback_path_class: RollbackPathClass::NotApplicable,
        evidence_class: EvidenceClass::FixtureRepoEvidence,
        known_limit_class: KnownLimitClass::NoneDeclared,
        downgrade_automation_class: DowngradeAutomationClass::AutoNarrowOnMissingFixture,
        confidence_class: ConfidenceClass::HighConfidence,
        evidence_refs: vec![fixture_ref()],
        disclosure_ref: Some(format!("{}#auto_narrow_on_missing_fixture", doc_ref())),
        raw_source_material_excluded: true,
        secrets_excluded: true,
        ambient_authority_excluded: true,
        captured_at: "2026-06-14T12:00:00Z".to_owned(),
    }
}

fn lane_rows(spec: &LaneSpec) -> Vec<MatrixRow> {
    let mut rows = Vec::new();

    let mut quality = base_row(
        &format!("row:{}:quality", spec.prefix),
        spec.lane,
        MatrixRowClass::MatrixLaneQuality,
    );
    quality.provider_family_class = spec.provider;
    quality.evidence_class = EvidenceClass::ArchetypeRepoEvidence;
    quality.downgrade_automation_class = DowngradeAutomationClass::AutoBlockOnMissingEvidence;
    quality.disclosure_ref = Some(format!("{}#auto_block_on_missing_evidence", doc_ref()));
    quality.evidence_refs = vec![doc_ref(), fixture_ref()];
    rows.push(quality);

    let mut capability = base_row(
        &format!("row:{}:capability", spec.prefix),
        spec.lane,
        MatrixRowClass::CapabilityNegotiationAdmission,
    );
    capability.capability_negotiation_class = spec.capability;
    rows.push(capability);

    let mut conflict = base_row(
        &format!("row:{}:conflict", spec.prefix),
        spec.lane,
        MatrixRowClass::ConflictArbitrationAdmission,
    );
    conflict.conflict_class = spec.conflict;
    rows.push(conflict);

    let mut diagnostic = base_row(
        &format!("row:{}:diagnostic", spec.prefix),
        spec.lane,
        MatrixRowClass::DiagnosticSourceAdmission,
    );
    diagnostic.diagnostic_source_class = spec.diagnostic;
    rows.push(diagnostic);

    let mut provenance = base_row(
        &format!("row:{}:provenance", spec.prefix),
        spec.lane,
        MatrixRowClass::ResultProvenanceAdmission,
    );
    provenance.result_provenance_class = spec.provenance;
    rows.push(provenance);

    let mut semantic = base_row(
        &format!("row:{}:semantic_mode", spec.prefix),
        spec.lane,
        MatrixRowClass::SemanticLayerModeAdmission,
    );
    semantic.provider_family_class = spec.provider;
    semantic.semantic_layer_mode_class = spec.mode;
    rows.push(semantic);

    let mut refactor = base_row(
        &format!("row:{}:refactor", spec.prefix),
        spec.lane,
        MatrixRowClass::RefactorTransactionAdmission,
    );
    refactor.refactor_transaction_class = spec.refactor;
    refactor.completeness_class = spec.completeness;
    refactor.rollback_path_class = spec.rollback;
    refactor.evidence_class = EvidenceClass::ConformanceSuiteEvidence;
    rows.push(refactor);

    let mut generated = base_row(
        &format!("row:{}:generated_policy", spec.prefix),
        spec.lane,
        MatrixRowClass::GeneratedArtifactPolicyAdmission,
    );
    generated.generated_artifact_policy_class = spec.generated;
    rows.push(generated);

    let mut downgrade = base_row(
        &format!("row:{}:downgrade_label", spec.prefix),
        spec.lane,
        MatrixRowClass::DowngradeLabelAdmission,
    );
    downgrade.downgrade_label_class = spec.downgrade_label;
    rows.push(downgrade);

    rows
}

fn projection(surface: ConsumerSurface, packet_id: &str) -> MatrixConsumerProjection {
    MatrixConsumerProjection {
        consumer_surface: surface,
        projection_ref: format!("projection:{}", surface.as_str()),
        matrix_packet_id_ref: packet_id.to_owned(),
        rendered_at: "2026-06-14T12:00:01Z".to_owned(),
        preserves_same_packet: true,
        preserves_lane_vocabulary: true,
        preserves_row_class_vocabulary: true,
        preserves_support_class_vocabulary: true,
        preserves_provider_family_vocabulary: true,
        preserves_capability_negotiation_vocabulary: true,
        preserves_conflict_vocabulary: true,
        preserves_diagnostic_source_vocabulary: true,
        preserves_result_provenance_vocabulary: true,
        preserves_semantic_layer_mode_vocabulary: true,
        preserves_refactor_transaction_vocabulary: true,
        preserves_completeness_vocabulary: true,
        preserves_generated_artifact_policy_vocabulary: true,
        preserves_downgrade_label_vocabulary: true,
        preserves_rollback_path_vocabulary: true,
        preserves_known_limit_vocabulary: true,
        preserves_downgrade_automation_vocabulary: true,
        preserves_evidence_class_vocabulary: true,
        supports_json_export: true,
        raw_private_material_excluded: true,
        ambient_authority_excluded: true,
    }
}

fn sample_input() -> ProviderRefactorMatrixTruthPacketInput {
    let packet_id = "packet:m5:provider_refactor_matrix";
    let mut rows = Vec::new();
    for spec in lane_specs() {
        rows.extend(lane_rows(&spec));
    }
    let projections = ConsumerSurface::REQUIRED
        .into_iter()
        .map(|surface| projection(surface, packet_id))
        .collect();
    ProviderRefactorMatrixTruthPacketInput {
        packet_id: packet_id.to_owned(),
        workflow_or_surface_id: "workflow.language.provider_refactor_matrix".to_owned(),
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
        ArtifactFamilyLaneClass::CodeUnderstandingGraphLane.as_str(),
        "code_understanding_graph_lane"
    );
    assert_eq!(
        MatrixRowClass::MatrixLaneQuality.as_str(),
        "matrix_lane_quality"
    );
    assert_eq!(
        MatrixRowClass::SemanticLayerModeAdmission.as_str(),
        "semantic_layer_mode_admission"
    );
    assert_eq!(SupportClass::Certified.as_str(), "certified");
    assert_eq!(SupportClass::SupportUnbound.as_str(), "support_unbound");
    assert_eq!(
        ProviderFamilyClass::SemanticGraphLane.as_str(),
        "semantic_graph_lane"
    );
    assert_eq!(
        ProviderFamilyClass::ProviderUnbound.as_str(),
        "provider_unbound"
    );
    assert_eq!(
        CapabilityNegotiationClass::FullSemanticNegotiated.as_str(),
        "full_semantic_negotiated"
    );
    assert_eq!(
        ConflictClass::ArbitratedWinnerLoserPreserved.as_str(),
        "arbitrated_winner_loser_preserved"
    );
    assert_eq!(
        DiagnosticSourceClass::GeneratedArtifactValidation.as_str(),
        "generated_artifact_validation"
    );
    assert_eq!(
        ResultProvenanceClass::StalePendingRefresh.as_str(),
        "stale_pending_refresh"
    );
    assert_eq!(
        SemanticLayerModeClass::NotebookGeneratedBridge.as_str(),
        "notebook_generated_bridge"
    );
    assert_eq!(SemanticLayerModeClass::CompareOnly.as_str(), "compare_only");
    assert_eq!(
        RefactorTransactionClass::SchemaCodegenRewrite.as_str(),
        "schema_codegen_rewrite"
    );
    assert_eq!(
        RefactorTransactionClass::AiPlannedTransform.as_str(),
        "ai_planned_transform"
    );
    assert_eq!(CompletenessClass::Complete.as_str(), "complete");
    assert_eq!(
        GeneratedArtifactPolicyClass::EditBlockedGeneratedSource.as_str(),
        "edit_blocked_generated_source"
    );
    assert_eq!(
        DowngradeLabelClass::GeneratedEditToRegenerateFirst.as_str(),
        "generated_edit_to_regenerate_first"
    );
    assert_eq!(
        RollbackPathClass::RegenerateFirstThenReplay.as_str(),
        "regenerate_first_then_replay"
    );
    assert_eq!(
        ConsumerSurface::GeneratedArtifactSurface.as_str(),
        "generated_artifact_surface"
    );
    assert_eq!(PromotionState::BlocksStable.as_str(), "blocks_stable");
    assert_eq!(
        FindingKind::MutationBypassesPreviewOrRollback.as_str(),
        "mutation_bypasses_preview_or_rollback"
    );
    assert_eq!(
        FindingKind::MissingSemanticLayerModeCoverage.as_str(),
        "missing_semantic_layer_mode_coverage"
    );
}

#[test]
fn baseline_materialization_is_stable() {
    let packet = ProviderRefactorMatrixTruthPacket::materialize(sample_input());
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
            "support:m5:provider_refactor_matrix",
            "2026-06-14T12:00:10Z"
        )
        .is_export_safe());
}

#[test]
fn certified_with_unbound_evidence_blocks() {
    let mut input = sample_input();
    input.rows[0].evidence_class = EvidenceClass::EvidenceUnbound;
    let packet = ProviderRefactorMatrixTruthPacket::materialize(input);
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
fn missing_semantic_mode_admission_for_certified_lane_blocks() {
    let mut input = sample_input();
    input.rows.retain(|row| {
        !(row.lane_class == ArtifactFamilyLaneClass::FrameworkPackLane
            && row.row_class == MatrixRowClass::SemanticLayerModeAdmission)
    });
    let packet = ProviderRefactorMatrixTruthPacket::materialize(input);
    assert_eq!(packet.promotion_state, PromotionState::BlocksStable);
    assert!(packet
        .validation_findings
        .iter()
        .any(|finding| finding.finding_kind == FindingKind::MissingSemanticLayerModeCoverage));
}

#[test]
fn mutating_refactor_without_safe_rollback_blocks() {
    let mut input = sample_input();
    for row in &mut input.rows {
        if row.lane_class == ArtifactFamilyLaneClass::StructuredArtifactLane
            && row.row_class == MatrixRowClass::RefactorTransactionAdmission
        {
            row.rollback_path_class = RollbackPathClass::NoSafeRollbackAvailable;
        }
    }
    let packet = ProviderRefactorMatrixTruthPacket::materialize(input);
    assert_eq!(packet.promotion_state, PromotionState::BlocksStable);
    assert!(packet
        .validation_findings
        .iter()
        .any(|finding| finding.finding_kind == FindingKind::MutationBypassesPreviewOrRollback));
}

#[test]
fn narrowed_row_without_disclosure_ref_blocks() {
    let mut input = sample_input();
    input.rows[0].support_class = SupportClass::CertifiedBelow;
    input.rows[0].disclosure_ref = None;
    let packet = ProviderRefactorMatrixTruthPacket::materialize(input);
    assert_eq!(packet.promotion_state, PromotionState::BlocksStable);
    assert!(packet
        .validation_findings
        .iter()
        .any(|finding| finding.finding_kind == FindingKind::NarrowedRowMissingDisclosureRef));
}

#[test]
fn dimension_bound_on_wrong_row_class_blocks() {
    let mut input = sample_input();
    // Bind a conflict class on the capability-negotiation admission row.
    for row in &mut input.rows {
        if row.lane_class == ArtifactFamilyLaneClass::FrameworkPackLane
            && row.row_class == MatrixRowClass::CapabilityNegotiationAdmission
        {
            row.conflict_class = ConflictClass::SingleProviderNoConflict;
        }
    }
    let packet = ProviderRefactorMatrixTruthPacket::materialize(input);
    assert_eq!(packet.promotion_state, PromotionState::BlocksStable);
    assert!(packet
        .validation_findings
        .iter()
        .any(|finding| finding.finding_kind == FindingKind::ConflictNotPermittedOnRowClass));
}

#[test]
fn projection_drop_blocks_promotion() {
    let mut input = sample_input();
    input
        .consumer_projections
        .retain(|p| p.consumer_surface != ConsumerSurface::NotebookSurface);
    let packet = ProviderRefactorMatrixTruthPacket::materialize(input);
    assert_eq!(packet.promotion_state, PromotionState::BlocksStable);
    assert!(packet
        .validation_findings
        .iter()
        .any(|finding| finding.finding_kind == FindingKind::MissingConsumerProjection));
}

#[test]
fn collapsed_provider_family_vocabulary_blocks() {
    let mut input = sample_input();
    for projection in &mut input.consumer_projections {
        if projection.consumer_surface == ConsumerSurface::HelpAbout {
            projection.preserves_provider_family_vocabulary = false;
        }
    }
    let packet = ProviderRefactorMatrixTruthPacket::materialize(input);
    assert_eq!(packet.promotion_state, PromotionState::BlocksStable);
    assert!(packet
        .validation_findings
        .iter()
        .any(|finding| finding.finding_kind == FindingKind::ProviderFamilyVocabularyCollapsed));
}

#[test]
fn raw_source_material_blocks_promotion() {
    let mut input = sample_input();
    input.rows[0].raw_source_material_excluded = false;
    let packet = ProviderRefactorMatrixTruthPacket::materialize(input);
    assert_eq!(packet.promotion_state, PromotionState::BlocksStable);
    assert!(packet
        .validation_findings
        .iter()
        .any(|finding| finding.finding_kind == FindingKind::RawSourceMaterialPresent));
}
