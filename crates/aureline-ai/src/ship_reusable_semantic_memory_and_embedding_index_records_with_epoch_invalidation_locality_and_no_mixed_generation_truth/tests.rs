use super::*;

const PACKET_ID: &str = "m5-semantic-recall-records:stable:0001";

fn lineage(graph: &str, generation: &str) -> EpochLineage {
    EpochLineage {
        graph_epoch: graph.to_owned(),
        docs_epoch: "docs-epoch-2026-06-a".to_owned(),
        model_epoch: "model-epoch-embed-3".to_owned(),
        embedding_generation: generation.to_owned(),
    }
}

fn records() -> Vec<SemanticRecallRecord> {
    vec![
        SemanticRecallRecord {
            record_id: "workspace-semantic-memory-local".to_owned(),
            artifact_kind: SemanticArtifactKind::ReusableSemanticMemory,
            label_summary:
                "Workspace reusable semantic memory built on-device from the workspace graph and docs corpus"
                    .to_owned(),
            epoch_lineage: lineage("graph-epoch-2026-06-a", "gen-3"),
            bound_epochs: vec![EpochKind::Graph, EpochKind::Docs, EpochKind::Model],
            invalidation_triggers: vec![
                EpochInvalidationTrigger::GraphEpochBump,
                EpochInvalidationTrigger::DocsEpochBump,
                EpochInvalidationTrigger::ModelEpochBump,
                EpochInvalidationTrigger::EmbeddingGenerationBump,
                EpochInvalidationTrigger::ContentHashKey,
            ],
            generation_state: RetrievalGenerationState::Current,
            mixed_generation_detected: false,
            locality: SemanticLocalityClass::LocalDeviceOnly,
            delete_posture: SemanticDeleteExportPosture::UserScoped,
            export_posture: SemanticDeleteExportPosture::UserScoped,
            consumer_surfaces: vec![
                RecallConsumerSurface::ComposerAssist,
                RecallConsumerSurface::CodeUnderstanding,
            ],
            degraded_label: None,
            evidence_refs: vec!["evidence:reusable-semantic-memory:m5".to_owned()],
            source_contract_refs: vec![
                SEMANTIC_RECALL_RECORDS_MEMORY_CLASS_CONTRACT_REF.to_owned(),
            ],
        },
        SemanticRecallRecord {
            record_id: "workspace-embedding-index-mirrored".to_owned(),
            artifact_kind: SemanticArtifactKind::EmbeddingIndex,
            label_summary:
                "Workspace-mirrored embedding index over the docs corpus, single embedding generation"
                    .to_owned(),
            epoch_lineage: lineage("graph-epoch-2026-06-a", "gen-3"),
            bound_epochs: vec![EpochKind::Docs, EpochKind::Model],
            invalidation_triggers: vec![
                EpochInvalidationTrigger::DocsEpochBump,
                EpochInvalidationTrigger::ModelEpochBump,
                EpochInvalidationTrigger::EmbeddingGenerationBump,
                EpochInvalidationTrigger::ContentHashKey,
            ],
            generation_state: RetrievalGenerationState::Current,
            mixed_generation_detected: false,
            locality: SemanticLocalityClass::WorkspaceMirrored,
            delete_posture: SemanticDeleteExportPosture::WorkspaceScoped,
            export_posture: SemanticDeleteExportPosture::WorkspaceScoped,
            consumer_surfaces: vec![
                RecallConsumerSurface::SemanticSearch,
                RecallConsumerSurface::DocsBrowserRecall,
            ],
            degraded_label: None,
            evidence_refs: vec!["evidence:embedding-index:m5".to_owned()],
            source_contract_refs: vec![SEMANTIC_RECALL_RECORDS_RETRIEVAL_CONTRACT_REF.to_owned()],
        },
        SemanticRecallRecord {
            record_id: "managed-embedding-index-recomputing".to_owned(),
            artifact_kind: SemanticArtifactKind::EmbeddingIndex,
            label_summary: "Managed-hosted embedding index recomputing after a model epoch bump"
                .to_owned(),
            epoch_lineage: EpochLineage {
                graph_epoch: "graph-epoch-2026-06-a".to_owned(),
                docs_epoch: "docs-epoch-2026-06-a".to_owned(),
                model_epoch: "model-epoch-embed-4".to_owned(),
                embedding_generation: "gen-4".to_owned(),
            },
            bound_epochs: vec![EpochKind::Graph, EpochKind::Model],
            invalidation_triggers: vec![
                EpochInvalidationTrigger::GraphEpochBump,
                EpochInvalidationTrigger::ModelEpochBump,
                EpochInvalidationTrigger::EmbeddingGenerationBump,
            ],
            generation_state: RetrievalGenerationState::Recomputing,
            mixed_generation_detected: false,
            locality: SemanticLocalityClass::ManagedHosted,
            delete_posture: SemanticDeleteExportPosture::TenantScoped,
            export_posture: SemanticDeleteExportPosture::TenantScoped,
            consumer_surfaces: vec![
                RecallConsumerSurface::SemanticSearch,
                RecallConsumerSurface::ManagedOfflineReport,
            ],
            degraded_label: Some(
                "Managed embedding index recomputing after model epoch bump to gen-4; results labeled recomputing, not served as current retrieval truth"
                    .to_owned(),
            ),
            evidence_refs: vec!["evidence:embedding-index:m5".to_owned()],
            source_contract_refs: vec![SEMANTIC_RECALL_RECORDS_RETRIEVAL_CONTRACT_REF.to_owned()],
        },
        SemanticRecallRecord {
            record_id: "managed-semantic-memory-mixed-blocked".to_owned(),
            artifact_kind: SemanticArtifactKind::ReusableSemanticMemory,
            label_summary:
                "Managed-hosted reusable semantic memory spanning two embedding generations"
                    .to_owned(),
            epoch_lineage: EpochLineage {
                graph_epoch: "graph-epoch-2026-06-a".to_owned(),
                docs_epoch: "docs-epoch-2026-06-a".to_owned(),
                model_epoch: "model-epoch-embed-4".to_owned(),
                embedding_generation: "gen-3+gen-4".to_owned(),
            },
            bound_epochs: vec![EpochKind::Docs, EpochKind::Model],
            invalidation_triggers: vec![
                EpochInvalidationTrigger::DocsEpochBump,
                EpochInvalidationTrigger::ModelEpochBump,
                EpochInvalidationTrigger::EmbeddingGenerationBump,
            ],
            generation_state: RetrievalGenerationState::MixedBlocked,
            mixed_generation_detected: true,
            locality: SemanticLocalityClass::ManagedHosted,
            delete_posture: SemanticDeleteExportPosture::TenantScoped,
            export_posture: SemanticDeleteExportPosture::TenantScoped,
            consumer_surfaces: vec![RecallConsumerSurface::SemanticSearch],
            degraded_label: Some(
                "Managed semantic memory blocked: entries span embedding generations gen-3 and gen-4; withheld from current retrieval truth until recomputed onto one generation"
                    .to_owned(),
            ),
            evidence_refs: vec!["evidence:reusable-semantic-memory:m5".to_owned()],
            source_contract_refs: vec![
                SEMANTIC_RECALL_RECORDS_CONTEXT_ASSEMBLY_CONTRACT_REF.to_owned(),
            ],
        },
        SemanticRecallRecord {
            record_id: "workspace-embedding-index-invalidated".to_owned(),
            artifact_kind: SemanticArtifactKind::EmbeddingIndex,
            label_summary: "Workspace-mirrored embedding index invalidated by a graph epoch bump"
                .to_owned(),
            epoch_lineage: lineage("graph-epoch-2026-05-z", "gen-3"),
            bound_epochs: vec![EpochKind::Graph, EpochKind::Model],
            invalidation_triggers: vec![
                EpochInvalidationTrigger::GraphEpochBump,
                EpochInvalidationTrigger::ModelEpochBump,
                EpochInvalidationTrigger::EmbeddingGenerationBump,
            ],
            generation_state: RetrievalGenerationState::Invalidated,
            mixed_generation_detected: false,
            locality: SemanticLocalityClass::WorkspaceMirrored,
            delete_posture: SemanticDeleteExportPosture::WorkspaceScoped,
            export_posture: SemanticDeleteExportPosture::WorkspaceScoped,
            consumer_surfaces: vec![RecallConsumerSurface::CodeUnderstanding],
            degraded_label: Some(
                "Workspace embedding index invalidated: built on graph epoch graph-epoch-2026-05-z, superseded by graph-epoch-2026-06-a; awaiting recompute"
                    .to_owned(),
            ),
            evidence_refs: vec!["evidence:embedding-index:m5".to_owned()],
            source_contract_refs: vec![SEMANTIC_RECALL_RECORDS_RETRIEVAL_CONTRACT_REF.to_owned()],
        },
        SemanticRecallRecord {
            record_id: "org-semantic-memory-policy-blocked".to_owned(),
            artifact_kind: SemanticArtifactKind::ReusableSemanticMemory,
            label_summary: "Org reusable semantic memory under a managed region gate".to_owned(),
            epoch_lineage: lineage("graph-epoch-2026-06-a", "gen-3"),
            bound_epochs: vec![EpochKind::Docs, EpochKind::Model],
            invalidation_triggers: vec![
                EpochInvalidationTrigger::DocsEpochBump,
                EpochInvalidationTrigger::ModelEpochBump,
                EpochInvalidationTrigger::EmbeddingGenerationBump,
            ],
            generation_state: RetrievalGenerationState::Current,
            mixed_generation_detected: false,
            locality: SemanticLocalityClass::PolicyBlocked,
            delete_posture: SemanticDeleteExportPosture::OrgScoped,
            export_posture: SemanticDeleteExportPosture::OrgScoped,
            consumer_surfaces: vec![
                RecallConsumerSurface::DocsBrowserRecall,
                RecallConsumerSurface::SupportExport,
            ],
            degraded_label: Some(
                "Org reusable semantic memory withheld: region policy gate blocks managed retrieval in this tenant region; delete and export remain org-scoped"
                    .to_owned(),
            ),
            evidence_refs: vec!["evidence:reusable-semantic-memory:m5".to_owned()],
            source_contract_refs: vec![
                SEMANTIC_RECALL_RECORDS_DELETE_EXPORT_CONTRACT_REF.to_owned(),
            ],
        },
    ]
}

fn guardrails() -> SemanticRecordGuardrails {
    SemanticRecordGuardrails {
        no_cross_workspace_recall_by_default: true,
        no_cross_tenant_recall_by_default: true,
        mixed_generation_never_current_truth: true,
        invalidated_or_recomputing_lanes_labeled: true,
        every_durable_record_declares_delete_export: true,
        locality_states_remain_distinct: true,
        epoch_bump_invalidates_bound_records: true,
    }
}

fn consumer_projection() -> SemanticRecordConsumerProjection {
    SemanticRecordConsumerProjection {
        composer_shows_memory_and_generation: true,
        docs_browser_shows_provenance_and_locality: true,
        search_shows_retrieval_generation: true,
        support_export_shows_locality_and_epochs: true,
        managed_offline_shows_locality_truth: true,
        invalidated_or_recomputing_lanes_labeled_below_current: true,
    }
}

fn proof_freshness() -> SemanticRecordProofFreshness {
    SemanticRecordProofFreshness {
        proof_freshness_slo_hours: 168,
        last_proof_refresh: "2026-06-13T00:00:00Z".to_owned(),
        auto_narrow_on_stale: true,
    }
}

fn source_contract_refs() -> Vec<String> {
    vec![
        SEMANTIC_RECALL_RECORDS_SCHEMA_REF.to_owned(),
        SEMANTIC_RECALL_RECORDS_DOC_REF.to_owned(),
        SEMANTIC_RECALL_RECORDS_RECALL_MATRIX_CONTRACT_REF.to_owned(),
        SEMANTIC_RECALL_RECORDS_MEMORY_CLASS_CONTRACT_REF.to_owned(),
        SEMANTIC_RECALL_RECORDS_DELETE_EXPORT_CONTRACT_REF.to_owned(),
        SEMANTIC_RECALL_RECORDS_CONTEXT_ASSEMBLY_CONTRACT_REF.to_owned(),
        SEMANTIC_RECALL_RECORDS_RETRIEVAL_CONTRACT_REF.to_owned(),
    ]
}

fn packet() -> SemanticRecallRecordsPacket {
    SemanticRecallRecordsPacket::new(SemanticRecallRecordsPacketInput {
        packet_id: PACKET_ID.to_owned(),
        records_label: "Reusable Semantic-Memory and Embedding-Index Records".to_owned(),
        records: records(),
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
fn all_kinds_epochs_and_localities_present() {
    let packet = packet();
    for kind in SemanticArtifactKind::ALL {
        assert!(packet.materialized_kinds().contains(&kind));
    }
    for epoch in EpochKind::ALL {
        assert!(packet.bound_epoch_kinds().contains(&epoch));
    }
    for locality in SemanticLocalityClass::ALL {
        assert!(packet.represented_localities().contains(&locality));
    }
}

#[test]
fn missing_artifact_kind_fails() {
    let mut packet = packet();
    packet
        .records
        .retain(|rec| rec.artifact_kind != SemanticArtifactKind::EmbeddingIndex);
    assert!(packet
        .validate()
        .contains(&SemanticRecallRecordsViolation::RequiredArtifactKindMissing));
}

#[test]
fn missing_epoch_coverage_fails() {
    let mut packet = packet();
    // Drop the graph epoch from every record's bound set.
    for rec in &mut packet.records {
        rec.bound_epochs.retain(|epoch| *epoch != EpochKind::Graph);
    }
    assert!(packet
        .validate()
        .contains(&SemanticRecallRecordsViolation::RequiredEpochCoverageMissing));
}

#[test]
fn missing_locality_coverage_fails() {
    let mut packet = packet();
    // Re-home the only policy-blocked record so that locality is unrepresented.
    packet.records[5].locality = SemanticLocalityClass::ManagedHosted;
    assert!(packet
        .validate()
        .contains(&SemanticRecallRecordsViolation::RequiredLocalityCoverageMissing));
}

#[test]
fn no_invalidated_lane_case_fails() {
    let mut packet = packet();
    // Force every record to current truth; nothing demonstrates a labeled lane.
    for rec in &mut packet.records {
        rec.generation_state = RetrievalGenerationState::Current;
        rec.mixed_generation_detected = false;
    }
    assert!(packet
        .validate()
        .contains(&SemanticRecallRecordsViolation::InvalidatedLaneCaseMissing));
}

#[test]
fn mixed_generation_current_truth_fails() {
    let mut packet = packet();
    // A mixed-generation record claiming current retrieval truth must fail.
    packet.records[3].generation_state = RetrievalGenerationState::Current;
    let violations = packet.validate();
    assert!(
        violations.contains(&SemanticRecallRecordsViolation::MixedGenerationMasqueradesAsCurrent)
    );
}

#[test]
fn embedding_index_without_generation_bump_fails() {
    let mut packet = packet();
    // workspace-embedding-index-mirrored must bind the embedding-generation bump.
    packet.records[1]
        .invalidation_triggers
        .retain(|trigger| *trigger != EpochInvalidationTrigger::EmbeddingGenerationBump);
    assert!(packet
        .validate()
        .contains(&SemanticRecallRecordsViolation::EmbeddingIndexGenerationUngoverned));
}

#[test]
fn bound_epoch_without_trigger_fails() {
    let mut packet = packet();
    // Bind the graph epoch but drop its bump trigger.
    packet.records[1].bound_epochs.push(EpochKind::Graph);
    assert!(packet
        .validate()
        .contains(&SemanticRecallRecordsViolation::BoundEpochTriggerMissing));
}

#[test]
fn incomplete_epoch_lineage_fails() {
    let mut packet = packet();
    packet.records[0].epoch_lineage.model_epoch = String::new();
    assert!(packet
        .validate()
        .contains(&SemanticRecallRecordsViolation::EpochLineageIncomplete));
}

#[test]
fn durable_record_without_delete_export_fails() {
    let mut packet = packet();
    packet.records[0].delete_posture = SemanticDeleteExportPosture::NotApplicable;
    assert!(packet
        .validate()
        .contains(&SemanticRecallRecordsViolation::DurableRecordMissingDeleteExport));
}

#[test]
fn non_current_without_label_fails() {
    let mut packet = packet();
    // managed-embedding-index-recomputing is non-current; clearing its label fails.
    packet.records[2].degraded_label = None;
    assert!(packet
        .validate()
        .contains(&SemanticRecallRecordsViolation::DegradedLabelMissing));
}

#[test]
fn policy_blocked_without_label_fails() {
    let mut packet = packet();
    // org-semantic-memory-policy-blocked is current truth but policy-blocked, so a
    // precise label is still required.
    packet.records[5].degraded_label = None;
    assert!(packet
        .validate()
        .contains(&SemanticRecallRecordsViolation::DegradedLabelMissing));
}

#[test]
fn generic_label_fails() {
    let mut packet = packet();
    packet.records[2].degraded_label = Some("retrieval unavailable".to_owned());
    assert!(packet
        .validate()
        .contains(&SemanticRecallRecordsViolation::DegradedLabelMissing));
}

#[test]
fn missing_consumer_surfaces_fails() {
    let mut packet = packet();
    packet.records[0].consumer_surfaces.clear();
    assert!(packet
        .validate()
        .contains(&SemanticRecallRecordsViolation::ConsumerSurfacesMissing));
}

#[test]
fn missing_source_contracts_fails() {
    let mut packet = packet();
    packet.source_contract_refs.clear();
    assert!(packet
        .validate()
        .contains(&SemanticRecallRecordsViolation::MissingSourceContracts));
}

#[test]
fn guardrails_incomplete_fails() {
    let mut packet = packet();
    packet.guardrails.mixed_generation_never_current_truth = false;
    assert!(packet
        .validate()
        .contains(&SemanticRecallRecordsViolation::GuardrailsIncomplete));
}

#[test]
fn consumer_projection_incomplete_fails() {
    let mut packet = packet();
    packet
        .consumer_projection
        .invalidated_or_recomputing_lanes_labeled_below_current = false;
    assert!(packet
        .validate()
        .contains(&SemanticRecallRecordsViolation::ConsumerProjectionIncomplete));
}

#[test]
fn proof_freshness_incomplete_fails() {
    let mut packet = packet();
    packet.proof_freshness.proof_freshness_slo_hours = 0;
    assert!(packet
        .validate()
        .contains(&SemanticRecallRecordsViolation::ProofFreshnessIncomplete));
}

#[test]
fn export_safe_json_round_trips() {
    let packet = packet();
    let json = packet.export_safe_json();
    let parsed: SemanticRecallRecordsPacket =
        serde_json::from_str(&json).expect("export json parses back");
    assert_eq!(parsed, packet);
}

#[test]
fn markdown_summary_names_records() {
    let summary = packet().render_markdown_summary();
    assert!(summary.contains("Reusable Semantic-Memory and Embedding-Index Records"));
    assert!(summary.contains("embedding_index"));
    assert!(summary.contains("mixed_blocked"));
    assert!(summary.contains("Degraded:"));
}

#[test]
fn checked_support_export_validates() {
    let packet = current_stable_semantic_recall_records_export()
        .expect("checked semantic recall records export validates");
    assert_eq!(packet.packet_id, PACKET_ID);
}
