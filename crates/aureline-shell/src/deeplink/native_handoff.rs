//! Native desktop handoff review and ownership cues.
//!
//! This module consumes platform `deep_link_intent_record` and
//! `native_file_affordance_case_record` shapes and projects the shell-side
//! review truth the user must see before an OS-originated action executes:
//! source/origin, resulting command class, target identity, owning channel and
//! build, trust/profile boundary, replay posture, and recovery actions. It is
//! intentionally pure; callers can log or render the review records without
//! launching files, mutating state, or reusing external authority.

use std::path::Path;

use aureline_commands::invocation::now_rfc3339;
use serde::{Deserialize, Serialize};

/// Stable record kind for [`NativeBoundaryHandoffPacket`] payloads.
pub const NATIVE_BOUNDARY_HANDOFF_PACKET_RECORD_KIND: &str =
    "native_boundary_handoff_packet_record";

/// Stable schema version for native boundary handoff packets.
pub const NATIVE_BOUNDARY_HANDOFF_PACKET_SCHEMA_VERSION: u32 = 1;

/// Stable record kind for [`NativeBoundaryHandoffReviewRecord`] payloads.
pub const NATIVE_BOUNDARY_HANDOFF_REVIEW_RECORD_KIND: &str =
    "native_boundary_handoff_review_record";

/// Stable record kind for [`NativeFileHandoffReviewRecord`] payloads.
pub const NATIVE_FILE_HANDOFF_REVIEW_RECORD_KIND: &str = "native_file_handoff_review_record";

/// OS-facing source surface that produced a native entry request.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SourceSurfaceClass {
    /// Open request from the OS shell.
    SystemOpen,
    /// File association activation.
    FileAssociation,
    /// Open-with activation.
    OpenWith,
    /// Reveal-in-system-shell activation.
    RevealInSystemShell,
    /// Native open dialog result.
    NativeOpenDialog,
    /// Native save dialog result.
    NativeSaveDialog,
    /// Dock/taskbar recent item.
    DockTaskbarRecent,
    /// Dock/taskbar jump-list style action.
    DockTaskbarJumpAction,
    /// OS notification click.
    OsNotificationClick,
    /// OS badge activation.
    OsBadgeActivation,
    /// System share target.
    SystemShareTarget,
    /// Copy path or permalink surface.
    CopyPathOrPermalink,
    /// Drag/drop open.
    DragDropOpen,
    /// Open-from-terminal request.
    OpenFromTerminal,
    /// Default-browser callback.
    DefaultBrowserCallback,
    /// Protocol-handler invocation.
    ProtocolHandler,
    /// Companion handoff return.
    CompanionHandoffReturn,
}

impl SourceSurfaceClass {
    /// Returns the stable token for this source surface.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SystemOpen => "system_open",
            Self::FileAssociation => "file_association",
            Self::OpenWith => "open_with",
            Self::RevealInSystemShell => "reveal_in_system_shell",
            Self::NativeOpenDialog => "native_open_dialog",
            Self::NativeSaveDialog => "native_save_dialog",
            Self::DockTaskbarRecent => "dock_taskbar_recent",
            Self::DockTaskbarJumpAction => "dock_taskbar_jump_action",
            Self::OsNotificationClick => "os_notification_click",
            Self::OsBadgeActivation => "os_badge_activation",
            Self::SystemShareTarget => "system_share_target",
            Self::CopyPathOrPermalink => "copy_path_or_permalink",
            Self::DragDropOpen => "drag_drop_open",
            Self::OpenFromTerminal => "open_from_terminal",
            Self::DefaultBrowserCallback => "default_browser_callback",
            Self::ProtocolHandler => "protocol_handler",
            Self::CompanionHandoffReturn => "companion_handoff_return",
        }
    }

    /// True when the surface is a summary-only OS affordance.
    pub const fn is_summary_only(self) -> bool {
        matches!(
            self,
            Self::DockTaskbarRecent
                | Self::DockTaskbarJumpAction
                | Self::OsNotificationClick
                | Self::OsBadgeActivation
        )
    }
}

/// Origin class for a native handoff request.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum OriginClass {
    /// Originated in the local OS shell.
    OsShell,
    /// Returned from the system default browser.
    SystemDefaultBrowser,
    /// First-party web origin.
    FirstPartyWeb,
    /// Trusted companion client.
    TrustedCompanion,
    /// External provider origin.
    ExternalProvider,
    /// Collaboration service origin.
    CollaborationService,
    /// Local CLI origin.
    LocalCli,
    /// Installer or update flow origin.
    InstallerOrUpdateFlow,
    /// Unknown or untrusted origin.
    UnknownUntrusted,
}

impl OriginClass {
    /// Returns the stable token for this origin class.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::OsShell => "os_shell",
            Self::SystemDefaultBrowser => "system_default_browser",
            Self::FirstPartyWeb => "first_party_web",
            Self::TrustedCompanion => "trusted_companion",
            Self::ExternalProvider => "external_provider",
            Self::CollaborationService => "collaboration_service",
            Self::LocalCli => "local_cli",
            Self::InstallerOrUpdateFlow => "installer_or_update_flow",
            Self::UnknownUntrusted => "unknown_untrusted",
        }
    }
}

/// Product route class resolved from the native handoff.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RouteClass {
    /// Open a local file.
    LocalFileOpen,
    /// Open a workspace.
    WorkspaceOpen,
    /// Open a review or work item.
    ReviewOrWorkItem,
    /// Complete or restart an auth callback.
    AuthCallback,
    /// Join or inspect a collaboration session.
    CollaborationSessionJoin,
    /// Resume a managed workspace.
    ManagedWorkspaceResume,
    /// Invoke a command.
    CommandInvocation,
    /// Return from an external browser object.
    ExternalBrowserReturn,
    /// Open settings or policy review.
    SettingsOrPolicyReview,
    /// Open support or incident context.
    SupportOrIncident,
    /// Hand off to provider console.
    ProviderConsoleHandoff,
    /// Recover an unavailable target.
    UnavailableTargetRecovery,
}

impl RouteClass {
    /// Returns the stable token for this route class.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LocalFileOpen => "local_file_open",
            Self::WorkspaceOpen => "workspace_open",
            Self::ReviewOrWorkItem => "review_or_work_item",
            Self::AuthCallback => "auth_callback",
            Self::CollaborationSessionJoin => "collaboration_session_join",
            Self::ManagedWorkspaceResume => "managed_workspace_resume",
            Self::CommandInvocation => "command_invocation",
            Self::ExternalBrowserReturn => "external_browser_return",
            Self::SettingsOrPolicyReview => "settings_or_policy_review",
            Self::SupportOrIncident => "support_or_incident",
            Self::ProviderConsoleHandoff => "provider_console_handoff",
            Self::UnavailableTargetRecovery => "unavailable_target_recovery",
        }
    }
}

/// Canonical target kind bound by a handoff.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TargetKind {
    /// Local file target.
    LocalFile,
    /// Local folder target.
    LocalFolder,
    /// Workspace manifest target.
    WorkspaceManifest,
    /// Workspace root target.
    WorkspaceRoot,
    /// Recent-work entry target.
    RecentWorkEntry,
    /// Review thread target.
    ReviewThread,
    /// Work item target.
    WorkItem,
    /// Incident target.
    Incident,
    /// Auth session target.
    AuthSession,
    /// Collaboration session target.
    CollaborationSession,
    /// Managed workspace target.
    ManagedWorkspace,
    /// Command target.
    CommandTarget,
    /// Route object target.
    RouteObject,
    /// Notification event target.
    NotificationEvent,
    /// Support packet target.
    SupportPacket,
    /// Settings or policy surface target.
    SettingsOrPolicySurface,
    /// Provider console target.
    ProviderConsoleTarget,
    /// Unknown target.
    UnknownTarget,
}

impl TargetKind {
    /// Returns the stable token for this target kind.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LocalFile => "local_file",
            Self::LocalFolder => "local_folder",
            Self::WorkspaceManifest => "workspace_manifest",
            Self::WorkspaceRoot => "workspace_root",
            Self::RecentWorkEntry => "recent_work_entry",
            Self::ReviewThread => "review_thread",
            Self::WorkItem => "work_item",
            Self::Incident => "incident",
            Self::AuthSession => "auth_session",
            Self::CollaborationSession => "collaboration_session",
            Self::ManagedWorkspace => "managed_workspace",
            Self::CommandTarget => "command_target",
            Self::RouteObject => "route_object",
            Self::NotificationEvent => "notification_event",
            Self::SupportPacket => "support_packet",
            Self::SettingsOrPolicySurface => "settings_or_policy_surface",
            Self::ProviderConsoleTarget => "provider_console_target",
            Self::UnknownTarget => "unknown_target",
        }
    }
}

/// Target freshness class.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FreshnessClass {
    /// Current live authority.
    AuthoritativeLive,
    /// Warm cached state.
    WarmCached,
    /// Degraded cached state.
    DegradedCached,
    /// Stale state.
    Stale,
    /// Unverified state.
    Unverified,
}

impl FreshnessClass {
    /// Returns the stable token for this freshness class.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::AuthoritativeLive => "authoritative_live",
            Self::WarmCached => "warm_cached",
            Self::DegradedCached => "degraded_cached",
            Self::Stale => "stale",
            Self::Unverified => "unverified",
        }
    }
}

/// Availability class for the handoff target.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TargetAvailabilityClass {
    /// Target is exactly available.
    ExactAvailable,
    /// Target is available read-only.
    AvailableReadOnly,
    /// Target is available but stale.
    StaleAvailable,
    /// Target moved or alias changed.
    MovedOrAliasChanged,
    /// Target is missing or unmounted.
    MissingOrUnmounted,
    /// Target is blocked by policy.
    BlockedByPolicy,
    /// Target requires authentication.
    AuthRequired,
    /// Remote target is unreachable.
    RemoteUnreachable,
    /// Target expired.
    Expired,
    /// Target is ambiguous.
    Ambiguous,
    /// Target availability is unknown.
    Unknown,
}

impl TargetAvailabilityClass {
    /// Returns the stable token for this availability class.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ExactAvailable => "exact_available",
            Self::AvailableReadOnly => "available_read_only",
            Self::StaleAvailable => "stale_available",
            Self::MovedOrAliasChanged => "moved_or_alias_changed",
            Self::MissingOrUnmounted => "missing_or_unmounted",
            Self::BlockedByPolicy => "blocked_by_policy",
            Self::AuthRequired => "auth_required",
            Self::RemoteUnreachable => "remote_unreachable",
            Self::Expired => "expired",
            Self::Ambiguous => "ambiguous",
            Self::Unknown => "unknown",
        }
    }

    /// True when exact execution cannot proceed directly.
    pub const fn requires_recovery(self) -> bool {
        !matches!(self, Self::ExactAvailable)
    }

    /// True when a placeholder recovery card is required.
    pub const fn requires_placeholder(self) -> bool {
        matches!(
            self,
            Self::MovedOrAliasChanged
                | Self::MissingOrUnmounted
                | Self::RemoteUnreachable
                | Self::BlockedByPolicy
                | Self::Expired
                | Self::Ambiguous
                | Self::Unknown
        )
    }
}

/// Requested action class from the native handoff.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RequestedActionClass {
    /// Inspect only.
    InspectOnly,
    /// Reveal only.
    RevealOnly,
    /// Open existing context.
    OpenExistingContext,
    /// Create or add context.
    CreateOrAddContext,
    /// Join collaboration presence.
    JoinPresence,
    /// Resume session.
    ResumeSession,
    /// Auth return.
    AuthReturn,
    /// Retry or reconnect.
    RetryOrReconnect,
    /// Acknowledge notification.
    AcknowledgeNotification,
    /// Mutating command request.
    MutatingCommandRequest,
    /// Privileged authority widening.
    PrivilegedAuthorityWidening,
}

impl RequestedActionClass {
    /// Returns the stable token for this action class.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::InspectOnly => "inspect_only",
            Self::RevealOnly => "reveal_only",
            Self::OpenExistingContext => "open_existing_context",
            Self::CreateOrAddContext => "create_or_add_context",
            Self::JoinPresence => "join_presence",
            Self::ResumeSession => "resume_session",
            Self::AuthReturn => "auth_return",
            Self::RetryOrReconnect => "retry_or_reconnect",
            Self::AcknowledgeNotification => "acknowledge_notification",
            Self::MutatingCommandRequest => "mutating_command_request",
            Self::PrivilegedAuthorityWidening => "privileged_authority_widening",
        }
    }

    /// True when the action cannot run from an OS summary surface.
    pub const fn is_high_risk(self) -> bool {
        matches!(
            self,
            Self::CreateOrAddContext
                | Self::JoinPresence
                | Self::ResumeSession
                | Self::MutatingCommandRequest
                | Self::PrivilegedAuthorityWidening
        )
    }
}

/// Authority delta caused by the native handoff.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AuthorityDeltaClass {
    /// No authority delta.
    None,
    /// Trust boundary crossing.
    TrustBoundaryCrossing,
    /// Policy boundary crossing.
    PolicyBoundaryCrossing,
    /// Auth scope widening.
    AuthScopeWidening,
    /// Remote authority rebind.
    RemoteAuthorityRebind,
    /// Collaboration presence widening.
    CollaborationPresenceWidening,
    /// External visibility widening.
    ExternalVisibilityWidening,
    /// Destructive or mutating action.
    DestructiveOrMutating,
    /// Unknown delta requiring review.
    UnknownRequiresReview,
}

impl AuthorityDeltaClass {
    /// Returns the stable token for this authority delta.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::None => "none",
            Self::TrustBoundaryCrossing => "trust_boundary_crossing",
            Self::PolicyBoundaryCrossing => "policy_boundary_crossing",
            Self::AuthScopeWidening => "auth_scope_widening",
            Self::RemoteAuthorityRebind => "remote_authority_rebind",
            Self::CollaborationPresenceWidening => "collaboration_presence_widening",
            Self::ExternalVisibilityWidening => "external_visibility_widening",
            Self::DestructiveOrMutating => "destructive_or_mutating",
            Self::UnknownRequiresReview => "unknown_requires_review",
        }
    }

    /// True when the delta requires native review.
    pub const fn requires_review(self) -> bool {
        !matches!(self, Self::None)
    }
}

/// Trust review requirement for an admitted or denied handoff.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TrustReviewRequirement {
    /// No review required.
    NoReviewRequired,
    /// Review before open.
    ReviewRequiredBeforeOpen,
    /// Review before command.
    ReviewRequiredBeforeCommand,
    /// Policy review required.
    PolicyReviewRequired,
    /// Auth step-up required.
    AuthStepUpRequired,
    /// Tenant-scope confirmation required.
    TenantScopeConfirmationRequired,
    /// Blocked until policy changes.
    BlockedRequiresPolicyChange,
}

impl TrustReviewRequirement {
    /// Returns the stable token for this trust review requirement.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NoReviewRequired => "no_review_required",
            Self::ReviewRequiredBeforeOpen => "review_required_before_open",
            Self::ReviewRequiredBeforeCommand => "review_required_before_command",
            Self::PolicyReviewRequired => "policy_review_required",
            Self::AuthStepUpRequired => "auth_step_up_required",
            Self::TenantScopeConfirmationRequired => "tenant_scope_confirmation_required",
            Self::BlockedRequiresPolicyChange => "blocked_requires_policy_change",
        }
    }

    /// True when execution must pass through native review.
    pub const fn requires_review(self) -> bool {
        !matches!(self, Self::NoReviewRequired)
    }
}

/// Policy resolution class for the handoff.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PolicyResolutionClass {
    /// Allowed under current policy.
    AllowedCurrentPolicy,
    /// Allowed after review.
    AllowedAfterReview,
    /// Denied by policy.
    DeniedPolicy,
    /// Denied by workspace trust.
    DeniedWorkspaceTrust,
    /// Denied because scope is missing.
    DeniedScopeMissing,
    /// Denied because origin is untrusted.
    DeniedOriginUntrusted,
    /// Denied by replay policy.
    DeniedReplay,
    /// Denied because target is unavailable.
    DeniedTargetUnavailable,
    /// Denied because handler ownership is invalid.
    DeniedHandlerOwnership,
    /// Degraded to a placeholder.
    DegradedOpenPlaceholder,
}

impl PolicyResolutionClass {
    /// Returns the stable token for this policy resolution.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::AllowedCurrentPolicy => "allowed_current_policy",
            Self::AllowedAfterReview => "allowed_after_review",
            Self::DeniedPolicy => "denied_policy",
            Self::DeniedWorkspaceTrust => "denied_workspace_trust",
            Self::DeniedScopeMissing => "denied_scope_missing",
            Self::DeniedOriginUntrusted => "denied_origin_untrusted",
            Self::DeniedReplay => "denied_replay",
            Self::DeniedTargetUnavailable => "denied_target_unavailable",
            Self::DeniedHandlerOwnership => "denied_handler_ownership",
            Self::DegradedOpenPlaceholder => "degraded_open_placeholder",
        }
    }

    /// True when exact execution is denied.
    pub const fn is_denied(self) -> bool {
        matches!(
            self,
            Self::DeniedPolicy
                | Self::DeniedWorkspaceTrust
                | Self::DeniedScopeMissing
                | Self::DeniedOriginUntrusted
                | Self::DeniedReplay
                | Self::DeniedTargetUnavailable
                | Self::DeniedHandlerOwnership
        )
    }
}

/// Replay posture for the handoff.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ReplayPosture {
    /// Single-use intent.
    SingleUse,
    /// Bounded reuse.
    BoundedReuse,
    /// Read-only resumable.
    ReadOnlyResumable,
    /// Replay denied because consumed.
    ReplayDeniedConsumed,
    /// Replay denied because expired.
    ReplayDeniedExpired,
    /// Replay denied because policy epoch changed.
    ReplayDeniedPolicyEpochChanged,
    /// Replay denied because target drifted.
    ReplayDeniedTargetDrifted,
    /// Replay denied because origin mismatched.
    ReplayDeniedOriginMismatch,
}

impl ReplayPosture {
    /// Returns the stable token for this replay posture.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SingleUse => "single_use",
            Self::BoundedReuse => "bounded_reuse",
            Self::ReadOnlyResumable => "read_only_resumable",
            Self::ReplayDeniedConsumed => "replay_denied_consumed",
            Self::ReplayDeniedExpired => "replay_denied_expired",
            Self::ReplayDeniedPolicyEpochChanged => "replay_denied_policy_epoch_changed",
            Self::ReplayDeniedTargetDrifted => "replay_denied_target_drifted",
            Self::ReplayDeniedOriginMismatch => "replay_denied_origin_mismatch",
        }
    }

    /// True when replay is denied.
    pub const fn is_denied(self) -> bool {
        matches!(
            self,
            Self::ReplayDeniedConsumed
                | Self::ReplayDeniedExpired
                | Self::ReplayDeniedPolicyEpochChanged
                | Self::ReplayDeniedTargetDrifted
                | Self::ReplayDeniedOriginMismatch
        )
    }
}

/// Fallback class for a degraded or denied handoff.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FallbackClass {
    /// Open intent review sheet.
    OpenIntentReviewSheet,
    /// Open read-only placeholder.
    OpenReadOnlyPlaceholder,
    /// Open cached context.
    OpenCachedContext,
    /// Locate missing target.
    LocateMissingTarget,
    /// Continue local-only.
    ContinueLocalOnly,
    /// Open activity center.
    OpenActivityCenter,
    /// Open default browser.
    OpenDefaultBrowser,
    /// Deny with explanation.
    DenyWithExplanation,
    /// Export context.
    ExportContext,
    /// No fallback available.
    NoFallbackAvailable,
}

impl FallbackClass {
    /// Returns the stable token for this fallback class.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::OpenIntentReviewSheet => "open_intent_review_sheet",
            Self::OpenReadOnlyPlaceholder => "open_read_only_placeholder",
            Self::OpenCachedContext => "open_cached_context",
            Self::LocateMissingTarget => "locate_missing_target",
            Self::ContinueLocalOnly => "continue_local_only",
            Self::OpenActivityCenter => "open_activity_center",
            Self::OpenDefaultBrowser => "open_default_browser",
            Self::DenyWithExplanation => "deny_with_explanation",
            Self::ExportContext => "export_context",
            Self::NoFallbackAvailable => "no_fallback_available",
        }
    }
}

/// Handler ownership class for OS entry points.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum HandlerOwnershipClass {
    /// Machine-global registered handler.
    MachineGlobalRegistered,
    /// Current-user registered handler.
    CurrentUserRegistered,
    /// Portable local-only handler.
    PortableLocalOnly,
    /// Managed policy owns handler.
    ManagedPolicyOwned,
    /// No handler ownership.
    NoHandlerOwnership,
    /// Conflict with unknown owner.
    ConflictUnknownOwner,
}

impl HandlerOwnershipClass {
    /// Returns the stable token for this handler ownership class.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::MachineGlobalRegistered => "machine_global_registered",
            Self::CurrentUserRegistered => "current_user_registered",
            Self::PortableLocalOnly => "portable_local_only",
            Self::ManagedPolicyOwned => "managed_policy_owned",
            Self::NoHandlerOwnership => "no_handler_ownership",
            Self::ConflictUnknownOwner => "conflict_unknown_owner",
        }
    }

    /// True when the handler owner cannot be trusted.
    pub const fn is_conflict(self) -> bool {
        matches!(self, Self::ConflictUnknownOwner | Self::NoHandlerOwnership)
    }
}

/// Ownership-change review state for handler claims.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum OwnershipChangeReviewState {
    /// No ownership change.
    NoChange,
    /// Preview required before ownership changes.
    PreviewRequiredBeforeChange,
    /// Blocked by policy.
    BlockedByPolicy,
    /// Accepted after review.
    AcceptedAfterReview,
    /// Denied because of conflict.
    DeniedConflict,
}

impl OwnershipChangeReviewState {
    /// Returns the stable token for this ownership review state.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NoChange => "no_change",
            Self::PreviewRequiredBeforeChange => "preview_required_before_change",
            Self::BlockedByPolicy => "blocked_by_policy",
            Self::AcceptedAfterReview => "accepted_after_review",
            Self::DeniedConflict => "denied_conflict",
        }
    }

    /// True when the owning build/channel must be shown before execution.
    pub const fn requires_owner_preview(self) -> bool {
        matches!(
            self,
            Self::PreviewRequiredBeforeChange | Self::BlockedByPolicy | Self::DeniedConflict
        )
    }
}

/// Trust state at handoff review time.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TrustState {
    /// Trusted.
    Trusted,
    /// Restricted.
    Restricted,
    /// Pending evaluation.
    PendingEvaluation,
    /// Unknown.
    Unknown,
}

impl TrustState {
    /// Returns the stable token for this trust state.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Trusted => "trusted",
            Self::Restricted => "restricted",
            Self::PendingEvaluation => "pending_evaluation",
            Self::Unknown => "unknown",
        }
    }
}

/// Review surface selected for a native handoff.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum NativeReviewSurfaceClass {
    /// Exact open through the canonical command path.
    ExactOpen,
    /// Reviewed intent sheet before execution.
    ReviewedIntentSheet,
    /// Placeholder recovery card.
    PlaceholderRecoveryCard,
    /// Denial and recovery sheet.
    DeniedRecoverySheet,
    /// Product-owned native review surface.
    ProductOwnedNativeReview,
}

impl NativeReviewSurfaceClass {
    /// Returns the stable token for this review surface.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ExactOpen => "exact_open",
            Self::ReviewedIntentSheet => "reviewed_intent_sheet",
            Self::PlaceholderRecoveryCard => "placeholder_recovery_card",
            Self::DeniedRecoverySheet => "denied_recovery_sheet",
            Self::ProductOwnedNativeReview => "product_owned_native_review",
        }
    }
}

/// Native file affordance class.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum NativeFileAffordanceClass {
    /// Native open dialog.
    NativeOpenDialog,
    /// Native save dialog.
    NativeSaveDialog,
    /// Reveal in system shell.
    RevealInSystemShell,
    /// Open from terminal.
    OpenFromTerminal,
    /// File association.
    FileAssociation,
    /// Drag-drop.
    DragDrop,
    /// Clipboard.
    Clipboard,
    /// Browser-originated open request.
    BrowserOriginatedOpenRequest,
}

impl NativeFileAffordanceClass {
    /// Returns the stable token for this native file affordance.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NativeOpenDialog => "native_open_dialog",
            Self::NativeSaveDialog => "native_save_dialog",
            Self::RevealInSystemShell => "reveal_in_system_shell",
            Self::OpenFromTerminal => "open_from_terminal",
            Self::FileAssociation => "file_association",
            Self::DragDrop => "drag_drop",
            Self::Clipboard => "clipboard",
            Self::BrowserOriginatedOpenRequest => "browser_originated_open_request",
        }
    }
}

/// Native file target kind.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum NativeFileTargetKind {
    /// Local file target.
    LocalFile,
    /// Network share target.
    NetworkShare,
    /// Removable volume target.
    RemovableVolume,
    /// Generated artifact target.
    GeneratedArtifact,
    /// Managed file target.
    ManagedFile,
    /// External URI or out-of-scope target.
    ExternalUriOrOutOfScope,
    /// Unknown target.
    Unknown,
}

impl NativeFileTargetKind {
    /// Returns the stable token for this native file target kind.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LocalFile => "local_file",
            Self::NetworkShare => "network_share",
            Self::RemovableVolume => "removable_volume",
            Self::GeneratedArtifact => "generated_artifact",
            Self::ManagedFile => "managed_file",
            Self::ExternalUriOrOutOfScope => "external_uri_or_out_of_scope",
            Self::Unknown => "unknown",
        }
    }
}

/// Write posture bound by a native file affordance.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum WritePostureClass {
    /// Writes are allowed.
    WritesAllowed,
    /// Read-only.
    ReadOnly,
    /// Blocked by policy or read-only state.
    BlockedPolicyOrReadOnly,
    /// Blocked because target is ambiguous or wrong.
    BlockedAmbiguousOrWrongTarget,
    /// Blocked until review.
    BlockedRequiresReview,
    /// Blocked until retarget.
    BlockedRequiresRetarget,
}

impl WritePostureClass {
    /// Returns the stable token for this write posture.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WritesAllowed => "writes_allowed",
            Self::ReadOnly => "read_only",
            Self::BlockedPolicyOrReadOnly => "blocked_policy_or_read_only",
            Self::BlockedAmbiguousOrWrongTarget => "blocked_ambiguous_or_wrong_target",
            Self::BlockedRequiresReview => "blocked_requires_review",
            Self::BlockedRequiresRetarget => "blocked_requires_retarget",
        }
    }

    /// True when writes must be blocked or reviewed.
    pub const fn blocks_write(self) -> bool {
        !matches!(self, Self::WritesAllowed)
    }
}

/// Native-file review surface class from fixture cases.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum NativeFileReviewSurfaceClass {
    /// No review surface.
    None,
    /// Target review sheet.
    TargetReviewSheet,
    /// Write review sheet.
    WriteReviewSheet,
    /// Deep-link interstitial.
    DeepLinkInterstitial,
    /// Placeholder recovery surface.
    PlaceholderRecoverySurface,
}

impl NativeFileReviewSurfaceClass {
    /// Returns the stable token for this native-file review surface.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::None => "none",
            Self::TargetReviewSheet => "target_review_sheet",
            Self::WriteReviewSheet => "write_review_sheet",
            Self::DeepLinkInterstitial => "deep_link_interstitial",
            Self::PlaceholderRecoverySurface => "placeholder_recovery_surface",
        }
    }
}

/// Save-target token posture.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SaveTokenPosture {
    /// Token is deferred until write.
    DeferredUntilWrite,
    /// Token minted for open or entry.
    MintedForOpenOrEntry,
    /// Token minted for save or overwrite.
    MintedForSaveOrOverwrite,
    /// Token unavailable and writes are blocked.
    UnavailableBlocksWrite,
}

impl SaveTokenPosture {
    /// Returns the stable token for this save-token posture.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::DeferredUntilWrite => "deferred_until_write",
            Self::MintedForOpenOrEntry => "minted_for_open_or_entry",
            Self::MintedForSaveOrOverwrite => "minted_for_save_or_overwrite",
            Self::UnavailableBlocksWrite => "unavailable_blocks_write",
        }
    }
}

/// Literal target format class.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LiteralTargetFormatClass {
    /// Windows drive path.
    WindowsDrivePath,
    /// Windows UNC path.
    WindowsUncPath,
    /// POSIX path.
    PosixPath,
    /// File URI.
    FileUri,
    /// Unknown format.
    Unknown,
}

impl LiteralTargetFormatClass {
    /// Returns the stable token for this target format class.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WindowsDrivePath => "windows_drive_path",
            Self::WindowsUncPath => "windows_unc_path",
            Self::PosixPath => "posix_path",
            Self::FileUri => "file_uri",
            Self::Unknown => "unknown",
        }
    }
}

/// External path classification for native file affordances.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ExternalPathClass {
    /// Local volume.
    LocalVolume,
    /// Removable volume.
    RemovableVolume,
    /// Network share UNC path.
    NetworkShareUnc,
    /// Network share mapped drive.
    NetworkShareMappedDrive,
    /// Network share POSIX mount.
    NetworkSharePosixMount,
    /// Cloud sync root.
    CloudSyncRoot,
    /// Case variant observed.
    CaseVariantObserved,
    /// Unicode normalization variant observed.
    UnicodeNormalizationVariantObserved,
    /// Unknown path class.
    Unknown,
}

impl ExternalPathClass {
    /// Returns the stable token for this path class.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LocalVolume => "local_volume",
            Self::RemovableVolume => "removable_volume",
            Self::NetworkShareUnc => "network_share_unc",
            Self::NetworkShareMappedDrive => "network_share_mapped_drive",
            Self::NetworkSharePosixMount => "network_share_posix_mount",
            Self::CloudSyncRoot => "cloud_sync_root",
            Self::CaseVariantObserved => "case_variant_observed",
            Self::UnicodeNormalizationVariantObserved => "unicode_normalization_variant_observed",
            Self::Unknown => "unknown",
        }
    }
}

/// Canonical target identity carried by a platform deep-link intent.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PlatformTargetIdentity {
    /// Target kind.
    pub target_kind: TargetKind,
    /// Canonical object identity ref.
    pub object_identity_ref: String,
    /// Optional target revision ref.
    #[serde(default)]
    pub target_revision_ref: Option<String>,
    /// Optional last-known-good ref.
    #[serde(default)]
    pub last_known_good_ref: Option<String>,
    /// Availability class.
    pub availability_class: TargetAvailabilityClass,
    /// Freshness class.
    pub freshness_class: FreshnessClass,
}

/// Handler ownership carried by a platform deep-link intent.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PlatformHandlerOwnership {
    /// Handler ownership class.
    pub ownership_class: HandlerOwnershipClass,
    /// Ownership review state.
    pub ownership_review_state: OwnershipChangeReviewState,
    /// Owning channel ref, if any.
    #[serde(default)]
    pub owning_channel_ref: Option<String>,
    /// Owner build ref, if any.
    #[serde(default)]
    pub owner_build_ref: Option<String>,
}

/// Fallback resolution carried by a platform deep-link intent.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PlatformFallbackResolution {
    /// Fallback class.
    pub fallback_class: FallbackClass,
    /// Fallback command id ref, if any.
    #[serde(default)]
    pub fallback_command_id_ref: Option<String>,
    /// Fallback target identity ref, if any.
    #[serde(default)]
    pub fallback_target_identity_ref: Option<String>,
    /// True when fallback preserves source, route, and target identity.
    pub preserves_user_intent: bool,
}

/// Policy context carried by a platform deep-link intent.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PlatformPolicyContext {
    /// Policy epoch ref.
    pub policy_epoch: String,
    /// Workspace trust state.
    pub trust_state: TrustState,
    /// Tenant or workspace scope ref, if any.
    #[serde(default)]
    pub tenant_or_workspace_scope_ref: Option<String>,
}

/// Platform `deep_link_intent_record` subset consumed by the shell.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PlatformDeepLinkIntentRecord {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub deep_link_intent_schema_version: u32,
    /// Stable intent id.
    pub intent_id: String,
    /// OS-facing source surface.
    pub source_surface_class: SourceSurfaceClass,
    /// Origin class.
    pub origin_class: OriginClass,
    /// Opaque origin ref.
    pub origin_ref: String,
    /// Route class.
    pub route_class: RouteClass,
    /// Target identity.
    pub target_identity: PlatformTargetIdentity,
    /// Requested action class.
    pub requested_action_class: RequestedActionClass,
    /// Authority delta.
    pub authority_delta_class: AuthorityDeltaClass,
    /// Command id ref.
    pub command_id_ref: String,
    /// Trust review requirement.
    pub trust_review_requirement: TrustReviewRequirement,
    /// Policy resolution.
    pub policy_resolution_class: PolicyResolutionClass,
    /// Replay posture.
    pub replay_posture: ReplayPosture,
    /// Fallback resolution.
    pub fallback: PlatformFallbackResolution,
    /// Handler ownership.
    pub handler_ownership: PlatformHandlerOwnership,
    /// Visible review disclosure field tokens.
    pub review_disclosure_fields: Vec<String>,
    /// Degraded reason tokens.
    pub degraded_reasons: Vec<String>,
    /// Policy context.
    pub policy_context: PlatformPolicyContext,
}

/// Canonical refs carried by native-file-affordance cases.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NativeFileCanonicalRefs {
    /// Stable command id ref.
    pub command_id_ref: String,
    /// Canonical object identity ref.
    pub object_identity_ref: String,
    /// Optional save-target token ref.
    #[serde(default)]
    pub save_target_token_ref: Option<String>,
    /// Optional write-review sheet ref.
    #[serde(default)]
    pub write_review_sheet_ref: Option<String>,
}

/// Literal target carried by native-file-affordance cases.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NativeLiteralInput {
    /// Exact literal target string delivered by the OS or dialog.
    pub literal: String,
    /// Literal target format class.
    pub format_class: LiteralTargetFormatClass,
    /// Display label derived from the literal target.
    pub display_label: String,
}

/// Binding summary carried by native-file-affordance cases.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NativeBindingSummary {
    /// Availability class.
    pub availability_class: TargetAvailabilityClass,
    /// External path classes.
    pub external_path_classes: Vec<ExternalPathClass>,
    /// Trust state.
    pub trust_state: TrustState,
    /// Write posture.
    pub write_posture: WritePostureClass,
    /// True when target resolves outside expected scope.
    #[serde(default)]
    pub outside_expected_scope: bool,
    /// True when aliases or collisions must be disclosed.
    #[serde(default)]
    pub alias_disclosure_required: bool,
}

/// Expected behavior carried by native-file-affordance cases.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NativeExpectedBehavior {
    /// Required review surface.
    pub review_surface_class: NativeFileReviewSurfaceClass,
    /// Save-target token posture.
    pub save_token_posture: SaveTokenPosture,
    /// True when identity must be preserved.
    pub must_preserve_identity: bool,
    /// True when safe-preview status must be preserved.
    pub must_preserve_safe_preview_status: bool,
    /// True when writes are forbidden without token proof.
    #[serde(default = "default_true")]
    pub must_not_write_without_token: bool,
}

const fn default_true() -> bool {
    true
}

/// Native-file-affordance fixture case consumed by the shell.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NativeFileAffordanceCaseRecord {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub native_file_affordance_schema_version: u32,
    /// Stable case id.
    pub case_id: String,
    /// Native affordance class.
    pub affordance_class: NativeFileAffordanceClass,
    /// Native target kind.
    pub target_kind: NativeFileTargetKind,
    /// Scenario summary.
    pub scenario_summary: String,
    /// Canonical backing refs.
    pub canonical_backing_refs: NativeFileCanonicalRefs,
    /// Literal input from the OS or dialog.
    pub literal_input: NativeLiteralInput,
    /// Bound target summary.
    pub binding: NativeBindingSummary,
    /// Expected behavior.
    pub expected_behavior: NativeExpectedBehavior,
    /// Must-not-happen assertions.
    pub must_not_happen: Vec<String>,
}

/// Recovery action shown on a native handoff review surface.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NativeRecoveryAction {
    /// Stable action token.
    pub action_token: String,
    /// User-facing action label.
    pub label: String,
    /// True when the action preserves the original source/target identity.
    pub preserves_user_intent: bool,
}

/// Target summary projected into a native handoff review record.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NativeTargetSummary {
    /// Target kind token.
    pub target_kind_token: String,
    /// Canonical object identity ref.
    pub object_identity_ref: String,
    /// Availability class token.
    pub availability_class_token: String,
    /// Freshness class token.
    pub freshness_class_token: String,
}

/// Review projection for a platform deep-link/native handoff intent.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NativeBoundaryHandoffReviewRecord {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Stable review id.
    pub review_id: String,
    /// Source intent id.
    pub intent_id: String,
    /// Source surface token.
    pub source_surface_token: String,
    /// Origin class token.
    pub origin_class_token: String,
    /// Route class token.
    pub route_class_token: String,
    /// Requested action token.
    pub requested_action_token: String,
    /// Authority delta token.
    pub authority_delta_token: String,
    /// Resulting command id.
    pub command_id_ref: String,
    /// Target summary.
    pub target: NativeTargetSummary,
    /// Handler ownership token.
    pub handler_ownership_token: String,
    /// Ownership review token.
    pub ownership_review_state_token: String,
    /// Owning channel ref.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub owning_channel_ref: Option<String>,
    /// Owner build ref.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub owner_build_ref: Option<String>,
    /// Trust state token.
    pub trust_state_token: String,
    /// Policy epoch ref.
    pub policy_epoch_ref: String,
    /// Tenant or workspace scope ref.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tenant_or_workspace_scope_ref: Option<String>,
    /// Review surface token.
    pub review_surface_token: String,
    /// True when native review is required before execution.
    pub native_review_required: bool,
    /// True when exact command execution is allowed after any required review.
    pub execution_allowed: bool,
    /// True when the request must not execute directly from the OS surface.
    pub direct_os_execution_forbidden: bool,
    /// True when placeholder recovery is required.
    pub placeholder_recovery_required: bool,
    /// Recovery actions for denied or degraded targets.
    pub recovery_actions: Vec<NativeRecoveryAction>,
    /// Disclosure fields visible before execution.
    pub disclosure_fields: Vec<String>,
    /// Degraded reason tokens.
    pub degraded_reason_tokens: Vec<String>,
    /// Host-rendered summary line.
    pub summary: String,
}

/// Review projection for a native file affordance case.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NativeFileHandoffReviewRecord {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Stable review id.
    pub review_id: String,
    /// Source case id.
    pub case_id: String,
    /// Native affordance token.
    pub affordance_class_token: String,
    /// Target kind token.
    pub target_kind_token: String,
    /// Command id ref.
    pub command_id_ref: String,
    /// Canonical object identity ref.
    pub object_identity_ref: String,
    /// Literal target label.
    pub literal_target_label: String,
    /// Literal target format token.
    pub literal_target_format_token: String,
    /// Availability class token.
    pub availability_class_token: String,
    /// External path class tokens.
    pub external_path_class_tokens: Vec<String>,
    /// Trust state token.
    pub trust_state_token: String,
    /// Write posture token.
    pub write_posture_token: String,
    /// Review surface token.
    pub review_surface_token: String,
    /// Save-token posture token.
    pub save_token_posture_token: String,
    /// True when identity was preserved.
    pub identity_preserved: bool,
    /// True when safe-preview state was preserved.
    pub safe_preview_preserved: bool,
    /// True when writes are blocked without a token or review.
    pub writes_blocked_without_token: bool,
    /// Recovery actions to render for placeholder or blocked targets.
    pub recovery_actions: Vec<NativeRecoveryAction>,
    /// Host-rendered summary line.
    pub summary: String,
}

/// Runtime/support packet for native handoff alpha evidence.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NativeBoundaryHandoffPacket {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Packet id.
    pub packet_id: String,
    /// Running build identity ref.
    pub build_identity_ref: String,
    /// Projection timestamp.
    pub generated_at: String,
    /// Deep-link/native handoff review rows.
    pub handoff_reviews: Vec<NativeBoundaryHandoffReviewRecord>,
    /// Native file affordance review rows.
    pub native_file_reviews: Vec<NativeFileHandoffReviewRecord>,
}

impl NativeBoundaryHandoffPacket {
    /// Returns a handoff review for a source surface.
    pub fn handoff_for_source(
        &self,
        source: SourceSurfaceClass,
    ) -> Option<&NativeBoundaryHandoffReviewRecord> {
        self.handoff_reviews
            .iter()
            .find(|row| row.source_surface_token == source.as_str())
    }

    /// Returns a native file review for an affordance class.
    pub fn native_file_review_for(
        &self,
        affordance: NativeFileAffordanceClass,
    ) -> Option<&NativeFileHandoffReviewRecord> {
        self.native_file_reviews
            .iter()
            .find(|row| row.affordance_class_token == affordance.as_str())
    }
}

/// Builds the seeded alpha packet for native desktop handoff evidence.
pub fn seeded_native_boundary_handoff_packet(
    build_identity_ref: impl Into<String>,
) -> NativeBoundaryHandoffPacket {
    let build_identity_ref = build_identity_ref.into();
    let handoff_intents = vec![
        system_open_workspace_intent(),
        parse_deep_link_fixture(include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/platform/exact_target_reopen_cases/local_file_open_admitted_exact.yaml"
        ))),
        parse_deep_link_fixture(include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/platform/exact_target_reopen_cases/workspace_open_missing_target_denied.yaml"
        ))),
        dock_jump_workspace_intent(),
        parse_deep_link_fixture(include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/platform/exact_target_reopen_cases/review_handoff_admitted_exact.yaml"
        ))),
        privileged_protocol_command_intent(),
        parse_deep_link_fixture(include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/platform/exact_target_reopen_cases/auth_callback_replay_denied_consumed.yaml"
        ))),
    ];
    let native_file_cases = vec![
        parse_native_file_fixture(include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/platform/native_file_affordance_cases/local_file_native_open_dialog_binds_identity.yaml"
        ))),
        parse_native_file_fixture(include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/platform/native_file_affordance_cases/generated_artifact_native_save_dialog_routes_through_write_review.yaml"
        ))),
        reveal_in_shell_case(),
        parse_native_file_fixture(include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/platform/native_file_affordance_cases/removable_volume_missing_target_opens_placeholder_recovery.yaml"
        ))),
    ];

    NativeBoundaryHandoffPacket {
        record_kind: NATIVE_BOUNDARY_HANDOFF_PACKET_RECORD_KIND.to_owned(),
        schema_version: NATIVE_BOUNDARY_HANDOFF_PACKET_SCHEMA_VERSION,
        packet_id: "native-boundary-handoff:alpha:seed".to_owned(),
        build_identity_ref,
        generated_at: now_rfc3339(),
        handoff_reviews: handoff_intents
            .iter()
            .map(review_native_handoff_intent)
            .collect(),
        native_file_reviews: native_file_cases
            .iter()
            .map(review_native_file_affordance_case)
            .collect(),
    }
}

/// Writes a native boundary handoff packet to
/// `<evidence_root>/native_boundary_handoff_latest.json`.
pub fn write_native_boundary_handoff_log(
    evidence_root: &Path,
    packet: &NativeBoundaryHandoffPacket,
) -> Result<(), String> {
    std::fs::create_dir_all(evidence_root)
        .map_err(|err| format!("create native handoff evidence root failed: {err}"))?;
    let path = evidence_root.join("native_boundary_handoff_latest.json");
    let json = serde_json::to_string_pretty(packet)
        .map_err(|err| format!("serialize native boundary handoff packet failed: {err}"))?;
    std::fs::write(&path, json).map_err(|err| format!("write {} failed: {err}", path.display()))
}

/// Projects native review truth for a platform deep-link intent.
pub fn review_native_handoff_intent(
    intent: &PlatformDeepLinkIntentRecord,
) -> NativeBoundaryHandoffReviewRecord {
    let native_review_required = intent.trust_review_requirement.requires_review()
        || intent.authority_delta_class.requires_review()
        || intent.requested_action_class.is_high_risk()
        || intent
            .handler_ownership
            .ownership_review_state
            .requires_owner_preview()
        || intent
            .target_identity
            .availability_class
            .requires_recovery()
        || intent.replay_posture.is_denied()
        || intent.handler_ownership.ownership_class.is_conflict()
        || intent.origin_class == OriginClass::UnknownUntrusted;
    let denied = intent.policy_resolution_class.is_denied()
        || intent.replay_posture.is_denied()
        || intent.handler_ownership.ownership_class.is_conflict()
        || intent.origin_class == OriginClass::UnknownUntrusted;
    let placeholder_recovery_required = intent
        .target_identity
        .availability_class
        .requires_placeholder()
        || matches!(
            intent.fallback.fallback_class,
            FallbackClass::LocateMissingTarget
                | FallbackClass::OpenReadOnlyPlaceholder
                | FallbackClass::OpenCachedContext
        );
    let review_surface = if placeholder_recovery_required {
        NativeReviewSurfaceClass::PlaceholderRecoveryCard
    } else if denied {
        NativeReviewSurfaceClass::DeniedRecoverySheet
    } else if intent.requested_action_class.is_high_risk()
        || intent.authority_delta_class.requires_review()
    {
        NativeReviewSurfaceClass::ProductOwnedNativeReview
    } else if native_review_required {
        NativeReviewSurfaceClass::ReviewedIntentSheet
    } else {
        NativeReviewSurfaceClass::ExactOpen
    };
    let execution_allowed = !denied && !placeholder_recovery_required;
    let direct_os_execution_forbidden = native_review_required
        || intent.source_surface_class.is_summary_only()
        || intent.requested_action_class.is_high_risk();
    let disclosure_fields = required_disclosure_fields(intent);
    let recovery_actions = recovery_actions_for_intent(intent);

    NativeBoundaryHandoffReviewRecord {
        record_kind: NATIVE_BOUNDARY_HANDOFF_REVIEW_RECORD_KIND.to_owned(),
        schema_version: NATIVE_BOUNDARY_HANDOFF_PACKET_SCHEMA_VERSION,
        review_id: format!("native-handoff-review:{}", intent.intent_id),
        intent_id: intent.intent_id.clone(),
        source_surface_token: intent.source_surface_class.as_str().to_owned(),
        origin_class_token: intent.origin_class.as_str().to_owned(),
        route_class_token: intent.route_class.as_str().to_owned(),
        requested_action_token: intent.requested_action_class.as_str().to_owned(),
        authority_delta_token: intent.authority_delta_class.as_str().to_owned(),
        command_id_ref: intent.command_id_ref.clone(),
        target: NativeTargetSummary {
            target_kind_token: intent.target_identity.target_kind.as_str().to_owned(),
            object_identity_ref: intent.target_identity.object_identity_ref.clone(),
            availability_class_token: intent
                .target_identity
                .availability_class
                .as_str()
                .to_owned(),
            freshness_class_token: intent.target_identity.freshness_class.as_str().to_owned(),
        },
        handler_ownership_token: intent.handler_ownership.ownership_class.as_str().to_owned(),
        ownership_review_state_token: intent
            .handler_ownership
            .ownership_review_state
            .as_str()
            .to_owned(),
        owning_channel_ref: intent.handler_ownership.owning_channel_ref.clone(),
        owner_build_ref: intent.handler_ownership.owner_build_ref.clone(),
        trust_state_token: intent.policy_context.trust_state.as_str().to_owned(),
        policy_epoch_ref: intent.policy_context.policy_epoch.clone(),
        tenant_or_workspace_scope_ref: intent.policy_context.tenant_or_workspace_scope_ref.clone(),
        review_surface_token: review_surface.as_str().to_owned(),
        native_review_required,
        execution_allowed,
        direct_os_execution_forbidden,
        placeholder_recovery_required,
        recovery_actions,
        disclosure_fields,
        degraded_reason_tokens: intent.degraded_reasons.clone(),
        summary: handoff_summary(intent, review_surface, execution_allowed),
    }
}

/// Projects native file handoff truth for a native affordance case.
pub fn review_native_file_affordance_case(
    case: &NativeFileAffordanceCaseRecord,
) -> NativeFileHandoffReviewRecord {
    let recovery_actions = recovery_actions_for_native_file_case(case);
    NativeFileHandoffReviewRecord {
        record_kind: NATIVE_FILE_HANDOFF_REVIEW_RECORD_KIND.to_owned(),
        schema_version: NATIVE_BOUNDARY_HANDOFF_PACKET_SCHEMA_VERSION,
        review_id: format!("native-file-handoff-review:{}", case.case_id),
        case_id: case.case_id.clone(),
        affordance_class_token: case.affordance_class.as_str().to_owned(),
        target_kind_token: case.target_kind.as_str().to_owned(),
        command_id_ref: case.canonical_backing_refs.command_id_ref.clone(),
        object_identity_ref: case.canonical_backing_refs.object_identity_ref.clone(),
        literal_target_label: case.literal_input.display_label.clone(),
        literal_target_format_token: case.literal_input.format_class.as_str().to_owned(),
        availability_class_token: case.binding.availability_class.as_str().to_owned(),
        external_path_class_tokens: case
            .binding
            .external_path_classes
            .iter()
            .map(|value| value.as_str().to_owned())
            .collect(),
        trust_state_token: case.binding.trust_state.as_str().to_owned(),
        write_posture_token: case.binding.write_posture.as_str().to_owned(),
        review_surface_token: case
            .expected_behavior
            .review_surface_class
            .as_str()
            .to_owned(),
        save_token_posture_token: case
            .expected_behavior
            .save_token_posture
            .as_str()
            .to_owned(),
        identity_preserved: case.expected_behavior.must_preserve_identity,
        safe_preview_preserved: case.expected_behavior.must_preserve_safe_preview_status,
        writes_blocked_without_token: case.expected_behavior.must_not_write_without_token
            && case.binding.write_posture.blocks_write(),
        recovery_actions,
        summary: native_file_summary(case),
    }
}

fn required_disclosure_fields(intent: &PlatformDeepLinkIntentRecord) -> Vec<String> {
    let mut fields = vec![
        "origin",
        "source_surface",
        "route_class",
        "target_identity",
        "command_id",
        "handler_owner",
        "trust_state",
        "policy_epoch",
        "replay_posture",
        "fallback",
    ];
    if intent.authority_delta_class.requires_review() {
        fields.push("authority_delta");
    }
    if intent.source_surface_class == SourceSurfaceClass::DefaultBrowserCallback {
        fields.push("browser_handoff");
    }
    if intent
        .handler_ownership
        .ownership_review_state
        .requires_owner_preview()
    {
        fields.push("owner_build_or_channel");
    }
    fields.sort_unstable();
    fields.dedup();
    fields.into_iter().map(str::to_owned).collect()
}

fn recovery_actions_for_intent(intent: &PlatformDeepLinkIntentRecord) -> Vec<NativeRecoveryAction> {
    match intent.fallback.fallback_class {
        FallbackClass::LocateMissingTarget => vec![
            action("locate", "Locate", true),
            action("open_cached_context", "Open cached context", true),
            action("close_placeholder", "Close placeholder", true),
        ],
        FallbackClass::OpenReadOnlyPlaceholder => vec![
            action(
                "open_read_only_placeholder",
                "Open read-only placeholder",
                true,
            ),
            action("locate", "Locate", true),
            action("close_placeholder", "Close placeholder", true),
        ],
        FallbackClass::OpenCachedContext => vec![
            action("open_cached_context", "Open cached context", true),
            action("locate", "Locate", true),
            action("close_placeholder", "Close placeholder", true),
        ],
        FallbackClass::OpenIntentReviewSheet => {
            vec![
                action("review_intent", "Review intent", true),
                action("cancel", "Cancel", true),
            ]
        }
        FallbackClass::OpenDefaultBrowser => vec![
            action("restart_browser_handoff", "Restart in browser", true),
            action("cancel", "Cancel", true),
        ],
        FallbackClass::OpenActivityCenter => vec![
            action("open_activity_center", "Open Activity Center", true),
            action("cancel", "Cancel", true),
        ],
        FallbackClass::ContinueLocalOnly => vec![
            action("continue_local_only", "Continue local-only", true),
            action("cancel", "Cancel", true),
        ],
        FallbackClass::DenyWithExplanation | FallbackClass::NoFallbackAvailable => {
            vec![action("close_placeholder", "Close placeholder", true)]
        }
        FallbackClass::ExportContext => vec![
            action("export_context", "Export context", true),
            action("cancel", "Cancel", true),
        ],
    }
}

fn recovery_actions_for_native_file_case(
    case: &NativeFileAffordanceCaseRecord,
) -> Vec<NativeRecoveryAction> {
    if case.binding.availability_class.requires_placeholder()
        || case.expected_behavior.review_surface_class
            == NativeFileReviewSurfaceClass::PlaceholderRecoverySurface
    {
        return vec![
            action("locate", "Locate", true),
            action("open_cached_context", "Open cached context", true),
            action("close_placeholder", "Close placeholder", true),
        ];
    }
    if case.expected_behavior.review_surface_class == NativeFileReviewSurfaceClass::WriteReviewSheet
    {
        return vec![
            action("review_write", "Review write", true),
            action("choose_another_target", "Choose another target", true),
            action("cancel", "Cancel", true),
        ];
    }
    if case.affordance_class == NativeFileAffordanceClass::RevealInSystemShell {
        return vec![
            action("reveal_in_system_shell", "Reveal in system shell", true),
            action("copy_bound_path", "Copy bound path", true),
        ];
    }
    vec![action("open_bound_target", "Open bound target", true)]
}

fn action(
    action_token: impl Into<String>,
    label: impl Into<String>,
    preserves_user_intent: bool,
) -> NativeRecoveryAction {
    NativeRecoveryAction {
        action_token: action_token.into(),
        label: label.into(),
        preserves_user_intent,
    }
}

fn handoff_summary(
    intent: &PlatformDeepLinkIntentRecord,
    surface: NativeReviewSurfaceClass,
    execution_allowed: bool,
) -> String {
    format!(
        "{} -> {} uses {} for {} owned by {} / {}; review_surface={} execution_allowed={}",
        intent.source_surface_class.as_str(),
        intent.route_class.as_str(),
        intent.command_id_ref,
        intent.target_identity.object_identity_ref,
        intent
            .handler_ownership
            .owning_channel_ref
            .as_deref()
            .unwrap_or("unknown_channel"),
        intent
            .handler_ownership
            .owner_build_ref
            .as_deref()
            .unwrap_or("unknown_build"),
        surface.as_str(),
        execution_allowed
    )
}

fn native_file_summary(case: &NativeFileAffordanceCaseRecord) -> String {
    format!(
        "{} binds {} to {} with write_posture={} review_surface={}",
        case.affordance_class.as_str(),
        case.literal_input.display_label,
        case.canonical_backing_refs.object_identity_ref,
        case.binding.write_posture.as_str(),
        case.expected_behavior.review_surface_class.as_str()
    )
}

fn parse_deep_link_fixture(payload: &str) -> PlatformDeepLinkIntentRecord {
    serde_yaml::from_str(payload).expect("deep-link intent fixture must parse")
}

fn parse_native_file_fixture(payload: &str) -> NativeFileAffordanceCaseRecord {
    serde_yaml::from_str(payload).expect("native file affordance fixture must parse")
}

fn system_open_workspace_intent() -> PlatformDeepLinkIntentRecord {
    PlatformDeepLinkIntentRecord {
        record_kind: "deep_link_intent_record".to_owned(),
        deep_link_intent_schema_version: 1,
        intent_id: "platform:intent:alpha:system-open:workspace:01".to_owned(),
        source_surface_class: SourceSurfaceClass::SystemOpen,
        origin_class: OriginClass::OsShell,
        origin_ref: "origin:os-shell:system-open:workspace:alpha".to_owned(),
        route_class: RouteClass::WorkspaceOpen,
        target_identity: PlatformTargetIdentity {
            target_kind: TargetKind::WorkspaceRoot,
            object_identity_ref: "obj:workspace-root:alpha-local:01".to_owned(),
            target_revision_ref: Some("rev:workspace-root:alpha-local:2026-05-13".to_owned()),
            last_known_good_ref: Some("recent:workspace:alpha-local:last-opened".to_owned()),
            availability_class: TargetAvailabilityClass::ExactAvailable,
            freshness_class: FreshnessClass::AuthoritativeLive,
        },
        requested_action_class: RequestedActionClass::OpenExistingContext,
        authority_delta_class: AuthorityDeltaClass::None,
        command_id_ref: "cmd:workspace.open_folder".to_owned(),
        trust_review_requirement: TrustReviewRequirement::NoReviewRequired,
        policy_resolution_class: PolicyResolutionClass::AllowedCurrentPolicy,
        replay_posture: ReplayPosture::SingleUse,
        fallback: PlatformFallbackResolution {
            fallback_class: FallbackClass::OpenIntentReviewSheet,
            fallback_command_id_ref: Some("cmd:workspace.open_folder".to_owned()),
            fallback_target_identity_ref: Some("obj:workspace-root:alpha-local:01".to_owned()),
            preserves_user_intent: true,
        },
        handler_ownership: PlatformHandlerOwnership {
            ownership_class: HandlerOwnershipClass::CurrentUserRegistered,
            ownership_review_state: OwnershipChangeReviewState::NoChange,
            owning_channel_ref: Some("channel:preview".to_owned()),
            owner_build_ref: Some("build:aureline:preview:alpha".to_owned()),
        },
        review_disclosure_fields: vec![
            "origin".to_owned(),
            "source_surface".to_owned(),
            "target_identity".to_owned(),
            "command_id".to_owned(),
            "handler_owner".to_owned(),
        ],
        degraded_reasons: vec!["none".to_owned()],
        policy_context: PlatformPolicyContext {
            policy_epoch: "pe:alpha:system-open:01".to_owned(),
            trust_state: TrustState::Trusted,
            tenant_or_workspace_scope_ref: Some("scope:workspace:alpha-local".to_owned()),
        },
    }
}

fn privileged_protocol_command_intent() -> PlatformDeepLinkIntentRecord {
    PlatformDeepLinkIntentRecord {
        record_kind: "deep_link_intent_record".to_owned(),
        deep_link_intent_schema_version: 1,
        intent_id: "platform:intent:alpha:protocol:privileged-command:01".to_owned(),
        source_surface_class: SourceSurfaceClass::ProtocolHandler,
        origin_class: OriginClass::SystemDefaultBrowser,
        origin_ref: "origin:browser:protocol-command:alpha".to_owned(),
        route_class: RouteClass::CommandInvocation,
        target_identity: PlatformTargetIdentity {
            target_kind: TargetKind::CommandTarget,
            object_identity_ref: "obj:command:trust-elevation:alpha".to_owned(),
            target_revision_ref: None,
            last_known_good_ref: None,
            availability_class: TargetAvailabilityClass::ExactAvailable,
            freshness_class: FreshnessClass::AuthoritativeLive,
        },
        requested_action_class: RequestedActionClass::PrivilegedAuthorityWidening,
        authority_delta_class: AuthorityDeltaClass::UnknownRequiresReview,
        command_id_ref: "cmd:trust.review_change".to_owned(),
        trust_review_requirement: TrustReviewRequirement::ReviewRequiredBeforeCommand,
        policy_resolution_class: PolicyResolutionClass::AllowedAfterReview,
        replay_posture: ReplayPosture::SingleUse,
        fallback: PlatformFallbackResolution {
            fallback_class: FallbackClass::OpenIntentReviewSheet,
            fallback_command_id_ref: Some("cmd:trust.review_change".to_owned()),
            fallback_target_identity_ref: Some("obj:command:trust-elevation:alpha".to_owned()),
            preserves_user_intent: true,
        },
        handler_ownership: PlatformHandlerOwnership {
            ownership_class: HandlerOwnershipClass::MachineGlobalRegistered,
            ownership_review_state: OwnershipChangeReviewState::PreviewRequiredBeforeChange,
            owning_channel_ref: Some("channel:stable".to_owned()),
            owner_build_ref: Some("build:aureline:stable:alpha".to_owned()),
        },
        review_disclosure_fields: vec![
            "origin".to_owned(),
            "source_surface".to_owned(),
            "route_class".to_owned(),
            "target_identity".to_owned(),
            "command_id".to_owned(),
            "authority_delta".to_owned(),
            "handler_owner".to_owned(),
            "replay_posture".to_owned(),
            "fallback".to_owned(),
        ],
        degraded_reasons: vec!["none".to_owned()],
        policy_context: PlatformPolicyContext {
            policy_epoch: "pe:alpha:protocol-command:01".to_owned(),
            trust_state: TrustState::PendingEvaluation,
            tenant_or_workspace_scope_ref: Some("scope:workspace:alpha-local".to_owned()),
        },
    }
}

fn dock_jump_workspace_intent() -> PlatformDeepLinkIntentRecord {
    PlatformDeepLinkIntentRecord {
        record_kind: "deep_link_intent_record".to_owned(),
        deep_link_intent_schema_version: 1,
        intent_id: "platform:intent:alpha:dock-jump:workspace:01".to_owned(),
        source_surface_class: SourceSurfaceClass::DockTaskbarJumpAction,
        origin_class: OriginClass::OsShell,
        origin_ref: "origin:os-shell:dock-jump:workspace:alpha".to_owned(),
        route_class: RouteClass::WorkspaceOpen,
        target_identity: PlatformTargetIdentity {
            target_kind: TargetKind::RecentWorkEntry,
            object_identity_ref: "obj:recent-work:workspace:alpha-jump:01".to_owned(),
            target_revision_ref: Some("rev:recent-work:workspace:alpha-jump:2026-05-13".to_owned()),
            last_known_good_ref: Some("snapshot:workspace:alpha-jump:lkg:01".to_owned()),
            availability_class: TargetAvailabilityClass::ExactAvailable,
            freshness_class: FreshnessClass::WarmCached,
        },
        requested_action_class: RequestedActionClass::OpenExistingContext,
        authority_delta_class: AuthorityDeltaClass::None,
        command_id_ref: "cmd:start_center.open_recent".to_owned(),
        trust_review_requirement: TrustReviewRequirement::NoReviewRequired,
        policy_resolution_class: PolicyResolutionClass::AllowedCurrentPolicy,
        replay_posture: ReplayPosture::BoundedReuse,
        fallback: PlatformFallbackResolution {
            fallback_class: FallbackClass::OpenActivityCenter,
            fallback_command_id_ref: Some("cmd:activity.open_event".to_owned()),
            fallback_target_identity_ref: Some(
                "obj:activity-event:dock-jump:workspace:alpha".to_owned(),
            ),
            preserves_user_intent: true,
        },
        handler_ownership: PlatformHandlerOwnership {
            ownership_class: HandlerOwnershipClass::CurrentUserRegistered,
            ownership_review_state: OwnershipChangeReviewState::NoChange,
            owning_channel_ref: Some("channel:preview".to_owned()),
            owner_build_ref: Some("build:aureline:preview:alpha".to_owned()),
        },
        review_disclosure_fields: vec![
            "origin".to_owned(),
            "source_surface".to_owned(),
            "route_class".to_owned(),
            "target_identity".to_owned(),
            "command_id".to_owned(),
            "handler_owner".to_owned(),
            "replay_posture".to_owned(),
            "fallback".to_owned(),
        ],
        degraded_reasons: vec!["none".to_owned()],
        policy_context: PlatformPolicyContext {
            policy_epoch: "pe:alpha:dock-jump:01".to_owned(),
            trust_state: TrustState::Trusted,
            tenant_or_workspace_scope_ref: Some("scope:workspace:alpha-jump".to_owned()),
        },
    }
}

fn reveal_in_shell_case() -> NativeFileAffordanceCaseRecord {
    NativeFileAffordanceCaseRecord {
        record_kind: "native_file_affordance_case_record".to_owned(),
        native_file_affordance_schema_version: 1,
        case_id: "platform:native-file-affordance:reveal:local-file:alpha".to_owned(),
        affordance_class: NativeFileAffordanceClass::RevealInSystemShell,
        target_kind: NativeFileTargetKind::LocalFile,
        scenario_summary:
            "Reveal in system shell preserves bound identity and remains reveal-only.".to_owned(),
        canonical_backing_refs: NativeFileCanonicalRefs {
            command_id_ref: "cmd:shell.reveal_path".to_owned(),
            object_identity_ref: "obj:vfs:file:local:reveal-alpha".to_owned(),
            save_target_token_ref: None,
            write_review_sheet_ref: None,
        },
        literal_input: NativeLiteralInput {
            literal: "/Users/dev/src/app.ts".to_owned(),
            format_class: LiteralTargetFormatClass::PosixPath,
            display_label: "/Users/dev/src/app.ts".to_owned(),
        },
        binding: NativeBindingSummary {
            availability_class: TargetAvailabilityClass::ExactAvailable,
            external_path_classes: vec![ExternalPathClass::LocalVolume],
            trust_state: TrustState::Trusted,
            write_posture: WritePostureClass::ReadOnly,
            outside_expected_scope: false,
            alias_disclosure_required: false,
        },
        expected_behavior: NativeExpectedBehavior {
            review_surface_class: NativeFileReviewSurfaceClass::None,
            save_token_posture: SaveTokenPosture::DeferredUntilWrite,
            must_preserve_identity: true,
            must_preserve_safe_preview_status: false,
            must_not_write_without_token: true,
        },
        must_not_happen: vec![
            "reveal in shell rewrites the bound target as canonical truth".to_owned(),
            "reveal-only affordance performs a write-like action".to_owned(),
        ],
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn seeded_packet_covers_system_open_recent_protocol_callback_and_file_flows() {
        let packet = seeded_native_boundary_handoff_packet("id:build:test:native-handoff");
        assert_eq!(
            packet.record_kind,
            NATIVE_BOUNDARY_HANDOFF_PACKET_RECORD_KIND
        );
        assert!(packet
            .handoff_for_source(SourceSurfaceClass::SystemOpen)
            .is_some());
        assert!(packet
            .handoff_for_source(SourceSurfaceClass::FileAssociation)
            .is_some());
        assert!(packet
            .handoff_for_source(SourceSurfaceClass::DockTaskbarRecent)
            .is_some());
        assert!(packet
            .handoff_for_source(SourceSurfaceClass::DockTaskbarJumpAction)
            .is_some());
        assert!(packet
            .handoff_for_source(SourceSurfaceClass::ProtocolHandler)
            .is_some());
        assert!(packet
            .handoff_for_source(SourceSurfaceClass::DefaultBrowserCallback)
            .is_some());
        assert!(packet
            .native_file_review_for(NativeFileAffordanceClass::NativeOpenDialog)
            .is_some());
        assert!(packet
            .native_file_review_for(NativeFileAffordanceClass::NativeSaveDialog)
            .is_some());
        assert!(packet
            .native_file_review_for(NativeFileAffordanceClass::RevealInSystemShell)
            .is_some());
    }

    #[test]
    fn system_open_names_channel_build_target_and_command_before_open() {
        let packet = seeded_native_boundary_handoff_packet("id:build:test:native-handoff");
        let row = packet
            .handoff_for_source(SourceSurfaceClass::SystemOpen)
            .expect("system-open row");
        assert_eq!(row.command_id_ref, "cmd:workspace.open_folder");
        assert_eq!(
            row.target.object_identity_ref,
            "obj:workspace-root:alpha-local:01"
        );
        assert_eq!(row.owning_channel_ref.as_deref(), Some("channel:preview"));
        assert_eq!(
            row.owner_build_ref.as_deref(),
            Some("build:aureline:preview:alpha")
        );
        assert_eq!(row.review_surface_token, "exact_open");
        assert!(row.execution_allowed);
        assert!(!row.direct_os_execution_forbidden);
    }

    #[test]
    fn recent_missing_target_degrades_to_placeholder_recovery() {
        let packet = seeded_native_boundary_handoff_packet("id:build:test:native-handoff");
        let row = packet
            .handoff_for_source(SourceSurfaceClass::DockTaskbarRecent)
            .expect("recent item row");
        assert_eq!(row.target.availability_class_token, "missing_or_unmounted");
        assert_eq!(row.review_surface_token, "placeholder_recovery_card");
        assert!(!row.execution_allowed);
        assert!(row.placeholder_recovery_required);
        let actions: Vec<&str> = row
            .recovery_actions
            .iter()
            .map(|action| action.action_token.as_str())
            .collect();
        assert!(actions.contains(&"locate"));
        assert!(actions.contains(&"open_cached_context"));
        assert!(actions.contains(&"close_placeholder"));
    }

    #[test]
    fn file_association_and_jump_action_name_owner_target_and_review_path() {
        let packet = seeded_native_boundary_handoff_packet("id:build:test:native-handoff");
        let file_association = packet
            .handoff_for_source(SourceSurfaceClass::FileAssociation)
            .expect("file association row");
        assert_eq!(file_association.command_id_ref, "cmd:workspace.open_file");
        assert_eq!(
            file_association.target.object_identity_ref,
            "obj:file:workspace:demo:src-main:01"
        );
        assert_eq!(
            file_association.owning_channel_ref.as_deref(),
            Some("channel:stable")
        );
        assert_eq!(
            file_association.review_surface_token,
            "product_owned_native_review"
        );
        assert!(file_association.direct_os_execution_forbidden);

        let jump = packet
            .handoff_for_source(SourceSurfaceClass::DockTaskbarJumpAction)
            .expect("dock/taskbar jump action row");
        assert_eq!(jump.command_id_ref, "cmd:start_center.open_recent");
        assert_eq!(
            jump.target.object_identity_ref,
            "obj:recent-work:workspace:alpha-jump:01"
        );
        assert_eq!(
            jump.owner_build_ref.as_deref(),
            Some("build:aureline:preview:alpha")
        );
        assert!(jump.execution_allowed);
        assert!(jump.direct_os_execution_forbidden);
    }

    #[test]
    fn protocol_handler_high_risk_command_returns_to_native_review() {
        let intent = privileged_protocol_command_intent();
        let row = review_native_handoff_intent(&intent);
        assert_eq!(row.source_surface_token, "protocol_handler");
        assert_eq!(row.requested_action_token, "privileged_authority_widening");
        assert_eq!(row.review_surface_token, "product_owned_native_review");
        assert!(row.native_review_required);
        assert!(row.direct_os_execution_forbidden);
        assert!(row.execution_allowed);
        assert!(row
            .disclosure_fields
            .contains(&"owner_build_or_channel".to_owned()));
    }

    #[test]
    fn auth_callback_replay_denied_cannot_reuse_expired_authority() {
        let packet = seeded_native_boundary_handoff_packet("id:build:test:native-handoff");
        let row = packet
            .handoff_for_source(SourceSurfaceClass::DefaultBrowserCallback)
            .expect("auth callback row");
        assert_eq!(row.degraded_reason_tokens, vec!["replay_denied"]);
        assert_eq!(row.review_surface_token, "placeholder_recovery_card");
        assert!(!row.execution_allowed);
        assert!(row.direct_os_execution_forbidden);
        assert!(row
            .recovery_actions
            .iter()
            .any(|action| action.action_token == "restart_browser_handoff"));
    }

    #[test]
    fn native_open_save_reveal_and_missing_volume_keep_identity_and_write_safety() {
        let packet = seeded_native_boundary_handoff_packet("id:build:test:native-handoff");
        let open = packet
            .native_file_review_for(NativeFileAffordanceClass::NativeOpenDialog)
            .expect("native open case");
        assert_eq!(open.availability_class_token, "exact_available");
        assert_eq!(open.object_identity_ref, "obj:vfs:file:local:01");
        assert_eq!(open.review_surface_token, "none");
        assert!(open.identity_preserved);

        let save = packet
            .native_file_review_for(NativeFileAffordanceClass::NativeSaveDialog)
            .expect("native save case");
        assert_eq!(save.review_surface_token, "write_review_sheet");
        assert_eq!(save.write_posture_token, "blocked_requires_review");
        assert!(save.writes_blocked_without_token);

        let reveal = packet
            .native_file_review_for(NativeFileAffordanceClass::RevealInSystemShell)
            .expect("reveal case");
        assert_eq!(reveal.command_id_ref, "cmd:shell.reveal_path");
        assert_eq!(reveal.write_posture_token, "read_only");
        assert!(reveal
            .recovery_actions
            .iter()
            .any(|action| action.action_token == "reveal_in_system_shell"));

        let missing = packet
            .native_file_reviews
            .iter()
            .find(|row| row.target_kind_token == "removable_volume")
            .expect("missing removable volume case");
        assert_eq!(missing.availability_class_token, "missing_or_unmounted");
        assert_eq!(missing.review_surface_token, "placeholder_recovery_surface");
        assert!(missing
            .recovery_actions
            .iter()
            .any(|action| action.action_token == "locate"));
    }

    #[test]
    fn external_fixtures_round_trip_through_alpha_review_model() {
        let local_file = parse_deep_link_fixture(include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/platform/exact_target_reopen_cases/local_file_open_admitted_exact.yaml"
        )));
        let review = review_native_handoff_intent(&local_file);
        assert_eq!(review.source_surface_token, "file_association");
        assert_eq!(review.target.availability_class_token, "exact_available");
        assert_eq!(review.owning_channel_ref.as_deref(), Some("channel:stable"));

        let missing_volume = parse_native_file_fixture(include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/platform/native_file_affordance_cases/removable_volume_missing_target_opens_placeholder_recovery.yaml"
        )));
        let file_review = review_native_file_affordance_case(&missing_volume);
        assert_eq!(file_review.target_kind_token, "removable_volume");
        assert_eq!(file_review.availability_class_token, "missing_or_unmounted");
        assert!(file_review
            .recovery_actions
            .iter()
            .any(|action| action.action_token == "open_cached_context"));
    }
}
