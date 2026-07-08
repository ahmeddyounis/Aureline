//! Rebase todo rows and sequence-editor headers with original order, commit
//! identity, resulting operation, unresolved blockers, and checkpoint state
//! across claimed M5 sequence-edit surfaces.
//!
//! This module narrows the two **sequence-edit** components frozen in
//! [`crate::freeze_the_m5_git_history_sequence_component_matrix`] —
//! `rebase_todo_row` and `sequence_editor_header` — into an implemented,
//! export-safe row contract. A [`RebaseTodoRow`] answers, from itself alone,
//! which commit it operates on (short id, subject, author), which rebase
//! operation it applies (pick, reword, edit, squash, fixup, or drop), where the
//! commit sat in the **original** sequence versus where it sits now, whether it
//! is reordered / squashed / dropped, which blockers are still unresolved, and
//! whether a recovery checkpoint is available. A [`SequenceEditorHeaderRow`]
//! frames the whole interactive-rebase session: its onto/base ref, the original
//! pre-rebase tip that anchors recovery, how many commits were reordered /
//! squashed / dropped, and that the session confirms as a full sequence-rewrite
//! rather than one ambiguous confirm.
//!
//! Two honesty axes anchor the acceptance criteria. First, a sequence plan can
//! be reviewed and modified without losing original order/identity context: the
//! claimed plan state is always derivable from the operation and the
//! original/display positions (`PlanStateMisrepresented`), the commit identity is
//! always explicit (`CommitIdentityMissing`), and the header always names the
//! original tip that anchors recovery and how original order is preserved
//! (`OriginalTipRecoveryAnchorMissing`, `OriginalOrderContextMissing`). Second,
//! the raw-todo fallback and the structured-card view stay meaning-equivalent:
//! each row's raw `pick <sha> <subject>`-style line must agree with its
//! structured operation and commit (`RawTodoLineMisaligned`), so the same
//! sequence never appears to mean different things across desktop, CLI/help, and
//! export surfaces.
//!
//! The shared downgrade vocabulary ([`GitHistoryDowngradeState`]), the shared
//! consumer surfaces ([`ComponentConsumerSurface`]), and the shared
//! mutation-review class ([`MutationReviewClass`]) are reused directly from the
//! frozen matrix so downgrades, parity, and the sequence-rewrite confirm read the
//! same everywhere. Local-only recovery stays explicit even when a
//! provider-linked review state also exists. Raw patch bytes, raw object bodies,
//! raw provider payloads, and credentials stay outside the support boundary.
//!
//! The boundary schema is
//! [`schemas/ui/m5-rebase-todo-sequence-editor-component.schema.json`](../../../../schemas/ui/m5-rebase-todo-sequence-editor-component.schema.json).
//! The contract doc is
//! [`docs/git/m5/implement_rebase_todo_rows_and_sequence_editor_headers.md`](../../../../docs/git/m5/implement_rebase_todo_rows_and_sequence_editor_headers.md).
//! The protected fixture directory is
//! [`fixtures/ui/m5-rebase-todo-sequence-editor-components/`](../../../../fixtures/ui/m5-rebase-todo-sequence-editor-components/).

#[cfg(test)]
mod tests;

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

use crate::freeze_the_m5_git_history_sequence_component_matrix::{
    ComponentConsumerSurface, GitHistoryDowngradeState, M5GitHistoryComponent, MutationReviewClass,
};

/// Stable record-kind tag carried by [`RebaseSequenceEditPacket`].
pub const REBASE_SEQUENCE_RECORD_KIND: &str = "git_rebase_sequence_edit_component_truth";

/// Schema version for rebase-todo / sequence-editor component records.
pub const REBASE_SEQUENCE_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the boundary schema.
pub const REBASE_SEQUENCE_SCHEMA_REF: &str =
    "schemas/ui/m5-rebase-todo-sequence-editor-component.schema.json";

/// Repo-relative path of the contract doc.
pub const REBASE_SEQUENCE_DOC_REF: &str =
    "docs/git/m5/implement_rebase_todo_rows_and_sequence_editor_headers.md";

/// Repo-relative path of the frozen component matrix this lane implements.
pub const REBASE_SEQUENCE_COMPONENT_MATRIX_CONTRACT_REF: &str =
    "schemas/ui/m5-git-history-sequence-component-matrix.schema.json";

/// Repo-relative path of the canonical sequence-edit-session contract.
pub const REBASE_SEQUENCE_SEQUENCE_EDIT_CONTRACT_REF: &str =
    "schemas/git/sequence_edit_session.schema.json";

/// Repo-relative path of the canonical recovery-checkpoint contract.
pub const REBASE_SEQUENCE_CHECKPOINT_CONTRACT_REF: &str =
    "schemas/git/recovery_checkpoint.schema.json";

/// Repo-relative path of the canonical sequence-edit review/ref-update contract.
pub const REBASE_SEQUENCE_REVIEW_CONTRACT_REF: &str =
    "schemas/review/sequence-edit-conflict-session-stash-entry-ref-update.schema.json";

/// Repo-relative path of the protected fixture directory.
pub const REBASE_SEQUENCE_FIXTURE_DIR: &str =
    "fixtures/ui/m5-rebase-todo-sequence-editor-components";

/// Repo-relative path of the checked support-export artifact.
pub const REBASE_SEQUENCE_ARTIFACT_REF: &str =
    "artifacts/release/m5-rebase-todo-sequence-editor-components-proof/support_export.json";

/// Repo-relative path of the checked Markdown summary.
pub const REBASE_SEQUENCE_SUMMARY_REF: &str =
    "artifacts/release/m5-rebase-todo-sequence-editor-components-proof/summary.md";

/// The two sequence-edit components this lane implements.
///
/// These are exactly the sequence-edit pair of the frozen matrix: a rebase todo
/// row for one commit, and the header framing the whole sequence-editor session.
pub const REBASE_SEQUENCE_COMPONENTS: [M5GitHistoryComponent; 2] = [
    M5GitHistoryComponent::RebaseTodoRow,
    M5GitHistoryComponent::SequenceEditorHeader,
];

/// The rebase operation a single todo row applies to its commit.
///
/// These verbs are never collapsed into one ambiguous confirm: pick keeps the
/// commit as-is, reword rewrites only its message, edit stops to amend it, squash
/// and fixup fold it into the previous commit (squash keeps the message, fixup
/// discards it), and drop removes it from history.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SequenceOperation {
    /// Keep the commit exactly as it is.
    Pick,
    /// Keep the commit but rewrite its message.
    Reword,
    /// Stop at the commit to amend its content.
    Edit,
    /// Fold the commit into the previous one, keeping a combined message.
    Squash,
    /// Fold the commit into the previous one, discarding its message.
    Fixup,
    /// Remove the commit from the rewritten history.
    Drop,
}

impl SequenceOperation {
    /// Every operation, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::Pick,
        Self::Reword,
        Self::Edit,
        Self::Squash,
        Self::Fixup,
        Self::Drop,
    ];

    /// Stable token recorded in the packet; equal to the raw git-rebase-todo verb.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Pick => "pick",
            Self::Reword => "reword",
            Self::Edit => "edit",
            Self::Squash => "squash",
            Self::Fixup => "fixup",
            Self::Drop => "drop",
        }
    }

    /// The raw git-rebase-todo verb token this operation writes on a todo line.
    pub const fn todo_token(self) -> &'static str {
        self.as_str()
    }

    /// Whether this operation folds the commit into the previous one.
    pub const fn combines_with_previous(self) -> bool {
        matches!(self, Self::Squash | Self::Fixup)
    }

    /// Whether this operation removes the commit from history.
    pub const fn removes_commit(self) -> bool {
        matches!(self, Self::Drop)
    }

    /// Whether this operation rewrites or removes history (everything but `pick`),
    /// and so must keep a recovery checkpoint reachable.
    pub const fn rewrites_or_removes(self) -> bool {
        !matches!(self, Self::Pick)
    }
}

/// How a todo row's commit differs from its place in the **original** sequence.
///
/// This is derived from the operation and the original/display positions, never
/// asserted independently, so a plan can be modified without ever losing the
/// original-order context.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TodoPlanState {
    /// Same operation-neutral position as the original sequence.
    Unchanged,
    /// Kept, but moved to a different position than the original sequence.
    Reordered,
    /// Folded into the previous commit (squash/fixup).
    SquashedIntoPrevious,
    /// Removed from the rewritten history (drop).
    Dropped,
}

impl TodoPlanState {
    /// Every plan state, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::Unchanged,
        Self::Reordered,
        Self::SquashedIntoPrevious,
        Self::Dropped,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Unchanged => "unchanged",
            Self::Reordered => "reordered",
            Self::SquashedIntoPrevious => "squashed_into_previous",
            Self::Dropped => "dropped",
        }
    }

    /// Whether this row differs from its original-sequence placement.
    pub const fn differs_from_original(self) -> bool {
        !matches!(self, Self::Unchanged)
    }

    /// Whether this state removes the commit from the rewritten history.
    pub const fn is_destructive(self) -> bool {
        matches!(self, Self::Dropped)
    }
}

/// Derives the plan state a todo row must claim from its operation and its
/// original versus display position.
///
/// Drop takes precedence (the commit is gone), then squash/fixup (folded into the
/// previous commit), then a changed position (reordered), otherwise unchanged.
/// Because the claimed `plan_state` is checked against this, a structured card can
/// never quietly misrepresent whether a commit moved, folded, or vanished.
pub fn resolve_todo_plan_state(
    operation: SequenceOperation,
    original_index: u32,
    display_index: u32,
) -> TodoPlanState {
    if operation.removes_commit() {
        TodoPlanState::Dropped
    } else if operation.combines_with_previous() {
        TodoPlanState::SquashedIntoPrevious
    } else if original_index != display_index {
        TodoPlanState::Reordered
    } else {
        TodoPlanState::Unchanged
    }
}

/// An unresolved blocker that stops a sequence step from applying cleanly.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SequenceBlockerKind {
    /// A merge conflict must be resolved at this step before it can continue.
    ConflictAtStep,
    /// A squash needs a combined message before it can be written.
    MissingSquashMessage,
    /// A fixup targets a commit that was dropped or reordered away.
    OrphanedFixupTarget,
    /// The step would produce an empty commit that must be kept or dropped.
    EmptyCommitAfterEdit,
    /// The rewritten commit must be signed before the step can complete.
    SignatureRequired,
}

impl SequenceBlockerKind {
    /// Every blocker kind, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::ConflictAtStep,
        Self::MissingSquashMessage,
        Self::OrphanedFixupTarget,
        Self::EmptyCommitAfterEdit,
        Self::SignatureRequired,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ConflictAtStep => "conflict_at_step",
            Self::MissingSquashMessage => "missing_squash_message",
            Self::OrphanedFixupTarget => "orphaned_fixup_target",
            Self::EmptyCommitAfterEdit => "empty_commit_after_edit",
            Self::SignatureRequired => "signature_required",
        }
    }
}

/// Availability of a recovery checkpoint for a sequence step or session.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SequenceCheckpointState {
    /// A pre-sequence checkpoint was captured and is reachable.
    Captured,
    /// A checkpoint is captured before each applied step.
    PerStepCaptured,
    /// No explicit checkpoint; only a reflog-based recovery fallback exists.
    ReflogFallbackOnly,
    /// No checkpoint and no recovery is available yet.
    Unavailable,
}

impl SequenceCheckpointState {
    /// Every checkpoint state, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::Captured,
        Self::PerStepCaptured,
        Self::ReflogFallbackOnly,
        Self::Unavailable,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Captured => "captured",
            Self::PerStepCaptured => "per_step_captured",
            Self::ReflogFallbackOnly => "reflog_fallback_only",
            Self::Unavailable => "unavailable",
        }
    }

    /// Whether the step/session can be recovered at all.
    pub const fn is_recoverable(self) -> bool {
        !matches!(self, Self::Unavailable)
    }

    /// Whether an explicit checkpoint (not just a reflog fallback) exists.
    pub const fn has_explicit_checkpoint(self) -> bool {
        matches!(self, Self::Captured | Self::PerStepCaptured)
    }
}

/// Recovery disclosures a sequence step must carry, derived from its operation
/// and checkpoint state.
///
/// A history-rewriting operation must stay recoverable; every step discloses its
/// checkpoint state; a step is only genuinely recoverable when its checkpoint is
/// reachable.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SequenceStepRecovery {
    /// Whether the step's operation rewrites/removes history and so must recover.
    pub must_be_recoverable: bool,
    /// Whether the step must disclose its checkpoint state (always true).
    pub must_disclose_checkpoint: bool,
    /// Whether the step is genuinely still recoverable.
    pub is_recoverable: bool,
}

/// Resolves the recovery disclosures a sequence step must carry from its
/// operation and checkpoint state.
///
/// Any operation other than a plain `pick` rewrites or removes history, so a
/// recovery checkpoint must stay reachable; the checkpoint state is always
/// disclosed; and the step is only genuinely recoverable when its checkpoint has
/// not become unavailable.
pub fn resolve_sequence_step_recovery(
    operation: SequenceOperation,
    checkpoint: SequenceCheckpointState,
) -> SequenceStepRecovery {
    SequenceStepRecovery {
        must_be_recoverable: operation.rewrites_or_removes(),
        must_disclose_checkpoint: true,
        is_recoverable: checkpoint.is_recoverable(),
    }
}

/// One rebase todo row: a single commit's line in an interactive-rebase plan.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RebaseTodoRow {
    /// Stable row id.
    pub row_id: String,
    /// Canonical component this row implements; must be `rebase_todo_row`.
    pub component: M5GitHistoryComponent,
    /// Zero-based position of the commit in the **original** sequence.
    pub original_index: u32,
    /// Zero-based position of the commit in the current edited plan.
    pub display_index: u32,
    /// Short commit id (for example `a1b2c3d`); part of the explicit identity.
    pub commit_short_id: String,
    /// Commit subject line; part of the explicit identity.
    pub commit_subject: String,
    /// Commit author identity; part of the explicit identity.
    pub commit_author: String,
    /// The rebase operation this row applies.
    pub operation: SequenceOperation,
    /// How the row differs from its original-sequence placement (derived-checked).
    pub plan_state: TodoPlanState,
    /// Blockers still unresolved at this step, in display order.
    pub unresolved_blockers: Vec<SequenceBlockerKind>,
    /// Human-readable disclosure of the unresolved blockers.
    pub blocker_disclosure: String,
    /// Checkpoint availability for this step.
    pub checkpoint_state: SequenceCheckpointState,
    /// Human-readable disclosure of the checkpoint state.
    pub checkpoint_disclosure: String,
    /// The raw git-rebase-todo line this row structures (raw-fallback parity).
    pub raw_todo_line: String,
    /// Shared downgrade states this component may surface.
    pub downgrade_vocab: Vec<GitHistoryDowngradeState>,
    /// Component fields the surface projects, in display order.
    pub fields_shown: Vec<String>,
    /// Source contract refs consumed by this component.
    pub source_contract_refs: Vec<String>,
}

impl RebaseTodoRow {
    /// The plan state this row's operation and positions imply.
    pub fn derived_plan_state(&self) -> TodoPlanState {
        resolve_todo_plan_state(self.operation, self.original_index, self.display_index)
    }

    /// The recovery disclosures this row's step must carry.
    pub fn recovery(&self) -> SequenceStepRecovery {
        resolve_sequence_step_recovery(self.operation, self.checkpoint_state)
    }

    /// Whether the commit identity is fully explicit (id, subject, and author).
    pub fn commit_identity_explicit(&self) -> bool {
        !self.commit_short_id.trim().is_empty()
            && !self.commit_subject.trim().is_empty()
            && !self.commit_author.trim().is_empty()
    }

    /// Whether the raw todo line agrees with the structured operation and commit.
    ///
    /// This is the raw-fallback / structured-card meaning-equivalence check: the
    /// raw line must lead with the operation's verb token and name the same
    /// commit, so the same sequence never means two different things.
    pub fn raw_and_structured_agree(&self) -> bool {
        let raw = self.raw_todo_line.trim();
        let mut tokens = raw.split_whitespace();
        let verb = tokens.next().unwrap_or("");
        verb.eq_ignore_ascii_case(self.operation.todo_token())
            && raw
                .to_ascii_lowercase()
                .contains(&self.commit_short_id.trim().to_ascii_lowercase())
            && !self.commit_short_id.trim().is_empty()
    }
}

/// One sequence-editor header: framing for an interactive-rebase session.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SequenceEditorHeaderRow {
    /// Stable row id.
    pub row_id: String,
    /// Canonical component this row implements; must be `sequence_editor_header`.
    pub component: M5GitHistoryComponent,
    /// Human-readable session label.
    pub session_label: String,
    /// The onto/base ref the sequence rebases onto (for example `main`).
    pub onto_ref: String,
    /// The original pre-rebase tip that anchors recovery (for example `feature@{1}`).
    pub original_tip_ref: String,
    /// Total commits in the original sequence.
    pub total_commits: u32,
    /// How many commits are reordered relative to the original sequence.
    pub reordered_count: u32,
    /// How many commits are squashed/fixed up into a previous commit.
    pub squashed_count: u32,
    /// How many commits are dropped from the rewritten history.
    pub dropped_count: u32,
    /// How many steps still carry an unresolved blocker.
    pub unresolved_blocker_count: u32,
    /// Checkpoint availability for the whole session.
    pub checkpoint_state: SequenceCheckpointState,
    /// Human-readable disclosure of the checkpoint state.
    pub checkpoint_disclosure: String,
    /// The mutation-review class the session confirms as (never collapsed).
    pub review_class: MutationReviewClass,
    /// How the original order is preserved and shown while editing.
    pub original_order_note: String,
    /// Shared downgrade states this component may surface.
    pub downgrade_vocab: Vec<GitHistoryDowngradeState>,
    /// Component fields the surface projects, in display order.
    pub fields_shown: Vec<String>,
    /// Source contract refs consumed by this component.
    pub source_contract_refs: Vec<String>,
}

impl SequenceEditorHeaderRow {
    /// The recovery disclosures the whole session must carry.
    ///
    /// A session that reorders, squashes, or drops anything rewrites history and
    /// so must stay recoverable; a purely `unchanged` review still discloses its
    /// checkpoint state.
    pub fn recovery(&self) -> SequenceStepRecovery {
        let rewrites =
            self.reordered_count > 0 || self.squashed_count > 0 || self.dropped_count > 0;
        SequenceStepRecovery {
            must_be_recoverable: rewrites,
            must_disclose_checkpoint: true,
            is_recoverable: self.checkpoint_state.is_recoverable(),
        }
    }

    /// Whether the reordered/squashed/dropped counts stay within the total.
    pub fn counts_are_consistent(&self) -> bool {
        self.reordered_count <= self.total_commits
            && self.squashed_count <= self.total_commits
            && self.dropped_count <= self.total_commits
            && self.unresolved_blocker_count <= self.total_commits
            && self.squashed_count + self.dropped_count <= self.total_commits
    }
}

/// Trust and provenance review block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RebaseSequenceTrustReview {
    /// Every todo row names its commit identity (id, subject, author).
    pub commit_identity_always_explicit: bool,
    /// The original-sequence order/position is always preserved.
    pub original_order_always_preserved: bool,
    /// The claimed plan state always matches the derived plan state.
    pub plan_state_never_misrepresented: bool,
    /// Rebase operations stay distinct, never collapsed into one confirm.
    pub operations_stay_distinct: bool,
    /// Unresolved blockers are always disclosed, never hidden.
    pub unresolved_blockers_always_disclosed: bool,
    /// A checkpoint stays reachable after a history-rewriting step.
    pub checkpoint_reachable_after_rewrite: bool,
    /// The raw-todo fallback and structured card stay meaning-equivalent.
    pub raw_and_structured_meaning_equivalent: bool,
    /// The session confirms as a full sequence-rewrite, not one ambiguous confirm.
    pub sequence_rewrite_confirm_never_collapsed: bool,
    /// Local-only recovery stays explicit even with provider review state.
    pub local_only_recovery_stays_explicit: bool,
    /// One component contract is reused with no hidden per-surface meaning.
    pub one_component_contract_no_hidden_meaning: bool,
    /// Downgrade narrows the claim rather than hiding the component.
    pub downgrade_narrows_instead_of_hides: bool,
    /// Stale or underqualified components automatically block promotion.
    pub stale_or_underqualified_blocks_promotion: bool,
}

impl RebaseSequenceTrustReview {
    /// Whether every invariant holds.
    pub const fn all_hold(&self) -> bool {
        self.commit_identity_always_explicit
            && self.original_order_always_preserved
            && self.plan_state_never_misrepresented
            && self.operations_stay_distinct
            && self.unresolved_blockers_always_disclosed
            && self.checkpoint_reachable_after_rewrite
            && self.raw_and_structured_meaning_equivalent
            && self.sequence_rewrite_confirm_never_collapsed
            && self.local_only_recovery_stays_explicit
            && self.one_component_contract_no_hidden_meaning
            && self.downgrade_narrows_instead_of_hides
            && self.stale_or_underqualified_blocks_promotion
    }
}

/// Consumer projection block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RebaseSequenceConsumerProjection {
    /// Git-history surfaces reuse one component contract.
    pub git_history_reuses_one_contract: bool,
    /// Review surfaces reuse one component contract.
    pub review_reuses_one_contract: bool,
    /// Help/support surfaces reuse one component contract.
    pub help_support_reuses_one_contract: bool,
    /// Support/export surfaces reuse one component contract.
    pub support_export_reuses_one_contract: bool,
    /// The raw-todo fallback stays meaning-equivalent across surfaces.
    pub raw_fallback_equivalent_across_surfaces: bool,
    /// CLI / headless shows component truth.
    pub cli_headless_shows_truth: bool,
    /// Provider overlay shows component truth without overwriting local truth.
    pub provider_overlay_shows_truth: bool,
    /// AI-context assembly shows component truth.
    pub ai_context_shows_truth: bool,
}

impl RebaseSequenceConsumerProjection {
    /// Whether every projection invariant holds.
    pub const fn all_hold(&self) -> bool {
        self.git_history_reuses_one_contract
            && self.review_reuses_one_contract
            && self.help_support_reuses_one_contract
            && self.support_export_reuses_one_contract
            && self.raw_fallback_equivalent_across_surfaces
            && self.cli_headless_shows_truth
            && self.provider_overlay_shows_truth
            && self.ai_context_shows_truth
    }
}

/// Proof freshness block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RebaseSequenceProofFreshness {
    /// Proof-freshness SLO in hours.
    pub proof_freshness_slo_hours: u32,
    /// RFC 3339 timestamp of the last proof refresh.
    pub last_proof_refresh: String,
    /// True when stale proof automatically narrows the lane.
    pub auto_narrow_on_stale: bool,
}

/// Constructor input for [`RebaseSequenceEditPacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RebaseSequenceEditPacketInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable surface label.
    pub surface_label: String,
    /// Rebase todo rows.
    pub todo_rows: Vec<RebaseTodoRow>,
    /// Sequence-editor header rows.
    pub sequence_headers: Vec<SequenceEditorHeaderRow>,
    /// Shared downgrade states that apply to this lane.
    pub downgrade_triggers: Vec<GitHistoryDowngradeState>,
    /// Consumer surfaces that must reuse this component contract.
    pub consumer_surfaces: Vec<ComponentConsumerSurface>,
    /// Trust review block.
    pub trust_review: RebaseSequenceTrustReview,
    /// Consumer projection block.
    pub consumer_projection: RebaseSequenceConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: RebaseSequenceProofFreshness,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

/// Export-safe rebase-todo / sequence-editor component packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RebaseSequenceEditPacket {
    /// Record kind; must equal [`REBASE_SEQUENCE_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`REBASE_SEQUENCE_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable surface label.
    pub surface_label: String,
    /// Rebase todo rows.
    pub todo_rows: Vec<RebaseTodoRow>,
    /// Sequence-editor header rows.
    pub sequence_headers: Vec<SequenceEditorHeaderRow>,
    /// Shared downgrade states that apply to this lane.
    pub downgrade_triggers: Vec<GitHistoryDowngradeState>,
    /// Consumer surfaces that must reuse this component contract.
    pub consumer_surfaces: Vec<ComponentConsumerSurface>,
    /// Trust review block.
    pub trust_review: RebaseSequenceTrustReview,
    /// Consumer projection block.
    pub consumer_projection: RebaseSequenceConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: RebaseSequenceProofFreshness,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

impl RebaseSequenceEditPacket {
    /// Builds a rebase-todo / sequence-editor component packet from stable-lane input.
    pub fn new(input: RebaseSequenceEditPacketInput) -> Self {
        Self {
            record_kind: REBASE_SEQUENCE_RECORD_KIND.to_owned(),
            schema_version: REBASE_SEQUENCE_SCHEMA_VERSION,
            packet_id: input.packet_id,
            surface_label: input.surface_label,
            todo_rows: input.todo_rows,
            sequence_headers: input.sequence_headers,
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

    /// Validates the rebase-todo / sequence-editor invariants.
    ///
    /// Returns every violation found rather than stopping at the first, so a
    /// regenerator or CI gate can report the full set at once.
    pub fn validate(&self) -> Vec<RebaseSequenceViolation> {
        let mut violations = Vec::new();

        if self.record_kind != REBASE_SEQUENCE_RECORD_KIND {
            violations.push(RebaseSequenceViolation::WrongRecordKind);
        }
        if self.schema_version != REBASE_SEQUENCE_SCHEMA_VERSION {
            violations.push(RebaseSequenceViolation::WrongSchemaVersion);
        }
        if self.packet_id.trim().is_empty()
            || self.surface_label.trim().is_empty()
            || self.redaction_class_token.trim().is_empty()
            || self.minted_at.trim().is_empty()
        {
            violations.push(RebaseSequenceViolation::MissingIdentity);
        }
        if self.downgrade_triggers.is_empty() {
            violations.push(RebaseSequenceViolation::DowngradeTriggersMissing);
        }
        if self.consumer_surfaces.is_empty() {
            violations.push(RebaseSequenceViolation::ConsumerSurfacesMissing);
        }

        validate_source_contracts(self, &mut violations);
        validate_todo_rows(self, &mut violations);
        validate_sequence_headers(self, &mut violations);

        if !self.trust_review.all_hold() {
            violations.push(RebaseSequenceViolation::TrustReviewIncomplete);
        }
        if !self.consumer_projection.all_hold() {
            violations.push(RebaseSequenceViolation::ConsumerProjectionIncomplete);
        }
        if self.proof_freshness.proof_freshness_slo_hours == 0
            || self.proof_freshness.last_proof_refresh.trim().is_empty()
        {
            violations.push(RebaseSequenceViolation::ProofFreshnessIncomplete);
        }

        if json_contains_forbidden_boundary_material(
            &serde_json::to_value(self).expect("rebase sequence packet serializes"),
        ) {
            violations.push(RebaseSequenceViolation::RawBoundaryMaterialInExport);
        }

        violations
    }

    /// Deterministic export-safe pretty JSON.
    ///
    /// # Panics
    ///
    /// Panics only if serializing this metadata-only packet fails.
    pub fn export_safe_json(&self) -> String {
        serde_json::to_string_pretty(self).expect("rebase sequence packet serializes")
    }

    /// Deterministic Markdown summary for support, review, or release handoff.
    pub fn render_markdown_summary(&self) -> String {
        let dropped = self
            .todo_rows
            .iter()
            .filter(|row| row.plan_state.is_destructive())
            .count();
        let blocked = self
            .todo_rows
            .iter()
            .filter(|row| !row.unresolved_blockers.is_empty())
            .count();

        let mut out = String::new();
        out.push_str(
            "# Rebase Todo Rows and Sequence-Editor Headers: Ordered-Plan and Checkpoint Truth\n\n",
        );
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- Surface: `{}`\n", self.surface_label));
        out.push_str(&format!(
            "- Todo rows: {} ({} dropped, {} with unresolved blockers); sequence headers: {}\n",
            self.todo_rows.len(),
            dropped,
            blocked,
            self.sequence_headers.len()
        ));
        out.push_str(&format!(
            "- Proof freshness SLO: {} hours (last refresh: {})\n",
            self.proof_freshness.proof_freshness_slo_hours, self.proof_freshness.last_proof_refresh
        ));

        out.push_str("\n## Rebase todo rows\n\n");
        for row in &self.todo_rows {
            out.push_str(&format!(
                "- `{}` [orig #{} → #{}] `{}` \"{}\" — {} ({})\n",
                row.operation.as_str(),
                row.original_index,
                row.display_index,
                row.commit_short_id,
                row.commit_subject,
                row.plan_state.as_str(),
                row.checkpoint_state.as_str()
            ));
        }

        out.push_str("\n## Sequence-editor headers\n\n");
        for header in &self.sequence_headers {
            out.push_str(&format!(
                "- **{}** onto `{}` (recover from `{}`) — {} commits: {} reordered / {} squashed / {} dropped\n",
                header.session_label,
                header.onto_ref,
                header.original_tip_ref,
                header.total_commits,
                header.reordered_count,
                header.squashed_count,
                header.dropped_count
            ));
        }
        out
    }
}

/// Errors emitted when reading the checked-in rebase-todo / sequence-editor export.
#[derive(Debug)]
pub enum RebaseSequenceArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<RebaseSequenceViolation>),
}

impl fmt::Display for RebaseSequenceArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(formatter, "rebase sequence export parse failed: {error}")
            }
            Self::Validation(violations) => {
                let tokens = violations
                    .iter()
                    .map(|violation| violation.as_str())
                    .collect::<Vec<_>>()
                    .join(",");
                write!(
                    formatter,
                    "rebase sequence export failed validation: {tokens}"
                )
            }
        }
    }
}

impl Error for RebaseSequenceArtifactError {}

/// Validation failures emitted by [`RebaseSequenceEditPacket::validate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum RebaseSequenceViolation {
    /// Packet record kind is wrong.
    WrongRecordKind,
    /// Packet schema version is wrong.
    WrongSchemaVersion,
    /// A required identity field is missing.
    MissingIdentity,
    /// Source contract refs are incomplete.
    MissingSourceContracts,
    /// No rebase todo rows are present.
    RebaseTodoRowsMissing,
    /// No sequence-editor headers are present.
    SequenceEditorHeadersMissing,
    /// A rebase todo row is incomplete.
    TodoRowIncomplete,
    /// A todo row's component is not `rebase_todo_row`.
    WrongComponentForTodoRow,
    /// A todo row does not spell out its full commit identity.
    CommitIdentityMissing,
    /// A todo row's claimed plan state does not match the derived plan state.
    PlanStateMisrepresented,
    /// A todo row's raw todo line disagrees with its structured operation/commit.
    RawTodoLineMisaligned,
    /// A todo row has unresolved blockers but does not disclose them.
    UnresolvedBlockerNotDisclosed,
    /// A todo row does not disclose its checkpoint state.
    CheckpointStateUndisclosed,
    /// A history-rewriting todo row has no reachable recovery checkpoint.
    RecoveryCheckpointMissing,
    /// A sequence-editor header row is incomplete.
    HeaderRowIncomplete,
    /// A header row's component is not `sequence_editor_header`.
    WrongComponentForHeaderRow,
    /// A header does not name the onto/base ref.
    OntoRefMissing,
    /// A header does not name the original tip that anchors recovery.
    OriginalTipRecoveryAnchorMissing,
    /// A header does not describe how original order is preserved.
    OriginalOrderContextMissing,
    /// A header's reordered/squashed/dropped counts exceed the total.
    HeaderCountsInconsistent,
    /// A header collapses the sequence-rewrite confirm into another class.
    SequenceConfirmCollapsed,
    /// A history-rewriting header has no reachable recovery checkpoint.
    HeaderRecoveryCheckpointMissing,
    /// A header does not disclose its checkpoint state.
    HeaderCheckpointUndisclosed,
    /// No unchanged todo row remains to prove original order survives editing.
    OriginalOrderPreservationCoverageMissing,
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

impl RebaseSequenceViolation {
    /// Stable token used in tests and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WrongRecordKind => "wrong_record_kind",
            Self::WrongSchemaVersion => "wrong_schema_version",
            Self::MissingIdentity => "missing_identity",
            Self::MissingSourceContracts => "missing_source_contracts",
            Self::RebaseTodoRowsMissing => "rebase_todo_rows_missing",
            Self::SequenceEditorHeadersMissing => "sequence_editor_headers_missing",
            Self::TodoRowIncomplete => "todo_row_incomplete",
            Self::WrongComponentForTodoRow => "wrong_component_for_todo_row",
            Self::CommitIdentityMissing => "commit_identity_missing",
            Self::PlanStateMisrepresented => "plan_state_misrepresented",
            Self::RawTodoLineMisaligned => "raw_todo_line_misaligned",
            Self::UnresolvedBlockerNotDisclosed => "unresolved_blocker_not_disclosed",
            Self::CheckpointStateUndisclosed => "checkpoint_state_undisclosed",
            Self::RecoveryCheckpointMissing => "recovery_checkpoint_missing",
            Self::HeaderRowIncomplete => "header_row_incomplete",
            Self::WrongComponentForHeaderRow => "wrong_component_for_header_row",
            Self::OntoRefMissing => "onto_ref_missing",
            Self::OriginalTipRecoveryAnchorMissing => "original_tip_recovery_anchor_missing",
            Self::OriginalOrderContextMissing => "original_order_context_missing",
            Self::HeaderCountsInconsistent => "header_counts_inconsistent",
            Self::SequenceConfirmCollapsed => "sequence_confirm_collapsed",
            Self::HeaderRecoveryCheckpointMissing => "header_recovery_checkpoint_missing",
            Self::HeaderCheckpointUndisclosed => "header_checkpoint_undisclosed",
            Self::OriginalOrderPreservationCoverageMissing => {
                "original_order_preservation_coverage_missing"
            }
            Self::DowngradeTriggersMissing => "downgrade_triggers_missing",
            Self::ConsumerSurfacesMissing => "consumer_surfaces_missing",
            Self::TrustReviewIncomplete => "trust_review_incomplete",
            Self::ConsumerProjectionIncomplete => "consumer_projection_incomplete",
            Self::ProofFreshnessIncomplete => "proof_freshness_incomplete",
            Self::RawBoundaryMaterialInExport => "raw_boundary_material_in_export",
        }
    }
}

/// Reads and validates the checked-in stable rebase-todo / sequence-editor export.
///
/// # Errors
///
/// Returns [`RebaseSequenceArtifactError`] when the checked-in export fails to
/// parse or violates the contract.
pub fn current_rebase_sequence_edit_export(
) -> Result<RebaseSequenceEditPacket, RebaseSequenceArtifactError> {
    let packet: RebaseSequenceEditPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/release/m5-rebase-todo-sequence-editor-components-proof/support_export.json"
    )))
    .map_err(RebaseSequenceArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(RebaseSequenceArtifactError::Validation(violations))
    }
}

fn validate_source_contracts(
    packet: &RebaseSequenceEditPacket,
    violations: &mut Vec<RebaseSequenceViolation>,
) {
    let refs: BTreeSet<&str> = packet
        .source_contract_refs
        .iter()
        .map(String::as_str)
        .collect();
    for required in [
        REBASE_SEQUENCE_SCHEMA_REF,
        REBASE_SEQUENCE_DOC_REF,
        REBASE_SEQUENCE_COMPONENT_MATRIX_CONTRACT_REF,
        REBASE_SEQUENCE_SEQUENCE_EDIT_CONTRACT_REF,
        REBASE_SEQUENCE_CHECKPOINT_CONTRACT_REF,
        REBASE_SEQUENCE_REVIEW_CONTRACT_REF,
    ] {
        if !refs.contains(required) {
            violations.push(RebaseSequenceViolation::MissingSourceContracts);
            return;
        }
    }
}

fn validate_todo_rows(
    packet: &RebaseSequenceEditPacket,
    violations: &mut Vec<RebaseSequenceViolation>,
) {
    if packet.todo_rows.is_empty() {
        violations.push(RebaseSequenceViolation::RebaseTodoRowsMissing);
        return;
    }

    let mut any_unchanged = false;

    for row in &packet.todo_rows {
        if row.plan_state == TodoPlanState::Unchanged {
            any_unchanged = true;
        }

        if row.row_id.trim().is_empty()
            || row.raw_todo_line.trim().is_empty()
            || row.downgrade_vocab.is_empty()
            || row.fields_shown.is_empty()
            || row.source_contract_refs.is_empty()
        {
            violations.push(RebaseSequenceViolation::TodoRowIncomplete);
        }

        if row.component != M5GitHistoryComponent::RebaseTodoRow {
            violations.push(RebaseSequenceViolation::WrongComponentForTodoRow);
        }

        if !row.commit_identity_explicit() {
            violations.push(RebaseSequenceViolation::CommitIdentityMissing);
        }
        if row.plan_state != row.derived_plan_state() {
            violations.push(RebaseSequenceViolation::PlanStateMisrepresented);
        }
        if !row.raw_and_structured_agree() {
            violations.push(RebaseSequenceViolation::RawTodoLineMisaligned);
        }
        if !row.unresolved_blockers.is_empty() && row.blocker_disclosure.trim().is_empty() {
            violations.push(RebaseSequenceViolation::UnresolvedBlockerNotDisclosed);
        }
        if row.checkpoint_disclosure.trim().is_empty() {
            violations.push(RebaseSequenceViolation::CheckpointStateUndisclosed);
        }

        let recovery = row.recovery();
        if recovery.must_be_recoverable && !recovery.is_recoverable {
            violations.push(RebaseSequenceViolation::RecoveryCheckpointMissing);
        }
    }

    if !any_unchanged {
        violations.push(RebaseSequenceViolation::OriginalOrderPreservationCoverageMissing);
    }
}

fn validate_sequence_headers(
    packet: &RebaseSequenceEditPacket,
    violations: &mut Vec<RebaseSequenceViolation>,
) {
    if packet.sequence_headers.is_empty() {
        violations.push(RebaseSequenceViolation::SequenceEditorHeadersMissing);
        return;
    }

    for header in &packet.sequence_headers {
        if header.row_id.trim().is_empty()
            || header.session_label.trim().is_empty()
            || header.downgrade_vocab.is_empty()
            || header.fields_shown.is_empty()
            || header.source_contract_refs.is_empty()
        {
            violations.push(RebaseSequenceViolation::HeaderRowIncomplete);
        }

        if header.component != M5GitHistoryComponent::SequenceEditorHeader {
            violations.push(RebaseSequenceViolation::WrongComponentForHeaderRow);
        }

        if header.onto_ref.trim().is_empty() {
            violations.push(RebaseSequenceViolation::OntoRefMissing);
        }
        if header.original_tip_ref.trim().is_empty() {
            violations.push(RebaseSequenceViolation::OriginalTipRecoveryAnchorMissing);
        }
        if header.original_order_note.trim().is_empty() {
            violations.push(RebaseSequenceViolation::OriginalOrderContextMissing);
        }
        if !header.counts_are_consistent() {
            violations.push(RebaseSequenceViolation::HeaderCountsInconsistent);
        }
        if header.review_class != MutationReviewClass::SequenceRewriteConfirm {
            violations.push(RebaseSequenceViolation::SequenceConfirmCollapsed);
        }
        if header.checkpoint_disclosure.trim().is_empty() {
            violations.push(RebaseSequenceViolation::HeaderCheckpointUndisclosed);
        }

        let recovery = header.recovery();
        if recovery.must_be_recoverable && !recovery.is_recoverable {
            violations.push(RebaseSequenceViolation::HeaderRecoveryCheckpointMissing);
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
