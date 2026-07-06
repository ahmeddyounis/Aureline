//! One reusable M5 remote-target-pill / environment-status-strip primitive:
//! target identity, host-or-environment boundary, degraded / reconnect state,
//! resolved runtime kind and label, winning source, scope, readiness, and a
//! one-step "Why this context?" entrypoint, projected the same way across every M5
//! run-capable surface.
//!
//! Aureline's frozen runtime-boundary component matrix
//! ([`crate::freeze_the_m5_terminal_tab_remote_target_pill_environment_status_strip_toolchain_pin_row_presence_avatar_stack_and_repair_action_card_component_matrix`])
//! names the remote target pill and the environment status strip as two governed
//! component families and freezes their controlled vocabulary — the host-boundary
//! classes, the remote connection states, and the runtime source classes. This
//! module *implements* those two contracts as one reusable primitive so a user can
//! tell, from the same place they launch work, which target and runtime won, where
//! the value came from, and whether the current state is ready, degraded, or
//! blocked — instead of inferring the active host / runtime from unrelated logs or
//! settings panels.
//!
//! The primitive has two halves:
//!
//! 1. A resolver — [`resolve_run_context`] — that takes one run context's target
//!    identity, host boundary, remote connection state, resolved runtime kind and
//!    label, winning runtime source, resolved scope, and effective-value provenance,
//!    and produces one [`M5ResolvedRunContext`] carrying the derived remote-target
//!    posture (local-inline versus connected versus reconnecting versus offline-
//!    cached versus disconnected) and the derived environment readiness (ready
//!    versus degraded-cached / degraded-narrowed / degraded-unreachable versus
//!    blocked-by-policy / blocked-unresolved). The resolver never shows a cached,
//!    narrowed, or policy-blocked effective value as cleanly ready, and never masks
//!    a degraded or disconnected remote target as a healthy one.
//! 2. A parity matrix — [`M5RemoteTargetEnvironmentPrimitivePacket`] — that binds one
//!    row per claimed M5 run-capable surface (the run console, the test runner, the
//!    debug session, the notebook runtime, the request runner, the database session,
//!    the preview server, the pipeline run, and the incident surface) to the shared
//!    remote-target-pill anatomy and environment-status-strip anatomy, the same
//!    target postures, readiness states, provenance states, and scopes, the same
//!    export fields, and the same non-visual accessibility routes, so the source /
//!    scope / readiness truth stays identical on every surface and the support /
//!    export packet reconstructs target and runtime resolution from one shared model.
//!
//! The host-boundary class ([`M5HostBoundaryClass`]), remote connection state
//! ([`M5RemoteConnectionState`]), runtime source class ([`M5RuntimeSourceClass`]),
//! non-visual accessibility routes ([`M5RuntimeBoundaryAccessibilityRoute`]),
//! qualification classes ([`M5RuntimeBoundaryQualificationClass`]), and downgrade
//! triggers ([`M5RuntimeBoundaryDowngradeTrigger`]) are reused verbatim from the
//! frozen runtime-boundary matrix; the shell topology — zones, responsive classes,
//! window classes, and consumer surfaces — is reused from the frozen shell-zone
//! matrix. This module mints new vocabulary only for what the frozen matrix left
//! implicit about the pill and the strip themselves: their run-capable surfaces,
//! their anatomy parts, their derived target postures, their environment readiness
//! states, their effective-value provenance, their resolved scopes, and their
//! export fields. No M5 surface invents a second target or environment grammar.
//!
//! Raw URLs, raw endpoints, raw usernames, raw hostnames, tokens, credentials, and
//! user text bodies stay outside the support boundary; every target identity,
//! runtime kind, and resolved runtime label is carried only as an opaque,
//! export-safe representation.
//!
//! The boundary schema is
//! [`schemas/ui/m5-remote-target-pill.schema.json`](../../../../schemas/ui/m5-remote-target-pill.schema.json)
//! and the contract doc is
//! [`docs/components/m5_remote_target_environment_primitive_contract.md`](../../../../docs/components/m5_remote_target_environment_primitive_contract.md).
//! The protected fixture directory is
//! [`fixtures/ui/m5-remote-target-environment-primitive/`](../../../../fixtures/ui/m5-remote-target-environment-primitive/).

mod seed;
#[cfg(test)]
mod tests;

pub use seed::{
    seeded_m5_remote_target_environment_primitive_incident_surface_beta_narrowed,
    seeded_m5_remote_target_environment_primitive_packet,
    seeded_m5_remote_target_environment_primitive_pipeline_run_preview_narrowed,
    M5_REMOTE_TARGET_ENVIRONMENT_PRIMITIVE_PACKET_ID,
};

// The host-boundary class, remote connection state, runtime source class,
// accessibility routes, qualification classes, and downgrade triggers are frozen
// once, in the runtime-boundary component matrix. This primitive reuses them
// verbatim so it never invents a parallel target or environment vocabulary.
pub use crate::freeze_the_m5_terminal_tab_remote_target_pill_environment_status_strip_toolchain_pin_row_presence_avatar_stack_and_repair_action_card_component_matrix::{
    M5HostBoundaryClass, M5RemoteConnectionState, M5RuntimeBoundaryAccessibilityRoute,
    M5RuntimeBoundaryDowngradeTrigger, M5RuntimeBoundaryQualificationClass, M5RuntimeSourceClass,
};

// The canonical shell topology — zones, responsive classes, window classes, and
// consumer surfaces — is frozen once, in the shell-zone matrix.
pub use crate::freeze_the_m5_shell_zone_responsive_class_and_multi_window_continuity_matrix::{
    M5ResponsiveClass, M5ShellConsumerSurface, M5ShellZoneSlot, M5WindowClass,
};

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

/// Stable record-kind tag carried by [`M5RemoteTargetEnvironmentPrimitivePacket`].
pub const M5_REMOTE_TARGET_ENVIRONMENT_PRIMITIVE_RECORD_KIND: &str =
    "implement_m5_remote_target_pill_and_environment_status_strip_runtime_source_readiness_and_context_entrypoint_primitive";

/// Schema version for M5 remote-target / environment-primitive records.
pub const M5_REMOTE_TARGET_ENVIRONMENT_PRIMITIVE_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the remote-target-pill boundary schema (the packet schema).
pub const M5_REMOTE_TARGET_SCHEMA_REF: &str = "schemas/ui/m5-remote-target-pill.schema.json";

/// Repo-relative path of the companion environment-status-strip component schema.
pub const M5_ENVIRONMENT_STRIP_SCHEMA_REF: &str =
    "schemas/ui/m5-environment-status-strip.schema.json";

/// Repo-relative path of the contract doc.
pub const M5_REMOTE_TARGET_ENVIRONMENT_DOC_REF: &str =
    "docs/components/m5_remote_target_environment_primitive_contract.md";

/// Repo-relative path of the frozen shell-zone schema this primitive binds against.
pub const M5_REMOTE_TARGET_ENVIRONMENT_SHELL_ZONE_REF: &str =
    "schemas/shell/m5-shell-zone.schema.json";

/// Repo-relative path of the frozen runtime-boundary component matrix this primitive
/// narrows from.
pub const M5_REMOTE_TARGET_ENVIRONMENT_COMPONENT_MATRIX_REF: &str =
    "schemas/ui/m5-runtime-boundary-components.schema.json";

/// Repo-relative path of the environment-status-strip registry contract this
/// primitive projects source / scope / readiness truth from.
pub const M5_REMOTE_TARGET_ENVIRONMENT_EXECUTION_CONTEXT_REF: &str =
    "schemas/runtime/m5-environment-status-strip.schema.json";

/// Repo-relative path of the target-context contract this primitive projects
/// target-identity and remote-connection truth from.
pub const M5_REMOTE_TARGET_ENVIRONMENT_TARGET_CONTEXT_REF: &str =
    "schemas/runtime/target_context.schema.json";

/// Repo-relative path of the protected fixture directory.
pub const M5_REMOTE_TARGET_ENVIRONMENT_FIXTURE_DIR: &str =
    "fixtures/ui/m5-remote-target-environment-primitive";

/// Repo-relative path of the checked support-export artifact.
pub const M5_REMOTE_TARGET_ENVIRONMENT_ARTIFACT_REF: &str =
    "artifacts/release/m5-remote-target-environment-proof/support_export.json";

/// Repo-relative path of the checked machine-readable matrix CSV.
pub const M5_REMOTE_TARGET_ENVIRONMENT_CSV_REF: &str =
    "artifacts/release/m5-remote-target-environment-proof/matrix.csv";

/// Repo-relative path of the checked Markdown report.
pub const M5_REMOTE_TARGET_ENVIRONMENT_REPORT_REF: &str =
    "artifacts/components/m5-remote-target-environment-primitive.md";

/// One claimed M5 run-capable surface that renders the shared remote-target pill and
/// environment status strip. These are the surfaces the acceptance criteria name —
/// the run console, the test runner, the debug session, the notebook runtime, the
/// request runner, the database session, the preview server, the pipeline run, and
/// the incident surface.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5RunCapableSurface {
    /// The primary run console.
    RunConsole,
    /// The test runner.
    TestRunner,
    /// The debug session.
    DebugSession,
    /// The notebook runtime / kernel.
    NotebookRuntime,
    /// The request / REPL runner.
    RequestRunner,
    /// The database session.
    DatabaseSession,
    /// The preview server.
    PreviewServer,
    /// The pipeline run.
    PipelineRun,
    /// The incident / break-glass surface.
    IncidentSurface,
}

impl M5RunCapableSurface {
    /// Every claimed run-capable surface, in declaration order.
    pub const ALL: [Self; 9] = [
        Self::RunConsole,
        Self::TestRunner,
        Self::DebugSession,
        Self::NotebookRuntime,
        Self::RequestRunner,
        Self::DatabaseSession,
        Self::PreviewServer,
        Self::PipelineRun,
        Self::IncidentSurface,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::RunConsole => "run_console",
            Self::TestRunner => "test_runner",
            Self::DebugSession => "debug_session",
            Self::NotebookRuntime => "notebook_runtime",
            Self::RequestRunner => "request_runner",
            Self::DatabaseSession => "database_session",
            Self::PreviewServer => "preview_server",
            Self::PipelineRun => "pipeline_run",
            Self::IncidentSurface => "incident_surface",
        }
    }

    /// Review-safe label for evidence packets and docs.
    pub const fn label(self) -> &'static str {
        match self {
            Self::RunConsole => "Run Console",
            Self::TestRunner => "Test Runner",
            Self::DebugSession => "Debug Session",
            Self::NotebookRuntime => "Notebook Runtime",
            Self::RequestRunner => "Request Runner",
            Self::DatabaseSession => "Database Session",
            Self::PreviewServer => "Preview Server",
            Self::PipelineRun => "Pipeline Run",
            Self::IncidentSurface => "Incident Surface",
        }
    }
}

/// One anatomy part the shared remote-target pill surfaces. The first three in
/// [`M5RemoteTargetPillPart::MANDATORY`] are required on every pill so a user can
/// tell which target won before launching work.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5RemoteTargetPillPart {
    /// The target identity / label.
    TargetIdentity,
    /// The local-or-remote-or-container-or-managed host / environment class.
    HostOrEnvironmentClass,
    /// The typed remote connection state.
    ConnectionState,
    /// The degraded / reconnect cue.
    DegradedOrReconnectCue,
    /// The affordance that opens fuller context inspection.
    ContextInspectAffordance,
}

impl M5RemoteTargetPillPart {
    /// Every remote-target-pill part, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::TargetIdentity,
        Self::HostOrEnvironmentClass,
        Self::ConnectionState,
        Self::DegradedOrReconnectCue,
        Self::ContextInspectAffordance,
    ];

    /// The pill parts every remote target pill must render.
    pub const MANDATORY: [Self; 3] = [
        Self::TargetIdentity,
        Self::HostOrEnvironmentClass,
        Self::ConnectionState,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::TargetIdentity => "target_identity",
            Self::HostOrEnvironmentClass => "host_or_environment_class",
            Self::ConnectionState => "connection_state",
            Self::DegradedOrReconnectCue => "degraded_or_reconnect_cue",
            Self::ContextInspectAffordance => "context_inspect_affordance",
        }
    }
}

/// One anatomy part the shared environment status strip surfaces. The parts in
/// [`M5EnvironmentStripPart::MANDATORY`] are required on every strip so a user can
/// tell what runtime won, where it came from, and whether it is ready.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5EnvironmentStripPart {
    /// The toolchain / runtime kind.
    RuntimeKind,
    /// The resolved runtime label and version.
    ResolvedLabelVersion,
    /// The winning source cue.
    WinningSource,
    /// The resolved scope cue.
    ScopeCue,
    /// The readiness state.
    ReadinessState,
    /// The effective-value provenance cue (cached / narrowed / policy-blocked).
    EffectiveValueProvenanceCue,
    /// The one-step "Why this context?" entrypoint.
    WhyThisContextEntrypoint,
}

impl M5EnvironmentStripPart {
    /// Every environment-strip part, in declaration order.
    pub const ALL: [Self; 7] = [
        Self::RuntimeKind,
        Self::ResolvedLabelVersion,
        Self::WinningSource,
        Self::ScopeCue,
        Self::ReadinessState,
        Self::EffectiveValueProvenanceCue,
        Self::WhyThisContextEntrypoint,
    ];

    /// The strip parts every environment status strip must render.
    pub const MANDATORY: [Self; 5] = [
        Self::RuntimeKind,
        Self::ResolvedLabelVersion,
        Self::WinningSource,
        Self::ReadinessState,
        Self::WhyThisContextEntrypoint,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::RuntimeKind => "runtime_kind",
            Self::ResolvedLabelVersion => "resolved_label_version",
            Self::WinningSource => "winning_source",
            Self::ScopeCue => "scope_cue",
            Self::ReadinessState => "readiness_state",
            Self::EffectiveValueProvenanceCue => "effective_value_provenance_cue",
            Self::WhyThisContextEntrypoint => "why_this_context_entrypoint",
        }
    }
}

/// The derived posture of a remote target pill — whether the target is the local
/// machine, a healthy remote, or a degraded / reconnecting / offline / disconnected
/// remote, so a stale or offline connection is never shown as connected.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5RemoteTargetPosture {
    /// The local machine; no remote pill is needed.
    LocalInline,
    /// A healthy, connected remote target.
    ConnectedHealthy,
    /// A remote target whose connection is being established.
    Establishing,
    /// A remote target that dropped and is reconnecting.
    Reconnecting,
    /// A remote target served from an offline / mirrored cache.
    OfflineCached,
    /// A disconnected remote target.
    Disconnected,
}

impl M5RemoteTargetPosture {
    /// Every remote-target posture, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::LocalInline,
        Self::ConnectedHealthy,
        Self::Establishing,
        Self::Reconnecting,
        Self::OfflineCached,
        Self::Disconnected,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LocalInline => "local_inline",
            Self::ConnectedHealthy => "connected_healthy",
            Self::Establishing => "establishing",
            Self::Reconnecting => "reconnecting",
            Self::OfflineCached => "offline_cached",
            Self::Disconnected => "disconnected",
        }
    }

    /// Whether this posture is a degraded remote target (reconnecting, offline, or
    /// disconnected) that must show a degraded / reconnect cue.
    pub const fn is_degraded(self) -> bool {
        matches!(
            self,
            Self::Reconnecting | Self::OfflineCached | Self::Disconnected
        )
    }
}

/// The derived readiness of an environment status strip — the headline verdict for
/// whether the resolved runtime is ready to use, degraded, or blocked. A cached,
/// narrowed, or policy-blocked effective value is never shown as cleanly ready.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5EnvironmentReadiness {
    /// The resolved runtime is ready to use.
    Ready,
    /// The effective value is served from an offline cache.
    DegradedCached,
    /// The effective value is a narrowed / approximate resolution.
    DegradedNarrowed,
    /// The remote target is unreachable so the environment cannot be confirmed.
    DegradedUnreachableTarget,
    /// The effective value is blocked by policy.
    BlockedByPolicy,
    /// The effective value could not be resolved.
    BlockedUnresolved,
}

impl M5EnvironmentReadiness {
    /// Every readiness state, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::Ready,
        Self::DegradedCached,
        Self::DegradedNarrowed,
        Self::DegradedUnreachableTarget,
        Self::BlockedByPolicy,
        Self::BlockedUnresolved,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Ready => "ready",
            Self::DegradedCached => "degraded_cached",
            Self::DegradedNarrowed => "degraded_narrowed",
            Self::DegradedUnreachableTarget => "degraded_unreachable_target",
            Self::BlockedByPolicy => "blocked_by_policy",
            Self::BlockedUnresolved => "blocked_unresolved",
        }
    }

    /// Whether this readiness state is the clean, ready-to-use verdict.
    pub const fn is_ready(self) -> bool {
        matches!(self, Self::Ready)
    }

    /// Whether this readiness state blocks work before it starts.
    pub const fn is_blocked(self) -> bool {
        matches!(self, Self::BlockedByPolicy | Self::BlockedUnresolved)
    }
}

/// The provenance of the effective resolved runtime value, so a cached, narrowed, or
/// policy-blocked value never masquerades as a cleanly resolved one.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5EffectiveValueProvenance {
    /// The value resolved cleanly from live truth.
    Resolved,
    /// The value is served from an offline / mirrored cache.
    CachedOffline,
    /// The value is a narrowed / approximate resolution.
    NarrowedApproximate,
    /// The value is blocked by policy.
    PolicyBlocked,
    /// The value could not be resolved.
    Unresolved,
}

impl M5EffectiveValueProvenance {
    /// Every provenance class, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::Resolved,
        Self::CachedOffline,
        Self::NarrowedApproximate,
        Self::PolicyBlocked,
        Self::Unresolved,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Resolved => "resolved",
            Self::CachedOffline => "cached_offline",
            Self::NarrowedApproximate => "narrowed_approximate",
            Self::PolicyBlocked => "policy_blocked",
            Self::Unresolved => "unresolved",
        }
    }
}

/// The scope at which the resolved runtime value won, so the winning scope is always
/// explicit alongside the winning source.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ResolvedScope {
    /// A session-scoped resolution.
    SessionScope,
    /// A project-scoped resolution.
    ProjectScope,
    /// A workspace-scoped resolution.
    WorkspaceScope,
    /// A host-scoped resolution.
    HostScope,
    /// A global-default-scoped resolution.
    GlobalDefaultScope,
}

impl M5ResolvedScope {
    /// Every resolved scope, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::SessionScope,
        Self::ProjectScope,
        Self::WorkspaceScope,
        Self::HostScope,
        Self::GlobalDefaultScope,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SessionScope => "session_scope",
            Self::ProjectScope => "project_scope",
            Self::WorkspaceScope => "workspace_scope",
            Self::HostScope => "host_scope",
            Self::GlobalDefaultScope => "global_default_scope",
        }
    }
}

/// A field the support / export packet carries so target and runtime resolution is
/// reconstructable from the shared model. The fields in
/// [`M5RunContextExportField::MANDATORY`] are required.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5RunContextExportField {
    /// The opaque target-identity representation.
    TargetIdentity,
    /// The host-boundary class.
    HostBoundary,
    /// The remote connection state.
    ConnectionState,
    /// The derived remote-target posture.
    TargetPosture,
    /// The runtime kind.
    RuntimeKind,
    /// The resolved runtime label / version.
    ResolvedRuntime,
    /// The winning runtime source.
    RuntimeSource,
    /// The resolved scope.
    ResolvedScope,
    /// The effective-value provenance.
    EffectiveValueProvenance,
    /// The derived environment readiness.
    Readiness,
    /// The "Why this context?" entrypoint presence.
    WhyContextEntrypoint,
}

impl M5RunContextExportField {
    /// Every export field, in declaration order.
    pub const ALL: [Self; 11] = [
        Self::TargetIdentity,
        Self::HostBoundary,
        Self::ConnectionState,
        Self::TargetPosture,
        Self::RuntimeKind,
        Self::ResolvedRuntime,
        Self::RuntimeSource,
        Self::ResolvedScope,
        Self::EffectiveValueProvenance,
        Self::Readiness,
        Self::WhyContextEntrypoint,
    ];

    /// The export fields every run-context export must carry.
    pub const MANDATORY: [Self; 6] = [
        Self::TargetIdentity,
        Self::HostBoundary,
        Self::RuntimeKind,
        Self::RuntimeSource,
        Self::ResolvedScope,
        Self::Readiness,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::TargetIdentity => "target_identity",
            Self::HostBoundary => "host_boundary",
            Self::ConnectionState => "connection_state",
            Self::TargetPosture => "target_posture",
            Self::RuntimeKind => "runtime_kind",
            Self::ResolvedRuntime => "resolved_runtime",
            Self::RuntimeSource => "runtime_source",
            Self::ResolvedScope => "resolved_scope",
            Self::EffectiveValueProvenance => "effective_value_provenance",
            Self::Readiness => "readiness",
            Self::WhyContextEntrypoint => "why_context_entrypoint",
        }
    }
}

/// True when this host boundary is the local machine.
const fn host_is_local(host: M5HostBoundaryClass) -> bool {
    matches!(host, M5HostBoundaryClass::LocalHost)
}

/// True when this connection state means the remote target is unreachable right now.
const fn connection_is_unreachable(state: M5RemoteConnectionState) -> bool {
    matches!(
        state,
        M5RemoteConnectionState::Reconnecting | M5RemoteConnectionState::Disconnected
    )
}

/// The full input to the run-context resolver for one run-capable surface.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5RunContextResolutionInput {
    /// The opaque, export-safe target-identity representation.
    pub context_title: String,
    /// The host boundary the target runs on.
    pub host_boundary: M5HostBoundaryClass,
    /// The remote connection state. Required for a non-local host, forbidden for a
    /// local host.
    pub connection_state: Option<M5RemoteConnectionState>,
    /// The opaque runtime-kind representation (e.g. an interpreter / SDK family).
    pub runtime_kind_repr: String,
    /// The opaque resolved-runtime label / version representation.
    pub resolved_runtime_repr: String,
    /// The source class that won the resolved runtime.
    pub runtime_source: M5RuntimeSourceClass,
    /// The scope at which the resolved runtime won.
    pub scope: M5ResolvedScope,
    /// The provenance of the effective resolved value.
    pub effective_value_provenance: M5EffectiveValueProvenance,
}

/// The resolved target / environment truth for one run-capable surface.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ResolvedRunContext {
    /// The opaque target-identity representation.
    pub context_title: String,
    /// The host boundary the target runs on.
    pub host_boundary: M5HostBoundaryClass,
    /// True when the host boundary is a remote (non-local) target.
    pub is_remote: bool,
    /// The remote connection state, when non-local.
    pub connection_state: Option<M5RemoteConnectionState>,
    /// The derived remote-target posture.
    pub target_posture: M5RemoteTargetPosture,
    /// True when the remote target is degraded (reconnecting / offline / disconnected).
    pub target_is_degraded: bool,
    /// The opaque runtime-kind representation.
    pub runtime_kind_repr: String,
    /// The opaque resolved-runtime label / version representation.
    pub resolved_runtime_repr: String,
    /// The winning runtime source.
    pub runtime_source: M5RuntimeSourceClass,
    /// The resolved scope.
    pub scope: M5ResolvedScope,
    /// The provenance of the effective resolved value.
    pub effective_value_provenance: M5EffectiveValueProvenance,
    /// The derived environment readiness.
    pub readiness: M5EnvironmentReadiness,
    /// True when the environment is cleanly ready to use.
    pub is_ready: bool,
    /// True when the environment is blocked before work can start.
    pub is_blocked: bool,
    /// True when the "Why this context?" entrypoint is exposed. Always `true`.
    pub exposes_why_context_entrypoint: bool,
}

/// Errors returned by [`resolve_run_context`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum M5RunContextResolutionError {
    /// The target-identity title was empty.
    EmptyContextTitle,
    /// The runtime-kind representation was empty.
    EmptyRuntimeKind,
    /// The resolved-runtime representation was empty.
    EmptyResolvedRuntime,
    /// A non-local host carried no connection state.
    RemoteHostMissingConnectionState,
    /// A local host carried a remote connection state.
    LocalHostWithConnectionState,
    /// A representation carried forbidden material.
    ForbiddenContextMaterial,
}

impl M5RunContextResolutionError {
    /// Stable token for tests and diagnostics.
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::EmptyContextTitle => "empty_context_title",
            Self::EmptyRuntimeKind => "empty_runtime_kind",
            Self::EmptyResolvedRuntime => "empty_resolved_runtime",
            Self::RemoteHostMissingConnectionState => "remote_host_missing_connection_state",
            Self::LocalHostWithConnectionState => "local_host_with_connection_state",
            Self::ForbiddenContextMaterial => "forbidden_context_material",
        }
    }
}

impl fmt::Display for M5RunContextResolutionError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "run-context resolution error: {}", self.as_str())
    }
}

impl Error for M5RunContextResolutionError {}

/// Resolves one run context's remote-target pill and environment status strip from
/// its target / runtime state.
///
/// The remote-target posture is derived from the host boundary and the remote
/// connection state, so a degraded or disconnected remote target is never shown as a
/// healthy one. The environment readiness is the headline verdict: a policy-blocked
/// value is blocked-by-policy, an unresolved value is blocked-unresolved, a cached
/// value is degraded-cached, a narrowed value is degraded-narrowed, an unreachable
/// remote target is degraded-unreachable, and only a cleanly resolved, reachable
/// value is ready. A cached, narrowed, or policy-blocked effective value is therefore
/// never presented as cleanly ready, and the "Why this context?" entrypoint is always
/// exposed from the same place work launches.
pub fn resolve_run_context(
    input: &M5RunContextResolutionInput,
) -> Result<M5ResolvedRunContext, M5RunContextResolutionError> {
    if input.context_title.trim().is_empty() {
        return Err(M5RunContextResolutionError::EmptyContextTitle);
    }
    if input.runtime_kind_repr.trim().is_empty() {
        return Err(M5RunContextResolutionError::EmptyRuntimeKind);
    }
    if input.resolved_runtime_repr.trim().is_empty() {
        return Err(M5RunContextResolutionError::EmptyResolvedRuntime);
    }
    for repr in [
        &input.context_title,
        &input.runtime_kind_repr,
        &input.resolved_runtime_repr,
    ] {
        if value_repr_is_forbidden(repr) {
            return Err(M5RunContextResolutionError::ForbiddenContextMaterial);
        }
    }

    let is_local = host_is_local(input.host_boundary);
    match (is_local, input.connection_state.is_some()) {
        (true, true) => return Err(M5RunContextResolutionError::LocalHostWithConnectionState),
        (false, false) => {
            return Err(M5RunContextResolutionError::RemoteHostMissingConnectionState)
        }
        _ => {}
    }
    let is_remote = !is_local;

    let target_posture = if is_local {
        M5RemoteTargetPosture::LocalInline
    } else {
        match input
            .connection_state
            .expect("remote host has connection state")
        {
            M5RemoteConnectionState::Connected => M5RemoteTargetPosture::ConnectedHealthy,
            M5RemoteConnectionState::Connecting => M5RemoteTargetPosture::Establishing,
            M5RemoteConnectionState::Reconnecting => M5RemoteTargetPosture::Reconnecting,
            M5RemoteConnectionState::Disconnected => M5RemoteTargetPosture::Disconnected,
            M5RemoteConnectionState::OfflineCached => M5RemoteTargetPosture::OfflineCached,
        }
    };
    let target_is_degraded = target_posture.is_degraded();

    let readiness = match input.effective_value_provenance {
        M5EffectiveValueProvenance::PolicyBlocked => M5EnvironmentReadiness::BlockedByPolicy,
        M5EffectiveValueProvenance::Unresolved => M5EnvironmentReadiness::BlockedUnresolved,
        M5EffectiveValueProvenance::CachedOffline => M5EnvironmentReadiness::DegradedCached,
        M5EffectiveValueProvenance::NarrowedApproximate => M5EnvironmentReadiness::DegradedNarrowed,
        M5EffectiveValueProvenance::Resolved => {
            if is_remote
                && input
                    .connection_state
                    .is_some_and(connection_is_unreachable)
            {
                M5EnvironmentReadiness::DegradedUnreachableTarget
            } else {
                M5EnvironmentReadiness::Ready
            }
        }
    };

    Ok(M5ResolvedRunContext {
        context_title: input.context_title.clone(),
        host_boundary: input.host_boundary,
        is_remote,
        connection_state: input.connection_state,
        target_posture,
        target_is_degraded,
        runtime_kind_repr: input.runtime_kind_repr.clone(),
        resolved_runtime_repr: input.resolved_runtime_repr.clone(),
        runtime_source: input.runtime_source,
        scope: input.scope,
        effective_value_provenance: input.effective_value_provenance,
        readiness,
        is_ready: readiness.is_ready(),
        is_blocked: readiness.is_blocked(),
        exposes_why_context_entrypoint: true,
    })
}

/// One worked resolution case carried in the packet so the support / export packet
/// reconstructs target and runtime resolution from the shared model.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5RunContextResolutionCase {
    /// The resolver input.
    pub input: M5RunContextResolutionInput,
    /// The resolved truth. Must equal `resolve_run_context(&input)`.
    pub resolved: M5ResolvedRunContext,
}

impl M5RunContextResolutionCase {
    /// Builds a case by resolving `input`.
    ///
    /// # Panics
    ///
    /// Panics if `input` does not resolve; seed inputs are always valid.
    pub fn resolved(input: M5RunContextResolutionInput) -> Self {
        let resolved = resolve_run_context(&input).expect("seed resolution case is valid");
        Self { input, resolved }
    }

    /// True when the stored resolution matches a fresh resolve of the input.
    pub fn is_self_consistent(&self) -> bool {
        resolve_run_context(&self.input).as_ref() == Ok(&self.resolved)
    }
}

/// One row in the primitive matrix: one run-capable surface bound to the shared pill
/// and strip anatomy, target postures, readiness states, provenance states, scopes,
/// export fields, and accessibility routes.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5RunCapableSurfaceRow {
    /// Run-capable surface family.
    pub run_surface: M5RunCapableSurface,
    /// Qualification class earned by this surface.
    pub qualification: M5RuntimeBoundaryQualificationClass,
    /// Owner role accountable for keeping this surface governed.
    pub owner_role: String,
    /// Human-readable scope summary.
    pub scope_summary: String,
    /// Canonical shell zone this pill / strip attaches to.
    pub shell_zone_slot: M5ShellZoneSlot,
    /// Responsive classes this pill / strip must survive.
    pub responsive_classes: Vec<M5ResponsiveClass>,
    /// Window classes this pill / strip keeps continuity across.
    pub window_classes: Vec<M5WindowClass>,
    /// Remote-target-pill parts this surface renders (must include the mandatory
    /// parts).
    pub pill_parts: Vec<M5RemoteTargetPillPart>,
    /// Environment-strip parts this surface renders (must include the mandatory
    /// parts).
    pub strip_parts: Vec<M5EnvironmentStripPart>,
    /// Target postures this surface distinguishes.
    pub target_postures: Vec<M5RemoteTargetPosture>,
    /// Readiness states this surface distinguishes.
    pub readiness_states: Vec<M5EnvironmentReadiness>,
    /// Effective-value provenance states this surface distinguishes.
    pub provenance_states: Vec<M5EffectiveValueProvenance>,
    /// Resolved scopes this surface distinguishes.
    pub resolved_scopes: Vec<M5ResolvedScope>,
    /// Export fields this surface carries (must include the mandatory fields).
    pub export_fields: Vec<M5RunContextExportField>,
    /// Non-visual accessibility routes this surface offers.
    pub accessibility_routes: Vec<M5RuntimeBoundaryAccessibilityRoute>,
    /// Shell subsystems that consume this surface's projection.
    pub consumer_surfaces: Vec<M5ShellConsumerSurface>,
    /// Downgrade triggers that apply to this surface.
    pub downgrade_triggers: Vec<M5RuntimeBoundaryDowngradeTrigger>,
    /// Proof packet refs that keep this row current.
    pub required_proof_packet_refs: Vec<String>,
    /// Source contract refs consumed by this row.
    pub source_contract_refs: Vec<String>,
    /// Worked resolution cases proving the resolver on this surface.
    pub example_resolutions: Vec<M5RunContextResolutionCase>,
    /// Hard invariant: this surface never masks the host / environment boundary. MUST
    /// be `false`.
    pub masks_host_or_environment_boundary: bool,
    /// Hard invariant: this surface never conflates ready with degraded or blocked.
    /// MUST be `false`.
    pub conflates_ready_and_degraded_or_blocked: bool,
    /// Hard invariant: this surface never invents a private status grammar. MUST be
    /// `false`.
    pub invents_private_status_grammar: bool,
    /// Hard invariant: this surface never hides the "Why this context?" entrypoint.
    /// MUST be `false`.
    pub hides_why_this_context_entrypoint: bool,
}

impl M5RunCapableSurfaceRow {
    /// True when the row declares every mandatory pill part.
    fn declares_mandatory_pill_parts(&self) -> bool {
        let present: BTreeSet<M5RemoteTargetPillPart> = self.pill_parts.iter().copied().collect();
        M5RemoteTargetPillPart::MANDATORY
            .iter()
            .all(|part| present.contains(part))
    }

    /// True when the row declares every mandatory strip part.
    fn declares_mandatory_strip_parts(&self) -> bool {
        let present: BTreeSet<M5EnvironmentStripPart> = self.strip_parts.iter().copied().collect();
        M5EnvironmentStripPart::MANDATORY
            .iter()
            .all(|part| present.contains(part))
    }

    /// True when the row declares every mandatory export field.
    fn declares_mandatory_export_fields(&self) -> bool {
        let present: BTreeSet<M5RunContextExportField> =
            self.export_fields.iter().copied().collect();
        M5RunContextExportField::MANDATORY
            .iter()
            .all(|field| present.contains(field))
    }

    /// True when the row's hard invariants hold.
    fn honours_invariants(&self) -> bool {
        !self.masks_host_or_environment_boundary
            && !self.conflates_ready_and_degraded_or_blocked
            && !self.invents_private_status_grammar
            && !self.hides_why_this_context_entrypoint
    }
}

/// Self-describing controlled-vocabulary set carried by this primitive.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5RemoteTargetEnvironmentVocabularySet {
    /// Run-capable surface tokens.
    pub run_surfaces: Vec<String>,
    /// Remote-target-pill-part tokens.
    pub pill_parts: Vec<String>,
    /// Environment-strip-part tokens.
    pub strip_parts: Vec<String>,
    /// Target-posture tokens.
    pub target_postures: Vec<String>,
    /// Readiness-state tokens.
    pub readiness_states: Vec<String>,
    /// Effective-value-provenance tokens.
    pub provenance_states: Vec<String>,
    /// Resolved-scope tokens.
    pub resolved_scopes: Vec<String>,
    /// Export-field tokens.
    pub export_fields: Vec<String>,
    /// Host-boundary-class tokens (reused from the frozen matrix).
    pub host_boundary_classes: Vec<String>,
    /// Connection-state tokens (reused from the frozen matrix).
    pub connection_states: Vec<String>,
    /// Runtime-source-class tokens (reused from the frozen matrix).
    pub runtime_source_classes: Vec<String>,
    /// Accessibility-route tokens (reused from the frozen matrix).
    pub accessibility_routes: Vec<String>,
}

impl M5RemoteTargetEnvironmentVocabularySet {
    /// Builds the canonical vocabulary set from the typed `ALL` arrays.
    pub fn canonical() -> Self {
        Self {
            run_surfaces: tokens(&M5RunCapableSurface::ALL, |v| v.as_str()),
            pill_parts: tokens(&M5RemoteTargetPillPart::ALL, |v| v.as_str()),
            strip_parts: tokens(&M5EnvironmentStripPart::ALL, |v| v.as_str()),
            target_postures: tokens(&M5RemoteTargetPosture::ALL, |v| v.as_str()),
            readiness_states: tokens(&M5EnvironmentReadiness::ALL, |v| v.as_str()),
            provenance_states: tokens(&M5EffectiveValueProvenance::ALL, |v| v.as_str()),
            resolved_scopes: tokens(&M5ResolvedScope::ALL, |v| v.as_str()),
            export_fields: tokens(&M5RunContextExportField::ALL, |v| v.as_str()),
            host_boundary_classes: tokens(&M5HostBoundaryClass::ALL, |v| v.as_str()),
            connection_states: tokens(&M5RemoteConnectionState::ALL, |v| v.as_str()),
            runtime_source_classes: tokens(&M5RuntimeSourceClass::ALL, |v| v.as_str()),
            accessibility_routes: tokens(&M5RuntimeBoundaryAccessibilityRoute::ALL, |v| v.as_str()),
        }
    }

    /// Returns true when this set matches the canonical token lists exactly.
    pub fn matches_canonical(&self) -> bool {
        *self == Self::canonical()
    }
}

fn tokens<T: Copy>(items: &[T], to_token: impl Fn(T) -> &'static str) -> Vec<String> {
    items.iter().map(|v| to_token(*v).to_owned()).collect()
}

/// Governance-review block; every flag is a hard invariant.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5RemoteTargetEnvironmentGovernanceReview {
    /// One primitive carries target and environment truth on every surface.
    pub one_primitive_carries_target_and_environment: bool,
    /// The target identity and host boundary are shown before launch.
    pub target_identity_and_host_boundary_always_shown: bool,
    /// The winning source and scope are always explicit.
    pub winning_source_and_scope_always_explicit: bool,
    /// The readiness state is always resolved (never left implicit).
    pub readiness_state_always_resolved: bool,
    /// A cached, narrowed, or blocked value is never shown as ready.
    pub cached_narrowed_or_blocked_never_shown_as_ready: bool,
    /// The "Why this context?" entrypoint is always present.
    pub why_this_context_entrypoint_always_present: bool,
    /// The support / export packet reconstructs source / scope / readiness truth.
    pub support_export_reconstructs_source_scope_readiness: bool,
    /// No surface invents a second target / environment status grammar.
    pub no_surface_invents_second_status_grammar: bool,
    /// Every row is bound to a canonical shell zone.
    pub every_row_bound_to_shell_zone: bool,
    /// Every row declares a non-visual accessibility route.
    pub every_row_declares_accessibility_route: bool,
    /// Later M5 rows cannot invent parallel target / environment vocabulary.
    pub later_rows_cannot_invent_parallel_vocabulary: bool,
}

/// Consumer projection block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5RemoteTargetEnvironmentConsumerProjection {
    /// Run, test, debug, notebook, request, database, preview, pipeline, and incident
    /// surfaces all consume the shared primitive.
    pub run_capable_surfaces_consume_shared_primitive: bool,
    /// The readiness resolver reads a single canonical source.
    pub readiness_resolver_reads_single_source: bool,
    /// The winning-source cue reads a single canonical resolution source.
    pub winning_source_reads_single_resolution_source: bool,
    /// The target pill reads a single canonical connection source.
    pub target_pill_reads_single_connection_source: bool,
    /// Support / export reads a single canonical run-context source.
    pub support_export_reads_single_source: bool,
}

/// Proof freshness block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5RemoteTargetEnvironmentProofFreshness {
    /// Proof-freshness SLO in hours.
    pub proof_freshness_slo_hours: u32,
    /// RFC 3339 timestamp of the last proof refresh.
    pub last_proof_refresh: String,
    /// True when stale proof automatically narrows the primitive.
    pub auto_narrow_on_stale: bool,
}

/// Release and support parity posture for the primitive.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5RemoteTargetEnvironmentReleasePosture {
    /// Ref of the supporting release packet.
    pub release_packet_ref: String,
    /// Ref of the supporting target / environment audit.
    pub environment_audit_ref: String,
    /// True when support / export parity is required for every surface.
    pub support_export_parity_required: bool,
    /// True when accessibility parity is required for every surface.
    pub accessibility_parity_required: bool,
}

/// Constructor input for [`M5RemoteTargetEnvironmentPrimitivePacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M5RemoteTargetEnvironmentPrimitivePacketInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable matrix label.
    pub matrix_label: String,
    /// Surface rows.
    pub surface_rows: Vec<M5RunCapableSurfaceRow>,
    /// Frozen controlled-vocabulary set.
    pub vocabulary_set: M5RemoteTargetEnvironmentVocabularySet,
    /// Governance-review block.
    pub governance_review: M5RemoteTargetEnvironmentGovernanceReview,
    /// Consumer projection block.
    pub consumer_projection: M5RemoteTargetEnvironmentConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: M5RemoteTargetEnvironmentProofFreshness,
    /// Release and support parity posture.
    pub release_posture: M5RemoteTargetEnvironmentReleasePosture,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

/// Export-safe M5 remote-target / environment-primitive packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5RemoteTargetEnvironmentPrimitivePacket {
    /// Record kind; must equal [`M5_REMOTE_TARGET_ENVIRONMENT_PRIMITIVE_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal
    /// [`M5_REMOTE_TARGET_ENVIRONMENT_PRIMITIVE_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable matrix label.
    pub matrix_label: String,
    /// Surface rows.
    pub surface_rows: Vec<M5RunCapableSurfaceRow>,
    /// Frozen controlled-vocabulary set.
    pub vocabulary_set: M5RemoteTargetEnvironmentVocabularySet,
    /// Governance-review block.
    pub governance_review: M5RemoteTargetEnvironmentGovernanceReview,
    /// Consumer projection block.
    pub consumer_projection: M5RemoteTargetEnvironmentConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: M5RemoteTargetEnvironmentProofFreshness,
    /// Release and support parity posture.
    pub release_posture: M5RemoteTargetEnvironmentReleasePosture,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

impl M5RemoteTargetEnvironmentPrimitivePacket {
    /// Builds an M5 remote-target / environment-primitive packet from stable-lane
    /// input.
    pub fn new(input: M5RemoteTargetEnvironmentPrimitivePacketInput) -> Self {
        Self {
            record_kind: M5_REMOTE_TARGET_ENVIRONMENT_PRIMITIVE_RECORD_KIND.to_owned(),
            schema_version: M5_REMOTE_TARGET_ENVIRONMENT_PRIMITIVE_SCHEMA_VERSION,
            packet_id: input.packet_id,
            matrix_label: input.matrix_label,
            surface_rows: input.surface_rows,
            vocabulary_set: input.vocabulary_set,
            governance_review: input.governance_review,
            consumer_projection: input.consumer_projection,
            proof_freshness: input.proof_freshness,
            release_posture: input.release_posture,
            source_contract_refs: input.source_contract_refs,
            redaction_class_token: input.redaction_class_token,
            minted_at: input.minted_at,
        }
    }

    /// Validates the M5 remote-target / environment-primitive invariants.
    pub fn validate(&self) -> Vec<M5RemoteTargetEnvironmentPrimitiveViolation> {
        let mut violations = Vec::new();

        if self.record_kind != M5_REMOTE_TARGET_ENVIRONMENT_PRIMITIVE_RECORD_KIND {
            violations.push(M5RemoteTargetEnvironmentPrimitiveViolation::WrongRecordKind);
        }
        if self.schema_version != M5_REMOTE_TARGET_ENVIRONMENT_PRIMITIVE_SCHEMA_VERSION {
            violations.push(M5RemoteTargetEnvironmentPrimitiveViolation::WrongSchemaVersion);
        }
        if self.packet_id.trim().is_empty()
            || self.matrix_label.trim().is_empty()
            || self.redaction_class_token.trim().is_empty()
            || self.minted_at.trim().is_empty()
        {
            violations.push(M5RemoteTargetEnvironmentPrimitiveViolation::MissingIdentity);
        }

        validate_source_contracts(self, &mut violations);
        validate_vocabulary_set(self, &mut violations);
        validate_surface_rows(self, &mut violations);
        validate_policy_blocked_readiness_covered(self, &mut violations);
        validate_cached_or_narrowed_readiness_covered(self, &mut violations);
        validate_remote_degraded_disclosure_covered(self, &mut violations);
        validate_governance_review(self, &mut violations);
        validate_consumer_projection(self, &mut violations);
        validate_proof_freshness(self, &mut violations);
        validate_release_posture(self, &mut violations);

        if json_contains_forbidden_material(
            &serde_json::to_value(self)
                .expect("m5 remote-target / environment primitive packet serializes"),
        ) {
            violations.push(M5RemoteTargetEnvironmentPrimitiveViolation::RawMaterialInExport);
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
            .expect("m5 remote-target / environment primitive packet serializes")
    }

    /// Deterministic, machine-readable matrix CSV: one row per run-capable surface.
    pub fn render_matrix_csv(&self) -> String {
        let mut out = String::new();
        out.push_str(
            "run_surface,qualification,owner,shell_zone_slot,pill_parts,strip_parts,target_postures,readiness_states,provenance_states,resolved_scopes,export_fields,example_count\n",
        );
        for row in &self.surface_rows {
            out.push_str(&format!(
                "{},{},{},{},{},{},{},{},{},{},{},{}\n",
                row.run_surface.as_str(),
                row.qualification.as_str(),
                csv_field(&row.owner_role),
                row.shell_zone_slot.as_str(),
                join_tokens(&row.pill_parts, |v| v.as_str()),
                join_tokens(&row.strip_parts, |v| v.as_str()),
                join_tokens(&row.target_postures, |v| v.as_str()),
                join_tokens(&row.readiness_states, |v| v.as_str()),
                join_tokens(&row.provenance_states, |v| v.as_str()),
                join_tokens(&row.resolved_scopes, |v| v.as_str()),
                join_tokens(&row.export_fields, |v| v.as_str()),
                row.example_resolutions.len(),
            ));
        }
        out
    }

    /// Deterministic Markdown report for support, docs, or review handoff.
    pub fn render_markdown_summary(&self) -> String {
        let stable_rows = self
            .surface_rows
            .iter()
            .filter(|row| row.qualification.is_stable())
            .count();
        let mut out = String::new();
        out.push_str(
            "# M5 Remote-Target Pill and Environment-Status Strip Primitive: Source, Scope, and Readiness\n\n",
        );
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- Label: `{}`\n", self.matrix_label));
        out.push_str(&format!(
            "- Run-capable surfaces: {} ({} stable)\n",
            self.surface_rows.len(),
            stable_rows
        ));
        out.push_str(&format!(
            "- Target postures: {}\n",
            self.vocabulary_set.target_postures.join(", ")
        ));
        out.push_str(&format!(
            "- Readiness states: {}\n",
            self.vocabulary_set.readiness_states.join(", ")
        ));
        out.push_str(&format!(
            "- Provenance states: {}\n",
            self.vocabulary_set.provenance_states.join(", ")
        ));
        out.push_str(&format!(
            "- Resolved scopes: {}\n",
            self.vocabulary_set.resolved_scopes.join(", ")
        ));
        out.push_str(&format!(
            "- Runtime source classes: {}\n",
            self.vocabulary_set.runtime_source_classes.join(", ")
        ));
        out.push_str(&format!(
            "- Proof freshness SLO: {} hours (last refresh: {})\n",
            self.proof_freshness.proof_freshness_slo_hours, self.proof_freshness.last_proof_refresh
        ));
        out.push_str("\n## Run-capable surfaces\n\n");
        for row in &self.surface_rows {
            out.push_str(&format!(
                "- **{}**: `{}`\n",
                row.run_surface.label(),
                row.qualification.as_str()
            ));
            out.push_str(&format!("  - Owner: {}\n", row.owner_role));
            out.push_str(&format!("  - Scope: {}\n", row.scope_summary));
            out.push_str(&format!(
                "  - Shell zone: `{}`\n",
                row.shell_zone_slot.as_str()
            ));
            out.push_str(&format!(
                "  - Worked resolutions: {}\n",
                row.example_resolutions.len()
            ));
            for case in &row.example_resolutions {
                out.push_str(&format!(
                    "    - `{}` on `{}` → target `{}`, `{}` (source `{}`, scope `{}`, {})\n",
                    case.resolved.context_title,
                    case.resolved.host_boundary.as_str(),
                    case.resolved.target_posture.as_str(),
                    case.resolved.readiness.as_str(),
                    case.resolved.runtime_source.as_str(),
                    case.resolved.scope.as_str(),
                    case.resolved.effective_value_provenance.as_str(),
                ));
            }
        }
        out
    }
}

/// Errors emitted when reading the checked-in M5 remote-target / environment export.
#[derive(Debug)]
pub enum M5RemoteTargetEnvironmentPrimitiveArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<M5RemoteTargetEnvironmentPrimitiveViolation>),
}

impl fmt::Display for M5RemoteTargetEnvironmentPrimitiveArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(
                    formatter,
                    "m5 remote-target / environment primitive export parse failed: {error}"
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
                    "m5 remote-target / environment primitive export failed validation: {tokens}"
                )
            }
        }
    }
}

impl Error for M5RemoteTargetEnvironmentPrimitiveArtifactError {}

/// Validation failures emitted by
/// [`M5RemoteTargetEnvironmentPrimitivePacket::validate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum M5RemoteTargetEnvironmentPrimitiveViolation {
    /// Packet record kind is wrong.
    WrongRecordKind,
    /// Packet schema version is wrong.
    WrongSchemaVersion,
    /// Required identity field is missing.
    MissingIdentity,
    /// Source contract refs are incomplete.
    MissingSourceContracts,
    /// The controlled vocabulary set drifted from the canonical token lists.
    VocabularySetDrift,
    /// A required run-capable surface family is missing from the matrix.
    RequiredSurfaceMissing,
    /// A surface row is incomplete.
    SurfaceRowIncomplete,
    /// A surface row omits one of the mandatory remote-target-pill parts.
    MandatoryPillPartMissing,
    /// A surface row omits one of the mandatory environment-strip parts.
    MandatoryStripPartMissing,
    /// A surface row declares no target postures.
    TargetPostureMissing,
    /// A surface row declares no readiness states.
    ReadinessStateMissing,
    /// A surface row declares no provenance states.
    ProvenanceStateMissing,
    /// A surface row declares no resolved scopes.
    ResolvedScopeMissing,
    /// A surface row omits one of the mandatory export fields.
    MandatoryExportFieldMissing,
    /// A surface row declares no accessibility routes (or misses keyboard focus).
    AccessibilityRouteMissing,
    /// A surface row declares no consumer surfaces.
    ConsumerSurfacesMissing,
    /// A surface row declares no downgrade triggers.
    DowngradeTriggersMissing,
    /// A surface row declares no worked resolution cases.
    ExampleResolutionMissing,
    /// A worked resolution case does not match a fresh resolve of its input.
    ExampleResolutionDrift,
    /// A surface claiming Stable is missing required proof packet refs.
    StableSurfaceMissingProof,
    /// No worked resolution proves a policy-blocked value resolving to blocked-by-
    /// policy readiness.
    PolicyBlockedReadinessUnproven,
    /// No worked resolution proves a cached or narrowed value resolving to a degraded,
    /// non-ready readiness.
    CachedOrNarrowedReadinessUnproven,
    /// No worked resolution proves a degraded remote target disclosing a non-healthy
    /// posture.
    RemoteDegradedDisclosureUnproven,
    /// A surface row violates a hard invariant.
    SurfaceInvariantViolated,
    /// Governance review does not satisfy required invariants.
    GovernanceReviewIncomplete,
    /// Consumer projection does not satisfy required invariants.
    ConsumerProjectionIncomplete,
    /// Proof freshness block is incomplete.
    ProofFreshnessIncomplete,
    /// Release / support parity posture is incomplete.
    ReleasePostureIncomplete,
    /// Export contains raw sensitive material.
    RawMaterialInExport,
}

impl M5RemoteTargetEnvironmentPrimitiveViolation {
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
            Self::MandatoryPillPartMissing => "mandatory_pill_part_missing",
            Self::MandatoryStripPartMissing => "mandatory_strip_part_missing",
            Self::TargetPostureMissing => "target_posture_missing",
            Self::ReadinessStateMissing => "readiness_state_missing",
            Self::ProvenanceStateMissing => "provenance_state_missing",
            Self::ResolvedScopeMissing => "resolved_scope_missing",
            Self::MandatoryExportFieldMissing => "mandatory_export_field_missing",
            Self::AccessibilityRouteMissing => "accessibility_route_missing",
            Self::ConsumerSurfacesMissing => "consumer_surfaces_missing",
            Self::DowngradeTriggersMissing => "downgrade_triggers_missing",
            Self::ExampleResolutionMissing => "example_resolution_missing",
            Self::ExampleResolutionDrift => "example_resolution_drift",
            Self::StableSurfaceMissingProof => "stable_surface_missing_proof",
            Self::PolicyBlockedReadinessUnproven => "policy_blocked_readiness_unproven",
            Self::CachedOrNarrowedReadinessUnproven => "cached_or_narrowed_readiness_unproven",
            Self::RemoteDegradedDisclosureUnproven => "remote_degraded_disclosure_unproven",
            Self::SurfaceInvariantViolated => "surface_invariant_violated",
            Self::GovernanceReviewIncomplete => "governance_review_incomplete",
            Self::ConsumerProjectionIncomplete => "consumer_projection_incomplete",
            Self::ProofFreshnessIncomplete => "proof_freshness_incomplete",
            Self::ReleasePostureIncomplete => "release_posture_incomplete",
            Self::RawMaterialInExport => "raw_material_in_export",
        }
    }
}

/// Reads and validates the checked-in stable M5 remote-target / environment export.
pub fn current_stable_m5_remote_target_environment_primitive_export(
) -> Result<M5RemoteTargetEnvironmentPrimitivePacket, M5RemoteTargetEnvironmentPrimitiveArtifactError>
{
    let packet: M5RemoteTargetEnvironmentPrimitivePacket =
        serde_json::from_str(include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../artifacts/release/m5-remote-target-environment-proof/support_export.json"
        )))
        .map_err(M5RemoteTargetEnvironmentPrimitiveArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(M5RemoteTargetEnvironmentPrimitiveArtifactError::Validation(
            violations,
        ))
    }
}

fn validate_source_contracts(
    packet: &M5RemoteTargetEnvironmentPrimitivePacket,
    violations: &mut Vec<M5RemoteTargetEnvironmentPrimitiveViolation>,
) {
    let refs: BTreeSet<&str> = packet
        .source_contract_refs
        .iter()
        .map(String::as_str)
        .collect();
    for required in [
        M5_REMOTE_TARGET_SCHEMA_REF,
        M5_ENVIRONMENT_STRIP_SCHEMA_REF,
        M5_REMOTE_TARGET_ENVIRONMENT_DOC_REF,
        M5_REMOTE_TARGET_ENVIRONMENT_SHELL_ZONE_REF,
        M5_REMOTE_TARGET_ENVIRONMENT_COMPONENT_MATRIX_REF,
        M5_REMOTE_TARGET_ENVIRONMENT_EXECUTION_CONTEXT_REF,
        M5_REMOTE_TARGET_ENVIRONMENT_TARGET_CONTEXT_REF,
    ] {
        if !refs.contains(required) {
            violations.push(M5RemoteTargetEnvironmentPrimitiveViolation::MissingSourceContracts);
            return;
        }
    }
}

fn validate_vocabulary_set(
    packet: &M5RemoteTargetEnvironmentPrimitivePacket,
    violations: &mut Vec<M5RemoteTargetEnvironmentPrimitiveViolation>,
) {
    if !packet.vocabulary_set.matches_canonical() {
        violations.push(M5RemoteTargetEnvironmentPrimitiveViolation::VocabularySetDrift);
    }
}

fn validate_surface_rows(
    packet: &M5RemoteTargetEnvironmentPrimitivePacket,
    violations: &mut Vec<M5RemoteTargetEnvironmentPrimitiveViolation>,
) {
    let present: BTreeSet<M5RunCapableSurface> = packet
        .surface_rows
        .iter()
        .map(|row| row.run_surface)
        .collect();
    for required in M5RunCapableSurface::ALL {
        if !present.contains(&required) {
            violations.push(M5RemoteTargetEnvironmentPrimitiveViolation::RequiredSurfaceMissing);
            return;
        }
    }

    for row in &packet.surface_rows {
        if row.owner_role.trim().is_empty()
            || row.scope_summary.trim().is_empty()
            || row.source_contract_refs.is_empty()
            || row.pill_parts.is_empty()
            || row.strip_parts.is_empty()
        {
            violations.push(M5RemoteTargetEnvironmentPrimitiveViolation::SurfaceRowIncomplete);
        }
        if !row.declares_mandatory_pill_parts() {
            violations.push(M5RemoteTargetEnvironmentPrimitiveViolation::MandatoryPillPartMissing);
        }
        if !row.declares_mandatory_strip_parts() {
            violations.push(M5RemoteTargetEnvironmentPrimitiveViolation::MandatoryStripPartMissing);
        }
        if row.target_postures.is_empty() {
            violations.push(M5RemoteTargetEnvironmentPrimitiveViolation::TargetPostureMissing);
        }
        if row.readiness_states.is_empty() {
            violations.push(M5RemoteTargetEnvironmentPrimitiveViolation::ReadinessStateMissing);
        }
        if row.provenance_states.is_empty() {
            violations.push(M5RemoteTargetEnvironmentPrimitiveViolation::ProvenanceStateMissing);
        }
        if row.resolved_scopes.is_empty() {
            violations.push(M5RemoteTargetEnvironmentPrimitiveViolation::ResolvedScopeMissing);
        }
        if !row.declares_mandatory_export_fields() {
            violations
                .push(M5RemoteTargetEnvironmentPrimitiveViolation::MandatoryExportFieldMissing);
        }
        if row.accessibility_routes.is_empty()
            || !row
                .accessibility_routes
                .contains(&M5RuntimeBoundaryAccessibilityRoute::KeyboardFocusable)
        {
            violations.push(M5RemoteTargetEnvironmentPrimitiveViolation::AccessibilityRouteMissing);
        }
        if row.consumer_surfaces.is_empty() {
            violations.push(M5RemoteTargetEnvironmentPrimitiveViolation::ConsumerSurfacesMissing);
        }
        if row.downgrade_triggers.is_empty() {
            violations.push(M5RemoteTargetEnvironmentPrimitiveViolation::DowngradeTriggersMissing);
        }
        if row.example_resolutions.is_empty() {
            violations.push(M5RemoteTargetEnvironmentPrimitiveViolation::ExampleResolutionMissing);
        }
        if row
            .example_resolutions
            .iter()
            .any(|case| !case.is_self_consistent())
        {
            violations.push(M5RemoteTargetEnvironmentPrimitiveViolation::ExampleResolutionDrift);
        }
        if row.qualification.is_stable() && row.required_proof_packet_refs.is_empty() {
            violations.push(M5RemoteTargetEnvironmentPrimitiveViolation::StableSurfaceMissingProof);
        }
        if !row.honours_invariants() {
            violations.push(M5RemoteTargetEnvironmentPrimitiveViolation::SurfaceInvariantViolated);
        }
    }
}

/// At least one worked resolution across the matrix must prove a policy-blocked
/// effective value resolving to a blocked-by-policy readiness — the acceptance-
/// criterion example that a policy-blocked value is not shown as ready.
fn validate_policy_blocked_readiness_covered(
    packet: &M5RemoteTargetEnvironmentPrimitivePacket,
    violations: &mut Vec<M5RemoteTargetEnvironmentPrimitiveViolation>,
) {
    let proven = packet.surface_rows.iter().any(|row| {
        row.example_resolutions.iter().any(|case| {
            case.resolved.effective_value_provenance == M5EffectiveValueProvenance::PolicyBlocked
                && case.resolved.readiness == M5EnvironmentReadiness::BlockedByPolicy
                && case.resolved.is_blocked
        })
    });
    if !proven {
        violations
            .push(M5RemoteTargetEnvironmentPrimitiveViolation::PolicyBlockedReadinessUnproven);
    }
}

/// At least one worked resolution across the matrix must prove a cached or narrowed
/// effective value resolving to a degraded, non-ready readiness — the acceptance-
/// criterion example that a cached or narrowed value keeps its degraded truth.
fn validate_cached_or_narrowed_readiness_covered(
    packet: &M5RemoteTargetEnvironmentPrimitivePacket,
    violations: &mut Vec<M5RemoteTargetEnvironmentPrimitiveViolation>,
) {
    let proven = packet.surface_rows.iter().any(|row| {
        row.example_resolutions.iter().any(|case| {
            matches!(
                case.resolved.effective_value_provenance,
                M5EffectiveValueProvenance::CachedOffline
                    | M5EffectiveValueProvenance::NarrowedApproximate
            ) && !case.resolved.is_ready
                && matches!(
                    case.resolved.readiness,
                    M5EnvironmentReadiness::DegradedCached
                        | M5EnvironmentReadiness::DegradedNarrowed
                )
        })
    });
    if !proven {
        violations
            .push(M5RemoteTargetEnvironmentPrimitiveViolation::CachedOrNarrowedReadinessUnproven);
    }
}

/// At least one worked resolution across the matrix must prove a degraded remote
/// target disclosing a non-healthy posture — the spec requirement that a pill shows
/// degraded / reconnect state rather than masking it as connected.
fn validate_remote_degraded_disclosure_covered(
    packet: &M5RemoteTargetEnvironmentPrimitivePacket,
    violations: &mut Vec<M5RemoteTargetEnvironmentPrimitiveViolation>,
) {
    let proven = packet.surface_rows.iter().any(|row| {
        row.example_resolutions.iter().any(|case| {
            case.resolved.is_remote
                && case.resolved.target_is_degraded
                && case.resolved.target_posture != M5RemoteTargetPosture::ConnectedHealthy
        })
    });
    if !proven {
        violations
            .push(M5RemoteTargetEnvironmentPrimitiveViolation::RemoteDegradedDisclosureUnproven);
    }
}

fn validate_governance_review(
    packet: &M5RemoteTargetEnvironmentPrimitivePacket,
    violations: &mut Vec<M5RemoteTargetEnvironmentPrimitiveViolation>,
) {
    let review = &packet.governance_review;
    for ok in [
        review.one_primitive_carries_target_and_environment,
        review.target_identity_and_host_boundary_always_shown,
        review.winning_source_and_scope_always_explicit,
        review.readiness_state_always_resolved,
        review.cached_narrowed_or_blocked_never_shown_as_ready,
        review.why_this_context_entrypoint_always_present,
        review.support_export_reconstructs_source_scope_readiness,
        review.no_surface_invents_second_status_grammar,
        review.every_row_bound_to_shell_zone,
        review.every_row_declares_accessibility_route,
        review.later_rows_cannot_invent_parallel_vocabulary,
    ] {
        if !ok {
            violations
                .push(M5RemoteTargetEnvironmentPrimitiveViolation::GovernanceReviewIncomplete);
            return;
        }
    }
}

fn validate_consumer_projection(
    packet: &M5RemoteTargetEnvironmentPrimitivePacket,
    violations: &mut Vec<M5RemoteTargetEnvironmentPrimitiveViolation>,
) {
    let projection = &packet.consumer_projection;
    for ok in [
        projection.run_capable_surfaces_consume_shared_primitive,
        projection.readiness_resolver_reads_single_source,
        projection.winning_source_reads_single_resolution_source,
        projection.target_pill_reads_single_connection_source,
        projection.support_export_reads_single_source,
    ] {
        if !ok {
            violations
                .push(M5RemoteTargetEnvironmentPrimitiveViolation::ConsumerProjectionIncomplete);
            return;
        }
    }
}

fn validate_proof_freshness(
    packet: &M5RemoteTargetEnvironmentPrimitivePacket,
    violations: &mut Vec<M5RemoteTargetEnvironmentPrimitiveViolation>,
) {
    if packet.proof_freshness.proof_freshness_slo_hours == 0
        || packet.proof_freshness.last_proof_refresh.trim().is_empty()
    {
        violations.push(M5RemoteTargetEnvironmentPrimitiveViolation::ProofFreshnessIncomplete);
    }
}

fn validate_release_posture(
    packet: &M5RemoteTargetEnvironmentPrimitivePacket,
    violations: &mut Vec<M5RemoteTargetEnvironmentPrimitiveViolation>,
) {
    let posture = &packet.release_posture;
    if posture.release_packet_ref.trim().is_empty()
        || posture.environment_audit_ref.trim().is_empty()
        || !posture.support_export_parity_required
        || !posture.accessibility_parity_required
    {
        violations.push(M5RemoteTargetEnvironmentPrimitiveViolation::ReleasePostureIncomplete);
    }
}

/// Joins tokens for a CSV cell with a `|` separator so a single cell never
/// introduces a stray comma.
fn join_tokens<T, F>(items: &[T], to_token: F) -> String
where
    F: Fn(&T) -> &'static str,
{
    items
        .iter()
        .map(|item| to_token(item))
        .collect::<Vec<_>>()
        .join("|")
}

/// Quotes a free-text CSV field when it contains a comma or quote.
fn csv_field(value: &str) -> String {
    if value.contains(',') || value.contains('"') || value.contains('\n') {
        format!("\"{}\"", value.replace('"', "\"\""))
    } else {
        value.to_owned()
    }
}

/// True when a single representation carries obviously forbidden material.
fn value_repr_is_forbidden(value: &str) -> bool {
    let lower = value.to_lowercase();
    lower.contains("api_key")
        || lower.contains("password")
        || lower.contains("secret")
        || lower.contains("bearer ")
        || lower.contains("://")
}

/// Heuristic that rejects obviously forbidden material in export-safe JSON.
fn json_contains_forbidden_material(value: &serde_json::Value) -> bool {
    match value {
        serde_json::Value::String(s) => value_repr_is_forbidden(s),
        serde_json::Value::Array(arr) => arr.iter().any(json_contains_forbidden_material),
        serde_json::Value::Object(map) => map.values().any(json_contains_forbidden_material),
        _ => false,
    }
}
