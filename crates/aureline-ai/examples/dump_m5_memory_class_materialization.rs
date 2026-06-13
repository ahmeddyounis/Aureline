//! Conformance dump for the materialized AI memory-class packet.
//!
//! Prints the export-safe JSON of the canonical stable materialization. The
//! output is the checked-in support export under
//! `artifacts/ai/m5/implement_turn_thread_workspace_or_org_memory_classes_prompt_result_cache_objects_and_deletion_export_retention_truth/`.

use aureline_ai::implement_turn_thread_workspace_or_org_memory_classes_prompt_result_cache_objects_and_deletion_export_retention_truth::{
    MemoryArtifactClass, MemoryAvailabilityClass, MemoryClassConsumerProjection,
    MemoryClassGuardrails, MemoryClassMaterializationPacket, MemoryClassMaterializationPacketInput,
    MemoryClassObjectRecord, MemoryClassProofFreshness, MemoryConsumerFlow,
    MemoryDeleteExportPosture, MemoryInvalidationClass, MemoryLocalityClass, MemoryRetentionClass,
    MemoryScopeClass, MEMORY_CLASS_MATERIALIZATION_DELETE_EXPORT_CONTRACT_REF,
    MEMORY_CLASS_MATERIALIZATION_DOC_REF, MEMORY_CLASS_MATERIALIZATION_MEMORY_CLASS_CONTRACT_REF,
    MEMORY_CLASS_MATERIALIZATION_RECALL_MATRIX_CONTRACT_REF,
    MEMORY_CLASS_MATERIALIZATION_SCHEMA_REF, MEMORY_CLASS_MATERIALIZATION_SPEND_RECEIPT_CONTRACT_REF,
};

fn main() {
    let packet = canonical_packet();
    let violations = packet.validate();
    assert!(
        violations.is_empty(),
        "canonical materialization invalid: {violations:?}"
    );
    println!("{}", packet.export_safe_json());
}

fn canonical_packet() -> MemoryClassMaterializationPacket {
    MemoryClassMaterializationPacket::new(MemoryClassMaterializationPacketInput {
        packet_id: "m5-ai-memory-class-materialization:stable:0001".to_owned(),
        materialization_label: "Materialized AI Memory Classes".to_owned(),
        memory_objects: vec![
            MemoryClassObjectRecord {
                object_id: "turn-ephemeral-state".to_owned(),
                scope: MemoryScopeClass::Turn,
                artifact_class: MemoryArtifactClass::EphemeralTurnState,
                label_summary:
                    "Per-turn composer working state cleared when the turn ends; never durably retained"
                        .to_owned(),
                retention: MemoryRetentionClass::SessionOnly,
                delete_posture: MemoryDeleteExportPosture::EphemeralAutoExpire,
                export_posture: MemoryDeleteExportPosture::EphemeralAutoExpire,
                locality: MemoryLocalityClass::LocalDeviceOnly,
                invalidation_classes: vec![
                    MemoryInvalidationClass::TtlExpiry,
                    MemoryInvalidationClass::ManualPurge,
                ],
                availability: MemoryAvailabilityClass::Available,
                degraded_label: None,
                consumer_flows: vec![
                    MemoryConsumerFlow::ComposerAssist,
                    MemoryConsumerFlow::PatchReview,
                    MemoryConsumerFlow::BranchAgent,
                ],
                evidence_refs: vec!["evidence:ai-memory-state:m5".to_owned()],
                source_contract_refs: vec![
                    MEMORY_CLASS_MATERIALIZATION_MEMORY_CLASS_CONTRACT_REF.to_owned(),
                ],
            },
            MemoryClassObjectRecord {
                object_id: "thread-derived-cache".to_owned(),
                scope: MemoryScopeClass::Thread,
                artifact_class: MemoryArtifactClass::EvictableDerivedCache,
                label_summary:
                    "Thread-scoped derived context cache; recomputable, evictable, not authoritative"
                        .to_owned(),
                retention: MemoryRetentionClass::UntilManualEvict,
                delete_posture: MemoryDeleteExportPosture::UserScoped,
                export_posture: MemoryDeleteExportPosture::UserScoped,
                locality: MemoryLocalityClass::WorkspaceLocal,
                invalidation_classes: vec![
                    MemoryInvalidationClass::ContentHashKey,
                    MemoryInvalidationClass::TtlExpiry,
                    MemoryInvalidationClass::PolicyEpochBump,
                ],
                availability: MemoryAvailabilityClass::Available,
                degraded_label: None,
                consumer_flows: vec![
                    MemoryConsumerFlow::ComposerAssist,
                    MemoryConsumerFlow::DocsBrowserRecall,
                ],
                evidence_refs: vec!["evidence:ai-memory-state:m5".to_owned()],
                source_contract_refs: vec![
                    MEMORY_CLASS_MATERIALIZATION_MEMORY_CLASS_CONTRACT_REF.to_owned(),
                ],
            },
            MemoryClassObjectRecord {
                object_id: "thread-prompt-result-cache".to_owned(),
                scope: MemoryScopeClass::Thread,
                artifact_class: MemoryArtifactClass::PromptResultCache,
                label_summary:
                    "Prompt-result cache keyed by content hash with a bounded TTL; never shadow telemetry"
                        .to_owned(),
                retention: MemoryRetentionClass::TtlBounded,
                delete_posture: MemoryDeleteExportPosture::UserScoped,
                export_posture: MemoryDeleteExportPosture::UserScoped,
                locality: MemoryLocalityClass::WorkspaceLocal,
                invalidation_classes: vec![
                    MemoryInvalidationClass::ContentHashKey,
                    MemoryInvalidationClass::TtlExpiry,
                    MemoryInvalidationClass::PolicyEpochBump,
                    MemoryInvalidationClass::TrustNarrowing,
                ],
                availability: MemoryAvailabilityClass::Available,
                degraded_label: None,
                consumer_flows: vec![
                    MemoryConsumerFlow::ComposerAssist,
                    MemoryConsumerFlow::PatchReview,
                    MemoryConsumerFlow::DocsBrowserRecall,
                ],
                evidence_refs: vec!["evidence:prompt-result-cache:m5".to_owned()],
                source_contract_refs: vec![
                    MEMORY_CLASS_MATERIALIZATION_SPEND_RECEIPT_CONTRACT_REF.to_owned(),
                ],
            },
            MemoryClassObjectRecord {
                object_id: "workspace-semantic-memory".to_owned(),
                scope: MemoryScopeClass::Workspace,
                artifact_class: MemoryArtifactClass::ReusableSemanticMemory,
                label_summary:
                    "Workspace reusable semantic memory retained until the user revokes consent"
                        .to_owned(),
                retention: MemoryRetentionClass::UntilUserRevoked,
                delete_posture: MemoryDeleteExportPosture::WorkspaceScoped,
                export_posture: MemoryDeleteExportPosture::WorkspaceScoped,
                locality: MemoryLocalityClass::WorkspaceLocal,
                invalidation_classes: vec![
                    MemoryInvalidationClass::EmbeddingGenerationBump,
                    MemoryInvalidationClass::ContentHashKey,
                    MemoryInvalidationClass::ScopeRevocation,
                ],
                availability: MemoryAvailabilityClass::Available,
                degraded_label: None,
                consumer_flows: vec![
                    MemoryConsumerFlow::ComposerAssist,
                    MemoryConsumerFlow::PatchReview,
                    MemoryConsumerFlow::DocsBrowserRecall,
                    MemoryConsumerFlow::BranchAgent,
                ],
                evidence_refs: vec!["evidence:reusable-semantic-memory:m5".to_owned()],
                source_contract_refs: vec![
                    MEMORY_CLASS_MATERIALIZATION_DELETE_EXPORT_CONTRACT_REF.to_owned(),
                ],
            },
            MemoryClassObjectRecord {
                object_id: "workspace-saved-memory".to_owned(),
                scope: MemoryScopeClass::Workspace,
                artifact_class: MemoryArtifactClass::DurableSavedMemory,
                label_summary: "Explicit durable saved memory retained until the user deletes it"
                    .to_owned(),
                retention: MemoryRetentionClass::DurableUntilDeleted,
                delete_posture: MemoryDeleteExportPosture::WorkspaceScoped,
                export_posture: MemoryDeleteExportPosture::WorkspaceScoped,
                locality: MemoryLocalityClass::WorkspaceLocal,
                invalidation_classes: vec![
                    MemoryInvalidationClass::ScopeRevocation,
                    MemoryInvalidationClass::PolicyEpochBump,
                    MemoryInvalidationClass::ManualPurge,
                ],
                availability: MemoryAvailabilityClass::Available,
                degraded_label: None,
                consumer_flows: vec![
                    MemoryConsumerFlow::ComposerAssist,
                    MemoryConsumerFlow::PatchReview,
                    MemoryConsumerFlow::BranchAgent,
                ],
                evidence_refs: vec!["evidence:durable-saved-memory:m5".to_owned()],
                source_contract_refs: vec![
                    MEMORY_CLASS_MATERIALIZATION_DELETE_EXPORT_CONTRACT_REF.to_owned(),
                ],
            },
            MemoryClassObjectRecord {
                object_id: "org-saved-memory".to_owned(),
                scope: MemoryScopeClass::Org,
                artifact_class: MemoryArtifactClass::DurableSavedMemory,
                label_summary:
                    "Org/tenant-scoped durable saved memory, region-pinned, no cross-tenant recall"
                        .to_owned(),
                retention: MemoryRetentionClass::DurableUntilDeleted,
                delete_posture: MemoryDeleteExportPosture::OrgScoped,
                export_posture: MemoryDeleteExportPosture::OrgScoped,
                locality: MemoryLocalityClass::TenantRegionPinned,
                invalidation_classes: vec![
                    MemoryInvalidationClass::ScopeRevocation,
                    MemoryInvalidationClass::PolicyEpochBump,
                    MemoryInvalidationClass::TrustNarrowing,
                    MemoryInvalidationClass::ManualPurge,
                ],
                availability: MemoryAvailabilityClass::Available,
                degraded_label: None,
                consumer_flows: vec![
                    MemoryConsumerFlow::ComposerAssist,
                    MemoryConsumerFlow::DocsBrowserRecall,
                ],
                evidence_refs: vec!["evidence:org-saved-memory:m5".to_owned()],
                source_contract_refs: vec![
                    MEMORY_CLASS_MATERIALIZATION_DELETE_EXPORT_CONTRACT_REF.to_owned(),
                ],
            },
            MemoryClassObjectRecord {
                object_id: "org-semantic-memory-region-blocked".to_owned(),
                scope: MemoryScopeClass::Org,
                artifact_class: MemoryArtifactClass::ReusableSemanticMemory,
                label_summary: "Org shared semantic memory under a managed region gate".to_owned(),
                retention: MemoryRetentionClass::EvidenceRetentionHold,
                delete_posture: MemoryDeleteExportPosture::OrgScoped,
                export_posture: MemoryDeleteExportPosture::OrgScoped,
                locality: MemoryLocalityClass::ManagedHostedRegionPinned,
                invalidation_classes: vec![
                    MemoryInvalidationClass::EmbeddingGenerationBump,
                    MemoryInvalidationClass::ScopeRevocation,
                    MemoryInvalidationClass::PolicyEpochBump,
                ],
                availability: MemoryAvailabilityClass::PolicyBlockedRegionGate,
                degraded_label: Some(
                    "Org reusable semantic memory narrowed: region gate blocks managed retrieval in this tenant region"
                        .to_owned(),
                ),
                consumer_flows: vec![MemoryConsumerFlow::DocsBrowserRecall],
                evidence_refs: vec!["evidence:org-semantic-memory:m5".to_owned()],
                source_contract_refs: vec![
                    MEMORY_CLASS_MATERIALIZATION_DELETE_EXPORT_CONTRACT_REF.to_owned(),
                ],
            },
        ],
        guardrails: MemoryClassGuardrails {
            no_cross_workspace_memory_by_default: true,
            no_cross_tenant_memory_by_default: true,
            prompt_result_caches_not_shadow_telemetry: true,
            every_durable_class_declares_retention_delete_export: true,
            ephemeral_state_separated_from_durable_memory: true,
            missing_classes_degrade_to_precise_labels: true,
            mixed_retrieval_generations_labeled: true,
        },
        consumer_projection: MemoryClassConsumerProjection {
            composer_shows_memory_classes: true,
            review_shows_memory_classes: true,
            docs_browser_shows_memory_classes: true,
            agent_flow_shows_memory_classes: true,
            support_export_preserves_class_distinctions: true,
            unavailable_classes_labeled_precisely: true,
        },
        proof_freshness: MemoryClassProofFreshness {
            proof_freshness_slo_hours: 168,
            last_proof_refresh: "2026-06-13T00:00:00Z".to_owned(),
            auto_narrow_on_stale: true,
        },
        source_contract_refs: vec![
            MEMORY_CLASS_MATERIALIZATION_SCHEMA_REF.to_owned(),
            MEMORY_CLASS_MATERIALIZATION_DOC_REF.to_owned(),
            MEMORY_CLASS_MATERIALIZATION_RECALL_MATRIX_CONTRACT_REF.to_owned(),
            MEMORY_CLASS_MATERIALIZATION_MEMORY_CLASS_CONTRACT_REF.to_owned(),
            MEMORY_CLASS_MATERIALIZATION_DELETE_EXPORT_CONTRACT_REF.to_owned(),
            MEMORY_CLASS_MATERIALIZATION_SPEND_RECEIPT_CONTRACT_REF.to_owned(),
        ],
        redaction_class_token: "metadata_safe_default".to_owned(),
        minted_at: "2026-06-13T00:00:00Z".to_owned(),
    })
}
