//! Regenerates the checked-in typed refactor transaction truth packet and its
//! protected fixture corpus from the real validator, so the fixtures can never
//! drift from the materialized packet.
//!
//! Run with:
//!
//! ```
//! cargo run -p aureline-language --example dump_typed_refactor_transaction_truth_packet
//! ```
//!
//! It writes:
//!
//! - `artifacts/language/m5/typed_refactor_transaction_truth_packet.json`
//! - `fixtures/language/m5/typed_refactor_transaction_truth_packet/*.json`

use std::path::PathBuf;

use aureline_language::code_action_quick_fix_picker_truth_packet::{
    ArtifactFamilyLaneClass, DisagreementVisibilityClass, MutationScopeClass,
};
use aureline_language::provider_refactor_matrix_truth_packet::{
    CompletenessClass, ConfidenceClass, ConsumerSurface, DowngradeAutomationClass, EvidenceClass,
    GeneratedArtifactPolicyClass, KnownLimitClass, ProviderFamilyClass, RefactorTransactionClass,
    RollbackPathClass, SupportClass,
};
use aureline_language::typed_refactor_transaction_truth_packet::{
    ApplyPipelineClass, TransactionConsumerProjection, TransactionRow, TransactionRowClass,
    TypedRefactorTransactionTruthPacket, TypedRefactorTransactionTruthPacketInput,
    ValidationPlanClass, TYPED_REFACTOR_TRANSACTION_TRUTH_DOC_REF,
    TYPED_REFACTOR_TRANSACTION_TRUTH_FIXTURE_DIR, TYPED_REFACTOR_TRANSACTION_TRUTH_SCHEMA_REF,
};
use serde_json::{json, Value};

const TS: &str = "2026-06-14T12:00:00Z";
const PACKET_ID: &str = "packet:m5:typed_refactor_transaction:stable";
const WORKFLOW: &str = "workflow.language.typed_refactor_transaction.stable";

fn disclosure(anchor: &str) -> String {
    format!("{TYPED_REFACTOR_TRANSACTION_TRUTH_DOC_REF}#{anchor}")
}

fn evidence_refs() -> Vec<String> {
    vec![
        TYPED_REFACTOR_TRANSACTION_TRUTH_DOC_REF.to_owned(),
        TYPED_REFACTOR_TRANSACTION_TRUTH_FIXTURE_DIR.to_owned(),
    ]
}

fn rollback_needs_checkpoint(rollback: RollbackPathClass) -> bool {
    matches!(
        rollback,
        RollbackPathClass::ExactUndoViaLocalHistoryCheckpoint
            | RollbackPathClass::CompensatingRevertViaWorkspaceDiff
            | RollbackPathClass::GroupedMutationJournalRevert
    )
}

fn pipeline_applies_mutation(pipeline: ApplyPipelineClass) -> bool {
    matches!(
        pipeline,
        ApplyPipelineClass::SavePipelineWithJournal | ApplyPipelineClass::PreviewThenSavePipeline
    )
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
        evidence_refs: evidence_refs(),
        disclosure_ref: Some(disclosure("auto_narrow_on_missing_fixture")),
        engine_identity_label: None,
        validation_plan_ref: None,
        checkpoint_ref: None,
        raw_source_material_excluded: true,
        secrets_excluded: true,
        ambient_authority_excluded: true,
        captured_at: TS.to_owned(),
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
    quality.disclosure_ref = Some(disclosure("auto_block_on_missing_evidence"));
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
    target.evidence_class = EvidenceClass::ConformanceSuiteEvidence;
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
    pipeline.reuses_save_pipeline = pipeline_applies_mutation(spec.pipeline);
    pipeline.reuses_mutation_journal = pipeline_applies_mutation(spec.pipeline);
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
    if rollback_needs_checkpoint(spec.rollback) {
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

fn baseline_rows() -> Vec<TransactionRow> {
    let mut rows = Vec::new();
    for spec in lane_specs() {
        rows.extend(lane_rows(&spec));
    }
    rows
}

fn projections(packet_id: &str) -> Vec<TransactionConsumerProjection> {
    ConsumerSurface::REQUIRED
        .into_iter()
        .map(|surface| TransactionConsumerProjection {
            consumer_surface: surface,
            projection_ref: format!("projection:typed_refactor_transaction:{}", surface.as_str()),
            transaction_packet_id_ref: packet_id.to_owned(),
            rendered_at: TS.to_owned(),
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
        })
        .collect()
}

fn baseline_input(packet_id: &str) -> TypedRefactorTransactionTruthPacketInput {
    TypedRefactorTransactionTruthPacketInput {
        packet_id: packet_id.to_owned(),
        workflow_or_surface_id: WORKFLOW.to_owned(),
        generated_at: TS.to_owned(),
        covered_lanes: ArtifactFamilyLaneClass::REQUIRED.to_vec(),
        rows: baseline_rows(),
        consumer_projections: projections(packet_id),
        source_contract_refs: vec![
            TYPED_REFACTOR_TRANSACTION_TRUTH_SCHEMA_REF.to_owned(),
            TYPED_REFACTOR_TRANSACTION_TRUTH_DOC_REF.to_owned(),
        ],
    }
}

fn token_array(tokens: Vec<&'static str>) -> Value {
    Value::Array(tokens.into_iter().map(|t| json!(t)).collect())
}

fn expect_block(
    packet: &TypedRefactorTransactionTruthPacket,
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
        "engine_identity_tokens": token_array(packet.engine_identity_tokens()),
        "refactor_class_tokens": token_array(packet.refactor_class_tokens()),
        "target_scope_tokens": token_array(packet.target_scope_tokens()),
        "scope_completeness_tokens": token_array(packet.scope_completeness_tokens()),
        "validation_plan_tokens": token_array(packet.validation_plan_tokens()),
        "generated_asset_policy_tokens": token_array(packet.generated_asset_policy_tokens()),
        "apply_pipeline_tokens": token_array(packet.apply_pipeline_tokens()),
        "rollback_checkpoint_tokens": token_array(packet.rollback_checkpoint_tokens()),
        "disagreement_visibility_tokens": token_array(packet.disagreement_visibility_tokens()),
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
    input: TypedRefactorTransactionTruthPacketInput,
    export_safe: bool,
    expected_finding_kinds: &[&str],
) -> Value {
    let packet = TypedRefactorTransactionTruthPacket::materialize(input.clone());
    json!({
        "record_kind": "typed_refactor_transaction_truth_stable_case",
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
    input: &'a mut TypedRefactorTransactionTruthPacketInput,
    row_id: &str,
) -> &'a mut TransactionRow {
    input
        .rows
        .iter_mut()
        .find(|row| row.row_id == row_id)
        .unwrap_or_else(|| panic!("row {row_id} must exist"))
}

fn main() {
    // Checked-in stable artifact packet.
    let packet = TypedRefactorTransactionTruthPacket::materialize(baseline_input(PACKET_ID));
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
        "artifacts/language/m5/typed_refactor_transaction_truth_packet.json",
        &serde_json::to_value(&packet).expect("packet serializes"),
    );

    let dir = TYPED_REFACTOR_TRANSACTION_TRUTH_FIXTURE_DIR;
    let id = |suffix: &str| format!("packet:m5:typed_refactor_transaction:{suffix}");

    // Baseline stable fixture.
    write_json(
        &format!("{dir}/baseline_stable.json"),
        &fixture(
            "baseline_stable",
            "Baseline stable posture: every M5 artifact family (framework pack, notebook cell, docs artifact, request/structured artifact, config artifact, and generated source) carries a transaction_lane_quality row at certified that names its acting engine, exports an engine-identity label, and binds the refactor class, plus one admission row per transaction dimension: target scope (co-binding the missing-scope count and the typed completeness label), grouped hunks (co-binding the hunk count, impact summary, and ownership hint), validation plan (exporting a plan ref), generated-asset policy, apply pipeline (reusing the save pipeline and mutation journal for mutating applies, preserving source fidelity, and refusing a privileged fast path), rollback checkpoint (exporting a checkpoint ref on automatic routes), and provider-disagreement visibility. Every transaction is a typed transaction rather than an optimistic multi-file edit; the preview never overclaims completeness; the apply never bypasses the save pipeline, mutation journal, or source fidelity; generated source is never treated as ordinary text; disagreement keeps the winner and loser both inspectable; and all ten required consumer projections preserve the packet verbatim.",
            baseline_input(&id("baseline_stable")),
            true,
            &[],
        ),
    );

    // Negative cases: each takes the baseline and trips one guardrail.
    type NegativeCase = (
        &'static str,
        &'static str,
        fn(&mut TypedRefactorTransactionTruthPacketInput),
        &'static [&'static str],
    );
    let cases: Vec<NegativeCase> = vec![
        (
            "certified_with_unbound_evidence_blocks_stable",
            "A transaction_lane_quality row claims certified while its evidence class is evidence_unbound; the packet emits missing_evidence_class plus certified_with_unbound_binding and blocks the stable claim instead of inheriting an adjacent certified row.",
            |input| {
                row_mut(input, "row:framework:quality").evidence_class = EvidenceClass::EvidenceUnbound;
            },
            &["missing_evidence_class", "certified_with_unbound_binding"],
        ),
        (
            "missing_target_scope_admission_blocks_stable",
            "A lane claims certified but drops its target_scope_admission row; the packet emits missing_target_scope_coverage and blocks the stable claim, so a framework-aware transform cannot run without enumerating its target scope and missing-scope set.",
            |input| {
                input
                    .rows
                    .retain(|row| row.row_id != "row:framework:target_scope");
            },
            &["missing_target_scope_coverage"],
        ),
        (
            "scope_completeness_overclaimed_blocks_stable",
            "A target_scope_admission row labels the preview complete while leaving targets out of scope; the packet emits scope_completeness_overclaimed so a transaction cannot hide an incomplete target set behind a complete label.",
            |input| {
                let row = row_mut(input, "row:framework:target_scope");
                row.scope_completeness_class = CompletenessClass::Complete;
                row.missing_scope_count = 3;
            },
            &["scope_completeness_overclaimed"],
        ),
        (
            "grouped_hunks_missing_impact_summary_blocks_stable",
            "A grouped_hunks_admission row groups hunks but attaches no impact summary; the packet emits missing_impact_summary so a framework-aware or structured-artifact transform always carries grouped hunks with an impact summary.",
            |input| {
                row_mut(input, "row:framework:grouped_hunks").impact_summary_present = false;
            },
            &["missing_impact_summary"],
        ),
        (
            "validation_plan_missing_plan_ref_blocks_stable",
            "A validation_plan_admission row runs a validation plan but exports no plan ref; the packet emits missing_validation_plan_ref so a typed transaction always exports the validation plan its apply runs.",
            |input| {
                row_mut(input, "row:framework:validation_plan").validation_plan_ref = None;
            },
            &["missing_validation_plan_ref"],
        ),
        (
            "apply_pipeline_bypasses_save_pipeline_blocks_stable",
            "A mutating apply_pipeline_admission row does not reuse the save pipeline; the packet emits apply_pipeline_bypasses_save_pipeline so a refactor apply cannot take an optimistic write path around the normal save pipeline.",
            |input| {
                row_mut(input, "row:framework:apply_pipeline").reuses_save_pipeline = false;
            },
            &["apply_pipeline_bypasses_save_pipeline"],
        ),
        (
            "apply_pipeline_bypasses_mutation_journal_blocks_stable",
            "A mutating apply_pipeline_admission row does not reuse the mutation journal; the packet emits apply_pipeline_bypasses_mutation_journal so a refactor apply is always recorded in the mutation journal that owns its grouped revert.",
            |input| {
                row_mut(input, "row:framework:apply_pipeline").reuses_mutation_journal = false;
            },
            &["apply_pipeline_bypasses_mutation_journal"],
        ),
        (
            "source_fidelity_bypassed_blocks_stable",
            "An apply_pipeline_admission row does not preserve source fidelity; the packet emits source_fidelity_bypassed so a refactor apply cannot bypass the source-fidelity protections the launch-language safety model requires.",
            |input| {
                row_mut(input, "row:framework:apply_pipeline").source_fidelity_preserved = false;
            },
            &["source_fidelity_bypassed"],
        ),
        (
            "privileged_fast_path_blocks_stable",
            "An apply_pipeline_admission row takes a privileged fast path; the packet emits privileged_fast_path_not_permitted so AI-planned or framework transforms cannot take a privileged fast path around the typed refactor transaction.",
            |input| {
                row_mut(input, "row:framework:apply_pipeline").privileged_fast_path = true;
            },
            &["privileged_fast_path_not_permitted"],
        ),
        (
            "mutating_transaction_without_checkpoint_blocks_stable",
            "A rollback_checkpoint_admission row claims an automatic rollback route but exports no checkpoint ref; the packet emits missing_checkpoint_ref so AI-planned, schema/codegen, organize-imports, and notebook/generated transactions cannot bypass the rollback checkpoint the launch-language refactor safety model requires.",
            |input| {
                row_mut(input, "row:framework:rollback").checkpoint_ref = None;
            },
            &["missing_checkpoint_ref"],
        ),
        (
            "generated_policy_bypassed_blocks_stable",
            "The generated_source lane's generated_asset_policy_admission row binds not_generated; the packet emits generated_policy_bypassed so generated, notebook, lockfile, and config artifacts are never treated as ordinary text when policy requires regenerate/compare/block semantics.",
            |input| {
                row_mut(input, "row:generated:generated_policy").generated_asset_policy_class =
                    GeneratedArtifactPolicyClass::NotGenerated;
            },
            &["generated_policy_bypassed"],
        ),
        (
            "disagreement_collapsed_to_ranking_only_blocks_stable",
            "A provider_disagreement_admission row collapses the disagreement into ranking-only output; the packet emits disagreement_collapsed_to_ranking_only so the losing engine and downgrade reason stay inspectable rather than being hidden behind a single ranked result.",
            |input| {
                row_mut(input, "row:framework:disagreement").disagreement_visibility_class =
                    DisagreementVisibilityClass::RankingOnlyCollapsed;
            },
            &["disagreement_collapsed_to_ranking_only"],
        ),
        (
            "missing_engine_identity_label_blocks_stable",
            "A transaction_lane_quality row names a concrete acting engine but exports no engine-identity label; the packet emits missing_engine_identity_label so the transaction always names which engine planned it.",
            |input| {
                row_mut(input, "row:framework:quality").engine_identity_label = None;
            },
            &["missing_engine_identity_label"],
        ),
        (
            "narrowed_row_missing_disclosure_ref_blocks_stable",
            "A row narrows to certified_below but drops its disclosure ref; the packet emits narrowed_row_missing_disclosure_ref (and, because the row still binds a non-`none` downgrade automation, downgrade_automation_missing_disclosure_ref) and blocks the stable claim until the narrowing is disclosed.",
            |input| {
                let row = row_mut(input, "row:framework:quality");
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
            "A row admits raw source bodies past the boundary; the packet emits raw_source_material_present and blocks the stable claim because raw source bodies, refactor diffs, generated artifact bodies, notebook outputs, provider payloads, secrets, and ambient credentials must never leak through the transaction boundary.",
            |input| {
                row_mut(input, "row:framework:quality").raw_source_material_excluded = false;
            },
            &["raw_source_material_present"],
        ),
        (
            "projection_collapses_target_scope_vocabulary_blocks_stable",
            "The help_about consumer projection collapses the target-scope vocabulary; the packet emits target_scope_vocabulary_collapsed plus consumer_projection_drift and missing_consumer_projection because surfaces MUST preserve the closed target-scope vocabulary that distinguishes single-file, multi-file, cross-artifact, generated-artifact, structured-artifact, and workspace-wide scopes.",
            |input| {
                for projection in &mut input.consumer_projections {
                    if projection.consumer_surface == ConsumerSurface::HelpAbout {
                        projection.preserves_target_scope_vocabulary = false;
                    }
                }
            },
            &[
                "target_scope_vocabulary_collapsed",
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
