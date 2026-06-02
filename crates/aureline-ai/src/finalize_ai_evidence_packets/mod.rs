//! Stable, replay-safe AI evidence packet finalization records.
//!
//! This module turns AI evidence packets into first-class product artifacts.
//! It does not re-derive mutation, run-history, or replay truth: the
//! [`crate::evidence::AiMutationEvidencePacket`] mutation wedge, the
//! [`crate::run_history`] run-history/rerun-review lane, and the replay-packet
//! boundary remain canonical for their own slices. The finalization packet
//! references those lineages by id and binds them into one export-safe packet
//! with a stable evidence id, the six stable evidence blocks (intent and
//! requested scope, context inputs, tool and policy decisions, produced
//! diff/write scope, validation and outcome, and rollback/export), right-sized
//! packet classes, a redaction manifest, a retained-artifact inventory, replay
//! lineage, retrieval-lane provenance, and the AI review-assist branches.
//!
//! The record is export-safe. It carries refs, state tokens, coarse classes,
//! counts, and review labels only. Raw prompt bodies, source file bodies,
//! provider payloads, raw retrieval vectors or chunks, endpoint URLs,
//! credentials, raw token counts, exact prices, and billing-account ids stay
//! outside the support boundary.

use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

/// Stable record-kind tag carried by [`AiEvidencePacketFinalization`].
pub const AI_EVIDENCE_PACKET_FINALIZATION_RECORD_KIND: &str = "ai_evidence_packet_finalization";

/// Schema version for AI evidence packet finalization records.
pub const AI_EVIDENCE_PACKET_FINALIZATION_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the finalization boundary schema.
pub const AI_EVIDENCE_PACKET_FINALIZATION_SCHEMA_REF: &str =
    "schemas/ai/ai_evidence_packet_finalization.schema.json";

/// Repo-relative path of the finalization contract doc.
pub const AI_EVIDENCE_PACKET_FINALIZATION_AI_DOC_REF: &str =
    "docs/ai/m4/finalize_ai_evidence_packets.md";

/// Repo-relative path of the frozen evidence-replayability contract.
pub const AI_EVIDENCE_PACKET_FINALIZATION_BASE_CONTRACT_REF: &str =
    "docs/ai/evidence_replayability_contract.md";

/// Repo-relative path of the protected finalization fixture directory.
pub const AI_EVIDENCE_PACKET_FINALIZATION_FIXTURE_DIR: &str =
    "fixtures/ai/m4/finalize_ai_evidence_packets";

/// Repo-relative path of the checked finalization export.
pub const AI_EVIDENCE_PACKET_FINALIZATION_ARTIFACT_REF: &str =
    "artifacts/ai/m4/finalize_ai_evidence_packets/support_export.json";

/// Repo-relative path of the checked finalization Markdown summary.
pub const AI_EVIDENCE_PACKET_FINALIZATION_SUMMARY_REF: &str =
    "artifacts/ai/m4/finalize_ai_evidence_packets/summary.md";

/// Export class a finalized evidence packet can be right-sized into.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EvidencePacketClass {
    /// Inline evidence stub shown next to an AI result.
    InlineEvidenceStub,
    /// Operator-facing packet for the person who ran the action.
    OperatorPacket,
    /// Support packet for handoff outside the live product.
    SupportPacket,
    /// Compliance/audit packet for retained, attestable review.
    ComplianceAuditPacket,
}

impl EvidencePacketClass {
    /// Stable token used in exports and fixtures.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::InlineEvidenceStub => "inline_evidence_stub",
            Self::OperatorPacket => "operator_packet",
            Self::SupportPacket => "support_packet",
            Self::ComplianceAuditPacket => "compliance_audit_packet",
        }
    }

    /// Packet classes that must be coverable before the lane claims Stable.
    pub const fn required_coverage() -> [Self; 4] {
        [
            Self::InlineEvidenceStub,
            Self::OperatorPacket,
            Self::SupportPacket,
            Self::ComplianceAuditPacket,
        ]
    }
}

/// Where the canonical evidence id traces its lineage back to.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EvidenceOriginClass {
    /// Originating composer turn.
    OriginatingTurn,
    /// Foreground AI run.
    AgentRun,
    /// Background branch-agent job.
    BranchAgentJob,
    /// Replay of an earlier packet.
    ReplayAction,
    /// Rerun derived from an earlier packet.
    RerunAction,
}

impl EvidenceOriginClass {
    /// Stable token used in exports and fixtures.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::OriginatingTurn => "originating_turn",
            Self::AgentRun => "agent_run",
            Self::BranchAgentJob => "branch_agent_job",
            Self::ReplayAction => "replay_action",
            Self::RerunAction => "rerun_action",
        }
    }

    const fn is_replay_or_rerun(self) -> bool {
        matches!(self, Self::ReplayAction | Self::RerunAction)
    }
}

/// Distinct absence buckets that must never collapse into one generic absence.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AbsenceStateClass {
    /// The candidate was omitted (e.g. budget or relevance) but is inspectable.
    Omitted,
    /// The candidate was blocked behind a policy or quarantine fence.
    Blocked,
    /// The candidate was included only as a summary, not raw content.
    Summarized,
    /// The candidate was never requested for this run.
    NotRequested,
}

impl AbsenceStateClass {
    /// Stable token used in exports and fixtures.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Omitted => "omitted",
            Self::Blocked => "blocked",
            Self::Summarized => "summarized",
            Self::NotRequested => "not_requested",
        }
    }

    /// Absence states the packet must keep distinct to claim Stable.
    pub const fn required_coverage() -> [Self; 4] {
        [
            Self::Omitted,
            Self::Blocked,
            Self::Summarized,
            Self::NotRequested,
        ]
    }
}

/// Why content was removed in the export-safe redaction manifest.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RedactionReasonClass {
    /// Secret or credential material.
    SecretMaterial,
    /// Tainted external content held behind a fence.
    TaintedExternalContent,
    /// The content's policy class exceeds the export sink.
    PolicyClassExceedsSink,
    /// The retention window for the content expired.
    RetentionWindowExpired,
    /// The provider never disclosed the content.
    ProviderDidNotDisclose,
    /// The operator or admin requested the redaction.
    UserRequestedRedaction,
}

impl RedactionReasonClass {
    /// Stable token used in exports and fixtures.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SecretMaterial => "secret_material",
            Self::TaintedExternalContent => "tainted_external_content",
            Self::PolicyClassExceedsSink => "policy_class_exceeds_sink",
            Self::RetentionWindowExpired => "retention_window_expired",
            Self::ProviderDidNotDisclose => "provider_did_not_disclose",
            Self::UserRequestedRedaction => "user_requested_redaction",
        }
    }
}

/// Whether a redaction changed reproducibility.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ReproducibilityImpactClass {
    /// Reproducibility is unchanged.
    Unchanged,
    /// Reproducibility is degraded but a partial replay is possible.
    DegradedReproducibility,
    /// Reproducibility is lost; replay is blocked.
    ReplayBlocked,
}

impl ReproducibilityImpactClass {
    /// Stable token used in exports and fixtures.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Unchanged => "unchanged",
            Self::DegradedReproducibility => "degraded_reproducibility",
            Self::ReplayBlocked => "replay_blocked",
        }
    }

    const fn requires_note(self) -> bool {
        !matches!(self, Self::Unchanged)
    }
}

/// Artifact class disclosed in the retained-evidence inventory.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RetainedArtifactClass {
    /// Evidence copy retained only because evidence policy requires it.
    EvidenceRetainedByPolicy,
    /// The conversation thread itself.
    ConversationThread,
    /// A prompt/result cache.
    PromptResultCache,
    /// A reusable repo fact.
    ReusableRepoFact,
    /// Explicit saved memory.
    ExplicitSavedMemory,
}

impl RetainedArtifactClass {
    /// Stable token used in exports and fixtures.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::EvidenceRetainedByPolicy => "evidence_retained_by_policy",
            Self::ConversationThread => "conversation_thread",
            Self::PromptResultCache => "prompt_result_cache",
            Self::ReusableRepoFact => "reusable_repo_fact",
            Self::ExplicitSavedMemory => "explicit_saved_memory",
        }
    }

    /// Artifact classes the inventory must distinguish to claim Stable.
    pub const fn required_coverage() -> [Self; 5] {
        [
            Self::EvidenceRetainedByPolicy,
            Self::ConversationThread,
            Self::PromptResultCache,
            Self::ReusableRepoFact,
            Self::ExplicitSavedMemory,
        ]
    }

    /// Classes that are cleared when the originating thread is deleted.
    const fn cleared_with_thread(self) -> bool {
        matches!(self, Self::ConversationThread | Self::PromptResultCache)
    }
}

/// Replay posture for the finalized packet.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ReplayPostureClass {
    /// The run can be reconstructed faithfully.
    ReconstructibleFull,
    /// Replay is degraded; some evidence was not retained.
    IncompleteDegradedReplay,
    /// Replay is blocked; required evidence is missing.
    IncompleteReplayBlocked,
}

impl ReplayPostureClass {
    /// Stable token used in exports and fixtures.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ReconstructibleFull => "reconstructible_full",
            Self::IncompleteDegradedReplay => "incomplete_degraded_replay",
            Self::IncompleteReplayBlocked => "incomplete_replay_blocked",
        }
    }

    const fn is_incomplete(self) -> bool {
        !matches!(self, Self::ReconstructibleFull)
    }
}

/// Local-versus-managed recall posture preserved across export and replay.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RecallLocalityClass {
    /// Semantic recall ran local to the device only.
    LocalOnly,
    /// Semantic recall used a managed-cloud index.
    ManagedCloud,
    /// Semantic recall combined local and managed lanes.
    HybridLocalAndManaged,
    /// No semantic recall influenced this run.
    NoRecallUsed,
}

impl RecallLocalityClass {
    /// Stable token used in exports and fixtures.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LocalOnly => "local_only",
            Self::ManagedCloud => "managed_cloud",
            Self::HybridLocalAndManaged => "hybrid_local_and_managed",
            Self::NoRecallUsed => "no_recall_used",
        }
    }
}

/// Retrieval lane that participated in context assembly.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RetrievalLaneClass {
    /// Lexical/keyword search.
    LexicalSearch,
    /// Semantic recall over embeddings.
    SemanticRecall,
    /// Graph navigation.
    GraphNavigation,
    /// Docs/knowledge retrieval.
    DocsKnowledge,
    /// History recall.
    HistoryRecall,
}

impl RetrievalLaneClass {
    /// Stable token used in exports and fixtures.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LexicalSearch => "lexical_search",
            Self::SemanticRecall => "semantic_recall",
            Self::GraphNavigation => "graph_navigation",
            Self::DocsKnowledge => "docs_knowledge",
            Self::HistoryRecall => "history_recall",
        }
    }

    const fn is_semantic(self) -> bool {
        matches!(self, Self::SemanticRecall)
    }
}

/// Outbound authority preserved for an AI review-assist branch.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum OutboundTargetClass {
    /// The finding stayed local; nothing left the workspace.
    StayedLocal,
    /// The finding was copied to the clipboard.
    CopiedToClipboard,
    /// The finding was exported to a local file.
    ExportedFile,
    /// The finding was published to a provider thread.
    PublishedToProviderThread,
    /// The finding was published to a check/CI surface.
    PublishedToCheckSurface,
    /// The finding was published to a code-review thread.
    PublishedToReviewThread,
}

impl OutboundTargetClass {
    /// Stable token used in exports and fixtures.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::StayedLocal => "stayed_local",
            Self::CopiedToClipboard => "copied_to_clipboard",
            Self::ExportedFile => "exported_file",
            Self::PublishedToProviderThread => "published_to_provider_thread",
            Self::PublishedToCheckSurface => "published_to_check_surface",
            Self::PublishedToReviewThread => "published_to_review_thread",
        }
    }

    const fn is_published(self) -> bool {
        matches!(
            self,
            Self::PublishedToProviderThread
                | Self::PublishedToCheckSurface
                | Self::PublishedToReviewThread
        )
    }
}

/// Redaction posture applied before an outbound review action.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum OutboundRedactionPostureClass {
    /// Full detail retained in the outbound copy.
    FullDetailRetained,
    /// Only metadata left the workspace.
    MetadataOnly,
    /// Content was redacted before going outbound.
    RedactedBeforeOutbound,
}

impl OutboundRedactionPostureClass {
    /// Stable token used in exports and fixtures.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::FullDetailRetained => "full_detail_retained",
            Self::MetadataOnly => "metadata_only",
            Self::RedactedBeforeOutbound => "redacted_before_outbound",
        }
    }
}

/// AI review-assist branch class exported with the same lineage the GUI shows.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AiEvidenceBranchClass {
    /// AI-proposed candidate test.
    CandidateTestProposal,
    /// AI assumption review.
    AssumptionReview,
    /// Sandbox validation run.
    SandboxValidation,
    /// AI review finding.
    AiReviewFinding,
    /// Publish-preview surface.
    PublishPreview,
    /// Outbound review action.
    OutboundReviewAction,
}

impl AiEvidenceBranchClass {
    /// Stable token used in exports and fixtures.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::CandidateTestProposal => "candidate_test_proposal",
            Self::AssumptionReview => "assumption_review",
            Self::SandboxValidation => "sandbox_validation",
            Self::AiReviewFinding => "ai_review_finding",
            Self::PublishPreview => "publish_preview",
            Self::OutboundReviewAction => "outbound_review_action",
        }
    }

    /// Branch classes the packet must cover to claim Stable.
    pub const fn required_coverage() -> [Self; 6] {
        [
            Self::CandidateTestProposal,
            Self::AssumptionReview,
            Self::SandboxValidation,
            Self::AiReviewFinding,
            Self::PublishPreview,
            Self::OutboundReviewAction,
        ]
    }
}

/// Validation outcome recorded in the validation block.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FinalizedValidationOutcomeClass {
    /// Validation was not run.
    NotRun,
    /// Validation passed cleanly.
    Passed,
    /// Validation passed with warnings.
    PassedWithWarnings,
    /// Validation failed.
    Failed,
    /// Validation could not be reproduced.
    PartialUnreproducible,
}

impl FinalizedValidationOutcomeClass {
    /// Stable token used in exports and fixtures.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NotRun => "not_run",
            Self::Passed => "passed",
            Self::PassedWithWarnings => "passed_with_warnings",
            Self::Failed => "failed",
            Self::PartialUnreproducible => "partial_unreproducible",
        }
    }
}

/// Intent and requested-scope evidence block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct IntentScopeBlock {
    /// Review-safe summary of what the operator asked.
    pub intent_summary_label: String,
    /// Review-safe summary of the requested scope.
    pub requested_scope_label: String,
    /// Coarse intent class token.
    pub intent_class_token: String,
    /// Requested write-scope class label.
    pub requested_write_scope_label: String,
}

/// One distinct-absence row for a context candidate.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AbsenceRow {
    /// Stable object identity of the absent candidate.
    pub subject_ref: String,
    /// Which distinct absence bucket this candidate is in.
    pub absence_state: AbsenceStateClass,
    /// Review-safe explanation of the absence.
    pub reason_label: String,
    /// Inspect action ref so the absence stays inspectable.
    pub inspect_action_ref: String,
}

/// Retrieval-lane provenance for hybrid retrieval or semantic recall.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RetrievalProvenance {
    /// True when semantic recall or hybrid retrieval influenced context.
    pub recall_used: bool,
    /// Local-versus-managed recall posture.
    pub locality_class: RecallLocalityClass,
    /// Retrieval epoch ref.
    pub retrieval_epoch_ref: String,
    /// Participating retrieval lanes.
    pub participating_lanes: Vec<RetrievalLaneClass>,
    /// Embedder/model identity ref backing semantic recall.
    pub embedder_model_identity_ref: String,
    /// Distinct source count that influenced context.
    pub source_count: u32,
    /// Chunk or anchor count surfaced (never raw bodies).
    pub chunk_or_anchor_count: u32,
    /// Candidate classes omitted/blocked/summarized/not-requested by retrieval.
    pub omitted_candidate_classes: Vec<AbsenceStateClass>,
    /// True when raw vectors are excluded from the export.
    pub raw_vectors_excluded: bool,
    /// True when raw code-bearing chunks are excluded from the export.
    pub raw_chunks_excluded: bool,
}

/// Context-inputs evidence block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ContextInputsBlock {
    /// Context assembly ref.
    pub context_assembly_ref: String,
    /// Context snapshot ref.
    pub context_snapshot_ref: String,
    /// Included source count.
    pub included_source_count: u32,
    /// Retrieval-lane provenance.
    pub retrieval_provenance: RetrievalProvenance,
    /// Distinct-absence rows for omitted/blocked/summarized/not-requested.
    pub absence_rows: Vec<AbsenceRow>,
}

/// One tool or policy decision row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ToolPolicyDecisionRow {
    /// Stable decision id.
    pub decision_id: String,
    /// Review-safe tool or policy label.
    pub tool_or_policy_label: String,
    /// Coarse decision token (e.g. allowed/denied/fenced).
    pub decision_token: String,
    /// True when this decision required an approval.
    pub approval_required: bool,
    /// Approval ref when approval was required.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub approval_ref: Option<String>,
}

/// Tool-and-policy-decisions evidence block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ToolPolicyBlock {
    /// Tool-call lineage refs.
    pub tool_call_lineage_refs: Vec<String>,
    /// Policy epoch ref.
    pub policy_epoch_ref: String,
    /// Approval-timeline ref.
    pub approval_timeline_ref: String,
    /// Individual tool/policy decision rows.
    pub decisions: Vec<ToolPolicyDecisionRow>,
}

/// Produced-diff and write-scope evidence block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DiffWriteScopeBlock {
    /// Patch-review summary ref.
    pub patch_review_summary_ref: String,
    /// Changed file count.
    pub changed_file_count: u32,
    /// Generated artifact count.
    pub generated_artifact_count: u32,
    /// Review-safe write-scope class label.
    pub write_scope_class_label: String,
    /// Produced artifact refs.
    pub produced_artifact_refs: Vec<String>,
}

/// Validation-and-outcome evidence block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ValidationBlock {
    /// Validation summary refs (the validation receipts).
    pub validation_summary_refs: Vec<String>,
    /// Validation outcome class.
    pub validation_outcome_class: FinalizedValidationOutcomeClass,
    /// Review-safe validation note.
    pub validation_note_label: String,
}

/// Rollback-and-export evidence block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RollbackExportBlock {
    /// Rollback checkpoint ref when the run mutated state.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rollback_checkpoint_ref: Option<String>,
    /// Mutation journal ref when the run mutated state.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mutation_journal_ref: Option<String>,
    /// JSON export ref.
    pub json_export_ref: String,
    /// Markdown summary ref.
    pub markdown_summary_ref: String,
    /// Export lineage refs (prior exports this one descends from).
    pub export_lineage_refs: Vec<String>,
}

/// One packet-class projection row proving lineage parity across surfaces.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PacketClassRow {
    /// Packet class.
    pub packet_class: EvidencePacketClass,
    /// Projection ref for this class.
    pub projection_ref: String,
    /// True when this projection preserves the canonical evidence id.
    pub preserves_evidence_id: bool,
    /// True when this projection preserves the redaction manifest.
    pub preserves_redaction_manifest: bool,
    /// True when this projection preserves rollback/export refs.
    pub preserves_rollback_export_refs: bool,
    /// True when provider/model identity stays aligned with on-screen review.
    pub preserves_provider_model_identity: bool,
    /// True when the approval path stays aligned with on-screen review.
    pub preserves_approval_path: bool,
    /// True when validation receipts stay aligned with on-screen review.
    pub preserves_validation_receipts: bool,
    /// True when write-scope classes stay aligned with on-screen review.
    pub preserves_write_scope_classes: bool,
    /// True when this class is reachable in the UI path.
    pub ui_parity: bool,
    /// True when this class is reachable in the CLI/headless path.
    pub cli_parity: bool,
    /// True when this class is reachable in the support/export path.
    pub support_parity: bool,
}

impl PacketClassRow {
    fn preserves_full_lineage(&self) -> bool {
        self.preserves_evidence_id
            && self.preserves_redaction_manifest
            && self.preserves_rollback_export_refs
            && self.preserves_provider_model_identity
            && self.preserves_approval_path
            && self.preserves_validation_receipts
            && self.preserves_write_scope_classes
            && self.ui_parity
            && self.cli_parity
            && self.support_parity
    }
}

/// One redaction-manifest row treating redaction as evidence.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RedactionManifestRow {
    /// Stable redaction id.
    pub redaction_id: String,
    /// Stable object identity of what was redacted.
    pub subject_ref: String,
    /// Why the content was removed.
    pub reason_class: RedactionReasonClass,
    /// Review-safe summary of what was removed.
    pub removed_summary_label: String,
    /// Whether reproducibility changed.
    pub reproducibility_impact: ReproducibilityImpactClass,
    /// Review-safe reproducibility note (required when impact is not unchanged).
    pub reproducibility_note_label: String,
}

/// One retained-artifact inventory row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RetainedArtifactRow {
    /// Artifact class.
    pub artifact_class: RetainedArtifactClass,
    /// Inventory ref.
    pub inventory_ref: String,
    /// True when this artifact survives originating-thread deletion.
    pub retained_after_thread_deletion: bool,
    /// True when this copy is retained only because evidence policy requires it.
    pub retained_only_for_evidence_policy: bool,
    /// Review-safe disclosure label.
    pub disclosure_label: String,
}

/// Replay/rerun lineage with incomplete-replay posture.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReplayLineage {
    /// Run-history entry ref.
    pub run_history_entry_ref: String,
    /// Approval-timeline ref.
    pub approval_timeline_ref: String,
    /// Rerun-review sheet ref.
    pub rerun_review_ref: String,
    /// Replay packet ref.
    pub replay_packet_ref: String,
    /// Replay posture.
    pub replay_posture: ReplayPostureClass,
    /// Reason the replay is incomplete (required when posture is incomplete).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub incompleteness_reason_label: Option<String>,
    /// True when new tool calls during replay require fresh approval.
    pub requires_fresh_approval_for_new_tool_calls: bool,
    /// Original packet ref cited by a replay/rerun packet.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cites_original_packet_ref: Option<String>,
}

/// One AI review-assist evidence branch.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EvidenceBranchRow {
    /// Branch class.
    pub branch_class: AiEvidenceBranchClass,
    /// Stable subject ref for this branch.
    pub branch_subject_ref: String,
    /// Review-safe validation status label.
    pub validation_status_label: String,
    /// Outbound authority class (stayed local / copied / published).
    pub outbound_target_class: OutboundTargetClass,
    /// Redaction posture applied before outbound.
    pub outbound_redaction_posture: OutboundRedactionPostureClass,
    /// Review-safe scope label.
    pub scope_label: String,
    /// True when the branch stayed local.
    pub stayed_local: bool,
}

/// Constructor input for [`AiEvidencePacketFinalization::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AiEvidencePacketFinalizationInput {
    /// Stable packet id for this record.
    pub packet_id: String,
    /// Canonical evidence id shared across packet classes and replays.
    pub evidence_id: String,
    /// Display label.
    pub display_label: String,
    /// Lineage origin class.
    pub origin_class: EvidenceOriginClass,
    /// Originating turn ref.
    pub originating_turn_ref: String,
    /// Originating run ref.
    pub originating_run_ref: String,
    /// Branch-agent job ref when the origin is a branch-agent job.
    pub branch_agent_job_ref: Option<String>,
    /// Replay/rerun action ref when the origin is a replay or rerun.
    pub replay_action_ref: Option<String>,
    /// Provider label aligned with on-screen review.
    pub provider_label: String,
    /// Model label aligned with on-screen review.
    pub model_label: String,
    /// Approval-path label aligned with on-screen review.
    pub approval_path_label: String,
    /// Route receipt ref.
    pub route_receipt_ref: String,
    /// Spend receipt ref.
    pub spend_receipt_ref: String,
    /// Intent and requested-scope block.
    pub intent_scope: IntentScopeBlock,
    /// Context-inputs block.
    pub context_inputs: ContextInputsBlock,
    /// Tool-and-policy-decisions block.
    pub tool_policy: ToolPolicyBlock,
    /// Produced-diff and write-scope block.
    pub diff_write_scope: DiffWriteScopeBlock,
    /// Validation-and-outcome block.
    pub validation: ValidationBlock,
    /// Rollback-and-export block.
    pub rollback_export: RollbackExportBlock,
    /// Packet-class projection rows.
    pub packet_class_rows: Vec<PacketClassRow>,
    /// Redaction manifest.
    pub redaction_manifest: Vec<RedactionManifestRow>,
    /// Retained-artifact inventory.
    pub retained_artifact_inventory: Vec<RetainedArtifactRow>,
    /// Replay/rerun lineage.
    pub replay_lineage: ReplayLineage,
    /// AI review-assist evidence branches.
    pub evidence_branches: Vec<EvidenceBranchRow>,
    /// Source contracts consumed by this packet.
    pub source_contract_refs: Vec<String>,
    /// JSON export ref.
    pub json_export_ref: String,
    /// Markdown summary ref.
    pub markdown_summary_ref: String,
    /// Overall packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

/// Export-safe AI evidence packet finalization record.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AiEvidencePacketFinalization {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Stable packet id for this record.
    pub packet_id: String,
    /// Canonical evidence id shared across packet classes and replays.
    pub evidence_id: String,
    /// Display label.
    pub display_label: String,
    /// Lineage origin class.
    pub origin_class: EvidenceOriginClass,
    /// Originating turn ref.
    pub originating_turn_ref: String,
    /// Originating run ref.
    pub originating_run_ref: String,
    /// Branch-agent job ref when the origin is a branch-agent job.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub branch_agent_job_ref: Option<String>,
    /// Replay/rerun action ref when the origin is a replay or rerun.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub replay_action_ref: Option<String>,
    /// Provider label aligned with on-screen review.
    pub provider_label: String,
    /// Model label aligned with on-screen review.
    pub model_label: String,
    /// Approval-path label aligned with on-screen review.
    pub approval_path_label: String,
    /// Route receipt ref.
    pub route_receipt_ref: String,
    /// Spend receipt ref.
    pub spend_receipt_ref: String,
    /// Intent and requested-scope block.
    pub intent_scope: IntentScopeBlock,
    /// Context-inputs block.
    pub context_inputs: ContextInputsBlock,
    /// Tool-and-policy-decisions block.
    pub tool_policy: ToolPolicyBlock,
    /// Produced-diff and write-scope block.
    pub diff_write_scope: DiffWriteScopeBlock,
    /// Validation-and-outcome block.
    pub validation: ValidationBlock,
    /// Rollback-and-export block.
    pub rollback_export: RollbackExportBlock,
    /// Packet-class projection rows.
    pub packet_class_rows: Vec<PacketClassRow>,
    /// Redaction manifest treating redaction as evidence.
    pub redaction_manifest: Vec<RedactionManifestRow>,
    /// Retained-artifact inventory distinct from threads, caches, and memory.
    pub retained_artifact_inventory: Vec<RetainedArtifactRow>,
    /// Replay/rerun lineage with incomplete-replay posture.
    pub replay_lineage: ReplayLineage,
    /// AI review-assist evidence branches.
    pub evidence_branches: Vec<EvidenceBranchRow>,
    /// Source contracts consumed by this packet.
    pub source_contract_refs: Vec<String>,
    /// JSON export ref.
    pub json_export_ref: String,
    /// Markdown summary ref.
    pub markdown_summary_ref: String,
    /// Overall packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

impl AiEvidencePacketFinalization {
    /// Builds a finalization packet from the stable-lane input.
    pub fn new(input: AiEvidencePacketFinalizationInput) -> Self {
        Self {
            record_kind: AI_EVIDENCE_PACKET_FINALIZATION_RECORD_KIND.to_owned(),
            schema_version: AI_EVIDENCE_PACKET_FINALIZATION_SCHEMA_VERSION,
            packet_id: input.packet_id,
            evidence_id: input.evidence_id,
            display_label: input.display_label,
            origin_class: input.origin_class,
            originating_turn_ref: input.originating_turn_ref,
            originating_run_ref: input.originating_run_ref,
            branch_agent_job_ref: input.branch_agent_job_ref,
            replay_action_ref: input.replay_action_ref,
            provider_label: input.provider_label,
            model_label: input.model_label,
            approval_path_label: input.approval_path_label,
            route_receipt_ref: input.route_receipt_ref,
            spend_receipt_ref: input.spend_receipt_ref,
            intent_scope: input.intent_scope,
            context_inputs: input.context_inputs,
            tool_policy: input.tool_policy,
            diff_write_scope: input.diff_write_scope,
            validation: input.validation,
            rollback_export: input.rollback_export,
            packet_class_rows: input.packet_class_rows,
            redaction_manifest: input.redaction_manifest,
            retained_artifact_inventory: input.retained_artifact_inventory,
            replay_lineage: input.replay_lineage,
            evidence_branches: input.evidence_branches,
            source_contract_refs: input.source_contract_refs,
            json_export_ref: input.json_export_ref,
            markdown_summary_ref: input.markdown_summary_ref,
            redaction_class_token: input.redaction_class_token,
            minted_at: input.minted_at,
        }
    }

    /// Validates the finalization packet's stable-line invariants.
    pub fn validate(&self) -> Vec<AiEvidencePacketFinalizationViolation> {
        let mut violations = Vec::new();
        if self.record_kind != AI_EVIDENCE_PACKET_FINALIZATION_RECORD_KIND {
            violations.push(AiEvidencePacketFinalizationViolation::WrongRecordKind);
        }
        if self.schema_version != AI_EVIDENCE_PACKET_FINALIZATION_SCHEMA_VERSION {
            violations.push(AiEvidencePacketFinalizationViolation::WrongSchemaVersion);
        }
        if self.packet_id.trim().is_empty()
            || self.evidence_id.trim().is_empty()
            || self.display_label.trim().is_empty()
            || self.provider_label.trim().is_empty()
            || self.model_label.trim().is_empty()
            || self.approval_path_label.trim().is_empty()
            || self.route_receipt_ref.trim().is_empty()
            || self.spend_receipt_ref.trim().is_empty()
            || self.redaction_class_token.trim().is_empty()
            || self.minted_at.trim().is_empty()
        {
            violations.push(AiEvidencePacketFinalizationViolation::MissingIdentity);
        }
        validate_origin_lineage(self, &mut violations);
        validate_source_contracts(self, &mut violations);
        validate_evidence_blocks(self, &mut violations);
        validate_retrieval_provenance(self, &mut violations);
        validate_absence_distinction(self, &mut violations);
        validate_packet_classes(self, &mut violations);
        validate_redaction_manifest(self, &mut violations);
        validate_retained_inventory(self, &mut violations);
        validate_replay_lineage(self, &mut violations);
        validate_evidence_branches(self, &mut violations);
        if self.json_export_ref.trim().is_empty() || self.markdown_summary_ref.trim().is_empty() {
            violations.push(AiEvidencePacketFinalizationViolation::ExportRefsMissing);
        }
        if json_contains_forbidden_boundary_material(
            &serde_json::to_value(self).expect("ai evidence packet finalization serializes"),
        ) {
            violations.push(AiEvidencePacketFinalizationViolation::RawBoundaryMaterialInExport);
        }
        violations
    }

    /// Deterministic export-safe JSON.
    ///
    /// # Panics
    ///
    /// Panics only if serializing this metadata-only packet fails.
    pub fn export_safe_json(&self) -> String {
        serde_json::to_string_pretty(self).expect("ai evidence packet finalization serializes")
    }

    /// Deterministic Markdown summary for support, docs, or review handoff.
    pub fn render_markdown_summary(&self) -> String {
        let mut out = String::new();
        out.push_str("# AI Evidence Packet Finalization\n\n");
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- Evidence id: `{}`\n", self.evidence_id));
        out.push_str(&format!(
            "- Origin: `{}` (turn `{}` / run `{}`)\n",
            self.origin_class.as_str(),
            self.originating_turn_ref,
            self.originating_run_ref
        ));
        out.push_str(&format!(
            "- Route: `{} / {}` via `{}`\n",
            self.provider_label, self.model_label, self.approval_path_label
        ));
        out.push_str(&format!(
            "- Packet classes / redaction rows / retained-artifact rows: {} / {} / {}\n",
            self.packet_class_rows.len(),
            self.redaction_manifest.len(),
            self.retained_artifact_inventory.len()
        ));
        out.push_str(&format!(
            "- Retrieval recall: {} (locality `{}`, {} lanes)\n",
            self.context_inputs.retrieval_provenance.recall_used,
            self.context_inputs
                .retrieval_provenance
                .locality_class
                .as_str(),
            self.context_inputs
                .retrieval_provenance
                .participating_lanes
                .len()
        ));
        out.push_str(&format!(
            "- Replay posture: `{}` (fresh approval for new tools: {})\n",
            self.replay_lineage.replay_posture.as_str(),
            self.replay_lineage
                .requires_fresh_approval_for_new_tool_calls
        ));
        out.push_str(&format!(
            "- Validation outcome: `{}`\n",
            self.validation.validation_outcome_class.as_str()
        ));
        out.push_str(&format!(
            "- Review-assist branches: {}\n",
            self.evidence_branches.len()
        ));
        out
    }
}

/// Errors emitted when reading the checked-in finalization export.
#[derive(Debug)]
pub enum AiEvidencePacketFinalizationArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<AiEvidencePacketFinalizationViolation>),
}

impl fmt::Display for AiEvidencePacketFinalizationArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(
                    formatter,
                    "ai evidence packet finalization export parse failed: {error}"
                )
            }
            Self::Validation(violations) => {
                let tokens = violations
                    .iter()
                    .map(|violation| violation.as_str())
                    .collect::<Vec<_>>()
                    .join(",");
                write!(
                    formatter,
                    "ai evidence packet finalization export failed validation: {tokens}"
                )
            }
        }
    }
}

impl Error for AiEvidencePacketFinalizationArtifactError {}

/// Validation failures emitted by [`AiEvidencePacketFinalization::validate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AiEvidencePacketFinalizationViolation {
    /// Packet record kind is wrong.
    WrongRecordKind,
    /// Packet schema version is wrong.
    WrongSchemaVersion,
    /// Required identity field is missing.
    MissingIdentity,
    /// Origin lineage refs do not match the declared origin class.
    OriginLineageIncomplete,
    /// Source contract refs are incomplete.
    MissingSourceContracts,
    /// A stable evidence block is incomplete.
    EvidenceBlockIncomplete,
    /// Retrieval-lane provenance is incomplete or implies a broader posture.
    RetrievalProvenanceIncomplete,
    /// Omitted/blocked/summarized/not-requested states are not kept distinct.
    AbsenceStateDistinctionMissing,
    /// A packet class is not covered.
    PacketClassCoverageMissing,
    /// A packet class projection does not preserve full lineage parity.
    PacketClassLineageBroken,
    /// A redaction-manifest row is incomplete.
    RedactionManifestIncomplete,
    /// The retained-artifact inventory does not disclose retention truth.
    RetainedInventoryIncomplete,
    /// Replay lineage is incomplete for an incomplete or replay-origin packet.
    ReplayLineageIncomplete,
    /// A replay/rerun packet overwrites prior evidence instead of citing it.
    ReplayOverwritesHistory,
    /// An AI review-assist branch class is not covered.
    EvidenceBranchCoverageMissing,
    /// An AI review-assist branch hides its outbound authority.
    EvidenceBranchOutboundUnclear,
    /// Export refs are missing.
    ExportRefsMissing,
    /// Export contains raw boundary material.
    RawBoundaryMaterialInExport,
}

impl AiEvidencePacketFinalizationViolation {
    /// Stable token used in tests and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WrongRecordKind => "wrong_record_kind",
            Self::WrongSchemaVersion => "wrong_schema_version",
            Self::MissingIdentity => "missing_identity",
            Self::OriginLineageIncomplete => "origin_lineage_incomplete",
            Self::MissingSourceContracts => "missing_source_contracts",
            Self::EvidenceBlockIncomplete => "evidence_block_incomplete",
            Self::RetrievalProvenanceIncomplete => "retrieval_provenance_incomplete",
            Self::AbsenceStateDistinctionMissing => "absence_state_distinction_missing",
            Self::PacketClassCoverageMissing => "packet_class_coverage_missing",
            Self::PacketClassLineageBroken => "packet_class_lineage_broken",
            Self::RedactionManifestIncomplete => "redaction_manifest_incomplete",
            Self::RetainedInventoryIncomplete => "retained_inventory_incomplete",
            Self::ReplayLineageIncomplete => "replay_lineage_incomplete",
            Self::ReplayOverwritesHistory => "replay_overwrites_history",
            Self::EvidenceBranchCoverageMissing => "evidence_branch_coverage_missing",
            Self::EvidenceBranchOutboundUnclear => "evidence_branch_outbound_unclear",
            Self::ExportRefsMissing => "export_refs_missing",
            Self::RawBoundaryMaterialInExport => "raw_boundary_material_in_export",
        }
    }
}

/// Returns the checked-in AI evidence packet finalization export.
///
/// # Errors
///
/// Returns an artifact error if the checked-in export does not parse or validate.
pub fn current_stable_ai_evidence_packet_finalization_export(
) -> Result<AiEvidencePacketFinalization, AiEvidencePacketFinalizationArtifactError> {
    let packet: AiEvidencePacketFinalization = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/ai/m4/finalize_ai_evidence_packets/support_export.json"
    )))
    .map_err(AiEvidencePacketFinalizationArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(AiEvidencePacketFinalizationArtifactError::Validation(
            violations,
        ))
    }
}

fn validate_origin_lineage(
    packet: &AiEvidencePacketFinalization,
    violations: &mut Vec<AiEvidencePacketFinalizationViolation>,
) {
    let mut broken = packet.originating_turn_ref.trim().is_empty()
        || packet.originating_run_ref.trim().is_empty();
    if packet.origin_class == EvidenceOriginClass::BranchAgentJob
        && !packet
            .branch_agent_job_ref
            .as_deref()
            .is_some_and(|reference| !reference.trim().is_empty())
    {
        broken = true;
    }
    if packet.origin_class.is_replay_or_rerun()
        && !packet
            .replay_action_ref
            .as_deref()
            .is_some_and(|reference| !reference.trim().is_empty())
    {
        broken = true;
    }
    if broken {
        violations.push(AiEvidencePacketFinalizationViolation::OriginLineageIncomplete);
    }
}

fn validate_source_contracts(
    packet: &AiEvidencePacketFinalization,
    violations: &mut Vec<AiEvidencePacketFinalizationViolation>,
) {
    for required in [
        AI_EVIDENCE_PACKET_FINALIZATION_AI_DOC_REF,
        AI_EVIDENCE_PACKET_FINALIZATION_BASE_CONTRACT_REF,
        AI_EVIDENCE_PACKET_FINALIZATION_SCHEMA_REF,
    ] {
        if !packet
            .source_contract_refs
            .iter()
            .any(|reference| reference == required)
        {
            violations.push(AiEvidencePacketFinalizationViolation::MissingSourceContracts);
            break;
        }
    }
}

fn validate_evidence_blocks(
    packet: &AiEvidencePacketFinalization,
    violations: &mut Vec<AiEvidencePacketFinalizationViolation>,
) {
    let intent = &packet.intent_scope;
    let context = &packet.context_inputs;
    let tool = &packet.tool_policy;
    let diff = &packet.diff_write_scope;
    let validation = &packet.validation;
    let rollback = &packet.rollback_export;

    let mut incomplete = intent.intent_summary_label.trim().is_empty()
        || intent.requested_scope_label.trim().is_empty()
        || intent.intent_class_token.trim().is_empty()
        || intent.requested_write_scope_label.trim().is_empty()
        || context.context_assembly_ref.trim().is_empty()
        || context.context_snapshot_ref.trim().is_empty()
        || tool.policy_epoch_ref.trim().is_empty()
        || tool.approval_timeline_ref.trim().is_empty()
        || diff.patch_review_summary_ref.trim().is_empty()
        || diff.write_scope_class_label.trim().is_empty()
        || validation.validation_summary_refs.is_empty()
        || validation.validation_note_label.trim().is_empty()
        || rollback.json_export_ref.trim().is_empty()
        || rollback.markdown_summary_ref.trim().is_empty();

    for decision in &tool.decisions {
        let approval_present = decision
            .approval_ref
            .as_deref()
            .is_some_and(|reference| !reference.trim().is_empty());
        if decision.decision_id.trim().is_empty()
            || decision.tool_or_policy_label.trim().is_empty()
            || decision.decision_token.trim().is_empty()
            || (decision.approval_required && !approval_present)
        {
            incomplete = true;
            break;
        }
    }

    if incomplete {
        violations.push(AiEvidencePacketFinalizationViolation::EvidenceBlockIncomplete);
    }
}

fn validate_retrieval_provenance(
    packet: &AiEvidencePacketFinalization,
    violations: &mut Vec<AiEvidencePacketFinalizationViolation>,
) {
    let provenance = &packet.context_inputs.retrieval_provenance;
    if !provenance.recall_used {
        // A run that used no semantic recall must not imply a managed posture or
        // claim participating lanes.
        if provenance.locality_class != RecallLocalityClass::NoRecallUsed
            || !provenance.participating_lanes.is_empty()
        {
            violations.push(AiEvidencePacketFinalizationViolation::RetrievalProvenanceIncomplete);
        }
        return;
    }
    if provenance.locality_class == RecallLocalityClass::NoRecallUsed
        || provenance.participating_lanes.is_empty()
        || provenance.retrieval_epoch_ref.trim().is_empty()
    {
        violations.push(AiEvidencePacketFinalizationViolation::RetrievalProvenanceIncomplete);
        return;
    }
    // Semantic recall must keep epoch and embedder identity and must always omit
    // raw vectors and raw code-bearing chunks by default.
    let semantic = provenance
        .participating_lanes
        .iter()
        .any(|lane| lane.is_semantic());
    if semantic
        && (provenance.embedder_model_identity_ref.trim().is_empty()
            || !provenance.raw_vectors_excluded
            || !provenance.raw_chunks_excluded)
    {
        violations.push(AiEvidencePacketFinalizationViolation::RetrievalProvenanceIncomplete);
    }
}

fn validate_absence_distinction(
    packet: &AiEvidencePacketFinalization,
    violations: &mut Vec<AiEvidencePacketFinalizationViolation>,
) {
    for row in &packet.context_inputs.absence_rows {
        if row.subject_ref.trim().is_empty()
            || row.reason_label.trim().is_empty()
            || row.inspect_action_ref.trim().is_empty()
        {
            violations.push(AiEvidencePacketFinalizationViolation::AbsenceStateDistinctionMissing);
            return;
        }
    }
    for required in AbsenceStateClass::required_coverage() {
        if !packet
            .context_inputs
            .absence_rows
            .iter()
            .any(|row| row.absence_state == required)
        {
            violations.push(AiEvidencePacketFinalizationViolation::AbsenceStateDistinctionMissing);
            return;
        }
    }
}

fn validate_packet_classes(
    packet: &AiEvidencePacketFinalization,
    violations: &mut Vec<AiEvidencePacketFinalizationViolation>,
) {
    for required in EvidencePacketClass::required_coverage() {
        match packet
            .packet_class_rows
            .iter()
            .find(|row| row.packet_class == required)
        {
            Some(_) => {}
            None => {
                violations.push(AiEvidencePacketFinalizationViolation::PacketClassCoverageMissing);
                return;
            }
        }
    }
    for row in &packet.packet_class_rows {
        if row.projection_ref.trim().is_empty() || !row.preserves_full_lineage() {
            violations.push(AiEvidencePacketFinalizationViolation::PacketClassLineageBroken);
            return;
        }
    }
}

fn validate_redaction_manifest(
    packet: &AiEvidencePacketFinalization,
    violations: &mut Vec<AiEvidencePacketFinalizationViolation>,
) {
    for row in &packet.redaction_manifest {
        let note_present = !row.reproducibility_note_label.trim().is_empty();
        if row.redaction_id.trim().is_empty()
            || row.subject_ref.trim().is_empty()
            || row.removed_summary_label.trim().is_empty()
            || (row.reproducibility_impact.requires_note() && !note_present)
        {
            violations.push(AiEvidencePacketFinalizationViolation::RedactionManifestIncomplete);
            return;
        }
    }
}

fn validate_retained_inventory(
    packet: &AiEvidencePacketFinalization,
    violations: &mut Vec<AiEvidencePacketFinalizationViolation>,
) {
    for row in &packet.retained_artifact_inventory {
        let mut broken =
            row.inventory_ref.trim().is_empty() || row.disclosure_label.trim().is_empty();
        // A conversation thread or prompt/result cache must not be disclosed as
        // surviving thread deletion.
        if row.artifact_class.cleared_with_thread() && row.retained_after_thread_deletion {
            broken = true;
        }
        // Only the policy-retained evidence class may claim retention purely for
        // evidence policy.
        if row.retained_only_for_evidence_policy
            && row.artifact_class != RetainedArtifactClass::EvidenceRetainedByPolicy
        {
            broken = true;
        }
        // Evidence retained by policy must survive thread deletion and disclose
        // it is held only for evidence policy.
        if row.artifact_class == RetainedArtifactClass::EvidenceRetainedByPolicy
            && (!row.retained_after_thread_deletion || !row.retained_only_for_evidence_policy)
        {
            broken = true;
        }
        if broken {
            violations.push(AiEvidencePacketFinalizationViolation::RetainedInventoryIncomplete);
            return;
        }
    }
    for required in RetainedArtifactClass::required_coverage() {
        if !packet
            .retained_artifact_inventory
            .iter()
            .any(|row| row.artifact_class == required)
        {
            violations.push(AiEvidencePacketFinalizationViolation::RetainedInventoryIncomplete);
            return;
        }
    }
}

fn validate_replay_lineage(
    packet: &AiEvidencePacketFinalization,
    violations: &mut Vec<AiEvidencePacketFinalizationViolation>,
) {
    let lineage = &packet.replay_lineage;
    if lineage.run_history_entry_ref.trim().is_empty()
        || lineage.approval_timeline_ref.trim().is_empty()
        || lineage.rerun_review_ref.trim().is_empty()
        || lineage.replay_packet_ref.trim().is_empty()
    {
        violations.push(AiEvidencePacketFinalizationViolation::ReplayLineageIncomplete);
        return;
    }
    // An incomplete replay must say so plainly and require fresh approval for any
    // new tool calls rather than implying a faithful rerun.
    if lineage.replay_posture.is_incomplete() {
        let reason_present = lineage
            .incompleteness_reason_label
            .as_deref()
            .is_some_and(|reason| !reason.trim().is_empty());
        if !reason_present || !lineage.requires_fresh_approval_for_new_tool_calls {
            violations.push(AiEvidencePacketFinalizationViolation::ReplayLineageIncomplete);
            return;
        }
    }
    // A replay/rerun packet must cite the original packet and must not overwrite
    // it with its own id.
    if packet.origin_class.is_replay_or_rerun() {
        match lineage.cites_original_packet_ref.as_deref() {
            Some(original) if !original.trim().is_empty() => {
                if original == packet.packet_id || original == packet.evidence_id {
                    violations.push(AiEvidencePacketFinalizationViolation::ReplayOverwritesHistory);
                }
            }
            _ => {
                violations.push(AiEvidencePacketFinalizationViolation::ReplayLineageIncomplete);
            }
        }
    }
}

fn validate_evidence_branches(
    packet: &AiEvidencePacketFinalization,
    violations: &mut Vec<AiEvidencePacketFinalizationViolation>,
) {
    for required in AiEvidenceBranchClass::required_coverage() {
        if !packet
            .evidence_branches
            .iter()
            .any(|row| row.branch_class == required)
        {
            violations.push(AiEvidencePacketFinalizationViolation::EvidenceBranchCoverageMissing);
            return;
        }
    }
    for row in &packet.evidence_branches {
        if row.branch_subject_ref.trim().is_empty()
            || row.validation_status_label.trim().is_empty()
            || row.scope_label.trim().is_empty()
        {
            violations.push(AiEvidencePacketFinalizationViolation::EvidenceBranchOutboundUnclear);
            return;
        }
        let published = row.outbound_target_class.is_published();
        // A published finding cannot also claim it stayed local; a local finding
        // must report the stayed-local target.
        if published && row.stayed_local {
            violations.push(AiEvidencePacketFinalizationViolation::EvidenceBranchOutboundUnclear);
            return;
        }
        if row.outbound_target_class == OutboundTargetClass::StayedLocal && !row.stayed_local {
            violations.push(AiEvidencePacketFinalizationViolation::EvidenceBranchOutboundUnclear);
            return;
        }
    }
}

fn json_contains_forbidden_boundary_material(value: &serde_json::Value) -> bool {
    match value {
        serde_json::Value::String(text) => contains_forbidden_boundary_material(text),
        serde_json::Value::Array(values) => {
            values.iter().any(json_contains_forbidden_boundary_material)
        }
        serde_json::Value::Object(values) => values
            .values()
            .any(json_contains_forbidden_boundary_material),
        _ => false,
    }
}

fn contains_forbidden_boundary_material(value: &str) -> bool {
    let lower = value.to_ascii_lowercase();
    lower.contains("://")
        || lower.contains("api_key")
        || lower.contains("api-key")
        || lower.contains("oauth_token")
        || lower.contains("bearer ")
        || lower.contains("billing-account")
        || lower.contains("raw_prompt")
        || lower.contains("/users/")
}

#[cfg(test)]
mod tests;
