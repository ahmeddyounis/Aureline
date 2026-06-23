//! M5 debug-contracts matrix: the frozen, typed contract for Aureline's
//! debug-session, attach-target, breakpoint/watch/evaluate, console, chronology,
//! replay, and notebook-debug-parity truth.
//!
//! Aureline debugs against a governed object model, and debug truth stays
//! explicit and replay-safe. Launch, attach, core-file, replay, and inspect-only
//! sessions stay distinct; a breakpoint that never bound is not drawn as a
//! confirmed stop; a stack frame that maps approximately is not shown as an exact
//! source line; a variable captured at a prior stop never masquerades as the live
//! value of a running target; an evaluation discloses its side-effect risk before
//! it runs; notebook, debugger, and replay surfaces share one support vocabulary;
//! and a restored layout never implies reacquired authority over a target. The
//! objects that carry that truth already exist across the runtime, notebook,
//! profiler, and debug crates, each with a typed record and a boundary schema.
//! What was still implicit was a single place that names the debugger object
//! *families*, freezes their stable identifiers and required fields, pins one
//! controlled vocabulary across session modes, breakpoint/mapping states, variable
//! freshness, evaluate purity, mapping fidelity, and restore/reattach posture,
//! maps each object to the proof packet that keeps it current, and states the
//! invariants every debugger-facing surface must hold. This lane is that place.
//!
//! The matrix does four things:
//!
//! 1. **Names the debugger object families** ([`DebugObjectClass`]) and, for each,
//!    cites the canonical boundary schema(s) it binds, the crate module that
//!    already produces that truth, the required fields it must carry, the
//!    qualification states it can show, and the
//!    [`proof packet`](DebugObjectEntry::proof_packet_ref) that keeps it current —
//!    so notebook, profiler, incident, support, AI, and core debug surfaces point
//!    at the same object model rather than re-expressing debug truth ad hoc.
//! 2. **Freezes one qualification-state vocabulary** ([`DebugStateClass`]) spanning
//!    launch/attach/core-file/replay/inspect-only session modes, breakpoint
//!    verification and mapping states, variable freshness, evaluate purity classes,
//!    frame-mapping fidelity, and restore/reattach posture. Each state carries
//!    computed honesty flags: whether it requires disclosure, whether it implies
//!    live authority, and whether it discloses side-effect risk.
//! 3. **Defines the controlled vocabulary** ([`DebugVocabulary`]) the spec
//!    requires: session mode, breakpoint/mapping state, variable freshness, evaluate
//!    purity, mapping fidelity, and restore/reattach posture. Each object declares
//!    which axes it binds.
//! 4. **Covers every consumer surface** ([`DebugConsumer`]): the core debugger,
//!    notebook debug, profiler, incident review, support export, AI context, review
//!    workspace, CLI/headless, and docs/help surfaces that render these objects.
//!
//! [`m5_debug_contracts_matrix`] is the canonical binding: it builds the matrix
//! deterministically and computes each [`DebugContractInvariant`]'s `holds` flag
//! from the built objects and states, so the checked-in fixture and the freeze gate
//! freeze the contract byte-for-byte and an inconsistent edit flips an invariant
//! and fails CI. In particular [`DebugContractInvariant`]
//! `debug_contracts.proof_packet_mapped` flips false the moment a claimed debugger
//! object lacks a mapped proof packet, so stable promotion cannot harden a debugger
//! claim without current proof. The record carries no source bodies, raw paths,
//! provider payloads, URLs, hostnames, or credentials — only opaque object refs,
//! stable tokens, and short reviewable sentences — so it is safe for support export.

use serde::{Deserialize, Serialize};

#[cfg(test)]
mod tests;

/// Schema version for the M5 debug-contracts matrix.
pub const M5_DEBUG_CONTRACTS_SCHEMA_VERSION: u32 = 1;

/// Schema reference for the M5 debug-contracts matrix.
pub const M5_DEBUG_CONTRACTS_SCHEMA_REF: &str = "schemas/debug/m5_debug_contracts.schema.json";

/// Stable record-kind tag for the M5 debug-contracts matrix.
pub const M5_DEBUG_CONTRACTS_RECORD_KIND: &str = "m5_debug_contracts_matrix";

/// Stable id for the canonical M5 debug-contracts matrix.
pub const M5_DEBUG_CONTRACTS_MATRIX_ID: &str = "m5-debug-contracts:matrix:0001";

/// Evaluation stamp for the canonical matrix. Held as a constant so the canonical
/// binding stays deterministic and the fixture freezes byte-for-byte.
pub const M5_DEBUG_CONTRACTS_AS_OF: &str = "2026-06-23T00:00:00Z";

/// The freeze gate that keeps the matrix binding current. Stable promotion runs
/// this gate; it fails when the in-code matrix drifts from the checked-in fixture
/// or any invariant flips.
pub const M5_DEBUG_CONTRACTS_FREEZE_GATE_REF: &str =
    "crates/aureline-debug/tests/m5_debug_contracts.rs";

/// The checked-in canonical matrix fixture.
pub const M5_DEBUG_CONTRACTS_FIXTURE_REF: &str =
    "fixtures/debug/m5_debug_contracts/canonical_matrix.json";

/// The contract narrative document.
pub const M5_DEBUG_CONTRACTS_DOC_REF: &str = "docs/debug/m5_debug_contracts.md";

/// The human-readable evidence companion artifact.
pub const M5_DEBUG_CONTRACTS_ARTIFACT_REF: &str = "artifacts/debug/m5_debug_contracts.md";

// ---------------------------------------------------------------------------
// Debugger object families.
// ---------------------------------------------------------------------------

/// The closed set of governed debugger object families this matrix freezes.
///
/// Each family is one governed debugger object. Adding a family is a breaking
/// change to the matrix; renaming one breaks every consumer that resolves an
/// object by token, so the tokens are frozen here.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DebugObjectClass {
    /// The debug session: a live or post-mortem session with a distinct mode —
    /// launch, attach, core-file, replay, or inspect-only — and the authority
    /// posture it currently holds.
    DebugSession,
    /// The attach target: the descriptor for the process, container, remote
    /// helper, core file, or replay capture a session attaches to or launches.
    AttachTarget,
    /// The breakpoint spec: one requested breakpoint plus its verification state
    /// and where it actually bound.
    BreakpointSpec,
    /// The frame mapping: the mapping from one stack frame to source, with its
    /// mapping fidelity.
    FrameMapping,
    /// The variable / watch snapshot: variables, scopes, and watch expressions
    /// captured at a stop, each carrying its freshness.
    VariableWatchSnapshot,
    /// The evaluate request / result: one evaluate or REPL request, its declared
    /// evaluation purity, and its result.
    EvaluateRequestResult,
    /// The console emission: one console or debug-output emission, its stream
    /// class, and whether it is live or replayed.
    ConsoleEmission,
    /// The chronology capability: the time-travel capability a session declares —
    /// capture state, replay support class, and recorded scope.
    ChronologyCapability,
    /// The replay session: an inspect-only session reconstructed from a recorded
    /// capture.
    ReplaySession,
    /// The notebook-debug parity record: the frame-to-cell linkage, kernel-bridge
    /// support state, and restart-consequence posture.
    NotebookDebugParity,
}

impl DebugObjectClass {
    /// All object families, in matrix order.
    pub const ALL: [Self; 10] = [
        Self::DebugSession,
        Self::AttachTarget,
        Self::BreakpointSpec,
        Self::FrameMapping,
        Self::VariableWatchSnapshot,
        Self::EvaluateRequestResult,
        Self::ConsoleEmission,
        Self::ChronologyCapability,
        Self::ReplaySession,
        Self::NotebookDebugParity,
    ];

    /// Stable snake_case token for this family.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::DebugSession => "debug_session",
            Self::AttachTarget => "attach_target",
            Self::BreakpointSpec => "breakpoint_spec",
            Self::FrameMapping => "frame_mapping",
            Self::VariableWatchSnapshot => "variable_watch_snapshot",
            Self::EvaluateRequestResult => "evaluate_request_result",
            Self::ConsoleEmission => "console_emission",
            Self::ChronologyCapability => "chronology_capability",
            Self::ReplaySession => "replay_session",
            Self::NotebookDebugParity => "notebook_debug_parity",
        }
    }

    /// Stable object id, namespaced so it is unique across the product.
    pub fn object_id(self) -> String {
        format!("debug_object.{}", self.as_str())
    }

    /// Human-readable label.
    pub const fn label(self) -> &'static str {
        match self {
            Self::DebugSession => "Debug session",
            Self::AttachTarget => "Attach target",
            Self::BreakpointSpec => "Breakpoint spec",
            Self::FrameMapping => "Frame mapping",
            Self::VariableWatchSnapshot => "Variable / watch snapshot",
            Self::EvaluateRequestResult => "Evaluate request / result",
            Self::ConsoleEmission => "Console emission",
            Self::ChronologyCapability => "Chronology capability",
            Self::ReplaySession => "Replay session",
            Self::NotebookDebugParity => "Notebook-debug parity",
        }
    }

    /// The three objects that carry one shared support vocabulary across notebook,
    /// debugger, and replay surfaces.
    pub const SHARED_SUPPORT_VOCABULARY_OBJECTS: [Self; 3] = [
        Self::ChronologyCapability,
        Self::ReplaySession,
        Self::NotebookDebugParity,
    ];
}

// ---------------------------------------------------------------------------
// Unified qualification-state vocabulary.
// ---------------------------------------------------------------------------

/// One shared qualification-state vocabulary spanning every debugger object.
///
/// The tokens span the launch/attach/core-file/replay/inspect-only session modes,
/// breakpoint verification and mapping states, variable freshness, evaluate purity
/// classes, frame-mapping fidelity, and restore/reattach posture the contract
/// requires. Each [`DebugStateTerm`] in the matrix carries computed honesty flags,
/// so this vocabulary never silently lets an inspect-only, stale, approximate, or
/// effectful state pose as a clean live success.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DebugStateClass {
    /// Session launched and owns the target process with live authority.
    SessionLaunch,
    /// Session attached to an already-running process with live authority.
    SessionAttach,
    /// Session opened a core / crash dump: inspect-only, no live authority.
    SessionCoreFile,
    /// Session replays a recorded capture: inspect-only, no live authority.
    SessionReplay,
    /// Session is inspect-only for another reason (read-only target or policy).
    SessionInspectOnly,
    /// Breakpoint verified and bound at its requested location.
    BreakpointVerified,
    /// Breakpoint accepted but not yet bound; pending verification.
    BreakpointPending,
    /// Breakpoint could not bind and remains unverified / unbound.
    BreakpointUnboundUnverified,
    /// Breakpoint bound at an adjusted location, disclosed as relocated.
    BreakpointMappingAdjusted,
    /// Breakpoint rejected by the target or adapter.
    BreakpointRejected,
    /// Value is live at the current stop.
    VariableLiveAtStop,
    /// Value was captured at a prior stop and is stale since the target resumed.
    VariableStaleSinceResume,
    /// Value is unavailable or optimized out.
    VariableUnavailableOptimizedOut,
    /// Evaluation is read-only and free of side effects.
    EvaluateSideEffectFree,
    /// Evaluation mutates target state; side-effect risk disclosed.
    EvaluateMutating,
    /// Evaluation has unknown side effects; risk disclosed.
    EvaluateUnknownSideEffects,
    /// Effectful evaluation blocked because the session is inspect-only.
    EvaluateBlockedInspectOnly,
    /// Frame maps exactly to current source.
    MappingExact,
    /// Frame maps approximately (line-only or drifted), disclosed.
    MappingApproximate,
    /// Frame resolves a symbol name only, without authoritative source lines.
    MappingSymbolOnly,
    /// Frame could not be mapped to source.
    MappingUnmapped,
    /// Layout restored from a prior session, but not reattached: no authority.
    RestoreLayoutOnlyNotReattached,
    /// Restore requires an explicit reattach before any live control.
    RestoreReattachRequired,
    /// Session genuinely reattached and reacquired live authority.
    RestoreReacquiredAuthority,
}

impl DebugStateClass {
    /// All states, in vocabulary order.
    pub const ALL: [Self; 24] = [
        Self::SessionLaunch,
        Self::SessionAttach,
        Self::SessionCoreFile,
        Self::SessionReplay,
        Self::SessionInspectOnly,
        Self::BreakpointVerified,
        Self::BreakpointPending,
        Self::BreakpointUnboundUnverified,
        Self::BreakpointMappingAdjusted,
        Self::BreakpointRejected,
        Self::VariableLiveAtStop,
        Self::VariableStaleSinceResume,
        Self::VariableUnavailableOptimizedOut,
        Self::EvaluateSideEffectFree,
        Self::EvaluateMutating,
        Self::EvaluateUnknownSideEffects,
        Self::EvaluateBlockedInspectOnly,
        Self::MappingExact,
        Self::MappingApproximate,
        Self::MappingSymbolOnly,
        Self::MappingUnmapped,
        Self::RestoreLayoutOnlyNotReattached,
        Self::RestoreReattachRequired,
        Self::RestoreReacquiredAuthority,
    ];

    /// The five distinct session modes that must stay distinct.
    pub const SESSION_MODES: [Self; 5] = [
        Self::SessionLaunch,
        Self::SessionAttach,
        Self::SessionCoreFile,
        Self::SessionReplay,
        Self::SessionInspectOnly,
    ];

    /// Stable snake_case token for this state.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SessionLaunch => "session_launch",
            Self::SessionAttach => "session_attach",
            Self::SessionCoreFile => "session_core_file",
            Self::SessionReplay => "session_replay",
            Self::SessionInspectOnly => "session_inspect_only",
            Self::BreakpointVerified => "breakpoint_verified",
            Self::BreakpointPending => "breakpoint_pending",
            Self::BreakpointUnboundUnverified => "breakpoint_unbound_unverified",
            Self::BreakpointMappingAdjusted => "breakpoint_mapping_adjusted",
            Self::BreakpointRejected => "breakpoint_rejected",
            Self::VariableLiveAtStop => "variable_live_at_stop",
            Self::VariableStaleSinceResume => "variable_stale_since_resume",
            Self::VariableUnavailableOptimizedOut => "variable_unavailable_optimized_out",
            Self::EvaluateSideEffectFree => "evaluate_side_effect_free",
            Self::EvaluateMutating => "evaluate_mutating",
            Self::EvaluateUnknownSideEffects => "evaluate_unknown_side_effects",
            Self::EvaluateBlockedInspectOnly => "evaluate_blocked_inspect_only",
            Self::MappingExact => "mapping_exact",
            Self::MappingApproximate => "mapping_approximate",
            Self::MappingSymbolOnly => "mapping_symbol_only",
            Self::MappingUnmapped => "mapping_unmapped",
            Self::RestoreLayoutOnlyNotReattached => "restore_layout_only_not_reattached",
            Self::RestoreReattachRequired => "restore_reattach_required",
            Self::RestoreReacquiredAuthority => "restore_reacquired_authority",
        }
    }

    /// Human-readable label.
    pub const fn label(self) -> &'static str {
        match self {
            Self::SessionLaunch => "Session — launch",
            Self::SessionAttach => "Session — attach",
            Self::SessionCoreFile => "Session — core file (inspect-only)",
            Self::SessionReplay => "Session — replay (inspect-only)",
            Self::SessionInspectOnly => "Session — inspect-only",
            Self::BreakpointVerified => "Breakpoint — verified",
            Self::BreakpointPending => "Breakpoint — pending",
            Self::BreakpointUnboundUnverified => "Breakpoint — unbound / unverified",
            Self::BreakpointMappingAdjusted => "Breakpoint — relocated (adjusted)",
            Self::BreakpointRejected => "Breakpoint — rejected",
            Self::VariableLiveAtStop => "Variable — live at stop",
            Self::VariableStaleSinceResume => "Variable — stale since resume",
            Self::VariableUnavailableOptimizedOut => "Variable — unavailable / optimized out",
            Self::EvaluateSideEffectFree => "Evaluate — side-effect-free",
            Self::EvaluateMutating => "Evaluate — mutating",
            Self::EvaluateUnknownSideEffects => "Evaluate — unknown side effects",
            Self::EvaluateBlockedInspectOnly => "Evaluate — blocked (inspect-only)",
            Self::MappingExact => "Mapping — exact",
            Self::MappingApproximate => "Mapping — approximate",
            Self::MappingSymbolOnly => "Mapping — symbol-only",
            Self::MappingUnmapped => "Mapping — unmapped",
            Self::RestoreLayoutOnlyNotReattached => "Restore — layout only (not reattached)",
            Self::RestoreReattachRequired => "Restore — reattach required",
            Self::RestoreReacquiredAuthority => "Restore — reacquired authority",
        }
    }

    /// The controlled-vocabulary axis this state belongs to.
    pub const fn vocabulary(self) -> DebugVocabulary {
        match self {
            Self::SessionLaunch
            | Self::SessionAttach
            | Self::SessionCoreFile
            | Self::SessionReplay
            | Self::SessionInspectOnly => DebugVocabulary::SessionMode,
            Self::BreakpointVerified
            | Self::BreakpointPending
            | Self::BreakpointUnboundUnverified
            | Self::BreakpointMappingAdjusted
            | Self::BreakpointRejected => DebugVocabulary::BreakpointState,
            Self::VariableLiveAtStop
            | Self::VariableStaleSinceResume
            | Self::VariableUnavailableOptimizedOut => DebugVocabulary::VariableFreshness,
            Self::EvaluateSideEffectFree
            | Self::EvaluateMutating
            | Self::EvaluateUnknownSideEffects
            | Self::EvaluateBlockedInspectOnly => DebugVocabulary::EvaluatePurity,
            Self::MappingExact
            | Self::MappingApproximate
            | Self::MappingSymbolOnly
            | Self::MappingUnmapped => DebugVocabulary::MappingFidelity,
            Self::RestoreLayoutOnlyNotReattached
            | Self::RestoreReattachRequired
            | Self::RestoreReacquiredAuthority => DebugVocabulary::RestorePosture,
        }
    }

    /// Whether this state must render with a visible caveat: anything other than a
    /// clean confirmed-good state cannot be shown as an unquestioned success.
    pub const fn requires_disclosure(self) -> bool {
        !matches!(
            self,
            Self::SessionLaunch
                | Self::SessionAttach
                | Self::BreakpointVerified
                | Self::VariableLiveAtStop
                | Self::EvaluateSideEffectFree
                | Self::MappingExact
                | Self::RestoreReacquiredAuthority
        )
    }

    /// Whether this state asserts that the debugger currently holds live, current
    /// authority over a running target.
    pub const fn implies_live_authority(self) -> bool {
        matches!(
            self,
            Self::SessionLaunch
                | Self::SessionAttach
                | Self::VariableLiveAtStop
                | Self::RestoreReacquiredAuthority
        )
    }

    /// Whether this state discloses an evaluation side-effect risk.
    pub const fn discloses_side_effect_risk(self) -> bool {
        matches!(
            self,
            Self::EvaluateMutating | Self::EvaluateUnknownSideEffects
        )
    }

    /// Whether this state is an inspect-only posture that withholds live control.
    pub const fn is_inspect_only(self) -> bool {
        matches!(
            self,
            Self::SessionCoreFile
                | Self::SessionReplay
                | Self::SessionInspectOnly
                | Self::EvaluateBlockedInspectOnly
        )
    }

    /// Whether this state concerns authority over a target — a session mode, a
    /// restore/reattach posture, or a variable-liveness class.
    pub const fn concerns_authority(self) -> bool {
        matches!(
            self,
            Self::SessionLaunch
                | Self::SessionAttach
                | Self::SessionCoreFile
                | Self::SessionReplay
                | Self::SessionInspectOnly
                | Self::RestoreLayoutOnlyNotReattached
                | Self::RestoreReattachRequired
                | Self::RestoreReacquiredAuthority
                | Self::VariableLiveAtStop
                | Self::VariableStaleSinceResume
        )
    }
}

// ---------------------------------------------------------------------------
// Controlled vocabulary axes.
// ---------------------------------------------------------------------------

/// The named controlled-vocabulary axes this matrix defines and each object
/// declares it binds.
///
/// These are exactly the vocabularies the contract requires: session mode,
/// breakpoint/mapping state, variable freshness, evaluate purity, mapping fidelity,
/// and restore/reattach posture.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DebugVocabulary {
    /// Launch / attach / core-file / replay / inspect-only session mode.
    SessionMode,
    /// Breakpoint verification and bound-location mapping state.
    BreakpointState,
    /// Variable / watch value freshness.
    VariableFreshness,
    /// Evaluate / REPL purity class.
    EvaluatePurity,
    /// Frame-to-source mapping fidelity.
    MappingFidelity,
    /// Restore / reattach posture.
    RestorePosture,
}

impl DebugVocabulary {
    /// All controlled-vocabulary axes, in order.
    pub const ALL: [Self; 6] = [
        Self::SessionMode,
        Self::BreakpointState,
        Self::VariableFreshness,
        Self::EvaluatePurity,
        Self::MappingFidelity,
        Self::RestorePosture,
    ];

    /// Stable snake_case token for this axis.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SessionMode => "session_mode",
            Self::BreakpointState => "breakpoint_state",
            Self::VariableFreshness => "variable_freshness",
            Self::EvaluatePurity => "evaluate_purity",
            Self::MappingFidelity => "mapping_fidelity",
            Self::RestorePosture => "restore_posture",
        }
    }

    /// Human-readable label.
    pub const fn label(self) -> &'static str {
        match self {
            Self::SessionMode => "Session mode",
            Self::BreakpointState => "Breakpoint / mapping state",
            Self::VariableFreshness => "Variable freshness",
            Self::EvaluatePurity => "Evaluate purity",
            Self::MappingFidelity => "Mapping fidelity",
            Self::RestorePosture => "Restore / reattach posture",
        }
    }
}

// ---------------------------------------------------------------------------
// Consumer surfaces.
// ---------------------------------------------------------------------------

/// The surfaces that render a debugger object instead of restating debug truth ad
/// hoc.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DebugConsumer {
    /// The core debugger UI: session header, call stack, variables, breakpoints.
    CoreDebugger,
    /// The notebook debug surface: kernel bridge, frame-to-cell linkage.
    NotebookDebug,
    /// The profiler / trace / replay workspace.
    Profiler,
    /// The incident / crash review surface.
    IncidentReview,
    /// The support bundle / export packet.
    SupportExport,
    /// The AI context picker, composer, and tool-call evidence.
    AiContext,
    /// The review workspace and hosted review evidence.
    ReviewWorkspace,
    /// CLI, SDK, and headless inspection.
    CliHeadless,
    /// Docs, Help, and About truth surfaces.
    DocsHelp,
}

impl DebugConsumer {
    /// All consumer surfaces, in order.
    pub const ALL: [Self; 9] = [
        Self::CoreDebugger,
        Self::NotebookDebug,
        Self::Profiler,
        Self::IncidentReview,
        Self::SupportExport,
        Self::AiContext,
        Self::ReviewWorkspace,
        Self::CliHeadless,
        Self::DocsHelp,
    ];

    /// The six consumer surfaces the contract names explicitly: notebook, profiler,
    /// incident, support, AI, and core debug.
    pub const NAMED_REQUIRED: [Self; 6] = [
        Self::CoreDebugger,
        Self::NotebookDebug,
        Self::Profiler,
        Self::IncidentReview,
        Self::SupportExport,
        Self::AiContext,
    ];

    /// Stable snake_case token for this consumer.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::CoreDebugger => "core_debugger",
            Self::NotebookDebug => "notebook_debug",
            Self::Profiler => "profiler",
            Self::IncidentReview => "incident_review",
            Self::SupportExport => "support_export",
            Self::AiContext => "ai_context",
            Self::ReviewWorkspace => "review_workspace",
            Self::CliHeadless => "cli_headless",
            Self::DocsHelp => "docs_help",
        }
    }

    /// Human-readable label.
    pub const fn label(self) -> &'static str {
        match self {
            Self::CoreDebugger => "Core debugger",
            Self::NotebookDebug => "Notebook debug",
            Self::Profiler => "Profiler",
            Self::IncidentReview => "Incident review",
            Self::SupportExport => "Support export",
            Self::AiContext => "AI context",
            Self::ReviewWorkspace => "Review workspace",
            Self::CliHeadless => "CLI / headless",
            Self::DocsHelp => "Docs / Help",
        }
    }
}

/// Redaction posture applied to a debugger object on export surfaces.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DebugRedactionClass {
    /// Metadata-safe default — the export default for debug surfaces.
    MetadataSafeDefault,
    /// Summary text and stable refs only, never source or value bodies.
    SummaryAndRefsOnly,
    /// Operator-only restricted projection.
    OperatorOnlyRestricted,
    /// Internal-support restricted projection.
    InternalSupportRestricted,
}

impl DebugRedactionClass {
    /// Stable snake_case token for this redaction class.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::MetadataSafeDefault => "metadata_safe_default",
            Self::SummaryAndRefsOnly => "summary_and_refs_only",
            Self::OperatorOnlyRestricted => "operator_only_restricted",
            Self::InternalSupportRestricted => "internal_support_restricted",
        }
    }
}

// ---------------------------------------------------------------------------
// Record structs.
// ---------------------------------------------------------------------------

/// One `(token, label)` definition in the shared vocabulary.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DebugTokenDef {
    /// Stable token.
    pub token: String,
    /// Human-readable label.
    pub label: String,
}

/// The controlled-vocabulary token sets and bound source schemas this matrix
/// freezes.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DebugSharedVocabulary {
    /// Session modes (`session_mode` axis).
    pub session_modes: Vec<DebugTokenDef>,
    /// Breakpoint / mapping states (`breakpoint_state`).
    pub breakpoint_states: Vec<DebugTokenDef>,
    /// Variable freshness classes (`variable_freshness`).
    pub variable_freshness_classes: Vec<DebugTokenDef>,
    /// Evaluate purity classes (`evaluate_purity`).
    pub evaluate_purity_classes: Vec<DebugTokenDef>,
    /// Mapping fidelity classes (`mapping_fidelity`).
    pub mapping_fidelity_classes: Vec<DebugTokenDef>,
    /// Restore / reattach postures (`restore_posture`).
    pub restore_postures: Vec<DebugTokenDef>,
    /// Redaction classes governing export.
    pub redaction_classes: Vec<DebugTokenDef>,
    /// Consumer classes that render these objects.
    pub consumer_classes: Vec<DebugTokenDef>,
    /// The boundary schemas this matrix binds as truth sources.
    pub source_schema_refs: Vec<String>,
}

/// One state in the unified qualification vocabulary, with computed honesty flags.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DebugStateTerm {
    /// The state.
    pub state: DebugStateClass,
    /// Stable token (equals `state.as_str()`), surfaced for reuse by consumers.
    pub token: String,
    /// Human-readable label.
    pub label: String,
    /// The controlled-vocabulary axis this state belongs to.
    pub vocabulary: DebugVocabulary,
    /// Whether this state must render with a visible caveat.
    pub requires_disclosure: bool,
    /// Whether this state asserts the debugger holds live, current authority.
    pub implies_live_authority: bool,
    /// Whether this state discloses an evaluation side-effect risk.
    pub discloses_side_effect_risk: bool,
    /// Whether this state is an inspect-only posture that withholds live control.
    pub is_inspect_only: bool,
    /// Whether this state concerns authority over a target.
    pub concerns_authority: bool,
}

/// One required field a debugger object must carry.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DebugFieldDef {
    /// Stable field id (matches the producing struct field).
    pub field_id: String,
    /// Human-readable label.
    pub label: String,
    /// Whether the field is required on every instance of the object.
    pub required: bool,
}

/// One debugger object-family entry.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DebugObjectEntry {
    /// The object family.
    pub object: DebugObjectClass,
    /// Stable, namespaced object id.
    pub object_id: String,
    /// Human-readable label.
    pub label: String,
    /// One reviewable sentence describing the object.
    pub summary: String,
    /// The canonical boundary schema(s) this object binds.
    pub canonical_schema_refs: Vec<String>,
    /// The crate module(s) that already produce this truth.
    pub produced_by_refs: Vec<String>,
    /// The proof packet (contract, fixture, or evidence) that keeps this object
    /// current. Stable promotion fails when this is empty.
    pub proof_packet_ref: String,
    /// The consumers that render this object.
    pub consumed_by: Vec<DebugConsumer>,
    /// The qualification states from the unified vocabulary this object can show.
    pub applicable_states: Vec<DebugStateClass>,
    /// The controlled-vocabulary axes this object binds.
    pub controlled_vocabularies: Vec<DebugVocabulary>,
    /// The required fields this object must carry.
    pub required_fields: Vec<DebugFieldDef>,
    /// Whether the object always renders its state vocabulary rather than
    /// collapsing it into an undifferentiated success.
    pub state_always_visible: bool,
    /// Whether the object discloses its authority posture — live launch/attach
    /// versus core-file/replay/inspect-only — so a restored or replayed view never
    /// implies reacquired live control.
    pub discloses_authority_posture: bool,
    /// Whether the object tracks value/recording freshness so a stale capture is
    /// never shown as live.
    pub freshness_tracked: bool,
    /// Whether the object discloses evaluation side-effect risk before running.
    pub side_effect_disclosed: bool,
    /// Whether this object is source-attributed — it names a stable source or
    /// evidence anchor rather than asserting an unsourced fact.
    pub carries_source_attribution: bool,
    /// The field that carries that source attribution, if any.
    pub source_attribution_field: Option<String>,
    /// The default redaction posture on export.
    pub default_redaction: DebugRedactionClass,
    /// Whether the object is locally inspectable (never console-only / portal-only).
    pub locally_inspectable: bool,
    /// Whether the object is typed (never reduced to a prose-only or toast-only view).
    pub typed_not_prose_only: bool,
    /// One reviewable sentence stating the object's debug-truth rule.
    pub boundary_note: String,
}

impl DebugObjectEntry {
    /// Whether the object binds the named controlled-vocabulary axis.
    pub fn binds(&self, vocab: DebugVocabulary) -> bool {
        self.controlled_vocabularies.contains(&vocab)
    }

    /// Whether the object can show a given qualification state.
    pub fn can_show(&self, state: DebugStateClass) -> bool {
        self.applicable_states.contains(&state)
    }

    /// Whether the object can show any state that concerns authority but does not
    /// imply live authority — the case where authority posture must be disclosed.
    pub fn can_show_non_live_authority_state(&self) -> bool {
        self.applicable_states
            .iter()
            .any(|s| s.concerns_authority() && !s.implies_live_authority())
    }

    /// Whether the object has a required field with the given id.
    pub fn has_field(&self, field_id: &str) -> bool {
        self.required_fields.iter().any(|f| f.field_id == field_id)
    }
}

/// One frozen invariant, with a computed `holds` flag.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DebugContractInvariant {
    /// Stable invariant id.
    pub invariant_id: String,
    /// The invariant statement.
    pub statement: String,
    /// Whether the built matrix satisfies the invariant.
    pub holds: bool,
}

/// The frozen M5 debug-contracts matrix.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5DebugContractsMatrix {
    /// Stable record-kind tag.
    pub record_kind: String,
    /// Schema version.
    pub m5_debug_contracts_schema_version: u32,
    /// Schema reference.
    pub schema_ref: String,
    /// Stable matrix id.
    pub matrix_id: String,
    /// Evaluation stamp.
    pub as_of: String,
    /// The freeze gate that keeps the matrix binding current.
    pub freeze_gate_ref: String,
    /// One reviewable sentence summarizing the matrix.
    pub summary: String,
    /// The controlled-vocabulary token sets and bound source schemas.
    pub shared_vocabulary: DebugSharedVocabulary,
    /// The unified qualification-state vocabulary.
    pub state_vocabulary: Vec<DebugStateTerm>,
    /// The debugger object-family entries.
    pub objects: Vec<DebugObjectEntry>,
    /// The computed invariants.
    pub invariants: Vec<DebugContractInvariant>,
    /// Whether raw source bodies and payloads are excluded (always true).
    pub raw_payload_excluded: bool,
}

/// Error returned when the matrix fails a structural consistency check.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DebugContractsValidationError {
    /// The failed check.
    pub reason: String,
}

impl std::fmt::Display for DebugContractsValidationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "m5 debug-contracts matrix invalid: {}", self.reason)
    }
}

impl std::error::Error for DebugContractsValidationError {}

impl M5DebugContractsMatrix {
    /// Returns the entry for an object family, if present.
    pub fn object(&self, object: DebugObjectClass) -> Option<&DebugObjectEntry> {
        self.objects.iter().find(|o| o.object == object)
    }

    /// Returns the state term for a state, if present.
    pub fn state_term(&self, state: DebugStateClass) -> Option<&DebugStateTerm> {
        self.state_vocabulary.iter().find(|t| t.state == state)
    }

    /// Whether every computed invariant holds.
    pub fn all_invariants_hold(&self) -> bool {
        self.invariants.iter().all(|i| i.holds)
    }

    /// Whether the record is safe to place in a support export: raw payloads are
    /// excluded and every ref is a repo-relative object ref, never a URL, host,
    /// credential, or absolute path.
    pub fn is_support_export_safe(&self) -> bool {
        if !self.raw_payload_excluded {
            return false;
        }
        self.all_refs().all(is_export_safe_ref)
    }

    /// Every ref string carried by the matrix, for export-safety auditing.
    fn all_refs(&self) -> impl Iterator<Item = &str> {
        let from_shared = self
            .shared_vocabulary
            .source_schema_refs
            .iter()
            .map(String::as_str);
        let from_objects = self.objects.iter().flat_map(|o| {
            o.canonical_schema_refs
                .iter()
                .map(String::as_str)
                .chain(o.produced_by_refs.iter().map(String::as_str))
                .chain(std::iter::once(o.proof_packet_ref.as_str()))
        });
        let from_gate = std::iter::once(self.freeze_gate_ref.as_str());
        from_shared.chain(from_objects).chain(from_gate)
    }

    /// Re-checks structural consistency and returns an error on the first failure.
    /// Complements the computed [`DebugContractInvariant`]s with the uniqueness and
    /// completeness checks a consumer relies on.
    pub fn validate(&self) -> Result<(), DebugContractsValidationError> {
        let fail = |reason: String| Err(DebugContractsValidationError { reason });

        if self.record_kind != M5_DEBUG_CONTRACTS_RECORD_KIND {
            return fail(format!("unexpected record_kind {}", self.record_kind));
        }
        if self.schema_ref != M5_DEBUG_CONTRACTS_SCHEMA_REF {
            return fail(format!("unexpected schema_ref {}", self.schema_ref));
        }
        if self.m5_debug_contracts_schema_version != M5_DEBUG_CONTRACTS_SCHEMA_VERSION {
            return fail("unexpected schema version".to_owned());
        }
        if !self.raw_payload_excluded {
            return fail("raw_payload_excluded must be true".to_owned());
        }

        // Every object family and state is present exactly once.
        for object in DebugObjectClass::ALL {
            if self.objects.iter().filter(|o| o.object == object).count() != 1 {
                return fail(format!(
                    "object {} not present exactly once",
                    object.as_str()
                ));
            }
        }
        for state in DebugStateClass::ALL {
            if self
                .state_vocabulary
                .iter()
                .filter(|t| t.state == state)
                .count()
                != 1
            {
                return fail(format!("state {} not present exactly once", state.as_str()));
            }
        }

        // Stable ids and tokens are unique.
        if !all_unique(self.objects.iter().map(|o| o.object_id.as_str())) {
            return fail("object ids are not unique".to_owned());
        }
        if !all_unique(self.state_vocabulary.iter().map(|t| t.token.as_str())) {
            return fail("state tokens are not unique".to_owned());
        }

        // Per-object structural floor: typed, evidenced, fielded, proven.
        for entry in &self.objects {
            if entry.object_id != entry.object.object_id() {
                return fail(format!("object id mismatch for {}", entry.object.as_str()));
            }
            if entry.canonical_schema_refs.is_empty() {
                return fail(format!("object {} cites no schema", entry.object.as_str()));
            }
            if entry.produced_by_refs.is_empty() {
                return fail(format!("object {} has no producer", entry.object.as_str()));
            }
            if entry.proof_packet_ref.is_empty() {
                return fail(format!(
                    "object {} has no mapped proof packet",
                    entry.object.as_str()
                ));
            }
            if entry.applicable_states.is_empty() {
                return fail(format!(
                    "object {} declares no states",
                    entry.object.as_str()
                ));
            }
            if entry.controlled_vocabularies.is_empty() {
                return fail(format!(
                    "object {} binds no controlled vocabulary",
                    entry.object.as_str()
                ));
            }
            if entry.required_fields.is_empty() {
                return fail(format!(
                    "object {} declares no required fields",
                    entry.object.as_str()
                ));
            }
            if entry.consumed_by.is_empty() {
                return fail(format!("object {} has no consumer", entry.object.as_str()));
            }
            for state in &entry.applicable_states {
                if self.state_term(*state).is_none() {
                    return fail(format!(
                        "object {} references undefined state {}",
                        entry.object.as_str(),
                        state.as_str()
                    ));
                }
                // Each applicable state's axis must be a vocabulary the object binds.
                if !entry.binds(state.vocabulary()) {
                    return fail(format!(
                        "object {} can show state {} but does not bind its axis {}",
                        entry.object.as_str(),
                        state.as_str(),
                        state.vocabulary().as_str()
                    ));
                }
            }
        }

        if !self.is_support_export_safe() {
            return fail("matrix is not support-export safe".to_owned());
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

fn all_unique<'a>(iter: impl Iterator<Item = &'a str>) -> bool {
    let mut seen = std::collections::BTreeSet::new();
    iter.into_iter().all(|item| seen.insert(item))
}

/// Whether a ref is safe to export: a repo-relative object ref or opaque
/// `aureline://` handle, never a URL, host, credential, or absolute path.
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

/// Builds the canonical M5 debug-contracts matrix.
///
/// Deterministic: the same bytes every call. The invariant `holds` flags are
/// computed from the built objects and states, so an inconsistent edit flips an
/// invariant rather than silently passing.
pub fn m5_debug_contracts_matrix() -> M5DebugContractsMatrix {
    let state_vocabulary = build_state_vocabulary();
    let objects = build_objects();
    let shared_vocabulary = build_shared_vocabulary(&objects);
    let invariants = compute_invariants(&objects, &state_vocabulary);

    M5DebugContractsMatrix {
        record_kind: M5_DEBUG_CONTRACTS_RECORD_KIND.to_owned(),
        m5_debug_contracts_schema_version: M5_DEBUG_CONTRACTS_SCHEMA_VERSION,
        schema_ref: M5_DEBUG_CONTRACTS_SCHEMA_REF.to_owned(),
        matrix_id: M5_DEBUG_CONTRACTS_MATRIX_ID.to_owned(),
        as_of: M5_DEBUG_CONTRACTS_AS_OF.to_owned(),
        freeze_gate_ref: M5_DEBUG_CONTRACTS_FREEZE_GATE_REF.to_owned(),
        summary: "One frozen, typed matrix for Aureline's M5 debug truth — debug sessions, attach \
                  targets, breakpoint specs, frame mappings, variable/watch snapshots, evaluate \
                  requests, console emissions, chronology capabilities, replay sessions, and \
                  notebook-debug parity records — across the core debugger, notebook debug, \
                  profiler, incident review, support export, AI context, review workspace, \
                  CLI/headless, and docs/help surfaces, with each object mapped to the proof packet \
                  that keeps it current. Debug truth stays explicit and replay-safe: launch, \
                  attach, core-file, replay, and inspect-only sessions stay distinct; breakpoint \
                  and mapping states stay visible; variables and watches never masquerade as live \
                  when stale; evaluation discloses side-effect risk; notebook, debugger, and replay \
                  share one support vocabulary; and a restored layout never implies reacquired \
                  authority over a target."
            .to_owned(),
        shared_vocabulary,
        state_vocabulary,
        objects,
        invariants,
        raw_payload_excluded: true,
    }
}

fn build_state_vocabulary() -> Vec<DebugStateTerm> {
    DebugStateClass::ALL
        .iter()
        .map(|state| DebugStateTerm {
            state: *state,
            token: state.as_str().to_owned(),
            label: state.label().to_owned(),
            vocabulary: state.vocabulary(),
            requires_disclosure: state.requires_disclosure(),
            implies_live_authority: state.implies_live_authority(),
            discloses_side_effect_risk: state.discloses_side_effect_risk(),
            is_inspect_only: state.is_inspect_only(),
            concerns_authority: state.concerns_authority(),
        })
        .collect()
}

fn field(field_id: &str, label: &str, required: bool) -> DebugFieldDef {
    DebugFieldDef {
        field_id: field_id.to_owned(),
        label: label.to_owned(),
        required,
    }
}

fn strvec(items: &[&str]) -> Vec<String> {
    items.iter().map(|s| (*s).to_owned()).collect()
}

fn build_objects() -> Vec<DebugObjectEntry> {
    use DebugConsumer::*;
    use DebugStateClass::*;
    use DebugVocabulary::*;

    vec![
        DebugObjectEntry {
            object: DebugObjectClass::DebugSession,
            object_id: DebugObjectClass::DebugSession.object_id(),
            label: DebugObjectClass::DebugSession.label().to_owned(),
            summary: "A live or post-mortem debug session: a stable id, the distinct session mode \
                      (launch, attach, core-file, replay, or inspect-only), the target descriptor \
                      ref, the run state and stop reason, thread and frame refs, and the authority \
                      posture it currently holds with the freshness of that authority."
                .to_owned(),
            canonical_schema_refs: strvec(&[
                "schemas/runtime/debug_session.schema.json",
                "schemas/execution/debug_session.schema.json",
            ]),
            produced_by_refs: strvec(&[
                "crates/aureline-debug/src/m5_debug_session_descriptors/mod.rs",
                "crates/aureline-runtime/src/debug/host.rs",
                "crates/aureline-runtime/src/debug/records.rs",
            ]),
            proof_packet_ref: "fixtures/runtime/debugger_host_beta/protected_walk_local.json"
                .to_owned(),
            consumed_by: vec![
                CoreDebugger,
                NotebookDebug,
                Profiler,
                IncidentReview,
                SupportExport,
                AiContext,
                ReviewWorkspace,
                CliHeadless,
            ],
            applicable_states: vec![
                SessionLaunch,
                SessionAttach,
                SessionCoreFile,
                SessionReplay,
                SessionInspectOnly,
                RestoreLayoutOnlyNotReattached,
                RestoreReattachRequired,
                RestoreReacquiredAuthority,
            ],
            controlled_vocabularies: vec![SessionMode, RestorePosture],
            required_fields: vec![
                field("session_id", "Session id", true),
                field("session_mode", "Session mode", true),
                field("target_ref", "Target descriptor ref", true),
                field("run_state", "Run state", true),
                field("authority_posture", "Authority posture", true),
                field("authority_freshness", "Authority freshness", true),
                field("stop_reason", "Stop reason", false),
                field("thread_refs", "Thread refs", true),
                field("summary", "Export-safe summary", true),
            ],
            state_always_visible: true,
            discloses_authority_posture: true,
            freshness_tracked: true,
            side_effect_disclosed: false,
            carries_source_attribution: true,
            source_attribution_field: Some("target_ref".to_owned()),
            default_redaction: DebugRedactionClass::MetadataSafeDefault,
            locally_inspectable: true,
            typed_not_prose_only: true,
            boundary_note: "The session names its mode explicitly — launch, attach, core-file, \
                            replay, and inspect-only stay distinct — and discloses whether it holds \
                            live authority, so a restored layout or replayed session never implies \
                            reacquired live control of a target."
                .to_owned(),
        },
        DebugObjectEntry {
            object: DebugObjectClass::AttachTarget,
            object_id: DebugObjectClass::AttachTarget.object_id(),
            label: DebugObjectClass::AttachTarget.label().to_owned(),
            summary: "The descriptor for the process, container, remote helper, core file, or \
                      replay capture a session attaches to or launches: a stable id, the target \
                      kind, the transport posture, the attach-versus-launch posture, the negotiated \
                      capability set, and the negotiation evidence behind it."
                .to_owned(),
            canonical_schema_refs: strvec(&[
                "schemas/runtime/stabilize_debugger_host_and_adapter_negotiation_truth.schema.json",
                "schemas/remote/attach_session.schema.json",
            ]),
            produced_by_refs: strvec(&[
                "crates/aureline-debug/src/m5_debug_session_descriptors/mod.rs",
                "crates/aureline-runtime/src/stabilize_debugger_host_and_adapter_negotiation/mod.rs",
            ]),
            proof_packet_ref:
                "fixtures/runtime/m4/stabilize_debugger_host_and_adapter_negotiation/baseline_stable.json"
                    .to_owned(),
            consumed_by: vec![
                CoreDebugger,
                NotebookDebug,
                Profiler,
                IncidentReview,
                SupportExport,
                CliHeadless,
            ],
            applicable_states: vec![
                SessionLaunch,
                SessionAttach,
                SessionCoreFile,
                SessionReplay,
                SessionInspectOnly,
            ],
            controlled_vocabularies: vec![SessionMode],
            required_fields: vec![
                field("target_id", "Target id", true),
                field("target_kind", "Target kind", true),
                field("session_mode", "Session mode", true),
                field("transport_ref", "Transport ref", true),
                field("capability_set_ref", "Capability set ref", true),
                field("negotiation_evidence_ref", "Negotiation evidence ref", true),
                field("summary", "Export-safe summary", true),
            ],
            state_always_visible: true,
            discloses_authority_posture: true,
            freshness_tracked: false,
            side_effect_disclosed: false,
            carries_source_attribution: true,
            source_attribution_field: Some("negotiation_evidence_ref".to_owned()),
            default_redaction: DebugRedactionClass::MetadataSafeDefault,
            locally_inspectable: true,
            typed_not_prose_only: true,
            boundary_note: "A target descriptor records whether the session launched it or attached \
                            to an already-running process, and core-file and replay targets are \
                            marked inspect-only, so the surface never offers live control over a \
                            target that cannot accept it."
                .to_owned(),
        },
        DebugObjectEntry {
            object: DebugObjectClass::BreakpointSpec,
            object_id: DebugObjectClass::BreakpointSpec.object_id(),
            label: DebugObjectClass::BreakpointSpec.label().to_owned(),
            summary: "One requested breakpoint — line, conditional, function, logpoint, data, or \
                      exception — plus where it actually bound: a stable id, the requested location, \
                      the verification state, the adjusted bound location when relocated, the \
                      binding condition, and the disclosure when it stays pending or unbound."
                .to_owned(),
            canonical_schema_refs: strvec(&[
                "schemas/runtime/harden_breakpoint_call_stack_variables_watch_evaluate_and_truth.schema.json",
            ]),
            produced_by_refs: strvec(&[
                "crates/aureline-runtime/src/harden_breakpoint_call_stack_variables_watch_evaluate_and/mod.rs",
            ]),
            proof_packet_ref:
                "fixtures/runtime/m4/harden_breakpoint_call_stack_variables_watch_evaluate_and/baseline_stable.json"
                    .to_owned(),
            consumed_by: vec![
                CoreDebugger,
                NotebookDebug,
                IncidentReview,
                SupportExport,
                CliHeadless,
            ],
            applicable_states: vec![
                BreakpointVerified,
                BreakpointPending,
                BreakpointUnboundUnverified,
                BreakpointMappingAdjusted,
                BreakpointRejected,
            ],
            controlled_vocabularies: vec![BreakpointState],
            required_fields: vec![
                field("breakpoint_id", "Breakpoint id", true),
                field("requested_location_ref", "Requested location ref", true),
                field("breakpoint_kind", "Breakpoint kind", true),
                field("verification_state", "Verification state", true),
                field("bound_location_ref", "Bound location ref", false),
                field("condition", "Condition", false),
                field("disclosure_reason", "Disclosure reason", false),
                field("summary", "Export-safe summary", true),
            ],
            state_always_visible: true,
            discloses_authority_posture: false,
            freshness_tracked: false,
            side_effect_disclosed: false,
            carries_source_attribution: true,
            source_attribution_field: Some("requested_location_ref".to_owned()),
            default_redaction: DebugRedactionClass::MetadataSafeDefault,
            locally_inspectable: true,
            typed_not_prose_only: true,
            boundary_note: "Every breakpoint keeps its verification state visible — verified, \
                            pending, unbound, relocated, or rejected — so a breakpoint that bound at \
                            an adjusted line, or never bound at all, is never drawn as a confirmed \
                            stop."
                .to_owned(),
        },
        DebugObjectEntry {
            object: DebugObjectClass::FrameMapping,
            object_id: DebugObjectClass::FrameMapping.object_id(),
            label: DebugObjectClass::FrameMapping.label().to_owned(),
            summary: "The mapping from one stack frame to source: a stable id, the frame ref, the \
                      resolved source anchor, the mapping fidelity (exact, approximate, symbol-only, \
                      or unmapped), and the build/source identity behind that fidelity."
                .to_owned(),
            canonical_schema_refs: strvec(&[
                "schemas/execution/mapping_quality.schema.json",
                "schemas/debug/symbolication_contract.schema.json",
            ]),
            produced_by_refs: strvec(&[
                "crates/aureline-debug/src/symbolication/mod.rs",
                "crates/aureline-runtime/src/harden_breakpoint_call_stack_variables_watch_evaluate_and/mod.rs",
            ]),
            proof_packet_ref: "fixtures/debug/symbolication/exact_local_report.json".to_owned(),
            consumed_by: vec![
                CoreDebugger,
                NotebookDebug,
                Profiler,
                IncidentReview,
                SupportExport,
                AiContext,
                CliHeadless,
            ],
            applicable_states: vec![
                MappingExact,
                MappingApproximate,
                MappingSymbolOnly,
                MappingUnmapped,
            ],
            controlled_vocabularies: vec![MappingFidelity],
            required_fields: vec![
                field("mapping_id", "Mapping id", true),
                field("frame_ref", "Frame ref", true),
                field("source_anchor_ref", "Source anchor ref", false),
                field("mapping_fidelity", "Mapping fidelity", true),
                field("build_identity_ref", "Build identity ref", true),
                field("summary", "Export-safe summary", true),
            ],
            state_always_visible: true,
            discloses_authority_posture: false,
            freshness_tracked: false,
            side_effect_disclosed: false,
            carries_source_attribution: true,
            source_attribution_field: Some("source_anchor_ref".to_owned()),
            default_redaction: DebugRedactionClass::MetadataSafeDefault,
            locally_inspectable: true,
            typed_not_prose_only: true,
            boundary_note: "Each frame carries its mapping fidelity, so an approximate, symbol-only, \
                            or unmapped frame is labeled as such and an inexact mapping is never \
                            presented as an exact source line."
                .to_owned(),
        },
        DebugObjectEntry {
            object: DebugObjectClass::VariableWatchSnapshot,
            object_id: DebugObjectClass::VariableWatchSnapshot.object_id(),
            label: DebugObjectClass::VariableWatchSnapshot.label().to_owned(),
            summary: "A snapshot of variables, scopes, and watch expressions captured at a stop: a \
                      stable id, the owning frame/scope ref, the captured-at-stop ref, the entry \
                      set, and the freshness of each value — live at the current stop, stale since \
                      the target resumed, or unavailable / optimized out."
                .to_owned(),
            canonical_schema_refs: strvec(&[
                "schemas/execution/watch_controller_state.schema.json",
                "schemas/runtime/harden_breakpoint_call_stack_variables_watch_evaluate_and_truth.schema.json",
            ]),
            produced_by_refs: strvec(&[
                "crates/aureline-runtime/src/harden_breakpoint_call_stack_variables_watch_evaluate_and/mod.rs",
            ]),
            proof_packet_ref:
                "fixtures/runtime/m4/harden_breakpoint_call_stack_variables_watch_evaluate_and/baseline_stable.json"
                    .to_owned(),
            consumed_by: vec![
                CoreDebugger,
                NotebookDebug,
                IncidentReview,
                SupportExport,
                AiContext,
                CliHeadless,
            ],
            applicable_states: vec![
                VariableLiveAtStop,
                VariableStaleSinceResume,
                VariableUnavailableOptimizedOut,
            ],
            controlled_vocabularies: vec![VariableFreshness],
            required_fields: vec![
                field("snapshot_id", "Snapshot id", true),
                field("scope_ref", "Scope ref", true),
                field("captured_at_stop_ref", "Captured-at-stop ref", true),
                field("entries", "Entries", true),
                field("freshness", "Freshness", true),
                field("summary", "Export-safe summary", true),
            ],
            state_always_visible: true,
            discloses_authority_posture: true,
            freshness_tracked: true,
            side_effect_disclosed: false,
            carries_source_attribution: true,
            source_attribution_field: Some("scope_ref".to_owned()),
            default_redaction: DebugRedactionClass::SummaryAndRefsOnly,
            locally_inspectable: true,
            typed_not_prose_only: true,
            boundary_note: "Every value carries its freshness, so a value captured at a prior stop \
                            is marked stale-since-resume and never rendered as the live current \
                            value of a running target."
                .to_owned(),
        },
        DebugObjectEntry {
            object: DebugObjectClass::EvaluateRequestResult,
            object_id: DebugObjectClass::EvaluateRequestResult.object_id(),
            label: DebugObjectClass::EvaluateRequestResult.label().to_owned(),
            summary: "One evaluate / REPL request and its result: a stable id, the expression \
                      context (frame, scope, or session), the declared evaluation purity \
                      (side-effect-free, mutating, unknown, or blocked in inspect-only), the result \
                      ref, and the side-effect disclosure that accompanies it."
                .to_owned(),
            canonical_schema_refs: strvec(&[
                "schemas/runtime/harden_breakpoint_call_stack_variables_watch_evaluate_and_truth.schema.json",
            ]),
            produced_by_refs: strvec(&[
                "crates/aureline-runtime/src/harden_breakpoint_call_stack_variables_watch_evaluate_and/mod.rs",
            ]),
            proof_packet_ref:
                "fixtures/runtime/m4/harden_breakpoint_call_stack_variables_watch_evaluate_and/baseline_stable.json"
                    .to_owned(),
            consumed_by: vec![
                CoreDebugger,
                NotebookDebug,
                IncidentReview,
                SupportExport,
                AiContext,
                CliHeadless,
            ],
            applicable_states: vec![
                EvaluateSideEffectFree,
                EvaluateMutating,
                EvaluateUnknownSideEffects,
                EvaluateBlockedInspectOnly,
            ],
            controlled_vocabularies: vec![EvaluatePurity],
            required_fields: vec![
                field("request_id", "Request id", true),
                field("context_ref", "Context ref", true),
                field("purity_class", "Purity class", true),
                field("result_ref", "Result ref", false),
                field("side_effect_disclosure", "Side-effect disclosure", true),
                field("summary", "Export-safe summary", true),
            ],
            state_always_visible: true,
            discloses_authority_posture: false,
            freshness_tracked: false,
            side_effect_disclosed: true,
            carries_source_attribution: true,
            source_attribution_field: Some("context_ref".to_owned()),
            default_redaction: DebugRedactionClass::SummaryAndRefsOnly,
            locally_inspectable: true,
            typed_not_prose_only: true,
            boundary_note: "Every evaluate discloses its side-effect risk before it runs — a \
                            mutating or unknown-effect expression is flagged, and an inspect-only \
                            session blocks effectful evaluation rather than silently mutating a \
                            core-file or replay target."
                .to_owned(),
        },
        DebugObjectEntry {
            object: DebugObjectClass::ConsoleEmission,
            object_id: DebugObjectClass::ConsoleEmission.object_id(),
            label: DebugObjectClass::ConsoleEmission.label().to_owned(),
            summary: "One console / debug-output emission: a stable id, the owning session ref, the \
                      stream class (stdout, stderr, debug console, or telemetry), the source \
                      attribution, and whether the emission is live or replayed from a captured \
                      session, with the fidelity of any source link it offers."
                .to_owned(),
            canonical_schema_refs: strvec(&["schemas/runtime/console_event.schema.json"]),
            produced_by_refs: strvec(&[
                "crates/aureline-runtime/src/m5_task_event_envelope_bus/mod.rs",
                "crates/aureline-runtime/src/debug/records.rs",
            ]),
            proof_packet_ref:
                "fixtures/runtime/browser_inspection_cases/console_live_exact_mapping.yaml"
                    .to_owned(),
            consumed_by: vec![
                CoreDebugger,
                NotebookDebug,
                Profiler,
                IncidentReview,
                SupportExport,
                CliHeadless,
            ],
            applicable_states: vec![
                SessionLaunch,
                SessionAttach,
                SessionReplay,
                SessionInspectOnly,
                MappingExact,
                MappingApproximate,
                MappingUnmapped,
            ],
            controlled_vocabularies: vec![SessionMode, MappingFidelity],
            required_fields: vec![
                field("emission_id", "Emission id", true),
                field("session_ref", "Session ref", true),
                field("stream_class", "Stream class", true),
                field("origin_session_mode", "Origin session mode", true),
                field("source_anchor_ref", "Source anchor ref", false),
                field("summary", "Export-safe summary", true),
            ],
            state_always_visible: true,
            discloses_authority_posture: true,
            freshness_tracked: false,
            side_effect_disclosed: false,
            carries_source_attribution: true,
            source_attribution_field: Some("session_ref".to_owned()),
            default_redaction: DebugRedactionClass::SummaryAndRefsOnly,
            locally_inspectable: true,
            typed_not_prose_only: true,
            boundary_note: "A console emission carries whether it is live or replayed from a \
                            captured session and the fidelity of any source link it offers, so a \
                            replayed line is never shown as live output and an approximate source \
                            link is never shown as exact."
                .to_owned(),
        },
        DebugObjectEntry {
            object: DebugObjectClass::ChronologyCapability,
            object_id: DebugObjectClass::ChronologyCapability.object_id(),
            label: DebugObjectClass::ChronologyCapability.label().to_owned(),
            summary: "The chronology / time-travel capability a session declares: a stable id, the \
                      capture state, the replay support class, the scope of what was recorded, the \
                      mapping fidelity of recorded frames, and the known limits — so reverse-step \
                      and timeline affordances are offered only where the recording supports them."
                .to_owned(),
            canonical_schema_refs: strvec(&[
                "schemas/debug/chronology-replay-support.schema.json",
                "schemas/debug/recording_mode.schema.json",
            ]),
            produced_by_refs: strvec(&[
                "crates/aureline-debug/src/qualify_chronology_capture_and_replay_support_classes/mod.rs",
            ]),
            proof_packet_ref: "fixtures/debug/chronology_cases/supported_recorded_session.yaml"
                .to_owned(),
            consumed_by: vec![
                CoreDebugger,
                NotebookDebug,
                Profiler,
                IncidentReview,
                SupportExport,
                CliHeadless,
            ],
            applicable_states: vec![
                SessionReplay,
                SessionInspectOnly,
                MappingExact,
                MappingApproximate,
                MappingUnmapped,
            ],
            controlled_vocabularies: vec![SessionMode, MappingFidelity],
            required_fields: vec![
                field("capability_id", "Capability id", true),
                field("session_ref", "Session ref", true),
                field("capture_state_ref", "Capture state ref", true),
                field("replay_support_class_ref", "Replay support class ref", true),
                field("recorded_scope_ref", "Recorded scope ref", true),
                field("known_limits", "Known limits", true),
                field("summary", "Export-safe summary", true),
            ],
            state_always_visible: true,
            discloses_authority_posture: true,
            freshness_tracked: true,
            side_effect_disclosed: false,
            carries_source_attribution: true,
            source_attribution_field: Some("session_ref".to_owned()),
            default_redaction: DebugRedactionClass::MetadataSafeDefault,
            locally_inspectable: true,
            typed_not_prose_only: true,
            boundary_note: "The capability names exactly what was recorded and its replay support \
                            class, so reverse-step and timeline controls appear only where the \
                            chronology supports them and a partial or expired recording is disclosed \
                            rather than presented as a full timeline."
                .to_owned(),
        },
        DebugObjectEntry {
            object: DebugObjectClass::ReplaySession,
            object_id: DebugObjectClass::ReplaySession.object_id(),
            label: DebugObjectClass::ReplaySession.label().to_owned(),
            summary: "A replay session reconstructed from a recorded capture: a stable id, the \
                      source capture ref, the replay scope, the inspect-only posture it holds, and \
                      the mapping fidelity of replayed frames to current source — never granting \
                      live mutation authority over the original target."
                .to_owned(),
            canonical_schema_refs: strvec(&[
                "schemas/runtime/replay_capability_alpha.schema.json",
                "schemas/runtime/runtime_replay_pack.schema.json",
                "schemas/debug/reverse_step_control.schema.json",
            ]),
            produced_by_refs: strvec(&[
                "crates/aureline-runtime/src/m5_replay_bundles/mod.rs",
                "crates/aureline-runtime/src/trace_replay_alpha/mod.rs",
            ]),
            proof_packet_ref: "fixtures/runtime/m3/replay_packets/local_task_exact_read_only.json"
                .to_owned(),
            consumed_by: vec![
                CoreDebugger,
                NotebookDebug,
                Profiler,
                IncidentReview,
                SupportExport,
                CliHeadless,
            ],
            applicable_states: vec![
                SessionReplay,
                SessionInspectOnly,
                MappingExact,
                MappingApproximate,
                MappingSymbolOnly,
                MappingUnmapped,
                RestoreLayoutOnlyNotReattached,
                RestoreReattachRequired,
            ],
            controlled_vocabularies: vec![SessionMode, MappingFidelity, RestorePosture],
            required_fields: vec![
                field("replay_id", "Replay id", true),
                field("source_capture_ref", "Source capture ref", true),
                field("replay_scope_ref", "Replay scope ref", true),
                field("session_mode", "Session mode", true),
                field("frame_mapping_fidelity", "Frame mapping fidelity", true),
                field("summary", "Export-safe summary", true),
            ],
            state_always_visible: true,
            discloses_authority_posture: true,
            freshness_tracked: true,
            side_effect_disclosed: false,
            carries_source_attribution: true,
            source_attribution_field: Some("source_capture_ref".to_owned()),
            default_redaction: DebugRedactionClass::MetadataSafeDefault,
            locally_inspectable: true,
            typed_not_prose_only: true,
            boundary_note: "A replay session stays inspect-only: it reconstructs a recorded capture \
                            and discloses replayed-frame mapping fidelity, but it never implies \
                            live, reacquired authority over the process the capture came from."
                .to_owned(),
        },
        DebugObjectEntry {
            object: DebugObjectClass::NotebookDebugParity,
            object_id: DebugObjectClass::NotebookDebugParity.object_id(),
            label: DebugObjectClass::NotebookDebugParity.label().to_owned(),
            summary: "The notebook-debug parity record linking a debugger frame to its notebook \
                      cell and recording kernel-restart consequences: a stable id, the frame ref, \
                      the cell linkage and its mapping fidelity, the supported / degraded / \
                      unsupported debug state of the kernel bridge, and the restart-consequence \
                      posture."
                .to_owned(),
            canonical_schema_refs: strvec(&[
                "schemas/notebook/ship_the_notebook_debugger_bridge_frame_to_cell_linkage_and_kernel_restart_consequence_records.schema.json",
                "schemas/notebook/implement_notebook_debugger_support_states_breakpoint_affordances_and_unsupported_state_cues.schema.json",
            ]),
            produced_by_refs: strvec(&[
                "crates/aureline-notebook/src/ship_the_notebook_debugger_bridge_frame_to_cell_linkage_and_kernel_restart_consequence_records/mod.rs",
                "crates/aureline-notebook/src/implement_notebook_debugger_support_states_breakpoint_affordances_and_unsupported_state_cues/mod.rs",
            ]),
            proof_packet_ref:
                "fixtures/notebook/m5/ship_the_notebook_debugger_bridge_frame_to_cell_linkage_and_kernel_restart_consequence_records/frame_cell_link_exact_match.json"
                    .to_owned(),
            consumed_by: vec![
                CoreDebugger,
                NotebookDebug,
                IncidentReview,
                SupportExport,
                AiContext,
                CliHeadless,
            ],
            applicable_states: vec![
                SessionLaunch,
                SessionAttach,
                SessionReplay,
                SessionInspectOnly,
                BreakpointVerified,
                BreakpointUnboundUnverified,
                MappingExact,
                MappingApproximate,
                MappingUnmapped,
                RestoreReattachRequired,
            ],
            controlled_vocabularies: vec![
                SessionMode,
                BreakpointState,
                MappingFidelity,
                RestorePosture,
            ],
            required_fields: vec![
                field("parity_id", "Parity id", true),
                field("frame_ref", "Frame ref", true),
                field("cell_linkage_ref", "Cell linkage ref", false),
                field("cell_mapping_fidelity", "Cell mapping fidelity", true),
                field("bridge_support_state", "Bridge support state", true),
                field("restart_consequence_posture", "Restart-consequence posture", true),
                field("summary", "Export-safe summary", true),
            ],
            state_always_visible: true,
            discloses_authority_posture: true,
            freshness_tracked: true,
            side_effect_disclosed: false,
            carries_source_attribution: true,
            source_attribution_field: Some("frame_ref".to_owned()),
            default_redaction: DebugRedactionClass::MetadataSafeDefault,
            locally_inspectable: true,
            typed_not_prose_only: true,
            boundary_note: "The parity record maps a debugger frame to its notebook cell with \
                            explicit fidelity and names the kernel bridge's support state and \
                            restart consequences, so an unsupported or degraded notebook debug \
                            surface is disclosed and a kernel restart's effect on debug state is \
                            never hidden."
                .to_owned(),
        },
    ]
}

fn build_shared_vocabulary(objects: &[DebugObjectEntry]) -> DebugSharedVocabulary {
    let axis_tokens = |axis: DebugVocabulary| -> Vec<DebugTokenDef> {
        DebugStateClass::ALL
            .iter()
            .filter(|s| s.vocabulary() == axis)
            .map(|s| DebugTokenDef {
                token: s.as_str().to_owned(),
                label: s.label().to_owned(),
            })
            .collect()
    };

    let mut source_schema_refs: Vec<String> = objects
        .iter()
        .flat_map(|o| o.canonical_schema_refs.iter().cloned())
        .collect();
    source_schema_refs.sort();
    source_schema_refs.dedup();

    let redaction_classes = [
        DebugRedactionClass::MetadataSafeDefault,
        DebugRedactionClass::SummaryAndRefsOnly,
        DebugRedactionClass::OperatorOnlyRestricted,
        DebugRedactionClass::InternalSupportRestricted,
    ]
    .iter()
    .map(|r| DebugTokenDef {
        token: r.as_str().to_owned(),
        label: redaction_label(*r).to_owned(),
    })
    .collect();

    DebugSharedVocabulary {
        session_modes: axis_tokens(DebugVocabulary::SessionMode),
        breakpoint_states: axis_tokens(DebugVocabulary::BreakpointState),
        variable_freshness_classes: axis_tokens(DebugVocabulary::VariableFreshness),
        evaluate_purity_classes: axis_tokens(DebugVocabulary::EvaluatePurity),
        mapping_fidelity_classes: axis_tokens(DebugVocabulary::MappingFidelity),
        restore_postures: axis_tokens(DebugVocabulary::RestorePosture),
        redaction_classes,
        consumer_classes: DebugConsumer::ALL
            .iter()
            .map(|c| DebugTokenDef {
                token: c.as_str().to_owned(),
                label: c.label().to_owned(),
            })
            .collect(),
        source_schema_refs,
    }
}

fn redaction_label(r: DebugRedactionClass) -> &'static str {
    match r {
        DebugRedactionClass::MetadataSafeDefault => "Metadata-safe default",
        DebugRedactionClass::SummaryAndRefsOnly => "Summary and refs only",
        DebugRedactionClass::OperatorOnlyRestricted => "Operator-only restricted",
        DebugRedactionClass::InternalSupportRestricted => "Internal-support restricted",
    }
}

// ---------------------------------------------------------------------------
// Invariants.
// ---------------------------------------------------------------------------

fn invariant(id: &str, statement: &str, holds: bool) -> DebugContractInvariant {
    DebugContractInvariant {
        invariant_id: id.to_owned(),
        statement: statement.to_owned(),
        holds,
    }
}

fn compute_invariants(
    objects: &[DebugObjectEntry],
    states: &[DebugStateTerm],
) -> Vec<DebugContractInvariant> {
    use DebugObjectClass::*;
    use DebugStateClass::*;
    use DebugVocabulary::*;

    let object = |class: DebugObjectClass| objects.iter().find(|o| o.object == class);
    let state_term = |state: DebugStateClass| states.iter().find(|t| t.state == state);

    let mut out = Vec::new();

    // Every object points at a canonical object and a producer.
    out.push(invariant(
        "debug_contracts.canonical_object_identity",
        "Every debugger object cites at least one canonical boundary schema and at least one \
         producing crate module, so notebook/profiler/incident/support/AI/core-debug point at the \
         same objects.",
        objects
            .iter()
            .all(|o| !o.canonical_schema_refs.is_empty() && !o.produced_by_refs.is_empty()),
    ));

    // Release-automation binding: every object maps to a proof packet. A claimed
    // debugger object with no mapped proof row flips this false and fails promotion.
    out.push(invariant(
        "debug_contracts.proof_packet_mapped",
        "Every debugger object maps to a non-empty proof packet that keeps it current, so stable \
         promotion fails when a claimed debugger-facing surface lacks a mapped proof row.",
        objects.iter().all(|o| !o.proof_packet_ref.is_empty()),
    ));

    // Session modes stay distinct, and launch/attach/core-file/replay/inspect-only
    // are all representable.
    out.push(invariant(
        "debug_contracts.session_modes_distinct",
        "Launch, attach, core-file, replay, and inspect-only are five distinct session-mode tokens, \
         and the debug-session object can show all five, so a launch is never conflated with an \
         attach and a replay is never conflated with a live session.",
        all_unique(DebugStateClass::SESSION_MODES.iter().map(|s| s.as_str()))
            && DebugStateClass::SESSION_MODES.len() == 5
            && object(DebugSession)
                .is_some_and(|o| DebugStateClass::SESSION_MODES.iter().all(|s| o.can_show(*s))),
    ));

    // Inspect-only modes carry no live authority and require disclosure.
    out.push(invariant(
        "debug_contracts.inspect_only_modes_carry_no_live_authority",
        "Core-file, replay, and inspect-only session modes never imply live authority and always \
         require disclosure, so an inspect-only session never offers live control over its target.",
        [SessionCoreFile, SessionReplay, SessionInspectOnly]
            .iter()
            .all(|s| {
                state_term(*s).is_some_and(|t| !t.implies_live_authority && t.requires_disclosure)
            }),
    ));

    // Breakpoint and mapping states stay visible.
    out.push(invariant(
        "debug_contracts.breakpoint_and_mapping_states_visible",
        "The breakpoint-spec and frame-mapping objects keep their state always visible and can show \
         pending, unbound, relocated, rejected, approximate, symbol-only, and unmapped states that \
         require disclosure, so a non-verified breakpoint or inexact frame mapping is never drawn \
         as confirmed.",
        object(BreakpointSpec).is_some_and(|o| {
            o.state_always_visible
                && o.binds(BreakpointState)
                && o.can_show(BreakpointPending)
                && o.can_show(BreakpointUnboundUnverified)
                && o.can_show(BreakpointMappingAdjusted)
                && o.can_show(BreakpointRejected)
        }) && object(FrameMapping).is_some_and(|o| {
            o.state_always_visible
                && o.binds(MappingFidelity)
                && o.can_show(MappingApproximate)
                && o.can_show(MappingSymbolOnly)
                && o.can_show(MappingUnmapped)
        }) && states
            .iter()
            .filter(|t| {
                matches!(t.vocabulary, BreakpointState | MappingFidelity)
                    && !matches!(t.state, BreakpointVerified | MappingExact)
            })
            .all(|t| t.requires_disclosure),
    ));

    // Variables never masquerade as live when stale.
    out.push(invariant(
        "debug_contracts.variables_never_masquerade_as_live",
        "A value captured at a prior stop is marked stale-since-resume — it does not imply live \
         authority and requires disclosure — and the variable/watch snapshot tracks freshness, so a \
         stale value is never rendered as the live value of a running target.",
        state_term(VariableStaleSinceResume)
            .is_some_and(|t| !t.implies_live_authority && t.requires_disclosure)
            && object(VariableWatchSnapshot).is_some_and(|o| {
                o.freshness_tracked
                    && o.binds(VariableFreshness)
                    && o.can_show(VariableLiveAtStop)
                    && o.can_show(VariableStaleSinceResume)
            }),
    ));

    // Evaluation discloses side-effect risk.
    out.push(invariant(
        "debug_contracts.evaluate_discloses_side_effects",
        "Mutating and unknown-side-effect evaluations disclose their side-effect risk, an \
         inspect-only session can block effectful evaluation, and the evaluate object marks \
         side-effect disclosure, so an evaluation never silently mutates target state.",
        [EvaluateMutating, EvaluateUnknownSideEffects]
            .iter()
            .all(|s| state_term(*s).is_some_and(|t| t.discloses_side_effect_risk))
            && object(EvaluateRequestResult).is_some_and(|o| {
                o.side_effect_disclosed
                    && o.binds(EvaluatePurity)
                    && o.can_show(EvaluateMutating)
                    && o.can_show(EvaluateUnknownSideEffects)
                    && o.can_show(EvaluateBlockedInspectOnly)
            }),
    ));

    // Notebook, debugger, and replay share one support vocabulary.
    out.push(invariant(
        "debug_contracts.shared_support_vocabulary",
        "The chronology-capability, replay-session, and notebook-debug-parity objects each bind the \
         shared session-mode and mapping-fidelity vocabularies and are all consumed by the support \
         export surface, so notebook, debugger, and replay speak one support vocabulary rather than \
         three.",
        DebugObjectClass::SHARED_SUPPORT_VOCABULARY_OBJECTS.iter().all(|class| {
            object(*class).is_some_and(|o| {
                o.binds(SessionMode)
                    && o.binds(MappingFidelity)
                    && o.consumed_by.contains(&DebugConsumer::SupportExport)
            })
        }),
    ));

    // Restored layouts never imply reacquired authority.
    out.push(invariant(
        "debug_contracts.restore_never_reacquires_authority",
        "A layout-only restore and a reattach-required restore never imply live authority, and only \
         an explicit reacquired-authority posture does, so a restored debug layout never implies the \
         debugger silently reacquired control of a target.",
        [RestoreLayoutOnlyNotReattached, RestoreReattachRequired]
            .iter()
            .all(|s| state_term(*s).is_some_and(|t| !t.implies_live_authority))
            && state_term(RestoreReacquiredAuthority)
                .is_some_and(|t| t.implies_live_authority)
            && object(DebugSession).is_some_and(|o| {
                o.discloses_authority_posture && o.can_show(RestoreLayoutOnlyNotReattached)
            }),
    ));

    // Every object that can be non-live in an authority sense discloses posture.
    out.push(invariant(
        "debug_contracts.authority_posture_disclosed",
        "Every object that can show a session, restore, or variable-liveness state that does not \
         imply live authority discloses its authority posture, so a replayed, restored, or stale \
         view never implies reacquired live control.",
        objects
            .iter()
            .all(|o| !o.can_show_non_live_authority_state() || o.discloses_authority_posture),
    ));

    // Every named controlled vocabulary is bound by some object.
    out.push(invariant(
        "debug_contracts.controlled_vocabulary_complete",
        "Each of the six named controlled vocabularies — session mode, breakpoint/mapping state, \
         variable freshness, evaluate purity, mapping fidelity, and restore/reattach posture — is \
         bound by at least one object.",
        DebugVocabulary::ALL
            .iter()
            .all(|v| objects.iter().any(|o| o.binds(*v))),
    ));

    // Required consumers all render the shared object model.
    out.push(invariant(
        "debug_contracts.consumers_share_object_model",
        "Every named consumer surface — core debugger, notebook debug, profiler, incident review, \
         support export, and AI context — renders at least one object, so each points at the shared \
         model rather than re-expressing debug truth ad hoc.",
        DebugConsumer::NAMED_REQUIRED
            .iter()
            .all(|c| objects.iter().any(|o| o.consumed_by.contains(c))),
    ));

    // Stable ids and tokens defined once and unique.
    out.push(invariant(
        "debug_contracts.stable_ids_unique",
        "Object ids and state tokens are each defined once and unique, so consumers can resolve an \
         object or state by a stable token.",
        all_unique(objects.iter().map(|o| o.object_id.as_str()))
            && all_unique(states.iter().map(|t| t.token.as_str())),
    ));

    // Every object family is present.
    out.push(invariant(
        "debug_contracts.all_objects_present",
        "Every governed debugger object family in the matrix is present exactly once.",
        DebugObjectClass::ALL
            .iter()
            .all(|class| objects.iter().filter(|o| o.object == *class).count() == 1),
    ));

    // Typed, never prose-only.
    out.push(invariant(
        "debug_contracts.typed_not_prose_only",
        "Every object is typed and locally inspectable: it carries state terms, required fields, \
         and schema refs, names a consumer, and is never reduced to a prose-only view.",
        objects.iter().all(|o| {
            o.typed_not_prose_only
                && o.locally_inspectable
                && !o.applicable_states.is_empty()
                && !o.required_fields.is_empty()
                && !o.canonical_schema_refs.is_empty()
                && !o.consumed_by.is_empty()
        }),
    ));

    out
}

// ---------------------------------------------------------------------------
// Human-readable projection.
// ---------------------------------------------------------------------------

/// Renders the matrix as human-readable lines for CLI/headless and support.
pub fn m5_debug_contracts_lines(matrix: &M5DebugContractsMatrix) -> Vec<String> {
    let mut lines = Vec::new();
    lines.push(format!(
        "M5 debug-contracts matrix — {} ({})",
        matrix.matrix_id, matrix.as_of
    ));
    lines.push(matrix.summary.clone());
    lines.push(format!(
        "Objects: {}  States: {}  Invariants: {}",
        matrix.objects.len(),
        matrix.state_vocabulary.len(),
        matrix.invariants.len(),
    ));

    lines.push("Objects:".to_owned());
    for o in &matrix.objects {
        let vocab: Vec<&str> = o
            .controlled_vocabularies
            .iter()
            .map(|v| v.as_str())
            .collect();
        let states: Vec<&str> = o.applicable_states.iter().map(|s| s.as_str()).collect();
        let consumers: Vec<&str> = o.consumed_by.iter().map(|c| c.as_str()).collect();
        lines.push(format!(
            "  - {} [{}] authority_disclosed={} freshness_tracked={} side_effect_disclosed={}",
            o.object.as_str(),
            o.object_id,
            o.discloses_authority_posture,
            o.freshness_tracked,
            o.side_effect_disclosed,
        ));
        lines.push(format!("      {}", o.summary));
        lines.push(format!("      vocabularies: {}", vocab.join(", ")));
        lines.push(format!("      states: {}", states.join(", ")));
        lines.push(format!("      consumers: {}", consumers.join(", ")));
        lines.push(format!(
            "      schemas: {}",
            o.canonical_schema_refs.join(", ")
        ));
        lines.push(format!("      proof: {}", o.proof_packet_ref));
    }

    lines.push("Invariants:".to_owned());
    for i in &matrix.invariants {
        lines.push(format!(
            "  - [{}] {}",
            if i.holds { "ok" } else { "FAIL" },
            i.invariant_id
        ));
    }

    lines
}
