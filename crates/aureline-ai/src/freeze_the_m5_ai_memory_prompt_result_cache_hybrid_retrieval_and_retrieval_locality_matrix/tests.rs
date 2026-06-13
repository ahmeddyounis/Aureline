use super::*;

const PACKET_ID: &str = "m5-ai-recall-matrix:stable:0001";

fn surface_rows() -> Vec<M5AiRecallMatrixSurfaceRow> {
    vec![
        M5AiRecallMatrixSurfaceRow {
            surface: M5RecallSurface::ComposerAssist,
            qualification: M5RecallQualificationClass::Stable,
            scope_summary:
                "Composer recall over reusable semantic memory and prompt-result cache with visible memory/retrieval disclosure"
                    .to_owned(),
            memory_cache_classes: vec![
                M5MemoryCacheClass::PromptResultCache,
                M5MemoryCacheClass::ReusableSemanticMemory,
            ],
            retrieval_lanes: vec![
                M5RetrievalLane::SemanticEmbedding,
                M5RetrievalLane::HybridFusion,
            ],
            locality_posture: M5LocalityPosture::WorkspaceLocal,
            delete_export_posture: M5DeleteExportPosture::UserDeletableExportable,
            budget_receipt_expectation: M5BudgetReceiptExpectation::RouteAndSpendReceiptRequired,
            cache_invalidation_classes: vec![
                M5CacheInvalidationClass::ContentHashKey,
                M5CacheInvalidationClass::PolicyEpochBump,
                M5CacheInvalidationClass::TrustNarrowing,
            ],
            evidence_requirement: M5RecallEvidenceRequirement::Required,
            required_evidence_packet_refs: vec![
                "evidence:ai-memory-state:m5".to_owned(),
                "evidence:prompt-composer-conformance:m5".to_owned(),
            ],
            downgrade_triggers: vec![
                M5RecallDowngradeTrigger::ProofStale,
                M5RecallDowngradeTrigger::StaleHybridRetrieval,
                M5RecallDowngradeTrigger::BudgetExhausted,
            ],
            source_contract_refs: vec![
                M5_AI_RECALL_MATRIX_MEMORY_CLASS_CONTRACT_REF.to_owned(),
                M5_AI_RECALL_MATRIX_CONTEXT_ASSEMBLY_CONTRACT_REF.to_owned(),
            ],
            consumer_surfaces: vec![
                M5RecallConsumerSurface::DesktopComposer,
                M5RecallConsumerSurface::Diagnostics,
                M5RecallConsumerSurface::SupportExport,
            ],
        },
        M5AiRecallMatrixSurfaceRow {
            surface: M5RecallSurface::DocsBrowserRecall,
            qualification: M5RecallQualificationClass::Stable,
            scope_summary:
                "Docs and in-app browser recall with cited provenance, workspace-local embedding index, and locality disclosure"
                    .to_owned(),
            memory_cache_classes: vec![
                M5MemoryCacheClass::EmbeddingIndex,
                M5MemoryCacheClass::PromptResultCache,
            ],
            retrieval_lanes: vec![
                M5RetrievalLane::LexicalKeyword,
                M5RetrievalLane::SemanticEmbedding,
                M5RetrievalLane::HybridFusion,
            ],
            locality_posture: M5LocalityPosture::WorkspaceLocal,
            delete_export_posture: M5DeleteExportPosture::WorkspaceDeletableExportable,
            budget_receipt_expectation: M5BudgetReceiptExpectation::SpendReceiptRequired,
            cache_invalidation_classes: vec![
                M5CacheInvalidationClass::ContentHashKey,
                M5CacheInvalidationClass::EmbeddingGenerationBump,
                M5CacheInvalidationClass::TtlExpiry,
            ],
            evidence_requirement: M5RecallEvidenceRequirement::Required,
            required_evidence_packet_refs: vec!["evidence:docs-recall-provenance:m5".to_owned()],
            downgrade_triggers: vec![
                M5RecallDowngradeTrigger::ProofStale,
                M5RecallDowngradeTrigger::EmbeddingGenerationMismatch,
                M5RecallDowngradeTrigger::StaleHybridRetrieval,
            ],
            source_contract_refs: vec![M5_AI_RECALL_MATRIX_RETRIEVAL_CONTRACT_REF.to_owned()],
            consumer_surfaces: vec![
                M5RecallConsumerSurface::DocsBrowserCompanion,
                M5RecallConsumerSurface::SearchSurface,
                M5RecallConsumerSurface::SupportExport,
            ],
        },
        M5AiRecallMatrixSurfaceRow {
            surface: M5RecallSurface::CodeUnderstanding,
            qualification: M5RecallQualificationClass::Beta,
            scope_summary:
                "Codebase-understanding recall over the workspace graph and embedding index with checkpointed locality"
                    .to_owned(),
            memory_cache_classes: vec![M5MemoryCacheClass::EmbeddingIndex],
            retrieval_lanes: vec![
                M5RetrievalLane::GraphTraversal,
                M5RetrievalLane::SemanticEmbedding,
            ],
            locality_posture: M5LocalityPosture::WorkspaceLocal,
            delete_export_posture: M5DeleteExportPosture::WorkspaceDeletableExportable,
            budget_receipt_expectation: M5BudgetReceiptExpectation::BudgetCappedWithFallback,
            cache_invalidation_classes: vec![
                M5CacheInvalidationClass::EmbeddingGenerationBump,
                M5CacheInvalidationClass::ContentHashKey,
            ],
            evidence_requirement: M5RecallEvidenceRequirement::Required,
            required_evidence_packet_refs: vec!["evidence:code-understanding-recall:m5".to_owned()],
            downgrade_triggers: vec![
                M5RecallDowngradeTrigger::ProofStale,
                M5RecallDowngradeTrigger::EmbeddingGenerationMismatch,
                M5RecallDowngradeTrigger::UpstreamDependencyNarrowed,
            ],
            source_contract_refs: vec![M5_AI_RECALL_MATRIX_RETRIEVAL_CONTRACT_REF.to_owned()],
            consumer_surfaces: vec![
                M5RecallConsumerSurface::DesktopComposer,
                M5RecallConsumerSurface::SearchSurface,
                M5RecallConsumerSurface::SupportExport,
            ],
        },
        M5AiRecallMatrixSurfaceRow {
            surface: M5RecallSurface::SemanticSearch,
            qualification: M5RecallQualificationClass::Stable,
            scope_summary:
                "Semantic and hybrid search with labeled retrieval lanes and content-hash cache keys"
                    .to_owned(),
            memory_cache_classes: vec![
                M5MemoryCacheClass::EmbeddingIndex,
                M5MemoryCacheClass::PromptResultCache,
            ],
            retrieval_lanes: vec![
                M5RetrievalLane::LexicalKeyword,
                M5RetrievalLane::SemanticEmbedding,
                M5RetrievalLane::HybridFusion,
            ],
            locality_posture: M5LocalityPosture::WorkspaceLocal,
            delete_export_posture: M5DeleteExportPosture::WorkspaceDeletableExportable,
            budget_receipt_expectation: M5BudgetReceiptExpectation::SpendReceiptRequired,
            cache_invalidation_classes: vec![
                M5CacheInvalidationClass::ContentHashKey,
                M5CacheInvalidationClass::EmbeddingGenerationBump,
            ],
            evidence_requirement: M5RecallEvidenceRequirement::Required,
            required_evidence_packet_refs: vec!["evidence:search-retrieval-truth:m5".to_owned()],
            downgrade_triggers: vec![
                M5RecallDowngradeTrigger::ProofStale,
                M5RecallDowngradeTrigger::StaleHybridRetrieval,
                M5RecallDowngradeTrigger::EmbeddingGenerationMismatch,
            ],
            source_contract_refs: vec![M5_AI_RECALL_MATRIX_RETRIEVAL_CONTRACT_REF.to_owned()],
            consumer_surfaces: vec![
                M5RecallConsumerSurface::SearchSurface,
                M5RecallConsumerSurface::Diagnostics,
                M5RecallConsumerSurface::SupportExport,
            ],
        },
        M5AiRecallMatrixSurfaceRow {
            surface: M5RecallSurface::SupportExport,
            qualification: M5RecallQualificationClass::Stable,
            scope_summary:
                "Support and export projection that names locality posture, delete/export behavior, and receipts without raw bodies"
                    .to_owned(),
            memory_cache_classes: vec![M5MemoryCacheClass::NoDurableMemory],
            retrieval_lanes: vec![M5RetrievalLane::NoRetrieval],
            locality_posture: M5LocalityPosture::WorkspaceLocal,
            delete_export_posture: M5DeleteExportPosture::NotApplicable,
            budget_receipt_expectation: M5BudgetReceiptExpectation::LocalNoSpendReceipt,
            cache_invalidation_classes: vec![M5CacheInvalidationClass::ManualPurge],
            evidence_requirement: M5RecallEvidenceRequirement::Required,
            required_evidence_packet_refs: vec!["evidence:support-export-recall:m5".to_owned()],
            downgrade_triggers: vec![
                M5RecallDowngradeTrigger::ProofStale,
                M5RecallDowngradeTrigger::UpstreamDependencyNarrowed,
            ],
            source_contract_refs: vec![M5_AI_RECALL_MATRIX_DELETE_EXPORT_CONTRACT_REF.to_owned()],
            consumer_surfaces: vec![
                M5RecallConsumerSurface::SupportExport,
                M5RecallConsumerSurface::Diagnostics,
            ],
        },
        M5AiRecallMatrixSurfaceRow {
            surface: M5RecallSurface::ManagedOffline,
            qualification: M5RecallQualificationClass::Beta,
            scope_summary:
                "Managed and offline reporting that names region-pinned locality and local-no-spend receipt truth"
                    .to_owned(),
            memory_cache_classes: vec![M5MemoryCacheClass::EphemeralSessionState],
            retrieval_lanes: vec![M5RetrievalLane::NoRetrieval],
            locality_posture: M5LocalityPosture::ManagedHostedRegionPinned,
            delete_export_posture: M5DeleteExportPosture::EphemeralAutoExpire,
            budget_receipt_expectation: M5BudgetReceiptExpectation::LocalNoSpendReceipt,
            cache_invalidation_classes: vec![M5CacheInvalidationClass::TtlExpiry],
            evidence_requirement: M5RecallEvidenceRequirement::Recommended,
            required_evidence_packet_refs: vec!["evidence:managed-offline-recall:m5".to_owned()],
            downgrade_triggers: vec![
                M5RecallDowngradeTrigger::ProofStale,
                M5RecallDowngradeTrigger::LocalityUnavailable,
                M5RecallDowngradeTrigger::ProviderUnavailable,
            ],
            source_contract_refs: vec![M5_AI_RECALL_MATRIX_SPEND_RECEIPT_CONTRACT_REF.to_owned()],
            consumer_surfaces: vec![
                M5RecallConsumerSurface::ManagedOfflineReport,
                M5RecallConsumerSurface::SupportExport,
            ],
        },
    ]
}

fn guardrails() -> M5AiRecallMatrixGuardrails {
    M5AiRecallMatrixGuardrails {
        no_cross_workspace_recall_by_default: true,
        no_cross_tenant_recall_by_default: true,
        mixed_generation_embeddings_labeled: true,
        stale_hybrid_retrieval_never_current_truth: true,
        every_durable_artifact_declares_retention: true,
        caches_are_not_shadow_telemetry: true,
        spend_or_route_failures_have_precise_fallback: true,
    }
}

fn consumer_projection() -> M5AiRecallMatrixConsumerProjection {
    M5AiRecallMatrixConsumerProjection {
        composer_shows_memory_and_retrieval: true,
        docs_browser_shows_provenance_and_locality: true,
        search_shows_retrieval_lanes: true,
        support_export_shows_locality_and_receipts: true,
        diagnostics_shows_cache_and_budget: true,
        managed_offline_shows_locality_truth: true,
        unqualified_surfaces_labeled_below_stable: true,
    }
}

fn proof_freshness() -> M5AiRecallMatrixProofFreshness {
    M5AiRecallMatrixProofFreshness {
        proof_freshness_slo_hours: 168,
        last_proof_refresh: "2026-06-13T00:00:00Z".to_owned(),
        auto_narrow_on_stale: true,
    }
}

fn source_contract_refs() -> Vec<String> {
    vec![
        M5_AI_RECALL_MATRIX_SCHEMA_REF.to_owned(),
        M5_AI_RECALL_MATRIX_DOC_REF.to_owned(),
        M5_AI_RECALL_MATRIX_MEMORY_CLASS_CONTRACT_REF.to_owned(),
        M5_AI_RECALL_MATRIX_DELETE_EXPORT_CONTRACT_REF.to_owned(),
        M5_AI_RECALL_MATRIX_SPEND_RECEIPT_CONTRACT_REF.to_owned(),
        M5_AI_RECALL_MATRIX_CONTEXT_ASSEMBLY_CONTRACT_REF.to_owned(),
        M5_AI_RECALL_MATRIX_RETRIEVAL_CONTRACT_REF.to_owned(),
    ]
}

fn packet() -> M5AiRecallMatrixPacket {
    M5AiRecallMatrixPacket::new(M5AiRecallMatrixPacketInput {
        packet_id: PACKET_ID.to_owned(),
        matrix_label: "M5 AI Recall Matrix".to_owned(),
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
fn m5_ai_recall_matrix_packet_validates() {
    let packet = packet();
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
}

#[test]
fn all_required_surfaces_present() {
    let packet = packet();
    for surface in M5RecallSurface::ALL {
        assert!(packet.surface_rows.iter().any(|row| row.surface == surface));
    }
}

#[test]
fn missing_surface_fails_validation() {
    let mut packet = packet();
    packet
        .surface_rows
        .retain(|row| row.surface != M5RecallSurface::SemanticSearch);
    assert!(packet
        .validate()
        .contains(&M5AiRecallMatrixViolation::RequiredSurfaceMissing));
}

#[test]
fn stable_surface_missing_evidence_fails() {
    let mut packet = packet();
    packet.surface_rows[0].required_evidence_packet_refs.clear();
    let violations = packet.validate();
    assert!(violations.contains(&M5AiRecallMatrixViolation::StableSurfaceMissingEvidence));
    // Missing evidence also makes the surface not recall-complete, so the stable
    // claim must be flagged as exceeding its recall evidence.
    assert!(violations.contains(&M5AiRecallMatrixViolation::StableClaimExceedsRecallEvidence));
}

#[test]
fn missing_cache_invalidation_fails() {
    let mut packet = packet();
    packet.surface_rows[1].cache_invalidation_classes.clear();
    assert!(packet
        .validate()
        .contains(&M5AiRecallMatrixViolation::MissingCacheInvalidation));
}

#[test]
fn durable_memory_without_delete_export_fails() {
    let mut packet = packet();
    // ComposerAssist holds durable memory; claiming NotApplicable delete/export
    // must be rejected.
    packet.surface_rows[0].delete_export_posture = M5DeleteExportPosture::NotApplicable;
    assert!(packet
        .validate()
        .contains(&M5AiRecallMatrixViolation::DurableMemoryMissingDeleteExport));
}

#[test]
fn stable_claim_exceeding_recall_evidence_narrows() {
    let mut packet = packet();
    // Drop the retrieval lanes from a Stable surface: it is no longer
    // recall-complete and the effective qualification narrows to Preview.
    packet.surface_rows[0].retrieval_lanes.clear();
    let row = &packet.surface_rows[0];
    assert_eq!(
        row.effective_qualification(),
        M5RecallQualificationClass::Preview
    );
    assert!(packet
        .validate()
        .contains(&M5AiRecallMatrixViolation::StableClaimExceedsRecallEvidence));
}

#[test]
fn missing_downgrade_triggers_fails() {
    let mut packet = packet();
    packet.surface_rows[2].downgrade_triggers.clear();
    assert!(packet
        .validate()
        .contains(&M5AiRecallMatrixViolation::DowngradeTriggersMissing));
}

#[test]
fn missing_consumer_surfaces_fails() {
    let mut packet = packet();
    packet.surface_rows[3].consumer_surfaces.clear();
    assert!(packet
        .validate()
        .contains(&M5AiRecallMatrixViolation::ConsumerSurfacesMissing));
}

#[test]
fn missing_source_contracts_fails() {
    let mut packet = packet();
    packet.source_contract_refs.clear();
    assert!(packet
        .validate()
        .contains(&M5AiRecallMatrixViolation::MissingSourceContracts));
}

#[test]
fn guardrails_incomplete_fails() {
    let mut packet = packet();
    packet.guardrails.no_cross_tenant_recall_by_default = false;
    assert!(packet
        .validate()
        .contains(&M5AiRecallMatrixViolation::GuardrailsIncomplete));
}

#[test]
fn consumer_projection_incomplete_fails() {
    let mut packet = packet();
    packet
        .consumer_projection
        .unqualified_surfaces_labeled_below_stable = false;
    assert!(packet
        .validate()
        .contains(&M5AiRecallMatrixViolation::ConsumerProjectionIncomplete));
}

#[test]
fn proof_freshness_incomplete_fails() {
    let mut packet = packet();
    packet.proof_freshness.proof_freshness_slo_hours = 0;
    assert!(packet
        .validate()
        .contains(&M5AiRecallMatrixViolation::ProofFreshnessIncomplete));
}

#[test]
fn export_safe_json_round_trips() {
    let packet = packet();
    let json = packet.export_safe_json();
    let parsed: M5AiRecallMatrixPacket =
        serde_json::from_str(&json).expect("export json parses back");
    assert_eq!(parsed, packet);
}

#[test]
fn markdown_summary_names_surfaces() {
    let summary = packet().render_markdown_summary();
    assert!(summary.contains("M5 AI Recall Matrix"));
    assert!(summary.contains("composer_assist"));
    assert!(summary.contains("Locality:"));
}

#[test]
fn checked_support_export_validates() {
    let packet = current_stable_m5_ai_recall_matrix_export()
        .expect("checked M5 AI recall matrix export validates");
    assert_eq!(packet.packet_id, PACKET_ID);
}
