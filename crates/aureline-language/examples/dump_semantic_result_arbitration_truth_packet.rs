//! Regenerates the checked-in semantic-result arbitration truth packet and its
//! protected fixture corpus from the real validator, so the fixtures can never
//! drift from the materialized packet.
//!
//! Run with:
//!
//! ```
//! cargo run -p aureline-language --example dump_semantic_result_arbitration_truth_packet
//! ```
//!
//! It writes:
//!
//! - `artifacts/language/m5/semantic_result_arbitration_truth_packet.json`
//! - `fixtures/language/m5/semantic_result_arbitration_truth_packet/*.json`

use std::path::PathBuf;

use aureline_language::provider_refactor_matrix_truth_packet::{
    CompletenessClass, ConfidenceClass, ConflictClass, ConsumerSurface, DowngradeAutomationClass,
    EvidenceClass, KnownLimitClass, ProviderFamilyClass, SupportClass,
};
use aureline_language::semantic_result_arbitration_truth_packet::{
    AlternateProviderVisibilityClass, AnchorActionClass, ArbitrationBasisClass, ClaimScopeClass,
    CoverageGapClass, DisagreementImpactClass, DisagreementVisibilityClass, FallbackBannerClass,
    InspectorRouteClass, LostGuaranteeClass, ResultArbitrationConsumerProjection,
    ResultArbitrationRow, ResultLaneClass, ResultSurfaceClass, ResultTierClass,
    RetainedGuaranteeClass, SemanticResultArbitrationTruthPacket,
    SemanticResultArbitrationTruthPacketInput, SEMANTIC_RESULT_ARBITRATION_SURFACE_SOURCE_REF,
    SEMANTIC_RESULT_ARBITRATION_TRUTH_DOC_REF, SEMANTIC_RESULT_ARBITRATION_TRUTH_FIXTURE_DIR,
    SEMANTIC_RESULT_ARBITRATION_TRUTH_SCHEMA_REF,
};
use serde_json::{json, Value};

const TS: &str = "2026-06-14T12:00:00Z";
const PACKET_ID: &str = "packet:m5:semantic_result_arbitration:stable";
const WORKFLOW: &str = "workflow.language.semantic_result_arbitration.stable";

fn disclosure(anchor: &str) -> String {
    format!("{SEMANTIC_RESULT_ARBITRATION_TRUTH_DOC_REF}#{anchor}")
}

fn evidence_refs() -> Vec<String> {
    vec![
        SEMANTIC_RESULT_ARBITRATION_TRUTH_DOC_REF.to_owned(),
        SEMANTIC_RESULT_ARBITRATION_TRUTH_FIXTURE_DIR.to_owned(),
    ]
}

/// A fully valid, exact-semantic, single-provider base row.
fn base(
    row_id: &str,
    surface: ResultSurfaceClass,
    lane: ResultLaneClass,
    provider: ProviderFamilyClass,
) -> ResultArbitrationRow {
    ResultArbitrationRow {
        row_id: row_id.to_owned(),
        result_surface_class: surface,
        result_lane_class: lane,
        support_class: SupportClass::Certified,
        acting_provider_family_class: provider,
        arbitration_basis_class: ArbitrationBasisClass::SingleProviderAuthoritative,
        alternate_provider_visibility_class:
            AlternateProviderVisibilityClass::NotApplicableSingleProvider,
        inspector_route_class: InspectorRouteClass::OpenArbitrationInspector,
        conflict_class: ConflictClass::SingleProviderNoConflict,
        disagreement_impact_class: DisagreementImpactClass::None,
        disagreement_visibility_class: DisagreementVisibilityClass::None,
        result_tier_class: ResultTierClass::ExactSemantic,
        fallback_banner_class: FallbackBannerClass::None,
        retained_guarantee_class: RetainedGuaranteeClass::FullSemanticGuarantee,
        lost_guarantee_class: LostGuaranteeClass::NoneLost,
        claim_scope_class: ClaimScopeClass::SingleTarget,
        coverage_gap_class: CoverageGapClass::None,
        anchor_action_class: AnchorActionClass::NavigationOnly,
        preview_completeness_class: CompletenessClass::NotApplicable,
        rollback_checkpoint_ref: None,
        evidence_class: EvidenceClass::FixtureRepoEvidence,
        known_limit_class: KnownLimitClass::NoneDeclared,
        downgrade_automation_class: DowngradeAutomationClass::AutoNarrowOnMissingFixture,
        confidence_class: ConfidenceClass::HighConfidence,
        evidence_refs: evidence_refs(),
        disclosure_ref: Some(disclosure("auto_narrow_on_missing_fixture")),
        raw_source_material_excluded: true,
        secrets_excluded: true,
        ambient_authority_excluded: true,
        captured_at: TS.to_owned(),
    }
}

/// Exact, whole-workspace, all-results semantic row (e.g. all-references).
fn exact_all_results(mut row: ResultArbitrationRow) -> ResultArbitrationRow {
    row.arbitration_basis_class = ArbitrationBasisClass::HighestSemanticAuthority;
    row.claim_scope_class = ClaimScopeClass::WholeWorkspaceAllResults;
    row.anchor_action_class = AnchorActionClass::ResultOnly;
    row.evidence_class = EvidenceClass::ConformanceSuiteEvidence;
    row
}

/// Arbitrated conflict, alternates preserved, target-identity disagreement made
/// visible inline.
fn arbitrated_conflict(mut row: ResultArbitrationRow) -> ResultArbitrationRow {
    row.arbitration_basis_class = ArbitrationBasisClass::FrameworkOverlayPrecedence;
    row.alternate_provider_visibility_class =
        AlternateProviderVisibilityClass::AlternatesPreservedInspectable;
    row.inspector_route_class = InspectorRouteClass::OpenDisagreementDetail;
    row.conflict_class = ConflictClass::ArbitratedWinnerLoserPreserved;
    row.disagreement_impact_class = DisagreementImpactClass::TargetIdentityChanged;
    row.disagreement_visibility_class = DisagreementVisibilityClass::InlineConflictPanel;
    row
}

/// Cached-semantic reuse with a labeled banner.
fn cached_semantic(mut row: ResultArbitrationRow) -> ResultArbitrationRow {
    row.arbitration_basis_class = ArbitrationBasisClass::FreshnessRecency;
    row.inspector_route_class = InspectorRouteClass::OpenProvenancePill;
    row.result_tier_class = ResultTierClass::CachedSemantic;
    row.fallback_banner_class = FallbackBannerClass::CachedSemanticReuse;
    row.retained_guarantee_class = RetainedGuaranteeClass::FileLocalSemantic;
    row.lost_guarantee_class = LostGuaranteeClass::LostCrossFileSemantic;
    row.claim_scope_class = ClaimScopeClass::LoadedSliceResults;
    row.downgrade_automation_class = DowngradeAutomationClass::AutoNarrowOnStaleProvenance;
    row.disclosure_ref = Some(disclosure("auto_narrow_on_stale_provenance"));
    row
}

/// Semantic-to-text fallback banner with the lost guarantee recorded.
fn text_fallback(mut row: ResultArbitrationRow) -> ResultArbitrationRow {
    row.acting_provider_family_class = ProviderFamilyClass::TextFallback;
    row.arbitration_basis_class = ArbitrationBasisClass::NarrowedNoSemanticWinner;
    row.inspector_route_class = InspectorRouteClass::OpenProvenancePill;
    row.result_tier_class = ResultTierClass::TextLexical;
    row.fallback_banner_class = FallbackBannerClass::SemanticToTextFallback;
    row.retained_guarantee_class = RetainedGuaranteeClass::LexicalMatchOnly;
    row.lost_guarantee_class = LostGuaranteeClass::LostAllReferencesGuarantee;
    row.claim_scope_class = ClaimScopeClass::ActiveFileResults;
    row.downgrade_automation_class = DowngradeAutomationClass::AutoNarrowOnProviderUnavailable;
    row.disclosure_ref = Some(disclosure("auto_narrow_on_provider_unavailable"));
    row.known_limit_class = KnownLimitClass::ProviderFamilySubsetOnly;
    row
}

/// Heuristic / structural fallback with an unresolved disagreement surfaced.
fn heuristic_structural(mut row: ResultArbitrationRow) -> ResultArbitrationRow {
    row.acting_provider_family_class = ProviderFamilyClass::SemanticGraphLane;
    row.arbitration_basis_class = ArbitrationBasisClass::OnlyAdmissibleProvider;
    row.alternate_provider_visibility_class =
        AlternateProviderVisibilityClass::AlternatesPreservedInspectable;
    row.inspector_route_class = InspectorRouteClass::OpenDisagreementDetail;
    row.conflict_class = ConflictClass::UnresolvedDisagreementSurfaced;
    row.disagreement_impact_class = DisagreementImpactClass::ScopeCoverageChanged;
    row.disagreement_visibility_class = DisagreementVisibilityClass::SidePanelInspector;
    row.result_tier_class = ResultTierClass::HeuristicStructural;
    row.fallback_banner_class = FallbackBannerClass::SemanticToHeuristicFallback;
    row.retained_guarantee_class = RetainedGuaranteeClass::StructuralMatchOnly;
    row.lost_guarantee_class = LostGuaranteeClass::LostSemanticTargetIdentity;
    row.claim_scope_class = ClaimScopeClass::GeneratedExcludedResults;
    row.coverage_gap_class = CoverageGapClass::GeneratedOnlyEdgesSkipped;
    row.downgrade_automation_class = DowngradeAutomationClass::AutoNarrowOnConflictUnresolved;
    row.disclosure_ref = Some(disclosure("auto_narrow_on_conflict_unresolved"));
    row.known_limit_class = KnownLimitClass::DiagnosticSourceSubsetOnly;
    row
}

/// Partial-semantic result anchoring a mutating completion follow-up with a
/// typed preview completeness and a rollback checkpoint.
fn partial_mutating(mut row: ResultArbitrationRow) -> ResultArbitrationRow {
    row.arbitration_basis_class = ArbitrationBasisClass::HighestSemanticAuthority;
    row.inspector_route_class = InspectorRouteClass::OpenProvenancePill;
    row.result_tier_class = ResultTierClass::PartialSemantic;
    row.fallback_banner_class = FallbackBannerClass::SemanticToFileLocalFallback;
    row.retained_guarantee_class = RetainedGuaranteeClass::FileLocalSemantic;
    row.lost_guarantee_class = LostGuaranteeClass::LostWholeWorkspaceScope;
    row.claim_scope_class = ClaimScopeClass::LoadedSliceResults;
    row.coverage_gap_class = CoverageGapClass::UnloadedSlicesSkipped;
    row.anchor_action_class = AnchorActionClass::MutatingFollowupPreview;
    row.preview_completeness_class = CompletenessClass::Partial;
    row.rollback_checkpoint_ref = Some("checkpoint:rollback:semantic_result:01".to_owned());
    row.downgrade_automation_class = DowngradeAutomationClass::AutoNarrowOnPreviewPartial;
    row.disclosure_ref = Some(disclosure("auto_narrow_on_preview_partial"));
    row.known_limit_class = KnownLimitClass::SemanticModeSubsetOnly;
    row
}

fn provider_for(surface: ResultSurfaceClass) -> ProviderFamilyClass {
    match surface {
        ResultSurfaceClass::SearchSurface => ProviderFamilyClass::SemanticGraphLane,
        ResultSurfaceClass::DocsSurface => ProviderFamilyClass::LspProvider,
        ResultSurfaceClass::FrameworkSurface => ProviderFamilyClass::FrameworkAnalyzer,
        ResultSurfaceClass::NotebookSurface => ProviderFamilyClass::NotebookAdapter,
        ResultSurfaceClass::GeneratedSourceSurface => ProviderFamilyClass::GeneratedSourceBridge,
    }
}

/// Builds the 5x4 baseline matrix of result-arbitration rows.
fn baseline_rows() -> Vec<ResultArbitrationRow> {
    use ResultLaneClass::*;
    use ResultSurfaceClass::*;
    // archetype selector per (surface, lane), chosen to cover the full
    // vocabulary while keeping every row valid.
    type Arch = fn(ResultArbitrationRow) -> ResultArbitrationRow;
    let identity: Arch = |row| row;
    let plan: [(ResultSurfaceClass, ResultLaneClass, Arch); 20] = [
        (SearchSurface, Definition, identity),
        (SearchSurface, References, text_fallback),
        (SearchSurface, Hierarchy, heuristic_structural),
        (SearchSurface, Completion, cached_semantic),
        (DocsSurface, Definition, identity),
        (DocsSurface, References, arbitrated_conflict),
        (DocsSurface, Hierarchy, exact_all_results),
        (DocsSurface, Completion, text_fallback),
        (FrameworkSurface, Definition, arbitrated_conflict),
        (FrameworkSurface, References, exact_all_results),
        (FrameworkSurface, Hierarchy, cached_semantic),
        (FrameworkSurface, Completion, partial_mutating),
        (NotebookSurface, Definition, cached_semantic),
        (NotebookSurface, References, heuristic_structural),
        (NotebookSurface, Hierarchy, identity),
        (NotebookSurface, Completion, identity),
        (GeneratedSourceSurface, Definition, identity),
        (GeneratedSourceSurface, References, partial_mutating),
        (GeneratedSourceSurface, Hierarchy, text_fallback),
        (GeneratedSourceSurface, Completion, arbitrated_conflict),
    ];
    plan.into_iter()
        .map(|(surface, lane, arch)| {
            let row_id = format!("row:{}:{}", surface.as_str(), lane.as_str());
            arch(base(&row_id, surface, lane, provider_for(surface)))
        })
        .collect()
}

fn projections(packet_id: &str) -> Vec<ResultArbitrationConsumerProjection> {
    ConsumerSurface::REQUIRED
        .into_iter()
        .map(|surface| ResultArbitrationConsumerProjection {
            consumer_surface: surface,
            projection_ref: format!(
                "projection:semantic_result_arbitration:{}",
                surface.as_str()
            ),
            surface_packet_id_ref: packet_id.to_owned(),
            rendered_at: TS.to_owned(),
            preserves_same_packet: true,
            preserves_result_surface_vocabulary: true,
            preserves_result_lane_vocabulary: true,
            preserves_support_class_vocabulary: true,
            preserves_provider_family_vocabulary: true,
            preserves_arbitration_basis_vocabulary: true,
            preserves_alternate_provider_visibility_vocabulary: true,
            preserves_inspector_route_vocabulary: true,
            preserves_conflict_vocabulary: true,
            preserves_disagreement_impact_vocabulary: true,
            preserves_disagreement_visibility_vocabulary: true,
            preserves_result_tier_vocabulary: true,
            preserves_fallback_banner_vocabulary: true,
            preserves_retained_guarantee_vocabulary: true,
            preserves_lost_guarantee_vocabulary: true,
            preserves_claim_scope_vocabulary: true,
            preserves_coverage_gap_vocabulary: true,
            preserves_anchor_action_vocabulary: true,
            preserves_completeness_vocabulary: true,
            preserves_evidence_class_vocabulary: true,
            preserves_known_limit_vocabulary: true,
            preserves_downgrade_automation_vocabulary: true,
            supports_json_export: true,
            raw_private_material_excluded: true,
            ambient_authority_excluded: true,
        })
        .collect()
}

fn baseline_input(packet_id: &str) -> SemanticResultArbitrationTruthPacketInput {
    SemanticResultArbitrationTruthPacketInput {
        packet_id: packet_id.to_owned(),
        workflow_or_surface_id: WORKFLOW.to_owned(),
        generated_at: TS.to_owned(),
        covered_surfaces: ResultSurfaceClass::REQUIRED.to_vec(),
        rows: baseline_rows(),
        consumer_projections: projections(packet_id),
        source_contract_refs: vec![
            SEMANTIC_RESULT_ARBITRATION_TRUTH_SCHEMA_REF.to_owned(),
            SEMANTIC_RESULT_ARBITRATION_SURFACE_SOURCE_REF.to_owned(),
        ],
    }
}

fn token_array(tokens: Vec<&'static str>) -> Value {
    Value::Array(tokens.into_iter().map(|t| json!(t)).collect())
}

fn expect_block(
    packet: &SemanticResultArbitrationTruthPacket,
    export_safe: bool,
    expected_finding_kinds: &[&str],
) -> Value {
    json!({
        "promotion_state": packet.promotion_state.as_str(),
        "validation_finding_count": packet.validation_findings.len(),
        "row_count": packet.rows.len(),
        "result_surface_tokens": token_array(packet.result_surface_tokens()),
        "result_lane_tokens": token_array(packet.result_lane_tokens()),
        "support_class_tokens": token_array(packet.support_class_tokens()),
        "provider_family_tokens": token_array(packet.provider_family_tokens()),
        "arbitration_basis_tokens": token_array(packet.arbitration_basis_tokens()),
        "alternate_provider_visibility_tokens": token_array(packet.alternate_provider_visibility_tokens()),
        "inspector_route_tokens": token_array(packet.inspector_route_tokens()),
        "conflict_tokens": token_array(packet.conflict_tokens()),
        "disagreement_impact_tokens": token_array(packet.disagreement_impact_tokens()),
        "disagreement_visibility_tokens": token_array(packet.disagreement_visibility_tokens()),
        "result_tier_tokens": token_array(packet.result_tier_tokens()),
        "fallback_banner_tokens": token_array(packet.fallback_banner_tokens()),
        "retained_guarantee_tokens": token_array(packet.retained_guarantee_tokens()),
        "lost_guarantee_tokens": token_array(packet.lost_guarantee_tokens()),
        "claim_scope_tokens": token_array(packet.claim_scope_tokens()),
        "coverage_gap_tokens": token_array(packet.coverage_gap_tokens()),
        "anchor_action_tokens": token_array(packet.anchor_action_tokens()),
        "completeness_tokens": token_array(packet.completeness_tokens()),
        "evidence_class_tokens": token_array(packet.evidence_class_tokens()),
        "known_limit_tokens": token_array(packet.known_limit_tokens()),
        "downgrade_automation_tokens": token_array(packet.downgrade_automation_tokens()),
        "support_export_safe": export_safe,
        "expected_finding_kinds": expected_finding_kinds,
    })
}

fn fixture(
    case_name: &str,
    scenario: &str,
    input: SemanticResultArbitrationTruthPacketInput,
    export_safe: bool,
    expected_finding_kinds: &[&str],
) -> Value {
    let packet = SemanticResultArbitrationTruthPacket::materialize(input.clone());
    json!({
        "record_kind": "semantic_result_arbitration_truth_stable_case",
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

fn main() {
    // Checked-in stable artifact packet.
    let packet = SemanticResultArbitrationTruthPacket::materialize(baseline_input(PACKET_ID));
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
        "artifacts/language/m5/semantic_result_arbitration_truth_packet.json",
        &serde_json::to_value(&packet).expect("packet serializes"),
    );

    let dir = SEMANTIC_RESULT_ARBITRATION_TRUTH_FIXTURE_DIR;
    let id = |suffix: &str| format!("packet:m5:semantic_result_arbitration:{suffix}");

    // Baseline stable fixture.
    write_json(
        &format!("{dir}/baseline_stable.json"),
        &fixture(
            "baseline_stable",
            "Baseline stable posture: every result surface (search, docs, framework, notebook, generated source) carries definition, references, hierarchy, and completion rows. Each row names the acting provider that won and the basis it won on, keeps the losing providers inspectable when a disagreement is arbitrated or surfaced unresolved, opens an arbitration / disagreement / provenance detail route instead of an opaque spinner, surfaces a visible detail path whenever the conflict changes target identity, scope coverage, or refactor safety, and carries a semantic-to-text / heuristic / file-local / cached fallback banner that records both the guarantee that remains and the guarantee that was lost whenever the answer degraded below exact semantic. Whole-workspace / all-results wording appears only on exact semantic rows with no coverage gap, a text or heuristic answer never claims whole-workspace scope, a mutating completion follow-up binds a typed preview completeness and a rollback checkpoint, and all ten required consumer projections preserve the packet verbatim.",
            baseline_input(&id("baseline_stable")),
            true,
            &[],
        ),
    );

    // Negative cases: each takes the baseline and trips a guardrail on one row.
    // (case_name, scenario, single-row mutation, expected finding kinds)
    type NegativeCase = (
        &'static str,
        &'static str,
        fn(&mut ResultArbitrationRow),
        &'static [&'static str],
    );
    let cases: Vec<NegativeCase> = vec![
        (
            "losing_provider_collapsed_blocks_stable",
            "A disagreement is arbitrated but the losing provider is collapsed into ranking-only output, so what the alternate provider said is no longer inspectable. The row narrows below stable.",
            |row| {
                row.conflict_class = ConflictClass::ArbitratedWinnerLoserPreserved;
                row.alternate_provider_visibility_class =
                    AlternateProviderVisibilityClass::AlternatesCollapsedRankingOnly;
                row.inspector_route_class = InspectorRouteClass::OpenDisagreementDetail;
                row.disagreement_impact_class = DisagreementImpactClass::FreshnessOnly;
                row.disagreement_visibility_class = DisagreementVisibilityClass::InlineConflictPanel;
            },
            &["losing_provider_collapsed"],
        ),
        (
            "material_conflict_without_detail_path_blocks_stable",
            "Providers disagree in a way that changes scope coverage, but the row offers no visible disagreement detail path. The row narrows below stable.",
            |row| {
                row.conflict_class = ConflictClass::ArbitratedWinnerLoserPreserved;
                row.alternate_provider_visibility_class =
                    AlternateProviderVisibilityClass::AlternatesPreservedInspectable;
                row.disagreement_impact_class = DisagreementImpactClass::ScopeCoverageChanged;
                row.disagreement_visibility_class = DisagreementVisibilityClass::None;
                row.inspector_route_class = InspectorRouteClass::NotApplicable;
            },
            &["disagreement_detail_path_missing"],
        ),
        (
            "opaque_spinner_route_blocks_stable",
            "An opaque loading spinner stands in for a real inspection route, hiding why the winning provider was chosen. The row narrows below stable.",
            |row| {
                row.inspector_route_class = InspectorRouteClass::OpaqueSpinner;
            },
            &["opaque_inspector_route"],
        ),
        (
            "silently_fused_conflict_blocks_stable",
            "A target-identity conflict is silently fused into an exact answer with no visible disagreement, so the user cannot tell two providers disagreed about which target the answer points to. The row narrows below stable.",
            |row| {
                row.conflict_class = ConflictClass::ArbitratedWinnerLoserPreserved;
                row.alternate_provider_visibility_class =
                    AlternateProviderVisibilityClass::AlternatesPreservedInspectable;
                row.disagreement_impact_class = DisagreementImpactClass::TargetIdentityChanged;
                row.disagreement_visibility_class = DisagreementVisibilityClass::None;
                row.inspector_route_class = InspectorRouteClass::OpenDisagreementDetail;
            },
            &["silent_fusion_of_conflict", "disagreement_detail_path_missing"],
        ),
        (
            "fallback_banner_missing_blocks_stable",
            "A result degraded to text / lexical behavior but shows no fallback banner and records no lost guarantee, so the surface keeps claiming a semantic answer. The row narrows below stable.",
            |row| {
                row.result_tier_class = ResultTierClass::TextLexical;
                row.retained_guarantee_class = RetainedGuaranteeClass::LexicalMatchOnly;
                row.claim_scope_class = ClaimScopeClass::ActiveFileResults;
                row.fallback_banner_class = FallbackBannerClass::None;
                row.lost_guarantee_class = LostGuaranteeClass::NoneLost;
            },
            &["fallback_banner_missing"],
        ),
        (
            "exact_result_with_fallback_banner_blocks_stable",
            "An exact semantic result carries a fallback banner and a lost guarantee, which would mislabel a complete answer as degraded. The row narrows below stable.",
            |row| {
                row.fallback_banner_class = FallbackBannerClass::SemanticToTextFallback;
                row.lost_guarantee_class = LostGuaranteeClass::LostCrossFileSemantic;
            },
            &["fallback_banner_on_exact_result"],
        ),
        (
            "overclaimed_all_references_on_lexical_blocks_stable",
            "A text / lexical result claims whole-workspace all-references scope when only lexical evidence exists. The surface must stop claiming all-references; the row narrows below stable.",
            |row| {
                row.result_tier_class = ResultTierClass::TextLexical;
                row.retained_guarantee_class = RetainedGuaranteeClass::LexicalMatchOnly;
                row.fallback_banner_class = FallbackBannerClass::SemanticToTextFallback;
                row.lost_guarantee_class = LostGuaranteeClass::LostAllReferencesGuarantee;
                row.claim_scope_class = ClaimScopeClass::WholeWorkspaceAllResults;
            },
            &["overclaimed_scope_on_lexical_evidence"],
        ),
        (
            "whole_workspace_wording_with_excluded_roots_blocks_stable",
            "A partial-semantic result keeps whole-workspace wording after excluded roots were skipped. The wording must narrow to the scanned scope; the row narrows below stable.",
            |row| {
                row.result_tier_class = ResultTierClass::PartialSemantic;
                row.fallback_banner_class = FallbackBannerClass::SemanticToFileLocalFallback;
                row.lost_guarantee_class = LostGuaranteeClass::LostWholeWorkspaceScope;
                row.retained_guarantee_class = RetainedGuaranteeClass::FileLocalSemantic;
                row.claim_scope_class = ClaimScopeClass::WholeWorkspaceAllResults;
                row.coverage_gap_class = CoverageGapClass::ExcludedRootsSkipped;
            },
            &["whole_workspace_wording_with_coverage_gap"],
        ),
        (
            "mutating_followup_without_rollback_blocks_stable",
            "A completion result anchors a mutating follow-up that bypasses the rollback checkpoint required by the launch-language refactor safety model. The row narrows below stable.",
            |row| {
                row.anchor_action_class = AnchorActionClass::MutatingFollowupPreview;
                row.preview_completeness_class = CompletenessClass::Complete;
                row.rollback_checkpoint_ref = None;
            },
            &["mutating_anchor_bypasses_preview"],
        ),
        (
            "certified_with_unbound_evidence_blocks_stable",
            "A row claims certified while leaving its evidence binding unbound, so the certification rests on nothing. The validator narrows below stable instead of inheriting an adjacent certified row.",
            |row| {
                row.evidence_class = EvidenceClass::EvidenceUnbound;
            },
            &["missing_evidence_class", "certified_with_unbound_binding"],
        ),
        (
            "raw_source_material_blocks_stable",
            "A row admits raw source bodies past the metadata-only boundary. The row narrows below stable.",
            |row| {
                row.raw_source_material_excluded = false;
            },
            &["raw_source_material_present"],
        ),
        (
            "narrowed_row_missing_disclosure_ref_blocks_stable",
            "A row narrowed below certified carries no disclosure ref, so the narrowing is undisclosed. The row narrows below stable.",
            |row| {
                row.support_class = SupportClass::CertifiedBelow;
                row.downgrade_automation_class = DowngradeAutomationClass::None;
                row.known_limit_class = KnownLimitClass::NoneDeclared;
                row.disclosure_ref = None;
            },
            &["narrowed_row_missing_disclosure_ref"],
        ),
    ];

    for (case_name, scenario, mutate, kinds) in cases {
        let mut input = baseline_input(&id(case_name));
        mutate(&mut input.rows[0]);
        write_json(
            &format!("{dir}/{case_name}.json"),
            &fixture(case_name, scenario, input, false, kinds),
        );
    }
}
