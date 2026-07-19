//! Keyboard / screen-reader / CLI / export parity and honest auto-narrowing for
//! the M5 workflow-bundle components.
//!
//! This module is the M05-850 accessibility-and-auto-narrowing capstone over the
//! frozen M5 workflow-bundle component matrix
//! ([`crate::freeze_the_m5_workflow_bundle_launch_badge_detail_review_drift_and_rollback_component_matrix`]).
//! Where the freeze matrix defines the reusable start-center bundle card,
//! certified-archetype badge group, bundle detail page, install / update review
//! sheet, drift banner, local-override row, rollback / remove card, class-disclosure
//! card, and claim-narrowing row primitives, and the 845-849 implementation lanes
//! resolve their per-surface truth, this lane certifies — per component family —
//! that workflow-bundle claims stay **keyboard-complete, assistive-tech-reachable,
//! CLI/export-safe, and self-narrowing** rather than presenting a stale, partial,
//! offline, or policy-blocked bundle as fully certified or fully self-sufficient:
//!
//! - **Keyboard / screen-reader / CLI reach.** Every family exposes a
//!   keyboard-complete, screen-reader-reachable, and CLI/headless-reachable path
//!   into the same bundle identity, signer / source class, certification freshness,
//!   drift state, and rollback path the rich surface shows — never a view-only card
//!   that strands assistive-tech or headless users. Hierarchy-heavy families (the
//!   bundle detail page's component / dependency inventory) additionally bind their
//!   tree to a flat list / textual path.
//! - **Export parity.** The support / release export reconstructs each component's
//!   meaning from typed tokens and opaque refs without a screenshot, preserving the
//!   same bundle IDs, source classes, evidence ages, and drift states shown
//!   in-product.
//! - **Honest auto-narrowing.** When bundle freshness, certification evidence,
//!   source provenance, artifact availability, or dependency posture is partial,
//!   stale, imported, mirror-only, offline, or policy-blocked, the component's
//!   bundle-support claim auto-narrows to supported / limited / retest-pending /
//!   imported / mirror-only / offline-cache-only / policy-blocked, discloses the
//!   narrowing with a precise trigger and binding dimension, and preserves the
//!   canonical bundle identity rather than silently dropping it. A component with
//!   every dimension intact must NOT carry a spurious narrowing.
//! - **Cross-surface disclosure.** The same narrowed state surfaces in UI,
//!   docs/help, migration packets, diagnostics, and support/admin exports so claim
//!   publication and field triage stay aligned on workflow-bundle downgrade
//!   behavior.
//!
//! Each [`BundleAccessibilityRow`] keys on one
//! [`crate::freeze_the_m5_workflow_bundle_launch_badge_detail_review_drift_and_rollback_component_matrix::M5WorkflowBundleComponentFamily`]
//! and reuses that frozen family vocabulary plus the frozen
//! [`M5BundleRequiredLabel`] and [`M5BundleComponentDowngradeTrigger`] and the shared
//! [`M5BundleDisclosureSurfaceFamily`] consumer surfaces rather than minting parallel
//! synonyms, so the certified labels stay byte-identical to the matrix and the
//! sibling primitive packets.
//!
//! The packet is metadata-only: raw manifests, credentials, entitlement tokens, and
//! provider cursors never cross this boundary; the packet carries only typed class
//! tokens, opaque summary / evidence refs, booleans, and redacted labels so support
//! and diagnostics exports can reconstruct exactly what an accessible fallback would
//! have shown without leaking bundle state.
//!
//! The boundary schema is
//! [`schemas/ui/m5-workflow-bundle-component-accessibility-fallback.schema.json`](../../../../schemas/ui/m5-workflow-bundle-component-accessibility-fallback.schema.json).
//! The contract doc is
//! [`docs/bundles/m5_workflow_bundle_component_accessibility_fallback.md`](../../../../docs/bundles/m5_workflow_bundle_component_accessibility_fallback.md).

#[cfg(test)]
mod tests;

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

// Reused frozen component vocabulary — the capstone certifies the freeze matrix's
// families, required labels, and downgrade triggers rather than mint parallel ones.
use crate::freeze_the_m5_workflow_bundle_launch_badge_detail_review_drift_and_rollback_component_matrix::{
    M5BundleComponentDowngradeTrigger, M5BundleRequiredLabel, M5WorkflowBundleComponentFamily,
};
// Reused consumer-surface family already minted by the class-disclosure primitive: the
// same start-center / detail / migration / docs-help / diagnostics / support surfaces
// ingest this accessibility fallback, so no parallel surface vocabulary is coined.
use crate::implement_the_m5_bundle_class_disclosure_cards_and_claim_narrowing_rows::M5BundleDisclosureSurfaceFamily;

/// Schema version stamped on the M05-850 bundle accessibility fallback packet.
pub const BUNDLE_A11Y_FALLBACK_SCHEMA_VERSION: u32 = 1;

/// Stable record-kind tag carried by [`BundleAccessibilityPacket`].
pub const BUNDLE_A11Y_FALLBACK_RECORD_KIND: &str =
    "m5_workflow_bundle_component_accessibility_fallback_packet";

/// Stable record-kind tag carried by each [`BundleAccessibilityRow`].
pub const BUNDLE_A11Y_FALLBACK_ROW_RECORD_KIND: &str =
    "m5_workflow_bundle_component_accessibility_fallback_row";

/// Repo-relative path of the boundary schema.
pub const BUNDLE_A11Y_FALLBACK_SCHEMA_REF: &str =
    "schemas/ui/m5-workflow-bundle-component-accessibility-fallback.schema.json";

/// Repo-relative path of the contract doc.
pub const BUNDLE_A11Y_FALLBACK_DOC_REF: &str =
    "docs/bundles/m5_workflow_bundle_component_accessibility_fallback.md";

/// Repo-relative path of the frozen workflow-bundle component matrix this lane
/// certifies.
pub const BUNDLE_A11Y_FALLBACK_COMPONENT_MATRIX_REF: &str =
    "schemas/ui/m5-workflow-bundle-component-matrix.schema.json";

/// Repo-relative path of the protected fixture directory.
pub const BUNDLE_A11Y_FALLBACK_FIXTURE_DIR: &str =
    "fixtures/ui/m5-workflow-bundle-component-accessibility-fallback";

/// Repo-relative path of the checked support-export artifact (the `include_str!`
/// canonical).
pub const BUNDLE_A11Y_FALLBACK_ARTIFACT_REF: &str =
    "artifacts/release/m5-workflow-bundle-component-accessibility-fallback-proof/support_export.json";

/// Repo-relative path of the checked machine-readable matrix CSV.
pub const BUNDLE_A11Y_FALLBACK_CSV_REF: &str =
    "artifacts/release/m5-workflow-bundle-component-accessibility-fallback-proof/matrix.csv";

/// Repo-relative path of the checked Markdown report.
pub const BUNDLE_A11Y_FALLBACK_REPORT_REF: &str =
    "artifacts/release/m5-workflow-bundle-component-accessibility-fallback-proof/report.md";

/// The reusable component families that render a non-linear hierarchy (the bundle
/// detail page's component / dependency inventory) and therefore MUST bind their
/// tree to an equivalent flat list / textual path so the hierarchy is navigable
/// non-visually.
const fn family_is_hierarchy_heavy(family: M5WorkflowBundleComponentFamily) -> bool {
    matches!(family, M5WorkflowBundleComponentFamily::BundleDetailPage)
}

/// The workflow-bundle dimension whose weakening a family primarily discloses. Every
/// row must model at least this dimension so its key weakening axis is covered.
const fn family_primary_dimension(
    family: M5WorkflowBundleComponentFamily,
) -> M5BundleClaimDimension {
    match family {
        M5WorkflowBundleComponentFamily::StartCenterBundleCard
        | M5WorkflowBundleComponentFamily::CertifiedArchetypeBadgeGroup
        | M5WorkflowBundleComponentFamily::BundleClaimNarrowingRow => {
            M5BundleClaimDimension::CertificationEvidence
        }
        M5WorkflowBundleComponentFamily::BundleDetailPage => {
            M5BundleClaimDimension::DependencyPosture
        }
        M5WorkflowBundleComponentFamily::BundleInstallUpdateReviewSheet
        | M5WorkflowBundleComponentFamily::BundleRollbackRemoveCard => {
            M5BundleClaimDimension::ArtifactAvailability
        }
        M5WorkflowBundleComponentFamily::BundleDriftBanner
        | M5WorkflowBundleComponentFamily::BundleLocalOverrideRow => {
            M5BundleClaimDimension::BundleFreshness
        }
        M5WorkflowBundleComponentFamily::BundleClassDisclosureCard => {
            M5BundleClaimDimension::SourceProvenance
        }
    }
}

/// A rendered fallback modality for a workflow-bundle component.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5BundleFallbackModality {
    /// A rich, structured (tree / grouped inventory) projection.
    Structured,
    /// A flat list projection.
    List,
    /// A textual / source-first projection.
    Textual,
    /// A CLI / headless line projection.
    Cli,
}

impl M5BundleFallbackModality {
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

/// A rendering-surface capability tier. Distinct from the semantic consumer surface:
/// the same component may render at desktop-full capability or narrow to a companion,
/// read-only browser, headless CLI, handoff packet, or support export.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5BundleRenderingSurface {
    /// The full-capability desktop workspace surface.
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

impl M5BundleRenderingSurface {
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
pub enum BundleNonVisualReachState {
    /// Fully traversable and labeled with no loss.
    ReachableAndLabeled,
    /// Reachable and labeled, but with a disclosed reduction (yellow).
    DisclosedReducedButReachable,
    /// A view-only surface that traps keyboard / assistive-tech / headless users
    /// (red).
    ViewOnlyTrap,
}

impl BundleNonVisualReachState {
    /// Returns true when the state never strands keyboard / assistive-tech /
    /// headless users.
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

/// Whether an export-safe summary preserves the component meaning without a
/// screenshot.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BundleExportSummaryState {
    /// The component meaning reconstructs from the summary without a screenshot.
    ReconstructableWithoutScreenshot,
    /// Partial capture, but disclosed (yellow).
    DisclosedPartialCapture,
    /// The export relies on a screenshot to carry meaning (red).
    AbsentNeedsScreenshot,
}

impl BundleExportSummaryState {
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
pub enum BundleNarrowingDisclosureState {
    /// Full label and summary parity with the desktop surface.
    ParityPreserved,
    /// Reduced interactivity, disclosed with preserved labels (yellow).
    DisclosedNarrowed,
    /// Interactivity, state, or actions dropped without disclosure (red).
    SilentlyDropped,
}

impl BundleNarrowingDisclosureState {
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

/// The bundle-support claim ceiling a component asserts: how strong a support
/// posture it lets a bundle present. Auto-narrowing lowers this ceiling when a
/// bundle dimension weakens so a stale, partial, offline, or policy-blocked bundle
/// can never present as fully certified or fully self-sufficient.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5BundleSupportClaim {
    /// Fully certified and current: the strongest self-sufficient claim.
    Certified,
    /// Supported but not top-certified: a maintained, standalone claim.
    Supported,
    /// Limited support: usable, but with a disclosed reduction in scope.
    Limited,
    /// Retest-pending: certification evidence is stale and must be re-run.
    RetestPending,
    /// Imported / bridged: derived from another tool or user handoff, not native.
    Imported,
    /// Mirror-only: served from a stale mirror, not a live first-party registry.
    MirrorOnly,
    /// Offline-cache-only: only a cached-offline snapshot is available.
    OfflineCacheOnly,
    /// Policy-blocked: a required entitlement / policy dependency is unmet.
    PolicyBlocked,
}

impl M5BundleSupportClaim {
    /// Every claim tier, strongest first.
    pub const ALL: [Self; 8] = [
        Self::Certified,
        Self::Supported,
        Self::Limited,
        Self::RetestPending,
        Self::Imported,
        Self::MirrorOnly,
        Self::OfflineCacheOnly,
        Self::PolicyBlocked,
    ];

    /// Capability rank; a higher rank asserts a stronger support posture. Narrowing
    /// lowers rank.
    pub const fn capability_rank(self) -> u8 {
        match self {
            Self::Certified => 7,
            Self::Supported => 6,
            Self::Limited => 5,
            Self::RetestPending => 4,
            Self::Imported => 3,
            Self::MirrorOnly => 2,
            Self::OfflineCacheOnly => 1,
            Self::PolicyBlocked => 0,
        }
    }

    /// Returns true when this claim asserts full certification.
    pub const fn asserts_full_certification(self) -> bool {
        matches!(self, Self::Certified)
    }

    /// Returns true when this claim asserts a fully self-sufficient (standalone,
    /// live-first-party) posture.
    pub const fn asserts_full_self_sufficiency(self) -> bool {
        matches!(self, Self::Certified | Self::Supported)
    }

    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Certified => "certified",
            Self::Supported => "supported",
            Self::Limited => "limited",
            Self::RetestPending => "retest_pending",
            Self::Imported => "imported",
            Self::MirrorOnly => "mirror_only",
            Self::OfflineCacheOnly => "offline_cache_only",
            Self::PolicyBlocked => "policy_blocked",
        }
    }
}

/// The workflow-bundle dimension whose state governs how far a component may claim
/// to be certified, self-sufficient, or current.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5BundleClaimDimension {
    /// Bundle freshness: is the local bundle state current, or has it drifted /
    /// aged?
    BundleFreshness,
    /// Certification evidence: is the certification current, or stale and due for
    /// retest?
    CertificationEvidence,
    /// Source provenance: is the bundle a native first-party read, or imported /
    /// bridged?
    SourceProvenance,
    /// Artifact availability: is the bundle served live, or only from a stale mirror
    /// / offline cache?
    ArtifactAvailability,
    /// Dependency posture: are the bundle's entitlement / policy dependencies met?
    DependencyPosture,
}

impl M5BundleClaimDimension {
    /// Every dimension, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::BundleFreshness,
        Self::CertificationEvidence,
        Self::SourceProvenance,
        Self::ArtifactAvailability,
        Self::DependencyPosture,
    ];

    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::BundleFreshness => "bundle_freshness",
            Self::CertificationEvidence => "certification_evidence",
            Self::SourceProvenance => "source_provenance",
            Self::ArtifactAvailability => "artifact_availability",
            Self::DependencyPosture => "dependency_posture",
        }
    }
}

/// The observed condition of one workflow-bundle dimension. Anything weaker than
/// [`Self::Intact`] imposes a narrowing ceiling on the component's support claim.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5BundleClaimConditionState {
    /// Fully current / certified / native / met — imposes no ceiling.
    Intact,
    /// Partially resolved — scope is reduced, support drops to limited.
    Partial,
    /// Stale — certification is aged and must be re-run; support drops to
    /// retest-pending.
    Stale,
    /// Imported / bridged — not a native first-party read; support drops to
    /// imported.
    Imported,
    /// Mirror-stale — served from a stale mirror; support drops to mirror-only.
    MirrorStale,
    /// Offline-only — only a cached-offline snapshot is reachable; support drops to
    /// offline-cache-only.
    OfflineOnly,
    /// Policy-blocked — a required entitlement / policy dependency is unmet; support
    /// drops to policy-blocked.
    PolicyBlocked,
}

impl M5BundleClaimConditionState {
    /// Every condition state, in declaration order.
    pub const ALL: [Self; 7] = [
        Self::Intact,
        Self::Partial,
        Self::Stale,
        Self::Imported,
        Self::MirrorStale,
        Self::OfflineOnly,
        Self::PolicyBlocked,
    ];

    /// Returns true when the dimension is weaker than intact and therefore imposes a
    /// narrowing ceiling.
    pub const fn is_weak(self) -> bool {
        !matches!(self, Self::Intact)
    }

    /// The strongest bundle-support claim this condition state permits.
    pub const fn permitted_ceiling(self) -> M5BundleSupportClaim {
        match self {
            Self::Intact => M5BundleSupportClaim::Certified,
            Self::Partial => M5BundleSupportClaim::Limited,
            Self::Stale => M5BundleSupportClaim::RetestPending,
            Self::Imported => M5BundleSupportClaim::Imported,
            Self::MirrorStale => M5BundleSupportClaim::MirrorOnly,
            Self::OfflineOnly => M5BundleSupportClaim::OfflineCacheOnly,
            Self::PolicyBlocked => M5BundleSupportClaim::PolicyBlocked,
        }
    }

    /// The frozen downgrade trigger this condition names when it narrows a claim.
    /// [`Self::Intact`] never binds a narrowing, so its trigger is never rendered.
    pub const fn default_trigger(self) -> M5BundleComponentDowngradeTrigger {
        match self {
            Self::Intact | Self::Stale => M5BundleComponentDowngradeTrigger::StaleCertification,
            Self::Partial => M5BundleComponentDowngradeTrigger::LocalOverrideDrift,
            Self::Imported => M5BundleComponentDowngradeTrigger::ImportedNotNative,
            Self::MirrorStale => M5BundleComponentDowngradeTrigger::MirrorStale,
            Self::OfflineOnly => M5BundleComponentDowngradeTrigger::OfflineCacheOnly,
            Self::PolicyBlocked => M5BundleComponentDowngradeTrigger::EntitlementDependencyUnmet,
        }
    }

    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Intact => "intact",
            Self::Partial => "partial",
            Self::Stale => "stale",
            Self::Imported => "imported",
            Self::MirrorStale => "mirror_stale",
            Self::OfflineOnly => "offline_only",
            Self::PolicyBlocked => "policy_blocked",
        }
    }
}

/// One workflow-bundle dimension's observed condition on a component.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BundleClaimConditionEntry {
    /// Which dimension this entry describes.
    pub dimension: M5BundleClaimDimension,
    /// The observed condition state of the dimension.
    pub state: M5BundleClaimConditionState,
}

/// An honest bundle-support-claim auto-narrow block. When a bundle dimension
/// weakens, the component's support claim lowers to the permitted ceiling, names the
/// binding dimension and frozen trigger, and preserves the canonical bundle identity
/// rather than silently dropping it.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BundleClaimAutoNarrow {
    /// The support claim the component is narrowed to.
    pub narrowed_to: M5BundleSupportClaim,
    /// The dimension whose weakness bound the narrowing (the one imposing the
    /// strongest ceiling constraint).
    pub binding_dimension: M5BundleClaimDimension,
    /// The frozen downgrade trigger (reused vocabulary) the narrowing names.
    pub trigger: M5BundleComponentDowngradeTrigger,
    /// A precise, non-generic label safe to render.
    pub narrowed_label: String,
    /// The canonical bundle identity, signer / source class, evidence age, and drift
    /// state are preserved rather than dropped; must hold.
    pub preserves_canonical_identity: bool,
}

impl BundleClaimAutoNarrow {
    /// Whether the auto-narrow block is honest: it preserves canonical identity and
    /// carries a precise, non-generic label.
    pub fn is_honest(&self) -> bool {
        self.preserves_canonical_identity && !label_is_generic(&self.narrowed_label)
    }
}

/// Copy / export parity for a component's accessible fallback: the same truth must
/// be copyable as text / JSON / Markdown, and a screenshot is never the only export.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BundleCopyExportParity {
    /// The copy / export formats offered (must include text, json, markdown).
    #[serde(default)]
    pub formats: Vec<String>,
    /// The named export fields the summary carries.
    #[serde(default)]
    pub export_fields: Vec<String>,
    /// A screenshot is never the only export; must always hold.
    pub screenshot_only_prohibited: bool,
}

impl BundleCopyExportParity {
    /// Whether the copy / export parity is complete: text / JSON / Markdown are all
    /// offered, at least one export field is named, and screenshots are prohibited as
    /// the sole export.
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
pub struct BundleRenderingNarrowingDisclosure {
    /// The rendering surface being narrowed.
    pub rendering_surface: M5BundleRenderingSurface,
    /// How the surface discloses its reduced interactivity.
    pub state: BundleNarrowingDisclosureState,
    /// The labels preserved across the narrowing.
    #[serde(default)]
    pub preserved_labels: Vec<String>,
    /// The interactions reduced on the narrowed surface.
    #[serde(default)]
    pub reduced_interactions: Vec<String>,
}

/// Derived qualification status for a bundle accessibility row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BundleAccessibilityStatus {
    /// Full keyboard / screen-reader / CLI / export parity with no narrowing (green).
    Parity,
    /// Reduced but fully disclosed, reachable, and honestly auto-narrowed (yellow).
    NarrowedDisclosed,
    /// Strands assistive tech, needs a screenshot, over-claims support, or drops
    /// state silently (red).
    Stranded,
}

impl BundleAccessibilityStatus {
    /// Stable token recorded in the summary / CSV.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Parity => "parity",
            Self::NarrowedDisclosed => "narrowed_disclosed",
            Self::Stranded => "stranded",
        }
    }
}

/// Accessibility / auto-narrowing parity row for one workflow-bundle component
/// family.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BundleAccessibilityRow {
    /// Record kind; must equal [`BUNDLE_A11Y_FALLBACK_ROW_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`BUNDLE_A11Y_FALLBACK_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable row id.
    pub row_id: String,
    /// The frozen component family this row certifies.
    pub component_family: M5WorkflowBundleComponentFamily,
    /// Ref to the frozen matrix family schema this row certifies.
    pub source_family_schema_ref: String,
    /// Opaque ref to the bundle / archetype context this component acts on; stays
    /// visible on every surface, so this is never empty.
    pub bundle_context_ref: String,
    /// Rendered modalities offered; a hierarchy-heavy family must also offer a
    /// non-visual (list / textual / cli) path.
    #[serde(default)]
    pub fallback_modalities: Vec<M5BundleFallbackModality>,
    /// The non-visual / CLI path reaches the same canonical bundle identity, source
    /// class, evidence age, and drift state as the rich surface; must hold.
    pub reaches_canonical_truth: bool,
    /// Keyboard reach into the non-visual path.
    pub keyboard_reach: BundleNonVisualReachState,
    /// Screen-reader reach into the non-visual path.
    pub screen_reader_reach: BundleNonVisualReachState,
    /// CLI / headless reach into the non-visual path.
    pub cli_reach: BundleNonVisualReachState,
    /// Whether the export-safe summary preserves component meaning.
    pub export_summary: BundleExportSummaryState,
    /// Ref to the export-safe summary object for this component.
    pub export_summary_ref: String,
    /// The copy / export parity of the accessible fallback.
    pub copy_export: BundleCopyExportParity,
    /// The full support claim this family asserts when every dimension is intact.
    pub full_support_claim: M5BundleSupportClaim,
    /// The observed condition of each modeled bundle dimension.
    #[serde(default)]
    pub claim_conditions: Vec<BundleClaimConditionEntry>,
    /// The honest auto-narrow block, present only when some dimension weakens below
    /// the family's full claim.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub claim_narrow: Option<BundleClaimAutoNarrow>,
    /// Rendering surfaces this component is certified on.
    #[serde(default)]
    pub rendering_surfaces: Vec<M5BundleRenderingSurface>,
    /// Per-surface narrowing disclosures.
    #[serde(default)]
    pub narrowing_disclosures: Vec<BundleRenderingNarrowingDisclosure>,
    /// The required labels the accessible fallback preserves (reused vocabulary).
    #[serde(default)]
    pub required_labels: Vec<M5BundleRequiredLabel>,
    /// Semantic consumer surfaces this component is embedded in (reused vocabulary).
    #[serde(default)]
    pub consumer_surfaces: Vec<M5BundleDisclosureSurfaceFamily>,
    /// Source contract refs backing this row.
    #[serde(default)]
    pub source_refs: Vec<String>,
    /// ISO 8601 UTC timestamp the accessibility posture was observed.
    pub observed_at: String,
    /// Evidence packet refs backing this row.
    #[serde(default)]
    pub evidence_refs: Vec<String>,
}

impl BundleAccessibilityRow {
    /// Returns true when this family renders a non-linear hierarchy and must bind to
    /// a flat non-visual path.
    pub const fn is_hierarchy_heavy(&self) -> bool {
        family_is_hierarchy_heavy(self.component_family)
    }

    /// Returns true when at least one non-visual (list / textual / cli) fallback
    /// modality is offered.
    pub fn has_non_visual_fallback(&self) -> bool {
        self.fallback_modalities.iter().any(|m| m.is_non_visual())
    }

    /// The condition state observed for one dimension, or `Intact` when the row does
    /// not model that dimension.
    pub fn condition_for(&self, dimension: M5BundleClaimDimension) -> M5BundleClaimConditionState {
        self.claim_conditions
            .iter()
            .find(|c| c.dimension == dimension)
            .map(|c| c.state)
            .unwrap_or(M5BundleClaimConditionState::Intact)
    }

    /// Whether any modeled dimension is weaker than intact.
    pub fn has_weak_dimension(&self) -> bool {
        self.claim_conditions.iter().any(|c| c.state.is_weak())
    }

    /// The strongest support claim permitted after applying every modeled
    /// dimension's ceiling, capped at the family's full claim.
    pub fn permitted_claim(&self) -> M5BundleSupportClaim {
        let mut permitted = self.full_support_claim;
        for condition in &self.claim_conditions {
            let ceiling = condition.state.permitted_ceiling();
            if ceiling.capability_rank() < permitted.capability_rank() {
                permitted = ceiling;
            }
        }
        permitted
    }

    /// The dimension imposing the strongest (lowest-rank) ceiling, if any weak
    /// dimension narrows below the family's full claim.
    pub fn binding_dimension(&self) -> Option<M5BundleClaimDimension> {
        let mut binding: Option<(M5BundleClaimDimension, u8)> = None;
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
    pub fn effective_claim(&self) -> M5BundleSupportClaim {
        match &self.claim_narrow {
            Some(narrow) => narrow.narrowed_to,
            None => self.full_support_claim,
        }
    }

    /// AC1: a stale or partial bundle can no longer present as fully certified or
    /// fully self-sufficient. The effective claim never exceeds the permitted
    /// ceiling; when a dimension narrows below the full claim, an honest narrow block
    /// is present, narrows to exactly the permitted ceiling, binds to the
    /// ceiling-imposing dimension with its frozen trigger, and preserves canonical
    /// identity. When nothing narrows, no spurious narrow block is present.
    pub fn claim_is_honest(&self) -> bool {
        let permitted = self.permitted_claim();
        if self.effective_claim().capability_rank() > permitted.capability_rank() {
            return false;
        }
        match (&self.claim_narrow, self.binding_dimension()) {
            (Some(narrow), Some(binding)) => {
                let binding_state = self.condition_for(binding);
                narrow.is_honest()
                    && narrow.narrowed_to == permitted
                    && narrow.binding_dimension == binding
                    && narrow.trigger == binding_state.default_trigger()
                    && binding_state.is_weak()
            }
            // A narrow block with no ceiling-imposing dimension is spurious.
            (Some(_), None) => false,
            // A ceiling-imposing dimension with no narrow block over-claims.
            (None, Some(_)) => false,
            (None, None) => true,
        }
    }

    /// AC2: accessibility and export surfaces reach the same canonical truth — no
    /// keyboard / screen-reader / CLI trap, a hierarchy-heavy family offers a
    /// non-visual fallback, and the export reconstructs meaning without a screenshot.
    pub fn reaches_canonical_truth_via_at(&self) -> bool {
        self.reaches_canonical_truth
            && !self.bundle_context_ref.trim().is_empty()
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

    /// AC3: every narrower rendering surface discloses its reduced interactivity and
    /// keeps its labels, so claim publication and field triage stay aligned on the
    /// same narrowed state.
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

    /// Whether every mandatory required label is preserved on the accessible
    /// fallback.
    pub fn preserves_mandatory_labels(&self) -> bool {
        M5BundleRequiredLabel::MANDATORY
            .iter()
            .all(|label| self.required_labels.contains(label))
    }

    /// Derived qualification status.
    pub fn status(&self) -> BundleAccessibilityStatus {
        if !self.claim_is_honest()
            || !self.reaches_canonical_truth_via_at()
            || !self.export_preserves_meaning()
            || !self.narrowing_disclosed()
            || !self.models_primary_dimension()
            || !self.preserves_mandatory_labels()
        {
            return BundleAccessibilityStatus::Stranded;
        }
        if self.is_reduced() {
            BundleAccessibilityStatus::NarrowedDisclosed
        } else {
            BundleAccessibilityStatus::Parity
        }
    }

    /// Whether the row's identity and evidence fields are complete.
    pub fn is_complete(&self) -> bool {
        self.record_kind == BUNDLE_A11Y_FALLBACK_ROW_RECORD_KIND
            && self.schema_version == BUNDLE_A11Y_FALLBACK_SCHEMA_VERSION
            && !self.row_id.trim().is_empty()
            && !self.source_family_schema_ref.trim().is_empty()
            && !self.bundle_context_ref.trim().is_empty()
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

/// Rolled-up summary of an M05-850 bundle accessibility fallback packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BundleAccessibilitySummary {
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

/// Constructor input for [`BundleAccessibilityPacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BundleAccessibilityPacketInput {
    pub packet_id: String,
    pub as_of: String,
    pub matrix_ref: String,
    pub rows: Vec<BundleAccessibilityRow>,
}

/// Checked-in M05-850 bundle accessibility fallback packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BundleAccessibilityPacket {
    pub schema_version: u32,
    pub record_kind: String,
    pub packet_id: String,
    pub as_of: String,
    pub matrix_ref: String,
    #[serde(default)]
    pub rows: Vec<BundleAccessibilityRow>,
    pub summary: BundleAccessibilitySummary,
}

impl BundleAccessibilityPacket {
    /// Builds a packet, stamping the record kind, schema version, and computed
    /// summary.
    pub fn new(input: BundleAccessibilityPacketInput) -> Self {
        let mut packet = Self {
            schema_version: BUNDLE_A11Y_FALLBACK_SCHEMA_VERSION,
            record_kind: BUNDLE_A11Y_FALLBACK_RECORD_KIND.to_owned(),
            packet_id: input.packet_id,
            as_of: input.as_of,
            matrix_ref: input.matrix_ref,
            rows: input.rows,
            summary: BundleAccessibilitySummary {
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
    pub fn represented_families(&self) -> BTreeSet<M5WorkflowBundleComponentFamily> {
        self.rows.iter().map(|r| r.component_family).collect()
    }

    /// Dimensions exercised by some row's claim conditions.
    pub fn exercised_dimensions(&self) -> BTreeSet<M5BundleClaimDimension> {
        self.rows
            .iter()
            .flat_map(|r| r.claim_conditions.iter().map(|c| c.dimension))
            .collect()
    }

    /// Support claim tiers that appear as an effective claim across the rows.
    pub fn represented_effective_claims(&self) -> BTreeSet<M5BundleSupportClaim> {
        self.rows.iter().map(|r| r.effective_claim()).collect()
    }

    /// Consumer surfaces ingesting some row in this packet.
    pub fn represented_consumer_surfaces(&self) -> BTreeSet<M5BundleDisclosureSurfaceFamily> {
        self.rows
            .iter()
            .flat_map(|r| r.consumer_surfaces.iter().copied())
            .collect()
    }

    /// Computes summary fields from the packet contents.
    pub fn computed_summary(&self) -> BundleAccessibilitySummary {
        let mut rendering = BTreeSet::new();
        let mut consumers = BTreeSet::new();
        for row in &self.rows {
            rendering.extend(row.rendering_surfaces.iter().copied());
            consumers.extend(row.consumer_surfaces.iter().copied());
        }

        let hierarchy_heavy: Vec<&BundleAccessibilityRow> = self
            .rows
            .iter()
            .filter(|row| row.is_hierarchy_heavy())
            .collect();

        let mut green = 0;
        let mut yellow = 0;
        let mut red = 0;
        for row in &self.rows {
            match row.status() {
                BundleAccessibilityStatus::Parity => green += 1,
                BundleAccessibilityStatus::NarrowedDisclosed => yellow += 1,
                BundleAccessibilityStatus::Stranded => red += 1,
            }
        }

        BundleAccessibilitySummary {
            family_count: self.rows.len(),
            hierarchy_heavy_family_count: hierarchy_heavy.len(),
            all_hierarchy_heavy_have_non_visual_fallback: hierarchy_heavy
                .iter()
                .all(|row| row.has_non_visual_fallback()),
            all_reach_canonical_truth_via_at: self
                .rows
                .iter()
                .all(BundleAccessibilityRow::reaches_canonical_truth_via_at),
            all_claims_honest: self
                .rows
                .iter()
                .all(BundleAccessibilityRow::claim_is_honest),
            all_export_summaries_preserve_meaning: self
                .rows
                .iter()
                .all(BundleAccessibilityRow::export_preserves_meaning),
            all_narrowing_disclosed: self
                .rows
                .iter()
                .all(BundleAccessibilityRow::narrowing_disclosed),
            green_count: green,
            yellow_count: yellow,
            red_count: red,
            rendering_surface_count: rendering.len(),
            consumer_surface_count: consumers.len(),
        }
    }

    /// Validates the packet and returns every contract violation.
    pub fn validate(&self) -> Vec<BundleAccessibilityViolation> {
        let mut violations = Vec::new();

        if self.schema_version != BUNDLE_A11Y_FALLBACK_SCHEMA_VERSION {
            violations.push(BundleAccessibilityViolation::SchemaVersion {
                expected: BUNDLE_A11Y_FALLBACK_SCHEMA_VERSION,
                actual: self.schema_version,
            });
        }
        if self.record_kind != BUNDLE_A11Y_FALLBACK_RECORD_KIND {
            violations.push(BundleAccessibilityViolation::RecordKind {
                expected: BUNDLE_A11Y_FALLBACK_RECORD_KIND.to_owned(),
                actual: self.record_kind.clone(),
            });
        }
        if self.packet_id.trim().is_empty()
            || self.as_of.trim().is_empty()
            || self.matrix_ref.trim().is_empty()
        {
            violations.push(BundleAccessibilityViolation::MissingIdentity);
        }

        let mut row_ids = BTreeSet::new();
        let mut seen_families = BTreeSet::new();
        for row in &self.rows {
            if !row_ids.insert(row.row_id.clone()) {
                violations.push(BundleAccessibilityViolation::DuplicateId {
                    id: row.row_id.clone(),
                });
            }
            seen_families.insert(row.component_family);

            if !row.is_complete() {
                violations.push(BundleAccessibilityViolation::IncompleteRow {
                    id: row.row_id.clone(),
                });
            }

            // Each row must model its family's primary weakening dimension.
            if !row.models_primary_dimension() {
                violations.push(BundleAccessibilityViolation::MissingPrimaryDimension {
                    id: row.row_id.clone(),
                    dimension: family_primary_dimension(row.component_family),
                });
            }

            // Each row must preserve every mandatory bundle label.
            if !row.preserves_mandatory_labels() {
                violations.push(BundleAccessibilityViolation::MissingMandatoryLabel {
                    id: row.row_id.clone(),
                });
            }

            // A hierarchy-heavy family must render a structured tree *and* a
            // non-visual path.
            if row.is_hierarchy_heavy()
                && !row
                    .fallback_modalities
                    .contains(&M5BundleFallbackModality::Structured)
            {
                violations.push(
                    BundleAccessibilityViolation::HierarchyHeavyMissingStructured {
                        id: row.row_id.clone(),
                    },
                );
            }

            // AC1: claim never over-asserts support for a weakened bundle.
            if !row.claim_is_honest() {
                violations.push(BundleAccessibilityViolation::ClaimOverAsserted {
                    id: row.row_id.clone(),
                });
            }

            // AC2: assistive-tech / CLI reach the same canonical truth.
            if !row.reaches_canonical_truth_via_at() {
                violations.push(BundleAccessibilityViolation::AssistiveTechStranded {
                    id: row.row_id.clone(),
                });
            }

            // Export preserves meaning without a screenshot.
            if !row.export_preserves_meaning() {
                violations.push(BundleAccessibilityViolation::ExportRequiresScreenshot {
                    id: row.row_id.clone(),
                });
            }

            // AC3: narrowing disclosed on every narrowed rendering surface.
            if !row.narrowing_disclosed() {
                violations.push(
                    BundleAccessibilityViolation::NarrowingDropsContextSilently {
                        id: row.row_id.clone(),
                    },
                );
            }

            // Consumer parity: at least two consumer surfaces ingest the row.
            if row.consumer_surfaces.len() < 2 {
                violations.push(BundleAccessibilityViolation::MissingConsumerParity {
                    id: row.row_id.clone(),
                });
            }

            // No red rows may ship.
            if row.status() == BundleAccessibilityStatus::Stranded {
                violations.push(BundleAccessibilityViolation::StrandedRow {
                    id: row.row_id.clone(),
                });
            }
        }

        // Coverage: every frozen family is certified at least once.
        for family in M5WorkflowBundleComponentFamily::ALL {
            if !seen_families.contains(&family) {
                violations.push(BundleAccessibilityViolation::MissingFamilyCoverage { family });
            }
        }

        // Coverage: every weakening dimension is exercised somewhere.
        let exercised = self.exercised_dimensions();
        for dimension in M5BundleClaimDimension::ALL {
            if !exercised.contains(&dimension) {
                violations
                    .push(BundleAccessibilityViolation::MissingDimensionCoverage { dimension });
            }
        }

        // Coverage: every support claim tier appears as an effective claim, so the
        // full narrowing spectrum (certified → … → policy-blocked) is proven
        // end-to-end.
        let effective = self.represented_effective_claims();
        for claim in M5BundleSupportClaim::ALL {
            if !effective.contains(&claim) {
                violations.push(BundleAccessibilityViolation::MissingClaimTierCoverage { claim });
            }
        }

        // AC3 cross-surface: the same narrowed state must reach docs/help,
        // migration, diagnostics, and support/admin exports — so every consumer
        // surface is exercised at least once across the packet.
        let consumers = self.represented_consumer_surfaces();
        for surface in M5BundleDisclosureSurfaceFamily::ALL {
            if !consumers.contains(&surface) {
                violations
                    .push(BundleAccessibilityViolation::MissingConsumerSurfaceCoverage { surface });
            }
        }

        if self.summary != self.computed_summary() {
            violations.push(BundleAccessibilityViolation::SummaryMismatch);
        }

        if json_contains_forbidden_material(
            &serde_json::to_value(self).expect("bundle accessibility fallback packet serializes"),
        ) {
            violations.push(BundleAccessibilityViolation::RawBoundaryMaterialInExport);
        }

        violations
    }

    /// Deterministic export-safe JSON.
    ///
    /// # Panics
    ///
    /// Panics only if serializing this metadata-only packet fails.
    pub fn export_safe_json(&self) -> String {
        serde_json::to_string_pretty(self).expect("bundle accessibility fallback packet serializes")
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
        out.push_str("# M5 Workflow-Bundle Component Accessibility & Auto-Narrowing\n\n");
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- As of: `{}`\n", self.as_of));
        out.push_str(&format!(
            "- Families: {} certified across {} / {} frozen families\n",
            self.summary.family_count,
            self.represented_families().len(),
            M5WorkflowBundleComponentFamily::ALL.len(),
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

/// Reads and validates the checked-in bundle accessibility fallback export.
pub fn current_m5_bundle_a11y_fallback_export(
) -> Result<BundleAccessibilityPacket, BundleAccessibilityArtifactError> {
    let packet: BundleAccessibilityPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/release/m5-workflow-bundle-component-accessibility-fallback-proof/support_export.json"
    )))
    .map_err(BundleAccessibilityArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(BundleAccessibilityArtifactError::Validation(violations))
    }
}

/// Errors emitted when reading the checked-in bundle accessibility fallback export.
#[derive(Debug)]
pub enum BundleAccessibilityArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<BundleAccessibilityViolation>),
}

impl fmt::Display for BundleAccessibilityArtifactError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(
                    f,
                    "bundle accessibility fallback export parse failed: {error}"
                )
            }
            Self::Validation(violations) => {
                write!(
                    f,
                    "bundle accessibility fallback export failed validation: {} violation(s)",
                    violations.len()
                )
            }
        }
    }
}

impl Error for BundleAccessibilityArtifactError {}

/// Validation failure for M05-850 bundle accessibility fallback packets.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BundleAccessibilityViolation {
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
        dimension: M5BundleClaimDimension,
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
        family: M5WorkflowBundleComponentFamily,
    },
    MissingDimensionCoverage {
        dimension: M5BundleClaimDimension,
    },
    MissingClaimTierCoverage {
        claim: M5BundleSupportClaim,
    },
    MissingConsumerSurfaceCoverage {
        surface: M5BundleDisclosureSurfaceFamily,
    },
    SummaryMismatch,
    RawBoundaryMaterialInExport,
}

impl fmt::Display for BundleAccessibilityViolation {
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
                write!(f, "row {id} drops a mandatory bundle label")
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
                    "row {id} over-asserts bundle support for a weakened bundle, or narrows spuriously"
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
            Self::RawBoundaryMaterialInExport => {
                write!(f, "export contains raw boundary material")
            }
        }
    }
}

impl Error for BundleAccessibilityViolation {}

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

/// Builds the canonical, checked-in bundle accessibility fallback packet. This is
/// the one source of truth shared by the tests, the example dump, and the on-disk
/// support export so all three stay byte-aligned.
pub fn seeded_m5_bundle_a11y_fallback_packet() -> BundleAccessibilityPacket {
    BundleAccessibilityPacket::new(BundleAccessibilityPacketInput {
        packet_id: "m5-workflow-bundle-component-accessibility-fallback:stable:0001".to_owned(),
        as_of: "2026-07-06T00:00:00Z".to_owned(),
        matrix_ref: BUNDLE_A11Y_FALLBACK_COMPONENT_MATRIX_REF.to_owned(),
        rows: seeded_rows(),
    })
}

fn ev(id: &str) -> Vec<String> {
    vec![format!("evidence:bundle-a11y:{id}")]
}

fn all_required_labels() -> Vec<M5BundleRequiredLabel> {
    M5BundleRequiredLabel::ALL.to_vec()
}

fn copy_export(fields: &[&str]) -> BundleCopyExportParity {
    BundleCopyExportParity {
        formats: vec!["text".to_owned(), "json".to_owned(), "markdown".to_owned()],
        export_fields: fields.iter().map(|f| (*f).to_owned()).collect(),
        screenshot_only_prohibited: true,
    }
}

fn condition(
    dimension: M5BundleClaimDimension,
    state: M5BundleClaimConditionState,
) -> BundleClaimConditionEntry {
    BundleClaimConditionEntry { dimension, state }
}

/// The two consumer surfaces every row ships to at minimum — diagnostics and
/// support/export replay — so the narrowed state always reaches field triage.
fn base_consumers(
    extra: &[M5BundleDisclosureSurfaceFamily],
) -> Vec<M5BundleDisclosureSurfaceFamily> {
    let mut out = vec![
        M5BundleDisclosureSurfaceFamily::DiagnosticsClassReport,
        M5BundleDisclosureSurfaceFamily::SupportExportReplay,
    ];
    out.extend_from_slice(extra);
    out
}

/// Disclosures for the CLI-headless and support-export surfaces. A green (full
/// parity) row keeps full label and summary parity on the narrower surfaces; a
/// narrowed row discloses the reduced interactions it drops there.
fn surface_disclosures(
    labels: &[&str],
    state: BundleNarrowingDisclosureState,
) -> Vec<BundleRenderingNarrowingDisclosure> {
    let preserved: Vec<String> = labels.iter().map(|l| (*l).to_owned()).collect();
    vec![
        BundleRenderingNarrowingDisclosure {
            rendering_surface: M5BundleRenderingSurface::CliHeadless,
            state,
            preserved_labels: preserved.clone(),
            reduced_interactions: vec!["pointer_interaction".to_owned()],
        },
        BundleRenderingNarrowingDisclosure {
            rendering_surface: M5BundleRenderingSurface::SupportExport,
            state,
            preserved_labels: preserved,
            reduced_interactions: vec!["live_action".to_owned()],
        },
    ]
}

/// Disclosures for a full-parity (green) row: the narrower surfaces preserve full
/// label and summary parity.
fn parity_surfaces(labels: &[&str]) -> Vec<BundleRenderingNarrowingDisclosure> {
    surface_disclosures(labels, BundleNarrowingDisclosureState::ParityPreserved)
}

/// Disclosures for a narrowed (yellow) row: the narrower surfaces disclose their
/// reduced interactions while preserving labels.
fn narrowed_surfaces(labels: &[&str]) -> Vec<BundleRenderingNarrowingDisclosure> {
    surface_disclosures(labels, BundleNarrowingDisclosureState::DisclosedNarrowed)
}

fn rendering_surfaces() -> Vec<M5BundleRenderingSurface> {
    vec![
        M5BundleRenderingSurface::DesktopFull,
        M5BundleRenderingSurface::CliHeadless,
        M5BundleRenderingSurface::SupportExport,
    ]
}

fn seeded_rows() -> Vec<BundleAccessibilityRow> {
    vec![
        // Start-center bundle card — certification evidence intact; the card offers a
        // fully certified, self-sufficient bundle and is fully reachable on every
        // surface (green).
        BundleAccessibilityRow {
            record_kind: BUNDLE_A11Y_FALLBACK_ROW_RECORD_KIND.to_owned(),
            schema_version: BUNDLE_A11Y_FALLBACK_SCHEMA_VERSION,
            row_id: "a11y:start-center-bundle-card".to_owned(),
            component_family: M5WorkflowBundleComponentFamily::StartCenterBundleCard,
            source_family_schema_ref: BUNDLE_A11Y_FALLBACK_COMPONENT_MATRIX_REF.to_owned(),
            bundle_context_ref: "bundle:stack:0001".to_owned(),
            fallback_modalities: vec![
                M5BundleFallbackModality::Structured,
                M5BundleFallbackModality::List,
                M5BundleFallbackModality::Textual,
                M5BundleFallbackModality::Cli,
            ],
            reaches_canonical_truth: true,
            keyboard_reach: BundleNonVisualReachState::ReachableAndLabeled,
            screen_reader_reach: BundleNonVisualReachState::ReachableAndLabeled,
            cli_reach: BundleNonVisualReachState::ReachableAndLabeled,
            export_summary: BundleExportSummaryState::ReconstructableWithoutScreenshot,
            export_summary_ref: "summary:start-center-bundle-card:a11y".to_owned(),
            copy_export: copy_export(&[
                "bundle_id",
                "signer_source",
                "support_class",
                "cert_freshness",
            ]),
            full_support_claim: M5BundleSupportClaim::Certified,
            claim_conditions: vec![condition(
                M5BundleClaimDimension::CertificationEvidence,
                M5BundleClaimConditionState::Intact,
            )],
            claim_narrow: None,
            rendering_surfaces: rendering_surfaces(),
            narrowing_disclosures: parity_surfaces(&[
                "bundle_id",
                "signer_source",
                "cert_freshness",
            ]),
            required_labels: all_required_labels(),
            consumer_surfaces: base_consumers(&[
                M5BundleDisclosureSurfaceFamily::StartCenterClassCard,
                M5BundleDisclosureSurfaceFamily::DocsHelpClassBlock,
            ]),
            source_refs: vec![
                "UI/UX Spec §6.15".to_owned(),
                BUNDLE_A11Y_FALLBACK_DOC_REF.to_owned(),
            ],
            observed_at: "2026-07-06T00:00:00Z".to_owned(),
            evidence_refs: ev("start-center-bundle-card"),
        },
        // Certified-archetype badge group — certification evidence stale, so the
        // badge claim auto-narrows to retest-pending: the badges stay visible but no
        // longer read as currently certified (yellow).
        BundleAccessibilityRow {
            record_kind: BUNDLE_A11Y_FALLBACK_ROW_RECORD_KIND.to_owned(),
            schema_version: BUNDLE_A11Y_FALLBACK_SCHEMA_VERSION,
            row_id: "a11y:certified-archetype-badge-group".to_owned(),
            component_family: M5WorkflowBundleComponentFamily::CertifiedArchetypeBadgeGroup,
            source_family_schema_ref: BUNDLE_A11Y_FALLBACK_COMPONENT_MATRIX_REF.to_owned(),
            bundle_context_ref: "archetype:family:0002".to_owned(),
            fallback_modalities: vec![
                M5BundleFallbackModality::List,
                M5BundleFallbackModality::Textual,
                M5BundleFallbackModality::Cli,
            ],
            reaches_canonical_truth: true,
            keyboard_reach: BundleNonVisualReachState::ReachableAndLabeled,
            screen_reader_reach: BundleNonVisualReachState::ReachableAndLabeled,
            cli_reach: BundleNonVisualReachState::ReachableAndLabeled,
            export_summary: BundleExportSummaryState::ReconstructableWithoutScreenshot,
            export_summary_ref: "summary:certified-archetype-badge-group:a11y".to_owned(),
            copy_export: copy_export(&[
                "archetype_family",
                "cert_target",
                "cert_freshness",
                "badge_count",
            ]),
            full_support_claim: M5BundleSupportClaim::Certified,
            claim_conditions: vec![condition(
                M5BundleClaimDimension::CertificationEvidence,
                M5BundleClaimConditionState::Stale,
            )],
            claim_narrow: Some(BundleClaimAutoNarrow {
                narrowed_to: M5BundleSupportClaim::RetestPending,
                binding_dimension: M5BundleClaimDimension::CertificationEvidence,
                trigger: M5BundleComponentDowngradeTrigger::StaleCertification,
                narrowed_label: "Certification aged — badges shown retest-pending, not current"
                    .to_owned(),
                preserves_canonical_identity: true,
            }),
            rendering_surfaces: rendering_surfaces(),
            narrowing_disclosures: narrowed_surfaces(&[
                "archetype_family",
                "cert_target",
                "cert_freshness",
            ]),
            required_labels: all_required_labels(),
            consumer_surfaces: base_consumers(&[
                M5BundleDisclosureSurfaceFamily::StartCenterClassCard,
            ]),
            source_refs: vec![
                "UX Guide §16.21".to_owned(),
                BUNDLE_A11Y_FALLBACK_DOC_REF.to_owned(),
            ],
            observed_at: "2026-07-06T00:00:00Z".to_owned(),
            evidence_refs: ev("certified-archetype-badge-group"),
        },
        // Bundle detail page — hierarchy-heavy (component / dependency inventory); a
        // required entitlement / policy dependency is unmet, so the page auto-narrows
        // to policy-blocked and binds its inventory tree to a flat list / textual
        // path (yellow).
        BundleAccessibilityRow {
            record_kind: BUNDLE_A11Y_FALLBACK_ROW_RECORD_KIND.to_owned(),
            schema_version: BUNDLE_A11Y_FALLBACK_SCHEMA_VERSION,
            row_id: "a11y:bundle-detail-page".to_owned(),
            component_family: M5WorkflowBundleComponentFamily::BundleDetailPage,
            source_family_schema_ref: BUNDLE_A11Y_FALLBACK_COMPONENT_MATRIX_REF.to_owned(),
            bundle_context_ref: "bundle:stack:0003".to_owned(),
            fallback_modalities: vec![
                M5BundleFallbackModality::Structured,
                M5BundleFallbackModality::List,
                M5BundleFallbackModality::Textual,
                M5BundleFallbackModality::Cli,
            ],
            reaches_canonical_truth: true,
            keyboard_reach: BundleNonVisualReachState::ReachableAndLabeled,
            screen_reader_reach: BundleNonVisualReachState::DisclosedReducedButReachable,
            cli_reach: BundleNonVisualReachState::ReachableAndLabeled,
            export_summary: BundleExportSummaryState::ReconstructableWithoutScreenshot,
            export_summary_ref: "summary:bundle-detail-page:a11y".to_owned(),
            copy_export: copy_export(&[
                "bundle_id",
                "signer_source",
                "entitlement_deps",
                "mirror_posture",
            ]),
            full_support_claim: M5BundleSupportClaim::Certified,
            claim_conditions: vec![condition(
                M5BundleClaimDimension::DependencyPosture,
                M5BundleClaimConditionState::PolicyBlocked,
            )],
            claim_narrow: Some(BundleClaimAutoNarrow {
                narrowed_to: M5BundleSupportClaim::PolicyBlocked,
                binding_dimension: M5BundleClaimDimension::DependencyPosture,
                trigger: M5BundleComponentDowngradeTrigger::EntitlementDependencyUnmet,
                narrowed_label: "Entitlement dependency unmet — bundle blocked by policy"
                    .to_owned(),
                preserves_canonical_identity: true,
            }),
            rendering_surfaces: rendering_surfaces(),
            narrowing_disclosures: narrowed_surfaces(&[
                "bundle_id",
                "signer_source",
                "entitlement_deps",
            ]),
            required_labels: all_required_labels(),
            consumer_surfaces: base_consumers(&[
                M5BundleDisclosureSurfaceFamily::BundleDetailClassPanel,
            ]),
            source_refs: vec![
                "UI/UX Spec §6.15".to_owned(),
                BUNDLE_A11Y_FALLBACK_DOC_REF.to_owned(),
            ],
            observed_at: "2026-07-06T00:00:00Z".to_owned(),
            evidence_refs: ev("bundle-detail-page"),
        },
        // Install / update review sheet — its full claim is Supported by nature (it
        // reviews before applying, it does not itself certify); artifact
        // availability is intact, so it stays green without narrowing.
        BundleAccessibilityRow {
            record_kind: BUNDLE_A11Y_FALLBACK_ROW_RECORD_KIND.to_owned(),
            schema_version: BUNDLE_A11Y_FALLBACK_SCHEMA_VERSION,
            row_id: "a11y:bundle-install-update-review-sheet".to_owned(),
            component_family: M5WorkflowBundleComponentFamily::BundleInstallUpdateReviewSheet,
            source_family_schema_ref: BUNDLE_A11Y_FALLBACK_COMPONENT_MATRIX_REF.to_owned(),
            bundle_context_ref: "bundle:stack:0004".to_owned(),
            fallback_modalities: vec![
                M5BundleFallbackModality::List,
                M5BundleFallbackModality::Textual,
                M5BundleFallbackModality::Cli,
            ],
            reaches_canonical_truth: true,
            keyboard_reach: BundleNonVisualReachState::ReachableAndLabeled,
            screen_reader_reach: BundleNonVisualReachState::ReachableAndLabeled,
            cli_reach: BundleNonVisualReachState::ReachableAndLabeled,
            export_summary: BundleExportSummaryState::ReconstructableWithoutScreenshot,
            export_summary_ref: "summary:bundle-install-update-review-sheet:a11y".to_owned(),
            copy_export: copy_export(&[
                "bundle_id",
                "diff_scope",
                "local_override_state",
                "resolution",
            ]),
            full_support_claim: M5BundleSupportClaim::Supported,
            claim_conditions: vec![condition(
                M5BundleClaimDimension::ArtifactAvailability,
                M5BundleClaimConditionState::Intact,
            )],
            claim_narrow: None,
            rendering_surfaces: rendering_surfaces(),
            narrowing_disclosures: parity_surfaces(&["bundle_id", "diff_scope", "resolution"]),
            required_labels: all_required_labels(),
            consumer_surfaces: base_consumers(&[
                M5BundleDisclosureSurfaceFamily::BundleDetailClassPanel,
                M5BundleDisclosureSurfaceFamily::MigrationClassDisclosureRow,
            ]),
            source_refs: vec![
                "UX Guide §16.20".to_owned(),
                BUNDLE_A11Y_FALLBACK_DOC_REF.to_owned(),
            ],
            observed_at: "2026-07-06T00:00:00Z".to_owned(),
            evidence_refs: ev("bundle-install-update-review-sheet"),
        },
        // Bundle drift banner — local bundle state has partially drifted, so the
        // bundle's claim auto-narrows to limited: it stays usable but its scope is
        // reduced and disclosed (yellow).
        BundleAccessibilityRow {
            record_kind: BUNDLE_A11Y_FALLBACK_ROW_RECORD_KIND.to_owned(),
            schema_version: BUNDLE_A11Y_FALLBACK_SCHEMA_VERSION,
            row_id: "a11y:bundle-drift-banner".to_owned(),
            component_family: M5WorkflowBundleComponentFamily::BundleDriftBanner,
            source_family_schema_ref: BUNDLE_A11Y_FALLBACK_COMPONENT_MATRIX_REF.to_owned(),
            bundle_context_ref: "bundle:stack:0005".to_owned(),
            fallback_modalities: vec![
                M5BundleFallbackModality::List,
                M5BundleFallbackModality::Textual,
                M5BundleFallbackModality::Cli,
            ],
            reaches_canonical_truth: true,
            keyboard_reach: BundleNonVisualReachState::ReachableAndLabeled,
            screen_reader_reach: BundleNonVisualReachState::ReachableAndLabeled,
            cli_reach: BundleNonVisualReachState::ReachableAndLabeled,
            export_summary: BundleExportSummaryState::ReconstructableWithoutScreenshot,
            export_summary_ref: "summary:bundle-drift-banner:a11y".to_owned(),
            copy_export: copy_export(&[
                "bundle_id",
                "drift_state",
                "override_state",
                "signer_source",
            ]),
            full_support_claim: M5BundleSupportClaim::Certified,
            claim_conditions: vec![condition(
                M5BundleClaimDimension::BundleFreshness,
                M5BundleClaimConditionState::Partial,
            )],
            claim_narrow: Some(BundleClaimAutoNarrow {
                narrowed_to: M5BundleSupportClaim::Limited,
                binding_dimension: M5BundleClaimDimension::BundleFreshness,
                trigger: M5BundleComponentDowngradeTrigger::LocalOverrideDrift,
                narrowed_label:
                    "Local overrides diverged — bundle support limited to unchanged scope"
                        .to_owned(),
                preserves_canonical_identity: true,
            }),
            rendering_surfaces: rendering_surfaces(),
            narrowing_disclosures: narrowed_surfaces(&[
                "bundle_id",
                "drift_state",
                "override_state",
            ]),
            required_labels: all_required_labels(),
            consumer_surfaces: base_consumers(&[
                M5BundleDisclosureSurfaceFamily::BundleDetailClassPanel,
            ]),
            source_refs: vec![
                "UI/UX Spec §23.49".to_owned(),
                BUNDLE_A11Y_FALLBACK_DOC_REF.to_owned(),
            ],
            observed_at: "2026-07-06T00:00:00Z".to_owned(),
            evidence_refs: ev("bundle-drift-banner"),
        },
        // Bundle local-override row — freshness intact, but the overridden asset is
        // only served from a stale mirror, so the claim auto-narrows to mirror-only
        // (yellow).
        BundleAccessibilityRow {
            record_kind: BUNDLE_A11Y_FALLBACK_ROW_RECORD_KIND.to_owned(),
            schema_version: BUNDLE_A11Y_FALLBACK_SCHEMA_VERSION,
            row_id: "a11y:bundle-local-override-row".to_owned(),
            component_family: M5WorkflowBundleComponentFamily::BundleLocalOverrideRow,
            source_family_schema_ref: BUNDLE_A11Y_FALLBACK_COMPONENT_MATRIX_REF.to_owned(),
            bundle_context_ref: "bundle:asset:0006".to_owned(),
            fallback_modalities: vec![
                M5BundleFallbackModality::List,
                M5BundleFallbackModality::Textual,
                M5BundleFallbackModality::Cli,
            ],
            reaches_canonical_truth: true,
            keyboard_reach: BundleNonVisualReachState::ReachableAndLabeled,
            screen_reader_reach: BundleNonVisualReachState::ReachableAndLabeled,
            cli_reach: BundleNonVisualReachState::ReachableAndLabeled,
            export_summary: BundleExportSummaryState::ReconstructableWithoutScreenshot,
            export_summary_ref: "summary:bundle-local-override-row:a11y".to_owned(),
            copy_export: copy_export(&["asset_ref", "ownership", "resolution", "mirror_posture"]),
            full_support_claim: M5BundleSupportClaim::Certified,
            claim_conditions: vec![
                condition(
                    M5BundleClaimDimension::BundleFreshness,
                    M5BundleClaimConditionState::Intact,
                ),
                condition(
                    M5BundleClaimDimension::ArtifactAvailability,
                    M5BundleClaimConditionState::MirrorStale,
                ),
            ],
            claim_narrow: Some(BundleClaimAutoNarrow {
                narrowed_to: M5BundleSupportClaim::MirrorOnly,
                binding_dimension: M5BundleClaimDimension::ArtifactAvailability,
                trigger: M5BundleComponentDowngradeTrigger::MirrorStale,
                narrowed_label: "Override asset served from a stale mirror — mirror-only"
                    .to_owned(),
                preserves_canonical_identity: true,
            }),
            rendering_surfaces: rendering_surfaces(),
            narrowing_disclosures: narrowed_surfaces(&["asset_ref", "ownership", "mirror_posture"]),
            required_labels: all_required_labels(),
            consumer_surfaces: base_consumers(&[
                M5BundleDisclosureSurfaceFamily::BundleDetailClassPanel,
            ]),
            source_refs: vec![
                "UI/UX Spec §23.49".to_owned(),
                BUNDLE_A11Y_FALLBACK_DOC_REF.to_owned(),
            ],
            observed_at: "2026-07-06T00:00:00Z".to_owned(),
            evidence_refs: ev("bundle-local-override-row"),
        },
        // Bundle rollback / remove card — the rollback checkpoint is only reachable
        // from an offline cache, so the claim auto-narrows to offline-cache-only
        // (yellow).
        BundleAccessibilityRow {
            record_kind: BUNDLE_A11Y_FALLBACK_ROW_RECORD_KIND.to_owned(),
            schema_version: BUNDLE_A11Y_FALLBACK_SCHEMA_VERSION,
            row_id: "a11y:bundle-rollback-remove-card".to_owned(),
            component_family: M5WorkflowBundleComponentFamily::BundleRollbackRemoveCard,
            source_family_schema_ref: BUNDLE_A11Y_FALLBACK_COMPONENT_MATRIX_REF.to_owned(),
            bundle_context_ref: "bundle:stack:0007".to_owned(),
            fallback_modalities: vec![
                M5BundleFallbackModality::List,
                M5BundleFallbackModality::Textual,
                M5BundleFallbackModality::Cli,
            ],
            reaches_canonical_truth: true,
            keyboard_reach: BundleNonVisualReachState::ReachableAndLabeled,
            screen_reader_reach: BundleNonVisualReachState::ReachableAndLabeled,
            cli_reach: BundleNonVisualReachState::ReachableAndLabeled,
            export_summary: BundleExportSummaryState::ReconstructableWithoutScreenshot,
            export_summary_ref: "summary:bundle-rollback-remove-card:a11y".to_owned(),
            copy_export: copy_export(&[
                "bundle_id",
                "rollback_path",
                "side_effects",
                "removable_ownership",
            ]),
            full_support_claim: M5BundleSupportClaim::Certified,
            claim_conditions: vec![condition(
                M5BundleClaimDimension::ArtifactAvailability,
                M5BundleClaimConditionState::OfflineOnly,
            )],
            claim_narrow: Some(BundleClaimAutoNarrow {
                narrowed_to: M5BundleSupportClaim::OfflineCacheOnly,
                binding_dimension: M5BundleClaimDimension::ArtifactAvailability,
                trigger: M5BundleComponentDowngradeTrigger::OfflineCacheOnly,
                narrowed_label: "Rollback checkpoint reachable from offline cache only".to_owned(),
                preserves_canonical_identity: true,
            }),
            rendering_surfaces: rendering_surfaces(),
            narrowing_disclosures: narrowed_surfaces(&[
                "bundle_id",
                "rollback_path",
                "side_effects",
            ]),
            required_labels: all_required_labels(),
            consumer_surfaces: base_consumers(&[
                M5BundleDisclosureSurfaceFamily::BundleDetailClassPanel,
            ]),
            source_refs: vec![
                "UI/UX Spec §23.49".to_owned(),
                BUNDLE_A11Y_FALLBACK_DOC_REF.to_owned(),
            ],
            observed_at: "2026-07-06T00:00:00Z".to_owned(),
            evidence_refs: ev("bundle-rollback-remove-card"),
        },
        // Bundle class-disclosure card — the bundle is an imported / bridged user
        // handoff, not a native first-party read, so the claim auto-narrows to
        // imported (yellow).
        BundleAccessibilityRow {
            record_kind: BUNDLE_A11Y_FALLBACK_ROW_RECORD_KIND.to_owned(),
            schema_version: BUNDLE_A11Y_FALLBACK_SCHEMA_VERSION,
            row_id: "a11y:bundle-class-disclosure-card".to_owned(),
            component_family: M5WorkflowBundleComponentFamily::BundleClassDisclosureCard,
            source_family_schema_ref: BUNDLE_A11Y_FALLBACK_COMPONENT_MATRIX_REF.to_owned(),
            bundle_context_ref: "bundle:stack:0008".to_owned(),
            fallback_modalities: vec![
                M5BundleFallbackModality::List,
                M5BundleFallbackModality::Textual,
                M5BundleFallbackModality::Cli,
            ],
            reaches_canonical_truth: true,
            keyboard_reach: BundleNonVisualReachState::ReachableAndLabeled,
            screen_reader_reach: BundleNonVisualReachState::ReachableAndLabeled,
            cli_reach: BundleNonVisualReachState::ReachableAndLabeled,
            export_summary: BundleExportSummaryState::ReconstructableWithoutScreenshot,
            export_summary_ref: "summary:bundle-class-disclosure-card:a11y".to_owned(),
            copy_export: copy_export(&[
                "bundle_class",
                "signer_source",
                "cert_target",
                "scorecard_class",
            ]),
            full_support_claim: M5BundleSupportClaim::Certified,
            claim_conditions: vec![condition(
                M5BundleClaimDimension::SourceProvenance,
                M5BundleClaimConditionState::Imported,
            )],
            claim_narrow: Some(BundleClaimAutoNarrow {
                narrowed_to: M5BundleSupportClaim::Imported,
                binding_dimension: M5BundleClaimDimension::SourceProvenance,
                trigger: M5BundleComponentDowngradeTrigger::ImportedNotNative,
                narrowed_label: "Imported user handoff — not a native first-party bundle"
                    .to_owned(),
                preserves_canonical_identity: true,
            }),
            rendering_surfaces: rendering_surfaces(),
            narrowing_disclosures: narrowed_surfaces(&[
                "bundle_class",
                "signer_source",
                "cert_target",
            ]),
            required_labels: all_required_labels(),
            consumer_surfaces: base_consumers(&[
                M5BundleDisclosureSurfaceFamily::MigrationClassDisclosureRow,
            ]),
            source_refs: vec![
                "UX Guide §16.21".to_owned(),
                BUNDLE_A11Y_FALLBACK_DOC_REF.to_owned(),
            ],
            observed_at: "2026-07-06T00:00:00Z".to_owned(),
            evidence_refs: ev("bundle-class-disclosure-card"),
        },
        // Bundle claim-narrowing row — certification evidence stale, so the row does
        // exactly what it exists to do: auto-narrow the bundle claim to
        // retest-pending and name the reason (yellow).
        BundleAccessibilityRow {
            record_kind: BUNDLE_A11Y_FALLBACK_ROW_RECORD_KIND.to_owned(),
            schema_version: BUNDLE_A11Y_FALLBACK_SCHEMA_VERSION,
            row_id: "a11y:bundle-claim-narrowing-row".to_owned(),
            component_family: M5WorkflowBundleComponentFamily::BundleClaimNarrowingRow,
            source_family_schema_ref: BUNDLE_A11Y_FALLBACK_COMPONENT_MATRIX_REF.to_owned(),
            bundle_context_ref: "bundle:stack:0009".to_owned(),
            fallback_modalities: vec![
                M5BundleFallbackModality::List,
                M5BundleFallbackModality::Textual,
                M5BundleFallbackModality::Cli,
            ],
            reaches_canonical_truth: true,
            keyboard_reach: BundleNonVisualReachState::ReachableAndLabeled,
            screen_reader_reach: BundleNonVisualReachState::ReachableAndLabeled,
            cli_reach: BundleNonVisualReachState::ReachableAndLabeled,
            export_summary: BundleExportSummaryState::ReconstructableWithoutScreenshot,
            export_summary_ref: "summary:bundle-claim-narrowing-row:a11y".to_owned(),
            copy_export: copy_export(&[
                "bundle_id",
                "cert_freshness",
                "imported_confidence",
                "narrowing_reason",
            ]),
            full_support_claim: M5BundleSupportClaim::Certified,
            claim_conditions: vec![condition(
                M5BundleClaimDimension::CertificationEvidence,
                M5BundleClaimConditionState::Stale,
            )],
            claim_narrow: Some(BundleClaimAutoNarrow {
                narrowed_to: M5BundleSupportClaim::RetestPending,
                binding_dimension: M5BundleClaimDimension::CertificationEvidence,
                trigger: M5BundleComponentDowngradeTrigger::StaleCertification,
                narrowed_label: "Certification stale — claim narrowed to retest-pending".to_owned(),
                preserves_canonical_identity: true,
            }),
            rendering_surfaces: rendering_surfaces(),
            narrowing_disclosures: narrowed_surfaces(&[
                "bundle_id",
                "cert_freshness",
                "narrowing_reason",
            ]),
            required_labels: all_required_labels(),
            consumer_surfaces: base_consumers(&[
                M5BundleDisclosureSurfaceFamily::MigrationClassDisclosureRow,
                M5BundleDisclosureSurfaceFamily::DocsHelpClassBlock,
            ]),
            source_refs: vec![
                "UI/UX Spec §23.49".to_owned(),
                BUNDLE_A11Y_FALLBACK_DOC_REF.to_owned(),
            ],
            observed_at: "2026-07-06T00:00:00Z".to_owned(),
            evidence_refs: ev("bundle-claim-narrowing-row"),
        },
    ]
}
