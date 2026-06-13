//! Conformance dump for the retrieval-locality inspector packet.
//!
//! Prints the export-safe JSON of the canonical stable inspector. The output is
//! the checked-in support export under
//! `artifacts/ai/m5/add_retrieval_locality_inspectors_contribution_lanes_ranking_or_chunking_reasons_and_lexical_or_graph_or_docs_pack_or_em/`.

use aureline_ai::add_retrieval_locality_inspectors_contribution_lanes_ranking_or_chunking_reasons_and_lexical_or_graph_or_docs_pack_or_em::{
    CompletenessClass, ContributionLaneClass, ContributionLaneRow, ContributionState,
    InspectorConsumerSurface, InspectorDowngradeTrigger, ProviderOverlayPosture,
    RetrievalGenerationState, RetrievalInspectorSurface, RetrievalInspectorSurfaceRow,
    RetrievalLocalityClass, RetrievalLocalityInspectorConsumerProjection,
    RetrievalLocalityInspectorGuardrails, RetrievalLocalityInspectorPacket,
    RetrievalLocalityInspectorPacketInput, RetrievalLocalityInspectorProofFreshness,
    SelectionReasonClass, SelectionReasonKind,
    RETRIEVAL_LOCALITY_INSPECTOR_CONTEXT_ASSEMBLY_CONTRACT_REF,
    RETRIEVAL_LOCALITY_INSPECTOR_DOC_REF, RETRIEVAL_LOCALITY_INSPECTOR_EXPLAINABILITY_CONTRACT_REF,
    RETRIEVAL_LOCALITY_INSPECTOR_RANKING_CONTRACT_REF,
    RETRIEVAL_LOCALITY_INSPECTOR_RECALL_MATRIX_CONTRACT_REF, RETRIEVAL_LOCALITY_INSPECTOR_SCHEMA_REF,
    RETRIEVAL_LOCALITY_INSPECTOR_SPEND_RECEIPT_CONTRACT_REF,
};

fn main() {
    let packet = canonical_packet();
    let violations = packet.validate();
    assert!(
        violations.is_empty(),
        "canonical inspector invalid: {violations:?}"
    );
    println!("{}", packet.export_safe_json());
}

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

fn canonical_packet() -> RetrievalLocalityInspectorPacket {
    RetrievalLocalityInspectorPacket::new(RetrievalLocalityInspectorPacketInput {
        packet_id: "retrieval-locality-inspector:stable:0001".to_owned(),
        inspector_label: "Retrieval Locality Inspector".to_owned(),
        surface_rows: vec![
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
        ],
        guardrails: RetrievalLocalityInspectorGuardrails {
            no_cross_workspace_recall_by_default: true,
            no_cross_tenant_recall_by_default: true,
            mixed_generation_labeled_never_masquerades: true,
            degraded_lanes_never_implied_complete: true,
            provider_overlay_always_disclosed: true,
            replay_preserves_lane_and_locality_labels: true,
            hidden_scope_counts_disclosed: true,
        },
        consumer_projection: RetrievalLocalityInspectorConsumerProjection {
            search_labels_all_contribution_lanes: true,
            docs_recall_labels_lanes_and_locality: true,
            context_pack_labels_lanes_and_ranking_or_chunking: true,
            diagnostics_shows_hidden_scope_and_degraded: true,
            support_export_preserves_labels: true,
            replay_preserves_labels: true,
            unqualified_completeness_labeled: true,
        },
        proof_freshness: RetrievalLocalityInspectorProofFreshness {
            proof_freshness_slo_hours: 168,
            last_proof_refresh: "2026-06-13T00:00:00Z".to_owned(),
            auto_narrow_on_stale: true,
        },
        source_contract_refs: vec![
            RETRIEVAL_LOCALITY_INSPECTOR_SCHEMA_REF.to_owned(),
            RETRIEVAL_LOCALITY_INSPECTOR_DOC_REF.to_owned(),
            RETRIEVAL_LOCALITY_INSPECTOR_RANKING_CONTRACT_REF.to_owned(),
            RETRIEVAL_LOCALITY_INSPECTOR_EXPLAINABILITY_CONTRACT_REF.to_owned(),
            RETRIEVAL_LOCALITY_INSPECTOR_CONTEXT_ASSEMBLY_CONTRACT_REF.to_owned(),
            RETRIEVAL_LOCALITY_INSPECTOR_SPEND_RECEIPT_CONTRACT_REF.to_owned(),
            RETRIEVAL_LOCALITY_INSPECTOR_RECALL_MATRIX_CONTRACT_REF.to_owned(),
        ],
        redaction_class_token: "metadata_safe_default".to_owned(),
        minted_at: "2026-06-13T00:00:00Z".to_owned(),
    })
}
