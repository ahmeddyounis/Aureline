//! Per-verb history-surgery review sheets with explicit target truth.
//!
//! Risky history mutations used to be command-only escape hatches: a user who
//! wanted to rebase, cherry-pick, revert, reset, apply a patch, or force-push had
//! to drop to the shell, and the product could not say *what* would be rewritten,
//! *where*, under *which* policy, or how it could be undone. This module replaces
//! that with one durable, serde-serializable [`HistorySurgeryReviewSheet`] per
//! risky verb. Each sheet names the exact repository or worktree target and the
//! exact target ref it would mutate, surfaces the pre-execution gate states a user
//! must see before Continue / Skip / Publish, keeps the raw rebase-todo or patch
//! text (and the structured cards derived from it) inspectable, and keeps a
//! reflog or checkpoint recovery path visible.
//!
//! The six verbs stay **distinct**: a [`HistorySurgeryVerb`] is never collapsed
//! into a generic "rewrite history" sheet, because users need the exact verb,
//! target, policy, and recovery semantics. A sheet is not a badge — it carries a
//! [`ReviewDecision`] that is *derived* from the gate states by
//! [`HistorySurgeryReviewSheet::derive_decision`], so the product can always
//! explain why a mutation was **allowed**, **blocked**, or **downgraded**, and a
//! stored decision can be re-derived and verified against its gates.
//!
//! Three guarantees are encoded in the derivation rather than left to prose:
//!
//! * Every risky mutation names an exact, unambiguous target ref and a
//!   repository-or-worktree target kind; a sheet with no target is invalid.
//! * A risky mutation is never *allowed* without a reachable recovery path
//!   (an explicit checkpoint, or an acknowledged reflog-only fallback), and the
//!   recovery path is visible before execution.
//! * A provider outage never blocks local truth: the provider-overlay gate can
//!   only **downgrade** a sheet to local-only, never block it, so local preview,
//!   abort, and restore stay available offline.
//!
//! The boundary schema is
//! [`schemas/git/history-surgery-review.schema.json`](../../../../schemas/git/history-surgery-review.schema.json).
//! The protected fixture corpus is
//! [`fixtures/git/m5/rebase-cherry-pick-reset/`](../../../../fixtures/git/m5/rebase-cherry-pick-reset/).
//! The checked-in canonical packet is
//! [`artifacts/git/m5/history_surgery_review/history_surgery_review_sheets.json`](../../../../artifacts/git/m5/history_surgery_review/history_surgery_review_sheets.json).

#[cfg(test)]
mod tests;

use std::collections::HashSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

use crate::finalize_sequence_edit_conflict_session_stash_entry_and_ref_update_truth::{
    RISKY_VCS_APPROVAL_STATES, RISKY_VCS_DIVERGENCE_CLASSES, RISKY_VCS_PROTECTED_BRANCH_POSTURES,
};

/// Schema version for [`HistorySurgeryReviewPacket`].
pub const HISTORY_SURGERY_REVIEW_SCHEMA_VERSION: u32 = 1;

/// Stable record-kind tag carried by [`HistorySurgeryReviewPacket`].
pub const HISTORY_SURGERY_REVIEW_PACKET_RECORD_KIND: &str = "git_history_surgery_review_packet";

/// Stable record-kind tag carried by [`HistorySurgeryReviewSheet`].
pub const HISTORY_SURGERY_REVIEW_SHEET_RECORD_KIND: &str = "git_history_surgery_review_sheet";

/// Stable record-kind tag carried by [`HistorySurgeryReviewSupportExport`].
pub const HISTORY_SURGERY_REVIEW_SUPPORT_EXPORT_RECORD_KIND: &str =
    "git_history_surgery_review_support_export";

/// Repo-relative path of the boundary schema.
pub const HISTORY_SURGERY_REVIEW_SCHEMA_REF: &str =
    "schemas/git/history-surgery-review.schema.json";

/// Repo-relative path of the protected fixture corpus directory.
pub const HISTORY_SURGERY_REVIEW_FIXTURE_DIR: &str = "fixtures/git/m5/rebase-cherry-pick-reset";

/// Repo-relative path of the checked-in canonical sheets packet.
pub const HISTORY_SURGERY_REVIEW_ARTIFACT_REF: &str =
    "artifacts/git/m5/history_surgery_review/history_surgery_review_sheets.json";

/// Identity fields a support export must retain after redaction so a sheet can be
/// reconstructed without leaking raw boundary material.
pub const HISTORY_SURGERY_REVIEW_REQUIRED_RECONSTRUCTION_FIELDS: [&str; 8] = [
    "verb",
    "repo_ref",
    "worktree_ref",
    "target_kind",
    "primary_target_ref",
    "decision_outcome",
    "decision_reason",
    "recovery_visible",
];

/// Closed set of repository-or-worktree target kinds.
///
/// Every sheet names exactly one, so a risky mutation can never be ambiguous
/// about whether it targets the repository root or a specific linked worktree.
pub const HISTORY_SURGERY_TARGET_KINDS: &[&str] = &["repository_root", "linked_worktree"];

/// Closed set of merge-queue states a sheet may surface before execution.
pub const HISTORY_SURGERY_MERGE_QUEUE_STATES: &[&str] = &[
    "not_enqueued",
    "queued_position_known",
    "queued_invalidated_by_rewrite",
    "queue_blocks_history_surgery",
];

/// Closed set of dirty-worktree states a sheet may surface before execution.
pub const HISTORY_SURGERY_DIRTY_WORKTREE_STATES: &[&str] = &[
    "clean",
    "untracked_present",
    "dirty_tracked_changes",
    "dirty_blocks_operation",
];

/// Closed set of conflict-source states a sheet may surface before execution.
pub const HISTORY_SURGERY_CONFLICT_SOURCE_STATES: &[&str] = &[
    "no_conflicts",
    "conflicts_resolved_ready_to_continue",
    "conflicts_present_blocks_continue",
    "conflict_source_unknown_requires_refresh",
];

/// Closed set of provider-overlay states layered over local Git truth.
pub const HISTORY_SURGERY_PROVIDER_OVERLAY_STATES: &[&str] = &[
    "overlay_fresh",
    "overlay_stale",
    "overlay_unavailable_local_only",
];

/// Closed set of reset modes a [`HistorySurgeryVerb::Reset`] sheet may name.
pub const HISTORY_SURGERY_RESET_MODES: &[&str] = &["soft", "mixed", "hard", "keep", "merge"];

/// Closed set of local-first actions every sheet must keep available offline.
///
/// These never depend on a provider; they are what keeps local preview, abort,
/// and restore truth reachable even during a provider outage.
pub const HISTORY_SURGERY_LOCAL_ACTIONS: &[&str] = &[
    "preview",
    "continue",
    "skip",
    "abort",
    "restore_checkpoint",
    "inspect_raw_source",
];

/// Local-first actions a sheet must always offer regardless of provider state.
pub const HISTORY_SURGERY_REQUIRED_LOCAL_ACTIONS: [&str; 2] = ["preview", "abort"];

/// Closed set of decision reason tokens carried by a [`ReviewDecision`].
pub const HISTORY_SURGERY_DECISION_REASONS: &[&str] = &[
    "allowed_with_recovery",
    "blocked_unresolved_conflict",
    "blocked_protected_branch",
    "blocked_merge_queue",
    "blocked_stale_review_invalidated",
    "blocked_dirty_worktree",
    "blocked_no_recovery_path",
    "blocked_source_text_unavailable",
    "downgraded_checkpoint_to_reflog_only",
    "downgraded_raw_inspection_only",
    "downgraded_merge_queue_drop",
    "downgraded_dirty_autostash",
    "downgraded_provider_unavailable_local_only",
    "downgraded_provider_overlay_stale",
];

/// A risky history-surgery verb that carries its own review sheet.
///
/// The verbs stay distinct because each has different target, policy, and
/// recovery semantics; they are never normalized into one generic sheet.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum HistorySurgeryVerb {
    /// Replay commits onto a new base (interactive or not).
    Rebase,
    /// Apply one or more commits onto the current branch.
    CherryPick,
    /// Create inverse commits that undo one or more commits.
    Revert,
    /// Move HEAD (and optionally index/worktree) to another ref.
    Reset,
    /// Apply a patch / diff body to the worktree or index.
    PatchApply,
    /// Rewrite a published ref with a leased force-push.
    ForcePush,
}

impl HistorySurgeryVerb {
    /// Every verb, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::Rebase,
        Self::CherryPick,
        Self::Revert,
        Self::Reset,
        Self::PatchApply,
        Self::ForcePush,
    ];

    /// Stable token used by fixtures, schemas, and support packets.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Rebase => "rebase",
            Self::CherryPick => "cherry_pick",
            Self::Revert => "revert",
            Self::Reset => "reset",
            Self::PatchApply => "patch_apply",
            Self::ForcePush => "force_push",
        }
    }

    /// Whether the verb must preserve an inspectable raw todo/patch body.
    ///
    /// A rebase preserves its interactive todo and a patch-apply preserves its
    /// patch text; both must stay inspectable even when structured parsing fails.
    pub const fn requires_source_text(self) -> bool {
        matches!(self, Self::Rebase | Self::PatchApply)
    }

    /// Whether the verb names at least one source commit ref it operates on.
    pub const fn requires_source_commits(self) -> bool {
        matches!(self, Self::CherryPick | Self::Revert)
    }

    /// Whether the verb requires an explicit reset mode.
    pub const fn requires_reset_mode(self) -> bool {
        matches!(self, Self::Reset)
    }

    /// Whether the verb rewrites a published ref over the network.
    ///
    /// Only the force-push verb reaches the network; the rest are local rewrites
    /// whose provider overlay is advisory.
    pub const fn is_publish_class(self) -> bool {
        matches!(self, Self::ForcePush)
    }
}

/// Verdict a single gate contributes toward the overall [`ReviewDecision`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum GateVerdict {
    /// The gate places no constraint on the mutation.
    Clear,
    /// The gate narrows the mutation but does not block it.
    Downgrade,
    /// The gate blocks the mutation until it is resolved.
    Block,
}

/// Overall outcome of a review sheet's gate evaluation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ReviewDecisionOutcome {
    /// The mutation may run; a recovery path is visible.
    Allowed,
    /// At least one gate narrows the mutation (raw-only inspection, reflog-only
    /// recovery, provider unavailable, autostash, or merge-queue drop).
    Downgraded,
    /// At least one gate blocks the mutation until it is resolved.
    Blocked,
}

impl ReviewDecisionOutcome {
    /// Stable token used by fixtures, schemas, and support packets.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Allowed => "allowed",
            Self::Downgraded => "downgraded",
            Self::Blocked => "blocked",
        }
    }

    /// Whether the outcome permits the mutation to run.
    pub const fn permits_execution(self) -> bool {
        matches!(self, Self::Allowed)
    }
}

/// The derived, explainable decision for a single review sheet.
///
/// Every field is a deterministic function of the sheet's verb, target, and gate
/// states, so a stored decision can be re-derived and verified — the product can
/// always say *why* a risky mutation was allowed, blocked, or downgraded.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReviewDecision {
    /// Overall outcome.
    pub outcome: ReviewDecisionOutcome,
    /// Primary reason token from [`HISTORY_SURGERY_DECISION_REASONS`].
    pub primary_reason: String,
    /// Gate tokens that contributed the decisive severity, in priority order.
    pub contributing_gates: Vec<String>,
    /// Whether a reachable recovery path is visible before execution.
    pub recovery_visible: bool,
    /// Whether local preview/abort/restore truth is available without a provider.
    pub local_truth_available_offline: bool,
    /// Redaction-safe, deterministic one-line explanation.
    pub explanation: String,
}

/// A durable per-verb history-surgery review sheet with explicit target truth.
///
/// One sheet exists per risky verb a user can run on a target. The structured
/// gate fields are the canonical substrate; [`HistorySurgeryReviewSheet::new`]
/// derives the [`ReviewDecision`] from them, so the sheet is never a badge.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HistorySurgeryReviewSheet {
    /// Record-kind tag; must equal [`HISTORY_SURGERY_REVIEW_SHEET_RECORD_KIND`].
    pub record_kind: String,
    /// Stable sheet identity (referenced by rows and support export).
    pub sheet_id: String,
    /// The risky verb this sheet reviews.
    pub verb: HistorySurgeryVerb,
    /// Redaction-safe repository ref the sheet belongs to.
    pub repo_ref: String,
    /// Redaction-safe worktree ref the mutation runs in.
    pub worktree_ref: String,
    /// Whether the mutation targets the repository root or a linked worktree.
    pub target_kind: String,
    /// Exact, unambiguous target ref the mutation would move or rewrite.
    pub primary_target_ref: String,
    /// Secondary refs (rebase onto/base, cherry-pick/revert source commits,
    /// force-push remote ref). Their meaning is verb-specific.
    pub secondary_refs: Vec<String>,
    /// Reset mode, for [`HistorySurgeryVerb::Reset`] sheets only.
    pub reset_mode: Option<String>,
    /// Force-with-lease expected old value, for [`HistorySurgeryVerb::ForcePush`].
    pub force_lease_ref: Option<String>,
    /// Divergence class, for [`HistorySurgeryVerb::ForcePush`] sheets only.
    pub divergence_class: Option<String>,
    /// Protected-branch posture gate (from [`RISKY_VCS_PROTECTED_BRANCH_POSTURES`]).
    pub protected_branch_posture: String,
    /// Stale-review / approval gate (from [`RISKY_VCS_APPROVAL_STATES`]).
    pub stale_review_state: String,
    /// Merge-queue gate (from [`HISTORY_SURGERY_MERGE_QUEUE_STATES`]).
    pub merge_queue_state: String,
    /// Dirty-worktree gate (from [`HISTORY_SURGERY_DIRTY_WORKTREE_STATES`]).
    pub dirty_worktree_state: String,
    /// Conflict-source gate (from [`HISTORY_SURGERY_CONFLICT_SOURCE_STATES`]).
    pub conflict_source_state: String,
    /// Provider-overlay gate (from [`HISTORY_SURGERY_PROVIDER_OVERLAY_STATES`]).
    pub provider_overlay_state: String,
    /// Ref to the exact raw todo/patch text, when the verb preserves it.
    pub raw_source_text_ref: Option<String>,
    /// Ref to the structured cards derived from the same source text.
    pub structured_cards_ref: Option<String>,
    /// Checkpoint lineage refs protecting this mutation's recovery.
    pub checkpoint_lineage_refs: Vec<String>,
    /// True when only a reflog-only fallback is available (no full checkpoint).
    pub reflog_only_fallback: bool,
    /// Local-first actions kept available regardless of provider state.
    pub local_actions: Vec<String>,
    /// Derived decision; equals [`HistorySurgeryReviewSheet::derive_decision`].
    pub decision: ReviewDecision,
    /// Created timestamp (RFC 3339).
    pub created_at: String,
    /// Updated timestamp (RFC 3339).
    pub updated_at: String,
    /// Redaction-safe summary label.
    pub summary_label: String,
}

/// Constructor input for [`HistorySurgeryReviewSheet::new`] (decision excluded).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HistorySurgeryReviewSheetInput {
    /// Stable sheet identity.
    pub sheet_id: String,
    /// The risky verb this sheet reviews.
    pub verb: HistorySurgeryVerb,
    /// Redaction-safe repository ref.
    pub repo_ref: String,
    /// Redaction-safe worktree ref.
    pub worktree_ref: String,
    /// Target kind token.
    pub target_kind: String,
    /// Exact target ref.
    pub primary_target_ref: String,
    /// Verb-specific secondary refs.
    pub secondary_refs: Vec<String>,
    /// Reset mode (reset only).
    pub reset_mode: Option<String>,
    /// Force-with-lease expected old value (force-push only).
    pub force_lease_ref: Option<String>,
    /// Divergence class (force-push only).
    pub divergence_class: Option<String>,
    /// Protected-branch posture gate.
    pub protected_branch_posture: String,
    /// Stale-review / approval gate.
    pub stale_review_state: String,
    /// Merge-queue gate.
    pub merge_queue_state: String,
    /// Dirty-worktree gate.
    pub dirty_worktree_state: String,
    /// Conflict-source gate.
    pub conflict_source_state: String,
    /// Provider-overlay gate.
    pub provider_overlay_state: String,
    /// Raw todo/patch text ref.
    pub raw_source_text_ref: Option<String>,
    /// Structured cards ref.
    pub structured_cards_ref: Option<String>,
    /// Checkpoint lineage refs.
    pub checkpoint_lineage_refs: Vec<String>,
    /// True when only a reflog-only fallback is available.
    pub reflog_only_fallback: bool,
    /// Local-first actions kept available offline.
    pub local_actions: Vec<String>,
    /// Created timestamp (RFC 3339).
    pub created_at: String,
    /// Updated timestamp (RFC 3339).
    pub updated_at: String,
    /// Redaction-safe summary label.
    pub summary_label: String,
}

/// One named gate and the severity/reason it contributes.
struct GateOutcome {
    /// Stable gate name for `contributing_gates`.
    name: &'static str,
    /// The gate's verdict.
    verdict: GateVerdict,
    /// Reason token if this gate is the decisive one (severity-specific).
    reason: &'static str,
}

impl HistorySurgeryReviewSheet {
    /// Builds a sheet from input and derives its decision from the gate states.
    pub fn new(input: HistorySurgeryReviewSheetInput) -> Self {
        let mut sheet = Self {
            record_kind: HISTORY_SURGERY_REVIEW_SHEET_RECORD_KIND.to_owned(),
            sheet_id: input.sheet_id,
            verb: input.verb,
            repo_ref: input.repo_ref,
            worktree_ref: input.worktree_ref,
            target_kind: input.target_kind,
            primary_target_ref: input.primary_target_ref,
            secondary_refs: input.secondary_refs,
            reset_mode: input.reset_mode,
            force_lease_ref: input.force_lease_ref,
            divergence_class: input.divergence_class,
            protected_branch_posture: input.protected_branch_posture,
            stale_review_state: input.stale_review_state,
            merge_queue_state: input.merge_queue_state,
            dirty_worktree_state: input.dirty_worktree_state,
            conflict_source_state: input.conflict_source_state,
            provider_overlay_state: input.provider_overlay_state,
            raw_source_text_ref: input.raw_source_text_ref,
            structured_cards_ref: input.structured_cards_ref,
            checkpoint_lineage_refs: input.checkpoint_lineage_refs,
            reflog_only_fallback: input.reflog_only_fallback,
            local_actions: input.local_actions,
            // Placeholder; replaced by the derived decision immediately below.
            decision: ReviewDecision {
                outcome: ReviewDecisionOutcome::Blocked,
                primary_reason: String::new(),
                contributing_gates: Vec::new(),
                recovery_visible: false,
                local_truth_available_offline: false,
                explanation: String::new(),
            },
            created_at: input.created_at,
            updated_at: input.updated_at,
            summary_label: input.summary_label,
        };
        sheet.decision = sheet.derive_decision();
        sheet
    }

    /// Whether the sheet retains exact repository and worktree identity.
    pub fn identity_preserved(&self) -> bool {
        !self.repo_ref.trim().is_empty() && !self.worktree_ref.trim().is_empty()
    }

    /// Whether a reachable recovery path is visible for this mutation.
    pub fn recovery_visible(&self) -> bool {
        !self.checkpoint_lineage_refs.is_empty() || self.reflog_only_fallback
    }

    /// Gate evaluations in fixed priority order, used to derive the decision.
    ///
    /// The order is the precedence used when several gates fire at the decisive
    /// severity: the first gate in this list wins the primary reason.
    fn gate_outcomes(&self) -> Vec<GateOutcome> {
        vec![
            GateOutcome {
                name: "conflict_source",
                verdict: conflict_source_verdict(&self.conflict_source_state),
                reason: "blocked_unresolved_conflict",
            },
            GateOutcome {
                name: "protected_branch",
                verdict: protected_branch_verdict(&self.protected_branch_posture),
                reason: "blocked_protected_branch",
            },
            GateOutcome {
                name: "merge_queue",
                verdict: merge_queue_verdict(&self.merge_queue_state),
                reason: match merge_queue_verdict(&self.merge_queue_state) {
                    GateVerdict::Downgrade => "downgraded_merge_queue_drop",
                    _ => "blocked_merge_queue",
                },
            },
            GateOutcome {
                name: "stale_review",
                verdict: stale_review_verdict(&self.stale_review_state),
                reason: "blocked_stale_review_invalidated",
            },
            GateOutcome {
                name: "recovery",
                verdict: self.recovery_verdict(),
                reason: match self.recovery_verdict() {
                    GateVerdict::Downgrade => "downgraded_checkpoint_to_reflog_only",
                    _ => "blocked_no_recovery_path",
                },
            },
            GateOutcome {
                name: "source_inspection",
                verdict: self.inspection_verdict(),
                reason: match self.inspection_verdict() {
                    GateVerdict::Downgrade => "downgraded_raw_inspection_only",
                    _ => "blocked_source_text_unavailable",
                },
            },
            GateOutcome {
                name: "dirty_worktree",
                verdict: dirty_worktree_verdict(&self.dirty_worktree_state),
                reason: match dirty_worktree_verdict(&self.dirty_worktree_state) {
                    GateVerdict::Downgrade => "downgraded_dirty_autostash",
                    _ => "blocked_dirty_worktree",
                },
            },
            GateOutcome {
                name: "provider_overlay",
                verdict: provider_overlay_verdict(&self.provider_overlay_state),
                reason: match self.provider_overlay_state.as_str() {
                    "overlay_unavailable_local_only" => {
                        "downgraded_provider_unavailable_local_only"
                    }
                    _ => "downgraded_provider_overlay_stale",
                },
            },
        ]
    }

    /// Recovery gate: a mutation with no recovery is blocked; a reflog-only
    /// fallback is a downgrade; an explicit checkpoint is clear.
    fn recovery_verdict(&self) -> GateVerdict {
        if !self.checkpoint_lineage_refs.is_empty() {
            GateVerdict::Clear
        } else if self.reflog_only_fallback {
            GateVerdict::Downgrade
        } else {
            GateVerdict::Block
        }
    }

    /// Source-inspection gate: a verb that preserves raw text must keep it
    /// inspectable; missing raw text blocks, raw-without-structured downgrades.
    fn inspection_verdict(&self) -> GateVerdict {
        if !self.verb.requires_source_text() {
            return GateVerdict::Clear;
        }
        let raw_ok = self
            .raw_source_text_ref
            .as_deref()
            .is_some_and(|value| !value.trim().is_empty());
        let cards_ok = self
            .structured_cards_ref
            .as_deref()
            .is_some_and(|value| !value.trim().is_empty());
        match (raw_ok, cards_ok) {
            (false, _) => GateVerdict::Block,
            (true, false) => GateVerdict::Downgrade,
            (true, true) => GateVerdict::Clear,
        }
    }

    /// Derives the explainable decision from the gate states.
    ///
    /// The outcome is the worst severity across all gates; the primary reason is
    /// the first gate (in priority order) at that severity. Provider outages only
    /// ever downgrade, so they never block local truth.
    pub fn derive_decision(&self) -> ReviewDecision {
        let gates = self.gate_outcomes();
        let recovery_visible = self.recovery_visible();
        // Local preview/abort/restore truth is reachable whenever a recovery path
        // exists, independent of provider availability.
        let local_truth_available_offline = recovery_visible;

        let blocking: Vec<&GateOutcome> = gates
            .iter()
            .filter(|gate| gate.verdict == GateVerdict::Block)
            .collect();
        let downgrading: Vec<&GateOutcome> = gates
            .iter()
            .filter(|gate| gate.verdict == GateVerdict::Downgrade)
            .collect();

        let (outcome, decisive): (ReviewDecisionOutcome, &[&GateOutcome]) = if !blocking.is_empty()
        {
            (ReviewDecisionOutcome::Blocked, blocking.as_slice())
        } else if !downgrading.is_empty() {
            (ReviewDecisionOutcome::Downgraded, downgrading.as_slice())
        } else {
            (ReviewDecisionOutcome::Allowed, &[])
        };

        let primary_reason = decisive
            .first()
            .map_or("allowed_with_recovery", |gate| gate.reason)
            .to_owned();
        let contributing_gates: Vec<String> =
            decisive.iter().map(|gate| gate.name.to_owned()).collect();

        let explanation = format!(
            "{} on {} `{}`: {}",
            self.verb.as_str(),
            self.target_kind,
            self.primary_target_ref,
            primary_reason
        );

        ReviewDecision {
            outcome,
            primary_reason,
            contributing_gates,
            recovery_visible,
            local_truth_available_offline,
            explanation,
        }
    }
}

fn protected_branch_verdict(token: &str) -> GateVerdict {
    match token {
        "no_protected_refs" => GateVerdict::Clear,
        "protected_branch_readonly" | "protected_branch_blocked" | "policy_lock_active" => {
            GateVerdict::Block
        }
        _ => GateVerdict::Block,
    }
}

fn stale_review_verdict(token: &str) -> GateVerdict {
    match token {
        "approval_not_required" | "approved_current" | "approval_required_outstanding" => {
            GateVerdict::Clear
        }
        "approval_invalidated_by_changes" => GateVerdict::Block,
        _ => GateVerdict::Block,
    }
}

fn merge_queue_verdict(token: &str) -> GateVerdict {
    match token {
        "not_enqueued" => GateVerdict::Clear,
        "queued_position_known" => GateVerdict::Downgrade,
        "queued_invalidated_by_rewrite" | "queue_blocks_history_surgery" => GateVerdict::Block,
        _ => GateVerdict::Block,
    }
}

fn dirty_worktree_verdict(token: &str) -> GateVerdict {
    match token {
        "clean" | "untracked_present" => GateVerdict::Clear,
        "dirty_tracked_changes" => GateVerdict::Downgrade,
        "dirty_blocks_operation" => GateVerdict::Block,
        _ => GateVerdict::Block,
    }
}

fn conflict_source_verdict(token: &str) -> GateVerdict {
    match token {
        "no_conflicts" | "conflicts_resolved_ready_to_continue" => GateVerdict::Clear,
        "conflicts_present_blocks_continue" | "conflict_source_unknown_requires_refresh" => {
            GateVerdict::Block
        }
        _ => GateVerdict::Block,
    }
}

/// Provider overlay never blocks: an outage downgrades to local-only truth.
fn provider_overlay_verdict(token: &str) -> GateVerdict {
    match token {
        "overlay_fresh" => GateVerdict::Clear,
        "overlay_stale" | "overlay_unavailable_local_only" => GateVerdict::Downgrade,
        // An unknown overlay token is treated as a stale downgrade, never a block,
        // so an unexpected provider state can never gate local truth.
        _ => GateVerdict::Downgrade,
    }
}

/// Redaction-safe support-export projection for a review packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HistorySurgeryReviewSupportExport {
    /// Record-kind tag.
    pub record_kind: String,
    /// Stable export id.
    pub export_id: String,
    /// Sheet ids included in the export.
    pub sheet_refs: Vec<String>,
    /// Identity fields retained after redaction.
    pub reconstruction_fields: Vec<String>,
    /// True when no raw paths are embedded.
    pub raw_paths_redacted: bool,
    /// True when no raw patch/todo bodies are embedded.
    pub raw_patch_bodies_redacted: bool,
    /// True when no raw provider payloads are embedded.
    pub raw_provider_payloads_redacted: bool,
}

/// Top-level packet binding per-verb history-surgery review sheets.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HistorySurgeryReviewPacket {
    /// Record-kind tag.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Generation timestamp (RFC 3339).
    pub generated_at: String,
    /// Repository ref every sheet in this packet belongs to.
    pub repo_ref: String,
    /// Per-verb review sheets.
    pub sheets: Vec<HistorySurgeryReviewSheet>,
    /// Redaction-safe support-export projection.
    pub support_export: HistorySurgeryReviewSupportExport,
}

impl HistorySurgeryReviewPacket {
    /// Parses a packet from JSON and validates its cross-row invariants.
    ///
    /// # Errors
    ///
    /// Returns [`HistorySurgeryReviewError`] when the JSON is invalid or the
    /// parsed packet violates the history-surgery review contract.
    pub fn parse_json(input: &str) -> Result<Self, HistorySurgeryReviewError> {
        let packet: Self = serde_json::from_str(input).map_err(HistorySurgeryReviewError::Json)?;
        let violations = packet.validate();
        if violations.is_empty() {
            Ok(packet)
        } else {
            Err(HistorySurgeryReviewError::Validation(violations))
        }
    }

    /// Validates every sheet, decision, and support-export invariant.
    ///
    /// Returns every violation found rather than stopping at the first, so a
    /// regenerator or CI gate can report the full set at once.
    pub fn validate(&self) -> Vec<HistorySurgeryReviewValidationError> {
        let mut errors = Vec::new();

        if self.record_kind != HISTORY_SURGERY_REVIEW_PACKET_RECORD_KIND {
            errors.push(HistorySurgeryReviewValidationError::WrongRecordKind {
                observed: self.record_kind.clone(),
            });
        }
        if self.schema_version != HISTORY_SURGERY_REVIEW_SCHEMA_VERSION {
            errors.push(HistorySurgeryReviewValidationError::WrongSchemaVersion {
                observed: self.schema_version,
            });
        }
        if self.packet_id.trim().is_empty()
            || self.generated_at.trim().is_empty()
            || self.repo_ref.trim().is_empty()
        {
            errors.push(HistorySurgeryReviewValidationError::MissingIdentity);
        }
        if self.sheets.is_empty() {
            errors.push(HistorySurgeryReviewValidationError::NoSheets);
        }

        let mut sheet_ids: HashSet<&str> = HashSet::new();
        for sheet in &self.sheets {
            if !sheet_ids.insert(sheet.sheet_id.as_str()) {
                errors.push(HistorySurgeryReviewValidationError::DuplicateSheetId {
                    sheet_id: sheet.sheet_id.clone(),
                });
            }
            validate_sheet(sheet, &self.repo_ref, &mut errors);
        }

        validate_support_export(self, &sheet_ids, &mut errors);

        if json_contains_forbidden_boundary_material(
            &serde_json::to_value(self).expect("history surgery review packet serializes"),
        ) {
            errors.push(HistorySurgeryReviewValidationError::RawBoundaryMaterialInExport);
        }

        errors
    }

    /// Deterministic export-safe pretty JSON.
    ///
    /// # Panics
    ///
    /// Panics only if serializing this metadata-only packet fails.
    pub fn export_safe_json(&self) -> String {
        serde_json::to_string_pretty(self).expect("history surgery review packet serializes")
    }

    /// Deterministic Markdown summary for support, docs, or review handoff.
    pub fn render_markdown_summary(&self) -> String {
        let mut out = String::new();
        out.push_str("# History-Surgery Review Sheets\n\n");
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- Repository: `{}`\n", self.repo_ref));
        out.push_str(&format!("- Sheets: {}\n\n", self.sheets.len()));
        out.push_str("## Sheets\n\n");
        for sheet in &self.sheets {
            out.push_str(&format!(
                "- **{}** → `{}` (`{}`): {} — {} (recovery_visible {})\n",
                sheet.verb.as_str(),
                sheet.primary_target_ref,
                sheet.target_kind,
                sheet.decision.outcome.as_str(),
                sheet.decision.primary_reason,
                sheet.decision.recovery_visible,
            ));
        }
        out
    }
}

/// Reads and validates the checked-in canonical sheets packet.
///
/// # Errors
///
/// Returns [`HistorySurgeryReviewError`] when the checked-in packet fails to
/// parse or violates the history-surgery review contract.
pub fn current_history_surgery_review_sheets(
) -> Result<HistorySurgeryReviewPacket, HistorySurgeryReviewError> {
    HistorySurgeryReviewPacket::parse_json(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/git/m5/history_surgery_review/history_surgery_review_sheets.json"
    )))
}

fn validate_sheet(
    sheet: &HistorySurgeryReviewSheet,
    packet_repo_ref: &str,
    errors: &mut Vec<HistorySurgeryReviewValidationError>,
) {
    let sheet_id = sheet.sheet_id.clone();

    if sheet.record_kind != HISTORY_SURGERY_REVIEW_SHEET_RECORD_KIND {
        errors.push(HistorySurgeryReviewValidationError::WrongRecordKind {
            observed: sheet.record_kind.clone(),
        });
    }
    if sheet.sheet_id.trim().is_empty()
        || sheet.created_at.trim().is_empty()
        || sheet.updated_at.trim().is_empty()
        || sheet.summary_label.trim().is_empty()
    {
        errors.push(HistorySurgeryReviewValidationError::SheetMissingIdentity {
            sheet_id: sheet_id.clone(),
        });
    }
    if !sheet.identity_preserved() || sheet.repo_ref != packet_repo_ref {
        errors.push(
            HistorySurgeryReviewValidationError::SheetIdentityNotPreserved {
                sheet_id: sheet_id.clone(),
            },
        );
    }

    // Every risky mutation names an exact, unambiguous target.
    if sheet.primary_target_ref.trim().is_empty() {
        errors.push(HistorySurgeryReviewValidationError::MissingTarget {
            sheet_id: sheet_id.clone(),
        });
    }
    if !HISTORY_SURGERY_TARGET_KINDS.contains(&sheet.target_kind.as_str()) {
        errors.push(HistorySurgeryReviewValidationError::FieldOutOfVocabulary {
            sheet_id: sheet_id.clone(),
            field: "target_kind".to_owned(),
        });
    }

    validate_gate_vocab(sheet, &sheet_id, errors);
    validate_verb_specific(sheet, &sheet_id, errors);
    validate_local_actions(sheet, &sheet_id, errors);
    validate_decision(sheet, &sheet_id, errors);
}

fn validate_gate_vocab(
    sheet: &HistorySurgeryReviewSheet,
    sheet_id: &str,
    errors: &mut Vec<HistorySurgeryReviewValidationError>,
) {
    for (value, vocab, field) in [
        (
            sheet.protected_branch_posture.as_str(),
            RISKY_VCS_PROTECTED_BRANCH_POSTURES,
            "protected_branch_posture",
        ),
        (
            sheet.stale_review_state.as_str(),
            RISKY_VCS_APPROVAL_STATES,
            "stale_review_state",
        ),
        (
            sheet.merge_queue_state.as_str(),
            HISTORY_SURGERY_MERGE_QUEUE_STATES,
            "merge_queue_state",
        ),
        (
            sheet.dirty_worktree_state.as_str(),
            HISTORY_SURGERY_DIRTY_WORKTREE_STATES,
            "dirty_worktree_state",
        ),
        (
            sheet.conflict_source_state.as_str(),
            HISTORY_SURGERY_CONFLICT_SOURCE_STATES,
            "conflict_source_state",
        ),
        (
            sheet.provider_overlay_state.as_str(),
            HISTORY_SURGERY_PROVIDER_OVERLAY_STATES,
            "provider_overlay_state",
        ),
    ] {
        if !vocab.contains(&value) {
            errors.push(HistorySurgeryReviewValidationError::FieldOutOfVocabulary {
                sheet_id: sheet_id.to_owned(),
                field: field.to_owned(),
            });
        }
    }
}

fn validate_verb_specific(
    sheet: &HistorySurgeryReviewSheet,
    sheet_id: &str,
    errors: &mut Vec<HistorySurgeryReviewValidationError>,
) {
    if sheet.verb.requires_source_text() {
        let raw_ok = sheet
            .raw_source_text_ref
            .as_deref()
            .is_some_and(|value| !value.trim().is_empty());
        if !raw_ok {
            errors.push(
                HistorySurgeryReviewValidationError::SourceTextNotPreserved {
                    sheet_id: sheet_id.to_owned(),
                },
            );
        }
    }

    if sheet.verb.requires_source_commits() && sheet.secondary_refs.is_empty() {
        errors.push(HistorySurgeryReviewValidationError::MissingSourceCommits {
            sheet_id: sheet_id.to_owned(),
        });
    }

    if sheet.verb.requires_reset_mode() {
        match sheet.reset_mode.as_deref() {
            Some(mode) if HISTORY_SURGERY_RESET_MODES.contains(&mode) => {}
            _ => errors.push(HistorySurgeryReviewValidationError::FieldOutOfVocabulary {
                sheet_id: sheet_id.to_owned(),
                field: "reset_mode".to_owned(),
            }),
        }
    } else if sheet.reset_mode.is_some() {
        errors.push(
            HistorySurgeryReviewValidationError::VerbFieldNotApplicable {
                sheet_id: sheet_id.to_owned(),
                field: "reset_mode".to_owned(),
            },
        );
    }

    if sheet.verb.is_publish_class() {
        let lease_missing = match sheet.force_lease_ref.as_deref() {
            Some(value) => value.trim().is_empty(),
            None => true,
        };
        if lease_missing {
            errors.push(HistorySurgeryReviewValidationError::ForcePushMissingLease {
                sheet_id: sheet_id.to_owned(),
            });
        }
        match sheet.divergence_class.as_deref() {
            Some(class) if RISKY_VCS_DIVERGENCE_CLASSES.contains(&class) => {}
            _ => errors.push(HistorySurgeryReviewValidationError::FieldOutOfVocabulary {
                sheet_id: sheet_id.to_owned(),
                field: "divergence_class".to_owned(),
            }),
        }
    } else {
        if sheet.force_lease_ref.is_some() {
            errors.push(
                HistorySurgeryReviewValidationError::VerbFieldNotApplicable {
                    sheet_id: sheet_id.to_owned(),
                    field: "force_lease_ref".to_owned(),
                },
            );
        }
        if sheet.divergence_class.is_some() {
            errors.push(
                HistorySurgeryReviewValidationError::VerbFieldNotApplicable {
                    sheet_id: sheet_id.to_owned(),
                    field: "divergence_class".to_owned(),
                },
            );
        }
    }
}

fn validate_local_actions(
    sheet: &HistorySurgeryReviewSheet,
    sheet_id: &str,
    errors: &mut Vec<HistorySurgeryReviewValidationError>,
) {
    let mut seen: HashSet<&str> = HashSet::new();
    for action in &sheet.local_actions {
        if !HISTORY_SURGERY_LOCAL_ACTIONS.contains(&action.as_str()) {
            errors.push(HistorySurgeryReviewValidationError::FieldOutOfVocabulary {
                sheet_id: sheet_id.to_owned(),
                field: "local_actions".to_owned(),
            });
        }
        seen.insert(action.as_str());
    }
    // Local preview and abort truth must always be reachable, even offline.
    for required in HISTORY_SURGERY_REQUIRED_LOCAL_ACTIONS {
        if !seen.contains(required) {
            errors.push(HistorySurgeryReviewValidationError::MissingLocalAction {
                sheet_id: sheet_id.to_owned(),
                action: required.to_owned(),
            });
        }
    }
    // When a recovery path exists, restore must be offered as a local action.
    if sheet.recovery_visible() && !seen.contains("restore_checkpoint") {
        errors.push(HistorySurgeryReviewValidationError::MissingLocalAction {
            sheet_id: sheet_id.to_owned(),
            action: "restore_checkpoint".to_owned(),
        });
    }
}

fn validate_decision(
    sheet: &HistorySurgeryReviewSheet,
    sheet_id: &str,
    errors: &mut Vec<HistorySurgeryReviewValidationError>,
) {
    // The stored decision must equal the deterministic derivation; this is what
    // proves the sheet can explain why a mutation was allowed/blocked/downgraded.
    let derived = sheet.derive_decision();
    if sheet.decision != derived {
        errors.push(
            HistorySurgeryReviewValidationError::DecisionDoesNotMatchGates {
                sheet_id: sheet_id.to_owned(),
            },
        );
    }

    if !HISTORY_SURGERY_DECISION_REASONS.contains(&sheet.decision.primary_reason.as_str()) {
        errors.push(HistorySurgeryReviewValidationError::FieldOutOfVocabulary {
            sheet_id: sheet_id.to_owned(),
            field: "decision.primary_reason".to_owned(),
        });
    }

    // A risky mutation is never allowed without a visible recovery path.
    if sheet.decision.outcome == ReviewDecisionOutcome::Allowed && !sheet.decision.recovery_visible
    {
        errors.push(
            HistorySurgeryReviewValidationError::AllowedWithoutRecovery {
                sheet_id: sheet_id.to_owned(),
            },
        );
    }

    // A provider outage must never block local truth: even when the overlay is
    // unavailable, local preview/abort/restore stays available offline.
    if sheet.provider_overlay_state == "overlay_unavailable_local_only"
        && (sheet.decision.outcome == ReviewDecisionOutcome::Blocked
            && sheet
                .decision
                .contributing_gates
                .iter()
                .all(|gate| gate == "provider_overlay"))
    {
        errors.push(
            HistorySurgeryReviewValidationError::ProviderOutageBlocksLocalTruth {
                sheet_id: sheet_id.to_owned(),
            },
        );
    }
    if !sheet.decision.local_truth_available_offline && sheet.recovery_visible() {
        errors.push(
            HistorySurgeryReviewValidationError::ProviderOutageBlocksLocalTruth {
                sheet_id: sheet_id.to_owned(),
            },
        );
    }
}

fn validate_support_export(
    packet: &HistorySurgeryReviewPacket,
    sheet_ids: &HashSet<&str>,
    errors: &mut Vec<HistorySurgeryReviewValidationError>,
) {
    let export = &packet.support_export;
    if export.record_kind != HISTORY_SURGERY_REVIEW_SUPPORT_EXPORT_RECORD_KIND {
        errors.push(HistorySurgeryReviewValidationError::WrongRecordKind {
            observed: export.record_kind.clone(),
        });
    }
    for sheet_ref in &export.sheet_refs {
        if !sheet_ids.contains(sheet_ref.as_str()) {
            errors.push(
                HistorySurgeryReviewValidationError::UnknownSupportSheetRef {
                    sheet_ref: sheet_ref.clone(),
                },
            );
        }
    }
    // Every sheet must be reconstructable from the support export.
    for sheet in &packet.sheets {
        if !export
            .sheet_refs
            .iter()
            .any(|reference| reference == &sheet.sheet_id)
        {
            errors.push(
                HistorySurgeryReviewValidationError::SupportExportMissingSheet {
                    sheet_id: sheet.sheet_id.clone(),
                },
            );
        }
    }
    for required in HISTORY_SURGERY_REVIEW_REQUIRED_RECONSTRUCTION_FIELDS {
        if !export
            .reconstruction_fields
            .iter()
            .any(|field| field == required)
        {
            errors.push(
                HistorySurgeryReviewValidationError::SupportExportMissingField {
                    field: required.to_string(),
                },
            );
        }
    }
    if !export.raw_paths_redacted
        || !export.raw_patch_bodies_redacted
        || !export.raw_provider_payloads_redacted
    {
        errors.push(HistorySurgeryReviewValidationError::SupportExportEmbedsRawMaterial);
    }
}

/// Heuristic that rejects obviously forbidden material in export-safe JSON.
fn json_contains_forbidden_boundary_material(value: &serde_json::Value) -> bool {
    match value {
        serde_json::Value::String(text) => {
            let lower = text.to_lowercase();
            lower.contains("api_key")
                || lower.contains("password")
                || lower.contains("secret")
                || lower.contains("bearer ")
        }
        serde_json::Value::Array(items) => {
            items.iter().any(json_contains_forbidden_boundary_material)
        }
        serde_json::Value::Object(map) => {
            map.values().any(json_contains_forbidden_boundary_material)
        }
        _ => false,
    }
}

/// Error returned while parsing a history-surgery review packet.
#[derive(Debug)]
pub enum HistorySurgeryReviewError {
    /// JSON parsing failed.
    Json(serde_json::Error),
    /// Cross-row validation failed.
    Validation(Vec<HistorySurgeryReviewValidationError>),
}

impl fmt::Display for HistorySurgeryReviewError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Json(error) => {
                write!(
                    formatter,
                    "failed to parse history surgery review packet JSON: {error}"
                )
            }
            Self::Validation(errors) => {
                write!(
                    formatter,
                    "history surgery review packet has validation errors: "
                )?;
                for (index, error) in errors.iter().enumerate() {
                    if index > 0 {
                        write!(formatter, "; ")?;
                    }
                    write!(formatter, "{error}")?;
                }
                Ok(())
            }
        }
    }
}

impl Error for HistorySurgeryReviewError {}

/// Cross-row validation error for a history-surgery review packet.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum HistorySurgeryReviewValidationError {
    /// A record-kind tag does not match the stable contract.
    WrongRecordKind {
        /// Observed record-kind tag.
        observed: String,
    },
    /// The packet schema version is unsupported.
    WrongSchemaVersion {
        /// Observed schema version.
        observed: u32,
    },
    /// A required packet identity field is missing.
    MissingIdentity,
    /// The packet carries no sheets.
    NoSheets,
    /// A sheet id is declared more than once.
    DuplicateSheetId {
        /// Duplicated sheet id.
        sheet_id: String,
    },
    /// A sheet is missing a required identity field.
    SheetMissingIdentity {
        /// Sheet id.
        sheet_id: String,
    },
    /// A sheet does not preserve exact repo/worktree identity.
    SheetIdentityNotPreserved {
        /// Sheet id.
        sheet_id: String,
    },
    /// A sheet names no exact target ref.
    MissingTarget {
        /// Sheet id.
        sheet_id: String,
    },
    /// A field value is outside its closed vocabulary.
    FieldOutOfVocabulary {
        /// Sheet id.
        sheet_id: String,
        /// Field name.
        field: String,
    },
    /// A rebase/patch-apply sheet does not preserve raw todo/patch text.
    SourceTextNotPreserved {
        /// Sheet id.
        sheet_id: String,
    },
    /// A cherry-pick/revert sheet names no source commit refs.
    MissingSourceCommits {
        /// Sheet id.
        sheet_id: String,
    },
    /// A non-applicable verb-specific field is set on the wrong verb.
    VerbFieldNotApplicable {
        /// Sheet id.
        sheet_id: String,
        /// Field name.
        field: String,
    },
    /// A force-push sheet names no force-with-lease expected old value.
    ForcePushMissingLease {
        /// Sheet id.
        sheet_id: String,
    },
    /// A sheet omits a required local-first action.
    MissingLocalAction {
        /// Sheet id.
        sheet_id: String,
        /// Missing action.
        action: String,
    },
    /// A sheet's stored decision does not match its derived decision.
    DecisionDoesNotMatchGates {
        /// Sheet id.
        sheet_id: String,
    },
    /// A sheet is allowed without a visible recovery path.
    AllowedWithoutRecovery {
        /// Sheet id.
        sheet_id: String,
    },
    /// A provider outage blocks local preview/abort/restore truth.
    ProviderOutageBlocksLocalTruth {
        /// Sheet id.
        sheet_id: String,
    },
    /// A support-export sheet ref is unknown.
    UnknownSupportSheetRef {
        /// Unknown sheet ref.
        sheet_ref: String,
    },
    /// A sheet is missing from the support export lineage.
    SupportExportMissingSheet {
        /// Sheet id.
        sheet_id: String,
    },
    /// The support export omits a required reconstruction field.
    SupportExportMissingField {
        /// Missing reconstruction field.
        field: String,
    },
    /// The support export embeds raw paths, bodies, or provider payloads.
    SupportExportEmbedsRawMaterial,
    /// The export contains obviously forbidden boundary material.
    RawBoundaryMaterialInExport,
}

impl fmt::Display for HistorySurgeryReviewValidationError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::WrongRecordKind { observed } => {
                write!(formatter, "unexpected record kind {observed}")
            }
            Self::WrongSchemaVersion { observed } => {
                write!(formatter, "unsupported schema version {observed}")
            }
            Self::MissingIdentity => write!(formatter, "packet is missing identity fields"),
            Self::NoSheets => write!(formatter, "packet carries no sheets"),
            Self::DuplicateSheetId { sheet_id } => {
                write!(formatter, "sheet id {sheet_id} is declared more than once")
            }
            Self::SheetMissingIdentity { sheet_id } => {
                write!(formatter, "sheet {sheet_id} is missing identity fields")
            }
            Self::SheetIdentityNotPreserved { sheet_id } => write!(
                formatter,
                "sheet {sheet_id} does not preserve repo/worktree identity"
            ),
            Self::MissingTarget { sheet_id } => {
                write!(formatter, "sheet {sheet_id} names no exact target ref")
            }
            Self::FieldOutOfVocabulary { sheet_id, field } => {
                write!(
                    formatter,
                    "sheet {sheet_id} field {field} is out of vocabulary"
                )
            }
            Self::SourceTextNotPreserved { sheet_id } => write!(
                formatter,
                "sheet {sheet_id} does not preserve raw todo/patch text"
            ),
            Self::MissingSourceCommits { sheet_id } => {
                write!(formatter, "sheet {sheet_id} names no source commit refs")
            }
            Self::VerbFieldNotApplicable { sheet_id, field } => write!(
                formatter,
                "sheet {sheet_id} sets {field} on a verb that does not use it"
            ),
            Self::ForcePushMissingLease { sheet_id } => write!(
                formatter,
                "force-push sheet {sheet_id} names no force-with-lease expected value"
            ),
            Self::MissingLocalAction { sheet_id, action } => write!(
                formatter,
                "sheet {sheet_id} omits required local action {action}"
            ),
            Self::DecisionDoesNotMatchGates { sheet_id } => write!(
                formatter,
                "sheet {sheet_id} stored decision does not match its gate states"
            ),
            Self::AllowedWithoutRecovery { sheet_id } => write!(
                formatter,
                "sheet {sheet_id} is allowed without a visible recovery path"
            ),
            Self::ProviderOutageBlocksLocalTruth { sheet_id } => write!(
                formatter,
                "sheet {sheet_id} lets a provider outage block local truth"
            ),
            Self::UnknownSupportSheetRef { sheet_ref } => {
                write!(
                    formatter,
                    "support export references unknown sheet {sheet_ref}"
                )
            }
            Self::SupportExportMissingSheet { sheet_id } => {
                write!(formatter, "support export omits sheet {sheet_id}")
            }
            Self::SupportExportMissingField { field } => {
                write!(
                    formatter,
                    "support export missing reconstruction field {field}"
                )
            }
            Self::SupportExportEmbedsRawMaterial => write!(
                formatter,
                "support export embeds raw paths, bodies, or provider payloads"
            ),
            Self::RawBoundaryMaterialInExport => {
                write!(formatter, "export contains forbidden boundary material")
            }
        }
    }
}
