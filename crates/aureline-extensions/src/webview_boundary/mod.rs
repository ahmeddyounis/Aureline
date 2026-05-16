//! Extension webview and hosted-surface boundary audit.
//!
//! This module owns the extension-specific beta packet that proves an
//! extension-contributed embedded surface keeps the host chrome, browser
//! handoff posture, and support-export truth aligned. The packet is narrower
//! than the shell-wide embedded-boundary page: it focuses on extension-owned
//! webviews, hosted dashboards, provider-auth checkpoints, documentation panes,
//! and browser-runtime bridges.
//!
//! One [`ExtensionWebviewBoundaryAuditPacket`] is the inspectable answer to
//! "does this extension surface disclose owner, publisher, origin, permission
//! state, trust class, and handoff posture without delegating native approvals
//! to embedded content?". The first consumers are a headless dump command,
//! the metadata-safe [`ExtensionWebviewBoundarySupportExport`], and the beta
//! docs page at
//! [`/docs/extensions/m3/webview_boundary_beta.md`](../../../../docs/extensions/m3/webview_boundary_beta.md).
//!
//! The cross-tool schema is
//! [`/schemas/extensions/webview_boundary_audit.schema.json`](../../../../schemas/extensions/webview_boundary_audit.schema.json),
//! and the checked fixture corpus lives under
//! [`/fixtures/extensions/m3/webview_boundary_audit/`](../../../../fixtures/extensions/m3/webview_boundary_audit/).

use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

#[cfg(test)]
mod tests;

/// Record-kind tag carried by [`ExtensionWebviewBoundaryAuditPacket`].
pub const EXTENSION_WEBVIEW_BOUNDARY_AUDIT_PACKET_RECORD_KIND: &str =
    "extension_webview_boundary_audit_packet";

/// Record-kind tag carried by [`ExtensionWebviewBoundaryRow`].
pub const EXTENSION_WEBVIEW_BOUNDARY_ROW_RECORD_KIND: &str = "extension_webview_boundary_row";

/// Record-kind tag carried by [`ExtensionWebviewBoundarySupportRow`].
pub const EXTENSION_WEBVIEW_BOUNDARY_SUPPORT_ROW_RECORD_KIND: &str =
    "extension_webview_boundary_support_row";

/// Record-kind tag carried by [`ExtensionWebviewBoundaryDefect`].
pub const EXTENSION_WEBVIEW_BOUNDARY_DEFECT_RECORD_KIND: &str = "extension_webview_boundary_defect";

/// Record-kind tag carried by [`ExtensionWebviewBoundarySupportExport`].
pub const EXTENSION_WEBVIEW_BOUNDARY_SUPPORT_EXPORT_RECORD_KIND: &str =
    "extension_webview_boundary_support_export";

/// Schema version for extension webview boundary payloads.
pub const EXTENSION_WEBVIEW_BOUNDARY_SCHEMA_VERSION: u32 = 1;

/// Shared contract ref used by page rows, support rows, docs, and artifacts.
pub const EXTENSION_WEBVIEW_BOUNDARY_SHARED_CONTRACT_REF: &str =
    "extensions:webview_boundary_beta:v1";

/// Extension-owned embedded surface families covered by the audit.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ExtensionEmbeddedSurfaceClass {
    /// Extension-rendered local or bundled webview panel.
    ExtensionWebviewPanel,
    /// Extension-rendered hosted dashboard or service page.
    HostedDashboard,
    /// Provider-owned authentication checkpoint or confirmation surface.
    ProviderAuthSurface,
    /// Browser-runtime adapter, DOM/CSS inspector, or mobile-webview bridge.
    BrowserRuntimeBridge,
    /// Extension-contributed documentation provider pane.
    DocumentationProviderPane,
}

impl ExtensionEmbeddedSurfaceClass {
    /// Returns the stable schema token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ExtensionWebviewPanel => "extension_webview_panel",
            Self::HostedDashboard => "hosted_dashboard",
            Self::ProviderAuthSurface => "provider_auth_surface",
            Self::BrowserRuntimeBridge => "browser_runtime_bridge",
            Self::DocumentationProviderPane => "documentation_provider_pane",
        }
    }
}

/// Origin family used by an extension embedded surface.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ExtensionOriginClass {
    /// Content served from signed extension-local assets.
    ExtensionLocalAsset,
    /// Content served by the extension publisher or its hosted service.
    PublisherHostedOrigin,
    /// A cross-origin frame or subresource limits host inspection.
    CrossOriginSubframe,
    /// Provider-owned auth domain used for sign-in or step-up.
    ProviderOwnedAuthDomain,
    /// Browser, simulator, device, or runtime origin inspected through a bridge.
    RuntimeTargetOrigin,
}

impl ExtensionOriginClass {
    /// Returns the stable schema token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ExtensionLocalAsset => "extension_local_asset",
            Self::PublisherHostedOrigin => "publisher_hosted_origin",
            Self::CrossOriginSubframe => "cross_origin_subframe",
            Self::ProviderOwnedAuthDomain => "provider_owned_auth_domain",
            Self::RuntimeTargetOrigin => "runtime_target_origin",
        }
    }
}

/// Boundary state rendered in host chrome.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ExtensionBoundaryStateClass {
    /// Origin and embedded content are current and verified for the row.
    LiveVerified,
    /// Cross-origin rules limit host or extension inspection.
    CrossOriginLimited,
    /// A stale snapshot is visible while the live surface is unavailable.
    StaleSnapshot,
    /// Policy blocks rendering or external navigation.
    PolicyBlocked,
    /// Certificate or origin verification failed.
    CertificateFailed,
}

impl ExtensionBoundaryStateClass {
    /// Returns the stable schema token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LiveVerified => "live_verified",
            Self::CrossOriginLimited => "cross_origin_limited",
            Self::StaleSnapshot => "stale_snapshot",
            Self::PolicyBlocked => "policy_blocked",
            Self::CertificateFailed => "certificate_failed",
        }
    }
}

/// Trust class rendered by the host and quoted by support exports.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ExtensionSurfaceTrustClass {
    /// Trusted under the current extension, origin, and policy evidence.
    Trusted,
    /// Usable only with visible capability or origin limits.
    Limited,
    /// Stale evidence is visible but not treated as live truth.
    Stale,
    /// Policy blocks the surface or its external path.
    PolicyBlocked,
    /// Origin or publisher evidence is not trusted.
    Untrusted,
}

impl ExtensionSurfaceTrustClass {
    /// Returns the stable schema token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Trusted => "trusted",
            Self::Limited => "limited",
            Self::Stale => "stale",
            Self::PolicyBlocked => "policy_blocked",
            Self::Untrusted => "untrusted",
        }
    }
}

/// Permission posture rendered in the owner-origin chrome.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ExtensionSurfacePermissionClass {
    /// Host controls inspection and the embedded body cannot mutate state.
    HostOwnedInspectOnly,
    /// Host only permits browser handoff or read-only browser actions.
    HostOwnedBrowserOnly,
    /// Host requires a native step-up before privileged action.
    HostOwnedWithNativeStepUpRequired,
    /// Extension may request mutations, but host review owns confirmation.
    ExtensionMutableWithHostReview,
}

impl ExtensionSurfacePermissionClass {
    /// Returns the stable schema token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::HostOwnedInspectOnly => "host_owned_inspect_only",
            Self::HostOwnedBrowserOnly => "host_owned_browser_only",
            Self::HostOwnedWithNativeStepUpRequired => "host_owned_with_native_step_up_required",
            Self::ExtensionMutableWithHostReview => "extension_mutable_with_host_review",
        }
    }
}

/// Browser-handoff posture rendered for the surface.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ExtensionBrowserHandoffPostureClass {
    /// System browser is the default path for risky or provider-owned content.
    SystemBrowserFirst,
    /// Device-code flow is offered when browser launch is unavailable.
    DeviceCodeFallbackOffered,
    /// External open exists conceptually but policy blocks it.
    ExternalOpenBlockedByPolicy,
    /// External open is unavailable while the client is offline.
    ExternalOpenUnavailableOffline,
    /// Surface is in-product only and not risky or provider-owned.
    InProductOnly,
}

impl ExtensionBrowserHandoffPostureClass {
    /// Returns the stable schema token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SystemBrowserFirst => "system_browser_first",
            Self::DeviceCodeFallbackOffered => "device_code_fallback_offered",
            Self::ExternalOpenBlockedByPolicy => "external_open_blocked_by_policy",
            Self::ExternalOpenUnavailableOffline => "external_open_unavailable_offline",
            Self::InProductOnly => "in_product_only",
        }
    }
}

/// External fallback target used by a handoff posture.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ExtensionFallbackTargetClass {
    /// A typed packet opens the system browser.
    SystemBrowserHandoffPacket,
    /// A typed device-code flow is the fallback.
    DeviceCodeFlow,
    /// Policy review is the only allowed next step.
    PolicyReview,
    /// No external handoff is exposed.
    NoExternalHandoff,
}

impl ExtensionFallbackTargetClass {
    /// Returns the stable schema token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SystemBrowserHandoffPacket => "system_browser_handoff_packet",
            Self::DeviceCodeFlow => "device_code_flow",
            Self::PolicyReview => "policy_review",
            Self::NoExternalHandoff => "no_external_handoff",
        }
    }
}

/// Reason shown when the surface offers or blocks external browser handoff.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ExtensionBrowserHandoffReasonClass {
    /// Cross-origin active content cannot be inspected safely in product.
    CrossOriginActiveContent,
    /// Provider-owned authentication must stay in browser or device-code flow.
    ProviderOwnedAuth,
    /// Hosted dashboard authority is outside the local product boundary.
    HostedDashboardAuthority,
    /// Runtime protocol limitations require a browser or device target.
    BrowserRuntimeProtocolLimit,
    /// Provider documentation needs an external source or active site.
    DocumentationProviderExternal,
    /// No handoff reason applies.
    NotApplicable,
}

impl ExtensionBrowserHandoffReasonClass {
    /// Returns the stable schema token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::CrossOriginActiveContent => "cross_origin_active_content",
            Self::ProviderOwnedAuth => "provider_owned_auth",
            Self::HostedDashboardAuthority => "hosted_dashboard_authority",
            Self::BrowserRuntimeProtocolLimit => "browser_runtime_protocol_limit",
            Self::DocumentationProviderExternal => "documentation_provider_external",
            Self::NotApplicable => "not_applicable",
        }
    }
}

/// Host chrome controls that must stay outside the embedded body.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ExtensionHostChromeControlClass {
    /// Host chrome renders the extension name.
    ExtensionName,
    /// Host chrome renders the publisher label.
    PublisherLabel,
    /// Host chrome renders the origin label or host.
    OriginLabel,
    /// Host chrome renders the boundary state.
    BoundaryState,
    /// Host chrome renders the trust class.
    TrustClass,
    /// Host chrome renders the permission state.
    PermissionState,
    /// Host chrome renders reload or retry control.
    Reload,
    /// Host chrome renders external browser handoff or its blocked state.
    OpenInBrowser,
    /// Host chrome renders support/export affordance or evidence link.
    SupportExport,
    /// Host chrome renders profile, tenant, workspace, or target scope.
    ProfileOrTargetScope,
}

impl ExtensionHostChromeControlClass {
    /// Returns the stable schema token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ExtensionName => "extension_name",
            Self::PublisherLabel => "publisher_label",
            Self::OriginLabel => "origin_label",
            Self::BoundaryState => "boundary_state",
            Self::TrustClass => "trust_class",
            Self::PermissionState => "permission_state",
            Self::Reload => "reload",
            Self::OpenInBrowser => "open_in_browser",
            Self::SupportExport => "support_export",
            Self::ProfileOrTargetScope => "profile_or_target_scope",
        }
    }
}

/// Visual and accessibility inheritance disclosure for extension webviews.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ExtensionInheritanceClass {
    /// The surface inherits this host axis.
    Inherits,
    /// The surface partially inherits and names limitations elsewhere.
    Partial,
    /// The surface does not inherit and must label the gap.
    DoesNotInherit,
    /// The surface failed to disclose this axis.
    NotDisclosed,
}

impl ExtensionInheritanceClass {
    /// Returns the stable schema token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Inherits => "inherits",
            Self::Partial => "partial",
            Self::DoesNotInherit => "does_not_inherit",
            Self::NotDisclosed => "not_disclosed",
        }
    }
}

/// Disclosure set for theme, zoom, density, focus, and motion parity.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExtensionAppearanceInheritance {
    /// Theme/color inheritance disclosure.
    pub theme_class: ExtensionInheritanceClass,
    /// Zoom level inheritance disclosure.
    pub zoom_class: ExtensionInheritanceClass,
    /// Density inheritance disclosure.
    pub density_class: ExtensionInheritanceClass,
    /// Keyboard focus and focus-ring inheritance disclosure.
    pub focus_class: ExtensionInheritanceClass,
    /// Reduced-motion inheritance disclosure.
    pub reduced_motion_class: ExtensionInheritanceClass,
    /// High-contrast or forced-colors inheritance disclosure.
    pub contrast_class: ExtensionInheritanceClass,
}

impl ExtensionAppearanceInheritance {
    /// Returns `true` when every required inheritance axis is disclosed.
    pub fn all_axes_disclosed(&self) -> bool {
        [
            self.theme_class,
            self.zoom_class,
            self.density_class,
            self.focus_class,
            self.reduced_motion_class,
            self.contrast_class,
        ]
        .iter()
        .all(|class| *class != ExtensionInheritanceClass::NotDisclosed)
    }
}

/// Location of high-risk approval authority for an extension webview.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ExtensionNativeApprovalBoundaryClass {
    /// High-risk approvals stay on host-native surfaces.
    HostOwnedNativeSurface,
    /// Embedded content attempted to mint a native approval equivalent.
    EmbeddedSurfaceAttemptedApproval,
}

impl ExtensionNativeApprovalBoundaryClass {
    /// Returns the stable schema token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::HostOwnedNativeSurface => "host_owned_native_surface",
            Self::EmbeddedSurfaceAttemptedApproval => "embedded_surface_attempted_approval",
        }
    }
}

/// Host authority scope declared for the surface.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ExtensionHostAuthorityScopeClass {
    /// No host authority is delegated to the embedded content.
    NoHostAuthority,
    /// Host authority is declared and bounded by the manifest and row.
    DeclaredBoundedHostAuthority,
    /// Host authority is broad or unbounded.
    UnboundedHostAuthority,
}

impl ExtensionHostAuthorityScopeClass {
    /// Returns the stable schema token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NoHostAuthority => "no_host_authority",
            Self::DeclaredBoundedHostAuthority => "declared_bounded_host_authority",
            Self::UnboundedHostAuthority => "unbounded_host_authority",
        }
    }
}

/// Row-level decision after validation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ExtensionWebviewBoundaryDecisionClass {
    /// Row is acceptable for the claimed beta surface.
    Conformant,
    /// Row is visible but must be reviewed before broadening support.
    NeedsReview,
    /// Row fails the boundary audit and must not be claimed.
    Refused,
}

impl ExtensionWebviewBoundaryDecisionClass {
    /// Returns the stable schema token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Conformant => "conformant",
            Self::NeedsReview => "needs_review",
            Self::Refused => "refused",
        }
    }
}

/// Defect vocabulary emitted by the audit validator.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ExtensionWebviewBoundaryDefectKind {
    /// Owner, publisher, or origin chrome is missing.
    OwnerOriginChromeMissing,
    /// Required host chrome control is missing.
    HostChromeControlMissing,
    /// Permission state is missing or not exposed in host chrome.
    PermissionStateMissing,
    /// Host trust class and embedded-content trust claim disagree.
    TrustClassParityDrift,
    /// Risky or provider-owned content did not keep a safe browser baseline.
    RiskySurfaceWithoutSafeBrowserBaseline,
    /// System-browser-first row is missing a handoff packet reference.
    BrowserHandoffPacketMissing,
    /// Embedded content attempted to own a high-risk native approval.
    EmbeddedNativeApprovalAttempt,
    /// Host authority is unbounded.
    UnboundedHostAuthority,
    /// Appearance or accessibility inheritance disclosure is missing.
    AppearanceInheritanceDisclosureMissing,
    /// Support export row drifted from the product row.
    SupportExportParityDrift,
    /// Support export contains raw private material.
    RawPrivateMaterialExported,
}

impl ExtensionWebviewBoundaryDefectKind {
    /// Returns the stable schema token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::OwnerOriginChromeMissing => "owner_origin_chrome_missing",
            Self::HostChromeControlMissing => "host_chrome_control_missing",
            Self::PermissionStateMissing => "permission_state_missing",
            Self::TrustClassParityDrift => "trust_class_parity_drift",
            Self::RiskySurfaceWithoutSafeBrowserBaseline => {
                "risky_surface_without_safe_browser_baseline"
            }
            Self::BrowserHandoffPacketMissing => "browser_handoff_packet_missing",
            Self::EmbeddedNativeApprovalAttempt => "embedded_native_approval_attempt",
            Self::UnboundedHostAuthority => "unbounded_host_authority",
            Self::AppearanceInheritanceDisclosureMissing => {
                "appearance_inheritance_disclosure_missing"
            }
            Self::SupportExportParityDrift => "support_export_parity_drift",
            Self::RawPrivateMaterialExported => "raw_private_material_exported",
        }
    }
}

/// Input row supplied by extension host, SDK, or fixture code.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExtensionWebviewBoundaryInput {
    /// Stable row id.
    pub row_id: String,
    /// Stable extension id.
    pub extension_id: String,
    /// Human-readable extension name rendered in host chrome.
    pub extension_name: String,
    /// Human-readable publisher label rendered in host chrome.
    pub publisher_label: String,
    /// Stable embedded surface id.
    pub surface_id: String,
    /// Human-readable embedded surface label.
    pub surface_label: String,
    /// Surface family covered by the row.
    pub surface_class: ExtensionEmbeddedSurfaceClass,
    /// Origin label shown in host chrome.
    pub origin_label: String,
    /// Host or domain label shown in host chrome.
    pub origin_host_label: String,
    /// Origin family for the embedded content.
    pub origin_class: ExtensionOriginClass,
    /// Boundary state shown in host chrome.
    pub boundary_state_class: ExtensionBoundaryStateClass,
    /// Trust class rendered by host chrome.
    pub host_chrome_trust_class: ExtensionSurfaceTrustClass,
    /// Trust class claimed by the embedded body or extension metadata.
    pub embedded_content_trust_class: ExtensionSurfaceTrustClass,
    /// Permission posture shown in host chrome.
    pub permission_state_class: ExtensionSurfacePermissionClass,
    /// Browser handoff posture for this surface.
    pub browser_handoff_posture_class: ExtensionBrowserHandoffPostureClass,
    /// Target class for external fallback.
    pub fallback_target_class: ExtensionFallbackTargetClass,
    /// Reason attached to the handoff posture.
    pub browser_handoff_reason_class: ExtensionBrowserHandoffReasonClass,
    /// Optional typed browser handoff packet ref.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub browser_handoff_packet_ref: Option<String>,
    /// Profile, tenant, workspace, or runtime target scope shown in chrome.
    pub current_scope_label: String,
    /// Controls rendered by host chrome instead of embedded content.
    pub host_chrome_controls: Vec<ExtensionHostChromeControlClass>,
    /// Appearance and accessibility inheritance disclosure.
    pub appearance_inheritance: ExtensionAppearanceInheritance,
    /// Location of native approval authority.
    pub native_approval_boundary_class: ExtensionNativeApprovalBoundaryClass,
    /// Host authority scope for the embedded content.
    pub host_authority_scope_class: ExtensionHostAuthorityScopeClass,
    /// True when active content, credentials, mutation, or provider authority is present.
    pub risky_content_flow: bool,
    /// Permission manifest or effective-permission refs cited by the row.
    #[serde(default)]
    pub permission_evidence_refs: Vec<String>,
    /// Runtime, registry, policy, or support refs cited by the row.
    #[serde(default)]
    pub evidence_refs: Vec<String>,
    /// Human-readable summary safe for product chrome and support export.
    pub support_boundary_summary: String,
    /// Timestamp used by generated row fixtures.
    pub generated_at: String,
}

/// Audited product row for one extension embedded surface.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExtensionWebviewBoundaryRow {
    /// Discriminator for this record family.
    pub record_kind: String,
    /// Schema version for the row.
    pub schema_version: u32,
    /// Shared contract ref used by every row and export.
    pub shared_contract_ref: String,
    /// Stable row id.
    pub row_id: String,
    /// Stable extension id.
    pub extension_id: String,
    /// Human-readable extension name rendered in host chrome.
    pub extension_name: String,
    /// Human-readable publisher label rendered in host chrome.
    pub publisher_label: String,
    /// Stable embedded surface id.
    pub surface_id: String,
    /// Human-readable embedded surface label.
    pub surface_label: String,
    /// Surface family covered by the row.
    pub surface_class: ExtensionEmbeddedSurfaceClass,
    /// Origin label shown in host chrome.
    pub origin_label: String,
    /// Host or domain label shown in host chrome.
    pub origin_host_label: String,
    /// Origin family for the embedded content.
    pub origin_class: ExtensionOriginClass,
    /// Boundary state shown in host chrome.
    pub boundary_state_class: ExtensionBoundaryStateClass,
    /// Trust class rendered by host chrome.
    pub host_chrome_trust_class: ExtensionSurfaceTrustClass,
    /// Trust class claimed by the embedded body or extension metadata.
    pub embedded_content_trust_class: ExtensionSurfaceTrustClass,
    /// Permission posture shown in host chrome.
    pub permission_state_class: ExtensionSurfacePermissionClass,
    /// Browser handoff posture for this surface.
    pub browser_handoff_posture_class: ExtensionBrowserHandoffPostureClass,
    /// Target class for external fallback.
    pub fallback_target_class: ExtensionFallbackTargetClass,
    /// Reason attached to the handoff posture.
    pub browser_handoff_reason_class: ExtensionBrowserHandoffReasonClass,
    /// Optional typed browser handoff packet ref.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub browser_handoff_packet_ref: Option<String>,
    /// Profile, tenant, workspace, or runtime target scope shown in chrome.
    pub current_scope_label: String,
    /// Controls rendered by host chrome instead of embedded content.
    pub host_chrome_controls: Vec<ExtensionHostChromeControlClass>,
    /// Appearance and accessibility inheritance disclosure.
    pub appearance_inheritance: ExtensionAppearanceInheritance,
    /// Location of native approval authority.
    pub native_approval_boundary_class: ExtensionNativeApprovalBoundaryClass,
    /// Host authority scope for the embedded content.
    pub host_authority_scope_class: ExtensionHostAuthorityScopeClass,
    /// True when active content, credentials, mutation, or provider authority is present.
    pub risky_content_flow: bool,
    /// True when safe browser/device/policy fallback is required.
    pub safe_browser_baseline_required: bool,
    /// True when the required safe browser/device/policy fallback is present.
    pub safe_browser_baseline_satisfied: bool,
    /// Permission manifest or effective-permission refs cited by the row.
    pub permission_evidence_refs: Vec<String>,
    /// Runtime, registry, policy, or support refs cited by the row.
    pub evidence_refs: Vec<String>,
    /// Product-visible boundary finding refs mirrored into support export.
    pub visible_boundary_finding_refs: Vec<String>,
    /// Human-readable summary safe for product chrome and support export.
    pub support_boundary_summary: String,
    /// Row-level decision after validation.
    pub decision_class: ExtensionWebviewBoundaryDecisionClass,
    /// Defect kind tokens found on this row before support-export parity checks.
    pub row_defect_kind_tokens: Vec<String>,
    /// Timestamp used by generated row fixtures.
    pub generated_at: String,
}

/// Export-safe support row paired with an audited product row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExtensionWebviewBoundarySupportRow {
    /// Discriminator for this record family.
    pub record_kind: String,
    /// Schema version for the support row.
    pub schema_version: u32,
    /// Shared contract ref used by every row and export.
    pub shared_contract_ref: String,
    /// Stable support row id.
    pub support_row_id: String,
    /// Product row this support row mirrors.
    pub row_ref: String,
    /// Stable extension id.
    pub extension_id: String,
    /// Human-readable extension name rendered in host chrome.
    pub extension_name: String,
    /// Human-readable publisher label rendered in host chrome.
    pub publisher_label: String,
    /// Human-readable embedded surface label.
    pub surface_label: String,
    /// Surface family covered by the row.
    pub surface_class: ExtensionEmbeddedSurfaceClass,
    /// Host or domain label shown in host chrome.
    pub origin_host_label: String,
    /// Boundary state shown in host chrome.
    pub boundary_state_class: ExtensionBoundaryStateClass,
    /// Trust class rendered by host chrome.
    pub host_chrome_trust_class: ExtensionSurfaceTrustClass,
    /// Permission posture shown in host chrome.
    pub permission_state_class: ExtensionSurfacePermissionClass,
    /// Browser handoff posture for this surface.
    pub browser_handoff_posture_class: ExtensionBrowserHandoffPostureClass,
    /// Target class for external fallback.
    pub fallback_target_class: ExtensionFallbackTargetClass,
    /// Optional typed browser handoff packet ref.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub browser_handoff_packet_ref: Option<String>,
    /// Profile, tenant, workspace, or runtime target scope shown in chrome.
    pub current_scope_label: String,
    /// Product-visible boundary finding refs mirrored from the product row.
    pub visible_boundary_finding_refs: Vec<String>,
    /// Defect kind tokens mirrored from the product row.
    pub row_defect_kind_tokens: Vec<String>,
    /// Export-safe summary.
    pub support_boundary_summary: String,
}

/// Typed defect emitted by extension webview boundary validation.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExtensionWebviewBoundaryDefect {
    /// Discriminator for this record family.
    pub record_kind: String,
    /// Schema version for this defect.
    pub schema_version: u32,
    /// Shared contract ref used by every row and export.
    pub shared_contract_ref: String,
    /// Stable defect id.
    pub defect_id: String,
    /// Defect kind from the closed vocabulary.
    pub defect_kind: ExtensionWebviewBoundaryDefectKind,
    /// Row that emitted the defect.
    pub row_ref: String,
    /// Field or group that failed validation.
    pub field: String,
    /// Export-safe validation message.
    pub message: String,
    /// True when the product row can show the same defect.
    pub visible_in_product: bool,
    /// True when the defect can be included in support export.
    pub support_export_safe: bool,
}

impl ExtensionWebviewBoundaryDefect {
    fn new(
        row_ref: impl Into<String>,
        defect_kind: ExtensionWebviewBoundaryDefectKind,
        field: impl Into<String>,
        message: impl Into<String>,
    ) -> Self {
        let row_ref = row_ref.into();
        Self {
            record_kind: EXTENSION_WEBVIEW_BOUNDARY_DEFECT_RECORD_KIND.to_owned(),
            schema_version: EXTENSION_WEBVIEW_BOUNDARY_SCHEMA_VERSION,
            shared_contract_ref: EXTENSION_WEBVIEW_BOUNDARY_SHARED_CONTRACT_REF.to_owned(),
            defect_id: format!(
                "extension-webview-boundary:defect:{}:{}",
                defect_kind.as_str(),
                row_ref
            ),
            defect_kind,
            row_ref,
            field: field.into(),
            message: message.into(),
            visible_in_product: true,
            support_export_safe: true,
        }
    }
}

/// Aggregate summary for an audit packet.
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct ExtensionWebviewBoundarySummary {
    /// Number of audited product rows.
    pub row_count: usize,
    /// Number of support rows.
    pub support_row_count: usize,
    /// Number of rows whose decision is conformant.
    pub conformant_row_count: usize,
    /// Number of rows whose decision is needs-review.
    pub needs_review_row_count: usize,
    /// Number of rows whose decision is refused.
    pub refused_row_count: usize,
    /// Number of rows requiring a safe browser baseline.
    pub safe_browser_baseline_required_row_count: usize,
    /// Number of required rows satisfying the safe baseline.
    pub safe_browser_baseline_satisfied_row_count: usize,
    /// Number of rows with matching host and embedded trust classes.
    pub trust_class_parity_row_count: usize,
    /// Number of rows with host-owned native approval authority.
    pub host_owned_native_approval_row_count: usize,
    /// Number of emitted validation defects.
    pub defect_count: usize,
    /// Surface families present in the packet.
    pub surface_classes_present: Vec<ExtensionEmbeddedSurfaceClass>,
    /// Boundary states present in the packet.
    pub boundary_states_present: Vec<ExtensionBoundaryStateClass>,
    /// Handoff postures present in the packet.
    pub handoff_postures_present: Vec<ExtensionBrowserHandoffPostureClass>,
}

impl ExtensionWebviewBoundarySummary {
    fn from_rows(
        rows: &[ExtensionWebviewBoundaryRow],
        support_rows: &[ExtensionWebviewBoundarySupportRow],
        defects: &[ExtensionWebviewBoundaryDefect],
    ) -> Self {
        let mut surface_classes_present = Vec::new();
        let mut boundary_states_present = Vec::new();
        let mut handoff_postures_present = Vec::new();
        for row in rows {
            push_unique(&mut surface_classes_present, row.surface_class);
            push_unique(&mut boundary_states_present, row.boundary_state_class);
            push_unique(
                &mut handoff_postures_present,
                row.browser_handoff_posture_class,
            );
        }

        Self {
            row_count: rows.len(),
            support_row_count: support_rows.len(),
            conformant_row_count: rows
                .iter()
                .filter(|row| {
                    row.decision_class == ExtensionWebviewBoundaryDecisionClass::Conformant
                })
                .count(),
            needs_review_row_count: rows
                .iter()
                .filter(|row| {
                    row.decision_class == ExtensionWebviewBoundaryDecisionClass::NeedsReview
                })
                .count(),
            refused_row_count: rows
                .iter()
                .filter(|row| row.decision_class == ExtensionWebviewBoundaryDecisionClass::Refused)
                .count(),
            safe_browser_baseline_required_row_count: rows
                .iter()
                .filter(|row| row.safe_browser_baseline_required)
                .count(),
            safe_browser_baseline_satisfied_row_count: rows
                .iter()
                .filter(|row| {
                    row.safe_browser_baseline_required && row.safe_browser_baseline_satisfied
                })
                .count(),
            trust_class_parity_row_count: rows
                .iter()
                .filter(|row| row.host_chrome_trust_class == row.embedded_content_trust_class)
                .count(),
            host_owned_native_approval_row_count: rows
                .iter()
                .filter(|row| {
                    row.native_approval_boundary_class
                        == ExtensionNativeApprovalBoundaryClass::HostOwnedNativeSurface
                })
                .count(),
            defect_count: defects.len(),
            surface_classes_present,
            boundary_states_present,
            handoff_postures_present,
        }
    }
}

/// Top-level extension webview boundary audit packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExtensionWebviewBoundaryAuditPacket {
    /// Discriminator for this record family.
    pub record_kind: String,
    /// Schema version for this packet.
    pub schema_version: u32,
    /// Shared contract ref used by every row and export.
    pub shared_contract_ref: String,
    /// Stable audit id.
    pub audit_id: String,
    /// Timestamp used by generated packet fixtures.
    pub generated_at: String,
    /// Reviewer-facing docs page.
    pub docs_ref: String,
    /// Cross-tool JSON schema ref.
    pub schema_ref: String,
    /// Human-readable generated report ref.
    pub report_ref: String,
    /// Aggregate summary.
    pub summary: ExtensionWebviewBoundarySummary,
    /// Audited product rows.
    pub rows: Vec<ExtensionWebviewBoundaryRow>,
    /// Support rows paired with the product rows.
    pub support_rows: Vec<ExtensionWebviewBoundarySupportRow>,
    /// Validation defects emitted by the packet.
    pub defects: Vec<ExtensionWebviewBoundaryDefect>,
}

impl ExtensionWebviewBoundaryAuditPacket {
    /// Builds a packet from already evaluated rows.
    pub fn from_rows(
        audit_id: impl Into<String>,
        generated_at: impl Into<String>,
        rows: Vec<ExtensionWebviewBoundaryRow>,
    ) -> Self {
        let support_rows: Vec<ExtensionWebviewBoundarySupportRow> = rows
            .iter()
            .map(project_extension_webview_boundary_support_row)
            .collect();
        let defects = audit_extension_webview_boundary_rows(&rows, &support_rows);
        let summary = ExtensionWebviewBoundarySummary::from_rows(&rows, &support_rows, &defects);
        Self {
            record_kind: EXTENSION_WEBVIEW_BOUNDARY_AUDIT_PACKET_RECORD_KIND.to_owned(),
            schema_version: EXTENSION_WEBVIEW_BOUNDARY_SCHEMA_VERSION,
            shared_contract_ref: EXTENSION_WEBVIEW_BOUNDARY_SHARED_CONTRACT_REF.to_owned(),
            audit_id: audit_id.into(),
            generated_at: generated_at.into(),
            docs_ref: "docs/extensions/m3/webview_boundary_beta.md".to_owned(),
            schema_ref: "schemas/extensions/webview_boundary_audit.schema.json".to_owned(),
            report_ref: "artifacts/extensions/m3/webview_boundary_report.md".to_owned(),
            summary,
            rows,
            support_rows,
            defects,
        }
    }
}

/// Metadata-safe export projected from an audit packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExtensionWebviewBoundarySupportExport {
    /// Discriminator for this record family.
    pub record_kind: String,
    /// Schema version for this export.
    pub schema_version: u32,
    /// Shared contract ref used by every row and export.
    pub shared_contract_ref: String,
    /// Stable export id.
    pub export_id: String,
    /// Timestamp used by generated export fixtures.
    pub generated_at: String,
    /// Audit packet this export mirrors.
    pub audit_ref: String,
    /// Reviewer-facing docs page.
    pub docs_ref: String,
    /// Human-readable generated report ref.
    pub report_ref: String,
    /// Aggregate summary mirrored from the audit packet.
    pub summary: ExtensionWebviewBoundarySummary,
    /// Export-safe support rows.
    pub support_rows: Vec<ExtensionWebviewBoundarySupportRow>,
    /// Defect counts keyed by closed defect token.
    pub defect_counts_by_kind: BTreeMap<String, usize>,
    /// True when raw URLs, tokens, cookies, DOM text, and private payloads are excluded.
    pub raw_private_material_excluded: bool,
}

/// Evaluates one input row into an audited product row.
pub fn evaluate_extension_webview_boundary_row(
    input: ExtensionWebviewBoundaryInput,
) -> ExtensionWebviewBoundaryRow {
    let safe_browser_baseline_required = requires_safe_browser_baseline(
        input.surface_class,
        input.origin_class,
        input.risky_content_flow,
    );
    let safe_browser_baseline_satisfied = safe_browser_baseline_satisfied(
        input.browser_handoff_posture_class,
        safe_browser_baseline_required,
    );
    let visible_boundary_finding_refs = visible_boundary_finding_refs(&input.row_id);

    let mut row = ExtensionWebviewBoundaryRow {
        record_kind: EXTENSION_WEBVIEW_BOUNDARY_ROW_RECORD_KIND.to_owned(),
        schema_version: EXTENSION_WEBVIEW_BOUNDARY_SCHEMA_VERSION,
        shared_contract_ref: EXTENSION_WEBVIEW_BOUNDARY_SHARED_CONTRACT_REF.to_owned(),
        row_id: input.row_id,
        extension_id: input.extension_id,
        extension_name: input.extension_name,
        publisher_label: input.publisher_label,
        surface_id: input.surface_id,
        surface_label: input.surface_label,
        surface_class: input.surface_class,
        origin_label: input.origin_label,
        origin_host_label: input.origin_host_label,
        origin_class: input.origin_class,
        boundary_state_class: input.boundary_state_class,
        host_chrome_trust_class: input.host_chrome_trust_class,
        embedded_content_trust_class: input.embedded_content_trust_class,
        permission_state_class: input.permission_state_class,
        browser_handoff_posture_class: input.browser_handoff_posture_class,
        fallback_target_class: input.fallback_target_class,
        browser_handoff_reason_class: input.browser_handoff_reason_class,
        browser_handoff_packet_ref: input.browser_handoff_packet_ref,
        current_scope_label: input.current_scope_label,
        host_chrome_controls: input.host_chrome_controls,
        appearance_inheritance: input.appearance_inheritance,
        native_approval_boundary_class: input.native_approval_boundary_class,
        host_authority_scope_class: input.host_authority_scope_class,
        risky_content_flow: input.risky_content_flow,
        safe_browser_baseline_required,
        safe_browser_baseline_satisfied,
        permission_evidence_refs: input.permission_evidence_refs,
        evidence_refs: input.evidence_refs,
        visible_boundary_finding_refs,
        support_boundary_summary: input.support_boundary_summary,
        decision_class: ExtensionWebviewBoundaryDecisionClass::Conformant,
        row_defect_kind_tokens: Vec::new(),
        generated_at: input.generated_at,
    };

    let row_defects = validate_extension_webview_boundary_row(&row);
    row.row_defect_kind_tokens = defect_kind_tokens(&row_defects);
    row.decision_class = if row_defects.is_empty() {
        ExtensionWebviewBoundaryDecisionClass::Conformant
    } else if row_defects.iter().any(|defect| {
        matches!(
            defect.defect_kind,
            ExtensionWebviewBoundaryDefectKind::EmbeddedNativeApprovalAttempt
                | ExtensionWebviewBoundaryDefectKind::UnboundedHostAuthority
                | ExtensionWebviewBoundaryDefectKind::RiskySurfaceWithoutSafeBrowserBaseline
        )
    }) {
        ExtensionWebviewBoundaryDecisionClass::Refused
    } else {
        ExtensionWebviewBoundaryDecisionClass::NeedsReview
    };
    row
}

/// Projects one product row into a metadata-safe support row.
pub fn project_extension_webview_boundary_support_row(
    row: &ExtensionWebviewBoundaryRow,
) -> ExtensionWebviewBoundarySupportRow {
    ExtensionWebviewBoundarySupportRow {
        record_kind: EXTENSION_WEBVIEW_BOUNDARY_SUPPORT_ROW_RECORD_KIND.to_owned(),
        schema_version: EXTENSION_WEBVIEW_BOUNDARY_SCHEMA_VERSION,
        shared_contract_ref: EXTENSION_WEBVIEW_BOUNDARY_SHARED_CONTRACT_REF.to_owned(),
        support_row_id: format!("extension-webview-boundary:support-row:{}", row.row_id),
        row_ref: row.row_id.clone(),
        extension_id: row.extension_id.clone(),
        extension_name: row.extension_name.clone(),
        publisher_label: row.publisher_label.clone(),
        surface_label: row.surface_label.clone(),
        surface_class: row.surface_class,
        origin_host_label: row.origin_host_label.clone(),
        boundary_state_class: row.boundary_state_class,
        host_chrome_trust_class: row.host_chrome_trust_class,
        permission_state_class: row.permission_state_class,
        browser_handoff_posture_class: row.browser_handoff_posture_class,
        fallback_target_class: row.fallback_target_class,
        browser_handoff_packet_ref: row.browser_handoff_packet_ref.clone(),
        current_scope_label: row.current_scope_label.clone(),
        visible_boundary_finding_refs: row.visible_boundary_finding_refs.clone(),
        row_defect_kind_tokens: row.row_defect_kind_tokens.clone(),
        support_boundary_summary: row.support_boundary_summary.clone(),
    }
}

/// Projects a metadata-safe support export from an audit packet.
pub fn project_extension_webview_boundary_support_export(
    packet: &ExtensionWebviewBoundaryAuditPacket,
    export_id: impl Into<String>,
    generated_at: impl Into<String>,
) -> ExtensionWebviewBoundarySupportExport {
    let mut defect_counts_by_kind = BTreeMap::new();
    for defect in &packet.defects {
        *defect_counts_by_kind
            .entry(defect.defect_kind.as_str().to_owned())
            .or_insert(0) += 1;
    }

    ExtensionWebviewBoundarySupportExport {
        record_kind: EXTENSION_WEBVIEW_BOUNDARY_SUPPORT_EXPORT_RECORD_KIND.to_owned(),
        schema_version: EXTENSION_WEBVIEW_BOUNDARY_SCHEMA_VERSION,
        shared_contract_ref: EXTENSION_WEBVIEW_BOUNDARY_SHARED_CONTRACT_REF.to_owned(),
        export_id: export_id.into(),
        generated_at: generated_at.into(),
        audit_ref: packet.audit_id.clone(),
        docs_ref: packet.docs_ref.clone(),
        report_ref: packet.report_ref.clone(),
        summary: packet.summary.clone(),
        support_rows: packet.support_rows.clone(),
        defect_counts_by_kind,
        raw_private_material_excluded: true,
    }
}

/// Validates one product row without considering support-export parity.
pub fn validate_extension_webview_boundary_row(
    row: &ExtensionWebviewBoundaryRow,
) -> Vec<ExtensionWebviewBoundaryDefect> {
    let mut defects = Vec::new();

    if row.extension_name.trim().is_empty()
        || row.publisher_label.trim().is_empty()
        || row.origin_label.trim().is_empty()
        || row.origin_host_label.trim().is_empty()
    {
        defects.push(ExtensionWebviewBoundaryDefect::new(
            &row.row_id,
            ExtensionWebviewBoundaryDefectKind::OwnerOriginChromeMissing,
            "owner_origin_chrome",
            "extension name, publisher label, origin label, and origin host must all be visible",
        ));
    }

    for control in required_host_chrome_controls() {
        if !row.host_chrome_controls.contains(&control) {
            defects.push(ExtensionWebviewBoundaryDefect::new(
                &row.row_id,
                ExtensionWebviewBoundaryDefectKind::HostChromeControlMissing,
                "host_chrome_controls",
                format!(
                    "required host chrome control `{}` is missing",
                    control.as_str()
                ),
            ));
            break;
        }
    }

    if !row
        .host_chrome_controls
        .contains(&ExtensionHostChromeControlClass::PermissionState)
    {
        defects.push(ExtensionWebviewBoundaryDefect::new(
            &row.row_id,
            ExtensionWebviewBoundaryDefectKind::PermissionStateMissing,
            "permission_state_class",
            "permission state must be visible in host chrome",
        ));
    }

    if row.host_chrome_trust_class != row.embedded_content_trust_class {
        defects.push(ExtensionWebviewBoundaryDefect::new(
            &row.row_id,
            ExtensionWebviewBoundaryDefectKind::TrustClassParityDrift,
            "host_chrome_trust_class",
            "host chrome trust class must match the extension or embedded-content trust claim",
        ));
    }

    if row.safe_browser_baseline_required && !row.safe_browser_baseline_satisfied {
        defects.push(ExtensionWebviewBoundaryDefect::new(
            &row.row_id,
            ExtensionWebviewBoundaryDefectKind::RiskySurfaceWithoutSafeBrowserBaseline,
            "browser_handoff_posture_class",
            "risky or provider-owned content must keep a system-browser, device-code, policy-blocked, or offline-blocked baseline",
        ));
    }

    if row.browser_handoff_posture_class == ExtensionBrowserHandoffPostureClass::SystemBrowserFirst
        && row
            .browser_handoff_packet_ref
            .as_deref()
            .map(str::trim)
            .unwrap_or_default()
            .is_empty()
    {
        defects.push(ExtensionWebviewBoundaryDefect::new(
            &row.row_id,
            ExtensionWebviewBoundaryDefectKind::BrowserHandoffPacketMissing,
            "browser_handoff_packet_ref",
            "system-browser-first posture must quote a browser handoff packet ref",
        ));
    }

    if row.native_approval_boundary_class
        != ExtensionNativeApprovalBoundaryClass::HostOwnedNativeSurface
    {
        defects.push(ExtensionWebviewBoundaryDefect::new(
            &row.row_id,
            ExtensionWebviewBoundaryDefectKind::EmbeddedNativeApprovalAttempt,
            "native_approval_boundary_class",
            "high-risk approvals must remain host-owned native surfaces",
        ));
    }

    if row.host_authority_scope_class == ExtensionHostAuthorityScopeClass::UnboundedHostAuthority {
        defects.push(ExtensionWebviewBoundaryDefect::new(
            &row.row_id,
            ExtensionWebviewBoundaryDefectKind::UnboundedHostAuthority,
            "host_authority_scope_class",
            "embedded surfaces must not receive unbounded host authority",
        ));
    }

    if !row.appearance_inheritance.all_axes_disclosed() {
        defects.push(ExtensionWebviewBoundaryDefect::new(
            &row.row_id,
            ExtensionWebviewBoundaryDefectKind::AppearanceInheritanceDisclosureMissing,
            "appearance_inheritance",
            "theme, zoom, density, focus, motion, and contrast inheritance must be disclosed",
        ));
    }

    defects
}

/// Audits product rows and support rows together.
pub fn audit_extension_webview_boundary_rows(
    rows: &[ExtensionWebviewBoundaryRow],
    support_rows: &[ExtensionWebviewBoundarySupportRow],
) -> Vec<ExtensionWebviewBoundaryDefect> {
    let mut defects = Vec::new();
    let support_index: BTreeMap<&str, &ExtensionWebviewBoundarySupportRow> = support_rows
        .iter()
        .map(|row| (row.row_ref.as_str(), row))
        .collect();

    for row in rows {
        defects.extend(validate_extension_webview_boundary_row(row));
        match support_index.get(row.row_id.as_str()) {
            Some(support) if support_row_matches_product_row(row, support) => {}
            Some(_) => defects.push(ExtensionWebviewBoundaryDefect::new(
                &row.row_id,
                ExtensionWebviewBoundaryDefectKind::SupportExportParityDrift,
                "support_row",
                "support export row drifted from the product row",
            )),
            None => defects.push(ExtensionWebviewBoundaryDefect::new(
                &row.row_id,
                ExtensionWebviewBoundaryDefectKind::SupportExportParityDrift,
                "support_row",
                "support export row is missing",
            )),
        }
    }

    defects
}

/// Validates that a packet has current constants and no defects.
pub fn validate_extension_webview_boundary_packet(
    packet: &ExtensionWebviewBoundaryAuditPacket,
) -> Result<(), Vec<ExtensionWebviewBoundaryDefect>> {
    let mut defects = Vec::new();
    if packet.record_kind != EXTENSION_WEBVIEW_BOUNDARY_AUDIT_PACKET_RECORD_KIND {
        defects.push(packet_defect(
            ExtensionWebviewBoundaryDefectKind::SupportExportParityDrift,
            "record_kind",
            "packet record kind is not current",
        ));
    }
    if packet.schema_version != EXTENSION_WEBVIEW_BOUNDARY_SCHEMA_VERSION {
        defects.push(packet_defect(
            ExtensionWebviewBoundaryDefectKind::SupportExportParityDrift,
            "schema_version",
            "packet schema version is not current",
        ));
    }
    if packet.shared_contract_ref != EXTENSION_WEBVIEW_BOUNDARY_SHARED_CONTRACT_REF {
        defects.push(packet_defect(
            ExtensionWebviewBoundaryDefectKind::SupportExportParityDrift,
            "shared_contract_ref",
            "packet shared contract ref is not current",
        ));
    }
    defects.extend(audit_extension_webview_boundary_rows(
        &packet.rows,
        &packet.support_rows,
    ));
    if defects.is_empty() {
        Ok(())
    } else {
        Err(defects)
    }
}

/// Validates that a support export mirrors the packet and excludes private material.
pub fn validate_extension_webview_boundary_support_export(
    packet: &ExtensionWebviewBoundaryAuditPacket,
    export: &ExtensionWebviewBoundarySupportExport,
) -> Result<(), Vec<ExtensionWebviewBoundaryDefect>> {
    let mut defects = Vec::new();
    if export.record_kind != EXTENSION_WEBVIEW_BOUNDARY_SUPPORT_EXPORT_RECORD_KIND {
        defects.push(packet_defect(
            ExtensionWebviewBoundaryDefectKind::SupportExportParityDrift,
            "record_kind",
            "support export record kind is not current",
        ));
    }
    if export.schema_version != EXTENSION_WEBVIEW_BOUNDARY_SCHEMA_VERSION {
        defects.push(packet_defect(
            ExtensionWebviewBoundaryDefectKind::SupportExportParityDrift,
            "schema_version",
            "support export schema version is not current",
        ));
    }
    if export.shared_contract_ref != EXTENSION_WEBVIEW_BOUNDARY_SHARED_CONTRACT_REF {
        defects.push(packet_defect(
            ExtensionWebviewBoundaryDefectKind::SupportExportParityDrift,
            "shared_contract_ref",
            "support export shared contract ref is not current",
        ));
    }
    if export.audit_ref != packet.audit_id
        || export.summary != packet.summary
        || export.support_rows != packet.support_rows
    {
        defects.push(packet_defect(
            ExtensionWebviewBoundaryDefectKind::SupportExportParityDrift,
            "support_export",
            "support export no longer mirrors the audit packet",
        ));
    }
    if !export.raw_private_material_excluded {
        defects.push(packet_defect(
            ExtensionWebviewBoundaryDefectKind::RawPrivateMaterialExported,
            "raw_private_material_excluded",
            "support export must exclude raw private material",
        ));
    }
    if defects.is_empty() {
        Ok(())
    } else {
        Err(defects)
    }
}

/// Builds the seeded extension webview boundary audit packet.
pub fn seeded_extension_webview_boundary_audit_packet() -> ExtensionWebviewBoundaryAuditPacket {
    let rows = seeded_extension_webview_boundary_inputs()
        .into_iter()
        .map(evaluate_extension_webview_boundary_row)
        .collect();
    ExtensionWebviewBoundaryAuditPacket::from_rows(
        "extension-webview-boundary:audit:beta:default",
        "2026-05-16T00:00:00Z",
        rows,
    )
}

/// Returns the fixture inputs used by the seeded audit packet.
pub fn seeded_extension_webview_boundary_inputs() -> Vec<ExtensionWebviewBoundaryInput> {
    vec![
        ExtensionWebviewBoundaryInput {
            row_id: "extension-webview-boundary:dev.aureline.samples/wasm-notes:readme-preview"
                .to_owned(),
            extension_id: "dev.aureline.samples/wasm-notes".to_owned(),
            extension_name: "Wasm Notes".to_owned(),
            publisher_label: "Aureline Samples".to_owned(),
            surface_id: "surface:wasm-notes:readme-preview".to_owned(),
            surface_label: "README preview webview".to_owned(),
            surface_class: ExtensionEmbeddedSurfaceClass::ExtensionWebviewPanel,
            origin_label: "extension://dev.aureline.samples/wasm-notes/readme.html".to_owned(),
            origin_host_label: "dev.aureline.samples".to_owned(),
            origin_class: ExtensionOriginClass::ExtensionLocalAsset,
            boundary_state_class: ExtensionBoundaryStateClass::LiveVerified,
            host_chrome_trust_class: ExtensionSurfaceTrustClass::Trusted,
            embedded_content_trust_class: ExtensionSurfaceTrustClass::Trusted,
            permission_state_class: ExtensionSurfacePermissionClass::HostOwnedInspectOnly,
            browser_handoff_posture_class:
                ExtensionBrowserHandoffPostureClass::SystemBrowserFirst,
            fallback_target_class: ExtensionFallbackTargetClass::SystemBrowserHandoffPacket,
            browser_handoff_reason_class:
                ExtensionBrowserHandoffReasonClass::DocumentationProviderExternal,
            browser_handoff_packet_ref: Some(
                "browser-handoff:extension:wasm-notes-readme-preview".to_owned(),
            ),
            current_scope_label: "local workspace docs preview".to_owned(),
            host_chrome_controls: standard_host_chrome_controls(),
            appearance_inheritance: full_inheritance(),
            native_approval_boundary_class:
                ExtensionNativeApprovalBoundaryClass::HostOwnedNativeSurface,
            host_authority_scope_class: ExtensionHostAuthorityScopeClass::NoHostAuthority,
            risky_content_flow: false,
            permission_evidence_refs: vec![
                "permission_manifest:dev.aureline.samples/wasm-notes:ui.preview".to_owned(),
            ],
            evidence_refs: vec![
                "registry_descriptor:dev.aureline.samples/wasm-notes:1.0.0-beta.1".to_owned(),
                "support_export:extension_inspector:wasm-notes".to_owned(),
            ],
            support_boundary_summary:
                "Local extension webview preview; owner, publisher, origin, permission, and browser handoff are host-rendered."
                    .to_owned(),
            generated_at: "2026-05-16T00:00:00Z".to_owned(),
        },
        ExtensionWebviewBoundaryInput {
            row_id:
                "extension-webview-boundary:com.acme.cloud-tools:status-dashboard".to_owned(),
            extension_id: "com.acme.cloud-tools".to_owned(),
            extension_name: "Acme Cloud Tools".to_owned(),
            publisher_label: "Acme Cloud, Inc.".to_owned(),
            surface_id: "surface:acme-cloud:status-dashboard".to_owned(),
            surface_label: "Hosted status dashboard".to_owned(),
            surface_class: ExtensionEmbeddedSurfaceClass::HostedDashboard,
            origin_label: "https://status.acme.example/embedded".to_owned(),
            origin_host_label: "status.acme.example".to_owned(),
            origin_class: ExtensionOriginClass::CrossOriginSubframe,
            boundary_state_class: ExtensionBoundaryStateClass::CrossOriginLimited,
            host_chrome_trust_class: ExtensionSurfaceTrustClass::Limited,
            embedded_content_trust_class: ExtensionSurfaceTrustClass::Limited,
            permission_state_class: ExtensionSurfacePermissionClass::HostOwnedBrowserOnly,
            browser_handoff_posture_class:
                ExtensionBrowserHandoffPostureClass::SystemBrowserFirst,
            fallback_target_class: ExtensionFallbackTargetClass::SystemBrowserHandoffPacket,
            browser_handoff_reason_class:
                ExtensionBrowserHandoffReasonClass::CrossOriginActiveContent,
            browser_handoff_packet_ref: Some(
                "browser-handoff:extension:acme-status-dashboard".to_owned(),
            ),
            current_scope_label: "workspace status object: acme-prod".to_owned(),
            host_chrome_controls: standard_host_chrome_controls(),
            appearance_inheritance: partial_inheritance(),
            native_approval_boundary_class:
                ExtensionNativeApprovalBoundaryClass::HostOwnedNativeSurface,
            host_authority_scope_class:
                ExtensionHostAuthorityScopeClass::DeclaredBoundedHostAuthority,
            risky_content_flow: true,
            permission_evidence_refs: vec![
                "permission_manifest:com.acme.cloud-tools:network.status-read".to_owned(),
                "effective_permission:com.acme.cloud-tools:hosted-dashboard".to_owned(),
            ],
            evidence_refs: vec![
                "runtime_contract:com.acme.cloud-tools:external-host".to_owned(),
                "policy_context:egress:approved-status-host".to_owned(),
            ],
            support_boundary_summary:
                "Cross-origin dashboard remains inspect-only in product; system browser handoff preserves the provider status object."
                    .to_owned(),
            generated_at: "2026-05-16T00:00:00Z".to_owned(),
        },
        ExtensionWebviewBoundaryInput {
            row_id: "extension-webview-boundary:org.python.docs:cached-provider-pane".to_owned(),
            extension_id: "org.python.docs".to_owned(),
            extension_name: "Python Reference Docs".to_owned(),
            publisher_label: "Python Docs Community".to_owned(),
            surface_id: "surface:python-docs:cached-provider-pane".to_owned(),
            surface_label: "Cached provider docs pane".to_owned(),
            surface_class: ExtensionEmbeddedSurfaceClass::DocumentationProviderPane,
            origin_label: "https://docs.python.example/library/pathlib.html".to_owned(),
            origin_host_label: "docs.python.example".to_owned(),
            origin_class: ExtensionOriginClass::PublisherHostedOrigin,
            boundary_state_class: ExtensionBoundaryStateClass::StaleSnapshot,
            host_chrome_trust_class: ExtensionSurfaceTrustClass::Stale,
            embedded_content_trust_class: ExtensionSurfaceTrustClass::Stale,
            permission_state_class: ExtensionSurfacePermissionClass::HostOwnedInspectOnly,
            browser_handoff_posture_class:
                ExtensionBrowserHandoffPostureClass::ExternalOpenUnavailableOffline,
            fallback_target_class: ExtensionFallbackTargetClass::NoExternalHandoff,
            browser_handoff_reason_class:
                ExtensionBrowserHandoffReasonClass::DocumentationProviderExternal,
            browser_handoff_packet_ref: None,
            current_scope_label: "offline docs pack: python-reference".to_owned(),
            host_chrome_controls: standard_host_chrome_controls(),
            appearance_inheritance: full_inheritance(),
            native_approval_boundary_class:
                ExtensionNativeApprovalBoundaryClass::HostOwnedNativeSurface,
            host_authority_scope_class: ExtensionHostAuthorityScopeClass::NoHostAuthority,
            risky_content_flow: true,
            permission_evidence_refs: vec![
                "permission_manifest:org.python.docs:docs-provider-read".to_owned(),
            ],
            evidence_refs: vec![
                "docs_pack:python-reference:cached".to_owned(),
                "network_state:offline".to_owned(),
            ],
            support_boundary_summary:
                "Provider docs pane is a stale offline snapshot; host chrome keeps source, origin, and external-open unavailable state visible."
                    .to_owned(),
            generated_at: "2026-05-16T00:00:00Z".to_owned(),
        },
        ExtensionWebviewBoundaryInput {
            row_id: "extension-webview-boundary:org.gitforge.review:auth-checkpoint".to_owned(),
            extension_id: "org.gitforge.review".to_owned(),
            extension_name: "GitForge Review".to_owned(),
            publisher_label: "GitForge".to_owned(),
            surface_id: "surface:gitforge:auth-checkpoint".to_owned(),
            surface_label: "Provider sign-in checkpoint".to_owned(),
            surface_class: ExtensionEmbeddedSurfaceClass::ProviderAuthSurface,
            origin_label: "https://auth.gitforge.example/oauth/authorize".to_owned(),
            origin_host_label: "auth.gitforge.example".to_owned(),
            origin_class: ExtensionOriginClass::ProviderOwnedAuthDomain,
            boundary_state_class: ExtensionBoundaryStateClass::LiveVerified,
            host_chrome_trust_class: ExtensionSurfaceTrustClass::Limited,
            embedded_content_trust_class: ExtensionSurfaceTrustClass::Limited,
            permission_state_class:
                ExtensionSurfacePermissionClass::HostOwnedWithNativeStepUpRequired,
            browser_handoff_posture_class:
                ExtensionBrowserHandoffPostureClass::SystemBrowserFirst,
            fallback_target_class: ExtensionFallbackTargetClass::SystemBrowserHandoffPacket,
            browser_handoff_reason_class: ExtensionBrowserHandoffReasonClass::ProviderOwnedAuth,
            browser_handoff_packet_ref: Some(
                "browser-handoff:extension:gitforge-auth-checkpoint".to_owned(),
            ),
            current_scope_label: "provider account: gitforge/acme".to_owned(),
            host_chrome_controls: standard_host_chrome_controls(),
            appearance_inheritance: full_inheritance(),
            native_approval_boundary_class:
                ExtensionNativeApprovalBoundaryClass::HostOwnedNativeSurface,
            host_authority_scope_class:
                ExtensionHostAuthorityScopeClass::DeclaredBoundedHostAuthority,
            risky_content_flow: true,
            permission_evidence_refs: vec![
                "permission_manifest:org.gitforge.review:connected-provider-access".to_owned(),
                "effective_permission:org.gitforge.review:oauth-handoff".to_owned(),
            ],
            evidence_refs: vec![
                "auth_policy:system-browser-first".to_owned(),
                "provider_descriptor:gitforge".to_owned(),
            ],
            support_boundary_summary:
                "Provider-owned auth checkpoint uses system browser first; embedded body never collects raw credentials or owns step-up approval."
                    .to_owned(),
            generated_at: "2026-05-16T00:00:00Z".to_owned(),
        },
        ExtensionWebviewBoundaryInput {
            row_id: "extension-webview-boundary:dev.browser.inspect:runtime-storage".to_owned(),
            extension_id: "dev.browser.inspect".to_owned(),
            extension_name: "Browser Inspect".to_owned(),
            publisher_label: "Aureline Labs".to_owned(),
            surface_id: "surface:browser-inspect:runtime-storage".to_owned(),
            surface_label: "Runtime storage bridge".to_owned(),
            surface_class: ExtensionEmbeddedSurfaceClass::BrowserRuntimeBridge,
            origin_label: "browser-target://tab/local-web-app".to_owned(),
            origin_host_label: "localhost:5173".to_owned(),
            origin_class: ExtensionOriginClass::RuntimeTargetOrigin,
            boundary_state_class: ExtensionBoundaryStateClass::StaleSnapshot,
            host_chrome_trust_class: ExtensionSurfaceTrustClass::Stale,
            embedded_content_trust_class: ExtensionSurfaceTrustClass::Stale,
            permission_state_class:
                ExtensionSurfacePermissionClass::ExtensionMutableWithHostReview,
            browser_handoff_posture_class:
                ExtensionBrowserHandoffPostureClass::SystemBrowserFirst,
            fallback_target_class: ExtensionFallbackTargetClass::SystemBrowserHandoffPacket,
            browser_handoff_reason_class:
                ExtensionBrowserHandoffReasonClass::BrowserRuntimeProtocolLimit,
            browser_handoff_packet_ref: Some(
                "browser-handoff:extension:browser-inspect-runtime-tab".to_owned(),
            ),
            current_scope_label: "browser target: local-web-app tab".to_owned(),
            host_chrome_controls: standard_host_chrome_controls(),
            appearance_inheritance: partial_inheritance(),
            native_approval_boundary_class:
                ExtensionNativeApprovalBoundaryClass::HostOwnedNativeSurface,
            host_authority_scope_class:
                ExtensionHostAuthorityScopeClass::DeclaredBoundedHostAuthority,
            risky_content_flow: true,
            permission_evidence_refs: vec![
                "permission_manifest:dev.browser.inspect:runtime-storage".to_owned(),
                "effective_permission:dev.browser.inspect:storage-mutation-review".to_owned(),
            ],
            evidence_refs: vec![
                "runtime_session:browser-inspect:local-web-app:last-snapshot".to_owned(),
                "redaction_policy:browser-storage:support-safe".to_owned(),
            ],
            support_boundary_summary:
                "Runtime bridge keeps stale storage as a labeled snapshot and routes mutation-capable actions through host review."
                    .to_owned(),
            generated_at: "2026-05-16T00:00:00Z".to_owned(),
        },
        ExtensionWebviewBoundaryInput {
            row_id:
                "extension-webview-boundary:com.vendor.billing:policy-blocked-account".to_owned(),
            extension_id: "com.vendor.billing".to_owned(),
            extension_name: "Vendor Billing Helper".to_owned(),
            publisher_label: "Vendor Systems".to_owned(),
            surface_id: "surface:vendor-billing:account".to_owned(),
            surface_label: "Hosted account dashboard".to_owned(),
            surface_class: ExtensionEmbeddedSurfaceClass::HostedDashboard,
            origin_label: "https://billing.vendor.example/account".to_owned(),
            origin_host_label: "billing.vendor.example".to_owned(),
            origin_class: ExtensionOriginClass::PublisherHostedOrigin,
            boundary_state_class: ExtensionBoundaryStateClass::PolicyBlocked,
            host_chrome_trust_class: ExtensionSurfaceTrustClass::PolicyBlocked,
            embedded_content_trust_class: ExtensionSurfaceTrustClass::PolicyBlocked,
            permission_state_class: ExtensionSurfacePermissionClass::HostOwnedBrowserOnly,
            browser_handoff_posture_class:
                ExtensionBrowserHandoffPostureClass::ExternalOpenBlockedByPolicy,
            fallback_target_class: ExtensionFallbackTargetClass::PolicyReview,
            browser_handoff_reason_class:
                ExtensionBrowserHandoffReasonClass::HostedDashboardAuthority,
            browser_handoff_packet_ref: None,
            current_scope_label: "managed org policy: hosted billing disabled".to_owned(),
            host_chrome_controls: standard_host_chrome_controls(),
            appearance_inheritance: full_inheritance(),
            native_approval_boundary_class:
                ExtensionNativeApprovalBoundaryClass::HostOwnedNativeSurface,
            host_authority_scope_class: ExtensionHostAuthorityScopeClass::NoHostAuthority,
            risky_content_flow: true,
            permission_evidence_refs: vec![
                "permission_manifest:com.vendor.billing:hosted-dashboard".to_owned(),
            ],
            evidence_refs: vec![
                "policy_decision:external-open:billing-vendor-blocked".to_owned(),
                "support_export:policy:billing-dashboard-blocked".to_owned(),
            ],
            support_boundary_summary:
                "Hosted billing dashboard is policy-blocked; host chrome shows the owner, origin, blocked handoff, and policy review path."
                    .to_owned(),
            generated_at: "2026-05-16T00:00:00Z".to_owned(),
        },
    ]
}

fn required_host_chrome_controls() -> [ExtensionHostChromeControlClass; 9] {
    [
        ExtensionHostChromeControlClass::ExtensionName,
        ExtensionHostChromeControlClass::PublisherLabel,
        ExtensionHostChromeControlClass::OriginLabel,
        ExtensionHostChromeControlClass::BoundaryState,
        ExtensionHostChromeControlClass::TrustClass,
        ExtensionHostChromeControlClass::PermissionState,
        ExtensionHostChromeControlClass::OpenInBrowser,
        ExtensionHostChromeControlClass::SupportExport,
        ExtensionHostChromeControlClass::ProfileOrTargetScope,
    ]
}

fn standard_host_chrome_controls() -> Vec<ExtensionHostChromeControlClass> {
    let mut controls = required_host_chrome_controls().to_vec();
    controls.push(ExtensionHostChromeControlClass::Reload);
    controls
}

fn full_inheritance() -> ExtensionAppearanceInheritance {
    ExtensionAppearanceInheritance {
        theme_class: ExtensionInheritanceClass::Inherits,
        zoom_class: ExtensionInheritanceClass::Inherits,
        density_class: ExtensionInheritanceClass::Inherits,
        focus_class: ExtensionInheritanceClass::Inherits,
        reduced_motion_class: ExtensionInheritanceClass::Inherits,
        contrast_class: ExtensionInheritanceClass::Inherits,
    }
}

fn partial_inheritance() -> ExtensionAppearanceInheritance {
    ExtensionAppearanceInheritance {
        theme_class: ExtensionInheritanceClass::Inherits,
        zoom_class: ExtensionInheritanceClass::Inherits,
        density_class: ExtensionInheritanceClass::Partial,
        focus_class: ExtensionInheritanceClass::Inherits,
        reduced_motion_class: ExtensionInheritanceClass::Inherits,
        contrast_class: ExtensionInheritanceClass::Partial,
    }
}

fn requires_safe_browser_baseline(
    surface_class: ExtensionEmbeddedSurfaceClass,
    origin_class: ExtensionOriginClass,
    risky_content_flow: bool,
) -> bool {
    risky_content_flow
        || matches!(
            surface_class,
            ExtensionEmbeddedSurfaceClass::HostedDashboard
                | ExtensionEmbeddedSurfaceClass::ProviderAuthSurface
                | ExtensionEmbeddedSurfaceClass::BrowserRuntimeBridge
        )
        || matches!(
            origin_class,
            ExtensionOriginClass::PublisherHostedOrigin
                | ExtensionOriginClass::CrossOriginSubframe
                | ExtensionOriginClass::ProviderOwnedAuthDomain
                | ExtensionOriginClass::RuntimeTargetOrigin
        )
}

fn safe_browser_baseline_satisfied(
    posture: ExtensionBrowserHandoffPostureClass,
    required: bool,
) -> bool {
    !required
        || matches!(
            posture,
            ExtensionBrowserHandoffPostureClass::SystemBrowserFirst
                | ExtensionBrowserHandoffPostureClass::DeviceCodeFallbackOffered
                | ExtensionBrowserHandoffPostureClass::ExternalOpenBlockedByPolicy
                | ExtensionBrowserHandoffPostureClass::ExternalOpenUnavailableOffline
        )
}

fn visible_boundary_finding_refs(row_id: &str) -> Vec<String> {
    [
        "owner-origin-chrome",
        "trust-class-parity",
        "browser-handoff",
        "native-approval-boundary",
        "support-export-parity",
    ]
    .iter()
    .map(|suffix| format!("extension-webview-boundary:finding:{row_id}:{suffix}"))
    .collect()
}

fn support_row_matches_product_row(
    row: &ExtensionWebviewBoundaryRow,
    support: &ExtensionWebviewBoundarySupportRow,
) -> bool {
    support.record_kind == EXTENSION_WEBVIEW_BOUNDARY_SUPPORT_ROW_RECORD_KIND
        && support.schema_version == EXTENSION_WEBVIEW_BOUNDARY_SCHEMA_VERSION
        && support.shared_contract_ref == EXTENSION_WEBVIEW_BOUNDARY_SHARED_CONTRACT_REF
        && support.extension_id == row.extension_id
        && support.extension_name == row.extension_name
        && support.publisher_label == row.publisher_label
        && support.surface_label == row.surface_label
        && support.surface_class == row.surface_class
        && support.origin_host_label == row.origin_host_label
        && support.boundary_state_class == row.boundary_state_class
        && support.host_chrome_trust_class == row.host_chrome_trust_class
        && support.permission_state_class == row.permission_state_class
        && support.browser_handoff_posture_class == row.browser_handoff_posture_class
        && support.fallback_target_class == row.fallback_target_class
        && support.browser_handoff_packet_ref == row.browser_handoff_packet_ref
        && support.current_scope_label == row.current_scope_label
        && support.visible_boundary_finding_refs == row.visible_boundary_finding_refs
        && support.row_defect_kind_tokens == row.row_defect_kind_tokens
        && support.support_boundary_summary == row.support_boundary_summary
}

fn defect_kind_tokens(defects: &[ExtensionWebviewBoundaryDefect]) -> Vec<String> {
    let mut tokens: Vec<String> = Vec::new();
    for defect in defects {
        let token = defect.defect_kind.as_str().to_owned();
        if !tokens.contains(&token) {
            tokens.push(token);
        }
    }
    tokens.sort();
    tokens
}

fn packet_defect(
    defect_kind: ExtensionWebviewBoundaryDefectKind,
    field: impl Into<String>,
    message: impl Into<String>,
) -> ExtensionWebviewBoundaryDefect {
    ExtensionWebviewBoundaryDefect::new(
        "extension-webview-boundary:audit:packet",
        defect_kind,
        field,
        message,
    )
}

fn push_unique<T: Copy + PartialEq>(values: &mut Vec<T>, value: T) {
    if !values.contains(&value) {
        values.push(value);
    }
}
