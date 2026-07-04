//! Implements the reusable rerun-comparison-sheet and retry-scope-review primitive: a
//! rerun comparison sheet, a set of changed-context rows, a CLI / headless line, and a
//! support-export projection that all resolve from one bounded rerun review and share
//! one sheet identity, one prior-run identity, and one prior-attempt identity, so a
//! rerun that would leave the shell stays explicit about which reviewed action it is
//! (`Rerun exactly`, `Rerun with current context`, or `Retry failed step only`), which
//! inputs, targets, runtimes, profiles, authority, and side-effect classes have
//! changed, and why the product believes the new attempt differs from the earlier one.
//!
//! Where
//! [`crate::freeze_the_m5_run_attempt_input_request_artifact_publish_rerun_review_and_debug_hierarchy_component_matrix`]
//! *freezes* the reusable execution-lifecycle component families as a governed
//! contract, this module *narrows* one of those families —
//! [`crate::freeze_the_m5_run_attempt_input_request_artifact_publish_rerun_review_and_debug_hierarchy_component_matrix::M5ExecutionComponentFamily::RerunComparisonSheet`]
//! — into one working primitive with a real **resolver**. A single rerun review
//! projects onto surfaces that share one sheet identity, one prior-run identity, and one
//! prior-attempt identity, so reviewed rerun mode, changed-context diff, retry scope,
//! and prior-attempt lineage never blur across the sheet, the change rows, the CLI /
//! headless line, and the support-export projection.
//!
//! The three acceptance criteria the resolver proves:
//!
//! - **AC1 — rerun controls no longer present as one generic action when inputs,
//!   targets, or authority have changed.** When the current context differs from an
//!   exact replay, the sheet keeps `Rerun exactly` and `Rerun with current context`
//!   distinct reviewed actions rather than collapsing them into one button; a retry of a
//!   failed step is offered as its own reviewed action when the prior run failed.
//! - **AC2 — users can review the changed execution context before retrying.** Every
//!   changed dimension — input, target, runtime, profile, approval / authority, or
//!   side-effect class — is enumerated with a before / after summary and is shown before
//!   dispatch, never after the rerun has already left the shell.
//! - **AC3 — support and export artifacts preserve the reviewed rerun mode and the
//!   changed-input summary.** Every export carries the reviewed rerun mode, retry scope,
//!   the changed-context summary, and the prior-attempt lineage plus the difference
//!   reason, so a support replay reconstructs exactly which rerun was reviewed.
//!
//! Raw command bytes, secret values, credentials, provider cursors, and raw diff
//! payloads never cross this boundary; the resolver carries only opaque refs, typed
//! class tokens, booleans, and redacted labels, so support and diagnostics exports
//! reconstruct exactly what a surface would have shown without leaking source or live
//! payloads.
//!
//! The boundary schema is
//! [`schemas/ui/m5-rerun-comparison-sheet.schema.json`](../../../../schemas/ui/m5-rerun-comparison-sheet.schema.json).
//! The contract doc is
//! [`docs/run-test-debug/m5_rerun_comparison_sheet_primitive.md`](../../../../docs/run-test-debug/m5_rerun_comparison_sheet_primitive.md).

#[cfg(test)]
mod tests;

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

use crate::freeze_the_m5_run_attempt_input_request_artifact_publish_rerun_review_and_debug_hierarchy_component_matrix::{
    DegradedState, M5ExecutionDowngradeTrigger, M5ExecutionLocality, M5ExecutionTruthMode,
    M5RerunContext, M5RunOutcome,
};
use crate::implement_the_m5_run_attempt_header_and_attempt_selector_primitive::M5RunAttemptSurfaceFamily;

/// Stable record-kind tag carried by [`M5RerunReviewPrimitivePacket`].
pub const M5_RERUN_REVIEW_RECORD_KIND: &str = "m5_rerun_comparison_sheet_primitive";

/// Schema version for the rerun-comparison-sheet primitive packet.
pub const M5_RERUN_REVIEW_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the boundary schema.
pub const M5_RERUN_REVIEW_SCHEMA_REF: &str = "schemas/ui/m5-rerun-comparison-sheet.schema.json";

/// Repo-relative path of the contract doc.
pub const M5_RERUN_REVIEW_DOC_REF: &str =
    "docs/run-test-debug/m5_rerun_comparison_sheet_primitive.md";

/// Repo-relative path of the frozen component-matrix contract this primitive narrows.
pub const M5_RERUN_REVIEW_COMPONENT_MATRIX_REF: &str =
    "schemas/ui/m5-execution-lifecycle-component-matrix.schema.json";

/// Repo-relative path of the protected fixture directory.
pub const M5_RERUN_REVIEW_FIXTURE_DIR: &str = "fixtures/ui/m5-rerun-comparison-sheet-primitive";

/// Repo-relative path of the checked support-export artifact (the `include_str!`
/// canonical).
pub const M5_RERUN_REVIEW_ARTIFACT_REF: &str =
    "artifacts/release/m5-rerun-comparison-sheet-primitive-proof/support_export.json";

/// Repo-relative path of the checked matrix CSV.
pub const M5_RERUN_REVIEW_CSV_REF: &str =
    "artifacts/release/m5-rerun-comparison-sheet-primitive-proof/matrix.csv";

/// Repo-relative path of the checked Markdown report.
pub const M5_RERUN_REVIEW_REPORT_REF: &str =
    "artifacts/release/m5-rerun-comparison-sheet-primitive-proof/report.md";

// --- minted controlled vocabulary ---

/// Closed rerun-mode vocabulary. Names the distinct reviewed action a rerun sheet
/// offers so a rerun-exactly, a rerun-with-current-context, and a retry-failed-step
/// never collapse into one generic button when they are not semantically equivalent.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5RerunMode {
    /// Replays the exact prior selection, environment, and inputs.
    RerunExactly,
    /// Reruns against the current (possibly changed) context.
    RerunWithCurrentContext,
    /// Retries only the failed step / units of the prior attempt.
    RetryFailedStepOnly,
}

impl M5RerunMode {
    /// Every rerun mode, in declaration order.
    pub const ALL: [Self; 3] = [
        Self::RerunExactly,
        Self::RerunWithCurrentContext,
        Self::RetryFailedStepOnly,
    ];

    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::RerunExactly => "rerun_exactly",
            Self::RerunWithCurrentContext => "rerun_with_current_context",
            Self::RetryFailedStepOnly => "retry_failed_step_only",
        }
    }

    /// Human-readable label for the sheet and report.
    pub const fn label(self) -> &'static str {
        match self {
            Self::RerunExactly => "Rerun exactly",
            Self::RerunWithCurrentContext => "Rerun with current context",
            Self::RetryFailedStepOnly => "Retry failed step only",
        }
    }

    /// True when the mode replays the exact prior context.
    pub const fn is_exact_replay(self) -> bool {
        matches!(self, Self::RerunExactly)
    }
}

/// Closed retry-scope vocabulary. Names how much of the prior run a rerun re-executes
/// so a whole-run rerun and a failed-step retry never read as the same scope.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5RetryScope {
    /// Re-executes the whole run from the start.
    WholeRun,
    /// Re-executes only the failed step / units.
    FailedStepOnly,
    /// Re-executes an explicitly selected subset of units.
    SelectedSubset,
}

impl M5RetryScope {
    /// Every retry scope, in declaration order.
    pub const ALL: [Self; 3] = [
        Self::WholeRun,
        Self::FailedStepOnly,
        Self::SelectedSubset,
    ];

    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WholeRun => "whole_run",
            Self::FailedStepOnly => "failed_step_only",
            Self::SelectedSubset => "selected_subset",
        }
    }
}

/// Closed rerun-change-dimension vocabulary. Names the execution dimensions a rerun
/// sheet diffs so a changed input, target, runtime, profile, authority, or side-effect
/// class is always enumerated rather than folded into a single "context changed" flag.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5RerunChangeDimension {
    /// The run's inputs / arguments.
    Input,
    /// The run's target (what is executed).
    Target,
    /// The runtime / toolchain the run executes on.
    Runtime,
    /// The launch / execution profile.
    Profile,
    /// The approval / auth posture the run requires.
    ApprovalAuthority,
    /// The side-effect class the run performs.
    SideEffectClass,
}

impl M5RerunChangeDimension {
    /// Every change dimension, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::Input,
        Self::Target,
        Self::Runtime,
        Self::Profile,
        Self::ApprovalAuthority,
        Self::SideEffectClass,
    ];

    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Input => "input",
            Self::Target => "target",
            Self::Runtime => "runtime",
            Self::Profile => "profile",
            Self::ApprovalAuthority => "approval_authority",
            Self::SideEffectClass => "side_effect_class",
        }
    }

    /// Human-readable label for the report.
    pub const fn label(self) -> &'static str {
        match self {
            Self::Input => "Input",
            Self::Target => "Target",
            Self::Runtime => "Runtime",
            Self::Profile => "Profile",
            Self::ApprovalAuthority => "Approval / authority",
            Self::SideEffectClass => "Side-effect class",
        }
    }
}

/// Closed rerun-change-state vocabulary. Names whether a dimension is unchanged,
/// changed, cannot be confirmed, or is not applicable so a rerun never silently claims a
/// dimension is unchanged when the product cannot prove it.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5RerunChangeState {
    /// The dimension is unchanged from the prior attempt.
    Unchanged,
    /// The dimension has changed from the prior attempt.
    Changed,
    /// The dimension's equivalence could not be confirmed; it must be reviewed.
    Unknown,
    /// The dimension does not apply to this rerun mode.
    NotApplicable,
}

impl M5RerunChangeState {
    /// Every change state, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::Unchanged,
        Self::Changed,
        Self::Unknown,
        Self::NotApplicable,
    ];

    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Unchanged => "unchanged",
            Self::Changed => "changed",
            Self::Unknown => "unknown",
            Self::NotApplicable => "not_applicable",
        }
    }

    /// True when the dimension has changed and so must show a before / after delta.
    pub const fn is_changed(self) -> bool {
        matches!(self, Self::Changed)
    }

    /// True when the dimension must be reviewed before dispatch: it changed, or its
    /// equivalence could not be confirmed.
    pub const fn requires_review(self) -> bool {
        matches!(self, Self::Changed | Self::Unknown)
    }
}

/// Closed side-effect-class vocabulary. Names the escalating tiers of side effect a run
/// performs so a rerun that would escalate from read-only to an external or irreversible
/// write is never dispatched without surfacing the escalation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5SideEffectClass {
    /// No side effects.
    None,
    /// Reads external state but performs no writes.
    ReadOnly,
    /// Writes only to local, reversible state.
    LocalWrite,
    /// Writes to external state.
    ExternalWrite,
    /// Performs an irreversible action.
    Irreversible,
}

impl M5SideEffectClass {
    /// Every side-effect class, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::None,
        Self::ReadOnly,
        Self::LocalWrite,
        Self::ExternalWrite,
        Self::Irreversible,
    ];

    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::None => "none",
            Self::ReadOnly => "read_only",
            Self::LocalWrite => "local_write",
            Self::ExternalWrite => "external_write",
            Self::Irreversible => "irreversible",
        }
    }

    /// Escalation rank; a higher rank is a more consequential side effect.
    pub const fn rank(self) -> u8 {
        match self {
            Self::None => 0,
            Self::ReadOnly => 1,
            Self::LocalWrite => 2,
            Self::ExternalWrite => 3,
            Self::Irreversible => 4,
        }
    }

    /// True when this class is a more consequential side effect than `prior`.
    pub const fn escalates_beyond(self, prior: Self) -> bool {
        self.rank() > prior.rank()
    }
}

/// Closed export-field vocabulary. Names the fields the support / export packet must
/// carry per surface; the mandatory subset must appear on every row so a support replay
/// reconstructs the reviewed rerun mode and the changed-input summary (AC3).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5RerunExportField {
    /// The prior run identity being compared against.
    PriorRunId,
    /// The prior attempt identity, distinct from the prior run identity.
    PriorAttemptId,
    /// The 1-based ordinal of the prior attempt.
    PriorAttemptOrdinal,
    /// The 1-based ordinal the rerun's new attempt would take.
    NewAttemptOrdinal,
    /// The reviewed rerun mode.
    RerunMode,
    /// The rerun context (exact / current / modified).
    RerunContext,
    /// The retry scope (whole run / failed step / selected subset).
    RetryScope,
    /// The changed dimensions enumerated on the sheet.
    ChangedDimensions,
    /// The one-line changed-context summary.
    ChangeSummary,
    /// The reason the product believes the attempts differ.
    DifferenceReason,
    /// The rerun's side-effect class.
    SideEffectClass,
    /// The baseline run ref the sheet compares against.
    BaselineRunRef,
}

impl M5RerunExportField {
    /// Every export field, in declaration order.
    pub const ALL: [Self; 12] = [
        Self::PriorRunId,
        Self::PriorAttemptId,
        Self::PriorAttemptOrdinal,
        Self::NewAttemptOrdinal,
        Self::RerunMode,
        Self::RerunContext,
        Self::RetryScope,
        Self::ChangedDimensions,
        Self::ChangeSummary,
        Self::DifferenceReason,
        Self::SideEffectClass,
        Self::BaselineRunRef,
    ];

    /// The mandatory subset every row must carry: the prior run/attempt IDs, the
    /// reviewed rerun mode, the changed dimensions and one-line summary, and the
    /// difference reason that must survive into any support export (AC3).
    pub const MANDATORY: [Self; 6] = [
        Self::PriorRunId,
        Self::PriorAttemptId,
        Self::RerunMode,
        Self::ChangedDimensions,
        Self::ChangeSummary,
        Self::DifferenceReason,
    ];

    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::PriorRunId => "prior_run_id",
            Self::PriorAttemptId => "prior_attempt_id",
            Self::PriorAttemptOrdinal => "prior_attempt_ordinal",
            Self::NewAttemptOrdinal => "new_attempt_ordinal",
            Self::RerunMode => "rerun_mode",
            Self::RerunContext => "rerun_context",
            Self::RetryScope => "retry_scope",
            Self::ChangedDimensions => "changed_dimensions",
            Self::ChangeSummary => "change_summary",
            Self::DifferenceReason => "difference_reason",
            Self::SideEffectClass => "side_effect_class",
            Self::BaselineRunRef => "baseline_run_ref",
        }
    }
}

// --- resolver input ---

/// One changed-context dimension within a rerun review.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5RerunChangeInput {
    /// Which execution dimension this row diffs.
    pub dimension: M5RerunChangeDimension,
    /// Whether the dimension is unchanged, changed, unknown, or not applicable.
    pub state: M5RerunChangeState,
    /// Human-readable prior value; required for a changed dimension.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub before_label: Option<String>,
    /// Human-readable new value; required for a changed or unknown dimension.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub after_label: Option<String>,
    /// A human-readable detail line describing the dimension's state.
    pub detail: String,
}

/// The full input to the rerun-review resolver for one bounded rerun of a prior run and
/// attempt.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5RerunReviewInput {
    /// The stable sheet identity that must survive across every projection.
    pub sheet_id: String,
    /// Opaque ref to the prior run identity; never raw run bytes.
    pub prior_run_ref: String,
    /// Opaque ref to the prior attempt identity; distinct from the prior run identity.
    pub prior_attempt_ref: String,
    /// 1-based ordinal of the prior attempt within the run.
    pub prior_attempt_ordinal: u32,
    /// 1-based ordinal the rerun's new attempt would take; must be after the prior one.
    pub new_attempt_ordinal: u32,
    /// The prior run's user-visible outcome.
    pub prior_run_outcome: M5RunOutcome,
    /// Opaque ref to the baseline run the sheet compares against.
    pub baseline_run_ref: String,
    /// Human-readable run label.
    pub run_label: String,
    /// Human-readable context summary.
    pub context_summary: String,
    /// Relative age label of the prior attempt ("2m ago").
    pub age_label: String,
    /// The captured-versus-live truth class of the sheet.
    pub truth_mode: M5ExecutionTruthMode,
    /// The local / remote / container / managed target boundary.
    pub target_boundary: M5ExecutionLocality,
    /// The reviewed rerun mode.
    pub rerun_mode: M5RerunMode,
    /// The rerun context (exact / current / modified).
    pub rerun_context: M5RerunContext,
    /// The retry scope of the rerun.
    pub retry_scope: M5RetryScope,
    /// The distinct reviewed actions the sheet offers; must not collapse to one generic
    /// action when the context has changed.
    pub available_modes: Vec<M5RerunMode>,
    /// The prior attempt's side-effect class.
    pub prior_side_effect_class: M5SideEffectClass,
    /// The rerun's side-effect class.
    pub rerun_side_effect_class: M5SideEffectClass,
    /// The reason the product believes the attempts differ (or that they are identical).
    pub difference_reason: String,
    /// The changed-context dimensions enumerated on the sheet.
    #[serde(default)]
    pub changed_dimensions: Vec<M5RerunChangeInput>,
    /// An externally-observed narrowing that degrades the surface.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub degraded: Option<DegradedState>,
}

// --- resolved projections ---

/// The resolved rerun comparison sheet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ResolvedRerunComparisonSheet {
    /// The sheet identity — identical to every other projection.
    pub sheet_id: String,
    /// The opaque prior-run ref.
    pub prior_run_ref: String,
    /// The opaque prior-attempt ref.
    pub prior_attempt_ref: String,
    /// The 1-based prior-attempt ordinal.
    pub prior_attempt_ordinal: u32,
    /// The 1-based new-attempt ordinal.
    pub new_attempt_ordinal: u32,
    /// The opaque baseline-run ref.
    pub baseline_run_ref: String,
    /// The prior run's user-visible outcome.
    pub prior_run_outcome: M5RunOutcome,
    /// The reviewed rerun mode.
    pub rerun_mode: M5RerunMode,
    /// The rerun context.
    pub rerun_context: M5RerunContext,
    /// The retry scope.
    pub retry_scope: M5RetryScope,
    /// The distinct reviewed actions offered.
    pub available_modes: Vec<M5RerunMode>,
    /// The reason the product believes the attempts differ.
    pub difference_reason: String,
    /// The prior side-effect class.
    pub prior_side_effect_class: M5SideEffectClass,
    /// The rerun side-effect class.
    pub rerun_side_effect_class: M5SideEffectClass,
    /// The rerun would escalate the side-effect class beyond the prior attempt.
    pub side_effect_escalates: bool,
    /// The current context has a reviewable change (a changed / unknown dimension or a
    /// side-effect escalation).
    pub context_has_changes: bool,
    /// The reviewed modes are semantically equivalent (no reviewable change), so they
    /// may be offered as one action.
    pub modes_semantically_equivalent: bool,
    /// A one-line changed-context summary safe to render and export.
    pub change_summary: String,
    /// The sheet discloses the exact-versus-current-context difference; always holds by
    /// construction.
    pub discloses_context_delta: bool,
    /// The context diff is shown before dispatch, never after; always holds by
    /// construction.
    pub context_diff_shown_before_dispatch: bool,
    /// The sheet keeps the reviewed actions distinct when they are not equivalent;
    /// always holds by construction.
    pub presents_distinct_actions: bool,
    /// The sheet cites the prior attempt, the new attempt, and the difference reason;
    /// always holds by construction.
    pub cites_prior_attempt: bool,
}

/// The resolved changed-context row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ResolvedRerunChangeRow {
    /// The sheet identity — identical to every other projection.
    pub sheet_id: String,
    /// The opaque prior-run ref.
    pub prior_run_ref: String,
    /// The opaque prior-attempt ref.
    pub prior_attempt_ref: String,
    /// Which execution dimension this row diffs.
    pub dimension: M5RerunChangeDimension,
    /// The dimension's change state.
    pub state: M5RerunChangeState,
    /// The prior value, when present.
    pub before_label: Option<String>,
    /// The new value, when present.
    pub after_label: Option<String>,
    /// A deterministic, human-readable change summary.
    pub change_summary: String,
    /// The row must be reviewed before dispatch (changed or unknown).
    pub requires_review: bool,
    /// The row is shown before dispatch; always holds by construction.
    pub shown_before_dispatch: bool,
}

/// The resolved CLI / headless line.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ResolvedRerunCliLine {
    /// The sheet identity — identical to every other projection.
    pub sheet_id: String,
    /// The opaque prior-run ref.
    pub prior_run_ref: String,
    /// The opaque prior-attempt ref.
    pub prior_attempt_ref: String,
    /// The deterministic single-line summary in the shared rerun vocabulary.
    pub line: String,
    /// The reviewed rerun mode.
    pub rerun_mode: M5RerunMode,
    /// The retry scope.
    pub retry_scope: M5RetryScope,
}

/// The resolved support-export projection.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ResolvedRerunExport {
    /// The sheet identity — identical to every other projection.
    pub sheet_id: String,
    /// The opaque prior-run ref — identical to every other projection.
    pub prior_run_ref: String,
    /// The opaque prior-attempt ref — identical to every other projection.
    pub prior_attempt_ref: String,
    /// The 1-based prior-attempt ordinal.
    pub prior_attempt_ordinal: u32,
    /// The 1-based new-attempt ordinal.
    pub new_attempt_ordinal: u32,
    /// The reviewed rerun mode.
    pub rerun_mode: M5RerunMode,
    /// The rerun context.
    pub rerun_context: M5RerunContext,
    /// The retry scope.
    pub retry_scope: M5RetryScope,
    /// The baseline run ref.
    pub baseline_run_ref: String,
    /// The prior run's user-visible outcome.
    pub prior_run_outcome: M5RunOutcome,
    /// The captured-versus-live truth class.
    pub truth_mode: M5ExecutionTruthMode,
    /// The target boundary.
    pub target_boundary: M5ExecutionLocality,
    /// The rerun's side-effect class.
    pub side_effect_class: M5SideEffectClass,
    /// The changed dimensions enumerated on the sheet.
    pub changed_dimensions: Vec<M5RerunChangeDimension>,
    /// The one-line changed-context summary.
    pub change_summary: String,
    /// The reason the product believes the attempts differ.
    pub difference_reason: String,
    /// The export fields this projection carries; includes the mandatory subset.
    pub export_fields: Vec<M5RerunExportField>,
}

/// The resolved rerun-review truth shared across the comparison sheet, the change rows,
/// the CLI line, and the support-export projection.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ResolvedRerunReview {
    /// The stable sheet identity.
    pub sheet_id: String,
    /// The opaque prior-run ref.
    pub prior_run_ref: String,
    /// The opaque prior-attempt ref.
    pub prior_attempt_ref: String,
    /// The 1-based prior-attempt ordinal.
    pub prior_attempt_ordinal: u32,
    /// The resolved comparison sheet.
    pub sheet: M5ResolvedRerunComparisonSheet,
    /// The resolved changed-context rows.
    pub change_rows: Vec<M5ResolvedRerunChangeRow>,
    /// The resolved CLI / headless line.
    pub cli_line: M5ResolvedRerunCliLine,
    /// The resolved support-export projection.
    pub export: M5ResolvedRerunExport,
    /// Rerun controls stay distinct reviewed actions when the context has changed (AC1).
    pub distinct_actions_preserved: bool,
    /// The changed execution context is reviewable before dispatch (AC2).
    pub context_reviewable_before_dispatch: bool,
    /// The reviewed rerun mode and changed-context summary survive into the export (AC3).
    pub export_preserves_mode_and_summary: bool,
    /// The narrowing carried through from the input, when present.
    pub degraded: Option<DegradedState>,
}

impl M5ResolvedRerunReview {
    /// True when the sheet identity, prior-run ref, and prior-attempt ref are identical
    /// across the sheet, the change rows, the CLI line, and the export.
    pub fn identity_consistent(&self) -> bool {
        let rows_ok = self.change_rows.iter().all(|row| {
            row.sheet_id == self.sheet_id
                && row.prior_run_ref == self.prior_run_ref
                && row.prior_attempt_ref == self.prior_attempt_ref
        });
        self.sheet.sheet_id == self.sheet_id
            && self.sheet.prior_run_ref == self.prior_run_ref
            && self.sheet.prior_attempt_ref == self.prior_attempt_ref
            && rows_ok
            && self.cli_line.sheet_id == self.sheet_id
            && self.cli_line.prior_run_ref == self.prior_run_ref
            && self.cli_line.prior_attempt_ref == self.prior_attempt_ref
            && self.export.sheet_id == self.sheet_id
            && self.export.prior_run_ref == self.prior_run_ref
            && self.export.prior_attempt_ref == self.prior_attempt_ref
    }

    /// True when the rerun controls do not collapse into one generic action once the
    /// context has changed: when a reviewable change exists, the exact and
    /// current-context actions stay distinct on the sheet (AC1). Holds trivially when
    /// the reviewed modes are semantically equivalent.
    pub fn distinguishes_rerun_actions(&self) -> bool {
        if !self.sheet.context_has_changes {
            return true;
        }
        self.sheet.presents_distinct_actions
            && self.sheet.available_modes.contains(&M5RerunMode::RerunExactly)
            && self
                .sheet
                .available_modes
                .contains(&M5RerunMode::RerunWithCurrentContext)
    }

    /// True when every changed / unknown dimension is enumerated with a summary and is
    /// shown before dispatch, so the changed execution context is reviewable before the
    /// rerun leaves the shell (AC2).
    pub fn discloses_context_delta_before_dispatch(&self) -> bool {
        self.sheet.context_diff_shown_before_dispatch
            && self.change_rows.iter().all(|row| {
                row.shown_before_dispatch
                    && (!row.state.requires_review() || !row.change_summary.trim().is_empty())
            })
            && (!self.sheet.rerun_context.differs_from_exact() || self.sheet.discloses_context_delta)
    }

    /// True when the sheet cites the prior attempt, the new attempt, and the reason the
    /// product believes the attempts differ, so prior lineage is preserved (AC3 lineage).
    pub fn preserves_prior_lineage(&self) -> bool {
        self.sheet.cites_prior_attempt
            && self.export.prior_attempt_ref == self.prior_attempt_ref
            && self.export.new_attempt_ordinal > self.export.prior_attempt_ordinal
            && !self.export.difference_reason.trim().is_empty()
    }
}

/// Errors returned by [`resolve_rerun_review`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum M5RerunReviewError {
    /// The sheet identity was empty.
    EmptySheetId,
    /// The prior-run ref was empty.
    EmptyPriorRunRef,
    /// The prior-attempt ref was empty.
    EmptyPriorAttemptRef,
    /// The baseline-run ref was empty.
    EmptyBaselineRef,
    /// The run label was empty.
    EmptyRunLabel,
    /// The context summary was empty.
    EmptyContextSummary,
    /// The age label was empty.
    EmptyAgeLabel,
    /// The difference reason was empty.
    EmptyDifferenceReason,
    /// The prior-attempt ordinal was zero.
    InvalidPriorOrdinal,
    /// The new-attempt ordinal was not after the prior one.
    NewAttemptNotAfterPrior,
    /// The prior-run ref and prior-attempt ref were equal — identity collapsed.
    RunAttemptIdentityCollapsed,
    /// A label, ref, or note carried forbidden material.
    ForbiddenMaterial,
    /// The rerun mode and rerun context disagreed.
    RerunModeContextMismatch,
    /// The retry scope did not match the rerun mode.
    RetryScopeInconsistentWithMode,
    /// A retry-failed-step rerun was offered for a run that did not fail.
    RetryFailedStepNotApplicable,
    /// The available modes did not include the chosen rerun mode.
    ChosenModeNotOffered,
    /// A mode appeared more than once in the available modes.
    DuplicateAvailableMode,
    /// The context changed but the distinct rerun actions collapsed into one generic
    /// action.
    DistinctRerunActionsCollapsed,
    /// A changed-context dimension appeared more than once.
    DuplicateChangeDimension,
    /// A changed-context row carried an empty detail.
    ChangeRowIncomplete,
    /// A changed dimension named no before / after delta.
    ChangedDimensionMissingDelta,
    /// A side-effect escalation was not disclosed as a reviewable change.
    SideEffectEscalationNotDisclosed,
    /// A degraded block carried a generic non-answer label.
    DegradedLabelGeneric,
}

impl M5RerunReviewError {
    /// Stable token for tests and diagnostics.
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::EmptySheetId => "empty_sheet_id",
            Self::EmptyPriorRunRef => "empty_prior_run_ref",
            Self::EmptyPriorAttemptRef => "empty_prior_attempt_ref",
            Self::EmptyBaselineRef => "empty_baseline_ref",
            Self::EmptyRunLabel => "empty_run_label",
            Self::EmptyContextSummary => "empty_context_summary",
            Self::EmptyAgeLabel => "empty_age_label",
            Self::EmptyDifferenceReason => "empty_difference_reason",
            Self::InvalidPriorOrdinal => "invalid_prior_ordinal",
            Self::NewAttemptNotAfterPrior => "new_attempt_not_after_prior",
            Self::RunAttemptIdentityCollapsed => "run_attempt_identity_collapsed",
            Self::ForbiddenMaterial => "forbidden_material",
            Self::RerunModeContextMismatch => "rerun_mode_context_mismatch",
            Self::RetryScopeInconsistentWithMode => "retry_scope_inconsistent_with_mode",
            Self::RetryFailedStepNotApplicable => "retry_failed_step_not_applicable",
            Self::ChosenModeNotOffered => "chosen_mode_not_offered",
            Self::DuplicateAvailableMode => "duplicate_available_mode",
            Self::DistinctRerunActionsCollapsed => "distinct_rerun_actions_collapsed",
            Self::DuplicateChangeDimension => "duplicate_change_dimension",
            Self::ChangeRowIncomplete => "change_row_incomplete",
            Self::ChangedDimensionMissingDelta => "changed_dimension_missing_delta",
            Self::SideEffectEscalationNotDisclosed => "side_effect_escalation_not_disclosed",
            Self::DegradedLabelGeneric => "degraded_label_generic",
        }
    }
}

impl fmt::Display for M5RerunReviewError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "rerun-review resolution error: {}", self.as_str())
    }
}

impl Error for M5RerunReviewError {}

/// Resolves one rerun review into its shared comparison sheet, changed-context rows,
/// CLI / headless line, and support-export projection.
///
/// The projections share one sheet identity, one prior-run ref, and one prior-attempt
/// ref, so reviewed rerun mode, changed-context diff, retry scope, and prior-attempt
/// lineage never blur. When the current context differs from an exact replay, the sheet
/// keeps the reviewed actions distinct; every changed dimension is shown before
/// dispatch; and the reviewed mode and changed-context summary survive into the export.
///
/// # Errors
///
/// Returns an [`M5RerunReviewError`] when identity is missing or collapsed, the rerun
/// mode disagrees with its context or retry scope, a changed context collapses the
/// distinct actions or hides its delta, a side-effect escalation is not disclosed, or
/// any ref / label carries forbidden material.
pub fn resolve_rerun_review(
    input: &M5RerunReviewInput,
) -> Result<M5ResolvedRerunReview, M5RerunReviewError> {
    if input.sheet_id.trim().is_empty() {
        return Err(M5RerunReviewError::EmptySheetId);
    }
    if input.prior_run_ref.trim().is_empty() {
        return Err(M5RerunReviewError::EmptyPriorRunRef);
    }
    if input.prior_attempt_ref.trim().is_empty() {
        return Err(M5RerunReviewError::EmptyPriorAttemptRef);
    }
    if input.baseline_run_ref.trim().is_empty() {
        return Err(M5RerunReviewError::EmptyBaselineRef);
    }
    if input.run_label.trim().is_empty() {
        return Err(M5RerunReviewError::EmptyRunLabel);
    }
    if input.context_summary.trim().is_empty() {
        return Err(M5RerunReviewError::EmptyContextSummary);
    }
    if input.age_label.trim().is_empty() {
        return Err(M5RerunReviewError::EmptyAgeLabel);
    }
    if input.difference_reason.trim().is_empty() {
        return Err(M5RerunReviewError::EmptyDifferenceReason);
    }
    if input.prior_attempt_ordinal == 0 {
        return Err(M5RerunReviewError::InvalidPriorOrdinal);
    }
    if input.new_attempt_ordinal <= input.prior_attempt_ordinal {
        return Err(M5RerunReviewError::NewAttemptNotAfterPrior);
    }
    if input.prior_run_ref.trim() == input.prior_attempt_ref.trim() {
        return Err(M5RerunReviewError::RunAttemptIdentityCollapsed);
    }

    for value in [
        input.prior_run_ref.as_str(),
        input.prior_attempt_ref.as_str(),
        input.baseline_run_ref.as_str(),
        input.run_label.as_str(),
        input.context_summary.as_str(),
        input.age_label.as_str(),
        input.difference_reason.as_str(),
    ] {
        if value_is_forbidden(value) {
            return Err(M5RerunReviewError::ForbiddenMaterial);
        }
    }

    // The reviewed mode and its context must agree: an exact replay is the exact
    // context; every other mode reruns against a context that differs from exact.
    if !mode_context_consistent(input.rerun_mode, input.rerun_context) {
        return Err(M5RerunReviewError::RerunModeContextMismatch);
    }
    // The reviewed mode and its retry scope must agree.
    if !mode_scope_consistent(input.rerun_mode, input.retry_scope) {
        return Err(M5RerunReviewError::RetryScopeInconsistentWithMode);
    }
    // A retry-failed-step rerun is only offered when the prior run actually failed or
    // partially completed.
    if input.rerun_mode == M5RerunMode::RetryFailedStepOnly
        && !prior_supports_failed_step_retry(input.prior_run_outcome)
    {
        return Err(M5RerunReviewError::RetryFailedStepNotApplicable);
    }

    // The available modes must be a distinct set that includes the chosen mode.
    let mut seen_modes: BTreeSet<M5RerunMode> = BTreeSet::new();
    for mode in &input.available_modes {
        if !seen_modes.insert(*mode) {
            return Err(M5RerunReviewError::DuplicateAvailableMode);
        }
    }
    if !seen_modes.contains(&input.rerun_mode) {
        return Err(M5RerunReviewError::ChosenModeNotOffered);
    }

    let side_effect_escalates = input
        .rerun_side_effect_class
        .escalates_beyond(input.prior_side_effect_class);

    let change_rows = resolve_change_rows(input)?;

    // A side-effect escalation must be disclosed as a reviewable side-effect-class row.
    if side_effect_escalates
        && !change_rows.iter().any(|row| {
            row.dimension == M5RerunChangeDimension::SideEffectClass && row.state.requires_review()
        })
    {
        return Err(M5RerunReviewError::SideEffectEscalationNotDisclosed);
    }

    let context_has_changes =
        change_rows.iter().any(|row| row.requires_review) || side_effect_escalates;

    // AC1: once the context has changed, the exact and current-context actions must stay
    // distinct reviewed actions rather than collapsing into one generic control.
    if context_has_changes
        && (!seen_modes.contains(&M5RerunMode::RerunExactly)
            || !seen_modes.contains(&M5RerunMode::RerunWithCurrentContext))
    {
        return Err(M5RerunReviewError::DistinctRerunActionsCollapsed);
    }

    if let Some(degraded) = &input.degraded {
        if !degraded.is_honest() {
            return Err(M5RerunReviewError::DegradedLabelGeneric);
        }
    }

    let change_summary = render_change_summary(input, &change_rows, side_effect_escalates);
    let modes_semantically_equivalent = !context_has_changes;

    let sheet = M5ResolvedRerunComparisonSheet {
        sheet_id: input.sheet_id.clone(),
        prior_run_ref: input.prior_run_ref.clone(),
        prior_attempt_ref: input.prior_attempt_ref.clone(),
        prior_attempt_ordinal: input.prior_attempt_ordinal,
        new_attempt_ordinal: input.new_attempt_ordinal,
        baseline_run_ref: input.baseline_run_ref.clone(),
        prior_run_outcome: input.prior_run_outcome,
        rerun_mode: input.rerun_mode,
        rerun_context: input.rerun_context,
        retry_scope: input.retry_scope,
        available_modes: input.available_modes.clone(),
        difference_reason: input.difference_reason.clone(),
        prior_side_effect_class: input.prior_side_effect_class,
        rerun_side_effect_class: input.rerun_side_effect_class,
        side_effect_escalates,
        context_has_changes,
        modes_semantically_equivalent,
        change_summary: change_summary.clone(),
        discloses_context_delta: true,
        context_diff_shown_before_dispatch: true,
        presents_distinct_actions: true,
        cites_prior_attempt: true,
    };

    let cli_line = M5ResolvedRerunCliLine {
        sheet_id: input.sheet_id.clone(),
        prior_run_ref: input.prior_run_ref.clone(),
        prior_attempt_ref: input.prior_attempt_ref.clone(),
        line: render_cli_line(input, &change_rows, side_effect_escalates),
        rerun_mode: input.rerun_mode,
        retry_scope: input.retry_scope,
    };

    let export = M5ResolvedRerunExport {
        sheet_id: input.sheet_id.clone(),
        prior_run_ref: input.prior_run_ref.clone(),
        prior_attempt_ref: input.prior_attempt_ref.clone(),
        prior_attempt_ordinal: input.prior_attempt_ordinal,
        new_attempt_ordinal: input.new_attempt_ordinal,
        rerun_mode: input.rerun_mode,
        rerun_context: input.rerun_context,
        retry_scope: input.retry_scope,
        baseline_run_ref: input.baseline_run_ref.clone(),
        prior_run_outcome: input.prior_run_outcome,
        truth_mode: input.truth_mode,
        target_boundary: input.target_boundary,
        side_effect_class: input.rerun_side_effect_class,
        changed_dimensions: change_rows
            .iter()
            .filter(|row| row.requires_review)
            .map(|row| row.dimension)
            .collect(),
        change_summary,
        difference_reason: input.difference_reason.clone(),
        export_fields: M5RerunExportField::ALL.to_vec(),
    };

    Ok(M5ResolvedRerunReview {
        sheet_id: input.sheet_id.clone(),
        prior_run_ref: input.prior_run_ref.clone(),
        prior_attempt_ref: input.prior_attempt_ref.clone(),
        prior_attempt_ordinal: input.prior_attempt_ordinal,
        sheet,
        change_rows,
        cli_line,
        export,
        distinct_actions_preserved: true,
        context_reviewable_before_dispatch: true,
        export_preserves_mode_and_summary: true,
        degraded: input.degraded.clone(),
    })
}

fn resolve_change_rows(
    input: &M5RerunReviewInput,
) -> Result<Vec<M5ResolvedRerunChangeRow>, M5RerunReviewError> {
    let mut seen_dimensions: BTreeSet<M5RerunChangeDimension> = BTreeSet::new();
    let mut rows = Vec::with_capacity(input.changed_dimensions.len());
    for change in &input.changed_dimensions {
        if change.detail.trim().is_empty() {
            return Err(M5RerunReviewError::ChangeRowIncomplete);
        }
        for value in [change.detail.as_str()]
            .into_iter()
            .chain(change.before_label.as_deref())
            .chain(change.after_label.as_deref())
        {
            if value_is_forbidden(value) {
                return Err(M5RerunReviewError::ForbiddenMaterial);
            }
        }
        // A changed dimension must name both its prior and new value so the delta is
        // reviewable before dispatch.
        if change.state.is_changed()
            && (change
                .before_label
                .as_deref()
                .map_or(true, |label| label.trim().is_empty())
                || change
                    .after_label
                    .as_deref()
                    .map_or(true, |label| label.trim().is_empty()))
        {
            return Err(M5RerunReviewError::ChangedDimensionMissingDelta);
        }
        if !seen_dimensions.insert(change.dimension) {
            return Err(M5RerunReviewError::DuplicateChangeDimension);
        }

        rows.push(M5ResolvedRerunChangeRow {
            sheet_id: input.sheet_id.clone(),
            prior_run_ref: input.prior_run_ref.clone(),
            prior_attempt_ref: input.prior_attempt_ref.clone(),
            dimension: change.dimension,
            state: change.state,
            before_label: change.before_label.clone(),
            after_label: change.after_label.clone(),
            change_summary: render_change_row_summary(change),
            requires_review: change.state.requires_review(),
            shown_before_dispatch: true,
        });
    }
    Ok(rows)
}

/// True when a rerun mode and rerun context are consistent: an exact replay pairs with
/// the exact-replay context; every other mode pairs with a context that differs from an
/// exact replay.
fn mode_context_consistent(mode: M5RerunMode, context: M5RerunContext) -> bool {
    match mode {
        M5RerunMode::RerunExactly => context == M5RerunContext::ExactReplay,
        M5RerunMode::RerunWithCurrentContext | M5RerunMode::RetryFailedStepOnly => {
            context.differs_from_exact()
        }
    }
}

/// True when a rerun mode and its retry scope are consistent.
fn mode_scope_consistent(mode: M5RerunMode, scope: M5RetryScope) -> bool {
    match mode {
        M5RerunMode::RerunExactly => scope == M5RetryScope::WholeRun,
        M5RerunMode::RerunWithCurrentContext => {
            matches!(scope, M5RetryScope::WholeRun | M5RetryScope::SelectedSubset)
        }
        M5RerunMode::RetryFailedStepOnly => scope == M5RetryScope::FailedStepOnly,
    }
}

/// True when a prior outcome can offer a retry of only the failed step / units.
fn prior_supports_failed_step_retry(outcome: M5RunOutcome) -> bool {
    matches!(outcome, M5RunOutcome::Failed | M5RunOutcome::PartiallyComplete)
}

/// A deterministic, human-readable summary for one changed-context row.
fn render_change_row_summary(change: &M5RerunChangeInput) -> String {
    match (change.state, change.before_label.as_deref(), change.after_label.as_deref()) {
        (M5RerunChangeState::Changed, Some(before), Some(after)) => format!(
            "{} changed: {} → {} ({})",
            change.dimension.as_str(),
            before,
            after,
            change.detail
        ),
        (M5RerunChangeState::Unknown, _, Some(after)) => format!(
            "{} could not be confirmed unchanged; now {} ({})",
            change.dimension.as_str(),
            after,
            change.detail
        ),
        (M5RerunChangeState::Unknown, _, None) => format!(
            "{} could not be confirmed unchanged ({})",
            change.dimension.as_str(),
            change.detail
        ),
        (M5RerunChangeState::Unchanged, _, _) => {
            format!("{} unchanged ({})", change.dimension.as_str(), change.detail)
        }
        (M5RerunChangeState::NotApplicable, _, _) => {
            format!("{} not applicable ({})", change.dimension.as_str(), change.detail)
        }
        (M5RerunChangeState::Changed, _, _) => {
            format!("{} changed ({})", change.dimension.as_str(), change.detail)
        }
    }
}

/// Builds the one-line changed-context summary safe to render and export.
fn render_change_summary(
    input: &M5RerunReviewInput,
    rows: &[M5ResolvedRerunChangeRow],
    side_effect_escalates: bool,
) -> String {
    let changed: Vec<&str> = rows
        .iter()
        .filter(|row| row.requires_review)
        .map(|row| row.dimension.as_str())
        .collect();
    let base = if changed.is_empty() {
        format!(
            "no reviewable context change; {} of the prior attempt",
            input.rerun_mode.label().to_lowercase()
        )
    } else {
        format!(
            "{} to review before {}: {}",
            changed.len(),
            input.rerun_mode.label().to_lowercase(),
            changed.join(", ")
        )
    };
    if side_effect_escalates {
        format!(
            "{base}; side effects escalate {} → {}",
            input.prior_side_effect_class.as_str(),
            input.rerun_side_effect_class.as_str()
        )
    } else {
        base
    }
}

/// Renders the deterministic CLI / headless line in the shared rerun vocabulary.
fn render_cli_line(
    input: &M5RerunReviewInput,
    rows: &[M5ResolvedRerunChangeRow],
    side_effect_escalates: bool,
) -> String {
    let changed = rows.iter().filter(|row| row.requires_review).count();
    format!(
        "sheet={sheet} prior_run={run} prior_attempt=#{prior} new_attempt=#{new} mode={mode} \
context={context} scope={scope} changed={changed} escalates={escalates} baseline={baseline}",
        sheet = input.sheet_id,
        run = input.prior_run_ref,
        prior = input.prior_attempt_ordinal,
        new = input.new_attempt_ordinal,
        mode = input.rerun_mode.as_str(),
        context = input.rerun_context.as_str(),
        scope = input.retry_scope.as_str(),
        changed = changed,
        escalates = side_effect_escalates,
        baseline = input.baseline_run_ref,
    )
}

/// True when a slice of export fields declares every mandatory field.
fn declares_mandatory_export_fields(fields: &[M5RerunExportField]) -> bool {
    let present: BTreeSet<M5RerunExportField> = fields.iter().copied().collect();
    M5RerunExportField::MANDATORY
        .iter()
        .all(|field| present.contains(field))
}

/// True when a label, ref, or note carries obviously forbidden material.
fn value_is_forbidden(value: &str) -> bool {
    let lower = value.to_lowercase();
    lower.contains("api_key")
        || lower.contains("password")
        || lower.contains("secret=")
        || lower.contains("bearer ")
        || lower.contains("://")
        || lower.contains("-----begin")
}

/// One worked resolution case carried in the packet so the support / export packet
/// reconstructs rerun truth from the shared model.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5RerunReviewCase {
    /// The resolver input.
    pub input: M5RerunReviewInput,
    /// The resolved review. Must equal `resolve_rerun_review(&input)`.
    pub resolved: M5ResolvedRerunReview,
}

impl M5RerunReviewCase {
    /// Builds a case by resolving `input`.
    ///
    /// # Panics
    ///
    /// Panics if `input` does not resolve; seed inputs are always valid.
    pub fn resolved(input: M5RerunReviewInput) -> Self {
        let resolved = resolve_rerun_review(&input).expect("seed rerun-review case is valid");
        Self { input, resolved }
    }

    /// True when the stored resolution matches a fresh resolve of the input.
    pub fn is_self_consistent(&self) -> bool {
        resolve_rerun_review(&self.input).as_ref() == Ok(&self.resolved)
    }
}

/// One row in the primitive matrix: one execution surface family bound to the shared
/// rerun-comparison contract.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5RerunSurfaceRow {
    /// The execution surface family.
    pub surface_family: M5RunAttemptSurfaceFamily,
    /// Owner role accountable for keeping this surface governed.
    pub owner_role: String,
    /// Human-readable scope summary.
    pub scope_summary: String,
    /// Rerun modes this surface can offer (must be non-empty).
    pub rerun_modes: Vec<M5RerunMode>,
    /// Change dimensions this surface can diff (may be empty for exact-only surfaces).
    pub change_dimensions: Vec<M5RerunChangeDimension>,
    /// Export fields this row carries (must include the mandatory fields).
    pub export_fields: Vec<M5RerunExportField>,
    /// Downgrade triggers that apply to this surface (must be non-empty).
    pub downgrade_triggers: Vec<M5ExecutionDowngradeTrigger>,
    /// Consumer surfaces that ingest this row's projection (must be non-empty).
    pub consumer_surfaces: Vec<String>,
    /// Source contract refs consumed by this row (must be non-empty).
    pub source_contract_refs: Vec<String>,
    /// Worked resolution cases proving the resolver on this surface (must be non-empty).
    pub example_reruns: Vec<M5RerunReviewCase>,
    /// Hard invariant: this row never collapses the distinct reviewed actions. MUST be
    /// `false`.
    pub collapses_distinct_actions: bool,
    /// Hard invariant: this row never hides changed execution context. MUST be `false`.
    pub hides_changed_context: bool,
    /// Hard invariant: this row never drops prior-attempt lineage. MUST be `false`.
    pub drops_prior_lineage: bool,
    /// Hard invariant: this row never drops the exported rerun mode or summary. MUST be
    /// `false`.
    pub drops_export_mode_or_summary: bool,
}

impl M5RerunSurfaceRow {
    /// True when the row declares every mandatory export field.
    fn declares_mandatory_export_fields(&self) -> bool {
        declares_mandatory_export_fields(&self.export_fields)
    }

    /// True when the row's hard invariants hold.
    fn honours_invariants(&self) -> bool {
        !self.collapses_distinct_actions
            && !self.hides_changed_context
            && !self.drops_prior_lineage
            && !self.drops_export_mode_or_summary
    }
}

/// Self-describing controlled-vocabulary set minted / reused by this primitive.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5RerunVocabularySet {
    /// Surface-family tokens (reused from the run/attempt-header primitive).
    pub surface_families: Vec<String>,
    /// Rerun-mode tokens.
    pub rerun_modes: Vec<String>,
    /// Rerun-context tokens (reused from the frozen matrix).
    pub rerun_contexts: Vec<String>,
    /// Retry-scope tokens.
    pub retry_scopes: Vec<String>,
    /// Change-dimension tokens.
    pub change_dimensions: Vec<String>,
    /// Change-state tokens.
    pub change_states: Vec<String>,
    /// Side-effect-class tokens.
    pub side_effect_classes: Vec<String>,
    /// Export-field tokens.
    pub export_fields: Vec<String>,
    /// Run-outcome tokens (reused from the frozen matrix).
    pub outcomes: Vec<String>,
    /// Truth-class tokens (reused from the frozen matrix).
    pub truth_modes: Vec<String>,
    /// Execution-boundary tokens (reused from the frozen matrix).
    pub localities: Vec<String>,
    /// Downgrade-trigger tokens (reused from the frozen matrix).
    pub downgrade_triggers: Vec<String>,
}

impl M5RerunVocabularySet {
    /// Builds the canonical vocabulary set from the typed `ALL` arrays.
    pub fn canonical() -> Self {
        Self {
            surface_families: tokens(&M5RunAttemptSurfaceFamily::ALL, |v| v.as_str()),
            rerun_modes: tokens(&M5RerunMode::ALL, |v| v.as_str()),
            rerun_contexts: tokens(&RERUN_CONTEXT_ALL, |v| v.as_str()),
            retry_scopes: tokens(&M5RetryScope::ALL, |v| v.as_str()),
            change_dimensions: tokens(&M5RerunChangeDimension::ALL, |v| v.as_str()),
            change_states: tokens(&M5RerunChangeState::ALL, |v| v.as_str()),
            side_effect_classes: tokens(&M5SideEffectClass::ALL, |v| v.as_str()),
            export_fields: tokens(&M5RerunExportField::ALL, |v| v.as_str()),
            outcomes: tokens(&M5RunOutcome::ALL, |v| v.as_str()),
            truth_modes: tokens(&TRUTH_MODE_ALL, |v| v.as_str()),
            localities: tokens(&LOCALITY_ALL, |v| v.as_str()),
            downgrade_triggers: tokens(&DOWNGRADE_TRIGGER_ALL, |v| v.as_str()),
        }
    }

    /// Returns true when this set matches the canonical token lists exactly.
    pub fn matches_canonical(&self) -> bool {
        *self == Self::canonical()
    }
}

/// The rerun contexts reused from the frozen matrix, in a stable order.
const RERUN_CONTEXT_ALL: [M5RerunContext; 4] = [
    M5RerunContext::ExactReplay,
    M5RerunContext::CurrentContext,
    M5RerunContext::ModifiedSelection,
    M5RerunContext::ModifiedEnvironment,
];

/// The truth classes reused from the frozen matrix, in a stable order.
const TRUTH_MODE_ALL: [M5ExecutionTruthMode; 5] = [
    M5ExecutionTruthMode::Live,
    M5ExecutionTruthMode::Captured,
    M5ExecutionTruthMode::Imported,
    M5ExecutionTruthMode::Planned,
    M5ExecutionTruthMode::ProviderReported,
];

/// The execution boundaries reused from the frozen matrix, in a stable order.
const LOCALITY_ALL: [M5ExecutionLocality; 4] = [
    M5ExecutionLocality::Local,
    M5ExecutionLocality::Remote,
    M5ExecutionLocality::Container,
    M5ExecutionLocality::Managed,
];

/// The downgrade triggers reused from the frozen matrix, in a stable order.
const DOWNGRADE_TRIGGER_ALL: [M5ExecutionDowngradeTrigger; 9] = [
    M5ExecutionDowngradeTrigger::RunAttemptIdentityUnresolved,
    M5ExecutionDowngradeTrigger::InputConsequenceUnknown,
    M5ExecutionDowngradeTrigger::ArtifactLineageLost,
    M5ExecutionDowngradeTrigger::ArtifactRetentionExpired,
    M5ExecutionDowngradeTrigger::RerunContextDrift,
    M5ExecutionDowngradeTrigger::CapturedEvidenceOnly,
    M5ExecutionDowngradeTrigger::ConnectorLost,
    M5ExecutionDowngradeTrigger::DebugAdapterUnavailable,
    M5ExecutionDowngradeTrigger::SymbolsUnavailable,
];

fn tokens<T: Copy>(items: &[T], to_token: impl Fn(T) -> &'static str) -> Vec<String> {
    items.iter().map(|v| to_token(*v).to_owned()).collect()
}

/// Governance-review block; every flag is a hard invariant.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5RerunGovernanceReview {
    /// One primitive carries sheet / change-row / CLI-line / export truth on every
    /// surface.
    pub one_primitive_carries_all_surfaces: bool,
    /// Distinct reviewed actions are never collapsed into one generic control once the
    /// context has changed.
    pub distinct_rerun_actions_never_collapsed: bool,
    /// The changed execution context is reviewable before dispatch.
    pub changed_context_reviewable_before_dispatch: bool,
    /// Prior-attempt lineage is preserved so the sheet can cite the earlier attempt.
    pub prior_attempt_lineage_preserved: bool,
    /// The reviewed rerun mode and changed-context summary survive into the export.
    pub reviewed_mode_and_summary_survive_export: bool,
    /// The support / export packet reconstructs rerun truth.
    pub support_export_reconstructs_rerun: bool,
    /// Later M5 rows cannot invent parallel rerun / retry vocabulary.
    pub later_rows_cannot_invent_parallel_vocabulary: bool,
}

/// Consumer projection block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5RerunConsumerProjection {
    /// Task / test / request / notebook / AI / publish / preview surfaces all consume
    /// the shared primitive.
    pub execution_surfaces_consume_shared_primitive: bool,
    /// The rerun resolver reads a single canonical model.
    pub resolver_reads_single_model: bool,
    /// The change rows read a single canonical diff source.
    pub change_rows_read_single_diff_source: bool,
    /// Support / export reads a single canonical rerun source.
    pub support_export_reads_single_source: bool,
}

/// Release and support parity posture for the rerun primitive.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5RerunReleasePosture {
    /// Ref of the supporting release packet.
    pub release_packet_ref: String,
    /// Ref of the supporting rerun audit.
    pub rerun_audit_ref: String,
    /// True when support / export parity is required for every surface.
    pub support_export_parity_required: bool,
    /// True when accessibility parity is required for every surface.
    pub accessibility_parity_required: bool,
}

/// Constructor input for [`M5RerunReviewPrimitivePacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M5RerunReviewPrimitivePacketInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable matrix label.
    pub matrix_label: String,
    /// Surface rows.
    pub surface_rows: Vec<M5RerunSurfaceRow>,
    /// Frozen controlled-vocabulary set.
    pub vocabulary_set: M5RerunVocabularySet,
    /// Governance-review block.
    pub governance_review: M5RerunGovernanceReview,
    /// Consumer projection block.
    pub consumer_projection: M5RerunConsumerProjection,
    /// Release and support parity posture.
    pub release_posture: M5RerunReleasePosture,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

/// Export-safe M5 rerun-comparison-sheet primitive packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5RerunReviewPrimitivePacket {
    /// Record kind; must equal [`M5_RERUN_REVIEW_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`M5_RERUN_REVIEW_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable matrix label.
    pub matrix_label: String,
    /// Surface rows.
    pub surface_rows: Vec<M5RerunSurfaceRow>,
    /// Frozen controlled-vocabulary set.
    pub vocabulary_set: M5RerunVocabularySet,
    /// Governance-review block.
    pub governance_review: M5RerunGovernanceReview,
    /// Consumer projection block.
    pub consumer_projection: M5RerunConsumerProjection,
    /// Release and support parity posture.
    pub release_posture: M5RerunReleasePosture,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

impl M5RerunReviewPrimitivePacket {
    /// Builds an M5 rerun primitive packet from stable-lane input.
    pub fn new(input: M5RerunReviewPrimitivePacketInput) -> Self {
        Self {
            record_kind: M5_RERUN_REVIEW_RECORD_KIND.to_owned(),
            schema_version: M5_RERUN_REVIEW_SCHEMA_VERSION,
            packet_id: input.packet_id,
            matrix_label: input.matrix_label,
            surface_rows: input.surface_rows,
            vocabulary_set: input.vocabulary_set,
            governance_review: input.governance_review,
            consumer_projection: input.consumer_projection,
            release_posture: input.release_posture,
            source_contract_refs: input.source_contract_refs,
            redaction_class_token: input.redaction_class_token,
            minted_at: input.minted_at,
        }
    }

    /// Validates the M5 rerun primitive invariants.
    pub fn validate(&self) -> Vec<M5RerunViolation> {
        let mut violations = Vec::new();

        if self.record_kind != M5_RERUN_REVIEW_RECORD_KIND {
            violations.push(M5RerunViolation::WrongRecordKind);
        }
        if self.schema_version != M5_RERUN_REVIEW_SCHEMA_VERSION {
            violations.push(M5RerunViolation::WrongSchemaVersion);
        }
        if self.packet_id.trim().is_empty()
            || self.matrix_label.trim().is_empty()
            || self.redaction_class_token.trim().is_empty()
            || self.minted_at.trim().is_empty()
        {
            violations.push(M5RerunViolation::MissingIdentity);
        }

        validate_source_contracts(self, &mut violations);
        validate_vocabulary_set(self, &mut violations);
        validate_surface_rows(self, &mut violations);
        validate_acceptance_criteria_covered(self, &mut violations);
        validate_governance_review(self, &mut violations);
        validate_consumer_projection(self, &mut violations);
        validate_release_posture(self, &mut violations);

        if json_contains_forbidden_material(
            &serde_json::to_value(self).expect("m5 rerun primitive packet serializes"),
        ) {
            violations.push(M5RerunViolation::RawMaterialInExport);
        }

        violations
    }

    /// Deterministic export-safe JSON.
    ///
    /// # Panics
    ///
    /// Panics only if serializing this metadata-only packet fails.
    pub fn export_safe_json(&self) -> String {
        serde_json::to_string_pretty(self).expect("m5 rerun primitive packet serializes")
    }

    /// Deterministic, machine-readable matrix CSV: one row per surface family.
    pub fn render_matrix_csv(&self) -> String {
        let mut out = String::new();
        out.push_str("surface_family,owner,rerun_modes,change_dimensions,example_count\n");
        for row in &self.surface_rows {
            out.push_str(&format!(
                "{},{},{},{},{}\n",
                row.surface_family.as_str(),
                csv_field(&row.owner_role),
                join_tokens(&row.rerun_modes, |v| v.as_str()),
                join_tokens(&row.change_dimensions, |v| v.as_str()),
                row.example_reruns.len(),
            ));
        }
        out
    }

    /// Deterministic Markdown report for support, docs, or review handoff.
    pub fn render_markdown_summary(&self) -> String {
        let mut out = String::new();
        out.push_str("# M5 Rerun-Comparison-Sheet Primitive\n\n");
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- Label: `{}`\n", self.matrix_label));
        out.push_str(&format!(
            "- Execution surfaces: {} / {}\n",
            self.surface_rows.len(),
            M5RunAttemptSurfaceFamily::ALL.len(),
        ));
        out.push_str(&format!(
            "- Rerun modes: {}\n",
            self.vocabulary_set.rerun_modes.join(", ")
        ));
        out.push_str(&format!(
            "- Change dimensions: {}\n",
            self.vocabulary_set.change_dimensions.join(", ")
        ));
        out.push_str("\n## Execution surfaces\n\n");
        for row in &self.surface_rows {
            out.push_str(&format!("- **{}**\n", row.surface_family.label()));
            out.push_str(&format!("  - Owner: {}\n", row.owner_role));
            out.push_str(&format!("  - Scope: {}\n", row.scope_summary));
            out.push_str(&format!("  - Worked cases: {}\n", row.example_reruns.len()));
            for case in &row.example_reruns {
                out.push_str(&format!(
                    "    - `{}` → prior `{}` [{}], {} → {} ({} changed dim(s))\n",
                    case.resolved.sheet_id,
                    case.resolved.prior_run_ref,
                    case.resolved.export.prior_run_outcome.as_str(),
                    case.resolved.sheet.rerun_mode.as_str(),
                    case.resolved.sheet.retry_scope.as_str(),
                    case.resolved.export.changed_dimensions.len(),
                ));
            }
        }
        out
    }
}

/// Errors emitted when reading the checked-in M5 rerun export.
#[derive(Debug)]
pub enum M5RerunArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<M5RerunViolation>),
}

impl fmt::Display for M5RerunArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(formatter, "m5 rerun primitive export parse failed: {error}")
            }
            Self::Validation(violations) => {
                let tokens = violations
                    .iter()
                    .map(|violation| violation.as_str())
                    .collect::<Vec<_>>()
                    .join(",");
                write!(
                    formatter,
                    "m5 rerun primitive export failed validation: {tokens}"
                )
            }
        }
    }
}

impl Error for M5RerunArtifactError {}

/// Validation failures emitted by [`M5RerunReviewPrimitivePacket::validate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum M5RerunViolation {
    /// Packet record kind is wrong.
    WrongRecordKind,
    /// Packet schema version is wrong.
    WrongSchemaVersion,
    /// Required identity field is missing.
    MissingIdentity,
    /// Source contract refs are incomplete.
    MissingSourceContracts,
    /// The frozen vocabulary set drifted from the canonical token lists.
    VocabularySetDrift,
    /// A required execution surface family is missing from the matrix.
    RequiredSurfaceMissing,
    /// A surface row is incomplete.
    SurfaceRowIncomplete,
    /// A surface row omits one of the mandatory export fields.
    MandatoryExportFieldMissing,
    /// A surface row declares no rerun modes.
    RerunModesMissing,
    /// A surface row declares no downgrade triggers.
    DowngradeTriggersMissing,
    /// A surface row declares no consumer surfaces.
    ConsumerSurfacesMissing,
    /// A surface row declares no worked rerun cases.
    ExampleRerunsMissing,
    /// A worked rerun case does not match a fresh resolve of its input.
    ExampleRerunDrift,
    /// A surface row violates a hard invariant.
    SurfaceInvariantViolated,
    /// No worked case proves distinct reviewed actions survive a changed context (AC1),
    /// or a rerun mode is not covered across the matrix.
    DistinctActionsUnproven,
    /// No worked case proves the changed context is reviewable before dispatch (AC2), or
    /// the change dimensions are not fully covered.
    ContextReviewUnproven,
    /// No worked case proves the reviewed mode and summary survive the export (AC3).
    ExportPreservationUnproven,
    /// Governance review does not satisfy required invariants.
    GovernanceReviewIncomplete,
    /// Consumer projection does not satisfy required invariants.
    ConsumerProjectionIncomplete,
    /// Release / support parity posture is incomplete.
    ReleasePostureIncomplete,
    /// Export contains raw sensitive material.
    RawMaterialInExport,
}

impl M5RerunViolation {
    /// Stable token used in tests and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WrongRecordKind => "wrong_record_kind",
            Self::WrongSchemaVersion => "wrong_schema_version",
            Self::MissingIdentity => "missing_identity",
            Self::MissingSourceContracts => "missing_source_contracts",
            Self::VocabularySetDrift => "vocabulary_set_drift",
            Self::RequiredSurfaceMissing => "required_surface_missing",
            Self::SurfaceRowIncomplete => "surface_row_incomplete",
            Self::MandatoryExportFieldMissing => "mandatory_export_field_missing",
            Self::RerunModesMissing => "rerun_modes_missing",
            Self::DowngradeTriggersMissing => "downgrade_triggers_missing",
            Self::ConsumerSurfacesMissing => "consumer_surfaces_missing",
            Self::ExampleRerunsMissing => "example_reruns_missing",
            Self::ExampleRerunDrift => "example_rerun_drift",
            Self::SurfaceInvariantViolated => "surface_invariant_violated",
            Self::DistinctActionsUnproven => "distinct_actions_unproven",
            Self::ContextReviewUnproven => "context_review_unproven",
            Self::ExportPreservationUnproven => "export_preservation_unproven",
            Self::GovernanceReviewIncomplete => "governance_review_incomplete",
            Self::ConsumerProjectionIncomplete => "consumer_projection_incomplete",
            Self::ReleasePostureIncomplete => "release_posture_incomplete",
            Self::RawMaterialInExport => "raw_material_in_export",
        }
    }
}

/// Reads and validates the checked-in stable M5 rerun export.
///
/// # Errors
///
/// Returns an artifact error if the export cannot parse or fails validation.
pub fn current_stable_m5_rerun_review_export(
) -> Result<M5RerunReviewPrimitivePacket, M5RerunArtifactError> {
    let packet: M5RerunReviewPrimitivePacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/release/m5-rerun-comparison-sheet-primitive-proof/support_export.json"
    )))
    .map_err(M5RerunArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(M5RerunArtifactError::Validation(violations))
    }
}

fn validate_source_contracts(
    packet: &M5RerunReviewPrimitivePacket,
    violations: &mut Vec<M5RerunViolation>,
) {
    let refs: BTreeSet<&str> = packet
        .source_contract_refs
        .iter()
        .map(String::as_str)
        .collect();
    for required in [
        M5_RERUN_REVIEW_SCHEMA_REF,
        M5_RERUN_REVIEW_DOC_REF,
        M5_RERUN_REVIEW_COMPONENT_MATRIX_REF,
        M5_RERUN_REVIEW_ARTIFACT_REF,
    ] {
        if !refs.contains(required) {
            violations.push(M5RerunViolation::MissingSourceContracts);
            return;
        }
    }
}

fn validate_vocabulary_set(
    packet: &M5RerunReviewPrimitivePacket,
    violations: &mut Vec<M5RerunViolation>,
) {
    if !packet.vocabulary_set.matches_canonical() {
        violations.push(M5RerunViolation::VocabularySetDrift);
    }
}

fn validate_surface_rows(
    packet: &M5RerunReviewPrimitivePacket,
    violations: &mut Vec<M5RerunViolation>,
) {
    let present: BTreeSet<M5RunAttemptSurfaceFamily> = packet
        .surface_rows
        .iter()
        .map(|row| row.surface_family)
        .collect();
    for required in M5RunAttemptSurfaceFamily::ALL {
        if !present.contains(&required) {
            violations.push(M5RerunViolation::RequiredSurfaceMissing);
            return;
        }
    }

    for row in &packet.surface_rows {
        if row.owner_role.trim().is_empty()
            || row.scope_summary.trim().is_empty()
            || row.source_contract_refs.is_empty()
        {
            violations.push(M5RerunViolation::SurfaceRowIncomplete);
        }
        if !row.declares_mandatory_export_fields() {
            violations.push(M5RerunViolation::MandatoryExportFieldMissing);
        }
        if row.rerun_modes.is_empty() {
            violations.push(M5RerunViolation::RerunModesMissing);
        }
        if row.downgrade_triggers.is_empty() {
            violations.push(M5RerunViolation::DowngradeTriggersMissing);
        }
        if row.consumer_surfaces.is_empty() {
            violations.push(M5RerunViolation::ConsumerSurfacesMissing);
        }
        if row.example_reruns.is_empty() {
            violations.push(M5RerunViolation::ExampleRerunsMissing);
        }
        if row.example_reruns.iter().any(|case| !case.is_self_consistent()) {
            violations.push(M5RerunViolation::ExampleRerunDrift);
        }
        if !row.honours_invariants() {
            violations.push(M5RerunViolation::SurfaceInvariantViolated);
        }
    }
}

/// The acceptance criteria must each be demonstrated across the matrix: distinct
/// reviewed actions survive a changed context (AC1), the changed context is reviewable
/// before dispatch (AC2), and the reviewed mode and summary survive the export (AC3).
fn validate_acceptance_criteria_covered(
    packet: &M5RerunReviewPrimitivePacket,
    violations: &mut Vec<M5RerunViolation>,
) {
    let cases: Vec<&M5ResolvedRerunReview> = packet
        .surface_rows
        .iter()
        .flat_map(|row| row.example_reruns.iter().map(|case| &case.resolved))
        .collect();

    // AC1: at least one case where a changed context keeps the reviewed actions
    // distinct, every rerun mode is covered, and every case distinguishes its actions
    // and keeps identity consistent.
    let mut modes_seen: BTreeSet<M5RerunMode> = BTreeSet::new();
    for resolved in &cases {
        modes_seen.insert(resolved.sheet.rerun_mode);
    }
    let distinct_proven = cases.iter().any(|resolved| {
        resolved.sheet.context_has_changes && resolved.sheet.available_modes.len() >= 2
    }) && cases
        .iter()
        .all(|resolved| resolved.distinguishes_rerun_actions() && resolved.identity_consistent())
        && M5RerunMode::ALL.iter().all(|mode| modes_seen.contains(mode));
    if !distinct_proven {
        violations.push(M5RerunViolation::DistinctActionsUnproven);
    }

    // AC2: at least one case with a changed dimension shown before dispatch, every case
    // discloses its context delta before dispatch, and every change dimension appears at
    // least once across the matrix.
    let mut dimensions_seen: BTreeSet<M5RerunChangeDimension> = BTreeSet::new();
    for resolved in &cases {
        for row in &resolved.change_rows {
            dimensions_seen.insert(row.dimension);
        }
    }
    let review_proven = cases.iter().any(|resolved| {
        resolved
            .change_rows
            .iter()
            .any(|row| row.state.is_changed() && row.shown_before_dispatch)
    }) && cases
        .iter()
        .all(|resolved| resolved.discloses_context_delta_before_dispatch())
        && M5RerunChangeDimension::ALL
            .iter()
            .all(|dimension| dimensions_seen.contains(dimension));
    if !review_proven {
        violations.push(M5RerunViolation::ContextReviewUnproven);
    }

    // AC3: every case preserves its prior lineage and carries the reviewed mode plus a
    // non-empty change summary in its export.
    let export_proven = cases.iter().all(|resolved| {
        resolved.preserves_prior_lineage()
            && !resolved.export.change_summary.trim().is_empty()
            && resolved.export.export_fields.contains(&M5RerunExportField::RerunMode)
    });
    if !export_proven {
        violations.push(M5RerunViolation::ExportPreservationUnproven);
    }
}

fn validate_governance_review(
    packet: &M5RerunReviewPrimitivePacket,
    violations: &mut Vec<M5RerunViolation>,
) {
    let review = &packet.governance_review;
    for ok in [
        review.one_primitive_carries_all_surfaces,
        review.distinct_rerun_actions_never_collapsed,
        review.changed_context_reviewable_before_dispatch,
        review.prior_attempt_lineage_preserved,
        review.reviewed_mode_and_summary_survive_export,
        review.support_export_reconstructs_rerun,
        review.later_rows_cannot_invent_parallel_vocabulary,
    ] {
        if !ok {
            violations.push(M5RerunViolation::GovernanceReviewIncomplete);
            return;
        }
    }
}

fn validate_consumer_projection(
    packet: &M5RerunReviewPrimitivePacket,
    violations: &mut Vec<M5RerunViolation>,
) {
    let projection = &packet.consumer_projection;
    for ok in [
        projection.execution_surfaces_consume_shared_primitive,
        projection.resolver_reads_single_model,
        projection.change_rows_read_single_diff_source,
        projection.support_export_reads_single_source,
    ] {
        if !ok {
            violations.push(M5RerunViolation::ConsumerProjectionIncomplete);
            return;
        }
    }
}

fn validate_release_posture(
    packet: &M5RerunReviewPrimitivePacket,
    violations: &mut Vec<M5RerunViolation>,
) {
    let posture = &packet.release_posture;
    if posture.release_packet_ref.trim().is_empty()
        || posture.rerun_audit_ref.trim().is_empty()
        || !posture.support_export_parity_required
        || !posture.accessibility_parity_required
    {
        violations.push(M5RerunViolation::ReleasePostureIncomplete);
    }
}

/// Joins tokens for a CSV cell with a `|` separator so a single cell never introduces a
/// stray comma.
fn join_tokens<T, F>(items: &[T], to_token: F) -> String
where
    F: Fn(&T) -> &'static str,
{
    items.iter().map(to_token).collect::<Vec<_>>().join("|")
}

/// Quotes a free-text CSV field when it contains a comma or quote.
fn csv_field(value: &str) -> String {
    if value.contains(',') || value.contains('"') || value.contains('\n') {
        format!("\"{}\"", value.replace('"', "\"\""))
    } else {
        value.to_owned()
    }
}

/// Heuristic that rejects obviously forbidden material in export-safe JSON.
fn json_contains_forbidden_material(value: &serde_json::Value) -> bool {
    match value {
        serde_json::Value::String(s) => value_is_forbidden(s),
        serde_json::Value::Array(arr) => arr.iter().any(json_contains_forbidden_material),
        serde_json::Value::Object(map) => map.values().any(json_contains_forbidden_material),
        _ => false,
    }
}

include!("seed.rs");
