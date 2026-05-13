//! Restricted-mode workspace trust gates and launch-wedge disclosure.
//!
//! This module owns the alpha trust packet a launch surface can mint when a
//! workspace opens under restricted mode. The packet keeps the workspace trust
//! state visible after open, projects allowed and blocked capabilities with
//! source / scope / recovery details, and validates the guardrails that keep
//! local reading useful while execution, mutation, install, provider, and AI
//! apply lanes stay gated.
//!
//! The packet deliberately reuses the auth lane's [`IdentityModeAlias`] and
//! the workspace crate's binary [`TrustState`] posture while adding the finer
//! restricted-mode state vocabulary needed by launch surfaces. Downstream shell,
//! CLI, support, task, debug, extension, and AI lanes should quote this packet
//! or its [`RestrictedModeLaunchWedgeDisclosure`] projection instead of
//! inventing a local `is_trusted` flag.

use serde::{Deserialize, Serialize};

pub use crate::browser_callback::{IdentityModeAlias, TrustState};

/// Record-kind tag carried on serialized [`RestrictedModeAlphaPacket`] payloads.
pub const RESTRICTED_MODE_ALPHA_PACKET_RECORD_KIND: &str =
    "restricted_mode_launch_wedge_packet_record";

/// Schema version of the restricted-mode alpha packet payload.
///
/// Bumped on breaking payload changes; additive-optional fields do not bump
/// this version.
pub const RESTRICTED_MODE_ALPHA_SCHEMA_VERSION: u32 = 1;

/// Fine-grained workspace trust state used by restricted-mode launch gates.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RestrictedModeTrustStateClass {
    /// No remembered decision exists for this workspace; restricted posture
    /// applies until a user or admin decision says otherwise.
    UntrustedUnknown,
    /// Local read / edit / search / save is admitted while execution and
    /// mutation surfaces are gated.
    Restricted,
    /// The workspace is trusted under the current user and policy ceiling.
    Trusted,
    /// The workspace is trusted with an explicit expiry.
    TrustedTimeBounded,
    /// The user trusted the workspace, but policy narrowed one or more
    /// capability families.
    TrustedPolicyDegraded,
    /// Recovery forced restricted mode after a startup or runtime fault.
    RestrictedRecoveryFallback,
    /// Restricted posture plus an extension quarantine is active.
    RestrictedExtensionQuarantine,
    /// Trust was explicitly revoked; restricted posture applies.
    TrustRevoked,
    /// A required identity or policy source is unavailable; restricted
    /// posture applies until the gate resolves.
    TrustUnavailableIdentityGate,
}

impl RestrictedModeTrustStateClass {
    /// Stable string token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::UntrustedUnknown => "untrusted_unknown",
            Self::Restricted => "restricted",
            Self::Trusted => "trusted",
            Self::TrustedTimeBounded => "trusted_time_bounded",
            Self::TrustedPolicyDegraded => "trusted_policy_degraded",
            Self::RestrictedRecoveryFallback => "restricted_recovery_fallback",
            Self::RestrictedExtensionQuarantine => "restricted_extension_quarantine",
            Self::TrustRevoked => "trust_revoked",
            Self::TrustUnavailableIdentityGate => "trust_unavailable_identity_gate",
        }
    }

    /// True when the restricted-posture floor must remain enforced.
    pub const fn restricted_floor_applies(self) -> bool {
        matches!(
            self,
            Self::UntrustedUnknown
                | Self::Restricted
                | Self::RestrictedRecoveryFallback
                | Self::RestrictedExtensionQuarantine
                | Self::TrustRevoked
                | Self::TrustUnavailableIdentityGate
        )
    }

    /// True when the state admits trusted execution by default.
    pub const fn trusted_execution_available(self) -> bool {
        matches!(self, Self::Trusted | Self::TrustedTimeBounded)
    }

    /// Returns the binary workspace trust posture used by earlier workspace
    /// records.
    pub const fn binary_trust_state(self) -> TrustState {
        if self.trusted_execution_available() || matches!(self, Self::TrustedPolicyDegraded) {
            TrustState::Trusted
        } else if matches!(self, Self::UntrustedUnknown) {
            TrustState::PendingEvaluation
        } else {
            TrustState::Restricted
        }
    }
}

/// Entry transition that produced the current launch trust state.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RestrictedModeEntryTransitionClass {
    /// First open of an unknown workspace.
    InitialOpenUntrusted,
    /// User explicitly opened the workspace in restricted mode.
    OpenInRestrictedMode,
    /// User continued a currently-open workspace after a denied, expired, or
    /// revoked trust grant.
    ContinueInRestrictedMode,
    /// Workspace opened restricted with layout restore suspended.
    OpenWithoutRestore,
    /// Recovery forced restricted mode and suspended activators.
    SafeModeWorkspaceRestricted,
    /// Restricted mode plus extension quarantine.
    ExtensionQuarantineRestricted,
    /// User granted trust for the current session.
    GrantTrustSession,
    /// User granted remembered trust for the workspace.
    GrantTrustRemembered,
    /// Admin policy pre-bound trust for the workspace.
    GrantTrustAdminPrebinding,
    /// User, admin, or emergency action revoked trust.
    RevokeTrust,
    /// Policy narrowed a trusted workspace.
    PolicyNarrowToDegraded,
    /// Policy restored a previously narrowed trusted workspace.
    PolicyRestoreTrusted,
    /// Emergency action forced restricted posture.
    EmergencyActionForceRestricted,
    /// Identity or policy source needed for trust is unavailable.
    IdentityGateUnavailable,
}

impl RestrictedModeEntryTransitionClass {
    /// Stable string token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::InitialOpenUntrusted => "initial_open_untrusted",
            Self::OpenInRestrictedMode => "open_in_restricted_mode",
            Self::ContinueInRestrictedMode => "continue_in_restricted_mode",
            Self::OpenWithoutRestore => "open_without_restore",
            Self::SafeModeWorkspaceRestricted => "safe_mode_workspace_restricted",
            Self::ExtensionQuarantineRestricted => "extension_quarantine_restricted",
            Self::GrantTrustSession => "grant_trust_session",
            Self::GrantTrustRemembered => "grant_trust_remembered",
            Self::GrantTrustAdminPrebinding => "grant_trust_admin_prebinding",
            Self::RevokeTrust => "revoke_trust",
            Self::PolicyNarrowToDegraded => "policy_narrow_to_degraded",
            Self::PolicyRestoreTrusted => "policy_restore_trusted",
            Self::EmergencyActionForceRestricted => "emergency_action_force_restricted",
            Self::IdentityGateUnavailable => "identity_gate_unavailable",
        }
    }

    /// True when the transition widens trust and therefore requires an
    /// explicit trust-grant audit event.
    pub const fn widens_trust(self) -> bool {
        matches!(
            self,
            Self::GrantTrustSession
                | Self::GrantTrustRemembered
                | Self::GrantTrustAdminPrebinding
                | Self::PolicyRestoreTrusted
        )
    }

    /// True when layout restore must be suspended.
    pub const fn suspends_layout_restore(self) -> bool {
        matches!(
            self,
            Self::OpenWithoutRestore | Self::SafeModeWorkspaceRestricted
        )
    }
}

/// Authority that produced or narrowed the trust decision.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TrustDecisionSourceClass {
    /// The current local user made the decision.
    LocalUser,
    /// A local administrator made the decision.
    LocalAdmin,
    /// A managed administrator or managed policy source made the decision.
    ManagedAdmin,
    /// A signed repository allowance admitted the workspace.
    SignedRepoAllowance,
    /// A signed emergency-action bundle narrowed the workspace.
    EmergencyActionSigner,
    /// A recovery-ladder action narrowed the workspace.
    RecoveryLadderApplication,
    /// A policy epoch roll changed the effective state.
    PolicyEpochRoll,
    /// Identity or policy availability changed the effective state.
    IdentityGateUnavailableSystem,
    /// No remembered decision exists yet.
    NoRememberedDecision,
}

impl TrustDecisionSourceClass {
    /// Stable string token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LocalUser => "local_user",
            Self::LocalAdmin => "local_admin",
            Self::ManagedAdmin => "managed_admin",
            Self::SignedRepoAllowance => "signed_repo_allowance",
            Self::EmergencyActionSigner => "emergency_action_signer",
            Self::RecoveryLadderApplication => "recovery_ladder_application",
            Self::PolicyEpochRoll => "policy_epoch_roll",
            Self::IdentityGateUnavailableSystem => "identity_gate_unavailable_system",
            Self::NoRememberedDecision => "no_remembered_decision",
        }
    }
}

/// Why the trust state changed or was resolved.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TrustReasonClass {
    /// First open found no remembered decision.
    FirstOpenNoRememberedDecision,
    /// User explicitly declined trust.
    ExplicitUserDecline,
    /// User or admin explicitly granted trust.
    ExplicitUserGrant,
    /// User, admin, or emergency action explicitly revoked trust.
    ExplicitUserRevoke,
    /// Session grant expired.
    SessionExpired,
    /// Admin policy narrowed the effective trust posture.
    AdminPolicyNarrowed,
    /// Admin policy restored a previously narrowed posture.
    AdminPolicyRestored,
    /// Emergency action forced restricted posture.
    EmergencyActionApplied,
    /// Recovery ladder forced restricted posture.
    RecoveryLadderFallback,
    /// Extension quarantine narrowed the workspace.
    ExtensionQuarantineApplied,
    /// Identity gate became unavailable.
    IdentityGateUnavailable,
    /// Policy epoch roll requires a renewed prompt.
    PolicyEpochRollRequiredReprompt,
}

impl TrustReasonClass {
    /// Stable string token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::FirstOpenNoRememberedDecision => "first_open_no_remembered_decision",
            Self::ExplicitUserDecline => "explicit_user_decline",
            Self::ExplicitUserGrant => "explicit_user_grant",
            Self::ExplicitUserRevoke => "explicit_user_revoke",
            Self::SessionExpired => "session_expired",
            Self::AdminPolicyNarrowed => "admin_policy_narrowed",
            Self::AdminPolicyRestored => "admin_policy_restored",
            Self::EmergencyActionApplied => "emergency_action_applied",
            Self::RecoveryLadderFallback => "recovery_ladder_fallback",
            Self::ExtensionQuarantineApplied => "extension_quarantine_applied",
            Self::IdentityGateUnavailable => "identity_gate_unavailable",
            Self::PolicyEpochRollRequiredReprompt => "policy_epoch_roll_required_reprompt",
        }
    }
}

/// Remembered-decision scope attached to a trust decision.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RememberedDecisionScopeClass {
    /// Decision applies to the current process or time window only.
    SessionOnly,
    /// Decision applies to the exact workspace root for the current user.
    PerWorkspacePerUser,
    /// Decision applies to immediate child workspaces of a parent directory.
    PerParentDirectoryPerUser,
    /// Decision is owned by an admin policy scope.
    AdminPolicyScope,
    /// Decline is recorded without suppressing future prompts.
    NeverRemembered,
    /// No remembered scope exists for this system-issued state.
    NotRememberedSystemIssued,
}

impl RememberedDecisionScopeClass {
    /// Stable string token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SessionOnly => "session_only",
            Self::PerWorkspacePerUser => "per_workspace_per_user",
            Self::PerParentDirectoryPerUser => "per_parent_directory_per_user",
            Self::AdminPolicyScope => "admin_policy_scope",
            Self::NeverRemembered => "never_remembered",
            Self::NotRememberedSystemIssued => "not_remembered_system_issued",
        }
    }
}

/// Launch-wedge capability family controlled by workspace trust.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LaunchWedgeCapabilityFamily {
    /// Open folder, reopen recent, restore last session, or reopen restricted.
    WorkspaceOpenRestore,
    /// Open, edit, and save local files.
    EditorReadWrite,
    /// Search local text, symbols, and references.
    SearchLocal,
    /// Read repository status, branches, log, or diff.
    LocalGitRead,
    /// Stage, commit, fetch, pull, push, merge, rebase, or otherwise mutate Git state.
    LocalGitWrite,
    /// Dispatch commands from palette, keybindings, menus, or automation.
    ShellCommandPalette,
    /// Run repo-defined tasks.
    TasksRun,
    /// Open a user-initiated terminal.
    TerminalManualOpen,
    /// Launch a repo-owned terminal recipe or auto-start hook.
    TerminalRepoRecipeLaunch,
    /// Launch a repo-defined debug configuration.
    DebugLaunch,
    /// Attach to a workspace-declared notebook kernel.
    NotebookKernelConnect,
    /// Execute a notebook cell.
    NotebookCellExecute,
    /// Assemble or inspect AI context without applying changes.
    AiContextRead,
    /// Apply AI-generated edits or create files.
    AiApplyMutation,
    /// Let AI invoke a mutating tool.
    AiToolCallMutating,
    /// Auto-activate extensions on workspace events.
    ExtensionActivation,
    /// Install or update extensions in the workspace context.
    ExtensionInstall,
    /// Run repo-owned environment activators such as direnv or devcontainer lifecycle hooks.
    EnvironmentActivatorRun,
    /// Run package install helpers or install/update scripts.
    PackageInstallHelper,
    /// Start browser or device handoff for a connected provider.
    ConnectedProviderOpen,
    /// Call a connected-provider tool with egress, billing, or mutation effects.
    ConnectedProviderToolCall,
    /// Attach to a remote, SSH, container, or managed target.
    RemoteAttach,
    /// Launch a workspace-declared MCP server.
    McpServerLaunch,
    /// Export redacted support metadata.
    SupportBundleExport,
    /// Read effective admin policy summary.
    AdminPolicyRead,
}

impl LaunchWedgeCapabilityFamily {
    /// Stable string token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WorkspaceOpenRestore => "workspace_open_restore",
            Self::EditorReadWrite => "editor_read_write",
            Self::SearchLocal => "search_local",
            Self::LocalGitRead => "local_git_read",
            Self::LocalGitWrite => "local_git_write",
            Self::ShellCommandPalette => "shell_command_palette",
            Self::TasksRun => "tasks_run",
            Self::TerminalManualOpen => "terminal_manual_open",
            Self::TerminalRepoRecipeLaunch => "terminal_repo_recipe_launch",
            Self::DebugLaunch => "debug_launch",
            Self::NotebookKernelConnect => "notebook_kernel_connect",
            Self::NotebookCellExecute => "notebook_cell_execute",
            Self::AiContextRead => "ai_context_read",
            Self::AiApplyMutation => "ai_apply_mutation",
            Self::AiToolCallMutating => "ai_tool_call_mutating",
            Self::ExtensionActivation => "extension_activation",
            Self::ExtensionInstall => "extension_install",
            Self::EnvironmentActivatorRun => "environment_activator_run",
            Self::PackageInstallHelper => "package_install_helper",
            Self::ConnectedProviderOpen => "connected_provider_open",
            Self::ConnectedProviderToolCall => "connected_provider_tool_call",
            Self::RemoteAttach => "remote_attach",
            Self::McpServerLaunch => "mcp_server_launch",
            Self::SupportBundleExport => "support_bundle_export",
            Self::AdminPolicyRead => "admin_policy_read",
        }
    }

    /// True when this family belongs to the restricted-posture floor.
    pub const fn restricted_floor_family(self) -> bool {
        matches!(
            self,
            Self::WorkspaceOpenRestore
                | Self::EditorReadWrite
                | Self::SearchLocal
                | Self::LocalGitRead
                | Self::SupportBundleExport
                | Self::AdminPolicyRead
        )
    }

    /// Short label suitable for shell and support projections.
    pub const fn label(self) -> &'static str {
        match self {
            Self::WorkspaceOpenRestore => "Open and restore workspace",
            Self::EditorReadWrite => "Read, edit, and save local files",
            Self::SearchLocal => "Search and navigate local workspace",
            Self::LocalGitRead => "Inspect local Git state",
            Self::LocalGitWrite => "Mutate local Git state",
            Self::ShellCommandPalette => "Use command palette",
            Self::TasksRun => "Run workspace tasks",
            Self::TerminalManualOpen => "Open manual terminal",
            Self::TerminalRepoRecipeLaunch => "Launch repo terminal recipe",
            Self::DebugLaunch => "Start debugger launch",
            Self::NotebookKernelConnect => "Connect notebook kernel",
            Self::NotebookCellExecute => "Execute notebook cell",
            Self::AiContextRead => "Inspect AI context",
            Self::AiApplyMutation => "Apply AI changes",
            Self::AiToolCallMutating => "Run AI mutating tool",
            Self::ExtensionActivation => "Activate workspace extensions",
            Self::ExtensionInstall => "Install or update extension",
            Self::EnvironmentActivatorRun => "Run environment activator",
            Self::PackageInstallHelper => "Run package install helper",
            Self::ConnectedProviderOpen => "Open connected-provider handoff",
            Self::ConnectedProviderToolCall => "Call connected-provider tool",
            Self::RemoteAttach => "Attach remote target",
            Self::McpServerLaunch => "Launch workspace MCP server",
            Self::SupportBundleExport => "Export support metadata",
            Self::AdminPolicyRead => "Read policy summary",
        }
    }
}

/// Effective authority for a capability family.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CapabilityAuthorityClass {
    /// Admitted under the active trust state without extra gating.
    Allowed,
    /// Admitted only in a read posture.
    ReadOnly,
    /// Admitted as a non-committing preview.
    DegradedPreviewOnly,
    /// Denied pending workspace trust.
    BlockedPendingTrust,
    /// Denied pending a per-invocation approval ticket.
    BlockedPendingApproval,
    /// Admitted only with a per-invocation approval ticket.
    ApprovalRequiredPerInvocation,
    /// Denied by admin, managed, or emergency policy.
    PolicyDenied,
    /// Denied by quarantine state.
    QuarantineDenied,
    /// No meaningful operation exists for this trust state.
    NotApplicable,
}

impl CapabilityAuthorityClass {
    /// Stable string token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Allowed => "allowed",
            Self::ReadOnly => "read_only",
            Self::DegradedPreviewOnly => "degraded_preview_only",
            Self::BlockedPendingTrust => "blocked_pending_trust",
            Self::BlockedPendingApproval => "blocked_pending_approval",
            Self::ApprovalRequiredPerInvocation => "approval_required_per_invocation",
            Self::PolicyDenied => "policy_denied",
            Self::QuarantineDenied => "quarantine_denied",
            Self::NotApplicable => "not_applicable",
        }
    }

    /// True when the capability can proceed without a blocked state.
    pub const fn is_admitted(self) -> bool {
        matches!(self, Self::Allowed | Self::ReadOnly)
    }

    /// True when the capability must show a blocked, approval, policy, or
    /// quarantine explanation.
    pub const fn requires_explanation(self) -> bool {
        !matches!(self, Self::Allowed | Self::ReadOnly | Self::NotApplicable)
    }
}

/// Side-effect family attached to a capability row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ExternalEffectClass {
    /// Local metadata or local file read/write with no execution or egress.
    LocalOnlyNoExternalEffect,
    /// Launches a local process or shell.
    LocalProcessLaunch,
    /// Mutates workspace files, repository state, or generated artifacts.
    WorkspaceMutation,
    /// Sends data to or receives authority from a provider or network service.
    NetworkOrProviderEgress,
    /// Depends on a hosted service or managed control plane.
    HostedDependency,
    /// Uses credential, secret, or identity authority.
    SecretOrIdentityUse,
    /// Installs or updates code, packages, extensions, or runtime artifacts.
    InstallOrUpdateMutation,
}

impl ExternalEffectClass {
    /// Stable string token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LocalOnlyNoExternalEffect => "local_only_no_external_effect",
            Self::LocalProcessLaunch => "local_process_launch",
            Self::WorkspaceMutation => "workspace_mutation",
            Self::NetworkOrProviderEgress => "network_or_provider_egress",
            Self::HostedDependency => "hosted_dependency",
            Self::SecretOrIdentityUse => "secret_or_identity_use",
            Self::InstallOrUpdateMutation => "install_or_update_mutation",
        }
    }

    /// True when the row must disclose hosted, network, or provider
    /// dependency instead of implying local-only behavior.
    pub const fn requires_hosted_dependency_disclosure(self) -> bool {
        matches!(
            self,
            Self::NetworkOrProviderEgress | Self::HostedDependency | Self::SecretOrIdentityUse
        )
    }

    /// True when the row needs an install/update review before mutation.
    pub const fn requires_install_review(self) -> bool {
        matches!(self, Self::InstallOrUpdateMutation)
    }
}

/// Recovery or review action offered for a trust-gated capability.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TrustRecoveryActionClass {
    /// Open the trust-grant affordance with a scope picker.
    RequestTrustGrant,
    /// Open a session-only trust grant affordance.
    RequestTrustGrantSessionOnly,
    /// Route to a per-invocation approval ticket.
    RequestApprovalTicket,
    /// Ask an administrator to change policy.
    RequestAdminPolicyChange,
    /// Open policy details.
    OpenPolicyDetails,
    /// Open details for the blocked capability.
    OpenCapabilityDetails,
    /// Export support diagnostics with trust posture included.
    RouteToSupportBundleExport,
    /// Continue restricted without elevation.
    ContinueRestrictedNoElevation,
    /// Use a local read-only or metadata-only alternative.
    UseLocalReadOnlyAlternative,
}

impl TrustRecoveryActionClass {
    /// Stable string token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::RequestTrustGrant => "request_trust_grant",
            Self::RequestTrustGrantSessionOnly => "request_trust_grant_session_only",
            Self::RequestApprovalTicket => "request_approval_ticket",
            Self::RequestAdminPolicyChange => "request_admin_policy_change",
            Self::OpenPolicyDetails => "open_policy_details",
            Self::OpenCapabilityDetails => "open_capability_details",
            Self::RouteToSupportBundleExport => "route_to_support_bundle_export",
            Self::ContinueRestrictedNoElevation => "continue_restricted_no_elevation",
            Self::UseLocalReadOnlyAlternative => "use_local_read_only_alternative",
        }
    }

    /// Short shell/support label.
    pub const fn label(self) -> &'static str {
        match self {
            Self::RequestTrustGrant => "Trust workspace",
            Self::RequestTrustGrantSessionOnly => "Trust for session",
            Self::RequestApprovalTicket => "Request approval",
            Self::RequestAdminPolicyChange => "Request admin change",
            Self::OpenPolicyDetails => "Open policy details",
            Self::OpenCapabilityDetails => "Open capability details",
            Self::RouteToSupportBundleExport => "Export diagnostics",
            Self::ContinueRestrictedNoElevation => "Continue restricted",
            Self::UseLocalReadOnlyAlternative => "Use read-only alternative",
        }
    }
}

/// Audit event emitted for trust-state resolution or matrix decisions.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TrustAuditEventClass {
    /// Trust state was resolved at open or restore.
    WorkspaceTrustStateResolved,
    /// Trust was granted.
    WorkspaceTrustGranted,
    /// Trust was declined.
    WorkspaceTrustDeclined,
    /// Trust was revoked.
    WorkspaceTrustRevoked,
    /// A policy narrowed trust.
    WorkspaceTrustPolicyNarrowed,
    /// A recovery action forced restricted mode.
    WorkspaceTrustRecoveryApplied,
    /// A surface request was denied by the trust matrix.
    WorkspaceTrustMatrixRowDenied,
    /// A surface request was admitted by the trust matrix.
    WorkspaceTrustMatrixRowAdmitted,
}

impl TrustAuditEventClass {
    /// Stable string token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WorkspaceTrustStateResolved => "workspace_trust_state_resolved",
            Self::WorkspaceTrustGranted => "workspace_trust_granted",
            Self::WorkspaceTrustDeclined => "workspace_trust_declined",
            Self::WorkspaceTrustRevoked => "workspace_trust_revoked",
            Self::WorkspaceTrustPolicyNarrowed => "workspace_trust_policy_narrowed",
            Self::WorkspaceTrustRecoveryApplied => "workspace_trust_recovery_applied",
            Self::WorkspaceTrustMatrixRowDenied => "workspace_trust_matrix_row_denied",
            Self::WorkspaceTrustMatrixRowAdmitted => "workspace_trust_matrix_row_admitted",
        }
    }
}

/// Escalation cue exposed by the packet.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TrustEscalationCueClass {
    /// Present workspace trust grant.
    RequestTrustGrant,
    /// Present session-only trust grant.
    RequestTrustGrantSessionOnly,
    /// Request a per-invocation approval ticket.
    RequestApprovalTicket,
    /// Request admin policy change.
    RequestAdminPolicyChange,
    /// Export support diagnostics.
    RouteToSupportBundleExport,
    /// Keep working restricted.
    ContinueRestrictedNoElevation,
}

impl TrustEscalationCueClass {
    /// Stable string token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::RequestTrustGrant => "request_trust_grant",
            Self::RequestTrustGrantSessionOnly => "request_trust_grant_session_only",
            Self::RequestApprovalTicket => "request_approval_ticket",
            Self::RequestAdminPolicyChange => "request_admin_policy_change",
            Self::RouteToSupportBundleExport => "route_to_support_bundle_export",
            Self::ContinueRestrictedNoElevation => "continue_restricted_no_elevation",
        }
    }
}

/// Source that resolved the workspace trust state.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RestrictedModeTrustSource {
    /// Source class.
    pub source_class: TrustDecisionSourceClass,
    /// Stable token for [`Self::source_class`].
    pub source_class_token: String,
    /// Opaque source ref safe for support export.
    pub source_ref: String,
    /// Human-readable source label.
    pub source_label: String,
    /// Reason class.
    pub reason_class: TrustReasonClass,
    /// Stable token for [`Self::reason_class`].
    pub reason_class_token: String,
    /// Remembered decision scope.
    pub remembered_decision_scope: RememberedDecisionScopeClass,
    /// Stable token for [`Self::remembered_decision_scope`].
    pub remembered_decision_scope_token: String,
    /// Policy epoch or local policy bundle ref active for the decision.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub policy_epoch_ref: Option<String>,
    /// Refs that explain why the trust state resolved this way.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub source_reason_refs: Vec<String>,
}

/// Source that admitted, narrowed, or blocked one capability row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CapabilityDecisionSource {
    /// Source class.
    pub source_class: TrustDecisionSourceClass,
    /// Stable token for [`Self::source_class`].
    pub source_class_token: String,
    /// Opaque source ref safe for support export.
    pub source_ref: String,
    /// Human-readable source label.
    pub source_label: String,
    /// Reason class.
    pub reason_class: TrustReasonClass,
    /// Stable token for [`Self::reason_class`].
    pub reason_class_token: String,
}

/// Scope the capability decision applies to.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CapabilityScope {
    /// Opaque scope ref.
    pub scope_ref: String,
    /// Human-readable scope label.
    pub scope_label: String,
    /// Opaque workspace root ref.
    pub workspace_root_ref: String,
    /// Policy epoch used to evaluate this scope.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub policy_epoch_ref: Option<String>,
}

/// One capability gate row shown on launch and kept visible after open.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CapabilityDisclosureRow {
    /// Stable capability row id.
    pub capability_ref: String,
    /// Capability family.
    pub surface_family: LaunchWedgeCapabilityFamily,
    /// Stable token for [`Self::surface_family`].
    pub surface_family_token: String,
    /// Short capability label.
    pub capability_label: String,
    /// Effective authority for this trust state.
    pub authority: CapabilityAuthorityClass,
    /// Stable token for [`Self::authority`].
    pub authority_token: String,
    /// Side-effect class.
    pub external_effect_class: ExternalEffectClass,
    /// Stable token for [`Self::external_effect_class`].
    pub external_effect_token: String,
    /// Source of the decision.
    pub decision_source: CapabilityDecisionSource,
    /// Scope of the decision.
    pub scope: CapabilityScope,
    /// Plain-language explanation for why this row is allowed, blocked, or
    /// degraded.
    pub explanation_label: String,
    /// Recovery or review actions offered by the row.
    pub recovery_actions: Vec<TrustRecoveryActionClass>,
    /// Stable tokens for [`Self::recovery_actions`].
    pub recovery_action_tokens: Vec<String>,
    /// Human-readable recovery label.
    pub recovery_label: String,
    /// Whether this row is shown on the launch surface.
    pub visible_on_launch: bool,
    /// Whether the same gate remains visible after the workspace is open.
    pub sticky_after_open: bool,
    /// Whether hosted, network, provider, or identity dependency is disclosed.
    pub hosted_dependency_disclosed: bool,
    /// Whether plaintext secret fallback is allowed by this row.
    pub plaintext_secret_fallback_allowed: bool,
    /// Whether install/update mutation requires review before it can run.
    pub install_or_update_review_required: bool,
}

impl CapabilityDisclosureRow {
    fn new(
        family: LaunchWedgeCapabilityFamily,
        authority: CapabilityAuthorityClass,
        effect: ExternalEffectClass,
        trust_source: &RestrictedModeTrustSource,
        scope: CapabilityScope,
        explanation_label: impl Into<String>,
        recovery_actions: Vec<TrustRecoveryActionClass>,
    ) -> Self {
        let capability_ref = format!(
            "capability:{}:{}",
            scope.workspace_root_ref,
            family.as_str()
        );
        let recovery_action_tokens = recovery_actions
            .iter()
            .map(|action| action.as_str().to_owned())
            .collect();
        let recovery_label = recovery_actions
            .first()
            .map(|action| action.label().to_owned())
            .unwrap_or_else(|| "No recovery action".to_owned());
        Self {
            capability_ref,
            surface_family: family,
            surface_family_token: family.as_str().to_owned(),
            capability_label: family.label().to_owned(),
            authority,
            authority_token: authority.as_str().to_owned(),
            external_effect_class: effect,
            external_effect_token: effect.as_str().to_owned(),
            decision_source: CapabilityDecisionSource {
                source_class: trust_source.source_class,
                source_class_token: trust_source.source_class_token.clone(),
                source_ref: trust_source.source_ref.clone(),
                source_label: trust_source.source_label.clone(),
                reason_class: trust_source.reason_class,
                reason_class_token: trust_source.reason_class_token.clone(),
            },
            scope,
            explanation_label: explanation_label.into(),
            recovery_actions,
            recovery_action_tokens,
            recovery_label,
            visible_on_launch: true,
            sticky_after_open: true,
            hosted_dependency_disclosed: effect.requires_hosted_dependency_disclosure(),
            plaintext_secret_fallback_allowed: false,
            install_or_update_review_required: effect.requires_install_review(),
        }
    }

    /// True when the row is currently blocked or needs approval.
    pub const fn blocked_or_needs_review(&self) -> bool {
        self.authority.requires_explanation()
    }
}

/// Trust decision receipt carried by the packet for audit and support export.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TrustDecisionReceipt {
    /// Audit event class.
    pub audit_event_class: TrustAuditEventClass,
    /// Stable token for [`Self::audit_event_class`].
    pub audit_event_token: String,
    /// Transition id that produced the state.
    pub transition_id: RestrictedModeEntryTransitionClass,
    /// Stable token for [`Self::transition_id`].
    pub transition_token: String,
    /// Affected capability families in stable token form.
    pub affected_surface_tokens: Vec<String>,
    /// Escalation cues available from the launch disclosure.
    pub escalation_cues: Vec<TrustEscalationCueClass>,
    /// Stable tokens for [`Self::escalation_cues`].
    pub escalation_cue_tokens: Vec<String>,
    /// Opaque recovery action ref when a recovery ladder caused this state.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub recovery_action_ref: Option<String>,
    /// Whether the trust gate must remain visible after open.
    pub visible_after_open_required: bool,
    /// Redaction class for support and CLI export.
    pub redaction_class: String,
}

/// Inputs for staging a restricted-mode launch packet.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StageRestrictedModeLaunchRequest<'a> {
    /// Stable packet id.
    pub packet_id: &'a str,
    /// Opaque workspace root ref.
    pub workspace_root_ref: &'a str,
    /// Human-readable workspace scope label.
    pub workspace_display_scope: &'a str,
    /// Current identity mode.
    pub identity_mode: IdentityModeAlias,
    /// Trust state before this transition.
    pub prior_trust_state: Option<RestrictedModeTrustStateClass>,
    /// Effective trust state after this transition.
    pub effective_trust_state: RestrictedModeTrustStateClass,
    /// Entry transition that produced the state.
    pub entry_transition: RestrictedModeEntryTransitionClass,
    /// Source class.
    pub source_class: TrustDecisionSourceClass,
    /// Opaque source ref.
    pub source_ref: &'a str,
    /// Human-readable source label.
    pub source_label: &'a str,
    /// Reason class.
    pub reason_class: TrustReasonClass,
    /// Remembered decision scope.
    pub remembered_decision_scope: RememberedDecisionScopeClass,
    /// Active policy epoch ref.
    pub policy_epoch_ref: Option<&'a str>,
    /// Source reason refs.
    pub source_reason_refs: &'a [&'a str],
    /// Recovery action ref when a recovery ladder forced the state.
    pub recovery_action_ref: Option<&'a str>,
    /// Mint timestamp.
    pub issued_at: &'a str,
}

/// Canonical restricted-mode launch packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RestrictedModeAlphaPacket {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Opaque workspace root ref.
    pub workspace_root_ref: String,
    /// Human-readable workspace scope label.
    pub workspace_display_scope: String,
    /// Current identity mode.
    pub identity_mode: IdentityModeAlias,
    /// Binary trust posture reused by earlier workspace records.
    pub binary_trust_state: TrustState,
    /// Effective detailed trust state.
    pub effective_trust_state: RestrictedModeTrustStateClass,
    /// Stable token for [`Self::effective_trust_state`].
    pub effective_trust_state_token: String,
    /// Trust state before the transition.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub prior_trust_state: Option<RestrictedModeTrustStateClass>,
    /// Stable token for [`Self::prior_trust_state`].
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub prior_trust_state_token: Option<String>,
    /// Entry transition that produced the state.
    pub entry_transition: RestrictedModeEntryTransitionClass,
    /// Stable token for [`Self::entry_transition`].
    pub entry_transition_token: String,
    /// Whether layout restore is suspended by this transition.
    pub layout_restore_suspended: bool,
    /// Source and remembered-scope explanation.
    pub trust_source: RestrictedModeTrustSource,
    /// Capability gates shown on launch and kept after open.
    pub capability_gates: Vec<CapabilityDisclosureRow>,
    /// Audit / support receipt.
    pub decision_receipt: TrustDecisionReceipt,
    /// Mint timestamp.
    pub issued_at: String,
}

impl RestrictedModeAlphaPacket {
    /// Stage a restricted-mode launch packet and its default capability matrix.
    pub fn stage_launch_wedge(request: StageRestrictedModeLaunchRequest<'_>) -> Self {
        let trust_source = RestrictedModeTrustSource {
            source_class: request.source_class,
            source_class_token: request.source_class.as_str().to_owned(),
            source_ref: request.source_ref.to_owned(),
            source_label: request.source_label.to_owned(),
            reason_class: request.reason_class,
            reason_class_token: request.reason_class.as_str().to_owned(),
            remembered_decision_scope: request.remembered_decision_scope,
            remembered_decision_scope_token: request.remembered_decision_scope.as_str().to_owned(),
            policy_epoch_ref: request.policy_epoch_ref.map(str::to_owned),
            source_reason_refs: request
                .source_reason_refs
                .iter()
                .map(|entry| (*entry).to_owned())
                .collect(),
        };
        let capability_gates = default_capability_gates(
            request.workspace_root_ref,
            request.workspace_display_scope,
            request.effective_trust_state,
            &trust_source,
        );
        let affected_surface_tokens = capability_gates
            .iter()
            .map(|row| row.surface_family_token.clone())
            .collect();
        let escalation_cues = escalation_cues_for(request.effective_trust_state);
        let escalation_cue_tokens = escalation_cues
            .iter()
            .map(|cue| cue.as_str().to_owned())
            .collect();
        let audit_event_class = audit_event_for(request.entry_transition);

        Self {
            record_kind: RESTRICTED_MODE_ALPHA_PACKET_RECORD_KIND.to_owned(),
            schema_version: RESTRICTED_MODE_ALPHA_SCHEMA_VERSION,
            packet_id: request.packet_id.to_owned(),
            workspace_root_ref: request.workspace_root_ref.to_owned(),
            workspace_display_scope: request.workspace_display_scope.to_owned(),
            identity_mode: request.identity_mode,
            binary_trust_state: request.effective_trust_state.binary_trust_state(),
            effective_trust_state: request.effective_trust_state,
            effective_trust_state_token: request.effective_trust_state.as_str().to_owned(),
            prior_trust_state: request.prior_trust_state,
            prior_trust_state_token: request
                .prior_trust_state
                .map(|state| state.as_str().to_owned()),
            entry_transition: request.entry_transition,
            entry_transition_token: request.entry_transition.as_str().to_owned(),
            layout_restore_suspended: request.entry_transition.suspends_layout_restore(),
            trust_source,
            capability_gates,
            decision_receipt: TrustDecisionReceipt {
                audit_event_class,
                audit_event_token: audit_event_class.as_str().to_owned(),
                transition_id: request.entry_transition,
                transition_token: request.entry_transition.as_str().to_owned(),
                affected_surface_tokens,
                escalation_cues,
                escalation_cue_tokens,
                recovery_action_ref: request.recovery_action_ref.map(str::to_owned),
                visible_after_open_required: true,
                redaction_class: "metadata_safe_default".to_owned(),
            },
            issued_at: request.issued_at.to_owned(),
        }
    }

    /// Returns rows admitted under the active trust state.
    pub fn allowed_rows(&self) -> Vec<&CapabilityDisclosureRow> {
        self.capability_gates
            .iter()
            .filter(|row| row.authority.is_admitted())
            .collect()
    }

    /// Returns rows blocked, degraded, or requiring approval under the active
    /// trust state.
    pub fn blocked_or_review_rows(&self) -> Vec<&CapabilityDisclosureRow> {
        self.capability_gates
            .iter()
            .filter(|row| row.blocked_or_needs_review())
            .collect()
    }

    /// True when the packet exposes both allowed and blocked/review rows.
    pub fn has_allowed_and_blocked_disclosure(&self) -> bool {
        !self.allowed_rows().is_empty() && !self.blocked_or_review_rows().is_empty()
    }

    /// True when the restricted-posture floor remains usable.
    pub fn restricted_floor_available(&self) -> bool {
        required_floor_families().iter().all(|family| {
            self.capability_gates.iter().any(|row| {
                row.surface_family == *family
                    && matches!(
                        row.authority,
                        CapabilityAuthorityClass::Allowed | CapabilityAuthorityClass::ReadOnly
                    )
            })
        })
    }

    /// True when every visible launch gate remains sticky after open.
    pub fn trust_gate_persists_after_open(&self) -> bool {
        self.decision_receipt.visible_after_open_required
            && self
                .capability_gates
                .iter()
                .all(|row| row.visible_on_launch && row.sticky_after_open)
    }

    /// True when all blocked or approval rows disclose source, scope, and
    /// recovery actions.
    pub fn blocked_rows_explain_source_scope_and_recovery(&self) -> bool {
        self.blocked_or_review_rows().iter().all(|row| {
            !row.decision_source.source_ref.trim().is_empty()
                && !row.decision_source.source_label.trim().is_empty()
                && !row.scope.scope_ref.trim().is_empty()
                && !row.scope.scope_label.trim().is_empty()
                && !row.explanation_label.trim().is_empty()
                && !row.recovery_actions.is_empty()
        })
    }

    /// True when restricted workspaces do not allow execution, mutation, or
    /// external-effect families outside the restricted-posture floor.
    pub fn restricted_execution_and_mutation_are_gated(&self) -> bool {
        if !self.effective_trust_state.restricted_floor_applies() {
            return true;
        }
        self.capability_gates.iter().all(|row| {
            row.surface_family.restricted_floor_family()
                || !matches!(
                    row.authority,
                    CapabilityAuthorityClass::Allowed | CapabilityAuthorityClass::ReadOnly
                )
                || row.external_effect_class == ExternalEffectClass::LocalOnlyNoExternalEffect
        })
    }

    /// True when guardrails around trust widening, hosted dependencies,
    /// secret fallback, and install/update review hold.
    pub fn guardrails_hold(&self) -> bool {
        let no_silent_widening = !self.entry_transition.widens_trust()
            || self.decision_receipt.audit_event_class
                == TrustAuditEventClass::WorkspaceTrustGranted;
        let no_hidden_hosted_dependency = self.capability_gates.iter().all(|row| {
            !row.external_effect_class
                .requires_hosted_dependency_disclosure()
                || row.hosted_dependency_disclosed
        });
        let no_plaintext_secret_fallback = self
            .capability_gates
            .iter()
            .all(|row| !row.plaintext_secret_fallback_allowed);
        let install_review_required = self.capability_gates.iter().all(|row| {
            !row.external_effect_class.requires_install_review()
                || row.install_or_update_review_required
        });
        no_silent_widening
            && no_hidden_hosted_dependency
            && no_plaintext_secret_fallback
            && install_review_required
    }

    /// Validate the packet against the restricted-mode alpha invariants.
    pub fn validate(&self) -> Vec<RestrictedModeValidationError> {
        let mut errors = Vec::new();
        if self.record_kind != RESTRICTED_MODE_ALPHA_PACKET_RECORD_KIND {
            errors.push(RestrictedModeValidationError::WrongRecordKind);
        }
        if self.schema_version != RESTRICTED_MODE_ALPHA_SCHEMA_VERSION {
            errors.push(RestrictedModeValidationError::WrongSchemaVersion);
        }
        if self.packet_id.trim().is_empty()
            || self.workspace_root_ref.trim().is_empty()
            || self.workspace_display_scope.trim().is_empty()
        {
            errors.push(RestrictedModeValidationError::MissingWorkspaceIdentity);
        }
        if self.effective_trust_state_token != self.effective_trust_state.as_str() {
            errors.push(RestrictedModeValidationError::MismatchedTrustStateToken);
        }
        if self.entry_transition_token != self.entry_transition.as_str() {
            errors.push(RestrictedModeValidationError::MismatchedTransitionToken);
        }
        if self.capability_gates.is_empty() {
            errors.push(RestrictedModeValidationError::MissingCapabilityRows);
        }
        if !self.has_allowed_and_blocked_disclosure() {
            errors.push(RestrictedModeValidationError::MissingAllowedOrBlockedDisclosure);
        }
        if self.effective_trust_state.restricted_floor_applies()
            && !self.restricted_floor_available()
        {
            errors.push(RestrictedModeValidationError::RestrictedFloorMissing);
        }
        if !self.restricted_execution_and_mutation_are_gated() {
            errors.push(RestrictedModeValidationError::RestrictedExecutionNotGated);
        }
        if !self.blocked_rows_explain_source_scope_and_recovery() {
            errors.push(RestrictedModeValidationError::BlockedRowsMissingExplanation);
        }
        if !self.trust_gate_persists_after_open() {
            errors.push(RestrictedModeValidationError::GateNotStickyAfterOpen);
        }
        if !self.guardrails_hold() {
            errors.push(RestrictedModeValidationError::GuardrailViolation);
        }
        errors
    }

    /// Project a launch-wedge disclosure from this packet.
    pub fn launch_wedge_disclosure(&self) -> RestrictedModeLaunchWedgeDisclosure {
        RestrictedModeLaunchWedgeDisclosure::from_packet(self)
    }

    /// Render the packet as deterministic plaintext for shell, CLI, and
    /// support surfaces.
    pub fn render_plaintext(&self) -> String {
        let mut out = String::new();
        out.push_str(&format!(
            "restricted_mode_packet: {} schema={}\n",
            self.packet_id, self.schema_version
        ));
        out.push_str(&format!(
            "workspace: {} ({})\n",
            self.workspace_display_scope, self.workspace_root_ref
        ));
        out.push_str(&format!(
            "trust: state={} binary={} transition={} source={} reason={} remembered_scope={}\n",
            self.effective_trust_state_token,
            self.binary_trust_state.as_str(),
            self.entry_transition_token,
            self.trust_source.source_class_token,
            self.trust_source.reason_class_token,
            self.trust_source.remembered_decision_scope_token,
        ));
        out.push_str(&format!(
            "visible_after_open: {}\n",
            self.decision_receipt.visible_after_open_required
        ));
        out.push_str("allowed_capabilities:\n");
        for row in self.allowed_rows() {
            out.push_str(&format!(
                "  - {} authority={} scope={} source={} recovery={}\n",
                row.surface_family_token,
                row.authority_token,
                row.scope.scope_label,
                row.decision_source.source_label,
                row.recovery_action_tokens.join("|")
            ));
        }
        out.push_str("blocked_or_review_capabilities:\n");
        for row in self.blocked_or_review_rows() {
            out.push_str(&format!(
                "  - {} authority={} effect={} scope={} source={} recovery={} explanation={}\n",
                row.surface_family_token,
                row.authority_token,
                row.external_effect_token,
                row.scope.scope_label,
                row.decision_source.source_label,
                row.recovery_action_tokens.join("|"),
                row.explanation_label
            ));
        }
        out
    }
}

/// Shell/support projection for one restricted-mode launch packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RestrictedModeLaunchWedgeDisclosure {
    /// Source packet id.
    pub packet_id: String,
    /// Prototype label token quoted by the wedge inspector.
    pub prototype_label_token: String,
    /// Prototype label display text quoted by the wedge inspector.
    pub prototype_label_display: String,
    /// Effective trust state token.
    pub trust_state_token: String,
    /// Entry transition token.
    pub entry_transition_token: String,
    /// Whether the gate remains visible after open.
    pub trust_gate_visible_after_open: bool,
    /// Allowed rows.
    pub allowed_capabilities: Vec<CapabilityDisclosureRow>,
    /// Blocked, degraded, or approval-needed rows.
    pub blocked_or_review_capabilities: Vec<CapabilityDisclosureRow>,
    /// Primary status label for compact shell chrome.
    pub status_label: String,
}

impl RestrictedModeLaunchWedgeDisclosure {
    /// Project a disclosure from a [`RestrictedModeAlphaPacket`].
    pub fn from_packet(packet: &RestrictedModeAlphaPacket) -> Self {
        Self {
            packet_id: packet.packet_id.clone(),
            prototype_label_token: "prototype_restricted_mode_launch_wedge".to_owned(),
            prototype_label_display: "Prototype - restricted-mode launch wedge".to_owned(),
            trust_state_token: packet.effective_trust_state_token.clone(),
            entry_transition_token: packet.entry_transition_token.clone(),
            trust_gate_visible_after_open: packet.trust_gate_persists_after_open(),
            allowed_capabilities: packet.allowed_rows().into_iter().cloned().collect(),
            blocked_or_review_capabilities: packet
                .blocked_or_review_rows()
                .into_iter()
                .cloned()
                .collect(),
            status_label: status_label(packet).to_owned(),
        }
    }

    /// Render the disclosure as deterministic plaintext.
    pub fn render_plaintext(&self) -> String {
        let mut out = String::new();
        out.push_str(&format!(
            "restricted_mode_launch_wedge: {} state={} transition={} visible_after_open={}\n",
            self.packet_id,
            self.trust_state_token,
            self.entry_transition_token,
            self.trust_gate_visible_after_open
        ));
        out.push_str(&format!("status: {}\n", self.status_label));
        out.push_str("allowed:\n");
        for row in &self.allowed_capabilities {
            out.push_str(&format!(
                "  - {} source={} scope={} recovery={}\n",
                row.surface_family_token,
                row.decision_source.source_label,
                row.scope.scope_label,
                row.recovery_action_tokens.join("|")
            ));
        }
        out.push_str("blocked_or_review:\n");
        for row in &self.blocked_or_review_capabilities {
            out.push_str(&format!(
                "  - {} authority={} source={} scope={} recovery={}\n",
                row.surface_family_token,
                row.authority_token,
                row.decision_source.source_label,
                row.scope.scope_label,
                row.recovery_action_tokens.join("|")
            ));
        }
        out
    }
}

/// Validation error emitted by [`RestrictedModeAlphaPacket::validate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum RestrictedModeValidationError {
    /// Record kind does not match.
    WrongRecordKind,
    /// Schema version does not match.
    WrongSchemaVersion,
    /// Packet or workspace identity is missing.
    MissingWorkspaceIdentity,
    /// Trust-state token does not match the enum value.
    MismatchedTrustStateToken,
    /// Transition token does not match the enum value.
    MismatchedTransitionToken,
    /// Packet has no capability rows.
    MissingCapabilityRows,
    /// Packet does not expose both allowed and blocked/review rows.
    MissingAllowedOrBlockedDisclosure,
    /// Restricted-posture floor is missing.
    RestrictedFloorMissing,
    /// Restricted execution or mutation row is not gated.
    RestrictedExecutionNotGated,
    /// A blocked row lacks source, scope, explanation, or recovery.
    BlockedRowsMissingExplanation,
    /// Trust gate is not sticky after open.
    GateNotStickyAfterOpen,
    /// A guardrail was violated.
    GuardrailViolation,
}

impl RestrictedModeValidationError {
    /// Stable string token.
    pub const fn token(self) -> &'static str {
        match self {
            Self::WrongRecordKind => "wrong_record_kind",
            Self::WrongSchemaVersion => "wrong_schema_version",
            Self::MissingWorkspaceIdentity => "missing_workspace_identity",
            Self::MismatchedTrustStateToken => "mismatched_trust_state_token",
            Self::MismatchedTransitionToken => "mismatched_transition_token",
            Self::MissingCapabilityRows => "missing_capability_rows",
            Self::MissingAllowedOrBlockedDisclosure => "missing_allowed_or_blocked_disclosure",
            Self::RestrictedFloorMissing => "restricted_floor_missing",
            Self::RestrictedExecutionNotGated => "restricted_execution_not_gated",
            Self::BlockedRowsMissingExplanation => "blocked_rows_missing_explanation",
            Self::GateNotStickyAfterOpen => "gate_not_sticky_after_open",
            Self::GuardrailViolation => "guardrail_violation",
        }
    }
}

fn default_capability_gates(
    workspace_root_ref: &str,
    workspace_display_scope: &str,
    trust_state: RestrictedModeTrustStateClass,
    trust_source: &RestrictedModeTrustSource,
) -> Vec<CapabilityDisclosureRow> {
    let scope = |family: LaunchWedgeCapabilityFamily| CapabilityScope {
        scope_ref: format!("scope:{}:{}", workspace_root_ref, family.as_str()),
        scope_label: workspace_display_scope.to_owned(),
        workspace_root_ref: workspace_root_ref.to_owned(),
        policy_epoch_ref: trust_source.policy_epoch_ref.clone(),
    };
    let floor_source = "Restricted mode keeps local reading, editing, search, save, Git inspection, policy read, and support export available.";
    let gated_source = "Restricted mode blocks execution, mutation, external effect, install, and provider capability until trust or approval is granted.";

    let mut rows = vec![
        CapabilityDisclosureRow::new(
            LaunchWedgeCapabilityFamily::WorkspaceOpenRestore,
            CapabilityAuthorityClass::Allowed,
            ExternalEffectClass::LocalOnlyNoExternalEffect,
            trust_source,
            scope(LaunchWedgeCapabilityFamily::WorkspaceOpenRestore),
            floor_source,
            vec![TrustRecoveryActionClass::ContinueRestrictedNoElevation],
        ),
        CapabilityDisclosureRow::new(
            LaunchWedgeCapabilityFamily::EditorReadWrite,
            CapabilityAuthorityClass::Allowed,
            ExternalEffectClass::LocalOnlyNoExternalEffect,
            trust_source,
            scope(LaunchWedgeCapabilityFamily::EditorReadWrite),
            floor_source,
            vec![TrustRecoveryActionClass::ContinueRestrictedNoElevation],
        ),
        CapabilityDisclosureRow::new(
            LaunchWedgeCapabilityFamily::SearchLocal,
            CapabilityAuthorityClass::Allowed,
            ExternalEffectClass::LocalOnlyNoExternalEffect,
            trust_source,
            scope(LaunchWedgeCapabilityFamily::SearchLocal),
            floor_source,
            vec![TrustRecoveryActionClass::ContinueRestrictedNoElevation],
        ),
        CapabilityDisclosureRow::new(
            LaunchWedgeCapabilityFamily::LocalGitRead,
            CapabilityAuthorityClass::ReadOnly,
            ExternalEffectClass::LocalOnlyNoExternalEffect,
            trust_source,
            scope(LaunchWedgeCapabilityFamily::LocalGitRead),
            "Git status, log, branches, and diff remain inspectable; write actions stay gated.",
            vec![TrustRecoveryActionClass::ContinueRestrictedNoElevation],
        ),
        CapabilityDisclosureRow::new(
            LaunchWedgeCapabilityFamily::SupportBundleExport,
            CapabilityAuthorityClass::ReadOnly,
            ExternalEffectClass::LocalOnlyNoExternalEffect,
            trust_source,
            scope(LaunchWedgeCapabilityFamily::SupportBundleExport),
            "Redacted support metadata can be exported with trust posture included.",
            vec![TrustRecoveryActionClass::RouteToSupportBundleExport],
        ),
        CapabilityDisclosureRow::new(
            LaunchWedgeCapabilityFamily::AdminPolicyRead,
            CapabilityAuthorityClass::ReadOnly,
            ExternalEffectClass::LocalOnlyNoExternalEffect,
            trust_source,
            scope(LaunchWedgeCapabilityFamily::AdminPolicyRead),
            "Effective policy summary remains readable so blocked actions can name their source.",
            vec![TrustRecoveryActionClass::OpenPolicyDetails],
        ),
    ];

    for (family, effect) in [
        (
            LaunchWedgeCapabilityFamily::LocalGitWrite,
            ExternalEffectClass::WorkspaceMutation,
        ),
        (
            LaunchWedgeCapabilityFamily::TasksRun,
            ExternalEffectClass::LocalProcessLaunch,
        ),
        (
            LaunchWedgeCapabilityFamily::TerminalManualOpen,
            ExternalEffectClass::LocalProcessLaunch,
        ),
        (
            LaunchWedgeCapabilityFamily::TerminalRepoRecipeLaunch,
            ExternalEffectClass::LocalProcessLaunch,
        ),
        (
            LaunchWedgeCapabilityFamily::DebugLaunch,
            ExternalEffectClass::LocalProcessLaunch,
        ),
        (
            LaunchWedgeCapabilityFamily::NotebookKernelConnect,
            ExternalEffectClass::LocalProcessLaunch,
        ),
        (
            LaunchWedgeCapabilityFamily::NotebookCellExecute,
            ExternalEffectClass::LocalProcessLaunch,
        ),
        (
            LaunchWedgeCapabilityFamily::AiContextRead,
            ExternalEffectClass::NetworkOrProviderEgress,
        ),
        (
            LaunchWedgeCapabilityFamily::AiApplyMutation,
            ExternalEffectClass::WorkspaceMutation,
        ),
        (
            LaunchWedgeCapabilityFamily::AiToolCallMutating,
            ExternalEffectClass::WorkspaceMutation,
        ),
        (
            LaunchWedgeCapabilityFamily::ExtensionActivation,
            ExternalEffectClass::LocalProcessLaunch,
        ),
        (
            LaunchWedgeCapabilityFamily::ExtensionInstall,
            ExternalEffectClass::InstallOrUpdateMutation,
        ),
        (
            LaunchWedgeCapabilityFamily::EnvironmentActivatorRun,
            ExternalEffectClass::LocalProcessLaunch,
        ),
        (
            LaunchWedgeCapabilityFamily::PackageInstallHelper,
            ExternalEffectClass::InstallOrUpdateMutation,
        ),
        (
            LaunchWedgeCapabilityFamily::ConnectedProviderOpen,
            ExternalEffectClass::SecretOrIdentityUse,
        ),
        (
            LaunchWedgeCapabilityFamily::ConnectedProviderToolCall,
            ExternalEffectClass::NetworkOrProviderEgress,
        ),
        (
            LaunchWedgeCapabilityFamily::RemoteAttach,
            ExternalEffectClass::HostedDependency,
        ),
        (
            LaunchWedgeCapabilityFamily::McpServerLaunch,
            ExternalEffectClass::LocalProcessLaunch,
        ),
    ] {
        let authority = authority_for(trust_state, family, effect);
        let recovery_actions = recovery_actions_for(authority, family);
        let mut row = CapabilityDisclosureRow::new(
            family,
            authority,
            effect,
            trust_source,
            scope(family),
            gated_explanation(trust_state, family, gated_source),
            recovery_actions,
        );
        if effect.requires_hosted_dependency_disclosure() {
            row.hosted_dependency_disclosed = true;
        }
        rows.push(row);
    }

    rows
}

const fn authority_for(
    trust_state: RestrictedModeTrustStateClass,
    family: LaunchWedgeCapabilityFamily,
    effect: ExternalEffectClass,
) -> CapabilityAuthorityClass {
    if trust_state.trusted_execution_available() {
        if effect.requires_install_review() {
            return CapabilityAuthorityClass::BlockedPendingApproval;
        }
        if matches!(
            family,
            LaunchWedgeCapabilityFamily::AiApplyMutation
                | LaunchWedgeCapabilityFamily::AiToolCallMutating
                | LaunchWedgeCapabilityFamily::RemoteAttach
                | LaunchWedgeCapabilityFamily::ConnectedProviderToolCall
        ) {
            return CapabilityAuthorityClass::ApprovalRequiredPerInvocation;
        }
        return CapabilityAuthorityClass::Allowed;
    }
    if matches!(
        trust_state,
        RestrictedModeTrustStateClass::TrustedPolicyDegraded
    ) {
        if matches!(
            family,
            LaunchWedgeCapabilityFamily::AiApplyMutation
                | LaunchWedgeCapabilityFamily::AiToolCallMutating
                | LaunchWedgeCapabilityFamily::ConnectedProviderToolCall
        ) {
            return CapabilityAuthorityClass::ApprovalRequiredPerInvocation;
        }
        if effect.requires_install_review() {
            return CapabilityAuthorityClass::BlockedPendingApproval;
        }
        return CapabilityAuthorityClass::Allowed;
    }
    match family {
        LaunchWedgeCapabilityFamily::AiContextRead => CapabilityAuthorityClass::DegradedPreviewOnly,
        LaunchWedgeCapabilityFamily::ConnectedProviderOpen
        | LaunchWedgeCapabilityFamily::ConnectedProviderToolCall
        | LaunchWedgeCapabilityFamily::RemoteAttach
        | LaunchWedgeCapabilityFamily::TerminalManualOpen => {
            CapabilityAuthorityClass::BlockedPendingApproval
        }
        _ => CapabilityAuthorityClass::BlockedPendingTrust,
    }
}

fn recovery_actions_for(
    authority: CapabilityAuthorityClass,
    family: LaunchWedgeCapabilityFamily,
) -> Vec<TrustRecoveryActionClass> {
    match authority {
        CapabilityAuthorityClass::Allowed | CapabilityAuthorityClass::ReadOnly => {
            vec![TrustRecoveryActionClass::ContinueRestrictedNoElevation]
        }
        CapabilityAuthorityClass::DegradedPreviewOnly => vec![
            TrustRecoveryActionClass::UseLocalReadOnlyAlternative,
            TrustRecoveryActionClass::RequestTrustGrantSessionOnly,
            TrustRecoveryActionClass::OpenCapabilityDetails,
        ],
        CapabilityAuthorityClass::BlockedPendingApproval
        | CapabilityAuthorityClass::ApprovalRequiredPerInvocation => vec![
            TrustRecoveryActionClass::RequestApprovalTicket,
            TrustRecoveryActionClass::ContinueRestrictedNoElevation,
            TrustRecoveryActionClass::RouteToSupportBundleExport,
        ],
        CapabilityAuthorityClass::PolicyDenied => vec![
            TrustRecoveryActionClass::OpenPolicyDetails,
            TrustRecoveryActionClass::RequestAdminPolicyChange,
            TrustRecoveryActionClass::RouteToSupportBundleExport,
        ],
        CapabilityAuthorityClass::QuarantineDenied => vec![
            TrustRecoveryActionClass::OpenCapabilityDetails,
            TrustRecoveryActionClass::RouteToSupportBundleExport,
        ],
        CapabilityAuthorityClass::BlockedPendingTrust => {
            if matches!(
                family,
                LaunchWedgeCapabilityFamily::ExtensionInstall
                    | LaunchWedgeCapabilityFamily::PackageInstallHelper
            ) {
                vec![
                    TrustRecoveryActionClass::RequestTrustGrantSessionOnly,
                    TrustRecoveryActionClass::RequestApprovalTicket,
                    TrustRecoveryActionClass::ContinueRestrictedNoElevation,
                ]
            } else {
                vec![
                    TrustRecoveryActionClass::RequestTrustGrantSessionOnly,
                    TrustRecoveryActionClass::ContinueRestrictedNoElevation,
                ]
            }
        }
        CapabilityAuthorityClass::NotApplicable => Vec::new(),
    }
}

fn gated_explanation(
    trust_state: RestrictedModeTrustStateClass,
    family: LaunchWedgeCapabilityFamily,
    default_label: &str,
) -> String {
    match family {
        LaunchWedgeCapabilityFamily::AiContextRead => {
            "AI context can be inspected locally, but provider dispatch waits for trust and policy review.".to_owned()
        }
        LaunchWedgeCapabilityFamily::ExtensionInstall
        | LaunchWedgeCapabilityFamily::PackageInstallHelper => {
            "Install or update mutation requires trust plus an explicit review before code or scripts can run.".to_owned()
        }
        LaunchWedgeCapabilityFamily::ConnectedProviderOpen
        | LaunchWedgeCapabilityFamily::ConnectedProviderToolCall
        | LaunchWedgeCapabilityFamily::RemoteAttach => {
            "External provider, identity, or hosted target capability requires approval and boundary disclosure.".to_owned()
        }
        _ if trust_state == RestrictedModeTrustStateClass::TrustedPolicyDegraded => {
            "Policy narrowed this trusted workspace; the row stays visible until policy changes.".to_owned()
        }
        _ => default_label.to_owned(),
    }
}

fn escalation_cues_for(trust_state: RestrictedModeTrustStateClass) -> Vec<TrustEscalationCueClass> {
    if trust_state.restricted_floor_applies() {
        vec![
            TrustEscalationCueClass::RequestTrustGrantSessionOnly,
            TrustEscalationCueClass::RequestApprovalTicket,
            TrustEscalationCueClass::RouteToSupportBundleExport,
            TrustEscalationCueClass::ContinueRestrictedNoElevation,
        ]
    } else if matches!(
        trust_state,
        RestrictedModeTrustStateClass::TrustedPolicyDegraded
    ) {
        vec![
            TrustEscalationCueClass::RequestApprovalTicket,
            TrustEscalationCueClass::RequestAdminPolicyChange,
            TrustEscalationCueClass::RouteToSupportBundleExport,
        ]
    } else {
        vec![TrustEscalationCueClass::RouteToSupportBundleExport]
    }
}

const fn audit_event_for(transition: RestrictedModeEntryTransitionClass) -> TrustAuditEventClass {
    match transition {
        RestrictedModeEntryTransitionClass::GrantTrustSession
        | RestrictedModeEntryTransitionClass::GrantTrustRemembered
        | RestrictedModeEntryTransitionClass::GrantTrustAdminPrebinding => {
            TrustAuditEventClass::WorkspaceTrustGranted
        }
        RestrictedModeEntryTransitionClass::RevokeTrust => {
            TrustAuditEventClass::WorkspaceTrustRevoked
        }
        RestrictedModeEntryTransitionClass::PolicyNarrowToDegraded => {
            TrustAuditEventClass::WorkspaceTrustPolicyNarrowed
        }
        RestrictedModeEntryTransitionClass::SafeModeWorkspaceRestricted
        | RestrictedModeEntryTransitionClass::ExtensionQuarantineRestricted => {
            TrustAuditEventClass::WorkspaceTrustRecoveryApplied
        }
        _ => TrustAuditEventClass::WorkspaceTrustStateResolved,
    }
}

const fn required_floor_families() -> &'static [LaunchWedgeCapabilityFamily] {
    &[
        LaunchWedgeCapabilityFamily::WorkspaceOpenRestore,
        LaunchWedgeCapabilityFamily::EditorReadWrite,
        LaunchWedgeCapabilityFamily::SearchLocal,
        LaunchWedgeCapabilityFamily::LocalGitRead,
        LaunchWedgeCapabilityFamily::SupportBundleExport,
        LaunchWedgeCapabilityFamily::AdminPolicyRead,
    ]
}

const fn status_label(packet: &RestrictedModeAlphaPacket) -> &'static str {
    if packet.effective_trust_state.restricted_floor_applies() {
        "Restricted mode: local work is available; execution and mutation stay gated."
    } else if matches!(
        packet.effective_trust_state,
        RestrictedModeTrustStateClass::TrustedPolicyDegraded
    ) {
        "Trusted, narrowed by policy: affected capabilities need review."
    } else {
        "Trusted: execution-capable rows still show review requirements where needed."
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn restricted_packet() -> RestrictedModeAlphaPacket {
        RestrictedModeAlphaPacket::stage_launch_wedge(StageRestrictedModeLaunchRequest {
            packet_id: "restricted-mode:launch:seed",
            workspace_root_ref: "workspace:demo",
            workspace_display_scope: "Current demo workspace",
            identity_mode: IdentityModeAlias::AccountFreeLocal,
            prior_trust_state: Some(RestrictedModeTrustStateClass::UntrustedUnknown),
            effective_trust_state: RestrictedModeTrustStateClass::Restricted,
            entry_transition: RestrictedModeEntryTransitionClass::OpenInRestrictedMode,
            source_class: TrustDecisionSourceClass::LocalUser,
            source_ref: "trust-decision:local:restricted-open",
            source_label: "Local user chose restricted mode.",
            reason_class: TrustReasonClass::ExplicitUserDecline,
            remembered_decision_scope: RememberedDecisionScopeClass::NeverRemembered,
            policy_epoch_ref: Some("policy-epoch:local"),
            source_reason_refs: &["trust-source:first-open"],
            recovery_action_ref: None,
            issued_at: "2026-05-13T22:00:00Z",
        })
    }

    #[test]
    fn restricted_packet_discloses_allowed_and_blocked_capabilities() {
        let packet = restricted_packet();
        assert!(packet.validate().is_empty());
        assert!(packet.restricted_floor_available());
        assert!(packet.has_allowed_and_blocked_disclosure());
        assert!(packet.restricted_execution_and_mutation_are_gated());
        assert!(packet.blocked_rows_explain_source_scope_and_recovery());
        assert!(packet.trust_gate_persists_after_open());
        assert!(packet.guardrails_hold());

        let task = packet
            .capability_gates
            .iter()
            .find(|row| row.surface_family == LaunchWedgeCapabilityFamily::TasksRun)
            .expect("tasks row");
        assert_eq!(
            task.authority,
            CapabilityAuthorityClass::BlockedPendingTrust
        );
        assert!(task
            .recovery_actions
            .contains(&TrustRecoveryActionClass::RequestTrustGrantSessionOnly));

        let editor = packet
            .capability_gates
            .iter()
            .find(|row| row.surface_family == LaunchWedgeCapabilityFamily::EditorReadWrite)
            .expect("editor row");
        assert_eq!(editor.authority, CapabilityAuthorityClass::Allowed);
    }

    #[test]
    fn disclosure_projection_keeps_gate_visible_after_open() {
        let packet = restricted_packet();
        let disclosure = packet.launch_wedge_disclosure();
        assert_eq!(
            disclosure.prototype_label_token,
            "prototype_restricted_mode_launch_wedge"
        );
        assert!(disclosure.trust_gate_visible_after_open);
        assert!(!disclosure.allowed_capabilities.is_empty());
        assert!(!disclosure.blocked_or_review_capabilities.is_empty());
        let plaintext = disclosure.render_plaintext();
        assert!(plaintext.contains("state=restricted"));
        assert!(plaintext.contains("blocked_or_review"));
        assert!(plaintext.contains("tasks_run"));
    }
}
