use super::*;

fn doc_ref() -> String {
    PROVIDER_REFACTOR_CERTIFICATION_TRUTH_DOC_REF.to_owned()
}

fn fixture_ref() -> String {
    PROVIDER_REFACTOR_CERTIFICATION_TRUTH_FIXTURE_DIR.to_owned()
}

fn evidence_refs() -> Vec<String> {
    vec![doc_ref(), fixture_ref()]
}

/// Per-lane posture used to seed a fully covered, certified lane.
struct LaneSpec {
    lane: ArtifactFamilyLaneClass,
    prefix: &'static str,
    provider: ProviderFamilyClass,
    arbitration: ArbitrationProofClass,
    conflict: ConflictClass,
    convergence: ConvergenceProofClass,
    refactor: RefactorTransactionClass,
    completeness: CompletenessClass,
    rollback_path: RollbackPathClass,
    determinism: RollbackDeterminismClass,
    generated: GeneratedArtifactPolicyClass,
    drills: &'static [EvidenceDrillClass],
}

fn lane_specs() -> Vec<LaneSpec> {
    vec![
        LaneSpec {
            lane: ArtifactFamilyLaneClass::FrameworkPackLane,
            prefix: "framework",
            provider: ProviderFamilyClass::FrameworkAnalyzer,
            arbitration: ArbitrationProofClass::AgreementAndDisagreementProven,
            conflict: ConflictClass::ArbitratedWinnerLoserPreserved,
            convergence: ConvergenceProofClass::MultiSourceConvergedLabeled,
            refactor: RefactorTransactionClass::Extract,
            completeness: CompletenessClass::Complete,
            rollback_path: RollbackPathClass::GroupedMutationJournalRevert,
            determinism: RollbackDeterminismClass::DeterministicRollbackProven,
            generated: GeneratedArtifactPolicyClass::NotGenerated,
            drills: &[
                EvidenceDrillClass::FixtureRepoDrill,
                EvidenceDrillClass::PartialScopeDrill,
                EvidenceDrillClass::ProviderCrashQuarantineDrill,
            ],
        },
        LaneSpec {
            lane: ArtifactFamilyLaneClass::NotebookCellLane,
            prefix: "notebook",
            provider: ProviderFamilyClass::NotebookAdapter,
            arbitration: ArbitrationProofClass::SingleProviderNoConflict,
            conflict: ConflictClass::SingleProviderNoConflict,
            convergence: ConvergenceProofClass::ProvenancePreservedPerSource,
            refactor: RefactorTransactionClass::NotebookGeneratedEdit,
            completeness: CompletenessClass::Partial,
            rollback_path: RollbackPathClass::CompensatingRevertViaWorkspaceDiff,
            determinism: RollbackDeterminismClass::CheckpointReplayVerified,
            generated: GeneratedArtifactPolicyClass::RegenerateBeforeEdit,
            drills: &[EvidenceDrillClass::NotebookCaseDrill],
        },
        LaneSpec {
            lane: ArtifactFamilyLaneClass::GeneratedSourceLane,
            prefix: "generated",
            provider: ProviderFamilyClass::GeneratedSourceBridge,
            arbitration: ArbitrationProofClass::ProviderCrashQuarantineProven,
            conflict: ConflictClass::PolicyOverrideRecorded,
            convergence: ConvergenceProofClass::SuppressionStatePreserved,
            refactor: RefactorTransactionClass::SchemaCodegenRewrite,
            completeness: CompletenessClass::Complete,
            rollback_path: RollbackPathClass::RegenerateFirstThenReplay,
            determinism: RollbackDeterminismClass::RegenerationReplayVerified,
            generated: GeneratedArtifactPolicyClass::EditWithRegenerationReplay,
            drills: &[EvidenceDrillClass::GeneratedCaseDrill],
        },
        LaneSpec {
            lane: ArtifactFamilyLaneClass::StructuredArtifactLane,
            prefix: "structured",
            provider: ProviderFamilyClass::LspProvider,
            arbitration: ArbitrationProofClass::DowngradeHonestyProven,
            conflict: ConflictClass::SingleProviderNoConflict,
            convergence: ConvergenceProofClass::FreshnessLabeled,
            refactor: RefactorTransactionClass::Rename,
            completeness: CompletenessClass::Complete,
            rollback_path: RollbackPathClass::ExactUndoViaLocalHistoryCheckpoint,
            determinism: RollbackDeterminismClass::DeterministicRollbackProven,
            generated: GeneratedArtifactPolicyClass::NotGenerated,
            drills: &[EvidenceDrillClass::ConfigCaseDrill],
        },
        LaneSpec {
            lane: ArtifactFamilyLaneClass::CodeUnderstandingGraphLane,
            prefix: "graph",
            provider: ProviderFamilyClass::SemanticGraphLane,
            arbitration: ArbitrationProofClass::DisagreementWinnerLoserPreserved,
            conflict: ConflictClass::UnresolvedDisagreementSurfaced,
            convergence: ConvergenceProofClass::MultiSourceConvergedLabeled,
            refactor: RefactorTransactionClass::CompareOnlyNoMutation,
            completeness: CompletenessClass::Complete,
            rollback_path: RollbackPathClass::ManualReviewRequiredNoAutomaticPath,
            determinism: RollbackDeterminismClass::ManualReviewOnly,
            generated: GeneratedArtifactPolicyClass::CompareOnlyGenerated,
            drills: &[EvidenceDrillClass::RollbackDeterminismDrill],
        },
    ]
}

fn base_row(
    row_id: &str,
    lane: ArtifactFamilyLaneClass,
    row_class: CertificationRowClass,
) -> CertificationRow {
    CertificationRow {
        row_id: row_id.to_owned(),
        lane_class: lane,
        row_class,
        support_class: SupportClass::Certified,
        provider_family_class: ProviderFamilyClass::NotApplicable,
        verdict_class: CertificationVerdictClass::NotApplicable,
        arbitration_proof_class: ArbitrationProofClass::NotApplicable,
        conflict_class: ConflictClass::NotApplicable,
        convergence_proof_class: ConvergenceProofClass::NotApplicable,
        refactor_transaction_class: RefactorTransactionClass::NotApplicable,
        completeness_class: CompletenessClass::NotApplicable,
        rollback_path_class: RollbackPathClass::NotApplicable,
        rollback_determinism_class: RollbackDeterminismClass::NotApplicable,
        generated_artifact_policy_class: GeneratedArtifactPolicyClass::NotApplicable,
        evidence_drill_class: EvidenceDrillClass::NotApplicable,
        evidence_class: EvidenceClass::FixtureRepoEvidence,
        known_limit_class: KnownLimitClass::NoneDeclared,
        downgrade_automation_class: DowngradeAutomationClass::AutoNarrowOnMissingFixture,
        confidence_class: ConfidenceClass::HighConfidence,
        disagreement_inspectable: true,
        evidence_refs: evidence_refs(),
        disclosure_ref: Some(format!("{}#auto_narrow_on_missing_fixture", doc_ref())),
        raw_source_material_excluded: true,
        secrets_excluded: true,
        ambient_authority_excluded: true,
        captured_at: "2026-06-14T12:00:00Z".to_owned(),
    }
}

fn lane_rows(spec: &LaneSpec) -> Vec<CertificationRow> {
    let mut rows = Vec::new();

    let mut quality = base_row(
        &format!("row:{}:lane", spec.prefix),
        spec.lane,
        CertificationRowClass::LaneCertification,
    );
    quality.provider_family_class = spec.provider;
    quality.verdict_class = CertificationVerdictClass::Certified;
    quality.evidence_class = EvidenceClass::ArchetypeRepoEvidence;
    quality.downgrade_automation_class = DowngradeAutomationClass::AutoBlockOnMissingEvidence;
    quality.disclosure_ref = Some(format!("{}#auto_block_on_missing_evidence", doc_ref()));
    rows.push(quality);

    let mut arbitration = base_row(
        &format!("row:{}:arbitration", spec.prefix),
        spec.lane,
        CertificationRowClass::ProviderArbitrationCertification,
    );
    arbitration.arbitration_proof_class = spec.arbitration;
    arbitration.conflict_class = spec.conflict;
    rows.push(arbitration);

    let mut convergence = base_row(
        &format!("row:{}:convergence", spec.prefix),
        spec.lane,
        CertificationRowClass::DiagnosticConvergenceCertification,
    );
    convergence.convergence_proof_class = spec.convergence;
    rows.push(convergence);

    let mut refactor = base_row(
        &format!("row:{}:refactor_preview", spec.prefix),
        spec.lane,
        CertificationRowClass::RefactorPreviewCertification,
    );
    refactor.refactor_transaction_class = spec.refactor;
    refactor.completeness_class = spec.completeness;
    rows.push(refactor);

    let mut rollback = base_row(
        &format!("row:{}:rollback", spec.prefix),
        spec.lane,
        CertificationRowClass::RollbackDeterminismCertification,
    );
    rollback.rollback_path_class = spec.rollback_path;
    rollback.rollback_determinism_class = spec.determinism;
    rows.push(rollback);

    let mut generated = base_row(
        &format!("row:{}:generated_policy", spec.prefix),
        spec.lane,
        CertificationRowClass::GeneratedArtifactPolicyCertification,
    );
    generated.generated_artifact_policy_class = spec.generated;
    rows.push(generated);

    for (index, drill) in spec.drills.iter().enumerate() {
        let mut drill_row = base_row(
            &format!("row:{}:drill:{index}", spec.prefix),
            spec.lane,
            CertificationRowClass::EvidenceDrillAdmission,
        );
        drill_row.evidence_drill_class = *drill;
        drill_row.evidence_class = EvidenceClass::ConformanceSuiteEvidence;
        rows.push(drill_row);
    }

    rows
}

fn baseline_rows() -> Vec<CertificationRow> {
    let mut rows = Vec::new();
    for spec in lane_specs() {
        rows.extend(lane_rows(&spec));
    }
    rows
}

fn projection(surface: ConsumerSurface, packet_id: &str) -> CertificationConsumerProjection {
    CertificationConsumerProjection {
        consumer_surface: surface,
        projection_ref: format!("projection:certification:{}", surface.as_str()),
        certification_packet_id_ref: packet_id.to_owned(),
        rendered_at: "2026-06-14T12:00:00Z".to_owned(),
        preserves_same_packet: true,
        preserves_lane_vocabulary: true,
        preserves_row_class_vocabulary: true,
        preserves_support_class_vocabulary: true,
        preserves_provider_family_vocabulary: true,
        preserves_verdict_vocabulary: true,
        preserves_arbitration_proof_vocabulary: true,
        preserves_conflict_vocabulary: true,
        preserves_convergence_proof_vocabulary: true,
        preserves_refactor_transaction_vocabulary: true,
        preserves_completeness_vocabulary: true,
        preserves_rollback_path_vocabulary: true,
        preserves_rollback_determinism_vocabulary: true,
        preserves_generated_artifact_policy_vocabulary: true,
        preserves_evidence_drill_vocabulary: true,
        preserves_known_limit_vocabulary: true,
        preserves_downgrade_automation_vocabulary: true,
        preserves_evidence_class_vocabulary: true,
        supports_json_export: true,
        raw_private_material_excluded: true,
        ambient_authority_excluded: true,
    }
}

fn projections(packet_id: &str) -> Vec<CertificationConsumerProjection> {
    ConsumerSurface::REQUIRED
        .into_iter()
        .map(|surface| projection(surface, packet_id))
        .collect()
}

fn baseline_input(packet_id: &str) -> ProviderRefactorCertificationTruthPacketInput {
    ProviderRefactorCertificationTruthPacketInput {
        packet_id: packet_id.to_owned(),
        workflow_or_surface_id: "workflow.language.provider_refactor_certification.stable"
            .to_owned(),
        generated_at: "2026-06-14T12:00:00Z".to_owned(),
        covered_lanes: ArtifactFamilyLaneClass::REQUIRED.to_vec(),
        rows: baseline_rows(),
        consumer_projections: projections(packet_id),
        source_contract_refs: vec![
            PROVIDER_REFACTOR_CERTIFICATION_TRUTH_SCHEMA_REF.to_owned(),
            doc_ref(),
        ],
    }
}

fn row_mut<'a>(
    input: &'a mut ProviderRefactorCertificationTruthPacketInput,
    row_id: &str,
) -> &'a mut CertificationRow {
    input
        .rows
        .iter_mut()
        .find(|row| row.row_id == row_id)
        .unwrap_or_else(|| panic!("row {row_id} must exist"))
}

fn finding_kinds(packet: &ProviderRefactorCertificationTruthPacket) -> Vec<&'static str> {
    packet
        .validation_findings
        .iter()
        .map(|finding| finding.finding_kind.as_str())
        .collect()
}

const ID: &str = "packet:provider_refactor_certification:test";

#[test]
fn baseline_materializes_stable() {
    let packet = ProviderRefactorCertificationTruthPacket::materialize(baseline_input(ID));
    assert_eq!(
        packet.promotion_state,
        PromotionState::Stable,
        "baseline must be stable, got {:?}",
        finding_kinds(&packet)
    );
    assert!(packet.validation_findings.is_empty());
    assert!(packet.is_stable());
    for surface in ConsumerSurface::REQUIRED {
        assert!(packet.has_projection_for(surface));
    }
}

#[test]
fn baseline_covers_every_required_drill() {
    let packet = ProviderRefactorCertificationTruthPacket::materialize(baseline_input(ID));
    let observed: BTreeSet<&str> = packet.evidence_drill_tokens().into_iter().collect();
    for drill in EvidenceDrillClass::REQUIRED {
        assert!(
            observed.contains(drill.as_str()),
            "baseline must enumerate the {} drill",
            drill.as_str()
        );
    }
}

#[test]
fn support_export_is_safe_for_stable_packet() {
    let packet = ProviderRefactorCertificationTruthPacket::materialize(baseline_input(ID));
    let export = packet.support_export("export:test", "2026-06-14T12:00:10Z");
    assert!(export.is_export_safe());
    assert_eq!(export.certification_packet_id_ref, packet.packet_id);
}

#[test]
fn certified_with_unbound_evidence_blocks_stable() {
    let mut input = baseline_input(ID);
    row_mut(&mut input, "row:framework:lane").evidence_class = EvidenceClass::EvidenceUnbound;
    let packet = ProviderRefactorCertificationTruthPacket::materialize(input);
    assert_eq!(packet.promotion_state, PromotionState::BlocksStable);
    let kinds = finding_kinds(&packet);
    assert!(kinds.contains(&"missing_evidence_class"));
    assert!(kinds.contains(&"certified_with_unbound_binding"));
}

#[test]
fn missing_arbitration_certification_blocks_stable() {
    let mut input = baseline_input(ID);
    input
        .rows
        .retain(|row| row.row_id != "row:framework:arbitration");
    let packet = ProviderRefactorCertificationTruthPacket::materialize(input);
    assert!(finding_kinds(&packet).contains(&"missing_provider_arbitration_coverage"));
}

#[test]
fn missing_required_drill_blocks_stable() {
    let mut input = baseline_input(ID);
    // Drop the only rollback-determinism drill in the packet.
    input.rows.retain(|row| row.row_id != "row:graph:drill:0");
    let packet = ProviderRefactorCertificationTruthPacket::materialize(input);
    let kinds = finding_kinds(&packet);
    assert!(
        kinds.contains(&"missing_required_evidence_drill"),
        "expected missing_required_evidence_drill, got {kinds:?}"
    );
}

#[test]
fn mutating_refactor_without_labeled_preview_blocks_stable() {
    let mut input = baseline_input(ID);
    row_mut(&mut input, "row:framework:refactor_preview").completeness_class =
        CompletenessClass::Unsupported;
    let packet = ProviderRefactorCertificationTruthPacket::materialize(input);
    assert!(finding_kinds(&packet).contains(&"mutation_bypasses_preview_or_rollback"));
}

#[test]
fn nondeterministic_rollback_blocks_certified_lane() {
    let mut input = baseline_input(ID);
    row_mut(&mut input, "row:framework:rollback").rollback_determinism_class =
        RollbackDeterminismClass::NondeterministicUnsafe;
    let packet = ProviderRefactorCertificationTruthPacket::materialize(input);
    assert!(finding_kinds(&packet).contains(&"rollback_determinism_not_proven"));
}

#[test]
fn disagreement_collapsed_to_ranking_blocks_stable() {
    let mut input = baseline_input(ID);
    row_mut(&mut input, "row:framework:arbitration").disagreement_inspectable = false;
    let packet = ProviderRefactorCertificationTruthPacket::materialize(input);
    assert!(finding_kinds(&packet).contains(&"disagreement_collapsed_to_ranking_only"));
}

#[test]
fn verdict_support_mismatch_blocks_stable() {
    let mut input = baseline_input(ID);
    row_mut(&mut input, "row:framework:lane").verdict_class =
        CertificationVerdictClass::BlockedPendingEvidence;
    let packet = ProviderRefactorCertificationTruthPacket::materialize(input);
    assert!(finding_kinds(&packet).contains(&"verdict_support_mismatch"));
}

#[test]
fn dimension_bound_on_wrong_row_class_blocks_stable() {
    let mut input = baseline_input(ID);
    // Bind an arbitration proof on the convergence row.
    row_mut(&mut input, "row:framework:convergence").arbitration_proof_class =
        ArbitrationProofClass::AgreementAndDisagreementProven;
    let packet = ProviderRefactorCertificationTruthPacket::materialize(input);
    assert!(finding_kinds(&packet).contains(&"arbitration_proof_not_permitted_on_row_class"));
}

#[test]
fn narrowed_row_missing_disclosure_ref_blocks_stable() {
    let mut input = baseline_input(ID);
    let row = row_mut(&mut input, "row:framework:lane");
    row.support_class = SupportClass::CertifiedBelow;
    row.disclosure_ref = None;
    let packet = ProviderRefactorCertificationTruthPacket::materialize(input);
    let kinds = finding_kinds(&packet);
    assert!(kinds.contains(&"narrowed_row_missing_disclosure_ref"));
    assert!(kinds.contains(&"downgrade_automation_missing_disclosure_ref"));
}

#[test]
fn raw_source_material_blocks_stable() {
    let mut input = baseline_input(ID);
    row_mut(&mut input, "row:framework:lane").raw_source_material_excluded = false;
    let packet = ProviderRefactorCertificationTruthPacket::materialize(input);
    assert!(finding_kinds(&packet).contains(&"raw_source_material_present"));
}

#[test]
fn projection_collapses_arbitration_proof_vocabulary_blocks_stable() {
    let mut input = baseline_input(ID);
    for projection in &mut input.consumer_projections {
        if projection.consumer_surface == ConsumerSurface::CompatibilityReport {
            projection.preserves_arbitration_proof_vocabulary = false;
        }
    }
    let packet = ProviderRefactorCertificationTruthPacket::materialize(input);
    let kinds = finding_kinds(&packet);
    assert!(kinds.contains(&"arbitration_proof_vocabulary_collapsed"));
    assert!(kinds.contains(&"consumer_projection_drift"));
    assert!(kinds.contains(&"missing_consumer_projection"));
}

#[test]
fn missing_consumer_projection_blocks_stable() {
    let mut input = baseline_input(ID);
    input
        .consumer_projections
        .retain(|projection| projection.consumer_surface != ConsumerSurface::HelpAbout);
    let packet = ProviderRefactorCertificationTruthPacket::materialize(input);
    assert!(finding_kinds(&packet).contains(&"missing_consumer_projection"));
}
