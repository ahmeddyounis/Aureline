//! Per-verb stash/shelf review sheets and reflog/checkpoint restore surfaces.
//!
//! Stash/shelf entries and reflog/checkpoint recovery used to be the part of the
//! Git story that quietly disappeared the moment a provider was stale or a risky
//! mutation only half-completed. A user who wanted to apply, pop, drop, or create a
//! branch from a stash had to trust transient modal state, and a recovery path —
//! the reflog entry or the durable checkpoint that lets a botched rewrite be
//! undone — was at best a toast that vanished on reload. This module replaces that
//! with one durable, serde-serializable [`StashRecoverySheet`] per verb. Each sheet
//! names the exact repository-or-worktree target it would act on, keeps the
//! pre-execution gate states inspectable, keeps the recovery path visible, and
//! carries the restore surface's expiry, retention, and compare / open-diff truth.
//!
//! The verbs stay **distinct**. Apply, pop, drop, and create-branch are never
//! collapsed into one generic "use stash" action, because they differ in whether
//! they consume the stash entry and in what they leave behind; reflog-restore and
//! checkpoint-restore are likewise distinct because one is a best-effort,
//! expiry-bounded fallback and the other is a durable, retained anchor. A sheet is
//! not a badge: it carries a [`StashRecoveryDecision`] *derived* from the gate
//! states by [`StashRecoverySheet::derive_decision`], so the product can always
//! explain why a verb was **allowed**, **blocked**, or **downgraded**, and a stored
//! decision can be re-derived and verified against its gates.
//!
//! Three guarantees are encoded in the derivation rather than left to prose:
//!
//! * A destructive verb is never *allowed* without a reachable recovery path (an
//!   explicit checkpoint, or an acknowledged reflog-only fallback), and the recovery
//!   path is visible before execution.
//! * When only a reflog-based recovery exists, the sheet preserves the caveats that
//!   come with it (the entry can expire; the index/untracked state may not restore),
//!   so a restore never silently pretends to be a full checkpoint.
//! * A provider or auth outage never blocks local truth: the provider-overlay gate
//!   can only **downgrade** a sheet to local-only, never block it, so local preview,
//!   continue, abort, stash inspection, and checkpoint restore stay reachable offline.
//!
//! The boundary schema is
//! [`schemas/git/stash-recovery.schema.json`](../../../../schemas/git/stash-recovery.schema.json).
//! The protected fixture corpus is
//! [`fixtures/git/m5/stash-recovery/`](../../../../fixtures/git/m5/stash-recovery/).
//! The checked-in canonical packet is
//! [`artifacts/git/m5/stash_recovery/stash_recovery.json`](../../../../artifacts/git/m5/stash_recovery/stash_recovery.json).

#[cfg(test)]
mod tests;

use std::collections::HashSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

/// Schema version for [`StashRecoveryPacket`].
pub const STASH_RECOVERY_SCHEMA_VERSION: u32 = 1;

/// Stable record-kind tag carried by [`StashRecoveryPacket`].
pub const STASH_RECOVERY_PACKET_RECORD_KIND: &str = "git_stash_recovery_packet";

/// Stable record-kind tag carried by [`StashRecoverySheet`].
pub const STASH_RECOVERY_SHEET_RECORD_KIND: &str = "git_stash_recovery_sheet";

/// Stable record-kind tag carried by [`StashRecoverySupportExport`].
pub const STASH_RECOVERY_SUPPORT_EXPORT_RECORD_KIND: &str = "git_stash_recovery_support_export";

/// Repo-relative path of the boundary schema.
pub const STASH_RECOVERY_SCHEMA_REF: &str = "schemas/git/stash-recovery.schema.json";

/// Repo-relative path of the protected fixture corpus directory.
pub const STASH_RECOVERY_FIXTURE_DIR: &str = "fixtures/git/m5/stash-recovery";

/// Repo-relative path of the checked-in canonical sheets packet.
pub const STASH_RECOVERY_ARTIFACT_REF: &str = "artifacts/git/m5/stash_recovery/stash_recovery.json";

/// Identity fields a support export must retain after redaction so a sheet can be
/// reconstructed without leaking raw boundary material.
pub const STASH_RECOVERY_REQUIRED_RECONSTRUCTION_FIELDS: [&str; 8] = [
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
/// Every sheet names exactly one, so a verb can never be ambiguous about whether it
/// targets the repository root or a specific linked worktree.
pub const STASH_RECOVERY_TARGET_KINDS: &[&str] = &["repository_root", "linked_worktree"];

/// Closed set of dirty-worktree states a sheet may surface before execution.
pub const STASH_RECOVERY_DIRTY_WORKTREE_STATES: &[&str] = &[
    "clean",
    "untracked_present",
    "dirty_tracked_changes",
    "dirty_blocks_operation",
];

/// Closed set of conflict-source states a sheet may surface before execution.
pub const STASH_RECOVERY_CONFLICT_SOURCE_STATES: &[&str] = &[
    "no_conflicts",
    "conflicts_resolved_ready_to_continue",
    "conflicts_present_blocks_continue",
    "conflict_source_unknown_requires_refresh",
];

/// Closed set of provider-overlay states layered over local Git truth.
pub const STASH_RECOVERY_PROVIDER_OVERLAY_STATES: &[&str] = &[
    "overlay_fresh",
    "overlay_stale",
    "overlay_unavailable_local_only",
];

/// Closed set of stash-availability states a stash verb may surface.
///
/// `not_applicable` is the only legal value for a recovery verb, which acts on a
/// reflog/checkpoint anchor rather than a stash entry.
pub const STASH_RECOVERY_STASH_AVAILABILITY_STATES: &[&str] = &[
    "not_applicable",
    "stash_present",
    "stash_already_applied_present",
    "stash_missing_or_consumed",
];

/// Closed set of recovery-anchor expiry states a recovery verb may surface.
///
/// `not_applicable` is the only legal value for a stash verb, which carries no
/// reflog/checkpoint anchor of its own.
pub const STASH_RECOVERY_ANCHOR_EXPIRY_STATES: &[&str] = &[
    "not_applicable",
    "fresh_retained",
    "expiring_soon",
    "expired_unrecoverable",
];

/// Closed set of recovery-anchor kinds.
pub const STASH_RECOVERY_ANCHOR_KINDS: &[&str] = &["reflog", "checkpoint"];

/// Closed set of retention classes a recovery anchor may declare.
pub const STASH_RECOVERY_RETENTION_CLASSES: &[&str] = &[
    "session_only",
    "retained_default_window",
    "retained_extended",
    "pinned_no_expiry",
];

/// Closed set of restore caveats a sheet may preserve.
///
/// These keep a reflog-only restore honest about what it cannot guarantee, so a
/// recovery never pretends to be a full checkpoint.
pub const STASH_RECOVERY_RESTORE_CAVEATS: &[&str] = &[
    "reflog_entry_may_expire",
    "reflog_only_no_full_checkpoint",
    "index_state_not_restored",
    "untracked_files_not_restored",
    "anchor_expiring_soon",
];

/// Closed set of local-first actions every sheet may keep available offline.
///
/// These never depend on a provider; they are what keeps local preview, continue,
/// abort, stash inspection, and checkpoint restore reachable during an outage.
pub const STASH_RECOVERY_LOCAL_ACTIONS: &[&str] = &[
    "preview",
    "continue",
    "abort",
    "inspect_stash",
    "restore_checkpoint",
    "compare",
    "open_diff",
];

/// Local-first actions a sheet must always offer regardless of provider state.
pub const STASH_RECOVERY_REQUIRED_LOCAL_ACTIONS: [&str; 2] = ["preview", "abort"];

/// Closed set of decision reason tokens carried by a [`StashRecoveryDecision`].
pub const STASH_RECOVERY_DECISION_REASONS: &[&str] = &[
    "allowed_with_recovery",
    "blocked_stash_unavailable",
    "blocked_recovery_anchor_expired",
    "blocked_unresolved_conflict",
    "blocked_dirty_worktree",
    "blocked_no_recovery_path",
    "downgraded_recovery_anchor_expiring",
    "downgraded_dirty_autostash",
    "downgraded_checkpoint_to_reflog_only",
    "downgraded_provider_unavailable_local_only",
    "downgraded_provider_overlay_stale",
];

/// A stash/shelf or reflog/checkpoint recovery verb that carries its own sheet.
///
/// The verbs stay distinct because each has different consume, target, and recovery
/// semantics; they are never normalized into one generic "use stash" or "recover"
/// action.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum StashRecoveryVerb {
    /// Apply a stash entry to the worktree, keeping it in the stash list.
    StashApply,
    /// Apply a stash entry to the worktree and drop it on success.
    StashPop,
    /// Drop a stash entry without applying it.
    StashDrop,
    /// Create a branch from a stash entry, then drop the entry.
    StashCreateBranch,
    /// Restore a ref position from the reflog (best-effort, expiry-bounded).
    ReflogRestore,
    /// Restore from a durable, retained checkpoint anchor.
    CheckpointRestore,
}

impl StashRecoveryVerb {
    /// Every verb, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::StashApply,
        Self::StashPop,
        Self::StashDrop,
        Self::StashCreateBranch,
        Self::ReflogRestore,
        Self::CheckpointRestore,
    ];

    /// Stable token used by fixtures, schemas, and support packets.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::StashApply => "stash_apply",
            Self::StashPop => "stash_pop",
            Self::StashDrop => "stash_drop",
            Self::StashCreateBranch => "stash_create_branch",
            Self::ReflogRestore => "reflog_restore",
            Self::CheckpointRestore => "checkpoint_restore",
        }
    }

    /// Whether the verb acts on a stash/shelf entry.
    pub const fn is_stash_verb(self) -> bool {
        matches!(
            self,
            Self::StashApply | Self::StashPop | Self::StashDrop | Self::StashCreateBranch
        )
    }

    /// Whether the verb restores from a reflog or checkpoint recovery anchor.
    pub const fn is_recovery_verb(self) -> bool {
        matches!(self, Self::ReflogRestore | Self::CheckpointRestore)
    }

    /// Whether the verb requires an exact stash entry it acts on.
    pub const fn requires_stash_entry(self) -> bool {
        self.is_stash_verb()
    }

    /// Whether the verb names a new branch it creates from a stash entry.
    pub const fn requires_new_branch(self) -> bool {
        matches!(self, Self::StashCreateBranch)
    }

    /// Whether the verb requires a reflog/checkpoint recovery anchor it restores from.
    pub const fn requires_recovery_anchor(self) -> bool {
        self.is_recovery_verb()
    }

    /// Whether the verb consumes (removes) the stash entry it acts on.
    ///
    /// Apply preserves the entry; pop, drop, and create-branch consume it. This is
    /// the core distinction that keeps the four stash verbs from collapsing.
    pub const fn consumes_stash_entry(self) -> bool {
        matches!(
            self,
            Self::StashPop | Self::StashDrop | Self::StashCreateBranch
        )
    }

    /// The anchor kind a recovery verb must restore from, if any.
    pub const fn expected_anchor_kind(self) -> Option<&'static str> {
        match self {
            Self::ReflogRestore => Some("reflog"),
            Self::CheckpointRestore => Some("checkpoint"),
            _ => None,
        }
    }
}

/// Verdict a single gate contributes toward the overall [`StashRecoveryDecision`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum GateVerdict {
    /// The gate places no constraint on the verb.
    Clear,
    /// The gate narrows the verb but does not block it.
    Downgrade,
    /// The gate blocks the verb until it is resolved.
    Block,
}

/// Overall outcome of a sheet's gate evaluation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum StashRecoveryOutcome {
    /// The verb may run; a recovery path is visible.
    Allowed,
    /// At least one gate narrows the verb (reflog-only recovery, anchor expiring,
    /// autostash, provider unavailable, or a stale overlay).
    Downgraded,
    /// At least one gate blocks the verb until it is resolved.
    Blocked,
}

impl StashRecoveryOutcome {
    /// Stable token used by fixtures, schemas, and support packets.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Allowed => "allowed",
            Self::Downgraded => "downgraded",
            Self::Blocked => "blocked",
        }
    }

    /// Whether the outcome permits the verb to run.
    pub const fn permits_execution(self) -> bool {
        matches!(self, Self::Allowed)
    }
}

/// The reflog/checkpoint anchor a recovery verb restores from.
///
/// This is the durable restore surface the spec requires: it names the exact anchor,
/// its expiry instant and retention class, and the compare / open-diff actions a user
/// can run before committing to the restore.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RecoveryAnchor {
    /// Anchor kind (`reflog` or `checkpoint`).
    pub anchor_kind: String,
    /// Redaction-safe ref to the exact anchor position.
    pub anchor_ref: String,
    /// Expiry instant (RFC 3339), or `null` for a pinned anchor that never expires.
    pub expires_at: Option<String>,
    /// Retention class from [`STASH_RECOVERY_RETENTION_CLASSES`].
    pub retention_class: String,
    /// Ref to the compare action (anchor vs current state).
    pub compare_action_ref: String,
    /// Ref to the open-diff action for this anchor.
    pub open_diff_action_ref: String,
}

/// The derived, explainable decision for a single sheet.
///
/// Every field is a deterministic function of the sheet's verb, target, and gate
/// states, so a stored decision can be re-derived and verified — the product can
/// always say *why* a verb was allowed, blocked, or downgraded.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StashRecoveryDecision {
    /// Overall outcome.
    pub outcome: StashRecoveryOutcome,
    /// Primary reason token from [`STASH_RECOVERY_DECISION_REASONS`].
    pub primary_reason: String,
    /// Gate tokens that contributed the decisive severity, in priority order.
    pub contributing_gates: Vec<String>,
    /// Whether a reachable recovery path is visible before execution.
    pub recovery_visible: bool,
    /// Whether local preview/continue/abort/inspect/restore truth is available
    /// without a provider.
    pub local_truth_available_offline: bool,
    /// Redaction-safe, deterministic one-line explanation.
    pub explanation: String,
}

/// A durable per-verb stash/recovery review sheet with explicit target truth.
///
/// One sheet exists per verb a user can run on a target. The structured gate fields
/// are the canonical substrate; [`StashRecoverySheet::new`] derives the
/// [`StashRecoveryDecision`] from them, so the sheet is never a badge.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StashRecoverySheet {
    /// Record-kind tag; must equal [`STASH_RECOVERY_SHEET_RECORD_KIND`].
    pub record_kind: String,
    /// Stable sheet identity (referenced by rows and support export).
    pub sheet_id: String,
    /// The verb this sheet reviews.
    pub verb: StashRecoveryVerb,
    /// Redaction-safe repository ref the sheet belongs to.
    pub repo_ref: String,
    /// Redaction-safe worktree ref the verb runs in.
    pub worktree_ref: String,
    /// Whether the verb targets the repository root or a linked worktree.
    pub target_kind: String,
    /// Exact, unambiguous target ref the verb would act on.
    pub primary_target_ref: String,
    /// Redaction-safe ref to the exact stash entry (stash verbs only).
    pub stash_entry_ref: Option<String>,
    /// Stash list index of the entry (stash verbs only).
    pub stash_index: Option<u32>,
    /// New branch ref created from the entry ([`StashRecoveryVerb::StashCreateBranch`]).
    pub new_branch_ref: Option<String>,
    /// The reflog/checkpoint anchor restored from (recovery verbs only).
    pub recovery_anchor: Option<RecoveryAnchor>,
    /// Stash-availability gate (from [`STASH_RECOVERY_STASH_AVAILABILITY_STATES`]).
    pub stash_availability_state: String,
    /// Anchor-expiry gate (from [`STASH_RECOVERY_ANCHOR_EXPIRY_STATES`]).
    pub anchor_expiry_state: String,
    /// Dirty-worktree gate (from [`STASH_RECOVERY_DIRTY_WORKTREE_STATES`]).
    pub dirty_worktree_state: String,
    /// Conflict-source gate (from [`STASH_RECOVERY_CONFLICT_SOURCE_STATES`]).
    pub conflict_source_state: String,
    /// Provider-overlay gate (from [`STASH_RECOVERY_PROVIDER_OVERLAY_STATES`]).
    pub provider_overlay_state: String,
    /// Checkpoint lineage refs protecting this verb's recovery.
    pub checkpoint_lineage_refs: Vec<String>,
    /// True when only a reflog-only fallback is available (no full checkpoint).
    pub reflog_only_fallback: bool,
    /// Caveats preserved when recovery is reflog-only or an anchor is expiring.
    pub restore_caveats: Vec<String>,
    /// Local-first actions kept available regardless of provider state.
    pub local_actions: Vec<String>,
    /// Derived decision; equals [`StashRecoverySheet::derive_decision`].
    pub decision: StashRecoveryDecision,
    /// Created timestamp (RFC 3339).
    pub created_at: String,
    /// Updated timestamp (RFC 3339).
    pub updated_at: String,
    /// Redaction-safe summary label.
    pub summary_label: String,
}

/// Constructor input for [`StashRecoverySheet::new`] (decision excluded).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StashRecoverySheetInput {
    /// Stable sheet identity.
    pub sheet_id: String,
    /// The verb this sheet reviews.
    pub verb: StashRecoveryVerb,
    /// Redaction-safe repository ref.
    pub repo_ref: String,
    /// Redaction-safe worktree ref.
    pub worktree_ref: String,
    /// Target kind token.
    pub target_kind: String,
    /// Exact target ref.
    pub primary_target_ref: String,
    /// Exact stash entry ref (stash verbs only).
    pub stash_entry_ref: Option<String>,
    /// Stash list index (stash verbs only).
    pub stash_index: Option<u32>,
    /// New branch ref (create-branch only).
    pub new_branch_ref: Option<String>,
    /// Reflog/checkpoint anchor (recovery verbs only).
    pub recovery_anchor: Option<RecoveryAnchor>,
    /// Stash-availability gate.
    pub stash_availability_state: String,
    /// Anchor-expiry gate.
    pub anchor_expiry_state: String,
    /// Dirty-worktree gate.
    pub dirty_worktree_state: String,
    /// Conflict-source gate.
    pub conflict_source_state: String,
    /// Provider-overlay gate.
    pub provider_overlay_state: String,
    /// Checkpoint lineage refs.
    pub checkpoint_lineage_refs: Vec<String>,
    /// True when only a reflog-only fallback is available.
    pub reflog_only_fallback: bool,
    /// Preserved restore caveats.
    pub restore_caveats: Vec<String>,
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

impl StashRecoverySheet {
    /// Builds a sheet from input and derives its decision from the gate states.
    pub fn new(input: StashRecoverySheetInput) -> Self {
        let mut sheet = Self {
            record_kind: STASH_RECOVERY_SHEET_RECORD_KIND.to_owned(),
            sheet_id: input.sheet_id,
            verb: input.verb,
            repo_ref: input.repo_ref,
            worktree_ref: input.worktree_ref,
            target_kind: input.target_kind,
            primary_target_ref: input.primary_target_ref,
            stash_entry_ref: input.stash_entry_ref,
            stash_index: input.stash_index,
            new_branch_ref: input.new_branch_ref,
            recovery_anchor: input.recovery_anchor,
            stash_availability_state: input.stash_availability_state,
            anchor_expiry_state: input.anchor_expiry_state,
            dirty_worktree_state: input.dirty_worktree_state,
            conflict_source_state: input.conflict_source_state,
            provider_overlay_state: input.provider_overlay_state,
            checkpoint_lineage_refs: input.checkpoint_lineage_refs,
            reflog_only_fallback: input.reflog_only_fallback,
            restore_caveats: input.restore_caveats,
            local_actions: input.local_actions,
            // Placeholder; replaced by the derived decision immediately below.
            decision: StashRecoveryDecision {
                outcome: StashRecoveryOutcome::Blocked,
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

    /// Whether a reachable recovery path is visible for this verb.
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
                name: "stash_availability",
                verdict: stash_availability_verdict(&self.stash_availability_state),
                reason: "blocked_stash_unavailable",
            },
            GateOutcome {
                name: "anchor_expiry",
                verdict: anchor_expiry_verdict(&self.anchor_expiry_state),
                reason: match anchor_expiry_verdict(&self.anchor_expiry_state) {
                    GateVerdict::Downgrade => "downgraded_recovery_anchor_expiring",
                    _ => "blocked_recovery_anchor_expired",
                },
            },
            GateOutcome {
                name: "conflict_source",
                verdict: conflict_source_verdict(&self.conflict_source_state),
                reason: "blocked_unresolved_conflict",
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
                name: "recovery",
                verdict: self.recovery_verdict(),
                reason: match self.recovery_verdict() {
                    GateVerdict::Downgrade => "downgraded_checkpoint_to_reflog_only",
                    _ => "blocked_no_recovery_path",
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

    /// Recovery gate: a verb with no recovery is blocked; a reflog-only fallback is
    /// a downgrade; an explicit checkpoint is clear.
    fn recovery_verdict(&self) -> GateVerdict {
        if !self.checkpoint_lineage_refs.is_empty() {
            GateVerdict::Clear
        } else if self.reflog_only_fallback {
            GateVerdict::Downgrade
        } else {
            GateVerdict::Block
        }
    }

    /// Derives the explainable decision from the gate states.
    ///
    /// The outcome is the worst severity across all gates; the primary reason is the
    /// first gate (in priority order) at that severity. Provider outages only ever
    /// downgrade, so they never block local truth.
    pub fn derive_decision(&self) -> StashRecoveryDecision {
        let gates = self.gate_outcomes();
        let recovery_visible = self.recovery_visible();
        // Local preview/continue/abort/inspect/restore truth is reachable whenever a
        // recovery path exists, independent of provider availability.
        let local_truth_available_offline = recovery_visible;

        let blocking: Vec<&GateOutcome> = gates
            .iter()
            .filter(|gate| gate.verdict == GateVerdict::Block)
            .collect();
        let downgrading: Vec<&GateOutcome> = gates
            .iter()
            .filter(|gate| gate.verdict == GateVerdict::Downgrade)
            .collect();

        let (outcome, decisive): (StashRecoveryOutcome, &[&GateOutcome]) = if !blocking.is_empty() {
            (StashRecoveryOutcome::Blocked, blocking.as_slice())
        } else if !downgrading.is_empty() {
            (StashRecoveryOutcome::Downgraded, downgrading.as_slice())
        } else {
            (StashRecoveryOutcome::Allowed, &[])
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

        StashRecoveryDecision {
            outcome,
            primary_reason,
            contributing_gates,
            recovery_visible,
            local_truth_available_offline,
            explanation,
        }
    }
}

fn stash_availability_verdict(token: &str) -> GateVerdict {
    match token {
        "not_applicable" | "stash_present" | "stash_already_applied_present" => GateVerdict::Clear,
        "stash_missing_or_consumed" => GateVerdict::Block,
        _ => GateVerdict::Block,
    }
}

fn anchor_expiry_verdict(token: &str) -> GateVerdict {
    match token {
        "not_applicable" | "fresh_retained" => GateVerdict::Clear,
        "expiring_soon" => GateVerdict::Downgrade,
        "expired_unrecoverable" => GateVerdict::Block,
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

fn dirty_worktree_verdict(token: &str) -> GateVerdict {
    match token {
        "clean" | "untracked_present" => GateVerdict::Clear,
        "dirty_tracked_changes" => GateVerdict::Downgrade,
        "dirty_blocks_operation" => GateVerdict::Block,
        _ => GateVerdict::Block,
    }
}

/// Provider overlay never blocks: an outage downgrades to local-only truth.
fn provider_overlay_verdict(token: &str) -> GateVerdict {
    match token {
        "overlay_fresh" => GateVerdict::Clear,
        "overlay_stale" | "overlay_unavailable_local_only" => GateVerdict::Downgrade,
        // An unknown overlay token is treated as a stale downgrade, never a block, so
        // an unexpected provider state can never gate local truth.
        _ => GateVerdict::Downgrade,
    }
}

/// Redaction-safe support-export projection for a stash/recovery packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StashRecoverySupportExport {
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
    /// True when no raw patch/diff bodies are embedded.
    pub raw_patch_bodies_redacted: bool,
    /// True when no raw provider payloads are embedded.
    pub raw_provider_payloads_redacted: bool,
}

/// Top-level packet binding per-verb stash/recovery review sheets.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StashRecoveryPacket {
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
    pub sheets: Vec<StashRecoverySheet>,
    /// Redaction-safe support-export projection.
    pub support_export: StashRecoverySupportExport,
}

impl StashRecoveryPacket {
    /// Parses a packet from JSON and validates its cross-row invariants.
    ///
    /// # Errors
    ///
    /// Returns [`StashRecoveryError`] when the JSON is invalid or the parsed packet
    /// violates the stash/recovery contract.
    pub fn parse_json(input: &str) -> Result<Self, StashRecoveryError> {
        let packet: Self = serde_json::from_str(input).map_err(StashRecoveryError::Json)?;
        let violations = packet.validate();
        if violations.is_empty() {
            Ok(packet)
        } else {
            Err(StashRecoveryError::Validation(violations))
        }
    }

    /// Validates every sheet, decision, and support-export invariant.
    ///
    /// Returns every violation found rather than stopping at the first, so a
    /// regenerator or CI gate can report the full set at once.
    pub fn validate(&self) -> Vec<StashRecoveryValidationError> {
        let mut errors = Vec::new();

        if self.record_kind != STASH_RECOVERY_PACKET_RECORD_KIND {
            errors.push(StashRecoveryValidationError::WrongRecordKind {
                observed: self.record_kind.clone(),
            });
        }
        if self.schema_version != STASH_RECOVERY_SCHEMA_VERSION {
            errors.push(StashRecoveryValidationError::WrongSchemaVersion {
                observed: self.schema_version,
            });
        }
        if self.packet_id.trim().is_empty()
            || self.generated_at.trim().is_empty()
            || self.repo_ref.trim().is_empty()
        {
            errors.push(StashRecoveryValidationError::MissingIdentity);
        }
        if self.sheets.is_empty() {
            errors.push(StashRecoveryValidationError::NoSheets);
        }

        let mut sheet_ids: HashSet<&str> = HashSet::new();
        for sheet in &self.sheets {
            if !sheet_ids.insert(sheet.sheet_id.as_str()) {
                errors.push(StashRecoveryValidationError::DuplicateSheetId {
                    sheet_id: sheet.sheet_id.clone(),
                });
            }
            validate_sheet(sheet, &self.repo_ref, &mut errors);
        }

        validate_support_export(self, &sheet_ids, &mut errors);

        if json_contains_forbidden_boundary_material(
            &serde_json::to_value(self).expect("stash recovery packet serializes"),
        ) {
            errors.push(StashRecoveryValidationError::RawBoundaryMaterialInExport);
        }

        errors
    }

    /// Deterministic export-safe pretty JSON.
    ///
    /// # Panics
    ///
    /// Panics only if serializing this metadata-only packet fails.
    pub fn export_safe_json(&self) -> String {
        serde_json::to_string_pretty(self).expect("stash recovery packet serializes")
    }

    /// Deterministic Markdown summary for support, docs, or review handoff.
    pub fn render_markdown_summary(&self) -> String {
        let mut out = String::new();
        out.push_str("# Stash & Recovery Review Sheets\n\n");
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
/// Returns [`StashRecoveryError`] when the checked-in packet fails to parse or
/// violates the stash/recovery contract.
pub fn current_stash_recovery_sheets() -> Result<StashRecoveryPacket, StashRecoveryError> {
    StashRecoveryPacket::parse_json(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/git/m5/stash_recovery/stash_recovery.json"
    )))
}

fn validate_sheet(
    sheet: &StashRecoverySheet,
    packet_repo_ref: &str,
    errors: &mut Vec<StashRecoveryValidationError>,
) {
    let sheet_id = sheet.sheet_id.clone();

    if sheet.record_kind != STASH_RECOVERY_SHEET_RECORD_KIND {
        errors.push(StashRecoveryValidationError::WrongRecordKind {
            observed: sheet.record_kind.clone(),
        });
    }
    if sheet.sheet_id.trim().is_empty()
        || sheet.created_at.trim().is_empty()
        || sheet.updated_at.trim().is_empty()
        || sheet.summary_label.trim().is_empty()
    {
        errors.push(StashRecoveryValidationError::SheetMissingIdentity {
            sheet_id: sheet_id.clone(),
        });
    }
    if !sheet.identity_preserved() || sheet.repo_ref != packet_repo_ref {
        errors.push(StashRecoveryValidationError::SheetIdentityNotPreserved {
            sheet_id: sheet_id.clone(),
        });
    }

    // Every verb names an exact, unambiguous target.
    if sheet.primary_target_ref.trim().is_empty() {
        errors.push(StashRecoveryValidationError::MissingTarget {
            sheet_id: sheet_id.clone(),
        });
    }
    if !STASH_RECOVERY_TARGET_KINDS.contains(&sheet.target_kind.as_str()) {
        errors.push(StashRecoveryValidationError::FieldOutOfVocabulary {
            sheet_id: sheet_id.clone(),
            field: "target_kind".to_owned(),
        });
    }

    validate_gate_vocab(sheet, &sheet_id, errors);
    validate_verb_specific(sheet, &sheet_id, errors);
    validate_caveats(sheet, &sheet_id, errors);
    validate_local_actions(sheet, &sheet_id, errors);
    validate_decision(sheet, &sheet_id, errors);
}

fn validate_gate_vocab(
    sheet: &StashRecoverySheet,
    sheet_id: &str,
    errors: &mut Vec<StashRecoveryValidationError>,
) {
    for (value, vocab, field) in [
        (
            sheet.stash_availability_state.as_str(),
            STASH_RECOVERY_STASH_AVAILABILITY_STATES,
            "stash_availability_state",
        ),
        (
            sheet.anchor_expiry_state.as_str(),
            STASH_RECOVERY_ANCHOR_EXPIRY_STATES,
            "anchor_expiry_state",
        ),
        (
            sheet.dirty_worktree_state.as_str(),
            STASH_RECOVERY_DIRTY_WORKTREE_STATES,
            "dirty_worktree_state",
        ),
        (
            sheet.conflict_source_state.as_str(),
            STASH_RECOVERY_CONFLICT_SOURCE_STATES,
            "conflict_source_state",
        ),
        (
            sheet.provider_overlay_state.as_str(),
            STASH_RECOVERY_PROVIDER_OVERLAY_STATES,
            "provider_overlay_state",
        ),
    ] {
        if !vocab.contains(&value) {
            errors.push(StashRecoveryValidationError::FieldOutOfVocabulary {
                sheet_id: sheet_id.to_owned(),
                field: field.to_owned(),
            });
        }
    }
}

fn validate_verb_specific(
    sheet: &StashRecoverySheet,
    sheet_id: &str,
    errors: &mut Vec<StashRecoveryValidationError>,
) {
    // Stash verbs name an exact stash entry; recovery verbs never carry one.
    if sheet.verb.requires_stash_entry() {
        let entry_ok = sheet
            .stash_entry_ref
            .as_deref()
            .is_some_and(|value| !value.trim().is_empty());
        if !entry_ok || sheet.stash_index.is_none() {
            errors.push(StashRecoveryValidationError::MissingStashEntry {
                sheet_id: sheet_id.to_owned(),
            });
        }
        if sheet.stash_availability_state == "not_applicable" {
            errors.push(StashRecoveryValidationError::FieldOutOfVocabulary {
                sheet_id: sheet_id.to_owned(),
                field: "stash_availability_state".to_owned(),
            });
        }
    } else {
        if sheet.stash_entry_ref.is_some() {
            errors.push(StashRecoveryValidationError::VerbFieldNotApplicable {
                sheet_id: sheet_id.to_owned(),
                field: "stash_entry_ref".to_owned(),
            });
        }
        if sheet.stash_index.is_some() {
            errors.push(StashRecoveryValidationError::VerbFieldNotApplicable {
                sheet_id: sheet_id.to_owned(),
                field: "stash_index".to_owned(),
            });
        }
        if sheet.stash_availability_state != "not_applicable" {
            errors.push(StashRecoveryValidationError::FieldOutOfVocabulary {
                sheet_id: sheet_id.to_owned(),
                field: "stash_availability_state".to_owned(),
            });
        }
    }

    // Only create-branch names a new branch ref.
    if sheet.verb.requires_new_branch() {
        let branch_ok = sheet
            .new_branch_ref
            .as_deref()
            .is_some_and(|value| !value.trim().is_empty());
        if !branch_ok {
            errors.push(StashRecoveryValidationError::MissingNewBranch {
                sheet_id: sheet_id.to_owned(),
            });
        }
    } else if sheet.new_branch_ref.is_some() {
        errors.push(StashRecoveryValidationError::VerbFieldNotApplicable {
            sheet_id: sheet_id.to_owned(),
            field: "new_branch_ref".to_owned(),
        });
    }

    // Recovery verbs carry a validated anchor; stash verbs never do.
    if sheet.verb.requires_recovery_anchor() {
        match &sheet.recovery_anchor {
            Some(anchor) => validate_recovery_anchor(sheet, anchor, sheet_id, errors),
            None => errors.push(StashRecoveryValidationError::MissingRecoveryAnchor {
                sheet_id: sheet_id.to_owned(),
            }),
        }
        if sheet.anchor_expiry_state == "not_applicable" {
            errors.push(StashRecoveryValidationError::FieldOutOfVocabulary {
                sheet_id: sheet_id.to_owned(),
                field: "anchor_expiry_state".to_owned(),
            });
        }
    } else {
        if sheet.recovery_anchor.is_some() {
            errors.push(StashRecoveryValidationError::VerbFieldNotApplicable {
                sheet_id: sheet_id.to_owned(),
                field: "recovery_anchor".to_owned(),
            });
        }
        if sheet.anchor_expiry_state != "not_applicable" {
            errors.push(StashRecoveryValidationError::FieldOutOfVocabulary {
                sheet_id: sheet_id.to_owned(),
                field: "anchor_expiry_state".to_owned(),
            });
        }
    }
}

fn validate_recovery_anchor(
    sheet: &StashRecoverySheet,
    anchor: &RecoveryAnchor,
    sheet_id: &str,
    errors: &mut Vec<StashRecoveryValidationError>,
) {
    if !STASH_RECOVERY_ANCHOR_KINDS.contains(&anchor.anchor_kind.as_str()) {
        errors.push(StashRecoveryValidationError::FieldOutOfVocabulary {
            sheet_id: sheet_id.to_owned(),
            field: "recovery_anchor.anchor_kind".to_owned(),
        });
    }
    // The anchor kind must match the restore verb (a reflog-restore restores from a
    // reflog entry; a checkpoint-restore from a durable checkpoint).
    if let Some(expected) = sheet.verb.expected_anchor_kind() {
        if anchor.anchor_kind != expected {
            errors.push(StashRecoveryValidationError::AnchorKindMismatch {
                sheet_id: sheet_id.to_owned(),
            });
        }
    }
    if anchor.anchor_ref.trim().is_empty() {
        errors.push(StashRecoveryValidationError::MissingTarget {
            sheet_id: sheet_id.to_owned(),
        });
    }
    if !STASH_RECOVERY_RETENTION_CLASSES.contains(&anchor.retention_class.as_str()) {
        errors.push(StashRecoveryValidationError::FieldOutOfVocabulary {
            sheet_id: sheet_id.to_owned(),
            field: "recovery_anchor.retention_class".to_owned(),
        });
    }
    // The restore surface must offer compare and open-diff actions.
    if anchor.compare_action_ref.trim().is_empty() || anchor.open_diff_action_ref.trim().is_empty()
    {
        errors.push(StashRecoveryValidationError::AnchorMissingCompareActions {
            sheet_id: sheet_id.to_owned(),
        });
    }
}

fn validate_caveats(
    sheet: &StashRecoverySheet,
    sheet_id: &str,
    errors: &mut Vec<StashRecoveryValidationError>,
) {
    for caveat in &sheet.restore_caveats {
        if !STASH_RECOVERY_RESTORE_CAVEATS.contains(&caveat.as_str()) {
            errors.push(StashRecoveryValidationError::FieldOutOfVocabulary {
                sheet_id: sheet_id.to_owned(),
                field: "restore_caveats".to_owned(),
            });
        }
    }
    // A reflog-only recovery must preserve its caveats rather than silently
    // pretending to be a full checkpoint.
    if sheet.reflog_only_fallback && sheet.restore_caveats.is_empty() {
        errors.push(StashRecoveryValidationError::MissingReflogCaveat {
            sheet_id: sheet_id.to_owned(),
        });
    }
    // An expiring anchor must say so in its caveats.
    if sheet.anchor_expiry_state == "expiring_soon"
        && !sheet
            .restore_caveats
            .iter()
            .any(|caveat| caveat == "anchor_expiring_soon")
    {
        errors.push(StashRecoveryValidationError::MissingExpiryCaveat {
            sheet_id: sheet_id.to_owned(),
        });
    }
}

fn validate_local_actions(
    sheet: &StashRecoverySheet,
    sheet_id: &str,
    errors: &mut Vec<StashRecoveryValidationError>,
) {
    let mut seen: HashSet<&str> = HashSet::new();
    for action in &sheet.local_actions {
        if !STASH_RECOVERY_LOCAL_ACTIONS.contains(&action.as_str()) {
            errors.push(StashRecoveryValidationError::FieldOutOfVocabulary {
                sheet_id: sheet_id.to_owned(),
                field: "local_actions".to_owned(),
            });
        }
        seen.insert(action.as_str());
    }
    // Local preview and abort truth must always be reachable, even offline.
    for required in STASH_RECOVERY_REQUIRED_LOCAL_ACTIONS {
        if !seen.contains(required) {
            errors.push(StashRecoveryValidationError::MissingLocalAction {
                sheet_id: sheet_id.to_owned(),
                action: required.to_owned(),
            });
        }
    }
    // When a recovery path exists, restore must be offered as a local action.
    if sheet.recovery_visible() && !seen.contains("restore_checkpoint") {
        errors.push(StashRecoveryValidationError::MissingLocalAction {
            sheet_id: sheet_id.to_owned(),
            action: "restore_checkpoint".to_owned(),
        });
    }
    // A stash verb must keep its entry inspectable.
    if sheet.verb.is_stash_verb() && !seen.contains("inspect_stash") {
        errors.push(StashRecoveryValidationError::MissingLocalAction {
            sheet_id: sheet_id.to_owned(),
            action: "inspect_stash".to_owned(),
        });
    }
    // A recovery verb's restore surface must keep compare and open-diff reachable.
    if sheet.verb.is_recovery_verb() {
        for required in ["compare", "open_diff"] {
            if !seen.contains(required) {
                errors.push(StashRecoveryValidationError::MissingLocalAction {
                    sheet_id: sheet_id.to_owned(),
                    action: required.to_owned(),
                });
            }
        }
    }
}

fn validate_decision(
    sheet: &StashRecoverySheet,
    sheet_id: &str,
    errors: &mut Vec<StashRecoveryValidationError>,
) {
    // The stored decision must equal the deterministic derivation; this is what
    // proves the sheet can explain why a verb was allowed/blocked/downgraded.
    let derived = sheet.derive_decision();
    if sheet.decision != derived {
        errors.push(StashRecoveryValidationError::DecisionDoesNotMatchGates {
            sheet_id: sheet_id.to_owned(),
        });
    }

    if !STASH_RECOVERY_DECISION_REASONS.contains(&sheet.decision.primary_reason.as_str()) {
        errors.push(StashRecoveryValidationError::FieldOutOfVocabulary {
            sheet_id: sheet_id.to_owned(),
            field: "decision.primary_reason".to_owned(),
        });
    }

    // A verb is never allowed without a visible recovery path.
    if sheet.decision.outcome == StashRecoveryOutcome::Allowed && !sheet.decision.recovery_visible {
        errors.push(StashRecoveryValidationError::AllowedWithoutRecovery {
            sheet_id: sheet_id.to_owned(),
        });
    }

    // A provider outage must never block local truth: even when the overlay is
    // unavailable, local preview/continue/abort/inspect/restore stays available.
    if sheet.provider_overlay_state == "overlay_unavailable_local_only"
        && sheet.decision.outcome == StashRecoveryOutcome::Blocked
        && sheet
            .decision
            .contributing_gates
            .iter()
            .all(|gate| gate == "provider_overlay")
    {
        errors.push(
            StashRecoveryValidationError::ProviderOutageBlocksLocalTruth {
                sheet_id: sheet_id.to_owned(),
            },
        );
    }
    if !sheet.decision.local_truth_available_offline && sheet.recovery_visible() {
        errors.push(
            StashRecoveryValidationError::ProviderOutageBlocksLocalTruth {
                sheet_id: sheet_id.to_owned(),
            },
        );
    }
}

fn validate_support_export(
    packet: &StashRecoveryPacket,
    sheet_ids: &HashSet<&str>,
    errors: &mut Vec<StashRecoveryValidationError>,
) {
    let export = &packet.support_export;
    if export.record_kind != STASH_RECOVERY_SUPPORT_EXPORT_RECORD_KIND {
        errors.push(StashRecoveryValidationError::WrongRecordKind {
            observed: export.record_kind.clone(),
        });
    }
    for sheet_ref in &export.sheet_refs {
        if !sheet_ids.contains(sheet_ref.as_str()) {
            errors.push(StashRecoveryValidationError::UnknownSupportSheetRef {
                sheet_ref: sheet_ref.clone(),
            });
        }
    }
    // Every sheet must be reconstructable from the support export.
    for sheet in &packet.sheets {
        if !export
            .sheet_refs
            .iter()
            .any(|reference| reference == &sheet.sheet_id)
        {
            errors.push(StashRecoveryValidationError::SupportExportMissingSheet {
                sheet_id: sheet.sheet_id.clone(),
            });
        }
    }
    for required in STASH_RECOVERY_REQUIRED_RECONSTRUCTION_FIELDS {
        if !export
            .reconstruction_fields
            .iter()
            .any(|field| field == required)
        {
            errors.push(StashRecoveryValidationError::SupportExportMissingField {
                field: required.to_string(),
            });
        }
    }
    if !export.raw_paths_redacted
        || !export.raw_patch_bodies_redacted
        || !export.raw_provider_payloads_redacted
    {
        errors.push(StashRecoveryValidationError::SupportExportEmbedsRawMaterial);
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

/// Error returned while parsing a stash/recovery packet.
#[derive(Debug)]
pub enum StashRecoveryError {
    /// JSON parsing failed.
    Json(serde_json::Error),
    /// Cross-row validation failed.
    Validation(Vec<StashRecoveryValidationError>),
}

impl fmt::Display for StashRecoveryError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Json(error) => {
                write!(
                    formatter,
                    "failed to parse stash recovery packet JSON: {error}"
                )
            }
            Self::Validation(errors) => {
                write!(formatter, "stash recovery packet has validation errors: ")?;
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

impl Error for StashRecoveryError {}

/// Cross-row validation error for a stash/recovery packet.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StashRecoveryValidationError {
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
    /// A stash verb names no exact stash entry.
    MissingStashEntry {
        /// Sheet id.
        sheet_id: String,
    },
    /// A create-branch verb names no new branch ref.
    MissingNewBranch {
        /// Sheet id.
        sheet_id: String,
    },
    /// A recovery verb carries no reflog/checkpoint anchor.
    MissingRecoveryAnchor {
        /// Sheet id.
        sheet_id: String,
    },
    /// A recovery anchor's kind does not match its restore verb.
    AnchorKindMismatch {
        /// Sheet id.
        sheet_id: String,
    },
    /// A recovery anchor omits its compare or open-diff action.
    AnchorMissingCompareActions {
        /// Sheet id.
        sheet_id: String,
    },
    /// A reflog-only recovery preserves no caveats.
    MissingReflogCaveat {
        /// Sheet id.
        sheet_id: String,
    },
    /// An expiring anchor omits its expiry caveat.
    MissingExpiryCaveat {
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
    /// A provider outage blocks local preview/continue/abort/inspect/restore truth.
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

impl fmt::Display for StashRecoveryValidationError {
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
            Self::MissingStashEntry { sheet_id } => {
                write!(
                    formatter,
                    "stash sheet {sheet_id} names no exact stash entry"
                )
            }
            Self::MissingNewBranch { sheet_id } => {
                write!(
                    formatter,
                    "create-branch sheet {sheet_id} names no new branch ref"
                )
            }
            Self::MissingRecoveryAnchor { sheet_id } => {
                write!(formatter, "recovery sheet {sheet_id} carries no anchor")
            }
            Self::AnchorKindMismatch { sheet_id } => write!(
                formatter,
                "sheet {sheet_id} anchor kind does not match its restore verb"
            ),
            Self::AnchorMissingCompareActions { sheet_id } => write!(
                formatter,
                "sheet {sheet_id} anchor omits its compare/open-diff actions"
            ),
            Self::MissingReflogCaveat { sheet_id } => write!(
                formatter,
                "reflog-only sheet {sheet_id} preserves no restore caveats"
            ),
            Self::MissingExpiryCaveat { sheet_id } => write!(
                formatter,
                "expiring-anchor sheet {sheet_id} omits its expiry caveat"
            ),
            Self::VerbFieldNotApplicable { sheet_id, field } => write!(
                formatter,
                "sheet {sheet_id} sets {field} on a verb that does not use it"
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
