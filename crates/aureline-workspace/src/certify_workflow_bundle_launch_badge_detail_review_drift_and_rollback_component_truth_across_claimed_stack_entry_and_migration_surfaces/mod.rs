//! Surface certification for the M5 workflow-bundle, launch-badge, detail-review,
//! drift, and rollback components.
//!
//! This module is the M05-851 certification capstone that CLOSES the frozen M5
//! workflow-bundle component lane
//! ([`crate::freeze_the_m5_workflow_bundle_launch_badge_detail_review_drift_and_rollback_component_matrix`]).
//! Where the freeze matrix (M05-844) defines the reusable start-center bundle card,
//! certified-archetype badge group, bundle detail page, install/update review sheet,
//! drift banner, local-override row, rollback/remove card, class-disclosure card, and
//! claim-narrowing row primitives, the 845-849 implementation lanes resolve their
//! per-surface truth, and the M05-850 accessibility capstone certifies keyboard /
//! screen-reader / CLI / export parity per family, this lane keys on the **claimed M5
//! stack-entry and migration surface** and certifies that the shared component family
//! behaves consistently on every consumer:
//!
//! - **Certify or auto-narrow (AC1).** Each surface either passes the shared
//!   workflow-bundle component packet (green) or auto-narrows its bundle-support claim
//!   to limited / retest-pending / imported / mirror-only / offline-cache-only /
//!   policy-blocked (yellow), disclosing the binding component group and the frozen
//!   downgrade trigger. A surface that hides drift, over-asserts support, or drops
//!   export truth is blocked (red) and may not ship.
//! - **Degraded paths narrow visibly (AC2).** Compatibility across the
//!   native / mirror / offline / managed / imported distribution paths is captured per
//!   surface; a path whose parity is not current forces the claim to narrow rather than
//!   inheriting a full-truth label from a healthier lane. The support / release export
//!   always reconstructs each surface's meaning from typed tokens without a screenshot.
//! - **Anchored to a reusable component family (AC3).** Every surface cites the ONE
//!   canonical workflow-bundle component bundle and references the canonical component
//!   families it consumes, so M5 stack-entry and migration claims are anchored to a
//!   shared component family rather than feature-local registry or onboarding chrome.
//!
//! Each [`BundleSurfaceCertRow`] keys on one [`M5WorkflowBundleClaimedSurface`] and
//! reuses the frozen [`M5WorkflowBundleComponentFamily`], [`M5BundleRequiredLabel`],
//! and [`M5BundleComponentDowngradeTrigger`] vocabulary plus the shared
//! [`M5BundleSupportClaim`] claim tier rather than minting parallel synonyms, so the
//! certified labels stay byte-identical to the matrix and the sibling primitive and
//! accessibility packets.
//!
//! The packet is metadata-only: raw manifest bytes, credentials, entitlement tokens,
//! mirror URLs, and provider cursors never cross this boundary; the packet carries only
//! typed class tokens, opaque summary / evidence refs, booleans, and redacted labels.
//!
//! The boundary schema is
//! [`schemas/ui/m5-workflow-bundle-surface-certification.schema.json`](../../../../schemas/ui/m5-workflow-bundle-surface-certification.schema.json).
//! The contract doc is
//! [`docs/bundles/m5_workflow_bundle_surface_certification.md`](../../../../docs/bundles/m5_workflow_bundle_surface_certification.md).

#[cfg(test)]
mod tests;

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

use crate::freeze_the_m5_workflow_bundle_launch_badge_detail_review_drift_and_rollback_component_matrix::{
    M5BundleComponentDowngradeTrigger, M5BundleRequiredLabel, M5WorkflowBundleComponentFamily,
};
use crate::implement_keyboard_screen_reader_cli_export_parity_and_bundle_claim_auto_narrowing::M5BundleSupportClaim;

/// Schema version stamped on the M05-851 workflow-bundle surface certification packet.
pub const BUNDLE_SURFACE_CERT_SCHEMA_VERSION: u32 = 1;

/// Stable record-kind tag carried by [`BundleSurfaceCertPacket`].
pub const BUNDLE_SURFACE_CERT_RECORD_KIND: &str = "m5_workflow_bundle_surface_certification_packet";

/// Stable record-kind tag carried by each [`BundleSurfaceCertRow`].
pub const BUNDLE_SURFACE_CERT_ROW_RECORD_KIND: &str =
    "m5_workflow_bundle_surface_certification_row";

/// Repo-relative path of the boundary schema.
pub const BUNDLE_SURFACE_CERT_SCHEMA_REF: &str =
    "schemas/ui/m5-workflow-bundle-surface-certification.schema.json";

/// Repo-relative path of the contract doc.
pub const BUNDLE_SURFACE_CERT_DOC_REF: &str =
    "docs/bundles/m5_workflow_bundle_surface_certification.md";

/// Repo-relative path of the frozen workflow-bundle component matrix this lane
/// certifies against.
pub const BUNDLE_SURFACE_CERT_COMPONENT_MATRIX_REF: &str =
    "schemas/ui/m5-workflow-bundle-component-matrix.schema.json";

/// Repo-relative path of the ONE canonical workflow-bundle component bundle every
/// certified surface cites. This is the frozen M05-844 release proof — the single
/// source of truth for the reusable component family.
pub const BUNDLE_SURFACE_CERT_BUNDLE_REF: &str =
    "artifacts/release/m5-workflow-bundle-component-proof/support_export.json";

/// Repo-relative path of the protected fixture directory.
pub const BUNDLE_SURFACE_CERT_FIXTURE_DIR: &str =
    "fixtures/ui/m5-workflow-bundle-surface-certification";

/// Repo-relative path of the checked support-export artifact (the `include_str!`
/// canonical).
pub const BUNDLE_SURFACE_CERT_ARTIFACT_REF: &str =
    "artifacts/release/m5-workflow-bundle-surface-certification-proof/support_export.json";

/// Repo-relative path of the checked machine-readable matrix CSV.
pub const BUNDLE_SURFACE_CERT_CSV_REF: &str =
    "artifacts/release/m5-workflow-bundle-surface-certification-proof/matrix.csv";

/// Repo-relative path of the checked Markdown report.
pub const BUNDLE_SURFACE_CERT_REPORT_REF: &str =
    "artifacts/release/m5-workflow-bundle-surface-certification-proof/report.md";

/// The claimed M5 stack-entry / migration surface a certification row keys on. The
/// first six are interactive bundle consumers; the last three are release-evidence
/// surfaces that publish and replay the certified truth.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5WorkflowBundleClaimedSurface {
    /// The start-center bundle picker surface.
    StartCenterPicker,
    /// The onboarding / guided stack-entry flow surface.
    OnboardingFlow,
    /// The migration-center surface (imported-user handoff, migration gaps).
    MigrationCenter,
    /// The docs / help center surface.
    DocsHelp,
    /// The diagnostics surface.
    Diagnostics,
    /// The CLI / headless consumer surface.
    CliHeadless,
    /// The support / export replay surface (release evidence).
    SupportExportReplay,
    /// The docs / help embeds surface (release evidence).
    DocsHelpEmbeds,
    /// The release-proof surface (release evidence).
    ReleaseProof,
}

impl M5WorkflowBundleClaimedSurface {
    /// Every claimed surface, in declaration order.
    pub const ALL: [Self; 9] = [
        Self::StartCenterPicker,
        Self::OnboardingFlow,
        Self::MigrationCenter,
        Self::DocsHelp,
        Self::Diagnostics,
        Self::CliHeadless,
        Self::SupportExportReplay,
        Self::DocsHelpEmbeds,
        Self::ReleaseProof,
    ];

    /// The release-evidence surfaces that must each be certified so claim publication
    /// and field triage stay anchored to the same component truth.
    pub const EVIDENCE_SURFACES: [Self; 3] = [
        Self::SupportExportReplay,
        Self::DocsHelpEmbeds,
        Self::ReleaseProof,
    ];

    /// Returns true when the surface is a release-evidence surface rather than an
    /// interactive bundle consumer.
    pub const fn is_evidence(self) -> bool {
        matches!(
            self,
            Self::SupportExportReplay | Self::DocsHelpEmbeds | Self::ReleaseProof
        )
    }

    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::StartCenterPicker => "start_center_picker",
            Self::OnboardingFlow => "onboarding_flow",
            Self::MigrationCenter => "migration_center",
            Self::DocsHelp => "docs_help",
            Self::Diagnostics => "diagnostics",
            Self::CliHeadless => "cli_headless",
            Self::SupportExportReplay => "support_export_replay",
            Self::DocsHelpEmbeds => "docs_help_embeds",
            Self::ReleaseProof => "release_proof",
        }
    }

    /// Human-readable label for the Markdown report.
    pub const fn label(self) -> &'static str {
        match self {
            Self::StartCenterPicker => "Start-center bundle picker",
            Self::OnboardingFlow => "Onboarding / guided stack entry",
            Self::MigrationCenter => "Migration center",
            Self::DocsHelp => "Docs / help center",
            Self::Diagnostics => "Diagnostics",
            Self::CliHeadless => "CLI / headless",
            Self::SupportExportReplay => "Support / export replay",
            Self::DocsHelpEmbeds => "Docs / help embeds",
            Self::ReleaseProof => "Release proof",
        }
    }
}

/// A reusable workflow-bundle component group a surface consumes. Each group maps to
/// one or more frozen [`M5WorkflowBundleComponentFamily`] and drives exactly one
/// certification truth axis.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5WorkflowBundleComponentGroup {
    /// The launch-wedge component group (start-center bundle card + certified-archetype
    /// badge group).
    LaunchWedge,
    /// The detail / review component group (bundle detail page + install/update review
    /// sheet).
    DetailReview,
    /// The drift / override component group (drift banner + local-override row).
    DriftOverride,
    /// The rollback / remove component group (rollback/remove card).
    RollbackRemove,
    /// The class-disclosure component group (class-disclosure card + claim-narrowing
    /// row).
    ClassDisclosure,
}

impl M5WorkflowBundleComponentGroup {
    /// Every component group, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::LaunchWedge,
        Self::DetailReview,
        Self::DriftOverride,
        Self::RollbackRemove,
        Self::ClassDisclosure,
    ];

    /// The frozen component families this group maps to.
    pub fn families(self) -> Vec<M5WorkflowBundleComponentFamily> {
        match self {
            Self::LaunchWedge => vec![
                M5WorkflowBundleComponentFamily::StartCenterBundleCard,
                M5WorkflowBundleComponentFamily::CertifiedArchetypeBadgeGroup,
            ],
            Self::DetailReview => vec![
                M5WorkflowBundleComponentFamily::BundleDetailPage,
                M5WorkflowBundleComponentFamily::BundleInstallUpdateReviewSheet,
            ],
            Self::DriftOverride => vec![
                M5WorkflowBundleComponentFamily::BundleDriftBanner,
                M5WorkflowBundleComponentFamily::BundleLocalOverrideRow,
            ],
            Self::RollbackRemove => {
                vec![M5WorkflowBundleComponentFamily::BundleRollbackRemoveCard]
            }
            Self::ClassDisclosure => vec![
                M5WorkflowBundleComponentFamily::BundleClassDisclosureCard,
                M5WorkflowBundleComponentFamily::BundleClaimNarrowingRow,
            ],
        }
    }

    /// The frozen downgrade trigger a narrowing of this group binds to.
    pub const fn default_trigger(self) -> M5BundleComponentDowngradeTrigger {
        match self {
            Self::LaunchWedge => M5BundleComponentDowngradeTrigger::StaleCertification,
            Self::DetailReview => M5BundleComponentDowngradeTrigger::EntitlementDependencyUnmet,
            Self::DriftOverride => M5BundleComponentDowngradeTrigger::LocalOverrideDrift,
            Self::RollbackRemove => M5BundleComponentDowngradeTrigger::RollbackOnlyPath,
            Self::ClassDisclosure => M5BundleComponentDowngradeTrigger::ImportedNotNative,
        }
    }

    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LaunchWedge => "launch_wedge",
            Self::DetailReview => "detail_review",
            Self::DriftOverride => "drift_override",
            Self::RollbackRemove => "rollback_remove",
            Self::ClassDisclosure => "class_disclosure",
        }
    }
}

/// A bundle distribution path whose native/mirror/offline/managed/imported parity the
/// certification captures.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5BundleDistributionPath {
    /// Served live from a first-party registry.
    Native,
    /// Served from a registry mirror (freshness bounded by mirror age).
    Mirror,
    /// Served from a cached-offline snapshot; no live registry reachable.
    Offline,
    /// Served through a managed / org-approved entitlement plane.
    Managed,
    /// Imported / bridged from another tool or user handoff.
    Imported,
}

impl M5BundleDistributionPath {
    /// Every distribution path, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::Native,
        Self::Mirror,
        Self::Offline,
        Self::Managed,
        Self::Imported,
    ];

    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Native => "native",
            Self::Mirror => "mirror",
            Self::Offline => "offline",
            Self::Managed => "managed",
            Self::Imported => "imported",
        }
    }
}

/// Whether a distribution path's parity is current, disclosed-degraded, or unsupported.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5BundleDistributionParityState {
    /// The path's component parity is current on this surface.
    Current,
    /// The path's parity is degraded but disclosed (forces a narrowed claim).
    DisclosedNarrowed,
    /// The path is unsupported on this surface (forces a blocked claim).
    Unsupported,
}

impl M5BundleDistributionParityState {
    /// Returns true when the path is current.
    pub const fn is_current(self) -> bool {
        matches!(self, Self::Current)
    }

    /// Returns true when the path carries a disclosed degradation.
    pub const fn is_disclosed_reduction(self) -> bool {
        matches!(self, Self::DisclosedNarrowed)
    }

    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Current => "current",
            Self::DisclosedNarrowed => "disclosed_narrowed",
            Self::Unsupported => "unsupported",
        }
    }
}

/// Generates a gated per-group truth axis: certified / disclosed-narrowed / blocked,
/// plus `not_applicable` when the surface does not consume the group.
macro_rules! gated_truth_axis {
    ($name:ident, $doc:literal) => {
        #[doc = $doc]
        #[derive(
            Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize,
        )]
        #[serde(rename_all = "snake_case")]
        pub enum $name {
            /// The component group is certified: full component truth on this surface.
            Certified,
            /// The component group is reduced but disclosed (yellow).
            DisclosedNarrowed,
            /// The component group hides drift or over-claims (red).
            Blocked,
            /// The surface does not consume this component group.
            NotApplicable,
        }

        impl $name {
            /// Returns true when the axis never hides drift (is not blocked).
            pub const fn never_violates(self) -> bool {
                !matches!(self, Self::Blocked)
            }

            /// Returns true when the axis carries a disclosed reduction.
            pub const fn is_disclosed_reduction(self) -> bool {
                matches!(self, Self::DisclosedNarrowed)
            }

            /// Returns true when the surface does not consume this component group.
            pub const fn is_not_applicable(self) -> bool {
                matches!(self, Self::NotApplicable)
            }

            /// Stable token recorded in the row.
            pub const fn as_str(self) -> &'static str {
                match self {
                    Self::Certified => "certified",
                    Self::DisclosedNarrowed => "disclosed_narrowed",
                    Self::Blocked => "blocked",
                    Self::NotApplicable => "not_applicable",
                }
            }
        }
    };
}

gated_truth_axis!(
    LaunchWedgeTruthState,
    "Certification of start-center bundle card and certified-archetype badge truth on a surface."
);
gated_truth_axis!(
    DetailReviewTruthState,
    "Certification of bundle detail-page and install/update review-sheet diff-scope truth on a surface."
);
gated_truth_axis!(
    DriftOverrideTruthState,
    "Certification of drift-banner and local-override-row truth on a surface."
);
gated_truth_axis!(
    RollbackRemoveTruthState,
    "Certification of rollback/remove-card rollback-path and side-effect truth on a surface."
);
gated_truth_axis!(
    ClassDisclosureTruthState,
    "Certification of class-disclosure card and claim-narrowing row truth on a surface."
);

/// The always-applicable export-parity axis: the support / release export must
/// reconstruct the surface's certified truth without a screenshot.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ClaimExportParityState {
    /// The export reconstructs the full certified truth.
    Certified,
    /// The export reconstructs a disclosed-partial projection (yellow).
    DisclosedPartial,
    /// The export drops truth or relies on a screenshot (red).
    Dropped,
}

impl ClaimExportParityState {
    /// Returns true when the export never drops truth.
    pub const fn never_violates(self) -> bool {
        !matches!(self, Self::Dropped)
    }

    /// Returns true when the export carries a disclosed reduction.
    pub const fn is_disclosed_reduction(self) -> bool {
        matches!(self, Self::DisclosedPartial)
    }

    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Certified => "certified",
            Self::DisclosedPartial => "disclosed_partial",
            Self::Dropped => "dropped",
        }
    }
}

/// The reduction level of one component group's axis on a surface.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum AxisLevel {
    NotApplicable,
    Certified,
    Disclosed,
    Blocked,
}

/// One distribution-path compatibility note for a certified surface.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BundleDistributionCompatibility {
    /// The distribution path this note describes.
    pub path: M5BundleDistributionPath,
    /// The path's parity on this surface.
    pub parity: M5BundleDistributionParityState,
    /// A precise, non-generic note; required when the path is not current.
    #[serde(default)]
    pub note: String,
}

impl BundleDistributionCompatibility {
    /// Whether the note is well-formed: a current path may carry no note, but a
    /// degraded / unsupported path must carry a precise, non-generic explanation.
    pub fn is_well_formed(&self) -> bool {
        self.parity.is_current() || (!self.note.trim().is_empty() && !label_is_generic(&self.note))
    }
}

/// An honest bundle-support-claim auto-narrow block for a surface. When a consumed
/// component group's truth axis reduces, the surface's support claim lowers to the
/// permitted ceiling, names the binding group and frozen trigger, and preserves the
/// canonical component identity rather than inheriting a full-truth label.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SurfaceClaimAutoNarrow {
    /// The support claim the surface is narrowed to.
    pub narrowed_to: M5BundleSupportClaim,
    /// The component group whose reduced axis bound the narrowing.
    pub binding_group: M5WorkflowBundleComponentGroup,
    /// The frozen downgrade trigger (reused vocabulary) the narrowing names.
    pub trigger: M5BundleComponentDowngradeTrigger,
    /// A precise, non-generic label safe to render.
    pub narrowed_label: String,
    /// The canonical component identity and boundary are preserved rather than
    /// dropped; must hold.
    pub preserves_component_identity: bool,
}

impl SurfaceClaimAutoNarrow {
    /// Whether the auto-narrow block is honest: it preserves component identity and
    /// carries a precise, non-generic label.
    pub fn is_honest(&self) -> bool {
        self.preserves_component_identity && !label_is_generic(&self.narrowed_label)
    }
}

/// A named export field the certified support / release export preserves.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5BundleCertExportField {
    /// The claimed surface identity.
    SurfaceIdentity,
    /// The component groups the surface consumes.
    ConsumedGroups,
    /// The declared support claim.
    DeclaredClaim,
    /// The effective (post-narrowing) support claim.
    EffectiveClaim,
    /// The per-axis certification truth.
    PerAxisTruth,
    /// The distribution-path compatibility notes.
    CompatibilityNotes,
    /// The narrowed-claim reason, when narrowed.
    NarrowedReason,
    /// The canonical certification bundle ref.
    CertificationBundleRef,
}

impl M5BundleCertExportField {
    /// Every export field, in declaration order.
    pub const ALL: [Self; 8] = [
        Self::SurfaceIdentity,
        Self::ConsumedGroups,
        Self::DeclaredClaim,
        Self::EffectiveClaim,
        Self::PerAxisTruth,
        Self::CompatibilityNotes,
        Self::NarrowedReason,
        Self::CertificationBundleRef,
    ];

    /// The mandatory subset every certified surface's export must carry.
    pub const MANDATORY: [Self; 6] = [
        Self::SurfaceIdentity,
        Self::ConsumedGroups,
        Self::DeclaredClaim,
        Self::EffectiveClaim,
        Self::PerAxisTruth,
        Self::CertificationBundleRef,
    ];

    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SurfaceIdentity => "surface_identity",
            Self::ConsumedGroups => "consumed_groups",
            Self::DeclaredClaim => "declared_claim",
            Self::EffectiveClaim => "effective_claim",
            Self::PerAxisTruth => "per_axis_truth",
            Self::CompatibilityNotes => "compatibility_notes",
            Self::NarrowedReason => "narrowed_reason",
            Self::CertificationBundleRef => "certification_bundle_ref",
        }
    }
}

/// Copy / export parity for a certified surface: the same truth must be copyable as
/// text / JSON / Markdown, and a screenshot is never the only export.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CertCopyExportParity {
    /// The copy / export formats offered (must include text, json, markdown).
    #[serde(default)]
    pub formats: Vec<String>,
    /// A screenshot is never the only export; must always hold.
    pub screenshot_only_prohibited: bool,
}

impl CertCopyExportParity {
    /// Whether the copy / export parity is complete: text / JSON / Markdown are all
    /// offered and screenshots are prohibited as the sole export.
    pub fn is_complete(&self) -> bool {
        self.screenshot_only_prohibited
            && self.formats.iter().any(|f| f == "text")
            && self.formats.iter().any(|f| f == "json")
            && self.formats.iter().any(|f| f == "markdown")
    }
}

/// Derived certification status for a bundle surface row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5BundleSurfaceCertStatus {
    /// The surface passes the shared component packet with no narrowing (green).
    Certified,
    /// The surface auto-narrows its claim, honestly disclosed (yellow).
    NarrowedDisclosed,
    /// The surface hides drift, over-claims, or drops truth (red).
    Blocked,
}

impl M5BundleSurfaceCertStatus {
    /// Stable token recorded in the summary / CSV.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Certified => "certified",
            Self::NarrowedDisclosed => "narrowed_disclosed",
            Self::Blocked => "blocked",
        }
    }
}

/// A certification row for one claimed M5 stack-entry / migration surface.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BundleSurfaceCertRow {
    /// Record kind; must equal [`BUNDLE_SURFACE_CERT_ROW_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`BUNDLE_SURFACE_CERT_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable row id.
    pub row_id: String,
    /// The claimed surface this row certifies.
    pub claimed_surface: M5WorkflowBundleClaimedSurface,
    /// Ref to the frozen matrix this row certifies against.
    pub source_matrix_ref: String,
    /// Ref to the ONE canonical component bundle this surface cites.
    pub certification_bundle_ref: String,
    /// Opaque ref to the bundle context this surface acts on.
    pub bundle_context_ref: String,
    /// The component groups this surface consumes.
    #[serde(default)]
    pub consumed_groups: Vec<M5WorkflowBundleComponentGroup>,
    /// The support claim the surface declares.
    pub declared_claim: M5BundleSupportClaim,
    /// The support claim the surface effectively asserts after narrowing.
    pub effective_claim: M5BundleSupportClaim,
    /// Launch-wedge truth axis (`not_applicable` unless the launch-wedge group is
    /// consumed).
    pub launch_wedge_truth: LaunchWedgeTruthState,
    /// Detail / review truth axis.
    pub detail_review_truth: DetailReviewTruthState,
    /// Drift / override truth axis.
    pub drift_override_truth: DriftOverrideTruthState,
    /// Rollback / remove truth axis.
    pub rollback_remove_truth: RollbackRemoveTruthState,
    /// Class-disclosure truth axis.
    pub class_disclosure_truth: ClassDisclosureTruthState,
    /// The always-applicable export-parity axis.
    pub export_parity: ClaimExportParityState,
    /// Distribution-path compatibility notes.
    #[serde(default)]
    pub compatibility_notes: Vec<BundleDistributionCompatibility>,
    /// The honest auto-narrow block, present only when the surface narrows its claim.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub claim_auto_narrow: Option<SurfaceClaimAutoNarrow>,
    /// The copy / export parity of the certified surface.
    pub copy_export: CertCopyExportParity,
    /// The named export fields the certified export carries.
    #[serde(default)]
    pub export_fields: Vec<M5BundleCertExportField>,
    /// The required labels the certified surface preserves (reused vocabulary).
    #[serde(default)]
    pub required_labels: Vec<M5BundleRequiredLabel>,
    /// The canonical component families the surface references (reused vocabulary).
    #[serde(default)]
    pub consumer_families: Vec<M5WorkflowBundleComponentFamily>,
    /// Source contract refs backing this row.
    #[serde(default)]
    pub source_refs: Vec<String>,
    /// ISO 8601 UTC timestamp the certification was observed.
    pub observed_at: String,
    /// Evidence packet refs backing this row.
    #[serde(default)]
    pub evidence_refs: Vec<String>,
}

impl BundleSurfaceCertRow {
    /// Returns true when this surface is a release-evidence surface.
    pub const fn is_evidence_surface(&self) -> bool {
        self.claimed_surface.is_evidence()
    }

    /// Whether the surface declares that it consumes the component group.
    pub fn consumes_group(&self, group: M5WorkflowBundleComponentGroup) -> bool {
        self.consumed_groups.contains(&group)
    }

    /// The reduction level of one component group's truth axis.
    fn group_axis_level(&self, group: M5WorkflowBundleComponentGroup) -> AxisLevel {
        let (na, disclosed, blocked) = match group {
            M5WorkflowBundleComponentGroup::LaunchWedge => (
                self.launch_wedge_truth.is_not_applicable(),
                self.launch_wedge_truth.is_disclosed_reduction(),
                !self.launch_wedge_truth.never_violates(),
            ),
            M5WorkflowBundleComponentGroup::DetailReview => (
                self.detail_review_truth.is_not_applicable(),
                self.detail_review_truth.is_disclosed_reduction(),
                !self.detail_review_truth.never_violates(),
            ),
            M5WorkflowBundleComponentGroup::DriftOverride => (
                self.drift_override_truth.is_not_applicable(),
                self.drift_override_truth.is_disclosed_reduction(),
                !self.drift_override_truth.never_violates(),
            ),
            M5WorkflowBundleComponentGroup::RollbackRemove => (
                self.rollback_remove_truth.is_not_applicable(),
                self.rollback_remove_truth.is_disclosed_reduction(),
                !self.rollback_remove_truth.never_violates(),
            ),
            M5WorkflowBundleComponentGroup::ClassDisclosure => (
                self.class_disclosure_truth.is_not_applicable(),
                self.class_disclosure_truth.is_disclosed_reduction(),
                !self.class_disclosure_truth.never_violates(),
            ),
        };
        if na {
            AxisLevel::NotApplicable
        } else if blocked {
            AxisLevel::Blocked
        } else if disclosed {
            AxisLevel::Disclosed
        } else {
            AxisLevel::Certified
        }
    }

    /// AC3 invariant: each gated axis is `not_applicable` exactly when the surface
    /// does not consume its component group.
    pub fn axes_match_consumed_groups(&self) -> bool {
        M5WorkflowBundleComponentGroup::ALL.iter().all(|&group| {
            let na = self.group_axis_level(group) == AxisLevel::NotApplicable;
            na != self.consumes_group(group)
        })
    }

    /// Whether any consumed-group axis is blocked, or the export axis dropped truth.
    pub fn any_axis_blocked(&self) -> bool {
        !self.export_parity.never_violates()
            || M5WorkflowBundleComponentGroup::ALL
                .iter()
                .any(|&group| self.group_axis_level(group) == AxisLevel::Blocked)
    }

    /// Whether any consumed-group axis is disclosed, or the export axis is a disclosed
    /// partial.
    pub fn any_axis_disclosed(&self) -> bool {
        self.export_parity.is_disclosed_reduction()
            || M5WorkflowBundleComponentGroup::ALL
                .iter()
                .any(|&group| self.group_axis_level(group) == AxisLevel::Disclosed)
    }

    /// The status implied purely by the truth axes.
    fn axis_status(&self) -> M5BundleSurfaceCertStatus {
        if self.any_axis_blocked() {
            M5BundleSurfaceCertStatus::Blocked
        } else if self.any_axis_disclosed() {
            M5BundleSurfaceCertStatus::NarrowedDisclosed
        } else {
            M5BundleSurfaceCertStatus::Certified
        }
    }

    /// The consumed component group (in canonical order) whose axis is reduced or
    /// blocked, i.e. the group that binds the narrowing.
    pub fn binding_group(&self) -> Option<M5WorkflowBundleComponentGroup> {
        M5WorkflowBundleComponentGroup::ALL
            .iter()
            .copied()
            .find(|&group| {
                matches!(
                    self.group_axis_level(group),
                    AxisLevel::Disclosed | AxisLevel::Blocked
                )
            })
    }

    /// Whether the effective claim is narrowed below the declared claim.
    pub fn claim_narrowed(&self) -> bool {
        self.effective_claim.capability_rank() < self.declared_claim.capability_rank()
    }

    /// AC1: a stale or degraded surface can no longer inherit a full-truth label. The
    /// effective claim never exceeds the declared claim; a certified surface asserts
    /// its declared claim with no narrow block; a narrowed surface carries an honest
    /// narrow block bound to a reduced consumed group with its frozen trigger.
    pub fn claim_is_honest(&self) -> bool {
        if self.effective_claim.capability_rank() > self.declared_claim.capability_rank() {
            return false;
        }
        match self.axis_status() {
            // Blocked rows are rejected by `status`; do not additionally constrain.
            M5BundleSurfaceCertStatus::Blocked => true,
            M5BundleSurfaceCertStatus::NarrowedDisclosed => {
                self.claim_narrowed()
                    && match (&self.claim_auto_narrow, self.binding_group()) {
                        (Some(narrow), Some(group)) => {
                            narrow.is_honest()
                                && narrow.narrowed_to == self.effective_claim
                                && narrow.binding_group == group
                                && narrow.trigger == group.default_trigger()
                                && self.consumes_group(group)
                        }
                        _ => false,
                    }
            }
            M5BundleSurfaceCertStatus::Certified => {
                !self.claim_narrowed() && self.claim_auto_narrow.is_none()
            }
        }
    }

    /// AC2: a path whose parity is not current forces the claim to narrow rather than
    /// inheriting a full-truth label.
    pub fn unsupported_paths_narrowed(&self) -> bool {
        let any_not_current = self
            .compatibility_notes
            .iter()
            .any(|c| !c.parity.is_current());
        !any_not_current || self.claim_narrowed()
    }

    /// Whether every compatibility note is well-formed and at least one path is
    /// covered.
    pub fn compatibility_notes_valid(&self) -> bool {
        !self.compatibility_notes.is_empty()
            && self.compatibility_notes.iter().all(|c| c.is_well_formed())
    }

    /// The export preserves the surface's certified truth without a screenshot and
    /// carries every mandatory export field.
    pub fn export_preserves_truth(&self) -> bool {
        self.export_parity.never_violates()
            && self.copy_export.is_complete()
            && M5BundleCertExportField::MANDATORY
                .iter()
                .all(|f| self.export_fields.contains(f))
    }

    /// AC3: the surface references every canonical component family of the groups it
    /// consumes, so its claim is anchored to the shared component family.
    pub fn references_canonical_families(&self) -> bool {
        !self.consumed_groups.is_empty()
            && self.consumed_groups.iter().all(|group| {
                group
                    .families()
                    .iter()
                    .all(|family| self.consumer_families.contains(family))
            })
    }

    /// Derived certification status.
    pub fn status(&self) -> M5BundleSurfaceCertStatus {
        if !self.claim_is_honest()
            || !self.export_preserves_truth()
            || !self.unsupported_paths_narrowed()
            || !self.compatibility_notes_valid()
            || !self.references_canonical_families()
            || !self.axes_match_consumed_groups()
        {
            return M5BundleSurfaceCertStatus::Blocked;
        }
        self.axis_status()
    }

    /// Whether the row's identity and evidence fields are complete.
    pub fn is_complete(&self) -> bool {
        self.record_kind == BUNDLE_SURFACE_CERT_ROW_RECORD_KIND
            && self.schema_version == BUNDLE_SURFACE_CERT_SCHEMA_VERSION
            && !self.row_id.trim().is_empty()
            && self.source_matrix_ref == BUNDLE_SURFACE_CERT_COMPONENT_MATRIX_REF
            && !self.certification_bundle_ref.trim().is_empty()
            && !self.bundle_context_ref.trim().is_empty()
            && !self.consumed_groups.is_empty()
            && !self.export_fields.is_empty()
            && self.required_labels.len() >= M5BundleRequiredLabel::MANDATORY.len()
            && !self.observed_at.trim().is_empty()
            && !self.evidence_refs.is_empty()
            && self.evidence_refs.iter().all(|r| !r.trim().is_empty())
    }

    /// Deterministic governed chip line for this row.
    pub fn chip_tokens(&self) -> String {
        format!(
            "surface={surface} launch_wedge={launch} detail={detail} drift={drift} \
rollback={rollback} class={class} export={export} declared={declared} effective={effective} \
status={status}",
            surface = self.claimed_surface.as_str(),
            launch = self.launch_wedge_truth.as_str(),
            detail = self.detail_review_truth.as_str(),
            drift = self.drift_override_truth.as_str(),
            rollback = self.rollback_remove_truth.as_str(),
            class = self.class_disclosure_truth.as_str(),
            export = self.export_parity.as_str(),
            declared = self.declared_claim.as_str(),
            effective = self.effective_claim.as_str(),
            status = self.status().as_str(),
        )
    }
}

/// Rolled-up summary of an M05-851 workflow-bundle surface certification packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BundleSurfaceCertSummary {
    pub surface_count: usize,
    pub evidence_surface_count: usize,
    pub green_count: usize,
    pub yellow_count: usize,
    pub red_count: usize,
    pub consumed_group_count: usize,
    pub path_count: usize,
    pub all_claims_honest: bool,
    pub all_export_preserve_truth: bool,
    pub all_unsupported_paths_narrowed: bool,
    pub group_coverage_complete: bool,
    pub path_coverage_complete: bool,
}

/// Constructor input for [`BundleSurfaceCertPacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BundleSurfaceCertPacketInput {
    pub packet_id: String,
    pub as_of: String,
    pub matrix_ref: String,
    pub certification_bundle_ref: String,
    pub rows: Vec<BundleSurfaceCertRow>,
}

/// Checked-in M05-851 workflow-bundle surface certification packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BundleSurfaceCertPacket {
    pub schema_version: u32,
    pub record_kind: String,
    pub packet_id: String,
    pub as_of: String,
    pub matrix_ref: String,
    pub certification_bundle_ref: String,
    #[serde(default)]
    pub rows: Vec<BundleSurfaceCertRow>,
    pub summary: BundleSurfaceCertSummary,
}

impl BundleSurfaceCertPacket {
    /// Builds a packet, stamping the record kind, schema version, and computed
    /// summary.
    pub fn new(input: BundleSurfaceCertPacketInput) -> Self {
        let mut packet = Self {
            schema_version: BUNDLE_SURFACE_CERT_SCHEMA_VERSION,
            record_kind: BUNDLE_SURFACE_CERT_RECORD_KIND.to_owned(),
            packet_id: input.packet_id,
            as_of: input.as_of,
            matrix_ref: input.matrix_ref,
            certification_bundle_ref: input.certification_bundle_ref,
            rows: input.rows,
            summary: BundleSurfaceCertSummary {
                surface_count: 0,
                evidence_surface_count: 0,
                green_count: 0,
                yellow_count: 0,
                red_count: 0,
                consumed_group_count: 0,
                path_count: 0,
                all_claims_honest: false,
                all_export_preserve_truth: false,
                all_unsupported_paths_narrowed: false,
                group_coverage_complete: false,
                path_coverage_complete: false,
            },
        };
        packet.summary = packet.computed_summary();
        packet
    }

    /// Surfaces represented by some row in this packet.
    pub fn represented_surfaces(&self) -> BTreeSet<M5WorkflowBundleClaimedSurface> {
        self.rows.iter().map(|r| r.claimed_surface).collect()
    }

    /// Component groups consumed by some row.
    pub fn consumed_groups(&self) -> BTreeSet<M5WorkflowBundleComponentGroup> {
        self.rows
            .iter()
            .flat_map(|r| r.consumed_groups.iter().copied())
            .collect()
    }

    /// Distribution paths covered by some row's compatibility notes.
    pub fn covered_paths(&self) -> BTreeSet<M5BundleDistributionPath> {
        self.rows
            .iter()
            .flat_map(|r| r.compatibility_notes.iter().map(|c| c.path))
            .collect()
    }

    /// Computes summary fields from the packet contents.
    pub fn computed_summary(&self) -> BundleSurfaceCertSummary {
        let mut green = 0;
        let mut yellow = 0;
        let mut red = 0;
        for row in &self.rows {
            match row.status() {
                M5BundleSurfaceCertStatus::Certified => green += 1,
                M5BundleSurfaceCertStatus::NarrowedDisclosed => yellow += 1,
                M5BundleSurfaceCertStatus::Blocked => red += 1,
            }
        }
        let consumed = self.consumed_groups();
        let paths = self.covered_paths();

        BundleSurfaceCertSummary {
            surface_count: self.rows.len(),
            evidence_surface_count: self.rows.iter().filter(|r| r.is_evidence_surface()).count(),
            green_count: green,
            yellow_count: yellow,
            red_count: red,
            consumed_group_count: consumed.len(),
            path_count: paths.len(),
            all_claims_honest: self.rows.iter().all(BundleSurfaceCertRow::claim_is_honest),
            all_export_preserve_truth: self
                .rows
                .iter()
                .all(BundleSurfaceCertRow::export_preserves_truth),
            all_unsupported_paths_narrowed: self
                .rows
                .iter()
                .all(BundleSurfaceCertRow::unsupported_paths_narrowed),
            group_coverage_complete: M5WorkflowBundleComponentGroup::ALL
                .iter()
                .all(|g| consumed.contains(g)),
            path_coverage_complete: M5BundleDistributionPath::ALL
                .iter()
                .all(|p| paths.contains(p)),
        }
    }

    /// Validates the packet and returns every contract violation.
    pub fn validate(&self) -> Vec<BundleSurfaceCertViolation> {
        let mut violations = Vec::new();

        if self.schema_version != BUNDLE_SURFACE_CERT_SCHEMA_VERSION {
            violations.push(BundleSurfaceCertViolation::SchemaVersion {
                expected: BUNDLE_SURFACE_CERT_SCHEMA_VERSION,
                actual: self.schema_version,
            });
        }
        if self.record_kind != BUNDLE_SURFACE_CERT_RECORD_KIND {
            violations.push(BundleSurfaceCertViolation::RecordKind {
                expected: BUNDLE_SURFACE_CERT_RECORD_KIND.to_owned(),
                actual: self.record_kind.clone(),
            });
        }
        if self.packet_id.trim().is_empty()
            || self.as_of.trim().is_empty()
            || self.matrix_ref.trim().is_empty()
            || self.certification_bundle_ref.trim().is_empty()
        {
            violations.push(BundleSurfaceCertViolation::MissingIdentity);
        }

        let mut row_ids = BTreeSet::new();
        let mut seen_surfaces = BTreeSet::new();
        for row in &self.rows {
            if !row_ids.insert(row.row_id.clone()) {
                violations.push(BundleSurfaceCertViolation::DuplicateId {
                    id: row.row_id.clone(),
                });
            }
            seen_surfaces.insert(row.claimed_surface);

            if !row.is_complete() {
                violations.push(BundleSurfaceCertViolation::IncompleteRow {
                    id: row.row_id.clone(),
                });
            }

            // Every surface cites the ONE canonical component bundle.
            if row.certification_bundle_ref != self.certification_bundle_ref {
                violations.push(BundleSurfaceCertViolation::BundleRefMismatch {
                    id: row.row_id.clone(),
                });
            }

            // AC3: each gated axis is applicable exactly when its group is consumed.
            if !row.axes_match_consumed_groups() {
                violations.push(BundleSurfaceCertViolation::AxisApplicabilityMismatch {
                    id: row.row_id.clone(),
                });
            }

            // AC1: the claim never over-asserts support for a reduced surface.
            if !row.claim_is_honest() {
                violations.push(BundleSurfaceCertViolation::ClaimOverAsserted {
                    id: row.row_id.clone(),
                });
            }

            // AC2: unsupported / degraded paths force a narrowed claim.
            if !row.unsupported_paths_narrowed() {
                violations.push(BundleSurfaceCertViolation::UnsupportedPathNotNarrowed {
                    id: row.row_id.clone(),
                });
            }
            if !row.compatibility_notes_valid() {
                violations.push(BundleSurfaceCertViolation::CompatibilityNoteMalformed {
                    id: row.row_id.clone(),
                });
            }

            // AC2: export preserves truth without a screenshot.
            if !row.export_preserves_truth() {
                violations.push(BundleSurfaceCertViolation::ExportDropsTruth {
                    id: row.row_id.clone(),
                });
            }

            // AC3: anchored to the canonical component family.
            if !row.references_canonical_families() {
                violations.push(BundleSurfaceCertViolation::NotAnchoredToCanonicalFamily {
                    id: row.row_id.clone(),
                });
            }

            // No blocked (red) surface may ship.
            if row.status() == M5BundleSurfaceCertStatus::Blocked {
                violations.push(BundleSurfaceCertViolation::BlockedSurface {
                    id: row.row_id.clone(),
                });
            }
        }

        // Coverage: every claimed surface is certified at least once.
        for surface in M5WorkflowBundleClaimedSurface::ALL {
            if !seen_surfaces.contains(&surface) {
                violations.push(BundleSurfaceCertViolation::MissingSurfaceCoverage { surface });
            }
        }

        // Coverage: every release-evidence surface is present.
        for surface in M5WorkflowBundleClaimedSurface::EVIDENCE_SURFACES {
            if !seen_surfaces.contains(&surface) {
                violations.push(BundleSurfaceCertViolation::MissingEvidenceSurface { surface });
            }
        }

        // Coverage: every component group is consumed somewhere.
        let consumed = self.consumed_groups();
        for group in M5WorkflowBundleComponentGroup::ALL {
            if !consumed.contains(&group) {
                violations.push(BundleSurfaceCertViolation::MissingGroupCoverage { group });
            }
        }

        // Coverage: every distribution path is exercised somewhere.
        let paths = self.covered_paths();
        for path in M5BundleDistributionPath::ALL {
            if !paths.contains(&path) {
                violations.push(BundleSurfaceCertViolation::MissingPathCoverage { path });
            }
        }

        if self.summary != self.computed_summary() {
            violations.push(BundleSurfaceCertViolation::SummaryMismatch);
        }

        if json_contains_forbidden_material(
            &serde_json::to_value(self)
                .expect("workflow-bundle surface certification packet serializes"),
        ) {
            violations.push(BundleSurfaceCertViolation::RawBoundaryMaterialInExport);
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
            .expect("workflow-bundle surface certification packet serializes")
    }

    /// Deterministic CSV of the certified rows for release / support handoff.
    pub fn render_matrix_csv(&self) -> String {
        let mut out = String::from(
            "row_id,claimed_surface,launch_wedge,detail_review,drift_override,rollback_remove,class_disclosure,export_parity,declared_claim,effective_claim,status\n",
        );
        for row in &self.rows {
            out.push_str(&format!(
                "{id},{surface},{launch},{detail},{drift},{rollback},{class},{export},{declared},{effective},{status}\n",
                id = row.row_id,
                surface = row.claimed_surface.as_str(),
                launch = row.launch_wedge_truth.as_str(),
                detail = row.detail_review_truth.as_str(),
                drift = row.drift_override_truth.as_str(),
                rollback = row.rollback_remove_truth.as_str(),
                class = row.class_disclosure_truth.as_str(),
                export = row.export_parity.as_str(),
                declared = row.declared_claim.as_str(),
                effective = row.effective_claim.as_str(),
                status = row.status().as_str(),
            ));
        }
        out
    }

    /// Deterministic Markdown report for support, docs, or release handoff.
    pub fn render_markdown_summary(&self) -> String {
        let mut out = String::new();
        out.push_str("# M5 Workflow-Bundle Component Surface Certification\n\n");
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- As of: `{}`\n", self.as_of));
        out.push_str(&format!("- Bundle: `{}`\n", self.certification_bundle_ref));
        out.push_str(&format!(
            "- Surfaces: {} certified across {} / {} claimed surfaces\n",
            self.summary.surface_count,
            self.represented_surfaces().len(),
            M5WorkflowBundleClaimedSurface::ALL.len(),
        ));
        out.push_str(&format!(
            "- Status: {} green / {} yellow / {} red\n",
            self.summary.green_count, self.summary.yellow_count, self.summary.red_count,
        ));
        out.push_str("\n## Surfaces\n\n");
        for row in &self.rows {
            out.push_str(&format!(
                "- **{}** ({}) — {}\n",
                row.row_id,
                row.claimed_surface.label(),
                row.chip_tokens(),
            ));
            if let Some(narrow) = &row.claim_auto_narrow {
                out.push_str(&format!(
                    "  - Auto-narrow: {} → {} (group={}, trigger={}) — {}\n",
                    row.declared_claim.as_str(),
                    narrow.narrowed_to.as_str(),
                    narrow.binding_group.as_str(),
                    narrow.trigger.as_str(),
                    narrow.narrowed_label,
                ));
            }
        }
        out
    }
}

/// Reads and validates the checked-in workflow-bundle surface certification export.
pub fn current_m5_bundle_surface_cert_export(
) -> Result<BundleSurfaceCertPacket, BundleSurfaceCertArtifactError> {
    let packet: BundleSurfaceCertPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/release/m5-workflow-bundle-surface-certification-proof/support_export.json"
    )))
    .map_err(BundleSurfaceCertArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(BundleSurfaceCertArtifactError::Validation(violations))
    }
}

/// Errors emitted when reading the checked-in workflow-bundle surface certification
/// export.
#[derive(Debug)]
pub enum BundleSurfaceCertArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<BundleSurfaceCertViolation>),
}

impl fmt::Display for BundleSurfaceCertArtifactError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(
                    f,
                    "workflow-bundle surface certification export parse failed: {error}"
                )
            }
            Self::Validation(violations) => {
                write!(
                    f,
                    "workflow-bundle surface certification export failed validation: {} violation(s)",
                    violations.len()
                )
            }
        }
    }
}

impl Error for BundleSurfaceCertArtifactError {}

/// Validation failure for M05-851 workflow-bundle surface certification packets.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BundleSurfaceCertViolation {
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
    BundleRefMismatch {
        id: String,
    },
    AxisApplicabilityMismatch {
        id: String,
    },
    ClaimOverAsserted {
        id: String,
    },
    UnsupportedPathNotNarrowed {
        id: String,
    },
    CompatibilityNoteMalformed {
        id: String,
    },
    ExportDropsTruth {
        id: String,
    },
    NotAnchoredToCanonicalFamily {
        id: String,
    },
    BlockedSurface {
        id: String,
    },
    MissingSurfaceCoverage {
        surface: M5WorkflowBundleClaimedSurface,
    },
    MissingEvidenceSurface {
        surface: M5WorkflowBundleClaimedSurface,
    },
    MissingGroupCoverage {
        group: M5WorkflowBundleComponentGroup,
    },
    MissingPathCoverage {
        path: M5BundleDistributionPath,
    },
    SummaryMismatch,
    RawBoundaryMaterialInExport,
}

impl fmt::Display for BundleSurfaceCertViolation {
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
            Self::IncompleteRow { id } => write!(f, "incomplete certification row: {id}"),
            Self::BundleRefMismatch { id } => {
                write!(
                    f,
                    "row {id} does not cite the packet's canonical bundle ref"
                )
            }
            Self::AxisApplicabilityMismatch { id } => {
                write!(
                    f,
                    "row {id} has a truth axis that is applicable without its component group (or vice versa)"
                )
            }
            Self::ClaimOverAsserted { id } => {
                write!(
                    f,
                    "row {id} over-asserts bundle support for a reduced surface, or narrows spuriously"
                )
            }
            Self::UnsupportedPathNotNarrowed { id } => {
                write!(
                    f,
                    "row {id} has a non-current distribution path but does not narrow its claim"
                )
            }
            Self::CompatibilityNoteMalformed { id } => {
                write!(
                    f,
                    "row {id} has a missing or generic distribution-path compatibility note"
                )
            }
            Self::ExportDropsTruth { id } => {
                write!(
                    f,
                    "row {id} export cannot preserve certified truth without a screenshot"
                )
            }
            Self::NotAnchoredToCanonicalFamily { id } => {
                write!(
                    f,
                    "row {id} does not reference the canonical families of its consumed groups"
                )
            }
            Self::BlockedSurface { id } => {
                write!(f, "row {id} is blocked (red) and may not ship")
            }
            Self::MissingSurfaceCoverage { surface } => {
                write!(
                    f,
                    "claimed surface {surface:?} is not certified in the packet"
                )
            }
            Self::MissingEvidenceSurface { surface } => {
                write!(f, "release-evidence surface {surface:?} is missing")
            }
            Self::MissingGroupCoverage { group } => {
                write!(
                    f,
                    "component group {} is not consumed in the packet",
                    group.as_str()
                )
            }
            Self::MissingPathCoverage { path } => {
                write!(
                    f,
                    "distribution path {} is not exercised in the packet",
                    path.as_str()
                )
            }
            Self::SummaryMismatch => write!(f, "computed summary does not match stored summary"),
            Self::RawBoundaryMaterialInExport => {
                write!(f, "export contains raw boundary material")
            }
        }
    }
}

impl Error for BundleSurfaceCertViolation {}

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

/// Builds the canonical, checked-in workflow-bundle surface certification packet. This
/// is the one source of truth shared by the tests, the fixtures, and the on-disk
/// support export so all stay byte-aligned.
pub fn seeded_m5_bundle_surface_cert_packet() -> BundleSurfaceCertPacket {
    BundleSurfaceCertPacket::new(BundleSurfaceCertPacketInput {
        packet_id: "m5-workflow-bundle-surface-certification:stable:0001".to_owned(),
        as_of: "2026-07-06T00:00:00Z".to_owned(),
        matrix_ref: BUNDLE_SURFACE_CERT_COMPONENT_MATRIX_REF.to_owned(),
        certification_bundle_ref: BUNDLE_SURFACE_CERT_BUNDLE_REF.to_owned(),
        rows: seeded_rows(),
    })
}

include!("seed.rs");
