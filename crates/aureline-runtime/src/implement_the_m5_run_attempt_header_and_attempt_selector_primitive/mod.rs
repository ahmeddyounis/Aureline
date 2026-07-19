//! Implements the reusable run/attempt-header primitive: a run/attempt header, an
//! attempt selector, a CLI / headless header line, and a support-export projection
//! that all resolve from one run-and-attempt context and share one run identity, one
//! attempt identity, and one outcome-state label, so every claimed M5 execution
//! surface exposes stable run IDs, attempt IDs, initiator, target, context summary,
//! age, and user-visible outcome state *before* actions continue.
//!
//! Where
//! [`crate::freeze_the_m5_run_attempt_input_request_artifact_publish_rerun_review_and_debug_hierarchy_component_matrix`]
//! *freezes* the reusable execution-lifecycle component families as a governed
//! contract, this module *narrows* one of those families —
//! [`crate::freeze_the_m5_run_attempt_input_request_artifact_publish_rerun_review_and_debug_hierarchy_component_matrix::M5ExecutionComponentFamily::RunAttemptHeader`]
//! — plus the attempt selector it implies into one working primitive with a real
//! **resolver**. A single run-and-attempt context projects onto four surfaces that
//! share one run identity, one attempt identity, and one truth class, so run-versus-
//! attempt identity, queue reason, admission-control class, target boundary, and the
//! user-visible outcome state never blur across the header, the attempt selector, the
//! CLI/headless line, and the support-export projection.
//!
//! The three acceptance criteria the resolver proves:
//!
//! - **AC1 — one run with multiple attempts is distinguishable from multiple
//!   separate runs without leaving the surface.** The header keeps the run ref and
//!   the attempt ref distinct, and the attempt selector lists every attempt of the
//!   *same* run, so a retry, rerun, or resume never reads as a different run.
//! - **AC2 — header state labels stay consistent across surfaces.** The outcome-state
//!   label is derived from one closed outcome vocabulary, so the same run outcome
//!   reads identically across task, test, request, notebook, AI-mediated execution,
//!   publish, and preview flows.
//! - **AC3 — exported evidence and support packets preserve the same run/attempt IDs
//!   and visible states shown in-product.** The support-export projection carries the
//!   run ref, attempt ref, attempt ordinal, outcome, and truth class byte-for-byte
//!   with the header.
//!
//! Raw run logs, raw stdout/stderr bytes, provider cursors, credentials, and raw
//! event payloads never cross this boundary; the resolver carries only opaque refs,
//! typed class tokens, booleans, and redacted labels, so support and diagnostics
//! exports reconstruct exactly what a surface would have shown without leaking source
//! or live payloads.
//!
//! The boundary schema is
//! [`schemas/ui/m5-run-attempt-header.schema.json`](../../../../schemas/ui/m5-run-attempt-header.schema.json).
//! The contract doc is
//! [`docs/run-test-debug/m5_run_attempt_header_primitive.md`](../../../../docs/run-test-debug/m5_run_attempt_header_primitive.md).

#[cfg(test)]
mod tests;

use std::collections::{BTreeMap, BTreeSet};
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

use crate::freeze_the_m5_run_attempt_input_request_artifact_publish_rerun_review_and_debug_hierarchy_component_matrix::{
    DegradedState, M5ExecutionDowngradeTrigger, M5ExecutionLocality, M5ExecutionTruthMode,
    M5RunOutcome,
};

/// Stable record-kind tag carried by [`M5RunAttemptHeaderPrimitivePacket`].
pub const M5_RUN_ATTEMPT_HEADER_RECORD_KIND: &str = "m5_run_attempt_header_primitive";

/// Schema version for the run/attempt-header primitive packet.
pub const M5_RUN_ATTEMPT_HEADER_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the boundary schema.
pub const M5_RUN_ATTEMPT_HEADER_SCHEMA_REF: &str = "schemas/ui/m5-run-attempt-header.schema.json";

/// Repo-relative path of the contract doc.
pub const M5_RUN_ATTEMPT_HEADER_DOC_REF: &str =
    "docs/run-test-debug/m5_run_attempt_header_primitive.md";

/// Repo-relative path of the frozen component-matrix contract this primitive narrows.
pub const M5_RUN_ATTEMPT_HEADER_COMPONENT_MATRIX_REF: &str =
    "schemas/ui/m5-execution-lifecycle-component-matrix.schema.json";

/// Repo-relative path of the protected fixture directory.
pub const M5_RUN_ATTEMPT_HEADER_FIXTURE_DIR: &str = "fixtures/ui/m5-run-attempt-header-primitive";

/// Repo-relative path of the checked support-export artifact (the `include_str!`
/// canonical).
pub const M5_RUN_ATTEMPT_HEADER_ARTIFACT_REF: &str =
    "artifacts/release/m5-run-attempt-header-primitive-proof/support_export.json";

/// Repo-relative path of the checked matrix CSV.
pub const M5_RUN_ATTEMPT_HEADER_CSV_REF: &str =
    "artifacts/release/m5-run-attempt-header-primitive-proof/matrix.csv";

/// Repo-relative path of the checked Markdown report.
pub const M5_RUN_ATTEMPT_HEADER_REPORT_REF: &str =
    "artifacts/release/m5-run-attempt-header-primitive-proof/report.md";

// --- minted controlled vocabulary ---

/// Closed run/attempt-header surface family. Each family is one execution flow that
/// ingests the shared primitive; the matrix must define every one.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5RunAttemptSurfaceFamily {
    /// The task-run pane.
    TaskRunPane,
    /// The test-run pane.
    TestRunPane,
    /// The API / request-run pane.
    RequestRunPane,
    /// A notebook cell / notebook execution surface.
    NotebookExecution,
    /// An AI-mediated execution surface (agent-driven run).
    AiMediatedExecution,
    /// A publish / deploy flow.
    PublishFlow,
    /// A preview / render flow.
    PreviewFlow,
    /// The run history / activity-center list.
    HistoryActivityCenter,
    /// The support / export replay surface reconstructing run/attempt truth.
    SupportExportReplay,
    /// The companion-surface run summary.
    CompanionSummary,
}

impl M5RunAttemptSurfaceFamily {
    /// Every execution flow, in declaration order.
    pub const ALL: [Self; 10] = [
        Self::TaskRunPane,
        Self::TestRunPane,
        Self::RequestRunPane,
        Self::NotebookExecution,
        Self::AiMediatedExecution,
        Self::PublishFlow,
        Self::PreviewFlow,
        Self::HistoryActivityCenter,
        Self::SupportExportReplay,
        Self::CompanionSummary,
    ];

    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::TaskRunPane => "task_run_pane",
            Self::TestRunPane => "test_run_pane",
            Self::RequestRunPane => "request_run_pane",
            Self::NotebookExecution => "notebook_execution",
            Self::AiMediatedExecution => "ai_mediated_execution",
            Self::PublishFlow => "publish_flow",
            Self::PreviewFlow => "preview_flow",
            Self::HistoryActivityCenter => "history_activity_center",
            Self::SupportExportReplay => "support_export_replay",
            Self::CompanionSummary => "companion_summary",
        }
    }

    /// Human-readable label for the Markdown report.
    pub const fn label(self) -> &'static str {
        match self {
            Self::TaskRunPane => "Task-run pane",
            Self::TestRunPane => "Test-run pane",
            Self::RequestRunPane => "Request-run pane",
            Self::NotebookExecution => "Notebook execution",
            Self::AiMediatedExecution => "AI-mediated execution",
            Self::PublishFlow => "Publish flow",
            Self::PreviewFlow => "Preview flow",
            Self::HistoryActivityCenter => "History / activity center",
            Self::SupportExportReplay => "Support / export replay",
            Self::CompanionSummary => "Companion summary",
        }
    }
}

/// Closed run-initiator vocabulary. Names who started a run so a scheduled or
/// agent-driven run never reads as a manual one.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5RunInitiatorClass {
    /// A user started the run manually.
    UserManual,
    /// A schedule / cron started the run.
    Scheduled,
    /// A CI / pipeline trigger started the run.
    CiTriggered,
    /// An AI agent started the run.
    AgentAi,
    /// A file / source watcher started the run automatically.
    WatchAuto,
}

impl M5RunInitiatorClass {
    /// Every initiator class, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::UserManual,
        Self::Scheduled,
        Self::CiTriggered,
        Self::AgentAi,
        Self::WatchAuto,
    ];

    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::UserManual => "user_manual",
            Self::Scheduled => "scheduled",
            Self::CiTriggered => "ci_triggered",
            Self::AgentAi => "agent_ai",
            Self::WatchAuto => "watch_auto",
        }
    }
}

/// Closed admission-control vocabulary. Names how a run was admitted so a queued run
/// always discloses why it is waiting rather than appearing stuck.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5AdmissionControlClass {
    /// Admitted immediately with no queuing.
    Immediate,
    /// Queued waiting on execution capacity.
    CapacityQueued,
    /// Queued waiting on an upstream dependency.
    DependencyQueued,
    /// Queued behind a concurrency limit.
    ConcurrencyLimited,
    /// Held by a policy / approval gate before admission.
    PolicyGated,
}

impl M5AdmissionControlClass {
    /// Every admission-control class, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::Immediate,
        Self::CapacityQueued,
        Self::DependencyQueued,
        Self::ConcurrencyLimited,
        Self::PolicyGated,
    ];

    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Immediate => "immediate",
            Self::CapacityQueued => "capacity_queued",
            Self::DependencyQueued => "dependency_queued",
            Self::ConcurrencyLimited => "concurrency_limited",
            Self::PolicyGated => "policy_gated",
        }
    }

    /// True when this class denotes a run held in a queue for a disclosed reason.
    pub const fn is_queued(self) -> bool {
        !matches!(self, Self::Immediate)
    }
}

/// Closed export-field vocabulary. Names the fields the support / export packet must
/// carry per surface; the mandatory subset must appear on every row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5RunAttemptExportField {
    /// The stable run identity shared across surfaces.
    RunId,
    /// The stable attempt identity, distinct from the run identity.
    AttemptId,
    /// The 1-based attempt ordinal within the run.
    AttemptOrdinal,
    /// Who initiated the run.
    Initiator,
    /// The local / remote / container / managed target boundary.
    TargetBoundary,
    /// The user-visible outcome state.
    Outcome,
    /// The captured-versus-live truth class.
    TruthClass,
    /// The human-readable context summary.
    ContextSummary,
    /// The relative age label.
    Age,
}

impl M5RunAttemptExportField {
    /// Every export field, in declaration order.
    pub const ALL: [Self; 9] = [
        Self::RunId,
        Self::AttemptId,
        Self::AttemptOrdinal,
        Self::Initiator,
        Self::TargetBoundary,
        Self::Outcome,
        Self::TruthClass,
        Self::ContextSummary,
        Self::Age,
    ];

    /// The mandatory subset every row must carry: the run/attempt IDs, ordinal,
    /// outcome, and truth class that must survive into any support export (AC3).
    pub const MANDATORY: [Self; 5] = [
        Self::RunId,
        Self::AttemptId,
        Self::AttemptOrdinal,
        Self::Outcome,
        Self::TruthClass,
    ];

    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::RunId => "run_id",
            Self::AttemptId => "attempt_id",
            Self::AttemptOrdinal => "attempt_ordinal",
            Self::Initiator => "initiator",
            Self::TargetBoundary => "target_boundary",
            Self::Outcome => "outcome",
            Self::TruthClass => "truth_class",
            Self::ContextSummary => "context_summary",
            Self::Age => "age",
        }
    }
}

// --- shared value structs ---

/// One attempt of a run, as listed in the attempt selector. Every attempt in the
/// selector belongs to the same run, so a run with multiple attempts never reads as
/// multiple separate runs.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5SiblingAttempt {
    /// Opaque ref to the attempt identity; never raw run bytes.
    pub attempt_ref: String,
    /// 1-based ordinal of this attempt within the run.
    pub attempt_ordinal: u32,
    /// The user-visible outcome of this attempt.
    pub outcome: M5RunOutcome,
    /// Whether this row is the currently-selected attempt.
    pub is_current: bool,
}

// --- resolver input ---

/// The full input to the run/attempt-header resolver for one run-and-attempt context.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5RunAttemptHeaderInput {
    /// The stable header identity that must survive across the header, attempt
    /// selector, CLI line, and support-export projection.
    pub header_id: String,
    /// Opaque ref to the run identity; never raw run bytes.
    pub run_ref: String,
    /// Opaque ref to the attempt identity; distinct from the run identity.
    pub attempt_ref: String,
    /// 1-based ordinal of the current attempt within the run.
    pub attempt_ordinal: u32,
    /// Human-readable run label.
    pub run_label: String,
    /// Who initiated the run.
    pub initiator: M5RunInitiatorClass,
    /// Opaque initiator name / handle, when known; never raw credentials.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub initiator_label: Option<String>,
    /// Opaque ref to the target the run acts on; never raw endpoint data.
    pub target_ref: String,
    /// The local / remote / container / managed target boundary.
    pub target_boundary: M5ExecutionLocality,
    /// Human-readable context summary shown on the header.
    pub context_summary: String,
    /// Relative age label ("2m ago", "just now").
    pub age_label: String,
    /// The user-visible outcome state.
    pub outcome: M5RunOutcome,
    /// The captured-versus-live truth class.
    pub truth_mode: M5ExecutionTruthMode,
    /// How the run was admitted.
    pub admission_control: M5AdmissionControlClass,
    /// A precise queue reason, required when the run is admission-queued.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub queue_reason: Option<String>,
    /// The run's position in the queue / attempt ordering, when known.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub relative_ordering: Option<u32>,
    /// The other attempts of *this* run, powering the attempt selector. Never
    /// includes the current attempt.
    #[serde(default)]
    pub sibling_attempts: Vec<M5SiblingAttempt>,
    /// An externally-observed narrowing (connector loss, captured-only, stale) that
    /// degrades the surface.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub degraded: Option<DegradedState>,
}

// --- resolved projections ---

/// The resolved run/attempt header.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ResolvedRunAttemptHeader {
    /// The header identity — identical to the selector, CLI line, and export.
    pub header_id: String,
    /// The opaque run ref.
    pub run_ref: String,
    /// The opaque attempt ref — distinct from the run ref.
    pub attempt_ref: String,
    /// The 1-based attempt ordinal.
    pub attempt_ordinal: u32,
    /// The human-readable run label.
    pub run_label: String,
    /// Who initiated the run.
    pub initiator: M5RunInitiatorClass,
    /// The opaque initiator name, when known.
    pub initiator_label: Option<String>,
    /// The opaque target ref.
    pub target_ref: String,
    /// The target boundary.
    pub target_boundary: M5ExecutionLocality,
    /// The context summary.
    pub context_summary: String,
    /// The relative age label.
    pub age_label: String,
    /// The user-visible outcome.
    pub outcome: M5RunOutcome,
    /// The captured-versus-live truth class.
    pub truth_mode: M5ExecutionTruthMode,
    /// How the run was admitted.
    pub admission_control: M5AdmissionControlClass,
    /// The disclosed queue reason, when the run is admission-queued.
    pub queue_reason: Option<String>,
    /// Run and attempt identity stay distinct; always holds by construction.
    pub run_and_attempt_distinct: bool,
    /// The canonical outcome-state label, derived from the closed outcome vocabulary.
    pub state_label: String,
}

/// The resolved attempt selector.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ResolvedAttemptSelector {
    /// The header identity — identical to every other projection.
    pub header_id: String,
    /// The opaque run ref every listed attempt belongs to.
    pub run_ref: String,
    /// Every attempt of this run, including the current one, ordered by ordinal.
    pub attempts: Vec<M5SiblingAttempt>,
    /// The number of attempts of this run.
    pub attempt_count: u32,
    /// The opaque ref of the currently-selected attempt.
    pub current_attempt_ref: String,
    /// The run's position in the queue / attempt ordering, when known.
    pub relative_ordering: Option<u32>,
    /// Every listed attempt shares this run — a run with multiple attempts never
    /// reads as multiple separate runs. Always holds by construction.
    pub all_attempts_share_run: bool,
}

/// The resolved CLI / headless header line.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ResolvedCliHeaderLine {
    /// The header identity — identical to every other projection.
    pub header_id: String,
    /// The opaque run ref.
    pub run_ref: String,
    /// The opaque attempt ref.
    pub attempt_ref: String,
    /// The deterministic single-line summary in the shared header vocabulary.
    pub line: String,
    /// The user-visible outcome.
    pub outcome: M5RunOutcome,
    /// The captured-versus-live truth class.
    pub truth_mode: M5ExecutionTruthMode,
}

/// The resolved support-export projection.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ResolvedRunAttemptExport {
    /// The header identity — identical to every other projection.
    pub header_id: String,
    /// The opaque run ref — identical to the header's run ref.
    pub run_ref: String,
    /// The opaque attempt ref — identical to the header's attempt ref.
    pub attempt_ref: String,
    /// The 1-based attempt ordinal — identical to the header's ordinal.
    pub attempt_ordinal: u32,
    /// The user-visible outcome — identical to the header's outcome.
    pub outcome: M5RunOutcome,
    /// The captured-versus-live truth class — identical to the header's.
    pub truth_mode: M5ExecutionTruthMode,
    /// The target boundary — identical to the header's.
    pub target_boundary: M5ExecutionLocality,
    /// The export fields this projection carries; includes the mandatory subset.
    pub export_fields: Vec<M5RunAttemptExportField>,
    /// The canonical outcome-state label — identical to the header's.
    pub state_label: String,
}

/// The resolved run/attempt truth shared across the header, attempt selector, CLI
/// line, and support-export projection.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ResolvedRunAttempt {
    /// The stable header identity.
    pub header_id: String,
    /// The resolved run/attempt header.
    pub header: M5ResolvedRunAttemptHeader,
    /// The resolved attempt selector.
    pub selector: M5ResolvedAttemptSelector,
    /// The resolved CLI / headless header line.
    pub cli_line: M5ResolvedCliHeaderLine,
    /// The resolved support-export projection.
    pub export: M5ResolvedRunAttemptExport,
    /// Run identity and attempt identity are disclosed and kept distinct (AC1).
    pub run_identity_disclosed: bool,
    /// The outcome-state label is identical across the header, CLI line, and export
    /// (AC2).
    pub state_label_parity: bool,
    /// The support-export projection preserves the run/attempt IDs and visible states
    /// (AC3).
    pub export_preserves_ids_states: bool,
    /// The narrowing carried through from the input, when present.
    pub degraded: Option<DegradedState>,
}

impl M5ResolvedRunAttempt {
    /// True when the header identity is identical across the header, selector, CLI
    /// line, and export, and the run/attempt refs agree across projections.
    pub fn identity_consistent(&self) -> bool {
        self.header.header_id == self.header_id
            && self.selector.header_id == self.header_id
            && self.cli_line.header_id == self.header_id
            && self.export.header_id == self.header_id
            && self.selector.run_ref == self.header.run_ref
            && self.cli_line.run_ref == self.header.run_ref
            && self.export.run_ref == self.header.run_ref
            && self.cli_line.attempt_ref == self.header.attempt_ref
            && self.export.attempt_ref == self.header.attempt_ref
    }

    /// True when run identity and attempt identity stay distinct.
    pub fn run_and_attempt_distinct(&self) -> bool {
        self.header.run_and_attempt_distinct && self.header.run_ref != self.header.attempt_ref
    }

    /// True when one run with multiple attempts is distinguishable from multiple
    /// separate runs: the header keeps run and attempt distinct, and the selector
    /// lists every attempt of the *same* run, current attempt included (AC1).
    pub fn distinguishes_attempts_from_runs(&self) -> bool {
        self.run_and_attempt_distinct()
            && self.selector.all_attempts_share_run
            && self.selector.attempt_count as usize == self.selector.attempts.len()
            && self.selector.current_attempt_ref == self.header.attempt_ref
            && self
                .selector
                .attempts
                .iter()
                .any(|a| a.attempt_ref == self.header.attempt_ref && a.is_current)
            && self
                .selector
                .attempts
                .iter()
                .all(|a| a.attempt_ref != self.header.run_ref)
    }

    /// True when the outcome-state label is identical across the header, CLI line, and
    /// export, and every projection reports the same outcome (AC2).
    pub fn state_labels_consistent(&self) -> bool {
        self.header.state_label == self.export.state_label
            && self.header.outcome == self.export.outcome
            && self.header.outcome == self.cli_line.outcome
            && self.header.state_label == outcome_state_label(self.header.outcome)
    }

    /// True when the support-export projection preserves the run/attempt IDs and
    /// visible states, and declares the mandatory export fields (AC3).
    pub fn export_preserves_ids_and_states(&self) -> bool {
        self.export.run_ref == self.header.run_ref
            && self.export.attempt_ref == self.header.attempt_ref
            && self.export.attempt_ordinal == self.header.attempt_ordinal
            && self.export.outcome == self.header.outcome
            && self.export.truth_mode == self.header.truth_mode
            && self.export.target_boundary == self.header.target_boundary
            && declares_mandatory_export_fields(&self.export.export_fields)
    }
}

/// Errors returned by [`resolve_run_attempt_header`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum M5RunAttemptResolutionError {
    /// The header identity was empty.
    EmptyHeaderId,
    /// The run ref was empty.
    EmptyRunRef,
    /// The attempt ref was empty.
    EmptyAttemptRef,
    /// The run label was empty.
    EmptyRunLabel,
    /// The target ref was empty.
    EmptyTargetRef,
    /// The context summary was empty.
    EmptyContextSummary,
    /// The age label was empty.
    EmptyAgeLabel,
    /// The attempt ordinal was zero.
    InvalidAttemptOrdinal,
    /// The run ref and attempt ref were equal — run and attempt identity collapsed.
    RunAttemptIdentityCollapsed,
    /// A label, ref, or note carried forbidden material.
    ForbiddenMaterial,
    /// A sibling attempt was incomplete (empty ref or zero ordinal).
    SiblingAttemptIncomplete,
    /// A sibling attempt collapsed with the current attempt or the run identity.
    SiblingAttemptCollapsed,
    /// Two sibling attempts shared a ref or ordinal.
    DuplicateSiblingAttempt,
    /// A queued outcome named no admission reason (admission was `Immediate`).
    QueuedWithoutAdmissionReason,
    /// An admission-queued run carried no queue reason.
    QueueReasonMissing,
    /// An actively executing outcome was not shown as live truth.
    ActiveOutcomeNotLive,
    /// A stale output claimed live truth.
    StaleOutputClaimsLive,
    /// A degraded block carried a generic non-answer label.
    DegradedLabelGeneric,
}

impl M5RunAttemptResolutionError {
    /// Stable token for tests and diagnostics.
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::EmptyHeaderId => "empty_header_id",
            Self::EmptyRunRef => "empty_run_ref",
            Self::EmptyAttemptRef => "empty_attempt_ref",
            Self::EmptyRunLabel => "empty_run_label",
            Self::EmptyTargetRef => "empty_target_ref",
            Self::EmptyContextSummary => "empty_context_summary",
            Self::EmptyAgeLabel => "empty_age_label",
            Self::InvalidAttemptOrdinal => "invalid_attempt_ordinal",
            Self::RunAttemptIdentityCollapsed => "run_attempt_identity_collapsed",
            Self::ForbiddenMaterial => "forbidden_material",
            Self::SiblingAttemptIncomplete => "sibling_attempt_incomplete",
            Self::SiblingAttemptCollapsed => "sibling_attempt_collapsed",
            Self::DuplicateSiblingAttempt => "duplicate_sibling_attempt",
            Self::QueuedWithoutAdmissionReason => "queued_without_admission_reason",
            Self::QueueReasonMissing => "queue_reason_missing",
            Self::ActiveOutcomeNotLive => "active_outcome_not_live",
            Self::StaleOutputClaimsLive => "stale_output_claims_live",
            Self::DegradedLabelGeneric => "degraded_label_generic",
        }
    }
}

impl fmt::Display for M5RunAttemptResolutionError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "run/attempt-header resolution error: {}",
            self.as_str()
        )
    }
}

impl Error for M5RunAttemptResolutionError {}

/// Resolves one run-and-attempt context into its shared header, attempt selector, CLI
/// / headless line, and support-export projection.
///
/// The four surfaces share one header identity, one run ref, one attempt ref, and one
/// outcome-state label, so run-versus-attempt identity, queue reason, admission-control
/// class, target boundary, and the user-visible outcome state never blur across them.
/// The attempt selector lists every attempt of the same run so a retry, rerun, or
/// resume never reads as a different run; the export projection preserves the same
/// run/attempt IDs and visible states shown in-product.
///
/// # Errors
///
/// Returns an [`M5RunAttemptResolutionError`] when identity is missing or collapsed,
/// a sibling attempt is incomplete or collapsed, a queued run hides its admission
/// reason, an active outcome is not live truth, a stale output claims live, or any
/// ref / label carries forbidden material.
pub fn resolve_run_attempt_header(
    input: &M5RunAttemptHeaderInput,
) -> Result<M5ResolvedRunAttempt, M5RunAttemptResolutionError> {
    if input.header_id.trim().is_empty() {
        return Err(M5RunAttemptResolutionError::EmptyHeaderId);
    }
    if input.run_ref.trim().is_empty() {
        return Err(M5RunAttemptResolutionError::EmptyRunRef);
    }
    if input.attempt_ref.trim().is_empty() {
        return Err(M5RunAttemptResolutionError::EmptyAttemptRef);
    }
    if input.run_label.trim().is_empty() {
        return Err(M5RunAttemptResolutionError::EmptyRunLabel);
    }
    if input.target_ref.trim().is_empty() {
        return Err(M5RunAttemptResolutionError::EmptyTargetRef);
    }
    if input.context_summary.trim().is_empty() {
        return Err(M5RunAttemptResolutionError::EmptyContextSummary);
    }
    if input.age_label.trim().is_empty() {
        return Err(M5RunAttemptResolutionError::EmptyAgeLabel);
    }
    if input.attempt_ordinal == 0 {
        return Err(M5RunAttemptResolutionError::InvalidAttemptOrdinal);
    }
    // Run identity and attempt identity must never collapse.
    if input.run_ref.trim() == input.attempt_ref.trim() {
        return Err(M5RunAttemptResolutionError::RunAttemptIdentityCollapsed);
    }

    for value in [
        input.run_ref.as_str(),
        input.attempt_ref.as_str(),
        input.run_label.as_str(),
        input.target_ref.as_str(),
        input.context_summary.as_str(),
        input.age_label.as_str(),
    ]
    .into_iter()
    .chain(input.initiator_label.as_deref())
    .chain(input.queue_reason.as_deref())
    {
        if value_is_forbidden(value) {
            return Err(M5RunAttemptResolutionError::ForbiddenMaterial);
        }
    }

    // Sibling attempts must be complete, distinct from the current attempt and the
    // run, and distinct from one another.
    let mut seen_refs: BTreeSet<&str> = BTreeSet::new();
    let mut seen_ordinals: BTreeSet<u32> = BTreeSet::new();
    seen_ordinals.insert(input.attempt_ordinal);
    for sibling in &input.sibling_attempts {
        if sibling.attempt_ref.trim().is_empty() || sibling.attempt_ordinal == 0 {
            return Err(M5RunAttemptResolutionError::SiblingAttemptIncomplete);
        }
        if value_is_forbidden(&sibling.attempt_ref) {
            return Err(M5RunAttemptResolutionError::ForbiddenMaterial);
        }
        if sibling.attempt_ref.trim() == input.attempt_ref.trim()
            || sibling.attempt_ref.trim() == input.run_ref.trim()
        {
            return Err(M5RunAttemptResolutionError::SiblingAttemptCollapsed);
        }
        if !seen_refs.insert(sibling.attempt_ref.trim())
            || !seen_ordinals.insert(sibling.attempt_ordinal)
        {
            return Err(M5RunAttemptResolutionError::DuplicateSiblingAttempt);
        }
    }

    // A queued run must always disclose why it is waiting.
    if input.outcome == M5RunOutcome::Queued
        && input.admission_control == M5AdmissionControlClass::Immediate
    {
        return Err(M5RunAttemptResolutionError::QueuedWithoutAdmissionReason);
    }
    if input.admission_control.is_queued()
        && !input
            .queue_reason
            .as_deref()
            .is_some_and(|reason| !reason.trim().is_empty())
    {
        return Err(M5RunAttemptResolutionError::QueueReasonMissing);
    }

    // An actively executing outcome must be shown as live truth; a stale output must
    // never claim live control. This mirrors the frozen run/attempt-header honesty.
    if input.outcome.is_active() && !input.truth_mode.is_live() {
        return Err(M5RunAttemptResolutionError::ActiveOutcomeNotLive);
    }
    if input.outcome == M5RunOutcome::StaleOutput && input.truth_mode.is_live() {
        return Err(M5RunAttemptResolutionError::StaleOutputClaimsLive);
    }

    if let Some(degraded) = &input.degraded {
        if !degraded.is_honest() {
            return Err(M5RunAttemptResolutionError::DegradedLabelGeneric);
        }
    }

    let state_label = outcome_state_label(input.outcome).to_owned();

    let header = M5ResolvedRunAttemptHeader {
        header_id: input.header_id.clone(),
        run_ref: input.run_ref.clone(),
        attempt_ref: input.attempt_ref.clone(),
        attempt_ordinal: input.attempt_ordinal,
        run_label: input.run_label.clone(),
        initiator: input.initiator,
        initiator_label: input.initiator_label.clone(),
        target_ref: input.target_ref.clone(),
        target_boundary: input.target_boundary,
        context_summary: input.context_summary.clone(),
        age_label: input.age_label.clone(),
        outcome: input.outcome,
        truth_mode: input.truth_mode,
        admission_control: input.admission_control,
        queue_reason: input.queue_reason.clone(),
        run_and_attempt_distinct: true,
        state_label: state_label.clone(),
    };

    // The attempt selector combines the current attempt with its siblings, ordered by
    // ordinal for determinism. Every listed attempt belongs to this run.
    let mut attempts: Vec<M5SiblingAttempt> = input
        .sibling_attempts
        .iter()
        .map(|sibling| M5SiblingAttempt {
            attempt_ref: sibling.attempt_ref.clone(),
            attempt_ordinal: sibling.attempt_ordinal,
            outcome: sibling.outcome,
            is_current: false,
        })
        .collect();
    attempts.push(M5SiblingAttempt {
        attempt_ref: input.attempt_ref.clone(),
        attempt_ordinal: input.attempt_ordinal,
        outcome: input.outcome,
        is_current: true,
    });
    attempts.sort_by(|a, b| {
        a.attempt_ordinal
            .cmp(&b.attempt_ordinal)
            .then_with(|| a.attempt_ref.cmp(&b.attempt_ref))
    });

    let selector = M5ResolvedAttemptSelector {
        header_id: input.header_id.clone(),
        run_ref: input.run_ref.clone(),
        attempt_count: attempts.len() as u32,
        current_attempt_ref: input.attempt_ref.clone(),
        relative_ordering: input.relative_ordering,
        all_attempts_share_run: true,
        attempts,
    };

    let cli_line = M5ResolvedCliHeaderLine {
        header_id: input.header_id.clone(),
        run_ref: input.run_ref.clone(),
        attempt_ref: input.attempt_ref.clone(),
        line: render_cli_line(input),
        outcome: input.outcome,
        truth_mode: input.truth_mode,
    };

    let export = M5ResolvedRunAttemptExport {
        header_id: input.header_id.clone(),
        run_ref: input.run_ref.clone(),
        attempt_ref: input.attempt_ref.clone(),
        attempt_ordinal: input.attempt_ordinal,
        outcome: input.outcome,
        truth_mode: input.truth_mode,
        target_boundary: input.target_boundary,
        export_fields: M5RunAttemptExportField::ALL.to_vec(),
        state_label,
    };

    let resolved = M5ResolvedRunAttempt {
        header_id: input.header_id.clone(),
        header,
        selector,
        cli_line,
        export,
        run_identity_disclosed: true,
        state_label_parity: true,
        export_preserves_ids_states: true,
        degraded: input.degraded.clone(),
    };

    Ok(resolved)
}

/// The canonical outcome-state label for one run outcome. Derived from the closed
/// outcome vocabulary so the same outcome reads identically across every surface.
fn outcome_state_label(outcome: M5RunOutcome) -> &'static str {
    match outcome {
        M5RunOutcome::Queued => "Queued",
        M5RunOutcome::Preparing => "Preparing",
        M5RunOutcome::Running => "Running",
        M5RunOutcome::WaitingInput => "Waiting for input",
        M5RunOutcome::PartiallyComplete => "Partially complete",
        M5RunOutcome::Passed => "Passed",
        M5RunOutcome::Failed => "Failed",
        M5RunOutcome::Cancelled => "Cancelled",
        M5RunOutcome::StaleOutput => "Stale output",
    }
}

/// Renders the deterministic CLI / headless header line in the shared header
/// vocabulary — the same run, attempt, outcome, truth, and boundary tokens the header
/// renders, so CLI and headless summaries never invent a parallel vocabulary.
fn render_cli_line(input: &M5RunAttemptHeaderInput) -> String {
    format!(
        "run={run} attempt=#{ordinal} state={state} truth={truth} initiator={initiator} \
boundary={boundary} admission={admission} age={age}",
        run = input.run_ref,
        ordinal = input.attempt_ordinal,
        state = input.outcome.as_str(),
        truth = input.truth_mode.as_str(),
        initiator = input.initiator.as_str(),
        boundary = input.target_boundary.as_str(),
        admission = input.admission_control.as_str(),
        age = input.age_label,
    )
}

/// True when a slice of export fields declares every mandatory field.
fn declares_mandatory_export_fields(fields: &[M5RunAttemptExportField]) -> bool {
    let present: BTreeSet<M5RunAttemptExportField> = fields.iter().copied().collect();
    M5RunAttemptExportField::MANDATORY
        .iter()
        .all(|field| present.contains(field))
}

/// True when a label, ref, or note carries obviously forbidden material.
fn value_is_forbidden(value: &str) -> bool {
    let lower = value.to_lowercase();
    lower.contains("api_key")
        || lower.contains("password")
        || lower.contains("secret")
        || lower.contains("bearer ")
        || lower.contains("://")
        || lower.contains("-----begin")
}

/// One worked resolution case carried in the packet so the support / export packet
/// reconstructs run/attempt truth from the shared model.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5RunAttemptCase {
    /// The resolver input.
    pub input: M5RunAttemptHeaderInput,
    /// The resolved run/attempt truth. Must equal
    /// `resolve_run_attempt_header(&input)`.
    pub resolved: M5ResolvedRunAttempt,
}

impl M5RunAttemptCase {
    /// Builds a case by resolving `input`.
    ///
    /// # Panics
    ///
    /// Panics if `input` does not resolve; seed inputs are always valid.
    pub fn resolved(input: M5RunAttemptHeaderInput) -> Self {
        let resolved = resolve_run_attempt_header(&input).expect("seed run/attempt case is valid");
        Self { input, resolved }
    }

    /// True when the stored resolution matches a fresh resolve of the input.
    pub fn is_self_consistent(&self) -> bool {
        resolve_run_attempt_header(&self.input).as_ref() == Ok(&self.resolved)
    }
}

/// One row in the primitive matrix: one execution surface family bound to the shared
/// run/attempt-header contract.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5RunAttemptSurfaceRow {
    /// The execution surface family.
    pub surface_family: M5RunAttemptSurfaceFamily,
    /// Owner role accountable for keeping this surface governed.
    pub owner_role: String,
    /// Human-readable scope summary.
    pub scope_summary: String,
    /// Outcomes this surface can render (must be non-empty).
    pub outcomes: Vec<M5RunOutcome>,
    /// Truth classes this surface renders (must be non-empty).
    pub truth_modes: Vec<M5ExecutionTruthMode>,
    /// Export fields this row carries (must include the mandatory fields).
    pub export_fields: Vec<M5RunAttemptExportField>,
    /// Downgrade triggers that apply to this surface (must be non-empty).
    pub downgrade_triggers: Vec<M5ExecutionDowngradeTrigger>,
    /// Consumer surfaces that ingest this row's projection (must be non-empty).
    pub consumer_surfaces: Vec<String>,
    /// Source contract refs consumed by this row (must be non-empty).
    pub source_contract_refs: Vec<String>,
    /// Worked resolution cases proving the resolver on this surface (must be
    /// non-empty).
    pub example_headers: Vec<M5RunAttemptCase>,
    /// Hard invariant: this row never hides the run or attempt identity. MUST be
    /// `false`.
    pub hides_run_or_attempt_identity: bool,
    /// Hard invariant: this row never blurs run and attempt identity. MUST be
    /// `false`.
    pub blurs_run_and_attempt: bool,
    /// Hard invariant: this row never drops the shared state-label parity. MUST be
    /// `false`.
    pub drops_state_label_parity: bool,
    /// Hard invariant: this row never drops the exported IDs or states. MUST be
    /// `false`.
    pub drops_export_ids_or_states: bool,
}

impl M5RunAttemptSurfaceRow {
    /// True when the row declares every mandatory export field.
    fn declares_mandatory_export_fields(&self) -> bool {
        declares_mandatory_export_fields(&self.export_fields)
    }

    /// True when the row's hard invariants hold.
    fn honours_invariants(&self) -> bool {
        !self.hides_run_or_attempt_identity
            && !self.blurs_run_and_attempt
            && !self.drops_state_label_parity
            && !self.drops_export_ids_or_states
    }
}

/// Self-describing controlled-vocabulary set minted / reused by this primitive.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5RunAttemptVocabularySet {
    /// Surface-family tokens.
    pub surface_families: Vec<String>,
    /// Initiator-class tokens.
    pub initiator_classes: Vec<String>,
    /// Admission-control tokens.
    pub admission_controls: Vec<String>,
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

impl M5RunAttemptVocabularySet {
    /// Builds the canonical vocabulary set from the typed `ALL` arrays.
    pub fn canonical() -> Self {
        Self {
            surface_families: tokens(&M5RunAttemptSurfaceFamily::ALL, |v| v.as_str()),
            initiator_classes: tokens(&M5RunInitiatorClass::ALL, |v| v.as_str()),
            admission_controls: tokens(&M5AdmissionControlClass::ALL, |v| v.as_str()),
            export_fields: tokens(&M5RunAttemptExportField::ALL, |v| v.as_str()),
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

/// The truth classes reused from the frozen matrix, in a stable order.
/// [`M5ExecutionTruthMode`] is a pure token set, so the order is pinned here.
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
pub struct M5RunAttemptGovernanceReview {
    /// One primitive carries header / selector / CLI-line / export truth on every
    /// surface.
    pub one_primitive_carries_all_surfaces: bool,
    /// Run identity and attempt identity are kept distinct everywhere a retry,
    /// rerun, or resume can occur.
    pub run_and_attempt_identity_kept_distinct: bool,
    /// Header state labels stay consistent across surfaces.
    pub state_labels_consistent_across_surfaces: bool,
    /// Queue reason and admission-control class are preserved in the shared header
    /// vocabulary used by CLI / headless summaries.
    pub queue_and_admission_preserved_in_shared_vocabulary: bool,
    /// Exported evidence preserves the run/attempt IDs and visible states.
    pub exported_evidence_preserves_ids_and_states: bool,
    /// The support / export packet reconstructs run/attempt truth.
    pub support_export_reconstructs_run_attempt: bool,
    /// Later M5 rows cannot invent parallel run/attempt-header vocabulary.
    pub later_rows_cannot_invent_parallel_vocabulary: bool,
}

/// Consumer projection block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5RunAttemptConsumerProjection {
    /// Task / test / request / notebook / AI / publish / preview surfaces all consume
    /// the shared primitive.
    pub execution_surfaces_consume_shared_primitive: bool,
    /// The run/attempt resolver reads a single canonical model.
    pub resolver_reads_single_model: bool,
    /// The attempt selector reads a single canonical run source.
    pub attempt_selector_reads_single_run_source: bool,
    /// Support / export reads a single canonical run/attempt source.
    pub support_export_reads_single_source: bool,
}

/// Release and support parity posture for the run/attempt-header primitive.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5RunAttemptReleasePosture {
    /// Ref of the supporting release packet.
    pub release_packet_ref: String,
    /// Ref of the supporting header audit.
    pub header_audit_ref: String,
    /// True when support / export parity is required for every surface.
    pub support_export_parity_required: bool,
    /// True when accessibility parity is required for every surface.
    pub accessibility_parity_required: bool,
}

/// Constructor input for [`M5RunAttemptHeaderPrimitivePacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M5RunAttemptHeaderPrimitivePacketInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable matrix label.
    pub matrix_label: String,
    /// Surface rows.
    pub surface_rows: Vec<M5RunAttemptSurfaceRow>,
    /// Frozen controlled-vocabulary set.
    pub vocabulary_set: M5RunAttemptVocabularySet,
    /// Governance-review block.
    pub governance_review: M5RunAttemptGovernanceReview,
    /// Consumer projection block.
    pub consumer_projection: M5RunAttemptConsumerProjection,
    /// Release and support parity posture.
    pub release_posture: M5RunAttemptReleasePosture,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

/// Export-safe M5 run/attempt-header primitive packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5RunAttemptHeaderPrimitivePacket {
    /// Record kind; must equal [`M5_RUN_ATTEMPT_HEADER_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`M5_RUN_ATTEMPT_HEADER_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable matrix label.
    pub matrix_label: String,
    /// Surface rows.
    pub surface_rows: Vec<M5RunAttemptSurfaceRow>,
    /// Frozen controlled-vocabulary set.
    pub vocabulary_set: M5RunAttemptVocabularySet,
    /// Governance-review block.
    pub governance_review: M5RunAttemptGovernanceReview,
    /// Consumer projection block.
    pub consumer_projection: M5RunAttemptConsumerProjection,
    /// Release and support parity posture.
    pub release_posture: M5RunAttemptReleasePosture,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

impl M5RunAttemptHeaderPrimitivePacket {
    /// Builds an M5 run/attempt-header primitive packet from stable-lane input.
    pub fn new(input: M5RunAttemptHeaderPrimitivePacketInput) -> Self {
        Self {
            record_kind: M5_RUN_ATTEMPT_HEADER_RECORD_KIND.to_owned(),
            schema_version: M5_RUN_ATTEMPT_HEADER_SCHEMA_VERSION,
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

    /// Validates the M5 run/attempt-header primitive invariants.
    pub fn validate(&self) -> Vec<M5RunAttemptViolation> {
        let mut violations = Vec::new();

        if self.record_kind != M5_RUN_ATTEMPT_HEADER_RECORD_KIND {
            violations.push(M5RunAttemptViolation::WrongRecordKind);
        }
        if self.schema_version != M5_RUN_ATTEMPT_HEADER_SCHEMA_VERSION {
            violations.push(M5RunAttemptViolation::WrongSchemaVersion);
        }
        if self.packet_id.trim().is_empty()
            || self.matrix_label.trim().is_empty()
            || self.redaction_class_token.trim().is_empty()
            || self.minted_at.trim().is_empty()
        {
            violations.push(M5RunAttemptViolation::MissingIdentity);
        }

        validate_source_contracts(self, &mut violations);
        validate_vocabulary_set(self, &mut violations);
        validate_surface_rows(self, &mut violations);
        validate_acceptance_criteria_covered(self, &mut violations);
        validate_governance_review(self, &mut violations);
        validate_consumer_projection(self, &mut violations);
        validate_release_posture(self, &mut violations);

        if json_contains_forbidden_material(
            &serde_json::to_value(self).expect("m5 run/attempt-header primitive packet serializes"),
        ) {
            violations.push(M5RunAttemptViolation::RawMaterialInExport);
        }

        violations
    }

    /// Deterministic export-safe JSON.
    ///
    /// # Panics
    ///
    /// Panics only if serializing this metadata-only packet fails.
    pub fn export_safe_json(&self) -> String {
        serde_json::to_string_pretty(self)
            .expect("m5 run/attempt-header primitive packet serializes")
    }

    /// Deterministic, machine-readable matrix CSV: one row per surface family.
    pub fn render_matrix_csv(&self) -> String {
        let mut out = String::new();
        out.push_str("surface_family,owner,outcomes,truth_modes,export_fields,example_count\n");
        for row in &self.surface_rows {
            out.push_str(&format!(
                "{},{},{},{},{},{}\n",
                row.surface_family.as_str(),
                csv_field(&row.owner_role),
                join_tokens(&row.outcomes, |v| v.as_str()),
                join_tokens(&row.truth_modes, |v| v.as_str()),
                join_tokens(&row.export_fields, |v| v.as_str()),
                row.example_headers.len(),
            ));
        }
        out
    }

    /// Deterministic Markdown report for support, docs, or review handoff.
    pub fn render_markdown_summary(&self) -> String {
        let mut out = String::new();
        out.push_str("# M5 Run/Attempt-Header Primitive: Header and Attempt Selector\n\n");
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- Label: `{}`\n", self.matrix_label));
        out.push_str(&format!(
            "- Execution surfaces: {} / {}\n",
            self.surface_rows.len(),
            M5RunAttemptSurfaceFamily::ALL.len(),
        ));
        out.push_str(&format!(
            "- Initiator classes: {}\n",
            self.vocabulary_set.initiator_classes.join(", ")
        ));
        out.push_str(&format!(
            "- Admission-control classes: {}\n",
            self.vocabulary_set.admission_controls.join(", ")
        ));
        out.push_str("\n## Execution surfaces\n\n");
        for row in &self.surface_rows {
            out.push_str(&format!("- **{}**\n", row.surface_family.label()));
            out.push_str(&format!("  - Owner: {}\n", row.owner_role));
            out.push_str(&format!("  - Scope: {}\n", row.scope_summary));
            out.push_str(&format!(
                "  - Worked cases: {}\n",
                row.example_headers.len()
            ));
            for case in &row.example_headers {
                out.push_str(&format!(
                    "    - `{}` → run `{}` attempt #{} [{}] ({}), {} attempt(s)\n",
                    case.resolved.header_id,
                    case.resolved.header.run_ref,
                    case.resolved.header.attempt_ordinal,
                    case.resolved.header.state_label,
                    case.resolved.header.truth_mode.as_str(),
                    case.resolved.selector.attempt_count,
                ));
            }
        }
        out
    }
}

/// Errors emitted when reading the checked-in M5 run/attempt-header export.
#[derive(Debug)]
pub enum M5RunAttemptArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<M5RunAttemptViolation>),
}

impl fmt::Display for M5RunAttemptArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(
                    formatter,
                    "m5 run/attempt-header primitive export parse failed: {error}"
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
                    "m5 run/attempt-header primitive export failed validation: {tokens}"
                )
            }
        }
    }
}

impl Error for M5RunAttemptArtifactError {}

/// Validation failures emitted by [`M5RunAttemptHeaderPrimitivePacket::validate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum M5RunAttemptViolation {
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
    /// A surface row declares no outcomes.
    OutcomeMissing,
    /// A surface row declares no truth classes.
    TruthModeMissing,
    /// A surface row omits one of the mandatory export fields.
    MandatoryExportFieldMissing,
    /// A surface row declares no downgrade triggers.
    DowngradeTriggersMissing,
    /// A surface row declares no consumer surfaces.
    ConsumerSurfacesMissing,
    /// A surface row declares no worked header cases.
    ExampleHeadersMissing,
    /// A worked header case does not match a fresh resolve of its input.
    ExampleHeaderDrift,
    /// A surface row violates a hard invariant.
    SurfaceInvariantViolated,
    /// No worked case proves one run with multiple attempts distinguishable from
    /// multiple separate runs (AC1).
    IdentityDistinctnessUnproven,
    /// No worked case proves header state labels consistent across surfaces (AC2).
    StateLabelParityUnproven,
    /// No worked case proves exported IDs and states preserved (AC3).
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

impl M5RunAttemptViolation {
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
            Self::OutcomeMissing => "outcome_missing",
            Self::TruthModeMissing => "truth_mode_missing",
            Self::MandatoryExportFieldMissing => "mandatory_export_field_missing",
            Self::DowngradeTriggersMissing => "downgrade_triggers_missing",
            Self::ConsumerSurfacesMissing => "consumer_surfaces_missing",
            Self::ExampleHeadersMissing => "example_headers_missing",
            Self::ExampleHeaderDrift => "example_header_drift",
            Self::SurfaceInvariantViolated => "surface_invariant_violated",
            Self::IdentityDistinctnessUnproven => "identity_distinctness_unproven",
            Self::StateLabelParityUnproven => "state_label_parity_unproven",
            Self::ExportPreservationUnproven => "export_preservation_unproven",
            Self::GovernanceReviewIncomplete => "governance_review_incomplete",
            Self::ConsumerProjectionIncomplete => "consumer_projection_incomplete",
            Self::ReleasePostureIncomplete => "release_posture_incomplete",
            Self::RawMaterialInExport => "raw_material_in_export",
        }
    }
}

/// Reads and validates the checked-in stable M5 run/attempt-header export.
///
/// # Errors
///
/// Returns an artifact error if the export cannot parse or fails validation.
pub fn current_stable_m5_run_attempt_header_export(
) -> Result<M5RunAttemptHeaderPrimitivePacket, M5RunAttemptArtifactError> {
    let packet: M5RunAttemptHeaderPrimitivePacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/release/m5-run-attempt-header-primitive-proof/support_export.json"
    )))
    .map_err(M5RunAttemptArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(M5RunAttemptArtifactError::Validation(violations))
    }
}

fn validate_source_contracts(
    packet: &M5RunAttemptHeaderPrimitivePacket,
    violations: &mut Vec<M5RunAttemptViolation>,
) {
    let refs: BTreeSet<&str> = packet
        .source_contract_refs
        .iter()
        .map(String::as_str)
        .collect();
    for required in [
        M5_RUN_ATTEMPT_HEADER_SCHEMA_REF,
        M5_RUN_ATTEMPT_HEADER_DOC_REF,
        M5_RUN_ATTEMPT_HEADER_COMPONENT_MATRIX_REF,
        M5_RUN_ATTEMPT_HEADER_ARTIFACT_REF,
    ] {
        if !refs.contains(required) {
            violations.push(M5RunAttemptViolation::MissingSourceContracts);
            return;
        }
    }
}

fn validate_vocabulary_set(
    packet: &M5RunAttemptHeaderPrimitivePacket,
    violations: &mut Vec<M5RunAttemptViolation>,
) {
    if !packet.vocabulary_set.matches_canonical() {
        violations.push(M5RunAttemptViolation::VocabularySetDrift);
    }
}

fn validate_surface_rows(
    packet: &M5RunAttemptHeaderPrimitivePacket,
    violations: &mut Vec<M5RunAttemptViolation>,
) {
    let present: BTreeSet<M5RunAttemptSurfaceFamily> = packet
        .surface_rows
        .iter()
        .map(|row| row.surface_family)
        .collect();
    for required in M5RunAttemptSurfaceFamily::ALL {
        if !present.contains(&required) {
            violations.push(M5RunAttemptViolation::RequiredSurfaceMissing);
            return;
        }
    }

    for row in &packet.surface_rows {
        if row.owner_role.trim().is_empty()
            || row.scope_summary.trim().is_empty()
            || row.source_contract_refs.is_empty()
        {
            violations.push(M5RunAttemptViolation::SurfaceRowIncomplete);
        }
        if row.outcomes.is_empty() {
            violations.push(M5RunAttemptViolation::OutcomeMissing);
        }
        if row.truth_modes.is_empty() {
            violations.push(M5RunAttemptViolation::TruthModeMissing);
        }
        if !row.declares_mandatory_export_fields() {
            violations.push(M5RunAttemptViolation::MandatoryExportFieldMissing);
        }
        if row.downgrade_triggers.is_empty() {
            violations.push(M5RunAttemptViolation::DowngradeTriggersMissing);
        }
        if row.consumer_surfaces.is_empty() {
            violations.push(M5RunAttemptViolation::ConsumerSurfacesMissing);
        }
        if row.example_headers.is_empty() {
            violations.push(M5RunAttemptViolation::ExampleHeadersMissing);
        }
        if row
            .example_headers
            .iter()
            .any(|case| !case.is_self_consistent())
        {
            violations.push(M5RunAttemptViolation::ExampleHeaderDrift);
        }
        if !row.honours_invariants() {
            violations.push(M5RunAttemptViolation::SurfaceInvariantViolated);
        }
    }
}

/// The acceptance criteria must each be demonstrated by at least one worked case
/// across the matrix: one run with multiple attempts distinguishable from multiple
/// separate runs (AC1), header state labels consistent across surfaces (AC2), and
/// exported IDs and states preserved (AC3).
fn validate_acceptance_criteria_covered(
    packet: &M5RunAttemptHeaderPrimitivePacket,
    violations: &mut Vec<M5RunAttemptViolation>,
) {
    let cases: Vec<&M5ResolvedRunAttempt> = packet
        .surface_rows
        .iter()
        .flat_map(|row| row.example_headers.iter().map(|case| &case.resolved))
        .collect();

    // AC1: at least one case shows a run with two or more attempts, distinguishable
    // from separate runs, and every case keeps run and attempt distinct.
    let identity_proven = cases.iter().any(|resolved| {
        resolved.distinguishes_attempts_from_runs()
            && resolved.identity_consistent()
            && resolved.selector.attempt_count >= 2
    }) && cases
        .iter()
        .all(|resolved| resolved.run_and_attempt_distinct());
    if !identity_proven {
        violations.push(M5RunAttemptViolation::IdentityDistinctnessUnproven);
    }

    // AC2: at least one outcome is rendered by two or more distinct surface families,
    // and every case keeps its state label consistent across projections.
    let mut outcome_to_families: BTreeMap<M5RunOutcome, BTreeSet<M5RunAttemptSurfaceFamily>> =
        BTreeMap::new();
    for row in &packet.surface_rows {
        for case in &row.example_headers {
            outcome_to_families
                .entry(case.resolved.header.outcome)
                .or_default()
                .insert(row.surface_family);
        }
    }
    let cross_surface = outcome_to_families
        .values()
        .any(|families| families.len() >= 2);
    let labels_proven = cross_surface
        && cases
            .iter()
            .all(|resolved| resolved.state_labels_consistent());
    if !labels_proven {
        violations.push(M5RunAttemptViolation::StateLabelParityUnproven);
    }

    // AC3: every case preserves the run/attempt IDs and visible states into the
    // support-export projection.
    let export_proven = !cases.is_empty()
        && cases
            .iter()
            .all(|resolved| resolved.export_preserves_ids_and_states());
    if !export_proven {
        violations.push(M5RunAttemptViolation::ExportPreservationUnproven);
    }
}

fn validate_governance_review(
    packet: &M5RunAttemptHeaderPrimitivePacket,
    violations: &mut Vec<M5RunAttemptViolation>,
) {
    let review = &packet.governance_review;
    for ok in [
        review.one_primitive_carries_all_surfaces,
        review.run_and_attempt_identity_kept_distinct,
        review.state_labels_consistent_across_surfaces,
        review.queue_and_admission_preserved_in_shared_vocabulary,
        review.exported_evidence_preserves_ids_and_states,
        review.support_export_reconstructs_run_attempt,
        review.later_rows_cannot_invent_parallel_vocabulary,
    ] {
        if !ok {
            violations.push(M5RunAttemptViolation::GovernanceReviewIncomplete);
            return;
        }
    }
}

fn validate_consumer_projection(
    packet: &M5RunAttemptHeaderPrimitivePacket,
    violations: &mut Vec<M5RunAttemptViolation>,
) {
    let projection = &packet.consumer_projection;
    for ok in [
        projection.execution_surfaces_consume_shared_primitive,
        projection.resolver_reads_single_model,
        projection.attempt_selector_reads_single_run_source,
        projection.support_export_reads_single_source,
    ] {
        if !ok {
            violations.push(M5RunAttemptViolation::ConsumerProjectionIncomplete);
            return;
        }
    }
}

fn validate_release_posture(
    packet: &M5RunAttemptHeaderPrimitivePacket,
    violations: &mut Vec<M5RunAttemptViolation>,
) {
    let posture = &packet.release_posture;
    if posture.release_packet_ref.trim().is_empty()
        || posture.header_audit_ref.trim().is_empty()
        || !posture.support_export_parity_required
        || !posture.accessibility_parity_required
    {
        violations.push(M5RunAttemptViolation::ReleasePostureIncomplete);
    }
}

/// Joins tokens for a CSV cell with a `|` separator so a single cell never introduces
/// a stray comma.
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
