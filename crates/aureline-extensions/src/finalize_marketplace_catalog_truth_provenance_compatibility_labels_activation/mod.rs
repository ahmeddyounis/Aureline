//! Finalize marketplace catalog truth for the stable line — provenance,
//! compatibility labels, activation budgets, support class, and discoverability
//! posture in one conformance-backed, mirrorable, automatically-narrowing packet.
//!
//! The beta-level [`crate::marketplace_truth`] module owns the per-row projection
//! that joins a catalog descriptor to the current generated compatibility report.
//! This module owns the layer above it — the **stable, evidence-backed catalog
//! truth** a claimed stable marketplace row carries, and the **stability
//! qualification** that truth is allowed to claim.
//!
//! A stable catalog-truth row must bind, machine-readably:
//!
//! - the **identity** (catalog descriptor ref, package identity, the pinned
//!   catalog-truth version, the publisher trust tier, and the lifecycle state),
//! - the **provenance** posture (`verified_publisher` / `official_pack` /
//!   `enterprise_approved` / `community` / `under_review`) mechanically sourced
//!   from one registry/status model rather than from badge text copied into
//!   multiple views,
//! - the **surface boundary** (runtime class, host boundary, hosted-surface and
//!   browser-handoff implications, and any reduced accessibility or theming
//!   parity) so a package verified on one runtime cannot imply parity everywhere,
//! - the **discoverability posture** (`ranked_normally`, `penalized_*`, or
//!   `quarantined`) kept *separate* from provenance and support-class truth and
//!   explainable without raw install count or operator notes,
//! - the **machine-readable compatibility scorecards** for the top imported
//!   extensions, bridge classes, workflow bundles, and certified reference
//!   workspaces — each exposing a parity band, a freshness window, an evidence
//!   source, and a downgrade state — so catalog truth never collapses into
//!   ratings or install counts and never inherits parity from an adjacent claim,
//! - the **activation-budget** instrumentation (so an unbounded activation cost
//!   can never ride a stable claim),
//! - the **support class** (with whether it is profile-limited and which runtime
//!   profile it was verified on),
//! - the **publisher-continuity** binding, and
//! - the **view alignment** that keeps the compatibility labels, support class,
//!   provenance, activation budget, and scorecard links aligned across the public
//!   registry, approved mirror, and side-load views.
//!
//! The central rule mirrors the rest of the stable line: a **stable** catalog-truth
//! claim may never be implied from a catalog row alone. A row that renders a
//! `stable` badge must pin the published catalog-truth version, be evidence-backed
//! (not catalog-asserted), keep its publisher trust tier out of quarantine, stay on
//! an installable lifecycle, keep its provenance out of `under_review`, be ranked
//! normally (not penalized or quarantined), carry a verified runtime class, disclose
//! any hosted-surface or browser-handoff boundary, ship at least one compatibility
//! scorecard with no inherited / unsupported / stale / parity-limited / not-yet-run
//! scorecard, keep its activation cost bounded and within budget, keep a stable-grade
//! and non-profile-limited support class, keep its publisher continuity current, keep
//! its truth aligned across views, and be fully attributed. When any of those fails,
//! the visible tier is **automatically narrowed below Stable** (`beta`, `preview`, or
//! `withdrawn`) with machine-readable reasons rather than left asserting a catalog
//! readiness the evidence cannot back.
//!
//! Three guardrails are encoded so they cannot be papered over:
//!
//! - **No catalog-only trust.** A `catalog_asserted_only` claim basis can never back
//!   a stable catalog-truth claim; it narrows below Stable.
//! - **No unbounded activation cost.** An `unbounded` activation budget withdraws the
//!   row outright; an `over_budget` budget narrows to `beta`.
//! - **No parity inheritance.** A scorecard whose evidence source is
//!   `inherited_from_adjacent` can never back a stable claim; it narrows below
//!   Stable, so discoverability cannot inherit parity from an adjacent bridge or
//!   bundle claim.
//!
//! The companion schema lives at
//! [`schemas/extensions/stable_marketplace_catalog_truth.schema.json`](../../../../schemas/extensions/stable_marketplace_catalog_truth.schema.json).
//! Canonical fixtures live under
//! `fixtures/extensions/m4/finalize-marketplace-catalog-truth-provenance-compatibility-labels-activation/`.

use std::collections::BTreeSet;
use std::fmt;

use serde::{Deserialize, Serialize};

#[cfg(test)]
mod tests;

// ---------------------------------------------------------------------------
// Constants
// ---------------------------------------------------------------------------

/// Schema version for every stable marketplace catalog-truth record.
pub const STABLE_MARKETPLACE_CATALOG_TRUTH_SCHEMA_VERSION: u32 = 1;

/// The published, stable catalog-truth version. A `stable` claim must pin exactly
/// this version; any other version narrows below Stable.
pub const STABLE_MARKETPLACE_CATALOG_PUBLISHED_VERSION: u32 = 1;

/// Canonical schema path the packet cites.
pub const STABLE_MARKETPLACE_CATALOG_TRUTH_SCHEMA_REF: &str =
    "schemas/extensions/stable_marketplace_catalog_truth.schema.json";

/// Record-kind tag for [`StableMarketplaceCatalogTruthPacket`].
pub const STABLE_MARKETPLACE_CATALOG_TRUTH_PACKET_RECORD_KIND: &str =
    "stable_marketplace_catalog_truth_packet";

/// Record-kind tag for [`MarketplaceCatalogTruthIdentity`].
pub const MARKETPLACE_CATALOG_TRUTH_IDENTITY_RECORD_KIND: &str =
    "stable_marketplace_catalog_truth_identity";

/// Record-kind tag for [`MarketplaceCatalogProvenance`].
pub const MARKETPLACE_CATALOG_PROVENANCE_RECORD_KIND: &str =
    "stable_marketplace_catalog_provenance";

/// Record-kind tag for [`MarketplaceCatalogSurfaceBoundary`].
pub const MARKETPLACE_CATALOG_SURFACE_BOUNDARY_RECORD_KIND: &str =
    "stable_marketplace_catalog_surface_boundary";

/// Record-kind tag for [`MarketplaceCatalogDiscoverabilityPosture`].
pub const MARKETPLACE_CATALOG_DISCOVERABILITY_POSTURE_RECORD_KIND: &str =
    "stable_marketplace_catalog_discoverability_posture";

/// Record-kind tag for [`MarketplaceCompatibilityScorecard`].
pub const MARKETPLACE_COMPATIBILITY_SCORECARD_RECORD_KIND: &str =
    "stable_marketplace_catalog_compatibility_scorecard";

/// Record-kind tag for [`MarketplaceCompatibilitySummary`].
pub const MARKETPLACE_COMPATIBILITY_SUMMARY_RECORD_KIND: &str =
    "stable_marketplace_catalog_compatibility_summary";

/// Record-kind tag for [`MarketplaceCatalogActivationBudget`].
pub const MARKETPLACE_CATALOG_ACTIVATION_BUDGET_RECORD_KIND: &str =
    "stable_marketplace_catalog_activation_budget";

/// Record-kind tag for [`MarketplaceCatalogSupportClass`].
pub const MARKETPLACE_CATALOG_SUPPORT_CLASS_RECORD_KIND: &str =
    "stable_marketplace_catalog_support_class";

/// Record-kind tag for [`MarketplaceCatalogPublisherContinuity`].
pub const MARKETPLACE_CATALOG_PUBLISHER_CONTINUITY_RECORD_KIND: &str =
    "stable_marketplace_catalog_publisher_continuity";

/// Record-kind tag for [`MarketplaceCatalogViewAlignment`].
pub const MARKETPLACE_CATALOG_VIEW_ALIGNMENT_RECORD_KIND: &str =
    "stable_marketplace_catalog_view_alignment";

/// Record-kind tag for [`MarketplaceCatalogTruthQualificationClaim`].
pub const MARKETPLACE_CATALOG_TRUTH_QUALIFICATION_CLAIM_RECORD_KIND: &str =
    "stable_marketplace_catalog_truth_qualification_claim";

/// Record-kind tag for [`DowngradedCatalogBanner`].
pub const DOWNGRADED_CATALOG_BANNER_RECORD_KIND: &str = "stable_marketplace_catalog_downgraded_banner";

/// Record-kind tag for [`StableMarketplaceCatalogTruthInspection`].
pub const STABLE_MARKETPLACE_CATALOG_TRUTH_INSPECTION_RECORD_KIND: &str =
    "stable_marketplace_catalog_truth_inspection";

/// Record-kind tag for [`StableMarketplaceCatalogTruthSupportExport`].
pub const STABLE_MARKETPLACE_CATALOG_TRUTH_SUPPORT_EXPORT_RECORD_KIND: &str =
    "stable_marketplace_catalog_truth_support_export";

/// Closed publisher-trust-tier vocabulary.
pub const TRUST_TIER_CLASSES: &[&str] = &[
    "verified_publisher",
    "known_publisher",
    "community_unverified",
    "quarantined",
];

/// Closed lifecycle-state vocabulary mirrored from the extension lifecycle lane.
pub const LIFECYCLE_STATE_CLASSES: &[&str] = &[
    "installed",
    "pending_activation",
    "active",
    "recovered",
    "degraded",
    "disabled",
    "quarantined",
    "removed",
    "publisher_blocked",
];

/// Lifecycle states a stable catalog-truth claim may keep (installable / runnable).
pub const INSTALLABLE_LIFECYCLE_STATES: &[&str] =
    &["installed", "pending_activation", "active", "recovered"];

/// Closed provenance vocabulary, mechanically sourced from one registry/status model.
pub const PROVENANCE_CLASSES: &[&str] = &[
    "verified_publisher",
    "official_pack",
    "enterprise_approved",
    "community",
    "under_review",
];

/// Closed runtime-class vocabulary, shared with the stable runtime ABI lane.
pub const RUNTIME_CLASSES: &[&str] = &[
    "passive_package",
    "wasm_capability_sandbox",
    "declarative_host_rendered_view",
    "external_host",
    "compatibility_bridge",
    "remote_side_component",
];

/// Closed host-boundary vocabulary.
pub const HOST_BOUNDARY_CLASSES: &[&str] = &[
    "in_process_passive",
    "wasm_sandbox",
    "supervised_external_process",
    "host_rendered_surface",
    "hosted_remote_surface",
    "browser_handoff",
];

/// Host boundaries that require an explicit hosted-surface / browser-handoff
/// disclosure before a stable claim can hold.
pub const SURFACE_DISCLOSURE_BOUNDARIES: &[&str] =
    &["host_rendered_surface", "hosted_remote_surface", "browser_handoff"];

/// Closed discoverability ranking-state vocabulary, kept separate from provenance
/// and support-class truth.
pub const RANKING_STATE_CLASSES: &[&str] = &[
    "ranked_normally",
    "penalized_staleness",
    "penalized_performance",
    "penalized_trust",
    "quarantined",
];

/// Ranking states that penalize prominence without quarantining the row.
pub const PENALIZED_RANKING_STATES: &[&str] =
    &["penalized_staleness", "penalized_performance", "penalized_trust"];

/// Closed compatibility-scorecard subject vocabulary.
pub const SCORECARD_SUBJECT_CLASSES: &[&str] = &[
    "imported_extension",
    "bridge_class",
    "workflow_bundle",
    "certified_reference_workspace",
];

/// Closed parity-band vocabulary.
pub const PARITY_BAND_CLASSES: &[&str] = &[
    "full_parity",
    "high_parity",
    "partial_parity",
    "limited_parity",
    "unsupported",
];

/// Closed freshness-window vocabulary. `not_evaluated` means evidence has not run.
pub const FRESHNESS_WINDOW_CLASSES: &[&str] =
    &["current", "aging", "stale", "expired", "not_evaluated"];

/// Closed evidence-source vocabulary. `inherited_from_adjacent` may never back stable.
pub const EVIDENCE_SOURCE_CLASSES: &[&str] = &[
    "conformance_suite",
    "certified_workspace",
    "bridge_matrix",
    "vendor_attested",
    "inherited_from_adjacent",
];

/// Closed scorecard-downgrade-state vocabulary.
pub const SCORECARD_DOWNGRADE_STATE_CLASSES: &[&str] =
    &["none", "narrowed", "downgraded", "unsupported"];

/// Closed activation-budget vocabulary. `within_budget` is the only state a stable
/// claim may keep.
pub const ACTIVATION_BUDGET_CLASSES: &[&str] =
    &["within_budget", "over_budget", "unbounded", "not_measured"];

/// Closed support-class vocabulary surfaced as a catalog fact.
pub const SUPPORT_CLASS_CLASSES: &[&str] = &[
    "certified",
    "supported",
    "limited",
    "community",
    "experimental",
    "unsupported",
];

/// Support classes that may keep a stable claim (subject to the profile-limited flag).
pub const STABLE_GRADE_SUPPORT_CLASSES: &[&str] = &["certified", "supported", "community"];

/// Closed publisher-continuity vocabulary. `current` is the only state a stable claim
/// may keep.
pub const PUBLISHER_CONTINUITY_CLASSES: &[&str] = &["current", "stale", "missing", "revoked"];

/// Closed catalog-view vocabulary. A stable claim must keep its truth aligned across
/// all three.
pub const CATALOG_VIEW_CLASSES: &[&str] = &["public_registry", "approved_mirror", "side_load"];

/// Closed set of switch-readiness stability tiers.
pub const STABILITY_TIERS: &[&str] = &["stable", "beta", "preview", "withdrawn"];

/// Tiers that count as a *stable* catalog-truth claim.
pub const STABLE_TIERS: &[&str] = &["stable"];

/// Closed claim-basis vocabulary. `catalog_asserted_only` may never back stable.
pub const CLAIM_BASIS_CLASSES: &[&str] = &["evidence_backed", "catalog_asserted_only"];

/// Closed support-claim vocabulary a tier may imply.
pub const SUPPORT_CLAIM_CLASSES: &[&str] = &[
    "stable_catalog_truth_claim",
    "beta_catalog_truth_partial_claim",
    "preview_catalog_truth_experimental_claim",
    "withdrawn_no_catalog_truth_claim",
];

/// Closed set of reasons that narrow a stable catalog-truth claim below Stable.
pub const MARKETPLACE_CATALOG_DOWNGRADE_REASONS: &[&str] = &[
    "catalog_version_not_published",
    "catalog_only_trust_not_evidence_backed",
    "trust_tier_quarantined",
    "lifecycle_not_installable",
    "provenance_under_review",
    "quarantined_from_discovery",
    "ranking_penalized",
    "runtime_class_unverified",
    "hosted_surface_boundary_undisclosed",
    "missing_required_scorecard",
    "compatibility_unsupported",
    "compatibility_parity_limited",
    "scorecard_parity_inherited",
    "scorecard_evidence_not_run",
    "scorecard_freshness_stale",
    "activation_cost_unbounded",
    "activation_cost_over_budget",
    "activation_cost_not_measured",
    "support_class_below_stable_grade",
    "support_class_limited",
    "support_class_profile_limited",
    "publisher_continuity_revoked",
    "publisher_continuity_missing",
    "publisher_continuity_stale",
    "views_not_aligned",
    "attribution_incomplete",
];

/// Reasons that narrow all the way to `withdrawn` (the row cannot be trusted as
/// stable catalog truth at all).
const WITHDRAWN_CLASS_REASONS: &[&str] = &[
    "lifecycle_not_installable",
    "provenance_under_review",
    "quarantined_from_discovery",
    "runtime_class_unverified",
    "missing_required_scorecard",
    "compatibility_unsupported",
    "activation_cost_unbounded",
    "publisher_continuity_revoked",
];

/// Reasons that narrow to `preview` (a structural / trust / disclosure shortfall).
const PREVIEW_CLASS_REASONS: &[&str] = &[
    "catalog_version_not_published",
    "catalog_only_trust_not_evidence_backed",
    "trust_tier_quarantined",
    "hosted_surface_boundary_undisclosed",
    "scorecard_parity_inherited",
    "scorecard_evidence_not_run",
    "activation_cost_not_measured",
    "support_class_below_stable_grade",
    "publisher_continuity_missing",
    "views_not_aligned",
    "attribution_incomplete",
];

/// Reasons whose only effect is a safe narrowing to `beta`.
const BETA_CLASS_REASONS: &[&str] = &[
    "ranking_penalized",
    "compatibility_parity_limited",
    "scorecard_freshness_stale",
    "activation_cost_over_budget",
    "support_class_limited",
    "support_class_profile_limited",
    "publisher_continuity_stale",
];

/// Closed set of consumer surfaces that ingest this packet.
pub const STABLE_MARKETPLACE_CATALOG_CONSUMER_SURFACES: &[&str] = &[
    "marketplace_result_row",
    "marketplace_detail_page",
    "marketplace_compare_row",
    "install_review",
    "diagnostics",
    "support_export",
    "docs_help_surface",
    "release_packet",
    "cli_inspector",
    "mirror_packet",
];

// ---------------------------------------------------------------------------
// Input types
// ---------------------------------------------------------------------------

/// Input describing a stable marketplace catalog-truth packet to materialize.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StableMarketplaceCatalogTruthInput {
    /// Stable packet identity.
    pub packet_id: String,
    /// Timestamp used for deterministic fixture output.
    pub generated_at: String,
    /// Identity input.
    pub identity: MarketplaceCatalogTruthIdentityInput,
    /// Provenance input.
    pub provenance: MarketplaceCatalogProvenanceInput,
    /// Surface-boundary input.
    pub surface_boundary: MarketplaceCatalogSurfaceBoundaryInput,
    /// Discoverability-posture input.
    pub discoverability: MarketplaceCatalogDiscoverabilityPostureInput,
    /// Compatibility scorecards (imported extensions, bridge classes, bundles,
    /// certified reference workspaces).
    #[serde(default)]
    pub scorecards: Vec<MarketplaceCompatibilityScorecardInput>,
    /// Activation-budget input for the row's worst-case surface.
    pub activation_budget: MarketplaceCatalogActivationBudgetInput,
    /// Support-class input.
    pub support_class: MarketplaceCatalogSupportClassInput,
    /// Publisher-continuity input.
    pub publisher_continuity: MarketplaceCatalogPublisherContinuityInput,
    /// View-alignment input.
    pub view_alignment: MarketplaceCatalogViewAlignmentInput,
    /// Stability qualification claim input.
    pub claim: MarketplaceCatalogTruthQualificationClaimInput,
    /// Consumer surfaces bound to this packet.
    pub consumer_surfaces: Vec<String>,
    /// Reviewable summary safe for support/export surfaces.
    pub summary_label: String,
}

/// Input for [`MarketplaceCatalogTruthIdentity`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MarketplaceCatalogTruthIdentityInput {
    /// Ref to the catalog descriptor record this row stabilizes.
    pub catalog_descriptor_ref: String,
    /// Opaque marketplace row identity ref.
    pub row_identity_ref: String,
    /// Extension identity.
    pub extension_identity: String,
    /// Extension version.
    pub extension_version: String,
    /// Package id.
    pub package_id: String,
    /// Published catalog-truth version this row pins.
    pub catalog_version: u32,
    /// Source package the row came from.
    pub source_package_ref: String,
    /// Publisher trust tier.
    pub publisher_trust_tier_class: String,
    /// Current lifecycle state.
    pub lifecycle_state_class: String,
}

/// Input for [`MarketplaceCatalogProvenance`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MarketplaceCatalogProvenanceInput {
    /// Provenance class mechanically sourced from one registry/status model.
    pub provenance_class: String,
    /// Whether the provenance is mechanically sourced (not copied badge text).
    pub mechanically_sourced: bool,
    /// Ref to the registry/status record the provenance was sourced from.
    pub status_source_ref: String,
    /// Reviewable summary.
    pub summary_label: String,
}

/// Input for [`MarketplaceCatalogSurfaceBoundary`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MarketplaceCatalogSurfaceBoundaryInput {
    /// Runtime class for the package.
    pub runtime_class: String,
    /// Host boundary for the package.
    pub host_boundary_class: String,
    /// Whether the runtime class is verified for this package (not implied from a
    /// different runtime or deployment profile).
    pub runtime_class_verified: bool,
    /// Whether the package implies a hosted surface.
    pub hosted_surface_implication: bool,
    /// Whether the package implies a browser handoff.
    pub browser_handoff_implication: bool,
    /// Whether the hosted-surface / browser-handoff boundary is disclosed.
    pub surface_boundary_disclosed: bool,
    /// Whether the package has reduced accessibility parity.
    pub reduced_accessibility_parity: bool,
    /// Whether the package has reduced theming parity.
    pub reduced_theming_parity: bool,
    /// Reviewable summary.
    pub summary_label: String,
}

/// Input for [`MarketplaceCatalogDiscoverabilityPosture`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MarketplaceCatalogDiscoverabilityPostureInput {
    /// Ranking state, kept separate from provenance and support-class truth.
    pub ranking_state_class: String,
    /// Whether ranking can be explained without raw install count or operator notes.
    pub ranking_explained_without_install_count: bool,
    /// Ref to the ranking-rationale record.
    pub ranking_rationale_ref: String,
    /// Reviewable summary.
    pub summary_label: String,
}

/// Input for one [`MarketplaceCompatibilityScorecard`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MarketplaceCompatibilityScorecardInput {
    /// Stable scorecard id.
    pub scorecard_id: String,
    /// Scorecard subject class.
    pub subject_class: String,
    /// Parity band.
    pub parity_band_class: String,
    /// Freshness window.
    pub freshness_window_class: String,
    /// Evidence source.
    pub evidence_source_class: String,
    /// Scorecard downgrade state.
    pub downgrade_state_class: String,
    /// Opaque ref to the machine-readable scorecard.
    pub scorecard_ref: String,
    /// Reviewable summary.
    pub summary_label: String,
}

/// Input for [`MarketplaceCatalogActivationBudget`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MarketplaceCatalogActivationBudgetInput {
    /// Activation-budget posture for the worst-case surface.
    pub budget_class: String,
    /// Ref to the measured activation cost.
    pub measured_cost_ref: String,
    /// Ref to the declared activation-budget ceiling.
    pub budget_ceiling_ref: String,
    /// Number of surfaces whose activation cost was measured.
    pub measured_surface_count: usize,
    /// Reviewable summary.
    pub summary_label: String,
}

/// Input for [`MarketplaceCatalogSupportClass`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MarketplaceCatalogSupportClassInput {
    /// Support class surfaced as a catalog fact.
    pub support_class_class: String,
    /// Whether support is limited to a specific runtime/deployment profile.
    pub profile_limited: bool,
    /// Ref to the runtime/deployment profile the support class was verified on.
    pub verified_runtime_profile_ref: String,
    /// Ref to the support-class evidence.
    pub evidence_ref: String,
    /// Reviewable summary.
    pub summary_label: String,
}

/// Input for [`MarketplaceCatalogPublisherContinuity`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MarketplaceCatalogPublisherContinuityInput {
    /// Continuity state for the row publisher.
    pub continuity_state_class: String,
    /// Opaque ref to the publisher-continuity packet, when one is bound.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub continuity_packet_ref: Option<String>,
    /// Reviewable summary.
    pub summary_label: String,
}

/// Input for [`MarketplaceCatalogViewAlignment`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MarketplaceCatalogViewAlignmentInput {
    /// Views the catalog truth is asserted to remain aligned across.
    pub aligned_views: Vec<String>,
    /// Whether the runtime class is preserved across all views.
    pub runtime_class_preserved_across_views: bool,
    /// Reviewable summary.
    pub summary_label: String,
}

/// Input for [`MarketplaceCatalogTruthQualificationClaim`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MarketplaceCatalogTruthQualificationClaimInput {
    /// Catalog-truth tier claimed by the row.
    pub claimed_tier: String,
    /// Claim basis: evidence-backed vs catalog-asserted only.
    pub claim_basis_class: String,
    /// Reviewable summary.
    pub summary_label: String,
}

// ---------------------------------------------------------------------------
// Record types
// ---------------------------------------------------------------------------

/// Identity shared across every surface.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MarketplaceCatalogTruthIdentity {
    /// Record-kind tag.
    pub record_kind: String,
    /// Integer schema version.
    pub schema_version: u32,
    /// Ref to the catalog descriptor record this row stabilizes.
    pub catalog_descriptor_ref: String,
    /// Opaque marketplace row identity ref.
    pub row_identity_ref: String,
    /// Extension identity.
    pub extension_identity: String,
    /// Extension version.
    pub extension_version: String,
    /// Package id.
    pub package_id: String,
    /// Published catalog-truth version this row pins.
    pub catalog_version: u32,
    /// Source package the row came from.
    pub source_package_ref: String,
    /// Publisher trust tier.
    pub publisher_trust_tier_class: String,
    /// Current lifecycle state.
    pub lifecycle_state_class: String,
}

impl MarketplaceCatalogTruthIdentity {
    /// Returns true when the row pins the published stable catalog-truth version.
    pub fn catalog_version_current(&self) -> bool {
        self.catalog_version == STABLE_MARKETPLACE_CATALOG_PUBLISHED_VERSION
    }

    /// Returns true when the lifecycle is installable.
    pub fn lifecycle_installable(&self) -> bool {
        INSTALLABLE_LIFECYCLE_STATES.contains(&self.lifecycle_state_class.as_str())
    }
}

/// Provenance posture, mechanically sourced from one registry/status model.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MarketplaceCatalogProvenance {
    /// Record-kind tag.
    pub record_kind: String,
    /// Integer schema version.
    pub schema_version: u32,
    /// Provenance class.
    pub provenance_class: String,
    /// Whether the provenance is mechanically sourced (not copied badge text).
    pub mechanically_sourced: bool,
    /// Ref to the registry/status record the provenance was sourced from.
    pub status_source_ref: String,
    /// Reviewable summary.
    pub summary_label: String,
}

impl MarketplaceCatalogProvenance {
    /// Returns true when the provenance is in moderation / under review.
    pub fn under_review(&self) -> bool {
        self.provenance_class == "under_review"
    }
}

/// Surface boundary: runtime class, host boundary, hosted-surface / browser-handoff
/// implications, and reduced accessibility / theming parity.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MarketplaceCatalogSurfaceBoundary {
    /// Record-kind tag.
    pub record_kind: String,
    /// Integer schema version.
    pub schema_version: u32,
    /// Runtime class for the package.
    pub runtime_class: String,
    /// Host boundary for the package.
    pub host_boundary_class: String,
    /// Whether the runtime class is verified for this package.
    pub runtime_class_verified: bool,
    /// Whether the package implies a hosted surface.
    pub hosted_surface_implication: bool,
    /// Whether the package implies a browser handoff.
    pub browser_handoff_implication: bool,
    /// Whether the hosted-surface / browser-handoff boundary is disclosed.
    pub surface_boundary_disclosed: bool,
    /// Whether the package has reduced accessibility parity.
    pub reduced_accessibility_parity: bool,
    /// Whether the package has reduced theming parity.
    pub reduced_theming_parity: bool,
    /// Reviewable summary.
    pub summary_label: String,
}

impl MarketplaceCatalogSurfaceBoundary {
    /// Returns true when this boundary requires an explicit hosted-surface /
    /// browser-handoff disclosure before a stable claim can hold.
    pub fn requires_surface_disclosure(&self) -> bool {
        self.hosted_surface_implication
            || self.browser_handoff_implication
            || SURFACE_DISCLOSURE_BOUNDARIES.contains(&self.host_boundary_class.as_str())
    }

    /// Returns true when any required surface-boundary disclosure is satisfied.
    pub fn surface_disclosure_ok(&self) -> bool {
        !self.requires_surface_disclosure() || self.surface_boundary_disclosed
    }
}

/// Discoverability posture, kept separate from provenance and support-class truth.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MarketplaceCatalogDiscoverabilityPosture {
    /// Record-kind tag.
    pub record_kind: String,
    /// Integer schema version.
    pub schema_version: u32,
    /// Ranking state.
    pub ranking_state_class: String,
    /// Whether ranking is explainable without raw install count or operator notes.
    pub ranking_explained_without_install_count: bool,
    /// Ref to the ranking-rationale record.
    pub ranking_rationale_ref: String,
    /// Reviewable summary.
    pub summary_label: String,
}

impl MarketplaceCatalogDiscoverabilityPosture {
    /// Returns true when the row is quarantined from prominent discovery.
    pub fn quarantined(&self) -> bool {
        self.ranking_state_class == "quarantined"
    }

    /// Returns true when the row is penalized in ranking without being quarantined.
    pub fn penalized(&self) -> bool {
        PENALIZED_RANKING_STATES.contains(&self.ranking_state_class.as_str())
    }
}

/// One machine-readable compatibility scorecard.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MarketplaceCompatibilityScorecard {
    /// Record-kind tag.
    pub record_kind: String,
    /// Integer schema version.
    pub schema_version: u32,
    /// Stable scorecard id.
    pub scorecard_id: String,
    /// Scorecard subject class.
    pub subject_class: String,
    /// Parity band.
    pub parity_band_class: String,
    /// Freshness window.
    pub freshness_window_class: String,
    /// Evidence source.
    pub evidence_source_class: String,
    /// Scorecard downgrade state.
    pub downgrade_state_class: String,
    /// Opaque ref to the machine-readable scorecard.
    pub scorecard_ref: String,
    /// Reviewable summary.
    pub summary_label: String,
}

impl MarketplaceCompatibilityScorecard {
    /// Returns true when the scorecard inherits parity from an adjacent claim.
    pub fn parity_inherited(&self) -> bool {
        self.evidence_source_class == "inherited_from_adjacent"
    }

    /// Returns true when the scorecard's evidence has not been run.
    pub fn evidence_not_run(&self) -> bool {
        self.freshness_window_class == "not_evaluated"
    }

    /// Returns true when the scorecard's freshness window is stale or expired.
    pub fn stale(&self) -> bool {
        matches!(self.freshness_window_class.as_str(), "stale" | "expired")
    }

    /// Returns true when the scorecard reports an unsupported parity or downgrade.
    pub fn unsupported(&self) -> bool {
        self.parity_band_class == "unsupported" || self.downgrade_state_class == "unsupported"
    }

    /// Returns true when the scorecard reports a parity-limited or downgraded posture
    /// short of unsupported.
    pub fn parity_limited(&self) -> bool {
        matches!(self.parity_band_class.as_str(), "partial_parity" | "limited_parity")
            || matches!(self.downgrade_state_class.as_str(), "narrowed" | "downgraded")
    }
}

/// Aggregate compatibility summary across the scorecards.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MarketplaceCompatibilitySummary {
    /// Record-kind tag.
    pub record_kind: String,
    /// Integer schema version.
    pub schema_version: u32,
    /// Total number of scorecards.
    pub scorecard_count: usize,
    /// Number of scorecards that inherit parity from an adjacent claim.
    pub inherited_count: usize,
    /// Number of scorecards whose evidence has not run.
    pub not_evaluated_count: usize,
    /// Number of stale or expired scorecards.
    pub stale_count: usize,
    /// Number of unsupported scorecards.
    pub unsupported_count: usize,
    /// Number of parity-limited scorecards short of unsupported.
    pub parity_limited_count: usize,
    /// Subject classes present in the row (sorted, deduped).
    pub present_subjects: Vec<String>,
    /// True when at least one scorecard is present.
    pub has_scorecard: bool,
    /// True when no scorecard inherits parity from an adjacent claim.
    pub no_inherited_parity: bool,
    /// True when every scorecard is fresh, evaluated, and at least parity-limited
    /// rather than unsupported.
    pub all_scorecards_supportable: bool,
}

impl MarketplaceCompatibilitySummary {
    /// Returns true when the compatibility evidence can back a stable claim.
    pub fn stable_backable(&self) -> bool {
        self.has_scorecard
            && self.no_inherited_parity
            && self.not_evaluated_count == 0
            && self.unsupported_count == 0
    }
}

/// Activation-budget instrumentation for the row's worst-case surface.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MarketplaceCatalogActivationBudget {
    /// Record-kind tag.
    pub record_kind: String,
    /// Integer schema version.
    pub schema_version: u32,
    /// Activation-budget posture.
    pub budget_class: String,
    /// Ref to the measured activation cost.
    pub measured_cost_ref: String,
    /// Ref to the declared activation-budget ceiling.
    pub budget_ceiling_ref: String,
    /// Number of surfaces whose activation cost was measured.
    pub measured_surface_count: usize,
    /// Reviewable summary.
    pub summary_label: String,
}

impl MarketplaceCatalogActivationBudget {
    /// Returns true when the activation cost is bounded and within budget.
    pub fn within_budget(&self) -> bool {
        self.budget_class == "within_budget"
    }

    /// Returns true when the activation cost is unbounded.
    pub fn unbounded(&self) -> bool {
        self.budget_class == "unbounded"
    }
}

/// Support-class binding surfaced as a catalog fact.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MarketplaceCatalogSupportClass {
    /// Record-kind tag.
    pub record_kind: String,
    /// Integer schema version.
    pub schema_version: u32,
    /// Support class.
    pub support_class_class: String,
    /// Whether support is limited to a specific runtime/deployment profile.
    pub profile_limited: bool,
    /// Ref to the runtime/deployment profile the support class was verified on.
    pub verified_runtime_profile_ref: String,
    /// Ref to the support-class evidence.
    pub evidence_ref: String,
    /// Reviewable summary.
    pub summary_label: String,
}

impl MarketplaceCatalogSupportClass {
    /// Returns true when the support class is a stable-grade class.
    pub fn stable_grade(&self) -> bool {
        STABLE_GRADE_SUPPORT_CLASSES.contains(&self.support_class_class.as_str())
    }

    /// Returns true when the support class is the narrower `limited` grade.
    pub fn limited(&self) -> bool {
        self.support_class_class == "limited"
    }
}

/// Publisher-continuity binding for the catalog row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MarketplaceCatalogPublisherContinuity {
    /// Record-kind tag.
    pub record_kind: String,
    /// Integer schema version.
    pub schema_version: u32,
    /// Continuity state for the row publisher.
    pub continuity_state_class: String,
    /// Opaque ref to the publisher-continuity packet, when one is bound.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub continuity_packet_ref: Option<String>,
    /// Reviewable summary.
    pub summary_label: String,
}

impl MarketplaceCatalogPublisherContinuity {
    /// Returns true when the continuity is current.
    pub fn current(&self) -> bool {
        self.continuity_state_class == "current"
    }
}

/// View-alignment binding: keeps catalog truth aligned across views.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MarketplaceCatalogViewAlignment {
    /// Record-kind tag.
    pub record_kind: String,
    /// Integer schema version.
    pub schema_version: u32,
    /// Views the catalog truth is asserted to remain aligned across.
    pub aligned_views: Vec<String>,
    /// Whether the runtime class is preserved across all views.
    pub runtime_class_preserved_across_views: bool,
    /// Reviewable summary.
    pub summary_label: String,
}

impl MarketplaceCatalogViewAlignment {
    /// Returns true when the truth is aligned across every catalog view and the
    /// runtime class is preserved across views.
    pub fn all_views_aligned(&self) -> bool {
        self.runtime_class_preserved_across_views
            && CATALOG_VIEW_CLASSES
                .iter()
                .all(|v| self.aligned_views.iter().any(|a| a == v))
    }
}

/// Stability qualification claim after the posture is applied.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MarketplaceCatalogTruthQualificationClaim {
    /// Record-kind tag.
    pub record_kind: String,
    /// Integer schema version.
    pub schema_version: u32,
    /// Catalog-truth tier claimed by the row.
    pub claimed_tier: String,
    /// Effective tier after the posture is applied.
    pub effective_tier: String,
    /// Support claim the effective tier is allowed to imply.
    pub support_claim_class: String,
    /// Claim basis: evidence-backed vs catalog-asserted only.
    pub claim_basis_class: String,
    /// True when the claimed tier was narrowed below Stable.
    pub downgraded: bool,
    /// Reasons that narrowed the claim.
    pub downgrade_reasons: Vec<String>,
    /// Reviewable summary.
    pub summary_label: String,
}

/// Downgraded-row banner requirement. Raised whenever a reviewer must see a catalog
/// shortfall before relying on the row before install or enablement.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DowngradedCatalogBanner {
    /// Record-kind tag.
    pub record_kind: String,
    /// Integer schema version.
    pub schema_version: u32,
    /// True when a downgraded-row banner must be displayed.
    pub must_display: bool,
    /// Most-severe applicable banner reason, drawn from the downgrade vocabulary.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub banner_reason_class: Option<String>,
    /// Reviewable summary.
    pub summary_label: String,
}

/// Compact inspection row for CLI/headless and dashboard surfaces.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StableMarketplaceCatalogTruthInspection {
    /// Record-kind tag.
    pub record_kind: String,
    /// Integer schema version.
    pub schema_version: u32,
    /// Packet inspected by this row.
    pub packet_id_ref: String,
    /// Effective catalog-truth tier.
    pub effective_tier: String,
    /// True when the claim is a stable catalog-truth claim.
    pub stable_claim: bool,
    /// True when the row pins the published catalog-truth version.
    pub catalog_version_current: bool,
    /// Publisher trust tier.
    pub trust_tier_class: String,
    /// Provenance class.
    pub provenance_class: String,
    /// True when the lifecycle is installable.
    pub lifecycle_installable: bool,
    /// Ranking state.
    pub ranking_state_class: String,
    /// Runtime class.
    pub runtime_class: String,
    /// Host boundary class.
    pub host_boundary_class: String,
    /// True when the runtime class is verified.
    pub runtime_class_verified: bool,
    /// True when any required surface-boundary disclosure is satisfied.
    pub surface_boundary_disclosed: bool,
    /// Support class.
    pub support_class_class: String,
    /// True when support is profile-limited.
    pub support_profile_limited: bool,
    /// Activation-budget posture.
    pub activation_budget_class: String,
    /// True when the activation cost is bounded and within budget.
    pub activation_within_budget: bool,
    /// Publisher-continuity state.
    pub publisher_continuity_class: String,
    /// True when the compatibility evidence can back a stable claim.
    pub compatibility_stable_backable: bool,
    /// True when the catalog truth is aligned across all views.
    pub views_aligned: bool,
    /// True when the claimed tier was narrowed.
    pub downgraded: bool,
    /// True when a downgraded-row banner is required.
    pub downgraded_banner_required: bool,
    /// True when identity and every artifact are fully attributed.
    pub attribution_complete: bool,
    /// Number of compatibility scorecards.
    pub scorecard_count: usize,
    /// Number of scorecards inheriting parity from an adjacent claim.
    pub inherited_scorecard_count: usize,
    /// Number of unsupported scorecards.
    pub unsupported_scorecard_count: usize,
    /// Number of stale or expired scorecards.
    pub stale_scorecard_count: usize,
    /// Reviewable summary.
    pub summary_label: String,
}

// ---------------------------------------------------------------------------
// Packet type
// ---------------------------------------------------------------------------

/// Stable marketplace catalog-truth packet consumed by marketplace result, detail,
/// and compare rows, install review, diagnostics, support export, docs/help, release
/// packets, the CLI inspector, and mirror packets.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StableMarketplaceCatalogTruthPacket {
    /// Record-kind tag.
    pub record_kind: String,
    /// Integer schema version.
    pub schema_version: u32,
    /// Stable packet identity.
    pub packet_id: String,
    /// Timestamp used for deterministic fixture output.
    pub generated_at: String,
    /// Identity.
    pub identity: MarketplaceCatalogTruthIdentity,
    /// Provenance.
    pub provenance: MarketplaceCatalogProvenance,
    /// Surface boundary.
    pub surface_boundary: MarketplaceCatalogSurfaceBoundary,
    /// Discoverability posture.
    pub discoverability: MarketplaceCatalogDiscoverabilityPosture,
    /// Compatibility scorecards.
    pub scorecards: Vec<MarketplaceCompatibilityScorecard>,
    /// Aggregate compatibility summary.
    pub compatibility_summary: MarketplaceCompatibilitySummary,
    /// Activation-budget instrumentation.
    pub activation_budget: MarketplaceCatalogActivationBudget,
    /// Support class.
    pub support_class: MarketplaceCatalogSupportClass,
    /// Publisher-continuity binding.
    pub publisher_continuity: MarketplaceCatalogPublisherContinuity,
    /// View-alignment binding.
    pub view_alignment: MarketplaceCatalogViewAlignment,
    /// Stability qualification claim after the posture is applied.
    pub claim: MarketplaceCatalogTruthQualificationClaim,
    /// Downgraded-row banner requirement.
    pub downgraded_banner: DowngradedCatalogBanner,
    /// Consumer surfaces bound to this packet.
    pub consumer_surfaces: Vec<String>,
    /// Source schemas the packet cites.
    pub source_schema_refs: Vec<String>,
    /// False so a catalog row can never imply stable trust on its own.
    pub allows_catalog_only_trust: bool,
    /// False so an unbounded activation cost can never ride a stable row.
    pub allows_unbounded_activation_cost: bool,
    /// False so a scorecard can never inherit parity from an adjacent claim and ride
    /// a stable row.
    pub allows_inherited_parity_stable_claim: bool,
    /// False so discovery prominence can never imply support-class or provenance truth.
    pub allows_ranking_implied_trust: bool,
    /// Inspection row.
    pub inspection: StableMarketplaceCatalogTruthInspection,
}

impl StableMarketplaceCatalogTruthPacket {
    /// Builds a stable marketplace catalog-truth packet from input, deriving the
    /// aggregate compatibility summary and applying the catalog posture to the
    /// claimed tier so any required downgrade below Stable is automatic.
    ///
    /// # Errors
    ///
    /// Returns [`StableMarketplaceCatalogTruthValidationError`] when the input violates
    /// an identity, provenance, surface, scorecard, budget, support, continuity, view,
    /// or claim invariant.
    pub fn from_input(
        input: StableMarketplaceCatalogTruthInput,
    ) -> Result<Self, StableMarketplaceCatalogTruthValidationError> {
        validate_input(&input)?;

        let identity = identity_record(&input.identity);
        let provenance = provenance_record(&input.provenance);
        let surface_boundary = surface_boundary_record(&input.surface_boundary);
        let discoverability = discoverability_record(&input.discoverability);
        let scorecards: Vec<MarketplaceCompatibilityScorecard> =
            input.scorecards.iter().map(scorecard_record).collect();
        let compatibility_summary = summarize_compatibility(&scorecards);
        let activation_budget = activation_budget_record(&input.activation_budget);
        let support_class = support_class_record(&input.support_class);
        let publisher_continuity = publisher_continuity_record(&input.publisher_continuity);
        let view_alignment = view_alignment_record(&input.view_alignment);
        let attribution_complete = attribution_is_complete(&identity, &provenance, &scorecards);

        let posture = CatalogPosture {
            identity: &identity,
            provenance: &provenance,
            surface_boundary: &surface_boundary,
            discoverability: &discoverability,
            compatibility_summary: &compatibility_summary,
            activation_budget: &activation_budget,
            support_class: &support_class,
            publisher_continuity: &publisher_continuity,
            view_alignment: &view_alignment,
            attribution_complete,
        };

        let claim = claim_record(&input.claim, &posture);
        let downgraded_banner = banner_record(&posture);
        let inspection = inspection_record(&input.packet_id, &posture, &claim, &downgraded_banner);

        let packet = Self {
            record_kind: STABLE_MARKETPLACE_CATALOG_TRUTH_PACKET_RECORD_KIND.to_string(),
            schema_version: STABLE_MARKETPLACE_CATALOG_TRUTH_SCHEMA_VERSION,
            packet_id: input.packet_id,
            generated_at: input.generated_at,
            identity,
            provenance,
            surface_boundary,
            discoverability,
            scorecards,
            compatibility_summary,
            activation_budget,
            support_class,
            publisher_continuity,
            view_alignment,
            claim,
            downgraded_banner,
            consumer_surfaces: input.consumer_surfaces,
            source_schema_refs: vec![STABLE_MARKETPLACE_CATALOG_TRUTH_SCHEMA_REF.to_string()],
            allows_catalog_only_trust: false,
            allows_unbounded_activation_cost: false,
            allows_inherited_parity_stable_claim: false,
            allows_ranking_implied_trust: false,
            inspection,
        };
        packet.validate()?;
        Ok(packet)
    }

    /// Validates this packet against the stable catalog-truth invariants.
    ///
    /// # Errors
    ///
    /// Returns [`StableMarketplaceCatalogTruthValidationError`] when an invariant is
    /// violated.
    pub fn validate(&self) -> Result<(), StableMarketplaceCatalogTruthValidationError> {
        ensure_eq(
            self.record_kind.as_str(),
            STABLE_MARKETPLACE_CATALOG_TRUTH_PACKET_RECORD_KIND,
            "record_kind",
        )?;
        ensure_eq_u32(
            self.schema_version,
            STABLE_MARKETPLACE_CATALOG_TRUTH_SCHEMA_VERSION,
            "schema_version",
        )?;
        ensure_nonempty(&self.packet_id, "packet_id")?;
        ensure_nonempty(&self.generated_at, "generated_at")?;

        validate_identity(&self.identity)?;
        validate_provenance(&self.provenance)?;
        validate_surface_boundary(&self.surface_boundary)?;
        validate_discoverability(&self.discoverability)?;
        if self.scorecards.is_empty() {
            return Err(err("packet must carry at least one compatibility scorecard"));
        }
        let mut scorecard_ids = BTreeSet::new();
        for scorecard in &self.scorecards {
            validate_scorecard(scorecard)?;
            if !scorecard_ids.insert(scorecard.scorecard_id.as_str()) {
                return Err(err(format!(
                    "duplicate scorecard_id: {}",
                    scorecard.scorecard_id
                )));
            }
        }
        validate_compatibility_summary(&self.compatibility_summary)?;
        validate_activation_budget(&self.activation_budget)?;
        validate_support_class(&self.support_class)?;
        validate_publisher_continuity(&self.publisher_continuity)?;
        validate_view_alignment(&self.view_alignment)?;
        validate_claim(&self.claim)?;
        validate_banner(&self.downgraded_banner)?;

        for surface in &self.consumer_surfaces {
            ensure_token(
                STABLE_MARKETPLACE_CATALOG_CONSUMER_SURFACES,
                surface,
                "consumer_surface",
            )?;
        }
        if self.consumer_surfaces.is_empty() {
            return Err(err("packet must bind at least one consumer surface"));
        }
        if !self
            .source_schema_refs
            .iter()
            .any(|r| r == STABLE_MARKETPLACE_CATALOG_TRUTH_SCHEMA_REF)
        {
            return Err(err("packet must cite its source schema"));
        }

        // No catalog-only trust, unbounded activation cost, inherited parity, or
        // ranking-implied trust may ride a published stable catalog row.
        if self.allows_catalog_only_trust
            || self.allows_unbounded_activation_cost
            || self.allows_inherited_parity_stable_claim
            || self.allows_ranking_implied_trust
        {
            return Err(err(
                "a stable catalog-truth packet must not allow catalog-only trust, unbounded activation cost, inherited parity, or ranking-implied trust",
            ));
        }

        // The provenance must be mechanically sourced, never copied badge text.
        if !self.provenance.mechanically_sourced {
            return Err(err(
                "provenance must be mechanically sourced from one registry/status model",
            ));
        }
        // Ranking must be explainable without raw install count or operator notes.
        if !self.discoverability.ranking_explained_without_install_count {
            return Err(err(
                "ranking state must be explainable without raw install count or operator notes",
            ));
        }

        // The compatibility summary is re-derived from the scorecards so a stored
        // packet cannot hide an inherited, unsupported, or stale scorecard.
        let derived_summary = summarize_compatibility(&self.scorecards);
        if derived_summary != self.compatibility_summary {
            return Err(err(
                "stored compatibility summary does not match the scorecard-derived truth",
            ));
        }

        // Stable-claim binding.
        if STABLE_TIERS.contains(&self.claim.effective_tier.as_str()) {
            if !self.identity.catalog_version_current() {
                return Err(err(
                    "stable effective tier must pin the published catalog-truth version",
                ));
            }
            if self.claim.claim_basis_class != "evidence_backed" {
                return Err(err(
                    "stable effective tier must be evidence-backed, not catalog-asserted",
                ));
            }
            if self.identity.publisher_trust_tier_class == "quarantined" {
                return Err(err(
                    "stable effective tier must not carry a quarantined trust tier",
                ));
            }
            if !self.identity.lifecycle_installable() {
                return Err(err("stable effective tier must stay on an installable lifecycle"));
            }
            if self.provenance.under_review() {
                return Err(err(
                    "stable effective tier must not carry an under-review provenance",
                ));
            }
            if self.discoverability.quarantined() || self.discoverability.penalized() {
                return Err(err(
                    "stable effective tier must be ranked normally, not penalized or quarantined",
                ));
            }
            if !self.surface_boundary.runtime_class_verified {
                return Err(err("stable effective tier must carry a verified runtime class"));
            }
            if !self.surface_boundary.surface_disclosure_ok() {
                return Err(err(
                    "stable effective tier must disclose any hosted-surface or browser-handoff boundary",
                ));
            }
            if !self.compatibility_summary.stable_backable() {
                return Err(err(
                    "stable effective tier must carry compatibility scorecards with no inherited, unsupported, or not-yet-run evidence",
                ));
            }
            if self.compatibility_summary.stale_count > 0
                || self.compatibility_summary.parity_limited_count > 0
            {
                return Err(err(
                    "stable effective tier must not carry a stale or parity-limited scorecard",
                ));
            }
            if !self.activation_budget.within_budget() {
                return Err(err(
                    "stable effective tier must keep its activation cost bounded and within budget",
                ));
            }
            if !self.support_class.stable_grade() || self.support_class.profile_limited {
                return Err(err(
                    "stable effective tier must keep a stable-grade, non-profile-limited support class",
                ));
            }
            if !self.publisher_continuity.current() {
                return Err(err(
                    "stable effective tier must keep its publisher continuity current",
                ));
            }
            if !self.view_alignment.all_views_aligned() {
                return Err(err(
                    "stable effective tier must keep its truth aligned across all catalog views",
                ));
            }
            if !self.attribution_complete() {
                return Err(err("stable effective tier must be fully attributed"));
            }
            if self.claim.downgraded {
                return Err(err(
                    "a stable effective tier must not also be marked downgraded",
                ));
            }
        }

        // Downgrade truth.
        if self.claim.downgraded {
            if self.claim.downgrade_reasons.is_empty() {
                return Err(err("a downgraded claim must carry at least one reason"));
            }
            if STABLE_TIERS.contains(&self.claim.effective_tier.as_str()) {
                return Err(err("a downgraded claim must not keep a stable tier"));
            }
        }

        // Re-derive the effective tier and downgrade verdict so the stored claim cannot
        // drift from the posture truth.
        let posture = CatalogPosture {
            identity: &self.identity,
            provenance: &self.provenance,
            surface_boundary: &self.surface_boundary,
            discoverability: &self.discoverability,
            compatibility_summary: &self.compatibility_summary,
            activation_budget: &self.activation_budget,
            support_class: &self.support_class,
            publisher_continuity: &self.publisher_continuity,
            view_alignment: &self.view_alignment,
            attribution_complete: self.attribution_complete(),
        };
        let derived = derive_effective_tier(
            &self.claim.claimed_tier,
            &self.claim.claim_basis_class,
            &posture,
        );
        if derived.effective_tier != self.claim.effective_tier {
            return Err(err(
                "stored effective tier does not match the posture-derived tier",
            ));
        }
        if derived.downgraded != self.claim.downgraded {
            return Err(err(
                "stored downgrade flag does not match the posture-derived verdict",
            ));
        }
        let stored: BTreeSet<&str> = self
            .claim
            .downgrade_reasons
            .iter()
            .map(String::as_str)
            .collect();
        let expected: BTreeSet<&str> =
            derived.downgrade_reasons.iter().map(String::as_str).collect();
        if stored != expected {
            return Err(err(
                "stored downgrade reasons do not match the posture-derived reasons",
            ));
        }

        // Banner truth.
        let banner_required = catalog_requires_warning(&posture);
        if self.downgraded_banner.must_display != banner_required {
            return Err(err(
                "downgraded-row banner must_display does not match the catalog posture",
            ));
        }

        validate_inspection(&self.inspection, self)?;
        Ok(())
    }

    /// Returns true when no stable claim is implied from catalog-only trust.
    pub fn no_catalog_only_stable_claim(&self) -> bool {
        if STABLE_TIERS.contains(&self.claim.effective_tier.as_str()) {
            return self.claim.claim_basis_class == "evidence_backed";
        }
        true
    }

    /// Returns true when identity, provenance, and every scorecard are fully attributed.
    pub fn attribution_complete(&self) -> bool {
        attribution_is_complete(&self.identity, &self.provenance, &self.scorecards)
    }
}

// ---------------------------------------------------------------------------
// Projection
// ---------------------------------------------------------------------------

/// Compact projection consumed by CLI/headless and dashboard surfaces.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StableMarketplaceCatalogTruthProjection {
    /// Stable packet identity.
    pub packet_id: String,
    /// Marketplace row identity.
    pub row_identity_ref: String,
    /// Claimed tier.
    pub claimed_tier: String,
    /// Effective tier after the posture is applied.
    pub effective_tier: String,
    /// Support claim class.
    pub support_claim_class: String,
    /// True when the claim is a stable catalog-truth claim.
    pub stable_claim: bool,
    /// True when the claimed tier was narrowed.
    pub downgraded: bool,
    /// Downgrade reasons.
    pub downgrade_reasons: Vec<String>,
    /// True when a downgraded-row banner is required.
    pub downgraded_banner_required: bool,
    /// Provenance class.
    pub provenance_class: String,
    /// Ranking state.
    pub ranking_state_class: String,
    /// Support class.
    pub support_class_class: String,
    /// Runtime class.
    pub runtime_class: String,
    /// Number of compatibility scorecards.
    pub scorecard_count: usize,
}

impl From<StableMarketplaceCatalogTruthPacket> for StableMarketplaceCatalogTruthProjection {
    fn from(packet: StableMarketplaceCatalogTruthPacket) -> Self {
        Self {
            packet_id: packet.packet_id,
            row_identity_ref: packet.identity.row_identity_ref,
            claimed_tier: packet.claim.claimed_tier,
            effective_tier: packet.claim.effective_tier,
            support_claim_class: packet.claim.support_claim_class,
            stable_claim: packet.inspection.stable_claim,
            downgraded: packet.claim.downgraded,
            downgrade_reasons: packet.claim.downgrade_reasons,
            downgraded_banner_required: packet.downgraded_banner.must_display,
            provenance_class: packet.provenance.provenance_class,
            ranking_state_class: packet.discoverability.ranking_state_class,
            support_class_class: packet.support_class.support_class_class,
            runtime_class: packet.surface_boundary.runtime_class,
            scorecard_count: packet.inspection.scorecard_count,
        }
    }
}

/// Parses and validates a materialized packet, returning the compact projection.
///
/// # Errors
///
/// Returns [`StableMarketplaceCatalogTruthError`] when the payload fails to parse or
/// violates the stable catalog-truth invariants.
pub fn project_stable_marketplace_catalog_truth(
    payload: &str,
) -> Result<StableMarketplaceCatalogTruthProjection, StableMarketplaceCatalogTruthError> {
    let packet: StableMarketplaceCatalogTruthPacket = serde_json::from_str(payload)?;
    packet.validate()?;
    Ok(StableMarketplaceCatalogTruthProjection::from(packet))
}

// ---------------------------------------------------------------------------
// Support export projection
// ---------------------------------------------------------------------------

/// Metadata-safe support/partner/mirror export row that quotes the same closed tokens
/// as the packet without leaking raw scorecard, evidence, or publisher-private bytes,
/// and preserves the runtime-class and profile-limited support truth so a package
/// verified on one runtime cannot imply parity everywhere else.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StableMarketplaceCatalogTruthSupportExport {
    /// Record-kind tag.
    pub record_kind: String,
    /// Integer schema version.
    pub schema_version: u32,
    /// Stable support-export id.
    pub export_id: String,
    /// Ref to the packet this export quotes.
    pub packet_ref: String,
    /// Marketplace row identity.
    pub row_identity_ref: String,
    /// Catalog descriptor ref.
    pub catalog_descriptor_ref: String,
    /// Extension identity.
    pub extension_identity: String,
    /// Extension version.
    pub extension_version: String,
    /// Provenance class.
    pub provenance_class: String,
    /// Publisher trust tier.
    pub trust_tier_class: String,
    /// Lifecycle state.
    pub lifecycle_state_class: String,
    /// Ranking state.
    pub ranking_state_class: String,
    /// Runtime class.
    pub runtime_class: String,
    /// Host boundary class.
    pub host_boundary_class: String,
    /// True when the runtime class is verified for this package.
    pub runtime_class_verified: bool,
    /// Support class.
    pub support_class_class: String,
    /// True when support is profile-limited.
    pub support_profile_limited: bool,
    /// Ref to the runtime profile the support class was verified on.
    pub verified_runtime_profile_ref: String,
    /// Activation-budget posture.
    pub activation_budget_class: String,
    /// Publisher-continuity state.
    pub publisher_continuity_class: String,
    /// Effective tier.
    pub effective_tier: String,
    /// Support claim class.
    pub support_claim_class: String,
    /// True when the claim was narrowed below Stable.
    pub downgraded: bool,
    /// Narrowing reasons.
    pub downgrade_reasons: Vec<String>,
    /// True when a downgraded-row banner is required.
    pub downgraded_banner_required: bool,
    /// True when the effective tier blocks the row as stable catalog truth (withdrawn).
    pub blocks_stable_catalog_truth: bool,
    /// Number of compatibility scorecards.
    pub scorecard_count: usize,
    /// Number of scorecards inheriting parity from an adjacent claim.
    pub inherited_scorecard_count: usize,
    /// Number of unsupported scorecards.
    pub unsupported_scorecard_count: usize,
    /// True when the catalog truth is aligned across all views.
    pub views_aligned: bool,
    /// Export-safe summary suitable for support/partner/mirror consumers.
    pub export_safe_summary: String,
}

/// Projects a packet into the metadata-safe support/partner/mirror export row.
pub fn project_stable_marketplace_catalog_truth_support_export(
    packet: &StableMarketplaceCatalogTruthPacket,
) -> StableMarketplaceCatalogTruthSupportExport {
    let blocks = packet.claim.effective_tier == "withdrawn";
    let export_safe_summary = format!(
        "{} Provenance={} trust={} lifecycle={} ranking={}. Runtime={} ({}) boundary={} verified={}. Support={} profile_limited={} on {}. Activation={}. Continuity={}. Scorecards={} inherited={} unsupported={} stale={}. Views aligned={}. Tier claimed={} effective={} (downgraded={}). Banner required={}.",
        packet.claim.summary_label,
        packet.provenance.provenance_class,
        packet.identity.publisher_trust_tier_class,
        packet.identity.lifecycle_state_class,
        packet.discoverability.ranking_state_class,
        packet.surface_boundary.runtime_class,
        packet.surface_boundary.host_boundary_class,
        packet.surface_boundary.host_boundary_class,
        packet.surface_boundary.runtime_class_verified,
        packet.support_class.support_class_class,
        packet.support_class.profile_limited,
        packet.support_class.verified_runtime_profile_ref,
        packet.activation_budget.budget_class,
        packet.publisher_continuity.continuity_state_class,
        packet.compatibility_summary.scorecard_count,
        packet.compatibility_summary.inherited_count,
        packet.compatibility_summary.unsupported_count,
        packet.compatibility_summary.stale_count,
        packet.view_alignment.all_views_aligned(),
        packet.claim.claimed_tier,
        packet.claim.effective_tier,
        packet.claim.downgraded,
        packet.downgraded_banner.must_display,
    );

    StableMarketplaceCatalogTruthSupportExport {
        record_kind: STABLE_MARKETPLACE_CATALOG_TRUTH_SUPPORT_EXPORT_RECORD_KIND.to_string(),
        schema_version: STABLE_MARKETPLACE_CATALOG_TRUTH_SCHEMA_VERSION,
        export_id: format!(
            "stable_marketplace_catalog_truth_support_export:{}",
            packet.packet_id
        ),
        packet_ref: packet.packet_id.clone(),
        row_identity_ref: packet.identity.row_identity_ref.clone(),
        catalog_descriptor_ref: packet.identity.catalog_descriptor_ref.clone(),
        extension_identity: packet.identity.extension_identity.clone(),
        extension_version: packet.identity.extension_version.clone(),
        provenance_class: packet.provenance.provenance_class.clone(),
        trust_tier_class: packet.identity.publisher_trust_tier_class.clone(),
        lifecycle_state_class: packet.identity.lifecycle_state_class.clone(),
        ranking_state_class: packet.discoverability.ranking_state_class.clone(),
        runtime_class: packet.surface_boundary.runtime_class.clone(),
        host_boundary_class: packet.surface_boundary.host_boundary_class.clone(),
        runtime_class_verified: packet.surface_boundary.runtime_class_verified,
        support_class_class: packet.support_class.support_class_class.clone(),
        support_profile_limited: packet.support_class.profile_limited,
        verified_runtime_profile_ref: packet.support_class.verified_runtime_profile_ref.clone(),
        activation_budget_class: packet.activation_budget.budget_class.clone(),
        publisher_continuity_class: packet.publisher_continuity.continuity_state_class.clone(),
        effective_tier: packet.claim.effective_tier.clone(),
        support_claim_class: packet.claim.support_claim_class.clone(),
        downgraded: packet.claim.downgraded,
        downgrade_reasons: packet.claim.downgrade_reasons.clone(),
        downgraded_banner_required: packet.downgraded_banner.must_display,
        blocks_stable_catalog_truth: blocks,
        scorecard_count: packet.compatibility_summary.scorecard_count,
        inherited_scorecard_count: packet.compatibility_summary.inherited_count,
        unsupported_scorecard_count: packet.compatibility_summary.unsupported_count,
        views_aligned: packet.view_alignment.all_views_aligned(),
        export_safe_summary,
    }
}

// ---------------------------------------------------------------------------
// Error types
// ---------------------------------------------------------------------------

/// Error enum for stable catalog-truth operations.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StableMarketplaceCatalogTruthError {
    /// Validation failed.
    Validation(StableMarketplaceCatalogTruthValidationError),
    /// Packet construction failed.
    Construction(String),
}

impl fmt::Display for StableMarketplaceCatalogTruthError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Validation(e) => write!(f, "validation error: {e}"),
            Self::Construction(msg) => write!(f, "construction error: {msg}"),
        }
    }
}

impl std::error::Error for StableMarketplaceCatalogTruthError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Validation(e) => Some(e),
            Self::Construction(_) => None,
        }
    }
}

/// Validation error for stable catalog-truth packets.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StableMarketplaceCatalogTruthValidationError {
    /// Redaction-safe message.
    pub message: String,
}

impl fmt::Display for StableMarketplaceCatalogTruthValidationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for StableMarketplaceCatalogTruthValidationError {}

impl StableMarketplaceCatalogTruthValidationError {
    /// Returns the validation failure message.
    pub fn message(&self) -> &str {
        &self.message
    }
}

impl From<serde_json::Error> for StableMarketplaceCatalogTruthError {
    fn from(err: serde_json::Error) -> Self {
        Self::Validation(StableMarketplaceCatalogTruthValidationError {
            message: err.to_string(),
        })
    }
}

impl From<StableMarketplaceCatalogTruthValidationError> for StableMarketplaceCatalogTruthError {
    fn from(err: StableMarketplaceCatalogTruthValidationError) -> Self {
        Self::Validation(err)
    }
}

// ---------------------------------------------------------------------------
// Compatibility aggregation
// ---------------------------------------------------------------------------

/// Summarizes the compatibility posture across the scorecards. The summary is derived
/// purely from the scorecards so it cannot drift from its evidence.
fn summarize_compatibility(
    scorecards: &[MarketplaceCompatibilityScorecard],
) -> MarketplaceCompatibilitySummary {
    let inherited_count = scorecards.iter().filter(|s| s.parity_inherited()).count();
    let not_evaluated_count = scorecards.iter().filter(|s| s.evidence_not_run()).count();
    let stale_count = scorecards.iter().filter(|s| s.stale()).count();
    let unsupported_count = scorecards.iter().filter(|s| s.unsupported()).count();
    let parity_limited_count = scorecards
        .iter()
        .filter(|s| !s.unsupported() && s.parity_limited())
        .count();

    let present: BTreeSet<String> = scorecards.iter().map(|s| s.subject_class.clone()).collect();
    let present_subjects: Vec<String> = present.iter().cloned().collect();

    let has_scorecard = !scorecards.is_empty();
    let no_inherited_parity = inherited_count == 0;
    let all_scorecards_supportable = has_scorecard
        && unsupported_count == 0
        && not_evaluated_count == 0
        && stale_count == 0;

    MarketplaceCompatibilitySummary {
        record_kind: MARKETPLACE_COMPATIBILITY_SUMMARY_RECORD_KIND.to_string(),
        schema_version: STABLE_MARKETPLACE_CATALOG_TRUTH_SCHEMA_VERSION,
        scorecard_count: scorecards.len(),
        inherited_count,
        not_evaluated_count,
        stale_count,
        unsupported_count,
        parity_limited_count,
        present_subjects,
        has_scorecard,
        no_inherited_parity,
        all_scorecards_supportable,
    }
}

// ---------------------------------------------------------------------------
// Effective-tier derivation (the automatic narrowing)
// ---------------------------------------------------------------------------

/// Bundle of derived records used to apply the catalog posture.
struct CatalogPosture<'a> {
    identity: &'a MarketplaceCatalogTruthIdentity,
    provenance: &'a MarketplaceCatalogProvenance,
    surface_boundary: &'a MarketplaceCatalogSurfaceBoundary,
    discoverability: &'a MarketplaceCatalogDiscoverabilityPosture,
    compatibility_summary: &'a MarketplaceCompatibilitySummary,
    activation_budget: &'a MarketplaceCatalogActivationBudget,
    support_class: &'a MarketplaceCatalogSupportClass,
    publisher_continuity: &'a MarketplaceCatalogPublisherContinuity,
    view_alignment: &'a MarketplaceCatalogViewAlignment,
    attribution_complete: bool,
}

struct DerivedTier {
    effective_tier: String,
    support_claim: String,
    downgraded: bool,
    downgrade_reasons: Vec<String>,
}

/// Collects the narrowing reasons triggered by the catalog posture.
fn posture_reasons(posture: &CatalogPosture<'_>) -> Vec<String> {
    let mut reasons: Vec<String> = Vec::new();

    if !posture.identity.catalog_version_current() {
        reasons.push("catalog_version_not_published".to_string());
    }
    if posture.identity.publisher_trust_tier_class == "quarantined" {
        reasons.push("trust_tier_quarantined".to_string());
    }
    if !posture.identity.lifecycle_installable() {
        reasons.push("lifecycle_not_installable".to_string());
    }
    if posture.provenance.under_review() {
        reasons.push("provenance_under_review".to_string());
    }
    if posture.discoverability.quarantined() {
        reasons.push("quarantined_from_discovery".to_string());
    } else if posture.discoverability.penalized() {
        reasons.push("ranking_penalized".to_string());
    }
    if !posture.surface_boundary.runtime_class_verified {
        reasons.push("runtime_class_unverified".to_string());
    }
    if !posture.surface_boundary.surface_disclosure_ok() {
        reasons.push("hosted_surface_boundary_undisclosed".to_string());
    }
    if !posture.compatibility_summary.has_scorecard {
        reasons.push("missing_required_scorecard".to_string());
    }
    if posture.compatibility_summary.unsupported_count > 0 {
        reasons.push("compatibility_unsupported".to_string());
    }
    if posture.compatibility_summary.parity_limited_count > 0 {
        reasons.push("compatibility_parity_limited".to_string());
    }
    if posture.compatibility_summary.inherited_count > 0 {
        reasons.push("scorecard_parity_inherited".to_string());
    }
    if posture.compatibility_summary.not_evaluated_count > 0 {
        reasons.push("scorecard_evidence_not_run".to_string());
    }
    if posture.compatibility_summary.stale_count > 0 {
        reasons.push("scorecard_freshness_stale".to_string());
    }
    match posture.activation_budget.budget_class.as_str() {
        "unbounded" => reasons.push("activation_cost_unbounded".to_string()),
        "over_budget" => reasons.push("activation_cost_over_budget".to_string()),
        "not_measured" => reasons.push("activation_cost_not_measured".to_string()),
        _ => {}
    }
    if posture.support_class.limited() {
        reasons.push("support_class_limited".to_string());
    } else if !posture.support_class.stable_grade() {
        reasons.push("support_class_below_stable_grade".to_string());
    }
    if posture.support_class.profile_limited {
        reasons.push("support_class_profile_limited".to_string());
    }
    match posture.publisher_continuity.continuity_state_class.as_str() {
        "revoked" => reasons.push("publisher_continuity_revoked".to_string()),
        "missing" => reasons.push("publisher_continuity_missing".to_string()),
        "stale" => reasons.push("publisher_continuity_stale".to_string()),
        _ => {}
    }
    if !posture.view_alignment.all_views_aligned() {
        reasons.push("views_not_aligned".to_string());
    }
    if !posture.attribution_complete {
        reasons.push("attribution_incomplete".to_string());
    }

    reasons.sort();
    reasons.dedup();
    reasons
}

/// Applies the catalog posture to a claimed tier, narrowing automatically below Stable
/// when the evidence can no longer back it. The claim basis is folded in separately so
/// a `catalog_asserted_only` basis can never back a stable claim.
fn derive_effective_tier(
    claimed_tier: &str,
    claim_basis: &str,
    posture: &CatalogPosture<'_>,
) -> DerivedTier {
    // Non-stable claims are already honest; they pass through unchanged.
    if !STABLE_TIERS.contains(&claimed_tier) {
        return DerivedTier {
            effective_tier: claimed_tier.to_string(),
            support_claim: support_claim_for(claimed_tier),
            downgraded: false,
            downgrade_reasons: Vec::new(),
        };
    }

    let mut reasons = posture_reasons(posture);
    if claim_basis != "evidence_backed" {
        reasons.push("catalog_only_trust_not_evidence_backed".to_string());
        reasons.sort();
        reasons.dedup();
    }

    if reasons.is_empty() {
        DerivedTier {
            effective_tier: claimed_tier.to_string(),
            support_claim: support_claim_for(claimed_tier),
            downgraded: false,
            downgrade_reasons: Vec::new(),
        }
    } else {
        let effective_tier = narrow_tier_for(&reasons);
        DerivedTier {
            effective_tier: effective_tier.to_string(),
            support_claim: support_claim_for(effective_tier),
            downgraded: true,
            downgrade_reasons: reasons,
        }
    }
}

/// Picks the effective tier given the active narrowing reasons.
fn narrow_tier_for(reasons: &[String]) -> &'static str {
    if reasons.iter().any(|r| WITHDRAWN_CLASS_REASONS.contains(&r.as_str())) {
        "withdrawn"
    } else if reasons.iter().any(|r| PREVIEW_CLASS_REASONS.contains(&r.as_str())) {
        "preview"
    } else {
        debug_assert!(reasons.iter().all(|r| BETA_CLASS_REASONS.contains(&r.as_str())));
        "beta"
    }
}

/// Maps an effective tier to the support claim it may imply.
fn support_claim_for(tier: &str) -> String {
    match tier {
        "stable" => "stable_catalog_truth_claim",
        "beta" => "beta_catalog_truth_partial_claim",
        "preview" => "preview_catalog_truth_experimental_claim",
        "withdrawn" => "withdrawn_no_catalog_truth_claim",
        _ => "preview_catalog_truth_experimental_claim",
    }
    .to_string()
}

/// Returns true when identity, provenance, and every scorecard are fully attributed.
fn attribution_is_complete(
    identity: &MarketplaceCatalogTruthIdentity,
    provenance: &MarketplaceCatalogProvenance,
    scorecards: &[MarketplaceCompatibilityScorecard],
) -> bool {
    !identity.catalog_descriptor_ref.trim().is_empty()
        && !identity.row_identity_ref.trim().is_empty()
        && !identity.source_package_ref.trim().is_empty()
        && !provenance.status_source_ref.trim().is_empty()
        && scorecards.iter().all(|s| {
            !s.scorecard_id.trim().is_empty() && !s.scorecard_ref.trim().is_empty()
        })
}

/// Returns true when the catalog posture requires a pre-trust warning banner.
fn catalog_requires_warning(posture: &CatalogPosture<'_>) -> bool {
    posture.identity.publisher_trust_tier_class == "quarantined"
        || !posture.identity.lifecycle_installable()
        || posture.provenance.under_review()
        || posture.discoverability.quarantined()
        || !posture.surface_boundary.runtime_class_verified
        || !posture.surface_boundary.surface_disclosure_ok()
        || !posture.compatibility_summary.has_scorecard
        || posture.compatibility_summary.unsupported_count > 0
        || posture.compatibility_summary.inherited_count > 0
        || posture.activation_budget.unbounded()
        || matches!(
            posture.publisher_continuity.continuity_state_class.as_str(),
            "revoked" | "missing"
        )
}

/// Picks the most-severe banner reason for a row that requires a warning.
fn banner_reason_for(posture: &CatalogPosture<'_>) -> Option<String> {
    if !posture.compatibility_summary.has_scorecard {
        return Some("missing_required_scorecard".to_string());
    }
    if posture.compatibility_summary.unsupported_count > 0 {
        return Some("compatibility_unsupported".to_string());
    }
    if !posture.surface_boundary.runtime_class_verified {
        return Some("runtime_class_unverified".to_string());
    }
    if posture.provenance.under_review() {
        return Some("provenance_under_review".to_string());
    }
    if posture.discoverability.quarantined() {
        return Some("quarantined_from_discovery".to_string());
    }
    if posture.activation_budget.unbounded() {
        return Some("activation_cost_unbounded".to_string());
    }
    if posture.publisher_continuity.continuity_state_class == "revoked" {
        return Some("publisher_continuity_revoked".to_string());
    }
    if !posture.identity.lifecycle_installable() {
        return Some("lifecycle_not_installable".to_string());
    }
    if posture.compatibility_summary.inherited_count > 0 {
        return Some("scorecard_parity_inherited".to_string());
    }
    if !posture.surface_boundary.surface_disclosure_ok() {
        return Some("hosted_surface_boundary_undisclosed".to_string());
    }
    if posture.publisher_continuity.continuity_state_class == "missing" {
        return Some("publisher_continuity_missing".to_string());
    }
    if posture.identity.publisher_trust_tier_class == "quarantined" {
        return Some("trust_tier_quarantined".to_string());
    }
    None
}

// ---------------------------------------------------------------------------
// Record constructors
// ---------------------------------------------------------------------------

fn identity_record(input: &MarketplaceCatalogTruthIdentityInput) -> MarketplaceCatalogTruthIdentity {
    MarketplaceCatalogTruthIdentity {
        record_kind: MARKETPLACE_CATALOG_TRUTH_IDENTITY_RECORD_KIND.to_string(),
        schema_version: STABLE_MARKETPLACE_CATALOG_TRUTH_SCHEMA_VERSION,
        catalog_descriptor_ref: input.catalog_descriptor_ref.clone(),
        row_identity_ref: input.row_identity_ref.clone(),
        extension_identity: input.extension_identity.clone(),
        extension_version: input.extension_version.clone(),
        package_id: input.package_id.clone(),
        catalog_version: input.catalog_version,
        source_package_ref: input.source_package_ref.clone(),
        publisher_trust_tier_class: input.publisher_trust_tier_class.clone(),
        lifecycle_state_class: input.lifecycle_state_class.clone(),
    }
}

fn provenance_record(input: &MarketplaceCatalogProvenanceInput) -> MarketplaceCatalogProvenance {
    MarketplaceCatalogProvenance {
        record_kind: MARKETPLACE_CATALOG_PROVENANCE_RECORD_KIND.to_string(),
        schema_version: STABLE_MARKETPLACE_CATALOG_TRUTH_SCHEMA_VERSION,
        provenance_class: input.provenance_class.clone(),
        mechanically_sourced: input.mechanically_sourced,
        status_source_ref: input.status_source_ref.clone(),
        summary_label: input.summary_label.clone(),
    }
}

fn surface_boundary_record(
    input: &MarketplaceCatalogSurfaceBoundaryInput,
) -> MarketplaceCatalogSurfaceBoundary {
    MarketplaceCatalogSurfaceBoundary {
        record_kind: MARKETPLACE_CATALOG_SURFACE_BOUNDARY_RECORD_KIND.to_string(),
        schema_version: STABLE_MARKETPLACE_CATALOG_TRUTH_SCHEMA_VERSION,
        runtime_class: input.runtime_class.clone(),
        host_boundary_class: input.host_boundary_class.clone(),
        runtime_class_verified: input.runtime_class_verified,
        hosted_surface_implication: input.hosted_surface_implication,
        browser_handoff_implication: input.browser_handoff_implication,
        surface_boundary_disclosed: input.surface_boundary_disclosed,
        reduced_accessibility_parity: input.reduced_accessibility_parity,
        reduced_theming_parity: input.reduced_theming_parity,
        summary_label: input.summary_label.clone(),
    }
}

fn discoverability_record(
    input: &MarketplaceCatalogDiscoverabilityPostureInput,
) -> MarketplaceCatalogDiscoverabilityPosture {
    MarketplaceCatalogDiscoverabilityPosture {
        record_kind: MARKETPLACE_CATALOG_DISCOVERABILITY_POSTURE_RECORD_KIND.to_string(),
        schema_version: STABLE_MARKETPLACE_CATALOG_TRUTH_SCHEMA_VERSION,
        ranking_state_class: input.ranking_state_class.clone(),
        ranking_explained_without_install_count: input.ranking_explained_without_install_count,
        ranking_rationale_ref: input.ranking_rationale_ref.clone(),
        summary_label: input.summary_label.clone(),
    }
}

fn scorecard_record(
    input: &MarketplaceCompatibilityScorecardInput,
) -> MarketplaceCompatibilityScorecard {
    MarketplaceCompatibilityScorecard {
        record_kind: MARKETPLACE_COMPATIBILITY_SCORECARD_RECORD_KIND.to_string(),
        schema_version: STABLE_MARKETPLACE_CATALOG_TRUTH_SCHEMA_VERSION,
        scorecard_id: input.scorecard_id.clone(),
        subject_class: input.subject_class.clone(),
        parity_band_class: input.parity_band_class.clone(),
        freshness_window_class: input.freshness_window_class.clone(),
        evidence_source_class: input.evidence_source_class.clone(),
        downgrade_state_class: input.downgrade_state_class.clone(),
        scorecard_ref: input.scorecard_ref.clone(),
        summary_label: input.summary_label.clone(),
    }
}

fn activation_budget_record(
    input: &MarketplaceCatalogActivationBudgetInput,
) -> MarketplaceCatalogActivationBudget {
    MarketplaceCatalogActivationBudget {
        record_kind: MARKETPLACE_CATALOG_ACTIVATION_BUDGET_RECORD_KIND.to_string(),
        schema_version: STABLE_MARKETPLACE_CATALOG_TRUTH_SCHEMA_VERSION,
        budget_class: input.budget_class.clone(),
        measured_cost_ref: input.measured_cost_ref.clone(),
        budget_ceiling_ref: input.budget_ceiling_ref.clone(),
        measured_surface_count: input.measured_surface_count,
        summary_label: input.summary_label.clone(),
    }
}

fn support_class_record(
    input: &MarketplaceCatalogSupportClassInput,
) -> MarketplaceCatalogSupportClass {
    MarketplaceCatalogSupportClass {
        record_kind: MARKETPLACE_CATALOG_SUPPORT_CLASS_RECORD_KIND.to_string(),
        schema_version: STABLE_MARKETPLACE_CATALOG_TRUTH_SCHEMA_VERSION,
        support_class_class: input.support_class_class.clone(),
        profile_limited: input.profile_limited,
        verified_runtime_profile_ref: input.verified_runtime_profile_ref.clone(),
        evidence_ref: input.evidence_ref.clone(),
        summary_label: input.summary_label.clone(),
    }
}

fn publisher_continuity_record(
    input: &MarketplaceCatalogPublisherContinuityInput,
) -> MarketplaceCatalogPublisherContinuity {
    MarketplaceCatalogPublisherContinuity {
        record_kind: MARKETPLACE_CATALOG_PUBLISHER_CONTINUITY_RECORD_KIND.to_string(),
        schema_version: STABLE_MARKETPLACE_CATALOG_TRUTH_SCHEMA_VERSION,
        continuity_state_class: input.continuity_state_class.clone(),
        continuity_packet_ref: input.continuity_packet_ref.clone(),
        summary_label: input.summary_label.clone(),
    }
}

fn view_alignment_record(
    input: &MarketplaceCatalogViewAlignmentInput,
) -> MarketplaceCatalogViewAlignment {
    MarketplaceCatalogViewAlignment {
        record_kind: MARKETPLACE_CATALOG_VIEW_ALIGNMENT_RECORD_KIND.to_string(),
        schema_version: STABLE_MARKETPLACE_CATALOG_TRUTH_SCHEMA_VERSION,
        aligned_views: input.aligned_views.clone(),
        runtime_class_preserved_across_views: input.runtime_class_preserved_across_views,
        summary_label: input.summary_label.clone(),
    }
}

fn claim_record(
    input: &MarketplaceCatalogTruthQualificationClaimInput,
    posture: &CatalogPosture<'_>,
) -> MarketplaceCatalogTruthQualificationClaim {
    let derived = derive_effective_tier(&input.claimed_tier, &input.claim_basis_class, posture);
    MarketplaceCatalogTruthQualificationClaim {
        record_kind: MARKETPLACE_CATALOG_TRUTH_QUALIFICATION_CLAIM_RECORD_KIND.to_string(),
        schema_version: STABLE_MARKETPLACE_CATALOG_TRUTH_SCHEMA_VERSION,
        claimed_tier: input.claimed_tier.clone(),
        effective_tier: derived.effective_tier,
        support_claim_class: derived.support_claim,
        claim_basis_class: input.claim_basis_class.clone(),
        downgraded: derived.downgraded,
        downgrade_reasons: derived.downgrade_reasons,
        summary_label: input.summary_label.clone(),
    }
}

fn banner_record(posture: &CatalogPosture<'_>) -> DowngradedCatalogBanner {
    let must_display = catalog_requires_warning(posture);
    let banner_reason_class = if must_display {
        banner_reason_for(posture)
    } else {
        None
    };
    let summary_label = if must_display {
        format!(
            "Catalog row requires review before install or enablement ({}).",
            banner_reason_class.clone().unwrap_or_default()
        )
    } else {
        "Catalog truth stabilized: provenance, compatibility, activation, support, and continuity all current."
            .to_string()
    };
    DowngradedCatalogBanner {
        record_kind: DOWNGRADED_CATALOG_BANNER_RECORD_KIND.to_string(),
        schema_version: STABLE_MARKETPLACE_CATALOG_TRUTH_SCHEMA_VERSION,
        must_display,
        banner_reason_class,
        summary_label,
    }
}

fn inspection_record(
    packet_id: &str,
    posture: &CatalogPosture<'_>,
    claim: &MarketplaceCatalogTruthQualificationClaim,
    banner: &DowngradedCatalogBanner,
) -> StableMarketplaceCatalogTruthInspection {
    let stable_claim = STABLE_TIERS.contains(&claim.effective_tier.as_str());

    StableMarketplaceCatalogTruthInspection {
        record_kind: STABLE_MARKETPLACE_CATALOG_TRUTH_INSPECTION_RECORD_KIND.to_string(),
        schema_version: STABLE_MARKETPLACE_CATALOG_TRUTH_SCHEMA_VERSION,
        packet_id_ref: packet_id.to_string(),
        effective_tier: claim.effective_tier.clone(),
        stable_claim,
        catalog_version_current: posture.identity.catalog_version_current(),
        trust_tier_class: posture.identity.publisher_trust_tier_class.clone(),
        provenance_class: posture.provenance.provenance_class.clone(),
        lifecycle_installable: posture.identity.lifecycle_installable(),
        ranking_state_class: posture.discoverability.ranking_state_class.clone(),
        runtime_class: posture.surface_boundary.runtime_class.clone(),
        host_boundary_class: posture.surface_boundary.host_boundary_class.clone(),
        runtime_class_verified: posture.surface_boundary.runtime_class_verified,
        surface_boundary_disclosed: posture.surface_boundary.surface_disclosure_ok(),
        support_class_class: posture.support_class.support_class_class.clone(),
        support_profile_limited: posture.support_class.profile_limited,
        activation_budget_class: posture.activation_budget.budget_class.clone(),
        activation_within_budget: posture.activation_budget.within_budget(),
        publisher_continuity_class: posture.publisher_continuity.continuity_state_class.clone(),
        compatibility_stable_backable: posture.compatibility_summary.stable_backable(),
        views_aligned: posture.view_alignment.all_views_aligned(),
        downgraded: claim.downgraded,
        downgraded_banner_required: banner.must_display,
        attribution_complete: posture.attribution_complete,
        scorecard_count: posture.compatibility_summary.scorecard_count,
        inherited_scorecard_count: posture.compatibility_summary.inherited_count,
        unsupported_scorecard_count: posture.compatibility_summary.unsupported_count,
        stale_scorecard_count: posture.compatibility_summary.stale_count,
        summary_label: claim.summary_label.clone(),
    }
}

// ---------------------------------------------------------------------------
// Validation helpers
// ---------------------------------------------------------------------------

fn validate_input(
    input: &StableMarketplaceCatalogTruthInput,
) -> Result<(), StableMarketplaceCatalogTruthValidationError> {
    ensure_nonempty(&input.packet_id, "packet_id")?;
    ensure_nonempty(&input.generated_at, "generated_at")?;
    ensure_nonempty(&input.summary_label, "summary_label")?;

    let id = &input.identity;
    ensure_nonempty(&id.catalog_descriptor_ref, "identity.catalog_descriptor_ref")?;
    if !id.catalog_descriptor_ref.starts_with("catalog_descriptor:") {
        return Err(err(
            "identity.catalog_descriptor_ref must start with 'catalog_descriptor:'",
        ));
    }
    ensure_nonempty(&id.row_identity_ref, "identity.row_identity_ref")?;
    ensure_nonempty(&id.extension_identity, "identity.extension_identity")?;
    ensure_nonempty(&id.extension_version, "identity.extension_version")?;
    ensure_nonempty(&id.package_id, "identity.package_id")?;
    ensure_nonempty(&id.source_package_ref, "identity.source_package_ref")?;
    ensure_token(
        TRUST_TIER_CLASSES,
        &id.publisher_trust_tier_class,
        "identity.publisher_trust_tier_class",
    )?;
    ensure_token(
        LIFECYCLE_STATE_CLASSES,
        &id.lifecycle_state_class,
        "identity.lifecycle_state_class",
    )?;

    let prov = &input.provenance;
    ensure_token(PROVENANCE_CLASSES, &prov.provenance_class, "provenance.provenance_class")?;
    ensure_nonempty(&prov.status_source_ref, "provenance.status_source_ref")?;

    let surface = &input.surface_boundary;
    ensure_token(RUNTIME_CLASSES, &surface.runtime_class, "surface_boundary.runtime_class")?;
    ensure_token(
        HOST_BOUNDARY_CLASSES,
        &surface.host_boundary_class,
        "surface_boundary.host_boundary_class",
    )?;

    let disc = &input.discoverability;
    ensure_token(
        RANKING_STATE_CLASSES,
        &disc.ranking_state_class,
        "discoverability.ranking_state_class",
    )?;
    ensure_nonempty(&disc.ranking_rationale_ref, "discoverability.ranking_rationale_ref")?;

    if input.scorecards.is_empty() {
        return Err(err("input must carry at least one compatibility scorecard"));
    }
    let mut scorecard_ids = BTreeSet::new();
    for s in &input.scorecards {
        ensure_nonempty(&s.scorecard_id, "scorecard.scorecard_id")?;
        if !scorecard_ids.insert(&s.scorecard_id) {
            return Err(err(format!("duplicate scorecard_id: {}", s.scorecard_id)));
        }
        ensure_token(SCORECARD_SUBJECT_CLASSES, &s.subject_class, "scorecard.subject_class")?;
        ensure_token(PARITY_BAND_CLASSES, &s.parity_band_class, "scorecard.parity_band_class")?;
        ensure_token(
            FRESHNESS_WINDOW_CLASSES,
            &s.freshness_window_class,
            "scorecard.freshness_window_class",
        )?;
        ensure_token(
            EVIDENCE_SOURCE_CLASSES,
            &s.evidence_source_class,
            "scorecard.evidence_source_class",
        )?;
        ensure_token(
            SCORECARD_DOWNGRADE_STATE_CLASSES,
            &s.downgrade_state_class,
            "scorecard.downgrade_state_class",
        )?;
        ensure_nonempty(&s.scorecard_ref, "scorecard.scorecard_ref")?;
    }

    let act = &input.activation_budget;
    ensure_token(ACTIVATION_BUDGET_CLASSES, &act.budget_class, "activation_budget.budget_class")?;
    ensure_nonempty(&act.measured_cost_ref, "activation_budget.measured_cost_ref")?;
    ensure_nonempty(&act.budget_ceiling_ref, "activation_budget.budget_ceiling_ref")?;

    let support = &input.support_class;
    ensure_token(
        SUPPORT_CLASS_CLASSES,
        &support.support_class_class,
        "support_class.support_class_class",
    )?;
    ensure_nonempty(
        &support.verified_runtime_profile_ref,
        "support_class.verified_runtime_profile_ref",
    )?;
    ensure_nonempty(&support.evidence_ref, "support_class.evidence_ref")?;

    let cont = &input.publisher_continuity;
    ensure_token(
        PUBLISHER_CONTINUITY_CLASSES,
        &cont.continuity_state_class,
        "publisher_continuity.continuity_state_class",
    )?;
    if cont.continuity_state_class == "current"
        && cont
            .continuity_packet_ref
            .as_ref()
            .map(|r| r.trim().is_empty())
            .unwrap_or(true)
    {
        return Err(err(
            "a current publisher continuity must bind a continuity_packet_ref",
        ));
    }

    let views = &input.view_alignment;
    for v in &views.aligned_views {
        ensure_token(CATALOG_VIEW_CLASSES, v, "view_alignment.aligned_views")?;
    }

    let claim = &input.claim;
    ensure_token(STABILITY_TIERS, &claim.claimed_tier, "claim.claimed_tier")?;
    ensure_token(CLAIM_BASIS_CLASSES, &claim.claim_basis_class, "claim.claim_basis_class")?;

    for surface in &input.consumer_surfaces {
        ensure_token(STABLE_MARKETPLACE_CATALOG_CONSUMER_SURFACES, surface, "consumer_surface")?;
    }
    if input.consumer_surfaces.is_empty() {
        return Err(err("input must bind at least one consumer surface"));
    }

    Ok(())
}

fn validate_identity(
    identity: &MarketplaceCatalogTruthIdentity,
) -> Result<(), StableMarketplaceCatalogTruthValidationError> {
    ensure_eq(
        identity.record_kind.as_str(),
        MARKETPLACE_CATALOG_TRUTH_IDENTITY_RECORD_KIND,
        "identity record_kind",
    )?;
    ensure_eq_u32(
        identity.schema_version,
        STABLE_MARKETPLACE_CATALOG_TRUTH_SCHEMA_VERSION,
        "identity schema_version",
    )?;
    ensure_token(
        TRUST_TIER_CLASSES,
        &identity.publisher_trust_tier_class,
        "identity publisher_trust_tier_class",
    )?;
    ensure_token(
        LIFECYCLE_STATE_CLASSES,
        &identity.lifecycle_state_class,
        "identity lifecycle_state_class",
    )?;
    Ok(())
}

fn validate_provenance(
    provenance: &MarketplaceCatalogProvenance,
) -> Result<(), StableMarketplaceCatalogTruthValidationError> {
    ensure_eq(
        provenance.record_kind.as_str(),
        MARKETPLACE_CATALOG_PROVENANCE_RECORD_KIND,
        "provenance record_kind",
    )?;
    ensure_token(
        PROVENANCE_CLASSES,
        &provenance.provenance_class,
        "provenance provenance_class",
    )?;
    ensure_nonempty(&provenance.status_source_ref, "provenance status_source_ref")?;
    Ok(())
}

fn validate_surface_boundary(
    surface: &MarketplaceCatalogSurfaceBoundary,
) -> Result<(), StableMarketplaceCatalogTruthValidationError> {
    ensure_eq(
        surface.record_kind.as_str(),
        MARKETPLACE_CATALOG_SURFACE_BOUNDARY_RECORD_KIND,
        "surface_boundary record_kind",
    )?;
    ensure_token(RUNTIME_CLASSES, &surface.runtime_class, "surface_boundary runtime_class")?;
    ensure_token(
        HOST_BOUNDARY_CLASSES,
        &surface.host_boundary_class,
        "surface_boundary host_boundary_class",
    )?;
    Ok(())
}

fn validate_discoverability(
    disc: &MarketplaceCatalogDiscoverabilityPosture,
) -> Result<(), StableMarketplaceCatalogTruthValidationError> {
    ensure_eq(
        disc.record_kind.as_str(),
        MARKETPLACE_CATALOG_DISCOVERABILITY_POSTURE_RECORD_KIND,
        "discoverability record_kind",
    )?;
    ensure_token(
        RANKING_STATE_CLASSES,
        &disc.ranking_state_class,
        "discoverability ranking_state_class",
    )?;
    Ok(())
}

fn validate_scorecard(
    scorecard: &MarketplaceCompatibilityScorecard,
) -> Result<(), StableMarketplaceCatalogTruthValidationError> {
    ensure_eq(
        scorecard.record_kind.as_str(),
        MARKETPLACE_COMPATIBILITY_SCORECARD_RECORD_KIND,
        "scorecard record_kind",
    )?;
    ensure_eq_u32(
        scorecard.schema_version,
        STABLE_MARKETPLACE_CATALOG_TRUTH_SCHEMA_VERSION,
        "scorecard schema_version",
    )?;
    ensure_nonempty(&scorecard.scorecard_id, "scorecard scorecard_id")?;
    ensure_token(SCORECARD_SUBJECT_CLASSES, &scorecard.subject_class, "scorecard subject_class")?;
    ensure_token(PARITY_BAND_CLASSES, &scorecard.parity_band_class, "scorecard parity_band_class")?;
    ensure_token(
        FRESHNESS_WINDOW_CLASSES,
        &scorecard.freshness_window_class,
        "scorecard freshness_window_class",
    )?;
    ensure_token(
        EVIDENCE_SOURCE_CLASSES,
        &scorecard.evidence_source_class,
        "scorecard evidence_source_class",
    )?;
    ensure_token(
        SCORECARD_DOWNGRADE_STATE_CLASSES,
        &scorecard.downgrade_state_class,
        "scorecard downgrade_state_class",
    )?;
    ensure_nonempty(&scorecard.scorecard_ref, "scorecard scorecard_ref")?;
    Ok(())
}

fn validate_compatibility_summary(
    summary: &MarketplaceCompatibilitySummary,
) -> Result<(), StableMarketplaceCatalogTruthValidationError> {
    ensure_eq(
        summary.record_kind.as_str(),
        MARKETPLACE_COMPATIBILITY_SUMMARY_RECORD_KIND,
        "compatibility_summary record_kind",
    )?;
    for subject in &summary.present_subjects {
        ensure_token(SCORECARD_SUBJECT_CLASSES, subject, "compatibility_summary.present_subjects")?;
    }
    Ok(())
}

fn validate_activation_budget(
    activation: &MarketplaceCatalogActivationBudget,
) -> Result<(), StableMarketplaceCatalogTruthValidationError> {
    ensure_eq(
        activation.record_kind.as_str(),
        MARKETPLACE_CATALOG_ACTIVATION_BUDGET_RECORD_KIND,
        "activation_budget record_kind",
    )?;
    ensure_token(
        ACTIVATION_BUDGET_CLASSES,
        &activation.budget_class,
        "activation_budget budget_class",
    )?;
    ensure_nonempty(&activation.measured_cost_ref, "activation_budget measured_cost_ref")?;
    ensure_nonempty(&activation.budget_ceiling_ref, "activation_budget budget_ceiling_ref")?;
    Ok(())
}

fn validate_support_class(
    support: &MarketplaceCatalogSupportClass,
) -> Result<(), StableMarketplaceCatalogTruthValidationError> {
    ensure_eq(
        support.record_kind.as_str(),
        MARKETPLACE_CATALOG_SUPPORT_CLASS_RECORD_KIND,
        "support_class record_kind",
    )?;
    ensure_token(
        SUPPORT_CLASS_CLASSES,
        &support.support_class_class,
        "support_class support_class_class",
    )?;
    ensure_nonempty(
        &support.verified_runtime_profile_ref,
        "support_class verified_runtime_profile_ref",
    )?;
    ensure_nonempty(&support.evidence_ref, "support_class evidence_ref")?;
    Ok(())
}

fn validate_publisher_continuity(
    continuity: &MarketplaceCatalogPublisherContinuity,
) -> Result<(), StableMarketplaceCatalogTruthValidationError> {
    ensure_eq(
        continuity.record_kind.as_str(),
        MARKETPLACE_CATALOG_PUBLISHER_CONTINUITY_RECORD_KIND,
        "publisher_continuity record_kind",
    )?;
    ensure_token(
        PUBLISHER_CONTINUITY_CLASSES,
        &continuity.continuity_state_class,
        "publisher_continuity continuity_state_class",
    )?;
    Ok(())
}

fn validate_view_alignment(
    views: &MarketplaceCatalogViewAlignment,
) -> Result<(), StableMarketplaceCatalogTruthValidationError> {
    ensure_eq(
        views.record_kind.as_str(),
        MARKETPLACE_CATALOG_VIEW_ALIGNMENT_RECORD_KIND,
        "view_alignment record_kind",
    )?;
    for v in &views.aligned_views {
        ensure_token(CATALOG_VIEW_CLASSES, v, "view_alignment aligned_views")?;
    }
    Ok(())
}

fn validate_claim(
    claim: &MarketplaceCatalogTruthQualificationClaim,
) -> Result<(), StableMarketplaceCatalogTruthValidationError> {
    ensure_eq(
        claim.record_kind.as_str(),
        MARKETPLACE_CATALOG_TRUTH_QUALIFICATION_CLAIM_RECORD_KIND,
        "claim record_kind",
    )?;
    ensure_token(STABILITY_TIERS, &claim.claimed_tier, "claim claimed_tier")?;
    ensure_token(STABILITY_TIERS, &claim.effective_tier, "claim effective_tier")?;
    ensure_token(
        SUPPORT_CLAIM_CLASSES,
        &claim.support_claim_class,
        "claim support_claim_class",
    )?;
    ensure_token(CLAIM_BASIS_CLASSES, &claim.claim_basis_class, "claim claim_basis_class")?;
    for reason in &claim.downgrade_reasons {
        ensure_token(MARKETPLACE_CATALOG_DOWNGRADE_REASONS, reason, "claim downgrade_reason")?;
    }
    Ok(())
}

fn validate_banner(
    banner: &DowngradedCatalogBanner,
) -> Result<(), StableMarketplaceCatalogTruthValidationError> {
    ensure_eq(
        banner.record_kind.as_str(),
        DOWNGRADED_CATALOG_BANNER_RECORD_KIND,
        "banner record_kind",
    )?;
    if let Some(reason) = &banner.banner_reason_class {
        ensure_token(MARKETPLACE_CATALOG_DOWNGRADE_REASONS, reason, "banner banner_reason_class")?;
        if !banner.must_display {
            return Err(err("banner_reason_class is set but must_display is false"));
        }
    } else if banner.must_display {
        return Err(err("must_display is true but no banner_reason_class is set"));
    }
    Ok(())
}

fn validate_inspection(
    inspection: &StableMarketplaceCatalogTruthInspection,
    packet: &StableMarketplaceCatalogTruthPacket,
) -> Result<(), StableMarketplaceCatalogTruthValidationError> {
    ensure_eq(
        inspection.record_kind.as_str(),
        STABLE_MARKETPLACE_CATALOG_TRUTH_INSPECTION_RECORD_KIND,
        "inspection record_kind",
    )?;
    ensure_eq(
        inspection.packet_id_ref.as_str(),
        packet.packet_id.as_str(),
        "inspection packet_id_ref",
    )?;
    ensure_eq(
        inspection.effective_tier.as_str(),
        packet.claim.effective_tier.as_str(),
        "inspection effective_tier",
    )?;
    if inspection.downgraded != packet.claim.downgraded {
        return Err(err("inspection downgraded is inconsistent"));
    }
    if inspection.downgraded_banner_required != packet.downgraded_banner.must_display {
        return Err(err("inspection downgraded_banner_required is inconsistent"));
    }
    if inspection.attribution_complete != packet.attribution_complete() {
        return Err(err("inspection attribution_complete is inconsistent"));
    }
    if inspection.scorecard_count != packet.scorecards.len() {
        return Err(err("inspection scorecard_count is inconsistent"));
    }
    if inspection.inherited_scorecard_count != packet.compatibility_summary.inherited_count {
        return Err(err("inspection inherited_scorecard_count is inconsistent"));
    }
    if inspection.unsupported_scorecard_count != packet.compatibility_summary.unsupported_count {
        return Err(err("inspection unsupported_scorecard_count is inconsistent"));
    }
    if inspection.views_aligned != packet.view_alignment.all_views_aligned() {
        return Err(err("inspection views_aligned is inconsistent"));
    }
    Ok(())
}

// ---------------------------------------------------------------------------
// Validation utilities
// ---------------------------------------------------------------------------

fn err(message: impl Into<String>) -> StableMarketplaceCatalogTruthValidationError {
    StableMarketplaceCatalogTruthValidationError {
        message: message.into(),
    }
}

fn ensure_eq<T>(
    left: T,
    right: T,
    field: &str,
) -> Result<(), StableMarketplaceCatalogTruthValidationError>
where
    T: PartialEq + fmt::Display,
{
    if left != right {
        return Err(err(format!("{field} mismatch: expected {right}, got {left}")));
    }
    Ok(())
}

fn ensure_eq_u32(
    left: u32,
    right: u32,
    field: &str,
) -> Result<(), StableMarketplaceCatalogTruthValidationError> {
    if left != right {
        return Err(err(format!("{field} mismatch: expected {right}, got {left}")));
    }
    Ok(())
}

fn ensure_nonempty(
    value: &str,
    field: &str,
) -> Result<(), StableMarketplaceCatalogTruthValidationError> {
    if value.trim().is_empty() {
        return Err(err(format!("{field} must not be empty")));
    }
    Ok(())
}

fn ensure_token(
    tokens: &[&str],
    value: &str,
    field: &str,
) -> Result<(), StableMarketplaceCatalogTruthValidationError> {
    if !tokens.contains(&value) {
        return Err(err(format!("{field} must be one of {tokens:?}, got {value}")));
    }
    Ok(())
}
