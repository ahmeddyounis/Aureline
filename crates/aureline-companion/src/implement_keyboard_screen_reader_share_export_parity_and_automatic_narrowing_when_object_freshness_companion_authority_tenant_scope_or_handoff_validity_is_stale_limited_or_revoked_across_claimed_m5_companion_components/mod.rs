//! Keyboard / screen-reader / share-export parity and honest automatic narrowing for the
//! M5 companion components (notification row, mobile review card, CI-status card, session-follow
//! tile, incident-snapshot card, and desktop-handoff sheet).
//!
//! This module is the M05-1002 accessibility-and-auto-narrowing capstone over the frozen M5
//! companion component matrix ([`crate::freeze_the_m5_companion_component_matrix`]). Where the
//! freeze matrix defines the reusable companion-client component primitives, and the 997-999
//! implementation lanes plus the 1000-1001 degraded-state / consumer lanes resolve their
//! per-surface truth, this lane certifies — per component family — that companion claims stay
//! **keyboard-complete, assistive-tech-reachable, share/export-safe, and self-narrowing** rather
//! than presenting a stale object, a limited companion authority, a narrowed tenant scope, or a
//! revoked handoff as a still live, in-authority, fully companion-safe surface:
//!
//! - **Keyboard / screen-reader / CLI reach.** Every family exposes a keyboard-complete,
//!   screen-reader-reachable, and CLI/headless-reachable path into the same canonical object
//!   identity, client scope, freshness, companion-versus-desktop capability boundary, severity,
//!   and exact desktop-handoff target the rich component shows — never a hover-only chip that
//!   strands assistive-tech or headless users. Hierarchy-heavy families (the incident-snapshot
//!   card's nested service / run / severity / status lineage) additionally bind their tree to a
//!   flat list / textual path.
//! - **Share / export parity.** The support / notification-export / issue-report share
//!   reconstructs each component's meaning from typed tokens and opaque refs without a screenshot,
//!   preserving the same canonical IDs, client scopes, freshness classes, capability boundaries,
//!   handoff targets, and narrowing reasons shown in-product so companion truth can be
//!   reconstructed without screenshots or private team memory — and never a raw payload body.
//! - **Honest auto-narrowing.** When object freshness is stale, companion authority is limited,
//!   tenant scope has narrowed, or handoff validity is revoked, the component's companion claim
//!   auto-narrows from `LiveCompanionSafe` / `CachedContinuitySafe` to a stale-freshness /
//!   limited-authority / narrowed-tenant / revoked-handoff projection, discloses the narrowing
//!   with a precise trigger and binding dimension, and preserves the canonical identity / scope /
//!   freshness / handoff lineage — the underlying object lineage is never dropped opaquely. A
//!   component with every dimension intact must NOT carry a spurious narrowing, and a stale,
//!   limited, or revoked state can never keep a live-companion-safe claim.
//! - **Cross-surface disclosure.** The same narrowed state surfaces in the notification-triage,
//!   review-queue, CI-status, session-follow, incident-awareness, desktop-handoff, status-bar,
//!   general product UI, and support / export share so product, docs, and release publication stay
//!   aligned on companion-boundary downgrade behavior rather than drifting in copy — a live-looking
//!   surface can never outrun the freshness / authority / tenant / handoff proof it is being viewed
//!   away from.
//!
//! Each [`CompanionComponentAccessibilityRow`] keys on one
//! [`crate::freeze_the_m5_companion_component_matrix::M5CompanionComponentFamily`] and reuses that
//! frozen family vocabulary plus the frozen [`M5CompanionRequiredLabel`] and
//! [`M5CompanionDowngradeTrigger`] and the shared [`M5CompanionConsumerSurface`] consumer surfaces
//! rather than minting parallel synonyms, so the certified labels stay byte-identical to the matrix
//! and the sibling primitive packets.
//!
//! The packet is metadata-only: raw payload bodies, message contents, request bodies, and endpoint
//! secrets never cross this boundary; the packet carries only typed class tokens, opaque object /
//! scope / handoff refs, booleans, and redacted labels so support, release, and diagnostics shares
//! can reconstruct exactly what an accessible fallback would have shown without leaking object
//! contents.

#[cfg(test)]
mod tests;

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

// Reused frozen component vocabulary — the capstone certifies the freeze matrix's families,
// required labels, downgrade triggers, and consumer surfaces rather than mint parallel ones.
use crate::freeze_the_m5_companion_component_matrix::{
    M5CompanionComponentFamily, M5CompanionConsumerSurface, M5CompanionDowngradeTrigger,
    M5CompanionRequiredLabel, M5_COMPANION_COMPONENT_SCHEMA_REF,
};

/// Schema version stamped on the M05-1002 companion component accessibility parity packet.
pub const COMPANION_COMPONENT_A11Y_SCHEMA_VERSION: u32 = 1;

/// Stable record-kind tag carried by [`CompanionComponentAccessibilityPacket`].
pub const COMPANION_COMPONENT_A11Y_RECORD_KIND: &str =
    "m5_companion_component_accessibility_parity_packet";

/// Stable record-kind tag carried by each [`CompanionComponentAccessibilityRow`].
pub const COMPANION_COMPONENT_A11Y_ROW_RECORD_KIND: &str =
    "m5_companion_component_accessibility_parity_row";

/// Repo-relative path of the boundary schema.
pub const COMPANION_COMPONENT_A11Y_SCHEMA_REF: &str =
    "schemas/ui/m5-companion-component-accessibility-parity.schema.json";

/// Repo-relative path of the contract doc.
pub const COMPANION_COMPONENT_A11Y_DOC_REF: &str =
    "docs/companion/m5_companion_component_accessibility_parity.md";

/// Repo-relative path of the frozen companion component matrix this lane certifies.
pub const COMPANION_COMPONENT_A11Y_COMPONENT_MATRIX_REF: &str = M5_COMPANION_COMPONENT_SCHEMA_REF;

/// Repo-relative path of the protected fixture directory.
pub const COMPANION_COMPONENT_A11Y_FIXTURE_DIR: &str =
    "fixtures/ui/m5-companion-component-accessibility-parity";

/// Repo-relative path of the checked support-export artifact (the `include_str!` canonical).
pub const COMPANION_COMPONENT_A11Y_ARTIFACT_REF: &str =
    "artifacts/release/m5-companion-component-accessibility-proof/support_export.json";

/// Repo-relative path of the checked machine-readable matrix CSV.
pub const COMPANION_COMPONENT_A11Y_CSV_REF: &str =
    "artifacts/release/m5-companion-component-accessibility-proof/matrix.csv";

/// Repo-relative path of the checked Markdown report.
pub const COMPANION_COMPONENT_A11Y_REPORT_REF: &str =
    "artifacts/release/m5-companion-component-accessibility-proof.md";

/// The reusable component families that render a non-linear hierarchy (the incident-snapshot
/// card's nested service / run / severity / status lineage) and therefore MUST bind their tree to
/// an equivalent flat list / textual path so the hierarchy is navigable non-visually.
const fn family_is_hierarchy_heavy(family: M5CompanionComponentFamily) -> bool {
    matches!(family, M5CompanionComponentFamily::IncidentSnapshotCard)
}

/// The companion dimension whose weakening a family primarily discloses. Every row must model at
/// least this dimension so its key weakening axis is covered.
const fn family_primary_dimension(
    family: M5CompanionComponentFamily,
) -> M5CompanionComponentClaimDimension {
    match family {
        M5CompanionComponentFamily::NotificationRow => {
            M5CompanionComponentClaimDimension::ObjectFreshness
        }
        M5CompanionComponentFamily::MobileReviewCard => {
            M5CompanionComponentClaimDimension::CompanionAuthority
        }
        M5CompanionComponentFamily::CiStatusCard => {
            M5CompanionComponentClaimDimension::ObjectFreshness
        }
        M5CompanionComponentFamily::SessionFollowTile => {
            M5CompanionComponentClaimDimension::TenantScope
        }
        M5CompanionComponentFamily::IncidentSnapshotCard => {
            M5CompanionComponentClaimDimension::ObjectFreshness
        }
        M5CompanionComponentFamily::DesktopHandoffSheet => {
            M5CompanionComponentClaimDimension::HandoffValidity
        }
    }
}

/// A rendered fallback modality for a companion component.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5CompanionComponentFallbackModality {
    /// A rich, structured (nested service / run / severity / status tree) projection.
    Structured,
    /// A flat list projection.
    List,
    /// A textual / source-first projection.
    Textual,
    /// A CLI / headless line projection.
    Cli,
}

impl M5CompanionComponentFallbackModality {
    /// Returns true when the modality is reachable without interpreting a rich, structured
    /// surface (i.e. a keyboard / screen-reader / headless path).
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

/// A rendering-surface capability tier. Distinct from the semantic consumer surface: the same
/// component may render at desktop-full capability or narrow to a companion, read-only browser,
/// headless CLI, offline handoff packet, or support export.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5CompanionComponentRenderingSurface {
    /// The full-capability desktop surface.
    DesktopFull,
    /// The companion app.
    CompanionApp,
    /// A read-only browser projection.
    BrowserReadonly,
    /// A headless CLI surface.
    CliHeadless,
    /// An offline handoff packet.
    HandoffPacket,
    /// A support / release / evaluation export.
    SupportExport,
}

impl M5CompanionComponentRenderingSurface {
    /// Returns true when the surface narrows interactivity below the desktop full-capability
    /// baseline and therefore must disclose its reduction.
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
pub enum CompanionComponentNonVisualReachState {
    /// Fully traversable and labeled with no loss.
    ReachableAndLabeled,
    /// Reachable and labeled, but with a disclosed reduction (yellow).
    DisclosedReducedButReachable,
    /// A view-only / hover-only surface that traps keyboard / assistive-tech / headless users
    /// (red).
    ViewOnlyTrap,
}

impl CompanionComponentNonVisualReachState {
    /// Returns true when the state never strands keyboard / assistive-tech / headless users.
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

/// Whether a share/export-safe summary preserves the component meaning without a screenshot.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CompanionComponentExportSummaryState {
    /// The component meaning reconstructs from the summary without a screenshot.
    ReconstructableWithoutScreenshot,
    /// Partial capture, but disclosed (yellow).
    DisclosedPartialCapture,
    /// The export relies on a screenshot to carry meaning (red).
    AbsentNeedsScreenshot,
}

impl CompanionComponentExportSummaryState {
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
pub enum CompanionComponentNarrowingDisclosureState {
    /// Full label and summary parity with the desktop surface.
    ParityPreserved,
    /// Reduced interactivity, disclosed with preserved labels (yellow).
    DisclosedNarrowed,
    /// Interactivity, state, or actions dropped without disclosure (red).
    SilentlyDropped,
}

impl CompanionComponentNarrowingDisclosureState {
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

/// The companion claim ceiling a component asserts: how strong a live-companion-safe posture it lets
/// a surface present. Auto-narrowing lowers this ceiling when a companion dimension weakens so a
/// stale object, a limited companion authority, a narrowed tenant scope, or a revoked handoff can
/// never keep an old `LiveCompanionSafe` or `CachedContinuitySafe` label — a stale, limited, or
/// revoked state never masquerades as live-companion-safe.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5CompanionComponentClaim {
    /// Live-companion-safe: fresh object data, an in-authority companion capability, an in-scope
    /// tenant, and a resolvable handoff — the strongest claim, a surface the companion can trust,
    /// triage, and (where permitted) act on right now.
    LiveCompanionSafe,
    /// Cached-continuity-safe: a self-sufficient cached / summary-first projection (usable as a
    /// stable, labeled snapshot) that is not itself a live, in-authority companion-safe surface.
    CachedContinuitySafe,
    /// Stale-freshness projection: object freshness has gone stale; the surface must not read as
    /// live and stays a stale-freshness projection until refreshed.
    StaleFreshnessProjection,
    /// Limited-authority projection: companion authority is limited (desktop-required or
    /// read-only); the surface cannot present as a fully companion-safe action surface and stays a
    /// limited-authority projection until widened on desktop.
    LimitedAuthorityProjection,
    /// Narrowed-tenant projection: the tenant / client scope has narrowed from what was granted;
    /// the surface cannot present as an in-scope object and stays a narrowed-tenant projection
    /// until the scope is reconciled.
    NarrowedTenantProjection,
    /// Revoked-handoff projection: the handoff target is revoked or unresolvable; the surface
    /// cannot claim it will open the intended object on desktop and stays a revoked-handoff
    /// projection.
    RevokedHandoffProjection,
}

impl M5CompanionComponentClaim {
    /// Every claim tier, strongest first.
    pub const ALL: [Self; 6] = [
        Self::LiveCompanionSafe,
        Self::CachedContinuitySafe,
        Self::StaleFreshnessProjection,
        Self::LimitedAuthorityProjection,
        Self::NarrowedTenantProjection,
        Self::RevokedHandoffProjection,
    ];

    /// Capability rank; a higher rank asserts a stronger companion posture. Narrowing lowers rank.
    pub const fn capability_rank(self) -> u8 {
        match self {
            Self::LiveCompanionSafe => 5,
            Self::CachedContinuitySafe => 4,
            Self::StaleFreshnessProjection => 3,
            Self::LimitedAuthorityProjection => 2,
            Self::NarrowedTenantProjection => 1,
            Self::RevokedHandoffProjection => 0,
        }
    }

    /// Returns true when this claim asserts a live, in-authority, companion-safe surface.
    pub const fn asserts_live_companion_safe(self) -> bool {
        matches!(self, Self::LiveCompanionSafe)
    }

    /// Returns true when this claim asserts a fully self-sufficient (live-companion-safe or
    /// cached-continuity-safe) projection.
    pub const fn asserts_full_projection(self) -> bool {
        matches!(self, Self::LiveCompanionSafe | Self::CachedContinuitySafe)
    }

    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LiveCompanionSafe => "live_companion_safe",
            Self::CachedContinuitySafe => "cached_continuity_safe",
            Self::StaleFreshnessProjection => "stale_freshness_projection",
            Self::LimitedAuthorityProjection => "limited_authority_projection",
            Self::NarrowedTenantProjection => "narrowed_tenant_projection",
            Self::RevokedHandoffProjection => "revoked_handoff_projection",
        }
    }
}

/// The companion dimension whose state governs how far a component may claim to be a live,
/// in-authority, companion-safe surface. The four dimensions map 1:1 to the four spec narrowing
/// axes — object freshness, companion authority, tenant scope, and handoff validity — so every
/// family carries an honest narrowing path.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5CompanionComponentClaimDimension {
    /// Object freshness: is the component's object data fresh, or has it gone stale?
    ObjectFreshness,
    /// Companion authority: is the component fully companion-safe, or is its authority limited
    /// (desktop-required / read-only)?
    CompanionAuthority,
    /// Tenant scope: is the forwarded / delegated tenant scope in scope, or has it narrowed?
    TenantScope,
    /// Handoff validity: does the component's desktop-handoff target resolve exactly, or is it
    /// revoked / unresolvable?
    HandoffValidity,
}

impl M5CompanionComponentClaimDimension {
    /// Every dimension, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::ObjectFreshness,
        Self::CompanionAuthority,
        Self::TenantScope,
        Self::HandoffValidity,
    ];

    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ObjectFreshness => "object_freshness",
            Self::CompanionAuthority => "companion_authority",
            Self::TenantScope => "tenant_scope",
            Self::HandoffValidity => "handoff_validity",
        }
    }
}

/// The observed condition of one companion dimension. Anything weaker than [`Self::LiveInScope`]
/// imposes a narrowing ceiling on the component's companion claim. The four spec axes the lane must
/// auto-narrow on — stale object freshness, limited companion authority, a narrowed tenant scope,
/// and a revoked handoff — are [`Self::FreshnessStale`], [`Self::AuthorityLimited`],
/// [`Self::TenantScopeNarrowed`], and [`Self::HandoffRevoked`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5CompanionComponentConditionState {
    /// Fresh, in-authority, in-scope, and handoff-resolvable — imposes no ceiling.
    LiveInScope,
    /// Object freshness has gone stale — the surface cannot read as live; companion claim drops to
    /// a stale-freshness projection.
    FreshnessStale,
    /// Companion authority is limited (desktop-required / read-only) — the surface cannot present
    /// as fully companion-safe; companion claim drops to a limited-authority projection.
    AuthorityLimited,
    /// The tenant / client scope has narrowed from what was granted — the surface cannot present as
    /// in-scope; companion claim drops to a narrowed-tenant projection.
    TenantScopeNarrowed,
    /// The desktop-handoff target is revoked or unresolvable — the surface cannot claim it will
    /// open the intended object; companion claim drops to a revoked-handoff projection.
    HandoffRevoked,
}

impl M5CompanionComponentConditionState {
    /// Every condition state, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::LiveInScope,
        Self::FreshnessStale,
        Self::AuthorityLimited,
        Self::TenantScopeNarrowed,
        Self::HandoffRevoked,
    ];

    /// Returns true when the dimension is weaker than live-in-scope and therefore imposes a
    /// narrowing ceiling.
    pub const fn is_weak(self) -> bool {
        !matches!(self, Self::LiveInScope)
    }

    /// Returns true when the condition reflects a stale object, a limited authority, or a revoked
    /// handoff — a state that must never be shown as live-companion-safe because it would silently
    /// imply live data or allowed companion mutation.
    pub const fn is_stale_limited_or_revoked(self) -> bool {
        matches!(
            self,
            Self::FreshnessStale | Self::AuthorityLimited | Self::HandoffRevoked
        )
    }

    /// The strongest companion claim this condition state permits.
    pub const fn permitted_ceiling(self) -> M5CompanionComponentClaim {
        match self {
            Self::LiveInScope => M5CompanionComponentClaim::LiveCompanionSafe,
            Self::FreshnessStale => M5CompanionComponentClaim::StaleFreshnessProjection,
            Self::AuthorityLimited => M5CompanionComponentClaim::LimitedAuthorityProjection,
            Self::TenantScopeNarrowed => M5CompanionComponentClaim::NarrowedTenantProjection,
            Self::HandoffRevoked => M5CompanionComponentClaim::RevokedHandoffProjection,
        }
    }

    /// The frozen downgrade trigger this condition names when its weakness binds a narrowing. Each
    /// state maps to the on-topic frozen trigger the freeze matrix already governs, so the certified
    /// reason stays byte-identical to the matrix.
    pub const fn default_trigger(self) -> M5CompanionDowngradeTrigger {
        match self {
            // The live baseline never narrows; kept for exhaustiveness.
            Self::LiveInScope => M5CompanionDowngradeTrigger::FreshnessHidden,
            Self::FreshnessStale => M5CompanionDowngradeTrigger::FreshnessHidden,
            Self::AuthorityLimited => M5CompanionDowngradeTrigger::CapabilityBoundaryUnstated,
            Self::TenantScopeNarrowed => M5CompanionDowngradeTrigger::ClientScopeUnstated,
            Self::HandoffRevoked => M5CompanionDowngradeTrigger::HandoffTargetUnresolved,
        }
    }

    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LiveInScope => "live_in_scope",
            Self::FreshnessStale => "freshness_stale",
            Self::AuthorityLimited => "authority_limited",
            Self::TenantScopeNarrowed => "tenant_scope_narrowed",
            Self::HandoffRevoked => "handoff_revoked",
        }
    }
}

/// One companion dimension's observed condition on a component.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CompanionComponentClaimConditionEntry {
    /// Which dimension this entry describes.
    pub dimension: M5CompanionComponentClaimDimension,
    /// The observed condition state of the dimension.
    pub state: M5CompanionComponentConditionState,
}

/// An honest companion-claim auto-narrow block. When a companion dimension weakens, the component's
/// companion claim lowers to the permitted ceiling, names the binding dimension and frozen trigger,
/// and preserves the canonical identity / scope / freshness / handoff lineage rather than silently
/// dropping it — the underlying object lineage is never erased opaquely.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CompanionComponentClaimAutoNarrow {
    /// The companion claim the component is narrowed to.
    pub narrowed_to: M5CompanionComponentClaim,
    /// The dimension whose weakness bound the narrowing (the one imposing the strongest ceiling
    /// constraint).
    pub binding_dimension: M5CompanionComponentClaimDimension,
    /// The frozen downgrade trigger (reused vocabulary) the narrowing names.
    pub trigger: M5CompanionDowngradeTrigger,
    /// A precise, non-generic label safe to render.
    pub narrowed_label: String,
    /// The canonical object identity, client scope, freshness, and handoff target are preserved
    /// rather than dropped; must hold.
    pub preserves_canonical_identity: bool,
    /// The underlying identity / scope / freshness / handoff lineage is preserved (never dropped)
    /// across the narrowing; must hold so stale-freshness, limited-authority, narrowed-tenant, and
    /// revoked-handoff states never fail opaquely.
    pub preserves_lineage_continuity: bool,
}

impl CompanionComponentClaimAutoNarrow {
    /// Whether the auto-narrow block is honest: it preserves canonical identity and object lineage
    /// and carries a precise, non-generic label.
    pub fn is_honest(&self) -> bool {
        self.preserves_canonical_identity
            && self.preserves_lineage_continuity
            && !label_is_generic(&self.narrowed_label)
    }
}

/// Copy / share-export parity for a component's accessible fallback: the same truth must be
/// copyable as text / JSON / Markdown, and a screenshot is never the only export.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CompanionComponentCopyExportParity {
    /// The copy / export formats offered (must include text, json, markdown).
    #[serde(default)]
    pub formats: Vec<String>,
    /// The named export fields the summary carries.
    #[serde(default)]
    pub export_fields: Vec<String>,
    /// A screenshot is never the only export; must always hold.
    pub screenshot_only_prohibited: bool,
}

impl CompanionComponentCopyExportParity {
    /// Whether the copy / export parity is complete: text / JSON / Markdown are all offered, at
    /// least one export field is named, and screenshots are prohibited as the sole export.
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
pub struct CompanionComponentRenderingNarrowingDisclosure {
    /// The rendering surface being narrowed.
    pub rendering_surface: M5CompanionComponentRenderingSurface,
    /// How the surface discloses its reduced interactivity.
    pub state: CompanionComponentNarrowingDisclosureState,
    /// The labels preserved across the narrowing.
    #[serde(default)]
    pub preserved_labels: Vec<String>,
    /// The interactions reduced on the narrowed surface.
    #[serde(default)]
    pub reduced_interactions: Vec<String>,
}

/// Derived qualification status for a companion component accessibility row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CompanionComponentAccessibilityStatus {
    /// Full keyboard / screen-reader / share-export parity with no narrowing (green).
    Parity,
    /// Reduced but fully disclosed, reachable, and honestly auto-narrowed (yellow).
    NarrowedDisclosed,
    /// Strands assistive tech, needs a screenshot, over-claims liveness, or drops state silently
    /// (red).
    Stranded,
}

impl CompanionComponentAccessibilityStatus {
    /// Stable token recorded in the summary / CSV.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Parity => "parity",
            Self::NarrowedDisclosed => "narrowed_disclosed",
            Self::Stranded => "stranded",
        }
    }
}

/// Accessibility / auto-narrowing parity row for one companion component family.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CompanionComponentAccessibilityRow {
    /// Record kind; must equal [`COMPANION_COMPONENT_A11Y_ROW_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`COMPANION_COMPONENT_A11Y_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable row id.
    pub row_id: String,
    /// The frozen component family this row certifies.
    pub component_family: M5CompanionComponentFamily,
    /// Ref to the frozen matrix family schema this row certifies.
    pub source_family_schema_ref: String,
    /// Opaque ref to the object / session / handoff this component acts on; stays visible on every
    /// surface, so this is never empty.
    pub object_context_ref: String,
    /// Rendered modalities offered; a hierarchy-heavy family must also offer a non-visual
    /// (list / textual / cli) path.
    #[serde(default)]
    pub fallback_modalities: Vec<M5CompanionComponentFallbackModality>,
    /// The non-visual / CLI path reaches the same canonical object identity, client scope,
    /// freshness, capability boundary, severity, and handoff target as the rich surface; must hold.
    pub reaches_canonical_truth: bool,
    /// Keyboard reach into the non-visual path.
    pub keyboard_reach: CompanionComponentNonVisualReachState,
    /// Screen-reader reach into the non-visual path.
    pub screen_reader_reach: CompanionComponentNonVisualReachState,
    /// CLI / headless reach into the non-visual path.
    pub cli_reach: CompanionComponentNonVisualReachState,
    /// Whether the share/export-safe summary preserves component meaning.
    pub export_summary: CompanionComponentExportSummaryState,
    /// Ref to the export-safe summary object for this component.
    pub export_summary_ref: String,
    /// The copy / share-export parity of the accessible fallback.
    pub copy_export: CompanionComponentCopyExportParity,
    /// The full companion claim this family asserts when every dimension is intact.
    pub full_companion_claim: M5CompanionComponentClaim,
    /// The observed condition of each modeled companion dimension.
    #[serde(default)]
    pub claim_conditions: Vec<CompanionComponentClaimConditionEntry>,
    /// The honest auto-narrow block, present only when some dimension weakens below the family's
    /// full claim.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub claim_narrow: Option<CompanionComponentClaimAutoNarrow>,
    /// Whether the underlying object lineage is preserved on this component regardless of narrowing;
    /// must hold so stale-freshness, limited-authority, narrowed-tenant, and revoked-handoff states
    /// never fail opaquely.
    pub lineage_preserved: bool,
    /// Rendering surfaces this component is certified on.
    #[serde(default)]
    pub rendering_surfaces: Vec<M5CompanionComponentRenderingSurface>,
    /// Per-surface narrowing disclosures.
    #[serde(default)]
    pub narrowing_disclosures: Vec<CompanionComponentRenderingNarrowingDisclosure>,
    /// The required labels the accessible fallback preserves (reused vocabulary).
    #[serde(default)]
    pub required_labels: Vec<M5CompanionRequiredLabel>,
    /// Semantic consumer surfaces this component is embedded in (reused vocabulary).
    #[serde(default)]
    pub consumer_surfaces: Vec<M5CompanionConsumerSurface>,
    /// Source contract refs backing this row.
    #[serde(default)]
    pub source_refs: Vec<String>,
    /// ISO 8601 UTC timestamp the accessibility posture was observed.
    pub observed_at: String,
    /// Evidence packet refs backing this row.
    #[serde(default)]
    pub evidence_refs: Vec<String>,
}

impl CompanionComponentAccessibilityRow {
    /// Returns true when this family renders a non-linear hierarchy and must bind to a flat
    /// non-visual path.
    pub const fn is_hierarchy_heavy(&self) -> bool {
        family_is_hierarchy_heavy(self.component_family)
    }

    /// Returns true when at least one non-visual (list / textual / cli) fallback modality is
    /// offered.
    pub fn has_non_visual_fallback(&self) -> bool {
        self.fallback_modalities.iter().any(|m| m.is_non_visual())
    }

    /// The condition state observed for one dimension, or `LiveInScope` when the row does not model
    /// that dimension.
    pub fn condition_for(
        &self,
        dimension: M5CompanionComponentClaimDimension,
    ) -> M5CompanionComponentConditionState {
        self.claim_conditions
            .iter()
            .find(|c| c.dimension == dimension)
            .map(|c| c.state)
            .unwrap_or(M5CompanionComponentConditionState::LiveInScope)
    }

    /// Whether any modeled dimension is weaker than live-in-scope.
    pub fn has_weak_dimension(&self) -> bool {
        self.claim_conditions.iter().any(|c| c.state.is_weak())
    }

    /// The strongest companion claim permitted after applying every modeled dimension's ceiling,
    /// capped at the family's full claim.
    pub fn permitted_claim(&self) -> M5CompanionComponentClaim {
        let mut permitted = self.full_companion_claim;
        for condition in &self.claim_conditions {
            let ceiling = condition.state.permitted_ceiling();
            if ceiling.capability_rank() < permitted.capability_rank() {
                permitted = ceiling;
            }
        }
        permitted
    }

    /// The condition entry imposing the strongest (lowest-rank) ceiling, if any weak dimension
    /// narrows below the family's full claim.
    pub fn binding_condition(&self) -> Option<&CompanionComponentClaimConditionEntry> {
        let mut binding: Option<(&CompanionComponentClaimConditionEntry, u8)> = None;
        for condition in &self.claim_conditions {
            if !condition.state.is_weak() {
                continue;
            }
            let ceiling = condition.state.permitted_ceiling();
            if ceiling.capability_rank() >= self.full_companion_claim.capability_rank() {
                // The dimension is weak but does not narrow below the full claim.
                continue;
            }
            let rank = ceiling.capability_rank();
            match binding {
                Some((_, best)) if best <= rank => {}
                _ => binding = Some((condition, rank)),
            }
        }
        binding.map(|(condition, _)| condition)
    }

    /// The dimension imposing the strongest (lowest-rank) ceiling, if any.
    pub fn binding_dimension(&self) -> Option<M5CompanionComponentClaimDimension> {
        self.binding_condition().map(|c| c.dimension)
    }

    /// The companion claim this component effectively asserts after narrowing.
    pub fn effective_claim(&self) -> M5CompanionComponentClaim {
        match &self.claim_narrow {
            Some(narrow) => narrow.narrowed_to,
            None => self.full_companion_claim,
        }
    }

    /// AC / auto-narrowing honesty: a stale object, a limited companion authority, a narrowed tenant
    /// scope, or a revoked handoff can no longer keep an old `LiveCompanionSafe` /
    /// `CachedContinuitySafe` label. The effective claim never exceeds the permitted ceiling; when a
    /// dimension narrows below the full claim, an honest narrow block is present, narrows to exactly
    /// the permitted ceiling, binds to the ceiling-imposing dimension with its frozen trigger, and
    /// preserves canonical identity and object lineage. When nothing narrows, no spurious narrow
    /// block is present.
    pub fn claim_is_honest(&self) -> bool {
        let permitted = self.permitted_claim();
        if self.effective_claim().capability_rank() > permitted.capability_rank() {
            return false;
        }
        match (&self.claim_narrow, self.binding_condition()) {
            (Some(narrow), Some(binding)) => {
                narrow.is_honest()
                    && narrow.narrowed_to == permitted
                    && narrow.binding_dimension == binding.dimension
                    && narrow.trigger == binding.state.default_trigger()
                    && binding.state.is_weak()
            }
            // A narrow block with no ceiling-imposing dimension is spurious.
            (Some(_), None) => false,
            // A ceiling-imposing dimension with no narrow block over-claims.
            (None, Some(_)) => false,
            (None, None) => true,
        }
    }

    /// AC / live-safety honesty: a stale, limited, or revoked state (which would silently imply live
    /// data or allowed companion mutation) never keeps a live-companion-safe claim. When such a
    /// state is modeled, the effective claim must not assert `LiveCompanionSafe`.
    pub fn live_safety_holds(&self) -> bool {
        let has_stale_limited_or_revoked = self
            .claim_conditions
            .iter()
            .any(|c| c.state.is_stale_limited_or_revoked());
        !(has_stale_limited_or_revoked && self.effective_claim().asserts_live_companion_safe())
    }

    /// AC / assistive-tech reach: accessibility and export surfaces reach the same canonical
    /// truth — no keyboard / screen-reader / CLI trap, a hierarchy-heavy family offers a
    /// non-visual fallback, and the export reconstructs meaning without a screenshot.
    pub fn reaches_canonical_truth_via_at(&self) -> bool {
        self.reaches_canonical_truth
            && !self.object_context_ref.trim().is_empty()
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

    /// AC / no-loss: stale-freshness, limited-authority, narrowed-tenant, and revoked-handoff states
    /// preserve the underlying object lineage. The row must assert `lineage_preserved`, and any
    /// narrow block must preserve lineage continuity too.
    pub fn preserves_lineage_continuity(&self) -> bool {
        self.lineage_preserved
            && self
                .claim_narrow
                .as_ref()
                .map(|n| n.preserves_lineage_continuity)
                .unwrap_or(true)
    }

    /// Whether any axis is in a disclosed-reduction (yellow) state or the component carries an
    /// honest claim narrow.
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

    /// AC / cross-surface disclosure: every narrower rendering surface discloses its reduced
    /// interactivity and keeps its labels, so product / docs / release publication stay aligned on
    /// the same narrowed state.
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
        // Every disclosure never silently drops and preserves labels on a narrowed surface.
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
        M5CompanionRequiredLabel::MANDATORY
            .iter()
            .all(|label| self.required_labels.contains(label))
    }

    /// Derived qualification status.
    pub fn status(&self) -> CompanionComponentAccessibilityStatus {
        if !self.claim_is_honest()
            || !self.live_safety_holds()
            || !self.reaches_canonical_truth_via_at()
            || !self.export_preserves_meaning()
            || !self.preserves_lineage_continuity()
            || !self.narrowing_disclosed()
            || !self.models_primary_dimension()
            || !self.preserves_mandatory_labels()
        {
            return CompanionComponentAccessibilityStatus::Stranded;
        }
        if self.is_reduced() {
            CompanionComponentAccessibilityStatus::NarrowedDisclosed
        } else {
            CompanionComponentAccessibilityStatus::Parity
        }
    }

    /// Whether the row's identity and evidence fields are complete.
    pub fn is_complete(&self) -> bool {
        self.record_kind == COMPANION_COMPONENT_A11Y_ROW_RECORD_KIND
            && self.schema_version == COMPANION_COMPONENT_A11Y_SCHEMA_VERSION
            && !self.row_id.trim().is_empty()
            && !self.source_family_schema_ref.trim().is_empty()
            && !self.object_context_ref.trim().is_empty()
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
            full = self.full_companion_claim.as_str(),
            effective = self.effective_claim().as_str(),
            status = self.status().as_str(),
        )
    }
}

/// Rolled-up summary of an M05-1002 companion component accessibility parity packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CompanionComponentAccessibilitySummary {
    pub row_count: usize,
    pub family_count: usize,
    pub hierarchy_heavy_family_count: usize,
    pub all_hierarchy_heavy_have_non_visual_fallback: bool,
    pub all_reach_canonical_truth_via_at: bool,
    pub all_claims_honest: bool,
    pub all_live_safety_holds: bool,
    pub all_export_summaries_preserve_meaning: bool,
    pub all_lineage_preserved: bool,
    pub all_narrowing_disclosed: bool,
    pub green_count: usize,
    pub yellow_count: usize,
    pub red_count: usize,
    pub rendering_surface_count: usize,
    pub consumer_surface_count: usize,
}

/// Constructor input for [`CompanionComponentAccessibilityPacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CompanionComponentAccessibilityPacketInput {
    pub packet_id: String,
    pub as_of: String,
    pub matrix_ref: String,
    pub rows: Vec<CompanionComponentAccessibilityRow>,
}

/// Checked-in M05-1002 companion component accessibility parity packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CompanionComponentAccessibilityPacket {
    pub schema_version: u32,
    pub record_kind: String,
    pub packet_id: String,
    pub as_of: String,
    pub matrix_ref: String,
    #[serde(default)]
    pub rows: Vec<CompanionComponentAccessibilityRow>,
    pub summary: CompanionComponentAccessibilitySummary,
}

impl CompanionComponentAccessibilityPacket {
    /// Builds a packet, stamping the record kind, schema version, and computed summary.
    pub fn new(input: CompanionComponentAccessibilityPacketInput) -> Self {
        let mut packet = Self {
            schema_version: COMPANION_COMPONENT_A11Y_SCHEMA_VERSION,
            record_kind: COMPANION_COMPONENT_A11Y_RECORD_KIND.to_owned(),
            packet_id: input.packet_id,
            as_of: input.as_of,
            matrix_ref: input.matrix_ref,
            rows: input.rows,
            summary: CompanionComponentAccessibilitySummary {
                row_count: 0,
                family_count: 0,
                hierarchy_heavy_family_count: 0,
                all_hierarchy_heavy_have_non_visual_fallback: false,
                all_reach_canonical_truth_via_at: false,
                all_claims_honest: false,
                all_live_safety_holds: false,
                all_export_summaries_preserve_meaning: false,
                all_lineage_preserved: false,
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
    pub fn represented_families(&self) -> BTreeSet<M5CompanionComponentFamily> {
        self.rows.iter().map(|r| r.component_family).collect()
    }

    /// Dimensions exercised by some row's claim conditions.
    pub fn exercised_dimensions(&self) -> BTreeSet<M5CompanionComponentClaimDimension> {
        self.rows
            .iter()
            .flat_map(|r| r.claim_conditions.iter().map(|c| c.dimension))
            .collect()
    }

    /// Condition states exercised by some row's claim conditions.
    pub fn exercised_condition_states(&self) -> BTreeSet<M5CompanionComponentConditionState> {
        self.rows
            .iter()
            .flat_map(|r| r.claim_conditions.iter().map(|c| c.state))
            .collect()
    }

    /// Companion claim tiers that appear as an effective claim across the rows.
    pub fn represented_effective_claims(&self) -> BTreeSet<M5CompanionComponentClaim> {
        self.rows.iter().map(|r| r.effective_claim()).collect()
    }

    /// Consumer surfaces ingesting some row in this packet.
    pub fn represented_consumer_surfaces(&self) -> BTreeSet<M5CompanionConsumerSurface> {
        self.rows
            .iter()
            .flat_map(|r| r.consumer_surfaces.iter().copied())
            .collect()
    }

    /// Computes summary fields from the packet contents.
    pub fn computed_summary(&self) -> CompanionComponentAccessibilitySummary {
        let mut rendering = BTreeSet::new();
        let mut consumers: BTreeSet<M5CompanionConsumerSurface> = BTreeSet::new();
        for row in &self.rows {
            rendering.extend(row.rendering_surfaces.iter().copied());
            consumers.extend(row.consumer_surfaces.iter().copied());
        }

        let hierarchy_heavy: Vec<&CompanionComponentAccessibilityRow> = self
            .rows
            .iter()
            .filter(|row| row.is_hierarchy_heavy())
            .collect();

        let mut green = 0;
        let mut yellow = 0;
        let mut red = 0;
        for row in &self.rows {
            match row.status() {
                CompanionComponentAccessibilityStatus::Parity => green += 1,
                CompanionComponentAccessibilityStatus::NarrowedDisclosed => yellow += 1,
                CompanionComponentAccessibilityStatus::Stranded => red += 1,
            }
        }

        CompanionComponentAccessibilitySummary {
            row_count: self.rows.len(),
            family_count: self.represented_families().len(),
            hierarchy_heavy_family_count: hierarchy_heavy.len(),
            all_hierarchy_heavy_have_non_visual_fallback: hierarchy_heavy
                .iter()
                .all(|row| row.has_non_visual_fallback()),
            all_reach_canonical_truth_via_at: self
                .rows
                .iter()
                .all(CompanionComponentAccessibilityRow::reaches_canonical_truth_via_at),
            all_claims_honest: self
                .rows
                .iter()
                .all(CompanionComponentAccessibilityRow::claim_is_honest),
            all_live_safety_holds: self
                .rows
                .iter()
                .all(CompanionComponentAccessibilityRow::live_safety_holds),
            all_export_summaries_preserve_meaning: self
                .rows
                .iter()
                .all(CompanionComponentAccessibilityRow::export_preserves_meaning),
            all_lineage_preserved: self
                .rows
                .iter()
                .all(CompanionComponentAccessibilityRow::preserves_lineage_continuity),
            all_narrowing_disclosed: self
                .rows
                .iter()
                .all(CompanionComponentAccessibilityRow::narrowing_disclosed),
            green_count: green,
            yellow_count: yellow,
            red_count: red,
            rendering_surface_count: rendering.len(),
            consumer_surface_count: consumers.len(),
        }
    }

    /// Validates the packet and returns every contract violation.
    pub fn validate(&self) -> Vec<CompanionComponentAccessibilityViolation> {
        let mut violations = Vec::new();

        if self.schema_version != COMPANION_COMPONENT_A11Y_SCHEMA_VERSION {
            violations.push(CompanionComponentAccessibilityViolation::SchemaVersion {
                expected: COMPANION_COMPONENT_A11Y_SCHEMA_VERSION,
                actual: self.schema_version,
            });
        }
        if self.record_kind != COMPANION_COMPONENT_A11Y_RECORD_KIND {
            violations.push(CompanionComponentAccessibilityViolation::RecordKind {
                expected: COMPANION_COMPONENT_A11Y_RECORD_KIND.to_owned(),
                actual: self.record_kind.clone(),
            });
        }
        if self.packet_id.trim().is_empty()
            || self.as_of.trim().is_empty()
            || self.matrix_ref.trim().is_empty()
        {
            violations.push(CompanionComponentAccessibilityViolation::MissingIdentity);
        }

        let mut row_ids = BTreeSet::new();
        let mut seen_families = BTreeSet::new();
        let mut has_stale_limited_or_revoked_row = false;
        for row in &self.rows {
            if !row_ids.insert(row.row_id.clone()) {
                violations.push(CompanionComponentAccessibilityViolation::DuplicateId {
                    id: row.row_id.clone(),
                });
            }
            seen_families.insert(row.component_family);
            if row
                .claim_conditions
                .iter()
                .any(|c| c.state.is_stale_limited_or_revoked())
            {
                has_stale_limited_or_revoked_row = true;
            }

            if !row.is_complete() {
                violations.push(CompanionComponentAccessibilityViolation::IncompleteRow {
                    id: row.row_id.clone(),
                });
            }

            // Each row must model its family's primary weakening dimension.
            if !row.models_primary_dimension() {
                violations.push(
                    CompanionComponentAccessibilityViolation::MissingPrimaryDimension {
                        id: row.row_id.clone(),
                        dimension: family_primary_dimension(row.component_family),
                    },
                );
            }

            // Each row must preserve every mandatory companion label.
            if !row.preserves_mandatory_labels() {
                violations.push(
                    CompanionComponentAccessibilityViolation::MissingMandatoryLabel {
                        id: row.row_id.clone(),
                    },
                );
            }

            // A hierarchy-heavy family must render a structured tree *and* a non-visual path.
            if row.is_hierarchy_heavy()
                && !row
                    .fallback_modalities
                    .contains(&M5CompanionComponentFallbackModality::Structured)
            {
                violations.push(
                    CompanionComponentAccessibilityViolation::HierarchyHeavyMissingStructured {
                        id: row.row_id.clone(),
                    },
                );
            }

            // AC1: claim never over-asserts a live / cached-safe surface for a weakened one.
            if !row.claim_is_honest() {
                violations.push(
                    CompanionComponentAccessibilityViolation::ClaimOverAsserted {
                        id: row.row_id.clone(),
                    },
                );
            }

            // AC2: a stale / limited / revoked state never keeps a live-companion-safe claim.
            if !row.live_safety_holds() {
                violations.push(
                    CompanionComponentAccessibilityViolation::StaleShownAsLiveAndSafe {
                        id: row.row_id.clone(),
                    },
                );
            }

            // Assistive-tech / CLI reach the same canonical truth.
            if !row.reaches_canonical_truth_via_at() {
                violations.push(
                    CompanionComponentAccessibilityViolation::AssistiveTechStranded {
                        id: row.row_id.clone(),
                    },
                );
            }

            // Export preserves meaning without a screenshot.
            if !row.export_preserves_meaning() {
                violations.push(
                    CompanionComponentAccessibilityViolation::ExportRequiresScreenshot {
                        id: row.row_id.clone(),
                    },
                );
            }

            // AC / no-loss: stale-freshness, limited-authority, narrowed-tenant, and revoked-handoff
            // states preserve object lineage.
            if !row.preserves_lineage_continuity() {
                violations.push(CompanionComponentAccessibilityViolation::LineageDropped {
                    id: row.row_id.clone(),
                });
            }

            // Narrowing disclosed on every narrowed rendering surface.
            if !row.narrowing_disclosed() {
                violations.push(
                    CompanionComponentAccessibilityViolation::NarrowingDropsContextSilently {
                        id: row.row_id.clone(),
                    },
                );
            }

            // Consumer parity: at least two consumer surfaces ingest the row.
            if row.consumer_surfaces.len() < 2 {
                violations.push(
                    CompanionComponentAccessibilityViolation::MissingConsumerParity {
                        id: row.row_id.clone(),
                    },
                );
            }

            // No red rows may ship.
            if row.status() == CompanionComponentAccessibilityStatus::Stranded {
                violations.push(CompanionComponentAccessibilityViolation::StrandedRow {
                    id: row.row_id.clone(),
                });
            }
        }

        // Coverage: every frozen family is certified at least once.
        for family in M5CompanionComponentFamily::ALL {
            if !seen_families.contains(&family) {
                violations.push(
                    CompanionComponentAccessibilityViolation::MissingFamilyCoverage { family },
                );
            }
        }

        // Coverage: every weakening dimension is exercised somewhere.
        let exercised = self.exercised_dimensions();
        for dimension in M5CompanionComponentClaimDimension::ALL {
            if !exercised.contains(&dimension) {
                violations.push(
                    CompanionComponentAccessibilityViolation::MissingDimensionCoverage {
                        dimension,
                    },
                );
            }
        }

        // Coverage: every condition state (the live baseline plus each spec narrowing axis) is
        // exercised somewhere, so the full narrowing spectrum is proven end-to-end.
        let states = self.exercised_condition_states();
        for state in M5CompanionComponentConditionState::ALL {
            if !states.contains(&state) {
                violations.push(
                    CompanionComponentAccessibilityViolation::MissingConditionStateCoverage {
                        state,
                    },
                );
            }
        }

        // Coverage: every companion claim tier appears as an effective claim, so the full narrowing
        // spectrum (live-companion-safe → … → revoked-handoff) is proven end-to-end.
        let effective = self.represented_effective_claims();
        for claim in M5CompanionComponentClaim::ALL {
            if !effective.contains(&claim) {
                violations.push(
                    CompanionComponentAccessibilityViolation::MissingClaimTierCoverage { claim },
                );
            }
        }

        // Live-safety honesty must be proven with at least one stale / limited / revoked row in the
        // packet, so the "stale / limited / revoked never shown as live-companion-safe" guarantee is
        // exercised end-to-end.
        if !has_stale_limited_or_revoked_row {
            violations.push(CompanionComponentAccessibilityViolation::LiveSafetyHonestyUnproven);
        }

        // Cross-surface: the same narrowed state must reach the notification-triage, review-queue,
        // CI-status, session-follow, incident-awareness, desktop-handoff, status-bar, product UI,
        // and support / export share — so every consumer surface is exercised at least once across
        // the packet.
        let consumers = self.represented_consumer_surfaces();
        for surface in M5CompanionConsumerSurface::ALL {
            if !consumers.contains(&surface) {
                violations.push(
                    CompanionComponentAccessibilityViolation::MissingConsumerSurfaceCoverage {
                        surface,
                    },
                );
            }
        }

        if self.summary != self.computed_summary() {
            violations.push(CompanionComponentAccessibilityViolation::SummaryMismatch);
        }

        if json_contains_forbidden_material(
            &serde_json::to_value(self)
                .expect("companion component accessibility parity packet serializes"),
        ) {
            violations.push(CompanionComponentAccessibilityViolation::RawCompanionMaterialInExport);
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
            .expect("companion component accessibility parity packet serializes")
    }

    /// Deterministic CSV of the certified rows for support / release handoff.
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
                full = row.full_companion_claim.as_str(),
                effective = row.effective_claim().as_str(),
                status = row.status().as_str(),
            ));
        }
        out
    }

    /// Deterministic Markdown report for support, docs, or release handoff.
    pub fn render_markdown_summary(&self) -> String {
        let mut out = String::new();
        out.push_str("# M5 Companion Component Accessibility & Auto-Narrowing\n\n");
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- As of: `{}`\n", self.as_of));
        out.push_str(&format!(
            "- Families: {} certified across {} / {} frozen families\n",
            self.summary.family_count,
            self.represented_families().len(),
            M5CompanionComponentFamily::ALL.len(),
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
                    row.full_companion_claim.as_str(),
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

/// Reads and validates the checked-in companion component accessibility parity export.
pub fn current_m5_companion_component_a11y_export(
) -> Result<CompanionComponentAccessibilityPacket, CompanionComponentAccessibilityArtifactError> {
    let packet: CompanionComponentAccessibilityPacket =
        serde_json::from_str(include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../artifacts/release/m5-companion-component-accessibility-proof/support_export.json"
        )))
        .map_err(CompanionComponentAccessibilityArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(CompanionComponentAccessibilityArtifactError::Validation(
            violations,
        ))
    }
}

/// Errors emitted when reading the checked-in companion component accessibility parity export.
#[derive(Debug)]
pub enum CompanionComponentAccessibilityArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<CompanionComponentAccessibilityViolation>),
}

impl fmt::Display for CompanionComponentAccessibilityArtifactError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(
                    f,
                    "companion component accessibility parity export parse failed: {error}"
                )
            }
            Self::Validation(violations) => {
                write!(
                    f,
                    "companion component accessibility parity export failed validation: {} violation(s)",
                    violations.len()
                )
            }
        }
    }
}

impl Error for CompanionComponentAccessibilityArtifactError {}

/// Validation failure for M05-1002 companion component accessibility parity packets.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CompanionComponentAccessibilityViolation {
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
        dimension: M5CompanionComponentClaimDimension,
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
    StaleShownAsLiveAndSafe {
        id: String,
    },
    AssistiveTechStranded {
        id: String,
    },
    ExportRequiresScreenshot {
        id: String,
    },
    LineageDropped {
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
        family: M5CompanionComponentFamily,
    },
    MissingDimensionCoverage {
        dimension: M5CompanionComponentClaimDimension,
    },
    MissingConditionStateCoverage {
        state: M5CompanionComponentConditionState,
    },
    MissingClaimTierCoverage {
        claim: M5CompanionComponentClaim,
    },
    LiveSafetyHonestyUnproven,
    MissingConsumerSurfaceCoverage {
        surface: M5CompanionConsumerSurface,
    },
    SummaryMismatch,
    RawCompanionMaterialInExport,
}

impl fmt::Display for CompanionComponentAccessibilityViolation {
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
                write!(f, "row {id} drops a mandatory companion label")
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
                    "row {id} over-asserts a live / cached-safe surface for a weakened one, or narrows spuriously"
                )
            }
            Self::StaleShownAsLiveAndSafe { id } => {
                write!(
                    f,
                    "row {id} shows a stale, limited, or revoked state as live-companion-safe"
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
            Self::LineageDropped { id } => {
                write!(
                    f,
                    "row {id} does not preserve object lineage across narrowing"
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
            Self::MissingConditionStateCoverage { state } => {
                write!(
                    f,
                    "condition state {} is not exercised in the packet",
                    state.as_str()
                )
            }
            Self::MissingClaimTierCoverage { claim } => {
                write!(
                    f,
                    "companion claim tier {} does not appear as an effective claim",
                    claim.as_str()
                )
            }
            Self::LiveSafetyHonestyUnproven => {
                write!(
                    f,
                    "no stale / limited / revoked row is present to prove the live-safety-honesty guarantee"
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
            Self::RawCompanionMaterialInExport => {
                write!(f, "export contains raw companion payload material")
            }
        }
    }
}

impl Error for CompanionComponentAccessibilityViolation {}

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
            | "blocked"
            | "unresolved"
            | "read only"
            | "stale"
            | "limited"
            | "revoked"
            | "cached"
            | "offline"
    )
}

/// Heuristic that rejects obviously forbidden material in export-safe JSON. The companion packet is
/// metadata-only, so this flags actual raw material — passwords, passphrases, bearer tokens, PEM
/// blocks, and embedded URLs — that must never cross the companion boundary.
fn json_contains_forbidden_material(value: &serde_json::Value) -> bool {
    match value {
        serde_json::Value::String(s) => {
            let lower = s.to_lowercase();
            lower.contains("password")
                || lower.contains("passphrase")
                || lower.contains("-----begin")
                || lower.contains("bearer ")
                || lower.contains("://")
        }
        serde_json::Value::Array(arr) => arr.iter().any(json_contains_forbidden_material),
        serde_json::Value::Object(map) => map.values().any(json_contains_forbidden_material),
        _ => false,
    }
}

/// Builds the canonical, checked-in companion component accessibility parity packet. This is the one
/// source of truth shared by the tests and the on-disk support export so both stay byte-aligned.
pub fn seeded_m5_companion_component_a11y_packet() -> CompanionComponentAccessibilityPacket {
    CompanionComponentAccessibilityPacket::new(CompanionComponentAccessibilityPacketInput {
        packet_id: "m5-companion-component-accessibility-parity:stable:0001".to_owned(),
        as_of: "2026-07-09T00:00:00Z".to_owned(),
        matrix_ref: COMPANION_COMPONENT_A11Y_COMPONENT_MATRIX_REF.to_owned(),
        rows: seeded_rows(),
    })
}

fn ev(id: &str) -> Vec<String> {
    vec![format!("evidence:companion-component-a11y:{id}")]
}

fn all_required_labels() -> Vec<M5CompanionRequiredLabel> {
    M5CompanionRequiredLabel::ALL.to_vec()
}

fn copy_export(fields: &[&str]) -> CompanionComponentCopyExportParity {
    CompanionComponentCopyExportParity {
        formats: vec!["text".to_owned(), "json".to_owned(), "markdown".to_owned()],
        export_fields: fields.iter().map(|f| (*f).to_owned()).collect(),
        screenshot_only_prohibited: true,
    }
}

fn condition(
    dimension: M5CompanionComponentClaimDimension,
    state: M5CompanionComponentConditionState,
) -> CompanionComponentClaimConditionEntry {
    CompanionComponentClaimConditionEntry { dimension, state }
}

/// The two consumer surfaces every row ships to at minimum — support / release share and the
/// general product UI — so the narrowed state always reaches headless field triage.
fn base_consumers(extra: &[M5CompanionConsumerSurface]) -> Vec<M5CompanionConsumerSurface> {
    let mut out = vec![
        M5CompanionConsumerSurface::SupportExport,
        M5CompanionConsumerSurface::ProductUi,
    ];
    out.extend_from_slice(extra);
    out
}

/// Disclosures for the CLI-headless and support-export surfaces. A green (full parity) row keeps
/// full label and summary parity on the narrower surfaces; a narrowed row discloses the reduced
/// interactions it drops there.
fn surface_disclosures(
    labels: &[&str],
    state: CompanionComponentNarrowingDisclosureState,
) -> Vec<CompanionComponentRenderingNarrowingDisclosure> {
    let preserved: Vec<String> = labels.iter().map(|l| (*l).to_owned()).collect();
    vec![
        CompanionComponentRenderingNarrowingDisclosure {
            rendering_surface: M5CompanionComponentRenderingSurface::CliHeadless,
            state,
            preserved_labels: preserved.clone(),
            reduced_interactions: vec!["pointer_interaction".to_owned()],
        },
        CompanionComponentRenderingNarrowingDisclosure {
            rendering_surface: M5CompanionComponentRenderingSurface::SupportExport,
            state,
            preserved_labels: preserved,
            reduced_interactions: vec!["desktop_required_action".to_owned()],
        },
    ]
}

/// Disclosures for a full-parity (green) row: the narrower surfaces preserve full label and
/// summary parity.
fn parity_surfaces(labels: &[&str]) -> Vec<CompanionComponentRenderingNarrowingDisclosure> {
    surface_disclosures(
        labels,
        CompanionComponentNarrowingDisclosureState::ParityPreserved,
    )
}

/// Disclosures for a narrowed (yellow) row: the narrower surfaces disclose their reduced
/// interactions while preserving labels.
fn narrowed_surfaces(labels: &[&str]) -> Vec<CompanionComponentRenderingNarrowingDisclosure> {
    surface_disclosures(
        labels,
        CompanionComponentNarrowingDisclosureState::DisclosedNarrowed,
    )
}

fn rendering_surfaces() -> Vec<M5CompanionComponentRenderingSurface> {
    vec![
        M5CompanionComponentRenderingSurface::DesktopFull,
        M5CompanionComponentRenderingSurface::CliHeadless,
        M5CompanionComponentRenderingSurface::SupportExport,
    ]
}

fn seeded_rows() -> Vec<CompanionComponentAccessibilityRow> {
    vec![
        // Notification row (object freshness stale) — the notification's object data has gone stale,
        // so the row auto-narrows to a stale-freshness projection rather than reading as live, while
        // keeping its canonical object identity, client scope, and severity visible (yellow).
        CompanionComponentAccessibilityRow {
            record_kind: COMPANION_COMPONENT_A11Y_ROW_RECORD_KIND.to_owned(),
            schema_version: COMPANION_COMPONENT_A11Y_SCHEMA_VERSION,
            row_id: "a11y:notification-row-freshness-stale".to_owned(),
            component_family: M5CompanionComponentFamily::NotificationRow,
            source_family_schema_ref: COMPANION_COMPONENT_A11Y_COMPONENT_MATRIX_REF.to_owned(),
            object_context_ref: "companion:notification-row:0001".to_owned(),
            fallback_modalities: vec![
                M5CompanionComponentFallbackModality::List,
                M5CompanionComponentFallbackModality::Textual,
                M5CompanionComponentFallbackModality::Cli,
            ],
            reaches_canonical_truth: true,
            keyboard_reach: CompanionComponentNonVisualReachState::ReachableAndLabeled,
            screen_reader_reach: CompanionComponentNonVisualReachState::ReachableAndLabeled,
            cli_reach: CompanionComponentNonVisualReachState::ReachableAndLabeled,
            export_summary: CompanionComponentExportSummaryState::ReconstructableWithoutScreenshot,
            export_summary_ref: "summary:notification-row-freshness-stale:a11y".to_owned(),
            copy_export: copy_export(&[
                "object_identity",
                "client_scope",
                "freshness",
                "keyboard_route",
            ]),
            full_companion_claim: M5CompanionComponentClaim::LiveCompanionSafe,
            claim_conditions: vec![condition(
                M5CompanionComponentClaimDimension::ObjectFreshness,
                M5CompanionComponentConditionState::FreshnessStale,
            )],
            claim_narrow: Some(CompanionComponentClaimAutoNarrow {
                narrowed_to: M5CompanionComponentClaim::StaleFreshnessProjection,
                binding_dimension: M5CompanionComponentClaimDimension::ObjectFreshness,
                trigger: M5CompanionDowngradeTrigger::FreshnessHidden,
                narrowed_label:
                    "Object freshness has gone stale and this notification must be refreshed — shown as a stale-freshness projection with its canonical object identity, client scope, and severity still preserved, never as a live notification"
                        .to_owned(),
                preserves_canonical_identity: true,
                preserves_lineage_continuity: true,
            }),
            lineage_preserved: true,
            rendering_surfaces: rendering_surfaces(),
            narrowing_disclosures: narrowed_surfaces(&[
                "object_identity",
                "client_scope",
                "freshness",
            ]),
            required_labels: all_required_labels(),
            consumer_surfaces: base_consumers(&[
                M5CompanionConsumerSurface::NotificationTriageUi,
                M5CompanionConsumerSurface::StatusBarUi,
            ]),
            source_refs: vec![
                "UI/UX Spec §16.13 mobile companion notification triage".to_owned(),
                COMPANION_COMPONENT_A11Y_DOC_REF.to_owned(),
            ],
            observed_at: "2026-07-09T00:00:00Z".to_owned(),
            evidence_refs: ev("notification-row-freshness-stale"),
        },
        // Mobile review card (companion authority limited) — the review requires desktop authority,
        // so the card auto-narrows to a limited-authority projection rather than presenting a fully
        // companion-completable review, while keeping its review kind and capability boundary
        // visible (yellow).
        CompanionComponentAccessibilityRow {
            record_kind: COMPANION_COMPONENT_A11Y_ROW_RECORD_KIND.to_owned(),
            schema_version: COMPANION_COMPONENT_A11Y_SCHEMA_VERSION,
            row_id: "a11y:mobile-review-card-authority-limited".to_owned(),
            component_family: M5CompanionComponentFamily::MobileReviewCard,
            source_family_schema_ref: COMPANION_COMPONENT_A11Y_COMPONENT_MATRIX_REF.to_owned(),
            object_context_ref: "companion:mobile-review-card:0002".to_owned(),
            fallback_modalities: vec![
                M5CompanionComponentFallbackModality::List,
                M5CompanionComponentFallbackModality::Textual,
                M5CompanionComponentFallbackModality::Cli,
            ],
            reaches_canonical_truth: true,
            keyboard_reach: CompanionComponentNonVisualReachState::ReachableAndLabeled,
            screen_reader_reach: CompanionComponentNonVisualReachState::ReachableAndLabeled,
            cli_reach: CompanionComponentNonVisualReachState::ReachableAndLabeled,
            export_summary: CompanionComponentExportSummaryState::ReconstructableWithoutScreenshot,
            export_summary_ref: "summary:mobile-review-card-authority-limited:a11y".to_owned(),
            copy_export: copy_export(&[
                "review_kind",
                "capability_boundary",
                "handoff_target",
                "keyboard_route",
            ]),
            full_companion_claim: M5CompanionComponentClaim::LiveCompanionSafe,
            claim_conditions: vec![condition(
                M5CompanionComponentClaimDimension::CompanionAuthority,
                M5CompanionComponentConditionState::AuthorityLimited,
            )],
            claim_narrow: Some(CompanionComponentClaimAutoNarrow {
                narrowed_to: M5CompanionComponentClaim::LimitedAuthorityProjection,
                binding_dimension: M5CompanionComponentClaimDimension::CompanionAuthority,
                trigger: M5CompanionDowngradeTrigger::CapabilityBoundaryUnstated,
                narrowed_label:
                    "This review requires desktop authority and only a read-only companion view remains — shown as a limited-authority projection that names its review kind and capability boundary, never as a companion-completable review"
                        .to_owned(),
                preserves_canonical_identity: true,
                preserves_lineage_continuity: true,
            }),
            lineage_preserved: true,
            rendering_surfaces: rendering_surfaces(),
            narrowing_disclosures: narrowed_surfaces(&[
                "review_kind",
                "capability_boundary",
                "handoff_target",
            ]),
            required_labels: all_required_labels(),
            consumer_surfaces: base_consumers(&[M5CompanionConsumerSurface::ReviewQueueUi]),
            source_refs: vec![
                "UX Design System §16.37 companion review cards".to_owned(),
                COMPANION_COMPONENT_A11Y_DOC_REF.to_owned(),
            ],
            observed_at: "2026-07-09T00:00:00Z".to_owned(),
            evidence_refs: ev("mobile-review-card-authority-limited"),
        },
        // CI-status card — the pipeline status is fresh and in-authority, so the card is
        // live-companion-safe and reachable on every surface (green).
        CompanionComponentAccessibilityRow {
            record_kind: COMPANION_COMPONENT_A11Y_ROW_RECORD_KIND.to_owned(),
            schema_version: COMPANION_COMPONENT_A11Y_SCHEMA_VERSION,
            row_id: "a11y:ci-status-card".to_owned(),
            component_family: M5CompanionComponentFamily::CiStatusCard,
            source_family_schema_ref: COMPANION_COMPONENT_A11Y_COMPONENT_MATRIX_REF.to_owned(),
            object_context_ref: "companion:ci-status-card:0003".to_owned(),
            fallback_modalities: vec![
                M5CompanionComponentFallbackModality::List,
                M5CompanionComponentFallbackModality::Textual,
                M5CompanionComponentFallbackModality::Cli,
            ],
            reaches_canonical_truth: true,
            keyboard_reach: CompanionComponentNonVisualReachState::ReachableAndLabeled,
            screen_reader_reach: CompanionComponentNonVisualReachState::ReachableAndLabeled,
            cli_reach: CompanionComponentNonVisualReachState::ReachableAndLabeled,
            export_summary: CompanionComponentExportSummaryState::ReconstructableWithoutScreenshot,
            export_summary_ref: "summary:ci-status-card:a11y".to_owned(),
            copy_export: copy_export(&[
                "run_identity",
                "commit_identity",
                "freshness",
                "keyboard_route",
            ]),
            full_companion_claim: M5CompanionComponentClaim::LiveCompanionSafe,
            claim_conditions: vec![condition(
                M5CompanionComponentClaimDimension::ObjectFreshness,
                M5CompanionComponentConditionState::LiveInScope,
            )],
            claim_narrow: None,
            lineage_preserved: true,
            rendering_surfaces: rendering_surfaces(),
            narrowing_disclosures: parity_surfaces(&[
                "run_identity",
                "commit_identity",
                "freshness",
            ]),
            required_labels: all_required_labels(),
            consumer_surfaces: base_consumers(&[
                M5CompanionConsumerSurface::CiStatusUi,
                M5CompanionConsumerSurface::StatusBarUi,
            ]),
            source_refs: vec![
                "TDD command/handoff parity and browser/desktop boundary rules".to_owned(),
                COMPANION_COMPONENT_A11Y_DOC_REF.to_owned(),
            ],
            observed_at: "2026-07-09T00:00:00Z".to_owned(),
            evidence_refs: ev("ci-status-card"),
        },
        // Session-follow tile (tenant scope narrowed) — the followed session's tenant scope has
        // narrowed from what was granted, so the tile auto-narrows to a narrowed-tenant projection
        // rather than presenting an in-scope session, while naming its presenter and session
        // identity (yellow).
        CompanionComponentAccessibilityRow {
            record_kind: COMPANION_COMPONENT_A11Y_ROW_RECORD_KIND.to_owned(),
            schema_version: COMPANION_COMPONENT_A11Y_SCHEMA_VERSION,
            row_id: "a11y:session-follow-tile-tenant-narrowed".to_owned(),
            component_family: M5CompanionComponentFamily::SessionFollowTile,
            source_family_schema_ref: COMPANION_COMPONENT_A11Y_COMPONENT_MATRIX_REF.to_owned(),
            object_context_ref: "companion:session-follow-tile:0004".to_owned(),
            fallback_modalities: vec![
                M5CompanionComponentFallbackModality::List,
                M5CompanionComponentFallbackModality::Textual,
                M5CompanionComponentFallbackModality::Cli,
            ],
            reaches_canonical_truth: true,
            keyboard_reach: CompanionComponentNonVisualReachState::ReachableAndLabeled,
            screen_reader_reach: CompanionComponentNonVisualReachState::ReachableAndLabeled,
            cli_reach: CompanionComponentNonVisualReachState::ReachableAndLabeled,
            export_summary: CompanionComponentExportSummaryState::ReconstructableWithoutScreenshot,
            export_summary_ref: "summary:session-follow-tile-tenant-narrowed:a11y".to_owned(),
            copy_export: copy_export(&[
                "presenter_identity",
                "session_identity",
                "client_scope",
                "keyboard_route",
            ]),
            full_companion_claim: M5CompanionComponentClaim::LiveCompanionSafe,
            claim_conditions: vec![condition(
                M5CompanionComponentClaimDimension::TenantScope,
                M5CompanionComponentConditionState::TenantScopeNarrowed,
            )],
            claim_narrow: Some(CompanionComponentClaimAutoNarrow {
                narrowed_to: M5CompanionComponentClaim::NarrowedTenantProjection,
                binding_dimension: M5CompanionComponentClaimDimension::TenantScope,
                trigger: M5CompanionDowngradeTrigger::ClientScopeUnstated,
                narrowed_label:
                    "The followed session's tenant scope has narrowed from what was granted and must be reconciled — shown as a narrowed-tenant projection that names its presenter and session identity, never as an in-scope live session"
                        .to_owned(),
                preserves_canonical_identity: true,
                preserves_lineage_continuity: true,
            }),
            lineage_preserved: true,
            rendering_surfaces: rendering_surfaces(),
            narrowing_disclosures: narrowed_surfaces(&[
                "presenter_identity",
                "session_identity",
                "client_scope",
            ]),
            required_labels: all_required_labels(),
            consumer_surfaces: base_consumers(&[M5CompanionConsumerSurface::SessionFollowUi]),
            source_refs: vec![
                "UI/UX Spec §16.13 session follow".to_owned(),
                COMPANION_COMPONENT_A11Y_DOC_REF.to_owned(),
            ],
            observed_at: "2026-07-09T00:00:00Z".to_owned(),
            evidence_refs: ev("session-follow-tile-tenant-narrowed"),
        },
        // Incident-snapshot card — hierarchy-heavy (nested service / run / severity / status
        // lineage); its freshness is live and it is a self-sufficient cached, summary-first
        // projection (awareness-only, not itself a live remediation surface), so it is
        // cached-continuity-safe and binds its nested lineage to a flat list / textual path (green).
        CompanionComponentAccessibilityRow {
            record_kind: COMPANION_COMPONENT_A11Y_ROW_RECORD_KIND.to_owned(),
            schema_version: COMPANION_COMPONENT_A11Y_SCHEMA_VERSION,
            row_id: "a11y:incident-snapshot-card".to_owned(),
            component_family: M5CompanionComponentFamily::IncidentSnapshotCard,
            source_family_schema_ref: COMPANION_COMPONENT_A11Y_COMPONENT_MATRIX_REF.to_owned(),
            object_context_ref: "companion:incident-snapshot-card:0005".to_owned(),
            fallback_modalities: vec![
                M5CompanionComponentFallbackModality::Structured,
                M5CompanionComponentFallbackModality::List,
                M5CompanionComponentFallbackModality::Textual,
                M5CompanionComponentFallbackModality::Cli,
            ],
            reaches_canonical_truth: true,
            keyboard_reach: CompanionComponentNonVisualReachState::ReachableAndLabeled,
            screen_reader_reach: CompanionComponentNonVisualReachState::ReachableAndLabeled,
            cli_reach: CompanionComponentNonVisualReachState::ReachableAndLabeled,
            export_summary: CompanionComponentExportSummaryState::ReconstructableWithoutScreenshot,
            export_summary_ref: "summary:incident-snapshot-card:a11y".to_owned(),
            copy_export: copy_export(&[
                "service_identity",
                "severity",
                "incident_status",
                "freshness",
            ]),
            full_companion_claim: M5CompanionComponentClaim::CachedContinuitySafe,
            claim_conditions: vec![condition(
                M5CompanionComponentClaimDimension::ObjectFreshness,
                M5CompanionComponentConditionState::LiveInScope,
            )],
            claim_narrow: None,
            lineage_preserved: true,
            rendering_surfaces: rendering_surfaces(),
            narrowing_disclosures: parity_surfaces(&[
                "service_identity",
                "severity",
                "incident_status",
            ]),
            required_labels: all_required_labels(),
            consumer_surfaces: base_consumers(&[M5CompanionConsumerSurface::IncidentAwarenessUi]),
            source_refs: vec![
                "UX Design System §16.37 incident snapshot cards".to_owned(),
                COMPANION_COMPONENT_A11Y_DOC_REF.to_owned(),
            ],
            observed_at: "2026-07-09T00:00:00Z".to_owned(),
            evidence_refs: ev("incident-snapshot-card"),
        },
        // Desktop-handoff sheet (handoff validity revoked) — the handoff target is revoked and no
        // longer resolves exactly, so the sheet auto-narrows to a revoked-handoff projection rather
        // than implying it will open the intended object on desktop, while naming its target object
        // and identity (yellow).
        CompanionComponentAccessibilityRow {
            record_kind: COMPANION_COMPONENT_A11Y_ROW_RECORD_KIND.to_owned(),
            schema_version: COMPANION_COMPONENT_A11Y_SCHEMA_VERSION,
            row_id: "a11y:desktop-handoff-sheet-handoff-revoked".to_owned(),
            component_family: M5CompanionComponentFamily::DesktopHandoffSheet,
            source_family_schema_ref: COMPANION_COMPONENT_A11Y_COMPONENT_MATRIX_REF.to_owned(),
            object_context_ref: "companion:desktop-handoff-sheet:0006".to_owned(),
            fallback_modalities: vec![
                M5CompanionComponentFallbackModality::List,
                M5CompanionComponentFallbackModality::Textual,
                M5CompanionComponentFallbackModality::Cli,
            ],
            reaches_canonical_truth: true,
            keyboard_reach: CompanionComponentNonVisualReachState::ReachableAndLabeled,
            screen_reader_reach: CompanionComponentNonVisualReachState::ReachableAndLabeled,
            cli_reach: CompanionComponentNonVisualReachState::ReachableAndLabeled,
            export_summary: CompanionComponentExportSummaryState::ReconstructableWithoutScreenshot,
            export_summary_ref: "summary:desktop-handoff-sheet-handoff-revoked:a11y".to_owned(),
            copy_export: copy_export(&[
                "target_object",
                "target_identity",
                "handoff_target",
                "keyboard_route",
            ]),
            full_companion_claim: M5CompanionComponentClaim::LiveCompanionSafe,
            claim_conditions: vec![condition(
                M5CompanionComponentClaimDimension::HandoffValidity,
                M5CompanionComponentConditionState::HandoffRevoked,
            )],
            claim_narrow: Some(CompanionComponentClaimAutoNarrow {
                narrowed_to: M5CompanionComponentClaim::RevokedHandoffProjection,
                binding_dimension: M5CompanionComponentClaimDimension::HandoffValidity,
                trigger: M5CompanionDowngradeTrigger::HandoffTargetUnresolved,
                narrowed_label:
                    "The desktop-handoff target is revoked and no longer resolves exactly — shown as a revoked-handoff projection that names its target object and identity, never as a sheet that will open the intended object on desktop"
                        .to_owned(),
                preserves_canonical_identity: true,
                preserves_lineage_continuity: true,
            }),
            lineage_preserved: true,
            rendering_surfaces: rendering_surfaces(),
            narrowing_disclosures: narrowed_surfaces(&[
                "target_object",
                "target_identity",
                "handoff_target",
            ]),
            required_labels: all_required_labels(),
            consumer_surfaces: base_consumers(&[M5CompanionConsumerSurface::DesktopHandoffUi]),
            source_refs: vec![
                "UI/UX Spec §16.13 desktop handoff".to_owned(),
                COMPANION_COMPONENT_A11Y_DOC_REF.to_owned(),
            ],
            observed_at: "2026-07-09T00:00:00Z".to_owned(),
            evidence_refs: ev("desktop-handoff-sheet-handoff-revoked"),
        },
    ]
}
