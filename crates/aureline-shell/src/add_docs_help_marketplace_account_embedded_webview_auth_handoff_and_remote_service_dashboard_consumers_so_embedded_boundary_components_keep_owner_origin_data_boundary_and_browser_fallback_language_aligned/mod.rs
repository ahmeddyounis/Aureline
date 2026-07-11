//! Shared docs/help-pane, marketplace/account, extension-owned embedded-webview,
//! browser/device-code auth-handoff, remote/service-dashboard, and support /
//! export + release-packet consumers for the frozen M5 embedded-boundary
//! components.
//!
//! This module is the M05-1074 consumer-adoption lane over the frozen M5
//! embedded-boundary component matrix
//! ([`crate::freeze_the_m5_docs_pane_header_embedded_origin_bar_boundary_fact_grid_marketplace_account_boundary_card_auth_handoff_card_remote_service_dashboard_header_open_in_browser_handoff_row_and_embedded_state_panel_component_matrix`]).
//! Where the freeze matrix defines the eight reusable docs-pane header,
//! embedded-origin bar, boundary-fact grid, marketplace/account boundary card,
//! auth-handoff card, remote/service dashboard header, open-in-browser handoff
//! row, and embedded-state panel primitives — and the four B127 implement lanes
//! wire their resolvers and controls contracts — this lane proves those families
//! are reusable *primitives* rather than per-pane embedded chrome. It adopts them
//! across the claimed M5 embedded / browser-handoff consumer classes:
//!
//! 1. a docs / help pane,
//! 2. a marketplace / account content surface,
//! 3. an extension-owned embedded-webview surface,
//! 4. a browser / device-code auth-handoff surface,
//! 5. a remote / service dashboard surface, and
//! 6. a support / export + release-packet lane (AC2).
//!
//! Each [`EmbeddedConsumerRow`] points back to exactly one canonical component
//! family (its per-family matrix schema) and the one canonical controls contract
//! (schema + doc + release-proof artifact) its family group belongs to, instead
//! of cloning pane-local embedded chrome. Every consumer — even a read-only,
//! inspect-only, export-only, or docs reference — keeps the identical
//! owner/origin, data-boundary, source/version/last-updated, network/offline
//! state, browser-fallback, account-scope, freshness, capability-limit, and
//! no-embedded-high-risk-approval labels and the identical frozen
//! boundary-disposition vocabulary. A narrower consumer discloses the reduction
//! with a reduced-capability banner (and, when it punts to another surface, a
//! desktop / companion / browser / support-packet note) rather than renaming or
//! dropping governed boundary truth, so docs, marketplace, webview, auth, remote,
//! and support panes never fork embedded-boundary vocabulary by surface. This is
//! what makes the same origin / boundary state render with one vocabulary and one
//! component family across every claimed consumer (AC1), and lets help / support
//! / release packets drop bespoke per-pane prose (AC2).
//!
//! The four spec guardrails are enforced per row and must all stay false: no
//! consumer imitates native permission or irreversible approval UI; no consumer
//! hides owner/origin or browser fallback behind menus only; no consumer renders
//! a stale / offline / provider-blocked pane as fresh first-party local truth; no
//! consumer embeds a high-risk approval without a native step-up.
//!
//! The packet is metadata-only: raw provider tokens, credential material, and
//! cookies never cross this boundary; the packet carries only typed class tokens,
//! opaque boundary-state refs, booleans, and redacted labels.
//!
//! The boundary schema is
//! [`schemas/ui/m5-embedded-boundary-component-consumer.schema.json`](../../../../schemas/ui/m5-embedded-boundary-component-consumer.schema.json).
//! The contract doc is
//! [`docs/help/m5_embedded_boundary_component_consumer_contract.md`](../../../../docs/help/m5_embedded_boundary_component_consumer_contract.md).

#[cfg(test)]
mod tests;

use std::collections::{BTreeMap, BTreeSet};
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

use crate::freeze_the_m5_docs_pane_header_embedded_origin_bar_boundary_fact_grid_marketplace_account_boundary_card_auth_handoff_card_remote_service_dashboard_header_open_in_browser_handoff_row_and_embedded_state_panel_component_matrix as matrix;
use crate::implement_the_m5_auth_handoff_card_and_remote_service_dashboard_header_provider_domain_reason_fallback_local_continuity_device_code_expiry_target_service_identity_freshness_export_open_console_and_no_embedded_high_risk_approval_primitive as auth_dashboard_controls;
use crate::implement_the_m5_docs_pane_header_and_boundary_fact_grid_source_class_version_owner_origin_open_externally_and_data_boundary_primitive as docs_boundary_controls;
use crate::implement_the_m5_embedded_origin_bar_and_embedded_state_panel_extension_publisher_origin_permission_capability_reload_open_in_browser_and_offline_stale_policy_blocked_cross_origin_state_primitive as origin_state_controls;
use crate::implement_the_m5_marketplace_account_boundary_card_and_open_in_browser_handoff_row_origin_account_scope_profile_region_tenant_network_state_browser_fallback_and_local_safe_continuity_primitive as marketplace_handoff_controls;

pub use matrix::{
    M5EmbeddedBoundaryComponentFamily, M5EmbeddedBoundaryDisposition, M5EmbeddedConsumerSurface,
};

/// Schema version stamped on the M05-1074 consumer packet.
pub const EMBEDDED_CONSUMER_SCHEMA_VERSION: u32 = 1;

/// Stable record-kind tag carried by [`EmbeddedConsumerPacket`].
pub const EMBEDDED_CONSUMER_RECORD_KIND: &str = "m5_embedded_boundary_component_consumer_packet";

/// Stable record-kind tag carried by each [`EmbeddedConsumerRow`].
pub const EMBEDDED_CONSUMER_ROW_RECORD_KIND: &str = "m5_embedded_boundary_component_consumer_row";

/// Repo-relative path of the boundary schema.
pub const EMBEDDED_CONSUMER_SCHEMA_REF: &str =
    "schemas/ui/m5-embedded-boundary-component-consumer.schema.json";

/// Repo-relative path of the contract doc.
pub const EMBEDDED_CONSUMER_DOC_REF: &str =
    "docs/help/m5_embedded_boundary_component_consumer_contract.md";

/// Repo-relative path of the frozen embedded-boundary component matrix release
/// proof these consumers adopt.
pub const EMBEDDED_CONSUMER_MATRIX_REF: &str = matrix::M5_EMBEDDED_BOUNDARY_COMPONENT_ARTIFACT_REF;

/// Repo-relative path of the shared frozen component-matrix schema.
pub const EMBEDDED_CONSUMER_SHARED_SCHEMA_REF: &str =
    matrix::M5_EMBEDDED_BOUNDARY_COMPONENT_SCHEMA_REF;

/// Repo-relative path of the checked support-export artifact (the `include_str!`
/// canonical).
pub const EMBEDDED_CONSUMER_ARTIFACT_REF: &str =
    "artifacts/release/m5-embedded-boundary-component-consumer-proof/support_export.json";

/// Repo-relative path of the checked machine-readable matrix CSV.
pub const EMBEDDED_CONSUMER_CSV_REF: &str =
    "artifacts/release/m5-embedded-boundary-component-consumer-proof/matrix.csv";

/// Repo-relative path of the checked Markdown report.
pub const EMBEDDED_CONSUMER_REPORT_REF: &str =
    "artifacts/release/m5-embedded-boundary-component-consumer-proof/report.md";

/// Repo-relative path of the checked consumer-fixture directory.
pub const EMBEDDED_CONSUMER_FIXTURE_DIR: &str =
    "fixtures/ui/m5-embedded-boundary-component-consumers";

/// The controlled label families a consumer must preserve identically across
/// every embedded / browser-handoff surface. These are the track-invariant truth
/// pillars: owner/origin, the data boundary, source/version/last-updated,
/// network/offline state, the browser fallback, account scope, freshness,
/// capability limits, and the no-embedded-high-risk-approval promise. The union
/// of every row's `preserved_label_families` must cover this set.
pub const REQUIRED_LABEL_FAMILIES: [&str; 9] = [
    "owner_origin",
    "data_boundary",
    "source_version_last_updated",
    "network_offline_state",
    "browser_fallback",
    "account_scope",
    "freshness",
    "capability_limits",
    "no_embedded_high_risk_approval",
];

/// The canonical boundary-disposition vocabulary every consumer keeps visible
/// even when narrowed or export-only — the frozen `M5EmbeddedBoundaryDisposition`
/// set (live-first-party-local / live-first-party-hosted / live-provider-owned /
/// stale-snapshot / offline-snapshot / provider-blocked / browser-handoff-only /
/// capability-limited / not-evaluated). Every consumer renders the same origin /
/// boundary state with these exact tokens rather than pane-local phrasing (AC1).
pub fn canonical_boundary_disposition_vocab() -> Vec<String> {
    M5EmbeddedBoundaryDisposition::ALL
        .iter()
        .map(|d| d.as_str().to_owned())
        .collect()
}

/// Whether a token is one of the frozen boundary-disposition tokens.
pub fn is_canonical_boundary_disposition(token: &str) -> bool {
    M5EmbeddedBoundaryDisposition::ALL
        .iter()
        .any(|d| d.as_str() == token)
}

/// The canonical per-family matrix schema that defines a family's contract.
pub fn canonical_family_schema_ref_for(family: M5EmbeddedBoundaryComponentFamily) -> &'static str {
    family.canonical_component_schema_ref()
}

/// The single primary boundary label family a component family must always
/// preserve — the boundary axis it exists to name. A consumer may narrow
/// authority, but it must never drop this label, so the family's core
/// owner/origin, data-boundary, freshness, account-scope, browser-fallback,
/// source, or offline-state truth is never silently lost.
pub const fn family_primary_label(family: M5EmbeddedBoundaryComponentFamily) -> &'static str {
    use M5EmbeddedBoundaryComponentFamily::*;
    match family {
        DocsPaneHeader => "source_version_last_updated",
        EmbeddedOriginBar => "owner_origin",
        BoundaryFactGrid => "data_boundary",
        MarketplaceAccountBoundaryCard => "account_scope",
        AuthHandoffCard => "browser_fallback",
        RemoteServiceDashboardHeader => "freshness",
        OpenInBrowserHandoffRow => "browser_fallback",
        EmbeddedStatePanel => "network_offline_state",
    }
}

/// The four B127 controls contracts the eight component families group into. A
/// consumer must point at the one canonical controls contract for its family's
/// lane rather than inventing a pane-local one — this is the heart of the
/// "embedded panes no longer fork boundary vocabulary" acceptance criterion.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5EmbeddedControlsLane {
    /// Docs-pane header + boundary-fact grid controls (M05-1070 lane 1).
    DocsBoundaryFacts,
    /// Embedded-origin bar + embedded-state panel controls (M05-1070 lane 2).
    OriginState,
    /// Marketplace/account boundary card + open-in-browser handoff row controls
    /// (M05-1072 marketplace lane).
    MarketplaceHandoff,
    /// Auth-handoff card + remote/service dashboard header controls (M05-1072
    /// auth lane).
    AuthDashboard,
}

impl M5EmbeddedControlsLane {
    /// Every controls lane, in declaration order.
    pub const ALL: [M5EmbeddedControlsLane; 4] = [
        M5EmbeddedControlsLane::DocsBoundaryFacts,
        M5EmbeddedControlsLane::OriginState,
        M5EmbeddedControlsLane::MarketplaceHandoff,
        M5EmbeddedControlsLane::AuthDashboard,
    ];

    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::DocsBoundaryFacts => "docs_boundary_facts",
            Self::OriginState => "origin_state",
            Self::MarketplaceHandoff => "marketplace_handoff",
            Self::AuthDashboard => "auth_dashboard",
        }
    }

    /// The canonical controls schema every surface reuses for this lane.
    pub const fn canonical_schema_ref(self) -> &'static str {
        match self {
            Self::DocsBoundaryFacts => docs_boundary_controls::M5_DOCS_BOUNDARY_CONTROLS_SCHEMA_REF,
            Self::OriginState => {
                origin_state_controls::M5_EMBEDDED_ORIGIN_STATE_CONTROLS_SCHEMA_REF
            }
            Self::MarketplaceHandoff => {
                marketplace_handoff_controls::M5_MARKETPLACE_HANDOFF_CONTROLS_SCHEMA_REF
            }
            Self::AuthDashboard => auth_dashboard_controls::M5_AUTH_DASHBOARD_CONTROLS_SCHEMA_REF,
        }
    }

    /// The canonical controls contract doc for this lane.
    pub const fn canonical_doc_ref(self) -> &'static str {
        match self {
            Self::DocsBoundaryFacts => docs_boundary_controls::M5_DOCS_BOUNDARY_CONTROLS_DOC_REF,
            Self::OriginState => origin_state_controls::M5_EMBEDDED_ORIGIN_STATE_CONTROLS_DOC_REF,
            Self::MarketplaceHandoff => {
                marketplace_handoff_controls::M5_MARKETPLACE_HANDOFF_CONTROLS_DOC_REF
            }
            Self::AuthDashboard => auth_dashboard_controls::M5_AUTH_DASHBOARD_CONTROLS_DOC_REF,
        }
    }

    /// The canonical controls release-proof artifact every consumer points back
    /// to as the first-resolved truth for this lane.
    pub const fn canonical_artifact_ref(self) -> &'static str {
        match self {
            Self::DocsBoundaryFacts => {
                docs_boundary_controls::M5_DOCS_BOUNDARY_CONTROLS_ARTIFACT_REF
            }
            Self::OriginState => {
                origin_state_controls::M5_EMBEDDED_ORIGIN_STATE_CONTROLS_ARTIFACT_REF
            }
            Self::MarketplaceHandoff => {
                marketplace_handoff_controls::M5_MARKETPLACE_HANDOFF_CONTROLS_ARTIFACT_REF
            }
            Self::AuthDashboard => auth_dashboard_controls::M5_AUTH_DASHBOARD_CONTROLS_ARTIFACT_REF,
        }
    }
}

/// The one controls lane a component family belongs to. The eight frozen
/// families group into the four B127 controls contracts; a consumer must reuse
/// the lane's canonical contract rather than forking it per surface.
pub const fn controls_lane_for(
    family: M5EmbeddedBoundaryComponentFamily,
) -> M5EmbeddedControlsLane {
    use M5EmbeddedBoundaryComponentFamily::*;
    match family {
        DocsPaneHeader | BoundaryFactGrid => M5EmbeddedControlsLane::DocsBoundaryFacts,
        EmbeddedOriginBar | EmbeddedStatePanel => M5EmbeddedControlsLane::OriginState,
        MarketplaceAccountBoundaryCard | OpenInBrowserHandoffRow => {
            M5EmbeddedControlsLane::MarketplaceHandoff
        }
        AuthHandoffCard | RemoteServiceDashboardHeader => M5EmbeddedControlsLane::AuthDashboard,
    }
}

/// The six claimed M5 embedded / browser-handoff consumer classes that must each
/// adopt at least one canonical component family.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ConsumerClass {
    /// A documentation / help pane.
    DocsHelpPane,
    /// A marketplace / account content surface.
    MarketplaceAccount,
    /// An extension-owned embedded-webview surface.
    EmbeddedWebview,
    /// A browser / device-code auth-handoff surface.
    AuthHandoff,
    /// A remote / service dashboard surface.
    RemoteServiceDashboard,
    /// A support / export + release-packet lane (AC2).
    SupportExportHelp,
}

impl ConsumerClass {
    /// Every consumer class that must be present for cross-surface reuse.
    pub const ALL: [ConsumerClass; 6] = [
        ConsumerClass::DocsHelpPane,
        ConsumerClass::MarketplaceAccount,
        ConsumerClass::EmbeddedWebview,
        ConsumerClass::AuthHandoff,
        ConsumerClass::RemoteServiceDashboard,
        ConsumerClass::SupportExportHelp,
    ];

    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::DocsHelpPane => "docs_help_pane",
            Self::MarketplaceAccount => "marketplace_account",
            Self::EmbeddedWebview => "embedded_webview",
            Self::AuthHandoff => "auth_handoff",
            Self::RemoteServiceDashboard => "remote_service_dashboard",
            Self::SupportExportHelp => "support_export_help",
        }
    }

    /// True when this class renders non-first-party-local (marketplace / account,
    /// extension-owned webview, provider auth handoff, or remote/service
    /// dashboard) content and therefore must never drop the adopted family's
    /// primary boundary label — the owner/origin, data-boundary, browser-fallback,
    /// account-scope, freshness, or offline-state truth that says whose content it
    /// is and why the flow crosses the browser or provider boundary.
    pub const fn is_boundary_crossing(self) -> bool {
        matches!(
            self,
            Self::MarketplaceAccount
                | Self::EmbeddedWebview
                | Self::AuthHandoff
                | Self::RemoteServiceDashboard
        )
    }
}

/// The consumer class a concrete matrix consumer surface belongs to. Reuses the
/// matrix's own [`M5EmbeddedConsumerSurface`] taxonomy rather than inventing a
/// parallel one.
pub const fn consumer_class_for(surface: M5EmbeddedConsumerSurface) -> ConsumerClass {
    use M5EmbeddedConsumerSurface::*;
    match surface {
        DocsBrowserUi => ConsumerClass::DocsHelpPane,
        MarketplaceUi | AccountUi => ConsumerClass::MarketplaceAccount,
        EmbeddedWebviewUi | ProductUi => ConsumerClass::EmbeddedWebview,
        AuthHandoffUi => ConsumerClass::AuthHandoff,
        RemoteDashboardUi => ConsumerClass::RemoteServiceDashboard,
        SupportExport => ConsumerClass::SupportExportHelp,
    }
}

/// True when this surface is the docs / help reference surface (AC2).
pub const fn is_docs_help_surface(surface: M5EmbeddedConsumerSurface) -> bool {
    matches!(surface, M5EmbeddedConsumerSurface::DocsBrowserUi)
}

/// True when this surface is the support / export + release-packet surface (AC2).
pub const fn is_support_export_surface(surface: M5EmbeddedConsumerSurface) -> bool {
    matches!(surface, M5EmbeddedConsumerSurface::SupportExport)
}

/// The rendering authority a consumer exercises over a canonical component.
///
/// A consumer may narrow authority (read-only, inspect-only, override-gated,
/// export-only, policy-blocked) but never rename or drop the governed boundary
/// truth.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AuthorityMode {
    /// Full-interactive control (act on the embedded component directly).
    FullInteractive,
    /// Read-only projection of the component.
    ReadOnly,
    /// Inspect-only: read every governed label but take no action.
    InspectOnly,
    /// Override-gated: the action is visible but staged behind an explicit gate
    /// (e.g. a browser step-up) before it applies.
    OverrideGated,
    /// Export-only: reconstruct the component from an export packet.
    ExportOnly,
    /// Policy-blocked: the component is visible but action is gated by policy.
    PolicyBlocked,
}

impl AuthorityMode {
    /// Every authority mode, in declaration order.
    pub const ALL: [AuthorityMode; 6] = [
        AuthorityMode::FullInteractive,
        AuthorityMode::ReadOnly,
        AuthorityMode::InspectOnly,
        AuthorityMode::OverrideGated,
        AuthorityMode::ExportOnly,
        AuthorityMode::PolicyBlocked,
    ];

    /// Returns true when the consumer narrows below full-interactive authority
    /// and therefore must disclose the reduction with a banner.
    pub const fn is_narrowed(self) -> bool {
        !matches!(self, Self::FullInteractive)
    }

    /// The banner `capability_state` label this authority maps to.
    pub const fn capability_state(self) -> &'static str {
        match self {
            Self::FullInteractive => "full",
            Self::ReadOnly => "read_only",
            Self::InspectOnly => "inspect_only",
            Self::OverrideGated => "override_gated",
            Self::ExportOnly => "export_only",
            Self::PolicyBlocked => "policy_blocked",
        }
    }
}

/// The surface a narrower consumer hands off to when it cannot act on the
/// embedded component locally.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum HandoffTarget {
    /// No handoff: the consumer renders and acts on the component in-place.
    None,
    /// Punt to the desktop shell to act on the boundary state.
    DesktopShell,
    /// Punt to the companion app.
    CompanionApp,
    /// Punt to a read-only browser surface (the browser fallback).
    BrowserReadonly,
    /// Punt to a portable support / export packet.
    SupportPacket,
    /// Punt to a headless CLI.
    CliHeadless,
}

impl HandoffTarget {
    /// Returns true when the consumer punts to another surface and therefore
    /// must carry a desktop / companion / browser / support note.
    pub const fn requires_note(self) -> bool {
        !matches!(self, Self::None)
    }

    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::None => "none",
            Self::DesktopShell => "desktop_shell",
            Self::CompanionApp => "companion_app",
            Self::BrowserReadonly => "browser_readonly",
            Self::SupportPacket => "support_packet",
            Self::CliHeadless => "cli_headless",
        }
    }
}

/// Whether the consumer preserves the canonical component's controlled labels.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LabelParityState {
    /// Full label parity across the boundary truth pillars.
    Preserved,
    /// Reduced interactivity, disclosed, but the labels are still preserved.
    DisclosedNarrowed,
    /// A label was renamed, flattened, or dropped (red; blocks review).
    RenamedOrDropped,
}

impl LabelParityState {
    /// Returns true when no controlled label is renamed or dropped.
    pub const fn keeps_labels(self) -> bool {
        !matches!(self, Self::RenamedOrDropped)
    }

    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Preserved => "preserved",
            Self::DisclosedNarrowed => "disclosed_narrowed",
            Self::RenamedOrDropped => "renamed_or_dropped",
        }
    }
}

/// The copy / export parity a consumer keeps for the adopted component: the
/// governed labels must be copyable as text / JSON / Markdown, and a
/// screenshot-only export is prohibited (it would lose the machine-readable
/// owner / origin / boundary / disposition identity support and automation need
/// to reconstruct the boundary state).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CopyExportParity {
    /// The copy formats the consumer offers (must include text / json /
    /// markdown).
    #[serde(default)]
    pub formats: Vec<String>,
    /// The export fields the consumer preserves.
    #[serde(default)]
    pub export_fields: Vec<String>,
    /// True when a screenshot-only export is prohibited.
    pub screenshot_only_prohibited: bool,
}

impl CopyExportParity {
    /// Whether the parity offers text / JSON / Markdown copy and prohibits a
    /// screenshot-only export.
    pub fn is_complete(&self) -> bool {
        let has = |f: &str| self.formats.iter().any(|v| v == f);
        has("text")
            && has("json")
            && has("markdown")
            && !self.export_fields.is_empty()
            && self.screenshot_only_prohibited
    }
}

/// The reduced-capability banner a narrower consumer shows to disclose the
/// control it drops relative to the full owner-origin-boundary component.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReducedCapabilityBanner {
    /// Stable banner id.
    pub banner_id: String,
    /// The visible, non-generic banner label.
    pub visible_label: String,
    /// The capability state; must match the row's `authority_mode`.
    pub capability_state: String,
    /// The capabilities the narrowed surface is missing relative to full.
    #[serde(default)]
    pub missing_capabilities: Vec<String>,
}

/// One consumer adopting one canonical embedded-boundary component family on one
/// M5 surface.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EmbeddedConsumerRow {
    /// Record kind; must equal [`EMBEDDED_CONSUMER_ROW_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`EMBEDDED_CONSUMER_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable row id.
    pub row_id: String,
    /// The claimed consumer class.
    pub consumer_class: ConsumerClass,
    /// The concrete surface; must belong to `consumer_class`.
    pub consumer_surface: M5EmbeddedConsumerSurface,
    /// The single canonical component family this consumer reuses.
    pub component_family: M5EmbeddedBoundaryComponentFamily,
    /// The controls lane the family belongs to; must equal
    /// `controls_lane_for(component_family)`.
    pub controls_lane: M5EmbeddedControlsLane,
    /// The canonical per-family matrix schema. Must equal
    /// `canonical_family_schema_ref_for(component_family)`.
    pub canonical_family_schema_ref: String,
    /// The canonical controls schema for the lane. Must equal
    /// `controls_lane.canonical_schema_ref()`.
    pub canonical_controls_schema_ref: String,
    /// The canonical controls release-proof artifact(s) this consumer points
    /// back to. Must contain `controls_lane.canonical_artifact_ref()`.
    #[serde(default)]
    pub canonical_controls_artifact_refs: Vec<String>,
    /// True when the consumer references the canonical family + controls lane
    /// rather than cloning pane-local embedded chrome.
    pub references_canonical_not_local_prose: bool,
    /// An opaque, redaction-safe ref to the origin / boundary state the user saw,
    /// so support and automation can reconstruct it without leaking raw provider
    /// tokens, credential material, or cookies.
    pub boundary_state_ref: String,
    /// The rendering authority the consumer exercises.
    pub authority_mode: AuthorityMode,
    /// The controlled label families the consumer preserves verbatim (subset of
    /// [`REQUIRED_LABEL_FAMILIES`]).
    #[serde(default)]
    pub preserved_label_families: Vec<String>,
    /// The frozen boundary-disposition vocabulary the consumer keeps visible even
    /// when narrowed.
    #[serde(default)]
    pub boundary_disposition_vocab: Vec<String>,
    /// Whether the consumer keeps the controlled labels.
    pub label_parity: LabelParityState,
    /// The surface a narrower consumer hands off to, if any.
    pub handoff_target: HandoffTarget,
    /// The desktop / companion / browser / support note ref; required when
    /// `handoff_target` is not `None`.
    #[serde(default)]
    pub handoff_note_ref: String,
    /// The reduced-capability banner, present only when the consumer narrows.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reduced_capability_banner: Option<ReducedCapabilityBanner>,
    /// The copy / export parity of the adopted component.
    pub copy_export: CopyExportParity,
    /// Guardrail: the consumer imitates native permission or irreversible
    /// approval UI. Must be false.
    pub imitates_native_permission_or_approval_ui: bool,
    /// Guardrail: the consumer hides owner/origin or browser fallback behind
    /// menus only. Must be false.
    pub hides_owner_origin_or_browser_fallback_in_menus_only: bool,
    /// Guardrail: the consumer renders a stale / offline / provider-blocked pane
    /// as fresh first-party local truth. Must be false.
    pub renders_stale_or_blocked_as_fresh_first_party_truth: bool,
    /// Guardrail: the consumer embeds a high-risk approval without a native
    /// step-up. Must be false.
    pub embeds_high_risk_approval_without_native_step_up: bool,
    /// Source contract refs backing this row.
    #[serde(default)]
    pub source_refs: Vec<String>,
    /// ISO 8601 UTC timestamp the adoption was observed.
    pub observed_at: String,
    /// Evidence packet refs backing this row.
    #[serde(default)]
    pub evidence_refs: Vec<String>,
}

impl EmbeddedConsumerRow {
    /// Returns true when the consumer narrows below full authority.
    pub fn is_narrowed(&self) -> bool {
        self.authority_mode.is_narrowed()
    }

    /// The surface's declared class matches the row's declared class.
    pub fn surface_class_consistent(&self) -> bool {
        consumer_class_for(self.consumer_surface) == self.consumer_class
    }

    /// AC (no fork): the consumer reuses the canonical controls contract for its
    /// family's lane rather than a pane-local one.
    pub fn controls_lane_is_canonical(&self) -> bool {
        self.controls_lane == controls_lane_for(self.component_family)
            && self.canonical_controls_schema_ref == self.controls_lane.canonical_schema_ref()
            && self
                .canonical_controls_artifact_refs
                .iter()
                .any(|r| r == self.controls_lane.canonical_artifact_ref())
    }

    /// AC1 (canonical): the consumer points back to exactly one canonical family
    /// — the declared matrix schema matches the family, a controls release-proof
    /// artifact is referenced, and no pane-local embedded chrome is cloned.
    pub fn points_to_canonical_family(&self) -> bool {
        self.canonical_family_schema_ref == canonical_family_schema_ref_for(self.component_family)
            && self.controls_lane_is_canonical()
            && self.references_canonical_not_local_prose
    }

    /// AC1 (parity): the consumer preserves the family's controlled label
    /// families and frozen boundary-disposition vocabulary rather than renaming
    /// or omitting them.
    pub fn preserves_labels(&self) -> bool {
        self.label_parity.keeps_labels()
            && !self.preserved_label_families.is_empty()
            && self
                .preserved_label_families
                .iter()
                .all(|f| REQUIRED_LABEL_FAMILIES.contains(&f.as_str()))
            && !self.boundary_disposition_vocab.is_empty()
            && self
                .boundary_disposition_vocab
                .iter()
                .all(|v| is_canonical_boundary_disposition(v))
    }

    /// AC (boundary truth): every row preserves the adopted family's primary
    /// boundary label, and a boundary-crossing consumer (marketplace / account,
    /// embedded webview, provider auth handoff, or remote/service dashboard) never
    /// drops it — so an embedded pane never hides whose content it renders or why
    /// the flow crosses the browser or provider boundary.
    pub fn preserves_primary_boundary_truth(&self) -> bool {
        let primary = family_primary_label(self.component_family);
        self.preserved_label_families.iter().any(|f| f == primary)
    }

    /// AC2: the row carries the opaque boundary-state ref and canonical controls
    /// contract support and automation reconstruct the seen state from.
    pub fn supports_state_reconstruction(&self) -> bool {
        !self.boundary_state_ref.trim().is_empty()
            && self.controls_lane_is_canonical()
            && self.copy_export.is_complete()
    }

    /// The four spec guardrails are all clear (false).
    pub fn guardrails_clear(&self) -> bool {
        self.first_failed_guardrail().is_none()
    }

    /// The first guardrail that is (wrongly) set, if any.
    pub fn first_failed_guardrail(&self) -> Option<&'static str> {
        if self.imitates_native_permission_or_approval_ui {
            Some("imitates_native_permission_or_approval_ui")
        } else if self.hides_owner_origin_or_browser_fallback_in_menus_only {
            Some("hides_owner_origin_or_browser_fallback_in_menus_only")
        } else if self.renders_stale_or_blocked_as_fresh_first_party_truth {
            Some("renders_stale_or_blocked_as_fresh_first_party_truth")
        } else if self.embeds_high_risk_approval_without_native_step_up {
            Some("embeds_high_risk_approval_without_native_step_up")
        } else {
            None
        }
    }

    /// AC (disclosure): a narrower consumer discloses the reduction with a
    /// reduced-capability banner whose state matches the authority mode, and
    /// carries a note whenever it punts to another surface.
    pub fn discloses_narrowing(&self) -> bool {
        if self.is_narrowed() {
            match &self.reduced_capability_banner {
                None => return false,
                Some(banner) => {
                    if banner.banner_id.trim().is_empty()
                        || banner.visible_label.trim().is_empty()
                        || label_is_generic(&banner.visible_label)
                        || banner.capability_state != self.authority_mode.capability_state()
                        || banner.capability_state == "full"
                        || banner.missing_capabilities.is_empty()
                    {
                        return false;
                    }
                }
            }
            // A narrowed consumer that keeps every label is disclosed-narrowed,
            // never plain preserved.
            if self.label_parity == LabelParityState::Preserved {
                return false;
            }
        } else if self.reduced_capability_banner.is_some() {
            // A full-interactive consumer must not carry a spurious banner.
            return false;
        }
        if self.handoff_target.requires_note() && self.handoff_note_ref.trim().is_empty() {
            return false;
        }
        true
    }

    /// Whether the row's identity and evidence fields are complete.
    pub fn is_complete(&self) -> bool {
        self.record_kind == EMBEDDED_CONSUMER_ROW_RECORD_KIND
            && self.schema_version == EMBEDDED_CONSUMER_SCHEMA_VERSION
            && !self.row_id.trim().is_empty()
            && !self.boundary_state_ref.trim().is_empty()
            && !self.canonical_family_schema_ref.trim().is_empty()
            && !self.canonical_controls_schema_ref.trim().is_empty()
            && !self.canonical_controls_artifact_refs.is_empty()
            && !self.observed_at.trim().is_empty()
            && !self.evidence_refs.is_empty()
            && self.evidence_refs.iter().all(|r| !r.trim().is_empty())
    }

    /// Deterministic governed chip line for this row.
    pub fn chip_tokens(&self) -> String {
        format!(
            "surface={surface} class={class} family={family} lane={lane} \
authority={authority} label_parity={label_parity} handoff={handoff}",
            surface = self.consumer_surface.as_str(),
            class = self.consumer_class.as_str(),
            family = self.component_family.as_str(),
            lane = self.controls_lane.as_str(),
            authority = self.authority_mode.capability_state(),
            label_parity = self.label_parity.as_str(),
            handoff = self.handoff_target.as_str(),
        )
    }
}

/// Rolled-up summary of an M05-1074 consumer packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EmbeddedConsumerSummary {
    pub row_count: usize,
    pub consumer_class_count: usize,
    pub consumer_surface_count: usize,
    pub component_family_count: usize,
    pub controls_lane_count: usize,
    pub boundary_disposition_count: usize,
    pub all_rows_point_to_canonical_family: bool,
    pub all_rows_preserve_labels: bool,
    pub all_rows_use_canonical_controls_lane: bool,
    pub all_boundary_rows_preserve_primary_truth: bool,
    pub all_rows_reconstructable: bool,
    pub all_narrowed_rows_disclose: bool,
    pub all_rows_have_copy_export: bool,
    pub all_rows_guardrails_clear: bool,
    pub controls_lanes_stable_across_surfaces: bool,
    pub docs_help_pane_consumer_present: bool,
    pub marketplace_account_consumer_present: bool,
    pub embedded_webview_consumer_present: bool,
    pub auth_handoff_consumer_present: bool,
    pub remote_service_dashboard_consumer_present: bool,
    pub support_export_help_consumer_present: bool,
    pub docs_help_reference_present: bool,
    pub support_export_reference_present: bool,
    pub label_family_coverage_complete: bool,
    pub boundary_disposition_coverage_complete: bool,
    pub families_reused_across_classes: usize,
}

/// Constructor input for [`EmbeddedConsumerPacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EmbeddedConsumerPacketInput {
    pub packet_id: String,
    pub as_of: String,
    pub matrix_ref: String,
    pub rows: Vec<EmbeddedConsumerRow>,
}

/// Checked-in M05-1074 consumer packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EmbeddedConsumerPacket {
    pub schema_version: u32,
    pub record_kind: String,
    pub packet_id: String,
    pub as_of: String,
    pub matrix_ref: String,
    #[serde(default)]
    pub rows: Vec<EmbeddedConsumerRow>,
    pub summary: EmbeddedConsumerSummary,
}

impl EmbeddedConsumerPacket {
    /// Builds a packet, stamping the record kind, schema version, and computed
    /// summary.
    pub fn new(input: EmbeddedConsumerPacketInput) -> Self {
        let mut packet = Self {
            schema_version: EMBEDDED_CONSUMER_SCHEMA_VERSION,
            record_kind: EMBEDDED_CONSUMER_RECORD_KIND.to_owned(),
            packet_id: input.packet_id,
            as_of: input.as_of,
            matrix_ref: input.matrix_ref,
            rows: input.rows,
            summary: EmbeddedConsumerSummary {
                row_count: 0,
                consumer_class_count: 0,
                consumer_surface_count: 0,
                component_family_count: 0,
                controls_lane_count: 0,
                boundary_disposition_count: 0,
                all_rows_point_to_canonical_family: false,
                all_rows_preserve_labels: false,
                all_rows_use_canonical_controls_lane: false,
                all_boundary_rows_preserve_primary_truth: false,
                all_rows_reconstructable: false,
                all_narrowed_rows_disclose: false,
                all_rows_have_copy_export: false,
                all_rows_guardrails_clear: false,
                controls_lanes_stable_across_surfaces: false,
                docs_help_pane_consumer_present: false,
                marketplace_account_consumer_present: false,
                embedded_webview_consumer_present: false,
                auth_handoff_consumer_present: false,
                remote_service_dashboard_consumer_present: false,
                support_export_help_consumer_present: false,
                docs_help_reference_present: false,
                support_export_reference_present: false,
                label_family_coverage_complete: false,
                boundary_disposition_coverage_complete: false,
                families_reused_across_classes: 0,
            },
        };
        packet.summary = packet.computed_summary();
        packet
    }

    /// Families represented by some row in this packet.
    pub fn represented_families(&self) -> BTreeSet<M5EmbeddedBoundaryComponentFamily> {
        self.rows.iter().map(|r| r.component_family).collect()
    }

    /// The union of every row's preserved label families.
    pub fn covered_label_families(&self) -> BTreeSet<String> {
        self.rows
            .iter()
            .flat_map(|r| r.preserved_label_families.iter().cloned())
            .collect()
    }

    /// The union of every row's boundary-disposition vocabulary.
    pub fn covered_boundary_dispositions(&self) -> BTreeSet<String> {
        self.rows
            .iter()
            .flat_map(|r| r.boundary_disposition_vocab.iter().cloned())
            .collect()
    }

    /// The count of component families adopted by two or more distinct consumer
    /// classes — the strongest evidence that a family is a reusable primitive.
    pub fn families_reused_across_classes(&self) -> usize {
        M5EmbeddedBoundaryComponentFamily::ALL
            .iter()
            .filter(|family| {
                let classes: BTreeSet<ConsumerClass> = self
                    .rows
                    .iter()
                    .filter(|r| r.component_family == **family)
                    .map(|r| r.consumer_class)
                    .collect();
                classes.len() >= 2
            })
            .count()
    }

    /// Whether every family maps to exactly one controls lane across every
    /// surface — no surface forks the lane by consumer.
    pub fn controls_lanes_stable_across_surfaces(&self) -> bool {
        let mut per_family: BTreeMap<
            M5EmbeddedBoundaryComponentFamily,
            BTreeSet<M5EmbeddedControlsLane>,
        > = BTreeMap::new();
        for row in &self.rows {
            per_family
                .entry(row.component_family)
                .or_default()
                .insert(row.controls_lane);
        }
        per_family.values().all(|lanes| lanes.len() <= 1)
    }

    /// Whether some docs / help surface references the canonical families (AC2).
    pub fn has_docs_help_reference(&self) -> bool {
        self.rows.iter().any(|r| {
            is_docs_help_surface(r.consumer_surface) && r.references_canonical_not_local_prose
        })
    }

    /// Whether some support / export surface references the canonical families —
    /// the release-packet half of AC2.
    pub fn has_support_export_reference(&self) -> bool {
        self.rows.iter().any(|r| {
            is_support_export_surface(r.consumer_surface) && r.references_canonical_not_local_prose
        })
    }

    /// Computes summary fields from the packet contents.
    pub fn computed_summary(&self) -> EmbeddedConsumerSummary {
        let mut classes = BTreeSet::new();
        let mut surfaces = BTreeSet::new();
        let mut families = BTreeSet::new();
        let mut lanes = BTreeSet::new();
        for row in &self.rows {
            classes.insert(row.consumer_class);
            surfaces.insert(row.consumer_surface);
            families.insert(row.component_family);
            lanes.insert(row.controls_lane);
        }

        let has_class = |c: ConsumerClass| classes.contains(&c);
        let covered = self.covered_label_families();
        let covered_dispositions = self.covered_boundary_dispositions();

        EmbeddedConsumerSummary {
            row_count: self.rows.len(),
            consumer_class_count: classes.len(),
            consumer_surface_count: surfaces.len(),
            component_family_count: families.len(),
            controls_lane_count: lanes.len(),
            boundary_disposition_count: covered_dispositions.len(),
            all_rows_point_to_canonical_family: self
                .rows
                .iter()
                .all(EmbeddedConsumerRow::points_to_canonical_family),
            all_rows_preserve_labels: self.rows.iter().all(EmbeddedConsumerRow::preserves_labels),
            all_rows_use_canonical_controls_lane: self
                .rows
                .iter()
                .all(EmbeddedConsumerRow::controls_lane_is_canonical),
            all_boundary_rows_preserve_primary_truth: self
                .rows
                .iter()
                .all(EmbeddedConsumerRow::preserves_primary_boundary_truth),
            all_rows_reconstructable: self
                .rows
                .iter()
                .all(EmbeddedConsumerRow::supports_state_reconstruction),
            all_narrowed_rows_disclose: self
                .rows
                .iter()
                .all(EmbeddedConsumerRow::discloses_narrowing),
            all_rows_have_copy_export: self.rows.iter().all(|r| r.copy_export.is_complete()),
            all_rows_guardrails_clear: self.rows.iter().all(EmbeddedConsumerRow::guardrails_clear),
            controls_lanes_stable_across_surfaces: self.controls_lanes_stable_across_surfaces(),
            docs_help_pane_consumer_present: has_class(ConsumerClass::DocsHelpPane),
            marketplace_account_consumer_present: has_class(ConsumerClass::MarketplaceAccount),
            embedded_webview_consumer_present: has_class(ConsumerClass::EmbeddedWebview),
            auth_handoff_consumer_present: has_class(ConsumerClass::AuthHandoff),
            remote_service_dashboard_consumer_present: has_class(
                ConsumerClass::RemoteServiceDashboard,
            ),
            support_export_help_consumer_present: has_class(ConsumerClass::SupportExportHelp),
            docs_help_reference_present: self.has_docs_help_reference(),
            support_export_reference_present: self.has_support_export_reference(),
            label_family_coverage_complete: REQUIRED_LABEL_FAMILIES
                .iter()
                .all(|f| covered.contains(*f)),
            boundary_disposition_coverage_complete: M5EmbeddedBoundaryDisposition::ALL
                .iter()
                .all(|d| covered_dispositions.contains(d.as_str())),
            families_reused_across_classes: self.families_reused_across_classes(),
        }
    }

    /// Validates the packet and returns every contract violation.
    pub fn validate(&self) -> Vec<EmbeddedConsumerViolation> {
        let mut violations = Vec::new();

        if self.schema_version != EMBEDDED_CONSUMER_SCHEMA_VERSION {
            violations.push(EmbeddedConsumerViolation::SchemaVersion {
                expected: EMBEDDED_CONSUMER_SCHEMA_VERSION,
                actual: self.schema_version,
            });
        }
        if self.record_kind != EMBEDDED_CONSUMER_RECORD_KIND {
            violations.push(EmbeddedConsumerViolation::RecordKind {
                expected: EMBEDDED_CONSUMER_RECORD_KIND.to_owned(),
                actual: self.record_kind.clone(),
            });
        }
        if self.packet_id.trim().is_empty()
            || self.as_of.trim().is_empty()
            || self.matrix_ref.trim().is_empty()
        {
            violations.push(EmbeddedConsumerViolation::MissingIdentity);
        }

        let mut row_ids = BTreeSet::new();
        let mut seen_classes = BTreeSet::new();
        for row in &self.rows {
            if !row_ids.insert(row.row_id.clone()) {
                violations.push(EmbeddedConsumerViolation::DuplicateId {
                    id: row.row_id.clone(),
                });
            }
            seen_classes.insert(row.consumer_class);

            if !row.is_complete() {
                violations.push(EmbeddedConsumerViolation::IncompleteRow {
                    id: row.row_id.clone(),
                });
            }

            // The concrete surface must belong to the declared consumer class.
            if !row.surface_class_consistent() {
                violations.push(EmbeddedConsumerViolation::SurfaceClassMismatch {
                    id: row.row_id.clone(),
                });
            }

            // AC1: exactly one canonical family, no cloned pane-local chrome.
            if !row.points_to_canonical_family() {
                violations.push(EmbeddedConsumerViolation::NotCanonicalFamily {
                    id: row.row_id.clone(),
                });
            }

            // AC (no fork): canonical controls lane per family.
            if !row.controls_lane_is_canonical() {
                violations.push(EmbeddedConsumerViolation::NonCanonicalControlsLane {
                    id: row.row_id.clone(),
                });
            }

            // AC1: controlled label families / boundary-disposition vocab preserved.
            if !row.preserves_labels() {
                violations.push(EmbeddedConsumerViolation::LabelParityBroken {
                    id: row.row_id.clone(),
                });
            }

            // AC (boundary truth): the family's primary boundary label is kept.
            if !row.preserves_primary_boundary_truth() {
                violations.push(EmbeddedConsumerViolation::PrimaryBoundaryTruthDropped {
                    id: row.row_id.clone(),
                });
            }

            // AC2: boundary state is reconstructable from the opaque ref +
            // canonical controls contract.
            if !row.supports_state_reconstruction() {
                violations.push(EmbeddedConsumerViolation::StateNotReconstructable {
                    id: row.row_id.clone(),
                });
            }

            // Disclosure: narrower consumers disclose reduction with banner + note.
            if !row.discloses_narrowing() {
                violations.push(EmbeddedConsumerViolation::NarrowedWithoutDisclosure {
                    id: row.row_id.clone(),
                });
            }

            // Copy / export parity: text / JSON / Markdown, screenshot prohibited.
            if !row.copy_export.is_complete() {
                violations.push(EmbeddedConsumerViolation::MissingCopyExportParity {
                    id: row.row_id.clone(),
                });
            }

            // Spec guardrails must all stay false.
            if let Some(guardrail) = row.first_failed_guardrail() {
                violations.push(EmbeddedConsumerViolation::GuardrailViolated {
                    id: row.row_id.clone(),
                    guardrail,
                });
            }
        }

        // Cross-surface reuse spans all six claimed consumer classes.
        for class in ConsumerClass::ALL {
            if !seen_classes.contains(&class) {
                violations.push(EmbeddedConsumerViolation::MissingConsumerClass { class });
            }
        }

        // Every frozen family is adopted by at least one consumer.
        let families = self.represented_families();
        for family in M5EmbeddedBoundaryComponentFamily::ALL {
            if !families.contains(&family) {
                violations.push(EmbeddedConsumerViolation::MissingFamilyCoverage { family });
            }
        }

        // AC1: at least one family is reused across two or more consumer classes
        // so multiple M5 surfaces point back to one canonical family.
        if self.families_reused_across_classes() == 0 {
            violations.push(EmbeddedConsumerViolation::NoFamilyReusedAcrossClasses);
        }

        // AC (no fork): families resolve to one stable controls lane per family.
        if !self.controls_lanes_stable_across_surfaces() {
            violations.push(EmbeddedConsumerViolation::ControlsLaneForkedAcrossSurfaces);
        }

        // AC1: the controlled label families are collectively preserved.
        let covered = self.covered_label_families();
        for family in REQUIRED_LABEL_FAMILIES {
            if !covered.contains(family) {
                violations.push(EmbeddedConsumerViolation::MissingLabelFamily {
                    family: family.to_owned(),
                });
            }
        }

        // AC1: the frozen boundary-disposition vocabulary is collectively preserved.
        let covered_dispositions = self.covered_boundary_dispositions();
        for disposition in M5EmbeddedBoundaryDisposition::ALL {
            if !covered_dispositions.contains(disposition.as_str()) {
                violations.push(EmbeddedConsumerViolation::MissingBoundaryDisposition {
                    disposition: disposition.as_str().to_owned(),
                });
            }
        }

        // AC2: a docs / help consumer references the canonical components rather
        // than cloning local embedded chrome.
        if !self.has_docs_help_reference() {
            violations.push(EmbeddedConsumerViolation::MissingDocsHelpReference);
        }

        // AC2: a support / export + release-packet consumer references the
        // canonical components so release packets drop bespoke per-pane prose.
        if !self.has_support_export_reference() {
            violations.push(EmbeddedConsumerViolation::MissingSupportExportReference);
        }

        if self.summary != self.computed_summary() {
            violations.push(EmbeddedConsumerViolation::SummaryMismatch);
        }

        if json_contains_forbidden_material(
            &serde_json::to_value(self).expect("consumer packet serializes"),
        ) {
            violations.push(EmbeddedConsumerViolation::RawBoundaryMaterialInExport);
        }

        violations
    }

    /// Deterministic export-safe JSON.
    ///
    /// # Panics
    ///
    /// Panics only if serializing this metadata-only packet fails.
    pub fn export_safe_json(&self) -> String {
        serde_json::to_string_pretty(self).expect("consumer packet serializes")
    }

    /// Deterministic CSV of the adoption rows for release / support handoff.
    pub fn render_matrix_csv(&self) -> String {
        let mut out = String::from(
            "row_id,consumer_class,consumer_surface,component_family,controls_lane,authority,label_parity,handoff\n",
        );
        for row in &self.rows {
            out.push_str(&format!(
                "{id},{class},{surface},{family},{lane},{authority},{label_parity},{handoff}\n",
                id = row.row_id,
                class = row.consumer_class.as_str(),
                surface = row.consumer_surface.as_str(),
                family = row.component_family.as_str(),
                lane = row.controls_lane.as_str(),
                authority = row.authority_mode.capability_state(),
                label_parity = row.label_parity.as_str(),
                handoff = row.handoff_target.as_str(),
            ));
        }
        out
    }

    /// Deterministic Markdown report for support, docs, or release handoff.
    pub fn render_markdown_summary(&self) -> String {
        let mut out = String::new();
        out.push_str("# M5 Embedded-Boundary Component Consumers\n\n");
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- As of: `{}`\n", self.as_of));
        out.push_str(&format!(
            "- Rows: {} across {} consumer classes and {} / {} frozen families\n",
            self.summary.row_count,
            self.summary.consumer_class_count,
            self.represented_families().len(),
            M5EmbeddedBoundaryComponentFamily::ALL.len(),
        ));
        out.push_str(&format!(
            "- Controls lanes adopted: {} / {}\n",
            self.summary.controls_lane_count,
            M5EmbeddedControlsLane::ALL.len(),
        ));
        out.push_str(&format!(
            "- Boundary dispositions preserved: {} / {}\n",
            self.summary.boundary_disposition_count,
            M5EmbeddedBoundaryDisposition::ALL.len(),
        ));
        out.push_str(&format!(
            "- Families reused across classes: {}\n",
            self.summary.families_reused_across_classes,
        ));
        out.push_str("\n## Rows\n\n");
        for row in &self.rows {
            out.push_str(&format!("- **{}** — {}\n", row.row_id, row.chip_tokens()));
        }
        out
    }
}

/// Reads and validates the checked-in consumer export.
pub fn current_m5_embedded_boundary_component_consumers_export(
) -> Result<EmbeddedConsumerPacket, EmbeddedConsumerArtifactError> {
    let packet: EmbeddedConsumerPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/release/m5-embedded-boundary-component-consumer-proof/support_export.json"
    )))
    .map_err(EmbeddedConsumerArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(EmbeddedConsumerArtifactError::Validation(violations))
    }
}

/// Errors emitted when reading the checked-in consumer export.
#[derive(Debug)]
pub enum EmbeddedConsumerArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<EmbeddedConsumerViolation>),
}

impl fmt::Display for EmbeddedConsumerArtifactError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(f, "consumer export parse failed: {error}")
            }
            Self::Validation(violations) => {
                write!(
                    f,
                    "consumer export failed validation: {} violation(s)",
                    violations.len()
                )
            }
        }
    }
}

impl Error for EmbeddedConsumerArtifactError {}

/// Validation failure for M05-1074 consumer packets.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EmbeddedConsumerViolation {
    SchemaVersion {
        expected: u32,
        actual: u32,
    },
    RecordKind {
        expected: String,
        actual: String,
    },
    MissingIdentity,
    DuplicateId {
        id: String,
    },
    IncompleteRow {
        id: String,
    },
    SurfaceClassMismatch {
        id: String,
    },
    NotCanonicalFamily {
        id: String,
    },
    NonCanonicalControlsLane {
        id: String,
    },
    LabelParityBroken {
        id: String,
    },
    PrimaryBoundaryTruthDropped {
        id: String,
    },
    StateNotReconstructable {
        id: String,
    },
    NarrowedWithoutDisclosure {
        id: String,
    },
    MissingCopyExportParity {
        id: String,
    },
    GuardrailViolated {
        id: String,
        guardrail: &'static str,
    },
    MissingConsumerClass {
        class: ConsumerClass,
    },
    MissingFamilyCoverage {
        family: M5EmbeddedBoundaryComponentFamily,
    },
    NoFamilyReusedAcrossClasses,
    ControlsLaneForkedAcrossSurfaces,
    MissingLabelFamily {
        family: String,
    },
    MissingBoundaryDisposition {
        disposition: String,
    },
    MissingDocsHelpReference,
    MissingSupportExportReference,
    SummaryMismatch,
    RawBoundaryMaterialInExport,
}

impl fmt::Display for EmbeddedConsumerViolation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SchemaVersion { expected, actual } => {
                write!(
                    f,
                    "schema version mismatch: expected {expected}, got {actual}"
                )
            }
            Self::RecordKind { expected, actual } => {
                write!(f, "record kind mismatch: expected {expected}, got {actual}")
            }
            Self::MissingIdentity => write!(f, "packet identity fields are missing"),
            Self::DuplicateId { id } => write!(f, "duplicate row id: {id}"),
            Self::IncompleteRow { id } => write!(f, "incomplete consumer row: {id}"),
            Self::SurfaceClassMismatch { id } => {
                write!(
                    f,
                    "row {id} declares a surface that does not belong to its consumer class"
                )
            }
            Self::NotCanonicalFamily { id } => {
                write!(
                    f,
                    "row {id} does not point back to exactly one canonical component family"
                )
            }
            Self::NonCanonicalControlsLane { id } => {
                write!(
                    f,
                    "row {id} forks the controls lane instead of reusing the canonical contract"
                )
            }
            Self::LabelParityBroken { id } => {
                write!(
                    f,
                    "row {id} renames or drops a canonical owner/origin, data-boundary, source, \
network/offline, browser-fallback, account-scope, freshness, capability-limit, or \
no-embedded-high-risk-approval label"
                )
            }
            Self::PrimaryBoundaryTruthDropped { id } => {
                write!(
                    f,
                    "row {id} drops the adopted family's primary boundary label (owner/origin, \
data boundary, freshness, account scope, browser fallback, source, or offline state)"
                )
            }
            Self::StateNotReconstructable { id } => {
                write!(
                    f,
                    "row {id} cannot be reconstructed from its boundary-state ref and controls contract"
                )
            }
            Self::NarrowedWithoutDisclosure { id } => {
                write!(
                    f,
                    "row {id} narrows authority without a reduced-capability banner or handoff note"
                )
            }
            Self::MissingCopyExportParity { id } => {
                write!(
                    f,
                    "row {id} is missing text / JSON / Markdown copy-export parity"
                )
            }
            Self::GuardrailViolated { id, guardrail } => {
                write!(f, "row {id} violates guardrail {guardrail}")
            }
            Self::MissingConsumerClass { class } => {
                write!(f, "consumer class {class:?} is not adopted in the packet")
            }
            Self::MissingFamilyCoverage { family } => {
                write!(
                    f,
                    "component family {family:?} is not adopted in the packet"
                )
            }
            Self::NoFamilyReusedAcrossClasses => write!(
                f,
                "no component family is adopted across two or more consumer classes"
            ),
            Self::ControlsLaneForkedAcrossSurfaces => write!(
                f,
                "a component family resolves to more than one controls lane across surfaces"
            ),
            Self::MissingLabelFamily { family } => {
                write!(
                    f,
                    "controlled label family {family} is not preserved anywhere"
                )
            }
            Self::MissingBoundaryDisposition { disposition } => {
                write!(
                    f,
                    "boundary-disposition token {disposition} is not preserved anywhere"
                )
            }
            Self::MissingDocsHelpReference => write!(
                f,
                "no docs / help consumer references the canonical component families"
            ),
            Self::MissingSupportExportReference => write!(
                f,
                "no support / export consumer references the canonical component families"
            ),
            Self::SummaryMismatch => write!(f, "computed summary does not match stored summary"),
            Self::RawBoundaryMaterialInExport => {
                write!(f, "export contains raw boundary material")
            }
        }
    }
}

impl Error for EmbeddedConsumerViolation {}

/// Whether a banner label is a generic non-answer rather than a precise label.
/// Adds the embedded / browser-handoff generic phrasings the spec forbids
/// collapsing into (offline, stale, blocked, loading, embedded content) to the
/// shared generic-label blocklist. These are matched as *whole* labels rather
/// than substrings so a descriptive banner may still name "provider offline
/// snapshot" or "extension-owned webview" as a boundary state without being
/// flagged; only a banner whose entire label collapses to the generic phrase is
/// rejected.
fn label_is_generic(label: &str) -> bool {
    let trimmed = label.trim();
    if trimmed.is_empty() {
        return true;
    }
    let lower = trimmed.to_lowercase();
    if lower.contains("get started") {
        return true;
    }
    matches!(
        lower.as_str(),
        "unsupported"
            | "not supported"
            | "unavailable"
            | "not available"
            | "n/a"
            | "error"
            | "failed"
            | "degraded"
            | "narrowed"
            | "fallback"
            | "reduced"
            | "read only"
            | "read-only"
            | "offline"
            | "stale"
            | "blocked"
            | "loading"
            | "content"
            | "embedded"
            | "webview"
            | "provider"
    )
}

/// Heuristic that rejects obviously forbidden material in export-safe JSON.
fn json_contains_forbidden_material(value: &serde_json::Value) -> bool {
    match value {
        serde_json::Value::String(s) => {
            let lower = s.to_lowercase();
            lower.contains("api_key")
                || lower.contains("password")
                || lower.contains("secret")
                || lower.contains("-----begin")
                || lower.contains("bearer ")
        }
        serde_json::Value::Array(arr) => arr.iter().any(json_contains_forbidden_material),
        serde_json::Value::Object(map) => map.values().any(json_contains_forbidden_material),
        _ => false,
    }
}

/// Builds the canonical, checked-in consumer packet. This is the one source of
/// truth shared by the tests and the on-disk support export so both stay
/// byte-aligned.
pub fn seeded_m5_embedded_boundary_component_consumers_packet() -> EmbeddedConsumerPacket {
    EmbeddedConsumerPacket::new(EmbeddedConsumerPacketInput {
        packet_id: "m5-embedded-boundary-component-consumers:stable:0001".to_owned(),
        as_of: "2026-07-10T00:00:00Z".to_owned(),
        matrix_ref: EMBEDDED_CONSUMER_MATRIX_REF.to_owned(),
        rows: seeded_rows(),
    })
}

fn ev(id: &str) -> Vec<String> {
    vec![format!("evidence:embedded-boundary-consumer:{id}")]
}

fn copy_export(fields: &[&str]) -> CopyExportParity {
    CopyExportParity {
        formats: vec!["text".to_owned(), "json".to_owned(), "markdown".to_owned()],
        export_fields: fields.iter().map(|f| (*f).to_owned()).collect(),
        screenshot_only_prohibited: true,
    }
}

fn labels(families: &[&str]) -> Vec<String> {
    families.iter().map(|f| (*f).to_owned()).collect()
}

fn banner(
    id: &str,
    label: &str,
    authority: AuthorityMode,
    missing: &[&str],
) -> ReducedCapabilityBanner {
    ReducedCapabilityBanner {
        banner_id: id.to_owned(),
        visible_label: label.to_owned(),
        capability_state: authority.capability_state().to_owned(),
        missing_capabilities: missing.iter().map(|m| (*m).to_owned()).collect(),
    }
}

#[allow(clippy::too_many_arguments)]
fn row(
    row_id: &str,
    consumer_surface: M5EmbeddedConsumerSurface,
    component_family: M5EmbeddedBoundaryComponentFamily,
    authority_mode: AuthorityMode,
    label_families: &[&str],
    export_fields: &[&str],
    handoff_target: HandoffTarget,
    handoff_note_ref: &str,
    reduced_capability_banner: Option<ReducedCapabilityBanner>,
) -> EmbeddedConsumerRow {
    let label_parity = if authority_mode.is_narrowed() {
        LabelParityState::DisclosedNarrowed
    } else {
        LabelParityState::Preserved
    };
    let controls_lane = controls_lane_for(component_family);
    EmbeddedConsumerRow {
        record_kind: EMBEDDED_CONSUMER_ROW_RECORD_KIND.to_owned(),
        schema_version: EMBEDDED_CONSUMER_SCHEMA_VERSION,
        row_id: row_id.to_owned(),
        consumer_class: consumer_class_for(consumer_surface),
        consumer_surface,
        component_family,
        controls_lane,
        canonical_family_schema_ref: canonical_family_schema_ref_for(component_family).to_owned(),
        canonical_controls_schema_ref: controls_lane.canonical_schema_ref().to_owned(),
        canonical_controls_artifact_refs: vec![controls_lane.canonical_artifact_ref().to_owned()],
        references_canonical_not_local_prose: true,
        boundary_state_ref: format!("boundary-state:{row_id}"),
        authority_mode,
        preserved_label_families: labels(label_families),
        boundary_disposition_vocab: canonical_boundary_disposition_vocab(),
        label_parity,
        handoff_target,
        handoff_note_ref: handoff_note_ref.to_owned(),
        reduced_capability_banner,
        copy_export: copy_export(export_fields),
        imitates_native_permission_or_approval_ui: false,
        hides_owner_origin_or_browser_fallback_in_menus_only: false,
        renders_stale_or_blocked_as_fresh_first_party_truth: false,
        embeds_high_risk_approval_without_native_step_up: false,
        source_refs: vec![
            EMBEDDED_CONSUMER_MATRIX_REF.to_owned(),
            EMBEDDED_CONSUMER_SHARED_SCHEMA_REF.to_owned(),
            controls_lane.canonical_doc_ref().to_owned(),
        ],
        observed_at: "2026-07-10T00:00:00Z".to_owned(),
        evidence_refs: ev(row_id),
    }
}

fn seeded_rows() -> Vec<EmbeddedConsumerRow> {
    use AuthorityMode::*;
    use HandoffTarget as H;
    use M5EmbeddedBoundaryComponentFamily::*;
    use M5EmbeddedConsumerSurface::*;

    vec![
        // --- Docs / help pane ----------------------------------------------
        row(
            "consumer:docs-help:docs-pane-header",
            DocsBrowserUi,
            DocsPaneHeader,
            FullInteractive,
            &["source_version_last_updated", "owner_origin", "freshness"],
            &[
                "source_version_last_updated",
                "owner_origin",
                "freshness",
                "controls_lane",
            ],
            H::None,
            "",
            None,
        ),
        row(
            "consumer:docs-help:boundary-fact-grid",
            DocsBrowserUi,
            BoundaryFactGrid,
            ReadOnly,
            &["data_boundary", "owner_origin", "freshness"],
            &[
                "data_boundary",
                "owner_origin",
                "freshness",
                "controls_lane",
            ],
            H::None,
            "",
            Some(banner(
                "banner:docs-help:boundary-fact-grid",
                "Read-only docs boundary facts: names the owner/origin, the data boundary, and the freshness of the docs shown; opening or refreshing stays in the desktop shell",
                ReadOnly,
                &["open_externally", "refresh_docs_source"],
            )),
        ),
        // --- Marketplace / account content ---------------------------------
        row(
            "consumer:marketplace:account-boundary-card",
            MarketplaceUi,
            MarketplaceAccountBoundaryCard,
            FullInteractive,
            &["account_scope", "owner_origin", "data_boundary"],
            &[
                "account_scope",
                "owner_origin",
                "data_boundary",
                "controls_lane",
            ],
            H::None,
            "",
            None,
        ),
        row(
            "consumer:account:account-boundary-card",
            AccountUi,
            MarketplaceAccountBoundaryCard,
            ReadOnly,
            &["account_scope", "owner_origin", "data_boundary"],
            &[
                "account_scope",
                "owner_origin",
                "data_boundary",
                "controls_lane",
            ],
            H::None,
            "",
            Some(banner(
                "banner:account:account-boundary-card",
                "Read-only account boundary card: names the current profile, account scope, and owner/origin behind the account pane; changing account scope stays in the desktop shell",
                ReadOnly,
                &["switch_account_scope", "manage_profile"],
            )),
        ),
        row(
            "consumer:marketplace:open-in-browser-row",
            MarketplaceUi,
            OpenInBrowserHandoffRow,
            ReadOnly,
            &["browser_fallback", "owner_origin", "data_boundary"],
            &[
                "browser_fallback",
                "owner_origin",
                "data_boundary",
                "controls_lane",
            ],
            H::BrowserReadonly,
            "handoff:marketplace:open-in-browser-row-browser",
            Some(banner(
                "banner:marketplace:open-in-browser-row",
                "Read-only open-in-browser row: names the browser fallback and the data boundary a marketplace listing crosses; completing the action opens the external browser",
                ReadOnly,
                &["complete_in_app", "purchase_in_app"],
            )),
        ),
        // --- Extension-owned embedded webview ------------------------------
        row(
            "consumer:embedded-webview:origin-bar",
            EmbeddedWebviewUi,
            EmbeddedOriginBar,
            ReadOnly,
            &["owner_origin", "capability_limits", "data_boundary"],
            &[
                "owner_origin",
                "capability_limits",
                "data_boundary",
                "controls_lane",
            ],
            H::None,
            "",
            Some(banner(
                "banner:embedded-webview:origin-bar",
                "Read-only embedded-origin bar: names the extension, publisher, and origin that own this webview and its capability limits; native permission changes stay in the desktop shell",
                ReadOnly,
                &["grant_native_permission", "change_capability"],
            )),
        ),
        row(
            "consumer:embedded-webview:state-panel",
            EmbeddedWebviewUi,
            EmbeddedStatePanel,
            InspectOnly,
            &[
                "network_offline_state",
                "capability_limits",
                "freshness",
                "owner_origin",
            ],
            &[
                "network_offline_state",
                "capability_limits",
                "freshness",
                "controls_lane",
            ],
            H::None,
            "",
            Some(banner(
                "banner:embedded-webview:state-panel",
                "Inspect-only embedded-state panel: names whether the webview is stale, offline, policy-blocked, or cross-origin-limited and its capability limits; it never imitates native permission chrome",
                InspectOnly,
                &["reload_webview", "grant_native_permission"],
            )),
        ),
        row(
            "consumer:product:embedded-origin-bar",
            ProductUi,
            EmbeddedOriginBar,
            ReadOnly,
            &["owner_origin", "capability_limits", "data_boundary"],
            &[
                "owner_origin",
                "capability_limits",
                "data_boundary",
                "controls_lane",
            ],
            H::None,
            "",
            Some(banner(
                "banner:product:embedded-origin-bar",
                "Read-only embedded-origin bar in product chrome: names the owner/origin and capability limits of a contributed webview so it never reads as first-party native chrome",
                ReadOnly,
                &["grant_native_permission", "change_capability"],
            )),
        ),
        // --- Browser / device-code auth handoff ----------------------------
        row(
            "consumer:auth-handoff:auth-card",
            AuthHandoffUi,
            AuthHandoffCard,
            OverrideGated,
            &[
                "browser_fallback",
                "account_scope",
                "no_embedded_high_risk_approval",
                "data_boundary",
            ],
            &[
                "browser_fallback",
                "account_scope",
                "no_embedded_high_risk_approval",
                "controls_lane",
            ],
            H::BrowserReadonly,
            "handoff:auth-handoff:auth-card-browser",
            Some(banner(
                "banner:auth-handoff:auth-card",
                "Browser-gated auth-handoff card: names the provider/domain, the reason for handoff, the device-code expiry, and which local state stays intact; the actual sign-in approval happens in the browser, never embedded",
                OverrideGated,
                &["approve_sign_in_embedded", "enter_credential_embedded"],
            )),
        ),
        row(
            "consumer:auth-handoff:open-in-browser-row",
            AuthHandoffUi,
            OpenInBrowserHandoffRow,
            ReadOnly,
            &["browser_fallback", "data_boundary", "owner_origin"],
            &[
                "browser_fallback",
                "data_boundary",
                "owner_origin",
                "controls_lane",
            ],
            H::BrowserReadonly,
            "handoff:auth-handoff:open-in-browser-row-browser",
            Some(banner(
                "banner:auth-handoff:open-in-browser-row",
                "Read-only open-in-browser row: names the browser fallback and the data boundary the sign-in crosses; the handoff lands on the provider's own page, not a generic landing page",
                ReadOnly,
                &["complete_sign_in_embedded"],
            )),
        ),
        // --- Remote / service dashboard ------------------------------------
        row(
            "consumer:remote-dashboard:dashboard-header",
            RemoteDashboardUi,
            RemoteServiceDashboardHeader,
            ReadOnly,
            &["freshness", "owner_origin", "network_offline_state"],
            &[
                "freshness",
                "owner_origin",
                "network_offline_state",
                "controls_lane",
            ],
            H::None,
            "",
            Some(banner(
                "banner:remote-dashboard:dashboard-header",
                "Read-only remote/service dashboard header: names the target/service identity, provider health, freshness, and offline state; it never substitutes for local recovery",
                ReadOnly,
                &["open_service_console", "act_on_service"],
            )),
        ),
        row(
            "consumer:remote-dashboard:state-panel",
            RemoteDashboardUi,
            EmbeddedStatePanel,
            ReadOnly,
            &["network_offline_state", "capability_limits", "owner_origin"],
            &[
                "network_offline_state",
                "capability_limits",
                "owner_origin",
                "controls_lane",
            ],
            H::None,
            "",
            Some(banner(
                "banner:remote-dashboard:state-panel",
                "Read-only embedded-state panel: names whether the dashboard is stale, offline, or provider-blocked and its capability limits, so a blocked pane never reads as fresh first-party local truth",
                ReadOnly,
                &["reload_dashboard", "act_on_service"],
            )),
        ),
        // --- Support / export + release packet (AC2) -----------------------
        row(
            "consumer:support-export:docs-pane-header",
            SupportExport,
            DocsPaneHeader,
            ExportOnly,
            &[
                "source_version_last_updated",
                "freshness",
                "owner_origin",
            ],
            &[
                "source_version_last_updated",
                "freshness",
                "owner_origin",
                "boundary_state_ref",
                "controls_lane",
            ],
            H::SupportPacket,
            "handoff:support-export:docs-pane-header-support-packet",
            Some(banner(
                "banner:support-export:docs-pane-header",
                "Export-only support replay: reconstruct the docs source class, version, last-updated, owner/origin, and freshness the user saw from the support packet",
                ExportOnly,
                &["open_externally", "refresh_docs_source"],
            )),
        ),
        row(
            "consumer:support-export:remote-dashboard-header",
            SupportExport,
            RemoteServiceDashboardHeader,
            ExportOnly,
            &["freshness", "owner_origin", "network_offline_state"],
            &[
                "freshness",
                "owner_origin",
                "network_offline_state",
                "boundary_state_ref",
                "controls_lane",
            ],
            H::SupportPacket,
            "handoff:support-export:remote-dashboard-header-support-packet",
            Some(banner(
                "banner:support-export:remote-dashboard-header",
                "Export-only support replay: reconstruct the target/service identity, provider health, freshness, and offline state the dashboard header showed from the support packet",
                ExportOnly,
                &["open_service_console", "act_on_service"],
            )),
        ),
        row(
            "consumer:support-export:auth-card",
            SupportExport,
            AuthHandoffCard,
            ExportOnly,
            &[
                "browser_fallback",
                "no_embedded_high_risk_approval",
                "account_scope",
            ],
            &[
                "browser_fallback",
                "no_embedded_high_risk_approval",
                "account_scope",
                "boundary_state_ref",
                "controls_lane",
            ],
            H::SupportPacket,
            "handoff:support-export:auth-card-support-packet",
            Some(banner(
                "banner:support-export:auth-card",
                "Export-only support replay: reconstruct the provider/domain, the browser fallback, the account scope, and that approval was never embedded from the support packet",
                ExportOnly,
                &["approve_sign_in_embedded", "enter_credential_embedded"],
            )),
        ),
    ]
}
