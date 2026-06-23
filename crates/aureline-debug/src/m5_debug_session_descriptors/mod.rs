//! Typed debug-session and attach-target descriptors: the canonical M5 record
//! every debugger-capable surface reads to explain what was launched or attached,
//! against which target, with what current authority and adapter posture.
//!
//! The [`m5_debug_contracts`](crate::m5_debug_contracts) matrix *names* the
//! debugger object families and freezes their vocabulary. This lane *materializes*
//! two of those families as concrete, serde-serializable records — the
//! [`DebugSessionDescriptor`] and the [`AttachTargetDescriptor`] — and freezes a
//! canonical [`DebugSessionDescriptorSet`] that holds one descriptor per session
//! mode plus the restore/reattach cases that prove the no-silent-reattach rule.
//!
//! Debug truth stays explicit and replay-safe:
//!
//! - **Five distinct session modes.** Launch, attach, core-file, replay, and
//!   inspect-only are five [`DebugSessionModeClass`] tokens, never one generic
//!   debug session. Only launch and attach can hold live authority; core-file,
//!   replay, and inspect-only are post-mortem and inspect-only.
//! - **Target identity survives the picker.** An [`AttachTargetDescriptor`] carries
//!   the target/process identity, the local/remote/container/managed
//!   [`TargetBoundaryClass`], the [`TargetMutabilityClass`], the
//!   [`TargetPrivilegeClass`], the adapter ref/version, the [`AdapterDriftClass`],
//!   and the build/artifact identity. Each session echoes those into a
//!   [`TargetIdentityEcho`] and the freeze gate proves the echo equals the
//!   referenced target, so identity, mutability, privilege class, and adapter drift
//!   are preserved from picker through active session and export packet.
//! - **Adapter drift is a first-class label.** Adapter-drift, reconnect-required,
//!   inspect-only-no-adapter, and unsupported-skew are [`AdapterDriftClass`] states
//!   carried on launchers, headers, inspectors, restore surfaces, and support
//!   exports — a drifted or reconnect-required adapter never silently poses as a
//!   current one.
//! - **Restore never reacquires authority silently.** A session restore may reopen
//!   layout and history, but the [`ReentryPosture`] names the re-entry explicitly:
//!   a restored-layout-only or reattach-required posture holds no live authority,
//!   and only an explicit reattach reacquires it. The
//!   [`DebugSessionDescriptor::holds_live_authority`] flag is derived from the
//!   mode, the re-entry posture, and the adapter drift together, and the freeze gate
//!   re-derives it so a restored or replayed view can never claim live control.
//! - **One execution-context pipeline.** Every session carries a
//!   [`DebugEntrypointClass`] and a non-empty
//!   [`execution_context_id`](DebugSessionDescriptor::execution_context_id), so
//!   re-entry, restart, and open-in-support flows reuse one canonical session
//!   identity rather than minting notebook-only or support-only sessions.
//!
//! [`m5_debug_session_descriptor_set`] is the canonical binding: it builds the set
//! deterministically and computes each [`DescriptorInvariant`]'s `holds` flag from
//! the built descriptors, so the checked-in fixture and the freeze gate freeze the
//! contract byte-for-byte and an inconsistent edit flips an invariant and fails CI.
//! The record carries no source bodies, raw paths, provider payloads, URLs,
//! hostnames, or credentials — only opaque object refs, stable tokens, and short
//! reviewable sentences — so it is safe for support export.
//!
//! The cross-tool boundary schema is at
//! [`/schemas/debug/m5_debug_session_descriptors.schema.json`](../../../schemas/debug/m5_debug_session_descriptors.schema.json).
//! The checked-in stable packet is at
//! [`/fixtures/debug/m5_debug_session_descriptors/canonical_set.json`](../../../fixtures/debug/m5_debug_session_descriptors/canonical_set.json).
//! The reviewer-facing contract is at
//! [`/docs/debug/m5_debug_session_descriptors.md`](../../../docs/debug/m5_debug_session_descriptors.md).

use serde::{Deserialize, Serialize};

use crate::m5_debug_contracts::DebugConsumer;

#[cfg(test)]
mod tests;

/// Schema version for the M5 debug-session descriptor set.
pub const M5_DEBUG_SESSION_DESCRIPTORS_SCHEMA_VERSION: u32 = 1;

/// Schema reference for the M5 debug-session descriptor set.
pub const M5_DEBUG_SESSION_DESCRIPTORS_SCHEMA_REF: &str =
    "schemas/debug/m5_debug_session_descriptors.schema.json";

/// Stable record-kind tag for the descriptor set.
pub const M5_DEBUG_SESSION_DESCRIPTORS_RECORD_KIND: &str = "m5_debug_session_descriptor_set";

/// Stable id for the canonical descriptor set.
pub const M5_DEBUG_SESSION_DESCRIPTORS_SET_ID: &str = "m5-debug-session-descriptors:set:0001";

/// Evaluation stamp for the canonical set. Held as a constant so the canonical
/// binding stays deterministic and the fixture freezes byte-for-byte.
pub const M5_DEBUG_SESSION_DESCRIPTORS_AS_OF: &str = "2026-06-23T00:00:00Z";

/// The freeze gate that keeps the descriptor set current. Stable promotion runs
/// this gate; it fails when the in-code set drifts from the checked-in fixture or
/// any invariant flips.
pub const M5_DEBUG_SESSION_DESCRIPTORS_FREEZE_GATE_REF: &str =
    "crates/aureline-debug/tests/m5_debug_session_descriptors.rs";

/// The checked-in canonical descriptor-set fixture.
pub const M5_DEBUG_SESSION_DESCRIPTORS_FIXTURE_REF: &str =
    "fixtures/debug/m5_debug_session_descriptors/canonical_set.json";

/// The contract narrative document.
pub const M5_DEBUG_SESSION_DESCRIPTORS_DOC_REF: &str = "docs/debug/m5_debug_session_descriptors.md";

/// The human-readable evidence companion artifact.
pub const M5_DEBUG_SESSION_DESCRIPTORS_ARTIFACT_REF: &str =
    "artifacts/debug/m5_debug_session_descriptors.md";

// ---------------------------------------------------------------------------
// Session mode.
// ---------------------------------------------------------------------------

/// The five distinct debug-session modes that must never collapse into one generic
/// session.
///
/// Launch and attach hold live authority over a running target; core-file, replay,
/// and inspect-only are post-mortem or read-only and hold none.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DebugSessionModeClass {
    /// The debugger launched and owns the target process.
    Launch,
    /// The debugger attached to an already-running target.
    Attach,
    /// The debugger opened a core / crash dump: post-mortem, inspect-only.
    CoreFile,
    /// The debugger replays a recorded capture: reconstructed, inspect-only.
    Replay,
    /// The session is inspect-only for another reason (read-only target or policy).
    InspectOnly,
}

impl DebugSessionModeClass {
    /// All session modes, in canonical order.
    pub const ALL: [Self; 5] = [
        Self::Launch,
        Self::Attach,
        Self::CoreFile,
        Self::Replay,
        Self::InspectOnly,
    ];

    /// Stable snake_case token for this mode.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Launch => "launch",
            Self::Attach => "attach",
            Self::CoreFile => "core_file",
            Self::Replay => "replay",
            Self::InspectOnly => "inspect_only",
        }
    }

    /// Human-readable label.
    pub const fn label(self) -> &'static str {
        match self {
            Self::Launch => "Launch",
            Self::Attach => "Attach",
            Self::CoreFile => "Core file",
            Self::Replay => "Replay",
            Self::InspectOnly => "Inspect only",
        }
    }

    /// Whether this mode can hold live authority over a running target. Only launch
    /// and attach can; the post-mortem and read-only modes never do.
    pub const fn mode_holds_live_authority(self) -> bool {
        matches!(self, Self::Launch | Self::Attach)
    }

    /// Whether this mode is an inspect-only posture that withholds live control.
    pub const fn is_inspect_only(self) -> bool {
        matches!(self, Self::CoreFile | Self::Replay | Self::InspectOnly)
    }
}

// ---------------------------------------------------------------------------
// Debug entrypoint (the command side of the command/result pair).
// ---------------------------------------------------------------------------

/// The debugger entrypoint that produced a session — the command half of the
/// command/result pair.
///
/// Every entrypoint routes through the same execution-context/result pipeline, so
/// re-entry, restart, and open-in-support flows reuse one canonical session
/// identity instead of minting a fresh, surface-local session.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DebugEntrypointClass {
    /// Launch a new target under the debugger.
    LaunchTarget,
    /// Attach to an already-running target.
    AttachTarget,
    /// Open a core / crash dump.
    OpenCoreFile,
    /// Open a recorded replay capture.
    OpenReplay,
    /// Restore a prior session's layout and history without reattaching.
    RestoreSession,
    /// Reattach a restored session to reacquire live authority.
    Reattach,
    /// Restart the target inside an existing session.
    Restart,
    /// Open a session read-only from a support export.
    OpenInSupport,
}

impl DebugEntrypointClass {
    /// All entrypoints, in canonical order.
    pub const ALL: [Self; 8] = [
        Self::LaunchTarget,
        Self::AttachTarget,
        Self::OpenCoreFile,
        Self::OpenReplay,
        Self::RestoreSession,
        Self::Reattach,
        Self::Restart,
        Self::OpenInSupport,
    ];

    /// Stable snake_case token for this entrypoint.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LaunchTarget => "launch_target",
            Self::AttachTarget => "attach_target",
            Self::OpenCoreFile => "open_core_file",
            Self::OpenReplay => "open_replay",
            Self::RestoreSession => "restore_session",
            Self::Reattach => "reattach",
            Self::Restart => "restart",
            Self::OpenInSupport => "open_in_support",
        }
    }

    /// Human-readable label.
    pub const fn label(self) -> &'static str {
        match self {
            Self::LaunchTarget => "Launch target",
            Self::AttachTarget => "Attach target",
            Self::OpenCoreFile => "Open core file",
            Self::OpenReplay => "Open replay",
            Self::RestoreSession => "Restore session",
            Self::Reattach => "Reattach",
            Self::Restart => "Restart",
            Self::OpenInSupport => "Open in support",
        }
    }
}

// ---------------------------------------------------------------------------
// Target boundary, mutability, privilege, and kind.
// ---------------------------------------------------------------------------

/// The local/remote/container/managed boundary a target sits behind.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TargetBoundaryClass {
    /// Target runs on the same local host as the debugger.
    Local,
    /// Target runs on a remote host reached over a connector.
    Remote,
    /// Target runs inside a container.
    Container,
    /// Target runs under a managed runtime / platform helper.
    Managed,
}

impl TargetBoundaryClass {
    /// All boundary classes, in canonical order.
    pub const ALL: [Self; 4] = [Self::Local, Self::Remote, Self::Container, Self::Managed];

    /// Stable snake_case token for this boundary class.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Local => "local",
            Self::Remote => "remote",
            Self::Container => "container",
            Self::Managed => "managed",
        }
    }

    /// Human-readable label.
    pub const fn label(self) -> &'static str {
        match self {
            Self::Local => "Local",
            Self::Remote => "Remote",
            Self::Container => "Container",
            Self::Managed => "Managed",
        }
    }

    /// Whether the target sits across a trust boundary that must be disclosed.
    pub const fn crosses_trust_boundary(self) -> bool {
        !matches!(self, Self::Local)
    }
}

/// Whether the debugger may mutate the target's state.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TargetMutabilityClass {
    /// A live target the debugger may write to and resume.
    Mutable,
    /// A post-mortem capture (core / replay) that is intrinsically read-only.
    ReadOnlyCapture,
    /// A live target the debugger may inspect but policy forbids mutating.
    PolicyWriteProtected,
}

impl TargetMutabilityClass {
    /// All mutability classes, in canonical order.
    pub const ALL: [Self; 3] = [
        Self::Mutable,
        Self::ReadOnlyCapture,
        Self::PolicyWriteProtected,
    ];

    /// Stable snake_case token for this mutability class.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Mutable => "mutable",
            Self::ReadOnlyCapture => "read_only_capture",
            Self::PolicyWriteProtected => "policy_write_protected",
        }
    }

    /// Human-readable label.
    pub const fn label(self) -> &'static str {
        match self {
            Self::Mutable => "Mutable",
            Self::ReadOnlyCapture => "Read-only capture",
            Self::PolicyWriteProtected => "Policy write-protected",
        }
    }

    /// Whether the debugger may mutate the target.
    pub const fn permits_mutation(self) -> bool {
        matches!(self, Self::Mutable)
    }
}

/// The privilege class the debugged target runs under.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TargetPrivilegeClass {
    /// Target runs sandboxed below the user's own privilege.
    Sandboxed,
    /// Target runs at the user's standard privilege.
    UserStandard,
    /// Target runs with elevated privilege.
    Elevated,
    /// Target runs at system privilege.
    System,
}

impl TargetPrivilegeClass {
    /// All privilege classes, in canonical order.
    pub const ALL: [Self; 4] = [
        Self::Sandboxed,
        Self::UserStandard,
        Self::Elevated,
        Self::System,
    ];

    /// Stable snake_case token for this privilege class.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Sandboxed => "sandboxed",
            Self::UserStandard => "user_standard",
            Self::Elevated => "elevated",
            Self::System => "system",
        }
    }

    /// Human-readable label.
    pub const fn label(self) -> &'static str {
        match self {
            Self::Sandboxed => "Sandboxed",
            Self::UserStandard => "User standard",
            Self::Elevated => "Elevated",
            Self::System => "System",
        }
    }

    /// Whether the privilege class must be disclosed because it exceeds the user's
    /// own standard privilege.
    pub const fn requires_disclosure(self) -> bool {
        matches!(self, Self::Elevated | Self::System)
    }
}

/// The kind of thing a session attaches to or launches.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TargetKindClass {
    /// A local OS process.
    Process,
    /// A process inside a container.
    ContainerProcess,
    /// A process reached over a remote debug helper.
    RemoteHelperProcess,
    /// A managed-runtime process reached over a platform connector.
    ManagedRuntimeProcess,
    /// A core / crash dump file.
    CoreFile,
    /// A recorded replay capture.
    ReplayCapture,
}

impl TargetKindClass {
    /// All target kinds, in canonical order.
    pub const ALL: [Self; 6] = [
        Self::Process,
        Self::ContainerProcess,
        Self::RemoteHelperProcess,
        Self::ManagedRuntimeProcess,
        Self::CoreFile,
        Self::ReplayCapture,
    ];

    /// Stable snake_case token for this target kind.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Process => "process",
            Self::ContainerProcess => "container_process",
            Self::RemoteHelperProcess => "remote_helper_process",
            Self::ManagedRuntimeProcess => "managed_runtime_process",
            Self::CoreFile => "core_file",
            Self::ReplayCapture => "replay_capture",
        }
    }

    /// Human-readable label.
    pub const fn label(self) -> &'static str {
        match self {
            Self::Process => "Process",
            Self::ContainerProcess => "Container process",
            Self::RemoteHelperProcess => "Remote helper process",
            Self::ManagedRuntimeProcess => "Managed runtime process",
            Self::CoreFile => "Core file",
            Self::ReplayCapture => "Replay capture",
        }
    }

    /// Whether this kind is an intrinsically read-only post-mortem capture.
    pub const fn is_post_mortem_capture(self) -> bool {
        matches!(self, Self::CoreFile | Self::ReplayCapture)
    }
}

// ---------------------------------------------------------------------------
// Adapter drift.
// ---------------------------------------------------------------------------

/// The adapter posture a session or target currently holds — drift, reconnect,
/// inspect-only, or skew states are first-class labels, never implicit.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AdapterDriftClass {
    /// Adapter ref/version match what the session negotiated; in sync.
    AdapterCurrent,
    /// Adapter version drifted from the negotiated one; disclosed but still
    /// connected and controllable.
    AdapterDrifted,
    /// Adapter connection was lost; an explicit reconnect is required before live
    /// control.
    ReconnectRequired,
    /// No live adapter is present (core-file / replay); inspect-only.
    InspectOnlyNoAdapter,
    /// Adapter / protocol skew is unsupported; negotiated capabilities cannot be
    /// trusted.
    UnsupportedSkew,
}

impl AdapterDriftClass {
    /// All adapter-drift states, in canonical order.
    pub const ALL: [Self; 5] = [
        Self::AdapterCurrent,
        Self::AdapterDrifted,
        Self::ReconnectRequired,
        Self::InspectOnlyNoAdapter,
        Self::UnsupportedSkew,
    ];

    /// Stable snake_case token for this adapter-drift state.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::AdapterCurrent => "adapter_current",
            Self::AdapterDrifted => "adapter_drifted",
            Self::ReconnectRequired => "reconnect_required",
            Self::InspectOnlyNoAdapter => "inspect_only_no_adapter",
            Self::UnsupportedSkew => "unsupported_skew",
        }
    }

    /// Human-readable label.
    pub const fn label(self) -> &'static str {
        match self {
            Self::AdapterCurrent => "Adapter current",
            Self::AdapterDrifted => "Adapter drifted",
            Self::ReconnectRequired => "Reconnect required",
            Self::InspectOnlyNoAdapter => "Inspect-only (no adapter)",
            Self::UnsupportedSkew => "Unsupported skew",
        }
    }

    /// Whether this state must render with a visible caveat. Anything other than a
    /// current, in-sync adapter must be disclosed.
    pub const fn requires_disclosure(self) -> bool {
        !matches!(self, Self::AdapterCurrent)
    }

    /// Whether the adapter posture permits live control of the target. A drifted
    /// adapter still controls the target (with a disclosed caveat); reconnect,
    /// inspect-only, and unsupported-skew postures do not.
    pub const fn permits_live_control(self) -> bool {
        matches!(self, Self::AdapterCurrent | Self::AdapterDrifted)
    }
}

// ---------------------------------------------------------------------------
// Re-entry / restore posture.
// ---------------------------------------------------------------------------

/// The re-entry posture a session holds — how it came to exist and whether that
/// path reacquired live authority.
///
/// Session restore may reopen layout and history, but it never silently relaunches
/// or reattaches: a restored-layout-only or reattach-required posture holds no live
/// authority, and only an explicit reattach reacquires it.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ReentryPosture {
    /// A fresh launch or attach, not a restore.
    InitialEntry,
    /// Layout and history were restored, but the session was not reattached: no
    /// authority.
    RestoredLayoutOnly,
    /// Restore requires an explicit reattach before any live control.
    ReattachRequired,
    /// The session genuinely reattached and reacquired live authority.
    ReattachedReacquiredAuthority,
    /// The session was opened read-only from a support export.
    OpenedInSupport,
}

impl ReentryPosture {
    /// All re-entry postures, in canonical order.
    pub const ALL: [Self; 5] = [
        Self::InitialEntry,
        Self::RestoredLayoutOnly,
        Self::ReattachRequired,
        Self::ReattachedReacquiredAuthority,
        Self::OpenedInSupport,
    ];

    /// Stable snake_case token for this posture.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::InitialEntry => "initial_entry",
            Self::RestoredLayoutOnly => "restored_layout_only",
            Self::ReattachRequired => "reattach_required",
            Self::ReattachedReacquiredAuthority => "reattached_reacquired_authority",
            Self::OpenedInSupport => "opened_in_support",
        }
    }

    /// Human-readable label.
    pub const fn label(self) -> &'static str {
        match self {
            Self::InitialEntry => "Initial entry",
            Self::RestoredLayoutOnly => "Restored layout only",
            Self::ReattachRequired => "Reattach required",
            Self::ReattachedReacquiredAuthority => "Reattached (reacquired authority)",
            Self::OpenedInSupport => "Opened in support",
        }
    }

    /// Whether this posture can hold live authority. Only the initial entry and an
    /// explicit reattach can; a layout-only restore, a pending reattach, and a
    /// support-export open never do.
    pub const fn implies_live_authority(self) -> bool {
        matches!(
            self,
            Self::InitialEntry | Self::ReattachedReacquiredAuthority
        )
    }

    /// Whether this posture must render with a visible caveat.
    pub const fn requires_disclosure(self) -> bool {
        !matches!(self, Self::InitialEntry)
    }
}

// ---------------------------------------------------------------------------
// Run state.
// ---------------------------------------------------------------------------

/// The current run state of a session's target.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SessionRunStateClass {
    /// The live target is executing.
    Running,
    /// The live target is stopped at a breakpoint, exception, or manual pause.
    Paused,
    /// A post-mortem or read-only snapshot the user can inspect but not run.
    ReconstructedInspectable,
    /// A restored session with no live target until an explicit reattach.
    AwaitingReattach,
    /// The session ended; only its history is inspectable.
    Terminated,
}

impl SessionRunStateClass {
    /// All run states, in canonical order.
    pub const ALL: [Self; 5] = [
        Self::Running,
        Self::Paused,
        Self::ReconstructedInspectable,
        Self::AwaitingReattach,
        Self::Terminated,
    ];

    /// Stable snake_case token for this run state.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Running => "running",
            Self::Paused => "paused",
            Self::ReconstructedInspectable => "reconstructed_inspectable",
            Self::AwaitingReattach => "awaiting_reattach",
            Self::Terminated => "terminated",
        }
    }

    /// Human-readable label.
    pub const fn label(self) -> &'static str {
        match self {
            Self::Running => "Running",
            Self::Paused => "Paused",
            Self::ReconstructedInspectable => "Reconstructed (inspectable)",
            Self::AwaitingReattach => "Awaiting reattach",
            Self::Terminated => "Terminated",
        }
    }

    /// Whether this run state forbids live authority. A reconstructed snapshot, a
    /// session awaiting reattach, and a terminated session can never hold live
    /// control, so a `true` here paired with `holds_live_authority` is a lie the
    /// freeze gate rejects.
    pub const fn forbids_live_authority(self) -> bool {
        matches!(
            self,
            Self::ReconstructedInspectable | Self::AwaitingReattach | Self::Terminated
        )
    }
}

// ---------------------------------------------------------------------------
// Record structs.
// ---------------------------------------------------------------------------

/// The exact target / process identity a target descriptor carries so support
/// evidence never confuses one debugged target for another.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DebugTargetIdentity {
    /// Canonical target id from the shared execution context.
    pub canonical_target_id: String,
    /// Plain-language target label suitable for support export.
    pub target_label: String,
    /// Opaque inferior process id token when attached/launched, never a raw pid.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub inferior_process_ref: Option<String>,
    /// Opaque digest of the resolved working directory, never a raw path.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub working_directory_digest: Option<String>,
    /// Build / artifact identity the target was produced from.
    pub build_artifact_id: String,
}

/// Stable identity for the debug adapter behind a target.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DebugAdapterRef {
    /// Stable adapter id (e.g. `adapter:python:debugpy`).
    pub adapter_id: String,
    /// Plain-language adapter label.
    pub adapter_label: String,
    /// Adapter implementation version.
    pub adapter_version: String,
    /// DAP protocol version the session negotiated, once settled.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub negotiated_protocol_version: Option<String>,
}

/// A typed attach-target descriptor: the picker-stage truth for the process,
/// container, remote helper, core file, or replay capture a session attaches to or
/// launches.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AttachTargetDescriptor {
    /// Stable, namespaced descriptor id.
    pub descriptor_id: String,
    /// The target kind.
    pub target_kind: TargetKindClass,
    /// Stable token for the target kind.
    pub target_kind_token: String,
    /// The exact target / process identity.
    pub target_identity: DebugTargetIdentity,
    /// The local/remote/container/managed boundary.
    pub boundary_class: TargetBoundaryClass,
    /// Stable token for the boundary class.
    pub boundary_token: String,
    /// Whether the target sits across a trust boundary that must be disclosed.
    pub boundary_crosses_trust: bool,
    /// The mutability class.
    pub mutability_class: TargetMutabilityClass,
    /// Stable token for the mutability class.
    pub mutability_token: String,
    /// Whether the debugger may mutate the target.
    pub permits_mutation: bool,
    /// The privilege class.
    pub privilege_class: TargetPrivilegeClass,
    /// Stable token for the privilege class.
    pub privilege_token: String,
    /// Whether the privilege class must be disclosed.
    pub privilege_requires_disclosure: bool,
    /// The adapter behind this target.
    pub adapter: DebugAdapterRef,
    /// The adapter-drift posture for this target.
    pub adapter_drift: AdapterDriftClass,
    /// Stable token for the adapter-drift posture.
    pub adapter_drift_token: String,
    /// Whether the adapter-drift posture must be disclosed.
    pub adapter_drift_requires_disclosure: bool,
    /// Stable tokens for the negotiated adapter capabilities.
    pub capability_refs: Vec<String>,
    /// The proof packet (negotiation evidence) that keeps this descriptor current.
    pub negotiation_evidence_ref: String,
    /// One reviewable export-safe sentence describing the target.
    pub summary: String,
}

impl AttachTargetDescriptor {
    /// Builds an attach-target descriptor, deriving every computed token and honesty
    /// flag from the typed enums so the row cannot disagree with itself.
    #[allow(clippy::too_many_arguments)]
    pub fn build(
        descriptor_id: impl Into<String>,
        target_kind: TargetKindClass,
        target_identity: DebugTargetIdentity,
        boundary_class: TargetBoundaryClass,
        mutability_class: TargetMutabilityClass,
        privilege_class: TargetPrivilegeClass,
        adapter: DebugAdapterRef,
        adapter_drift: AdapterDriftClass,
        capability_refs: Vec<String>,
        negotiation_evidence_ref: impl Into<String>,
        summary: impl Into<String>,
    ) -> Self {
        Self {
            descriptor_id: descriptor_id.into(),
            target_kind,
            target_kind_token: target_kind.as_str().to_owned(),
            target_identity,
            boundary_class,
            boundary_token: boundary_class.as_str().to_owned(),
            boundary_crosses_trust: boundary_class.crosses_trust_boundary(),
            mutability_class,
            mutability_token: mutability_class.as_str().to_owned(),
            permits_mutation: mutability_class.permits_mutation(),
            privilege_class,
            privilege_token: privilege_class.as_str().to_owned(),
            privilege_requires_disclosure: privilege_class.requires_disclosure(),
            adapter,
            adapter_drift,
            adapter_drift_token: adapter_drift.as_str().to_owned(),
            adapter_drift_requires_disclosure: adapter_drift.requires_disclosure(),
            capability_refs,
            negotiation_evidence_ref: negotiation_evidence_ref.into(),
            summary: summary.into(),
        }
    }
}

/// The identity a session echoes from its target so the freeze gate can prove the
/// target identity, mutability, privilege class, and adapter drift survive from the
/// picker descriptor into the active session and the export packet unchanged.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TargetIdentityEcho {
    /// Canonical target id echoed from the target descriptor.
    pub canonical_target_id: String,
    /// Build / artifact identity echoed from the target descriptor.
    pub build_artifact_id: String,
    /// Mutability token echoed from the target descriptor.
    pub mutability_token: String,
    /// Privilege token echoed from the target descriptor.
    pub privilege_token: String,
    /// Boundary token echoed from the target descriptor.
    pub boundary_token: String,
    /// Adapter-drift token echoed from the target descriptor.
    pub adapter_drift_token: String,
}

impl TargetIdentityEcho {
    /// Builds the echo for a target descriptor.
    pub fn of(target: &AttachTargetDescriptor) -> Self {
        Self {
            canonical_target_id: target.target_identity.canonical_target_id.clone(),
            build_artifact_id: target.target_identity.build_artifact_id.clone(),
            mutability_token: target.mutability_token.clone(),
            privilege_token: target.privilege_token.clone(),
            boundary_token: target.boundary_token.clone(),
            adapter_drift_token: target.adapter_drift_token.clone(),
        }
    }

    /// Whether this echo matches the given target descriptor exactly.
    pub fn matches(&self, target: &AttachTargetDescriptor) -> bool {
        *self == Self::of(target)
    }
}

/// A typed debug-session descriptor: the active-session truth a debugger-capable
/// surface reads to explain what was launched or attached, against which target,
/// with what current authority and adapter posture.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DebugSessionDescriptor {
    /// Stable, namespaced session id.
    pub session_id: String,
    /// Execution context id anchoring target and toolchain identity. The session
    /// routes through the shared execution-context/result pipeline, so re-entry,
    /// restart, and open-in-support flows reuse one canonical identity.
    pub execution_context_id: String,
    /// The entrypoint that produced the session — the command half of the pair.
    pub entrypoint: DebugEntrypointClass,
    /// Stable token for the entrypoint.
    pub entrypoint_token: String,
    /// The session mode — the result half of the pair.
    pub mode: DebugSessionModeClass,
    /// Stable token for the session mode.
    pub mode_token: String,
    /// Whether the mode can hold live authority.
    pub mode_holds_live_authority: bool,
    /// Whether the mode is an inspect-only posture.
    pub mode_is_inspect_only: bool,
    /// The current run state.
    pub run_state: SessionRunStateClass,
    /// Stable token for the run state.
    pub run_state_token: String,
    /// The re-entry / restore posture.
    pub reentry_posture: ReentryPosture,
    /// Stable token for the re-entry posture.
    pub reentry_token: String,
    /// Whether the re-entry posture can hold live authority.
    pub reentry_implies_live_authority: bool,
    /// The current adapter-drift posture.
    pub adapter_drift: AdapterDriftClass,
    /// Stable token for the adapter-drift posture.
    pub adapter_drift_token: String,
    /// Whether the adapter-drift posture must be disclosed.
    pub adapter_drift_requires_disclosure: bool,
    /// Whether the session currently holds live authority over a running target —
    /// derived from the mode, the re-entry posture, and the adapter drift together.
    pub holds_live_authority: bool,
    /// Stable ref to the [`AttachTargetDescriptor::descriptor_id`] this session runs.
    pub target_descriptor_ref: String,
    /// The identity echoed from that target descriptor.
    pub target_identity_echo: TargetIdentityEcho,
    /// Opaque thread refs, never raw thread handles.
    pub thread_refs: Vec<String>,
    /// Why the target last stopped, when paused.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stop_reason: Option<String>,
    /// The proof packet that keeps this session descriptor current.
    pub proof_packet_ref: String,
    /// One reviewable export-safe sentence describing the session.
    pub summary: String,
}

impl DebugSessionDescriptor {
    /// Derives whether a session holds live authority from its mode, re-entry
    /// posture, and adapter drift. A session holds live authority only when the mode
    /// can, the re-entry posture reacquired it, and the adapter still permits live
    /// control.
    pub const fn derive_holds_live_authority(
        mode: DebugSessionModeClass,
        reentry: ReentryPosture,
        drift: AdapterDriftClass,
    ) -> bool {
        mode.mode_holds_live_authority()
            && reentry.implies_live_authority()
            && drift.permits_live_control()
    }

    /// Builds a session descriptor, deriving every computed token and honesty flag
    /// from the typed enums and the referenced target so the row cannot disagree
    /// with itself.
    #[allow(clippy::too_many_arguments)]
    pub fn build(
        session_id: impl Into<String>,
        execution_context_id: impl Into<String>,
        entrypoint: DebugEntrypointClass,
        mode: DebugSessionModeClass,
        run_state: SessionRunStateClass,
        reentry_posture: ReentryPosture,
        adapter_drift: AdapterDriftClass,
        target: &AttachTargetDescriptor,
        thread_refs: Vec<String>,
        stop_reason: Option<String>,
        proof_packet_ref: impl Into<String>,
        summary: impl Into<String>,
    ) -> Self {
        Self {
            session_id: session_id.into(),
            execution_context_id: execution_context_id.into(),
            entrypoint,
            entrypoint_token: entrypoint.as_str().to_owned(),
            mode,
            mode_token: mode.as_str().to_owned(),
            mode_holds_live_authority: mode.mode_holds_live_authority(),
            mode_is_inspect_only: mode.is_inspect_only(),
            run_state,
            run_state_token: run_state.as_str().to_owned(),
            reentry_posture,
            reentry_token: reentry_posture.as_str().to_owned(),
            reentry_implies_live_authority: reentry_posture.implies_live_authority(),
            adapter_drift,
            adapter_drift_token: adapter_drift.as_str().to_owned(),
            adapter_drift_requires_disclosure: adapter_drift.requires_disclosure(),
            holds_live_authority: Self::derive_holds_live_authority(
                mode,
                reentry_posture,
                adapter_drift,
            ),
            target_descriptor_ref: target.descriptor_id.clone(),
            target_identity_echo: TargetIdentityEcho::of(target),
            thread_refs,
            stop_reason,
            proof_packet_ref: proof_packet_ref.into(),
            summary: summary.into(),
        }
    }
}

/// One frozen invariant, with a computed `holds` flag.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DescriptorInvariant {
    /// Stable invariant id.
    pub invariant_id: String,
    /// The invariant statement.
    pub statement: String,
    /// Whether the built set satisfies the invariant.
    pub holds: bool,
}

/// The frozen, typed M5 debug-session descriptor set.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DebugSessionDescriptorSet {
    /// Stable record-kind tag.
    pub record_kind: String,
    /// Schema version.
    pub m5_debug_session_descriptors_schema_version: u32,
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
    /// The surfaces that consume the descriptor set.
    pub consumer_surfaces: Vec<DebugConsumer>,
    /// The attach-target descriptors (picker-stage truth).
    pub targets: Vec<AttachTargetDescriptor>,
    /// The debug-session descriptors (active-session truth).
    pub sessions: Vec<DebugSessionDescriptor>,
    /// The computed invariants.
    pub invariants: Vec<DescriptorInvariant>,
    /// Whether raw source bodies and payloads are excluded (always true).
    pub raw_payload_excluded: bool,
}

/// Error returned when the descriptor set fails a structural consistency check.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DescriptorSetValidationError {
    /// The failed check.
    pub reason: String,
}

impl std::fmt::Display for DescriptorSetValidationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "m5 debug-session descriptor set invalid: {}",
            self.reason
        )
    }
}

impl std::error::Error for DescriptorSetValidationError {}

impl DebugSessionDescriptorSet {
    /// Returns the target descriptor with the given id, if present.
    pub fn target(&self, descriptor_id: &str) -> Option<&AttachTargetDescriptor> {
        self.targets
            .iter()
            .find(|t| t.descriptor_id == descriptor_id)
    }

    /// Returns the first session in the given mode, if present.
    pub fn session_in_mode(&self, mode: DebugSessionModeClass) -> Option<&DebugSessionDescriptor> {
        self.sessions.iter().find(|s| s.mode == mode)
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

    fn all_refs(&self) -> impl Iterator<Item = &str> {
        let from_set = self
            .source_schema_refs
            .iter()
            .map(String::as_str)
            .chain(self.producer_refs.iter().map(String::as_str))
            .chain(std::iter::once(self.freeze_gate_ref.as_str()));
        let from_targets = self
            .targets
            .iter()
            .map(|t| t.negotiation_evidence_ref.as_str());
        let from_sessions = self.sessions.iter().map(|s| s.proof_packet_ref.as_str());
        from_set.chain(from_targets).chain(from_sessions)
    }

    /// Re-checks structural consistency and returns an error on the first failure.
    ///
    /// # Errors
    ///
    /// Returns a [`DescriptorSetValidationError`] when an identifier, a ref, a
    /// cross-reference, an echo, or an invariant is inconsistent.
    pub fn validate(&self) -> Result<(), DescriptorSetValidationError> {
        let fail = |reason: String| Err(DescriptorSetValidationError { reason });

        if self.record_kind != M5_DEBUG_SESSION_DESCRIPTORS_RECORD_KIND {
            return fail(format!("unexpected record_kind {}", self.record_kind));
        }
        if self.schema_ref != M5_DEBUG_SESSION_DESCRIPTORS_SCHEMA_REF {
            return fail(format!("unexpected schema_ref {}", self.schema_ref));
        }
        if self.m5_debug_session_descriptors_schema_version
            != M5_DEBUG_SESSION_DESCRIPTORS_SCHEMA_VERSION
        {
            return fail("unexpected schema version".to_owned());
        }
        if !self.raw_payload_excluded {
            return fail("raw_payload_excluded must be true".to_owned());
        }
        if self.targets.is_empty() {
            return fail("no targets".to_owned());
        }
        if self.sessions.is_empty() {
            return fail("no sessions".to_owned());
        }

        // Stable ids are unique.
        if !all_unique(self.targets.iter().map(|t| t.descriptor_id.as_str())) {
            return fail("target descriptor ids are not unique".to_owned());
        }
        if !all_unique(self.sessions.iter().map(|s| s.session_id.as_str())) {
            return fail("session ids are not unique".to_owned());
        }

        // Every session mode appears at least once, so the five stay distinct.
        for mode in DebugSessionModeClass::ALL {
            if self.session_in_mode(mode).is_none() {
                return fail(format!("session mode {} is not present", mode.as_str()));
            }
        }

        // Per-target structural floor.
        for target in &self.targets {
            if target.descriptor_id.is_empty() {
                return fail("target has empty descriptor id".to_owned());
            }
            if target.negotiation_evidence_ref.is_empty() {
                return fail(format!(
                    "target {} has no negotiation evidence",
                    target.descriptor_id
                ));
            }
            if !target_flags_consistent(target) {
                return fail(format!(
                    "target {} computed flags disagree with its enums",
                    target.descriptor_id
                ));
            }
        }

        // Per-session structural floor and cross-references.
        for session in &self.sessions {
            if session.execution_context_id.is_empty() {
                return fail(format!(
                    "session {} has no execution context",
                    session.session_id
                ));
            }
            if session.proof_packet_ref.is_empty() {
                return fail(format!(
                    "session {} has no proof packet",
                    session.session_id
                ));
            }
            if !session_flags_consistent(session) {
                return fail(format!(
                    "session {} computed flags disagree with its enums",
                    session.session_id
                ));
            }
            let target = match self.target(&session.target_descriptor_ref) {
                Some(target) => target,
                None => {
                    return fail(format!(
                        "session {} references unknown target {}",
                        session.session_id, session.target_descriptor_ref
                    ))
                }
            };
            if !session.target_identity_echo.matches(target) {
                return fail(format!(
                    "session {} identity echo does not match target {}",
                    session.session_id, target.descriptor_id
                ));
            }
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

fn target_flags_consistent(target: &AttachTargetDescriptor) -> bool {
    target.target_kind_token == target.target_kind.as_str()
        && target.boundary_token == target.boundary_class.as_str()
        && target.boundary_crosses_trust == target.boundary_class.crosses_trust_boundary()
        && target.mutability_token == target.mutability_class.as_str()
        && target.permits_mutation == target.mutability_class.permits_mutation()
        && target.privilege_token == target.privilege_class.as_str()
        && target.privilege_requires_disclosure == target.privilege_class.requires_disclosure()
        && target.adapter_drift_token == target.adapter_drift.as_str()
        && target.adapter_drift_requires_disclosure == target.adapter_drift.requires_disclosure()
}

fn session_flags_consistent(session: &DebugSessionDescriptor) -> bool {
    session.entrypoint_token == session.entrypoint.as_str()
        && session.mode_token == session.mode.as_str()
        && session.mode_holds_live_authority == session.mode.mode_holds_live_authority()
        && session.mode_is_inspect_only == session.mode.is_inspect_only()
        && session.run_state_token == session.run_state.as_str()
        && session.reentry_token == session.reentry_posture.as_str()
        && session.reentry_implies_live_authority
            == session.reentry_posture.implies_live_authority()
        && session.adapter_drift_token == session.adapter_drift.as_str()
        && session.adapter_drift_requires_disclosure == session.adapter_drift.requires_disclosure()
        && session.holds_live_authority
            == DebugSessionDescriptor::derive_holds_live_authority(
                session.mode,
                session.reentry_posture,
                session.adapter_drift,
            )
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

/// Builds the canonical M5 debug-session descriptor set.
///
/// Deterministic: the same bytes every call. Each invariant's `holds` flag is
/// computed from the built descriptors, so an inconsistent edit flips an invariant
/// rather than silently passing.
pub fn m5_debug_session_descriptor_set() -> DebugSessionDescriptorSet {
    let targets = build_targets();
    let sessions = build_sessions(&targets);
    let invariants = compute_invariants(&targets, &sessions);

    DebugSessionDescriptorSet {
        record_kind: M5_DEBUG_SESSION_DESCRIPTORS_RECORD_KIND.to_owned(),
        m5_debug_session_descriptors_schema_version: M5_DEBUG_SESSION_DESCRIPTORS_SCHEMA_VERSION,
        schema_ref: M5_DEBUG_SESSION_DESCRIPTORS_SCHEMA_REF.to_owned(),
        set_id: M5_DEBUG_SESSION_DESCRIPTORS_SET_ID.to_owned(),
        as_of: M5_DEBUG_SESSION_DESCRIPTORS_AS_OF.to_owned(),
        freeze_gate_ref: M5_DEBUG_SESSION_DESCRIPTORS_FREEZE_GATE_REF.to_owned(),
        summary: "One frozen, typed set of M5 debug-session and attach-target descriptors. Launch, \
                  attach, core-file, replay, and inspect-only appear as five distinct session \
                  modes and command/result objects; an attach target carries its identity, \
                  local/remote/container/managed boundary, mutability, privilege class, adapter \
                  ref/version, adapter drift, and build/artifact identity, and every session echoes \
                  that identity so it survives from picker to active session to export packet; \
                  adapter drift, reconnect-required, inspect-only, and unsupported-skew are \
                  first-class labels; and a session restore reopens layout and history but never \
                  silently relaunches or reattaches — the re-entry posture names re-entry \
                  explicitly and live authority is derived from mode, posture, and adapter drift \
                  together."
            .to_owned(),
        source_schema_refs: strvec(&[
            "schemas/runtime/debug_session.schema.json",
            "schemas/runtime/stabilize_debugger_host_and_adapter_negotiation_truth.schema.json",
            "schemas/debug/m5_debug_contracts.schema.json",
        ]),
        producer_refs: strvec(&[
            "crates/aureline-debug/src/m5_debug_session_descriptors/mod.rs",
            "crates/aureline-runtime/src/debug/records.rs",
            "crates/aureline-runtime/src/stabilize_debugger_host_and_adapter_negotiation/mod.rs",
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
        targets,
        sessions,
        invariants,
        raw_payload_excluded: true,
    }
}

fn strvec(items: &[&str]) -> Vec<String> {
    items.iter().map(|s| (*s).to_owned()).collect()
}

const NEGOTIATION_EVIDENCE_REF: &str =
    "fixtures/runtime/m4/stabilize_debugger_host_and_adapter_negotiation/baseline_stable.json";
const LIVE_PROOF_REF: &str = "fixtures/runtime/debugger_host_beta/protected_walk_local.json";
const CORE_PROOF_REF: &str = "fixtures/debug/symbolication/exact_local_report.json";
const REPLAY_PROOF_REF: &str = "fixtures/runtime/m3/replay_packets/local_task_exact_read_only.json";

fn adapter(version: &str, negotiated: Option<&str>) -> DebugAdapterRef {
    DebugAdapterRef {
        adapter_id: "adapter:generic:dap".to_owned(),
        adapter_label: "Generic DAP adapter".to_owned(),
        adapter_version: version.to_owned(),
        negotiated_protocol_version: negotiated.map(str::to_owned),
    }
}

fn identity(target: &str, label: &str, pid: Option<&str>, build: &str) -> DebugTargetIdentity {
    DebugTargetIdentity {
        canonical_target_id: target.to_owned(),
        target_label: label.to_owned(),
        inferior_process_ref: pid.map(str::to_owned),
        working_directory_digest: Some("wd:digest:0a1b2c".to_owned()),
        build_artifact_id: build.to_owned(),
    }
}

fn build_targets() -> Vec<AttachTargetDescriptor> {
    vec![
        AttachTargetDescriptor::build(
            "debug.attach_target:local_launch:0001",
            TargetKindClass::Process,
            identity(
                "target:local:service-api",
                "service-api (local process)",
                Some("proc:ref:0001"),
                "build:debug:service-api:9f1c",
            ),
            TargetBoundaryClass::Local,
            TargetMutabilityClass::Mutable,
            TargetPrivilegeClass::UserStandard,
            adapter("1.4.0", Some("1.65")),
            AdapterDriftClass::AdapterCurrent,
            strvec(&[
                "function_breakpoints",
                "conditional_breakpoints",
                "terminate_request",
            ]),
            NEGOTIATION_EVIDENCE_REF,
            "Local service process the debugger launched and owns, at the user's standard \
             privilege, with a current adapter.",
        ),
        AttachTargetDescriptor::build(
            "debug.attach_target:remote_attach:0002",
            TargetKindClass::RemoteHelperProcess,
            identity(
                "target:remote:worker-7",
                "worker-7 (remote helper)",
                Some("proc:ref:0002"),
                "build:debug:worker:3a4d",
            ),
            TargetBoundaryClass::Remote,
            TargetMutabilityClass::Mutable,
            TargetPrivilegeClass::Elevated,
            adapter("1.4.0", Some("1.65")),
            AdapterDriftClass::AdapterCurrent,
            strvec(&["function_breakpoints", "terminate_request"]),
            NEGOTIATION_EVIDENCE_REF,
            "Remote worker process the debugger attached to over a helper, at elevated privilege \
             across a trust boundary, with a current adapter.",
        ),
        AttachTargetDescriptor::build(
            "debug.attach_target:core_file:0003",
            TargetKindClass::CoreFile,
            identity(
                "target:core:service-api-crash",
                "service-api crash dump",
                None,
                "build:debug:service-api:9f1c",
            ),
            TargetBoundaryClass::Local,
            TargetMutabilityClass::ReadOnlyCapture,
            TargetPrivilegeClass::System,
            adapter("1.4.0", None),
            AdapterDriftClass::InspectOnlyNoAdapter,
            Vec::new(),
            CORE_PROOF_REF,
            "Local core dump of a crashed service, read-only and post-mortem, with no live \
             adapter, captured from a system-privilege target.",
        ),
        AttachTargetDescriptor::build(
            "debug.attach_target:replay_capture:0004",
            TargetKindClass::ReplayCapture,
            identity(
                "target:replay:task-run-42",
                "task run 42 (replay capture)",
                None,
                "build:debug:task:7b2e",
            ),
            TargetBoundaryClass::Local,
            TargetMutabilityClass::ReadOnlyCapture,
            TargetPrivilegeClass::UserStandard,
            adapter("1.4.0", None),
            AdapterDriftClass::InspectOnlyNoAdapter,
            strvec(&["reverse_execution"]),
            REPLAY_PROOF_REF,
            "Local recorded replay capture of a task run, reconstructed and read-only, with no \
             live adapter.",
        ),
        AttachTargetDescriptor::build(
            "debug.attach_target:container_inspect:0005",
            TargetKindClass::ContainerProcess,
            identity(
                "target:container:sidecar-3",
                "sidecar-3 (container process)",
                Some("proc:ref:0005"),
                "build:debug:sidecar:5c6f",
            ),
            TargetBoundaryClass::Container,
            TargetMutabilityClass::PolicyWriteProtected,
            TargetPrivilegeClass::UserStandard,
            adapter("1.2.0", Some("1.59")),
            AdapterDriftClass::UnsupportedSkew,
            Vec::new(),
            NEGOTIATION_EVIDENCE_REF,
            "Container sidecar process inspect-only under a write-protect policy, behind a \
             container boundary, with an unsupported adapter/protocol skew.",
        ),
        AttachTargetDescriptor::build(
            "debug.attach_target:managed_drift:0006",
            TargetKindClass::ManagedRuntimeProcess,
            identity(
                "target:managed:fn-runtime-9",
                "fn-runtime-9 (managed runtime)",
                Some("proc:ref:0006"),
                "build:debug:fn:8d9a",
            ),
            TargetBoundaryClass::Managed,
            TargetMutabilityClass::Mutable,
            TargetPrivilegeClass::UserStandard,
            adapter("1.5.1", Some("1.65")),
            AdapterDriftClass::AdapterDrifted,
            strvec(&["function_breakpoints", "memory_access"]),
            NEGOTIATION_EVIDENCE_REF,
            "Managed-runtime process the debugger launched over a connector, with a drifted but \
             still-connected adapter disclosed.",
        ),
    ]
}

fn build_sessions(targets: &[AttachTargetDescriptor]) -> Vec<DebugSessionDescriptor> {
    let target_by = |id: &str| {
        targets
            .iter()
            .find(|t| t.descriptor_id == id)
            .expect("canonical target id resolves")
    };

    let local_launch = target_by("debug.attach_target:local_launch:0001");
    let remote_attach = target_by("debug.attach_target:remote_attach:0002");
    let core_file = target_by("debug.attach_target:core_file:0003");
    let replay = target_by("debug.attach_target:replay_capture:0004");
    let container = target_by("debug.attach_target:container_inspect:0005");
    let managed = target_by("debug.attach_target:managed_drift:0006");

    vec![
        DebugSessionDescriptor::build(
            "debug.session:launch:0001",
            "exec.ctx:service-api:0001",
            DebugEntrypointClass::LaunchTarget,
            DebugSessionModeClass::Launch,
            SessionRunStateClass::Running,
            ReentryPosture::InitialEntry,
            AdapterDriftClass::AdapterCurrent,
            local_launch,
            strvec(&["thread:ref:main", "thread:ref:worker"]),
            None,
            LIVE_PROOF_REF,
            "Launch session that owns the local service process with live authority and a current \
             adapter.",
        ),
        DebugSessionDescriptor::build(
            "debug.session:attach:0002",
            "exec.ctx:worker-7:0002",
            DebugEntrypointClass::AttachTarget,
            DebugSessionModeClass::Attach,
            SessionRunStateClass::Paused,
            ReentryPosture::InitialEntry,
            AdapterDriftClass::AdapterCurrent,
            remote_attach,
            strvec(&["thread:ref:main"]),
            Some("breakpoint".to_owned()),
            LIVE_PROOF_REF,
            "Attach session paused at a breakpoint on a remote elevated worker, holding live \
             authority, with target identity, mutability, and privilege preserved from the \
             picker.",
        ),
        DebugSessionDescriptor::build(
            "debug.session:core_file:0003",
            "exec.ctx:service-api-crash:0003",
            DebugEntrypointClass::OpenCoreFile,
            DebugSessionModeClass::CoreFile,
            SessionRunStateClass::ReconstructedInspectable,
            ReentryPosture::InitialEntry,
            AdapterDriftClass::InspectOnlyNoAdapter,
            core_file,
            strvec(&["thread:ref:crashed"]),
            Some("fatal_signal".to_owned()),
            CORE_PROOF_REF,
            "Core-file session reconstructed from a crash dump: inspect-only, with no live \
             authority and no live adapter.",
        ),
        DebugSessionDescriptor::build(
            "debug.session:replay:0004",
            "exec.ctx:task-run-42:0004",
            DebugEntrypointClass::OpenReplay,
            DebugSessionModeClass::Replay,
            SessionRunStateClass::ReconstructedInspectable,
            ReentryPosture::InitialEntry,
            AdapterDriftClass::InspectOnlyNoAdapter,
            replay,
            strvec(&["thread:ref:replayed"]),
            None,
            REPLAY_PROOF_REF,
            "Replay session reconstructed from a recorded capture: inspect-only, with no live \
             authority.",
        ),
        DebugSessionDescriptor::build(
            "debug.session:inspect_only:0005",
            "exec.ctx:sidecar-3:0005",
            DebugEntrypointClass::AttachTarget,
            DebugSessionModeClass::InspectOnly,
            SessionRunStateClass::ReconstructedInspectable,
            ReentryPosture::InitialEntry,
            AdapterDriftClass::UnsupportedSkew,
            container,
            strvec(&["thread:ref:main"]),
            None,
            CORE_PROOF_REF,
            "Inspect-only session on a write-protected container process whose adapter skew is \
             unsupported, so it holds no live authority.",
        ),
        DebugSessionDescriptor::build(
            "debug.session:restored_layout:0006",
            "exec.ctx:worker-7:0002",
            DebugEntrypointClass::RestoreSession,
            DebugSessionModeClass::Attach,
            SessionRunStateClass::AwaitingReattach,
            ReentryPosture::RestoredLayoutOnly,
            AdapterDriftClass::ReconnectRequired,
            remote_attach,
            Vec::new(),
            None,
            LIVE_PROOF_REF,
            "Restored attach session that reopened layout and history but was not reattached: it \
             awaits an explicit reattach and holds no live authority.",
        ),
        DebugSessionDescriptor::build(
            "debug.session:reattached:0007",
            "exec.ctx:worker-7:0002",
            DebugEntrypointClass::Reattach,
            DebugSessionModeClass::Attach,
            SessionRunStateClass::Running,
            ReentryPosture::ReattachedReacquiredAuthority,
            AdapterDriftClass::AdapterCurrent,
            remote_attach,
            strvec(&["thread:ref:main"]),
            None,
            LIVE_PROOF_REF,
            "Reattached session that explicitly reacquired live authority over the same remote \
             worker, reusing one canonical session identity through the execution-context \
             pipeline.",
        ),
        DebugSessionDescriptor::build(
            "debug.session:managed_drift:0008",
            "exec.ctx:fn-runtime-9:0006",
            DebugEntrypointClass::LaunchTarget,
            DebugSessionModeClass::Launch,
            SessionRunStateClass::Running,
            ReentryPosture::InitialEntry,
            AdapterDriftClass::AdapterDrifted,
            managed,
            strvec(&["thread:ref:main"]),
            None,
            LIVE_PROOF_REF,
            "Launch session on a managed runtime whose adapter drifted but stays connected: it \
             holds live authority with the drift disclosed.",
        ),
    ]
}

fn invariant(invariant_id: &str, statement: &str, holds: bool) -> DescriptorInvariant {
    DescriptorInvariant {
        invariant_id: invariant_id.to_owned(),
        statement: statement.to_owned(),
        holds,
    }
}

fn compute_invariants(
    targets: &[AttachTargetDescriptor],
    sessions: &[DebugSessionDescriptor],
) -> Vec<DescriptorInvariant> {
    // Every one of the five session modes is materialized at least once.
    let session_modes_distinct = DebugSessionModeClass::ALL
        .iter()
        .all(|mode| sessions.iter().any(|s| s.mode == *mode))
        && DebugSessionModeClass::ALL
            .iter()
            .map(|m| m.as_str())
            .collect::<std::collections::BTreeSet<_>>()
            .len()
            == DebugSessionModeClass::ALL.len();

    // Inspect-only modes never hold live authority.
    let inspect_only_no_authority = sessions
        .iter()
        .filter(|s| s.mode.is_inspect_only())
        .all(|s| !s.holds_live_authority);

    // A restored-layout-only or reattach-required posture never holds authority,
    // and only an explicit reattach reacquires it.
    let restore_never_reacquires = sessions.iter().all(|s| match s.reentry_posture {
        ReentryPosture::RestoredLayoutOnly
        | ReentryPosture::ReattachRequired
        | ReentryPosture::OpenedInSupport => !s.holds_live_authority,
        ReentryPosture::InitialEntry | ReentryPosture::ReattachedReacquiredAuthority => true,
    }) && sessions.iter().any(|s| {
        s.reentry_posture == ReentryPosture::RestoredLayoutOnly && !s.holds_live_authority
    });

    // Every session's live-authority flag equals the derivation from mode, posture,
    // and adapter drift.
    let live_authority_derived = sessions.iter().all(|s| {
        s.holds_live_authority
            == DebugSessionDescriptor::derive_holds_live_authority(
                s.mode,
                s.reentry_posture,
                s.adapter_drift,
            )
    });

    // Each session resolves its target and echoes its identity, mutability,
    // privilege, boundary, and adapter drift unchanged.
    let identity_preserved = sessions.iter().all(|s| {
        targets
            .iter()
            .find(|t| t.descriptor_id == s.target_descriptor_ref)
            .is_some_and(|t| s.target_identity_echo.matches(t))
    });

    // Adapter drift is a first-class label on every session and target, and any
    // non-current state requires disclosure.
    let adapter_drift_first_class = sessions
        .iter()
        .all(|s| s.adapter_drift_requires_disclosure == s.adapter_drift.requires_disclosure())
        && targets
            .iter()
            .all(|t| t.adapter_drift_requires_disclosure == t.adapter_drift.requires_disclosure())
        && sessions
            .iter()
            .any(|s| s.adapter_drift == AdapterDriftClass::AdapterDrifted)
        && sessions
            .iter()
            .any(|s| s.adapter_drift == AdapterDriftClass::ReconnectRequired)
        && targets
            .iter()
            .any(|t| t.adapter_drift == AdapterDriftClass::UnsupportedSkew);

    // Every session routes through the execution-context/result pipeline: a
    // non-empty execution context id and a typed entrypoint.
    let routes_execution_context = sessions
        .iter()
        .all(|s| !s.execution_context_id.is_empty() && s.entrypoint_token == s.entrypoint.as_str());

    // No run state that forbids live authority is paired with a held authority.
    let run_state_authority_consistent = sessions
        .iter()
        .all(|s| !(s.run_state.forbids_live_authority() && s.holds_live_authority));

    // Build / artifact identity is preserved from target to session echo.
    let build_identity_preserved = sessions.iter().all(|s| {
        targets
            .iter()
            .find(|t| t.descriptor_id == s.target_descriptor_ref)
            .is_some_and(|t| {
                s.target_identity_echo.build_artifact_id == t.target_identity.build_artifact_id
            })
    });

    vec![
        invariant(
            "descriptors.session_modes_distinct",
            "Launch, attach, core-file, replay, and inspect-only appear as five distinct session \
             modes, never one generic debug session.",
            session_modes_distinct,
        ),
        invariant(
            "descriptors.inspect_only_modes_hold_no_live_authority",
            "Core-file, replay, and inspect-only sessions never hold live authority over a target.",
            inspect_only_no_authority,
        ),
        invariant(
            "descriptors.restore_never_reacquires_authority_silently",
            "A restored-layout-only or reattach-required session holds no live authority; only an \
             explicit reattach reacquires it.",
            restore_never_reacquires,
        ),
        invariant(
            "descriptors.live_authority_derived_from_mode_posture_drift",
            "Each session's live-authority flag equals the derivation from its mode, re-entry \
             posture, and adapter drift together.",
            live_authority_derived,
        ),
        invariant(
            "descriptors.attach_identity_preserved_picker_to_session",
            "Each session resolves its attach target and echoes the target identity, mutability, \
             privilege class, boundary, and adapter drift unchanged from the picker descriptor.",
            identity_preserved,
        ),
        invariant(
            "descriptors.adapter_drift_first_class",
            "Adapter drift is a first-class label on every session and target — drift, \
             reconnect-required, inspect-only, and unsupported-skew all appear and require \
             disclosure.",
            adapter_drift_first_class,
        ),
        invariant(
            "descriptors.every_session_routes_execution_context",
            "Every session carries a non-empty execution-context id and a typed entrypoint, so \
             re-entry, restart, and open-in-support reuse one canonical identity.",
            routes_execution_context,
        ),
        invariant(
            "descriptors.run_state_authority_consistent",
            "No reconstructed, awaiting-reattach, or terminated run state is paired with held live \
             authority.",
            run_state_authority_consistent,
        ),
        invariant(
            "descriptors.build_artifact_identity_preserved",
            "Build / artifact identity is preserved from the attach target into the session's \
             identity echo.",
            build_identity_preserved,
        ),
    ]
}

// ---------------------------------------------------------------------------
// Human-readable projection.
// ---------------------------------------------------------------------------

/// Renders the descriptor set as human-readable lines for CLI/headless and support.
pub fn m5_debug_session_descriptor_lines(set: &DebugSessionDescriptorSet) -> Vec<String> {
    let mut lines = Vec::new();
    lines.push(format!(
        "M5 debug-session descriptors — {} ({})",
        set.set_id, set.as_of
    ));
    lines.push(set.summary.clone());
    lines.push(format!(
        "Targets: {}  Sessions: {}  Invariants: {}",
        set.targets.len(),
        set.sessions.len(),
        set.invariants.len(),
    ));

    lines.push("Attach targets:".to_owned());
    for target in &set.targets {
        lines.push(format!(
            "  - {} [{}] boundary={} mutability={} privilege={} drift={}",
            target.descriptor_id,
            target.target_kind_token,
            target.boundary_token,
            target.mutability_token,
            target.privilege_token,
            target.adapter_drift_token,
        ));
        lines.push(format!("      {}", target.summary));
        lines.push(format!(
            "      evidence: {}",
            target.negotiation_evidence_ref
        ));
    }

    lines.push("Sessions:".to_owned());
    for session in &set.sessions {
        lines.push(format!(
            "  - {} mode={} entrypoint={} run_state={} reentry={} drift={} live_authority={}",
            session.session_id,
            session.mode_token,
            session.entrypoint_token,
            session.run_state_token,
            session.reentry_token,
            session.adapter_drift_token,
            session.holds_live_authority,
        ));
        lines.push(format!("      {}", session.summary));
        lines.push(format!("      target: {}", session.target_descriptor_ref));
        lines.push(format!("      proof: {}", session.proof_packet_ref));
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
