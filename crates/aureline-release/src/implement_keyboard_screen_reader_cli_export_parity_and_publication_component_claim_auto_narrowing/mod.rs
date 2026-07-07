//! Keyboard / screen-reader / CLI / export parity and honest auto-narrowing for
//! the M5 release-center publication components.
//!
//! This module is the M05-866 accessibility-and-auto-narrowing capstone over the
//! frozen M5 release-center component matrix
//! ([`crate::freeze_the_m5_release_candidate_card_version_bump_row_publish_target_row_artifact_provenance_bundle_card_and_promotion_timeline_component_matrix`]).
//! Where the freeze matrix defines the reusable release-candidate card, version-bump
//! row, publish-target row / review sheet, artifact-provenance-bundle card,
//! promotion-timeline step, and rollback / revocation row primitives, and the 861-864
//! implementation lanes resolve their per-surface truth, this lane certifies — per
//! component family — that publication claims stay **keyboard-complete,
//! assistive-tech-reachable, CLI/export-safe, and self-narrowing** rather than
//! presenting stale evidence, a partial signature/attestation, a masked target-auth
//! posture, or an unverified mirror as still `Certified` or `Supported`:
//!
//! - **Keyboard / screen-reader / CLI reach.** Every family exposes a
//!   keyboard-complete, screen-reader-reachable, and CLI/headless-reachable path into
//!   the same candidate scope / blocker freshness, public-surface impact, target
//!   visibility / mutability / auth source, signature / attestation / SBOM / digest
//!   lineage, rollout ring, and rollback blast radius the rich surface shows — never a
//!   view-only card that strands assistive-tech or headless users. Hierarchy-heavy
//!   families (the artifact-provenance-bundle card's digest-lineage tree with its
//!   attestation / SBOM sub-rows) additionally bind their tree to a flat list /
//!   textual path.
//! - **Export parity.** The support / release / evaluation export reconstructs each
//!   component's meaning from typed tokens and opaque refs without a screenshot,
//!   preserving the same auth sources, provenance states, rollout rings, and rollback
//!   scopes shown in-product.
//! - **Honest auto-narrowing.** When evidence freshness, signature / attestation
//!   state, target auth posture, or mirror verification becomes stale, partial, or
//!   policy-blocked, the component's publication-support claim auto-narrows from
//!   `Certified` / `Supported` to degraded / provisional / unverified / policy-blocked,
//!   discloses the narrowing with a precise trigger and binding dimension, and
//!   preserves the canonical candidate / target / provenance / timeline identity rather
//!   than silently dropping it. A component with every dimension intact must NOT carry
//!   a spurious narrowing.
//! - **Cross-surface disclosure.** The same narrowed state surfaces in the release
//!   center, update center, docs / help, evaluation packs, headless CLI, and
//!   support / admin exports so claim publication and field triage stay aligned on
//!   publication-component downgrade behavior — a public-facing claim can never outrun
//!   the proof it is being viewed away from.
//!
//! Each [`PublicationAccessibilityRow`] keys on one
//! [`crate::freeze_the_m5_release_candidate_card_version_bump_row_publish_target_row_artifact_provenance_bundle_card_and_promotion_timeline_component_matrix::M5ReleaseCenterComponentFamily`]
//! and reuses that frozen family vocabulary plus the frozen
//! [`M5ReleaseCenterRequiredLabel`] and [`M5ReleaseCenterDowngradeTrigger`] and the
//! shared [`M5ReleaseCenterConsumerSurface`] consumer surfaces rather than minting
//! parallel synonyms, so the certified labels stay byte-identical to the matrix and the
//! sibling primitive packets.
//!
//! The packet is metadata-only: raw artifacts, signing keys, publish credentials, and
//! mirror cursors never cross this boundary; the packet carries only typed class
//! tokens, opaque summary / evidence refs, booleans, and redacted labels so support and
//! diagnostics exports can reconstruct exactly what an accessible fallback would have
//! shown without leaking release material.
//!
//! The boundary schema is
//! [`schemas/ui/m5-publication-component-accessibility-fallback.schema.json`](../../../../schemas/ui/m5-publication-component-accessibility-fallback.schema.json).
//! The contract doc is
//! [`docs/release/m5_publication_component_accessibility_fallback.md`](../../../../docs/release/m5_publication_component_accessibility_fallback.md).

#[cfg(test)]
mod tests;

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

// Reused frozen component vocabulary — the capstone certifies the freeze matrix's
// families, required labels, downgrade triggers, and consumer surfaces rather than
// mint parallel ones.
use crate::freeze_the_m5_release_candidate_card_version_bump_row_publish_target_row_artifact_provenance_bundle_card_and_promotion_timeline_component_matrix::{
    M5ReleaseCenterComponentFamily, M5ReleaseCenterConsumerSurface,
    M5ReleaseCenterDowngradeTrigger, M5ReleaseCenterRequiredLabel,
};

/// Schema version stamped on the M05-866 publication-component accessibility fallback
/// packet.
pub const PUBLICATION_A11Y_FALLBACK_SCHEMA_VERSION: u32 = 1;

/// Stable record-kind tag carried by [`PublicationAccessibilityPacket`].
pub const PUBLICATION_A11Y_FALLBACK_RECORD_KIND: &str =
    "m5_publication_component_accessibility_fallback_packet";

/// Stable record-kind tag carried by each [`PublicationAccessibilityRow`].
pub const PUBLICATION_A11Y_FALLBACK_ROW_RECORD_KIND: &str =
    "m5_publication_component_accessibility_fallback_row";

/// Repo-relative path of the boundary schema.
pub const PUBLICATION_A11Y_FALLBACK_SCHEMA_REF: &str =
    "schemas/ui/m5-publication-component-accessibility-fallback.schema.json";

/// Repo-relative path of the contract doc.
pub const PUBLICATION_A11Y_FALLBACK_DOC_REF: &str =
    "docs/release/m5_publication_component_accessibility_fallback.md";

/// Repo-relative path of the frozen release-center component matrix this lane
/// certifies.
pub const PUBLICATION_A11Y_FALLBACK_COMPONENT_MATRIX_REF: &str =
    "schemas/ui/m5-release-center-components.schema.json";

/// Repo-relative path of the protected fixture directory.
pub const PUBLICATION_A11Y_FALLBACK_FIXTURE_DIR: &str =
    "fixtures/ui/m5-publication-component-accessibility-fallback";

/// Repo-relative path of the checked support-export artifact (the `include_str!`
/// canonical).
pub const PUBLICATION_A11Y_FALLBACK_ARTIFACT_REF: &str =
    "artifacts/release/m5-publication-component-accessibility-fallback-proof/support_export.json";

/// Repo-relative path of the checked machine-readable matrix CSV.
pub const PUBLICATION_A11Y_FALLBACK_CSV_REF: &str =
    "artifacts/release/m5-publication-component-accessibility-fallback-proof/matrix.csv";

/// Repo-relative path of the checked Markdown report.
pub const PUBLICATION_A11Y_FALLBACK_REPORT_REF: &str =
    "artifacts/release/m5-publication-component-accessibility-fallback-proof/report.md";

/// The reusable component families that render a non-linear hierarchy (the
/// artifact-provenance-bundle card's digest-lineage tree with its attestation / SBOM
/// sub-rows) and therefore MUST bind their tree to an equivalent flat list / textual
/// path so the hierarchy is navigable non-visually.
const fn family_is_hierarchy_heavy(family: M5ReleaseCenterComponentFamily) -> bool {
    matches!(
        family,
        M5ReleaseCenterComponentFamily::ArtifactProvenanceBundleCard
    )
}

/// The publication dimension whose weakening a family primarily discloses. Every row
/// must model at least this dimension so its key weakening axis is covered.
const fn family_primary_dimension(
    family: M5ReleaseCenterComponentFamily,
) -> M5PublicationClaimDimension {
    match family {
        M5ReleaseCenterComponentFamily::ReleaseCandidateCard => {
            M5PublicationClaimDimension::EvidenceFreshness
        }
        M5ReleaseCenterComponentFamily::VersionBumpRow => {
            M5PublicationClaimDimension::PublicSurfaceImpact
        }
        M5ReleaseCenterComponentFamily::PublishTargetRow => {
            M5PublicationClaimDimension::TargetAuthPosture
        }
        M5ReleaseCenterComponentFamily::ArtifactProvenanceBundleCard => {
            M5PublicationClaimDimension::SignatureAttestationState
        }
        M5ReleaseCenterComponentFamily::PromotionTimelineStep => {
            M5PublicationClaimDimension::MirrorVerification
        }
        M5ReleaseCenterComponentFamily::RollbackRevocationRow => {
            M5PublicationClaimDimension::RollbackBlastRadius
        }
    }
}

/// A rendered fallback modality for a publication component.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5PublicationFallbackModality {
    /// A rich, structured (digest-lineage tree / grouped bundle) projection.
    Structured,
    /// A flat list projection.
    List,
    /// A textual / source-first projection.
    Textual,
    /// A CLI / headless line projection.
    Cli,
}

impl M5PublicationFallbackModality {
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
pub enum M5PublicationRenderingSurface {
    /// The full-capability desktop release-center surface.
    DesktopFull,
    /// The companion app.
    CompanionApp,
    /// A read-only browser projection.
    BrowserReadonly,
    /// A headless CLI surface.
    CliHeadless,
    /// A handoff packet.
    HandoffPacket,
    /// A support / admin / evaluation export.
    SupportExport,
}

impl M5PublicationRenderingSurface {
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
pub enum PublicationNonVisualReachState {
    /// Fully traversable and labeled with no loss.
    ReachableAndLabeled,
    /// Reachable and labeled, but with a disclosed reduction (yellow).
    DisclosedReducedButReachable,
    /// A view-only surface that traps keyboard / assistive-tech / headless users
    /// (red).
    ViewOnlyTrap,
}

impl PublicationNonVisualReachState {
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
pub enum PublicationExportSummaryState {
    /// The component meaning reconstructs from the summary without a screenshot.
    ReconstructableWithoutScreenshot,
    /// Partial capture, but disclosed (yellow).
    DisclosedPartialCapture,
    /// The export relies on a screenshot to carry meaning (red).
    AbsentNeedsScreenshot,
}

impl PublicationExportSummaryState {
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
pub enum PublicationNarrowingDisclosureState {
    /// Full label and summary parity with the desktop surface.
    ParityPreserved,
    /// Reduced interactivity, disclosed with preserved labels (yellow).
    DisclosedNarrowed,
    /// Interactivity, state, or actions dropped without disclosure (red).
    SilentlyDropped,
}

impl PublicationNarrowingDisclosureState {
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

/// The publication-support claim ceiling a component asserts: how strong a
/// publication posture it lets a surface present. Auto-narrowing lowers this ceiling
/// when a publication dimension weakens so a stale, partial, or policy-blocked
/// release can never keep an old `Certified` or `Supported` label.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5PublicationSupportClaim {
    /// Certified: a fully proven, current, signature-and-lineage-verified publication
    /// claim — the strongest claim.
    Certified,
    /// Supported: a resolved, self-sufficient publication object (a version-bump
    /// proposal or publish target) that is not itself a certified artifact claim.
    Supported,
    /// Degraded: usable, but with a disclosed reduction in scope or confidence.
    Degraded,
    /// Provisional: the underlying proof / mirror verification is stale and being
    /// re-established; state is last-known, not current.
    Provisional,
    /// Unverified: the signature / attestation / evidence could not be verified; the
    /// claim is reconstructed from unproven material.
    Unverified,
    /// Policy-blocked: a required entitlement / policy dependency is unmet.
    PolicyBlocked,
}

impl M5PublicationSupportClaim {
    /// Every claim tier, strongest first.
    pub const ALL: [Self; 6] = [
        Self::Certified,
        Self::Supported,
        Self::Degraded,
        Self::Provisional,
        Self::Unverified,
        Self::PolicyBlocked,
    ];

    /// Capability rank; a higher rank asserts a stronger publication posture.
    /// Narrowing lowers rank.
    pub const fn capability_rank(self) -> u8 {
        match self {
            Self::Certified => 5,
            Self::Supported => 4,
            Self::Degraded => 3,
            Self::Provisional => 2,
            Self::Unverified => 1,
            Self::PolicyBlocked => 0,
        }
    }

    /// Returns true when this claim asserts a fully proven, certified publication.
    pub const fn asserts_certified(self) -> bool {
        matches!(self, Self::Certified)
    }

    /// Returns true when this claim asserts a fully self-sufficient (certified or
    /// resolved / supported) posture.
    pub const fn asserts_full_self_sufficiency(self) -> bool {
        matches!(self, Self::Certified | Self::Supported)
    }

    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Certified => "certified",
            Self::Supported => "supported",
            Self::Degraded => "degraded",
            Self::Provisional => "provisional",
            Self::Unverified => "unverified",
            Self::PolicyBlocked => "policy_blocked",
        }
    }
}

/// The publication dimension whose state governs how far a component may claim to be
/// certified, supported, or authoritative. The first four are exactly the axes the
/// spec requires auto-narrowing on — evidence freshness, signature / attestation
/// state, target auth posture, and mirror verification; the last two cover the
/// version-bump and rollback families' primary weakening axes so every frozen family
/// carries an honest narrowing path.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5PublicationClaimDimension {
    /// Evidence freshness: is the candidate's blocker / qualification evidence current,
    /// or stale / pending re-verification?
    EvidenceFreshness,
    /// Public-surface impact: is the version bump's derived compatibility impact fully
    /// resolved, or partial / unstated?
    PublicSurfaceImpact,
    /// Target auth posture: is the publish target's auth source verified and scoped,
    /// or masked / ambient?
    TargetAuthPosture,
    /// Signature / attestation state: is the provenance bundle's signature and
    /// attestation verified, or unverified / partial?
    SignatureAttestationState,
    /// Mirror verification: has the promotion / mirror proof been verified and stayed
    /// fresh, or gone stale?
    MirrorVerification,
    /// Rollback blast radius: is the rollback / revocation's blast radius proven and
    /// executable, or policy-gated / understated?
    RollbackBlastRadius,
}

impl M5PublicationClaimDimension {
    /// Every dimension, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::EvidenceFreshness,
        Self::PublicSurfaceImpact,
        Self::TargetAuthPosture,
        Self::SignatureAttestationState,
        Self::MirrorVerification,
        Self::RollbackBlastRadius,
    ];

    /// The frozen downgrade trigger this dimension names when its weakness binds a
    /// narrowing. Each dimension maps to the on-topic frozen trigger the freeze matrix
    /// already governs, so the certified reason stays byte-identical to the matrix.
    pub const fn default_trigger(self) -> M5ReleaseCenterDowngradeTrigger {
        match self {
            Self::EvidenceFreshness => M5ReleaseCenterDowngradeTrigger::BlockerFreshnessHidden,
            Self::PublicSurfaceImpact => M5ReleaseCenterDowngradeTrigger::VersionBumpImpactUnstated,
            Self::TargetAuthPosture => M5ReleaseCenterDowngradeTrigger::TargetAuthSourceMasked,
            Self::SignatureAttestationState => {
                M5ReleaseCenterDowngradeTrigger::SignatureOrAttestationOverclaimed
            }
            Self::MirrorVerification => M5ReleaseCenterDowngradeTrigger::ProofStale,
            Self::RollbackBlastRadius => {
                M5ReleaseCenterDowngradeTrigger::RollbackBlastRadiusUnderstated
            }
        }
    }

    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::EvidenceFreshness => "evidence_freshness",
            Self::PublicSurfaceImpact => "public_surface_impact",
            Self::TargetAuthPosture => "target_auth_posture",
            Self::SignatureAttestationState => "signature_attestation_state",
            Self::MirrorVerification => "mirror_verification",
            Self::RollbackBlastRadius => "rollback_blast_radius",
        }
    }
}

/// The observed condition of one publication dimension. Anything weaker than
/// [`Self::Verified`] imposes a narrowing ceiling on the component's support claim.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5PublicationConditionState {
    /// Fully verified / current / proven — imposes no ceiling.
    Verified,
    /// Partially resolved — scope or confidence is reduced; support drops to degraded.
    Partial,
    /// Stale — the proof / mirror verification aged out and is re-establishing;
    /// support drops to provisional.
    Stale,
    /// Unverified — the signature / attestation / evidence could not be proven;
    /// support drops to unverified.
    Unverified,
    /// Policy-blocked — a required entitlement / policy dependency is unmet; support
    /// drops to policy-blocked.
    PolicyBlocked,
}

impl M5PublicationConditionState {
    /// Every condition state, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::Verified,
        Self::Partial,
        Self::Stale,
        Self::Unverified,
        Self::PolicyBlocked,
    ];

    /// Returns true when the dimension is weaker than verified and therefore imposes a
    /// narrowing ceiling.
    pub const fn is_weak(self) -> bool {
        !matches!(self, Self::Verified)
    }

    /// The strongest publication-support claim this condition state permits.
    pub const fn permitted_ceiling(self) -> M5PublicationSupportClaim {
        match self {
            Self::Verified => M5PublicationSupportClaim::Certified,
            Self::Partial => M5PublicationSupportClaim::Degraded,
            Self::Stale => M5PublicationSupportClaim::Provisional,
            Self::Unverified => M5PublicationSupportClaim::Unverified,
            Self::PolicyBlocked => M5PublicationSupportClaim::PolicyBlocked,
        }
    }

    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Verified => "verified",
            Self::Partial => "partial",
            Self::Stale => "stale",
            Self::Unverified => "unverified",
            Self::PolicyBlocked => "policy_blocked",
        }
    }
}

/// One publication dimension's observed condition on a component.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PublicationClaimConditionEntry {
    /// Which dimension this entry describes.
    pub dimension: M5PublicationClaimDimension,
    /// The observed condition state of the dimension.
    pub state: M5PublicationConditionState,
}

/// An honest publication-support-claim auto-narrow block. When a publication
/// dimension weakens, the component's support claim lowers to the permitted ceiling,
/// names the binding dimension and frozen trigger, and preserves the canonical
/// candidate / target / provenance / timeline identity rather than silently dropping
/// it.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PublicationClaimAutoNarrow {
    /// The support claim the component is narrowed to.
    pub narrowed_to: M5PublicationSupportClaim,
    /// The dimension whose weakness bound the narrowing (the one imposing the
    /// strongest ceiling constraint).
    pub binding_dimension: M5PublicationClaimDimension,
    /// The frozen downgrade trigger (reused vocabulary) the narrowing names.
    pub trigger: M5ReleaseCenterDowngradeTrigger,
    /// A precise, non-generic label safe to render.
    pub narrowed_label: String,
    /// The canonical candidate scope, target identity, provenance digest, and rollback
    /// scope are preserved rather than dropped; must hold.
    pub preserves_canonical_identity: bool,
}

impl PublicationClaimAutoNarrow {
    /// Whether the auto-narrow block is honest: it preserves canonical identity and
    /// carries a precise, non-generic label.
    pub fn is_honest(&self) -> bool {
        self.preserves_canonical_identity && !label_is_generic(&self.narrowed_label)
    }
}

/// Copy / export parity for a component's accessible fallback: the same truth must be
/// copyable as text / JSON / Markdown, and a screenshot is never the only export.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PublicationCopyExportParity {
    /// The copy / export formats offered (must include text, json, markdown).
    #[serde(default)]
    pub formats: Vec<String>,
    /// The named export fields the summary carries.
    #[serde(default)]
    pub export_fields: Vec<String>,
    /// A screenshot is never the only export; must always hold.
    pub screenshot_only_prohibited: bool,
}

impl PublicationCopyExportParity {
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
pub struct PublicationRenderingNarrowingDisclosure {
    /// The rendering surface being narrowed.
    pub rendering_surface: M5PublicationRenderingSurface,
    /// How the surface discloses its reduced interactivity.
    pub state: PublicationNarrowingDisclosureState,
    /// The labels preserved across the narrowing.
    #[serde(default)]
    pub preserved_labels: Vec<String>,
    /// The interactions reduced on the narrowed surface.
    #[serde(default)]
    pub reduced_interactions: Vec<String>,
}

/// Derived qualification status for a publication accessibility row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PublicationAccessibilityStatus {
    /// Full keyboard / screen-reader / CLI / export parity with no narrowing (green).
    Parity,
    /// Reduced but fully disclosed, reachable, and honestly auto-narrowed (yellow).
    NarrowedDisclosed,
    /// Strands assistive tech, needs a screenshot, over-claims support, or drops
    /// state silently (red).
    Stranded,
}

impl PublicationAccessibilityStatus {
    /// Stable token recorded in the summary / CSV.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Parity => "parity",
            Self::NarrowedDisclosed => "narrowed_disclosed",
            Self::Stranded => "stranded",
        }
    }
}

/// Accessibility / auto-narrowing parity row for one release-center publication
/// component family.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PublicationAccessibilityRow {
    /// Record kind; must equal [`PUBLICATION_A11Y_FALLBACK_ROW_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`PUBLICATION_A11Y_FALLBACK_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable row id.
    pub row_id: String,
    /// The frozen component family this row certifies.
    pub component_family: M5ReleaseCenterComponentFamily,
    /// Ref to the frozen matrix family schema this row certifies.
    pub source_family_schema_ref: String,
    /// Opaque ref to the release / artifact / target context this component acts on;
    /// stays visible on every surface, so this is never empty.
    pub publication_context_ref: String,
    /// Rendered modalities offered; a hierarchy-heavy family must also offer a
    /// non-visual (list / textual / cli) path.
    #[serde(default)]
    pub fallback_modalities: Vec<M5PublicationFallbackModality>,
    /// The non-visual / CLI path reaches the same canonical candidate scope, target
    /// auth, provenance, and rollback truth as the rich surface; must hold.
    pub reaches_canonical_truth: bool,
    /// Keyboard reach into the non-visual path.
    pub keyboard_reach: PublicationNonVisualReachState,
    /// Screen-reader reach into the non-visual path.
    pub screen_reader_reach: PublicationNonVisualReachState,
    /// CLI / headless reach into the non-visual path.
    pub cli_reach: PublicationNonVisualReachState,
    /// Whether the export-safe summary preserves component meaning.
    pub export_summary: PublicationExportSummaryState,
    /// Ref to the export-safe summary object for this component.
    pub export_summary_ref: String,
    /// The copy / export parity of the accessible fallback.
    pub copy_export: PublicationCopyExportParity,
    /// The full support claim this family asserts when every dimension is intact.
    pub full_support_claim: M5PublicationSupportClaim,
    /// The observed condition of each modeled publication dimension.
    #[serde(default)]
    pub claim_conditions: Vec<PublicationClaimConditionEntry>,
    /// The honest auto-narrow block, present only when some dimension weakens below
    /// the family's full claim.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub claim_narrow: Option<PublicationClaimAutoNarrow>,
    /// Rendering surfaces this component is certified on.
    #[serde(default)]
    pub rendering_surfaces: Vec<M5PublicationRenderingSurface>,
    /// Per-surface narrowing disclosures.
    #[serde(default)]
    pub narrowing_disclosures: Vec<PublicationRenderingNarrowingDisclosure>,
    /// The required labels the accessible fallback preserves (reused vocabulary).
    #[serde(default)]
    pub required_labels: Vec<M5ReleaseCenterRequiredLabel>,
    /// Semantic consumer surfaces this component is embedded in (reused vocabulary).
    #[serde(default)]
    pub consumer_surfaces: Vec<M5ReleaseCenterConsumerSurface>,
    /// Source contract refs backing this row.
    #[serde(default)]
    pub source_refs: Vec<String>,
    /// ISO 8601 UTC timestamp the accessibility posture was observed.
    pub observed_at: String,
    /// Evidence packet refs backing this row.
    #[serde(default)]
    pub evidence_refs: Vec<String>,
}

impl PublicationAccessibilityRow {
    /// Returns true when this family renders a non-linear hierarchy and must bind to a
    /// flat non-visual path.
    pub const fn is_hierarchy_heavy(&self) -> bool {
        family_is_hierarchy_heavy(self.component_family)
    }

    /// Returns true when at least one non-visual (list / textual / cli) fallback
    /// modality is offered.
    pub fn has_non_visual_fallback(&self) -> bool {
        self.fallback_modalities.iter().any(|m| m.is_non_visual())
    }

    /// The condition state observed for one dimension, or `Verified` when the row does
    /// not model that dimension.
    pub fn condition_for(
        &self,
        dimension: M5PublicationClaimDimension,
    ) -> M5PublicationConditionState {
        self.claim_conditions
            .iter()
            .find(|c| c.dimension == dimension)
            .map(|c| c.state)
            .unwrap_or(M5PublicationConditionState::Verified)
    }

    /// Whether any modeled dimension is weaker than verified.
    pub fn has_weak_dimension(&self) -> bool {
        self.claim_conditions.iter().any(|c| c.state.is_weak())
    }

    /// The strongest support claim permitted after applying every modeled dimension's
    /// ceiling, capped at the family's full claim.
    pub fn permitted_claim(&self) -> M5PublicationSupportClaim {
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
    pub fn binding_dimension(&self) -> Option<M5PublicationClaimDimension> {
        let mut binding: Option<(M5PublicationClaimDimension, u8)> = None;
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
    pub fn effective_claim(&self) -> M5PublicationSupportClaim {
        match &self.claim_narrow {
            Some(narrow) => narrow.narrowed_to,
            None => self.full_support_claim,
        }
    }

    /// AC / auto-narrowing honesty: a stale, partial, or policy-blocked publication
    /// can no longer keep an old `Certified` / `Supported` label. The effective claim
    /// never exceeds the permitted ceiling; when a dimension narrows below the full
    /// claim, an honest narrow block is present, narrows to exactly the permitted
    /// ceiling, binds to the ceiling-imposing dimension with its frozen trigger, and
    /// preserves canonical identity. When nothing narrows, no spurious narrow block is
    /// present.
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
    /// canonical truth — no keyboard / screen-reader / CLI trap, a hierarchy-heavy
    /// family offers a non-visual fallback, and the export reconstructs meaning without
    /// a screenshot.
    pub fn reaches_canonical_truth_via_at(&self) -> bool {
        self.reaches_canonical_truth
            && !self.publication_context_ref.trim().is_empty()
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
    /// reduced interactivity and keeps its labels, so claim publication and field
    /// triage stay aligned on the same narrowed state.
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
        M5ReleaseCenterRequiredLabel::MANDATORY
            .iter()
            .all(|label| self.required_labels.contains(label))
    }

    /// Derived qualification status.
    pub fn status(&self) -> PublicationAccessibilityStatus {
        if !self.claim_is_honest()
            || !self.reaches_canonical_truth_via_at()
            || !self.export_preserves_meaning()
            || !self.narrowing_disclosed()
            || !self.models_primary_dimension()
            || !self.preserves_mandatory_labels()
        {
            return PublicationAccessibilityStatus::Stranded;
        }
        if self.is_reduced() {
            PublicationAccessibilityStatus::NarrowedDisclosed
        } else {
            PublicationAccessibilityStatus::Parity
        }
    }

    /// Whether the row's identity and evidence fields are complete.
    pub fn is_complete(&self) -> bool {
        self.record_kind == PUBLICATION_A11Y_FALLBACK_ROW_RECORD_KIND
            && self.schema_version == PUBLICATION_A11Y_FALLBACK_SCHEMA_VERSION
            && !self.row_id.trim().is_empty()
            && !self.source_family_schema_ref.trim().is_empty()
            && !self.publication_context_ref.trim().is_empty()
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

/// Rolled-up summary of an M05-866 publication-component accessibility fallback
/// packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PublicationAccessibilitySummary {
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

/// Constructor input for [`PublicationAccessibilityPacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PublicationAccessibilityPacketInput {
    pub packet_id: String,
    pub as_of: String,
    pub matrix_ref: String,
    pub rows: Vec<PublicationAccessibilityRow>,
}

/// Checked-in M05-866 publication-component accessibility fallback packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PublicationAccessibilityPacket {
    pub schema_version: u32,
    pub record_kind: String,
    pub packet_id: String,
    pub as_of: String,
    pub matrix_ref: String,
    #[serde(default)]
    pub rows: Vec<PublicationAccessibilityRow>,
    pub summary: PublicationAccessibilitySummary,
}

impl PublicationAccessibilityPacket {
    /// Builds a packet, stamping the record kind, schema version, and computed
    /// summary.
    pub fn new(input: PublicationAccessibilityPacketInput) -> Self {
        let mut packet = Self {
            schema_version: PUBLICATION_A11Y_FALLBACK_SCHEMA_VERSION,
            record_kind: PUBLICATION_A11Y_FALLBACK_RECORD_KIND.to_owned(),
            packet_id: input.packet_id,
            as_of: input.as_of,
            matrix_ref: input.matrix_ref,
            rows: input.rows,
            summary: PublicationAccessibilitySummary {
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
    pub fn represented_families(&self) -> BTreeSet<M5ReleaseCenterComponentFamily> {
        self.rows.iter().map(|r| r.component_family).collect()
    }

    /// Dimensions exercised by some row's claim conditions.
    pub fn exercised_dimensions(&self) -> BTreeSet<M5PublicationClaimDimension> {
        self.rows
            .iter()
            .flat_map(|r| r.claim_conditions.iter().map(|c| c.dimension))
            .collect()
    }

    /// Support claim tiers that appear as an effective claim across the rows.
    pub fn represented_effective_claims(&self) -> BTreeSet<M5PublicationSupportClaim> {
        self.rows.iter().map(|r| r.effective_claim()).collect()
    }

    /// Consumer surfaces ingesting some row in this packet.
    pub fn represented_consumer_surfaces(&self) -> BTreeSet<M5ReleaseCenterConsumerSurface> {
        self.rows
            .iter()
            .flat_map(|r| r.consumer_surfaces.iter().copied())
            .collect()
    }

    /// Computes summary fields from the packet contents.
    pub fn computed_summary(&self) -> PublicationAccessibilitySummary {
        let mut rendering = BTreeSet::new();
        let mut consumers: BTreeSet<M5ReleaseCenterConsumerSurface> = BTreeSet::new();
        for row in &self.rows {
            rendering.extend(row.rendering_surfaces.iter().copied());
            consumers.extend(row.consumer_surfaces.iter().copied());
        }

        let hierarchy_heavy: Vec<&PublicationAccessibilityRow> = self
            .rows
            .iter()
            .filter(|row| row.is_hierarchy_heavy())
            .collect();

        let mut green = 0;
        let mut yellow = 0;
        let mut red = 0;
        for row in &self.rows {
            match row.status() {
                PublicationAccessibilityStatus::Parity => green += 1,
                PublicationAccessibilityStatus::NarrowedDisclosed => yellow += 1,
                PublicationAccessibilityStatus::Stranded => red += 1,
            }
        }

        PublicationAccessibilitySummary {
            family_count: self.rows.len(),
            hierarchy_heavy_family_count: hierarchy_heavy.len(),
            all_hierarchy_heavy_have_non_visual_fallback: hierarchy_heavy
                .iter()
                .all(|row| row.has_non_visual_fallback()),
            all_reach_canonical_truth_via_at: self
                .rows
                .iter()
                .all(PublicationAccessibilityRow::reaches_canonical_truth_via_at),
            all_claims_honest: self
                .rows
                .iter()
                .all(PublicationAccessibilityRow::claim_is_honest),
            all_export_summaries_preserve_meaning: self
                .rows
                .iter()
                .all(PublicationAccessibilityRow::export_preserves_meaning),
            all_narrowing_disclosed: self
                .rows
                .iter()
                .all(PublicationAccessibilityRow::narrowing_disclosed),
            green_count: green,
            yellow_count: yellow,
            red_count: red,
            rendering_surface_count: rendering.len(),
            consumer_surface_count: consumers.len(),
        }
    }

    /// Validates the packet and returns every contract violation.
    pub fn validate(&self) -> Vec<PublicationAccessibilityViolation> {
        let mut violations = Vec::new();

        if self.schema_version != PUBLICATION_A11Y_FALLBACK_SCHEMA_VERSION {
            violations.push(PublicationAccessibilityViolation::SchemaVersion {
                expected: PUBLICATION_A11Y_FALLBACK_SCHEMA_VERSION,
                actual: self.schema_version,
            });
        }
        if self.record_kind != PUBLICATION_A11Y_FALLBACK_RECORD_KIND {
            violations.push(PublicationAccessibilityViolation::RecordKind {
                expected: PUBLICATION_A11Y_FALLBACK_RECORD_KIND.to_owned(),
                actual: self.record_kind.clone(),
            });
        }
        if self.packet_id.trim().is_empty()
            || self.as_of.trim().is_empty()
            || self.matrix_ref.trim().is_empty()
        {
            violations.push(PublicationAccessibilityViolation::MissingIdentity);
        }

        let mut row_ids = BTreeSet::new();
        let mut seen_families = BTreeSet::new();
        for row in &self.rows {
            if !row_ids.insert(row.row_id.clone()) {
                violations.push(PublicationAccessibilityViolation::DuplicateId {
                    id: row.row_id.clone(),
                });
            }
            seen_families.insert(row.component_family);

            if !row.is_complete() {
                violations.push(PublicationAccessibilityViolation::IncompleteRow {
                    id: row.row_id.clone(),
                });
            }

            // Each row must model its family's primary weakening dimension.
            if !row.models_primary_dimension() {
                violations.push(PublicationAccessibilityViolation::MissingPrimaryDimension {
                    id: row.row_id.clone(),
                    dimension: family_primary_dimension(row.component_family),
                });
            }

            // Each row must preserve every mandatory release-center label.
            if !row.preserves_mandatory_labels() {
                violations.push(PublicationAccessibilityViolation::MissingMandatoryLabel {
                    id: row.row_id.clone(),
                });
            }

            // A hierarchy-heavy family must render a structured tree *and* a non-visual
            // path.
            if row.is_hierarchy_heavy()
                && !row
                    .fallback_modalities
                    .contains(&M5PublicationFallbackModality::Structured)
            {
                violations.push(
                    PublicationAccessibilityViolation::HierarchyHeavyMissingStructured {
                        id: row.row_id.clone(),
                    },
                );
            }

            // AC2: claim never over-asserts a certified / supported publication for a
            // weakened one.
            if !row.claim_is_honest() {
                violations.push(PublicationAccessibilityViolation::ClaimOverAsserted {
                    id: row.row_id.clone(),
                });
            }

            // Assistive-tech / CLI reach the same canonical truth.
            if !row.reaches_canonical_truth_via_at() {
                violations.push(PublicationAccessibilityViolation::AssistiveTechStranded {
                    id: row.row_id.clone(),
                });
            }

            // Export preserves meaning without a screenshot.
            if !row.export_preserves_meaning() {
                violations.push(
                    PublicationAccessibilityViolation::ExportRequiresScreenshot {
                        id: row.row_id.clone(),
                    },
                );
            }

            // Narrowing disclosed on every narrowed rendering surface.
            if !row.narrowing_disclosed() {
                violations.push(
                    PublicationAccessibilityViolation::NarrowingDropsContextSilently {
                        id: row.row_id.clone(),
                    },
                );
            }

            // Consumer parity: at least two consumer surfaces ingest the row.
            if row.consumer_surfaces.len() < 2 {
                violations.push(PublicationAccessibilityViolation::MissingConsumerParity {
                    id: row.row_id.clone(),
                });
            }

            // No red rows may ship.
            if row.status() == PublicationAccessibilityStatus::Stranded {
                violations.push(PublicationAccessibilityViolation::StrandedRow {
                    id: row.row_id.clone(),
                });
            }
        }

        // Coverage: every frozen family is certified at least once.
        for family in M5ReleaseCenterComponentFamily::ALL {
            if !seen_families.contains(&family) {
                violations
                    .push(PublicationAccessibilityViolation::MissingFamilyCoverage { family });
            }
        }

        // Coverage: every weakening dimension is exercised somewhere.
        let exercised = self.exercised_dimensions();
        for dimension in M5PublicationClaimDimension::ALL {
            if !exercised.contains(&dimension) {
                violations.push(
                    PublicationAccessibilityViolation::MissingDimensionCoverage { dimension },
                );
            }
        }

        // Coverage: every support claim tier appears as an effective claim, so the full
        // narrowing spectrum (certified → … → policy-blocked) is proven end-to-end.
        let effective = self.represented_effective_claims();
        for claim in M5PublicationSupportClaim::ALL {
            if !effective.contains(&claim) {
                violations
                    .push(PublicationAccessibilityViolation::MissingClaimTierCoverage { claim });
            }
        }

        // Cross-surface: the same narrowed state must reach the release center, update
        // center, docs / help, evaluation packs, CLI, and support / admin exports — so
        // every consumer surface is exercised at least once across the packet.
        let consumers = self.represented_consumer_surfaces();
        for surface in M5ReleaseCenterConsumerSurface::ALL {
            if !consumers.contains(&surface) {
                violations.push(
                    PublicationAccessibilityViolation::MissingConsumerSurfaceCoverage { surface },
                );
            }
        }

        if self.summary != self.computed_summary() {
            violations.push(PublicationAccessibilityViolation::SummaryMismatch);
        }

        if json_contains_forbidden_material(
            &serde_json::to_value(self)
                .expect("publication accessibility fallback packet serializes"),
        ) {
            violations.push(PublicationAccessibilityViolation::RawReleaseMaterialInExport);
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
            .expect("publication accessibility fallback packet serializes")
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
        out.push_str("# M5 Publication-Component Accessibility & Auto-Narrowing\n\n");
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- As of: `{}`\n", self.as_of));
        out.push_str(&format!(
            "- Families: {} certified across {} / {} frozen families\n",
            self.summary.family_count,
            self.represented_families().len(),
            M5ReleaseCenterComponentFamily::ALL.len(),
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

/// Reads and validates the checked-in publication-component accessibility fallback
/// export.
pub fn current_m5_publication_a11y_fallback_export(
) -> Result<PublicationAccessibilityPacket, PublicationAccessibilityArtifactError> {
    let packet: PublicationAccessibilityPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/release/m5-publication-component-accessibility-fallback-proof/support_export.json"
    )))
    .map_err(PublicationAccessibilityArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(PublicationAccessibilityArtifactError::Validation(
            violations,
        ))
    }
}

/// Errors emitted when reading the checked-in publication-component accessibility
/// fallback export.
#[derive(Debug)]
pub enum PublicationAccessibilityArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<PublicationAccessibilityViolation>),
}

impl fmt::Display for PublicationAccessibilityArtifactError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(
                    f,
                    "publication accessibility fallback export parse failed: {error}"
                )
            }
            Self::Validation(violations) => {
                write!(
                    f,
                    "publication accessibility fallback export failed validation: {} violation(s)",
                    violations.len()
                )
            }
        }
    }
}

impl Error for PublicationAccessibilityArtifactError {}

/// Validation failure for M05-866 publication-component accessibility fallback
/// packets.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PublicationAccessibilityViolation {
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
        dimension: M5PublicationClaimDimension,
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
        family: M5ReleaseCenterComponentFamily,
    },
    MissingDimensionCoverage {
        dimension: M5PublicationClaimDimension,
    },
    MissingClaimTierCoverage {
        claim: M5PublicationSupportClaim,
    },
    MissingConsumerSurfaceCoverage {
        surface: M5ReleaseCenterConsumerSurface,
    },
    SummaryMismatch,
    RawReleaseMaterialInExport,
}

impl fmt::Display for PublicationAccessibilityViolation {
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
                write!(f, "row {id} drops a mandatory release-center label")
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
                    "row {id} over-asserts a certified / supported publication for a weakened one, or narrows spuriously"
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
            Self::RawReleaseMaterialInExport => {
                write!(f, "export contains raw release material")
            }
        }
    }
}

impl Error for PublicationAccessibilityViolation {}

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

/// Builds the canonical, checked-in publication-component accessibility fallback
/// packet. This is the one source of truth shared by the tests, the example dump, and
/// the on-disk support export so all three stay byte-aligned.
pub fn seeded_m5_publication_a11y_fallback_packet() -> PublicationAccessibilityPacket {
    PublicationAccessibilityPacket::new(PublicationAccessibilityPacketInput {
        packet_id: "m5-publication-component-accessibility-fallback:stable:0001".to_owned(),
        as_of: "2026-07-06T00:00:00Z".to_owned(),
        matrix_ref: PUBLICATION_A11Y_FALLBACK_COMPONENT_MATRIX_REF.to_owned(),
        rows: seeded_rows(),
    })
}

fn ev(id: &str) -> Vec<String> {
    vec![format!("evidence:publication-a11y:{id}")]
}

fn all_required_labels() -> Vec<M5ReleaseCenterRequiredLabel> {
    M5ReleaseCenterRequiredLabel::ALL.to_vec()
}

fn copy_export(fields: &[&str]) -> PublicationCopyExportParity {
    PublicationCopyExportParity {
        formats: vec!["text".to_owned(), "json".to_owned(), "markdown".to_owned()],
        export_fields: fields.iter().map(|f| (*f).to_owned()).collect(),
        screenshot_only_prohibited: true,
    }
}

fn condition(
    dimension: M5PublicationClaimDimension,
    state: M5PublicationConditionState,
) -> PublicationClaimConditionEntry {
    PublicationClaimConditionEntry { dimension, state }
}

/// The two consumer surfaces every row ships to at minimum — support / admin export
/// and CLI inspect — so the narrowed state always reaches headless field triage.
fn base_consumers(extra: &[M5ReleaseCenterConsumerSurface]) -> Vec<M5ReleaseCenterConsumerSurface> {
    let mut out = vec![
        M5ReleaseCenterConsumerSurface::SupportExport,
        M5ReleaseCenterConsumerSurface::CliInspect,
    ];
    out.extend_from_slice(extra);
    out
}

/// Disclosures for the CLI-headless and support-export surfaces. A green (full parity)
/// row keeps full label and summary parity on the narrower surfaces; a narrowed row
/// discloses the reduced interactions it drops there.
fn surface_disclosures(
    labels: &[&str],
    state: PublicationNarrowingDisclosureState,
) -> Vec<PublicationRenderingNarrowingDisclosure> {
    let preserved: Vec<String> = labels.iter().map(|l| (*l).to_owned()).collect();
    vec![
        PublicationRenderingNarrowingDisclosure {
            rendering_surface: M5PublicationRenderingSurface::CliHeadless,
            state,
            preserved_labels: preserved.clone(),
            reduced_interactions: vec!["pointer_interaction".to_owned()],
        },
        PublicationRenderingNarrowingDisclosure {
            rendering_surface: M5PublicationRenderingSurface::SupportExport,
            state,
            preserved_labels: preserved,
            reduced_interactions: vec!["live_action".to_owned()],
        },
    ]
}

/// Disclosures for a full-parity (green) row: the narrower surfaces preserve full
/// label and summary parity.
fn parity_surfaces(labels: &[&str]) -> Vec<PublicationRenderingNarrowingDisclosure> {
    surface_disclosures(labels, PublicationNarrowingDisclosureState::ParityPreserved)
}

/// Disclosures for a narrowed (yellow) row: the narrower surfaces disclose their
/// reduced interactions while preserving labels.
fn narrowed_surfaces(labels: &[&str]) -> Vec<PublicationRenderingNarrowingDisclosure> {
    surface_disclosures(
        labels,
        PublicationNarrowingDisclosureState::DisclosedNarrowed,
    )
}

fn rendering_surfaces() -> Vec<M5PublicationRenderingSurface> {
    vec![
        M5PublicationRenderingSurface::DesktopFull,
        M5PublicationRenderingSurface::CliHeadless,
        M5PublicationRenderingSurface::SupportExport,
    ]
}

fn seeded_rows() -> Vec<PublicationAccessibilityRow> {
    vec![
        // Release candidate card — evidence freshness is current and no hard blocker is
        // open, so the card carries a fully certified, promotable candidate and is
        // reachable on every surface (green).
        PublicationAccessibilityRow {
            record_kind: PUBLICATION_A11Y_FALLBACK_ROW_RECORD_KIND.to_owned(),
            schema_version: PUBLICATION_A11Y_FALLBACK_SCHEMA_VERSION,
            row_id: "a11y:release-candidate-card".to_owned(),
            component_family: M5ReleaseCenterComponentFamily::ReleaseCandidateCard,
            source_family_schema_ref: PUBLICATION_A11Y_FALLBACK_COMPONENT_MATRIX_REF.to_owned(),
            publication_context_ref: "candidate:rc:0001".to_owned(),
            fallback_modalities: vec![
                M5PublicationFallbackModality::List,
                M5PublicationFallbackModality::Textual,
                M5PublicationFallbackModality::Cli,
            ],
            reaches_canonical_truth: true,
            keyboard_reach: PublicationNonVisualReachState::ReachableAndLabeled,
            screen_reader_reach: PublicationNonVisualReachState::ReachableAndLabeled,
            cli_reach: PublicationNonVisualReachState::ReachableAndLabeled,
            export_summary: PublicationExportSummaryState::ReconstructableWithoutScreenshot,
            export_summary_ref: "summary:release-candidate-card:a11y".to_owned(),
            copy_export: copy_export(&[
                "candidate_scope",
                "blocker_state",
                "evidence_freshness",
                "promotability",
            ]),
            full_support_claim: M5PublicationSupportClaim::Certified,
            claim_conditions: vec![condition(
                M5PublicationClaimDimension::EvidenceFreshness,
                M5PublicationConditionState::Verified,
            )],
            claim_narrow: None,
            rendering_surfaces: rendering_surfaces(),
            narrowing_disclosures: parity_surfaces(&[
                "candidate_scope",
                "blocker_state",
                "evidence_freshness",
            ]),
            required_labels: all_required_labels(),
            consumer_surfaces: base_consumers(&[
                M5ReleaseCenterConsumerSurface::ReleaseCenterUi,
                M5ReleaseCenterConsumerSurface::HelpAbout,
            ]),
            source_refs: vec![
                "UX Guide §15.3 release-center templates".to_owned(),
                PUBLICATION_A11Y_FALLBACK_DOC_REF.to_owned(),
            ],
            observed_at: "2026-07-06T00:00:00Z".to_owned(),
            evidence_refs: ev("release-candidate-card"),
        },
        // Version-bump row — the derived public-surface impact is fully resolved, so
        // the row carries a supported, self-sufficient bump proposal with no unstated
        // compatibility impact (green).
        PublicationAccessibilityRow {
            record_kind: PUBLICATION_A11Y_FALLBACK_ROW_RECORD_KIND.to_owned(),
            schema_version: PUBLICATION_A11Y_FALLBACK_SCHEMA_VERSION,
            row_id: "a11y:version-bump-row".to_owned(),
            component_family: M5ReleaseCenterComponentFamily::VersionBumpRow,
            source_family_schema_ref: PUBLICATION_A11Y_FALLBACK_COMPONENT_MATRIX_REF.to_owned(),
            publication_context_ref: "bump:proposal:0002".to_owned(),
            fallback_modalities: vec![
                M5PublicationFallbackModality::List,
                M5PublicationFallbackModality::Textual,
                M5PublicationFallbackModality::Cli,
            ],
            reaches_canonical_truth: true,
            keyboard_reach: PublicationNonVisualReachState::ReachableAndLabeled,
            screen_reader_reach: PublicationNonVisualReachState::ReachableAndLabeled,
            cli_reach: PublicationNonVisualReachState::ReachableAndLabeled,
            export_summary: PublicationExportSummaryState::ReconstructableWithoutScreenshot,
            export_summary_ref: "summary:version-bump-row:a11y".to_owned(),
            copy_export: copy_export(&[
                "prior_version",
                "next_version",
                "bump_class",
                "public_surface_impact",
            ]),
            full_support_claim: M5PublicationSupportClaim::Supported,
            claim_conditions: vec![condition(
                M5PublicationClaimDimension::PublicSurfaceImpact,
                M5PublicationConditionState::Verified,
            )],
            claim_narrow: None,
            rendering_surfaces: rendering_surfaces(),
            narrowing_disclosures: parity_surfaces(&[
                "bump_class",
                "public_surface_impact",
                "next_version",
            ]),
            required_labels: all_required_labels(),
            consumer_surfaces: base_consumers(&[
                M5ReleaseCenterConsumerSurface::ServiceHealth,
                M5ReleaseCenterConsumerSurface::DocsPortal,
            ]),
            source_refs: vec![
                "UI/UX Spec compatibility-claim obligations".to_owned(),
                PUBLICATION_A11Y_FALLBACK_DOC_REF.to_owned(),
            ],
            observed_at: "2026-07-06T00:00:00Z".to_owned(),
            evidence_refs: ev("version-bump-row"),
        },
        // Publish-target row — the target's auth source is only partially resolved (an
        // ambient credential still shadows the scoped one), so the publish claim
        // auto-narrows to degraded until the auth posture clears (yellow).
        PublicationAccessibilityRow {
            record_kind: PUBLICATION_A11Y_FALLBACK_ROW_RECORD_KIND.to_owned(),
            schema_version: PUBLICATION_A11Y_FALLBACK_SCHEMA_VERSION,
            row_id: "a11y:publish-target-row".to_owned(),
            component_family: M5ReleaseCenterComponentFamily::PublishTargetRow,
            source_family_schema_ref: PUBLICATION_A11Y_FALLBACK_COMPONENT_MATRIX_REF.to_owned(),
            publication_context_ref: "target:registry:0003".to_owned(),
            fallback_modalities: vec![
                M5PublicationFallbackModality::List,
                M5PublicationFallbackModality::Textual,
                M5PublicationFallbackModality::Cli,
            ],
            reaches_canonical_truth: true,
            keyboard_reach: PublicationNonVisualReachState::ReachableAndLabeled,
            screen_reader_reach: PublicationNonVisualReachState::ReachableAndLabeled,
            cli_reach: PublicationNonVisualReachState::ReachableAndLabeled,
            export_summary: PublicationExportSummaryState::ReconstructableWithoutScreenshot,
            export_summary_ref: "summary:publish-target-row:a11y".to_owned(),
            copy_export: copy_export(&[
                "target_visibility",
                "target_mutability",
                "auth_source",
                "dry_run_availability",
            ]),
            full_support_claim: M5PublicationSupportClaim::Certified,
            claim_conditions: vec![condition(
                M5PublicationClaimDimension::TargetAuthPosture,
                M5PublicationConditionState::Partial,
            )],
            claim_narrow: Some(PublicationClaimAutoNarrow {
                narrowed_to: M5PublicationSupportClaim::Degraded,
                binding_dimension: M5PublicationClaimDimension::TargetAuthPosture,
                trigger: M5ReleaseCenterDowngradeTrigger::TargetAuthSourceMasked,
                narrowed_label:
                    "Target auth partially resolved — publish shown degraded until the scoped credential wins over the ambient one"
                        .to_owned(),
                preserves_canonical_identity: true,
            }),
            rendering_surfaces: rendering_surfaces(),
            narrowing_disclosures: narrowed_surfaces(&[
                "target_visibility",
                "target_mutability",
                "auth_source",
            ]),
            required_labels: all_required_labels(),
            consumer_surfaces: base_consumers(&[
                M5ReleaseCenterConsumerSurface::AdminConsole,
                M5ReleaseCenterConsumerSurface::EvaluationPack,
            ]),
            source_refs: vec![
                "TDD publish-target architecture / auth-source disclosure".to_owned(),
                PUBLICATION_A11Y_FALLBACK_DOC_REF.to_owned(),
            ],
            observed_at: "2026-07-06T00:00:00Z".to_owned(),
            evidence_refs: ev("publish-target-row"),
        },
        // Artifact-provenance-bundle card — hierarchy-heavy (digest-lineage tree with
        // attestation / SBOM sub-rows); the signature and attestation could not be
        // verified on this build, so the provenance claim auto-narrows to unverified
        // and binds the lineage tree to a flat list / textual path (yellow).
        PublicationAccessibilityRow {
            record_kind: PUBLICATION_A11Y_FALLBACK_ROW_RECORD_KIND.to_owned(),
            schema_version: PUBLICATION_A11Y_FALLBACK_SCHEMA_VERSION,
            row_id: "a11y:artifact-provenance-bundle-card".to_owned(),
            component_family: M5ReleaseCenterComponentFamily::ArtifactProvenanceBundleCard,
            source_family_schema_ref: PUBLICATION_A11Y_FALLBACK_COMPONENT_MATRIX_REF.to_owned(),
            publication_context_ref: "artifact:bundle:0004".to_owned(),
            fallback_modalities: vec![
                M5PublicationFallbackModality::Structured,
                M5PublicationFallbackModality::List,
                M5PublicationFallbackModality::Textual,
                M5PublicationFallbackModality::Cli,
            ],
            reaches_canonical_truth: true,
            keyboard_reach: PublicationNonVisualReachState::ReachableAndLabeled,
            screen_reader_reach: PublicationNonVisualReachState::DisclosedReducedButReachable,
            cli_reach: PublicationNonVisualReachState::ReachableAndLabeled,
            export_summary: PublicationExportSummaryState::ReconstructableWithoutScreenshot,
            export_summary_ref: "summary:artifact-provenance-bundle-card:a11y".to_owned(),
            copy_export: copy_export(&[
                "signature_status",
                "attestation_status",
                "sbom_status",
                "digest_lineage",
            ]),
            full_support_claim: M5PublicationSupportClaim::Certified,
            claim_conditions: vec![condition(
                M5PublicationClaimDimension::SignatureAttestationState,
                M5PublicationConditionState::Unverified,
            )],
            claim_narrow: Some(PublicationClaimAutoNarrow {
                narrowed_to: M5PublicationSupportClaim::Unverified,
                binding_dimension: M5PublicationClaimDimension::SignatureAttestationState,
                trigger: M5ReleaseCenterDowngradeTrigger::SignatureOrAttestationOverclaimed,
                narrowed_label:
                    "Signature and attestation unverified on this build — provenance shown from unproven material, not certified"
                        .to_owned(),
                preserves_canonical_identity: true,
            }),
            rendering_surfaces: rendering_surfaces(),
            narrowing_disclosures: narrowed_surfaces(&[
                "signature_status",
                "attestation_status",
                "digest_lineage",
            ]),
            required_labels: all_required_labels(),
            consumer_surfaces: base_consumers(&[
                M5ReleaseCenterConsumerSurface::MirrorConsole,
                M5ReleaseCenterConsumerSurface::ProductUi,
            ]),
            source_refs: vec![
                "TAD release artifact graph / build & provenance architecture".to_owned(),
                PUBLICATION_A11Y_FALLBACK_DOC_REF.to_owned(),
            ],
            observed_at: "2026-07-06T00:00:00Z".to_owned(),
            evidence_refs: ev("artifact-provenance-bundle-card"),
        },
        // Promotion-timeline step — the mirror verification proof aged out and is being
        // re-established, so the step's promotion claim auto-narrows to provisional and
        // reads from last-known mirror state rather than a fresh verification (yellow).
        PublicationAccessibilityRow {
            record_kind: PUBLICATION_A11Y_FALLBACK_ROW_RECORD_KIND.to_owned(),
            schema_version: PUBLICATION_A11Y_FALLBACK_SCHEMA_VERSION,
            row_id: "a11y:promotion-timeline-step".to_owned(),
            component_family: M5ReleaseCenterComponentFamily::PromotionTimelineStep,
            source_family_schema_ref: PUBLICATION_A11Y_FALLBACK_COMPONENT_MATRIX_REF.to_owned(),
            publication_context_ref: "promotion:step:0005".to_owned(),
            fallback_modalities: vec![
                M5PublicationFallbackModality::List,
                M5PublicationFallbackModality::Textual,
                M5PublicationFallbackModality::Cli,
            ],
            reaches_canonical_truth: true,
            keyboard_reach: PublicationNonVisualReachState::ReachableAndLabeled,
            screen_reader_reach: PublicationNonVisualReachState::ReachableAndLabeled,
            cli_reach: PublicationNonVisualReachState::ReachableAndLabeled,
            export_summary: PublicationExportSummaryState::ReconstructableWithoutScreenshot,
            export_summary_ref: "summary:promotion-timeline-step:a11y".to_owned(),
            copy_export: copy_export(&[
                "rollout_ring",
                "stage_state",
                "immutable_digest",
                "mirror_verification",
            ]),
            full_support_claim: M5PublicationSupportClaim::Certified,
            claim_conditions: vec![condition(
                M5PublicationClaimDimension::MirrorVerification,
                M5PublicationConditionState::Stale,
            )],
            claim_narrow: Some(PublicationClaimAutoNarrow {
                narrowed_to: M5PublicationSupportClaim::Provisional,
                binding_dimension: M5PublicationClaimDimension::MirrorVerification,
                trigger: M5ReleaseCenterDowngradeTrigger::ProofStale,
                narrowed_label:
                    "Mirror verification stale — promotion shown from last-known mirror state until re-verification lands"
                        .to_owned(),
                preserves_canonical_identity: true,
            }),
            rendering_surfaces: rendering_surfaces(),
            narrowing_disclosures: narrowed_surfaces(&[
                "rollout_ring",
                "stage_state",
                "immutable_digest",
            ]),
            required_labels: all_required_labels(),
            consumer_surfaces: base_consumers(&[
                M5ReleaseCenterConsumerSurface::ReleaseCenterUi,
                M5ReleaseCenterConsumerSurface::DocsPortal,
            ]),
            source_refs: vec![
                "TAD promotion evidence / mirror parity".to_owned(),
                PUBLICATION_A11Y_FALLBACK_DOC_REF.to_owned(),
            ],
            observed_at: "2026-07-06T00:00:00Z".to_owned(),
            evidence_refs: ev("promotion-timeline-step"),
        },
        // Rollback / revocation row — the rollback requires policy approval and its
        // blast radius cannot be executed until approved, so the row auto-narrows to
        // policy-blocked rather than presenting a ready "Roll back now" (yellow).
        PublicationAccessibilityRow {
            record_kind: PUBLICATION_A11Y_FALLBACK_ROW_RECORD_KIND.to_owned(),
            schema_version: PUBLICATION_A11Y_FALLBACK_SCHEMA_VERSION,
            row_id: "a11y:rollback-revocation-row".to_owned(),
            component_family: M5ReleaseCenterComponentFamily::RollbackRevocationRow,
            source_family_schema_ref: PUBLICATION_A11Y_FALLBACK_COMPONENT_MATRIX_REF.to_owned(),
            publication_context_ref: "rollback:event:0006".to_owned(),
            fallback_modalities: vec![
                M5PublicationFallbackModality::List,
                M5PublicationFallbackModality::Textual,
                M5PublicationFallbackModality::Cli,
            ],
            reaches_canonical_truth: true,
            keyboard_reach: PublicationNonVisualReachState::ReachableAndLabeled,
            screen_reader_reach: PublicationNonVisualReachState::ReachableAndLabeled,
            cli_reach: PublicationNonVisualReachState::ReachableAndLabeled,
            export_summary: PublicationExportSummaryState::ReconstructableWithoutScreenshot,
            export_summary_ref: "summary:rollback-revocation-row:a11y".to_owned(),
            copy_export: copy_export(&[
                "blast_radius",
                "revocation_scope",
                "last_known_good",
                "reversible_window",
            ]),
            full_support_claim: M5PublicationSupportClaim::Supported,
            claim_conditions: vec![condition(
                M5PublicationClaimDimension::RollbackBlastRadius,
                M5PublicationConditionState::PolicyBlocked,
            )],
            claim_narrow: Some(PublicationClaimAutoNarrow {
                narrowed_to: M5PublicationSupportClaim::PolicyBlocked,
                binding_dimension: M5PublicationClaimDimension::RollbackBlastRadius,
                trigger: M5ReleaseCenterDowngradeTrigger::RollbackBlastRadiusUnderstated,
                narrowed_label:
                    "Rollback blocked by policy — blast radius not executable until an approver signs off"
                        .to_owned(),
                preserves_canonical_identity: true,
            }),
            rendering_surfaces: rendering_surfaces(),
            narrowing_disclosures: narrowed_surfaces(&[
                "blast_radius",
                "revocation_scope",
                "last_known_good",
            ]),
            required_labels: all_required_labels(),
            consumer_surfaces: base_consumers(&[
                M5ReleaseCenterConsumerSurface::AdminConsole,
                M5ReleaseCenterConsumerSurface::MirrorConsole,
            ]),
            source_refs: vec![
                "TDD update & rollback / revocation records".to_owned(),
                PUBLICATION_A11Y_FALLBACK_DOC_REF.to_owned(),
            ],
            observed_at: "2026-07-06T00:00:00Z".to_owned(),
            evidence_refs: ev("rollback-revocation-row"),
        },
    ]
}
