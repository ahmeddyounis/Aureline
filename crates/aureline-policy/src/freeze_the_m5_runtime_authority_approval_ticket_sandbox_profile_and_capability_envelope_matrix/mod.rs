//! Frozen M5 runtime-authority, approval-ticket, sandbox-profile, and
//! capability-envelope matrix for the new M5 executing surfaces.
//!
//! This module locks the canonical M5 runtime-authority qualification for every
//! claimed M5 executing surface — request/API sends, database actions, notebook
//! kernels, scaffold hooks, preview servers, AI tools, recipes, browser-routed
//! actions, incident flows, and remote mutations — into one export-safe packet.
//! Each [`M5RuntimeAuthorityMatrixSurfaceRow`] binds a surface to its
//! qualification class, default sandbox profile, required approval-ticket
//! posture, allowed capability classes, secret scope, degraded fallback,
//! unsupported-profile behavior, ticket expiry, downgrade triggers, source
//! contracts, and consumer-surface parity.
//!
//! The matrix is the single source of truth for whether these executing surfaces
//! may ship as Stable, Beta, or Preview, or must narrow further before command,
//! policy, secret-broker, remote, and help/support teams harden their own
//! incompatible per-surface authority rules. It references the shared
//! runtime-authority, approval-ticket, and secret-handle contracts by id rather
//! than embedding their content. Raw secret material, raw provider payloads,
//! credential bodies, and live ticket signatures stay outside the support
//! boundary.
//!
//! The track invariant is no ambient privilege: no AI tool, extension, recipe,
//! browser route, or remote helper self-issues authority; target identity,
//! sandbox profile, secret scope, policy epoch, expiry, and degraded fallback
//! stay inspectable and export-safe; and if enforcement cannot be honored the
//! surface narrows or fails closed instead of silently widening.
//!
//! The boundary schema is
//! [`schemas/execution-auth/freeze-the-m5-runtime-authority-approval-ticket-sandbox-profile-and-capability-envelope-matrix.schema.json`](../../../../schemas/execution-auth/freeze-the-m5-runtime-authority-approval-ticket-sandbox-profile-and-capability-envelope-matrix.schema.json).
//! The contract doc is
//! [`docs/execution-auth/m5/freeze-the-m5-runtime-authority-approval-ticket-sandbox-profile-and-capability-envelope-matrix.md`](../../../../docs/execution-auth/m5/freeze-the-m5-runtime-authority-approval-ticket-sandbox-profile-and-capability-envelope-matrix.md).
//! The protected fixture directory is
//! [`fixtures/execution-auth/m5/freeze-the-m5-runtime-authority-approval-ticket-sandbox-profile-and-capability-envelope-matrix/`](../../../../fixtures/execution-auth/m5/freeze-the-m5-runtime-authority-approval-ticket-sandbox-profile-and-capability-envelope-matrix/).

#[cfg(test)]
mod tests;

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

/// Stable record-kind tag carried by [`M5RuntimeAuthorityMatrixPacket`].
pub const M5_RUNTIME_AUTHORITY_MATRIX_RECORD_KIND: &str =
    "freeze_m5_runtime_authority_approval_ticket_sandbox_profile_and_capability_envelope_matrix";

/// Schema version for the M5 runtime-authority maturity-matrix records.
pub const M5_RUNTIME_AUTHORITY_MATRIX_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the boundary schema.
pub const M5_RUNTIME_AUTHORITY_MATRIX_SCHEMA_REF: &str =
    "schemas/execution-auth/freeze-the-m5-runtime-authority-approval-ticket-sandbox-profile-and-capability-envelope-matrix.schema.json";

/// Repo-relative path of the M5 runtime-authority maturity-matrix contract doc.
pub const M5_RUNTIME_AUTHORITY_MATRIX_DOC_REF: &str =
    "docs/execution-auth/m5/freeze-the-m5-runtime-authority-approval-ticket-sandbox-profile-and-capability-envelope-matrix.md";

/// Repo-relative path of the frozen runtime-authority issuer contract.
pub const M5_RUNTIME_AUTHORITY_MATRIX_ISSUER_CONTRACT_REF: &str =
    "schemas/security/runtime_authority_issuer.schema.json";

/// Repo-relative path of the frozen authority-ticket contract.
pub const M5_RUNTIME_AUTHORITY_MATRIX_AUTHORITY_TICKET_CONTRACT_REF: &str =
    "schemas/security/authority_ticket.schema.json";

/// Repo-relative path of the frozen approval-ticket contract.
pub const M5_RUNTIME_AUTHORITY_MATRIX_APPROVAL_TICKET_CONTRACT_REF: &str =
    "schemas/security/approval_ticket.schema.json";

/// Repo-relative path of the frozen secret-handle contract.
pub const M5_RUNTIME_AUTHORITY_MATRIX_SECRET_HANDLE_CONTRACT_REF: &str =
    "schemas/security/secret_handle.schema.json";

/// Repo-relative path of the frozen M5 secret-boundary depth contract.
pub const M5_RUNTIME_AUTHORITY_MATRIX_SECRET_BOUNDARY_CONTRACT_REF: &str =
    "schemas/security/m5-secret-boundary-depth.schema.json";

/// Repo-relative path of the protected fixture directory.
pub const M5_RUNTIME_AUTHORITY_MATRIX_FIXTURE_DIR: &str =
    "fixtures/execution-auth/m5/freeze-the-m5-runtime-authority-approval-ticket-sandbox-profile-and-capability-envelope-matrix";

/// Repo-relative path of the checked support-export artifact.
pub const M5_RUNTIME_AUTHORITY_MATRIX_ARTIFACT_REF: &str =
    "artifacts/execution-auth/m5/freeze-the-m5-runtime-authority-approval-ticket-sandbox-profile-and-capability-envelope-matrix/support_export.json";

/// Repo-relative path of the checked Markdown summary.
pub const M5_RUNTIME_AUTHORITY_MATRIX_SUMMARY_REF: &str =
    "artifacts/execution-auth/m5/freeze-the-m5-runtime-authority-approval-ticket-sandbox-profile-and-capability-envelope-matrix.md";

/// Stable packet id minted by [`frozen_stable_m5_runtime_authority_matrix_packet`].
pub const M5_RUNTIME_AUTHORITY_MATRIX_PACKET_ID: &str = "m5-runtime-authority-matrix:stable:0001";

/// One M5 executing surface governed by this runtime-authority matrix.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ExecutingSurface {
    /// Request/API send actions.
    RequestApiSend,
    /// Database read/write actions.
    DatabaseAction,
    /// Notebook execution kernels.
    NotebookKernel,
    /// Scaffold and generator hooks.
    ScaffoldHook,
    /// Local preview servers.
    PreviewServer,
    /// AI tool invocations.
    AiTool,
    /// Saved automation recipes.
    Recipe,
    /// Browser-routed actions.
    BrowserRoutedAction,
    /// Incident response flows.
    IncidentFlow,
    /// Remote mutation actions.
    RemoteMutation,
}

impl M5ExecutingSurface {
    /// Every executing surface, in declaration order.
    pub const ALL: [Self; 10] = [
        Self::RequestApiSend,
        Self::DatabaseAction,
        Self::NotebookKernel,
        Self::ScaffoldHook,
        Self::PreviewServer,
        Self::AiTool,
        Self::Recipe,
        Self::BrowserRoutedAction,
        Self::IncidentFlow,
        Self::RemoteMutation,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::RequestApiSend => "request_api_send",
            Self::DatabaseAction => "database_action",
            Self::NotebookKernel => "notebook_kernel",
            Self::ScaffoldHook => "scaffold_hook",
            Self::PreviewServer => "preview_server",
            Self::AiTool => "ai_tool",
            Self::Recipe => "recipe",
            Self::BrowserRoutedAction => "browser_routed_action",
            Self::IncidentFlow => "incident_flow",
            Self::RemoteMutation => "remote_mutation",
        }
    }

    /// Whether this surface is an untrusted helper (AI, recipe, browser, or
    /// remote) that must never self-issue authority and therefore must require an
    /// externally issued ticket.
    pub const fn is_untrusted_helper(self) -> bool {
        matches!(
            self,
            Self::AiTool | Self::Recipe | Self::BrowserRoutedAction | Self::RemoteMutation
        )
    }
}

/// Qualification class for an M5 executing surface.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5RuntimeAuthorityQualificationClass {
    /// Surface qualifies for the Stable claim.
    Stable,
    /// Surface is narrowed to Beta.
    Beta,
    /// Surface is narrowed to Preview.
    Preview,
    /// Surface is experimental and not claimed.
    Experimental,
    /// Surface is unavailable on this build.
    Unavailable,
    /// Surface is held pending upstream resolution.
    Held,
}

impl M5RuntimeAuthorityQualificationClass {
    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Stable => "stable",
            Self::Beta => "beta",
            Self::Preview => "preview",
            Self::Experimental => "experimental",
            Self::Unavailable => "unavailable",
            Self::Held => "held",
        }
    }

    /// Whether the surface may carry a public Stable claim.
    pub const fn is_stable(self) -> bool {
        matches!(self, Self::Stable)
    }
}

/// Default execution-isolation (sandbox) profile for a surface.
///
/// Profiles are ordered from least to most isolated; an executing surface is
/// pinned to exactly one default profile and never silently widens below it.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5SandboxProfile {
    /// No execution; the surface is inert and never runs code.
    InertNoExecution,
    /// In-process trusted-local execution under the host policy epoch.
    InProcessTrustedLocal,
    /// Network egress brokered through the transport plane; no local process.
    BrokeredNetworkOnly,
    /// Isolated local subprocess with a scoped capability envelope.
    SubprocessIsolatedLocal,
    /// Container-isolated local runtime.
    ContainerIsolatedLocal,
    /// Isolated remote runtime confined to a managed sandbox.
    IsolatedRemoteRuntime,
}

impl M5SandboxProfile {
    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::InertNoExecution => "inert_no_execution",
            Self::InProcessTrustedLocal => "in_process_trusted_local",
            Self::BrokeredNetworkOnly => "brokered_network_only",
            Self::SubprocessIsolatedLocal => "subprocess_isolated_local",
            Self::ContainerIsolatedLocal => "container_isolated_local",
            Self::IsolatedRemoteRuntime => "isolated_remote_runtime",
        }
    }

    /// Whether this profile confines execution to an isolated boundary (rather
    /// than running in-process under ambient host trust).
    pub const fn is_isolated(self) -> bool {
        matches!(
            self,
            Self::BrokeredNetworkOnly
                | Self::SubprocessIsolatedLocal
                | Self::ContainerIsolatedLocal
                | Self::IsolatedRemoteRuntime
                | Self::InertNoExecution
        )
    }
}

/// Required approval-ticket posture for a surface.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ApprovalTicketPosture {
    /// A fresh approval ticket is required for every action.
    TicketRequiredPerAction,
    /// A time-bounded ticket is required per execution session.
    TicketRequiredPerSession,
    /// A scoped capability-envelope ticket is required per target scope.
    TicketRequiredPerScope,
    /// Authority flows from a standing policy-epoch ticket; no ad hoc issuance.
    StandingPolicyTicket,
    /// No ticket is required because the surface is read-only and non-mutating.
    NoTicketRequiredReadOnly,
    /// The surface is blocked from issuing or self-issuing any authority.
    BlockedNoSelfIssue,
}

impl M5ApprovalTicketPosture {
    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::TicketRequiredPerAction => "ticket_required_per_action",
            Self::TicketRequiredPerSession => "ticket_required_per_session",
            Self::TicketRequiredPerScope => "ticket_required_per_scope",
            Self::StandingPolicyTicket => "standing_policy_ticket",
            Self::NoTicketRequiredReadOnly => "no_ticket_required_read_only",
            Self::BlockedNoSelfIssue => "blocked_no_self_issue",
        }
    }

    /// Whether authority under this posture is externally issued (by an approval
    /// ticket or a standing policy epoch) rather than self-asserted by the
    /// executing surface.
    pub const fn is_externally_issued(self) -> bool {
        matches!(
            self,
            Self::TicketRequiredPerAction
                | Self::TicketRequiredPerSession
                | Self::TicketRequiredPerScope
                | Self::StandingPolicyTicket
                | Self::BlockedNoSelfIssue
        )
    }

    /// Whether this posture binds authority to a time-bounded approval ticket
    /// that must carry a non-zero expiry.
    pub const fn is_time_bounded_ticket(self) -> bool {
        matches!(
            self,
            Self::TicketRequiredPerAction
                | Self::TicketRequiredPerSession
                | Self::TicketRequiredPerScope
        )
    }
}

/// One capability class allowed inside a surface's capability envelope.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5CapabilityClass {
    /// Read access to the workspace.
    ReadWorkspace,
    /// Write/mutate access to the workspace.
    WriteWorkspace,
    /// Outbound network egress.
    NetworkEgress,
    /// Spawning local processes.
    ProcessSpawn,
    /// Handle-only secret projection (never raw material).
    SecretHandleProjection,
    /// Database read access.
    DatabaseRead,
    /// Database write/mutation access.
    DatabaseWrite,
    /// Remote-resource mutation.
    RemoteMutation,
    /// Browser navigation and routed page actions.
    BrowserNavigation,
}

impl M5CapabilityClass {
    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ReadWorkspace => "read_workspace",
            Self::WriteWorkspace => "write_workspace",
            Self::NetworkEgress => "network_egress",
            Self::ProcessSpawn => "process_spawn",
            Self::SecretHandleProjection => "secret_handle_projection",
            Self::DatabaseRead => "database_read",
            Self::DatabaseWrite => "database_write",
            Self::RemoteMutation => "remote_mutation",
            Self::BrowserNavigation => "browser_navigation",
        }
    }

    /// Whether this capability is elevated — it mutates state, spawns processes,
    /// egresses the network, or projects a secret — and therefore must be gated
    /// behind an externally issued approval ticket.
    pub const fn is_elevated(self) -> bool {
        matches!(
            self,
            Self::WriteWorkspace
                | Self::NetworkEgress
                | Self::ProcessSpawn
                | Self::SecretHandleProjection
                | Self::DatabaseWrite
                | Self::RemoteMutation
        )
    }

    /// Whether this capability projects secret material and therefore requires a
    /// non-empty secret scope.
    pub const fn requires_secret_scope(self) -> bool {
        matches!(self, Self::SecretHandleProjection)
    }
}

/// Secret-scope posture for a surface's capability envelope.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5SecretScope {
    /// The surface has no access to any secret material.
    NoSecretAccess,
    /// The surface receives handle-only, delegated secret references.
    HandleOnlyDelegated,
    /// The surface receives a scoped, broker-minted secret reference.
    ScopedBrokeredSecret,
}

impl M5SecretScope {
    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NoSecretAccess => "no_secret_access",
            Self::HandleOnlyDelegated => "handle_only_delegated",
            Self::ScopedBrokeredSecret => "scoped_brokered_secret",
        }
    }

    /// Whether this scope grants any secret access.
    pub const fn grants_secret_access(self) -> bool {
        !matches!(self, Self::NoSecretAccess)
    }
}

/// Degraded fallback applied when a surface cannot run at full authority.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5DegradedFallback {
    /// Narrow the surface to read-only.
    NarrowToReadOnly,
    /// Narrow the surface to a sanitized dry-run preview.
    NarrowToSanitizedPreview,
    /// Require a fresh ticket before any further action.
    RequireFreshTicket,
    /// Fall back to offline local-core-only behavior.
    OfflineLocalCoreOnly,
    /// Fail closed and block the action.
    FailClosedBlock,
}

impl M5DegradedFallback {
    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NarrowToReadOnly => "narrow_to_read_only",
            Self::NarrowToSanitizedPreview => "narrow_to_sanitized_preview",
            Self::RequireFreshTicket => "require_fresh_ticket",
            Self::OfflineLocalCoreOnly => "offline_local_core_only",
            Self::FailClosedBlock => "fail_closed_block",
        }
    }
}

/// Behavior when the surface's default sandbox profile is unsupported on the
/// running platform.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5UnsupportedProfileBehavior {
    /// Fail closed and block; the surface is unavailable on this platform.
    FailClosedUnsupported,
    /// Narrow to a stricter, supported profile.
    NarrowToStricterProfile,
    /// Disable the surface entirely with an explicit, inspectable reason.
    DisableSurfaceWithReason,
    /// The default profile is supported on every platform.
    FullySupportedAllPlatforms,
}

impl M5UnsupportedProfileBehavior {
    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::FailClosedUnsupported => "fail_closed_unsupported",
            Self::NarrowToStricterProfile => "narrow_to_stricter_profile",
            Self::DisableSurfaceWithReason => "disable_surface_with_reason",
            Self::FullySupportedAllPlatforms => "fully_supported_all_platforms",
        }
    }
}

/// Downgrade trigger that can narrow a surface below its claimed qualification.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5RuntimeAuthorityDowngradeTrigger {
    /// The approval ticket has expired.
    ApprovalTicketExpired,
    /// A replayed approval ticket was detected.
    ApprovalTicketReplayed,
    /// The default sandbox profile is unavailable on this platform.
    SandboxProfileUnavailable,
    /// The governing policy epoch was superseded.
    PolicyEpochSuperseded,
    /// The secret broker is unavailable.
    SecretBrokerUnavailable,
    /// The required enforcement backend is missing.
    EnforcementBackendMissing,
    /// The execution target identity could not be verified.
    TargetIdentityUnverified,
    /// Scope expanded beyond the qualified capability envelope.
    ScopeExpansionUnqualified,
    /// Ambient machine privilege was detected.
    AmbientPrivilegeDetected,
    /// An upstream dependency lane narrowed.
    UpstreamDependencyNarrowed,
}

impl M5RuntimeAuthorityDowngradeTrigger {
    /// Every trigger, in declaration order.
    pub const ALL: [Self; 10] = [
        Self::ApprovalTicketExpired,
        Self::ApprovalTicketReplayed,
        Self::SandboxProfileUnavailable,
        Self::PolicyEpochSuperseded,
        Self::SecretBrokerUnavailable,
        Self::EnforcementBackendMissing,
        Self::TargetIdentityUnverified,
        Self::ScopeExpansionUnqualified,
        Self::AmbientPrivilegeDetected,
        Self::UpstreamDependencyNarrowed,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ApprovalTicketExpired => "approval_ticket_expired",
            Self::ApprovalTicketReplayed => "approval_ticket_replayed",
            Self::SandboxProfileUnavailable => "sandbox_profile_unavailable",
            Self::PolicyEpochSuperseded => "policy_epoch_superseded",
            Self::SecretBrokerUnavailable => "secret_broker_unavailable",
            Self::EnforcementBackendMissing => "enforcement_backend_missing",
            Self::TargetIdentityUnverified => "target_identity_unverified",
            Self::ScopeExpansionUnqualified => "scope_expansion_unqualified",
            Self::AmbientPrivilegeDetected => "ambient_privilege_detected",
            Self::UpstreamDependencyNarrowed => "upstream_dependency_narrowed",
        }
    }
}

/// Consumer surface that must project a surface's runtime-authority truth.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5RuntimeAuthorityConsumerSurface {
    /// Desktop shell trust prompt and authority inspector.
    DesktopShell,
    /// Command palette / command parity surface.
    CommandPalette,
    /// Policy inspector.
    PolicyInspector,
    /// CLI / headless replay or JSON output.
    CliHeadless,
    /// Support / export packet.
    SupportExport,
    /// Diagnostics or telemetry surface.
    Diagnostics,
    /// Help / About surface.
    HelpAbout,
    /// Release evidence review.
    ReleaseEvidence,
}

impl M5RuntimeAuthorityConsumerSurface {
    /// Every surface, in declaration order.
    pub const ALL: [Self; 8] = [
        Self::DesktopShell,
        Self::CommandPalette,
        Self::PolicyInspector,
        Self::CliHeadless,
        Self::SupportExport,
        Self::Diagnostics,
        Self::HelpAbout,
        Self::ReleaseEvidence,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::DesktopShell => "desktop_shell",
            Self::CommandPalette => "command_palette",
            Self::PolicyInspector => "policy_inspector",
            Self::CliHeadless => "cli_headless",
            Self::SupportExport => "support_export",
            Self::Diagnostics => "diagnostics",
            Self::HelpAbout => "help_about",
            Self::ReleaseEvidence => "release_evidence",
        }
    }
}

/// One row in the M5 runtime-authority maturity matrix.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5RuntimeAuthorityMatrixSurfaceRow {
    /// Executing surface.
    pub surface: M5ExecutingSurface,
    /// Qualification class earned by this surface.
    pub qualification: M5RuntimeAuthorityQualificationClass,
    /// Human-readable scope summary.
    pub scope_summary: String,
    /// Default execution-isolation (sandbox) profile.
    pub default_sandbox_profile: M5SandboxProfile,
    /// Required approval-ticket posture.
    pub approval_ticket_posture: M5ApprovalTicketPosture,
    /// Allowed capability classes inside the envelope.
    pub allowed_capability_classes: Vec<M5CapabilityClass>,
    /// Secret-scope posture.
    pub secret_scope: M5SecretScope,
    /// Degraded fallback when full authority cannot be honored.
    pub degraded_fallback: M5DegradedFallback,
    /// Behavior when the default sandbox profile is unsupported on this platform.
    pub unsupported_profile_behavior: M5UnsupportedProfileBehavior,
    /// Approval-ticket expiry in seconds; zero when not a time-bounded ticket.
    pub ticket_expiry_seconds: u32,
    /// Required evidence packet refs for this qualification.
    pub required_evidence_packet_refs: Vec<String>,
    /// Downgrade triggers that apply to this surface.
    pub downgrade_triggers: Vec<M5RuntimeAuthorityDowngradeTrigger>,
    /// Source contract refs consumed by this surface.
    pub source_contract_refs: Vec<String>,
    /// Consumer surfaces that must project this surface's authority truth.
    pub consumer_surfaces: Vec<M5RuntimeAuthorityConsumerSurface>,
}

/// Runtime-authority trust and isolation review block.
///
/// Every field encodes a hard invariant; all must hold for the matrix to
/// validate.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5RuntimeAuthorityMatrixTrustReview {
    /// No surface is granted ambient machine privilege.
    pub no_ambient_machine_privilege: bool,
    /// No AI tool, extension, recipe, browser route, or remote helper self-issues authority.
    pub no_self_issued_authority_by_helpers: bool,
    /// Target identity is inspectable and export-safe on every surface.
    pub target_identity_inspectable_and_export_safe: bool,
    /// Sandbox profile is inspectable on every surface.
    pub sandbox_profile_inspectable: bool,
    /// Secret scope is handle-only; no raw secret material is projected.
    pub secret_scope_handle_only_no_raw_material: bool,
    /// Policy epoch and expiry are inspectable on every surface.
    pub policy_epoch_and_expiry_inspectable: bool,
    /// Degraded fallback is inspectable on every surface.
    pub degraded_fallback_inspectable: bool,
    /// Enforcement fails closed when it cannot be honored, never silently widening.
    pub fail_closed_when_enforcement_unavailable: bool,
    /// A missing enforcement backend automatically narrows the claim.
    pub missing_enforcement_backend_auto_narrows: bool,
    /// No raw secret material is exported inside envelopes, tickets, or support packets.
    pub no_raw_secret_material_in_exports: bool,
}

/// Consumer projection block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5RuntimeAuthorityMatrixConsumerProjection {
    /// Desktop shell shows the full authority envelope (target, profile, ticket, scope, expiry).
    pub desktop_shows_authority_envelope: bool,
    /// Command palette and policy inspector reference the same matrix.
    pub command_and_policy_reference_same_matrix: bool,
    /// CLI / headless shows the authority envelope and qualification.
    pub cli_headless_shows_authority_envelope: bool,
    /// Support export shows the authority envelope and qualification.
    pub support_export_shows_authority_envelope: bool,
    /// Diagnostics shows the authority envelope and qualification.
    pub diagnostics_shows_authority_envelope: bool,
    /// Help / About shows qualification truth.
    pub help_about_shows_qualification: bool,
    /// Release evidence consumes this matrix instead of cloning per-surface prose.
    pub release_evidence_consumes_matrix: bool,
    /// Preview / Labs surfaces are visibly labeled when not covered by this packet.
    pub preview_labs_label_for_unqualified_surfaces: bool,
}

/// Proof freshness block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5RuntimeAuthorityMatrixProofFreshness {
    /// Proof-freshness SLO in hours.
    pub proof_freshness_slo_hours: u32,
    /// RFC 3339 timestamp of the last proof refresh.
    pub last_proof_refresh: String,
    /// True when stale proof automatically narrows the surface.
    pub auto_narrow_on_stale: bool,
}

/// Constructor input for [`M5RuntimeAuthorityMatrixPacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M5RuntimeAuthorityMatrixPacketInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable matrix label.
    pub matrix_label: String,
    /// Surface rows.
    pub surface_rows: Vec<M5RuntimeAuthorityMatrixSurfaceRow>,
    /// Trust review block.
    pub trust_review: M5RuntimeAuthorityMatrixTrustReview,
    /// Consumer projection block.
    pub consumer_projection: M5RuntimeAuthorityMatrixConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: M5RuntimeAuthorityMatrixProofFreshness,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

/// Export-safe frozen M5 runtime-authority maturity-matrix packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5RuntimeAuthorityMatrixPacket {
    /// Record kind; must equal [`M5_RUNTIME_AUTHORITY_MATRIX_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`M5_RUNTIME_AUTHORITY_MATRIX_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable matrix label.
    pub matrix_label: String,
    /// Surface rows.
    pub surface_rows: Vec<M5RuntimeAuthorityMatrixSurfaceRow>,
    /// Trust review block.
    pub trust_review: M5RuntimeAuthorityMatrixTrustReview,
    /// Consumer projection block.
    pub consumer_projection: M5RuntimeAuthorityMatrixConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: M5RuntimeAuthorityMatrixProofFreshness,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

impl M5RuntimeAuthorityMatrixPacket {
    /// Builds an M5 runtime-authority maturity-matrix packet from frozen input.
    pub fn new(input: M5RuntimeAuthorityMatrixPacketInput) -> Self {
        Self {
            record_kind: M5_RUNTIME_AUTHORITY_MATRIX_RECORD_KIND.to_owned(),
            schema_version: M5_RUNTIME_AUTHORITY_MATRIX_SCHEMA_VERSION,
            packet_id: input.packet_id,
            matrix_label: input.matrix_label,
            surface_rows: input.surface_rows,
            trust_review: input.trust_review,
            consumer_projection: input.consumer_projection,
            proof_freshness: input.proof_freshness,
            source_contract_refs: input.source_contract_refs,
            redaction_class_token: input.redaction_class_token,
            minted_at: input.minted_at,
        }
    }

    /// Validates the M5 runtime-authority maturity-matrix invariants.
    pub fn validate(&self) -> Vec<M5RuntimeAuthorityMatrixViolation> {
        let mut violations = Vec::new();

        if self.record_kind != M5_RUNTIME_AUTHORITY_MATRIX_RECORD_KIND {
            violations.push(M5RuntimeAuthorityMatrixViolation::WrongRecordKind);
        }
        if self.schema_version != M5_RUNTIME_AUTHORITY_MATRIX_SCHEMA_VERSION {
            violations.push(M5RuntimeAuthorityMatrixViolation::WrongSchemaVersion);
        }
        if self.packet_id.trim().is_empty()
            || self.matrix_label.trim().is_empty()
            || self.redaction_class_token.trim().is_empty()
            || self.minted_at.trim().is_empty()
        {
            violations.push(M5RuntimeAuthorityMatrixViolation::MissingIdentity);
        }

        validate_source_contracts(self, &mut violations);
        validate_surface_rows(self, &mut violations);
        validate_trust_review(self, &mut violations);
        validate_consumer_projection(self, &mut violations);
        validate_proof_freshness(self, &mut violations);

        if json_contains_forbidden_boundary_material(
            &serde_json::to_value(self).expect("m5 runtime-authority matrix packet serializes"),
        ) {
            violations.push(M5RuntimeAuthorityMatrixViolation::RawBoundaryMaterialInExport);
        }

        violations
    }

    /// Deterministic export-safe JSON.
    ///
    /// # Panics
    ///
    /// Panics only if serializing this metadata-only packet fails.
    pub fn export_safe_json(&self) -> String {
        serde_json::to_string_pretty(self).expect("m5 runtime-authority matrix packet serializes")
    }

    /// Deterministic Markdown summary for support, review, or release handoff.
    pub fn render_markdown_summary(&self) -> String {
        let stable_surfaces = self
            .surface_rows
            .iter()
            .filter(|row| row.qualification.is_stable())
            .count();
        let mut out = String::new();
        out.push_str(
            "# M5 Runtime-Authority, Approval-Ticket, Sandbox-Profile, and Capability-Envelope Matrix\n\n",
        );
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- Label: `{}`\n", self.matrix_label));
        out.push_str(&format!(
            "- Surfaces: {} ({} stable)\n",
            self.surface_rows.len(),
            stable_surfaces
        ));
        out.push_str(&format!(
            "- Proof freshness SLO: {} hours (last refresh: {})\n",
            self.proof_freshness.proof_freshness_slo_hours, self.proof_freshness.last_proof_refresh
        ));
        out.push_str("\n## Executing surfaces\n\n");
        for row in &self.surface_rows {
            out.push_str(&format!(
                "- **{}**: `{}`\n",
                row.surface.as_str(),
                row.qualification.as_str()
            ));
            out.push_str(&format!("  - Scope: {}\n", row.scope_summary));
            out.push_str(&format!(
                "  - Sandbox: {} · Ticket: {} (expiry {}s)\n",
                row.default_sandbox_profile.as_str(),
                row.approval_ticket_posture.as_str(),
                row.ticket_expiry_seconds
            ));
            out.push_str(&format!(
                "  - Secret scope: {} · Degraded: {} · Unsupported: {}\n",
                row.secret_scope.as_str(),
                row.degraded_fallback.as_str(),
                row.unsupported_profile_behavior.as_str()
            ));
            let caps = row
                .allowed_capability_classes
                .iter()
                .map(|c| c.as_str())
                .collect::<Vec<_>>()
                .join(", ");
            out.push_str(&format!("  - Capabilities: {caps}\n"));
        }
        out
    }
}

/// Errors emitted when reading the checked-in M5 runtime-authority matrix export.
#[derive(Debug)]
pub enum M5RuntimeAuthorityMatrixArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<M5RuntimeAuthorityMatrixViolation>),
}

impl fmt::Display for M5RuntimeAuthorityMatrixArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(
                    formatter,
                    "m5 runtime-authority matrix export parse failed: {error}"
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
                    "m5 runtime-authority matrix export failed validation: {tokens}"
                )
            }
        }
    }
}

impl Error for M5RuntimeAuthorityMatrixArtifactError {}

/// Validation failures emitted by [`M5RuntimeAuthorityMatrixPacket::validate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum M5RuntimeAuthorityMatrixViolation {
    /// Packet record kind is wrong.
    WrongRecordKind,
    /// Packet schema version is wrong.
    WrongSchemaVersion,
    /// Required identity field is missing.
    MissingIdentity,
    /// Source contract refs are incomplete.
    MissingSourceContracts,
    /// A required executing surface is missing from the matrix.
    RequiredSurfaceMissing,
    /// A surface row is incomplete.
    SurfaceRowIncomplete,
    /// A surface claiming Stable is missing required evidence packet refs.
    StableSurfaceMissingEvidence,
    /// A surface has no allowed capability classes.
    CapabilityEnvelopeEmpty,
    /// A surface has no downgrade triggers.
    DowngradeTriggersMissing,
    /// A surface has no consumer surfaces.
    ConsumerSurfacesMissing,
    /// An untrusted helper surface self-issues authority instead of requiring a ticket.
    SelfIssuedAuthorityForbidden,
    /// A surface allows an elevated capability without an externally issued ticket.
    ElevatedCapabilityWithoutTicket,
    /// A time-bounded approval-ticket posture carries no expiry.
    TicketExpiryMissing,
    /// A secret-projecting capability is declared without a secret scope.
    SecretScopeInconsistent,
    /// Trust review does not satisfy required invariants.
    TrustReviewIncomplete,
    /// Consumer projection does not satisfy required invariants.
    ConsumerProjectionIncomplete,
    /// Proof freshness block is incomplete.
    ProofFreshnessIncomplete,
    /// Export contains raw boundary material.
    RawBoundaryMaterialInExport,
}

impl M5RuntimeAuthorityMatrixViolation {
    /// Stable token used in tests and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WrongRecordKind => "wrong_record_kind",
            Self::WrongSchemaVersion => "wrong_schema_version",
            Self::MissingIdentity => "missing_identity",
            Self::MissingSourceContracts => "missing_source_contracts",
            Self::RequiredSurfaceMissing => "required_surface_missing",
            Self::SurfaceRowIncomplete => "surface_row_incomplete",
            Self::StableSurfaceMissingEvidence => "stable_surface_missing_evidence",
            Self::CapabilityEnvelopeEmpty => "capability_envelope_empty",
            Self::DowngradeTriggersMissing => "downgrade_triggers_missing",
            Self::ConsumerSurfacesMissing => "consumer_surfaces_missing",
            Self::SelfIssuedAuthorityForbidden => "self_issued_authority_forbidden",
            Self::ElevatedCapabilityWithoutTicket => "elevated_capability_without_ticket",
            Self::TicketExpiryMissing => "ticket_expiry_missing",
            Self::SecretScopeInconsistent => "secret_scope_inconsistent",
            Self::TrustReviewIncomplete => "trust_review_incomplete",
            Self::ConsumerProjectionIncomplete => "consumer_projection_incomplete",
            Self::ProofFreshnessIncomplete => "proof_freshness_incomplete",
            Self::RawBoundaryMaterialInExport => "raw_boundary_material_in_export",
        }
    }
}

/// Builds the canonical frozen stable M5 runtime-authority matrix packet.
///
/// This is the single in-code source of truth for the checked-in support export
/// at [`M5_RUNTIME_AUTHORITY_MATRIX_ARTIFACT_REF`]; the matrix dumper emits this
/// packet and a test asserts the checked-in artifact deserializes back to it
/// unchanged.
pub fn frozen_stable_m5_runtime_authority_matrix_packet() -> M5RuntimeAuthorityMatrixPacket {
    use M5ApprovalTicketPosture as Ticket;
    use M5CapabilityClass as Cap;
    use M5DegradedFallback as Degraded;
    use M5ExecutingSurface as Surface;
    use M5RuntimeAuthorityConsumerSurface as Consumer;
    use M5RuntimeAuthorityDowngradeTrigger as Trigger;
    use M5RuntimeAuthorityQualificationClass as Qual;
    use M5SandboxProfile as Sandbox;
    use M5SecretScope as Secret;
    use M5UnsupportedProfileBehavior as Unsupported;

    let issuer_contract = M5_RUNTIME_AUTHORITY_MATRIX_ISSUER_CONTRACT_REF.to_owned();
    let authority_ticket_contract =
        M5_RUNTIME_AUTHORITY_MATRIX_AUTHORITY_TICKET_CONTRACT_REF.to_owned();
    let approval_ticket_contract =
        M5_RUNTIME_AUTHORITY_MATRIX_APPROVAL_TICKET_CONTRACT_REF.to_owned();
    let secret_handle_contract = M5_RUNTIME_AUTHORITY_MATRIX_SECRET_HANDLE_CONTRACT_REF.to_owned();

    let surface_rows = vec![
        M5RuntimeAuthorityMatrixSurfaceRow {
            surface: Surface::RequestApiSend,
            qualification: Qual::Stable,
            scope_summary: "Request/API sends run with no local process under a brokered network-only profile; each send carries a scoped capability-envelope ticket, projects secrets handle-only through the broker, and narrows to a fresh-ticket prompt rather than reusing stale authority".to_owned(),
            default_sandbox_profile: Sandbox::BrokeredNetworkOnly,
            approval_ticket_posture: Ticket::TicketRequiredPerScope,
            allowed_capability_classes: vec![Cap::NetworkEgress, Cap::SecretHandleProjection],
            secret_scope: Secret::HandleOnlyDelegated,
            degraded_fallback: Degraded::RequireFreshTicket,
            unsupported_profile_behavior: Unsupported::NarrowToStricterProfile,
            ticket_expiry_seconds: 3600,
            required_evidence_packet_refs: vec!["evidence:request-api-send-capability-envelope:m5".to_owned()],
            downgrade_triggers: vec![Trigger::ApprovalTicketExpired, Trigger::SecretBrokerUnavailable, Trigger::PolicyEpochSuperseded, Trigger::TargetIdentityUnverified],
            source_contract_refs: vec![approval_ticket_contract.clone(), secret_handle_contract.clone()],
            consumer_surfaces: vec![Consumer::DesktopShell, Consumer::CommandPalette, Consumer::CliHeadless, Consumer::SupportExport, Consumer::Diagnostics],
        },
        M5RuntimeAuthorityMatrixSurfaceRow {
            surface: Surface::DatabaseAction,
            qualification: Qual::Beta,
            scope_summary: "Database actions run through a brokered network-only profile; reads use a standing scope ticket while writes require a fresh per-action ticket, secrets stay handle-only, and the surface narrows to read-only when the write ticket cannot be honored".to_owned(),
            default_sandbox_profile: Sandbox::BrokeredNetworkOnly,
            approval_ticket_posture: Ticket::TicketRequiredPerAction,
            allowed_capability_classes: vec![Cap::DatabaseRead, Cap::DatabaseWrite, Cap::NetworkEgress, Cap::SecretHandleProjection],
            secret_scope: Secret::HandleOnlyDelegated,
            degraded_fallback: Degraded::NarrowToReadOnly,
            unsupported_profile_behavior: Unsupported::NarrowToStricterProfile,
            ticket_expiry_seconds: 900,
            required_evidence_packet_refs: vec!["evidence:database-action-write-ticket:m5".to_owned()],
            downgrade_triggers: vec![Trigger::ApprovalTicketExpired, Trigger::ApprovalTicketReplayed, Trigger::SecretBrokerUnavailable, Trigger::TargetIdentityUnverified],
            source_contract_refs: vec![approval_ticket_contract.clone(), secret_handle_contract.clone()],
            consumer_surfaces: vec![Consumer::DesktopShell, Consumer::PolicyInspector, Consumer::CliHeadless, Consumer::SupportExport, Consumer::Diagnostics],
        },
        M5RuntimeAuthorityMatrixSurfaceRow {
            surface: Surface::NotebookKernel,
            qualification: Qual::Beta,
            scope_summary: "Notebook kernels run in an isolated local subprocess with a per-session ticket; the envelope grants scoped workspace read/write, process spawn, and brokered network, with no secret access, and narrows to read-only when the sandbox profile is unavailable".to_owned(),
            default_sandbox_profile: Sandbox::SubprocessIsolatedLocal,
            approval_ticket_posture: Ticket::TicketRequiredPerSession,
            allowed_capability_classes: vec![Cap::ReadWorkspace, Cap::WriteWorkspace, Cap::ProcessSpawn, Cap::NetworkEgress],
            secret_scope: Secret::NoSecretAccess,
            degraded_fallback: Degraded::NarrowToReadOnly,
            unsupported_profile_behavior: Unsupported::NarrowToStricterProfile,
            ticket_expiry_seconds: 7200,
            required_evidence_packet_refs: vec!["evidence:notebook-kernel-isolation:m5".to_owned()],
            downgrade_triggers: vec![Trigger::ApprovalTicketExpired, Trigger::SandboxProfileUnavailable, Trigger::EnforcementBackendMissing, Trigger::ScopeExpansionUnqualified],
            source_contract_refs: vec![issuer_contract.clone(), authority_ticket_contract.clone()],
            consumer_surfaces: vec![Consumer::DesktopShell, Consumer::CliHeadless, Consumer::SupportExport, Consumer::Diagnostics],
        },
        M5RuntimeAuthorityMatrixSurfaceRow {
            surface: Surface::ScaffoldHook,
            qualification: Qual::Beta,
            scope_summary: "Scaffold hooks run in an isolated local subprocess with a fresh per-action ticket; the envelope grants scoped workspace read/write and process spawn, holds no secrets, and narrows to a sanitized dry-run preview when the hook cannot be confined".to_owned(),
            default_sandbox_profile: Sandbox::SubprocessIsolatedLocal,
            approval_ticket_posture: Ticket::TicketRequiredPerAction,
            allowed_capability_classes: vec![Cap::ReadWorkspace, Cap::WriteWorkspace, Cap::ProcessSpawn],
            secret_scope: Secret::NoSecretAccess,
            degraded_fallback: Degraded::NarrowToSanitizedPreview,
            unsupported_profile_behavior: Unsupported::NarrowToStricterProfile,
            ticket_expiry_seconds: 600,
            required_evidence_packet_refs: vec!["evidence:scaffold-hook-dry-run-preview:m5".to_owned()],
            downgrade_triggers: vec![Trigger::ApprovalTicketExpired, Trigger::SandboxProfileUnavailable, Trigger::EnforcementBackendMissing, Trigger::ScopeExpansionUnqualified],
            source_contract_refs: vec![issuer_contract.clone(), authority_ticket_contract.clone()],
            consumer_surfaces: vec![Consumer::DesktopShell, Consumer::CliHeadless, Consumer::SupportExport, Consumer::Diagnostics],
        },
        M5RuntimeAuthorityMatrixSurfaceRow {
            surface: Surface::PreviewServer,
            qualification: Qual::Beta,
            scope_summary: "Preview servers run in a container-isolated local runtime with a per-session ticket; the envelope grants scoped workspace read, process spawn, and brokered network, holds no secrets, and narrows to read-only when isolation is unavailable".to_owned(),
            default_sandbox_profile: Sandbox::ContainerIsolatedLocal,
            approval_ticket_posture: Ticket::TicketRequiredPerSession,
            allowed_capability_classes: vec![Cap::ReadWorkspace, Cap::ProcessSpawn, Cap::NetworkEgress],
            secret_scope: Secret::NoSecretAccess,
            degraded_fallback: Degraded::NarrowToReadOnly,
            unsupported_profile_behavior: Unsupported::NarrowToStricterProfile,
            ticket_expiry_seconds: 7200,
            required_evidence_packet_refs: vec!["evidence:preview-server-container-isolation:m5".to_owned()],
            downgrade_triggers: vec![Trigger::ApprovalTicketExpired, Trigger::SandboxProfileUnavailable, Trigger::EnforcementBackendMissing, Trigger::ScopeExpansionUnqualified],
            source_contract_refs: vec![issuer_contract.clone(), authority_ticket_contract.clone()],
            consumer_surfaces: vec![Consumer::DesktopShell, Consumer::CliHeadless, Consumer::SupportExport, Consumer::Diagnostics],
        },
        M5RuntimeAuthorityMatrixSurfaceRow {
            surface: Surface::AiTool,
            qualification: Qual::Beta,
            scope_summary: "AI tools never self-issue authority; every invocation runs in an isolated local subprocess behind a fresh per-action ticket with a scoped workspace and brokered-network envelope, handle-only secrets, and narrows to a sanitized preview rather than acting on stale or unverified authority".to_owned(),
            default_sandbox_profile: Sandbox::SubprocessIsolatedLocal,
            approval_ticket_posture: Ticket::TicketRequiredPerAction,
            allowed_capability_classes: vec![Cap::ReadWorkspace, Cap::WriteWorkspace, Cap::NetworkEgress, Cap::SecretHandleProjection],
            secret_scope: Secret::HandleOnlyDelegated,
            degraded_fallback: Degraded::NarrowToSanitizedPreview,
            unsupported_profile_behavior: Unsupported::NarrowToStricterProfile,
            ticket_expiry_seconds: 300,
            required_evidence_packet_refs: vec!["evidence:ai-tool-no-self-issue:m5".to_owned()],
            downgrade_triggers: vec![Trigger::ApprovalTicketExpired, Trigger::AmbientPrivilegeDetected, Trigger::SecretBrokerUnavailable, Trigger::ScopeExpansionUnqualified],
            source_contract_refs: vec![authority_ticket_contract.clone(), secret_handle_contract.clone()],
            consumer_surfaces: vec![Consumer::DesktopShell, Consumer::PolicyInspector, Consumer::CliHeadless, Consumer::SupportExport, Consumer::Diagnostics],
        },
        M5RuntimeAuthorityMatrixSurfaceRow {
            surface: Surface::Recipe,
            qualification: Qual::Beta,
            scope_summary: "Recipes never self-issue authority; each run binds to a scoped capability-envelope ticket inside an isolated local subprocess granting scoped workspace read/write and process spawn, holds no secrets, and narrows to a sanitized preview when the envelope cannot be enforced".to_owned(),
            default_sandbox_profile: Sandbox::SubprocessIsolatedLocal,
            approval_ticket_posture: Ticket::TicketRequiredPerScope,
            allowed_capability_classes: vec![Cap::ReadWorkspace, Cap::WriteWorkspace, Cap::ProcessSpawn],
            secret_scope: Secret::NoSecretAccess,
            degraded_fallback: Degraded::NarrowToSanitizedPreview,
            unsupported_profile_behavior: Unsupported::NarrowToStricterProfile,
            ticket_expiry_seconds: 1800,
            required_evidence_packet_refs: vec!["evidence:recipe-scoped-envelope:m5".to_owned()],
            downgrade_triggers: vec![Trigger::ApprovalTicketExpired, Trigger::AmbientPrivilegeDetected, Trigger::EnforcementBackendMissing, Trigger::ScopeExpansionUnqualified],
            source_contract_refs: vec![authority_ticket_contract.clone(), issuer_contract.clone()],
            consumer_surfaces: vec![Consumer::DesktopShell, Consumer::CommandPalette, Consumer::CliHeadless, Consumer::SupportExport, Consumer::Diagnostics],
        },
        M5RuntimeAuthorityMatrixSurfaceRow {
            surface: Surface::BrowserRoutedAction,
            qualification: Qual::Preview,
            scope_summary: "Browser-routed actions never self-issue authority; each routed action runs in an isolated remote runtime behind a fresh per-action ticket with a navigation-and-brokered-network envelope, holds no secrets, and fails closed when the isolated runtime is unavailable on this platform".to_owned(),
            default_sandbox_profile: Sandbox::IsolatedRemoteRuntime,
            approval_ticket_posture: Ticket::TicketRequiredPerAction,
            allowed_capability_classes: vec![Cap::BrowserNavigation, Cap::NetworkEgress],
            secret_scope: Secret::NoSecretAccess,
            degraded_fallback: Degraded::NarrowToReadOnly,
            unsupported_profile_behavior: Unsupported::FailClosedUnsupported,
            ticket_expiry_seconds: 300,
            required_evidence_packet_refs: vec!["evidence:browser-routed-isolation:m5".to_owned()],
            downgrade_triggers: vec![Trigger::ApprovalTicketExpired, Trigger::SandboxProfileUnavailable, Trigger::AmbientPrivilegeDetected, Trigger::ScopeExpansionUnqualified],
            source_contract_refs: vec![authority_ticket_contract.clone(), issuer_contract.clone()],
            consumer_surfaces: vec![Consumer::DesktopShell, Consumer::CliHeadless, Consumer::SupportExport, Consumer::HelpAbout],
        },
        M5RuntimeAuthorityMatrixSurfaceRow {
            surface: Surface::IncidentFlow,
            qualification: Qual::Beta,
            scope_summary: "Incident flows run under a brokered network-only profile with a scoped capability-envelope ticket; the envelope grants scoped workspace read and brokered network with handle-only secrets, and narrows to read-only when the policy epoch or ticket cannot be honored".to_owned(),
            default_sandbox_profile: Sandbox::BrokeredNetworkOnly,
            approval_ticket_posture: Ticket::TicketRequiredPerScope,
            allowed_capability_classes: vec![Cap::ReadWorkspace, Cap::NetworkEgress, Cap::SecretHandleProjection],
            secret_scope: Secret::HandleOnlyDelegated,
            degraded_fallback: Degraded::NarrowToReadOnly,
            unsupported_profile_behavior: Unsupported::NarrowToStricterProfile,
            ticket_expiry_seconds: 3600,
            required_evidence_packet_refs: vec!["evidence:incident-flow-scoped-envelope:m5".to_owned()],
            downgrade_triggers: vec![Trigger::ApprovalTicketExpired, Trigger::PolicyEpochSuperseded, Trigger::SecretBrokerUnavailable, Trigger::TargetIdentityUnverified],
            source_contract_refs: vec![approval_ticket_contract.clone(), secret_handle_contract.clone()],
            consumer_surfaces: vec![Consumer::DesktopShell, Consumer::PolicyInspector, Consumer::CliHeadless, Consumer::SupportExport, Consumer::Diagnostics],
        },
        M5RuntimeAuthorityMatrixSurfaceRow {
            surface: Surface::RemoteMutation,
            qualification: Qual::Preview,
            scope_summary: "Remote mutations are the highest-risk surface and never self-issue authority; each mutation runs in an isolated remote runtime behind a fresh per-action ticket with a scoped broker-minted secret, fails closed and blocks rather than degrading, and is unavailable when the isolated runtime is unsupported on this platform".to_owned(),
            default_sandbox_profile: Sandbox::IsolatedRemoteRuntime,
            approval_ticket_posture: Ticket::TicketRequiredPerAction,
            allowed_capability_classes: vec![Cap::RemoteMutation, Cap::NetworkEgress, Cap::SecretHandleProjection],
            secret_scope: Secret::ScopedBrokeredSecret,
            degraded_fallback: Degraded::FailClosedBlock,
            unsupported_profile_behavior: Unsupported::FailClosedUnsupported,
            ticket_expiry_seconds: 300,
            required_evidence_packet_refs: vec!["evidence:remote-mutation-fail-closed:m5".to_owned()],
            downgrade_triggers: vec![Trigger::ApprovalTicketExpired, Trigger::ApprovalTicketReplayed, Trigger::SandboxProfileUnavailable, Trigger::SecretBrokerUnavailable, Trigger::AmbientPrivilegeDetected],
            source_contract_refs: vec![authority_ticket_contract.clone(), secret_handle_contract.clone()],
            consumer_surfaces: vec![Consumer::DesktopShell, Consumer::PolicyInspector, Consumer::CliHeadless, Consumer::SupportExport, Consumer::ReleaseEvidence],
        },
    ];

    M5RuntimeAuthorityMatrixPacket::new(M5RuntimeAuthorityMatrixPacketInput {
        packet_id: M5_RUNTIME_AUTHORITY_MATRIX_PACKET_ID.to_owned(),
        matrix_label:
            "M5 Runtime-Authority, Approval-Ticket, Sandbox-Profile, and Capability-Envelope Matrix"
                .to_owned(),
        surface_rows,
        trust_review: M5RuntimeAuthorityMatrixTrustReview {
            no_ambient_machine_privilege: true,
            no_self_issued_authority_by_helpers: true,
            target_identity_inspectable_and_export_safe: true,
            sandbox_profile_inspectable: true,
            secret_scope_handle_only_no_raw_material: true,
            policy_epoch_and_expiry_inspectable: true,
            degraded_fallback_inspectable: true,
            fail_closed_when_enforcement_unavailable: true,
            missing_enforcement_backend_auto_narrows: true,
            no_raw_secret_material_in_exports: true,
        },
        consumer_projection: M5RuntimeAuthorityMatrixConsumerProjection {
            desktop_shows_authority_envelope: true,
            command_and_policy_reference_same_matrix: true,
            cli_headless_shows_authority_envelope: true,
            support_export_shows_authority_envelope: true,
            diagnostics_shows_authority_envelope: true,
            help_about_shows_qualification: true,
            release_evidence_consumes_matrix: true,
            preview_labs_label_for_unqualified_surfaces: true,
        },
        proof_freshness: M5RuntimeAuthorityMatrixProofFreshness {
            proof_freshness_slo_hours: 168,
            last_proof_refresh: "2026-06-10T00:00:00Z".to_owned(),
            auto_narrow_on_stale: true,
        },
        source_contract_refs: vec![
            M5_RUNTIME_AUTHORITY_MATRIX_SCHEMA_REF.to_owned(),
            M5_RUNTIME_AUTHORITY_MATRIX_DOC_REF.to_owned(),
            M5_RUNTIME_AUTHORITY_MATRIX_ISSUER_CONTRACT_REF.to_owned(),
            M5_RUNTIME_AUTHORITY_MATRIX_AUTHORITY_TICKET_CONTRACT_REF.to_owned(),
            M5_RUNTIME_AUTHORITY_MATRIX_APPROVAL_TICKET_CONTRACT_REF.to_owned(),
            M5_RUNTIME_AUTHORITY_MATRIX_SECRET_HANDLE_CONTRACT_REF.to_owned(),
            M5_RUNTIME_AUTHORITY_MATRIX_SECRET_BOUNDARY_CONTRACT_REF.to_owned(),
        ],
        redaction_class_token: "metadata_safe_default".to_owned(),
        minted_at: "2026-06-10T00:00:00Z".to_owned(),
    })
}

/// Reads and validates the checked-in stable M5 runtime-authority matrix export.
pub fn current_stable_m5_runtime_authority_matrix_export(
) -> Result<M5RuntimeAuthorityMatrixPacket, M5RuntimeAuthorityMatrixArtifactError> {
    let packet: M5RuntimeAuthorityMatrixPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/execution-auth/m5/freeze-the-m5-runtime-authority-approval-ticket-sandbox-profile-and-capability-envelope-matrix/support_export.json"
    )))
    .map_err(M5RuntimeAuthorityMatrixArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(M5RuntimeAuthorityMatrixArtifactError::Validation(
            violations,
        ))
    }
}

fn validate_source_contracts(
    packet: &M5RuntimeAuthorityMatrixPacket,
    violations: &mut Vec<M5RuntimeAuthorityMatrixViolation>,
) {
    let refs: BTreeSet<&str> = packet
        .source_contract_refs
        .iter()
        .map(String::as_str)
        .collect();
    for required in [
        M5_RUNTIME_AUTHORITY_MATRIX_SCHEMA_REF,
        M5_RUNTIME_AUTHORITY_MATRIX_DOC_REF,
        M5_RUNTIME_AUTHORITY_MATRIX_ISSUER_CONTRACT_REF,
        M5_RUNTIME_AUTHORITY_MATRIX_AUTHORITY_TICKET_CONTRACT_REF,
        M5_RUNTIME_AUTHORITY_MATRIX_APPROVAL_TICKET_CONTRACT_REF,
        M5_RUNTIME_AUTHORITY_MATRIX_SECRET_HANDLE_CONTRACT_REF,
        M5_RUNTIME_AUTHORITY_MATRIX_SECRET_BOUNDARY_CONTRACT_REF,
    ] {
        if !refs.contains(required) {
            violations.push(M5RuntimeAuthorityMatrixViolation::MissingSourceContracts);
            return;
        }
    }
}

fn validate_surface_rows(
    packet: &M5RuntimeAuthorityMatrixPacket,
    violations: &mut Vec<M5RuntimeAuthorityMatrixViolation>,
) {
    let present: BTreeSet<M5ExecutingSurface> =
        packet.surface_rows.iter().map(|row| row.surface).collect();
    for required in M5ExecutingSurface::ALL {
        if !present.contains(&required) {
            violations.push(M5RuntimeAuthorityMatrixViolation::RequiredSurfaceMissing);
            return;
        }
    }

    for row in &packet.surface_rows {
        if row.scope_summary.trim().is_empty() || row.source_contract_refs.is_empty() {
            violations.push(M5RuntimeAuthorityMatrixViolation::SurfaceRowIncomplete);
        }
        if row.allowed_capability_classes.is_empty() {
            violations.push(M5RuntimeAuthorityMatrixViolation::CapabilityEnvelopeEmpty);
        }
        if row.qualification.is_stable() && row.required_evidence_packet_refs.is_empty() {
            violations.push(M5RuntimeAuthorityMatrixViolation::StableSurfaceMissingEvidence);
        }
        if row.downgrade_triggers.is_empty() {
            violations.push(M5RuntimeAuthorityMatrixViolation::DowngradeTriggersMissing);
        }
        if row.consumer_surfaces.is_empty() {
            violations.push(M5RuntimeAuthorityMatrixViolation::ConsumerSurfacesMissing);
        }
        if row.surface.is_untrusted_helper() && !row.approval_ticket_posture.is_externally_issued()
        {
            violations.push(M5RuntimeAuthorityMatrixViolation::SelfIssuedAuthorityForbidden);
        }
        let has_elevated = row
            .allowed_capability_classes
            .iter()
            .any(|cap| cap.is_elevated());
        if has_elevated && !row.approval_ticket_posture.is_externally_issued() {
            violations.push(M5RuntimeAuthorityMatrixViolation::ElevatedCapabilityWithoutTicket);
        }
        if row.approval_ticket_posture.is_time_bounded_ticket() && row.ticket_expiry_seconds == 0 {
            violations.push(M5RuntimeAuthorityMatrixViolation::TicketExpiryMissing);
        }
        let projects_secret = row
            .allowed_capability_classes
            .iter()
            .any(|cap| cap.requires_secret_scope());
        if projects_secret && !row.secret_scope.grants_secret_access() {
            violations.push(M5RuntimeAuthorityMatrixViolation::SecretScopeInconsistent);
        }
    }
}

fn validate_trust_review(
    packet: &M5RuntimeAuthorityMatrixPacket,
    violations: &mut Vec<M5RuntimeAuthorityMatrixViolation>,
) {
    let review = &packet.trust_review;
    for ok in [
        review.no_ambient_machine_privilege,
        review.no_self_issued_authority_by_helpers,
        review.target_identity_inspectable_and_export_safe,
        review.sandbox_profile_inspectable,
        review.secret_scope_handle_only_no_raw_material,
        review.policy_epoch_and_expiry_inspectable,
        review.degraded_fallback_inspectable,
        review.fail_closed_when_enforcement_unavailable,
        review.missing_enforcement_backend_auto_narrows,
        review.no_raw_secret_material_in_exports,
    ] {
        if !ok {
            violations.push(M5RuntimeAuthorityMatrixViolation::TrustReviewIncomplete);
            return;
        }
    }
}

fn validate_consumer_projection(
    packet: &M5RuntimeAuthorityMatrixPacket,
    violations: &mut Vec<M5RuntimeAuthorityMatrixViolation>,
) {
    let projection = &packet.consumer_projection;
    for ok in [
        projection.desktop_shows_authority_envelope,
        projection.command_and_policy_reference_same_matrix,
        projection.cli_headless_shows_authority_envelope,
        projection.support_export_shows_authority_envelope,
        projection.diagnostics_shows_authority_envelope,
        projection.help_about_shows_qualification,
        projection.release_evidence_consumes_matrix,
        projection.preview_labs_label_for_unqualified_surfaces,
    ] {
        if !ok {
            violations.push(M5RuntimeAuthorityMatrixViolation::ConsumerProjectionIncomplete);
            return;
        }
    }
}

fn validate_proof_freshness(
    packet: &M5RuntimeAuthorityMatrixPacket,
    violations: &mut Vec<M5RuntimeAuthorityMatrixViolation>,
) {
    if packet.proof_freshness.proof_freshness_slo_hours == 0
        || packet.proof_freshness.last_proof_refresh.trim().is_empty()
    {
        violations.push(M5RuntimeAuthorityMatrixViolation::ProofFreshnessIncomplete);
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
