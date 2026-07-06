//! Frozen reusable workflow-bundle component matrix: start-center bundle cards,
//! certified-archetype badge groups, bundle detail pages, install/update review sheets,
//! drift banners, local-override rows, rollback/remove cards, bundle class disclosure
//! cards, and claim-narrowing rows.
//!
//! Where [`crate::m5_workflow_bundle_manifests`] freezes the canonical bundle *manifest*
//! model, [`crate::m5_bundle_scorecards`] carries *compatibility scorecard* truth,
//! [`crate::m5_bundle_review_and_rollback`] carries the *install / update / remove / drift
//! review* vocabulary, and [`crate::m5_entry_and_bundle_governance`] carries *stack-entry
//! governance* truth, this module freezes the reusable **workflow-bundle component**
//! contract: the cards, badges, banners, rows, and sheets users actually rely on when they
//! choose, review, adopt, drift, roll back, or export a supported workflow bundle, so later
//! stack-entry and migration rows reference one canonical component family instead of
//! reinventing badge meanings and stale-claim wording in registry or onboarding prose.
//!
//! One [`WorkflowBundleComponentMatrix`] packet defines every reusable primitive, its state
//! vocabulary, its required labels, and its export / assistive parity expectations, binding
//! each onto the same support-class / lifecycle, signer / source, certification-freshness,
//! mirror/offline, bundle-class, drift, local-override, and dependency-marker vocabulary
//! already used across Aureline's bundle-manifest, scorecard, review/rollback, and
//! entry-governance contracts — never a bespoke per-registry or per-archetype badge system.
//!
//! The honesty rules the spec freezes, carried by every [`ComponentRow`]:
//!
//! - **Signer / source and certification freshness stay explicit.** A start-center bundle
//!   card, detail page, or class disclosure card never hides who signed a bundle, where it
//!   came from, its support class, or how stale its certification is.
//! - **Diff scope and local-override state are never hidden.** An install / update review
//!   sheet and a local-override row keep the exact diff scope and override ownership
//!   inspectable and never apply before review.
//! - **Drift never reads like a generic package update.** A drift banner keeps bundle drift
//!   distinct from an ordinary version bump and discloses local-override state.
//! - **Rollback path and side effects are disclosed before remove.** A rollback / remove
//!   card names the rollback path and side effects before a durable removal.
//! - **Stale certification narrows claims rather than inventing wording.** A claim-narrowing
//!   row narrows on stale certification and names the reason instead of coining private
//!   stale-claim copy.
//! - **Badges and classes carry no private meaning.** A certified-archetype badge group and
//!   a bundle class disclosure card project the shared certification / class vocabulary and
//!   never mint a second badge meaning.
//!
//! Raw manifest bytes, credentials, entitlement tokens, mirror URLs, and provider cursors
//! never cross this boundary; the packet carries only typed class tokens, opaque bundle /
//! archetype / asset refs, booleans, and redacted labels, so support and diagnostics exports
//! can reconstruct exactly what a component would have shown without leaking source or live
//! payloads.
//!
//! The boundary schema is
//! [`schemas/ui/m5-workflow-bundle-component-matrix.schema.json`](../../../../schemas/ui/m5-workflow-bundle-component-matrix.schema.json).
//! The contract doc is
//! [`docs/bundles/m5_workflow_bundle_component_matrix.md`](../../../../docs/bundles/m5_workflow_bundle_component_matrix.md).
//! The protected fixture directory is
//! [`fixtures/ui/m5-workflow-bundle-components/`](../../../../fixtures/ui/m5-workflow-bundle-components/).

#[cfg(test)]
mod tests;

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

// Reused canonical bundle vocabulary — components bind to these shared enums rather than
// mint parallel badge / class / drift / freshness terms.
use crate::m5_bundle_review_and_rollback::{
    AssetOwnership, BundleReviewOperation, DiffAction, DriftState, ResolutionChoice,
};
use crate::m5_bundle_scorecards::{
    BundleScorecardClass, EvidenceFreshness, ImportedVsNativeConfidence,
};
use crate::m5_entry_and_bundle_governance::{ArchetypeConfidence, BundleClass, SourceTrust};
use crate::m5_workflow_bundle_manifests::{CertificationTarget, LifecycleStage};

/// Stable record-kind tag carried by [`WorkflowBundleComponentMatrix`].
pub const WORKFLOW_BUNDLE_COMPONENT_MATRIX_RECORD_KIND: &str =
    "m5_workflow_bundle_component_matrix";

/// Schema version for the workflow-bundle component matrix packet.
pub const WORKFLOW_BUNDLE_COMPONENT_MATRIX_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the boundary schema.
pub const WORKFLOW_BUNDLE_COMPONENT_MATRIX_SCHEMA_REF: &str =
    "schemas/ui/m5-workflow-bundle-component-matrix.schema.json";

/// Repo-relative path of the contract doc.
pub const WORKFLOW_BUNDLE_COMPONENT_MATRIX_DOC_REF: &str =
    "docs/bundles/m5_workflow_bundle_component_matrix.md";

/// Repo-relative path of the protected fixture directory.
pub const WORKFLOW_BUNDLE_COMPONENT_MATRIX_FIXTURE_DIR: &str =
    "fixtures/ui/m5-workflow-bundle-components";

/// Repo-relative path of the checked support-export artifact (the `include_str!` canonical).
pub const WORKFLOW_BUNDLE_COMPONENT_MATRIX_ARTIFACT_REF: &str =
    "artifacts/release/m5-workflow-bundle-component-proof/support_export.json";

/// Repo-relative path of the checked Markdown matrix summary.
pub const WORKFLOW_BUNDLE_COMPONENT_MATRIX_SUMMARY_REF: &str =
    "artifacts/design/m5-workflow-bundle-component-matrix.md";

/// Closed reusable workflow-bundle component family. Each family is one governed primitive
/// later stack-entry and migration rows reference by name; the matrix must define every one.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5WorkflowBundleComponentFamily {
    /// A start-center card that offers a workflow bundle for a stack.
    StartCenterBundleCard,
    /// A grouped set of certified-archetype badges.
    CertifiedArchetypeBadgeGroup,
    /// A bundle detail page describing one workflow bundle.
    BundleDetailPage,
    /// An install / update review sheet shown before applying a bundle change.
    BundleInstallUpdateReviewSheet,
    /// A drift banner for a bundle whose local state has diverged.
    BundleDriftBanner,
    /// A local-override row describing one overridden bundle-owned asset.
    BundleLocalOverrideRow,
    /// A rollback / remove card shown before durably removing a bundle.
    BundleRollbackRemoveCard,
    /// A bundle class disclosure card explaining a bundle's class and source.
    BundleClassDisclosureCard,
    /// A claim-narrowing row that narrows a bundle claim on stale certification.
    BundleClaimNarrowingRow,
}

impl M5WorkflowBundleComponentFamily {
    /// All reusable component families, in canonical order.
    pub const ALL: [Self; 9] = [
        Self::StartCenterBundleCard,
        Self::CertifiedArchetypeBadgeGroup,
        Self::BundleDetailPage,
        Self::BundleInstallUpdateReviewSheet,
        Self::BundleDriftBanner,
        Self::BundleLocalOverrideRow,
        Self::BundleRollbackRemoveCard,
        Self::BundleClassDisclosureCard,
        Self::BundleClaimNarrowingRow,
    ];

    /// Stable snake_case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::StartCenterBundleCard => "start_center_bundle_card",
            Self::CertifiedArchetypeBadgeGroup => "certified_archetype_badge_group",
            Self::BundleDetailPage => "bundle_detail_page",
            Self::BundleInstallUpdateReviewSheet => "bundle_install_update_review_sheet",
            Self::BundleDriftBanner => "bundle_drift_banner",
            Self::BundleLocalOverrideRow => "bundle_local_override_row",
            Self::BundleRollbackRemoveCard => "bundle_rollback_remove_card",
            Self::BundleClassDisclosureCard => "bundle_class_disclosure_card",
            Self::BundleClaimNarrowingRow => "bundle_claim_narrowing_row",
        }
    }
}

/// Mirror/offline provenance + freshness truth class a component binds to. Only [`Self::Live`]
/// reads as a current first-party source; everything else discloses a narrower posture.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5BundleTruthMode {
    /// A live, current, first-party bundle registry read.
    Live,
    /// A mirrored copy of a registry (freshness bounded by mirror age).
    Mirrored,
    /// A cached-offline snapshot; no live registry reachable.
    CachedOffline,
    /// Imported from another tool / user handoff; not a native registry read.
    Imported,
    /// Reported by an external provider rather than observed locally.
    ProviderSupplied,
}

impl M5BundleTruthMode {
    /// All truth modes, in canonical order.
    pub const ALL: [Self; 5] = [
        Self::Live,
        Self::Mirrored,
        Self::CachedOffline,
        Self::Imported,
        Self::ProviderSupplied,
    ];

    /// Stable snake_case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Live => "live",
            Self::Mirrored => "mirrored",
            Self::CachedOffline => "cached_offline",
            Self::Imported => "imported",
            Self::ProviderSupplied => "provider_supplied",
        }
    }

    /// Whether this truth mode reads as a current first-party source.
    pub const fn is_current_source(self) -> bool {
        matches!(self, Self::Live)
    }
}

/// Descriptor for a start-center bundle card. Keeps signer/source, support class, and
/// certification freshness explicit before a stack is chosen.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StartCenterBundleCardDescriptor {
    /// Opaque bundle id ref the card offers.
    pub bundle_id_ref: String,
    /// Bundle class this card offers.
    pub bundle_class: BundleClass,
    /// Signer / source trust of the bundle.
    pub signer_source: SourceTrust,
    /// Support-class / lifecycle stage of the bundle.
    pub lifecycle_stage: LifecycleStage,
    /// Certification target the bundle claims.
    pub certification: CertificationTarget,
    /// Certification freshness of the claim.
    pub certification_freshness: EvidenceFreshness,
    /// Compatible Aureline range the bundle declares (opaque range token).
    pub compatible_aureline_range: String,
    /// The card discloses signer / source; must hold.
    pub discloses_signer_source: bool,
    /// The card discloses certification freshness; must hold.
    pub discloses_certification_freshness: bool,
}

impl StartCenterBundleCardDescriptor {
    /// Whether this descriptor is internally honest.
    pub fn is_honest(&self) -> bool {
        self.discloses_signer_source
            && self.discloses_certification_freshness
            && !self.compatible_aureline_range.trim().is_empty()
    }
}

/// Descriptor for a certified-archetype badge group. Projects the shared certification
/// vocabulary and never mints a private badge meaning.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CertifiedArchetypeBadgeGroupDescriptor {
    /// Opaque archetype-family ref the badges describe.
    pub archetype_family_ref: String,
    /// Detected archetype confidence.
    pub archetype_confidence: ArchetypeConfidence,
    /// Certification target backing the badges.
    pub certification: CertificationTarget,
    /// Certification freshness of the badge claim.
    pub certification_freshness: EvidenceFreshness,
    /// Number of badges rendered in the group.
    pub badge_count: u32,
    /// The group invents a private badge meaning; must be false.
    pub invents_private_badge_meaning: bool,
    /// The group discloses certification freshness; must hold.
    pub discloses_certification_freshness: bool,
}

impl CertifiedArchetypeBadgeGroupDescriptor {
    /// Whether this descriptor is internally honest.
    pub fn is_honest(&self) -> bool {
        !self.invents_private_badge_meaning
            && self.discloses_certification_freshness
            && self.badge_count > 0
    }
}

/// Descriptor for a bundle detail page. Keeps signer/source, certification, compatible
/// range, entitlement/policy dependencies, and mirror/offline posture explicit.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BundleDetailPageDescriptor {
    /// Opaque bundle id ref the page describes.
    pub bundle_id_ref: String,
    /// Signer / source trust of the bundle.
    pub signer_source: SourceTrust,
    /// Certification target the bundle claims.
    pub certification: CertificationTarget,
    /// Certification freshness of the claim.
    pub certification_freshness: EvidenceFreshness,
    /// Compatible Aureline range the bundle declares (opaque range token).
    pub compatible_aureline_range: String,
    /// The page lists entitlement / policy dependencies; must hold.
    pub lists_entitlement_dependencies: bool,
    /// The page discloses mirror / offline posture; must hold.
    pub discloses_mirror_offline_posture: bool,
}

impl BundleDetailPageDescriptor {
    /// Whether this descriptor is internally honest.
    pub fn is_honest(&self) -> bool {
        self.lists_entitlement_dependencies
            && self.discloses_mirror_offline_posture
            && !self.compatible_aureline_range.trim().is_empty()
    }
}

/// Descriptor for a bundle install / update review sheet. Keeps diff scope and local-override
/// state inspectable and never applies before review.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BundleInstallUpdateReviewSheetDescriptor {
    /// Opaque bundle id ref the sheet reviews.
    pub bundle_id_ref: String,
    /// The review operation.
    pub operation: BundleReviewOperation,
    /// The dominant diff action in scope.
    pub diff_scope: DiffAction,
    /// The local-override ownership state entering review.
    pub local_override_state: AssetOwnership,
    /// The resolution choice the sheet defaults to.
    pub resolution: ResolutionChoice,
    /// The sheet reviews before applying a durable change; must hold.
    pub reviewed_before_apply: bool,
    /// The sheet discloses diff scope; must hold.
    pub discloses_diff_scope: bool,
}

impl BundleInstallUpdateReviewSheetDescriptor {
    /// Whether this descriptor is internally honest.
    pub fn is_honest(&self) -> bool {
        self.reviewed_before_apply && self.discloses_diff_scope
    }
}

/// Descriptor for a bundle drift banner. Keeps drift distinct from a generic package update
/// and discloses local-override state.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BundleDriftBannerDescriptor {
    /// Opaque bundle id ref the banner covers.
    pub bundle_id_ref: String,
    /// The drift state the banner reports.
    pub drift_state: DriftState,
    /// The local-override ownership state contributing to drift.
    pub local_override_state: AssetOwnership,
    /// The banner reads like a generic package update; must be false.
    pub reads_like_generic_package_update: bool,
    /// The banner discloses local-override state; must hold.
    pub discloses_override_state: bool,
}

impl BundleDriftBannerDescriptor {
    /// Whether this descriptor is internally honest.
    pub fn is_honest(&self) -> bool {
        !self.reads_like_generic_package_update && self.discloses_override_state
    }
}

/// Descriptor for a bundle local-override row. Keeps override ownership explicit and never
/// silently discards local work.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BundleLocalOverrideRowDescriptor {
    /// Opaque asset ref the row describes.
    pub asset_ref: String,
    /// The override ownership state of the asset.
    pub ownership: AssetOwnership,
    /// The resolution choice offered for the override.
    pub resolution: ResolutionChoice,
    /// The dominant diff action for the override.
    pub diff_scope: DiffAction,
    /// The row preserves local overrides rather than silently discarding them; must hold.
    pub preserves_local_override: bool,
    /// The row discloses ownership; must hold.
    pub discloses_ownership: bool,
}

impl BundleLocalOverrideRowDescriptor {
    /// Whether this descriptor is internally honest.
    pub fn is_honest(&self) -> bool {
        self.preserves_local_override && self.discloses_ownership
    }
}

/// Descriptor for a bundle rollback / remove card. Names the rollback path and side effects
/// before a durable removal.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BundleRollbackRemoveCardDescriptor {
    /// Opaque bundle id ref the card removes / rolls back.
    pub bundle_id_ref: String,
    /// The review operation (remove / update / install rollback).
    pub operation: BundleReviewOperation,
    /// The ownership state of the removable assets.
    pub removable_ownership: AssetOwnership,
    /// The card discloses the rollback path; must hold.
    pub discloses_rollback_path: bool,
    /// The card discloses side effects of removal; must hold.
    pub discloses_side_effects: bool,
}

impl BundleRollbackRemoveCardDescriptor {
    /// Whether this descriptor is internally honest.
    pub fn is_honest(&self) -> bool {
        self.discloses_rollback_path && self.discloses_side_effects
    }
}

/// Descriptor for a bundle class disclosure card. Explains a bundle's class and source using
/// the shared class vocabulary and never invents a private class meaning.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BundleClassDisclosureCardDescriptor {
    /// The bundle class disclosed.
    pub bundle_class: BundleClass,
    /// Signer / source trust of the bundle.
    pub signer_source: SourceTrust,
    /// Certification target the bundle claims.
    pub certification: CertificationTarget,
    /// The scorecard class the bundle carries.
    pub scorecard_class: BundleScorecardClass,
    /// The card discloses the class meaning; must hold.
    pub discloses_class_meaning: bool,
    /// The card invents a private class meaning; must be false.
    pub invents_private_class_meaning: bool,
}

impl BundleClassDisclosureCardDescriptor {
    /// Whether this descriptor is internally honest.
    pub fn is_honest(&self) -> bool {
        self.discloses_class_meaning && !self.invents_private_class_meaning
    }
}

/// Descriptor for a bundle claim-narrowing row. Narrows a bundle claim on stale certification
/// and names the reason rather than coining private stale-claim wording.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BundleClaimNarrowingRowDescriptor {
    /// Opaque bundle id ref the row narrows.
    pub bundle_id_ref: String,
    /// Certification freshness driving the narrowing.
    pub certification_freshness: EvidenceFreshness,
    /// Imported-vs-native confidence contributing to the narrowing.
    pub imported_confidence: ImportedVsNativeConfidence,
    /// The row narrows the claim on stale certification; must hold.
    pub narrows_on_stale_claim: bool,
    /// The row discloses the narrowing reason; must hold.
    pub discloses_narrowing_reason: bool,
    /// The row invents private stale-claim wording; must be false.
    pub invents_stale_wording: bool,
}

impl BundleClaimNarrowingRowDescriptor {
    /// Whether this descriptor is internally honest.
    pub fn is_honest(&self) -> bool {
        self.narrows_on_stale_claim
            && self.discloses_narrowing_reason
            && !self.invents_stale_wording
    }
}

/// Required label a workflow-bundle component renders. The mandatory subset must be present
/// on every row so no surface can drop bundle identity, signer/source, freshness, or a
/// keyboard route.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5BundleRequiredLabel {
    /// The bundle's stable identity.
    BundleIdentity,
    /// The signer / source of the bundle.
    SignerSource,
    /// The support / lifecycle class of the bundle.
    SupportClass,
    /// The certification freshness of the claim.
    CertificationFreshness,
    /// The mirror / offline posture of the source.
    MirrorOfflinePosture,
    /// The keyboard / assistive route into the component.
    KeyboardRoute,
}

impl M5BundleRequiredLabel {
    /// All required labels, in canonical order.
    pub const ALL: [Self; 6] = [
        Self::BundleIdentity,
        Self::SignerSource,
        Self::SupportClass,
        Self::CertificationFreshness,
        Self::MirrorOfflinePosture,
        Self::KeyboardRoute,
    ];

    /// The labels that must be present on every row.
    pub const MANDATORY: [Self; 4] = [
        Self::BundleIdentity,
        Self::SignerSource,
        Self::CertificationFreshness,
        Self::KeyboardRoute,
    ];

    /// Stable snake_case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::BundleIdentity => "bundle_identity",
            Self::SignerSource => "signer_source",
            Self::SupportClass => "support_class",
            Self::CertificationFreshness => "certification_freshness",
            Self::MirrorOfflinePosture => "mirror_offline_posture",
            Self::KeyboardRoute => "keyboard_route",
        }
    }
}

/// Why a workflow-bundle component is degraded / narrowed below its full claim.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5BundleComponentDowngradeTrigger {
    /// Certification is stale, so the bundle claim narrows.
    StaleCertification,
    /// The source is a stale mirror.
    MirrorStale,
    /// Only a cached-offline snapshot is available.
    OfflineCacheOnly,
    /// The signer / source is unverified.
    UnverifiedSigner,
    /// Local overrides have diverged from the bundle.
    LocalOverrideDrift,
    /// The bundle's compatible Aureline range does not cover this build.
    IncompatibleAureline,
    /// A required entitlement / policy dependency is unmet.
    EntitlementDependencyUnmet,
    /// The bundle is imported / bridged rather than native.
    ImportedNotNative,
    /// The only forward path is a rollback / removal.
    RollbackOnlyPath,
}

impl M5BundleComponentDowngradeTrigger {
    /// Stable snake_case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::StaleCertification => "stale_certification",
            Self::MirrorStale => "mirror_stale",
            Self::OfflineCacheOnly => "offline_cache_only",
            Self::UnverifiedSigner => "unverified_signer",
            Self::LocalOverrideDrift => "local_override_drift",
            Self::IncompatibleAureline => "incompatible_aureline",
            Self::EntitlementDependencyUnmet => "entitlement_dependency_unmet",
            Self::ImportedNotNative => "imported_not_native",
            Self::RollbackOnlyPath => "rollback_only_path",
        }
    }
}

/// A typed degraded-state block. When present, the component is narrowed below its full
/// capability and names why with an explicit, non-generic label.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DegradedState {
    /// Why the component is degraded.
    pub trigger: M5BundleComponentDowngradeTrigger,
    /// A precise, non-generic label safe to render.
    pub degraded_label: String,
}

impl DegradedState {
    /// Whether the degraded label is precise rather than a generic non-answer.
    pub fn is_honest(&self) -> bool {
        !label_is_generic(&self.degraded_label)
    }
}

/// One reusable workflow-bundle component: the shared truth row every stack-entry or
/// migration surface ingests instead of cloning bundle-picker / review chrome.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ComponentRow {
    /// Stable component id.
    pub component_id: String,
    /// Which reusable component family this row is.
    pub family: M5WorkflowBundleComponentFamily,
    /// Human-readable label of the surface the component appears on.
    pub surface_label: String,
    /// The mirror/offline provenance / freshness truth class the component binds to.
    pub truth_mode: M5BundleTruthMode,
    /// The bundle class the component acts on.
    pub bundle_class: BundleClass,
    /// Opaque ref to the bundle / archetype context the component acts on; never empty.
    pub bundle_context_ref: String,
    /// The required labels this component renders; must include every mandatory label.
    pub required_labels: Vec<M5BundleRequiredLabel>,
    /// The component projects an export-safe support summary; must hold.
    pub export_safe: bool,
    /// The component exposes a keyboard / assistive route; must hold.
    pub assistive_ready: bool,
    /// The start-center bundle card descriptor, present only for a card row.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub start_center_bundle_card: Option<StartCenterBundleCardDescriptor>,
    /// The certified-archetype badge group descriptor, present only for a badge-group row.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub certified_archetype_badge_group: Option<CertifiedArchetypeBadgeGroupDescriptor>,
    /// The bundle detail page descriptor, present only for a detail-page row.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bundle_detail_page: Option<BundleDetailPageDescriptor>,
    /// The install / update review sheet descriptor, present only for a review-sheet row.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bundle_install_update_review_sheet: Option<BundleInstallUpdateReviewSheetDescriptor>,
    /// The drift banner descriptor, present only for a drift-banner row.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bundle_drift_banner: Option<BundleDriftBannerDescriptor>,
    /// The local-override row descriptor, present only for an override row.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bundle_local_override_row: Option<BundleLocalOverrideRowDescriptor>,
    /// The rollback / remove card descriptor, present only for a rollback-card row.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bundle_rollback_remove_card: Option<BundleRollbackRemoveCardDescriptor>,
    /// The bundle class disclosure card descriptor, present only for a disclosure-card row.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bundle_class_disclosure_card: Option<BundleClassDisclosureCardDescriptor>,
    /// The claim-narrowing row descriptor, present only for a claim-narrowing row.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bundle_claim_narrowing_row: Option<BundleClaimNarrowingRowDescriptor>,
    /// The typed degraded-state block, present only when the component is narrowed.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub degraded: Option<DegradedState>,
    /// Human-readable label summary safe to render on the row.
    pub label_summary: String,
    /// ISO 8601 UTC timestamp the component state was observed.
    pub observed_at: String,
    /// Evidence packet refs backing this row.
    pub evidence_refs: Vec<String>,
}

impl ComponentRow {
    /// Whether the family-specific payload is present exactly for this family and absent for
    /// every other family.
    pub fn payload_matches_family(&self) -> bool {
        let present = [
            self.start_center_bundle_card.is_some(),
            self.certified_archetype_badge_group.is_some(),
            self.bundle_detail_page.is_some(),
            self.bundle_install_update_review_sheet.is_some(),
            self.bundle_drift_banner.is_some(),
            self.bundle_local_override_row.is_some(),
            self.bundle_rollback_remove_card.is_some(),
            self.bundle_class_disclosure_card.is_some(),
            self.bundle_claim_narrowing_row.is_some(),
        ];
        if present.iter().filter(|p| **p).count() != 1 {
            return false;
        }
        match self.family {
            M5WorkflowBundleComponentFamily::StartCenterBundleCard => {
                self.start_center_bundle_card.is_some()
            }
            M5WorkflowBundleComponentFamily::CertifiedArchetypeBadgeGroup => {
                self.certified_archetype_badge_group.is_some()
            }
            M5WorkflowBundleComponentFamily::BundleDetailPage => self.bundle_detail_page.is_some(),
            M5WorkflowBundleComponentFamily::BundleInstallUpdateReviewSheet => {
                self.bundle_install_update_review_sheet.is_some()
            }
            M5WorkflowBundleComponentFamily::BundleDriftBanner => {
                self.bundle_drift_banner.is_some()
            }
            M5WorkflowBundleComponentFamily::BundleLocalOverrideRow => {
                self.bundle_local_override_row.is_some()
            }
            M5WorkflowBundleComponentFamily::BundleRollbackRemoveCard => {
                self.bundle_rollback_remove_card.is_some()
            }
            M5WorkflowBundleComponentFamily::BundleClassDisclosureCard => {
                self.bundle_class_disclosure_card.is_some()
            }
            M5WorkflowBundleComponentFamily::BundleClaimNarrowingRow => {
                self.bundle_claim_narrowing_row.is_some()
            }
        }
    }

    /// Whether the family payload, where present, is internally honest.
    pub fn payload_honest(&self) -> bool {
        self.start_center_bundle_card
            .as_ref()
            .map_or(true, |d| d.is_honest())
            && self
                .certified_archetype_badge_group
                .as_ref()
                .map_or(true, |d| d.is_honest())
            && self
                .bundle_detail_page
                .as_ref()
                .map_or(true, |d| d.is_honest())
            && self
                .bundle_install_update_review_sheet
                .as_ref()
                .map_or(true, |d| d.is_honest())
            && self
                .bundle_drift_banner
                .as_ref()
                .map_or(true, |d| d.is_honest())
            && self
                .bundle_local_override_row
                .as_ref()
                .map_or(true, |d| d.is_honest())
            && self
                .bundle_rollback_remove_card
                .as_ref()
                .map_or(true, |d| d.is_honest())
            && self
                .bundle_class_disclosure_card
                .as_ref()
                .map_or(true, |d| d.is_honest())
            && self
                .bundle_claim_narrowing_row
                .as_ref()
                .map_or(true, |d| d.is_honest())
    }

    /// Whether a class-bearing descriptor discloses the same bundle class the row records
    /// (a card / disclosure never invents a second bundle story).
    pub fn descriptor_matches_row(&self) -> bool {
        let card_ok = self
            .start_center_bundle_card
            .as_ref()
            .map_or(true, |c| c.bundle_class == self.bundle_class);
        let disclosure_ok = self
            .bundle_class_disclosure_card
            .as_ref()
            .map_or(true, |c| c.bundle_class == self.bundle_class);
        card_ok && disclosure_ok
    }

    /// Whether every mandatory required label is present on the row.
    pub fn mandatory_labels_present(&self) -> bool {
        let present: BTreeSet<M5BundleRequiredLabel> =
            self.required_labels.iter().copied().collect();
        M5BundleRequiredLabel::MANDATORY
            .iter()
            .all(|label| present.contains(label))
    }

    /// Whether the degraded block, when present, is honest.
    pub fn degraded_ok(&self) -> bool {
        self.degraded.as_ref().map_or(true, |d| d.is_honest())
    }

    /// True when this row is a complete, honest degraded / narrowed component.
    pub fn is_degraded(&self) -> bool {
        self.degraded.is_some() && self.is_complete()
    }

    /// Deterministic governed chip line for this row.
    pub fn chip_tokens(&self) -> String {
        format!(
            "family={family} truth={truth} class={class} \
export_safe={export_safe} assistive={assistive}",
            family = self.family.as_str(),
            truth = self.truth_mode.as_str(),
            class = self.bundle_class.as_str(),
            export_safe = self.export_safe,
            assistive = self.assistive_ready,
        )
    }

    /// Whether every dimension required to record this row is present and internally
    /// consistent.
    pub fn is_complete(&self) -> bool {
        !self.component_id.trim().is_empty()
            && !self.surface_label.trim().is_empty()
            && !self.bundle_context_ref.trim().is_empty()
            && !self.label_summary.trim().is_empty()
            && !self.observed_at.trim().is_empty()
            && self.export_safe
            && self.assistive_ready
            && self.payload_matches_family()
            && self.payload_honest()
            && self.descriptor_matches_row()
            && self.mandatory_labels_present()
            && self.degraded_ok()
            && !self.evidence_refs.is_empty()
            && self.evidence_refs.iter().all(|r| !r.trim().is_empty())
    }
}

/// Guardrail invariants block for the workflow-bundle component matrix.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorkflowBundleComponentGuardrails {
    /// Signer / source, support class, and certification freshness stay explicit on every
    /// surface a bundle can be chosen, reviewed, or exported.
    pub signer_source_and_freshness_explicit_on_every_surface: bool,
    /// Diff scope and local-override state are never hidden on a review sheet or override row.
    pub diff_scope_and_local_override_state_never_hidden: bool,
    /// Drift banners never read like a generic package update.
    pub drift_never_reads_like_generic_package_update: bool,
    /// Rollback path and side effects are disclosed before a durable remove.
    pub rollback_path_and_side_effects_disclosed_before_remove: bool,
    /// Stale certification narrows the claim rather than being silently shown as current.
    pub stale_certification_narrows_claims_never_silently: bool,
    /// Entitlement / policy dependencies stay explicit on detail and disclosure surfaces.
    pub entitlement_policy_dependencies_stay_explicit: bool,
    /// Badges and classes carry no private meaning outside the shared vocabulary.
    pub badges_and_classes_carry_no_private_meaning: bool,
    /// Exported evidence preserves the same bundle ids, classes, and freshness shown
    /// in-product.
    pub exported_evidence_preserves_bundle_ids_classes_and_freshness: bool,
    /// Components bind to the shared bundle-manifest / scorecard / review / governance
    /// vocabulary rather than bespoke registry / onboarding chrome.
    pub components_bound_to_shared_bundle_vocabulary: bool,
    /// The matrix does not widen into new manifest formats, certification systems, or
    /// marketplace backends.
    pub no_new_manifest_formats_certification_systems_or_marketplaces: bool,
}

impl WorkflowBundleComponentGuardrails {
    /// Whether every guardrail invariant holds.
    pub fn all_hold(&self) -> bool {
        self.signer_source_and_freshness_explicit_on_every_surface
            && self.diff_scope_and_local_override_state_never_hidden
            && self.drift_never_reads_like_generic_package_update
            && self.rollback_path_and_side_effects_disclosed_before_remove
            && self.stale_certification_narrows_claims_never_silently
            && self.entitlement_policy_dependencies_stay_explicit
            && self.badges_and_classes_carry_no_private_meaning
            && self.exported_evidence_preserves_bundle_ids_classes_and_freshness
            && self.components_bound_to_shared_bundle_vocabulary
            && self.no_new_manifest_formats_certification_systems_or_marketplaces
    }
}

/// Consumer-projection block for the workflow-bundle component matrix.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorkflowBundleComponentConsumerProjection {
    /// The start center ingests these component rows instead of cloning chrome.
    pub start_center_ingests_components: bool,
    /// The migration center ingests the same component rows.
    pub migration_center_ingests_components: bool,
    /// Docs / help ingests the same component rows.
    pub docs_help_ingests_components: bool,
    /// Diagnostics ingests the same component rows.
    pub diagnostics_ingests_components: bool,
    /// Support export ingests the same component rows.
    pub support_export_ingests_components: bool,
    /// Release-control surfaces ingest the same component rows.
    pub release_control_ingests_components: bool,
    /// Later stack-entry / migration rows reference one canonical component family instead
    /// of restating bundle truth in registry / onboarding prose.
    pub later_rows_reference_one_canonical_family: bool,
}

impl WorkflowBundleComponentConsumerProjection {
    /// Whether every consumer-projection invariant holds.
    pub fn all_hold(&self) -> bool {
        self.start_center_ingests_components
            && self.migration_center_ingests_components
            && self.docs_help_ingests_components
            && self.diagnostics_ingests_components
            && self.support_export_ingests_components
            && self.release_control_ingests_components
            && self.later_rows_reference_one_canonical_family
    }
}

/// Constructor input for [`WorkflowBundleComponentMatrix::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WorkflowBundleComponentMatrixInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable matrix label.
    pub set_label: String,
    /// Per-component rows.
    pub components: Vec<ComponentRow>,
    /// Guardrail invariants block.
    pub guardrails: WorkflowBundleComponentGuardrails,
    /// Consumer projection block.
    pub consumer_projection: WorkflowBundleComponentConsumerProjection,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

/// Export-safe workflow-bundle component matrix packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorkflowBundleComponentMatrix {
    /// Record kind; must equal [`WORKFLOW_BUNDLE_COMPONENT_MATRIX_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`WORKFLOW_BUNDLE_COMPONENT_MATRIX_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable matrix label.
    pub set_label: String,
    /// Per-component rows.
    pub components: Vec<ComponentRow>,
    /// Guardrail invariants block.
    pub guardrails: WorkflowBundleComponentGuardrails,
    /// Consumer projection block.
    pub consumer_projection: WorkflowBundleComponentConsumerProjection,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

impl WorkflowBundleComponentMatrix {
    /// Builds a workflow-bundle component matrix packet.
    pub fn new(input: WorkflowBundleComponentMatrixInput) -> Self {
        Self {
            record_kind: WORKFLOW_BUNDLE_COMPONENT_MATRIX_RECORD_KIND.to_owned(),
            schema_version: WORKFLOW_BUNDLE_COMPONENT_MATRIX_SCHEMA_VERSION,
            packet_id: input.packet_id,
            set_label: input.set_label,
            components: input.components,
            guardrails: input.guardrails,
            consumer_projection: input.consumer_projection,
            source_contract_refs: input.source_contract_refs,
            redaction_class_token: input.redaction_class_token,
            minted_at: input.minted_at,
        }
    }

    /// Families represented by some row in this matrix.
    pub fn represented_families(&self) -> BTreeSet<M5WorkflowBundleComponentFamily> {
        self.components.iter().map(|r| r.family).collect()
    }

    /// Count of rows that are complete, honest degraded / narrowed components.
    pub fn degraded_row_count(&self) -> usize {
        self.components.iter().filter(|r| r.is_degraded()).count()
    }

    /// Validates the workflow-bundle component matrix invariants.
    pub fn validate(&self) -> Vec<WorkflowBundleComponentViolation> {
        let mut violations = Vec::new();

        if self.record_kind != WORKFLOW_BUNDLE_COMPONENT_MATRIX_RECORD_KIND {
            violations.push(WorkflowBundleComponentViolation::WrongRecordKind);
        }
        if self.schema_version != WORKFLOW_BUNDLE_COMPONENT_MATRIX_SCHEMA_VERSION {
            violations.push(WorkflowBundleComponentViolation::WrongSchemaVersion);
        }
        if self.packet_id.trim().is_empty()
            || self.set_label.trim().is_empty()
            || self.redaction_class_token.trim().is_empty()
            || self.minted_at.trim().is_empty()
        {
            violations.push(WorkflowBundleComponentViolation::MissingIdentity);
        }

        validate_source_contracts(self, &mut violations);
        validate_coverage(self, &mut violations);
        validate_rows(self, &mut violations);
        validate_guardrails(self, &mut violations);
        validate_consumer_projection(self, &mut violations);

        if json_contains_forbidden_boundary_material(
            &serde_json::to_value(self).expect("workflow-bundle component matrix serializes"),
        ) {
            violations.push(WorkflowBundleComponentViolation::RawBoundaryMaterialInExport);
        }

        violations
    }

    /// Deterministic export-safe JSON.
    ///
    /// # Panics
    ///
    /// Panics only if serializing this metadata-only packet fails.
    pub fn export_safe_json(&self) -> String {
        serde_json::to_string_pretty(self).expect("workflow-bundle component matrix serializes")
    }

    /// Deterministic CSV of the component rows for release / support handoff.
    pub fn render_matrix_csv(&self) -> String {
        let mut out = String::from(
            "component_id,family,truth_mode,bundle_class,export_safe,assistive_ready,degraded\n",
        );
        for row in &self.components {
            out.push_str(&format!(
                "{id},{family},{truth},{class},{export_safe},{assistive},{degraded}\n",
                id = row.component_id,
                family = row.family.as_str(),
                truth = row.truth_mode.as_str(),
                class = row.bundle_class.as_str(),
                export_safe = row.export_safe,
                assistive = row.assistive_ready,
                degraded = row.degraded.as_ref().map_or("none", |d| d.trigger.as_str()),
            ));
        }
        out
    }

    /// Deterministic Markdown summary for support, docs, or release handoff.
    pub fn render_markdown_summary(&self) -> String {
        let mut out = String::new();
        out.push_str("# M5 Workflow-Bundle Component Matrix\n\n");
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- Label: `{}`\n", self.set_label));
        out.push_str(&format!(
            "- Components: {} across {} / {} families ({} degraded)\n",
            self.components.len(),
            self.represented_families().len(),
            M5WorkflowBundleComponentFamily::ALL.len(),
            self.degraded_row_count(),
        ));
        out.push_str("\n## Components\n\n");
        for row in &self.components {
            out.push_str(&format!(
                "- **{}** ({}) — {}\n",
                row.component_id,
                row.family.as_str(),
                row.surface_label,
            ));
            out.push_str(&format!("  - {}\n", row.label_summary));
            out.push_str(&format!("  - {}\n", row.chip_tokens()));
            if let Some(degraded) = &row.degraded {
                out.push_str(&format!(
                    "  - Degraded: trigger={} — {}\n",
                    degraded.trigger.as_str(),
                    degraded.degraded_label,
                ));
            }
        }
        out
    }
}

/// A workflow-bundle component matrix invariant violation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WorkflowBundleComponentViolation {
    /// Packet record kind is wrong.
    WrongRecordKind,
    /// Packet schema version is wrong.
    WrongSchemaVersion,
    /// Required identity field is missing.
    MissingIdentity,
    /// Required base source contract refs are incomplete.
    MissingSourceContracts,
    /// A required reusable component family is defined by no row.
    RequiredFamilyMissing,
    /// The matrix demonstrates no complete degraded / narrowed row.
    DegradedCaseMissing,
    /// A row is incomplete.
    RowIncomplete,
    /// A row's family-specific payload is missing, extra, or wrong for its family.
    PayloadFamilyMismatch,
    /// A row's family payload is internally dishonest.
    PayloadDishonest,
    /// A class-bearing descriptor discloses a class different from its row.
    DescriptorRowMismatch,
    /// A row omits a mandatory required label.
    MandatoryLabelMissing,
    /// A row is not export-safe or not assistive-ready.
    ParityMissing,
    /// A degraded block carries a generic non-answer label.
    DegradedLabelGeneric,
    /// A row lacks evidence refs.
    RowEvidenceMissing,
    /// Guardrail block does not satisfy required invariants.
    GuardrailsIncomplete,
    /// Consumer projection does not satisfy required invariants.
    ConsumerProjectionIncomplete,
    /// Export contains raw boundary material.
    RawBoundaryMaterialInExport,
}

impl WorkflowBundleComponentViolation {
    /// Stable token used in tests and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WrongRecordKind => "wrong_record_kind",
            Self::WrongSchemaVersion => "wrong_schema_version",
            Self::MissingIdentity => "missing_identity",
            Self::MissingSourceContracts => "missing_source_contracts",
            Self::RequiredFamilyMissing => "required_family_missing",
            Self::DegradedCaseMissing => "degraded_case_missing",
            Self::RowIncomplete => "row_incomplete",
            Self::PayloadFamilyMismatch => "payload_family_mismatch",
            Self::PayloadDishonest => "payload_dishonest",
            Self::DescriptorRowMismatch => "descriptor_row_mismatch",
            Self::MandatoryLabelMissing => "mandatory_label_missing",
            Self::ParityMissing => "parity_missing",
            Self::DegradedLabelGeneric => "degraded_label_generic",
            Self::RowEvidenceMissing => "row_evidence_missing",
            Self::GuardrailsIncomplete => "guardrails_incomplete",
            Self::ConsumerProjectionIncomplete => "consumer_projection_incomplete",
            Self::RawBoundaryMaterialInExport => "raw_boundary_material_in_export",
        }
    }
}

/// An error reading or validating the checked-in workflow-bundle component export.
#[derive(Debug)]
pub enum WorkflowBundleComponentArtifactError {
    /// The support export could not be parsed.
    SupportExport(serde_json::Error),
    /// The parsed support export failed validation.
    Validation(Vec<WorkflowBundleComponentViolation>),
}

impl fmt::Display for WorkflowBundleComponentArtifactError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(err) => {
                write!(
                    f,
                    "workflow-bundle component support export parse error: {err}"
                )
            }
            Self::Validation(violations) => {
                let tokens: Vec<&str> = violations.iter().map(|v| v.clone().as_str()).collect();
                write!(
                    f,
                    "workflow-bundle component support export failed validation: {}",
                    tokens.join(", ")
                )
            }
        }
    }
}

impl Error for WorkflowBundleComponentArtifactError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::SupportExport(err) => Some(err),
            Self::Validation(_) => None,
        }
    }
}

/// Reads and validates the checked-in workflow-bundle component export.
///
/// # Errors
///
/// Returns an artifact error if the export cannot parse or fails validation.
pub fn current_m5_workflow_bundle_component_matrix_export(
) -> Result<WorkflowBundleComponentMatrix, WorkflowBundleComponentArtifactError> {
    let packet: WorkflowBundleComponentMatrix = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/release/m5-workflow-bundle-component-proof/support_export.json"
    )))
    .map_err(WorkflowBundleComponentArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(WorkflowBundleComponentArtifactError::Validation(violations))
    }
}

fn validate_source_contracts(
    packet: &WorkflowBundleComponentMatrix,
    violations: &mut Vec<WorkflowBundleComponentViolation>,
) {
    let refs: BTreeSet<&str> = packet
        .source_contract_refs
        .iter()
        .map(String::as_str)
        .collect();
    for required in [
        WORKFLOW_BUNDLE_COMPONENT_MATRIX_SCHEMA_REF,
        WORKFLOW_BUNDLE_COMPONENT_MATRIX_DOC_REF,
        WORKFLOW_BUNDLE_COMPONENT_MATRIX_ARTIFACT_REF,
    ] {
        if !refs.contains(required) {
            violations.push(WorkflowBundleComponentViolation::MissingSourceContracts);
            break;
        }
    }
}

fn validate_coverage(
    packet: &WorkflowBundleComponentMatrix,
    violations: &mut Vec<WorkflowBundleComponentViolation>,
) {
    let families = packet.represented_families();
    for required in M5WorkflowBundleComponentFamily::ALL {
        if !families.contains(&required) {
            violations.push(WorkflowBundleComponentViolation::RequiredFamilyMissing);
            break;
        }
    }
    if packet.degraded_row_count() == 0 {
        violations.push(WorkflowBundleComponentViolation::DegradedCaseMissing);
    }
}

fn validate_rows(
    packet: &WorkflowBundleComponentMatrix,
    violations: &mut Vec<WorkflowBundleComponentViolation>,
) {
    for row in &packet.components {
        if !row.is_complete() {
            violations.push(WorkflowBundleComponentViolation::RowIncomplete);
        }
        if !row.payload_matches_family() {
            violations.push(WorkflowBundleComponentViolation::PayloadFamilyMismatch);
        }
        if !row.payload_honest() {
            violations.push(WorkflowBundleComponentViolation::PayloadDishonest);
        }
        if !row.descriptor_matches_row() {
            violations.push(WorkflowBundleComponentViolation::DescriptorRowMismatch);
        }
        if !row.mandatory_labels_present() {
            violations.push(WorkflowBundleComponentViolation::MandatoryLabelMissing);
        }
        if !row.export_safe || !row.assistive_ready {
            violations.push(WorkflowBundleComponentViolation::ParityMissing);
        }
        if !row.degraded_ok() {
            violations.push(WorkflowBundleComponentViolation::DegradedLabelGeneric);
        }
        if row.evidence_refs.is_empty() || row.evidence_refs.iter().any(|r| r.trim().is_empty()) {
            violations.push(WorkflowBundleComponentViolation::RowEvidenceMissing);
        }
    }
}

fn validate_guardrails(
    packet: &WorkflowBundleComponentMatrix,
    violations: &mut Vec<WorkflowBundleComponentViolation>,
) {
    if !packet.guardrails.all_hold() {
        violations.push(WorkflowBundleComponentViolation::GuardrailsIncomplete);
    }
}

fn validate_consumer_projection(
    packet: &WorkflowBundleComponentMatrix,
    violations: &mut Vec<WorkflowBundleComponentViolation>,
) {
    if !packet.consumer_projection.all_hold() {
        violations.push(WorkflowBundleComponentViolation::ConsumerProjectionIncomplete);
    }
}

/// Whether a degraded label is a generic non-answer rather than a precise label.
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
            | "stale"
            | "no data"
            | "blocked"
            | "degraded"
            | "offline"
            | "drift"
            | "outdated"
    )
}

/// Heuristic that rejects obviously forbidden material in export-safe JSON.
fn json_contains_forbidden_boundary_material(value: &serde_json::Value) -> bool {
    match value {
        serde_json::Value::String(s) => {
            let lower = s.to_lowercase();
            lower.contains("api_key")
                || lower.contains("password")
                || lower.contains("secret")
                || lower.contains("-----begin")
                || lower.contains("bearer ")
                || lower.contains("://")
        }
        serde_json::Value::Array(arr) => arr.iter().any(json_contains_forbidden_boundary_material),
        serde_json::Value::Object(map) => {
            map.values().any(json_contains_forbidden_boundary_material)
        }
        _ => false,
    }
}

/// Builds the canonical, checked-in workflow-bundle component matrix packet. This is the one
/// source of truth shared by the tests and the on-disk support export so both stay
/// byte-aligned.
pub fn seeded_workflow_bundle_component_matrix() -> WorkflowBundleComponentMatrix {
    WorkflowBundleComponentMatrix::new(WorkflowBundleComponentMatrixInput {
        packet_id: "m5-workflow-bundle-component-matrix:stable:0001".to_owned(),
        set_label: "M5 Workflow-Bundle Component Matrix".to_owned(),
        components: seeded_components(),
        guardrails: seeded_guardrails(),
        consumer_projection: seeded_consumer_projection(),
        source_contract_refs: seeded_source_contract_refs(),
        redaction_class_token: "metadata_safe_default".to_owned(),
        minted_at: "2026-07-06T00:00:00Z".to_owned(),
    })
}

fn seeded_guardrails() -> WorkflowBundleComponentGuardrails {
    WorkflowBundleComponentGuardrails {
        signer_source_and_freshness_explicit_on_every_surface: true,
        diff_scope_and_local_override_state_never_hidden: true,
        drift_never_reads_like_generic_package_update: true,
        rollback_path_and_side_effects_disclosed_before_remove: true,
        stale_certification_narrows_claims_never_silently: true,
        entitlement_policy_dependencies_stay_explicit: true,
        badges_and_classes_carry_no_private_meaning: true,
        exported_evidence_preserves_bundle_ids_classes_and_freshness: true,
        components_bound_to_shared_bundle_vocabulary: true,
        no_new_manifest_formats_certification_systems_or_marketplaces: true,
    }
}

fn seeded_consumer_projection() -> WorkflowBundleComponentConsumerProjection {
    WorkflowBundleComponentConsumerProjection {
        start_center_ingests_components: true,
        migration_center_ingests_components: true,
        docs_help_ingests_components: true,
        diagnostics_ingests_components: true,
        support_export_ingests_components: true,
        release_control_ingests_components: true,
        later_rows_reference_one_canonical_family: true,
    }
}

fn seeded_source_contract_refs() -> Vec<String> {
    vec![
        WORKFLOW_BUNDLE_COMPONENT_MATRIX_SCHEMA_REF.to_owned(),
        WORKFLOW_BUNDLE_COMPONENT_MATRIX_DOC_REF.to_owned(),
        WORKFLOW_BUNDLE_COMPONENT_MATRIX_ARTIFACT_REF.to_owned(),
        WORKFLOW_BUNDLE_COMPONENT_MATRIX_FIXTURE_DIR.to_owned(),
        WORKFLOW_BUNDLE_COMPONENT_MATRIX_SUMMARY_REF.to_owned(),
        "crates/aureline-workspace/src/m5_workflow_bundle_manifests/mod.rs".to_owned(),
        "crates/aureline-workspace/src/m5_bundle_scorecards/mod.rs".to_owned(),
        "crates/aureline-workspace/src/m5_bundle_review_and_rollback/mod.rs".to_owned(),
        "crates/aureline-workspace/src/m5_entry_and_bundle_governance/mod.rs".to_owned(),
    ]
}

fn ev(id: &str) -> Vec<String> {
    vec![format!("evidence:workflow-bundle-component:{id}")]
}

fn mandatory_labels() -> Vec<M5BundleRequiredLabel> {
    M5BundleRequiredLabel::ALL.to_vec()
}

#[allow(clippy::too_many_arguments)]
fn base_row(
    component_id: &str,
    family: M5WorkflowBundleComponentFamily,
    surface_label: &str,
    modes: (M5BundleTruthMode, BundleClass),
    context_ref: &str,
    label_summary: &str,
    evidence_id: &str,
) -> ComponentRow {
    let (truth_mode, bundle_class) = modes;
    ComponentRow {
        component_id: component_id.to_owned(),
        family,
        surface_label: surface_label.to_owned(),
        truth_mode,
        bundle_class,
        bundle_context_ref: context_ref.to_owned(),
        required_labels: mandatory_labels(),
        export_safe: true,
        assistive_ready: true,
        start_center_bundle_card: None,
        certified_archetype_badge_group: None,
        bundle_detail_page: None,
        bundle_install_update_review_sheet: None,
        bundle_drift_banner: None,
        bundle_local_override_row: None,
        bundle_rollback_remove_card: None,
        bundle_class_disclosure_card: None,
        bundle_claim_narrowing_row: None,
        degraded: None,
        label_summary: label_summary.to_owned(),
        observed_at: "2026-07-06T00:00:00Z".to_owned(),
        evidence_refs: ev(evidence_id),
    }
}

fn seeded_components() -> Vec<ComponentRow> {
    let mut rows = Vec::new();

    // Start-center bundle card — a certified, current, first-party launch bundle.
    let mut row = base_row(
        "component:start-center-bundle-card:0001",
        M5WorkflowBundleComponentFamily::StartCenterBundleCard,
        "Start-center card offering a certified launch bundle",
        (M5BundleTruthMode::Live, BundleClass::LaunchBundle),
        "bundle_context:launch:0001",
        "A start-center bundle card keeps signer/source, support class, and certification freshness explicit before a stack is chosen",
        "start-center-bundle-card:0001",
    );
    row.start_center_bundle_card = Some(StartCenterBundleCardDescriptor {
        bundle_id_ref: "bundle:launch:0001".to_owned(),
        bundle_class: BundleClass::LaunchBundle,
        signer_source: SourceTrust::FirstParty,
        lifecycle_stage: LifecycleStage::Stable,
        certification: CertificationTarget::Certified,
        certification_freshness: EvidenceFreshness::Fresh,
        compatible_aureline_range: "1.4.x-1.7.x".to_owned(),
        discloses_signer_source: true,
        discloses_certification_freshness: true,
    });
    rows.push(row);

    // Start-center bundle card — an imported bundle read from a stale mirror, narrows.
    let mut row = base_row(
        "component:start-center-bundle-card:0002",
        M5WorkflowBundleComponentFamily::StartCenterBundleCard,
        "Start-center card offering an imported bundle from a stale mirror",
        (M5BundleTruthMode::Mirrored, BundleClass::ImportedHandoffBundle),
        "bundle_context:launch:0002",
        "A start-center bundle card discloses that this imported bundle was read from a stale mirror rather than imply a live certified source",
        "start-center-bundle-card:0002",
    );
    row.start_center_bundle_card = Some(StartCenterBundleCardDescriptor {
        bundle_id_ref: "bundle:imported:0002".to_owned(),
        bundle_class: BundleClass::ImportedHandoffBundle,
        signer_source: SourceTrust::TrustedRemote,
        lifecycle_stage: LifecycleStage::MirrorOnly,
        certification: CertificationTarget::ImportedPendingReview,
        certification_freshness: EvidenceFreshness::Aging,
        compatible_aureline_range: "1.5.x-1.7.x".to_owned(),
        discloses_signer_source: true,
        discloses_certification_freshness: true,
    });
    row.degraded = Some(DegradedState {
        trigger: M5BundleComponentDowngradeTrigger::MirrorStale,
        degraded_label: "This bundle was read from a mirror last refreshed beyond its freshness window; the card names the mirror age and offers a refresh route".to_owned(),
    });
    rows.push(row);

    // Certified-archetype badge group — a confirmed Rust workspace archetype.
    let mut row = base_row(
        "component:certified-archetype-badge-group:0001",
        M5WorkflowBundleComponentFamily::CertifiedArchetypeBadgeGroup,
        "Certified-archetype badge group for a confirmed Rust workspace",
        (M5BundleTruthMode::Live, BundleClass::FrameworkPack),
        "bundle_context:archetype:0001",
        "A certified-archetype badge group projects the shared certification vocabulary and never mints a private badge meaning",
        "certified-archetype-badge-group:0001",
    );
    row.certified_archetype_badge_group = Some(CertifiedArchetypeBadgeGroupDescriptor {
        archetype_family_ref: "rust_workspace".to_owned(),
        archetype_confidence: ArchetypeConfidence::Confirmed,
        certification: CertificationTarget::Certified,
        certification_freshness: EvidenceFreshness::Fresh,
        badge_count: 3,
        invents_private_badge_meaning: false,
        discloses_certification_freshness: true,
    });
    rows.push(row);

    // Bundle detail page — a managed-approved bundle listing entitlement dependencies.
    let mut row = base_row(
        "component:bundle-detail-page:0001",
        M5WorkflowBundleComponentFamily::BundleDetailPage,
        "Bundle detail page for a managed-approved bundle",
        (M5BundleTruthMode::Live, BundleClass::OrgManagedBundle),
        "bundle_context:detail:0001",
        "A bundle detail page keeps signer/source, certification, compatible range, entitlement dependencies, and mirror/offline posture explicit",
        "bundle-detail-page:0001",
    );
    row.bundle_detail_page = Some(BundleDetailPageDescriptor {
        bundle_id_ref: "bundle:managed:0001".to_owned(),
        signer_source: SourceTrust::FirstParty,
        certification: CertificationTarget::ManagedApproved,
        certification_freshness: EvidenceFreshness::Fresh,
        compatible_aureline_range: "1.6.x-1.7.x".to_owned(),
        lists_entitlement_dependencies: true,
        discloses_mirror_offline_posture: true,
    });
    rows.push(row);

    // Install / update review sheet — an update reviewed before apply, diff scope shown.
    let mut row = base_row(
        "component:bundle-install-update-review-sheet:0001",
        M5WorkflowBundleComponentFamily::BundleInstallUpdateReviewSheet,
        "Install/update review sheet for a bundle update",
        (M5BundleTruthMode::Live, BundleClass::LaunchBundle),
        "bundle_context:review:0001",
        "An install/update review sheet keeps diff scope and local-override state inspectable and never applies before review",
        "bundle-install-update-review-sheet:0001",
    );
    row.bundle_install_update_review_sheet = Some(BundleInstallUpdateReviewSheetDescriptor {
        bundle_id_ref: "bundle:launch:0001".to_owned(),
        operation: BundleReviewOperation::Update,
        diff_scope: DiffAction::Modified,
        local_override_state: AssetOwnership::LocallyOverridden,
        resolution: ResolutionChoice::Compare,
        reviewed_before_apply: true,
        discloses_diff_scope: true,
    });
    rows.push(row);

    // Drift banner — a diverged bundle, distinct from a generic package update, narrows.
    let mut row = base_row(
        "component:bundle-drift-banner:0001",
        M5WorkflowBundleComponentFamily::BundleDriftBanner,
        "Drift banner for a diverged bundle",
        (M5BundleTruthMode::Live, BundleClass::LaunchBundle),
        "bundle_context:drift:0001",
        "A drift banner keeps bundle drift distinct from a generic package update and discloses local-override state",
        "bundle-drift-banner:0001",
    );
    row.bundle_drift_banner = Some(BundleDriftBannerDescriptor {
        bundle_id_ref: "bundle:launch:0001".to_owned(),
        drift_state: DriftState::Diverged,
        local_override_state: AssetOwnership::LocallyOverridden,
        reads_like_generic_package_update: false,
        discloses_override_state: true,
    });
    row.degraded = Some(DegradedState {
        trigger: M5BundleComponentDowngradeTrigger::LocalOverrideDrift,
        degraded_label: "Local overrides on this bundle have diverged from the certified revision; the banner names the diverged assets and offers a compare/adopt route".to_owned(),
    });
    rows.push(row);

    // Local-override row — a locally overridden asset preserved, ownership disclosed.
    let mut row = base_row(
        "component:bundle-local-override-row:0001",
        M5WorkflowBundleComponentFamily::BundleLocalOverrideRow,
        "Local-override row for an overridden bundle-owned asset",
        (M5BundleTruthMode::Live, BundleClass::LaunchBundle),
        "bundle_context:override:0001",
        "A local-override row keeps override ownership explicit and never silently discards local work",
        "bundle-local-override-row:0001",
    );
    row.bundle_local_override_row = Some(BundleLocalOverrideRowDescriptor {
        asset_ref: "asset:launch-recipe:0001".to_owned(),
        ownership: AssetOwnership::LocallyOverridden,
        resolution: ResolutionChoice::KeepLocal,
        diff_scope: DiffAction::Modified,
        preserves_local_override: true,
        discloses_ownership: true,
    });
    rows.push(row);

    // Rollback / remove card — a removal with rollback path and side effects disclosed.
    let mut row = base_row(
        "component:bundle-rollback-remove-card:0001",
        M5WorkflowBundleComponentFamily::BundleRollbackRemoveCard,
        "Rollback/remove card for a bundle removal",
        (M5BundleTruthMode::Live, BundleClass::TemplateBundle),
        "bundle_context:rollback:0001",
        "A rollback/remove card names the rollback path and side effects before a durable removal",
        "bundle-rollback-remove-card:0001",
    );
    row.bundle_rollback_remove_card = Some(BundleRollbackRemoveCardDescriptor {
        bundle_id_ref: "bundle:template:0001".to_owned(),
        operation: BundleReviewOperation::Remove,
        removable_ownership: AssetOwnership::Removable,
        discloses_rollback_path: true,
        discloses_side_effects: true,
    });
    rows.push(row);

    // Bundle class disclosure card — a community-reviewed template bundle class explained.
    let mut row = base_row(
        "component:bundle-class-disclosure-card:0001",
        M5WorkflowBundleComponentFamily::BundleClassDisclosureCard,
        "Bundle class disclosure card for a community template bundle",
        (M5BundleTruthMode::Live, BundleClass::TemplateBundle),
        "bundle_context:class:0001",
        "A bundle class disclosure card explains a bundle's class and source using the shared class vocabulary and never invents a private class meaning",
        "bundle-class-disclosure-card:0001",
    );
    row.bundle_class_disclosure_card = Some(BundleClassDisclosureCardDescriptor {
        bundle_class: BundleClass::TemplateBundle,
        signer_source: SourceTrust::UnverifiedRemote,
        certification: CertificationTarget::CommunityReviewed,
        scorecard_class: BundleScorecardClass::Community,
        discloses_class_meaning: true,
        invents_private_class_meaning: false,
    });
    rows.push(row);

    // Claim-narrowing row — a bundle whose stale certification narrows its claim, narrows.
    let mut row = base_row(
        "component:bundle-claim-narrowing-row:0001",
        M5WorkflowBundleComponentFamily::BundleClaimNarrowingRow,
        "Claim-narrowing row for a bundle with stale certification",
        (M5BundleTruthMode::Imported, BundleClass::ImportedHandoffBundle),
        "bundle_context:narrowing:0001",
        "A claim-narrowing row narrows a bundle claim on stale certification and names the reason rather than coining private stale-claim wording",
        "bundle-claim-narrowing-row:0001",
    );
    row.bundle_claim_narrowing_row = Some(BundleClaimNarrowingRowDescriptor {
        bundle_id_ref: "bundle:imported:0002".to_owned(),
        certification_freshness: EvidenceFreshness::Stale,
        imported_confidence: ImportedVsNativeConfidence::Bridged,
        narrows_on_stale_claim: true,
        discloses_narrowing_reason: true,
        invents_stale_wording: false,
    });
    row.degraded = Some(DegradedState {
        trigger: M5BundleComponentDowngradeTrigger::StaleCertification,
        degraded_label: "This imported bundle's certification is past its freshness window, so the row narrows the claim to bridged and names the required re-certification".to_owned(),
    });
    rows.push(row);

    rows
}
