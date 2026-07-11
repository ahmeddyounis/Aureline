//! Keyboard / screen-reader / reduced-motion / high-contrast / CLI / export /
//! support-packet parity and honest auto-narrowing for the M5 embedded-boundary
//! components.
//!
//! This module is the M05-1073 accessibility-and-auto-narrowing capstone over the
//! frozen M5 embedded-boundary component matrix
//! ([`crate::freeze_the_m5_docs_pane_header_embedded_origin_bar_boundary_fact_grid_marketplace_account_boundary_card_auth_handoff_card_remote_service_dashboard_header_open_in_browser_handoff_row_and_embedded_state_panel_component_matrix`]).
//! Where the freeze matrix defines the reusable docs-pane header, embedded-origin bar,
//! boundary-fact grid, marketplace/account boundary card, auth-handoff card,
//! remote/service dashboard header, open-in-browser handoff row, and embedded-state
//! panel primitives, and the sibling implementation lanes resolve their per-surface
//! truth, this lane certifies — per component family — that owner/origin, data-boundary,
//! browser-fallback, capability-limit, account-scope, and freshness truth stays
//! **keyboard-complete, screen-reader-reachable, reduced-motion safe, high-contrast
//! legible, CLI/export-safe, and self-narrowing** rather than presenting a stale,
//! offline, provider-blocked, or partial boundary state as still fresh first-party
//! `full-truth`:
//!
//! - **Keyboard / screen-reader / CLI reach.** Every family exposes a
//!   keyboard-complete, screen-reader-reachable, and CLI/headless-reachable path into
//!   the same owner/origin, data boundary, browser fallback, capability limits, account
//!   scope, and freshness the rich embedded surface shows — never a hover-only or
//!   menu-only chrome that strands assistive-tech or headless users. Hierarchy-heavy
//!   families (the boundary-fact grid's nested owner/origin / data-boundary / freshness
//!   grid) additionally bind their structured layout to a flat list / textual path.
//! - **Export parity.** The support / release export reconstructs each component's
//!   meaning from typed tokens and opaque refs without a screenshot, preserving the same
//!   owner/origin, data boundary, browser fallback, capability limits, account scope, and
//!   freshness shown in-product.
//! - **Honest auto-narrowing.** When the owner/origin, data boundary, browser fallback,
//!   capability limit, account scope, or freshness becomes partial, stale, offline, or
//!   provider-blocked, the component's boundary-support claim auto-narrows from
//!   `full-truth` / `resolved-truth` to degraded / stale / offline / provider-blocked,
//!   discloses the narrowing with a precise frozen trigger and binding dimension, and
//!   preserves the canonical owner / origin / data-boundary / fallback / freshness
//!   identity rather than silently dropping it or letting a stale pane read as fresh. A
//!   component with every dimension intact must NOT carry a spurious narrowing.
//! - **Cross-surface disclosure.** The same narrowed state surfaces in the docs/help
//!   browser, marketplace/account panes, the remote/service dashboard, embedded webviews,
//!   the auth-handoff surface, headless CLI, and support/admin exports so claim
//!   publication and field triage stay aligned on embedded-boundary downgrade behavior.
//!
//! Each [`EmbeddedBoundaryAccessibilityRow`] keys on one
//! [`crate::freeze_the_m5_docs_pane_header_embedded_origin_bar_boundary_fact_grid_marketplace_account_boundary_card_auth_handoff_card_remote_service_dashboard_header_open_in_browser_handoff_row_and_embedded_state_panel_component_matrix::M5EmbeddedBoundaryComponentFamily`]
//! and reuses that frozen family vocabulary plus the frozen
//! [`M5EmbeddedRequiredLabel`] and [`M5EmbeddedDowngradeTrigger`] and the shared
//! [`M5EmbeddedConsumerSurface`] consumer surfaces rather than minting parallel
//! synonyms, so the certified labels stay byte-identical to the matrix and the sibling
//! primitive packets.
//!
//! The packet is metadata-only: raw provider tokens, cookies, credentials, and page
//! bodies never cross this boundary; the packet carries only typed class tokens, opaque
//! summary / evidence refs, booleans, and redacted labels so support and diagnostics
//! exports can reconstruct exactly what an accessible fallback would have shown without
//! leaking embedded state.
//!
//! The boundary schema is
//! [`schemas/ui/m5-embedded-boundary-component-accessibility-parity.schema.json`](../../../../schemas/ui/m5-embedded-boundary-component-accessibility-parity.schema.json).
//! The contract doc is
//! [`docs/help/m5_embedded_boundary_component_accessibility_parity.md`](../../../../docs/help/m5_embedded_boundary_component_accessibility_parity.md).

#[cfg(test)]
mod tests;

use std::collections::{BTreeSet, HashSet};
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

// Reused frozen component vocabulary — the capstone certifies the freeze matrix's
// families, required labels, and downgrade triggers rather than mint parallel ones.
use crate::freeze_the_m5_docs_pane_header_embedded_origin_bar_boundary_fact_grid_marketplace_account_boundary_card_auth_handoff_card_remote_service_dashboard_header_open_in_browser_handoff_row_and_embedded_state_panel_component_matrix::{
    M5EmbeddedBoundaryComponentFamily, M5EmbeddedConsumerSurface, M5EmbeddedDowngradeTrigger,
    M5EmbeddedRequiredLabel, M5_EMBEDDED_BOUNDARY_COMPONENT_SCHEMA_REF,
};

/// Schema version stamped on the M05-1073 embedded-boundary-component accessibility
/// parity packet.
pub const EMBEDDED_BOUNDARY_A11Y_SCHEMA_VERSION: u32 = 1;

/// Stable record-kind tag carried by [`EmbeddedBoundaryAccessibilityPacket`].
pub const EMBEDDED_BOUNDARY_A11Y_RECORD_KIND: &str =
    "m5_embedded_boundary_component_accessibility_parity_packet";

/// Stable record-kind tag carried by each [`EmbeddedBoundaryAccessibilityRow`].
pub const EMBEDDED_BOUNDARY_A11Y_ROW_RECORD_KIND: &str =
    "m5_embedded_boundary_component_accessibility_parity_row";

/// Repo-relative path of the boundary schema.
pub const EMBEDDED_BOUNDARY_A11Y_SCHEMA_REF: &str =
    "schemas/ui/m5-embedded-boundary-component-accessibility-parity.schema.json";

/// Repo-relative path of the contract doc.
pub const EMBEDDED_BOUNDARY_A11Y_DOC_REF: &str =
    "docs/help/m5_embedded_boundary_component_accessibility_parity.md";

/// Repo-relative path of the frozen embedded-boundary component matrix this lane
/// certifies.
pub const EMBEDDED_BOUNDARY_A11Y_COMPONENT_MATRIX_REF: &str =
    M5_EMBEDDED_BOUNDARY_COMPONENT_SCHEMA_REF;

/// Repo-relative path of the protected fixture directory.
pub const EMBEDDED_BOUNDARY_A11Y_FIXTURE_DIR: &str =
    "fixtures/ui/m5-embedded-boundary-component-accessibility-parity";

/// Repo-relative path of the checked support-export artifact (the `include_str!`
/// canonical).
pub const EMBEDDED_BOUNDARY_A11Y_ARTIFACT_REF: &str =
    "artifacts/release/m5-embedded-boundary-component-accessibility-proof/support_export.json";

/// Repo-relative path of the checked machine-readable matrix CSV.
pub const EMBEDDED_BOUNDARY_A11Y_CSV_REF: &str =
    "artifacts/release/m5-embedded-boundary-component-accessibility-proof/matrix.csv";

/// Repo-relative path of the checked Markdown report.
pub const EMBEDDED_BOUNDARY_A11Y_REPORT_REF: &str =
    "artifacts/release/m5-embedded-boundary-component-accessibility-proof/report.md";

/// The reusable component families that render a non-linear hierarchy (the boundary-fact
/// grid's owner/origin / data-boundary / freshness grid) and therefore MUST bind their
/// grid to an equivalent flat list / textual path so the layout is navigable
/// non-visually.
const fn family_is_hierarchy_heavy(family: M5EmbeddedBoundaryComponentFamily) -> bool {
    matches!(family, M5EmbeddedBoundaryComponentFamily::BoundaryFactGrid)
}

/// The embedded-boundary dimension whose weakening a family primarily discloses. Every
/// row must model at least this dimension so its key weakening axis is covered.
const fn family_primary_dimension(
    family: M5EmbeddedBoundaryComponentFamily,
) -> M5EmbeddedClaimDimension {
    match family {
        M5EmbeddedBoundaryComponentFamily::DocsPaneHeader
        | M5EmbeddedBoundaryComponentFamily::RemoteServiceDashboardHeader => {
            M5EmbeddedClaimDimension::FreshnessTruth
        }
        M5EmbeddedBoundaryComponentFamily::EmbeddedOriginBar => {
            M5EmbeddedClaimDimension::OwnerOriginTruth
        }
        M5EmbeddedBoundaryComponentFamily::BoundaryFactGrid => {
            M5EmbeddedClaimDimension::DataBoundaryTruth
        }
        M5EmbeddedBoundaryComponentFamily::MarketplaceAccountBoundaryCard => {
            M5EmbeddedClaimDimension::AccountScopeTruth
        }
        M5EmbeddedBoundaryComponentFamily::AuthHandoffCard
        | M5EmbeddedBoundaryComponentFamily::OpenInBrowserHandoffRow => {
            M5EmbeddedClaimDimension::BrowserFallbackTruth
        }
        M5EmbeddedBoundaryComponentFamily::EmbeddedStatePanel => {
            M5EmbeddedClaimDimension::CapabilityLimitTruth
        }
    }
}

/// A rendered fallback modality for an embedded-boundary component.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5EmbeddedFallbackModality {
    /// A rich, structured (owner/origin / data-boundary / freshness grid) projection.
    Structured,
    /// A flat list projection.
    List,
    /// A textual / source-first projection.
    Textual,
    /// A CLI / headless line projection.
    Cli,
}

impl M5EmbeddedFallbackModality {
    /// Returns true when the modality is reachable without interpreting a rich,
    /// structured surface (i.e. a keyboard / screen-reader / headless path).
    pub const fn is_non_visual(self) -> bool {
        matches!(self, Self::List | Self::Textual | Self::Cli)
    }

    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Structured => "structured",
            Self::List => "list",
            Self::Textual => "textual",
            Self::Cli => "cli",
        }
    }
}

/// A rendering-surface capability tier. Distinct from the semantic consumer surface: the
/// same component may render at desktop-full capability or narrow to a companion,
/// read-only browser, headless CLI, handoff packet, or support export.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5EmbeddedRenderingSurface {
    /// The full-capability desktop shell surface.
    DesktopFull,
    /// The companion app.
    CompanionApp,
    /// A read-only browser projection.
    BrowserReadonly,
    /// A headless CLI surface.
    CliHeadless,
    /// A handoff packet.
    HandoffPacket,
    /// A support / admin export.
    SupportExport,
}

impl M5EmbeddedRenderingSurface {
    /// Returns true when the surface narrows interactivity below the desktop
    /// full-capability baseline and therefore must disclose its reduction.
    pub const fn is_narrowed(self) -> bool {
        !matches!(self, Self::DesktopFull)
    }

    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::DesktopFull => "desktop_full",
            Self::CompanionApp => "companion_app",
            Self::BrowserReadonly => "browser_readonly",
            Self::CliHeadless => "cli_headless",
            Self::HandoffPacket => "handoff_packet",
            Self::SupportExport => "support_export",
        }
    }
}

/// Keyboard / screen-reader / CLI reach for a component's non-visual path.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EmbeddedNonVisualReachState {
    /// Fully traversable and labeled with no loss.
    ReachableAndLabeled,
    /// Reachable and labeled, but with a disclosed reduction (yellow).
    DisclosedReducedButReachable,
    /// A view-only / hover-only / menu-only surface that traps keyboard / assistive-tech
    /// / headless users (red).
    ViewOnlyTrap,
}

impl EmbeddedNonVisualReachState {
    /// Returns true when the state never strands keyboard / assistive-tech / headless
    /// users.
    pub const fn never_traps(self) -> bool {
        !matches!(self, Self::ViewOnlyTrap)
    }

    /// Returns true when the state carries a disclosed reduction.
    pub const fn is_disclosed_reduction(self) -> bool {
        matches!(self, Self::DisclosedReducedButReachable)
    }

    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ReachableAndLabeled => "reachable_and_labeled",
            Self::DisclosedReducedButReachable => "disclosed_reduced_but_reachable",
            Self::ViewOnlyTrap => "view_only_trap",
        }
    }
}

/// Whether an export-safe summary preserves the component meaning without a screenshot.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EmbeddedExportSummaryState {
    /// The component meaning reconstructs from the summary without a screenshot.
    ReconstructableWithoutScreenshot,
    /// Partial capture, but disclosed (yellow).
    DisclosedPartialCapture,
    /// The export relies on a screenshot to carry meaning (red).
    AbsentNeedsScreenshot,
}

impl EmbeddedExportSummaryState {
    /// Returns true when the export never falls back to a screenshot alone.
    pub const fn never_screenshot_only(self) -> bool {
        !matches!(self, Self::AbsentNeedsScreenshot)
    }

    /// Returns true when the state carries a disclosed reduction.
    pub const fn is_disclosed_reduction(self) -> bool {
        matches!(self, Self::DisclosedPartialCapture)
    }

    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ReconstructableWithoutScreenshot => "reconstructable_without_screenshot",
            Self::DisclosedPartialCapture => "disclosed_partial_capture",
            Self::AbsentNeedsScreenshot => "absent_needs_screenshot",
        }
    }
}

/// Whether a narrower rendering surface discloses its reduced interactivity.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EmbeddedNarrowingDisclosureState {
    /// Full label and summary parity with the desktop surface.
    ParityPreserved,
    /// Reduced interactivity, disclosed with preserved labels (yellow).
    DisclosedNarrowed,
    /// Interactivity, state, or actions dropped without disclosure (red).
    SilentlyDropped,
}

impl EmbeddedNarrowingDisclosureState {
    /// Returns true when the surface never silently drops state or actions.
    pub const fn never_drops_silently(self) -> bool {
        !matches!(self, Self::SilentlyDropped)
    }

    /// Returns true when the state carries a disclosed reduction.
    pub const fn is_disclosed_reduction(self) -> bool {
        matches!(self, Self::DisclosedNarrowed)
    }

    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ParityPreserved => "parity_preserved",
            Self::DisclosedNarrowed => "disclosed_narrowed",
            Self::SilentlyDropped => "silently_dropped",
        }
    }
}

/// The boundary-support claim ceiling a component asserts: how strong an embedded-boundary
/// truth it lets a surface present. Auto-narrowing lowers this ceiling when a boundary
/// dimension weakens so a stale, offline, provider-blocked, or partial pane can never keep
/// an old fresh first-party `full-truth` label.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5EmbeddedAccessClaim {
    /// Full-truth: the component's live, first-party-local owner/origin / state /
    /// freshness truth is fully reachable and current — the strongest claim.
    FullTruth,
    /// Resolved-truth: a resolved, self-sufficient boundary posture (a fully-labelled
    /// origin bar, handoff card, or account boundary) that is not itself a live-adapting
    /// first-party-local stream.
    ResolvedTruth,
    /// Degraded: usable, but with a disclosed reduction in owner/origin, data-boundary, or
    /// freshness confidence.
    Degraded,
    /// Stale: a stale snapshot is deliberately kept visible pending refresh, not a live
    /// current value.
    Stale,
    /// Offline: an offline snapshot with no live refresh or browser-fallback path.
    Offline,
    /// Provider-blocked: a provider or policy blocks the embedded content or the handoff.
    ProviderBlocked,
}

impl M5EmbeddedAccessClaim {
    /// Every claim tier, strongest first.
    pub const ALL: [Self; 6] = [
        Self::FullTruth,
        Self::ResolvedTruth,
        Self::Degraded,
        Self::Stale,
        Self::Offline,
        Self::ProviderBlocked,
    ];

    /// Capability rank; a higher rank asserts a stronger boundary posture. Narrowing
    /// lowers rank.
    pub const fn capability_rank(self) -> u8 {
        match self {
            Self::FullTruth => 5,
            Self::ResolvedTruth => 4,
            Self::Degraded => 3,
            Self::Stale => 2,
            Self::Offline => 1,
            Self::ProviderBlocked => 0,
        }
    }

    /// Returns true when this claim asserts live, current, fresh first-party-local truth.
    pub const fn asserts_live_truth(self) -> bool {
        matches!(self, Self::FullTruth)
    }

    /// Returns true when this claim asserts a fully self-sufficient (live or resolved /
    /// current) boundary posture.
    pub const fn asserts_full_self_sufficiency(self) -> bool {
        matches!(self, Self::FullTruth | Self::ResolvedTruth)
    }

    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::FullTruth => "full_truth",
            Self::ResolvedTruth => "resolved_truth",
            Self::Degraded => "degraded",
            Self::Stale => "stale",
            Self::Offline => "offline",
            Self::ProviderBlocked => "provider_blocked",
        }
    }
}

/// The embedded-boundary dimension whose state governs how far a component may claim
/// fresh, current first-party-local truth. These are exactly the axes the spec requires
/// auto-narrowing on: owner/origin, data boundary, browser fallback, capability limit,
/// freshness (stale/offline), and account scope.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5EmbeddedClaimDimension {
    /// Owner/origin truth: is who owns the embedded content and its origin resolved and
    /// named?
    OwnerOriginTruth,
    /// Data-boundary truth: is the data-exit boundary the content crosses resolved and
    /// named?
    DataBoundaryTruth,
    /// Browser-fallback truth: is the browser / device-code fallback path resolved and
    /// reachable, not hidden in menus only?
    BrowserFallbackTruth,
    /// Capability-limit truth: are the capability limits of the embedded surface relative
    /// to native chrome resolved and named?
    CapabilityLimitTruth,
    /// Freshness truth: is the source / version / last-updated freshness resolved, or has
    /// it gone stale / offline?
    FreshnessTruth,
    /// Account-scope truth: is whose account the surface is scoped to resolved and named?
    AccountScopeTruth,
}

impl M5EmbeddedClaimDimension {
    /// Every dimension, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::OwnerOriginTruth,
        Self::DataBoundaryTruth,
        Self::BrowserFallbackTruth,
        Self::CapabilityLimitTruth,
        Self::FreshnessTruth,
        Self::AccountScopeTruth,
    ];

    /// The frozen downgrade trigger this dimension names when its weakness binds a
    /// narrowing. Each dimension maps to the on-topic frozen trigger the freeze matrix
    /// already governs, so the certified reason stays byte-identical to the matrix.
    pub const fn default_trigger(self) -> M5EmbeddedDowngradeTrigger {
        match self {
            Self::OwnerOriginTruth => M5EmbeddedDowngradeTrigger::OwnerOrOriginUnstated,
            Self::DataBoundaryTruth => M5EmbeddedDowngradeTrigger::DataBoundaryUnstated,
            Self::BrowserFallbackTruth => {
                M5EmbeddedDowngradeTrigger::BrowserFallbackHiddenInMenusOnly
            }
            Self::CapabilityLimitTruth => M5EmbeddedDowngradeTrigger::CapabilityLimitsUnstated,
            Self::FreshnessTruth => M5EmbeddedDowngradeTrigger::FreshnessOrLastUpdatedUnstated,
            Self::AccountScopeTruth => M5EmbeddedDowngradeTrigger::AccountScopeUnstated,
        }
    }

    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::OwnerOriginTruth => "owner_origin_truth",
            Self::DataBoundaryTruth => "data_boundary_truth",
            Self::BrowserFallbackTruth => "browser_fallback_truth",
            Self::CapabilityLimitTruth => "capability_limit_truth",
            Self::FreshnessTruth => "freshness_truth",
            Self::AccountScopeTruth => "account_scope_truth",
        }
    }
}

/// The observed condition of one embedded-boundary dimension. Anything weaker than
/// [`Self::Intact`] imposes a narrowing ceiling on the component's support claim.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5EmbeddedConditionState {
    /// Fully verified / current / live first-party-local — imposes no ceiling.
    Intact,
    /// Partially resolved — scope or confidence is reduced; support drops to degraded.
    Partial,
    /// Stale — a stale snapshot is deliberately kept visible pending refresh; support
    /// drops to stale.
    Stale,
    /// Offline — an offline snapshot with no refresh / fallback path; support drops to
    /// offline.
    Offline,
    /// Provider-blocked — a provider or policy blocks the content or handoff; support
    /// drops to provider-blocked.
    ProviderBlocked,
}

impl M5EmbeddedConditionState {
    /// Every condition state, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::Intact,
        Self::Partial,
        Self::Stale,
        Self::Offline,
        Self::ProviderBlocked,
    ];

    /// Returns true when the dimension is weaker than intact and therefore imposes a
    /// narrowing ceiling.
    pub const fn is_weak(self) -> bool {
        !matches!(self, Self::Intact)
    }

    /// The strongest boundary-support claim this condition state permits.
    pub const fn permitted_ceiling(self) -> M5EmbeddedAccessClaim {
        match self {
            Self::Intact => M5EmbeddedAccessClaim::FullTruth,
            Self::Partial => M5EmbeddedAccessClaim::Degraded,
            Self::Stale => M5EmbeddedAccessClaim::Stale,
            Self::Offline => M5EmbeddedAccessClaim::Offline,
            Self::ProviderBlocked => M5EmbeddedAccessClaim::ProviderBlocked,
        }
    }

    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Intact => "intact",
            Self::Partial => "partial",
            Self::Stale => "stale",
            Self::Offline => "offline",
            Self::ProviderBlocked => "provider_blocked",
        }
    }
}

/// One embedded-boundary dimension's observed condition on a component.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EmbeddedClaimConditionEntry {
    /// Which dimension this entry describes.
    pub dimension: M5EmbeddedClaimDimension,
    /// The observed condition state of the dimension.
    pub state: M5EmbeddedConditionState,
}

/// An honest boundary-support-claim auto-narrow block. When a boundary dimension weakens,
/// the component's support claim lowers to the permitted ceiling, names the binding
/// dimension and frozen trigger, and preserves the canonical owner / origin / data-boundary
/// / fallback / freshness identity rather than silently dropping it.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EmbeddedClaimAutoNarrow {
    /// The support claim the component is narrowed to.
    pub narrowed_to: M5EmbeddedAccessClaim,
    /// The dimension whose weakness bound the narrowing (the one imposing the strongest
    /// ceiling constraint).
    pub binding_dimension: M5EmbeddedClaimDimension,
    /// The frozen downgrade trigger (reused vocabulary) the narrowing names.
    pub trigger: M5EmbeddedDowngradeTrigger,
    /// A precise, non-generic label safe to render.
    pub narrowed_label: String,
    /// The canonical owner, origin, data boundary, browser fallback, and freshness are
    /// preserved rather than dropped; must hold.
    pub preserves_canonical_identity: bool,
}

impl EmbeddedClaimAutoNarrow {
    /// Whether the auto-narrow block is honest: it preserves canonical identity and
    /// carries a precise, non-generic label.
    pub fn is_honest(&self) -> bool {
        self.preserves_canonical_identity && !label_is_generic(&self.narrowed_label)
    }
}

/// Copy / export parity for a component's accessible fallback: the same truth must be
/// copyable as text / JSON / Markdown, and a screenshot is never the only export.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EmbeddedCopyExportParity {
    /// The copy / export formats offered (must include text, json, markdown).
    #[serde(default)]
    pub formats: Vec<String>,
    /// The named export fields the summary carries.
    #[serde(default)]
    pub export_fields: Vec<String>,
    /// A screenshot is never the only export; must always hold.
    pub screenshot_only_prohibited: bool,
}

impl EmbeddedCopyExportParity {
    /// Whether the copy / export parity is complete: text / JSON / Markdown are all
    /// offered, at least one export field is named, and screenshots are prohibited as the
    /// sole export.
    pub fn is_complete(&self) -> bool {
        self.screenshot_only_prohibited
            && self.formats.iter().any(|f| f == "text")
            && self.formats.iter().any(|f| f == "json")
            && self.formats.iter().any(|f| f == "markdown")
            && !self.export_fields.is_empty()
    }
}

/// Per-rendering-surface narrowing disclosure.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EmbeddedRenderingNarrowingDisclosure {
    /// The rendering surface being narrowed.
    pub rendering_surface: M5EmbeddedRenderingSurface,
    /// How the surface discloses its reduced interactivity.
    pub state: EmbeddedNarrowingDisclosureState,
    /// The labels preserved across the narrowing.
    #[serde(default)]
    pub preserved_labels: Vec<String>,
    /// The interactions reduced on the narrowed surface.
    #[serde(default)]
    pub reduced_interactions: Vec<String>,
}

/// Derived qualification status for an embedded-boundary-component accessibility row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EmbeddedAccessibilityStatus {
    /// Full keyboard / screen-reader / CLI / export parity with no narrowing (green).
    Parity,
    /// Reduced but fully disclosed, reachable, and honestly auto-narrowed (yellow).
    NarrowedDisclosed,
    /// Strands assistive tech, needs a screenshot, over-claims boundary truth, or drops
    /// state silently (red).
    Stranded,
}

impl EmbeddedAccessibilityStatus {
    /// Stable token recorded in the summary / CSV.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Parity => "parity",
            Self::NarrowedDisclosed => "narrowed_disclosed",
            Self::Stranded => "stranded",
        }
    }
}

/// Accessibility / auto-narrowing parity row for one embedded-boundary component family.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EmbeddedBoundaryAccessibilityRow {
    /// Record kind; must equal [`EMBEDDED_BOUNDARY_A11Y_ROW_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`EMBEDDED_BOUNDARY_A11Y_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable row id.
    pub row_id: String,
    /// The frozen component family this row certifies.
    pub component_family: M5EmbeddedBoundaryComponentFamily,
    /// Ref to the frozen matrix family schema this row certifies.
    pub source_family_schema_ref: String,
    /// Opaque ref to the embedded surface / origin / provider context this component acts
    /// on; stays visible on every surface, so this is never empty.
    pub boundary_context_ref: String,
    /// Rendered modalities offered; a hierarchy-heavy family must also offer a non-visual
    /// (list / textual / cli) path.
    #[serde(default)]
    pub fallback_modalities: Vec<M5EmbeddedFallbackModality>,
    /// The non-visual / CLI path reaches the same canonical owner/origin, data boundary,
    /// browser fallback, capability limits, account scope, and freshness as the rich
    /// surface; must hold.
    pub reaches_canonical_truth: bool,
    /// Keyboard reach into the non-visual path.
    pub keyboard_reach: EmbeddedNonVisualReachState,
    /// Screen-reader reach into the non-visual path.
    pub screen_reader_reach: EmbeddedNonVisualReachState,
    /// CLI / headless reach into the non-visual path.
    pub cli_reach: EmbeddedNonVisualReachState,
    /// Whether the export-safe summary preserves component meaning.
    pub export_summary: EmbeddedExportSummaryState,
    /// Ref to the export-safe summary object for this component.
    pub export_summary_ref: String,
    /// The copy / export parity of the accessible fallback.
    pub copy_export: EmbeddedCopyExportParity,
    /// The full support claim this family asserts when every dimension is intact.
    pub full_support_claim: M5EmbeddedAccessClaim,
    /// The observed condition of each modeled boundary dimension.
    #[serde(default)]
    pub claim_conditions: Vec<EmbeddedClaimConditionEntry>,
    /// The honest auto-narrow block, present only when some dimension weakens below the
    /// family's full claim.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub claim_narrow: Option<EmbeddedClaimAutoNarrow>,
    /// Rendering surfaces this component is certified on.
    #[serde(default)]
    pub rendering_surfaces: Vec<M5EmbeddedRenderingSurface>,
    /// Per-surface narrowing disclosures.
    #[serde(default)]
    pub narrowing_disclosures: Vec<EmbeddedRenderingNarrowingDisclosure>,
    /// The required labels the accessible fallback preserves (reused vocabulary).
    #[serde(default)]
    pub required_labels: Vec<M5EmbeddedRequiredLabel>,
    /// Semantic consumer surfaces this component is embedded in (reused vocabulary).
    #[serde(default)]
    pub consumer_surfaces: Vec<M5EmbeddedConsumerSurface>,
    /// Source contract refs backing this row.
    #[serde(default)]
    pub source_refs: Vec<String>,
    /// ISO 8601 UTC timestamp the accessibility posture was observed.
    pub observed_at: String,
    /// Evidence packet refs backing this row.
    #[serde(default)]
    pub evidence_refs: Vec<String>,
}

impl EmbeddedBoundaryAccessibilityRow {
    /// Returns true when this family renders a non-linear hierarchy and must bind to a
    /// flat non-visual path.
    pub const fn is_hierarchy_heavy(&self) -> bool {
        family_is_hierarchy_heavy(self.component_family)
    }

    /// Returns true when at least one non-visual (list / textual / cli) fallback modality
    /// is offered.
    pub fn has_non_visual_fallback(&self) -> bool {
        self.fallback_modalities.iter().any(|m| m.is_non_visual())
    }

    /// The condition state observed for one dimension, or `Intact` when the row does not
    /// model that dimension.
    pub fn condition_for(&self, dimension: M5EmbeddedClaimDimension) -> M5EmbeddedConditionState {
        self.claim_conditions
            .iter()
            .find(|c| c.dimension == dimension)
            .map(|c| c.state)
            .unwrap_or(M5EmbeddedConditionState::Intact)
    }

    /// Whether any modeled dimension is weaker than intact.
    pub fn has_weak_dimension(&self) -> bool {
        self.claim_conditions.iter().any(|c| c.state.is_weak())
    }

    /// The strongest support claim permitted after applying every modeled dimension's
    /// ceiling, capped at the family's full claim.
    pub fn permitted_claim(&self) -> M5EmbeddedAccessClaim {
        let mut permitted = self.full_support_claim;
        for condition in &self.claim_conditions {
            let ceiling = condition.state.permitted_ceiling();
            if ceiling.capability_rank() < permitted.capability_rank() {
                permitted = ceiling;
            }
        }
        permitted
    }

    /// The dimension imposing the strongest (lowest-rank) ceiling, if any weak dimension
    /// narrows below the family's full claim.
    pub fn binding_dimension(&self) -> Option<M5EmbeddedClaimDimension> {
        let mut binding: Option<(M5EmbeddedClaimDimension, u8)> = None;
        for condition in &self.claim_conditions {
            if !condition.state.is_weak() {
                continue;
            }
            let ceiling = condition.state.permitted_ceiling();
            if ceiling.capability_rank() >= self.full_support_claim.capability_rank() {
                // The dimension is weak but does not narrow below the full claim.
                continue;
            }
            let rank = ceiling.capability_rank();
            match binding {
                Some((_, best)) if best <= rank => {}
                _ => binding = Some((condition.dimension, rank)),
            }
        }
        binding.map(|(dimension, _)| dimension)
    }

    /// The support claim this component effectively asserts after narrowing.
    pub fn effective_claim(&self) -> M5EmbeddedAccessClaim {
        match &self.claim_narrow {
            Some(narrow) => narrow.narrowed_to,
            None => self.full_support_claim,
        }
    }

    /// AC / auto-narrowing honesty: a stale, offline, provider-blocked, or partial boundary
    /// state can no longer keep an old fresh first-party `full-truth` / `resolved-truth`
    /// label. The effective claim never exceeds the permitted ceiling; when a dimension
    /// narrows below the full claim, an honest narrow block is present, narrows to exactly
    /// the permitted ceiling, binds to the ceiling-imposing dimension with its frozen
    /// trigger, and preserves canonical identity. When nothing narrows, no spurious narrow
    /// block is present.
    pub fn claim_is_honest(&self) -> bool {
        let permitted = self.permitted_claim();
        if self.effective_claim().capability_rank() > permitted.capability_rank() {
            return false;
        }
        match (&self.claim_narrow, self.binding_dimension()) {
            (Some(narrow), Some(binding)) => {
                narrow.is_honest()
                    && narrow.narrowed_to == permitted
                    && narrow.binding_dimension == binding
                    && narrow.trigger == binding.default_trigger()
                    && self.condition_for(binding).is_weak()
            }
            // A narrow block with no ceiling-imposing dimension is spurious.
            (Some(_), None) => false,
            // A ceiling-imposing dimension with no narrow block over-claims.
            (None, Some(_)) => false,
            (None, None) => true,
        }
    }

    /// AC / assistive-tech reach: accessibility and export surfaces reach the same
    /// canonical truth — no keyboard / screen-reader / CLI trap, a hierarchy-heavy family
    /// offers a non-visual fallback, and the export reconstructs meaning without a
    /// screenshot.
    pub fn reaches_canonical_truth_via_at(&self) -> bool {
        self.reaches_canonical_truth
            && !self.boundary_context_ref.trim().is_empty()
            && self.keyboard_reach.never_traps()
            && self.screen_reader_reach.never_traps()
            && self.cli_reach.never_traps()
            && (!self.is_hierarchy_heavy() || self.has_non_visual_fallback())
    }

    /// The export preserves the component meaning without a screenshot.
    pub fn export_preserves_meaning(&self) -> bool {
        self.export_summary.never_screenshot_only()
            && !self.export_summary_ref.trim().is_empty()
            && self.copy_export.is_complete()
    }

    /// Whether any axis is in a disclosed-reduction (yellow) state or the component
    /// carries an honest claim narrow.
    pub fn is_reduced(&self) -> bool {
        self.claim_narrow.is_some()
            || self.keyboard_reach.is_disclosed_reduction()
            || self.screen_reader_reach.is_disclosed_reduction()
            || self.cli_reach.is_disclosed_reduction()
            || self.export_summary.is_disclosed_reduction()
            || self
                .narrowing_disclosures
                .iter()
                .any(|d| d.state.is_disclosed_reduction())
    }

    /// AC / cross-surface disclosure: every narrower rendering surface discloses its
    /// reduced interactivity and keeps its labels, so claim publication and field triage
    /// stay aligned on the same narrowed state.
    pub fn narrowing_disclosed(&self) -> bool {
        // Every declared narrowed rendering surface has a disclosure entry.
        for surface in &self.rendering_surfaces {
            if surface.is_narrowed()
                && !self
                    .narrowing_disclosures
                    .iter()
                    .any(|d| d.rendering_surface == *surface)
            {
                return false;
            }
        }
        // Every disclosure never silently drops and preserves labels on a narrowed
        // surface.
        self.narrowing_disclosures.iter().all(|d| {
            d.state.never_drops_silently()
                && (!d.rendering_surface.is_narrowed() || !d.preserved_labels.is_empty())
        })
    }

    /// Whether the row models its family's primary weakening dimension.
    pub fn models_primary_dimension(&self) -> bool {
        let primary = family_primary_dimension(self.component_family);
        self.claim_conditions.iter().any(|c| c.dimension == primary)
    }

    /// Whether every mandatory required label is preserved on the accessible fallback.
    pub fn preserves_mandatory_labels(&self) -> bool {
        M5EmbeddedRequiredLabel::MANDATORY
            .iter()
            .all(|label| self.required_labels.contains(label))
    }

    /// Derived qualification status.
    pub fn status(&self) -> EmbeddedAccessibilityStatus {
        if !self.claim_is_honest()
            || !self.reaches_canonical_truth_via_at()
            || !self.export_preserves_meaning()
            || !self.narrowing_disclosed()
            || !self.models_primary_dimension()
            || !self.preserves_mandatory_labels()
        {
            return EmbeddedAccessibilityStatus::Stranded;
        }
        if self.is_reduced() {
            EmbeddedAccessibilityStatus::NarrowedDisclosed
        } else {
            EmbeddedAccessibilityStatus::Parity
        }
    }

    /// Whether the row's identity and evidence fields are complete.
    pub fn is_complete(&self) -> bool {
        self.record_kind == EMBEDDED_BOUNDARY_A11Y_ROW_RECORD_KIND
            && self.schema_version == EMBEDDED_BOUNDARY_A11Y_SCHEMA_VERSION
            && !self.row_id.trim().is_empty()
            && !self.source_family_schema_ref.trim().is_empty()
            && !self.boundary_context_ref.trim().is_empty()
            && !self.fallback_modalities.is_empty()
            && !self.claim_conditions.is_empty()
            && !self.observed_at.trim().is_empty()
            && !self.evidence_refs.is_empty()
            && self.evidence_refs.iter().all(|r| !r.trim().is_empty())
    }

    /// Deterministic governed chip line for this row.
    pub fn chip_tokens(&self) -> String {
        format!(
            "family={family} keyboard={keyboard} screen_reader={screen_reader} cli={cli} \
export={export} full_claim={full} effective_claim={effective} status={status}",
            family = self.component_family.as_str(),
            keyboard = self.keyboard_reach.as_str(),
            screen_reader = self.screen_reader_reach.as_str(),
            cli = self.cli_reach.as_str(),
            export = self.export_summary.as_str(),
            full = self.full_support_claim.as_str(),
            effective = self.effective_claim().as_str(),
            status = self.status().as_str(),
        )
    }
}

/// Rolled-up summary of an M05-1073 embedded-boundary-component accessibility parity
/// packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EmbeddedBoundaryAccessibilitySummary {
    pub family_count: usize,
    pub hierarchy_heavy_family_count: usize,
    pub all_hierarchy_heavy_have_non_visual_fallback: bool,
    pub all_reach_canonical_truth_via_at: bool,
    pub all_claims_honest: bool,
    pub all_export_summaries_preserve_meaning: bool,
    pub all_narrowing_disclosed: bool,
    pub green_count: usize,
    pub yellow_count: usize,
    pub red_count: usize,
    pub rendering_surface_count: usize,
    pub consumer_surface_count: usize,
}

/// Constructor input for [`EmbeddedBoundaryAccessibilityPacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EmbeddedBoundaryAccessibilityPacketInput {
    pub packet_id: String,
    pub as_of: String,
    pub matrix_ref: String,
    pub rows: Vec<EmbeddedBoundaryAccessibilityRow>,
}

/// Checked-in M05-1073 embedded-boundary-component accessibility parity packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EmbeddedBoundaryAccessibilityPacket {
    pub schema_version: u32,
    pub record_kind: String,
    pub packet_id: String,
    pub as_of: String,
    pub matrix_ref: String,
    #[serde(default)]
    pub rows: Vec<EmbeddedBoundaryAccessibilityRow>,
    pub summary: EmbeddedBoundaryAccessibilitySummary,
}

impl EmbeddedBoundaryAccessibilityPacket {
    /// Builds a packet, stamping the record kind, schema version, and computed summary.
    pub fn new(input: EmbeddedBoundaryAccessibilityPacketInput) -> Self {
        let mut packet = Self {
            schema_version: EMBEDDED_BOUNDARY_A11Y_SCHEMA_VERSION,
            record_kind: EMBEDDED_BOUNDARY_A11Y_RECORD_KIND.to_owned(),
            packet_id: input.packet_id,
            as_of: input.as_of,
            matrix_ref: input.matrix_ref,
            rows: input.rows,
            summary: EmbeddedBoundaryAccessibilitySummary {
                family_count: 0,
                hierarchy_heavy_family_count: 0,
                all_hierarchy_heavy_have_non_visual_fallback: false,
                all_reach_canonical_truth_via_at: false,
                all_claims_honest: false,
                all_export_summaries_preserve_meaning: false,
                all_narrowing_disclosed: false,
                green_count: 0,
                yellow_count: 0,
                red_count: 0,
                rendering_surface_count: 0,
                consumer_surface_count: 0,
            },
        };
        packet.summary = packet.computed_summary();
        packet
    }

    /// Families represented by some row in this packet.
    pub fn represented_families(&self) -> BTreeSet<M5EmbeddedBoundaryComponentFamily> {
        self.rows.iter().map(|r| r.component_family).collect()
    }

    /// Dimensions exercised by some row's claim conditions.
    pub fn exercised_dimensions(&self) -> BTreeSet<M5EmbeddedClaimDimension> {
        self.rows
            .iter()
            .flat_map(|r| r.claim_conditions.iter().map(|c| c.dimension))
            .collect()
    }

    /// Support claim tiers that appear as an effective claim across the rows.
    pub fn represented_effective_claims(&self) -> BTreeSet<M5EmbeddedAccessClaim> {
        self.rows.iter().map(|r| r.effective_claim()).collect()
    }

    /// Consumer surfaces ingesting some row in this packet.
    pub fn represented_consumer_surfaces(&self) -> HashSet<M5EmbeddedConsumerSurface> {
        self.rows
            .iter()
            .flat_map(|r| r.consumer_surfaces.iter().copied())
            .collect()
    }

    /// Computes summary fields from the packet contents.
    pub fn computed_summary(&self) -> EmbeddedBoundaryAccessibilitySummary {
        let mut rendering = BTreeSet::new();
        let mut consumers: HashSet<M5EmbeddedConsumerSurface> = HashSet::new();
        for row in &self.rows {
            rendering.extend(row.rendering_surfaces.iter().copied());
            consumers.extend(row.consumer_surfaces.iter().copied());
        }

        let hierarchy_heavy: Vec<&EmbeddedBoundaryAccessibilityRow> = self
            .rows
            .iter()
            .filter(|row| row.is_hierarchy_heavy())
            .collect();

        let mut green = 0;
        let mut yellow = 0;
        let mut red = 0;
        for row in &self.rows {
            match row.status() {
                EmbeddedAccessibilityStatus::Parity => green += 1,
                EmbeddedAccessibilityStatus::NarrowedDisclosed => yellow += 1,
                EmbeddedAccessibilityStatus::Stranded => red += 1,
            }
        }

        EmbeddedBoundaryAccessibilitySummary {
            family_count: self.rows.len(),
            hierarchy_heavy_family_count: hierarchy_heavy.len(),
            all_hierarchy_heavy_have_non_visual_fallback: hierarchy_heavy
                .iter()
                .all(|row| row.has_non_visual_fallback()),
            all_reach_canonical_truth_via_at: self
                .rows
                .iter()
                .all(EmbeddedBoundaryAccessibilityRow::reaches_canonical_truth_via_at),
            all_claims_honest: self
                .rows
                .iter()
                .all(EmbeddedBoundaryAccessibilityRow::claim_is_honest),
            all_export_summaries_preserve_meaning: self
                .rows
                .iter()
                .all(EmbeddedBoundaryAccessibilityRow::export_preserves_meaning),
            all_narrowing_disclosed: self
                .rows
                .iter()
                .all(EmbeddedBoundaryAccessibilityRow::narrowing_disclosed),
            green_count: green,
            yellow_count: yellow,
            red_count: red,
            rendering_surface_count: rendering.len(),
            consumer_surface_count: consumers.len(),
        }
    }

    /// Validates the packet and returns every contract violation.
    pub fn validate(&self) -> Vec<EmbeddedBoundaryAccessibilityViolation> {
        let mut violations = Vec::new();

        if self.schema_version != EMBEDDED_BOUNDARY_A11Y_SCHEMA_VERSION {
            violations.push(EmbeddedBoundaryAccessibilityViolation::SchemaVersion {
                expected: EMBEDDED_BOUNDARY_A11Y_SCHEMA_VERSION,
                actual: self.schema_version,
            });
        }
        if self.record_kind != EMBEDDED_BOUNDARY_A11Y_RECORD_KIND {
            violations.push(EmbeddedBoundaryAccessibilityViolation::RecordKind {
                expected: EMBEDDED_BOUNDARY_A11Y_RECORD_KIND.to_owned(),
                actual: self.record_kind.clone(),
            });
        }
        if self.packet_id.trim().is_empty()
            || self.as_of.trim().is_empty()
            || self.matrix_ref.trim().is_empty()
        {
            violations.push(EmbeddedBoundaryAccessibilityViolation::MissingIdentity);
        }

        let mut row_ids = BTreeSet::new();
        let mut seen_families = BTreeSet::new();
        for row in &self.rows {
            if !row_ids.insert(row.row_id.clone()) {
                violations.push(EmbeddedBoundaryAccessibilityViolation::DuplicateId {
                    id: row.row_id.clone(),
                });
            }
            seen_families.insert(row.component_family);

            if !row.is_complete() {
                violations.push(EmbeddedBoundaryAccessibilityViolation::IncompleteRow {
                    id: row.row_id.clone(),
                });
            }

            // Each row must model its family's primary weakening dimension.
            if !row.models_primary_dimension() {
                violations.push(
                    EmbeddedBoundaryAccessibilityViolation::MissingPrimaryDimension {
                        id: row.row_id.clone(),
                        dimension: family_primary_dimension(row.component_family),
                    },
                );
            }

            // Each row must preserve every mandatory embedded-boundary label.
            if !row.preserves_mandatory_labels() {
                violations.push(
                    EmbeddedBoundaryAccessibilityViolation::MissingMandatoryLabel {
                        id: row.row_id.clone(),
                    },
                );
            }

            // A hierarchy-heavy family must render a structured grid *and* a non-visual
            // path.
            if row.is_hierarchy_heavy()
                && !row
                    .fallback_modalities
                    .contains(&M5EmbeddedFallbackModality::Structured)
            {
                violations.push(
                    EmbeddedBoundaryAccessibilityViolation::HierarchyHeavyMissingStructured {
                        id: row.row_id.clone(),
                    },
                );
            }

            // AC1: claim never over-asserts fresh first-party truth for a weakened one.
            if !row.claim_is_honest() {
                violations.push(EmbeddedBoundaryAccessibilityViolation::ClaimOverAsserted {
                    id: row.row_id.clone(),
                });
            }

            // Assistive-tech / CLI reach the same canonical truth.
            if !row.reaches_canonical_truth_via_at() {
                violations.push(
                    EmbeddedBoundaryAccessibilityViolation::AssistiveTechStranded {
                        id: row.row_id.clone(),
                    },
                );
            }

            // Export preserves meaning without a screenshot.
            if !row.export_preserves_meaning() {
                violations.push(
                    EmbeddedBoundaryAccessibilityViolation::ExportRequiresScreenshot {
                        id: row.row_id.clone(),
                    },
                );
            }

            // Narrowing disclosed on every narrowed rendering surface.
            if !row.narrowing_disclosed() {
                violations.push(
                    EmbeddedBoundaryAccessibilityViolation::NarrowingDropsContextSilently {
                        id: row.row_id.clone(),
                    },
                );
            }

            // Consumer parity: at least two consumer surfaces ingest the row.
            if row.consumer_surfaces.len() < 2 {
                violations.push(
                    EmbeddedBoundaryAccessibilityViolation::MissingConsumerParity {
                        id: row.row_id.clone(),
                    },
                );
            }

            // No red rows may ship.
            if row.status() == EmbeddedAccessibilityStatus::Stranded {
                violations.push(EmbeddedBoundaryAccessibilityViolation::StrandedRow {
                    id: row.row_id.clone(),
                });
            }
        }

        // Coverage: every frozen family is certified at least once.
        for family in M5EmbeddedBoundaryComponentFamily::ALL {
            if !seen_families.contains(&family) {
                violations
                    .push(EmbeddedBoundaryAccessibilityViolation::MissingFamilyCoverage { family });
            }
        }

        // Coverage: every weakening dimension is exercised somewhere.
        let exercised = self.exercised_dimensions();
        for dimension in M5EmbeddedClaimDimension::ALL {
            if !exercised.contains(&dimension) {
                violations.push(
                    EmbeddedBoundaryAccessibilityViolation::MissingDimensionCoverage { dimension },
                );
            }
        }

        // Coverage: every support claim tier appears as an effective claim, so the full
        // narrowing spectrum (full-truth → … → provider-blocked) is proven end-to-end.
        let effective = self.represented_effective_claims();
        for claim in M5EmbeddedAccessClaim::ALL {
            if !effective.contains(&claim) {
                violations.push(
                    EmbeddedBoundaryAccessibilityViolation::MissingClaimTierCoverage { claim },
                );
            }
        }

        // Cross-surface: the same narrowed state must reach the docs/help browser,
        // marketplace/account panes, the remote/service dashboard, embedded webviews, the
        // auth-handoff surface, and support/admin exports — so every consumer surface is
        // exercised at least once across the packet.
        let consumers = self.represented_consumer_surfaces();
        for surface in M5_EMBEDDED_BOUNDARY_A11Y_CONSUMER_SURFACES {
            if !consumers.contains(&surface) {
                violations.push(
                    EmbeddedBoundaryAccessibilityViolation::MissingConsumerSurfaceCoverage {
                        surface,
                    },
                );
            }
        }

        if self.summary != self.computed_summary() {
            violations.push(EmbeddedBoundaryAccessibilityViolation::SummaryMismatch);
        }

        if json_contains_forbidden_material(
            &serde_json::to_value(self)
                .expect("embedded-boundary accessibility parity packet serializes"),
        ) {
            violations.push(EmbeddedBoundaryAccessibilityViolation::RawEmbeddedMaterialInExport);
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
            .expect("embedded-boundary accessibility parity packet serializes")
    }

    /// Deterministic CSV of the certified rows for release / support handoff.
    pub fn render_matrix_csv(&self) -> String {
        let mut out = String::from(
            "row_id,component_family,keyboard_reach,screen_reader_reach,cli_reach,export_summary,full_claim,effective_claim,status\n",
        );
        for row in &self.rows {
            out.push_str(&format!(
                "{id},{family},{keyboard},{screen_reader},{cli},{export},{full},{effective},{status}\n",
                id = row.row_id,
                family = row.component_family.as_str(),
                keyboard = row.keyboard_reach.as_str(),
                screen_reader = row.screen_reader_reach.as_str(),
                cli = row.cli_reach.as_str(),
                export = row.export_summary.as_str(),
                full = row.full_support_claim.as_str(),
                effective = row.effective_claim().as_str(),
                status = row.status().as_str(),
            ));
        }
        out
    }

    /// Deterministic Markdown report for support, docs, or release handoff.
    pub fn render_markdown_summary(&self) -> String {
        let mut out = String::new();
        out.push_str("# M5 Embedded-Boundary Component Accessibility & Auto-Narrowing\n\n");
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- As of: `{}`\n", self.as_of));
        out.push_str(&format!(
            "- Families: {} certified across {} / {} frozen families\n",
            self.summary.family_count,
            self.represented_families().len(),
            M5EmbeddedBoundaryComponentFamily::ALL.len(),
        ));
        out.push_str(&format!(
            "- Status: {} green / {} yellow / {} red\n",
            self.summary.green_count, self.summary.yellow_count, self.summary.red_count,
        ));
        out.push_str("\n## Rows\n\n");
        for row in &self.rows {
            out.push_str(&format!(
                "- **{}** ({}) — {}\n",
                row.row_id,
                row.component_family.as_str(),
                row.chip_tokens(),
            ));
            if let Some(narrow) = &row.claim_narrow {
                out.push_str(&format!(
                    "  - Auto-narrow: {} → {} (dimension={}, trigger={}) — {}\n",
                    row.full_support_claim.as_str(),
                    narrow.narrowed_to.as_str(),
                    narrow.binding_dimension.as_str(),
                    narrow.trigger.as_str(),
                    narrow.narrowed_label,
                ));
            }
        }
        out
    }
}

/// Reads and validates the checked-in embedded-boundary-component accessibility parity
/// export.
pub fn current_m5_embedded_boundary_a11y_export(
) -> Result<EmbeddedBoundaryAccessibilityPacket, EmbeddedBoundaryAccessibilityArtifactError> {
    let packet: EmbeddedBoundaryAccessibilityPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/release/m5-embedded-boundary-component-accessibility-proof/support_export.json"
    )))
    .map_err(EmbeddedBoundaryAccessibilityArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(EmbeddedBoundaryAccessibilityArtifactError::Validation(
            violations,
        ))
    }
}

/// Errors emitted when reading the checked-in embedded-boundary-component accessibility
/// parity export.
#[derive(Debug)]
pub enum EmbeddedBoundaryAccessibilityArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<EmbeddedBoundaryAccessibilityViolation>),
}

impl fmt::Display for EmbeddedBoundaryAccessibilityArtifactError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(
                    f,
                    "embedded-boundary accessibility parity export parse failed: {error}"
                )
            }
            Self::Validation(violations) => {
                write!(
                    f,
                    "embedded-boundary accessibility parity export failed validation: {} violation(s)",
                    violations.len()
                )
            }
        }
    }
}

impl Error for EmbeddedBoundaryAccessibilityArtifactError {}

/// The consumer surfaces this lane requires the packet to exercise. The full
/// [`M5EmbeddedConsumerSurface`] set — the support export and product surfaces plus the
/// docs-browser / marketplace / account / remote-dashboard / embedded-webview /
/// auth-handoff surfaces where an embedded-boundary component is embedded.
pub const M5_EMBEDDED_BOUNDARY_A11Y_CONSUMER_SURFACES: [M5EmbeddedConsumerSurface; 8] =
    M5EmbeddedConsumerSurface::ALL;

/// Validation failure for M05-1073 embedded-boundary-component accessibility parity
/// packets.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EmbeddedBoundaryAccessibilityViolation {
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
    MissingPrimaryDimension {
        id: String,
        dimension: M5EmbeddedClaimDimension,
    },
    MissingMandatoryLabel {
        id: String,
    },
    HierarchyHeavyMissingStructured {
        id: String,
    },
    ClaimOverAsserted {
        id: String,
    },
    AssistiveTechStranded {
        id: String,
    },
    ExportRequiresScreenshot {
        id: String,
    },
    NarrowingDropsContextSilently {
        id: String,
    },
    MissingConsumerParity {
        id: String,
    },
    StrandedRow {
        id: String,
    },
    MissingFamilyCoverage {
        family: M5EmbeddedBoundaryComponentFamily,
    },
    MissingDimensionCoverage {
        dimension: M5EmbeddedClaimDimension,
    },
    MissingClaimTierCoverage {
        claim: M5EmbeddedAccessClaim,
    },
    MissingConsumerSurfaceCoverage {
        surface: M5EmbeddedConsumerSurface,
    },
    SummaryMismatch,
    RawEmbeddedMaterialInExport,
}

impl fmt::Display for EmbeddedBoundaryAccessibilityViolation {
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
            Self::IncompleteRow { id } => write!(f, "incomplete accessibility row: {id}"),
            Self::MissingPrimaryDimension { id, dimension } => {
                write!(
                    f,
                    "row {id} does not model its family's primary dimension {}",
                    dimension.as_str()
                )
            }
            Self::MissingMandatoryLabel { id } => {
                write!(f, "row {id} drops a mandatory embedded-boundary label")
            }
            Self::HierarchyHeavyMissingStructured { id } => {
                write!(
                    f,
                    "hierarchy-heavy row {id} does not render a structured modality"
                )
            }
            Self::ClaimOverAsserted { id } => {
                write!(
                    f,
                    "row {id} over-asserts fresh first-party truth for a weakened one, or narrows spuriously"
                )
            }
            Self::AssistiveTechStranded { id } => {
                write!(
                    f,
                    "row {id} strands keyboard / assistive-tech / CLI users from the canonical truth"
                )
            }
            Self::ExportRequiresScreenshot { id } => {
                write!(
                    f,
                    "row {id} export cannot preserve meaning without a screenshot"
                )
            }
            Self::NarrowingDropsContextSilently { id } => {
                write!(
                    f,
                    "row {id} narrows a rendering surface without disclosing it"
                )
            }
            Self::MissingConsumerParity { id } => {
                write!(f, "row {id} is missing secondary consumer parity")
            }
            Self::StrandedRow { id } => write!(f, "row {id} is stranded (red) and may not ship"),
            Self::MissingFamilyCoverage { family } => {
                write!(
                    f,
                    "component family {family:?} is not certified in the packet"
                )
            }
            Self::MissingDimensionCoverage { dimension } => {
                write!(
                    f,
                    "claim dimension {} is not exercised in the packet",
                    dimension.as_str()
                )
            }
            Self::MissingClaimTierCoverage { claim } => {
                write!(
                    f,
                    "support claim tier {} does not appear as an effective claim",
                    claim.as_str()
                )
            }
            Self::MissingConsumerSurfaceCoverage { surface } => {
                write!(
                    f,
                    "consumer surface {} does not ingest any row in the packet",
                    surface.as_str()
                )
            }
            Self::SummaryMismatch => write!(f, "computed summary does not match stored summary"),
            Self::RawEmbeddedMaterialInExport => {
                write!(f, "export contains raw embedded material")
            }
        }
    }
}

impl Error for EmbeddedBoundaryAccessibilityViolation {}

/// Whether a narrowed label is a generic non-answer rather than a precise label.
fn label_is_generic(label: &str) -> bool {
    let trimmed = label.trim();
    if trimmed.is_empty() {
        return true;
    }
    let lower = trimmed.to_lowercase();
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
            | "offline"
            | "stale"
            | "blocked"
            | "loading"
            | "content"
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

/// Builds the canonical, checked-in embedded-boundary-component accessibility parity
/// packet. This is the one source of truth shared by the tests, the artifact writer, and
/// the on-disk support export so all three stay byte-aligned.
pub fn seeded_m5_embedded_boundary_a11y_packet() -> EmbeddedBoundaryAccessibilityPacket {
    EmbeddedBoundaryAccessibilityPacket::new(EmbeddedBoundaryAccessibilityPacketInput {
        packet_id: "m5-embedded-boundary-component-accessibility-parity:stable:0001".to_owned(),
        as_of: "2026-07-10T00:00:00Z".to_owned(),
        matrix_ref: EMBEDDED_BOUNDARY_A11Y_COMPONENT_MATRIX_REF.to_owned(),
        rows: seeded_rows(),
    })
}

fn ev(id: &str) -> Vec<String> {
    vec![format!("evidence:embedded-boundary-a11y:{id}")]
}

fn all_required_labels() -> Vec<M5EmbeddedRequiredLabel> {
    M5EmbeddedRequiredLabel::ALL.to_vec()
}

fn copy_export(fields: &[&str]) -> EmbeddedCopyExportParity {
    EmbeddedCopyExportParity {
        formats: vec!["text".to_owned(), "json".to_owned(), "markdown".to_owned()],
        export_fields: fields.iter().map(|f| (*f).to_owned()).collect(),
        screenshot_only_prohibited: true,
    }
}

fn condition(
    dimension: M5EmbeddedClaimDimension,
    state: M5EmbeddedConditionState,
) -> EmbeddedClaimConditionEntry {
    EmbeddedClaimConditionEntry { dimension, state }
}

/// The two consumer surfaces every row ships to at minimum — support/export replay and
/// the general product UI — so the narrowed state always reaches field triage.
fn base_consumers(extra: &[M5EmbeddedConsumerSurface]) -> Vec<M5EmbeddedConsumerSurface> {
    let mut out = vec![
        M5EmbeddedConsumerSurface::SupportExport,
        M5EmbeddedConsumerSurface::ProductUi,
    ];
    out.extend_from_slice(extra);
    out
}

/// Disclosures for the CLI-headless and support-export surfaces. A green (full parity)
/// row keeps full label and summary parity on the narrower surfaces; a narrowed row
/// discloses the reduced interactions it drops there.
fn surface_disclosures(
    labels: &[&str],
    state: EmbeddedNarrowingDisclosureState,
) -> Vec<EmbeddedRenderingNarrowingDisclosure> {
    let preserved: Vec<String> = labels.iter().map(|l| (*l).to_owned()).collect();
    vec![
        EmbeddedRenderingNarrowingDisclosure {
            rendering_surface: M5EmbeddedRenderingSurface::CliHeadless,
            state,
            preserved_labels: preserved.clone(),
            reduced_interactions: vec!["pointer_interaction".to_owned()],
        },
        EmbeddedRenderingNarrowingDisclosure {
            rendering_surface: M5EmbeddedRenderingSurface::SupportExport,
            state,
            preserved_labels: preserved,
            reduced_interactions: vec!["live_action".to_owned()],
        },
    ]
}

/// Disclosures for a full-parity (green) row: the narrower surfaces preserve full label
/// and summary parity.
fn parity_surfaces(labels: &[&str]) -> Vec<EmbeddedRenderingNarrowingDisclosure> {
    surface_disclosures(labels, EmbeddedNarrowingDisclosureState::ParityPreserved)
}

/// Disclosures for a narrowed (yellow) row: the narrower surfaces disclose their reduced
/// interactions while preserving labels.
fn narrowed_surfaces(labels: &[&str]) -> Vec<EmbeddedRenderingNarrowingDisclosure> {
    surface_disclosures(labels, EmbeddedNarrowingDisclosureState::DisclosedNarrowed)
}

fn rendering_surfaces() -> Vec<M5EmbeddedRenderingSurface> {
    vec![
        M5EmbeddedRenderingSurface::DesktopFull,
        M5EmbeddedRenderingSurface::CliHeadless,
        M5EmbeddedRenderingSurface::SupportExport,
    ]
}

fn seeded_rows() -> Vec<EmbeddedBoundaryAccessibilityRow> {
    vec![
        // Docs-pane header — the source class / version / owner-origin / last-updated
        // freshness of the rendered docs is resolved and current; the header offers a
        // fully live, first-party-local docs truth reachable on every surface (green).
        EmbeddedBoundaryAccessibilityRow {
            record_kind: EMBEDDED_BOUNDARY_A11Y_ROW_RECORD_KIND.to_owned(),
            schema_version: EMBEDDED_BOUNDARY_A11Y_SCHEMA_VERSION,
            row_id: "a11y:docs-pane-header".to_owned(),
            component_family: M5EmbeddedBoundaryComponentFamily::DocsPaneHeader,
            source_family_schema_ref: EMBEDDED_BOUNDARY_A11Y_COMPONENT_MATRIX_REF.to_owned(),
            boundary_context_ref: "embedded-boundary:docs-pane-header:0001".to_owned(),
            fallback_modalities: vec![
                M5EmbeddedFallbackModality::List,
                M5EmbeddedFallbackModality::Textual,
                M5EmbeddedFallbackModality::Cli,
            ],
            reaches_canonical_truth: true,
            keyboard_reach: EmbeddedNonVisualReachState::ReachableAndLabeled,
            screen_reader_reach: EmbeddedNonVisualReachState::ReachableAndLabeled,
            cli_reach: EmbeddedNonVisualReachState::ReachableAndLabeled,
            export_summary: EmbeddedExportSummaryState::ReconstructableWithoutScreenshot,
            export_summary_ref: "summary:docs-pane-header:a11y".to_owned(),
            copy_export: copy_export(&[
                "source_class",
                "owner_origin",
                "last_updated",
                "open_externally",
            ]),
            full_support_claim: M5EmbeddedAccessClaim::FullTruth,
            claim_conditions: vec![condition(
                M5EmbeddedClaimDimension::FreshnessTruth,
                M5EmbeddedConditionState::Intact,
            )],
            claim_narrow: None,
            rendering_surfaces: rendering_surfaces(),
            narrowing_disclosures: parity_surfaces(&[
                "source_class",
                "owner_origin",
                "last_updated",
            ]),
            required_labels: all_required_labels(),
            consumer_surfaces: base_consumers(&[M5EmbeddedConsumerSurface::DocsBrowserUi]),
            source_refs: vec![
                "TDD §7.3.6 documentation browser / docs-integrity browser handoff".to_owned(),
                EMBEDDED_BOUNDARY_A11Y_DOC_REF.to_owned(),
            ],
            observed_at: "2026-07-10T00:00:00Z".to_owned(),
            evidence_refs: ev("docs-pane-header"),
        },
        // Embedded-origin bar — the extension / publisher / origin / permission truth is
        // fully resolved and labelled; the bar reports a ready, self-sufficient owner /
        // origin chrome (never a live-adapting first-party-local stream) (green).
        EmbeddedBoundaryAccessibilityRow {
            record_kind: EMBEDDED_BOUNDARY_A11Y_ROW_RECORD_KIND.to_owned(),
            schema_version: EMBEDDED_BOUNDARY_A11Y_SCHEMA_VERSION,
            row_id: "a11y:embedded-origin-bar".to_owned(),
            component_family: M5EmbeddedBoundaryComponentFamily::EmbeddedOriginBar,
            source_family_schema_ref: EMBEDDED_BOUNDARY_A11Y_COMPONENT_MATRIX_REF.to_owned(),
            boundary_context_ref: "embedded-boundary:embedded-origin-bar:0002".to_owned(),
            fallback_modalities: vec![
                M5EmbeddedFallbackModality::List,
                M5EmbeddedFallbackModality::Textual,
                M5EmbeddedFallbackModality::Cli,
            ],
            reaches_canonical_truth: true,
            keyboard_reach: EmbeddedNonVisualReachState::ReachableAndLabeled,
            screen_reader_reach: EmbeddedNonVisualReachState::ReachableAndLabeled,
            cli_reach: EmbeddedNonVisualReachState::ReachableAndLabeled,
            export_summary: EmbeddedExportSummaryState::ReconstructableWithoutScreenshot,
            export_summary_ref: "summary:embedded-origin-bar:a11y".to_owned(),
            copy_export: copy_export(&[
                "extension_publisher",
                "origin",
                "permission_scope",
                "capability_limits",
            ]),
            full_support_claim: M5EmbeddedAccessClaim::ResolvedTruth,
            claim_conditions: vec![condition(
                M5EmbeddedClaimDimension::OwnerOriginTruth,
                M5EmbeddedConditionState::Intact,
            )],
            claim_narrow: None,
            rendering_surfaces: rendering_surfaces(),
            narrowing_disclosures: parity_surfaces(&[
                "extension_publisher",
                "origin",
                "permission_scope",
            ]),
            required_labels: all_required_labels(),
            consumer_surfaces: base_consumers(&[M5EmbeddedConsumerSurface::EmbeddedWebviewUi]),
            source_refs: vec![
                "UI/UX Spec §21.14 extension-authored webviews / dashboards".to_owned(),
                EMBEDDED_BOUNDARY_A11Y_DOC_REF.to_owned(),
            ],
            observed_at: "2026-07-10T00:00:00Z".to_owned(),
            evidence_refs: ev("embedded-origin-bar"),
        },
        // Boundary-fact grid — hierarchy-heavy (nested owner/origin / data-boundary /
        // freshness grid); the data-exit boundary is only partially resolved (mirror /
        // provider path still settling), so the grid auto-narrows to degraded rather than
        // reading as fresh first-party truth and binds its grid to a flat list / textual
        // path (yellow).
        EmbeddedBoundaryAccessibilityRow {
            record_kind: EMBEDDED_BOUNDARY_A11Y_ROW_RECORD_KIND.to_owned(),
            schema_version: EMBEDDED_BOUNDARY_A11Y_SCHEMA_VERSION,
            row_id: "a11y:boundary-fact-grid".to_owned(),
            component_family: M5EmbeddedBoundaryComponentFamily::BoundaryFactGrid,
            source_family_schema_ref: EMBEDDED_BOUNDARY_A11Y_COMPONENT_MATRIX_REF.to_owned(),
            boundary_context_ref: "embedded-boundary:boundary-fact-grid:0003".to_owned(),
            fallback_modalities: vec![
                M5EmbeddedFallbackModality::Structured,
                M5EmbeddedFallbackModality::List,
                M5EmbeddedFallbackModality::Textual,
                M5EmbeddedFallbackModality::Cli,
            ],
            reaches_canonical_truth: true,
            keyboard_reach: EmbeddedNonVisualReachState::ReachableAndLabeled,
            screen_reader_reach: EmbeddedNonVisualReachState::DisclosedReducedButReachable,
            cli_reach: EmbeddedNonVisualReachState::ReachableAndLabeled,
            export_summary: EmbeddedExportSummaryState::ReconstructableWithoutScreenshot,
            export_summary_ref: "summary:boundary-fact-grid:a11y".to_owned(),
            copy_export: copy_export(&[
                "owner_origin",
                "data_boundary",
                "freshness",
                "open_externally",
            ]),
            full_support_claim: M5EmbeddedAccessClaim::FullTruth,
            claim_conditions: vec![condition(
                M5EmbeddedClaimDimension::DataBoundaryTruth,
                M5EmbeddedConditionState::Partial,
            )],
            claim_narrow: Some(EmbeddedClaimAutoNarrow {
                narrowed_to: M5EmbeddedAccessClaim::Degraded,
                binding_dimension: M5EmbeddedClaimDimension::DataBoundaryTruth,
                trigger: M5EmbeddedDowngradeTrigger::DataBoundaryUnstated,
                narrowed_label:
                    "Data boundary partially resolved — grid shown degraded until the mirror-versus-provider exit path settles"
                        .to_owned(),
                preserves_canonical_identity: true,
            }),
            rendering_surfaces: rendering_surfaces(),
            narrowing_disclosures: narrowed_surfaces(&[
                "owner_origin",
                "data_boundary",
                "freshness",
            ]),
            required_labels: all_required_labels(),
            consumer_surfaces: base_consumers(&[M5EmbeddedConsumerSurface::DocsBrowserUi]),
            source_refs: vec![
                "TAD §9.9 scoped browser and web-surface architecture".to_owned(),
                EMBEDDED_BOUNDARY_A11Y_DOC_REF.to_owned(),
            ],
            observed_at: "2026-07-10T00:00:00Z".to_owned(),
            evidence_refs: ev("boundary-fact-grid"),
        },
        // Marketplace/account boundary card — the account scope is only partially resolved
        // (org-versus-managed-tenant attribution still resolving), so the card auto-narrows
        // to degraded rather than reading as a fully-attributed account scope (yellow).
        EmbeddedBoundaryAccessibilityRow {
            record_kind: EMBEDDED_BOUNDARY_A11Y_ROW_RECORD_KIND.to_owned(),
            schema_version: EMBEDDED_BOUNDARY_A11Y_SCHEMA_VERSION,
            row_id: "a11y:marketplace-account-boundary-card".to_owned(),
            component_family: M5EmbeddedBoundaryComponentFamily::MarketplaceAccountBoundaryCard,
            source_family_schema_ref: EMBEDDED_BOUNDARY_A11Y_COMPONENT_MATRIX_REF.to_owned(),
            boundary_context_ref: "embedded-boundary:marketplace-account-boundary-card:0004"
                .to_owned(),
            fallback_modalities: vec![
                M5EmbeddedFallbackModality::List,
                M5EmbeddedFallbackModality::Textual,
                M5EmbeddedFallbackModality::Cli,
            ],
            reaches_canonical_truth: true,
            keyboard_reach: EmbeddedNonVisualReachState::ReachableAndLabeled,
            screen_reader_reach: EmbeddedNonVisualReachState::ReachableAndLabeled,
            cli_reach: EmbeddedNonVisualReachState::ReachableAndLabeled,
            export_summary: EmbeddedExportSummaryState::ReconstructableWithoutScreenshot,
            export_summary_ref: "summary:marketplace-account-boundary-card:a11y".to_owned(),
            copy_export: copy_export(&[
                "origin",
                "account_scope",
                "current_profile",
                "data_boundary",
            ]),
            full_support_claim: M5EmbeddedAccessClaim::ResolvedTruth,
            claim_conditions: vec![condition(
                M5EmbeddedClaimDimension::AccountScopeTruth,
                M5EmbeddedConditionState::Partial,
            )],
            claim_narrow: Some(EmbeddedClaimAutoNarrow {
                narrowed_to: M5EmbeddedAccessClaim::Degraded,
                binding_dimension: M5EmbeddedClaimDimension::AccountScopeTruth,
                trigger: M5EmbeddedDowngradeTrigger::AccountScopeUnstated,
                narrowed_label:
                    "Account scope partially resolved — shown degraded until the org-versus-managed-tenant profile resolves"
                        .to_owned(),
                preserves_canonical_identity: true,
            }),
            rendering_surfaces: rendering_surfaces(),
            narrowing_disclosures: narrowed_surfaces(&[
                "origin",
                "account_scope",
                "current_profile",
            ]),
            required_labels: all_required_labels(),
            consumer_surfaces: base_consumers(&[
                M5EmbeddedConsumerSurface::MarketplaceUi,
                M5EmbeddedConsumerSurface::AccountUi,
            ]),
            source_refs: vec![
                "UI/UX Spec §21.14 marketplace / account hosted-surface guidance".to_owned(),
                EMBEDDED_BOUNDARY_A11Y_DOC_REF.to_owned(),
            ],
            observed_at: "2026-07-10T00:00:00Z".to_owned(),
            evidence_refs: ev("marketplace-account-boundary-card"),
        },
        // Remote/service dashboard header — the provider snapshot is stale beyond the
        // freshness grace window and deliberately kept visible pending refresh, so the
        // header auto-narrows to stale rather than reading as a fresh first-party value
        // (yellow).
        EmbeddedBoundaryAccessibilityRow {
            record_kind: EMBEDDED_BOUNDARY_A11Y_ROW_RECORD_KIND.to_owned(),
            schema_version: EMBEDDED_BOUNDARY_A11Y_SCHEMA_VERSION,
            row_id: "a11y:remote-service-dashboard-header".to_owned(),
            component_family: M5EmbeddedBoundaryComponentFamily::RemoteServiceDashboardHeader,
            source_family_schema_ref: EMBEDDED_BOUNDARY_A11Y_COMPONENT_MATRIX_REF.to_owned(),
            boundary_context_ref: "embedded-boundary:remote-service-dashboard-header:0005"
                .to_owned(),
            fallback_modalities: vec![
                M5EmbeddedFallbackModality::List,
                M5EmbeddedFallbackModality::Textual,
                M5EmbeddedFallbackModality::Cli,
            ],
            reaches_canonical_truth: true,
            keyboard_reach: EmbeddedNonVisualReachState::ReachableAndLabeled,
            screen_reader_reach: EmbeddedNonVisualReachState::ReachableAndLabeled,
            cli_reach: EmbeddedNonVisualReachState::ReachableAndLabeled,
            export_summary: EmbeddedExportSummaryState::ReconstructableWithoutScreenshot,
            export_summary_ref: "summary:remote-service-dashboard-header:a11y".to_owned(),
            copy_export: copy_export(&[
                "target_service_identity",
                "freshness",
                "provider_health",
                "open_console",
            ]),
            full_support_claim: M5EmbeddedAccessClaim::FullTruth,
            claim_conditions: vec![condition(
                M5EmbeddedClaimDimension::FreshnessTruth,
                M5EmbeddedConditionState::Stale,
            )],
            claim_narrow: Some(EmbeddedClaimAutoNarrow {
                narrowed_to: M5EmbeddedAccessClaim::Stale,
                binding_dimension: M5EmbeddedClaimDimension::FreshnessTruth,
                trigger: M5EmbeddedDowngradeTrigger::FreshnessOrLastUpdatedUnstated,
                narrowed_label:
                    "Provider snapshot stale — shown as a stale snapshot with its last-updated time, not a fresh first-party value, pending refresh"
                        .to_owned(),
                preserves_canonical_identity: true,
            }),
            rendering_surfaces: rendering_surfaces(),
            narrowing_disclosures: narrowed_surfaces(&[
                "target_service_identity",
                "freshness",
                "provider_health",
            ]),
            required_labels: all_required_labels(),
            consumer_surfaces: base_consumers(&[M5EmbeddedConsumerSurface::RemoteDashboardUi]),
            source_refs: vec![
                "Milestones v3.1 embedded docs / webview / auth-handoff owner-origin model".to_owned(),
                EMBEDDED_BOUNDARY_A11Y_DOC_REF.to_owned(),
            ],
            observed_at: "2026-07-10T00:00:00Z".to_owned(),
            evidence_refs: ev("remote-service-dashboard-header"),
        },
        // Auth-handoff card — the browser / device-code fallback path is offline (no
        // network to reach the browser handoff), so the card auto-narrows to offline
        // rather than reading as an available live sign-in, while preserving the
        // local-safe continuity identity (yellow).
        EmbeddedBoundaryAccessibilityRow {
            record_kind: EMBEDDED_BOUNDARY_A11Y_ROW_RECORD_KIND.to_owned(),
            schema_version: EMBEDDED_BOUNDARY_A11Y_SCHEMA_VERSION,
            row_id: "a11y:auth-handoff-card".to_owned(),
            component_family: M5EmbeddedBoundaryComponentFamily::AuthHandoffCard,
            source_family_schema_ref: EMBEDDED_BOUNDARY_A11Y_COMPONENT_MATRIX_REF.to_owned(),
            boundary_context_ref: "embedded-boundary:auth-handoff-card:0006".to_owned(),
            fallback_modalities: vec![
                M5EmbeddedFallbackModality::List,
                M5EmbeddedFallbackModality::Textual,
                M5EmbeddedFallbackModality::Cli,
            ],
            reaches_canonical_truth: true,
            keyboard_reach: EmbeddedNonVisualReachState::ReachableAndLabeled,
            screen_reader_reach: EmbeddedNonVisualReachState::ReachableAndLabeled,
            cli_reach: EmbeddedNonVisualReachState::ReachableAndLabeled,
            export_summary: EmbeddedExportSummaryState::ReconstructableWithoutScreenshot,
            export_summary_ref: "summary:auth-handoff-card:a11y".to_owned(),
            copy_export: copy_export(&[
                "provider_domain",
                "handoff_reason",
                "browser_fallback",
                "local_safe_continuity",
            ]),
            full_support_claim: M5EmbeddedAccessClaim::ResolvedTruth,
            claim_conditions: vec![condition(
                M5EmbeddedClaimDimension::BrowserFallbackTruth,
                M5EmbeddedConditionState::Offline,
            )],
            claim_narrow: Some(EmbeddedClaimAutoNarrow {
                narrowed_to: M5EmbeddedAccessClaim::Offline,
                binding_dimension: M5EmbeddedClaimDimension::BrowserFallbackTruth,
                trigger: M5EmbeddedDowngradeTrigger::BrowserFallbackHiddenInMenusOnly,
                narrowed_label:
                    "Browser handoff offline — shown as offline with local-safe continuity intact, not an available live sign-in, until the network returns"
                        .to_owned(),
                preserves_canonical_identity: true,
            }),
            rendering_surfaces: rendering_surfaces(),
            narrowing_disclosures: narrowed_surfaces(&[
                "provider_domain",
                "handoff_reason",
                "browser_fallback",
            ]),
            required_labels: all_required_labels(),
            consumer_surfaces: base_consumers(&[M5EmbeddedConsumerSurface::AuthHandoffUi]),
            source_refs: vec![
                "UI/UX Spec §18.44 embedded-auth boundary honesty".to_owned(),
                EMBEDDED_BOUNDARY_A11Y_DOC_REF.to_owned(),
            ],
            observed_at: "2026-07-10T00:00:00Z".to_owned(),
            evidence_refs: ev("auth-handoff-card"),
        },
        // Open-in-browser handoff row — the browser fallback is offline (no reachable
        // network path), so the row auto-narrows to offline rather than dropping the user
        // onto a generic landing page, while preserving the object identity and
        // reason-for-handoff (yellow).
        EmbeddedBoundaryAccessibilityRow {
            record_kind: EMBEDDED_BOUNDARY_A11Y_ROW_RECORD_KIND.to_owned(),
            schema_version: EMBEDDED_BOUNDARY_A11Y_SCHEMA_VERSION,
            row_id: "a11y:open-in-browser-handoff-row".to_owned(),
            component_family: M5EmbeddedBoundaryComponentFamily::OpenInBrowserHandoffRow,
            source_family_schema_ref: EMBEDDED_BOUNDARY_A11Y_COMPONENT_MATRIX_REF.to_owned(),
            boundary_context_ref: "embedded-boundary:open-in-browser-handoff-row:0007".to_owned(),
            fallback_modalities: vec![
                M5EmbeddedFallbackModality::List,
                M5EmbeddedFallbackModality::Textual,
                M5EmbeddedFallbackModality::Cli,
            ],
            reaches_canonical_truth: true,
            keyboard_reach: EmbeddedNonVisualReachState::ReachableAndLabeled,
            screen_reader_reach: EmbeddedNonVisualReachState::ReachableAndLabeled,
            cli_reach: EmbeddedNonVisualReachState::ReachableAndLabeled,
            export_summary: EmbeddedExportSummaryState::ReconstructableWithoutScreenshot,
            export_summary_ref: "summary:open-in-browser-handoff-row:a11y".to_owned(),
            copy_export: copy_export(&[
                "object_identity",
                "handoff_reason",
                "browser_fallback",
                "local_safe_continuity",
            ]),
            full_support_claim: M5EmbeddedAccessClaim::ResolvedTruth,
            claim_conditions: vec![condition(
                M5EmbeddedClaimDimension::BrowserFallbackTruth,
                M5EmbeddedConditionState::Offline,
            )],
            claim_narrow: Some(EmbeddedClaimAutoNarrow {
                narrowed_to: M5EmbeddedAccessClaim::Offline,
                binding_dimension: M5EmbeddedClaimDimension::BrowserFallbackTruth,
                trigger: M5EmbeddedDowngradeTrigger::BrowserFallbackHiddenInMenusOnly,
                narrowed_label:
                    "Open-in-browser offline — shown as offline with the object identity and reason-for-handoff preserved, not a generic landing page, until the network returns"
                        .to_owned(),
                preserves_canonical_identity: true,
            }),
            rendering_surfaces: rendering_surfaces(),
            narrowing_disclosures: narrowed_surfaces(&[
                "object_identity",
                "handoff_reason",
                "browser_fallback",
            ]),
            required_labels: all_required_labels(),
            consumer_surfaces: base_consumers(&[M5EmbeddedConsumerSurface::AuthHandoffUi]),
            source_refs: vec![
                "UX Design System §16.19 embedded webviews / auth handoff".to_owned(),
                EMBEDDED_BOUNDARY_A11Y_DOC_REF.to_owned(),
            ],
            observed_at: "2026-07-10T00:00:00Z".to_owned(),
            evidence_refs: ev("open-in-browser-handoff-row"),
        },
        // Embedded-state panel — a provider or policy blocks the embedded content, so the
        // panel auto-narrows to provider-blocked rather than presenting the blocked pane as
        // fresh first-party truth, and never imitates native permission chrome (yellow).
        EmbeddedBoundaryAccessibilityRow {
            record_kind: EMBEDDED_BOUNDARY_A11Y_ROW_RECORD_KIND.to_owned(),
            schema_version: EMBEDDED_BOUNDARY_A11Y_SCHEMA_VERSION,
            row_id: "a11y:embedded-state-panel".to_owned(),
            component_family: M5EmbeddedBoundaryComponentFamily::EmbeddedStatePanel,
            source_family_schema_ref: EMBEDDED_BOUNDARY_A11Y_COMPONENT_MATRIX_REF.to_owned(),
            boundary_context_ref: "embedded-boundary:embedded-state-panel:0008".to_owned(),
            fallback_modalities: vec![
                M5EmbeddedFallbackModality::List,
                M5EmbeddedFallbackModality::Textual,
                M5EmbeddedFallbackModality::Cli,
            ],
            reaches_canonical_truth: true,
            keyboard_reach: EmbeddedNonVisualReachState::ReachableAndLabeled,
            screen_reader_reach: EmbeddedNonVisualReachState::ReachableAndLabeled,
            cli_reach: EmbeddedNonVisualReachState::ReachableAndLabeled,
            export_summary: EmbeddedExportSummaryState::ReconstructableWithoutScreenshot,
            export_summary_ref: "summary:embedded-state-panel:a11y".to_owned(),
            copy_export: copy_export(&[
                "embedded_state",
                "capability_limits",
                "owner_origin",
                "recovery_path",
            ]),
            full_support_claim: M5EmbeddedAccessClaim::ResolvedTruth,
            claim_conditions: vec![condition(
                M5EmbeddedClaimDimension::CapabilityLimitTruth,
                M5EmbeddedConditionState::ProviderBlocked,
            )],
            claim_narrow: Some(EmbeddedClaimAutoNarrow {
                narrowed_to: M5EmbeddedAccessClaim::ProviderBlocked,
                binding_dimension: M5EmbeddedClaimDimension::CapabilityLimitTruth,
                trigger: M5EmbeddedDowngradeTrigger::CapabilityLimitsUnstated,
                narrowed_label:
                    "Embedded content provider-blocked — shown as blocked-by-provider with its capability limits named, not fresh first-party truth, and never imitating native permission chrome"
                        .to_owned(),
                preserves_canonical_identity: true,
            }),
            rendering_surfaces: rendering_surfaces(),
            narrowing_disclosures: narrowed_surfaces(&[
                "embedded_state",
                "capability_limits",
                "owner_origin",
            ]),
            required_labels: all_required_labels(),
            consumer_surfaces: base_consumers(&[M5EmbeddedConsumerSurface::EmbeddedWebviewUi]),
            source_refs: vec![
                "UI/UX Spec §21.14 extension-authored webviews / dashboards".to_owned(),
                EMBEDDED_BOUNDARY_A11Y_DOC_REF.to_owned(),
            ],
            observed_at: "2026-07-10T00:00:00Z".to_owned(),
            evidence_refs: ev("embedded-state-panel"),
        },
    ]
}
