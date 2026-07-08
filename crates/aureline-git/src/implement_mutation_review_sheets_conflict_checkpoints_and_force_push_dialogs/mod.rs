//! Cherry-pick/revert review sheets, patch-apply review sheets,
//! conflict-checkpoint cards, and force-push review dialogs with exact
//! target-ref, publish consequence, and recovery truth across claimed M5 Git
//! mutation flows.
//!
//! This module narrows the four **risky-mutation review** components frozen in
//! [`crate::freeze_the_m5_git_history_sequence_component_matrix`] —
//! `cherry_pick_revert_review_sheet`, `patch_apply_review_sheet`,
//! `conflict_checkpoint_card`, and `force_push_review_dialog` — into an
//! implemented, export-safe row contract. Three of the four surfaces drive a
//! history-mutating verb ([`CherryPickRevertReviewSheet`],
//! [`PatchApplyReviewSheet`], and [`ForcePushReviewDialog`]); the fourth
//! ([`ConflictCheckpointCard`]) is a read-only recovery surface that keeps the
//! base/ours/theirs/result context, unresolved count, and reopen/restore
//! behavior of a conflict checkpoint visible after a risky mutation.
//!
//! Two honesty axes anchor the acceptance criteria. First, no claimed risky Git
//! flow may use one ambiguous confirm button or hide the exact target ref /
//! worktree being mutated: every risky surface confirms as its own distinct
//! mutation-review class ([`MutationReviewClass`]) rather than a shared confirm
//! (`MutationVerbConfirmCollapsed`), always names the target ref and worktree
//! (`TargetRefWorktreeMissing`), and always keeps a reachable rollback and
//! recovery checkpoint (`RollbackActionMissing`, `MutationRecoveryUnreachable`).
//! Second, conflict and publish consequences stay explicit even when the same
//! change also touches hosted review state: every risky surface discloses what it
//! will publish or rewrite (`PublishConsequenceMissing`), and when a surface also
//! affects a hosted review it discloses the approval consequence
//! (`ApprovalConsequenceMissing`) instead of silently invalidating it — while the
//! conflict-checkpoint card keeps local recovery context visible regardless of any
//! provider-linked review.
//!
//! The shared downgrade vocabulary ([`GitHistoryDowngradeState`]), the shared
//! consumer surfaces ([`ComponentConsumerSurface`]), and the shared
//! mutation-review class ([`MutationReviewClass`]) are reused directly from the
//! frozen matrix so downgrades, parity, and each distinct confirm read the same
//! everywhere. Local-only recovery stays explicit even when a provider-linked
//! review state also exists. Raw patch bytes, raw object bodies, raw provider
//! payloads, and credentials stay outside the support boundary.
//!
//! The boundary schema is
//! [`schemas/ui/m5-git-mutation-review-recovery-component.schema.json`](../../../../schemas/ui/m5-git-mutation-review-recovery-component.schema.json).
//! The contract doc is
//! [`docs/git/m5/implement_mutation_review_sheets_conflict_checkpoints_and_force_push_dialogs.md`](../../../../docs/git/m5/implement_mutation_review_sheets_conflict_checkpoints_and_force_push_dialogs.md).
//! The protected fixture directory is
//! [`fixtures/ui/m5-git-mutation-review-recovery-components/`](../../../../fixtures/ui/m5-git-mutation-review-recovery-components/).

#[cfg(test)]
mod tests;

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

use crate::freeze_the_m5_git_history_sequence_component_matrix::{
    ComponentConsumerSurface, GitHistoryDowngradeState, M5GitHistoryComponent, MutationReviewClass,
};

/// Stable record-kind tag carried by [`GitMutationReviewPacket`].
pub const GIT_MUTATION_REVIEW_RECORD_KIND: &str = "git_mutation_review_recovery_component_truth";

/// Schema version for mutation-review / recovery component records.
pub const GIT_MUTATION_REVIEW_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the boundary schema.
pub const GIT_MUTATION_REVIEW_SCHEMA_REF: &str =
    "schemas/ui/m5-git-mutation-review-recovery-component.schema.json";

/// Repo-relative path of the contract doc.
pub const GIT_MUTATION_REVIEW_DOC_REF: &str =
    "docs/git/m5/implement_mutation_review_sheets_conflict_checkpoints_and_force_push_dialogs.md";

/// Repo-relative path of the frozen component matrix this lane implements.
pub const GIT_MUTATION_REVIEW_COMPONENT_MATRIX_CONTRACT_REF: &str =
    "schemas/ui/m5-git-history-sequence-component-matrix.schema.json";

/// Repo-relative path of the canonical history-surgery review contract.
pub const GIT_MUTATION_REVIEW_HISTORY_SURGERY_CONTRACT_REF: &str =
    "schemas/git/history-surgery-review.schema.json";

/// Repo-relative path of the canonical conflict-session contract.
pub const GIT_MUTATION_REVIEW_CONFLICT_SESSION_CONTRACT_REF: &str =
    "schemas/git/conflict_session.schema.json";

/// Repo-relative path of the canonical ref-update lineage contract.
pub const GIT_MUTATION_REVIEW_REF_UPDATE_CONTRACT_REF: &str =
    "schemas/review/sequence-edit-conflict-session-stash-entry-ref-update.schema.json";

/// Repo-relative path of the protected fixture directory.
pub const GIT_MUTATION_REVIEW_FIXTURE_DIR: &str =
    "fixtures/ui/m5-git-mutation-review-recovery-components";

/// Repo-relative path of the checked support-export artifact.
pub const GIT_MUTATION_REVIEW_ARTIFACT_REF: &str =
    "artifacts/release/m5-git-mutation-review-recovery-components-proof/support_export.json";

/// Repo-relative path of the checked Markdown summary.
pub const GIT_MUTATION_REVIEW_SUMMARY_REF: &str =
    "artifacts/release/m5-git-mutation-review-recovery-components-proof/summary.md";

/// The four mutation-review / recovery components this lane implements.
pub const GIT_MUTATION_REVIEW_COMPONENTS: [M5GitHistoryComponent; 4] = [
    M5GitHistoryComponent::CherryPickRevertReviewSheet,
    M5GitHistoryComponent::PatchApplyReviewSheet,
    M5GitHistoryComponent::ConflictCheckpointCard,
    M5GitHistoryComponent::ForcePushReviewDialog,
];

/// The mutation-review class a given component must confirm as.
///
/// This is the anti-ambiguous-confirm anchor: each risky verb keeps its own
/// distinct confirm rather than collapsing into a shared button, and the
/// read-only checkpoint card must never claim a mutating class.
pub const fn expected_review_class(component: M5GitHistoryComponent) -> MutationReviewClass {
    match component {
        M5GitHistoryComponent::CherryPickRevertReviewSheet => {
            MutationReviewClass::ExplicitVerbConfirm
        }
        M5GitHistoryComponent::PatchApplyReviewSheet => MutationReviewClass::PatchApplyConfirm,
        M5GitHistoryComponent::ForcePushReviewDialog => MutationReviewClass::ForcePushConfirm,
        _ => MutationReviewClass::DisplayOnlyNoMutation,
    }
}

/// Whether a risky mutation also touches hosted review state, and how.
///
/// The `LocalOnly` case affects no hosted review; every other case affects a
/// hosted review and so must disclose an explicit approval consequence rather
/// than silently invalidating it.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum HostedReviewImpact {
    /// No hosted review is affected; the mutation is purely local.
    LocalOnly,
    /// The mutation updates an open hosted review (new commits appear there).
    UpdatesHostedReview,
    /// The mutation rewrites reviewed commits and invalidates their approval.
    InvalidatesApproval,
    /// The mutation makes the local branch diverge from its hosted counterpart.
    DivergesFromHosted,
}

impl HostedReviewImpact {
    /// Every impact, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::LocalOnly,
        Self::UpdatesHostedReview,
        Self::InvalidatesApproval,
        Self::DivergesFromHosted,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LocalOnly => "local_only",
            Self::UpdatesHostedReview => "updates_hosted_review",
            Self::InvalidatesApproval => "invalidates_approval",
            Self::DivergesFromHosted => "diverges_from_hosted",
        }
    }

    /// Whether this impact touches a hosted review and so must disclose the
    /// approval consequence explicitly.
    pub const fn affects_hosted_review(self) -> bool {
        !matches!(self, Self::LocalOnly)
    }
}

/// Whether a risky mutation is ready to run, or blocked, and why.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MutationBlockerState {
    /// No blocker; the mutation can run against the named target.
    Ready,
    /// The mutation is expected to conflict and stop at a checkpoint.
    ConflictExpected,
    /// The worktree is dirty at the target and must be cleaned or stashed first.
    DirtyWorktree,
    /// A protected-ref policy blocks the mutation until it is satisfied.
    ProtectedRefPolicy,
    /// The rewritten/pushed ref must be signed before it can complete.
    SignatureRequired,
}

impl MutationBlockerState {
    /// Every blocker state, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::Ready,
        Self::ConflictExpected,
        Self::DirtyWorktree,
        Self::ProtectedRefPolicy,
        Self::SignatureRequired,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Ready => "ready",
            Self::ConflictExpected => "conflict_expected",
            Self::DirtyWorktree => "dirty_worktree",
            Self::ProtectedRefPolicy => "protected_ref_policy",
            Self::SignatureRequired => "signature_required",
        }
    }

    /// Whether the mutation is currently blocked (anything but ready).
    pub const fn is_blocked(self) -> bool {
        !matches!(self, Self::Ready)
    }
}

/// Availability of a recovery checkpoint for a risky mutation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MutationCheckpointState {
    /// A pre-mutation checkpoint was captured and is reachable.
    PreMutationCaptured,
    /// No explicit checkpoint; only a reflog-based recovery fallback exists.
    ReflogFallbackOnly,
    /// No checkpoint and no recovery is available yet.
    Unavailable,
}

impl MutationCheckpointState {
    /// Every checkpoint state, in declaration order.
    pub const ALL: [Self; 3] = [
        Self::PreMutationCaptured,
        Self::ReflogFallbackOnly,
        Self::Unavailable,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::PreMutationCaptured => "pre_mutation_captured",
            Self::ReflogFallbackOnly => "reflog_fallback_only",
            Self::Unavailable => "unavailable",
        }
    }

    /// Whether the mutation can be recovered at all.
    pub const fn is_recoverable(self) -> bool {
        !matches!(self, Self::Unavailable)
    }
}

/// The distinct verb a cherry-pick/revert review sheet confirms.
///
/// Cherry-pick and revert are never collapsed into one ambiguous confirm: a
/// cherry-pick replays a commit's change forward, while a revert applies its
/// inverse.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CherryPickRevertVerb {
    /// Replay the named commit's change onto the target.
    CherryPick,
    /// Apply the inverse of the named commit's change onto the target.
    Revert,
}

impl CherryPickRevertVerb {
    /// Both verbs, in declaration order.
    pub const ALL: [Self; 2] = [Self::CherryPick, Self::Revert];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::CherryPick => "cherry_pick",
            Self::Revert => "revert",
        }
    }
}

/// Where the bytes for a patch-apply come from.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PatchSource {
    /// A mailbox series of one or more formatted commits (`git am`).
    MailboxSeries,
    /// A single unified diff file.
    DiffFile,
    /// A range-diff produced from two commit ranges.
    RangeDiff,
}

impl PatchSource {
    /// Every source, in declaration order.
    pub const ALL: [Self; 3] = [Self::MailboxSeries, Self::DiffFile, Self::RangeDiff];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::MailboxSeries => "mailbox_series",
            Self::DiffFile => "diff_file",
            Self::RangeDiff => "range_diff",
        }
    }
}

/// How a patch-apply writes its changes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PatchApplyMode {
    /// Fall back to a three-way merge when context does not match exactly.
    ThreeWayMerge,
    /// Apply to both the index and the worktree.
    IndexAndWorktree,
    /// Check only whether the patch would apply; write nothing.
    CheckOnly,
}

impl PatchApplyMode {
    /// Every mode, in declaration order.
    pub const ALL: [Self; 3] = [Self::ThreeWayMerge, Self::IndexAndWorktree, Self::CheckOnly];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ThreeWayMerge => "three_way_merge",
            Self::IndexAndWorktree => "index_and_worktree",
            Self::CheckOnly => "check_only",
        }
    }

    /// Whether the mode actually writes changes (check-only does not).
    pub const fn writes_changes(self) -> bool {
        !matches!(self, Self::CheckOnly)
    }
}

/// One side of a captured merge conflict.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ConflictSide {
    /// The common ancestor content.
    Base,
    /// The content on the current side (`HEAD`).
    Ours,
    /// The content being merged in.
    Theirs,
    /// The resolved result content.
    Result,
}

impl ConflictSide {
    /// Every side, in declaration order.
    pub const ALL: [Self; 4] = [Self::Base, Self::Ours, Self::Theirs, Self::Result];

    /// The three sides every conflict checkpoint must always keep visible.
    pub const REQUIRED_CONTEXT: [Self; 3] = [Self::Base, Self::Ours, Self::Theirs];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Base => "base",
            Self::Ours => "ours",
            Self::Theirs => "theirs",
            Self::Result => "result",
        }
    }
}

/// How a conflict checkpoint can be reopened or restored.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CheckpointReopenState {
    /// Still unresolved and reopenable for further editing.
    Reopenable,
    /// Resolved but restorable back to the captured checkpoint.
    RestorableFromCheckpoint,
    /// Resolved and applied to the result.
    ResolvedApplied,
}

impl CheckpointReopenState {
    /// Every reopen state, in declaration order.
    pub const ALL: [Self; 3] = [
        Self::Reopenable,
        Self::RestorableFromCheckpoint,
        Self::ResolvedApplied,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Reopenable => "reopenable",
            Self::RestorableFromCheckpoint => "restorable_from_checkpoint",
            Self::ResolvedApplied => "resolved_applied",
        }
    }

    /// Whether the checkpoint still offers a way back in (reopen or restore).
    pub const fn offers_reopen_path(self) -> bool {
        matches!(self, Self::Reopenable | Self::RestorableFromCheckpoint)
    }
}

/// The lease safety of a force-push (ref rewrite).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ForcePushSafety {
    /// `--force-with-lease`: refuse if the remote moved unexpectedly.
    ForceWithLease,
    /// `--force-with-lease=<ref>:<expected>`: refuse unless the remote matches.
    ForceWithLeaseExpecting,
    /// `--force`: overwrite unconditionally (no lease protection).
    PlainForce,
}

impl ForcePushSafety {
    /// Every safety mode, in declaration order.
    pub const ALL: [Self; 3] = [
        Self::ForceWithLease,
        Self::ForceWithLeaseExpecting,
        Self::PlainForce,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ForceWithLease => "force_with_lease",
            Self::ForceWithLeaseExpecting => "force_with_lease_expecting",
            Self::PlainForce => "plain_force",
        }
    }

    /// Whether a lease guards the remote tip before it is overwritten.
    pub const fn uses_lease(self) -> bool {
        !matches!(self, Self::PlainForce)
    }
}

/// Recovery disclosures a risky mutation surface must carry.
///
/// Every risky surface must name its target, disclose what it publishes, keep a
/// rollback reachable, and — when it also affects hosted review — disclose the
/// approval consequence rather than silently invalidating it.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MutationReviewDisclosure {
    /// Whether the surface confirms its own distinct verb (never a shared confirm).
    pub requires_distinct_confirm: bool,
    /// Whether the surface must name its exact target ref and worktree.
    pub must_name_target: bool,
    /// Whether the surface must disclose its publish/rewrite consequence.
    pub must_disclose_publish_consequence: bool,
    /// Whether the surface must disclose the approval consequence.
    pub must_disclose_approval_consequence: bool,
    /// Whether the surface must keep a reachable recovery checkpoint.
    pub must_stay_recoverable: bool,
}

/// Resolves the disclosures a risky mutation surface must carry from its
/// component and its hosted-review impact.
///
/// A risky component always confirms distinctly, names its target, discloses its
/// publish consequence, and stays recoverable; the approval consequence becomes
/// required only when the change also touches a hosted review.
pub fn resolve_mutation_review_disclosure(
    component: M5GitHistoryComponent,
    hosted_impact: HostedReviewImpact,
) -> MutationReviewDisclosure {
    let risky = component.is_risky_mutation_surface();
    MutationReviewDisclosure {
        requires_distinct_confirm: risky,
        must_name_target: risky,
        must_disclose_publish_consequence: risky,
        must_disclose_approval_consequence: hosted_impact.affects_hosted_review(),
        must_stay_recoverable: risky,
    }
}

/// Common view over a risky mutation surface, used to validate the shared
/// target/publish/approval/recovery invariants once for all three risky surfaces.
struct RiskySurfaceView<'a> {
    component: M5GitHistoryComponent,
    review_class: MutationReviewClass,
    target_ref: &'a str,
    target_worktree: &'a str,
    publish_consequence: &'a str,
    approval_consequence: &'a str,
    hosted_review_impact: HostedReviewImpact,
    blocked_state: MutationBlockerState,
    blocker_disclosure: &'a str,
    rollback_action: &'a str,
    checkpoint_state: MutationCheckpointState,
    checkpoint_disclosure: &'a str,
    downgrade_vocab_empty: bool,
    fields_shown_empty: bool,
    source_contract_refs_empty: bool,
}

fn validate_risky_surface(
    view: &RiskySurfaceView<'_>,
    violations: &mut Vec<GitMutationReviewViolation>,
) {
    let disclosure = resolve_mutation_review_disclosure(view.component, view.hosted_review_impact);

    if view.downgrade_vocab_empty || view.fields_shown_empty || view.source_contract_refs_empty {
        violations.push(GitMutationReviewViolation::RiskySurfaceIncomplete);
    }
    if view.review_class != expected_review_class(view.component) {
        violations.push(GitMutationReviewViolation::MutationVerbConfirmCollapsed);
    }
    if disclosure.must_name_target
        && (view.target_ref.trim().is_empty() || view.target_worktree.trim().is_empty())
    {
        violations.push(GitMutationReviewViolation::TargetRefWorktreeMissing);
    }
    if disclosure.must_disclose_publish_consequence && view.publish_consequence.trim().is_empty() {
        violations.push(GitMutationReviewViolation::PublishConsequenceMissing);
    }
    if disclosure.must_disclose_approval_consequence && view.approval_consequence.trim().is_empty()
    {
        violations.push(GitMutationReviewViolation::ApprovalConsequenceMissing);
    }
    if disclosure.must_stay_recoverable && view.rollback_action.trim().is_empty() {
        violations.push(GitMutationReviewViolation::RollbackActionMissing);
    }
    if view.blocked_state.is_blocked() && view.blocker_disclosure.trim().is_empty() {
        violations.push(GitMutationReviewViolation::MutationBlockerNotDisclosed);
    }
    if view.checkpoint_disclosure.trim().is_empty() {
        violations.push(GitMutationReviewViolation::MutationCheckpointUndisclosed);
    }
    if disclosure.must_stay_recoverable && !view.checkpoint_state.is_recoverable() {
        violations.push(GitMutationReviewViolation::MutationRecoveryUnreachable);
    }
}

/// One cherry-pick / revert review sheet shown before the verb runs.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CherryPickRevertReviewSheet {
    /// Stable row id.
    pub row_id: String,
    /// Canonical component; must be `cherry_pick_revert_review_sheet`.
    pub component: M5GitHistoryComponent,
    /// The distinct verb this sheet confirms (cherry-pick vs revert).
    pub verb: CherryPickRevertVerb,
    /// Short id of the commit being replayed/inverted (explicit identity).
    pub source_commit_short_id: String,
    /// Subject of the source commit (explicit identity).
    pub source_commit_subject: String,
    /// The exact target ref the new commit lands on (for example `main`).
    pub target_ref: String,
    /// The exact worktree the mutation runs in.
    pub target_worktree: String,
    /// The mutation-review class this sheet confirms; must be `explicit_verb_confirm`.
    pub review_class: MutationReviewClass,
    /// What this verb publishes/creates (a new commit on the target).
    pub publish_consequence: String,
    /// How the change affects hosted review state.
    pub hosted_review_impact: HostedReviewImpact,
    /// The explicit approval consequence when hosted review is affected.
    pub approval_consequence: String,
    /// Whether the mutation is ready or blocked, and why.
    pub blocked_state: MutationBlockerState,
    /// Human-readable disclosure of any blocker.
    pub blocker_disclosure: String,
    /// The rollback action that undoes this verb.
    pub rollback_action: String,
    /// Checkpoint availability for this mutation.
    pub checkpoint_state: MutationCheckpointState,
    /// Human-readable disclosure of the checkpoint state.
    pub checkpoint_disclosure: String,
    /// Shared downgrade states this component may surface.
    pub downgrade_vocab: Vec<GitHistoryDowngradeState>,
    /// Component fields the surface projects, in display order.
    pub fields_shown: Vec<String>,
    /// Source contract refs consumed by this component.
    pub source_contract_refs: Vec<String>,
}

impl CherryPickRevertReviewSheet {
    fn risky_view(&self) -> RiskySurfaceView<'_> {
        RiskySurfaceView {
            component: self.component,
            review_class: self.review_class,
            target_ref: &self.target_ref,
            target_worktree: &self.target_worktree,
            publish_consequence: &self.publish_consequence,
            approval_consequence: &self.approval_consequence,
            hosted_review_impact: self.hosted_review_impact,
            blocked_state: self.blocked_state,
            blocker_disclosure: &self.blocker_disclosure,
            rollback_action: &self.rollback_action,
            checkpoint_state: self.checkpoint_state,
            checkpoint_disclosure: &self.checkpoint_disclosure,
            downgrade_vocab_empty: self.downgrade_vocab.is_empty(),
            fields_shown_empty: self.fields_shown.is_empty(),
            source_contract_refs_empty: self.source_contract_refs.is_empty(),
        }
    }

    /// Whether the source commit identity is fully explicit (id and subject).
    pub fn source_identity_explicit(&self) -> bool {
        !self.source_commit_short_id.trim().is_empty()
            && !self.source_commit_subject.trim().is_empty()
    }
}

/// One patch-apply review sheet shown before a patch/mailbox is applied.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PatchApplyReviewSheet {
    /// Stable row id.
    pub row_id: String,
    /// Canonical component; must be `patch_apply_review_sheet`.
    pub component: M5GitHistoryComponent,
    /// Where the patch bytes come from.
    pub patch_source: PatchSource,
    /// How the patch is written.
    pub apply_mode: PatchApplyMode,
    /// The exact target ref the patch applies onto.
    pub target_ref: String,
    /// The exact worktree the patch applies in.
    pub target_worktree: String,
    /// Number of commits carried by the patch (>=1 for a mailbox series).
    pub commit_count: u32,
    /// Number of files the patch touches.
    pub affected_file_count: u32,
    /// The exact paths the patch touches (must match `affected_file_count`).
    pub affected_paths: Vec<String>,
    /// The mutation-review class this sheet confirms; must be `patch_apply_confirm`.
    pub review_class: MutationReviewClass,
    /// What applying the patch publishes/writes.
    pub publish_consequence: String,
    /// How the change affects hosted review state.
    pub hosted_review_impact: HostedReviewImpact,
    /// The explicit approval consequence when hosted review is affected.
    pub approval_consequence: String,
    /// Whether the mutation is ready or blocked, and why.
    pub blocked_state: MutationBlockerState,
    /// Human-readable disclosure of any blocker.
    pub blocker_disclosure: String,
    /// The rollback action that undoes the apply.
    pub rollback_action: String,
    /// Checkpoint availability for this mutation.
    pub checkpoint_state: MutationCheckpointState,
    /// Human-readable disclosure of the checkpoint state.
    pub checkpoint_disclosure: String,
    /// Shared downgrade states this component may surface.
    pub downgrade_vocab: Vec<GitHistoryDowngradeState>,
    /// Component fields the surface projects, in display order.
    pub fields_shown: Vec<String>,
    /// Source contract refs consumed by this component.
    pub source_contract_refs: Vec<String>,
}

impl PatchApplyReviewSheet {
    fn risky_view(&self) -> RiskySurfaceView<'_> {
        RiskySurfaceView {
            component: self.component,
            review_class: self.review_class,
            target_ref: &self.target_ref,
            target_worktree: &self.target_worktree,
            publish_consequence: &self.publish_consequence,
            approval_consequence: &self.approval_consequence,
            hosted_review_impact: self.hosted_review_impact,
            blocked_state: self.blocked_state,
            blocker_disclosure: &self.blocker_disclosure,
            rollback_action: &self.rollback_action,
            checkpoint_state: self.checkpoint_state,
            checkpoint_disclosure: &self.checkpoint_disclosure,
            downgrade_vocab_empty: self.downgrade_vocab.is_empty(),
            fields_shown_empty: self.fields_shown.is_empty(),
            source_contract_refs_empty: self.source_contract_refs.is_empty(),
        }
    }

    /// Whether the declared file count matches the enumerated affected paths.
    pub fn file_count_consistent(&self) -> bool {
        self.affected_file_count as usize == self.affected_paths.len()
    }
}

/// One conflict-checkpoint card exposing a captured conflict for recovery.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ConflictCheckpointCard {
    /// Stable row id.
    pub row_id: String,
    /// Canonical component; must be `conflict_checkpoint_card`.
    pub component: M5GitHistoryComponent,
    /// Human-readable checkpoint label.
    pub checkpoint_label: String,
    /// The mutation that produced this checkpoint (for example a cherry-pick).
    pub originating_operation: String,
    /// The exact ref the conflict was captured against.
    pub target_ref: String,
    /// The exact worktree the conflict is held in.
    pub target_worktree: String,
    /// The conflict sides preserved by the card (base/ours/theirs required).
    pub sides_present: Vec<ConflictSide>,
    /// Number of paths still unresolved.
    pub unresolved_count: u32,
    /// Total number of conflicted paths in the checkpoint.
    pub total_conflict_count: u32,
    /// How the checkpoint can be reopened or restored.
    pub reopen_state: CheckpointReopenState,
    /// Human-readable reopen/restore behavior disclosure.
    pub reopen_disclosure: String,
    /// Checkpoint availability for this card.
    pub checkpoint_state: MutationCheckpointState,
    /// Human-readable disclosure of the checkpoint state.
    pub checkpoint_disclosure: String,
    /// The mutation-review class; must be `display_only_no_mutation`.
    pub review_class: MutationReviewClass,
    /// Shared downgrade states this component may surface.
    pub downgrade_vocab: Vec<GitHistoryDowngradeState>,
    /// Component fields the surface projects, in display order.
    pub fields_shown: Vec<String>,
    /// Source contract refs consumed by this component.
    pub source_contract_refs: Vec<String>,
}

impl ConflictCheckpointCard {
    /// Whether base/ours/theirs context is all preserved by the card.
    pub fn preserves_required_context(&self) -> bool {
        ConflictSide::REQUIRED_CONTEXT
            .iter()
            .all(|side| self.sides_present.contains(side))
    }

    /// Whether the unresolved count stays within the total.
    pub fn counts_are_consistent(&self) -> bool {
        self.unresolved_count <= self.total_conflict_count
    }
}

/// One force-push review dialog shown before a ref rewrite is published.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ForcePushReviewDialog {
    /// Stable row id.
    pub row_id: String,
    /// Canonical component; must be `force_push_review_dialog`.
    pub component: M5GitHistoryComponent,
    /// The remote being pushed to (for example `origin`).
    pub target_remote: String,
    /// The exact ref being rewritten (for example `feature/review-lane`).
    pub target_ref: String,
    /// The exact worktree the push runs from.
    pub target_worktree: String,
    /// Short id of the local tip being published.
    pub local_tip_short_id: String,
    /// Short id of the remote tip being overwritten.
    pub remote_tip_short_id: String,
    /// How many remote-only commits the rewrite drops.
    pub overwrites_commit_count: u32,
    /// The lease safety guarding the remote tip.
    pub safety: ForcePushSafety,
    /// Human-readable disclosure of what the lease protects.
    pub lease_disclosure: String,
    /// The mutation-review class; must be `force_push_confirm`.
    pub review_class: MutationReviewClass,
    /// What the force-push publishes/rewrites on the remote.
    pub publish_consequence: String,
    /// How the change affects hosted review state.
    pub hosted_review_impact: HostedReviewImpact,
    /// The explicit approval consequence when hosted review is affected.
    pub approval_consequence: String,
    /// Whether the mutation is ready or blocked, and why.
    pub blocked_state: MutationBlockerState,
    /// Human-readable disclosure of any blocker.
    pub blocker_disclosure: String,
    /// The rollback action that restores the overwritten remote tip.
    pub rollback_action: String,
    /// The ref that recovers the pre-push remote tip (for example `origin/x@{1}`).
    pub recovery_ref: String,
    /// Checkpoint availability for this mutation.
    pub checkpoint_state: MutationCheckpointState,
    /// Human-readable disclosure of the checkpoint state.
    pub checkpoint_disclosure: String,
    /// Shared downgrade states this component may surface.
    pub downgrade_vocab: Vec<GitHistoryDowngradeState>,
    /// Component fields the surface projects, in display order.
    pub fields_shown: Vec<String>,
    /// Source contract refs consumed by this component.
    pub source_contract_refs: Vec<String>,
}

impl ForcePushReviewDialog {
    fn risky_view(&self) -> RiskySurfaceView<'_> {
        RiskySurfaceView {
            component: self.component,
            review_class: self.review_class,
            target_ref: &self.target_ref,
            target_worktree: &self.target_worktree,
            publish_consequence: &self.publish_consequence,
            approval_consequence: &self.approval_consequence,
            hosted_review_impact: self.hosted_review_impact,
            blocked_state: self.blocked_state,
            blocker_disclosure: &self.blocker_disclosure,
            rollback_action: &self.rollback_action,
            checkpoint_state: self.checkpoint_state,
            checkpoint_disclosure: &self.checkpoint_disclosure,
            downgrade_vocab_empty: self.downgrade_vocab.is_empty(),
            fields_shown_empty: self.fields_shown.is_empty(),
            source_contract_refs_empty: self.source_contract_refs.is_empty(),
        }
    }

    /// Whether both the local and remote tips are named explicitly.
    pub fn tips_explicit(&self) -> bool {
        !self.local_tip_short_id.trim().is_empty() && !self.remote_tip_short_id.trim().is_empty()
    }
}

/// Trust and provenance review block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GitMutationReviewTrustReview {
    /// No risky flow collapses distinct verbs into one ambiguous confirm.
    pub distinct_verbs_never_collapsed: bool,
    /// The exact target ref and worktree are always named for a mutation.
    pub target_ref_worktree_always_named: bool,
    /// Affected commits/files are always explicit before the verb runs.
    pub affected_scope_always_explicit: bool,
    /// The publish/rewrite consequence is always disclosed for a mutation.
    pub publish_consequence_always_disclosed: bool,
    /// The approval consequence is explicit whenever hosted review is affected.
    pub approval_consequence_explicit_when_hosted: bool,
    /// A rollback/recovery action stays reachable after a risky mutation.
    pub rollback_recovery_always_reachable: bool,
    /// Conflict base/ours/theirs/result context survives the mutation.
    pub conflict_context_survives_mutation: bool,
    /// Local-only recovery stays explicit even with provider review state.
    pub local_only_recovery_stays_explicit: bool,
    /// One component contract is reused with no hidden per-surface meaning.
    pub one_component_contract_no_hidden_meaning: bool,
    /// Downgrade narrows the claim rather than hiding the component.
    pub downgrade_narrows_instead_of_hides: bool,
    /// Stale or underqualified components automatically block promotion.
    pub stale_or_underqualified_blocks_promotion: bool,
}

impl GitMutationReviewTrustReview {
    /// Whether every invariant holds.
    pub const fn all_hold(&self) -> bool {
        self.distinct_verbs_never_collapsed
            && self.target_ref_worktree_always_named
            && self.affected_scope_always_explicit
            && self.publish_consequence_always_disclosed
            && self.approval_consequence_explicit_when_hosted
            && self.rollback_recovery_always_reachable
            && self.conflict_context_survives_mutation
            && self.local_only_recovery_stays_explicit
            && self.one_component_contract_no_hidden_meaning
            && self.downgrade_narrows_instead_of_hides
            && self.stale_or_underqualified_blocks_promotion
    }
}

/// Consumer projection block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GitMutationReviewConsumerProjection {
    /// Git-history surfaces reuse one component contract.
    pub git_history_reuses_one_contract: bool,
    /// Review surfaces reuse one component contract.
    pub review_reuses_one_contract: bool,
    /// Help/support surfaces reuse one component contract.
    pub help_support_reuses_one_contract: bool,
    /// Support/export surfaces reuse one component contract.
    pub support_export_reuses_one_contract: bool,
    /// Conflict/publish consequences stay explicit across every surface.
    pub consequences_explicit_across_surfaces: bool,
    /// CLI / headless shows component truth.
    pub cli_headless_shows_truth: bool,
    /// Provider overlay shows component truth without overwriting local truth.
    pub provider_overlay_shows_truth: bool,
    /// AI-context assembly shows component truth.
    pub ai_context_shows_truth: bool,
}

impl GitMutationReviewConsumerProjection {
    /// Whether every projection invariant holds.
    pub const fn all_hold(&self) -> bool {
        self.git_history_reuses_one_contract
            && self.review_reuses_one_contract
            && self.help_support_reuses_one_contract
            && self.support_export_reuses_one_contract
            && self.consequences_explicit_across_surfaces
            && self.cli_headless_shows_truth
            && self.provider_overlay_shows_truth
            && self.ai_context_shows_truth
    }
}

/// Proof freshness block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GitMutationReviewProofFreshness {
    /// Proof-freshness SLO in hours.
    pub proof_freshness_slo_hours: u32,
    /// RFC 3339 timestamp of the last proof refresh.
    pub last_proof_refresh: String,
    /// True when stale proof automatically narrows the lane.
    pub auto_narrow_on_stale: bool,
}

/// Constructor input for [`GitMutationReviewPacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GitMutationReviewPacketInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable surface label.
    pub surface_label: String,
    /// Cherry-pick / revert review sheets.
    pub cherry_pick_revert_sheets: Vec<CherryPickRevertReviewSheet>,
    /// Patch-apply review sheets.
    pub patch_apply_sheets: Vec<PatchApplyReviewSheet>,
    /// Conflict-checkpoint cards.
    pub conflict_checkpoint_cards: Vec<ConflictCheckpointCard>,
    /// Force-push review dialogs.
    pub force_push_dialogs: Vec<ForcePushReviewDialog>,
    /// Shared downgrade states that apply to this lane.
    pub downgrade_triggers: Vec<GitHistoryDowngradeState>,
    /// Consumer surfaces that must reuse this component contract.
    pub consumer_surfaces: Vec<ComponentConsumerSurface>,
    /// Trust review block.
    pub trust_review: GitMutationReviewTrustReview,
    /// Consumer projection block.
    pub consumer_projection: GitMutationReviewConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: GitMutationReviewProofFreshness,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

/// Export-safe mutation-review / recovery component packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GitMutationReviewPacket {
    /// Record kind; must equal [`GIT_MUTATION_REVIEW_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`GIT_MUTATION_REVIEW_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable surface label.
    pub surface_label: String,
    /// Cherry-pick / revert review sheets.
    pub cherry_pick_revert_sheets: Vec<CherryPickRevertReviewSheet>,
    /// Patch-apply review sheets.
    pub patch_apply_sheets: Vec<PatchApplyReviewSheet>,
    /// Conflict-checkpoint cards.
    pub conflict_checkpoint_cards: Vec<ConflictCheckpointCard>,
    /// Force-push review dialogs.
    pub force_push_dialogs: Vec<ForcePushReviewDialog>,
    /// Shared downgrade states that apply to this lane.
    pub downgrade_triggers: Vec<GitHistoryDowngradeState>,
    /// Consumer surfaces that must reuse this component contract.
    pub consumer_surfaces: Vec<ComponentConsumerSurface>,
    /// Trust review block.
    pub trust_review: GitMutationReviewTrustReview,
    /// Consumer projection block.
    pub consumer_projection: GitMutationReviewConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: GitMutationReviewProofFreshness,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

impl GitMutationReviewPacket {
    /// Builds a mutation-review / recovery component packet from stable-lane input.
    pub fn new(input: GitMutationReviewPacketInput) -> Self {
        Self {
            record_kind: GIT_MUTATION_REVIEW_RECORD_KIND.to_owned(),
            schema_version: GIT_MUTATION_REVIEW_SCHEMA_VERSION,
            packet_id: input.packet_id,
            surface_label: input.surface_label,
            cherry_pick_revert_sheets: input.cherry_pick_revert_sheets,
            patch_apply_sheets: input.patch_apply_sheets,
            conflict_checkpoint_cards: input.conflict_checkpoint_cards,
            force_push_dialogs: input.force_push_dialogs,
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

    /// Validates the mutation-review / recovery invariants.
    ///
    /// Returns every violation found rather than stopping at the first, so a
    /// regenerator or CI gate can report the full set at once.
    pub fn validate(&self) -> Vec<GitMutationReviewViolation> {
        let mut violations = Vec::new();

        if self.record_kind != GIT_MUTATION_REVIEW_RECORD_KIND {
            violations.push(GitMutationReviewViolation::WrongRecordKind);
        }
        if self.schema_version != GIT_MUTATION_REVIEW_SCHEMA_VERSION {
            violations.push(GitMutationReviewViolation::WrongSchemaVersion);
        }
        if self.packet_id.trim().is_empty()
            || self.surface_label.trim().is_empty()
            || self.redaction_class_token.trim().is_empty()
            || self.minted_at.trim().is_empty()
        {
            violations.push(GitMutationReviewViolation::MissingIdentity);
        }
        if self.downgrade_triggers.is_empty() {
            violations.push(GitMutationReviewViolation::DowngradeTriggersMissing);
        }
        if self.consumer_surfaces.is_empty() {
            violations.push(GitMutationReviewViolation::ConsumerSurfacesMissing);
        }

        validate_source_contracts(self, &mut violations);
        validate_cherry_pick_revert_sheets(self, &mut violations);
        validate_patch_apply_sheets(self, &mut violations);
        validate_conflict_checkpoint_cards(self, &mut violations);
        validate_force_push_dialogs(self, &mut violations);

        if !self.trust_review.all_hold() {
            violations.push(GitMutationReviewViolation::TrustReviewIncomplete);
        }
        if !self.consumer_projection.all_hold() {
            violations.push(GitMutationReviewViolation::ConsumerProjectionIncomplete);
        }
        if self.proof_freshness.proof_freshness_slo_hours == 0
            || self.proof_freshness.last_proof_refresh.trim().is_empty()
        {
            violations.push(GitMutationReviewViolation::ProofFreshnessIncomplete);
        }

        if json_contains_forbidden_boundary_material(
            &serde_json::to_value(self).expect("git mutation review packet serializes"),
        ) {
            violations.push(GitMutationReviewViolation::RawBoundaryMaterialInExport);
        }

        violations
    }

    /// Deterministic export-safe pretty JSON.
    ///
    /// # Panics
    ///
    /// Panics only if serializing this metadata-only packet fails.
    pub fn export_safe_json(&self) -> String {
        serde_json::to_string_pretty(self).expect("git mutation review packet serializes")
    }

    /// Deterministic Markdown summary for support, review, or release handoff.
    pub fn render_markdown_summary(&self) -> String {
        let mut out = String::new();
        out.push_str(
            "# Git Mutation Review Sheets, Conflict Checkpoints, and Force-Push Dialogs: Target and Recovery Truth\n\n",
        );
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- Surface: `{}`\n", self.surface_label));
        out.push_str(&format!(
            "- Cherry-pick/revert sheets: {}; patch-apply sheets: {}; conflict cards: {}; force-push dialogs: {}\n",
            self.cherry_pick_revert_sheets.len(),
            self.patch_apply_sheets.len(),
            self.conflict_checkpoint_cards.len(),
            self.force_push_dialogs.len(),
        ));
        out.push_str(&format!(
            "- Proof freshness SLO: {} hours (last refresh: {})\n",
            self.proof_freshness.proof_freshness_slo_hours, self.proof_freshness.last_proof_refresh
        ));

        out.push_str("\n## Cherry-pick / revert review sheets\n\n");
        for sheet in &self.cherry_pick_revert_sheets {
            out.push_str(&format!(
                "- `{}` `{}` \"{}\" onto `{}` — {} ({})\n",
                sheet.verb.as_str(),
                sheet.source_commit_short_id,
                sheet.source_commit_subject,
                sheet.target_ref,
                sheet.hosted_review_impact.as_str(),
                sheet.checkpoint_state.as_str(),
            ));
        }

        out.push_str("\n## Patch-apply review sheets\n\n");
        for sheet in &self.patch_apply_sheets {
            out.push_str(&format!(
                "- `{}` via `{}` onto `{}` — {} files / {} commits ({})\n",
                sheet.patch_source.as_str(),
                sheet.apply_mode.as_str(),
                sheet.target_ref,
                sheet.affected_file_count,
                sheet.commit_count,
                sheet.checkpoint_state.as_str(),
            ));
        }

        out.push_str("\n## Conflict-checkpoint cards\n\n");
        for card in &self.conflict_checkpoint_cards {
            out.push_str(&format!(
                "- **{}** on `{}` — {}/{} unresolved ({})\n",
                card.checkpoint_label,
                card.target_ref,
                card.unresolved_count,
                card.total_conflict_count,
                card.reopen_state.as_str(),
            ));
        }

        out.push_str("\n## Force-push review dialogs\n\n");
        for dialog in &self.force_push_dialogs {
            out.push_str(&format!(
                "- `{}` → `{}/{}` overwrites `{}` with `{}` ({} commits, {})\n",
                dialog.safety.as_str(),
                dialog.target_remote,
                dialog.target_ref,
                dialog.remote_tip_short_id,
                dialog.local_tip_short_id,
                dialog.overwrites_commit_count,
                dialog.hosted_review_impact.as_str(),
            ));
        }
        out
    }
}

/// Errors emitted when reading the checked-in mutation-review export.
#[derive(Debug)]
pub enum GitMutationReviewArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<GitMutationReviewViolation>),
}

impl fmt::Display for GitMutationReviewArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(
                    formatter,
                    "git mutation review export parse failed: {error}"
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
                    "git mutation review export failed validation: {tokens}"
                )
            }
        }
    }
}

impl Error for GitMutationReviewArtifactError {}

/// Validation failures emitted by [`GitMutationReviewPacket::validate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GitMutationReviewViolation {
    /// Packet record kind is wrong.
    WrongRecordKind,
    /// Packet schema version is wrong.
    WrongSchemaVersion,
    /// A required identity field is missing.
    MissingIdentity,
    /// Source contract refs are incomplete.
    MissingSourceContracts,
    /// A risky surface row is structurally incomplete.
    RiskySurfaceIncomplete,
    /// A risky surface collapses its distinct verb into a shared/ambiguous confirm.
    MutationVerbConfirmCollapsed,
    /// A risky surface does not name its exact target ref/worktree.
    TargetRefWorktreeMissing,
    /// A risky surface does not disclose its publish/rewrite consequence.
    PublishConsequenceMissing,
    /// A hosted-affecting surface does not disclose its approval consequence.
    ApprovalConsequenceMissing,
    /// A risky surface does not name a rollback action.
    RollbackActionMissing,
    /// A blocked surface does not disclose its blocker.
    MutationBlockerNotDisclosed,
    /// A risky surface does not disclose its checkpoint state.
    MutationCheckpointUndisclosed,
    /// A risky surface has no reachable recovery checkpoint.
    MutationRecoveryUnreachable,
    /// No cherry-pick / revert review sheets are present.
    CherryPickRevertSheetsMissing,
    /// A cherry-pick / revert sheet's component is wrong.
    WrongComponentForCherrySheet,
    /// A cherry-pick / revert sheet does not spell out its source commit identity.
    CherrySourceCommitIdentityMissing,
    /// The cherry-pick and revert verbs are not both represented.
    CherryVerbCoverageMissing,
    /// No patch-apply review sheets are present.
    PatchApplySheetsMissing,
    /// A patch-apply sheet's component is wrong.
    WrongComponentForPatchSheet,
    /// A patch-apply sheet's file count disagrees with its affected paths.
    PatchFileCountInconsistent,
    /// A patch-apply sheet does not enumerate the files it touches.
    PatchAffectedPathsMissing,
    /// No conflict-checkpoint cards are present.
    ConflictCheckpointCardsMissing,
    /// A conflict-checkpoint card row is incomplete.
    ConflictCardIncomplete,
    /// A conflict-checkpoint card's component is wrong.
    WrongComponentForConflictCard,
    /// A conflict-checkpoint card drops base/ours/theirs context.
    ConflictContextIncomplete,
    /// A conflict-checkpoint card's unresolved count exceeds its total.
    ConflictCountsInconsistent,
    /// A card with unresolved conflicts offers no reopen/restore path.
    UnresolvedConflictNotReopenable,
    /// A conflict-checkpoint card does not disclose its reopen/restore behavior.
    ConflictReopenBehaviorMissing,
    /// A conflict-checkpoint card claims a mutating review class.
    ConflictCardClaimsMutatingClass,
    /// No force-push review dialogs are present.
    ForcePushDialogsMissing,
    /// A force-push dialog's component is wrong.
    WrongComponentForForcePushDialog,
    /// A force-push dialog does not name the target remote and ref.
    ForcePushTargetMissing,
    /// A force-push dialog does not name the local and remote tips.
    ForcePushTipsMissing,
    /// A force-push dialog does not name a recovery ref for the overwritten tip.
    ForcePushRecoveryRefMissing,
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

impl GitMutationReviewViolation {
    /// Stable token used in tests and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WrongRecordKind => "wrong_record_kind",
            Self::WrongSchemaVersion => "wrong_schema_version",
            Self::MissingIdentity => "missing_identity",
            Self::MissingSourceContracts => "missing_source_contracts",
            Self::RiskySurfaceIncomplete => "risky_surface_incomplete",
            Self::MutationVerbConfirmCollapsed => "mutation_verb_confirm_collapsed",
            Self::TargetRefWorktreeMissing => "target_ref_worktree_missing",
            Self::PublishConsequenceMissing => "publish_consequence_missing",
            Self::ApprovalConsequenceMissing => "approval_consequence_missing",
            Self::RollbackActionMissing => "rollback_action_missing",
            Self::MutationBlockerNotDisclosed => "mutation_blocker_not_disclosed",
            Self::MutationCheckpointUndisclosed => "mutation_checkpoint_undisclosed",
            Self::MutationRecoveryUnreachable => "mutation_recovery_unreachable",
            Self::CherryPickRevertSheetsMissing => "cherry_pick_revert_sheets_missing",
            Self::WrongComponentForCherrySheet => "wrong_component_for_cherry_sheet",
            Self::CherrySourceCommitIdentityMissing => "cherry_source_commit_identity_missing",
            Self::CherryVerbCoverageMissing => "cherry_verb_coverage_missing",
            Self::PatchApplySheetsMissing => "patch_apply_sheets_missing",
            Self::WrongComponentForPatchSheet => "wrong_component_for_patch_sheet",
            Self::PatchFileCountInconsistent => "patch_file_count_inconsistent",
            Self::PatchAffectedPathsMissing => "patch_affected_paths_missing",
            Self::ConflictCheckpointCardsMissing => "conflict_checkpoint_cards_missing",
            Self::ConflictCardIncomplete => "conflict_card_incomplete",
            Self::WrongComponentForConflictCard => "wrong_component_for_conflict_card",
            Self::ConflictContextIncomplete => "conflict_context_incomplete",
            Self::ConflictCountsInconsistent => "conflict_counts_inconsistent",
            Self::UnresolvedConflictNotReopenable => "unresolved_conflict_not_reopenable",
            Self::ConflictReopenBehaviorMissing => "conflict_reopen_behavior_missing",
            Self::ConflictCardClaimsMutatingClass => "conflict_card_claims_mutating_class",
            Self::ForcePushDialogsMissing => "force_push_dialogs_missing",
            Self::WrongComponentForForcePushDialog => "wrong_component_for_force_push_dialog",
            Self::ForcePushTargetMissing => "force_push_target_missing",
            Self::ForcePushTipsMissing => "force_push_tips_missing",
            Self::ForcePushRecoveryRefMissing => "force_push_recovery_ref_missing",
            Self::DowngradeTriggersMissing => "downgrade_triggers_missing",
            Self::ConsumerSurfacesMissing => "consumer_surfaces_missing",
            Self::TrustReviewIncomplete => "trust_review_incomplete",
            Self::ConsumerProjectionIncomplete => "consumer_projection_incomplete",
            Self::ProofFreshnessIncomplete => "proof_freshness_incomplete",
            Self::RawBoundaryMaterialInExport => "raw_boundary_material_in_export",
        }
    }
}

/// Reads and validates the checked-in stable mutation-review export.
///
/// # Errors
///
/// Returns [`GitMutationReviewArtifactError`] when the checked-in export fails to
/// parse or violates the contract.
pub fn current_git_mutation_review_export(
) -> Result<GitMutationReviewPacket, GitMutationReviewArtifactError> {
    let packet: GitMutationReviewPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/release/m5-git-mutation-review-recovery-components-proof/support_export.json"
    )))
    .map_err(GitMutationReviewArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(GitMutationReviewArtifactError::Validation(violations))
    }
}

fn validate_source_contracts(
    packet: &GitMutationReviewPacket,
    violations: &mut Vec<GitMutationReviewViolation>,
) {
    let refs: BTreeSet<&str> = packet
        .source_contract_refs
        .iter()
        .map(String::as_str)
        .collect();
    for required in [
        GIT_MUTATION_REVIEW_SCHEMA_REF,
        GIT_MUTATION_REVIEW_DOC_REF,
        GIT_MUTATION_REVIEW_COMPONENT_MATRIX_CONTRACT_REF,
        GIT_MUTATION_REVIEW_HISTORY_SURGERY_CONTRACT_REF,
        GIT_MUTATION_REVIEW_CONFLICT_SESSION_CONTRACT_REF,
        GIT_MUTATION_REVIEW_REF_UPDATE_CONTRACT_REF,
    ] {
        if !refs.contains(required) {
            violations.push(GitMutationReviewViolation::MissingSourceContracts);
            return;
        }
    }
}

fn validate_cherry_pick_revert_sheets(
    packet: &GitMutationReviewPacket,
    violations: &mut Vec<GitMutationReviewViolation>,
) {
    if packet.cherry_pick_revert_sheets.is_empty() {
        violations.push(GitMutationReviewViolation::CherryPickRevertSheetsMissing);
        return;
    }

    let mut verbs_seen: BTreeSet<CherryPickRevertVerb> = BTreeSet::new();

    for sheet in &packet.cherry_pick_revert_sheets {
        verbs_seen.insert(sheet.verb);

        if sheet.row_id.trim().is_empty() {
            violations.push(GitMutationReviewViolation::RiskySurfaceIncomplete);
        }
        if sheet.component != M5GitHistoryComponent::CherryPickRevertReviewSheet {
            violations.push(GitMutationReviewViolation::WrongComponentForCherrySheet);
        }
        if !sheet.source_identity_explicit() {
            violations.push(GitMutationReviewViolation::CherrySourceCommitIdentityMissing);
        }
        validate_risky_surface(&sheet.risky_view(), violations);
    }

    if !(verbs_seen.contains(&CherryPickRevertVerb::CherryPick)
        && verbs_seen.contains(&CherryPickRevertVerb::Revert))
    {
        violations.push(GitMutationReviewViolation::CherryVerbCoverageMissing);
    }
}

fn validate_patch_apply_sheets(
    packet: &GitMutationReviewPacket,
    violations: &mut Vec<GitMutationReviewViolation>,
) {
    if packet.patch_apply_sheets.is_empty() {
        violations.push(GitMutationReviewViolation::PatchApplySheetsMissing);
        return;
    }

    for sheet in &packet.patch_apply_sheets {
        if sheet.row_id.trim().is_empty() {
            violations.push(GitMutationReviewViolation::RiskySurfaceIncomplete);
        }
        if sheet.component != M5GitHistoryComponent::PatchApplyReviewSheet {
            violations.push(GitMutationReviewViolation::WrongComponentForPatchSheet);
        }
        if sheet.affected_paths.is_empty() {
            violations.push(GitMutationReviewViolation::PatchAffectedPathsMissing);
        }
        if !sheet.file_count_consistent() {
            violations.push(GitMutationReviewViolation::PatchFileCountInconsistent);
        }
        validate_risky_surface(&sheet.risky_view(), violations);
    }
}

fn validate_conflict_checkpoint_cards(
    packet: &GitMutationReviewPacket,
    violations: &mut Vec<GitMutationReviewViolation>,
) {
    if packet.conflict_checkpoint_cards.is_empty() {
        violations.push(GitMutationReviewViolation::ConflictCheckpointCardsMissing);
        return;
    }

    for card in &packet.conflict_checkpoint_cards {
        if card.row_id.trim().is_empty()
            || card.checkpoint_label.trim().is_empty()
            || card.originating_operation.trim().is_empty()
            || card.target_ref.trim().is_empty()
            || card.target_worktree.trim().is_empty()
            || card.checkpoint_disclosure.trim().is_empty()
            || card.downgrade_vocab.is_empty()
            || card.fields_shown.is_empty()
            || card.source_contract_refs.is_empty()
        {
            violations.push(GitMutationReviewViolation::ConflictCardIncomplete);
        }
        if card.component != M5GitHistoryComponent::ConflictCheckpointCard {
            violations.push(GitMutationReviewViolation::WrongComponentForConflictCard);
        }
        if !card.preserves_required_context() {
            violations.push(GitMutationReviewViolation::ConflictContextIncomplete);
        }
        if !card.counts_are_consistent() {
            violations.push(GitMutationReviewViolation::ConflictCountsInconsistent);
        }
        // A card with unresolved conflicts must keep a way back in; a card claiming
        // it is resolved-and-applied must have nothing left unresolved.
        if card.unresolved_count > 0 && !card.reopen_state.offers_reopen_path() {
            violations.push(GitMutationReviewViolation::UnresolvedConflictNotReopenable);
        }
        if card.reopen_disclosure.trim().is_empty() {
            violations.push(GitMutationReviewViolation::ConflictReopenBehaviorMissing);
        }
        if card.review_class != MutationReviewClass::DisplayOnlyNoMutation {
            violations.push(GitMutationReviewViolation::ConflictCardClaimsMutatingClass);
        }
    }
}

fn validate_force_push_dialogs(
    packet: &GitMutationReviewPacket,
    violations: &mut Vec<GitMutationReviewViolation>,
) {
    if packet.force_push_dialogs.is_empty() {
        violations.push(GitMutationReviewViolation::ForcePushDialogsMissing);
        return;
    }

    for dialog in &packet.force_push_dialogs {
        if dialog.row_id.trim().is_empty() {
            violations.push(GitMutationReviewViolation::RiskySurfaceIncomplete);
        }
        if dialog.component != M5GitHistoryComponent::ForcePushReviewDialog {
            violations.push(GitMutationReviewViolation::WrongComponentForForcePushDialog);
        }
        if dialog.target_remote.trim().is_empty() || dialog.target_ref.trim().is_empty() {
            violations.push(GitMutationReviewViolation::ForcePushTargetMissing);
        }
        if !dialog.tips_explicit() {
            violations.push(GitMutationReviewViolation::ForcePushTipsMissing);
        }
        if dialog.recovery_ref.trim().is_empty() {
            violations.push(GitMutationReviewViolation::ForcePushRecoveryRefMissing);
        }
        validate_risky_surface(&dialog.risky_view(), violations);
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
