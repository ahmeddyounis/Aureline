//! Regenerates the checked-in wide-scope refactor fallback truth packet and its
//! protected fixture corpus from the real validator, so the fixtures can never
//! drift from the materialized packet.
//!
//! Run with:
//!
//! ```
//! cargo run -p aureline-language --example dump_wide_scope_refactor_fallback_truth_packet
//! ```
//!
//! It writes:
//!
//! - `artifacts/language/m5/wide_scope_refactor_fallback_truth_packet.json`
//! - `fixtures/language/m5/wide_scope_refactor_fallback_truth_packet/*.json`

use std::path::PathBuf;

use aureline_language::code_action_quick_fix_picker_truth_packet::{
    ArtifactFamilyLaneClass, DisagreementVisibilityClass, MutationScopeClass,
};
use aureline_language::provider_refactor_matrix_truth_packet::{
    CompletenessClass, ConfidenceClass, ConsumerSurface, DowngradeAutomationClass, EvidenceClass,
    KnownLimitClass, ProviderFamilyClass, RefactorTransactionClass, RollbackPathClass,
    SupportClass,
};
use aureline_language::wide_scope_refactor_fallback_truth_packet::{
    ApplyFallbackPostureClass, FallbackConsumerProjection, FallbackRow, FallbackRowClass,
    ReviewerHintClass, WideScopeRefactorFallbackTruthPacket,
    WideScopeRefactorFallbackTruthPacketInput, WIDE_SCOPE_REFACTOR_FALLBACK_TRUTH_DOC_REF,
    WIDE_SCOPE_REFACTOR_FALLBACK_TRUTH_FIXTURE_DIR, WIDE_SCOPE_REFACTOR_FALLBACK_TRUTH_SCHEMA_REF,
};
use serde_json::{json, Value};

const TS: &str = "2026-06-14T12:00:00Z";
const PACKET_ID: &str = "packet:m5:wide_scope_refactor_fallback:stable";
const WORKFLOW: &str = "workflow.language.wide_scope_refactor_fallback.stable";

fn disclosure(anchor: &str) -> String {
    format!("{WIDE_SCOPE_REFACTOR_FALLBACK_TRUTH_DOC_REF}#{anchor}")
}

fn evidence_refs() -> Vec<String> {
    vec![
        WIDE_SCOPE_REFACTOR_FALLBACK_TRUTH_DOC_REF.to_owned(),
        WIDE_SCOPE_REFACTOR_FALLBACK_TRUTH_FIXTURE_DIR.to_owned(),
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
        evidence_refs: evidence_refs(),
        disclosure_ref: Some(disclosure("auto_narrow_on_missing_fixture")),
        engine_identity_label: None,
        impact_packet_ref: None,
        review_anchor_ref: None,
        checkpoint_ref: None,
        lineage_ref: None,
        raw_source_material_excluded: true,
        secrets_excluded: true,
        ambient_authority_excluded: true,
        captured_at: TS.to_owned(),
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
    quality.disclosure_ref = Some(disclosure("auto_block_on_missing_evidence"));
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
    posture.evidence_class = EvidenceClass::ConformanceSuiteEvidence;
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
    if rollback_needs_checkpoint(spec.rollback) {
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

fn baseline_rows() -> Vec<FallbackRow> {
    let mut rows = Vec::new();
    for spec in lane_specs() {
        rows.extend(lane_rows(&spec));
    }
    rows
}

fn projections(packet_id: &str) -> Vec<FallbackConsumerProjection> {
    ConsumerSurface::REQUIRED
        .into_iter()
        .map(|surface| FallbackConsumerProjection {
            consumer_surface: surface,
            projection_ref: format!(
                "projection:wide_scope_refactor_fallback:{}",
                surface.as_str()
            ),
            fallback_packet_id_ref: packet_id.to_owned(),
            rendered_at: TS.to_owned(),
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
        })
        .collect()
}

fn baseline_input(packet_id: &str) -> WideScopeRefactorFallbackTruthPacketInput {
    WideScopeRefactorFallbackTruthPacketInput {
        packet_id: packet_id.to_owned(),
        workflow_or_surface_id: WORKFLOW.to_owned(),
        generated_at: TS.to_owned(),
        covered_lanes: ArtifactFamilyLaneClass::REQUIRED.to_vec(),
        rows: baseline_rows(),
        consumer_projections: projections(packet_id),
        source_contract_refs: vec![
            WIDE_SCOPE_REFACTOR_FALLBACK_TRUTH_SCHEMA_REF.to_owned(),
            WIDE_SCOPE_REFACTOR_FALLBACK_TRUTH_DOC_REF.to_owned(),
        ],
    }
}

fn token_array(tokens: Vec<&'static str>) -> Value {
    Value::Array(tokens.into_iter().map(|t| json!(t)).collect())
}

fn expect_block(
    packet: &WideScopeRefactorFallbackTruthPacket,
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
        "apply_posture_tokens": token_array(packet.apply_posture_tokens()),
        "target_scope_tokens": token_array(packet.target_scope_tokens()),
        "scope_completeness_tokens": token_array(packet.scope_completeness_tokens()),
        "confidence_tokens": token_array(packet.confidence_tokens()),
        "reviewer_hint_tokens": token_array(packet.reviewer_hint_tokens()),
        "rollback_path_tokens": token_array(packet.rollback_path_tokens()),
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
    input: WideScopeRefactorFallbackTruthPacketInput,
    export_safe: bool,
    expected_finding_kinds: &[&str],
) -> Value {
    let packet = WideScopeRefactorFallbackTruthPacket::materialize(input.clone());
    json!({
        "record_kind": "wide_scope_refactor_fallback_truth_stable_case",
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
    input: &'a mut WideScopeRefactorFallbackTruthPacketInput,
    row_id: &str,
) -> &'a mut FallbackRow {
    input
        .rows
        .iter_mut()
        .find(|row| row.row_id == row_id)
        .unwrap_or_else(|| panic!("row {row_id} must exist"))
}

fn main() {
    // Checked-in stable artifact packet.
    let packet = WideScopeRefactorFallbackTruthPacket::materialize(baseline_input(PACKET_ID));
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
        "artifacts/language/m5/wide_scope_refactor_fallback_truth_packet.json",
        &serde_json::to_value(&packet).expect("packet serializes"),
    );

    let dir = WIDE_SCOPE_REFACTOR_FALLBACK_TRUTH_FIXTURE_DIR;
    let id = |suffix: &str| format!("packet:m5:wide_scope_refactor_fallback:{suffix}");

    // Baseline stable fixture.
    write_json(
        &format!("{dir}/baseline_stable.json"),
        &fixture(
            "baseline_stable",
            "Baseline stable posture: every M5 artifact family (framework pack, notebook cell, docs artifact, request/structured artifact, config artifact, and generated source) carries a fallback_lane_quality row at certified that names its acting engine, exports an engine-identity label, and binds the refactor class, plus one admission row per fallback dimension: apply posture (co-binding the target scope, the typed completeness label, the confidence tier, and the missing-scope count), impact packet (co-binding the impacted-target and impacted-owner counts, the impact summary, and the missing-scope explanation), reviewer hint (exporting a review anchor and owner hint), rollback path (exporting a checkpoint ref on automatic routes), support-export parity (preserving the refactor lineage and missing-scope explanation with a lineage ref), and provider-disagreement visibility. Every wide-scope lane defaults to a safe fallback (side-branch, worktree, staged, or compare-only); only the narrow, complete, high-confidence docs lane applies all on the live workspace; no writing fallback runs without a safe rollback path; impact packets preserve the missing-scope explanation; support/export preserves the refactor lineage; disagreement keeps the winner and loser both inspectable; and all ten required consumer projections preserve the packet verbatim.",
            baseline_input(&id("baseline_stable")),
            true,
            &[],
        ),
    );

    // Negative cases: each takes the baseline and trips one guardrail.
    type NegativeCase = (
        &'static str,
        &'static str,
        fn(&mut WideScopeRefactorFallbackTruthPacketInput),
        &'static [&'static str],
    );
    let cases: Vec<NegativeCase> = vec![
        (
            "certified_with_unbound_evidence_blocks_stable",
            "A fallback_lane_quality row claims certified while its evidence class is evidence_unbound; the packet emits missing_evidence_class plus certified_with_unbound_binding and blocks the stable claim instead of inheriting an adjacent certified row.",
            |input| {
                row_mut(input, "row:framework:quality").evidence_class = EvidenceClass::EvidenceUnbound;
            },
            &["missing_evidence_class", "certified_with_unbound_binding"],
        ),
        (
            "missing_apply_posture_admission_blocks_stable",
            "A lane claims certified but drops its apply_posture_admission row; the packet emits missing_apply_posture_coverage and blocks the stable claim, so a wide-scope transform cannot run without declaring its safe apply posture, scope, completeness, and confidence.",
            |input| {
                input
                    .rows
                    .retain(|row| row.row_id != "row:framework:apply_posture");
            },
            &["missing_apply_posture_coverage"],
        ),
        (
            "unsafe_apply_all_on_wide_scope_blocks_stable",
            "A wide-scope (multi-file) apply_posture_admission row offers apply_all_on_live_workspace; the packet emits unsafe_apply_all_below_threshold so a wide-scope transform cannot expose an apply-all on the live workspace and must default to a side-branch, worktree, or staged-apply flow instead.",
            |input| {
                row_mut(input, "row:framework:apply_posture").apply_posture_class =
                    ApplyFallbackPostureClass::ApplyAllOnLiveWorkspace;
            },
            &["unsafe_apply_all_below_threshold"],
        ),
        (
            "unsafe_apply_all_on_low_confidence_blocks_stable",
            "A narrow, complete apply_posture_admission row offers apply_all_on_live_workspace at low confidence; the packet emits unsafe_apply_all_below_threshold so a low-confidence transform defaults away from apply-all even when its scope is narrow.",
            |input| {
                row_mut(input, "row:docs:apply_posture").confidence_class =
                    ConfidenceClass::LowConfidence;
            },
            &["unsafe_apply_all_below_threshold"],
        ),
        (
            "scope_completeness_overclaimed_blocks_stable",
            "An apply_posture_admission row labels the preview complete while leaving targets out of scope; the packet emits scope_completeness_overclaimed so a fallback cannot hide an incomplete target set behind a complete label.",
            |input| {
                let row = row_mut(input, "row:framework:apply_posture");
                row.scope_completeness_class = CompletenessClass::Complete;
                row.missing_scope_count = 3;
            },
            &["scope_completeness_overclaimed"],
        ),
        (
            "impact_packet_missing_summary_blocks_stable",
            "An impact_packet_admission row documents impacted targets but attaches no impact summary; the packet emits missing_impact_summary so reviewers always receive an impact summary with the fallback.",
            |input| {
                row_mut(input, "row:framework:impact_packet").impact_summary_present = false;
            },
            &["missing_impact_summary"],
        ),
        (
            "impact_packet_missing_ref_blocks_stable",
            "An impact_packet_admission row documents impacted targets but exports no impact-packet ref; the packet emits missing_impact_packet_ref so the impact packet stays inspectable from review and support surfaces.",
            |input| {
                row_mut(input, "row:framework:impact_packet").impact_packet_ref = None;
            },
            &["missing_impact_packet_ref"],
        ),
        (
            "impact_packet_drops_missing_scope_blocks_stable",
            "The notebook lane left two targets out of scope, but its impact_packet_admission row attaches no missing-scope explanation; the packet emits impact_packet_drops_missing_scope so impact packets always preserve the missing-scope explanation when a fallback leaves targets unloaded or out of scope.",
            |input| {
                row_mut(input, "row:notebook:impact_packet").missing_scope_explanation_present =
                    false;
            },
            &["impact_packet_drops_missing_scope"],
        ),
        (
            "reviewer_hint_missing_anchor_blocks_stable",
            "A reviewer_hint_admission row routes to a reviewer but exports no review-anchor ref; the packet emits missing_review_anchor_ref so a wide-scope fallback always carries a review anchor for the routed reviewer/owner.",
            |input| {
                row_mut(input, "row:framework:reviewer_hint").review_anchor_ref = None;
            },
            &["missing_review_anchor_ref"],
        ),
        (
            "reviewer_hint_missing_owner_hint_blocks_stable",
            "A reviewer_hint_admission row routes to a reviewer but attaches no owner hint; the packet emits missing_owner_hint so reviewer/owner hints are never dropped from the fallback.",
            |input| {
                row_mut(input, "row:framework:reviewer_hint").owner_hint_present = false;
            },
            &["missing_owner_hint"],
        ),
        (
            "writing_fallback_without_safe_rollback_blocks_stable",
            "A lane writes source under a side-branch apply but its rollback_path_admission binds no_safe_rollback_available; the packet emits writing_fallback_without_safe_rollback so a writing fallback can never run without a safe rollback path.",
            |input| {
                row_mut(input, "row:framework:rollback").rollback_path_class =
                    RollbackPathClass::NoSafeRollbackAvailable;
            },
            &["writing_fallback_without_safe_rollback"],
        ),
        (
            "mutating_fallback_without_checkpoint_blocks_stable",
            "A rollback_path_admission row claims an automatic rollback route but exports no checkpoint ref; the packet emits missing_checkpoint_ref so a writing fallback always exports the rollback checkpoint the launch-language refactor safety model requires.",
            |input| {
                row_mut(input, "row:framework:rollback").checkpoint_ref = None;
            },
            &["missing_checkpoint_ref"],
        ),
        (
            "support_export_drops_lineage_blocks_stable",
            "A support_export_parity_admission row drops the refactor lineage; the packet emits support_export_drops_lineage so support and export consumers always preserve the refactor lineage of a low-confidence transform.",
            |input| {
                row_mut(input, "row:framework:support_export_parity").preserves_refactor_lineage =
                    false;
            },
            &["support_export_drops_lineage"],
        ),
        (
            "support_export_missing_lineage_ref_blocks_stable",
            "A support_export_parity_admission row exports no lineage ref; the packet emits missing_lineage_ref so the refactor lineage stays addressable from support and export bundles.",
            |input| {
                row_mut(input, "row:framework:support_export_parity").lineage_ref = None;
            },
            &["missing_lineage_ref"],
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
            "A fallback_lane_quality row names a concrete acting engine but exports no engine-identity label; the packet emits missing_engine_identity_label so the fallback always names which engine planned the transform.",
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
            "A row admits raw source bodies past the boundary; the packet emits raw_source_material_present and blocks the stable claim because raw source bodies, refactor diffs, generated artifact bodies, notebook outputs, provider payloads, secrets, and ambient credentials must never leak through the fallback boundary.",
            |input| {
                row_mut(input, "row:framework:quality").raw_source_material_excluded = false;
            },
            &["raw_source_material_present"],
        ),
        (
            "projection_collapses_apply_posture_vocabulary_blocks_stable",
            "The help_about consumer projection collapses the apply-posture vocabulary; the packet emits apply_posture_vocabulary_collapsed plus consumer_projection_drift and missing_consumer_projection because surfaces MUST preserve the closed apply-posture vocabulary that distinguishes side-branch, worktree, staged, compare-only, blocked, and apply-all-on-live postures.",
            |input| {
                for projection in &mut input.consumer_projections {
                    if projection.consumer_surface == ConsumerSurface::HelpAbout {
                        projection.preserves_apply_posture_vocabulary = false;
                    }
                }
            },
            &[
                "apply_posture_vocabulary_collapsed",
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
