//! M5 embedded service-dashboard / webview origin bars, device-permission rows,
//! and browser / device-code auth handoff cards, bound to the frozen
//! operator-surface matrix.
//!
//! The [operator-surface matrix](crate::m5_operator_surfaces) freezes the
//! *families* of operator surface — including the embedded provider/auth boundary
//! state — the one shared state vocabulary, and the invariants every surface must
//! hold. The [maintenance windows](crate::m5_maintenance_windows) and
//! [response panes](crate::m5_response_panes) lanes build planned-operation and
//! guided-response surfaces. This lane builds the first real **embedded boundary**
//! surfaces: the origin bars, device-permission rows, and browser / device-code
//! auth handoff cards that let an operator tell *who owns and renders* an embedded
//! dashboard, *what is native product chrome versus provider / webview chrome*,
//! *what device or browser permissions are in play*, and *when Aureline
//! intentionally hands work to the system browser or a device-code flow*.
//!
//! Each [`EmbeddedSurfaceCard`] pins the truth a generic webview frame never does:
//!
//! 1. **An origin bar.** Every embedded surface names its [`OriginBar`]: the
//!    [`OriginOwnerClass`] (native chrome, Aureline-owned webview, extension, third
//!    party, or unknown), an opaque owner label and origin handle, the optional
//!    extension that provides it, the [`PermissionStateClass`], the explicit
//!    [`CapabilityLimitation`]s, an open-in-browser action, a freshness stamp, and
//!    the required visible language shown verbatim — and it never impersonates a
//!    native approval, update, or product-security surface.
//! 2. **Device-permission rows.** A surface that uses a device capability carries
//!    [`DevicePermissionRow`]s that name the actor, the [`ProcessingClass`]
//!    (local-only, provider-processed, or mixed), the [`RetentionClass`] and a
//!    storage / retention note, a revoke / open-system-settings action, and the
//!    local-continuity posture if the permission is revoked.
//! 3. **Browser / device-code auth handoff.** A surface that hands auth to the
//!    system browser or a device-code flow carries an [`AuthHandoffCard`] that makes
//!    the [`HandoffReasonClass`], the [`HandoffTargetClass`], the fallback state,
//!    the verification-code class and expiry, and the return path visible — never
//!    hidden behind a generic "Continue".
//!
//! [`embedded_surface_set`] is the canonical binding: it builds the cards
//! deterministically and computes each [`EmbeddedInvariant`]'s `holds` flag and the
//! per-card effective state from the built data, so the checked-in fixture and the
//! replay gate freeze the contract byte-for-byte and an inconsistent edit flips an
//! invariant rather than silently passing. The record carries no endpoint URLs,
//! hostnames, credentials, raw payloads, verification-code values, or absolute
//! paths — only opaque object refs, stable tokens, and short reviewable sentences —
//! so it is safe for support export.

use serde::{Deserialize, Serialize};
use time::format_description::well_known::Rfc3339;
use time::OffsetDateTime;

use crate::m5_operator_boards::{compute_effective_state, BlockerWaiverClass, FreshnessClass};
use crate::m5_operator_surfaces::{
    ConsumerClass, LiveSnapshotClass, OperatorStateClass, OperatorSurfaceClass, RedactionClass,
    ScopeClass, TokenDef,
};

#[cfg(test)]
mod tests;

/// Schema version for the embedded-surface set.
pub const M5_EMBEDDED_DASHBOARDS_SCHEMA_VERSION: u32 = 1;

/// Schema reference for the embedded-surface set.
pub const M5_EMBEDDED_DASHBOARDS_SCHEMA_REF: &str =
    "schemas/ops/m5-embedded-dashboards.schema.json";

/// Stable record-kind tag for the embedded-surface set.
pub const M5_EMBEDDED_DASHBOARDS_RECORD_KIND: &str = "m5_embedded_surface_set";

/// Stable id for the canonical embedded-surface set.
pub const M5_EMBEDDED_DASHBOARDS_SET_ID: &str = "m5-embedded-dashboards:set:0001";

/// Evaluation stamp for the canonical set. Held as a constant so the canonical
/// binding stays deterministic and the fixture freezes byte-for-byte.
pub const M5_EMBEDDED_DASHBOARDS_AS_OF: &str = "2026-06-22T00:00:00Z";

/// The operator-surface matrix fixture this set binds for surface identity.
pub const M5_EMBEDDED_DASHBOARDS_MATRIX_REF: &str =
    "fixtures/ops/m5-operator-surfaces/canonical_matrix.json";

/// The matrix record kind this set binds.
pub const M5_EMBEDDED_DASHBOARDS_MATRIX_RECORD_KIND: &str = "m5_operator_surface_matrix";

// ---------------------------------------------------------------------------
// Surface kind.
// ---------------------------------------------------------------------------

/// The kind of embedded surface a card describes.
///
/// The kind selects which sub-blocks a card carries: a dashboard or provider page
/// always carries an [`OriginBar`]; a device-capture surface additionally carries
/// [`DevicePermissionRow`]s; an auth handoff additionally carries an
/// [`AuthHandoffCard`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EmbeddedSurfaceKind {
    /// An embedded service / observability dashboard rendered in a webview.
    ServiceDashboard,
    /// An embedded third-party provider page rendered in a webview.
    ProviderPage,
    /// A surface that uses one or more device capabilities (camera, microphone,
    /// screen, clipboard, …) for capture.
    DeviceCaptureSurface,
    /// An explicit handoff of claimed-identity / provider auth to the system
    /// browser.
    BrowserAuthHandoff,
    /// An explicit device-code auth handoff: a short user code is shown and the
    /// flow polls for completion.
    DeviceCodeAuthHandoff,
}

impl EmbeddedSurfaceKind {
    /// All kinds, in set order.
    pub const ALL: [Self; 5] = [
        Self::ServiceDashboard,
        Self::ProviderPage,
        Self::DeviceCaptureSurface,
        Self::BrowserAuthHandoff,
        Self::DeviceCodeAuthHandoff,
    ];

    /// Stable snake_case token for this kind.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ServiceDashboard => "service_dashboard",
            Self::ProviderPage => "provider_page",
            Self::DeviceCaptureSurface => "device_capture_surface",
            Self::BrowserAuthHandoff => "browser_auth_handoff",
            Self::DeviceCodeAuthHandoff => "device_code_auth_handoff",
        }
    }

    /// Human-readable label.
    pub const fn label(self) -> &'static str {
        match self {
            Self::ServiceDashboard => "Embedded service dashboard",
            Self::ProviderPage => "Embedded provider page",
            Self::DeviceCaptureSurface => "Device-capture surface",
            Self::BrowserAuthHandoff => "Browser auth handoff",
            Self::DeviceCodeAuthHandoff => "Device-code auth handoff",
        }
    }

    /// Whether this kind is an embedded webview (rather than a handoff card).
    pub const fn is_embedded_webview(self) -> bool {
        matches!(
            self,
            Self::ServiceDashboard | Self::ProviderPage | Self::DeviceCaptureSurface
        )
    }

    /// Whether this kind renders a page that should offer an open-in-browser exit.
    ///
    /// A device-capture surface is native chrome that happens to use device
    /// permissions, not a provider page, so it carries no open-in-browser action.
    pub const fn requires_open_in_browser(self) -> bool {
        matches!(self, Self::ServiceDashboard | Self::ProviderPage)
    }

    /// Whether this kind hands auth out to the system browser or a device code.
    pub const fn is_auth_handoff(self) -> bool {
        matches!(self, Self::BrowserAuthHandoff | Self::DeviceCodeAuthHandoff)
    }
}

// ---------------------------------------------------------------------------
// Origin / owner.
// ---------------------------------------------------------------------------

/// Who owns and renders an embedded surface's content.
///
/// This is the heart of the boundary-honesty contract: an operator can tell native
/// product chrome from provider / webview chrome before trusting a dashboard.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum OriginOwnerClass {
    /// Native Aureline product chrome — not embedded content.
    FirstPartyNativeChrome,
    /// Aureline-owned content rendered in a webview.
    FirstPartyWebview,
    /// Content provided by an installed extension.
    ExtensionProvided,
    /// Content owned by a third-party provider / service.
    ThirdPartyProvider,
    /// The origin cannot be determined and requires review before trust.
    UnknownOrigin,
}

impl OriginOwnerClass {
    /// All owner classes, in set order.
    pub const ALL: [Self; 5] = [
        Self::FirstPartyNativeChrome,
        Self::FirstPartyWebview,
        Self::ExtensionProvided,
        Self::ThirdPartyProvider,
        Self::UnknownOrigin,
    ];

    /// Stable snake_case token for this owner class.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::FirstPartyNativeChrome => "first_party_native_chrome",
            Self::FirstPartyWebview => "first_party_webview",
            Self::ExtensionProvided => "extension_provided",
            Self::ThirdPartyProvider => "third_party_provider",
            Self::UnknownOrigin => "unknown_origin",
        }
    }

    /// Human-readable label.
    pub const fn label(self) -> &'static str {
        match self {
            Self::FirstPartyNativeChrome => "Native product chrome",
            Self::FirstPartyWebview => "Aureline webview",
            Self::ExtensionProvided => "Extension-provided",
            Self::ThirdPartyProvider => "Third-party provider",
            Self::UnknownOrigin => "Unknown origin",
        }
    }

    /// Whether this owner class is embedded (non-native) content the operator must
    /// be able to distinguish from native chrome.
    pub const fn is_embedded_content(self) -> bool {
        !matches!(self, Self::FirstPartyNativeChrome)
    }

    /// Whether an unknown / undeterminable origin requires review before trust.
    pub const fn requires_review(self) -> bool {
        matches!(self, Self::UnknownOrigin)
    }
}

/// The permission / capability posture an embedded surface holds.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PermissionStateClass {
    /// Fully sandboxed: no elevated capabilities beyond rendering.
    Sandboxed,
    /// Specific, scoped capabilities granted.
    ScopedGranted,
    /// Broad capabilities granted.
    BroadGranted,
    /// Capabilities were revoked.
    Revoked,
    /// The permission posture is unknown and requires review.
    RequiresReview,
}

impl PermissionStateClass {
    /// All permission states, in set order.
    pub const ALL: [Self; 5] = [
        Self::Sandboxed,
        Self::ScopedGranted,
        Self::BroadGranted,
        Self::Revoked,
        Self::RequiresReview,
    ];

    /// Stable snake_case token for this permission state.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Sandboxed => "sandboxed",
            Self::ScopedGranted => "scoped_granted",
            Self::BroadGranted => "broad_granted",
            Self::Revoked => "revoked",
            Self::RequiresReview => "requires_review",
        }
    }

    /// Human-readable label.
    pub const fn label(self) -> &'static str {
        match self {
            Self::Sandboxed => "Sandboxed",
            Self::ScopedGranted => "Scoped capabilities granted",
            Self::BroadGranted => "Broad capabilities granted",
            Self::Revoked => "Revoked",
            Self::RequiresReview => "Requires review",
        }
    }
}

/// One named capability the embedded surface is *limited* from doing.
///
/// Capability limitations make the embedded boundary concrete: they state what the
/// surface cannot reach, so a webview never reads as having the same authority as
/// native product chrome.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CapabilityLimitationClass {
    /// Cannot present a native approval, update, or product-security prompt.
    NoNativeApproval,
    /// Cannot read or write the local filesystem.
    NoFilesystemAccess,
    /// Cannot read local credentials or tokens.
    NoCredentialAccess,
    /// Cannot run local commands or processes.
    NoLocalCommandExecution,
    /// Network access is scoped to its declared origin.
    NetworkScopedToOrigin,
    /// Renders read-only content; cannot mutate Aureline state.
    ReadOnlyContent,
}

impl CapabilityLimitationClass {
    /// All limitation classes, in set order.
    pub const ALL: [Self; 6] = [
        Self::NoNativeApproval,
        Self::NoFilesystemAccess,
        Self::NoCredentialAccess,
        Self::NoLocalCommandExecution,
        Self::NetworkScopedToOrigin,
        Self::ReadOnlyContent,
    ];

    /// Stable snake_case token for this limitation.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NoNativeApproval => "no_native_approval",
            Self::NoFilesystemAccess => "no_filesystem_access",
            Self::NoCredentialAccess => "no_credential_access",
            Self::NoLocalCommandExecution => "no_local_command_execution",
            Self::NetworkScopedToOrigin => "network_scoped_to_origin",
            Self::ReadOnlyContent => "read_only_content",
        }
    }

    /// Human-readable label.
    pub const fn label(self) -> &'static str {
        match self {
            Self::NoNativeApproval => "No native approval",
            Self::NoFilesystemAccess => "No filesystem access",
            Self::NoCredentialAccess => "No credential access",
            Self::NoLocalCommandExecution => "No local command execution",
            Self::NetworkScopedToOrigin => "Network scoped to origin",
            Self::ReadOnlyContent => "Read-only content",
        }
    }
}

// ---------------------------------------------------------------------------
// Device permissions.
// ---------------------------------------------------------------------------

/// A device capability an embedded surface may use for capture.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DevicePermissionClass {
    /// The camera.
    Camera,
    /// The microphone.
    Microphone,
    /// Screen / window capture.
    ScreenCapture,
    /// Clipboard read / write.
    Clipboard,
    /// Coarse or precise location.
    Location,
    /// OS notifications.
    Notifications,
}

impl DevicePermissionClass {
    /// All device permissions, in set order.
    pub const ALL: [Self; 6] = [
        Self::Camera,
        Self::Microphone,
        Self::ScreenCapture,
        Self::Clipboard,
        Self::Location,
        Self::Notifications,
    ];

    /// Stable snake_case token for this permission.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Camera => "camera",
            Self::Microphone => "microphone",
            Self::ScreenCapture => "screen_capture",
            Self::Clipboard => "clipboard",
            Self::Location => "location",
            Self::Notifications => "notifications",
        }
    }

    /// Human-readable label.
    pub const fn label(self) -> &'static str {
        match self {
            Self::Camera => "Camera",
            Self::Microphone => "Microphone",
            Self::ScreenCapture => "Screen capture",
            Self::Clipboard => "Clipboard",
            Self::Location => "Location",
            Self::Notifications => "Notifications",
        }
    }
}

/// Where data captured under a device permission is processed.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ProcessingClass {
    /// Captured and processed entirely on this device.
    LocalOnly,
    /// Sent to and processed by a provider.
    ProviderProcessed,
    /// Captured locally, then a derived result is sent to a provider.
    MixedLocalThenProvider,
}

impl ProcessingClass {
    /// All processing classes, in set order.
    pub const ALL: [Self; 3] = [
        Self::LocalOnly,
        Self::ProviderProcessed,
        Self::MixedLocalThenProvider,
    ];

    /// Stable snake_case token for this processing class.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LocalOnly => "local_only",
            Self::ProviderProcessed => "provider_processed",
            Self::MixedLocalThenProvider => "mixed_local_then_provider",
        }
    }

    /// Human-readable label.
    pub const fn label(self) -> &'static str {
        match self {
            Self::LocalOnly => "Local only",
            Self::ProviderProcessed => "Provider processed",
            Self::MixedLocalThenProvider => "Local then provider",
        }
    }
}

/// The storage / retention posture for data captured under a device permission.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RetentionClass {
    /// Not stored: discarded after use.
    NotStored,
    /// Held locally for the current session only.
    LocalSessionOnly,
    /// Persisted locally until the user clears it.
    LocalPersisted,
    /// Retained by a provider under its own policy.
    ProviderRetained,
}

impl RetentionClass {
    /// All retention classes, in set order.
    pub const ALL: [Self; 4] = [
        Self::NotStored,
        Self::LocalSessionOnly,
        Self::LocalPersisted,
        Self::ProviderRetained,
    ];

    /// Stable snake_case token for this retention class.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NotStored => "not_stored",
            Self::LocalSessionOnly => "local_session_only",
            Self::LocalPersisted => "local_persisted",
            Self::ProviderRetained => "provider_retained",
        }
    }

    /// Human-readable label.
    pub const fn label(self) -> &'static str {
        match self {
            Self::NotStored => "Not stored",
            Self::LocalSessionOnly => "Local session only",
            Self::LocalPersisted => "Local persisted",
            Self::ProviderRetained => "Provider retained",
        }
    }
}

/// The revoke action a device-permission row offers.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RevokeActionClass {
    /// Revoke the grant inside Aureline.
    RevokeInAureline,
    /// Open the OS system settings to change the permission.
    OpenSystemSettings,
    /// Both an in-app revoke and a system-settings deep link.
    RevokeAndOpenSystemSettings,
}

impl RevokeActionClass {
    /// All revoke actions, in set order.
    pub const ALL: [Self; 3] = [
        Self::RevokeInAureline,
        Self::OpenSystemSettings,
        Self::RevokeAndOpenSystemSettings,
    ];

    /// Stable snake_case token for this revoke action.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::RevokeInAureline => "revoke_in_aureline",
            Self::OpenSystemSettings => "open_system_settings",
            Self::RevokeAndOpenSystemSettings => "revoke_and_open_system_settings",
        }
    }

    /// Human-readable label.
    pub const fn label(self) -> &'static str {
        match self {
            Self::RevokeInAureline => "Revoke in Aureline",
            Self::OpenSystemSettings => "Open system settings",
            Self::RevokeAndOpenSystemSettings => "Revoke / open system settings",
        }
    }

    /// Whether this action deep-links to the OS system settings.
    pub const fn opens_system_settings(self) -> bool {
        matches!(
            self,
            Self::OpenSystemSettings | Self::RevokeAndOpenSystemSettings
        )
    }
}

// ---------------------------------------------------------------------------
// Auth handoff.
// ---------------------------------------------------------------------------

/// Why a surface hands auth out rather than handling it embedded.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum HandoffReasonClass {
    /// Claimed-identity sign-in must complete in the system browser.
    ClaimedIdentityAuth,
    /// Provider auth must complete on the provider's own page.
    ProviderAuthRequired,
    /// A high-risk approval cannot be granted in an embedded surface.
    HighRiskApproval,
    /// The provider manages this page and it must render in the browser.
    ProviderManagedPage,
    /// Policy requires the flow to run in an external, attributable surface.
    PolicyRequiresExternal,
}

impl HandoffReasonClass {
    /// All handoff reasons, in set order.
    pub const ALL: [Self; 5] = [
        Self::ClaimedIdentityAuth,
        Self::ProviderAuthRequired,
        Self::HighRiskApproval,
        Self::ProviderManagedPage,
        Self::PolicyRequiresExternal,
    ];

    /// Stable snake_case token for this reason.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ClaimedIdentityAuth => "claimed_identity_auth",
            Self::ProviderAuthRequired => "provider_auth_required",
            Self::HighRiskApproval => "high_risk_approval",
            Self::ProviderManagedPage => "provider_managed_page",
            Self::PolicyRequiresExternal => "policy_requires_external",
        }
    }

    /// Human-readable label.
    pub const fn label(self) -> &'static str {
        match self {
            Self::ClaimedIdentityAuth => "Claimed-identity auth",
            Self::ProviderAuthRequired => "Provider auth required",
            Self::HighRiskApproval => "High-risk approval",
            Self::ProviderManagedPage => "Provider-managed page",
            Self::PolicyRequiresExternal => "Policy requires external",
        }
    }
}

/// Where an auth handoff sends the operator.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum HandoffTargetClass {
    /// The system browser.
    SystemBrowser,
    /// A vendor portal opened in the system browser.
    VendorPortalInBrowser,
    /// A provider console opened in the system browser.
    ProviderConsoleInBrowser,
    /// A device-code verification flow (a short code, polled for completion).
    DeviceCodeVerification,
}

impl HandoffTargetClass {
    /// All handoff targets, in set order.
    pub const ALL: [Self; 4] = [
        Self::SystemBrowser,
        Self::VendorPortalInBrowser,
        Self::ProviderConsoleInBrowser,
        Self::DeviceCodeVerification,
    ];

    /// Stable snake_case token for this target.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SystemBrowser => "system_browser",
            Self::VendorPortalInBrowser => "vendor_portal_in_browser",
            Self::ProviderConsoleInBrowser => "provider_console_in_browser",
            Self::DeviceCodeVerification => "device_code_verification",
        }
    }

    /// Human-readable label.
    pub const fn label(self) -> &'static str {
        match self {
            Self::SystemBrowser => "System browser",
            Self::VendorPortalInBrowser => "Vendor portal (browser)",
            Self::ProviderConsoleInBrowser => "Provider console (browser)",
            Self::DeviceCodeVerification => "Device-code verification",
        }
    }

    /// Whether this target is the device-code flow.
    pub const fn is_device_code(self) -> bool {
        matches!(self, Self::DeviceCodeVerification)
    }

    /// Whether this target is an external, attributable exit (always true: every
    /// handoff target leaves the embedded surface).
    pub const fn is_attributable_exit(self) -> bool {
        true
    }
}

/// The fallback posture of an auth handoff.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum HandoffFallbackClass {
    /// The system browser opened successfully.
    SystemBrowserOpened,
    /// The browser could not open; a device-code fallback is offered.
    DeviceCodeFallbackAvailable,
    /// Manual code entry is offered as a fallback.
    ManualCodeEntryAvailable,
    /// The device-code flow is polling for completion.
    PollingForCompletion,
    /// No fallback is available and the handoff is blocked.
    NoFallbackBlocked,
}

impl HandoffFallbackClass {
    /// All fallback classes, in set order.
    pub const ALL: [Self; 5] = [
        Self::SystemBrowserOpened,
        Self::DeviceCodeFallbackAvailable,
        Self::ManualCodeEntryAvailable,
        Self::PollingForCompletion,
        Self::NoFallbackBlocked,
    ];

    /// Stable snake_case token for this fallback class.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SystemBrowserOpened => "system_browser_opened",
            Self::DeviceCodeFallbackAvailable => "device_code_fallback_available",
            Self::ManualCodeEntryAvailable => "manual_code_entry_available",
            Self::PollingForCompletion => "polling_for_completion",
            Self::NoFallbackBlocked => "no_fallback_blocked",
        }
    }

    /// Human-readable label.
    pub const fn label(self) -> &'static str {
        match self {
            Self::SystemBrowserOpened => "System browser opened",
            Self::DeviceCodeFallbackAvailable => "Device-code fallback available",
            Self::ManualCodeEntryAvailable => "Manual code entry available",
            Self::PollingForCompletion => "Polling for completion",
            Self::NoFallbackBlocked => "No fallback — blocked",
        }
    }

    /// Whether this fallback state blocks the handoff from completing.
    pub const fn is_blocked(self) -> bool {
        matches!(self, Self::NoFallbackBlocked)
    }
}

// ---------------------------------------------------------------------------
// Computed state.
// ---------------------------------------------------------------------------

/// The displayed matrix state a card maps to before the no-silent-green downgrade.
///
/// An auth handoff is an [`OperatorStateClass::EmbeddedBoundaryHandoff`]; an
/// unknown / undeterminable origin requires a boundary recheck; a blocked or
/// expired device-code handoff is [`OperatorStateClass::Blocked`]; a snapshot-only
/// surface is [`OperatorStateClass::ImportedSnapshotNoLive`]; everything else is
/// [`OperatorStateClass::Clear`] and may be downgraded by freshness.
pub fn displayed_state(
    kind: EmbeddedSurfaceKind,
    owner: OriginOwnerClass,
    live_vs_snapshot: LiveSnapshotClass,
    handoff_blocked: bool,
) -> OperatorStateClass {
    if handoff_blocked {
        return OperatorStateClass::Blocked;
    }
    if owner.requires_review() {
        return OperatorStateClass::BoundaryDriftRecheckRequired;
    }
    if live_vs_snapshot == LiveSnapshotClass::SnapshotOnly {
        return OperatorStateClass::ImportedSnapshotNoLive;
    }
    if kind.is_auth_handoff() {
        return OperatorStateClass::EmbeddedBoundaryHandoff;
    }
    OperatorStateClass::Clear
}

// ---------------------------------------------------------------------------
// Record structs.
// ---------------------------------------------------------------------------

/// An open-in-browser action attached to an origin bar.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OpenInBrowserAction {
    /// Whether an open-in-browser action is available for this surface.
    pub available: bool,
    /// The handoff target the action opens.
    pub target: HandoffTargetClass,
    /// The visible label of the action.
    pub label: String,
}

/// One named capability limitation with a reviewable note.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CapabilityLimitation {
    /// The limitation class.
    pub class: CapabilityLimitationClass,
    /// Human-readable label.
    pub label: String,
    /// One reviewable sentence stating the limitation.
    pub note: String,
}

/// The origin bar every embedded surface carries.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OriginBar {
    /// Who owns and renders the content.
    pub owner_class: OriginOwnerClass,
    /// An opaque, reviewable owner label (no raw hostnames, URLs, or credentials).
    pub owner_label: String,
    /// An opaque `aureline://` origin handle.
    pub origin_ref: String,
    /// The extension that provides the surface, if extension-provided.
    pub extension_ref: Option<String>,
    /// The permission / capability posture of the surface.
    pub permission_state: PermissionStateClass,
    /// The named capability limitations the surface declares.
    pub capability_limitations: Vec<CapabilityLimitation>,
    /// The open-in-browser action.
    pub open_in_browser: OpenInBrowserAction,
    /// The freshness of the surface's content.
    pub freshness: FreshnessClass,
    /// The latest moment the surface's content was refreshed (RFC3339, with offset).
    pub latest_refresh_at: String,
    /// Whether the surface impersonates a native approval / update / security
    /// surface (always false for this record).
    pub native_surface_impersonation: bool,
    /// The required visible language the surface shows verbatim — for example
    /// `Embedded — Provider: Example CI`.
    pub required_visible_language: String,
}

/// One device-permission row attached to a capture surface.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DevicePermissionRow {
    /// The device capability in play.
    pub permission: DevicePermissionClass,
    /// Human-readable label.
    pub label: String,
    /// The actor using the capability (an opaque, reviewable label).
    pub actor: String,
    /// Where the captured data is processed.
    pub processing_class: ProcessingClass,
    /// The storage / retention posture for the captured data.
    pub retention_class: RetentionClass,
    /// One reviewable sentence stating the storage / retention note.
    pub retention_note: String,
    /// The revoke action this row offers.
    pub revoke_action: RevokeActionClass,
    /// Whether the revoke action deep-links to the OS system settings.
    pub opens_system_settings: bool,
    /// One reviewable sentence stating what stays local if the permission is
    /// revoked.
    pub local_continuity: String,
    /// Whether the permission is currently granted.
    pub granted: bool,
}

/// The browser / device-code auth handoff card.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AuthHandoffCard {
    /// Why the surface hands auth out rather than handling it embedded.
    pub reason: HandoffReasonClass,
    /// One reviewable sentence stating the reason.
    pub reason_note: String,
    /// Where the handoff sends the operator.
    pub target: HandoffTargetClass,
    /// Whether the handoff prefers the system browser or a device-code flow
    /// (always true: a handoff never completes inside the embedded surface).
    pub prefers_external: bool,
    /// The fallback posture of the handoff.
    pub fallback: HandoffFallbackClass,
    /// Whether a verification code is shown (device-code flow).
    pub verification_code_shown: bool,
    /// The display class of the verification code (the code value never crosses
    /// this boundary).
    pub code_display_class: Option<String>,
    /// The verification code expiry (RFC3339, with offset), if a code is shown.
    pub code_expiry_at: Option<String>,
    /// Whether the verification code has expired.
    pub code_expired: bool,
    /// One reviewable sentence stating the return path back into Aureline.
    pub return_path: String,
    /// The canonical `aureline://` object the handoff returns to.
    pub return_anchor_ref: String,
    /// Whether the handoff is exposed behind a generic `Continue` affordance
    /// (always false: the reason and target are explicit).
    pub hidden_behind_generic_continue: bool,
}

/// The local-continuity posture of an embedded surface while the boundary is in
/// effect.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LocalContinuityClass {
    /// Local work continues unaffected.
    LocalWorkContinues,
    /// The surface is read-only until the operator returns from the handoff.
    ReadOnlyUntilReturn,
    /// Managed work is blocked until auth completes; local work still continues.
    BlockedUntilAuth,
}

impl LocalContinuityClass {
    /// All continuity classes, in set order.
    pub const ALL: [Self; 3] = [
        Self::LocalWorkContinues,
        Self::ReadOnlyUntilReturn,
        Self::BlockedUntilAuth,
    ];

    /// Stable snake_case token for this continuity class.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LocalWorkContinues => "local_work_continues",
            Self::ReadOnlyUntilReturn => "read_only_until_return",
            Self::BlockedUntilAuth => "blocked_until_auth",
        }
    }

    /// Human-readable label.
    pub const fn label(self) -> &'static str {
        match self {
            Self::LocalWorkContinues => "Local work continues",
            Self::ReadOnlyUntilReturn => "Read-only until return",
            Self::BlockedUntilAuth => "Blocked until auth",
        }
    }
}

/// One embedded surface card.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EmbeddedSurfaceCard {
    /// Stable card id.
    pub card_id: String,
    /// The canonical object handle this card is about.
    pub object_ref: String,
    /// Short title.
    pub title: String,
    /// One reviewable sentence describing the surface.
    pub summary: String,
    /// The surface kind.
    pub kind: EmbeddedSurfaceKind,
    /// The bound matrix surface family (always the embedded boundary state).
    pub surface: OperatorSurfaceClass,
    /// The bound matrix surface id.
    pub surface_id: String,
    /// The displayed matrix state before the no-silent-green downgrade.
    pub displayed_state: OperatorStateClass,
    /// The computed effective state — the no-silent-green downgrade of the
    /// displayed state and the origin-bar freshness.
    pub effective_state: OperatorStateClass,
    /// The surface owner.
    pub owner: String,
    /// Who holds the decision right for trusting this surface.
    pub decision_right: String,
    /// Local-versus-shared scope of the underlying object.
    pub scope: ScopeClass,
    /// The default redaction posture on export.
    pub default_redaction: RedactionClass,
    /// The consumers that render this surface.
    pub consumed_by: Vec<ConsumerClass>,
    /// Live-versus-snapshot posture.
    pub live_vs_snapshot: LiveSnapshotClass,
    /// The origin bar (every embedded surface carries one).
    pub origin_bar: OriginBar,
    /// The device-permission rows (empty unless the surface uses device
    /// capabilities).
    pub device_permissions: Vec<DevicePermissionRow>,
    /// The auth handoff card (present only for handoff surfaces).
    pub auth_handoff: Option<AuthHandoffCard>,
    /// The local-continuity posture while the boundary is in effect.
    pub local_continuity: LocalContinuityClass,
    /// The canonical object the open-details affordance routes to (equals
    /// [`EmbeddedSurfaceCard::object_ref`]).
    pub open_detail_ref: String,
}

impl EmbeddedSurfaceCard {
    /// Whether the surface's auth handoff is blocked (no fallback).
    pub fn handoff_blocked(&self) -> bool {
        self.auth_handoff
            .as_ref()
            .is_some_and(|h| h.fallback.is_blocked())
    }
}

/// One frozen invariant, with a computed `holds` flag.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EmbeddedInvariant {
    /// Stable invariant id.
    pub invariant_id: String,
    /// The invariant statement.
    pub statement: String,
    /// Whether the built set satisfies the invariant.
    pub holds: bool,
}

/// The frozen embedded-surface set.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EmbeddedSurfaceSet {
    /// Stable record-kind tag.
    pub record_kind: String,
    /// Schema version.
    pub m5_embedded_dashboards_schema_version: u32,
    /// Schema reference.
    pub schema_ref: String,
    /// Stable set id.
    pub set_id: String,
    /// Evaluation stamp.
    pub as_of: String,
    /// One reviewable sentence summarizing the set.
    pub summary: String,
    /// The operator-surface matrix fixture this set binds for surface identity.
    pub matrix_ref: String,
    /// The matrix record kind this set binds.
    pub matrix_record_kind: String,
    /// The surface-kind vocabulary, for consumers.
    pub surface_kind_vocabulary: Vec<TokenDef>,
    /// The origin-owner vocabulary, for consumers.
    pub origin_owner_vocabulary: Vec<TokenDef>,
    /// The capability-limitation vocabulary, for consumers.
    pub capability_limitation_vocabulary: Vec<TokenDef>,
    /// The device-permission vocabulary, for consumers.
    pub device_permission_vocabulary: Vec<TokenDef>,
    /// The handoff-target vocabulary, for consumers.
    pub handoff_target_vocabulary: Vec<TokenDef>,
    /// The handoff-reason vocabulary, for consumers.
    pub handoff_reason_vocabulary: Vec<TokenDef>,
    /// The cards.
    pub surfaces: Vec<EmbeddedSurfaceCard>,
    /// The computed invariants.
    pub invariants: Vec<EmbeddedInvariant>,
    /// Whether raw payloads are excluded (always true for this record).
    pub raw_payload_excluded: bool,
}

/// Error returned when the set fails a structural consistency check.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EmbeddedValidationError {
    /// The failed check.
    pub reason: String,
}

impl std::fmt::Display for EmbeddedValidationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "embedded-surface set invalid: {}", self.reason)
    }
}

impl std::error::Error for EmbeddedValidationError {}

impl EmbeddedSurfaceSet {
    /// Returns the card with the given id, if present.
    pub fn surface(&self, card_id: &str) -> Option<&EmbeddedSurfaceCard> {
        self.surfaces.iter().find(|c| c.card_id == card_id)
    }

    /// Whether every computed invariant holds.
    pub fn all_invariants_hold(&self) -> bool {
        self.invariants.iter().all(|i| i.holds)
    }

    /// Whether the record is safe to place in a support export: raw payloads are
    /// excluded and every ref is a repo-relative object ref or `aureline://`
    /// handle, never a URL, host, credential, or absolute path.
    pub fn is_support_export_safe(&self) -> bool {
        if !self.raw_payload_excluded {
            return false;
        }
        self.all_refs().iter().all(|r| is_export_safe_ref(r))
    }

    /// Every ref string carried by the set, for export-safety auditing.
    fn all_refs(&self) -> Vec<&str> {
        let mut refs = vec![self.matrix_ref.as_str(), self.schema_ref.as_str()];
        for c in &self.surfaces {
            refs.push(c.object_ref.as_str());
            refs.push(c.open_detail_ref.as_str());
            refs.push(c.origin_bar.origin_ref.as_str());
            if let Some(ext) = &c.origin_bar.extension_ref {
                refs.push(ext.as_str());
            }
            if let Some(h) = &c.auth_handoff {
                refs.push(h.return_anchor_ref.as_str());
            }
        }
        refs
    }

    /// Re-checks structural consistency and returns an error on the first failure.
    /// Complements the computed [`EmbeddedInvariant`]s with the uniqueness,
    /// surface-binding, computed-state, origin, device-permission, and handoff
    /// checks a consumer relies on.
    pub fn validate(&self) -> Result<(), EmbeddedValidationError> {
        let fail = |reason: String| Err(EmbeddedValidationError { reason });

        if self.record_kind != M5_EMBEDDED_DASHBOARDS_RECORD_KIND {
            return fail(format!("unexpected record_kind {}", self.record_kind));
        }
        if self.schema_ref != M5_EMBEDDED_DASHBOARDS_SCHEMA_REF {
            return fail(format!("unexpected schema_ref {}", self.schema_ref));
        }
        if !self.raw_payload_excluded {
            return fail("raw_payload_excluded must be true".to_owned());
        }
        if self.matrix_record_kind != M5_EMBEDDED_DASHBOARDS_MATRIX_RECORD_KIND {
            return fail("matrix_record_kind must bind the operator-surface matrix".to_owned());
        }
        if self.surfaces.is_empty() {
            return fail("set has no surfaces".to_owned());
        }
        if !all_unique(self.surfaces.iter().map(|c| c.card_id.as_str())) {
            return fail("card ids are not unique".to_owned());
        }

        let matrix = crate::m5_operator_surfaces::operator_surface_matrix();
        let embedded = OperatorSurfaceClass::EmbeddedBoundaryState;

        for c in &self.surfaces {
            // Every card binds the embedded-boundary matrix family by its id.
            if c.surface != embedded
                || c.surface_id != embedded.surface_id()
                || matrix.surface(embedded).is_none()
            {
                return fail(format!(
                    "card {} does not bind the matrix surface",
                    c.card_id
                ));
            }
            // Canonical identity.
            if !c.object_ref.starts_with("aureline://") || c.open_detail_ref != c.object_ref {
                return fail(format!("card {} hides its canonical object", c.card_id));
            }
            if c.owner.is_empty() || c.decision_right.is_empty() {
                return fail(format!("card {} hides owner / decision-right", c.card_id));
            }

            // Origin bar: owner / origin visible, never native-surface impersonation.
            let bar = &c.origin_bar;
            if bar.owner_label.is_empty() || !bar.origin_ref.starts_with("aureline://") {
                return fail(format!("card {} hides its origin", c.card_id));
            }
            if bar.native_surface_impersonation {
                return fail(format!("card {} impersonates a native surface", c.card_id));
            }
            if bar.required_visible_language.is_empty() {
                return fail(format!(
                    "card {} shows no required visible language",
                    c.card_id
                ));
            }
            // Extension-provided surfaces name the extension; others do not invent one.
            match (bar.owner_class, &bar.extension_ref) {
                (OriginOwnerClass::ExtensionProvided, None) => {
                    return fail(format!(
                        "card {} is extension-provided but names no extension",
                        c.card_id
                    ))
                }
                (OriginOwnerClass::ExtensionProvided, Some(ext)) if ext.is_empty() => {
                    return fail(format!("card {} names an empty extension", c.card_id))
                }
                _ => {}
            }
            // Embedded webviews name at least one capability limitation and an
            // open-in-browser action, and always declare no-native-approval.
            if c.kind.is_embedded_webview() {
                if bar.capability_limitations.is_empty() {
                    return fail(format!(
                        "card {} is an embedded webview with no capability limitation",
                        c.card_id
                    ));
                }
                if !bar
                    .capability_limitations
                    .iter()
                    .any(|l| l.class == CapabilityLimitationClass::NoNativeApproval)
                {
                    return fail(format!(
                        "card {} does not declare the no-native-approval limitation",
                        c.card_id
                    ));
                }
            }
            if c.kind.requires_open_in_browser() && !bar.open_in_browser.available {
                return fail(format!(
                    "card {} is an embedded page with no open-in-browser action",
                    c.card_id
                ));
            }
            for limitation in &bar.capability_limitations {
                if limitation.note.is_empty() {
                    return fail(format!(
                        "card {} declares a capability limitation with no note",
                        c.card_id
                    ));
                }
            }
            if bar.open_in_browser.available && bar.open_in_browser.label.is_empty() {
                return fail(format!(
                    "card {} offers an open-in-browser action with no label",
                    c.card_id
                ));
            }
            if !timestamp_carries_offset(&bar.latest_refresh_at) {
                return fail(format!(
                    "card {} latest-refresh stamp carries no explicit offset",
                    c.card_id
                ));
            }
            if parse_rfc3339(&bar.latest_refresh_at).is_none() {
                return fail(format!(
                    "card {} has an unparseable latest-refresh stamp",
                    c.card_id
                ));
            }

            // Device-permission rows disclose actor, processing, retention, revoke,
            // and local-continuity.
            if c.kind == EmbeddedSurfaceKind::DeviceCaptureSurface
                && c.device_permissions.is_empty()
            {
                return fail(format!(
                    "card {} is a device-capture surface with no device permissions",
                    c.card_id
                ));
            }
            for row in &c.device_permissions {
                if row.actor.is_empty() {
                    return fail(format!("card {} device row names no actor", c.card_id));
                }
                if row.retention_note.is_empty() {
                    return fail(format!(
                        "card {} device row names no retention note",
                        c.card_id
                    ));
                }
                if row.local_continuity.is_empty() {
                    return fail(format!(
                        "card {} device row names no local-continuity posture",
                        c.card_id
                    ));
                }
                if row.opens_system_settings != row.revoke_action.opens_system_settings() {
                    return fail(format!(
                        "card {} device row open-system-settings flag is not computed",
                        c.card_id
                    ));
                }
            }

            // Auth handoff cards make reason, target, code/expiry, fallback, and
            // return path explicit, never behind a generic Continue.
            match (c.kind.is_auth_handoff(), &c.auth_handoff) {
                (true, None) => {
                    return fail(format!(
                        "card {} is a handoff with no handoff card",
                        c.card_id
                    ))
                }
                (false, Some(_)) => {
                    return fail(format!(
                        "card {} is not a handoff but carries a handoff card",
                        c.card_id
                    ))
                }
                (false, None) => {}
                (true, Some(h)) => {
                    if h.reason_note.is_empty() {
                        return fail(format!("card {} handoff names no reason", c.card_id));
                    }
                    if !h.prefers_external {
                        return fail(format!(
                            "card {} handoff does not prefer an external surface",
                            c.card_id
                        ));
                    }
                    if h.hidden_behind_generic_continue {
                        return fail(format!(
                            "card {} hides its handoff behind a generic Continue",
                            c.card_id
                        ));
                    }
                    if h.return_path.is_empty() || !h.return_anchor_ref.starts_with("aureline://") {
                        return fail(format!("card {} handoff hides its return path", c.card_id));
                    }
                    // Device-code handoffs show a code class and expiry.
                    let is_device_code = c.kind == EmbeddedSurfaceKind::DeviceCodeAuthHandoff
                        || h.target.is_device_code();
                    if is_device_code {
                        if !h.verification_code_shown {
                            return fail(format!(
                                "card {} is a device-code handoff that shows no code",
                                c.card_id
                            ));
                        }
                        match (&h.code_display_class, &h.code_expiry_at) {
                            (Some(class), Some(expiry)) => {
                                if class.is_empty() {
                                    return fail(format!(
                                        "card {} device-code handoff names an empty code class",
                                        c.card_id
                                    ));
                                }
                                if !timestamp_carries_offset(expiry)
                                    || parse_rfc3339(expiry).is_none()
                                {
                                    return fail(format!(
                                        "card {} device-code expiry is not a valid timestamp",
                                        c.card_id
                                    ));
                                }
                            }
                            _ => {
                                return fail(format!(
                                    "card {} device-code handoff hides its code class or expiry",
                                    c.card_id
                                ))
                            }
                        }
                    }
                    // An expired or no-fallback handoff blocks; the displayed state
                    // must reflect that.
                    let blocked = h.code_expired || h.fallback.is_blocked();
                    if blocked && c.displayed_state != OperatorStateClass::Blocked {
                        return fail(format!(
                            "card {} is a blocked/expired handoff but not in the blocked state",
                            c.card_id
                        ));
                    }
                }
            }

            // Displayed state matches the computed mapping.
            let expected_displayed = displayed_state(
                c.kind,
                bar.owner_class,
                c.live_vs_snapshot,
                c.handoff_blocked(),
            );
            // A code-expired handoff is also a blocked displayed state.
            let expected_displayed = if c.auth_handoff.as_ref().is_some_and(|h| h.code_expired) {
                OperatorStateClass::Blocked
            } else {
                expected_displayed
            };
            if c.displayed_state != expected_displayed {
                return fail(format!(
                    "card {} displayed state is not the computed state",
                    c.card_id
                ));
            }
            // Effective state is the computed no-silent-green downgrade.
            let expected_effective =
                compute_effective_state(c.displayed_state, bar.freshness, BlockerWaiverClass::None);
            if c.effective_state != expected_effective {
                return fail(format!(
                    "card {} effective state is not the computed no-silent-green state",
                    c.card_id
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

/// Parses an RFC3339 timestamp, returning `None` on any parse failure.
fn parse_rfc3339(value: &str) -> Option<OffsetDateTime> {
    OffsetDateTime::parse(value, &Rfc3339).ok()
}

/// Whether a timestamp carries an explicit UTC offset (a trailing `Z` or a
/// `±hh:mm` offset), so no surface relies on a vague local-without-zone time.
fn timestamp_carries_offset(ts: &str) -> bool {
    if !ts.contains('T') {
        return false;
    }
    ts.ends_with('Z') || has_numeric_offset_suffix(ts)
}

/// Whether the string ends with a `±hh:mm` offset.
fn has_numeric_offset_suffix(ts: &str) -> bool {
    if ts.len() < 6 {
        return false;
    }
    let suffix = &ts[ts.len() - 6..];
    is_utc_offset(suffix)
}

/// Whether the string is a `±hh:mm` UTC offset (for example `-04:00`).
fn is_utc_offset(s: &str) -> bool {
    let bytes = s.as_bytes();
    if bytes.len() != 6 {
        return false;
    }
    (bytes[0] == b'+' || bytes[0] == b'-')
        && bytes[1].is_ascii_digit()
        && bytes[2].is_ascii_digit()
        && bytes[3] == b':'
        && bytes[4].is_ascii_digit()
        && bytes[5].is_ascii_digit()
}

// ---------------------------------------------------------------------------
// Canonical binding.
// ---------------------------------------------------------------------------

/// Builds the canonical embedded-surface set.
///
/// Deterministic: the same bytes every call. Each card's displayed and effective
/// state and the invariant `holds` flags are computed from the built data, so an
/// inconsistent edit flips an invariant rather than silently passing.
pub fn embedded_surface_set() -> EmbeddedSurfaceSet {
    let surfaces = build_surfaces();
    let invariants = compute_invariants(&surfaces);

    EmbeddedSurfaceSet {
        record_kind: M5_EMBEDDED_DASHBOARDS_RECORD_KIND.to_owned(),
        m5_embedded_dashboards_schema_version: M5_EMBEDDED_DASHBOARDS_SCHEMA_VERSION,
        schema_ref: M5_EMBEDDED_DASHBOARDS_SCHEMA_REF.to_owned(),
        set_id: M5_EMBEDDED_DASHBOARDS_SET_ID.to_owned(),
        as_of: M5_EMBEDDED_DASHBOARDS_AS_OF.to_owned(),
        summary: "Embedded service-dashboard / provider-page origin bars, device-permission rows, \
                  and browser / device-code auth handoff cards with owner/origin and capability \
                  truth, device processing/retention/revoke disclosure, and explicit handoff \
                  reason/target/code/expiry/return paths — bound to the operator-surface matrix so \
                  no embedded content impersonates native approval and no boundary hides behind a \
                  generic Continue."
            .to_owned(),
        matrix_ref: M5_EMBEDDED_DASHBOARDS_MATRIX_REF.to_owned(),
        matrix_record_kind: M5_EMBEDDED_DASHBOARDS_MATRIX_RECORD_KIND.to_owned(),
        surface_kind_vocabulary: token_defs(
            EmbeddedSurfaceKind::ALL
                .iter()
                .map(|k| (k.as_str(), k.label())),
        ),
        origin_owner_vocabulary: token_defs(
            OriginOwnerClass::ALL
                .iter()
                .map(|o| (o.as_str(), o.label())),
        ),
        capability_limitation_vocabulary: token_defs(
            CapabilityLimitationClass::ALL
                .iter()
                .map(|l| (l.as_str(), l.label())),
        ),
        device_permission_vocabulary: token_defs(
            DevicePermissionClass::ALL
                .iter()
                .map(|p| (p.as_str(), p.label())),
        ),
        handoff_target_vocabulary: token_defs(
            HandoffTargetClass::ALL
                .iter()
                .map(|t| (t.as_str(), t.label())),
        ),
        handoff_reason_vocabulary: token_defs(
            HandoffReasonClass::ALL
                .iter()
                .map(|r| (r.as_str(), r.label())),
        ),
        surfaces,
        invariants,
        raw_payload_excluded: true,
    }
}

fn token_defs<'a>(items: impl Iterator<Item = (&'a str, &'a str)>) -> Vec<TokenDef> {
    items
        .map(|(token, label)| TokenDef {
            token: token.to_owned(),
            label: label.to_owned(),
        })
        .collect()
}

fn limitation(class: CapabilityLimitationClass, note: &str) -> CapabilityLimitation {
    CapabilityLimitation {
        class,
        label: class.label().to_owned(),
        note: note.to_owned(),
    }
}

#[allow(clippy::too_many_arguments)]
fn device_row(
    permission: DevicePermissionClass,
    processing_class: ProcessingClass,
    actor: &str,
    retention_class: RetentionClass,
    retention_note: &str,
    revoke_action: RevokeActionClass,
    local_continuity: &str,
    granted: bool,
) -> DevicePermissionRow {
    DevicePermissionRow {
        permission,
        label: permission.label().to_owned(),
        actor: actor.to_owned(),
        processing_class,
        retention_class,
        retention_note: retention_note.to_owned(),
        revoke_action,
        opens_system_settings: revoke_action.opens_system_settings(),
        local_continuity: local_continuity.to_owned(),
        granted,
    }
}

/// Assembles one card, computing its displayed and effective state from inputs.
#[allow(clippy::too_many_arguments)]
fn card(
    card_id: &str,
    object_ref: &str,
    title: &str,
    summary: &str,
    kind: EmbeddedSurfaceKind,
    owner: &str,
    decision_right: &str,
    scope: ScopeClass,
    default_redaction: RedactionClass,
    consumed_by: Vec<ConsumerClass>,
    live_vs_snapshot: LiveSnapshotClass,
    origin_bar: OriginBar,
    device_permissions: Vec<DevicePermissionRow>,
    auth_handoff: Option<AuthHandoffCard>,
    local_continuity: LocalContinuityClass,
) -> EmbeddedSurfaceCard {
    let embedded = OperatorSurfaceClass::EmbeddedBoundaryState;
    let handoff_blocked = auth_handoff
        .as_ref()
        .is_some_and(|h| h.fallback.is_blocked());
    let code_expired = auth_handoff.as_ref().is_some_and(|h| h.code_expired);
    let displayed = if code_expired {
        OperatorStateClass::Blocked
    } else {
        displayed_state(
            kind,
            origin_bar.owner_class,
            live_vs_snapshot,
            handoff_blocked,
        )
    };
    let effective =
        compute_effective_state(displayed, origin_bar.freshness, BlockerWaiverClass::None);

    EmbeddedSurfaceCard {
        card_id: card_id.to_owned(),
        object_ref: object_ref.to_owned(),
        title: title.to_owned(),
        summary: summary.to_owned(),
        kind,
        surface: embedded,
        surface_id: embedded.surface_id(),
        displayed_state: displayed,
        effective_state: effective,
        owner: owner.to_owned(),
        decision_right: decision_right.to_owned(),
        scope,
        default_redaction,
        consumed_by,
        live_vs_snapshot,
        origin_bar,
        device_permissions,
        auth_handoff,
        local_continuity,
        open_detail_ref: object_ref.to_owned(),
    }
}

fn build_surfaces() -> Vec<EmbeddedSurfaceCard> {
    use ConsumerClass::*;

    vec![
        // 1. First-party webview service dashboard (clear, fresh).
        card(
            "m5-embedded-dashboards:card:0001",
            "aureline://embedded/service-dashboard/0001",
            "Service health dashboard",
            "An Aureline-owned observability dashboard rendered in a webview; the origin bar names \
             it as Aureline chrome and lists what the webview cannot reach.",
            EmbeddedSurfaceKind::ServiceDashboard,
            "platform-observability",
            "platform-observability owns trust for embedded service dashboards",
            ScopeClass::SharedTeam,
            RedactionClass::MetadataSafeDefault,
            vec![ShellUi, CliHeadless, IncidentWorkspace, SupportExport],
            LiveSnapshotClass::SnapshotCapable,
            OriginBar {
                owner_class: OriginOwnerClass::FirstPartyWebview,
                owner_label: "Aureline service dashboard".to_owned(),
                origin_ref: "aureline://embedded/origin/service-dashboard".to_owned(),
                extension_ref: None,
                permission_state: PermissionStateClass::Sandboxed,
                capability_limitations: vec![
                    limitation(
                        CapabilityLimitationClass::NoNativeApproval,
                        "Cannot present a native approval, update, or security prompt.",
                    ),
                    limitation(
                        CapabilityLimitationClass::ReadOnlyContent,
                        "Renders read-only metrics; cannot mutate Aureline state.",
                    ),
                    limitation(
                        CapabilityLimitationClass::NetworkScopedToOrigin,
                        "Network is scoped to the dashboard origin.",
                    ),
                ],
                open_in_browser: OpenInBrowserAction {
                    available: true,
                    target: HandoffTargetClass::SystemBrowser,
                    label: "Open dashboard in browser".to_owned(),
                },
                freshness: FreshnessClass::Fresh,
                latest_refresh_at: "2026-06-22T00:00:00Z".to_owned(),
                native_surface_impersonation: false,
                required_visible_language: "Embedded — Aureline service dashboard".to_owned(),
            },
            vec![],
            None,
            LocalContinuityClass::LocalWorkContinues,
        ),
        // 2. Extension-provided dashboard (scoped grant, recent).
        card(
            "m5-embedded-dashboards:card:0002",
            "aureline://embedded/service-dashboard/0002",
            "CI pipeline dashboard (extension)",
            "An extension-provided CI dashboard; the origin bar names the extension and the scoped \
             capabilities it holds, and offers an open-in-browser exit.",
            EmbeddedSurfaceKind::ServiceDashboard,
            "developer-tools-guild",
            "the installing operator owns trust for extension dashboards",
            ScopeClass::SharedTeam,
            RedactionClass::MetadataSafeDefault,
            vec![ShellUi, CliHeadless, IncidentWorkspace, SupportExport],
            LiveSnapshotClass::SnapshotCapable,
            OriginBar {
                owner_class: OriginOwnerClass::ExtensionProvided,
                owner_label: "Extension: pipeline-dashboard".to_owned(),
                origin_ref: "aureline://embedded/origin/extension-ci-dashboard".to_owned(),
                extension_ref: Some("aureline://extension/pipeline-dashboard".to_owned()),
                permission_state: PermissionStateClass::ScopedGranted,
                capability_limitations: vec![
                    limitation(
                        CapabilityLimitationClass::NoNativeApproval,
                        "Cannot present a native approval; status comes from the provider only.",
                    ),
                    limitation(
                        CapabilityLimitationClass::NoCredentialAccess,
                        "Cannot read local credentials or tokens.",
                    ),
                    limitation(
                        CapabilityLimitationClass::NoLocalCommandExecution,
                        "Cannot run local commands or processes.",
                    ),
                ],
                open_in_browser: OpenInBrowserAction {
                    available: true,
                    target: HandoffTargetClass::SystemBrowser,
                    label: "Open pipeline in browser".to_owned(),
                },
                freshness: FreshnessClass::Recent,
                latest_refresh_at: "2026-06-21T23:40:00Z".to_owned(),
                native_surface_impersonation: false,
                required_visible_language: "Embedded — Extension: pipeline-dashboard".to_owned(),
            },
            vec![],
            None,
            LocalContinuityClass::LocalWorkContinues,
        ),
        // 3. Third-party provider page (broad grant, stale → unconfirmed downgrade).
        card(
            "m5-embedded-dashboards:card:0003",
            "aureline://embedded/provider-page/0003",
            "Provider billing console",
            "A third-party provider page rendered in a webview; its content is stale so the headline \
             downgrades from clear to unconfirmed, and the operator is steered to open it in the \
             browser.",
            EmbeddedSurfaceKind::ProviderPage,
            "billing-admins",
            "billing-admins own trust for the provider billing console",
            ScopeClass::ManagedOrg,
            RedactionClass::OperatorOnlyRestricted,
            vec![ShellUi, CliHeadless, AdminQueue, SupportExport, ManagedService],
            LiveSnapshotClass::SnapshotCapable,
            OriginBar {
                owner_class: OriginOwnerClass::ThirdPartyProvider,
                owner_label: "Provider: billing console".to_owned(),
                origin_ref: "aureline://embedded/origin/provider-billing".to_owned(),
                extension_ref: None,
                permission_state: PermissionStateClass::BroadGranted,
                capability_limitations: vec![
                    limitation(
                        CapabilityLimitationClass::NoNativeApproval,
                        "Provider content cannot present an Aureline approval or security prompt.",
                    ),
                    limitation(
                        CapabilityLimitationClass::NoFilesystemAccess,
                        "Cannot read or write the local filesystem.",
                    ),
                    limitation(
                        CapabilityLimitationClass::NoCredentialAccess,
                        "Cannot read local credentials; sign-in is a separate browser handoff.",
                    ),
                ],
                open_in_browser: OpenInBrowserAction {
                    available: true,
                    target: HandoffTargetClass::VendorPortalInBrowser,
                    label: "Open billing console in browser".to_owned(),
                },
                freshness: FreshnessClass::Stale,
                latest_refresh_at: "2026-06-21T08:00:00Z".to_owned(),
                native_surface_impersonation: false,
                required_visible_language: "Embedded — Provider: billing console".to_owned(),
            },
            vec![],
            None,
            LocalContinuityClass::LocalWorkContinues,
        ),
        // 4. Unknown-origin surface (requires boundary recheck).
        card(
            "m5-embedded-dashboards:card:0004",
            "aureline://embedded/provider-page/0004",
            "Unverified embedded page",
            "An embedded page whose origin could not be verified; it requires a boundary recheck \
             before trust and never presents as native chrome.",
            EmbeddedSurfaceKind::ProviderPage,
            "platform-observability",
            "platform-observability owns trust review for unverified origins",
            ScopeClass::SharedTeam,
            RedactionClass::OperatorOnlyRestricted,
            vec![ShellUi, CliHeadless, IncidentWorkspace, SupportExport],
            LiveSnapshotClass::SnapshotCapable,
            OriginBar {
                owner_class: OriginOwnerClass::UnknownOrigin,
                owner_label: "Unknown origin — unverified".to_owned(),
                origin_ref: "aureline://embedded/origin/unverified".to_owned(),
                extension_ref: None,
                permission_state: PermissionStateClass::RequiresReview,
                capability_limitations: vec![
                    limitation(
                        CapabilityLimitationClass::NoNativeApproval,
                        "An unverified surface can never present a native approval.",
                    ),
                    limitation(
                        CapabilityLimitationClass::ReadOnlyContent,
                        "Held read-only until the origin is reviewed.",
                    ),
                ],
                open_in_browser: OpenInBrowserAction {
                    available: true,
                    target: HandoffTargetClass::SystemBrowser,
                    label: "Open in browser to verify".to_owned(),
                },
                freshness: FreshnessClass::Fresh,
                latest_refresh_at: "2026-06-22T00:00:00Z".to_owned(),
                native_surface_impersonation: false,
                required_visible_language: "Embedded — Unknown origin (requires review)".to_owned(),
            },
            vec![],
            None,
            LocalContinuityClass::ReadOnlyUntilReturn,
        ),
        // 5. Device-capture surface (screen + mic; local-then-provider).
        card(
            "m5-embedded-dashboards:card:0005",
            "aureline://embedded/device-capture/0005",
            "Session capture for a support bundle",
            "A capture surface that records the screen and microphone for a support bundle; each \
             device-permission row names the actor, where capture is processed, how long it is \
             kept, and how to revoke it.",
            EmbeddedSurfaceKind::DeviceCaptureSurface,
            "support-engineering",
            "the capturing operator owns the device grant",
            ScopeClass::LocalPrivate,
            RedactionClass::PrivateTriageOnly,
            vec![ShellUi, CliHeadless, SupportExport],
            LiveSnapshotClass::SnapshotCapable,
            OriginBar {
                owner_class: OriginOwnerClass::FirstPartyNativeChrome,
                owner_label: "Aureline session capture".to_owned(),
                origin_ref: "aureline://embedded/origin/session-capture".to_owned(),
                extension_ref: None,
                permission_state: PermissionStateClass::ScopedGranted,
                capability_limitations: vec![
                    limitation(
                        CapabilityLimitationClass::NoNativeApproval,
                        "Capture chrome cannot stand in for a native approval prompt.",
                    ),
                    limitation(
                        CapabilityLimitationClass::NetworkScopedToOrigin,
                        "Uploads only happen through the reviewed support-bundle path.",
                    ),
                ],
                open_in_browser: OpenInBrowserAction {
                    available: false,
                    target: HandoffTargetClass::SystemBrowser,
                    label: String::new(),
                },
                freshness: FreshnessClass::Fresh,
                latest_refresh_at: "2026-06-22T00:00:00Z".to_owned(),
                native_surface_impersonation: false,
                required_visible_language: "Recording — screen and microphone".to_owned(),
            },
            vec![
                device_row(
                    DevicePermissionClass::ScreenCapture,
                    ProcessingClass::LocalOnly,
                    "Aureline session capture",
                    RetentionClass::LocalSessionOnly,
                    "Screen frames stay on this device and are discarded when the bundle is closed.",
                    RevokeActionClass::RevokeAndOpenSystemSettings,
                    "Stopping screen capture leaves the rest of the bundle intact.",
                    true,
                ),
                device_row(
                    DevicePermissionClass::Microphone,
                    ProcessingClass::MixedLocalThenProvider,
                    "Aureline session capture",
                    RetentionClass::ProviderRetained,
                    "Audio is transcribed; only the redacted transcript is retained by the provider.",
                    RevokeActionClass::OpenSystemSettings,
                    "Muting the microphone keeps the screen recording running.",
                    true,
                ),
            ],
            None,
            LocalContinuityClass::LocalWorkContinues,
        ),
        // 6. Browser auth handoff (claimed identity → system browser).
        card(
            "m5-embedded-dashboards:card:0006",
            "aureline://embedded/auth-handoff/0006",
            "Sign in with your identity provider",
            "A claimed-identity sign-in that hands off to the system browser; the card names why it \
             leaves, where it goes, and how the operator returns — never a bare Continue.",
            EmbeddedSurfaceKind::BrowserAuthHandoff,
            "identity-platform",
            "identity-platform owns the claimed-identity auth flow",
            ScopeClass::SharedTeam,
            RedactionClass::OperatorOnlyRestricted,
            vec![ShellUi, CliHeadless, CompanionBrowser, SupportExport],
            LiveSnapshotClass::SnapshotCapable,
            OriginBar {
                owner_class: OriginOwnerClass::ThirdPartyProvider,
                owner_label: "Identity provider sign-in".to_owned(),
                origin_ref: "aureline://embedded/origin/identity-signin".to_owned(),
                extension_ref: None,
                permission_state: PermissionStateClass::Sandboxed,
                capability_limitations: vec![limitation(
                    CapabilityLimitationClass::NoNativeApproval,
                    "Sign-in completes in the system browser, never as an embedded native prompt.",
                )],
                open_in_browser: OpenInBrowserAction {
                    available: true,
                    target: HandoffTargetClass::SystemBrowser,
                    label: "Continue sign-in in browser".to_owned(),
                },
                freshness: FreshnessClass::Fresh,
                latest_refresh_at: "2026-06-22T00:00:00Z".to_owned(),
                native_surface_impersonation: false,
                required_visible_language: "Opening your identity provider in the system browser"
                    .to_owned(),
            },
            vec![],
            Some(AuthHandoffCard {
                reason: HandoffReasonClass::ClaimedIdentityAuth,
                reason_note: "Claimed-identity sign-in must complete in the system browser so the \
                              session is attributable and not embedded."
                    .to_owned(),
                target: HandoffTargetClass::SystemBrowser,
                prefers_external: true,
                fallback: HandoffFallbackClass::SystemBrowserOpened,
                verification_code_shown: false,
                code_display_class: None,
                code_expiry_at: None,
                code_expired: false,
                return_path: "Return to this workspace automatically once the browser sign-in \
                              completes."
                    .to_owned(),
                return_anchor_ref: "aureline://embedded/auth-handoff/0006".to_owned(),
                hidden_behind_generic_continue: false,
            }),
            LocalContinuityClass::BlockedUntilAuth,
        ),
        // 7. Device-code auth handoff (provider auth via short code, polling).
        card(
            "m5-embedded-dashboards:card:0007",
            "aureline://embedded/auth-handoff/0007",
            "Authorize this device",
            "A provider device-code authorization: the card shows the code class and expiry, names \
             the polling fallback, and states the return path.",
            EmbeddedSurfaceKind::DeviceCodeAuthHandoff,
            "identity-platform",
            "identity-platform owns the device-code auth flow",
            ScopeClass::SharedTeam,
            RedactionClass::OperatorOnlyRestricted,
            vec![ShellUi, CliHeadless, CompanionBrowser, SupportExport],
            LiveSnapshotClass::SnapshotCapable,
            OriginBar {
                owner_class: OriginOwnerClass::ThirdPartyProvider,
                owner_label: "Provider device authorization".to_owned(),
                origin_ref: "aureline://embedded/origin/device-code".to_owned(),
                extension_ref: None,
                permission_state: PermissionStateClass::Sandboxed,
                capability_limitations: vec![limitation(
                    CapabilityLimitationClass::NoNativeApproval,
                    "Authorization happens on the provider's page; the device only shows a code.",
                )],
                open_in_browser: OpenInBrowserAction {
                    available: true,
                    target: HandoffTargetClass::DeviceCodeVerification,
                    label: "Open verification page in browser".to_owned(),
                },
                freshness: FreshnessClass::Fresh,
                latest_refresh_at: "2026-06-22T00:00:00Z".to_owned(),
                native_surface_impersonation: false,
                required_visible_language: "Enter the code on the provider's verification page"
                    .to_owned(),
            },
            vec![],
            Some(AuthHandoffCard {
                reason: HandoffReasonClass::ProviderAuthRequired,
                reason_note: "The provider authorizes this device through a verification page; \
                              Aureline only displays the short code."
                    .to_owned(),
                target: HandoffTargetClass::DeviceCodeVerification,
                prefers_external: true,
                fallback: HandoffFallbackClass::PollingForCompletion,
                verification_code_shown: true,
                code_display_class: Some("short_user_code".to_owned()),
                code_expiry_at: Some("2026-06-22T00:15:00Z".to_owned()),
                code_expired: false,
                return_path: "Aureline polls for completion and returns you here once the device \
                              is authorized."
                    .to_owned(),
                return_anchor_ref: "aureline://embedded/auth-handoff/0007".to_owned(),
                hidden_behind_generic_continue: false,
            }),
            LocalContinuityClass::BlockedUntilAuth,
        ),
        // 8. Expired device-code handoff (blocked; exercises the blocked path).
        card(
            "m5-embedded-dashboards:card:0008",
            "aureline://embedded/auth-handoff/0008",
            "Device code expired",
            "A device-code authorization whose code expired before use; the handoff is blocked, the \
             card says so, and it offers a fresh code rather than a silent retry.",
            EmbeddedSurfaceKind::DeviceCodeAuthHandoff,
            "identity-platform",
            "identity-platform owns the device-code auth flow",
            ScopeClass::SharedTeam,
            RedactionClass::OperatorOnlyRestricted,
            vec![ShellUi, CliHeadless, CompanionBrowser, SupportExport],
            LiveSnapshotClass::SnapshotCapable,
            OriginBar {
                owner_class: OriginOwnerClass::ThirdPartyProvider,
                owner_label: "Provider device authorization".to_owned(),
                origin_ref: "aureline://embedded/origin/device-code".to_owned(),
                extension_ref: None,
                permission_state: PermissionStateClass::Sandboxed,
                capability_limitations: vec![limitation(
                    CapabilityLimitationClass::NoNativeApproval,
                    "Authorization happens on the provider's page; the device only shows a code.",
                )],
                open_in_browser: OpenInBrowserAction {
                    available: true,
                    target: HandoffTargetClass::DeviceCodeVerification,
                    label: "Request a new code".to_owned(),
                },
                freshness: FreshnessClass::Fresh,
                latest_refresh_at: "2026-06-22T00:00:00Z".to_owned(),
                native_surface_impersonation: false,
                required_visible_language: "This code has expired — request a new one".to_owned(),
            },
            vec![],
            Some(AuthHandoffCard {
                reason: HandoffReasonClass::ProviderAuthRequired,
                reason_note: "The verification code expired before authorization completed."
                    .to_owned(),
                target: HandoffTargetClass::DeviceCodeVerification,
                prefers_external: true,
                fallback: HandoffFallbackClass::ManualCodeEntryAvailable,
                verification_code_shown: true,
                code_display_class: Some("short_user_code".to_owned()),
                code_expiry_at: Some("2026-06-21T23:50:00Z".to_owned()),
                code_expired: true,
                return_path: "Request a fresh code to retry; you stay on this card until you do."
                    .to_owned(),
                return_anchor_ref: "aureline://embedded/auth-handoff/0008".to_owned(),
                hidden_behind_generic_continue: false,
            }),
            LocalContinuityClass::BlockedUntilAuth,
        ),
    ]
}

// ---------------------------------------------------------------------------
// Invariants.
// ---------------------------------------------------------------------------

fn invariant(id: &str, statement: &str, holds: bool) -> EmbeddedInvariant {
    EmbeddedInvariant {
        invariant_id: id.to_owned(),
        statement: statement.to_owned(),
        holds,
    }
}

fn compute_invariants(surfaces: &[EmbeddedSurfaceCard]) -> Vec<EmbeddedInvariant> {
    let embedded = OperatorSurfaceClass::EmbeddedBoundaryState;
    let mut out = Vec::new();

    // Every card binds the embedded-boundary matrix family.
    out.push(invariant(
        "embedded_dashboards.surface_binding",
        "Every card binds the operator-surface matrix embedded-boundary family by that matrix's own \
         surface id, so embedded surfaces share one truth model rather than a parallel one.",
        surfaces
            .iter()
            .all(|c| c.surface == embedded && c.surface_id == embedded.surface_id()),
    ));

    // Canonical object identity.
    out.push(invariant(
        "embedded_dashboards.canonical_object_identity",
        "Every card points at one canonical aureline:// object and routes its open-details \
         affordance to the same object.",
        surfaces
            .iter()
            .all(|c| c.object_ref.starts_with("aureline://") && c.open_detail_ref == c.object_ref),
    ));

    // Owner / origin is always visible.
    out.push(invariant(
        "embedded_dashboards.origin_owner_visible",
        "Every card's origin bar names who owns and renders the content and an opaque origin handle, \
         so an operator can tell native chrome from provider/webview chrome.",
        surfaces.iter().all(|c| {
            !c.origin_bar.owner_label.is_empty()
                && c.origin_bar.origin_ref.starts_with("aureline://")
        }),
    ));

    // No native-surface impersonation.
    out.push(invariant(
        "embedded_dashboards.no_native_surface_impersonation",
        "No embedded surface impersonates a native approval, update, or product-security surface; \
         every card shows required visible language verbatim.",
        surfaces.iter().all(|c| {
            !c.origin_bar.native_surface_impersonation
                && !c.origin_bar.required_visible_language.is_empty()
        }),
    ));

    // Embedded webviews name capability limitations and an open-in-browser exit.
    out.push(invariant(
        "embedded_dashboards.capability_limitations_named",
        "Every embedded webview names at least one capability limitation including the \
         no-native-approval limitation, and every embedded page offers an open-in-browser exit.",
        surfaces.iter().all(|c| {
            let webview_ok = !c.kind.is_embedded_webview()
                || (!c.origin_bar.capability_limitations.is_empty()
                    && c.origin_bar
                        .capability_limitations
                        .iter()
                        .any(|l| l.class == CapabilityLimitationClass::NoNativeApproval));
            let browser_ok =
                !c.kind.requires_open_in_browser() || c.origin_bar.open_in_browser.available;
            webview_ok && browser_ok
        }),
    ));

    // Device-permission rows disclose processing, retention, revoke, continuity.
    out.push(invariant(
        "embedded_dashboards.device_permissions_disclose_truth",
        "Every device-permission row names the actor, the processing class, a storage/retention \
         note, a revoke / open-system-settings action, and the local-continuity posture.",
        surfaces.iter().all(|c| {
            c.device_permissions.iter().all(|r| {
                !r.actor.is_empty()
                    && !r.retention_note.is_empty()
                    && !r.local_continuity.is_empty()
                    && r.opens_system_settings == r.revoke_action.opens_system_settings()
            })
        }),
    ));

    // A device-capture surface actually carries device-permission rows.
    out.push(invariant(
        "embedded_dashboards.capture_surfaces_carry_permissions",
        "Every device-capture surface carries at least one device-permission row.",
        surfaces.iter().all(|c| {
            c.kind != EmbeddedSurfaceKind::DeviceCaptureSurface || !c.device_permissions.is_empty()
        }),
    ));

    // Handoffs make reason, target, and return path explicit.
    out.push(invariant(
        "embedded_dashboards.handoff_reason_and_return_visible",
        "Every auth handoff names its reason, prefers an external system-browser or device-code \
         target, and states a return path — never a generic Continue.",
        surfaces
            .iter()
            .all(|c| match (c.kind.is_auth_handoff(), &c.auth_handoff) {
                (true, Some(h)) => {
                    !h.reason_note.is_empty()
                        && h.prefers_external
                        && !h.hidden_behind_generic_continue
                        && !h.return_path.is_empty()
                        && h.return_anchor_ref.starts_with("aureline://")
                }
                (false, None) => true,
                _ => false,
            }),
    ));

    // Device-code handoffs show a code class and expiry; expired ones block.
    out.push(invariant(
        "embedded_dashboards.device_code_shows_code_and_expiry",
        "Every device-code handoff shows the code class and expiry, and an expired code is in the \
         blocked state rather than retried silently.",
        surfaces.iter().all(|c| {
            let is_device_code = c.kind == EmbeddedSurfaceKind::DeviceCodeAuthHandoff;
            match (is_device_code, &c.auth_handoff) {
                (true, Some(h)) => {
                    h.verification_code_shown
                        && h.code_display_class.as_ref().is_some_and(|s| !s.is_empty())
                        && h.code_expiry_at.is_some()
                        && (!h.code_expired || c.displayed_state == OperatorStateClass::Blocked)
                }
                (false, _) => true,
                _ => false,
            }
        }),
    ));

    // Effective state is the computed no-silent-green downgrade.
    out.push(invariant(
        "embedded_dashboards.effective_state_computed",
        "Every card's effective state is the computed no-silent-green downgrade of its displayed \
         state and the origin-bar freshness, so a stale embedded surface never reads as confirmed \
         clear.",
        surfaces.iter().all(|c| {
            c.effective_state
                == compute_effective_state(
                    c.displayed_state,
                    c.origin_bar.freshness,
                    BlockerWaiverClass::None,
                )
        }),
    ));

    // Origin and capability detail survive an export (the third acceptance
    // criterion): every card keeps a non-empty owner label and either a limitation
    // or a device-permission row to carry capability detail.
    out.push(invariant(
        "embedded_dashboards.origin_and_capability_exportable",
        "Every card carries non-empty origin and capability detail (owner label plus at least one \
         capability limitation or device-permission row), so a support/export packet never loses \
         origin or capability-limitation detail.",
        surfaces.iter().all(|c| {
            !c.origin_bar.owner_label.is_empty()
                && (!c.origin_bar.capability_limitations.is_empty()
                    || !c.device_permissions.is_empty())
        }),
    ));

    // Every surface kind is exercised.
    out.push(invariant(
        "embedded_dashboards.all_kinds_present",
        "Every embedded-surface kind — service dashboard, provider page, device capture, browser \
         auth handoff, and device-code auth handoff — is present in the set.",
        EmbeddedSurfaceKind::ALL
            .iter()
            .all(|kind| surfaces.iter().any(|c| c.kind == *kind)),
    ));

    // Stable ids unique.
    out.push(invariant(
        "embedded_dashboards.stable_ids_unique",
        "Card ids are unique, so a consumer can resolve an embedded surface by a stable id.",
        all_unique(surfaces.iter().map(|c| c.card_id.as_str())),
    ));

    out
}

// ---------------------------------------------------------------------------
// Human-readable projection.
// ---------------------------------------------------------------------------

/// Renders the set as human-readable lines for CLI/headless and support.
pub fn embedded_surface_lines(set: &EmbeddedSurfaceSet) -> Vec<String> {
    let mut lines = Vec::new();
    lines.push(format!(
        "Embedded service dashboards & auth handoffs — {} ({})",
        set.set_id, set.as_of
    ));
    lines.push(set.summary.clone());
    lines.push(format!(
        "Surfaces: {}  Invariants: {}",
        set.surfaces.len(),
        set.invariants.len()
    ));

    for c in &set.surfaces {
        lines.push(format!(
            "  - {} [{}] kind={} owner={} state={} (displayed {})",
            c.card_id,
            c.title,
            c.kind.as_str(),
            c.origin_bar.owner_class.as_str(),
            c.effective_state.as_str(),
            c.displayed_state.as_str(),
        ));
        lines.push(format!(
            "      origin: {}",
            c.origin_bar.required_visible_language
        ));
        if !c.origin_bar.capability_limitations.is_empty() {
            let limits: Vec<&str> = c
                .origin_bar
                .capability_limitations
                .iter()
                .map(|l| l.class.as_str())
                .collect();
            lines.push(format!("      limitations: {}", limits.join(", ")));
        }
        if c.origin_bar.open_in_browser.available {
            lines.push(format!(
                "      open-in-browser: {} → {}",
                c.origin_bar.open_in_browser.label,
                c.origin_bar.open_in_browser.target.as_str()
            ));
        }
        for r in &c.device_permissions {
            lines.push(format!(
                "      device: {} actor={} processing={} retention={} revoke={}",
                r.permission.as_str(),
                r.actor,
                r.processing_class.as_str(),
                r.retention_class.as_str(),
                r.revoke_action.as_str(),
            ));
        }
        if let Some(h) = &c.auth_handoff {
            lines.push(format!(
                "      handoff: reason={} target={} fallback={} code_shown={} expired={}",
                h.reason.as_str(),
                h.target.as_str(),
                h.fallback.as_str(),
                h.verification_code_shown,
                h.code_expired,
            ));
            lines.push(format!("      return: {}", h.return_path));
        }
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
