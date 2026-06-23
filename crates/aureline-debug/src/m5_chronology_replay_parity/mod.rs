//! Typed chronology-capability descriptors, replay sessions, timeline bookmarks,
//! notebook-kernel capability descriptors, cell-frame links, and restart/reconnect
//! consequence records: the canonical M5 records every live-debug, replay, notebook,
//! profiler, AI, and support surface reads to speak about *what time-travel and
//! notebook-debug a backend actually supports*, *what a replay session reconstructed and
//! from which capture*, *where a timeline bookmark is pinned*, and *what a restart or
//! reconnect preserved, lost, invalidated, or left stale* — without re-expressing
//! debugger folklore ad hoc.
//!
//! The [`m5_debug_contracts`](crate::m5_debug_contracts) matrix *names* the debugger
//! object families and freezes their vocabulary; the
//! [`m5_debug_session_descriptors`](crate::m5_debug_session_descriptors),
//! [`m5_breakpoint_specs`](crate::m5_breakpoint_specs),
//! [`m5_frame_variable_snapshots`](crate::m5_frame_variable_snapshots), and
//! [`m5_evaluate_repl_sheets`](crate::m5_evaluate_repl_sheets) lanes materialize the
//! session, attach-target, breakpoint, frame-mapping, variable/watch, evaluate, and
//! console families. This lane *materializes* the
//! [`DebugObjectClass::ChronologyCapability`](crate::m5_debug_contracts::DebugObjectClass::ChronologyCapability),
//! [`DebugObjectClass::ReplaySession`](crate::m5_debug_contracts::DebugObjectClass::ReplaySession),
//! and
//! [`DebugObjectClass::NotebookDebugParity`](crate::m5_debug_contracts::DebugObjectClass::NotebookDebugParity)
//! families as concrete, serde-serializable records and freezes a canonical
//! [`ChronologyReplayParitySet`].
//!
//! Chronology, replay, and notebook-debug truth stays explicit, replay-safe, and
//! per-backend:
//!
//! - **One support-class vocabulary everywhere.** Every descriptor carries one
//!   [`DebugSupportClass`] (`supported`, `limited`, `unavailable`, `policy_blocked`) shared
//!   across live debug, replay, notebook bridge, presentation, and support export, so a
//!   surface never invents a private support label.
//! - **No inherited claims across backends.** Each capability descriptor derives its
//!   [`CapabilitySupportPill`] only from its *own* support class and [`TimelineState`]; an
//!   `unavailable` or `policy_blocked` backend supports zero verbs and grants no replay or
//!   notebook-debug claim, so an unsupported runtime never inherits a neighbor's chronology.
//! - **Replay is inspect-only and capture-bound.** A [`ReplaySession`] is always
//!   inspect-only and names the [`CaptureIdentity`] it reconstructs; a [`TimelineBookmark`]
//!   is bound to exactly one capture/session/target identity and survives support export and
//!   restore review.
//! - **Restart/reconnect consequences are itemized, never flattened.** A
//!   [`RestartConsequenceRecord`] names, per subject (variables, queued cells, debug state,
//!   breakpoints, transient outputs), whether it was [`Preserved`](ConsequenceDisposition::Preserved),
//!   [`Lost`](ConsequenceDisposition::Lost), [`Invalidated`](ConsequenceDisposition::Invalidated),
//!   or [`Stale`](ConsequenceDisposition::Stale) — so the product can explain a restart or
//!   reconnect rather than collapsing it into a generic banner.
//! - **A degraded link never poses as exact.** A [`CellFrameLink`] renders an exact
//!   frame-to-cell link only when its [`CellLinkFidelity`] is exact and its support class
//!   permits use; an approximate, stale, or unmapped link is never drawn exact.
//!
//! [`m5_chronology_replay_parity_set`] is the canonical binding: it builds the set
//! deterministically and computes each [`ParityInvariant`]'s `holds` flag from the built
//! records, so the checked-in fixture and the freeze gate freeze the contract byte-for-byte
//! and an inconsistent edit flips an invariant and fails CI. The record carries no raw
//! capture bodies, value bodies, raw paths, provider payloads, URLs, hostnames, or
//! credentials — only opaque object refs, stable tokens, opaque digests, and short
//! reviewable sentences — so it is safe for support export.
//!
//! The cross-tool boundary schema is at
//! [`/schemas/debug/m5_chronology_replay_parity.schema.json`](../../../schemas/debug/m5_chronology_replay_parity.schema.json).
//! The checked-in stable packet is at
//! [`/fixtures/debug/m5_chronology_replay_parity/canonical_set.json`](../../../fixtures/debug/m5_chronology_replay_parity/canonical_set.json).
//! The reviewer-facing contract is at
//! [`/docs/debug/m5_chronology_replay_parity.md`](../../../docs/debug/m5_chronology_replay_parity.md).

use serde::{Deserialize, Serialize};

use crate::m5_debug_contracts::DebugConsumer;

#[cfg(test)]
mod tests;

/// Schema version for the M5 chronology/replay/parity set.
pub const M5_CHRONOLOGY_REPLAY_PARITY_SCHEMA_VERSION: u32 = 1;

/// Schema reference for the M5 chronology/replay/parity set.
pub const M5_CHRONOLOGY_REPLAY_PARITY_SCHEMA_REF: &str =
    "schemas/debug/m5_chronology_replay_parity.schema.json";

/// Stable record-kind tag for the chronology/replay/parity set.
pub const M5_CHRONOLOGY_REPLAY_PARITY_RECORD_KIND: &str = "m5_chronology_replay_parity_set";

/// Stable id for the canonical chronology/replay/parity set.
pub const M5_CHRONOLOGY_REPLAY_PARITY_SET_ID: &str = "m5-chronology-replay-parity:set:0001";

/// Evaluation stamp for the canonical set. Held as a constant so the canonical binding
/// stays deterministic and the fixture freezes byte-for-byte.
pub const M5_CHRONOLOGY_REPLAY_PARITY_AS_OF: &str = "2026-06-23T00:00:00Z";

/// The freeze gate that keeps the chronology/replay/parity set current. Stable promotion
/// runs this gate; it fails when the in-code set drifts from the checked-in fixture or any
/// invariant flips.
pub const M5_CHRONOLOGY_REPLAY_PARITY_FREEZE_GATE_REF: &str =
    "crates/aureline-debug/tests/m5_chronology_replay_parity.rs";

/// The checked-in canonical chronology/replay/parity fixture.
pub const M5_CHRONOLOGY_REPLAY_PARITY_FIXTURE_REF: &str =
    "fixtures/debug/m5_chronology_replay_parity/canonical_set.json";

/// The contract narrative document.
pub const M5_CHRONOLOGY_REPLAY_PARITY_DOC_REF: &str = "docs/debug/m5_chronology_replay_parity.md";

/// The human-readable evidence companion artifact.
pub const M5_CHRONOLOGY_REPLAY_PARITY_ARTIFACT_REF: &str =
    "artifacts/debug/m5_chronology_replay_parity.md";

// ---------------------------------------------------------------------------
// Backend / runtime / toolchain family.
// ---------------------------------------------------------------------------

/// The backend / runtime / toolchain family a debug, replay, or notebook capability
/// belongs to. Each descriptor pins its own family so a chronology or notebook-debug claim
/// is never inherited from a neighboring backend.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RuntimeBackendFamily {
    /// A local native process debugged directly on the host.
    LocalNative,
    /// A process reached through a remote debug helper / agent.
    RemoteHelper,
    /// A process inside a container or sandbox.
    Container,
    /// A managed cloud / hosted runtime.
    ManagedRuntime,
    /// A browser / web runtime.
    BrowserRuntime,
    /// A notebook kernel reached through the notebook debug bridge.
    NotebookKernel,
}

impl RuntimeBackendFamily {
    /// All backend families, in canonical order.
    pub const ALL: [Self; 6] = [
        Self::LocalNative,
        Self::RemoteHelper,
        Self::Container,
        Self::ManagedRuntime,
        Self::BrowserRuntime,
        Self::NotebookKernel,
    ];

    /// Stable snake_case token for this family.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LocalNative => "local_native",
            Self::RemoteHelper => "remote_helper",
            Self::Container => "container",
            Self::ManagedRuntime => "managed_runtime",
            Self::BrowserRuntime => "browser_runtime",
            Self::NotebookKernel => "notebook_kernel",
        }
    }

    /// Human-readable label.
    pub const fn label(self) -> &'static str {
        match self {
            Self::LocalNative => "Local native",
            Self::RemoteHelper => "Remote helper",
            Self::Container => "Container",
            Self::ManagedRuntime => "Managed runtime",
            Self::BrowserRuntime => "Browser runtime",
            Self::NotebookKernel => "Notebook kernel",
        }
    }
}

// ---------------------------------------------------------------------------
// Support class.
// ---------------------------------------------------------------------------

/// The shared support-class vocabulary every chronology, replay, notebook-bridge, and
/// support-export surface reads. This is the one vocabulary the spec pins for the lane:
/// `supported`, `limited`, `unavailable`, `policy_blocked`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DebugSupportClass {
    /// Fully supported with no caveat.
    Supported,
    /// Usable, but with a disclosed limitation (a subset of verbs, partial history, etc.).
    Limited,
    /// Not available on this backend; the gap is named, not silently dropped.
    Unavailable,
    /// Blocked by an explicit policy rule rather than a technical gap.
    PolicyBlocked,
}

impl DebugSupportClass {
    /// All support classes, in canonical order.
    pub const ALL: [Self; 4] = [
        Self::Supported,
        Self::Limited,
        Self::Unavailable,
        Self::PolicyBlocked,
    ];

    /// Stable snake_case token for this support class.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Supported => "supported",
            Self::Limited => "limited",
            Self::Unavailable => "unavailable",
            Self::PolicyBlocked => "policy_blocked",
        }
    }

    /// Human-readable label.
    pub const fn label(self) -> &'static str {
        match self {
            Self::Supported => "Supported",
            Self::Limited => "Limited",
            Self::Unavailable => "Unavailable",
            Self::PolicyBlocked => "Policy-blocked",
        }
    }

    /// Whether this class permits using the capability at all. Supported and limited do.
    pub const fn permits_use(self) -> bool {
        matches!(self, Self::Supported | Self::Limited)
    }

    /// Whether this class is full, caveat-free support.
    pub const fn is_full_support(self) -> bool {
        matches!(self, Self::Supported)
    }

    /// Whether this class is inert — it carries no verbs and grants no capability.
    pub const fn is_inert(self) -> bool {
        matches!(self, Self::Unavailable | Self::PolicyBlocked)
    }
}

// ---------------------------------------------------------------------------
// Timeline state.
// ---------------------------------------------------------------------------

/// The chronology / replay timeline state a capability or session is in. Drives whether
/// time-travel verbs are backed and whether a caveat must be disclosed.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TimelineState {
    /// A live session with no chronology recording.
    LiveNoRecording,
    /// Currently capturing a timeline.
    Recording,
    /// A complete recorded timeline is available.
    RecordedComplete,
    /// A partial recorded timeline (bounded window or since-attach) is available.
    RecordedPartial,
    /// A recorded capture is actively being replayed.
    ReplayActive,
    /// The capture expired or was evicted; no timeline remains.
    Expired,
    /// The capture no longer matches the current build/artifact identity.
    Mismatched,
    /// No timeline is or can be present on this backend.
    Unavailable,
}

impl TimelineState {
    /// All timeline states, in canonical order.
    pub const ALL: [Self; 8] = [
        Self::LiveNoRecording,
        Self::Recording,
        Self::RecordedComplete,
        Self::RecordedPartial,
        Self::ReplayActive,
        Self::Expired,
        Self::Mismatched,
        Self::Unavailable,
    ];

    /// Stable snake_case token for this timeline state.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LiveNoRecording => "live_no_recording",
            Self::Recording => "recording",
            Self::RecordedComplete => "recorded_complete",
            Self::RecordedPartial => "recorded_partial",
            Self::ReplayActive => "replay_active",
            Self::Expired => "expired",
            Self::Mismatched => "mismatched",
            Self::Unavailable => "unavailable",
        }
    }

    /// Human-readable label.
    pub const fn label(self) -> &'static str {
        match self {
            Self::LiveNoRecording => "Live (not recording)",
            Self::Recording => "Recording",
            Self::RecordedComplete => "Recorded (complete)",
            Self::RecordedPartial => "Recorded (partial)",
            Self::ReplayActive => "Replaying",
            Self::Expired => "Capture expired",
            Self::Mismatched => "Capture mismatched",
            Self::Unavailable => "Unavailable",
        }
    }

    /// Whether a recorded / replayable timeline backs time-travel in this state.
    pub const fn permits_time_travel(self) -> bool {
        matches!(
            self,
            Self::Recording | Self::RecordedComplete | Self::RecordedPartial | Self::ReplayActive
        )
    }

    /// Whether this state must render with a visible caveat.
    pub const fn requires_disclosure(self) -> bool {
        matches!(
            self,
            Self::RecordedPartial | Self::Expired | Self::Mismatched | Self::Unavailable
        )
    }

    /// Whether this state is an inspect-only replay of a recording.
    pub const fn is_replay(self) -> bool {
        matches!(self, Self::ReplayActive)
    }
}

// ---------------------------------------------------------------------------
// Capability verbs.
// ---------------------------------------------------------------------------

/// One debug / chronology / notebook capability verb. The single shared verb vocabulary so
/// chronology, replay, and notebook-debug descriptors name the same actions, and a verb is
/// listed only when its descriptor's support class and timeline back it.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CapabilityVerb {
    /// Set a breakpoint.
    SetBreakpoint,
    /// Step forward (over / into / out).
    Step,
    /// Continue execution.
    Continue,
    /// Step backward in a recorded timeline.
    ReverseStep,
    /// Continue backward to the previous stop in a recorded timeline.
    ReverseContinue,
    /// Jump to a recorded event in the timeline.
    JumpToEvent,
    /// Set a timeline bookmark.
    SetBookmark,
    /// Jump to a timeline bookmark.
    JumpToBookmark,
    /// Evaluate an expression.
    Evaluate,
    /// Inspect variables / scopes.
    InspectVariables,
    /// Inspect a historical (past) frame in a recorded timeline.
    InspectHistoricalFrame,
}

impl CapabilityVerb {
    /// All verbs, in canonical order.
    pub const ALL: [Self; 11] = [
        Self::SetBreakpoint,
        Self::Step,
        Self::Continue,
        Self::ReverseStep,
        Self::ReverseContinue,
        Self::JumpToEvent,
        Self::SetBookmark,
        Self::JumpToBookmark,
        Self::Evaluate,
        Self::InspectVariables,
        Self::InspectHistoricalFrame,
    ];

    /// Stable snake_case token for this verb.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SetBreakpoint => "set_breakpoint",
            Self::Step => "step",
            Self::Continue => "continue",
            Self::ReverseStep => "reverse_step",
            Self::ReverseContinue => "reverse_continue",
            Self::JumpToEvent => "jump_to_event",
            Self::SetBookmark => "set_bookmark",
            Self::JumpToBookmark => "jump_to_bookmark",
            Self::Evaluate => "evaluate",
            Self::InspectVariables => "inspect_variables",
            Self::InspectHistoricalFrame => "inspect_historical_frame",
        }
    }

    /// Whether this verb requires a recorded / replayable timeline to back it.
    pub const fn requires_time_travel(self) -> bool {
        matches!(
            self,
            Self::ReverseStep
                | Self::ReverseContinue
                | Self::JumpToEvent
                | Self::SetBookmark
                | Self::JumpToBookmark
                | Self::InspectHistoricalFrame
        )
    }
}

// ---------------------------------------------------------------------------
// Recorded scope.
// ---------------------------------------------------------------------------

/// What span of execution a chronology capture covers.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RecordedScope {
    /// The full session from start.
    FullSession,
    /// Everything since the debugger attached.
    SinceAttach,
    /// A bounded rolling window.
    BoundedWindow,
    /// Nothing is recorded.
    None,
}

impl RecordedScope {
    /// All recorded scopes, in canonical order.
    pub const ALL: [Self; 4] = [
        Self::FullSession,
        Self::SinceAttach,
        Self::BoundedWindow,
        Self::None,
    ];

    /// Stable snake_case token for this scope.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::FullSession => "full_session",
            Self::SinceAttach => "since_attach",
            Self::BoundedWindow => "bounded_window",
            Self::None => "none",
        }
    }

    /// Human-readable label.
    pub const fn label(self) -> &'static str {
        match self {
            Self::FullSession => "Full session",
            Self::SinceAttach => "Since attach",
            Self::BoundedWindow => "Bounded window",
            Self::None => "None",
        }
    }

    /// Whether this scope records any history.
    pub const fn records_history(self) -> bool {
        !matches!(self, Self::None)
    }
}

// ---------------------------------------------------------------------------
// Notebook parity.
// ---------------------------------------------------------------------------

/// Whether a chronology / replay capability is mirrored on the notebook debug surface.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum NotebookParityClass {
    /// The capability is mirrored in the notebook debug surface.
    Mirrored,
    /// The notebook surface offers a disclosed-divergent subset.
    Divergent,
    /// No notebook-debug parity exists for this backend.
    Unsupported,
    /// Notebook parity does not apply to this backend.
    NotApplicable,
}

impl NotebookParityClass {
    /// All parity classes, in canonical order.
    pub const ALL: [Self; 4] = [
        Self::Mirrored,
        Self::Divergent,
        Self::Unsupported,
        Self::NotApplicable,
    ];

    /// Stable snake_case token for this parity class.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Mirrored => "mirrored",
            Self::Divergent => "divergent",
            Self::Unsupported => "unsupported",
            Self::NotApplicable => "not_applicable",
        }
    }

    /// Human-readable label.
    pub const fn label(self) -> &'static str {
        match self {
            Self::Mirrored => "Mirrored in notebook",
            Self::Divergent => "Divergent in notebook",
            Self::Unsupported => "No notebook parity",
            Self::NotApplicable => "Notebook parity N/A",
        }
    }
}

// ---------------------------------------------------------------------------
// Capability support pill.
// ---------------------------------------------------------------------------

/// The single canonical support pill every chronology, replay, and notebook-kernel
/// descriptor renders — one support class, one timeline state, with every capability flag
/// derived from the descriptor's own truth.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CapabilitySupportPill {
    /// The support class.
    pub support_class: DebugSupportClass,
    /// Stable token for the support class.
    pub support_class_token: String,
    /// The timeline state.
    pub timeline_state: TimelineState,
    /// Stable token for the timeline state.
    pub timeline_state_token: String,
    /// One reviewable label combining support class and timeline state.
    pub label: String,
    /// Whether the capability is usable at all (support class permits use).
    pub permits_use: bool,
    /// Whether time-travel verbs are backed (permits use and the timeline supports it).
    pub time_travel_available: bool,
    /// Whether this descriptor must render with a visible caveat.
    pub requires_disclosure: bool,
    /// Whether this descriptor is inert — no verbs, no capability.
    pub is_inert: bool,
    /// Whether the timeline is an inspect-only replay of a recording.
    pub is_inspect_only_timeline: bool,
}

impl CapabilitySupportPill {
    /// Builds the canonical support pill, deriving every flag from the support class and
    /// timeline state so the pill cannot disagree with itself.
    pub fn derive(support_class: DebugSupportClass, timeline_state: TimelineState) -> Self {
        let permits_use = support_class.permits_use();
        let time_travel_available = permits_use && timeline_state.permits_time_travel();
        let requires_disclosure =
            !support_class.is_full_support() || timeline_state.requires_disclosure();
        let is_inert = support_class.is_inert();
        let is_inspect_only_timeline = timeline_state.is_replay();

        let label = format!("{} · {}", support_class.label(), timeline_state.label());

        Self {
            support_class,
            support_class_token: support_class.as_str().to_owned(),
            timeline_state,
            timeline_state_token: timeline_state.as_str().to_owned(),
            label,
            permits_use,
            time_travel_available,
            requires_disclosure,
            is_inert,
            is_inspect_only_timeline,
        }
    }

    /// Whether this pill equals the canonical derivation for the given inputs.
    pub fn matches_derivation(
        &self,
        support_class: DebugSupportClass,
        timeline_state: TimelineState,
    ) -> bool {
        *self == Self::derive(support_class, timeline_state)
    }
}

// ---------------------------------------------------------------------------
// Capture identity.
// ---------------------------------------------------------------------------

/// The identity a replay session reconstructs and a timeline bookmark binds to: one
/// capture, one originating session, one target, and the optional exact-build artifact ref.
/// No raw paths cross this boundary.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CaptureIdentity {
    /// Stable capture id.
    pub capture_id: String,
    /// Stable session id the capture was recorded from.
    pub session_id: String,
    /// Stable target id the session ran against.
    pub target_id: String,
    /// Opaque exact-build artifact ref the capture was recorded against, when one is bound.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub build_artifact_ref: Option<String>,
}

impl CaptureIdentity {
    /// Builds a capture identity.
    pub fn build(
        capture_id: impl Into<String>,
        session_id: impl Into<String>,
        target_id: impl Into<String>,
        build_artifact_ref: Option<&str>,
    ) -> Self {
        Self {
            capture_id: capture_id.into(),
            session_id: session_id.into(),
            target_id: target_id.into(),
            build_artifact_ref: build_artifact_ref.map(str::to_owned),
        }
    }

    /// Whether the identity is fully bound (capture, session, and target are all present).
    pub fn is_fully_bound(&self) -> bool {
        !self.capture_id.is_empty() && !self.session_id.is_empty() && !self.target_id.is_empty()
    }

    /// Whether two identities name the same capture, session, and target.
    pub fn same_as(&self, other: &Self) -> bool {
        self.capture_id == other.capture_id
            && self.session_id == other.session_id
            && self.target_id == other.target_id
    }
}

// ---------------------------------------------------------------------------
// Restart / reconnect consequence.
// ---------------------------------------------------------------------------

/// What triggered a restart / reconnect consequence record.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ConsequenceTrigger {
    /// A debug session was restarted.
    SessionRestart,
    /// A debug session reconnected after a transport drop.
    Reconnect,
    /// A notebook kernel was restarted.
    KernelRestart,
    /// A notebook kernel's transport was lost and a reconnect was attempted.
    TransportLostReconnect,
    /// A replay capture was reacquired / reloaded.
    ReplayReacquire,
}

impl ConsequenceTrigger {
    /// All triggers, in canonical order.
    pub const ALL: [Self; 5] = [
        Self::SessionRestart,
        Self::Reconnect,
        Self::KernelRestart,
        Self::TransportLostReconnect,
        Self::ReplayReacquire,
    ];

    /// Stable snake_case token for this trigger.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SessionRestart => "session_restart",
            Self::Reconnect => "reconnect",
            Self::KernelRestart => "kernel_restart",
            Self::TransportLostReconnect => "transport_lost_reconnect",
            Self::ReplayReacquire => "replay_reacquire",
        }
    }

    /// Human-readable label.
    pub const fn label(self) -> &'static str {
        match self {
            Self::SessionRestart => "Session restart",
            Self::Reconnect => "Reconnect",
            Self::KernelRestart => "Kernel restart",
            Self::TransportLostReconnect => "Transport lost, reconnect attempted",
            Self::ReplayReacquire => "Replay capture reacquired",
        }
    }
}

/// The subject of one restart/reconnect consequence entry. The five subjects every
/// restart/reconnect surface must itemize, so consequences are never flattened into one
/// banner.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ConsequenceSubject {
    /// Variable / scope state.
    Variables,
    /// Queued / pending notebook cells.
    QueuedCells,
    /// Debugger / bridge state.
    DebugState,
    /// Breakpoints.
    Breakpoints,
    /// Transient outputs (console / stdout / stream output).
    TransientOutputs,
}

impl ConsequenceSubject {
    /// All subjects, in canonical order. These are the subjects a restart/reconnect surface
    /// must explain, not collapse.
    pub const ALL: [Self; 5] = [
        Self::Variables,
        Self::QueuedCells,
        Self::DebugState,
        Self::Breakpoints,
        Self::TransientOutputs,
    ];

    /// Stable snake_case token for this subject.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Variables => "variables",
            Self::QueuedCells => "queued_cells",
            Self::DebugState => "debug_state",
            Self::Breakpoints => "breakpoints",
            Self::TransientOutputs => "transient_outputs",
        }
    }

    /// Human-readable label.
    pub const fn label(self) -> &'static str {
        match self {
            Self::Variables => "Variables",
            Self::QueuedCells => "Queued cells",
            Self::DebugState => "Debug state",
            Self::Breakpoints => "Breakpoints",
            Self::TransientOutputs => "Transient outputs",
        }
    }
}

/// What happened to a subject across a restart / reconnect.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ConsequenceDisposition {
    /// Carried across intact.
    Preserved,
    /// Gone — must be re-established.
    Lost,
    /// Still present but no longer valid; must be discarded.
    Invalidated,
    /// Still shown but no longer current; flagged stale.
    Stale,
}

impl ConsequenceDisposition {
    /// All dispositions, in canonical order.
    pub const ALL: [Self; 4] = [Self::Preserved, Self::Lost, Self::Invalidated, Self::Stale];

    /// Stable snake_case token for this disposition.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Preserved => "preserved",
            Self::Lost => "lost",
            Self::Invalidated => "invalidated",
            Self::Stale => "stale",
        }
    }

    /// Human-readable label.
    pub const fn label(self) -> &'static str {
        match self {
            Self::Preserved => "Preserved",
            Self::Lost => "Lost",
            Self::Invalidated => "Invalidated",
            Self::Stale => "Stale",
        }
    }

    /// Whether the subject survived the restart/reconnect intact.
    pub const fn survived(self) -> bool {
        matches!(self, Self::Preserved)
    }
}

/// One itemized restart/reconnect consequence: what happened to one subject.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ConsequenceEntry {
    /// The subject of this entry.
    pub subject: ConsequenceSubject,
    /// Stable token for the subject.
    pub subject_token: String,
    /// What happened to the subject.
    pub disposition: ConsequenceDisposition,
    /// Stable token for the disposition.
    pub disposition_token: String,
    /// One reviewable export-safe sentence describing the outcome.
    pub detail: String,
}

impl ConsequenceEntry {
    /// Builds a consequence entry, deriving the subject and disposition tokens.
    pub fn build(
        subject: ConsequenceSubject,
        disposition: ConsequenceDisposition,
        detail: impl Into<String>,
    ) -> Self {
        Self {
            subject,
            subject_token: subject.as_str().to_owned(),
            disposition,
            disposition_token: disposition.as_str().to_owned(),
            detail: detail.into(),
        }
    }

    /// Whether the carried tokens agree with their enums.
    pub fn is_consistent(&self) -> bool {
        self.subject_token == self.subject.as_str()
            && self.disposition_token == self.disposition.as_str()
    }
}

/// A typed restart/reconnect consequence record: the canonical record every notebook,
/// debug, and replay surface reads to explain — per subject — what a restart or reconnect
/// preserved, lost, invalidated, or left stale. It never flattens those outcomes into one
/// generic banner.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RestartConsequenceRecord {
    /// Stable, namespaced consequence id.
    pub consequence_id: String,
    /// What triggered the consequence.
    pub trigger: ConsequenceTrigger,
    /// Stable token for the trigger.
    pub trigger_token: String,
    /// Stable session / kernel / replay ref the consequence applies to.
    pub session_ref: String,
    /// The itemized per-subject outcomes.
    pub entries: Vec<ConsequenceEntry>,
    /// The proof packet that keeps this record current.
    pub proof_packet_ref: String,
    /// One reviewable export-safe sentence summarizing the consequence.
    pub summary: String,
}

impl RestartConsequenceRecord {
    /// Builds a restart consequence record, deriving the trigger token.
    pub fn build(
        consequence_id: impl Into<String>,
        trigger: ConsequenceTrigger,
        session_ref: impl Into<String>,
        entries: Vec<ConsequenceEntry>,
        proof_packet_ref: impl Into<String>,
        summary: impl Into<String>,
    ) -> Self {
        Self {
            consequence_id: consequence_id.into(),
            trigger,
            trigger_token: trigger.as_str().to_owned(),
            session_ref: session_ref.into(),
            entries,
            proof_packet_ref: proof_packet_ref.into(),
            summary: summary.into(),
        }
    }

    /// The disposition recorded for a subject, if itemized.
    pub fn disposition_for(&self, subject: ConsequenceSubject) -> Option<ConsequenceDisposition> {
        self.entries
            .iter()
            .find(|e| e.subject == subject)
            .map(|e| e.disposition)
    }

    /// Whether every required subject is itemized exactly once.
    pub fn itemizes_every_subject(&self) -> bool {
        ConsequenceSubject::ALL.iter().all(|subject| {
            self.entries
                .iter()
                .filter(|e| e.subject == *subject)
                .count()
                == 1
        })
    }
}

// ---------------------------------------------------------------------------
// Chronology capability descriptor.
// ---------------------------------------------------------------------------

/// A typed chronology-capability descriptor: the canonical record every live-debug,
/// replay, notebook, profiler, AI, and support surface reads to know *what time-travel a
/// backend actually supports* — its backend family, support class, timeline state, the
/// verbs it backs, what scope it records, and whether the capability is mirrored in the
/// notebook surface.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ChronologyCapabilityDescriptor {
    /// Stable, namespaced descriptor id.
    pub descriptor_id: String,
    /// The backend / runtime / toolchain family.
    pub backend_family: RuntimeBackendFamily,
    /// Stable token for the backend family.
    pub backend_family_token: String,
    /// The support class.
    pub support_class: DebugSupportClass,
    /// Stable token for the support class.
    pub support_class_token: String,
    /// The timeline state.
    pub timeline_state: TimelineState,
    /// Stable token for the timeline state.
    pub timeline_state_token: String,
    /// The canonical support pill every surface renders.
    pub support_pill: CapabilitySupportPill,
    /// The chronology / time-travel verbs this descriptor backs.
    pub supported_verbs: Vec<CapabilityVerb>,
    /// The scope of execution recorded.
    pub recorded_scope: RecordedScope,
    /// Stable token for the recorded scope.
    pub recorded_scope_token: String,
    /// Whether the chronology capability is mirrored in the notebook surface.
    pub notebook_parity: NotebookParityClass,
    /// Stable token for the notebook parity class.
    pub notebook_parity_token: String,
    /// Stable session ref this capability belongs to.
    pub session_ref: String,
    /// Opaque capture ref, when a capture is bound.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub capture_ref: Option<String>,
    /// The proof packet that keeps this descriptor current.
    pub proof_packet_ref: String,
    /// One reviewable export-safe sentence describing the descriptor.
    pub summary: String,
}

impl ChronologyCapabilityDescriptor {
    /// Builds a chronology capability descriptor, deriving every computed token and the
    /// support pill so the descriptor cannot disagree with itself.
    #[allow(clippy::too_many_arguments)]
    pub fn build(
        descriptor_id: impl Into<String>,
        backend_family: RuntimeBackendFamily,
        support_class: DebugSupportClass,
        timeline_state: TimelineState,
        supported_verbs: Vec<CapabilityVerb>,
        recorded_scope: RecordedScope,
        notebook_parity: NotebookParityClass,
        session_ref: impl Into<String>,
        capture_ref: Option<&str>,
        proof_packet_ref: impl Into<String>,
        summary: impl Into<String>,
    ) -> Self {
        Self {
            descriptor_id: descriptor_id.into(),
            backend_family,
            backend_family_token: backend_family.as_str().to_owned(),
            support_class,
            support_class_token: support_class.as_str().to_owned(),
            timeline_state,
            timeline_state_token: timeline_state.as_str().to_owned(),
            support_pill: CapabilitySupportPill::derive(support_class, timeline_state),
            supported_verbs,
            recorded_scope,
            recorded_scope_token: recorded_scope.as_str().to_owned(),
            notebook_parity,
            notebook_parity_token: notebook_parity.as_str().to_owned(),
            session_ref: session_ref.into(),
            capture_ref: capture_ref.map(str::to_owned),
            proof_packet_ref: proof_packet_ref.into(),
            summary: summary.into(),
        }
    }

    /// Whether this descriptor backs the given verb.
    pub fn backs(&self, verb: CapabilityVerb) -> bool {
        self.supported_verbs.contains(&verb)
    }
}

// ---------------------------------------------------------------------------
// Replay session.
// ---------------------------------------------------------------------------

/// A typed replay session: an inspect-only session reconstructed from a recorded capture.
/// The canonical record every replay workspace, incident review, notebook, AI, and support
/// surface reads to know which capture it reconstructed, what it can do, and what a
/// reconnect / reacquire preserved or lost.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReplaySession {
    /// Stable, namespaced replay-session id.
    pub replay_session_id: String,
    /// The backend / runtime / toolchain family the capture was recorded from.
    pub backend_family: RuntimeBackendFamily,
    /// Stable token for the backend family.
    pub backend_family_token: String,
    /// The support class of the replay.
    pub support_class: DebugSupportClass,
    /// Stable token for the support class.
    pub support_class_token: String,
    /// The timeline state of the replay.
    pub timeline_state: TimelineState,
    /// Stable token for the timeline state.
    pub timeline_state_token: String,
    /// The canonical support pill every surface renders.
    pub support_pill: CapabilitySupportPill,
    /// The replay verbs this session backs.
    pub supported_verbs: Vec<CapabilityVerb>,
    /// The capture identity this session reconstructs.
    pub capture: CaptureIdentity,
    /// Whether the session is inspect-only (always true for a replay).
    pub inspect_only: bool,
    /// The chronology descriptor this replay is sourced from.
    pub source_chronology_ref: String,
    /// The restart/reconnect consequence record for this replay, when one applies.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub restart_consequence_ref: Option<String>,
    /// Whether the chronology capability is mirrored in the notebook surface.
    pub notebook_parity: NotebookParityClass,
    /// Stable token for the notebook parity class.
    pub notebook_parity_token: String,
    /// The proof packet that keeps this session current.
    pub proof_packet_ref: String,
    /// One reviewable export-safe sentence describing the session.
    pub summary: String,
}

impl ReplaySession {
    /// Builds a replay session, deriving every computed token and the support pill. A replay
    /// session is always inspect-only.
    #[allow(clippy::too_many_arguments)]
    pub fn build(
        replay_session_id: impl Into<String>,
        backend_family: RuntimeBackendFamily,
        support_class: DebugSupportClass,
        timeline_state: TimelineState,
        supported_verbs: Vec<CapabilityVerb>,
        capture: CaptureIdentity,
        source_chronology_ref: impl Into<String>,
        restart_consequence_ref: Option<&str>,
        notebook_parity: NotebookParityClass,
        proof_packet_ref: impl Into<String>,
        summary: impl Into<String>,
    ) -> Self {
        Self {
            replay_session_id: replay_session_id.into(),
            backend_family,
            backend_family_token: backend_family.as_str().to_owned(),
            support_class,
            support_class_token: support_class.as_str().to_owned(),
            timeline_state,
            timeline_state_token: timeline_state.as_str().to_owned(),
            support_pill: CapabilitySupportPill::derive(support_class, timeline_state),
            supported_verbs,
            capture,
            inspect_only: true,
            source_chronology_ref: source_chronology_ref.into(),
            restart_consequence_ref: restart_consequence_ref.map(str::to_owned),
            notebook_parity,
            notebook_parity_token: notebook_parity.as_str().to_owned(),
            proof_packet_ref: proof_packet_ref.into(),
            summary: summary.into(),
        }
    }
}

// ---------------------------------------------------------------------------
// Timeline bookmark.
// ---------------------------------------------------------------------------

/// What kind of timeline bookmark this is.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BookmarkKind {
    /// A bookmark a user set explicitly.
    UserSet,
    /// A bookmark auto-placed at a recorded event.
    AutoEvent,
    /// A bookmark auto-placed at an error / exception stop.
    ErrorStop,
}

impl BookmarkKind {
    /// All bookmark kinds, in canonical order.
    pub const ALL: [Self; 3] = [Self::UserSet, Self::AutoEvent, Self::ErrorStop];

    /// Stable snake_case token for this kind.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::UserSet => "user_set",
            Self::AutoEvent => "auto_event",
            Self::ErrorStop => "error_stop",
        }
    }

    /// Human-readable label.
    pub const fn label(self) -> &'static str {
        match self {
            Self::UserSet => "User bookmark",
            Self::AutoEvent => "Event bookmark",
            Self::ErrorStop => "Error-stop bookmark",
        }
    }
}

/// A typed timeline bookmark: bound to exactly one capture/session/target identity, it
/// survives support export and restore review, so a bookmark placed in a replay timeline is
/// never orphaned from the capture it pins.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TimelineBookmark {
    /// Stable, namespaced bookmark id.
    pub bookmark_id: String,
    /// The capture/session/target identity this bookmark is bound to.
    pub capture: CaptureIdentity,
    /// The replay session this bookmark belongs to.
    pub replay_session_ref: String,
    /// Opaque digest of the timeline position; never a raw value or path.
    pub position_digest: String,
    /// The kind of bookmark.
    pub kind: BookmarkKind,
    /// Stable token for the kind.
    pub kind_token: String,
    /// One reviewable export-safe label for the bookmark.
    pub label: String,
    /// Whether the bookmark survives a support export (always true).
    pub survives_support_export: bool,
    /// Whether the bookmark survives a restore review (always true).
    pub survives_restore_review: bool,
    /// The proof packet that keeps this bookmark current.
    pub proof_packet_ref: String,
    /// One reviewable export-safe sentence describing the bookmark.
    pub summary: String,
}

impl TimelineBookmark {
    /// Builds a timeline bookmark, deriving the kind token. A bookmark is built to survive
    /// support export and restore review.
    #[allow(clippy::too_many_arguments)]
    pub fn build(
        bookmark_id: impl Into<String>,
        capture: CaptureIdentity,
        replay_session_ref: impl Into<String>,
        position_digest: impl Into<String>,
        kind: BookmarkKind,
        label: impl Into<String>,
        proof_packet_ref: impl Into<String>,
        summary: impl Into<String>,
    ) -> Self {
        Self {
            bookmark_id: bookmark_id.into(),
            capture,
            replay_session_ref: replay_session_ref.into(),
            position_digest: position_digest.into(),
            kind,
            kind_token: kind.as_str().to_owned(),
            label: label.into(),
            survives_support_export: true,
            survives_restore_review: true,
            proof_packet_ref: proof_packet_ref.into(),
            summary: summary.into(),
        }
    }
}

// ---------------------------------------------------------------------------
// Notebook-kernel capability descriptor.
// ---------------------------------------------------------------------------

/// A typed notebook-kernel capability descriptor: the canonical record the notebook debug
/// bridge, replay, AI, and support surfaces read to know *what debugging a notebook kernel
/// supports* — its backend family, support class, timeline state, the debug verbs it backs,
/// and the restart consequence that applies when the kernel restarts.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NotebookKernelCapabilityDescriptor {
    /// Stable, namespaced kernel id.
    pub kernel_id: String,
    /// The backend / runtime / toolchain family (a notebook kernel).
    pub backend_family: RuntimeBackendFamily,
    /// Stable token for the backend family.
    pub backend_family_token: String,
    /// The support class.
    pub support_class: DebugSupportClass,
    /// Stable token for the support class.
    pub support_class_token: String,
    /// The timeline state.
    pub timeline_state: TimelineState,
    /// Stable token for the timeline state.
    pub timeline_state_token: String,
    /// The canonical support pill every surface renders.
    pub support_pill: CapabilitySupportPill,
    /// The debug verbs this kernel backs.
    pub supported_verbs: Vec<CapabilityVerb>,
    /// Opaque kernel / kernel-session ref.
    pub kernel_ref: String,
    /// The restart consequence record that applies when this kernel restarts.
    pub restart_consequence_ref: String,
    /// The proof packet that keeps this descriptor current.
    pub proof_packet_ref: String,
    /// One reviewable export-safe sentence describing the descriptor.
    pub summary: String,
}

impl NotebookKernelCapabilityDescriptor {
    /// Builds a notebook-kernel capability descriptor, deriving every computed token and the
    /// support pill.
    #[allow(clippy::too_many_arguments)]
    pub fn build(
        kernel_id: impl Into<String>,
        backend_family: RuntimeBackendFamily,
        support_class: DebugSupportClass,
        timeline_state: TimelineState,
        supported_verbs: Vec<CapabilityVerb>,
        kernel_ref: impl Into<String>,
        restart_consequence_ref: impl Into<String>,
        proof_packet_ref: impl Into<String>,
        summary: impl Into<String>,
    ) -> Self {
        Self {
            kernel_id: kernel_id.into(),
            backend_family,
            backend_family_token: backend_family.as_str().to_owned(),
            support_class,
            support_class_token: support_class.as_str().to_owned(),
            timeline_state,
            timeline_state_token: timeline_state.as_str().to_owned(),
            support_pill: CapabilitySupportPill::derive(support_class, timeline_state),
            supported_verbs,
            kernel_ref: kernel_ref.into(),
            restart_consequence_ref: restart_consequence_ref.into(),
            proof_packet_ref: proof_packet_ref.into(),
            summary: summary.into(),
        }
    }
}

// ---------------------------------------------------------------------------
// Cell-frame link.
// ---------------------------------------------------------------------------

/// How faithfully a debugger frame maps to a notebook cell.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CellLinkFidelity {
    /// An exact frame-to-cell mapping backed by current identity.
    Exact,
    /// An approximate mapping (nearest cell / line drift).
    Approximate,
    /// A previously exact mapping gone stale after a restart or edit.
    Stale,
    /// No mapping; the frame could not be tied to a cell.
    Unmapped,
}

impl CellLinkFidelity {
    /// All fidelities, in canonical order.
    pub const ALL: [Self; 4] = [Self::Exact, Self::Approximate, Self::Stale, Self::Unmapped];

    /// Stable snake_case token for this fidelity.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Exact => "exact",
            Self::Approximate => "approximate",
            Self::Stale => "stale",
            Self::Unmapped => "unmapped",
        }
    }

    /// Human-readable label.
    pub const fn label(self) -> &'static str {
        match self {
            Self::Exact => "Exact",
            Self::Approximate => "Approximate",
            Self::Stale => "Stale",
            Self::Unmapped => "Unmapped",
        }
    }

    /// Whether this fidelity is an exact mapping.
    pub const fn is_exact(self) -> bool {
        matches!(self, Self::Exact)
    }
}

/// A typed frame-to-cell link: the canonical record the notebook debug bridge and replay
/// surfaces read to tie a debugger frame to a notebook cell. It renders an exact link only
/// when the mapping is exact and supported; a degraded mapping is never drawn exact.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CellFrameLink {
    /// Stable, namespaced link id.
    pub link_id: String,
    /// The notebook kernel this link belongs to.
    pub kernel_ref: String,
    /// Stable frame ref.
    pub frame_ref: String,
    /// Stable notebook cell ref.
    pub cell_ref: String,
    /// The mapping fidelity.
    pub fidelity: CellLinkFidelity,
    /// Stable token for the fidelity.
    pub fidelity_token: String,
    /// The support class of the link.
    pub support_class: DebugSupportClass,
    /// Stable token for the support class.
    pub support_class_token: String,
    /// Whether an exact frame-to-cell link should render — derived from fidelity and support.
    pub renders_exact_link: bool,
    /// The proof packet that keeps this link current.
    pub proof_packet_ref: String,
    /// One reviewable export-safe sentence describing the link.
    pub summary: String,
}

impl CellFrameLink {
    /// Builds a frame-to-cell link, deriving the computed tokens and the exact-link flag so a
    /// degraded mapping can never be drawn exact.
    #[allow(clippy::too_many_arguments)]
    pub fn build(
        link_id: impl Into<String>,
        kernel_ref: impl Into<String>,
        frame_ref: impl Into<String>,
        cell_ref: impl Into<String>,
        fidelity: CellLinkFidelity,
        support_class: DebugSupportClass,
        proof_packet_ref: impl Into<String>,
        summary: impl Into<String>,
    ) -> Self {
        let renders_exact_link = fidelity.is_exact() && support_class.permits_use();
        Self {
            link_id: link_id.into(),
            kernel_ref: kernel_ref.into(),
            frame_ref: frame_ref.into(),
            cell_ref: cell_ref.into(),
            fidelity,
            fidelity_token: fidelity.as_str().to_owned(),
            support_class,
            support_class_token: support_class.as_str().to_owned(),
            renders_exact_link,
            proof_packet_ref: proof_packet_ref.into(),
            summary: summary.into(),
        }
    }

    /// Whether the carried tokens and exact-link flag agree with the enums.
    pub fn is_consistent(&self) -> bool {
        self.fidelity_token == self.fidelity.as_str()
            && self.support_class_token == self.support_class.as_str()
            && self.renders_exact_link
                == (self.fidelity.is_exact() && self.support_class.permits_use())
    }
}

// ---------------------------------------------------------------------------
// Invariants and set.
// ---------------------------------------------------------------------------

/// One frozen invariant, with a computed `holds` flag.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ParityInvariant {
    /// Stable invariant id.
    pub invariant_id: String,
    /// The invariant statement.
    pub statement: String,
    /// Whether the built set satisfies the invariant.
    pub holds: bool,
}

/// The frozen, typed M5 chronology/replay/parity set.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ChronologyReplayParitySet {
    /// Stable record-kind tag.
    pub record_kind: String,
    /// Schema version.
    pub m5_chronology_replay_parity_schema_version: u32,
    /// Schema reference.
    pub schema_ref: String,
    /// Stable set id.
    pub set_id: String,
    /// Evaluation stamp.
    pub as_of: String,
    /// The freeze gate that keeps the set current.
    pub freeze_gate_ref: String,
    /// One reviewable sentence summarizing the set.
    pub summary: String,
    /// The boundary schemas this set binds as truth sources.
    pub source_schema_refs: Vec<String>,
    /// The crate modules that already produce this truth.
    pub producer_refs: Vec<String>,
    /// The surfaces that consume these records.
    pub consumer_surfaces: Vec<DebugConsumer>,
    /// The chronology capability descriptors.
    pub chronology_capabilities: Vec<ChronologyCapabilityDescriptor>,
    /// The replay sessions.
    pub replay_sessions: Vec<ReplaySession>,
    /// The timeline bookmarks.
    pub timeline_bookmarks: Vec<TimelineBookmark>,
    /// The notebook-kernel capability descriptors.
    pub notebook_kernels: Vec<NotebookKernelCapabilityDescriptor>,
    /// The frame-to-cell links.
    pub cell_frame_links: Vec<CellFrameLink>,
    /// The restart/reconnect consequence records for notebook, debug, and replay sessions.
    pub restart_consequences: Vec<RestartConsequenceRecord>,
    /// The computed invariants.
    pub invariants: Vec<ParityInvariant>,
    /// Whether raw capture/value bodies are excluded (always true).
    pub raw_payload_excluded: bool,
}

/// Error returned when the chronology/replay/parity set fails a structural consistency
/// check.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ChronologyReplayParitySetValidationError {
    /// The failed check.
    pub reason: String,
}

impl std::fmt::Display for ChronologyReplayParitySetValidationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "m5 chronology/replay/parity set invalid: {}",
            self.reason
        )
    }
}

impl std::error::Error for ChronologyReplayParitySetValidationError {}

impl ChronologyReplayParitySet {
    /// Returns the chronology descriptor with the given id, if present.
    pub fn chronology(&self, descriptor_id: &str) -> Option<&ChronologyCapabilityDescriptor> {
        self.chronology_capabilities
            .iter()
            .find(|c| c.descriptor_id == descriptor_id)
    }

    /// Returns the replay session with the given id, if present.
    pub fn replay_session(&self, replay_session_id: &str) -> Option<&ReplaySession> {
        self.replay_sessions
            .iter()
            .find(|r| r.replay_session_id == replay_session_id)
    }

    /// Returns the notebook kernel with the given id, if present.
    pub fn notebook_kernel(&self, kernel_id: &str) -> Option<&NotebookKernelCapabilityDescriptor> {
        self.notebook_kernels
            .iter()
            .find(|k| k.kernel_id == kernel_id)
    }

    /// Returns the restart consequence with the given id, if present.
    pub fn restart_consequence(&self, consequence_id: &str) -> Option<&RestartConsequenceRecord> {
        self.restart_consequences
            .iter()
            .find(|c| c.consequence_id == consequence_id)
    }

    /// Returns the first chronology descriptor in the given support class, if present.
    pub fn chronology_in_support_class(
        &self,
        support_class: DebugSupportClass,
    ) -> Option<&ChronologyCapabilityDescriptor> {
        self.chronology_capabilities
            .iter()
            .find(|c| c.support_class == support_class)
    }

    /// Whether every computed invariant holds.
    pub fn all_invariants_hold(&self) -> bool {
        self.invariants.iter().all(|i| i.holds)
    }

    /// Whether the record is safe to place in a support export: raw payloads are excluded
    /// and every ref is a repo-relative object ref, never a URL, host, credential, or
    /// absolute path.
    pub fn is_support_export_safe(&self) -> bool {
        if !self.raw_payload_excluded {
            return false;
        }
        self.all_refs().into_iter().all(is_export_safe_ref)
    }

    fn all_refs(&self) -> Vec<&str> {
        let mut refs: Vec<&str> = Vec::new();
        refs.extend(self.source_schema_refs.iter().map(String::as_str));
        refs.extend(self.producer_refs.iter().map(String::as_str));
        refs.push(self.freeze_gate_ref.as_str());
        refs.extend(
            self.chronology_capabilities
                .iter()
                .map(|c| c.proof_packet_ref.as_str()),
        );
        refs.extend(
            self.replay_sessions
                .iter()
                .map(|r| r.proof_packet_ref.as_str()),
        );
        refs.extend(
            self.timeline_bookmarks
                .iter()
                .map(|b| b.proof_packet_ref.as_str()),
        );
        refs.extend(
            self.notebook_kernels
                .iter()
                .map(|k| k.proof_packet_ref.as_str()),
        );
        refs.extend(
            self.cell_frame_links
                .iter()
                .map(|l| l.proof_packet_ref.as_str()),
        );
        refs.extend(
            self.restart_consequences
                .iter()
                .map(|c| c.proof_packet_ref.as_str()),
        );
        refs
    }

    /// Re-checks structural consistency and returns an error on the first failure.
    ///
    /// # Errors
    ///
    /// Returns a [`ChronologyReplayParitySetValidationError`] when an identifier, a ref, a
    /// computed flag, a support pill, a capability/support rule, a capture binding, a
    /// restart-consequence rule, a linkage, or an invariant is inconsistent.
    pub fn validate(&self) -> Result<(), ChronologyReplayParitySetValidationError> {
        let fail = |reason: String| Err(ChronologyReplayParitySetValidationError { reason });

        if self.record_kind != M5_CHRONOLOGY_REPLAY_PARITY_RECORD_KIND {
            return fail(format!("unexpected record_kind {}", self.record_kind));
        }
        if self.schema_ref != M5_CHRONOLOGY_REPLAY_PARITY_SCHEMA_REF {
            return fail(format!("unexpected schema_ref {}", self.schema_ref));
        }
        if self.m5_chronology_replay_parity_schema_version
            != M5_CHRONOLOGY_REPLAY_PARITY_SCHEMA_VERSION
        {
            return fail("unexpected schema version".to_owned());
        }
        if !self.raw_payload_excluded {
            return fail("raw_payload_excluded must be true".to_owned());
        }
        if self.chronology_capabilities.is_empty() {
            return fail("no chronology capabilities".to_owned());
        }
        if self.replay_sessions.is_empty() {
            return fail("no replay sessions".to_owned());
        }
        if self.timeline_bookmarks.is_empty() {
            return fail("no timeline bookmarks".to_owned());
        }
        if self.notebook_kernels.is_empty() {
            return fail("no notebook kernels".to_owned());
        }
        if self.cell_frame_links.is_empty() {
            return fail("no cell-frame links".to_owned());
        }
        if self.restart_consequences.is_empty() {
            return fail("no restart consequences".to_owned());
        }

        // Stable ids are unique.
        if !all_unique(
            self.chronology_capabilities
                .iter()
                .map(|c| c.descriptor_id.as_str()),
        ) {
            return fail("chronology descriptor ids are not unique".to_owned());
        }
        if !all_unique(
            self.replay_sessions
                .iter()
                .map(|r| r.replay_session_id.as_str()),
        ) {
            return fail("replay session ids are not unique".to_owned());
        }
        if !all_unique(
            self.timeline_bookmarks
                .iter()
                .map(|b| b.bookmark_id.as_str()),
        ) {
            return fail("timeline bookmark ids are not unique".to_owned());
        }
        if !all_unique(self.notebook_kernels.iter().map(|k| k.kernel_id.as_str())) {
            return fail("notebook kernel ids are not unique".to_owned());
        }
        if !all_unique(self.cell_frame_links.iter().map(|l| l.link_id.as_str())) {
            return fail("cell-frame link ids are not unique".to_owned());
        }
        if !all_unique(
            self.restart_consequences
                .iter()
                .map(|c| c.consequence_id.as_str()),
        ) {
            return fail("restart consequence ids are not unique".to_owned());
        }

        // The full support-class vocabulary is materialized across chronology descriptors.
        for class in DebugSupportClass::ALL {
            if self.chronology_in_support_class(class).is_none() {
                return fail(format!(
                    "support class {} is not materialized",
                    class.as_str()
                ));
            }
        }

        // Per-descriptor structural floor and capability rules.
        for c in &self.chronology_capabilities {
            validate_chronology(c)
                .map_err(|reason| ChronologyReplayParitySetValidationError { reason })?;
        }
        for r in &self.replay_sessions {
            validate_replay(r)
                .map_err(|reason| ChronologyReplayParitySetValidationError { reason })?;
            // A replay session sources from a chronology descriptor in the set.
            if self.chronology(&r.source_chronology_ref).is_none() {
                return fail(format!(
                    "replay session {} sources from missing chronology {}",
                    r.replay_session_id, r.source_chronology_ref
                ));
            }
            // A referenced restart consequence resolves.
            if let Some(ref cref) = r.restart_consequence_ref {
                if self.restart_consequence(cref).is_none() {
                    return fail(format!(
                        "replay session {} references missing restart consequence {cref}",
                        r.replay_session_id
                    ));
                }
            }
        }
        for k in &self.notebook_kernels {
            validate_kernel(k)
                .map_err(|reason| ChronologyReplayParitySetValidationError { reason })?;
            // The kernel's restart consequence resolves.
            if self
                .restart_consequence(&k.restart_consequence_ref)
                .is_none()
            {
                return fail(format!(
                    "notebook kernel {} references missing restart consequence {}",
                    k.kernel_id, k.restart_consequence_ref
                ));
            }
        }
        for l in &self.cell_frame_links {
            validate_cell_link(l)
                .map_err(|reason| ChronologyReplayParitySetValidationError { reason })?;
            // The link's kernel resolves.
            if self.notebook_kernel(&l.kernel_ref).is_none() {
                return fail(format!(
                    "cell-frame link {} references missing kernel {}",
                    l.link_id, l.kernel_ref
                ));
            }
        }
        for b in &self.timeline_bookmarks {
            validate_bookmark(b)
                .map_err(|reason| ChronologyReplayParitySetValidationError { reason })?;
            // The bookmark's replay session resolves, and the capture identity matches it.
            match self.replay_session(&b.replay_session_ref) {
                None => {
                    return fail(format!(
                        "timeline bookmark {} references missing replay session {}",
                        b.bookmark_id, b.replay_session_ref
                    ));
                }
                Some(rs) => {
                    if !rs.capture.same_as(&b.capture) {
                        return fail(format!(
                            "timeline bookmark {} is bound to a capture that differs from its replay session",
                            b.bookmark_id
                        ));
                    }
                }
            }
        }
        for c in &self.restart_consequences {
            validate_consequence(c)
                .map_err(|reason| ChronologyReplayParitySetValidationError { reason })?;
        }

        if !self.is_support_export_safe() {
            return fail("set is not support-export safe".to_owned());
        }
        if !self.all_invariants_hold() {
            let failed: Vec<&str> = self
                .invariants
                .iter()
                .filter(|i| !i.holds)
                .map(|i| i.invariant_id.as_str())
                .collect();
            return fail(format!("invariants do not hold: {}", failed.join(", ")));
        }
        Ok(())
    }
}

fn validate_chronology(c: &ChronologyCapabilityDescriptor) -> Result<(), String> {
    if c.descriptor_id.is_empty() {
        return Err("chronology descriptor has empty id".to_owned());
    }
    if c.session_ref.is_empty() {
        return Err(format!(
            "chronology {} has empty session ref",
            c.descriptor_id
        ));
    }
    if c.proof_packet_ref.is_empty() {
        return Err(format!(
            "chronology {} has no proof packet",
            c.descriptor_id
        ));
    }
    if !capability_tokens_consistent(
        &c.support_pill,
        c.support_class,
        c.support_class_token.as_str(),
        c.timeline_state,
        c.timeline_state_token.as_str(),
    ) {
        return Err(format!(
            "chronology {} support tokens or pill disagree with its enums",
            c.descriptor_id
        ));
    }
    if c.backend_family_token != c.backend_family.as_str()
        || c.recorded_scope_token != c.recorded_scope.as_str()
        || c.notebook_parity_token != c.notebook_parity.as_str()
    {
        return Err(format!(
            "chronology {} carries a stale enum token",
            c.descriptor_id
        ));
    }
    // Time-travel verbs are backed only when the pill says time travel is available.
    verbs_match_capability(
        &c.descriptor_id,
        &c.supported_verbs,
        c.support_pill.time_travel_available,
    )?;
    // An inert descriptor backs no verbs and records no history.
    if c.support_pill.is_inert {
        if !c.supported_verbs.is_empty() {
            return Err(format!(
                "inert chronology {} must back no verbs",
                c.descriptor_id
            ));
        }
        if c.recorded_scope.records_history() {
            return Err(format!(
                "inert chronology {} must record no history",
                c.descriptor_id
            ));
        }
    }
    Ok(())
}

fn validate_replay(r: &ReplaySession) -> Result<(), String> {
    if r.replay_session_id.is_empty() {
        return Err("replay session has empty id".to_owned());
    }
    if r.source_chronology_ref.is_empty() {
        return Err(format!(
            "replay session {} has empty source chronology ref",
            r.replay_session_id
        ));
    }
    if r.proof_packet_ref.is_empty() {
        return Err(format!(
            "replay session {} has no proof packet",
            r.replay_session_id
        ));
    }
    // A replay is always inspect-only.
    if !r.inspect_only {
        return Err(format!(
            "replay session {} must be inspect-only",
            r.replay_session_id
        ));
    }
    if !r.capture.is_fully_bound() {
        return Err(format!(
            "replay session {} has an unbound capture identity",
            r.replay_session_id
        ));
    }
    if !capability_tokens_consistent(
        &r.support_pill,
        r.support_class,
        r.support_class_token.as_str(),
        r.timeline_state,
        r.timeline_state_token.as_str(),
    ) {
        return Err(format!(
            "replay session {} support tokens or pill disagree with its enums",
            r.replay_session_id
        ));
    }
    if r.backend_family_token != r.backend_family.as_str()
        || r.notebook_parity_token != r.notebook_parity.as_str()
    {
        return Err(format!(
            "replay session {} carries a stale enum token",
            r.replay_session_id
        ));
    }
    verbs_match_capability(
        &r.replay_session_id,
        &r.supported_verbs,
        r.support_pill.time_travel_available,
    )?;
    Ok(())
}

fn validate_kernel(k: &NotebookKernelCapabilityDescriptor) -> Result<(), String> {
    if k.kernel_id.is_empty() {
        return Err("notebook kernel has empty id".to_owned());
    }
    if k.kernel_ref.is_empty() {
        return Err(format!(
            "notebook kernel {} has empty kernel ref",
            k.kernel_id
        ));
    }
    if k.restart_consequence_ref.is_empty() {
        return Err(format!(
            "notebook kernel {} has no restart consequence ref",
            k.kernel_id
        ));
    }
    if k.proof_packet_ref.is_empty() {
        return Err(format!(
            "notebook kernel {} has no proof packet",
            k.kernel_id
        ));
    }
    if k.backend_family != RuntimeBackendFamily::NotebookKernel {
        return Err(format!(
            "notebook kernel {} must carry the notebook_kernel backend family",
            k.kernel_id
        ));
    }
    if !capability_tokens_consistent(
        &k.support_pill,
        k.support_class,
        k.support_class_token.as_str(),
        k.timeline_state,
        k.timeline_state_token.as_str(),
    ) {
        return Err(format!(
            "notebook kernel {} support tokens or pill disagree with its enums",
            k.kernel_id
        ));
    }
    if k.backend_family_token != k.backend_family.as_str() {
        return Err(format!(
            "notebook kernel {} carries a stale enum token",
            k.kernel_id
        ));
    }
    // Debug verbs are backed only when the support class permits use; an inert kernel backs
    // none. (Notebook debug does not require a recorded timeline.)
    if k.support_pill.permits_use {
        if k.supported_verbs.is_empty() {
            return Err(format!(
                "supported notebook kernel {} must back at least one verb",
                k.kernel_id
            ));
        }
    } else if !k.supported_verbs.is_empty() {
        return Err(format!(
            "inert notebook kernel {} must back no verbs",
            k.kernel_id
        ));
    }
    Ok(())
}

fn validate_cell_link(l: &CellFrameLink) -> Result<(), String> {
    if l.link_id.is_empty() {
        return Err("cell-frame link has empty id".to_owned());
    }
    if l.frame_ref.is_empty() || l.cell_ref.is_empty() {
        return Err(format!(
            "cell-frame link {} has an empty frame/cell ref",
            l.link_id
        ));
    }
    if l.proof_packet_ref.is_empty() {
        return Err(format!("cell-frame link {} has no proof packet", l.link_id));
    }
    if !l.is_consistent() {
        return Err(format!(
            "cell-frame link {} tokens or exact-link flag disagree with its enums",
            l.link_id
        ));
    }
    // A degraded or unsupported link is never drawn exact.
    if l.renders_exact_link && (!l.fidelity.is_exact() || !l.support_class.permits_use()) {
        return Err(format!(
            "cell-frame link {} renders an exact link without an exact, supported mapping",
            l.link_id
        ));
    }
    Ok(())
}

fn validate_bookmark(b: &TimelineBookmark) -> Result<(), String> {
    if b.bookmark_id.is_empty() {
        return Err("timeline bookmark has empty id".to_owned());
    }
    if b.position_digest.is_empty() {
        return Err(format!(
            "timeline bookmark {} has empty position digest",
            b.bookmark_id
        ));
    }
    if b.replay_session_ref.is_empty() {
        return Err(format!(
            "timeline bookmark {} has empty replay session ref",
            b.bookmark_id
        ));
    }
    if b.proof_packet_ref.is_empty() {
        return Err(format!(
            "timeline bookmark {} has no proof packet",
            b.bookmark_id
        ));
    }
    if b.kind_token != b.kind.as_str() {
        return Err(format!(
            "timeline bookmark {} carries a stale kind token",
            b.bookmark_id
        ));
    }
    // A bookmark is bound to exactly one capture/session/target identity.
    if !b.capture.is_fully_bound() {
        return Err(format!(
            "timeline bookmark {} is not bound to a full capture identity",
            b.bookmark_id
        ));
    }
    // A bookmark survives support export and restore review.
    if !b.survives_support_export || !b.survives_restore_review {
        return Err(format!(
            "timeline bookmark {} must survive support export and restore review",
            b.bookmark_id
        ));
    }
    Ok(())
}

fn validate_consequence(c: &RestartConsequenceRecord) -> Result<(), String> {
    if c.consequence_id.is_empty() {
        return Err("restart consequence has empty id".to_owned());
    }
    if c.session_ref.is_empty() {
        return Err(format!(
            "restart consequence {} has empty session ref",
            c.consequence_id
        ));
    }
    if c.proof_packet_ref.is_empty() {
        return Err(format!(
            "restart consequence {} has no proof packet",
            c.consequence_id
        ));
    }
    if c.trigger_token != c.trigger.as_str() {
        return Err(format!(
            "restart consequence {} carries a stale trigger token",
            c.consequence_id
        ));
    }
    // Every required subject is itemized exactly once, so a restart/reconnect is never
    // flattened into a generic banner.
    if !c.itemizes_every_subject() {
        return Err(format!(
            "restart consequence {} must itemize variables, queued cells, debug state, breakpoints, and transient outputs",
            c.consequence_id
        ));
    }
    for entry in &c.entries {
        if !entry.is_consistent() {
            return Err(format!(
                "restart consequence {} carries an entry with a stale token",
                c.consequence_id
            ));
        }
        if entry.detail.is_empty() {
            return Err(format!(
                "restart consequence {} carries an entry with no detail",
                c.consequence_id
            ));
        }
    }
    Ok(())
}

fn capability_tokens_consistent(
    pill: &CapabilitySupportPill,
    support_class: DebugSupportClass,
    support_class_token: &str,
    timeline_state: TimelineState,
    timeline_state_token: &str,
) -> bool {
    support_class_token == support_class.as_str()
        && timeline_state_token == timeline_state.as_str()
        && pill.matches_derivation(support_class, timeline_state)
}

fn verbs_match_capability(
    id: &str,
    verbs: &[CapabilityVerb],
    time_travel_available: bool,
) -> Result<(), String> {
    // Time-travel verbs require a recorded/replayable timeline.
    for verb in verbs {
        if verb.requires_time_travel() && !time_travel_available {
            return Err(format!(
                "{id} backs time-travel verb {} without a replayable timeline",
                verb.as_str()
            ));
        }
    }
    // A capability that can time-travel backs at least one verb; one that cannot backs none.
    if time_travel_available {
        if verbs.is_empty() {
            return Err(format!("{id} permits time travel but backs no verbs"));
        }
    } else if !verbs.is_empty() {
        return Err(format!("{id} backs verbs without a replayable timeline"));
    }
    Ok(())
}

fn all_unique<'a>(iter: impl Iterator<Item = &'a str>) -> bool {
    let mut seen = std::collections::BTreeSet::new();
    iter.into_iter().all(|item| seen.insert(item))
}

/// Whether a ref is safe to export: a repo-relative object ref or opaque `aureline://`
/// handle, never a URL, host, credential, or absolute path.
fn is_export_safe_ref(r: &str) -> bool {
    if r.is_empty() || r.starts_with('/') || (r.contains("://") && !r.starts_with("aureline://")) {
        return false;
    }
    r.starts_with("schemas/")
        || r.starts_with("crates/")
        || r.starts_with("artifacts/")
        || r.starts_with("fixtures/")
        || r.starts_with("docs/")
        || r.starts_with("aureline://")
}

// ---------------------------------------------------------------------------
// Canonical binding.
// ---------------------------------------------------------------------------

/// Builds the canonical M5 chronology/replay/parity set.
///
/// Deterministic: the same bytes every call. Each invariant's `holds` flag is computed from
/// the built records, so an inconsistent edit flips an invariant rather than silently
/// passing.
pub fn m5_chronology_replay_parity_set() -> ChronologyReplayParitySet {
    let chronology_capabilities = build_chronology();
    let replay_sessions = build_replays();
    let timeline_bookmarks = build_bookmarks();
    let notebook_kernels = build_kernels();
    let cell_frame_links = build_cell_links();
    let restart_consequences = build_consequences();
    let invariants = compute_invariants(
        &chronology_capabilities,
        &replay_sessions,
        &timeline_bookmarks,
        &notebook_kernels,
        &cell_frame_links,
        &restart_consequences,
    );

    ChronologyReplayParitySet {
        record_kind: M5_CHRONOLOGY_REPLAY_PARITY_RECORD_KIND.to_owned(),
        m5_chronology_replay_parity_schema_version: M5_CHRONOLOGY_REPLAY_PARITY_SCHEMA_VERSION,
        schema_ref: M5_CHRONOLOGY_REPLAY_PARITY_SCHEMA_REF.to_owned(),
        set_id: M5_CHRONOLOGY_REPLAY_PARITY_SET_ID.to_owned(),
        as_of: M5_CHRONOLOGY_REPLAY_PARITY_AS_OF.to_owned(),
        freeze_gate_ref: M5_CHRONOLOGY_REPLAY_PARITY_FREEZE_GATE_REF.to_owned(),
        summary: "One frozen, typed set of M5 chronology-capability descriptors, replay \
                  sessions, timeline bookmarks, notebook-kernel capability descriptors, \
                  cell-frame links, and restart/reconnect consequence records. Every \
                  descriptor carries one support pill that pins one support class (supported, \
                  limited, unavailable, policy-blocked) and one timeline state, derived only \
                  from its own backend, so an unsupported runtime never inherits a neighbor's \
                  chronology or notebook-debug claim; a replay session is always inspect-only \
                  and names the capture it reconstructs; a timeline bookmark is bound to one \
                  capture/session/target identity and survives support export and restore \
                  review; a restart/reconnect consequence itemizes — per variables, queued \
                  cells, debug state, breakpoints, and transient outputs — what was preserved, \
                  lost, invalidated, or left stale rather than flattening into one banner; and \
                  a frame-to-cell link renders exact only when its mapping is exact and \
                  supported."
            .to_owned(),
        source_schema_refs: strvec(&[
            "schemas/debug/m5_chronology_replay_parity.schema.json",
            "schemas/debug/m5_debug_contracts.schema.json",
            "schemas/debug/chronology-replay-support.schema.json",
            "schemas/notebook/ship_the_notebook_debugger_bridge_frame_to_cell_linkage_and_kernel_restart_consequence_records.schema.json",
        ]),
        producer_refs: strvec(&[
            "crates/aureline-debug/src/m5_chronology_replay_parity/mod.rs",
            "crates/aureline-debug/src/qualify_chronology_capture_and_replay_support_classes/mod.rs",
            "crates/aureline-notebook/src/ship_the_notebook_debugger_bridge_frame_to_cell_linkage_and_kernel_restart_consequence_records/mod.rs",
        ]),
        consumer_surfaces: vec![
            DebugConsumer::CoreDebugger,
            DebugConsumer::NotebookDebug,
            DebugConsumer::Profiler,
            DebugConsumer::IncidentReview,
            DebugConsumer::SupportExport,
            DebugConsumer::AiContext,
            DebugConsumer::ReviewWorkspace,
            DebugConsumer::CliHeadless,
        ],
        chronology_capabilities,
        replay_sessions,
        timeline_bookmarks,
        notebook_kernels,
        cell_frame_links,
        restart_consequences,
        invariants,
        raw_payload_excluded: true,
    }
}

fn strvec(items: &[&str]) -> Vec<String> {
    items.iter().map(|s| (*s).to_owned()).collect()
}

// Proof packets — every ref below is an on-disk fixture/schema the freeze gate verifies.
const CHRONO_SUPPORTED_PROOF: &str =
    "fixtures/debug/chronology_cases/supported_recorded_session.yaml";
const CHRONO_PARTIAL_PROOF: &str = "fixtures/debug/chronology_cases/partial_history_recording.yaml";
const CHRONO_LIVE_PROOF: &str =
    "fixtures/debug/chronology_cases/unrecorded_live_debug_session.yaml";
const CHRONO_EXPIRED_PROOF: &str = "fixtures/debug/chronology_cases/expired_capture.yaml";
const CHRONO_MISMATCH_PROOF: &str =
    "fixtures/debug/chronology_cases/artifact_mismatch_after_rebuild.yaml";
const REPLAY_CONTAINER_PROOF: &str =
    "fixtures/runtime/container_cases/session_replayed_from_container_capture.json";
const REPLAY_BROWSER_PROOF: &str =
    "fixtures/runtime/browser_runtime_cases/session_replayed_from_capture.yaml";
const REPLAY_LOCAL_PROOF: &str =
    "fixtures/runtime/m3/replay_packets/local_task_exact_read_only.json";
const KERNEL_PRESERVED_PROOF: &str =
    "fixtures/notebook/m5/ship_the_notebook_debugger_bridge_frame_to_cell_linkage_and_kernel_restart_consequence_records/restart_consequence_preserved.json";
const KERNEL_RESET_PROOF: &str =
    "fixtures/notebook/m5/ship_the_notebook_debugger_bridge_frame_to_cell_linkage_and_kernel_restart_consequence_records/restart_consequence_reset.json";
const KERNEL_UNAVAILABLE_PROOF: &str =
    "fixtures/notebook/m5/ship_the_notebook_debugger_bridge_frame_to_cell_linkage_and_kernel_restart_consequence_records/restart_consequence_unavailable.json";
const LINK_EXACT_PROOF: &str =
    "fixtures/notebook/m5/ship_the_notebook_debugger_bridge_frame_to_cell_linkage_and_kernel_restart_consequence_records/frame_cell_link_exact_match.json";
const LINK_STALE_PROOF: &str =
    "fixtures/notebook/m5/ship_the_notebook_debugger_bridge_frame_to_cell_linkage_and_kernel_restart_consequence_records/frame_cell_link_stale.json";
const LINK_NO_MAPPING_PROOF: &str =
    "fixtures/notebook/m5/ship_the_notebook_debugger_bridge_frame_to_cell_linkage_and_kernel_restart_consequence_records/frame_cell_link_no_mapping.json";

// Stable identities reused across records.
const CHRONO_LOCAL: &str = "debug.chronology:local_native_supported:0001";
const CHRONO_REMOTE: &str = "debug.chronology:remote_helper_partial:0002";
const CHRONO_CONTAINER: &str = "debug.chronology:container_recording:0003";
const CHRONO_BROWSER: &str = "debug.chronology:browser_unavailable:0004";
const CHRONO_MANAGED: &str = "debug.chronology:managed_policy_blocked:0005";
const CHRONO_REMOTE_EXPIRED: &str = "debug.chronology:remote_helper_expired:0006";

const REPLAY_LOCAL: &str = "debug.replay:local_native_active:0001";
const REPLAY_CONTAINER: &str = "debug.replay:container_mismatched:0002";

const CAPTURE_LOCAL: &str = "debug.capture:local_native:0001";
const CAPTURE_CONTAINER: &str = "debug.capture:container:0002";

const KERNEL_PY: &str = "notebook.kernel:python_local_supported:0001";
const KERNEL_REMOTE: &str = "notebook.kernel:python_remote_limited:0002";
const KERNEL_MANAGED: &str = "notebook.kernel:managed_policy_blocked:0003";

const RC_SESSION_RESTART: &str = "debug.consequence:session_restart:0001";
const RC_DEBUG_RECONNECT: &str = "debug.consequence:debug_reconnect:0002";
const RC_KERNEL_RESTART: &str = "debug.consequence:kernel_restart_preserved:0003";
const RC_KERNEL_RESET: &str = "debug.consequence:kernel_restart_reset:0004";
const RC_TRANSPORT_LOST: &str = "debug.consequence:kernel_transport_lost:0005";
const RC_REPLAY_REACQUIRE: &str = "debug.consequence:replay_reacquire:0006";

fn build_chronology() -> Vec<ChronologyCapabilityDescriptor> {
    use CapabilityVerb::*;
    use DebugSupportClass::*;
    use NotebookParityClass::*;
    use RecordedScope::*;
    use RuntimeBackendFamily::*;
    use TimelineState::*;

    let full_verbs = vec![
        SetBreakpoint,
        Step,
        Continue,
        ReverseStep,
        ReverseContinue,
        JumpToEvent,
        SetBookmark,
        JumpToBookmark,
        Evaluate,
        InspectVariables,
        InspectHistoricalFrame,
    ];
    let partial_verbs = vec![
        SetBreakpoint,
        Step,
        Continue,
        ReverseStep,
        JumpToEvent,
        InspectVariables,
        InspectHistoricalFrame,
    ];
    let recording_verbs = vec![
        SetBreakpoint,
        Step,
        Continue,
        ReverseStep,
        JumpToEvent,
        SetBookmark,
        Evaluate,
        InspectVariables,
        InspectHistoricalFrame,
    ];

    vec![
        // 1. Local native, fully supported, complete recorded timeline: the clean
        //    time-travel path with full notebook parity.
        ChronologyCapabilityDescriptor::build(
            CHRONO_LOCAL,
            LocalNative,
            Supported,
            RecordedComplete,
            full_verbs,
            FullSession,
            Mirrored,
            "debug.session:local-launch:0001",
            Some(CAPTURE_LOCAL),
            CHRONO_SUPPORTED_PROOF,
            "Local native session with a complete recorded timeline: fully supported \
             time-travel, every reverse verb backed, mirrored in the notebook surface.",
        ),
        // 2. Remote helper, limited, partial history: a disclosed subset of time-travel
        //    verbs, divergent notebook parity.
        ChronologyCapabilityDescriptor::build(
            CHRONO_REMOTE,
            RemoteHelper,
            Limited,
            RecordedPartial,
            partial_verbs,
            SinceAttach,
            Divergent,
            "debug.session:remote-attach:0002",
            Some("debug.capture:remote_helper:0003"),
            CHRONO_PARTIAL_PROOF,
            "Remote-helper session with partial history since attach: limited time-travel \
             with reverse-continue withheld, disclosed as a divergent notebook subset.",
        ),
        // 3. Container, supported, currently recording: time-travel within the captured
        //    prefix.
        ChronologyCapabilityDescriptor::build(
            CHRONO_CONTAINER,
            Container,
            Supported,
            Recording,
            recording_verbs,
            BoundedWindow,
            Mirrored,
            "debug.session:container-attach:0003",
            Some(CAPTURE_CONTAINER),
            REPLAY_CONTAINER_PROOF,
            "Container session recording a bounded window: supported time-travel within the \
             captured prefix, mirrored in the notebook surface.",
        ),
        // 4. Browser runtime, unavailable: no chronology — the guardrail case that must not
        //    inherit a neighboring backend's time-travel.
        ChronologyCapabilityDescriptor::build(
            CHRONO_BROWSER,
            BrowserRuntime,
            DebugSupportClass::Unavailable,
            TimelineState::Unavailable,
            Vec::new(),
            RecordedScope::None,
            NotApplicable,
            "debug.session:browser-attach:0004",
            Option::None,
            REPLAY_BROWSER_PROOF,
            "Browser runtime without chronology support: time-travel unavailable, no verbs \
             backed, and no notebook parity inherited from neighboring backends.",
        ),
        // 5. Managed runtime, policy-blocked, live: recording disabled by policy, not a
        //    technical gap.
        ChronologyCapabilityDescriptor::build(
            CHRONO_MANAGED,
            ManagedRuntime,
            PolicyBlocked,
            LiveNoRecording,
            Vec::new(),
            RecordedScope::None,
            Unsupported,
            "debug.session:managed-attach:0005",
            Option::None,
            CHRONO_LIVE_PROOF,
            "Managed runtime with chronology blocked by policy: live session, no recording, \
             no verbs backed, named as policy-blocked rather than a missing feature.",
        ),
        // 6. Remote helper, capture expired: timeline gone, time-travel unavailable until a
        //    fresh recording.
        ChronologyCapabilityDescriptor::build(
            CHRONO_REMOTE_EXPIRED,
            RemoteHelper,
            DebugSupportClass::Unavailable,
            Expired,
            Vec::new(),
            RecordedScope::None,
            Unsupported,
            "debug.session:remote-attach:0006",
            Option::None,
            CHRONO_EXPIRED_PROOF,
            "Remote-helper capture expired and evicted: timeline gone, time-travel \
             unavailable until a fresh recording is made.",
        ),
    ]
}

fn build_replays() -> Vec<ReplaySession> {
    use CapabilityVerb::*;
    use DebugSupportClass::*;
    use NotebookParityClass::*;
    use RuntimeBackendFamily::*;
    use TimelineState::*;

    let replay_verbs = vec![
        ReverseStep,
        ReverseContinue,
        JumpToEvent,
        SetBookmark,
        JumpToBookmark,
        InspectVariables,
        InspectHistoricalFrame,
    ];

    vec![
        // 1. Local native replay actively reconstructing a capture: supported, full replay
        //    verbs, inspect-only, with a reconnect/reacquire consequence.
        ReplaySession::build(
            REPLAY_LOCAL,
            LocalNative,
            Supported,
            ReplayActive,
            replay_verbs,
            CaptureIdentity::build(
                CAPTURE_LOCAL,
                "debug.session:local-launch:0001",
                "debug.target:local_native_pid:0001",
                Some("build.artifact:local_native_exact:0001"),
            ),
            CHRONO_LOCAL,
            Some(RC_REPLAY_REACQUIRE),
            Mirrored,
            REPLAY_LOCAL_PROOF,
            "Local native replay reconstructing a complete capture: supported, inspect-only, \
             full replay verbs backed, with an itemized reacquire consequence.",
        ),
        // 2. Container replay whose capture no longer matches the rebuilt artifact: limited
        //    support, mismatched timeline, no replay verbs until re-recorded.
        ReplaySession::build(
            REPLAY_CONTAINER,
            Container,
            Limited,
            Mismatched,
            Vec::new(),
            CaptureIdentity::build(
                CAPTURE_CONTAINER,
                "debug.session:container-attach:0003",
                "debug.target:container_pid:0002",
                Some("build.artifact:container_rebuilt:0002"),
            ),
            CHRONO_CONTAINER,
            Option::None,
            Divergent,
            CHRONO_MISMATCH_PROOF,
            "Container replay whose capture no longer matches the rebuilt artifact: replay \
             degraded to a disclosed mismatch with no replay verbs until re-recorded.",
        ),
    ]
}

fn build_bookmarks() -> Vec<TimelineBookmark> {
    use BookmarkKind::*;

    let capture = CaptureIdentity::build(
        CAPTURE_LOCAL,
        "debug.session:local-launch:0001",
        "debug.target:local_native_pid:0001",
        Some("build.artifact:local_native_exact:0001"),
    );

    vec![
        // 1. A user-set bookmark pinned to the local capture.
        TimelineBookmark::build(
            "debug.bookmark:user_request_entry:0001",
            capture.clone(),
            REPLAY_LOCAL,
            "timeline:digest:b1aa11",
            UserSet,
            "Request handler entry",
            REPLAY_LOCAL_PROOF,
            "A user bookmark pinned to the request-handler entry in the local capture: bound \
             to one capture/session/target identity, survives export and restore review.",
        ),
        // 2. An auto-placed event bookmark.
        TimelineBookmark::build(
            "debug.bookmark:auto_db_commit:0002",
            capture.clone(),
            REPLAY_LOCAL,
            "timeline:digest:b2bb22",
            AutoEvent,
            "Database commit event",
            REPLAY_LOCAL_PROOF,
            "An auto-placed bookmark at the database-commit event: bound to the same capture \
             identity and preserved across support export.",
        ),
        // 3. An error-stop bookmark.
        TimelineBookmark::build(
            "debug.bookmark:error_unhandled:0003",
            capture,
            REPLAY_LOCAL,
            "timeline:digest:b3cc33",
            ErrorStop,
            "Unhandled exception stop",
            REPLAY_LOCAL_PROOF,
            "An error-stop bookmark at the unhandled exception: bound to the capture identity \
             and surviving restore review for incident handoff.",
        ),
    ]
}

fn build_kernels() -> Vec<NotebookKernelCapabilityDescriptor> {
    use CapabilityVerb::*;
    use DebugSupportClass::*;
    use RuntimeBackendFamily::*;
    use TimelineState::*;

    let kernel_verbs = vec![SetBreakpoint, Step, Continue, Evaluate, InspectVariables];
    let limited_kernel_verbs = vec![SetBreakpoint, Step, InspectVariables];

    vec![
        // 1. Local Python kernel, fully supported debug bridge: live (kernels do not record),
        //    full debug verbs, breakpoints preserved across restart.
        NotebookKernelCapabilityDescriptor::build(
            KERNEL_PY,
            NotebookKernel,
            Supported,
            LiveNoRecording,
            kernel_verbs,
            "notebook.kernel.session:python_local:0001",
            RC_KERNEL_RESTART,
            KERNEL_PRESERVED_PROOF,
            "Local Python kernel with a fully supported debug bridge: full debug verbs, \
             breakpoints preserved across kernel restart.",
        ),
        // 2. Remote Python kernel, limited debug bridge: a disclosed subset of verbs, fresh
        //    session on restart.
        NotebookKernelCapabilityDescriptor::build(
            KERNEL_REMOTE,
            NotebookKernel,
            Limited,
            LiveNoRecording,
            limited_kernel_verbs,
            "notebook.kernel.session:python_remote:0002",
            RC_KERNEL_RESET,
            KERNEL_RESET_PROOF,
            "Remote Python kernel with a limited debug bridge: a disclosed subset of verbs, a \
             fresh session with state lost on restart.",
        ),
        // 3. Managed kernel, debug blocked by policy: no verbs, transport-lost reconnect
        //    consequence.
        NotebookKernelCapabilityDescriptor::build(
            KERNEL_MANAGED,
            NotebookKernel,
            PolicyBlocked,
            TimelineState::Unavailable,
            Vec::new(),
            "notebook.kernel.session:managed:0003",
            RC_TRANSPORT_LOST,
            KERNEL_UNAVAILABLE_PROOF,
            "Managed kernel with debug blocked by policy: no debug verbs backed, named as \
             policy-blocked, with a transport-lost reconnect consequence itemized.",
        ),
    ]
}

fn build_cell_links() -> Vec<CellFrameLink> {
    use CellLinkFidelity::*;
    use DebugSupportClass::*;

    vec![
        // 1. Exact, supported: the only link that renders an exact frame-to-cell mapping.
        CellFrameLink::build(
            "notebook.cell_frame:exact_current:0001",
            KERNEL_PY,
            "debug.frame:kernel_current:0001",
            "notebook:doc:analysis#cell-2",
            Exact,
            Supported,
            LINK_EXACT_PROOF,
            "An exact frame-to-cell mapping on a supported kernel: the current frame ties to \
             cell 2 and renders as an exact link.",
        ),
        // 2. Approximate, supported: a nearest-cell mapping, never drawn exact.
        CellFrameLink::build(
            "notebook.cell_frame:approximate_nearest:0002",
            KERNEL_PY,
            "debug.frame:kernel_helper:0002",
            "notebook:doc:analysis#cell-3",
            Approximate,
            Supported,
            LINK_EXACT_PROOF,
            "An approximate frame-to-cell mapping to the nearest cell: disclosed as \
             approximate and never drawn as an exact link.",
        ),
        // 3. Stale after restart, limited: a previously exact mapping gone stale.
        CellFrameLink::build(
            "notebook.cell_frame:stale_after_restart:0003",
            KERNEL_REMOTE,
            "debug.frame:kernel_stale:0003",
            "notebook:doc:analysis#cell-4",
            Stale,
            Limited,
            LINK_STALE_PROOF,
            "A frame-to-cell mapping gone stale after a kernel restart: disclosed as stale \
             rather than presented as a current exact link.",
        ),
        // 4. Unmapped, policy-blocked: no mapping on a policy-blocked kernel.
        CellFrameLink::build(
            "notebook.cell_frame:unmapped_blocked:0004",
            KERNEL_MANAGED,
            "debug.frame:kernel_unmapped:0004",
            "notebook:doc:analysis#cell-5",
            Unmapped,
            PolicyBlocked,
            LINK_NO_MAPPING_PROOF,
            "An unmapped frame on a policy-blocked kernel: no cell mapping, never drawn exact, \
             with the gap named rather than hidden.",
        ),
    ]
}

fn build_consequences() -> Vec<RestartConsequenceRecord> {
    use ConsequenceDisposition::*;
    use ConsequenceSubject::*;
    use ConsequenceTrigger::*;

    vec![
        // 1. Debug session restart: state lost, breakpoints preserved.
        RestartConsequenceRecord::build(
            RC_SESSION_RESTART,
            SessionRestart,
            "debug.session:local-launch:0001",
            vec![
                ConsequenceEntry::build(
                    Variables,
                    Lost,
                    "All variable state was discarded on restart.",
                ),
                ConsequenceEntry::build(
                    QueuedCells,
                    Lost,
                    "No queued cells apply to a debug session restart.",
                ),
                ConsequenceEntry::build(
                    DebugState,
                    Lost,
                    "The debug session state was reset to a fresh launch.",
                ),
                ConsequenceEntry::build(
                    Breakpoints,
                    Preserved,
                    "Breakpoints were re-bound to the fresh session.",
                ),
                ConsequenceEntry::build(
                    TransientOutputs,
                    Lost,
                    "Transient console output was cleared.",
                ),
            ],
            REPLAY_LOCAL_PROOF,
            "Debug session restart: variables, debug state, and transient outputs lost; \
             breakpoints preserved and re-bound to the fresh session.",
        ),
        // 2. Debug session reconnect after a transport drop: state preserved or stale.
        RestartConsequenceRecord::build(
            RC_DEBUG_RECONNECT,
            Reconnect,
            "debug.session:remote-attach:0002",
            vec![
                ConsequenceEntry::build(
                    Variables,
                    Preserved,
                    "Variable state was recovered intact on reconnect.",
                ),
                ConsequenceEntry::build(
                    QueuedCells,
                    Stale,
                    "Queued cells survived but may be out of date.",
                ),
                ConsequenceEntry::build(
                    DebugState,
                    Invalidated,
                    "In-flight debug state was invalidated and re-synced.",
                ),
                ConsequenceEntry::build(
                    Breakpoints,
                    Preserved,
                    "Breakpoints were preserved across the reconnect.",
                ),
                ConsequenceEntry::build(
                    TransientOutputs,
                    Stale,
                    "Transient output captured during the drop is flagged stale.",
                ),
            ],
            CHRONO_PARTIAL_PROOF,
            "Debug session reconnect after a transport drop: variables and breakpoints \
             preserved; debug state invalidated; queued cells and transient outputs stale.",
        ),
        // 3. Notebook kernel restart preserving breakpoints.
        RestartConsequenceRecord::build(
            RC_KERNEL_RESTART,
            KernelRestart,
            "notebook.kernel.session:python_local:0001",
            vec![
                ConsequenceEntry::build(
                    Variables,
                    Lost,
                    "Kernel variables were cleared on restart.",
                ),
                ConsequenceEntry::build(
                    QueuedCells,
                    Lost,
                    "The pending execution queue was cleared.",
                ),
                ConsequenceEntry::build(
                    DebugState,
                    Lost,
                    "The debugger bridge reset to a fresh kernel session.",
                ),
                ConsequenceEntry::build(
                    Breakpoints,
                    Preserved,
                    "Breakpoints were retained across the restart.",
                ),
                ConsequenceEntry::build(
                    TransientOutputs,
                    Lost,
                    "Transient cell outputs were discarded.",
                ),
            ],
            KERNEL_PRESERVED_PROOF,
            "Notebook kernel restart preserving breakpoints: variables, queued cells, debug \
             state, and transient outputs lost; breakpoints retained.",
        ),
        // 4. Notebook kernel restart with a fresh session: everything lost.
        RestartConsequenceRecord::build(
            RC_KERNEL_RESET,
            KernelRestart,
            "notebook.kernel.session:python_remote:0002",
            vec![
                ConsequenceEntry::build(
                    Variables,
                    Lost,
                    "Kernel variables were cleared on the fresh session.",
                ),
                ConsequenceEntry::build(
                    QueuedCells,
                    Lost,
                    "The pending execution queue was cleared.",
                ),
                ConsequenceEntry::build(
                    DebugState,
                    Lost,
                    "The debugger bridge reset with no retained state.",
                ),
                ConsequenceEntry::build(
                    Breakpoints,
                    Lost,
                    "Breakpoints were not retained on this kernel.",
                ),
                ConsequenceEntry::build(
                    TransientOutputs,
                    Lost,
                    "Transient cell outputs were discarded.",
                ),
            ],
            KERNEL_RESET_PROOF,
            "Notebook kernel restart with a fresh session: variables, queued cells, debug \
             state, breakpoints, and transient outputs all lost.",
        ),
        // 5. Notebook kernel transport lost, reconnect attempted: mixed dispositions.
        RestartConsequenceRecord::build(
            RC_TRANSPORT_LOST,
            TransportLostReconnect,
            "notebook.kernel.session:managed:0003",
            vec![
                ConsequenceEntry::build(
                    Variables,
                    Stale,
                    "Variable state predates the transport drop and is stale.",
                ),
                ConsequenceEntry::build(
                    QueuedCells,
                    Invalidated,
                    "Queued cells were invalidated by the dropped transport.",
                ),
                ConsequenceEntry::build(
                    DebugState,
                    Invalidated,
                    "The debugger bridge state was invalidated.",
                ),
                ConsequenceEntry::build(
                    Breakpoints,
                    Preserved,
                    "Breakpoint definitions were preserved client-side.",
                ),
                ConsequenceEntry::build(
                    TransientOutputs,
                    Stale,
                    "Transient outputs from before the drop are flagged stale.",
                ),
            ],
            KERNEL_UNAVAILABLE_PROOF,
            "Notebook kernel transport lost with a reconnect attempt: breakpoints preserved; \
             debug state and queued cells invalidated; variables and transient outputs stale.",
        ),
        // 6. Replay capture reacquired: a recording reload.
        RestartConsequenceRecord::build(
            RC_REPLAY_REACQUIRE,
            ReplayReacquire,
            REPLAY_LOCAL,
            vec![
                ConsequenceEntry::build(
                    Variables,
                    Preserved,
                    "Recorded variable snapshots were reloaded intact.",
                ),
                ConsequenceEntry::build(
                    QueuedCells,
                    Lost,
                    "No live queue applies to an inspect-only replay.",
                ),
                ConsequenceEntry::build(
                    DebugState,
                    Preserved,
                    "The replay position and frame stack were restored.",
                ),
                ConsequenceEntry::build(
                    Breakpoints,
                    Preserved,
                    "Timeline breakpoints were preserved across the reacquire.",
                ),
                ConsequenceEntry::build(
                    TransientOutputs,
                    Stale,
                    "Replayed transient outputs are flagged as captured, not live.",
                ),
            ],
            REPLAY_LOCAL_PROOF,
            "Replay capture reacquired: recorded variables, replay position, and breakpoints \
             preserved; replayed transient outputs flagged stale; no live queue applies.",
        ),
    ]
}

fn invariant(invariant_id: &str, statement: &str, holds: bool) -> ParityInvariant {
    ParityInvariant {
        invariant_id: invariant_id.to_owned(),
        statement: statement.to_owned(),
        holds,
    }
}

fn compute_invariants(
    chronology: &[ChronologyCapabilityDescriptor],
    replays: &[ReplaySession],
    bookmarks: &[TimelineBookmark],
    kernels: &[NotebookKernelCapabilityDescriptor],
    links: &[CellFrameLink],
    consequences: &[RestartConsequenceRecord],
) -> Vec<ParityInvariant> {
    // Every capability descriptor carries one support pill whose flags equal the derivation
    // from its own support class and timeline state.
    let one_canonical_support_pill = chronology.iter().all(|c| {
        c.support_pill
            .matches_derivation(c.support_class, c.timeline_state)
    }) && replays.iter().all(|r| {
        r.support_pill
            .matches_derivation(r.support_class, r.timeline_state)
    }) && kernels.iter().all(|k| {
        k.support_pill
            .matches_derivation(k.support_class, k.timeline_state)
    });

    // The full support-class vocabulary is materialized across chronology descriptors.
    let support_class_complete = DebugSupportClass::ALL
        .iter()
        .all(|class| chronology.iter().any(|c| c.support_class == *class));

    // One support-class vocabulary is shared across live debug, replay, and notebook: each
    // family materializes at least one supported and one non-full-support member, proving
    // the same vocabulary is reused rather than re-expressed per surface.
    let one_shared_vocabulary = chronology.iter().any(|c| c.support_class.is_full_support())
        && replays.iter().any(|r| r.support_class.is_full_support())
        && kernels.iter().any(|k| k.support_class.is_full_support())
        && chronology
            .iter()
            .any(|c| !c.support_class.is_full_support())
        && kernels.iter().any(|k| !k.support_class.is_full_support());

    // An unsupported / policy-blocked backend backs no verbs and grants no time-travel, so a
    // neighboring backend's chronology is never inherited.
    let no_inherited_claims = chronology.iter().all(|c| {
        if c.support_pill.is_inert {
            c.supported_verbs.is_empty()
                && !c.support_pill.time_travel_available
                && !c.recorded_scope.records_history()
        } else {
            true
        }
    }) && kernels.iter().all(|k| {
        if k.support_pill.is_inert {
            k.supported_verbs.is_empty()
        } else {
            true
        }
    }) && chronology.iter().any(|c| c.support_pill.is_inert);

    // Time-travel verbs are backed only when the timeline supports it: a descriptor that
    // cannot time-travel backs none, and at least one descriptor backs reverse verbs.
    let verb_capability_pairs = || {
        chronology
            .iter()
            .map(|c| (&c.supported_verbs, c.support_pill.time_travel_available))
            .chain(
                replays
                    .iter()
                    .map(|r| (&r.supported_verbs, r.support_pill.time_travel_available)),
            )
    };
    let time_travel_verbs_backed_correctly = verb_capability_pairs().all(|(verbs, available)| {
        verbs.iter().all(|v| !v.requires_time_travel() || available)
            && (available != verbs.is_empty())
    }) && verb_capability_pairs()
        .any(|(verbs, _)| verbs.iter().any(|v| v.requires_time_travel()));

    // Replay sessions are always inspect-only and bound to a full capture identity sourced
    // from a chronology descriptor in the set.
    let replay_inspect_only_and_capture_bound = !replays.is_empty()
        && replays.iter().all(|r| {
            r.inspect_only
                && r.capture.is_fully_bound()
                && chronology
                    .iter()
                    .any(|c| c.descriptor_id == r.source_chronology_ref)
        });

    // Every timeline bookmark is bound to exactly one capture/session/target identity, that
    // identity matches its replay session, and the bookmark survives export and restore.
    let bookmarks_bound_and_survive = !bookmarks.is_empty()
        && bookmarks.iter().all(|b| {
            b.capture.is_fully_bound()
                && b.survives_support_export
                && b.survives_restore_review
                && replays.iter().any(|r| {
                    r.replay_session_id == b.replay_session_ref && r.capture.same_as(&b.capture)
                })
        });

    // Every restart/reconnect consequence itemizes the five subjects exactly once, so it is
    // never flattened into a generic banner.
    let consequences_itemized = !consequences.is_empty()
        && consequences.iter().all(|c| {
            c.itemizes_every_subject()
                && c.entries
                    .iter()
                    .all(|e| e.is_consistent() && !e.detail.is_empty())
        });

    // Every consequence subject the spec names — variables, queued cells, debug state,
    // breakpoints, transient outputs — is covered in every consequence record.
    let consequence_subjects_complete = consequences.iter().all(|c| {
        ConsequenceSubject::ALL
            .iter()
            .all(|s| c.disposition_for(*s).is_some())
    });

    // The full disposition vocabulary (preserved, lost, invalidated, stale) is materialized
    // across consequence entries.
    let disposition_vocabulary_complete = ConsequenceDisposition::ALL.iter().all(|d| {
        consequences
            .iter()
            .flat_map(|c| c.entries.iter())
            .any(|e| e.disposition == *d)
    });

    // The full trigger vocabulary is materialized across consequence records.
    let trigger_vocabulary_complete = ConsequenceTrigger::ALL
        .iter()
        .all(|t| consequences.iter().any(|c| c.trigger == *t));

    // Restart/reconnect consequences exist for notebook, debug, and replay sessions — not
    // just one of them.
    let consequences_cover_all_session_kinds = consequences.iter().any(|c| {
        matches!(
            c.trigger,
            ConsequenceTrigger::KernelRestart | ConsequenceTrigger::TransportLostReconnect
        )
    }) && consequences.iter().any(|c| {
        matches!(
            c.trigger,
            ConsequenceTrigger::SessionRestart | ConsequenceTrigger::Reconnect
        )
    }) && consequences
        .iter()
        .any(|c| c.trigger == ConsequenceTrigger::ReplayReacquire);

    // A frame-to-cell link renders exact only when its mapping is exact and supported; a
    // degraded or unsupported mapping is never drawn exact, and an exact case exists.
    let link_exact_only_when_exact_and_supported = links
        .iter()
        .all(|l| l.renders_exact_link == (l.fidelity.is_exact() && l.support_class.permits_use()))
        && links.iter().any(|l| l.renders_exact_link)
        && links
            .iter()
            .any(|l| !l.fidelity.is_exact() && !l.renders_exact_link);

    // The full cell-link fidelity vocabulary is materialized.
    let link_fidelity_complete = CellLinkFidelity::ALL
        .iter()
        .all(|f| links.iter().any(|l| l.fidelity == *f));

    // Every notebook kernel and cell-frame link resolves its cross-references within the set.
    let notebook_linkage_resolves = links
        .iter()
        .all(|l| kernels.iter().any(|k| k.kernel_id == l.kernel_ref))
        && kernels.iter().all(|k| {
            consequences
                .iter()
                .any(|c| c.consequence_id == k.restart_consequence_ref)
        });

    // Every record retains its typed tokens and cites an export-safe proof packet, so support
    // export never flattens it into rendered chrome.
    let export_retains_state = chronology.iter().all(|c| {
        !c.support_pill.support_class_token.is_empty()
            && !c.proof_packet_ref.is_empty()
            && is_export_safe_ref(&c.proof_packet_ref)
    }) && replays
        .iter()
        .all(|r| !r.proof_packet_ref.is_empty() && is_export_safe_ref(&r.proof_packet_ref))
        && bookmarks
            .iter()
            .all(|b| !b.proof_packet_ref.is_empty() && is_export_safe_ref(&b.proof_packet_ref))
        && kernels
            .iter()
            .all(|k| !k.proof_packet_ref.is_empty() && is_export_safe_ref(&k.proof_packet_ref))
        && links
            .iter()
            .all(|l| !l.proof_packet_ref.is_empty() && is_export_safe_ref(&l.proof_packet_ref))
        && consequences
            .iter()
            .all(|c| !c.proof_packet_ref.is_empty() && is_export_safe_ref(&c.proof_packet_ref));

    vec![
        invariant(
            "capability.one_canonical_support_pill",
            "Every chronology, replay, and notebook-kernel descriptor carries exactly one support \
             pill whose support-class and timeline tokens come from the frozen vocabulary and whose \
             flags equal their derivation.",
            one_canonical_support_pill,
        ),
        invariant(
            "capability.support_class_vocabulary_complete",
            "Supported, limited, unavailable, and policy-blocked are all materialized.",
            support_class_complete,
        ),
        invariant(
            "capability.one_shared_support_vocabulary",
            "Live debug, replay, and notebook surfaces reuse one support-class vocabulary rather \
             than re-expressing support per surface.",
            one_shared_vocabulary,
        ),
        invariant(
            "capability.no_inherited_claims_across_backends",
            "An unavailable or policy-blocked backend backs no verbs, grants no time-travel, and \
             records no history, so an unsupported runtime never inherits a neighbor's chronology \
             or notebook-debug claim.",
            no_inherited_claims,
        ),
        invariant(
            "capability.time_travel_verbs_backed_only_when_replayable",
            "A time-travel verb is backed only when a recorded/replayable timeline supports it; a \
             descriptor that cannot time-travel backs no verbs.",
            time_travel_verbs_backed_correctly,
        ),
        invariant(
            "replay.inspect_only_and_capture_bound",
            "Every replay session is inspect-only, bound to a full capture identity, and sourced \
             from a chronology descriptor in the set.",
            replay_inspect_only_and_capture_bound,
        ),
        invariant(
            "bookmark.bound_to_one_capture_and_survives_export",
            "Every timeline bookmark is bound to one capture/session/target identity that matches \
             its replay session and survives support export and restore review.",
            bookmarks_bound_and_survive,
        ),
        invariant(
            "consequence.itemized_never_flattened",
            "Every restart/reconnect consequence itemizes variables, queued cells, debug state, \
             breakpoints, and transient outputs exactly once, never flattening them into a generic \
             banner.",
            consequences_itemized,
        ),
        invariant(
            "consequence.required_subjects_complete",
            "Every restart/reconnect consequence explains what happened to variables, queued cells, \
             debug state, breakpoints, and transient outputs.",
            consequence_subjects_complete,
        ),
        invariant(
            "consequence.disposition_vocabulary_complete",
            "Preserved, lost, invalidated, and stale are all materialized across consequence \
             entries.",
            disposition_vocabulary_complete,
        ),
        invariant(
            "consequence.trigger_vocabulary_complete",
            "Session restart, reconnect, kernel restart, transport-lost reconnect, and replay \
             reacquire are all materialized.",
            trigger_vocabulary_complete,
        ),
        invariant(
            "consequence.covers_notebook_debug_and_replay",
            "Restart/reconnect consequences exist for notebook, debug, and replay sessions, not \
             just one of them.",
            consequences_cover_all_session_kinds,
        ),
        invariant(
            "link.exact_only_when_exact_and_supported",
            "A frame-to-cell link renders exact only when its mapping is exact and supported; an \
             approximate, stale, or unmapped link is never drawn exact.",
            link_exact_only_when_exact_and_supported,
        ),
        invariant(
            "link.fidelity_vocabulary_complete",
            "Exact, approximate, stale, and unmapped cell-link fidelities are all materialized.",
            link_fidelity_complete,
        ),
        invariant(
            "set.notebook_linkage_resolves",
            "Every cell-frame link resolves to a kernel in the set, and every kernel resolves its \
             restart-consequence reference.",
            notebook_linkage_resolves,
        ),
        invariant(
            "set.export_retains_capability_state",
            "Every record retains its typed support/timeline/disposition tokens and cites an \
             export-safe proof packet, so support export never flattens it into chrome.",
            export_retains_state,
        ),
    ]
}

// ---------------------------------------------------------------------------
// Human-readable projection.
// ---------------------------------------------------------------------------

/// Renders the chronology/replay/parity set as human-readable lines for CLI/headless and
/// support.
pub fn m5_chronology_replay_parity_lines(set: &ChronologyReplayParitySet) -> Vec<String> {
    let mut lines = Vec::new();
    lines.push(format!(
        "M5 chronology, replay, timeline bookmarks & notebook-debug parity — {} ({})",
        set.set_id, set.as_of
    ));
    lines.push(set.summary.clone());
    lines.push(format!(
        "Chronology: {}  Replays: {}  Bookmarks: {}  Kernels: {}  Links: {}  Consequences: {}  Invariants: {}",
        set.chronology_capabilities.len(),
        set.replay_sessions.len(),
        set.timeline_bookmarks.len(),
        set.notebook_kernels.len(),
        set.cell_frame_links.len(),
        set.restart_consequences.len(),
        set.invariants.len(),
    ));

    lines.push("Chronology capabilities:".to_owned());
    for c in &set.chronology_capabilities {
        lines.push(format!(
            "  - {} family={} support={} timeline={} time_travel={} verbs={} parity={}",
            c.descriptor_id,
            c.backend_family_token,
            c.support_class_token,
            c.timeline_state_token,
            c.support_pill.time_travel_available,
            c.supported_verbs.len(),
            c.notebook_parity_token,
        ));
        lines.push(format!("      {}", c.summary));
        lines.push(format!("      proof: {}", c.proof_packet_ref));
    }

    lines.push("Replay sessions:".to_owned());
    for r in &set.replay_sessions {
        lines.push(format!(
            "  - {} family={} support={} timeline={} inspect_only={} capture={} verbs={}",
            r.replay_session_id,
            r.backend_family_token,
            r.support_class_token,
            r.timeline_state_token,
            r.inspect_only,
            r.capture.capture_id,
            r.supported_verbs.len(),
        ));
        lines.push(format!("      {}", r.summary));
        lines.push(format!("      proof: {}", r.proof_packet_ref));
    }

    lines.push("Timeline bookmarks:".to_owned());
    for b in &set.timeline_bookmarks {
        lines.push(format!(
            "  - {} kind={} capture={} replay={} export_safe={} restore_safe={}",
            b.bookmark_id,
            b.kind_token,
            b.capture.capture_id,
            b.replay_session_ref,
            b.survives_support_export,
            b.survives_restore_review,
        ));
        lines.push(format!("      {}", b.summary));
    }

    lines.push("Notebook kernels:".to_owned());
    for k in &set.notebook_kernels {
        lines.push(format!(
            "  - {} support={} timeline={} verbs={} restart_consequence={}",
            k.kernel_id,
            k.support_class_token,
            k.timeline_state_token,
            k.supported_verbs.len(),
            k.restart_consequence_ref,
        ));
        lines.push(format!("      {}", k.summary));
        lines.push(format!("      proof: {}", k.proof_packet_ref));
    }

    lines.push("Cell-frame links:".to_owned());
    for l in &set.cell_frame_links {
        lines.push(format!(
            "  - {} fidelity={} support={} renders_exact={} frame={} cell={}",
            l.link_id,
            l.fidelity_token,
            l.support_class_token,
            l.renders_exact_link,
            l.frame_ref,
            l.cell_ref,
        ));
        lines.push(format!("      {}", l.summary));
    }

    lines.push("Restart/reconnect consequences:".to_owned());
    for c in &set.restart_consequences {
        lines.push(format!(
            "  - {} trigger={} session={}",
            c.consequence_id, c.trigger_token, c.session_ref,
        ));
        for e in &c.entries {
            lines.push(format!(
                "      {} -> {}: {}",
                e.subject_token, e.disposition_token, e.detail
            ));
        }
        lines.push(format!("      proof: {}", c.proof_packet_ref));
    }

    lines.push("Invariants:".to_owned());
    for i in &set.invariants {
        lines.push(format!(
            "  - [{}] {}",
            if i.holds { "ok" } else { "FAIL" },
            i.invariant_id
        ));
    }

    lines
}
