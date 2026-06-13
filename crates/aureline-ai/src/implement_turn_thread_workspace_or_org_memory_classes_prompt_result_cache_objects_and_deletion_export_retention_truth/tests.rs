use super::*;

const PACKET_ID: &str = "m5-ai-memory-class-materialization:stable:0001";

fn memory_objects() -> Vec<MemoryClassObjectRecord> {
    vec![
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
            label_summary:
                "Explicit durable saved memory retained until the user deletes it"
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
            label_summary:
                "Org shared semantic memory under a managed region gate"
                    .to_owned(),
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
    ]
}

fn guardrails() -> MemoryClassGuardrails {
    MemoryClassGuardrails {
        no_cross_workspace_memory_by_default: true,
        no_cross_tenant_memory_by_default: true,
        prompt_result_caches_not_shadow_telemetry: true,
        every_durable_class_declares_retention_delete_export: true,
        ephemeral_state_separated_from_durable_memory: true,
        missing_classes_degrade_to_precise_labels: true,
        mixed_retrieval_generations_labeled: true,
    }
}

fn consumer_projection() -> MemoryClassConsumerProjection {
    MemoryClassConsumerProjection {
        composer_shows_memory_classes: true,
        review_shows_memory_classes: true,
        docs_browser_shows_memory_classes: true,
        agent_flow_shows_memory_classes: true,
        support_export_preserves_class_distinctions: true,
        unavailable_classes_labeled_precisely: true,
    }
}

fn proof_freshness() -> MemoryClassProofFreshness {
    MemoryClassProofFreshness {
        proof_freshness_slo_hours: 168,
        last_proof_refresh: "2026-06-13T00:00:00Z".to_owned(),
        auto_narrow_on_stale: true,
    }
}

fn source_contract_refs() -> Vec<String> {
    vec![
        MEMORY_CLASS_MATERIALIZATION_SCHEMA_REF.to_owned(),
        MEMORY_CLASS_MATERIALIZATION_DOC_REF.to_owned(),
        MEMORY_CLASS_MATERIALIZATION_RECALL_MATRIX_CONTRACT_REF.to_owned(),
        MEMORY_CLASS_MATERIALIZATION_MEMORY_CLASS_CONTRACT_REF.to_owned(),
        MEMORY_CLASS_MATERIALIZATION_DELETE_EXPORT_CONTRACT_REF.to_owned(),
        MEMORY_CLASS_MATERIALIZATION_SPEND_RECEIPT_CONTRACT_REF.to_owned(),
    ]
}

fn packet() -> MemoryClassMaterializationPacket {
    MemoryClassMaterializationPacket::new(MemoryClassMaterializationPacketInput {
        packet_id: PACKET_ID.to_owned(),
        materialization_label: "Materialized AI Memory Classes".to_owned(),
        memory_objects: memory_objects(),
        guardrails: guardrails(),
        consumer_projection: consumer_projection(),
        proof_freshness: proof_freshness(),
        source_contract_refs: source_contract_refs(),
        redaction_class_token: "metadata_safe_default".to_owned(),
        minted_at: "2026-06-13T00:00:00Z".to_owned(),
    })
}

#[test]
fn packet_validates() {
    let packet = packet();
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
}

#[test]
fn all_scopes_and_classes_present() {
    let packet = packet();
    for scope in MemoryScopeClass::ALL {
        assert!(packet.materialized_scopes().contains(&scope));
    }
    for class in MemoryArtifactClass::ALL {
        assert!(packet.materialized_classes().contains(&class));
    }
}

#[test]
fn missing_scope_fails() {
    let mut packet = packet();
    packet
        .memory_objects
        .retain(|obj| obj.scope != MemoryScopeClass::Org);
    assert!(packet
        .validate()
        .contains(&MemoryClassMaterializationViolation::RequiredScopeMissing));
}

#[test]
fn missing_artifact_class_fails() {
    let mut packet = packet();
    packet
        .memory_objects
        .retain(|obj| obj.artifact_class != MemoryArtifactClass::PromptResultCache);
    assert!(packet
        .validate()
        .contains(&MemoryClassMaterializationViolation::RequiredArtifactClassMissing));
}

#[test]
fn durable_object_without_delete_export_fails() {
    let mut packet = packet();
    // workspace-saved-memory is durable; claiming not-applicable delete must fail.
    packet.memory_objects[4].delete_posture = MemoryDeleteExportPosture::NotApplicable;
    let violations = packet.validate();
    assert!(
        violations.contains(&MemoryClassMaterializationViolation::DurableObjectMissingDeleteExport)
    );
}

#[test]
fn ephemeral_with_durable_retention_fails() {
    let mut packet = packet();
    // turn-ephemeral-state must not claim a durable retention class.
    packet.memory_objects[0].retention = MemoryRetentionClass::DurableUntilDeleted;
    assert!(packet
        .validate()
        .contains(&MemoryClassMaterializationViolation::RetentionClassMismatch));
}

#[test]
fn durable_with_session_retention_fails() {
    let mut packet = packet();
    // workspace-saved-memory is durable; session-only retention is inconsistent.
    packet.memory_objects[4].retention = MemoryRetentionClass::SessionOnly;
    assert!(packet
        .validate()
        .contains(&MemoryClassMaterializationViolation::RetentionClassMismatch));
}

#[test]
fn locality_exceeding_scope_fails() {
    let mut packet = packet();
    // A workspace object claiming tenant-region locality widens beyond its scope.
    packet.memory_objects[3].locality = MemoryLocalityClass::TenantRegionPinned;
    assert!(packet
        .validate()
        .contains(&MemoryClassMaterializationViolation::LocalityExceedsScope));
}

#[test]
fn prompt_result_cache_without_content_hash_fails() {
    let mut packet = packet();
    // thread-prompt-result-cache must stay content-hash keyed.
    packet.memory_objects[2]
        .invalidation_classes
        .retain(|class| *class != MemoryInvalidationClass::ContentHashKey);
    assert!(packet
        .validate()
        .contains(&MemoryClassMaterializationViolation::PromptResultCacheUnbounded));
}

#[test]
fn prompt_result_cache_durable_retention_fails() {
    let mut packet = packet();
    // A prompt-result cache claiming durable-until-deleted is shadow telemetry.
    packet.memory_objects[2].retention = MemoryRetentionClass::DurableUntilDeleted;
    let violations = packet.validate();
    assert!(violations.contains(&MemoryClassMaterializationViolation::PromptResultCacheUnbounded));
}

#[test]
fn non_available_without_label_fails() {
    let mut packet = packet();
    // org-semantic-memory-region-blocked is non-available; clearing its precise
    // label must fail rather than collapse into a generic state.
    packet.memory_objects[6].degraded_label = None;
    assert!(packet
        .validate()
        .contains(&MemoryClassMaterializationViolation::AvailabilityLabelMissing));
}

#[test]
fn non_available_with_generic_label_fails() {
    let mut packet = packet();
    packet.memory_objects[6].degraded_label = Some("memory unavailable".to_owned());
    assert!(packet
        .validate()
        .contains(&MemoryClassMaterializationViolation::AvailabilityLabelMissing));
}

#[test]
fn missing_consumer_flows_fails() {
    let mut packet = packet();
    packet.memory_objects[1].consumer_flows.clear();
    assert!(packet
        .validate()
        .contains(&MemoryClassMaterializationViolation::ConsumerFlowsMissing));
}

#[test]
fn missing_source_contracts_fails() {
    let mut packet = packet();
    packet.source_contract_refs.clear();
    assert!(packet
        .validate()
        .contains(&MemoryClassMaterializationViolation::MissingSourceContracts));
}

#[test]
fn guardrails_incomplete_fails() {
    let mut packet = packet();
    packet.guardrails.prompt_result_caches_not_shadow_telemetry = false;
    assert!(packet
        .validate()
        .contains(&MemoryClassMaterializationViolation::GuardrailsIncomplete));
}

#[test]
fn consumer_projection_incomplete_fails() {
    let mut packet = packet();
    packet
        .consumer_projection
        .unavailable_classes_labeled_precisely = false;
    assert!(packet
        .validate()
        .contains(&MemoryClassMaterializationViolation::ConsumerProjectionIncomplete));
}

#[test]
fn proof_freshness_incomplete_fails() {
    let mut packet = packet();
    packet.proof_freshness.proof_freshness_slo_hours = 0;
    assert!(packet
        .validate()
        .contains(&MemoryClassMaterializationViolation::ProofFreshnessIncomplete));
}

#[test]
fn export_safe_json_round_trips() {
    let packet = packet();
    let json = packet.export_safe_json();
    let parsed: MemoryClassMaterializationPacket =
        serde_json::from_str(&json).expect("export json parses back");
    assert_eq!(parsed, packet);
}

#[test]
fn markdown_summary_names_objects() {
    let summary = packet().render_markdown_summary();
    assert!(summary.contains("Materialized AI Memory Classes"));
    assert!(summary.contains("prompt_result_cache"));
    assert!(summary.contains("Degraded:"));
}

#[test]
fn checked_support_export_validates() {
    let packet = current_stable_memory_class_materialization_export()
        .expect("checked memory class materialization export validates");
    assert_eq!(packet.packet_id, PACKET_ID);
}
