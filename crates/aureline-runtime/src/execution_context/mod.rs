//! Execution-context object model and resolver seed.
//!
//! This module owns one canonical [`ExecutionContext`] record and a small
//! [`ExecutionContextResolver`] that mints context records for the terminal,
//! task seed, and debug-prep seed lanes through the same API.
//!
//! ## Why one object, not three
//!
//! Terminal, task, and debug-prep surfaces all need the same shape: who is
//! invoking, which target the work runs against, which toolchain version, the
//! environment capsule, the trust / policy snapshot, and how the resolver
//! settled the inputs (provenance). Forking copies of that shape per surface
//! produces drift the moment one surface upgrades a vocabulary while another
//! lags. The resolver mints exactly one [`ExecutionContext`] shape; downstream
//! surfaces read it through the typed accessors and keep their own state to
//! presentation only.
//!
//! ## Seed scope
//!
//! The initial resolver is intentionally small. It models:
//!
//! - target identity for `local_host` and a small remote/container set,
//! - toolchain identity for the seed lanes (terminal shell, task runner,
//!   debug-prep adapter),
//! - environment capsule reference,
//! - trust state, identity mode, and policy epoch,
//! - workset scope class,
//! - cache disposition,
//! - provenance with per-input precedence decisions,
//! - degraded-field records.
//!
//! Full activator decisions, full target reachability orchestration, full
//! capsule materialisation, and the full ADR-0009 wrapper-provenance chain are
//! out of scope for this seed. The cross-tool boundary schema at
//! [`schemas/runtime/execution_context.schema.json`] keeps the door open to
//! grow this object without forking truth.
//!
//! ## Failure-drill posture
//!
//! When a caller passes conflicting inputs (e.g. an explicit target override
//! that disagrees with the surface-requested target, or an explicit working
//! directory that disagrees with the workspace default), the resolver records
//! the winning [`ResolverInputSource`] in the [`Provenance`] decisions list so
//! every consumer can quote which input survived without re-deriving the
//! precedence locally. The fixture
//! `fixtures/runtime/execution_context_seed_cases/conflicting_inputs.json`
//! exercises the drill end to end.

use serde::{Deserialize, Serialize};

use crate::detectors::node::{NodeToolchainDetection, NodeToolchainResolutionState};
use crate::detectors::python::{PythonEnvironmentDetection, PythonEnvironmentResolutionState};
use crate::discovery::toolchains::WorkspaceToolchainDiscovery;

pub use aureline_workspace::TrustState;

pub mod beta;

/// Schema version of the seed [`ExecutionContext`] record this crate emits.
///
/// Bumped on breaking payload changes; additive-optional fields do not bump
/// this version. The cross-tool boundary schema's
/// `execution_context_schema_version` follows the same versioning rule.
pub const EXECUTION_CONTEXT_SCHEMA_VERSION: u32 = 1;

/// Stable record-kind tag for the seed [`ExecutionContext`] payload.
pub const EXECUTION_CONTEXT_RECORD_KIND: &str = "execution_context_record";

/// Surface class of the invoking lane.
///
/// Frozen vocabulary mirrored from the boundary schema. Surfaces never invent
/// new tokens locally.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SurfaceClass {
    Terminal,
    Task,
    Debug,
    Test,
    NotebookKernel,
    Scaffolding,
    AiToolCall,
    DoctorRepair,
    ImportProbe,
    ReplayProbe,
}

impl SurfaceClass {
    /// Stable string token for this surface class.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Terminal => "terminal",
            Self::Task => "task",
            Self::Debug => "debug",
            Self::Test => "test",
            Self::NotebookKernel => "notebook_kernel",
            Self::Scaffolding => "scaffolding",
            Self::AiToolCall => "ai_tool_call",
            Self::DoctorRepair => "doctor_repair",
            Self::ImportProbe => "import_probe",
            Self::ReplayProbe => "replay_probe",
        }
    }
}

/// Actor class for the invocation subject.
///
/// Frozen vocabulary mirrored from the boundary schema.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ActorClass {
    UserKeystroke,
    UserCommand,
    SessionOverride,
    WorkspaceMigration,
    ExtensionApi,
    AiApply,
    ScheduledTask,
    ImportedProfile,
    AdminPolicyInjector,
}

impl ActorClass {
    /// Stable string token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::UserKeystroke => "user_keystroke",
            Self::UserCommand => "user_command",
            Self::SessionOverride => "session_override",
            Self::WorkspaceMigration => "workspace_migration",
            Self::ExtensionApi => "extension_api",
            Self::AiApply => "ai_apply",
            Self::ScheduledTask => "scheduled_task",
            Self::ImportedProfile => "imported_profile",
            Self::AdminPolicyInjector => "admin_policy_injector",
        }
    }
}

/// Identity mode (re-export of ADR-0001 vocabulary).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum IdentityMode {
    AccountFreeLocal,
    SelfHostedOrg,
    ManagedConvenience,
}

impl IdentityMode {
    /// Stable string token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::AccountFreeLocal => "account_free_local",
            Self::SelfHostedOrg => "self_hosted_org",
            Self::ManagedConvenience => "managed_convenience",
        }
    }
}

/// Target class. Closed seed vocabulary; widening is additive-minor.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TargetClass {
    LocalHost,
    SshRemote,
    ContainerLocal,
    Devcontainer,
    RemoteWorkspaceVm,
    PrebuildRuntime,
    ManagedWorkspace,
    NotebookKernelLocal,
    NotebookKernelRemote,
    AiSandbox,
}

impl TargetClass {
    /// Stable string token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LocalHost => "local_host",
            Self::SshRemote => "ssh_remote",
            Self::ContainerLocal => "container_local",
            Self::Devcontainer => "devcontainer",
            Self::RemoteWorkspaceVm => "remote_workspace_vm",
            Self::PrebuildRuntime => "prebuild_runtime",
            Self::ManagedWorkspace => "managed_workspace",
            Self::NotebookKernelLocal => "notebook_kernel_local",
            Self::NotebookKernelRemote => "notebook_kernel_remote",
            Self::AiSandbox => "ai_sandbox",
        }
    }

    /// True when the target class is not the local desktop and consumer chrome
    /// must render the local-vs-managed boundary cue.
    pub const fn is_remote_or_managed(self) -> bool {
        !matches!(self, Self::LocalHost)
    }
}

/// Target reachability vocabulary.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ReachabilityState {
    Reachable,
    Warming,
    Degraded,
    Unreachable,
    PolicyBlocked,
}

impl ReachabilityState {
    /// Stable string token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Reachable => "reachable",
            Self::Warming => "warming",
            Self::Degraded => "degraded",
            Self::Unreachable => "unreachable",
            Self::PolicyBlocked => "policy_blocked",
        }
    }
}

/// Route class selected for a launchable action.
///
/// This is route truth, not execution-target truth: a tunneled route can still
/// execute on [`TargetClass::SshRemote`] while the route label says the action
/// exposes or traverses a tunnel.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ExecutionRouteClass {
    /// Work stays inside the local process or local host.
    LocalOnly,
    /// Work stays local to the selected target boundary.
    TargetLocal,
    /// Work is attached through an SSH or remote-agent route.
    RemoteAgentAttachRoute,
    /// Work reaches a managed control-plane route.
    ManagedControlPlaneRoute,
    /// Work is handed to the system browser.
    BrowserHandoffRoute,
    /// Work reaches a connected provider route.
    ProviderRoute,
    /// Work enters a publish pipeline.
    PublishPipelineRoute,
    /// Work traverses a declared tunnel exposure route.
    TunnelExposedRoute,
    /// Route truth is unavailable and must be reviewed before mutation.
    UnknownRequiresReview,
}

impl ExecutionRouteClass {
    /// Stable token recorded in execution contexts and downstream cards.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LocalOnly => "local_only",
            Self::TargetLocal => "target_local",
            Self::RemoteAgentAttachRoute => "remote_agent_attach_route",
            Self::ManagedControlPlaneRoute => "managed_control_plane_route",
            Self::BrowserHandoffRoute => "browser_handoff_route",
            Self::ProviderRoute => "provider_route",
            Self::PublishPipelineRoute => "publish_pipeline_route",
            Self::TunnelExposedRoute => "tunnel_exposed_route",
            Self::UnknownRequiresReview => "unknown_requires_review",
        }
    }

    /// Short label safe for cards, chrome, and support export summaries.
    pub const fn label(self) -> &'static str {
        match self {
            Self::LocalOnly => "Local route",
            Self::TargetLocal => "Target-local route",
            Self::RemoteAgentAttachRoute => "Remote attach route",
            Self::ManagedControlPlaneRoute => "Managed route",
            Self::BrowserHandoffRoute => "Browser handoff",
            Self::ProviderRoute => "Provider route",
            Self::PublishPipelineRoute => "Publish pipeline",
            Self::TunnelExposedRoute => "Tunnel route",
            Self::UnknownRequiresReview => "Unknown route",
        }
    }
}

/// Route-origin label attached to an [`ExecutionContext`].
///
/// The record keeps route truth next to target truth without widening
/// [`TargetClass`]. For tunnel routes, `target_identity_ref` and
/// `tunnel_session_ref` preserve the SSH/tunnel transport and target identity
/// needed by chrome, support exports, and review packets.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExecutionRouteOrigin {
    /// Route class selected for the action.
    pub route_class: ExecutionRouteClass,
    /// Stable route-class token.
    pub route_class_token: String,
    /// Human-readable route label.
    pub route_label: String,
    /// Human-readable transport label such as `SSH remote` or `SSH tunnel`.
    pub transport_label: String,
    /// Tunnel session reference when the route traverses a tunnel.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tunnel_session_ref: Option<String>,
    /// Target identity reference preserved for route reconstruction.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target_identity_ref: Option<String>,
}

impl ExecutionRouteOrigin {
    /// Builds a default route-origin label from the resolved target class.
    pub fn for_target(target_class: TargetClass, target_identity_ref: impl Into<String>) -> Self {
        let target_identity_ref = target_identity_ref.into();
        let (route_class, route_label, transport_label) = match target_class {
            TargetClass::LocalHost | TargetClass::NotebookKernelLocal => {
                (ExecutionRouteClass::LocalOnly, "Local route", "local host")
            }
            TargetClass::ContainerLocal | TargetClass::Devcontainer => (
                ExecutionRouteClass::TargetLocal,
                "Target-local route",
                "local container",
            ),
            TargetClass::SshRemote => (
                ExecutionRouteClass::RemoteAgentAttachRoute,
                "Remote attach route",
                "SSH remote",
            ),
            TargetClass::RemoteWorkspaceVm | TargetClass::NotebookKernelRemote => (
                ExecutionRouteClass::RemoteAgentAttachRoute,
                "Remote attach route",
                "remote agent",
            ),
            TargetClass::PrebuildRuntime
            | TargetClass::ManagedWorkspace
            | TargetClass::AiSandbox => (
                ExecutionRouteClass::ManagedControlPlaneRoute,
                "Managed route",
                "managed workspace",
            ),
        };
        Self::new(
            route_class,
            route_label,
            transport_label,
            None,
            Some(target_identity_ref),
        )
    }

    /// Builds a tunnel-exposed route label without changing the execution
    /// target class.
    pub fn tunnel_exposed(
        transport_label: impl Into<String>,
        tunnel_session_ref: impl Into<String>,
        target_identity_ref: impl Into<String>,
    ) -> Self {
        Self::new(
            ExecutionRouteClass::TunnelExposedRoute,
            "Tunnel route",
            transport_label,
            Some(tunnel_session_ref.into()),
            Some(target_identity_ref.into()),
        )
    }

    fn new(
        route_class: ExecutionRouteClass,
        route_label: impl Into<String>,
        transport_label: impl Into<String>,
        tunnel_session_ref: Option<String>,
        target_identity_ref: Option<String>,
    ) -> Self {
        Self {
            route_class,
            route_class_token: route_class.as_str().to_owned(),
            route_label: route_label.into(),
            transport_label: transport_label.into(),
            tunnel_session_ref,
            target_identity_ref,
        }
    }
}

/// Toolchain class. Seed vocabulary covering terminal/task/debug-prep lanes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ToolchainClass {
    Interpreter,
    CompilerToolchain,
    PackageManagerRunner,
    ContainerisedRuntime,
    NotebookKernelRuntime,
    LanguageServerProcess,
    DebugAdapterRuntime,
    TestRunnerRuntime,
    BuildDriverRuntime,
    AiToolRuntime,
    /// Bare login shell — the seed default for terminal sessions before any
    /// task or debug surface narrows the toolchain.
    LoginShell,
}

impl ToolchainClass {
    /// Stable string token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Interpreter => "interpreter",
            Self::CompilerToolchain => "compiler_toolchain",
            Self::PackageManagerRunner => "package_manager_runner",
            Self::ContainerisedRuntime => "containerised_runtime",
            Self::NotebookKernelRuntime => "notebook_kernel_runtime",
            Self::LanguageServerProcess => "language_server_process",
            Self::DebugAdapterRuntime => "debug_adapter_runtime",
            Self::TestRunnerRuntime => "test_runner_runtime",
            Self::BuildDriverRuntime => "build_driver_runtime",
            Self::AiToolRuntime => "ai_tool_runtime",
            Self::LoginShell => "login_shell",
        }
    }
}

/// Activation strategy for a toolchain.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ActivationStrategy {
    AmbientPath,
    EnvManagerShim,
    VenvActivation,
    NixShell,
    NixFlake,
    Direnv,
    DevcontainerBuild,
    OciImageRef,
    ExplicitOverride,
    FallbackResolution,
}

impl ActivationStrategy {
    /// Stable string token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::AmbientPath => "ambient_path",
            Self::EnvManagerShim => "env_manager_shim",
            Self::VenvActivation => "venv_activation",
            Self::NixShell => "nix_shell",
            Self::NixFlake => "nix_flake",
            Self::Direnv => "direnv",
            Self::DevcontainerBuild => "devcontainer_build",
            Self::OciImageRef => "oci_image_ref",
            Self::ExplicitOverride => "explicit_override",
            Self::FallbackResolution => "fallback_resolution",
        }
    }
}

/// Workset scope class. Seed re-export of the boundary-schema vocabulary.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ScopeClass {
    CurrentRoot,
    NamedWorkset,
    SparseSlice,
    FullWorkspace,
    PolicyLimitedView,
    ReviewWorkspace,
    CompanionSurface,
}

impl ScopeClass {
    /// Stable string token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::CurrentRoot => "current_root",
            Self::NamedWorkset => "named_workset",
            Self::SparseSlice => "sparse_slice",
            Self::FullWorkspace => "full_workspace",
            Self::PolicyLimitedView => "policy_limited_view",
            Self::ReviewWorkspace => "review_workspace",
            Self::CompanionSurface => "companion_surface",
        }
    }
}

/// Cache / reuse disposition for the resolved context.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CacheDisposition {
    Cold,
    Warm,
    PrebuildReused,
    CapsuleReused,
    RejectedDrift,
    RejectedPolicy,
    RejectedTrust,
}

impl CacheDisposition {
    /// Stable string token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Cold => "cold",
            Self::Warm => "warm",
            Self::PrebuildReused => "prebuild_reused",
            Self::CapsuleReused => "capsule_reused",
            Self::RejectedDrift => "rejected_drift",
            Self::RejectedPolicy => "rejected_policy",
            Self::RejectedTrust => "rejected_trust",
        }
    }
}

/// Capsule drift state (seed re-export of the ADR-0006 vocabulary).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CapsuleDriftState {
    InSync,
    StaleInputs,
    GeneratorChanged,
    ManuallyDiverged,
    UnknownLineage,
}

impl CapsuleDriftState {
    /// Stable string token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::InSync => "in_sync",
            Self::StaleInputs => "stale_inputs",
            Self::GeneratorChanged => "generator_changed",
            Self::ManuallyDiverged => "manually_diverged",
            Self::UnknownLineage => "unknown_lineage",
        }
    }
}

/// Resolver confidence label for the provenance record.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ConfidenceLevel {
    High,
    Medium,
    Low,
}

impl ConfidenceLevel {
    /// Stable string token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::High => "high",
            Self::Medium => "medium",
            Self::Low => "low",
        }
    }
}

/// Reason a resolved target carries its current confidence label.
///
/// Multiple reasons may be present on one context. Consumers should render
/// the tokens directly instead of inferring target confidence from raw target
/// class or reachability fields.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TargetConfidenceReason {
    ExactLocalTarget,
    SurfaceRequestedTarget,
    ExplicitTargetOverride,
    WorkspaceDefaultTarget,
    ResolverFallbackTarget,
    ConflictingTargetSources,
    RemoteOrManagedBoundary,
    TrustPending,
    TrustRestricted,
    PolicyBlockedReachability,
    CapsuleDrift,
    PrebuildRuntime,
    MixedVersionUnchecked,
}

impl TargetConfidenceReason {
    /// Stable string token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ExactLocalTarget => "exact_local_target",
            Self::SurfaceRequestedTarget => "surface_requested_target",
            Self::ExplicitTargetOverride => "explicit_target_override",
            Self::WorkspaceDefaultTarget => "workspace_default_target",
            Self::ResolverFallbackTarget => "resolver_fallback_target",
            Self::ConflictingTargetSources => "conflicting_target_sources",
            Self::RemoteOrManagedBoundary => "remote_or_managed_boundary",
            Self::TrustPending => "trust_pending",
            Self::TrustRestricted => "trust_restricted",
            Self::PolicyBlockedReachability => "policy_blocked_reachability",
            Self::CapsuleDrift => "capsule_drift",
            Self::PrebuildRuntime => "prebuild_runtime",
            Self::MixedVersionUnchecked => "mixed_version_unchecked",
        }
    }
}

/// Confidence record for the resolved target.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TargetConfidence {
    /// Coarse confidence level every launch surface can render.
    pub level: ConfidenceLevel,
    /// Structured reasons that produced [`Self::level`].
    pub reasons: Vec<TargetConfidenceReason>,
}

/// Prebuild reuse state projected onto every execution-context record.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PrebuildReuseState {
    NotApplicable,
    Candidate,
    Reused,
    RejectedDrift,
    RejectedPolicy,
    RejectedTrust,
}

impl PrebuildReuseState {
    /// Stable string token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NotApplicable => "not_applicable",
            Self::Candidate => "candidate",
            Self::Reused => "reused",
            Self::RejectedDrift => "rejected_drift",
            Self::RejectedPolicy => "rejected_policy",
            Self::RejectedTrust => "rejected_trust",
        }
    }
}

/// Why a prebuild candidate was rejected.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PrebuildInvalidationReason {
    CapsuleDrift,
    PolicyEpochAdvanced,
    TrustStateRestricted,
    TrustStatePending,
    TargetClassChanged,
}

impl PrebuildInvalidationReason {
    /// Stable string token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::CapsuleDrift => "capsule_drift",
            Self::PolicyEpochAdvanced => "policy_epoch_advanced",
            Self::TrustStateRestricted => "trust_state_restricted",
            Self::TrustStatePending => "trust_state_pending",
            Self::TargetClassChanged => "target_class_changed",
        }
    }
}

/// Export-safe prebuild metadata derived during context resolution.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PrebuildMetadata {
    /// Effective prebuild reuse state.
    pub reuse_state: PrebuildReuseState,
    /// Opaque snapshot reference when a prebuild runtime is in play.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub snapshot_ref: Option<String>,
    /// Fingerprint token used to compare prebuild compatibility.
    pub compatibility_fingerprint: String,
    /// Typed rejection reason when [`Self::reuse_state`] is rejected.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub invalidation_reason: Option<PrebuildInvalidationReason>,
}

/// Mixed-version posture between the local client and any helper-backed target.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MixedVersionDriftState {
    NotApplicable,
    Aligned,
    NotNegotiated,
    DriftDetected,
}

impl MixedVersionDriftState {
    /// Stable string token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NotApplicable => "not_applicable",
            Self::Aligned => "aligned",
            Self::NotNegotiated => "not_negotiated",
            Self::DriftDetected => "drift_detected",
        }
    }
}

/// Structured reason for a mixed-version posture.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MixedVersionReason {
    LocalOnly,
    HelperBoundaryNotNegotiated,
    ProtocolsAligned,
    ProtocolSkewDetected,
}

impl MixedVersionReason {
    /// Stable string token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LocalOnly => "local_only",
            Self::HelperBoundaryNotNegotiated => "helper_boundary_not_negotiated",
            Self::ProtocolsAligned => "protocols_aligned",
            Self::ProtocolSkewDetected => "protocol_skew_detected",
        }
    }
}

/// Export-safe mixed-version drift projection.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MixedVersionDrift {
    /// Effective helper/client skew state.
    pub state: MixedVersionDriftState,
    /// Client protocol family recorded by the resolver.
    pub client_protocol: String,
    /// Helper protocol family when a helper advertised one.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub helper_protocol: Option<String>,
    /// Structured reason for the state.
    pub reason: MixedVersionReason,
}

/// Resolver explanation effect class.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ExecutionContextEffectClass {
    SelectedByPrecedence,
    ConflictResolved,
    TargetBoundaryVisible,
    TargetBoundaryLocal,
    PolicyAllowed,
    PolicyNarrowed,
    PolicyBlocked,
    TrustAccepted,
    TrustPending,
    TrustRestricted,
    ScopeSelected,
    PrebuildNotApplicable,
    PrebuildReused,
    PrebuildRejected,
    MixedVersionNotApplicable,
    MixedVersionUnchecked,
    ReusableAcrossSurfaces,
}

impl ExecutionContextEffectClass {
    /// Stable string token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SelectedByPrecedence => "selected_by_precedence",
            Self::ConflictResolved => "conflict_resolved",
            Self::TargetBoundaryVisible => "target_boundary_visible",
            Self::TargetBoundaryLocal => "target_boundary_local",
            Self::PolicyAllowed => "policy_allowed",
            Self::PolicyNarrowed => "policy_narrowed",
            Self::PolicyBlocked => "policy_blocked",
            Self::TrustAccepted => "trust_accepted",
            Self::TrustPending => "trust_pending",
            Self::TrustRestricted => "trust_restricted",
            Self::ScopeSelected => "scope_selected",
            Self::PrebuildNotApplicable => "prebuild_not_applicable",
            Self::PrebuildReused => "prebuild_reused",
            Self::PrebuildRejected => "prebuild_rejected",
            Self::MixedVersionNotApplicable => "mixed_version_not_applicable",
            Self::MixedVersionUnchecked => "mixed_version_unchecked",
            Self::ReusableAcrossSurfaces => "reusable_across_surfaces",
        }
    }
}

/// Resolver reason code for an [`ExecutionContextExplanation`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ExecutionContextReasonCode {
    ExplicitOverrideWon,
    SurfaceRequestWon,
    WorkspaceDefaultWon,
    ResolverFallbackUsed,
    LowerPrecedenceConflict,
    LocalTargetNoBoundary,
    RemoteOrManagedBoundary,
    PolicyEpochCurrent,
    PolicyNarrowedByTrust,
    PolicyBlockedTargetReachability,
    TrustStateTrusted,
    TrustStateRestricted,
    TrustStatePendingEvaluation,
    WorkspaceScopeDefault,
    PrebuildTargetNotSelected,
    PrebuildSnapshotCompatible,
    PrebuildRejectedByCapsuleDrift,
    PrebuildRejectedByTrust,
    LocalOnlyNoHelperVersion,
    HelperBoundaryNotNegotiated,
    SharedContextContract,
}

impl ExecutionContextReasonCode {
    /// Stable string token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ExplicitOverrideWon => "explicit_override_won",
            Self::SurfaceRequestWon => "surface_request_won",
            Self::WorkspaceDefaultWon => "workspace_default_won",
            Self::ResolverFallbackUsed => "resolver_fallback_used",
            Self::LowerPrecedenceConflict => "lower_precedence_conflict",
            Self::LocalTargetNoBoundary => "local_target_no_boundary",
            Self::RemoteOrManagedBoundary => "remote_or_managed_boundary",
            Self::PolicyEpochCurrent => "policy_epoch_current",
            Self::PolicyNarrowedByTrust => "policy_narrowed_by_trust",
            Self::PolicyBlockedTargetReachability => "policy_blocked_target_reachability",
            Self::TrustStateTrusted => "trust_state_trusted",
            Self::TrustStateRestricted => "trust_state_restricted",
            Self::TrustStatePendingEvaluation => "trust_state_pending_evaluation",
            Self::WorkspaceScopeDefault => "workspace_scope_default",
            Self::PrebuildTargetNotSelected => "prebuild_target_not_selected",
            Self::PrebuildSnapshotCompatible => "prebuild_snapshot_compatible",
            Self::PrebuildRejectedByCapsuleDrift => "prebuild_rejected_by_capsule_drift",
            Self::PrebuildRejectedByTrust => "prebuild_rejected_by_trust",
            Self::LocalOnlyNoHelperVersion => "local_only_no_helper_version",
            Self::HelperBoundaryNotNegotiated => "helper_boundary_not_negotiated",
            Self::SharedContextContract => "shared_context_contract",
        }
    }
}

/// Authority class that produced a resolver explanation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ExecutionContextReasonSource {
    Resolver,
    ExplicitOverride,
    SurfaceRequest,
    WorkspaceAuthority,
    PolicyAuthority,
    TrustAuthority,
    EnvironmentCapsule,
    HelperBoundary,
}

impl ExecutionContextReasonSource {
    /// Stable string token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Resolver => "resolver",
            Self::ExplicitOverride => "explicit_override",
            Self::SurfaceRequest => "surface_request",
            Self::WorkspaceAuthority => "workspace_authority",
            Self::PolicyAuthority => "policy_authority",
            Self::TrustAuthority => "trust_authority",
            Self::EnvironmentCapsule => "environment_capsule",
            Self::HelperBoundary => "helper_boundary",
        }
    }
}

/// One structured explanation row attached to the canonical context.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExecutionContextExplanation {
    /// Dotted path into the context field the explanation describes.
    pub field_path: String,
    /// Effect class the resolver applied.
    pub effect: ExecutionContextEffectClass,
    /// Stable reason code for this effect.
    pub reason_code: ExecutionContextReasonCode,
    /// Authority/source that produced the reason.
    pub source: ExecutionContextReasonSource,
    /// Token form of the resolved value or state.
    pub resolved_value_token: String,
    /// Input sources that participated in the explanation, when applicable.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub related_input_sources: Vec<ResolverInputSource>,
    /// Launch-capable surfaces allowed to reuse this explanation.
    pub applicable_surfaces: Vec<SurfaceClass>,
}

/// Frozen degraded-field reason vocabulary used by [`DegradedFieldRecord`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DegradedFieldReason {
    ToolchainFallback,
    ActivatorBlockedByTrust,
    ActivatorBlockedByPolicy,
    ActivatorUnsupportedOnTarget,
    CapsuleUnresolved,
    CapsuleDriftDetected,
    TargetUnreachable,
    PolicyEpochStale,
    TrustStateUnresolved,
    WorksetMemberUnavailable,
    ProvenanceGap,
    ConfidenceLow,
    RemoteAgentScopeMismatch,
}

impl DegradedFieldReason {
    /// Stable string token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ToolchainFallback => "toolchain_fallback",
            Self::ActivatorBlockedByTrust => "activator_blocked_by_trust",
            Self::ActivatorBlockedByPolicy => "activator_blocked_by_policy",
            Self::ActivatorUnsupportedOnTarget => "activator_unsupported_on_target",
            Self::CapsuleUnresolved => "capsule_unresolved",
            Self::CapsuleDriftDetected => "capsule_drift_detected",
            Self::TargetUnreachable => "target_unreachable",
            Self::PolicyEpochStale => "policy_epoch_stale",
            Self::TrustStateUnresolved => "trust_state_unresolved",
            Self::WorksetMemberUnavailable => "workset_member_unavailable",
            Self::ProvenanceGap => "provenance_gap",
            Self::ConfidenceLow => "confidence_low",
            Self::RemoteAgentScopeMismatch => "remote_agent_scope_mismatch",
        }
    }
}

/// One degraded-field record. A non-empty list forces every consumer that
/// renders the context to surface a visible honesty marker.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DegradedFieldRecord {
    pub field_path: String,
    pub reason: DegradedFieldReason,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub repair_hook_ref: Option<String>,
}

/// Typed invocation descriptor.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct InvocationSubject {
    pub command_id: String,
    pub surface: SurfaceClass,
    pub actor_class: ActorClass,
    pub workspace_id: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub profile_id: Option<String>,
}

/// Seed target identity record. Local-host targets carry a canonical id; the
/// remote / managed variants reuse the same shape and let surfaces inspect
/// `target_class` to decide whether the local-vs-managed boundary cue is
/// required.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TargetIdentity {
    pub target_class: TargetClass,
    pub canonical_target_id: String,
    /// Resolved working directory for the launch. Distinct from the workspace
    /// presentation path; surfaces quote this verbatim and never re-derive it
    /// from raw env state.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub working_directory: Option<String>,
    pub reachability_state: ReachabilityState,
    /// True when the target's host is not the local desktop and the chrome
    /// MUST render the local-vs-managed boundary cue.
    pub local_vs_managed_boundary_visible: bool,
}

/// Seed toolchain identity record.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ToolchainIdentity {
    pub toolchain_class: ToolchainClass,
    pub toolchain_id: String,
    pub resolved_version: String,
    pub activation_strategy: ActivationStrategy,
    pub degraded_fallback_flag: bool,
}

/// Seed environment-capsule reference. Capsule bodies live in the cache; this
/// record quotes the reference, the hash, and the drift label.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EnvironmentCapsuleRef {
    pub capsule_id: String,
    pub capsule_hash: String,
    pub resolved_schema_version: String,
    pub drift_state: CapsuleDriftState,
}

/// Trust-and-policy snapshot copied from the workspace authority at resolve
/// time.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PolicyAndTrust {
    pub trust_state: TrustState,
    pub identity_mode: IdentityMode,
    pub policy_epoch: u64,
}

/// Resolver input field that recorded a precedence decision.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ResolverInputField {
    TargetClass,
    WorkingDirectory,
    ToolchainClass,
}

impl ResolverInputField {
    /// Stable string token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::TargetClass => "target_class",
            Self::WorkingDirectory => "working_directory",
            Self::ToolchainClass => "toolchain_class",
        }
    }
}

/// Where a resolver input originated. Listed in highest-precedence-first order
/// so callers can reason about ties without consulting documentation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ResolverInputSource {
    /// Explicit per-call override (highest precedence).
    ExplicitOverride,
    /// Surface-requested value (e.g. terminal pane requested SSH target).
    SurfaceRequested,
    /// Workspace-default (lowest precedence before resolver fallback).
    WorkspaceDefault,
    /// Resolver fallback when no other source supplied a value.
    ResolverFallback,
}

impl ResolverInputSource {
    /// Stable string token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ExplicitOverride => "explicit_override",
            Self::SurfaceRequested => "surface_requested",
            Self::WorkspaceDefault => "workspace_default",
            Self::ResolverFallback => "resolver_fallback",
        }
    }

    /// Lower numbers are higher precedence.
    const fn rank(self) -> u8 {
        match self {
            Self::ExplicitOverride => 0,
            Self::SurfaceRequested => 1,
            Self::WorkspaceDefault => 2,
            Self::ResolverFallback => 3,
        }
    }
}

/// One resolver-input precedence decision. The winning source is recorded
/// even when no conflict occurred; consumers MAY skip rendering the row when
/// `conflicting_sources` is empty.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ResolverInputDecision {
    pub field: ResolverInputField,
    pub winning_source: ResolverInputSource,
    /// Sources that were considered but lost to `winning_source`. Empty when
    /// only one source contributed a value.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub conflicting_sources: Vec<ResolverInputSource>,
    /// Stable string token of the value the resolver settled on. Surfaces
    /// quote this verbatim in inspectors and support exports.
    pub resolved_value_token: String,
}

/// Provenance record colocated with the [`ExecutionContext`]. Carries the
/// resolver version, the recording timestamp, the confidence level, and the
/// per-input decision log.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Provenance {
    pub provenance_record_id: String,
    pub recorded_at: String,
    pub resolver_version: String,
    pub confidence_level: ConfidenceLevel,
    pub input_decisions: Vec<ResolverInputDecision>,
}

/// Canonical seed [`ExecutionContext`] record.
///
/// One inspectable object that the terminal pane, the task seed, and the
/// debug-prep seed all consume. Surfaces never re-derive its fields; they
/// project the record verbatim.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExecutionContext {
    pub record_kind: String,
    pub schema_version: u32,
    pub execution_context_id: String,
    pub invocation_subject: InvocationSubject,
    pub target_identity: TargetIdentity,
    pub target_confidence: TargetConfidence,
    pub toolchain_identity: ToolchainIdentity,
    pub environment_capsule_ref: EnvironmentCapsuleRef,
    pub prebuild_metadata: PrebuildMetadata,
    pub policy_and_trust: PolicyAndTrust,
    pub workset_scope_class: ScopeClass,
    pub cache_disposition: CacheDisposition,
    pub mixed_version_drift: MixedVersionDrift,
    pub provenance: Provenance,
    pub reusable_surfaces: Vec<SurfaceClass>,
    pub explanations: Vec<ExecutionContextExplanation>,
    /// Route-origin label for surfaces that must distinguish remote target
    /// identity from tunnel exposure or browser/provider handoff.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub route_origin: Option<ExecutionRouteOrigin>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub node_toolchain_detection: Option<NodeToolchainDetection>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub python_environment_detection: Option<PythonEnvironmentDetection>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub workspace_toolchain_discovery: Option<WorkspaceToolchainDiscovery>,
    #[serde(default)]
    pub degraded_fields: Vec<DegradedFieldRecord>,
}

impl ExecutionContext {
    /// Returns the canonical execution-context id. Surfaces store this id in
    /// session headers, task channels, and debug-prep records so a support
    /// export can round-trip every artifact to the same context.
    pub fn execution_context_id(&self) -> &str {
        &self.execution_context_id
    }

    /// True when the context has at least one degraded field, signalling that
    /// every rendering surface MUST surface a visible honesty marker.
    pub fn has_degraded_field(&self) -> bool {
        !self.degraded_fields.is_empty()
    }

    /// True when the local-vs-managed boundary cue MUST be rendered for this
    /// context (the target is not the local desktop, or the trust posture has
    /// not yet been resolved).
    pub fn boundary_cue_visible(&self) -> bool {
        self.target_identity.local_vs_managed_boundary_visible
    }

    /// Attaches a Node detector report to this context before a launch
    /// surface dispatches work.
    ///
    /// The report stays embedded in the canonical context so task, test,
    /// debug, inspector, and support surfaces quote the same Node/package
    /// manager truth. Fallback, missing, unsupported, or ambiguous detector
    /// states also add visible degraded-field markers.
    pub fn with_node_toolchain_detection(mut self, detection: NodeToolchainDetection) -> Self {
        self.record_node_detection_degraded_fields(&detection);
        self.node_toolchain_detection = Some(detection);
        self
    }

    /// Attaches a Python environment detector report to this context before
    /// a launch surface dispatches work.
    ///
    /// The report stays embedded in the canonical context so task, test,
    /// debug, notebook, inspector, and support surfaces quote the same
    /// interpreter and manager truth. Fallback, missing, unsupported,
    /// ambiguous, or failed detector states add visible degraded-field
    /// markers.
    pub fn with_python_environment_detection(
        mut self,
        detection: PythonEnvironmentDetection,
    ) -> Self {
        self.record_python_detection_degraded_fields(&detection);
        self.python_environment_detection = Some(detection);
        self
    }

    /// Attaches an opinion-free workspace toolchain discovery report.
    ///
    /// The report records presence/version evidence only and does not add
    /// degraded-field markers. Launch-specific consumers can still attach the
    /// stricter Node or Python reports separately when readiness decisions are
    /// needed.
    pub fn with_workspace_toolchain_discovery(
        mut self,
        discovery: WorkspaceToolchainDiscovery,
    ) -> Self {
        self.workspace_toolchain_discovery = Some(discovery);
        self
    }

    /// Attaches explicit route-origin truth to this context.
    ///
    /// Callers use this when the execution target remains unchanged but the
    /// route itself is materially different, such as a tunneled route.
    pub fn with_route_origin(mut self, route_origin: ExecutionRouteOrigin) -> Self {
        self.route_origin = Some(route_origin);
        self
    }

    fn record_node_detection_degraded_fields(&mut self, detection: &NodeToolchainDetection) {
        add_node_detection_degraded_field(
            &mut self.degraded_fields,
            "node_toolchain_detection.node_runtime",
            detection.node_runtime.resolution_state,
        );
        add_node_detection_degraded_field(
            &mut self.degraded_fields,
            "node_toolchain_detection.package_manager",
            detection.package_manager.resolution_state,
        );
    }

    fn record_python_detection_degraded_fields(&mut self, detection: &PythonEnvironmentDetection) {
        add_python_detection_degraded_field(
            &mut self.degraded_fields,
            "python_environment_detection.interpreter",
            detection.interpreter.resolution_state,
        );
        add_python_detection_degraded_field(
            &mut self.degraded_fields,
            "python_environment_detection.environment_manager",
            detection.environment_manager.resolution_state,
        );
        if detection.has_detector_failure()
            && !self.degraded_fields.iter().any(|field| {
                field.field_path == "python_environment_detection.provenance_cards"
                    && field.reason == DegradedFieldReason::ProvenanceGap
            })
        {
            self.degraded_fields.push(DegradedFieldRecord {
                field_path: "python_environment_detection.provenance_cards".to_owned(),
                reason: DegradedFieldReason::ProvenanceGap,
                repair_hook_ref: Some("doctor.repair.python_environment".to_owned()),
            });
        }
    }
}

/// Inputs to [`ExecutionContextResolver::resolve`].
///
/// A request bundles the surface-requested values and any explicit per-call
/// overrides. The resolver applies a deterministic precedence
/// (explicit override > surface requested > workspace default > resolver
/// fallback) and records the winning source in [`Provenance::input_decisions`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExecutionContextRequest<'a> {
    pub command_id: &'a str,
    pub surface: SurfaceClass,
    pub actor_class: ActorClass,
    pub trust_state: TrustState,
    pub observed_at: &'a str,

    /// Surface-requested target class. The terminal pane requesting an SSH
    /// session would set this to [`TargetClass::SshRemote`].
    pub requested_target_class: Option<TargetClass>,
    /// Surface-requested working directory.
    pub requested_working_directory: Option<&'a str>,
    /// Surface-requested toolchain class.
    pub requested_toolchain_class: Option<ToolchainClass>,

    /// Explicit per-call target-class override (highest precedence).
    pub override_target_class: Option<TargetClass>,
    /// Explicit per-call working-directory override.
    pub override_working_directory: Option<&'a str>,
    /// Explicit per-call toolchain-class override.
    pub override_toolchain_class: Option<ToolchainClass>,
}

impl<'a> ExecutionContextRequest<'a> {
    /// Convenience constructor for a terminal-pane request that asks for a
    /// local login shell against the workspace default working directory.
    pub fn local_terminal_seed(
        command_id: &'a str,
        trust_state: TrustState,
        observed_at: &'a str,
    ) -> Self {
        Self {
            command_id,
            surface: SurfaceClass::Terminal,
            actor_class: ActorClass::UserCommand,
            trust_state,
            observed_at,
            requested_target_class: Some(TargetClass::LocalHost),
            requested_working_directory: None,
            requested_toolchain_class: Some(ToolchainClass::LoginShell),
            override_target_class: None,
            override_working_directory: None,
            override_toolchain_class: None,
        }
    }

    /// Convenience constructor for a task-seed request that asks for a build
    /// driver runtime against the workspace default working directory.
    pub fn task_seed(command_id: &'a str, trust_state: TrustState, observed_at: &'a str) -> Self {
        Self {
            command_id,
            surface: SurfaceClass::Task,
            actor_class: ActorClass::UserCommand,
            trust_state,
            observed_at,
            requested_target_class: Some(TargetClass::LocalHost),
            requested_working_directory: None,
            requested_toolchain_class: Some(ToolchainClass::BuildDriverRuntime),
            override_target_class: None,
            override_working_directory: None,
            override_toolchain_class: None,
        }
    }

    /// Convenience constructor for a package-script task request that asks for
    /// a package-manager runner against the workspace default working
    /// directory.
    pub fn package_script_task_seed(
        command_id: &'a str,
        trust_state: TrustState,
        observed_at: &'a str,
    ) -> Self {
        Self {
            command_id,
            surface: SurfaceClass::Task,
            actor_class: ActorClass::UserCommand,
            trust_state,
            observed_at,
            requested_target_class: Some(TargetClass::LocalHost),
            requested_working_directory: None,
            requested_toolchain_class: Some(ToolchainClass::PackageManagerRunner),
            override_target_class: None,
            override_working_directory: None,
            override_toolchain_class: None,
        }
    }

    /// Convenience constructor for a test-run seed request that asks for a
    /// test-runner runtime against the workspace default working directory.
    pub fn test_seed(command_id: &'a str, trust_state: TrustState, observed_at: &'a str) -> Self {
        Self {
            command_id,
            surface: SurfaceClass::Test,
            actor_class: ActorClass::UserCommand,
            trust_state,
            observed_at,
            requested_target_class: Some(TargetClass::LocalHost),
            requested_working_directory: None,
            requested_toolchain_class: Some(ToolchainClass::TestRunnerRuntime),
            override_target_class: None,
            override_working_directory: None,
            override_toolchain_class: None,
        }
    }

    /// Convenience constructor for a debug-prep seed request that asks for a
    /// debug-adapter runtime against the workspace default working directory.
    pub fn debug_prep_seed(
        command_id: &'a str,
        trust_state: TrustState,
        observed_at: &'a str,
    ) -> Self {
        Self {
            command_id,
            surface: SurfaceClass::Debug,
            actor_class: ActorClass::UserCommand,
            trust_state,
            observed_at,
            requested_target_class: Some(TargetClass::LocalHost),
            requested_working_directory: None,
            requested_toolchain_class: Some(ToolchainClass::DebugAdapterRuntime),
            override_target_class: None,
            override_working_directory: None,
            override_toolchain_class: None,
        }
    }

    /// Convenience constructor for an AI tool-call seed request that asks for
    /// an AI-tool runtime against the workspace default working directory.
    pub fn ai_tool_call_seed(
        command_id: &'a str,
        trust_state: TrustState,
        observed_at: &'a str,
    ) -> Self {
        Self {
            command_id,
            surface: SurfaceClass::AiToolCall,
            actor_class: ActorClass::AiApply,
            trust_state,
            observed_at,
            requested_target_class: Some(TargetClass::AiSandbox),
            requested_working_directory: None,
            requested_toolchain_class: Some(ToolchainClass::AiToolRuntime),
            override_target_class: None,
            override_working_directory: None,
            override_toolchain_class: None,
        }
    }
}

/// Resolver configuration captured at construction time. These values come
/// from the workspace authority (lifecycle machine, settings registry, trust
/// policy) and the resolver does not invent them locally.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExecutionContextResolverConfig {
    pub workspace_id: String,
    pub profile_id: Option<String>,
    pub identity_mode: IdentityMode,
    pub policy_epoch: u64,
    pub workspace_default_target_class: TargetClass,
    pub workspace_default_working_directory: Option<String>,
    pub workspace_default_scope_class: ScopeClass,
    pub local_host_canonical_id: String,
    pub environment_capsule_ref: EnvironmentCapsuleRef,
    pub resolver_version: String,
}

/// Resolver seed that mints [`ExecutionContext`] records for the terminal,
/// task, and debug-prep lanes.
///
/// The resolver is deterministic: same inputs and same monotonic timestamps
/// produce the same record. The resolver does not spawn processes; it builds
/// the canonical object so downstream surfaces have one truth to read.
#[derive(Debug, Clone)]
pub struct ExecutionContextResolver {
    config: ExecutionContextResolverConfig,
    next_sequence: u64,
}

impl ExecutionContextResolver {
    /// Construct a resolver from a frozen configuration snapshot.
    pub fn new(config: ExecutionContextResolverConfig) -> Self {
        Self {
            config,
            next_sequence: 0,
        }
    }

    /// Returns the workspace id this resolver is wired to.
    pub fn workspace_id(&self) -> &str {
        &self.config.workspace_id
    }

    /// Returns the resolver-version token recorded on every provenance row.
    pub fn resolver_version(&self) -> &str {
        &self.config.resolver_version
    }

    /// Resolve one [`ExecutionContextRequest`] into a canonical
    /// [`ExecutionContext`].
    ///
    /// The resolver applies the precedence
    /// `explicit_override > surface_requested > workspace_default > resolver_fallback`
    /// and records the winning source in [`Provenance::input_decisions`].
    /// Conflicting sources are listed in `conflicting_sources` so consumers
    /// can render a "why this target?" inspector without re-deriving the
    /// precedence locally.
    pub fn resolve(&mut self, request: ExecutionContextRequest<'_>) -> ExecutionContext {
        let sequence = self.next_sequence;
        self.next_sequence = self.next_sequence.saturating_add(1);

        let (target_class, target_decision) = self.resolve_target_class(&request);
        let (working_directory, wd_decision) = self.resolve_working_directory(&request);
        let (toolchain_class, toolchain_decision) = self.resolve_toolchain_class(&request);
        let input_decisions = vec![target_decision, wd_decision, toolchain_decision];

        let target_identity = TargetIdentity {
            target_class,
            canonical_target_id: self.canonical_target_id_for(target_class),
            working_directory: working_directory.clone(),
            reachability_state: ReachabilityState::Reachable,
            local_vs_managed_boundary_visible: target_class.is_remote_or_managed(),
        };

        let toolchain_identity = ToolchainIdentity {
            toolchain_class,
            toolchain_id: default_toolchain_id_for(toolchain_class).to_owned(),
            resolved_version: "seed".to_owned(),
            activation_strategy: default_activation_strategy_for(toolchain_class),
            degraded_fallback_flag: false,
        };

        let policy_and_trust = PolicyAndTrust {
            trust_state: request.trust_state,
            identity_mode: self.config.identity_mode,
            policy_epoch: self.config.policy_epoch,
        };

        let prebuild_metadata = prebuild_metadata_for(
            &self.config.environment_capsule_ref,
            target_class,
            request.trust_state,
            self.config.policy_epoch,
        );
        let cache_disposition = cache_disposition_for_prebuild(&prebuild_metadata);
        let mixed_version_drift = mixed_version_drift_for(target_class);
        let reusable_surfaces = reusable_launch_surfaces();

        let invocation_subject = InvocationSubject {
            command_id: request.command_id.to_owned(),
            surface: request.surface,
            actor_class: request.actor_class,
            workspace_id: self.config.workspace_id.clone(),
            profile_id: self.config.profile_id.clone(),
        };

        let execution_context_id = format!(
            "exec:{ws}:{surface}:{seq}",
            ws = self.config.workspace_id,
            surface = request.surface.as_str(),
            seq = sequence,
        );

        let provenance_record_id = format!("prov:{execution_context_id}");

        let confidence_level = if request.trust_state == TrustState::Trusted {
            ConfidenceLevel::High
        } else {
            ConfidenceLevel::Medium
        };

        let mut degraded_fields = Vec::new();
        if request.trust_state == TrustState::PendingEvaluation {
            degraded_fields.push(DegradedFieldRecord {
                field_path: "policy_and_trust.trust_state".to_owned(),
                reason: DegradedFieldReason::TrustStateUnresolved,
                repair_hook_ref: None,
            });
        }
        if self.config.environment_capsule_ref.drift_state != CapsuleDriftState::InSync {
            degraded_fields.push(DegradedFieldRecord {
                field_path: "environment_capsule_ref.drift_state".to_owned(),
                reason: DegradedFieldReason::CapsuleDriftDetected,
                repair_hook_ref: Some("doctor.repair.environment_capsule".to_owned()),
            });
        }

        let target_confidence = target_confidence_for(
            target_class,
            &input_decisions,
            request.trust_state,
            target_identity.reachability_state,
            self.config.environment_capsule_ref.drift_state,
            mixed_version_drift.state,
        );

        let provenance = Provenance {
            provenance_record_id,
            recorded_at: request.observed_at.to_owned(),
            resolver_version: self.config.resolver_version.clone(),
            confidence_level,
            input_decisions,
        };
        let explanations = build_explanations(
            &provenance.input_decisions,
            &target_identity,
            &policy_and_trust,
            self.config.workspace_default_scope_class,
            &prebuild_metadata,
            &mixed_version_drift,
            &reusable_surfaces,
        );

        ExecutionContext {
            record_kind: EXECUTION_CONTEXT_RECORD_KIND.to_owned(),
            schema_version: EXECUTION_CONTEXT_SCHEMA_VERSION,
            execution_context_id,
            invocation_subject,
            target_identity,
            target_confidence,
            toolchain_identity,
            environment_capsule_ref: self.config.environment_capsule_ref.clone(),
            prebuild_metadata,
            policy_and_trust,
            workset_scope_class: self.config.workspace_default_scope_class,
            cache_disposition,
            mixed_version_drift,
            provenance,
            reusable_surfaces,
            explanations,
            route_origin: None,
            node_toolchain_detection: None,
            python_environment_detection: None,
            workspace_toolchain_discovery: None,
            degraded_fields,
        }
    }

    fn resolve_target_class(
        &self,
        request: &ExecutionContextRequest<'_>,
    ) -> (TargetClass, ResolverInputDecision) {
        let candidates: [(Option<TargetClass>, ResolverInputSource); 3] = [
            (
                request.override_target_class,
                ResolverInputSource::ExplicitOverride,
            ),
            (
                request.requested_target_class,
                ResolverInputSource::SurfaceRequested,
            ),
            (
                Some(self.config.workspace_default_target_class),
                ResolverInputSource::WorkspaceDefault,
            ),
        ];
        let (winner, decision) = pick_with_decision(
            ResolverInputField::TargetClass,
            candidates.iter().copied(),
            self.config.workspace_default_target_class,
            ResolverInputSource::ResolverFallback,
            |class| class.as_str().to_owned(),
        );
        (winner, decision)
    }

    fn resolve_working_directory(
        &self,
        request: &ExecutionContextRequest<'_>,
    ) -> (Option<String>, ResolverInputDecision) {
        let candidates: [(Option<&str>, ResolverInputSource); 3] = [
            (
                request.override_working_directory,
                ResolverInputSource::ExplicitOverride,
            ),
            (
                request.requested_working_directory,
                ResolverInputSource::SurfaceRequested,
            ),
            (
                self.config.workspace_default_working_directory.as_deref(),
                ResolverInputSource::WorkspaceDefault,
            ),
        ];

        let mut contributing: Vec<(String, ResolverInputSource)> = candidates
            .into_iter()
            .filter_map(|(value, source)| value.map(|v| (v.to_owned(), source)))
            .collect();

        if contributing.is_empty() {
            return (
                None,
                ResolverInputDecision {
                    field: ResolverInputField::WorkingDirectory,
                    winning_source: ResolverInputSource::ResolverFallback,
                    conflicting_sources: Vec::new(),
                    resolved_value_token: String::new(),
                },
            );
        }

        contributing.sort_by_key(|(_, source)| source.rank());
        let (winning_value, winning_source) = contributing[0].clone();

        let mut conflicting = Vec::new();
        for (value, source) in contributing.iter().skip(1) {
            if *value != winning_value {
                conflicting.push(*source);
            }
        }

        (
            Some(winning_value.clone()),
            ResolverInputDecision {
                field: ResolverInputField::WorkingDirectory,
                winning_source,
                conflicting_sources: conflicting,
                resolved_value_token: winning_value,
            },
        )
    }

    fn resolve_toolchain_class(
        &self,
        request: &ExecutionContextRequest<'_>,
    ) -> (ToolchainClass, ResolverInputDecision) {
        let surface_default = surface_default_toolchain(request.surface);
        let candidates: [(Option<ToolchainClass>, ResolverInputSource); 3] = [
            (
                request.override_toolchain_class,
                ResolverInputSource::ExplicitOverride,
            ),
            (
                request.requested_toolchain_class,
                ResolverInputSource::SurfaceRequested,
            ),
            (Some(surface_default), ResolverInputSource::WorkspaceDefault),
        ];
        let (winner, decision) = pick_with_decision(
            ResolverInputField::ToolchainClass,
            candidates.iter().copied(),
            surface_default,
            ResolverInputSource::ResolverFallback,
            |class| class.as_str().to_owned(),
        );
        (winner, decision)
    }

    fn canonical_target_id_for(&self, target_class: TargetClass) -> String {
        match target_class {
            TargetClass::LocalHost => self.config.local_host_canonical_id.clone(),
            other => format!("seed:{}", other.as_str()),
        }
    }
}

fn reusable_launch_surfaces() -> Vec<SurfaceClass> {
    vec![
        SurfaceClass::Terminal,
        SurfaceClass::Task,
        SurfaceClass::Test,
        SurfaceClass::Debug,
        SurfaceClass::AiToolCall,
    ]
}

fn prebuild_metadata_for(
    capsule: &EnvironmentCapsuleRef,
    target_class: TargetClass,
    trust_state: TrustState,
    policy_epoch: u64,
) -> PrebuildMetadata {
    if target_class != TargetClass::PrebuildRuntime {
        return PrebuildMetadata {
            reuse_state: PrebuildReuseState::NotApplicable,
            snapshot_ref: None,
            compatibility_fingerprint: compatibility_fingerprint_token(
                capsule,
                target_class,
                policy_epoch,
            ),
            invalidation_reason: None,
        };
    }

    let snapshot_ref = Some(format!("prebuild:{}:snapshot", capsule.capsule_id));
    let (reuse_state, invalidation_reason) = match (trust_state, capsule.drift_state) {
        (TrustState::PendingEvaluation, _) => (
            PrebuildReuseState::RejectedTrust,
            Some(PrebuildInvalidationReason::TrustStatePending),
        ),
        (TrustState::Restricted, _) => (
            PrebuildReuseState::RejectedTrust,
            Some(PrebuildInvalidationReason::TrustStateRestricted),
        ),
        (_, drift_state) if drift_state != CapsuleDriftState::InSync => (
            PrebuildReuseState::RejectedDrift,
            Some(PrebuildInvalidationReason::CapsuleDrift),
        ),
        _ => (PrebuildReuseState::Reused, None),
    };

    PrebuildMetadata {
        reuse_state,
        snapshot_ref,
        compatibility_fingerprint: compatibility_fingerprint_token(
            capsule,
            target_class,
            policy_epoch,
        ),
        invalidation_reason,
    }
}

fn compatibility_fingerprint_token(
    capsule: &EnvironmentCapsuleRef,
    target_class: TargetClass,
    policy_epoch: u64,
) -> String {
    format!(
        "fp:{}:{}:policy{}",
        capsule.capsule_hash,
        target_class.as_str(),
        policy_epoch
    )
}

fn cache_disposition_for_prebuild(metadata: &PrebuildMetadata) -> CacheDisposition {
    match metadata.reuse_state {
        PrebuildReuseState::NotApplicable | PrebuildReuseState::Candidate => CacheDisposition::Cold,
        PrebuildReuseState::Reused => CacheDisposition::PrebuildReused,
        PrebuildReuseState::RejectedDrift => CacheDisposition::RejectedDrift,
        PrebuildReuseState::RejectedPolicy => CacheDisposition::RejectedPolicy,
        PrebuildReuseState::RejectedTrust => CacheDisposition::RejectedTrust,
    }
}

fn mixed_version_drift_for(target_class: TargetClass) -> MixedVersionDrift {
    if target_class.is_remote_or_managed() {
        MixedVersionDrift {
            state: MixedVersionDriftState::NotNegotiated,
            client_protocol: "execution-context-alpha.v1".to_owned(),
            helper_protocol: None,
            reason: MixedVersionReason::HelperBoundaryNotNegotiated,
        }
    } else {
        MixedVersionDrift {
            state: MixedVersionDriftState::NotApplicable,
            client_protocol: "execution-context-alpha.v1".to_owned(),
            helper_protocol: None,
            reason: MixedVersionReason::LocalOnly,
        }
    }
}

fn target_confidence_for(
    target_class: TargetClass,
    input_decisions: &[ResolverInputDecision],
    trust_state: TrustState,
    reachability_state: ReachabilityState,
    capsule_drift_state: CapsuleDriftState,
    mixed_version_state: MixedVersionDriftState,
) -> TargetConfidence {
    let mut reasons = Vec::new();
    let target_decision = input_decisions
        .iter()
        .find(|decision| decision.field == ResolverInputField::TargetClass);

    if target_class == TargetClass::LocalHost {
        reasons.push(TargetConfidenceReason::ExactLocalTarget);
    } else {
        reasons.push(TargetConfidenceReason::RemoteOrManagedBoundary);
    }

    if target_class == TargetClass::PrebuildRuntime {
        reasons.push(TargetConfidenceReason::PrebuildRuntime);
    }

    if let Some(decision) = target_decision {
        match decision.winning_source {
            ResolverInputSource::ExplicitOverride => {
                reasons.push(TargetConfidenceReason::ExplicitTargetOverride)
            }
            ResolverInputSource::SurfaceRequested => {
                reasons.push(TargetConfidenceReason::SurfaceRequestedTarget)
            }
            ResolverInputSource::WorkspaceDefault => {
                reasons.push(TargetConfidenceReason::WorkspaceDefaultTarget)
            }
            ResolverInputSource::ResolverFallback => {
                reasons.push(TargetConfidenceReason::ResolverFallbackTarget)
            }
        }
        if !decision.conflicting_sources.is_empty() {
            reasons.push(TargetConfidenceReason::ConflictingTargetSources);
        }
    }

    match trust_state {
        TrustState::Trusted => {}
        TrustState::Restricted => reasons.push(TargetConfidenceReason::TrustRestricted),
        TrustState::PendingEvaluation => reasons.push(TargetConfidenceReason::TrustPending),
    }

    if reachability_state == ReachabilityState::PolicyBlocked {
        reasons.push(TargetConfidenceReason::PolicyBlockedReachability);
    }
    if capsule_drift_state != CapsuleDriftState::InSync {
        reasons.push(TargetConfidenceReason::CapsuleDrift);
    }
    if mixed_version_state == MixedVersionDriftState::NotNegotiated {
        reasons.push(TargetConfidenceReason::MixedVersionUnchecked);
    }

    let level = if reasons.iter().any(|reason| {
        matches!(
            reason,
            TargetConfidenceReason::TrustPending
                | TargetConfidenceReason::PolicyBlockedReachability
                | TargetConfidenceReason::CapsuleDrift
                | TargetConfidenceReason::ResolverFallbackTarget
        )
    }) {
        ConfidenceLevel::Low
    } else if reasons.iter().any(|reason| {
        matches!(
            reason,
            TargetConfidenceReason::RemoteOrManagedBoundary
                | TargetConfidenceReason::ConflictingTargetSources
                | TargetConfidenceReason::TrustRestricted
                | TargetConfidenceReason::MixedVersionUnchecked
        )
    }) {
        ConfidenceLevel::Medium
    } else {
        ConfidenceLevel::High
    };

    TargetConfidence { level, reasons }
}

fn build_explanations(
    input_decisions: &[ResolverInputDecision],
    target: &TargetIdentity,
    policy: &PolicyAndTrust,
    scope_class: ScopeClass,
    prebuild: &PrebuildMetadata,
    mixed_version: &MixedVersionDrift,
    reusable_surfaces: &[SurfaceClass],
) -> Vec<ExecutionContextExplanation> {
    let mut explanations = Vec::new();
    for decision in input_decisions {
        explanations.push(precedence_explanation(
            decision,
            ExecutionContextEffectClass::SelectedByPrecedence,
            reason_for_winning_source(decision.winning_source),
            source_for_winning_source(decision.winning_source),
            reusable_surfaces,
        ));
        if !decision.conflicting_sources.is_empty() {
            explanations.push(precedence_explanation(
                decision,
                ExecutionContextEffectClass::ConflictResolved,
                ExecutionContextReasonCode::LowerPrecedenceConflict,
                ExecutionContextReasonSource::Resolver,
                reusable_surfaces,
            ));
        }
    }

    explanations.push(ExecutionContextExplanation {
        field_path: "target_identity.local_vs_managed_boundary_visible".to_owned(),
        effect: if target.local_vs_managed_boundary_visible {
            ExecutionContextEffectClass::TargetBoundaryVisible
        } else {
            ExecutionContextEffectClass::TargetBoundaryLocal
        },
        reason_code: if target.local_vs_managed_boundary_visible {
            ExecutionContextReasonCode::RemoteOrManagedBoundary
        } else {
            ExecutionContextReasonCode::LocalTargetNoBoundary
        },
        source: ExecutionContextReasonSource::Resolver,
        resolved_value_token: target.local_vs_managed_boundary_visible.to_string(),
        related_input_sources: Vec::new(),
        applicable_surfaces: reusable_surfaces.to_vec(),
    });

    explanations.push(trust_explanation(policy, reusable_surfaces));
    explanations.push(policy_explanation(target, policy, reusable_surfaces));

    explanations.push(ExecutionContextExplanation {
        field_path: "workset_scope_class".to_owned(),
        effect: ExecutionContextEffectClass::ScopeSelected,
        reason_code: ExecutionContextReasonCode::WorkspaceScopeDefault,
        source: ExecutionContextReasonSource::WorkspaceAuthority,
        resolved_value_token: scope_class.as_str().to_owned(),
        related_input_sources: Vec::new(),
        applicable_surfaces: reusable_surfaces.to_vec(),
    });

    explanations.push(prebuild_explanation(prebuild, reusable_surfaces));
    explanations.push(mixed_version_explanation(mixed_version, reusable_surfaces));
    explanations.push(ExecutionContextExplanation {
        field_path: "reusable_surfaces".to_owned(),
        effect: ExecutionContextEffectClass::ReusableAcrossSurfaces,
        reason_code: ExecutionContextReasonCode::SharedContextContract,
        source: ExecutionContextReasonSource::Resolver,
        resolved_value_token: reusable_surfaces
            .iter()
            .map(|surface| surface.as_str())
            .collect::<Vec<_>>()
            .join("|"),
        related_input_sources: Vec::new(),
        applicable_surfaces: reusable_surfaces.to_vec(),
    });
    explanations
}

fn precedence_explanation(
    decision: &ResolverInputDecision,
    effect: ExecutionContextEffectClass,
    reason_code: ExecutionContextReasonCode,
    source: ExecutionContextReasonSource,
    reusable_surfaces: &[SurfaceClass],
) -> ExecutionContextExplanation {
    let mut related_input_sources = vec![decision.winning_source];
    related_input_sources.extend(decision.conflicting_sources.iter().copied());
    ExecutionContextExplanation {
        field_path: format!("resolver_input.{}", decision.field.as_str()),
        effect,
        reason_code,
        source,
        resolved_value_token: decision.resolved_value_token.clone(),
        related_input_sources,
        applicable_surfaces: reusable_surfaces.to_vec(),
    }
}

const fn reason_for_winning_source(source: ResolverInputSource) -> ExecutionContextReasonCode {
    match source {
        ResolverInputSource::ExplicitOverride => ExecutionContextReasonCode::ExplicitOverrideWon,
        ResolverInputSource::SurfaceRequested => ExecutionContextReasonCode::SurfaceRequestWon,
        ResolverInputSource::WorkspaceDefault => ExecutionContextReasonCode::WorkspaceDefaultWon,
        ResolverInputSource::ResolverFallback => ExecutionContextReasonCode::ResolverFallbackUsed,
    }
}

const fn source_for_winning_source(source: ResolverInputSource) -> ExecutionContextReasonSource {
    match source {
        ResolverInputSource::ExplicitOverride => ExecutionContextReasonSource::ExplicitOverride,
        ResolverInputSource::SurfaceRequested => ExecutionContextReasonSource::SurfaceRequest,
        ResolverInputSource::WorkspaceDefault => ExecutionContextReasonSource::WorkspaceAuthority,
        ResolverInputSource::ResolverFallback => ExecutionContextReasonSource::Resolver,
    }
}

fn trust_explanation(
    policy: &PolicyAndTrust,
    reusable_surfaces: &[SurfaceClass],
) -> ExecutionContextExplanation {
    let (effect, reason_code) = match policy.trust_state {
        TrustState::Trusted => (
            ExecutionContextEffectClass::TrustAccepted,
            ExecutionContextReasonCode::TrustStateTrusted,
        ),
        TrustState::Restricted => (
            ExecutionContextEffectClass::TrustRestricted,
            ExecutionContextReasonCode::TrustStateRestricted,
        ),
        TrustState::PendingEvaluation => (
            ExecutionContextEffectClass::TrustPending,
            ExecutionContextReasonCode::TrustStatePendingEvaluation,
        ),
    };
    ExecutionContextExplanation {
        field_path: "policy_and_trust.trust_state".to_owned(),
        effect,
        reason_code,
        source: ExecutionContextReasonSource::TrustAuthority,
        resolved_value_token: trust_state_token(policy.trust_state).to_owned(),
        related_input_sources: Vec::new(),
        applicable_surfaces: reusable_surfaces.to_vec(),
    }
}

fn policy_explanation(
    target: &TargetIdentity,
    policy: &PolicyAndTrust,
    reusable_surfaces: &[SurfaceClass],
) -> ExecutionContextExplanation {
    let (effect, reason_code, value_token) =
        if target.reachability_state == ReachabilityState::PolicyBlocked {
            (
                ExecutionContextEffectClass::PolicyBlocked,
                ExecutionContextReasonCode::PolicyBlockedTargetReachability,
                "policy_blocked",
            )
        } else if policy.trust_state != TrustState::Trusted {
            (
                ExecutionContextEffectClass::PolicyNarrowed,
                ExecutionContextReasonCode::PolicyNarrowedByTrust,
                trust_state_token(policy.trust_state),
            )
        } else {
            (
                ExecutionContextEffectClass::PolicyAllowed,
                ExecutionContextReasonCode::PolicyEpochCurrent,
                "allowed",
            )
        };
    ExecutionContextExplanation {
        field_path: "policy_and_trust.policy_epoch".to_owned(),
        effect,
        reason_code,
        source: ExecutionContextReasonSource::PolicyAuthority,
        resolved_value_token: format!("{value_token}:{}", policy.policy_epoch),
        related_input_sources: Vec::new(),
        applicable_surfaces: reusable_surfaces.to_vec(),
    }
}

fn prebuild_explanation(
    prebuild: &PrebuildMetadata,
    reusable_surfaces: &[SurfaceClass],
) -> ExecutionContextExplanation {
    let (effect, reason_code) = match prebuild.reuse_state {
        PrebuildReuseState::NotApplicable | PrebuildReuseState::Candidate => (
            ExecutionContextEffectClass::PrebuildNotApplicable,
            ExecutionContextReasonCode::PrebuildTargetNotSelected,
        ),
        PrebuildReuseState::Reused => (
            ExecutionContextEffectClass::PrebuildReused,
            ExecutionContextReasonCode::PrebuildSnapshotCompatible,
        ),
        PrebuildReuseState::RejectedDrift => (
            ExecutionContextEffectClass::PrebuildRejected,
            ExecutionContextReasonCode::PrebuildRejectedByCapsuleDrift,
        ),
        PrebuildReuseState::RejectedPolicy | PrebuildReuseState::RejectedTrust => (
            ExecutionContextEffectClass::PrebuildRejected,
            ExecutionContextReasonCode::PrebuildRejectedByTrust,
        ),
    };
    ExecutionContextExplanation {
        field_path: "prebuild_metadata.reuse_state".to_owned(),
        effect,
        reason_code,
        source: ExecutionContextReasonSource::EnvironmentCapsule,
        resolved_value_token: prebuild.reuse_state.as_str().to_owned(),
        related_input_sources: Vec::new(),
        applicable_surfaces: reusable_surfaces.to_vec(),
    }
}

fn mixed_version_explanation(
    mixed_version: &MixedVersionDrift,
    reusable_surfaces: &[SurfaceClass],
) -> ExecutionContextExplanation {
    let (effect, reason_code) = match mixed_version.state {
        MixedVersionDriftState::NotApplicable | MixedVersionDriftState::Aligned => (
            ExecutionContextEffectClass::MixedVersionNotApplicable,
            ExecutionContextReasonCode::LocalOnlyNoHelperVersion,
        ),
        MixedVersionDriftState::NotNegotiated | MixedVersionDriftState::DriftDetected => (
            ExecutionContextEffectClass::MixedVersionUnchecked,
            ExecutionContextReasonCode::HelperBoundaryNotNegotiated,
        ),
    };
    ExecutionContextExplanation {
        field_path: "mixed_version_drift.state".to_owned(),
        effect,
        reason_code,
        source: ExecutionContextReasonSource::HelperBoundary,
        resolved_value_token: mixed_version.state.as_str().to_owned(),
        related_input_sources: Vec::new(),
        applicable_surfaces: reusable_surfaces.to_vec(),
    }
}

const fn trust_state_token(state: TrustState) -> &'static str {
    match state {
        TrustState::Trusted => "trusted",
        TrustState::Restricted => "restricted",
        TrustState::PendingEvaluation => "pending_evaluation",
    }
}

const fn default_toolchain_id_for(class: ToolchainClass) -> &'static str {
    match class {
        ToolchainClass::LoginShell => "shell.login_shell",
        ToolchainClass::BuildDriverRuntime => "build.seed_driver",
        ToolchainClass::TestRunnerRuntime => "test.seed_runner",
        ToolchainClass::DebugAdapterRuntime => "debug.seed_adapter",
        ToolchainClass::Interpreter => "interpreter.seed",
        ToolchainClass::CompilerToolchain => "compiler.seed",
        ToolchainClass::PackageManagerRunner => "pkg.seed_runner",
        ToolchainClass::ContainerisedRuntime => "container.seed_runtime",
        ToolchainClass::NotebookKernelRuntime => "notebook.seed_kernel",
        ToolchainClass::LanguageServerProcess => "ls.seed_server",
        ToolchainClass::AiToolRuntime => "ai.seed_runtime",
    }
}

const fn default_activation_strategy_for(class: ToolchainClass) -> ActivationStrategy {
    match class {
        ToolchainClass::LoginShell => ActivationStrategy::AmbientPath,
        _ => ActivationStrategy::FallbackResolution,
    }
}

const fn surface_default_toolchain(surface: SurfaceClass) -> ToolchainClass {
    match surface {
        SurfaceClass::Terminal => ToolchainClass::LoginShell,
        SurfaceClass::Task => ToolchainClass::BuildDriverRuntime,
        SurfaceClass::Debug => ToolchainClass::DebugAdapterRuntime,
        SurfaceClass::Test => ToolchainClass::TestRunnerRuntime,
        SurfaceClass::NotebookKernel => ToolchainClass::NotebookKernelRuntime,
        SurfaceClass::Scaffolding => ToolchainClass::BuildDriverRuntime,
        SurfaceClass::AiToolCall => ToolchainClass::AiToolRuntime,
        SurfaceClass::DoctorRepair => ToolchainClass::BuildDriverRuntime,
        SurfaceClass::ImportProbe => ToolchainClass::BuildDriverRuntime,
        SurfaceClass::ReplayProbe => ToolchainClass::BuildDriverRuntime,
    }
}

fn pick_with_decision<T, I, F>(
    field: ResolverInputField,
    candidates: I,
    fallback_value: T,
    fallback_source: ResolverInputSource,
    render: F,
) -> (T, ResolverInputDecision)
where
    T: Copy + PartialEq,
    I: IntoIterator<Item = (Option<T>, ResolverInputSource)>,
    F: Fn(T) -> String,
{
    let mut contributing: Vec<(T, ResolverInputSource)> = candidates
        .into_iter()
        .filter_map(|(value, source)| value.map(|v| (v, source)))
        .collect();
    if contributing.is_empty() {
        contributing.push((fallback_value, fallback_source));
    }
    contributing.sort_by_key(|(_, source)| source.rank());
    let (winning_value, winning_source) = contributing[0];

    let mut conflicting = Vec::new();
    for (value, source) in contributing.iter().skip(1) {
        if *value != winning_value {
            conflicting.push(*source);
        }
    }

    (
        winning_value,
        ResolverInputDecision {
            field,
            winning_source,
            conflicting_sources: conflicting,
            resolved_value_token: render(winning_value),
        },
    )
}

fn add_node_detection_degraded_field(
    degraded_fields: &mut Vec<DegradedFieldRecord>,
    field_path: &str,
    state: NodeToolchainResolutionState,
) {
    let reason = match state {
        NodeToolchainResolutionState::Resolved => return,
        NodeToolchainResolutionState::Fallback => DegradedFieldReason::ToolchainFallback,
        NodeToolchainResolutionState::Missing => DegradedFieldReason::ProvenanceGap,
        NodeToolchainResolutionState::Ambiguous => DegradedFieldReason::ConfidenceLow,
        NodeToolchainResolutionState::Unsupported => {
            DegradedFieldReason::ActivatorUnsupportedOnTarget
        }
    };
    if degraded_fields
        .iter()
        .any(|field| field.field_path == field_path && field.reason == reason)
    {
        return;
    }
    degraded_fields.push(DegradedFieldRecord {
        field_path: field_path.to_owned(),
        reason,
        repair_hook_ref: Some("doctor.repair.node_toolchain".to_owned()),
    });
}

fn add_python_detection_degraded_field(
    degraded_fields: &mut Vec<DegradedFieldRecord>,
    field_path: &str,
    state: PythonEnvironmentResolutionState,
) {
    let reason = match state {
        PythonEnvironmentResolutionState::Resolved => return,
        PythonEnvironmentResolutionState::Fallback => DegradedFieldReason::ToolchainFallback,
        PythonEnvironmentResolutionState::Missing => DegradedFieldReason::ProvenanceGap,
        PythonEnvironmentResolutionState::Ambiguous => DegradedFieldReason::ConfidenceLow,
        PythonEnvironmentResolutionState::Unsupported => {
            DegradedFieldReason::ActivatorUnsupportedOnTarget
        }
    };
    if degraded_fields
        .iter()
        .any(|field| field.field_path == field_path && field.reason == reason)
    {
        return;
    }
    degraded_fields.push(DegradedFieldRecord {
        field_path: field_path.to_owned(),
        reason,
        repair_hook_ref: Some("doctor.repair.python_environment".to_owned()),
    });
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::path::Path;

    fn baseline_config() -> ExecutionContextResolverConfig {
        ExecutionContextResolverConfig {
            workspace_id: "ws-test".to_owned(),
            profile_id: Some("prof.default".to_owned()),
            identity_mode: IdentityMode::AccountFreeLocal,
            policy_epoch: 1,
            workspace_default_target_class: TargetClass::LocalHost,
            workspace_default_working_directory: Some("/workspace".to_owned()),
            workspace_default_scope_class: ScopeClass::CurrentRoot,
            local_host_canonical_id: "localhost:darwin-arm64".to_owned(),
            environment_capsule_ref: EnvironmentCapsuleRef {
                capsule_id: "caps:ws-test:seed".to_owned(),
                capsule_hash: "sha256:seed".to_owned(),
                resolved_schema_version: "1".to_owned(),
                drift_state: CapsuleDriftState::InSync,
            },
            resolver_version: "seed-0".to_owned(),
        }
    }

    #[test]
    fn terminal_seed_resolves_against_local_host_with_login_shell() {
        let mut resolver = ExecutionContextResolver::new(baseline_config());
        let context = resolver.resolve(ExecutionContextRequest::local_terminal_seed(
            "terminal.open",
            TrustState::Trusted,
            "mono:0",
        ));

        assert_eq!(context.record_kind, EXECUTION_CONTEXT_RECORD_KIND);
        assert_eq!(context.schema_version, EXECUTION_CONTEXT_SCHEMA_VERSION);
        assert_eq!(context.invocation_subject.surface, SurfaceClass::Terminal);
        assert_eq!(context.target_identity.target_class, TargetClass::LocalHost);
        assert_eq!(
            context.toolchain_identity.toolchain_class,
            ToolchainClass::LoginShell
        );
        assert_eq!(
            context.toolchain_identity.activation_strategy,
            ActivationStrategy::AmbientPath
        );
        assert!(!context.boundary_cue_visible());
        assert!(!context.has_degraded_field());
        assert_eq!(
            context.target_identity.working_directory.as_deref(),
            Some("/workspace")
        );
        assert_eq!(context.cache_disposition, CacheDisposition::Cold);
        assert_eq!(context.provenance.confidence_level, ConfidenceLevel::High);
    }

    #[test]
    fn launch_surfaces_share_the_same_object_shape() {
        let mut resolver = ExecutionContextResolver::new(baseline_config());

        let terminal = resolver.resolve(ExecutionContextRequest::local_terminal_seed(
            "terminal.open",
            TrustState::Trusted,
            "mono:0",
        ));
        let task = resolver.resolve(ExecutionContextRequest::task_seed(
            "task.run.cargo_build",
            TrustState::Trusted,
            "mono:1",
        ));
        let package_script = resolver.resolve(ExecutionContextRequest::package_script_task_seed(
            "task.run.package_script",
            TrustState::Trusted,
            "mono:2",
        ));
        let debug = resolver.resolve(ExecutionContextRequest::debug_prep_seed(
            "debug.prep.attach",
            TrustState::Trusted,
            "mono:3",
        ));
        let test = resolver.resolve(ExecutionContextRequest::test_seed(
            "test.run.changed",
            TrustState::Trusted,
            "mono:4",
        ));
        let ai = resolver.resolve(ExecutionContextRequest::ai_tool_call_seed(
            "ai.apply.preview",
            TrustState::Trusted,
            "mono:5",
        ));

        for ctx in [&terminal, &task, &package_script, &debug, &test, &ai] {
            assert_eq!(ctx.record_kind, EXECUTION_CONTEXT_RECORD_KIND);
            assert_eq!(ctx.schema_version, EXECUTION_CONTEXT_SCHEMA_VERSION);
            assert_eq!(ctx.invocation_subject.workspace_id, "ws-test");
            assert_eq!(ctx.workset_scope_class, ScopeClass::CurrentRoot);
            assert_eq!(ctx.policy_and_trust.trust_state, TrustState::Trusted);
            assert_eq!(
                ctx.policy_and_trust.identity_mode,
                IdentityMode::AccountFreeLocal
            );
            assert!(ctx.reusable_surfaces.contains(&SurfaceClass::Terminal));
            assert!(ctx.reusable_surfaces.contains(&SurfaceClass::Task));
            assert!(ctx.reusable_surfaces.contains(&SurfaceClass::Test));
            assert!(ctx.reusable_surfaces.contains(&SurfaceClass::Debug));
            assert!(ctx.reusable_surfaces.contains(&SurfaceClass::AiToolCall));
        }

        assert_eq!(
            (
                terminal.invocation_subject.surface,
                task.invocation_subject.surface,
                package_script.invocation_subject.surface,
                debug.invocation_subject.surface,
                test.invocation_subject.surface,
                ai.invocation_subject.surface,
            ),
            (
                SurfaceClass::Terminal,
                SurfaceClass::Task,
                SurfaceClass::Task,
                SurfaceClass::Debug,
                SurfaceClass::Test,
                SurfaceClass::AiToolCall,
            )
        );
        assert_eq!(
            (
                terminal.toolchain_identity.toolchain_class,
                task.toolchain_identity.toolchain_class,
                package_script.toolchain_identity.toolchain_class,
                debug.toolchain_identity.toolchain_class,
                test.toolchain_identity.toolchain_class,
                ai.toolchain_identity.toolchain_class,
            ),
            (
                ToolchainClass::LoginShell,
                ToolchainClass::BuildDriverRuntime,
                ToolchainClass::PackageManagerRunner,
                ToolchainClass::DebugAdapterRuntime,
                ToolchainClass::TestRunnerRuntime,
                ToolchainClass::AiToolRuntime,
            )
        );

        assert_ne!(
            terminal.execution_context_id, task.execution_context_id,
            "every resolved context carries a unique id"
        );
        assert_ne!(
            task.execution_context_id,
            package_script.execution_context_id
        );
        assert_ne!(
            package_script.execution_context_id,
            debug.execution_context_id
        );
        assert_ne!(debug.execution_context_id, test.execution_context_id);
        assert_ne!(test.execution_context_id, ai.execution_context_id);
    }

    #[test]
    fn pending_trust_state_records_a_degraded_field() {
        let mut resolver = ExecutionContextResolver::new(baseline_config());
        let context = resolver.resolve(ExecutionContextRequest::local_terminal_seed(
            "terminal.open",
            TrustState::PendingEvaluation,
            "mono:0",
        ));
        assert!(context.has_degraded_field());
        assert_eq!(context.degraded_fields.len(), 1);
        assert_eq!(
            context.degraded_fields[0].reason,
            DegradedFieldReason::TrustStateUnresolved
        );
        assert_eq!(context.provenance.confidence_level, ConfidenceLevel::Medium);
    }

    #[test]
    fn explicit_override_wins_over_surface_request_and_records_conflict() {
        // Failure drill: caller passes conflicting cwd / target inputs. The
        // resolver settles on the highest-precedence source and records the
        // losing source(s) in the provenance row so support exports can quote
        // exactly which input survived.
        let mut resolver = ExecutionContextResolver::new(baseline_config());
        let mut request = ExecutionContextRequest::local_terminal_seed(
            "terminal.open",
            TrustState::Trusted,
            "mono:0",
        );
        request.override_target_class = Some(TargetClass::SshRemote);
        request.override_working_directory = Some("/srv/code");
        request.requested_working_directory = Some("/workspace/active");

        let context = resolver.resolve(request);
        assert_eq!(
            context.target_identity.target_class,
            TargetClass::SshRemote,
            "explicit override wins"
        );
        assert!(context.boundary_cue_visible());
        assert_eq!(
            context.target_identity.working_directory.as_deref(),
            Some("/srv/code")
        );

        let target_decision = context
            .provenance
            .input_decisions
            .iter()
            .find(|d| d.field == ResolverInputField::TargetClass)
            .expect("target decision must be recorded");
        assert_eq!(
            target_decision.winning_source,
            ResolverInputSource::ExplicitOverride
        );
        assert!(target_decision
            .conflicting_sources
            .contains(&ResolverInputSource::SurfaceRequested));
        assert!(target_decision
            .conflicting_sources
            .contains(&ResolverInputSource::WorkspaceDefault));
        assert_eq!(target_decision.resolved_value_token, "ssh_remote");

        let wd_decision = context
            .provenance
            .input_decisions
            .iter()
            .find(|d| d.field == ResolverInputField::WorkingDirectory)
            .expect("working-directory decision must be recorded");
        assert_eq!(
            wd_decision.winning_source,
            ResolverInputSource::ExplicitOverride
        );
        assert!(wd_decision
            .conflicting_sources
            .contains(&ResolverInputSource::SurfaceRequested));
        assert!(wd_decision
            .conflicting_sources
            .contains(&ResolverInputSource::WorkspaceDefault));
        assert_eq!(wd_decision.resolved_value_token, "/srv/code");
    }

    #[test]
    fn surface_request_wins_when_no_explicit_override_and_workspace_default_disagrees() {
        let mut resolver = ExecutionContextResolver::new(baseline_config());
        let mut request = ExecutionContextRequest::local_terminal_seed(
            "terminal.open",
            TrustState::Trusted,
            "mono:0",
        );
        request.requested_working_directory = Some("/workspace/active");

        let context = resolver.resolve(request);
        assert_eq!(
            context.target_identity.working_directory.as_deref(),
            Some("/workspace/active")
        );
        let wd_decision = context
            .provenance
            .input_decisions
            .iter()
            .find(|d| d.field == ResolverInputField::WorkingDirectory)
            .expect("working-directory decision must be recorded");
        assert_eq!(
            wd_decision.winning_source,
            ResolverInputSource::SurfaceRequested
        );
        assert_eq!(
            wd_decision.conflicting_sources,
            vec![ResolverInputSource::WorkspaceDefault]
        );
    }

    #[test]
    fn fixture_failure_drill_matches_resolver_output() {
        let fixture_path = Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../../fixtures/runtime/execution_context_seed_cases/conflicting_inputs.json");
        let payload = std::fs::read_to_string(&fixture_path).expect("fixture must read");
        let fixture: ConflictingInputsFixture =
            serde_json::from_str(&payload).expect("fixture must parse");

        assert_eq!(fixture.record_kind, "execution_context_seed_case");
        assert_eq!(fixture.schema_version, 1);

        let mut resolver = ExecutionContextResolver::new(baseline_config());
        let context = resolver.resolve(ExecutionContextRequest {
            command_id: &fixture.input.command_id,
            surface: fixture.input.surface,
            actor_class: fixture.input.actor_class,
            trust_state: fixture.input.trust_state,
            observed_at: &fixture.input.observed_at,
            requested_target_class: fixture.input.requested_target_class,
            requested_working_directory: fixture.input.requested_working_directory.as_deref(),
            requested_toolchain_class: fixture.input.requested_toolchain_class,
            override_target_class: fixture.input.override_target_class,
            override_working_directory: fixture.input.override_working_directory.as_deref(),
            override_toolchain_class: fixture.input.override_toolchain_class,
        });

        assert_eq!(
            context.target_identity.target_class,
            fixture.expect.target_class
        );
        assert_eq!(
            context.target_identity.working_directory,
            fixture.expect.working_directory
        );

        let target_decision = context
            .provenance
            .input_decisions
            .iter()
            .find(|d| d.field == ResolverInputField::TargetClass)
            .expect("target decision must be recorded");
        assert_eq!(
            target_decision.winning_source,
            fixture.expect.target_winning_source
        );
        for source in &fixture.expect.target_conflicting_sources {
            assert!(target_decision.conflicting_sources.contains(source));
        }

        let wd_decision = context
            .provenance
            .input_decisions
            .iter()
            .find(|d| d.field == ResolverInputField::WorkingDirectory)
            .expect("working-directory decision must be recorded");
        assert_eq!(
            wd_decision.winning_source,
            fixture.expect.working_directory_winning_source
        );
    }

    #[test]
    fn execution_context_alpha_fixtures_replay_structured_explanations() {
        let fixture_root = Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../../fixtures/runtime/execution_context_alpha");
        for fixture_name in [
            "reusable_launch_surfaces.json",
            "restricted_remote_policy_narrow.json",
            "prebuild_drift_rejected.json",
        ] {
            let fixture_path = fixture_root.join(fixture_name);
            let payload = std::fs::read_to_string(&fixture_path).expect("fixture must read");
            let fixture: ExecutionContextAlphaFixture =
                serde_json::from_str(&payload).expect("fixture must parse");

            assert_eq!(fixture.record_kind, "execution_context_alpha_case");
            assert_eq!(fixture.schema_version, 1);

            let mut config = baseline_config();
            if let Some(alpha_config) = fixture.config {
                if let Some(drift_state) = alpha_config.capsule_drift_state {
                    config.environment_capsule_ref.drift_state = drift_state;
                }
            }

            let mut resolver = ExecutionContextResolver::new(config);
            let context = resolver.resolve(ExecutionContextRequest {
                command_id: &fixture.input.command_id,
                surface: fixture.input.surface,
                actor_class: fixture.input.actor_class,
                trust_state: fixture.input.trust_state,
                observed_at: &fixture.input.observed_at,
                requested_target_class: fixture.input.requested_target_class,
                requested_working_directory: fixture.input.requested_working_directory.as_deref(),
                requested_toolchain_class: fixture.input.requested_toolchain_class,
                override_target_class: fixture.input.override_target_class,
                override_working_directory: fixture.input.override_working_directory.as_deref(),
                override_toolchain_class: fixture.input.override_toolchain_class,
            });

            let exported = serde_json::to_value(&context).expect("context must export as JSON");
            assert_eq!(
                exported["record_kind"],
                serde_json::Value::String(EXECUTION_CONTEXT_RECORD_KIND.to_owned())
            );
            assert!(exported.get("target_confidence").is_some());
            assert!(exported.get("prebuild_metadata").is_some());
            assert!(exported.get("mixed_version_drift").is_some());
            assert!(exported.get("explanations").is_some());
            assert!(exported.get("degraded_fields").is_some());

            assert_eq!(
                context.target_confidence.level,
                fixture.expect.target_confidence_level
            );
            for reason in &fixture.expect.target_confidence_reasons {
                assert!(
                    context.target_confidence.reasons.contains(reason),
                    "missing target confidence reason {reason:?} in {fixture_name}"
                );
            }
            for surface in &fixture.expect.reusable_surfaces {
                assert!(
                    context.reusable_surfaces.contains(surface),
                    "missing reusable surface {surface:?} in {fixture_name}"
                );
            }
            assert_eq!(
                context.prebuild_metadata.reuse_state,
                fixture.expect.prebuild_reuse_state
            );
            if let Some(reason) = fixture.expect.prebuild_invalidation_reason {
                assert_eq!(context.prebuild_metadata.invalidation_reason, Some(reason));
            }
            if let Some(cache) = fixture.expect.cache_disposition {
                assert_eq!(context.cache_disposition, cache);
            }
            assert_eq!(
                context.mixed_version_drift.state,
                fixture.expect.mixed_version_state
            );
            for reason in &fixture.expect.degraded_reasons {
                assert!(
                    context
                        .degraded_fields
                        .iter()
                        .any(|field| field.reason == *reason),
                    "missing degraded reason {reason:?} in {fixture_name}"
                );
            }
            for effect in &fixture.expect.required_effects {
                assert!(
                    context
                        .explanations
                        .iter()
                        .any(|explanation| explanation.effect == *effect),
                    "missing explanation effect {effect:?} in {fixture_name}"
                );
            }
            for reason_code in &fixture.expect.required_reason_codes {
                assert!(
                    context
                        .explanations
                        .iter()
                        .any(|explanation| explanation.reason_code == *reason_code),
                    "missing explanation reason {reason_code:?} in {fixture_name}"
                );
            }
        }
    }

    #[derive(Debug, Deserialize)]
    struct ConflictingInputsFixture {
        record_kind: String,
        schema_version: u32,
        input: ConflictingInputsInput,
        expect: ConflictingInputsExpect,
    }

    #[derive(Debug, Deserialize)]
    struct ConflictingInputsInput {
        command_id: String,
        surface: SurfaceClass,
        actor_class: ActorClass,
        trust_state: TrustState,
        observed_at: String,
        #[serde(default)]
        requested_target_class: Option<TargetClass>,
        #[serde(default)]
        requested_working_directory: Option<String>,
        #[serde(default)]
        requested_toolchain_class: Option<ToolchainClass>,
        #[serde(default)]
        override_target_class: Option<TargetClass>,
        #[serde(default)]
        override_working_directory: Option<String>,
        #[serde(default)]
        override_toolchain_class: Option<ToolchainClass>,
    }

    #[derive(Debug, Deserialize)]
    struct ConflictingInputsExpect {
        target_class: TargetClass,
        working_directory: Option<String>,
        target_winning_source: ResolverInputSource,
        target_conflicting_sources: Vec<ResolverInputSource>,
        working_directory_winning_source: ResolverInputSource,
    }

    #[derive(Debug, Deserialize)]
    struct ExecutionContextAlphaFixture {
        record_kind: String,
        schema_version: u32,
        #[serde(default)]
        config: Option<ExecutionContextAlphaConfig>,
        input: ConflictingInputsInput,
        expect: ExecutionContextAlphaExpect,
    }

    #[derive(Debug, Deserialize)]
    struct ExecutionContextAlphaConfig {
        #[serde(default)]
        capsule_drift_state: Option<CapsuleDriftState>,
    }

    #[derive(Debug, Deserialize)]
    struct ExecutionContextAlphaExpect {
        target_confidence_level: ConfidenceLevel,
        #[serde(default)]
        target_confidence_reasons: Vec<TargetConfidenceReason>,
        prebuild_reuse_state: PrebuildReuseState,
        #[serde(default)]
        prebuild_invalidation_reason: Option<PrebuildInvalidationReason>,
        #[serde(default)]
        cache_disposition: Option<CacheDisposition>,
        mixed_version_state: MixedVersionDriftState,
        #[serde(default)]
        reusable_surfaces: Vec<SurfaceClass>,
        #[serde(default)]
        degraded_reasons: Vec<DegradedFieldReason>,
        #[serde(default)]
        required_effects: Vec<ExecutionContextEffectClass>,
        #[serde(default)]
        required_reason_codes: Vec<ExecutionContextReasonCode>,
    }
}
