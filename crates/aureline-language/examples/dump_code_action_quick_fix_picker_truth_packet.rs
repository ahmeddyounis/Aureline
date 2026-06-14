//! Regenerates the checked-in code-action / quick-fix picker truth packet and
//! its protected fixture corpus from the real validator, so the fixtures can
//! never drift from the materialized packet.
//!
//! Run with:
//!
//! ```
//! cargo run -p aureline-language --example dump_code_action_quick_fix_picker_truth_packet
//! ```
//!
//! It writes:
//!
//! - `artifacts/language/m5/code_action_quick_fix_picker_truth_packet.json`
//! - `fixtures/language/m5/code_action_quick_fix_picker_truth_packet/*.json`

use std::path::PathBuf;

use aureline_language::code_action_quick_fix_picker_truth_packet::{
    ApplyPostureClass, ArtifactFamilyLaneClass, CodeActionQuickFixPickerTruthPacket,
    CodeActionQuickFixPickerTruthPacketInput, DisagreementVisibilityClass, FallbackPathClass,
    MutationScopeClass, PickerConsumerProjection, PickerRow, PickerRowClass, ValidationHookClass,
    CODE_ACTION_QUICK_FIX_PICKER_TRUTH_DOC_REF, CODE_ACTION_QUICK_FIX_PICKER_TRUTH_FIXTURE_DIR,
    CODE_ACTION_QUICK_FIX_PICKER_TRUTH_SCHEMA_REF,
};
use aureline_language::provider_refactor_matrix_truth_packet::{
    CompletenessClass, ConfidenceClass, ConsumerSurface, DowngradeAutomationClass, EvidenceClass,
    GeneratedArtifactPolicyClass, KnownLimitClass, ProviderFamilyClass, RollbackPathClass,
    SupportClass,
};
use serde_json::{json, Value};

const TS: &str = "2026-06-14T12:00:00Z";
const PACKET_ID: &str = "packet:m5:code_action_quick_fix_picker:stable";
const WORKFLOW: &str = "workflow.language.code_action_quick_fix_picker.stable";

fn disclosure(anchor: &str) -> String {
    format!("{CODE_ACTION_QUICK_FIX_PICKER_TRUTH_DOC_REF}#{anchor}")
}

fn evidence_refs() -> Vec<String> {
    vec![
        CODE_ACTION_QUICK_FIX_PICKER_TRUTH_DOC_REF.to_owned(),
        CODE_ACTION_QUICK_FIX_PICKER_TRUTH_FIXTURE_DIR.to_owned(),
    ]
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
        evidence_refs: evidence_refs(),
        disclosure_ref: Some(disclosure("auto_narrow_on_missing_fixture")),
        acting_provider_label: None,
        preview_hash_ref: None,
        checkpoint_ref: None,
        raw_source_material_excluded: true,
        secrets_excluded: true,
        ambient_authority_excluded: true,
        captured_at: TS.to_owned(),
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
    quality.disclosure_ref = Some(disclosure("auto_block_on_missing_evidence"));
    quality.evidence_refs = evidence_refs();
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

fn baseline_rows() -> Vec<PickerRow> {
    let mut rows = Vec::new();
    for spec in lane_specs() {
        rows.extend(lane_rows(&spec));
    }
    rows
}

fn projections(packet_id: &str) -> Vec<PickerConsumerProjection> {
    ConsumerSurface::REQUIRED
        .into_iter()
        .map(|surface| PickerConsumerProjection {
            consumer_surface: surface,
            projection_ref: format!(
                "projection:code_action_quick_fix_picker:{}",
                surface.as_str()
            ),
            picker_packet_id_ref: packet_id.to_owned(),
            rendered_at: TS.to_owned(),
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
        })
        .collect()
}

fn baseline_input(packet_id: &str) -> CodeActionQuickFixPickerTruthPacketInput {
    CodeActionQuickFixPickerTruthPacketInput {
        packet_id: packet_id.to_owned(),
        workflow_or_surface_id: WORKFLOW.to_owned(),
        generated_at: TS.to_owned(),
        covered_lanes: ArtifactFamilyLaneClass::REQUIRED.to_vec(),
        rows: baseline_rows(),
        consumer_projections: projections(packet_id),
        source_contract_refs: vec![
            CODE_ACTION_QUICK_FIX_PICKER_TRUTH_SCHEMA_REF.to_owned(),
            CODE_ACTION_QUICK_FIX_PICKER_TRUTH_DOC_REF.to_owned(),
        ],
    }
}

fn token_array(tokens: Vec<&'static str>) -> Value {
    Value::Array(tokens.into_iter().map(|t| json!(t)).collect())
}

fn expect_block(
    packet: &CodeActionQuickFixPickerTruthPacket,
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
        "acting_provider_tokens": token_array(packet.acting_provider_tokens()),
        "apply_posture_tokens": token_array(packet.apply_posture_tokens()),
        "mutation_scope_tokens": token_array(packet.mutation_scope_tokens()),
        "validation_hook_tokens": token_array(packet.validation_hook_tokens()),
        "generated_asset_policy_tokens": token_array(packet.generated_asset_policy_tokens()),
        "fallback_path_tokens": token_array(packet.fallback_path_tokens()),
        "disagreement_visibility_tokens": token_array(packet.disagreement_visibility_tokens()),
        "rollback_checkpoint_tokens": token_array(packet.rollback_checkpoint_tokens()),
        "preview_completeness_tokens": token_array(packet.preview_completeness_tokens()),
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
    input: CodeActionQuickFixPickerTruthPacketInput,
    export_safe: bool,
    expected_finding_kinds: &[&str],
) -> Value {
    let packet = CodeActionQuickFixPickerTruthPacket::materialize(input.clone());
    json!({
        "record_kind": "code_action_quick_fix_picker_truth_stable_case",
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
    input: &'a mut CodeActionQuickFixPickerTruthPacketInput,
    row_id: &str,
) -> &'a mut PickerRow {
    input
        .rows
        .iter_mut()
        .find(|row| row.row_id == row_id)
        .unwrap_or_else(|| panic!("row {row_id} must exist"))
}

fn main() {
    // Checked-in stable artifact packet.
    let packet = CodeActionQuickFixPickerTruthPacket::materialize(baseline_input(PACKET_ID));
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
        "artifacts/language/m5/code_action_quick_fix_picker_truth_packet.json",
        &serde_json::to_value(&packet).expect("packet serializes"),
    );

    let dir = CODE_ACTION_QUICK_FIX_PICKER_TRUTH_FIXTURE_DIR;
    let id = |suffix: &str| format!("packet:m5:code_action_quick_fix_picker:{suffix}");

    // Baseline stable fixture.
    write_json(
        &format!("{dir}/baseline_stable.json"),
        &fixture(
            "baseline_stable",
            "Baseline stable posture: every M5 artifact family (framework pack, notebook cell, docs artifact, request/structured artifact, config artifact, and generated source) carries a picker_lane_quality row at certified that names its acting provider and exports an acting-provider label, plus one admission row per picker dimension: apply posture (co-binding mutation scope, validation hook, typed preview completeness, the exported preview hash, and the exported rollback checkpoint ref), generated-asset policy, fallback / manual path, provider-disagreement visibility, and rollback checkpoint route. Every mutating apply states whether it is inline-safe, preview-required, compare-only, or blocked-pending-review; one-click inline apply never widens into generated or structured artifacts without a preview; a preview-required action exports a preview hash and a typed completeness label; a mutating apply exports a rollback checkpoint ref; provider disagreement keeps the winner and loser both inspectable rather than collapsing to ranking-only; manual-fix guidance stays visible; and all ten required consumer projections preserve the packet verbatim.",
            baseline_input(&id("baseline_stable")),
            true,
            &[],
        ),
    );

    // Negative cases: each takes the baseline and trips one guardrail.
    type NegativeCase = (
        &'static str,
        &'static str,
        fn(&mut CodeActionQuickFixPickerTruthPacketInput),
        &'static [&'static str],
    );
    let cases: Vec<NegativeCase> = vec![
        (
            "certified_with_unbound_evidence_blocks_stable",
            "A picker_lane_quality row claims certified while its evidence class is evidence_unbound; the packet emits missing_evidence_class plus certified_with_unbound_binding and blocks the stable claim instead of inheriting an adjacent certified row.",
            |input| {
                row_mut(input, "row:framework:quality").evidence_class = EvidenceClass::EvidenceUnbound;
            },
            &["missing_evidence_class", "certified_with_unbound_binding"],
        ),
        (
            "missing_apply_posture_admission_blocks_stable",
            "A lane claims certified but drops its apply_posture_admission row; the packet emits missing_apply_posture_coverage and blocks the stable claim, so an artifact family cannot offer a mutating code action whose apply posture was never enumerated.",
            |input| {
                input
                    .rows
                    .retain(|row| row.row_id != "row:framework:apply_posture");
            },
            &["missing_apply_posture_coverage"],
        ),
        (
            "inline_apply_widens_scope_without_preview_blocks_stable",
            "An apply_posture_admission row sets posture inline_safe while its mutation scope reaches into generated artifacts; the packet emits inline_apply_widens_scope_without_preview so one-click fixes cannot widen into generated or structured artifacts without a typed preview.",
            |input| {
                let row = row_mut(input, "row:framework:apply_posture");
                row.apply_posture_class = ApplyPostureClass::InlineSafe;
                row.mutation_scope_class = MutationScopeClass::GeneratedArtifactScope;
            },
            &["inline_apply_widens_scope_without_preview"],
        ),
        (
            "preview_required_without_preview_hash_blocks_stable",
            "A preview-required apply_posture_admission row exports no preview hash ref; the packet emits missing_preview_hash_ref so a previewable action always exports the preview the action packet must carry.",
            |input| {
                row_mut(input, "row:framework:apply_posture").preview_hash_ref = None;
            },
            &["missing_preview_hash_ref"],
        ),
        (
            "mutating_action_without_checkpoint_blocks_stable",
            "A mutating, applying apply_posture_admission row exports no rollback checkpoint ref; the packet emits missing_checkpoint_ref so AI-planned, schema/codegen, organize-imports, and notebook/generated edits cannot bypass the rollback checkpoint the launch-language refactor safety model requires.",
            |input| {
                row_mut(input, "row:framework:apply_posture").checkpoint_ref = None;
            },
            &["missing_checkpoint_ref"],
        ),
        (
            "missing_acting_provider_label_blocks_stable",
            "A picker_lane_quality row names a concrete acting provider but exports no acting-provider label; the packet emits missing_acting_provider_label so the picker entry always names which engine is acting.",
            |input| {
                row_mut(input, "row:framework:quality").acting_provider_label = None;
            },
            &["missing_acting_provider_label"],
        ),
        (
            "disagreement_collapsed_to_ranking_only_blocks_stable",
            "A provider_disagreement_admission row collapses the disagreement into ranking-only output; the packet emits disagreement_collapsed_to_ranking_only so the losing provider and downgrade reason stay inspectable rather than being hidden behind a single ranked result.",
            |input| {
                row_mut(input, "row:framework:disagreement").disagreement_visibility_class =
                    DisagreementVisibilityClass::RankingOnlyCollapsed;
            },
            &["disagreement_collapsed_to_ranking_only"],
        ),
        (
            "manual_fix_guidance_hidden_blocks_stable",
            "A fallback_path_admission row goes low confidence yet offers a none-needed fallback; the packet emits manual_fix_guidance_hidden so a partial, stale, or low-confidence acting provider can never hide its manual-fix or repair guidance.",
            |input| {
                let row = row_mut(input, "row:framework:fallback");
                row.support_class = SupportClass::CertifiedBelow;
                row.confidence_class = ConfidenceClass::LowConfidence;
                row.fallback_path_class = FallbackPathClass::NoneNeeded;
                row.disclosure_ref = Some(disclosure("auto_narrow_on_missing_fixture"));
            },
            &["manual_fix_guidance_hidden"],
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
            "A row admits raw source bodies past the boundary; the packet emits raw_source_material_present and blocks the stable claim because raw source bodies, refactor diffs, generated artifact bodies, notebook outputs, provider payloads, secrets, and ambient credentials must never leak through the picker boundary.",
            |input| {
                row_mut(input, "row:framework:quality").raw_source_material_excluded = false;
            },
            &["raw_source_material_present"],
        ),
        (
            "projection_collapses_apply_posture_vocabulary_blocks_stable",
            "The help_about consumer projection collapses the apply-posture vocabulary; the packet emits apply_posture_vocabulary_collapsed plus consumer_projection_drift and missing_consumer_projection because surfaces MUST preserve the closed apply-posture vocabulary that distinguishes inline-safe, preview-required, compare-only, and blocked-pending-review.",
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
