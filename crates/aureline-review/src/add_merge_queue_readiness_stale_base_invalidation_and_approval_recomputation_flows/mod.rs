//! Merge-queue readiness, stale-base invalidation, and approval-recomputation flows.
//!
//! This module implements the canonical M5 truth packet for keeping a merge
//! queue honest as the base advances under queued changes. It binds three
//! pillars into one export-safe record:
//!
//! - **Merge-queue readiness** — each [`MergeQueueReadinessRow`] carries a
//!   queued change's target identity, durable review anchor, queue position,
//!   base freshness, readiness verdict, mutation authority, and the blocking
//!   reasons behind a non-ready verdict, so the queue never overstates that a
//!   change is ready to land.
//! - **Stale-base invalidation** — each [`StaleBaseInvalidationRow`] records how
//!   a base advance invalidates a queued entry's readiness, labeling the
//!   invalidation action (requeue after rerun, auto-rebase proposed, eject to
//!   author, hold for resolution) rather than silently keeping the entry green.
//! - **Approval recomputation** — each [`ApprovalRecomputationRow`] records how
//!   approvals are recomputed when the diff or base changes, labeling whether
//!   approvals were retained, fully reset, partially invalidated, or
//!   re-requested rather than implying a stale approval still holds.
//!
//! The packet references upstream merge-queue, landing, anchor stability, and
//! pipeline-run contracts by id rather than embedding their content. Raw diff
//! bodies, raw build logs, raw pipeline artifacts, raw provider payloads,
//! credentials, and live provider responses stay outside the support boundary.
//!
//! The boundary schema is
//! [`schemas/review/add-merge-queue-readiness-stale-base-invalidation-and-approval-recomputation-flows.schema.json`](../../../../schemas/review/add-merge-queue-readiness-stale-base-invalidation-and-approval-recomputation-flows.schema.json).
//! The contract doc is
//! [`docs/review/m5/add_merge_queue_readiness_stale_base_invalidation_and_approval_recomputation_flows.md`](../../../../docs/review/m5/add_merge_queue_readiness_stale_base_invalidation_and_approval_recomputation_flows.md).
//! The protected fixture directory is
//! [`fixtures/review/m5/add_merge_queue_readiness_stale_base_invalidation_and_approval_recomputation_flows/`](../../../../fixtures/review/m5/add_merge_queue_readiness_stale_base_invalidation_and_approval_recomputation_flows/).

#[cfg(test)]
mod tests;

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

/// Stable record-kind tag carried by [`MergeQueueReadinessPacket`].
pub const MERGE_QUEUE_READINESS_RECORD_KIND: &str =
    "merge_queue_readiness_stale_base_invalidation_and_approval_recomputation";

/// Schema version for merge-queue readiness records.
pub const MERGE_QUEUE_READINESS_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the boundary schema.
pub const MERGE_QUEUE_READINESS_SCHEMA_REF: &str =
    "schemas/review/add-merge-queue-readiness-stale-base-invalidation-and-approval-recomputation-flows.schema.json";

/// Repo-relative path of the merge-queue readiness contract doc.
pub const MERGE_QUEUE_READINESS_DOC_REF: &str =
    "docs/review/m5/add_merge_queue_readiness_stale_base_invalidation_and_approval_recomputation_flows.md";

/// Repo-relative path of the merge-queue entry contract this lane builds on.
pub const MERGE_QUEUE_READINESS_MERGE_QUEUE_CONTRACT_REF: &str =
    "schemas/review/merge_queue_entry.schema.json";

/// Repo-relative path of the landing-candidate contract that anchors readiness.
pub const MERGE_QUEUE_READINESS_LANDING_CONTRACT_REF: &str =
    "schemas/review/landing_candidate.schema.json";

/// Repo-relative path of the anchor-stability contract reused for approval recomputation.
pub const MERGE_QUEUE_READINESS_ANCHOR_STABILITY_CONTRACT_REF: &str =
    "schemas/review/review_stabilization.schema.json";

/// Repo-relative path of the pipeline-run contract that gating checks mirror.
pub const MERGE_QUEUE_READINESS_PIPELINE_RUN_CONTRACT_REF: &str =
    "schemas/ci/pipeline_run_row.schema.json";

/// Repo-relative path of the protected fixture directory.
pub const MERGE_QUEUE_READINESS_FIXTURE_DIR: &str =
    "fixtures/review/m5/add_merge_queue_readiness_stale_base_invalidation_and_approval_recomputation_flows";

/// Repo-relative path of the checked support-export artifact.
pub const MERGE_QUEUE_READINESS_ARTIFACT_REF: &str =
    "artifacts/review/m5/add_merge_queue_readiness_stale_base_invalidation_and_approval_recomputation_flows/support_export.json";

/// Repo-relative path of the checked Markdown summary.
pub const MERGE_QUEUE_READINESS_SUMMARY_REF: &str =
    "artifacts/review/m5/add_merge_queue_readiness_stale_base_invalidation_and_approval_recomputation_flows.md";

/// Base-freshness class shown for a queued merge-queue entry.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum QueueBaseFreshness {
    /// Entry is current against the queue base.
    Current,
    /// The base advanced under the entry; the queue shows a stale-base label.
    StaleBase,
    /// Entry and base histories have diverged.
    Diverged,
    /// Freshness cannot be computed (for example, offline).
    Unknown,
}

impl QueueBaseFreshness {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Current => "current",
            Self::StaleBase => "stale_base",
            Self::Diverged => "diverged",
            Self::Unknown => "unknown",
        }
    }

    /// Whether this class represents a stale or diverged base that must be labeled.
    pub const fn is_stale(self) -> bool {
        matches!(self, Self::StaleBase | Self::Diverged)
    }
}

/// Readiness verdict shown for a queued merge-queue entry.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MergeQueueReadinessVerdict {
    /// The entry is ready to land against the current base.
    Ready,
    /// The entry is blocked because one or more required checks have not passed.
    BlockedOnChecks,
    /// The entry is blocked because required approvals are missing or recomputed away.
    BlockedOnApprovals,
    /// The entry is blocked because the base advanced and the entry is stale.
    BlockedOnStaleBase,
    /// The entry is blocked by a policy or legal gate.
    BlockedOnPolicy,
    /// The entry is held manually and is not advancing.
    Holding,
}

impl MergeQueueReadinessVerdict {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Ready => "ready",
            Self::BlockedOnChecks => "blocked_on_checks",
            Self::BlockedOnApprovals => "blocked_on_approvals",
            Self::BlockedOnStaleBase => "blocked_on_stale_base",
            Self::BlockedOnPolicy => "blocked_on_policy",
            Self::Holding => "holding",
        }
    }

    /// Whether this verdict represents a non-ready entry that must carry a blocking reason.
    pub const fn requires_blocking_reason(self) -> bool {
        !matches!(self, Self::Ready)
    }

    /// Whether this verdict requires a matching stale-base invalidation record.
    pub const fn requires_stale_base_invalidation(self) -> bool {
        matches!(self, Self::BlockedOnStaleBase)
    }
}

/// Mutation authority a merge-queue surface may exercise for an entry.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MutationAuthorityClass {
    /// Surface is read-only and never mutates workspace, repository, or remote state.
    ReadOnlyNoMutation,
    /// Surface may trigger writes that stay individually attributable and reviewable.
    AttributableWrite,
}

impl MutationAuthorityClass {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ReadOnlyNoMutation => "read_only_no_mutation",
            Self::AttributableWrite => "attributable_write",
        }
    }
}

/// Action taken when a base advance invalidates a queued entry's readiness.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum StaleBaseInvalidationAction {
    /// Entry is requeued after its checks rerun against the new base.
    RequeueAfterRerun,
    /// An attributable auto-rebase onto the new base is proposed for review.
    AutoRebaseProposed,
    /// Entry is ejected back to its author to resolve against the new base.
    EjectToAuthor,
    /// Entry is held until a conflict or policy block is resolved.
    HoldForResolution,
    /// The base advance did not affect this entry; no action is needed.
    NoActionNeeded,
}

impl StaleBaseInvalidationAction {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::RequeueAfterRerun => "requeue_after_rerun",
            Self::AutoRebaseProposed => "auto_rebase_proposed",
            Self::EjectToAuthor => "eject_to_author",
            Self::HoldForResolution => "hold_for_resolution",
            Self::NoActionNeeded => "no_action_needed",
        }
    }

    /// Whether this action requires an explicit, non-empty invalidation label.
    pub const fn requires_label(self) -> bool {
        !matches!(self, Self::NoActionNeeded)
    }
}

/// Event that triggered an approval-recomputation pass.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RecomputationTrigger {
    /// The change's diff was rewritten under existing approvals.
    DiffChanged,
    /// The queue base advanced under the entry.
    BaseAdvanced,
    /// A requeue was requested for the entry.
    RequeueRequested,
    /// A reviewer was removed from the required set.
    ReviewerRemoved,
    /// The approval policy for the target changed.
    PolicyChanged,
}

impl RecomputationTrigger {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::DiffChanged => "diff_changed",
            Self::BaseAdvanced => "base_advanced",
            Self::RequeueRequested => "requeue_requested",
            Self::ReviewerRemoved => "reviewer_removed",
            Self::PolicyChanged => "policy_changed",
        }
    }
}

/// Outcome of recomputing approvals for a queued entry.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ApprovalRecomputationOutcome {
    /// Approvals still apply against the recomputed diff and base.
    Retained,
    /// Every approval was invalidated and reset.
    ResetFull,
    /// Some approvals were invalidated; the rest still apply.
    InvalidatedPartial,
    /// Approvals were invalidated and re-requested from reviewers.
    ReRequested,
    /// Recomputation does not apply to this entry yet.
    NotApplicable,
}

impl ApprovalRecomputationOutcome {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Retained => "retained",
            Self::ResetFull => "reset_full",
            Self::InvalidatedPartial => "invalidated_partial",
            Self::ReRequested => "re_requested",
            Self::NotApplicable => "not_applicable",
        }
    }

    /// Whether this outcome requires an explicit, non-empty recomputation label.
    pub const fn requires_label(self) -> bool {
        matches!(
            self,
            Self::ResetFull | Self::InvalidatedPartial | Self::ReRequested
        )
    }

    /// Whether this outcome lowers the approval count and must not increase it.
    pub const fn lowers_approval_count(self) -> bool {
        matches!(self, Self::ResetFull | Self::InvalidatedPartial)
    }
}

/// Downgrade trigger that can narrow this lane below its claimed qualification.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MergeQueueReadinessDowngradeTrigger {
    /// Proof packet has gone stale.
    ProofStale,
    /// Policy or legal block applies.
    PolicyBlocked,
    /// A stale base was surfaced without an explicit invalidation label.
    StaleBaseUnlabeled,
    /// An approval recomputation was surfaced without an explicit label.
    ApprovalRecomputeUnlabeled,
    /// A readiness verdict overstated readiness relative to its blocking reasons.
    ReadinessOverstated,
    /// Workspace trust narrowed.
    TrustNarrowing,
    /// Scope expanded beyond the qualified merge-queue boundary.
    ScopeExpansionUnqualified,
    /// An upstream dependency lane narrowed.
    UpstreamDependencyNarrowed,
}

impl MergeQueueReadinessDowngradeTrigger {
    /// Every trigger, in declaration order.
    pub const ALL: [Self; 8] = [
        Self::ProofStale,
        Self::PolicyBlocked,
        Self::StaleBaseUnlabeled,
        Self::ApprovalRecomputeUnlabeled,
        Self::ReadinessOverstated,
        Self::TrustNarrowing,
        Self::ScopeExpansionUnqualified,
        Self::UpstreamDependencyNarrowed,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ProofStale => "proof_stale",
            Self::PolicyBlocked => "policy_blocked",
            Self::StaleBaseUnlabeled => "stale_base_unlabeled",
            Self::ApprovalRecomputeUnlabeled => "approval_recompute_unlabeled",
            Self::ReadinessOverstated => "readiness_overstated",
            Self::TrustNarrowing => "trust_narrowing",
            Self::ScopeExpansionUnqualified => "scope_expansion_unqualified",
            Self::UpstreamDependencyNarrowed => "upstream_dependency_narrowed",
        }
    }
}

/// Consumer surface that must project this lane's truth.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MergeQueueReadinessConsumerSurface {
    /// Merge-queue panel.
    MergeQueuePanel,
    /// Review workspace header.
    ReviewWorkspaceHeader,
    /// Pipeline viewer.
    PipelineViewer,
    /// CLI / headless replay or JSON output.
    CliHeadless,
    /// Support / export packet.
    SupportExport,
    /// Diagnostics or telemetry surface.
    Diagnostics,
    /// Help / About surface.
    HelpAbout,
}

impl MergeQueueReadinessConsumerSurface {
    /// Every surface, in declaration order.
    pub const ALL: [Self; 7] = [
        Self::MergeQueuePanel,
        Self::ReviewWorkspaceHeader,
        Self::PipelineViewer,
        Self::CliHeadless,
        Self::SupportExport,
        Self::Diagnostics,
        Self::HelpAbout,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::MergeQueuePanel => "merge_queue_panel",
            Self::ReviewWorkspaceHeader => "review_workspace_header",
            Self::PipelineViewer => "pipeline_viewer",
            Self::CliHeadless => "cli_headless",
            Self::SupportExport => "support_export",
            Self::Diagnostics => "diagnostics",
            Self::HelpAbout => "help_about",
        }
    }
}

/// One merge-queue readiness row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MergeQueueReadinessRow {
    /// Stable queue entry id.
    pub entry_id: String,
    /// Human-readable target identity (what is queued to land).
    pub target_identity_label: String,
    /// Durable review anchor id bound to this entry.
    pub durable_anchor_id: String,
    /// Position in the merge queue (1-based).
    pub queue_position: u32,
    /// Base freshness class shown for the entry.
    pub base_freshness: QueueBaseFreshness,
    /// Readiness verdict shown for the entry.
    pub readiness_verdict: MergeQueueReadinessVerdict,
    /// Mutation authority the surface may exercise for the entry.
    pub mutation_authority: MutationAuthorityClass,
    /// Human-readable summary of required checks and their state.
    pub required_checks_summary: String,
    /// Human-readable summary of required approvals and their state.
    pub required_approvals_summary: String,
    /// Blocking reasons; required and non-empty when the verdict is not `ready`.
    pub blocking_reasons: Vec<String>,
    /// Source contract refs consumed by this entry.
    pub source_contract_refs: Vec<String>,
}

/// One stale-base invalidation row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StaleBaseInvalidationRow {
    /// Queue entry id whose readiness was invalidated.
    pub entry_id: String,
    /// Human-readable label of the base advance that triggered invalidation.
    pub base_advance_label: String,
    /// Action taken in response to the base advance.
    pub invalidation_action: StaleBaseInvalidationAction,
    /// Whether checks must rerun before the entry can re-enter the queue.
    pub recompute_required: bool,
    /// Invalidation label; required and non-empty when the action requires it.
    pub invalidation_label: String,
}

/// One approval-recomputation row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ApprovalRecomputationRow {
    /// Queue entry id whose approvals were recomputed.
    pub entry_id: String,
    /// Event that triggered the recomputation pass.
    pub trigger: RecomputationTrigger,
    /// Resulting recomputation outcome.
    pub outcome: ApprovalRecomputationOutcome,
    /// Approval count before recomputation.
    pub approvals_before: u32,
    /// Approval count after recomputation.
    pub approvals_after: u32,
    /// Recomputation label; required and non-empty when the outcome requires it.
    pub recomputation_label: String,
}

/// Trust and provenance review block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MergeQueueReadinessTrustReview {
    /// Readiness verdicts are explicit, never implied.
    pub readiness_verdict_explicit: bool,
    /// Readiness is never overstated relative to its blocking reasons.
    pub readiness_never_overstated: bool,
    /// Stale-base invalidation is labeled, never silently kept green.
    pub stale_base_invalidation_labeled_not_hidden: bool,
    /// Approval recomputation is labeled, never silently retained.
    pub approval_recompute_labeled_not_hidden: bool,
    /// Approval recomputation runs when the base or diff changes.
    pub approval_recomputes_on_base_or_diff_change: bool,
    /// Base freshness is shown explicitly.
    pub base_freshness_explicit: bool,
    /// Entry target identity is explicit.
    pub target_identity_explicit: bool,
    /// No merge-queue surface creates hidden write scope; every action is attributable.
    pub no_hidden_write_scope: bool,
    /// Downgrade narrows the claim rather than hiding the lane.
    pub downgrade_narrows_instead_of_hides: bool,
    /// Stale or underqualified rows automatically block promotion.
    pub stale_or_underqualified_blocks_promotion: bool,
}

impl MergeQueueReadinessTrustReview {
    /// Whether every invariant holds.
    pub const fn all_hold(&self) -> bool {
        self.readiness_verdict_explicit
            && self.readiness_never_overstated
            && self.stale_base_invalidation_labeled_not_hidden
            && self.approval_recompute_labeled_not_hidden
            && self.approval_recomputes_on_base_or_diff_change
            && self.base_freshness_explicit
            && self.target_identity_explicit
            && self.no_hidden_write_scope
            && self.downgrade_narrows_instead_of_hides
            && self.stale_or_underqualified_blocks_promotion
    }
}

/// Consumer projection block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MergeQueueReadinessConsumerProjection {
    /// Merge-queue panel shows the readiness verdict.
    pub merge_queue_shows_readiness_verdict: bool,
    /// Merge-queue panel shows base freshness truth.
    pub merge_queue_shows_base_freshness: bool,
    /// Merge-queue panel shows blocking reasons for non-ready entries.
    pub merge_queue_shows_blocking_reasons: bool,
    /// Stale-base surface shows the invalidation action.
    pub stale_base_shows_invalidation_action: bool,
    /// Approval surface shows the recomputation outcome and label.
    pub approval_surface_shows_recomputation: bool,
    /// CLI / headless shows qualification truth.
    pub cli_headless_shows_truth: bool,
    /// Support export shows qualification truth.
    pub support_export_shows_truth: bool,
    /// Diagnostics shows qualification truth.
    pub diagnostics_shows_truth: bool,
    /// Help / About shows qualification truth.
    pub help_about_shows_truth: bool,
    /// Preview / Labs lanes are labeled when not covered by this packet.
    pub preview_labs_label_for_unqualified: bool,
}

impl MergeQueueReadinessConsumerProjection {
    /// Whether every projection invariant holds.
    pub const fn all_hold(&self) -> bool {
        self.merge_queue_shows_readiness_verdict
            && self.merge_queue_shows_base_freshness
            && self.merge_queue_shows_blocking_reasons
            && self.stale_base_shows_invalidation_action
            && self.approval_surface_shows_recomputation
            && self.cli_headless_shows_truth
            && self.support_export_shows_truth
            && self.diagnostics_shows_truth
            && self.help_about_shows_truth
            && self.preview_labs_label_for_unqualified
    }
}

/// Proof freshness block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MergeQueueReadinessProofFreshness {
    /// Proof-freshness SLO in hours.
    pub proof_freshness_slo_hours: u32,
    /// RFC 3339 timestamp of the last proof refresh.
    pub last_proof_refresh: String,
    /// True when stale proof automatically narrows the lane.
    pub auto_narrow_on_stale: bool,
}

/// Constructor input for [`MergeQueueReadinessPacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MergeQueueReadinessPacketInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable surface label.
    pub surface_label: String,
    /// Merge-queue readiness rows.
    pub readiness_entries: Vec<MergeQueueReadinessRow>,
    /// Stale-base invalidation rows.
    pub stale_base_invalidations: Vec<StaleBaseInvalidationRow>,
    /// Approval-recomputation rows.
    pub approval_recomputations: Vec<ApprovalRecomputationRow>,
    /// Downgrade triggers that apply to this lane.
    pub downgrade_triggers: Vec<MergeQueueReadinessDowngradeTrigger>,
    /// Consumer surfaces that must project this lane's truth.
    pub consumer_surfaces: Vec<MergeQueueReadinessConsumerSurface>,
    /// Trust review block.
    pub trust_review: MergeQueueReadinessTrustReview,
    /// Consumer projection block.
    pub consumer_projection: MergeQueueReadinessConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: MergeQueueReadinessProofFreshness,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

/// Export-safe merge-queue readiness, stale-base invalidation, and approval-recomputation packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MergeQueueReadinessPacket {
    /// Record kind; must equal [`MERGE_QUEUE_READINESS_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`MERGE_QUEUE_READINESS_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable surface label.
    pub surface_label: String,
    /// Merge-queue readiness rows.
    pub readiness_entries: Vec<MergeQueueReadinessRow>,
    /// Stale-base invalidation rows.
    pub stale_base_invalidations: Vec<StaleBaseInvalidationRow>,
    /// Approval-recomputation rows.
    pub approval_recomputations: Vec<ApprovalRecomputationRow>,
    /// Downgrade triggers that apply to this lane.
    pub downgrade_triggers: Vec<MergeQueueReadinessDowngradeTrigger>,
    /// Consumer surfaces that must project this lane's truth.
    pub consumer_surfaces: Vec<MergeQueueReadinessConsumerSurface>,
    /// Trust review block.
    pub trust_review: MergeQueueReadinessTrustReview,
    /// Consumer projection block.
    pub consumer_projection: MergeQueueReadinessConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: MergeQueueReadinessProofFreshness,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

impl MergeQueueReadinessPacket {
    /// Builds a merge-queue readiness packet from stable-lane input.
    pub fn new(input: MergeQueueReadinessPacketInput) -> Self {
        Self {
            record_kind: MERGE_QUEUE_READINESS_RECORD_KIND.to_owned(),
            schema_version: MERGE_QUEUE_READINESS_SCHEMA_VERSION,
            packet_id: input.packet_id,
            surface_label: input.surface_label,
            readiness_entries: input.readiness_entries,
            stale_base_invalidations: input.stale_base_invalidations,
            approval_recomputations: input.approval_recomputations,
            downgrade_triggers: input.downgrade_triggers,
            consumer_surfaces: input.consumer_surfaces,
            trust_review: input.trust_review,
            consumer_projection: input.consumer_projection,
            proof_freshness: input.proof_freshness,
            source_contract_refs: input.source_contract_refs,
            redaction_class_token: input.redaction_class_token,
            minted_at: input.minted_at,
        }
    }

    /// Validates the merge-queue readiness invariants.
    pub fn validate(&self) -> Vec<MergeQueueReadinessViolation> {
        let mut violations = Vec::new();

        if self.record_kind != MERGE_QUEUE_READINESS_RECORD_KIND {
            violations.push(MergeQueueReadinessViolation::WrongRecordKind);
        }
        if self.schema_version != MERGE_QUEUE_READINESS_SCHEMA_VERSION {
            violations.push(MergeQueueReadinessViolation::WrongSchemaVersion);
        }
        if self.packet_id.trim().is_empty()
            || self.surface_label.trim().is_empty()
            || self.redaction_class_token.trim().is_empty()
            || self.minted_at.trim().is_empty()
        {
            violations.push(MergeQueueReadinessViolation::MissingIdentity);
        }
        if self.downgrade_triggers.is_empty() {
            violations.push(MergeQueueReadinessViolation::DowngradeTriggersMissing);
        }
        if self.consumer_surfaces.is_empty() {
            violations.push(MergeQueueReadinessViolation::ConsumerSurfacesMissing);
        }

        validate_source_contracts(self, &mut violations);
        validate_readiness_entries(self, &mut violations);
        validate_stale_base_invalidations(self, &mut violations);
        validate_approval_recomputations(self, &mut violations);

        if !self.trust_review.all_hold() {
            violations.push(MergeQueueReadinessViolation::TrustReviewIncomplete);
        }
        if !self.consumer_projection.all_hold() {
            violations.push(MergeQueueReadinessViolation::ConsumerProjectionIncomplete);
        }
        if self.proof_freshness.proof_freshness_slo_hours == 0
            || self.proof_freshness.last_proof_refresh.trim().is_empty()
        {
            violations.push(MergeQueueReadinessViolation::ProofFreshnessIncomplete);
        }

        if json_contains_forbidden_boundary_material(
            &serde_json::to_value(self).expect("merge-queue readiness packet serializes"),
        ) {
            violations.push(MergeQueueReadinessViolation::RawBoundaryMaterialInExport);
        }

        violations
    }

    /// Deterministic export-safe JSON.
    ///
    /// # Panics
    ///
    /// Panics only if serializing this metadata-only packet fails.
    pub fn export_safe_json(&self) -> String {
        serde_json::to_string_pretty(self).expect("merge-queue readiness packet serializes")
    }

    /// Deterministic Markdown summary for support, review, or release handoff.
    pub fn render_markdown_summary(&self) -> String {
        let ready_entries = self
            .readiness_entries
            .iter()
            .filter(|row| row.readiness_verdict == MergeQueueReadinessVerdict::Ready)
            .count();
        let stale_entries = self
            .readiness_entries
            .iter()
            .filter(|row| row.base_freshness.is_stale())
            .count();
        let recomputed_away = self
            .approval_recomputations
            .iter()
            .filter(|row| row.outcome.requires_label())
            .count();

        let mut out = String::new();
        out.push_str(
            "# Merge-Queue Readiness, Stale-Base Invalidation, and Approval Recomputation\n\n",
        );
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- Surface: `{}`\n", self.surface_label));
        out.push_str(&format!(
            "- Readiness entries: {} ({} ready, {} on a stale or diverged base)\n",
            self.readiness_entries.len(),
            ready_entries,
            stale_entries
        ));
        out.push_str(&format!(
            "- Stale-base invalidations: {}\n",
            self.stale_base_invalidations.len()
        ));
        out.push_str(&format!(
            "- Approval recomputations: {} ({} invalidated, reset, or re-requested)\n",
            self.approval_recomputations.len(),
            recomputed_away
        ));
        out.push_str(&format!(
            "- Proof freshness SLO: {} hours (last refresh: {})\n",
            self.proof_freshness.proof_freshness_slo_hours, self.proof_freshness.last_proof_refresh
        ));

        out.push_str("\n## Readiness\n\n");
        for row in &self.readiness_entries {
            out.push_str(&format!(
                "- **{}** (#{}) → anchor `{}`: base `{}`, verdict `{}`, authority `{}`\n",
                row.target_identity_label,
                row.queue_position,
                row.durable_anchor_id,
                row.base_freshness.as_str(),
                row.readiness_verdict.as_str(),
                row.mutation_authority.as_str()
            ));
        }

        out.push_str("\n## Stale-base invalidation\n\n");
        for row in &self.stale_base_invalidations {
            out.push_str(&format!(
                "- `{}`: {} (recompute required: {})\n",
                row.entry_id,
                row.invalidation_action.as_str(),
                row.recompute_required
            ));
        }

        out.push_str("\n## Approval recomputation\n\n");
        for row in &self.approval_recomputations {
            out.push_str(&format!(
                "- `{}` on {}: {} ({} → {} approvals)\n",
                row.entry_id,
                row.trigger.as_str(),
                row.outcome.as_str(),
                row.approvals_before,
                row.approvals_after
            ));
        }
        out
    }
}

/// Errors emitted when reading the checked-in merge-queue readiness export.
#[derive(Debug)]
pub enum MergeQueueReadinessArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<MergeQueueReadinessViolation>),
}

impl fmt::Display for MergeQueueReadinessArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(
                    formatter,
                    "merge-queue readiness export parse failed: {error}"
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
                    "merge-queue readiness export failed validation: {tokens}"
                )
            }
        }
    }
}

impl Error for MergeQueueReadinessArtifactError {}

/// Validation failures emitted by [`MergeQueueReadinessPacket::validate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MergeQueueReadinessViolation {
    /// Packet record kind is wrong.
    WrongRecordKind,
    /// Packet schema version is wrong.
    WrongSchemaVersion,
    /// Required identity field is missing.
    MissingIdentity,
    /// Source contract refs are incomplete.
    MissingSourceContracts,
    /// No readiness rows are present.
    ReadinessEntriesMissing,
    /// A readiness row is incomplete.
    ReadinessRowIncomplete,
    /// A non-ready entry is missing its blocking reasons.
    BlockingReasonMissing,
    /// A stale-base-blocked entry has no stale-base invalidation record.
    StaleBaseEntryMissingInvalidation,
    /// A readiness entry has no approval-recomputation record.
    EntryMissingApprovalRecomputation,
    /// No stale-base invalidation rows are present.
    StaleBaseInvalidationsMissing,
    /// A stale-base invalidation row is incomplete.
    StaleBaseInvalidationRowIncomplete,
    /// An invalidating action is missing its invalidation label.
    InvalidationLabelMissing,
    /// No approval-recomputation rows are present.
    ApprovalRecomputationsMissing,
    /// An approval-recomputation row is incomplete.
    ApprovalRecomputationRowIncomplete,
    /// A label-required recomputation outcome is missing its label.
    RecomputationLabelMissing,
    /// An invalidating recomputation outcome increased the approval count.
    ApprovalCountInconsistent,
    /// No downgrade triggers are present.
    DowngradeTriggersMissing,
    /// No consumer surfaces are present.
    ConsumerSurfacesMissing,
    /// Trust review does not satisfy required invariants.
    TrustReviewIncomplete,
    /// Consumer projection does not satisfy required invariants.
    ConsumerProjectionIncomplete,
    /// Proof freshness block is incomplete.
    ProofFreshnessIncomplete,
    /// Export contains raw boundary material.
    RawBoundaryMaterialInExport,
}

impl MergeQueueReadinessViolation {
    /// Stable token used in tests and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WrongRecordKind => "wrong_record_kind",
            Self::WrongSchemaVersion => "wrong_schema_version",
            Self::MissingIdentity => "missing_identity",
            Self::MissingSourceContracts => "missing_source_contracts",
            Self::ReadinessEntriesMissing => "readiness_entries_missing",
            Self::ReadinessRowIncomplete => "readiness_row_incomplete",
            Self::BlockingReasonMissing => "blocking_reason_missing",
            Self::StaleBaseEntryMissingInvalidation => "stale_base_entry_missing_invalidation",
            Self::EntryMissingApprovalRecomputation => "entry_missing_approval_recomputation",
            Self::StaleBaseInvalidationsMissing => "stale_base_invalidations_missing",
            Self::StaleBaseInvalidationRowIncomplete => "stale_base_invalidation_row_incomplete",
            Self::InvalidationLabelMissing => "invalidation_label_missing",
            Self::ApprovalRecomputationsMissing => "approval_recomputations_missing",
            Self::ApprovalRecomputationRowIncomplete => "approval_recomputation_row_incomplete",
            Self::RecomputationLabelMissing => "recomputation_label_missing",
            Self::ApprovalCountInconsistent => "approval_count_inconsistent",
            Self::DowngradeTriggersMissing => "downgrade_triggers_missing",
            Self::ConsumerSurfacesMissing => "consumer_surfaces_missing",
            Self::TrustReviewIncomplete => "trust_review_incomplete",
            Self::ConsumerProjectionIncomplete => "consumer_projection_incomplete",
            Self::ProofFreshnessIncomplete => "proof_freshness_incomplete",
            Self::RawBoundaryMaterialInExport => "raw_boundary_material_in_export",
        }
    }
}

/// Reads and validates the checked-in stable merge-queue readiness export.
pub fn current_merge_queue_readiness_export(
) -> Result<MergeQueueReadinessPacket, MergeQueueReadinessArtifactError> {
    let packet: MergeQueueReadinessPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/review/m5/add_merge_queue_readiness_stale_base_invalidation_and_approval_recomputation_flows/support_export.json"
    )))
    .map_err(MergeQueueReadinessArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(MergeQueueReadinessArtifactError::Validation(violations))
    }
}

fn validate_source_contracts(
    packet: &MergeQueueReadinessPacket,
    violations: &mut Vec<MergeQueueReadinessViolation>,
) {
    let refs: BTreeSet<&str> = packet
        .source_contract_refs
        .iter()
        .map(String::as_str)
        .collect();
    for required in [
        MERGE_QUEUE_READINESS_SCHEMA_REF,
        MERGE_QUEUE_READINESS_DOC_REF,
        MERGE_QUEUE_READINESS_MERGE_QUEUE_CONTRACT_REF,
        MERGE_QUEUE_READINESS_LANDING_CONTRACT_REF,
        MERGE_QUEUE_READINESS_ANCHOR_STABILITY_CONTRACT_REF,
        MERGE_QUEUE_READINESS_PIPELINE_RUN_CONTRACT_REF,
    ] {
        if !refs.contains(required) {
            violations.push(MergeQueueReadinessViolation::MissingSourceContracts);
            return;
        }
    }
}

fn validate_readiness_entries(
    packet: &MergeQueueReadinessPacket,
    violations: &mut Vec<MergeQueueReadinessViolation>,
) {
    if packet.readiness_entries.is_empty() {
        violations.push(MergeQueueReadinessViolation::ReadinessEntriesMissing);
        return;
    }

    let recomputed_entries: BTreeSet<&str> = packet
        .approval_recomputations
        .iter()
        .map(|row| row.entry_id.as_str())
        .collect();
    let invalidated_entries: BTreeSet<&str> = packet
        .stale_base_invalidations
        .iter()
        .map(|row| row.entry_id.as_str())
        .collect();

    for row in &packet.readiness_entries {
        if row.entry_id.trim().is_empty()
            || row.target_identity_label.trim().is_empty()
            || row.durable_anchor_id.trim().is_empty()
            || row.required_checks_summary.trim().is_empty()
            || row.required_approvals_summary.trim().is_empty()
            || row.source_contract_refs.is_empty()
        {
            violations.push(MergeQueueReadinessViolation::ReadinessRowIncomplete);
        }
        if row.readiness_verdict.requires_blocking_reason() && row.blocking_reasons.is_empty() {
            violations.push(MergeQueueReadinessViolation::BlockingReasonMissing);
        }
        let needs_invalidation = row.readiness_verdict.requires_stale_base_invalidation()
            || row.base_freshness.is_stale();
        if needs_invalidation
            && !row.entry_id.trim().is_empty()
            && !invalidated_entries.contains(row.entry_id.as_str())
        {
            violations.push(MergeQueueReadinessViolation::StaleBaseEntryMissingInvalidation);
        }
        if !row.entry_id.trim().is_empty() && !recomputed_entries.contains(row.entry_id.as_str()) {
            violations.push(MergeQueueReadinessViolation::EntryMissingApprovalRecomputation);
        }
    }
}

fn validate_stale_base_invalidations(
    packet: &MergeQueueReadinessPacket,
    violations: &mut Vec<MergeQueueReadinessViolation>,
) {
    if packet.stale_base_invalidations.is_empty() {
        violations.push(MergeQueueReadinessViolation::StaleBaseInvalidationsMissing);
        return;
    }

    for row in &packet.stale_base_invalidations {
        if row.entry_id.trim().is_empty() || row.base_advance_label.trim().is_empty() {
            violations.push(MergeQueueReadinessViolation::StaleBaseInvalidationRowIncomplete);
        }
        if row.invalidation_action.requires_label() && row.invalidation_label.trim().is_empty() {
            violations.push(MergeQueueReadinessViolation::InvalidationLabelMissing);
        }
    }
}

fn validate_approval_recomputations(
    packet: &MergeQueueReadinessPacket,
    violations: &mut Vec<MergeQueueReadinessViolation>,
) {
    if packet.approval_recomputations.is_empty() {
        violations.push(MergeQueueReadinessViolation::ApprovalRecomputationsMissing);
        return;
    }

    for row in &packet.approval_recomputations {
        if row.entry_id.trim().is_empty() {
            violations.push(MergeQueueReadinessViolation::ApprovalRecomputationRowIncomplete);
        }
        if row.outcome.requires_label() && row.recomputation_label.trim().is_empty() {
            violations.push(MergeQueueReadinessViolation::RecomputationLabelMissing);
        }
        if row.outcome.lowers_approval_count() && row.approvals_after > row.approvals_before {
            violations.push(MergeQueueReadinessViolation::ApprovalCountInconsistent);
        }
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
        }
        serde_json::Value::Array(arr) => arr.iter().any(json_contains_forbidden_boundary_material),
        serde_json::Value::Object(map) => {
            map.values().any(json_contains_forbidden_boundary_material)
        }
        _ => false,
    }
}
