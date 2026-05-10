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
//! ## Seed scope (M1)
//!
//! The resolver in M1 is intentionally small. It models:
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

pub use aureline_workspace::TrustState;

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
    pub toolchain_identity: ToolchainIdentity,
    pub environment_capsule_ref: EnvironmentCapsuleRef,
    pub policy_and_trust: PolicyAndTrust,
    pub workset_scope_class: ScopeClass,
    pub cache_disposition: CacheDisposition,
    pub provenance: Provenance,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
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

        let provenance = Provenance {
            provenance_record_id,
            recorded_at: request.observed_at.to_owned(),
            resolver_version: self.config.resolver_version.clone(),
            confidence_level,
            input_decisions: vec![target_decision, wd_decision, toolchain_decision],
        };

        ExecutionContext {
            record_kind: EXECUTION_CONTEXT_RECORD_KIND.to_owned(),
            schema_version: EXECUTION_CONTEXT_SCHEMA_VERSION,
            execution_context_id,
            invocation_subject,
            target_identity,
            toolchain_identity,
            environment_capsule_ref: self.config.environment_capsule_ref.clone(),
            policy_and_trust,
            workset_scope_class: self.config.workspace_default_scope_class,
            cache_disposition: CacheDisposition::Cold,
            provenance,
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
    fn terminal_task_and_debug_seeds_share_the_same_object_shape() {
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
        let debug = resolver.resolve(ExecutionContextRequest::debug_prep_seed(
            "debug.prep.attach",
            TrustState::Trusted,
            "mono:2",
        ));

        for ctx in [&terminal, &task, &debug] {
            assert_eq!(ctx.record_kind, EXECUTION_CONTEXT_RECORD_KIND);
            assert_eq!(ctx.schema_version, EXECUTION_CONTEXT_SCHEMA_VERSION);
            assert_eq!(ctx.invocation_subject.workspace_id, "ws-test");
            assert_eq!(ctx.target_identity.target_class, TargetClass::LocalHost);
            assert_eq!(ctx.workset_scope_class, ScopeClass::CurrentRoot);
            assert_eq!(ctx.policy_and_trust.trust_state, TrustState::Trusted);
            assert_eq!(
                ctx.policy_and_trust.identity_mode,
                IdentityMode::AccountFreeLocal
            );
        }

        assert_eq!(
            (
                terminal.invocation_subject.surface,
                task.invocation_subject.surface,
                debug.invocation_subject.surface,
            ),
            (
                SurfaceClass::Terminal,
                SurfaceClass::Task,
                SurfaceClass::Debug
            )
        );
        assert_eq!(
            (
                terminal.toolchain_identity.toolchain_class,
                task.toolchain_identity.toolchain_class,
                debug.toolchain_identity.toolchain_class,
            ),
            (
                ToolchainClass::LoginShell,
                ToolchainClass::BuildDriverRuntime,
                ToolchainClass::DebugAdapterRuntime,
            )
        );

        assert_ne!(
            terminal.execution_context_id, task.execution_context_id,
            "every resolved context carries a unique id"
        );
        assert_ne!(task.execution_context_id, debug.execution_context_id);
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
}
