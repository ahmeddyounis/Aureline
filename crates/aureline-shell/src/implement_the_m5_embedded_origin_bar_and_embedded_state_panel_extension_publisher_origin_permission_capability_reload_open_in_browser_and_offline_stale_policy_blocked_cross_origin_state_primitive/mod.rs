//! Implemented M5 embedded-origin-bar and embedded-state-panel primitives.
//!
//! The frozen [embedded-boundary component matrix][matrix] names the reusable embedded /
//! browser-handoff boundary UI components and locks their controlled vocabulary. This module is
//! the second implement lane over that matrix: it turns the two contributed-webview components —
//! the extension-owned **embedded origin bar** and the **embedded-state panel** — into resolvers
//! that produce export-safe, honest projections instead of anonymous web-pane chrome.
//!
//! Two acceptance criteria drive the resolvers:
//!
//! * **AC1 — no claimed M5 contributed webview can appear without visible owner/origin chrome and
//!   capability-limit disclosure.** [`resolve_embedded_origin_bar`] refuses to read as a clean bar
//!   when the owner / origin is undisclosed (or the origin is blocked / untrusted), when an
//!   extension-owned surface hides its publisher or extension name, or when the capability limits
//!   relative to native trusted chrome are unstated. A clean bar names its owner/origin, publisher,
//!   permission state, and capability limits, and offers a reload action and an open-in-browser
//!   path.
//! * **AC2 — embedded contributed surfaces never imitate native permission, trust, update, or
//!   irreversible confirmation UI.** Both [`resolve_embedded_origin_bar`] and
//!   [`resolve_embedded_state_panel`] degrade to their `ImitatesNativePermissionUi` reason the
//!   moment the surface is flagged as imitating native permission / trust / update / confirmation
//!   chrome, and [`resolve_embedded_state_panel`] additionally degrades to
//!   [`M5EmbeddedStatePanelDegradeReason::BlockedShownAsFresh`] whenever a stale, offline,
//!   policy-blocked, certificate-denied, or cross-origin-limited state is rendered as fresh
//!   first-party truth.
//!
//! The resolvers reuse the frozen matrix vocabulary directly — the single controlled
//! [`M5EmbeddedBoundaryDisposition`] boundary-disposition vocabulary, the
//! [`M5EmbeddedFreshnessState`] freshness vocabulary, the [`WebviewOwnerClass`] owner/origin
//! vocabulary, the [`OriginDisclosureClass`] origin-disclosure vocabulary, the
//! [`WebviewPermissionState`] permission vocabulary, the [`CapabilityLimitClass`] capability-limit
//! vocabulary, and the [`BrowserHandoffKind`] browser-fallback vocabulary — so this lane can never
//! fork its own owner, origin, permission, capability, or fallback wording.
//!
//! [matrix]: crate::freeze_the_m5_docs_pane_header_embedded_origin_bar_boundary_fact_grid_marketplace_account_boundary_card_auth_handoff_card_remote_service_dashboard_header_open_in_browser_handoff_row_and_embedded_state_panel_component_matrix

mod seed;
#[cfg(test)]
mod tests;

pub use seed::{
    seeded_m5_embedded_origin_state_controls,
    seeded_m5_embedded_origin_state_controls_embedded_webview_beta_narrowed,
    seeded_m5_embedded_origin_state_controls_remote_dashboard_preview_narrowed,
    M5_EMBEDDED_ORIGIN_STATE_CONTROLS_PACKET_ID,
};

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

use crate::freeze_the_m5_docs_pane_header_embedded_origin_bar_boundary_fact_grid_marketplace_account_boundary_card_auth_handoff_card_remote_service_dashboard_header_open_in_browser_handoff_row_and_embedded_state_panel_component_matrix::{
    M5EmbeddedAccessibilityRoute, M5EmbeddedBoundaryDisposition, M5EmbeddedConsumerSurface,
    M5EmbeddedDeploymentLine, M5EmbeddedDowngradeTrigger, M5EmbeddedFreshnessState,
    M5EmbeddedQualificationClass, M5EmbeddedRequiredLabel, M5_EMBEDDED_BOUNDARY_COMPONENT_DOC_REF,
    M5_EMBEDDED_BOUNDARY_COMPONENT_SCHEMA_REF, M5_EMBEDDED_ORIGIN_BAR_SCHEMA_REF,
    M5_EMBEDDED_STATE_PANEL_SCHEMA_REF,
};
use crate::m5_auth_boundaries::{
    BrowserHandoffKind, CapabilityLimitClass, OriginDisclosureClass, WebviewOwnerClass,
    WebviewPermissionState, M5_AUTH_BOUNDARY_CONTRACT_DOC_REF,
};

/// Stable record-kind tag carried by [`M5EmbeddedOriginStateControlsPacket`].
pub const M5_EMBEDDED_ORIGIN_STATE_CONTROLS_RECORD_KIND: &str =
    "implement_m5_embedded_origin_bar_and_embedded_state_panel_controls";

/// Schema version for M5 embedded-origin-bar / embedded-state-panel controls records.
pub const M5_EMBEDDED_ORIGIN_STATE_CONTROLS_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the combined controls boundary schema.
pub const M5_EMBEDDED_ORIGIN_STATE_CONTROLS_SCHEMA_REF: &str =
    "schemas/ui/m5-embedded-origin-bar-state-panel-controls.schema.json";

/// Repo-relative path of the controls doc.
pub const M5_EMBEDDED_ORIGIN_STATE_CONTROLS_DOC_REF: &str =
    "docs/help/m5_embedded_origin_bar_and_embedded_state_panel_controls.md";

/// Repo-relative path of the checked support-export artifact.
pub const M5_EMBEDDED_ORIGIN_STATE_CONTROLS_ARTIFACT_REF: &str =
    "artifacts/release/m5-embedded-origin-bar-state-panel-controls-proof/support_export.json";

/// Repo-relative path of the checked machine-readable controls CSV.
pub const M5_EMBEDDED_ORIGIN_STATE_CONTROLS_CSV_REF: &str =
    "artifacts/release/m5-embedded-origin-bar-state-panel-controls-proof/matrix.csv";

/// Repo-relative path of the checked Markdown design report.
pub const M5_EMBEDDED_ORIGIN_STATE_CONTROLS_REPORT_REF: &str =
    "artifacts/release/m5-embedded-origin-bar-state-panel-controls-proof/summary.md";

/// Repo-relative path of the protected fixture directory.
pub const M5_EMBEDDED_ORIGIN_STATE_CONTROLS_FIXTURE_DIR: &str =
    "fixtures/ui/m5-embedded-origin-bar-state-panel-controls";

/// Consumer surface an embedded-origin-state controls row projects onto. Reuses the frozen matrix
/// consumer-surface taxonomy so no lane invents a parallel surface set.
pub type M5EmbeddedOriginStateConsumerSurface = M5EmbeddedConsumerSurface;

/// Origin-disclosure classes an origin bar may carry, pinned locally because
/// [`OriginDisclosureClass`] does not expose an `ALL` array. Order matches its declaration.
pub const BOUND_ORIGIN_DISCLOSURES: [OriginDisclosureClass; 4] = [
    OriginDisclosureClass::NamedExtensionOrigin,
    OriginDisclosureClass::NamedProviderOrigin,
    OriginDisclosureClass::FirstPartyOrigin,
    OriginDisclosureClass::UndisclosedOriginBlocked,
];

/// Webview permission states an origin bar may carry, pinned locally because
/// [`WebviewPermissionState`] does not expose an `ALL` array. Order matches its declaration.
pub const BOUND_PERMISSION_STATES: [WebviewPermissionState; 4] = [
    WebviewPermissionState::NoElevatedPermissions,
    WebviewPermissionState::ScopedPermissionsGranted,
    WebviewPermissionState::PermissionRequestPending,
    WebviewPermissionState::PermissionDenied,
];

/// The single controlled embedded-state class an embedded-state panel can explain. These are the
/// exact states the spec requires a panel to explain with the same severity and support-boundary
/// vocabulary as first-party Aureline surfaces.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5EmbeddedStateClass {
    /// The embedded surface is live and healthy.
    LiveHealthy,
    /// A stale snapshot beyond the freshness grace window, labelled as stale.
    StaleSnapshot,
    /// An offline snapshot with no live refresh path, labelled as offline.
    OfflineSnapshot,
    /// Content blocked by a policy, labelled as blocked.
    PolicyBlocked,
    /// The surface certificate could not be verified and the content is denied.
    CertificateDenied,
    /// The surface is limited by a cross-origin boundary.
    CrossOriginLimited,
    /// The embedded state cannot currently be determined.
    StateUnknown,
}

impl M5EmbeddedStateClass {
    /// Every state class, in declaration order.
    pub const ALL: [Self; 7] = [
        Self::LiveHealthy,
        Self::StaleSnapshot,
        Self::OfflineSnapshot,
        Self::PolicyBlocked,
        Self::CertificateDenied,
        Self::CrossOriginLimited,
        Self::StateUnknown,
    ];

    /// The five non-live states a panel must be able to explain honestly.
    pub const NON_LIVE: [Self; 5] = [
        Self::StaleSnapshot,
        Self::OfflineSnapshot,
        Self::PolicyBlocked,
        Self::CertificateDenied,
        Self::CrossOriginLimited,
    ];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LiveHealthy => "live_healthy",
            Self::StaleSnapshot => "stale_snapshot",
            Self::OfflineSnapshot => "offline_snapshot",
            Self::PolicyBlocked => "policy_blocked",
            Self::CertificateDenied => "certificate_denied",
            Self::CrossOriginLimited => "cross_origin_limited",
            Self::StateUnknown => "state_unknown",
        }
    }

    /// Whether this state class is known (anything but unknown).
    pub const fn is_known(self) -> bool {
        !matches!(self, Self::StateUnknown)
    }

    /// Whether this state is a non-live state that must never be shown as fresh first-party truth.
    pub const fn is_non_live(self) -> bool {
        matches!(
            self,
            Self::StaleSnapshot
                | Self::OfflineSnapshot
                | Self::PolicyBlocked
                | Self::CertificateDenied
                | Self::CrossOriginLimited
        )
    }
}

/// One mandatory rendered part an embedded origin bar or embedded-state panel must be able to
/// show, so no boundary truth is left implicit.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5EmbeddedOriginStateAnatomyPart {
    /// The component's stable identity / what it represents.
    Identity,
    /// The component's current typed boundary disposition.
    State,
    /// The non-visual keyboard route to the component.
    KeyboardRoute,
    /// The owner / origin class behind the surface (origin bar).
    OwnerOrigin,
    /// The publisher / extension name behind the surface (origin bar).
    PublisherIdentity,
    /// The permission state of the surface (origin bar).
    PermissionState,
    /// The capability limits relative to native trusted chrome (origin bar).
    CapabilityLimits,
    /// The reload action for the surface (origin bar).
    ReloadAction,
    /// The open-in-browser handoff path (origin bar).
    OpenInBrowser,
    /// The explained embedded state (state panel).
    StateExplanation,
    /// The shared severity vocabulary (state panel).
    Severity,
    /// The support-boundary the panel names (state panel).
    SupportBoundary,
    /// The next safe recovery action (state panel).
    RecoveryAction,
}

impl M5EmbeddedOriginStateAnatomyPart {
    /// Every anatomy part, in declaration order.
    pub const ALL: [Self; 13] = [
        Self::Identity,
        Self::State,
        Self::KeyboardRoute,
        Self::OwnerOrigin,
        Self::PublisherIdentity,
        Self::PermissionState,
        Self::CapabilityLimits,
        Self::ReloadAction,
        Self::OpenInBrowser,
        Self::StateExplanation,
        Self::Severity,
        Self::SupportBoundary,
        Self::RecoveryAction,
    ];

    /// The three parts every claimed component must be able to show.
    pub const MANDATORY: [Self; 3] = [Self::Identity, Self::State, Self::KeyboardRoute];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Identity => "identity",
            Self::State => "state",
            Self::KeyboardRoute => "keyboard_route",
            Self::OwnerOrigin => "owner_origin",
            Self::PublisherIdentity => "publisher_identity",
            Self::PermissionState => "permission_state",
            Self::CapabilityLimits => "capability_limits",
            Self::ReloadAction => "reload_action",
            Self::OpenInBrowser => "open_in_browser",
            Self::StateExplanation => "state_explanation",
            Self::Severity => "severity",
            Self::SupportBoundary => "support_boundary",
            Self::RecoveryAction => "recovery_action",
        }
    }
}

/// Next safe action a component surfaces so a user is never left without a route out.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5EmbeddedOriginStateNextAction {
    /// Reload the embedded surface.
    ReloadSurface,
    /// Open the source externally in the browser.
    OpenInBrowser,
    /// Review the surface's permission state.
    ReviewPermissions,
    /// View the capability limits relative to native trusted chrome.
    ViewCapabilityLimits,
    /// Review diagnostics for the unavailable signal.
    ReviewDiagnostics,
    /// No action is needed; the component is clean.
    NoActionNeeded,
}

impl M5EmbeddedOriginStateNextAction {
    /// Every next action, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::ReloadSurface,
        Self::OpenInBrowser,
        Self::ReviewPermissions,
        Self::ViewCapabilityLimits,
        Self::ReviewDiagnostics,
        Self::NoActionNeeded,
    ];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ReloadSurface => "reload_surface",
            Self::OpenInBrowser => "open_in_browser",
            Self::ReviewPermissions => "review_permissions",
            Self::ViewCapabilityLimits => "view_capability_limits",
            Self::ReviewDiagnostics => "review_diagnostics",
            Self::NoActionNeeded => "no_action_needed",
        }
    }
}

/// Field an embedded-origin-state controls row exposes in the support export.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5EmbeddedOriginStateExportField {
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
    /// The owner / origin named by the origin bar.
    OwnerOrigin,
    /// The permission state named by the origin bar.
    PermissionState,
    /// The capability limits named by the origin bar.
    CapabilityLimits,
    /// The embedded-state class named by the state panel.
    StateClass,
    /// The open-in-browser handoff exposure.
    OpenInBrowser,
    /// The accountable owner role.
    OwnerRole,
}

impl M5EmbeddedOriginStateExportField {
    /// Every export field, in declaration order.
    pub const ALL: [Self; 11] = [
        Self::ConsumerSurface,
        Self::ComponentFamilies,
        Self::BoundaryDispositions,
        Self::DegradeReasons,
        Self::Qualification,
        Self::OwnerOrigin,
        Self::PermissionState,
        Self::CapabilityLimits,
        Self::StateClass,
        Self::OpenInBrowser,
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
            Self::OwnerOrigin => "owner_origin",
            Self::PermissionState => "permission_state",
            Self::CapabilityLimits => "capability_limits",
            Self::StateClass => "state_class",
            Self::OpenInBrowser => "open_in_browser",
            Self::OwnerRole => "owner_role",
        }
    }
}

/// Reason an embedded origin bar degraded below a clean, fully-legible state. The degrade-first
/// ladder returns one of these instead of ever letting an anonymous or masquerading bar read as a
/// clean pass.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5EmbeddedOriginBarDegradeReason {
    /// The owner / origin is undisclosed, blocked, or untrusted (AC1 violation).
    OwnerOrOriginUnstated,
    /// An extension-owned surface hides its publisher or extension name (AC1 violation).
    PublisherOrExtensionUnstated,
    /// The capability limits relative to native trusted chrome are unstated (AC1 violation).
    CapabilityLimitsUnstated,
    /// The bar imitates native permission / trust / update / confirmation UI (AC2 violation).
    ImitatesNativePermissionUi,
    /// The supporting proof packet has gone stale.
    ProofStale,
}

impl M5EmbeddedOriginBarDegradeReason {
    /// Every degrade reason, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::OwnerOrOriginUnstated,
        Self::PublisherOrExtensionUnstated,
        Self::CapabilityLimitsUnstated,
        Self::ImitatesNativePermissionUi,
        Self::ProofStale,
    ];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::OwnerOrOriginUnstated => "owner_or_origin_unstated",
            Self::PublisherOrExtensionUnstated => "publisher_or_extension_unstated",
            Self::CapabilityLimitsUnstated => "capability_limits_unstated",
            Self::ImitatesNativePermissionUi => "imitates_native_permission_ui",
            Self::ProofStale => "proof_stale",
        }
    }

    /// Next safe action for this reason.
    pub const fn next_action(self) -> M5EmbeddedOriginStateNextAction {
        match self {
            Self::OwnerOrOriginUnstated | Self::PublisherOrExtensionUnstated | Self::ProofStale => {
                M5EmbeddedOriginStateNextAction::ReviewDiagnostics
            }
            Self::CapabilityLimitsUnstated => M5EmbeddedOriginStateNextAction::ViewCapabilityLimits,
            Self::ImitatesNativePermissionUi => M5EmbeddedOriginStateNextAction::ReviewPermissions,
        }
    }

    /// The matching downgrade trigger recorded on the frozen matrix.
    pub const fn downgrade_trigger(self) -> M5EmbeddedDowngradeTrigger {
        match self {
            Self::OwnerOrOriginUnstated | Self::PublisherOrExtensionUnstated => {
                M5EmbeddedDowngradeTrigger::OwnerOrOriginUnstated
            }
            Self::CapabilityLimitsUnstated => M5EmbeddedDowngradeTrigger::CapabilityLimitsUnstated,
            Self::ImitatesNativePermissionUi => {
                M5EmbeddedDowngradeTrigger::ImitatesNativeApprovalChrome
            }
            Self::ProofStale => M5EmbeddedDowngradeTrigger::ProofStale,
        }
    }
}

/// Reason an embedded-state panel degraded below a clean, fully-legible state.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5EmbeddedStatePanelDegradeReason {
    /// The embedded-state class is unstated; a user cannot tell what state the surface is in.
    StateClassUnstated,
    /// The state is not explained.
    StateNotExplained,
    /// The severity or support-boundary vocabulary is not the shared first-party vocabulary.
    SupportBoundaryOrSeverityUnstated,
    /// A stale, offline, policy-blocked, certificate-denied, or cross-origin-limited state is
    /// rendered as fresh first-party truth (AC2 / guardrail violation).
    BlockedShownAsFresh,
    /// The panel imitates native permission / trust / update / confirmation UI (AC2 violation).
    ImitatesNativePermissionUi,
    /// The supporting proof packet has gone stale.
    ProofStale,
}

impl M5EmbeddedStatePanelDegradeReason {
    /// Every degrade reason, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::StateClassUnstated,
        Self::StateNotExplained,
        Self::SupportBoundaryOrSeverityUnstated,
        Self::BlockedShownAsFresh,
        Self::ImitatesNativePermissionUi,
        Self::ProofStale,
    ];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::StateClassUnstated => "state_class_unstated",
            Self::StateNotExplained => "state_not_explained",
            Self::SupportBoundaryOrSeverityUnstated => "support_boundary_or_severity_unstated",
            Self::BlockedShownAsFresh => "blocked_shown_as_fresh",
            Self::ImitatesNativePermissionUi => "imitates_native_permission_ui",
            Self::ProofStale => "proof_stale",
        }
    }

    /// Next safe action for this reason.
    pub const fn next_action(self) -> M5EmbeddedOriginStateNextAction {
        match self {
            Self::StateClassUnstated
            | Self::StateNotExplained
            | Self::SupportBoundaryOrSeverityUnstated
            | Self::ProofStale => M5EmbeddedOriginStateNextAction::ReviewDiagnostics,
            Self::BlockedShownAsFresh => M5EmbeddedOriginStateNextAction::OpenInBrowser,
            Self::ImitatesNativePermissionUi => M5EmbeddedOriginStateNextAction::ReviewPermissions,
        }
    }

    /// The matching downgrade trigger recorded on the frozen matrix.
    pub const fn downgrade_trigger(self) -> M5EmbeddedDowngradeTrigger {
        match self {
            Self::StateClassUnstated => M5EmbeddedDowngradeTrigger::FreshnessOrLastUpdatedUnstated,
            Self::StateNotExplained | Self::SupportBoundaryOrSeverityUnstated => {
                M5EmbeddedDowngradeTrigger::GenericChromeWordingUsed
            }
            Self::BlockedShownAsFresh => M5EmbeddedDowngradeTrigger::StaleOrBlockedShownAsFresh,
            Self::ImitatesNativePermissionUi => {
                M5EmbeddedDowngradeTrigger::ImitatesNativeApprovalChrome
            }
            Self::ProofStale => M5EmbeddedDowngradeTrigger::ProofStale,
        }
    }
}

/// Maps an origin bar's owner class and freshness state to the single controlled boundary
/// disposition.
fn disposition_for_origin(
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
            WebviewOwnerClass::FirstPartyEmbedded => D::LiveFirstPartyLocal,
            WebviewOwnerClass::UnknownUntrusted => D::NotEvaluated,
        },
    }
}

/// Maps an embedded-state class and owner class to the single controlled boundary disposition.
fn disposition_for_state(
    state: M5EmbeddedStateClass,
    owner: WebviewOwnerClass,
) -> M5EmbeddedBoundaryDisposition {
    use M5EmbeddedBoundaryDisposition as D;
    match state {
        M5EmbeddedStateClass::StaleSnapshot => D::StaleSnapshot,
        M5EmbeddedStateClass::OfflineSnapshot => D::OfflineSnapshot,
        M5EmbeddedStateClass::PolicyBlocked | M5EmbeddedStateClass::CertificateDenied => {
            D::ProviderBlocked
        }
        M5EmbeddedStateClass::CrossOriginLimited => D::CapabilityLimited,
        M5EmbeddedStateClass::LiveHealthy => match owner {
            WebviewOwnerClass::ExtensionOwned => D::CapabilityLimited,
            WebviewOwnerClass::ProviderOwned => D::LiveProviderOwned,
            WebviewOwnerClass::FirstPartyEmbedded => D::LiveFirstPartyLocal,
            WebviewOwnerClass::UnknownUntrusted => D::NotEvaluated,
        },
        M5EmbeddedStateClass::StateUnknown => D::NotEvaluated,
    }
}

/// Input to [`resolve_embedded_origin_bar`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M5EmbeddedOriginBarResolutionInput {
    /// Stable identity of the bar instance.
    pub bar_id: String,
    /// The owner / origin class behind the embedded surface.
    pub owner_class: WebviewOwnerClass,
    /// The origin-disclosure class shown on the bar.
    pub origin_disclosure: OriginDisclosureClass,
    /// The extension name (empty means unstated).
    pub extension_name: String,
    /// The publisher (empty means unstated).
    pub publisher: String,
    /// True when the owner / origin is disclosed on the bar chrome, never menu-only.
    pub owner_origin_disclosed: bool,
    /// The permission state of the surface.
    pub permission_state: WebviewPermissionState,
    /// Capability limits the bar names relative to native trusted chrome.
    pub capability_limits: Vec<CapabilityLimitClass>,
    /// True when the capability limits are disclosed on the bar, never menu-only.
    pub capability_limits_disclosed: bool,
    /// True when a reload action is offered on the bar.
    pub reload_available: bool,
    /// The browser-handoff kind the open-in-browser path uses, if any.
    pub open_in_browser_kind: Option<BrowserHandoffKind>,
    /// True when an open-in-browser path is offered on the bar, never menu-only.
    pub open_in_browser_available: bool,
    /// True when the bar imitates native permission / trust / update / confirmation UI.
    pub imitates_native_permission_ui: bool,
    /// The freshness / last-updated state of the surface.
    pub freshness: M5EmbeddedFreshnessState,
    /// True when the supporting proof packet is fresh.
    pub proof_fresh: bool,
}

/// Resolved, export-safe embedded origin bar projection.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ResolvedEmbeddedOriginBar {
    /// Stable identity of the bar instance.
    pub bar_id: String,
    /// Owner / origin token named by the bar.
    pub owner_origin: String,
    /// Origin-disclosure token named by the bar.
    pub origin_disclosure: String,
    /// Extension name named by the bar.
    pub extension_name: String,
    /// Publisher named by the bar.
    pub publisher: String,
    /// Single controlled boundary disposition carried by the bar.
    pub boundary_disposition: M5EmbeddedBoundaryDisposition,
    /// Permission-state token named by the bar.
    pub permission_state: String,
    /// Capability-limit tokens named by the bar.
    pub capability_limits: Vec<String>,
    /// Freshness token named by the bar.
    pub freshness: String,
    /// Whether a reload action is offered.
    pub reload_available: bool,
    /// Open-in-browser handoff token, if any.
    pub open_in_browser_kind: Option<String>,
    /// Whether an open-in-browser path is offered.
    pub open_in_browser_available: bool,
    /// Degrade reason, if the bar could not read as a clean, fully-legible state.
    pub degrade_reason: Option<M5EmbeddedOriginBarDegradeReason>,
    /// Next safe action offered.
    pub next_action: M5EmbeddedOriginStateNextAction,
    /// AC1: whether the owner / origin is disclosed on the bar chrome.
    pub owner_origin_disclosed: bool,
    /// AC1: whether the capability limits are disclosed on the bar chrome.
    pub capability_limits_disclosed: bool,
    /// Guardrail (MUST be `false` on a clean bar): the bar imitates native permission / trust /
    /// update / confirmation UI.
    pub imitates_native_permission_ui: bool,
}

impl M5ResolvedEmbeddedOriginBar {
    /// Whether this bar reads as a clean, fully-legible state.
    pub fn is_clean(&self) -> bool {
        self.degrade_reason.is_none()
    }

    /// Whether this bar hides its owner/origin or its capability limits (an AC1 violation).
    pub fn hides_owner_or_capability(&self) -> bool {
        !self.owner_origin_disclosed || !self.capability_limits_disclosed
    }
}

/// Input to [`resolve_embedded_state_panel`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M5EmbeddedStatePanelResolutionInput {
    /// Stable identity of the panel instance.
    pub panel_id: String,
    /// The embedded-state class the panel explains.
    pub state_class: M5EmbeddedStateClass,
    /// The owner / origin class behind the embedded surface.
    pub owner_class: WebviewOwnerClass,
    /// True when the panel explains why the surface is in this state.
    pub state_explained: bool,
    /// True when the panel uses the shared first-party severity and support-boundary vocabulary.
    pub severity_and_support_boundary_shared: bool,
    /// True when a recovery action is offered on the panel.
    pub recovery_action_available: bool,
    /// True when a non-live state is rendered as fresh first-party truth (a guardrail violation).
    pub shown_as_fresh_first_party: bool,
    /// True when the panel imitates native permission / trust / update / confirmation UI.
    pub imitates_native_permission_ui: bool,
    /// True when the supporting proof packet is fresh.
    pub proof_fresh: bool,
}

/// Resolved, export-safe embedded-state panel projection.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ResolvedEmbeddedStatePanel {
    /// Stable identity of the panel instance.
    pub panel_id: String,
    /// The embedded-state class token named by the panel.
    pub state_class: String,
    /// The owner / origin token named by the panel.
    pub owner_origin: String,
    /// Single controlled boundary disposition carried by the panel.
    pub boundary_disposition: M5EmbeddedBoundaryDisposition,
    /// Whether the panel explains why the surface is in this state.
    pub state_explained: bool,
    /// Whether the panel uses the shared first-party severity and support-boundary vocabulary.
    pub severity_and_support_boundary_shared: bool,
    /// Whether a recovery action is offered.
    pub recovery_action_available: bool,
    /// Degrade reason, if the panel could not read as a clean, fully-legible state.
    pub degrade_reason: Option<M5EmbeddedStatePanelDegradeReason>,
    /// Next safe action offered.
    pub next_action: M5EmbeddedOriginStateNextAction,
    /// Guardrail (MUST be `false` on a clean panel): a non-live state is rendered as fresh
    /// first-party truth.
    pub renders_blocked_as_fresh: bool,
    /// Guardrail (MUST be `false` on a clean panel): the panel imitates native permission / trust
    /// / update / confirmation UI.
    pub imitates_native_permission_ui: bool,
}

impl M5ResolvedEmbeddedStatePanel {
    /// Whether this panel reads as a clean, fully-legible state.
    pub fn is_clean(&self) -> bool {
        self.degrade_reason.is_none()
    }
}

/// Error emitted when a resolver input carries invalid or forbidden material.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum M5EmbeddedOriginStateResolutionError {
    /// The origin bar id was empty.
    EmptyBarId,
    /// The state panel id was empty.
    EmptyPanelId,
    /// A field carried forbidden raw material (secret / endpoint).
    ForbiddenMaterial,
}

impl M5EmbeddedOriginStateResolutionError {
    /// Stable token used in tests and exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::EmptyBarId => "empty_bar_id",
            Self::EmptyPanelId => "empty_panel_id",
            Self::ForbiddenMaterial => "forbidden_material",
        }
    }
}

impl fmt::Display for M5EmbeddedOriginStateResolutionError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "m5 embedded-origin-state resolution error: {}",
            self.as_str()
        )
    }
}

impl Error for M5EmbeddedOriginStateResolutionError {}

/// Resolves an embedded origin bar, proving AC1: a contributed webview never appears without
/// visible owner/origin chrome and capability-limit disclosure, and proving AC2's origin-bar half:
/// the bar never imitates native permission / trust / update / confirmation UI.
pub fn resolve_embedded_origin_bar(
    input: M5EmbeddedOriginBarResolutionInput,
) -> Result<M5ResolvedEmbeddedOriginBar, M5EmbeddedOriginStateResolutionError> {
    if input.bar_id.trim().is_empty() {
        return Err(M5EmbeddedOriginStateResolutionError::EmptyBarId);
    }
    if string_is_forbidden(&input.bar_id)
        || string_is_forbidden(&input.extension_name)
        || string_is_forbidden(&input.publisher)
    {
        return Err(M5EmbeddedOriginStateResolutionError::ForbiddenMaterial);
    }

    let owner_origin_disclosed = input.owner_origin_disclosed
        && input.owner_class != WebviewOwnerClass::UnknownUntrusted
        && input.origin_disclosure != OriginDisclosureClass::UndisclosedOriginBlocked;
    let publisher_unstated = input.owner_class == WebviewOwnerClass::ExtensionOwned
        && (input.extension_name.trim().is_empty() || input.publisher.trim().is_empty());
    let capability_limits_disclosed =
        input.capability_limits_disclosed && !input.capability_limits.is_empty();

    let degrade_reason = if !owner_origin_disclosed {
        Some(M5EmbeddedOriginBarDegradeReason::OwnerOrOriginUnstated)
    } else if publisher_unstated {
        Some(M5EmbeddedOriginBarDegradeReason::PublisherOrExtensionUnstated)
    } else if !capability_limits_disclosed {
        Some(M5EmbeddedOriginBarDegradeReason::CapabilityLimitsUnstated)
    } else if input.imitates_native_permission_ui {
        Some(M5EmbeddedOriginBarDegradeReason::ImitatesNativePermissionUi)
    } else if !input.proof_fresh {
        Some(M5EmbeddedOriginBarDegradeReason::ProofStale)
    } else {
        None
    };

    let boundary_disposition = match degrade_reason {
        Some(_) => M5EmbeddedBoundaryDisposition::NotEvaluated,
        None => disposition_for_origin(input.owner_class, input.freshness),
    };
    let next_action = match degrade_reason {
        Some(reason) => reason.next_action(),
        None => M5EmbeddedOriginStateNextAction::NoActionNeeded,
    };

    Ok(M5ResolvedEmbeddedOriginBar {
        bar_id: input.bar_id,
        owner_origin: input.owner_class.as_str().to_owned(),
        origin_disclosure: input.origin_disclosure.as_str().to_owned(),
        extension_name: input.extension_name,
        publisher: input.publisher,
        boundary_disposition,
        permission_state: input.permission_state.as_str().to_owned(),
        capability_limits: input
            .capability_limits
            .iter()
            .map(|c| c.as_str().to_owned())
            .collect(),
        freshness: input.freshness.as_str().to_owned(),
        reload_available: input.reload_available,
        open_in_browser_kind: input.open_in_browser_kind.map(|k| k.as_str().to_owned()),
        open_in_browser_available: input.open_in_browser_available,
        degrade_reason,
        next_action,
        owner_origin_disclosed,
        capability_limits_disclosed,
        imitates_native_permission_ui: input.imitates_native_permission_ui,
    })
}

/// Resolves an embedded-state panel, proving AC2's panel half: a stale, offline, policy-blocked,
/// certificate-denied, or cross-origin-limited state is explained with the shared severity and
/// support-boundary vocabulary, is never rendered as fresh first-party truth, and never imitates
/// native permission / trust / update / confirmation UI.
pub fn resolve_embedded_state_panel(
    input: M5EmbeddedStatePanelResolutionInput,
) -> Result<M5ResolvedEmbeddedStatePanel, M5EmbeddedOriginStateResolutionError> {
    if input.panel_id.trim().is_empty() {
        return Err(M5EmbeddedOriginStateResolutionError::EmptyPanelId);
    }
    if string_is_forbidden(&input.panel_id) {
        return Err(M5EmbeddedOriginStateResolutionError::ForbiddenMaterial);
    }

    let renders_blocked_as_fresh =
        input.state_class.is_non_live() && input.shown_as_fresh_first_party;

    let degrade_reason = if !input.state_class.is_known() {
        Some(M5EmbeddedStatePanelDegradeReason::StateClassUnstated)
    } else if !input.state_explained {
        Some(M5EmbeddedStatePanelDegradeReason::StateNotExplained)
    } else if !input.severity_and_support_boundary_shared {
        Some(M5EmbeddedStatePanelDegradeReason::SupportBoundaryOrSeverityUnstated)
    } else if renders_blocked_as_fresh {
        Some(M5EmbeddedStatePanelDegradeReason::BlockedShownAsFresh)
    } else if input.imitates_native_permission_ui {
        Some(M5EmbeddedStatePanelDegradeReason::ImitatesNativePermissionUi)
    } else if !input.proof_fresh {
        Some(M5EmbeddedStatePanelDegradeReason::ProofStale)
    } else {
        None
    };

    let boundary_disposition = match degrade_reason {
        Some(_) => M5EmbeddedBoundaryDisposition::NotEvaluated,
        None => disposition_for_state(input.state_class, input.owner_class),
    };
    let next_action = match degrade_reason {
        Some(reason) => reason.next_action(),
        None => M5EmbeddedOriginStateNextAction::NoActionNeeded,
    };

    Ok(M5ResolvedEmbeddedStatePanel {
        panel_id: input.panel_id,
        state_class: input.state_class.as_str().to_owned(),
        owner_origin: input.owner_class.as_str().to_owned(),
        boundary_disposition,
        state_explained: input.state_explained,
        severity_and_support_boundary_shared: input.severity_and_support_boundary_shared,
        recovery_action_available: input.recovery_action_available,
        degrade_reason,
        next_action,
        renders_blocked_as_fresh,
        imitates_native_permission_ui: input.imitates_native_permission_ui,
    })
}

/// One controls row: one consumer surface bound to the resolved origin bar and embedded-state
/// panel examples it must project honestly.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5EmbeddedOriginStateControlsRow {
    /// Consumer surface this row projects onto.
    pub consumer_surface: M5EmbeddedOriginStateConsumerSurface,
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
    pub anatomy_parts: Vec<M5EmbeddedOriginStateAnatomyPart>,
    /// Export fields exposed (must include the mandatory five).
    pub export_fields: Vec<M5EmbeddedOriginStateExportField>,
    /// Downgrade triggers that apply to this row.
    pub downgrade_triggers: Vec<M5EmbeddedDowngradeTrigger>,
    /// Resolved embedded origin bar examples.
    pub embedded_origin_bar_examples: Vec<M5ResolvedEmbeddedOriginBar>,
    /// Resolved embedded-state panel examples.
    pub embedded_state_panel_examples: Vec<M5ResolvedEmbeddedStatePanel>,
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

impl M5EmbeddedOriginStateControlsRow {
    fn declares_mandatory_anatomy(&self) -> bool {
        let present: BTreeSet<M5EmbeddedOriginStateAnatomyPart> =
            self.anatomy_parts.iter().copied().collect();
        M5EmbeddedOriginStateAnatomyPart::MANDATORY
            .iter()
            .all(|part| present.contains(part))
    }

    fn declares_mandatory_export_fields(&self) -> bool {
        let present: BTreeSet<M5EmbeddedOriginStateExportField> =
            self.export_fields.iter().copied().collect();
        M5EmbeddedOriginStateExportField::MANDATORY
            .iter()
            .all(|field| present.contains(field))
    }

    fn honours_invariants(&self) -> bool {
        !self.masquerades_as_native_approval_chrome
            && !self.hides_owner_origin_or_handoff_in_menus_only
            && !self.renders_stale_or_blocked_as_fresh_first_party_truth
            && !self.embeds_high_risk_approval_without_native_step_up
    }

    /// True when every resolved example on this row is honest: no clean bar hides its owner/origin
    /// or capability limits or imitates native UI, and no clean panel imitates native UI or renders
    /// a blocked state as fresh.
    fn examples_are_honest(&self) -> bool {
        self.embedded_origin_bar_examples.iter().all(|ex| {
            !(ex.is_clean() && (ex.hides_owner_or_capability() || ex.imitates_native_permission_ui))
        }) && self.embedded_state_panel_examples.iter().all(|ex| {
            !(ex.is_clean() && (ex.imitates_native_permission_ui || ex.renders_blocked_as_fresh))
        })
    }
}

/// Self-describing controlled-vocabulary set frozen by the controls packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5EmbeddedOriginStateVocabularySet {
    /// Boundary-disposition tokens (bound from the frozen matrix).
    pub boundary_dispositions: Vec<String>,
    /// Embedded-state-class tokens.
    pub state_classes: Vec<String>,
    /// Owner-class tokens (bound from the auth-boundary object model).
    pub owner_classes: Vec<String>,
    /// Origin-disclosure tokens (bound from the auth-boundary object model).
    pub origin_disclosures: Vec<String>,
    /// Permission-state tokens (bound from the auth-boundary object model).
    pub permission_states: Vec<String>,
    /// Capability-limit tokens (bound from the auth-boundary object model).
    pub capability_limits: Vec<String>,
    /// Browser-handoff-kind tokens (bound from the auth-boundary object model).
    pub browser_handoff_kinds: Vec<String>,
    /// Freshness-state tokens (bound from the frozen matrix).
    pub freshness_states: Vec<String>,
    /// Origin-bar degrade-reason tokens.
    pub origin_bar_degrade_reasons: Vec<String>,
    /// State-panel degrade-reason tokens.
    pub state_panel_degrade_reasons: Vec<String>,
    /// Anatomy-part tokens.
    pub anatomy_parts: Vec<String>,
    /// Next-action tokens.
    pub next_actions: Vec<String>,
    /// Export-field tokens.
    pub export_fields: Vec<String>,
    /// Consumer-surface tokens.
    pub consumer_surfaces: Vec<String>,
}

impl M5EmbeddedOriginStateVocabularySet {
    /// Builds the canonical vocabulary set from the typed `ALL` arrays.
    pub fn canonical() -> Self {
        Self {
            boundary_dispositions: tokens(&M5EmbeddedBoundaryDisposition::ALL, |v| v.as_str()),
            state_classes: tokens(&M5EmbeddedStateClass::ALL, |v| v.as_str()),
            owner_classes: tokens(&WebviewOwnerClass::ALL, |v| v.as_str()),
            origin_disclosures: tokens(&BOUND_ORIGIN_DISCLOSURES, |v| v.as_str()),
            permission_states: tokens(&BOUND_PERMISSION_STATES, |v| v.as_str()),
            capability_limits: tokens(&CapabilityLimitClass::ALL, |v| v.as_str()),
            browser_handoff_kinds: tokens(&BrowserHandoffKind::ALL, |v| v.as_str()),
            freshness_states: tokens(&M5EmbeddedFreshnessState::ALL, |v| v.as_str()),
            origin_bar_degrade_reasons: tokens(&M5EmbeddedOriginBarDegradeReason::ALL, |v| {
                v.as_str()
            }),
            state_panel_degrade_reasons: tokens(&M5EmbeddedStatePanelDegradeReason::ALL, |v| {
                v.as_str()
            }),
            anatomy_parts: tokens(&M5EmbeddedOriginStateAnatomyPart::ALL, |v| v.as_str()),
            next_actions: tokens(&M5EmbeddedOriginStateNextAction::ALL, |v| v.as_str()),
            export_fields: tokens(&M5EmbeddedOriginStateExportField::ALL, |v| v.as_str()),
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
pub struct M5EmbeddedOriginStateGovernanceReview {
    /// The origin bar always names its owner/origin and publisher.
    pub origin_bar_names_owner_and_publisher: bool,
    /// The origin bar always discloses its capability limits.
    pub origin_bar_discloses_capability_limits: bool,
    /// Owner and origin are always explicit, never menu-only.
    pub owner_and_origin_always_explicit: bool,
    /// The open-in-browser path is always exposed, never menu-only.
    pub open_in_browser_always_exposed: bool,
    /// No embedded surface imitates native permission / trust / update / confirmation UI.
    pub no_surface_imitates_native_ui: bool,
    /// The state panel always explains its state with the shared severity/support vocabulary.
    pub state_panel_explains_state_with_shared_vocabulary: bool,
    /// A stale, offline, or blocked state is never shown as fresh first-party truth.
    pub stale_or_blocked_never_shown_as_fresh: bool,
    /// Every row declares the mandatory anatomy parts.
    pub every_row_declares_mandatory_anatomy: bool,
    /// Every row declares a non-visual accessibility route.
    pub every_row_declares_accessibility_route: bool,
    /// The lane reuses the frozen matrix vocabulary rather than inventing parallel wording.
    pub reuses_frozen_matrix_vocabulary: bool,
}

/// Consumer projection block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5EmbeddedOriginStateConsumerProjection {
    /// Embedded surfaces consume the shared owner/origin vocabulary.
    pub embedded_surfaces_consume_owner_origin_vocabulary: bool,
    /// Embedded surfaces consume the shared capability-limit vocabulary.
    pub embedded_surfaces_consume_capability_limit_vocabulary: bool,
    /// State panels consume the shared severity / support-boundary vocabulary.
    pub state_panels_consume_shared_severity_vocabulary: bool,
    /// Support / export reads a single canonical boundary source.
    pub support_export_reads_single_boundary_source: bool,
}

/// Proof freshness block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5EmbeddedOriginStateProofFreshness {
    /// Proof-freshness SLO in hours.
    pub proof_freshness_slo_hours: u32,
    /// RFC 3339 timestamp of the last proof refresh.
    pub last_proof_refresh: String,
    /// True when stale proof automatically narrows the component.
    pub auto_narrow_on_stale: bool,
}

/// Release and support parity posture for the controls lane.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5EmbeddedOriginStateReleasePosture {
    /// Ref of the supporting proof packet for the lane.
    pub proof_packet_ref: String,
    /// Ref of the supporting boundary audit for the lane.
    pub boundary_audit_ref: String,
    /// True when support/export parity is required for every row.
    pub support_export_parity_required: bool,
    /// True when accessibility parity is required for every row.
    pub accessibility_parity_required: bool,
}

/// Constructor input for [`M5EmbeddedOriginStateControlsPacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M5EmbeddedOriginStateControlsPacketInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable controls label.
    pub controls_label: String,
    /// Controls rows.
    pub controls_rows: Vec<M5EmbeddedOriginStateControlsRow>,
    /// Frozen controlled-vocabulary set.
    pub vocabulary_set: M5EmbeddedOriginStateVocabularySet,
    /// Governance-review block.
    pub governance_review: M5EmbeddedOriginStateGovernanceReview,
    /// Consumer projection block.
    pub consumer_projection: M5EmbeddedOriginStateConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: M5EmbeddedOriginStateProofFreshness,
    /// Release and support parity posture.
    pub release_posture: M5EmbeddedOriginStateReleasePosture,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

/// Export-safe M5 embedded-origin-bar / embedded-state-panel controls packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5EmbeddedOriginStateControlsPacket {
    /// Record kind; must equal [`M5_EMBEDDED_ORIGIN_STATE_CONTROLS_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`M5_EMBEDDED_ORIGIN_STATE_CONTROLS_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable controls label.
    pub controls_label: String,
    /// Controls rows.
    pub controls_rows: Vec<M5EmbeddedOriginStateControlsRow>,
    /// Frozen controlled-vocabulary set.
    pub vocabulary_set: M5EmbeddedOriginStateVocabularySet,
    /// Governance-review block.
    pub governance_review: M5EmbeddedOriginStateGovernanceReview,
    /// Consumer projection block.
    pub consumer_projection: M5EmbeddedOriginStateConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: M5EmbeddedOriginStateProofFreshness,
    /// Release and support parity posture.
    pub release_posture: M5EmbeddedOriginStateReleasePosture,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

impl M5EmbeddedOriginStateControlsPacket {
    /// Builds a controls packet from stable-lane input.
    pub fn new(input: M5EmbeddedOriginStateControlsPacketInput) -> Self {
        Self {
            record_kind: M5_EMBEDDED_ORIGIN_STATE_CONTROLS_RECORD_KIND.to_owned(),
            schema_version: M5_EMBEDDED_ORIGIN_STATE_CONTROLS_SCHEMA_VERSION,
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
    pub fn validate(&self) -> Vec<M5EmbeddedOriginStateControlsViolation> {
        let mut violations = Vec::new();

        if self.record_kind != M5_EMBEDDED_ORIGIN_STATE_CONTROLS_RECORD_KIND {
            violations.push(M5EmbeddedOriginStateControlsViolation::WrongRecordKind);
        }
        if self.schema_version != M5_EMBEDDED_ORIGIN_STATE_CONTROLS_SCHEMA_VERSION {
            violations.push(M5EmbeddedOriginStateControlsViolation::WrongSchemaVersion);
        }
        if self.packet_id.trim().is_empty()
            || self.controls_label.trim().is_empty()
            || self.redaction_class_token.trim().is_empty()
            || self.minted_at.trim().is_empty()
        {
            violations.push(M5EmbeddedOriginStateControlsViolation::MissingIdentity);
        }

        validate_source_contracts(self, &mut violations);
        if !self.vocabulary_set.matches_canonical() {
            violations.push(M5EmbeddedOriginStateControlsViolation::VocabularySetDrift);
        }
        validate_controls_rows(self, &mut violations);
        validate_governance_review(self, &mut violations);
        validate_consumer_projection(self, &mut violations);
        validate_proof_freshness(self, &mut violations);
        validate_release_posture(self, &mut violations);
        validate_acceptance_criteria(self, &mut violations);

        if json_contains_forbidden_material(
            &serde_json::to_value(self)
                .expect("m5 embedded-origin-state controls packet serializes"),
        ) {
            violations.push(M5EmbeddedOriginStateControlsViolation::RawMaterialInExport);
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
            .expect("m5 embedded-origin-state controls packet serializes")
    }

    /// Deterministic, machine-readable controls CSV: one row per consumer surface.
    pub fn render_matrix_csv(&self) -> String {
        let mut out = String::new();
        out.push_str(
            "consumer_surface,qualification,owner,origin_bar_examples,state_panel_examples,degrade_reasons,downgrade_triggers\n",
        );
        for row in &self.controls_rows {
            let degrades: Vec<&str> = row
                .embedded_origin_bar_examples
                .iter()
                .filter_map(|ex| ex.degrade_reason.map(|r| r.as_str()))
                .chain(
                    row.embedded_state_panel_examples
                        .iter()
                        .filter_map(|ex| ex.degrade_reason.map(|r| r.as_str())),
                )
                .collect();
            out.push_str(&format!(
                "{},{},{},{},{},{},{}\n",
                row.consumer_surface.as_str(),
                row.qualification.as_str(),
                csv_field(&row.owner_role),
                row.embedded_origin_bar_examples.len(),
                row.embedded_state_panel_examples.len(),
                degrades.join("|"),
                join_tokens(&row.downgrade_triggers, |v| v.as_str()),
            ));
        }
        out
    }

    /// Deterministic Markdown report for support, docs, or review handoff.
    pub fn render_markdown_summary(&self) -> String {
        let mut out = String::new();
        out.push_str("# M5 Embedded-Origin-Bar and Embedded-State-Panel Controls\n\n");
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- Label: `{}`\n", self.controls_label));
        out.push_str(&format!(
            "- Consumer surfaces: {}\n",
            self.controls_rows.len()
        ));
        out.push_str(&format!(
            "- State classes: {}\n",
            self.vocabulary_set.state_classes.join(", ")
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
                "  - Origin-bar examples: {} / state-panel examples: {}\n",
                row.embedded_origin_bar_examples.len(),
                row.embedded_state_panel_examples.len()
            ));
        }
        out
    }
}

/// Errors emitted when reading the checked-in stable controls export.
#[derive(Debug)]
pub enum M5EmbeddedOriginStateControlsArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<M5EmbeddedOriginStateControlsViolation>),
}

impl fmt::Display for M5EmbeddedOriginStateControlsArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(
                    formatter,
                    "m5 embedded-origin-state controls export parse failed: {error}"
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
                    "m5 embedded-origin-state controls export failed validation: {tokens}"
                )
            }
        }
    }
}

impl Error for M5EmbeddedOriginStateControlsArtifactError {}

/// Validation failures emitted by [`M5EmbeddedOriginStateControlsPacket::validate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum M5EmbeddedOriginStateControlsViolation {
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
    /// A controls row carries a dishonest clean example (native imitation, hidden owner/capability,
    /// or blocked-shown-as-fresh).
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
    /// AC1 is not proven: clean bars do not cover the owner/origin + capability disclosure, no
    /// undisclosed-owner or undisclosed-capability bar degrades, or a required embedded state is not
    /// honestly rendered.
    Ac1NotProven,
    /// AC2 is not proven: no origin bar or state panel native-imitation degrades, or a clean example
    /// imitates native UI.
    Ac2NotProven,
    /// Export contains raw sensitive material.
    RawMaterialInExport,
}

impl M5EmbeddedOriginStateControlsViolation {
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
pub fn current_stable_m5_embedded_origin_state_controls_export(
) -> Result<M5EmbeddedOriginStateControlsPacket, M5EmbeddedOriginStateControlsArtifactError> {
    let packet: M5EmbeddedOriginStateControlsPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/release/m5-embedded-origin-bar-state-panel-controls-proof/support_export.json"
    )))
    .map_err(M5EmbeddedOriginStateControlsArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(M5EmbeddedOriginStateControlsArtifactError::Validation(
            violations,
        ))
    }
}

fn validate_source_contracts(
    packet: &M5EmbeddedOriginStateControlsPacket,
    violations: &mut Vec<M5EmbeddedOriginStateControlsViolation>,
) {
    let refs: BTreeSet<&str> = packet
        .source_contract_refs
        .iter()
        .map(String::as_str)
        .collect();
    for required in [
        M5_EMBEDDED_ORIGIN_STATE_CONTROLS_SCHEMA_REF,
        M5_EMBEDDED_ORIGIN_STATE_CONTROLS_DOC_REF,
        M5_EMBEDDED_BOUNDARY_COMPONENT_SCHEMA_REF,
        M5_EMBEDDED_BOUNDARY_COMPONENT_DOC_REF,
        M5_EMBEDDED_ORIGIN_BAR_SCHEMA_REF,
        M5_EMBEDDED_STATE_PANEL_SCHEMA_REF,
    ] {
        if !refs.contains(required) {
            violations.push(M5EmbeddedOriginStateControlsViolation::MissingSourceContracts);
            return;
        }
    }
}

fn validate_controls_rows(
    packet: &M5EmbeddedOriginStateControlsPacket,
    violations: &mut Vec<M5EmbeddedOriginStateControlsViolation>,
) {
    if packet.controls_rows.is_empty() {
        violations.push(M5EmbeddedOriginStateControlsViolation::NoControlsRows);
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
            violations.push(M5EmbeddedOriginStateControlsViolation::ControlsRowIncomplete);
        }
        if !row.declares_mandatory_anatomy() {
            violations.push(M5EmbeddedOriginStateControlsViolation::MandatoryAnatomyMissing);
        }
        if !row.declares_mandatory_export_fields() {
            violations.push(M5EmbeddedOriginStateControlsViolation::MandatoryExportFieldMissing);
        }
        let refs: BTreeSet<&str> = row
            .source_contract_refs
            .iter()
            .map(String::as_str)
            .collect();
        if !refs.contains(M5_EMBEDDED_ORIGIN_BAR_SCHEMA_REF)
            || !refs.contains(M5_EMBEDDED_STATE_PANEL_SCHEMA_REF)
        {
            violations.push(M5EmbeddedOriginStateControlsViolation::ComponentSchemaRefMissing);
        }
        if row.embedded_origin_bar_examples.is_empty()
            || row.embedded_state_panel_examples.is_empty()
        {
            violations.push(M5EmbeddedOriginStateControlsViolation::ExamplesMissing);
        }
        if !row.examples_are_honest() {
            violations.push(M5EmbeddedOriginStateControlsViolation::DishonestExample);
        }
        if !row.honours_invariants() {
            violations.push(M5EmbeddedOriginStateControlsViolation::RowInvariantViolated);
        }
    }
}

fn validate_governance_review(
    packet: &M5EmbeddedOriginStateControlsPacket,
    violations: &mut Vec<M5EmbeddedOriginStateControlsViolation>,
) {
    let review = &packet.governance_review;
    for ok in [
        review.origin_bar_names_owner_and_publisher,
        review.origin_bar_discloses_capability_limits,
        review.owner_and_origin_always_explicit,
        review.open_in_browser_always_exposed,
        review.no_surface_imitates_native_ui,
        review.state_panel_explains_state_with_shared_vocabulary,
        review.stale_or_blocked_never_shown_as_fresh,
        review.every_row_declares_mandatory_anatomy,
        review.every_row_declares_accessibility_route,
        review.reuses_frozen_matrix_vocabulary,
    ] {
        if !ok {
            violations.push(M5EmbeddedOriginStateControlsViolation::GovernanceReviewIncomplete);
            return;
        }
    }
}

fn validate_consumer_projection(
    packet: &M5EmbeddedOriginStateControlsPacket,
    violations: &mut Vec<M5EmbeddedOriginStateControlsViolation>,
) {
    let projection = &packet.consumer_projection;
    for ok in [
        projection.embedded_surfaces_consume_owner_origin_vocabulary,
        projection.embedded_surfaces_consume_capability_limit_vocabulary,
        projection.state_panels_consume_shared_severity_vocabulary,
        projection.support_export_reads_single_boundary_source,
    ] {
        if !ok {
            violations.push(M5EmbeddedOriginStateControlsViolation::ConsumerProjectionIncomplete);
            return;
        }
    }
}

fn validate_proof_freshness(
    packet: &M5EmbeddedOriginStateControlsPacket,
    violations: &mut Vec<M5EmbeddedOriginStateControlsViolation>,
) {
    if packet.proof_freshness.proof_freshness_slo_hours == 0
        || packet.proof_freshness.last_proof_refresh.trim().is_empty()
    {
        violations.push(M5EmbeddedOriginStateControlsViolation::ProofFreshnessIncomplete);
    }
}

fn validate_release_posture(
    packet: &M5EmbeddedOriginStateControlsPacket,
    violations: &mut Vec<M5EmbeddedOriginStateControlsViolation>,
) {
    let posture = &packet.release_posture;
    if posture.proof_packet_ref.trim().is_empty()
        || posture.boundary_audit_ref.trim().is_empty()
        || !posture.support_export_parity_required
        || !posture.accessibility_parity_required
    {
        violations.push(M5EmbeddedOriginStateControlsViolation::ReleasePostureIncomplete);
    }
}

/// Proves the two acceptance criteria are exercised by the packet's resolved examples, not merely
/// asserted by governance bools.
fn validate_acceptance_criteria(
    packet: &M5EmbeddedOriginStateControlsPacket,
    violations: &mut Vec<M5EmbeddedOriginStateControlsViolation>,
) {
    let bar_examples = || {
        packet
            .controls_rows
            .iter()
            .flat_map(|row| row.embedded_origin_bar_examples.iter())
    };
    let panel_examples = || {
        packet
            .controls_rows
            .iter()
            .flat_map(|row| row.embedded_state_panel_examples.iter())
    };

    // AC1: a contributed webview never appears without visible owner/origin chrome and
    // capability-limit disclosure — clean bars disclose both, an undisclosed-owner bar degrades, an
    // undisclosed-capability bar degrades, and no clean bar hides owner/origin or capability limits.
    // Every required embedded state is also honestly rendered by a clean panel.
    let covers_owner_and_capability = bar_examples()
        .any(|ex| ex.is_clean() && ex.owner_origin_disclosed && ex.capability_limits_disclosed);
    let owner_unstated_degrades = bar_examples().any(|ex| {
        ex.degrade_reason == Some(M5EmbeddedOriginBarDegradeReason::OwnerOrOriginUnstated)
    });
    let capability_unstated_degrades = bar_examples().any(|ex| {
        ex.degrade_reason == Some(M5EmbeddedOriginBarDegradeReason::CapabilityLimitsUnstated)
    });
    let no_clean_bar_hides =
        bar_examples().all(|ex| !(ex.is_clean() && ex.hides_owner_or_capability()));
    let clean_states: BTreeSet<&str> = panel_examples()
        .filter(|ex| ex.is_clean())
        .map(|ex| ex.state_class.as_str())
        .collect();
    let covers_required_states = M5EmbeddedStateClass::NON_LIVE
        .iter()
        .all(|state| clean_states.contains(state.as_str()));
    if !(covers_owner_and_capability
        && owner_unstated_degrades
        && capability_unstated_degrades
        && no_clean_bar_hides
        && covers_required_states)
    {
        violations.push(M5EmbeddedOriginStateControlsViolation::Ac1NotProven);
    }

    // AC2: no embedded contributed surface imitates native permission / trust / update /
    // confirmation UI — at least one origin bar and one state panel native-imitation degrades, and
    // no clean example imitates.
    let bar_imitation_degrades = bar_examples().any(|ex| {
        ex.degrade_reason == Some(M5EmbeddedOriginBarDegradeReason::ImitatesNativePermissionUi)
            && ex.imitates_native_permission_ui
    });
    let panel_imitation_degrades = panel_examples().any(|ex| {
        ex.degrade_reason == Some(M5EmbeddedStatePanelDegradeReason::ImitatesNativePermissionUi)
            && ex.imitates_native_permission_ui
    });
    let no_clean_imitates = bar_examples()
        .all(|ex| !(ex.is_clean() && ex.imitates_native_permission_ui))
        && panel_examples().all(|ex| !(ex.is_clean() && ex.imitates_native_permission_ui));
    if !(bar_imitation_degrades && panel_imitation_degrades && no_clean_imitates) {
        violations.push(M5EmbeddedOriginStateControlsViolation::Ac2NotProven);
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
