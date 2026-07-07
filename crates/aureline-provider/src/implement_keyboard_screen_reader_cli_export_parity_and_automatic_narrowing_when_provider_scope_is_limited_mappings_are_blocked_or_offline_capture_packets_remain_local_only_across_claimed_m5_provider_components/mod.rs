//! Keyboard / screen-reader / CLI / export parity and honest automatic narrowing for the
//! M5 provider-account / mapping / sync / offline-capture / privacy-redaction components.
//!
//! This module is the M05-922 accessibility-and-auto-narrowing capstone over the frozen
//! M5 provider-account / offline-capture component matrix
//! ([`crate::freeze_the_m5_provider_account_row_project_or_board_mapping_row_sync_behavior_row_offline_capture_row_and_privacy_redaction_row_component_matrix`]).
//! Where the freeze matrix defines the reusable provider-account row, project/board mapping
//! row, sync-behavior row, offline-capture row, and privacy/redaction row primitives, and the
//! 917-921 implementation / consumer lanes resolve their per-surface truth, this lane certifies
//! — per component family — that provider-boundary claims stay **keyboard-complete,
//! assistive-tech-reachable, CLI/export-safe, and self-narrowing** rather than presenting a
//! limited-scope session, a policy-blocked mapping, or a local-only offline-capture packet as a
//! still fully committed, ready-to-write provider surface:
//!
//! - **Keyboard / screen-reader / CLI reach.** Every family exposes a keyboard-complete,
//!   screen-reader-reachable, and CLI/headless-reachable path into the same provider identity,
//!   tenant scope, effective write scope, mapping origin, sync mode, queued-draft state, and
//!   redaction / export boundary the rich component shows — never a hover-only chip that
//!   strands assistive-tech or headless users. Hierarchy-heavy families (the offline-capture
//!   row's nested packet / queued-draft / destination lineage) additionally bind their tree to
//!   a flat list / textual path.
//! - **Export parity.** The support / release / evaluation export reconstructs each
//!   component's meaning from typed tokens and opaque refs without a screenshot, preserving the
//!   same stable connection states, tenant / write-scope labels, mapping origins, sync modes,
//!   queued-draft counts, redaction classes, and narrowing reasons shown in-product so
//!   account / mapping / sync / offline / privacy truth can be reconstructed without
//!   screenshots or private team memory.
//! - **Honest auto-narrowing.** When provider scope is limited, a session is stale, a mapping
//!   is policy-blocked, or an offline-capture packet remains local-only, the component's
//!   provider claim auto-narrows from `ProviderCommitted` / `ReviewableProjection` to a
//!   limited-scope / stale-session / policy-blocked-mapping / local-only-packet projection,
//!   discloses the narrowing with a precise trigger and binding dimension, and preserves the
//!   canonical account / mapping / queued-draft / redaction lineage — the underlying provider
//!   lineage is never dropped opaquely. A component with every dimension intact must NOT carry
//!   a spurious narrowing, and a cached or offline state can never keep a committed claim.
//! - **Cross-surface disclosure.** The same narrowed state surfaces in the account-settings UI,
//!   mapping-picker, sync-status, offline-queue, privacy-review, status-bar, general product
//!   UI, headless CLI, and support / release exports so product, docs, and release publication
//!   stay aligned on provider-boundary downgrade behavior rather than drifting in copy — a
//!   committed-looking surface can never outrun the scope / mapping / packet / redaction proof
//!   it is being viewed away from.
//!
//! Each [`ProviderComponentAccessibilityRow`] keys on one
//! [`crate::freeze_the_m5_provider_account_row_project_or_board_mapping_row_sync_behavior_row_offline_capture_row_and_privacy_redaction_row_component_matrix::M5ProviderAccountOfflineComponentFamily`]
//! and reuses that frozen family vocabulary plus the frozen [`M5ProviderRequiredLabel`] and
//! [`M5ProviderDowngradeTrigger`] and the shared [`M5ProviderConsumerSurface`] consumer surfaces
//! rather than minting parallel synonyms, so the certified labels stay byte-identical to the
//! matrix and the sibling primitive packets.
//!
//! The packet is metadata-only: raw credentials, tokens, request bodies, and endpoint secrets
//! never cross this boundary; the packet carries only typed class tokens, opaque account /
//! mapping / packet refs, booleans, and redacted labels so support, release, and diagnostics
//! exports can reconstruct exactly what an accessible fallback would have shown without leaking
//! provider material.

#[cfg(test)]
mod tests;

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

// Reused frozen component vocabulary — the capstone certifies the freeze matrix's families,
// required labels, downgrade triggers, and consumer surfaces rather than mint parallel ones.
use crate::freeze_the_m5_provider_account_row_project_or_board_mapping_row_sync_behavior_row_offline_capture_row_and_privacy_redaction_row_component_matrix::{
    M5ProviderAccountOfflineComponentFamily, M5ProviderConsumerSurface, M5ProviderDowngradeTrigger,
    M5ProviderRequiredLabel,
};

/// Schema version stamped on the M05-922 provider-account / offline-capture component
/// accessibility fallback packet.
pub const PROVIDER_COMPONENT_A11Y_FALLBACK_SCHEMA_VERSION: u32 = 1;

/// Stable record-kind tag carried by [`ProviderComponentAccessibilityPacket`].
pub const PROVIDER_COMPONENT_A11Y_FALLBACK_RECORD_KIND: &str =
    "m5_provider_account_offline_capture_component_accessibility_fallback_packet";

/// Stable record-kind tag carried by each [`ProviderComponentAccessibilityRow`].
pub const PROVIDER_COMPONENT_A11Y_FALLBACK_ROW_RECORD_KIND: &str =
    "m5_provider_account_offline_capture_component_accessibility_fallback_row";

/// Repo-relative path of the boundary schema.
pub const PROVIDER_COMPONENT_A11Y_FALLBACK_SCHEMA_REF: &str =
    "schemas/ui/m5-provider-account-offline-capture-component-accessibility-fallback.schema.json";

/// Repo-relative path of the contract doc.
pub const PROVIDER_COMPONENT_A11Y_FALLBACK_DOC_REF: &str =
    "docs/providers/m5_provider_account_offline_capture_component_accessibility_fallback.md";

/// Repo-relative path of the frozen provider-account / offline-capture component matrix this
/// lane certifies.
pub const PROVIDER_COMPONENT_A11Y_FALLBACK_COMPONENT_MATRIX_REF: &str =
    "schemas/ui/m5-provider-account-offline-capture-component-matrix.schema.json";

/// Repo-relative path of the protected fixture directory.
pub const PROVIDER_COMPONENT_A11Y_FALLBACK_FIXTURE_DIR: &str =
    "fixtures/ui/m5-provider-account-offline-capture-component-accessibility-fallback";

/// Repo-relative path of the checked support-export artifact (the `include_str!` canonical).
pub const PROVIDER_COMPONENT_A11Y_FALLBACK_ARTIFACT_REF: &str =
    "artifacts/release/m5-provider-account-offline-capture-component-accessibility-fallback/support_export.json";

/// Repo-relative path of the checked machine-readable matrix CSV.
pub const PROVIDER_COMPONENT_A11Y_FALLBACK_CSV_REF: &str =
    "artifacts/release/m5-provider-account-offline-capture-component-accessibility-fallback/matrix.csv";

/// Repo-relative path of the checked Markdown report.
pub const PROVIDER_COMPONENT_A11Y_FALLBACK_REPORT_REF: &str =
    "artifacts/release/m5-provider-account-offline-capture-component-accessibility-fallback.md";

/// The reusable component families that render a non-linear hierarchy (the offline-capture
/// row's nested packet / queued-draft / destination lineage) and therefore MUST bind their tree
/// to an equivalent flat list / textual path so the hierarchy is navigable non-visually.
const fn family_is_hierarchy_heavy(family: M5ProviderAccountOfflineComponentFamily) -> bool {
    matches!(
        family,
        M5ProviderAccountOfflineComponentFamily::OfflineCaptureRow
    )
}

/// The provider dimension whose weakening a family primarily discloses. Every row must model at
/// least this dimension so its key weakening axis is covered.
const fn family_primary_dimension(
    family: M5ProviderAccountOfflineComponentFamily,
) -> M5ProviderComponentClaimDimension {
    match family {
        M5ProviderAccountOfflineComponentFamily::ProviderAccountRow => {
            M5ProviderComponentClaimDimension::ConnectionAndScope
        }
        M5ProviderAccountOfflineComponentFamily::ProjectOrBoardMappingRow => {
            M5ProviderComponentClaimDimension::MappingOrigin
        }
        M5ProviderAccountOfflineComponentFamily::SyncBehaviorRow => {
            M5ProviderComponentClaimDimension::SyncBehavior
        }
        M5ProviderAccountOfflineComponentFamily::OfflineCaptureRow => {
            M5ProviderComponentClaimDimension::OfflineCapture
        }
        M5ProviderAccountOfflineComponentFamily::PrivacyRedactionRow => {
            M5ProviderComponentClaimDimension::RedactionBoundary
        }
    }
}

/// A rendered fallback modality for a provider-account / offline-capture component.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ProviderComponentFallbackModality {
    /// A rich, structured (nested packet / queued-draft tree) projection.
    Structured,
    /// A flat list projection.
    List,
    /// A textual / source-first projection.
    Textual,
    /// A CLI / headless line projection.
    Cli,
}

impl M5ProviderComponentFallbackModality {
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
pub enum M5ProviderComponentRenderingSurface {
    /// The full-capability desktop provider surface.
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

impl M5ProviderComponentRenderingSurface {
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
pub enum ProviderComponentNonVisualReachState {
    /// Fully traversable and labeled with no loss.
    ReachableAndLabeled,
    /// Reachable and labeled, but with a disclosed reduction (yellow).
    DisclosedReducedButReachable,
    /// A view-only / hover-only surface that traps keyboard / assistive-tech / headless users
    /// (red).
    ViewOnlyTrap,
}

impl ProviderComponentNonVisualReachState {
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

/// Whether an export-safe summary preserves the component meaning without a screenshot.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ProviderComponentExportSummaryState {
    /// The component meaning reconstructs from the summary without a screenshot.
    ReconstructableWithoutScreenshot,
    /// Partial capture, but disclosed (yellow).
    DisclosedPartialCapture,
    /// The export relies on a screenshot to carry meaning (red).
    AbsentNeedsScreenshot,
}

impl ProviderComponentExportSummaryState {
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
pub enum ProviderComponentNarrowingDisclosureState {
    /// Full label and summary parity with the desktop surface.
    ParityPreserved,
    /// Reduced interactivity, disclosed with preserved labels (yellow).
    DisclosedNarrowed,
    /// Interactivity, state, or actions dropped without disclosure (red).
    SilentlyDropped,
}

impl ProviderComponentNarrowingDisclosureState {
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

/// The provider claim ceiling a component asserts: how strong a provider-boundary posture it
/// lets a surface present. Auto-narrowing lowers this ceiling when a provider dimension weakens
/// so a limited-scope session, a policy-blocked mapping, or a local-only offline-capture packet
/// can never keep an old `ProviderCommitted` or `ReviewableProjection` label — a cached or
/// offline state never masquerades as committed.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ProviderComponentClaim {
    /// Provider-committed: a reachable, in-scope, session-fresh account with a resolved
    /// mapping and a routed sync — the strongest claim, a surface Aureline can read and write
    /// against right now and commit to the provider.
    ProviderCommitted,
    /// Reviewable projection: a self-sufficient, reviewable read-only projection (state a
    /// reviewer can read) that is not itself a certified committed write path.
    ReviewableProjection,
    /// Limited-scope projection: usable, but the effective write scope is limited — the surface
    /// cannot perform the full committed write and stays a read / limited-write projection.
    LimitedScopeProjection,
    /// Stale-session projection: the session is stale and only a cached read is available; the
    /// surface must re-authenticate before it can be trusted as live.
    StaleSessionProjection,
    /// Policy-blocked mapping: the project/board mapping is policy-blocked; no committed
    /// destination can resolve and the row stays a blocked-mapping explanation.
    PolicyBlockedMapping,
    /// Local-only packet: the offline-capture packet remains local-only; nothing has been
    /// published to the provider and the row stays a queued, publish-later capture.
    LocalOnlyPacket,
}

impl M5ProviderComponentClaim {
    /// Every claim tier, strongest first.
    pub const ALL: [Self; 6] = [
        Self::ProviderCommitted,
        Self::ReviewableProjection,
        Self::LimitedScopeProjection,
        Self::StaleSessionProjection,
        Self::PolicyBlockedMapping,
        Self::LocalOnlyPacket,
    ];

    /// Capability rank; a higher rank asserts a stronger provider posture. Narrowing lowers
    /// rank.
    pub const fn capability_rank(self) -> u8 {
        match self {
            Self::ProviderCommitted => 5,
            Self::ReviewableProjection => 4,
            Self::LimitedScopeProjection => 3,
            Self::StaleSessionProjection => 2,
            Self::PolicyBlockedMapping => 1,
            Self::LocalOnlyPacket => 0,
        }
    }

    /// Returns true when this claim asserts a fully committed, ready-to-write provider surface.
    pub const fn asserts_provider_committed(self) -> bool {
        matches!(self, Self::ProviderCommitted)
    }

    /// Returns true when this claim asserts a fully self-sufficient (committed or reviewable)
    /// projection.
    pub const fn asserts_full_projection(self) -> bool {
        matches!(self, Self::ProviderCommitted | Self::ReviewableProjection)
    }

    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ProviderCommitted => "provider_committed",
            Self::ReviewableProjection => "reviewable_projection",
            Self::LimitedScopeProjection => "limited_scope_projection",
            Self::StaleSessionProjection => "stale_session_projection",
            Self::PolicyBlockedMapping => "policy_blocked_mapping",
            Self::LocalOnlyPacket => "local_only_packet",
        }
    }
}

/// The provider dimension whose state governs how far a component may claim to be a committed,
/// ready-to-write provider surface. The dimensions map 1:1 to the five frozen component
/// families so every family carries an honest narrowing path.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ProviderComponentClaimDimension {
    /// Connection and scope: is the provider-account row reachable, session-fresh, and in full
    /// write scope, or is the scope limited / the session stale?
    ConnectionAndScope,
    /// Mapping origin: does the project/board mapping row resolve a committed destination, or is
    /// the mapping policy-blocked?
    MappingOrigin,
    /// Sync behavior: does the sync-behavior row carry a stated sync mode and effective write
    /// scope with a visible queued-draft state, or is the behavior unstated?
    SyncBehavior,
    /// Offline capture: does the offline-capture row route its packet to the provider, or does
    /// the packet remain local-only?
    OfflineCapture,
    /// Redaction boundary: does the privacy/redaction row name its redaction class and
    /// metadata-safe export boundary, or is the boundary hidden?
    RedactionBoundary,
}

impl M5ProviderComponentClaimDimension {
    /// Every dimension, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::ConnectionAndScope,
        Self::MappingOrigin,
        Self::SyncBehavior,
        Self::OfflineCapture,
        Self::RedactionBoundary,
    ];

    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ConnectionAndScope => "connection_and_scope",
            Self::MappingOrigin => "mapping_origin",
            Self::SyncBehavior => "sync_behavior",
            Self::OfflineCapture => "offline_capture",
            Self::RedactionBoundary => "redaction_boundary",
        }
    }
}

/// The observed condition of one provider dimension. Anything weaker than [`Self::InScopeCommitted`]
/// imposes a narrowing ceiling on the component's provider claim. The four spec axes the lane
/// must auto-narrow on — limited provider scope, a stale session, a policy-blocked mapping, and
/// a local-only offline-capture packet — are [`Self::ScopeLimited`], [`Self::SessionStale`],
/// [`Self::MappingPolicyBlocked`], and [`Self::PacketLocalOnly`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ProviderComponentConditionState {
    /// Reachable, session-fresh, in-scope, mapped, and routed — imposes no ceiling.
    InScopeCommitted,
    /// Effective write scope is limited — the surface cannot commit the full write; provider
    /// claim drops to a limited-scope projection.
    ScopeLimited,
    /// The session is stale — only a cached read is available; provider claim drops to a
    /// stale-session projection.
    SessionStale,
    /// The mapping is policy-blocked — no committed destination resolves; provider claim drops
    /// to a policy-blocked mapping.
    MappingPolicyBlocked,
    /// The offline-capture packet remains local-only — nothing has been published; provider
    /// claim drops to a local-only packet.
    PacketLocalOnly,
}

impl M5ProviderComponentConditionState {
    /// Every condition state, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::InScopeCommitted,
        Self::ScopeLimited,
        Self::SessionStale,
        Self::MappingPolicyBlocked,
        Self::PacketLocalOnly,
    ];

    /// Returns true when the dimension is weaker than committed and therefore imposes a
    /// narrowing ceiling.
    pub const fn is_weak(self) -> bool {
        !matches!(self, Self::InScopeCommitted)
    }

    /// Returns true when the condition reflects a cached or offline state that must never be
    /// shown as provider-committed.
    pub const fn is_cached_or_offline(self) -> bool {
        matches!(self, Self::SessionStale | Self::PacketLocalOnly)
    }

    /// The strongest provider claim this condition state permits.
    pub const fn permitted_ceiling(self) -> M5ProviderComponentClaim {
        match self {
            Self::InScopeCommitted => M5ProviderComponentClaim::ProviderCommitted,
            Self::ScopeLimited => M5ProviderComponentClaim::LimitedScopeProjection,
            Self::SessionStale => M5ProviderComponentClaim::StaleSessionProjection,
            Self::MappingPolicyBlocked => M5ProviderComponentClaim::PolicyBlockedMapping,
            Self::PacketLocalOnly => M5ProviderComponentClaim::LocalOnlyPacket,
        }
    }

    /// The frozen downgrade trigger this condition names when its weakness binds a narrowing.
    /// Each state maps to the on-topic frozen trigger the freeze matrix already governs, so the
    /// certified reason stays byte-identical to the matrix.
    pub const fn default_trigger(self) -> M5ProviderDowngradeTrigger {
        match self {
            // The committed baseline never narrows; kept for exhaustiveness.
            Self::InScopeCommitted => M5ProviderDowngradeTrigger::ConnectionStateUnstated,
            Self::ScopeLimited => M5ProviderDowngradeTrigger::WriteScopeUnstated,
            Self::SessionStale => M5ProviderDowngradeTrigger::ConnectionStateUnstated,
            Self::MappingPolicyBlocked => M5ProviderDowngradeTrigger::MappingOriginUnstated,
            Self::PacketLocalOnly => M5ProviderDowngradeTrigger::OfflineCaptureStateUnstated,
        }
    }

    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::InScopeCommitted => "in_scope_committed",
            Self::ScopeLimited => "scope_limited",
            Self::SessionStale => "session_stale",
            Self::MappingPolicyBlocked => "mapping_policy_blocked",
            Self::PacketLocalOnly => "packet_local_only",
        }
    }
}

/// One provider dimension's observed condition on a component.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProviderComponentClaimConditionEntry {
    /// Which dimension this entry describes.
    pub dimension: M5ProviderComponentClaimDimension,
    /// The observed condition state of the dimension.
    pub state: M5ProviderComponentConditionState,
}

/// An honest provider-claim auto-narrow block. When a provider dimension weakens, the
/// component's provider claim lowers to the permitted ceiling, names the binding dimension and
/// frozen trigger, and preserves the canonical account / mapping / queued-draft / redaction
/// lineage rather than silently dropping it — the underlying provider lineage is never erased
/// opaquely.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProviderComponentClaimAutoNarrow {
    /// The provider claim the component is narrowed to.
    pub narrowed_to: M5ProviderComponentClaim,
    /// The dimension whose weakness bound the narrowing (the one imposing the strongest ceiling
    /// constraint).
    pub binding_dimension: M5ProviderComponentClaimDimension,
    /// The frozen downgrade trigger (reused vocabulary) the narrowing names.
    pub trigger: M5ProviderDowngradeTrigger,
    /// A precise, non-generic label safe to render.
    pub narrowed_label: String,
    /// The canonical provider identity, tenant scope, mapping origin, queued-draft state, and
    /// redaction posture are preserved rather than dropped; must hold.
    pub preserves_canonical_identity: bool,
    /// The underlying account / mapping / queued-draft / redaction lineage is preserved (never
    /// dropped) across the narrowing; must hold so limited-scope, stale-session, policy-blocked,
    /// and local-only states never fail opaquely.
    pub preserves_lineage_continuity: bool,
}

impl ProviderComponentClaimAutoNarrow {
    /// Whether the auto-narrow block is honest: it preserves canonical identity and provider
    /// lineage and carries a precise, non-generic label.
    pub fn is_honest(&self) -> bool {
        self.preserves_canonical_identity
            && self.preserves_lineage_continuity
            && !label_is_generic(&self.narrowed_label)
    }
}

/// Copy / export parity for a component's accessible fallback: the same truth must be copyable
/// as text / JSON / Markdown, and a screenshot is never the only export.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProviderComponentCopyExportParity {
    /// The copy / export formats offered (must include text, json, markdown).
    #[serde(default)]
    pub formats: Vec<String>,
    /// The named export fields the summary carries.
    #[serde(default)]
    pub export_fields: Vec<String>,
    /// A screenshot is never the only export; must always hold.
    pub screenshot_only_prohibited: bool,
}

impl ProviderComponentCopyExportParity {
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
pub struct ProviderComponentRenderingNarrowingDisclosure {
    /// The rendering surface being narrowed.
    pub rendering_surface: M5ProviderComponentRenderingSurface,
    /// How the surface discloses its reduced interactivity.
    pub state: ProviderComponentNarrowingDisclosureState,
    /// The labels preserved across the narrowing.
    #[serde(default)]
    pub preserved_labels: Vec<String>,
    /// The interactions reduced on the narrowed surface.
    #[serde(default)]
    pub reduced_interactions: Vec<String>,
}

/// Derived qualification status for a provider-account accessibility row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ProviderComponentAccessibilityStatus {
    /// Full keyboard / screen-reader / CLI / export parity with no narrowing (green).
    Parity,
    /// Reduced but fully disclosed, reachable, and honestly auto-narrowed (yellow).
    NarrowedDisclosed,
    /// Strands assistive tech, needs a screenshot, over-claims commitment, or drops state
    /// silently (red).
    Stranded,
}

impl ProviderComponentAccessibilityStatus {
    /// Stable token recorded in the summary / CSV.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Parity => "parity",
            Self::NarrowedDisclosed => "narrowed_disclosed",
            Self::Stranded => "stranded",
        }
    }
}

/// Accessibility / auto-narrowing parity row for one provider-account / offline-capture
/// component family.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProviderComponentAccessibilityRow {
    /// Record kind; must equal [`PROVIDER_COMPONENT_A11Y_FALLBACK_ROW_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`PROVIDER_COMPONENT_A11Y_FALLBACK_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable row id.
    pub row_id: String,
    /// The frozen component family this row certifies.
    pub component_family: M5ProviderAccountOfflineComponentFamily,
    /// Ref to the frozen matrix family schema this row certifies.
    pub source_family_schema_ref: String,
    /// Opaque ref to the account / mapping / sync / offline-packet / redaction object this
    /// component acts on; stays visible on every surface, so this is never empty.
    pub provider_context_ref: String,
    /// Rendered modalities offered; a hierarchy-heavy family must also offer a non-visual
    /// (list / textual / cli) path.
    #[serde(default)]
    pub fallback_modalities: Vec<M5ProviderComponentFallbackModality>,
    /// The non-visual / CLI path reaches the same canonical provider identity, scope, mapping,
    /// sync, queued-draft, and redaction truth as the rich surface; must hold.
    pub reaches_canonical_truth: bool,
    /// Keyboard reach into the non-visual path.
    pub keyboard_reach: ProviderComponentNonVisualReachState,
    /// Screen-reader reach into the non-visual path.
    pub screen_reader_reach: ProviderComponentNonVisualReachState,
    /// CLI / headless reach into the non-visual path.
    pub cli_reach: ProviderComponentNonVisualReachState,
    /// Whether the export-safe summary preserves component meaning.
    pub export_summary: ProviderComponentExportSummaryState,
    /// Ref to the export-safe summary object for this component.
    pub export_summary_ref: String,
    /// The copy / export parity of the accessible fallback.
    pub copy_export: ProviderComponentCopyExportParity,
    /// The full provider claim this family asserts when every dimension is intact.
    pub full_provider_claim: M5ProviderComponentClaim,
    /// The observed condition of each modeled provider dimension.
    #[serde(default)]
    pub claim_conditions: Vec<ProviderComponentClaimConditionEntry>,
    /// The honest auto-narrow block, present only when some dimension weakens below the family's
    /// full claim.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub claim_narrow: Option<ProviderComponentClaimAutoNarrow>,
    /// Whether the underlying provider lineage is preserved on this component regardless of
    /// narrowing; must hold so limited-scope, stale-session, policy-blocked, and local-only
    /// states never fail opaquely.
    pub lineage_preserved: bool,
    /// Rendering surfaces this component is certified on.
    #[serde(default)]
    pub rendering_surfaces: Vec<M5ProviderComponentRenderingSurface>,
    /// Per-surface narrowing disclosures.
    #[serde(default)]
    pub narrowing_disclosures: Vec<ProviderComponentRenderingNarrowingDisclosure>,
    /// The required labels the accessible fallback preserves (reused vocabulary).
    #[serde(default)]
    pub required_labels: Vec<M5ProviderRequiredLabel>,
    /// Semantic consumer surfaces this component is embedded in (reused vocabulary).
    #[serde(default)]
    pub consumer_surfaces: Vec<M5ProviderConsumerSurface>,
    /// Source contract refs backing this row.
    #[serde(default)]
    pub source_refs: Vec<String>,
    /// ISO 8601 UTC timestamp the accessibility posture was observed.
    pub observed_at: String,
    /// Evidence packet refs backing this row.
    #[serde(default)]
    pub evidence_refs: Vec<String>,
}

impl ProviderComponentAccessibilityRow {
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

    /// The condition state observed for one dimension, or `InScopeCommitted` when the row does
    /// not model that dimension.
    pub fn condition_for(
        &self,
        dimension: M5ProviderComponentClaimDimension,
    ) -> M5ProviderComponentConditionState {
        self.claim_conditions
            .iter()
            .find(|c| c.dimension == dimension)
            .map(|c| c.state)
            .unwrap_or(M5ProviderComponentConditionState::InScopeCommitted)
    }

    /// Whether any modeled dimension is weaker than committed.
    pub fn has_weak_dimension(&self) -> bool {
        self.claim_conditions.iter().any(|c| c.state.is_weak())
    }

    /// The strongest provider claim permitted after applying every modeled dimension's ceiling,
    /// capped at the family's full claim.
    pub fn permitted_claim(&self) -> M5ProviderComponentClaim {
        let mut permitted = self.full_provider_claim;
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
    pub fn binding_condition(&self) -> Option<&ProviderComponentClaimConditionEntry> {
        let mut binding: Option<(&ProviderComponentClaimConditionEntry, u8)> = None;
        for condition in &self.claim_conditions {
            if !condition.state.is_weak() {
                continue;
            }
            let ceiling = condition.state.permitted_ceiling();
            if ceiling.capability_rank() >= self.full_provider_claim.capability_rank() {
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
    pub fn binding_dimension(&self) -> Option<M5ProviderComponentClaimDimension> {
        self.binding_condition().map(|c| c.dimension)
    }

    /// The provider claim this component effectively asserts after narrowing.
    pub fn effective_claim(&self) -> M5ProviderComponentClaim {
        match &self.claim_narrow {
            Some(narrow) => narrow.narrowed_to,
            None => self.full_provider_claim,
        }
    }

    /// AC / auto-narrowing honesty: a limited-scope session, a stale session, a policy-blocked
    /// mapping, or a local-only packet can no longer keep an old `ProviderCommitted` /
    /// `ReviewableProjection` label. The effective claim never exceeds the permitted ceiling;
    /// when a dimension narrows below the full claim, an honest narrow block is present, narrows
    /// to exactly the permitted ceiling, binds to the ceiling-imposing dimension with its frozen
    /// trigger, and preserves canonical identity and provider lineage. When nothing narrows, no
    /// spurious narrow block is present.
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

    /// AC / commit honesty: a cached or offline state (stale session, local-only packet) never
    /// keeps a committed claim. When such a state is modeled, the effective claim must not
    /// assert `ProviderCommitted`.
    pub fn commit_honesty_holds(&self) -> bool {
        let has_cached_or_offline = self
            .claim_conditions
            .iter()
            .any(|c| c.state.is_cached_or_offline());
        !(has_cached_or_offline && self.effective_claim().asserts_provider_committed())
    }

    /// AC / assistive-tech reach: accessibility and export surfaces reach the same canonical
    /// truth — no keyboard / screen-reader / CLI trap, a hierarchy-heavy family offers a
    /// non-visual fallback, and the export reconstructs meaning without a screenshot.
    pub fn reaches_canonical_truth_via_at(&self) -> bool {
        self.reaches_canonical_truth
            && !self.provider_context_ref.trim().is_empty()
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

    /// AC / no-loss: limited-scope, stale-session, policy-blocked, and local-only states
    /// preserve the underlying provider lineage. The row must assert `lineage_preserved`, and
    /// any narrow block must preserve lineage continuity too.
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
    /// interactivity and keeps its labels, so product / docs / release publication stay aligned
    /// on the same narrowed state.
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
        M5ProviderRequiredLabel::MANDATORY
            .iter()
            .all(|label| self.required_labels.contains(label))
    }

    /// Derived qualification status.
    pub fn status(&self) -> ProviderComponentAccessibilityStatus {
        if !self.claim_is_honest()
            || !self.commit_honesty_holds()
            || !self.reaches_canonical_truth_via_at()
            || !self.export_preserves_meaning()
            || !self.preserves_lineage_continuity()
            || !self.narrowing_disclosed()
            || !self.models_primary_dimension()
            || !self.preserves_mandatory_labels()
        {
            return ProviderComponentAccessibilityStatus::Stranded;
        }
        if self.is_reduced() {
            ProviderComponentAccessibilityStatus::NarrowedDisclosed
        } else {
            ProviderComponentAccessibilityStatus::Parity
        }
    }

    /// Whether the row's identity and evidence fields are complete.
    pub fn is_complete(&self) -> bool {
        self.record_kind == PROVIDER_COMPONENT_A11Y_FALLBACK_ROW_RECORD_KIND
            && self.schema_version == PROVIDER_COMPONENT_A11Y_FALLBACK_SCHEMA_VERSION
            && !self.row_id.trim().is_empty()
            && !self.source_family_schema_ref.trim().is_empty()
            && !self.provider_context_ref.trim().is_empty()
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
            full = self.full_provider_claim.as_str(),
            effective = self.effective_claim().as_str(),
            status = self.status().as_str(),
        )
    }
}

/// Rolled-up summary of an M05-922 provider-account / offline-capture component accessibility
/// fallback packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProviderComponentAccessibilitySummary {
    pub row_count: usize,
    pub family_count: usize,
    pub hierarchy_heavy_family_count: usize,
    pub all_hierarchy_heavy_have_non_visual_fallback: bool,
    pub all_reach_canonical_truth_via_at: bool,
    pub all_claims_honest: bool,
    pub all_commit_honesty_holds: bool,
    pub all_export_summaries_preserve_meaning: bool,
    pub all_lineage_preserved: bool,
    pub all_narrowing_disclosed: bool,
    pub green_count: usize,
    pub yellow_count: usize,
    pub red_count: usize,
    pub rendering_surface_count: usize,
    pub consumer_surface_count: usize,
}

/// Constructor input for [`ProviderComponentAccessibilityPacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProviderComponentAccessibilityPacketInput {
    pub packet_id: String,
    pub as_of: String,
    pub matrix_ref: String,
    pub rows: Vec<ProviderComponentAccessibilityRow>,
}

/// Checked-in M05-922 provider-account / offline-capture component accessibility fallback
/// packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProviderComponentAccessibilityPacket {
    pub schema_version: u32,
    pub record_kind: String,
    pub packet_id: String,
    pub as_of: String,
    pub matrix_ref: String,
    #[serde(default)]
    pub rows: Vec<ProviderComponentAccessibilityRow>,
    pub summary: ProviderComponentAccessibilitySummary,
}

impl ProviderComponentAccessibilityPacket {
    /// Builds a packet, stamping the record kind, schema version, and computed summary.
    pub fn new(input: ProviderComponentAccessibilityPacketInput) -> Self {
        let mut packet = Self {
            schema_version: PROVIDER_COMPONENT_A11Y_FALLBACK_SCHEMA_VERSION,
            record_kind: PROVIDER_COMPONENT_A11Y_FALLBACK_RECORD_KIND.to_owned(),
            packet_id: input.packet_id,
            as_of: input.as_of,
            matrix_ref: input.matrix_ref,
            rows: input.rows,
            summary: ProviderComponentAccessibilitySummary {
                row_count: 0,
                family_count: 0,
                hierarchy_heavy_family_count: 0,
                all_hierarchy_heavy_have_non_visual_fallback: false,
                all_reach_canonical_truth_via_at: false,
                all_claims_honest: false,
                all_commit_honesty_holds: false,
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
    pub fn represented_families(&self) -> BTreeSet<M5ProviderAccountOfflineComponentFamily> {
        self.rows.iter().map(|r| r.component_family).collect()
    }

    /// Dimensions exercised by some row's claim conditions.
    pub fn exercised_dimensions(&self) -> BTreeSet<M5ProviderComponentClaimDimension> {
        self.rows
            .iter()
            .flat_map(|r| r.claim_conditions.iter().map(|c| c.dimension))
            .collect()
    }

    /// Condition states exercised by some row's claim conditions.
    pub fn exercised_condition_states(&self) -> BTreeSet<M5ProviderComponentConditionState> {
        self.rows
            .iter()
            .flat_map(|r| r.claim_conditions.iter().map(|c| c.state))
            .collect()
    }

    /// Provider claim tiers that appear as an effective claim across the rows.
    pub fn represented_effective_claims(&self) -> BTreeSet<M5ProviderComponentClaim> {
        self.rows.iter().map(|r| r.effective_claim()).collect()
    }

    /// Consumer surfaces ingesting some row in this packet.
    pub fn represented_consumer_surfaces(&self) -> BTreeSet<M5ProviderConsumerSurface> {
        self.rows
            .iter()
            .flat_map(|r| r.consumer_surfaces.iter().copied())
            .collect()
    }

    /// Computes summary fields from the packet contents.
    pub fn computed_summary(&self) -> ProviderComponentAccessibilitySummary {
        let mut rendering = BTreeSet::new();
        let mut consumers: BTreeSet<M5ProviderConsumerSurface> = BTreeSet::new();
        for row in &self.rows {
            rendering.extend(row.rendering_surfaces.iter().copied());
            consumers.extend(row.consumer_surfaces.iter().copied());
        }

        let hierarchy_heavy: Vec<&ProviderComponentAccessibilityRow> = self
            .rows
            .iter()
            .filter(|row| row.is_hierarchy_heavy())
            .collect();

        let mut green = 0;
        let mut yellow = 0;
        let mut red = 0;
        for row in &self.rows {
            match row.status() {
                ProviderComponentAccessibilityStatus::Parity => green += 1,
                ProviderComponentAccessibilityStatus::NarrowedDisclosed => yellow += 1,
                ProviderComponentAccessibilityStatus::Stranded => red += 1,
            }
        }

        ProviderComponentAccessibilitySummary {
            row_count: self.rows.len(),
            family_count: self.represented_families().len(),
            hierarchy_heavy_family_count: hierarchy_heavy.len(),
            all_hierarchy_heavy_have_non_visual_fallback: hierarchy_heavy
                .iter()
                .all(|row| row.has_non_visual_fallback()),
            all_reach_canonical_truth_via_at: self
                .rows
                .iter()
                .all(ProviderComponentAccessibilityRow::reaches_canonical_truth_via_at),
            all_claims_honest: self
                .rows
                .iter()
                .all(ProviderComponentAccessibilityRow::claim_is_honest),
            all_commit_honesty_holds: self
                .rows
                .iter()
                .all(ProviderComponentAccessibilityRow::commit_honesty_holds),
            all_export_summaries_preserve_meaning: self
                .rows
                .iter()
                .all(ProviderComponentAccessibilityRow::export_preserves_meaning),
            all_lineage_preserved: self
                .rows
                .iter()
                .all(ProviderComponentAccessibilityRow::preserves_lineage_continuity),
            all_narrowing_disclosed: self
                .rows
                .iter()
                .all(ProviderComponentAccessibilityRow::narrowing_disclosed),
            green_count: green,
            yellow_count: yellow,
            red_count: red,
            rendering_surface_count: rendering.len(),
            consumer_surface_count: consumers.len(),
        }
    }

    /// Validates the packet and returns every contract violation.
    pub fn validate(&self) -> Vec<ProviderComponentAccessibilityViolation> {
        let mut violations = Vec::new();

        if self.schema_version != PROVIDER_COMPONENT_A11Y_FALLBACK_SCHEMA_VERSION {
            violations.push(ProviderComponentAccessibilityViolation::SchemaVersion {
                expected: PROVIDER_COMPONENT_A11Y_FALLBACK_SCHEMA_VERSION,
                actual: self.schema_version,
            });
        }
        if self.record_kind != PROVIDER_COMPONENT_A11Y_FALLBACK_RECORD_KIND {
            violations.push(ProviderComponentAccessibilityViolation::RecordKind {
                expected: PROVIDER_COMPONENT_A11Y_FALLBACK_RECORD_KIND.to_owned(),
                actual: self.record_kind.clone(),
            });
        }
        if self.packet_id.trim().is_empty()
            || self.as_of.trim().is_empty()
            || self.matrix_ref.trim().is_empty()
        {
            violations.push(ProviderComponentAccessibilityViolation::MissingIdentity);
        }

        let mut row_ids = BTreeSet::new();
        let mut seen_families = BTreeSet::new();
        let mut has_cached_or_offline_row = false;
        for row in &self.rows {
            if !row_ids.insert(row.row_id.clone()) {
                violations.push(ProviderComponentAccessibilityViolation::DuplicateId {
                    id: row.row_id.clone(),
                });
            }
            seen_families.insert(row.component_family);
            if row
                .claim_conditions
                .iter()
                .any(|c| c.state.is_cached_or_offline())
            {
                has_cached_or_offline_row = true;
            }

            if !row.is_complete() {
                violations.push(ProviderComponentAccessibilityViolation::IncompleteRow {
                    id: row.row_id.clone(),
                });
            }

            // Each row must model its family's primary weakening dimension.
            if !row.models_primary_dimension() {
                violations.push(
                    ProviderComponentAccessibilityViolation::MissingPrimaryDimension {
                        id: row.row_id.clone(),
                        dimension: family_primary_dimension(row.component_family),
                    },
                );
            }

            // Each row must preserve every mandatory provider label.
            if !row.preserves_mandatory_labels() {
                violations.push(
                    ProviderComponentAccessibilityViolation::MissingMandatoryLabel {
                        id: row.row_id.clone(),
                    },
                );
            }

            // A hierarchy-heavy family must render a structured tree *and* a non-visual path.
            if row.is_hierarchy_heavy()
                && !row
                    .fallback_modalities
                    .contains(&M5ProviderComponentFallbackModality::Structured)
            {
                violations.push(
                    ProviderComponentAccessibilityViolation::HierarchyHeavyMissingStructured {
                        id: row.row_id.clone(),
                    },
                );
            }

            // AC1: claim never over-asserts a committed / reviewable surface for a weakened one.
            if !row.claim_is_honest() {
                violations.push(ProviderComponentAccessibilityViolation::ClaimOverAsserted {
                    id: row.row_id.clone(),
                });
            }

            // AC2: a cached or offline state never keeps a committed claim.
            if !row.commit_honesty_holds() {
                violations.push(
                    ProviderComponentAccessibilityViolation::CachedOrOfflineShownAsCommitted {
                        id: row.row_id.clone(),
                    },
                );
            }

            // Assistive-tech / CLI reach the same canonical truth.
            if !row.reaches_canonical_truth_via_at() {
                violations.push(
                    ProviderComponentAccessibilityViolation::AssistiveTechStranded {
                        id: row.row_id.clone(),
                    },
                );
            }

            // Export preserves meaning without a screenshot.
            if !row.export_preserves_meaning() {
                violations.push(
                    ProviderComponentAccessibilityViolation::ExportRequiresScreenshot {
                        id: row.row_id.clone(),
                    },
                );
            }

            // AC / no-loss: limited-scope, stale-session, policy-blocked, and local-only states
            // preserve provider lineage.
            if !row.preserves_lineage_continuity() {
                violations.push(ProviderComponentAccessibilityViolation::LineageDropped {
                    id: row.row_id.clone(),
                });
            }

            // Narrowing disclosed on every narrowed rendering surface.
            if !row.narrowing_disclosed() {
                violations.push(
                    ProviderComponentAccessibilityViolation::NarrowingDropsContextSilently {
                        id: row.row_id.clone(),
                    },
                );
            }

            // Consumer parity: at least two consumer surfaces ingest the row.
            if row.consumer_surfaces.len() < 2 {
                violations.push(
                    ProviderComponentAccessibilityViolation::MissingConsumerParity {
                        id: row.row_id.clone(),
                    },
                );
            }

            // No red rows may ship.
            if row.status() == ProviderComponentAccessibilityStatus::Stranded {
                violations.push(ProviderComponentAccessibilityViolation::StrandedRow {
                    id: row.row_id.clone(),
                });
            }
        }

        // Coverage: every frozen family is certified at least once.
        for family in M5ProviderAccountOfflineComponentFamily::ALL {
            if !seen_families.contains(&family) {
                violations.push(
                    ProviderComponentAccessibilityViolation::MissingFamilyCoverage { family },
                );
            }
        }

        // Coverage: every weakening dimension is exercised somewhere.
        let exercised = self.exercised_dimensions();
        for dimension in M5ProviderComponentClaimDimension::ALL {
            if !exercised.contains(&dimension) {
                violations.push(
                    ProviderComponentAccessibilityViolation::MissingDimensionCoverage { dimension },
                );
            }
        }

        // Coverage: every condition state (the committed baseline plus each spec narrowing axis)
        // is exercised somewhere, so the full narrowing spectrum is proven end-to-end.
        let states = self.exercised_condition_states();
        for state in M5ProviderComponentConditionState::ALL {
            if !states.contains(&state) {
                violations.push(
                    ProviderComponentAccessibilityViolation::MissingConditionStateCoverage {
                        state,
                    },
                );
            }
        }

        // Coverage: every provider claim tier appears as an effective claim, so the full
        // narrowing spectrum (provider-committed → … → local-only-packet) is proven end-to-end.
        let effective = self.represented_effective_claims();
        for claim in M5ProviderComponentClaim::ALL {
            if !effective.contains(&claim) {
                violations.push(
                    ProviderComponentAccessibilityViolation::MissingClaimTierCoverage { claim },
                );
            }
        }

        // Commit honesty must be proven with at least one cached-or-offline row in the packet,
        // so the "cached / offline never shown as committed" guarantee is exercised end-to-end.
        if !has_cached_or_offline_row {
            violations.push(ProviderComponentAccessibilityViolation::CommitHonestyUnproven);
        }

        // Cross-surface: the same narrowed state must reach the account-settings, mapping-picker,
        // sync-status, offline-queue, privacy-review, status-bar, product UI, CLI, and support /
        // release exports — so every consumer surface is exercised at least once across the
        // packet.
        let consumers = self.represented_consumer_surfaces();
        for surface in M5ProviderConsumerSurface::ALL {
            if !consumers.contains(&surface) {
                violations.push(
                    ProviderComponentAccessibilityViolation::MissingConsumerSurfaceCoverage {
                        surface,
                    },
                );
            }
        }

        if self.summary != self.computed_summary() {
            violations.push(ProviderComponentAccessibilityViolation::SummaryMismatch);
        }

        if json_contains_forbidden_material(
            &serde_json::to_value(self).expect(
                "provider-account / offline-capture accessibility fallback packet serializes",
            ),
        ) {
            violations.push(ProviderComponentAccessibilityViolation::RawProviderMaterialInExport);
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
            .expect("provider-account / offline-capture accessibility fallback packet serializes")
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
                full = row.full_provider_claim.as_str(),
                effective = row.effective_claim().as_str(),
                status = row.status().as_str(),
            ));
        }
        out
    }

    /// Deterministic Markdown report for support, docs, or release handoff.
    pub fn render_markdown_summary(&self) -> String {
        let mut out = String::new();
        out.push_str(
            "# M5 Provider-Account / Offline-Capture Component Accessibility & Auto-Narrowing\n\n",
        );
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- As of: `{}`\n", self.as_of));
        out.push_str(&format!(
            "- Families: {} certified across {} / {} frozen families\n",
            self.summary.family_count,
            self.represented_families().len(),
            M5ProviderAccountOfflineComponentFamily::ALL.len(),
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
                    row.full_provider_claim.as_str(),
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

/// Reads and validates the checked-in provider-account / offline-capture component accessibility
/// fallback export.
pub fn current_m5_provider_component_a11y_fallback_export(
) -> Result<ProviderComponentAccessibilityPacket, ProviderComponentAccessibilityArtifactError> {
    let packet: ProviderComponentAccessibilityPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/release/m5-provider-account-offline-capture-component-accessibility-fallback/support_export.json"
    )))
    .map_err(ProviderComponentAccessibilityArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(ProviderComponentAccessibilityArtifactError::Validation(
            violations,
        ))
    }
}

/// Errors emitted when reading the checked-in provider-account / offline-capture component
/// accessibility fallback export.
#[derive(Debug)]
pub enum ProviderComponentAccessibilityArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<ProviderComponentAccessibilityViolation>),
}

impl fmt::Display for ProviderComponentAccessibilityArtifactError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(
                    f,
                    "provider-account / offline-capture accessibility fallback export parse failed: {error}"
                )
            }
            Self::Validation(violations) => {
                write!(
                    f,
                    "provider-account / offline-capture accessibility fallback export failed validation: {} violation(s)",
                    violations.len()
                )
            }
        }
    }
}

impl Error for ProviderComponentAccessibilityArtifactError {}

/// Validation failure for M05-922 provider-account / offline-capture component accessibility
/// fallback packets.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ProviderComponentAccessibilityViolation {
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
        dimension: M5ProviderComponentClaimDimension,
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
    CachedOrOfflineShownAsCommitted {
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
        family: M5ProviderAccountOfflineComponentFamily,
    },
    MissingDimensionCoverage {
        dimension: M5ProviderComponentClaimDimension,
    },
    MissingConditionStateCoverage {
        state: M5ProviderComponentConditionState,
    },
    MissingClaimTierCoverage {
        claim: M5ProviderComponentClaim,
    },
    CommitHonestyUnproven,
    MissingConsumerSurfaceCoverage {
        surface: M5ProviderConsumerSurface,
    },
    SummaryMismatch,
    RawProviderMaterialInExport,
}

impl fmt::Display for ProviderComponentAccessibilityViolation {
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
                write!(f, "row {id} drops a mandatory provider label")
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
                    "row {id} over-asserts a committed / reviewable surface for a weakened one, or narrows spuriously"
                )
            }
            Self::CachedOrOfflineShownAsCommitted { id } => {
                write!(
                    f,
                    "row {id} shows a cached or offline state as provider-committed"
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
                    "row {id} does not preserve provider lineage across narrowing"
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
                    "provider claim tier {} does not appear as an effective claim",
                    claim.as_str()
                )
            }
            Self::CommitHonestyUnproven => {
                write!(
                    f,
                    "no cached-or-offline row is present to prove the commit-honesty guarantee"
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
            Self::RawProviderMaterialInExport => {
                write!(f, "export contains raw provider material")
            }
        }
    }
}

impl Error for ProviderComponentAccessibilityViolation {}

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
            | "limited scope"
            | "stale session"
            | "policy blocked"
            | "local only"
            | "offline"
            | "cached"
            | "unmapped"
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

/// Builds the canonical, checked-in provider-account / offline-capture component accessibility
/// fallback packet. This is the one source of truth shared by the tests and the on-disk support
/// export so both stay byte-aligned.
pub fn seeded_m5_provider_component_a11y_fallback_packet() -> ProviderComponentAccessibilityPacket {
    ProviderComponentAccessibilityPacket::new(ProviderComponentAccessibilityPacketInput {
        packet_id:
            "m5-provider-account-offline-capture-component-accessibility-fallback:stable:0001"
                .to_owned(),
        as_of: "2026-07-07T00:00:00Z".to_owned(),
        matrix_ref: PROVIDER_COMPONENT_A11Y_FALLBACK_COMPONENT_MATRIX_REF.to_owned(),
        rows: seeded_rows(),
    })
}

fn ev(id: &str) -> Vec<String> {
    vec![format!(
        "evidence:provider-account-offline-capture-a11y:{id}"
    )]
}

fn all_required_labels() -> Vec<M5ProviderRequiredLabel> {
    M5ProviderRequiredLabel::ALL.to_vec()
}

fn copy_export(fields: &[&str]) -> ProviderComponentCopyExportParity {
    ProviderComponentCopyExportParity {
        formats: vec!["text".to_owned(), "json".to_owned(), "markdown".to_owned()],
        export_fields: fields.iter().map(|f| (*f).to_owned()).collect(),
        screenshot_only_prohibited: true,
    }
}

fn condition(
    dimension: M5ProviderComponentClaimDimension,
    state: M5ProviderComponentConditionState,
) -> ProviderComponentClaimConditionEntry {
    ProviderComponentClaimConditionEntry { dimension, state }
}

/// The two consumer surfaces every row ships to at minimum — support / release export and CLI
/// inspect — so the narrowed state always reaches headless field triage.
fn base_consumers(extra: &[M5ProviderConsumerSurface]) -> Vec<M5ProviderConsumerSurface> {
    let mut out = vec![
        M5ProviderConsumerSurface::SupportExport,
        M5ProviderConsumerSurface::CliInspect,
    ];
    out.extend_from_slice(extra);
    out
}

/// Disclosures for the CLI-headless and support-export surfaces. A green (full parity) row keeps
/// full label and summary parity on the narrower surfaces; a narrowed row discloses the reduced
/// interactions it drops there.
fn surface_disclosures(
    labels: &[&str],
    state: ProviderComponentNarrowingDisclosureState,
) -> Vec<ProviderComponentRenderingNarrowingDisclosure> {
    let preserved: Vec<String> = labels.iter().map(|l| (*l).to_owned()).collect();
    vec![
        ProviderComponentRenderingNarrowingDisclosure {
            rendering_surface: M5ProviderComponentRenderingSurface::CliHeadless,
            state,
            preserved_labels: preserved.clone(),
            reduced_interactions: vec!["pointer_interaction".to_owned()],
        },
        ProviderComponentRenderingNarrowingDisclosure {
            rendering_surface: M5ProviderComponentRenderingSurface::SupportExport,
            state,
            preserved_labels: preserved,
            reduced_interactions: vec!["live_provider_write".to_owned()],
        },
    ]
}

/// Disclosures for a full-parity (green) row: the narrower surfaces preserve full label and
/// summary parity.
fn parity_surfaces(labels: &[&str]) -> Vec<ProviderComponentRenderingNarrowingDisclosure> {
    surface_disclosures(
        labels,
        ProviderComponentNarrowingDisclosureState::ParityPreserved,
    )
}

/// Disclosures for a narrowed (yellow) row: the narrower surfaces disclose their reduced
/// interactions while preserving labels.
fn narrowed_surfaces(labels: &[&str]) -> Vec<ProviderComponentRenderingNarrowingDisclosure> {
    surface_disclosures(
        labels,
        ProviderComponentNarrowingDisclosureState::DisclosedNarrowed,
    )
}

fn rendering_surfaces() -> Vec<M5ProviderComponentRenderingSurface> {
    vec![
        M5ProviderComponentRenderingSurface::DesktopFull,
        M5ProviderComponentRenderingSurface::CliHeadless,
        M5ProviderComponentRenderingSurface::SupportExport,
    ]
}

fn seeded_rows() -> Vec<ProviderComponentAccessibilityRow> {
    vec![
        // Provider-account row (scope limited) — the account is reachable and session-fresh but
        // the granted write scope is limited, so the row auto-narrows to a limited-scope
        // projection rather than presenting a fully committed, ready-to-write account, while
        // keeping its identity, tenant scope, and connection state visible (yellow).
        ProviderComponentAccessibilityRow {
            record_kind: PROVIDER_COMPONENT_A11Y_FALLBACK_ROW_RECORD_KIND.to_owned(),
            schema_version: PROVIDER_COMPONENT_A11Y_FALLBACK_SCHEMA_VERSION,
            row_id: "a11y:provider-account-row-scope-limited".to_owned(),
            component_family: M5ProviderAccountOfflineComponentFamily::ProviderAccountRow,
            source_family_schema_ref: PROVIDER_COMPONENT_A11Y_FALLBACK_COMPONENT_MATRIX_REF
                .to_owned(),
            provider_context_ref: "provider:account-row:0001".to_owned(),
            fallback_modalities: vec![
                M5ProviderComponentFallbackModality::List,
                M5ProviderComponentFallbackModality::Textual,
                M5ProviderComponentFallbackModality::Cli,
            ],
            reaches_canonical_truth: true,
            keyboard_reach: ProviderComponentNonVisualReachState::ReachableAndLabeled,
            screen_reader_reach: ProviderComponentNonVisualReachState::ReachableAndLabeled,
            cli_reach: ProviderComponentNonVisualReachState::ReachableAndLabeled,
            export_summary: ProviderComponentExportSummaryState::ReconstructableWithoutScreenshot,
            export_summary_ref: "summary:provider-account-row-scope-limited:a11y".to_owned(),
            copy_export: copy_export(&[
                "provider_identity",
                "tenant_scope",
                "connection_state",
                "keyboard_route",
            ]),
            full_provider_claim: M5ProviderComponentClaim::ProviderCommitted,
            claim_conditions: vec![condition(
                M5ProviderComponentClaimDimension::ConnectionAndScope,
                M5ProviderComponentConditionState::ScopeLimited,
            )],
            claim_narrow: Some(ProviderComponentClaimAutoNarrow {
                narrowed_to: M5ProviderComponentClaim::LimitedScopeProjection,
                binding_dimension: M5ProviderComponentClaimDimension::ConnectionAndScope,
                trigger: M5ProviderDowngradeTrigger::WriteScopeUnstated,
                narrowed_label:
                    "Granted write scope is narrower than a full commit — shown as a limited-scope projection with its provider identity, tenant scope, and connection state still preserved, never as a fully committed account"
                        .to_owned(),
                preserves_canonical_identity: true,
                preserves_lineage_continuity: true,
            }),
            lineage_preserved: true,
            rendering_surfaces: rendering_surfaces(),
            narrowing_disclosures: narrowed_surfaces(&[
                "provider_identity",
                "tenant_scope",
                "connection_state",
            ]),
            required_labels: all_required_labels(),
            consumer_surfaces: base_consumers(&[
                M5ProviderConsumerSurface::AccountSettingsUi,
                M5ProviderConsumerSurface::StatusBarUi,
            ]),
            source_refs: vec![
                "UI/UX Spec §18.36 connected provider accounts".to_owned(),
                PROVIDER_COMPONENT_A11Y_FALLBACK_DOC_REF.to_owned(),
            ],
            observed_at: "2026-07-07T00:00:00Z".to_owned(),
            evidence_refs: ev("provider-account-row-scope-limited"),
        },
        // Provider-account row (session stale) — the session has gone stale so only a cached read
        // is available; the row auto-narrows to a stale-session projection and must never be
        // shown as a committed, live account (cached-never-committed), keeping identity and
        // tenant scope visible (yellow).
        ProviderComponentAccessibilityRow {
            record_kind: PROVIDER_COMPONENT_A11Y_FALLBACK_ROW_RECORD_KIND.to_owned(),
            schema_version: PROVIDER_COMPONENT_A11Y_FALLBACK_SCHEMA_VERSION,
            row_id: "a11y:provider-account-row-session-stale".to_owned(),
            component_family: M5ProviderAccountOfflineComponentFamily::ProviderAccountRow,
            source_family_schema_ref: PROVIDER_COMPONENT_A11Y_FALLBACK_COMPONENT_MATRIX_REF
                .to_owned(),
            provider_context_ref: "provider:account-row:0002".to_owned(),
            fallback_modalities: vec![
                M5ProviderComponentFallbackModality::List,
                M5ProviderComponentFallbackModality::Textual,
                M5ProviderComponentFallbackModality::Cli,
            ],
            reaches_canonical_truth: true,
            keyboard_reach: ProviderComponentNonVisualReachState::ReachableAndLabeled,
            screen_reader_reach: ProviderComponentNonVisualReachState::ReachableAndLabeled,
            cli_reach: ProviderComponentNonVisualReachState::ReachableAndLabeled,
            export_summary: ProviderComponentExportSummaryState::ReconstructableWithoutScreenshot,
            export_summary_ref: "summary:provider-account-row-session-stale:a11y".to_owned(),
            copy_export: copy_export(&[
                "provider_identity",
                "tenant_scope",
                "connection_state",
                "cached_read_notice",
            ]),
            full_provider_claim: M5ProviderComponentClaim::ProviderCommitted,
            claim_conditions: vec![condition(
                M5ProviderComponentClaimDimension::ConnectionAndScope,
                M5ProviderComponentConditionState::SessionStale,
            )],
            claim_narrow: Some(ProviderComponentClaimAutoNarrow {
                narrowed_to: M5ProviderComponentClaim::StaleSessionProjection,
                binding_dimension: M5ProviderComponentClaimDimension::ConnectionAndScope,
                trigger: M5ProviderDowngradeTrigger::ConnectionStateUnstated,
                narrowed_label:
                    "Session has expired and only a cached read remains — shown as a stale-session projection that must re-authenticate before it is trusted as live, never as a committed account"
                        .to_owned(),
                preserves_canonical_identity: true,
                preserves_lineage_continuity: true,
            }),
            lineage_preserved: true,
            rendering_surfaces: rendering_surfaces(),
            narrowing_disclosures: narrowed_surfaces(&[
                "provider_identity",
                "tenant_scope",
                "connection_state",
            ]),
            required_labels: all_required_labels(),
            consumer_surfaces: base_consumers(&[
                M5ProviderConsumerSurface::AccountSettingsUi,
                M5ProviderConsumerSurface::ProductUi,
            ]),
            source_refs: vec![
                "UI/UX Spec Appendix CW provider-account drills".to_owned(),
                PROVIDER_COMPONENT_A11Y_FALLBACK_DOC_REF.to_owned(),
            ],
            observed_at: "2026-07-07T00:00:00Z".to_owned(),
            evidence_refs: ev("provider-account-row-session-stale"),
        },
        // Project/board mapping row (policy-blocked) — the default-destination mapping is
        // policy-blocked, so the row auto-narrows to a policy-blocked mapping rather than
        // presenting a resolved, committed destination, while keeping the mapping origin and
        // target visible (yellow).
        ProviderComponentAccessibilityRow {
            record_kind: PROVIDER_COMPONENT_A11Y_FALLBACK_ROW_RECORD_KIND.to_owned(),
            schema_version: PROVIDER_COMPONENT_A11Y_FALLBACK_SCHEMA_VERSION,
            row_id: "a11y:project-or-board-mapping-row-policy-blocked".to_owned(),
            component_family: M5ProviderAccountOfflineComponentFamily::ProjectOrBoardMappingRow,
            source_family_schema_ref: PROVIDER_COMPONENT_A11Y_FALLBACK_COMPONENT_MATRIX_REF
                .to_owned(),
            provider_context_ref: "provider:project-or-board-mapping-row:0003".to_owned(),
            fallback_modalities: vec![
                M5ProviderComponentFallbackModality::List,
                M5ProviderComponentFallbackModality::Textual,
                M5ProviderComponentFallbackModality::Cli,
            ],
            reaches_canonical_truth: true,
            keyboard_reach: ProviderComponentNonVisualReachState::ReachableAndLabeled,
            screen_reader_reach: ProviderComponentNonVisualReachState::ReachableAndLabeled,
            cli_reach: ProviderComponentNonVisualReachState::ReachableAndLabeled,
            export_summary: ProviderComponentExportSummaryState::ReconstructableWithoutScreenshot,
            export_summary_ref: "summary:project-or-board-mapping-row-policy-blocked:a11y".to_owned(),
            copy_export: copy_export(&[
                "mapping_origin",
                "mapping_target",
                "policy_source",
                "keyboard_route",
            ]),
            full_provider_claim: M5ProviderComponentClaim::ProviderCommitted,
            claim_conditions: vec![condition(
                M5ProviderComponentClaimDimension::MappingOrigin,
                M5ProviderComponentConditionState::MappingPolicyBlocked,
            )],
            claim_narrow: Some(ProviderComponentClaimAutoNarrow {
                narrowed_to: M5ProviderComponentClaim::PolicyBlockedMapping,
                binding_dimension: M5ProviderComponentClaimDimension::MappingOrigin,
                trigger: M5ProviderDowngradeTrigger::MappingOriginUnstated,
                narrowed_label:
                    "Default-destination mapping is held by policy and cannot resolve a committed target — shown as a policy-blocked mapping that names the origin and policy source, never a resolved destination"
                        .to_owned(),
                preserves_canonical_identity: true,
                preserves_lineage_continuity: true,
            }),
            lineage_preserved: true,
            rendering_surfaces: rendering_surfaces(),
            narrowing_disclosures: narrowed_surfaces(&[
                "mapping_origin",
                "mapping_target",
                "policy_source",
            ]),
            required_labels: all_required_labels(),
            consumer_surfaces: base_consumers(&[
                M5ProviderConsumerSurface::MappingPickerUi,
                M5ProviderConsumerSurface::ProductUi,
            ]),
            source_refs: vec![
                "UI/UX Spec §23.44 project/board mappings".to_owned(),
                PROVIDER_COMPONENT_A11Y_FALLBACK_DOC_REF.to_owned(),
            ],
            observed_at: "2026-07-07T00:00:00Z".to_owned(),
            evidence_refs: ev("project-or-board-mapping-row-policy-blocked"),
        },
        // Sync-behavior row — the account is reachable and in scope with a stated sync mode,
        // effective write scope, and a visible queued-draft state, so the row is fully committed
        // and reachable on every surface (green).
        ProviderComponentAccessibilityRow {
            record_kind: PROVIDER_COMPONENT_A11Y_FALLBACK_ROW_RECORD_KIND.to_owned(),
            schema_version: PROVIDER_COMPONENT_A11Y_FALLBACK_SCHEMA_VERSION,
            row_id: "a11y:sync-behavior-row".to_owned(),
            component_family: M5ProviderAccountOfflineComponentFamily::SyncBehaviorRow,
            source_family_schema_ref: PROVIDER_COMPONENT_A11Y_FALLBACK_COMPONENT_MATRIX_REF
                .to_owned(),
            provider_context_ref: "provider:sync-behavior-row:0004".to_owned(),
            fallback_modalities: vec![
                M5ProviderComponentFallbackModality::List,
                M5ProviderComponentFallbackModality::Textual,
                M5ProviderComponentFallbackModality::Cli,
            ],
            reaches_canonical_truth: true,
            keyboard_reach: ProviderComponentNonVisualReachState::ReachableAndLabeled,
            screen_reader_reach: ProviderComponentNonVisualReachState::ReachableAndLabeled,
            cli_reach: ProviderComponentNonVisualReachState::ReachableAndLabeled,
            export_summary: ProviderComponentExportSummaryState::ReconstructableWithoutScreenshot,
            export_summary_ref: "summary:sync-behavior-row:a11y".to_owned(),
            copy_export: copy_export(&[
                "sync_mode",
                "effective_write_scope",
                "queued_draft_state",
                "keyboard_route",
            ]),
            full_provider_claim: M5ProviderComponentClaim::ProviderCommitted,
            claim_conditions: vec![condition(
                M5ProviderComponentClaimDimension::SyncBehavior,
                M5ProviderComponentConditionState::InScopeCommitted,
            )],
            claim_narrow: None,
            lineage_preserved: true,
            rendering_surfaces: rendering_surfaces(),
            narrowing_disclosures: parity_surfaces(&[
                "sync_mode",
                "effective_write_scope",
                "queued_draft_state",
            ]),
            required_labels: all_required_labels(),
            consumer_surfaces: base_consumers(&[
                M5ProviderConsumerSurface::SyncStatusUi,
                M5ProviderConsumerSurface::StatusBarUi,
            ]),
            source_refs: vec![
                "UI/UX Spec §23.44 sync / offline capture controls".to_owned(),
                PROVIDER_COMPONENT_A11Y_FALLBACK_DOC_REF.to_owned(),
            ],
            observed_at: "2026-07-07T00:00:00Z".to_owned(),
            evidence_refs: ev("sync-behavior-row"),
        },
        // Offline-capture row — hierarchy-heavy (nested packet / queued-draft / destination
        // lineage); the packet remains local-only, so the row auto-narrows to a local-only packet
        // and binds its nested lineage to a flat list / textual path (yellow).
        ProviderComponentAccessibilityRow {
            record_kind: PROVIDER_COMPONENT_A11Y_FALLBACK_ROW_RECORD_KIND.to_owned(),
            schema_version: PROVIDER_COMPONENT_A11Y_FALLBACK_SCHEMA_VERSION,
            row_id: "a11y:offline-capture-row".to_owned(),
            component_family: M5ProviderAccountOfflineComponentFamily::OfflineCaptureRow,
            source_family_schema_ref: PROVIDER_COMPONENT_A11Y_FALLBACK_COMPONENT_MATRIX_REF
                .to_owned(),
            provider_context_ref: "provider:offline-capture-row:0005".to_owned(),
            fallback_modalities: vec![
                M5ProviderComponentFallbackModality::Structured,
                M5ProviderComponentFallbackModality::List,
                M5ProviderComponentFallbackModality::Textual,
                M5ProviderComponentFallbackModality::Cli,
            ],
            reaches_canonical_truth: true,
            keyboard_reach: ProviderComponentNonVisualReachState::ReachableAndLabeled,
            screen_reader_reach: ProviderComponentNonVisualReachState::DisclosedReducedButReachable,
            cli_reach: ProviderComponentNonVisualReachState::ReachableAndLabeled,
            export_summary: ProviderComponentExportSummaryState::ReconstructableWithoutScreenshot,
            export_summary_ref: "summary:offline-capture-row:a11y".to_owned(),
            copy_export: copy_export(&[
                "packet_destination",
                "queued_draft_state",
                "capture_state",
                "publish_later_route",
            ]),
            full_provider_claim: M5ProviderComponentClaim::ProviderCommitted,
            claim_conditions: vec![condition(
                M5ProviderComponentClaimDimension::OfflineCapture,
                M5ProviderComponentConditionState::PacketLocalOnly,
            )],
            claim_narrow: Some(ProviderComponentClaimAutoNarrow {
                narrowed_to: M5ProviderComponentClaim::LocalOnlyPacket,
                binding_dimension: M5ProviderComponentClaimDimension::OfflineCapture,
                trigger: M5ProviderDowngradeTrigger::OfflineCaptureStateUnstated,
                narrowed_label:
                    "Captured packet is held on this machine and nothing has been published — shown as a local-only packet with its queued-draft count and publish-later route preserved, never as a committed provider write"
                        .to_owned(),
                preserves_canonical_identity: true,
                preserves_lineage_continuity: true,
            }),
            lineage_preserved: true,
            rendering_surfaces: rendering_surfaces(),
            narrowing_disclosures: narrowed_surfaces(&[
                "packet_destination",
                "queued_draft_state",
                "capture_state",
            ]),
            required_labels: all_required_labels(),
            consumer_surfaces: base_consumers(&[
                M5ProviderConsumerSurface::OfflineQueueUi,
                M5ProviderConsumerSurface::ProductUi,
            ]),
            source_refs: vec![
                "UI/UX Spec §23.44 offline capture controls".to_owned(),
                PROVIDER_COMPONENT_A11Y_FALLBACK_DOC_REF.to_owned(),
            ],
            observed_at: "2026-07-07T00:00:00Z".to_owned(),
            evidence_refs: ev("offline-capture-row"),
        },
        // Privacy/redaction row — the redaction class and metadata-safe export boundary are
        // stated and the row is a self-sufficient, reviewable read-only projection (not itself a
        // committed write), reachable on every surface (green).
        ProviderComponentAccessibilityRow {
            record_kind: PROVIDER_COMPONENT_A11Y_FALLBACK_ROW_RECORD_KIND.to_owned(),
            schema_version: PROVIDER_COMPONENT_A11Y_FALLBACK_SCHEMA_VERSION,
            row_id: "a11y:privacy-redaction-row".to_owned(),
            component_family: M5ProviderAccountOfflineComponentFamily::PrivacyRedactionRow,
            source_family_schema_ref: PROVIDER_COMPONENT_A11Y_FALLBACK_COMPONENT_MATRIX_REF
                .to_owned(),
            provider_context_ref: "provider:privacy-redaction-row:0006".to_owned(),
            fallback_modalities: vec![
                M5ProviderComponentFallbackModality::List,
                M5ProviderComponentFallbackModality::Textual,
                M5ProviderComponentFallbackModality::Cli,
            ],
            reaches_canonical_truth: true,
            keyboard_reach: ProviderComponentNonVisualReachState::ReachableAndLabeled,
            screen_reader_reach: ProviderComponentNonVisualReachState::ReachableAndLabeled,
            cli_reach: ProviderComponentNonVisualReachState::ReachableAndLabeled,
            export_summary: ProviderComponentExportSummaryState::ReconstructableWithoutScreenshot,
            export_summary_ref: "summary:privacy-redaction-row:a11y".to_owned(),
            copy_export: copy_export(&[
                "redaction_class",
                "export_boundary",
                "metadata_safe_notice",
                "keyboard_route",
            ]),
            full_provider_claim: M5ProviderComponentClaim::ReviewableProjection,
            claim_conditions: vec![condition(
                M5ProviderComponentClaimDimension::RedactionBoundary,
                M5ProviderComponentConditionState::InScopeCommitted,
            )],
            claim_narrow: None,
            lineage_preserved: true,
            rendering_surfaces: rendering_surfaces(),
            narrowing_disclosures: parity_surfaces(&[
                "redaction_class",
                "export_boundary",
                "metadata_safe_notice",
            ]),
            required_labels: all_required_labels(),
            consumer_surfaces: base_consumers(&[
                M5ProviderConsumerSurface::PrivacyReviewUi,
                M5ProviderConsumerSurface::AccountSettingsUi,
            ]),
            source_refs: vec![
                "UX Design System v1.37 work-item / offline-capture guidance".to_owned(),
                PROVIDER_COMPONENT_A11Y_FALLBACK_DOC_REF.to_owned(),
            ],
            observed_at: "2026-07-07T00:00:00Z".to_owned(),
            evidence_refs: ev("privacy-redaction-row"),
        },
    ]
}
