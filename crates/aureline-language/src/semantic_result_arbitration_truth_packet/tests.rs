use super::*;
use crate::provider_refactor_matrix_truth_packet::{
    CompletenessClass, ConfidenceClass, ConflictClass, ConsumerSurface, DowngradeAutomationClass,
    EvidenceClass, KnownLimitClass, ProviderFamilyClass, SupportClass,
};

const TS: &str = "2026-06-14T12:00:00Z";
const DISCLOSURE: &str =
    "docs/m5/arbitration-inspectors-disagreement-detail-and-semantic-to-text-fallback-banners.md#auto_narrow_on_missing_fixture";

/// A fully valid, exact-semantic, single-provider row on one surface+lane.
fn exact_row(
    row_id: &str,
    surface: ResultSurfaceClass,
    lane: ResultLaneClass,
) -> ResultArbitrationRow {
    ResultArbitrationRow {
        row_id: row_id.to_owned(),
        result_surface_class: surface,
        result_lane_class: lane,
        support_class: SupportClass::Certified,
        acting_provider_family_class: ProviderFamilyClass::LspProvider,
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
        evidence_refs: vec![SEMANTIC_RESULT_ARBITRATION_TRUTH_FIXTURE_DIR.to_owned()],
        disclosure_ref: Some(DISCLOSURE.to_owned()),
        raw_source_material_excluded: true,
        secrets_excluded: true,
        ambient_authority_excluded: true,
        captured_at: TS.to_owned(),
    }
}

fn projection(surface: ConsumerSurface, packet_id: &str) -> ResultArbitrationConsumerProjection {
    ResultArbitrationConsumerProjection {
        consumer_surface: surface,
        projection_ref: format!("projection:{}", surface.as_str()),
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
    }
}

const PACKET_ID: &str = "packet:test:semantic_result_arbitration";

/// Builds a valid baseline input covering all required surfaces and lanes plus
/// every required consumer projection.
fn baseline_input() -> SemanticResultArbitrationTruthPacketInput {
    let surfaces = ResultSurfaceClass::REQUIRED;
    let lanes = ResultLaneClass::REQUIRED;
    let mut rows = Vec::new();
    for surface in surfaces {
        for lane in lanes {
            rows.push(exact_row(
                &format!("row:{}:{}", surface.as_str(), lane.as_str()),
                surface,
                lane,
            ));
        }
    }
    let consumer_projections = ConsumerSurface::REQUIRED
        .into_iter()
        .map(|surface| projection(surface, PACKET_ID))
        .collect();
    SemanticResultArbitrationTruthPacketInput {
        packet_id: PACKET_ID.to_owned(),
        workflow_or_surface_id: "workflow.test.semantic_result_arbitration".to_owned(),
        generated_at: TS.to_owned(),
        covered_surfaces: surfaces.to_vec(),
        rows,
        consumer_projections,
        source_contract_refs: vec![SEMANTIC_RESULT_ARBITRATION_TRUTH_SCHEMA_REF.to_owned()],
    }
}

fn has_kind(packet: &SemanticResultArbitrationTruthPacket, kind: FindingKind) -> bool {
    packet
        .validation_findings
        .iter()
        .any(|finding| finding.finding_kind == kind)
}

#[test]
fn baseline_materializes_stable_and_export_safe() {
    let packet = SemanticResultArbitrationTruthPacket::materialize(baseline_input());
    assert_eq!(packet.promotion_state, PromotionState::Stable);
    assert!(packet.validation_findings.is_empty());
    assert!(packet.is_stable());
    assert_eq!(packet.rows.len(), 20);
    for surface in ResultSurfaceClass::REQUIRED {
        assert!(packet.result_surface_tokens().contains(&surface.as_str()));
    }
    for lane in ResultLaneClass::REQUIRED {
        assert!(packet.result_lane_tokens().contains(&lane.as_str()));
    }
    let export = packet.support_export("export:test", TS);
    assert!(export.is_export_safe());
}

#[test]
fn losing_provider_collapsed_blocks_stable() {
    let mut input = baseline_input();
    let row = &mut input.rows[0];
    row.conflict_class = ConflictClass::ArbitratedWinnerLoserPreserved;
    row.alternate_provider_visibility_class =
        AlternateProviderVisibilityClass::AlternatesCollapsedRankingOnly;
    row.inspector_route_class = InspectorRouteClass::OpenDisagreementDetail;
    let packet = SemanticResultArbitrationTruthPacket::materialize(input);
    assert_eq!(packet.promotion_state, PromotionState::BlocksStable);
    assert!(has_kind(&packet, FindingKind::LosingProviderCollapsed));
}

#[test]
fn material_conflict_without_detail_path_blocks_stable() {
    let mut input = baseline_input();
    let row = &mut input.rows[1];
    row.conflict_class = ConflictClass::ArbitratedWinnerLoserPreserved;
    row.alternate_provider_visibility_class =
        AlternateProviderVisibilityClass::AlternatesPreservedInspectable;
    row.disagreement_impact_class = DisagreementImpactClass::ScopeCoverageChanged;
    row.disagreement_visibility_class = DisagreementVisibilityClass::None;
    row.inspector_route_class = InspectorRouteClass::NotApplicable;
    let packet = SemanticResultArbitrationTruthPacket::materialize(input);
    assert!(has_kind(
        &packet,
        FindingKind::DisagreementDetailPathMissing
    ));
    assert_eq!(packet.promotion_state, PromotionState::BlocksStable);
}

#[test]
fn opaque_spinner_route_blocks_stable() {
    let mut input = baseline_input();
    input.rows[2].inspector_route_class = InspectorRouteClass::OpaqueSpinner;
    let packet = SemanticResultArbitrationTruthPacket::materialize(input);
    assert!(has_kind(&packet, FindingKind::OpaqueInspectorRoute));
}

#[test]
fn silent_fusion_of_conflict_blocks_stable() {
    let mut input = baseline_input();
    let row = &mut input.rows[3];
    row.conflict_class = ConflictClass::ArbitratedWinnerLoserPreserved;
    row.alternate_provider_visibility_class =
        AlternateProviderVisibilityClass::AlternatesPreservedInspectable;
    row.disagreement_impact_class = DisagreementImpactClass::TargetIdentityChanged;
    row.disagreement_visibility_class = DisagreementVisibilityClass::None;
    row.inspector_route_class = InspectorRouteClass::OpenDisagreementDetail;
    let packet = SemanticResultArbitrationTruthPacket::materialize(input);
    assert!(has_kind(&packet, FindingKind::SilentFusionOfConflict));
}

#[test]
fn degraded_result_without_banner_blocks_stable() {
    let mut input = baseline_input();
    let row = &mut input.rows[4];
    row.result_tier_class = ResultTierClass::TextLexical;
    row.retained_guarantee_class = RetainedGuaranteeClass::LexicalMatchOnly;
    row.claim_scope_class = ClaimScopeClass::ActiveFileResults;
    row.fallback_banner_class = FallbackBannerClass::None;
    row.lost_guarantee_class = LostGuaranteeClass::NoneLost;
    let packet = SemanticResultArbitrationTruthPacket::materialize(input);
    assert!(has_kind(&packet, FindingKind::FallbackBannerMissing));
}

#[test]
fn exact_result_with_banner_blocks_stable() {
    let mut input = baseline_input();
    input.rows[5].fallback_banner_class = FallbackBannerClass::SemanticToTextFallback;
    let packet = SemanticResultArbitrationTruthPacket::materialize(input);
    assert!(has_kind(&packet, FindingKind::FallbackBannerOnExactResult));
}

#[test]
fn overclaimed_scope_on_lexical_evidence_blocks_stable() {
    let mut input = baseline_input();
    let row = &mut input.rows[6];
    row.result_tier_class = ResultTierClass::TextLexical;
    row.retained_guarantee_class = RetainedGuaranteeClass::LexicalMatchOnly;
    row.fallback_banner_class = FallbackBannerClass::SemanticToTextFallback;
    row.lost_guarantee_class = LostGuaranteeClass::LostAllReferencesGuarantee;
    row.claim_scope_class = ClaimScopeClass::WholeWorkspaceAllResults;
    let packet = SemanticResultArbitrationTruthPacket::materialize(input);
    assert!(has_kind(
        &packet,
        FindingKind::OverclaimedScopeOnLexicalEvidence
    ));
}

#[test]
fn whole_workspace_wording_with_coverage_gap_blocks_stable() {
    let mut input = baseline_input();
    let row = &mut input.rows[7];
    row.result_tier_class = ResultTierClass::PartialSemantic;
    row.fallback_banner_class = FallbackBannerClass::SemanticToFileLocalFallback;
    row.lost_guarantee_class = LostGuaranteeClass::LostWholeWorkspaceScope;
    row.retained_guarantee_class = RetainedGuaranteeClass::FileLocalSemantic;
    row.claim_scope_class = ClaimScopeClass::WholeWorkspaceAllResults;
    row.coverage_gap_class = CoverageGapClass::ExcludedRootsSkipped;
    let packet = SemanticResultArbitrationTruthPacket::materialize(input);
    assert!(has_kind(
        &packet,
        FindingKind::WholeWorkspaceWordingWithCoverageGap
    ));
}

#[test]
fn mutating_anchor_without_rollback_blocks_stable() {
    let mut input = baseline_input();
    let row = &mut input.rows[8];
    row.anchor_action_class = AnchorActionClass::MutatingFollowupPreview;
    row.preview_completeness_class = CompletenessClass::Complete;
    row.rollback_checkpoint_ref = None;
    let packet = SemanticResultArbitrationTruthPacket::materialize(input);
    assert!(has_kind(
        &packet,
        FindingKind::MutatingAnchorBypassesPreview
    ));
}

#[test]
fn mutating_anchor_with_preview_and_rollback_is_stable() {
    let mut input = baseline_input();
    let row = &mut input.rows[9];
    row.anchor_action_class = AnchorActionClass::MutatingFollowupPreview;
    row.preview_completeness_class = CompletenessClass::Complete;
    row.rollback_checkpoint_ref = Some("checkpoint:rollback:test".to_owned());
    let packet = SemanticResultArbitrationTruthPacket::materialize(input);
    assert_eq!(packet.promotion_state, PromotionState::Stable);
}

#[test]
fn certified_with_unbound_evidence_blocks_stable() {
    let mut input = baseline_input();
    input.rows[10].evidence_class = EvidenceClass::EvidenceUnbound;
    let packet = SemanticResultArbitrationTruthPacket::materialize(input);
    assert!(has_kind(&packet, FindingKind::MissingEvidenceClass));
    assert!(has_kind(&packet, FindingKind::CertifiedWithUnboundBinding));
    assert_eq!(packet.promotion_state, PromotionState::BlocksStable);
}

#[test]
fn raw_source_material_blocks_stable() {
    let mut input = baseline_input();
    input.rows[11].raw_source_material_excluded = false;
    let packet = SemanticResultArbitrationTruthPacket::materialize(input);
    assert!(has_kind(&packet, FindingKind::RawSourceMaterialPresent));
}

#[test]
fn narrowed_row_without_disclosure_ref_blocks_stable() {
    let mut input = baseline_input();
    let row = &mut input.rows[12];
    row.support_class = SupportClass::CertifiedBelow;
    row.disclosure_ref = None;
    row.downgrade_automation_class = DowngradeAutomationClass::None;
    row.known_limit_class = KnownLimitClass::NoneDeclared;
    let packet = SemanticResultArbitrationTruthPacket::materialize(input);
    assert!(has_kind(
        &packet,
        FindingKind::NarrowedRowMissingDisclosureRef
    ));
}

#[test]
fn missing_consumer_projection_blocks_stable() {
    let mut input = baseline_input();
    input.consumer_projections.pop();
    let packet = SemanticResultArbitrationTruthPacket::materialize(input);
    assert!(has_kind(&packet, FindingKind::MissingConsumerProjection));
}

#[test]
fn missing_lane_coverage_blocks_stable() {
    let mut input = baseline_input();
    input
        .rows
        .retain(|row| row.result_lane_class != ResultLaneClass::Hierarchy);
    let packet = SemanticResultArbitrationTruthPacket::materialize(input);
    assert!(has_kind(&packet, FindingKind::MissingLaneCoverage));
}

#[test]
fn certified_at_low_confidence_narrows_below_stable() {
    let mut input = baseline_input();
    input.rows[13].confidence_class = ConfidenceClass::LowConfidence;
    let packet = SemanticResultArbitrationTruthPacket::materialize(input);
    assert!(has_kind(&packet, FindingKind::CertifiedAtLowConfidence));
    assert_eq!(packet.promotion_state, PromotionState::NarrowedBelowStable);
}

#[test]
fn checked_in_artifact_validates() {
    let packet = current_stable_semantic_result_arbitration_truth_packet()
        .expect("checked-in packet parses and validates");
    assert!(packet.validate().is_empty());
    assert_eq!(packet.promotion_state, PromotionState::Stable);
}
