//! Batch-review sheets and batch-action descriptors with
//! included / excluded / blocked / skipped counts, an undo/recovery class, and a
//! provider/policy-scope review gate for broad M5 actions.
//!
//! Where
//! [`crate::stabilize_selection_scope_and_batch_result_truth`] froze the portable
//! selection-scope and batch-result contract, and
//! [`crate::implement_selection_bars_range_anchor_and_stale_snapshot_guards`] made
//! the *live selection state* of a dense surface canonical, this module binds a
//! reviewed selection scope to the *broad action* about to run against it. It
//! makes the review sheet a canonical product object so a consequential rerun,
//! suppress, export, install, update, delete, share, or approve over a dense M5
//! collection previews the same truth before it mutates, exports, reruns,
//! installs, suppresses, or shares anything.
//!
//! Each [`BatchReviewSheet`] pins one [`DenseCollectionSurface`] rendered as a
//! [`CollectionViewKind`] to:
//!
//! - a [`BatchActionScopeDescriptor`] that names the [`BatchActionKind`], the
//!   [`BatchActionScopeClass`], the [`ExecutionOriginClass`], a
//!   [`BatchScopeCounts`] split into included / excluded / blocked / skipped /
//!   hidden members, and — the dimension this lane adds — an
//!   [`UndoRecoveryClass`] that is visible *and* exportable rather than inferred
//!   from ad hoc copy;
//! - per-member [`BatchReviewMemberRow`] disposition rows so the included,
//!   excluded, blocked, and skipped populations are named, not collapsed;
//! - [`ScopeBlock`] rows that thread policy / provider / workset / ownership /
//!   client / partial-data narrowing into the same packet so it is never hidden
//!   inside a generic filter chip; and
//! - an optional [`BatchResultSummary`] that preserves per-item or per-class
//!   success / failure truth after execution instead of collapsing a mixed
//!   outcome into one generic toast.
//!
//! The lane carries the guardrails the track demands: a consequential batch action
//! cannot run from a generic Continue button without a review sheet that names
//! included / excluded / blocked / skipped members and the recovery posture; a row
//! highlight never stands in for a durable selection; provider or policy narrowing
//! is never hidden in a generic chip; visible rows are never treated as all
//! matching rows without an explicit step; and a broad action never bypasses
//! preview because the list is virtualized or provider-backed.
//! [`BatchReviewSheet::reconstruction`] projects the same truth into a
//! redaction-aware [`BatchReviewSheetReconstruction`] that diagnostics and support
//! packets reuse instead of re-deriving disposition from raw rows.
//!
//! The boundary schema is
//! [`schemas/collections/add-batch-review-sheets-and-batch-action-descriptors-with-included-excluded-blocked-skippe.schema.json`](../../../../schemas/collections/add-batch-review-sheets-and-batch-action-descriptors-with-included-excluded-blocked-skippe.schema.json).
//! The contract doc is
//! [`docs/collections/m5/add-batch-review-sheets-and-batch-action-descriptors-with-included-excluded-blocked-skippe.md`](../../../../docs/collections/m5/add-batch-review-sheets-and-batch-action-descriptors-with-included-excluded-blocked-skippe.md).
//! The protected fixture directory is
//! [`fixtures/collections/m5/add-batch-review-sheets-and-batch-action-descriptors-with-included-excluded-blocked-skippe/`](../../../../fixtures/collections/m5/add-batch-review-sheets-and-batch-action-descriptors-with-included-excluded-blocked-skippe/).

#[cfg(test)]
mod tests;

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

use crate::freeze_the_m5_filter_ast_saved_view_column_preset_and_batch_action_descriptor_matrix::{
    BatchActionKind, BatchActionScopeClass, DenseCollectionSurface,
};
use crate::ship_result_scope_counters_and_hidden_narrowing_chips::CollectionViewKind;
use crate::stabilize_selection_scope_and_batch_result_truth::{
    BatchItemOutcome, BatchMemberDisposition, ExecutionOriginClass,
};

/// Stable record-kind tag carried by [`BatchReviewSheetPacket`].
pub const BATCH_REVIEW_SHEET_RECORD_KIND: &str = "m5_batch_review_sheet_packet";

/// Integer schema version for the batch-review sheet packet.
pub const BATCH_REVIEW_SHEET_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the boundary schema.
pub const BATCH_REVIEW_SHEET_SCHEMA_REF: &str =
    "schemas/collections/add-batch-review-sheets-and-batch-action-descriptors-with-included-excluded-blocked-skippe.schema.json";

/// Repo-relative path of the contract doc.
pub const BATCH_REVIEW_SHEET_DOC_REF: &str =
    "docs/collections/m5/add-batch-review-sheets-and-batch-action-descriptors-with-included-excluded-blocked-skippe.md";

/// Repo-relative path of the protected fixture directory.
pub const BATCH_REVIEW_SHEET_FIXTURE_DIR: &str =
    "fixtures/collections/m5/add-batch-review-sheets-and-batch-action-descriptors-with-included-excluded-blocked-skippe";

/// Repo-relative path of the checked support-export artifact.
pub const BATCH_REVIEW_SHEET_ARTIFACT_REF: &str =
    "artifacts/collections/m5/add-batch-review-sheets-and-batch-action-descriptors-with-included-excluded-blocked-skippe/support_export.json";

/// Repo-relative path of the checked Markdown summary.
pub const BATCH_REVIEW_SHEET_SUMMARY_REF: &str =
    "artifacts/collections/m5/add-batch-review-sheets-and-batch-action-descriptors-with-included-excluded-blocked-skippe.md";

/// The first real broad M5 actions this lane reviews live on these surfaces.
const REQUIRED_SHEET_SURFACES: [DenseCollectionSurface; 5] = [
    DenseCollectionSurface::PipelineRunList,
    DenseCollectionSurface::ReviewQueue,
    DenseCollectionSurface::IncidentList,
    DenseCollectionSurface::MarketplaceResults,
    DenseCollectionSurface::ProviderAdminTable,
];

/// How recoverable a broad action's effect is. This is the dimension this lane
/// adds: the undo/recovery posture is a canonical, visible, exportable product
/// object rather than a phrase inferred from ad hoc copy.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum UndoRecoveryClass {
    /// The action reads/exports/copies only; there is nothing to undo.
    NoMutation,
    /// The action is fully reversible with no data loss.
    FullyReversible,
    /// The action is reversible only within a stated time or version window.
    ReversibleWithinWindow,
    /// The action is reversible only by running a compensating inverse action
    /// (e.g. re-install after uninstall, re-open after dismiss).
    CompensatableViaInverse,
    /// Some members are reversible and some are not.
    PartiallyReversible,
    /// The action is destructive and cannot be undone.
    Irreversible,
}

impl UndoRecoveryClass {
    /// Every undo/recovery class, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::NoMutation,
        Self::FullyReversible,
        Self::ReversibleWithinWindow,
        Self::CompensatableViaInverse,
        Self::PartiallyReversible,
        Self::Irreversible,
    ];

    /// Stable token recorded in packets, fixtures, and exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NoMutation => "no_mutation",
            Self::FullyReversible => "fully_reversible",
            Self::ReversibleWithinWindow => "reversible_within_window",
            Self::CompensatableViaInverse => "compensatable_via_inverse",
            Self::PartiallyReversible => "partially_reversible",
            Self::Irreversible => "irreversible",
        }
    }

    /// True when the effect can be recovered at all (everything but
    /// [`UndoRecoveryClass::Irreversible`]).
    pub const fn is_recoverable(self) -> bool {
        !matches!(self, Self::Irreversible)
    }

    /// True when the action mutates state ([`UndoRecoveryClass::NoMutation`] does
    /// not).
    pub const fn mutates(self) -> bool {
        !matches!(self, Self::NoMutation)
    }

    /// True when the recovery posture is non-trivial and so the action demands an
    /// explicit review sheet rather than a generic Continue button.
    pub const fn requires_explicit_review(self) -> bool {
        matches!(
            self,
            Self::CompensatableViaInverse | Self::PartiallyReversible | Self::Irreversible
        )
    }
}

/// Why a member of the reviewed population is narrowed out of the action. Each
/// cause is threaded into the review packet so policy or provider narrowing is
/// never hidden inside a generic filter chip.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BatchScopeNarrowingCause {
    /// A policy / permission rule blocks the member.
    PolicyBlocked,
    /// The provider blocks or refuses the member.
    ProviderBlocked,
    /// The member is outside the active workset scope.
    WorksetScoped,
    /// The operator does not own the member.
    OwnershipBlocked,
    /// The client lacks the capability to act on the member.
    ClientCapability,
    /// Partial data means the member's eligibility is unknown.
    PartialDataUnknown,
}

impl BatchScopeNarrowingCause {
    /// Every narrowing cause, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::PolicyBlocked,
        Self::ProviderBlocked,
        Self::WorksetScoped,
        Self::OwnershipBlocked,
        Self::ClientCapability,
        Self::PartialDataUnknown,
    ];

    /// Stable token recorded in packets and exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::PolicyBlocked => "policy_blocked",
            Self::ProviderBlocked => "provider_blocked",
            Self::WorksetScoped => "workset_scoped",
            Self::OwnershipBlocked => "ownership_blocked",
            Self::ClientCapability => "client_capability",
            Self::PartialDataUnknown => "partial_data_unknown",
        }
    }

    /// True when the narrowing comes from a provider or policy decision, which the
    /// review sheet must surface explicitly rather than folding into a filter chip.
    pub const fn is_provider_or_policy(self) -> bool {
        matches!(self, Self::PolicyBlocked | Self::ProviderBlocked)
    }
}

/// One policy / provider / workset / ownership / client / partial-data block
/// shown in the review sheet. A block is always visible to the operator and
/// carries a precise reason, so narrowing is never silent.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ScopeBlock {
    /// Why the members are narrowed out.
    pub cause: BatchScopeNarrowingCause,
    /// Number of members blocked by this cause.
    pub member_count: u64,
    /// Redaction-aware, precise reason label — never a generic non-answer.
    pub reason_label: String,
    /// True when the block is surfaced to the operator rather than hidden in a
    /// generic filter chip (required).
    pub visible_to_operator: bool,
}

impl ScopeBlock {
    /// Whether the block is well formed: it covers at least one member, carries a
    /// precise reason, and is surfaced to the operator.
    pub fn is_valid(&self) -> bool {
        self.member_count > 0 && self.visible_to_operator && !label_is_generic(&self.reason_label)
    }
}

/// Included / excluded / blocked / skipped / hidden split of the reviewed
/// population for one batch action.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BatchScopeCounts {
    /// Members the action will act on.
    pub included: u64,
    /// Members the operator or filter excluded before commit.
    pub excluded: u64,
    /// Members blocked by policy, provider, ownership, or capability.
    pub blocked: u64,
    /// Members skipped as a known no-op.
    pub skipped: u64,
    /// Members hidden by the current filter or by policy.
    pub hidden: u64,
    /// Total population this sheet reviewed.
    pub total_reviewed: u64,
}

impl BatchScopeCounts {
    /// Whether the counts reconcile: included, excluded, blocked, skipped, and
    /// hidden partition the reviewed total exactly.
    pub fn reconciles(&self) -> bool {
        self.included
            .checked_add(self.excluded)
            .and_then(|sum| sum.checked_add(self.blocked))
            .and_then(|sum| sum.checked_add(self.skipped))
            .and_then(|sum| sum.checked_add(self.hidden))
            == Some(self.total_reviewed)
    }

    /// True when some members are blocked or hidden — the narrowing a review sheet
    /// must surface explicitly.
    pub fn has_blocked_or_hidden(&self) -> bool {
        self.blocked > 0 || self.hidden > 0
    }
}

/// One declared broad action with its scope, execution origin, counts, and
/// undo/recovery posture.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BatchActionScopeDescriptor {
    /// Stable action id.
    pub action_id: String,
    /// Action kind.
    pub action_kind: BatchActionKind,
    /// How the action's scope is established.
    pub scope_class: BatchActionScopeClass,
    /// Where the action executes.
    pub execution_origin: ExecutionOriginClass,
    /// Visible, exportable undo/recovery class.
    pub undo_recovery_class: UndoRecoveryClass,
    /// Included / excluded / blocked / skipped / hidden split.
    pub counts: BatchScopeCounts,
    /// True when the action mutates persistent or provider state.
    pub mutates_state: bool,
    /// True when a provider owns the execution or authoritative membership.
    pub provider_backed: bool,
    /// True when expansion from the visible rows to all matching rows required a
    /// deliberate, explicit step (never inferred from a visible selection).
    pub select_all_expansion_was_explicit: bool,
    /// Visible, exportable recovery-posture label describing how to undo, retry, or
    /// compensate — never a generic non-answer.
    pub undo_recovery_label: String,
}

impl BatchActionScopeDescriptor {
    /// Whether the action is consequential enough to demand a review sheet: it
    /// mutates state, is provider-backed, has a scope class that requires preview,
    /// has a non-trivial undo posture, or is an export / copy / share that carries
    /// data out.
    pub fn is_consequential(&self) -> bool {
        self.mutates_state
            || self.provider_backed
            || self.scope_class.requires_preview()
            || self.undo_recovery_class.requires_explicit_review()
            || matches!(
                self.action_kind,
                BatchActionKind::Export | BatchActionKind::Copy | BatchActionKind::Share
            )
    }

    /// Whether the descriptor satisfies its internal invariants: it carries
    /// identity, reconciled counts, a precise recovery label, and an undo class
    /// consistent with whether it mutates and whether a provider executes it.
    pub fn is_valid(&self) -> bool {
        if self.action_id.trim().is_empty() {
            return false;
        }
        if !self.counts.reconciles() {
            return false;
        }
        if label_is_generic(&self.undo_recovery_label) {
            return false;
        }
        // A mutating action cannot claim there is nothing to undo, and a
        // no-mutation action cannot claim to mutate.
        if self.mutates_state != self.undo_recovery_class.mutates() {
            return false;
        }
        // A provider-backed action cannot claim a purely local execution origin.
        if self.provider_backed && self.execution_origin == ExecutionOriginClass::LocalClient {
            return false;
        }
        true
    }
}

/// One reviewed member row in a batch review sheet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BatchReviewMemberRow {
    /// Stable item identity used across sort, filter, pagination, virtualization.
    pub stable_item_id: String,
    /// Redaction-aware label that never carries raw row bodies or query text.
    pub review_label: String,
    /// Member disposition in the review sheet.
    pub disposition: BatchMemberDisposition,
    /// Precise reason for the disposition — required and non-generic whenever the
    /// member is not plainly included.
    pub disposition_reason: String,
    /// True when the member is still inside the current filtered/sorted view.
    pub in_current_filter: bool,
}

impl BatchReviewMemberRow {
    /// Whether the row is well formed: it carries a stable id and a precise label,
    /// and any non-included disposition carries a precise reason.
    pub fn is_valid(&self) -> bool {
        if self.stable_item_id.trim().is_empty() || label_is_generic(&self.review_label) {
            return false;
        }
        if self.disposition == BatchMemberDisposition::Included {
            return true;
        }
        !label_is_generic(&self.disposition_reason)
    }
}

/// One per-item execution result row preserving the per-item outcome.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BatchItemResultRow {
    /// Stable item id this result is for.
    pub stable_item_id: String,
    /// Per-item outcome.
    pub outcome: BatchItemOutcome,
    /// Redaction-aware outcome label.
    pub outcome_label: String,
    /// Retry, rollback, or compensating-action ref for recovery.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub recovery_action_ref: Option<String>,
}

/// Post-execution result summary that preserves per-item or per-class
/// success / failure truth instead of collapsing a mixed outcome into one toast.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BatchResultSummary {
    /// Members that succeeded.
    pub succeeded_count: u64,
    /// Members that failed.
    pub failed_count: u64,
    /// Members skipped as a no-op.
    pub skipped_count: u64,
    /// Members blocked before mutation.
    pub blocked_count: u64,
    /// Precise summary label that names the mixed outcome — never a generic toast.
    pub summary_label: String,
    /// Per-item result rows preserving the per-item outcome.
    pub per_item_results: Vec<BatchItemResultRow>,
    /// True when the surface collapses the result into one generic toast
    /// (forbidden).
    pub collapses_to_single_toast: bool,
}

impl BatchResultSummary {
    /// True when at least two distinct outcome classes are present.
    pub fn is_mixed(&self) -> bool {
        let classes = [
            self.succeeded_count > 0,
            self.failed_count > 0,
            self.skipped_count > 0,
            self.blocked_count > 0,
        ];
        classes.iter().filter(|present| **present).count() >= 2
    }

    /// Whether the summary preserves per-item or per-class truth: it never
    /// collapses to a single toast, carries a precise label, and enumerates a
    /// per-item result for every member that failed or was blocked.
    pub fn preserves_per_item_truth(&self) -> bool {
        if self.collapses_to_single_toast {
            return false;
        }
        if label_is_generic(&self.summary_label) {
            return false;
        }
        // Every per-item result row must be well formed.
        let rows_ok = self.per_item_results.iter().all(|row| {
            !row.stable_item_id.trim().is_empty() && !label_is_generic(&row.outcome_label)
        });
        if !rows_ok {
            return false;
        }
        // A failed or blocked member must be individually accounted for so a
        // mixed outcome is never hidden behind the success count.
        let attention = self.failed_count + self.blocked_count;
        if attention == 0 {
            return true;
        }
        let enumerated_attention = self
            .per_item_results
            .iter()
            .filter(|row| {
                matches!(
                    row.outcome,
                    BatchItemOutcome::Failed | BatchItemOutcome::Blocked
                )
            })
            .count() as u64;
        enumerated_attention >= attention
    }
}

/// Redaction-aware projection of one review sheet for diagnostics and support
/// packets. Carries only ids, tokens, labels, and counts — never raw row bodies
/// or provider payloads.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BatchReviewSheetReconstruction {
    /// Sheet id this reconstruction projects.
    pub sheet_id: String,
    /// Surface token.
    pub surface_token: String,
    /// View-kind token.
    pub view_kind_token: String,
    /// Action-kind token.
    pub action_kind_token: String,
    /// Scope-class token.
    pub scope_class_token: String,
    /// Execution-origin token.
    pub execution_origin_token: String,
    /// Undo/recovery-class token.
    pub undo_recovery_class_token: String,
    /// True when the action is consequential.
    pub is_consequential: bool,
    /// Included count.
    pub included: u64,
    /// Excluded count.
    pub excluded: u64,
    /// Blocked count.
    pub blocked: u64,
    /// Skipped count.
    pub skipped: u64,
    /// Hidden count.
    pub hidden: u64,
    /// True when the sheet blocks a generic Continue.
    pub blocks_generic_continue: bool,
    /// True when the recovery posture is exportable.
    pub recovery_posture_exportable: bool,
    /// Provider/policy scope-block cause tokens surfaced by this sheet.
    pub provider_policy_block_tokens: Vec<String>,
    /// True when a result summary is present.
    pub has_result_summary: bool,
    /// True when a present result summary is mixed.
    pub result_is_mixed: bool,
}

/// One batch review sheet binding a reviewed selection scope to a broad action.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BatchReviewSheet {
    /// Stable sheet id.
    pub sheet_id: String,
    /// Bound dense collection surface.
    pub surface: DenseCollectionSurface,
    /// How the surface is rendered.
    pub view_kind: CollectionViewKind,
    /// Reviewed selection-scope object this sheet acts on.
    pub selection_id_ref: String,
    /// The broad action descriptor.
    pub action: BatchActionScopeDescriptor,
    /// Human-readable sheet title.
    pub sheet_title: String,
    /// Per-member disposition rows (enumerated or sampled).
    pub member_rows: Vec<BatchReviewMemberRow>,
    /// Policy / provider / workset / ownership / client / partial-data blocks.
    pub scope_blocks: Vec<ScopeBlock>,
    /// True when the action cannot commit without this review (required for
    /// consequential actions).
    pub requires_review_before_commit: bool,
    /// True when the action cannot run from a generic Continue button (required
    /// for consequential actions).
    pub blocks_generic_continue: bool,
    /// True when the sheet names the included / excluded / blocked / skipped
    /// members (required).
    pub names_included_excluded_blocked_skipped: bool,
    /// Visible, exportable recovery-posture label — never a generic non-answer.
    pub recovery_posture_label: String,
    /// True when the recovery posture is exportable for support and evidence
    /// (required).
    pub recovery_posture_exportable: bool,
    /// Post-execution result summary, present once the action has run.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub result_summary: Option<BatchResultSummary>,
    /// Screen-reader and keyboard-safe summary label.
    pub accessibility_summary: String,
    /// Evidence packet refs backing this sheet.
    pub evidence_refs: Vec<String>,
}

impl BatchReviewSheet {
    /// Whether the action is consequential.
    pub fn is_consequential(&self) -> bool {
        self.action.is_consequential()
    }

    /// Whether the review gate holds: a consequential action must require review,
    /// block a generic Continue, and name the included / excluded / blocked /
    /// skipped members before it can run.
    pub fn gate_holds(&self) -> bool {
        if !self.is_consequential() {
            return true;
        }
        self.requires_review_before_commit
            && self.blocks_generic_continue
            && self.names_included_excluded_blocked_skipped
    }

    /// Whether the recovery posture is disclosed: a precise, exportable label that
    /// the operator can see and support can export.
    pub fn recovery_disclosed(&self) -> bool {
        self.recovery_posture_exportable && !label_is_generic(&self.recovery_posture_label)
    }

    /// Whether narrowing is surfaced rather than hidden inside a generic filter
    /// chip: every block row is valid, and the scope blocks account for the
    /// blocked count exactly so each blocked member carries an explicit,
    /// operator-visible cause (policy, provider, workset, ownership, capability,
    /// or partial data).
    pub fn narrowing_surfaced(&self) -> bool {
        if !self.scope_blocks.iter().all(ScopeBlock::is_valid) {
            return false;
        }
        let block_sum: u64 = self
            .scope_blocks
            .iter()
            .map(|block| block.member_count)
            .sum();
        block_sum == self.action.counts.blocked
    }

    /// Whether the enumerated member rows are consistent with the counts: no
    /// disposition class enumerates more members than its count claims.
    pub fn member_rows_consistent_with_counts(&self) -> bool {
        let count_for = |disposition: BatchMemberDisposition| -> u64 {
            self.member_rows
                .iter()
                .filter(|row| row.disposition == disposition)
                .count() as u64
        };
        count_for(BatchMemberDisposition::Included) <= self.action.counts.included
            && count_for(BatchMemberDisposition::Excluded) <= self.action.counts.excluded
            && count_for(BatchMemberDisposition::Blocked) <= self.action.counts.blocked
            && count_for(BatchMemberDisposition::Skipped) <= self.action.counts.skipped
            && count_for(BatchMemberDisposition::Hidden) <= self.action.counts.hidden
    }

    /// Whether every dimension required to record this sheet is present and
    /// internally consistent.
    pub fn is_complete(&self) -> bool {
        !self.sheet_id.trim().is_empty()
            && !self.selection_id_ref.trim().is_empty()
            && !self.sheet_title.trim().is_empty()
            && !self.accessibility_summary.trim().is_empty()
            && self.action.is_valid()
            && self.member_rows.iter().all(BatchReviewMemberRow::is_valid)
            && self.scope_blocks.iter().all(ScopeBlock::is_valid)
            && self.member_rows_consistent_with_counts()
            && self.gate_holds()
            && self.recovery_disclosed()
            && self.narrowing_surfaced()
            && self
                .result_summary
                .as_ref()
                .map_or(true, BatchResultSummary::preserves_per_item_truth)
            && !self.evidence_refs.is_empty()
            && self.evidence_refs.iter().all(|r| !r.trim().is_empty())
    }

    /// Projects the sheet into a redaction-aware reconstruction for diagnostics
    /// and support packets.
    pub fn reconstruction(&self) -> BatchReviewSheetReconstruction {
        BatchReviewSheetReconstruction {
            sheet_id: self.sheet_id.clone(),
            surface_token: self.surface.as_str().to_owned(),
            view_kind_token: self.view_kind.as_str().to_owned(),
            action_kind_token: self.action.action_kind.as_str().to_owned(),
            scope_class_token: self.action.scope_class.as_str().to_owned(),
            execution_origin_token: self.action.execution_origin.as_str().to_owned(),
            undo_recovery_class_token: self.action.undo_recovery_class.as_str().to_owned(),
            is_consequential: self.is_consequential(),
            included: self.action.counts.included,
            excluded: self.action.counts.excluded,
            blocked: self.action.counts.blocked,
            skipped: self.action.counts.skipped,
            hidden: self.action.counts.hidden,
            blocks_generic_continue: self.blocks_generic_continue,
            recovery_posture_exportable: self.recovery_posture_exportable,
            provider_policy_block_tokens: self
                .scope_blocks
                .iter()
                .filter(|block| block.cause.is_provider_or_policy())
                .map(|block| block.cause.as_str().to_owned())
                .collect(),
            has_result_summary: self.result_summary.is_some(),
            result_is_mixed: self
                .result_summary
                .as_ref()
                .is_some_and(BatchResultSummary::is_mixed),
        }
    }
}

/// Guardrail invariants block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BatchReviewGuardrails {
    /// A row highlight never stands in for a durable selection.
    pub row_highlight_is_not_durable_selection: bool,
    /// Provider or policy narrowing is never hidden inside a generic filter chip.
    pub provider_policy_narrowing_never_hidden: bool,
    /// Visible rows are never treated as all matching rows without an explicit
    /// step.
    pub visible_rows_not_all_matching_without_explicit_step: bool,
    /// A broad action cannot bypass preview because the list is virtualized or
    /// provider-backed.
    pub broad_action_cannot_bypass_preview: bool,
    /// The undo/recovery class is visible and exportable, not inferred from copy.
    pub undo_recovery_class_visible_and_exportable: bool,
}

impl BatchReviewGuardrails {
    /// Whether every guardrail invariant holds.
    pub fn all_hold(&self) -> bool {
        self.row_highlight_is_not_durable_selection
            && self.provider_policy_narrowing_never_hidden
            && self.visible_rows_not_all_matching_without_explicit_step
            && self.broad_action_cannot_bypass_preview
            && self.undo_recovery_class_visible_and_exportable
    }
}

/// Consumer projection block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BatchReviewConsumerProjection {
    /// Product renders the review sheet and its counts from these records.
    pub product_renders_review_sheet: bool,
    /// Diagnostics reconstruct batch truth from these records.
    pub diagnostics_reconstructs_batch_truth: bool,
    /// Support/export reuses the batch-truth projection.
    pub support_export_reuses_records: bool,
    /// Docs and help reuse the batch-review vocabulary.
    pub docs_help_reuses_vocabulary: bool,
}

impl BatchReviewConsumerProjection {
    /// Whether every consumer-projection invariant holds.
    pub fn all_hold(&self) -> bool {
        self.product_renders_review_sheet
            && self.diagnostics_reconstructs_batch_truth
            && self.support_export_reuses_records
            && self.docs_help_reuses_vocabulary
    }
}

/// Constructor input for [`BatchReviewSheetPacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BatchReviewSheetPacketInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable packet label.
    pub packet_label: String,
    /// Per-action review sheets.
    pub sheets: Vec<BatchReviewSheet>,
    /// Guardrail invariants block.
    pub guardrails: BatchReviewGuardrails,
    /// Consumer projection block.
    pub consumer_projection: BatchReviewConsumerProjection,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

/// Export-safe batch-review sheet packet for the first real broad M5 actions.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BatchReviewSheetPacket {
    /// Record kind; must equal [`BATCH_REVIEW_SHEET_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`BATCH_REVIEW_SHEET_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable packet label.
    pub packet_label: String,
    /// Per-action review sheets.
    pub sheets: Vec<BatchReviewSheet>,
    /// Guardrail invariants block.
    pub guardrails: BatchReviewGuardrails,
    /// Consumer projection block.
    pub consumer_projection: BatchReviewConsumerProjection,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

impl BatchReviewSheetPacket {
    /// Builds a batch-review sheet packet.
    pub fn new(input: BatchReviewSheetPacketInput) -> Self {
        Self {
            record_kind: BATCH_REVIEW_SHEET_RECORD_KIND.to_owned(),
            schema_version: BATCH_REVIEW_SHEET_SCHEMA_VERSION,
            packet_id: input.packet_id,
            packet_label: input.packet_label,
            sheets: input.sheets,
            guardrails: input.guardrails,
            consumer_projection: input.consumer_projection,
            source_contract_refs: input.source_contract_refs,
            redaction_class_token: input.redaction_class_token,
            minted_at: input.minted_at,
        }
    }

    /// Surfaces represented by some sheet in this packet.
    pub fn represented_surfaces(&self) -> BTreeSet<DenseCollectionSurface> {
        self.sheets.iter().map(|sheet| sheet.surface).collect()
    }

    /// Action kinds represented by some sheet in this packet.
    pub fn represented_action_kinds(&self) -> BTreeSet<BatchActionKind> {
        self.sheets
            .iter()
            .map(|sheet| sheet.action.action_kind)
            .collect()
    }

    /// Undo/recovery classes represented by some sheet in this packet.
    pub fn represented_undo_classes(&self) -> BTreeSet<UndoRecoveryClass> {
        self.sheets
            .iter()
            .map(|sheet| sheet.action.undo_recovery_class)
            .collect()
    }

    /// Count of consequential sheets that block a generic Continue.
    pub fn gated_sheet_count(&self) -> usize {
        self.sheets
            .iter()
            .filter(|sheet| sheet.is_consequential() && sheet.blocks_generic_continue)
            .count()
    }

    /// Reconstructions for every sheet, used by diagnostics and support packets.
    pub fn reconstructions(&self) -> Vec<BatchReviewSheetReconstruction> {
        self.sheets
            .iter()
            .map(BatchReviewSheet::reconstruction)
            .collect()
    }

    /// Validates the batch-review sheet packet invariants.
    pub fn validate(&self) -> Vec<BatchReviewSheetViolation> {
        let mut violations = Vec::new();

        if self.record_kind != BATCH_REVIEW_SHEET_RECORD_KIND {
            violations.push(BatchReviewSheetViolation::WrongRecordKind);
        }
        if self.schema_version != BATCH_REVIEW_SHEET_SCHEMA_VERSION {
            violations.push(BatchReviewSheetViolation::WrongSchemaVersion);
        }
        if self.packet_id.trim().is_empty()
            || self.packet_label.trim().is_empty()
            || self.redaction_class_token.trim().is_empty()
            || self.minted_at.trim().is_empty()
        {
            violations.push(BatchReviewSheetViolation::MissingIdentity);
        }

        validate_source_contracts(self, &mut violations);
        validate_coverage(self, &mut violations);
        validate_sheets(self, &mut violations);

        if !self.guardrails.all_hold() {
            violations.push(BatchReviewSheetViolation::GuardrailsIncomplete);
        }
        if !self.consumer_projection.all_hold() {
            violations.push(BatchReviewSheetViolation::ConsumerProjectionIncomplete);
        }

        if json_contains_forbidden_boundary_material(
            &serde_json::to_value(self).expect("batch review sheet packet serializes"),
        ) {
            violations.push(BatchReviewSheetViolation::RawBoundaryMaterialInExport);
        }

        violations
    }

    /// Deterministic export-safe JSON.
    ///
    /// # Panics
    ///
    /// Panics only if serializing this metadata-only packet fails.
    pub fn export_safe_json(&self) -> String {
        serde_json::to_string_pretty(self).expect("batch review sheet packet serializes")
    }

    /// Deterministic Markdown summary for support, docs, or review handoff.
    pub fn render_markdown_summary(&self) -> String {
        let mut out = String::new();
        out.push_str("# M5 Batch-Review Sheets And Batch-Action Descriptors\n\n");
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- Label: `{}`\n", self.packet_label));
        out.push_str(&format!(
            "- Sheets: {} ({} gated)\n",
            self.sheets.len(),
            self.gated_sheet_count()
        ));
        out.push_str(&format!(
            "- Surfaces: {} / {}\n",
            self.represented_surfaces().len(),
            REQUIRED_SHEET_SURFACES.len()
        ));
        out.push_str(&format!(
            "- Undo classes: {} / {}\n",
            self.represented_undo_classes().len(),
            UndoRecoveryClass::ALL.len()
        ));
        out.push_str("\n## Sheets\n\n");
        for sheet in &self.sheets {
            out.push_str(&format!(
                "- **{}** ({} / {} / {}): {}\n",
                sheet.sheet_id,
                sheet.surface.as_str(),
                sheet.view_kind.as_str(),
                sheet.action.action_kind.as_str(),
                sheet.sheet_title,
            ));
            out.push_str(&format!(
                "  - included={} excluded={} blocked={} skipped={} hidden={} total={}\n",
                sheet.action.counts.included,
                sheet.action.counts.excluded,
                sheet.action.counts.blocked,
                sheet.action.counts.skipped,
                sheet.action.counts.hidden,
                sheet.action.counts.total_reviewed,
            ));
            out.push_str(&format!(
                "  - undo=`{}` origin=`{}` scope=`{}` gated={}\n",
                sheet.action.undo_recovery_class.as_str(),
                sheet.action.execution_origin.as_str(),
                sheet.action.scope_class.as_str(),
                sheet.blocks_generic_continue,
            ));
            out.push_str(&format!("  - recovery: {}\n", sheet.recovery_posture_label));
            for block in &sheet.scope_blocks {
                out.push_str(&format!(
                    "  - block `{}` x{}: {}\n",
                    block.cause.as_str(),
                    block.member_count,
                    block.reason_label,
                ));
            }
            if let Some(summary) = &sheet.result_summary {
                out.push_str(&format!(
                    "  - result: ok={} failed={} skipped={} blocked={} ({})\n",
                    summary.succeeded_count,
                    summary.failed_count,
                    summary.skipped_count,
                    summary.blocked_count,
                    summary.summary_label,
                ));
            }
        }
        out
    }
}

/// Errors emitted when reading the checked-in batch-review sheet export.
#[derive(Debug)]
pub enum BatchReviewSheetArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<BatchReviewSheetViolation>),
}

impl fmt::Display for BatchReviewSheetArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(formatter, "batch review sheet export parse failed: {error}")
            }
            Self::Validation(violations) => {
                let tokens = violations
                    .iter()
                    .map(|violation| violation.as_str())
                    .collect::<Vec<_>>()
                    .join(",");
                write!(
                    formatter,
                    "batch review sheet export failed validation: {tokens}"
                )
            }
        }
    }
}

impl Error for BatchReviewSheetArtifactError {}

/// Validation failures emitted by [`BatchReviewSheetPacket::validate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BatchReviewSheetViolation {
    /// Packet record kind is wrong.
    WrongRecordKind,
    /// Packet schema version is wrong.
    WrongSchemaVersion,
    /// Required identity field is missing.
    MissingIdentity,
    /// Required base source contract refs are incomplete.
    MissingSourceContracts,
    /// A required real M5 surface is reviewed by no sheet.
    RequiredSurfaceMissing,
    /// No sheet reviews a destructive (irreversible) action.
    IrreversibleCaseMissing,
    /// No sheet reviews a provider-backed action.
    ProviderBackedCaseMissing,
    /// No sheet surfaces a provider/policy scope block.
    ProviderPolicyBlockCaseMissing,
    /// No sheet preserves a mixed-outcome result summary.
    MixedOutcomeCaseMissing,
    /// No sheet demonstrates a gated, consequential action.
    GatedActionCaseMissing,
    /// A sheet is incomplete.
    SheetIncomplete,
    /// A consequential action could run from a generic Continue without review.
    ConsequentialActionBypassesReview,
    /// A sheet's action descriptor is invalid.
    ActionDescriptorInvalid,
    /// A sheet's scope counts do not reconcile.
    CountsDoNotReconcile,
    /// A sheet hides provider or policy narrowing.
    ProviderPolicyNarrowingHidden,
    /// A sheet's undo/recovery posture is not visible and exportable.
    RecoveryPostureUndisclosed,
    /// A sheet collapses a mixed outcome into one generic toast.
    MixedOutcomeCollapsed,
    /// A sheet's member rows exceed the counts they claim.
    MemberRowsExceedCounts,
    /// A sheet lacks evidence refs.
    SheetEvidenceMissing,
    /// Guardrail block does not satisfy required invariants.
    GuardrailsIncomplete,
    /// Consumer projection does not satisfy required invariants.
    ConsumerProjectionIncomplete,
    /// Export contains raw boundary material.
    RawBoundaryMaterialInExport,
}

impl BatchReviewSheetViolation {
    /// Stable token used in tests and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WrongRecordKind => "wrong_record_kind",
            Self::WrongSchemaVersion => "wrong_schema_version",
            Self::MissingIdentity => "missing_identity",
            Self::MissingSourceContracts => "missing_source_contracts",
            Self::RequiredSurfaceMissing => "required_surface_missing",
            Self::IrreversibleCaseMissing => "irreversible_case_missing",
            Self::ProviderBackedCaseMissing => "provider_backed_case_missing",
            Self::ProviderPolicyBlockCaseMissing => "provider_policy_block_case_missing",
            Self::MixedOutcomeCaseMissing => "mixed_outcome_case_missing",
            Self::GatedActionCaseMissing => "gated_action_case_missing",
            Self::SheetIncomplete => "sheet_incomplete",
            Self::ConsequentialActionBypassesReview => "consequential_action_bypasses_review",
            Self::ActionDescriptorInvalid => "action_descriptor_invalid",
            Self::CountsDoNotReconcile => "counts_do_not_reconcile",
            Self::ProviderPolicyNarrowingHidden => "provider_policy_narrowing_hidden",
            Self::RecoveryPostureUndisclosed => "recovery_posture_undisclosed",
            Self::MixedOutcomeCollapsed => "mixed_outcome_collapsed",
            Self::MemberRowsExceedCounts => "member_rows_exceed_counts",
            Self::SheetEvidenceMissing => "sheet_evidence_missing",
            Self::GuardrailsIncomplete => "guardrails_incomplete",
            Self::ConsumerProjectionIncomplete => "consumer_projection_incomplete",
            Self::RawBoundaryMaterialInExport => "raw_boundary_material_in_export",
        }
    }
}

/// Reads and validates the checked-in batch-review sheet export.
///
/// # Errors
///
/// Returns an artifact error if the export cannot parse or fails validation.
pub fn current_m5_batch_review_sheet_export(
) -> Result<BatchReviewSheetPacket, BatchReviewSheetArtifactError> {
    let packet: BatchReviewSheetPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/collections/m5/add-batch-review-sheets-and-batch-action-descriptors-with-included-excluded-blocked-skippe/support_export.json"
    )))
    .map_err(BatchReviewSheetArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(BatchReviewSheetArtifactError::Validation(violations))
    }
}

fn validate_source_contracts(
    packet: &BatchReviewSheetPacket,
    violations: &mut Vec<BatchReviewSheetViolation>,
) {
    let refs: BTreeSet<&str> = packet
        .source_contract_refs
        .iter()
        .map(String::as_str)
        .collect();
    for required in [
        BATCH_REVIEW_SHEET_SCHEMA_REF,
        BATCH_REVIEW_SHEET_DOC_REF,
        BATCH_REVIEW_SHEET_ARTIFACT_REF,
    ] {
        if !refs.contains(required) {
            violations.push(BatchReviewSheetViolation::MissingSourceContracts);
            break;
        }
    }
}

fn validate_coverage(
    packet: &BatchReviewSheetPacket,
    violations: &mut Vec<BatchReviewSheetViolation>,
) {
    let surfaces = packet.represented_surfaces();
    for required in REQUIRED_SHEET_SURFACES {
        if !surfaces.contains(&required) {
            violations.push(BatchReviewSheetViolation::RequiredSurfaceMissing);
            break;
        }
    }

    if !packet
        .sheets
        .iter()
        .any(|sheet| sheet.action.undo_recovery_class == UndoRecoveryClass::Irreversible)
    {
        violations.push(BatchReviewSheetViolation::IrreversibleCaseMissing);
    }
    if !packet
        .sheets
        .iter()
        .any(|sheet| sheet.action.provider_backed)
    {
        violations.push(BatchReviewSheetViolation::ProviderBackedCaseMissing);
    }
    if !packet.sheets.iter().any(|sheet| {
        sheet
            .scope_blocks
            .iter()
            .any(|block| block.cause.is_provider_or_policy())
    }) {
        violations.push(BatchReviewSheetViolation::ProviderPolicyBlockCaseMissing);
    }
    if !packet.sheets.iter().any(|sheet| {
        sheet
            .result_summary
            .as_ref()
            .is_some_and(BatchResultSummary::is_mixed)
    }) {
        violations.push(BatchReviewSheetViolation::MixedOutcomeCaseMissing);
    }
    if !packet
        .sheets
        .iter()
        .any(|sheet| sheet.is_consequential() && sheet.blocks_generic_continue)
    {
        violations.push(BatchReviewSheetViolation::GatedActionCaseMissing);
    }
}

fn validate_sheets(
    packet: &BatchReviewSheetPacket,
    violations: &mut Vec<BatchReviewSheetViolation>,
) {
    for sheet in &packet.sheets {
        if !sheet.is_complete() {
            violations.push(BatchReviewSheetViolation::SheetIncomplete);
        }
        if !sheet.action.is_valid() {
            violations.push(BatchReviewSheetViolation::ActionDescriptorInvalid);
        }
        if !sheet.action.counts.reconciles() {
            violations.push(BatchReviewSheetViolation::CountsDoNotReconcile);
        }
        if sheet.is_consequential() && !sheet.gate_holds() {
            violations.push(BatchReviewSheetViolation::ConsequentialActionBypassesReview);
        }
        if !sheet.narrowing_surfaced() {
            violations.push(BatchReviewSheetViolation::ProviderPolicyNarrowingHidden);
        }
        if !sheet.recovery_disclosed() {
            violations.push(BatchReviewSheetViolation::RecoveryPostureUndisclosed);
        }
        if sheet
            .result_summary
            .as_ref()
            .is_some_and(|summary| !summary.preserves_per_item_truth())
        {
            violations.push(BatchReviewSheetViolation::MixedOutcomeCollapsed);
        }
        if !sheet.member_rows_consistent_with_counts() {
            violations.push(BatchReviewSheetViolation::MemberRowsExceedCounts);
        }
        if sheet.evidence_refs.is_empty() || sheet.evidence_refs.iter().any(|r| r.trim().is_empty())
        {
            violations.push(BatchReviewSheetViolation::SheetEvidenceMissing);
        }
    }
}

/// Whether a label is a generic non-answer rather than a precise label. A generic
/// provider error must never stand in for precise batch-review truth.
fn label_is_generic(label: &str) -> bool {
    let trimmed = label.trim();
    if trimmed.is_empty() {
        return true;
    }
    let lower = trimmed.to_lowercase();
    matches!(
        lower.as_str(),
        "unavailable"
            | "not available"
            | "n/a"
            | "error"
            | "provider error"
            | "request failed"
            | "failed"
            | "blocked"
            | "hidden"
            | "skipped"
            | "excluded"
            | "done"
            | "ok"
            | "success"
            | "review"
            | "continue"
    )
}

/// Heuristic that rejects obviously forbidden material in export-safe JSON.
fn json_contains_forbidden_boundary_material(value: &serde_json::Value) -> bool {
    match value {
        serde_json::Value::String(s) => {
            let lower = s.to_lowercase();
            lower.contains("api_key")
                || lower.contains("password")
                || lower.contains("secret_value")
                || lower.contains("bearer ")
        }
        serde_json::Value::Array(arr) => arr.iter().any(json_contains_forbidden_boundary_material),
        serde_json::Value::Object(map) => {
            map.values().any(json_contains_forbidden_boundary_material)
        }
        _ => false,
    }
}
