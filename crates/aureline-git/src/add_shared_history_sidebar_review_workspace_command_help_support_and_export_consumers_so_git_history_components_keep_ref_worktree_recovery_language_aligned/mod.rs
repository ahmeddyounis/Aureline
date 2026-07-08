//! Shared history-sidebar / risky-mutation-sheet / review-workspace-banner /
//! command-help / support-bundle / exported-recovery-packet consumers that keep
//! the reusable Git-history and sequence components at ref, worktree, recovery,
//! and verb parity across every claimed M5 profile.
//!
//! This module is the closing consumer-adoption lane for the twelve reusable
//! Git-history and risky-mutation components frozen in
//! [`crate::freeze_the_m5_git_history_sequence_component_matrix`] and implemented
//! by the commit-graph / history-graph / branch-comparison / worktree lane, the
//! stash-entry / reflog-recovery lane, the rebase-todo / sequence-editor lane, and
//! the cherry-pick / revert / patch-apply / conflict-checkpoint / force-push lane.
//! It binds each shared component to the history sidebar, the risky-mutation review
//! sheet, the review-workspace banner, the Help surface, the support bundle, and the
//! exported recovery packet that render it, and proves — by fixtures, not
//! screenshots — that the same Git-history object presents the same exact target
//! ref, worktree scope, recovery destination, and primary verb wherever it appears.
//!
//! The core honesty axes are two. First, parity: for a given Git-history object,
//! every consumer surface must present identical parity facet values — the same ref
//! identity, the same worktree scope, the same recovery destination, and the same
//! primary verb. A surface may narrow how much it shows when local topology,
//! provider overlay, or recovery reachability degrades, but it may never reword the
//! underlying ref/worktree/recovery language per surface, collapse multiple Git
//! verbs into one ambiguous confirm, hide the exact target ref or worktree, let
//! conflict or recovery state disappear after a risky mutation, or drop local-only
//! recovery when provider-linked review state also exists. Second, disclosure: when
//! a surface narrows, it must do so through an explicit narrow banner that names the
//! reason, the preserved facets, and the next action — a detached/missing ref stays
//! spelled out, a reflog-only recovery destination stays named, and a local-only
//! continuation stays explicit rather than collapsing the object out of view.
//!
//! Component reuse is proven rather than inferred: every one of the twelve shared
//! components must be adopted by at least two distinct consumers, and Help, support,
//! and exported-recovery consumers must point at the canonical component contracts
//! by id. The component identity is reused directly from the frozen matrix
//! ([`M5GitHistoryComponent`]) and every render condition binds back to the frozen
//! downgrade vocabulary ([`GitHistoryDowngradeState`]), so component identity and
//! downgrade language read the same everywhere.
//!
//! The packet references upstream component contracts by id rather than embedding
//! their content. Raw paths, raw object bytes, raw branch names, raw patch/reflog/
//! stash bodies, raw provider payloads, and credentials stay outside the support
//! boundary.
//!
//! The boundary schema is
//! [`schemas/ui/m5-git-history-component-consumer.schema.json`](../../../../schemas/ui/m5-git-history-component-consumer.schema.json).
//! The contract doc is
//! [`docs/git/m5/add_shared_history_sidebar_review_workspace_command_help_support_and_export_consumers_so_git_history_components_keep_ref_worktree_recovery_language_aligned.md`](../../../../docs/git/m5/add_shared_history_sidebar_review_workspace_command_help_support_and_export_consumers_so_git_history_components_keep_ref_worktree_recovery_language_aligned.md).
//! The protected fixture directory is
//! [`fixtures/ui/m5-git-history-component-consumers/`](../../../../fixtures/ui/m5-git-history-component-consumers/).

#[cfg(test)]
mod tests;

use std::collections::{BTreeMap, BTreeSet};
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

use crate::freeze_the_m5_git_history_sequence_component_matrix::{
    GitHistoryDowngradeState, M5GitHistoryComponent,
};

/// Stable record-kind tag carried by [`GitHistoryComponentConsumerPacket`].
pub const GIT_HISTORY_COMPONENT_CONSUMER_RECORD_KIND: &str =
    "git_history_component_consumer_parity_truth";

/// Schema version for Git-history component consumer parity records.
pub const GIT_HISTORY_COMPONENT_CONSUMER_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the boundary schema.
pub const GIT_HISTORY_COMPONENT_CONSUMER_SCHEMA_REF: &str =
    "schemas/ui/m5-git-history-component-consumer.schema.json";

/// Repo-relative path of the contract doc.
pub const GIT_HISTORY_COMPONENT_CONSUMER_DOC_REF: &str =
    "docs/git/m5/add_shared_history_sidebar_review_workspace_command_help_support_and_export_consumers_so_git_history_components_keep_ref_worktree_recovery_language_aligned.md";

/// Repo-relative path of the frozen component matrix these consumers adopt.
pub const GIT_HISTORY_COMPONENT_CONSUMER_COMPONENT_MATRIX_CONTRACT_REF: &str =
    "schemas/ui/m5-git-history-sequence-component-matrix.schema.json";

/// Repo-relative path of the commit-graph / history-graph / branch-comparison /
/// worktree identity component contract.
pub const GIT_HISTORY_COMPONENT_CONSUMER_IDENTITY_CONTRACT_REF: &str =
    "schemas/ui/m5-git-history-identity-component.schema.json";

/// Repo-relative path of the stash-entry / reflog-recovery component contract.
pub const GIT_HISTORY_COMPONENT_CONSUMER_STASH_RECOVERY_CONTRACT_REF: &str =
    "schemas/ui/m5-stash-reflog-recovery-component.schema.json";

/// Repo-relative path of the rebase-todo / sequence-editor component contract.
pub const GIT_HISTORY_COMPONENT_CONSUMER_SEQUENCE_EDIT_CONTRACT_REF: &str =
    "schemas/ui/m5-rebase-todo-sequence-editor-component.schema.json";

/// Repo-relative path of the cherry-pick / revert / patch-apply / conflict-checkpoint
/// / force-push mutation-review component contract.
pub const GIT_HISTORY_COMPONENT_CONSUMER_MUTATION_REVIEW_CONTRACT_REF: &str =
    "schemas/ui/m5-git-mutation-review-recovery-component.schema.json";

/// Repo-relative path of the protected fixture directory.
pub const GIT_HISTORY_COMPONENT_CONSUMER_FIXTURE_DIR: &str =
    "fixtures/ui/m5-git-history-component-consumers";

/// Repo-relative path of the checked support-export artifact.
pub const GIT_HISTORY_COMPONENT_CONSUMER_ARTIFACT_REF: &str =
    "artifacts/release/m5-git-history-component-consumers-proof/support_export.json";

/// Repo-relative path of the checked Markdown summary.
pub const GIT_HISTORY_COMPONENT_CONSUMER_SUMMARY_REF: &str =
    "artifacts/release/m5-git-history-component-consumers-proof/summary.md";

/// Canonical component contract that a consumer must point at for a given component.
///
/// Each of the twelve shared components resolves to the checked-in schema of the
/// implement lane that owns it: the commit-graph / history-graph / branch-comparison
/// / worktree identity lane, the stash-entry / reflog-recovery lane, the rebase-todo
/// / sequence-editor lane, or the cherry-pick / revert / patch-apply /
/// conflict-checkpoint / force-push mutation-review lane.
pub const fn component_canonical_schema_ref(component: M5GitHistoryComponent) -> &'static str {
    match component {
        M5GitHistoryComponent::CommitGraphHeader
        | M5GitHistoryComponent::HistoryGraphRow
        | M5GitHistoryComponent::BranchComparisonChip
        | M5GitHistoryComponent::WorktreeRow => {
            GIT_HISTORY_COMPONENT_CONSUMER_IDENTITY_CONTRACT_REF
        }
        M5GitHistoryComponent::StashEntry | M5GitHistoryComponent::ReflogRecoveryBanner => {
            GIT_HISTORY_COMPONENT_CONSUMER_STASH_RECOVERY_CONTRACT_REF
        }
        M5GitHistoryComponent::RebaseTodoRow | M5GitHistoryComponent::SequenceEditorHeader => {
            GIT_HISTORY_COMPONENT_CONSUMER_SEQUENCE_EDIT_CONTRACT_REF
        }
        M5GitHistoryComponent::CherryPickRevertReviewSheet
        | M5GitHistoryComponent::PatchApplyReviewSheet
        | M5GitHistoryComponent::ConflictCheckpointCard
        | M5GitHistoryComponent::ForcePushReviewDialog => {
            GIT_HISTORY_COMPONENT_CONSUMER_MUTATION_REVIEW_CONTRACT_REF
        }
    }
}

/// Consumer surface that must reuse the shared Git-history components at full parity.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum GitHistoryComponentConsumer {
    /// Desktop history sidebar (commit graph, branches, worktrees, stash shelf).
    HistorySidebar,
    /// Risky-mutation review sheet (rebase, cherry-pick, revert, patch, force-push).
    RiskyMutationSheet,
    /// Review-workspace banner layered over an in-flight review.
    ReviewWorkspaceBanner,
    /// Command-help / About surface.
    CommandHelp,
    /// Support bundle.
    SupportBundle,
    /// Exported recovery packet / evidence.
    ExportedRecoveryPacket,
}

impl GitHistoryComponentConsumer {
    /// Every consumer, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::HistorySidebar,
        Self::RiskyMutationSheet,
        Self::ReviewWorkspaceBanner,
        Self::CommandHelp,
        Self::SupportBundle,
        Self::ExportedRecoveryPacket,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::HistorySidebar => "history_sidebar",
            Self::RiskyMutationSheet => "risky_mutation_sheet",
            Self::ReviewWorkspaceBanner => "review_workspace_banner",
            Self::CommandHelp => "command_help",
            Self::SupportBundle => "support_bundle",
            Self::ExportedRecoveryPacket => "exported_recovery_packet",
        }
    }

    /// Whether this consumer is a Help, support, or exported-recovery surface that
    /// must point at the canonical component contracts by id.
    pub const fn is_help_support_or_export(self) -> bool {
        matches!(
            self,
            Self::CommandHelp | Self::SupportBundle | Self::ExportedRecoveryPacket
        )
    }
}

/// A parity facet whose value must stay identical across surfaces for one object.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum GitHistoryComponentParityFacet {
    /// The exact target ref / commit identity the component names.
    RefIdentityLabel,
    /// The worktree / root scope the component names.
    WorktreeScopeLabel,
    /// The recovery checkpoint / destination the component names.
    RecoveryDestinationLabel,
    /// The primary Git verb the component drives (never collapsed).
    PrimaryVerb,
}

impl GitHistoryComponentParityFacet {
    /// Every parity facet, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::RefIdentityLabel,
        Self::WorktreeScopeLabel,
        Self::RecoveryDestinationLabel,
        Self::PrimaryVerb,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::RefIdentityLabel => "ref_identity_label",
            Self::WorktreeScopeLabel => "worktree_scope_label",
            Self::RecoveryDestinationLabel => "recovery_destination_label",
            Self::PrimaryVerb => "primary_verb",
        }
    }
}

/// The render condition of a Git-history object on one surface.
///
/// `AlignedLocalTruth` renders at full parity; every other condition binds back to a
/// frozen [`GitHistoryDowngradeState`] and narrows how much a surface shows without
/// ever rewording the underlying ref/worktree/recovery language.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum GitHistoryRenderCondition {
    /// Local Git truth is aligned; the component renders at full parity.
    AlignedLocalTruth,
    /// A provider overlay is stale relative to local Git truth.
    StaleProviderOverlay,
    /// The target ref is detached or missing; identity must be spelled out.
    DetachedOrMissingRef,
    /// The worktree is dirty or conflicted at the operation target.
    DirtyOrConflictedWorktree,
    /// Topology is shallow/partial/sparse, so history is incomplete here.
    ShallowOrPartialTopology,
    /// No checkpoint exists; only a reflog-only recovery fallback is offered.
    ReflogOnlyFallback,
    /// A prior approval was invalidated by this object's change.
    ApprovalInvalidated,
    /// Operating offline / local-only; provider handoff is unavailable.
    OfflineLocalOnly,
}

impl GitHistoryRenderCondition {
    /// Every render condition, in declaration order.
    pub const ALL: [Self; 8] = [
        Self::AlignedLocalTruth,
        Self::StaleProviderOverlay,
        Self::DetachedOrMissingRef,
        Self::DirtyOrConflictedWorktree,
        Self::ShallowOrPartialTopology,
        Self::ReflogOnlyFallback,
        Self::ApprovalInvalidated,
        Self::OfflineLocalOnly,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::AlignedLocalTruth => "aligned_local_truth",
            Self::StaleProviderOverlay => "stale_provider_overlay",
            Self::DetachedOrMissingRef => "detached_or_missing_ref",
            Self::DirtyOrConflictedWorktree => "dirty_or_conflicted_worktree",
            Self::ShallowOrPartialTopology => "shallow_or_partial_topology",
            Self::ReflogOnlyFallback => "reflog_only_fallback",
            Self::ApprovalInvalidated => "approval_invalidated",
            Self::OfflineLocalOnly => "offline_local_only",
        }
    }

    /// The frozen downgrade state this condition binds to, if it narrows.
    ///
    /// `AlignedLocalTruth` returns `None`; every other condition reuses the frozen
    /// [`GitHistoryDowngradeState`] vocabulary so downgrade language reads the same
    /// on every surface.
    pub const fn downgrade_state(self) -> Option<GitHistoryDowngradeState> {
        match self {
            Self::AlignedLocalTruth => None,
            Self::StaleProviderOverlay => Some(GitHistoryDowngradeState::StaleProviderOverlay),
            Self::DetachedOrMissingRef => Some(GitHistoryDowngradeState::DetachedOrMissingRef),
            Self::DirtyOrConflictedWorktree => {
                Some(GitHistoryDowngradeState::DirtyOrConflictedWorktree)
            }
            Self::ShallowOrPartialTopology => {
                Some(GitHistoryDowngradeState::ShallowOrPartialTopology)
            }
            Self::ReflogOnlyFallback => Some(GitHistoryDowngradeState::ReflogOnlyFallback),
            Self::ApprovalInvalidated => Some(GitHistoryDowngradeState::ApprovalInvalidated),
            Self::OfflineLocalOnly => Some(GitHistoryDowngradeState::OfflineLocalOnly),
        }
    }
}

/// How much of a shared component a consumer renders.
///
/// Narrowing changes how much is shown, never the underlying parity language: a
/// narrowed surface still carries the same ref identity, worktree scope, recovery
/// destination, and primary verb, and discloses the narrowing through an explicit
/// banner.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum GitHistoryRenderMode {
    /// Full parity; local Git truth is aligned.
    FullParity,
    /// Ref / worktree / topology identity is narrowed but spelled out.
    IdentityNarrowed,
    /// The recovery destination is narrowed (reflog-only or approval-invalidated).
    RecoveryNarrowed,
    /// Local-continue fallback; the surface continues from local-only truth.
    LocalContinueFallback,
}

impl GitHistoryRenderMode {
    /// Every render mode, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::FullParity,
        Self::IdentityNarrowed,
        Self::RecoveryNarrowed,
        Self::LocalContinueFallback,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::FullParity => "full_parity",
            Self::IdentityNarrowed => "identity_narrowed",
            Self::RecoveryNarrowed => "recovery_narrowed",
            Self::LocalContinueFallback => "local_continue_fallback",
        }
    }

    /// Whether this mode narrows below full parity.
    pub const fn is_narrowed(self) -> bool {
        !matches!(self, Self::FullParity)
    }
}

/// Why a surface narrowed its rendering of a shared component.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum GitHistoryNarrowReason {
    /// Ref / worktree / topology identity degraded and is disclosed, not hidden.
    RefWorktreeIdentityNarrowed,
    /// The recovery destination narrowed to reflog-only or an approval reset.
    RecoveryDestinationNarrowed,
    /// A local-only continuation is engaged while provider handoff is unavailable.
    LocalContinueEngaged,
}

impl GitHistoryNarrowReason {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::RefWorktreeIdentityNarrowed => "ref_worktree_identity_narrowed",
            Self::RecoveryDestinationNarrowed => "recovery_destination_narrowed",
            Self::LocalContinueEngaged => "local_continue_engaged",
        }
    }
}

/// The next action a narrow banner offers.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum GitHistoryNarrowNextAction {
    /// Reconcile the exact target ref / worktree identity.
    ReconcileRefWorktreeIdentity,
    /// Open the recovery checkpoint / destination.
    OpenRecoveryCheckpoint,
    /// Continue the history work locally.
    ContinueLocalHistory,
}

impl GitHistoryNarrowNextAction {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ReconcileRefWorktreeIdentity => "reconcile_ref_worktree_identity",
            Self::OpenRecoveryCheckpoint => "open_recovery_checkpoint",
            Self::ContinueLocalHistory => "continue_local_history",
        }
    }
}

/// Whether a binding preserves full parity or discloses a narrowed rendering.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum GitHistoryComponentParityState {
    /// All parity facets are preserved and shown in full.
    FacetsPreserved,
    /// All parity facets are preserved, and a narrowing is explicitly disclosed.
    FacetsDisclosedNarrowed,
}

impl GitHistoryComponentParityState {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::FacetsPreserved => "facets_preserved",
            Self::FacetsDisclosedNarrowed => "facets_disclosed_narrowed",
        }
    }
}

/// Downgrade trigger that can narrow this consumer lane below its claimed parity.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum GitHistoryComponentConsumerDowngradeTrigger {
    /// Proof packet has gone stale.
    ProofStale,
    /// Policy or legal block applies.
    PolicyBlocked,
    /// A provider overlay is stale relative to local Git truth.
    ProviderOverlayStale,
    /// An approval invalidation is pending and unresolved.
    ApprovalInvalidationPending,
    /// A recovery checkpoint / destination is unreachable.
    RecoveryCheckpointUnreachable,
    /// A local-continue fallback is unavailable while provider handoff is down.
    LocalContinueUnavailable,
    /// Parity drift was detected between surfaces for the same object.
    ParityDriftDetected,
    /// Consumer trust narrowed.
    TrustNarrowing,
    /// An upstream shared component narrowed.
    UpstreamComponentNarrowed,
}

impl GitHistoryComponentConsumerDowngradeTrigger {
    /// Every trigger, in declaration order.
    pub const ALL: [Self; 9] = [
        Self::ProofStale,
        Self::PolicyBlocked,
        Self::ProviderOverlayStale,
        Self::ApprovalInvalidationPending,
        Self::RecoveryCheckpointUnreachable,
        Self::LocalContinueUnavailable,
        Self::ParityDriftDetected,
        Self::TrustNarrowing,
        Self::UpstreamComponentNarrowed,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ProofStale => "proof_stale",
            Self::PolicyBlocked => "policy_blocked",
            Self::ProviderOverlayStale => "provider_overlay_stale",
            Self::ApprovalInvalidationPending => "approval_invalidation_pending",
            Self::RecoveryCheckpointUnreachable => "recovery_checkpoint_unreachable",
            Self::LocalContinueUnavailable => "local_continue_unavailable",
            Self::ParityDriftDetected => "parity_drift_detected",
            Self::TrustNarrowing => "trust_narrowing",
            Self::UpstreamComponentNarrowed => "upstream_component_narrowed",
        }
    }
}

/// The parity facet values a shared component presents for one Git-history object.
///
/// These four values must be identical across every consumer surface that shows the
/// same Git-history object. A surface may narrow how much it renders, but it may
/// never reword any of these values per surface.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GitHistoryComponentParityFacetValues {
    /// Exact target ref / commit identity (never reworded per surface).
    pub ref_identity_label: String,
    /// Worktree / root scope (identical across surfaces).
    pub worktree_scope_label: String,
    /// Recovery checkpoint / destination (identical across surfaces).
    pub recovery_destination_label: String,
    /// Primary Git verb (never collapsed, identical across surfaces).
    pub primary_verb: String,
}

impl GitHistoryComponentParityFacetValues {
    /// Whether every parity facet value is present.
    pub fn all_present(&self) -> bool {
        !self.ref_identity_label.trim().is_empty()
            && !self.worktree_scope_label.trim().is_empty()
            && !self.recovery_destination_label.trim().is_empty()
            && !self.primary_verb.trim().is_empty()
    }
}

/// The explicit banner a narrowed surface shows.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GitHistoryNarrowBanner {
    /// Why the surface narrowed.
    pub reason: GitHistoryNarrowReason,
    /// Note naming the preserved parity facets (never omitted).
    pub preserved_facets_note: String,
    /// The next action offered.
    pub next_action: GitHistoryNarrowNextAction,
    /// Human-readable next-action copy (never omitted).
    pub next_action_label: String,
}

/// Disclosures a consumer binding must carry, derived from its render condition.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GitHistoryComponentRenderDisclosure {
    /// The render mode the condition requires.
    pub expected_mode: GitHistoryRenderMode,
    /// The narrow reason the render mode requires, if any.
    pub narrow_reason: Option<GitHistoryNarrowReason>,
    /// Whether the binding must carry an explicit narrow banner.
    pub needs_narrow_banner: bool,
    /// Whether the binding must spell out the exact target ref identity.
    pub needs_ref_identity_note: bool,
    /// Whether the binding must name the recovery checkpoint / destination.
    pub needs_recovery_note: bool,
    /// Whether the binding must carry an explicit local-continue note.
    pub needs_local_continue_note: bool,
}

/// Resolves the render disclosures a consumer binding must carry from its condition.
///
/// Aligned local truth renders at full parity. A stale overlay, a detached/missing
/// ref, a dirty/conflicted worktree, and shallow/partial topology narrow identity
/// while keeping every parity facet — a detached/missing ref additionally spells out
/// the exact target ref. A reflog-only fallback and an invalidated approval narrow
/// the recovery destination but keep it named. An offline/local-only condition
/// engages the local-continue fallback so the history work never vanishes.
pub fn resolve_git_history_component_render_disclosure(
    condition: GitHistoryRenderCondition,
) -> GitHistoryComponentRenderDisclosure {
    let (expected_mode, narrow_reason) = match condition {
        GitHistoryRenderCondition::AlignedLocalTruth => (GitHistoryRenderMode::FullParity, None),
        GitHistoryRenderCondition::StaleProviderOverlay
        | GitHistoryRenderCondition::DetachedOrMissingRef
        | GitHistoryRenderCondition::DirtyOrConflictedWorktree
        | GitHistoryRenderCondition::ShallowOrPartialTopology => (
            GitHistoryRenderMode::IdentityNarrowed,
            Some(GitHistoryNarrowReason::RefWorktreeIdentityNarrowed),
        ),
        GitHistoryRenderCondition::ReflogOnlyFallback
        | GitHistoryRenderCondition::ApprovalInvalidated => (
            GitHistoryRenderMode::RecoveryNarrowed,
            Some(GitHistoryNarrowReason::RecoveryDestinationNarrowed),
        ),
        GitHistoryRenderCondition::OfflineLocalOnly => (
            GitHistoryRenderMode::LocalContinueFallback,
            Some(GitHistoryNarrowReason::LocalContinueEngaged),
        ),
    };

    GitHistoryComponentRenderDisclosure {
        expected_mode,
        narrow_reason,
        needs_narrow_banner: expected_mode.is_narrowed(),
        // A detached/missing ref must spell out the exact target ref identity.
        needs_ref_identity_note: matches!(
            condition,
            GitHistoryRenderCondition::DetachedOrMissingRef
        ),
        // A narrowed recovery destination must stay named, never reduced to a badge.
        needs_recovery_note: matches!(expected_mode, GitHistoryRenderMode::RecoveryNarrowed),
        // A local-only continuation must stay explicit.
        needs_local_continue_note: matches!(
            expected_mode,
            GitHistoryRenderMode::LocalContinueFallback
        ),
    }
}

/// The parity state a render mode requires.
pub const fn parity_state_for_mode(mode: GitHistoryRenderMode) -> GitHistoryComponentParityState {
    match mode {
        GitHistoryRenderMode::FullParity => GitHistoryComponentParityState::FacetsPreserved,
        GitHistoryRenderMode::IdentityNarrowed
        | GitHistoryRenderMode::RecoveryNarrowed
        | GitHistoryRenderMode::LocalContinueFallback => {
            GitHistoryComponentParityState::FacetsDisclosedNarrowed
        }
    }
}

/// One consumer binding: a shared component rendered on one consumer surface.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GitHistoryComponentConsumerBinding {
    /// Stable binding id.
    pub binding_id: String,
    /// Stable Git-history-object id (shared across surfaces that show the same object).
    pub history_object_id: String,
    /// Human-readable Git-history-object identity.
    pub history_object_label: String,
    /// Which shared component this binding renders.
    pub component: M5GitHistoryComponent,
    /// Which consumer surface renders it.
    pub consumer: GitHistoryComponentConsumer,
    /// The render condition of the object on this surface.
    pub condition: GitHistoryRenderCondition,
    /// How much of the component this surface renders.
    pub render_mode: GitHistoryRenderMode,
    /// The parity facet values presented (identical across surfaces for one object).
    pub parity_facets: GitHistoryComponentParityFacetValues,
    /// Whether facets are preserved in full or a narrowing is disclosed.
    pub parity_state: GitHistoryComponentParityState,
    /// The explicit narrow banner; required and complete when the binding narrows.
    pub narrow_banner: Option<GitHistoryNarrowBanner>,
    /// Ref-identity note; required and non-empty when the disclosure demands it.
    pub ref_identity_note: String,
    /// Recovery note; required and non-empty when the disclosure demands it.
    pub recovery_note: String,
    /// Local-continue note; required and non-empty when the disclosure demands it.
    pub local_continue_note: String,
    /// Guardrail: this surface collapses multiple Git verbs into one ambiguous confirm.
    pub collapses_git_verb_into_ambiguous_confirm: bool,
    /// Guardrail: this surface hides the exact target ref or worktree.
    pub hides_exact_target_ref_or_worktree: bool,
    /// Guardrail: this surface lets conflict/recovery state disappear after a mutation.
    pub drops_conflict_or_recovery_state_after_mutation: bool,
    /// Guardrail: this surface rewords the ref/worktree/recovery labels per surface.
    pub rewords_ref_worktree_recovery_labels_per_surface: bool,
    /// Guardrail: this surface hides local-only recovery when provider state exists.
    pub hides_local_only_recovery_when_provider_linked: bool,
    /// Source contract refs this binding points at.
    pub source_contract_refs: Vec<String>,
}

impl GitHistoryComponentConsumerBinding {
    /// Disclosures this binding must carry, derived from its render condition.
    pub fn disclosure(&self) -> GitHistoryComponentRenderDisclosure {
        resolve_git_history_component_render_disclosure(self.condition)
    }

    /// Whether this binding renders below full parity.
    pub fn is_narrowed(&self) -> bool {
        self.render_mode.is_narrowed()
    }

    /// Whether every guardrail row-invariant is false, as required.
    pub fn guardrails_hold(&self) -> bool {
        !self.collapses_git_verb_into_ambiguous_confirm
            && !self.hides_exact_target_ref_or_worktree
            && !self.drops_conflict_or_recovery_state_after_mutation
            && !self.rewords_ref_worktree_recovery_labels_per_surface
            && !self.hides_local_only_recovery_when_provider_linked
    }

    /// Whether this binding points at the canonical component schema and matrix.
    pub fn points_at_canonical_contracts(&self) -> bool {
        let component_ref = component_canonical_schema_ref(self.component);
        self.source_contract_refs
            .iter()
            .any(|reference| reference == component_ref)
            && self.source_contract_refs.iter().any(|reference| {
                reference == GIT_HISTORY_COMPONENT_CONSUMER_COMPONENT_MATRIX_CONTRACT_REF
            })
    }
}

/// Trust and provenance review block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GitHistoryComponentConsumerTrustReview {
    /// Component reuse is proven by fixtures rather than inferred from screenshots.
    pub component_reuse_proven_by_fixtures: bool,
    /// The same object presents the same ref/worktree/recovery language across surfaces.
    pub same_object_same_language_across_surfaces: bool,
    /// The exact target ref and worktree are never hidden.
    pub exact_target_ref_and_worktree_never_hidden: bool,
    /// Conflict and recovery state survives a risky mutation.
    pub conflict_and_recovery_state_survives_mutation: bool,
    /// Primary Git verbs are identical across surfaces.
    pub primary_verbs_identical_across_surfaces: bool,
    /// Ref/worktree/recovery labels are identical across surfaces.
    pub ref_worktree_recovery_labels_identical_across_surfaces: bool,
    /// Local-only recovery stays explicit even with provider-linked review state.
    pub local_only_recovery_stays_explicit_with_provider_state: bool,
    /// A recovery destination stays reachable for risky components.
    pub recovery_destination_always_reachable_when_risky: bool,
    /// No Git verb is collapsed into one ambiguous confirm.
    pub no_git_verb_collapsed_into_ambiguous_confirm: bool,
    /// Help, support, and export consumers point at the canonical contracts.
    pub help_support_export_point_canonical_contracts: bool,
    /// Downgrade narrows the claim rather than hiding the component.
    pub downgrade_narrows_instead_of_hides: bool,
    /// Stale or underqualified bindings automatically block promotion.
    pub stale_or_underqualified_blocks_promotion: bool,
}

impl GitHistoryComponentConsumerTrustReview {
    /// Whether every invariant holds.
    pub const fn all_hold(&self) -> bool {
        self.component_reuse_proven_by_fixtures
            && self.same_object_same_language_across_surfaces
            && self.exact_target_ref_and_worktree_never_hidden
            && self.conflict_and_recovery_state_survives_mutation
            && self.primary_verbs_identical_across_surfaces
            && self.ref_worktree_recovery_labels_identical_across_surfaces
            && self.local_only_recovery_stays_explicit_with_provider_state
            && self.recovery_destination_always_reachable_when_risky
            && self.no_git_verb_collapsed_into_ambiguous_confirm
            && self.help_support_export_point_canonical_contracts
            && self.downgrade_narrows_instead_of_hides
            && self.stale_or_underqualified_blocks_promotion
    }
}

/// Consumer projection block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GitHistoryComponentConsumerProjection {
    /// The history sidebar reuses the shared components.
    pub history_sidebar_reuses_shared_components: bool,
    /// The risky-mutation sheet reuses the shared components.
    pub risky_mutation_sheet_reuses_shared_components: bool,
    /// The review-workspace banner reuses the shared components.
    pub review_workspace_banner_reuses_shared_components: bool,
    /// The command-help surface reuses the shared components.
    pub command_help_reuses_shared_components: bool,
    /// The support bundle reuses the shared components.
    pub support_bundle_reuses_shared_components: bool,
    /// The exported recovery packet reuses the shared components.
    pub exported_recovery_packet_reuses_shared_components: bool,
    /// Every component is adopted by two or more consumers.
    pub every_component_adopted_by_two_or_more_consumers: bool,
    /// Parity facets are identical for the same Git-history object.
    pub parity_facets_identical_for_same_object: bool,
    /// Narrowing is disclosed rather than hidden.
    pub narrowing_disclosed_not_hidden: bool,
    /// Export preserves ref/worktree/recovery identity.
    pub export_preserves_ref_worktree_recovery_identity: bool,
}

impl GitHistoryComponentConsumerProjection {
    /// Whether every projection invariant holds.
    pub const fn all_hold(&self) -> bool {
        self.history_sidebar_reuses_shared_components
            && self.risky_mutation_sheet_reuses_shared_components
            && self.review_workspace_banner_reuses_shared_components
            && self.command_help_reuses_shared_components
            && self.support_bundle_reuses_shared_components
            && self.exported_recovery_packet_reuses_shared_components
            && self.every_component_adopted_by_two_or_more_consumers
            && self.parity_facets_identical_for_same_object
            && self.narrowing_disclosed_not_hidden
            && self.export_preserves_ref_worktree_recovery_identity
    }
}

/// Proof freshness block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GitHistoryComponentConsumerProofFreshness {
    /// Proof-freshness SLO in hours.
    pub proof_freshness_slo_hours: u32,
    /// RFC 3339 timestamp of the last proof refresh.
    pub last_proof_refresh: String,
    /// True when stale proof automatically narrows the lane.
    pub auto_narrow_on_stale: bool,
}

/// Constructor input for [`GitHistoryComponentConsumerPacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GitHistoryComponentConsumerPacketInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable surface label.
    pub surface_label: String,
    /// Consumer bindings.
    pub consumer_bindings: Vec<GitHistoryComponentConsumerBinding>,
    /// Downgrade triggers that apply to this lane.
    pub downgrade_triggers: Vec<GitHistoryComponentConsumerDowngradeTrigger>,
    /// Consumer surfaces this packet covers.
    pub consumer_surfaces: Vec<GitHistoryComponentConsumer>,
    /// Trust review block.
    pub trust_review: GitHistoryComponentConsumerTrustReview,
    /// Consumer projection block.
    pub consumer_projection: GitHistoryComponentConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: GitHistoryComponentConsumerProofFreshness,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

/// Export-safe Git-history component consumer parity packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GitHistoryComponentConsumerPacket {
    /// Record kind; must equal [`GIT_HISTORY_COMPONENT_CONSUMER_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`GIT_HISTORY_COMPONENT_CONSUMER_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable surface label.
    pub surface_label: String,
    /// Consumer bindings.
    pub consumer_bindings: Vec<GitHistoryComponentConsumerBinding>,
    /// Downgrade triggers that apply to this lane.
    pub downgrade_triggers: Vec<GitHistoryComponentConsumerDowngradeTrigger>,
    /// Consumer surfaces this packet covers.
    pub consumer_surfaces: Vec<GitHistoryComponentConsumer>,
    /// Trust review block.
    pub trust_review: GitHistoryComponentConsumerTrustReview,
    /// Consumer projection block.
    pub consumer_projection: GitHistoryComponentConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: GitHistoryComponentConsumerProofFreshness,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

impl GitHistoryComponentConsumerPacket {
    /// Builds a Git-history component consumer packet from stable-lane input.
    pub fn new(input: GitHistoryComponentConsumerPacketInput) -> Self {
        Self {
            record_kind: GIT_HISTORY_COMPONENT_CONSUMER_RECORD_KIND.to_owned(),
            schema_version: GIT_HISTORY_COMPONENT_CONSUMER_SCHEMA_VERSION,
            packet_id: input.packet_id,
            surface_label: input.surface_label,
            consumer_bindings: input.consumer_bindings,
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

    /// Validates the Git-history component consumer parity invariants.
    ///
    /// Returns every violation found rather than stopping at the first, so a
    /// regenerator or CI gate can report the full set at once.
    pub fn validate(&self) -> Vec<GitHistoryComponentConsumerViolation> {
        let mut violations = Vec::new();

        if self.record_kind != GIT_HISTORY_COMPONENT_CONSUMER_RECORD_KIND {
            violations.push(GitHistoryComponentConsumerViolation::WrongRecordKind);
        }
        if self.schema_version != GIT_HISTORY_COMPONENT_CONSUMER_SCHEMA_VERSION {
            violations.push(GitHistoryComponentConsumerViolation::WrongSchemaVersion);
        }
        if self.packet_id.trim().is_empty()
            || self.surface_label.trim().is_empty()
            || self.redaction_class_token.trim().is_empty()
            || self.minted_at.trim().is_empty()
        {
            violations.push(GitHistoryComponentConsumerViolation::MissingIdentity);
        }
        if self.downgrade_triggers.is_empty() {
            violations.push(GitHistoryComponentConsumerViolation::DowngradeTriggersMissing);
        }
        if self.consumer_surfaces.is_empty() {
            violations.push(GitHistoryComponentConsumerViolation::ConsumerSurfacesMissing);
        }

        validate_source_contracts(self, &mut violations);
        validate_bindings(self, &mut violations);

        if !self.trust_review.all_hold() {
            violations.push(GitHistoryComponentConsumerViolation::TrustReviewIncomplete);
        }
        if !self.consumer_projection.all_hold() {
            violations.push(GitHistoryComponentConsumerViolation::ConsumerProjectionIncomplete);
        }
        if self.proof_freshness.proof_freshness_slo_hours == 0
            || self.proof_freshness.last_proof_refresh.trim().is_empty()
        {
            violations.push(GitHistoryComponentConsumerViolation::ProofFreshnessIncomplete);
        }

        if json_contains_forbidden_boundary_material(
            &serde_json::to_value(self).expect("git history component consumer packet serializes"),
        ) {
            violations.push(GitHistoryComponentConsumerViolation::RawBoundaryMaterialInExport);
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
            .expect("git history component consumer packet serializes")
    }

    /// Deterministic Markdown summary for support, review, or release handoff.
    pub fn render_markdown_summary(&self) -> String {
        let narrowed = self
            .consumer_bindings
            .iter()
            .filter(|binding| binding.is_narrowed())
            .count();

        let mut out = String::new();
        out.push_str(
            "# Shared Git-History Component Consumers: Ref, Worktree, and Recovery Parity\n\n",
        );
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- Surface: `{}`\n", self.surface_label));
        out.push_str(&format!(
            "- Consumer bindings: {} ({} narrowed)\n",
            self.consumer_bindings.len(),
            narrowed
        ));
        out.push_str(&format!(
            "- Proof freshness SLO: {} hours (last refresh: {})\n",
            self.proof_freshness.proof_freshness_slo_hours, self.proof_freshness.last_proof_refresh
        ));

        out.push_str("\n## Consumer bindings\n\n");
        for binding in &self.consumer_bindings {
            out.push_str(&format!(
                "- **{}** [`{}`]: component `{}` on `{}`, mode `{}`\n",
                binding.history_object_label,
                binding.binding_id,
                binding.component.as_str(),
                binding.consumer.as_str(),
                binding.render_mode.as_str(),
            ));
        }
        out
    }
}

/// Errors emitted when reading the checked-in Git-history component consumer export.
#[derive(Debug)]
pub enum GitHistoryComponentConsumerArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<GitHistoryComponentConsumerViolation>),
}

impl fmt::Display for GitHistoryComponentConsumerArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(
                    formatter,
                    "git history component consumer export parse failed: {error}"
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
                    "git history component consumer export failed validation: {tokens}"
                )
            }
        }
    }
}

impl Error for GitHistoryComponentConsumerArtifactError {}

/// Validation failures emitted by [`GitHistoryComponentConsumerPacket::validate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GitHistoryComponentConsumerViolation {
    /// Packet record kind is wrong.
    WrongRecordKind,
    /// Packet schema version is wrong.
    WrongSchemaVersion,
    /// Required identity field is missing.
    MissingIdentity,
    /// Source contract refs are incomplete.
    MissingSourceContracts,
    /// No consumer bindings are present.
    ConsumerBindingsMissing,
    /// A consumer binding is incomplete.
    BindingIncomplete,
    /// A binding's parity facet values are incomplete.
    ParityFacetIncomplete,
    /// A binding's render mode does not match its render condition.
    RenderModeMismatch,
    /// A binding's parity state does not match its render mode.
    ParityStateMismatch,
    /// Two surfaces show the same Git-history object with different parity language.
    ParityDriftAcrossSurfaces,
    /// A shared component is not adopted by at least two distinct consumers.
    GitHistoryComponentReuseUnproven,
    /// A Help, support, or export binding does not point at the canonical contracts.
    HelpSupportExportReferenceMissing,
    /// A narrowed binding is missing its explicit narrow banner.
    NarrowBannerMissing,
    /// A narrow banner's reason does not match the required narrow reason.
    NarrowReasonMismatch,
    /// A narrow banner is missing its preserved-facets note.
    NarrowBannerPreservedFacetsMissing,
    /// A narrow banner is missing its next-action copy.
    NarrowNextActionMissing,
    /// A binding that must spell out the exact target ref is missing its note.
    RefIdentityNoteMissing,
    /// A binding that must name the recovery destination is missing its note.
    RecoveryNoteMissing,
    /// A binding that must preserve a local-continue path is missing its note.
    LocalContinueNoteMissing,
    /// A binding collapses multiple Git verbs into one ambiguous confirm.
    GitVerbCollapsedIntoAmbiguousConfirm,
    /// A binding hides the exact target ref or worktree.
    ExactTargetRefOrWorktreeHidden,
    /// A binding lets conflict/recovery state disappear after a mutation.
    ConflictOrRecoveryStateDroppedAfterMutation,
    /// A binding rewords the ref/worktree/recovery labels per surface.
    RefWorktreeRecoveryLabelsRewordedPerSurface,
    /// A binding hides local-only recovery when provider-linked state exists.
    LocalOnlyRecoveryHiddenWhenProviderLinked,
    /// Not every consumer surface appears among the bindings.
    ConsumerCoverageMissing,
    /// Not every shared component appears among the bindings.
    ComponentCoverageMissing,
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

impl GitHistoryComponentConsumerViolation {
    /// Stable token used in tests and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WrongRecordKind => "wrong_record_kind",
            Self::WrongSchemaVersion => "wrong_schema_version",
            Self::MissingIdentity => "missing_identity",
            Self::MissingSourceContracts => "missing_source_contracts",
            Self::ConsumerBindingsMissing => "consumer_bindings_missing",
            Self::BindingIncomplete => "binding_incomplete",
            Self::ParityFacetIncomplete => "parity_facet_incomplete",
            Self::RenderModeMismatch => "render_mode_mismatch",
            Self::ParityStateMismatch => "parity_state_mismatch",
            Self::ParityDriftAcrossSurfaces => "parity_drift_across_surfaces",
            Self::GitHistoryComponentReuseUnproven => "git_history_component_reuse_unproven",
            Self::HelpSupportExportReferenceMissing => "help_support_export_reference_missing",
            Self::NarrowBannerMissing => "narrow_banner_missing",
            Self::NarrowReasonMismatch => "narrow_reason_mismatch",
            Self::NarrowBannerPreservedFacetsMissing => "narrow_banner_preserved_facets_missing",
            Self::NarrowNextActionMissing => "narrow_next_action_missing",
            Self::RefIdentityNoteMissing => "ref_identity_note_missing",
            Self::RecoveryNoteMissing => "recovery_note_missing",
            Self::LocalContinueNoteMissing => "local_continue_note_missing",
            Self::GitVerbCollapsedIntoAmbiguousConfirm => {
                "git_verb_collapsed_into_ambiguous_confirm"
            }
            Self::ExactTargetRefOrWorktreeHidden => "exact_target_ref_or_worktree_hidden",
            Self::ConflictOrRecoveryStateDroppedAfterMutation => {
                "conflict_or_recovery_state_dropped_after_mutation"
            }
            Self::RefWorktreeRecoveryLabelsRewordedPerSurface => {
                "ref_worktree_recovery_labels_reworded_per_surface"
            }
            Self::LocalOnlyRecoveryHiddenWhenProviderLinked => {
                "local_only_recovery_hidden_when_provider_linked"
            }
            Self::ConsumerCoverageMissing => "consumer_coverage_missing",
            Self::ComponentCoverageMissing => "component_coverage_missing",
            Self::DowngradeTriggersMissing => "downgrade_triggers_missing",
            Self::ConsumerSurfacesMissing => "consumer_surfaces_missing",
            Self::TrustReviewIncomplete => "trust_review_incomplete",
            Self::ConsumerProjectionIncomplete => "consumer_projection_incomplete",
            Self::ProofFreshnessIncomplete => "proof_freshness_incomplete",
            Self::RawBoundaryMaterialInExport => "raw_boundary_material_in_export",
        }
    }
}

/// Reads and validates the checked-in stable Git-history component consumer export.
///
/// # Errors
///
/// Returns [`GitHistoryComponentConsumerArtifactError`] when the checked-in export
/// fails to parse or violates the frozen contract.
pub fn current_git_history_component_consumer_export(
) -> Result<GitHistoryComponentConsumerPacket, GitHistoryComponentConsumerArtifactError> {
    let packet: GitHistoryComponentConsumerPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/release/m5-git-history-component-consumers-proof/support_export.json"
    )))
    .map_err(GitHistoryComponentConsumerArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(GitHistoryComponentConsumerArtifactError::Validation(
            violations,
        ))
    }
}

fn validate_source_contracts(
    packet: &GitHistoryComponentConsumerPacket,
    violations: &mut Vec<GitHistoryComponentConsumerViolation>,
) {
    let refs: BTreeSet<&str> = packet
        .source_contract_refs
        .iter()
        .map(String::as_str)
        .collect();
    for required in [
        GIT_HISTORY_COMPONENT_CONSUMER_SCHEMA_REF,
        GIT_HISTORY_COMPONENT_CONSUMER_DOC_REF,
        GIT_HISTORY_COMPONENT_CONSUMER_COMPONENT_MATRIX_CONTRACT_REF,
        GIT_HISTORY_COMPONENT_CONSUMER_IDENTITY_CONTRACT_REF,
        GIT_HISTORY_COMPONENT_CONSUMER_STASH_RECOVERY_CONTRACT_REF,
        GIT_HISTORY_COMPONENT_CONSUMER_SEQUENCE_EDIT_CONTRACT_REF,
        GIT_HISTORY_COMPONENT_CONSUMER_MUTATION_REVIEW_CONTRACT_REF,
    ] {
        if !refs.contains(required) {
            violations.push(GitHistoryComponentConsumerViolation::MissingSourceContracts);
            return;
        }
    }
}

fn validate_bindings(
    packet: &GitHistoryComponentConsumerPacket,
    violations: &mut Vec<GitHistoryComponentConsumerViolation>,
) {
    if packet.consumer_bindings.is_empty() {
        violations.push(GitHistoryComponentConsumerViolation::ConsumerBindingsMissing);
        return;
    }

    // Parity: the parity facet values must be identical for every binding that
    // renders the same Git-history object.
    let mut object_facets: BTreeMap<&str, &GitHistoryComponentParityFacetValues> = BTreeMap::new();
    let mut parity_drift_reported = false;

    // Reuse: each component must be adopted by at least two distinct consumers.
    let mut component_consumers: BTreeMap<
        M5GitHistoryComponent,
        BTreeSet<GitHistoryComponentConsumer>,
    > = BTreeMap::new();
    let mut seen_consumers: BTreeSet<GitHistoryComponentConsumer> = BTreeSet::new();
    let mut seen_components: BTreeSet<M5GitHistoryComponent> = BTreeSet::new();

    for binding in &packet.consumer_bindings {
        if binding.binding_id.trim().is_empty()
            || binding.history_object_id.trim().is_empty()
            || binding.history_object_label.trim().is_empty()
            || binding.source_contract_refs.is_empty()
        {
            violations.push(GitHistoryComponentConsumerViolation::BindingIncomplete);
        }
        if !binding.parity_facets.all_present() {
            violations.push(GitHistoryComponentConsumerViolation::ParityFacetIncomplete);
        }

        let disclosure = binding.disclosure();

        if binding.render_mode != disclosure.expected_mode {
            violations.push(GitHistoryComponentConsumerViolation::RenderModeMismatch);
        }
        if binding.parity_state != parity_state_for_mode(binding.render_mode) {
            violations.push(GitHistoryComponentConsumerViolation::ParityStateMismatch);
        }

        // Narrowing disclosure.
        if disclosure.needs_narrow_banner {
            match &binding.narrow_banner {
                None => {
                    violations.push(GitHistoryComponentConsumerViolation::NarrowBannerMissing);
                }
                Some(banner) => {
                    if Some(banner.reason) != disclosure.narrow_reason {
                        violations.push(GitHistoryComponentConsumerViolation::NarrowReasonMismatch);
                    }
                    if banner.preserved_facets_note.trim().is_empty() {
                        violations.push(
                            GitHistoryComponentConsumerViolation::NarrowBannerPreservedFacetsMissing,
                        );
                    }
                    if banner.next_action_label.trim().is_empty() {
                        violations
                            .push(GitHistoryComponentConsumerViolation::NarrowNextActionMissing);
                    }
                }
            }
        } else if binding.narrow_banner.is_some() {
            // A full-parity binding must not carry a narrow banner.
            violations.push(GitHistoryComponentConsumerViolation::NarrowBannerMissing);
        }

        if disclosure.needs_ref_identity_note && binding.ref_identity_note.trim().is_empty() {
            violations.push(GitHistoryComponentConsumerViolation::RefIdentityNoteMissing);
        }
        if disclosure.needs_recovery_note && binding.recovery_note.trim().is_empty() {
            violations.push(GitHistoryComponentConsumerViolation::RecoveryNoteMissing);
        }
        if disclosure.needs_local_continue_note && binding.local_continue_note.trim().is_empty() {
            violations.push(GitHistoryComponentConsumerViolation::LocalContinueNoteMissing);
        }

        // Guardrail row-invariants (each must be false).
        if binding.collapses_git_verb_into_ambiguous_confirm {
            violations
                .push(GitHistoryComponentConsumerViolation::GitVerbCollapsedIntoAmbiguousConfirm);
        }
        if binding.hides_exact_target_ref_or_worktree {
            violations.push(GitHistoryComponentConsumerViolation::ExactTargetRefOrWorktreeHidden);
        }
        if binding.drops_conflict_or_recovery_state_after_mutation {
            violations.push(
                GitHistoryComponentConsumerViolation::ConflictOrRecoveryStateDroppedAfterMutation,
            );
        }
        if binding.rewords_ref_worktree_recovery_labels_per_surface {
            violations.push(
                GitHistoryComponentConsumerViolation::RefWorktreeRecoveryLabelsRewordedPerSurface,
            );
        }
        if binding.hides_local_only_recovery_when_provider_linked {
            violations.push(
                GitHistoryComponentConsumerViolation::LocalOnlyRecoveryHiddenWhenProviderLinked,
            );
        }

        // Help / support / export consumers must point at the canonical contracts.
        if binding.consumer.is_help_support_or_export() && !binding.points_at_canonical_contracts()
        {
            violations
                .push(GitHistoryComponentConsumerViolation::HelpSupportExportReferenceMissing);
        }

        // Parity drift accumulation.
        match object_facets.get(binding.history_object_id.as_str()) {
            None => {
                object_facets.insert(binding.history_object_id.as_str(), &binding.parity_facets);
            }
            Some(existing) => {
                if **existing != binding.parity_facets && !parity_drift_reported {
                    violations
                        .push(GitHistoryComponentConsumerViolation::ParityDriftAcrossSurfaces);
                    parity_drift_reported = true;
                }
            }
        }

        component_consumers
            .entry(binding.component)
            .or_default()
            .insert(binding.consumer);
        seen_consumers.insert(binding.consumer);
        seen_components.insert(binding.component);
    }

    // Coverage: every consumer and every component must appear.
    for consumer in GitHistoryComponentConsumer::ALL {
        if !seen_consumers.contains(&consumer) {
            violations.push(GitHistoryComponentConsumerViolation::ConsumerCoverageMissing);
            break;
        }
    }
    for component in M5GitHistoryComponent::ALL {
        if !seen_components.contains(&component) {
            violations.push(GitHistoryComponentConsumerViolation::ComponentCoverageMissing);
            break;
        }
    }

    // Reuse: every present component must be adopted by two or more distinct consumers.
    for consumers in component_consumers.values() {
        if consumers.len() < 2 {
            violations.push(GitHistoryComponentConsumerViolation::GitHistoryComponentReuseUnproven);
            break;
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
