//! Implements the reusable debug-session-header, thread/process-tree, and
//! dump/crash-artifact-card primitive: a debug session header, a set of thread / process
//! tree rows, a set of dump / crash artifact cards, a CLI / headless line, and a
//! support-export projection that all resolve from one bounded debug session and share
//! one session identity and one target identity, so a stopped debugger stays explicit
//! about whether the user is holding live attached control (`Launch` / `Attach`) or is
//! reading captured crash evidence (`Core` / `Replay` / `Inspect-only`), which threads
//! and processes make up the hierarchy, and how well each dump is symbolicated and
//! provenance-tracked before the user trusts stopped-state data.
//!
//! Where
//! [`crate::freeze_the_m5_run_attempt_input_request_artifact_publish_rerun_review_and_debug_hierarchy_component_matrix`]
//! *freezes* the reusable execution-lifecycle component families as a governed contract,
//! this module *narrows* the three debug families of that matrix —
//! [`M5ExecutionComponentFamily::DebugSessionHeader`], [`M5ExecutionComponentFamily::ThreadProcessTree`],
//! and [`M5ExecutionComponentFamily::DumpCrashArtifactCard`] — into one working primitive
//! with a real **resolver**. A single debug session projects onto surfaces that share one
//! session identity and one target identity, so live-versus-captured control truth,
//! thread / process hierarchy, and dump symbolication / provenance never blur across the
//! header, the tree rows, the dump cards, the CLI / headless line, and the support-export
//! projection.
//!
//! [`M5ExecutionComponentFamily::DebugSessionHeader`]: crate::freeze_the_m5_run_attempt_input_request_artifact_publish_rerun_review_and_debug_hierarchy_component_matrix::M5ExecutionComponentFamily::DebugSessionHeader
//! [`M5ExecutionComponentFamily::ThreadProcessTree`]: crate::freeze_the_m5_run_attempt_input_request_artifact_publish_rerun_review_and_debug_hierarchy_component_matrix::M5ExecutionComponentFamily::ThreadProcessTree
//! [`M5ExecutionComponentFamily::DumpCrashArtifactCard`]: crate::freeze_the_m5_run_attempt_input_request_artifact_publish_rerun_review_and_debug_hierarchy_component_matrix::M5ExecutionComponentFamily::DumpCrashArtifactCard
//!
//! The three acceptance criteria the resolver proves:
//!
//! - **AC1 — the debug hierarchy stays understandable in-product and in exported evidence
//!   even when the session is restored, degraded, or inspect-only.** The thread / process
//!   tree keeps its parent linkage and depth rather than flattening, and every projection
//!   carries the identity, control posture, and hierarchy so a restored or narrowed
//!   session reconstructs the same story.
//! - **AC2 — users can distinguish live attached control from captured crash analysis at
//!   a glance.** The control posture is derived purely from the session mode: only a
//!   `Launch` / `Attach` session reads as live attached control (and only against live
//!   truth), while `Core` / `Replay` / `Inspect-only` read as captured analysis, and a
//!   dump card never offers a live-control action.
//! - **AC3 — thread / process tree rows and dump cards preserve mapping-quality and
//!   provenance truth rather than collapsing into generic debug chrome.** Every dump card
//!   names its dump ref, its producing-run lineage, its symbolication state, and its build
//!   / symbol provenance, and every tree row preserves node identity, run state, and the
//!   selected thread.
//!
//! Raw process memory, register bytes, dump payloads, symbol blobs, credentials, and
//! provider cursors never cross this boundary; the resolver carries only opaque refs,
//! typed class tokens, booleans, and redacted labels, so support and diagnostics exports
//! reconstruct exactly what a surface would have shown without leaking source or live
//! payloads.
//!
//! The boundary schema is
//! [`schemas/ui/m5-debug-session-hierarchy.schema.json`](../../../../schemas/ui/m5-debug-session-hierarchy.schema.json).
//! The contract doc is
//! [`docs/run-test-debug/m5_debug_session_hierarchy_primitive.md`](../../../../docs/run-test-debug/m5_debug_session_hierarchy_primitive.md).

#[cfg(test)]
mod tests;

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

use crate::freeze_the_m5_run_attempt_input_request_artifact_publish_rerun_review_and_debug_hierarchy_component_matrix::{
    DegradedState, M5DebugSessionMode, M5ExecutionDowngradeTrigger, M5ExecutionLocality,
    M5ExecutionTruthMode, M5RetentionClass, M5RunOutcome, M5SymbolicationState,
};
use crate::implement_the_m5_run_attempt_header_and_attempt_selector_primitive::M5RunAttemptSurfaceFamily;

/// Stable record-kind tag carried by [`M5DebugHierarchyPrimitivePacket`].
pub const M5_DEBUG_HIERARCHY_RECORD_KIND: &str = "m5_debug_session_hierarchy_primitive";

/// Schema version for the debug-session-hierarchy primitive packet.
pub const M5_DEBUG_HIERARCHY_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the boundary schema.
pub const M5_DEBUG_HIERARCHY_SCHEMA_REF: &str = "schemas/ui/m5-debug-session-hierarchy.schema.json";

/// Repo-relative path of the contract doc.
pub const M5_DEBUG_HIERARCHY_DOC_REF: &str =
    "docs/run-test-debug/m5_debug_session_hierarchy_primitive.md";

/// Repo-relative path of the frozen component-matrix contract this primitive narrows.
pub const M5_DEBUG_HIERARCHY_COMPONENT_MATRIX_REF: &str =
    "schemas/ui/m5-execution-lifecycle-component-matrix.schema.json";

/// Repo-relative path of the protected fixture directory.
pub const M5_DEBUG_HIERARCHY_FIXTURE_DIR: &str = "fixtures/ui/m5-debug-session-hierarchy-primitive";

/// Repo-relative path of the checked support-export artifact (the `include_str!`
/// canonical).
pub const M5_DEBUG_HIERARCHY_ARTIFACT_REF: &str =
    "artifacts/release/m5-debug-session-hierarchy-primitive-proof/support_export.json";

/// Repo-relative path of the checked matrix CSV.
pub const M5_DEBUG_HIERARCHY_CSV_REF: &str =
    "artifacts/release/m5-debug-session-hierarchy-primitive-proof/matrix.csv";

/// Repo-relative path of the checked Markdown report.
pub const M5_DEBUG_HIERARCHY_REPORT_REF: &str =
    "artifacts/release/m5-debug-session-hierarchy-primitive-proof/report.md";

// --- minted controlled vocabulary ---

/// Closed debug-control-posture vocabulary. Names whether the debug session is holding
/// live attached control or is reading captured crash evidence so a stopped debugger
/// never blurs live control with a post-mortem analysis.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5DebugControlPosture {
    /// A live, attached session with control over a running target (`Launch` / `Attach`).
    LiveAttachedControl,
    /// A captured, post-mortem analysis of recorded evidence (`Core` / `Replay`).
    CapturedAnalysis,
    /// An inspect-only view with no control of any target.
    InspectOnlyView,
}

impl M5DebugControlPosture {
    /// Every control posture, in declaration order.
    pub const ALL: [Self; 3] = [
        Self::LiveAttachedControl,
        Self::CapturedAnalysis,
        Self::InspectOnlyView,
    ];

    /// The control posture a debug session mode establishes. Derived purely from the mode
    /// so the same mode always reads as the same posture (AC2).
    pub const fn for_mode(mode: M5DebugSessionMode) -> Self {
        match mode {
            M5DebugSessionMode::Launch | M5DebugSessionMode::Attach => Self::LiveAttachedControl,
            M5DebugSessionMode::Core | M5DebugSessionMode::Replay => Self::CapturedAnalysis,
            M5DebugSessionMode::InspectOnly => Self::InspectOnlyView,
        }
    }

    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LiveAttachedControl => "live_attached_control",
            Self::CapturedAnalysis => "captured_analysis",
            Self::InspectOnlyView => "inspect_only_view",
        }
    }

    /// Human-readable label for the header and report.
    pub const fn label(self) -> &'static str {
        match self {
            Self::LiveAttachedControl => "Live attached control",
            Self::CapturedAnalysis => "Captured analysis",
            Self::InspectOnlyView => "Inspect-only view",
        }
    }

    /// True when the posture grants live control of a running target.
    pub const fn is_live_control(self) -> bool {
        matches!(self, Self::LiveAttachedControl)
    }
}

/// Closed debug-adapter-state vocabulary. Names the connection state of the debug adapter
/// so a session whose adapter is gone never reads as if it were still under live control.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5DebugAdapterState {
    /// The adapter is connected and responsive.
    Connected,
    /// The adapter reconnected after a drop and control is restored.
    Restored,
    /// The adapter connection was lost.
    Disconnected,
    /// No adapter is available (captured evidence only).
    Unavailable,
}

impl M5DebugAdapterState {
    /// Every adapter state, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::Connected,
        Self::Restored,
        Self::Disconnected,
        Self::Unavailable,
    ];

    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Connected => "connected",
            Self::Restored => "restored",
            Self::Disconnected => "disconnected",
            Self::Unavailable => "unavailable",
        }
    }

    /// True when the adapter can carry live control of a running target.
    pub const fn is_live_capable(self) -> bool {
        matches!(self, Self::Connected | Self::Restored)
    }
}

/// Closed debug-stop-reason vocabulary. Names why the debugger is at its current stop so
/// a running session, a breakpoint stop, and a captured crash never read the same.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5DebugStopReason {
    /// The target is actively running (not stopped).
    Running,
    /// Stopped at a breakpoint.
    Breakpoint,
    /// Stopped on an unhandled or caught exception.
    Exception,
    /// Stopped on a signal.
    Signal,
    /// Stopped after completing a step.
    StepComplete,
    /// Paused by the user.
    PausedByUser,
    /// Stopped at the program entry point.
    EntryPoint,
    /// A captured crash / dump was recorded; there is no live target.
    CrashCapture,
}

impl M5DebugStopReason {
    /// Every stop reason, in declaration order.
    pub const ALL: [Self; 8] = [
        Self::Running,
        Self::Breakpoint,
        Self::Exception,
        Self::Signal,
        Self::StepComplete,
        Self::PausedByUser,
        Self::EntryPoint,
        Self::CrashCapture,
    ];

    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Running => "running",
            Self::Breakpoint => "breakpoint",
            Self::Exception => "exception",
            Self::Signal => "signal",
            Self::StepComplete => "step_complete",
            Self::PausedByUser => "paused_by_user",
            Self::EntryPoint => "entry_point",
            Self::CrashCapture => "crash_capture",
        }
    }

    /// True when the reason denotes an actively running (not stopped) target.
    pub const fn is_running(self) -> bool {
        matches!(self, Self::Running)
    }

    /// True when the reason denotes a captured crash rather than a live stop.
    pub const fn is_crash_capture(self) -> bool {
        matches!(self, Self::CrashCapture)
    }
}

/// Closed debug-node-kind vocabulary. Names whether a tree node is a process or a thread
/// so the process / thread hierarchy is never flattened into one generic list.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5DebugNodeKind {
    /// A process / target node.
    Process,
    /// A thread node beneath a process.
    Thread,
}

impl M5DebugNodeKind {
    /// Every node kind, in declaration order.
    pub const ALL: [Self; 2] = [Self::Process, Self::Thread];

    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Process => "process",
            Self::Thread => "thread",
        }
    }

    /// True when the node is a thread.
    pub const fn is_thread(self) -> bool {
        matches!(self, Self::Thread)
    }
}

/// Closed thread-run-state vocabulary. Names whether a thread is running, paused, or gone
/// so a paused thread never reads as running (or vice versa).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ThreadRunState {
    /// The thread is running.
    Running,
    /// The thread is paused / stopped.
    Paused,
    /// The thread is single-stepping.
    Stepping,
    /// The thread is waiting / blocked.
    Waiting,
    /// The thread has exited.
    Exited,
    /// The thread's run state is unknown (captured evidence without it).
    Unknown,
}

impl M5ThreadRunState {
    /// Every run state, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::Running,
        Self::Paused,
        Self::Stepping,
        Self::Waiting,
        Self::Exited,
        Self::Unknown,
    ];

    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Running => "running",
            Self::Paused => "paused",
            Self::Stepping => "stepping",
            Self::Waiting => "waiting",
            Self::Exited => "exited",
            Self::Unknown => "unknown",
        }
    }

    /// True when the thread is paused / stopped.
    pub const fn is_paused(self) -> bool {
        matches!(self, Self::Paused)
    }
}

/// Closed debug-action vocabulary. Names the actions a tree row or dump card can offer so
/// a captured surface never exposes a live-control action, and a dump card never implies
/// live control.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5DebugActionKind {
    /// Select / switch to a thread to inspect it (safe; read-only).
    SwitchThread,
    /// Continue the running target (requires live control).
    ContinueExecution,
    /// Pause the running target (requires live control).
    PauseExecution,
    /// Detach from the live session (requires live control).
    DetachSession,
    /// Open the raw dump / evidence (safe; read-only).
    OpenRawDump,
    /// Export the evidence for support (safe; read-only).
    ExportEvidence,
    /// Copy an opaque reference (safe; read-only).
    CopyReference,
    /// Open the mapped source location (safe; read-only).
    OpenInEditor,
}

impl M5DebugActionKind {
    /// Every action kind, in declaration order.
    pub const ALL: [Self; 8] = [
        Self::SwitchThread,
        Self::ContinueExecution,
        Self::PauseExecution,
        Self::DetachSession,
        Self::OpenRawDump,
        Self::ExportEvidence,
        Self::CopyReference,
        Self::OpenInEditor,
    ];

    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SwitchThread => "switch_thread",
            Self::ContinueExecution => "continue_execution",
            Self::PauseExecution => "pause_execution",
            Self::DetachSession => "detach_session",
            Self::OpenRawDump => "open_raw_dump",
            Self::ExportEvidence => "export_evidence",
            Self::CopyReference => "copy_reference",
            Self::OpenInEditor => "open_in_editor",
        }
    }

    /// True when the action mutates a live target and so requires live attached control.
    pub const fn implies_live_control(self) -> bool {
        matches!(
            self,
            Self::ContinueExecution | Self::PauseExecution | Self::DetachSession
        )
    }
}

/// Closed dump-artifact-kind vocabulary. Names what kind of captured artifact a dump card
/// represents so a minidump never reads as a full core, and a heap snapshot never reads
/// as a crash report.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5DumpArtifactKind {
    /// A minidump (partial process state).
    Minidump,
    /// A full core dump.
    FullCore,
    /// A structured crash report.
    CrashReport,
    /// A hang / deadlock report.
    HangReport,
    /// A heap snapshot.
    HeapSnapshot,
}

impl M5DumpArtifactKind {
    /// Every dump artifact kind, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::Minidump,
        Self::FullCore,
        Self::CrashReport,
        Self::HangReport,
        Self::HeapSnapshot,
    ];

    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Minidump => "minidump",
            Self::FullCore => "full_core",
            Self::CrashReport => "crash_report",
            Self::HangReport => "hang_report",
            Self::HeapSnapshot => "heap_snapshot",
        }
    }
}

/// Closed export-field vocabulary. Names the fields the support / export packet must carry
/// per surface; the mandatory subset must appear on every row so a support replay
/// reconstructs the control posture and the debug hierarchy (AC1 / AC2 / AC3).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5DebugExportField {
    /// The stable session identity.
    SessionId,
    /// The opaque session ref.
    SessionRef,
    /// The opaque target / process ref, distinct from the session ref.
    TargetRef,
    /// The debug session mode.
    SessionMode,
    /// The derived control posture (live / captured / inspect-only).
    ControlPosture,
    /// The captured-versus-live truth class.
    TruthClass,
    /// The local / remote / container / managed boundary.
    Locality,
    /// The debug adapter state.
    AdapterState,
    /// The current stop reason.
    StopReason,
    /// The number of nodes in the thread / process tree.
    ThreadNodeCount,
    /// The selected thread ref, when present.
    SelectedThread,
    /// The dump / crash artifact refs.
    DumpRefs,
    /// The dump symbolication state.
    SymbolicationState,
    /// The dump build / symbol provenance.
    BuildProvenance,
    /// The dump's producing-run lineage.
    ProducingRunRef,
    /// Whether the session was restored.
    SessionRestored,
}

impl M5DebugExportField {
    /// Every export field, in declaration order.
    pub const ALL: [Self; 16] = [
        Self::SessionId,
        Self::SessionRef,
        Self::TargetRef,
        Self::SessionMode,
        Self::ControlPosture,
        Self::TruthClass,
        Self::Locality,
        Self::AdapterState,
        Self::StopReason,
        Self::ThreadNodeCount,
        Self::SelectedThread,
        Self::DumpRefs,
        Self::SymbolicationState,
        Self::BuildProvenance,
        Self::ProducingRunRef,
        Self::SessionRestored,
    ];

    /// The mandatory subset every row must carry: the session / target identity, the
    /// session mode, the derived control posture, and the truth class so a support export
    /// always reconstructs whether the evidence was live or captured (AC2).
    pub const MANDATORY: [Self; 6] = [
        Self::SessionId,
        Self::SessionRef,
        Self::TargetRef,
        Self::SessionMode,
        Self::ControlPosture,
        Self::TruthClass,
    ];

    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SessionId => "session_id",
            Self::SessionRef => "session_ref",
            Self::TargetRef => "target_ref",
            Self::SessionMode => "session_mode",
            Self::ControlPosture => "control_posture",
            Self::TruthClass => "truth_class",
            Self::Locality => "locality",
            Self::AdapterState => "adapter_state",
            Self::StopReason => "stop_reason",
            Self::ThreadNodeCount => "thread_node_count",
            Self::SelectedThread => "selected_thread",
            Self::DumpRefs => "dump_refs",
            Self::SymbolicationState => "symbolication_state",
            Self::BuildProvenance => "build_provenance",
            Self::ProducingRunRef => "producing_run_ref",
            Self::SessionRestored => "session_restored",
        }
    }
}

// --- resolver input ---

/// One node in the thread / process tree.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5DebugTreeNodeInput {
    /// Opaque ref to this process / thread node; never raw process bytes.
    pub node_ref: String,
    /// Whether the node is a process or a thread.
    pub node_kind: M5DebugNodeKind,
    /// Opaque ref to the parent node; `None` only for a root process node.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parent_ref: Option<String>,
    /// Human-readable label.
    pub label: String,
    /// Number of threads under a process node (at least one for a live process); the
    /// number of nodes represented at a thread node is not counted here.
    pub thread_count: u32,
    /// The node's run state.
    pub run_state: M5ThreadRunState,
    /// Whether this thread node is the currently selected thread.
    pub is_selected: bool,
    /// The safe actions this row offers (switch / detach and read-only actions).
    #[serde(default)]
    pub available_actions: Vec<M5DebugActionKind>,
}

/// One dump / crash artifact card.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5DumpCardInput {
    /// Opaque ref to the dump / crash artifact; never raw dump bytes.
    pub dump_ref: String,
    /// Opaque ref to the run that produced the dump; lineage is never lost.
    pub producing_run_ref: String,
    /// What kind of captured artifact this is.
    pub artifact_kind: M5DumpArtifactKind,
    /// How well the dump is symbolicated.
    pub symbolication: M5SymbolicationState,
    /// Human-readable capture-time label.
    pub capture_time_label: String,
    /// Human-readable build provenance (e.g. exact-build id).
    pub build_provenance_label: String,
    /// Human-readable symbol provenance (e.g. matched / partial symbols).
    pub symbol_provenance_label: String,
    /// How long the dump artifact is retained.
    pub retention: M5RetentionClass,
    /// The read-only actions this card offers (open-raw / export / copy); never a
    /// live-control action.
    #[serde(default)]
    pub available_actions: Vec<M5DebugActionKind>,
}

/// The full input to the debug-hierarchy resolver for one bounded debug session.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5DebugHierarchyInput {
    /// The stable session identity that must survive across every projection.
    pub session_id: String,
    /// Opaque ref to the debug session; never raw session bytes.
    pub session_ref: String,
    /// Opaque ref to the target / process identity; distinct from the session ref.
    pub target_ref: String,
    /// Human-readable session label.
    pub session_label: String,
    /// Human-readable context summary.
    pub context_summary: String,
    /// Relative age label of the session ("2m ago").
    pub age_label: String,
    /// How the session was established.
    pub session_mode: M5DebugSessionMode,
    /// The captured-versus-live truth class.
    pub truth_mode: M5ExecutionTruthMode,
    /// The local / remote / container / managed boundary.
    pub locality: M5ExecutionLocality,
    /// The debug adapter state.
    pub adapter_state: M5DebugAdapterState,
    /// The current stop reason.
    pub stop_reason: M5DebugStopReason,
    /// The session's run outcome.
    pub session_outcome: M5RunOutcome,
    /// Whether the session was restored (e.g. after a reload or reconnect).
    #[serde(default)]
    pub restored: bool,
    /// The thread / process tree nodes; at least one (a root).
    pub tree_nodes: Vec<M5DebugTreeNodeInput>,
    /// The currently-selected thread ref, when a thread is selected.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub selected_thread_ref: Option<String>,
    /// The dump / crash artifact cards; may be empty for a live session.
    #[serde(default)]
    pub dump_cards: Vec<M5DumpCardInput>,
    /// An externally-observed narrowing that degrades the surface.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub degraded: Option<DegradedState>,
}

// --- resolved projections ---

/// The resolved debug session header.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ResolvedDebugSessionHeader {
    /// The session identity — identical to every other projection.
    pub session_id: String,
    /// The opaque session ref.
    pub session_ref: String,
    /// The opaque target ref.
    pub target_ref: String,
    /// The session label.
    pub session_label: String,
    /// The session mode.
    pub session_mode: M5DebugSessionMode,
    /// The derived control posture.
    pub control_posture: M5DebugControlPosture,
    /// The captured-versus-live truth class.
    pub truth_mode: M5ExecutionTruthMode,
    /// The execution boundary.
    pub locality: M5ExecutionLocality,
    /// The debug adapter state.
    pub adapter_state: M5DebugAdapterState,
    /// The current stop reason.
    pub stop_reason: M5DebugStopReason,
    /// The session run outcome.
    pub session_outcome: M5RunOutcome,
    /// Whether the session was restored.
    pub restored: bool,
    /// The header holds live attached control rather than captured evidence.
    pub is_live_control: bool,
    /// The header distinguishes live control from captured analysis; always holds by
    /// construction.
    pub distinguishes_live_from_captured: bool,
    /// The header makes the local / remote / container / managed boundary explicit;
    /// always holds by construction.
    pub boundary_explicit: bool,
}

/// The resolved thread / process tree row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ResolvedDebugTreeRow {
    /// The session identity — identical to every other projection.
    pub session_id: String,
    /// The opaque session ref.
    pub session_ref: String,
    /// The opaque target ref.
    pub target_ref: String,
    /// The opaque node ref.
    pub node_ref: String,
    /// Whether the node is a process or a thread.
    pub node_kind: M5DebugNodeKind,
    /// The parent node ref, when present.
    pub parent_ref: Option<String>,
    /// The depth of this node in the tree (0 for a root).
    pub depth: u32,
    /// The node label.
    pub label: String,
    /// The thread count under a process node.
    pub thread_count: u32,
    /// The node's run state.
    pub run_state: M5ThreadRunState,
    /// Whether this thread node is the selected thread.
    pub is_selected: bool,
    /// Whether the node's thread is paused.
    pub is_paused: bool,
    /// The safe actions this row offers.
    pub available_actions: Vec<M5DebugActionKind>,
    /// The row keeps its hierarchy parent linkage rather than flattening; always holds by
    /// construction.
    pub hierarchy_preserved: bool,
}

/// The resolved dump / crash artifact card.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ResolvedDumpCrashCard {
    /// The session identity — identical to every other projection.
    pub session_id: String,
    /// The opaque session ref.
    pub session_ref: String,
    /// The opaque target ref.
    pub target_ref: String,
    /// The opaque dump ref.
    pub dump_ref: String,
    /// The opaque producing-run ref (lineage).
    pub producing_run_ref: String,
    /// The dump artifact kind.
    pub artifact_kind: M5DumpArtifactKind,
    /// The dump symbolication state.
    pub symbolication: M5SymbolicationState,
    /// The capture-time label.
    pub capture_time_label: String,
    /// The build provenance label.
    pub build_provenance_label: String,
    /// The symbol provenance label.
    pub symbol_provenance_label: String,
    /// The dump retention class.
    pub retention: M5RetentionClass,
    /// A dump is captured evidence, never live control; always holds by construction.
    pub captured_truth: bool,
    /// The card never offers a live-control action; always holds by construction.
    pub implies_live_control: bool,
    /// The read-only actions this card offers.
    pub available_actions: Vec<M5DebugActionKind>,
    /// The symbolication state is disclosed on the card; always holds by construction.
    pub symbolication_disclosed: bool,
    /// The dump's producing-run lineage and build / symbol provenance are preserved;
    /// always holds by construction.
    pub provenance_preserved: bool,
}

/// The resolved CLI / headless line.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ResolvedDebugCliLine {
    /// The session identity — identical to every other projection.
    pub session_id: String,
    /// The opaque session ref.
    pub session_ref: String,
    /// The opaque target ref.
    pub target_ref: String,
    /// The deterministic single-line summary in the shared debug vocabulary.
    pub line: String,
    /// The session mode.
    pub session_mode: M5DebugSessionMode,
    /// The derived control posture.
    pub control_posture: M5DebugControlPosture,
}

/// A dump summary carried in the support-export projection.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5DebugExportDumpSummary {
    /// The opaque dump ref.
    pub dump_ref: String,
    /// The opaque producing-run ref.
    pub producing_run_ref: String,
    /// The dump artifact kind.
    pub artifact_kind: M5DumpArtifactKind,
    /// The dump symbolication state.
    pub symbolication: M5SymbolicationState,
}

/// A node summary carried in the support-export projection.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5DebugExportNodeSummary {
    /// The opaque node ref.
    pub node_ref: String,
    /// Whether the node is a process or a thread.
    pub node_kind: M5DebugNodeKind,
    /// The node's run state.
    pub run_state: M5ThreadRunState,
    /// The node depth in the tree.
    pub depth: u32,
}

/// The resolved support-export projection.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ResolvedDebugExport {
    /// The session identity — identical to every other projection.
    pub session_id: String,
    /// The opaque session ref — identical to every other projection.
    pub session_ref: String,
    /// The opaque target ref — identical to every other projection.
    pub target_ref: String,
    /// The session mode.
    pub session_mode: M5DebugSessionMode,
    /// The derived control posture.
    pub control_posture: M5DebugControlPosture,
    /// The captured-versus-live truth class.
    pub truth_mode: M5ExecutionTruthMode,
    /// The execution boundary.
    pub locality: M5ExecutionLocality,
    /// The debug adapter state.
    pub adapter_state: M5DebugAdapterState,
    /// The current stop reason.
    pub stop_reason: M5DebugStopReason,
    /// Whether the session was restored.
    pub restored: bool,
    /// The tree node summaries, preserving hierarchy depth.
    pub node_summaries: Vec<M5DebugExportNodeSummary>,
    /// The selected thread ref, when present.
    pub selected_thread_ref: Option<String>,
    /// The dump summaries, preserving lineage and symbolication.
    pub dump_summaries: Vec<M5DebugExportDumpSummary>,
    /// The export fields this projection carries; includes the mandatory subset.
    pub export_fields: Vec<M5DebugExportField>,
}

/// The resolved debug-hierarchy truth shared across the header, the tree rows, the dump
/// cards, the CLI line, and the support-export projection.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ResolvedDebugHierarchy {
    /// The stable session identity.
    pub session_id: String,
    /// The opaque session ref.
    pub session_ref: String,
    /// The opaque target ref.
    pub target_ref: String,
    /// The resolved debug session header.
    pub header: M5ResolvedDebugSessionHeader,
    /// The resolved thread / process tree rows.
    pub tree_rows: Vec<M5ResolvedDebugTreeRow>,
    /// The resolved dump / crash artifact cards.
    pub dump_cards: Vec<M5ResolvedDumpCrashCard>,
    /// The resolved CLI / headless line.
    pub cli_line: M5ResolvedDebugCliLine,
    /// The resolved support-export projection.
    pub export: M5ResolvedDebugExport,
    /// The debug hierarchy stays understandable even when restored / degraded /
    /// inspect-only (AC1).
    pub hierarchy_understandable: bool,
    /// Live attached control is distinguished from captured analysis (AC2).
    pub distinguishes_live_from_captured: bool,
    /// Thread rows and dump cards preserve mapping-quality and provenance truth (AC3).
    pub preserves_mapping_and_provenance: bool,
    /// The narrowing carried through from the input, when present.
    pub degraded: Option<DegradedState>,
}

impl M5ResolvedDebugHierarchy {
    /// True when the session identity, session ref, and target ref are identical across
    /// the header, the tree rows, the dump cards, the CLI line, and the export.
    pub fn identity_consistent(&self) -> bool {
        let rows_ok = self.tree_rows.iter().all(|row| {
            row.session_id == self.session_id
                && row.session_ref == self.session_ref
                && row.target_ref == self.target_ref
        });
        let cards_ok = self.dump_cards.iter().all(|card| {
            card.session_id == self.session_id
                && card.session_ref == self.session_ref
                && card.target_ref == self.target_ref
        });
        self.header.session_id == self.session_id
            && self.header.session_ref == self.session_ref
            && self.header.target_ref == self.target_ref
            && rows_ok
            && cards_ok
            && self.cli_line.session_id == self.session_id
            && self.cli_line.session_ref == self.session_ref
            && self.cli_line.target_ref == self.target_ref
            && self.export.session_id == self.session_id
            && self.export.session_ref == self.session_ref
            && self.export.target_ref == self.target_ref
    }

    /// True when the thread / process tree keeps its parent linkage and depth rather than
    /// flattening, and every projection carries the identity and control posture, so a
    /// restored / degraded / inspect-only session reconstructs the same story (AC1).
    pub fn hierarchy_understandable_when_narrowed(&self) -> bool {
        let hierarchy_preserved = self.tree_rows.iter().all(|row| row.hierarchy_preserved)
            && self
                .tree_rows
                .iter()
                .all(|row| row.parent_ref.is_some() == (row.depth > 0));
        let export_carries_hierarchy = self.export.node_summaries.len() == self.tree_rows.len();
        hierarchy_preserved
            && export_carries_hierarchy
            && !self.tree_rows.is_empty()
            && self.header.distinguishes_live_from_captured
    }

    /// True when the control posture is derived purely from the session mode, is honest
    /// against the truth class, and no dump card implies live control (AC2).
    pub fn distinguishes_control(&self) -> bool {
        let posture = M5DebugControlPosture::for_mode(self.header.session_mode);
        let posture_matches = self.header.control_posture == posture
            && self.header.is_live_control == posture.is_live_control()
            && self.cli_line.control_posture == posture
            && self.export.control_posture == posture;
        let truth_honest = if posture.is_live_control() {
            self.header.truth_mode.is_live()
        } else {
            !self.header.truth_mode.is_live()
        };
        let dumps_never_live = self
            .dump_cards
            .iter()
            .all(|card| card.captured_truth && !card.implies_live_control);
        posture_matches && truth_honest && dumps_never_live
    }

    /// True when every dump card preserves its lineage, symbolication, and provenance, and
    /// every tree row preserves node identity and run state (AC3).
    pub fn preserves_mapping_and_provenance(&self) -> bool {
        let cards_ok = self.dump_cards.iter().all(|card| {
            card.provenance_preserved
                && card.symbolication_disclosed
                && !card.dump_ref.trim().is_empty()
                && !card.producing_run_ref.trim().is_empty()
                && !card.build_provenance_label.trim().is_empty()
                && !card.symbol_provenance_label.trim().is_empty()
        });
        let rows_ok = self
            .tree_rows
            .iter()
            .all(|row| !row.node_ref.trim().is_empty() && !row.label.trim().is_empty());
        cards_ok && rows_ok
    }
}

/// Errors returned by [`resolve_debug_hierarchy`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum M5DebugHierarchyError {
    /// The session identity was empty.
    EmptySessionId,
    /// The session ref was empty.
    EmptySessionRef,
    /// The target ref was empty.
    EmptyTargetRef,
    /// The session label was empty.
    EmptySessionLabel,
    /// The context summary was empty.
    EmptyContextSummary,
    /// The age label was empty.
    EmptyAgeLabel,
    /// The session ref and target ref were equal — identity collapsed.
    SessionTargetIdentityCollapsed,
    /// A label, ref, or note carried forbidden material.
    ForbiddenMaterial,
    /// The thread / process tree had no nodes.
    EmptyTree,
    /// A tree node ref appeared more than once.
    DuplicateNode,
    /// A tree node carried an empty ref or label.
    NodeIncomplete,
    /// The tree had no root node (a node with no parent).
    TreeRootMissing,
    /// A non-root node named a parent that is not in the tree.
    TreeParentMissing,
    /// A process node claimed zero threads.
    ProcessThreadCountInvalid,
    /// More than one thread node was marked selected.
    MultipleThreadsSelected,
    /// The selected-thread ref is not a thread node in the tree.
    SelectedThreadNotInTree,
    /// The selected-thread ref and the selected node disagree.
    SelectedThreadMismatch,
    /// A live-control session claimed non-live truth (or vice versa).
    ControlPostureTruthMismatch,
    /// A live-control session's adapter cannot carry live control.
    LiveControlAdapterUnavailable,
    /// The stop reason is inconsistent with the control posture.
    StopReasonInconsistentWithControl,
    /// A captured / inspect-only tree row offered a live-control action.
    CapturedTreeRowImpliesLiveControl,
    /// A dump card ref was empty.
    DumpRefEmpty,
    /// A dump card lost its producing-run lineage.
    DumpLineageBroken,
    /// A dump card carried empty build / symbol provenance.
    DumpProvenanceMissing,
    /// A dump card carried an empty capture-time label.
    DumpCaptureTimeMissing,
    /// A dump card offered a live-control action.
    DumpCardImpliesLiveControl,
    /// A dump ref appeared more than once.
    DuplicateDump,
    /// A degraded block carried a generic non-answer label.
    DegradedLabelGeneric,
}

impl M5DebugHierarchyError {
    /// Stable token for tests and diagnostics.
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::EmptySessionId => "empty_session_id",
            Self::EmptySessionRef => "empty_session_ref",
            Self::EmptyTargetRef => "empty_target_ref",
            Self::EmptySessionLabel => "empty_session_label",
            Self::EmptyContextSummary => "empty_context_summary",
            Self::EmptyAgeLabel => "empty_age_label",
            Self::SessionTargetIdentityCollapsed => "session_target_identity_collapsed",
            Self::ForbiddenMaterial => "forbidden_material",
            Self::EmptyTree => "empty_tree",
            Self::DuplicateNode => "duplicate_node",
            Self::NodeIncomplete => "node_incomplete",
            Self::TreeRootMissing => "tree_root_missing",
            Self::TreeParentMissing => "tree_parent_missing",
            Self::ProcessThreadCountInvalid => "process_thread_count_invalid",
            Self::MultipleThreadsSelected => "multiple_threads_selected",
            Self::SelectedThreadNotInTree => "selected_thread_not_in_tree",
            Self::SelectedThreadMismatch => "selected_thread_mismatch",
            Self::ControlPostureTruthMismatch => "control_posture_truth_mismatch",
            Self::LiveControlAdapterUnavailable => "live_control_adapter_unavailable",
            Self::StopReasonInconsistentWithControl => "stop_reason_inconsistent_with_control",
            Self::CapturedTreeRowImpliesLiveControl => "captured_tree_row_implies_live_control",
            Self::DumpRefEmpty => "dump_ref_empty",
            Self::DumpLineageBroken => "dump_lineage_broken",
            Self::DumpProvenanceMissing => "dump_provenance_missing",
            Self::DumpCaptureTimeMissing => "dump_capture_time_missing",
            Self::DumpCardImpliesLiveControl => "dump_card_implies_live_control",
            Self::DuplicateDump => "duplicate_dump",
            Self::DegradedLabelGeneric => "degraded_label_generic",
        }
    }
}

impl fmt::Display for M5DebugHierarchyError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "debug-hierarchy resolution error: {}", self.as_str())
    }
}

impl Error for M5DebugHierarchyError {}

/// Resolves one debug session into its shared header, thread / process tree rows, dump /
/// crash artifact cards, CLI / headless line, and support-export projection.
///
/// The projections share one session identity and one target identity, so live-versus-
/// captured control truth, the thread / process hierarchy, and dump symbolication /
/// provenance never blur. The control posture is derived purely from the session mode;
/// the hierarchy keeps its parent linkage; and every dump card preserves its lineage and
/// provenance while never implying live control.
///
/// # Errors
///
/// Returns an [`M5DebugHierarchyError`] when identity is missing or collapsed, the tree is
/// empty or broken, the control posture disagrees with the truth class or adapter state, a
/// captured surface offers a live-control action, a dump loses its lineage or provenance,
/// or any ref / label carries forbidden material.
pub fn resolve_debug_hierarchy(
    input: &M5DebugHierarchyInput,
) -> Result<M5ResolvedDebugHierarchy, M5DebugHierarchyError> {
    if input.session_id.trim().is_empty() {
        return Err(M5DebugHierarchyError::EmptySessionId);
    }
    if input.session_ref.trim().is_empty() {
        return Err(M5DebugHierarchyError::EmptySessionRef);
    }
    if input.target_ref.trim().is_empty() {
        return Err(M5DebugHierarchyError::EmptyTargetRef);
    }
    if input.session_label.trim().is_empty() {
        return Err(M5DebugHierarchyError::EmptySessionLabel);
    }
    if input.context_summary.trim().is_empty() {
        return Err(M5DebugHierarchyError::EmptyContextSummary);
    }
    if input.age_label.trim().is_empty() {
        return Err(M5DebugHierarchyError::EmptyAgeLabel);
    }
    if input.session_ref.trim() == input.target_ref.trim() {
        return Err(M5DebugHierarchyError::SessionTargetIdentityCollapsed);
    }

    for value in [
        input.session_ref.as_str(),
        input.target_ref.as_str(),
        input.session_label.as_str(),
        input.context_summary.as_str(),
        input.age_label.as_str(),
    ] {
        if value_is_forbidden(value) {
            return Err(M5DebugHierarchyError::ForbiddenMaterial);
        }
    }

    let control_posture = M5DebugControlPosture::for_mode(input.session_mode);

    // AC2: the control posture must be honest against the truth class — a live-control
    // session is live truth; a captured / inspect-only session is never live truth.
    let truth_honest = if control_posture.is_live_control() {
        input.truth_mode.is_live()
    } else {
        !input.truth_mode.is_live()
    };
    if !truth_honest {
        return Err(M5DebugHierarchyError::ControlPostureTruthMismatch);
    }
    // A live-control session's adapter must be able to carry live control.
    if control_posture.is_live_control() && !input.adapter_state.is_live_capable() {
        return Err(M5DebugHierarchyError::LiveControlAdapterUnavailable);
    }
    // The stop reason must be consistent with the control posture.
    if !stop_reason_consistent(control_posture, input.stop_reason) {
        return Err(M5DebugHierarchyError::StopReasonInconsistentWithControl);
    }

    let tree_rows = resolve_tree_rows(input, control_posture)?;
    let dump_cards = resolve_dump_cards(input)?;

    if let Some(degraded) = &input.degraded {
        if !degraded.is_honest() {
            return Err(M5DebugHierarchyError::DegradedLabelGeneric);
        }
    }

    let header = M5ResolvedDebugSessionHeader {
        session_id: input.session_id.clone(),
        session_ref: input.session_ref.clone(),
        target_ref: input.target_ref.clone(),
        session_label: input.session_label.clone(),
        session_mode: input.session_mode,
        control_posture,
        truth_mode: input.truth_mode,
        locality: input.locality,
        adapter_state: input.adapter_state,
        stop_reason: input.stop_reason,
        session_outcome: input.session_outcome,
        restored: input.restored,
        is_live_control: control_posture.is_live_control(),
        distinguishes_live_from_captured: true,
        boundary_explicit: true,
    };

    let node_summaries: Vec<M5DebugExportNodeSummary> = tree_rows
        .iter()
        .map(|row| M5DebugExportNodeSummary {
            node_ref: row.node_ref.clone(),
            node_kind: row.node_kind,
            run_state: row.run_state,
            depth: row.depth,
        })
        .collect();
    let dump_summaries: Vec<M5DebugExportDumpSummary> = dump_cards
        .iter()
        .map(|card| M5DebugExportDumpSummary {
            dump_ref: card.dump_ref.clone(),
            producing_run_ref: card.producing_run_ref.clone(),
            artifact_kind: card.artifact_kind,
            symbolication: card.symbolication,
        })
        .collect();

    let cli_line = M5ResolvedDebugCliLine {
        session_id: input.session_id.clone(),
        session_ref: input.session_ref.clone(),
        target_ref: input.target_ref.clone(),
        line: render_cli_line(input, control_posture, &tree_rows, &dump_cards),
        session_mode: input.session_mode,
        control_posture,
    };

    let export = M5ResolvedDebugExport {
        session_id: input.session_id.clone(),
        session_ref: input.session_ref.clone(),
        target_ref: input.target_ref.clone(),
        session_mode: input.session_mode,
        control_posture,
        truth_mode: input.truth_mode,
        locality: input.locality,
        adapter_state: input.adapter_state,
        stop_reason: input.stop_reason,
        restored: input.restored,
        node_summaries,
        selected_thread_ref: input.selected_thread_ref.clone(),
        dump_summaries,
        export_fields: M5DebugExportField::ALL.to_vec(),
    };

    Ok(M5ResolvedDebugHierarchy {
        session_id: input.session_id.clone(),
        session_ref: input.session_ref.clone(),
        target_ref: input.target_ref.clone(),
        header,
        tree_rows,
        dump_cards,
        cli_line,
        export,
        hierarchy_understandable: true,
        distinguishes_live_from_captured: true,
        preserves_mapping_and_provenance: true,
        degraded: input.degraded.clone(),
    })
}

fn resolve_tree_rows(
    input: &M5DebugHierarchyInput,
    control_posture: M5DebugControlPosture,
) -> Result<Vec<M5ResolvedDebugTreeRow>, M5DebugHierarchyError> {
    if input.tree_nodes.is_empty() {
        return Err(M5DebugHierarchyError::EmptyTree);
    }

    let mut refs: BTreeSet<&str> = BTreeSet::new();
    for node in &input.tree_nodes {
        if node.node_ref.trim().is_empty() || node.label.trim().is_empty() {
            return Err(M5DebugHierarchyError::NodeIncomplete);
        }
        for value in [node.node_ref.as_str(), node.label.as_str()]
            .into_iter()
            .chain(node.parent_ref.as_deref())
        {
            if value_is_forbidden(value) {
                return Err(M5DebugHierarchyError::ForbiddenMaterial);
            }
        }
        if !refs.insert(node.node_ref.trim()) {
            return Err(M5DebugHierarchyError::DuplicateNode);
        }
    }

    // The tree must have at least one root (a node with no parent), and every non-root
    // parent must be present, so the hierarchy is never dangling or flattened.
    let mut has_root = false;
    for node in &input.tree_nodes {
        match &node.parent_ref {
            None => has_root = true,
            Some(parent) => {
                if !refs.contains(parent.trim()) {
                    return Err(M5DebugHierarchyError::TreeParentMissing);
                }
            }
        }
        if node.node_kind == M5DebugNodeKind::Process
            && node.run_state != M5ThreadRunState::Exited
            && node.thread_count == 0
        {
            return Err(M5DebugHierarchyError::ProcessThreadCountInvalid);
        }
    }
    if !has_root {
        return Err(M5DebugHierarchyError::TreeRootMissing);
    }

    // At most one thread may be selected, and it must agree with the selected-thread ref.
    let selected: Vec<&M5DebugTreeNodeInput> =
        input.tree_nodes.iter().filter(|node| node.is_selected).collect();
    if selected.len() > 1 {
        return Err(M5DebugHierarchyError::MultipleThreadsSelected);
    }
    if let Some(selected_ref) = input.selected_thread_ref.as_deref() {
        let matches_selected = input
            .tree_nodes
            .iter()
            .any(|node| node.node_ref.trim() == selected_ref.trim() && node.node_kind.is_thread());
        if !matches_selected {
            return Err(M5DebugHierarchyError::SelectedThreadNotInTree);
        }
        match selected.first() {
            Some(node) if node.node_ref.trim() != selected_ref.trim() => {
                return Err(M5DebugHierarchyError::SelectedThreadMismatch);
            }
            _ => {}
        }
    } else if !selected.is_empty() {
        return Err(M5DebugHierarchyError::SelectedThreadMismatch);
    }

    let mut rows = Vec::with_capacity(input.tree_nodes.len());
    for node in &input.tree_nodes {
        // AC2: a captured / inspect-only surface must not offer a live-control action.
        if !control_posture.is_live_control()
            && node
                .available_actions
                .iter()
                .any(|action| action.implies_live_control())
        {
            return Err(M5DebugHierarchyError::CapturedTreeRowImpliesLiveControl);
        }

        let depth = node_depth(&input.tree_nodes, node);
        rows.push(M5ResolvedDebugTreeRow {
            session_id: input.session_id.clone(),
            session_ref: input.session_ref.clone(),
            target_ref: input.target_ref.clone(),
            node_ref: node.node_ref.clone(),
            node_kind: node.node_kind,
            parent_ref: node.parent_ref.clone(),
            depth,
            label: node.label.clone(),
            thread_count: node.thread_count,
            run_state: node.run_state,
            is_selected: node.is_selected,
            is_paused: node.run_state.is_paused(),
            available_actions: node.available_actions.clone(),
            hierarchy_preserved: true,
        });
    }
    Ok(rows)
}

fn resolve_dump_cards(
    input: &M5DebugHierarchyInput,
) -> Result<Vec<M5ResolvedDumpCrashCard>, M5DebugHierarchyError> {
    let mut seen: BTreeSet<&str> = BTreeSet::new();
    let mut cards = Vec::with_capacity(input.dump_cards.len());
    for dump in &input.dump_cards {
        if dump.dump_ref.trim().is_empty() {
            return Err(M5DebugHierarchyError::DumpRefEmpty);
        }
        if dump.producing_run_ref.trim().is_empty() {
            return Err(M5DebugHierarchyError::DumpLineageBroken);
        }
        if dump.build_provenance_label.trim().is_empty()
            || dump.symbol_provenance_label.trim().is_empty()
        {
            return Err(M5DebugHierarchyError::DumpProvenanceMissing);
        }
        if dump.capture_time_label.trim().is_empty() {
            return Err(M5DebugHierarchyError::DumpCaptureTimeMissing);
        }
        for value in [
            dump.dump_ref.as_str(),
            dump.producing_run_ref.as_str(),
            dump.capture_time_label.as_str(),
            dump.build_provenance_label.as_str(),
            dump.symbol_provenance_label.as_str(),
        ] {
            if value_is_forbidden(value) {
                return Err(M5DebugHierarchyError::ForbiddenMaterial);
            }
        }
        // AC2: a dump card is captured evidence and must never offer a live-control action.
        if dump
            .available_actions
            .iter()
            .any(|action| action.implies_live_control())
        {
            return Err(M5DebugHierarchyError::DumpCardImpliesLiveControl);
        }
        if !seen.insert(dump.dump_ref.trim()) {
            return Err(M5DebugHierarchyError::DuplicateDump);
        }

        cards.push(M5ResolvedDumpCrashCard {
            session_id: input.session_id.clone(),
            session_ref: input.session_ref.clone(),
            target_ref: input.target_ref.clone(),
            dump_ref: dump.dump_ref.clone(),
            producing_run_ref: dump.producing_run_ref.clone(),
            artifact_kind: dump.artifact_kind,
            symbolication: dump.symbolication,
            capture_time_label: dump.capture_time_label.clone(),
            build_provenance_label: dump.build_provenance_label.clone(),
            symbol_provenance_label: dump.symbol_provenance_label.clone(),
            retention: dump.retention,
            captured_truth: true,
            implies_live_control: false,
            available_actions: dump.available_actions.clone(),
            symbolication_disclosed: true,
            provenance_preserved: true,
        });
    }
    Ok(cards)
}

/// The depth of a node in the tree, following `parent_ref` links up to a root. Guards
/// against cycles by bounding the walk to the node count.
fn node_depth(nodes: &[M5DebugTreeNodeInput], node: &M5DebugTreeNodeInput) -> u32 {
    let mut depth = 0u32;
    let mut current = node.parent_ref.as_deref();
    let mut guard = nodes.len();
    while let Some(parent) = current {
        depth += 1;
        if guard == 0 {
            break;
        }
        guard -= 1;
        current = nodes
            .iter()
            .find(|candidate| candidate.node_ref.trim() == parent.trim())
            .and_then(|found| found.parent_ref.as_deref());
    }
    depth
}

/// True when a control posture and a stop reason are consistent: a live-control session is
/// never a captured crash, and a captured / inspect-only session is never actively
/// running.
fn stop_reason_consistent(posture: M5DebugControlPosture, reason: M5DebugStopReason) -> bool {
    if posture.is_live_control() {
        !reason.is_crash_capture()
    } else {
        !reason.is_running()
    }
}

/// Renders the deterministic CLI / headless line in the shared debug vocabulary.
fn render_cli_line(
    input: &M5DebugHierarchyInput,
    control_posture: M5DebugControlPosture,
    tree_rows: &[M5ResolvedDebugTreeRow],
    dump_cards: &[M5ResolvedDumpCrashCard],
) -> String {
    let selected = input.selected_thread_ref.as_deref().unwrap_or("-");
    format!(
        "session={session} target={target} mode={mode} posture={posture} truth={truth} \
adapter={adapter} stop={stop} nodes={nodes} selected={selected} dumps={dumps} restored={restored}",
        session = input.session_id,
        target = input.target_ref,
        mode = input.session_mode.as_str(),
        posture = control_posture.as_str(),
        truth = input.truth_mode.as_str(),
        adapter = input.adapter_state.as_str(),
        stop = input.stop_reason.as_str(),
        nodes = tree_rows.len(),
        selected = selected,
        dumps = dump_cards.len(),
        restored = input.restored,
    )
}

/// True when a slice of export fields declares every mandatory field.
fn declares_mandatory_export_fields(fields: &[M5DebugExportField]) -> bool {
    let present: BTreeSet<M5DebugExportField> = fields.iter().copied().collect();
    M5DebugExportField::MANDATORY
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
/// reconstructs debug truth from the shared model.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5DebugHierarchyCase {
    /// The resolver input.
    pub input: M5DebugHierarchyInput,
    /// The resolved hierarchy. Must equal `resolve_debug_hierarchy(&input)`.
    pub resolved: M5ResolvedDebugHierarchy,
}

impl M5DebugHierarchyCase {
    /// Builds a case by resolving `input`.
    ///
    /// # Panics
    ///
    /// Panics if `input` does not resolve; seed inputs are always valid.
    pub fn resolved(input: M5DebugHierarchyInput) -> Self {
        let resolved = resolve_debug_hierarchy(&input).expect("seed debug-hierarchy case is valid");
        Self { input, resolved }
    }

    /// True when the stored resolution matches a fresh resolve of the input.
    pub fn is_self_consistent(&self) -> bool {
        resolve_debug_hierarchy(&self.input).as_ref() == Ok(&self.resolved)
    }
}

/// One row in the primitive matrix: one execution surface family bound to the shared
/// debug-hierarchy contract.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5DebugSurfaceRow {
    /// The execution surface family.
    pub surface_family: M5RunAttemptSurfaceFamily,
    /// Owner role accountable for keeping this surface governed.
    pub owner_role: String,
    /// Human-readable scope summary.
    pub scope_summary: String,
    /// Session modes this surface can host (must be non-empty).
    pub session_modes: Vec<M5DebugSessionMode>,
    /// Control postures this surface can present (must be non-empty).
    pub control_postures: Vec<M5DebugControlPosture>,
    /// Export fields this row carries (must include the mandatory fields).
    pub export_fields: Vec<M5DebugExportField>,
    /// Downgrade triggers that apply to this surface (must be non-empty).
    pub downgrade_triggers: Vec<M5ExecutionDowngradeTrigger>,
    /// Consumer surfaces that ingest this row's projection (must be non-empty).
    pub consumer_surfaces: Vec<String>,
    /// Source contract refs consumed by this row (must be non-empty).
    pub source_contract_refs: Vec<String>,
    /// Worked resolution cases proving the resolver on this surface (must be non-empty).
    pub example_sessions: Vec<M5DebugHierarchyCase>,
    /// Hard invariant: this row never blurs live control with captured analysis. MUST be
    /// `false`.
    pub blurs_live_and_captured: bool,
    /// Hard invariant: this row never flattens the thread / process hierarchy. MUST be
    /// `false`.
    pub flattens_hierarchy: bool,
    /// Hard invariant: this row never drops dump lineage or provenance. MUST be `false`.
    pub drops_provenance: bool,
    /// Hard invariant: this row never lets a dump card imply live control. MUST be
    /// `false`.
    pub dump_implies_live_control: bool,
}

impl M5DebugSurfaceRow {
    /// True when the row declares every mandatory export field.
    fn declares_mandatory_export_fields(&self) -> bool {
        declares_mandatory_export_fields(&self.export_fields)
    }

    /// True when the row's hard invariants hold.
    fn honours_invariants(&self) -> bool {
        !self.blurs_live_and_captured
            && !self.flattens_hierarchy
            && !self.drops_provenance
            && !self.dump_implies_live_control
    }
}

/// Self-describing controlled-vocabulary set minted / reused by this primitive.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5DebugVocabularySet {
    /// Surface-family tokens (reused from the run/attempt-header primitive).
    pub surface_families: Vec<String>,
    /// Debug-session-mode tokens (reused from the frozen matrix).
    pub session_modes: Vec<String>,
    /// Control-posture tokens.
    pub control_postures: Vec<String>,
    /// Adapter-state tokens.
    pub adapter_states: Vec<String>,
    /// Stop-reason tokens.
    pub stop_reasons: Vec<String>,
    /// Node-kind tokens.
    pub node_kinds: Vec<String>,
    /// Thread-run-state tokens.
    pub thread_run_states: Vec<String>,
    /// Action-kind tokens.
    pub action_kinds: Vec<String>,
    /// Dump-artifact-kind tokens.
    pub dump_artifact_kinds: Vec<String>,
    /// Symbolication-state tokens (reused from the frozen matrix).
    pub symbolication_states: Vec<String>,
    /// Retention-class tokens (reused from the frozen matrix).
    pub retention_classes: Vec<String>,
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

impl M5DebugVocabularySet {
    /// Builds the canonical vocabulary set from the typed `ALL` arrays.
    pub fn canonical() -> Self {
        Self {
            surface_families: tokens(&M5RunAttemptSurfaceFamily::ALL, |v| v.as_str()),
            session_modes: tokens(&DEBUG_SESSION_MODE_ALL, |v| v.as_str()),
            control_postures: tokens(&M5DebugControlPosture::ALL, |v| v.as_str()),
            adapter_states: tokens(&M5DebugAdapterState::ALL, |v| v.as_str()),
            stop_reasons: tokens(&M5DebugStopReason::ALL, |v| v.as_str()),
            node_kinds: tokens(&M5DebugNodeKind::ALL, |v| v.as_str()),
            thread_run_states: tokens(&M5ThreadRunState::ALL, |v| v.as_str()),
            action_kinds: tokens(&M5DebugActionKind::ALL, |v| v.as_str()),
            dump_artifact_kinds: tokens(&M5DumpArtifactKind::ALL, |v| v.as_str()),
            symbolication_states: tokens(&SYMBOLICATION_ALL, |v| v.as_str()),
            retention_classes: tokens(&RETENTION_ALL, |v| v.as_str()),
            export_fields: tokens(&M5DebugExportField::ALL, |v| v.as_str()),
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

/// The debug session modes reused from the frozen matrix, in a stable order.
const DEBUG_SESSION_MODE_ALL: [M5DebugSessionMode; 5] = [
    M5DebugSessionMode::Launch,
    M5DebugSessionMode::Attach,
    M5DebugSessionMode::Core,
    M5DebugSessionMode::Replay,
    M5DebugSessionMode::InspectOnly,
];

/// The symbolication states reused from the frozen matrix, in a stable order.
const SYMBOLICATION_ALL: [M5SymbolicationState; 4] = [
    M5SymbolicationState::Symbolicated,
    M5SymbolicationState::PartialSymbols,
    M5SymbolicationState::Unsymbolicated,
    M5SymbolicationState::SymbolsUnavailable,
];

/// The retention classes reused from the frozen matrix, in a stable order.
const RETENTION_ALL: [M5RetentionClass; 5] = [
    M5RetentionClass::RetainedDurable,
    M5RetentionClass::ExpiresScheduled,
    M5RetentionClass::EphemeralSessionOnly,
    M5RetentionClass::EvictedRecoverable,
    M5RetentionClass::EvictedGone,
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
pub struct M5DebugGovernanceReview {
    /// One primitive carries header / tree-row / dump-card / CLI-line / export truth on
    /// every surface.
    pub one_primitive_carries_all_surfaces: bool,
    /// Live attached control is never blurred with captured crash analysis.
    pub live_control_never_blurs_with_captured: bool,
    /// The thread / process hierarchy is never flattened.
    pub hierarchy_never_flattened: bool,
    /// Dump lineage, symbolication, and provenance are preserved.
    pub provenance_and_symbolication_preserved: bool,
    /// Dump cards never imply live control.
    pub dump_cards_never_imply_live_control: bool,
    /// The support / export packet reconstructs the debug hierarchy.
    pub support_export_reconstructs_debug_hierarchy: bool,
    /// Later M5 rows cannot invent parallel debug / hierarchy vocabulary.
    pub later_rows_cannot_invent_parallel_vocabulary: bool,
}

/// Consumer projection block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5DebugConsumerProjection {
    /// Task / test / request / notebook / AI / publish / preview surfaces all consume the
    /// shared primitive.
    pub execution_surfaces_consume_shared_primitive: bool,
    /// The debug resolver reads a single canonical model.
    pub resolver_reads_single_model: bool,
    /// The tree rows read a single canonical hierarchy source.
    pub tree_rows_read_single_hierarchy_source: bool,
    /// Support / export reads a single canonical debug source.
    pub support_export_reads_single_source: bool,
}

/// Release and support parity posture for the debug primitive.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5DebugReleasePosture {
    /// Ref of the supporting release packet.
    pub release_packet_ref: String,
    /// Ref of the supporting debug audit.
    pub debug_audit_ref: String,
    /// True when support / export parity is required for every surface.
    pub support_export_parity_required: bool,
    /// True when accessibility parity is required for every surface.
    pub accessibility_parity_required: bool,
}

/// Constructor input for [`M5DebugHierarchyPrimitivePacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M5DebugHierarchyPrimitivePacketInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable matrix label.
    pub matrix_label: String,
    /// Surface rows.
    pub surface_rows: Vec<M5DebugSurfaceRow>,
    /// Frozen controlled-vocabulary set.
    pub vocabulary_set: M5DebugVocabularySet,
    /// Governance-review block.
    pub governance_review: M5DebugGovernanceReview,
    /// Consumer projection block.
    pub consumer_projection: M5DebugConsumerProjection,
    /// Release and support parity posture.
    pub release_posture: M5DebugReleasePosture,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

/// Export-safe M5 debug-session-hierarchy primitive packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5DebugHierarchyPrimitivePacket {
    /// Record kind; must equal [`M5_DEBUG_HIERARCHY_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`M5_DEBUG_HIERARCHY_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable matrix label.
    pub matrix_label: String,
    /// Surface rows.
    pub surface_rows: Vec<M5DebugSurfaceRow>,
    /// Frozen controlled-vocabulary set.
    pub vocabulary_set: M5DebugVocabularySet,
    /// Governance-review block.
    pub governance_review: M5DebugGovernanceReview,
    /// Consumer projection block.
    pub consumer_projection: M5DebugConsumerProjection,
    /// Release and support parity posture.
    pub release_posture: M5DebugReleasePosture,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

impl M5DebugHierarchyPrimitivePacket {
    /// Builds an M5 debug primitive packet from stable-lane input.
    pub fn new(input: M5DebugHierarchyPrimitivePacketInput) -> Self {
        Self {
            record_kind: M5_DEBUG_HIERARCHY_RECORD_KIND.to_owned(),
            schema_version: M5_DEBUG_HIERARCHY_SCHEMA_VERSION,
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

    /// Validates the M5 debug primitive invariants.
    pub fn validate(&self) -> Vec<M5DebugViolation> {
        let mut violations = Vec::new();

        if self.record_kind != M5_DEBUG_HIERARCHY_RECORD_KIND {
            violations.push(M5DebugViolation::WrongRecordKind);
        }
        if self.schema_version != M5_DEBUG_HIERARCHY_SCHEMA_VERSION {
            violations.push(M5DebugViolation::WrongSchemaVersion);
        }
        if self.packet_id.trim().is_empty()
            || self.matrix_label.trim().is_empty()
            || self.redaction_class_token.trim().is_empty()
            || self.minted_at.trim().is_empty()
        {
            violations.push(M5DebugViolation::MissingIdentity);
        }

        validate_source_contracts(self, &mut violations);
        validate_vocabulary_set(self, &mut violations);
        validate_surface_rows(self, &mut violations);
        validate_acceptance_criteria_covered(self, &mut violations);
        validate_governance_review(self, &mut violations);
        validate_consumer_projection(self, &mut violations);
        validate_release_posture(self, &mut violations);

        if json_contains_forbidden_material(
            &serde_json::to_value(self).expect("m5 debug primitive packet serializes"),
        ) {
            violations.push(M5DebugViolation::RawMaterialInExport);
        }

        violations
    }

    /// Deterministic export-safe JSON.
    ///
    /// # Panics
    ///
    /// Panics only if serializing this metadata-only packet fails.
    pub fn export_safe_json(&self) -> String {
        serde_json::to_string_pretty(self).expect("m5 debug primitive packet serializes")
    }

    /// Deterministic, machine-readable matrix CSV: one row per surface family.
    pub fn render_matrix_csv(&self) -> String {
        let mut out = String::new();
        out.push_str("surface_family,owner,session_modes,control_postures,example_count\n");
        for row in &self.surface_rows {
            out.push_str(&format!(
                "{},{},{},{},{}\n",
                row.surface_family.as_str(),
                csv_field(&row.owner_role),
                join_tokens(&row.session_modes, |v| v.as_str()),
                join_tokens(&row.control_postures, |v| v.as_str()),
                row.example_sessions.len(),
            ));
        }
        out
    }

    /// Deterministic Markdown report for support, docs, or review handoff.
    pub fn render_markdown_summary(&self) -> String {
        let mut out = String::new();
        out.push_str("# M5 Debug-Session-Hierarchy Primitive\n\n");
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- Label: `{}`\n", self.matrix_label));
        out.push_str(&format!(
            "- Execution surfaces: {} / {}\n",
            self.surface_rows.len(),
            M5RunAttemptSurfaceFamily::ALL.len(),
        ));
        out.push_str(&format!(
            "- Session modes: {}\n",
            self.vocabulary_set.session_modes.join(", ")
        ));
        out.push_str(&format!(
            "- Control postures: {}\n",
            self.vocabulary_set.control_postures.join(", ")
        ));
        out.push_str("\n## Execution surfaces\n\n");
        for row in &self.surface_rows {
            out.push_str(&format!("- **{}**\n", row.surface_family.label()));
            out.push_str(&format!("  - Owner: {}\n", row.owner_role));
            out.push_str(&format!("  - Scope: {}\n", row.scope_summary));
            out.push_str(&format!("  - Worked cases: {}\n", row.example_sessions.len()));
            for case in &row.example_sessions {
                out.push_str(&format!(
                    "    - `{}` → target `{}` [{}], {} / {} ({} node(s), {} dump(s))\n",
                    case.resolved.session_id,
                    case.resolved.target_ref,
                    case.resolved.header.session_mode.as_str(),
                    case.resolved.header.control_posture.as_str(),
                    case.resolved.export.truth_mode.as_str(),
                    case.resolved.tree_rows.len(),
                    case.resolved.dump_cards.len(),
                ));
            }
        }
        out
    }
}

/// Errors emitted when reading the checked-in M5 debug export.
#[derive(Debug)]
pub enum M5DebugArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<M5DebugViolation>),
}

impl fmt::Display for M5DebugArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(formatter, "m5 debug primitive export parse failed: {error}")
            }
            Self::Validation(violations) => {
                let tokens = violations
                    .iter()
                    .map(|violation| violation.as_str())
                    .collect::<Vec<_>>()
                    .join(",");
                write!(
                    formatter,
                    "m5 debug primitive export failed validation: {tokens}"
                )
            }
        }
    }
}

impl Error for M5DebugArtifactError {}

/// Validation failures emitted by [`M5DebugHierarchyPrimitivePacket::validate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum M5DebugViolation {
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
    /// A surface row declares no session modes.
    SessionModesMissing,
    /// A surface row declares no control postures.
    ControlPosturesMissing,
    /// A surface row declares no downgrade triggers.
    DowngradeTriggersMissing,
    /// A surface row declares no consumer surfaces.
    ConsumerSurfacesMissing,
    /// A surface row declares no worked debug cases.
    ExampleSessionsMissing,
    /// A worked debug case does not match a fresh resolve of its input.
    ExampleSessionDrift,
    /// A surface row violates a hard invariant.
    SurfaceInvariantViolated,
    /// No worked case proves the hierarchy stays understandable when narrowed (AC1), or a
    /// session mode is not covered across the matrix.
    HierarchyUnderstandingUnproven,
    /// No worked case proves live control is distinguished from captured analysis (AC2),
    /// or a control posture is not covered.
    ControlDistinctionUnproven,
    /// No worked case proves the mapping-quality and provenance survive (AC3), or a
    /// symbolication state is not covered.
    ProvenancePreservationUnproven,
    /// Governance review does not satisfy required invariants.
    GovernanceReviewIncomplete,
    /// Consumer projection does not satisfy required invariants.
    ConsumerProjectionIncomplete,
    /// Release / support parity posture is incomplete.
    ReleasePostureIncomplete,
    /// Export contains raw sensitive material.
    RawMaterialInExport,
}

impl M5DebugViolation {
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
            Self::SessionModesMissing => "session_modes_missing",
            Self::ControlPosturesMissing => "control_postures_missing",
            Self::DowngradeTriggersMissing => "downgrade_triggers_missing",
            Self::ConsumerSurfacesMissing => "consumer_surfaces_missing",
            Self::ExampleSessionsMissing => "example_sessions_missing",
            Self::ExampleSessionDrift => "example_session_drift",
            Self::SurfaceInvariantViolated => "surface_invariant_violated",
            Self::HierarchyUnderstandingUnproven => "hierarchy_understanding_unproven",
            Self::ControlDistinctionUnproven => "control_distinction_unproven",
            Self::ProvenancePreservationUnproven => "provenance_preservation_unproven",
            Self::GovernanceReviewIncomplete => "governance_review_incomplete",
            Self::ConsumerProjectionIncomplete => "consumer_projection_incomplete",
            Self::ReleasePostureIncomplete => "release_posture_incomplete",
            Self::RawMaterialInExport => "raw_material_in_export",
        }
    }
}

/// Reads and validates the checked-in stable M5 debug export.
///
/// # Errors
///
/// Returns an artifact error if the export cannot parse or fails validation.
pub fn current_stable_m5_debug_hierarchy_export(
) -> Result<M5DebugHierarchyPrimitivePacket, M5DebugArtifactError> {
    let packet: M5DebugHierarchyPrimitivePacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/release/m5-debug-session-hierarchy-primitive-proof/support_export.json"
    )))
    .map_err(M5DebugArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(M5DebugArtifactError::Validation(violations))
    }
}

fn validate_source_contracts(
    packet: &M5DebugHierarchyPrimitivePacket,
    violations: &mut Vec<M5DebugViolation>,
) {
    let refs: BTreeSet<&str> = packet
        .source_contract_refs
        .iter()
        .map(String::as_str)
        .collect();
    for required in [
        M5_DEBUG_HIERARCHY_SCHEMA_REF,
        M5_DEBUG_HIERARCHY_DOC_REF,
        M5_DEBUG_HIERARCHY_COMPONENT_MATRIX_REF,
        M5_DEBUG_HIERARCHY_ARTIFACT_REF,
    ] {
        if !refs.contains(required) {
            violations.push(M5DebugViolation::MissingSourceContracts);
            return;
        }
    }
}

fn validate_vocabulary_set(
    packet: &M5DebugHierarchyPrimitivePacket,
    violations: &mut Vec<M5DebugViolation>,
) {
    if !packet.vocabulary_set.matches_canonical() {
        violations.push(M5DebugViolation::VocabularySetDrift);
    }
}

fn validate_surface_rows(
    packet: &M5DebugHierarchyPrimitivePacket,
    violations: &mut Vec<M5DebugViolation>,
) {
    let present: BTreeSet<M5RunAttemptSurfaceFamily> = packet
        .surface_rows
        .iter()
        .map(|row| row.surface_family)
        .collect();
    for required in M5RunAttemptSurfaceFamily::ALL {
        if !present.contains(&required) {
            violations.push(M5DebugViolation::RequiredSurfaceMissing);
            return;
        }
    }

    for row in &packet.surface_rows {
        if row.owner_role.trim().is_empty()
            || row.scope_summary.trim().is_empty()
            || row.source_contract_refs.is_empty()
        {
            violations.push(M5DebugViolation::SurfaceRowIncomplete);
        }
        if !row.declares_mandatory_export_fields() {
            violations.push(M5DebugViolation::MandatoryExportFieldMissing);
        }
        if row.session_modes.is_empty() {
            violations.push(M5DebugViolation::SessionModesMissing);
        }
        if row.control_postures.is_empty() {
            violations.push(M5DebugViolation::ControlPosturesMissing);
        }
        if row.downgrade_triggers.is_empty() {
            violations.push(M5DebugViolation::DowngradeTriggersMissing);
        }
        if row.consumer_surfaces.is_empty() {
            violations.push(M5DebugViolation::ConsumerSurfacesMissing);
        }
        if row.example_sessions.is_empty() {
            violations.push(M5DebugViolation::ExampleSessionsMissing);
        }
        if row.example_sessions.iter().any(|case| !case.is_self_consistent()) {
            violations.push(M5DebugViolation::ExampleSessionDrift);
        }
        if !row.honours_invariants() {
            violations.push(M5DebugViolation::SurfaceInvariantViolated);
        }
    }
}

/// The acceptance criteria must each be demonstrated across the matrix: the hierarchy
/// stays understandable when narrowed (AC1), live control is distinguished from captured
/// analysis (AC2), and mapping-quality / provenance survive (AC3).
fn validate_acceptance_criteria_covered(
    packet: &M5DebugHierarchyPrimitivePacket,
    violations: &mut Vec<M5DebugViolation>,
) {
    let cases: Vec<&M5ResolvedDebugHierarchy> = packet
        .surface_rows
        .iter()
        .flat_map(|row| row.example_sessions.iter().map(|case| &case.resolved))
        .collect();

    // AC1: at least one narrowed (restored / degraded / inspect-only) case still keeps its
    // hierarchy understandable, every case keeps its hierarchy and identity consistent,
    // and every session mode is covered across the matrix.
    let mut modes_seen: BTreeSet<M5DebugSessionMode> = BTreeSet::new();
    for resolved in &cases {
        modes_seen.insert(resolved.header.session_mode);
    }
    let narrowed_proven = cases.iter().any(|resolved| {
        (resolved.header.restored
            || resolved.degraded.is_some()
            || resolved.header.control_posture == M5DebugControlPosture::InspectOnlyView)
            && resolved.hierarchy_understandable_when_narrowed()
    });
    let hierarchy_proven = narrowed_proven
        && cases.iter().all(|resolved| {
            resolved.hierarchy_understandable_when_narrowed() && resolved.identity_consistent()
        })
        && DEBUG_SESSION_MODE_ALL
            .iter()
            .all(|mode| modes_seen.contains(mode));
    if !hierarchy_proven {
        violations.push(M5DebugViolation::HierarchyUnderstandingUnproven);
    }

    // AC2: at least one live-control case and at least one captured case, every case
    // distinguishes control, and every control posture is covered across the matrix.
    let mut postures_seen: BTreeSet<M5DebugControlPosture> = BTreeSet::new();
    for resolved in &cases {
        postures_seen.insert(resolved.header.control_posture);
    }
    let control_proven = cases
        .iter()
        .any(|resolved| resolved.header.is_live_control)
        && cases.iter().any(|resolved| !resolved.header.is_live_control)
        && cases.iter().all(|resolved| resolved.distinguishes_control())
        && M5DebugControlPosture::ALL
            .iter()
            .all(|posture| postures_seen.contains(posture));
    if !control_proven {
        violations.push(M5DebugViolation::ControlDistinctionUnproven);
    }

    // AC3: at least one case carries a dump card, every case preserves mapping-quality and
    // provenance, and every symbolication state is covered across the dump cards.
    let mut symbolication_seen: BTreeSet<M5SymbolicationState> = BTreeSet::new();
    for resolved in &cases {
        for card in &resolved.dump_cards {
            symbolication_seen.insert(card.symbolication);
        }
    }
    let provenance_proven = cases
        .iter()
        .any(|resolved| !resolved.dump_cards.is_empty())
        && cases
            .iter()
            .all(|resolved| resolved.preserves_mapping_and_provenance())
        && SYMBOLICATION_ALL
            .iter()
            .all(|state| symbolication_seen.contains(state));
    if !provenance_proven {
        violations.push(M5DebugViolation::ProvenancePreservationUnproven);
    }
}

fn validate_governance_review(
    packet: &M5DebugHierarchyPrimitivePacket,
    violations: &mut Vec<M5DebugViolation>,
) {
    let review = &packet.governance_review;
    for ok in [
        review.one_primitive_carries_all_surfaces,
        review.live_control_never_blurs_with_captured,
        review.hierarchy_never_flattened,
        review.provenance_and_symbolication_preserved,
        review.dump_cards_never_imply_live_control,
        review.support_export_reconstructs_debug_hierarchy,
        review.later_rows_cannot_invent_parallel_vocabulary,
    ] {
        if !ok {
            violations.push(M5DebugViolation::GovernanceReviewIncomplete);
            return;
        }
    }
}

fn validate_consumer_projection(
    packet: &M5DebugHierarchyPrimitivePacket,
    violations: &mut Vec<M5DebugViolation>,
) {
    let projection = &packet.consumer_projection;
    for ok in [
        projection.execution_surfaces_consume_shared_primitive,
        projection.resolver_reads_single_model,
        projection.tree_rows_read_single_hierarchy_source,
        projection.support_export_reads_single_source,
    ] {
        if !ok {
            violations.push(M5DebugViolation::ConsumerProjectionIncomplete);
            return;
        }
    }
}

fn validate_release_posture(
    packet: &M5DebugHierarchyPrimitivePacket,
    violations: &mut Vec<M5DebugViolation>,
) {
    let posture = &packet.release_posture;
    if posture.release_packet_ref.trim().is_empty()
        || posture.debug_audit_ref.trim().is_empty()
        || !posture.support_export_parity_required
        || !posture.accessibility_parity_required
    {
        violations.push(M5DebugViolation::ReleasePostureIncomplete);
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
