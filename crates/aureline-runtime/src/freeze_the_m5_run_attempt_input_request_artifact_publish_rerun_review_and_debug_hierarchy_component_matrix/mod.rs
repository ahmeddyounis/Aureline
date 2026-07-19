//! Frozen reusable execution-lifecycle component matrix: run/attempt headers,
//! input-request prompts, artifact-publish rows, rerun comparison sheets, debug
//! session headers, thread/process trees, and dump/crash artifact cards.
//!
//! Where [`crate::m5_task_event_envelope_bus`] freezes the canonical *task-event*
//! envelope, [`crate::stabilize_problem_records_output_channels_and_execution_evidence`]
//! stabilizes *output/problem* causality, [`crate::rerun`] and
//! [`crate::run_lineage`] carry *run/attempt lineage*, and the `aureline-debug`
//! contracts carry *debug session* truth, this module freezes the reusable
//! **execution-lifecycle component** contract: the headers, prompts, rows, sheets,
//! trees, and cards users actually rely on to understand execution identity, retry
//! scope, and captured-versus-live control truth before acting, so later M5 rows
//! reference one canonical component family instead of restating run/debug identity
//! truth in feature-local prose.
//!
//! One [`ExecutionLifecycleComponentMatrix`] packet defines every reusable
//! primitive, its state vocabulary, its required labels, and its export / assistive
//! parity expectations, binding each onto the same run-state, approval, retention,
//! execution-boundary, and captured-versus-live vocabulary already used across
//! Aureline's task-event, output/problem, request, notebook, and debug contracts —
//! never bespoke per-runtime or per-adapter chrome.
//!
//! The honesty rules the spec freezes, carried by every [`ComponentRow`]:
//!
//! - **Run identity and attempt identity stay distinct.** A run/attempt header
//!   never collapses the run it belongs to with the individual attempt it renders.
//! - **Outcomes remain stable across UI / CLI / export.** Queued, preparing,
//!   running, waiting-input, partially-complete, passed, failed, cancelled, and
//!   stale-output are one closed vocabulary bound to a captured-versus-live truth
//!   class; a stale output never reads as a live run.
//! - **Produced artifacts never lose producing-run lineage or retention truth.**
//!   An artifact-publish row and a dump/crash card always name the run that
//!   produced them and disclose whether the artifact is retained, expiring, or
//!   evicted.
//! - **Rerun controls disclose exact-versus-current-context differences before
//!   dispatch.** A rerun comparison sheet shows the context delta before it lets a
//!   run start, never after.
//! - **Debug hierarchy and cards keep launch / attach / core / replay /
//!   inspect-only, live-versus-captured, and local / remote / container / managed
//!   boundaries explicit.** A debug session header, thread/process tree, and
//!   dump/crash card never flatten live control and captured evidence into one
//!   generic session story.
//!
//! Raw run logs, raw stdout/stderr bytes, raw crash dumps, provider cursors,
//! credentials, and raw event payloads never cross this boundary; the packet
//! carries only typed class tokens, opaque run / attempt / artifact / evidence
//! refs, booleans, and redacted labels, so support and diagnostics exports can
//! reconstruct exactly what a component would have shown without leaking source or
//! live payloads.
//!
//! The boundary schema is
//! [`schemas/ui/m5-execution-lifecycle-component-matrix.schema.json`](../../../../schemas/ui/m5-execution-lifecycle-component-matrix.schema.json).
//! The contract doc is
//! [`docs/run-test-debug/m5_execution_lifecycle_component_matrix.md`](../../../../docs/run-test-debug/m5_execution_lifecycle_component_matrix.md).
//! The protected fixture directory is
//! [`fixtures/ui/m5-execution-lifecycle-components/`](../../../../fixtures/ui/m5-execution-lifecycle-components/).

#[cfg(test)]
mod tests;

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

/// Stable record-kind tag carried by [`ExecutionLifecycleComponentMatrix`].
pub const EXECUTION_LIFECYCLE_COMPONENT_MATRIX_RECORD_KIND: &str =
    "m5_execution_lifecycle_component_matrix";

/// Schema version for the execution-lifecycle component matrix packet.
pub const EXECUTION_LIFECYCLE_COMPONENT_MATRIX_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the boundary schema.
pub const EXECUTION_LIFECYCLE_COMPONENT_MATRIX_SCHEMA_REF: &str =
    "schemas/ui/m5-execution-lifecycle-component-matrix.schema.json";

/// Repo-relative path of the contract doc.
pub const EXECUTION_LIFECYCLE_COMPONENT_MATRIX_DOC_REF: &str =
    "docs/run-test-debug/m5_execution_lifecycle_component_matrix.md";

/// Repo-relative path of the protected fixture directory.
pub const EXECUTION_LIFECYCLE_COMPONENT_MATRIX_FIXTURE_DIR: &str =
    "fixtures/ui/m5-execution-lifecycle-components";

/// Repo-relative path of the checked support-export artifact (the `include_str!`
/// canonical).
pub const EXECUTION_LIFECYCLE_COMPONENT_MATRIX_ARTIFACT_REF: &str =
    "artifacts/release/m5-execution-lifecycle-component-proof/support_export.json";

/// Repo-relative path of the checked Markdown matrix summary.
pub const EXECUTION_LIFECYCLE_COMPONENT_MATRIX_SUMMARY_REF: &str =
    "artifacts/design/m5-execution-lifecycle-component-matrix.md";

/// Closed reusable execution-lifecycle component family. Each family is one
/// governed primitive later M5 rows reference by name; the matrix must define every
/// one.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ExecutionComponentFamily {
    /// A run/attempt header framing run-versus-attempt identity and outcome.
    RunAttemptHeader,
    /// An input-request prompt disclosing timeout / approval consequences.
    InputRequestPrompt,
    /// An artifact-publish row keeping producing-run lineage and retention truth.
    ArtifactPublishRow,
    /// A rerun comparison sheet disclosing exact-versus-current-context differences.
    RerunComparisonSheet,
    /// A debug session header framing launch / attach / core / replay / inspect-only.
    DebugSessionHeader,
    /// A thread / process tree disclosing live-versus-captured hierarchy truth.
    ThreadProcessTree,
    /// A dump / crash artifact card keeping producing-run lineage and symbolication.
    DumpCrashArtifactCard,
}

impl M5ExecutionComponentFamily {
    /// Every reusable component family the matrix must define, in declaration order.
    pub const ALL: [Self; 7] = [
        Self::RunAttemptHeader,
        Self::InputRequestPrompt,
        Self::ArtifactPublishRow,
        Self::RerunComparisonSheet,
        Self::DebugSessionHeader,
        Self::ThreadProcessTree,
        Self::DumpCrashArtifactCard,
    ];

    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::RunAttemptHeader => "run_attempt_header",
            Self::InputRequestPrompt => "input_request_prompt",
            Self::ArtifactPublishRow => "artifact_publish_row",
            Self::RerunComparisonSheet => "rerun_comparison_sheet",
            Self::DebugSessionHeader => "debug_session_header",
            Self::ThreadProcessTree => "thread_process_tree",
            Self::DumpCrashArtifactCard => "dump_crash_artifact_card",
        }
    }
}

/// Closed captured-versus-live truth class. Names whether a component renders a
/// live run, captured evidence, imported external truth, a planned/not-yet-started
/// run, or a provider-reported overlay, so captured evidence never reads as live
/// control.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ExecutionTruthMode {
    /// A live, actively executing run under live control.
    Live,
    /// Captured evidence from a completed or recorded run (replay / core / logs).
    Captured,
    /// Imported external truth (e.g. a CI run) that is read-only locally.
    Imported,
    /// A planned / queued run that has not started executing.
    Planned,
    /// A provider-reported overlay the provider owns and completes.
    ProviderReported,
}

impl M5ExecutionTruthMode {
    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Live => "live",
            Self::Captured => "captured",
            Self::Imported => "imported",
            Self::Planned => "planned",
            Self::ProviderReported => "provider_reported",
        }
    }

    /// True when this truth class is live control rather than captured evidence.
    pub const fn is_live(self) -> bool {
        matches!(self, Self::Live)
    }
}

/// Closed execution-boundary vocabulary. Names where a run / debug session executes
/// so a remote, container, or managed execution never reads as a local one.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ExecutionLocality {
    /// Executes on the local machine.
    Local,
    /// Executes on a remote host.
    Remote,
    /// Executes inside a container / devcontainer.
    Container,
    /// Executes in a managed / provider-hosted environment.
    Managed,
}

impl M5ExecutionLocality {
    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Local => "local",
            Self::Remote => "remote",
            Self::Container => "container",
            Self::Managed => "managed",
        }
    }
}

/// Closed run-outcome vocabulary. Names the stable outcome a run / attempt is in so
/// the same outcome reads identically across UI, CLI, and export.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5RunOutcome {
    /// Queued, not yet scheduled to run.
    Queued,
    /// Preparing (resolving toolchain / environment) before execution.
    Preparing,
    /// Actively running.
    Running,
    /// Paused waiting for an input request to be answered.
    WaitingInput,
    /// Partially complete: some units done, some still pending.
    PartiallyComplete,
    /// Completed successfully.
    Passed,
    /// Completed with failure.
    Failed,
    /// Cancelled before completion.
    Cancelled,
    /// Output is stale: from a prior run superseded by a source change.
    StaleOutput,
}

impl M5RunOutcome {
    /// Every stable outcome, in declaration order.
    pub const ALL: [Self; 9] = [
        Self::Queued,
        Self::Preparing,
        Self::Running,
        Self::WaitingInput,
        Self::PartiallyComplete,
        Self::Passed,
        Self::Failed,
        Self::Cancelled,
        Self::StaleOutput,
    ];

    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Queued => "queued",
            Self::Preparing => "preparing",
            Self::Running => "running",
            Self::WaitingInput => "waiting_input",
            Self::PartiallyComplete => "partially_complete",
            Self::Passed => "passed",
            Self::Failed => "failed",
            Self::Cancelled => "cancelled",
            Self::StaleOutput => "stale_output",
        }
    }

    /// True when the outcome denotes an actively executing run under live control.
    pub const fn is_active(self) -> bool {
        matches!(self, Self::Preparing | Self::Running | Self::WaitingInput)
    }

    /// True when the outcome is terminal (no further progress without a rerun).
    pub const fn is_terminal(self) -> bool {
        matches!(self, Self::Passed | Self::Failed | Self::Cancelled)
    }
}

/// A run/attempt header descriptor. Present only on a
/// [`M5ExecutionComponentFamily::RunAttemptHeader`] row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RunAttemptHeaderDescriptor {
    /// Opaque ref to the run identity; never raw run bytes.
    pub run_identity_ref: String,
    /// Opaque ref to the attempt identity; distinct from the run identity.
    pub attempt_identity_ref: String,
    /// 1-based ordinal of this attempt within the run.
    pub attempt_ordinal: u32,
    /// The stable outcome the header renders.
    pub outcome: M5RunOutcome,
    /// The captured-versus-live truth class; must match the row's truth mode.
    pub truth_mode: M5ExecutionTruthMode,
    /// The header keeps run and attempt identity distinct; must always hold.
    pub run_and_attempt_distinct: bool,
}

impl RunAttemptHeaderDescriptor {
    /// Whether the run/attempt header descriptor is internally complete and honest:
    /// run and attempt identity stay distinct, an active outcome is shown as live
    /// truth, and a stale output is never shown as live.
    pub fn is_honest(&self) -> bool {
        if self.run_identity_ref.trim().is_empty() || self.attempt_identity_ref.trim().is_empty() {
            return false;
        }
        if !self.run_and_attempt_distinct || self.run_identity_ref == self.attempt_identity_ref {
            return false;
        }
        if self.attempt_ordinal == 0 {
            return false;
        }
        // An actively executing outcome must be shown as live truth; captured or
        // stale outcomes must never claim live control.
        if self.outcome.is_active() && !self.truth_mode.is_live() {
            return false;
        }
        if self.outcome == M5RunOutcome::StaleOutput && self.truth_mode.is_live() {
            return false;
        }
        true
    }
}

/// Closed input-request-consequence vocabulary. Names what happens when an input
/// request times out or is dismissed so a prompt never hides its consequence.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5InputConsequence {
    /// On timeout the run is cancelled.
    TimeoutCancelsRun,
    /// On timeout a declared default value is applied.
    TimeoutAppliesDefault,
    /// The input requires explicit approval before the run may proceed.
    RequiresApproval,
    /// The run blocks indefinitely until the input is answered.
    BlocksUntilAnswered,
    /// Dismissing the prompt leaves the run waiting rather than cancelling it.
    DismissLeavesWaiting,
}

impl M5InputConsequence {
    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::TimeoutCancelsRun => "timeout_cancels_run",
            Self::TimeoutAppliesDefault => "timeout_applies_default",
            Self::RequiresApproval => "requires_approval",
            Self::BlocksUntilAnswered => "blocks_until_answered",
            Self::DismissLeavesWaiting => "dismiss_leaves_waiting",
        }
    }

    /// True when the consequence is governed by a timeout deadline.
    pub const fn needs_deadline(self) -> bool {
        matches!(self, Self::TimeoutCancelsRun | Self::TimeoutAppliesDefault)
    }
}

/// An input-request prompt descriptor. Present only on a
/// [`M5ExecutionComponentFamily::InputRequestPrompt`] row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct InputRequestPromptDescriptor {
    /// Opaque ref to the input request; never raw prompt bytes.
    pub prompt_ref: String,
    /// What happens on timeout / dismissal.
    pub consequence: M5InputConsequence,
    /// The prompt discloses its timeout behaviour; must always hold.
    pub discloses_timeout: bool,
    /// The prompt discloses its approval requirement; must always hold.
    pub discloses_approval: bool,
    /// The prompt carries a resolvable deadline; required when the consequence is
    /// timeout-governed.
    pub has_deadline: bool,
}

impl InputRequestPromptDescriptor {
    /// Whether the input-request prompt descriptor is internally complete and honest:
    /// it names its request, discloses both timeout and approval consequences, and
    /// carries a deadline when the consequence is timeout-governed.
    pub fn is_honest(&self) -> bool {
        if self.prompt_ref.trim().is_empty() {
            return false;
        }
        if !self.discloses_timeout || !self.discloses_approval {
            return false;
        }
        if self.consequence.needs_deadline() && !self.has_deadline {
            return false;
        }
        true
    }
}

/// Closed artifact-retention vocabulary. Names how long a produced artifact is
/// retained so an evicted or expiring artifact never reads as durably retained.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5RetentionClass {
    /// Retained durably; safe to reference indefinitely.
    RetainedDurable,
    /// Retained but scheduled to expire.
    ExpiresScheduled,
    /// Ephemeral: exists only for the current session.
    EphemeralSessionOnly,
    /// Evicted but recoverable (e.g. rebuildable from lineage).
    EvictedRecoverable,
    /// Evicted and gone; no longer retrievable.
    EvictedGone,
}

impl M5RetentionClass {
    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::RetainedDurable => "retained_durable",
            Self::ExpiresScheduled => "expires_scheduled",
            Self::EphemeralSessionOnly => "ephemeral_session_only",
            Self::EvictedRecoverable => "evicted_recoverable",
            Self::EvictedGone => "evicted_gone",
        }
    }

    /// True when the artifact is no longer live-available and must disclose so.
    pub const fn is_evicted(self) -> bool {
        matches!(self, Self::EvictedRecoverable | Self::EvictedGone)
    }
}

/// An artifact-publish row descriptor. Present only on a
/// [`M5ExecutionComponentFamily::ArtifactPublishRow`] row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ArtifactPublishRowDescriptor {
    /// Opaque ref to the published artifact; never raw artifact bytes.
    pub artifact_ref: String,
    /// Opaque ref to the run that produced the artifact; lineage is never lost.
    pub producing_run_ref: String,
    /// How long the artifact is retained.
    pub retention: M5RetentionClass,
    /// The producing-run lineage stays attached; must always hold.
    pub lineage_preserved: bool,
    /// The retention state is visible on the row; must always hold.
    pub retention_visible: bool,
}

impl ArtifactPublishRowDescriptor {
    /// Whether the artifact-publish descriptor is internally complete and honest: it
    /// names both the artifact and its producing run, preserves lineage, and keeps
    /// retention visible.
    pub fn is_honest(&self) -> bool {
        !self.artifact_ref.trim().is_empty()
            && !self.producing_run_ref.trim().is_empty()
            && self.lineage_preserved
            && self.retention_visible
    }
}

/// Closed rerun-context vocabulary. Names whether a rerun replays the exact prior
/// context or uses a changed one so a context drift is never dispatched silently.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5RerunContext {
    /// Replays the exact prior selection, environment, and inputs.
    ExactReplay,
    /// Reruns against the current (possibly changed) context.
    CurrentContext,
    /// Reruns with a modified selection of units.
    ModifiedSelection,
    /// Reruns with a modified environment / toolchain.
    ModifiedEnvironment,
}

impl M5RerunContext {
    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ExactReplay => "exact_replay",
            Self::CurrentContext => "current_context",
            Self::ModifiedSelection => "modified_selection",
            Self::ModifiedEnvironment => "modified_environment",
        }
    }

    /// True when the rerun context differs from an exact replay and so must surface a
    /// context delta before dispatch.
    pub const fn differs_from_exact(self) -> bool {
        !matches!(self, Self::ExactReplay)
    }
}

/// A rerun comparison sheet descriptor. Present only on a
/// [`M5ExecutionComponentFamily::RerunComparisonSheet`] row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RerunComparisonSheetDescriptor {
    /// Opaque ref to the baseline run being compared against.
    pub baseline_run_ref: String,
    /// Whether the rerun replays the exact context or a changed one.
    pub rerun_context: M5RerunContext,
    /// The sheet discloses the exact-versus-current-context difference; must hold.
    pub discloses_context_delta: bool,
    /// The context diff is shown before dispatch, never after; required when the
    /// context differs from an exact replay.
    pub context_diff_shown_before_dispatch: bool,
}

impl RerunComparisonSheetDescriptor {
    /// Whether the rerun-comparison descriptor is internally complete and honest: it
    /// names its baseline, discloses the context delta, and shows a changed-context
    /// diff before dispatch rather than after.
    pub fn is_honest(&self) -> bool {
        if self.baseline_run_ref.trim().is_empty() || !self.discloses_context_delta {
            return false;
        }
        if self.rerun_context.differs_from_exact() && !self.context_diff_shown_before_dispatch {
            return false;
        }
        true
    }
}

/// Closed debug-session-mode vocabulary. Names how a debug session was established
/// so live control (launch / attach) never blurs with captured evidence (core /
/// replay / inspect-only).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5DebugSessionMode {
    /// The debugger launched the target process.
    Launch,
    /// The debugger attached to a running process.
    Attach,
    /// A post-mortem core / crash dump session.
    Core,
    /// A time-travel / recorded replay session.
    Replay,
    /// An inspect-only session with no live control.
    InspectOnly,
}

impl M5DebugSessionMode {
    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Launch => "launch",
            Self::Attach => "attach",
            Self::Core => "core",
            Self::Replay => "replay",
            Self::InspectOnly => "inspect_only",
        }
    }

    /// True when this mode grants live control of a running target.
    pub const fn is_live_control(self) -> bool {
        matches!(self, Self::Launch | Self::Attach)
    }
}

/// A debug session header descriptor. Present only on a
/// [`M5ExecutionComponentFamily::DebugSessionHeader`] row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DebugSessionHeaderDescriptor {
    /// Opaque ref to the debug session; never raw process bytes.
    pub session_ref: String,
    /// How the session was established.
    pub session_mode: M5DebugSessionMode,
    /// The captured-versus-live truth class; must match the row's truth mode.
    pub truth_mode: M5ExecutionTruthMode,
    /// The local / remote / container / managed execution boundary; must match the
    /// row's locality.
    pub locality: M5ExecutionLocality,
    /// The session's boundary and truth class are explicit; must always hold.
    pub boundary_explicit: bool,
}

impl DebugSessionHeaderDescriptor {
    /// Whether the debug-session-header descriptor is internally complete and honest:
    /// a live-control mode is shown as live truth and a captured mode (core / replay /
    /// inspect-only) is never shown as live control.
    pub fn is_honest(&self) -> bool {
        if self.session_ref.trim().is_empty() || !self.boundary_explicit {
            return false;
        }
        if self.session_mode.is_live_control() {
            self.truth_mode.is_live()
        } else {
            !self.truth_mode.is_live()
        }
    }
}

/// A thread / process tree descriptor. Present only on a
/// [`M5ExecutionComponentFamily::ThreadProcessTree`] row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ThreadProcessTreeDescriptor {
    /// Opaque ref to the root process / thread node.
    pub root_ref: String,
    /// Number of nodes in the tree; at least one.
    pub node_count: u32,
    /// The captured-versus-live truth class; must match the row's truth mode.
    pub truth_mode: M5ExecutionTruthMode,
    /// The local / remote / container / managed execution boundary; must match the
    /// row's locality.
    pub locality: M5ExecutionLocality,
    /// The tree marks whether it is live or captured; must always hold.
    pub live_vs_captured_explicit: bool,
}

impl ThreadProcessTreeDescriptor {
    /// Whether the thread/process-tree descriptor is internally complete and honest.
    pub fn is_honest(&self) -> bool {
        !self.root_ref.trim().is_empty() && self.node_count >= 1 && self.live_vs_captured_explicit
    }
}

/// Closed symbolication-state vocabulary. Names how well a dump is symbolicated so an
/// unsymbolicated dump never reads as fully symbolicated.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5SymbolicationState {
    /// Fully symbolicated.
    Symbolicated,
    /// Partially symbolicated (some frames resolved).
    PartialSymbols,
    /// Not symbolicated (raw addresses only).
    Unsymbolicated,
    /// Symbols could not be resolved at all.
    SymbolsUnavailable,
}

impl M5SymbolicationState {
    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Symbolicated => "symbolicated",
            Self::PartialSymbols => "partial_symbols",
            Self::Unsymbolicated => "unsymbolicated",
            Self::SymbolsUnavailable => "symbols_unavailable",
        }
    }
}

/// A dump / crash artifact card descriptor. Present only on a
/// [`M5ExecutionComponentFamily::DumpCrashArtifactCard`] row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DumpCrashArtifactCardDescriptor {
    /// Opaque ref to the dump / crash artifact; never raw dump bytes.
    pub dump_ref: String,
    /// Opaque ref to the run that produced the dump; lineage is never lost.
    pub producing_run_ref: String,
    /// How well the dump is symbolicated.
    pub symbolication: M5SymbolicationState,
    /// How long the dump artifact is retained.
    pub retention: M5RetentionClass,
    /// A dump is captured evidence, never live control; must always hold.
    pub captured_truth: bool,
}

impl DumpCrashArtifactCardDescriptor {
    /// Whether the dump/crash-card descriptor is internally complete and honest: it
    /// names both the dump and its producing run, and is always captured truth.
    pub fn is_honest(&self) -> bool {
        !self.dump_ref.trim().is_empty()
            && !self.producing_run_ref.trim().is_empty()
            && self.captured_truth
    }
}

/// Closed required-label vocabulary. Names the labels a reusable execution-lifecycle
/// component must render; the mandatory subset must appear on every row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ExecutionRequiredLabel {
    /// The component's stable identity.
    Identity,
    /// The run / attempt / execution context the component acts on.
    ExecutionContext,
    /// The captured-versus-live truth class.
    TruthClass,
    /// The run outcome or component state.
    OutcomeOrState,
    /// The local / remote / container / managed execution boundary.
    ExecutionBoundary,
    /// The keyboard / assistive route into the component.
    KeyboardRoute,
}

impl M5ExecutionRequiredLabel {
    /// Every required label, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::Identity,
        Self::ExecutionContext,
        Self::TruthClass,
        Self::OutcomeOrState,
        Self::ExecutionBoundary,
        Self::KeyboardRoute,
    ];

    /// The mandatory subset that must appear on every row.
    pub const MANDATORY: [Self; 4] = [
        Self::Identity,
        Self::ExecutionContext,
        Self::TruthClass,
        Self::KeyboardRoute,
    ];

    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Identity => "identity",
            Self::ExecutionContext => "execution_context",
            Self::TruthClass => "truth_class",
            Self::OutcomeOrState => "outcome_or_state",
            Self::ExecutionBoundary => "execution_boundary",
            Self::KeyboardRoute => "keyboard_route",
        }
    }
}

/// Closed downgrade-trigger vocabulary. Names why a component row is in a degraded
/// state so support can reconstruct the narrowing.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ExecutionDowngradeTrigger {
    /// The run / attempt identity could not be fully resolved.
    RunAttemptIdentityUnresolved,
    /// The input-request consequence could not be established.
    InputConsequenceUnknown,
    /// A produced artifact lost its producing-run lineage.
    ArtifactLineageLost,
    /// A produced artifact's retention expired or was evicted.
    ArtifactRetentionExpired,
    /// A rerun context drifted from the baseline.
    RerunContextDrift,
    /// Only captured evidence is available; no live control.
    CapturedEvidenceOnly,
    /// A live connector to the run / session was lost.
    ConnectorLost,
    /// The debug adapter was unavailable.
    DebugAdapterUnavailable,
    /// The dump's symbols could not be resolved.
    SymbolsUnavailable,
}

impl M5ExecutionDowngradeTrigger {
    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::RunAttemptIdentityUnresolved => "run_attempt_identity_unresolved",
            Self::InputConsequenceUnknown => "input_consequence_unknown",
            Self::ArtifactLineageLost => "artifact_lineage_lost",
            Self::ArtifactRetentionExpired => "artifact_retention_expired",
            Self::RerunContextDrift => "rerun_context_drift",
            Self::CapturedEvidenceOnly => "captured_evidence_only",
            Self::ConnectorLost => "connector_lost",
            Self::DebugAdapterUnavailable => "debug_adapter_unavailable",
            Self::SymbolsUnavailable => "symbols_unavailable",
        }
    }
}

/// A typed degraded-state block. When present, the component is narrowed below its
/// full capability and names why with an explicit, non-generic label.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DegradedState {
    /// Why the component is degraded.
    pub trigger: M5ExecutionDowngradeTrigger,
    /// A precise, non-generic label safe to render.
    pub degraded_label: String,
}

impl DegradedState {
    /// Whether the degraded label is precise rather than a generic non-answer.
    pub fn is_honest(&self) -> bool {
        !label_is_generic(&self.degraded_label)
    }
}

/// One reusable execution-lifecycle component: the shared truth row every consumer
/// surface ingests instead of cloning run / debug chrome.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ComponentRow {
    /// Stable component id.
    pub component_id: String,
    /// Which reusable component family this row is.
    pub family: M5ExecutionComponentFamily,
    /// Human-readable label of the surface the component appears on.
    pub surface_label: String,
    /// The captured-versus-live truth class the component binds to.
    pub truth_mode: M5ExecutionTruthMode,
    /// The local / remote / container / managed execution boundary the component acts
    /// in.
    pub locality: M5ExecutionLocality,
    /// Opaque ref to the run / attempt / execution context the component acts on;
    /// execution context stays visible on every surface, so this is never empty.
    pub execution_context_ref: String,
    /// The required labels this component renders; must include every mandatory label.
    pub required_labels: Vec<M5ExecutionRequiredLabel>,
    /// The component projects an export-safe support summary; must hold.
    pub export_safe: bool,
    /// The component exposes a keyboard / assistive route; must hold.
    pub assistive_ready: bool,
    /// The run/attempt header descriptor, present only for a header row.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub run_attempt_header: Option<RunAttemptHeaderDescriptor>,
    /// The input-request prompt descriptor, present only for a prompt row.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub input_request_prompt: Option<InputRequestPromptDescriptor>,
    /// The artifact-publish descriptor, present only for an artifact row.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub artifact_publish_row: Option<ArtifactPublishRowDescriptor>,
    /// The rerun-comparison descriptor, present only for a rerun-sheet row.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rerun_comparison_sheet: Option<RerunComparisonSheetDescriptor>,
    /// The debug-session-header descriptor, present only for a debug-header row.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub debug_session_header: Option<DebugSessionHeaderDescriptor>,
    /// The thread/process-tree descriptor, present only for a tree row.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub thread_process_tree: Option<ThreadProcessTreeDescriptor>,
    /// The dump/crash-card descriptor, present only for a dump-card row.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dump_crash_artifact_card: Option<DumpCrashArtifactCardDescriptor>,
    /// The typed degraded-state block, present only when the component is narrowed.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub degraded: Option<DegradedState>,
    /// Human-readable label summary safe to render on the row.
    pub label_summary: String,
    /// ISO 8601 UTC timestamp the component state was observed.
    pub observed_at: String,
    /// Evidence packet refs backing this row.
    pub evidence_refs: Vec<String>,
}

impl ComponentRow {
    /// Whether the family-specific payload is present exactly for this family and
    /// absent for every other family.
    pub fn payload_matches_family(&self) -> bool {
        let present = [
            self.run_attempt_header.is_some(),
            self.input_request_prompt.is_some(),
            self.artifact_publish_row.is_some(),
            self.rerun_comparison_sheet.is_some(),
            self.debug_session_header.is_some(),
            self.thread_process_tree.is_some(),
            self.dump_crash_artifact_card.is_some(),
        ];
        // Exactly one payload present, and it is the one this family names.
        if present.iter().filter(|p| **p).count() != 1 {
            return false;
        }
        match self.family {
            M5ExecutionComponentFamily::RunAttemptHeader => self.run_attempt_header.is_some(),
            M5ExecutionComponentFamily::InputRequestPrompt => self.input_request_prompt.is_some(),
            M5ExecutionComponentFamily::ArtifactPublishRow => self.artifact_publish_row.is_some(),
            M5ExecutionComponentFamily::RerunComparisonSheet => {
                self.rerun_comparison_sheet.is_some()
            }
            M5ExecutionComponentFamily::DebugSessionHeader => self.debug_session_header.is_some(),
            M5ExecutionComponentFamily::ThreadProcessTree => self.thread_process_tree.is_some(),
            M5ExecutionComponentFamily::DumpCrashArtifactCard => {
                self.dump_crash_artifact_card.is_some()
            }
        }
    }

    /// Whether the family payload, where present, is internally honest.
    pub fn payload_honest(&self) -> bool {
        self.run_attempt_header
            .as_ref()
            .map_or(true, |d| d.is_honest())
            && self
                .input_request_prompt
                .as_ref()
                .map_or(true, |d| d.is_honest())
            && self
                .artifact_publish_row
                .as_ref()
                .map_or(true, |d| d.is_honest())
            && self
                .rerun_comparison_sheet
                .as_ref()
                .map_or(true, |d| d.is_honest())
            && self
                .debug_session_header
                .as_ref()
                .map_or(true, |d| d.is_honest())
            && self
                .thread_process_tree
                .as_ref()
                .map_or(true, |d| d.is_honest())
            && self
                .dump_crash_artifact_card
                .as_ref()
                .map_or(true, |d| d.is_honest())
    }

    /// Whether a truth-bearing descriptor discloses the same truth class and
    /// execution boundary the row records (a header / debug header / tree never
    /// invents a second execution story).
    pub fn descriptor_matches_row(&self) -> bool {
        let header_ok = self
            .run_attempt_header
            .as_ref()
            .map_or(true, |h| h.truth_mode == self.truth_mode);
        let debug_ok = self.debug_session_header.as_ref().map_or(true, |d| {
            d.truth_mode == self.truth_mode && d.locality == self.locality
        });
        let tree_ok = self.thread_process_tree.as_ref().map_or(true, |t| {
            t.truth_mode == self.truth_mode && t.locality == self.locality
        });
        header_ok && debug_ok && tree_ok
    }

    /// Whether every mandatory required label is present on the row.
    pub fn mandatory_labels_present(&self) -> bool {
        let present: BTreeSet<M5ExecutionRequiredLabel> =
            self.required_labels.iter().copied().collect();
        M5ExecutionRequiredLabel::MANDATORY
            .iter()
            .all(|label| present.contains(label))
    }

    /// Whether the degraded block, when present, is honest.
    pub fn degraded_ok(&self) -> bool {
        self.degraded.as_ref().map_or(true, |d| d.is_honest())
    }

    /// True when this row is a complete, honest degraded / narrowed component.
    pub fn is_degraded(&self) -> bool {
        self.degraded.is_some() && self.is_complete()
    }

    /// Deterministic governed chip line for this row.
    pub fn chip_tokens(&self) -> String {
        format!(
            "family={family} truth={truth} locality={locality} \
export_safe={export_safe} assistive={assistive}",
            family = self.family.as_str(),
            truth = self.truth_mode.as_str(),
            locality = self.locality.as_str(),
            export_safe = self.export_safe,
            assistive = self.assistive_ready,
        )
    }

    /// Whether every dimension required to record this row is present and internally
    /// consistent.
    pub fn is_complete(&self) -> bool {
        !self.component_id.trim().is_empty()
            && !self.surface_label.trim().is_empty()
            && !self.execution_context_ref.trim().is_empty()
            && !self.label_summary.trim().is_empty()
            && !self.observed_at.trim().is_empty()
            && self.export_safe
            && self.assistive_ready
            && self.payload_matches_family()
            && self.payload_honest()
            && self.descriptor_matches_row()
            && self.mandatory_labels_present()
            && self.degraded_ok()
            && !self.evidence_refs.is_empty()
            && self.evidence_refs.iter().all(|r| !r.trim().is_empty())
    }
}

/// Guardrail invariants block for the execution-lifecycle component matrix.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExecutionLifecycleGuardrails {
    /// Run identity and attempt identity stay distinct.
    pub run_and_attempt_identity_distinct: bool,
    /// Queued / preparing / running / waiting-input / partially-complete / passed /
    /// failed / cancelled / stale-output outcomes stay stable across UI / CLI /
    /// export.
    pub outcomes_stable_across_ui_cli_export: bool,
    /// Produced artifacts never lose producing-run lineage or retention truth.
    pub artifacts_never_lose_lineage_or_retention: bool,
    /// Rerun controls disclose exact-versus-current-context differences before
    /// dispatch.
    pub rerun_discloses_context_delta_before_dispatch: bool,
    /// Debug hierarchy / cards keep launch / attach / core / replay / inspect-only,
    /// live-versus-captured, and local / remote / container / managed explicit.
    pub debug_keeps_mode_truth_and_boundary_explicit: bool,
    /// Exported evidence preserves the same run / attempt IDs, outcome states, and
    /// lineage shown in-product.
    pub exported_evidence_preserves_ids_states_and_lineage: bool,
    /// Components bind to the shared run-state, approval, retention, and
    /// captured-versus-live vocabulary rather than bespoke runtime / adapter chrome.
    pub components_bound_to_shared_vocabulary: bool,
    /// The matrix does not widen into new runtimes, debug adapters, provider control
    /// APIs, or artifact backends.
    pub no_new_runtimes_adapters_or_backends: bool,
}

impl ExecutionLifecycleGuardrails {
    /// Whether every guardrail invariant holds.
    pub fn all_hold(&self) -> bool {
        self.run_and_attempt_identity_distinct
            && self.outcomes_stable_across_ui_cli_export
            && self.artifacts_never_lose_lineage_or_retention
            && self.rerun_discloses_context_delta_before_dispatch
            && self.debug_keeps_mode_truth_and_boundary_explicit
            && self.exported_evidence_preserves_ids_states_and_lineage
            && self.components_bound_to_shared_vocabulary
            && self.no_new_runtimes_adapters_or_backends
    }
}

/// Consumer-projection block for the execution-lifecycle component matrix.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExecutionLifecycleConsumerProjection {
    /// Product surfaces ingest these component rows instead of cloning chrome.
    pub product_ingests_components: bool,
    /// Docs / help ingests the same component rows.
    pub docs_help_ingests_components: bool,
    /// Diagnostics ingests the same component rows.
    pub diagnostics_ingests_components: bool,
    /// Support export ingests the same component rows.
    pub support_export_ingests_components: bool,
    /// Release-control surfaces ingest the same component rows.
    pub release_control_ingests_components: bool,
    /// Later M5 rows reference one canonical component family instead of restating
    /// run / debug identity truth in feature-local prose.
    pub later_rows_reference_one_canonical_family: bool,
}

impl ExecutionLifecycleConsumerProjection {
    /// Whether every consumer-projection invariant holds.
    pub fn all_hold(&self) -> bool {
        self.product_ingests_components
            && self.docs_help_ingests_components
            && self.diagnostics_ingests_components
            && self.support_export_ingests_components
            && self.release_control_ingests_components
            && self.later_rows_reference_one_canonical_family
    }
}

/// Constructor input for [`ExecutionLifecycleComponentMatrix::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExecutionLifecycleComponentMatrixInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable matrix label.
    pub set_label: String,
    /// Per-component rows.
    pub components: Vec<ComponentRow>,
    /// Guardrail invariants block.
    pub guardrails: ExecutionLifecycleGuardrails,
    /// Consumer projection block.
    pub consumer_projection: ExecutionLifecycleConsumerProjection,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

/// Export-safe execution-lifecycle component matrix packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExecutionLifecycleComponentMatrix {
    /// Record kind; must equal [`EXECUTION_LIFECYCLE_COMPONENT_MATRIX_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal
    /// [`EXECUTION_LIFECYCLE_COMPONENT_MATRIX_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable matrix label.
    pub set_label: String,
    /// Per-component rows.
    pub components: Vec<ComponentRow>,
    /// Guardrail invariants block.
    pub guardrails: ExecutionLifecycleGuardrails,
    /// Consumer projection block.
    pub consumer_projection: ExecutionLifecycleConsumerProjection,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

impl ExecutionLifecycleComponentMatrix {
    /// Builds an execution-lifecycle component matrix packet.
    pub fn new(input: ExecutionLifecycleComponentMatrixInput) -> Self {
        Self {
            record_kind: EXECUTION_LIFECYCLE_COMPONENT_MATRIX_RECORD_KIND.to_owned(),
            schema_version: EXECUTION_LIFECYCLE_COMPONENT_MATRIX_SCHEMA_VERSION,
            packet_id: input.packet_id,
            set_label: input.set_label,
            components: input.components,
            guardrails: input.guardrails,
            consumer_projection: input.consumer_projection,
            source_contract_refs: input.source_contract_refs,
            redaction_class_token: input.redaction_class_token,
            minted_at: input.minted_at,
        }
    }

    /// Families represented by some row in this matrix.
    pub fn represented_families(&self) -> BTreeSet<M5ExecutionComponentFamily> {
        self.components.iter().map(|r| r.family).collect()
    }

    /// Count of rows that are complete, honest degraded / narrowed components.
    pub fn degraded_row_count(&self) -> usize {
        self.components.iter().filter(|r| r.is_degraded()).count()
    }

    /// Validates the execution-lifecycle component matrix invariants.
    pub fn validate(&self) -> Vec<ExecutionLifecycleComponentViolation> {
        let mut violations = Vec::new();

        if self.record_kind != EXECUTION_LIFECYCLE_COMPONENT_MATRIX_RECORD_KIND {
            violations.push(ExecutionLifecycleComponentViolation::WrongRecordKind);
        }
        if self.schema_version != EXECUTION_LIFECYCLE_COMPONENT_MATRIX_SCHEMA_VERSION {
            violations.push(ExecutionLifecycleComponentViolation::WrongSchemaVersion);
        }
        if self.packet_id.trim().is_empty()
            || self.set_label.trim().is_empty()
            || self.redaction_class_token.trim().is_empty()
            || self.minted_at.trim().is_empty()
        {
            violations.push(ExecutionLifecycleComponentViolation::MissingIdentity);
        }

        validate_source_contracts(self, &mut violations);
        validate_coverage(self, &mut violations);
        validate_rows(self, &mut violations);
        validate_guardrails(self, &mut violations);
        validate_consumer_projection(self, &mut violations);

        if json_contains_forbidden_boundary_material(
            &serde_json::to_value(self).expect("execution-lifecycle component matrix serializes"),
        ) {
            violations.push(ExecutionLifecycleComponentViolation::RawBoundaryMaterialInExport);
        }

        violations
    }

    /// Deterministic export-safe JSON.
    ///
    /// # Panics
    ///
    /// Panics only if serializing this metadata-only packet fails.
    pub fn export_safe_json(&self) -> String {
        serde_json::to_string_pretty(self).expect("execution-lifecycle component matrix serializes")
    }

    /// Deterministic CSV of the component rows for release / support handoff.
    pub fn render_matrix_csv(&self) -> String {
        let mut out = String::from(
            "component_id,family,truth_mode,locality,export_safe,assistive_ready,degraded\n",
        );
        for row in &self.components {
            out.push_str(&format!(
                "{id},{family},{truth},{locality},{export_safe},{assistive},{degraded}\n",
                id = row.component_id,
                family = row.family.as_str(),
                truth = row.truth_mode.as_str(),
                locality = row.locality.as_str(),
                export_safe = row.export_safe,
                assistive = row.assistive_ready,
                degraded = row.degraded.as_ref().map_or("none", |d| d.trigger.as_str()),
            ));
        }
        out
    }

    /// Deterministic Markdown summary for support, docs, or release handoff.
    pub fn render_markdown_summary(&self) -> String {
        let mut out = String::new();
        out.push_str("# M5 Execution-Lifecycle Component Matrix\n\n");
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- Label: `{}`\n", self.set_label));
        out.push_str(&format!(
            "- Components: {} across {} / {} families ({} degraded)\n",
            self.components.len(),
            self.represented_families().len(),
            M5ExecutionComponentFamily::ALL.len(),
            self.degraded_row_count(),
        ));
        out.push_str("\n## Components\n\n");
        for row in &self.components {
            out.push_str(&format!(
                "- **{}** ({}) — {}\n",
                row.component_id,
                row.family.as_str(),
                row.surface_label,
            ));
            out.push_str(&format!("  - {}\n", row.label_summary));
            out.push_str(&format!("  - {}\n", row.chip_tokens()));
            if let Some(degraded) = &row.degraded {
                out.push_str(&format!(
                    "  - Degraded: trigger={} — {}\n",
                    degraded.trigger.as_str(),
                    degraded.degraded_label,
                ));
            }
        }
        out
    }
}

/// Errors emitted when reading the checked-in execution-lifecycle component export.
#[derive(Debug)]
pub enum ExecutionLifecycleComponentArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<ExecutionLifecycleComponentViolation>),
}

impl fmt::Display for ExecutionLifecycleComponentArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(
                    formatter,
                    "execution-lifecycle component export parse failed: {error}"
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
                    "execution-lifecycle component export failed validation: {tokens}"
                )
            }
        }
    }
}

impl Error for ExecutionLifecycleComponentArtifactError {}

/// Validation failures emitted by [`ExecutionLifecycleComponentMatrix::validate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ExecutionLifecycleComponentViolation {
    /// Packet record kind is wrong.
    WrongRecordKind,
    /// Packet schema version is wrong.
    WrongSchemaVersion,
    /// Required identity field is missing.
    MissingIdentity,
    /// Required base source contract refs are incomplete.
    MissingSourceContracts,
    /// A required reusable component family is defined by no row.
    RequiredFamilyMissing,
    /// The matrix demonstrates no complete degraded / narrowed row.
    DegradedCaseMissing,
    /// A row is incomplete.
    RowIncomplete,
    /// A row's family-specific payload is missing, extra, or wrong for its family.
    PayloadFamilyMismatch,
    /// A row's family payload is internally dishonest.
    PayloadDishonest,
    /// A truth-bearing descriptor discloses a class / boundary different from its row.
    DescriptorRowMismatch,
    /// A row omits a mandatory required label.
    MandatoryLabelMissing,
    /// A row is not export-safe or not assistive-ready.
    ParityMissing,
    /// A degraded block carries a generic non-answer label.
    DegradedLabelGeneric,
    /// A row lacks evidence refs.
    RowEvidenceMissing,
    /// Guardrail block does not satisfy required invariants.
    GuardrailsIncomplete,
    /// Consumer projection does not satisfy required invariants.
    ConsumerProjectionIncomplete,
    /// Export contains raw boundary material.
    RawBoundaryMaterialInExport,
}

impl ExecutionLifecycleComponentViolation {
    /// Stable token used in tests and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WrongRecordKind => "wrong_record_kind",
            Self::WrongSchemaVersion => "wrong_schema_version",
            Self::MissingIdentity => "missing_identity",
            Self::MissingSourceContracts => "missing_source_contracts",
            Self::RequiredFamilyMissing => "required_family_missing",
            Self::DegradedCaseMissing => "degraded_case_missing",
            Self::RowIncomplete => "row_incomplete",
            Self::PayloadFamilyMismatch => "payload_family_mismatch",
            Self::PayloadDishonest => "payload_dishonest",
            Self::DescriptorRowMismatch => "descriptor_row_mismatch",
            Self::MandatoryLabelMissing => "mandatory_label_missing",
            Self::ParityMissing => "parity_missing",
            Self::DegradedLabelGeneric => "degraded_label_generic",
            Self::RowEvidenceMissing => "row_evidence_missing",
            Self::GuardrailsIncomplete => "guardrails_incomplete",
            Self::ConsumerProjectionIncomplete => "consumer_projection_incomplete",
            Self::RawBoundaryMaterialInExport => "raw_boundary_material_in_export",
        }
    }
}

/// Reads and validates the checked-in execution-lifecycle component export.
///
/// # Errors
///
/// Returns an artifact error if the export cannot parse or fails validation.
pub fn current_m5_execution_lifecycle_component_matrix_export(
) -> Result<ExecutionLifecycleComponentMatrix, ExecutionLifecycleComponentArtifactError> {
    let packet: ExecutionLifecycleComponentMatrix = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/release/m5-execution-lifecycle-component-proof/support_export.json"
    )))
    .map_err(ExecutionLifecycleComponentArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(ExecutionLifecycleComponentArtifactError::Validation(
            violations,
        ))
    }
}

fn validate_source_contracts(
    packet: &ExecutionLifecycleComponentMatrix,
    violations: &mut Vec<ExecutionLifecycleComponentViolation>,
) {
    let refs: BTreeSet<&str> = packet
        .source_contract_refs
        .iter()
        .map(String::as_str)
        .collect();
    for required in [
        EXECUTION_LIFECYCLE_COMPONENT_MATRIX_SCHEMA_REF,
        EXECUTION_LIFECYCLE_COMPONENT_MATRIX_DOC_REF,
        EXECUTION_LIFECYCLE_COMPONENT_MATRIX_ARTIFACT_REF,
    ] {
        if !refs.contains(required) {
            violations.push(ExecutionLifecycleComponentViolation::MissingSourceContracts);
            break;
        }
    }
}

fn validate_coverage(
    packet: &ExecutionLifecycleComponentMatrix,
    violations: &mut Vec<ExecutionLifecycleComponentViolation>,
) {
    let families = packet.represented_families();
    for required in M5ExecutionComponentFamily::ALL {
        if !families.contains(&required) {
            violations.push(ExecutionLifecycleComponentViolation::RequiredFamilyMissing);
            break;
        }
    }
    if packet.degraded_row_count() == 0 {
        violations.push(ExecutionLifecycleComponentViolation::DegradedCaseMissing);
    }
}

fn validate_rows(
    packet: &ExecutionLifecycleComponentMatrix,
    violations: &mut Vec<ExecutionLifecycleComponentViolation>,
) {
    for row in &packet.components {
        if !row.is_complete() {
            violations.push(ExecutionLifecycleComponentViolation::RowIncomplete);
        }
        if !row.payload_matches_family() {
            violations.push(ExecutionLifecycleComponentViolation::PayloadFamilyMismatch);
        }
        if !row.payload_honest() {
            violations.push(ExecutionLifecycleComponentViolation::PayloadDishonest);
        }
        if !row.descriptor_matches_row() {
            violations.push(ExecutionLifecycleComponentViolation::DescriptorRowMismatch);
        }
        if !row.mandatory_labels_present() {
            violations.push(ExecutionLifecycleComponentViolation::MandatoryLabelMissing);
        }
        if !row.export_safe || !row.assistive_ready {
            violations.push(ExecutionLifecycleComponentViolation::ParityMissing);
        }
        if !row.degraded_ok() {
            violations.push(ExecutionLifecycleComponentViolation::DegradedLabelGeneric);
        }
        if row.evidence_refs.is_empty() || row.evidence_refs.iter().any(|r| r.trim().is_empty()) {
            violations.push(ExecutionLifecycleComponentViolation::RowEvidenceMissing);
        }
    }
}

fn validate_guardrails(
    packet: &ExecutionLifecycleComponentMatrix,
    violations: &mut Vec<ExecutionLifecycleComponentViolation>,
) {
    if !packet.guardrails.all_hold() {
        violations.push(ExecutionLifecycleComponentViolation::GuardrailsIncomplete);
    }
}

fn validate_consumer_projection(
    packet: &ExecutionLifecycleComponentMatrix,
    violations: &mut Vec<ExecutionLifecycleComponentViolation>,
) {
    if !packet.consumer_projection.all_hold() {
        violations.push(ExecutionLifecycleComponentViolation::ConsumerProjectionIncomplete);
    }
}

/// Whether a degraded label is a generic non-answer rather than a precise label.
fn label_is_generic(label: &str) -> bool {
    let trimmed = label.trim();
    if trimmed.is_empty() {
        return true;
    }
    let lower = trimmed.to_lowercase();
    matches!(
        lower.as_str(),
        "unsupported"
            | "not supported"
            | "unavailable"
            | "not available"
            | "n/a"
            | "error"
            | "failed"
            | "stale"
            | "no data"
            | "blocked"
            | "degraded"
            | "captured"
            | "cancelled"
    )
}

/// Heuristic that rejects obviously forbidden material in export-safe JSON.
fn json_contains_forbidden_boundary_material(value: &serde_json::Value) -> bool {
    match value {
        serde_json::Value::String(s) => {
            let lower = s.to_lowercase();
            lower.contains("api_key")
                || lower.contains("password")
                || lower.contains("secret")
                || lower.contains("-----begin")
                || lower.contains("bearer ")
        }
        serde_json::Value::Array(arr) => arr.iter().any(json_contains_forbidden_boundary_material),
        serde_json::Value::Object(map) => {
            map.values().any(json_contains_forbidden_boundary_material)
        }
        _ => false,
    }
}

/// Builds the canonical, checked-in execution-lifecycle component matrix packet. This
/// is the one source of truth shared by the tests and the on-disk support export so
/// both stay byte-aligned.
pub fn seeded_execution_lifecycle_component_matrix() -> ExecutionLifecycleComponentMatrix {
    ExecutionLifecycleComponentMatrix::new(ExecutionLifecycleComponentMatrixInput {
        packet_id: "m5-execution-lifecycle-component-matrix:stable:0001".to_owned(),
        set_label: "M5 Execution-Lifecycle Component Matrix".to_owned(),
        components: seeded_components(),
        guardrails: seeded_guardrails(),
        consumer_projection: seeded_consumer_projection(),
        source_contract_refs: seeded_source_contract_refs(),
        redaction_class_token: "metadata_safe_default".to_owned(),
        minted_at: "2026-07-04T00:00:00Z".to_owned(),
    })
}

fn ev(id: &str) -> Vec<String> {
    vec![format!("evidence:execution-lifecycle:{id}")]
}

fn mandatory_labels() -> Vec<M5ExecutionRequiredLabel> {
    vec![
        M5ExecutionRequiredLabel::Identity,
        M5ExecutionRequiredLabel::ExecutionContext,
        M5ExecutionRequiredLabel::TruthClass,
        M5ExecutionRequiredLabel::OutcomeOrState,
        M5ExecutionRequiredLabel::ExecutionBoundary,
        M5ExecutionRequiredLabel::KeyboardRoute,
    ]
}

fn seeded_components() -> Vec<ComponentRow> {
    vec![
        // Run/attempt header — a live running attempt, run and attempt distinct.
        ComponentRow {
            component_id: "component:run-attempt-header:0001".to_owned(),
            family: M5ExecutionComponentFamily::RunAttemptHeader,
            surface_label: "Run/attempt header for a live task run".to_owned(),
            truth_mode: M5ExecutionTruthMode::Live,
            locality: M5ExecutionLocality::Local,
            execution_context_ref: "execution_context:run:0001".to_owned(),
            required_labels: mandatory_labels(),
            export_safe: true,
            assistive_ready: true,
            run_attempt_header: Some(RunAttemptHeaderDescriptor {
                run_identity_ref: "run:build-and-test:0001".to_owned(),
                attempt_identity_ref: "attempt:build-and-test:0001#2".to_owned(),
                attempt_ordinal: 2,
                outcome: M5RunOutcome::Running,
                truth_mode: M5ExecutionTruthMode::Live,
                run_and_attempt_distinct: true,
            }),
            input_request_prompt: None,
            artifact_publish_row: None,
            rerun_comparison_sheet: None,
            debug_session_header: None,
            thread_process_tree: None,
            dump_crash_artifact_card: None,
            degraded: None,
            label_summary: "A run/attempt header keeps run and attempt identity distinct and shows an actively running attempt as live truth".to_owned(),
            observed_at: "2026-07-04T00:00:00Z".to_owned(),
            evidence_refs: ev("run-attempt-header:0001"),
        },
        // Run/attempt header — a stale output from a superseded run, narrows.
        ComponentRow {
            component_id: "component:run-attempt-header:0002".to_owned(),
            family: M5ExecutionComponentFamily::RunAttemptHeader,
            surface_label: "Run/attempt header for a superseded run with stale output".to_owned(),
            truth_mode: M5ExecutionTruthMode::Captured,
            locality: M5ExecutionLocality::Remote,
            execution_context_ref: "execution_context:run:0002".to_owned(),
            required_labels: mandatory_labels(),
            export_safe: true,
            assistive_ready: true,
            run_attempt_header: Some(RunAttemptHeaderDescriptor {
                run_identity_ref: "run:integration:0007".to_owned(),
                attempt_identity_ref: "attempt:integration:0007#1".to_owned(),
                attempt_ordinal: 1,
                outcome: M5RunOutcome::StaleOutput,
                truth_mode: M5ExecutionTruthMode::Captured,
                run_and_attempt_distinct: true,
            }),
            input_request_prompt: None,
            artifact_publish_row: None,
            rerun_comparison_sheet: None,
            debug_session_header: None,
            thread_process_tree: None,
            dump_crash_artifact_card: None,
            degraded: Some(DegradedState {
                trigger: M5ExecutionDowngradeTrigger::CapturedEvidenceOnly,
                degraded_label: "The source changed since this run; its output is marked stale and shown as captured evidence rather than a live result".to_owned(),
            }),
            label_summary: "A run/attempt header discloses stale output as captured evidence rather than let it read as a current live result".to_owned(),
            observed_at: "2026-07-04T00:00:00Z".to_owned(),
            evidence_refs: ev("run-attempt-header:0002"),
        },
        // Input-request prompt — waiting on approval with a timeout that cancels.
        ComponentRow {
            component_id: "component:input-request-prompt:0001".to_owned(),
            family: M5ExecutionComponentFamily::InputRequestPrompt,
            surface_label: "Input-request prompt awaiting an approval with a cancel-on-timeout".to_owned(),
            truth_mode: M5ExecutionTruthMode::Live,
            locality: M5ExecutionLocality::Local,
            execution_context_ref: "execution_context:run:0001".to_owned(),
            required_labels: mandatory_labels(),
            export_safe: true,
            assistive_ready: true,
            run_attempt_header: None,
            input_request_prompt: Some(InputRequestPromptDescriptor {
                prompt_ref: "input_request:approve-deploy:0001".to_owned(),
                consequence: M5InputConsequence::TimeoutCancelsRun,
                discloses_timeout: true,
                discloses_approval: true,
                has_deadline: true,
            }),
            artifact_publish_row: None,
            rerun_comparison_sheet: None,
            debug_session_header: None,
            thread_process_tree: None,
            dump_crash_artifact_card: None,
            degraded: None,
            label_summary: "An input-request prompt discloses that it needs approval and that a timeout will cancel the run, with a visible deadline".to_owned(),
            observed_at: "2026-07-04T00:00:00Z".to_owned(),
            evidence_refs: ev("input-request-prompt:0001"),
        },
        // Artifact-publish row — durable retention with producing-run lineage.
        ComponentRow {
            component_id: "component:artifact-publish-row:0001".to_owned(),
            family: M5ExecutionComponentFamily::ArtifactPublishRow,
            surface_label: "Artifact-publish row for a durably retained build artifact".to_owned(),
            truth_mode: M5ExecutionTruthMode::Captured,
            locality: M5ExecutionLocality::Container,
            execution_context_ref: "execution_context:run:0001".to_owned(),
            required_labels: mandatory_labels(),
            export_safe: true,
            assistive_ready: true,
            run_attempt_header: None,
            input_request_prompt: None,
            artifact_publish_row: Some(ArtifactPublishRowDescriptor {
                artifact_ref: "artifact:dist/app.tar.gz:0001".to_owned(),
                producing_run_ref: "run:build-and-test:0001".to_owned(),
                retention: M5RetentionClass::RetainedDurable,
                lineage_preserved: true,
                retention_visible: true,
            }),
            rerun_comparison_sheet: None,
            debug_session_header: None,
            thread_process_tree: None,
            dump_crash_artifact_card: None,
            degraded: None,
            label_summary: "An artifact-publish row names the run that produced the artifact and discloses durable retention".to_owned(),
            observed_at: "2026-07-04T00:00:00Z".to_owned(),
            evidence_refs: ev("artifact-publish-row:0001"),
        },
        // Artifact-publish row — evicted-but-recoverable artifact, narrows.
        ComponentRow {
            component_id: "component:artifact-publish-row:0002".to_owned(),
            family: M5ExecutionComponentFamily::ArtifactPublishRow,
            surface_label: "Artifact-publish row for an evicted-but-recoverable artifact".to_owned(),
            truth_mode: M5ExecutionTruthMode::Captured,
            locality: M5ExecutionLocality::Local,
            execution_context_ref: "execution_context:run:0003".to_owned(),
            required_labels: mandatory_labels(),
            export_safe: true,
            assistive_ready: true,
            run_attempt_header: None,
            input_request_prompt: None,
            artifact_publish_row: Some(ArtifactPublishRowDescriptor {
                artifact_ref: "artifact:coverage/report.lcov:0003".to_owned(),
                producing_run_ref: "run:coverage:0003".to_owned(),
                retention: M5RetentionClass::EvictedRecoverable,
                lineage_preserved: true,
                retention_visible: true,
            }),
            rerun_comparison_sheet: None,
            debug_session_header: None,
            thread_process_tree: None,
            dump_crash_artifact_card: None,
            degraded: Some(DegradedState {
                trigger: M5ExecutionDowngradeTrigger::ArtifactRetentionExpired,
                degraded_label: "The artifact was evicted from cache; its producing run is still known and the artifact is rebuildable from lineage".to_owned(),
            }),
            label_summary: "An artifact-publish row discloses that the artifact was evicted but keeps producing-run lineage so it can be rebuilt".to_owned(),
            observed_at: "2026-07-04T00:00:00Z".to_owned(),
            evidence_refs: ev("artifact-publish-row:0002"),
        },
        // Rerun comparison sheet — an exact replay, no context delta.
        ComponentRow {
            component_id: "component:rerun-comparison-sheet:0001".to_owned(),
            family: M5ExecutionComponentFamily::RerunComparisonSheet,
            surface_label: "Rerun comparison sheet for an exact replay".to_owned(),
            truth_mode: M5ExecutionTruthMode::Planned,
            locality: M5ExecutionLocality::Local,
            execution_context_ref: "execution_context:run:0001".to_owned(),
            required_labels: mandatory_labels(),
            export_safe: true,
            assistive_ready: true,
            run_attempt_header: None,
            input_request_prompt: None,
            artifact_publish_row: None,
            rerun_comparison_sheet: Some(RerunComparisonSheetDescriptor {
                baseline_run_ref: "run:build-and-test:0001".to_owned(),
                rerun_context: M5RerunContext::ExactReplay,
                discloses_context_delta: true,
                context_diff_shown_before_dispatch: true,
            }),
            debug_session_header: None,
            thread_process_tree: None,
            dump_crash_artifact_card: None,
            degraded: None,
            label_summary: "A rerun comparison sheet confirms an exact replay of the baseline run's selection, environment, and inputs".to_owned(),
            observed_at: "2026-07-04T00:00:00Z".to_owned(),
            evidence_refs: ev("rerun-comparison-sheet:0001"),
        },
        // Rerun comparison sheet — current-context rerun, context delta before dispatch.
        ComponentRow {
            component_id: "component:rerun-comparison-sheet:0002".to_owned(),
            family: M5ExecutionComponentFamily::RerunComparisonSheet,
            surface_label: "Rerun comparison sheet for a current-context rerun".to_owned(),
            truth_mode: M5ExecutionTruthMode::Planned,
            locality: M5ExecutionLocality::Remote,
            execution_context_ref: "execution_context:run:0004".to_owned(),
            required_labels: mandatory_labels(),
            export_safe: true,
            assistive_ready: true,
            run_attempt_header: None,
            input_request_prompt: None,
            artifact_publish_row: None,
            rerun_comparison_sheet: Some(RerunComparisonSheetDescriptor {
                baseline_run_ref: "run:integration:0004".to_owned(),
                rerun_context: M5RerunContext::CurrentContext,
                discloses_context_delta: true,
                context_diff_shown_before_dispatch: true,
            }),
            debug_session_header: None,
            thread_process_tree: None,
            dump_crash_artifact_card: None,
            degraded: Some(DegradedState {
                trigger: M5ExecutionDowngradeTrigger::RerunContextDrift,
                degraded_label: "This rerun uses the current context, which differs from the baseline; the changed selection and environment are shown before dispatch".to_owned(),
            }),
            label_summary: "A rerun comparison sheet shows the exact-versus-current-context difference before dispatch rather than after".to_owned(),
            observed_at: "2026-07-04T00:00:00Z".to_owned(),
            evidence_refs: ev("rerun-comparison-sheet:0002"),
        },
        // Debug session header — a live attach session, local.
        ComponentRow {
            component_id: "component:debug-session-header:0001".to_owned(),
            family: M5ExecutionComponentFamily::DebugSessionHeader,
            surface_label: "Debug session header for a live attach session".to_owned(),
            truth_mode: M5ExecutionTruthMode::Live,
            locality: M5ExecutionLocality::Local,
            execution_context_ref: "execution_context:debug:0001".to_owned(),
            required_labels: mandatory_labels(),
            export_safe: true,
            assistive_ready: true,
            run_attempt_header: None,
            input_request_prompt: None,
            artifact_publish_row: None,
            rerun_comparison_sheet: None,
            debug_session_header: Some(DebugSessionHeaderDescriptor {
                session_ref: "debug_session:attach:0001".to_owned(),
                session_mode: M5DebugSessionMode::Attach,
                truth_mode: M5ExecutionTruthMode::Live,
                locality: M5ExecutionLocality::Local,
                boundary_explicit: true,
            }),
            thread_process_tree: None,
            dump_crash_artifact_card: None,
            degraded: None,
            label_summary: "A debug session header names an attach session with live control on the local machine and keeps the boundary explicit".to_owned(),
            observed_at: "2026-07-04T00:00:00Z".to_owned(),
            evidence_refs: ev("debug-session-header:0001"),
        },
        // Debug session header — a captured replay session, managed, narrows.
        ComponentRow {
            component_id: "component:debug-session-header:0002".to_owned(),
            family: M5ExecutionComponentFamily::DebugSessionHeader,
            surface_label: "Debug session header for a captured replay session".to_owned(),
            truth_mode: M5ExecutionTruthMode::Captured,
            locality: M5ExecutionLocality::Managed,
            execution_context_ref: "execution_context:debug:0002".to_owned(),
            required_labels: mandatory_labels(),
            export_safe: true,
            assistive_ready: true,
            run_attempt_header: None,
            input_request_prompt: None,
            artifact_publish_row: None,
            rerun_comparison_sheet: None,
            debug_session_header: Some(DebugSessionHeaderDescriptor {
                session_ref: "debug_session:replay:0002".to_owned(),
                session_mode: M5DebugSessionMode::Replay,
                truth_mode: M5ExecutionTruthMode::Captured,
                locality: M5ExecutionLocality::Managed,
                boundary_explicit: true,
            }),
            thread_process_tree: None,
            dump_crash_artifact_card: None,
            degraded: Some(DegradedState {
                trigger: M5ExecutionDowngradeTrigger::CapturedEvidenceOnly,
                degraded_label: "This is a recorded replay in a managed environment; stepping navigates captured evidence and never controls a live process".to_owned(),
            }),
            label_summary: "A debug session header discloses a replay session as captured evidence, never as live control".to_owned(),
            observed_at: "2026-07-04T00:00:00Z".to_owned(),
            evidence_refs: ev("debug-session-header:0002"),
        },
        // Thread/process tree — a live container hierarchy.
        ComponentRow {
            component_id: "component:thread-process-tree:0001".to_owned(),
            family: M5ExecutionComponentFamily::ThreadProcessTree,
            surface_label: "Thread/process tree for a live containerized run".to_owned(),
            truth_mode: M5ExecutionTruthMode::Live,
            locality: M5ExecutionLocality::Container,
            execution_context_ref: "execution_context:debug:0003".to_owned(),
            required_labels: mandatory_labels(),
            export_safe: true,
            assistive_ready: true,
            run_attempt_header: None,
            input_request_prompt: None,
            artifact_publish_row: None,
            rerun_comparison_sheet: None,
            debug_session_header: None,
            thread_process_tree: Some(ThreadProcessTreeDescriptor {
                root_ref: "process:pid-1:0001".to_owned(),
                node_count: 12,
                truth_mode: M5ExecutionTruthMode::Live,
                locality: M5ExecutionLocality::Container,
                live_vs_captured_explicit: true,
            }),
            dump_crash_artifact_card: None,
            degraded: None,
            label_summary: "A thread/process tree marks itself as a live container hierarchy and names its execution boundary explicitly".to_owned(),
            observed_at: "2026-07-04T00:00:00Z".to_owned(),
            evidence_refs: ev("thread-process-tree:0001"),
        },
        // Dump/crash artifact card — a symbolicated crash dump, durable.
        ComponentRow {
            component_id: "component:dump-crash-artifact-card:0001".to_owned(),
            family: M5ExecutionComponentFamily::DumpCrashArtifactCard,
            surface_label: "Dump/crash artifact card for a symbolicated crash dump".to_owned(),
            truth_mode: M5ExecutionTruthMode::Captured,
            locality: M5ExecutionLocality::Remote,
            execution_context_ref: "execution_context:run:0005".to_owned(),
            required_labels: mandatory_labels(),
            export_safe: true,
            assistive_ready: true,
            run_attempt_header: None,
            input_request_prompt: None,
            artifact_publish_row: None,
            rerun_comparison_sheet: None,
            debug_session_header: None,
            thread_process_tree: None,
            dump_crash_artifact_card: Some(DumpCrashArtifactCardDescriptor {
                dump_ref: "dump:crash/core.0005".to_owned(),
                producing_run_ref: "run:integration:0005".to_owned(),
                symbolication: M5SymbolicationState::Symbolicated,
                retention: M5RetentionClass::RetainedDurable,
                captured_truth: true,
            }),
            degraded: None,
            label_summary: "A dump/crash artifact card names the run that produced the dump, shows it fully symbolicated, and marks it captured evidence".to_owned(),
            observed_at: "2026-07-04T00:00:00Z".to_owned(),
            evidence_refs: ev("dump-crash-artifact-card:0001"),
        },
        // Dump/crash artifact card — an unsymbolicatable dump, narrows.
        ComponentRow {
            component_id: "component:dump-crash-artifact-card:0002".to_owned(),
            family: M5ExecutionComponentFamily::DumpCrashArtifactCard,
            surface_label: "Dump/crash artifact card for a dump with unavailable symbols".to_owned(),
            truth_mode: M5ExecutionTruthMode::Captured,
            locality: M5ExecutionLocality::Container,
            execution_context_ref: "execution_context:run:0006".to_owned(),
            required_labels: mandatory_labels(),
            export_safe: true,
            assistive_ready: true,
            run_attempt_header: None,
            input_request_prompt: None,
            artifact_publish_row: None,
            rerun_comparison_sheet: None,
            debug_session_header: None,
            thread_process_tree: None,
            dump_crash_artifact_card: Some(DumpCrashArtifactCardDescriptor {
                dump_ref: "dump:crash/core.0006".to_owned(),
                producing_run_ref: "run:integration:0006".to_owned(),
                symbolication: M5SymbolicationState::SymbolsUnavailable,
                retention: M5RetentionClass::ExpiresScheduled,
                captured_truth: true,
            }),
            degraded: Some(DegradedState {
                trigger: M5ExecutionDowngradeTrigger::SymbolsUnavailable,
                degraded_label: "No symbols resolved for this dump; frames are shown as raw addresses and the card offers a symbol-upload route".to_owned(),
            }),
            label_summary: "A dump/crash artifact card discloses that symbols are unavailable rather than present raw addresses as a resolved stack".to_owned(),
            observed_at: "2026-07-04T00:00:00Z".to_owned(),
            evidence_refs: ev("dump-crash-artifact-card:0002"),
        },
    ]
}

fn seeded_guardrails() -> ExecutionLifecycleGuardrails {
    ExecutionLifecycleGuardrails {
        run_and_attempt_identity_distinct: true,
        outcomes_stable_across_ui_cli_export: true,
        artifacts_never_lose_lineage_or_retention: true,
        rerun_discloses_context_delta_before_dispatch: true,
        debug_keeps_mode_truth_and_boundary_explicit: true,
        exported_evidence_preserves_ids_states_and_lineage: true,
        components_bound_to_shared_vocabulary: true,
        no_new_runtimes_adapters_or_backends: true,
    }
}

fn seeded_consumer_projection() -> ExecutionLifecycleConsumerProjection {
    ExecutionLifecycleConsumerProjection {
        product_ingests_components: true,
        docs_help_ingests_components: true,
        diagnostics_ingests_components: true,
        support_export_ingests_components: true,
        release_control_ingests_components: true,
        later_rows_reference_one_canonical_family: true,
    }
}

fn seeded_source_contract_refs() -> Vec<String> {
    vec![
        EXECUTION_LIFECYCLE_COMPONENT_MATRIX_SCHEMA_REF.to_owned(),
        EXECUTION_LIFECYCLE_COMPONENT_MATRIX_DOC_REF.to_owned(),
        EXECUTION_LIFECYCLE_COMPONENT_MATRIX_ARTIFACT_REF.to_owned(),
        "schemas/task/m5-task-event-envelope.schema.json".to_owned(),
        "schemas/debug/m5-debug-session-descriptor.schema.json".to_owned(),
    ]
}
