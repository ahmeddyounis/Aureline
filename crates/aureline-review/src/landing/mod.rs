//! Landing-candidate and merge-queue truth for review workspaces.
//!
//! This module wraps the beta review-workspace packet with explicit
//! landing-candidate and merge-queue-entry objects so review surfaces can show
//! `mergeable`, `queue eligible`, `queued`, `stale base`, `checks stale`,
//! `policy blocked`, and `approval invalidated` as separable inspectable
//! truths. The data model never collapses provider-authoritative queue state,
//! repo-policy-managed queue state, and Aureline local estimates into one
//! ambiguous status, and landing is only possible from a reviewed candidate
//! object — never from ambient branch state.
//!
//! The records and command-graph operations are intentionally inert: they
//! describe *what* landing or queue mutation would do, never *cause* it.

use std::collections::BTreeSet;
use std::fmt;

use serde::{Deserialize, Serialize};

use crate::workspace::{
    ReviewWorkspaceBetaPacket, ReviewWorkspaceCheckFreshnessRecord, ReviewWorkspaceRecord,
};

/// Stable record-kind tag for [`LandingCandidatePacket`].
pub const LANDING_CANDIDATE_PACKET_RECORD_KIND: &str = "review_landing_candidate_packet";

/// Stable record-kind tag for [`LandingCandidateRecord`].
pub const LANDING_CANDIDATE_RECORD_KIND: &str = "review_landing_candidate_record";

/// Stable record-kind tag for [`MergeQueueEntryRecord`].
pub const MERGE_QUEUE_ENTRY_RECORD_KIND: &str = "review_merge_queue_entry_record";

/// Stable record-kind tag for [`LandingCommandRecord`].
pub const LANDING_COMMAND_RECORD_KIND: &str = "review_landing_command_record";

/// Stable record-kind tag for [`LandingInspectionRecord`].
pub const LANDING_INSPECTION_RECORD_KIND: &str = "review_landing_inspection_record";

/// Stable record-kind tag for [`LandingSupportExportPacket`].
pub const LANDING_SUPPORT_EXPORT_PACKET_RECORD_KIND: &str = "review_landing_support_export_packet";

/// Schema version for the landing-candidate packet.
pub const LANDING_CANDIDATE_SCHEMA_VERSION: u32 = 1;

/// Closed set of landing authority classes.
pub const LANDING_AUTHORITY_CLASSES: &[&str] = &[
    "provider_authoritative_queue_state",
    "repo_policy_managed_queue_state",
    "aureline_local_estimate_only",
];

/// Closed set of merge-strategy classes.
pub const LANDING_MERGE_STRATEGY_CLASSES: &[&str] = &[
    "rebase_then_fast_forward",
    "squash_then_fast_forward",
    "merge_commit",
    "fast_forward_only",
];

/// Closed set of inspectable mergeable states.
pub const LANDING_MERGEABLE_STATES: &[&str] = &[
    "mergeable",
    "not_mergeable_blocking",
    "mergeable_pending_eligibility",
];

/// Closed set of inspectable queue-eligibility states.
pub const LANDING_ELIGIBILITY_STATES: &[&str] = &["queue_eligible", "queue_not_eligible"];

/// Closed set of merge-queue lifecycle states observed by the review surface.
pub const MERGE_QUEUE_STATES: &[&str] = &[
    "not_queued",
    "queued",
    "dequeued_by_user",
    "dequeued_by_provider",
    "queued_invalidated_by_stale_base",
];

/// Closed set of stale-base states.
pub const LANDING_STALE_BASE_STATES: &[&str] = &[
    "base_current",
    "base_stale_within_grace",
    "base_stale_blocks_landing",
];

/// Closed set of checks-freshness states distinct from individual check status.
pub const LANDING_CHECKS_FRESHNESS_STATES: &[&str] = &[
    "checks_current",
    "checks_stale_within_grace",
    "checks_stale_blocks_landing",
];

/// Closed set of approval states.
pub const LANDING_APPROVAL_STATES: &[&str] = &[
    "approval_not_required_by_policy",
    "approval_required_outstanding",
    "approved_current",
    "approval_invalidated_by_changes",
];

/// Closed set of policy block states.
pub const LANDING_POLICY_BLOCK_STATES: &[&str] = &["policy_clear", "policy_blocked"];

/// Closed vocabulary of invalidation reasons that mark queue readiness stale.
pub const LANDING_INVALIDATION_REASONS: &[&str] = &[
    "stale_base",
    "checks_stale",
    "approval_invalidated",
    "policy_blocked",
    "review_pack_version_changed",
    "worktree_scope_changed",
    "environment_capsule_changed",
    "provider_overlay_stale",
    "queue_dequeued_by_provider",
];

/// Closed vocabulary of command-graph operations defined by the landing lane.
pub const LANDING_COMMAND_CLASSES: &[&str] = &[
    "enqueue",
    "dequeue",
    "approve",
    "request_changes",
    "rerun_pipeline",
    "publish_to_provider",
    "refresh_provider_overlay",
    "mark_review_pack_landed",
];

/// Closed posture vocabulary for provider publish operations.
pub const LANDING_PROVIDER_PUBLISH_POSTURES: &[&str] = &[
    "publish_minted_not_launched",
    "publish_in_flight",
    "publish_succeeded",
    "publish_failed",
    "publish_cancelled",
];

/// Closed set of blocking reasons surfaced when landing is not possible.
pub const LANDING_BLOCKED_REASONS: &[&str] = &[
    "target_branch_missing",
    "required_check_failed",
    "required_check_stale",
    "base_revision_stale",
    "approval_missing",
    "approval_invalidated",
    "policy_blocked",
    "review_pack_version_drift",
    "worktree_scope_changed",
    "environment_capsule_changed",
];

/// Closed set of consumer surfaces for the landing packet and support export.
pub const LANDING_CONSUMER_SURFACES: &[&str] = &[
    "review_workspace_inspector",
    "review_landing_strip",
    "merge_queue_inspector",
    "cli_headless_entry",
    "support_export",
    "browser_companion",
];

/// Input describing the landing candidate to materialize on top of a beta
/// review-workspace packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LandingCandidateInput {
    /// Stable landing candidate identity.
    pub landing_candidate_id: String,
    /// Stable packet identity.
    pub packet_id: String,
    /// Timestamp used for deterministic fixture output.
    pub generated_at: String,
    /// Target branch ref this candidate would land into.
    pub target_branch_ref: String,
    /// Base revision ref the candidate was reviewed against.
    pub base_revision_ref: String,
    /// Head revision ref of the candidate.
    pub head_revision_ref: String,
    /// Change-object ref linking the candidate to its change record.
    pub change_object_ref: String,
    /// Worktree-identity ref backing the change object.
    pub worktree_identity_ref: String,
    /// Review-pack digest pinned at review time.
    pub review_pack_digest_ref: String,
    /// Environment capsule digest pinned at review time.
    pub environment_capsule_digest_ref: String,
    /// Merge strategy class.
    pub merge_strategy_class: String,
    /// Landing authority class declared by the provider or repo policy.
    pub landing_authority_class: String,
    /// Mergeable truth as observed by the authority.
    pub mergeable_state: String,
    /// Stale-base truth observed at candidate freeze time.
    pub stale_base_state: String,
    /// Checks-freshness truth observed at candidate freeze time.
    pub checks_freshness_state: String,
    /// Approval truth observed at candidate freeze time.
    pub approval_state: String,
    /// Policy block truth observed at candidate freeze time.
    pub policy_block_state: String,
    /// Required-check identifiers from the workspace check-freshness rows.
    pub required_check_ids: Vec<String>,
    /// Active invalidation reasons; empty when none apply.
    #[serde(default)]
    pub invalidation_reasons: Vec<String>,
    /// Reviewable summary safe for support/export surfaces.
    pub summary_label: String,
    /// Optional merge-queue entry input.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub merge_queue_entry: Option<MergeQueueEntryInput>,
    /// Command-graph operations defined for this candidate.
    pub commands: Vec<LandingCommandInput>,
    /// Support/export envelope for this landing packet.
    pub support_export: LandingSupportExportInput,
}

/// Input describing the merge-queue entry attached to a landing candidate.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MergeQueueEntryInput {
    /// Stable merge-queue entry identity.
    pub merge_queue_entry_id: String,
    /// Queue authority class for the entry (provider, repo policy, local estimate).
    pub queue_authority_class: String,
    /// Queue lifecycle state.
    pub queue_state: String,
    /// Optional zero-based queue position; only present when authoritatively
    /// known.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub queue_position: Option<u32>,
    /// Optional queue length at observation time.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub queue_length: Option<u32>,
    /// Required-check identities the queue must observe pass before landing.
    pub required_check_ids: Vec<String>,
    /// Timestamp when the queue snapshot was captured.
    pub captured_at: String,
    /// Optional timestamp when the queue snapshot expires.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub expires_at: Option<String>,
    /// Active invalidation reasons specific to the queue entry.
    #[serde(default)]
    pub invalidation_reasons: Vec<String>,
    /// Reviewable summary safe for support/export surfaces.
    pub summary_label: String,
}

/// Input describing one command-graph operation defined for a landing
/// candidate.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LandingCommandInput {
    /// Stable command identity.
    pub command_id: String,
    /// Command class from the closed vocabulary.
    pub command_class: String,
    /// Target object ref the command would mutate.
    pub target_object_ref: String,
    /// Target object kind the command would mutate.
    pub target_object_kind: String,
    /// True when the command supports preview/dry-run.
    pub preview_supported: bool,
    /// True when the command emits an audit event when executed.
    pub emits_audit_event: bool,
    /// Provider publish posture when the command crosses the provider boundary.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub provider_publish_posture: Option<String>,
    /// Active blocked reasons preventing execution; empty when actionable.
    #[serde(default)]
    pub blocked_reasons: Vec<String>,
    /// Reviewable summary safe for support/export surfaces.
    pub summary_label: String,
}

/// Input row for the landing-candidate support/export packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LandingSupportExportInput {
    /// Stable support/export packet identity.
    pub support_export_id: String,
    /// Stable context ref used to reopen the landing candidate.
    pub reopen_context_ref: String,
    /// Command id used by CLI/headless or support tooling to reopen context.
    pub reopen_command_id_ref: String,
    /// Consumer surfaces that can read this support export.
    pub consumer_surfaces: Vec<String>,
    /// Redaction class applied to exported metadata.
    pub redaction_class: String,
    /// Reviewable summary safe for support/export surfaces.
    pub summary_label: String,
}

/// Landing-candidate record materialized from input plus workspace truth.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LandingCandidateRecord {
    /// Stable record-kind tag.
    pub record_kind: String,
    /// Integer schema version for the record.
    pub schema_version: u32,
    /// Stable landing candidate identity.
    pub landing_candidate_id: String,
    /// Review workspace this candidate belongs to.
    pub review_workspace_id_ref: String,
    /// Target branch ref.
    pub target_branch_ref: String,
    /// Base revision ref.
    pub base_revision_ref: String,
    /// Head revision ref.
    pub head_revision_ref: String,
    /// Change-object ref linking the candidate to its change record.
    pub change_object_ref: String,
    /// Worktree-identity ref backing the change object.
    pub worktree_identity_ref: String,
    /// Review-pack digest pinned at review time.
    pub review_pack_digest_ref: String,
    /// Environment capsule digest pinned at review time.
    pub environment_capsule_digest_ref: String,
    /// Merge strategy class.
    pub merge_strategy_class: String,
    /// Landing authority class.
    pub landing_authority_class: String,
    /// Mergeable truth.
    pub mergeable_state: String,
    /// Eligibility truth derived from blocking factors.
    pub eligibility_state: String,
    /// Stale-base truth.
    pub stale_base_state: String,
    /// Checks-freshness truth.
    pub checks_freshness_state: String,
    /// Approval truth.
    pub approval_state: String,
    /// Policy block truth.
    pub policy_block_state: String,
    /// Required-check identifiers carried from the workspace.
    pub required_check_ids: Vec<String>,
    /// Active invalidation reasons; empty when none apply.
    pub invalidation_reasons: Vec<String>,
    /// Active blocking reasons preventing landing; empty when landing is
    /// possible.
    pub blocked_reasons: Vec<String>,
    /// True when only an authoritative landing candidate may land. Always true
    /// in this beta to enforce the explicit landing requirement.
    pub landing_requires_explicit_candidate: bool,
    /// Timestamp the candidate was frozen.
    pub generated_at: String,
    /// Reviewable summary safe for support/export surfaces.
    pub summary_label: String,
}

/// Merge-queue entry record.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MergeQueueEntryRecord {
    /// Stable record-kind tag.
    pub record_kind: String,
    /// Integer schema version for the record.
    pub schema_version: u32,
    /// Stable merge-queue entry identity.
    pub merge_queue_entry_id: String,
    /// Landing candidate this entry belongs to.
    pub landing_candidate_id_ref: String,
    /// Review workspace this entry belongs to.
    pub review_workspace_id_ref: String,
    /// Queue authority class.
    pub queue_authority_class: String,
    /// Queue lifecycle state.
    pub queue_state: String,
    /// Optional zero-based queue position.
    pub queue_position: Option<u32>,
    /// Optional queue length at observation time.
    pub queue_length: Option<u32>,
    /// Required-check identifiers tracked by the queue.
    pub required_check_ids: Vec<String>,
    /// Capture timestamp.
    pub captured_at: String,
    /// Optional expiry timestamp.
    pub expires_at: Option<String>,
    /// Active invalidation reasons specific to the queue entry.
    pub invalidation_reasons: Vec<String>,
    /// True when the entry is the sole authoritative state for queue position.
    pub authoritative_position: bool,
    /// True when the recorded position is a local estimate only.
    pub local_estimate_only: bool,
    /// Reviewable summary safe for support/export surfaces.
    pub summary_label: String,
}

/// Command-graph operation record.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LandingCommandRecord {
    /// Stable record-kind tag.
    pub record_kind: String,
    /// Integer schema version for the record.
    pub schema_version: u32,
    /// Stable command identity.
    pub command_id: String,
    /// Landing candidate this command belongs to.
    pub landing_candidate_id_ref: String,
    /// Command class.
    pub command_class: String,
    /// Target object ref.
    pub target_object_ref: String,
    /// Target object kind.
    pub target_object_kind: String,
    /// True when preview/dry-run is supported.
    pub preview_supported: bool,
    /// True when the command emits an audit event when executed.
    pub emits_audit_event: bool,
    /// Optional provider publish posture for commands that cross the boundary.
    pub provider_publish_posture: Option<String>,
    /// Active blocked reasons preventing execution.
    pub blocked_reasons: Vec<String>,
    /// True when the command is actionable from the current candidate state.
    pub actionable: bool,
    /// Reviewable summary safe for support/export surfaces.
    pub summary_label: String,
}

/// Inspection row used by support/export and inspector surfaces.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LandingInspectionRecord {
    /// Stable record-kind tag.
    pub record_kind: String,
    /// Integer schema version for the record.
    pub schema_version: u32,
    /// Landing candidate inspected by this row.
    pub landing_candidate_id_ref: String,
    /// Review workspace inspected by this row.
    pub review_workspace_id_ref: String,
    /// True when the candidate is mergeable.
    pub mergeable: bool,
    /// True when the candidate is queue eligible.
    pub queue_eligible: bool,
    /// True when the candidate is queued.
    pub queued: bool,
    /// True when the base is stale and blocks landing.
    pub stale_base_blocks_landing: bool,
    /// True when one or more checks are stale enough to block landing.
    pub checks_stale_blocks_landing: bool,
    /// True when policy currently blocks landing.
    pub policy_blocks_landing: bool,
    /// True when approval is invalidated by recent changes.
    pub approval_invalidated: bool,
    /// True when the candidate's authority is provider-backed.
    pub provider_authoritative: bool,
    /// True when the queue state is a local estimate only.
    pub queue_state_is_local_estimate_only: bool,
    /// True when the candidate is currently invalidated by any reason.
    pub candidate_invalidated: bool,
    /// True when landing requires an explicit reviewed candidate.
    pub landing_requires_explicit_candidate: bool,
    /// Number of command-graph operations attached.
    pub command_count: usize,
    /// Number of required checks tracked by the candidate.
    pub required_check_count: usize,
    /// True when at least one command supports preview/dry-run.
    pub preview_capable: bool,
    /// True when support/export can reopen the candidate context.
    pub support_export_reopenable: bool,
    /// Reviewable summary safe for support/export surfaces.
    pub summary_label: String,
}

/// Support/export packet for the landing-candidate lane.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LandingSupportExportPacket {
    /// Stable record-kind tag.
    pub record_kind: String,
    /// Integer schema version for the record.
    pub schema_version: u32,
    /// Stable support/export packet identity.
    pub support_export_id: String,
    /// Landing candidate this packet exports.
    pub landing_candidate_id_ref: String,
    /// Review workspace this packet exports.
    pub review_workspace_id_ref: String,
    /// Stable context ref used to reopen the candidate.
    pub reopen_context_ref: String,
    /// Command id used by CLI/headless or support tooling to reopen context.
    pub reopen_command_id_ref: String,
    /// Command ids exported in this packet.
    pub command_id_refs: Vec<String>,
    /// Merge-queue entry ref, when present.
    pub merge_queue_entry_ref: Option<String>,
    /// Consumer surfaces that can read this support export.
    pub consumer_surfaces: Vec<String>,
    /// Source schemas the export cites.
    pub source_schema_refs: Vec<String>,
    /// False so raw URLs cannot cross the support boundary.
    pub raw_url_export_allowed: bool,
    /// False so raw provider payloads cannot cross the support boundary.
    pub raw_provider_payload_export_allowed: bool,
    /// Redaction class applied to exported metadata.
    pub redaction_class: String,
    /// Captured eligibility snapshot used by support to reconstruct truth.
    pub eligibility_snapshot: LandingEligibilitySnapshot,
    /// Reviewable summary safe for support/export surfaces.
    pub summary_label: String,
}

/// Eligibility snapshot embedded in the landing support-export packet.
///
/// Carries every separable truth surfaced by the inspector so support packets
/// can reconstruct why a change was or was not queue eligible at the time of
/// user action without consulting hosted state.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LandingEligibilitySnapshot {
    /// Mergeable state at export time.
    pub mergeable_state: String,
    /// Eligibility state at export time.
    pub eligibility_state: String,
    /// Queue state at export time.
    pub queue_state: String,
    /// Stale-base state at export time.
    pub stale_base_state: String,
    /// Checks-freshness state at export time.
    pub checks_freshness_state: String,
    /// Approval state at export time.
    pub approval_state: String,
    /// Policy block state at export time.
    pub policy_block_state: String,
    /// Active invalidation reasons at export time.
    pub invalidation_reasons: Vec<String>,
    /// Active blocked reasons at export time.
    pub blocked_reasons: Vec<String>,
    /// Landing authority class at export time.
    pub landing_authority_class: String,
    /// Queue authority class at export time, when an entry exists.
    pub queue_authority_class: Option<String>,
}

/// Landing-candidate packet consumed by review surfaces and support exports.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LandingCandidatePacket {
    /// Stable record-kind tag.
    pub record_kind: String,
    /// Integer schema version for the packet.
    pub schema_version: u32,
    /// Stable packet identity.
    pub packet_id: String,
    /// Timestamp used for deterministic fixture output.
    pub generated_at: String,
    /// Review workspace summary copied from the beta packet.
    pub review_workspace: ReviewWorkspaceRecord,
    /// Landing candidate record.
    pub landing_candidate: LandingCandidateRecord,
    /// Optional merge-queue entry record.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub merge_queue_entry: Option<MergeQueueEntryRecord>,
    /// Command-graph operation records.
    pub commands: Vec<LandingCommandRecord>,
    /// Support/export packet.
    pub support_export: LandingSupportExportPacket,
    /// Inspection row.
    pub inspection: LandingInspectionRecord,
}

impl LandingCandidatePacket {
    /// Builds a landing-candidate packet from a beta review-workspace packet
    /// and an input row.
    ///
    /// # Errors
    ///
    /// Returns [`LandingCandidateValidationError`] when the input violates a
    /// landing-candidate invariant.
    pub fn from_workspace_packet(
        input: LandingCandidateInput,
        workspace_packet: &ReviewWorkspaceBetaPacket,
    ) -> Result<Self, LandingCandidateValidationError> {
        validate_input(&input, workspace_packet)?;

        let landing_candidate = landing_candidate_record(&input, workspace_packet);
        let merge_queue_entry = input
            .merge_queue_entry
            .as_ref()
            .map(|entry| merge_queue_entry_record(entry, &landing_candidate, workspace_packet));
        let commands = input
            .commands
            .iter()
            .map(|command| landing_command_record(command, &landing_candidate))
            .collect::<Vec<_>>();
        let support_export = landing_support_export_packet(
            &input.support_export,
            &landing_candidate,
            workspace_packet,
            &commands,
            merge_queue_entry.as_ref(),
        );
        let inspection = landing_inspection_record(
            &landing_candidate,
            merge_queue_entry.as_ref(),
            &commands,
            &support_export,
        );

        let packet = Self {
            record_kind: LANDING_CANDIDATE_PACKET_RECORD_KIND.to_string(),
            schema_version: LANDING_CANDIDATE_SCHEMA_VERSION,
            packet_id: input.packet_id,
            generated_at: input.generated_at,
            review_workspace: workspace_packet.review_workspace.clone(),
            landing_candidate,
            merge_queue_entry,
            commands,
            support_export,
            inspection,
        };
        packet.validate()?;
        Ok(packet)
    }

    /// Validates this packet against the landing-candidate invariants.
    ///
    /// # Errors
    ///
    /// Returns [`LandingCandidateValidationError`] when an invariant is
    /// violated.
    pub fn validate(&self) -> Result<(), LandingCandidateValidationError> {
        ensure_eq(
            self.record_kind.as_str(),
            LANDING_CANDIDATE_PACKET_RECORD_KIND,
            "record_kind",
        )?;
        ensure_eq(
            self.schema_version,
            LANDING_CANDIDATE_SCHEMA_VERSION,
            "schema_version",
        )?;
        ensure_nonempty(&self.packet_id, "packet_id")?;
        ensure_nonempty(&self.generated_at, "generated_at")?;

        validate_candidate_record(
            &self.landing_candidate,
            &self.review_workspace.review_workspace_id,
        )?;
        if let Some(entry) = &self.merge_queue_entry {
            validate_queue_entry_record(
                entry,
                &self.landing_candidate.landing_candidate_id,
                &self.review_workspace.review_workspace_id,
            )?;
        } else if self.landing_candidate.eligibility_state == "queue_eligible"
            && self.landing_candidate.landing_authority_class != "aureline_local_estimate_only"
        {
            // Provider- or repo-authoritative candidates that claim eligibility
            // must surface a queue entry. Local estimates may omit the entry.
            return Err(landing_validation_error(
                "landing_candidate marked queue_eligible under non-local authority must include a merge_queue_entry",
            ));
        }
        for command in &self.commands {
            validate_command_record(command, &self.landing_candidate.landing_candidate_id)?;
        }
        validate_support_export(
            &self.support_export,
            &self.landing_candidate,
            &self.commands,
            self.merge_queue_entry.as_ref(),
        )?;
        validate_inspection(&self.inspection, self)?;
        Ok(())
    }

    /// Returns true when landing-truth axes are surfaced as separable
    /// inspectable truths.
    pub fn truths_are_separable(&self) -> bool {
        let candidate = &self.landing_candidate;
        contains_token(LANDING_MERGEABLE_STATES, &candidate.mergeable_state)
            && contains_token(LANDING_ELIGIBILITY_STATES, &candidate.eligibility_state)
            && contains_token(LANDING_STALE_BASE_STATES, &candidate.stale_base_state)
            && contains_token(
                LANDING_CHECKS_FRESHNESS_STATES,
                &candidate.checks_freshness_state,
            )
            && contains_token(LANDING_APPROVAL_STATES, &candidate.approval_state)
            && contains_token(LANDING_POLICY_BLOCK_STATES, &candidate.policy_block_state)
            && contains_token(
                LANDING_AUTHORITY_CLASSES,
                &candidate.landing_authority_class,
            )
    }

    /// Returns true when landing is only possible from this reviewed candidate.
    pub fn landing_requires_explicit_candidate(&self) -> bool {
        self.landing_candidate.landing_requires_explicit_candidate
    }

    /// Returns true when no raw escape hatch crosses the support boundary.
    pub fn raw_escape_hatches_absent(&self) -> bool {
        !self.support_export.raw_url_export_allowed
            && !self.support_export.raw_provider_payload_export_allowed
    }
}

/// Compact projection consumed by CLI/headless and inspector surfaces.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LandingCandidateProjection {
    /// Stable packet identity.
    pub packet_id: String,
    /// Landing candidate identity.
    pub landing_candidate_id: String,
    /// Review workspace identity.
    pub review_workspace_id: String,
    /// Landing authority class.
    pub landing_authority_class: String,
    /// Mergeable state.
    pub mergeable_state: String,
    /// Eligibility state.
    pub eligibility_state: String,
    /// Queue state ("not_queued" when no entry is present).
    pub queue_state: String,
    /// Stale-base state.
    pub stale_base_state: String,
    /// Checks-freshness state.
    pub checks_freshness_state: String,
    /// Approval state.
    pub approval_state: String,
    /// Policy block state.
    pub policy_block_state: String,
    /// Active invalidation reasons.
    pub invalidation_reasons: Vec<String>,
    /// Active blocked reasons.
    pub blocked_reasons: Vec<String>,
    /// Command count.
    pub command_count: usize,
    /// True when landing requires an explicit candidate.
    pub landing_requires_explicit_candidate: bool,
    /// True when support/export can reopen the candidate context.
    pub support_export_reopenable: bool,
    /// Consumer surfaces wired through the support export.
    pub consumer_surfaces: Vec<String>,
}

/// Parses and validates a materialized landing-candidate packet.
///
/// # Errors
///
/// Returns [`LandingCandidateError`] when the payload fails to parse or
/// violates the landing-candidate invariants.
pub fn project_landing_candidate_packet(
    payload: &str,
) -> Result<LandingCandidateProjection, LandingCandidateError> {
    let packet: LandingCandidatePacket = serde_json::from_str(payload)?;
    packet.validate()?;
    Ok(LandingCandidateProjection::from(packet))
}

impl From<LandingCandidatePacket> for LandingCandidateProjection {
    fn from(packet: LandingCandidatePacket) -> Self {
        let queue_state = packet
            .merge_queue_entry
            .as_ref()
            .map(|entry| entry.queue_state.clone())
            .unwrap_or_else(|| "not_queued".to_string());
        Self {
            packet_id: packet.packet_id,
            landing_candidate_id: packet.landing_candidate.landing_candidate_id,
            review_workspace_id: packet.review_workspace.review_workspace_id,
            landing_authority_class: packet.landing_candidate.landing_authority_class,
            mergeable_state: packet.landing_candidate.mergeable_state,
            eligibility_state: packet.landing_candidate.eligibility_state,
            queue_state,
            stale_base_state: packet.landing_candidate.stale_base_state,
            checks_freshness_state: packet.landing_candidate.checks_freshness_state,
            approval_state: packet.landing_candidate.approval_state,
            policy_block_state: packet.landing_candidate.policy_block_state,
            invalidation_reasons: packet.landing_candidate.invalidation_reasons,
            blocked_reasons: packet.landing_candidate.blocked_reasons,
            command_count: packet.commands.len(),
            landing_requires_explicit_candidate: packet
                .inspection
                .landing_requires_explicit_candidate,
            support_export_reopenable: packet.inspection.support_export_reopenable,
            consumer_surfaces: packet.support_export.consumer_surfaces,
        }
    }
}

/// Error returned when a landing-candidate payload cannot be projected.
#[derive(Debug)]
pub enum LandingCandidateError {
    /// The payload failed JSON parsing.
    Parse(serde_json::Error),
    /// The payload parsed but violated the landing-candidate invariants.
    Validation(LandingCandidateValidationError),
}

impl fmt::Display for LandingCandidateError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Parse(err) => write!(formatter, "landing candidate parse error: {err}"),
            Self::Validation(err) => write!(formatter, "landing candidate validation error: {err}"),
        }
    }
}

impl std::error::Error for LandingCandidateError {}

impl From<serde_json::Error> for LandingCandidateError {
    fn from(err: serde_json::Error) -> Self {
        Self::Parse(err)
    }
}

impl From<LandingCandidateValidationError> for LandingCandidateError {
    fn from(err: LandingCandidateValidationError) -> Self {
        Self::Validation(err)
    }
}

/// Validation failure for landing-candidate packets.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LandingCandidateValidationError {
    message: String,
}

impl LandingCandidateValidationError {
    /// Returns the validation failure message.
    pub fn message(&self) -> &str {
        &self.message
    }
}

impl fmt::Display for LandingCandidateValidationError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.message)
    }
}

impl std::error::Error for LandingCandidateValidationError {}

fn landing_candidate_record(
    input: &LandingCandidateInput,
    workspace_packet: &ReviewWorkspaceBetaPacket,
) -> LandingCandidateRecord {
    let mut invalidation_reasons = input.invalidation_reasons.clone();
    invalidation_reasons.extend(derive_invalidation_reasons(
        &input.stale_base_state,
        &input.checks_freshness_state,
        &input.approval_state,
        &input.policy_block_state,
        &workspace_packet.check_freshness,
    ));
    invalidation_reasons.sort();
    invalidation_reasons.dedup();

    let blocked_reasons = derive_blocked_reasons(
        &input.target_branch_ref,
        &input.stale_base_state,
        &input.checks_freshness_state,
        &input.approval_state,
        &input.policy_block_state,
        &invalidation_reasons,
    );

    let eligibility_state = if blocked_reasons.is_empty() {
        "queue_eligible".to_string()
    } else {
        "queue_not_eligible".to_string()
    };

    LandingCandidateRecord {
        record_kind: LANDING_CANDIDATE_RECORD_KIND.to_string(),
        schema_version: LANDING_CANDIDATE_SCHEMA_VERSION,
        landing_candidate_id: input.landing_candidate_id.clone(),
        review_workspace_id_ref: workspace_packet
            .review_workspace
            .review_workspace_id
            .clone(),
        target_branch_ref: input.target_branch_ref.clone(),
        base_revision_ref: input.base_revision_ref.clone(),
        head_revision_ref: input.head_revision_ref.clone(),
        change_object_ref: input.change_object_ref.clone(),
        worktree_identity_ref: input.worktree_identity_ref.clone(),
        review_pack_digest_ref: input.review_pack_digest_ref.clone(),
        environment_capsule_digest_ref: input.environment_capsule_digest_ref.clone(),
        merge_strategy_class: input.merge_strategy_class.clone(),
        landing_authority_class: input.landing_authority_class.clone(),
        mergeable_state: input.mergeable_state.clone(),
        eligibility_state,
        stale_base_state: input.stale_base_state.clone(),
        checks_freshness_state: input.checks_freshness_state.clone(),
        approval_state: input.approval_state.clone(),
        policy_block_state: input.policy_block_state.clone(),
        required_check_ids: input.required_check_ids.clone(),
        invalidation_reasons,
        blocked_reasons,
        landing_requires_explicit_candidate: true,
        generated_at: input.generated_at.clone(),
        summary_label: input.summary_label.clone(),
    }
}

fn merge_queue_entry_record(
    input: &MergeQueueEntryInput,
    candidate: &LandingCandidateRecord,
    workspace_packet: &ReviewWorkspaceBetaPacket,
) -> MergeQueueEntryRecord {
    let local_estimate_only = input.queue_authority_class == "aureline_local_estimate_only";
    let authoritative_position = !local_estimate_only
        && matches!(
            input.queue_authority_class.as_str(),
            "provider_authoritative_queue_state" | "repo_policy_managed_queue_state"
        )
        && input.queue_position.is_some();
    MergeQueueEntryRecord {
        record_kind: MERGE_QUEUE_ENTRY_RECORD_KIND.to_string(),
        schema_version: LANDING_CANDIDATE_SCHEMA_VERSION,
        merge_queue_entry_id: input.merge_queue_entry_id.clone(),
        landing_candidate_id_ref: candidate.landing_candidate_id.clone(),
        review_workspace_id_ref: workspace_packet
            .review_workspace
            .review_workspace_id
            .clone(),
        queue_authority_class: input.queue_authority_class.clone(),
        queue_state: input.queue_state.clone(),
        queue_position: input.queue_position,
        queue_length: input.queue_length,
        required_check_ids: input.required_check_ids.clone(),
        captured_at: input.captured_at.clone(),
        expires_at: input.expires_at.clone(),
        invalidation_reasons: input.invalidation_reasons.clone(),
        authoritative_position,
        local_estimate_only,
        summary_label: input.summary_label.clone(),
    }
}

fn landing_command_record(
    input: &LandingCommandInput,
    candidate: &LandingCandidateRecord,
) -> LandingCommandRecord {
    LandingCommandRecord {
        record_kind: LANDING_COMMAND_RECORD_KIND.to_string(),
        schema_version: LANDING_CANDIDATE_SCHEMA_VERSION,
        command_id: input.command_id.clone(),
        landing_candidate_id_ref: candidate.landing_candidate_id.clone(),
        command_class: input.command_class.clone(),
        target_object_ref: input.target_object_ref.clone(),
        target_object_kind: input.target_object_kind.clone(),
        preview_supported: input.preview_supported,
        emits_audit_event: input.emits_audit_event,
        provider_publish_posture: input.provider_publish_posture.clone(),
        blocked_reasons: input.blocked_reasons.clone(),
        actionable: input.blocked_reasons.is_empty(),
        summary_label: input.summary_label.clone(),
    }
}

fn landing_support_export_packet(
    input: &LandingSupportExportInput,
    candidate: &LandingCandidateRecord,
    workspace_packet: &ReviewWorkspaceBetaPacket,
    commands: &[LandingCommandRecord],
    merge_queue_entry: Option<&MergeQueueEntryRecord>,
) -> LandingSupportExportPacket {
    LandingSupportExportPacket {
        record_kind: LANDING_SUPPORT_EXPORT_PACKET_RECORD_KIND.to_string(),
        schema_version: LANDING_CANDIDATE_SCHEMA_VERSION,
        support_export_id: input.support_export_id.clone(),
        landing_candidate_id_ref: candidate.landing_candidate_id.clone(),
        review_workspace_id_ref: workspace_packet
            .review_workspace
            .review_workspace_id
            .clone(),
        reopen_context_ref: input.reopen_context_ref.clone(),
        reopen_command_id_ref: input.reopen_command_id_ref.clone(),
        command_id_refs: commands
            .iter()
            .map(|command| command.command_id.clone())
            .collect(),
        merge_queue_entry_ref: merge_queue_entry.map(|entry| entry.merge_queue_entry_id.clone()),
        consumer_surfaces: input.consumer_surfaces.clone(),
        source_schema_refs: vec![
            "schemas/review/landing_candidate.schema.json".to_string(),
            "schemas/review/merge_queue_entry.schema.json".to_string(),
            "schemas/review/review_workspace.schema.json".to_string(),
        ],
        raw_url_export_allowed: false,
        raw_provider_payload_export_allowed: false,
        redaction_class: input.redaction_class.clone(),
        eligibility_snapshot: LandingEligibilitySnapshot {
            mergeable_state: candidate.mergeable_state.clone(),
            eligibility_state: candidate.eligibility_state.clone(),
            queue_state: merge_queue_entry
                .map(|entry| entry.queue_state.clone())
                .unwrap_or_else(|| "not_queued".to_string()),
            stale_base_state: candidate.stale_base_state.clone(),
            checks_freshness_state: candidate.checks_freshness_state.clone(),
            approval_state: candidate.approval_state.clone(),
            policy_block_state: candidate.policy_block_state.clone(),
            invalidation_reasons: candidate.invalidation_reasons.clone(),
            blocked_reasons: candidate.blocked_reasons.clone(),
            landing_authority_class: candidate.landing_authority_class.clone(),
            queue_authority_class: merge_queue_entry
                .map(|entry| entry.queue_authority_class.clone()),
        },
        summary_label: input.summary_label.clone(),
    }
}

fn landing_inspection_record(
    candidate: &LandingCandidateRecord,
    merge_queue_entry: Option<&MergeQueueEntryRecord>,
    commands: &[LandingCommandRecord],
    support_export: &LandingSupportExportPacket,
) -> LandingInspectionRecord {
    let mergeable = candidate.mergeable_state == "mergeable"
        || candidate.mergeable_state == "mergeable_pending_eligibility";
    let queue_eligible = candidate.eligibility_state == "queue_eligible";
    let queued = matches!(
        merge_queue_entry.map(|entry| entry.queue_state.as_str()),
        Some("queued")
    );
    let stale_base_blocks_landing = candidate.stale_base_state == "base_stale_blocks_landing";
    let checks_stale_blocks_landing =
        candidate.checks_freshness_state == "checks_stale_blocks_landing";
    let policy_blocks_landing = candidate.policy_block_state == "policy_blocked";
    let approval_invalidated = candidate.approval_state == "approval_invalidated_by_changes";
    let provider_authoritative =
        candidate.landing_authority_class == "provider_authoritative_queue_state";
    let queue_state_is_local_estimate_only = merge_queue_entry
        .map(|entry| entry.local_estimate_only)
        .unwrap_or(false);
    let candidate_invalidated = !candidate.invalidation_reasons.is_empty();
    let preview_capable = commands.iter().any(|command| command.preview_supported);
    let support_export_reopenable = support_export_can_reopen(support_export, commands);

    LandingInspectionRecord {
        record_kind: LANDING_INSPECTION_RECORD_KIND.to_string(),
        schema_version: LANDING_CANDIDATE_SCHEMA_VERSION,
        landing_candidate_id_ref: candidate.landing_candidate_id.clone(),
        review_workspace_id_ref: candidate.review_workspace_id_ref.clone(),
        mergeable,
        queue_eligible,
        queued,
        stale_base_blocks_landing,
        checks_stale_blocks_landing,
        policy_blocks_landing,
        approval_invalidated,
        provider_authoritative,
        queue_state_is_local_estimate_only,
        candidate_invalidated,
        landing_requires_explicit_candidate: candidate.landing_requires_explicit_candidate,
        command_count: commands.len(),
        required_check_count: candidate.required_check_ids.len(),
        preview_capable,
        support_export_reopenable,
        summary_label: format!(
            "Landing candidate {} ({} command(s))",
            candidate.landing_candidate_id,
            commands.len()
        ),
    }
}

fn support_export_can_reopen(
    export: &LandingSupportExportPacket,
    commands: &[LandingCommandRecord],
) -> bool {
    !export.reopen_context_ref.trim().is_empty()
        && !export.reopen_command_id_ref.trim().is_empty()
        && !export.raw_url_export_allowed
        && !export.raw_provider_payload_export_allowed
        && !commands.is_empty()
        && export
            .consumer_surfaces
            .iter()
            .any(|surface| surface == "support_export")
}

fn validate_input(
    input: &LandingCandidateInput,
    workspace_packet: &ReviewWorkspaceBetaPacket,
) -> Result<(), LandingCandidateValidationError> {
    ensure_nonempty(&input.landing_candidate_id, "landing_candidate_id")?;
    ensure_nonempty(&input.packet_id, "packet_id")?;
    ensure_nonempty(&input.generated_at, "generated_at")?;
    ensure_nonempty(&input.target_branch_ref, "target_branch_ref")?;
    ensure_nonempty(&input.base_revision_ref, "base_revision_ref")?;
    ensure_nonempty(&input.head_revision_ref, "head_revision_ref")?;
    ensure_nonempty(&input.change_object_ref, "change_object_ref")?;
    ensure_nonempty(&input.worktree_identity_ref, "worktree_identity_ref")?;
    ensure_nonempty(&input.review_pack_digest_ref, "review_pack_digest_ref")?;
    ensure_nonempty(
        &input.environment_capsule_digest_ref,
        "environment_capsule_digest_ref",
    )?;
    ensure_token(
        LANDING_MERGE_STRATEGY_CLASSES,
        &input.merge_strategy_class,
        "merge_strategy_class",
    )?;
    ensure_token(
        LANDING_AUTHORITY_CLASSES,
        &input.landing_authority_class,
        "landing_authority_class",
    )?;
    ensure_token(
        LANDING_MERGEABLE_STATES,
        &input.mergeable_state,
        "mergeable_state",
    )?;
    ensure_token(
        LANDING_STALE_BASE_STATES,
        &input.stale_base_state,
        "stale_base_state",
    )?;
    ensure_token(
        LANDING_CHECKS_FRESHNESS_STATES,
        &input.checks_freshness_state,
        "checks_freshness_state",
    )?;
    ensure_token(
        LANDING_APPROVAL_STATES,
        &input.approval_state,
        "approval_state",
    )?;
    ensure_token(
        LANDING_POLICY_BLOCK_STATES,
        &input.policy_block_state,
        "policy_block_state",
    )?;
    for reason in &input.invalidation_reasons {
        ensure_token(LANDING_INVALIDATION_REASONS, reason, "invalidation_reasons")?;
    }
    if input.commands.is_empty() {
        return Err(landing_validation_error(
            "landing candidates must define at least one command-graph operation",
        ));
    }
    for command in &input.commands {
        validate_command_input(command)?;
    }
    if let Some(entry) = &input.merge_queue_entry {
        validate_queue_entry_input(entry)?;
    }
    validate_support_export_input(&input.support_export)?;

    let workspace_check_ids = workspace_packet
        .check_freshness
        .iter()
        .map(|check| check.check_id.as_str())
        .collect::<BTreeSet<_>>();
    for check_id in &input.required_check_ids {
        if !workspace_check_ids.contains(check_id.as_str()) {
            return Err(landing_validation_error(format!(
                "required_check_id '{check_id}' is not present in the review workspace",
            )));
        }
    }
    Ok(())
}

fn validate_candidate_record(
    candidate: &LandingCandidateRecord,
    workspace_id: &str,
) -> Result<(), LandingCandidateValidationError> {
    ensure_eq(
        candidate.record_kind.as_str(),
        LANDING_CANDIDATE_RECORD_KIND,
        "landing_candidate.record_kind",
    )?;
    ensure_eq(
        candidate.schema_version,
        LANDING_CANDIDATE_SCHEMA_VERSION,
        "landing_candidate.schema_version",
    )?;
    ensure_eq(
        candidate.review_workspace_id_ref.as_str(),
        workspace_id,
        "landing_candidate.review_workspace_id_ref",
    )?;
    ensure_token(
        LANDING_MERGE_STRATEGY_CLASSES,
        &candidate.merge_strategy_class,
        "landing_candidate.merge_strategy_class",
    )?;
    ensure_token(
        LANDING_AUTHORITY_CLASSES,
        &candidate.landing_authority_class,
        "landing_candidate.landing_authority_class",
    )?;
    ensure_token(
        LANDING_MERGEABLE_STATES,
        &candidate.mergeable_state,
        "landing_candidate.mergeable_state",
    )?;
    ensure_token(
        LANDING_ELIGIBILITY_STATES,
        &candidate.eligibility_state,
        "landing_candidate.eligibility_state",
    )?;
    ensure_token(
        LANDING_STALE_BASE_STATES,
        &candidate.stale_base_state,
        "landing_candidate.stale_base_state",
    )?;
    ensure_token(
        LANDING_CHECKS_FRESHNESS_STATES,
        &candidate.checks_freshness_state,
        "landing_candidate.checks_freshness_state",
    )?;
    ensure_token(
        LANDING_APPROVAL_STATES,
        &candidate.approval_state,
        "landing_candidate.approval_state",
    )?;
    ensure_token(
        LANDING_POLICY_BLOCK_STATES,
        &candidate.policy_block_state,
        "landing_candidate.policy_block_state",
    )?;
    for reason in &candidate.invalidation_reasons {
        ensure_token(
            LANDING_INVALIDATION_REASONS,
            reason,
            "landing_candidate.invalidation_reasons",
        )?;
    }
    for reason in &candidate.blocked_reasons {
        ensure_token(
            LANDING_BLOCKED_REASONS,
            reason,
            "landing_candidate.blocked_reasons",
        )?;
    }
    if !candidate.landing_requires_explicit_candidate {
        return Err(landing_validation_error(
            "landing_candidate.landing_requires_explicit_candidate must be true",
        ));
    }
    let derived_eligibility = if candidate.blocked_reasons.is_empty() {
        "queue_eligible"
    } else {
        "queue_not_eligible"
    };
    if candidate.eligibility_state != derived_eligibility {
        return Err(landing_validation_error(format!(
            "landing_candidate.eligibility_state '{}' must match blocked reasons '{}'",
            candidate.eligibility_state, derived_eligibility,
        )));
    }
    Ok(())
}

fn validate_queue_entry_input(
    entry: &MergeQueueEntryInput,
) -> Result<(), LandingCandidateValidationError> {
    ensure_nonempty(&entry.merge_queue_entry_id, "merge_queue_entry_id")?;
    ensure_token(
        LANDING_AUTHORITY_CLASSES,
        &entry.queue_authority_class,
        "merge_queue_entry.queue_authority_class",
    )?;
    ensure_token(
        MERGE_QUEUE_STATES,
        &entry.queue_state,
        "merge_queue_entry.queue_state",
    )?;
    for reason in &entry.invalidation_reasons {
        ensure_token(
            LANDING_INVALIDATION_REASONS,
            reason,
            "merge_queue_entry.invalidation_reasons",
        )?;
    }
    if entry.queue_authority_class == "aureline_local_estimate_only"
        && entry.queue_position.is_some()
    {
        return Err(landing_validation_error(
            "aureline_local_estimate_only entries must not claim an authoritative queue_position",
        ));
    }
    if entry.queue_state == "queued"
        && entry.queue_authority_class != "aureline_local_estimate_only"
        && entry.queue_position.is_none()
    {
        return Err(landing_validation_error(
            "queued entries under provider or repo-policy authority must report queue_position",
        ));
    }
    Ok(())
}

fn validate_queue_entry_record(
    entry: &MergeQueueEntryRecord,
    candidate_id: &str,
    workspace_id: &str,
) -> Result<(), LandingCandidateValidationError> {
    ensure_eq(
        entry.record_kind.as_str(),
        MERGE_QUEUE_ENTRY_RECORD_KIND,
        "merge_queue_entry.record_kind",
    )?;
    ensure_eq(
        entry.schema_version,
        LANDING_CANDIDATE_SCHEMA_VERSION,
        "merge_queue_entry.schema_version",
    )?;
    ensure_eq(
        entry.landing_candidate_id_ref.as_str(),
        candidate_id,
        "merge_queue_entry.landing_candidate_id_ref",
    )?;
    ensure_eq(
        entry.review_workspace_id_ref.as_str(),
        workspace_id,
        "merge_queue_entry.review_workspace_id_ref",
    )?;
    validate_queue_entry_input(&MergeQueueEntryInput {
        merge_queue_entry_id: entry.merge_queue_entry_id.clone(),
        queue_authority_class: entry.queue_authority_class.clone(),
        queue_state: entry.queue_state.clone(),
        queue_position: entry.queue_position,
        queue_length: entry.queue_length,
        required_check_ids: entry.required_check_ids.clone(),
        captured_at: entry.captured_at.clone(),
        expires_at: entry.expires_at.clone(),
        invalidation_reasons: entry.invalidation_reasons.clone(),
        summary_label: entry.summary_label.clone(),
    })?;
    let local_estimate_only = entry.queue_authority_class == "aureline_local_estimate_only";
    if entry.local_estimate_only != local_estimate_only {
        return Err(landing_validation_error(
            "merge_queue_entry.local_estimate_only must match the queue_authority_class",
        ));
    }
    if entry.authoritative_position && (local_estimate_only || entry.queue_position.is_none()) {
        return Err(landing_validation_error(
            "merge_queue_entry.authoritative_position must reflect provider or policy authority with a position",
        ));
    }
    Ok(())
}

fn validate_command_input(
    command: &LandingCommandInput,
) -> Result<(), LandingCandidateValidationError> {
    ensure_nonempty(&command.command_id, "command_id")?;
    ensure_token(
        LANDING_COMMAND_CLASSES,
        &command.command_class,
        "command.command_class",
    )?;
    ensure_nonempty(&command.target_object_ref, "command.target_object_ref")?;
    ensure_nonempty(&command.target_object_kind, "command.target_object_kind")?;
    for reason in &command.blocked_reasons {
        ensure_token(LANDING_BLOCKED_REASONS, reason, "command.blocked_reasons")?;
    }
    if let Some(posture) = &command.provider_publish_posture {
        ensure_token(
            LANDING_PROVIDER_PUBLISH_POSTURES,
            posture,
            "command.provider_publish_posture",
        )?;
    }
    if command.command_class == "publish_to_provider" && command.provider_publish_posture.is_none()
    {
        return Err(landing_validation_error(
            "publish_to_provider commands must declare a provider_publish_posture",
        ));
    }
    if !command.emits_audit_event {
        return Err(landing_validation_error(format!(
            "command '{}' must emit an audit event when executed",
            command.command_id,
        )));
    }
    Ok(())
}

fn validate_command_record(
    command: &LandingCommandRecord,
    candidate_id: &str,
) -> Result<(), LandingCandidateValidationError> {
    ensure_eq(
        command.record_kind.as_str(),
        LANDING_COMMAND_RECORD_KIND,
        "command.record_kind",
    )?;
    ensure_eq(
        command.schema_version,
        LANDING_CANDIDATE_SCHEMA_VERSION,
        "command.schema_version",
    )?;
    ensure_eq(
        command.landing_candidate_id_ref.as_str(),
        candidate_id,
        "command.landing_candidate_id_ref",
    )?;
    validate_command_input(&LandingCommandInput {
        command_id: command.command_id.clone(),
        command_class: command.command_class.clone(),
        target_object_ref: command.target_object_ref.clone(),
        target_object_kind: command.target_object_kind.clone(),
        preview_supported: command.preview_supported,
        emits_audit_event: command.emits_audit_event,
        provider_publish_posture: command.provider_publish_posture.clone(),
        blocked_reasons: command.blocked_reasons.clone(),
        summary_label: command.summary_label.clone(),
    })?;
    let derived_actionable = command.blocked_reasons.is_empty();
    if command.actionable != derived_actionable {
        return Err(landing_validation_error(format!(
            "command.actionable for '{}' must match blocked_reasons emptiness",
            command.command_id,
        )));
    }
    Ok(())
}

fn validate_support_export_input(
    export: &LandingSupportExportInput,
) -> Result<(), LandingCandidateValidationError> {
    ensure_nonempty(
        &export.support_export_id,
        "support_export.support_export_id",
    )?;
    ensure_nonempty(
        &export.reopen_context_ref,
        "support_export.reopen_context_ref",
    )?;
    ensure_nonempty(
        &export.reopen_command_id_ref,
        "support_export.reopen_command_id_ref",
    )?;
    if !export
        .consumer_surfaces
        .iter()
        .any(|surface| surface == "support_export")
    {
        return Err(landing_validation_error(
            "support_export.consumer_surfaces must include support_export",
        ));
    }
    if !export
        .consumer_surfaces
        .iter()
        .any(|surface| surface == "cli_headless_entry")
    {
        return Err(landing_validation_error(
            "support_export.consumer_surfaces must include cli_headless_entry",
        ));
    }
    for surface in &export.consumer_surfaces {
        ensure_token(
            LANDING_CONSUMER_SURFACES,
            surface,
            "support_export.consumer_surfaces",
        )?;
    }
    Ok(())
}

fn validate_support_export(
    export: &LandingSupportExportPacket,
    candidate: &LandingCandidateRecord,
    commands: &[LandingCommandRecord],
    merge_queue_entry: Option<&MergeQueueEntryRecord>,
) -> Result<(), LandingCandidateValidationError> {
    ensure_eq(
        export.record_kind.as_str(),
        LANDING_SUPPORT_EXPORT_PACKET_RECORD_KIND,
        "support_export.record_kind",
    )?;
    ensure_eq(
        export.schema_version,
        LANDING_CANDIDATE_SCHEMA_VERSION,
        "support_export.schema_version",
    )?;
    ensure_eq(
        export.landing_candidate_id_ref.as_str(),
        candidate.landing_candidate_id.as_str(),
        "support_export.landing_candidate_id_ref",
    )?;
    ensure_eq(
        export.review_workspace_id_ref.as_str(),
        candidate.review_workspace_id_ref.as_str(),
        "support_export.review_workspace_id_ref",
    )?;
    validate_support_export_input(&LandingSupportExportInput {
        support_export_id: export.support_export_id.clone(),
        reopen_context_ref: export.reopen_context_ref.clone(),
        reopen_command_id_ref: export.reopen_command_id_ref.clone(),
        consumer_surfaces: export.consumer_surfaces.clone(),
        redaction_class: export.redaction_class.clone(),
        summary_label: export.summary_label.clone(),
    })?;
    if export.raw_url_export_allowed || export.raw_provider_payload_export_allowed {
        return Err(landing_validation_error(
            "support_export raw escape hatches must remain false",
        ));
    }
    if !export
        .source_schema_refs
        .iter()
        .any(|schema| schema == "schemas/review/landing_candidate.schema.json")
    {
        return Err(landing_validation_error(
            "support_export must cite schemas/review/landing_candidate.schema.json",
        ));
    }
    if !export
        .source_schema_refs
        .iter()
        .any(|schema| schema == "schemas/review/merge_queue_entry.schema.json")
    {
        return Err(landing_validation_error(
            "support_export must cite schemas/review/merge_queue_entry.schema.json",
        ));
    }
    let expected_commands = commands
        .iter()
        .map(|command| command.command_id.clone())
        .collect::<BTreeSet<_>>();
    let actual_commands = export
        .command_id_refs
        .iter()
        .cloned()
        .collect::<BTreeSet<_>>();
    if expected_commands != actual_commands {
        return Err(landing_validation_error(
            "support_export.command_id_refs must match packet command ids",
        ));
    }
    match (merge_queue_entry, export.merge_queue_entry_ref.as_ref()) {
        (Some(entry), Some(reference)) if reference == &entry.merge_queue_entry_id => {}
        (None, None) => {}
        _ => {
            return Err(landing_validation_error(
                "support_export.merge_queue_entry_ref must match the packet's merge queue entry",
            ))
        }
    }
    if export.eligibility_snapshot.mergeable_state != candidate.mergeable_state
        || export.eligibility_snapshot.eligibility_state != candidate.eligibility_state
        || export.eligibility_snapshot.stale_base_state != candidate.stale_base_state
        || export.eligibility_snapshot.checks_freshness_state != candidate.checks_freshness_state
        || export.eligibility_snapshot.approval_state != candidate.approval_state
        || export.eligibility_snapshot.policy_block_state != candidate.policy_block_state
        || export.eligibility_snapshot.invalidation_reasons != candidate.invalidation_reasons
        || export.eligibility_snapshot.blocked_reasons != candidate.blocked_reasons
        || export.eligibility_snapshot.landing_authority_class != candidate.landing_authority_class
    {
        return Err(landing_validation_error(
            "support_export.eligibility_snapshot must mirror the landing candidate state",
        ));
    }
    let snapshot_queue_state = merge_queue_entry
        .map(|entry| entry.queue_state.clone())
        .unwrap_or_else(|| "not_queued".to_string());
    if export.eligibility_snapshot.queue_state != snapshot_queue_state {
        return Err(landing_validation_error(
            "support_export.eligibility_snapshot.queue_state must mirror the queue entry state",
        ));
    }
    let snapshot_queue_authority =
        merge_queue_entry.map(|entry| entry.queue_authority_class.clone());
    if export.eligibility_snapshot.queue_authority_class != snapshot_queue_authority {
        return Err(landing_validation_error(
            "support_export.eligibility_snapshot.queue_authority_class must mirror the queue entry",
        ));
    }
    Ok(())
}

fn validate_inspection(
    inspection: &LandingInspectionRecord,
    packet: &LandingCandidatePacket,
) -> Result<(), LandingCandidateValidationError> {
    ensure_eq(
        inspection.record_kind.as_str(),
        LANDING_INSPECTION_RECORD_KIND,
        "inspection.record_kind",
    )?;
    ensure_eq(
        inspection.schema_version,
        LANDING_CANDIDATE_SCHEMA_VERSION,
        "inspection.schema_version",
    )?;
    let expected = landing_inspection_record(
        &packet.landing_candidate,
        packet.merge_queue_entry.as_ref(),
        &packet.commands,
        &packet.support_export,
    );
    if inspection != &expected {
        return Err(landing_validation_error(
            "inspection row must equal the derived inspection from candidate, queue, commands, and support export",
        ));
    }
    Ok(())
}

fn derive_invalidation_reasons(
    stale_base_state: &str,
    checks_freshness_state: &str,
    approval_state: &str,
    policy_block_state: &str,
    workspace_checks: &[ReviewWorkspaceCheckFreshnessRecord],
) -> Vec<String> {
    let mut reasons = Vec::new();
    if stale_base_state == "base_stale_blocks_landing" {
        reasons.push("stale_base".to_string());
    }
    if checks_freshness_state == "checks_stale_blocks_landing" {
        reasons.push("checks_stale".to_string());
    }
    if approval_state == "approval_invalidated_by_changes" {
        reasons.push("approval_invalidated".to_string());
    }
    if policy_block_state == "policy_blocked" {
        reasons.push("policy_blocked".to_string());
    }
    if workspace_checks
        .iter()
        .any(|check| check.blocks_operator_truth_claim_when_stale && is_stale_check(check))
        && !reasons.iter().any(|reason| reason == "checks_stale")
    {
        reasons.push("checks_stale".to_string());
    }
    reasons
}

fn is_stale_check(check: &ReviewWorkspaceCheckFreshnessRecord) -> bool {
    matches!(
        check.check_freshness_class.as_str(),
        "check_stale_blocks_operator_truth" | "check_unavailable_blocks_operator_truth"
    )
}

fn derive_blocked_reasons(
    target_branch_ref: &str,
    stale_base_state: &str,
    checks_freshness_state: &str,
    approval_state: &str,
    policy_block_state: &str,
    invalidation_reasons: &[String],
) -> Vec<String> {
    let mut reasons = Vec::new();
    if target_branch_ref.trim().is_empty() {
        reasons.push("target_branch_missing".to_string());
    }
    if stale_base_state == "base_stale_blocks_landing" {
        reasons.push("base_revision_stale".to_string());
    }
    if checks_freshness_state == "checks_stale_blocks_landing" {
        reasons.push("required_check_stale".to_string());
    }
    if approval_state == "approval_required_outstanding" {
        reasons.push("approval_missing".to_string());
    }
    if approval_state == "approval_invalidated_by_changes" {
        reasons.push("approval_invalidated".to_string());
    }
    if policy_block_state == "policy_blocked" {
        reasons.push("policy_blocked".to_string());
    }
    if invalidation_reasons
        .iter()
        .any(|reason| reason == "review_pack_version_changed")
    {
        reasons.push("review_pack_version_drift".to_string());
    }
    if invalidation_reasons
        .iter()
        .any(|reason| reason == "worktree_scope_changed")
    {
        reasons.push("worktree_scope_changed".to_string());
    }
    if invalidation_reasons
        .iter()
        .any(|reason| reason == "environment_capsule_changed")
    {
        reasons.push("environment_capsule_changed".to_string());
    }
    reasons.sort();
    reasons.dedup();
    reasons
}

fn ensure_token(
    tokens: &[&str],
    value: &str,
    field: &str,
) -> Result<(), LandingCandidateValidationError> {
    if contains_token(tokens, value) {
        Ok(())
    } else {
        Err(landing_validation_error(format!(
            "unsupported {field} '{value}'"
        )))
    }
}

fn contains_token(tokens: &[&str], value: &str) -> bool {
    tokens.iter().any(|token| token == &value)
}

fn ensure_nonempty(value: &str, field: &str) -> Result<(), LandingCandidateValidationError> {
    if value.trim().is_empty() {
        Err(landing_validation_error(format!(
            "{field} must not be empty"
        )))
    } else {
        Ok(())
    }
}

fn ensure_eq<T>(actual: T, expected: T, field: &str) -> Result<(), LandingCandidateValidationError>
where
    T: PartialEq + fmt::Display,
{
    if actual == expected {
        Ok(())
    } else {
        Err(landing_validation_error(format!(
            "{field} expected '{expected}' but got '{actual}'"
        )))
    }
}

fn landing_validation_error(message: impl Into<String>) -> LandingCandidateValidationError {
    LandingCandidateValidationError {
        message: message.into(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::diff::{
        DiffFileInput, DiffHunkInput, DiffLineInput, DiffLineKind, DiffOpenTarget, DiffViewMode,
        DiffViewSurfacePacket,
    };
    use crate::workspace::{
        ReviewWorkItemLinkInput, ReviewWorkspaceBetaInput, ReviewWorkspaceCheckFreshnessInput,
        ReviewWorkspaceDurableCommentAnchorInput, ReviewWorkspaceSeedInput,
        ReviewWorkspaceSeedPacket, ReviewWorkspaceSupportExportInput,
    };
    use std::path::PathBuf;

    fn build_workspace_packet() -> ReviewWorkspaceBetaPacket {
        let diff_input = DiffFileInput {
            workspace_ref: "workspace.unit.landing".to_string(),
            truth_source_ref: "git.status.snapshot.unit.landing".to_string(),
            repo_root: PathBuf::from("/workspace/unit"),
            logical_root_ref: "root.local.unit".to_string(),
            worktree_ref: "worktree.local.unit".to_string(),
            group_token: "unstaged".to_string(),
            path: PathBuf::from("src/lib.rs"),
            original_path: None,
            status_code: ".M".to_string(),
            language_id: Some("rust".to_string()),
            view_mode: DiffViewMode::Unified,
            generated_at: "2026-05-19T00:00:00Z".to_string(),
            hunks: vec![DiffHunkInput {
                hunk_header: "@@ -1,2 +1,3 @@".to_string(),
                old_start: 1,
                old_lines: 2,
                new_start: 1,
                new_lines: 3,
                lines: vec![
                    DiffLineInput {
                        line_kind: DiffLineKind::Context,
                        old_line_number: Some(1),
                        new_line_number: Some(1),
                        raw_text: "pub fn demo() {".to_string(),
                    },
                    DiffLineInput {
                        line_kind: DiffLineKind::Addition,
                        old_line_number: None,
                        new_line_number: Some(2),
                        raw_text: "    trace_landing();".to_string(),
                    },
                ],
            }],
        };
        let open_target = DiffOpenTarget::from_change_list_row_parts(
            &diff_input.workspace_ref,
            &diff_input.truth_source_ref,
            "git.change.row.unit.landing.unstaged.src-lib-rs.modified",
            &diff_input.group_token,
            diff_input.path.clone(),
            diff_input.original_path.clone(),
            &diff_input.status_code,
            "modified",
        );
        let diff_packet = DiffViewSurfacePacket::from_file_input(open_target, diff_input);

        let seed_input = ReviewWorkspaceSeedInput {
            review_workspace_id: "review.git.workspace.unit.landing".to_string(),
            branch_or_worktree_ref: "worktree.local.unit".to_string(),
            base_revision_ref: Some("git.rev.base".to_string()),
            head_revision_ref: Some("git.rev.head".to_string()),
            actor_ref: "actor.local.dev".to_string(),
            policy_epoch: "policy.epoch.unit".to_string(),
            trust_state: "trusted".to_string(),
            execution_context_id: Some("exec.ctx.unit".to_string()),
            client_scopes: vec!["desktop_product".to_string()],
            created_at: "2026-05-19T00:00:00Z".to_string(),
            provider_overlay: None,
            work_item_links: vec![ReviewWorkItemLinkInput {
                work_item_detail_record_id_ref: "work_item.detail.unit".to_string(),
                target_object_identity_ref: "provider.object.issue.unit".to_string(),
                work_item_authority_class: "linked_review_only_no_provider_overlay".to_string(),
                write_authority_class: "write_admissible_local_draft_only_no_provider_path"
                    .to_string(),
                issue_to_branch_link_class: "linked_local_branch_or_worktree_no_provider_overlay"
                    .to_string(),
                actor_ref: "actor.local.dev".to_string(),
                command_id_ref: "cmd:review.workspace.link_work_item".to_string(),
                linked_at: "2026-05-19T00:00:00Z".to_string(),
                summary_label: "Work item linked to local review workspace".to_string(),
            }],
            graph_cue_packets: Vec::new(),
        };
        let seed_packet = ReviewWorkspaceSeedPacket::from_diff_packet(seed_input, &diff_packet);
        let anchor_id = seed_packet.anchors[0].anchor_id.clone();

        let beta_input = ReviewWorkspaceBetaInput {
            packet_id: "review.workspace.beta.unit.landing".to_string(),
            generated_at: "2026-05-19T00:00:00Z".to_string(),
            comment_anchors: vec![ReviewWorkspaceDurableCommentAnchorInput {
                source_anchor_id_ref: anchor_id,
                comment_thread_id: "review.thread.unit.landing".to_string(),
                comment_payload_label_opaque_ref: "label.review.comment.unit.landing".to_string(),
                posted_actor_ref: "actor.local.dev".to_string(),
                posted_at: "2026-05-19T00:00:00Z".to_string(),
                anchor_drift_state: "anchor_bound_exact".to_string(),
                anchor_drift_required_user_action:
                    "no_user_action_required_anchor_bound_or_remapped".to_string(),
                local_vs_provider_freshness_class: "local_only_no_provider_overlay".to_string(),
                remap_chain_target_id_refs: Vec::new(),
                archived_at: None,
                summary_label: "Comment bound to landing-unit demo".to_string(),
            }],
            check_freshness: vec![ReviewWorkspaceCheckFreshnessInput {
                check_id: "review.check.unit.local_ci".to_string(),
                check_kind: "local_ci_parity".to_string(),
                check_status_class: "check_passed".to_string(),
                check_freshness_class: "check_current".to_string(),
                check_authority_class: "local_review_pack".to_string(),
                evidence_ref: "review.evidence.unit.local_ci".to_string(),
                captured_at: "2026-05-19T00:00:00Z".to_string(),
                expires_at: Some("2026-05-19T02:00:00Z".to_string()),
                browser_state_independent: true,
                blocks_operator_truth_claim_when_stale: false,
                summary_label: "Local CI parity current".to_string(),
            }],
            browser_handoff: None,
            support_export: ReviewWorkspaceSupportExportInput {
                support_export_id: "support.export.review_workspace.unit.landing".to_string(),
                reopen_context_ref: "review.reopen_context.unit.landing".to_string(),
                reopen_command_id_ref: "cmd:review.workspace.reopen".to_string(),
                consumer_surfaces: vec![
                    "review_workspace_inspector".to_string(),
                    "cli_headless_entry".to_string(),
                    "support_export".to_string(),
                ],
                redaction_class: "metadata_safe_default".to_string(),
                summary_label: "Support export for landing-unit demo".to_string(),
            },
        };
        ReviewWorkspaceBetaPacket::from_seed_packet(beta_input, &seed_packet)
            .expect("workspace packet builds")
    }

    fn base_landing_input() -> LandingCandidateInput {
        LandingCandidateInput {
            landing_candidate_id: "review.landing_candidate.unit.demo".to_string(),
            packet_id: "review.landing_candidate.packet.unit.demo".to_string(),
            generated_at: "2026-05-19T00:00:00Z".to_string(),
            target_branch_ref: "git.branch.main".to_string(),
            base_revision_ref: "git.rev.base".to_string(),
            head_revision_ref: "git.rev.head".to_string(),
            change_object_ref: "change.object.unit.demo".to_string(),
            worktree_identity_ref: "worktree.local.unit".to_string(),
            review_pack_digest_ref: "review.pack.digest.unit.demo".to_string(),
            environment_capsule_digest_ref: "env.capsule.digest.unit.demo".to_string(),
            merge_strategy_class: "squash_then_fast_forward".to_string(),
            landing_authority_class: "provider_authoritative_queue_state".to_string(),
            mergeable_state: "mergeable".to_string(),
            stale_base_state: "base_current".to_string(),
            checks_freshness_state: "checks_current".to_string(),
            approval_state: "approved_current".to_string(),
            policy_block_state: "policy_clear".to_string(),
            required_check_ids: vec!["review.check.unit.local_ci".to_string()],
            invalidation_reasons: Vec::new(),
            summary_label: "Provider-authoritative mergeable candidate".to_string(),
            merge_queue_entry: Some(MergeQueueEntryInput {
                merge_queue_entry_id: "review.merge_queue.unit.demo".to_string(),
                queue_authority_class: "provider_authoritative_queue_state".to_string(),
                queue_state: "queued".to_string(),
                queue_position: Some(2),
                queue_length: Some(5),
                required_check_ids: vec!["review.check.unit.local_ci".to_string()],
                captured_at: "2026-05-19T00:00:00Z".to_string(),
                expires_at: Some("2026-05-19T02:00:00Z".to_string()),
                invalidation_reasons: Vec::new(),
                summary_label: "Queued behind one PR".to_string(),
            }),
            commands: vec![
                LandingCommandInput {
                    command_id: "cmd.landing.dequeue.unit.demo".to_string(),
                    command_class: "dequeue".to_string(),
                    target_object_ref: "review.merge_queue.unit.demo".to_string(),
                    target_object_kind: "review_merge_queue_entry".to_string(),
                    preview_supported: true,
                    emits_audit_event: true,
                    provider_publish_posture: None,
                    blocked_reasons: Vec::new(),
                    summary_label: "Dequeue without landing".to_string(),
                },
                LandingCommandInput {
                    command_id: "cmd.landing.publish_to_provider.unit.demo".to_string(),
                    command_class: "publish_to_provider".to_string(),
                    target_object_ref: "review.landing_candidate.unit.demo".to_string(),
                    target_object_kind: "review_landing_candidate".to_string(),
                    preview_supported: true,
                    emits_audit_event: true,
                    provider_publish_posture: Some("publish_minted_not_launched".to_string()),
                    blocked_reasons: Vec::new(),
                    summary_label: "Publish landing intent to provider".to_string(),
                },
            ],
            support_export: LandingSupportExportInput {
                support_export_id: "support.export.review_landing.unit.demo".to_string(),
                reopen_context_ref: "review.landing.reopen_context.unit.demo".to_string(),
                reopen_command_id_ref: "cmd:review.landing.reopen".to_string(),
                consumer_surfaces: vec![
                    "review_workspace_inspector".to_string(),
                    "review_landing_strip".to_string(),
                    "merge_queue_inspector".to_string(),
                    "cli_headless_entry".to_string(),
                    "support_export".to_string(),
                ],
                redaction_class: "metadata_safe_default".to_string(),
                summary_label: "Support export for landing-unit demo".to_string(),
            },
        }
    }

    #[test]
    fn mergeable_candidate_is_queue_eligible_and_queued() {
        let workspace_packet = build_workspace_packet();
        let packet =
            LandingCandidatePacket::from_workspace_packet(base_landing_input(), &workspace_packet)
                .expect("packet projects");

        assert_eq!(packet.landing_candidate.mergeable_state, "mergeable");
        assert_eq!(packet.landing_candidate.eligibility_state, "queue_eligible");
        assert!(packet.inspection.mergeable);
        assert!(packet.inspection.queue_eligible);
        assert!(packet.inspection.queued);
        assert!(!packet.inspection.stale_base_blocks_landing);
        assert!(!packet.inspection.checks_stale_blocks_landing);
        assert!(!packet.inspection.policy_blocks_landing);
        assert!(!packet.inspection.approval_invalidated);
        assert!(packet.inspection.provider_authoritative);
        assert!(packet.landing_requires_explicit_candidate());
        assert!(packet.truths_are_separable());
        assert!(packet.raw_escape_hatches_absent());
        assert!(packet.support_export.merge_queue_entry_ref.is_some());
    }

    #[test]
    fn stale_base_blocks_landing_and_records_invalidation() {
        let workspace_packet = build_workspace_packet();
        let mut input = base_landing_input();
        input.stale_base_state = "base_stale_blocks_landing".to_string();
        input.merge_queue_entry = Some(MergeQueueEntryInput {
            merge_queue_entry_id: "review.merge_queue.unit.demo".to_string(),
            queue_authority_class: "provider_authoritative_queue_state".to_string(),
            queue_state: "queued_invalidated_by_stale_base".to_string(),
            queue_position: Some(2),
            queue_length: Some(5),
            required_check_ids: vec!["review.check.unit.local_ci".to_string()],
            captured_at: "2026-05-19T00:00:00Z".to_string(),
            expires_at: Some("2026-05-19T02:00:00Z".to_string()),
            invalidation_reasons: vec!["stale_base".to_string()],
            summary_label: "Queued but invalidated by stale base".to_string(),
        });
        // Adjust commands so landing-time blocked reasons surface.
        input.commands[1].blocked_reasons = vec!["base_revision_stale".to_string()];

        let packet = LandingCandidatePacket::from_workspace_packet(input, &workspace_packet)
            .expect("packet projects with stale base");

        assert_eq!(
            packet.landing_candidate.eligibility_state,
            "queue_not_eligible"
        );
        assert!(packet.inspection.stale_base_blocks_landing);
        assert!(!packet.inspection.queue_eligible);
        assert!(!packet.inspection.queued);
        assert!(packet
            .landing_candidate
            .invalidation_reasons
            .contains(&"stale_base".to_string()));
        assert!(packet
            .landing_candidate
            .blocked_reasons
            .contains(&"base_revision_stale".to_string()));
        assert!(!packet.commands[1].actionable);
    }

    #[test]
    fn approval_invalidated_propagates_to_blocked_reasons() {
        let workspace_packet = build_workspace_packet();
        let mut input = base_landing_input();
        input.approval_state = "approval_invalidated_by_changes".to_string();
        input.commands[1].blocked_reasons = vec!["approval_invalidated".to_string()];

        let packet = LandingCandidatePacket::from_workspace_packet(input, &workspace_packet)
            .expect("packet projects with invalidated approval");

        assert!(packet.inspection.approval_invalidated);
        assert!(packet
            .landing_candidate
            .blocked_reasons
            .contains(&"approval_invalidated".to_string()));
        assert!(packet
            .landing_candidate
            .invalidation_reasons
            .contains(&"approval_invalidated".to_string()));
    }

    #[test]
    fn local_estimate_authority_cannot_carry_authoritative_position() {
        let workspace_packet = build_workspace_packet();
        let mut input = base_landing_input();
        input.merge_queue_entry = Some(MergeQueueEntryInput {
            merge_queue_entry_id: "review.merge_queue.unit.demo".to_string(),
            queue_authority_class: "aureline_local_estimate_only".to_string(),
            queue_state: "queued".to_string(),
            queue_position: Some(3),
            queue_length: Some(5),
            required_check_ids: vec!["review.check.unit.local_ci".to_string()],
            captured_at: "2026-05-19T00:00:00Z".to_string(),
            expires_at: None,
            invalidation_reasons: Vec::new(),
            summary_label: "Local estimate".to_string(),
        });

        let err = LandingCandidatePacket::from_workspace_packet(input, &workspace_packet)
            .expect_err("local estimate with authoritative position must fail");
        assert!(err
            .message()
            .contains("aureline_local_estimate_only entries must not claim"));
    }

    #[test]
    fn required_check_ids_must_exist_in_workspace() {
        let workspace_packet = build_workspace_packet();
        let mut input = base_landing_input();
        input.required_check_ids = vec!["review.check.unit.missing".to_string()];

        let err = LandingCandidatePacket::from_workspace_packet(input, &workspace_packet)
            .expect_err("missing check must fail");
        assert!(err.message().contains("review.check.unit.missing"));
    }

    #[test]
    fn publish_command_requires_provider_posture() {
        let workspace_packet = build_workspace_packet();
        let mut input = base_landing_input();
        input.commands[1].provider_publish_posture = None;

        let err = LandingCandidatePacket::from_workspace_packet(input, &workspace_packet)
            .expect_err("publish without posture must fail");
        assert!(err.message().contains("provider_publish_posture"));
    }

    #[test]
    fn projection_round_trips_through_json() {
        let workspace_packet = build_workspace_packet();
        let packet =
            LandingCandidatePacket::from_workspace_packet(base_landing_input(), &workspace_packet)
                .expect("packet projects");
        let payload = serde_json::to_string_pretty(&packet).expect("serialize");
        let projection = project_landing_candidate_packet(&payload).expect("payload re-projects");
        assert_eq!(projection.packet_id, packet.packet_id);
        assert_eq!(projection.queue_state, "queued");
        assert!(projection.landing_requires_explicit_candidate);
        assert!(projection.support_export_reopenable);
        assert!(projection
            .consumer_surfaces
            .iter()
            .any(|surface| surface == "merge_queue_inspector"));
    }
}
