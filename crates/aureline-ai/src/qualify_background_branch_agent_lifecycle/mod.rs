//! Stable qualification records for background branch-agent lifecycle truth.
//!
//! This module qualifies any M4-exposed branch-agent or long-running AI
//! automation lane as accountable background work. The packet binds launch
//! review, active run rows, checkpoint rows, drift/re-review drills, operator
//! takeover, completion review, cleanup posture, and support/export parity to
//! one stable run id.
//!
//! The record is export-safe. It carries stable ids, refs, controlled
//! vocabulary, booleans, coarse bands, and review labels only. Raw branch
//! names, raw worktree paths, raw diff bodies, raw logs, raw provider payloads,
//! endpoint URLs, credentials, exact token counts, and exact cost amounts stay
//! outside the support boundary.
//!
//! The boundary schema is
//! [`schemas/ai/background-branch-agent-run.schema.json`](../../../../schemas/ai/background-branch-agent-run.schema.json).
//! The contract doc is
//! [`docs/ai/m4/qualify-background-branch-agent-lifecycle.md`](../../../../docs/ai/m4/qualify-background-branch-agent-lifecycle.md).
//! The protected fixture directory is
//! [`fixtures/ai/m4/qualify-background-branch-agent-lifecycle/`](../../../../fixtures/ai/m4/qualify-background-branch-agent-lifecycle/).

#[cfg(test)]
mod tests;

use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

/// Stable record-kind tag carried by [`BackgroundBranchAgentLifecyclePacket`].
pub const BACKGROUND_BRANCH_AGENT_LIFECYCLE_RECORD_KIND: &str =
    "background_branch_agent_lifecycle_qualification";

/// Schema version for background branch-agent lifecycle qualification records.
pub const BACKGROUND_BRANCH_AGENT_LIFECYCLE_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the boundary schema.
pub const BACKGROUND_BRANCH_AGENT_LIFECYCLE_SCHEMA_REF: &str =
    "schemas/ai/background-branch-agent-run.schema.json";

/// Repo-relative path of the M4 lifecycle qualification contract doc.
pub const BACKGROUND_BRANCH_AGENT_LIFECYCLE_AI_DOC_REF: &str =
    "docs/ai/m4/qualify-background-branch-agent-lifecycle.md";

/// Repo-relative path of the frozen branch-agent lifecycle base contract.
pub const BACKGROUND_BRANCH_AGENT_BASE_CONTRACT_REF: &str =
    "docs/ai/background_branch_agent_lifecycle.md";

/// Repo-relative path of the protected fixture directory.
pub const BACKGROUND_BRANCH_AGENT_LIFECYCLE_FIXTURE_DIR: &str =
    "fixtures/ai/m4/qualify-background-branch-agent-lifecycle";

/// Repo-relative path of the checked support-export artifact.
pub const BACKGROUND_BRANCH_AGENT_LIFECYCLE_ARTIFACT_REF: &str =
    "artifacts/ai/m4/qualify-background-branch-agent-lifecycle/support_export.json";

/// Repo-relative path of the checked Markdown summary.
pub const BACKGROUND_BRANCH_AGENT_LIFECYCLE_SUMMARY_REF: &str =
    "artifacts/ai/m4/qualify-background-branch-agent-lifecycle.md";

/// Lifecycle state for a background branch-agent run.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BranchAgentRunState {
    /// The agent is preparing a plan.
    Planning,
    /// The agent is collecting context and evidence.
    CollectingContext,
    /// The agent is editing in an admitted isolated target.
    Editing,
    /// The agent is validating the produced change.
    Validating,
    /// The agent is stopped at an approval gate.
    AwaitingApproval,
    /// The run cannot progress without a user, policy, or route change.
    Blocked,
    /// The run failed and preserved review artifacts.
    Failed,
    /// The run is ready for human review.
    ReadyForReview,
    /// Drift requires fresh review or manual takeover before more writes.
    ReReviewRequired,
    /// The run was cancelled and preserved checkpoint lineage.
    Cancelled,
    /// The operator took over the run manually.
    OperatorTakeover,
}

impl BranchAgentRunState {
    /// Stable token used in exports and fixtures.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Planning => "planning",
            Self::CollectingContext => "collecting_context",
            Self::Editing => "editing",
            Self::Validating => "validating",
            Self::AwaitingApproval => "awaiting_approval",
            Self::Blocked => "blocked",
            Self::Failed => "failed",
            Self::ReadyForReview => "ready_for_review",
            Self::ReReviewRequired => "re_review_required",
            Self::Cancelled => "cancelled",
            Self::OperatorTakeover => "operator_takeover",
        }
    }
}

/// Execution locus where the branch-agent run is admitted to operate.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BranchAgentExecutionLocus {
    /// Local-only current-worktree assist; mutating writes are not automatic.
    LocalOnlyCurrentWorktreeAssist,
    /// Isolated side worktree attached to the same local repository.
    IsolatedSideWorktree,
    /// Side branch without a checked-out worktree.
    SideBranchNoWorktree,
    /// Ephemeral workspace isolated from the active worktree.
    EphemeralWorkspace,
    /// Managed remote workspace with explicit route and credential posture.
    ManagedRemoteWorkspace,
}

impl BranchAgentExecutionLocus {
    /// Stable token used in exports and fixtures.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LocalOnlyCurrentWorktreeAssist => "local_only_current_worktree_assist",
            Self::IsolatedSideWorktree => "isolated_side_worktree",
            Self::SideBranchNoWorktree => "side_branch_no_worktree",
            Self::EphemeralWorkspace => "ephemeral_workspace",
            Self::ManagedRemoteWorkspace => "managed_remote_workspace",
        }
    }
}

/// Controlled action classes exposed on active, checkpoint, or review rows.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BranchAgentOperatorAction {
    /// Pause the agent at the next safe checkpoint.
    Pause,
    /// Cancel the run while preserving review artifacts.
    Cancel,
    /// Open the review packet or diff.
    OpenReview,
    /// Take over the branch/worktree manually.
    TakeOverManually,
    /// Compare the produced change to its base.
    CompareToBase,
    /// Cherry-pick the change through a user-driven command.
    CherryPick,
    /// Rerun declared validation.
    RerunValidation,
    /// Discard or delete the side branch/worktree after review.
    DiscardBranch,
    /// Resume from a named checkpoint after fresh review.
    ResumeFromCheckpoint,
}

impl BranchAgentOperatorAction {
    /// Stable token used in exports and fixtures.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Pause => "pause",
            Self::Cancel => "cancel",
            Self::OpenReview => "open_review",
            Self::TakeOverManually => "take_over_manually",
            Self::CompareToBase => "compare_to_base",
            Self::CherryPick => "cherry_pick",
            Self::RerunValidation => "rerun_validation",
            Self::DiscardBranch => "discard_branch",
            Self::ResumeFromCheckpoint => "resume_from_checkpoint",
        }
    }
}

/// Drift trigger requiring re-review or operator takeover.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BranchAgentDriftTrigger {
    /// The base branch or base commit advanced.
    BaseBranchAdvance,
    /// The provider, model, route, or execution host changed.
    ProviderModelRouteDrift,
    /// The active policy epoch changed.
    PolicyEpochChange,
    /// Workspace trust narrowed.
    TrustNarrowing,
    /// The plan expanded beyond the reviewed boundary.
    BoundaryExpansion,
}

impl BranchAgentDriftTrigger {
    /// Stable token used in exports and fixtures.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::BaseBranchAdvance => "base_branch_advance",
            Self::ProviderModelRouteDrift => "provider_model_route_drift",
            Self::PolicyEpochChange => "policy_epoch_change",
            Self::TrustNarrowing => "trust_narrowing",
            Self::BoundaryExpansion => "boundary_expansion",
        }
    }

    /// Drift triggers that must be covered by the M4 drill packet.
    pub const fn required_coverage() -> [Self; 5] {
        [
            Self::BaseBranchAdvance,
            Self::ProviderModelRouteDrift,
            Self::PolicyEpochChange,
            Self::TrustNarrowing,
            Self::BoundaryExpansion,
        ]
    }
}

/// Cancellation posture shown to the operator.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BranchAgentCancellationPosture {
    /// Compute stopped immediately.
    ComputeStoppedImmediately,
    /// Compute is finishing a safe stop sequence.
    FinishingSafely,
    /// Compute continues only until the next safe checkpoint.
    ContinueUntilSafeCheckpoint,
    /// Cancellation was not requested for this row.
    NotCancelled,
}

/// Cleanup disposition for side branches, worktrees, and retained artifacts.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BranchAgentCleanupDisposition {
    /// Branch/worktree is retained for manual review.
    RetainForReview,
    /// Cleanup is available only through a previewed user action.
    PreviewDeleteAvailable,
    /// Branch/worktree was discarded after preserving evidence.
    DiscardedAfterEvidenceRetention,
    /// Cleanup is blocked by retention, legal hold, or policy.
    RetentionBlocked,
}

/// Launch review sheet disclosed before execution begins.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BranchAgentLaunchReviewSheet {
    /// Stable run id this row belongs to.
    pub stable_run_id: String,
    /// Requested user goal.
    pub requested_goal: String,
    /// Base branch or base commit ref.
    pub base_ref: String,
    /// Target branch or worktree identity ref.
    pub target_identity_ref: String,
    /// Tool or connector classes admitted for the run.
    pub tool_connector_classes: Vec<String>,
    /// Approval gates expected before writes or landing.
    pub approval_gates: Vec<String>,
    /// Coarse cost and risk band disclosed before dispatch.
    pub estimated_cost_risk_band: String,
    /// Secret or credential scope admitted to the run.
    pub secret_scope: String,
    /// Stop conditions that bound automation.
    pub stop_conditions: Vec<String>,
}

/// Active-run row projected into UI, export, and support surfaces.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BranchAgentActiveRunRow {
    /// Stable run id this row belongs to.
    pub stable_run_id: String,
    /// Elapsed time label safe for UI and support export.
    pub elapsed_time_label: String,
    /// Current milestone.
    pub current_milestone: BranchAgentRunState,
    /// Environment assumptions used by the agent.
    pub environment_assumptions: Vec<String>,
    /// Pending approvals on this row.
    pub pending_approvals: Vec<String>,
    /// Operator actions available on this row.
    pub operator_actions: Vec<BranchAgentOperatorAction>,
    /// Current execution locus for this row.
    pub execution_locus: BranchAgentExecutionLocus,
}

/// Checkpoint row preserving reviewable artifacts and lineage.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BranchAgentCheckpointRow {
    /// Stable run id this row belongs to.
    pub stable_run_id: String,
    /// Stable checkpoint id.
    pub checkpoint_id: String,
    /// Elapsed time label when the checkpoint was produced.
    pub elapsed_time_label: String,
    /// Milestone reached by this checkpoint.
    pub milestone: BranchAgentRunState,
    /// New artifacts produced by this checkpoint.
    pub new_artifact_refs: Vec<String>,
    /// Evidence refs cited by this checkpoint.
    pub evidence_refs: Vec<String>,
    /// Pending approvals after this checkpoint.
    pub pending_approvals: Vec<String>,
    /// Operator actions available from this checkpoint.
    pub operator_actions: Vec<BranchAgentOperatorAction>,
    /// Whether already-produced artifacts are retained for manual review.
    pub review_artifacts_preserved: bool,
}

/// Drift drill row proving the run pauses or narrows safely.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BranchAgentDriftDrillRow {
    /// Stable run id this row belongs to.
    pub stable_run_id: String,
    /// Drift trigger exercised by this drill.
    pub trigger: BranchAgentDriftTrigger,
    /// Human-readable drift summary.
    pub drift_summary: String,
    /// Resulting state after drift detection.
    pub resulting_state: BranchAgentRunState,
    /// Further writes are blocked until fresh review or takeover.
    pub further_writes_blocked: bool,
    /// Diff, logs, checkpoints, and cited evidence are retained.
    pub already_produced_artifacts_preserved: bool,
    /// Fresh review or operator takeover is required.
    pub requires_re_review_or_takeover: bool,
    /// The run pauses or narrows rather than widening silently.
    pub pauses_or_narrows_safely: bool,
}

/// Operator takeover row for a run.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BranchAgentTakeoverRow {
    /// Stable run id this row belongs to.
    pub stable_run_id: String,
    /// Branch identity is preserved.
    pub branch_identity_preserved: bool,
    /// Checkpoint lineage is preserved.
    pub checkpoint_lineage_preserved: bool,
    /// Tool-call history is preserved.
    pub tool_call_history_preserved: bool,
    /// Validation receipts are preserved.
    pub validation_receipts_preserved: bool,
    /// Pending writes are explicitly disclosed.
    pub pending_writes_disclosed: bool,
    /// Safe next step shown to the operator.
    pub safe_next_step: String,
    /// Rerun or manual continuation actions.
    pub rerun_options: Vec<BranchAgentOperatorAction>,
}

/// Completion review row for a run.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BranchAgentCompletionReview {
    /// Stable run id this row belongs to.
    pub stable_run_id: String,
    /// Diff summary ref or label.
    pub diff_summary_ref: String,
    /// Validation summary ref or label.
    pub validation_summary_ref: String,
    /// Evidence packet ref.
    pub evidence_packet_ref: String,
    /// Compare-to-base action is available.
    pub compare_to_base_available: bool,
    /// Cleanup or delete-branch options are available.
    pub cleanup_options_available: bool,
    /// Follow-up commands shown to the operator.
    pub follow_up_actions: Vec<BranchAgentOperatorAction>,
    /// Agent self-merge is forbidden.
    pub self_merge_forbidden: bool,
    /// Direct push to protected destinations is forbidden.
    pub protected_destination_push_forbidden: bool,
}

/// Cleanup row preserving retention truth.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BranchAgentCleanupRow {
    /// Stable run id this row belongs to.
    pub stable_run_id: String,
    /// Cleanup disposition.
    pub disposition: BranchAgentCleanupDisposition,
    /// Evidence retention posture.
    pub evidence_retention: String,
    /// Checkpoint retention posture.
    pub checkpoint_retention: String,
    /// Whether cleanup is previewed before destructive removal.
    pub preview_required_before_delete: bool,
    /// Whether diff, logs, and cited evidence survive cleanup.
    pub review_artifacts_survive_cleanup: bool,
    /// Export ref proving cleanup posture.
    pub support_export_ref: String,
}

/// Security and policy qualification block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BranchAgentSecurityReview {
    /// Mutating tools cannot be self-approved by the agent.
    pub no_self_approved_mutating_tools: bool,
    /// Worktree isolation cannot be bypassed.
    pub no_worktree_isolation_bypass: bool,
    /// Execution loci remain separate in state and export.
    pub execution_loci_not_collapsed: bool,
    /// Local-only, side-worktree, and managed-remote outcomes stay distinct.
    pub local_side_and_managed_loci_distinct: bool,
    /// Protected destinations cannot be self-pushed.
    pub protected_destination_self_push_blocked: bool,
}

/// Consumer parity block for UI, docs, support export, and release packets.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BranchAgentConsumerProjection {
    /// UI active rows consume this packet by stable run id.
    pub ui_rows_use_stable_run_id: bool,
    /// Support exports consume this packet by stable run id.
    pub support_exports_use_stable_run_id: bool,
    /// Evidence packets cite this packet by stable run id.
    pub evidence_packets_use_stable_run_id: bool,
    /// Docs and help material cite the governed contract.
    pub docs_help_cite_governed_contract: bool,
    /// Preview/Labs lanes are visibly labeled when not covered by this packet.
    pub preview_labs_label_for_unqualified_lanes: bool,
}

/// Root packet qualifying the background branch-agent lifecycle.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BackgroundBranchAgentLifecyclePacket {
    /// Record kind; must equal [`BACKGROUND_BRANCH_AGENT_LIFECYCLE_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal
    /// [`BACKGROUND_BRANCH_AGENT_LIFECYCLE_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Unique packet id.
    pub packet_id: String,
    /// Stable run id used across UI, export, support, and evidence packets.
    pub stable_run_id: String,
    /// Plan version the run is executing.
    pub plan_version: String,
    /// Initiator class or actor label.
    pub initiator: String,
    /// Branch identity ref.
    pub branch_identity_ref: String,
    /// Worktree identity ref.
    pub worktree_identity_ref: String,
    /// Base commit ref.
    pub base_commit_ref: String,
    /// Requested goal.
    pub requested_goal: String,
    /// Current lifecycle state.
    pub current_state: BranchAgentRunState,
    /// Current host/execution locus.
    pub current_execution_locus: BranchAgentExecutionLocus,
    /// Pending approval refs for the run.
    pub pending_approvals: Vec<String>,
    /// Evidence refs for the run.
    pub evidence_refs: Vec<String>,
    /// Cancellation posture.
    pub cancellation_posture: BranchAgentCancellationPosture,
    /// Cleanup posture for the run.
    pub cleanup_posture: BranchAgentCleanupDisposition,
    /// Launch review sheet shown before execution begins.
    pub launch_review: BranchAgentLaunchReviewSheet,
    /// Active run rows projected in the UI and exports.
    pub active_run_rows: Vec<BranchAgentActiveRunRow>,
    /// Checkpoint rows for review and recovery.
    pub checkpoint_rows: Vec<BranchAgentCheckpointRow>,
    /// Drift drills proving re-review behavior.
    pub drift_drills: Vec<BranchAgentDriftDrillRow>,
    /// Operator takeover row.
    pub takeover: BranchAgentTakeoverRow,
    /// Completion review row.
    pub completion_review: BranchAgentCompletionReview,
    /// Cleanup rows.
    pub cleanup_rows: Vec<BranchAgentCleanupRow>,
    /// Security and policy review block.
    pub security_review: BranchAgentSecurityReview,
    /// Consumer projection block.
    pub consumer_projection: BranchAgentConsumerProjection,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Redaction class token for support export.
    pub redaction_class_token: String,
    /// RFC 3339 timestamp when this packet was minted.
    pub minted_at: String,
}

/// An invariant violation found by [`BackgroundBranchAgentLifecyclePacket::validate`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BackgroundBranchAgentLifecycleViolation {
    /// The record_kind field does not match the expected constant.
    WrongRecordKind { found: String },
    /// The schema_version field does not match the expected constant.
    WrongSchemaVersion { found: u32 },
    /// A required identity field is empty.
    MissingIdentity { field: &'static str },
    /// A row does not carry the packet stable run id.
    RunIdMismatch { row: &'static str, found: String },
    /// Launch review is missing required pre-dispatch disclosure.
    LaunchReviewIncomplete { field: &'static str },
    /// No active run rows were provided.
    NoActiveRunRows,
    /// No checkpoint rows were provided.
    NoCheckpointRows,
    /// A row is missing required operator actions.
    MissingOperatorActions { row: &'static str },
    /// A checkpoint row does not preserve review artifacts.
    CheckpointDoesNotPreserveArtifacts { checkpoint_id: String },
    /// A required drift trigger is absent.
    MissingDriftTrigger { trigger: BranchAgentDriftTrigger },
    /// A drift drill does not pause or narrow safely.
    DriftDoesNotPauseOrNarrow { trigger: BranchAgentDriftTrigger },
    /// A drift drill permits further writes before re-review.
    DriftAllowsFurtherWrites { trigger: BranchAgentDriftTrigger },
    /// A drift drill loses already-produced artifacts.
    DriftLosesArtifacts { trigger: BranchAgentDriftTrigger },
    /// A drift drill does not require re-review or takeover.
    DriftMissingReReview { trigger: BranchAgentDriftTrigger },
    /// Operator takeover does not preserve required lineage.
    TakeoverLineageIncomplete,
    /// Completion review is missing required actions.
    CompletionReviewIncomplete { field: &'static str },
    /// The packet permits self-merge or protected destination self-push.
    UnsafeLandingPosture,
    /// Cleanup does not preserve reviewable artifacts.
    CleanupLosesReviewArtifacts,
    /// Security review does not satisfy required invariants.
    SecurityReviewIncomplete { field: &'static str },
    /// Consumer projection does not preserve stable run attribution.
    ConsumerProjectionIncomplete { field: &'static str },
    /// A required source contract ref is absent.
    MissingSourceContract { reference: &'static str },
}

impl fmt::Display for BackgroundBranchAgentLifecycleViolation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::WrongRecordKind { found } => write!(
                f,
                "record_kind is {found:?}, expected {:?}",
                BACKGROUND_BRANCH_AGENT_LIFECYCLE_RECORD_KIND
            ),
            Self::WrongSchemaVersion { found } => write!(
                f,
                "schema_version is {found}, expected {BACKGROUND_BRANCH_AGENT_LIFECYCLE_SCHEMA_VERSION}"
            ),
            Self::MissingIdentity { field } => write!(f, "missing required identity field {field}"),
            Self::RunIdMismatch { row, found } => {
                write!(f, "{row} has stable_run_id {found:?} that does not match packet")
            }
            Self::LaunchReviewIncomplete { field } => {
                write!(f, "launch review is missing required field {field}")
            }
            Self::NoActiveRunRows => write!(f, "packet must include at least one active run row"),
            Self::NoCheckpointRows => write!(f, "packet must include at least one checkpoint row"),
            Self::MissingOperatorActions { row } => {
                write!(f, "{row} is missing required operator actions")
            }
            Self::CheckpointDoesNotPreserveArtifacts { checkpoint_id } => {
                write!(f, "checkpoint {checkpoint_id:?} does not preserve artifacts")
            }
            Self::MissingDriftTrigger { trigger } => {
                write!(f, "missing drift trigger {:?}", trigger.as_str())
            }
            Self::DriftDoesNotPauseOrNarrow { trigger } => {
                write!(f, "drift {:?} does not pause or narrow safely", trigger.as_str())
            }
            Self::DriftAllowsFurtherWrites { trigger } => {
                write!(f, "drift {:?} allows further writes", trigger.as_str())
            }
            Self::DriftLosesArtifacts { trigger } => {
                write!(f, "drift {:?} loses review artifacts", trigger.as_str())
            }
            Self::DriftMissingReReview { trigger } => {
                write!(f, "drift {:?} does not require re-review", trigger.as_str())
            }
            Self::TakeoverLineageIncomplete => {
                write!(f, "operator takeover does not preserve required lineage")
            }
            Self::CompletionReviewIncomplete { field } => {
                write!(f, "completion review is missing {field}")
            }
            Self::UnsafeLandingPosture => {
                write!(f, "completion review permits unsafe landing posture")
            }
            Self::CleanupLosesReviewArtifacts => {
                write!(f, "cleanup loses reviewable artifacts")
            }
            Self::SecurityReviewIncomplete { field } => {
                write!(f, "security review is missing invariant {field}")
            }
            Self::ConsumerProjectionIncomplete { field } => {
                write!(f, "consumer projection is missing invariant {field}")
            }
            Self::MissingSourceContract { reference } => {
                write!(f, "missing source contract ref {reference}")
            }
        }
    }
}

impl Error for BackgroundBranchAgentLifecycleViolation {}

/// Error returned while loading the checked lifecycle artifact.
#[derive(Debug)]
pub enum BackgroundBranchAgentLifecycleArtifactError {
    /// The checked support export is malformed JSON.
    SupportExport(serde_json::Error),
    /// The checked support export parsed but violated the packet contract.
    Validation(Vec<BackgroundBranchAgentLifecycleViolation>),
}

impl fmt::Display for BackgroundBranchAgentLifecycleArtifactError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => write!(f, "failed to parse support export: {error}"),
            Self::Validation(violations) => {
                write!(f, "support export failed validation: {violations:?}")
            }
        }
    }
}

impl Error for BackgroundBranchAgentLifecycleArtifactError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::SupportExport(error) => Some(error),
            Self::Validation(_) => None,
        }
    }
}

impl BackgroundBranchAgentLifecyclePacket {
    /// Validates the lifecycle qualification packet.
    pub fn validate(&self) -> Vec<BackgroundBranchAgentLifecycleViolation> {
        let mut violations = Vec::new();

        if self.record_kind != BACKGROUND_BRANCH_AGENT_LIFECYCLE_RECORD_KIND {
            violations.push(BackgroundBranchAgentLifecycleViolation::WrongRecordKind {
                found: self.record_kind.clone(),
            });
        }
        if self.schema_version != BACKGROUND_BRANCH_AGENT_LIFECYCLE_SCHEMA_VERSION {
            violations.push(
                BackgroundBranchAgentLifecycleViolation::WrongSchemaVersion {
                    found: self.schema_version,
                },
            );
        }

        for (field, value) in [
            ("packet_id", self.packet_id.as_str()),
            ("stable_run_id", self.stable_run_id.as_str()),
            ("plan_version", self.plan_version.as_str()),
            ("initiator", self.initiator.as_str()),
            ("branch_identity_ref", self.branch_identity_ref.as_str()),
            ("worktree_identity_ref", self.worktree_identity_ref.as_str()),
            ("base_commit_ref", self.base_commit_ref.as_str()),
            ("requested_goal", self.requested_goal.as_str()),
        ] {
            if value.is_empty() {
                violations.push(BackgroundBranchAgentLifecycleViolation::MissingIdentity { field });
            }
        }

        validate_source_contracts(self, &mut violations);
        self.validate_launch_review(&mut violations);
        self.validate_rows(&mut violations);
        self.validate_drift_drills(&mut violations);
        self.validate_takeover(&mut violations);
        self.validate_completion_review(&mut violations);
        self.validate_cleanup(&mut violations);
        self.validate_security_review(&mut violations);
        self.validate_consumer_projection(&mut violations);

        violations
    }

    /// Serializes the packet as pretty JSON for checked fixtures and support export.
    pub fn export_safe_json(&self) -> String {
        serde_json::to_string_pretty(self).expect("lifecycle packet serializes")
    }

    fn validate_launch_review(
        &self,
        violations: &mut Vec<BackgroundBranchAgentLifecycleViolation>,
    ) {
        validate_run_id(
            "launch_review",
            &self.launch_review.stable_run_id,
            &self.stable_run_id,
            violations,
        );
        for (field, value) in [
            ("requested_goal", self.launch_review.requested_goal.as_str()),
            ("base_ref", self.launch_review.base_ref.as_str()),
            (
                "target_identity_ref",
                self.launch_review.target_identity_ref.as_str(),
            ),
            (
                "estimated_cost_risk_band",
                self.launch_review.estimated_cost_risk_band.as_str(),
            ),
            ("secret_scope", self.launch_review.secret_scope.as_str()),
        ] {
            if value.is_empty() {
                violations.push(
                    BackgroundBranchAgentLifecycleViolation::LaunchReviewIncomplete { field },
                );
            }
        }
        if self.launch_review.tool_connector_classes.is_empty() {
            violations.push(
                BackgroundBranchAgentLifecycleViolation::LaunchReviewIncomplete {
                    field: "tool_connector_classes",
                },
            );
        }
        if self.launch_review.approval_gates.is_empty() {
            violations.push(
                BackgroundBranchAgentLifecycleViolation::LaunchReviewIncomplete {
                    field: "approval_gates",
                },
            );
        }
        if self.launch_review.stop_conditions.is_empty() {
            violations.push(
                BackgroundBranchAgentLifecycleViolation::LaunchReviewIncomplete {
                    field: "stop_conditions",
                },
            );
        }
    }

    fn validate_rows(&self, violations: &mut Vec<BackgroundBranchAgentLifecycleViolation>) {
        if self.active_run_rows.is_empty() {
            violations.push(BackgroundBranchAgentLifecycleViolation::NoActiveRunRows);
        }
        for row in &self.active_run_rows {
            validate_run_id(
                "active_run_row",
                &row.stable_run_id,
                &self.stable_run_id,
                violations,
            );
            if row.operator_actions.is_empty() {
                violations.push(
                    BackgroundBranchAgentLifecycleViolation::MissingOperatorActions {
                        row: "active_run_row",
                    },
                );
            }
        }

        if self.checkpoint_rows.is_empty() {
            violations.push(BackgroundBranchAgentLifecycleViolation::NoCheckpointRows);
        }
        for row in &self.checkpoint_rows {
            validate_run_id(
                "checkpoint_row",
                &row.stable_run_id,
                &self.stable_run_id,
                violations,
            );
            if row.operator_actions.is_empty() {
                violations.push(
                    BackgroundBranchAgentLifecycleViolation::MissingOperatorActions {
                        row: "checkpoint_row",
                    },
                );
            }
            if !row.review_artifacts_preserved {
                violations.push(
                    BackgroundBranchAgentLifecycleViolation::CheckpointDoesNotPreserveArtifacts {
                        checkpoint_id: row.checkpoint_id.clone(),
                    },
                );
            }
        }
    }

    fn validate_drift_drills(&self, violations: &mut Vec<BackgroundBranchAgentLifecycleViolation>) {
        for required in BranchAgentDriftTrigger::required_coverage() {
            if !self.drift_drills.iter().any(|row| row.trigger == required) {
                violations.push(
                    BackgroundBranchAgentLifecycleViolation::MissingDriftTrigger {
                        trigger: required,
                    },
                );
            }
        }

        for row in &self.drift_drills {
            validate_run_id(
                "drift_drill_row",
                &row.stable_run_id,
                &self.stable_run_id,
                violations,
            );
            if !row.pauses_or_narrows_safely {
                violations.push(
                    BackgroundBranchAgentLifecycleViolation::DriftDoesNotPauseOrNarrow {
                        trigger: row.trigger,
                    },
                );
            }
            if !row.further_writes_blocked {
                violations.push(
                    BackgroundBranchAgentLifecycleViolation::DriftAllowsFurtherWrites {
                        trigger: row.trigger,
                    },
                );
            }
            if !row.already_produced_artifacts_preserved {
                violations.push(
                    BackgroundBranchAgentLifecycleViolation::DriftLosesArtifacts {
                        trigger: row.trigger,
                    },
                );
            }
            if !row.requires_re_review_or_takeover {
                violations.push(
                    BackgroundBranchAgentLifecycleViolation::DriftMissingReReview {
                        trigger: row.trigger,
                    },
                );
            }
        }
    }

    fn validate_takeover(&self, violations: &mut Vec<BackgroundBranchAgentLifecycleViolation>) {
        validate_run_id(
            "takeover",
            &self.takeover.stable_run_id,
            &self.stable_run_id,
            violations,
        );
        if !(self.takeover.branch_identity_preserved
            && self.takeover.checkpoint_lineage_preserved
            && self.takeover.tool_call_history_preserved
            && self.takeover.validation_receipts_preserved
            && self.takeover.pending_writes_disclosed)
        {
            violations.push(BackgroundBranchAgentLifecycleViolation::TakeoverLineageIncomplete);
        }
        if !self
            .takeover
            .rerun_options
            .contains(&BranchAgentOperatorAction::ResumeFromCheckpoint)
            || !self
                .takeover
                .rerun_options
                .contains(&BranchAgentOperatorAction::TakeOverManually)
        {
            violations.push(
                BackgroundBranchAgentLifecycleViolation::MissingOperatorActions { row: "takeover" },
            );
        }
    }

    fn validate_completion_review(
        &self,
        violations: &mut Vec<BackgroundBranchAgentLifecycleViolation>,
    ) {
        validate_run_id(
            "completion_review",
            &self.completion_review.stable_run_id,
            &self.stable_run_id,
            violations,
        );
        for (field, value) in [
            (
                "diff_summary_ref",
                self.completion_review.diff_summary_ref.as_str(),
            ),
            (
                "validation_summary_ref",
                self.completion_review.validation_summary_ref.as_str(),
            ),
            (
                "evidence_packet_ref",
                self.completion_review.evidence_packet_ref.as_str(),
            ),
        ] {
            if value.is_empty() {
                violations.push(
                    BackgroundBranchAgentLifecycleViolation::CompletionReviewIncomplete { field },
                );
            }
        }
        if !self.completion_review.compare_to_base_available {
            violations.push(
                BackgroundBranchAgentLifecycleViolation::CompletionReviewIncomplete {
                    field: "compare_to_base_available",
                },
            );
        }
        if !self.completion_review.cleanup_options_available {
            violations.push(
                BackgroundBranchAgentLifecycleViolation::CompletionReviewIncomplete {
                    field: "cleanup_options_available",
                },
            );
        }
        for action in [
            BranchAgentOperatorAction::OpenReview,
            BranchAgentOperatorAction::RerunValidation,
            BranchAgentOperatorAction::DiscardBranch,
        ] {
            if !self.completion_review.follow_up_actions.contains(&action) {
                violations.push(
                    BackgroundBranchAgentLifecycleViolation::CompletionReviewIncomplete {
                        field: action.as_str(),
                    },
                );
            }
        }
        if !(self.completion_review.self_merge_forbidden
            && self.completion_review.protected_destination_push_forbidden)
        {
            violations.push(BackgroundBranchAgentLifecycleViolation::UnsafeLandingPosture);
        }
    }

    fn validate_cleanup(&self, violations: &mut Vec<BackgroundBranchAgentLifecycleViolation>) {
        if self.cleanup_rows.is_empty() {
            violations.push(BackgroundBranchAgentLifecycleViolation::CleanupLosesReviewArtifacts);
        }
        for row in &self.cleanup_rows {
            validate_run_id(
                "cleanup_row",
                &row.stable_run_id,
                &self.stable_run_id,
                violations,
            );
            if !(row.preview_required_before_delete && row.review_artifacts_survive_cleanup) {
                violations
                    .push(BackgroundBranchAgentLifecycleViolation::CleanupLosesReviewArtifacts);
            }
        }
    }

    fn validate_security_review(
        &self,
        violations: &mut Vec<BackgroundBranchAgentLifecycleViolation>,
    ) {
        for (field, ok) in [
            (
                "no_self_approved_mutating_tools",
                self.security_review.no_self_approved_mutating_tools,
            ),
            (
                "no_worktree_isolation_bypass",
                self.security_review.no_worktree_isolation_bypass,
            ),
            (
                "execution_loci_not_collapsed",
                self.security_review.execution_loci_not_collapsed,
            ),
            (
                "local_side_and_managed_loci_distinct",
                self.security_review.local_side_and_managed_loci_distinct,
            ),
            (
                "protected_destination_self_push_blocked",
                self.security_review.protected_destination_self_push_blocked,
            ),
        ] {
            if !ok {
                violations.push(
                    BackgroundBranchAgentLifecycleViolation::SecurityReviewIncomplete { field },
                );
            }
        }
    }

    fn validate_consumer_projection(
        &self,
        violations: &mut Vec<BackgroundBranchAgentLifecycleViolation>,
    ) {
        for (field, ok) in [
            (
                "ui_rows_use_stable_run_id",
                self.consumer_projection.ui_rows_use_stable_run_id,
            ),
            (
                "support_exports_use_stable_run_id",
                self.consumer_projection.support_exports_use_stable_run_id,
            ),
            (
                "evidence_packets_use_stable_run_id",
                self.consumer_projection.evidence_packets_use_stable_run_id,
            ),
            (
                "docs_help_cite_governed_contract",
                self.consumer_projection.docs_help_cite_governed_contract,
            ),
            (
                "preview_labs_label_for_unqualified_lanes",
                self.consumer_projection
                    .preview_labs_label_for_unqualified_lanes,
            ),
        ] {
            if !ok {
                violations.push(
                    BackgroundBranchAgentLifecycleViolation::ConsumerProjectionIncomplete { field },
                );
            }
        }
    }
}

/// Returns the checked-in background branch-agent lifecycle export.
///
/// # Errors
///
/// Returns an artifact error if the checked-in export does not parse or
/// validate.
pub fn current_stable_background_branch_agent_lifecycle_export(
) -> Result<BackgroundBranchAgentLifecyclePacket, BackgroundBranchAgentLifecycleArtifactError> {
    let packet: BackgroundBranchAgentLifecyclePacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/ai/m4/qualify-background-branch-agent-lifecycle/support_export.json"
    )))
    .map_err(BackgroundBranchAgentLifecycleArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(BackgroundBranchAgentLifecycleArtifactError::Validation(
            violations,
        ))
    }
}

fn validate_run_id(
    row: &'static str,
    found: &str,
    expected: &str,
    violations: &mut Vec<BackgroundBranchAgentLifecycleViolation>,
) {
    if found != expected {
        violations.push(BackgroundBranchAgentLifecycleViolation::RunIdMismatch {
            row,
            found: found.to_owned(),
        });
    }
}

fn validate_source_contracts(
    packet: &BackgroundBranchAgentLifecyclePacket,
    violations: &mut Vec<BackgroundBranchAgentLifecycleViolation>,
) {
    for reference in [
        BACKGROUND_BRANCH_AGENT_LIFECYCLE_AI_DOC_REF,
        BACKGROUND_BRANCH_AGENT_BASE_CONTRACT_REF,
        BACKGROUND_BRANCH_AGENT_LIFECYCLE_SCHEMA_REF,
    ] {
        if !packet
            .source_contract_refs
            .iter()
            .any(|candidate| candidate == reference)
        {
            violations
                .push(BackgroundBranchAgentLifecycleViolation::MissingSourceContract { reference });
        }
    }
}
