//! High-authority and cross-boundary entry interstitials for the live shell.
//!
//! Plain local open stays fast: dragging a file in, double-clicking a file
//! association, or quick-opening a path that resolves to an exact, local,
//! already-trusted target never raises a prompt. This module is the gate for
//! everything *broader* than that — the OS-level and browser-level entry paths
//! that arrive from outside the shell and would otherwise execute before the
//! user can see what is about to happen.
//!
//! Six entry kinds flow through here when, and only when, the requested action
//! widens authority or crosses a boundary:
//!
//! - protocol / deep-link activation,
//! - auth callback return from the system browser,
//! - collaboration join,
//! - remote target open,
//! - managed-workspace resume,
//! - OS notification / system-surface reopen.
//!
//! The module never executes the entry. It projects a typed
//! [`EntryInterstitialRecord`] that discloses the origin/source, the requested
//! action class, the target identity and scope (workspace / tenant /
//! channel-or-build owner), the trust/policy effect, and the safe confirm /
//! reject / defer paths. A shell caller renders the record and only proceeds
//! through the **same canonical command** the in-product path would use — the
//! interstitial cannot widen authority relative to that path.
//!
//! Four honesty invariants ride on every record:
//!
//! 1. **No silent execution.** A high-authority entry that arrives from an OS
//!    surface or a browser callback always materializes a record with
//!    [`EntryInterstitialRecord::silent_execution_forbidden`] set; the caller
//!    must show confirm/reject before it runs.
//! 2. **Target truth is preserved.** When the target is moved, missing,
//!    policy-blocked, downgraded, expired, or unreachable, the record carries a
//!    truthful [`TargetPlaceholder`] with bounded fallback actions instead of
//!    opening an empty shell.
//! 3. **Never a generic home surface.** Notification and system-surface reopen
//!    resolve the exact object or an announced placeholder —
//!    [`EntryInterstitialRecord::reopens_generic_home`] is always `false`.
//! 4. **Canonical command, no widening.** The confirm action is bound to the
//!    same `canonical_command_ref` the in-product path uses, so the OS-origin
//!    path can never grant more authority than the in-product path.
//!
//! The same record is projected into a support packet
//! ([`support_export`]) so route/origin incidents can be reconstructed from
//! logs and exports without scraping transient UI text.

pub mod support_export;

use std::path::Path;

use serde::{Deserialize, Serialize};

/// Schema version exported with [`EntryInterstitialRecord`].
pub const ENTRY_INTERSTITIAL_SCHEMA_VERSION: u32 = 1;

/// Stable record-kind tag for [`EntryInterstitialRecord`].
pub const ENTRY_INTERSTITIAL_RECORD_KIND: &str = "entry_interstitial_record";

/// Stable record-kind tag for [`PlainLocalOpenAdmission`].
pub const PLAIN_LOCAL_OPEN_RECORD_KIND: &str = "entry_plain_local_open_record";

/// Canonical command id used for the reject / cancel path (no change).
pub const ENTRY_REJECT_COMMAND_ID: &str = "cmd:workspace.entry.drop";

/// Canonical command id used for the defer path (return to Start Center).
pub const ENTRY_DEFER_COMMAND_ID: &str = "cmd:start_center.open_recent";

/// The kind of higher-authority entry path that produced the interstitial.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EntryInterstitialKind {
    /// Protocol-handler activation or deep link resolved from a URL.
    ProtocolDeepLink,
    /// Auth callback returning from the system default browser.
    AuthCallbackReturn,
    /// Joining a live collaboration / presence session.
    CollaborationJoin,
    /// Opening a remote (SSH, container, devcontainer, cloud) target.
    RemoteTargetOpen,
    /// Resuming a managed-workspace session under org policy.
    ManagedResume,
    /// Reopening an object from an OS notification or system surface.
    NotificationReopen,
}

impl EntryInterstitialKind {
    /// Stable token used in records, fixtures, and logs.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ProtocolDeepLink => "protocol_deep_link",
            Self::AuthCallbackReturn => "auth_callback_return",
            Self::CollaborationJoin => "collaboration_join",
            Self::RemoteTargetOpen => "remote_target_open",
            Self::ManagedResume => "managed_resume",
            Self::NotificationReopen => "notification_reopen",
        }
    }

    /// True when the kind is inherently cross-boundary regardless of the
    /// resolved target: auth returns, collaboration joins, remote opens, and
    /// managed resumes always cross an authority, tenant, or remote boundary.
    /// Deep links and notification reopens may resolve to a plain local open
    /// and are evaluated against the rest of the request.
    pub const fn always_crosses_boundary(self) -> bool {
        matches!(
            self,
            Self::AuthCallbackReturn
                | Self::CollaborationJoin
                | Self::RemoteTargetOpen
                | Self::ManagedResume
        )
    }
}

/// Where the entry originated. Origins outside the shell can never bypass
/// review simply because they came from the OS or a browser.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EntrySourceClass {
    /// In-shell surface (Start Center, command palette) — local.
    InShellSurface,
    /// OS shell activation (file association, dock, jump list) — local.
    OsShell,
    /// System default browser callback / deep link.
    SystemDefaultBrowser,
    /// OS notification activation.
    OsNotification,
    /// System surface (lock screen, share sheet, widget).
    SystemSurface,
    /// First-party web origin.
    FirstPartyWeb,
    /// Trusted companion app / handoff.
    TrustedCompanion,
    /// Collaboration service.
    CollaborationService,
    /// External provider (third-party integration).
    ExternalProvider,
    /// Identity-provider auth callback.
    AuthProviderCallback,
    /// Managed admin surface / MDM-pushed action.
    ManagedAdminSurface,
    /// Local CLI / headless automation.
    LocalCli,
    /// Origin could not be verified as trusted.
    UnknownUntrusted,
}

impl EntrySourceClass {
    /// Stable token used in records, fixtures, and logs.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::InShellSurface => "in_shell_surface",
            Self::OsShell => "os_shell",
            Self::SystemDefaultBrowser => "system_default_browser",
            Self::OsNotification => "os_notification",
            Self::SystemSurface => "system_surface",
            Self::FirstPartyWeb => "first_party_web",
            Self::TrustedCompanion => "trusted_companion",
            Self::CollaborationService => "collaboration_service",
            Self::ExternalProvider => "external_provider",
            Self::AuthProviderCallback => "auth_provider_callback",
            Self::ManagedAdminSurface => "managed_admin_surface",
            Self::LocalCli => "local_cli",
            Self::UnknownUntrusted => "unknown_untrusted",
        }
    }

    /// True when the source is a local surface that does not by itself cross a
    /// boundary. Cross-boundary sources always force interstitial review.
    pub const fn is_local(self) -> bool {
        matches!(self, Self::InShellSurface | Self::OsShell | Self::LocalCli)
    }
}

/// The action the entry requests, viewed through an authority lens.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RequestedActionClass {
    /// Open an existing local context (no authority change).
    OpenExistingContext,
    /// Reopen / acknowledge and reopen an existing object.
    AcknowledgeAndReopen,
    /// Join a presence / collaboration session.
    JoinPresence,
    /// Resume a prior session.
    ResumeSession,
    /// Complete an auth return (token / scope grant).
    AuthReturn,
    /// Open a remote target (binds remote authority).
    RemoteTargetOpen,
    /// Resume a managed workspace under org policy.
    ManagedResume,
    /// Run a mutating command requested from outside the shell.
    MutatingCommandRequest,
    /// Explicitly widen privilege / authority.
    PrivilegedAuthorityWidening,
}

impl RequestedActionClass {
    /// Stable token used in records, fixtures, and logs.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::OpenExistingContext => "open_existing_context",
            Self::AcknowledgeAndReopen => "acknowledge_and_reopen",
            Self::JoinPresence => "join_presence",
            Self::ResumeSession => "resume_session",
            Self::AuthReturn => "auth_return",
            Self::RemoteTargetOpen => "remote_target_open",
            Self::ManagedResume => "managed_resume",
            Self::MutatingCommandRequest => "mutating_command_request",
            Self::PrivilegedAuthorityWidening => "privileged_authority_widening",
        }
    }

    /// True when the requested action raises authority beyond a plain read /
    /// open and therefore requires a reviewed interstitial.
    pub const fn is_authority_raising(self) -> bool {
        matches!(
            self,
            Self::JoinPresence
                | Self::ResumeSession
                | Self::AuthReturn
                | Self::RemoteTargetOpen
                | Self::ManagedResume
                | Self::MutatingCommandRequest
                | Self::PrivilegedAuthorityWidening
        )
    }
}

/// A boundary the entry crosses, recorded so the user can see *why* Aureline is
/// asking for confirmation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BoundaryClass {
    /// The action widens authority beyond plain open.
    AuthorityWidening,
    /// The target differs from the current target identity.
    TargetBoundary,
    /// The target belongs to a different tenant / organization.
    TenantBoundary,
    /// The action changes or re-evaluates workspace trust.
    TrustBoundary,
    /// The target is remote and binds remote authority.
    RemoteBoundary,
    /// Org policy must be reviewed before the action runs.
    PolicyBoundary,
}

impl BoundaryClass {
    /// Stable token used in records, fixtures, and logs.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::AuthorityWidening => "authority_widening",
            Self::TargetBoundary => "target_boundary",
            Self::TenantBoundary => "tenant_boundary",
            Self::TrustBoundary => "trust_boundary",
            Self::RemoteBoundary => "remote_boundary",
            Self::PolicyBoundary => "policy_boundary",
        }
    }
}

/// The class of target an entry resolves to.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EntryTargetKind {
    LocalFile,
    LocalFolder,
    WorkspaceRoot,
    RecentWorkEntry,
    ReviewOrWorkItem,
    CollaborationSession,
    RemoteWorkspace,
    ManagedWorkspace,
    AuthSession,
    CommandTarget,
}

impl EntryTargetKind {
    /// Stable token used in records, fixtures, and logs.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LocalFile => "local_file",
            Self::LocalFolder => "local_folder",
            Self::WorkspaceRoot => "workspace_root",
            Self::RecentWorkEntry => "recent_work_entry",
            Self::ReviewOrWorkItem => "review_or_work_item",
            Self::CollaborationSession => "collaboration_session",
            Self::RemoteWorkspace => "remote_workspace",
            Self::ManagedWorkspace => "managed_workspace",
            Self::AuthSession => "auth_session",
            Self::CommandTarget => "command_target",
        }
    }
}

/// Whether the resolved target is exactly what the entry claimed, or has
/// drifted away from it. Anything other than `ExactAvailable` preserves the
/// original intent through a placeholder rather than opening an empty shell.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TargetTruthState {
    /// The exact claimed object is present and reachable.
    ExactAvailable,
    /// A compatible-but-downgraded version is available.
    CompatibleDowngraded,
    /// The object moved or its alias / identity changed.
    MovedOrAliasChanged,
    /// The object is missing or unmounted.
    MissingOrUnmounted,
    /// Policy blocks opening this object.
    PolicyBlocked,
    /// Authentication / re-authentication is required first.
    AuthRequired,
    /// A remote target is currently unreachable.
    RemoteUnreachable,
    /// The target reference has expired.
    Expired,
    /// The reference is ambiguous and needs disambiguation.
    Ambiguous,
}

impl TargetTruthState {
    /// Stable token used in records, fixtures, and logs.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ExactAvailable => "exact_available",
            Self::CompatibleDowngraded => "compatible_downgraded",
            Self::MovedOrAliasChanged => "moved_or_alias_changed",
            Self::MissingOrUnmounted => "missing_or_unmounted",
            Self::PolicyBlocked => "policy_blocked",
            Self::AuthRequired => "auth_required",
            Self::RemoteUnreachable => "remote_unreachable",
            Self::Expired => "expired",
            Self::Ambiguous => "ambiguous",
        }
    }

    /// True when the exact claimed object is present and reachable.
    pub const fn resolves_to_exact_target(self) -> bool {
        matches!(self, Self::ExactAvailable)
    }

    /// True when the resolved target still carries the claimed object's bytes
    /// or layout, even if downgraded. Such targets open with a disclosure but
    /// without a missing-target placeholder.
    pub const fn carries_object(self) -> bool {
        matches!(self, Self::ExactAvailable | Self::CompatibleDowngraded)
    }
}

/// How the action changes or re-evaluates trust / policy authority.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AuthorityEffectClass {
    /// No authority change.
    NoChange,
    /// Workspace trust must be reviewed before the action runs.
    TrustReviewRequired,
    /// The action narrows authority (e.g. restricted mode).
    TrustNarrowing,
    /// Org policy must be reviewed.
    PolicyBoundaryReview,
    /// An auth scope grant is required.
    AuthScopeRequired,
    /// Remote authority must be rebound.
    RemoteAuthorityRebind,
    /// The target sits under a different tenant scope.
    TenantScopeChange,
}

impl AuthorityEffectClass {
    /// Stable token used in records, fixtures, and logs.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NoChange => "no_change",
            Self::TrustReviewRequired => "trust_review_required",
            Self::TrustNarrowing => "trust_narrowing",
            Self::PolicyBoundaryReview => "policy_boundary_review",
            Self::AuthScopeRequired => "auth_scope_required",
            Self::RemoteAuthorityRebind => "remote_authority_rebind",
            Self::TenantScopeChange => "tenant_scope_change",
        }
    }

    /// The boundary class this authority effect implies, if any.
    const fn implied_boundary(self) -> Option<BoundaryClass> {
        match self {
            Self::NoChange => None,
            Self::TrustReviewRequired | Self::TrustNarrowing => Some(BoundaryClass::TrustBoundary),
            Self::PolicyBoundaryReview => Some(BoundaryClass::PolicyBoundary),
            Self::AuthScopeRequired => Some(BoundaryClass::AuthorityWidening),
            Self::RemoteAuthorityRebind => Some(BoundaryClass::RemoteBoundary),
            Self::TenantScopeChange => Some(BoundaryClass::TenantBoundary),
        }
    }
}

/// Bounded fallback action offered when the target cannot be opened exactly.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FallbackAction {
    /// Cancel; nothing changes.
    CancelNoChange,
    /// Help the user locate a moved / missing target.
    LocateMissingTarget,
    /// Reconnect a remote / collaboration session first.
    ReconnectRequired,
    /// Re-authenticate before continuing.
    ReauthRequired,
    /// Inspect the target without opening or executing.
    InspectOnly,
    /// Open the surrounding context without restoring the missing object.
    OpenWithoutRestore,
    /// Retry later once the target is reachable.
    RetryLater,
    /// Return to the Start Center.
    ReturnToStartCenter,
    /// Export evidence for support.
    ExportEvidence,
}

impl FallbackAction {
    /// Stable token used in records, fixtures, and logs.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::CancelNoChange => "cancel_no_change",
            Self::LocateMissingTarget => "locate_missing_target",
            Self::ReconnectRequired => "reconnect_required",
            Self::ReauthRequired => "reauth_required",
            Self::InspectOnly => "inspect_only",
            Self::OpenWithoutRestore => "open_without_restore",
            Self::RetryLater => "retry_later",
            Self::ReturnToStartCenter => "return_to_start_center",
            Self::ExportEvidence => "export_evidence",
        }
    }
}

/// The role of an interstitial action.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum InterstitialActionKey {
    /// Proceed with the requested action via the canonical command.
    Confirm,
    /// Reject the action; nothing changes.
    Reject,
    /// Defer the decision and return to a safe surface.
    Defer,
}

impl InterstitialActionKey {
    /// Stable token used in records, fixtures, and logs.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Confirm => "confirm",
            Self::Reject => "reject",
            Self::Defer => "defer",
        }
    }
}

/// One action row exposed by the interstitial. Behavior binds to `command_id`,
/// never to a label, so the OS-origin path runs exactly the in-product command.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct InterstitialAction {
    pub action_key: InterstitialActionKey,
    pub command_id: String,
    pub enabled: bool,
    pub requires_confirmation: bool,
    /// Redaction-safe description of what this action does next.
    pub outcome_label: String,
}

/// Truthful placeholder shown when the target is not exactly available, with
/// bounded fallback actions that preserve the original intent.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TargetPlaceholder {
    /// Always `true`: the placeholder is announced rather than silently
    /// substituting a generic surface.
    pub announced: bool,
    /// Redaction-safe placeholder text (e.g. "Workspace moved").
    pub placeholder_label: String,
    /// Redaction-safe statement of the intent being preserved.
    pub preserved_intent_label: String,
    /// Bounded fallback actions; never empty.
    pub fallback_actions: Vec<FallbackAction>,
}

/// Target identity and scope disclosed before the action runs.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TargetScope {
    pub target_kind: EntryTargetKind,
    /// Opaque, log-safe object identity. Never a raw path or URL.
    pub object_identity_ref: String,
    /// Redaction-safe target label.
    pub target_label: String,
    /// Redaction-safe workspace scope label.
    pub workspace_scope_label: String,
    /// Redaction-safe tenant / organization scope label, where relevant.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tenant_scope_label: Option<String>,
    /// Redaction-safe channel or build-owner label, where relevant.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub channel_or_build_owner_label: Option<String>,
    pub truth_state: TargetTruthState,
    /// True when target identity itself must be reviewed before proceeding.
    pub identity_review_required: bool,
}

/// Input describing one inbound entry request to evaluate.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EntryInterstitialRequest {
    /// Opaque, log-safe id for this entry attempt.
    pub interstitial_id: String,
    pub kind: EntryInterstitialKind,
    pub source_class: EntrySourceClass,
    /// Redaction-safe source label.
    pub source_label: String,
    pub requested_action: RequestedActionClass,
    pub target: TargetScope,
    pub authority_effect: AuthorityEffectClass,
    /// True when the resolved target belongs to a different tenant than the
    /// active session.
    pub crosses_tenant_boundary: bool,
    /// The canonical in-product command id this action maps to. The confirm
    /// action is bound to exactly this id so authority cannot widen relative to
    /// the in-product path.
    pub canonical_command_id: String,
}

/// The decision for one entry request.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "decision", rename_all = "snake_case")]
pub enum EntryInterstitialDecision {
    /// Plain local open: no boundary crossed; the fast path runs directly.
    PlainLocalOpen(PlainLocalOpenAdmission),
    /// A reviewed interstitial is required before execution.
    InterstitialRequired(Box<EntryInterstitialRecord>),
}

impl EntryInterstitialDecision {
    /// True when a reviewed interstitial is required.
    pub const fn requires_interstitial(&self) -> bool {
        matches!(self, Self::InterstitialRequired(_))
    }

    /// Borrow the record when an interstitial is required.
    pub fn record(&self) -> Option<&EntryInterstitialRecord> {
        match self {
            Self::InterstitialRequired(record) => Some(record),
            Self::PlainLocalOpen(_) => None,
        }
    }
}

/// Fast-path admission for a plain local open that crosses no boundary.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PlainLocalOpenAdmission {
    pub record_kind: String,
    pub entry_interstitial_schema_version: u32,
    pub interstitial_id: String,
    pub kind: EntryInterstitialKind,
    /// Why no interstitial was required.
    pub reason: String,
    /// The canonical command the fast path runs.
    pub canonical_command_ref: String,
}

/// Canonical interstitial projection for a high-authority / cross-boundary
/// entry path.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EntryInterstitialRecord {
    pub record_kind: String,
    pub entry_interstitial_schema_version: u32,
    pub interstitial_id: String,
    pub kind: EntryInterstitialKind,
    pub source_class: EntrySourceClass,
    pub source_label: String,
    pub requested_action: RequestedActionClass,
    /// The boundaries crossed, in stable order. Never empty for a record.
    pub boundary_classes: Vec<BoundaryClass>,
    pub authority_effect: AuthorityEffectClass,
    pub target_scope: TargetScope,
    /// Present when the target is not exactly available.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target_placeholder: Option<TargetPlaceholder>,
    /// Redaction-safe statement of the trust/policy effect.
    pub trust_policy_effect_label: String,
    /// Redaction-safe explanation of why confirmation is being asked.
    pub confirm_explanation: String,
    /// Redaction-safe statement of what happens on reject.
    pub reject_outcome_label: String,
    /// Redaction-safe statement of what happens on defer.
    pub defer_outcome_label: String,
    pub actions: Vec<InterstitialAction>,
    /// Always `true`: the entry cannot execute silently.
    pub silent_execution_forbidden: bool,
    /// Always `true`: the confirm command equals the canonical in-product id.
    pub authority_not_widened: bool,
    /// Always `false`: reopen never lands on a generic home surface.
    pub reopens_generic_home: bool,
    /// The canonical in-product command the confirm action runs.
    pub canonical_command_ref: String,
    pub summary_line: String,
}

impl EntryInterstitialRecord {
    /// Borrow the confirm action, if present.
    pub fn confirm_action(&self) -> Option<&InterstitialAction> {
        self.actions
            .iter()
            .find(|a| a.action_key == InterstitialActionKey::Confirm)
    }

    /// True when the confirm action runs exactly the canonical in-product
    /// command (so authority cannot widen relative to the in-product path).
    pub fn confirm_matches_canonical(&self) -> bool {
        self.confirm_action()
            .map(|a| a.command_id == self.canonical_command_ref)
            .unwrap_or(false)
    }

    /// True when the reopen resolves to the exact object or an announced
    /// placeholder — never a generic home surface.
    pub fn reopens_exact_or_placeholder(&self) -> bool {
        self.target_scope.truth_state.resolves_to_exact_target()
            || self
                .target_placeholder
                .as_ref()
                .map(|p| p.announced)
                .unwrap_or(false)
    }
}

/// Computes the boundary classes a request crosses, in stable sorted order.
///
/// An empty result means the request is a plain local open and no interstitial
/// is required.
pub fn boundary_classes_for(request: &EntryInterstitialRequest) -> Vec<BoundaryClass> {
    let mut set: std::collections::BTreeSet<BoundaryClass> = std::collections::BTreeSet::new();

    if request.requested_action.is_authority_raising() {
        set.insert(BoundaryClass::AuthorityWidening);
    }
    // A non-local origin (browser callback, notification, external provider) is
    // always disclosed via `source_class`; it crosses a target boundary only
    // when the resolved target identity itself must be reviewed.
    if request.target.identity_review_required {
        set.insert(BoundaryClass::TargetBoundary);
    }
    if !request.target.truth_state.resolves_to_exact_target() {
        set.insert(BoundaryClass::TargetBoundary);
    }
    if request.crosses_tenant_boundary || request.target.tenant_scope_label.is_some() {
        match request.kind {
            EntryInterstitialKind::CollaborationJoin
            | EntryInterstitialKind::ManagedResume
            | EntryInterstitialKind::RemoteTargetOpen => {
                set.insert(BoundaryClass::TenantBoundary);
            }
            _ if request.crosses_tenant_boundary => {
                set.insert(BoundaryClass::TenantBoundary);
            }
            _ => {}
        }
    }
    if let Some(boundary) = request.authority_effect.implied_boundary() {
        set.insert(boundary);
    }
    match request.kind {
        EntryInterstitialKind::RemoteTargetOpen => {
            set.insert(BoundaryClass::RemoteBoundary);
        }
        EntryInterstitialKind::AuthCallbackReturn => {
            set.insert(BoundaryClass::AuthorityWidening);
        }
        EntryInterstitialKind::CollaborationJoin => {
            set.insert(BoundaryClass::RemoteBoundary);
        }
        EntryInterstitialKind::ManagedResume => {
            set.insert(BoundaryClass::PolicyBoundary);
        }
        EntryInterstitialKind::ProtocolDeepLink | EntryInterstitialKind::NotificationReopen => {}
    }

    set.into_iter().collect()
}

/// Builds the bounded fallback set for a non-exact target.
fn fallback_actions_for(truth_state: TargetTruthState) -> Vec<FallbackAction> {
    let mut actions = vec![FallbackAction::CancelNoChange];
    match truth_state {
        TargetTruthState::ExactAvailable => {}
        TargetTruthState::CompatibleDowngraded => {
            actions.push(FallbackAction::InspectOnly);
            actions.push(FallbackAction::OpenWithoutRestore);
        }
        TargetTruthState::MovedOrAliasChanged | TargetTruthState::Ambiguous => {
            actions.push(FallbackAction::LocateMissingTarget);
            actions.push(FallbackAction::InspectOnly);
        }
        TargetTruthState::MissingOrUnmounted | TargetTruthState::Expired => {
            actions.push(FallbackAction::LocateMissingTarget);
            actions.push(FallbackAction::OpenWithoutRestore);
            actions.push(FallbackAction::ReturnToStartCenter);
        }
        TargetTruthState::PolicyBlocked => {
            actions.push(FallbackAction::InspectOnly);
            actions.push(FallbackAction::ExportEvidence);
            actions.push(FallbackAction::ReturnToStartCenter);
        }
        TargetTruthState::AuthRequired => {
            actions.push(FallbackAction::ReauthRequired);
            actions.push(FallbackAction::ReturnToStartCenter);
        }
        TargetTruthState::RemoteUnreachable => {
            actions.push(FallbackAction::ReconnectRequired);
            actions.push(FallbackAction::RetryLater);
            actions.push(FallbackAction::ReturnToStartCenter);
        }
    }
    actions
}

/// True when, given the target truth state, the confirm action should be
/// disabled (the exact action cannot run; only fallbacks are safe).
const fn confirm_blocked_by_truth(truth_state: TargetTruthState) -> bool {
    matches!(
        truth_state,
        TargetTruthState::MissingOrUnmounted
            | TargetTruthState::PolicyBlocked
            | TargetTruthState::Expired
            | TargetTruthState::RemoteUnreachable
            | TargetTruthState::Ambiguous
    )
}

/// Evaluates an entry request, returning either the fast path or a reviewed
/// interstitial record.
pub fn evaluate_entry_interstitial(
    request: &EntryInterstitialRequest,
) -> EntryInterstitialDecision {
    let boundaries = boundary_classes_for(request);

    if boundaries.is_empty() && !request.kind.always_crosses_boundary() {
        return EntryInterstitialDecision::PlainLocalOpen(PlainLocalOpenAdmission {
            record_kind: PLAIN_LOCAL_OPEN_RECORD_KIND.to_string(),
            entry_interstitial_schema_version: ENTRY_INTERSTITIAL_SCHEMA_VERSION,
            interstitial_id: request.interstitial_id.clone(),
            kind: request.kind,
            reason: "plain_local_open_no_boundary_crossed".to_string(),
            canonical_command_ref: request.canonical_command_id.clone(),
        });
    }

    EntryInterstitialDecision::InterstitialRequired(Box::new(materialize_entry_interstitial(
        request, boundaries,
    )))
}

/// Materializes the interstitial record for a request known to cross a
/// boundary.
pub fn materialize_entry_interstitial(
    request: &EntryInterstitialRequest,
    boundary_classes: Vec<BoundaryClass>,
) -> EntryInterstitialRecord {
    let exact = request.target.truth_state.resolves_to_exact_target();

    let target_placeholder = if exact {
        None
    } else {
        Some(TargetPlaceholder {
            announced: true,
            placeholder_label: placeholder_label_for(request),
            preserved_intent_label: format!(
                "Keep {action} ready for {target} once it is reachable",
                action = human_action(request.requested_action),
                target = request.target.target_label,
            ),
            fallback_actions: fallback_actions_for(request.target.truth_state),
        })
    };

    let confirm_enabled = !confirm_blocked_by_truth(request.target.truth_state);
    let confirm_forbidden_reason = if confirm_enabled {
        "none"
    } else {
        request.target.truth_state.as_str()
    };

    let actions = vec![
        InterstitialAction {
            action_key: InterstitialActionKey::Confirm,
            command_id: request.canonical_command_id.clone(),
            enabled: confirm_enabled,
            requires_confirmation: true,
            outcome_label: if confirm_enabled {
                format!(
                    "Run {action} via {command}",
                    action = human_action(request.requested_action),
                    command = request.canonical_command_id,
                )
            } else {
                format!(
                    "Cannot run yet: target is {state}",
                    state = request.target.truth_state.as_str()
                )
            },
        },
        InterstitialAction {
            action_key: InterstitialActionKey::Reject,
            command_id: ENTRY_REJECT_COMMAND_ID.to_string(),
            enabled: true,
            requires_confirmation: false,
            outcome_label: "Nothing opens and nothing changes".to_string(),
        },
        InterstitialAction {
            action_key: InterstitialActionKey::Defer,
            command_id: ENTRY_DEFER_COMMAND_ID.to_string(),
            enabled: true,
            requires_confirmation: false,
            outcome_label: "Decide later from the Start Center; nothing changes now".to_string(),
        },
    ];

    let boundary_tokens = boundary_classes
        .iter()
        .map(|b| b.as_str())
        .collect::<Vec<_>>()
        .join("|");

    let trust_policy_effect_label = trust_policy_effect_label(request);
    let confirm_explanation = format!(
        "Aureline is asking because this {kind} from {source} crosses: {boundaries}. {effect}",
        kind = human_kind(request.kind),
        source = request.source_label,
        boundaries = boundary_tokens,
        effect = trust_policy_effect_label,
    );

    let summary_line = format!(
        "entry_interstitial: kind={kind}; source={source}; action={action}; \
         boundaries={boundaries}; target_truth={truth}; confirm_command={command}; \
         silent_execution_forbidden=true; reopens_generic_home=false",
        kind = request.kind.as_str(),
        source = request.source_class.as_str(),
        action = request.requested_action.as_str(),
        boundaries = boundary_tokens,
        truth = request.target.truth_state.as_str(),
        command = request.canonical_command_id,
    );

    let _ = confirm_forbidden_reason;

    EntryInterstitialRecord {
        record_kind: ENTRY_INTERSTITIAL_RECORD_KIND.to_string(),
        entry_interstitial_schema_version: ENTRY_INTERSTITIAL_SCHEMA_VERSION,
        interstitial_id: request.interstitial_id.clone(),
        kind: request.kind,
        source_class: request.source_class,
        source_label: request.source_label.clone(),
        requested_action: request.requested_action,
        boundary_classes,
        authority_effect: request.authority_effect,
        target_scope: request.target.clone(),
        target_placeholder,
        trust_policy_effect_label,
        confirm_explanation,
        reject_outcome_label: "Nothing opens and nothing changes".to_string(),
        defer_outcome_label: "Return to the Start Center; the request is preserved, not run"
            .to_string(),
        actions,
        silent_execution_forbidden: true,
        authority_not_widened: true,
        reopens_generic_home: false,
        canonical_command_ref: request.canonical_command_id.clone(),
        summary_line,
    }
}

fn placeholder_label_for(request: &EntryInterstitialRequest) -> String {
    let what = match request.target.target_kind {
        EntryTargetKind::WorkspaceRoot | EntryTargetKind::RemoteWorkspace => "Workspace",
        EntryTargetKind::ManagedWorkspace => "Managed workspace",
        EntryTargetKind::CollaborationSession => "Collaboration session",
        EntryTargetKind::AuthSession => "Sign-in session",
        EntryTargetKind::ReviewOrWorkItem => "Linked item",
        EntryTargetKind::RecentWorkEntry => "Recent item",
        EntryTargetKind::LocalFile | EntryTargetKind::LocalFolder => "File",
        EntryTargetKind::CommandTarget => "Command target",
    };
    let why = match request.target.truth_state {
        TargetTruthState::ExactAvailable => "available",
        TargetTruthState::CompatibleDowngraded => "available in a compatible form",
        TargetTruthState::MovedOrAliasChanged => "moved",
        TargetTruthState::MissingOrUnmounted => "not found",
        TargetTruthState::PolicyBlocked => "blocked by policy",
        TargetTruthState::AuthRequired => "waiting for sign-in",
        TargetTruthState::RemoteUnreachable => "unreachable",
        TargetTruthState::Expired => "expired",
        TargetTruthState::Ambiguous => "ambiguous",
    };
    format!("{what} {why}")
}

fn trust_policy_effect_label(request: &EntryInterstitialRequest) -> String {
    match request.authority_effect {
        AuthorityEffectClass::NoChange => "Trust and policy are unchanged.".to_string(),
        AuthorityEffectClass::TrustReviewRequired => {
            "Workspace trust will be reviewed before anything runs.".to_string()
        }
        AuthorityEffectClass::TrustNarrowing => {
            "This narrows capability (restricted mode).".to_string()
        }
        AuthorityEffectClass::PolicyBoundaryReview => {
            "Org policy applies and will be reviewed first.".to_string()
        }
        AuthorityEffectClass::AuthScopeRequired => {
            "A sign-in scope grant is required first.".to_string()
        }
        AuthorityEffectClass::RemoteAuthorityRebind => {
            "Remote authority will be rebound for this target.".to_string()
        }
        AuthorityEffectClass::TenantScopeChange => {
            "This target sits under a different organization.".to_string()
        }
    }
}

const fn human_kind(kind: EntryInterstitialKind) -> &'static str {
    match kind {
        EntryInterstitialKind::ProtocolDeepLink => "deep-link open",
        EntryInterstitialKind::AuthCallbackReturn => "sign-in return",
        EntryInterstitialKind::CollaborationJoin => "collaboration join",
        EntryInterstitialKind::RemoteTargetOpen => "remote open",
        EntryInterstitialKind::ManagedResume => "managed resume",
        EntryInterstitialKind::NotificationReopen => "notification reopen",
    }
}

const fn human_action(action: RequestedActionClass) -> &'static str {
    match action {
        RequestedActionClass::OpenExistingContext => "open",
        RequestedActionClass::AcknowledgeAndReopen => "reopen",
        RequestedActionClass::JoinPresence => "join",
        RequestedActionClass::ResumeSession => "resume",
        RequestedActionClass::AuthReturn => "complete sign-in",
        RequestedActionClass::RemoteTargetOpen => "remote open",
        RequestedActionClass::ManagedResume => "managed resume",
        RequestedActionClass::MutatingCommandRequest => "run command",
        RequestedActionClass::PrivilegedAuthorityWidening => "widen authority",
    }
}

/// Writes an entry-interstitial record to
/// `<recovery_root>/entry_interstitial_latest.json`.
pub fn write_entry_interstitial_log(
    recovery_root: &Path,
    record: &EntryInterstitialRecord,
) -> Result<(), String> {
    std::fs::create_dir_all(recovery_root)
        .map_err(|err| format!("create recovery root failed: {err}"))?;
    let path = recovery_root.join("entry_interstitial_latest.json");
    let json = serde_json::to_string_pretty(record)
        .map_err(|err| format!("serialize entry interstitial failed: {err}"))?;
    std::fs::write(&path, json).map_err(|err| format!("write {} failed: {err}", path.display()))?;
    Ok(())
}

/// Renders the interstitial as a single-line status string.
pub fn entry_interstitial_status_line(record: &EntryInterstitialRecord) -> String {
    record.summary_line.clone()
}

#[cfg(test)]
mod tests;
