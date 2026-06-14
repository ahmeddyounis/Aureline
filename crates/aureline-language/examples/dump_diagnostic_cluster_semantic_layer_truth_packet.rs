//! Regenerates the checked-in diagnostic-cluster semantic-layer truth packet
//! and its protected fixture corpus from the real validator, so the fixtures
//! can never drift from the materialized packet.
//!
//! Run with:
//!
//! ```
//! cargo run -p aureline-language --example dump_diagnostic_cluster_semantic_layer_truth_packet
//! ```
//!
//! It writes:
//!
//! - `artifacts/language/m5/diagnostic_cluster_semantic_layer_truth_packet.json`
//! - `fixtures/language/m5/diagnostic_cluster_semantic_layer_truth_packet/*.json`

use std::path::PathBuf;

use aureline_language::diagnostic_cluster_semantic_layer_truth_packet::{
    ClusterLaneClass, ClusterProvenanceClass, DetailSheetRouteClass,
    DiagnosticClusterConsumerProjection, DiagnosticClusterRow,
    DiagnosticClusterSemanticLayerTruthPacket, DiagnosticClusterSemanticLayerTruthPacketInput,
    FixOfferClass, FreshnessClass, ProviderDisagreementVisibilityClass, ScopeLabelClass,
    SemanticLayerBannerClass, SourceDifferentiationClass, SurfaceClass,
    DIAGNOSTIC_CLUSTER_SEMANTIC_LAYER_MATRIX_SOURCE_REF,
    DIAGNOSTIC_CLUSTER_SEMANTIC_LAYER_TRUTH_DOC_REF,
    DIAGNOSTIC_CLUSTER_SEMANTIC_LAYER_TRUTH_FIXTURE_DIR,
    DIAGNOSTIC_CLUSTER_SEMANTIC_LAYER_TRUTH_SCHEMA_REF,
};
use aureline_language::provider_refactor_matrix_truth_packet::{
    CompletenessClass, ConfidenceClass, ConflictClass, ConsumerSurface, DiagnosticSourceClass,
    DowngradeAutomationClass, EvidenceClass, KnownLimitClass, ProviderFamilyClass, SupportClass,
};
use serde_json::{json, Value};

const TS: &str = "2026-06-14T12:00:00Z";
const PACKET_ID: &str = "packet:m5:diagnostic_cluster_semantic_layer:stable";
const WORKFLOW: &str = "workflow.language.diagnostic_cluster_semantic_layer.stable";

fn disclosure(anchor: &str) -> String {
    format!("{DIAGNOSTIC_CLUSTER_SEMANTIC_LAYER_TRUTH_DOC_REF}#{anchor}")
}

fn evidence_refs() -> Vec<String> {
    vec![
        DIAGNOSTIC_CLUSTER_SEMANTIC_LAYER_TRUTH_DOC_REF.to_owned(),
        DIAGNOSTIC_CLUSTER_SEMANTIC_LAYER_TRUTH_FIXTURE_DIR.to_owned(),
    ]
}

/// A fully valid, single-source, single-provider, live-semantic base row.
fn base(
    row_id: &str,
    surface: SurfaceClass,
    lane: ClusterLaneClass,
    provider: ProviderFamilyClass,
    source: DiagnosticSourceClass,
) -> DiagnosticClusterRow {
    DiagnosticClusterRow {
        row_id: row_id.to_owned(),
        surface_class: surface,
        cluster_lane_class: lane,
        support_class: SupportClass::Certified,
        diagnostic_source_classes: vec![source],
        cluster_provenance_class: ClusterProvenanceClass::SingleProviderCluster,
        source_differentiation_class: SourceDifferentiationClass::SingleSourceNotApplicable,
        preserves_per_provider_detail: true,
        preserves_timestamps_epochs: true,
        preserves_suppression_baseline: true,
        preserves_related_evidence: true,
        detail_sheet_route_class: DetailSheetRouteClass::OpenClusterDetailSheet,
        semantic_layer_banner_class: SemanticLayerBannerClass::Semantic,
        freshness_class: FreshnessClass::Live,
        scope_label_class: ScopeLabelClass::ActiveFile,
        acting_provider_family_class: provider,
        conflict_class: ConflictClass::SingleProviderNoConflict,
        provider_disagreement_visibility_class:
            ProviderDisagreementVisibilityClass::NotApplicableSingleProvider,
        fix_offer_class: FixOfferClass::NoFixOffered,
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

/// Builds the baseline matrix of diagnostic-cluster rows covering every host
/// surface and every cluster lane while exercising the full vocabulary, all
/// valid.
fn baseline_rows() -> Vec<DiagnosticClusterRow> {
    use ClusterLaneClass::*;
    use SurfaceClass::*;

    let mut rows = Vec::new();

    // Row A — notebook surface / notebook lane: runtime-only banner from a
    // single notebook-kernel provider; nothing to fix.
    rows.push({
        let mut r = base(
            "row:notebook_surface:notebook",
            NotebookSurface,
            Notebook,
            ProviderFamilyClass::NotebookAdapter,
            DiagnosticSourceClass::NotebookKernel,
        );
        r.semantic_layer_banner_class = SemanticLayerBannerClass::RuntimeOnly;
        r.freshness_class = FreshnessClass::Warm;
        r.scope_label_class = ScopeLabelClass::OpenCells;
        r.downgrade_automation_class = DowngradeAutomationClass::AutoNarrowOnStaleProvenance;
        r.disclosure_ref = Some(disclosure("auto_narrow_on_stale_provenance"));
        r.evidence_class = EvidenceClass::ConformanceSuiteEvidence;
        r
    });

    // Row B — framework surface / framework lane: framework analyzer plus LSP
    // converge, the per-provider detail is preserved, the disagreement is
    // arbitrated with the loser kept inspectable, and a non-mutating fix is
    // offered.
    rows.push({
        let mut r = base(
            "row:framework_surface:framework",
            FrameworkSurface,
            Framework,
            ProviderFamilyClass::FrameworkAnalyzer,
            DiagnosticSourceClass::FrameworkSchema,
        );
        r.diagnostic_source_classes = vec![
            DiagnosticSourceClass::FrameworkSchema,
            DiagnosticSourceClass::Lsp,
        ];
        r.cluster_provenance_class = ClusterProvenanceClass::PerProviderPreserved;
        r.source_differentiation_class = SourceDifferentiationClass::DifferentiatedBySource;
        r.detail_sheet_route_class = DetailSheetRouteClass::OpenClusterDetailSheet;
        r.scope_label_class = ScopeLabelClass::LoadedSlice;
        r.conflict_class = ConflictClass::ArbitratedWinnerLoserPreserved;
        r.provider_disagreement_visibility_class =
            ProviderDisagreementVisibilityClass::LosersPreservedInspectable;
        r.fix_offer_class = FixOfferClass::NonMutatingFix;
        r.downgrade_automation_class = DowngradeAutomationClass::AutoNarrowOnConflictUnresolved;
        r.disclosure_ref = Some(disclosure("auto_narrow_on_conflict_unresolved"));
        r.evidence_class = EvidenceClass::ArchetypeRepoEvidence;
        r
    });

    // Row C — preview surface / compiler lane: exact live semantic compiler
    // diagnostics covering the whole workspace.
    rows.push({
        let mut r = base(
            "row:preview_surface:compiler",
            PreviewSurface,
            Compiler,
            ProviderFamilyClass::LspProvider,
            DiagnosticSourceClass::CompilerBuild,
        );
        r.scope_label_class = ScopeLabelClass::WholeWorkspace;
        r.evidence_class = EvidenceClass::BenchmarkEvidence;
        r
    });

    // Row D — generated-code surface / language-server lane: LSP plus
    // generated-artifact validation converge, the detail stays inspectable, and
    // a notebook/generated edit is offered behind a typed preview and rollback.
    rows.push({
        let mut r = base(
            "row:generated_code_surface:language_server",
            GeneratedCodeSurface,
            LanguageServer,
            ProviderFamilyClass::GeneratedSourceBridge,
            DiagnosticSourceClass::Lsp,
        );
        r.diagnostic_source_classes = vec![
            DiagnosticSourceClass::Lsp,
            DiagnosticSourceClass::GeneratedArtifactValidation,
        ];
        r.cluster_provenance_class = ClusterProvenanceClass::PerProviderPreserved;
        r.source_differentiation_class = SourceDifferentiationClass::DifferentiatedBySource;
        r.detail_sheet_route_class = DetailSheetRouteClass::OpenProviderBreakdown;
        r.semantic_layer_banner_class = SemanticLayerBannerClass::GraphWarm;
        r.freshness_class = FreshnessClass::Warm;
        r.scope_label_class = ScopeLabelClass::GeneratedExcluded;
        r.fix_offer_class = FixOfferClass::NotebookGeneratedFix;
        r.preview_completeness_class = CompletenessClass::Complete;
        r.rollback_checkpoint_ref = Some("checkpoint:rollback:diagnostic_cluster:01".to_owned());
        r.downgrade_automation_class = DowngradeAutomationClass::AutoNarrowOnPreviewPartial;
        r.known_limit_class = KnownLimitClass::GeneratedPolicySubsetOnly;
        r.disclosure_ref = Some(disclosure("auto_narrow_on_preview_partial"));
        r
    });

    // Row E — notebook surface / linter lane: a syntax-only linter cluster
    // narrowed below certified, offering a previewed quick fix.
    rows.push({
        let mut r = base(
            "row:notebook_surface:linter",
            NotebookSurface,
            Linter,
            ProviderFamilyClass::LspProvider,
            DiagnosticSourceClass::LinterFormatter,
        );
        r.support_class = SupportClass::CertifiedBelow;
        r.semantic_layer_banner_class = SemanticLayerBannerClass::SyntaxOnly;
        r.freshness_class = FreshnessClass::Warm;
        r.fix_offer_class = FixOfferClass::MutatingQuickFix;
        r.preview_completeness_class = CompletenessClass::Partial;
        r.rollback_checkpoint_ref = Some("checkpoint:rollback:diagnostic_cluster:02".to_owned());
        r.known_limit_class = KnownLimitClass::DiagnosticSourceSubsetOnly;
        r.downgrade_automation_class = DowngradeAutomationClass::AutoDemoteOnLowConfidence;
        r.confidence_class = ConfidenceClass::MediumConfidence;
        r.evidence_class = EvidenceClass::DocsDisclosureEvidence;
        r.disclosure_ref = Some(disclosure("diagnostic_source_subset_only"));
        r
    });

    // Row F — framework surface / runtime lane: runtime, policy, and static
    // findings converge but stay differentiated by source; the disagreement is
    // surfaced unresolved with the losers inspectable.
    rows.push({
        let mut r = base(
            "row:framework_surface:runtime",
            FrameworkSurface,
            Runtime,
            ProviderFamilyClass::FrameworkAnalyzer,
            DiagnosticSourceClass::RuntimeTestDebug,
        );
        r.diagnostic_source_classes = vec![
            DiagnosticSourceClass::RuntimeTestDebug,
            DiagnosticSourceClass::PolicyTrust,
            DiagnosticSourceClass::Lsp,
        ];
        r.cluster_provenance_class = ClusterProvenanceClass::PerProviderPreserved;
        r.source_differentiation_class = SourceDifferentiationClass::DifferentiatedBySource;
        r.detail_sheet_route_class = DetailSheetRouteClass::OpenProviderBreakdown;
        r.semantic_layer_banner_class = SemanticLayerBannerClass::RuntimeOnly;
        r.freshness_class = FreshnessClass::Warm;
        r.scope_label_class = ScopeLabelClass::LoadedSlice;
        r.conflict_class = ConflictClass::UnresolvedDisagreementSurfaced;
        r.provider_disagreement_visibility_class =
            ProviderDisagreementVisibilityClass::LosersPreservedInspectable;
        r.downgrade_automation_class = DowngradeAutomationClass::AutoNarrowOnConflictUnresolved;
        r.disclosure_ref = Some(disclosure("auto_narrow_on_conflict_unresolved"));
        r.evidence_class = EvidenceClass::ConformanceSuiteEvidence;
        r
    });

    // Row G — preview surface / policy lane: a cached policy cluster anchoring
    // an AI-planned fix behind a typed preview and rollback.
    rows.push({
        let mut r = base(
            "row:preview_surface:policy",
            PreviewSurface,
            Policy,
            ProviderFamilyClass::AiOverlay,
            DiagnosticSourceClass::PolicyTrust,
        );
        r.semantic_layer_banner_class = SemanticLayerBannerClass::Cached;
        r.freshness_class = FreshnessClass::Cached;
        r.conflict_class = ConflictClass::PolicyOverrideRecorded;
        r.fix_offer_class = FixOfferClass::AiPlannedFix;
        r.preview_completeness_class = CompletenessClass::Complete;
        r.rollback_checkpoint_ref = Some("checkpoint:rollback:diagnostic_cluster:03".to_owned());
        r.downgrade_automation_class = DowngradeAutomationClass::AutoNarrowOnStaleProvenance;
        r.disclosure_ref = Some(disclosure("auto_narrow_on_stale_provenance"));
        r.evidence_class = EvidenceClass::DesignPartnerEvidence;
        r
    });

    // Row H — generated-code surface / linter lane: a stale syntax-only cluster
    // on a single artifact offering an organize-imports rewrite behind preview.
    rows.push({
        let mut r = base(
            "row:generated_code_surface:linter",
            GeneratedCodeSurface,
            Linter,
            ProviderFamilyClass::GeneratedSourceBridge,
            DiagnosticSourceClass::LinterFormatter,
        );
        r.semantic_layer_banner_class = SemanticLayerBannerClass::SyntaxOnly;
        r.freshness_class = FreshnessClass::Stale;
        r.scope_label_class = ScopeLabelClass::SingleArtifact;
        r.fix_offer_class = FixOfferClass::OrganizeImportsFix;
        r.preview_completeness_class = CompletenessClass::Complete;
        r.rollback_checkpoint_ref = Some("checkpoint:rollback:diagnostic_cluster:04".to_owned());
        r.downgrade_automation_class = DowngradeAutomationClass::ManualOnlyPendingReview;
        r.disclosure_ref = Some(disclosure("manual_only_pending_review"));
        r
    });

    // Row I — generated-code surface / framework lane: a partial-semantic
    // cluster offering a schema/codegen rewrite behind a typed preview.
    rows.push({
        let mut r = base(
            "row:generated_code_surface:framework",
            GeneratedCodeSurface,
            Framework,
            ProviderFamilyClass::GeneratedSourceBridge,
            DiagnosticSourceClass::FrameworkSchema,
        );
        r.diagnostic_source_classes = vec![
            DiagnosticSourceClass::FrameworkSchema,
            DiagnosticSourceClass::GeneratedArtifactValidation,
        ];
        r.cluster_provenance_class = ClusterProvenanceClass::PerProviderPreserved;
        r.source_differentiation_class = SourceDifferentiationClass::DifferentiatedBySource;
        r.detail_sheet_route_class = DetailSheetRouteClass::OpenProviderBreakdown;
        r.semantic_layer_banner_class = SemanticLayerBannerClass::Partial;
        r.scope_label_class = ScopeLabelClass::LoadedSlice;
        r.conflict_class = ConflictClass::ArbitratedWinnerLoserPreserved;
        r.provider_disagreement_visibility_class =
            ProviderDisagreementVisibilityClass::LosersPreservedInspectable;
        r.fix_offer_class = FixOfferClass::SchemaCodegenFix;
        r.preview_completeness_class = CompletenessClass::Complete;
        r.rollback_checkpoint_ref = Some("checkpoint:rollback:diagnostic_cluster:05".to_owned());
        r.downgrade_automation_class = DowngradeAutomationClass::AutoNarrowOnPreviewPartial;
        r.disclosure_ref = Some(disclosure("auto_narrow_on_preview_partial"));
        r.evidence_class = EvidenceClass::FrameworkMigrationEvidence;
        r
    });

    rows
}

fn projections(packet_id: &str) -> Vec<DiagnosticClusterConsumerProjection> {
    ConsumerSurface::REQUIRED
        .into_iter()
        .map(|surface| DiagnosticClusterConsumerProjection {
            consumer_surface: surface,
            projection_ref: format!(
                "projection:diagnostic_cluster_semantic_layer:{}",
                surface.as_str()
            ),
            surface_packet_id_ref: packet_id.to_owned(),
            rendered_at: TS.to_owned(),
            preserves_same_packet: true,
            preserves_surface_vocabulary: true,
            preserves_cluster_lane_vocabulary: true,
            preserves_support_class_vocabulary: true,
            preserves_diagnostic_source_vocabulary: true,
            preserves_cluster_provenance_vocabulary: true,
            preserves_source_differentiation_vocabulary: true,
            preserves_detail_sheet_route_vocabulary: true,
            preserves_semantic_layer_banner_vocabulary: true,
            preserves_freshness_vocabulary: true,
            preserves_scope_label_vocabulary: true,
            preserves_provider_family_vocabulary: true,
            preserves_conflict_vocabulary: true,
            preserves_provider_disagreement_visibility_vocabulary: true,
            preserves_fix_offer_vocabulary: true,
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

fn baseline_input(packet_id: &str) -> DiagnosticClusterSemanticLayerTruthPacketInput {
    DiagnosticClusterSemanticLayerTruthPacketInput {
        packet_id: packet_id.to_owned(),
        workflow_or_surface_id: WORKFLOW.to_owned(),
        generated_at: TS.to_owned(),
        covered_surfaces: SurfaceClass::REQUIRED.to_vec(),
        rows: baseline_rows(),
        consumer_projections: projections(packet_id),
        source_contract_refs: vec![
            DIAGNOSTIC_CLUSTER_SEMANTIC_LAYER_TRUTH_SCHEMA_REF.to_owned(),
            DIAGNOSTIC_CLUSTER_SEMANTIC_LAYER_MATRIX_SOURCE_REF.to_owned(),
        ],
    }
}

fn token_array(tokens: Vec<&'static str>) -> Value {
    Value::Array(tokens.into_iter().map(|t| json!(t)).collect())
}

fn expect_block(
    packet: &DiagnosticClusterSemanticLayerTruthPacket,
    export_safe: bool,
    expected_finding_kinds: &[&str],
) -> Value {
    json!({
        "promotion_state": packet.promotion_state.as_str(),
        "validation_finding_count": packet.validation_findings.len(),
        "row_count": packet.rows.len(),
        "surface_tokens": token_array(packet.surface_tokens()),
        "cluster_lane_tokens": token_array(packet.cluster_lane_tokens()),
        "support_class_tokens": token_array(packet.support_class_tokens()),
        "diagnostic_source_tokens": token_array(packet.diagnostic_source_tokens()),
        "cluster_provenance_tokens": token_array(packet.cluster_provenance_tokens()),
        "source_differentiation_tokens": token_array(packet.source_differentiation_tokens()),
        "detail_sheet_route_tokens": token_array(packet.detail_sheet_route_tokens()),
        "semantic_layer_banner_tokens": token_array(packet.semantic_layer_banner_tokens()),
        "freshness_tokens": token_array(packet.freshness_tokens()),
        "scope_label_tokens": token_array(packet.scope_label_tokens()),
        "provider_family_tokens": token_array(packet.provider_family_tokens()),
        "conflict_tokens": token_array(packet.conflict_tokens()),
        "provider_disagreement_visibility_tokens": token_array(packet.provider_disagreement_visibility_tokens()),
        "fix_offer_tokens": token_array(packet.fix_offer_tokens()),
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
    input: DiagnosticClusterSemanticLayerTruthPacketInput,
    export_safe: bool,
    expected_finding_kinds: &[&str],
) -> Value {
    let packet = DiagnosticClusterSemanticLayerTruthPacket::materialize(input.clone());
    json!({
        "record_kind": "diagnostic_cluster_semantic_layer_truth_stable_case",
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
    let packet = DiagnosticClusterSemanticLayerTruthPacket::materialize(baseline_input(PACKET_ID));
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
        "artifacts/language/m5/diagnostic_cluster_semantic_layer_truth_packet.json",
        &serde_json::to_value(&packet).expect("packet serializes"),
    );

    let dir = DIAGNOSTIC_CLUSTER_SEMANTIC_LAYER_TRUTH_FIXTURE_DIR;
    let id = |suffix: &str| format!("packet:m5:diagnostic_cluster_semantic_layer:{suffix}");

    // Baseline stable fixture.
    write_json(
        &format!("{dir}/baseline_stable.json"),
        &fixture(
            "baseline_stable",
            "Baseline stable posture: every host surface (notebook, framework, preview, generated code) and every cluster lane (compiler, linter, language-server, framework, runtime, notebook, policy) carries at least one diagnostic-cluster row. Each row names the acting provider, lists the diagnostic source families that converged, and — when more than one provider converged — keeps per-provider detail, timestamps/epochs, suppression/baseline state, and related evidence inspectable behind a real detail-sheet route instead of an opaque spinner. Runtime, policy, and static findings stay differentiated by source rather than fusing into one undifferentiated row; an arbitrated or unresolved disagreement keeps the losing provider inspectable; the semantic-layer banner (semantic, graph-warm, syntax-only, cached, runtime-only, or partial) matches the freshness evidence; a whole-workspace scope appears only on live semantic evidence; and every offered fix names the acting provider and freshness/scope posture, with organize-imports, schema/codegen, AI-planned, and notebook/generated edits bound to a typed preview completeness and a rollback checkpoint. All ten required consumer projections preserve the packet verbatim.",
            baseline_input(&id("baseline_stable")),
            true,
            &[],
        ),
    );

    // Negative cases: each takes the baseline and trips a guardrail on row 0
    // (the notebook / notebook-lane cluster).
    type NegativeCase = (
        &'static str,
        &'static str,
        fn(&mut DiagnosticClusterRow),
        &'static [&'static str],
    );
    let cases: Vec<NegativeCase> = vec![
        (
            "cluster_provenance_collapsed_blocks_stable",
            "Two providers converge into one cluster but the per-provider detail is collapsed, so the user can no longer see what each provider reported. The row narrows below stable.",
            |row| {
                row.diagnostic_source_classes =
                    vec![DiagnosticSourceClass::Lsp, DiagnosticSourceClass::CompilerBuild];
                row.cluster_provenance_class = ClusterProvenanceClass::CollapsedLossy;
                row.source_differentiation_class = SourceDifferentiationClass::DifferentiatedBySource;
                row.detail_sheet_route_class = DetailSheetRouteClass::OpenProviderBreakdown;
            },
            &["cluster_provenance_collapsed"],
        ),
        (
            "dropped_suppression_state_blocks_stable",
            "A multi-provider cluster preserves its per-provider rows but drops suppression / baseline state from the detail sheet, hiding which findings were already triaged. The row narrows below stable.",
            |row| {
                row.diagnostic_source_classes =
                    vec![DiagnosticSourceClass::Lsp, DiagnosticSourceClass::LinterFormatter];
                row.cluster_provenance_class = ClusterProvenanceClass::PerProviderPreserved;
                row.source_differentiation_class = SourceDifferentiationClass::DifferentiatedBySource;
                row.detail_sheet_route_class = DetailSheetRouteClass::OpenProviderBreakdown;
                row.preserves_suppression_baseline = false;
            },
            &["cluster_provenance_collapsed"],
        ),
        (
            "sources_fused_undifferentiated_blocks_stable",
            "Runtime, policy, and static findings are fused into one undifferentiated error row, so a security finding reads the same as a lint warning. The row narrows below stable.",
            |row| {
                row.diagnostic_source_classes = vec![
                    DiagnosticSourceClass::RuntimeTestDebug,
                    DiagnosticSourceClass::PolicyTrust,
                    DiagnosticSourceClass::Lsp,
                ];
                row.cluster_provenance_class = ClusterProvenanceClass::PerProviderPreserved;
                row.source_differentiation_class = SourceDifferentiationClass::FusedUndifferentiated;
                row.detail_sheet_route_class = DetailSheetRouteClass::OpenProviderBreakdown;
            },
            &["sources_fused_undifferentiated"],
        ),
        (
            "losing_provider_collapsed_blocks_stable",
            "Two providers disagree but the losing provider is collapsed into ranking-only output, so what the alternate reported is no longer inspectable. The row narrows below stable.",
            |row| {
                row.diagnostic_source_classes =
                    vec![DiagnosticSourceClass::Lsp, DiagnosticSourceClass::FrameworkSchema];
                row.cluster_provenance_class = ClusterProvenanceClass::PerProviderPreserved;
                row.source_differentiation_class = SourceDifferentiationClass::DifferentiatedBySource;
                row.conflict_class = ConflictClass::ArbitratedWinnerLoserPreserved;
                row.provider_disagreement_visibility_class =
                    ProviderDisagreementVisibilityClass::LosersCollapsedRankingOnly;
                row.detail_sheet_route_class = DetailSheetRouteClass::OpenProviderBreakdown;
            },
            &["losing_provider_collapsed"],
        ),
        (
            "opaque_detail_sheet_route_blocks_stable",
            "An opaque loading spinner stands in for a real detail-sheet route, hiding the per-provider detail behind the cluster. The row narrows below stable.",
            |row| {
                row.detail_sheet_route_class = DetailSheetRouteClass::OpaqueSpinner;
            },
            &["opaque_detail_sheet_route"],
        ),
        (
            "multi_source_without_detail_sheet_blocks_stable",
            "A multi-provider cluster offers no inspectable detail-sheet route at all, so the converged findings cannot be unpacked. The row narrows below stable.",
            |row| {
                row.diagnostic_source_classes =
                    vec![DiagnosticSourceClass::Lsp, DiagnosticSourceClass::CompilerBuild];
                row.cluster_provenance_class = ClusterProvenanceClass::PerProviderPreserved;
                row.source_differentiation_class = SourceDifferentiationClass::DifferentiatedBySource;
                row.detail_sheet_route_class = DetailSheetRouteClass::NotApplicable;
            },
            &["detail_sheet_route_missing"],
        ),
        (
            "semantic_banner_on_stale_evidence_blocks_stable",
            "A surface claims the full semantic banner while the cluster evidence is stale, overstating how fresh the answer is. The banner must narrow to a degraded posture; the row narrows below stable.",
            |row| {
                row.semantic_layer_banner_class = SemanticLayerBannerClass::Semantic;
                row.freshness_class = FreshnessClass::Stale;
            },
            &["semantic_layer_overclaimed"],
        ),
        (
            "whole_workspace_scope_on_stale_evidence_blocks_stable",
            "A cached cluster keeps a whole-workspace scope label even though the evidence rests on cached, non-semantic data. The scope must narrow to the scanned slice; the row narrows below stable.",
            |row| {
                row.semantic_layer_banner_class = SemanticLayerBannerClass::Cached;
                row.freshness_class = FreshnessClass::Cached;
                row.scope_label_class = ScopeLabelClass::WholeWorkspace;
            },
            &["overclaimed_scope_on_stale_evidence"],
        ),
        (
            "fix_offered_without_freshness_blocks_stable",
            "A fix is offered while the freshness label is unbound, so the user cannot tell how fresh the diagnostic the fix targets is. The row narrows below stable.",
            |row| {
                row.support_class = SupportClass::CertifiedBelow;
                row.fix_offer_class = FixOfferClass::NonMutatingFix;
                row.freshness_class = FreshnessClass::FreshnessUnbound;
            },
            &["missing_freshness_label", "fix_offered_without_provider_or_freshness"],
        ),
        (
            "mutating_fix_without_rollback_blocks_stable",
            "An organize-imports rewrite is offered but bypasses the rollback checkpoint required by the launch-language refactor safety model. The row narrows below stable.",
            |row| {
                row.fix_offer_class = FixOfferClass::OrganizeImportsFix;
                row.preview_completeness_class = CompletenessClass::Complete;
                row.rollback_checkpoint_ref = None;
            },
            &["mutating_fix_bypasses_preview"],
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
