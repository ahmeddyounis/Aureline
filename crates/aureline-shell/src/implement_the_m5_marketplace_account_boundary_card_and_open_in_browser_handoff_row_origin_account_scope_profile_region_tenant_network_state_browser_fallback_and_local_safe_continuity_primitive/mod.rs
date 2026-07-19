//! Implemented M5 marketplace/account boundary cards and open-in-browser handoff rows.
//!
//! The frozen [embedded-boundary component matrix][matrix] names the reusable embedded /
//! browser-handoff boundary UI components and locks their controlled vocabulary. This module is
//! the third implement lane over that matrix: it turns the two provider-pane / browser-handoff
//! components — the **marketplace/account boundary card** and the **open-in-browser handoff row** —
//! into resolvers that produce export-safe, honest projections instead of anonymous product chrome.
//!
//! Two acceptance criteria drive the resolvers:
//!
//! * **AC1 — marketplace/account panes never hide identity, region, or service ownership behind
//!   generic product chrome.** [`resolve_marketplace_account_boundary_card`] refuses to read as a
//!   clean card when the owner/origin is undisclosed or untrusted, when the account scope is
//!   unstated, when the current profile or the region/tenant cue is unstated where it is relevant,
//!   when the network state or the browser fallback / retry path is unstated, or when generic
//!   product chrome conceals the identity, region, or service ownership. A clean card names its
//!   owner/origin, account scope, current profile, region/tenant (where relevant), network state,
//!   and browser fallback or retry path.
//! * **AC2 — browser handoffs preserve object identity and reason-for-handoff instead of dropping
//!   users onto a generic landing page.** [`resolve_open_in_browser_handoff_row`] degrades the
//!   moment the current object identity is dropped, the handoff lands on a generic page, the
//!   reason the in-product lane ended is unstated, or the local-safe continuity after handoff is
//!   left implicit.
//!
//! The resolvers reuse the frozen matrix vocabulary directly — the single controlled
//! [`M5EmbeddedBoundaryDisposition`] boundary-disposition vocabulary, the [`M5EmbeddedAccountScope`]
//! account-scope vocabulary, the [`M5EmbeddedFreshnessState`] freshness vocabulary, the
//! [`WebviewOwnerClass`] owner/origin vocabulary, the [`BrowserHandoffKind`] browser-fallback
//! vocabulary, the [`HandoffReasonClass`] handoff-reason vocabulary, and the [`FallbackStateClass`]
//! fallback vocabulary — so this lane can never fork its own owner, origin, account-scope, fallback,
//! or handoff-reason wording.
//!
//! [matrix]: crate::freeze_the_m5_docs_pane_header_embedded_origin_bar_boundary_fact_grid_marketplace_account_boundary_card_auth_handoff_card_remote_service_dashboard_header_open_in_browser_handoff_row_and_embedded_state_panel_component_matrix

mod seed;
#[cfg(test)]
mod tests;

pub use seed::{
    seeded_m5_marketplace_handoff_controls,
    seeded_m5_marketplace_handoff_controls_account_preview_narrowed,
    seeded_m5_marketplace_handoff_controls_marketplace_beta_narrowed,
    M5_MARKETPLACE_HANDOFF_CONTROLS_PACKET_ID,
};

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

use crate::freeze_the_m5_docs_pane_header_embedded_origin_bar_boundary_fact_grid_marketplace_account_boundary_card_auth_handoff_card_remote_service_dashboard_header_open_in_browser_handoff_row_and_embedded_state_panel_component_matrix::{
    M5EmbeddedAccessibilityRoute, M5EmbeddedAccountScope, M5EmbeddedBoundaryDisposition,
    M5EmbeddedConsumerSurface, M5EmbeddedDeploymentLine, M5EmbeddedDowngradeTrigger,
    M5EmbeddedFreshnessState, M5EmbeddedQualificationClass, M5EmbeddedRequiredLabel,
    M5_EMBEDDED_BOUNDARY_COMPONENT_DOC_REF, M5_EMBEDDED_BOUNDARY_COMPONENT_SCHEMA_REF,
    M5_MARKETPLACE_ACCOUNT_BOUNDARY_CARD_SCHEMA_REF, M5_OPEN_IN_BROWSER_HANDOFF_ROW_SCHEMA_REF,
};
use crate::m5_auth_boundaries::{
    BrowserHandoffKind, FallbackStateClass, HandoffReasonClass, WebviewOwnerClass,
    M5_AUTH_BOUNDARY_CONTRACT_DOC_REF,
};

/// Stable record-kind tag carried by [`M5MarketplaceHandoffControlsPacket`].
pub const M5_MARKETPLACE_HANDOFF_CONTROLS_RECORD_KIND: &str =
    "implement_m5_marketplace_account_boundary_card_and_open_in_browser_handoff_row_controls";

/// Schema version for M5 marketplace/account boundary-card and open-in-browser handoff-row controls.
pub const M5_MARKETPLACE_HANDOFF_CONTROLS_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the combined controls boundary schema.
pub const M5_MARKETPLACE_HANDOFF_CONTROLS_SCHEMA_REF: &str =
    "schemas/ui/m5-marketplace-account-boundary-open-in-browser-handoff-controls.schema.json";

/// Repo-relative path of the controls doc.
pub const M5_MARKETPLACE_HANDOFF_CONTROLS_DOC_REF: &str =
    "docs/help/m5_marketplace_account_boundary_and_open_in_browser_handoff_controls.md";

/// Repo-relative path of the checked support-export artifact.
pub const M5_MARKETPLACE_HANDOFF_CONTROLS_ARTIFACT_REF: &str =
    "artifacts/release/m5-marketplace-account-boundary-open-in-browser-handoff-controls-proof/support_export.json";

/// Repo-relative path of the checked machine-readable controls CSV.
pub const M5_MARKETPLACE_HANDOFF_CONTROLS_CSV_REF: &str =
    "artifacts/release/m5-marketplace-account-boundary-open-in-browser-handoff-controls-proof/matrix.csv";

/// Repo-relative path of the checked Markdown design report.
pub const M5_MARKETPLACE_HANDOFF_CONTROLS_REPORT_REF: &str =
    "artifacts/release/m5-marketplace-account-boundary-open-in-browser-handoff-controls-proof/summary.md";

/// Repo-relative path of the protected fixture directory.
pub const M5_MARKETPLACE_HANDOFF_CONTROLS_FIXTURE_DIR: &str =
    "fixtures/ui/m5-marketplace-account-boundary-open-in-browser-handoff-controls";

/// Consumer surface a marketplace/handoff controls row projects onto. Reuses the frozen matrix
/// consumer-surface taxonomy so no lane invents a parallel surface set.
pub type M5MarketplaceHandoffConsumerSurface = M5EmbeddedConsumerSurface;

/// Handoff-reason classes an open-in-browser handoff row may carry, pinned locally because
/// [`HandoffReasonClass`] does not expose an `ALL` array. Order matches its declaration.
pub const BOUND_HANDOFF_REASONS: [HandoffReasonClass; 4] = [
    HandoffReasonClass::AuthenticateWithProvider,
    HandoffReasonClass::AuthorizeDeviceCode,
    HandoffReasonClass::ViewProviderContent,
    HandoffReasonClass::OpenVendorResource,
];

/// Fallback-state classes an open-in-browser handoff row may carry, pinned locally because
/// [`FallbackStateClass`] does not expose an `ALL` array. Order matches its declaration.
pub const BOUND_FALLBACK_STATES: [FallbackStateClass; 4] = [
    FallbackStateClass::LocalContinuityPreserved,
    FallbackStateClass::RetryHandoffInApp,
    FallbackStateClass::ManualCodeEntry,
    FallbackStateClass::CopyLinkForManualOpen,
];

/// The single controlled network-state class a marketplace/account boundary card can name, so the
/// network state is never left implicit next to hosted or provider content.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5MarketplaceNetworkState {
    /// Online with a healthy connection.
    Online,
    /// Online but with degraded connectivity, labelled as degraded.
    DegradedConnectivity,
    /// Offline with no live connection, labelled as offline.
    Offline,
    /// Blocked by a captive portal or network policy, labelled as blocked.
    CaptivePortalOrBlocked,
    /// The network state cannot currently be determined.
    NetworkStateUnknown,
}

impl M5MarketplaceNetworkState {
    /// Every network state, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::Online,
        Self::DegradedConnectivity,
        Self::Offline,
        Self::CaptivePortalOrBlocked,
        Self::NetworkStateUnknown,
    ];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Online => "online",
            Self::DegradedConnectivity => "degraded_connectivity",
            Self::Offline => "offline",
            Self::CaptivePortalOrBlocked => "captive_portal_or_blocked",
            Self::NetworkStateUnknown => "network_state_unknown",
        }
    }

    /// Whether this network state is known (anything but unknown).
    pub const fn is_known(self) -> bool {
        !matches!(self, Self::NetworkStateUnknown)
    }
}

/// One mandatory rendered part a marketplace/account boundary card or an open-in-browser handoff
/// row must be able to show, so no boundary truth is left implicit.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5MarketplaceHandoffAnatomyPart {
    /// The component's stable identity / what it represents.
    Identity,
    /// The component's current typed boundary disposition.
    State,
    /// The non-visual keyboard route to the component.
    KeyboardRoute,
    /// The owner / origin class behind the surface (boundary card).
    OwnerOrigin,
    /// The account scope the surface is scoped to (boundary card).
    AccountScope,
    /// The current profile the surface renders under (boundary card).
    CurrentProfile,
    /// The region / tenant cue where relevant (boundary card).
    RegionOrTenant,
    /// The network state of the surface (boundary card).
    NetworkState,
    /// The browser fallback or retry path (boundary card).
    BrowserFallback,
    /// The preserved current object identity (handoff row).
    ObjectIdentity,
    /// The reason the in-product lane ended (handoff row).
    HandoffReason,
    /// The local-safe continuity after handoff (handoff row).
    LocalContinuity,
}

impl M5MarketplaceHandoffAnatomyPart {
    /// Every anatomy part, in declaration order.
    pub const ALL: [Self; 12] = [
        Self::Identity,
        Self::State,
        Self::KeyboardRoute,
        Self::OwnerOrigin,
        Self::AccountScope,
        Self::CurrentProfile,
        Self::RegionOrTenant,
        Self::NetworkState,
        Self::BrowserFallback,
        Self::ObjectIdentity,
        Self::HandoffReason,
        Self::LocalContinuity,
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
            Self::AccountScope => "account_scope",
            Self::CurrentProfile => "current_profile",
            Self::RegionOrTenant => "region_or_tenant",
            Self::NetworkState => "network_state",
            Self::BrowserFallback => "browser_fallback",
            Self::ObjectIdentity => "object_identity",
            Self::HandoffReason => "handoff_reason",
            Self::LocalContinuity => "local_continuity",
        }
    }
}

/// Next safe action a component surfaces so a user is never left without a route out.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5MarketplaceHandoffNextAction {
    /// Open the source externally in the browser.
    OpenInBrowser,
    /// Retry the handoff from within Aureline.
    RetryHandoff,
    /// Review the account scope the surface is bound to.
    ReviewAccountScope,
    /// Switch or confirm the current profile / region.
    SwitchProfile,
    /// Review diagnostics for the unavailable signal.
    ReviewDiagnostics,
    /// No action is needed; the component is clean.
    NoActionNeeded,
}

impl M5MarketplaceHandoffNextAction {
    /// Every next action, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::OpenInBrowser,
        Self::RetryHandoff,
        Self::ReviewAccountScope,
        Self::SwitchProfile,
        Self::ReviewDiagnostics,
        Self::NoActionNeeded,
    ];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::OpenInBrowser => "open_in_browser",
            Self::RetryHandoff => "retry_handoff",
            Self::ReviewAccountScope => "review_account_scope",
            Self::SwitchProfile => "switch_profile",
            Self::ReviewDiagnostics => "review_diagnostics",
            Self::NoActionNeeded => "no_action_needed",
        }
    }
}

/// Field a marketplace/handoff controls row exposes in the support export.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5MarketplaceHandoffExportField {
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
    /// The owner / origin named by the boundary card.
    OwnerOrigin,
    /// The account scope named by the boundary card.
    AccountScope,
    /// The current profile named by the boundary card.
    CurrentProfile,
    /// The network state named by the boundary card.
    NetworkState,
    /// The browser fallback / handoff kind named by the surface.
    BrowserFallbackKind,
    /// The accountable owner role.
    OwnerRole,
}

impl M5MarketplaceHandoffExportField {
    /// Every export field, in declaration order.
    pub const ALL: [Self; 11] = [
        Self::ConsumerSurface,
        Self::ComponentFamilies,
        Self::BoundaryDispositions,
        Self::DegradeReasons,
        Self::Qualification,
        Self::OwnerOrigin,
        Self::AccountScope,
        Self::CurrentProfile,
        Self::NetworkState,
        Self::BrowserFallbackKind,
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
            Self::AccountScope => "account_scope",
            Self::CurrentProfile => "current_profile",
            Self::NetworkState => "network_state",
            Self::BrowserFallbackKind => "browser_fallback_kind",
            Self::OwnerRole => "owner_role",
        }
    }
}

/// Reason a marketplace/account boundary card degraded below a clean, fully-legible state. The
/// degrade-first ladder returns one of these instead of ever letting an identity-concealing card
/// read as a clean pass.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5MarketplaceCardDegradeReason {
    /// The owner / origin (service ownership) is undisclosed or untrusted (AC1 violation).
    OwnerOrOriginUnstated,
    /// Generic product chrome conceals the identity, region, or service ownership (AC1 violation).
    GenericChromeConcealsIdentity,
    /// The account scope is unstated (AC1 violation).
    AccountScopeUnstated,
    /// The current profile or the region/tenant cue is unstated where it is relevant (AC1
    /// violation).
    ProfileOrRegionUnstated,
    /// The network state or the browser fallback / retry path is unstated.
    NetworkStateOrFallbackUnstated,
    /// The supporting proof packet has gone stale.
    ProofStale,
}

impl M5MarketplaceCardDegradeReason {
    /// Every degrade reason, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::OwnerOrOriginUnstated,
        Self::GenericChromeConcealsIdentity,
        Self::AccountScopeUnstated,
        Self::ProfileOrRegionUnstated,
        Self::NetworkStateOrFallbackUnstated,
        Self::ProofStale,
    ];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::OwnerOrOriginUnstated => "owner_or_origin_unstated",
            Self::GenericChromeConcealsIdentity => "generic_chrome_conceals_identity",
            Self::AccountScopeUnstated => "account_scope_unstated",
            Self::ProfileOrRegionUnstated => "profile_or_region_unstated",
            Self::NetworkStateOrFallbackUnstated => "network_state_or_fallback_unstated",
            Self::ProofStale => "proof_stale",
        }
    }

    /// Next safe action for this reason.
    pub const fn next_action(self) -> M5MarketplaceHandoffNextAction {
        match self {
            Self::OwnerOrOriginUnstated
            | Self::GenericChromeConcealsIdentity
            | Self::ProofStale => M5MarketplaceHandoffNextAction::ReviewDiagnostics,
            Self::AccountScopeUnstated => M5MarketplaceHandoffNextAction::ReviewAccountScope,
            Self::ProfileOrRegionUnstated => M5MarketplaceHandoffNextAction::SwitchProfile,
            Self::NetworkStateOrFallbackUnstated => M5MarketplaceHandoffNextAction::RetryHandoff,
        }
    }

    /// The matching downgrade trigger recorded on the frozen matrix.
    pub const fn downgrade_trigger(self) -> M5EmbeddedDowngradeTrigger {
        match self {
            Self::OwnerOrOriginUnstated => M5EmbeddedDowngradeTrigger::OwnerOrOriginUnstated,
            Self::GenericChromeConcealsIdentity => {
                M5EmbeddedDowngradeTrigger::GenericChromeWordingUsed
            }
            Self::AccountScopeUnstated | Self::ProfileOrRegionUnstated => {
                M5EmbeddedDowngradeTrigger::AccountScopeUnstated
            }
            Self::NetworkStateOrFallbackUnstated => {
                M5EmbeddedDowngradeTrigger::BrowserFallbackHiddenInMenusOnly
            }
            Self::ProofStale => M5EmbeddedDowngradeTrigger::ProofStale,
        }
    }
}

/// Reason an open-in-browser handoff row degraded below a clean, fully-legible state.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5OpenInBrowserRowDegradeReason {
    /// The current object identity was dropped by the handoff (AC2 violation).
    ObjectIdentityDropped,
    /// The handoff lands on a generic page instead of the current object (AC2 violation).
    LandsOnGenericPage,
    /// The reason the in-product lane ended is unstated (AC2 violation).
    HandoffReasonUnstated,
    /// The local-safe continuity after handoff is left implicit.
    LocalContinuityUnstated,
    /// The browser fallback / retry path is unavailable.
    BrowserFallbackUnavailable,
    /// The supporting proof packet has gone stale.
    ProofStale,
}

impl M5OpenInBrowserRowDegradeReason {
    /// Every degrade reason, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::ObjectIdentityDropped,
        Self::LandsOnGenericPage,
        Self::HandoffReasonUnstated,
        Self::LocalContinuityUnstated,
        Self::BrowserFallbackUnavailable,
        Self::ProofStale,
    ];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ObjectIdentityDropped => "object_identity_dropped",
            Self::LandsOnGenericPage => "lands_on_generic_page",
            Self::HandoffReasonUnstated => "handoff_reason_unstated",
            Self::LocalContinuityUnstated => "local_continuity_unstated",
            Self::BrowserFallbackUnavailable => "browser_fallback_unavailable",
            Self::ProofStale => "proof_stale",
        }
    }

    /// Next safe action for this reason.
    pub const fn next_action(self) -> M5MarketplaceHandoffNextAction {
        match self {
            Self::ObjectIdentityDropped
            | Self::HandoffReasonUnstated
            | Self::LocalContinuityUnstated
            | Self::ProofStale => M5MarketplaceHandoffNextAction::ReviewDiagnostics,
            Self::LandsOnGenericPage => M5MarketplaceHandoffNextAction::OpenInBrowser,
            Self::BrowserFallbackUnavailable => M5MarketplaceHandoffNextAction::RetryHandoff,
        }
    }

    /// The matching downgrade trigger recorded on the frozen matrix.
    pub const fn downgrade_trigger(self) -> M5EmbeddedDowngradeTrigger {
        match self {
            Self::ObjectIdentityDropped
            | Self::LandsOnGenericPage
            | Self::HandoffReasonUnstated => M5EmbeddedDowngradeTrigger::GenericChromeWordingUsed,
            Self::LocalContinuityUnstated | Self::BrowserFallbackUnavailable => {
                M5EmbeddedDowngradeTrigger::BrowserFallbackHiddenInMenusOnly
            }
            Self::ProofStale => M5EmbeddedDowngradeTrigger::ProofStale,
        }
    }
}

/// Maps a boundary card's owner class and network state to the single controlled boundary
/// disposition.
fn disposition_for_card(
    owner: WebviewOwnerClass,
    network: M5MarketplaceNetworkState,
) -> M5EmbeddedBoundaryDisposition {
    use M5EmbeddedBoundaryDisposition as D;
    match network {
        M5MarketplaceNetworkState::Offline => D::OfflineSnapshot,
        M5MarketplaceNetworkState::CaptivePortalOrBlocked => D::ProviderBlocked,
        _ => match owner {
            WebviewOwnerClass::ExtensionOwned => D::CapabilityLimited,
            WebviewOwnerClass::ProviderOwned => D::LiveProviderOwned,
            WebviewOwnerClass::FirstPartyEmbedded => D::LiveFirstPartyLocal,
            WebviewOwnerClass::UnknownUntrusted => D::NotEvaluated,
        },
    }
}

/// Input to [`resolve_marketplace_account_boundary_card`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M5MarketplaceAccountBoundaryCardResolutionInput {
    /// Stable identity of the card instance.
    pub card_id: String,
    /// The owner / origin class (service ownership) behind the surface.
    pub owner_class: WebviewOwnerClass,
    /// True when the owner / origin is disclosed on the card chrome, never menu-only.
    pub owner_origin_disclosed: bool,
    /// The account scope the surface is scoped to.
    pub account_scope: M5EmbeddedAccountScope,
    /// True when the account scope is disclosed on the card chrome.
    pub account_scope_disclosed: bool,
    /// The current profile the surface renders under (empty means unstated).
    pub current_profile: String,
    /// The region / tenant cue (empty means unstated).
    pub region_or_tenant: String,
    /// The network state of the surface.
    pub network_state: M5MarketplaceNetworkState,
    /// The browser-handoff kind the fallback / retry path uses, if any.
    pub browser_fallback_kind: Option<BrowserHandoffKind>,
    /// True when a browser fallback or retry path is offered on the card, never menu-only.
    pub browser_fallback_available: bool,
    /// True when generic product chrome conceals the identity, region, or service ownership.
    pub conceals_identity_behind_generic_chrome: bool,
    /// The freshness / last-updated state of the surface.
    pub freshness: M5EmbeddedFreshnessState,
    /// True when the supporting proof packet is fresh.
    pub proof_fresh: bool,
}

/// Resolved, export-safe marketplace/account boundary card projection.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ResolvedMarketplaceAccountBoundaryCard {
    /// Stable identity of the card instance.
    pub card_id: String,
    /// Owner / origin token named by the card.
    pub owner_origin: String,
    /// Account-scope token named by the card.
    pub account_scope: String,
    /// Current profile named by the card.
    pub current_profile: String,
    /// Region / tenant cue named by the card.
    pub region_or_tenant: String,
    /// Network-state token named by the card.
    pub network_state: String,
    /// Single controlled boundary disposition carried by the card.
    pub boundary_disposition: M5EmbeddedBoundaryDisposition,
    /// Browser-fallback handoff token, if any.
    pub browser_fallback_kind: Option<String>,
    /// Whether a browser fallback / retry path is offered.
    pub browser_fallback_available: bool,
    /// Freshness token named by the card.
    pub freshness: String,
    /// Degrade reason, if the card could not read as a clean, fully-legible state.
    pub degrade_reason: Option<M5MarketplaceCardDegradeReason>,
    /// Next safe action offered.
    pub next_action: M5MarketplaceHandoffNextAction,
    /// AC1: whether the owner / origin is disclosed on the card chrome.
    pub owner_origin_disclosed: bool,
    /// AC1: whether the account scope is disclosed on the card chrome.
    pub account_scope_disclosed: bool,
    /// Guardrail (MUST be `false` on a clean card): generic product chrome conceals the identity,
    /// region, or service ownership.
    pub conceals_identity_behind_generic_chrome: bool,
}

impl M5ResolvedMarketplaceAccountBoundaryCard {
    /// Whether this card reads as a clean, fully-legible state.
    pub fn is_clean(&self) -> bool {
        self.degrade_reason.is_none()
    }

    /// Whether this card hides its identity, region, or service ownership (an AC1 violation).
    pub fn hides_identity_region_or_ownership(&self) -> bool {
        !self.owner_origin_disclosed
            || !self.account_scope_disclosed
            || self.conceals_identity_behind_generic_chrome
    }
}

/// Input to [`resolve_open_in_browser_handoff_row`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M5OpenInBrowserHandoffRowResolutionInput {
    /// Stable identity of the row instance.
    pub row_id: String,
    /// The kind of browser handoff the row describes.
    pub handoff_kind: BrowserHandoffKind,
    /// Why the handoff happens (why the in-product lane ended).
    pub handoff_reason: HandoffReasonClass,
    /// Opaque reference of the current object identity being carried across the handoff.
    pub object_ref: String,
    /// Reviewer-facing label of the current object.
    pub object_label: String,
    /// True when the current object identity is preserved across the handoff.
    pub object_identity_preserved: bool,
    /// True when the reason the in-product lane ended is explained on the row.
    pub handoff_reason_stated: bool,
    /// What survives if the handoff is blocked or the browser cannot open.
    pub fallback_state: FallbackStateClass,
    /// True when the local-safe continuity after handoff is explicit on the row.
    pub local_continuity_explicit: bool,
    /// True when a browser fallback / retry path is available.
    pub browser_fallback_available: bool,
    /// True when the handoff lands on a generic page instead of the current object.
    pub lands_on_generic_page: bool,
    /// True when the supporting proof packet is fresh.
    pub proof_fresh: bool,
}

/// Resolved, export-safe open-in-browser handoff row projection.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ResolvedOpenInBrowserHandoffRow {
    /// Stable identity of the row instance.
    pub row_id: String,
    /// Browser-handoff-kind token named by the row.
    pub handoff_kind: String,
    /// Handoff-reason token named by the row.
    pub handoff_reason: String,
    /// Opaque reference of the current object identity.
    pub object_ref: String,
    /// Reviewer-facing label of the current object.
    pub object_label: String,
    /// Single controlled boundary disposition carried by the row.
    pub boundary_disposition: M5EmbeddedBoundaryDisposition,
    /// Fallback-state token named by the row.
    pub fallback_state: String,
    /// Whether the current object identity is preserved across the handoff.
    pub object_identity_preserved: bool,
    /// Whether the reason the in-product lane ended is explained.
    pub handoff_reason_stated: bool,
    /// Whether the local-safe continuity after handoff is explicit.
    pub local_continuity_explicit: bool,
    /// Whether a browser fallback / retry path is available.
    pub browser_fallback_available: bool,
    /// Degrade reason, if the row could not read as a clean, fully-legible state.
    pub degrade_reason: Option<M5OpenInBrowserRowDegradeReason>,
    /// Next safe action offered.
    pub next_action: M5MarketplaceHandoffNextAction,
    /// Guardrail (MUST be `false` on a clean row): the handoff lands on a generic page instead of
    /// the current object.
    pub lands_on_generic_page: bool,
}

impl M5ResolvedOpenInBrowserHandoffRow {
    /// Whether this row reads as a clean, fully-legible state.
    pub fn is_clean(&self) -> bool {
        self.degrade_reason.is_none()
    }

    /// Whether this row drops the current object identity or lands on a generic page (an AC2
    /// violation).
    pub fn drops_identity_or_lands_generic(&self) -> bool {
        !self.object_identity_preserved || self.lands_on_generic_page
    }
}

/// Error emitted when a resolver input carries invalid or forbidden material.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum M5MarketplaceHandoffResolutionError {
    /// The boundary-card id was empty.
    EmptyCardId,
    /// The handoff-row id was empty.
    EmptyRowId,
    /// A field carried forbidden raw material (secret / endpoint).
    ForbiddenMaterial,
}

impl M5MarketplaceHandoffResolutionError {
    /// Stable token used in tests and exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::EmptyCardId => "empty_card_id",
            Self::EmptyRowId => "empty_row_id",
            Self::ForbiddenMaterial => "forbidden_material",
        }
    }
}

impl fmt::Display for M5MarketplaceHandoffResolutionError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "m5 marketplace/handoff resolution error: {}",
            self.as_str()
        )
    }
}

impl Error for M5MarketplaceHandoffResolutionError {}

/// Resolves a marketplace/account boundary card, proving AC1: a marketplace/account pane never
/// hides its identity, region, or service ownership behind generic product chrome, and always names
/// its owner/origin, account scope, current profile, region/tenant (where relevant), network state,
/// and browser fallback or retry path.
pub fn resolve_marketplace_account_boundary_card(
    input: M5MarketplaceAccountBoundaryCardResolutionInput,
) -> Result<M5ResolvedMarketplaceAccountBoundaryCard, M5MarketplaceHandoffResolutionError> {
    if input.card_id.trim().is_empty() {
        return Err(M5MarketplaceHandoffResolutionError::EmptyCardId);
    }
    if string_is_forbidden(&input.card_id)
        || string_is_forbidden(&input.current_profile)
        || string_is_forbidden(&input.region_or_tenant)
    {
        return Err(M5MarketplaceHandoffResolutionError::ForbiddenMaterial);
    }

    let owner_origin_disclosed =
        input.owner_origin_disclosed && input.owner_class != WebviewOwnerClass::UnknownUntrusted;
    let account_scope_disclosed = input.account_scope_disclosed
        && input.account_scope != M5EmbeddedAccountScope::AccountScopeUnknown;
    let profile_required = matches!(
        input.account_scope,
        M5EmbeddedAccountScope::PersonalAccount
            | M5EmbeddedAccountScope::OrgWorkspace
            | M5EmbeddedAccountScope::ManagedTenant
    );
    let region_relevant = matches!(
        input.account_scope,
        M5EmbeddedAccountScope::OrgWorkspace | M5EmbeddedAccountScope::ManagedTenant
    );
    let profile_or_region_unstated = (profile_required && input.current_profile.trim().is_empty())
        || (region_relevant && input.region_or_tenant.trim().is_empty());
    let network_or_fallback_unstated =
        !input.network_state.is_known() || !input.browser_fallback_available;

    let degrade_reason = if !owner_origin_disclosed {
        Some(M5MarketplaceCardDegradeReason::OwnerOrOriginUnstated)
    } else if input.conceals_identity_behind_generic_chrome {
        Some(M5MarketplaceCardDegradeReason::GenericChromeConcealsIdentity)
    } else if !account_scope_disclosed {
        Some(M5MarketplaceCardDegradeReason::AccountScopeUnstated)
    } else if profile_or_region_unstated {
        Some(M5MarketplaceCardDegradeReason::ProfileOrRegionUnstated)
    } else if network_or_fallback_unstated {
        Some(M5MarketplaceCardDegradeReason::NetworkStateOrFallbackUnstated)
    } else if !input.proof_fresh {
        Some(M5MarketplaceCardDegradeReason::ProofStale)
    } else {
        None
    };

    let boundary_disposition = match degrade_reason {
        Some(_) => M5EmbeddedBoundaryDisposition::NotEvaluated,
        None => disposition_for_card(input.owner_class, input.network_state),
    };
    let next_action = match degrade_reason {
        Some(reason) => reason.next_action(),
        None => M5MarketplaceHandoffNextAction::NoActionNeeded,
    };

    Ok(M5ResolvedMarketplaceAccountBoundaryCard {
        card_id: input.card_id,
        owner_origin: input.owner_class.as_str().to_owned(),
        account_scope: input.account_scope.as_str().to_owned(),
        current_profile: input.current_profile,
        region_or_tenant: input.region_or_tenant,
        network_state: input.network_state.as_str().to_owned(),
        boundary_disposition,
        browser_fallback_kind: input.browser_fallback_kind.map(|k| k.as_str().to_owned()),
        browser_fallback_available: input.browser_fallback_available,
        freshness: input.freshness.as_str().to_owned(),
        degrade_reason,
        next_action,
        owner_origin_disclosed,
        account_scope_disclosed,
        conceals_identity_behind_generic_chrome: input.conceals_identity_behind_generic_chrome,
    })
}

/// Resolves an open-in-browser handoff row, proving AC2: a browser handoff preserves the current
/// object identity and reason-for-handoff and keeps local-safe continuity explicit, instead of
/// dropping the user onto a generic landing page.
pub fn resolve_open_in_browser_handoff_row(
    input: M5OpenInBrowserHandoffRowResolutionInput,
) -> Result<M5ResolvedOpenInBrowserHandoffRow, M5MarketplaceHandoffResolutionError> {
    if input.row_id.trim().is_empty() {
        return Err(M5MarketplaceHandoffResolutionError::EmptyRowId);
    }
    if string_is_forbidden(&input.row_id)
        || string_is_forbidden(&input.object_ref)
        || string_is_forbidden(&input.object_label)
    {
        return Err(M5MarketplaceHandoffResolutionError::ForbiddenMaterial);
    }

    let object_identity_preserved =
        input.object_identity_preserved && !input.object_ref.trim().is_empty();

    let degrade_reason = if !object_identity_preserved {
        Some(M5OpenInBrowserRowDegradeReason::ObjectIdentityDropped)
    } else if input.lands_on_generic_page {
        Some(M5OpenInBrowserRowDegradeReason::LandsOnGenericPage)
    } else if !input.handoff_reason_stated {
        Some(M5OpenInBrowserRowDegradeReason::HandoffReasonUnstated)
    } else if !input.local_continuity_explicit {
        Some(M5OpenInBrowserRowDegradeReason::LocalContinuityUnstated)
    } else if !input.browser_fallback_available {
        Some(M5OpenInBrowserRowDegradeReason::BrowserFallbackUnavailable)
    } else if !input.proof_fresh {
        Some(M5OpenInBrowserRowDegradeReason::ProofStale)
    } else {
        None
    };

    let boundary_disposition = match degrade_reason {
        Some(_) => M5EmbeddedBoundaryDisposition::NotEvaluated,
        None => M5EmbeddedBoundaryDisposition::BrowserHandoffOnly,
    };
    let next_action = match degrade_reason {
        Some(reason) => reason.next_action(),
        None => M5MarketplaceHandoffNextAction::NoActionNeeded,
    };

    Ok(M5ResolvedOpenInBrowserHandoffRow {
        row_id: input.row_id,
        handoff_kind: input.handoff_kind.as_str().to_owned(),
        handoff_reason: input.handoff_reason.as_str().to_owned(),
        object_ref: input.object_ref,
        object_label: input.object_label,
        boundary_disposition,
        fallback_state: input.fallback_state.as_str().to_owned(),
        object_identity_preserved,
        handoff_reason_stated: input.handoff_reason_stated,
        local_continuity_explicit: input.local_continuity_explicit,
        browser_fallback_available: input.browser_fallback_available,
        degrade_reason,
        next_action,
        lands_on_generic_page: input.lands_on_generic_page,
    })
}

/// One controls row: one consumer surface bound to the resolved marketplace/account boundary card
/// and open-in-browser handoff row examples it must project honestly.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5MarketplaceHandoffControlsRow {
    /// Consumer surface this row projects onto.
    pub consumer_surface: M5MarketplaceHandoffConsumerSurface,
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
    pub anatomy_parts: Vec<M5MarketplaceHandoffAnatomyPart>,
    /// Export fields exposed (must include the mandatory five).
    pub export_fields: Vec<M5MarketplaceHandoffExportField>,
    /// Downgrade triggers that apply to this row.
    pub downgrade_triggers: Vec<M5EmbeddedDowngradeTrigger>,
    /// Resolved marketplace/account boundary card examples.
    pub marketplace_account_boundary_card_examples: Vec<M5ResolvedMarketplaceAccountBoundaryCard>,
    /// Resolved open-in-browser handoff row examples.
    pub open_in_browser_handoff_row_examples: Vec<M5ResolvedOpenInBrowserHandoffRow>,
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

impl M5MarketplaceHandoffControlsRow {
    fn declares_mandatory_anatomy(&self) -> bool {
        let present: BTreeSet<M5MarketplaceHandoffAnatomyPart> =
            self.anatomy_parts.iter().copied().collect();
        M5MarketplaceHandoffAnatomyPart::MANDATORY
            .iter()
            .all(|part| present.contains(part))
    }

    fn declares_mandatory_export_fields(&self) -> bool {
        let present: BTreeSet<M5MarketplaceHandoffExportField> =
            self.export_fields.iter().copied().collect();
        M5MarketplaceHandoffExportField::MANDATORY
            .iter()
            .all(|field| present.contains(field))
    }

    fn honours_invariants(&self) -> bool {
        !self.masquerades_as_native_approval_chrome
            && !self.hides_owner_origin_or_handoff_in_menus_only
            && !self.renders_stale_or_blocked_as_fresh_first_party_truth
            && !self.embeds_high_risk_approval_without_native_step_up
    }

    /// True when every resolved example on this row is honest: no clean card hides its identity,
    /// region, or service ownership, and no clean handoff row drops the object identity or lands on
    /// a generic page.
    fn examples_are_honest(&self) -> bool {
        self.marketplace_account_boundary_card_examples
            .iter()
            .all(|ex| !(ex.is_clean() && ex.hides_identity_region_or_ownership()))
            && self
                .open_in_browser_handoff_row_examples
                .iter()
                .all(|ex| !(ex.is_clean() && ex.drops_identity_or_lands_generic()))
    }
}

/// Self-describing controlled-vocabulary set frozen by the controls packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5MarketplaceHandoffVocabularySet {
    /// Boundary-disposition tokens (bound from the frozen matrix).
    pub boundary_dispositions: Vec<String>,
    /// Owner-class tokens (bound from the auth-boundary object model).
    pub owner_classes: Vec<String>,
    /// Account-scope tokens (bound from the frozen matrix).
    pub account_scopes: Vec<String>,
    /// Network-state tokens.
    pub network_states: Vec<String>,
    /// Browser-handoff-kind tokens (bound from the auth-boundary object model).
    pub browser_handoff_kinds: Vec<String>,
    /// Handoff-reason tokens (bound from the auth-boundary object model).
    pub handoff_reasons: Vec<String>,
    /// Fallback-state tokens (bound from the auth-boundary object model).
    pub fallback_states: Vec<String>,
    /// Freshness-state tokens (bound from the frozen matrix).
    pub freshness_states: Vec<String>,
    /// Boundary-card degrade-reason tokens.
    pub card_degrade_reasons: Vec<String>,
    /// Handoff-row degrade-reason tokens.
    pub row_degrade_reasons: Vec<String>,
    /// Anatomy-part tokens.
    pub anatomy_parts: Vec<String>,
    /// Next-action tokens.
    pub next_actions: Vec<String>,
    /// Export-field tokens.
    pub export_fields: Vec<String>,
    /// Consumer-surface tokens.
    pub consumer_surfaces: Vec<String>,
}

impl M5MarketplaceHandoffVocabularySet {
    /// Builds the canonical vocabulary set from the typed `ALL` arrays.
    pub fn canonical() -> Self {
        Self {
            boundary_dispositions: tokens(&M5EmbeddedBoundaryDisposition::ALL, |v| v.as_str()),
            owner_classes: tokens(&WebviewOwnerClass::ALL, |v| v.as_str()),
            account_scopes: tokens(&M5EmbeddedAccountScope::ALL, |v| v.as_str()),
            network_states: tokens(&M5MarketplaceNetworkState::ALL, |v| v.as_str()),
            browser_handoff_kinds: tokens(&BrowserHandoffKind::ALL, |v| v.as_str()),
            handoff_reasons: tokens(&BOUND_HANDOFF_REASONS, |v| v.as_str()),
            fallback_states: tokens(&BOUND_FALLBACK_STATES, |v| v.as_str()),
            freshness_states: tokens(&M5EmbeddedFreshnessState::ALL, |v| v.as_str()),
            card_degrade_reasons: tokens(&M5MarketplaceCardDegradeReason::ALL, |v| v.as_str()),
            row_degrade_reasons: tokens(&M5OpenInBrowserRowDegradeReason::ALL, |v| v.as_str()),
            anatomy_parts: tokens(&M5MarketplaceHandoffAnatomyPart::ALL, |v| v.as_str()),
            next_actions: tokens(&M5MarketplaceHandoffNextAction::ALL, |v| v.as_str()),
            export_fields: tokens(&M5MarketplaceHandoffExportField::ALL, |v| v.as_str()),
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
pub struct M5MarketplaceHandoffGovernanceReview {
    /// The boundary card always names its owner/origin (service ownership).
    pub card_names_owner_and_ownership: bool,
    /// The boundary card always discloses its account scope.
    pub card_discloses_account_scope: bool,
    /// The current profile and region/tenant are always explicit where relevant.
    pub profile_and_region_always_explicit: bool,
    /// The network state and browser fallback / retry path are always exposed.
    pub network_state_and_fallback_always_exposed: bool,
    /// Generic product chrome never conceals identity, region, or service ownership.
    pub generic_chrome_never_conceals_identity: bool,
    /// The handoff row always preserves the current object identity.
    pub handoff_row_preserves_object_identity: bool,
    /// The handoff row always states the reason the in-product lane ended.
    pub handoff_row_states_reason: bool,
    /// The handoff never drops the user onto a generic landing page.
    pub handoff_never_lands_on_generic_page: bool,
    /// Local-safe continuity is always explicit after handoff.
    pub local_continuity_always_explicit: bool,
    /// The lane reuses the frozen matrix vocabulary rather than inventing parallel wording.
    pub reuses_frozen_matrix_vocabulary: bool,
}

/// Consumer projection block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5MarketplaceHandoffConsumerProjection {
    /// Marketplace/account surfaces consume the shared owner/origin vocabulary.
    pub surfaces_consume_owner_origin_vocabulary: bool,
    /// Marketplace/account surfaces consume the shared account-scope vocabulary.
    pub surfaces_consume_account_scope_vocabulary: bool,
    /// Handoff rows consume the shared browser-fallback and handoff-reason vocabulary.
    pub handoff_rows_consume_shared_fallback_vocabulary: bool,
    /// Support / export reads a single canonical boundary source.
    pub support_export_reads_single_boundary_source: bool,
}

/// Proof freshness block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5MarketplaceHandoffProofFreshness {
    /// Proof-freshness SLO in hours.
    pub proof_freshness_slo_hours: u32,
    /// RFC 3339 timestamp of the last proof refresh.
    pub last_proof_refresh: String,
    /// True when stale proof automatically narrows the component.
    pub auto_narrow_on_stale: bool,
}

/// Release and support parity posture for the controls lane.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5MarketplaceHandoffReleasePosture {
    /// Ref of the supporting proof packet for the lane.
    pub proof_packet_ref: String,
    /// Ref of the supporting boundary audit for the lane.
    pub boundary_audit_ref: String,
    /// True when support/export parity is required for every row.
    pub support_export_parity_required: bool,
    /// True when accessibility parity is required for every row.
    pub accessibility_parity_required: bool,
}

/// Constructor input for [`M5MarketplaceHandoffControlsPacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M5MarketplaceHandoffControlsPacketInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable controls label.
    pub controls_label: String,
    /// Controls rows.
    pub controls_rows: Vec<M5MarketplaceHandoffControlsRow>,
    /// Frozen controlled-vocabulary set.
    pub vocabulary_set: M5MarketplaceHandoffVocabularySet,
    /// Governance-review block.
    pub governance_review: M5MarketplaceHandoffGovernanceReview,
    /// Consumer projection block.
    pub consumer_projection: M5MarketplaceHandoffConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: M5MarketplaceHandoffProofFreshness,
    /// Release and support parity posture.
    pub release_posture: M5MarketplaceHandoffReleasePosture,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

/// Export-safe M5 marketplace/account boundary-card and open-in-browser handoff-row controls packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5MarketplaceHandoffControlsPacket {
    /// Record kind; must equal [`M5_MARKETPLACE_HANDOFF_CONTROLS_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`M5_MARKETPLACE_HANDOFF_CONTROLS_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable controls label.
    pub controls_label: String,
    /// Controls rows.
    pub controls_rows: Vec<M5MarketplaceHandoffControlsRow>,
    /// Frozen controlled-vocabulary set.
    pub vocabulary_set: M5MarketplaceHandoffVocabularySet,
    /// Governance-review block.
    pub governance_review: M5MarketplaceHandoffGovernanceReview,
    /// Consumer projection block.
    pub consumer_projection: M5MarketplaceHandoffConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: M5MarketplaceHandoffProofFreshness,
    /// Release and support parity posture.
    pub release_posture: M5MarketplaceHandoffReleasePosture,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

impl M5MarketplaceHandoffControlsPacket {
    /// Builds a controls packet from stable-lane input.
    pub fn new(input: M5MarketplaceHandoffControlsPacketInput) -> Self {
        Self {
            record_kind: M5_MARKETPLACE_HANDOFF_CONTROLS_RECORD_KIND.to_owned(),
            schema_version: M5_MARKETPLACE_HANDOFF_CONTROLS_SCHEMA_VERSION,
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
    pub fn validate(&self) -> Vec<M5MarketplaceHandoffControlsViolation> {
        let mut violations = Vec::new();

        if self.record_kind != M5_MARKETPLACE_HANDOFF_CONTROLS_RECORD_KIND {
            violations.push(M5MarketplaceHandoffControlsViolation::WrongRecordKind);
        }
        if self.schema_version != M5_MARKETPLACE_HANDOFF_CONTROLS_SCHEMA_VERSION {
            violations.push(M5MarketplaceHandoffControlsViolation::WrongSchemaVersion);
        }
        if self.packet_id.trim().is_empty()
            || self.controls_label.trim().is_empty()
            || self.redaction_class_token.trim().is_empty()
            || self.minted_at.trim().is_empty()
        {
            violations.push(M5MarketplaceHandoffControlsViolation::MissingIdentity);
        }

        validate_source_contracts(self, &mut violations);
        if !self.vocabulary_set.matches_canonical() {
            violations.push(M5MarketplaceHandoffControlsViolation::VocabularySetDrift);
        }
        validate_controls_rows(self, &mut violations);
        validate_governance_review(self, &mut violations);
        validate_consumer_projection(self, &mut violations);
        validate_proof_freshness(self, &mut violations);
        validate_release_posture(self, &mut violations);
        validate_acceptance_criteria(self, &mut violations);

        if json_contains_forbidden_material(
            &serde_json::to_value(self).expect("m5 marketplace/handoff controls packet serializes"),
        ) {
            violations.push(M5MarketplaceHandoffControlsViolation::RawMaterialInExport);
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
            .expect("m5 marketplace/handoff controls packet serializes")
    }

    /// Deterministic, machine-readable controls CSV: one row per consumer surface.
    pub fn render_matrix_csv(&self) -> String {
        let mut out = String::new();
        out.push_str(
            "consumer_surface,qualification,owner,card_examples,handoff_row_examples,degrade_reasons,downgrade_triggers\n",
        );
        for row in &self.controls_rows {
            let degrades: Vec<&str> = row
                .marketplace_account_boundary_card_examples
                .iter()
                .filter_map(|ex| ex.degrade_reason.map(|r| r.as_str()))
                .chain(
                    row.open_in_browser_handoff_row_examples
                        .iter()
                        .filter_map(|ex| ex.degrade_reason.map(|r| r.as_str())),
                )
                .collect();
            out.push_str(&format!(
                "{},{},{},{},{},{},{}\n",
                row.consumer_surface.as_str(),
                row.qualification.as_str(),
                csv_field(&row.owner_role),
                row.marketplace_account_boundary_card_examples.len(),
                row.open_in_browser_handoff_row_examples.len(),
                degrades.join("|"),
                join_tokens(&row.downgrade_triggers, |v| v.as_str()),
            ));
        }
        out
    }

    /// Deterministic Markdown report for support, docs, or review handoff.
    pub fn render_markdown_summary(&self) -> String {
        let mut out = String::new();
        out.push_str(
            "# M5 Marketplace/Account Boundary-Card and Open-in-Browser Handoff-Row Controls\n\n",
        );
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- Label: `{}`\n", self.controls_label));
        out.push_str(&format!(
            "- Consumer surfaces: {}\n",
            self.controls_rows.len()
        ));
        out.push_str(&format!(
            "- Account scopes: {}\n",
            self.vocabulary_set.account_scopes.join(", ")
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
                "  - Boundary-card examples: {} / handoff-row examples: {}\n",
                row.marketplace_account_boundary_card_examples.len(),
                row.open_in_browser_handoff_row_examples.len()
            ));
        }
        out
    }
}

/// Errors emitted when reading the checked-in stable controls export.
#[derive(Debug)]
pub enum M5MarketplaceHandoffControlsArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<M5MarketplaceHandoffControlsViolation>),
}

impl fmt::Display for M5MarketplaceHandoffControlsArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(
                    formatter,
                    "m5 marketplace/handoff controls export parse failed: {error}"
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
                    "m5 marketplace/handoff controls export failed validation: {tokens}"
                )
            }
        }
    }
}

impl Error for M5MarketplaceHandoffControlsArtifactError {}

/// Validation failures emitted by [`M5MarketplaceHandoffControlsPacket::validate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum M5MarketplaceHandoffControlsViolation {
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
    /// A controls row carries a dishonest clean example (hidden identity/region/ownership, or a
    /// handoff that drops the object identity or lands on a generic page).
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
    /// AC1 is not proven: clean cards do not cover the owner/origin + account-scope disclosure, no
    /// undisclosed-owner or generic-chrome-concealed card degrades, or a clean card hides its
    /// identity/region/ownership.
    Ac1NotProven,
    /// AC2 is not proven: no handoff row dropped-identity or generic-landing degrades, or a clean
    /// row drops the object identity or lands on a generic page.
    Ac2NotProven,
    /// Export contains raw sensitive material.
    RawMaterialInExport,
}

impl M5MarketplaceHandoffControlsViolation {
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
pub fn current_stable_m5_marketplace_handoff_controls_export(
) -> Result<M5MarketplaceHandoffControlsPacket, M5MarketplaceHandoffControlsArtifactError> {
    let packet: M5MarketplaceHandoffControlsPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/release/m5-marketplace-account-boundary-open-in-browser-handoff-controls-proof/support_export.json"
    )))
    .map_err(M5MarketplaceHandoffControlsArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(M5MarketplaceHandoffControlsArtifactError::Validation(
            violations,
        ))
    }
}

fn validate_source_contracts(
    packet: &M5MarketplaceHandoffControlsPacket,
    violations: &mut Vec<M5MarketplaceHandoffControlsViolation>,
) {
    let refs: BTreeSet<&str> = packet
        .source_contract_refs
        .iter()
        .map(String::as_str)
        .collect();
    for required in [
        M5_MARKETPLACE_HANDOFF_CONTROLS_SCHEMA_REF,
        M5_MARKETPLACE_HANDOFF_CONTROLS_DOC_REF,
        M5_EMBEDDED_BOUNDARY_COMPONENT_SCHEMA_REF,
        M5_EMBEDDED_BOUNDARY_COMPONENT_DOC_REF,
        M5_MARKETPLACE_ACCOUNT_BOUNDARY_CARD_SCHEMA_REF,
        M5_OPEN_IN_BROWSER_HANDOFF_ROW_SCHEMA_REF,
    ] {
        if !refs.contains(required) {
            violations.push(M5MarketplaceHandoffControlsViolation::MissingSourceContracts);
            return;
        }
    }
}

fn validate_controls_rows(
    packet: &M5MarketplaceHandoffControlsPacket,
    violations: &mut Vec<M5MarketplaceHandoffControlsViolation>,
) {
    if packet.controls_rows.is_empty() {
        violations.push(M5MarketplaceHandoffControlsViolation::NoControlsRows);
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
            violations.push(M5MarketplaceHandoffControlsViolation::ControlsRowIncomplete);
        }
        if !row.declares_mandatory_anatomy() {
            violations.push(M5MarketplaceHandoffControlsViolation::MandatoryAnatomyMissing);
        }
        if !row.declares_mandatory_export_fields() {
            violations.push(M5MarketplaceHandoffControlsViolation::MandatoryExportFieldMissing);
        }
        let refs: BTreeSet<&str> = row
            .source_contract_refs
            .iter()
            .map(String::as_str)
            .collect();
        if !refs.contains(M5_MARKETPLACE_ACCOUNT_BOUNDARY_CARD_SCHEMA_REF)
            || !refs.contains(M5_OPEN_IN_BROWSER_HANDOFF_ROW_SCHEMA_REF)
        {
            violations.push(M5MarketplaceHandoffControlsViolation::ComponentSchemaRefMissing);
        }
        if row.marketplace_account_boundary_card_examples.is_empty()
            || row.open_in_browser_handoff_row_examples.is_empty()
        {
            violations.push(M5MarketplaceHandoffControlsViolation::ExamplesMissing);
        }
        if !row.examples_are_honest() {
            violations.push(M5MarketplaceHandoffControlsViolation::DishonestExample);
        }
        if !row.honours_invariants() {
            violations.push(M5MarketplaceHandoffControlsViolation::RowInvariantViolated);
        }
    }
}

fn validate_governance_review(
    packet: &M5MarketplaceHandoffControlsPacket,
    violations: &mut Vec<M5MarketplaceHandoffControlsViolation>,
) {
    let review = &packet.governance_review;
    for ok in [
        review.card_names_owner_and_ownership,
        review.card_discloses_account_scope,
        review.profile_and_region_always_explicit,
        review.network_state_and_fallback_always_exposed,
        review.generic_chrome_never_conceals_identity,
        review.handoff_row_preserves_object_identity,
        review.handoff_row_states_reason,
        review.handoff_never_lands_on_generic_page,
        review.local_continuity_always_explicit,
        review.reuses_frozen_matrix_vocabulary,
    ] {
        if !ok {
            violations.push(M5MarketplaceHandoffControlsViolation::GovernanceReviewIncomplete);
            return;
        }
    }
}

fn validate_consumer_projection(
    packet: &M5MarketplaceHandoffControlsPacket,
    violations: &mut Vec<M5MarketplaceHandoffControlsViolation>,
) {
    let projection = &packet.consumer_projection;
    for ok in [
        projection.surfaces_consume_owner_origin_vocabulary,
        projection.surfaces_consume_account_scope_vocabulary,
        projection.handoff_rows_consume_shared_fallback_vocabulary,
        projection.support_export_reads_single_boundary_source,
    ] {
        if !ok {
            violations.push(M5MarketplaceHandoffControlsViolation::ConsumerProjectionIncomplete);
            return;
        }
    }
}

fn validate_proof_freshness(
    packet: &M5MarketplaceHandoffControlsPacket,
    violations: &mut Vec<M5MarketplaceHandoffControlsViolation>,
) {
    if packet.proof_freshness.proof_freshness_slo_hours == 0
        || packet.proof_freshness.last_proof_refresh.trim().is_empty()
    {
        violations.push(M5MarketplaceHandoffControlsViolation::ProofFreshnessIncomplete);
    }
}

fn validate_release_posture(
    packet: &M5MarketplaceHandoffControlsPacket,
    violations: &mut Vec<M5MarketplaceHandoffControlsViolation>,
) {
    let posture = &packet.release_posture;
    if posture.proof_packet_ref.trim().is_empty()
        || posture.boundary_audit_ref.trim().is_empty()
        || !posture.support_export_parity_required
        || !posture.accessibility_parity_required
    {
        violations.push(M5MarketplaceHandoffControlsViolation::ReleasePostureIncomplete);
    }
}

/// Proves the two acceptance criteria are exercised by the packet's resolved examples, not merely
/// asserted by governance bools.
fn validate_acceptance_criteria(
    packet: &M5MarketplaceHandoffControlsPacket,
    violations: &mut Vec<M5MarketplaceHandoffControlsViolation>,
) {
    let card_examples = || {
        packet
            .controls_rows
            .iter()
            .flat_map(|row| row.marketplace_account_boundary_card_examples.iter())
    };
    let row_examples = || {
        packet
            .controls_rows
            .iter()
            .flat_map(|row| row.open_in_browser_handoff_row_examples.iter())
    };

    // AC1: a marketplace/account pane never hides identity, region, or service ownership behind
    // generic product chrome — a clean card discloses owner/origin and account scope, an
    // undisclosed-owner card degrades, a generic-chrome-concealed card degrades, and no clean card
    // hides its identity/region/ownership.
    let covers_owner_and_scope = card_examples()
        .any(|ex| ex.is_clean() && ex.owner_origin_disclosed && ex.account_scope_disclosed);
    let owner_unstated_degrades = card_examples()
        .any(|ex| ex.degrade_reason == Some(M5MarketplaceCardDegradeReason::OwnerOrOriginUnstated));
    let generic_chrome_degrades = card_examples().any(|ex| {
        ex.degrade_reason == Some(M5MarketplaceCardDegradeReason::GenericChromeConcealsIdentity)
    });
    let no_clean_card_hides =
        card_examples().all(|ex| !(ex.is_clean() && ex.hides_identity_region_or_ownership()));
    if !(covers_owner_and_scope
        && owner_unstated_degrades
        && generic_chrome_degrades
        && no_clean_card_hides)
    {
        violations.push(M5MarketplaceHandoffControlsViolation::Ac1NotProven);
    }

    // AC2: a browser handoff preserves object identity and reason-for-handoff instead of dropping
    // users onto a generic landing page — a clean row preserves identity + reason + continuity, a
    // dropped-identity row degrades, a generic-landing row degrades, and no clean row drops the
    // identity or lands on a generic page.
    let covers_identity_and_reason = row_examples().any(|ex| {
        ex.is_clean()
            && ex.object_identity_preserved
            && ex.handoff_reason_stated
            && ex.local_continuity_explicit
    });
    let identity_dropped_degrades = row_examples().any(|ex| {
        ex.degrade_reason == Some(M5OpenInBrowserRowDegradeReason::ObjectIdentityDropped)
    });
    let generic_landing_degrades = row_examples()
        .any(|ex| ex.degrade_reason == Some(M5OpenInBrowserRowDegradeReason::LandsOnGenericPage));
    let no_clean_row_drops =
        row_examples().all(|ex| !(ex.is_clean() && ex.drops_identity_or_lands_generic()));
    if !(covers_identity_and_reason
        && identity_dropped_degrades
        && generic_landing_degrades
        && no_clean_row_drops)
    {
        violations.push(M5MarketplaceHandoffControlsViolation::Ac2NotProven);
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
