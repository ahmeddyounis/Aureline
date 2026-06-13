//! Prompt-composer draft and session-artifact records, attachment-and-mention
//! provenance, context add-or-remove receipts, and durable replay-safe evidence
//! packets across M5 AI surfaces.
//!
//! This module turns a prompt-composer draft into a durable-but-scoped session
//! artifact whose unit of truth is a [`SessionArtifactRow`]: a single composer
//! draft or session — inline assist, review, or docs/browser — binding the
//! attachment-and-mention provenance that explains where its context came from,
//! the context add-or-remove receipts that record what was added, omitted,
//! removed, or policy-filtered, the durable scope/locality/retention/delete
//! posture that keeps the artifact inspectable without drifting into hidden
//! memory, and the replay-safe evidence lineage that lets users, support, and
//! compliance inspect the turn without raw prompt text. The packet is the
//! canonical session-artifact source for shell, docs, support export, and
//! release tooling; consumers project it instead of re-deriving draft, mention,
//! context-change, or replay state by hand.
//!
//! The packet refuses to present a session artifact greener than its provenance
//! can back. Every attachment carries an origin, source class, and trust
//! posture; every mention resolves to a precise state and a resolved mention
//! names its target while a scope-excluded mention is never silently in scope;
//! every context change carries a replay-visible receipt and a precise reason
//! rather than an unspecified catch-all when a more precise reason exists; and
//! every artifact declares a scope-safe locality, a real delete/export posture
//! when it retains anything durable, and a replay-safe evidence packet that
//! never demands raw prompt text. A draft must stay scoped to its session rather
//! than drifting into durable memory, a workspace artifact never sits in a
//! tenant-wide store and an org artifact stays tenant-pinned so recall never
//! crosses a workspace or tenant boundary by default, and a claimed artifact
//! carries route, spend, operator, support, and compliance evidence refs plus a
//! verified rollback path where one exists. Every artifact carries a closed set
//! of downgrade rules — including the proof-stale and trust-narrowing triggers —
//! that narrow the claim instead of hiding it, reusing the attachment
//! semantic-role and provenance vocabularies frozen by the richer prompt
//! composer, the mention and source vocabularies frozen by the composer, the
//! scope, locality, retention, and delete vocabularies frozen by the AI memory
//! class lane, and the qualification, surface, downgrade-trigger, and
//! rollback-posture vocabularies frozen by the M5 AI workflow matrix, so no
//! artifact row may stay greener than its evidence.
//!
//! Raw prompt bodies, source file bodies, provider payloads, endpoint URLs,
//! credentials, raw token counts, and exact spend amounts stay outside the
//! support boundary; the packet carries refs, coarse classes, state tokens, and
//! review-safe labels only.
//!
//! The boundary schema is
//! [`schemas/ai/ship-prompt-composer-draft-and-session-artifact-records-attachment-and-mention-provenance-context-add-or-remove-receipts.schema.json`](../../../../schemas/ai/ship-prompt-composer-draft-and-session-artifact-records-attachment-and-mention-provenance-context-add-or-remove-receipts.schema.json).
//! The contract doc is
//! [`docs/ai/m5/ship_prompt_composer_draft_and_session_artifact_records_attachment_and_mention_provenance_context_add_or_remove_receipts.md`](../../../../docs/ai/m5/ship_prompt_composer_draft_and_session_artifact_records_attachment_and_mention_provenance_context_add_or_remove_receipts.md).
//! The protected fixture directory is
//! [`fixtures/ai/m5/ship_prompt_composer_draft_and_session_artifact_records_attachment_and_mention_provenance_context_add_or_remove_receipts/`](../../../../fixtures/ai/m5/ship_prompt_composer_draft_and_session_artifact_records_attachment_and_mention_provenance_context_add_or_remove_receipts/).

#[cfg(test)]
mod tests;

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

use crate::composer::{MentionKind, MentionResolutionState, SourceClass, TrustPosture};
use crate::context_inspector::{ContextItemStateClass, ContextOmissionReasonClass};
use crate::freeze_the_m5_ai_workflow_matrix_for_inline_assist_patch_review_and_branch_or_worktree_agents::{
    M5AiWorkflowConsumerSurface, M5AiWorkflowDowngradeTrigger, M5AiWorkflowQualificationClass,
    M5AiWorkflowRollbackPosture, M5_AI_WORKFLOW_MATRIX_SCHEMA_REF,
};
use crate::implement_a_richer_prompt_composer_with_intent_modes_typed_attachments_context_pinning_and_omitted_context_tru::{
    AttachmentProvenanceClass, AttachmentSemanticRoleClass, RICHER_PROMPT_COMPOSER_SCHEMA_REF,
};
use crate::implement_spend_and_route_receipts_budget_band_preflights_cumulative_branch_agent_ceilings_and_checkpoint_aware_cancella::AI_RUN_RECEIPT_SCHEMA_REF;
use crate::implement_turn_thread_workspace_or_org_memory_classes_prompt_result_cache_objects_and_deletion_export_retention_truth::{
    MemoryDeleteExportPosture, MemoryLocalityClass, MemoryRetentionClass, MemoryScopeClass,
    MEMORY_CLASS_MATERIALIZATION_SCHEMA_REF,
};

/// Stable record-kind tag carried by [`PromptSessionArtifactPacket`].
pub const SESSION_ARTIFACT_RECORD_KIND: &str =
    "ship_prompt_composer_draft_and_session_artifact_records_attachment_and_mention_provenance_context_add_or_remove_receipts";

/// Schema version for prompt session-artifact records.
pub const SESSION_ARTIFACT_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the boundary schema.
pub const SESSION_ARTIFACT_SCHEMA_REF: &str =
    "schemas/ai/ship-prompt-composer-draft-and-session-artifact-records-attachment-and-mention-provenance-context-add-or-remove-receipts.schema.json";

/// Repo-relative path of the session-artifact contract doc.
pub const SESSION_ARTIFACT_DOC_REF: &str =
    "docs/ai/m5/ship_prompt_composer_draft_and_session_artifact_records_attachment_and_mention_provenance_context_add_or_remove_receipts.md";

/// Repo-relative path of the protected fixture directory.
pub const SESSION_ARTIFACT_FIXTURE_DIR: &str =
    "fixtures/ai/m5/ship_prompt_composer_draft_and_session_artifact_records_attachment_and_mention_provenance_context_add_or_remove_receipts";

/// Repo-relative path of the checked support-export artifact.
pub const SESSION_ARTIFACT_ARTIFACT_REF: &str =
    "artifacts/ai/m5/ship_prompt_composer_draft_and_session_artifact_records_attachment_and_mention_provenance_context_add_or_remove_receipts/support_export.json";

/// Repo-relative path of the checked Markdown summary.
pub const SESSION_ARTIFACT_SUMMARY_REF: &str =
    "artifacts/ai/m5/ship_prompt_composer_draft_and_session_artifact_records_attachment_and_mention_provenance_context_add_or_remove_receipts.md";

/// Lifecycle class fixing how a prompt-composer draft persists as a session
/// artifact.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SessionArtifactClass {
    /// An in-progress composer draft, scoped to the current session.
    DraftInProgress,
    /// A durable-but-scoped session artifact promoted from a sent turn.
    ActiveSessionArtifact,
    /// A retained session artifact that is no longer the active draft.
    ArchivedSessionArtifact,
    /// A deletion record left behind after the artifact's body was deleted.
    DeletedTombstone,
}

impl SessionArtifactClass {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::DraftInProgress => "draft_in_progress",
            Self::ActiveSessionArtifact => "active_session_artifact",
            Self::ArchivedSessionArtifact => "archived_session_artifact",
            Self::DeletedTombstone => "deleted_tombstone",
        }
    }

    /// Whether this class is an in-progress draft that must stay session-scoped.
    pub const fn is_draft(self) -> bool {
        matches!(self, Self::DraftInProgress)
    }

    /// Whether this class is a deletion record carrying nothing durable to delete.
    pub const fn is_tombstone(self) -> bool {
        matches!(self, Self::DeletedTombstone)
    }
}

/// How a single context source entered, left, or stayed out of the composed turn.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ContextChangeKindClass {
    /// The user explicitly added the source.
    AddedByUser,
    /// The source trailed a typed mention the user placed.
    AddedByMentionTrail,
    /// Retrieval contributed the source into the context.
    AddedByRetrieval,
    /// A previously omitted source was restored into the context.
    RestoredAfterOmission,
    /// The user explicitly removed the source.
    RemovedByUser,
    /// The source was omitted because the context budget overflowed.
    OmittedUnderBudget,
    /// The source was removed because it went stale since it was added.
    RemovedAsStale,
    /// Policy, trust, or scope filtered the source out.
    PolicyFiltered,
}

impl ContextChangeKindClass {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::AddedByUser => "added_by_user",
            Self::AddedByMentionTrail => "added_by_mention_trail",
            Self::AddedByRetrieval => "added_by_retrieval",
            Self::RestoredAfterOmission => "restored_after_omission",
            Self::RemovedByUser => "removed_by_user",
            Self::OmittedUnderBudget => "omitted_under_budget",
            Self::RemovedAsStale => "removed_as_stale",
            Self::PolicyFiltered => "policy_filtered",
        }
    }

    /// Whether this change added or restored the source.
    pub const fn is_addition(self) -> bool {
        matches!(
            self,
            Self::AddedByUser
                | Self::AddedByMentionTrail
                | Self::AddedByRetrieval
                | Self::RestoredAfterOmission
        )
    }

    /// Whether this change removed, omitted, or filtered the source out.
    pub const fn is_removal_or_omission(self) -> bool {
        matches!(
            self,
            Self::RemovedByUser
                | Self::OmittedUnderBudget
                | Self::RemovedAsStale
                | Self::PolicyFiltered
        )
    }
}

/// Precise reason a context change fired.
///
/// [`Self::Unspecified`] is the catch-all that may never stand in for a precise
/// reason on a removal or omission when a more precise reason exists.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ContextChangeReasonClass {
    /// The user acted directly to add or remove the source.
    UserAction,
    /// A typed mention resolved and pulled the source in.
    MentionResolved,
    /// Retrieval ranked the source as a contribution.
    RetrievalContribution,
    /// The context budget could not fit the source.
    BudgetPressure,
    /// Policy or trust settings filtered the source out.
    PolicyFilter,
    /// The source went stale and was invalidated.
    StaleInvalidation,
    /// Scope or consent was revoked, dropping the source.
    ScopeRevoked,
    /// No more precise reason is recorded.
    Unspecified,
}

impl ContextChangeReasonClass {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::UserAction => "user_action",
            Self::MentionResolved => "mention_resolved",
            Self::RetrievalContribution => "retrieval_contribution",
            Self::BudgetPressure => "budget_pressure",
            Self::PolicyFilter => "policy_filter",
            Self::StaleInvalidation => "stale_invalidation",
            Self::ScopeRevoked => "scope_revoked",
            Self::Unspecified => "unspecified",
        }
    }

    /// Whether this is the generic catch-all reason.
    pub const fn is_unspecified(self) -> bool {
        matches!(self, Self::Unspecified)
    }
}

/// How replay-safe a session artifact's evidence lineage is.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ReplaySafetyClass {
    /// Replay reproduces the turn from refs and coarse classes alone.
    FullyReplaySafe,
    /// Replay reproduces the turn from a redacted, export-safe projection.
    ReplaySafeRedacted,
    /// Replay can only reconstruct review-safe labels, not full context.
    ReplayDegradedLabelsOnly,
    /// Replay is not possible without material that may not cross the boundary.
    NotReplaySafe,
}

impl ReplaySafetyClass {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::FullyReplaySafe => "fully_replay_safe",
            Self::ReplaySafeRedacted => "replay_safe_redacted",
            Self::ReplayDegradedLabelsOnly => "replay_degraded_labels_only",
            Self::NotReplaySafe => "not_replay_safe",
        }
    }

    /// Whether replay stays inside the export boundary.
    pub const fn is_replay_safe(self) -> bool {
        !matches!(self, Self::NotReplaySafe)
    }
}

/// One attachment's provenance within a session artifact.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AttachmentProvenanceRow {
    /// Stable attachment id shared with the composer draft.
    pub attachment_id: String,
    /// Stable object identity that survives display-label changes.
    pub stable_object_ref: String,
    /// Review-safe origin label.
    pub origin_label: String,
    /// Byte source class for the attachment.
    pub source_class: SourceClass,
    /// Semantic role within the composed context.
    pub semantic_role: AttachmentSemanticRoleClass,
    /// Provenance class for where the attachment came from.
    pub provenance_class: AttachmentProvenanceClass,
    /// Trust posture the composer carries on the attachment.
    pub trust_posture: TrustPosture,
    /// Inclusion state of the attachment in the artifact.
    pub context_state: ContextItemStateClass,
}

/// One mention's resolution provenance within a session artifact.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MentionProvenanceRow {
    /// Stable mention id.
    pub mention_id: String,
    /// Kind of mention the user placed.
    pub mention_kind: MentionKind,
    /// How the resolver settled the mention.
    pub resolution_state: MentionResolutionState,
    /// Stable target the mention resolved to, when resolved.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resolved_target_ref: Option<String>,
    /// Review-safe display label for the mention.
    pub display_label: String,
    /// Whether the mention's target is inside the request workspace scope.
    pub in_scope: bool,
}

impl MentionProvenanceRow {
    /// Whether the mention resolved to a stable target.
    pub fn is_resolved(&self) -> bool {
        self.resolution_state == MentionResolutionState::Resolved
    }

    /// Whether the row is internally consistent.
    ///
    /// A resolved mention names a non-empty target; only a resolved mention may
    /// be in scope; and a scope-excluded mention is never in scope.
    pub fn is_consistent(&self) -> bool {
        let resolved_target_present = self
            .resolved_target_ref
            .as_deref()
            .is_some_and(|target| !target.trim().is_empty());
        match self.resolution_state {
            MentionResolutionState::Resolved => resolved_target_present,
            MentionResolutionState::UnresolvedScopeExcluded => !self.in_scope,
            _ => !self.in_scope,
        }
    }
}

/// One context add-or-remove receipt within a session artifact.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ContextChangeReceipt {
    /// Stable receipt id.
    pub receipt_id: String,
    /// Stable source identity the change applies to.
    pub source_ref: String,
    /// Byte source class for the changed source.
    pub source_class: SourceClass,
    /// How the source entered, left, or stayed out of the context.
    pub change_kind: ContextChangeKindClass,
    /// Precise reason the change fired.
    pub change_reason: ContextChangeReasonClass,
    /// Omission reason when the change omitted or filtered the source.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub omission_reason: Option<ContextOmissionReasonClass>,
    /// Inclusion state of the source before the change.
    pub prior_state: ContextItemStateClass,
    /// Inclusion state of the source after the change.
    pub new_state: ContextItemStateClass,
    /// Whether the change can be reversed by the user.
    pub reversible: bool,
    /// Restore action ref when the change is reversible.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub restore_action_ref: String,
    /// Inspect action ref for the change.
    pub inspect_action_ref: String,
    /// Whether replay, support, and audit flows can see this change.
    pub replay_visible: bool,
}

/// Replay-safe evidence lineage for a session artifact.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReplaySafeEvidence {
    /// Stable, replay-safe evidence id.
    pub evidence_id: String,
    /// Replay lineage ref reconstructing the turn.
    pub replay_lineage_ref: String,
    /// Redaction manifest ref describing what was held back.
    pub redaction_manifest_ref: String,
    /// Route receipt ref backing the turn's route.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub route_receipt_ref: String,
    /// Spend receipt ref backing the turn's spend.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub spend_receipt_ref: String,
    /// Operator-facing evidence packet ref.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub operator_packet_ref: String,
    /// Support-export evidence packet ref.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub support_packet_ref: String,
    /// Compliance/audit evidence packet ref.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub compliance_packet_ref: String,
    /// How replay-safe the evidence is.
    pub replay_safety: ReplaySafetyClass,
    /// Whether replay needs raw prompt text; must be false on the boundary.
    pub requires_raw_prompt_for_replay: bool,
}

/// One downgrade rule that narrows an artifact's claim when a trigger fires.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SessionArtifactDowngradeRule {
    /// Trigger that fires this rule.
    pub trigger: M5AiWorkflowDowngradeTrigger,
    /// Qualification the artifact narrows to when the trigger fires.
    pub narrowed_to: M5AiWorkflowQualificationClass,
    /// Whether tooling enforces this rule automatically.
    pub auto_enforced: bool,
    /// Review-safe rationale for the narrowing.
    pub rationale: String,
}

/// One session-artifact row binding draft, provenance, context change, and
/// replay-safe evidence truth for a single composer draft or session.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SessionArtifactRow {
    /// Stable artifact id.
    pub artifact_id: String,
    /// Composer session ref the artifact belongs to.
    pub session_ref: String,
    /// Composer draft ref the artifact captures.
    pub draft_ref: String,
    /// Context snapshot ref the artifact was composed against.
    pub context_snapshot_ref: String,
    /// Review-safe artifact label.
    pub artifact_label: String,
    /// Consumer surface the artifact lives on.
    pub surface: M5AiWorkflowConsumerSurface,
    /// Lifecycle class fixing how the artifact persists.
    pub artifact_class: SessionArtifactClass,
    /// Qualification class claimed for this artifact.
    pub claimed_qualification: M5AiWorkflowQualificationClass,
    /// Scope the artifact is bound to.
    pub scope: MemoryScopeClass,
    /// Locality posture for the data the artifact holds.
    pub locality: MemoryLocalityClass,
    /// Retention class declared by the artifact.
    pub retention: MemoryRetentionClass,
    /// Delete/export posture for the artifact.
    pub delete_export_posture: MemoryDeleteExportPosture,
    /// Attachment-and-mention provenance: attachments.
    pub attachment_provenance: Vec<AttachmentProvenanceRow>,
    /// Attachment-and-mention provenance: mentions.
    pub mention_provenance: Vec<MentionProvenanceRow>,
    /// Context add-or-remove receipts.
    pub context_receipts: Vec<ContextChangeReceipt>,
    /// Replay-safe evidence lineage.
    pub evidence: ReplaySafeEvidence,
    /// Downgrade rules that narrow the claim.
    pub downgrade_rules: Vec<SessionArtifactDowngradeRule>,
    /// Rollback posture for a scope/retention change on this artifact.
    pub rollback_posture: M5AiWorkflowRollbackPosture,
    /// True when the rollback path has been drilled and verified.
    pub rollback_verified: bool,
    /// Inspect action ref for the artifact.
    pub inspect_action_ref: String,
    /// Delete action ref for the artifact.
    pub delete_action_ref: String,
    /// Export action ref for the artifact.
    pub export_action_ref: String,
}

impl SessionArtifactRow {
    /// Whether this artifact carries a publicly claimed qualification.
    ///
    /// Stable, Beta, and Preview are claimed lanes; Experimental, Held, and
    /// Unavailable are not.
    pub fn is_claimed(&self) -> bool {
        matches!(
            self.claimed_qualification,
            M5AiWorkflowQualificationClass::Stable
                | M5AiWorkflowQualificationClass::Beta
                | M5AiWorkflowQualificationClass::Preview
        )
    }

    /// Whether this artifact is an in-progress draft.
    pub fn is_draft(&self) -> bool {
        self.artifact_class.is_draft()
    }

    /// Whether the artifact retains anything durable that must declare a real
    /// delete/export posture.
    pub fn is_durable(&self) -> bool {
        self.retention.is_durable() && !self.artifact_class.is_tombstone()
    }

    /// Qualification this artifact narrows to when `trigger` fires.
    ///
    /// Returns the claimed qualification unchanged when no rule matches; this is
    /// the deterministic downgrade automation consumers and release tooling
    /// project instead of re-deriving narrowing locally.
    pub fn narrowed_qualification(
        &self,
        trigger: M5AiWorkflowDowngradeTrigger,
    ) -> M5AiWorkflowQualificationClass {
        self.downgrade_rules
            .iter()
            .find(|rule| rule.trigger == trigger)
            .map(|rule| rule.narrowed_to)
            .unwrap_or(self.claimed_qualification)
    }

    /// Renders a deterministic, review-safe inspector card for this artifact.
    pub fn render_inspector(&self) -> String {
        let mut out = String::new();
        out.push_str(&format!("### Artifact `{}`\n", self.artifact_id));
        out.push_str(&format!(
            "- Session: `{}` / draft `{}` on surface `{}`\n",
            self.session_ref,
            self.draft_ref,
            self.surface.as_str()
        ));
        out.push_str(&format!(
            "- Class: `{}` / qualification `{}`\n",
            self.artifact_class.as_str(),
            self.claimed_qualification.as_str()
        ));
        out.push_str(&format!(
            "- Scope: `{}` / locality `{}` / retention `{}` / delete `{}`\n",
            self.scope.as_str(),
            self.locality.as_str(),
            self.retention.as_str(),
            self.delete_export_posture.as_str()
        ));
        out.push_str(&format!(
            "- Attachments / mentions / context receipts: {} / {} / {}\n",
            self.attachment_provenance.len(),
            self.mention_provenance.len(),
            self.context_receipts.len()
        ));
        for receipt in &self.context_receipts {
            out.push_str(&format!(
                "  - context `{}`: `{}` ({}) replay-visible: {}\n",
                receipt.source_ref,
                receipt.change_kind.as_str(),
                receipt.change_reason.as_str(),
                receipt.replay_visible
            ));
        }
        out.push_str(&format!(
            "- Evidence: `{}` / replay `{}` (raw-prompt-free: {})\n",
            self.evidence.evidence_id,
            self.evidence.replay_safety.as_str(),
            !self.evidence.requires_raw_prompt_for_replay
        ));
        out
    }
}

/// Proof freshness block for the session-artifact packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SessionArtifactProofFreshness {
    /// Proof-freshness SLO in hours.
    pub proof_freshness_slo_hours: u32,
    /// RFC 3339 timestamp of the last proof refresh.
    pub last_proof_refresh: String,
    /// True when stale proof automatically narrows claimed artifacts.
    pub auto_narrow_on_stale: bool,
}

/// Constructor input for [`PromptSessionArtifactPacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PromptSessionArtifactPacketInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable catalogue label.
    pub catalogue_label: String,
    /// Session-artifact rows.
    pub artifacts: Vec<SessionArtifactRow>,
    /// Proof freshness block.
    pub proof_freshness: SessionArtifactProofFreshness,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

/// Export-safe prompt session-artifact packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PromptSessionArtifactPacket {
    /// Record kind; must equal [`SESSION_ARTIFACT_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`SESSION_ARTIFACT_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable catalogue label.
    pub catalogue_label: String,
    /// Session-artifact rows.
    pub artifacts: Vec<SessionArtifactRow>,
    /// Proof freshness block.
    pub proof_freshness: SessionArtifactProofFreshness,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

impl PromptSessionArtifactPacket {
    /// Builds a session-artifact packet from stable-lane input.
    pub fn new(input: PromptSessionArtifactPacketInput) -> Self {
        Self {
            record_kind: SESSION_ARTIFACT_RECORD_KIND.to_owned(),
            schema_version: SESSION_ARTIFACT_SCHEMA_VERSION,
            packet_id: input.packet_id,
            catalogue_label: input.catalogue_label,
            artifacts: input.artifacts,
            proof_freshness: input.proof_freshness,
            source_contract_refs: input.source_contract_refs,
            redaction_class_token: input.redaction_class_token,
            minted_at: input.minted_at,
        }
    }

    /// Validates the session-artifact invariants.
    pub fn validate(&self) -> Vec<SessionArtifactViolation> {
        let mut violations = Vec::new();

        if self.record_kind != SESSION_ARTIFACT_RECORD_KIND {
            violations.push(SessionArtifactViolation::WrongRecordKind);
        }
        if self.schema_version != SESSION_ARTIFACT_SCHEMA_VERSION {
            violations.push(SessionArtifactViolation::WrongSchemaVersion);
        }
        if self.packet_id.trim().is_empty()
            || self.catalogue_label.trim().is_empty()
            || self.redaction_class_token.trim().is_empty()
            || self.minted_at.trim().is_empty()
        {
            violations.push(SessionArtifactViolation::MissingIdentity);
        }

        validate_source_contracts(self, &mut violations);
        validate_artifacts_present(self, &mut violations);
        validate_surface_coverage(self, &mut violations);
        for artifact in &self.artifacts {
            validate_artifact(artifact, &mut violations);
        }
        validate_proof_freshness(self, &mut violations);

        if json_contains_forbidden_boundary_material(
            &serde_json::to_value(self).expect("session artifact packet serializes"),
        ) {
            violations.push(SessionArtifactViolation::RawBoundaryMaterialInExport);
        }

        violations
    }

    /// Count of artifacts carrying a publicly claimed qualification.
    pub fn claimed_artifact_count(&self) -> usize {
        self.artifacts.iter().filter(|a| a.is_claimed()).count()
    }

    /// Count of artifacts that are in-progress drafts.
    pub fn draft_artifact_count(&self) -> usize {
        self.artifacts.iter().filter(|a| a.is_draft()).count()
    }

    /// Returns the artifact row for `artifact_id`, if present.
    pub fn artifact(&self, artifact_id: &str) -> Option<&SessionArtifactRow> {
        self.artifacts.iter().find(|a| a.artifact_id == artifact_id)
    }

    /// Deterministic export-safe JSON.
    ///
    /// # Panics
    ///
    /// Panics only if serializing this metadata-only packet fails.
    pub fn export_safe_json(&self) -> String {
        serde_json::to_string_pretty(self).expect("session artifact packet serializes")
    }

    /// Deterministic Markdown summary for support, docs, or review handoff.
    pub fn render_markdown_summary(&self) -> String {
        let mut out = String::new();
        out.push_str("# Prompt-Composer Draft And Session-Artifact Records\n\n");
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- Label: `{}`\n", self.catalogue_label));
        out.push_str(&format!(
            "- Artifacts: {} ({} claimed, {} drafts)\n",
            self.artifacts.len(),
            self.claimed_artifact_count(),
            self.draft_artifact_count()
        ));
        out.push_str(&format!(
            "- Proof freshness SLO: {} hours (last refresh: {})\n",
            self.proof_freshness.proof_freshness_slo_hours, self.proof_freshness.last_proof_refresh
        ));
        out.push_str("\n## Artifact inspectors\n\n");
        for artifact in &self.artifacts {
            out.push_str(&artifact.render_inspector());
            out.push('\n');
        }
        out
    }
}

/// Errors emitted when reading the checked-in session-artifact export.
#[derive(Debug)]
pub enum SessionArtifactArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<SessionArtifactViolation>),
}

impl fmt::Display for SessionArtifactArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(formatter, "session artifact export parse failed: {error}")
            }
            Self::Validation(violations) => {
                let tokens = violations
                    .iter()
                    .map(|violation| violation.as_str())
                    .collect::<Vec<_>>()
                    .join(",");
                write!(
                    formatter,
                    "session artifact export failed validation: {tokens}"
                )
            }
        }
    }
}

impl Error for SessionArtifactArtifactError {}

/// Validation failures emitted by [`PromptSessionArtifactPacket::validate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SessionArtifactViolation {
    /// Packet record kind is wrong.
    WrongRecordKind,
    /// Packet schema version is wrong.
    WrongSchemaVersion,
    /// Required identity field is missing.
    MissingIdentity,
    /// Source contract refs are incomplete.
    MissingSourceContracts,
    /// The packet carries no artifacts.
    NoArtifacts,
    /// An artifact id appears more than once.
    DuplicateArtifact,
    /// A required consumer surface has no artifact covering it.
    SurfaceCoverageIncomplete,
    /// An artifact row is missing a required identity or label field.
    ArtifactRowIncomplete,
    /// An in-progress draft drifted into durable memory instead of staying scoped.
    DraftDriftedToDurableMemory,
    /// A durable artifact does not declare an actionable delete/export posture.
    DurableArtifactNotDeletable,
    /// An artifact's locality would allow cross-workspace or cross-tenant recall.
    ScopeLocalityMismatch,
    /// An attachment provenance row is missing origin or identity.
    AttachmentProvenanceIncomplete,
    /// A mention provenance row's resolution and scope disagree.
    MentionResolutionInconsistent,
    /// A context change receipt is missing a required field.
    ContextReceiptIncomplete,
    /// A context change is not visible to replay, support, or audit.
    ContextChangeNotReplayVisible,
    /// A removal or omission collapsed into an unspecified reason.
    ContextChangeReasonTooGeneric,
    /// The replay-safe evidence lineage is missing core refs.
    EvidenceLineageIncomplete,
    /// The evidence demands raw prompt text to replay.
    EvidenceRequiresRawPrompt,
    /// A claimed artifact's evidence is not replay-safe.
    EvidenceNotReplaySafe,
    /// A claimed artifact is missing required evidence packet refs.
    ClaimedArtifactMissingEvidence,
    /// A claimed artifact's reversing rollback path is not verified.
    ClaimedRollbackUnverified,
    /// An artifact has no downgrade rules.
    DowngradeRulesMissing,
    /// An artifact's downgrade rules omit the proof-stale trigger.
    DowngradeRuleMissingProofStale,
    /// An artifact's downgrade rules omit the trust-narrowing trigger.
    DowngradeRuleMissingTrustNarrowing,
    /// A downgrade rule does not narrow below the claimed qualification.
    DowngradeRuleNotNarrowing,
    /// Proof freshness block is incomplete.
    ProofFreshnessIncomplete,
    /// Export contains raw boundary material.
    RawBoundaryMaterialInExport,
}

impl SessionArtifactViolation {
    /// Stable token used in tests and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WrongRecordKind => "wrong_record_kind",
            Self::WrongSchemaVersion => "wrong_schema_version",
            Self::MissingIdentity => "missing_identity",
            Self::MissingSourceContracts => "missing_source_contracts",
            Self::NoArtifacts => "no_artifacts",
            Self::DuplicateArtifact => "duplicate_artifact",
            Self::SurfaceCoverageIncomplete => "surface_coverage_incomplete",
            Self::ArtifactRowIncomplete => "artifact_row_incomplete",
            Self::DraftDriftedToDurableMemory => "draft_drifted_to_durable_memory",
            Self::DurableArtifactNotDeletable => "durable_artifact_not_deletable",
            Self::ScopeLocalityMismatch => "scope_locality_mismatch",
            Self::AttachmentProvenanceIncomplete => "attachment_provenance_incomplete",
            Self::MentionResolutionInconsistent => "mention_resolution_inconsistent",
            Self::ContextReceiptIncomplete => "context_receipt_incomplete",
            Self::ContextChangeNotReplayVisible => "context_change_not_replay_visible",
            Self::ContextChangeReasonTooGeneric => "context_change_reason_too_generic",
            Self::EvidenceLineageIncomplete => "evidence_lineage_incomplete",
            Self::EvidenceRequiresRawPrompt => "evidence_requires_raw_prompt",
            Self::EvidenceNotReplaySafe => "evidence_not_replay_safe",
            Self::ClaimedArtifactMissingEvidence => "claimed_artifact_missing_evidence",
            Self::ClaimedRollbackUnverified => "claimed_rollback_unverified",
            Self::DowngradeRulesMissing => "downgrade_rules_missing",
            Self::DowngradeRuleMissingProofStale => "downgrade_rule_missing_proof_stale",
            Self::DowngradeRuleMissingTrustNarrowing => "downgrade_rule_missing_trust_narrowing",
            Self::DowngradeRuleNotNarrowing => "downgrade_rule_not_narrowing",
            Self::ProofFreshnessIncomplete => "proof_freshness_incomplete",
            Self::RawBoundaryMaterialInExport => "raw_boundary_material_in_export",
        }
    }
}

/// Reads and validates the checked-in session-artifact export.
///
/// # Errors
///
/// Returns an artifact error if the checked-in export does not parse or validate.
pub fn current_prompt_session_artifact_export(
) -> Result<PromptSessionArtifactPacket, SessionArtifactArtifactError> {
    let packet: PromptSessionArtifactPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/ai/m5/ship_prompt_composer_draft_and_session_artifact_records_attachment_and_mention_provenance_context_add_or_remove_receipts/support_export.json"
    )))
    .map_err(SessionArtifactArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(SessionArtifactArtifactError::Validation(violations))
    }
}

/// Required consumer surfaces the packet must cover across M5 AI surfaces.
const REQUIRED_SURFACES: [M5AiWorkflowConsumerSurface; 3] = [
    M5AiWorkflowConsumerSurface::DesktopComposer,
    M5AiWorkflowConsumerSurface::DesktopReviewWorkspace,
    M5AiWorkflowConsumerSurface::BrowserCompanion,
];

/// Ordinal rank used to compare qualification severity for downgrade rules.
///
/// Higher means a stronger public claim, so a downgrade must move to a strictly
/// lower rank.
fn qualification_rank(class: M5AiWorkflowQualificationClass) -> u8 {
    match class {
        M5AiWorkflowQualificationClass::Unavailable => 0,
        M5AiWorkflowQualificationClass::Held => 1,
        M5AiWorkflowQualificationClass::Experimental => 2,
        M5AiWorkflowQualificationClass::Preview => 3,
        M5AiWorkflowQualificationClass::Beta => 4,
        M5AiWorkflowQualificationClass::Stable => 5,
    }
}

/// Whether a scope keeps recall inside its boundary for the given locality.
///
/// A workspace artifact must not sit in a tenant-wide pinned store, and an org
/// artifact must stay tenant-pinned, so recall never crosses a workspace or
/// tenant boundary by default. Turn- and thread-scoped artifacts are
/// session-bound and accept any backing locality.
fn scope_allows_locality(scope: MemoryScopeClass, locality: MemoryLocalityClass) -> bool {
    match scope {
        MemoryScopeClass::Turn | MemoryScopeClass::Thread => true,
        MemoryScopeClass::Workspace => locality != MemoryLocalityClass::TenantRegionPinned,
        MemoryScopeClass::Org => matches!(
            locality,
            MemoryLocalityClass::TenantRegionPinned
                | MemoryLocalityClass::ManagedHostedRegionPinned
        ),
    }
}

fn validate_source_contracts(
    packet: &PromptSessionArtifactPacket,
    violations: &mut Vec<SessionArtifactViolation>,
) {
    let refs: BTreeSet<&str> = packet
        .source_contract_refs
        .iter()
        .map(String::as_str)
        .collect();
    for required in [
        SESSION_ARTIFACT_SCHEMA_REF,
        SESSION_ARTIFACT_DOC_REF,
        RICHER_PROMPT_COMPOSER_SCHEMA_REF,
        M5_AI_WORKFLOW_MATRIX_SCHEMA_REF,
        MEMORY_CLASS_MATERIALIZATION_SCHEMA_REF,
        AI_RUN_RECEIPT_SCHEMA_REF,
    ] {
        if !refs.contains(required) {
            violations.push(SessionArtifactViolation::MissingSourceContracts);
            return;
        }
    }
}

fn validate_artifacts_present(
    packet: &PromptSessionArtifactPacket,
    violations: &mut Vec<SessionArtifactViolation>,
) {
    if packet.artifacts.is_empty() {
        violations.push(SessionArtifactViolation::NoArtifacts);
        return;
    }
    let mut seen: BTreeSet<&str> = BTreeSet::new();
    for artifact in &packet.artifacts {
        if !seen.insert(artifact.artifact_id.as_str()) {
            violations.push(SessionArtifactViolation::DuplicateArtifact);
        }
    }
}

fn validate_surface_coverage(
    packet: &PromptSessionArtifactPacket,
    violations: &mut Vec<SessionArtifactViolation>,
) {
    for surface in REQUIRED_SURFACES {
        if !packet
            .artifacts
            .iter()
            .any(|artifact| artifact.surface == surface)
        {
            violations.push(SessionArtifactViolation::SurfaceCoverageIncomplete);
            return;
        }
    }
}

fn validate_artifact(
    artifact: &SessionArtifactRow,
    violations: &mut Vec<SessionArtifactViolation>,
) {
    if artifact.artifact_id.trim().is_empty()
        || artifact.session_ref.trim().is_empty()
        || artifact.draft_ref.trim().is_empty()
        || artifact.context_snapshot_ref.trim().is_empty()
        || artifact.artifact_label.trim().is_empty()
        || artifact.inspect_action_ref.trim().is_empty()
        || artifact.delete_action_ref.trim().is_empty()
        || artifact.export_action_ref.trim().is_empty()
        || artifact
            .downgrade_rules
            .iter()
            .any(|rule| rule.rationale.trim().is_empty())
    {
        violations.push(SessionArtifactViolation::ArtifactRowIncomplete);
    }

    validate_locality(artifact, violations);
    validate_attachments(artifact, violations);
    validate_mentions(artifact, violations);
    validate_context_receipts(artifact, violations);
    validate_evidence(artifact, violations);
    validate_downgrade_rules(artifact, violations);

    // A claimed artifact whose scope/retention change can be reversed must have
    // drilled that reversal; a non-applicable posture carries no reversal.
    if artifact.is_claimed()
        && artifact.rollback_posture != M5AiWorkflowRollbackPosture::NotApplicable
        && !artifact.rollback_verified
    {
        violations.push(SessionArtifactViolation::ClaimedRollbackUnverified);
    }
}

fn validate_locality(
    artifact: &SessionArtifactRow,
    violations: &mut Vec<SessionArtifactViolation>,
) {
    // A draft must stay scoped to its session rather than drifting into durable
    // memory.
    if artifact.is_draft() && artifact.retention.is_durable() {
        violations.push(SessionArtifactViolation::DraftDriftedToDurableMemory);
    }

    // Anything durable must declare a real, actionable delete/export posture.
    if artifact.is_durable() && !artifact.delete_export_posture.is_actionable() {
        violations.push(SessionArtifactViolation::DurableArtifactNotDeletable);
    }

    // Locality must keep recall inside the artifact's scope.
    if !scope_allows_locality(artifact.scope, artifact.locality) {
        violations.push(SessionArtifactViolation::ScopeLocalityMismatch);
    }
}

fn validate_attachments(
    artifact: &SessionArtifactRow,
    violations: &mut Vec<SessionArtifactViolation>,
) {
    for row in &artifact.attachment_provenance {
        if row.attachment_id.trim().is_empty()
            || row.stable_object_ref.trim().is_empty()
            || row.origin_label.trim().is_empty()
        {
            violations.push(SessionArtifactViolation::AttachmentProvenanceIncomplete);
            break;
        }
    }
}

fn validate_mentions(
    artifact: &SessionArtifactRow,
    violations: &mut Vec<SessionArtifactViolation>,
) {
    for row in &artifact.mention_provenance {
        if row.mention_id.trim().is_empty() || row.display_label.trim().is_empty() {
            violations.push(SessionArtifactViolation::ArtifactRowIncomplete);
            break;
        }
    }
    if artifact
        .mention_provenance
        .iter()
        .any(|row| !row.is_consistent())
    {
        violations.push(SessionArtifactViolation::MentionResolutionInconsistent);
    }
}

fn validate_context_receipts(
    artifact: &SessionArtifactRow,
    violations: &mut Vec<SessionArtifactViolation>,
) {
    let mut incomplete = false;
    let mut not_replay_visible = false;
    let mut reason_too_generic = false;
    for receipt in &artifact.context_receipts {
        if receipt.receipt_id.trim().is_empty()
            || receipt.source_ref.trim().is_empty()
            || receipt.inspect_action_ref.trim().is_empty()
            || (receipt.reversible && receipt.restore_action_ref.trim().is_empty())
        {
            incomplete = true;
        }
        // Every add/remove/omit must be inspectable in replay, support, and audit
        // so the lane never behaves like a hidden shadow store.
        if !receipt.replay_visible {
            not_replay_visible = true;
        }
        // A removal or omission must carry a precise reason rather than collapsing
        // into an unspecified catch-all.
        if receipt.change_kind.is_removal_or_omission() && receipt.change_reason.is_unspecified() {
            reason_too_generic = true;
        }
    }
    if incomplete {
        violations.push(SessionArtifactViolation::ContextReceiptIncomplete);
    }
    if not_replay_visible {
        violations.push(SessionArtifactViolation::ContextChangeNotReplayVisible);
    }
    if reason_too_generic {
        violations.push(SessionArtifactViolation::ContextChangeReasonTooGeneric);
    }
}

fn validate_evidence(
    artifact: &SessionArtifactRow,
    violations: &mut Vec<SessionArtifactViolation>,
) {
    let evidence = &artifact.evidence;

    // Every artifact carries a replay-safe id, replay lineage, and redaction
    // manifest so support and compliance can inspect the turn.
    if evidence.evidence_id.trim().is_empty()
        || evidence.replay_lineage_ref.trim().is_empty()
        || evidence.redaction_manifest_ref.trim().is_empty()
    {
        violations.push(SessionArtifactViolation::EvidenceLineageIncomplete);
    }

    // Replay must never demand raw prompt text on the boundary.
    if evidence.requires_raw_prompt_for_replay {
        violations.push(SessionArtifactViolation::EvidenceRequiresRawPrompt);
    }

    if artifact.is_claimed() {
        // A claimed artifact's replay must stay inside the export boundary.
        if !evidence.replay_safety.is_replay_safe() {
            violations.push(SessionArtifactViolation::EvidenceNotReplaySafe);
        }
        // A claimed artifact carries the full route, spend, operator, support, and
        // compliance evidence refs that back the turn.
        if evidence.route_receipt_ref.trim().is_empty()
            || evidence.spend_receipt_ref.trim().is_empty()
            || evidence.operator_packet_ref.trim().is_empty()
            || evidence.support_packet_ref.trim().is_empty()
            || evidence.compliance_packet_ref.trim().is_empty()
        {
            violations.push(SessionArtifactViolation::ClaimedArtifactMissingEvidence);
        }
    }
}

fn validate_downgrade_rules(
    artifact: &SessionArtifactRow,
    violations: &mut Vec<SessionArtifactViolation>,
) {
    if artifact.downgrade_rules.is_empty() {
        violations.push(SessionArtifactViolation::DowngradeRulesMissing);
        return;
    }

    if !artifact
        .downgrade_rules
        .iter()
        .any(|rule| rule.trigger == M5AiWorkflowDowngradeTrigger::ProofStale)
    {
        violations.push(SessionArtifactViolation::DowngradeRuleMissingProofStale);
    }

    // Scope or consent revocation narrows through the trust-narrowing trigger, so
    // every artifact must carry it.
    if !artifact
        .downgrade_rules
        .iter()
        .any(|rule| rule.trigger == M5AiWorkflowDowngradeTrigger::TrustNarrowing)
    {
        violations.push(SessionArtifactViolation::DowngradeRuleMissingTrustNarrowing);
    }

    let claimed_rank = qualification_rank(artifact.claimed_qualification);
    for rule in &artifact.downgrade_rules {
        if qualification_rank(rule.narrowed_to) >= claimed_rank {
            violations.push(SessionArtifactViolation::DowngradeRuleNotNarrowing);
            break;
        }
    }
}

fn validate_proof_freshness(
    packet: &PromptSessionArtifactPacket,
    violations: &mut Vec<SessionArtifactViolation>,
) {
    if packet.proof_freshness.proof_freshness_slo_hours == 0
        || packet.proof_freshness.last_proof_refresh.trim().is_empty()
    {
        violations.push(SessionArtifactViolation::ProofFreshnessIncomplete);
    }
}

/// Heuristic that rejects obviously forbidden material in export-safe JSON.
fn json_contains_forbidden_boundary_material(value: &serde_json::Value) -> bool {
    match value {
        serde_json::Value::String(s) => {
            let lower = s.to_lowercase();
            lower.contains("api_key")
                || lower.contains("password")
                || lower.contains("secret")
                || lower.contains("bearer ")
                || lower.contains("https://")
                || lower.contains("http://")
        }
        serde_json::Value::Array(arr) => arr.iter().any(json_contains_forbidden_boundary_material),
        serde_json::Value::Object(map) => {
            map.values().any(json_contains_forbidden_boundary_material)
        }
        _ => false,
    }
}
