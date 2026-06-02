use super::*;

const EVIDENCE_ID: &str = "ai-evidence:apply:stable:0001";
const TURN_REF: &str = "turn-draft:apply:stable:0001";
const RUN_REF: &str = "ai-run:apply:stable:0001";

fn intent_scope() -> IntentScopeBlock {
    IntentScopeBlock {
        intent_summary_label: "Apply the retry-backoff fix the operator requested.".to_owned(),
        requested_scope_label: "Edit the retry module and its unit tests only.".to_owned(),
        intent_class_token: "local_reversible_edit".to_owned(),
        requested_write_scope_label: "Two workspace files in the retry module.".to_owned(),
    }
}

fn retrieval_provenance() -> RetrievalProvenance {
    RetrievalProvenance {
        recall_used: true,
        locality_class: RecallLocalityClass::HybridLocalAndManaged,
        retrieval_epoch_ref: "retrieval-epoch:stable:0007".to_owned(),
        participating_lanes: vec![
            RetrievalLaneClass::LexicalSearch,
            RetrievalLaneClass::SemanticRecall,
            RetrievalLaneClass::DocsKnowledge,
        ],
        embedder_model_identity_ref: "embedder:hosted-recall:v3".to_owned(),
        source_count: 6,
        chunk_or_anchor_count: 11,
        omitted_candidate_classes: vec![AbsenceStateClass::Omitted, AbsenceStateClass::Blocked],
        raw_vectors_excluded: true,
        raw_chunks_excluded: true,
    }
}

fn absence_rows() -> Vec<AbsenceRow> {
    let specs = [
        (
            "history:large-diff",
            AbsenceStateClass::Omitted,
            "Trimmed for budget; still inspectable.",
        ),
        (
            "external-text:reviewer-notes",
            AbsenceStateClass::Blocked,
            "Held behind a quarantine fence.",
        ),
        (
            "docs:retry-policy",
            AbsenceStateClass::Summarized,
            "Included as a summary, not raw content.",
        ),
        (
            "connector:provider-thread",
            AbsenceStateClass::NotRequested,
            "Never requested for this run.",
        ),
    ];
    specs
        .into_iter()
        .map(|(subject, absence_state, reason)| AbsenceRow {
            subject_ref: subject.to_owned(),
            absence_state,
            reason_label: reason.to_owned(),
            inspect_action_ref: format!("action:evidence.inspect-absence:{subject}"),
        })
        .collect()
}

fn context_inputs() -> ContextInputsBlock {
    ContextInputsBlock {
        context_assembly_ref: "context-assembly:apply:stable:0001".to_owned(),
        context_snapshot_ref: "context-snapshot:apply:stable:0001".to_owned(),
        included_source_count: 6,
        retrieval_provenance: retrieval_provenance(),
        absence_rows: absence_rows(),
    }
}

fn tool_policy() -> ToolPolicyBlock {
    ToolPolicyBlock {
        tool_call_lineage_refs: vec![
            "tool-call:edit:0001".to_owned(),
            "tool-call:test:0002".to_owned(),
        ],
        policy_epoch_ref: "policy-epoch:stable:0004".to_owned(),
        approval_timeline_ref: "approval-timeline:apply:stable:0001".to_owned(),
        decisions: vec![
            ToolPolicyDecisionRow {
                decision_id: "decision:edit-allowed".to_owned(),
                tool_or_policy_label: "Workspace edit tool".to_owned(),
                decision_token: "allowed".to_owned(),
                approval_required: true,
                approval_ref: Some("approval:apply:stable:0001".to_owned()),
            },
            ToolPolicyDecisionRow {
                decision_id: "decision:external-publish-fenced".to_owned(),
                tool_or_policy_label: "External publish".to_owned(),
                decision_token: "fenced".to_owned(),
                approval_required: false,
                approval_ref: None,
            },
        ],
    }
}

fn diff_write_scope() -> DiffWriteScopeBlock {
    DiffWriteScopeBlock {
        patch_review_summary_ref: "patch-review:apply:stable:0001".to_owned(),
        changed_file_count: 2,
        generated_artifact_count: 1,
        write_scope_class_label: "Local reversible edit, two files.".to_owned(),
        produced_artifact_refs: vec!["artifact:patch:0001".to_owned()],
    }
}

fn validation() -> ValidationBlock {
    ValidationBlock {
        validation_summary_refs: vec!["validation:unit-tests:0001".to_owned()],
        validation_outcome_class: FinalizedValidationOutcomeClass::Passed,
        validation_note_label: "Retry unit tests passed after apply.".to_owned(),
    }
}

fn rollback_export() -> RollbackExportBlock {
    RollbackExportBlock {
        rollback_checkpoint_ref: Some("checkpoint:apply:stable:0001".to_owned()),
        mutation_journal_ref: Some("journal:apply:stable:0001".to_owned()),
        json_export_ref: AI_EVIDENCE_PACKET_FINALIZATION_ARTIFACT_REF.to_owned(),
        markdown_summary_ref: AI_EVIDENCE_PACKET_FINALIZATION_SUMMARY_REF.to_owned(),
        export_lineage_refs: vec!["export:operator:0001".to_owned()],
    }
}

fn packet_class_rows() -> Vec<PacketClassRow> {
    EvidencePacketClass::required_coverage()
        .into_iter()
        .map(|packet_class| PacketClassRow {
            packet_class,
            projection_ref: format!("projection:{}", packet_class.as_str()),
            preserves_evidence_id: true,
            preserves_redaction_manifest: true,
            preserves_rollback_export_refs: true,
            preserves_provider_model_identity: true,
            preserves_approval_path: true,
            preserves_validation_receipts: true,
            preserves_write_scope_classes: true,
            ui_parity: true,
            cli_parity: true,
            support_parity: true,
        })
        .collect()
}

fn redaction_manifest() -> Vec<RedactionManifestRow> {
    vec![
        RedactionManifestRow {
            redaction_id: "redaction:secret".to_owned(),
            subject_ref: "context:env-snippet".to_owned(),
            reason_class: RedactionReasonClass::SecretMaterial,
            removed_summary_label: "Removed a credential-bearing config line.".to_owned(),
            reproducibility_impact: ReproducibilityImpactClass::Unchanged,
            reproducibility_note_label: String::new(),
        },
        RedactionManifestRow {
            redaction_id: "redaction:tainted".to_owned(),
            subject_ref: "external-text:reviewer-notes".to_owned(),
            reason_class: RedactionReasonClass::TaintedExternalContent,
            removed_summary_label: "Removed quarantined external text body.".to_owned(),
            reproducibility_impact: ReproducibilityImpactClass::DegradedReproducibility,
            reproducibility_note_label: "Replay omits the tainted body; outcome may differ."
                .to_owned(),
        },
    ]
}

fn retained_inventory() -> Vec<RetainedArtifactRow> {
    vec![
        RetainedArtifactRow {
            artifact_class: RetainedArtifactClass::EvidenceRetainedByPolicy,
            inventory_ref: "inventory:evidence-copy".to_owned(),
            retained_after_thread_deletion: true,
            retained_only_for_evidence_policy: true,
            disclosure_label: "Evidence copy retained because evidence policy requires it."
                .to_owned(),
        },
        RetainedArtifactRow {
            artifact_class: RetainedArtifactClass::ConversationThread,
            inventory_ref: "inventory:thread".to_owned(),
            retained_after_thread_deletion: false,
            retained_only_for_evidence_policy: false,
            disclosure_label: "Conversation thread is removed on thread deletion.".to_owned(),
        },
        RetainedArtifactRow {
            artifact_class: RetainedArtifactClass::PromptResultCache,
            inventory_ref: "inventory:cache".to_owned(),
            retained_after_thread_deletion: false,
            retained_only_for_evidence_policy: false,
            disclosure_label: "Prompt/result cache is cleared on thread deletion.".to_owned(),
        },
        RetainedArtifactRow {
            artifact_class: RetainedArtifactClass::ReusableRepoFact,
            inventory_ref: "inventory:repo-fact".to_owned(),
            retained_after_thread_deletion: true,
            retained_only_for_evidence_policy: false,
            disclosure_label: "Reusable repo fact persists independent of this thread.".to_owned(),
        },
        RetainedArtifactRow {
            artifact_class: RetainedArtifactClass::ExplicitSavedMemory,
            inventory_ref: "inventory:saved-memory".to_owned(),
            retained_after_thread_deletion: true,
            retained_only_for_evidence_policy: false,
            disclosure_label: "Explicit saved memory persists until the operator deletes it."
                .to_owned(),
        },
    ]
}

fn replay_lineage() -> ReplayLineage {
    ReplayLineage {
        run_history_entry_ref: "run-history:apply:stable:0001".to_owned(),
        approval_timeline_ref: "approval-timeline:apply:stable:0001".to_owned(),
        rerun_review_ref: "rerun-review:apply:stable:0001".to_owned(),
        replay_packet_ref: "replay-packet:apply:stable:0001".to_owned(),
        replay_posture: ReplayPostureClass::ReconstructibleFull,
        incompleteness_reason_label: None,
        requires_fresh_approval_for_new_tool_calls: false,
        cites_original_packet_ref: None,
    }
}

fn evidence_branches() -> Vec<EvidenceBranchRow> {
    vec![
        EvidenceBranchRow {
            branch_class: AiEvidenceBranchClass::CandidateTestProposal,
            branch_subject_ref: "candidate-test:retry-backoff".to_owned(),
            validation_status_label: "Proposed; not yet run.".to_owned(),
            outbound_target_class: OutboundTargetClass::StayedLocal,
            outbound_redaction_posture: OutboundRedactionPostureClass::FullDetailRetained,
            scope_label: "Single test in the retry suite.".to_owned(),
            stayed_local: true,
        },
        EvidenceBranchRow {
            branch_class: AiEvidenceBranchClass::AssumptionReview,
            branch_subject_ref: "assumption:idempotent-retry".to_owned(),
            validation_status_label: "Operator confirmed.".to_owned(),
            outbound_target_class: OutboundTargetClass::StayedLocal,
            outbound_redaction_posture: OutboundRedactionPostureClass::FullDetailRetained,
            scope_label: "Retry idempotency assumption.".to_owned(),
            stayed_local: true,
        },
        EvidenceBranchRow {
            branch_class: AiEvidenceBranchClass::SandboxValidation,
            branch_subject_ref: "sandbox:retry-suite".to_owned(),
            validation_status_label: "Passed in sandbox.".to_owned(),
            outbound_target_class: OutboundTargetClass::StayedLocal,
            outbound_redaction_posture: OutboundRedactionPostureClass::FullDetailRetained,
            scope_label: "Sandboxed retry test run.".to_owned(),
            stayed_local: true,
        },
        EvidenceBranchRow {
            branch_class: AiEvidenceBranchClass::AiReviewFinding,
            branch_subject_ref: "finding:unbounded-retry".to_owned(),
            validation_status_label: "Confirmed by reviewer.".to_owned(),
            outbound_target_class: OutboundTargetClass::CopiedToClipboard,
            outbound_redaction_posture: OutboundRedactionPostureClass::MetadataOnly,
            scope_label: "Single review finding.".to_owned(),
            stayed_local: false,
        },
        EvidenceBranchRow {
            branch_class: AiEvidenceBranchClass::PublishPreview,
            branch_subject_ref: "publish-preview:review-thread".to_owned(),
            validation_status_label: "Previewed; not yet published.".to_owned(),
            outbound_target_class: OutboundTargetClass::StayedLocal,
            outbound_redaction_posture: OutboundRedactionPostureClass::RedactedBeforeOutbound,
            scope_label: "Preview of the outbound review comment.".to_owned(),
            stayed_local: true,
        },
        EvidenceBranchRow {
            branch_class: AiEvidenceBranchClass::OutboundReviewAction,
            branch_subject_ref: "outbound:review-thread-comment".to_owned(),
            validation_status_label: "Published to the review thread.".to_owned(),
            outbound_target_class: OutboundTargetClass::PublishedToReviewThread,
            outbound_redaction_posture: OutboundRedactionPostureClass::RedactedBeforeOutbound,
            scope_label: "One published review comment.".to_owned(),
            stayed_local: false,
        },
    ]
}

fn source_contract_refs() -> Vec<String> {
    vec![
        AI_EVIDENCE_PACKET_FINALIZATION_AI_DOC_REF.to_owned(),
        AI_EVIDENCE_PACKET_FINALIZATION_BASE_CONTRACT_REF.to_owned(),
        AI_EVIDENCE_PACKET_FINALIZATION_SCHEMA_REF.to_owned(),
    ]
}

fn input() -> AiEvidencePacketFinalizationInput {
    AiEvidencePacketFinalizationInput {
        packet_id: "ai-evidence-finalization:stable:0001".to_owned(),
        evidence_id: EVIDENCE_ID.to_owned(),
        display_label: "AI evidence packet finalization".to_owned(),
        origin_class: EvidenceOriginClass::OriginatingTurn,
        originating_turn_ref: TURN_REF.to_owned(),
        originating_run_ref: RUN_REF.to_owned(),
        branch_agent_job_ref: None,
        replay_action_ref: None,
        provider_label: "Aureline managed hosted AI".to_owned(),
        model_label: "Hosted apply review".to_owned(),
        approval_path_label: "One-time apply approval".to_owned(),
        route_receipt_ref: "route-receipt:apply:stable:0001".to_owned(),
        spend_receipt_ref: "spend-receipt:apply:stable:0001".to_owned(),
        intent_scope: intent_scope(),
        context_inputs: context_inputs(),
        tool_policy: tool_policy(),
        diff_write_scope: diff_write_scope(),
        validation: validation(),
        rollback_export: rollback_export(),
        packet_class_rows: packet_class_rows(),
        redaction_manifest: redaction_manifest(),
        retained_artifact_inventory: retained_inventory(),
        replay_lineage: replay_lineage(),
        evidence_branches: evidence_branches(),
        source_contract_refs: source_contract_refs(),
        json_export_ref: AI_EVIDENCE_PACKET_FINALIZATION_ARTIFACT_REF.to_owned(),
        markdown_summary_ref: AI_EVIDENCE_PACKET_FINALIZATION_SUMMARY_REF.to_owned(),
        redaction_class_token: "metadata_safe_default".to_owned(),
        minted_at: "2026-05-31T21:00:00Z".to_owned(),
    }
}

fn packet() -> AiEvidencePacketFinalization {
    AiEvidencePacketFinalization::new(input())
}

#[test]
fn finalization_packet_validates() {
    let packet = packet();
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
}

#[test]
fn packet_classes_cover_every_export_class() {
    let mut packet = packet();
    packet
        .packet_class_rows
        .retain(|row| row.packet_class != EvidencePacketClass::ComplianceAuditPacket);

    assert!(packet
        .validate()
        .contains(&AiEvidencePacketFinalizationViolation::PacketClassCoverageMissing));
}

#[test]
fn packet_class_must_preserve_full_lineage() {
    let mut packet = packet();
    packet.packet_class_rows[0].preserves_redaction_manifest = false;

    assert!(packet
        .validate()
        .contains(&AiEvidencePacketFinalizationViolation::PacketClassLineageBroken));
}

#[test]
fn absence_states_must_stay_distinct() {
    let mut packet = packet();
    packet
        .context_inputs
        .absence_rows
        .retain(|row| row.absence_state != AbsenceStateClass::NotRequested);

    assert!(packet
        .validate()
        .contains(&AiEvidencePacketFinalizationViolation::AbsenceStateDistinctionMissing));
}

#[test]
fn redaction_with_impact_must_carry_reproducibility_note() {
    let mut packet = packet();
    let degraded = packet
        .redaction_manifest
        .iter_mut()
        .find(|row| {
            row.reproducibility_impact == ReproducibilityImpactClass::DegradedReproducibility
        })
        .expect("degraded redaction row");
    degraded.reproducibility_note_label = String::new();

    assert!(packet
        .validate()
        .contains(&AiEvidencePacketFinalizationViolation::RedactionManifestIncomplete));
}

#[test]
fn conversation_thread_must_not_survive_thread_deletion() {
    let mut packet = packet();
    let thread = packet
        .retained_artifact_inventory
        .iter_mut()
        .find(|row| row.artifact_class == RetainedArtifactClass::ConversationThread)
        .expect("conversation thread row");
    thread.retained_after_thread_deletion = true;

    assert!(packet
        .validate()
        .contains(&AiEvidencePacketFinalizationViolation::RetainedInventoryIncomplete));
}

#[test]
fn retained_inventory_must_cover_every_artifact_class() {
    let mut packet = packet();
    packet
        .retained_artifact_inventory
        .retain(|row| row.artifact_class != RetainedArtifactClass::ExplicitSavedMemory);

    assert!(packet
        .validate()
        .contains(&AiEvidencePacketFinalizationViolation::RetainedInventoryIncomplete));
}

#[test]
fn semantic_recall_must_exclude_raw_vectors_and_chunks() {
    let mut packet = packet();
    packet
        .context_inputs
        .retrieval_provenance
        .raw_chunks_excluded = false;

    assert!(packet
        .validate()
        .contains(&AiEvidencePacketFinalizationViolation::RetrievalProvenanceIncomplete));
}

#[test]
fn no_recall_must_not_imply_managed_posture() {
    let mut packet = packet();
    let provenance = &mut packet.context_inputs.retrieval_provenance;
    provenance.recall_used = false;
    provenance.participating_lanes.clear();
    // Locality still claims a managed posture even though recall was not used.
    provenance.locality_class = RecallLocalityClass::ManagedCloud;

    assert!(packet
        .validate()
        .contains(&AiEvidencePacketFinalizationViolation::RetrievalProvenanceIncomplete));
}

#[test]
fn incomplete_replay_must_require_fresh_approval() {
    let mut packet = packet();
    packet.replay_lineage.replay_posture = ReplayPostureClass::IncompleteDegradedReplay;
    packet.replay_lineage.incompleteness_reason_label =
        Some("Connector output was not retained.".to_owned());
    packet
        .replay_lineage
        .requires_fresh_approval_for_new_tool_calls = false;

    assert!(packet
        .validate()
        .contains(&AiEvidencePacketFinalizationViolation::ReplayLineageIncomplete));
}

#[test]
fn replay_origin_must_cite_original_without_overwriting() {
    let mut packet = packet();
    packet.origin_class = EvidenceOriginClass::ReplayAction;
    packet.replay_action_ref = Some("replay-action:stable:0002".to_owned());
    // Citing its own id overwrites prior evidence instead of preserving it.
    packet.replay_lineage.cites_original_packet_ref = Some(packet.packet_id.clone());

    assert!(packet
        .validate()
        .contains(&AiEvidencePacketFinalizationViolation::ReplayOverwritesHistory));
}

#[test]
fn replay_origin_without_citation_is_incomplete() {
    let mut packet = packet();
    packet.origin_class = EvidenceOriginClass::RerunAction;
    packet.replay_action_ref = Some("rerun-action:stable:0002".to_owned());
    packet.replay_lineage.cites_original_packet_ref = None;

    assert!(packet
        .validate()
        .contains(&AiEvidencePacketFinalizationViolation::ReplayLineageIncomplete));
}

#[test]
fn branch_agent_origin_requires_job_ref() {
    let mut packet = packet();
    packet.origin_class = EvidenceOriginClass::BranchAgentJob;
    packet.branch_agent_job_ref = None;

    assert!(packet
        .validate()
        .contains(&AiEvidencePacketFinalizationViolation::OriginLineageIncomplete));
}

#[test]
fn evidence_branches_must_cover_every_class() {
    let mut packet = packet();
    packet
        .evidence_branches
        .retain(|row| row.branch_class != AiEvidenceBranchClass::PublishPreview);

    assert!(packet
        .validate()
        .contains(&AiEvidencePacketFinalizationViolation::EvidenceBranchCoverageMissing));
}

#[test]
fn published_branch_must_not_claim_it_stayed_local() {
    let mut packet = packet();
    let outbound = packet
        .evidence_branches
        .iter_mut()
        .find(|row| row.branch_class == AiEvidenceBranchClass::OutboundReviewAction)
        .expect("outbound review action");
    outbound.stayed_local = true;

    assert!(packet
        .validate()
        .contains(&AiEvidencePacketFinalizationViolation::EvidenceBranchOutboundUnclear));
}

#[test]
fn evidence_blocks_must_carry_validation_summaries() {
    let mut packet = packet();
    packet.validation.validation_summary_refs.clear();

    assert!(packet
        .validate()
        .contains(&AiEvidencePacketFinalizationViolation::EvidenceBlockIncomplete));
}

#[test]
fn missing_source_contract_is_rejected() {
    let mut packet = packet();
    packet
        .source_contract_refs
        .retain(|reference| reference != AI_EVIDENCE_PACKET_FINALIZATION_SCHEMA_REF);

    assert!(packet
        .validate()
        .contains(&AiEvidencePacketFinalizationViolation::MissingSourceContracts));
}

#[test]
#[ignore = "run manually to regenerate the checked artifact"]
fn emit_artifact() {
    let packet = packet();
    let root = concat!(env!("CARGO_MANIFEST_DIR"), "/../..");
    let dir = format!("{root}/artifacts/ai/m4/finalize_ai_evidence_packets");
    std::fs::create_dir_all(&dir).unwrap();
    std::fs::write(
        format!("{dir}/support_export.json"),
        format!("{}\n", packet.export_safe_json()),
    )
    .unwrap();
    std::fs::write(
        format!("{dir}/summary.md"),
        packet.render_markdown_summary(),
    )
    .unwrap();
    let fixture_dir = format!("{root}/fixtures/ai/m4/finalize_ai_evidence_packets");
    std::fs::create_dir_all(&fixture_dir).unwrap();
    std::fs::write(
        format!("{fixture_dir}/finalization_packet.json"),
        format!("{}\n", packet.export_safe_json()),
    )
    .unwrap();
}

#[test]
fn checked_artifact_validates() {
    let packet = current_stable_ai_evidence_packet_finalization_export()
        .expect("checked ai evidence packet finalization export validates");
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
}
