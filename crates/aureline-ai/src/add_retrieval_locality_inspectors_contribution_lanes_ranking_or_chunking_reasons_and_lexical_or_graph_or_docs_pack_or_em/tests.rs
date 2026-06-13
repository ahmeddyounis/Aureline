use super::*;

const PACKET_ID: &str = "retrieval-locality-inspector:stable:0001";

fn lane(
    lane: ContributionLaneClass,
    state: ContributionState,
    kind: SelectionReasonKind,
    reason: SelectionReasonClass,
    locality: RetrievalLocalityClass,
    generation_state: RetrievalGenerationState,
    summary: &str,
) -> ContributionLaneRow {
    ContributionLaneRow {
        lane,
        state,
        selection_reason_kind: kind,
        selection_reason: reason,
        locality,
        generation_state,
        reason_summary: summary.to_owned(),
    }
}

fn surface_rows() -> Vec<RetrievalInspectorSurfaceRow> {
    vec![
        RetrievalInspectorSurfaceRow {
            surface: RetrievalInspectorSurface::Search,
            scope_summary:
                "Search labels lexical, graph, and embedding contributions with per-lane ranking reasons and workspace-local locality"
                    .to_owned(),
            contribution_lanes: vec![
                lane(
                    ContributionLaneClass::LexicalKeyword,
                    ContributionState::Contributed,
                    SelectionReasonKind::Ranking,
                    SelectionReasonClass::RankByLexicalScore,
                    RetrievalLocalityClass::WorkspaceLocal,
                    RetrievalGenerationState::Current,
                    "Top lexical matches ranked by relevance score",
                ),
                lane(
                    ContributionLaneClass::GraphTraversal,
                    ContributionState::Contributed,
                    SelectionReasonKind::Ranking,
                    SelectionReasonClass::RankByGraphProximity,
                    RetrievalLocalityClass::WorkspaceLocal,
                    RetrievalGenerationState::Current,
                    "Symbols ranked by proximity in the workspace graph",
                ),
                lane(
                    ContributionLaneClass::EmbeddingVector,
                    ContributionState::Contributed,
                    SelectionReasonKind::Ranking,
                    SelectionReasonClass::RankBySemanticSimilarity,
                    RetrievalLocalityClass::WorkspaceLocal,
                    RetrievalGenerationState::Current,
                    "Semantic neighbors ranked by embedding similarity",
                ),
                lane(
                    ContributionLaneClass::ProviderOverlay,
                    ContributionState::Empty,
                    SelectionReasonKind::NotApplicable,
                    SelectionReasonClass::NotApplicable,
                    RetrievalLocalityClass::WorkspaceLocal,
                    RetrievalGenerationState::Current,
                    "Provider overlay not enabled for this search",
                ),
            ],
            hidden_scope_count: 0,
            degraded_lanes: vec![],
            provider_overlay_posture: ProviderOverlayPosture::LocalOnlyNoOverlay,
            completeness_claim: CompletenessClass::Complete,
            replay_label_parity: true,
            downgrade_triggers: vec![
                InspectorDowngradeTrigger::LaneDegraded,
                InspectorDowngradeTrigger::LocalityUnavailable,
                InspectorDowngradeTrigger::ProofStale,
            ],
            source_contract_refs: vec![
                RETRIEVAL_LOCALITY_INSPECTOR_RANKING_CONTRACT_REF.to_owned(),
                RETRIEVAL_LOCALITY_INSPECTOR_EXPLAINABILITY_CONTRACT_REF.to_owned(),
            ],
            consumer_surfaces: vec![
                InspectorConsumerSurface::SearchResults,
                InspectorConsumerSurface::Diagnostics,
                InspectorConsumerSurface::SupportExport,
            ],
        },
        RetrievalInspectorSurfaceRow {
            surface: RetrievalInspectorSurface::DocsRecall,
            scope_summary:
                "Docs recall labels lexical, docs-pack, and embedding contributions with chunking reasons; the stale embedding lane is shown degraded and hidden scope is disclosed"
                    .to_owned(),
            contribution_lanes: vec![
                lane(
                    ContributionLaneClass::LexicalKeyword,
                    ContributionState::Contributed,
                    SelectionReasonKind::Ranking,
                    SelectionReasonClass::RankByLexicalScore,
                    RetrievalLocalityClass::WorkspaceLocal,
                    RetrievalGenerationState::Current,
                    "Doc passages ranked by lexical relevance",
                ),
                lane(
                    ContributionLaneClass::DocsPack,
                    ContributionState::Contributed,
                    SelectionReasonKind::Chunking,
                    SelectionReasonClass::ChunkByDocStructure,
                    RetrievalLocalityClass::WorkspaceLocal,
                    RetrievalGenerationState::Current,
                    "Doc-pack sections chunked along document headings",
                ),
                lane(
                    ContributionLaneClass::EmbeddingVector,
                    ContributionState::Degraded,
                    SelectionReasonKind::Chunking,
                    SelectionReasonClass::ChunkBySemanticBoundary,
                    RetrievalLocalityClass::WorkspaceLocal,
                    RetrievalGenerationState::Stale,
                    "Embedding chunks served from a stale prior generation",
                ),
                lane(
                    ContributionLaneClass::ProviderOverlay,
                    ContributionState::Empty,
                    SelectionReasonKind::NotApplicable,
                    SelectionReasonClass::NotApplicable,
                    RetrievalLocalityClass::WorkspaceLocal,
                    RetrievalGenerationState::Current,
                    "Provider overlay not enabled for docs recall",
                ),
            ],
            hidden_scope_count: 3,
            degraded_lanes: vec![ContributionLaneClass::EmbeddingVector],
            provider_overlay_posture: ProviderOverlayPosture::LocalOnlyNoOverlay,
            completeness_claim: CompletenessClass::PartialHiddenScope,
            replay_label_parity: true,
            downgrade_triggers: vec![
                InspectorDowngradeTrigger::LaneDegraded,
                InspectorDowngradeTrigger::MixedGenerationUnlabeled,
                InspectorDowngradeTrigger::HiddenScopeUndisclosed,
                InspectorDowngradeTrigger::ProofStale,
            ],
            source_contract_refs: vec![
                RETRIEVAL_LOCALITY_INSPECTOR_EXPLAINABILITY_CONTRACT_REF.to_owned(),
                RETRIEVAL_LOCALITY_INSPECTOR_RANKING_CONTRACT_REF.to_owned(),
            ],
            consumer_surfaces: vec![
                InspectorConsumerSurface::DocsCompanion,
                InspectorConsumerSurface::SearchResults,
                InspectorConsumerSurface::SupportExport,
                InspectorConsumerSurface::ReplayPacket,
            ],
        },
        RetrievalInspectorSurfaceRow {
            surface: RetrievalInspectorSurface::AiContextPack,
            scope_summary:
                "AI context pack labels graph, embedding, docs-pack, and provider-overlay contributions with ranking and chunking reasons; the provider overlay is region-pinned and disclosed and mixed embedding generations are labeled"
                    .to_owned(),
            contribution_lanes: vec![
                lane(
                    ContributionLaneClass::GraphTraversal,
                    ContributionState::Contributed,
                    SelectionReasonKind::Ranking,
                    SelectionReasonClass::RankByGraphProximity,
                    RetrievalLocalityClass::WorkspaceLocal,
                    RetrievalGenerationState::Current,
                    "Graph neighbors ranked by proximity to the active symbol",
                ),
                lane(
                    ContributionLaneClass::EmbeddingVector,
                    ContributionState::Contributed,
                    SelectionReasonKind::Ranking,
                    SelectionReasonClass::RankByHybridFusion,
                    RetrievalLocalityClass::WorkspaceLocal,
                    RetrievalGenerationState::MixedGenerationLabeled,
                    "Embedding hits fused with lexical scores; mixed generations labeled",
                ),
                lane(
                    ContributionLaneClass::DocsPack,
                    ContributionState::Contributed,
                    SelectionReasonKind::Chunking,
                    SelectionReasonClass::ChunkByDocStructure,
                    RetrievalLocalityClass::WorkspaceLocal,
                    RetrievalGenerationState::Current,
                    "Doc-pack sections chunked along document structure",
                ),
                lane(
                    ContributionLaneClass::ProviderOverlay,
                    ContributionState::Contributed,
                    SelectionReasonKind::Overlay,
                    SelectionReasonClass::ProviderOverlayMerge,
                    RetrievalLocalityClass::ManagedHostedRegionPinned,
                    RetrievalGenerationState::Current,
                    "Region-pinned managed provider overlay merged with disclosure",
                ),
            ],
            hidden_scope_count: 0,
            degraded_lanes: vec![],
            provider_overlay_posture: ProviderOverlayPosture::OverlayDisclosed,
            completeness_claim: CompletenessClass::Complete,
            replay_label_parity: true,
            downgrade_triggers: vec![
                InspectorDowngradeTrigger::LaneDegraded,
                InspectorDowngradeTrigger::ProviderOverlayUndisclosed,
                InspectorDowngradeTrigger::MixedGenerationUnlabeled,
                InspectorDowngradeTrigger::ProofStale,
            ],
            source_contract_refs: vec![
                RETRIEVAL_LOCALITY_INSPECTOR_CONTEXT_ASSEMBLY_CONTRACT_REF.to_owned(),
                RETRIEVAL_LOCALITY_INSPECTOR_SPEND_RECEIPT_CONTRACT_REF.to_owned(),
                RETRIEVAL_LOCALITY_INSPECTOR_RANKING_CONTRACT_REF.to_owned(),
            ],
            consumer_surfaces: vec![
                InspectorConsumerSurface::ComposerContextPack,
                InspectorConsumerSurface::Diagnostics,
                InspectorConsumerSurface::SupportExport,
                InspectorConsumerSurface::ReplayPacket,
            ],
        },
    ]
}

fn guardrails() -> RetrievalLocalityInspectorGuardrails {
    RetrievalLocalityInspectorGuardrails {
        no_cross_workspace_recall_by_default: true,
        no_cross_tenant_recall_by_default: true,
        mixed_generation_labeled_never_masquerades: true,
        degraded_lanes_never_implied_complete: true,
        provider_overlay_always_disclosed: true,
        replay_preserves_lane_and_locality_labels: true,
        hidden_scope_counts_disclosed: true,
    }
}

fn consumer_projection() -> RetrievalLocalityInspectorConsumerProjection {
    RetrievalLocalityInspectorConsumerProjection {
        search_labels_all_contribution_lanes: true,
        docs_recall_labels_lanes_and_locality: true,
        context_pack_labels_lanes_and_ranking_or_chunking: true,
        diagnostics_shows_hidden_scope_and_degraded: true,
        support_export_preserves_labels: true,
        replay_preserves_labels: true,
        unqualified_completeness_labeled: true,
    }
}

fn proof_freshness() -> RetrievalLocalityInspectorProofFreshness {
    RetrievalLocalityInspectorProofFreshness {
        proof_freshness_slo_hours: 168,
        last_proof_refresh: "2026-06-13T00:00:00Z".to_owned(),
        auto_narrow_on_stale: true,
    }
}

fn source_contract_refs() -> Vec<String> {
    vec![
        RETRIEVAL_LOCALITY_INSPECTOR_SCHEMA_REF.to_owned(),
        RETRIEVAL_LOCALITY_INSPECTOR_DOC_REF.to_owned(),
        RETRIEVAL_LOCALITY_INSPECTOR_RANKING_CONTRACT_REF.to_owned(),
        RETRIEVAL_LOCALITY_INSPECTOR_EXPLAINABILITY_CONTRACT_REF.to_owned(),
        RETRIEVAL_LOCALITY_INSPECTOR_CONTEXT_ASSEMBLY_CONTRACT_REF.to_owned(),
        RETRIEVAL_LOCALITY_INSPECTOR_SPEND_RECEIPT_CONTRACT_REF.to_owned(),
        RETRIEVAL_LOCALITY_INSPECTOR_RECALL_MATRIX_CONTRACT_REF.to_owned(),
    ]
}

fn packet() -> RetrievalLocalityInspectorPacket {
    RetrievalLocalityInspectorPacket::new(RetrievalLocalityInspectorPacketInput {
        packet_id: PACKET_ID.to_owned(),
        inspector_label: "Retrieval Locality Inspector".to_owned(),
        surface_rows: surface_rows(),
        guardrails: guardrails(),
        consumer_projection: consumer_projection(),
        proof_freshness: proof_freshness(),
        source_contract_refs: source_contract_refs(),
        redaction_class_token: "metadata_safe_default".to_owned(),
        minted_at: "2026-06-13T00:00:00Z".to_owned(),
    })
}

#[test]
fn inspector_packet_validates() {
    let packet = packet();
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
}

#[test]
fn all_required_surfaces_present() {
    let packet = packet();
    for surface in RetrievalInspectorSurface::ALL {
        assert!(packet.surface_rows.iter().any(|row| row.surface == surface));
    }
}

#[test]
fn missing_surface_fails_validation() {
    let mut packet = packet();
    packet
        .surface_rows
        .retain(|row| row.surface != RetrievalInspectorSurface::AiContextPack);
    assert!(packet
        .validate()
        .contains(&RetrievalLocalityInspectorViolation::RequiredSurfaceMissing));
}

#[test]
fn all_five_lanes_are_labeled_across_surfaces() {
    let packet = packet();
    let mut seen: BTreeSet<ContributionLaneClass> = BTreeSet::new();
    for row in &packet.surface_rows {
        for lane in &row.contribution_lanes {
            seen.insert(lane.lane);
        }
    }
    for lane in ContributionLaneClass::ALL {
        assert!(seen.contains(&lane), "lane {lane:?} never labeled");
    }
}

#[test]
fn incoherent_lane_reason_kind_fails() {
    let mut packet = packet();
    // Declare a chunking reason but label the lane as ranking.
    packet.surface_rows[0].contribution_lanes[0].selection_reason =
        SelectionReasonClass::ChunkByFixedWindow;
    assert!(packet
        .validate()
        .contains(&RetrievalLocalityInspectorViolation::LaneLabelingIncoherent));
}

#[test]
fn non_overlay_lane_with_overlay_reason_fails() {
    let mut packet = packet();
    packet.surface_rows[0].contribution_lanes[0].selection_reason_kind =
        SelectionReasonKind::Overlay;
    packet.surface_rows[0].contribution_lanes[0].selection_reason =
        SelectionReasonClass::ProviderOverlayMerge;
    assert!(packet
        .validate()
        .contains(&RetrievalLocalityInspectorViolation::LaneLabelingIncoherent));
}

#[test]
fn stale_generation_not_marked_degraded_fails() {
    let mut packet = packet();
    // The docs-recall embedding lane is stale; presenting it as a clean
    // contribution must be rejected.
    packet.surface_rows[1].contribution_lanes[2].state = ContributionState::Contributed;
    // Keep degraded-lane disclosure consistent with the new state so the
    // generation-honesty violation is isolated.
    packet.surface_rows[1].degraded_lanes.clear();
    packet.surface_rows[1].completeness_claim = CompletenessClass::PartialHiddenScope;
    assert!(packet
        .validate()
        .contains(&RetrievalLocalityInspectorViolation::MixedGenerationMasquerades));
}

#[test]
fn degraded_lane_not_disclosed_fails() {
    let mut packet = packet();
    // Drop the disclosed degraded lane while the lane state stays degraded.
    packet.surface_rows[1].degraded_lanes.clear();
    assert!(packet
        .validate()
        .contains(&RetrievalLocalityInspectorViolation::DegradedLanesInconsistent));
}

#[test]
fn undisclosed_provider_overlay_fails() {
    let mut packet = packet();
    // The AI context pack has an active provider overlay; claiming no overlay
    // must be rejected.
    packet.surface_rows[2].provider_overlay_posture = ProviderOverlayPosture::LocalOnlyNoOverlay;
    assert!(packet
        .validate()
        .contains(&RetrievalLocalityInspectorViolation::ProviderOverlayUndisclosed));
}

#[test]
fn hidden_scope_claiming_complete_fails() {
    let mut packet = packet();
    // Docs recall hides scope; claiming complete must be rejected and the
    // effective completeness narrows.
    packet.surface_rows[1].completeness_claim = CompletenessClass::Complete;
    let violations = packet.validate();
    assert!(violations.contains(&RetrievalLocalityInspectorViolation::HiddenScopeImpliesComplete));
    assert_eq!(
        packet.surface_rows[1].effective_completeness(),
        CompletenessClass::DegradedSubset
    );
}

#[test]
fn effective_completeness_narrows_partial_when_only_hidden_scope() {
    let mut packet = packet();
    // Remove the degraded lane but keep hidden scope, then dishonestly claim
    // complete: the narrowing falls to partial-hidden-scope.
    packet.surface_rows[1].contribution_lanes[2].state = ContributionState::Empty;
    packet.surface_rows[1].contribution_lanes[2].selection_reason_kind =
        SelectionReasonKind::NotApplicable;
    packet.surface_rows[1].contribution_lanes[2].selection_reason =
        SelectionReasonClass::NotApplicable;
    packet.surface_rows[1].contribution_lanes[2].generation_state =
        RetrievalGenerationState::Current;
    packet.surface_rows[1].degraded_lanes.clear();
    packet.surface_rows[1].completeness_claim = CompletenessClass::Complete;
    assert_eq!(
        packet.surface_rows[1].effective_completeness(),
        CompletenessClass::PartialHiddenScope
    );
}

#[test]
fn replay_parity_missing_fails() {
    let mut packet = packet();
    // Docs recall is reachable by replay; dropping label parity must fail.
    packet.surface_rows[1].replay_label_parity = false;
    assert!(packet
        .validate()
        .contains(&RetrievalLocalityInspectorViolation::ReplayLabelParityMissing));
}

#[test]
fn missing_downgrade_triggers_fails() {
    let mut packet = packet();
    packet.surface_rows[0].downgrade_triggers.clear();
    assert!(packet
        .validate()
        .contains(&RetrievalLocalityInspectorViolation::DowngradeTriggersMissing));
}

#[test]
fn missing_consumer_surfaces_fails() {
    let mut packet = packet();
    packet.surface_rows[0].consumer_surfaces.clear();
    let violations = packet.validate();
    assert!(violations.contains(&RetrievalLocalityInspectorViolation::ConsumerSurfacesMissing));
}

#[test]
fn missing_source_contracts_fails() {
    let mut packet = packet();
    packet.source_contract_refs.clear();
    assert!(packet
        .validate()
        .contains(&RetrievalLocalityInspectorViolation::MissingSourceContracts));
}

#[test]
fn guardrails_incomplete_fails() {
    let mut packet = packet();
    packet.guardrails.provider_overlay_always_disclosed = false;
    assert!(packet
        .validate()
        .contains(&RetrievalLocalityInspectorViolation::GuardrailsIncomplete));
}

#[test]
fn consumer_projection_incomplete_fails() {
    let mut packet = packet();
    packet.consumer_projection.replay_preserves_labels = false;
    assert!(packet
        .validate()
        .contains(&RetrievalLocalityInspectorViolation::ConsumerProjectionIncomplete));
}

#[test]
fn proof_freshness_incomplete_fails() {
    let mut packet = packet();
    packet.proof_freshness.proof_freshness_slo_hours = 0;
    assert!(packet
        .validate()
        .contains(&RetrievalLocalityInspectorViolation::ProofFreshnessIncomplete));
}

#[test]
fn export_safe_json_round_trips() {
    let packet = packet();
    let json = packet.export_safe_json();
    let parsed: RetrievalLocalityInspectorPacket =
        serde_json::from_str(&json).expect("export json parses back");
    assert_eq!(parsed, packet);
}

#[test]
fn markdown_summary_names_surfaces_and_lanes() {
    let summary = packet().render_markdown_summary();
    assert!(summary.contains("Retrieval Locality Inspector"));
    assert!(summary.contains("ai_context_pack"));
    assert!(summary.contains("provider_overlay"));
    assert!(summary.contains("Provider overlay:"));
}

#[test]
fn checked_support_export_validates() {
    let packet = current_stable_retrieval_locality_inspector_export()
        .expect("checked retrieval locality inspector export validates");
    assert_eq!(packet.packet_id, PACKET_ID);
}

#[test]
fn degraded_provider_overlay_fixture_validates() {
    let packet: RetrievalLocalityInspectorPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/ai/m5/add_retrieval_locality_inspectors_contribution_lanes_ranking_or_chunking_reasons_and_lexical_or_graph_or_docs_pack_or_em/degraded_provider_overlay_context_pack.json"
    )))
    .expect("degraded provider overlay fixture parses");
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
    // The degraded provider-overlay context pack must never claim completeness.
    let pack = packet
        .surface_rows
        .iter()
        .find(|row| row.surface == RetrievalInspectorSurface::AiContextPack)
        .expect("fixture has an AI context pack surface");
    assert_eq!(pack.completeness_claim, CompletenessClass::DegradedSubset);
    assert_eq!(
        pack.provider_overlay_posture,
        ProviderOverlayPosture::OverlayDegraded
    );
}
