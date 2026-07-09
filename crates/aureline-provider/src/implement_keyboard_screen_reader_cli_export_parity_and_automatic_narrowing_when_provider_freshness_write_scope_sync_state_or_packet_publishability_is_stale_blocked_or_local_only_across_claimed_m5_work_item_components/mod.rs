//! Keyboard / screen-reader / CLI / export parity and honest automatic narrowing for the
//! M5 work-item components (work-item row, provider-chip group, relation strip, sync-pending
//! pill, work-item detail header, status-transition sheet, related-evidence card, and
//! offline-handoff-packet card).
//!
//! This module is the M05-986 accessibility-and-auto-narrowing capstone over the frozen M5
//! work-item component matrix
//! ([`crate::freeze_the_m5_work_item_component_matrix`]). Where the freeze matrix defines the
//! reusable work-item component primitives, and the 981-985 implementation / consumer lanes
//! resolve their per-surface truth, this lane certifies — per component family — that
//! work-item claims stay **keyboard-complete, assistive-tech-reachable, CLI/export-safe, and
//! self-narrowing** rather than presenting a stale provider freshness, a read-only or
//! policy-blocked write scope, a local-only sync state, or an unpublishable offline-handoff
//! packet as a still fully committed, ready-to-write provider surface:
//!
//! - **Keyboard / screen-reader / CLI reach.** Every family exposes a keyboard-complete,
//!   screen-reader-reachable, and CLI/headless-reachable path into the same canonical work-item
//!   identity, provider authority, local-versus-provider state, linked engineering context,
//!   side-effect preview, and publish-later continuity the rich component shows — never a
//!   hover-only chip that strands assistive-tech or headless users. Hierarchy-heavy families
//!   (the offline-handoff-packet card's nested packet / evidence / queued-draft lineage)
//!   additionally bind their tree to a flat list / textual path.
//! - **Export parity.** The support / release / evaluation export reconstructs each component's
//!   meaning from typed tokens and opaque refs without a screenshot, preserving the same
//!   canonical IDs, provider authorities, local-versus-provider labels, linked-context refs,
//!   side-effect previews, queued-draft counts, redaction classes, and narrowing reasons shown
//!   in-product so work-item truth can be reconstructed without screenshots or private team
//!   memory.
//! - **Honest auto-narrowing.** When provider freshness is stale, write scope is read-only or
//!   policy-blocked, sync state is local-only, or an offline-handoff packet cannot publish
//!   safely, the component's provider claim auto-narrows from `ProviderCommitted` /
//!   `ReviewableProjection` to a stale-freshness / read-only / local-only / unpublishable-packet
//!   projection, discloses the narrowing with a precise trigger and binding dimension, and
//!   preserves the canonical identity / authority / linked-context / queued-draft lineage — the
//!   underlying work-item lineage is never dropped opaquely. A component with every dimension
//!   intact must NOT carry a spurious narrowing, and a cached or offline state can never keep a
//!   committed claim.
//! - **Cross-surface disclosure.** The same narrowed state surfaces in the inbox, detail,
//!   relation-panel, sync-status, transition-sheet, evidence-panel, status-bar, general product
//!   UI, headless CLI, and support / release exports so product, docs, and release publication
//!   stay aligned on provider-boundary downgrade behavior rather than drifting in copy — a
//!   committed-looking surface can never outrun the freshness / write-scope / sync / packet proof
//!   it is being viewed away from.
//!
//! Each [`WorkItemComponentAccessibilityRow`] keys on one
//! [`crate::freeze_the_m5_work_item_component_matrix::M5WorkItemComponentFamily`] and reuses that
//! frozen family vocabulary plus the frozen [`M5WorkItemRequiredLabel`] and
//! [`M5WorkItemDowngradeTrigger`] and the shared [`M5WorkItemConsumerSurface`] consumer surfaces
//! rather than minting parallel synonyms, so the certified labels stay byte-identical to the
//! matrix and the sibling primitive packets.
//!
//! The packet is metadata-only: raw credentials, tokens, request bodies, and endpoint secrets
//! never cross this boundary; the packet carries only typed class tokens, opaque work-item /
//! relation / packet refs, booleans, and redacted labels so support, release, and diagnostics
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
use crate::freeze_the_m5_work_item_component_matrix::{
    M5WorkItemComponentFamily, M5WorkItemConsumerSurface, M5WorkItemDowngradeTrigger,
    M5WorkItemRequiredLabel,
};

/// Schema version stamped on the M05-986 work-item component accessibility parity packet.
pub const WORK_ITEM_COMPONENT_A11Y_SCHEMA_VERSION: u32 = 1;

/// Stable record-kind tag carried by [`WorkItemComponentAccessibilityPacket`].
pub const WORK_ITEM_COMPONENT_A11Y_RECORD_KIND: &str =
    "m5_work_item_component_accessibility_parity_packet";

/// Stable record-kind tag carried by each [`WorkItemComponentAccessibilityRow`].
pub const WORK_ITEM_COMPONENT_A11Y_ROW_RECORD_KIND: &str =
    "m5_work_item_component_accessibility_parity_row";

/// Repo-relative path of the boundary schema.
pub const WORK_ITEM_COMPONENT_A11Y_SCHEMA_REF: &str =
    "schemas/ui/m5-work-item-component-accessibility-parity.schema.json";

/// Repo-relative path of the contract doc.
pub const WORK_ITEM_COMPONENT_A11Y_DOC_REF: &str =
    "docs/team-workflows/m5_work_item_component_accessibility_parity.md";

/// Repo-relative path of the frozen work-item component matrix this lane certifies.
pub const WORK_ITEM_COMPONENT_A11Y_COMPONENT_MATRIX_REF: &str =
    "schemas/ui/m5-work-item-component-matrix.schema.json";

/// Repo-relative path of the protected fixture directory.
pub const WORK_ITEM_COMPONENT_A11Y_FIXTURE_DIR: &str =
    "fixtures/ui/m5-work-item-component-accessibility-parity";

/// Repo-relative path of the checked support-export artifact (the `include_str!` canonical).
pub const WORK_ITEM_COMPONENT_A11Y_ARTIFACT_REF: &str =
    "artifacts/release/m5-work-item-component-accessibility-proof/support_export.json";

/// Repo-relative path of the checked machine-readable matrix CSV.
pub const WORK_ITEM_COMPONENT_A11Y_CSV_REF: &str =
    "artifacts/release/m5-work-item-component-accessibility-proof/matrix.csv";

/// Repo-relative path of the checked Markdown report.
pub const WORK_ITEM_COMPONENT_A11Y_REPORT_REF: &str =
    "artifacts/release/m5-work-item-component-accessibility-proof.md";

/// The reusable component families that render a non-linear hierarchy (the
/// offline-handoff-packet card's nested packet / evidence / queued-draft lineage) and therefore
/// MUST bind their tree to an equivalent flat list / textual path so the hierarchy is navigable
/// non-visually.
const fn family_is_hierarchy_heavy(family: M5WorkItemComponentFamily) -> bool {
    matches!(family, M5WorkItemComponentFamily::OfflineHandoffPacketCard)
}

/// The provider dimension whose weakening a family primarily discloses. Every row must model at
/// least this dimension so its key weakening axis is covered.
const fn family_primary_dimension(
    family: M5WorkItemComponentFamily,
) -> M5WorkItemComponentClaimDimension {
    match family {
        M5WorkItemComponentFamily::WorkItemRow => {
            M5WorkItemComponentClaimDimension::ProviderFreshness
        }
        M5WorkItemComponentFamily::ProviderChipGroup => {
            M5WorkItemComponentClaimDimension::WriteScope
        }
        M5WorkItemComponentFamily::RelationStrip => {
            M5WorkItemComponentClaimDimension::ProviderFreshness
        }
        M5WorkItemComponentFamily::SyncPendingPill => M5WorkItemComponentClaimDimension::SyncState,
        M5WorkItemComponentFamily::WorkItemDetailHeader => {
            M5WorkItemComponentClaimDimension::ProviderFreshness
        }
        M5WorkItemComponentFamily::StatusTransitionSheet => {
            M5WorkItemComponentClaimDimension::WriteScope
        }
        M5WorkItemComponentFamily::RelatedEvidenceCard => {
            M5WorkItemComponentClaimDimension::ProviderFreshness
        }
        M5WorkItemComponentFamily::OfflineHandoffPacketCard => {
            M5WorkItemComponentClaimDimension::PacketPublishability
        }
    }
}

/// A rendered fallback modality for a work-item component.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5WorkItemComponentFallbackModality {
    /// A rich, structured (nested packet / evidence / queued-draft tree) projection.
    Structured,
    /// A flat list projection.
    List,
    /// A textual / source-first projection.
    Textual,
    /// A CLI / headless line projection.
    Cli,
}

impl M5WorkItemComponentFallbackModality {
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
pub enum M5WorkItemComponentRenderingSurface {
    /// The full-capability desktop work-item surface.
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

impl M5WorkItemComponentRenderingSurface {
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
pub enum WorkItemComponentNonVisualReachState {
    /// Fully traversable and labeled with no loss.
    ReachableAndLabeled,
    /// Reachable and labeled, but with a disclosed reduction (yellow).
    DisclosedReducedButReachable,
    /// A view-only / hover-only surface that traps keyboard / assistive-tech / headless users
    /// (red).
    ViewOnlyTrap,
}

impl WorkItemComponentNonVisualReachState {
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
pub enum WorkItemComponentExportSummaryState {
    /// The component meaning reconstructs from the summary without a screenshot.
    ReconstructableWithoutScreenshot,
    /// Partial capture, but disclosed (yellow).
    DisclosedPartialCapture,
    /// The export relies on a screenshot to carry meaning (red).
    AbsentNeedsScreenshot,
}

impl WorkItemComponentExportSummaryState {
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
pub enum WorkItemComponentNarrowingDisclosureState {
    /// Full label and summary parity with the desktop surface.
    ParityPreserved,
    /// Reduced interactivity, disclosed with preserved labels (yellow).
    DisclosedNarrowed,
    /// Interactivity, state, or actions dropped without disclosure (red).
    SilentlyDropped,
}

impl WorkItemComponentNarrowingDisclosureState {
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
/// so a stale provider freshness, a read-only / policy-blocked write scope, a local-only sync
/// state, or an unpublishable offline-handoff packet can never keep an old `ProviderCommitted`
/// or `ReviewableProjection` label — a cached or offline state never masquerades as committed.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5WorkItemComponentClaim {
    /// Provider-committed: a fresh, in-scope, provider-synced work item with a routed write path
    /// — the strongest claim, a surface Aureline can read and write against right now and commit
    /// to the provider.
    ProviderCommitted,
    /// Reviewable projection: a self-sufficient, reviewable read-only projection (state a
    /// reviewer can read) that is not itself a certified committed write path.
    ReviewableProjection,
    /// Stale-freshness projection: the provider projection is stale and only a cached read is
    /// available; the surface must refresh before it can be trusted as live.
    StaleFreshnessProjection,
    /// Read-only projection: the effective write scope is read-only or policy-blocked — the
    /// surface cannot perform the committed write and stays a read-only projection.
    ReadOnlyProjection,
    /// Local-only projection: the work item is local-only and not yet synced; nothing has been
    /// published to the provider and the row stays a queued, publish-later projection.
    LocalOnlyProjection,
    /// Unpublishable-packet projection: the offline-handoff packet cannot publish safely; nothing
    /// has been handed off and the row stays a held, retry-or-export capture.
    UnpublishablePacketProjection,
}

impl M5WorkItemComponentClaim {
    /// Every claim tier, strongest first.
    pub const ALL: [Self; 6] = [
        Self::ProviderCommitted,
        Self::ReviewableProjection,
        Self::StaleFreshnessProjection,
        Self::ReadOnlyProjection,
        Self::LocalOnlyProjection,
        Self::UnpublishablePacketProjection,
    ];

    /// Capability rank; a higher rank asserts a stronger provider posture. Narrowing lowers
    /// rank.
    pub const fn capability_rank(self) -> u8 {
        match self {
            Self::ProviderCommitted => 5,
            Self::ReviewableProjection => 4,
            Self::StaleFreshnessProjection => 3,
            Self::ReadOnlyProjection => 2,
            Self::LocalOnlyProjection => 1,
            Self::UnpublishablePacketProjection => 0,
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
            Self::StaleFreshnessProjection => "stale_freshness_projection",
            Self::ReadOnlyProjection => "read_only_projection",
            Self::LocalOnlyProjection => "local_only_projection",
            Self::UnpublishablePacketProjection => "unpublishable_packet_projection",
        }
    }
}

/// The provider dimension whose state governs how far a component may claim to be a committed,
/// ready-to-write provider surface. The four dimensions map 1:1 to the four spec narrowing axes
/// — provider freshness, effective write scope, sync state, and offline-handoff packet
/// publishability — so every family carries an honest narrowing path.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5WorkItemComponentClaimDimension {
    /// Provider freshness: is the component's provider projection (identity, authority, linked
    /// context, evidence) currently fresh, or has it gone stale?
    ProviderFreshness,
    /// Write scope: does the component's effective write scope permit a committed provider write,
    /// or is it read-only / policy-blocked?
    WriteScope,
    /// Sync state: is the work item provider-synced, or does it remain local-only and unpublished?
    SyncState,
    /// Packet publishability: can the offline-handoff packet publish safely, or is it held and
    /// unpublishable?
    PacketPublishability,
}

impl M5WorkItemComponentClaimDimension {
    /// Every dimension, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::ProviderFreshness,
        Self::WriteScope,
        Self::SyncState,
        Self::PacketPublishability,
    ];

    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ProviderFreshness => "provider_freshness",
            Self::WriteScope => "write_scope",
            Self::SyncState => "sync_state",
            Self::PacketPublishability => "packet_publishability",
        }
    }
}

/// The observed condition of one provider dimension. Anything weaker than
/// [`Self::FreshCommitted`] imposes a narrowing ceiling on the component's provider claim. The
/// four spec axes the lane must auto-narrow on — a stale provider freshness, a read-only /
/// policy-blocked write scope, a local-only sync state, and an unpublishable offline-handoff
/// packet — are [`Self::FreshnessStale`], [`Self::WriteScopeBlocked`], [`Self::SyncLocalOnly`],
/// and [`Self::PacketUnpublishable`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5WorkItemComponentConditionState {
    /// Fresh, in write scope, provider-synced, and publishable — imposes no ceiling.
    FreshCommitted,
    /// The provider projection is stale — only a cached read is available; provider claim drops
    /// to a stale-freshness projection.
    FreshnessStale,
    /// The effective write scope is read-only or policy-blocked — the surface cannot commit the
    /// write; provider claim drops to a read-only projection.
    WriteScopeBlocked,
    /// The work item is local-only and not yet synced — nothing has been published; provider
    /// claim drops to a local-only projection.
    SyncLocalOnly,
    /// The offline-handoff packet cannot publish safely — nothing has been handed off; provider
    /// claim drops to an unpublishable-packet projection.
    PacketUnpublishable,
}

impl M5WorkItemComponentConditionState {
    /// Every condition state, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::FreshCommitted,
        Self::FreshnessStale,
        Self::WriteScopeBlocked,
        Self::SyncLocalOnly,
        Self::PacketUnpublishable,
    ];

    /// Returns true when the dimension is weaker than committed and therefore imposes a
    /// narrowing ceiling.
    pub const fn is_weak(self) -> bool {
        !matches!(self, Self::FreshCommitted)
    }

    /// Returns true when the condition reflects a cached or offline state that must never be
    /// shown as provider-committed.
    pub const fn is_cached_or_offline(self) -> bool {
        matches!(
            self,
            Self::FreshnessStale | Self::SyncLocalOnly | Self::PacketUnpublishable
        )
    }

    /// The strongest provider claim this condition state permits.
    pub const fn permitted_ceiling(self) -> M5WorkItemComponentClaim {
        match self {
            Self::FreshCommitted => M5WorkItemComponentClaim::ProviderCommitted,
            Self::FreshnessStale => M5WorkItemComponentClaim::StaleFreshnessProjection,
            Self::WriteScopeBlocked => M5WorkItemComponentClaim::ReadOnlyProjection,
            Self::SyncLocalOnly => M5WorkItemComponentClaim::LocalOnlyProjection,
            Self::PacketUnpublishable => M5WorkItemComponentClaim::UnpublishablePacketProjection,
        }
    }

    /// The frozen downgrade trigger this condition names when its weakness binds a narrowing.
    /// Each state maps to the on-topic frozen trigger the freeze matrix already governs, so the
    /// certified reason stays byte-identical to the matrix.
    pub const fn default_trigger(self) -> M5WorkItemDowngradeTrigger {
        match self {
            // The committed baseline never narrows; kept for exhaustiveness.
            Self::FreshCommitted => M5WorkItemDowngradeTrigger::LocalVersusProviderStateHidden,
            Self::FreshnessStale => M5WorkItemDowngradeTrigger::LocalVersusProviderStateHidden,
            Self::WriteScopeBlocked => M5WorkItemDowngradeTrigger::ProviderAuthorityUnstated,
            Self::SyncLocalOnly => M5WorkItemDowngradeTrigger::SyncPendingStateHidden,
            Self::PacketUnpublishable => M5WorkItemDowngradeTrigger::PublishLaterContinuityHidden,
        }
    }

    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::FreshCommitted => "fresh_committed",
            Self::FreshnessStale => "freshness_stale",
            Self::WriteScopeBlocked => "write_scope_blocked",
            Self::SyncLocalOnly => "sync_local_only",
            Self::PacketUnpublishable => "packet_unpublishable",
        }
    }
}

/// One provider dimension's observed condition on a component.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorkItemComponentClaimConditionEntry {
    /// Which dimension this entry describes.
    pub dimension: M5WorkItemComponentClaimDimension,
    /// The observed condition state of the dimension.
    pub state: M5WorkItemComponentConditionState,
}

/// An honest provider-claim auto-narrow block. When a provider dimension weakens, the component's
/// provider claim lowers to the permitted ceiling, names the binding dimension and frozen
/// trigger, and preserves the canonical identity / authority / linked-context / queued-draft
/// lineage rather than silently dropping it — the underlying work-item lineage is never erased
/// opaquely.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorkItemComponentClaimAutoNarrow {
    /// The provider claim the component is narrowed to.
    pub narrowed_to: M5WorkItemComponentClaim,
    /// The dimension whose weakness bound the narrowing (the one imposing the strongest ceiling
    /// constraint).
    pub binding_dimension: M5WorkItemComponentClaimDimension,
    /// The frozen downgrade trigger (reused vocabulary) the narrowing names.
    pub trigger: M5WorkItemDowngradeTrigger,
    /// A precise, non-generic label safe to render.
    pub narrowed_label: String,
    /// The canonical work-item identity, provider authority, linked engineering context, and
    /// queued-draft state are preserved rather than dropped; must hold.
    pub preserves_canonical_identity: bool,
    /// The underlying identity / authority / linked-context / queued-draft lineage is preserved
    /// (never dropped) across the narrowing; must hold so stale-freshness, read-only, local-only,
    /// and unpublishable-packet states never fail opaquely.
    pub preserves_lineage_continuity: bool,
}

impl WorkItemComponentClaimAutoNarrow {
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
pub struct WorkItemComponentCopyExportParity {
    /// The copy / export formats offered (must include text, json, markdown).
    #[serde(default)]
    pub formats: Vec<String>,
    /// The named export fields the summary carries.
    #[serde(default)]
    pub export_fields: Vec<String>,
    /// A screenshot is never the only export; must always hold.
    pub screenshot_only_prohibited: bool,
}

impl WorkItemComponentCopyExportParity {
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
pub struct WorkItemComponentRenderingNarrowingDisclosure {
    /// The rendering surface being narrowed.
    pub rendering_surface: M5WorkItemComponentRenderingSurface,
    /// How the surface discloses its reduced interactivity.
    pub state: WorkItemComponentNarrowingDisclosureState,
    /// The labels preserved across the narrowing.
    #[serde(default)]
    pub preserved_labels: Vec<String>,
    /// The interactions reduced on the narrowed surface.
    #[serde(default)]
    pub reduced_interactions: Vec<String>,
}

/// Derived qualification status for a work-item component accessibility row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum WorkItemComponentAccessibilityStatus {
    /// Full keyboard / screen-reader / CLI / export parity with no narrowing (green).
    Parity,
    /// Reduced but fully disclosed, reachable, and honestly auto-narrowed (yellow).
    NarrowedDisclosed,
    /// Strands assistive tech, needs a screenshot, over-claims commitment, or drops state
    /// silently (red).
    Stranded,
}

impl WorkItemComponentAccessibilityStatus {
    /// Stable token recorded in the summary / CSV.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Parity => "parity",
            Self::NarrowedDisclosed => "narrowed_disclosed",
            Self::Stranded => "stranded",
        }
    }
}

/// Accessibility / auto-narrowing parity row for one work-item component family.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorkItemComponentAccessibilityRow {
    /// Record kind; must equal [`WORK_ITEM_COMPONENT_A11Y_ROW_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`WORK_ITEM_COMPONENT_A11Y_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable row id.
    pub row_id: String,
    /// The frozen component family this row certifies.
    pub component_family: M5WorkItemComponentFamily,
    /// Ref to the frozen matrix family schema this row certifies.
    pub source_family_schema_ref: String,
    /// Opaque ref to the work-item / relation / packet object this component acts on; stays
    /// visible on every surface, so this is never empty.
    pub work_item_context_ref: String,
    /// Rendered modalities offered; a hierarchy-heavy family must also offer a non-visual
    /// (list / textual / cli) path.
    #[serde(default)]
    pub fallback_modalities: Vec<M5WorkItemComponentFallbackModality>,
    /// The non-visual / CLI path reaches the same canonical identity, provider authority, local
    /// state, linked context, side-effect preview, and publish-later truth as the rich surface;
    /// must hold.
    pub reaches_canonical_truth: bool,
    /// Keyboard reach into the non-visual path.
    pub keyboard_reach: WorkItemComponentNonVisualReachState,
    /// Screen-reader reach into the non-visual path.
    pub screen_reader_reach: WorkItemComponentNonVisualReachState,
    /// CLI / headless reach into the non-visual path.
    pub cli_reach: WorkItemComponentNonVisualReachState,
    /// Whether the export-safe summary preserves component meaning.
    pub export_summary: WorkItemComponentExportSummaryState,
    /// Ref to the export-safe summary object for this component.
    pub export_summary_ref: String,
    /// The copy / export parity of the accessible fallback.
    pub copy_export: WorkItemComponentCopyExportParity,
    /// The full provider claim this family asserts when every dimension is intact.
    pub full_provider_claim: M5WorkItemComponentClaim,
    /// The observed condition of each modeled provider dimension.
    #[serde(default)]
    pub claim_conditions: Vec<WorkItemComponentClaimConditionEntry>,
    /// The honest auto-narrow block, present only when some dimension weakens below the family's
    /// full claim.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub claim_narrow: Option<WorkItemComponentClaimAutoNarrow>,
    /// Whether the underlying work-item lineage is preserved on this component regardless of
    /// narrowing; must hold so stale-freshness, read-only, local-only, and unpublishable-packet
    /// states never fail opaquely.
    pub lineage_preserved: bool,
    /// Rendering surfaces this component is certified on.
    #[serde(default)]
    pub rendering_surfaces: Vec<M5WorkItemComponentRenderingSurface>,
    /// Per-surface narrowing disclosures.
    #[serde(default)]
    pub narrowing_disclosures: Vec<WorkItemComponentRenderingNarrowingDisclosure>,
    /// The required labels the accessible fallback preserves (reused vocabulary).
    #[serde(default)]
    pub required_labels: Vec<M5WorkItemRequiredLabel>,
    /// Semantic consumer surfaces this component is embedded in (reused vocabulary).
    #[serde(default)]
    pub consumer_surfaces: Vec<M5WorkItemConsumerSurface>,
    /// Source contract refs backing this row.
    #[serde(default)]
    pub source_refs: Vec<String>,
    /// ISO 8601 UTC timestamp the accessibility posture was observed.
    pub observed_at: String,
    /// Evidence packet refs backing this row.
    #[serde(default)]
    pub evidence_refs: Vec<String>,
}

impl WorkItemComponentAccessibilityRow {
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

    /// The condition state observed for one dimension, or `FreshCommitted` when the row does not
    /// model that dimension.
    pub fn condition_for(
        &self,
        dimension: M5WorkItemComponentClaimDimension,
    ) -> M5WorkItemComponentConditionState {
        self.claim_conditions
            .iter()
            .find(|c| c.dimension == dimension)
            .map(|c| c.state)
            .unwrap_or(M5WorkItemComponentConditionState::FreshCommitted)
    }

    /// Whether any modeled dimension is weaker than committed.
    pub fn has_weak_dimension(&self) -> bool {
        self.claim_conditions.iter().any(|c| c.state.is_weak())
    }

    /// The strongest provider claim permitted after applying every modeled dimension's ceiling,
    /// capped at the family's full claim.
    pub fn permitted_claim(&self) -> M5WorkItemComponentClaim {
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
    pub fn binding_condition(&self) -> Option<&WorkItemComponentClaimConditionEntry> {
        let mut binding: Option<(&WorkItemComponentClaimConditionEntry, u8)> = None;
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
    pub fn binding_dimension(&self) -> Option<M5WorkItemComponentClaimDimension> {
        self.binding_condition().map(|c| c.dimension)
    }

    /// The provider claim this component effectively asserts after narrowing.
    pub fn effective_claim(&self) -> M5WorkItemComponentClaim {
        match &self.claim_narrow {
            Some(narrow) => narrow.narrowed_to,
            None => self.full_provider_claim,
        }
    }

    /// AC / auto-narrowing honesty: a stale freshness, a read-only / policy-blocked write scope,
    /// a local-only sync state, or an unpublishable packet can no longer keep an old
    /// `ProviderCommitted` / `ReviewableProjection` label. The effective claim never exceeds the
    /// permitted ceiling; when a dimension narrows below the full claim, an honest narrow block
    /// is present, narrows to exactly the permitted ceiling, binds to the ceiling-imposing
    /// dimension with its frozen trigger, and preserves canonical identity and provider lineage.
    /// When nothing narrows, no spurious narrow block is present.
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

    /// AC / commit honesty: a cached or offline state (stale freshness, local-only sync,
    /// unpublishable packet) never keeps a committed claim. When such a state is modeled, the
    /// effective claim must not assert `ProviderCommitted`.
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
            && !self.work_item_context_ref.trim().is_empty()
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

    /// AC / no-loss: stale-freshness, read-only, local-only, and unpublishable-packet states
    /// preserve the underlying work-item lineage. The row must assert `lineage_preserved`, and
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
        M5WorkItemRequiredLabel::MANDATORY
            .iter()
            .all(|label| self.required_labels.contains(label))
    }

    /// Derived qualification status.
    pub fn status(&self) -> WorkItemComponentAccessibilityStatus {
        if !self.claim_is_honest()
            || !self.commit_honesty_holds()
            || !self.reaches_canonical_truth_via_at()
            || !self.export_preserves_meaning()
            || !self.preserves_lineage_continuity()
            || !self.narrowing_disclosed()
            || !self.models_primary_dimension()
            || !self.preserves_mandatory_labels()
        {
            return WorkItemComponentAccessibilityStatus::Stranded;
        }
        if self.is_reduced() {
            WorkItemComponentAccessibilityStatus::NarrowedDisclosed
        } else {
            WorkItemComponentAccessibilityStatus::Parity
        }
    }

    /// Whether the row's identity and evidence fields are complete.
    pub fn is_complete(&self) -> bool {
        self.record_kind == WORK_ITEM_COMPONENT_A11Y_ROW_RECORD_KIND
            && self.schema_version == WORK_ITEM_COMPONENT_A11Y_SCHEMA_VERSION
            && !self.row_id.trim().is_empty()
            && !self.source_family_schema_ref.trim().is_empty()
            && !self.work_item_context_ref.trim().is_empty()
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

/// Rolled-up summary of an M05-986 work-item component accessibility parity packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorkItemComponentAccessibilitySummary {
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

/// Constructor input for [`WorkItemComponentAccessibilityPacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WorkItemComponentAccessibilityPacketInput {
    pub packet_id: String,
    pub as_of: String,
    pub matrix_ref: String,
    pub rows: Vec<WorkItemComponentAccessibilityRow>,
}

/// Checked-in M05-986 work-item component accessibility parity packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorkItemComponentAccessibilityPacket {
    pub schema_version: u32,
    pub record_kind: String,
    pub packet_id: String,
    pub as_of: String,
    pub matrix_ref: String,
    #[serde(default)]
    pub rows: Vec<WorkItemComponentAccessibilityRow>,
    pub summary: WorkItemComponentAccessibilitySummary,
}

impl WorkItemComponentAccessibilityPacket {
    /// Builds a packet, stamping the record kind, schema version, and computed summary.
    pub fn new(input: WorkItemComponentAccessibilityPacketInput) -> Self {
        let mut packet = Self {
            schema_version: WORK_ITEM_COMPONENT_A11Y_SCHEMA_VERSION,
            record_kind: WORK_ITEM_COMPONENT_A11Y_RECORD_KIND.to_owned(),
            packet_id: input.packet_id,
            as_of: input.as_of,
            matrix_ref: input.matrix_ref,
            rows: input.rows,
            summary: WorkItemComponentAccessibilitySummary {
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
    pub fn represented_families(&self) -> BTreeSet<M5WorkItemComponentFamily> {
        self.rows.iter().map(|r| r.component_family).collect()
    }

    /// Dimensions exercised by some row's claim conditions.
    pub fn exercised_dimensions(&self) -> BTreeSet<M5WorkItemComponentClaimDimension> {
        self.rows
            .iter()
            .flat_map(|r| r.claim_conditions.iter().map(|c| c.dimension))
            .collect()
    }

    /// Condition states exercised by some row's claim conditions.
    pub fn exercised_condition_states(&self) -> BTreeSet<M5WorkItemComponentConditionState> {
        self.rows
            .iter()
            .flat_map(|r| r.claim_conditions.iter().map(|c| c.state))
            .collect()
    }

    /// Provider claim tiers that appear as an effective claim across the rows.
    pub fn represented_effective_claims(&self) -> BTreeSet<M5WorkItemComponentClaim> {
        self.rows.iter().map(|r| r.effective_claim()).collect()
    }

    /// Consumer surfaces ingesting some row in this packet.
    pub fn represented_consumer_surfaces(&self) -> BTreeSet<M5WorkItemConsumerSurface> {
        self.rows
            .iter()
            .flat_map(|r| r.consumer_surfaces.iter().copied())
            .collect()
    }

    /// Computes summary fields from the packet contents.
    pub fn computed_summary(&self) -> WorkItemComponentAccessibilitySummary {
        let mut rendering = BTreeSet::new();
        let mut consumers: BTreeSet<M5WorkItemConsumerSurface> = BTreeSet::new();
        for row in &self.rows {
            rendering.extend(row.rendering_surfaces.iter().copied());
            consumers.extend(row.consumer_surfaces.iter().copied());
        }

        let hierarchy_heavy: Vec<&WorkItemComponentAccessibilityRow> = self
            .rows
            .iter()
            .filter(|row| row.is_hierarchy_heavy())
            .collect();

        let mut green = 0;
        let mut yellow = 0;
        let mut red = 0;
        for row in &self.rows {
            match row.status() {
                WorkItemComponentAccessibilityStatus::Parity => green += 1,
                WorkItemComponentAccessibilityStatus::NarrowedDisclosed => yellow += 1,
                WorkItemComponentAccessibilityStatus::Stranded => red += 1,
            }
        }

        WorkItemComponentAccessibilitySummary {
            row_count: self.rows.len(),
            family_count: self.represented_families().len(),
            hierarchy_heavy_family_count: hierarchy_heavy.len(),
            all_hierarchy_heavy_have_non_visual_fallback: hierarchy_heavy
                .iter()
                .all(|row| row.has_non_visual_fallback()),
            all_reach_canonical_truth_via_at: self
                .rows
                .iter()
                .all(WorkItemComponentAccessibilityRow::reaches_canonical_truth_via_at),
            all_claims_honest: self
                .rows
                .iter()
                .all(WorkItemComponentAccessibilityRow::claim_is_honest),
            all_commit_honesty_holds: self
                .rows
                .iter()
                .all(WorkItemComponentAccessibilityRow::commit_honesty_holds),
            all_export_summaries_preserve_meaning: self
                .rows
                .iter()
                .all(WorkItemComponentAccessibilityRow::export_preserves_meaning),
            all_lineage_preserved: self
                .rows
                .iter()
                .all(WorkItemComponentAccessibilityRow::preserves_lineage_continuity),
            all_narrowing_disclosed: self
                .rows
                .iter()
                .all(WorkItemComponentAccessibilityRow::narrowing_disclosed),
            green_count: green,
            yellow_count: yellow,
            red_count: red,
            rendering_surface_count: rendering.len(),
            consumer_surface_count: consumers.len(),
        }
    }

    /// Validates the packet and returns every contract violation.
    pub fn validate(&self) -> Vec<WorkItemComponentAccessibilityViolation> {
        let mut violations = Vec::new();

        if self.schema_version != WORK_ITEM_COMPONENT_A11Y_SCHEMA_VERSION {
            violations.push(WorkItemComponentAccessibilityViolation::SchemaVersion {
                expected: WORK_ITEM_COMPONENT_A11Y_SCHEMA_VERSION,
                actual: self.schema_version,
            });
        }
        if self.record_kind != WORK_ITEM_COMPONENT_A11Y_RECORD_KIND {
            violations.push(WorkItemComponentAccessibilityViolation::RecordKind {
                expected: WORK_ITEM_COMPONENT_A11Y_RECORD_KIND.to_owned(),
                actual: self.record_kind.clone(),
            });
        }
        if self.packet_id.trim().is_empty()
            || self.as_of.trim().is_empty()
            || self.matrix_ref.trim().is_empty()
        {
            violations.push(WorkItemComponentAccessibilityViolation::MissingIdentity);
        }

        let mut row_ids = BTreeSet::new();
        let mut seen_families = BTreeSet::new();
        let mut has_cached_or_offline_row = false;
        for row in &self.rows {
            if !row_ids.insert(row.row_id.clone()) {
                violations.push(WorkItemComponentAccessibilityViolation::DuplicateId {
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
                violations.push(WorkItemComponentAccessibilityViolation::IncompleteRow {
                    id: row.row_id.clone(),
                });
            }

            // Each row must model its family's primary weakening dimension.
            if !row.models_primary_dimension() {
                violations.push(
                    WorkItemComponentAccessibilityViolation::MissingPrimaryDimension {
                        id: row.row_id.clone(),
                        dimension: family_primary_dimension(row.component_family),
                    },
                );
            }

            // Each row must preserve every mandatory work-item label.
            if !row.preserves_mandatory_labels() {
                violations.push(
                    WorkItemComponentAccessibilityViolation::MissingMandatoryLabel {
                        id: row.row_id.clone(),
                    },
                );
            }

            // A hierarchy-heavy family must render a structured tree *and* a non-visual path.
            if row.is_hierarchy_heavy()
                && !row
                    .fallback_modalities
                    .contains(&M5WorkItemComponentFallbackModality::Structured)
            {
                violations.push(
                    WorkItemComponentAccessibilityViolation::HierarchyHeavyMissingStructured {
                        id: row.row_id.clone(),
                    },
                );
            }

            // AC1: claim never over-asserts a committed / reviewable surface for a weakened one.
            if !row.claim_is_honest() {
                violations.push(WorkItemComponentAccessibilityViolation::ClaimOverAsserted {
                    id: row.row_id.clone(),
                });
            }

            // AC2: a cached or offline state never keeps a committed claim.
            if !row.commit_honesty_holds() {
                violations.push(
                    WorkItemComponentAccessibilityViolation::CachedOrOfflineShownAsCommitted {
                        id: row.row_id.clone(),
                    },
                );
            }

            // Assistive-tech / CLI reach the same canonical truth.
            if !row.reaches_canonical_truth_via_at() {
                violations.push(
                    WorkItemComponentAccessibilityViolation::AssistiveTechStranded {
                        id: row.row_id.clone(),
                    },
                );
            }

            // Export preserves meaning without a screenshot.
            if !row.export_preserves_meaning() {
                violations.push(
                    WorkItemComponentAccessibilityViolation::ExportRequiresScreenshot {
                        id: row.row_id.clone(),
                    },
                );
            }

            // AC / no-loss: stale-freshness, read-only, local-only, and unpublishable-packet
            // states preserve work-item lineage.
            if !row.preserves_lineage_continuity() {
                violations.push(WorkItemComponentAccessibilityViolation::LineageDropped {
                    id: row.row_id.clone(),
                });
            }

            // Narrowing disclosed on every narrowed rendering surface.
            if !row.narrowing_disclosed() {
                violations.push(
                    WorkItemComponentAccessibilityViolation::NarrowingDropsContextSilently {
                        id: row.row_id.clone(),
                    },
                );
            }

            // Consumer parity: at least two consumer surfaces ingest the row.
            if row.consumer_surfaces.len() < 2 {
                violations.push(
                    WorkItemComponentAccessibilityViolation::MissingConsumerParity {
                        id: row.row_id.clone(),
                    },
                );
            }

            // No red rows may ship.
            if row.status() == WorkItemComponentAccessibilityStatus::Stranded {
                violations.push(WorkItemComponentAccessibilityViolation::StrandedRow {
                    id: row.row_id.clone(),
                });
            }
        }

        // Coverage: every frozen family is certified at least once.
        for family in M5WorkItemComponentFamily::ALL {
            if !seen_families.contains(&family) {
                violations.push(
                    WorkItemComponentAccessibilityViolation::MissingFamilyCoverage { family },
                );
            }
        }

        // Coverage: every weakening dimension is exercised somewhere.
        let exercised = self.exercised_dimensions();
        for dimension in M5WorkItemComponentClaimDimension::ALL {
            if !exercised.contains(&dimension) {
                violations.push(
                    WorkItemComponentAccessibilityViolation::MissingDimensionCoverage { dimension },
                );
            }
        }

        // Coverage: every condition state (the committed baseline plus each spec narrowing axis)
        // is exercised somewhere, so the full narrowing spectrum is proven end-to-end.
        let states = self.exercised_condition_states();
        for state in M5WorkItemComponentConditionState::ALL {
            if !states.contains(&state) {
                violations.push(
                    WorkItemComponentAccessibilityViolation::MissingConditionStateCoverage {
                        state,
                    },
                );
            }
        }

        // Coverage: every provider claim tier appears as an effective claim, so the full
        // narrowing spectrum (provider-committed → … → unpublishable-packet) is proven end-to-end.
        let effective = self.represented_effective_claims();
        for claim in M5WorkItemComponentClaim::ALL {
            if !effective.contains(&claim) {
                violations.push(
                    WorkItemComponentAccessibilityViolation::MissingClaimTierCoverage { claim },
                );
            }
        }

        // Commit honesty must be proven with at least one cached-or-offline row in the packet, so
        // the "cached / offline never shown as committed" guarantee is exercised end-to-end.
        if !has_cached_or_offline_row {
            violations.push(WorkItemComponentAccessibilityViolation::CommitHonestyUnproven);
        }

        // Cross-surface: the same narrowed state must reach the inbox, detail, relation-panel,
        // sync-status, transition-sheet, evidence-panel, product UI, CLI, and support / release
        // exports — so every consumer surface is exercised at least once across the packet.
        let consumers = self.represented_consumer_surfaces();
        for surface in M5WorkItemConsumerSurface::ALL {
            if !consumers.contains(&surface) {
                violations.push(
                    WorkItemComponentAccessibilityViolation::MissingConsumerSurfaceCoverage {
                        surface,
                    },
                );
            }
        }

        if self.summary != self.computed_summary() {
            violations.push(WorkItemComponentAccessibilityViolation::SummaryMismatch);
        }

        if json_contains_forbidden_material(
            &serde_json::to_value(self)
                .expect("work-item component accessibility parity packet serializes"),
        ) {
            violations.push(WorkItemComponentAccessibilityViolation::RawProviderMaterialInExport);
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
            .expect("work-item component accessibility parity packet serializes")
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
        out.push_str("# M5 Work-Item Component Accessibility & Auto-Narrowing\n\n");
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- As of: `{}`\n", self.as_of));
        out.push_str(&format!(
            "- Families: {} certified across {} / {} frozen families\n",
            self.summary.family_count,
            self.represented_families().len(),
            M5WorkItemComponentFamily::ALL.len(),
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

/// Reads and validates the checked-in work-item component accessibility parity export.
pub fn current_m5_work_item_component_a11y_export(
) -> Result<WorkItemComponentAccessibilityPacket, WorkItemComponentAccessibilityArtifactError> {
    let packet: WorkItemComponentAccessibilityPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/release/m5-work-item-component-accessibility-proof/support_export.json"
    )))
    .map_err(WorkItemComponentAccessibilityArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(WorkItemComponentAccessibilityArtifactError::Validation(
            violations,
        ))
    }
}

/// Errors emitted when reading the checked-in work-item component accessibility parity export.
#[derive(Debug)]
pub enum WorkItemComponentAccessibilityArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<WorkItemComponentAccessibilityViolation>),
}

impl fmt::Display for WorkItemComponentAccessibilityArtifactError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(
                    f,
                    "work-item component accessibility parity export parse failed: {error}"
                )
            }
            Self::Validation(violations) => {
                write!(
                    f,
                    "work-item component accessibility parity export failed validation: {} violation(s)",
                    violations.len()
                )
            }
        }
    }
}

impl Error for WorkItemComponentAccessibilityArtifactError {}

/// Validation failure for M05-986 work-item component accessibility parity packets.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WorkItemComponentAccessibilityViolation {
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
        dimension: M5WorkItemComponentClaimDimension,
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
        family: M5WorkItemComponentFamily,
    },
    MissingDimensionCoverage {
        dimension: M5WorkItemComponentClaimDimension,
    },
    MissingConditionStateCoverage {
        state: M5WorkItemComponentConditionState,
    },
    MissingClaimTierCoverage {
        claim: M5WorkItemComponentClaim,
    },
    CommitHonestyUnproven,
    MissingConsumerSurfaceCoverage {
        surface: M5WorkItemConsumerSurface,
    },
    SummaryMismatch,
    RawProviderMaterialInExport,
}

impl fmt::Display for WorkItemComponentAccessibilityViolation {
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
                write!(f, "row {id} drops a mandatory work-item label")
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
                    "row {id} does not preserve work-item lineage across narrowing"
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

impl Error for WorkItemComponentAccessibilityViolation {}

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
            | "local only"
            | "offline"
            | "cached"
            | "unpublished"
            | "ticket"
            | "task"
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

/// Builds the canonical, checked-in work-item component accessibility parity packet. This is the
/// one source of truth shared by the tests and the on-disk support export so both stay
/// byte-aligned.
pub fn seeded_m5_work_item_component_a11y_packet() -> WorkItemComponentAccessibilityPacket {
    WorkItemComponentAccessibilityPacket::new(WorkItemComponentAccessibilityPacketInput {
        packet_id: "m5-work-item-component-accessibility-parity:stable:0001".to_owned(),
        as_of: "2026-07-09T00:00:00Z".to_owned(),
        matrix_ref: WORK_ITEM_COMPONENT_A11Y_COMPONENT_MATRIX_REF.to_owned(),
        rows: seeded_rows(),
    })
}

fn ev(id: &str) -> Vec<String> {
    vec![format!("evidence:work-item-component-a11y:{id}")]
}

fn all_required_labels() -> Vec<M5WorkItemRequiredLabel> {
    M5WorkItemRequiredLabel::ALL.to_vec()
}

fn copy_export(fields: &[&str]) -> WorkItemComponentCopyExportParity {
    WorkItemComponentCopyExportParity {
        formats: vec!["text".to_owned(), "json".to_owned(), "markdown".to_owned()],
        export_fields: fields.iter().map(|f| (*f).to_owned()).collect(),
        screenshot_only_prohibited: true,
    }
}

fn condition(
    dimension: M5WorkItemComponentClaimDimension,
    state: M5WorkItemComponentConditionState,
) -> WorkItemComponentClaimConditionEntry {
    WorkItemComponentClaimConditionEntry { dimension, state }
}

/// The two consumer surfaces every row ships to at minimum — support / release export and CLI
/// inspect — so the narrowed state always reaches headless field triage.
fn base_consumers(extra: &[M5WorkItemConsumerSurface]) -> Vec<M5WorkItemConsumerSurface> {
    let mut out = vec![
        M5WorkItemConsumerSurface::SupportExport,
        M5WorkItemConsumerSurface::CliInspect,
    ];
    out.extend_from_slice(extra);
    out
}

/// Disclosures for the CLI-headless and support-export surfaces. A green (full parity) row keeps
/// full label and summary parity on the narrower surfaces; a narrowed row discloses the reduced
/// interactions it drops there.
fn surface_disclosures(
    labels: &[&str],
    state: WorkItemComponentNarrowingDisclosureState,
) -> Vec<WorkItemComponentRenderingNarrowingDisclosure> {
    let preserved: Vec<String> = labels.iter().map(|l| (*l).to_owned()).collect();
    vec![
        WorkItemComponentRenderingNarrowingDisclosure {
            rendering_surface: M5WorkItemComponentRenderingSurface::CliHeadless,
            state,
            preserved_labels: preserved.clone(),
            reduced_interactions: vec!["pointer_interaction".to_owned()],
        },
        WorkItemComponentRenderingNarrowingDisclosure {
            rendering_surface: M5WorkItemComponentRenderingSurface::SupportExport,
            state,
            preserved_labels: preserved,
            reduced_interactions: vec!["live_provider_write".to_owned()],
        },
    ]
}

/// Disclosures for a full-parity (green) row: the narrower surfaces preserve full label and
/// summary parity.
fn parity_surfaces(labels: &[&str]) -> Vec<WorkItemComponentRenderingNarrowingDisclosure> {
    surface_disclosures(
        labels,
        WorkItemComponentNarrowingDisclosureState::ParityPreserved,
    )
}

/// Disclosures for a narrowed (yellow) row: the narrower surfaces disclose their reduced
/// interactions while preserving labels.
fn narrowed_surfaces(labels: &[&str]) -> Vec<WorkItemComponentRenderingNarrowingDisclosure> {
    surface_disclosures(
        labels,
        WorkItemComponentNarrowingDisclosureState::DisclosedNarrowed,
    )
}

fn rendering_surfaces() -> Vec<M5WorkItemComponentRenderingSurface> {
    vec![
        M5WorkItemComponentRenderingSurface::DesktopFull,
        M5WorkItemComponentRenderingSurface::CliHeadless,
        M5WorkItemComponentRenderingSurface::SupportExport,
    ]
}

fn seeded_rows() -> Vec<WorkItemComponentAccessibilityRow> {
    vec![
        // Work-item row (provider freshness stale) — the row's provider projection has gone stale
        // so only a cached read is available; it auto-narrows to a stale-freshness projection
        // rather than presenting a live, committed work item, while keeping its canonical
        // identity, provider authority, and local-versus-provider state visible (yellow).
        WorkItemComponentAccessibilityRow {
            record_kind: WORK_ITEM_COMPONENT_A11Y_ROW_RECORD_KIND.to_owned(),
            schema_version: WORK_ITEM_COMPONENT_A11Y_SCHEMA_VERSION,
            row_id: "a11y:work-item-row-freshness-stale".to_owned(),
            component_family: M5WorkItemComponentFamily::WorkItemRow,
            source_family_schema_ref: WORK_ITEM_COMPONENT_A11Y_COMPONENT_MATRIX_REF.to_owned(),
            work_item_context_ref: "work-item:row:0001".to_owned(),
            fallback_modalities: vec![
                M5WorkItemComponentFallbackModality::List,
                M5WorkItemComponentFallbackModality::Textual,
                M5WorkItemComponentFallbackModality::Cli,
            ],
            reaches_canonical_truth: true,
            keyboard_reach: WorkItemComponentNonVisualReachState::ReachableAndLabeled,
            screen_reader_reach: WorkItemComponentNonVisualReachState::ReachableAndLabeled,
            cli_reach: WorkItemComponentNonVisualReachState::ReachableAndLabeled,
            export_summary: WorkItemComponentExportSummaryState::ReconstructableWithoutScreenshot,
            export_summary_ref: "summary:work-item-row-freshness-stale:a11y".to_owned(),
            copy_export: copy_export(&[
                "canonical_id",
                "provider_authority",
                "local_versus_provider_state",
                "keyboard_route",
            ]),
            full_provider_claim: M5WorkItemComponentClaim::ProviderCommitted,
            claim_conditions: vec![condition(
                M5WorkItemComponentClaimDimension::ProviderFreshness,
                M5WorkItemComponentConditionState::FreshnessStale,
            )],
            claim_narrow: Some(WorkItemComponentClaimAutoNarrow {
                narrowed_to: M5WorkItemComponentClaim::StaleFreshnessProjection,
                binding_dimension: M5WorkItemComponentClaimDimension::ProviderFreshness,
                trigger: M5WorkItemDowngradeTrigger::LocalVersusProviderStateHidden,
                narrowed_label:
                    "Provider projection has gone stale and only a cached read remains — shown as a stale-freshness projection that must refresh before it is trusted as live, with its canonical ID and provider authority still preserved, never as a live committed work item"
                        .to_owned(),
                preserves_canonical_identity: true,
                preserves_lineage_continuity: true,
            }),
            lineage_preserved: true,
            rendering_surfaces: rendering_surfaces(),
            narrowing_disclosures: narrowed_surfaces(&[
                "canonical_id",
                "provider_authority",
                "local_versus_provider_state",
            ]),
            required_labels: all_required_labels(),
            consumer_surfaces: base_consumers(&[
                M5WorkItemConsumerSurface::InboxUi,
                M5WorkItemConsumerSurface::ProductUi,
            ]),
            source_refs: vec![
                "UI/UX Spec §13.7 work items and change traceability".to_owned(),
                WORK_ITEM_COMPONENT_A11Y_DOC_REF.to_owned(),
            ],
            observed_at: "2026-07-09T00:00:00Z".to_owned(),
            evidence_refs: ev("work-item-row-freshness-stale"),
        },
        // Provider-chip group (write scope read-only / policy-blocked) — the granted write scope
        // is read-only or policy-blocked, so the chip group auto-narrows to a read-only
        // projection rather than presenting a committed write path, while naming the provider
        // authority and write posture (yellow).
        WorkItemComponentAccessibilityRow {
            record_kind: WORK_ITEM_COMPONENT_A11Y_ROW_RECORD_KIND.to_owned(),
            schema_version: WORK_ITEM_COMPONENT_A11Y_SCHEMA_VERSION,
            row_id: "a11y:provider-chip-group-write-scope-blocked".to_owned(),
            component_family: M5WorkItemComponentFamily::ProviderChipGroup,
            source_family_schema_ref: WORK_ITEM_COMPONENT_A11Y_COMPONENT_MATRIX_REF.to_owned(),
            work_item_context_ref: "work-item:provider-chip-group:0002".to_owned(),
            fallback_modalities: vec![
                M5WorkItemComponentFallbackModality::List,
                M5WorkItemComponentFallbackModality::Textual,
                M5WorkItemComponentFallbackModality::Cli,
            ],
            reaches_canonical_truth: true,
            keyboard_reach: WorkItemComponentNonVisualReachState::ReachableAndLabeled,
            screen_reader_reach: WorkItemComponentNonVisualReachState::ReachableAndLabeled,
            cli_reach: WorkItemComponentNonVisualReachState::ReachableAndLabeled,
            export_summary: WorkItemComponentExportSummaryState::ReconstructableWithoutScreenshot,
            export_summary_ref: "summary:provider-chip-group-write-scope-blocked:a11y".to_owned(),
            copy_export: copy_export(&[
                "provider_authority",
                "write_scope",
                "policy_source",
                "keyboard_route",
            ]),
            full_provider_claim: M5WorkItemComponentClaim::ProviderCommitted,
            claim_conditions: vec![condition(
                M5WorkItemComponentClaimDimension::WriteScope,
                M5WorkItemComponentConditionState::WriteScopeBlocked,
            )],
            claim_narrow: Some(WorkItemComponentClaimAutoNarrow {
                narrowed_to: M5WorkItemComponentClaim::ReadOnlyProjection,
                binding_dimension: M5WorkItemComponentClaimDimension::WriteScope,
                trigger: M5WorkItemDowngradeTrigger::ProviderAuthorityUnstated,
                narrowed_label:
                    "Granted write scope is read-only or held by policy — shown as a read-only projection that names its provider authority and policy source, never as a committed write path"
                        .to_owned(),
                preserves_canonical_identity: true,
                preserves_lineage_continuity: true,
            }),
            lineage_preserved: true,
            rendering_surfaces: rendering_surfaces(),
            narrowing_disclosures: narrowed_surfaces(&[
                "provider_authority",
                "write_scope",
                "policy_source",
            ]),
            required_labels: all_required_labels(),
            consumer_surfaces: base_consumers(&[
                M5WorkItemConsumerSurface::InboxUi,
                M5WorkItemConsumerSurface::DetailUi,
            ]),
            source_refs: vec![
                "UX Design System §16.71 provider chips".to_owned(),
                WORK_ITEM_COMPONENT_A11Y_DOC_REF.to_owned(),
            ],
            observed_at: "2026-07-09T00:00:00Z".to_owned(),
            evidence_refs: ev("provider-chip-group-write-scope-blocked"),
        },
        // Relation strip — the linked engineering context is fresh and resolved and the strip is
        // a self-sufficient, reviewable read-only projection (not itself a committed write),
        // reachable on every surface (green).
        WorkItemComponentAccessibilityRow {
            record_kind: WORK_ITEM_COMPONENT_A11Y_ROW_RECORD_KIND.to_owned(),
            schema_version: WORK_ITEM_COMPONENT_A11Y_SCHEMA_VERSION,
            row_id: "a11y:relation-strip".to_owned(),
            component_family: M5WorkItemComponentFamily::RelationStrip,
            source_family_schema_ref: WORK_ITEM_COMPONENT_A11Y_COMPONENT_MATRIX_REF.to_owned(),
            work_item_context_ref: "work-item:relation-strip:0003".to_owned(),
            fallback_modalities: vec![
                M5WorkItemComponentFallbackModality::List,
                M5WorkItemComponentFallbackModality::Textual,
                M5WorkItemComponentFallbackModality::Cli,
            ],
            reaches_canonical_truth: true,
            keyboard_reach: WorkItemComponentNonVisualReachState::ReachableAndLabeled,
            screen_reader_reach: WorkItemComponentNonVisualReachState::ReachableAndLabeled,
            cli_reach: WorkItemComponentNonVisualReachState::ReachableAndLabeled,
            export_summary: WorkItemComponentExportSummaryState::ReconstructableWithoutScreenshot,
            export_summary_ref: "summary:relation-strip:a11y".to_owned(),
            copy_export: copy_export(&[
                "linked_engineering_context",
                "relation_kind",
                "relation_health",
                "keyboard_route",
            ]),
            full_provider_claim: M5WorkItemComponentClaim::ReviewableProjection,
            claim_conditions: vec![condition(
                M5WorkItemComponentClaimDimension::ProviderFreshness,
                M5WorkItemComponentConditionState::FreshCommitted,
            )],
            claim_narrow: None,
            lineage_preserved: true,
            rendering_surfaces: rendering_surfaces(),
            narrowing_disclosures: parity_surfaces(&[
                "linked_engineering_context",
                "relation_kind",
                "relation_health",
            ]),
            required_labels: all_required_labels(),
            consumer_surfaces: base_consumers(&[
                M5WorkItemConsumerSurface::RelationPanelUi,
                M5WorkItemConsumerSurface::DetailUi,
            ]),
            source_refs: vec![
                "UX Design System §16.72 relation strips".to_owned(),
                WORK_ITEM_COMPONENT_A11Y_DOC_REF.to_owned(),
            ],
            observed_at: "2026-07-09T00:00:00Z".to_owned(),
            evidence_refs: ev("relation-strip"),
        },
        // Sync-pending pill (sync state local-only) — the work item is local-only and not yet
        // synced, so the pill auto-narrows to a local-only projection and must never be shown as
        // a committed provider write (cached-never-committed), keeping its queued-draft and
        // publish-later state visible (yellow).
        WorkItemComponentAccessibilityRow {
            record_kind: WORK_ITEM_COMPONENT_A11Y_ROW_RECORD_KIND.to_owned(),
            schema_version: WORK_ITEM_COMPONENT_A11Y_SCHEMA_VERSION,
            row_id: "a11y:sync-pending-pill-local-only".to_owned(),
            component_family: M5WorkItemComponentFamily::SyncPendingPill,
            source_family_schema_ref: WORK_ITEM_COMPONENT_A11Y_COMPONENT_MATRIX_REF.to_owned(),
            work_item_context_ref: "work-item:sync-pending-pill:0004".to_owned(),
            fallback_modalities: vec![
                M5WorkItemComponentFallbackModality::List,
                M5WorkItemComponentFallbackModality::Textual,
                M5WorkItemComponentFallbackModality::Cli,
            ],
            reaches_canonical_truth: true,
            keyboard_reach: WorkItemComponentNonVisualReachState::ReachableAndLabeled,
            screen_reader_reach: WorkItemComponentNonVisualReachState::ReachableAndLabeled,
            cli_reach: WorkItemComponentNonVisualReachState::ReachableAndLabeled,
            export_summary: WorkItemComponentExportSummaryState::ReconstructableWithoutScreenshot,
            export_summary_ref: "summary:sync-pending-pill-local-only:a11y".to_owned(),
            copy_export: copy_export(&[
                "local_versus_provider_state",
                "queued_draft_state",
                "publish_later_continuity",
                "keyboard_route",
            ]),
            full_provider_claim: M5WorkItemComponentClaim::ProviderCommitted,
            claim_conditions: vec![condition(
                M5WorkItemComponentClaimDimension::SyncState,
                M5WorkItemComponentConditionState::SyncLocalOnly,
            )],
            claim_narrow: Some(WorkItemComponentClaimAutoNarrow {
                narrowed_to: M5WorkItemComponentClaim::LocalOnlyProjection,
                binding_dimension: M5WorkItemComponentClaimDimension::SyncState,
                trigger: M5WorkItemDowngradeTrigger::SyncPendingStateHidden,
                narrowed_label:
                    "Change is held on this machine and nothing has been synced to the provider — shown as a local-only projection with its queued-draft count and publish-later route preserved, never as a committed provider write"
                        .to_owned(),
                preserves_canonical_identity: true,
                preserves_lineage_continuity: true,
            }),
            lineage_preserved: true,
            rendering_surfaces: rendering_surfaces(),
            narrowing_disclosures: narrowed_surfaces(&[
                "local_versus_provider_state",
                "queued_draft_state",
                "publish_later_continuity",
            ]),
            required_labels: all_required_labels(),
            consumer_surfaces: base_consumers(&[
                M5WorkItemConsumerSurface::SyncStatusUi,
                M5WorkItemConsumerSurface::ProductUi,
            ]),
            source_refs: vec![
                "UX Design System §20.13 sync-pending pills".to_owned(),
                WORK_ITEM_COMPONENT_A11Y_DOC_REF.to_owned(),
            ],
            observed_at: "2026-07-09T00:00:00Z".to_owned(),
            evidence_refs: ev("sync-pending-pill-local-only"),
        },
        // Work-item detail header — fresh, in scope, and provider-synced with its canonical
        // identity and provider authority stated, so the header is fully committed and reachable
        // on every surface (green).
        WorkItemComponentAccessibilityRow {
            record_kind: WORK_ITEM_COMPONENT_A11Y_ROW_RECORD_KIND.to_owned(),
            schema_version: WORK_ITEM_COMPONENT_A11Y_SCHEMA_VERSION,
            row_id: "a11y:work-item-detail-header".to_owned(),
            component_family: M5WorkItemComponentFamily::WorkItemDetailHeader,
            source_family_schema_ref: WORK_ITEM_COMPONENT_A11Y_COMPONENT_MATRIX_REF.to_owned(),
            work_item_context_ref: "work-item:detail-header:0005".to_owned(),
            fallback_modalities: vec![
                M5WorkItemComponentFallbackModality::List,
                M5WorkItemComponentFallbackModality::Textual,
                M5WorkItemComponentFallbackModality::Cli,
            ],
            reaches_canonical_truth: true,
            keyboard_reach: WorkItemComponentNonVisualReachState::ReachableAndLabeled,
            screen_reader_reach: WorkItemComponentNonVisualReachState::ReachableAndLabeled,
            cli_reach: WorkItemComponentNonVisualReachState::ReachableAndLabeled,
            export_summary: WorkItemComponentExportSummaryState::ReconstructableWithoutScreenshot,
            export_summary_ref: "summary:work-item-detail-header:a11y".to_owned(),
            copy_export: copy_export(&[
                "canonical_id",
                "provider_authority",
                "write_scope",
                "keyboard_route",
            ]),
            full_provider_claim: M5WorkItemComponentClaim::ProviderCommitted,
            claim_conditions: vec![condition(
                M5WorkItemComponentClaimDimension::ProviderFreshness,
                M5WorkItemComponentConditionState::FreshCommitted,
            )],
            claim_narrow: None,
            lineage_preserved: true,
            rendering_surfaces: rendering_surfaces(),
            narrowing_disclosures: parity_surfaces(&[
                "canonical_id",
                "provider_authority",
                "write_scope",
            ]),
            required_labels: all_required_labels(),
            consumer_surfaces: base_consumers(&[
                M5WorkItemConsumerSurface::DetailUi,
                M5WorkItemConsumerSurface::ProductUi,
            ]),
            source_refs: vec![
                "UX Design System §16.71 work-item detail headers".to_owned(),
                WORK_ITEM_COMPONENT_A11Y_DOC_REF.to_owned(),
            ],
            observed_at: "2026-07-09T00:00:00Z".to_owned(),
            evidence_refs: ev("work-item-detail-header"),
        },
        // Status-transition sheet — in write scope with a stated side-effect preview and
        // permission scope before write, so the sheet is fully committed and reachable on every
        // surface (green).
        WorkItemComponentAccessibilityRow {
            record_kind: WORK_ITEM_COMPONENT_A11Y_ROW_RECORD_KIND.to_owned(),
            schema_version: WORK_ITEM_COMPONENT_A11Y_SCHEMA_VERSION,
            row_id: "a11y:status-transition-sheet".to_owned(),
            component_family: M5WorkItemComponentFamily::StatusTransitionSheet,
            source_family_schema_ref: WORK_ITEM_COMPONENT_A11Y_COMPONENT_MATRIX_REF.to_owned(),
            work_item_context_ref: "work-item:status-transition-sheet:0006".to_owned(),
            fallback_modalities: vec![
                M5WorkItemComponentFallbackModality::List,
                M5WorkItemComponentFallbackModality::Textual,
                M5WorkItemComponentFallbackModality::Cli,
            ],
            reaches_canonical_truth: true,
            keyboard_reach: WorkItemComponentNonVisualReachState::ReachableAndLabeled,
            screen_reader_reach: WorkItemComponentNonVisualReachState::ReachableAndLabeled,
            cli_reach: WorkItemComponentNonVisualReachState::ReachableAndLabeled,
            export_summary: WorkItemComponentExportSummaryState::ReconstructableWithoutScreenshot,
            export_summary_ref: "summary:status-transition-sheet:a11y".to_owned(),
            copy_export: copy_export(&[
                "side_effect_preview",
                "permission_scope",
                "write_scope",
                "keyboard_route",
            ]),
            full_provider_claim: M5WorkItemComponentClaim::ProviderCommitted,
            claim_conditions: vec![condition(
                M5WorkItemComponentClaimDimension::WriteScope,
                M5WorkItemComponentConditionState::FreshCommitted,
            )],
            claim_narrow: None,
            lineage_preserved: true,
            rendering_surfaces: rendering_surfaces(),
            narrowing_disclosures: parity_surfaces(&[
                "side_effect_preview",
                "permission_scope",
                "write_scope",
            ]),
            required_labels: all_required_labels(),
            consumer_surfaces: base_consumers(&[
                M5WorkItemConsumerSurface::TransitionSheetUi,
                M5WorkItemConsumerSurface::DetailUi,
            ]),
            source_refs: vec![
                "UI/UX Spec §13.8 status-transition review".to_owned(),
                WORK_ITEM_COMPONENT_A11Y_DOC_REF.to_owned(),
            ],
            observed_at: "2026-07-09T00:00:00Z".to_owned(),
            evidence_refs: ev("status-transition-sheet"),
        },
        // Related-evidence card — evidence provenance is fresh and stated and the card is a
        // self-sufficient, reviewable read-only projection (not itself a committed write),
        // reachable on every surface (green).
        WorkItemComponentAccessibilityRow {
            record_kind: WORK_ITEM_COMPONENT_A11Y_ROW_RECORD_KIND.to_owned(),
            schema_version: WORK_ITEM_COMPONENT_A11Y_SCHEMA_VERSION,
            row_id: "a11y:related-evidence-card".to_owned(),
            component_family: M5WorkItemComponentFamily::RelatedEvidenceCard,
            source_family_schema_ref: WORK_ITEM_COMPONENT_A11Y_COMPONENT_MATRIX_REF.to_owned(),
            work_item_context_ref: "work-item:related-evidence-card:0007".to_owned(),
            fallback_modalities: vec![
                M5WorkItemComponentFallbackModality::List,
                M5WorkItemComponentFallbackModality::Textual,
                M5WorkItemComponentFallbackModality::Cli,
            ],
            reaches_canonical_truth: true,
            keyboard_reach: WorkItemComponentNonVisualReachState::ReachableAndLabeled,
            screen_reader_reach: WorkItemComponentNonVisualReachState::ReachableAndLabeled,
            cli_reach: WorkItemComponentNonVisualReachState::ReachableAndLabeled,
            export_summary: WorkItemComponentExportSummaryState::ReconstructableWithoutScreenshot,
            export_summary_ref: "summary:related-evidence-card:a11y".to_owned(),
            copy_export: copy_export(&[
                "evidence_provenance",
                "linked_engineering_context",
                "evidence_freshness",
                "keyboard_route",
            ]),
            full_provider_claim: M5WorkItemComponentClaim::ReviewableProjection,
            claim_conditions: vec![condition(
                M5WorkItemComponentClaimDimension::ProviderFreshness,
                M5WorkItemComponentConditionState::FreshCommitted,
            )],
            claim_narrow: None,
            lineage_preserved: true,
            rendering_surfaces: rendering_surfaces(),
            narrowing_disclosures: parity_surfaces(&[
                "evidence_provenance",
                "linked_engineering_context",
                "evidence_freshness",
            ]),
            required_labels: all_required_labels(),
            consumer_surfaces: base_consumers(&[
                M5WorkItemConsumerSurface::EvidencePanelUi,
                M5WorkItemConsumerSurface::DetailUi,
            ]),
            source_refs: vec![
                "UX Design System §16.72 related-evidence cards".to_owned(),
                WORK_ITEM_COMPONENT_A11Y_DOC_REF.to_owned(),
            ],
            observed_at: "2026-07-09T00:00:00Z".to_owned(),
            evidence_refs: ev("related-evidence-card"),
        },
        // Offline-handoff-packet card — hierarchy-heavy (nested packet / evidence / queued-draft
        // lineage); the packet cannot publish safely, so the card auto-narrows to an
        // unpublishable-packet projection and binds its nested lineage to a flat list / textual
        // path (yellow).
        WorkItemComponentAccessibilityRow {
            record_kind: WORK_ITEM_COMPONENT_A11Y_ROW_RECORD_KIND.to_owned(),
            schema_version: WORK_ITEM_COMPONENT_A11Y_SCHEMA_VERSION,
            row_id: "a11y:offline-handoff-packet-card-unpublishable".to_owned(),
            component_family: M5WorkItemComponentFamily::OfflineHandoffPacketCard,
            source_family_schema_ref: WORK_ITEM_COMPONENT_A11Y_COMPONENT_MATRIX_REF.to_owned(),
            work_item_context_ref: "work-item:offline-handoff-packet-card:0008".to_owned(),
            fallback_modalities: vec![
                M5WorkItemComponentFallbackModality::Structured,
                M5WorkItemComponentFallbackModality::List,
                M5WorkItemComponentFallbackModality::Textual,
                M5WorkItemComponentFallbackModality::Cli,
            ],
            reaches_canonical_truth: true,
            keyboard_reach: WorkItemComponentNonVisualReachState::ReachableAndLabeled,
            screen_reader_reach: WorkItemComponentNonVisualReachState::DisclosedReducedButReachable,
            cli_reach: WorkItemComponentNonVisualReachState::ReachableAndLabeled,
            export_summary: WorkItemComponentExportSummaryState::ReconstructableWithoutScreenshot,
            export_summary_ref: "summary:offline-handoff-packet-card-unpublishable:a11y".to_owned(),
            copy_export: copy_export(&[
                "handoff_destination",
                "export_boundary",
                "queued_draft_state",
                "publish_later_continuity",
            ]),
            full_provider_claim: M5WorkItemComponentClaim::ProviderCommitted,
            claim_conditions: vec![condition(
                M5WorkItemComponentClaimDimension::PacketPublishability,
                M5WorkItemComponentConditionState::PacketUnpublishable,
            )],
            claim_narrow: Some(WorkItemComponentClaimAutoNarrow {
                narrowed_to: M5WorkItemComponentClaim::UnpublishablePacketProjection,
                binding_dimension: M5WorkItemComponentClaimDimension::PacketPublishability,
                trigger: M5WorkItemDowngradeTrigger::PublishLaterContinuityHidden,
                narrowed_label:
                    "Captured handoff packet cannot publish safely and nothing has been handed off — shown as an unpublishable-packet projection with its destination, export boundary, and retry-or-export route preserved, never as a committed provider write"
                        .to_owned(),
                preserves_canonical_identity: true,
                preserves_lineage_continuity: true,
            }),
            lineage_preserved: true,
            rendering_surfaces: rendering_surfaces(),
            narrowing_disclosures: narrowed_surfaces(&[
                "handoff_destination",
                "export_boundary",
                "queued_draft_state",
            ]),
            required_labels: all_required_labels(),
            consumer_surfaces: base_consumers(&[
                M5WorkItemConsumerSurface::SyncStatusUi,
                M5WorkItemConsumerSurface::ProductUi,
            ]),
            source_refs: vec![
                "UX Design System §20.13 offline handoff packets".to_owned(),
                WORK_ITEM_COMPONENT_A11Y_DOC_REF.to_owned(),
            ],
            observed_at: "2026-07-09T00:00:00Z".to_owned(),
            evidence_refs: ev("offline-handoff-packet-card-unpublishable"),
        },
    ]
}
