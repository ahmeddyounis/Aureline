//! Regenerates the checked-in provider/refactor certification truth packet and
//! its protected fixture corpus from the real validator, so the fixtures can
//! never drift from the materialized packet.
//!
//! Run with:
//!
//! ```
//! cargo run -p aureline-language --example dump_provider_refactor_certification_truth_packet
//! ```
//!
//! It writes:
//!
//! - `artifacts/language/m5/provider_refactor_certification_truth_packet.json`
//! - `fixtures/language/m5/provider_refactor_certification_truth_packet/*.json`

use std::path::PathBuf;

use aureline_language::provider_refactor_certification_truth_packet::{
    ArbitrationProofClass, ArtifactFamilyLaneClass, CertificationConsumerProjection,
    CertificationRow, CertificationRowClass, CertificationVerdictClass, CompletenessClass,
    ConfidenceClass, ConflictClass, ConsumerSurface, ConvergenceProofClass,
    DowngradeAutomationClass, EvidenceClass, EvidenceDrillClass, GeneratedArtifactPolicyClass,
    KnownLimitClass, ProviderFamilyClass, ProviderRefactorCertificationTruthPacket,
    ProviderRefactorCertificationTruthPacketInput, RefactorTransactionClass,
    RollbackDeterminismClass, RollbackPathClass, SupportClass,
    PROVIDER_REFACTOR_CERTIFICATION_TRUTH_DOC_REF,
    PROVIDER_REFACTOR_CERTIFICATION_TRUTH_FIXTURE_DIR,
    PROVIDER_REFACTOR_CERTIFICATION_TRUTH_SCHEMA_REF,
};
use serde_json::{json, Value};

const TS: &str = "2026-06-14T12:00:00Z";
const PACKET_ID: &str = "packet:m5:provider_refactor_certification:stable";
const WORKFLOW: &str = "workflow.language.provider_refactor_certification.stable";

fn disclosure(anchor: &str) -> String {
    format!("{PROVIDER_REFACTOR_CERTIFICATION_TRUTH_DOC_REF}#{anchor}")
}

fn evidence_refs() -> Vec<String> {
    vec![
        PROVIDER_REFACTOR_CERTIFICATION_TRUTH_DOC_REF.to_owned(),
        PROVIDER_REFACTOR_CERTIFICATION_TRUTH_FIXTURE_DIR.to_owned(),
    ]
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
    confidence: ConfidenceClass,
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
            confidence: ConfidenceClass::HighConfidence,
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
            confidence: ConfidenceClass::MediumConfidence,
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
            confidence: ConfidenceClass::MediumConfidence,
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
            confidence: ConfidenceClass::HighConfidence,
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
            confidence: ConfidenceClass::MediumConfidence,
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
        disclosure_ref: Some(disclosure("auto_narrow_on_missing_fixture")),
        raw_source_material_excluded: true,
        secrets_excluded: true,
        ambient_authority_excluded: true,
        captured_at: TS.to_owned(),
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
    quality.disclosure_ref = Some(disclosure("auto_block_on_missing_evidence"));
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

    for row in &mut rows {
        row.confidence_class = spec.confidence;
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

fn projections(packet_id: &str) -> Vec<CertificationConsumerProjection> {
    ConsumerSurface::REQUIRED
        .into_iter()
        .map(|surface| CertificationConsumerProjection {
            consumer_surface: surface,
            projection_ref: format!(
                "projection:provider_refactor_certification:{}",
                surface.as_str()
            ),
            certification_packet_id_ref: packet_id.to_owned(),
            rendered_at: TS.to_owned(),
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
        })
        .collect()
}

fn baseline_input(packet_id: &str) -> ProviderRefactorCertificationTruthPacketInput {
    ProviderRefactorCertificationTruthPacketInput {
        packet_id: packet_id.to_owned(),
        workflow_or_surface_id: WORKFLOW.to_owned(),
        generated_at: TS.to_owned(),
        covered_lanes: ArtifactFamilyLaneClass::REQUIRED.to_vec(),
        rows: baseline_rows(),
        consumer_projections: projections(packet_id),
        source_contract_refs: vec![
            PROVIDER_REFACTOR_CERTIFICATION_TRUTH_SCHEMA_REF.to_owned(),
            PROVIDER_REFACTOR_CERTIFICATION_TRUTH_DOC_REF.to_owned(),
        ],
    }
}

fn token_array(tokens: Vec<&'static str>) -> Value {
    Value::Array(tokens.into_iter().map(|t| json!(t)).collect())
}

fn expect_block(
    packet: &ProviderRefactorCertificationTruthPacket,
    export_safe: bool,
    expected_finding_kinds: &[&str],
) -> Value {
    json!({
        "promotion_state": packet.promotion_state.as_str(),
        "validation_finding_count": packet.validation_findings.len(),
        "row_count": packet.rows.len(),
        "lane_tokens": token_array(packet.lane_tokens()),
        "row_class_tokens": token_array(packet.row_class_tokens()),
        "support_class_tokens": token_array(packet.support_class_tokens()),
        "provider_family_tokens": token_array(packet.provider_family_tokens()),
        "verdict_tokens": token_array(packet.verdict_tokens()),
        "arbitration_proof_tokens": token_array(packet.arbitration_proof_tokens()),
        "conflict_tokens": token_array(packet.conflict_tokens()),
        "convergence_proof_tokens": token_array(packet.convergence_proof_tokens()),
        "refactor_transaction_tokens": token_array(packet.refactor_transaction_tokens()),
        "completeness_tokens": token_array(packet.completeness_tokens()),
        "rollback_path_tokens": token_array(packet.rollback_path_tokens()),
        "rollback_determinism_tokens": token_array(packet.rollback_determinism_tokens()),
        "generated_artifact_policy_tokens": token_array(packet.generated_artifact_policy_tokens()),
        "evidence_drill_tokens": token_array(packet.evidence_drill_tokens()),
        "known_limit_tokens": token_array(packet.known_limit_tokens()),
        "downgrade_automation_tokens": token_array(packet.downgrade_automation_tokens()),
        "evidence_class_tokens": token_array(packet.evidence_class_tokens()),
        "support_export_safe": export_safe,
        "expected_finding_kinds": expected_finding_kinds,
    })
}

fn fixture(
    case_name: &str,
    scenario: &str,
    input: ProviderRefactorCertificationTruthPacketInput,
    export_safe: bool,
    expected_finding_kinds: &[&str],
) -> Value {
    let packet = ProviderRefactorCertificationTruthPacket::materialize(input.clone());
    json!({
        "record_kind": "provider_refactor_certification_truth_stable_case",
        "schema_version": 1,
        "case_name": case_name,
        "scenario": scenario,
        "input": serde_json::to_value(&input).expect("input serializes"),
        "expect": expect_block(&packet, export_safe, expected_finding_kinds),
    })
}

fn repo_path(rel: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join(rel)
}

fn write_json(rel: &str, value: &Value) {
    let path = repo_path(rel);
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent).expect("create parent dir");
    }
    let mut text = serde_json::to_string_pretty(value).expect("value serializes");
    text.push('\n');
    std::fs::write(&path, text).unwrap_or_else(|err| panic!("write {rel} failed: {err}"));
    println!("wrote {rel}");
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

fn main() {
    // Checked-in stable artifact packet.
    let packet = ProviderRefactorCertificationTruthPacket::materialize(baseline_input(PACKET_ID));
    assert!(
        packet.validation_findings.is_empty(),
        "baseline packet must be stable, got {:?}",
        packet
            .validation_findings
            .iter()
            .map(|f| f.finding_kind.as_str())
            .collect::<Vec<_>>()
    );
    write_json(
        "artifacts/language/m5/provider_refactor_certification_truth_packet.json",
        &serde_json::to_value(&packet).expect("packet serializes"),
    );

    let dir = PROVIDER_REFACTOR_CERTIFICATION_TRUTH_FIXTURE_DIR;
    let id = |suffix: &str| format!("packet:m5:provider_refactor_certification:{suffix}");

    // Baseline stable fixture.
    write_json(
        &format!("{dir}/baseline_stable.json"),
        &fixture(
            "baseline_stable",
            "Baseline stable posture: every claimed M5 artifact family (framework pack, notebook cell, generated source, structured artifact, and code-understanding graph) carries a lane_certification row at certified that names its acting provider family and a concrete verdict, plus one certification row per dimension — provider arbitration (binding the arbitration proof and conflict class with disagreement kept inspectable), diagnostic convergence (binding the convergence proof), refactor preview (co-binding the refactor class and a typed completeness label), rollback determinism (co-binding the rollback path and a proven determinism outcome), and generated-artifact policy — and the packet as a whole enumerates every required evidence drill: fixture repo, notebook case, generated case, config case, partial scope, provider crash/quarantine, and rollback determinism. All ten required consumer projections (framework-pack panel, structured-artifact runner, preview surface, compatibility report, archetype scorecard, release-narrowing automation, support export, Help/About, service health, and conformance dashboard) preserve the packet verbatim.",
            baseline_input(&id("baseline_stable")),
            true,
            &[],
        ),
    );

    // Negative cases: each takes the baseline and trips one guardrail.
    type NegativeCase = (
        &'static str,
        &'static str,
        fn(&mut ProviderRefactorCertificationTruthPacketInput),
        &'static [&'static str],
    );
    let cases: Vec<NegativeCase> = vec![
        (
            "certified_with_unbound_evidence_blocks_stable",
            "A lane_certification row claims certified while its evidence class is evidence_unbound; the packet emits missing_evidence_class plus certified_with_unbound_binding and blocks the stable claim instead of inheriting an adjacent certified row.",
            |input| {
                row_mut(input, "row:framework:lane").evidence_class = EvidenceClass::EvidenceUnbound;
            },
            &["missing_evidence_class", "certified_with_unbound_binding"],
        ),
        (
            "missing_provider_arbitration_certification_blocks_stable",
            "A lane claims certified but drops its provider_arbitration_certification row; the packet emits missing_provider_arbitration_coverage and blocks the stable claim, so a lane cannot keep a certified grade without proving its arbitration truth.",
            |input| {
                input
                    .rows
                    .retain(|row| row.row_id != "row:framework:arbitration");
            },
            &["missing_provider_arbitration_coverage"],
        ),
        (
            "missing_required_evidence_drill_blocks_stable",
            "The packet drops its only rollback-determinism drill; the packet emits missing_required_evidence_drill and blocks the stable claim because a certified packet must carry the full evidence-drill set (fixture repo, notebook/generated/config cases, partial-scope, provider crash/quarantine, and rollback determinism).",
            |input| {
                input.rows.retain(|row| row.row_id != "row:graph:drill:0");
            },
            &["missing_required_evidence_drill"],
        ),
        (
            "mutating_refactor_without_labeled_preview_blocks_stable",
            "A refactor_preview_certification row certifies a mutating refactor while its completeness label is unsupported; the packet emits mutation_bypasses_preview_or_rollback so AI-planned transforms, organize-imports, schema/codegen rewrites, and notebook/generated edits cannot certify behind an unlabeled or unsafe preview.",
            |input| {
                row_mut(input, "row:framework:refactor_preview").completeness_class =
                    CompletenessClass::Unsupported;
            },
            &["mutation_bypasses_preview_or_rollback"],
        ),
        (
            "nondeterministic_rollback_blocks_stable",
            "A rollback_determinism_certification row on a certified lane binds a nondeterministic/unsafe rollback; the packet emits rollback_determinism_not_proven so a lane cannot keep a certified grade without deterministic rollback.",
            |input| {
                row_mut(input, "row:framework:rollback").rollback_determinism_class =
                    RollbackDeterminismClass::NondeterministicUnsafe;
            },
            &["rollback_determinism_not_proven"],
        ),
        (
            "disagreement_collapsed_to_ranking_only_blocks_stable",
            "A provider_arbitration_certification row certifies provider disagreement but collapses it to a ranking-only result; the packet emits disagreement_collapsed_to_ranking_only so the losing provider and downgrade reason stay inspectable rather than hidden behind a single ranked result.",
            |input| {
                row_mut(input, "row:framework:arbitration").disagreement_inspectable = false;
            },
            &["disagreement_collapsed_to_ranking_only"],
        ),
        (
            "verdict_contradicts_certified_support_blocks_stable",
            "A lane_certification row claims certified support but binds a blocked_pending_evidence verdict; the packet emits verdict_support_mismatch so a lane cannot market a certified grade while its verdict says the lane is blocked or withdrawn.",
            |input| {
                row_mut(input, "row:framework:lane").verdict_class =
                    CertificationVerdictClass::BlockedPendingEvidence;
            },
            &["verdict_support_mismatch"],
        ),
        (
            "dimension_bound_on_wrong_row_class_blocks_stable",
            "A diagnostic_convergence_certification row also binds an arbitration proof; the packet emits arbitration_proof_not_permitted_on_row_class so each certification dimension stays owned by exactly one row class.",
            |input| {
                row_mut(input, "row:framework:convergence").arbitration_proof_class =
                    ArbitrationProofClass::AgreementAndDisagreementProven;
            },
            &["arbitration_proof_not_permitted_on_row_class"],
        ),
        (
            "narrowed_row_missing_disclosure_ref_blocks_stable",
            "A lane_certification row narrows to certified_below but drops its disclosure ref; the packet emits narrowed_row_missing_disclosure_ref (and, because the row still binds a non-`none` downgrade automation, downgrade_automation_missing_disclosure_ref) and blocks the stable claim until the narrowing is disclosed.",
            |input| {
                let row = row_mut(input, "row:framework:lane");
                row.support_class = SupportClass::CertifiedBelow;
                row.disclosure_ref = None;
            },
            &[
                "narrowed_row_missing_disclosure_ref",
                "downgrade_automation_missing_disclosure_ref",
            ],
        ),
        (
            "raw_source_material_blocks_stable",
            "A row admits raw source bodies past the boundary; the packet emits raw_source_material_present and blocks the stable claim because raw source bodies, refactor diffs, generated artifact bodies, notebook outputs, provider payloads, secrets, and ambient credentials must never leak through the certification boundary.",
            |input| {
                row_mut(input, "row:framework:lane").raw_source_material_excluded = false;
            },
            &["raw_source_material_present"],
        ),
        (
            "projection_collapses_arbitration_proof_vocabulary_blocks_stable",
            "The compatibility_report consumer projection collapses the arbitration-proof vocabulary; the packet emits arbitration_proof_vocabulary_collapsed plus consumer_projection_drift and missing_consumer_projection because surfaces MUST preserve the closed arbitration-proof vocabulary that distinguishes agreement/disagreement, loser preservation, downgrade honesty, and crash/quarantine recovery.",
            |input| {
                for projection in &mut input.consumer_projections {
                    if projection.consumer_surface == ConsumerSurface::CompatibilityReport {
                        projection.preserves_arbitration_proof_vocabulary = false;
                    }
                }
            },
            &[
                "arbitration_proof_vocabulary_collapsed",
                "consumer_projection_drift",
                "missing_consumer_projection",
            ],
        ),
    ];

    for (case_name, scenario, mutate, kinds) in cases {
        let mut input = baseline_input(&id(case_name));
        mutate(&mut input);
        write_json(
            &format!("{dir}/{case_name}.json"),
            &fixture(case_name, scenario, input, false, kinds),
        );
    }
}
