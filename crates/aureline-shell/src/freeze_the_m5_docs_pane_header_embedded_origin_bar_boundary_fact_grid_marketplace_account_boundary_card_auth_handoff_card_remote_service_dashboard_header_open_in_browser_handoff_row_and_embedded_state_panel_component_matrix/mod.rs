//! Frozen M5 docs-pane-header, embedded-origin-bar, boundary-fact-grid,
//! marketplace-account-boundary-card, auth-handoff-card, remote-service-dashboard-header,
//! open-in-browser-handoff-row, and embedded-state-panel component matrix.
//!
//! This module locks Aureline's reusable embedded / browser-handoff boundary UI components into
//! one export-safe packet. Every embedded or browser-handoff surface M5 claims that still ships
//! its own owner/origin chrome — the documentation-pane header, the embedded-origin bar, the
//! boundary-fact grid, the marketplace/account boundary card, the auth-handoff card, the
//! remote/service dashboard header, the open-in-browser handoff row, and the embedded-state
//! panel — is named once here and constrained by the same owner/origin, data-boundary,
//! source/version/freshness, network/offline, browser-fallback, account-scope, and
//! capability-limit vocabulary regardless of the surface family that renders it.
//!
//! The matrix does not re-architect the browser companion, the auth providers, or the service
//! dashboards that already own those records — it is the shared boundary-honesty component
//! contract layered on top of them. It binds directly to the frozen [M5 auth-boundary object
//! model][auth] so no later consumer can fork its own owner/origin or browser-handoff wording:
//! the origin-bearing components reuse the [`WebviewOwnerClass`] owner vocabulary, the
//! handoff-bearing components reuse the [`BrowserHandoffKind`] browser-handoff vocabulary, the
//! embedded surfaces reuse the [`CapabilityLimitClass`] capability-limit vocabulary, and every
//! data-crossing component reuses the [`DataExitBoundary`] data-boundary vocabulary.
//!
//! The controlled vocabularies are frozen in one self-describing
//! [`M5EmbeddedBoundaryVocabularySet`] rather than minted per surface. The single controlled
//! boundary-disposition vocabulary consumers bind to — live-first-party-local,
//! live-first-party-hosted, live-provider-owned, stale-snapshot, offline-snapshot,
//! provider-blocked, browser-handoff-only, capability-limited, and not-evaluated — keeps a
//! stale, offline, or provider-blocked pane from ever reading as fresh first-party local truth,
//! keeps owner/origin and browser fallback from hiding behind menus only, and keeps embedded
//! surfaces from imitating native permission or irreversible-approval chrome. Raw secret values
//! and private endpoints stay outside the export boundary.
//!
//! [auth]: crate::m5_auth_boundaries

mod seed;
#[cfg(test)]
mod tests;

pub use seed::{
    seeded_m5_embedded_boundary_component_matrix,
    seeded_m5_embedded_boundary_component_matrix_docs_pane_header_beta_narrowed,
    seeded_m5_embedded_boundary_component_matrix_embedded_state_panel_preview_narrowed,
    M5_EMBEDDED_BOUNDARY_COMPONENT_MATRIX_PACKET_ID,
};

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

use crate::m5_auth_boundaries::{
    BrowserHandoffKind, CapabilityLimitClass, DataExitBoundary, WebviewOwnerClass,
    M5_AUTH_BOUNDARY_CONTRACT_DOC_REF, M5_BROWSER_HANDOFF_CARD_SCHEMA_REF,
    M5_WEBVIEW_ORIGIN_BAR_SCHEMA_REF,
};

/// Stable record-kind tag carried by [`M5EmbeddedBoundaryComponentMatrixPacket`].
pub const M5_EMBEDDED_BOUNDARY_COMPONENT_MATRIX_RECORD_KIND: &str =
    "freeze_m5_docs_pane_header_embedded_origin_bar_boundary_fact_grid_marketplace_account_boundary_card_auth_handoff_card_remote_service_dashboard_header_open_in_browser_handoff_row_and_embedded_state_panel_component_matrix";

/// Schema version for M5 embedded-boundary component-matrix records.
pub const M5_EMBEDDED_BOUNDARY_COMPONENT_MATRIX_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the combined embedded-boundary component-matrix schema.
pub const M5_EMBEDDED_BOUNDARY_COMPONENT_SCHEMA_REF: &str =
    "schemas/ui/m5-embedded-boundary-component-matrix.schema.json";

/// Repo-relative path of the contract doc.
pub const M5_EMBEDDED_BOUNDARY_COMPONENT_DOC_REF: &str =
    "docs/help/m5_embedded_boundary_components_contract.md";

/// Repo-relative path of the docs-pane-header canonical component schema.
pub const M5_DOCS_PANE_HEADER_SCHEMA_REF: &str = "schemas/ui/m5-docs-pane-header.schema.json";

/// Repo-relative path of the embedded-origin-bar canonical component schema.
pub const M5_EMBEDDED_ORIGIN_BAR_SCHEMA_REF: &str = "schemas/ui/m5-embedded-origin-bar.schema.json";

/// Repo-relative path of the boundary-fact-grid canonical component schema.
pub const M5_BOUNDARY_FACT_GRID_SCHEMA_REF: &str = "schemas/ui/m5-boundary-fact-grid.schema.json";

/// Repo-relative path of the marketplace-account-boundary-card canonical component schema.
pub const M5_MARKETPLACE_ACCOUNT_BOUNDARY_CARD_SCHEMA_REF: &str =
    "schemas/ui/m5-marketplace-account-boundary-card.schema.json";

/// Repo-relative path of the auth-handoff-card canonical component schema.
pub const M5_AUTH_HANDOFF_CARD_SCHEMA_REF: &str = "schemas/ui/m5-auth-handoff-card.schema.json";

/// Repo-relative path of the remote-service-dashboard-header canonical component schema.
pub const M5_REMOTE_SERVICE_DASHBOARD_HEADER_SCHEMA_REF: &str =
    "schemas/ui/m5-remote-service-dashboard-header.schema.json";

/// Repo-relative path of the open-in-browser-handoff-row canonical component schema.
pub const M5_OPEN_IN_BROWSER_HANDOFF_ROW_SCHEMA_REF: &str =
    "schemas/ui/m5-open-in-browser-handoff-row.schema.json";

/// Repo-relative path of the embedded-state-panel canonical component schema.
pub const M5_EMBEDDED_STATE_PANEL_SCHEMA_REF: &str =
    "schemas/ui/m5-embedded-state-panel.schema.json";

/// Repo-relative path of the protected fixture directory.
pub const M5_EMBEDDED_BOUNDARY_COMPONENT_FIXTURE_DIR: &str =
    "fixtures/ui/m5-embedded-boundary-components";

/// Repo-relative path of the checked support-export artifact.
pub const M5_EMBEDDED_BOUNDARY_COMPONENT_ARTIFACT_REF: &str =
    "artifacts/release/m5-embedded-boundary-proof/support_export.json";

/// Repo-relative path of the checked machine-readable matrix CSV.
pub const M5_EMBEDDED_BOUNDARY_COMPONENT_CSV_REF: &str =
    "artifacts/release/m5-embedded-boundary-proof/matrix.csv";

/// Repo-relative path of the checked Markdown design report.
pub const M5_EMBEDDED_BOUNDARY_COMPONENT_REPORT_REF: &str =
    "artifacts/design/m5-embedded-boundary-component-matrix.md";

/// One of the eight governed embedded / browser-handoff component families this matrix freezes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5EmbeddedBoundaryComponentFamily {
    /// A documentation-pane header naming the owner/origin and freshness of the docs it renders.
    DocsPaneHeader,
    /// An embedded-origin bar naming who owns the embedded content and its capability limits.
    EmbeddedOriginBar,
    /// A boundary-fact grid naming owner/origin, data boundary, and freshness in one place.
    BoundaryFactGrid,
    /// A marketplace/account boundary card naming account scope and the data boundary.
    MarketplaceAccountBoundaryCard,
    /// An auth-handoff card naming the browser fallback and the data boundary of a sign-in.
    AuthHandoffCard,
    /// A remote/service dashboard header naming provider health and freshness.
    RemoteServiceDashboardHeader,
    /// An open-in-browser handoff row naming the browser fallback for a surface.
    OpenInBrowserHandoffRow,
    /// An embedded-state panel naming a stale, offline, or provider-blocked state explicitly.
    EmbeddedStatePanel,
}

impl M5EmbeddedBoundaryComponentFamily {
    /// Every governed component family, in declaration order.
    pub const ALL: [Self; 8] = [
        Self::DocsPaneHeader,
        Self::EmbeddedOriginBar,
        Self::BoundaryFactGrid,
        Self::MarketplaceAccountBoundaryCard,
        Self::AuthHandoffCard,
        Self::RemoteServiceDashboardHeader,
        Self::OpenInBrowserHandoffRow,
        Self::EmbeddedStatePanel,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::DocsPaneHeader => "docs_pane_header",
            Self::EmbeddedOriginBar => "embedded_origin_bar",
            Self::BoundaryFactGrid => "boundary_fact_grid",
            Self::MarketplaceAccountBoundaryCard => "marketplace_account_boundary_card",
            Self::AuthHandoffCard => "auth_handoff_card",
            Self::RemoteServiceDashboardHeader => "remote_service_dashboard_header",
            Self::OpenInBrowserHandoffRow => "open_in_browser_handoff_row",
            Self::EmbeddedStatePanel => "embedded_state_panel",
        }
    }

    /// The canonical per-component schema ref a downstream row points at instead of restating
    /// this component's boundary truth by hand.
    pub const fn canonical_component_schema_ref(self) -> &'static str {
        match self {
            Self::DocsPaneHeader => M5_DOCS_PANE_HEADER_SCHEMA_REF,
            Self::EmbeddedOriginBar => M5_EMBEDDED_ORIGIN_BAR_SCHEMA_REF,
            Self::BoundaryFactGrid => M5_BOUNDARY_FACT_GRID_SCHEMA_REF,
            Self::MarketplaceAccountBoundaryCard => M5_MARKETPLACE_ACCOUNT_BOUNDARY_CARD_SCHEMA_REF,
            Self::AuthHandoffCard => M5_AUTH_HANDOFF_CARD_SCHEMA_REF,
            Self::RemoteServiceDashboardHeader => M5_REMOTE_SERVICE_DASHBOARD_HEADER_SCHEMA_REF,
            Self::OpenInBrowserHandoffRow => M5_OPEN_IN_BROWSER_HANDOFF_ROW_SCHEMA_REF,
            Self::EmbeddedStatePanel => M5_EMBEDDED_STATE_PANEL_SCHEMA_REF,
        }
    }

    /// `true` when this family must name a controlled owner/origin class.
    pub const fn declares_owner_class(self) -> bool {
        matches!(
            self,
            Self::DocsPaneHeader
                | Self::EmbeddedOriginBar
                | Self::BoundaryFactGrid
                | Self::MarketplaceAccountBoundaryCard
                | Self::RemoteServiceDashboardHeader
                | Self::EmbeddedStatePanel
        )
    }

    /// `true` when this family must name a controlled data-exit boundary.
    pub const fn declares_data_boundary(self) -> bool {
        matches!(
            self,
            Self::BoundaryFactGrid
                | Self::MarketplaceAccountBoundaryCard
                | Self::AuthHandoffCard
                | Self::RemoteServiceDashboardHeader
                | Self::OpenInBrowserHandoffRow
        )
    }

    /// `true` when this family must name a controlled browser-handoff kind.
    pub const fn declares_browser_handoff(self) -> bool {
        matches!(self, Self::AuthHandoffCard | Self::OpenInBrowserHandoffRow)
    }

    /// `true` when this family must name a controlled capability limit.
    pub const fn declares_capability_limits(self) -> bool {
        matches!(
            self,
            Self::DocsPaneHeader
                | Self::EmbeddedOriginBar
                | Self::RemoteServiceDashboardHeader
                | Self::EmbeddedStatePanel
        )
    }

    /// `true` when this family must name a controlled freshness state.
    pub const fn declares_freshness(self) -> bool {
        matches!(
            self,
            Self::DocsPaneHeader
                | Self::BoundaryFactGrid
                | Self::RemoteServiceDashboardHeader
                | Self::EmbeddedStatePanel
        )
    }

    /// `true` when this family must name a controlled account scope.
    pub const fn declares_account_scope(self) -> bool {
        matches!(
            self,
            Self::MarketplaceAccountBoundaryCard | Self::AuthHandoffCard
        )
    }
}

/// The single controlled boundary-disposition vocabulary every embedded / browser-handoff
/// consumer binds to. These are the exact acceptance-criteria tokens that keep a stale, offline,
/// or provider-blocked pane from reading as fresh first-party local truth. No embedded surface
/// invents a parallel word for any of these dispositions.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5EmbeddedBoundaryDisposition {
    /// Live, first-party, local truth rendered by Aureline itself.
    LiveFirstPartyLocal,
    /// Live first-party content served from a first-party hosted surface.
    LiveFirstPartyHosted,
    /// Live content owned by a connected provider, labelled as provider-owned.
    LiveProviderOwned,
    /// A stale snapshot deliberately kept visible and labelled as stale.
    StaleSnapshot,
    /// An offline snapshot with no live refresh path, labelled as offline.
    OfflineSnapshot,
    /// Content blocked by a provider or a policy, labelled as blocked.
    ProviderBlocked,
    /// The surface can only be reached by handing off to the browser.
    BrowserHandoffOnly,
    /// An embedded surface with reduced capability relative to native trusted chrome.
    CapabilityLimited,
    /// The disposition cannot currently be evaluated.
    NotEvaluated,
}

impl M5EmbeddedBoundaryDisposition {
    /// Every disposition token, in declaration order.
    pub const ALL: [Self; 9] = [
        Self::LiveFirstPartyLocal,
        Self::LiveFirstPartyHosted,
        Self::LiveProviderOwned,
        Self::StaleSnapshot,
        Self::OfflineSnapshot,
        Self::ProviderBlocked,
        Self::BrowserHandoffOnly,
        Self::CapabilityLimited,
        Self::NotEvaluated,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LiveFirstPartyLocal => "live_first_party_local",
            Self::LiveFirstPartyHosted => "live_first_party_hosted",
            Self::LiveProviderOwned => "live_provider_owned",
            Self::StaleSnapshot => "stale_snapshot",
            Self::OfflineSnapshot => "offline_snapshot",
            Self::ProviderBlocked => "provider_blocked",
            Self::BrowserHandoffOnly => "browser_handoff_only",
            Self::CapabilityLimited => "capability_limited",
            Self::NotEvaluated => "not_evaluated",
        }
    }

    /// Whether this disposition is the one clean fresh-first-party-local truth state.
    pub const fn is_fresh_first_party_local(self) -> bool {
        matches!(self, Self::LiveFirstPartyLocal)
    }
}

/// Controlled freshness state — whether an embedded surface's content is live, warm, stale,
/// offline, or of unknown freshness, so source/version/last-updated truth is never left implicit
/// and a stale pane never reads as fresh.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5EmbeddedFreshnessState {
    /// Live and current within the freshness grace window.
    LiveFresh,
    /// A warm snapshot within the freshness grace window.
    WarmSnapshot,
    /// A stale snapshot beyond the grace window, labelled as stale.
    StaleSnapshot,
    /// An offline snapshot with no refresh path, labelled as offline.
    OfflineSnapshot,
    /// Freshness cannot currently be determined.
    FreshnessUnknown,
}

impl M5EmbeddedFreshnessState {
    /// Every freshness state, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::LiveFresh,
        Self::WarmSnapshot,
        Self::StaleSnapshot,
        Self::OfflineSnapshot,
        Self::FreshnessUnknown,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LiveFresh => "live_fresh",
            Self::WarmSnapshot => "warm_snapshot",
            Self::StaleSnapshot => "stale_snapshot",
            Self::OfflineSnapshot => "offline_snapshot",
            Self::FreshnessUnknown => "freshness_unknown",
        }
    }
}

/// Controlled account-scope class — whose account the marketplace/account or auth-handoff surface
/// is scoped to, so account scope is never left implicit next to hosted or provider content.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5EmbeddedAccountScope {
    /// No account; local-only usage.
    NoAccountLocal,
    /// A personal account.
    PersonalAccount,
    /// An organization workspace.
    OrgWorkspace,
    /// A managed / enterprise-administered tenant.
    ManagedTenant,
    /// Account scope cannot currently be resolved.
    AccountScopeUnknown,
}

impl M5EmbeddedAccountScope {
    /// Every account scope, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::NoAccountLocal,
        Self::PersonalAccount,
        Self::OrgWorkspace,
        Self::ManagedTenant,
        Self::AccountScopeUnknown,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NoAccountLocal => "no_account_local",
            Self::PersonalAccount => "personal_account",
            Self::OrgWorkspace => "org_workspace",
            Self::ManagedTenant => "managed_tenant",
            Self::AccountScopeUnknown => "account_scope_unknown",
        }
    }
}

/// Claimed M5 surface family that renders / consumes an embedded-boundary component. No component
/// may invent a parallel surface taxonomy.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5EmbeddedSurfaceFamily {
    /// The documentation / Help pane.
    DocsHelpPane,
    /// The marketplace or account surface.
    MarketplaceOrAccount,
    /// The remote / service dashboard surface.
    RemoteServiceDashboard,
    /// An embedded webview surface.
    EmbeddedWebview,
    /// A browser / device-code auth-handoff surface.
    AuthHandoff,
    /// The support export.
    SupportExport,
}

impl M5EmbeddedSurfaceFamily {
    /// Every surface family, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::DocsHelpPane,
        Self::MarketplaceOrAccount,
        Self::RemoteServiceDashboard,
        Self::EmbeddedWebview,
        Self::AuthHandoff,
        Self::SupportExport,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::DocsHelpPane => "docs_help_pane",
            Self::MarketplaceOrAccount => "marketplace_or_account",
            Self::RemoteServiceDashboard => "remote_service_dashboard",
            Self::EmbeddedWebview => "embedded_webview",
            Self::AuthHandoff => "auth_handoff",
            Self::SupportExport => "support_export",
        }
    }
}

/// Deployment line a component must survive with the same truth, so a component's owner/origin,
/// data-boundary, freshness, or fallback truth never silently narrows or widens between
/// deployment shapes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5EmbeddedDeploymentLine {
    /// The local open-source line.
    LocalOss,
    /// The self-hosted line.
    SelfHosted,
    /// The managed line.
    Managed,
    /// The air-gapped line.
    AirGapped,
    /// The mirror / offline line.
    MirrorOffline,
}

impl M5EmbeddedDeploymentLine {
    /// Every deployment line, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::LocalOss,
        Self::SelfHosted,
        Self::Managed,
        Self::AirGapped,
        Self::MirrorOffline,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LocalOss => "local_oss",
            Self::SelfHosted => "self_hosted",
            Self::Managed => "managed",
            Self::AirGapped => "air_gapped",
            Self::MirrorOffline => "mirror_offline",
        }
    }
}

/// Subsystem that consumes a component's projection.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5EmbeddedConsumerSurface {
    /// The docs / help browser UI.
    DocsBrowserUi,
    /// The marketplace UI.
    MarketplaceUi,
    /// The account UI.
    AccountUi,
    /// The remote / service dashboard UI.
    RemoteDashboardUi,
    /// The embedded webview UI.
    EmbeddedWebviewUi,
    /// The auth-handoff UI.
    AuthHandoffUi,
    /// The support export.
    SupportExport,
    /// The general product UI.
    ProductUi,
}

impl M5EmbeddedConsumerSurface {
    /// Every consumer surface, in declaration order.
    pub const ALL: [Self; 8] = [
        Self::DocsBrowserUi,
        Self::MarketplaceUi,
        Self::AccountUi,
        Self::RemoteDashboardUi,
        Self::EmbeddedWebviewUi,
        Self::AuthHandoffUi,
        Self::SupportExport,
        Self::ProductUi,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::DocsBrowserUi => "docs_browser_ui",
            Self::MarketplaceUi => "marketplace_ui",
            Self::AccountUi => "account_ui",
            Self::RemoteDashboardUi => "remote_dashboard_ui",
            Self::EmbeddedWebviewUi => "embedded_webview_ui",
            Self::AuthHandoffUi => "auth_handoff_ui",
            Self::SupportExport => "support_export",
            Self::ProductUi => "product_ui",
        }
    }
}

/// Non-visual / accessibility route every component must offer so no boundary truth is
/// hover-only, pointer-only, menu-only, or visually encoded alone.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5EmbeddedAccessibilityRoute {
    /// Reachable and operable by keyboard focus.
    KeyboardFocusable,
    /// Announced to a screen reader.
    ScreenReaderAnnounced,
    /// Reachable without pointer hover.
    NonHoverReachable,
    /// Pointer interaction is optional, never required.
    PointerOptional,
    /// Legible in high-contrast / reduced-motion modes.
    HighContrastSafe,
    /// Present in the support / export packet, never menu-only.
    SupportExportable,
}

impl M5EmbeddedAccessibilityRoute {
    /// Every accessibility route, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::KeyboardFocusable,
        Self::ScreenReaderAnnounced,
        Self::NonHoverReachable,
        Self::PointerOptional,
        Self::HighContrastSafe,
        Self::SupportExportable,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::KeyboardFocusable => "keyboard_focusable",
            Self::ScreenReaderAnnounced => "screen_reader_announced",
            Self::NonHoverReachable => "non_hover_reachable",
            Self::PointerOptional => "pointer_optional",
            Self::HighContrastSafe => "high_contrast_safe",
            Self::SupportExportable => "support_exportable",
        }
    }
}

/// Reason an embedded-boundary component has degraded below its qualified state. Required on
/// every row so a stale, unresolved, or narrowed fallback is never left implicit.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5EmbeddedDegradedReason {
    /// Proof has gone stale.
    ProofStale,
    /// Origin verification is unavailable.
    OriginVerificationUnavailable,
    /// Provider health is unknown.
    ProviderHealthUnknown,
    /// The freshness / last-updated signal is unavailable.
    FreshnessSignalUnavailable,
    /// The browser fallback is unavailable.
    BrowserFallbackUnavailable,
    /// An upstream boundary lane narrowed.
    UpstreamBoundaryNarrowed,
}

impl M5EmbeddedDegradedReason {
    /// Every degraded reason, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::ProofStale,
        Self::OriginVerificationUnavailable,
        Self::ProviderHealthUnknown,
        Self::FreshnessSignalUnavailable,
        Self::BrowserFallbackUnavailable,
        Self::UpstreamBoundaryNarrowed,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ProofStale => "proof_stale",
            Self::OriginVerificationUnavailable => "origin_verification_unavailable",
            Self::ProviderHealthUnknown => "provider_health_unknown",
            Self::FreshnessSignalUnavailable => "freshness_signal_unavailable",
            Self::BrowserFallbackUnavailable => "browser_fallback_unavailable",
            Self::UpstreamBoundaryNarrowed => "upstream_boundary_narrowed",
        }
    }
}

/// Mandatory label a claimed embedded-boundary component must be able to show. The first three
/// are hard requirements on every component; the remaining three close the acceptance-criteria
/// ambiguity about owner/origin, data boundary and browser fallback, and freshness plus
/// capability limits.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5EmbeddedRequiredLabel {
    /// The component's stable identity / what it represents.
    Identity,
    /// The component's current typed state / disposition.
    State,
    /// The non-visual keyboard route to the component.
    KeyboardRoute,
    /// The owner and origin behind the component.
    OwnerAndOrigin,
    /// The data boundary and browser fallback behind the component.
    DataBoundaryAndFallback,
    /// The freshness and capability limits behind the component.
    FreshnessAndCapabilityLimits,
}

impl M5EmbeddedRequiredLabel {
    /// Every declared label, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::Identity,
        Self::State,
        Self::KeyboardRoute,
        Self::OwnerAndOrigin,
        Self::DataBoundaryAndFallback,
        Self::FreshnessAndCapabilityLimits,
    ];

    /// The three labels every claimed component must be able to show.
    pub const MANDATORY: [Self; 3] = [Self::Identity, Self::State, Self::KeyboardRoute];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Identity => "identity",
            Self::State => "state",
            Self::KeyboardRoute => "keyboard_route",
            Self::OwnerAndOrigin => "owner_and_origin",
            Self::DataBoundaryAndFallback => "data_boundary_and_fallback",
            Self::FreshnessAndCapabilityLimits => "freshness_and_capability_limits",
        }
    }
}

/// Qualification class for an M5 embedded-boundary component row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5EmbeddedQualificationClass {
    /// Component qualifies for the Stable claim.
    Stable,
    /// Component is narrowed to Beta.
    Beta,
    /// Component is narrowed to Preview.
    Preview,
    /// Component is experimental and not claimed.
    Experimental,
    /// Component is unavailable on this build.
    Unavailable,
    /// Component is held pending upstream resolution.
    Held,
}

impl M5EmbeddedQualificationClass {
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

    /// Whether the component may carry a public Stable claim.
    pub const fn is_stable(self) -> bool {
        matches!(self, Self::Stable)
    }
}

/// Downgrade trigger that narrows an embedded-boundary component below its claim.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5EmbeddedDowngradeTrigger {
    /// A component left its owner or origin unstated.
    OwnerOrOriginUnstated,
    /// A component left its data boundary unstated.
    DataBoundaryUnstated,
    /// A component hid its browser fallback behind menus only.
    BrowserFallbackHiddenInMenusOnly,
    /// A component left its capability limits unstated.
    CapabilityLimitsUnstated,
    /// A component rendered a stale or blocked pane as fresh first-party truth.
    StaleOrBlockedShownAsFresh,
    /// A component left its account scope unstated.
    AccountScopeUnstated,
    /// A component left its freshness / last-updated unstated.
    FreshnessOrLastUpdatedUnstated,
    /// A component left provider health unstated.
    ProviderHealthUnstated,
    /// An embedded surface imitated native permission or approval chrome.
    ImitatesNativeApprovalChrome,
    /// A high-risk approval was embedded without a native step-up.
    HighRiskApprovalEmbedded,
    /// Generic chrome wording concealed owner, origin, or boundary truth.
    GenericChromeWordingUsed,
    /// The proof packet has gone stale.
    ProofStale,
}

impl M5EmbeddedDowngradeTrigger {
    /// Every trigger, in declaration order.
    pub const ALL: [Self; 12] = [
        Self::OwnerOrOriginUnstated,
        Self::DataBoundaryUnstated,
        Self::BrowserFallbackHiddenInMenusOnly,
        Self::CapabilityLimitsUnstated,
        Self::StaleOrBlockedShownAsFresh,
        Self::AccountScopeUnstated,
        Self::FreshnessOrLastUpdatedUnstated,
        Self::ProviderHealthUnstated,
        Self::ImitatesNativeApprovalChrome,
        Self::HighRiskApprovalEmbedded,
        Self::GenericChromeWordingUsed,
        Self::ProofStale,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::OwnerOrOriginUnstated => "owner_or_origin_unstated",
            Self::DataBoundaryUnstated => "data_boundary_unstated",
            Self::BrowserFallbackHiddenInMenusOnly => "browser_fallback_hidden_in_menus_only",
            Self::CapabilityLimitsUnstated => "capability_limits_unstated",
            Self::StaleOrBlockedShownAsFresh => "stale_or_blocked_shown_as_fresh",
            Self::AccountScopeUnstated => "account_scope_unstated",
            Self::FreshnessOrLastUpdatedUnstated => "freshness_or_last_updated_unstated",
            Self::ProviderHealthUnstated => "provider_health_unstated",
            Self::ImitatesNativeApprovalChrome => "imitates_native_approval_chrome",
            Self::HighRiskApprovalEmbedded => "high_risk_approval_embedded",
            Self::GenericChromeWordingUsed => "generic_chrome_wording_used",
            Self::ProofStale => "proof_stale",
        }
    }
}

/// One row in the matrix: one governed embedded-boundary component family bound to the surface-
/// specific truth it must project.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5EmbeddedBoundaryComponentRow {
    /// Governed component family.
    pub component_family: M5EmbeddedBoundaryComponentFamily,
    /// Qualification class earned by this component.
    pub qualification: M5EmbeddedQualificationClass,
    /// Owner role accountable for keeping this component governed.
    pub owner_role: String,
    /// Human-readable scope summary.
    pub scope_summary: String,
    /// Claimed M5 surface families that render / consume this component.
    pub surface_families: Vec<M5EmbeddedSurfaceFamily>,
    /// Deployment lines this component keeps the same truth across.
    pub deployment_lines: Vec<M5EmbeddedDeploymentLine>,
    /// Mandatory labels this component must be able to show (must include the three
    /// [`M5EmbeddedRequiredLabel::MANDATORY`] labels).
    pub required_labels: Vec<M5EmbeddedRequiredLabel>,
    /// Boundary dispositions this component can carry (the frozen AC vocabulary; required on
    /// every component).
    pub boundary_dispositions: Vec<M5EmbeddedBoundaryDisposition>,
    /// Owner/origin classes this component names (origin-bearing families only). Bound from the
    /// M5 auth-boundary object model.
    pub owner_classes: Vec<WebviewOwnerClass>,
    /// Data-exit boundaries this component names (data-crossing families only). Bound from the
    /// public-truth / auth-boundary object model.
    pub data_exit_boundaries: Vec<DataExitBoundary>,
    /// Browser-handoff kinds this component names (handoff families only). Bound from the M5
    /// auth-boundary object model.
    pub browser_handoff_kinds: Vec<BrowserHandoffKind>,
    /// Capability limits this component names (embedded families only). Bound from the M5
    /// auth-boundary object model.
    pub capability_limits: Vec<CapabilityLimitClass>,
    /// Freshness states this component names (freshness-bearing families only).
    pub freshness_states: Vec<M5EmbeddedFreshnessState>,
    /// Account scopes this component names (account families only).
    pub account_scopes: Vec<M5EmbeddedAccountScope>,
    /// Degraded reasons this component can name (required on every component).
    pub degraded_reasons: Vec<M5EmbeddedDegradedReason>,
    /// Non-visual accessibility routes this component offers.
    pub accessibility_routes: Vec<M5EmbeddedAccessibilityRoute>,
    /// Subsystems that consume this component's projection.
    pub consumer_surfaces: Vec<M5EmbeddedConsumerSurface>,
    /// Downgrade triggers that apply to this component.
    pub downgrade_triggers: Vec<M5EmbeddedDowngradeTrigger>,
    /// Proof packet refs that keep this component current.
    pub required_proof_packet_refs: Vec<String>,
    /// Source contract refs consumed by this component (must include its own canonical component
    /// schema so downstream rows have one target to point at).
    pub source_contract_refs: Vec<String>,
    /// Hard invariant: this component never imitates native permission or irreversible approval
    /// UI. MUST be `false`.
    pub imitates_native_permission_or_approval_ui: bool,
    /// Hard invariant: this component never hides owner/origin or browser fallback behind menus
    /// only. MUST be `false`.
    pub hides_owner_origin_or_browser_fallback_in_menus_only: bool,
    /// Hard invariant: this component never renders a stale, offline, or provider-blocked pane as
    /// fresh first-party local truth. MUST be `false`.
    pub renders_stale_or_blocked_as_fresh_first_party_truth: bool,
    /// Hard invariant: this component never embeds a high-risk approval without a native step-up.
    /// MUST be `false`.
    pub embeds_high_risk_approval_without_native_step_up: bool,
}

impl M5EmbeddedBoundaryComponentRow {
    /// `true` when the row declares all mandatory labels.
    fn declares_mandatory_labels(&self) -> bool {
        let present: BTreeSet<M5EmbeddedRequiredLabel> =
            self.required_labels.iter().copied().collect();
        M5EmbeddedRequiredLabel::MANDATORY
            .iter()
            .all(|label| present.contains(label))
    }

    /// `true` when the row's hard invariants hold.
    fn honours_invariants(&self) -> bool {
        !self.imitates_native_permission_or_approval_ui
            && !self.hides_owner_origin_or_browser_fallback_in_menus_only
            && !self.renders_stale_or_blocked_as_fresh_first_party_truth
            && !self.embeds_high_risk_approval_without_native_step_up
    }
}

/// Self-describing controlled-vocabulary set frozen by the matrix.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5EmbeddedBoundaryVocabularySet {
    /// Component-family tokens.
    pub component_families: Vec<String>,
    /// Boundary-disposition tokens.
    pub boundary_dispositions: Vec<String>,
    /// Owner-class tokens (bound from the auth-boundary object model).
    pub owner_classes: Vec<String>,
    /// Data-exit-boundary tokens (bound from the public-truth object model).
    pub data_exit_boundaries: Vec<String>,
    /// Browser-handoff-kind tokens (bound from the auth-boundary object model).
    pub browser_handoff_kinds: Vec<String>,
    /// Capability-limit tokens (bound from the auth-boundary object model).
    pub capability_limits: Vec<String>,
    /// Freshness-state tokens.
    pub freshness_states: Vec<String>,
    /// Account-scope tokens.
    pub account_scopes: Vec<String>,
    /// Surface-family tokens.
    pub surface_families: Vec<String>,
    /// Deployment-line tokens.
    pub deployment_lines: Vec<String>,
    /// Consumer-surface tokens.
    pub consumer_surfaces: Vec<String>,
    /// Accessibility-route tokens.
    pub accessibility_routes: Vec<String>,
    /// Degraded-reason tokens.
    pub degraded_reasons: Vec<String>,
    /// Required-label tokens.
    pub required_labels: Vec<String>,
}

impl M5EmbeddedBoundaryVocabularySet {
    /// Builds the canonical vocabulary set from the typed `ALL` arrays.
    pub fn canonical() -> Self {
        Self {
            component_families: tokens(&M5EmbeddedBoundaryComponentFamily::ALL, |v| v.as_str()),
            boundary_dispositions: tokens(&M5EmbeddedBoundaryDisposition::ALL, |v| v.as_str()),
            owner_classes: tokens(&WebviewOwnerClass::ALL, |v| v.as_str()),
            data_exit_boundaries: tokens(&BOUND_DATA_EXIT_BOUNDARIES, |v| v.as_str()),
            browser_handoff_kinds: tokens(&BrowserHandoffKind::ALL, |v| v.as_str()),
            capability_limits: tokens(&CapabilityLimitClass::ALL, |v| v.as_str()),
            freshness_states: tokens(&M5EmbeddedFreshnessState::ALL, |v| v.as_str()),
            account_scopes: tokens(&M5EmbeddedAccountScope::ALL, |v| v.as_str()),
            surface_families: tokens(&M5EmbeddedSurfaceFamily::ALL, |v| v.as_str()),
            deployment_lines: tokens(&M5EmbeddedDeploymentLine::ALL, |v| v.as_str()),
            consumer_surfaces: tokens(&M5EmbeddedConsumerSurface::ALL, |v| v.as_str()),
            accessibility_routes: tokens(&M5EmbeddedAccessibilityRoute::ALL, |v| v.as_str()),
            degraded_reasons: tokens(&M5EmbeddedDegradedReason::ALL, |v| v.as_str()),
            required_labels: tokens(&M5EmbeddedRequiredLabel::ALL, |v| v.as_str()),
        }
    }

    /// Returns true when this set matches the canonical token lists exactly.
    pub fn matches_canonical(&self) -> bool {
        *self == Self::canonical()
    }
}

/// The canonical data-exit boundaries bound from the shared public-truth object model, in
/// canonical order. [`DataExitBoundary`] does not export its own `ALL`, so the matrix pins the
/// full set here to keep the frozen vocabulary stable and complete.
pub const BOUND_DATA_EXIT_BOUNDARIES: [DataExitBoundary; 7] = [
    DataExitBoundary::NoPayloadLeavesProduct,
    DataExitBoundary::MetadataSafeObjectRefs,
    DataExitBoundary::ProposalRefsOnly,
    DataExitBoundary::RedactedSupportPacket,
    DataExitBoundary::SecurityPayloadsOnly,
    DataExitBoundary::ExternalPublicBrowse,
    DataExitBoundary::VendorOrThirdPartyOutbound,
];

fn tokens<T: Copy>(items: &[T], to_token: impl Fn(T) -> &'static str) -> Vec<String> {
    items.iter().map(|v| to_token(*v).to_owned()).collect()
}

/// Governance-review block; every flag is a hard invariant.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5EmbeddedBoundaryComponentGovernanceReview {
    /// The docs-pane header shows its owner/origin and freshness.
    pub docs_pane_header_shows_owner_origin_and_freshness: bool,
    /// The embedded-origin bar shows its owner and capability limits.
    pub embedded_origin_bar_shows_owner_and_capability_limits: bool,
    /// The boundary-fact grid shows owner/origin, data boundary, and freshness together.
    pub boundary_fact_grid_shows_owner_origin_data_boundary_freshness: bool,
    /// The marketplace/account boundary card shows its account scope.
    pub marketplace_account_boundary_card_shows_account_scope: bool,
    /// The auth-handoff card shows the browser fallback and data boundary.
    pub auth_handoff_card_shows_browser_fallback_and_data_boundary: bool,
    /// The remote/service dashboard header shows provider health and freshness.
    pub remote_service_dashboard_header_shows_provider_health_and_freshness: bool,
    /// The open-in-browser handoff row shows the browser fallback, never menu-only.
    pub open_in_browser_handoff_row_shows_browser_fallback: bool,
    /// The embedded-state panel shows stale/offline/blocked state explicitly.
    pub embedded_state_panel_shows_stale_offline_blocked_explicitly: bool,
    /// No embedded surface imitates native permission or irreversible-approval chrome.
    pub no_embedded_surface_imitates_native_approval_chrome: bool,
    /// Owner and origin are always explicit.
    pub owner_and_origin_always_explicit: bool,
    /// The data boundary is always explicit where it applies.
    pub data_boundary_always_explicit: bool,
    /// The browser fallback is never hidden behind menus only.
    pub browser_fallback_never_menu_only: bool,
    /// A stale, offline, or provider-blocked pane is never shown as fresh first-party truth.
    pub stale_offline_blocked_never_shown_as_fresh: bool,
    /// Every component keeps the same truth across every deployment line.
    pub every_component_declares_deployment_lines: bool,
    /// Every component declares a non-visual accessibility route.
    pub every_component_declares_accessibility_route: bool,
    /// Later M5 rows cannot invent parallel boundary vocabulary.
    pub later_rows_cannot_invent_parallel_boundary_vocabulary: bool,
}

/// Consumer projection block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5EmbeddedBoundaryComponentConsumerProjection {
    /// Docs and help surfaces consume the shared owner/origin vocabulary.
    pub docs_and_help_surfaces_consume_owner_origin_vocabulary: bool,
    /// Marketplace and account surfaces consume the shared account-scope vocabulary.
    pub marketplace_and_account_surfaces_consume_account_scope_vocabulary: bool,
    /// Remote dashboard surfaces consume the shared freshness vocabulary.
    pub remote_dashboard_surfaces_consume_freshness_vocabulary: bool,
    /// Webview surfaces consume the shared capability-limit vocabulary.
    pub webview_surfaces_consume_capability_limit_vocabulary: bool,
    /// Auth-handoff surfaces consume the shared browser-handoff vocabulary.
    pub auth_handoff_surfaces_consume_browser_handoff_vocabulary: bool,
    /// Support / export reads a single canonical boundary source.
    pub support_export_reads_single_boundary_source: bool,
}

/// Proof freshness block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5EmbeddedBoundaryComponentProofFreshness {
    /// Proof-freshness SLO in hours.
    pub proof_freshness_slo_hours: u32,
    /// RFC 3339 timestamp of the last proof refresh.
    pub last_proof_refresh: String,
    /// True when stale proof automatically narrows the component.
    pub auto_narrow_on_stale: bool,
}

/// Release and support parity posture for the embedded-boundary component lane.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5EmbeddedBoundaryComponentReleasePosture {
    /// Ref of the supporting proof packet for the lane.
    pub proof_packet_ref: String,
    /// Ref of the supporting boundary audit for the lane.
    pub boundary_audit_ref: String,
    /// True when support/export parity is required for every component.
    pub support_export_parity_required: bool,
    /// True when accessibility parity is required for every component.
    pub accessibility_parity_required: bool,
}

/// Constructor input for [`M5EmbeddedBoundaryComponentMatrixPacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M5EmbeddedBoundaryComponentMatrixPacketInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable matrix label.
    pub matrix_label: String,
    /// Component rows.
    pub component_rows: Vec<M5EmbeddedBoundaryComponentRow>,
    /// Frozen controlled-vocabulary set.
    pub vocabulary_set: M5EmbeddedBoundaryVocabularySet,
    /// Governance-review block.
    pub governance_review: M5EmbeddedBoundaryComponentGovernanceReview,
    /// Consumer projection block.
    pub consumer_projection: M5EmbeddedBoundaryComponentConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: M5EmbeddedBoundaryComponentProofFreshness,
    /// Release and support parity posture.
    pub release_posture: M5EmbeddedBoundaryComponentReleasePosture,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

/// Export-safe frozen M5 embedded-boundary component matrix packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5EmbeddedBoundaryComponentMatrixPacket {
    /// Record kind; must equal [`M5_EMBEDDED_BOUNDARY_COMPONENT_MATRIX_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`M5_EMBEDDED_BOUNDARY_COMPONENT_MATRIX_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable matrix label.
    pub matrix_label: String,
    /// Component rows.
    pub component_rows: Vec<M5EmbeddedBoundaryComponentRow>,
    /// Frozen controlled-vocabulary set.
    pub vocabulary_set: M5EmbeddedBoundaryVocabularySet,
    /// Governance-review block.
    pub governance_review: M5EmbeddedBoundaryComponentGovernanceReview,
    /// Consumer projection block.
    pub consumer_projection: M5EmbeddedBoundaryComponentConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: M5EmbeddedBoundaryComponentProofFreshness,
    /// Release and support parity posture.
    pub release_posture: M5EmbeddedBoundaryComponentReleasePosture,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

impl M5EmbeddedBoundaryComponentMatrixPacket {
    /// Builds an M5 embedded-boundary component matrix packet from stable-lane input.
    pub fn new(input: M5EmbeddedBoundaryComponentMatrixPacketInput) -> Self {
        Self {
            record_kind: M5_EMBEDDED_BOUNDARY_COMPONENT_MATRIX_RECORD_KIND.to_owned(),
            schema_version: M5_EMBEDDED_BOUNDARY_COMPONENT_MATRIX_SCHEMA_VERSION,
            packet_id: input.packet_id,
            matrix_label: input.matrix_label,
            component_rows: input.component_rows,
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

    /// Validates the M5 embedded-boundary component matrix invariants.
    pub fn validate(&self) -> Vec<M5EmbeddedBoundaryComponentMatrixViolation> {
        let mut violations = Vec::new();

        if self.record_kind != M5_EMBEDDED_BOUNDARY_COMPONENT_MATRIX_RECORD_KIND {
            violations.push(M5EmbeddedBoundaryComponentMatrixViolation::WrongRecordKind);
        }
        if self.schema_version != M5_EMBEDDED_BOUNDARY_COMPONENT_MATRIX_SCHEMA_VERSION {
            violations.push(M5EmbeddedBoundaryComponentMatrixViolation::WrongSchemaVersion);
        }
        if self.packet_id.trim().is_empty()
            || self.matrix_label.trim().is_empty()
            || self.redaction_class_token.trim().is_empty()
            || self.minted_at.trim().is_empty()
        {
            violations.push(M5EmbeddedBoundaryComponentMatrixViolation::MissingIdentity);
        }

        validate_source_contracts(self, &mut violations);
        validate_vocabulary_set(self, &mut violations);
        validate_component_rows(self, &mut violations);
        validate_governance_review(self, &mut violations);
        validate_consumer_projection(self, &mut violations);
        validate_proof_freshness(self, &mut violations);
        validate_release_posture(self, &mut violations);

        if json_contains_forbidden_material(
            &serde_json::to_value(self).expect("m5 embedded-boundary component matrix serializes"),
        ) {
            violations.push(M5EmbeddedBoundaryComponentMatrixViolation::RawMaterialInExport);
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
            .expect("m5 embedded-boundary component matrix packet serializes")
    }

    /// Deterministic, machine-readable matrix CSV: one row per governed component.
    pub fn render_matrix_csv(&self) -> String {
        let mut out = String::new();
        out.push_str(
            "component_family,qualification,owner,canonical_schema,surface_families,deployment_lines,required_labels,consumer_surfaces,downgrade_triggers\n",
        );
        for row in &self.component_rows {
            out.push_str(&format!(
                "{},{},{},{},{},{},{},{},{}\n",
                row.component_family.as_str(),
                row.qualification.as_str(),
                csv_field(&row.owner_role),
                row.component_family.canonical_component_schema_ref(),
                join_tokens(&row.surface_families, |v| v.as_str()),
                join_tokens(&row.deployment_lines, |v| v.as_str()),
                join_tokens(&row.required_labels, |v| v.as_str()),
                join_tokens(&row.consumer_surfaces, |v| v.as_str()),
                join_tokens(&row.downgrade_triggers, |v| v.as_str()),
            ));
        }
        out
    }

    /// Deterministic Markdown report for support, docs, or review handoff.
    pub fn render_markdown_summary(&self) -> String {
        let stable_components = self
            .component_rows
            .iter()
            .filter(|row| row.qualification.is_stable())
            .count();
        let mut out = String::new();
        out.push_str(
            "# M5 Docs-Pane-Header, Embedded-Origin-Bar, Boundary-Fact-Grid, Marketplace-Account-Boundary-Card, Auth-Handoff-Card, Remote-Service-Dashboard-Header, Open-In-Browser-Handoff-Row, and Embedded-State-Panel Component Matrix\n\n",
        );
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- Label: `{}`\n", self.matrix_label));
        out.push_str(&format!(
            "- Component families: {} ({} stable)\n",
            self.component_rows.len(),
            stable_components
        ));
        out.push_str(&format!(
            "- Boundary dispositions: {}\n",
            self.vocabulary_set.boundary_dispositions.join(", ")
        ));
        out.push_str(&format!(
            "- Owner classes: {}\n",
            self.vocabulary_set.owner_classes.join(", ")
        ));
        out.push_str(&format!(
            "- Proof freshness SLO: {} hours (last refresh: {})\n",
            self.proof_freshness.proof_freshness_slo_hours, self.proof_freshness.last_proof_refresh
        ));
        out.push_str("\n## Component families\n\n");
        for row in &self.component_rows {
            out.push_str(&format!(
                "- **{}**: `{}`\n",
                row.component_family.as_str(),
                row.qualification.as_str()
            ));
            out.push_str(&format!("  - Owner: {}\n", row.owner_role));
            out.push_str(&format!(
                "  - Canonical schema: `{}`\n",
                row.component_family.canonical_component_schema_ref()
            ));
            out.push_str(&format!("  - Scope: {}\n", row.scope_summary));
            out.push_str(&format!(
                "  - Required labels: {}\n",
                row.required_labels
                    .iter()
                    .map(|v| v.as_str())
                    .collect::<Vec<_>>()
                    .join(", ")
            ));
            out.push_str(&format!(
                "  - Accessibility routes: {}\n",
                row.accessibility_routes
                    .iter()
                    .map(|v| v.as_str())
                    .collect::<Vec<_>>()
                    .join(", ")
            ));
        }
        out
    }
}

/// Errors emitted when reading the checked-in M5 embedded-boundary matrix export.
#[derive(Debug)]
pub enum M5EmbeddedBoundaryComponentMatrixArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<M5EmbeddedBoundaryComponentMatrixViolation>),
}

impl fmt::Display for M5EmbeddedBoundaryComponentMatrixArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(
                    formatter,
                    "m5 embedded-boundary component matrix export parse failed: {error}"
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
                    "m5 embedded-boundary component matrix export failed validation: {tokens}"
                )
            }
        }
    }
}

impl Error for M5EmbeddedBoundaryComponentMatrixArtifactError {}

/// Validation failures emitted by [`M5EmbeddedBoundaryComponentMatrixPacket::validate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum M5EmbeddedBoundaryComponentMatrixViolation {
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
    /// A required governed component family is missing from the matrix.
    RequiredComponentMissing,
    /// A component row is incomplete.
    ComponentRowIncomplete,
    /// A component row omits one of the mandatory labels.
    MandatoryLabelMissing,
    /// A component row does not point at its own canonical component schema.
    ComponentSchemaRefMissing,
    /// A component declares no boundary dispositions.
    BoundaryDispositionMissing,
    /// An origin-bearing component declares no owner classes.
    OwnerClassMissing,
    /// A data-crossing component declares no data-exit boundaries.
    DataBoundaryMissing,
    /// A handoff component declares no browser-handoff kinds.
    BrowserHandoffMissing,
    /// An embedded component declares no capability limits.
    CapabilityLimitsMissing,
    /// A freshness-bearing component declares no freshness states.
    FreshnessStateMissing,
    /// An account component declares no account scopes.
    AccountScopeMissing,
    /// A component declares no degraded reasons.
    DegradedReasonMissing,
    /// A component declares no surface families.
    SurfaceFamilyMissing,
    /// A component declares no deployment lines.
    DeploymentLineMissing,
    /// A component declares no accessibility routes.
    AccessibilityRouteMissing,
    /// A component declares no consumer surfaces.
    ConsumerSurfacesMissing,
    /// A component declares no downgrade triggers.
    DowngradeTriggersMissing,
    /// A component claiming Stable is missing required proof packet refs.
    StableComponentMissingProof,
    /// A component violates a hard invariant (imitates native approval chrome, hides owner/origin
    /// or browser fallback in menus only, renders stale/blocked as fresh, or embeds a high-risk
    /// approval without a native step-up).
    ComponentInvariantViolated,
    /// Governance review does not satisfy required invariants.
    GovernanceReviewIncomplete,
    /// Consumer projection does not satisfy required invariants.
    ConsumerProjectionIncomplete,
    /// Proof freshness block is incomplete.
    ProofFreshnessIncomplete,
    /// Release/support parity posture is incomplete.
    ReleasePostureIncomplete,
    /// Export contains raw sensitive material.
    RawMaterialInExport,
}

impl M5EmbeddedBoundaryComponentMatrixViolation {
    /// Stable token used in tests and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WrongRecordKind => "wrong_record_kind",
            Self::WrongSchemaVersion => "wrong_schema_version",
            Self::MissingIdentity => "missing_identity",
            Self::MissingSourceContracts => "missing_source_contracts",
            Self::VocabularySetDrift => "vocabulary_set_drift",
            Self::RequiredComponentMissing => "required_component_missing",
            Self::ComponentRowIncomplete => "component_row_incomplete",
            Self::MandatoryLabelMissing => "mandatory_label_missing",
            Self::ComponentSchemaRefMissing => "component_schema_ref_missing",
            Self::BoundaryDispositionMissing => "boundary_disposition_missing",
            Self::OwnerClassMissing => "owner_class_missing",
            Self::DataBoundaryMissing => "data_boundary_missing",
            Self::BrowserHandoffMissing => "browser_handoff_missing",
            Self::CapabilityLimitsMissing => "capability_limits_missing",
            Self::FreshnessStateMissing => "freshness_state_missing",
            Self::AccountScopeMissing => "account_scope_missing",
            Self::DegradedReasonMissing => "degraded_reason_missing",
            Self::SurfaceFamilyMissing => "surface_family_missing",
            Self::DeploymentLineMissing => "deployment_line_missing",
            Self::AccessibilityRouteMissing => "accessibility_route_missing",
            Self::ConsumerSurfacesMissing => "consumer_surfaces_missing",
            Self::DowngradeTriggersMissing => "downgrade_triggers_missing",
            Self::StableComponentMissingProof => "stable_component_missing_proof",
            Self::ComponentInvariantViolated => "component_invariant_violated",
            Self::GovernanceReviewIncomplete => "governance_review_incomplete",
            Self::ConsumerProjectionIncomplete => "consumer_projection_incomplete",
            Self::ProofFreshnessIncomplete => "proof_freshness_incomplete",
            Self::ReleasePostureIncomplete => "release_posture_incomplete",
            Self::RawMaterialInExport => "raw_material_in_export",
        }
    }
}

/// Reads and validates the checked-in stable M5 embedded-boundary matrix export.
pub fn current_stable_m5_embedded_boundary_component_matrix_export(
) -> Result<M5EmbeddedBoundaryComponentMatrixPacket, M5EmbeddedBoundaryComponentMatrixArtifactError>
{
    let packet: M5EmbeddedBoundaryComponentMatrixPacket =
        serde_json::from_str(include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../artifacts/release/m5-embedded-boundary-proof/support_export.json"
        )))
        .map_err(M5EmbeddedBoundaryComponentMatrixArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(M5EmbeddedBoundaryComponentMatrixArtifactError::Validation(
            violations,
        ))
    }
}

fn validate_source_contracts(
    packet: &M5EmbeddedBoundaryComponentMatrixPacket,
    violations: &mut Vec<M5EmbeddedBoundaryComponentMatrixViolation>,
) {
    let refs: BTreeSet<&str> = packet
        .source_contract_refs
        .iter()
        .map(String::as_str)
        .collect();
    for required in [
        M5_EMBEDDED_BOUNDARY_COMPONENT_SCHEMA_REF,
        M5_EMBEDDED_BOUNDARY_COMPONENT_DOC_REF,
        M5_DOCS_PANE_HEADER_SCHEMA_REF,
        M5_EMBEDDED_ORIGIN_BAR_SCHEMA_REF,
        M5_BOUNDARY_FACT_GRID_SCHEMA_REF,
        M5_MARKETPLACE_ACCOUNT_BOUNDARY_CARD_SCHEMA_REF,
        M5_AUTH_HANDOFF_CARD_SCHEMA_REF,
        M5_REMOTE_SERVICE_DASHBOARD_HEADER_SCHEMA_REF,
        M5_OPEN_IN_BROWSER_HANDOFF_ROW_SCHEMA_REF,
        M5_EMBEDDED_STATE_PANEL_SCHEMA_REF,
    ] {
        if !refs.contains(required) {
            violations.push(M5EmbeddedBoundaryComponentMatrixViolation::MissingSourceContracts);
            return;
        }
    }
}

fn validate_vocabulary_set(
    packet: &M5EmbeddedBoundaryComponentMatrixPacket,
    violations: &mut Vec<M5EmbeddedBoundaryComponentMatrixViolation>,
) {
    if !packet.vocabulary_set.matches_canonical() {
        violations.push(M5EmbeddedBoundaryComponentMatrixViolation::VocabularySetDrift);
    }
}

fn validate_component_rows(
    packet: &M5EmbeddedBoundaryComponentMatrixPacket,
    violations: &mut Vec<M5EmbeddedBoundaryComponentMatrixViolation>,
) {
    let present: BTreeSet<M5EmbeddedBoundaryComponentFamily> = packet
        .component_rows
        .iter()
        .map(|row| row.component_family)
        .collect();
    for required in M5EmbeddedBoundaryComponentFamily::ALL {
        if !present.contains(&required) {
            violations.push(M5EmbeddedBoundaryComponentMatrixViolation::RequiredComponentMissing);
            return;
        }
    }

    for row in &packet.component_rows {
        let family = row.component_family;
        if row.owner_role.trim().is_empty()
            || row.scope_summary.trim().is_empty()
            || row.source_contract_refs.is_empty()
            || row.required_labels.is_empty()
        {
            violations.push(M5EmbeddedBoundaryComponentMatrixViolation::ComponentRowIncomplete);
        }
        if !row.declares_mandatory_labels() {
            violations.push(M5EmbeddedBoundaryComponentMatrixViolation::MandatoryLabelMissing);
        }
        if !row
            .source_contract_refs
            .iter()
            .any(|r| r == family.canonical_component_schema_ref())
        {
            violations.push(M5EmbeddedBoundaryComponentMatrixViolation::ComponentSchemaRefMissing);
        }
        if row.boundary_dispositions.is_empty() {
            violations.push(M5EmbeddedBoundaryComponentMatrixViolation::BoundaryDispositionMissing);
        }
        if family.declares_owner_class() && row.owner_classes.is_empty() {
            violations.push(M5EmbeddedBoundaryComponentMatrixViolation::OwnerClassMissing);
        }
        if family.declares_data_boundary() && row.data_exit_boundaries.is_empty() {
            violations.push(M5EmbeddedBoundaryComponentMatrixViolation::DataBoundaryMissing);
        }
        if family.declares_browser_handoff() && row.browser_handoff_kinds.is_empty() {
            violations.push(M5EmbeddedBoundaryComponentMatrixViolation::BrowserHandoffMissing);
        }
        if family.declares_capability_limits() && row.capability_limits.is_empty() {
            violations.push(M5EmbeddedBoundaryComponentMatrixViolation::CapabilityLimitsMissing);
        }
        if family.declares_freshness() && row.freshness_states.is_empty() {
            violations.push(M5EmbeddedBoundaryComponentMatrixViolation::FreshnessStateMissing);
        }
        if family.declares_account_scope() && row.account_scopes.is_empty() {
            violations.push(M5EmbeddedBoundaryComponentMatrixViolation::AccountScopeMissing);
        }
        if row.degraded_reasons.is_empty() {
            violations.push(M5EmbeddedBoundaryComponentMatrixViolation::DegradedReasonMissing);
        }
        if row.surface_families.is_empty() {
            violations.push(M5EmbeddedBoundaryComponentMatrixViolation::SurfaceFamilyMissing);
        }
        if row.deployment_lines.is_empty() {
            violations.push(M5EmbeddedBoundaryComponentMatrixViolation::DeploymentLineMissing);
        }
        if row.accessibility_routes.is_empty() {
            violations.push(M5EmbeddedBoundaryComponentMatrixViolation::AccessibilityRouteMissing);
        }
        if row.consumer_surfaces.is_empty() {
            violations.push(M5EmbeddedBoundaryComponentMatrixViolation::ConsumerSurfacesMissing);
        }
        if row.downgrade_triggers.is_empty() {
            violations.push(M5EmbeddedBoundaryComponentMatrixViolation::DowngradeTriggersMissing);
        }
        if row.qualification.is_stable() && row.required_proof_packet_refs.is_empty() {
            violations
                .push(M5EmbeddedBoundaryComponentMatrixViolation::StableComponentMissingProof);
        }
        if !row.honours_invariants() {
            violations.push(M5EmbeddedBoundaryComponentMatrixViolation::ComponentInvariantViolated);
        }
    }
}

fn validate_governance_review(
    packet: &M5EmbeddedBoundaryComponentMatrixPacket,
    violations: &mut Vec<M5EmbeddedBoundaryComponentMatrixViolation>,
) {
    let review = &packet.governance_review;
    for ok in [
        review.docs_pane_header_shows_owner_origin_and_freshness,
        review.embedded_origin_bar_shows_owner_and_capability_limits,
        review.boundary_fact_grid_shows_owner_origin_data_boundary_freshness,
        review.marketplace_account_boundary_card_shows_account_scope,
        review.auth_handoff_card_shows_browser_fallback_and_data_boundary,
        review.remote_service_dashboard_header_shows_provider_health_and_freshness,
        review.open_in_browser_handoff_row_shows_browser_fallback,
        review.embedded_state_panel_shows_stale_offline_blocked_explicitly,
        review.no_embedded_surface_imitates_native_approval_chrome,
        review.owner_and_origin_always_explicit,
        review.data_boundary_always_explicit,
        review.browser_fallback_never_menu_only,
        review.stale_offline_blocked_never_shown_as_fresh,
        review.every_component_declares_deployment_lines,
        review.every_component_declares_accessibility_route,
        review.later_rows_cannot_invent_parallel_boundary_vocabulary,
    ] {
        if !ok {
            violations.push(M5EmbeddedBoundaryComponentMatrixViolation::GovernanceReviewIncomplete);
            return;
        }
    }
}

fn validate_consumer_projection(
    packet: &M5EmbeddedBoundaryComponentMatrixPacket,
    violations: &mut Vec<M5EmbeddedBoundaryComponentMatrixViolation>,
) {
    let projection = &packet.consumer_projection;
    for ok in [
        projection.docs_and_help_surfaces_consume_owner_origin_vocabulary,
        projection.marketplace_and_account_surfaces_consume_account_scope_vocabulary,
        projection.remote_dashboard_surfaces_consume_freshness_vocabulary,
        projection.webview_surfaces_consume_capability_limit_vocabulary,
        projection.auth_handoff_surfaces_consume_browser_handoff_vocabulary,
        projection.support_export_reads_single_boundary_source,
    ] {
        if !ok {
            violations
                .push(M5EmbeddedBoundaryComponentMatrixViolation::ConsumerProjectionIncomplete);
            return;
        }
    }
}

fn validate_proof_freshness(
    packet: &M5EmbeddedBoundaryComponentMatrixPacket,
    violations: &mut Vec<M5EmbeddedBoundaryComponentMatrixViolation>,
) {
    if packet.proof_freshness.proof_freshness_slo_hours == 0
        || packet.proof_freshness.last_proof_refresh.trim().is_empty()
    {
        violations.push(M5EmbeddedBoundaryComponentMatrixViolation::ProofFreshnessIncomplete);
    }
}

fn validate_release_posture(
    packet: &M5EmbeddedBoundaryComponentMatrixPacket,
    violations: &mut Vec<M5EmbeddedBoundaryComponentMatrixViolation>,
) {
    let posture = &packet.release_posture;
    if posture.proof_packet_ref.trim().is_empty()
        || posture.boundary_audit_ref.trim().is_empty()
        || !posture.support_export_parity_required
        || !posture.accessibility_parity_required
    {
        violations.push(M5EmbeddedBoundaryComponentMatrixViolation::ReleasePostureIncomplete);
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

/// Heuristic that rejects obviously forbidden raw material in export-safe JSON. The controlled
/// vocabulary deliberately uses boundary words; what is rejected is a raw secret *value* shape —
/// a pasted passphrase, a bearer token, a raw endpoint URL, or a PEM key block.
fn json_contains_forbidden_material(value: &serde_json::Value) -> bool {
    match value {
        serde_json::Value::String(s) => {
            let lower = s.to_lowercase();
            lower.contains("password")
                || lower.contains("passphrase")
                || lower.contains("bearer ")
                || lower.contains("://")
                || lower.contains("-----begin")
        }
        serde_json::Value::Array(arr) => arr.iter().any(json_contains_forbidden_material),
        serde_json::Value::Object(map) => map.values().any(json_contains_forbidden_material),
        _ => false,
    }
}

/// Repo-relative refs of the auth-boundary object model this matrix binds against, so no consumer
/// forks its own owner/origin or browser-handoff vocabulary. Re-exported for callers that
/// assemble the full source-contract set.
pub const M5_EMBEDDED_BOUNDARY_BINDING_REFS: [&str; 3] = [
    M5_AUTH_BOUNDARY_CONTRACT_DOC_REF,
    M5_BROWSER_HANDOFF_CARD_SCHEMA_REF,
    M5_WEBVIEW_ORIGIN_BAR_SCHEMA_REF,
];
