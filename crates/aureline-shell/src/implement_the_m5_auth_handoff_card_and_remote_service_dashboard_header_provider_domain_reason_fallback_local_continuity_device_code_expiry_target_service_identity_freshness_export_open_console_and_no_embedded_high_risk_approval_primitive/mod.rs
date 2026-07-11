//! Implemented M5 auth handoff cards and remote/service dashboard headers.
//!
//! The frozen [embedded-boundary component matrix][matrix] names the reusable embedded /
//! browser-handoff boundary UI components and locks their controlled vocabulary. This module is
//! the fourth implement lane over that matrix: it turns the two auth / service-boundary
//! components — the **auth handoff card** and the **remote/service dashboard header** — into
//! resolvers that produce export-safe, honest projections instead of security-theater chrome.
//!
//! Two acceptance criteria drive the resolvers:
//!
//! * **AC1 — users can distinguish embedded sign-in checkpoints from system-browser or passkey
//!   handoff and know which local state remains intact while the handoff completes.**
//!   [`resolve_auth_handoff_card`] refuses to read as a clean card when the provider or domain is
//!   unstated, when the reason for handoff is unexplained, when the local-safe continuity note is
//!   missing, when the fallback state is unstated, when a device-code posture omits its code or
//!   expiry disclosure, when the embedded surface imitates native permission or approval chrome, or
//!   when a high-risk approval is embedded without a native step-up. A clean card names its handoff
//!   posture, provider/domain, reason, fallback state, local-continuity note, and — under a
//!   device-code posture — its code or expiry disclosure.
//! * **AC2 — remote or service dashboards never substitute for primary local recovery controls or
//!   hide their freshness and ownership boundaries.** [`resolve_remote_service_dashboard_header`]
//!   degrades the moment the target/service identity is unstated, the ownership boundary is
//!   undisclosed, the freshness/offline state is hidden, the dashboard substitutes for the primary
//!   local recovery controls, the export/open-console actions are unavailable, or a high-risk
//!   approval is allowed inside embedded chrome.
//!
//! The resolvers reuse the frozen matrix vocabulary directly — the single controlled
//! [`M5EmbeddedBoundaryDisposition`] boundary-disposition vocabulary, the [`M5EmbeddedFreshnessState`]
//! freshness vocabulary, the [`WebviewOwnerClass`] owner/origin vocabulary, the
//! [`BrowserHandoffKind`] browser-handoff vocabulary, the [`HandoffReasonClass`] handoff-reason
//! vocabulary, the [`FallbackStateClass`] fallback vocabulary, and the [`ExpiryDisclosureClass`]
//! device-code expiry vocabulary — so this lane can never fork its own provider, reason, fallback,
//! or expiry wording.
//!
//! [matrix]: crate::freeze_the_m5_docs_pane_header_embedded_origin_bar_boundary_fact_grid_marketplace_account_boundary_card_auth_handoff_card_remote_service_dashboard_header_open_in_browser_handoff_row_and_embedded_state_panel_component_matrix

mod seed;
#[cfg(test)]
mod tests;

pub use seed::{
    seeded_m5_auth_dashboard_controls,
    seeded_m5_auth_dashboard_controls_auth_handoff_beta_narrowed,
    seeded_m5_auth_dashboard_controls_remote_dashboard_preview_narrowed,
    M5_AUTH_DASHBOARD_CONTROLS_PACKET_ID,
};

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

use crate::freeze_the_m5_docs_pane_header_embedded_origin_bar_boundary_fact_grid_marketplace_account_boundary_card_auth_handoff_card_remote_service_dashboard_header_open_in_browser_handoff_row_and_embedded_state_panel_component_matrix::{
    M5EmbeddedAccessibilityRoute, M5EmbeddedBoundaryDisposition, M5EmbeddedConsumerSurface,
    M5EmbeddedDeploymentLine, M5EmbeddedDowngradeTrigger, M5EmbeddedFreshnessState,
    M5EmbeddedQualificationClass, M5EmbeddedRequiredLabel,
    M5_AUTH_HANDOFF_CARD_SCHEMA_REF, M5_EMBEDDED_BOUNDARY_COMPONENT_DOC_REF,
    M5_EMBEDDED_BOUNDARY_COMPONENT_SCHEMA_REF, M5_REMOTE_SERVICE_DASHBOARD_HEADER_SCHEMA_REF,
};
use crate::m5_auth_boundaries::{
    BrowserHandoffKind, ExpiryDisclosureClass, FallbackStateClass, HandoffReasonClass,
    WebviewOwnerClass, M5_AUTH_BOUNDARY_CONTRACT_DOC_REF,
};

/// Stable record-kind tag carried by [`M5AuthDashboardControlsPacket`].
pub const M5_AUTH_DASHBOARD_CONTROLS_RECORD_KIND: &str =
    "implement_m5_auth_handoff_card_and_remote_service_dashboard_header_controls";

/// Schema version for M5 auth handoff-card and remote/service dashboard-header controls.
pub const M5_AUTH_DASHBOARD_CONTROLS_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the combined controls boundary schema.
pub const M5_AUTH_DASHBOARD_CONTROLS_SCHEMA_REF: &str =
    "schemas/ui/m5-auth-handoff-card-remote-service-dashboard-header-controls.schema.json";

/// Repo-relative path of the controls doc.
pub const M5_AUTH_DASHBOARD_CONTROLS_DOC_REF: &str =
    "docs/help/m5_auth_handoff_card_and_remote_service_dashboard_header_controls.md";

/// Repo-relative path of the checked support-export artifact.
pub const M5_AUTH_DASHBOARD_CONTROLS_ARTIFACT_REF: &str =
    "artifacts/release/m5-auth-handoff-card-remote-service-dashboard-header-controls-proof/support_export.json";

/// Repo-relative path of the checked machine-readable controls CSV.
pub const M5_AUTH_DASHBOARD_CONTROLS_CSV_REF: &str =
    "artifacts/release/m5-auth-handoff-card-remote-service-dashboard-header-controls-proof/matrix.csv";

/// Repo-relative path of the checked Markdown design report.
pub const M5_AUTH_DASHBOARD_CONTROLS_REPORT_REF: &str =
    "artifacts/release/m5-auth-handoff-card-remote-service-dashboard-header-controls-proof/summary.md";

/// Repo-relative path of the protected fixture directory.
pub const M5_AUTH_DASHBOARD_CONTROLS_FIXTURE_DIR: &str =
    "fixtures/ui/m5-auth-handoff-card-remote-service-dashboard-header-controls";

/// Consumer surface an auth/dashboard controls row projects onto. Reuses the frozen matrix
/// consumer-surface taxonomy so no lane invents a parallel surface set.
pub type M5AuthDashboardConsumerSurface = M5EmbeddedConsumerSurface;

/// Handoff-reason classes an auth handoff card may carry, pinned locally because
/// [`HandoffReasonClass`] does not expose an `ALL` array. Order matches its declaration.
pub const BOUND_HANDOFF_REASONS: [HandoffReasonClass; 4] = [
    HandoffReasonClass::AuthenticateWithProvider,
    HandoffReasonClass::AuthorizeDeviceCode,
    HandoffReasonClass::ViewProviderContent,
    HandoffReasonClass::OpenVendorResource,
];

/// Fallback-state classes an auth handoff card may carry, pinned locally because
/// [`FallbackStateClass`] does not expose an `ALL` array. Order matches its declaration.
pub const BOUND_FALLBACK_STATES: [FallbackStateClass; 4] = [
    FallbackStateClass::LocalContinuityPreserved,
    FallbackStateClass::RetryHandoffInApp,
    FallbackStateClass::ManualCodeEntry,
    FallbackStateClass::CopyLinkForManualOpen,
];

/// Expiry-disclosure classes an auth handoff card may carry, pinned locally because
/// [`ExpiryDisclosureClass`] does not expose an `ALL` array. Order matches its declaration.
pub const BOUND_EXPIRY_DISCLOSURES: [ExpiryDisclosureClass; 3] = [
    ExpiryDisclosureClass::ExpiresWithCountdown,
    ExpiryDisclosureClass::ExpiresAtDisclosedTime,
    ExpiryDisclosureClass::NoExpiryApplicable,
];

/// The controlled auth-handoff posture — how a sign-in / authorization checkpoint reaches its
/// provider, so an embedded sign-in checkpoint is never confused with a system-browser or passkey
/// handoff.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5AuthHandoffPosture {
    /// An embedded sign-in checkpoint rendered inside Aureline chrome (capability-limited: it may
    /// never perform a high-risk approval without handing off to a native step-up).
    EmbeddedSignInCheckpoint,
    /// A sign-in handed off to the system browser.
    SystemBrowserHandoff,
    /// A passkey / platform-credential handoff to the operating system.
    PasskeyHandoff,
    /// A device-code authorization handoff (requires a code or expiry disclosure).
    DeviceCodeHandoff,
    /// A provider-content authorization handoff opened in the browser.
    ProviderContentHandoff,
}

impl M5AuthHandoffPosture {
    /// Every posture, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::EmbeddedSignInCheckpoint,
        Self::SystemBrowserHandoff,
        Self::PasskeyHandoff,
        Self::DeviceCodeHandoff,
        Self::ProviderContentHandoff,
    ];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::EmbeddedSignInCheckpoint => "embedded_sign_in_checkpoint",
            Self::SystemBrowserHandoff => "system_browser_handoff",
            Self::PasskeyHandoff => "passkey_handoff",
            Self::DeviceCodeHandoff => "device_code_handoff",
            Self::ProviderContentHandoff => "provider_content_handoff",
        }
    }

    /// True when this posture is an in-product embedded checkpoint rather than an external handoff.
    pub const fn is_embedded_checkpoint(self) -> bool {
        matches!(self, Self::EmbeddedSignInCheckpoint)
    }

    /// True when this posture must carry a device-code or expiry disclosure.
    pub const fn requires_device_code_disclosure(self) -> bool {
        matches!(self, Self::DeviceCodeHandoff)
    }
}

/// One mandatory rendered part an auth handoff card or a remote/service dashboard header must be
/// able to show, so no boundary truth is left implicit.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5AuthDashboardAnatomyPart {
    /// The component's stable identity / what it represents.
    Identity,
    /// The component's current typed boundary disposition.
    State,
    /// The non-visual keyboard route to the component.
    KeyboardRoute,
    /// The provider or domain behind the handoff (auth card).
    ProviderOrDomain,
    /// The reason for the handoff (auth card).
    HandoffReason,
    /// The fallback state if the handoff is blocked (auth card).
    FallbackState,
    /// The local-safe continuity note (auth card).
    LocalContinuity,
    /// The device-code / expiry disclosure where applicable (auth card).
    DeviceCodeOrExpiry,
    /// The target / service identity (dashboard header).
    ServiceIdentity,
    /// The freshness / offline state (dashboard header).
    FreshnessState,
    /// The export / open-console actions (dashboard header).
    ExportOrConsoleAction,
    /// The pointer back to the primary local recovery controls (dashboard header).
    LocalRecoveryAnchor,
}

impl M5AuthDashboardAnatomyPart {
    /// Every anatomy part, in declaration order.
    pub const ALL: [Self; 12] = [
        Self::Identity,
        Self::State,
        Self::KeyboardRoute,
        Self::ProviderOrDomain,
        Self::HandoffReason,
        Self::FallbackState,
        Self::LocalContinuity,
        Self::DeviceCodeOrExpiry,
        Self::ServiceIdentity,
        Self::FreshnessState,
        Self::ExportOrConsoleAction,
        Self::LocalRecoveryAnchor,
    ];

    /// The three parts every claimed component must be able to show.
    pub const MANDATORY: [Self; 3] = [Self::Identity, Self::State, Self::KeyboardRoute];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Identity => "identity",
            Self::State => "state",
            Self::KeyboardRoute => "keyboard_route",
            Self::ProviderOrDomain => "provider_or_domain",
            Self::HandoffReason => "handoff_reason",
            Self::FallbackState => "fallback_state",
            Self::LocalContinuity => "local_continuity",
            Self::DeviceCodeOrExpiry => "device_code_or_expiry",
            Self::ServiceIdentity => "service_identity",
            Self::FreshnessState => "freshness_state",
            Self::ExportOrConsoleAction => "export_or_console_action",
            Self::LocalRecoveryAnchor => "local_recovery_anchor",
        }
    }
}

/// Next safe action a component surfaces so a user is never left without a route out.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5AuthDashboardNextAction {
    /// Continue the sign-in / authorization in the system browser.
    OpenInSystemBrowser,
    /// Retry the handoff from within Aureline.
    RetryHandoff,
    /// Review the local-safe continuity note.
    ReviewLocalContinuity,
    /// Export or open the console for the dashboard.
    ExportOrOpenConsole,
    /// Review diagnostics for the unavailable signal.
    ReviewDiagnostics,
    /// No action is needed; the component is clean.
    NoActionNeeded,
}

impl M5AuthDashboardNextAction {
    /// Every next action, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::OpenInSystemBrowser,
        Self::RetryHandoff,
        Self::ReviewLocalContinuity,
        Self::ExportOrOpenConsole,
        Self::ReviewDiagnostics,
        Self::NoActionNeeded,
    ];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::OpenInSystemBrowser => "open_in_system_browser",
            Self::RetryHandoff => "retry_handoff",
            Self::ReviewLocalContinuity => "review_local_continuity",
            Self::ExportOrOpenConsole => "export_or_open_console",
            Self::ReviewDiagnostics => "review_diagnostics",
            Self::NoActionNeeded => "no_action_needed",
        }
    }
}

/// Field an auth/dashboard controls row exposes in the support export.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5AuthDashboardExportField {
    /// The consumer surface.
    ConsumerSurface,
    /// The component families covered.
    ComponentFamilies,
    /// The boundary dispositions carried.
    BoundaryDispositions,
    /// The degrade reasons observed.
    DegradeReasons,
    /// The qualification class.
    Qualification,
    /// The auth handoff posture named by the card.
    HandoffPosture,
    /// The handoff reason named by the card.
    HandoffReason,
    /// The target / service identity named by the dashboard header.
    ServiceIdentity,
    /// The freshness / offline state named by the dashboard header.
    FreshnessState,
    /// The fallback state named by the auth card.
    FallbackState,
    /// The accountable owner role.
    OwnerRole,
}

impl M5AuthDashboardExportField {
    /// Every export field, in declaration order.
    pub const ALL: [Self; 11] = [
        Self::ConsumerSurface,
        Self::ComponentFamilies,
        Self::BoundaryDispositions,
        Self::DegradeReasons,
        Self::Qualification,
        Self::HandoffPosture,
        Self::HandoffReason,
        Self::ServiceIdentity,
        Self::FreshnessState,
        Self::FallbackState,
        Self::OwnerRole,
    ];

    /// The five mandatory export fields.
    pub const MANDATORY: [Self; 5] = [
        Self::ConsumerSurface,
        Self::ComponentFamilies,
        Self::BoundaryDispositions,
        Self::DegradeReasons,
        Self::Qualification,
    ];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ConsumerSurface => "consumer_surface",
            Self::ComponentFamilies => "component_families",
            Self::BoundaryDispositions => "boundary_dispositions",
            Self::DegradeReasons => "degrade_reasons",
            Self::Qualification => "qualification",
            Self::HandoffPosture => "handoff_posture",
            Self::HandoffReason => "handoff_reason",
            Self::ServiceIdentity => "service_identity",
            Self::FreshnessState => "freshness_state",
            Self::FallbackState => "fallback_state",
            Self::OwnerRole => "owner_role",
        }
    }
}

/// Reason an auth handoff card degraded below a clean, fully-legible state. The degrade-first
/// ladder returns one of these instead of ever letting a security-theater card read as a clean
/// pass.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5AuthHandoffCardDegradeReason {
    /// The provider or domain is unstated (AC1 violation).
    ProviderOrDomainUnstated,
    /// The reason for the handoff is unexplained (AC1 violation).
    ReasonForHandoffUnstated,
    /// The local-safe continuity note is missing — the user cannot tell which local state remains
    /// intact while the handoff completes (AC1 violation).
    LocalContinuityUnstated,
    /// The fallback state (what survives if the handoff is blocked) is unstated.
    FallbackStateUnstated,
    /// A device-code posture omits its code or expiry disclosure.
    DeviceCodeOrExpiryUnstated,
    /// The embedded surface imitates native permission or approval chrome (guardrail).
    ImitatesNativeApprovalUi,
    /// A high-risk approval is embedded without a native step-up (guardrail).
    HighRiskApprovalEmbeddedWithoutStepUp,
    /// The supporting proof packet has gone stale.
    ProofStale,
}

impl M5AuthHandoffCardDegradeReason {
    /// Every degrade reason, in declaration order.
    pub const ALL: [Self; 8] = [
        Self::ProviderOrDomainUnstated,
        Self::ReasonForHandoffUnstated,
        Self::LocalContinuityUnstated,
        Self::FallbackStateUnstated,
        Self::DeviceCodeOrExpiryUnstated,
        Self::ImitatesNativeApprovalUi,
        Self::HighRiskApprovalEmbeddedWithoutStepUp,
        Self::ProofStale,
    ];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ProviderOrDomainUnstated => "provider_or_domain_unstated",
            Self::ReasonForHandoffUnstated => "reason_for_handoff_unstated",
            Self::LocalContinuityUnstated => "local_continuity_unstated",
            Self::FallbackStateUnstated => "fallback_state_unstated",
            Self::DeviceCodeOrExpiryUnstated => "device_code_or_expiry_unstated",
            Self::ImitatesNativeApprovalUi => "imitates_native_approval_ui",
            Self::HighRiskApprovalEmbeddedWithoutStepUp => {
                "high_risk_approval_embedded_without_step_up"
            }
            Self::ProofStale => "proof_stale",
        }
    }

    /// Next safe action for this reason.
    pub const fn next_action(self) -> M5AuthDashboardNextAction {
        match self {
            Self::ProviderOrDomainUnstated
            | Self::ReasonForHandoffUnstated
            | Self::ImitatesNativeApprovalUi
            | Self::HighRiskApprovalEmbeddedWithoutStepUp
            | Self::ProofStale => M5AuthDashboardNextAction::ReviewDiagnostics,
            Self::LocalContinuityUnstated => M5AuthDashboardNextAction::ReviewLocalContinuity,
            Self::FallbackStateUnstated | Self::DeviceCodeOrExpiryUnstated => {
                M5AuthDashboardNextAction::RetryHandoff
            }
        }
    }

    /// The matching downgrade trigger recorded on the frozen matrix.
    pub const fn downgrade_trigger(self) -> M5EmbeddedDowngradeTrigger {
        match self {
            Self::ProviderOrDomainUnstated => M5EmbeddedDowngradeTrigger::OwnerOrOriginUnstated,
            Self::ReasonForHandoffUnstated => M5EmbeddedDowngradeTrigger::GenericChromeWordingUsed,
            Self::LocalContinuityUnstated | Self::FallbackStateUnstated => {
                M5EmbeddedDowngradeTrigger::BrowserFallbackHiddenInMenusOnly
            }
            Self::DeviceCodeOrExpiryUnstated => {
                M5EmbeddedDowngradeTrigger::FreshnessOrLastUpdatedUnstated
            }
            Self::ImitatesNativeApprovalUi => {
                M5EmbeddedDowngradeTrigger::ImitatesNativeApprovalChrome
            }
            Self::HighRiskApprovalEmbeddedWithoutStepUp => {
                M5EmbeddedDowngradeTrigger::HighRiskApprovalEmbedded
            }
            Self::ProofStale => M5EmbeddedDowngradeTrigger::ProofStale,
        }
    }
}

/// Reason a remote/service dashboard header degraded below a clean, fully-legible state.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5RemoteServiceDashboardHeaderDegradeReason {
    /// The target / service identity is unstated (AC2 violation).
    ServiceIdentityUnstated,
    /// The ownership boundary (owner/origin) is undisclosed (AC2 violation).
    OwnershipBoundaryUnstated,
    /// The freshness / offline state is hidden (AC2 violation).
    FreshnessOrOfflineUnstated,
    /// The dashboard substitutes for the primary local recovery controls (AC2 violation).
    SubstitutesForLocalRecovery,
    /// The export / open-console actions are unavailable.
    ExportOrConsoleActionUnavailable,
    /// A high-risk approval is allowed inside embedded chrome (guardrail).
    HighRiskApprovalInEmbeddedChrome,
    /// The supporting proof packet has gone stale.
    ProofStale,
}

impl M5RemoteServiceDashboardHeaderDegradeReason {
    /// Every degrade reason, in declaration order.
    pub const ALL: [Self; 7] = [
        Self::ServiceIdentityUnstated,
        Self::OwnershipBoundaryUnstated,
        Self::FreshnessOrOfflineUnstated,
        Self::SubstitutesForLocalRecovery,
        Self::ExportOrConsoleActionUnavailable,
        Self::HighRiskApprovalInEmbeddedChrome,
        Self::ProofStale,
    ];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ServiceIdentityUnstated => "service_identity_unstated",
            Self::OwnershipBoundaryUnstated => "ownership_boundary_unstated",
            Self::FreshnessOrOfflineUnstated => "freshness_or_offline_unstated",
            Self::SubstitutesForLocalRecovery => "substitutes_for_local_recovery",
            Self::ExportOrConsoleActionUnavailable => "export_or_console_action_unavailable",
            Self::HighRiskApprovalInEmbeddedChrome => "high_risk_approval_in_embedded_chrome",
            Self::ProofStale => "proof_stale",
        }
    }

    /// Next safe action for this reason.
    pub const fn next_action(self) -> M5AuthDashboardNextAction {
        match self {
            Self::ServiceIdentityUnstated
            | Self::OwnershipBoundaryUnstated
            | Self::FreshnessOrOfflineUnstated
            | Self::HighRiskApprovalInEmbeddedChrome
            | Self::ProofStale => M5AuthDashboardNextAction::ReviewDiagnostics,
            Self::SubstitutesForLocalRecovery => M5AuthDashboardNextAction::ReviewLocalContinuity,
            Self::ExportOrConsoleActionUnavailable => {
                M5AuthDashboardNextAction::ExportOrOpenConsole
            }
        }
    }

    /// The matching downgrade trigger recorded on the frozen matrix.
    pub const fn downgrade_trigger(self) -> M5EmbeddedDowngradeTrigger {
        match self {
            Self::ServiceIdentityUnstated | Self::OwnershipBoundaryUnstated => {
                M5EmbeddedDowngradeTrigger::OwnerOrOriginUnstated
            }
            Self::FreshnessOrOfflineUnstated => {
                M5EmbeddedDowngradeTrigger::FreshnessOrLastUpdatedUnstated
            }
            Self::SubstitutesForLocalRecovery => {
                M5EmbeddedDowngradeTrigger::GenericChromeWordingUsed
            }
            Self::ExportOrConsoleActionUnavailable => {
                M5EmbeddedDowngradeTrigger::BrowserFallbackHiddenInMenusOnly
            }
            Self::HighRiskApprovalInEmbeddedChrome => {
                M5EmbeddedDowngradeTrigger::HighRiskApprovalEmbedded
            }
            Self::ProofStale => M5EmbeddedDowngradeTrigger::ProofStale,
        }
    }
}

/// Maps an auth handoff posture to the single controlled boundary disposition. An embedded sign-in
/// checkpoint is capability-limited in-product chrome; every external handoff is browser-handoff
/// only.
fn disposition_for_posture(posture: M5AuthHandoffPosture) -> M5EmbeddedBoundaryDisposition {
    if posture.is_embedded_checkpoint() {
        M5EmbeddedBoundaryDisposition::CapabilityLimited
    } else {
        M5EmbeddedBoundaryDisposition::BrowserHandoffOnly
    }
}

/// Maps a dashboard header's owner class and freshness to the single controlled boundary
/// disposition.
fn disposition_for_dashboard(
    owner: WebviewOwnerClass,
    freshness: M5EmbeddedFreshnessState,
) -> M5EmbeddedBoundaryDisposition {
    use M5EmbeddedBoundaryDisposition as D;
    match freshness {
        M5EmbeddedFreshnessState::StaleSnapshot => D::StaleSnapshot,
        M5EmbeddedFreshnessState::OfflineSnapshot => D::OfflineSnapshot,
        _ => match owner {
            WebviewOwnerClass::ExtensionOwned => D::CapabilityLimited,
            WebviewOwnerClass::ProviderOwned => D::LiveProviderOwned,
            WebviewOwnerClass::FirstPartyEmbedded => D::LiveFirstPartyHosted,
            WebviewOwnerClass::UnknownUntrusted => D::NotEvaluated,
        },
    }
}

/// Input to [`resolve_auth_handoff_card`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M5AuthHandoffCardResolutionInput {
    /// Stable identity of the card instance.
    pub card_id: String,
    /// How the sign-in / authorization reaches its provider.
    pub posture: M5AuthHandoffPosture,
    /// The browser-handoff kind the card describes.
    pub handoff_kind: BrowserHandoffKind,
    /// Why the handoff happens.
    pub handoff_reason: HandoffReasonClass,
    /// Reviewer-facing provider label (empty means unstated).
    pub provider_label: String,
    /// Reviewer-facing provider domain label, no scheme or credentials (empty means unstated).
    pub provider_domain_label: String,
    /// True when the reason for the handoff is explained on the card.
    pub reason_stated: bool,
    /// What survives if the handoff is blocked or the browser cannot open.
    pub fallback_state: FallbackStateClass,
    /// True when the fallback state is disclosed on the card.
    pub fallback_stated: bool,
    /// True when the local-safe continuity note (which local state remains intact) is explicit.
    pub local_continuity_stated: bool,
    /// How the device code's expiry is disclosed (for a device-code posture).
    pub expiry_disclosure: ExpiryDisclosureClass,
    /// True when the code or expiry disclosure is present where a device-code posture applies.
    pub device_code_stated: bool,
    /// True when the embedded surface imitates native permission or approval chrome (guardrail).
    pub imitates_native_approval_ui: bool,
    /// True when a high-risk approval is embedded without a native step-up (guardrail).
    pub embeds_high_risk_approval_without_step_up: bool,
    /// True when the supporting proof packet is fresh.
    pub proof_fresh: bool,
}

/// Resolved, export-safe auth handoff card projection.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ResolvedAuthHandoffCard {
    /// Stable identity of the card instance.
    pub card_id: String,
    /// Auth handoff posture token named by the card.
    pub posture: M5AuthHandoffPosture,
    /// Browser-handoff-kind token named by the card.
    pub handoff_kind: String,
    /// Handoff-reason token named by the card.
    pub handoff_reason: String,
    /// Provider label named by the card.
    pub provider_label: String,
    /// Provider domain label named by the card.
    pub provider_domain_label: String,
    /// Single controlled boundary disposition carried by the card.
    pub boundary_disposition: M5EmbeddedBoundaryDisposition,
    /// Fallback-state token named by the card.
    pub fallback_state: String,
    /// Expiry-disclosure token named by the card.
    pub expiry_disclosure: String,
    /// AC1: whether the provider or domain is named on the card.
    pub provider_or_domain_stated: bool,
    /// AC1: whether the reason for the handoff is explained.
    pub reason_stated: bool,
    /// AC1: whether the local-safe continuity note is explicit.
    pub local_continuity_stated: bool,
    /// AC1: whether the fallback state is disclosed.
    pub fallback_stated: bool,
    /// AC1: whether the device-code / expiry disclosure is present where the posture requires it.
    pub device_code_disclosure_present: bool,
    /// Degrade reason, if the card could not read as a clean, fully-legible state.
    pub degrade_reason: Option<M5AuthHandoffCardDegradeReason>,
    /// Next safe action offered.
    pub next_action: M5AuthDashboardNextAction,
    /// Guardrail (MUST be `false` on a clean card): the embedded surface imitates native permission
    /// or approval chrome.
    pub imitates_native_approval_ui: bool,
    /// Guardrail (MUST be `false` on a clean card): a high-risk approval is embedded without a
    /// native step-up.
    pub embeds_high_risk_approval_without_step_up: bool,
}

impl M5ResolvedAuthHandoffCard {
    /// Whether this card reads as a clean, fully-legible state.
    pub fn is_clean(&self) -> bool {
        self.degrade_reason.is_none()
    }

    /// Whether this card hides which local state remains intact or imitates native approval chrome
    /// (an AC1 violation).
    pub fn hides_continuity_or_imitates(&self) -> bool {
        !self.local_continuity_stated
            || self.imitates_native_approval_ui
            || self.embeds_high_risk_approval_without_step_up
    }
}

/// Input to [`resolve_remote_service_dashboard_header`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M5RemoteServiceDashboardHeaderResolutionInput {
    /// Stable identity of the header instance.
    pub header_id: String,
    /// The owner / origin class (service ownership) behind the dashboard.
    pub owner_class: WebviewOwnerClass,
    /// True when the owner / origin is disclosed on the header, never menu-only.
    pub owner_origin_disclosed: bool,
    /// Reviewer-facing target / service identity label (empty means unstated).
    pub service_identity_label: String,
    /// True when the target / service identity is disclosed on the header.
    pub service_identity_stated: bool,
    /// The freshness / offline state of the dashboard.
    pub freshness: M5EmbeddedFreshnessState,
    /// True when the freshness / offline state is disclosed on the header.
    pub freshness_stated: bool,
    /// True when an export action is offered on the header.
    pub export_action_available: bool,
    /// True when an open-console action is offered on the header.
    pub open_console_action_available: bool,
    /// True when the primary local recovery controls stay reachable alongside the dashboard.
    pub primary_local_recovery_available: bool,
    /// True when the dashboard substitutes for the primary local recovery controls (guardrail).
    pub substitutes_for_local_recovery: bool,
    /// True when a high-risk approval is allowed inside embedded chrome (guardrail).
    pub allows_high_risk_approval_in_embedded_chrome: bool,
    /// True when the supporting proof packet is fresh.
    pub proof_fresh: bool,
}

/// Resolved, export-safe remote/service dashboard header projection.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ResolvedRemoteServiceDashboardHeader {
    /// Stable identity of the header instance.
    pub header_id: String,
    /// Owner / origin token named by the header.
    pub owner_origin: String,
    /// Target / service identity label named by the header.
    pub service_identity_label: String,
    /// Freshness-state token named by the header.
    pub freshness: String,
    /// Single controlled boundary disposition carried by the header.
    pub boundary_disposition: M5EmbeddedBoundaryDisposition,
    /// AC2: whether the target / service identity is disclosed.
    pub service_identity_stated: bool,
    /// AC2: whether the owner / origin (ownership boundary) is disclosed.
    pub owner_origin_disclosed: bool,
    /// AC2: whether the freshness / offline state is disclosed.
    pub freshness_stated: bool,
    /// AC2: whether the primary local recovery controls stay reachable.
    pub primary_local_recovery_available: bool,
    /// Whether an export action is offered.
    pub export_action_available: bool,
    /// Whether an open-console action is offered.
    pub open_console_action_available: bool,
    /// Degrade reason, if the header could not read as a clean, fully-legible state.
    pub degrade_reason: Option<M5RemoteServiceDashboardHeaderDegradeReason>,
    /// Next safe action offered.
    pub next_action: M5AuthDashboardNextAction,
    /// Guardrail (MUST be `false` on a clean header): the dashboard substitutes for the primary
    /// local recovery controls.
    pub substitutes_for_local_recovery: bool,
    /// Guardrail (MUST be `false` on a clean header): a high-risk approval is allowed inside
    /// embedded chrome.
    pub allows_high_risk_approval_in_embedded_chrome: bool,
}

impl M5ResolvedRemoteServiceDashboardHeader {
    /// Whether this header reads as a clean, fully-legible state.
    pub fn is_clean(&self) -> bool {
        self.degrade_reason.is_none()
    }

    /// Whether this header substitutes for local recovery or hides its freshness / ownership
    /// boundaries (an AC2 violation).
    pub fn substitutes_or_hides_boundaries(&self) -> bool {
        self.substitutes_for_local_recovery
            || !self.freshness_stated
            || !self.owner_origin_disclosed
            || !self.service_identity_stated
    }
}

/// Error emitted when a resolver input carries invalid or forbidden material.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum M5AuthDashboardResolutionError {
    /// The auth-handoff-card id was empty.
    EmptyCardId,
    /// The dashboard-header id was empty.
    EmptyHeaderId,
    /// A field carried forbidden raw material (secret / endpoint).
    ForbiddenMaterial,
}

impl M5AuthDashboardResolutionError {
    /// Stable token used in tests and exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::EmptyCardId => "empty_card_id",
            Self::EmptyHeaderId => "empty_header_id",
            Self::ForbiddenMaterial => "forbidden_material",
        }
    }
}

impl fmt::Display for M5AuthDashboardResolutionError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "m5 auth/dashboard resolution error: {}",
            self.as_str()
        )
    }
}

impl Error for M5AuthDashboardResolutionError {}

/// Resolves an auth handoff card, proving AC1: users can distinguish embedded sign-in checkpoints
/// from system-browser or passkey handoff and know which local state remains intact while the
/// handoff completes. A clean card names its posture, provider/domain, reason, fallback state,
/// local-continuity note, and — under a device-code posture — its code or expiry disclosure, and
/// never imitates native approval chrome or embeds a high-risk approval without a native step-up.
pub fn resolve_auth_handoff_card(
    input: M5AuthHandoffCardResolutionInput,
) -> Result<M5ResolvedAuthHandoffCard, M5AuthDashboardResolutionError> {
    if input.card_id.trim().is_empty() {
        return Err(M5AuthDashboardResolutionError::EmptyCardId);
    }
    if string_is_forbidden(&input.card_id)
        || string_is_forbidden(&input.provider_label)
        || string_is_forbidden(&input.provider_domain_label)
    {
        return Err(M5AuthDashboardResolutionError::ForbiddenMaterial);
    }

    let provider_or_domain_stated =
        !input.provider_label.trim().is_empty() || !input.provider_domain_label.trim().is_empty();
    let device_code_required = input.posture.requires_device_code_disclosure()
        || input.handoff_kind.requires_device_code_disclosure();
    let device_code_disclosure_present = if device_code_required {
        input.device_code_stated && input.expiry_disclosure.discloses_expiry()
    } else {
        true
    };

    let degrade_reason = if !provider_or_domain_stated {
        Some(M5AuthHandoffCardDegradeReason::ProviderOrDomainUnstated)
    } else if !input.reason_stated {
        Some(M5AuthHandoffCardDegradeReason::ReasonForHandoffUnstated)
    } else if !input.local_continuity_stated {
        Some(M5AuthHandoffCardDegradeReason::LocalContinuityUnstated)
    } else if !input.fallback_stated {
        Some(M5AuthHandoffCardDegradeReason::FallbackStateUnstated)
    } else if !device_code_disclosure_present {
        Some(M5AuthHandoffCardDegradeReason::DeviceCodeOrExpiryUnstated)
    } else if input.imitates_native_approval_ui {
        Some(M5AuthHandoffCardDegradeReason::ImitatesNativeApprovalUi)
    } else if input.embeds_high_risk_approval_without_step_up {
        Some(M5AuthHandoffCardDegradeReason::HighRiskApprovalEmbeddedWithoutStepUp)
    } else if !input.proof_fresh {
        Some(M5AuthHandoffCardDegradeReason::ProofStale)
    } else {
        None
    };

    let boundary_disposition = match degrade_reason {
        Some(_) => M5EmbeddedBoundaryDisposition::NotEvaluated,
        None => disposition_for_posture(input.posture),
    };
    let next_action = match degrade_reason {
        Some(reason) => reason.next_action(),
        None => M5AuthDashboardNextAction::NoActionNeeded,
    };

    Ok(M5ResolvedAuthHandoffCard {
        card_id: input.card_id,
        posture: input.posture,
        handoff_kind: input.handoff_kind.as_str().to_owned(),
        handoff_reason: input.handoff_reason.as_str().to_owned(),
        provider_label: input.provider_label,
        provider_domain_label: input.provider_domain_label,
        boundary_disposition,
        fallback_state: input.fallback_state.as_str().to_owned(),
        expiry_disclosure: input.expiry_disclosure.as_str().to_owned(),
        provider_or_domain_stated,
        reason_stated: input.reason_stated,
        local_continuity_stated: input.local_continuity_stated,
        fallback_stated: input.fallback_stated,
        device_code_disclosure_present,
        degrade_reason,
        next_action,
        imitates_native_approval_ui: input.imitates_native_approval_ui,
        embeds_high_risk_approval_without_step_up: input.embeds_high_risk_approval_without_step_up,
    })
}

/// Resolves a remote/service dashboard header, proving AC2: a remote or service dashboard never
/// substitutes for the primary local recovery controls or hides its freshness and ownership
/// boundaries. A clean header names its target/service identity, owner/origin, freshness/offline
/// state, and export/open-console actions, keeps the primary local recovery controls reachable, and
/// never allows a high-risk approval inside embedded chrome.
pub fn resolve_remote_service_dashboard_header(
    input: M5RemoteServiceDashboardHeaderResolutionInput,
) -> Result<M5ResolvedRemoteServiceDashboardHeader, M5AuthDashboardResolutionError> {
    if input.header_id.trim().is_empty() {
        return Err(M5AuthDashboardResolutionError::EmptyHeaderId);
    }
    if string_is_forbidden(&input.header_id) || string_is_forbidden(&input.service_identity_label) {
        return Err(M5AuthDashboardResolutionError::ForbiddenMaterial);
    }

    let service_identity_stated =
        input.service_identity_stated && !input.service_identity_label.trim().is_empty();
    let owner_origin_disclosed =
        input.owner_origin_disclosed && input.owner_class != WebviewOwnerClass::UnknownUntrusted;
    let freshness_stated =
        input.freshness_stated && input.freshness != M5EmbeddedFreshnessState::FreshnessUnknown;
    let export_or_console_available =
        input.export_action_available || input.open_console_action_available;

    let degrade_reason = if !service_identity_stated {
        Some(M5RemoteServiceDashboardHeaderDegradeReason::ServiceIdentityUnstated)
    } else if !owner_origin_disclosed {
        Some(M5RemoteServiceDashboardHeaderDegradeReason::OwnershipBoundaryUnstated)
    } else if !freshness_stated {
        Some(M5RemoteServiceDashboardHeaderDegradeReason::FreshnessOrOfflineUnstated)
    } else if input.substitutes_for_local_recovery || !input.primary_local_recovery_available {
        Some(M5RemoteServiceDashboardHeaderDegradeReason::SubstitutesForLocalRecovery)
    } else if !export_or_console_available {
        Some(M5RemoteServiceDashboardHeaderDegradeReason::ExportOrConsoleActionUnavailable)
    } else if input.allows_high_risk_approval_in_embedded_chrome {
        Some(M5RemoteServiceDashboardHeaderDegradeReason::HighRiskApprovalInEmbeddedChrome)
    } else if !input.proof_fresh {
        Some(M5RemoteServiceDashboardHeaderDegradeReason::ProofStale)
    } else {
        None
    };

    let boundary_disposition = match degrade_reason {
        Some(_) => M5EmbeddedBoundaryDisposition::NotEvaluated,
        None => disposition_for_dashboard(input.owner_class, input.freshness),
    };
    let next_action = match degrade_reason {
        Some(reason) => reason.next_action(),
        None => M5AuthDashboardNextAction::NoActionNeeded,
    };

    Ok(M5ResolvedRemoteServiceDashboardHeader {
        header_id: input.header_id,
        owner_origin: input.owner_class.as_str().to_owned(),
        service_identity_label: input.service_identity_label,
        freshness: input.freshness.as_str().to_owned(),
        boundary_disposition,
        service_identity_stated,
        owner_origin_disclosed,
        freshness_stated,
        primary_local_recovery_available: input.primary_local_recovery_available,
        export_action_available: input.export_action_available,
        open_console_action_available: input.open_console_action_available,
        degrade_reason,
        next_action,
        substitutes_for_local_recovery: input.substitutes_for_local_recovery,
        allows_high_risk_approval_in_embedded_chrome: input
            .allows_high_risk_approval_in_embedded_chrome,
    })
}

/// One controls row: one consumer surface bound to the resolved auth handoff card and remote/service
/// dashboard header examples it must project honestly.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5AuthDashboardControlsRow {
    /// Consumer surface this row projects onto.
    pub consumer_surface: M5AuthDashboardConsumerSurface,
    /// Qualification class earned by this row.
    pub qualification: M5EmbeddedQualificationClass,
    /// Owner role accountable for keeping this row honest.
    pub owner_role: String,
    /// Human-readable scope summary.
    pub scope_summary: String,
    /// Deployment lines this row keeps the same truth across.
    pub deployment_lines: Vec<M5EmbeddedDeploymentLine>,
    /// Mandatory labels this row must be able to show.
    pub required_labels: Vec<M5EmbeddedRequiredLabel>,
    /// Non-visual accessibility routes offered.
    pub accessibility_routes: Vec<M5EmbeddedAccessibilityRoute>,
    /// Anatomy parts this row must be able to show (must include the mandatory three).
    pub anatomy_parts: Vec<M5AuthDashboardAnatomyPart>,
    /// Export fields exposed (must include the mandatory five).
    pub export_fields: Vec<M5AuthDashboardExportField>,
    /// Downgrade triggers that apply to this row.
    pub downgrade_triggers: Vec<M5EmbeddedDowngradeTrigger>,
    /// Resolved auth handoff card examples.
    pub auth_handoff_card_examples: Vec<M5ResolvedAuthHandoffCard>,
    /// Resolved remote/service dashboard header examples.
    pub remote_service_dashboard_header_examples: Vec<M5ResolvedRemoteServiceDashboardHeader>,
    /// Proof packet refs that keep this row current.
    pub required_proof_packet_refs: Vec<String>,
    /// Source contract refs consumed by this row (must include both component schemas).
    pub source_contract_refs: Vec<String>,
    /// Hard invariant: never masquerade as native permission or irreversible approval UI.
    pub masquerades_as_native_approval_chrome: bool,
    /// Hard invariant: never hide owner/origin or the browser handoff behind menus only.
    pub hides_owner_origin_or_handoff_in_menus_only: bool,
    /// Hard invariant: never render a stale, offline, or blocked pane as fresh first-party truth.
    pub renders_stale_or_blocked_as_fresh_first_party_truth: bool,
    /// Hard invariant: never embed a high-risk approval without a native step-up.
    pub embeds_high_risk_approval_without_native_step_up: bool,
}

impl M5AuthDashboardControlsRow {
    fn declares_mandatory_anatomy(&self) -> bool {
        let present: BTreeSet<M5AuthDashboardAnatomyPart> =
            self.anatomy_parts.iter().copied().collect();
        M5AuthDashboardAnatomyPart::MANDATORY
            .iter()
            .all(|part| present.contains(part))
    }

    fn declares_mandatory_export_fields(&self) -> bool {
        let present: BTreeSet<M5AuthDashboardExportField> =
            self.export_fields.iter().copied().collect();
        M5AuthDashboardExportField::MANDATORY
            .iter()
            .all(|field| present.contains(field))
    }

    fn honours_invariants(&self) -> bool {
        !self.masquerades_as_native_approval_chrome
            && !self.hides_owner_origin_or_handoff_in_menus_only
            && !self.renders_stale_or_blocked_as_fresh_first_party_truth
            && !self.embeds_high_risk_approval_without_native_step_up
    }

    /// True when every resolved example on this row is honest: no clean auth card hides which local
    /// state remains intact or imitates native approval chrome, and no clean dashboard header
    /// substitutes for local recovery or hides its freshness / ownership boundaries.
    fn examples_are_honest(&self) -> bool {
        self.auth_handoff_card_examples
            .iter()
            .all(|ex| !(ex.is_clean() && ex.hides_continuity_or_imitates()))
            && self
                .remote_service_dashboard_header_examples
                .iter()
                .all(|ex| !(ex.is_clean() && ex.substitutes_or_hides_boundaries()))
    }
}

/// Self-describing controlled-vocabulary set frozen by the controls packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5AuthDashboardVocabularySet {
    /// Boundary-disposition tokens (bound from the frozen matrix).
    pub boundary_dispositions: Vec<String>,
    /// Auth handoff posture tokens.
    pub handoff_postures: Vec<String>,
    /// Owner-class tokens (bound from the auth-boundary object model).
    pub owner_classes: Vec<String>,
    /// Browser-handoff-kind tokens (bound from the auth-boundary object model).
    pub browser_handoff_kinds: Vec<String>,
    /// Handoff-reason tokens (bound from the auth-boundary object model).
    pub handoff_reasons: Vec<String>,
    /// Fallback-state tokens (bound from the auth-boundary object model).
    pub fallback_states: Vec<String>,
    /// Expiry-disclosure tokens (bound from the auth-boundary object model).
    pub expiry_disclosures: Vec<String>,
    /// Freshness-state tokens (bound from the frozen matrix).
    pub freshness_states: Vec<String>,
    /// Auth-handoff-card degrade-reason tokens.
    pub auth_card_degrade_reasons: Vec<String>,
    /// Dashboard-header degrade-reason tokens.
    pub dashboard_header_degrade_reasons: Vec<String>,
    /// Anatomy-part tokens.
    pub anatomy_parts: Vec<String>,
    /// Next-action tokens.
    pub next_actions: Vec<String>,
    /// Export-field tokens.
    pub export_fields: Vec<String>,
    /// Consumer-surface tokens.
    pub consumer_surfaces: Vec<String>,
}

impl M5AuthDashboardVocabularySet {
    /// Builds the canonical vocabulary set from the typed `ALL` arrays.
    pub fn canonical() -> Self {
        Self {
            boundary_dispositions: tokens(&M5EmbeddedBoundaryDisposition::ALL, |v| v.as_str()),
            handoff_postures: tokens(&M5AuthHandoffPosture::ALL, |v| v.as_str()),
            owner_classes: tokens(&WebviewOwnerClass::ALL, |v| v.as_str()),
            browser_handoff_kinds: tokens(&BrowserHandoffKind::ALL, |v| v.as_str()),
            handoff_reasons: tokens(&BOUND_HANDOFF_REASONS, |v| v.as_str()),
            fallback_states: tokens(&BOUND_FALLBACK_STATES, |v| v.as_str()),
            expiry_disclosures: tokens(&BOUND_EXPIRY_DISCLOSURES, |v| v.as_str()),
            freshness_states: tokens(&M5EmbeddedFreshnessState::ALL, |v| v.as_str()),
            auth_card_degrade_reasons: tokens(&M5AuthHandoffCardDegradeReason::ALL, |v| v.as_str()),
            dashboard_header_degrade_reasons: tokens(
                &M5RemoteServiceDashboardHeaderDegradeReason::ALL,
                |v| v.as_str(),
            ),
            anatomy_parts: tokens(&M5AuthDashboardAnatomyPart::ALL, |v| v.as_str()),
            next_actions: tokens(&M5AuthDashboardNextAction::ALL, |v| v.as_str()),
            export_fields: tokens(&M5AuthDashboardExportField::ALL, |v| v.as_str()),
            consumer_surfaces: tokens(&M5EmbeddedConsumerSurface::ALL, |v| v.as_str()),
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
pub struct M5AuthDashboardGovernanceReview {
    /// The auth card always distinguishes embedded checkpoints from browser / passkey handoff.
    pub card_distinguishes_checkpoint_from_handoff: bool,
    /// The auth card always names its provider or domain and reason for handoff.
    pub card_names_provider_and_reason: bool,
    /// The auth card always keeps the local-safe continuity note explicit.
    pub card_keeps_local_continuity_explicit: bool,
    /// The auth card always discloses its code or expiry under a device-code posture.
    pub card_discloses_device_code_or_expiry: bool,
    /// No auth card imitates native permission or approval chrome.
    pub card_never_imitates_native_approval: bool,
    /// The dashboard header always names its target / service identity and ownership boundary.
    pub header_names_identity_and_ownership: bool,
    /// The dashboard header always discloses its freshness / offline state.
    pub header_discloses_freshness: bool,
    /// The dashboard never substitutes for the primary local recovery controls.
    pub dashboard_never_substitutes_for_local_recovery: bool,
    /// No embedded chrome performs a high-risk approval without a native step-up.
    pub no_high_risk_approval_in_embedded_chrome: bool,
    /// The lane reuses the frozen matrix vocabulary rather than inventing parallel wording.
    pub reuses_frozen_matrix_vocabulary: bool,
}

/// Consumer projection block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5AuthDashboardConsumerProjection {
    /// Auth surfaces consume the shared handoff-posture and reason vocabulary.
    pub surfaces_consume_handoff_posture_vocabulary: bool,
    /// Dashboard surfaces consume the shared owner/origin and freshness vocabulary.
    pub surfaces_consume_owner_freshness_vocabulary: bool,
    /// Auth cards consume the shared fallback and expiry vocabulary.
    pub cards_consume_shared_fallback_expiry_vocabulary: bool,
    /// Support / export reads a single canonical boundary source.
    pub support_export_reads_single_boundary_source: bool,
}

/// Proof freshness block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5AuthDashboardProofFreshness {
    /// Proof-freshness SLO in hours.
    pub proof_freshness_slo_hours: u32,
    /// RFC 3339 timestamp of the last proof refresh.
    pub last_proof_refresh: String,
    /// True when stale proof automatically narrows the component.
    pub auto_narrow_on_stale: bool,
}

/// Release and support parity posture for the controls lane.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5AuthDashboardReleasePosture {
    /// Ref of the supporting proof packet for the lane.
    pub proof_packet_ref: String,
    /// Ref of the supporting boundary audit for the lane.
    pub boundary_audit_ref: String,
    /// True when support/export parity is required for every row.
    pub support_export_parity_required: bool,
    /// True when accessibility parity is required for every row.
    pub accessibility_parity_required: bool,
}

/// Constructor input for [`M5AuthDashboardControlsPacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M5AuthDashboardControlsPacketInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable controls label.
    pub controls_label: String,
    /// Controls rows.
    pub controls_rows: Vec<M5AuthDashboardControlsRow>,
    /// Frozen controlled-vocabulary set.
    pub vocabulary_set: M5AuthDashboardVocabularySet,
    /// Governance-review block.
    pub governance_review: M5AuthDashboardGovernanceReview,
    /// Consumer projection block.
    pub consumer_projection: M5AuthDashboardConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: M5AuthDashboardProofFreshness,
    /// Release and support parity posture.
    pub release_posture: M5AuthDashboardReleasePosture,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

/// Export-safe M5 auth handoff-card and remote/service dashboard-header controls packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5AuthDashboardControlsPacket {
    /// Record kind; must equal [`M5_AUTH_DASHBOARD_CONTROLS_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`M5_AUTH_DASHBOARD_CONTROLS_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable controls label.
    pub controls_label: String,
    /// Controls rows.
    pub controls_rows: Vec<M5AuthDashboardControlsRow>,
    /// Frozen controlled-vocabulary set.
    pub vocabulary_set: M5AuthDashboardVocabularySet,
    /// Governance-review block.
    pub governance_review: M5AuthDashboardGovernanceReview,
    /// Consumer projection block.
    pub consumer_projection: M5AuthDashboardConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: M5AuthDashboardProofFreshness,
    /// Release and support parity posture.
    pub release_posture: M5AuthDashboardReleasePosture,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

impl M5AuthDashboardControlsPacket {
    /// Builds a controls packet from stable-lane input.
    pub fn new(input: M5AuthDashboardControlsPacketInput) -> Self {
        Self {
            record_kind: M5_AUTH_DASHBOARD_CONTROLS_RECORD_KIND.to_owned(),
            schema_version: M5_AUTH_DASHBOARD_CONTROLS_SCHEMA_VERSION,
            packet_id: input.packet_id,
            controls_label: input.controls_label,
            controls_rows: input.controls_rows,
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

    /// Validates the controls-packet invariants.
    pub fn validate(&self) -> Vec<M5AuthDashboardControlsViolation> {
        let mut violations = Vec::new();

        if self.record_kind != M5_AUTH_DASHBOARD_CONTROLS_RECORD_KIND {
            violations.push(M5AuthDashboardControlsViolation::WrongRecordKind);
        }
        if self.schema_version != M5_AUTH_DASHBOARD_CONTROLS_SCHEMA_VERSION {
            violations.push(M5AuthDashboardControlsViolation::WrongSchemaVersion);
        }
        if self.packet_id.trim().is_empty()
            || self.controls_label.trim().is_empty()
            || self.redaction_class_token.trim().is_empty()
            || self.minted_at.trim().is_empty()
        {
            violations.push(M5AuthDashboardControlsViolation::MissingIdentity);
        }

        validate_source_contracts(self, &mut violations);
        if !self.vocabulary_set.matches_canonical() {
            violations.push(M5AuthDashboardControlsViolation::VocabularySetDrift);
        }
        validate_controls_rows(self, &mut violations);
        validate_governance_review(self, &mut violations);
        validate_consumer_projection(self, &mut violations);
        validate_proof_freshness(self, &mut violations);
        validate_release_posture(self, &mut violations);
        validate_acceptance_criteria(self, &mut violations);

        if json_contains_forbidden_material(
            &serde_json::to_value(self).expect("m5 auth/dashboard controls packet serializes"),
        ) {
            violations.push(M5AuthDashboardControlsViolation::RawMaterialInExport);
        }

        violations
    }

    /// Deterministic export-safe JSON.
    ///
    /// # Panics
    ///
    /// Panics only if serializing this metadata-only packet fails.
    pub fn export_safe_json(&self) -> String {
        serde_json::to_string_pretty(self).expect("m5 auth/dashboard controls packet serializes")
    }

    /// Deterministic, machine-readable controls CSV: one row per consumer surface.
    pub fn render_matrix_csv(&self) -> String {
        let mut out = String::new();
        out.push_str(
            "consumer_surface,qualification,owner,auth_card_examples,dashboard_header_examples,degrade_reasons,downgrade_triggers\n",
        );
        for row in &self.controls_rows {
            let degrades: Vec<&str> = row
                .auth_handoff_card_examples
                .iter()
                .filter_map(|ex| ex.degrade_reason.map(|r| r.as_str()))
                .chain(
                    row.remote_service_dashboard_header_examples
                        .iter()
                        .filter_map(|ex| ex.degrade_reason.map(|r| r.as_str())),
                )
                .collect();
            out.push_str(&format!(
                "{},{},{},{},{},{},{}\n",
                row.consumer_surface.as_str(),
                row.qualification.as_str(),
                csv_field(&row.owner_role),
                row.auth_handoff_card_examples.len(),
                row.remote_service_dashboard_header_examples.len(),
                degrades.join("|"),
                join_tokens(&row.downgrade_triggers, |v| v.as_str()),
            ));
        }
        out
    }

    /// Deterministic Markdown report for support, docs, or review handoff.
    pub fn render_markdown_summary(&self) -> String {
        let mut out = String::new();
        out.push_str("# M5 Auth Handoff-Card and Remote/Service Dashboard-Header Controls\n\n");
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- Label: `{}`\n", self.controls_label));
        out.push_str(&format!(
            "- Consumer surfaces: {}\n",
            self.controls_rows.len()
        ));
        out.push_str(&format!(
            "- Handoff postures: {}\n",
            self.vocabulary_set.handoff_postures.join(", ")
        ));
        out.push_str(&format!(
            "- Proof freshness SLO: {} hours (last refresh: {})\n",
            self.proof_freshness.proof_freshness_slo_hours, self.proof_freshness.last_proof_refresh
        ));
        out.push_str("\n## Consumer surfaces\n\n");
        for row in &self.controls_rows {
            out.push_str(&format!(
                "- **{}**: `{}`\n",
                row.consumer_surface.as_str(),
                row.qualification.as_str()
            ));
            out.push_str(&format!("  - Owner: {}\n", row.owner_role));
            out.push_str(&format!("  - Scope: {}\n", row.scope_summary));
            out.push_str(&format!(
                "  - Auth-card examples: {} / dashboard-header examples: {}\n",
                row.auth_handoff_card_examples.len(),
                row.remote_service_dashboard_header_examples.len()
            ));
        }
        out
    }
}

/// Errors emitted when reading the checked-in stable controls export.
#[derive(Debug)]
pub enum M5AuthDashboardControlsArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<M5AuthDashboardControlsViolation>),
}

impl fmt::Display for M5AuthDashboardControlsArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(
                    formatter,
                    "m5 auth/dashboard controls export parse failed: {error}"
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
                    "m5 auth/dashboard controls export failed validation: {tokens}"
                )
            }
        }
    }
}

impl Error for M5AuthDashboardControlsArtifactError {}

/// Validation failures emitted by [`M5AuthDashboardControlsPacket::validate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum M5AuthDashboardControlsViolation {
    /// Packet record kind is wrong.
    WrongRecordKind,
    /// Packet schema version is wrong.
    WrongSchemaVersion,
    /// Required identity field is missing.
    MissingIdentity,
    /// Source contract refs are incomplete.
    MissingSourceContracts,
    /// The frozen vocabulary set drifted from the canonical token lists.
    VocabularySetDrift,
    /// The controls packet declares no rows.
    NoControlsRows,
    /// A controls row is incomplete.
    ControlsRowIncomplete,
    /// A controls row omits one of the mandatory anatomy parts.
    MandatoryAnatomyMissing,
    /// A controls row omits one of the mandatory export fields.
    MandatoryExportFieldMissing,
    /// A controls row does not point at both component schemas.
    ComponentSchemaRefMissing,
    /// A controls row carries no resolved examples.
    ExamplesMissing,
    /// A controls row carries a dishonest clean example (an auth card that hides continuity or
    /// imitates native approval, or a dashboard header that substitutes for local recovery or hides
    /// its freshness / ownership boundaries).
    DishonestExample,
    /// A controls row violates a hard invariant.
    RowInvariantViolated,
    /// Governance review does not satisfy required invariants.
    GovernanceReviewIncomplete,
    /// Consumer projection does not satisfy required invariants.
    ConsumerProjectionIncomplete,
    /// Proof freshness block is incomplete.
    ProofFreshnessIncomplete,
    /// Release/support parity posture is incomplete.
    ReleasePostureIncomplete,
    /// AC1 is not proven: clean cards do not distinguish the embedded checkpoint / browser / passkey
    /// postures, no local-continuity-unstated or native-approval-imitation card degrades, or a clean
    /// card hides which local state remains intact.
    Ac1NotProven,
    /// AC2 is not proven: no local-recovery-substitution or freshness-unstated header degrades, or a
    /// clean header substitutes for local recovery or hides its freshness / ownership boundaries.
    Ac2NotProven,
    /// Export contains raw sensitive material.
    RawMaterialInExport,
}

impl M5AuthDashboardControlsViolation {
    /// Stable token used in tests and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WrongRecordKind => "wrong_record_kind",
            Self::WrongSchemaVersion => "wrong_schema_version",
            Self::MissingIdentity => "missing_identity",
            Self::MissingSourceContracts => "missing_source_contracts",
            Self::VocabularySetDrift => "vocabulary_set_drift",
            Self::NoControlsRows => "no_controls_rows",
            Self::ControlsRowIncomplete => "controls_row_incomplete",
            Self::MandatoryAnatomyMissing => "mandatory_anatomy_missing",
            Self::MandatoryExportFieldMissing => "mandatory_export_field_missing",
            Self::ComponentSchemaRefMissing => "component_schema_ref_missing",
            Self::ExamplesMissing => "examples_missing",
            Self::DishonestExample => "dishonest_example",
            Self::RowInvariantViolated => "row_invariant_violated",
            Self::GovernanceReviewIncomplete => "governance_review_incomplete",
            Self::ConsumerProjectionIncomplete => "consumer_projection_incomplete",
            Self::ProofFreshnessIncomplete => "proof_freshness_incomplete",
            Self::ReleasePostureIncomplete => "release_posture_incomplete",
            Self::Ac1NotProven => "ac1_not_proven",
            Self::Ac2NotProven => "ac2_not_proven",
            Self::RawMaterialInExport => "raw_material_in_export",
        }
    }
}

/// Reads and validates the checked-in stable controls export.
pub fn current_stable_m5_auth_dashboard_controls_export(
) -> Result<M5AuthDashboardControlsPacket, M5AuthDashboardControlsArtifactError> {
    let packet: M5AuthDashboardControlsPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/release/m5-auth-handoff-card-remote-service-dashboard-header-controls-proof/support_export.json"
    )))
    .map_err(M5AuthDashboardControlsArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(M5AuthDashboardControlsArtifactError::Validation(violations))
    }
}

fn validate_source_contracts(
    packet: &M5AuthDashboardControlsPacket,
    violations: &mut Vec<M5AuthDashboardControlsViolation>,
) {
    let refs: BTreeSet<&str> = packet
        .source_contract_refs
        .iter()
        .map(String::as_str)
        .collect();
    for required in [
        M5_AUTH_DASHBOARD_CONTROLS_SCHEMA_REF,
        M5_AUTH_DASHBOARD_CONTROLS_DOC_REF,
        M5_EMBEDDED_BOUNDARY_COMPONENT_SCHEMA_REF,
        M5_EMBEDDED_BOUNDARY_COMPONENT_DOC_REF,
        M5_AUTH_HANDOFF_CARD_SCHEMA_REF,
        M5_REMOTE_SERVICE_DASHBOARD_HEADER_SCHEMA_REF,
    ] {
        if !refs.contains(required) {
            violations.push(M5AuthDashboardControlsViolation::MissingSourceContracts);
            return;
        }
    }
}

fn validate_controls_rows(
    packet: &M5AuthDashboardControlsPacket,
    violations: &mut Vec<M5AuthDashboardControlsViolation>,
) {
    if packet.controls_rows.is_empty() {
        violations.push(M5AuthDashboardControlsViolation::NoControlsRows);
        return;
    }
    for row in &packet.controls_rows {
        if row.owner_role.trim().is_empty()
            || row.scope_summary.trim().is_empty()
            || row.deployment_lines.is_empty()
            || row.required_labels.is_empty()
            || row.accessibility_routes.is_empty()
            || row.downgrade_triggers.is_empty()
            || row.required_proof_packet_refs.is_empty()
        {
            violations.push(M5AuthDashboardControlsViolation::ControlsRowIncomplete);
        }
        if !row.declares_mandatory_anatomy() {
            violations.push(M5AuthDashboardControlsViolation::MandatoryAnatomyMissing);
        }
        if !row.declares_mandatory_export_fields() {
            violations.push(M5AuthDashboardControlsViolation::MandatoryExportFieldMissing);
        }
        let refs: BTreeSet<&str> = row
            .source_contract_refs
            .iter()
            .map(String::as_str)
            .collect();
        if !refs.contains(M5_AUTH_HANDOFF_CARD_SCHEMA_REF)
            || !refs.contains(M5_REMOTE_SERVICE_DASHBOARD_HEADER_SCHEMA_REF)
        {
            violations.push(M5AuthDashboardControlsViolation::ComponentSchemaRefMissing);
        }
        if row.auth_handoff_card_examples.is_empty()
            || row.remote_service_dashboard_header_examples.is_empty()
        {
            violations.push(M5AuthDashboardControlsViolation::ExamplesMissing);
        }
        if !row.examples_are_honest() {
            violations.push(M5AuthDashboardControlsViolation::DishonestExample);
        }
        if !row.honours_invariants() {
            violations.push(M5AuthDashboardControlsViolation::RowInvariantViolated);
        }
    }
}

fn validate_governance_review(
    packet: &M5AuthDashboardControlsPacket,
    violations: &mut Vec<M5AuthDashboardControlsViolation>,
) {
    let review = &packet.governance_review;
    for ok in [
        review.card_distinguishes_checkpoint_from_handoff,
        review.card_names_provider_and_reason,
        review.card_keeps_local_continuity_explicit,
        review.card_discloses_device_code_or_expiry,
        review.card_never_imitates_native_approval,
        review.header_names_identity_and_ownership,
        review.header_discloses_freshness,
        review.dashboard_never_substitutes_for_local_recovery,
        review.no_high_risk_approval_in_embedded_chrome,
        review.reuses_frozen_matrix_vocabulary,
    ] {
        if !ok {
            violations.push(M5AuthDashboardControlsViolation::GovernanceReviewIncomplete);
            return;
        }
    }
}

fn validate_consumer_projection(
    packet: &M5AuthDashboardControlsPacket,
    violations: &mut Vec<M5AuthDashboardControlsViolation>,
) {
    let projection = &packet.consumer_projection;
    for ok in [
        projection.surfaces_consume_handoff_posture_vocabulary,
        projection.surfaces_consume_owner_freshness_vocabulary,
        projection.cards_consume_shared_fallback_expiry_vocabulary,
        projection.support_export_reads_single_boundary_source,
    ] {
        if !ok {
            violations.push(M5AuthDashboardControlsViolation::ConsumerProjectionIncomplete);
            return;
        }
    }
}

fn validate_proof_freshness(
    packet: &M5AuthDashboardControlsPacket,
    violations: &mut Vec<M5AuthDashboardControlsViolation>,
) {
    if packet.proof_freshness.proof_freshness_slo_hours == 0
        || packet.proof_freshness.last_proof_refresh.trim().is_empty()
    {
        violations.push(M5AuthDashboardControlsViolation::ProofFreshnessIncomplete);
    }
}

fn validate_release_posture(
    packet: &M5AuthDashboardControlsPacket,
    violations: &mut Vec<M5AuthDashboardControlsViolation>,
) {
    let posture = &packet.release_posture;
    if posture.proof_packet_ref.trim().is_empty()
        || posture.boundary_audit_ref.trim().is_empty()
        || !posture.support_export_parity_required
        || !posture.accessibility_parity_required
    {
        violations.push(M5AuthDashboardControlsViolation::ReleasePostureIncomplete);
    }
}

/// Proves the two acceptance criteria are exercised by the packet's resolved examples, not merely
/// asserted by governance bools.
fn validate_acceptance_criteria(
    packet: &M5AuthDashboardControlsPacket,
    violations: &mut Vec<M5AuthDashboardControlsViolation>,
) {
    let card_examples = || {
        packet
            .controls_rows
            .iter()
            .flat_map(|row| row.auth_handoff_card_examples.iter())
    };
    let header_examples = || {
        packet
            .controls_rows
            .iter()
            .flat_map(|row| row.remote_service_dashboard_header_examples.iter())
    };

    // AC1: users can distinguish embedded sign-in checkpoints from browser / passkey handoff and
    // know which local state remains intact — clean cards cover the embedded-checkpoint posture and
    // at least one external handoff posture, a local-continuity-unstated card degrades, a
    // native-approval-imitation card degrades, and no clean card hides continuity or imitates.
    let covers_embedded_checkpoint = card_examples().any(|ex| {
        ex.is_clean() && ex.posture.is_embedded_checkpoint() && ex.local_continuity_stated
    });
    let covers_external_handoff = card_examples().any(|ex| {
        ex.is_clean() && !ex.posture.is_embedded_checkpoint() && ex.local_continuity_stated
    });
    let continuity_unstated_degrades = card_examples().any(|ex| {
        ex.degrade_reason == Some(M5AuthHandoffCardDegradeReason::LocalContinuityUnstated)
    });
    let imitation_degrades = card_examples().any(|ex| {
        ex.degrade_reason == Some(M5AuthHandoffCardDegradeReason::ImitatesNativeApprovalUi)
    });
    let no_clean_card_hides =
        card_examples().all(|ex| !(ex.is_clean() && ex.hides_continuity_or_imitates()));
    if !(covers_embedded_checkpoint
        && covers_external_handoff
        && continuity_unstated_degrades
        && imitation_degrades
        && no_clean_card_hides)
    {
        violations.push(M5AuthDashboardControlsViolation::Ac1NotProven);
    }

    // AC2: a remote / service dashboard never substitutes for the primary local recovery controls or
    // hides its freshness and ownership boundaries — a clean header names identity + owner +
    // freshness and keeps local recovery reachable, a local-recovery-substitution header degrades, a
    // freshness-unstated header degrades, and no clean header substitutes or hides its boundaries.
    let covers_identity_and_freshness = header_examples().any(|ex| {
        ex.is_clean()
            && ex.service_identity_stated
            && ex.owner_origin_disclosed
            && ex.freshness_stated
            && ex.primary_local_recovery_available
    });
    let substitution_degrades = header_examples().any(|ex| {
        ex.degrade_reason
            == Some(M5RemoteServiceDashboardHeaderDegradeReason::SubstitutesForLocalRecovery)
    });
    let freshness_unstated_degrades = header_examples().any(|ex| {
        ex.degrade_reason
            == Some(M5RemoteServiceDashboardHeaderDegradeReason::FreshnessOrOfflineUnstated)
    });
    let no_clean_header_substitutes =
        header_examples().all(|ex| !(ex.is_clean() && ex.substitutes_or_hides_boundaries()));
    if !(covers_identity_and_freshness
        && substitution_degrades
        && freshness_unstated_degrades
        && no_clean_header_substitutes)
    {
        violations.push(M5AuthDashboardControlsViolation::Ac2NotProven);
    }
}

/// Joins tokens for a CSV cell with a `|` separator so a single cell never introduces a stray
/// comma.
fn join_tokens<T, F>(items: &[T], to_token: F) -> String
where
    F: Fn(&T) -> &'static str,
{
    items.iter().map(to_token).collect::<Vec<_>>().join("|")
}

/// Quotes a free-text CSV field when it contains a comma or quote.
fn csv_field(value: &str) -> String {
    if value.contains(',') || value.contains('"') || value.contains('\n') {
        format!("\"{}\"", value.replace('"', "\"\""))
    } else {
        value.to_owned()
    }
}

fn string_is_forbidden(value: &str) -> bool {
    let lower = value.to_lowercase();
    lower.contains("password")
        || lower.contains("passphrase")
        || lower.contains("bearer ")
        || lower.contains("://")
        || lower.contains("-----begin")
}

/// Heuristic that rejects obviously forbidden raw material in export-safe JSON.
fn json_contains_forbidden_material(value: &serde_json::Value) -> bool {
    match value {
        serde_json::Value::String(s) => string_is_forbidden(s),
        serde_json::Value::Array(arr) => arr.iter().any(json_contains_forbidden_material),
        serde_json::Value::Object(map) => map.values().any(json_contains_forbidden_material),
        _ => false,
    }
}
