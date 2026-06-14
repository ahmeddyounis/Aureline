use super::*;
use crate::provider_refactor_matrix_truth_packet::{
    CompletenessClass, ConfidenceClass, ConflictClass, ConsumerSurface, DiagnosticSourceClass,
    DowngradeAutomationClass, EvidenceClass, KnownLimitClass, ProviderFamilyClass, SupportClass,
};

const TS: &str = "2026-06-14T12:00:00Z";
const DISCLOSURE: &str =
    "docs/m5/diagnostic-clustering-semantic-layer-banners-and-detail-sheets.md#auto_narrow_on_missing_fixture";

/// A fully valid, single-source, single-provider, live-semantic row.
fn base_row(row_id: &str, surface: SurfaceClass, lane: ClusterLaneClass) -> DiagnosticClusterRow {
    DiagnosticClusterRow {
        row_id: row_id.to_owned(),
        surface_class: surface,
        cluster_lane_class: lane,
        support_class: SupportClass::Certified,
        diagnostic_source_classes: vec![DiagnosticSourceClass::Lsp],
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
        acting_provider_family_class: ProviderFamilyClass::LspProvider,
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
        evidence_refs: vec![DIAGNOSTIC_CLUSTER_SEMANTIC_LAYER_TRUTH_FIXTURE_DIR.to_owned()],
        disclosure_ref: Some(DISCLOSURE.to_owned()),
        raw_source_material_excluded: true,
        secrets_excluded: true,
        ambient_authority_excluded: true,
        captured_at: TS.to_owned(),
    }
}

fn projection(surface: ConsumerSurface, packet_id: &str) -> DiagnosticClusterConsumerProjection {
    DiagnosticClusterConsumerProjection {
        consumer_surface: surface,
        projection_ref: format!("projection:{}", surface.as_str()),
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
    }
}

const PACKET_ID: &str = "packet:test:diagnostic_cluster_semantic_layer";

/// Builds a valid baseline input covering all required surfaces and lanes plus
/// every required consumer projection.
fn baseline_input() -> DiagnosticClusterSemanticLayerTruthPacketInput {
    let surfaces = SurfaceClass::REQUIRED;
    let lanes = ClusterLaneClass::REQUIRED;
    let mut rows = Vec::new();
    // Place one row per (surface, lane) pair so both axes have full coverage.
    let surface_for = |index: usize| surfaces[index % surfaces.len()];
    for (index, lane) in lanes.into_iter().enumerate() {
        let surface = surface_for(index);
        rows.push(base_row(
            &format!("row:{}:{}", surface.as_str(), lane.as_str()),
            surface,
            lane,
        ));
    }
    // Ensure every surface appears even though there are more lanes than
    // surfaces only in one direction.
    for surface in surfaces {
        if !rows.iter().any(|row| row.surface_class == surface) {
            rows.push(base_row(
                &format!("row:{}:extra", surface.as_str()),
                surface,
                ClusterLaneClass::Compiler,
            ));
        }
    }
    let consumer_projections = ConsumerSurface::REQUIRED
        .into_iter()
        .map(|surface| projection(surface, PACKET_ID))
        .collect();
    DiagnosticClusterSemanticLayerTruthPacketInput {
        packet_id: PACKET_ID.to_owned(),
        workflow_or_surface_id: "workflow.test.diagnostic_cluster_semantic_layer".to_owned(),
        generated_at: TS.to_owned(),
        covered_surfaces: surfaces.to_vec(),
        rows,
        consumer_projections,
        source_contract_refs: vec![DIAGNOSTIC_CLUSTER_SEMANTIC_LAYER_TRUTH_SCHEMA_REF.to_owned()],
    }
}

fn has_kind(packet: &DiagnosticClusterSemanticLayerTruthPacket, kind: FindingKind) -> bool {
    packet
        .validation_findings
        .iter()
        .any(|finding| finding.finding_kind == kind)
}

#[test]
fn baseline_materializes_stable_and_export_safe() {
    let packet = DiagnosticClusterSemanticLayerTruthPacket::materialize(baseline_input());
    assert_eq!(packet.promotion_state, PromotionState::Stable);
    assert!(packet.validation_findings.is_empty());
    assert!(packet.is_stable());
    for surface in SurfaceClass::REQUIRED {
        assert!(packet.surface_tokens().contains(&surface.as_str()));
    }
    for lane in ClusterLaneClass::REQUIRED {
        assert!(packet.cluster_lane_tokens().contains(&lane.as_str()));
    }
    let export = packet.support_export("export:test", TS);
    assert!(export.is_export_safe());
}

#[test]
fn cluster_provenance_collapsed_blocks_stable() {
    let mut input = baseline_input();
    let row = &mut input.rows[0];
    row.diagnostic_source_classes = vec![
        DiagnosticSourceClass::Lsp,
        DiagnosticSourceClass::CompilerBuild,
    ];
    row.cluster_provenance_class = ClusterProvenanceClass::CollapsedLossy;
    row.source_differentiation_class = SourceDifferentiationClass::DifferentiatedBySource;
    row.detail_sheet_route_class = DetailSheetRouteClass::OpenProviderBreakdown;
    let packet = DiagnosticClusterSemanticLayerTruthPacket::materialize(input);
    assert!(has_kind(&packet, FindingKind::ClusterProvenanceCollapsed));
    assert_eq!(packet.promotion_state, PromotionState::BlocksStable);
}

#[test]
fn dropped_detail_flags_block_stable() {
    let mut input = baseline_input();
    let row = &mut input.rows[1];
    row.diagnostic_source_classes = vec![
        DiagnosticSourceClass::Lsp,
        DiagnosticSourceClass::LinterFormatter,
    ];
    row.cluster_provenance_class = ClusterProvenanceClass::PerProviderPreserved;
    row.source_differentiation_class = SourceDifferentiationClass::DifferentiatedBySource;
    row.detail_sheet_route_class = DetailSheetRouteClass::OpenProviderBreakdown;
    row.preserves_suppression_baseline = false;
    let packet = DiagnosticClusterSemanticLayerTruthPacket::materialize(input);
    assert!(has_kind(&packet, FindingKind::ClusterProvenanceCollapsed));
}

#[test]
fn fused_runtime_policy_static_blocks_stable() {
    let mut input = baseline_input();
    let row = &mut input.rows[2];
    row.diagnostic_source_classes = vec![
        DiagnosticSourceClass::RuntimeTestDebug,
        DiagnosticSourceClass::PolicyTrust,
        DiagnosticSourceClass::Lsp,
    ];
    row.cluster_provenance_class = ClusterProvenanceClass::PerProviderPreserved;
    row.source_differentiation_class = SourceDifferentiationClass::FusedUndifferentiated;
    row.detail_sheet_route_class = DetailSheetRouteClass::OpenProviderBreakdown;
    let packet = DiagnosticClusterSemanticLayerTruthPacket::materialize(input);
    assert!(has_kind(&packet, FindingKind::SourcesFusedUndifferentiated));
}

#[test]
fn losing_provider_collapsed_blocks_stable() {
    let mut input = baseline_input();
    let row = &mut input.rows[3];
    row.diagnostic_source_classes = vec![
        DiagnosticSourceClass::Lsp,
        DiagnosticSourceClass::FrameworkSchema,
    ];
    row.cluster_provenance_class = ClusterProvenanceClass::PerProviderPreserved;
    row.source_differentiation_class = SourceDifferentiationClass::DifferentiatedBySource;
    row.conflict_class = ConflictClass::ArbitratedWinnerLoserPreserved;
    row.provider_disagreement_visibility_class =
        ProviderDisagreementVisibilityClass::LosersCollapsedRankingOnly;
    row.detail_sheet_route_class = DetailSheetRouteClass::OpenProviderBreakdown;
    let packet = DiagnosticClusterSemanticLayerTruthPacket::materialize(input);
    assert!(has_kind(&packet, FindingKind::LosingProviderCollapsed));
}

#[test]
fn opaque_detail_sheet_route_blocks_stable() {
    let mut input = baseline_input();
    input.rows[4].detail_sheet_route_class = DetailSheetRouteClass::OpaqueSpinner;
    let packet = DiagnosticClusterSemanticLayerTruthPacket::materialize(input);
    assert!(has_kind(&packet, FindingKind::OpaqueDetailSheetRoute));
}

#[test]
fn multi_source_without_detail_sheet_blocks_stable() {
    let mut input = baseline_input();
    let row = &mut input.rows[5];
    row.diagnostic_source_classes = vec![
        DiagnosticSourceClass::Lsp,
        DiagnosticSourceClass::CompilerBuild,
    ];
    row.cluster_provenance_class = ClusterProvenanceClass::PerProviderPreserved;
    row.source_differentiation_class = SourceDifferentiationClass::DifferentiatedBySource;
    row.detail_sheet_route_class = DetailSheetRouteClass::NotApplicable;
    let packet = DiagnosticClusterSemanticLayerTruthPacket::materialize(input);
    assert!(has_kind(&packet, FindingKind::DetailSheetRouteMissing));
}

#[test]
fn semantic_banner_on_stale_evidence_blocks_stable() {
    let mut input = baseline_input();
    let row = &mut input.rows[0];
    row.semantic_layer_banner_class = SemanticLayerBannerClass::Semantic;
    row.freshness_class = FreshnessClass::Stale;
    let packet = DiagnosticClusterSemanticLayerTruthPacket::materialize(input);
    assert!(has_kind(&packet, FindingKind::SemanticLayerOverclaimed));
}

#[test]
fn whole_workspace_scope_on_stale_evidence_blocks_stable() {
    let mut input = baseline_input();
    let row = &mut input.rows[1];
    row.semantic_layer_banner_class = SemanticLayerBannerClass::Cached;
    row.freshness_class = FreshnessClass::Cached;
    row.scope_label_class = ScopeLabelClass::WholeWorkspace;
    let packet = DiagnosticClusterSemanticLayerTruthPacket::materialize(input);
    assert!(has_kind(
        &packet,
        FindingKind::OverclaimedScopeOnStaleEvidence
    ));
}

#[test]
fn fix_offered_without_provider_or_freshness_blocks_stable() {
    let mut input = baseline_input();
    let row = &mut input.rows[2];
    row.fix_offer_class = FixOfferClass::NonMutatingFix;
    row.freshness_class = FreshnessClass::FreshnessUnbound;
    let packet = DiagnosticClusterSemanticLayerTruthPacket::materialize(input);
    // Unbound freshness trips both the binding rule and the fix-naming rule.
    assert!(has_kind(
        &packet,
        FindingKind::FixOfferedWithoutProviderOrFreshness
    ));
    assert!(has_kind(&packet, FindingKind::MissingFreshnessLabel));
}

#[test]
fn mutating_fix_without_rollback_blocks_stable() {
    let mut input = baseline_input();
    let row = &mut input.rows[3];
    row.fix_offer_class = FixOfferClass::OrganizeImportsFix;
    row.preview_completeness_class = CompletenessClass::Complete;
    row.rollback_checkpoint_ref = None;
    let packet = DiagnosticClusterSemanticLayerTruthPacket::materialize(input);
    assert!(has_kind(&packet, FindingKind::MutatingFixBypassesPreview));
}

#[test]
fn mutating_fix_with_preview_and_rollback_is_stable() {
    let mut input = baseline_input();
    let row = &mut input.rows[4];
    row.fix_offer_class = FixOfferClass::MutatingQuickFix;
    row.preview_completeness_class = CompletenessClass::Complete;
    row.rollback_checkpoint_ref = Some("checkpoint:rollback:test".to_owned());
    let packet = DiagnosticClusterSemanticLayerTruthPacket::materialize(input);
    assert_eq!(packet.promotion_state, PromotionState::Stable);
}

#[test]
fn certified_with_unbound_evidence_blocks_stable() {
    let mut input = baseline_input();
    input.rows[5].evidence_class = EvidenceClass::EvidenceUnbound;
    let packet = DiagnosticClusterSemanticLayerTruthPacket::materialize(input);
    assert!(has_kind(&packet, FindingKind::MissingEvidenceClass));
    assert!(has_kind(&packet, FindingKind::CertifiedWithUnboundBinding));
    assert_eq!(packet.promotion_state, PromotionState::BlocksStable);
}

#[test]
fn raw_source_material_blocks_stable() {
    let mut input = baseline_input();
    input.rows[6].raw_source_material_excluded = false;
    let packet = DiagnosticClusterSemanticLayerTruthPacket::materialize(input);
    assert!(has_kind(&packet, FindingKind::RawSourceMaterialPresent));
}

#[test]
fn narrowed_row_without_disclosure_ref_blocks_stable() {
    let mut input = baseline_input();
    let row = &mut input.rows[0];
    row.support_class = SupportClass::CertifiedBelow;
    row.disclosure_ref = None;
    row.downgrade_automation_class = DowngradeAutomationClass::None;
    row.known_limit_class = KnownLimitClass::NoneDeclared;
    let packet = DiagnosticClusterSemanticLayerTruthPacket::materialize(input);
    assert!(has_kind(
        &packet,
        FindingKind::NarrowedRowMissingDisclosureRef
    ));
}

#[test]
fn missing_consumer_projection_blocks_stable() {
    let mut input = baseline_input();
    input.consumer_projections.pop();
    let packet = DiagnosticClusterSemanticLayerTruthPacket::materialize(input);
    assert!(has_kind(&packet, FindingKind::MissingConsumerProjection));
}

#[test]
fn missing_lane_coverage_blocks_stable() {
    let mut input = baseline_input();
    input
        .rows
        .retain(|row| row.cluster_lane_class != ClusterLaneClass::Policy);
    let packet = DiagnosticClusterSemanticLayerTruthPacket::materialize(input);
    assert!(has_kind(&packet, FindingKind::MissingClusterLaneCoverage));
}

#[test]
fn missing_surface_coverage_blocks_stable() {
    let mut input = baseline_input();
    input
        .rows
        .retain(|row| row.surface_class != SurfaceClass::PreviewSurface);
    let packet = DiagnosticClusterSemanticLayerTruthPacket::materialize(input);
    assert!(has_kind(&packet, FindingKind::MissingSurfaceCoverage));
}

#[test]
fn certified_at_low_confidence_narrows_below_stable() {
    let mut input = baseline_input();
    input.rows[1].confidence_class = ConfidenceClass::LowConfidence;
    let packet = DiagnosticClusterSemanticLayerTruthPacket::materialize(input);
    assert!(has_kind(&packet, FindingKind::CertifiedAtLowConfidence));
    assert_eq!(packet.promotion_state, PromotionState::NarrowedBelowStable);
}

#[test]
fn checked_in_artifact_validates() {
    let packet = current_stable_diagnostic_cluster_semantic_layer_truth_packet()
        .expect("checked-in packet parses and validates");
    assert!(packet.validate().is_empty());
    assert_eq!(packet.promotion_state, PromotionState::Stable);
}
