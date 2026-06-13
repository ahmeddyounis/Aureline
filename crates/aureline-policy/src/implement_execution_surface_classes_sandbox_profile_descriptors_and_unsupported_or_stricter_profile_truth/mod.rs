//! Execution-surface classes, sandbox-profile descriptors, and
//! unsupported-or-stricter-profile resolution for the M5 executing surfaces.
//!
//! The frozen runtime-authority matrix pins, per claimed M5 executing surface,
//! one **default** sandbox profile, approval-ticket posture, capability
//! envelope, secret scope, degraded fallback, and unsupported-profile behavior.
//! That matrix is platform-agnostic: it states the *intended* profile but not
//! what actually runs on a given platform when the intended isolation backend
//! is missing. This module closes that gap.
//!
//! It adds three things on top of the matrix and consumes it directly rather
//! than cloning its prose:
//!
//! 1. An **execution-surface class taxonomy**
//!    ([`M5ExecutionLaunchPath`]) that binds every concrete M5 launch path —
//!    task runs, terminal sessions, notebook cells, request/connector sends,
//!    database queries, debug sessions, AI tool calls, browser-routed actions,
//!    and remote mutations — to the single matrix surface whose authority row
//!    governs it, so per-surface wording stops drifting.
//! 2. **Sandbox-profile descriptors** ([`M5SandboxProfileDescriptor`]) that give
//!    every profile a stable id, version, backend class, isolation summary, and
//!    capability ceiling, so desktop, CLI/headless, diagnostics, and support
//!    surfaces display the same profile id, version, and backend class.
//! 3. **Unsupported-or-stricter-profile resolution**
//!    ([`resolve_surface_on_platform`]) that, for each surface on each platform,
//!    resolves the default profile to an effective profile — keeping it,
//!    narrowing to a stricter profile, or failing closed — and explains the
//!    reduced capability envelope.
//!
//! The track invariant is preserved: resolution never *widens* authority. A
//! surface whose default isolation backend is unavailable narrows to a strictly
//! more isolated profile (ultimately inert read-only) or fails closed; it never
//! falls back to a less isolated profile. Missing profile coverage narrows the
//! affected rows instead of letting them masquerade as parity-ready.
//!
//! The boundary schema is
//! [`schemas/execution-auth/implement-execution-surface-classes-sandbox-profile-descriptors-and-unsupported-or-stricter-profile-truth.schema.json`](../../../../schemas/execution-auth/implement-execution-surface-classes-sandbox-profile-descriptors-and-unsupported-or-stricter-profile-truth.schema.json).
//! The contract doc is
//! [`docs/execution-auth/m5/implement-execution-surface-classes-sandbox-profile-descriptors-and-unsupported-or-stricter-profile-truth.md`](../../../../docs/execution-auth/m5/implement-execution-surface-classes-sandbox-profile-descriptors-and-unsupported-or-stricter-profile-truth.md).

#[cfg(test)]
mod tests;

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

use super::freeze_the_m5_runtime_authority_approval_ticket_sandbox_profile_and_capability_envelope_matrix::{
    frozen_stable_m5_runtime_authority_matrix_packet, M5CapabilityClass, M5ExecutingSurface,
    M5RuntimeAuthorityMatrixSurfaceRow, M5RuntimeAuthorityQualificationClass, M5SandboxProfile,
    M5UnsupportedProfileBehavior, M5_RUNTIME_AUTHORITY_MATRIX_PACKET_ID,
    M5_RUNTIME_AUTHORITY_MATRIX_SCHEMA_REF,
};

/// Stable record-kind tag carried by [`M5ExecutionSurfaceResolutionPacket`].
pub const M5_EXECUTION_SURFACE_RESOLUTION_RECORD_KIND: &str =
    "implement_execution_surface_classes_sandbox_profile_descriptors_and_unsupported_or_stricter_profile_truth";

/// Schema version for the M5 execution-surface resolution records.
pub const M5_EXECUTION_SURFACE_RESOLUTION_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the boundary schema.
pub const M5_EXECUTION_SURFACE_RESOLUTION_SCHEMA_REF: &str =
    "schemas/execution-auth/implement-execution-surface-classes-sandbox-profile-descriptors-and-unsupported-or-stricter-profile-truth.schema.json";

/// Repo-relative path of the contract doc.
pub const M5_EXECUTION_SURFACE_RESOLUTION_DOC_REF: &str =
    "docs/execution-auth/m5/implement-execution-surface-classes-sandbox-profile-descriptors-and-unsupported-or-stricter-profile-truth.md";

/// Repo-relative path of the checked support-export artifact.
pub const M5_EXECUTION_SURFACE_RESOLUTION_ARTIFACT_REF: &str =
    "artifacts/execution-auth/m5/implement-execution-surface-classes-sandbox-profile-descriptors-and-unsupported-or-stricter-profile-truth/support_export.json";

/// Repo-relative path of the checked Markdown summary.
pub const M5_EXECUTION_SURFACE_RESOLUTION_SUMMARY_REF: &str =
    "artifacts/execution-auth/m5/implement-execution-surface-classes-sandbox-profile-descriptors-and-unsupported-or-stricter-profile-truth.md";

/// Repo-relative path of the protected fixture directory.
pub const M5_EXECUTION_SURFACE_RESOLUTION_FIXTURE_DIR: &str =
    "fixtures/execution-auth/m5/implement-execution-surface-classes-sandbox-profile-descriptors-and-unsupported-or-stricter-profile-truth";

/// Stable packet id minted by [`frozen_stable_m5_execution_surface_resolution_packet`].
pub const M5_EXECUTION_SURFACE_RESOLUTION_PACKET_ID: &str =
    "m5-execution-surface-resolution:stable:0001";

/// Coarse execution backend class shared by one or more sandbox profiles.
///
/// The backend class is the *surface class* projected to operators: it answers
/// "what kind of execution boundary is this?" independently of the finer
/// per-profile id. Consumers group and explain surfaces by backend class.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ExecutionBackendClass {
    /// In-process execution under the host policy epoch; no isolation boundary.
    InProcess,
    /// Network egress brokered through the transport plane; no local execution.
    BrokeredNetwork,
    /// Locally isolated execution (subprocess or container).
    LocalIsolated,
    /// Isolated remote runtime confined to a managed sandbox.
    RemoteIsolated,
    /// No execution; the surface is inert.
    Inert,
}

impl M5ExecutionBackendClass {
    /// Stable token recorded in resolution records.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::InProcess => "in_process",
            Self::BrokeredNetwork => "brokered_network",
            Self::LocalIsolated => "local_isolated",
            Self::RemoteIsolated => "remote_isolated",
            Self::Inert => "inert",
        }
    }
}

/// Extension methods that derive descriptor truth for a frozen sandbox profile.
///
/// These complement [`M5SandboxProfile`] (frozen by the matrix) with the
/// descriptor facts this resolution layer adds: stable id, version, backend
/// class, isolation rank, capability ceiling, and whether the profile is a
/// network-broker lane (excluded from the local-execution narrowing ladder).
trait M5SandboxProfileDescriptorExt {
    fn profile_id(self) -> &'static str;
    fn backend_class(self) -> M5ExecutionBackendClass;
    fn isolation_rank(self) -> u8;
    fn network_lane(self) -> bool;
    fn capability_ceiling(self) -> Vec<M5CapabilityClass>;
    fn isolation_summary(self) -> &'static str;
}

impl M5SandboxProfileDescriptorExt for M5SandboxProfile {
    /// Stable, dotted profile id displayed identically across every consumer.
    fn profile_id(self) -> &'static str {
        match self {
            Self::InertNoExecution => "sandbox.inert_no_execution",
            Self::InProcessTrustedLocal => "sandbox.in_process_trusted_local",
            Self::BrokeredNetworkOnly => "sandbox.brokered_network_only",
            Self::SubprocessIsolatedLocal => "sandbox.subprocess_isolated_local",
            Self::ContainerIsolatedLocal => "sandbox.container_isolated_local",
            Self::IsolatedRemoteRuntime => "sandbox.isolated_remote_runtime",
        }
    }

    /// Coarse backend class this profile belongs to.
    fn backend_class(self) -> M5ExecutionBackendClass {
        match self {
            Self::InertNoExecution => M5ExecutionBackendClass::Inert,
            Self::InProcessTrustedLocal => M5ExecutionBackendClass::InProcess,
            Self::BrokeredNetworkOnly => M5ExecutionBackendClass::BrokeredNetwork,
            Self::SubprocessIsolatedLocal | Self::ContainerIsolatedLocal => {
                M5ExecutionBackendClass::LocalIsolated
            }
            Self::IsolatedRemoteRuntime => M5ExecutionBackendClass::RemoteIsolated,
        }
    }

    /// Isolation rank; higher means a stricter (more isolated) boundary.
    ///
    /// The local-execution narrowing ladder is ordered by this rank. The
    /// network-broker lane is ranked above the local-execution profiles but is
    /// excluded from that ladder by [`network_lane`](Self::network_lane).
    fn isolation_rank(self) -> u8 {
        match self {
            Self::InProcessTrustedLocal => 0,
            Self::SubprocessIsolatedLocal => 1,
            Self::ContainerIsolatedLocal => 2,
            Self::IsolatedRemoteRuntime => 3,
            Self::BrokeredNetworkOnly => 4,
            Self::InertNoExecution => 5,
        }
    }

    /// Whether this profile is a network-broker lane rather than a code-execution
    /// boundary; network-lane profiles are never a narrowing target for a
    /// code-executing surface.
    fn network_lane(self) -> bool {
        matches!(self, Self::BrokeredNetworkOnly)
    }

    /// Maximum capability classes this profile can host.
    ///
    /// A surface's effective capability envelope is the intersection of its
    /// matrix-granted classes with this ceiling, so narrowing to a stricter
    /// profile strips any capability the stricter profile cannot host.
    fn capability_ceiling(self) -> Vec<M5CapabilityClass> {
        use M5CapabilityClass as Cap;
        match self {
            Self::InertNoExecution => vec![Cap::ReadWorkspace],
            Self::InProcessTrustedLocal => vec![
                Cap::ReadWorkspace,
                Cap::WriteWorkspace,
                Cap::NetworkEgress,
                Cap::ProcessSpawn,
                Cap::SecretHandleProjection,
                Cap::DatabaseRead,
                Cap::DatabaseWrite,
                Cap::BrowserNavigation,
            ],
            Self::BrokeredNetworkOnly => vec![
                Cap::NetworkEgress,
                Cap::SecretHandleProjection,
                Cap::DatabaseRead,
                Cap::DatabaseWrite,
                Cap::RemoteMutation,
            ],
            Self::SubprocessIsolatedLocal | Self::ContainerIsolatedLocal => vec![
                Cap::ReadWorkspace,
                Cap::WriteWorkspace,
                Cap::NetworkEgress,
                Cap::ProcessSpawn,
                Cap::SecretHandleProjection,
            ],
            Self::IsolatedRemoteRuntime => vec![
                Cap::ReadWorkspace,
                Cap::WriteWorkspace,
                Cap::NetworkEgress,
                Cap::ProcessSpawn,
                Cap::SecretHandleProjection,
                Cap::DatabaseRead,
                Cap::DatabaseWrite,
                Cap::RemoteMutation,
                Cap::BrowserNavigation,
            ],
        }
    }

    /// One-line operator-facing isolation summary.
    fn isolation_summary(self) -> &'static str {
        match self {
            Self::InertNoExecution => "No code execution; read-only inert surface.",
            Self::InProcessTrustedLocal => {
                "Runs in-process under the host policy epoch with no isolation boundary."
            }
            Self::BrokeredNetworkOnly => {
                "No local process; network egress brokered through the transport plane."
            }
            Self::SubprocessIsolatedLocal => {
                "Runs in an isolated local subprocess with a scoped capability envelope."
            }
            Self::ContainerIsolatedLocal => "Runs in a container-isolated local runtime.",
            Self::IsolatedRemoteRuntime => {
                "Runs in an isolated remote runtime confined to a managed sandbox."
            }
        }
    }
}

/// Pinned descriptor version for every sandbox profile in this packet.
pub const M5_SANDBOX_PROFILE_DESCRIPTOR_VERSION: u32 = 1;

/// Concrete M5 launch path bound to a governing matrix surface.
///
/// The launch path is the surface an operator actually triggers; the governing
/// matrix surface is the authority row that defines its profile, ticket posture,
/// and capability envelope. Several launch paths can share one authority row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ExecutionLaunchPath {
    /// Build/test/run task execution.
    TaskExecution,
    /// Interactive terminal session.
    TerminalSession,
    /// Notebook cell execution.
    NotebookCell,
    /// Request/API send.
    RequestSend,
    /// Database query or action.
    DatabaseQuery,
    /// Debug session launch or attach.
    DebugSession,
    /// External connector action.
    ConnectorAction,
    /// AI tool call.
    AiToolCall,
    /// Browser-routed action.
    BrowserRoutedAction,
    /// Remote mutation.
    RemoteMutation,
}

impl M5ExecutionLaunchPath {
    /// Every launch path, in declaration order.
    pub const ALL: [Self; 10] = [
        Self::TaskExecution,
        Self::TerminalSession,
        Self::NotebookCell,
        Self::RequestSend,
        Self::DatabaseQuery,
        Self::DebugSession,
        Self::ConnectorAction,
        Self::AiToolCall,
        Self::BrowserRoutedAction,
        Self::RemoteMutation,
    ];

    /// Stable token recorded in resolution records.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::TaskExecution => "task_execution",
            Self::TerminalSession => "terminal_session",
            Self::NotebookCell => "notebook_cell",
            Self::RequestSend => "request_send",
            Self::DatabaseQuery => "database_query",
            Self::DebugSession => "debug_session",
            Self::ConnectorAction => "connector_action",
            Self::AiToolCall => "ai_tool_call",
            Self::BrowserRoutedAction => "browser_routed_action",
            Self::RemoteMutation => "remote_mutation",
        }
    }

    /// Matrix surface whose authority row governs this launch path.
    pub const fn governing_surface(self) -> M5ExecutingSurface {
        match self {
            // Task runs share the isolated-local-subprocess authority row used
            // for generator hooks: spawn a local process, may write the
            // workspace, gated per action.
            Self::TaskExecution => M5ExecutingSurface::ScaffoldHook,
            // Terminal, notebook, and debug sessions are all per-session isolated
            // local subprocess kernels and share the notebook-kernel authority.
            Self::TerminalSession | Self::NotebookCell | Self::DebugSession => {
                M5ExecutingSurface::NotebookKernel
            }
            // Request sends and connector actions are brokered network-only sends.
            Self::RequestSend | Self::ConnectorAction => M5ExecutingSurface::RequestApiSend,
            Self::DatabaseQuery => M5ExecutingSurface::DatabaseAction,
            Self::AiToolCall => M5ExecutingSurface::AiTool,
            Self::BrowserRoutedAction => M5ExecutingSurface::BrowserRoutedAction,
            Self::RemoteMutation => M5ExecutingSurface::RemoteMutation,
        }
    }
}

/// Platform on which an M5 executing surface may launch.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ExecutionPlatform {
    /// Linux desktop build.
    LinuxDesktop,
    /// macOS desktop build.
    MacosDesktop,
    /// Windows desktop build.
    WindowsDesktop,
    /// Managed remote runtime.
    ManagedRemoteRuntime,
    /// Headless CI / automation runner.
    HeadlessCi,
}

impl M5ExecutionPlatform {
    /// Every platform, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::LinuxDesktop,
        Self::MacosDesktop,
        Self::WindowsDesktop,
        Self::ManagedRemoteRuntime,
        Self::HeadlessCi,
    ];

    /// Stable token recorded in resolution records.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LinuxDesktop => "linux_desktop",
            Self::MacosDesktop => "macos_desktop",
            Self::WindowsDesktop => "windows_desktop",
            Self::ManagedRemoteRuntime => "managed_remote_runtime",
            Self::HeadlessCi => "headless_ci",
        }
    }

    /// Sandbox profiles whose enforcement backend is available on this platform.
    ///
    /// This is the frozen platform-support table the resolver consults. macOS
    /// and Windows have no native local-container backend; the headless CI
    /// runner has neither container nor remote-runtime backends and no
    /// in-process trusted-local lane; the managed remote runtime has no
    /// in-process trusted-local lane.
    pub fn available_profiles(self) -> Vec<M5SandboxProfile> {
        use M5SandboxProfile as Profile;
        match self {
            Self::LinuxDesktop => vec![
                Profile::InProcessTrustedLocal,
                Profile::BrokeredNetworkOnly,
                Profile::SubprocessIsolatedLocal,
                Profile::ContainerIsolatedLocal,
                Profile::IsolatedRemoteRuntime,
                Profile::InertNoExecution,
            ],
            Self::MacosDesktop | Self::WindowsDesktop => vec![
                Profile::InProcessTrustedLocal,
                Profile::BrokeredNetworkOnly,
                Profile::SubprocessIsolatedLocal,
                Profile::IsolatedRemoteRuntime,
                Profile::InertNoExecution,
            ],
            Self::ManagedRemoteRuntime => vec![
                Profile::BrokeredNetworkOnly,
                Profile::SubprocessIsolatedLocal,
                Profile::ContainerIsolatedLocal,
                Profile::IsolatedRemoteRuntime,
                Profile::InertNoExecution,
            ],
            Self::HeadlessCi => vec![
                Profile::BrokeredNetworkOnly,
                Profile::SubprocessIsolatedLocal,
                Profile::InertNoExecution,
            ],
        }
    }
}

/// Resolution status of a surface's default profile on a platform.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ProfileResolutionStatus {
    /// The default profile is supported; full envelope honored.
    Supported,
    /// The default profile is unavailable; narrowed to a stricter profile.
    NarrowedToStricterProfile,
    /// The default profile is unavailable and the surface fails closed.
    UnsupportedFailClosed,
    /// The default profile is unavailable and the surface is disabled with reason.
    DisabledWithReason,
}

impl M5ProfileResolutionStatus {
    /// Stable token recorded in resolution records.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Supported => "supported",
            Self::NarrowedToStricterProfile => "narrowed_to_stricter_profile",
            Self::UnsupportedFailClosed => "unsupported_fail_closed",
            Self::DisabledWithReason => "disabled_with_reason",
        }
    }

    /// Whether the surface still has an effective execution profile.
    pub const fn has_effective_profile(self) -> bool {
        matches!(self, Self::Supported | Self::NarrowedToStricterProfile)
    }
}

/// Export-safe descriptor for one sandbox profile.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5SandboxProfileDescriptor {
    /// Frozen sandbox profile this descriptor describes.
    pub profile: M5SandboxProfile,
    /// Stable dotted profile id.
    pub profile_id: String,
    /// Descriptor version.
    pub profile_version: u32,
    /// Coarse backend class.
    pub backend_class: M5ExecutionBackendClass,
    /// Isolation rank; higher is stricter.
    pub isolation_rank: u8,
    /// Whether this profile is a network-broker lane.
    pub network_lane: bool,
    /// Operator-facing isolation summary.
    pub isolation_summary: String,
    /// Maximum capability classes this profile can host.
    pub capability_ceiling: Vec<M5CapabilityClass>,
}

impl M5SandboxProfileDescriptor {
    /// Builds the descriptor for a sandbox profile.
    fn for_profile(profile: M5SandboxProfile) -> Self {
        Self {
            profile,
            profile_id: profile.profile_id().to_owned(),
            profile_version: M5_SANDBOX_PROFILE_DESCRIPTOR_VERSION,
            backend_class: profile.backend_class(),
            isolation_rank: profile.isolation_rank(),
            network_lane: profile.network_lane(),
            isolation_summary: profile.isolation_summary().to_owned(),
            capability_ceiling: profile.capability_ceiling(),
        }
    }
}

/// Binding of a concrete launch path to its governing matrix surface.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ExecutionLaunchPathBinding {
    /// Concrete launch path operators trigger.
    pub launch_path: M5ExecutionLaunchPath,
    /// Matrix surface whose authority row governs the launch path.
    pub governing_surface: M5ExecutingSurface,
    /// Default backend class of the governing surface's profile.
    pub default_backend_class: M5ExecutionBackendClass,
    /// Why this launch path is governed by the named surface.
    pub note: String,
}

/// One surface resolved on one platform.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ResolvedSurfaceRow {
    /// Matrix surface being resolved.
    pub surface: M5ExecutingSurface,
    /// Default sandbox profile from the matrix.
    pub default_profile: M5SandboxProfile,
    /// Default profile id.
    pub default_profile_id: String,
    /// Default profile version.
    pub default_profile_version: u32,
    /// Default backend class.
    pub default_backend_class: M5ExecutionBackendClass,
    /// Resolution status of the default profile on this platform.
    pub resolution_status: M5ProfileResolutionStatus,
    /// Effective profile after resolution; `None` when the surface fails closed.
    pub effective_profile: Option<M5SandboxProfile>,
    /// Effective profile id; `None` when the surface fails closed.
    pub effective_profile_id: Option<String>,
    /// Effective profile version; `None` when the surface fails closed.
    pub effective_profile_version: Option<u32>,
    /// Effective backend class; `None` when the surface fails closed.
    pub effective_backend_class: Option<M5ExecutionBackendClass>,
    /// Effective qualification after resolution; narrows on degrade.
    pub effective_qualification: M5RuntimeAuthorityQualificationClass,
    /// Capability classes stripped relative to the matrix-granted envelope.
    pub stripped_capability_classes: Vec<M5CapabilityClass>,
    /// Operator-facing reduced-capability explanation.
    pub reduced_capability_explanation: String,
}

/// One platform's full resolution across every matrix surface.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5PlatformResolution {
    /// Platform being resolved.
    pub platform: M5ExecutionPlatform,
    /// Sandbox profiles available on this platform.
    pub available_profiles: Vec<M5SandboxProfile>,
    /// Resolved row per matrix surface.
    pub resolved_rows: Vec<M5ResolvedSurfaceRow>,
}

impl M5PlatformResolution {
    /// Validates this platform resolution in isolation.
    ///
    /// Checks the per-row invariants that hold without the full packet: status
    /// vs effective profile presence, never-widen, stricter-on-narrow, and a
    /// non-empty explanation. Used both by the packet validator and by the
    /// checked-in per-platform fixtures.
    pub fn validate(&self) -> Vec<M5ExecutionSurfaceResolutionViolation> {
        use M5ExecutionSurfaceResolutionViolation as V;
        let mut violations = Vec::new();
        let available: BTreeSet<M5SandboxProfile> =
            self.available_profiles.iter().copied().collect();
        for row in &self.resolved_rows {
            if row.reduced_capability_explanation.trim().is_empty() {
                violations.push(V::ReducedCapabilityExplanationMissing);
            }
            match row.resolution_status {
                M5ProfileResolutionStatus::Supported => {
                    if row.effective_profile != Some(row.default_profile) {
                        violations.push(V::SupportedRowNotDefaultProfile);
                    }
                    if !available.contains(&row.default_profile) {
                        violations.push(V::ResolvedProfileUnavailable);
                    }
                }
                M5ProfileResolutionStatus::NarrowedToStricterProfile => match row.effective_profile
                {
                    None => violations.push(V::NarrowedRowMissingEffectiveProfile),
                    Some(effective) => {
                        if !available.contains(&effective) {
                            violations.push(V::ResolvedProfileUnavailable);
                        }
                        if effective.isolation_rank() <= row.default_profile.isolation_rank() {
                            violations.push(V::ProfileWidenedOnNarrow);
                        }
                    }
                },
                M5ProfileResolutionStatus::UnsupportedFailClosed
                | M5ProfileResolutionStatus::DisabledWithReason => {
                    if row.effective_profile.is_some() {
                        violations.push(V::FailClosedRowHasEffectiveProfile);
                    }
                    if row.effective_qualification
                        != M5RuntimeAuthorityQualificationClass::Unavailable
                    {
                        violations.push(V::FailClosedRowNotUnavailable);
                    }
                }
            }
        }
        violations
    }
}

/// Trust and isolation review for the resolution packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ExecutionSurfaceResolutionTrustReview {
    /// Resolution never widens authority below the default profile's isolation.
    pub never_widens_on_narrow: bool,
    /// A surface fails closed when no stricter profile is available.
    pub fail_closed_when_no_stricter_profile: bool,
    /// Missing profile coverage narrows the affected rows instead of passing.
    pub missing_profile_coverage_narrows: bool,
    /// Desktop, CLI/headless, diagnostics, and support read one descriptor set.
    pub same_descriptor_across_consumers: bool,
    /// Every narrowed or disabled row carries a reduced-capability explanation.
    pub reduced_capability_explained: bool,
    /// No raw secret material appears in the export.
    pub no_raw_secret_material_in_exports: bool,
}

/// Consumer projection for the resolution packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ExecutionSurfaceResolutionConsumerProjection {
    /// Desktop shell shows the profile id, version, and backend class.
    pub desktop_shows_profile_descriptor: bool,
    /// CLI / headless shows the profile id, version, and backend class.
    pub cli_headless_shows_profile_descriptor: bool,
    /// Diagnostics shows the profile id, version, and backend class.
    pub diagnostics_shows_profile_descriptor: bool,
    /// Support export shows the full resolution.
    pub support_export_shows_resolution: bool,
    /// Help / About shows the backend class and narrowing.
    pub help_about_shows_backend_class: bool,
    /// Release evidence consumes this resolution instead of cloning prose.
    pub release_evidence_consumes_resolution: bool,
    /// Command and policy surfaces reference the same resolution.
    pub command_and_policy_reference_same_resolution: bool,
    /// Preview / Labs label applied to narrowed or disabled surfaces.
    pub preview_labs_label_for_narrowed_surfaces: bool,
}

/// Constructor input for [`M5ExecutionSurfaceResolutionPacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M5ExecutionSurfaceResolutionPacketInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable resolution label.
    pub resolution_label: String,
    /// Packet id of the frozen matrix this resolution is derived from.
    pub matrix_packet_id: String,
    /// Launch-path bindings.
    pub launch_path_bindings: Vec<M5ExecutionLaunchPathBinding>,
    /// Sandbox-profile descriptors.
    pub profile_descriptors: Vec<M5SandboxProfileDescriptor>,
    /// Per-platform resolutions.
    pub platform_resolutions: Vec<M5PlatformResolution>,
    /// Trust review block.
    pub trust_review: M5ExecutionSurfaceResolutionTrustReview,
    /// Consumer projection block.
    pub consumer_projection: M5ExecutionSurfaceResolutionConsumerProjection,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

/// Export-safe M5 execution-surface resolution packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ExecutionSurfaceResolutionPacket {
    /// Record kind; must equal [`M5_EXECUTION_SURFACE_RESOLUTION_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`M5_EXECUTION_SURFACE_RESOLUTION_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable resolution label.
    pub resolution_label: String,
    /// Packet id of the frozen matrix this resolution is derived from.
    pub matrix_packet_id: String,
    /// Launch-path bindings.
    pub launch_path_bindings: Vec<M5ExecutionLaunchPathBinding>,
    /// Sandbox-profile descriptors.
    pub profile_descriptors: Vec<M5SandboxProfileDescriptor>,
    /// Per-platform resolutions.
    pub platform_resolutions: Vec<M5PlatformResolution>,
    /// Trust review block.
    pub trust_review: M5ExecutionSurfaceResolutionTrustReview,
    /// Consumer projection block.
    pub consumer_projection: M5ExecutionSurfaceResolutionConsumerProjection,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

impl M5ExecutionSurfaceResolutionPacket {
    /// Builds a resolution packet from frozen input.
    pub fn new(input: M5ExecutionSurfaceResolutionPacketInput) -> Self {
        Self {
            record_kind: M5_EXECUTION_SURFACE_RESOLUTION_RECORD_KIND.to_owned(),
            schema_version: M5_EXECUTION_SURFACE_RESOLUTION_SCHEMA_VERSION,
            packet_id: input.packet_id,
            resolution_label: input.resolution_label,
            matrix_packet_id: input.matrix_packet_id,
            launch_path_bindings: input.launch_path_bindings,
            profile_descriptors: input.profile_descriptors,
            platform_resolutions: input.platform_resolutions,
            trust_review: input.trust_review,
            consumer_projection: input.consumer_projection,
            source_contract_refs: input.source_contract_refs,
            redaction_class_token: input.redaction_class_token,
            minted_at: input.minted_at,
        }
    }

    /// Validates the resolution-packet invariants.
    pub fn validate(&self) -> Vec<M5ExecutionSurfaceResolutionViolation> {
        use M5ExecutionSurfaceResolutionViolation as V;
        let mut violations = Vec::new();

        if self.record_kind != M5_EXECUTION_SURFACE_RESOLUTION_RECORD_KIND {
            violations.push(V::WrongRecordKind);
        }
        if self.schema_version != M5_EXECUTION_SURFACE_RESOLUTION_SCHEMA_VERSION {
            violations.push(V::WrongSchemaVersion);
        }
        if self.packet_id.trim().is_empty()
            || self.resolution_label.trim().is_empty()
            || self.matrix_packet_id.trim().is_empty()
            || self.redaction_class_token.trim().is_empty()
            || self.minted_at.trim().is_empty()
        {
            violations.push(V::MissingIdentity);
        }

        validate_source_contracts(self, &mut violations);
        validate_descriptors(self, &mut violations);
        validate_launch_path_bindings(self, &mut violations);
        validate_platform_resolutions(self, &mut violations);
        validate_trust_review(self, &mut violations);
        validate_consumer_projection(self, &mut violations);

        if json_contains_forbidden_boundary_material(
            &serde_json::to_value(self).expect("m5 execution-surface resolution packet serializes"),
        ) {
            violations.push(V::RawBoundaryMaterialInExport);
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
            .expect("m5 execution-surface resolution packet serializes")
    }

    /// Deterministic Markdown summary for support, help, or release handoff.
    pub fn render_markdown_summary(&self) -> String {
        let mut out = String::new();
        out.push_str("# M5 Execution-Surface Resolution\n\n");
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- Label: `{}`\n", self.resolution_label));
        out.push_str(&format!(
            "- Derived from matrix: `{}`\n",
            self.matrix_packet_id
        ));
        out.push_str(&format!(
            "- Profiles: {} · Launch paths: {} · Platforms: {}\n",
            self.profile_descriptors.len(),
            self.launch_path_bindings.len(),
            self.platform_resolutions.len()
        ));

        out.push_str("\n## Launch-path classes\n\n");
        for binding in &self.launch_path_bindings {
            out.push_str(&format!(
                "- **{}** → surface `{}` ({})\n",
                binding.launch_path.as_str(),
                binding.governing_surface.as_str(),
                binding.default_backend_class.as_str()
            ));
        }

        out.push_str("\n## Sandbox-profile descriptors\n\n");
        for descriptor in &self.profile_descriptors {
            out.push_str(&format!(
                "- `{}` v{} ({}) — {}\n",
                descriptor.profile_id,
                descriptor.profile_version,
                descriptor.backend_class.as_str(),
                descriptor.isolation_summary
            ));
        }

        out.push_str("\n## Platform resolution\n\n");
        for resolution in &self.platform_resolutions {
            out.push_str(&format!("### {}\n\n", resolution.platform.as_str()));
            for row in &resolution.resolved_rows {
                let effective = row
                    .effective_profile_id
                    .as_deref()
                    .unwrap_or("(none — fails closed)");
                out.push_str(&format!(
                    "- **{}**: `{}` → `{}` [{}] · qual `{}`\n",
                    row.surface.as_str(),
                    row.default_profile_id,
                    effective,
                    row.resolution_status.as_str(),
                    row.effective_qualification.as_str()
                ));
            }
            out.push('\n');
        }
        out
    }
}

/// Errors emitted when reading the checked-in resolution export.
#[derive(Debug)]
pub enum M5ExecutionSurfaceResolutionArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<M5ExecutionSurfaceResolutionViolation>),
}

impl fmt::Display for M5ExecutionSurfaceResolutionArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(
                    formatter,
                    "m5 execution-surface resolution export parse failed: {error}"
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
                    "m5 execution-surface resolution export failed validation: {tokens}"
                )
            }
        }
    }
}

impl Error for M5ExecutionSurfaceResolutionArtifactError {}

/// Validation failures emitted by [`M5ExecutionSurfaceResolutionPacket::validate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum M5ExecutionSurfaceResolutionViolation {
    /// Packet record kind is wrong.
    WrongRecordKind,
    /// Packet schema version is wrong.
    WrongSchemaVersion,
    /// Required identity field is missing.
    MissingIdentity,
    /// Source contract refs are incomplete.
    MissingSourceContracts,
    /// The matrix packet id does not match the frozen matrix.
    MatrixPacketIdMismatch,
    /// A sandbox profile is referenced without a descriptor.
    ProfileDescriptorMissing,
    /// A profile descriptor disagrees with the derived descriptor truth.
    ProfileDescriptorInconsistent,
    /// A launch-path binding is missing or names an unknown surface.
    LaunchPathBindingIncomplete,
    /// A required platform is missing from the resolution.
    MissingPlatformCoverage,
    /// A required surface is missing from a platform's resolution.
    MissingProfileCoverage,
    /// A supported row's effective profile is not its default profile.
    SupportedRowNotDefaultProfile,
    /// A narrowed row carries no effective profile.
    NarrowedRowMissingEffectiveProfile,
    /// A narrowed row's effective profile is not stricter than the default.
    ProfileWidenedOnNarrow,
    /// A resolved profile is not available on its platform.
    ResolvedProfileUnavailable,
    /// A fail-closed or disabled row still carries an effective profile.
    FailClosedRowHasEffectiveProfile,
    /// A fail-closed or disabled row is not narrowed to unavailable.
    FailClosedRowNotUnavailable,
    /// A narrowed or disabled row carries no reduced-capability explanation.
    ReducedCapabilityExplanationMissing,
    /// Trust review does not satisfy required invariants.
    TrustReviewIncomplete,
    /// Consumer projection does not satisfy required invariants.
    ConsumerProjectionIncomplete,
    /// Export contains raw boundary material.
    RawBoundaryMaterialInExport,
}

impl M5ExecutionSurfaceResolutionViolation {
    /// Stable token used in tests and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WrongRecordKind => "wrong_record_kind",
            Self::WrongSchemaVersion => "wrong_schema_version",
            Self::MissingIdentity => "missing_identity",
            Self::MissingSourceContracts => "missing_source_contracts",
            Self::MatrixPacketIdMismatch => "matrix_packet_id_mismatch",
            Self::ProfileDescriptorMissing => "profile_descriptor_missing",
            Self::ProfileDescriptorInconsistent => "profile_descriptor_inconsistent",
            Self::LaunchPathBindingIncomplete => "launch_path_binding_incomplete",
            Self::MissingPlatformCoverage => "missing_platform_coverage",
            Self::MissingProfileCoverage => "missing_profile_coverage",
            Self::SupportedRowNotDefaultProfile => "supported_row_not_default_profile",
            Self::NarrowedRowMissingEffectiveProfile => "narrowed_row_missing_effective_profile",
            Self::ProfileWidenedOnNarrow => "profile_widened_on_narrow",
            Self::ResolvedProfileUnavailable => "resolved_profile_unavailable",
            Self::FailClosedRowHasEffectiveProfile => "fail_closed_row_has_effective_profile",
            Self::FailClosedRowNotUnavailable => "fail_closed_row_not_unavailable",
            Self::ReducedCapabilityExplanationMissing => "reduced_capability_explanation_missing",
            Self::TrustReviewIncomplete => "trust_review_incomplete",
            Self::ConsumerProjectionIncomplete => "consumer_projection_incomplete",
            Self::RawBoundaryMaterialInExport => "raw_boundary_material_in_export",
        }
    }
}

/// Narrows a qualification class by one notch toward unavailable.
fn narrow_one(
    qualification: M5RuntimeAuthorityQualificationClass,
) -> M5RuntimeAuthorityQualificationClass {
    use M5RuntimeAuthorityQualificationClass as Q;
    match qualification {
        Q::Stable => Q::Beta,
        Q::Beta => Q::Preview,
        Q::Preview => Q::Experimental,
        Q::Experimental | Q::Unavailable => Q::Unavailable,
        Q::Held => Q::Held,
    }
}

/// Resolves a matrix surface row on a platform into an effective profile row.
///
/// This is the core unsupported-or-stricter-profile logic. If the default
/// profile's backend is available the surface is [`Supported`]. Otherwise the
/// matrix's `unsupported_profile_behavior` decides: fail closed, disable with
/// reason, or narrow to the *least disruptive strictly more isolated* profile
/// available on the platform (never a network-broker lane, never a less
/// isolated profile). When no stricter profile is available the surface fails
/// closed.
///
/// [`Supported`]: M5ProfileResolutionStatus::Supported
pub fn resolve_surface_on_platform(
    row: &M5RuntimeAuthorityMatrixSurfaceRow,
    platform: M5ExecutionPlatform,
) -> M5ResolvedSurfaceRow {
    let available = platform.available_profiles();
    let default = row.default_sandbox_profile;
    let default_id = default.profile_id().to_owned();

    let mut resolved = M5ResolvedSurfaceRow {
        surface: row.surface,
        default_profile: default,
        default_profile_id: default_id.clone(),
        default_profile_version: M5_SANDBOX_PROFILE_DESCRIPTOR_VERSION,
        default_backend_class: default.backend_class(),
        resolution_status: M5ProfileResolutionStatus::Supported,
        effective_profile: Some(default),
        effective_profile_id: Some(default_id),
        effective_profile_version: Some(M5_SANDBOX_PROFILE_DESCRIPTOR_VERSION),
        effective_backend_class: Some(default.backend_class()),
        effective_qualification: row.qualification,
        stripped_capability_classes: Vec::new(),
        reduced_capability_explanation: format!(
            "Runs under its default profile `{}` ({}) on {}; full capability envelope honored.",
            default.profile_id(),
            default.backend_class().as_str(),
            platform.as_str()
        ),
    };

    if available.contains(&default) {
        return resolved;
    }

    match row.unsupported_profile_behavior {
        M5UnsupportedProfileBehavior::DisableSurfaceWithReason => {
            set_no_execution(
                &mut resolved,
                M5ProfileResolutionStatus::DisabledWithReason,
                format!(
                    "Default profile `{}` backend is unavailable on {}; surface disabled with reason.",
                    default.profile_id(),
                    platform.as_str()
                ),
            );
        }
        M5UnsupportedProfileBehavior::FailClosedUnsupported
        | M5UnsupportedProfileBehavior::FullySupportedAllPlatforms => {
            // A surface marked fully-supported that is nonetheless unavailable
            // here must still fail closed rather than silently widen.
            set_no_execution(
                &mut resolved,
                M5ProfileResolutionStatus::UnsupportedFailClosed,
                format!(
                    "Default profile `{}` backend is unavailable on {}; surface fails closed and execution is blocked.",
                    default.profile_id(),
                    platform.as_str()
                ),
            );
        }
        M5UnsupportedProfileBehavior::NarrowToStricterProfile => {
            match stricter_available_profile(default, &available) {
                Some(stricter) => set_narrowed(&mut resolved, row, stricter, platform),
                None => set_no_execution(
                    &mut resolved,
                    M5ProfileResolutionStatus::UnsupportedFailClosed,
                    format!(
                        "Default profile `{}` backend is unavailable on {} and no stricter profile is available; surface fails closed.",
                        default.profile_id(),
                        platform.as_str()
                    ),
                ),
            }
        }
    }

    resolved
}

/// Picks the least disruptive strictly-more-isolated profile available.
///
/// Candidates are the platform's available profiles, excluding network-broker
/// lanes, with an isolation rank strictly greater than the default; the one
/// with the smallest such rank is chosen so narrowing is minimal while never
/// widening.
fn stricter_available_profile(
    default: M5SandboxProfile,
    available: &[M5SandboxProfile],
) -> Option<M5SandboxProfile> {
    available
        .iter()
        .copied()
        .filter(|candidate| {
            !candidate.network_lane() && candidate.isolation_rank() > default.isolation_rank()
        })
        .min_by_key(|candidate| candidate.isolation_rank())
}

/// Applies a narrowed-to-stricter resolution to a row.
fn set_narrowed(
    resolved: &mut M5ResolvedSurfaceRow,
    row: &M5RuntimeAuthorityMatrixSurfaceRow,
    effective: M5SandboxProfile,
    platform: M5ExecutionPlatform,
) {
    let ceiling: BTreeSet<M5CapabilityClass> = effective.capability_ceiling().into_iter().collect();
    let stripped: Vec<M5CapabilityClass> = row
        .allowed_capability_classes
        .iter()
        .copied()
        .filter(|cap| !ceiling.contains(cap))
        .collect();
    let stripped_tokens = stripped
        .iter()
        .map(|cap| cap.as_str())
        .collect::<Vec<_>>()
        .join(", ");
    resolved.resolution_status = M5ProfileResolutionStatus::NarrowedToStricterProfile;
    resolved.effective_profile = Some(effective);
    resolved.effective_profile_id = Some(effective.profile_id().to_owned());
    resolved.effective_profile_version = Some(M5_SANDBOX_PROFILE_DESCRIPTOR_VERSION);
    resolved.effective_backend_class = Some(effective.backend_class());
    resolved.effective_qualification = narrow_one(row.qualification);
    resolved.reduced_capability_explanation = format!(
        "Default profile `{}` backend is unavailable on {}; narrowed to stricter profile `{}` ({}). Stripped capabilities: {}.",
        row.default_sandbox_profile.profile_id(),
        platform.as_str(),
        effective.profile_id(),
        effective.backend_class().as_str(),
        if stripped_tokens.is_empty() {
            "none".to_owned()
        } else {
            stripped_tokens
        }
    );
    resolved.stripped_capability_classes = stripped;
}

/// Applies a no-execution resolution (fail closed / disabled) to a row.
fn set_no_execution(
    resolved: &mut M5ResolvedSurfaceRow,
    status: M5ProfileResolutionStatus,
    explanation: String,
) {
    resolved.resolution_status = status;
    resolved.effective_profile = None;
    resolved.effective_profile_id = None;
    resolved.effective_profile_version = None;
    resolved.effective_backend_class = None;
    resolved.effective_qualification = M5RuntimeAuthorityQualificationClass::Unavailable;
    resolved.stripped_capability_classes = Vec::new();
    resolved.reduced_capability_explanation = explanation;
}

/// Builds the canonical frozen stable M5 execution-surface resolution packet.
///
/// This consumes the frozen runtime-authority matrix directly and resolves every
/// matrix surface across every platform. It is the single in-code source of
/// truth for the checked-in support export at
/// [`M5_EXECUTION_SURFACE_RESOLUTION_ARTIFACT_REF`].
pub fn frozen_stable_m5_execution_surface_resolution_packet() -> M5ExecutionSurfaceResolutionPacket
{
    let matrix = frozen_stable_m5_runtime_authority_matrix_packet();

    let profile_descriptors = ordered_profiles()
        .into_iter()
        .map(M5SandboxProfileDescriptor::for_profile)
        .collect();

    let launch_path_bindings = M5ExecutionLaunchPath::ALL
        .into_iter()
        .map(|launch_path| {
            let surface = launch_path.governing_surface();
            let default_profile = matrix
                .surface_rows
                .iter()
                .find(|row| row.surface == surface)
                .map(|row| row.default_sandbox_profile)
                .unwrap_or(M5SandboxProfile::InertNoExecution);
            M5ExecutionLaunchPathBinding {
                launch_path,
                governing_surface: surface,
                default_backend_class: default_profile.backend_class(),
                note: launch_path_note(launch_path),
            }
        })
        .collect();

    let platform_resolutions = M5ExecutionPlatform::ALL
        .into_iter()
        .map(|platform| M5PlatformResolution {
            platform,
            available_profiles: platform.available_profiles(),
            resolved_rows: matrix
                .surface_rows
                .iter()
                .map(|row| resolve_surface_on_platform(row, platform))
                .collect(),
        })
        .collect();

    M5ExecutionSurfaceResolutionPacket::new(M5ExecutionSurfaceResolutionPacketInput {
        packet_id: M5_EXECUTION_SURFACE_RESOLUTION_PACKET_ID.to_owned(),
        resolution_label: "M5 Execution-Surface Classes, Sandbox-Profile Descriptors, and Unsupported-or-Stricter-Profile Resolution".to_owned(),
        matrix_packet_id: M5_RUNTIME_AUTHORITY_MATRIX_PACKET_ID.to_owned(),
        launch_path_bindings,
        profile_descriptors,
        platform_resolutions,
        trust_review: M5ExecutionSurfaceResolutionTrustReview {
            never_widens_on_narrow: true,
            fail_closed_when_no_stricter_profile: true,
            missing_profile_coverage_narrows: true,
            same_descriptor_across_consumers: true,
            reduced_capability_explained: true,
            no_raw_secret_material_in_exports: true,
        },
        consumer_projection: M5ExecutionSurfaceResolutionConsumerProjection {
            desktop_shows_profile_descriptor: true,
            cli_headless_shows_profile_descriptor: true,
            diagnostics_shows_profile_descriptor: true,
            support_export_shows_resolution: true,
            help_about_shows_backend_class: true,
            release_evidence_consumes_resolution: true,
            command_and_policy_reference_same_resolution: true,
            preview_labs_label_for_narrowed_surfaces: true,
        },
        source_contract_refs: vec![
            M5_EXECUTION_SURFACE_RESOLUTION_SCHEMA_REF.to_owned(),
            M5_EXECUTION_SURFACE_RESOLUTION_DOC_REF.to_owned(),
            M5_RUNTIME_AUTHORITY_MATRIX_SCHEMA_REF.to_owned(),
        ],
        redaction_class_token: "metadata_safe_default".to_owned(),
        minted_at: "2026-06-13T00:00:00Z".to_owned(),
    })
}

/// Reads and validates the checked-in stable resolution export.
pub fn current_stable_m5_execution_surface_resolution_export(
) -> Result<M5ExecutionSurfaceResolutionPacket, M5ExecutionSurfaceResolutionArtifactError> {
    let packet: M5ExecutionSurfaceResolutionPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/execution-auth/m5/implement-execution-surface-classes-sandbox-profile-descriptors-and-unsupported-or-stricter-profile-truth/support_export.json"
    )))
    .map_err(M5ExecutionSurfaceResolutionArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(M5ExecutionSurfaceResolutionArtifactError::Validation(
            violations,
        ))
    }
}

/// Every sandbox profile, in descriptor declaration order.
fn ordered_profiles() -> Vec<M5SandboxProfile> {
    vec![
        M5SandboxProfile::InertNoExecution,
        M5SandboxProfile::InProcessTrustedLocal,
        M5SandboxProfile::BrokeredNetworkOnly,
        M5SandboxProfile::SubprocessIsolatedLocal,
        M5SandboxProfile::ContainerIsolatedLocal,
        M5SandboxProfile::IsolatedRemoteRuntime,
    ]
}

/// Operator-facing note explaining a launch path's governing surface.
fn launch_path_note(launch_path: M5ExecutionLaunchPath) -> String {
    match launch_path {
        M5ExecutionLaunchPath::TaskExecution => {
            "Task runs share the isolated-local-subprocess authority row used for generator hooks."
        }
        M5ExecutionLaunchPath::TerminalSession => {
            "Terminal sessions share the per-session isolated-local-subprocess kernel authority row."
        }
        M5ExecutionLaunchPath::NotebookCell => {
            "Notebook cells run on the per-session isolated-local-subprocess kernel authority row."
        }
        M5ExecutionLaunchPath::RequestSend => {
            "Request/API sends use the brokered network-only send authority row."
        }
        M5ExecutionLaunchPath::DatabaseQuery => {
            "Database queries use the brokered database-action authority row."
        }
        M5ExecutionLaunchPath::DebugSession => {
            "Debug sessions share the per-session isolated-local-subprocess kernel authority row."
        }
        M5ExecutionLaunchPath::ConnectorAction => {
            "Connector actions are brokered network-only sends and use the request/API authority row."
        }
        M5ExecutionLaunchPath::AiToolCall => {
            "AI tool calls use the no-self-issue AI tool authority row."
        }
        M5ExecutionLaunchPath::BrowserRoutedAction => {
            "Browser-routed actions use the isolated-remote-runtime browser authority row."
        }
        M5ExecutionLaunchPath::RemoteMutation => {
            "Remote mutations use the fail-closed isolated-remote-runtime authority row."
        }
    }
    .to_owned()
}

fn validate_source_contracts(
    packet: &M5ExecutionSurfaceResolutionPacket,
    violations: &mut Vec<M5ExecutionSurfaceResolutionViolation>,
) {
    let refs: BTreeSet<&str> = packet
        .source_contract_refs
        .iter()
        .map(String::as_str)
        .collect();
    for required in [
        M5_EXECUTION_SURFACE_RESOLUTION_SCHEMA_REF,
        M5_EXECUTION_SURFACE_RESOLUTION_DOC_REF,
        M5_RUNTIME_AUTHORITY_MATRIX_SCHEMA_REF,
    ] {
        if !refs.contains(required) {
            violations.push(M5ExecutionSurfaceResolutionViolation::MissingSourceContracts);
            return;
        }
    }
    if packet.matrix_packet_id != M5_RUNTIME_AUTHORITY_MATRIX_PACKET_ID {
        violations.push(M5ExecutionSurfaceResolutionViolation::MatrixPacketIdMismatch);
    }
}

fn validate_descriptors(
    packet: &M5ExecutionSurfaceResolutionPacket,
    violations: &mut Vec<M5ExecutionSurfaceResolutionViolation>,
) {
    for profile in ordered_profiles() {
        match packet
            .profile_descriptors
            .iter()
            .find(|descriptor| descriptor.profile == profile)
        {
            None => {
                violations.push(M5ExecutionSurfaceResolutionViolation::ProfileDescriptorMissing)
            }
            Some(descriptor) => {
                let expected = M5SandboxProfileDescriptor::for_profile(profile);
                if descriptor != &expected {
                    violations
                        .push(M5ExecutionSurfaceResolutionViolation::ProfileDescriptorInconsistent);
                }
            }
        }
    }
}

fn validate_launch_path_bindings(
    packet: &M5ExecutionSurfaceResolutionPacket,
    violations: &mut Vec<M5ExecutionSurfaceResolutionViolation>,
) {
    let present: BTreeSet<M5ExecutionLaunchPath> = packet
        .launch_path_bindings
        .iter()
        .map(|binding| binding.launch_path)
        .collect();
    for required in M5ExecutionLaunchPath::ALL {
        if !present.contains(&required) {
            violations.push(M5ExecutionSurfaceResolutionViolation::LaunchPathBindingIncomplete);
            return;
        }
    }
    for binding in &packet.launch_path_bindings {
        if binding.governing_surface != binding.launch_path.governing_surface()
            || binding.note.trim().is_empty()
        {
            violations.push(M5ExecutionSurfaceResolutionViolation::LaunchPathBindingIncomplete);
            return;
        }
    }
}

fn validate_platform_resolutions(
    packet: &M5ExecutionSurfaceResolutionPacket,
    violations: &mut Vec<M5ExecutionSurfaceResolutionViolation>,
) {
    let described: BTreeSet<M5SandboxProfile> = packet
        .profile_descriptors
        .iter()
        .map(|descriptor| descriptor.profile)
        .collect();

    let platforms: BTreeSet<M5ExecutionPlatform> = packet
        .platform_resolutions
        .iter()
        .map(|resolution| resolution.platform)
        .collect();
    for required in M5ExecutionPlatform::ALL {
        if !platforms.contains(&required) {
            violations.push(M5ExecutionSurfaceResolutionViolation::MissingPlatformCoverage);
            return;
        }
    }

    for resolution in &packet.platform_resolutions {
        let surfaces: BTreeSet<M5ExecutingSurface> = resolution
            .resolved_rows
            .iter()
            .map(|row| row.surface)
            .collect();
        for required in M5ExecutingSurface::ALL {
            if !surfaces.contains(&required) {
                violations.push(M5ExecutionSurfaceResolutionViolation::MissingProfileCoverage);
                return;
            }
        }

        for row in &resolution.resolved_rows {
            if !described.contains(&row.default_profile) {
                violations.push(M5ExecutionSurfaceResolutionViolation::ProfileDescriptorMissing);
            }
            if let Some(effective) = row.effective_profile {
                if !described.contains(&effective) {
                    violations
                        .push(M5ExecutionSurfaceResolutionViolation::ProfileDescriptorMissing);
                }
            }
        }

        violations.extend(resolution.validate());
    }
}

fn validate_trust_review(
    packet: &M5ExecutionSurfaceResolutionPacket,
    violations: &mut Vec<M5ExecutionSurfaceResolutionViolation>,
) {
    let review = &packet.trust_review;
    for ok in [
        review.never_widens_on_narrow,
        review.fail_closed_when_no_stricter_profile,
        review.missing_profile_coverage_narrows,
        review.same_descriptor_across_consumers,
        review.reduced_capability_explained,
        review.no_raw_secret_material_in_exports,
    ] {
        if !ok {
            violations.push(M5ExecutionSurfaceResolutionViolation::TrustReviewIncomplete);
            return;
        }
    }
}

fn validate_consumer_projection(
    packet: &M5ExecutionSurfaceResolutionPacket,
    violations: &mut Vec<M5ExecutionSurfaceResolutionViolation>,
) {
    let projection = &packet.consumer_projection;
    for ok in [
        projection.desktop_shows_profile_descriptor,
        projection.cli_headless_shows_profile_descriptor,
        projection.diagnostics_shows_profile_descriptor,
        projection.support_export_shows_resolution,
        projection.help_about_shows_backend_class,
        projection.release_evidence_consumes_resolution,
        projection.command_and_policy_reference_same_resolution,
        projection.preview_labs_label_for_narrowed_surfaces,
    ] {
        if !ok {
            violations.push(M5ExecutionSurfaceResolutionViolation::ConsumerProjectionIncomplete);
            return;
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
                || lower.contains("bearer ")
                || lower.contains("-----begin")
        }
        serde_json::Value::Array(arr) => arr.iter().any(json_contains_forbidden_boundary_material),
        serde_json::Value::Object(map) => {
            map.values().any(json_contains_forbidden_boundary_material)
        }
        _ => false,
    }
}
