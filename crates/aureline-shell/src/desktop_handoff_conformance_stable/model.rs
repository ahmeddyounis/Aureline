//! Canonical stable truth model for **desktop handoff, file-association,
//! protocol-handler ownership, embedded auth-return-path, and system-browser
//! default conformance** on a claimed-stable desktop shell.
//!
//! ## Why one governed record per handoff posture
//!
//! An OS-originated entry path — a file association, a protocol-handler
//! invocation, a system `open`, a default-browser auth callback, a
//! reveal-in-shell, a recent-item or jump-list reopen, a removable-volume or
//! network-share reopen, a native open/save dialog — is replacement-grade only
//! when it lands on the **literal requested target** (or a truthful
//! placeholder), names **which channel or bundle owns the handler**, keeps
//! **trust / profile / tenant** review ahead of any widened authority, and
//! defaults identity and auth handoff to the **system browser** unless an
//! exception is surfaced explicitly. Competitors routinely fuse these: a
//! side-by-side Preview install silently steals the Stable channel's file
//! association, a protocol handler reopens a generic home pane instead of the
//! deep-linked object, an embedded web view swallows an auth callback with no
//! disclosed exception, or a missing removable root disappears instead of
//! rendering a recoverable placeholder.
//!
//! This module mints one governed [`DesktopHandoffConformanceRecord`] per
//! claimed-stable handoff posture. The record binds, for a single entry-path
//! identity:
//!
//! - **Typed target intent** — the literal target, the source-locator /
//!   deep-link intent, the requested action, the resulting mode, and the
//!   canonical object identity are preserved end to end; the path never reopens
//!   a generic shell or the wrong install.
//! - **Handler ownership** — the owning channel and build are explicit,
//!   side-by-side Stable / Preview / Beta / portable / admin-owned channels are
//!   enumerated, ownership never degrades into last-writer-wins, and handler
//!   spoofing fails closed.
//! - **System-browser default conformance** — claimed identity and auth rows
//!   default to system-browser handoff; a row that uses another path surfaces
//!   the exception explicitly with its target scope, return path, and recovery
//!   behaviour.
//! - **Trust / profile / tenant review** — review precedes any widened
//!   authority or resumed remote action; authority is never widened silently.
//! - **Truthful recovery** — a moved, removable, network, or missing target
//!   renders a recoverable placeholder with its last-seen identity, an
//!   unsaved-local-state posture, and explicit Locate / Open cached context /
//!   Close placeholder actions instead of disappearing or replaying work.
//! - **Per-OS conformance** — macOS, Windows, and Linux profiles each carry
//!   current proof.
//! - **A public claim ceiling** and **automatic narrowing** — a posture that
//!   cannot prove a pillar, or whose lowest binding surface marker is below
//!   Stable, narrows below Stable with a named reason instead of inheriting an
//!   adjacent green row.
//!
//! The desktop handoff-review UI, the CLI inspector, Help/About, and the
//! diagnostics support export read this record verbatim instead of cloning
//! status text. The entry-surface vocabulary, the handler-ownership classes,
//! the target-availability classes, and the system-browser exception classes
//! are **not** reinvented here: the record projects the live
//! [`crate::platform_integration`] native desktop contract packet, the
//! [`crate::deeplink::native_handoff`] handoff vocabulary, and the
//! [`crate::system_browser_return_paths`] auth-return page.

use std::collections::BTreeSet;

use serde::{Deserialize, Serialize};

use crate::deeplink::native_handoff::TargetAvailabilityClass;
use crate::notification_attention_stable::model::{
    is_canonical_object_ref, AccessibilityDisclosure, AttentionRouteSurface, EntryRouteRecord,
    LayoutMode, LifecycleMarker, RecoveryActionRole, RecoveryRouteRecord, StableClaimClass,
};
use crate::system_browser_return_paths::SystemBrowserPolicyExceptionClass;

/// Stable record-kind tag carried in serialized records.
pub const DESKTOP_HANDOFF_CONFORMANCE_RECORD_KIND: &str = "desktop_handoff_conformance_record";

/// Schema version for the [`DesktopHandoffConformanceRecord`] payload shape.
pub const DESKTOP_HANDOFF_CONFORMANCE_SCHEMA_VERSION: u32 = 1;

/// Shared contract ref consumed by every surface that ingests this record.
pub const DESKTOP_HANDOFF_CONFORMANCE_SHARED_CONTRACT_REF: &str =
    "shell:desktop_handoff_conformance_stable:v1";

/// Reviewer-facing notice rendered on every handoff-conformance surface.
pub const DESKTOP_HANDOFF_CONFORMANCE_NOTICE: &str =
    "Desktop-handoff truth: every OS-originated entry path — file association, protocol handler, \
     system open, default-browser auth callback, reveal-in-shell, recent-item or jump-list reopen, \
     removable-volume or network-share reopen, and native open/save — is bound to one typed target \
     intent that preserves the literal target, source-locator and deep-link intent, requested \
     action, resulting mode, and canonical object identity rather than reopening a generic shell or \
     the wrong install; handler ownership is explicit, naming the owning channel and build and \
     enumerating side-by-side Stable, Preview, Beta, portable, and admin-owned channels so file \
     associations, protocol handlers, auth callbacks, and recent-item registration never degrade \
     into last-writer-wins and handler spoofing fails closed; claimed identity and auth rows \
     default to system-browser handoff and any row that uses another path surfaces the exception \
     explicitly with its target scope, return path, and recovery behaviour; trust, profile, and \
     tenant review precedes any widened authority or resumed remote action and authority is never \
     widened silently; a moved, removable, network, or missing target renders a recoverable \
     placeholder with its last-seen identity, an unsaved-local-state posture, and explicit Locate, \
     Open cached context, and Close placeholder actions instead of disappearing or replaying work; \
     native open/save/reveal flows surface the canonical target path, overwrite or read-only \
     posture, and profile/remote boundary notes when the target is not the local default; macOS, \
     Windows, and Linux profiles each carry current proof; the desktop handoff review, CLI inspect, \
     Help/About, and support export read one shared record; and a posture that cannot prove a \
     pillar, or whose lowest binding surface marker is below Stable, narrows below Stable with a \
     named reason rather than inheriting an adjacent green row.";

/// Upper bound on a reviewable explanation sentence.
const MAX_SENTENCE_CHARS: usize = 1024;
/// Upper bound on a present ref.
const MAX_REF_CHARS: usize = 200;

fn is_reviewable_sentence(text: &str) -> bool {
    let trimmed = text.trim();
    !trimmed.is_empty() && trimmed.len() <= MAX_SENTENCE_CHARS
}

fn is_present_ref(text: &str) -> bool {
    let trimmed = text.trim();
    !trimmed.is_empty() && trimmed.len() <= MAX_REF_CHARS
}

fn require_canonical_ref(field: &'static str, value: &str) -> Result<(), BuildError> {
    if is_canonical_object_ref(value) {
        Ok(())
    } else {
        Err(BuildError::NonCanonicalRef {
            field,
            value: value.to_string(),
        })
    }
}

fn require_present_ref(field: &'static str, value: &str) -> Result<(), BuildError> {
    if is_present_ref(value) {
        Ok(())
    } else {
        Err(BuildError::MissingUpstreamRef { field })
    }
}

// ---------------------------------------------------------------------------
// Entry path + channel vocabularies
// ---------------------------------------------------------------------------

/// The OS-originated entry path a record covers. Each value maps to a typed
/// target intent rather than a generic shell launch.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EntryPathClass {
    /// A registered file-type association activation.
    FileAssociation,
    /// A custom-scheme protocol-handler invocation.
    ProtocolHandler,
    /// A generic system `open` of a workspace or file.
    SystemOpen,
    /// A default-browser auth callback returning to the app.
    DefaultBrowserAuthCallback,
    /// A reveal-in-system-shell affordance.
    RevealInSystemShell,
    /// A dock/taskbar/Start recent-item reopen.
    RecentItemReopen,
    /// A dock/taskbar jump-list style action.
    JumpListAction,
    /// A reopen whose target lives on a removable volume.
    RemovableVolumeReopen,
    /// A reopen whose target lives on a network share.
    NetworkShareReopen,
    /// A native open/save dialog result.
    NativeOpenSave,
}

impl EntryPathClass {
    /// Stable token recorded in fixtures and exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::FileAssociation => "file_association",
            Self::ProtocolHandler => "protocol_handler",
            Self::SystemOpen => "system_open",
            Self::DefaultBrowserAuthCallback => "default_browser_auth_callback",
            Self::RevealInSystemShell => "reveal_in_system_shell",
            Self::RecentItemReopen => "recent_item_reopen",
            Self::JumpListAction => "jump_list_action",
            Self::RemovableVolumeReopen => "removable_volume_reopen",
            Self::NetworkShareReopen => "network_share_reopen",
            Self::NativeOpenSave => "native_open_save",
        }
    }

    /// True when the entry path carries a claimed-identity or auth handoff that
    /// must default to the system browser unless an exception is disclosed.
    pub const fn is_auth_path(self) -> bool {
        matches!(self, Self::DefaultBrowserAuthCallback)
    }

    /// Every entry-path class the claimed-stable matrix must cover at least once.
    pub const REQUIRED: [Self; 10] = [
        Self::FileAssociation,
        Self::ProtocolHandler,
        Self::SystemOpen,
        Self::DefaultBrowserAuthCallback,
        Self::RevealInSystemShell,
        Self::RecentItemReopen,
        Self::JumpListAction,
        Self::RemovableVolumeReopen,
        Self::NetworkShareReopen,
        Self::NativeOpenSave,
    ];
}

/// Side-by-side install channel that can own a handoff surface. Ownership must
/// be explicit per channel so one install cannot silently steal another's file
/// association, protocol handler, auth callback, or recent-item registration.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ChannelClass {
    /// The Stable release channel.
    Stable,
    /// The Preview channel.
    Preview,
    /// The Beta channel.
    Beta,
    /// A portable, local-only install.
    Portable,
    /// An admin-managed / policy-owned install.
    AdminManaged,
}

impl ChannelClass {
    /// Stable token recorded in fixtures and exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Stable => "stable",
            Self::Preview => "preview",
            Self::Beta => "beta",
            Self::Portable => "portable",
            Self::AdminManaged => "admin_managed",
        }
    }

    /// Every channel a handler-ownership disclosure must enumerate so the
    /// owning channel is unambiguous across side-by-side installs.
    pub const REQUIRED: [Self; 5] = [
        Self::Stable,
        Self::Preview,
        Self::Beta,
        Self::Portable,
        Self::AdminManaged,
    ];
}

/// Per-OS desktop profile a conformance row covers.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PlatformProfileClass {
    /// macOS (universal).
    #[serde(rename = "macos")]
    MacOs,
    /// Windows (x86_64).
    Windows,
    /// Linux (GNOME/Wayland, x86_64).
    Linux,
}

impl PlatformProfileClass {
    /// Stable token recorded in fixtures and exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::MacOs => "macos",
            Self::Windows => "windows",
            Self::Linux => "linux",
        }
    }

    /// Every per-OS profile a Stable conformance posture must cover.
    pub const REQUIRED: [Self; 3] = [Self::MacOs, Self::Windows, Self::Linux];
}

/// Surface that ingests the shared handoff-conformance record.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum HandoffTruthSurface {
    /// The desktop handoff-review surface.
    DesktopHandoffReview,
    /// The CLI / headless inspector.
    CliInspect,
    /// The Help/About surface.
    HelpAbout,
    /// The diagnostics support export.
    SupportExport,
}

impl HandoffTruthSurface {
    /// Stable token recorded in fixtures and exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::DesktopHandoffReview => "desktop_handoff_review",
            Self::CliInspect => "cli_inspect",
            Self::HelpAbout => "help_about",
            Self::SupportExport => "support_export",
        }
    }

    /// The four surfaces that must all bind the shared record.
    pub const REQUIRED: [Self; 4] = [
        Self::DesktopHandoffReview,
        Self::CliInspect,
        Self::HelpAbout,
        Self::SupportExport,
    ];
}

/// Closed recovery-action vocabulary exposed on a handoff posture.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum HandoffRecoveryAction {
    /// Open the literal bound target.
    OpenBoundTarget,
    /// Locate a moved or unavailable target.
    LocateTarget,
    /// Open a read-only cached context for an unavailable target.
    OpenCachedContext,
    /// Close the truthful placeholder.
    ClosePlaceholder,
    /// Reconnect a remote-backed session (without resuming authority silently).
    ReconnectSession,
    /// Re-authenticate through the system browser after an expired callback.
    Reauthenticate,
    /// Review which channel or bundle owns the handler.
    ReviewHandlerOwnership,
    /// Export a redacted handoff-support packet.
    ExportHandoffSupport,
}

impl HandoffRecoveryAction {
    /// Stable action id quoted across surfaces.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::OpenBoundTarget => "open_bound_target",
            Self::LocateTarget => "locate_target",
            Self::OpenCachedContext => "open_cached_context",
            Self::ClosePlaceholder => "close_placeholder",
            Self::ReconnectSession => "reconnect_session",
            Self::Reauthenticate => "reauthenticate",
            Self::ReviewHandlerOwnership => "review_handler_ownership",
            Self::ExportHandoffSupport => "export_handoff_support",
        }
    }

    /// Reviewer-facing label.
    pub const fn surface_label(self) -> &'static str {
        match self {
            Self::OpenBoundTarget => "Open target",
            Self::LocateTarget => "Locate target",
            Self::OpenCachedContext => "Open cached context",
            Self::ClosePlaceholder => "Close placeholder",
            Self::ReconnectSession => "Reconnect session",
            Self::Reauthenticate => "Re-authenticate",
            Self::ReviewHandlerOwnership => "Review handler ownership",
            Self::ExportHandoffSupport => "Export handoff support",
        }
    }

    /// Placement / confirmation role for this action.
    pub const fn role(self) -> RecoveryActionRole {
        match self {
            Self::OpenBoundTarget | Self::LocateTarget | Self::OpenCachedContext => {
                RecoveryActionRole::Primary
            }
            Self::ClosePlaceholder | Self::ReconnectSession | Self::Reauthenticate => {
                RecoveryActionRole::Recovery
            }
            Self::ReviewHandlerOwnership | Self::ExportHandoffSupport => {
                RecoveryActionRole::Secondary
            }
        }
    }

    /// Builds a route record for this action.
    pub fn route(self) -> RecoveryRouteRecord {
        RecoveryRouteRecord {
            action_id: self.as_str().to_string(),
            action_label: self.surface_label().to_string(),
            action_role: self.role(),
            keyboard_reachable: true,
        }
    }

    /// The recovery actions every handoff posture must expose, regardless of
    /// target availability.
    pub const REQUIRED: [Self; 3] = [
        Self::OpenBoundTarget,
        Self::ReviewHandlerOwnership,
        Self::ExportHandoffSupport,
    ];
}

/// Returns the recovery routes a posture must expose, in rendered order, given
/// the target's recovery needs.
pub fn required_recovery_routes(
    needs_placeholder: bool,
    needs_reauth: bool,
    needs_reconnect: bool,
) -> Vec<RecoveryRouteRecord> {
    let mut actions = vec![HandoffRecoveryAction::OpenBoundTarget];
    if needs_placeholder {
        actions.push(HandoffRecoveryAction::LocateTarget);
        actions.push(HandoffRecoveryAction::OpenCachedContext);
        actions.push(HandoffRecoveryAction::ClosePlaceholder);
    }
    if needs_reconnect {
        actions.push(HandoffRecoveryAction::ReconnectSession);
    }
    if needs_reauth {
        actions.push(HandoffRecoveryAction::Reauthenticate);
    }
    actions.push(HandoffRecoveryAction::ReviewHandlerOwnership);
    actions.push(HandoffRecoveryAction::ExportHandoffSupport);
    actions
        .into_iter()
        .map(HandoffRecoveryAction::route)
        .collect()
}

/// Closed reason a posture is narrowed below Stable. Required whenever the claim
/// class is below the cutline; forbidden when it is Stable.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum HandoffNarrowingReason {
    /// The typed target intent (literal target, deep-link intent, resulting
    /// mode) is not preserved end to end.
    TypedIntentNotPreserved,
    /// Handler ownership is not explicit or degrades into last-writer-wins.
    HandlerOwnershipNotExplicit,
    /// A claimed identity / auth row does not default to the system browser and
    /// did not surface an explicit exception.
    AuthNotSystemBrowserDefault,
    /// Trust / profile / tenant review does not precede widened authority.
    TrustReviewNotEnforced,
    /// A moved or unavailable target does not render a truthful, recoverable
    /// placeholder.
    RecoveryNotTruthful,
    /// Per-OS conformance is incomplete for a claimed stable profile.
    PlatformConformanceIncomplete,
    /// The binding surface's own lifecycle marker is below Stable, so it must
    /// not inherit Stable by adjacency.
    SurfaceNotYetStable,
}

impl HandoffNarrowingReason {
    /// Stable token recorded in fixtures and exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::TypedIntentNotPreserved => "typed_intent_not_preserved",
            Self::HandlerOwnershipNotExplicit => "handler_ownership_not_explicit",
            Self::AuthNotSystemBrowserDefault => "auth_not_system_browser_default",
            Self::TrustReviewNotEnforced => "trust_review_not_enforced",
            Self::RecoveryNotTruthful => "recovery_not_truthful",
            Self::PlatformConformanceIncomplete => "platform_conformance_incomplete",
            Self::SurfaceNotYetStable => "surface_not_yet_stable",
        }
    }
}

// ---------------------------------------------------------------------------
// Pillar inputs
// ---------------------------------------------------------------------------

/// The typed target intent the entry path preserves end to end.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TypedTargetIntent {
    /// OS-facing source-surface token (e.g. `file_association`).
    pub source_surface_token: String,
    /// Origin class token supplied by the handoff validator.
    pub origin_class_token: String,
    /// Requested action class token after parsing.
    pub requested_action_class_token: String,
    /// Literal target label supplied by the OS or platform surface.
    pub literal_target_label: String,
    /// Export-safe literal target ref.
    pub literal_target_ref: String,
    /// Canonical object identity ref the path resolves to.
    pub canonical_target_ref: String,
    /// Target kind token.
    pub target_kind_token: String,
    /// Resulting mode token after admission or recovery routing.
    pub resulting_mode_token: String,
    /// Target availability class.
    pub availability: TargetAvailabilityClass,
    /// Target freshness token.
    pub freshness_class_token: String,
    /// Source-locator / deep-link intent ref, when the path carried one.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub deep_link_intent_ref: Option<String>,
    /// Whether the literal requested target is preserved.
    pub preserves_literal_target: bool,
    /// Whether the typed source-locator / deep-link intent is preserved.
    pub preserves_typed_intent: bool,
    /// Whether the resulting mode is preserved.
    pub preserves_resulting_mode: bool,
    /// Whether the path avoids reopening a generic shell or the wrong install.
    pub no_generic_shell_reopen: bool,
}

impl TypedTargetIntent {
    /// Returns `true` when the typed target intent is preserved end to end.
    pub fn intent_preserved(&self) -> bool {
        self.preserves_literal_target
            && self.preserves_typed_intent
            && self.preserves_resulting_mode
            && self.no_generic_shell_reopen
            && !self.canonical_target_ref.trim().is_empty()
    }
}

/// Handler-ownership disclosure for side-by-side channels.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HandlerOwnershipDisclosure {
    /// Canonical ref to the owning channel.
    pub owning_channel_ref: String,
    /// Canonical ref to the owning build.
    pub owner_build_ref: String,
    /// Which channel owns the handoff surface.
    pub owning_channel_class: ChannelClass,
    /// Handler-ownership token from the live handoff vocabulary.
    pub handler_ownership_token: String,
    /// Ownership-change review state token.
    pub ownership_review_state_token: String,
    /// Side-by-side channels disclosed for this surface.
    pub side_by_side_channels: Vec<ChannelClass>,
    /// Whether the owning channel and build are stated explicitly.
    pub ownership_explicit: bool,
    /// Whether ownership avoids last-writer-wins between side-by-side installs.
    pub no_last_writer_wins: bool,
    /// Whether handler spoofing fails closed.
    pub spoof_resistant: bool,
}

impl HandlerOwnershipDisclosure {
    /// Returns `true` when handler ownership is explicit, spoof-resistant, and
    /// enumerates every side-by-side channel.
    pub fn ownership_is_explicit(&self) -> bool {
        let channels: BTreeSet<ChannelClass> = self.side_by_side_channels.iter().copied().collect();
        self.ownership_explicit
            && self.no_last_writer_wins
            && self.spoof_resistant
            && ChannelClass::REQUIRED
                .iter()
                .all(|channel| channels.contains(channel))
            && !self.owning_channel_ref.trim().is_empty()
            && !self.owner_build_ref.trim().is_empty()
    }
}

/// System-browser default conformance posture for claimed-identity / auth rows.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AuthDefaultPosture {
    /// Whether this entry path carries a claimed-identity / auth handoff.
    pub applies: bool,
    /// Whether the row defaults to system-browser handoff.
    pub default_to_system_browser: bool,
    /// Whether the live row reports a system-browser default.
    pub system_browser_default: bool,
    /// Exception class naming why a non-system-browser default was chosen.
    pub exception_class: SystemBrowserPolicyExceptionClass,
    /// Target scope ref when an exception applies.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub exception_scope_ref: Option<String>,
    /// Return-path ref the auth callback returns through.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub return_path_ref: Option<String>,
    /// Return-mode token (e.g. system browser, device code, manual resume).
    pub return_mode_token: String,
    /// Recovery-behaviour ref when an exception applies.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub recovery_on_exception_ref: Option<String>,
    /// Whether an embedded web view captured the auth return (must be false
    /// unless an explicit exception is disclosed).
    pub embedded_browser_used: bool,
}

impl AuthDefaultPosture {
    /// Returns `true` when the row defaults to the system browser, or surfaces
    /// an explicit exception with its scope, return path, and recovery.
    pub fn is_system_browser_default_or_explicit_exception(&self) -> bool {
        if !self.applies {
            return true;
        }
        if self.default_to_system_browser
            && self.system_browser_default
            && self.exception_class.is_default_no_exception()
            && !self.embedded_browser_used
        {
            return true;
        }
        self.exception_class.is_explicit_exception()
            && self.exception_scope_ref.is_some()
            && self.return_path_ref.is_some()
            && self.recovery_on_exception_ref.is_some()
    }
}

/// Trust / profile / tenant review posture.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TrustReviewPosture {
    /// Trust-state token at handoff review time.
    pub trust_state_token: String,
    /// Profile / tenant / workspace scope ref.
    pub profile_or_tenant_scope_ref: String,
    /// Policy epoch ref used for admission.
    pub policy_epoch_ref: String,
    /// Requested authority scope token.
    pub requested_authority_scope_token: String,
    /// Granted authority scope token.
    pub granted_authority_scope_token: String,
    /// Whether review is required before any widened authority.
    pub review_required_before_widening: bool,
    /// Whether trust / profile / tenant are checked before risky execution.
    pub trust_profile_tenant_checked: bool,
    /// Whether authority is never widened silently.
    pub no_silent_authority_widening: bool,
}

impl TrustReviewPosture {
    /// Returns `true` when trust / profile / tenant review is enforced ahead of
    /// any widened authority.
    pub fn enforced(&self) -> bool {
        self.trust_profile_tenant_checked
            && self.no_silent_authority_widening
            && !self.profile_or_tenant_scope_ref.trim().is_empty()
            && !self.policy_epoch_ref.trim().is_empty()
    }
}

/// Truthful recovery posture for moved / removable / network / missing targets.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TargetRecoveryPosture {
    /// Target availability class.
    pub availability: TargetAvailabilityClass,
    /// Whether a truthful placeholder is rendered.
    pub placeholder_required: bool,
    /// Last-seen identity ref retained for a moved / missing target.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_seen_identity_ref: Option<String>,
    /// Unsaved-local-state posture token.
    pub unsaved_local_state_posture_token: String,
    /// Whether local work is preserved where possible.
    pub local_work_preserved: bool,
    /// Recovery actions rendered for the posture, in order.
    pub recovery_actions: Vec<HandoffRecoveryAction>,
    /// Whether no mutating work or stale authority can replay silently.
    pub no_silent_replay_or_authority_reuse: bool,
    /// Canonical target path label for open/save/reveal flows, when surfaced.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub canonical_target_path_label: Option<String>,
    /// Write posture token (overwrite / read-only / blocked) when the target is
    /// not the local default.
    pub write_posture_token: String,
    /// Profile / remote boundary note when the target is not the local default.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub profile_remote_boundary_note: Option<String>,
}

impl TargetRecoveryPosture {
    /// Returns `true` when recovery is truthful: no silent replay, and a target
    /// that needs a placeholder renders one with its identity and the explicit
    /// Locate / Open cached context / Close placeholder actions.
    pub fn recovery_truthful(&self) -> bool {
        if !self.no_silent_replay_or_authority_reuse {
            return false;
        }
        if self.availability.requires_recovery() && self.recovery_actions.is_empty() {
            return false;
        }
        if self.availability.requires_placeholder() {
            let has = |action: HandoffRecoveryAction| self.recovery_actions.contains(&action);
            return self.placeholder_required
                && self.last_seen_identity_ref.is_some()
                && has(HandoffRecoveryAction::LocateTarget)
                && has(HandoffRecoveryAction::OpenCachedContext)
                && has(HandoffRecoveryAction::ClosePlaceholder);
        }
        true
    }
}

/// One per-OS conformance row backing the posture.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PlatformConformanceRow {
    /// Per-OS profile.
    pub profile: PlatformProfileClass,
    /// Live platform-profile id from the contract packet.
    pub profile_id: String,
    /// Whether the profile is covered with current proof.
    pub covered: bool,
    /// Source proof ref.
    pub proof_ref: String,
    /// Drill-class tokens the profile exercised.
    pub drill_class_tokens: Vec<String>,
}

/// Input form of one binding surface projection.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HandoffSurfaceProjectionInput {
    /// The binding surface.
    pub surface: HandoffTruthSurface,
    /// The surface's own lifecycle marker.
    pub surface_marker: LifecycleMarker,
    /// Whether the surface reads the shared record rather than cloning prose.
    pub reads_shared_record: bool,
}

/// Output form of one binding surface projection, with a derived summary line.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HandoffSurfaceProjection {
    /// The binding surface.
    pub surface: HandoffTruthSurface,
    /// The surface's own lifecycle marker.
    pub surface_marker: LifecycleMarker,
    /// Whether the surface reads the shared record rather than cloning prose.
    pub reads_shared_record: bool,
    /// Derived, deterministic summary line the surface renders.
    pub summary_line: String,
}

/// The proven pillars of one handoff posture.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct HandoffPillars {
    /// Whether the typed target intent is preserved end to end.
    pub typed_intent_preserved: bool,
    /// Whether handler ownership is explicit and spoof-resistant.
    pub handler_ownership_explicit: bool,
    /// Whether auth defaults to the system browser or discloses an exception.
    pub system_browser_default_or_explicit_exception: bool,
    /// Whether trust / profile / tenant review precedes widened authority.
    pub trust_review_enforced: bool,
    /// Whether recovery is truthful for moved / missing targets.
    pub recovery_truthful: bool,
    /// Whether per-OS conformance is complete.
    pub platform_conformance_complete: bool,
}

/// The public claim ceiling: what a posture is allowed to assert. Each field
/// must be provable from the posture's real evidence; the builder enforces it.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct HandoffClaimCeiling {
    /// Whether the posture may claim typed intent is preserved.
    pub asserts_typed_intent_preserved: bool,
    /// Whether the posture may claim handler ownership is explicit.
    pub asserts_handler_ownership_explicit: bool,
    /// Whether the posture may claim system-browser-default conformance.
    pub asserts_system_browser_default: bool,
    /// Whether the posture may claim trust review is enforced.
    pub asserts_trust_review_enforced: bool,
    /// Whether the posture may claim recovery is truthful.
    pub asserts_recovery_truthful: bool,
    /// Whether the posture may claim per-OS conformance is complete.
    pub asserts_platform_conformance_complete: bool,
}

/// The derived stable-claim verdict for a posture.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HandoffQualification {
    /// The derived claim class (Stable when fully qualified, else narrowed).
    pub claim_class: StableClaimClass,
    /// Whether the posture qualifies at or above the launch cutline.
    pub qualifies_stable: bool,
    /// The reasons the posture is narrowed below Stable, in canonical order.
    pub narrowing_reasons: Vec<HandoffNarrowingReason>,
}

/// Upstream ids the record is a genuine projection of, kept for support
/// traceability. These are upstream source refs, not canonical durable objects.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HandoffUpstream {
    /// Native desktop contract packet id this record projects from.
    pub native_contract_packet_ref: String,
    /// System-browser return-paths page contract ref this record projects from.
    pub system_browser_page_ref: String,
    /// Ownership-audit ref this record projects from.
    pub ownership_audit_ref: String,
    /// Contributing event / row ids from the contract packet.
    pub contributing_event_refs: Vec<String>,
}

/// Validated input used to mint a [`DesktopHandoffConformanceRecord`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DesktopHandoffConformanceInput {
    /// Stable record id.
    pub record_id: String,
    /// UTC timestamp.
    pub as_of: String,
    /// Stable posture id.
    pub posture_id: String,
    /// Compact posture label.
    pub posture_label: String,
    /// Reviewer-facing title.
    pub title: String,
    /// Reviewer-facing summary.
    pub summary: String,
    /// The entry path this posture covers.
    pub entry_path: EntryPathClass,
    /// The typed target intent.
    pub intent: TypedTargetIntent,
    /// The handler-ownership disclosure.
    pub handler_ownership: HandlerOwnershipDisclosure,
    /// The system-browser default conformance posture.
    pub auth_default: AuthDefaultPosture,
    /// The trust / profile / tenant review posture.
    pub trust_review: TrustReviewPosture,
    /// The truthful recovery posture.
    pub recovery: TargetRecoveryPosture,
    /// The per-OS conformance rows.
    pub platform_conformance: Vec<PlatformConformanceRow>,
    /// The binding surface projections.
    pub surface_projections: Vec<HandoffSurfaceProjectionInput>,
    /// Public claim ceiling.
    pub claim_ceiling: HandoffClaimCeiling,
    /// Recovery routes in rendered order.
    pub recovery_routes: Vec<RecoveryRouteRecord>,
    /// Per-surface entry routes to the same posture.
    pub routes: Vec<EntryRouteRecord>,
    /// Accessibility disclosure across required layout modes.
    pub accessibility: AccessibilityDisclosure,
    /// Whether the posture stays available without an account.
    pub available_without_account: bool,
    /// Whether the posture stays available without managed services.
    pub available_without_managed_services: bool,
    /// Upstream ids the record projects from.
    pub upstream: HandoffUpstream,
    /// Canonical diagnostics-export ref.
    pub diagnostics_export_ref: String,
    /// Canonical support-export ref.
    pub support_export_ref: String,
    /// Canonical evidence refs.
    pub evidence_refs: Vec<String>,
    /// Canonical narrative refs.
    pub narrative_refs: Vec<String>,
}

/// The canonical, governed desktop handoff-conformance record.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DesktopHandoffConformanceRecord {
    /// Record discriminator.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Reviewer-facing notice.
    pub notice: String,
    /// Shared contract ref.
    pub shared_contract_ref: String,
    /// Stable record id.
    pub record_id: String,
    /// UTC timestamp.
    pub as_of: String,
    /// Stable posture id.
    pub posture_id: String,
    /// Compact posture label.
    pub posture_label: String,
    /// Reviewer-facing title.
    pub title: String,
    /// Reviewer-facing summary.
    pub summary: String,
    /// The entry path this posture covers.
    pub entry_path: EntryPathClass,
    /// The lowest binding-surface lifecycle marker.
    pub surface_lifecycle_marker: LifecycleMarker,
    /// The typed target intent.
    pub intent: TypedTargetIntent,
    /// The handler-ownership disclosure.
    pub handler_ownership: HandlerOwnershipDisclosure,
    /// The system-browser default conformance posture.
    pub auth_default: AuthDefaultPosture,
    /// The trust / profile / tenant review posture.
    pub trust_review: TrustReviewPosture,
    /// The truthful recovery posture.
    pub recovery: TargetRecoveryPosture,
    /// The per-OS conformance rows, in canonical order.
    pub platform_conformance: Vec<PlatformConformanceRow>,
    /// The binding surface projections, in canonical order.
    pub surface_projections: Vec<HandoffSurfaceProjection>,
    /// The proven pillars.
    pub pillars: HandoffPillars,
    /// Public claim ceiling.
    pub claim_ceiling: HandoffClaimCeiling,
    /// The derived stable-claim verdict.
    pub stable_qualification: HandoffQualification,
    /// Recovery routes in rendered order.
    pub recovery_routes: Vec<RecoveryRouteRecord>,
    /// Per-surface entry routes to the same posture.
    pub routes: Vec<EntryRouteRecord>,
    /// Accessibility disclosure across required layout modes.
    pub accessibility: AccessibilityDisclosure,
    /// Whether the posture stays available without an account.
    pub available_without_account: bool,
    /// Whether the posture stays available without managed services.
    pub available_without_managed_services: bool,
    /// True when there is anything narrowed or below-stable to disclose.
    pub honesty_marker_present: bool,
    /// Upstream ids the record projects from.
    pub upstream: HandoffUpstream,
    /// Canonical diagnostics-export ref.
    pub diagnostics_export_ref: String,
    /// Canonical support-export ref.
    pub support_export_ref: String,
    /// Canonical evidence refs.
    pub evidence_refs: Vec<String>,
    /// Canonical narrative refs.
    pub narrative_refs: Vec<String>,
}

/// Reasons a [`DesktopHandoffConformanceRecord`] cannot honestly be minted.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BuildError {
    /// A field that must be a reviewable sentence was empty or too long.
    InvalidSentence {
        /// The offending field.
        field: &'static str,
    },
    /// A field that must be a canonical object ref was not.
    NonCanonicalRef {
        /// The offending field.
        field: &'static str,
        /// The value that failed.
        value: String,
    },
    /// An upstream projection ref was missing.
    MissingUpstreamRef {
        /// The offending field.
        field: &'static str,
    },
    /// A side-by-side channel was missing from the ownership disclosure.
    MissingSideBySideChannel {
        /// The missing channel.
        channel: ChannelClass,
    },
    /// A per-OS conformance profile was missing.
    MissingPlatformProfile {
        /// The missing profile.
        profile: PlatformProfileClass,
    },
    /// A per-OS conformance profile lacked current proof.
    PlatformProofMissing {
        /// The profile that failed.
        profile: PlatformProfileClass,
    },
    /// The claim ceiling asserted preserved typed intent it cannot prove.
    OverclaimsTypedIntent,
    /// The claim ceiling asserted explicit ownership it cannot prove.
    OverclaimsHandlerOwnership,
    /// The claim ceiling asserted system-browser-default conformance it cannot
    /// prove.
    OverclaimsSystemBrowserDefault,
    /// The claim ceiling asserted enforced trust review it cannot prove.
    OverclaimsTrustReview,
    /// The claim ceiling asserted truthful recovery it cannot prove.
    OverclaimsRecovery,
    /// The claim ceiling asserted complete per-OS conformance it cannot prove.
    OverclaimsPlatformConformance,
    /// A required recovery route was missing.
    MissingRecoveryRoute {
        /// The missing action.
        action: HandoffRecoveryAction,
    },
    /// A recovery route was not keyboard reachable.
    RecoveryRouteNotKeyboardReachable {
        /// The offending action id.
        action_id: String,
    },
    /// A required binding surface projection was missing.
    SurfaceProjectionMissing {
        /// The missing surface.
        surface: HandoffTruthSurface,
    },
    /// A binding surface cloned prose instead of reading the shared record.
    SurfaceClonesProse {
        /// The offending surface.
        surface: HandoffTruthSurface,
    },
    /// A binding surface projection was duplicated.
    DuplicateSurfaceProjection {
        /// The duplicated surface.
        surface: HandoffTruthSurface,
    },
    /// A required entry-route surface was missing.
    RouteSurfaceMissing {
        /// The missing surface.
        surface: AttentionRouteSurface,
    },
    /// An entry route was not keyboard reachable.
    RouteNotKeyboardReachable {
        /// The offending surface.
        surface: AttentionRouteSurface,
    },
    /// An entry route did not activate the same posture.
    RouteTargetsDifferentItem {
        /// The offending surface.
        surface: AttentionRouteSurface,
    },
    /// An entry-route surface was duplicated.
    DuplicateRouteSurface {
        /// The duplicated surface.
        surface: AttentionRouteSurface,
    },
    /// The accessibility action labels did not match the recovery routes.
    AccessibilityActionLabelsMismatch,
    /// A required accessibility layout mode was missing.
    AccessibilityLayoutModeMissing {
        /// The missing mode.
        mode: LayoutMode,
    },
    /// An accessibility layout mode was unreachable or lost narration.
    AccessibilityLayoutModeUnreachable {
        /// The offending mode.
        mode: LayoutMode,
    },
    /// A posture was hidden when no account was present.
    HiddenWithoutAccount,
    /// A posture was hidden when managed services were absent.
    HiddenWithoutManagedServices,
}

impl core::fmt::Display for BuildError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::InvalidSentence { field } => {
                write!(f, "field `{field}` must be a non-empty reviewable sentence")
            }
            Self::NonCanonicalRef { field, value } => {
                write!(
                    f,
                    "field `{field}` must be a canonical object ref, got {value:?}"
                )
            }
            Self::MissingUpstreamRef { field } => {
                write!(f, "upstream projection ref `{field}` must be present")
            }
            Self::MissingSideBySideChannel { channel } => write!(
                f,
                "handler ownership must enumerate side-by-side channel `{}`",
                channel.as_str()
            ),
            Self::MissingPlatformProfile { profile } => {
                write!(
                    f,
                    "per-OS conformance is missing profile `{}`",
                    profile.as_str()
                )
            }
            Self::PlatformProofMissing { profile } => write!(
                f,
                "per-OS conformance profile `{}` lacks current proof",
                profile.as_str()
            ),
            Self::OverclaimsTypedIntent => write!(
                f,
                "claim ceiling may not assert preserved typed intent it cannot prove"
            ),
            Self::OverclaimsHandlerOwnership => write!(
                f,
                "claim ceiling may not assert explicit handler ownership it cannot prove"
            ),
            Self::OverclaimsSystemBrowserDefault => write!(
                f,
                "claim ceiling may not assert system-browser-default conformance it cannot prove"
            ),
            Self::OverclaimsTrustReview => write!(
                f,
                "claim ceiling may not assert enforced trust review it cannot prove"
            ),
            Self::OverclaimsRecovery => {
                write!(
                    f,
                    "claim ceiling may not assert truthful recovery it cannot prove"
                )
            }
            Self::OverclaimsPlatformConformance => write!(
                f,
                "claim ceiling may not assert complete per-OS conformance it cannot prove"
            ),
            Self::MissingRecoveryRoute { action } => {
                write!(
                    f,
                    "posture must expose recovery route `{}`",
                    action.as_str()
                )
            }
            Self::RecoveryRouteNotKeyboardReachable { action_id } => {
                write!(f, "recovery route `{action_id}` must be keyboard reachable")
            }
            Self::SurfaceProjectionMissing { surface } => {
                write!(f, "binding surface `{}` is missing", surface.as_str())
            }
            Self::SurfaceClonesProse { surface } => write!(
                f,
                "binding surface `{}` must read the shared record",
                surface.as_str()
            ),
            Self::DuplicateSurfaceProjection { surface } => {
                write!(f, "binding surface `{}` is duplicated", surface.as_str())
            }
            Self::RouteSurfaceMissing { surface } => {
                write!(f, "entry route surface `{}` is missing", surface.as_str())
            }
            Self::RouteNotKeyboardReachable { surface } => write!(
                f,
                "entry route surface `{}` must be keyboard reachable",
                surface.as_str()
            ),
            Self::RouteTargetsDifferentItem { surface } => write!(
                f,
                "entry route surface `{}` must activate the same posture",
                surface.as_str()
            ),
            Self::DuplicateRouteSurface { surface } => {
                write!(
                    f,
                    "entry route surface `{}` is duplicated",
                    surface.as_str()
                )
            }
            Self::AccessibilityActionLabelsMismatch => write!(
                f,
                "accessibility action labels must match the recovery routes in order"
            ),
            Self::AccessibilityLayoutModeMissing { mode } => {
                write!(
                    f,
                    "accessibility layout mode `{}` is missing",
                    mode.as_str()
                )
            }
            Self::AccessibilityLayoutModeUnreachable { mode } => write!(
                f,
                "accessibility layout mode `{}` must keep narration and reachable affordances",
                mode.as_str()
            ),
            Self::HiddenWithoutAccount => {
                write!(
                    f,
                    "a handoff posture must stay available without an account"
                )
            }
            Self::HiddenWithoutManagedServices => write!(
                f,
                "a handoff posture must stay available without managed services"
            ),
        }
    }
}

impl std::error::Error for BuildError {}

impl DesktopHandoffConformanceRecord {
    /// Builds a governed handoff-conformance record from validated input.
    ///
    /// Returns a [`BuildError`] when the input would mint a record that lies
    /// about typed-intent preservation, handler ownership, system-browser
    /// default conformance, trust review, truthful recovery, per-OS coverage,
    /// recovery routes, binding surfaces, route reachability, or accessibility.
    /// The stable claim class is *derived* from the evidence, so a posture can
    /// never publish a claim wider than its proof.
    pub fn build(input: DesktopHandoffConformanceInput) -> Result<Self, BuildError> {
        // --- text / ref validation -------------------------------------------
        if !is_reviewable_sentence(&input.title) {
            return Err(BuildError::InvalidSentence { field: "title" });
        }
        if !is_reviewable_sentence(&input.summary) {
            return Err(BuildError::InvalidSentence { field: "summary" });
        }
        if !is_reviewable_sentence(&input.posture_label) {
            return Err(BuildError::InvalidSentence {
                field: "posture_label",
            });
        }
        require_canonical_ref(
            "intent.canonical_target_ref",
            &input.intent.canonical_target_ref,
        )?;
        require_canonical_ref(
            "handler_ownership.owning_channel_ref",
            &input.handler_ownership.owning_channel_ref,
        )?;
        require_canonical_ref("diagnostics_export_ref", &input.diagnostics_export_ref)?;
        require_canonical_ref("support_export_ref", &input.support_export_ref)?;
        for evidence in &input.evidence_refs {
            require_canonical_ref("evidence_refs", evidence)?;
        }
        for narrative in &input.narrative_refs {
            require_canonical_ref("narrative_refs", narrative)?;
        }

        // Upstream and literal projection refs keep their source scheme; only
        // presence is required so support can trace the record back.
        require_present_ref(
            "intent.literal_target_ref",
            &input.intent.literal_target_ref,
        )?;
        require_present_ref(
            "handler_ownership.owner_build_ref",
            &input.handler_ownership.owner_build_ref,
        )?;
        require_present_ref(
            "trust_review.policy_epoch_ref",
            &input.trust_review.policy_epoch_ref,
        )?;
        require_present_ref(
            "upstream.native_contract_packet_ref",
            &input.upstream.native_contract_packet_ref,
        )?;
        require_present_ref(
            "upstream.system_browser_page_ref",
            &input.upstream.system_browser_page_ref,
        )?;
        require_present_ref(
            "upstream.ownership_audit_ref",
            &input.upstream.ownership_audit_ref,
        )?;

        // --- handler ownership: every side-by-side channel enumerated --------
        let channels: BTreeSet<ChannelClass> = input
            .handler_ownership
            .side_by_side_channels
            .iter()
            .copied()
            .collect();
        for required in ChannelClass::REQUIRED {
            if !channels.contains(&required) {
                return Err(BuildError::MissingSideBySideChannel { channel: required });
            }
        }

        // --- per-OS conformance: every profile present with current proof ----
        for required in PlatformProfileClass::REQUIRED {
            let row = input
                .platform_conformance
                .iter()
                .find(|row| row.profile == required)
                .ok_or(BuildError::MissingPlatformProfile { profile: required })?;
            if !row.covered || row.proof_ref.trim().is_empty() {
                return Err(BuildError::PlatformProofMissing { profile: required });
            }
        }
        let platform_conformance_complete = PlatformProfileClass::REQUIRED.iter().all(|profile| {
            input.platform_conformance.iter().any(|row| {
                row.profile == *profile && row.covered && !row.proof_ref.trim().is_empty()
            })
        });

        // --- derive the pillars from the evidence -----------------------------
        let typed_intent_preserved = input.intent.intent_preserved();
        let handler_ownership_explicit = input.handler_ownership.ownership_is_explicit();
        let system_browser_default_or_explicit_exception = input
            .auth_default
            .is_system_browser_default_or_explicit_exception();
        let trust_review_enforced = input.trust_review.enforced();
        let recovery_truthful = input.recovery.recovery_truthful();

        // --- claim ceiling: never claim what the product cannot prove ---------
        if input.claim_ceiling.asserts_typed_intent_preserved && !typed_intent_preserved {
            return Err(BuildError::OverclaimsTypedIntent);
        }
        if input.claim_ceiling.asserts_handler_ownership_explicit && !handler_ownership_explicit {
            return Err(BuildError::OverclaimsHandlerOwnership);
        }
        if input.claim_ceiling.asserts_system_browser_default
            && !system_browser_default_or_explicit_exception
        {
            return Err(BuildError::OverclaimsSystemBrowserDefault);
        }
        if input.claim_ceiling.asserts_trust_review_enforced && !trust_review_enforced {
            return Err(BuildError::OverclaimsTrustReview);
        }
        if input.claim_ceiling.asserts_recovery_truthful && !recovery_truthful {
            return Err(BuildError::OverclaimsRecovery);
        }
        if input.claim_ceiling.asserts_platform_conformance_complete
            && !platform_conformance_complete
        {
            return Err(BuildError::OverclaimsPlatformConformance);
        }

        // --- recovery routes -------------------------------------------------
        let route_ids: Vec<&str> = input
            .recovery_routes
            .iter()
            .map(|route| route.action_id.as_str())
            .collect();
        for required in HandoffRecoveryAction::REQUIRED {
            if !route_ids.iter().any(|id| *id == required.as_str()) {
                return Err(BuildError::MissingRecoveryRoute { action: required });
            }
        }
        for route in &input.recovery_routes {
            if !route.keyboard_reachable {
                return Err(BuildError::RecoveryRouteNotKeyboardReachable {
                    action_id: route.action_id.clone(),
                });
            }
        }

        // --- surface projections ---------------------------------------------
        let mut seen_surfaces: BTreeSet<HandoffTruthSurface> = BTreeSet::new();
        for projection in &input.surface_projections {
            if !seen_surfaces.insert(projection.surface) {
                return Err(BuildError::DuplicateSurfaceProjection {
                    surface: projection.surface,
                });
            }
            if !projection.reads_shared_record {
                return Err(BuildError::SurfaceClonesProse {
                    surface: projection.surface,
                });
            }
        }
        for required in HandoffTruthSurface::REQUIRED {
            if !seen_surfaces.contains(&required) {
                return Err(BuildError::SurfaceProjectionMissing { surface: required });
            }
        }
        let mut surface_projections: Vec<HandoffSurfaceProjection> = Vec::new();
        for required in HandoffTruthSurface::REQUIRED {
            let projection = input
                .surface_projections
                .iter()
                .find(|p| p.surface == required)
                .expect("surface presence checked above");
            surface_projections.push(HandoffSurfaceProjection {
                surface: required,
                surface_marker: projection.surface_marker,
                reads_shared_record: projection.reads_shared_record,
                summary_line: surface_summary_line(required, &input),
            });
        }
        let surface_lifecycle_marker = surface_projections
            .iter()
            .map(|projection| projection.surface_marker)
            .min()
            .unwrap_or(LifecycleMarker::Stable);

        // --- entry routes ----------------------------------------------------
        let mut seen_route_surfaces: Vec<AttentionRouteSurface> = Vec::new();
        for route in &input.routes {
            if seen_route_surfaces.contains(&route.surface) {
                return Err(BuildError::DuplicateRouteSurface {
                    surface: route.surface,
                });
            }
            seen_route_surfaces.push(route.surface);
            require_canonical_ref("routes.route_ref", &route.route_ref)?;
            if !route.keyboard_reachable {
                return Err(BuildError::RouteNotKeyboardReachable {
                    surface: route.surface,
                });
            }
            if !route.activates_same_item {
                return Err(BuildError::RouteTargetsDifferentItem {
                    surface: route.surface,
                });
            }
        }
        for required in AttentionRouteSurface::REQUIRED {
            if !seen_route_surfaces.contains(&required) {
                return Err(BuildError::RouteSurfaceMissing { surface: required });
            }
        }

        // --- accessibility ---------------------------------------------------
        if input.accessibility.action_labels.len() != input.recovery_routes.len() {
            return Err(BuildError::AccessibilityActionLabelsMismatch);
        }
        for (label, route) in input
            .accessibility
            .action_labels
            .iter()
            .zip(input.recovery_routes.iter())
        {
            if label != &route.action_label {
                return Err(BuildError::AccessibilityActionLabelsMismatch);
            }
        }
        for required in LayoutMode::REQUIRED {
            let Some(disclosure) = input
                .accessibility
                .layout_modes
                .iter()
                .find(|mode| mode.mode == required)
            else {
                return Err(BuildError::AccessibilityLayoutModeMissing { mode: required });
            };
            if !disclosure.row_narration_available || !disclosure.recovery_affordances_reachable {
                return Err(BuildError::AccessibilityLayoutModeUnreachable { mode: required });
            }
        }

        // --- availability ----------------------------------------------------
        if !input.available_without_account {
            return Err(BuildError::HiddenWithoutAccount);
        }
        if !input.available_without_managed_services {
            return Err(BuildError::HiddenWithoutManagedServices);
        }

        // --- pillars ---------------------------------------------------------
        let pillars = HandoffPillars {
            typed_intent_preserved,
            handler_ownership_explicit,
            system_browser_default_or_explicit_exception,
            trust_review_enforced,
            recovery_truthful,
            platform_conformance_complete,
        };

        // --- normalise per-OS conformance + upstream refs --------------------
        let mut platform_conformance = input.platform_conformance;
        platform_conformance.sort_by_key(|row| row.profile);
        let mut contributing_event_refs = input.upstream.contributing_event_refs.clone();
        contributing_event_refs.sort();
        contributing_event_refs.dedup();

        // --- derive the stable-claim verdict ---------------------------------
        let mut narrowing_reasons = Vec::new();
        if !typed_intent_preserved {
            narrowing_reasons.push(HandoffNarrowingReason::TypedIntentNotPreserved);
        }
        if !handler_ownership_explicit {
            narrowing_reasons.push(HandoffNarrowingReason::HandlerOwnershipNotExplicit);
        }
        if !system_browser_default_or_explicit_exception {
            narrowing_reasons.push(HandoffNarrowingReason::AuthNotSystemBrowserDefault);
        }
        if !trust_review_enforced {
            narrowing_reasons.push(HandoffNarrowingReason::TrustReviewNotEnforced);
        }
        if !recovery_truthful {
            narrowing_reasons.push(HandoffNarrowingReason::RecoveryNotTruthful);
        }
        if !platform_conformance_complete {
            narrowing_reasons.push(HandoffNarrowingReason::PlatformConformanceIncomplete);
        }
        if surface_lifecycle_marker.is_below_stable() {
            narrowing_reasons.push(HandoffNarrowingReason::SurfaceNotYetStable);
        }
        let qualifies_stable = narrowing_reasons.is_empty();
        let claim_class = if qualifies_stable {
            StableClaimClass::Stable
        } else if narrowing_reasons.len() == 1
            && narrowing_reasons[0] == HandoffNarrowingReason::SurfaceNotYetStable
        {
            match surface_lifecycle_marker {
                LifecycleMarker::Preview => StableClaimClass::Preview,
                _ => StableClaimClass::Beta,
            }
        } else {
            StableClaimClass::Beta
        };
        let stable_qualification = HandoffQualification {
            claim_class,
            qualifies_stable,
            narrowing_reasons,
        };
        let honesty_marker_present = !qualifies_stable
            || surface_lifecycle_marker.is_below_stable()
            || input.recovery.placeholder_required
            || (input.auth_default.applies
                && !input.auth_default.exception_class.is_default_no_exception());

        Ok(Self {
            record_kind: DESKTOP_HANDOFF_CONFORMANCE_RECORD_KIND.to_string(),
            schema_version: DESKTOP_HANDOFF_CONFORMANCE_SCHEMA_VERSION,
            notice: DESKTOP_HANDOFF_CONFORMANCE_NOTICE.to_string(),
            shared_contract_ref: DESKTOP_HANDOFF_CONFORMANCE_SHARED_CONTRACT_REF.to_string(),
            record_id: input.record_id,
            as_of: input.as_of,
            posture_id: input.posture_id,
            posture_label: input.posture_label,
            title: input.title,
            summary: input.summary,
            entry_path: input.entry_path,
            surface_lifecycle_marker,
            intent: input.intent,
            handler_ownership: input.handler_ownership,
            auth_default: input.auth_default,
            trust_review: input.trust_review,
            recovery: input.recovery,
            platform_conformance,
            surface_projections,
            pillars,
            claim_ceiling: input.claim_ceiling,
            stable_qualification,
            recovery_routes: input.recovery_routes,
            routes: input.routes,
            accessibility: input.accessibility,
            available_without_account: input.available_without_account,
            available_without_managed_services: input.available_without_managed_services,
            honesty_marker_present,
            upstream: HandoffUpstream {
                native_contract_packet_ref: input.upstream.native_contract_packet_ref,
                system_browser_page_ref: input.upstream.system_browser_page_ref,
                ownership_audit_ref: input.upstream.ownership_audit_ref,
                contributing_event_refs,
            },
            diagnostics_export_ref: input.diagnostics_export_ref,
            support_export_ref: input.support_export_ref,
            evidence_refs: input.evidence_refs,
            narrative_refs: input.narrative_refs,
        })
    }

    /// Returns a deterministic plaintext truth block for support exports.
    pub fn support_export_lines(&self) -> Vec<String> {
        let mut lines = vec![
            format!("desktop_handoff_conformance: {}", self.record_id),
            format!("as_of: {}", self.as_of),
            format!("posture: {} ({})", self.posture_id, self.posture_label),
            format!("entry_path: {}", self.entry_path.as_str()),
            format!(
                "surface_lifecycle_marker: {}",
                self.surface_lifecycle_marker.as_str()
            ),
            format!("title: {}", self.title),
            format!("summary: {}", self.summary),
            format!(
                "stable_qualification: class={} qualifies_stable={} narrowing=[{}]",
                self.stable_qualification.claim_class.as_str(),
                self.stable_qualification.qualifies_stable,
                self.stable_qualification
                    .narrowing_reasons
                    .iter()
                    .map(|reason| reason.as_str())
                    .collect::<Vec<_>>()
                    .join(", ")
            ),
            format!(
                "pillars: typed_intent={} handler_ownership={} system_browser_default={} trust_review={} recovery_truthful={} platform_conformance={}",
                self.pillars.typed_intent_preserved,
                self.pillars.handler_ownership_explicit,
                self.pillars.system_browser_default_or_explicit_exception,
                self.pillars.trust_review_enforced,
                self.pillars.recovery_truthful,
                self.pillars.platform_conformance_complete
            ),
            format!(
                "intent: literal={} canonical={} resulting_mode={} availability={} freshness={}",
                self.intent.literal_target_label,
                self.intent.canonical_target_ref,
                self.intent.resulting_mode_token,
                self.intent.availability.as_str(),
                self.intent.freshness_class_token
            ),
            format!(
                "handler_ownership: owning_channel={} owner_build={} channel_class={} ownership_token={} side_by_side=[{}] no_last_writer_wins={} spoof_resistant={}",
                self.handler_ownership.owning_channel_ref,
                self.handler_ownership.owner_build_ref,
                self.handler_ownership.owning_channel_class.as_str(),
                self.handler_ownership.handler_ownership_token,
                self.handler_ownership
                    .side_by_side_channels
                    .iter()
                    .map(|c| c.as_str())
                    .collect::<Vec<_>>()
                    .join(", "),
                self.handler_ownership.no_last_writer_wins,
                self.handler_ownership.spoof_resistant
            ),
            format!(
                "auth_default: applies={} default_to_system_browser={} system_browser_default={} exception={} return_mode={} embedded_browser_used={}",
                self.auth_default.applies,
                self.auth_default.default_to_system_browser,
                self.auth_default.system_browser_default,
                self.auth_default.exception_class.as_str(),
                self.auth_default.return_mode_token,
                self.auth_default.embedded_browser_used
            ),
            format!(
                "trust_review: trust_state={} scope={} requested_scope={} granted_scope={} no_silent_widening={}",
                self.trust_review.trust_state_token,
                self.trust_review.profile_or_tenant_scope_ref,
                self.trust_review.requested_authority_scope_token,
                self.trust_review.granted_authority_scope_token,
                self.trust_review.no_silent_authority_widening
            ),
            format!(
                "recovery: availability={} placeholder_required={} unsaved_local_state={} write_posture={} no_silent_replay={} actions=[{}]",
                self.recovery.availability.as_str(),
                self.recovery.placeholder_required,
                self.recovery.unsaved_local_state_posture_token,
                self.recovery.write_posture_token,
                self.recovery.no_silent_replay_or_authority_reuse,
                self.recovery
                    .recovery_actions
                    .iter()
                    .map(|a| a.as_str())
                    .collect::<Vec<_>>()
                    .join(", ")
            ),
        ];
        lines.push("platform_conformance:".to_string());
        for row in &self.platform_conformance {
            lines.push(format!(
                "  - {} profile_id={} covered={} drills=[{}] :: {}",
                row.profile.as_str(),
                row.profile_id,
                row.covered,
                row.drill_class_tokens.join(", "),
                row.proof_ref
            ));
        }
        lines.push("surface_projections:".to_string());
        for projection in &self.surface_projections {
            lines.push(format!(
                "  - {} marker={} reads_shared_record={} :: {}",
                projection.surface.as_str(),
                projection.surface_marker.as_str(),
                projection.reads_shared_record,
                projection.summary_line
            ));
        }
        lines.push(format!(
            "availability: without_account={} without_managed_services={}",
            self.available_without_account, self.available_without_managed_services
        ));
        lines.push(format!(
            "honesty_marker_present: {}",
            self.honesty_marker_present
        ));
        lines.push(format!(
            "diagnostics_export_ref: {}",
            self.diagnostics_export_ref
        ));
        lines.push(format!("support_export_ref: {}", self.support_export_ref));
        lines
    }
}

fn surface_summary_line(
    surface: HandoffTruthSurface,
    input: &DesktopHandoffConformanceInput,
) -> String {
    let prefix = match surface {
        HandoffTruthSurface::DesktopHandoffReview => "Handoff review",
        HandoffTruthSurface::CliInspect => "CLI inspect",
        HandoffTruthSurface::HelpAbout => "Help/About",
        HandoffTruthSurface::SupportExport => "Support export",
    };
    format!(
        "{prefix}: {} handoff resolves {} owned by {} ({}).",
        input.entry_path.as_str(),
        input.intent.canonical_target_ref,
        input.handler_ownership.owning_channel_class.as_str(),
        input.intent.resulting_mode_token,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn entry_path_auth_classification_is_correct() {
        assert!(EntryPathClass::DefaultBrowserAuthCallback.is_auth_path());
        assert!(!EntryPathClass::FileAssociation.is_auth_path());
    }

    #[test]
    fn auth_default_accepts_system_browser_or_disclosed_exception() {
        let mut posture = AuthDefaultPosture {
            applies: true,
            default_to_system_browser: true,
            system_browser_default: true,
            exception_class: SystemBrowserPolicyExceptionClass::SystemBrowserDefaultNoException,
            exception_scope_ref: None,
            return_path_ref: None,
            return_mode_token: "open_system_browser".to_string(),
            recovery_on_exception_ref: None,
            embedded_browser_used: false,
        };
        assert!(posture.is_system_browser_default_or_explicit_exception());

        // An embedded browser with no disclosed exception fails.
        posture.embedded_browser_used = true;
        assert!(!posture.is_system_browser_default_or_explicit_exception());

        // A disclosed admin exception with scope/return/recovery passes.
        let disclosed = AuthDefaultPosture {
            applies: true,
            default_to_system_browser: false,
            system_browser_default: false,
            exception_class: SystemBrowserPolicyExceptionClass::AdminPolicyDeviceCodeRequired,
            exception_scope_ref: Some("aureline://auth-scope/tenant".to_string()),
            return_path_ref: Some("aureline://auth-return/device-code".to_string()),
            return_mode_token: "device_code".to_string(),
            recovery_on_exception_ref: Some("aureline://auth-recovery/device-code".to_string()),
            embedded_browser_used: false,
        };
        assert!(disclosed.is_system_browser_default_or_explicit_exception());
    }

    #[test]
    fn required_recovery_routes_expand_with_needs() {
        let base = required_recovery_routes(false, false, false);
        let ids: Vec<&str> = base.iter().map(|r| r.action_id.as_str()).collect();
        for required in HandoffRecoveryAction::REQUIRED {
            assert!(ids.contains(&required.as_str()));
        }
        let placeholder = required_recovery_routes(true, true, true);
        let ids: Vec<String> = placeholder.iter().map(|r| r.action_id.clone()).collect();
        assert!(ids.iter().any(|id| id == "locate_target"));
        assert!(ids.iter().any(|id| id == "open_cached_context"));
        assert!(ids.iter().any(|id| id == "close_placeholder"));
        assert!(ids.iter().any(|id| id == "reauthenticate"));
        assert!(ids.iter().any(|id| id == "reconnect_session"));
    }
}
