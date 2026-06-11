//! Canonical M5 marketplace fact-views — result rows, detail fact grids, and
//! compare views that stay source-aware across every discovery flow.
//!
//! Where the
//! [`install-governance matrix`](crate::freeze_the_m5_ecosystem_install_lifecycle_state_and_activation_budget_matrix)
//! freezes one governance row per marketed M5 artifact family, this module freezes
//! how those families are *presented* in the marketplace and mirror/private-registry
//! discovery surfaces. It reuses the governance vocabulary — [`ArtifactFamily`],
//! [`RuntimeOrigin`], [`SupportClass`], [`LifecycleState`], and [`EvidenceFreshness`]
//! — instead of inventing separate view logic per pack family, and adds the
//! presentation-only vocabulary the storefront needs: [`SourceClass`],
//! [`BridgeNativeState`], [`MirrorPosture`], and [`DiscoveryChannel`].
//!
//! Three view shapes project the same fact set so a user, a support bundle, and a
//! release-evidence packet all cite identical truth:
//!
//! - a [`MarketplaceResultRow`] is the at-a-glance catalog row — package kind,
//!   source class, lifecycle state, support class, evidence freshness, runtime
//!   origin, bridge/native state, and mirror/private-registry posture;
//! - a [`MarketplaceDetailFactGrid`] is the per-listing detail page that pins the
//!   same facts plus the backing provenance, permission, compatibility, activation,
//!   rollback, and support-export refs; and
//! - a [`MarketplaceCompareView`] sets two or more listings side by side across the
//!   same axes.
//!
//! The presentation is honest by construction. The [`DisclosureLevel`] a row
//! publishes is **not** stored by hand: it is recomputed from the row's source
//! class, mirror posture, runtime origin, evidence freshness, support class, and
//! bridge/native state, and the stored level and [`DisclosureReason`] set must equal
//! that recomputation or validation fails. This enforces the lane guardrail —
//! reduced provenance *widens* disclosure and never collapses a fact: a mirrored,
//! private-registry, or manually-imported listing keeps every trust and
//! compatibility field its first-party sibling shows, and a side-loaded or
//! community listing surfaces a heightened warning rather than a thinner row.
//!
//! Every detail grid and every compare-view entry must reproduce its source row's
//! facts exactly, so a surface that renders one view can never drift from another,
//! and the same information architecture holds across the public registry, an
//! enterprise mirror, a private registry, and manual import even when some fields
//! are derived from local policy or mirror metadata.
//!
//! The packet is checked in at
//! `artifacts/ecosystem/m5/m5-marketplace-fact-views.json` and embedded here, so this
//! typed consumer and any CI gate agree on every row without a cargo build in CI.
//! The model is metadata-only: every field is a typed state or an opaque ref. It
//! carries no credential bodies, raw provider payloads, signing secrets, or mirror
//! tokens.

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

use crate::freeze_the_m5_ecosystem_install_lifecycle_state_and_activation_budget_matrix::{
    ArtifactFamily, EvidenceFreshness, LifecycleState, RuntimeOrigin, SupportClass,
};

/// Supported M5 marketplace fact-views schema version.
pub const M5_MARKETPLACE_FACT_VIEWS_SCHEMA_VERSION: u32 = 1;

/// Stable record-kind tag for the packet.
pub const M5_MARKETPLACE_FACT_VIEWS_RECORD_KIND: &str = "m5_marketplace_fact_views";

/// Repo-relative path to the checked-in packet.
pub const M5_MARKETPLACE_FACT_VIEWS_PATH: &str =
    "artifacts/ecosystem/m5/m5-marketplace-fact-views.json";

/// Embedded checked-in packet JSON.
pub const M5_MARKETPLACE_FACT_VIEWS_JSON: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../artifacts/ecosystem/m5/m5-marketplace-fact-views.json"
));

/// Publisher-trust origin of a marketplace listing.
///
/// Source class is distinct from [`RuntimeOrigin`] (how the artifact runs) and from
/// [`MirrorPosture`] (where it is served from): it names *who* stands behind the
/// listing. [`SourceClass::is_reduced_provenance`] marks the classes that widen a
/// row's [`DisclosureLevel`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SourceClass {
    /// Published and curated by the first party.
    FirstParty,
    /// Published by a verified partner.
    VerifiedPartner,
    /// Published by a community author.
    Community,
    /// Publisher is unverified (for example, a side-loaded artifact).
    Unverified,
}

impl SourceClass {
    /// Every source class, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::FirstParty,
        Self::VerifiedPartner,
        Self::Community,
        Self::Unverified,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::FirstParty => "first_party",
            Self::VerifiedPartner => "verified_partner",
            Self::Community => "community",
            Self::Unverified => "unverified",
        }
    }

    /// Whether this source class reduces provenance and widens disclosure.
    pub const fn is_reduced_provenance(self) -> bool {
        matches!(self, Self::Community | Self::Unverified)
    }
}

/// How a listing's runtime is backed.
///
/// Native artifacts run directly on the Aureline runtime; bridge-backed artifacts
/// run through a compatibility bridge, and local-model-hosted artifacts run on a
/// local-model host. Anything other than [`BridgeNativeState::Native`] widens
/// disclosure with [`DisclosureReason::NonNativeRuntime`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BridgeNativeState {
    /// Runs directly on the native runtime.
    Native,
    /// Runs through a compatibility bridge.
    BridgeBacked,
    /// Runs on a local-model host.
    LocalModelHosted,
}

impl BridgeNativeState {
    /// Every bridge/native state, in declaration order.
    pub const ALL: [Self; 3] = [Self::Native, Self::BridgeBacked, Self::LocalModelHosted];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Native => "native",
            Self::BridgeBacked => "bridge_backed",
            Self::LocalModelHosted => "local_model_hosted",
        }
    }

    /// Whether this state is anything other than a native runtime.
    pub const fn is_non_native(self) -> bool {
        !matches!(self, Self::Native)
    }
}

/// Distribution posture of a specific listing.
///
/// The posture captures how *this* copy is served, independent of the
/// [`DiscoveryChannel`] being browsed. A mirrored, private-registry, or
/// manually-imported posture widens disclosure but never collapses a fact field.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MirrorPosture {
    /// Served directly from the first-party storefront.
    DirectFirstParty,
    /// Re-served from an enterprise mirror.
    EnterpriseMirrored,
    /// Served from a private registry.
    PrivateRegistry,
    /// Side-loaded by manual import.
    ManuallyImported,
}

impl MirrorPosture {
    /// Every mirror posture, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::DirectFirstParty,
        Self::EnterpriseMirrored,
        Self::PrivateRegistry,
        Self::ManuallyImported,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::DirectFirstParty => "direct_first_party",
            Self::EnterpriseMirrored => "enterprise_mirrored",
            Self::PrivateRegistry => "private_registry",
            Self::ManuallyImported => "manually_imported",
        }
    }

    /// Whether this posture is a mirror or private-registry redistribution.
    pub const fn is_mirrored_or_private(self) -> bool {
        matches!(self, Self::EnterpriseMirrored | Self::PrivateRegistry)
    }

    /// Whether this posture is a side-loaded manual import.
    pub const fn is_manual_import(self) -> bool {
        matches!(self, Self::ManuallyImported)
    }
}

/// The discovery flow a row is browsed in.
///
/// The same information architecture holds across every channel; the channel is
/// recorded so support and release surfaces can prove a row was discoverable the
/// same way in each flow.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DiscoveryChannel {
    /// The public registry storefront.
    PublicRegistry,
    /// An enterprise mirror.
    EnterpriseMirror,
    /// A private registry.
    PrivateRegistry,
    /// Manual import.
    ManualImport,
}

impl DiscoveryChannel {
    /// Every discovery channel, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::PublicRegistry,
        Self::EnterpriseMirror,
        Self::PrivateRegistry,
        Self::ManualImport,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::PublicRegistry => "public_registry",
            Self::EnterpriseMirror => "enterprise_mirror",
            Self::PrivateRegistry => "private_registry",
            Self::ManualImport => "manual_import",
        }
    }
}

/// A reason a row widens its disclosure above the standard level.
///
/// Each reason is recomputed from the row's observed facts; the row's stored
/// [`MarketplaceResultRow::disclosure_reasons`] must equal the recomputed set.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DisclosureReason {
    /// The source class or runtime origin reduces provenance, or the listing was
    /// manually imported.
    ReducedProvenance,
    /// The listing is served from an enterprise mirror or a private registry.
    MirroredOrPrivateDistribution,
    /// The qualifying evidence is not current.
    EvidenceNotCurrent,
    /// The published support class is below full support.
    SupportNarrowed,
    /// The runtime is bridge-backed or local-model-hosted rather than native.
    NonNativeRuntime,
}

impl DisclosureReason {
    /// Every disclosure reason, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::ReducedProvenance,
        Self::MirroredOrPrivateDistribution,
        Self::EvidenceNotCurrent,
        Self::SupportNarrowed,
        Self::NonNativeRuntime,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ReducedProvenance => "reduced_provenance",
            Self::MirroredOrPrivateDistribution => "mirrored_or_private_distribution",
            Self::EvidenceNotCurrent => "evidence_not_current",
            Self::SupportNarrowed => "support_narrowed",
            Self::NonNativeRuntime => "non_native_runtime",
        }
    }

    /// The minimum disclosure level this reason forces.
    pub const fn min_disclosure_level(self) -> DisclosureLevel {
        match self {
            // Reduced provenance is the lane's headline trust gap and forces the
            // widest disclosure.
            Self::ReducedProvenance => DisclosureLevel::Heightened,
            Self::MirroredOrPrivateDistribution
            | Self::EvidenceNotCurrent
            | Self::SupportNarrowed
            | Self::NonNativeRuntime => DisclosureLevel::Caution,
        }
    }
}

/// The disclosure level a row publishes.
///
/// Ordered low-to-high by [`DisclosureLevel::rank`]: a [`DisclosureLevel::Standard`]
/// row carries the default disclosure, and a [`DisclosureLevel::Heightened`] row
/// carries the widest warning.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DisclosureLevel {
    /// Default disclosure; no widening reason applies.
    Standard,
    /// Widened disclosure; at least one caution-class reason applies.
    Caution,
    /// Widest disclosure; a reduced-provenance reason applies.
    Heightened,
}

impl DisclosureLevel {
    /// Every disclosure level, in declaration order.
    pub const ALL: [Self; 3] = [Self::Standard, Self::Caution, Self::Heightened];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Standard => "standard",
            Self::Caution => "caution",
            Self::Heightened => "heightened",
        }
    }

    /// Monotonic rank; higher means a wider disclosure.
    pub const fn rank(self) -> u8 {
        match self {
            Self::Standard => 0,
            Self::Caution => 1,
            Self::Heightened => 2,
        }
    }

    /// The wider (higher-rank) of two disclosure levels.
    pub const fn widen(self, other: Self) -> Self {
        if other.rank() > self.rank() {
            other
        } else {
            self
        }
    }
}

/// The cross-view fact set every view shape must reproduce identically.
///
/// Used by [`MarketplaceResultRow`], [`MarketplaceDetailFactGrid`], and
/// [`MarketplaceCompareEntry`] to prove no view drifts from another and that no
/// mirror/private/manual posture collapses a field.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MarketplaceFactSet {
    /// Package kind / marketed artifact family.
    pub package_kind: ArtifactFamily,
    /// Publisher-trust origin.
    pub source_class: SourceClass,
    /// Lifecycle state.
    pub lifecycle_state: LifecycleState,
    /// Published support class.
    pub support_class: SupportClass,
    /// Evidence freshness.
    pub evidence_freshness: EvidenceFreshness,
    /// Runtime origin.
    pub runtime_origin: RuntimeOrigin,
    /// Bridge/native state.
    pub bridge_native_state: BridgeNativeState,
    /// Distribution posture.
    pub mirror_posture: MirrorPosture,
    /// Discovery channel.
    pub discovery_channel: DiscoveryChannel,
    /// Recomputed disclosure level.
    pub disclosure_level: DisclosureLevel,
}

impl MarketplaceFactSet {
    /// The disclosure reasons recomputed from this fact set, in canonical order.
    pub fn computed_disclosure_reasons(&self) -> Vec<DisclosureReason> {
        let mut reasons = Vec::new();
        if self.source_class.is_reduced_provenance()
            || self.runtime_origin == RuntimeOrigin::UnsignedSideLoaded
            || self.mirror_posture.is_manual_import()
        {
            reasons.push(DisclosureReason::ReducedProvenance);
        }
        if self.mirror_posture.is_mirrored_or_private() {
            reasons.push(DisclosureReason::MirroredOrPrivateDistribution);
        }
        if !self.evidence_freshness.is_current() {
            reasons.push(DisclosureReason::EvidenceNotCurrent);
        }
        if self.support_class != SupportClass::FullySupported {
            reasons.push(DisclosureReason::SupportNarrowed);
        }
        if self.bridge_native_state.is_non_native() {
            reasons.push(DisclosureReason::NonNativeRuntime);
        }
        reasons
    }

    /// The disclosure level recomputed from this fact set.
    pub fn computed_disclosure_level(&self) -> DisclosureLevel {
        self.computed_disclosure_reasons()
            .into_iter()
            .fold(DisclosureLevel::Standard, |level, reason| {
                level.widen(reason.min_disclosure_level())
            })
    }
}

/// An at-a-glance marketplace result row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MarketplaceResultRow {
    /// Stable result-row id.
    pub row_id: String,
    /// Opaque ref to the underlying catalog listing.
    pub listing_ref: String,
    /// Human-readable listing label.
    pub display_label: String,
    /// Ref to the governance-matrix family this listing resolves through.
    pub governance_family_ref: String,
    /// Package kind / marketed artifact family.
    pub package_kind: ArtifactFamily,
    /// Publisher-trust origin.
    pub source_class: SourceClass,
    /// Lifecycle state.
    pub lifecycle_state: LifecycleState,
    /// Published support class.
    pub support_class: SupportClass,
    /// Evidence freshness.
    pub evidence_freshness: EvidenceFreshness,
    /// Runtime origin.
    pub runtime_origin: RuntimeOrigin,
    /// Bridge/native state.
    pub bridge_native_state: BridgeNativeState,
    /// Distribution posture.
    pub mirror_posture: MirrorPosture,
    /// Discovery channel.
    pub discovery_channel: DiscoveryChannel,
    /// Disclosure level; must equal the recomputed level.
    pub disclosure_level: DisclosureLevel,
    /// Disclosure reasons; must equal the recomputed set.
    #[serde(default)]
    pub disclosure_reasons: Vec<DisclosureReason>,
    /// Reviewer-facing summary.
    pub summary: String,
}

impl MarketplaceResultRow {
    /// The cross-view fact set this row exposes.
    pub fn fact_set(&self) -> MarketplaceFactSet {
        MarketplaceFactSet {
            package_kind: self.package_kind,
            source_class: self.source_class,
            lifecycle_state: self.lifecycle_state,
            support_class: self.support_class,
            evidence_freshness: self.evidence_freshness,
            runtime_origin: self.runtime_origin,
            bridge_native_state: self.bridge_native_state,
            mirror_posture: self.mirror_posture,
            discovery_channel: self.discovery_channel,
            disclosure_level: self.disclosure_level,
        }
    }

    /// The disclosure reasons recomputed from this row's facts.
    pub fn computed_disclosure_reasons(&self) -> Vec<DisclosureReason> {
        self.fact_set().computed_disclosure_reasons()
    }

    /// The disclosure level recomputed from this row's facts.
    pub fn computed_disclosure_level(&self) -> DisclosureLevel {
        self.fact_set().computed_disclosure_level()
    }

    /// Whether the row's stored disclosure level and reasons agree with the
    /// recomputed values.
    pub fn disclosure_consistent(&self) -> bool {
        self.disclosure_level == self.computed_disclosure_level()
            && self.disclosure_reasons == self.computed_disclosure_reasons()
    }

    /// Whether the row carries a widened (non-standard) disclosure.
    pub fn requires_widened_disclosure(&self) -> bool {
        self.disclosure_level != DisclosureLevel::Standard
    }
}

/// A per-listing detail fact grid.
///
/// The grid pins the same facts as its source [`MarketplaceResultRow`] plus the
/// backing provenance, permission, compatibility, activation, rollback, and
/// support-export refs, so a detail page never collapses a field — including for a
/// mirror, private-registry, or manually-imported listing.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MarketplaceDetailFactGrid {
    /// Stable grid id.
    pub grid_id: String,
    /// Result row this grid details.
    pub row_ref: String,
    /// Human-readable listing label.
    pub display_label: String,
    /// Package kind / marketed artifact family.
    pub package_kind: ArtifactFamily,
    /// Publisher-trust origin.
    pub source_class: SourceClass,
    /// Lifecycle state.
    pub lifecycle_state: LifecycleState,
    /// Published support class.
    pub support_class: SupportClass,
    /// Evidence freshness.
    pub evidence_freshness: EvidenceFreshness,
    /// Runtime origin.
    pub runtime_origin: RuntimeOrigin,
    /// Bridge/native state.
    pub bridge_native_state: BridgeNativeState,
    /// Distribution posture.
    pub mirror_posture: MirrorPosture,
    /// Discovery channel.
    pub discovery_channel: DiscoveryChannel,
    /// Disclosure level; must equal the source row's.
    pub disclosure_level: DisclosureLevel,
    /// Disclosure reasons; must equal the source row's.
    #[serde(default)]
    pub disclosure_reasons: Vec<DisclosureReason>,
    /// Ref to the listing's provenance/signature record.
    pub provenance_ref: String,
    /// Ref to the listing's permission manifest.
    pub permission_manifest_ref: String,
    /// Ref to the listing's compatibility/downgrade story.
    pub compatibility_ref: String,
    /// Ref to the listing's activation-budget record.
    pub activation_budget_ref: String,
    /// Ref to the listing's durable rollback path.
    pub rollback_ref: String,
    /// Ref binding this grid into support and release surfaces.
    pub support_export_ref: String,
    /// Additional source refs backing the grid.
    #[serde(default)]
    pub evidence_refs: Vec<String>,
    /// Reviewer-facing summary.
    pub summary: String,
}

impl MarketplaceDetailFactGrid {
    /// The cross-view fact set this grid exposes.
    pub fn fact_set(&self) -> MarketplaceFactSet {
        MarketplaceFactSet {
            package_kind: self.package_kind,
            source_class: self.source_class,
            lifecycle_state: self.lifecycle_state,
            support_class: self.support_class,
            evidence_freshness: self.evidence_freshness,
            runtime_origin: self.runtime_origin,
            bridge_native_state: self.bridge_native_state,
            mirror_posture: self.mirror_posture,
            discovery_channel: self.discovery_channel,
            disclosure_level: self.disclosure_level,
        }
    }

    /// Whether the grid carries its own non-empty backing refs.
    ///
    /// A mirror, private-registry, or manually-imported listing must still carry
    /// every backing ref a first-party listing carries; reduced provenance widens
    /// disclosure but must never drop evidence.
    pub fn has_required_refs(&self) -> bool {
        !self.provenance_ref.trim().is_empty()
            && !self.permission_manifest_ref.trim().is_empty()
            && !self.compatibility_ref.trim().is_empty()
            && !self.activation_budget_ref.trim().is_empty()
            && !self.rollback_ref.trim().is_empty()
            && !self.support_export_ref.trim().is_empty()
    }
}

/// One entry in a [`MarketplaceCompareView`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MarketplaceCompareEntry {
    /// Result row this entry compares.
    pub row_ref: String,
    /// Human-readable listing label.
    pub display_label: String,
    /// Package kind / marketed artifact family.
    pub package_kind: ArtifactFamily,
    /// Publisher-trust origin.
    pub source_class: SourceClass,
    /// Lifecycle state.
    pub lifecycle_state: LifecycleState,
    /// Published support class.
    pub support_class: SupportClass,
    /// Evidence freshness.
    pub evidence_freshness: EvidenceFreshness,
    /// Runtime origin.
    pub runtime_origin: RuntimeOrigin,
    /// Bridge/native state.
    pub bridge_native_state: BridgeNativeState,
    /// Distribution posture.
    pub mirror_posture: MirrorPosture,
    /// Discovery channel.
    pub discovery_channel: DiscoveryChannel,
    /// Disclosure level; must equal the source row's.
    pub disclosure_level: DisclosureLevel,
    /// Disclosure reasons; must equal the source row's.
    #[serde(default)]
    pub disclosure_reasons: Vec<DisclosureReason>,
}

impl MarketplaceCompareEntry {
    /// The cross-view fact set this entry exposes.
    pub fn fact_set(&self) -> MarketplaceFactSet {
        MarketplaceFactSet {
            package_kind: self.package_kind,
            source_class: self.source_class,
            lifecycle_state: self.lifecycle_state,
            support_class: self.support_class,
            evidence_freshness: self.evidence_freshness,
            runtime_origin: self.runtime_origin,
            bridge_native_state: self.bridge_native_state,
            mirror_posture: self.mirror_posture,
            discovery_channel: self.discovery_channel,
            disclosure_level: self.disclosure_level,
        }
    }
}

/// A side-by-side compare view across two or more listings.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MarketplaceCompareView {
    /// Stable compare-view id.
    pub compare_id: String,
    /// Human-readable compare-view title.
    pub title: String,
    /// Result rows compared, in display order.
    pub compared_row_refs: Vec<String>,
    /// Per-listing comparison entries.
    pub entries: Vec<MarketplaceCompareEntry>,
    /// Reviewer-facing summary.
    pub summary: String,
}

impl MarketplaceCompareView {
    /// Whether the entries cover exactly the compared rows, in order.
    pub fn entries_cover_compared_rows(&self) -> bool {
        self.entries.len() == self.compared_row_refs.len()
            && self
                .entries
                .iter()
                .zip(&self.compared_row_refs)
                .all(|(entry, row_ref)| &entry.row_ref == row_ref)
    }
}

/// Summary counts carried by the packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct M5MarketplaceFactViewsSummary {
    /// Total result rows.
    pub total_result_rows: usize,
    /// Total detail grids.
    pub total_detail_grids: usize,
    /// Total compare views.
    pub total_compare_views: usize,
    /// Rows at standard disclosure.
    pub standard_disclosure_rows: usize,
    /// Rows at caution disclosure.
    pub caution_disclosure_rows: usize,
    /// Rows at heightened disclosure.
    pub heightened_disclosure_rows: usize,
    /// Rows that carry a widened (non-standard) disclosure.
    pub widened_disclosure_rows: usize,
    /// Rows served from a mirror or private registry.
    pub mirrored_or_private_rows: usize,
    /// Rows side-loaded by manual import.
    pub manual_import_rows: usize,
    /// Rows whose runtime is non-native.
    pub non_native_runtime_rows: usize,
    /// Rows whose evidence is not current.
    pub not_current_evidence_rows: usize,
    /// Rows whose support is narrowed below full support.
    pub narrowed_support_rows: usize,
    /// Distinct package kinds across rows.
    pub distinct_package_kinds: usize,
    /// Distinct discovery channels across rows.
    pub distinct_discovery_channels: usize,
}

/// A redaction-safe export row projected from a result row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5MarketplaceFactViewsExportRow {
    /// Result-row id.
    pub row_id: String,
    /// Package-kind token.
    pub package_kind: String,
    /// Source-class token.
    pub source_class: String,
    /// Lifecycle-state token.
    pub lifecycle_state: String,
    /// Support-class token.
    pub support_class: String,
    /// Evidence-freshness token.
    pub evidence_freshness: String,
    /// Runtime-origin token.
    pub runtime_origin: String,
    /// Bridge/native-state token.
    pub bridge_native_state: String,
    /// Mirror-posture token.
    pub mirror_posture: String,
    /// Discovery-channel token.
    pub discovery_channel: String,
    /// Disclosure-level token.
    pub disclosure_level: String,
    /// Disclosure-reason tokens.
    pub disclosure_reasons: Vec<String>,
    /// Governance-matrix family ref.
    pub governance_family_ref: String,
    /// Whether the row carries a widened disclosure.
    pub widened_disclosure: bool,
    /// Human-readable summary.
    pub summary: String,
}

/// A redaction-safe export projection of the packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5MarketplaceFactViewsExportProjection {
    /// Packet id this projection was produced from.
    pub packet_id: String,
    /// Packet as-of date.
    pub as_of: String,
    /// Projected rows.
    pub rows: Vec<M5MarketplaceFactViewsExportRow>,
    /// Whether every view (row, grid, compare entry) is mutually consistent.
    pub all_views_consistent: bool,
    /// Rows that carry a widened disclosure.
    pub widened_disclosure_count: usize,
    /// Rows served from a mirror or private registry.
    pub mirrored_or_private_count: usize,
    /// Rows side-loaded by manual import.
    pub manual_import_count: usize,
}

/// The typed M5 marketplace fact-views packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct M5MarketplaceFactViews {
    /// Packet schema version.
    pub schema_version: u32,
    /// Record-kind discriminator.
    pub record_kind: String,
    /// Stable packet identifier.
    pub packet_id: String,
    /// Lifecycle status of this packet.
    pub status: String,
    /// Human-readable companion document.
    pub overview_page: String,
    /// UTC date this snapshot is current as of.
    pub as_of: String,
    /// Closed package-kind vocabulary (reused from the governance matrix).
    pub package_kinds: Vec<ArtifactFamily>,
    /// Closed source-class vocabulary.
    pub source_classes: Vec<SourceClass>,
    /// Closed support-class vocabulary (reused from the governance matrix).
    pub support_classes: Vec<SupportClass>,
    /// Closed runtime-origin vocabulary (reused from the governance matrix).
    pub runtime_origins: Vec<RuntimeOrigin>,
    /// Closed lifecycle-state vocabulary (reused from the governance matrix).
    pub lifecycle_states: Vec<LifecycleState>,
    /// Closed evidence-freshness vocabulary (reused from the governance matrix).
    pub evidence_freshness_classes: Vec<EvidenceFreshness>,
    /// Closed bridge/native-state vocabulary.
    pub bridge_native_states: Vec<BridgeNativeState>,
    /// Closed mirror-posture vocabulary.
    pub mirror_postures: Vec<MirrorPosture>,
    /// Closed discovery-channel vocabulary.
    pub discovery_channels: Vec<DiscoveryChannel>,
    /// Closed disclosure-level vocabulary.
    pub disclosure_levels: Vec<DisclosureLevel>,
    /// Closed disclosure-reason vocabulary.
    pub disclosure_reasons: Vec<DisclosureReason>,
    /// At-a-glance result rows.
    #[serde(default)]
    pub result_rows: Vec<MarketplaceResultRow>,
    /// Per-listing detail fact grids.
    #[serde(default)]
    pub detail_grids: Vec<MarketplaceDetailFactGrid>,
    /// Side-by-side compare views.
    #[serde(default)]
    pub compare_views: Vec<MarketplaceCompareView>,
    /// Summary counts.
    pub summary: M5MarketplaceFactViewsSummary,
}

impl M5MarketplaceFactViews {
    /// Returns the result row with the given id.
    pub fn result_row(&self, row_id: &str) -> Option<&MarketplaceResultRow> {
        self.result_rows.iter().find(|r| r.row_id == row_id)
    }

    /// Result rows discovered in the given channel.
    pub fn rows_in_channel(
        &self,
        channel: DiscoveryChannel,
    ) -> impl Iterator<Item = &MarketplaceResultRow> {
        self.result_rows
            .iter()
            .filter(move |r| r.discovery_channel == channel)
    }

    /// Result rows that carry a widened disclosure.
    pub fn widened_disclosure_rows(&self) -> impl Iterator<Item = &MarketplaceResultRow> {
        self.result_rows
            .iter()
            .filter(|r| r.requires_widened_disclosure())
    }

    /// Whether every row's stored disclosure agrees with the recomputed disclosure,
    /// every detail grid matches its row, and every compare entry matches its row.
    pub fn all_views_consistent(&self) -> bool {
        if !self.result_rows.iter().all(|r| r.disclosure_consistent()) {
            return false;
        }
        for grid in &self.detail_grids {
            match self.result_row(&grid.row_ref) {
                Some(row) => {
                    if grid.fact_set() != row.fact_set()
                        || grid.disclosure_reasons != row.disclosure_reasons
                    {
                        return false;
                    }
                }
                None => return false,
            }
        }
        for view in &self.compare_views {
            for entry in &view.entries {
                match self.result_row(&entry.row_ref) {
                    Some(row) => {
                        if entry.fact_set() != row.fact_set()
                            || entry.disclosure_reasons != row.disclosure_reasons
                        {
                            return false;
                        }
                    }
                    None => return false,
                }
            }
        }
        true
    }

    /// Recomputes the summary block from the rows, grids, and compare views.
    pub fn computed_summary(&self) -> M5MarketplaceFactViewsSummary {
        let count_level = |level: DisclosureLevel| {
            self.result_rows
                .iter()
                .filter(|r| r.disclosure_level == level)
                .count()
        };
        let package_kinds: BTreeSet<ArtifactFamily> =
            self.result_rows.iter().map(|r| r.package_kind).collect();
        let channels: BTreeSet<DiscoveryChannel> = self
            .result_rows
            .iter()
            .map(|r| r.discovery_channel)
            .collect();
        M5MarketplaceFactViewsSummary {
            total_result_rows: self.result_rows.len(),
            total_detail_grids: self.detail_grids.len(),
            total_compare_views: self.compare_views.len(),
            standard_disclosure_rows: count_level(DisclosureLevel::Standard),
            caution_disclosure_rows: count_level(DisclosureLevel::Caution),
            heightened_disclosure_rows: count_level(DisclosureLevel::Heightened),
            widened_disclosure_rows: self.widened_disclosure_rows().count(),
            mirrored_or_private_rows: self
                .result_rows
                .iter()
                .filter(|r| r.mirror_posture.is_mirrored_or_private())
                .count(),
            manual_import_rows: self
                .result_rows
                .iter()
                .filter(|r| r.mirror_posture.is_manual_import())
                .count(),
            non_native_runtime_rows: self
                .result_rows
                .iter()
                .filter(|r| r.bridge_native_state.is_non_native())
                .count(),
            not_current_evidence_rows: self
                .result_rows
                .iter()
                .filter(|r| !r.evidence_freshness.is_current())
                .count(),
            narrowed_support_rows: self
                .result_rows
                .iter()
                .filter(|r| r.support_class != SupportClass::FullySupported)
                .count(),
            distinct_package_kinds: package_kinds.len(),
            distinct_discovery_channels: channels.len(),
        }
    }

    /// Produces an export projection that downstream surfaces — support exports,
    /// docs/help, and release/public-truth packets — render instead of restating
    /// marketplace source-class, runtime, and disclosure status text by hand.
    pub fn export_projection(&self) -> M5MarketplaceFactViewsExportProjection {
        let rows = self
            .result_rows
            .iter()
            .map(|r| M5MarketplaceFactViewsExportRow {
                row_id: r.row_id.clone(),
                package_kind: r.package_kind.as_str().to_owned(),
                source_class: r.source_class.as_str().to_owned(),
                lifecycle_state: r.lifecycle_state.as_str().to_owned(),
                support_class: r.support_class.as_str().to_owned(),
                evidence_freshness: r.evidence_freshness.as_str().to_owned(),
                runtime_origin: r.runtime_origin.as_str().to_owned(),
                bridge_native_state: r.bridge_native_state.as_str().to_owned(),
                mirror_posture: r.mirror_posture.as_str().to_owned(),
                discovery_channel: r.discovery_channel.as_str().to_owned(),
                disclosure_level: r.disclosure_level.as_str().to_owned(),
                disclosure_reasons: r
                    .disclosure_reasons
                    .iter()
                    .map(|reason| reason.as_str().to_owned())
                    .collect(),
                governance_family_ref: r.governance_family_ref.clone(),
                widened_disclosure: r.requires_widened_disclosure(),
                summary: format!(
                    "{}: source {}, runtime {}, backing {}, posture {}, channel {}, support {}, freshness {}, disclosure {}",
                    r.package_kind.as_str(),
                    r.source_class.as_str(),
                    r.runtime_origin.as_str(),
                    r.bridge_native_state.as_str(),
                    r.mirror_posture.as_str(),
                    r.discovery_channel.as_str(),
                    r.support_class.as_str(),
                    r.evidence_freshness.as_str(),
                    r.disclosure_level.as_str(),
                ),
            })
            .collect();
        M5MarketplaceFactViewsExportProjection {
            packet_id: self.packet_id.clone(),
            as_of: self.as_of.clone(),
            rows,
            all_views_consistent: self.all_views_consistent(),
            widened_disclosure_count: self.widened_disclosure_rows().count(),
            mirrored_or_private_count: self
                .result_rows
                .iter()
                .filter(|r| r.mirror_posture.is_mirrored_or_private())
                .count(),
            manual_import_count: self
                .result_rows
                .iter()
                .filter(|r| r.mirror_posture.is_manual_import())
                .count(),
        }
    }

    /// Validates the packet, returning every violation found.
    pub fn validate(&self) -> Vec<M5MarketplaceFactViewsViolation> {
        let mut violations = Vec::new();
        self.validate_envelope(&mut violations);

        let mut seen_rows = BTreeSet::new();
        for row in &self.result_rows {
            if !seen_rows.insert(row.row_id.clone()) {
                violations.push(M5MarketplaceFactViewsViolation::DuplicateRowId {
                    row_id: row.row_id.clone(),
                });
            }
            self.validate_row(row, &mut violations);
        }

        let mut seen_grids = BTreeSet::new();
        for grid in &self.detail_grids {
            if !seen_grids.insert(grid.grid_id.clone()) {
                violations.push(M5MarketplaceFactViewsViolation::DuplicateGridId {
                    grid_id: grid.grid_id.clone(),
                });
            }
            self.validate_grid(grid, &mut violations);
        }

        let mut seen_views = BTreeSet::new();
        for view in &self.compare_views {
            if !seen_views.insert(view.compare_id.clone()) {
                violations.push(M5MarketplaceFactViewsViolation::DuplicateCompareId {
                    compare_id: view.compare_id.clone(),
                });
            }
            self.validate_compare_view(view, &mut violations);
        }

        if self.summary != self.computed_summary() {
            violations.push(M5MarketplaceFactViewsViolation::SummaryMismatch);
        }

        violations
    }

    fn validate_envelope(&self, violations: &mut Vec<M5MarketplaceFactViewsViolation>) {
        if self.schema_version != M5_MARKETPLACE_FACT_VIEWS_SCHEMA_VERSION {
            violations.push(M5MarketplaceFactViewsViolation::UnsupportedSchemaVersion {
                actual: self.schema_version,
            });
        }
        if self.record_kind != M5_MARKETPLACE_FACT_VIEWS_RECORD_KIND {
            violations.push(M5MarketplaceFactViewsViolation::UnsupportedRecordKind {
                actual: self.record_kind.clone(),
            });
        }
        for (field, value) in [
            ("packet_id", &self.packet_id),
            ("status", &self.status),
            ("overview_page", &self.overview_page),
            ("as_of", &self.as_of),
        ] {
            if value.trim().is_empty() {
                violations.push(M5MarketplaceFactViewsViolation::EmptyField {
                    id: "<packet>".to_owned(),
                    field_name: field,
                });
            }
        }
        for (field, ok) in [
            (
                "package_kinds",
                self.package_kinds == ArtifactFamily::ALL.to_vec(),
            ),
            (
                "source_classes",
                self.source_classes == SourceClass::ALL.to_vec(),
            ),
            (
                "support_classes",
                self.support_classes == SupportClass::ALL.to_vec(),
            ),
            (
                "runtime_origins",
                self.runtime_origins == RuntimeOrigin::ALL.to_vec(),
            ),
            (
                "lifecycle_states",
                self.lifecycle_states == LifecycleState::ALL.to_vec(),
            ),
            (
                "evidence_freshness_classes",
                self.evidence_freshness_classes == EvidenceFreshness::ALL.to_vec(),
            ),
            (
                "bridge_native_states",
                self.bridge_native_states == BridgeNativeState::ALL.to_vec(),
            ),
            (
                "mirror_postures",
                self.mirror_postures == MirrorPosture::ALL.to_vec(),
            ),
            (
                "discovery_channels",
                self.discovery_channels == DiscoveryChannel::ALL.to_vec(),
            ),
            (
                "disclosure_levels",
                self.disclosure_levels == DisclosureLevel::ALL.to_vec(),
            ),
            (
                "disclosure_reasons",
                self.disclosure_reasons == DisclosureReason::ALL.to_vec(),
            ),
        ] {
            if !ok {
                violations
                    .push(M5MarketplaceFactViewsViolation::ClosedVocabularyMismatch { field });
            }
        }
    }

    fn validate_row(
        &self,
        row: &MarketplaceResultRow,
        violations: &mut Vec<M5MarketplaceFactViewsViolation>,
    ) {
        for (field, value) in [
            ("row_id", &row.row_id),
            ("listing_ref", &row.listing_ref),
            ("display_label", &row.display_label),
            ("governance_family_ref", &row.governance_family_ref),
            ("summary", &row.summary),
        ] {
            if value.trim().is_empty() {
                violations.push(M5MarketplaceFactViewsViolation::EmptyField {
                    id: row.row_id.clone(),
                    field_name: field,
                });
            }
        }

        let mut seen_reasons = BTreeSet::new();
        for reason in &row.disclosure_reasons {
            if !seen_reasons.insert(*reason) {
                violations.push(M5MarketplaceFactViewsViolation::DuplicateDisclosureReason {
                    id: row.row_id.clone(),
                    reason: reason.as_str(),
                });
            }
        }

        // The published disclosure level must equal the recomputed level, so a
        // mirrored, private, side-loaded, stale, or non-native listing can never
        // present a narrower warning than its facts warrant.
        let computed_level = row.computed_disclosure_level();
        if row.disclosure_level != computed_level {
            violations.push(M5MarketplaceFactViewsViolation::DisclosureLevelMismatch {
                id: row.row_id.clone(),
                stored: row.disclosure_level.as_str(),
                computed: computed_level.as_str(),
            });
        }

        // The recorded reasons must equal the recomputed reasons, so a widening can
        // never be asserted or hidden by hand.
        if row.disclosure_reasons != row.computed_disclosure_reasons() {
            violations.push(M5MarketplaceFactViewsViolation::DisclosureReasonsMismatch {
                id: row.row_id.clone(),
            });
        }
    }

    fn validate_grid(
        &self,
        grid: &MarketplaceDetailFactGrid,
        violations: &mut Vec<M5MarketplaceFactViewsViolation>,
    ) {
        for (field, value) in [
            ("grid_id", &grid.grid_id),
            ("row_ref", &grid.row_ref),
            ("display_label", &grid.display_label),
            ("provenance_ref", &grid.provenance_ref),
            ("permission_manifest_ref", &grid.permission_manifest_ref),
            ("compatibility_ref", &grid.compatibility_ref),
            ("activation_budget_ref", &grid.activation_budget_ref),
            ("rollback_ref", &grid.rollback_ref),
            ("support_export_ref", &grid.support_export_ref),
            ("summary", &grid.summary),
        ] {
            if value.trim().is_empty() {
                violations.push(M5MarketplaceFactViewsViolation::EmptyField {
                    id: grid.grid_id.clone(),
                    field_name: field,
                });
            }
        }

        match self.result_row(&grid.row_ref) {
            Some(row) => {
                // A detail grid must reproduce its row's facts exactly — including for
                // a mirror/private/manual listing — so a detail page never collapses a
                // field a result row showed.
                if grid.fact_set() != row.fact_set()
                    || grid.disclosure_reasons != row.disclosure_reasons
                {
                    violations.push(M5MarketplaceFactViewsViolation::GridDriftsFromRow {
                        grid_id: grid.grid_id.clone(),
                        row_ref: grid.row_ref.clone(),
                    });
                }
            }
            None => violations.push(M5MarketplaceFactViewsViolation::DanglingRowRef {
                id: grid.grid_id.clone(),
                row_ref: grid.row_ref.clone(),
            }),
        }
    }

    fn validate_compare_view(
        &self,
        view: &MarketplaceCompareView,
        violations: &mut Vec<M5MarketplaceFactViewsViolation>,
    ) {
        for (field, value) in [
            ("compare_id", &view.compare_id),
            ("title", &view.title),
            ("summary", &view.summary),
        ] {
            if value.trim().is_empty() {
                violations.push(M5MarketplaceFactViewsViolation::EmptyField {
                    id: view.compare_id.clone(),
                    field_name: field,
                });
            }
        }

        if view.compared_row_refs.len() < 2 {
            violations.push(M5MarketplaceFactViewsViolation::CompareViewTooSmall {
                compare_id: view.compare_id.clone(),
            });
        }

        if !view.entries_cover_compared_rows() {
            violations.push(M5MarketplaceFactViewsViolation::CompareEntriesMismatch {
                compare_id: view.compare_id.clone(),
            });
        }

        for entry in &view.entries {
            match self.result_row(&entry.row_ref) {
                Some(row) => {
                    if entry.fact_set() != row.fact_set()
                        || entry.disclosure_reasons != row.disclosure_reasons
                    {
                        violations.push(
                            M5MarketplaceFactViewsViolation::CompareEntryDriftsFromRow {
                                compare_id: view.compare_id.clone(),
                                row_ref: entry.row_ref.clone(),
                            },
                        );
                    }
                }
                None => violations.push(M5MarketplaceFactViewsViolation::DanglingRowRef {
                    id: view.compare_id.clone(),
                    row_ref: entry.row_ref.clone(),
                }),
            }
        }
    }
}

/// A validation violation for the M5 marketplace fact-views packet.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum M5MarketplaceFactViewsViolation {
    /// The packet carries an unsupported schema version.
    UnsupportedSchemaVersion {
        /// Version found in the packet.
        actual: u32,
    },
    /// The packet carries an unsupported record kind.
    UnsupportedRecordKind {
        /// Record kind found in the packet.
        actual: String,
    },
    /// A closed vocabulary or pinned value is not canonical.
    ClosedVocabularyMismatch {
        /// Offending field.
        field: &'static str,
    },
    /// A required field is empty.
    EmptyField {
        /// Row, grid, view, or packet id.
        id: String,
        /// Field name.
        field_name: &'static str,
    },
    /// A result-row id appears more than once.
    DuplicateRowId {
        /// Duplicate row id.
        row_id: String,
    },
    /// A detail-grid id appears more than once.
    DuplicateGridId {
        /// Duplicate grid id.
        grid_id: String,
    },
    /// A compare-view id appears more than once.
    DuplicateCompareId {
        /// Duplicate compare-view id.
        compare_id: String,
    },
    /// A row lists a disclosure reason more than once.
    DuplicateDisclosureReason {
        /// Row id.
        id: String,
        /// Reason token.
        reason: &'static str,
    },
    /// A row's stored disclosure level disagrees with the recomputed level.
    DisclosureLevelMismatch {
        /// Row id.
        id: String,
        /// Stored level token.
        stored: &'static str,
        /// Recomputed level token.
        computed: &'static str,
    },
    /// A row's disclosure reasons disagree with the recomputed reasons.
    DisclosureReasonsMismatch {
        /// Row id.
        id: String,
    },
    /// A grid or compare entry references a row that does not exist.
    DanglingRowRef {
        /// Grid or compare-view id.
        id: String,
        /// Missing row ref.
        row_ref: String,
    },
    /// A detail grid's facts disagree with its source row.
    GridDriftsFromRow {
        /// Grid id.
        grid_id: String,
        /// Source row ref.
        row_ref: String,
    },
    /// A compare view references fewer than two rows.
    CompareViewTooSmall {
        /// Compare-view id.
        compare_id: String,
    },
    /// A compare view's entries do not cover its compared rows in order.
    CompareEntriesMismatch {
        /// Compare-view id.
        compare_id: String,
    },
    /// A compare entry's facts disagree with its source row.
    CompareEntryDriftsFromRow {
        /// Compare-view id.
        compare_id: String,
        /// Source row ref.
        row_ref: String,
    },
    /// The summary counts disagree with the rows, grids, and compare views.
    SummaryMismatch,
}

impl fmt::Display for M5MarketplaceFactViewsViolation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnsupportedSchemaVersion { actual } => {
                write!(f, "unsupported packet schema_version {actual}")
            }
            Self::UnsupportedRecordKind { actual } => {
                write!(f, "unsupported packet record_kind {actual}")
            }
            Self::ClosedVocabularyMismatch { field } => {
                write!(f, "packet {field} is not the canonical value")
            }
            Self::EmptyField { id, field_name } => {
                write!(f, "{id} has empty field {field_name}")
            }
            Self::DuplicateRowId { row_id } => write!(f, "duplicate result row id {row_id}"),
            Self::DuplicateGridId { grid_id } => write!(f, "duplicate detail grid id {grid_id}"),
            Self::DuplicateCompareId { compare_id } => {
                write!(f, "duplicate compare view id {compare_id}")
            }
            Self::DuplicateDisclosureReason { id, reason } => {
                write!(f, "row {id} repeats disclosure reason {reason}")
            }
            Self::DisclosureLevelMismatch {
                id,
                stored,
                computed,
            } => {
                write!(
                    f,
                    "row {id} publishes disclosure level {stored} but the recomputed level is {computed}"
                )
            }
            Self::DisclosureReasonsMismatch { id } => {
                write!(
                    f,
                    "row {id} disclosure reasons disagree with the recomputed set"
                )
            }
            Self::DanglingRowRef { id, row_ref } => {
                write!(f, "{id} references missing result row {row_ref}")
            }
            Self::GridDriftsFromRow { grid_id, row_ref } => {
                write!(f, "detail grid {grid_id} drifts from its row {row_ref}")
            }
            Self::CompareViewTooSmall { compare_id } => {
                write!(
                    f,
                    "compare view {compare_id} references fewer than two rows"
                )
            }
            Self::CompareEntriesMismatch { compare_id } => {
                write!(
                    f,
                    "compare view {compare_id} entries do not cover its compared rows in order"
                )
            }
            Self::CompareEntryDriftsFromRow {
                compare_id,
                row_ref,
            } => {
                write!(
                    f,
                    "compare view {compare_id} entry drifts from its row {row_ref}"
                )
            }
            Self::SummaryMismatch => write!(f, "packet summary counts disagree with the views"),
        }
    }
}

impl Error for M5MarketplaceFactViewsViolation {}

/// Loads the embedded M5 marketplace fact-views packet.
///
/// # Errors
///
/// Returns a JSON parse error when the checked-in packet no longer matches
/// [`M5MarketplaceFactViews`].
pub fn current_m5_marketplace_fact_views() -> Result<M5MarketplaceFactViews, serde_json::Error> {
    serde_json::from_str(M5_MARKETPLACE_FACT_VIEWS_JSON)
}

#[cfg(test)]
mod tests;
