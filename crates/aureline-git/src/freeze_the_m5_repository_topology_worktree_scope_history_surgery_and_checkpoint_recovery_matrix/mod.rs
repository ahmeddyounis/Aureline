//! Frozen M5 repository-topology, worktree-scope, history-surgery, and
//! checkpoint-recovery matrix.
//!
//! This module locks the canonical M5 Git depth contract for repositories that
//! are sparse, partial-cloned, shallow, submodule-backed, nested-independent,
//! worktree-rooted, Git-LFS pointer-backed, or intentionally generated/vendor
//! excluded, and for the risky history-surgery sessions those repositories
//! support (conflict resolution, sequence edits, recovery checkpoints, publish
//! ref-update proposals, and stash shelves). Each row binds one topology class
//! or session object to the controlled degraded vocabulary it may surface, the
//! mutation scope it permits, the preview that must precede a mutation, the
//! recovery class that must remain reachable, and the consumer surfaces that
//! must be able to express the same truth.
//!
//! The matrix is the single source of truth for whether an M5 Git or
//! source-acquisition surface may claim complete coverage, target a root for
//! mutation, or run a risky history operation. It references the canonical
//! topology, conflict-session, sequence-edit, recovery-checkpoint, stash, and
//! ref-update contracts by id rather than redefining or embedding them, so
//! provider overlay, AI context, search, review, CLI, and support/export flows
//! all read one vocabulary. Provider overlays never overwrite local Git truth,
//! and recovery checkpoints or reflog-only fallbacks stay visible before any
//! destructive operation.
//!
//! Topology truth is never reduced to a badge: the rows control actual
//! mutation, preview, recovery, and export behavior. Raw paths, raw object
//! bytes, raw branch names, raw patch/reflog/stash bodies, raw provider
//! payloads, and credentials stay outside the support boundary.
//!
//! The boundary schema is
//! [`schemas/git/freeze-the-m5-repository-topology-worktree-scope-history-surgery-and-checkpoint-recovery-matrix.schema.json`](../../../../schemas/git/freeze-the-m5-repository-topology-worktree-scope-history-surgery-and-checkpoint-recovery-matrix.schema.json).
//! The contract doc is
//! [`docs/git/m5/freeze_the_m5_repository_topology_worktree_scope_history_surgery_and_checkpoint_recovery_matrix.md`](../../../../docs/git/m5/freeze_the_m5_repository_topology_worktree_scope_history_surgery_and_checkpoint_recovery_matrix.md).
//! The protected fixture directory is
//! [`fixtures/git/m5/freeze_the_m5_repository_topology_worktree_scope_history_surgery_and_checkpoint_recovery_matrix/`](../../../../fixtures/git/m5/freeze_the_m5_repository_topology_worktree_scope_history_surgery_and_checkpoint_recovery_matrix/).

#[cfg(test)]
mod tests;

use std::collections::{BTreeSet, HashSet};
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

use crate::finalize_sequence_edit_conflict_session_stash_entry_and_ref_update_truth::{
    RISKY_VCS_CONFLICT_SESSION_RECORD_KIND, RISKY_VCS_RECOVERY_CHECKPOINT_RECORD_KIND,
    RISKY_VCS_REF_UPDATE_PROPOSAL_RECORD_KIND, RISKY_VCS_SEQUENCE_EDIT_SESSION_RECORD_KIND,
    RISKY_VCS_STASH_ENTRY_RECORD_KIND,
};
use crate::stabilize_repository_topology_truth::{
    RepositoryTopologyClass, TopologyHonestyLabel, TopologyOperationScope,
};

/// Stable record-kind tag carried by [`M5GitTopologyHistoryMatrixPacket`].
pub const M5_GIT_TOPOLOGY_HISTORY_MATRIX_RECORD_KIND: &str =
    "freeze_m5_repository_topology_worktree_scope_history_surgery_and_checkpoint_recovery_matrix";

/// Schema version for M5 Git topology and history-surgery matrix records.
pub const M5_GIT_TOPOLOGY_HISTORY_MATRIX_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the boundary schema.
pub const M5_GIT_TOPOLOGY_HISTORY_MATRIX_SCHEMA_REF: &str =
    "schemas/git/freeze-the-m5-repository-topology-worktree-scope-history-surgery-and-checkpoint-recovery-matrix.schema.json";

/// Repo-relative path of the contract doc.
pub const M5_GIT_TOPOLOGY_HISTORY_MATRIX_DOC_REF: &str =
    "docs/git/m5/freeze_the_m5_repository_topology_worktree_scope_history_surgery_and_checkpoint_recovery_matrix.md";

/// Repo-relative path of the frozen repository-topology truth contract.
pub const M5_GIT_TOPOLOGY_HISTORY_MATRIX_TOPOLOGY_CONTRACT_REF: &str =
    "schemas/review/repository-topology.schema.json";

/// Repo-relative path of the frozen conflict-session contract.
pub const M5_GIT_TOPOLOGY_HISTORY_MATRIX_CONFLICT_SESSION_CONTRACT_REF: &str =
    "schemas/git/conflict_session.schema.json";

/// Repo-relative path of the frozen sequence-edit-session contract.
pub const M5_GIT_TOPOLOGY_HISTORY_MATRIX_SEQUENCE_EDIT_CONTRACT_REF: &str =
    "schemas/git/sequence_edit_session.schema.json";

/// Repo-relative path of the frozen recovery-checkpoint contract.
pub const M5_GIT_TOPOLOGY_HISTORY_MATRIX_RECOVERY_CHECKPOINT_CONTRACT_REF: &str =
    "schemas/git/recovery_checkpoint.schema.json";

/// Repo-relative path of the frozen stash-entry contract.
pub const M5_GIT_TOPOLOGY_HISTORY_MATRIX_STASH_CONTRACT_REF: &str =
    "schemas/git/stash_entry.schema.json";

/// Repo-relative path of the frozen risky-VCS ref-update lineage contract.
pub const M5_GIT_TOPOLOGY_HISTORY_MATRIX_REF_UPDATE_CONTRACT_REF: &str =
    "schemas/review/sequence-edit-conflict-session-stash-entry-ref-update.schema.json";

/// Repo-relative path of the protected fixture directory.
pub const M5_GIT_TOPOLOGY_HISTORY_MATRIX_FIXTURE_DIR: &str =
    "fixtures/git/m5/freeze_the_m5_repository_topology_worktree_scope_history_surgery_and_checkpoint_recovery_matrix";

/// Repo-relative path of the checked support-export artifact.
pub const M5_GIT_TOPOLOGY_HISTORY_MATRIX_ARTIFACT_REF: &str =
    "artifacts/git/m5/freeze_the_m5_repository_topology_worktree_scope_history_surgery_and_checkpoint_recovery_matrix/support_export.json";

/// Repo-relative path of the checked Markdown summary.
pub const M5_GIT_TOPOLOGY_HISTORY_MATRIX_SUMMARY_REF: &str =
    "artifacts/git/m5/freeze_the_m5_repository_topology_worktree_scope_history_surgery_and_checkpoint_recovery_matrix.md";

/// Frozen topology classes the matrix must cover, in canonical order.
///
/// These are the eight topology classes that materially change product truth
/// and recovery posture; the two plain roots
/// ([`RepositoryTopologyClass::CurrentRepoRoot`] and
/// [`RepositoryTopologyClass::WorksetRoot`]) carry no degraded vocabulary and
/// are governed by the upstream topology-truth packet directly.
pub const M5_GIT_TOPOLOGY_HISTORY_MATRIX_REQUIRED_CLASSES: [RepositoryTopologyClass; 8] = [
    RepositoryTopologyClass::SparseCheckoutRoot,
    RepositoryTopologyClass::PartialClonePromisorRoot,
    RepositoryTopologyClass::ShallowHistoryRoot,
    RepositoryTopologyClass::SubmoduleRoot,
    RepositoryTopologyClass::NestedIndependentRepoRoot,
    RepositoryTopologyClass::WorktreeRoot,
    RepositoryTopologyClass::LfsHydrationBoundary,
    RepositoryTopologyClass::GeneratedVendorRoot,
];

/// History-surgery session object frozen by this matrix.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum HistorySurgerySession {
    /// In-progress conflict resolution for a merge, rebase, cherry-pick, etc.
    ConflictSession,
    /// Persisted interactive-rebase / sequence-edit todo session.
    SequenceEditSession,
    /// Pre-mutation checkpoint captured so a risky operation can be undone.
    RecoveryCheckpoint,
    /// A reviewable proposal to move a published ref (push / force-with-lease).
    PublishRefUpdateProposal,
    /// A captured stash shelf entry with explicit apply/pop/drop/branch scope.
    StashShelfEntry,
}

impl HistorySurgerySession {
    /// Every session object, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::ConflictSession,
        Self::SequenceEditSession,
        Self::RecoveryCheckpoint,
        Self::PublishRefUpdateProposal,
        Self::StashShelfEntry,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ConflictSession => "conflict_session",
            Self::SequenceEditSession => "sequence_edit_session",
            Self::RecoveryCheckpoint => "recovery_checkpoint",
            Self::PublishRefUpdateProposal => "publish_ref_update_proposal",
            Self::StashShelfEntry => "stash_shelf_entry",
        }
    }

    /// Canonical record-kind tag for the object this row governs.
    ///
    /// The matrix references the existing risky-VCS lineage objects by their
    /// frozen record-kind rather than redefining them, so every surface reads
    /// one vocabulary.
    pub const fn canonical_record_kind(self) -> &'static str {
        match self {
            Self::ConflictSession => RISKY_VCS_CONFLICT_SESSION_RECORD_KIND,
            Self::SequenceEditSession => RISKY_VCS_SEQUENCE_EDIT_SESSION_RECORD_KIND,
            Self::RecoveryCheckpoint => RISKY_VCS_RECOVERY_CHECKPOINT_RECORD_KIND,
            Self::PublishRefUpdateProposal => RISKY_VCS_REF_UPDATE_PROPOSAL_RECORD_KIND,
            Self::StashShelfEntry => RISKY_VCS_STASH_ENTRY_RECORD_KIND,
        }
    }

    /// Whether this object is itself the recovery surface (and so does not
    /// require a separate pre-mutation checkpoint).
    pub const fn is_recovery_object(self) -> bool {
        matches!(self, Self::RecoveryCheckpoint)
    }
}

/// Controlled degraded vocabulary a topology row may surface.
///
/// These states are distinct and may not collapse into a single "incomplete"
/// badge; each narrows a coverage claim and may gate mutation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DegradedTopologyState {
    /// Content is omitted by the active sparse slice or workset scope.
    Omitted,
    /// A known promisor/partial-clone object is not materialized locally.
    Unfetched,
    /// A submodule child checkout is not initialized.
    Uninitialized,
    /// Only Git LFS pointer metadata is available for the object.
    PointerOnly,
    /// A child root (submodule or nested repo) has uncommitted changes.
    DirtyChild,
    /// No checkpoint exists; only a reflog-only recovery fallback is offered.
    ReflogOnlyFallback,
    /// A provider overlay is stale relative to local Git truth.
    StaleProviderOverlay,
}

impl DegradedTopologyState {
    /// Every degraded state, in declaration order.
    pub const ALL: [Self; 7] = [
        Self::Omitted,
        Self::Unfetched,
        Self::Uninitialized,
        Self::PointerOnly,
        Self::DirtyChild,
        Self::ReflogOnlyFallback,
        Self::StaleProviderOverlay,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Omitted => "omitted",
            Self::Unfetched => "unfetched",
            Self::Uninitialized => "uninitialized",
            Self::PointerOnly => "pointer_only",
            Self::DirtyChild => "dirty_child",
            Self::ReflogOnlyFallback => "reflog_only_fallback",
            Self::StaleProviderOverlay => "stale_provider_overlay",
        }
    }
}

/// Preview class that must precede a mutation governed by a row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum OperationPreviewClass {
    /// Read-only row; no mutation and so no preview is required.
    NoPreviewReadOnly,
    /// Widening a sparse/workset scope is previewed before it applies.
    ScopeWidenPreview,
    /// Fetching missing objects or deepening history is previewed.
    FetchOrDeepenPreview,
    /// A working-tree/index diff is previewed before commit-class mutation.
    DiffPreview,
    /// The full sequence-edit plan is previewed before it runs.
    SequencePlanPreview,
    /// A ref-update before/after position is previewed before publish.
    RefUpdatePreview,
    /// A multi-root or parent-plus-child operation is previewed.
    MultiRootPreview,
    /// A destructive history rewrite or reset is previewed in full.
    DestructiveRewritePreview,
}

impl OperationPreviewClass {
    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NoPreviewReadOnly => "no_preview_read_only",
            Self::ScopeWidenPreview => "scope_widen_preview",
            Self::FetchOrDeepenPreview => "fetch_or_deepen_preview",
            Self::DiffPreview => "diff_preview",
            Self::SequencePlanPreview => "sequence_plan_preview",
            Self::RefUpdatePreview => "ref_update_preview",
            Self::MultiRootPreview => "multi_root_preview",
            Self::DestructiveRewritePreview => "destructive_rewrite_preview",
        }
    }

    /// Whether this preview class actually gates a mutation.
    pub const fn is_mutation_preview(self) -> bool {
        !matches!(self, Self::NoPreviewReadOnly)
    }
}

/// Recovery class that must remain reachable for a row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum OperationRecoveryClass {
    /// Read-only row; nothing is mutated, so no recovery is needed.
    NoMutationNoRecovery,
    /// A recovery checkpoint is captured before the mutation runs.
    CheckpointBeforeMutation,
    /// The original state is preserved on a restorable stash shelf.
    StashShelfRestore,
    /// A published ref-update can be rolled back to its prior position.
    RefUpdateRollback,
    /// No checkpoint is possible; a reflog-only fallback is disclosed.
    ReflogOnlyFallbackDisclosed,
    /// No recovery is possible, so the operation is blocked until resolved.
    NoRecoveryOperationBlocked,
}

impl OperationRecoveryClass {
    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NoMutationNoRecovery => "no_mutation_no_recovery",
            Self::CheckpointBeforeMutation => "checkpoint_before_mutation",
            Self::StashShelfRestore => "stash_shelf_restore",
            Self::RefUpdateRollback => "ref_update_rollback",
            Self::ReflogOnlyFallbackDisclosed => "reflog_only_fallback_disclosed",
            Self::NoRecoveryOperationBlocked => "no_recovery_operation_blocked",
        }
    }

    /// Whether this class provides a reachable recovery path for a mutation.
    pub const fn provides_recovery(self) -> bool {
        matches!(
            self,
            Self::CheckpointBeforeMutation
                | Self::StashShelfRestore
                | Self::RefUpdateRollback
                | Self::ReflogOnlyFallbackDisclosed
        )
    }
}

/// Consumer surface that must be able to express the matrix vocabulary.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MatrixConsumerSurface {
    /// Hosted provider overlay (status, PR, checks) layered over local truth.
    ProviderOverlay,
    /// AI-context assembly and evidence inspectors.
    AiContext,
    /// Search result and zero-result rows.
    Search,
    /// Review diff, summary, publish, and history-edit rows.
    Review,
    /// Redaction-safe support / export rows.
    SupportExport,
    /// CLI / headless replay or JSON output.
    Cli,
    /// Shell chrome, activity center, and status rows.
    Shell,
    /// Diagnostics or telemetry surface.
    Diagnostics,
}

impl MatrixConsumerSurface {
    /// Every surface, in declaration order.
    pub const ALL: [Self; 8] = [
        Self::ProviderOverlay,
        Self::AiContext,
        Self::Search,
        Self::Review,
        Self::SupportExport,
        Self::Cli,
        Self::Shell,
        Self::Diagnostics,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ProviderOverlay => "provider_overlay",
            Self::AiContext => "ai_context",
            Self::Search => "search",
            Self::Review => "review",
            Self::SupportExport => "support_export",
            Self::Cli => "cli",
            Self::Shell => "shell",
            Self::Diagnostics => "diagnostics",
        }
    }
}

/// Risky history operation that must carry an explicit preview and recovery.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RiskyHistoryOperation {
    /// Merge of one ref into another.
    Merge,
    /// Non-interactive rebase.
    Rebase,
    /// Interactive rebase / sequence edit.
    InteractiveRebase,
    /// Cherry-pick of one or more commits.
    CherryPick,
    /// Revert of one or more commits.
    Revert,
    /// Reset of HEAD/index/worktree.
    Reset,
    /// Apply a stash, keeping it.
    StashApply,
    /// Pop a stash, removing it on success.
    StashPop,
    /// Drop a stash without applying it.
    StashDrop,
    /// Create a branch from a stash.
    BranchFromStash,
    /// Publish to a remote / provider.
    Publish,
    /// Force-with-lease publish that rewrites a remote ref.
    ForcePushWithLease,
}

impl RiskyHistoryOperation {
    /// Every risky operation, in declaration order.
    pub const ALL: [Self; 12] = [
        Self::Merge,
        Self::Rebase,
        Self::InteractiveRebase,
        Self::CherryPick,
        Self::Revert,
        Self::Reset,
        Self::StashApply,
        Self::StashPop,
        Self::StashDrop,
        Self::BranchFromStash,
        Self::Publish,
        Self::ForcePushWithLease,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Merge => "merge",
            Self::Rebase => "rebase",
            Self::InteractiveRebase => "interactive_rebase",
            Self::CherryPick => "cherry_pick",
            Self::Revert => "revert",
            Self::Reset => "reset",
            Self::StashApply => "stash_apply",
            Self::StashPop => "stash_pop",
            Self::StashDrop => "stash_drop",
            Self::BranchFromStash => "branch_from_stash",
            Self::Publish => "publish",
            Self::ForcePushWithLease => "force_push_with_lease",
        }
    }
}

/// One topology-class row in the matrix.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TopologyClassRow {
    /// Canonical topology class governed by this row.
    pub topology_class: RepositoryTopologyClass,
    /// Human-readable scope summary.
    pub scope_summary: String,
    /// Honesty label downstream rows must preserve.
    pub honesty_label: TopologyHonestyLabel,
    /// Controlled degraded states this class may surface.
    pub degraded_states: Vec<DegradedTopologyState>,
    /// Mutation/export scope this class permits.
    pub mutation_control: TopologyOperationScope,
    /// Preview that must precede a mutation in this class.
    pub preview_class: OperationPreviewClass,
    /// Recovery class that must remain reachable for this class.
    pub recovery_class: OperationRecoveryClass,
    /// Whether a provider overlay may never overwrite local Git truth here.
    pub provider_overlay_never_overwrites_local_truth: bool,
    /// Consumer surfaces that must project this class.
    pub consumer_surfaces: Vec<MatrixConsumerSurface>,
}

impl TopologyClassRow {
    /// Whether this row permits any content mutation.
    pub fn permits_mutation(&self) -> bool {
        !matches!(
            self.mutation_control,
            TopologyOperationScope::MetadataOnly | TopologyOperationScope::MutationDenied
        )
    }
}

/// One history-surgery session-object row in the matrix.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SessionObjectRow {
    /// Session object governed by this row.
    pub session: HistorySurgerySession,
    /// Canonical record-kind tag the object carries.
    pub canonical_record_kind: String,
    /// Human-readable scope summary.
    pub scope_summary: String,
    /// Preview that must precede a mutation driven by this object.
    pub preview_class: OperationPreviewClass,
    /// Recovery class that must remain reachable for this object.
    pub recovery_class: OperationRecoveryClass,
    /// Mutation scope this object permits.
    pub mutation_control: TopologyOperationScope,
    /// Whether a recovery checkpoint must be captured before mutation.
    pub recovery_checkpoint_required_before_mutation: bool,
    /// Consumer surfaces that must project this object.
    pub consumer_surfaces: Vec<MatrixConsumerSurface>,
}

/// One degraded-vocabulary row in the matrix.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DegradedStateRow {
    /// Degraded state defined by this row.
    pub state: DegradedTopologyState,
    /// Human-readable meaning of the state.
    pub meaning: String,
    /// Whether this state narrows a coverage claim (never reduced to a badge).
    pub narrows_coverage_claim: bool,
    /// Whether this state gates mutation until it is resolved or acknowledged.
    pub blocks_mutation_until_resolved: bool,
    /// Whether this state must be visible before a destructive operation.
    pub visible_before_destructive_op: bool,
}

/// One risky-operation row in the matrix.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RiskyOperationRow {
    /// Risky operation governed by this row.
    pub operation: RiskyHistoryOperation,
    /// Preview that must precede the operation.
    pub preview_class: OperationPreviewClass,
    /// Recovery class that must remain reachable for the operation.
    pub recovery_class: OperationRecoveryClass,
    /// Whether the operation requires an explicit, unambiguous target.
    pub requires_explicit_target: bool,
    /// Whether recovery truth is shown before the operation executes.
    pub recovery_visible_before_execution: bool,
}

/// Governance review block proving the matrix controls behavior, not badges.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MatrixGovernanceReview {
    /// Topology truth is not reduced to a badge.
    pub topology_truth_not_reduced_to_badges: bool,
    /// The matrix controls actual mutation, preview, recovery, and export.
    pub matrix_controls_mutation_preview_recovery_export: bool,
    /// Provider overlays never overwrite local Git truth.
    pub provider_overlay_never_overwrites_local_truth: bool,
    /// Recovery checkpoints/fallbacks are visible before destructive ops.
    pub recovery_visible_before_destructive_ops: bool,
    /// Reflog-only fallback is disclosed when no checkpoint is possible.
    pub reflog_only_fallback_disclosed: bool,
    /// All claimed surfaces reference this one shared matrix.
    pub one_shared_matrix_across_surfaces: bool,
    /// The degraded vocabulary is shared across every surface.
    pub degraded_vocabulary_shared_across_surfaces: bool,
    /// Every risky operation has both a preview and a recovery class.
    pub risky_ops_have_preview_and_recovery: bool,
    /// Sparse/partial/shallow/submodule/nested/LFS/worktree stay distinct.
    pub topology_classes_stay_distinct: bool,
}

/// Consumer-parity review block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MatrixConsumerParity {
    /// Provider overlay can express the topology and recovery vocabulary.
    pub provider_overlay_expresses_vocabulary: bool,
    /// AI context can express the topology and recovery vocabulary.
    pub ai_context_expresses_vocabulary: bool,
    /// Search can express the topology and recovery vocabulary.
    pub search_expresses_vocabulary: bool,
    /// Review can express the topology and recovery vocabulary.
    pub review_expresses_vocabulary: bool,
    /// Support/export can express the topology and recovery vocabulary.
    pub support_export_expresses_vocabulary: bool,
    /// CLI/headless can express the topology and recovery vocabulary.
    pub cli_expresses_vocabulary: bool,
}

/// Freeze posture block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MatrixFreezePosture {
    /// True when the matrix is frozen as canonical M5 truth.
    pub frozen: bool,
    /// Review SLO in hours.
    pub review_slo_hours: u32,
    /// RFC 3339 timestamp of the last review.
    pub last_reviewed_at: String,
    /// True when stale review automatically narrows claims.
    pub auto_narrow_on_stale: bool,
}

/// Constructor input for [`M5GitTopologyHistoryMatrixPacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M5GitTopologyHistoryMatrixPacketInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable matrix label.
    pub matrix_label: String,
    /// Topology-class rows.
    pub topology_rows: Vec<TopologyClassRow>,
    /// Session-object rows.
    pub session_object_rows: Vec<SessionObjectRow>,
    /// Degraded-vocabulary rows.
    pub degraded_state_rows: Vec<DegradedStateRow>,
    /// Risky-operation rows.
    pub risky_operation_rows: Vec<RiskyOperationRow>,
    /// Governance review block.
    pub governance_review: MatrixGovernanceReview,
    /// Consumer-parity block.
    pub consumer_parity: MatrixConsumerParity,
    /// Freeze posture block.
    pub freeze_posture: MatrixFreezePosture,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

/// Export-safe frozen M5 Git topology and history-surgery matrix packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5GitTopologyHistoryMatrixPacket {
    /// Record kind; must equal [`M5_GIT_TOPOLOGY_HISTORY_MATRIX_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`M5_GIT_TOPOLOGY_HISTORY_MATRIX_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable matrix label.
    pub matrix_label: String,
    /// Topology-class rows.
    pub topology_rows: Vec<TopologyClassRow>,
    /// Session-object rows.
    pub session_object_rows: Vec<SessionObjectRow>,
    /// Degraded-vocabulary rows.
    pub degraded_state_rows: Vec<DegradedStateRow>,
    /// Risky-operation rows.
    pub risky_operation_rows: Vec<RiskyOperationRow>,
    /// Governance review block.
    pub governance_review: MatrixGovernanceReview,
    /// Consumer-parity block.
    pub consumer_parity: MatrixConsumerParity,
    /// Freeze posture block.
    pub freeze_posture: MatrixFreezePosture,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

impl M5GitTopologyHistoryMatrixPacket {
    /// Builds a matrix packet from frozen input.
    pub fn new(input: M5GitTopologyHistoryMatrixPacketInput) -> Self {
        Self {
            record_kind: M5_GIT_TOPOLOGY_HISTORY_MATRIX_RECORD_KIND.to_owned(),
            schema_version: M5_GIT_TOPOLOGY_HISTORY_MATRIX_SCHEMA_VERSION,
            packet_id: input.packet_id,
            matrix_label: input.matrix_label,
            topology_rows: input.topology_rows,
            session_object_rows: input.session_object_rows,
            degraded_state_rows: input.degraded_state_rows,
            risky_operation_rows: input.risky_operation_rows,
            governance_review: input.governance_review,
            consumer_parity: input.consumer_parity,
            freeze_posture: input.freeze_posture,
            source_contract_refs: input.source_contract_refs,
            redaction_class_token: input.redaction_class_token,
            minted_at: input.minted_at,
        }
    }

    /// Validates the matrix invariants.
    ///
    /// Returns every violation found rather than stopping at the first, so a
    /// regenerator or CI gate can report the full set at once.
    pub fn validate(&self) -> Vec<M5GitTopologyHistoryMatrixViolation> {
        let mut violations = Vec::new();

        if self.record_kind != M5_GIT_TOPOLOGY_HISTORY_MATRIX_RECORD_KIND {
            violations.push(M5GitTopologyHistoryMatrixViolation::WrongRecordKind);
        }
        if self.schema_version != M5_GIT_TOPOLOGY_HISTORY_MATRIX_SCHEMA_VERSION {
            violations.push(M5GitTopologyHistoryMatrixViolation::WrongSchemaVersion);
        }
        if self.packet_id.trim().is_empty()
            || self.matrix_label.trim().is_empty()
            || self.redaction_class_token.trim().is_empty()
            || self.minted_at.trim().is_empty()
        {
            violations.push(M5GitTopologyHistoryMatrixViolation::MissingIdentity);
        }

        validate_source_contracts(self, &mut violations);
        validate_topology_rows(self, &mut violations);
        validate_session_rows(self, &mut violations);
        validate_degraded_rows(self, &mut violations);
        validate_risky_rows(self, &mut violations);
        validate_governance_review(self, &mut violations);
        validate_consumer_parity(self, &mut violations);
        validate_freeze_posture(self, &mut violations);

        if json_contains_forbidden_boundary_material(
            &serde_json::to_value(self).expect("m5 git topology history matrix packet serializes"),
        ) {
            violations.push(M5GitTopologyHistoryMatrixViolation::RawBoundaryMaterialInExport);
        }

        violations
    }

    /// Deterministic export-safe pretty JSON.
    ///
    /// # Panics
    ///
    /// Panics only if serializing this metadata-only packet fails.
    pub fn export_safe_json(&self) -> String {
        serde_json::to_string_pretty(self)
            .expect("m5 git topology history matrix packet serializes")
    }

    /// Deterministic Markdown summary for support, docs, or review handoff.
    pub fn render_markdown_summary(&self) -> String {
        let mut out = String::new();
        out.push_str("# M5 Repository-Topology and History-Surgery Matrix\n\n");
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- Label: `{}`\n", self.matrix_label));
        out.push_str(&format!(
            "- Frozen: {} (review SLO: {} hours, last reviewed: {})\n",
            self.freeze_posture.frozen,
            self.freeze_posture.review_slo_hours,
            self.freeze_posture.last_reviewed_at
        ));
        out.push_str(&format!(
            "- Rows: {} topology / {} sessions / {} degraded states / {} risky ops\n",
            self.topology_rows.len(),
            self.session_object_rows.len(),
            self.degraded_state_rows.len(),
            self.risky_operation_rows.len()
        ));

        out.push_str("\n## Topology classes\n\n");
        for row in &self.topology_rows {
            out.push_str(&format!(
                "- **{}**: preview `{}`, recovery `{}`, mutation `{}`\n",
                row.topology_class.as_str(),
                row.preview_class.as_str(),
                row.recovery_class.as_str(),
                operation_scope_token(row.mutation_control),
            ));
        }

        out.push_str("\n## Session objects\n\n");
        for row in &self.session_object_rows {
            out.push_str(&format!(
                "- **{}** (`{}`): preview `{}`, recovery `{}`\n",
                row.session.as_str(),
                row.canonical_record_kind,
                row.preview_class.as_str(),
                row.recovery_class.as_str(),
            ));
        }

        out.push_str("\n## Degraded vocabulary\n\n");
        for row in &self.degraded_state_rows {
            out.push_str(&format!("- **{}**: {}\n", row.state.as_str(), row.meaning));
        }

        out.push_str("\n## Risky operations\n\n");
        for row in &self.risky_operation_rows {
            out.push_str(&format!(
                "- **{}**: preview `{}`, recovery `{}`\n",
                row.operation.as_str(),
                row.preview_class.as_str(),
                row.recovery_class.as_str(),
            ));
        }
        out
    }
}

/// Errors emitted when reading the checked-in matrix export.
#[derive(Debug)]
pub enum M5GitTopologyHistoryMatrixArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<M5GitTopologyHistoryMatrixViolation>),
}

impl fmt::Display for M5GitTopologyHistoryMatrixArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(
                    formatter,
                    "m5 git topology history matrix export parse failed: {error}"
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
                    "m5 git topology history matrix export failed validation: {tokens}"
                )
            }
        }
    }
}

impl Error for M5GitTopologyHistoryMatrixArtifactError {}

/// Validation failures emitted by [`M5GitTopologyHistoryMatrixPacket::validate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum M5GitTopologyHistoryMatrixViolation {
    /// Packet record kind is wrong.
    WrongRecordKind,
    /// Packet schema version is wrong.
    WrongSchemaVersion,
    /// A required identity field is missing.
    MissingIdentity,
    /// Source contract refs are incomplete.
    MissingSourceContracts,
    /// A required topology class is missing from the matrix.
    RequiredTopologyClassMissing,
    /// A topology class is listed more than once.
    DuplicateTopologyClass,
    /// A topology row is incomplete.
    TopologyRowIncomplete,
    /// A mutating topology row does not provide a recovery path.
    MutatingRowMissingRecovery,
    /// A required session object is missing from the matrix.
    RequiredSessionObjectMissing,
    /// A session row's record kind does not match the canonical object.
    SessionRecordKindMismatch,
    /// A session row is incomplete.
    SessionRowIncomplete,
    /// A required degraded state is missing from the matrix.
    RequiredDegradedStateMissing,
    /// A degraded row is incomplete (e.g. reduced to a badge).
    DegradedStateRowIncomplete,
    /// A required risky operation is missing from the matrix.
    RequiredRiskyOperationMissing,
    /// A risky operation is missing its preview or recovery class.
    RiskyOperationMissingPreviewOrRecovery,
    /// Governance review does not satisfy required invariants.
    GovernanceReviewIncomplete,
    /// Consumer parity does not satisfy required invariants.
    ConsumerParityIncomplete,
    /// Freeze posture block is incomplete.
    FreezePostureIncomplete,
    /// Export contains raw boundary material.
    RawBoundaryMaterialInExport,
}

impl M5GitTopologyHistoryMatrixViolation {
    /// Stable token used in tests and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WrongRecordKind => "wrong_record_kind",
            Self::WrongSchemaVersion => "wrong_schema_version",
            Self::MissingIdentity => "missing_identity",
            Self::MissingSourceContracts => "missing_source_contracts",
            Self::RequiredTopologyClassMissing => "required_topology_class_missing",
            Self::DuplicateTopologyClass => "duplicate_topology_class",
            Self::TopologyRowIncomplete => "topology_row_incomplete",
            Self::MutatingRowMissingRecovery => "mutating_row_missing_recovery",
            Self::RequiredSessionObjectMissing => "required_session_object_missing",
            Self::SessionRecordKindMismatch => "session_record_kind_mismatch",
            Self::SessionRowIncomplete => "session_row_incomplete",
            Self::RequiredDegradedStateMissing => "required_degraded_state_missing",
            Self::DegradedStateRowIncomplete => "degraded_state_row_incomplete",
            Self::RequiredRiskyOperationMissing => "required_risky_operation_missing",
            Self::RiskyOperationMissingPreviewOrRecovery => {
                "risky_operation_missing_preview_or_recovery"
            }
            Self::GovernanceReviewIncomplete => "governance_review_incomplete",
            Self::ConsumerParityIncomplete => "consumer_parity_incomplete",
            Self::FreezePostureIncomplete => "freeze_posture_incomplete",
            Self::RawBoundaryMaterialInExport => "raw_boundary_material_in_export",
        }
    }
}

/// Reads and validates the checked-in stable matrix export.
///
/// # Errors
///
/// Returns [`M5GitTopologyHistoryMatrixArtifactError`] when the checked-in
/// export fails to parse or violates the frozen contract.
pub fn current_stable_m5_git_topology_history_matrix_export(
) -> Result<M5GitTopologyHistoryMatrixPacket, M5GitTopologyHistoryMatrixArtifactError> {
    let packet: M5GitTopologyHistoryMatrixPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/git/m5/freeze_the_m5_repository_topology_worktree_scope_history_surgery_and_checkpoint_recovery_matrix/support_export.json"
    )))
    .map_err(M5GitTopologyHistoryMatrixArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(M5GitTopologyHistoryMatrixArtifactError::Validation(
            violations,
        ))
    }
}

/// Stable token for a [`TopologyOperationScope`] reused from the topology packet.
fn operation_scope_token(scope: TopologyOperationScope) -> &'static str {
    match scope {
        TopologyOperationScope::ActiveRootOnly => "active_root_only",
        TopologyOperationScope::ChildRootOnly => "child_root_only",
        TopologyOperationScope::ExplicitMultiRootPreviewRequired => {
            "explicit_multi_root_preview_required"
        }
        TopologyOperationScope::MetadataOnly => "metadata_only",
        TopologyOperationScope::MutationDenied => "mutation_denied",
    }
}

fn validate_source_contracts(
    packet: &M5GitTopologyHistoryMatrixPacket,
    violations: &mut Vec<M5GitTopologyHistoryMatrixViolation>,
) {
    let refs: BTreeSet<&str> = packet
        .source_contract_refs
        .iter()
        .map(String::as_str)
        .collect();
    for required in [
        M5_GIT_TOPOLOGY_HISTORY_MATRIX_SCHEMA_REF,
        M5_GIT_TOPOLOGY_HISTORY_MATRIX_DOC_REF,
        M5_GIT_TOPOLOGY_HISTORY_MATRIX_TOPOLOGY_CONTRACT_REF,
        M5_GIT_TOPOLOGY_HISTORY_MATRIX_CONFLICT_SESSION_CONTRACT_REF,
        M5_GIT_TOPOLOGY_HISTORY_MATRIX_SEQUENCE_EDIT_CONTRACT_REF,
        M5_GIT_TOPOLOGY_HISTORY_MATRIX_RECOVERY_CHECKPOINT_CONTRACT_REF,
        M5_GIT_TOPOLOGY_HISTORY_MATRIX_STASH_CONTRACT_REF,
        M5_GIT_TOPOLOGY_HISTORY_MATRIX_REF_UPDATE_CONTRACT_REF,
    ] {
        if !refs.contains(required) {
            violations.push(M5GitTopologyHistoryMatrixViolation::MissingSourceContracts);
            return;
        }
    }
}

fn validate_topology_rows(
    packet: &M5GitTopologyHistoryMatrixPacket,
    violations: &mut Vec<M5GitTopologyHistoryMatrixViolation>,
) {
    let mut seen: HashSet<RepositoryTopologyClass> = HashSet::new();
    for row in &packet.topology_rows {
        if !seen.insert(row.topology_class) {
            violations.push(M5GitTopologyHistoryMatrixViolation::DuplicateTopologyClass);
        }
        if row.scope_summary.trim().is_empty()
            || row.degraded_states.is_empty()
            || row.consumer_surfaces.is_empty()
        {
            violations.push(M5GitTopologyHistoryMatrixViolation::TopologyRowIncomplete);
        }
        if row.permits_mutation() && !row.recovery_class.provides_recovery() {
            violations.push(M5GitTopologyHistoryMatrixViolation::MutatingRowMissingRecovery);
        }
        if row.permits_mutation() && !row.preview_class.is_mutation_preview() {
            violations.push(M5GitTopologyHistoryMatrixViolation::TopologyRowIncomplete);
        }
    }
    for required in M5_GIT_TOPOLOGY_HISTORY_MATRIX_REQUIRED_CLASSES {
        if !seen.contains(&required) {
            violations.push(M5GitTopologyHistoryMatrixViolation::RequiredTopologyClassMissing);
            return;
        }
    }
}

fn validate_session_rows(
    packet: &M5GitTopologyHistoryMatrixPacket,
    violations: &mut Vec<M5GitTopologyHistoryMatrixViolation>,
) {
    let present: BTreeSet<HistorySurgerySession> = packet
        .session_object_rows
        .iter()
        .map(|row| row.session)
        .collect();
    for required in HistorySurgerySession::ALL {
        if !present.contains(&required) {
            violations.push(M5GitTopologyHistoryMatrixViolation::RequiredSessionObjectMissing);
            return;
        }
    }

    for row in &packet.session_object_rows {
        if row.canonical_record_kind != row.session.canonical_record_kind() {
            violations.push(M5GitTopologyHistoryMatrixViolation::SessionRecordKindMismatch);
        }
        if row.scope_summary.trim().is_empty() || row.consumer_surfaces.is_empty() {
            violations.push(M5GitTopologyHistoryMatrixViolation::SessionRowIncomplete);
        }
        // Every history-surgery object except the recovery object itself must
        // expose a reachable recovery path; the recovery object is the path.
        if !row.session.is_recovery_object() && !row.recovery_class.provides_recovery() {
            violations.push(M5GitTopologyHistoryMatrixViolation::SessionRowIncomplete);
        }
    }
}

fn validate_degraded_rows(
    packet: &M5GitTopologyHistoryMatrixPacket,
    violations: &mut Vec<M5GitTopologyHistoryMatrixViolation>,
) {
    let present: BTreeSet<DegradedTopologyState> = packet
        .degraded_state_rows
        .iter()
        .map(|row| row.state)
        .collect();
    for required in DegradedTopologyState::ALL {
        if !present.contains(&required) {
            violations.push(M5GitTopologyHistoryMatrixViolation::RequiredDegradedStateMissing);
            return;
        }
    }

    for row in &packet.degraded_state_rows {
        // Guardrail: a degraded state may not be reduced to a badge; it must
        // narrow the claim. The reflog-only fallback must always be visible
        // before a destructive operation.
        if row.meaning.trim().is_empty() || !row.narrows_coverage_claim {
            violations.push(M5GitTopologyHistoryMatrixViolation::DegradedStateRowIncomplete);
        }
        if row.state == DegradedTopologyState::ReflogOnlyFallback
            && !row.visible_before_destructive_op
        {
            violations.push(M5GitTopologyHistoryMatrixViolation::DegradedStateRowIncomplete);
        }
    }
}

fn validate_risky_rows(
    packet: &M5GitTopologyHistoryMatrixPacket,
    violations: &mut Vec<M5GitTopologyHistoryMatrixViolation>,
) {
    let present: BTreeSet<RiskyHistoryOperation> = packet
        .risky_operation_rows
        .iter()
        .map(|row| row.operation)
        .collect();
    for required in RiskyHistoryOperation::ALL {
        if !present.contains(&required) {
            violations.push(M5GitTopologyHistoryMatrixViolation::RequiredRiskyOperationMissing);
            return;
        }
    }

    for row in &packet.risky_operation_rows {
        if !row.preview_class.is_mutation_preview()
            || !row.recovery_class.provides_recovery()
            || !row.recovery_visible_before_execution
        {
            violations
                .push(M5GitTopologyHistoryMatrixViolation::RiskyOperationMissingPreviewOrRecovery);
        }
    }
}

fn validate_governance_review(
    packet: &M5GitTopologyHistoryMatrixPacket,
    violations: &mut Vec<M5GitTopologyHistoryMatrixViolation>,
) {
    let review = &packet.governance_review;
    for ok in [
        review.topology_truth_not_reduced_to_badges,
        review.matrix_controls_mutation_preview_recovery_export,
        review.provider_overlay_never_overwrites_local_truth,
        review.recovery_visible_before_destructive_ops,
        review.reflog_only_fallback_disclosed,
        review.one_shared_matrix_across_surfaces,
        review.degraded_vocabulary_shared_across_surfaces,
        review.risky_ops_have_preview_and_recovery,
        review.topology_classes_stay_distinct,
    ] {
        if !ok {
            violations.push(M5GitTopologyHistoryMatrixViolation::GovernanceReviewIncomplete);
            return;
        }
    }
}

fn validate_consumer_parity(
    packet: &M5GitTopologyHistoryMatrixPacket,
    violations: &mut Vec<M5GitTopologyHistoryMatrixViolation>,
) {
    let parity = &packet.consumer_parity;
    for ok in [
        parity.provider_overlay_expresses_vocabulary,
        parity.ai_context_expresses_vocabulary,
        parity.search_expresses_vocabulary,
        parity.review_expresses_vocabulary,
        parity.support_export_expresses_vocabulary,
        parity.cli_expresses_vocabulary,
    ] {
        if !ok {
            violations.push(M5GitTopologyHistoryMatrixViolation::ConsumerParityIncomplete);
            return;
        }
    }
}

fn validate_freeze_posture(
    packet: &M5GitTopologyHistoryMatrixPacket,
    violations: &mut Vec<M5GitTopologyHistoryMatrixViolation>,
) {
    if !packet.freeze_posture.frozen
        || packet.freeze_posture.review_slo_hours == 0
        || packet.freeze_posture.last_reviewed_at.trim().is_empty()
    {
        violations.push(M5GitTopologyHistoryMatrixViolation::FreezePostureIncomplete);
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
