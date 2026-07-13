//! Keyboard / screen-reader / high-zoom / reduced-motion / battery-saver / thermal-pressure / CLI / export
//! parity, and honest automatic claim narrowing for the M5 motion-token / reduced-motion / opacity-scrim /
//! layer-order / portal-ownership / iconography / illustration-boundary visual-interaction families.
//!
//! This module is the M05-1154 accessibility-power-and-auto-narrowing capstone over the frozen M5
//! motion / layer / iconography matrix ([`crate::m5_motion_layer_iconography_matrix`]). Where the freeze
//! matrix defines the seven governed visual-interaction families, and the 1149-1152 implementation lanes
//! resolve their per-surface motion, scrim, layering, icon, and illustration truth, this lane certifies —
//! per interaction family — that motion / overlay / layer / portal / icon / illustration claims stay
//! **keyboard-reachable, screen-reader-announced, high-zoom-legible, reduced-motion / power-saver /
//! thermal-safe, CLI/export-safe, and self-narrowing** rather than presenting a protected-path-delaying
//! motion, a scrim that erases orientation, an overlay that bypasses the shared z-order, an unlabeled
//! destructive icon, or an illustration masquerading as operational truth as still a stable, trusted
//! visual-interaction surface:
//!
//! - **Keyboard / screen-reader / high-zoom / reduced-motion / power-thermal / CLI reach.** Every family
//!   exposes a keyboard-reachable, screen-reader-announced, high-zoom-reflowing, reduced-motion-safe,
//!   battery-saver / thermal-pressure-safe, and CLI/headless-reachable path into the same interaction
//!   identity, semantic role, token reference, motion profile, layer tier, and accessible fallback the
//!   rendered surface shows — never a motion-only affordance, a hover-only overlay, an unlabeled symbol, or
//!   a decoration-only cue that strands assistive-tech or headless-CLI users. Structure-heavy families (the
//!   layer stack's z-tiers, the icon registry's semantic classes, the illustration set's placements)
//!   additionally bind their structured layout to a flat list / textual / CLI path.
//! - **Export parity.** The support / release / CLI export reconstructs each family's meaning from typed
//!   tokens and opaque refs **without a raw payload**, preserving the same identity, semantic role, token
//!   reference, motion profile, layer tier, and accessible fallback shown in-product so support, help, and
//!   release proof can reconstruct which visual-interaction truth class was active without leaking a raw
//!   duration curve, private z-index, icon glyph blob, or renderer-only screenshot.
//! - **Honest auto-narrowing.** When a motion token's protected-path timing evidence is stale, a
//!   reduced-motion clamp's power-saver / thermal safety cannot be confirmed, an opacity scrim's
//!   orientation / contrast preservation is unconfirmed, a portal's owning-surface attachment cannot be
//!   confirmed, or an illustration boundary can only be partially disclosed, the family's claim auto-narrows
//!   from `trusted_interaction_surface` / `reviewable_interaction_surface` to a motion-timing-unverified /
//!   reduced-motion-clamp-unverified / scrim-orientation-unverified / portal-ownership-unverified /
//!   illustration-boundary-disclosed projection, discloses the narrowing with a precise trigger and binding
//!   dimension, and preserves the canonical interaction identity / last-known token reference. The
//!   underlying motion / scrim / layer / icon / illustration truth is never dropped opaquely. A family with
//!   every dimension intact must NOT carry a spurious narrowing, and a protected-path-delaying / scrim-erasing
//!   / overlay-bypassing / unlabeled-destructive / impersonating state can never keep a trusted, stable
//!   interaction claim — meaning is never conveyed by motion, decoration, or an unlabeled symbol alone.
//! - **Cross-surface disclosure.** The same narrowed state surfaces in the shell UI, the editor UI, the help
//!   UI, the marketplace UI, the onboarding UI, the settings UI, the CLI export, the support export, and the
//!   product UI so product, help, and release publication stay aligned on downgrade behavior rather than
//!   drifting in copy — a trusted-looking interaction surface can never outrun the motion / scrim / layer /
//!   portal / icon / illustration evidence it is being viewed away from.
//!
//! Each [`VisualInteractionAccessibilityRow`] keys on one
//! [`crate::m5_motion_layer_iconography_matrix::M5VisualInteractionFamily`] and reuses that frozen family
//! vocabulary plus the frozen [`M5VisualInteractionRequiredLabel`], [`M5VisualInteractionDowngradeTrigger`],
//! and shared [`M5VisualInteractionConsumerSurface`] consumer surfaces rather than minting parallel
//! synonyms, so the certified labels stay byte-identical to the matrix and the sibling interaction packets.
//!
//! The packet is metadata-only: raw duration curves, z-index integers, glyph blobs, credentials, secrets,
//! and endpoint refs never cross this boundary; the packet carries only typed class tokens, opaque
//! interaction refs, booleans, and controlled labels so support, release, and diagnostics exports can
//! reconstruct exactly which visual-interaction truth class was active without leaking sensitive material or
//! a raw payload.

#[cfg(test)]
mod tests;

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

// Reused frozen visual-interaction vocabulary — the capstone certifies the freeze matrix's families,
// required labels, downgrade triggers, and consumer surfaces rather than mint parallel ones.
use crate::m5_motion_layer_iconography_matrix::{
    M5VisualInteractionConsumerSurface, M5VisualInteractionDowngradeTrigger,
    M5VisualInteractionFamily, M5VisualInteractionRequiredLabel,
    M5_MOTION_LAYER_ICONOGRAPHY_MATRIX_SCHEMA_REF,
};

/// Schema version stamped on the M05-1154 motion-layer-iconography accessibility parity packet.
pub const MOTION_LAYER_ICONOGRAPHY_A11Y_SCHEMA_VERSION: u32 = 1;

/// Stable record-kind tag carried by [`VisualInteractionAccessibilityPacket`].
pub const MOTION_LAYER_ICONOGRAPHY_A11Y_RECORD_KIND: &str =
    "m5_motion_layer_iconography_accessibility_parity_packet";

/// Stable record-kind tag carried by each [`VisualInteractionAccessibilityRow`].
pub const MOTION_LAYER_ICONOGRAPHY_A11Y_ROW_RECORD_KIND: &str =
    "m5_motion_layer_iconography_accessibility_parity_row";

/// Repo-relative path of the boundary schema.
pub const MOTION_LAYER_ICONOGRAPHY_A11Y_SCHEMA_REF: &str =
    "schemas/design-system/m5-motion-layer-iconography-accessibility-parity.schema.json";

/// Repo-relative path of the contract doc.
pub const MOTION_LAYER_ICONOGRAPHY_A11Y_DOC_REF: &str =
    "docs/design-system/m5_motion_layer_iconography_accessibility_parity.md";

/// Repo-relative path of the frozen motion-layer-iconography matrix this lane certifies.
pub const MOTION_LAYER_ICONOGRAPHY_A11Y_MATRIX_REF: &str =
    M5_MOTION_LAYER_ICONOGRAPHY_MATRIX_SCHEMA_REF;

/// Repo-relative path of the protected fixture directory.
pub const MOTION_LAYER_ICONOGRAPHY_A11Y_FIXTURE_DIR: &str =
    "fixtures/ui/m5-motion-layer-iconography-accessibility-parity";

/// Repo-relative path of the checked support-export artifact (the `include_str!` canonical).
pub const MOTION_LAYER_ICONOGRAPHY_A11Y_ARTIFACT_REF: &str =
    "artifacts/release/m5-motion-layer-iconography-accessibility-parity/support_export.json";

/// Repo-relative path of the checked machine-readable matrix CSV.
pub const MOTION_LAYER_ICONOGRAPHY_A11Y_CSV_REF: &str =
    "artifacts/release/m5-motion-layer-iconography-accessibility-parity/matrix.csv";

/// Repo-relative path of the checked Markdown report.
pub const MOTION_LAYER_ICONOGRAPHY_A11Y_REPORT_REF: &str =
    "artifacts/release/m5-motion-layer-iconography-accessibility-parity.md";

/// The reusable interaction families that render a dense, structured surface (the layer stack's z-tiers, the
/// icon registry's semantic classes, the illustration set's placements) and therefore MUST bind their
/// structured layout to an equivalent flat list / textual / CLI path so the structure is navigable
/// non-visually.
const fn family_is_structure_heavy(family: M5VisualInteractionFamily) -> bool {
    matches!(
        family,
        M5VisualInteractionFamily::LayerOrder
            | M5VisualInteractionFamily::Iconography
            | M5VisualInteractionFamily::Illustration
    )
}

/// The visual-interaction-truth dimension whose weakening a family primarily discloses. Every row must model
/// at least this dimension so its key weakening axis is covered.
const fn family_primary_dimension(
    family: M5VisualInteractionFamily,
) -> M5VisualInteractionClaimDimension {
    match family {
        M5VisualInteractionFamily::MotionToken => {
            M5VisualInteractionClaimDimension::MotionTimingClarity
        }
        M5VisualInteractionFamily::ReducedMotion => {
            M5VisualInteractionClaimDimension::ReducedMotionSafetyClarity
        }
        M5VisualInteractionFamily::OpacityScrim => {
            M5VisualInteractionClaimDimension::ScrimOrientationClarity
        }
        M5VisualInteractionFamily::LayerOrder => {
            M5VisualInteractionClaimDimension::LayerOrderClarity
        }
        M5VisualInteractionFamily::PortalOwnership => {
            M5VisualInteractionClaimDimension::PortalOwnershipClarity
        }
        M5VisualInteractionFamily::Iconography => {
            M5VisualInteractionClaimDimension::IconSemanticsClarity
        }
        M5VisualInteractionFamily::Illustration => {
            M5VisualInteractionClaimDimension::IllustrationBoundaryClarity
        }
    }
}

/// A rendered fallback modality for a visual-interaction family.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5VisualInteractionFallbackModality {
    /// A rich, structured (z-tier stack / icon class / illustration placement) projection.
    Structured,
    /// A flat list projection.
    List,
    /// A textual / label-first projection.
    Textual,
    /// A CLI / headless text projection.
    Cli,
}

impl M5VisualInteractionFallbackModality {
    /// Returns true when the modality is reachable without interpreting a rich, structured surface
    /// (i.e. a keyboard / screen-reader / CLI path).
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

/// A rendering-surface capability tier. Distinct from the semantic consumer surface: the same interaction may
/// render at desktop-full capability or narrow to a companion, read-only browser, headless CLI, docs export,
/// or support export.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5VisualInteractionRenderingSurface {
    /// The full-capability desktop surface.
    DesktopFull,
    /// The companion app.
    CompanionApp,
    /// A read-only browser projection.
    BrowserReadonly,
    /// A headless CLI surface.
    CliHeadless,
    /// A docs / help export projection.
    DocsExport,
    /// A support / release / evaluation export.
    SupportExport,
}

impl M5VisualInteractionRenderingSurface {
    /// Returns true when the surface narrows interactivity below the desktop full-capability baseline and
    /// therefore must disclose its reduction.
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
            Self::DocsExport => "docs_export",
            Self::SupportExport => "support_export",
        }
    }
}

/// Keyboard / screen-reader / high-zoom / reduced-motion / power-thermal / CLI reach for an interaction's
/// non-visual path.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum VisualInteractionNonVisualReachState {
    /// Fully traversable and labeled with no loss.
    ReachableAndLabeled,
    /// Reachable and labeled, but with a disclosed reduction (yellow).
    DisclosedReducedButReachable,
    /// A view-only / hover-only / motion-only surface that traps keyboard / assistive-tech / headless-CLI
    /// users (red).
    ViewOnlyTrap,
}

impl VisualInteractionNonVisualReachState {
    /// Returns true when the state never strands keyboard / assistive-tech / CLI users.
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

/// Whether an export-safe summary preserves the interaction meaning without leaking a raw payload.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum VisualInteractionExportSummaryState {
    /// The interaction meaning reconstructs from the metadata summary without a raw payload.
    ReconstructableWithoutRawPayload,
    /// Partial capture, but disclosed (yellow).
    DisclosedPartialCapture,
    /// The export can only carry meaning by dumping a raw payload (red).
    RequiresRawPayload,
}

impl VisualInteractionExportSummaryState {
    /// Returns true when the export never falls back to leaking a raw payload.
    pub const fn never_requires_raw_payload(self) -> bool {
        !matches!(self, Self::RequiresRawPayload)
    }

    /// Returns true when the state carries a disclosed reduction.
    pub const fn is_disclosed_reduction(self) -> bool {
        matches!(self, Self::DisclosedPartialCapture)
    }

    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ReconstructableWithoutRawPayload => "reconstructable_without_raw_payload",
            Self::DisclosedPartialCapture => "disclosed_partial_capture",
            Self::RequiresRawPayload => "requires_raw_payload",
        }
    }
}

/// Whether a narrower rendering surface discloses its reduced interactivity.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum VisualInteractionNarrowingDisclosureState {
    /// Full label and summary parity with the desktop surface.
    ParityPreserved,
    /// Reduced interactivity, disclosed with preserved labels (yellow).
    DisclosedNarrowed,
    /// Interactivity, state, or tokens dropped without disclosure (red).
    SilentlyDropped,
}

impl VisualInteractionNarrowingDisclosureState {
    /// Returns true when the surface never silently drops state or tokens.
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

/// The visual-interaction claim ceiling a family asserts: how strong a trusted / stable posture it lets a
/// surface present. Auto-narrowing lowers this ceiling when a motion-timing / reduced-motion-safety /
/// scrim-orientation / portal-ownership / illustration-boundary dimension weakens so a
/// protected-path-delaying motion, an unconfirmed reduced-motion clamp, an orientation-erasing scrim, a
/// detached portal, or a partially-disclosed illustration boundary can never keep an old
/// `TrustedInteractionSurface` or `ReviewableInteractionSurface` label — meaning is never conveyed by
/// motion, decoration, or an unlabeled symbol alone.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5VisualInteractionA11yClaim {
    /// Trusted interaction surface: a fully current, protected-path-safe, reduced-motion-safe,
    /// orientation-preserving, owning-surface-attached, labeled interaction — the strongest claim, a
    /// visual-interaction surface Aureline can present as exactly trusted and stable right now.
    TrustedInteractionSurface,
    /// Reviewable interaction surface: a self-sufficient, inspectable read-only interaction projection (a
    /// static z-tier / token reference a user can inspect) that is not itself an authoritative, live-rendering
    /// surface.
    ReviewableInteractionSurface,
    /// Motion-timing-unverified projection: the motion token's protected-path timing evidence is stale; the
    /// interaction stays a motion-timing-unverified projection with its last-known semantic role and static
    /// fallback preserved, never a fresh, protected-path-delaying motion shown as authoritative.
    MotionTimingUnverifiedProjection,
    /// Reduced-motion-clamp-unverified projection: the reduced-motion / power-saver / thermal clamp cannot be
    /// confirmed; the interaction stays a reduced-motion-clamp-unverified projection that keeps the last-known
    /// static fallback explicit, never a motion-only cue shown as clamp-safe.
    ReducedMotionClampUnverifiedProjection,
    /// Scrim-orientation-unverified projection: an opacity scrim's orientation / contrast preservation cannot
    /// be confirmed; the interaction stays a scrim-orientation-unverified projection that keeps the workspace
    /// orientation cue inspectable, never a scrim shown as orientation-safe when it may erase context.
    ScrimOrientationUnverifiedProjection,
    /// Portal-ownership-unverified projection: a portal's owning-surface attachment cannot be confirmed; the
    /// interaction stays a portal-ownership-unverified projection that keeps the owning-surface reference and
    /// z-tier inspectable, never an overlay shown as attached when it may bypass the shared z-order.
    PortalOwnershipUnverifiedProjection,
    /// Illustration-boundary-disclosed projection: an illustration boundary can only be partially disclosed;
    /// the interaction stays an illustration-boundary-disclosed projection that discloses the partial
    /// secondary-illustration boundary, never a decorative illustration shown as operational or security
    /// truth.
    IllustrationBoundaryDisclosedProjection,
}

impl M5VisualInteractionA11yClaim {
    /// Every claim tier, strongest first.
    pub const ALL: [Self; 7] = [
        Self::TrustedInteractionSurface,
        Self::ReviewableInteractionSurface,
        Self::MotionTimingUnverifiedProjection,
        Self::ReducedMotionClampUnverifiedProjection,
        Self::ScrimOrientationUnverifiedProjection,
        Self::PortalOwnershipUnverifiedProjection,
        Self::IllustrationBoundaryDisclosedProjection,
    ];

    /// Capability rank; a higher rank asserts a stronger posture. Narrowing lowers rank.
    pub const fn capability_rank(self) -> u8 {
        match self {
            Self::TrustedInteractionSurface => 6,
            Self::ReviewableInteractionSurface => 5,
            Self::MotionTimingUnverifiedProjection => 4,
            Self::ReducedMotionClampUnverifiedProjection => 3,
            Self::ScrimOrientationUnverifiedProjection => 2,
            Self::PortalOwnershipUnverifiedProjection => 1,
            Self::IllustrationBoundaryDisclosedProjection => 0,
        }
    }

    /// Returns true when this claim asserts a fully trusted, stable interaction surface.
    pub const fn asserts_trusted_surface(self) -> bool {
        matches!(self, Self::TrustedInteractionSurface)
    }

    /// Returns true when this claim asserts a fully self-sufficient (trusted or reviewable) surface.
    pub const fn asserts_self_sufficient_surface(self) -> bool {
        matches!(
            self,
            Self::TrustedInteractionSurface | Self::ReviewableInteractionSurface
        )
    }

    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::TrustedInteractionSurface => "trusted_interaction_surface",
            Self::ReviewableInteractionSurface => "reviewable_interaction_surface",
            Self::MotionTimingUnverifiedProjection => "motion_timing_unverified_projection",
            Self::ReducedMotionClampUnverifiedProjection => {
                "reduced_motion_clamp_unverified_projection"
            }
            Self::ScrimOrientationUnverifiedProjection => "scrim_orientation_unverified_projection",
            Self::PortalOwnershipUnverifiedProjection => "portal_ownership_unverified_projection",
            Self::IllustrationBoundaryDisclosedProjection => {
                "illustration_boundary_disclosed_projection"
            }
        }
    }
}

/// The motion-timing / reduced-motion-safety / scrim-orientation / layer-order / portal-ownership /
/// icon-semantics / illustration-boundary dimension whose state governs how far an interaction may claim to
/// be a fully trusted, stable visual surface. The dimensions map 1:1 to the seven frozen interaction
/// families so every family carries an honest narrowing path.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5VisualInteractionClaimDimension {
    /// Motion-timing clarity: does the motion token keep origin / continuity / completion cues without
    /// delaying protected-path input (motion-token)?
    MotionTimingClarity,
    /// Reduced-motion-safety clarity: does the reduced-motion behavior honor reduced-motion / power-saver /
    /// thermal clamps with a static fallback rather than a motion-only cue (reduced-motion)?
    ReducedMotionSafetyClarity,
    /// Scrim-orientation clarity: does the opacity scrim preserve workspace orientation and text contrast
    /// rather than erasing context (opacity-scrim)?
    ScrimOrientationClarity,
    /// Layer-order clarity: does the layer follow the one shared z-order model rather than an ad-hoc
    /// always-on-top bypass (layer-order)?
    LayerOrderClarity,
    /// Portal-ownership clarity: does the portal stay attached to its owning surface rather than detaching
    /// (portal-ownership)?
    PortalOwnershipClarity,
    /// Icon-semantics clarity: does the iconography stay semantic and labeled rather than an unlabeled symbol
    /// for an uncommon or destructive action (iconography)?
    IconSemanticsClarity,
    /// Illustration-boundary clarity: does the illustration stay secondary rather than impersonating
    /// operational, safety, or security truth (illustration)?
    IllustrationBoundaryClarity,
}

impl M5VisualInteractionClaimDimension {
    /// Every dimension, in declaration order.
    pub const ALL: [Self; 7] = [
        Self::MotionTimingClarity,
        Self::ReducedMotionSafetyClarity,
        Self::ScrimOrientationClarity,
        Self::LayerOrderClarity,
        Self::PortalOwnershipClarity,
        Self::IconSemanticsClarity,
        Self::IllustrationBoundaryClarity,
    ];

    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::MotionTimingClarity => "motion_timing_clarity",
            Self::ReducedMotionSafetyClarity => "reduced_motion_safety_clarity",
            Self::ScrimOrientationClarity => "scrim_orientation_clarity",
            Self::LayerOrderClarity => "layer_order_clarity",
            Self::PortalOwnershipClarity => "portal_ownership_clarity",
            Self::IconSemanticsClarity => "icon_semantics_clarity",
            Self::IllustrationBoundaryClarity => "illustration_boundary_clarity",
        }
    }
}

/// The observed condition of one visual-interaction-truth dimension. Anything weaker than
/// [`Self::FullyQualified`] imposes a narrowing ceiling on the interaction's claim. The stale / unconfirmed
/// states the lane must auto-narrow on as *weakened evidence* — a stale motion timing, an unconfirmed
/// reduced-motion clamp, an unconfirmed scrim orientation, and an unconfirmed portal ownership — are the
/// states that [`Self::cannot_be_shown_trusted`] flags. A partially-disclosed illustration boundary is an
/// honest disclosed-absence operation (a partial secondary-illustration boundary shown honestly with an
/// inspectable note), not a truth overstatement, so it is deliberately excluded there.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5VisualInteractionConditionState {
    /// Fully current, protected-path-safe, reduced-motion-safe, orientation-preserving,
    /// owning-surface-attached, labeled — imposes no ceiling.
    FullyQualified,
    /// The motion token's protected-path timing evidence is stale — claim drops to a
    /// motion-timing-unverified projection.
    MotionTimingEvidenceStale,
    /// The reduced-motion / power-saver / thermal clamp cannot be confirmed — claim drops to a
    /// reduced-motion-clamp-unverified projection.
    ReducedMotionSafetyUnconfirmed,
    /// The opacity scrim's orientation / contrast preservation cannot be confirmed — claim drops to a
    /// scrim-orientation-unverified projection.
    ScrimContrastUnconfirmed,
    /// The portal's owning-surface attachment cannot be confirmed — claim drops to a
    /// portal-ownership-unverified projection.
    PortalOwnershipUnconfirmed,
    /// The illustration boundary can only be partially disclosed — claim drops to an
    /// illustration-boundary-disclosed projection.
    IllustrationBoundaryDisclosedPartial,
}

impl M5VisualInteractionConditionState {
    /// Every condition state, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::FullyQualified,
        Self::MotionTimingEvidenceStale,
        Self::ReducedMotionSafetyUnconfirmed,
        Self::ScrimContrastUnconfirmed,
        Self::PortalOwnershipUnconfirmed,
        Self::IllustrationBoundaryDisclosedPartial,
    ];

    /// Returns true when the dimension is weaker than fully qualified and therefore imposes a narrowing
    /// ceiling.
    pub const fn is_weak(self) -> bool {
        !matches!(self, Self::FullyQualified)
    }

    /// Returns true when the condition reflects weakened evidence that cannot be shown as a fully trusted,
    /// stable interaction surface and must never be shown as such. A partially-disclosed illustration
    /// boundary is an honest disclosed-absence operation (a partial secondary-illustration boundary shown
    /// honestly with an inspectable note), not a truth overstatement, so it is deliberately excluded here.
    pub const fn cannot_be_shown_trusted(self) -> bool {
        matches!(
            self,
            Self::MotionTimingEvidenceStale
                | Self::ReducedMotionSafetyUnconfirmed
                | Self::ScrimContrastUnconfirmed
                | Self::PortalOwnershipUnconfirmed
        )
    }

    /// The strongest claim this condition state permits.
    pub const fn permitted_ceiling(self) -> M5VisualInteractionA11yClaim {
        match self {
            Self::FullyQualified => M5VisualInteractionA11yClaim::TrustedInteractionSurface,
            Self::MotionTimingEvidenceStale => {
                M5VisualInteractionA11yClaim::MotionTimingUnverifiedProjection
            }
            Self::ReducedMotionSafetyUnconfirmed => {
                M5VisualInteractionA11yClaim::ReducedMotionClampUnverifiedProjection
            }
            Self::ScrimContrastUnconfirmed => {
                M5VisualInteractionA11yClaim::ScrimOrientationUnverifiedProjection
            }
            Self::PortalOwnershipUnconfirmed => {
                M5VisualInteractionA11yClaim::PortalOwnershipUnverifiedProjection
            }
            Self::IllustrationBoundaryDisclosedPartial => {
                M5VisualInteractionA11yClaim::IllustrationBoundaryDisclosedProjection
            }
        }
    }

    /// The frozen downgrade trigger this condition names when its weakness binds a narrowing. Each state
    /// maps to the on-topic frozen trigger the freeze matrix already governs, so the certified reason stays
    /// byte-identical to the matrix.
    pub const fn default_trigger(self) -> M5VisualInteractionDowngradeTrigger {
        match self {
            // The fully-qualified baseline never narrows; kept for exhaustiveness.
            Self::FullyQualified => M5VisualInteractionDowngradeTrigger::ProofStale,
            Self::MotionTimingEvidenceStale => {
                M5VisualInteractionDowngradeTrigger::MotionDelayedProtectedInput
            }
            Self::ReducedMotionSafetyUnconfirmed => {
                M5VisualInteractionDowngradeTrigger::MotionMeaningLostUnderReducedMotion
            }
            Self::ScrimContrastUnconfirmed => {
                M5VisualInteractionDowngradeTrigger::ScrimErasedOrientationOrContrast
            }
            Self::PortalOwnershipUnconfirmed => {
                M5VisualInteractionDowngradeTrigger::PortalDetachedFromOwningSurface
            }
            Self::IllustrationBoundaryDisclosedPartial => {
                M5VisualInteractionDowngradeTrigger::ProofStale
            }
        }
    }

    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::FullyQualified => "fully_qualified",
            Self::MotionTimingEvidenceStale => "motion_timing_evidence_stale",
            Self::ReducedMotionSafetyUnconfirmed => "reduced_motion_safety_unconfirmed",
            Self::ScrimContrastUnconfirmed => "scrim_contrast_unconfirmed",
            Self::PortalOwnershipUnconfirmed => "portal_ownership_unconfirmed",
            Self::IllustrationBoundaryDisclosedPartial => "illustration_boundary_disclosed_partial",
        }
    }
}

/// One visual-interaction-truth dimension's observed condition on an interaction.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct VisualInteractionClaimConditionEntry {
    /// Which dimension this entry describes.
    pub dimension: M5VisualInteractionClaimDimension,
    /// The observed condition state of the dimension.
    pub state: M5VisualInteractionConditionState,
}

/// An honest claim auto-narrow block. When a visual-interaction-truth dimension weakens, the interaction's
/// claim lowers to the permitted ceiling, names the binding dimension and frozen trigger, and preserves the
/// canonical interaction identity / last-known token reference rather than silently dropping it — the
/// underlying motion / scrim / layer / icon / illustration truth is never erased opaquely.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct VisualInteractionClaimAutoNarrow {
    /// The claim the interaction is narrowed to.
    pub narrowed_to: M5VisualInteractionA11yClaim,
    /// The dimension whose weakness bound the narrowing (the one imposing the strongest ceiling
    /// constraint).
    pub binding_dimension: M5VisualInteractionClaimDimension,
    /// The frozen downgrade trigger (reused vocabulary) the narrowing names.
    pub trigger: M5VisualInteractionDowngradeTrigger,
    /// A precise, non-generic label safe to render.
    pub narrowed_label: String,
    /// The canonical interaction identity and last-known token reference are preserved rather than dropped;
    /// must hold.
    pub preserves_canonical_identity: bool,
    /// The underlying motion / scrim / layer / icon / illustration truth is preserved (never dropped) across
    /// the narrowing; must hold so motion-timing-unverified, reduced-motion-clamp-unverified,
    /// scrim-orientation-unverified, portal-ownership-unverified, and illustration-boundary-disclosed states
    /// never fail opaquely.
    pub preserves_truth_continuity: bool,
}

impl VisualInteractionClaimAutoNarrow {
    /// Whether the auto-narrow block is honest: it preserves canonical identity and motion / scrim / layer /
    /// icon / illustration truth and carries a precise, non-generic label.
    pub fn is_honest(&self) -> bool {
        self.preserves_canonical_identity
            && self.preserves_truth_continuity
            && !label_is_generic(&self.narrowed_label)
    }
}

/// Copy / export parity for an interaction's accessible fallback: the same truth must be copyable as
/// text / JSON / Markdown, and a raw payload is never the only export.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct VisualInteractionCopyExportParity {
    /// The copy / export formats offered (must include text, json, markdown).
    #[serde(default)]
    pub formats: Vec<String>,
    /// The named export fields the summary carries.
    #[serde(default)]
    pub export_fields: Vec<String>,
    /// A raw payload is never the only export; must always hold.
    pub raw_payload_only_prohibited: bool,
}

impl VisualInteractionCopyExportParity {
    /// Whether the copy / export parity is complete: text / JSON / Markdown are all offered, at least one
    /// export field is named, and a raw-payload-only export is prohibited.
    pub fn is_complete(&self) -> bool {
        self.raw_payload_only_prohibited
            && self.formats.iter().any(|f| f == "text")
            && self.formats.iter().any(|f| f == "json")
            && self.formats.iter().any(|f| f == "markdown")
            && !self.export_fields.is_empty()
    }
}

/// Per-rendering-surface narrowing disclosure.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct VisualInteractionRenderingNarrowingDisclosure {
    /// The rendering surface being narrowed.
    pub rendering_surface: M5VisualInteractionRenderingSurface,
    /// How the surface discloses its reduced interactivity.
    pub state: VisualInteractionNarrowingDisclosureState,
    /// The labels preserved across the narrowing.
    #[serde(default)]
    pub preserved_labels: Vec<String>,
    /// The interactions reduced on the narrowed surface.
    #[serde(default)]
    pub reduced_interactions: Vec<String>,
}

/// Derived qualification status for a visual-interaction accessibility row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum VisualInteractionAccessibilityStatus {
    /// Full keyboard / screen-reader / high-zoom / reduced-motion / power-thermal / CLI / export parity with
    /// no narrowing (green).
    Parity,
    /// Reduced but fully disclosed, reachable, and honestly auto-narrowed (yellow).
    NarrowedDisclosed,
    /// Strands assistive tech, needs a raw payload, over-claims trusted, or drops state silently (red).
    Stranded,
}

impl VisualInteractionAccessibilityStatus {
    /// Stable token recorded in the summary / CSV.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Parity => "parity",
            Self::NarrowedDisclosed => "narrowed_disclosed",
            Self::Stranded => "stranded",
        }
    }
}

/// Accessibility / auto-narrowing parity row for one visual-interaction family.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct VisualInteractionAccessibilityRow {
    /// Record kind; must equal [`MOTION_LAYER_ICONOGRAPHY_A11Y_ROW_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`MOTION_LAYER_ICONOGRAPHY_A11Y_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable row id.
    pub row_id: String,
    /// The frozen interaction family this row certifies.
    pub interaction_family: M5VisualInteractionFamily,
    /// Ref to the frozen canonical per-domain schema this row certifies.
    pub source_family_schema_ref: String,
    /// Opaque ref to the interaction this row represents; stays visible on every surface, so this is never
    /// empty.
    pub interaction_context_ref: String,
    /// Rendered modalities offered; a structure-heavy family must also offer a non-visual (list / textual /
    /// CLI) path.
    #[serde(default)]
    pub fallback_modalities: Vec<M5VisualInteractionFallbackModality>,
    /// The non-visual / CLI path reaches the same canonical identity, semantic role, token reference, motion
    /// profile, layer tier, and accessible fallback as the rendered interaction; must hold.
    pub reaches_canonical_truth: bool,
    /// Keyboard reach into the non-visual path.
    pub keyboard_reach: VisualInteractionNonVisualReachState,
    /// Screen-reader reach into the non-visual path.
    pub screen_reader_reach: VisualInteractionNonVisualReachState,
    /// High-zoom (reflow / magnification) legibility of the non-visual path.
    pub high_zoom_reach: VisualInteractionNonVisualReachState,
    /// Reduced-motion behavior of the non-visual path.
    pub reduced_motion_reach: VisualInteractionNonVisualReachState,
    /// Battery-saver / thermal-pressure clamp behavior of the non-visual path.
    pub power_thermal_reach: VisualInteractionNonVisualReachState,
    /// CLI / headless reach into the non-visual path.
    pub cli_reach: VisualInteractionNonVisualReachState,
    /// Whether the export-safe summary preserves interaction meaning.
    pub export_summary: VisualInteractionExportSummaryState,
    /// Ref to the export-safe summary object for this interaction.
    pub export_summary_ref: String,
    /// The copy / export parity of the accessible fallback.
    pub copy_export: VisualInteractionCopyExportParity,
    /// The full claim this family asserts when every dimension is intact.
    pub full_ready_claim: M5VisualInteractionA11yClaim,
    /// The observed condition of each modeled visual-interaction-truth dimension.
    #[serde(default)]
    pub claim_conditions: Vec<VisualInteractionClaimConditionEntry>,
    /// The honest auto-narrow block, present only when some dimension weakens below the family's full claim.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub claim_narrow: Option<VisualInteractionClaimAutoNarrow>,
    /// Whether the underlying motion / scrim / layer / icon / illustration truth is preserved on this
    /// interaction regardless of narrowing; must hold so every unverified projection never fails opaquely.
    pub truth_preserved: bool,
    /// Rendering surfaces this interaction is certified on.
    #[serde(default)]
    pub rendering_surfaces: Vec<M5VisualInteractionRenderingSurface>,
    /// Per-surface narrowing disclosures.
    #[serde(default)]
    pub narrowing_disclosures: Vec<VisualInteractionRenderingNarrowingDisclosure>,
    /// The required labels the accessible fallback preserves (reused vocabulary).
    #[serde(default)]
    pub required_labels: Vec<M5VisualInteractionRequiredLabel>,
    /// Semantic consumer surfaces this interaction is embedded in (reused vocabulary).
    #[serde(default)]
    pub consumer_surfaces: Vec<M5VisualInteractionConsumerSurface>,
    /// Source contract refs backing this row.
    #[serde(default)]
    pub source_refs: Vec<String>,
    /// ISO 8601 UTC timestamp the accessibility posture was observed.
    pub observed_at: String,
    /// Evidence packet refs backing this row.
    #[serde(default)]
    pub evidence_refs: Vec<String>,
}

impl VisualInteractionAccessibilityRow {
    /// Returns true when this family renders a dense, structured surface and must bind to a flat non-visual
    /// path.
    pub const fn is_structure_heavy(&self) -> bool {
        family_is_structure_heavy(self.interaction_family)
    }

    /// Returns true when at least one non-visual (list / textual / CLI) fallback modality is offered.
    pub fn has_non_visual_fallback(&self) -> bool {
        self.fallback_modalities.iter().any(|m| m.is_non_visual())
    }

    /// The condition state observed for one dimension, or `FullyQualified` when the row does not model that
    /// dimension.
    pub fn condition_for(
        &self,
        dimension: M5VisualInteractionClaimDimension,
    ) -> M5VisualInteractionConditionState {
        self.claim_conditions
            .iter()
            .find(|c| c.dimension == dimension)
            .map(|c| c.state)
            .unwrap_or(M5VisualInteractionConditionState::FullyQualified)
    }

    /// Whether any modeled dimension is weaker than fully qualified.
    pub fn has_weak_dimension(&self) -> bool {
        self.claim_conditions.iter().any(|c| c.state.is_weak())
    }

    /// The strongest claim permitted after applying every modeled dimension's ceiling, capped at the
    /// family's full claim.
    pub fn permitted_claim(&self) -> M5VisualInteractionA11yClaim {
        let mut permitted = self.full_ready_claim;
        for condition in &self.claim_conditions {
            let ceiling = condition.state.permitted_ceiling();
            if ceiling.capability_rank() < permitted.capability_rank() {
                permitted = ceiling;
            }
        }
        permitted
    }

    /// The condition entry imposing the strongest (lowest-rank) ceiling, if any weak dimension narrows below
    /// the family's full claim.
    pub fn binding_condition(&self) -> Option<&VisualInteractionClaimConditionEntry> {
        let mut binding: Option<(&VisualInteractionClaimConditionEntry, u8)> = None;
        for condition in &self.claim_conditions {
            if !condition.state.is_weak() {
                continue;
            }
            let ceiling = condition.state.permitted_ceiling();
            if ceiling.capability_rank() >= self.full_ready_claim.capability_rank() {
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
    pub fn binding_dimension(&self) -> Option<M5VisualInteractionClaimDimension> {
        self.binding_condition().map(|c| c.dimension)
    }

    /// The claim this interaction effectively asserts after narrowing.
    pub fn effective_claim(&self) -> M5VisualInteractionA11yClaim {
        match &self.claim_narrow {
            Some(narrow) => narrow.narrowed_to,
            None => self.full_ready_claim,
        }
    }

    /// AC / auto-narrowing honesty: a protected-path-delaying motion, an unconfirmed reduced-motion clamp, an
    /// orientation-erasing scrim, a detached portal, or a partially-disclosed illustration boundary can no
    /// longer keep an old `TrustedInteractionSurface` / `ReviewableInteractionSurface` label. The effective
    /// claim never exceeds the permitted ceiling; when a dimension narrows below the full claim, an honest
    /// narrow block is present, narrows to exactly the permitted ceiling, binds to the ceiling-imposing
    /// dimension with its frozen trigger, and preserves canonical identity and truth. When nothing narrows,
    /// no spurious narrow block is present.
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

    /// AC / trusted honesty: a protected-path-delaying / unconfirmed-reduced-motion / orientation-erasing /
    /// detached-portal state never keeps a trusted claim — meaning is never conveyed by motion, decoration,
    /// or an unlabeled symbol alone. When such a state is modeled, the effective claim must not assert
    /// `TrustedInteractionSurface`.
    pub fn trusted_honesty_holds(&self) -> bool {
        let has_unprovable_state = self
            .claim_conditions
            .iter()
            .any(|c| c.state.cannot_be_shown_trusted());
        !(has_unprovable_state && self.effective_claim().asserts_trusted_surface())
    }

    /// AC / assistive-tech reach: accessibility and export surfaces reach the same canonical truth — no
    /// keyboard / screen-reader / high-zoom / reduced-motion / power-thermal / CLI trap, a structure-heavy
    /// family offers a non-visual fallback, and the export reconstructs meaning without a raw payload.
    pub fn reaches_canonical_truth_via_at(&self) -> bool {
        self.reaches_canonical_truth
            && !self.interaction_context_ref.trim().is_empty()
            && self.keyboard_reach.never_traps()
            && self.screen_reader_reach.never_traps()
            && self.high_zoom_reach.never_traps()
            && self.reduced_motion_reach.never_traps()
            && self.power_thermal_reach.never_traps()
            && self.cli_reach.never_traps()
            && (!self.is_structure_heavy() || self.has_non_visual_fallback())
    }

    /// The export preserves the interaction meaning without leaking a raw payload.
    pub fn export_preserves_meaning(&self) -> bool {
        self.export_summary.never_requires_raw_payload()
            && !self.export_summary_ref.trim().is_empty()
            && self.copy_export.is_complete()
    }

    /// AC / no-loss: every unverified projection preserves the underlying motion / scrim / layer / icon /
    /// illustration truth. The row must assert `truth_preserved`, and any narrow block must preserve truth
    /// continuity too.
    pub fn preserves_truth_continuity(&self) -> bool {
        self.truth_preserved
            && self
                .claim_narrow
                .as_ref()
                .map(|n| n.preserves_truth_continuity)
                .unwrap_or(true)
    }

    /// Whether any axis is in a disclosed-reduction (yellow) state or the interaction carries an honest claim
    /// narrow.
    pub fn is_reduced(&self) -> bool {
        self.claim_narrow.is_some()
            || self.keyboard_reach.is_disclosed_reduction()
            || self.screen_reader_reach.is_disclosed_reduction()
            || self.high_zoom_reach.is_disclosed_reduction()
            || self.reduced_motion_reach.is_disclosed_reduction()
            || self.power_thermal_reach.is_disclosed_reduction()
            || self.cli_reach.is_disclosed_reduction()
            || self.export_summary.is_disclosed_reduction()
            || self
                .narrowing_disclosures
                .iter()
                .any(|d| d.state.is_disclosed_reduction())
    }

    /// AC / cross-surface disclosure: every narrower rendering surface discloses its reduced interactivity
    /// and keeps its labels, so product / help / release publication stay aligned on the same narrowed
    /// state.
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
        let primary = family_primary_dimension(self.interaction_family);
        self.claim_conditions.iter().any(|c| c.dimension == primary)
    }

    /// Whether every mandatory required label is preserved on the accessible fallback.
    pub fn preserves_mandatory_labels(&self) -> bool {
        M5VisualInteractionRequiredLabel::MANDATORY
            .iter()
            .all(|label| self.required_labels.contains(label))
    }

    /// Derived qualification status.
    pub fn status(&self) -> VisualInteractionAccessibilityStatus {
        if !self.claim_is_honest()
            || !self.trusted_honesty_holds()
            || !self.reaches_canonical_truth_via_at()
            || !self.export_preserves_meaning()
            || !self.preserves_truth_continuity()
            || !self.narrowing_disclosed()
            || !self.models_primary_dimension()
            || !self.preserves_mandatory_labels()
        {
            return VisualInteractionAccessibilityStatus::Stranded;
        }
        if self.is_reduced() {
            VisualInteractionAccessibilityStatus::NarrowedDisclosed
        } else {
            VisualInteractionAccessibilityStatus::Parity
        }
    }

    /// Whether the row's identity and evidence fields are complete.
    pub fn is_complete(&self) -> bool {
        self.record_kind == MOTION_LAYER_ICONOGRAPHY_A11Y_ROW_RECORD_KIND
            && self.schema_version == MOTION_LAYER_ICONOGRAPHY_A11Y_SCHEMA_VERSION
            && !self.row_id.trim().is_empty()
            && !self.source_family_schema_ref.trim().is_empty()
            && !self.interaction_context_ref.trim().is_empty()
            && !self.fallback_modalities.is_empty()
            && !self.claim_conditions.is_empty()
            && !self.observed_at.trim().is_empty()
            && !self.evidence_refs.is_empty()
            && self.evidence_refs.iter().all(|r| !r.trim().is_empty())
    }

    /// Deterministic governed chip line for this row.
    pub fn chip_tokens(&self) -> String {
        format!(
            "family={family} keyboard={keyboard} screen_reader={screen_reader} \
high_zoom={high_zoom} reduced_motion={reduced_motion} power_thermal={power_thermal} cli={cli} \
export={export} full_claim={full} effective_claim={effective} status={status}",
            family = self.interaction_family.as_str(),
            keyboard = self.keyboard_reach.as_str(),
            screen_reader = self.screen_reader_reach.as_str(),
            high_zoom = self.high_zoom_reach.as_str(),
            reduced_motion = self.reduced_motion_reach.as_str(),
            power_thermal = self.power_thermal_reach.as_str(),
            cli = self.cli_reach.as_str(),
            export = self.export_summary.as_str(),
            full = self.full_ready_claim.as_str(),
            effective = self.effective_claim().as_str(),
            status = self.status().as_str(),
        )
    }
}

/// Rolled-up summary of an M05-1154 motion-layer-iconography accessibility parity packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct VisualInteractionAccessibilitySummary {
    pub row_count: usize,
    pub family_count: usize,
    pub structure_heavy_family_count: usize,
    pub all_structure_heavy_have_non_visual_fallback: bool,
    pub all_reach_canonical_truth_via_at: bool,
    pub all_claims_honest: bool,
    pub all_trusted_honesty_holds: bool,
    pub all_export_summaries_preserve_meaning: bool,
    pub all_truth_preserved: bool,
    pub all_narrowing_disclosed: bool,
    pub green_count: usize,
    pub yellow_count: usize,
    pub red_count: usize,
    pub rendering_surface_count: usize,
    pub consumer_surface_count: usize,
}

/// Constructor input for [`VisualInteractionAccessibilityPacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VisualInteractionAccessibilityPacketInput {
    pub packet_id: String,
    pub as_of: String,
    pub matrix_ref: String,
    pub rows: Vec<VisualInteractionAccessibilityRow>,
}

/// Checked-in M05-1154 motion-layer-iconography accessibility parity packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct VisualInteractionAccessibilityPacket {
    pub schema_version: u32,
    pub record_kind: String,
    pub packet_id: String,
    pub as_of: String,
    pub matrix_ref: String,
    #[serde(default)]
    pub rows: Vec<VisualInteractionAccessibilityRow>,
    pub summary: VisualInteractionAccessibilitySummary,
}

impl VisualInteractionAccessibilityPacket {
    /// Builds a packet, stamping the record kind, schema version, and computed summary.
    pub fn new(input: VisualInteractionAccessibilityPacketInput) -> Self {
        let mut packet = Self {
            schema_version: MOTION_LAYER_ICONOGRAPHY_A11Y_SCHEMA_VERSION,
            record_kind: MOTION_LAYER_ICONOGRAPHY_A11Y_RECORD_KIND.to_owned(),
            packet_id: input.packet_id,
            as_of: input.as_of,
            matrix_ref: input.matrix_ref,
            rows: input.rows,
            summary: VisualInteractionAccessibilitySummary {
                row_count: 0,
                family_count: 0,
                structure_heavy_family_count: 0,
                all_structure_heavy_have_non_visual_fallback: false,
                all_reach_canonical_truth_via_at: false,
                all_claims_honest: false,
                all_trusted_honesty_holds: false,
                all_export_summaries_preserve_meaning: false,
                all_truth_preserved: false,
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
    pub fn represented_families(&self) -> BTreeSet<M5VisualInteractionFamily> {
        self.rows.iter().map(|r| r.interaction_family).collect()
    }

    /// Dimensions exercised by some row's claim conditions.
    pub fn exercised_dimensions(&self) -> BTreeSet<M5VisualInteractionClaimDimension> {
        self.rows
            .iter()
            .flat_map(|r| r.claim_conditions.iter().map(|c| c.dimension))
            .collect()
    }

    /// Condition states exercised by some row's claim conditions.
    pub fn exercised_condition_states(&self) -> BTreeSet<M5VisualInteractionConditionState> {
        self.rows
            .iter()
            .flat_map(|r| r.claim_conditions.iter().map(|c| c.state))
            .collect()
    }

    /// Claim tiers that appear as an effective claim across the rows.
    pub fn represented_effective_claims(&self) -> BTreeSet<M5VisualInteractionA11yClaim> {
        self.rows.iter().map(|r| r.effective_claim()).collect()
    }

    /// Consumer surfaces ingesting some row in this packet.
    pub fn represented_consumer_surfaces(&self) -> BTreeSet<M5VisualInteractionConsumerSurface> {
        self.rows
            .iter()
            .flat_map(|r| r.consumer_surfaces.iter().copied())
            .collect()
    }

    /// Computes summary fields from the packet contents.
    pub fn computed_summary(&self) -> VisualInteractionAccessibilitySummary {
        let mut rendering = BTreeSet::new();
        let mut consumers: BTreeSet<M5VisualInteractionConsumerSurface> = BTreeSet::new();
        for row in &self.rows {
            rendering.extend(row.rendering_surfaces.iter().copied());
            consumers.extend(row.consumer_surfaces.iter().copied());
        }

        let structure_heavy: Vec<&VisualInteractionAccessibilityRow> = self
            .rows
            .iter()
            .filter(|row| row.is_structure_heavy())
            .collect();

        let mut green = 0;
        let mut yellow = 0;
        let mut red = 0;
        for row in &self.rows {
            match row.status() {
                VisualInteractionAccessibilityStatus::Parity => green += 1,
                VisualInteractionAccessibilityStatus::NarrowedDisclosed => yellow += 1,
                VisualInteractionAccessibilityStatus::Stranded => red += 1,
            }
        }

        VisualInteractionAccessibilitySummary {
            row_count: self.rows.len(),
            family_count: self.represented_families().len(),
            structure_heavy_family_count: structure_heavy.len(),
            all_structure_heavy_have_non_visual_fallback: structure_heavy
                .iter()
                .all(|row| row.has_non_visual_fallback()),
            all_reach_canonical_truth_via_at: self
                .rows
                .iter()
                .all(VisualInteractionAccessibilityRow::reaches_canonical_truth_via_at),
            all_claims_honest: self
                .rows
                .iter()
                .all(VisualInteractionAccessibilityRow::claim_is_honest),
            all_trusted_honesty_holds: self
                .rows
                .iter()
                .all(VisualInteractionAccessibilityRow::trusted_honesty_holds),
            all_export_summaries_preserve_meaning: self
                .rows
                .iter()
                .all(VisualInteractionAccessibilityRow::export_preserves_meaning),
            all_truth_preserved: self
                .rows
                .iter()
                .all(VisualInteractionAccessibilityRow::preserves_truth_continuity),
            all_narrowing_disclosed: self
                .rows
                .iter()
                .all(VisualInteractionAccessibilityRow::narrowing_disclosed),
            green_count: green,
            yellow_count: yellow,
            red_count: red,
            rendering_surface_count: rendering.len(),
            consumer_surface_count: consumers.len(),
        }
    }

    /// Validates the packet and returns every contract violation.
    pub fn validate(&self) -> Vec<VisualInteractionAccessibilityViolation> {
        let mut violations = Vec::new();

        if self.schema_version != MOTION_LAYER_ICONOGRAPHY_A11Y_SCHEMA_VERSION {
            violations.push(VisualInteractionAccessibilityViolation::SchemaVersion {
                expected: MOTION_LAYER_ICONOGRAPHY_A11Y_SCHEMA_VERSION,
                actual: self.schema_version,
            });
        }
        if self.record_kind != MOTION_LAYER_ICONOGRAPHY_A11Y_RECORD_KIND {
            violations.push(VisualInteractionAccessibilityViolation::RecordKind {
                expected: MOTION_LAYER_ICONOGRAPHY_A11Y_RECORD_KIND.to_owned(),
                actual: self.record_kind.clone(),
            });
        }
        if self.packet_id.trim().is_empty()
            || self.as_of.trim().is_empty()
            || self.matrix_ref.trim().is_empty()
        {
            violations.push(VisualInteractionAccessibilityViolation::MissingIdentity);
        }

        let mut row_ids = BTreeSet::new();
        let mut seen_families = BTreeSet::new();
        let mut has_unprovable_row = false;
        for row in &self.rows {
            if !row_ids.insert(row.row_id.clone()) {
                violations.push(VisualInteractionAccessibilityViolation::DuplicateId {
                    id: row.row_id.clone(),
                });
            }
            seen_families.insert(row.interaction_family);
            if row
                .claim_conditions
                .iter()
                .any(|c| c.state.cannot_be_shown_trusted())
            {
                has_unprovable_row = true;
            }

            if !row.is_complete() {
                violations.push(VisualInteractionAccessibilityViolation::IncompleteRow {
                    id: row.row_id.clone(),
                });
            }

            // Each row must model its family's primary weakening dimension.
            if !row.models_primary_dimension() {
                violations.push(
                    VisualInteractionAccessibilityViolation::MissingPrimaryDimension {
                        id: row.row_id.clone(),
                        dimension: family_primary_dimension(row.interaction_family),
                    },
                );
            }

            // Each row must preserve every mandatory interaction label.
            if !row.preserves_mandatory_labels() {
                violations.push(
                    VisualInteractionAccessibilityViolation::MissingMandatoryLabel {
                        id: row.row_id.clone(),
                    },
                );
            }

            // A structure-heavy family must render a structured projection *and* a non-visual path.
            if row.is_structure_heavy()
                && !row
                    .fallback_modalities
                    .contains(&M5VisualInteractionFallbackModality::Structured)
            {
                violations.push(
                    VisualInteractionAccessibilityViolation::StructureHeavyMissingStructured {
                        id: row.row_id.clone(),
                    },
                );
            }

            // AC: claim never over-asserts a trusted / reviewable surface for a weakened one.
            if !row.claim_is_honest() {
                violations.push(VisualInteractionAccessibilityViolation::ClaimOverAsserted {
                    id: row.row_id.clone(),
                });
            }

            // AC / trusted honesty: a protected-path-delaying / unconfirmed-reduced-motion /
            // orientation-erasing / detached-portal state never keeps a trusted claim.
            if !row.trusted_honesty_holds() {
                violations.push(
                    VisualInteractionAccessibilityViolation::WeakStateShownAsTrusted {
                        id: row.row_id.clone(),
                    },
                );
            }

            // AC: assistive-tech / CLI reach the same canonical truth.
            if !row.reaches_canonical_truth_via_at() {
                violations.push(
                    VisualInteractionAccessibilityViolation::AssistiveTechStranded {
                        id: row.row_id.clone(),
                    },
                );
            }

            // AC: export preserves meaning without leaking a raw payload.
            if !row.export_preserves_meaning() {
                violations.push(
                    VisualInteractionAccessibilityViolation::ExportRequiresRawPayload {
                        id: row.row_id.clone(),
                    },
                );
            }

            // AC / no-loss: weakened states preserve motion / scrim / layer / icon / illustration truth.
            if !row.preserves_truth_continuity() {
                violations.push(VisualInteractionAccessibilityViolation::TruthDropped {
                    id: row.row_id.clone(),
                });
            }

            // Narrowing disclosed on every narrowed rendering surface.
            if !row.narrowing_disclosed() {
                violations.push(
                    VisualInteractionAccessibilityViolation::NarrowingDropsContextSilently {
                        id: row.row_id.clone(),
                    },
                );
            }

            // Consumer parity: at least two consumer surfaces ingest the row.
            if row.consumer_surfaces.len() < 2 {
                violations.push(
                    VisualInteractionAccessibilityViolation::MissingConsumerParity {
                        id: row.row_id.clone(),
                    },
                );
            }

            // No red rows may ship.
            if row.status() == VisualInteractionAccessibilityStatus::Stranded {
                violations.push(VisualInteractionAccessibilityViolation::StrandedRow {
                    id: row.row_id.clone(),
                });
            }
        }

        // Coverage: every frozen family is certified at least once.
        for family in M5VisualInteractionFamily::ALL {
            if !seen_families.contains(&family) {
                violations.push(
                    VisualInteractionAccessibilityViolation::MissingFamilyCoverage { family },
                );
            }
        }

        // Coverage: every weakening dimension is exercised somewhere.
        let exercised = self.exercised_dimensions();
        for dimension in M5VisualInteractionClaimDimension::ALL {
            if !exercised.contains(&dimension) {
                violations.push(
                    VisualInteractionAccessibilityViolation::MissingDimensionCoverage { dimension },
                );
            }
        }

        // Coverage: every condition state (the fully-qualified baseline plus each spec narrowing axis) is
        // exercised somewhere, so the full narrowing spectrum is proven end-to-end.
        let states = self.exercised_condition_states();
        for state in M5VisualInteractionConditionState::ALL {
            if !states.contains(&state) {
                violations.push(
                    VisualInteractionAccessibilityViolation::MissingConditionStateCoverage {
                        state,
                    },
                );
            }
        }

        // Coverage: every claim tier appears as an effective claim, so the full narrowing spectrum
        // (trusted → … → illustration-boundary-disclosed) is proven end-to-end.
        let effective = self.represented_effective_claims();
        for claim in M5VisualInteractionA11yClaim::ALL {
            if !effective.contains(&claim) {
                violations.push(
                    VisualInteractionAccessibilityViolation::MissingClaimTierCoverage { claim },
                );
            }
        }

        // Trusted honesty must be proven with at least one protected-path-delaying / unconfirmed-reduced-motion
        // / orientation-erasing / detached-portal row in the packet, so the "cannot-prove never shown as
        // trusted" guarantee is exercised end-to-end.
        if !has_unprovable_row {
            violations.push(VisualInteractionAccessibilityViolation::TrustedHonestyUnproven);
        }

        // Cross-surface: the same narrowed state must reach the shell, editor, help, marketplace, onboarding,
        // settings, CLI-export, support-export, and product surfaces — so every consumer surface is exercised
        // at least once across the packet.
        let consumers = self.represented_consumer_surfaces();
        for surface in M5VisualInteractionConsumerSurface::ALL {
            if !consumers.contains(&surface) {
                violations.push(
                    VisualInteractionAccessibilityViolation::MissingConsumerSurfaceCoverage {
                        surface,
                    },
                );
            }
        }

        if self.summary != self.computed_summary() {
            violations.push(VisualInteractionAccessibilityViolation::SummaryMismatch);
        }

        if json_contains_forbidden_material(
            &serde_json::to_value(self)
                .expect("motion-layer-iconography accessibility parity packet serializes"),
        ) {
            violations
                .push(VisualInteractionAccessibilityViolation::RawInteractionMaterialInExport);
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
            .expect("motion-layer-iconography accessibility parity packet serializes")
    }

    /// Deterministic CSV of the certified rows for support / release handoff.
    pub fn render_matrix_csv(&self) -> String {
        let mut out = String::from(
            "row_id,interaction_family,keyboard_reach,screen_reader_reach,high_zoom_reach,reduced_motion_reach,power_thermal_reach,cli_reach,export_summary,full_claim,effective_claim,status\n",
        );
        for row in &self.rows {
            out.push_str(&format!(
                "{id},{family},{keyboard},{screen_reader},{high_zoom},{reduced_motion},{power_thermal},{cli},{export},{full},{effective},{status}\n",
                id = row.row_id,
                family = row.interaction_family.as_str(),
                keyboard = row.keyboard_reach.as_str(),
                screen_reader = row.screen_reader_reach.as_str(),
                high_zoom = row.high_zoom_reach.as_str(),
                reduced_motion = row.reduced_motion_reach.as_str(),
                power_thermal = row.power_thermal_reach.as_str(),
                cli = row.cli_reach.as_str(),
                export = row.export_summary.as_str(),
                full = row.full_ready_claim.as_str(),
                effective = row.effective_claim().as_str(),
                status = row.status().as_str(),
            ));
        }
        out
    }

    /// Deterministic Markdown report for support, help, or release handoff.
    pub fn render_markdown_summary(&self) -> String {
        let mut out = String::new();
        out.push_str("# M5 Motion-Layer-Iconography Accessibility & Auto-Narrowing\n\n");
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- As of: `{}`\n", self.as_of));
        out.push_str(&format!(
            "- Families: {} certified across {} / {} frozen families\n",
            self.summary.family_count,
            self.represented_families().len(),
            M5VisualInteractionFamily::ALL.len(),
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
                row.interaction_family.as_str(),
                row.chip_tokens(),
            ));
            if let Some(narrow) = &row.claim_narrow {
                out.push_str(&format!(
                    "  - Auto-narrow: {} → {} (dimension={}, trigger={}) — {}\n",
                    row.full_ready_claim.as_str(),
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

/// Reads and validates the checked-in motion-layer-iconography accessibility parity export.
pub fn current_m5_motion_layer_iconography_a11y_export(
) -> Result<VisualInteractionAccessibilityPacket, VisualInteractionAccessibilityArtifactError> {
    let packet: VisualInteractionAccessibilityPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/release/m5-motion-layer-iconography-accessibility-parity/support_export.json"
    )))
    .map_err(VisualInteractionAccessibilityArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(VisualInteractionAccessibilityArtifactError::Validation(
            violations,
        ))
    }
}

/// Errors emitted when reading the checked-in motion-layer-iconography accessibility parity export.
#[derive(Debug)]
pub enum VisualInteractionAccessibilityArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<VisualInteractionAccessibilityViolation>),
}

impl fmt::Display for VisualInteractionAccessibilityArtifactError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(
                    f,
                    "motion-layer-iconography accessibility parity export parse failed: {error}"
                )
            }
            Self::Validation(violations) => {
                write!(
                    f,
                    "motion-layer-iconography accessibility parity export failed validation: {} violation(s)",
                    violations.len()
                )
            }
        }
    }
}

impl Error for VisualInteractionAccessibilityArtifactError {}

/// Validation failure for M05-1154 motion-layer-iconography accessibility parity packets.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum VisualInteractionAccessibilityViolation {
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
        dimension: M5VisualInteractionClaimDimension,
    },
    MissingMandatoryLabel {
        id: String,
    },
    StructureHeavyMissingStructured {
        id: String,
    },
    ClaimOverAsserted {
        id: String,
    },
    WeakStateShownAsTrusted {
        id: String,
    },
    AssistiveTechStranded {
        id: String,
    },
    ExportRequiresRawPayload {
        id: String,
    },
    TruthDropped {
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
        family: M5VisualInteractionFamily,
    },
    MissingDimensionCoverage {
        dimension: M5VisualInteractionClaimDimension,
    },
    MissingConditionStateCoverage {
        state: M5VisualInteractionConditionState,
    },
    MissingClaimTierCoverage {
        claim: M5VisualInteractionA11yClaim,
    },
    TrustedHonestyUnproven,
    MissingConsumerSurfaceCoverage {
        surface: M5VisualInteractionConsumerSurface,
    },
    SummaryMismatch,
    RawInteractionMaterialInExport,
}

impl VisualInteractionAccessibilityViolation {
    /// Stable token for CLI / support handoff.
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::SchemaVersion { .. } => "schema_version",
            Self::RecordKind { .. } => "record_kind",
            Self::MissingIdentity => "missing_identity",
            Self::DuplicateId { .. } => "duplicate_id",
            Self::IncompleteRow { .. } => "incomplete_row",
            Self::MissingPrimaryDimension { .. } => "missing_primary_dimension",
            Self::MissingMandatoryLabel { .. } => "missing_mandatory_label",
            Self::StructureHeavyMissingStructured { .. } => "structure_heavy_missing_structured",
            Self::ClaimOverAsserted { .. } => "claim_over_asserted",
            Self::WeakStateShownAsTrusted { .. } => "weak_state_shown_as_trusted",
            Self::AssistiveTechStranded { .. } => "assistive_tech_stranded",
            Self::ExportRequiresRawPayload { .. } => "export_requires_raw_payload",
            Self::TruthDropped { .. } => "truth_dropped",
            Self::NarrowingDropsContextSilently { .. } => "narrowing_drops_context_silently",
            Self::MissingConsumerParity { .. } => "missing_consumer_parity",
            Self::StrandedRow { .. } => "stranded_row",
            Self::MissingFamilyCoverage { .. } => "missing_family_coverage",
            Self::MissingDimensionCoverage { .. } => "missing_dimension_coverage",
            Self::MissingConditionStateCoverage { .. } => "missing_condition_state_coverage",
            Self::MissingClaimTierCoverage { .. } => "missing_claim_tier_coverage",
            Self::TrustedHonestyUnproven => "trusted_honesty_unproven",
            Self::MissingConsumerSurfaceCoverage { .. } => "missing_consumer_surface_coverage",
            Self::SummaryMismatch => "summary_mismatch",
            Self::RawInteractionMaterialInExport => "raw_interaction_material_in_export",
        }
    }
}

impl fmt::Display for VisualInteractionAccessibilityViolation {
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
                write!(f, "row {id} drops a mandatory interaction label")
            }
            Self::StructureHeavyMissingStructured { id } => {
                write!(
                    f,
                    "structure-heavy row {id} does not render a structured modality"
                )
            }
            Self::ClaimOverAsserted { id } => {
                write!(
                    f,
                    "row {id} over-asserts a trusted / reviewable surface for a weakened one, or narrows spuriously"
                )
            }
            Self::WeakStateShownAsTrusted { id } => {
                write!(
                    f,
                    "row {id} shows a protected-path-delaying / unconfirmed-reduced-motion / orientation-erasing / detached-portal state as a trusted interaction surface"
                )
            }
            Self::AssistiveTechStranded { id } => {
                write!(
                    f,
                    "row {id} strands keyboard / assistive-tech / high-zoom / reduced-motion / power-thermal / CLI users from the canonical truth"
                )
            }
            Self::ExportRequiresRawPayload { id } => {
                write!(
                    f,
                    "row {id} export cannot preserve meaning without leaking a raw payload"
                )
            }
            Self::TruthDropped { id } => {
                write!(
                    f,
                    "row {id} does not preserve motion / scrim / layer / icon / illustration truth across narrowing"
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
                    "interaction family {family:?} is not certified in the packet"
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
                    "claim tier {} does not appear as an effective claim",
                    claim.as_str()
                )
            }
            Self::TrustedHonestyUnproven => {
                write!(
                    f,
                    "no protected-path-delaying / unconfirmed-reduced-motion / orientation-erasing / detached-portal row is present to prove the trusted-honesty guarantee"
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
            Self::RawInteractionMaterialInExport => {
                write!(f, "export contains raw interaction material")
            }
        }
    }
}

impl Error for VisualInteractionAccessibilityViolation {}

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
            | "partial"
            | "stale"
            | "incomplete"
            | "not comparable"
            | "restricted"
            | "collapsed"
            | "ellipsis"
            | "mixed"
            | "expired"
            | "inferred"
            | "unverified"
            | "trusted"
    )
}

/// Heuristic that rejects obviously forbidden material in export-safe JSON.
fn json_contains_forbidden_material(value: &serde_json::Value) -> bool {
    match value {
        serde_json::Value::String(s) => {
            let lower = s.to_lowercase();
            lower.contains("api_key")
                || lower.contains("password")
                || lower.contains("passphrase")
                || lower.contains("secret")
                || lower.contains("-----begin")
                || lower.contains("bearer ")
        }
        serde_json::Value::Array(arr) => arr.iter().any(json_contains_forbidden_material),
        serde_json::Value::Object(map) => map.values().any(json_contains_forbidden_material),
        _ => false,
    }
}

/// The canonical packet id for the checked-in stable export.
pub const MOTION_LAYER_ICONOGRAPHY_A11Y_PACKET_ID: &str =
    "m5-motion-layer-iconography-accessibility-parity:stable:0001";

/// Builds the canonical, checked-in motion-layer-iconography accessibility parity packet. This is the one
/// source of truth shared by the tests and the on-disk support export so both stay byte-aligned.
pub fn seeded_m5_motion_layer_iconography_a11y_packet() -> VisualInteractionAccessibilityPacket {
    VisualInteractionAccessibilityPacket::new(VisualInteractionAccessibilityPacketInput {
        packet_id: MOTION_LAYER_ICONOGRAPHY_A11Y_PACKET_ID.to_owned(),
        as_of: "2026-07-13T00:00:00Z".to_owned(),
        matrix_ref: MOTION_LAYER_ICONOGRAPHY_A11Y_MATRIX_REF.to_owned(),
        rows: seeded_rows(),
    })
}

fn ev(id: &str) -> Vec<String> {
    vec![format!("evidence:motion-layer-iconography-a11y:{id}")]
}

fn all_required_labels() -> Vec<M5VisualInteractionRequiredLabel> {
    M5VisualInteractionRequiredLabel::ALL.to_vec()
}

fn copy_export(fields: &[&str]) -> VisualInteractionCopyExportParity {
    VisualInteractionCopyExportParity {
        formats: vec!["text".to_owned(), "json".to_owned(), "markdown".to_owned()],
        export_fields: fields.iter().map(|f| (*f).to_owned()).collect(),
        raw_payload_only_prohibited: true,
    }
}

fn condition(
    dimension: M5VisualInteractionClaimDimension,
    state: M5VisualInteractionConditionState,
) -> VisualInteractionClaimConditionEntry {
    VisualInteractionClaimConditionEntry { dimension, state }
}

/// The two consumer surfaces every row ships to at minimum — support / release export and the general
/// product UI — so the narrowed state always reaches headless field triage.
fn base_consumers(
    extra: &[M5VisualInteractionConsumerSurface],
) -> Vec<M5VisualInteractionConsumerSurface> {
    let mut out = vec![
        M5VisualInteractionConsumerSurface::SupportExport,
        M5VisualInteractionConsumerSurface::ProductUi,
    ];
    out.extend_from_slice(extra);
    out
}

/// Disclosures for the CLI-headless and support-export surfaces. A green (full parity) row keeps full label
/// and summary parity on the narrower surfaces; a narrowed row discloses the reduced interactions it drops
/// there.
fn surface_disclosures(
    labels: &[&str],
    state: VisualInteractionNarrowingDisclosureState,
) -> Vec<VisualInteractionRenderingNarrowingDisclosure> {
    let preserved: Vec<String> = labels.iter().map(|l| (*l).to_owned()).collect();
    vec![
        VisualInteractionRenderingNarrowingDisclosure {
            rendering_surface: M5VisualInteractionRenderingSurface::CliHeadless,
            state,
            preserved_labels: preserved.clone(),
            reduced_interactions: vec!["pointer_interaction".to_owned()],
        },
        VisualInteractionRenderingNarrowingDisclosure {
            rendering_surface: M5VisualInteractionRenderingSurface::SupportExport,
            state,
            preserved_labels: preserved,
            reduced_interactions: vec!["live_hover_affordance".to_owned()],
        },
    ]
}

/// Disclosures for a full-parity (green) row: the narrower surfaces preserve full label and summary parity.
fn parity_surfaces(labels: &[&str]) -> Vec<VisualInteractionRenderingNarrowingDisclosure> {
    surface_disclosures(
        labels,
        VisualInteractionNarrowingDisclosureState::ParityPreserved,
    )
}

/// Disclosures for a narrowed (yellow) row: the narrower surfaces disclose their reduced interactions while
/// preserving labels.
fn narrowed_surfaces(labels: &[&str]) -> Vec<VisualInteractionRenderingNarrowingDisclosure> {
    surface_disclosures(
        labels,
        VisualInteractionNarrowingDisclosureState::DisclosedNarrowed,
    )
}

fn rendering_surfaces() -> Vec<M5VisualInteractionRenderingSurface> {
    vec![
        M5VisualInteractionRenderingSurface::DesktopFull,
        M5VisualInteractionRenderingSurface::CliHeadless,
        M5VisualInteractionRenderingSurface::SupportExport,
    ]
}

fn non_visual_modalities() -> Vec<M5VisualInteractionFallbackModality> {
    vec![
        M5VisualInteractionFallbackModality::List,
        M5VisualInteractionFallbackModality::Textual,
        M5VisualInteractionFallbackModality::Cli,
    ]
}

fn structured_modalities() -> Vec<M5VisualInteractionFallbackModality> {
    vec![
        M5VisualInteractionFallbackModality::Structured,
        M5VisualInteractionFallbackModality::List,
        M5VisualInteractionFallbackModality::Textual,
        M5VisualInteractionFallbackModality::Cli,
    ]
}

const REACHABLE: VisualInteractionNonVisualReachState =
    VisualInteractionNonVisualReachState::ReachableAndLabeled;
const REDUCED: VisualInteractionNonVisualReachState =
    VisualInteractionNonVisualReachState::DisclosedReducedButReachable;

fn seeded_rows() -> Vec<VisualInteractionAccessibilityRow> {
    vec![
        // Iconography (semantic, labeled icons) — the iconography family keeps every action icon semantic and
        // labeled with a tooltip / accessible label, so it is a trusted interaction surface reachable on
        // every surface with no narrowing (green). Structure-heavy: its icon-class registry binds to a flat
        // list / textual path.
        VisualInteractionAccessibilityRow {
            record_kind: MOTION_LAYER_ICONOGRAPHY_A11Y_ROW_RECORD_KIND.to_owned(),
            schema_version: MOTION_LAYER_ICONOGRAPHY_A11Y_SCHEMA_VERSION,
            row_id: "a11y:iconography-semantic-labeled-icons".to_owned(),
            interaction_family: M5VisualInteractionFamily::Iconography,
            source_family_schema_ref: M5VisualInteractionFamily::Iconography
                .canonical_domain_schema_ref()
                .to_owned(),
            interaction_context_ref: "editor:iconography:0001".to_owned(),
            fallback_modalities: structured_modalities(),
            reaches_canonical_truth: true,
            keyboard_reach: REACHABLE,
            screen_reader_reach: REACHABLE,
            high_zoom_reach: REACHABLE,
            reduced_motion_reach: REACHABLE,
            power_thermal_reach: REACHABLE,
            cli_reach: REACHABLE,
            export_summary: VisualInteractionExportSummaryState::ReconstructableWithoutRawPayload,
            export_summary_ref: "summary:iconography-semantic-labeled-icons:a11y".to_owned(),
            copy_export: copy_export(&[
                "interaction_identity",
                "semantic_role",
                "token_reference",
                "accessible_label",
            ]),
            full_ready_claim: M5VisualInteractionA11yClaim::TrustedInteractionSurface,
            claim_conditions: vec![condition(
                M5VisualInteractionClaimDimension::IconSemanticsClarity,
                M5VisualInteractionConditionState::FullyQualified,
            )],
            claim_narrow: None,
            truth_preserved: true,
            rendering_surfaces: rendering_surfaces(),
            narrowing_disclosures: parity_surfaces(&[
                "interaction_identity",
                "semantic_role",
                "token_reference",
            ]),
            required_labels: all_required_labels(),
            consumer_surfaces: base_consumers(&[
                M5VisualInteractionConsumerSurface::EditorUi,
                M5VisualInteractionConsumerSurface::HelpUi,
            ]),
            source_refs: vec![
                "UX Style Guide §11 — Iconography guidance".to_owned(),
                MOTION_LAYER_ICONOGRAPHY_A11Y_DOC_REF.to_owned(),
            ],
            observed_at: "2026-07-13T00:00:00Z".to_owned(),
            evidence_refs: ev("iconography-semantic-labeled-icons"),
        },
        // Layer order (one shared z-order model) — the layer follows the one shared z-order model rather than
        // an ad-hoc always-on-top bypass, so it is a self-sufficient reviewable interaction surface a user can
        // inspect, but its narrower non-visual traversal discloses a reduced high-zoom reflow walk (yellow).
        // Structure-heavy: its z-tier stack binds to a flat list / textual path.
        VisualInteractionAccessibilityRow {
            record_kind: MOTION_LAYER_ICONOGRAPHY_A11Y_ROW_RECORD_KIND.to_owned(),
            schema_version: MOTION_LAYER_ICONOGRAPHY_A11Y_SCHEMA_VERSION,
            row_id: "a11y:layer-order-shared-z-order-model-stable".to_owned(),
            interaction_family: M5VisualInteractionFamily::LayerOrder,
            source_family_schema_ref: M5VisualInteractionFamily::LayerOrder
                .canonical_domain_schema_ref()
                .to_owned(),
            interaction_context_ref: "shell:layer-order:0002".to_owned(),
            fallback_modalities: structured_modalities(),
            reaches_canonical_truth: true,
            keyboard_reach: REACHABLE,
            screen_reader_reach: REACHABLE,
            high_zoom_reach: REDUCED,
            reduced_motion_reach: REACHABLE,
            power_thermal_reach: REACHABLE,
            cli_reach: REACHABLE,
            export_summary: VisualInteractionExportSummaryState::ReconstructableWithoutRawPayload,
            export_summary_ref: "summary:layer-order-shared-z-order-model-stable:a11y".to_owned(),
            copy_export: copy_export(&[
                "interaction_identity",
                "semantic_role",
                "token_reference",
                "layer_tier",
            ]),
            full_ready_claim: M5VisualInteractionA11yClaim::ReviewableInteractionSurface,
            claim_conditions: vec![condition(
                M5VisualInteractionClaimDimension::LayerOrderClarity,
                M5VisualInteractionConditionState::FullyQualified,
            )],
            claim_narrow: None,
            truth_preserved: true,
            rendering_surfaces: rendering_surfaces(),
            narrowing_disclosures: narrowed_surfaces(&[
                "interaction_identity",
                "semantic_role",
                "layer_tier",
            ]),
            required_labels: all_required_labels(),
            consumer_surfaces: base_consumers(&[
                M5VisualInteractionConsumerSurface::ShellUi,
                M5VisualInteractionConsumerSurface::MarketplaceUi,
            ]),
            source_refs: vec![
                "UI/UX Spec §5.4 — Shell overlay / layering".to_owned(),
                MOTION_LAYER_ICONOGRAPHY_A11Y_DOC_REF.to_owned(),
            ],
            observed_at: "2026-07-13T00:00:00Z".to_owned(),
            evidence_refs: ev("layer-order-shared-z-order-model-stable"),
        },
        // Motion token (protected-path timing evidence stale) — the motion token's protected-path timing
        // evidence is stale, so it auto-narrows to a motion-timing-unverified projection that keeps the
        // last-known semantic role and static fallback visible, never a fresh, protected-path-delaying motion
        // shown as authoritative (yellow). Its motion under battery-saver / thermal pressure narrows the
        // power-thermal path to a disclosed reduction.
        VisualInteractionAccessibilityRow {
            record_kind: MOTION_LAYER_ICONOGRAPHY_A11Y_ROW_RECORD_KIND.to_owned(),
            schema_version: MOTION_LAYER_ICONOGRAPHY_A11Y_SCHEMA_VERSION,
            row_id: "a11y:motion-token-protected-path-timing-stale".to_owned(),
            interaction_family: M5VisualInteractionFamily::MotionToken,
            source_family_schema_ref: M5VisualInteractionFamily::MotionToken
                .canonical_domain_schema_ref()
                .to_owned(),
            interaction_context_ref: "shell:motion-token:0003".to_owned(),
            fallback_modalities: non_visual_modalities(),
            reaches_canonical_truth: true,
            keyboard_reach: REACHABLE,
            screen_reader_reach: REACHABLE,
            high_zoom_reach: REACHABLE,
            reduced_motion_reach: REACHABLE,
            power_thermal_reach: REDUCED,
            cli_reach: REACHABLE,
            export_summary: VisualInteractionExportSummaryState::ReconstructableWithoutRawPayload,
            export_summary_ref: "summary:motion-token-protected-path-timing-stale:a11y".to_owned(),
            copy_export: copy_export(&[
                "interaction_identity",
                "semantic_role",
                "motion_profile",
                "last_known_static_fallback",
            ]),
            full_ready_claim: M5VisualInteractionA11yClaim::TrustedInteractionSurface,
            claim_conditions: vec![condition(
                M5VisualInteractionClaimDimension::MotionTimingClarity,
                M5VisualInteractionConditionState::MotionTimingEvidenceStale,
            )],
            claim_narrow: Some(VisualInteractionClaimAutoNarrow {
                narrowed_to: M5VisualInteractionA11yClaim::MotionTimingUnverifiedProjection,
                binding_dimension: M5VisualInteractionClaimDimension::MotionTimingClarity,
                trigger: M5VisualInteractionDowngradeTrigger::MotionDelayedProtectedInput,
                narrowed_label:
                    "This motion token's protected-path timing evidence is stale or unresolved — shown as a motion-timing-unverified projection that keeps the last-known semantic role and its static fallback visible, never presenting a stale motion curve as a protected-path-safe, authoritative animation"
                        .to_owned(),
                preserves_canonical_identity: true,
                preserves_truth_continuity: true,
            }),
            truth_preserved: true,
            rendering_surfaces: rendering_surfaces(),
            narrowing_disclosures: narrowed_surfaces(&[
                "interaction_identity",
                "semantic_role",
                "motion_profile",
            ]),
            required_labels: all_required_labels(),
            consumer_surfaces: base_consumers(&[
                M5VisualInteractionConsumerSurface::ShellUi,
                M5VisualInteractionConsumerSurface::OnboardingUi,
            ]),
            source_refs: vec![
                "UX Style Guide §9.6 — Motion tokens".to_owned(),
                MOTION_LAYER_ICONOGRAPHY_A11Y_DOC_REF.to_owned(),
            ],
            observed_at: "2026-07-13T00:00:00Z".to_owned(),
            evidence_refs: ev("motion-token-protected-path-timing-stale"),
        },
        // Reduced motion (reduced-motion / power-saver / thermal clamp unconfirmed) — the clamp's static
        // fallback equivalence cannot be confirmed, so it auto-narrows to a reduced-motion-clamp-unverified
        // projection that keeps the last-known static fallback explicit, never a motion-only cue shown as
        // clamp-safe (yellow). Its animated affordance narrows the reduced-motion path to a disclosed
        // reduction.
        VisualInteractionAccessibilityRow {
            record_kind: MOTION_LAYER_ICONOGRAPHY_A11Y_ROW_RECORD_KIND.to_owned(),
            schema_version: MOTION_LAYER_ICONOGRAPHY_A11Y_SCHEMA_VERSION,
            row_id: "a11y:reduced-motion-clamp-unconfirmed".to_owned(),
            interaction_family: M5VisualInteractionFamily::ReducedMotion,
            source_family_schema_ref: M5VisualInteractionFamily::ReducedMotion
                .canonical_domain_schema_ref()
                .to_owned(),
            interaction_context_ref: "shell:reduced-motion:0004".to_owned(),
            fallback_modalities: non_visual_modalities(),
            reaches_canonical_truth: true,
            keyboard_reach: REACHABLE,
            screen_reader_reach: REACHABLE,
            high_zoom_reach: REACHABLE,
            reduced_motion_reach: REDUCED,
            power_thermal_reach: REACHABLE,
            cli_reach: REACHABLE,
            export_summary: VisualInteractionExportSummaryState::ReconstructableWithoutRawPayload,
            export_summary_ref: "summary:reduced-motion-clamp-unconfirmed:a11y".to_owned(),
            copy_export: copy_export(&[
                "interaction_identity",
                "semantic_role",
                "motion_profile",
                "last_known_static_fallback",
            ]),
            full_ready_claim: M5VisualInteractionA11yClaim::TrustedInteractionSurface,
            claim_conditions: vec![condition(
                M5VisualInteractionClaimDimension::ReducedMotionSafetyClarity,
                M5VisualInteractionConditionState::ReducedMotionSafetyUnconfirmed,
            )],
            claim_narrow: Some(VisualInteractionClaimAutoNarrow {
                narrowed_to: M5VisualInteractionA11yClaim::ReducedMotionClampUnverifiedProjection,
                binding_dimension: M5VisualInteractionClaimDimension::ReducedMotionSafetyClarity,
                trigger: M5VisualInteractionDowngradeTrigger::MotionMeaningLostUnderReducedMotion,
                narrowed_label:
                    "This reduced-motion / power-saver / thermal clamp cannot confirm a static-fallback equivalent — shown as a reduced-motion-clamp-unverified projection that keeps the last-known static fallback explicit, never presenting a motion-only cue as clamp-safe when its non-motion equivalence cannot be confirmed"
                        .to_owned(),
                preserves_canonical_identity: true,
                preserves_truth_continuity: true,
            }),
            truth_preserved: true,
            rendering_surfaces: rendering_surfaces(),
            narrowing_disclosures: narrowed_surfaces(&[
                "interaction_identity",
                "semantic_role",
                "motion_profile",
            ]),
            required_labels: all_required_labels(),
            consumer_surfaces: base_consumers(&[
                M5VisualInteractionConsumerSurface::ShellUi,
                M5VisualInteractionConsumerSurface::EditorUi,
            ]),
            source_refs: vec![
                "UX Style Guide §9.7 — Reduced motion / power / thermal clamps".to_owned(),
                MOTION_LAYER_ICONOGRAPHY_A11Y_DOC_REF.to_owned(),
            ],
            observed_at: "2026-07-13T00:00:00Z".to_owned(),
            evidence_refs: ev("reduced-motion-clamp-unconfirmed"),
        },
        // Opacity scrim (orientation / contrast preservation unconfirmed) — the scrim's orientation and text
        // contrast preservation cannot be confirmed, so it auto-narrows to a scrim-orientation-unverified
        // projection that keeps the workspace orientation cue inspectable, never a scrim shown as
        // orientation-safe when it may erase context (yellow).
        VisualInteractionAccessibilityRow {
            record_kind: MOTION_LAYER_ICONOGRAPHY_A11Y_ROW_RECORD_KIND.to_owned(),
            schema_version: MOTION_LAYER_ICONOGRAPHY_A11Y_SCHEMA_VERSION,
            row_id: "a11y:opacity-scrim-orientation-contrast-unconfirmed".to_owned(),
            interaction_family: M5VisualInteractionFamily::OpacityScrim,
            source_family_schema_ref: M5VisualInteractionFamily::OpacityScrim
                .canonical_domain_schema_ref()
                .to_owned(),
            interaction_context_ref: "settings:opacity-scrim:0005".to_owned(),
            fallback_modalities: non_visual_modalities(),
            reaches_canonical_truth: true,
            keyboard_reach: REACHABLE,
            screen_reader_reach: REACHABLE,
            high_zoom_reach: REACHABLE,
            reduced_motion_reach: REACHABLE,
            power_thermal_reach: REACHABLE,
            cli_reach: REACHABLE,
            export_summary: VisualInteractionExportSummaryState::ReconstructableWithoutRawPayload,
            export_summary_ref: "summary:opacity-scrim-orientation-contrast-unconfirmed:a11y"
                .to_owned(),
            copy_export: copy_export(&[
                "interaction_identity",
                "semantic_role",
                "token_reference",
                "orientation_cue",
            ]),
            full_ready_claim: M5VisualInteractionA11yClaim::TrustedInteractionSurface,
            claim_conditions: vec![condition(
                M5VisualInteractionClaimDimension::ScrimOrientationClarity,
                M5VisualInteractionConditionState::ScrimContrastUnconfirmed,
            )],
            claim_narrow: Some(VisualInteractionClaimAutoNarrow {
                narrowed_to: M5VisualInteractionA11yClaim::ScrimOrientationUnverifiedProjection,
                binding_dimension: M5VisualInteractionClaimDimension::ScrimOrientationClarity,
                trigger: M5VisualInteractionDowngradeTrigger::ScrimErasedOrientationOrContrast,
                narrowed_label:
                    "This opacity scrim cannot confirm its orientation and text-contrast preservation — shown as a scrim-orientation-unverified projection that keeps the workspace orientation cue inspectable, never presenting a scrim as orientation-safe when it may erase workspace context or contrast"
                        .to_owned(),
                preserves_canonical_identity: true,
                preserves_truth_continuity: true,
            }),
            truth_preserved: true,
            rendering_surfaces: rendering_surfaces(),
            narrowing_disclosures: narrowed_surfaces(&[
                "interaction_identity",
                "semantic_role",
                "token_reference",
            ]),
            required_labels: all_required_labels(),
            consumer_surfaces: base_consumers(&[
                M5VisualInteractionConsumerSurface::SettingsUi,
                M5VisualInteractionConsumerSurface::ShellUi,
            ]),
            source_refs: vec![
                "UX Style Guide §10.1 — Opacity / scrim".to_owned(),
                MOTION_LAYER_ICONOGRAPHY_A11Y_DOC_REF.to_owned(),
            ],
            observed_at: "2026-07-13T00:00:00Z".to_owned(),
            evidence_refs: ev("opacity-scrim-orientation-contrast-unconfirmed"),
        },
        // Portal ownership (owning-surface attachment unconfirmed) — the portal's owning-surface attachment
        // cannot be confirmed, so it auto-narrows to a portal-ownership-unverified projection that keeps the
        // owning-surface reference and z-tier inspectable, never an overlay shown as attached when it may
        // bypass the shared z-order (yellow).
        VisualInteractionAccessibilityRow {
            record_kind: MOTION_LAYER_ICONOGRAPHY_A11Y_ROW_RECORD_KIND.to_owned(),
            schema_version: MOTION_LAYER_ICONOGRAPHY_A11Y_SCHEMA_VERSION,
            row_id: "a11y:portal-ownership-owning-surface-unconfirmed".to_owned(),
            interaction_family: M5VisualInteractionFamily::PortalOwnership,
            source_family_schema_ref: M5VisualInteractionFamily::PortalOwnership
                .canonical_domain_schema_ref()
                .to_owned(),
            interaction_context_ref: "marketplace:portal-ownership:0006".to_owned(),
            fallback_modalities: non_visual_modalities(),
            reaches_canonical_truth: true,
            keyboard_reach: REACHABLE,
            screen_reader_reach: REACHABLE,
            high_zoom_reach: REACHABLE,
            reduced_motion_reach: REACHABLE,
            power_thermal_reach: REACHABLE,
            cli_reach: REACHABLE,
            export_summary: VisualInteractionExportSummaryState::ReconstructableWithoutRawPayload,
            export_summary_ref: "summary:portal-ownership-owning-surface-unconfirmed:a11y"
                .to_owned(),
            copy_export: copy_export(&[
                "interaction_identity",
                "semantic_role",
                "token_reference",
                "owning_surface_reference",
            ]),
            full_ready_claim: M5VisualInteractionA11yClaim::TrustedInteractionSurface,
            claim_conditions: vec![condition(
                M5VisualInteractionClaimDimension::PortalOwnershipClarity,
                M5VisualInteractionConditionState::PortalOwnershipUnconfirmed,
            )],
            claim_narrow: Some(VisualInteractionClaimAutoNarrow {
                narrowed_to: M5VisualInteractionA11yClaim::PortalOwnershipUnverifiedProjection,
                binding_dimension: M5VisualInteractionClaimDimension::PortalOwnershipClarity,
                trigger: M5VisualInteractionDowngradeTrigger::PortalDetachedFromOwningSurface,
                narrowed_label:
                    "This portal cannot confirm its owning-surface attachment — shown as a portal-ownership-unverified projection that keeps the owning-surface reference and z-tier inspectable, never presenting an overlay as owning-surface-attached when it may bypass the shared z-order model"
                        .to_owned(),
                preserves_canonical_identity: true,
                preserves_truth_continuity: true,
            }),
            truth_preserved: true,
            rendering_surfaces: rendering_surfaces(),
            narrowing_disclosures: narrowed_surfaces(&[
                "interaction_identity",
                "semantic_role",
                "token_reference",
            ]),
            required_labels: all_required_labels(),
            consumer_surfaces: base_consumers(&[
                M5VisualInteractionConsumerSurface::MarketplaceUi,
                M5VisualInteractionConsumerSurface::CliExport,
            ]),
            source_refs: vec![
                "UI/UX Spec §5.5 — Portal / owning-surface attachment".to_owned(),
                MOTION_LAYER_ICONOGRAPHY_A11Y_DOC_REF.to_owned(),
            ],
            observed_at: "2026-07-13T00:00:00Z".to_owned(),
            evidence_refs: ev("portal-ownership-owning-surface-unconfirmed"),
        },
        // Illustration (boundary disclosed partial) — the illustration boundary can only disclose a partial
        // secondary-illustration boundary, so it auto-narrows to an illustration-boundary-disclosed projection
        // that discloses the partial boundary alongside the last-known secondary placement, never a decorative
        // illustration shown as operational or security truth (yellow). Structure-heavy: its illustration set
        // binds to a flat list / textual path. A partial boundary disclosure is an honest disclosed-absence
        // operation, not a trusted overstatement.
        VisualInteractionAccessibilityRow {
            record_kind: MOTION_LAYER_ICONOGRAPHY_A11Y_ROW_RECORD_KIND.to_owned(),
            schema_version: MOTION_LAYER_ICONOGRAPHY_A11Y_SCHEMA_VERSION,
            row_id: "a11y:illustration-boundary-disclosed-partial".to_owned(),
            interaction_family: M5VisualInteractionFamily::Illustration,
            source_family_schema_ref: M5VisualInteractionFamily::Illustration
                .canonical_domain_schema_ref()
                .to_owned(),
            interaction_context_ref: "onboarding:illustration:0007".to_owned(),
            fallback_modalities: structured_modalities(),
            reaches_canonical_truth: true,
            keyboard_reach: REACHABLE,
            screen_reader_reach: REACHABLE,
            high_zoom_reach: REACHABLE,
            reduced_motion_reach: REACHABLE,
            power_thermal_reach: REACHABLE,
            cli_reach: REACHABLE,
            export_summary: VisualInteractionExportSummaryState::ReconstructableWithoutRawPayload,
            export_summary_ref: "summary:illustration-boundary-disclosed-partial:a11y".to_owned(),
            copy_export: copy_export(&[
                "interaction_identity",
                "semantic_role",
                "token_reference",
                "partial_or_disclosed_note",
            ]),
            full_ready_claim: M5VisualInteractionA11yClaim::TrustedInteractionSurface,
            claim_conditions: vec![condition(
                M5VisualInteractionClaimDimension::IllustrationBoundaryClarity,
                M5VisualInteractionConditionState::IllustrationBoundaryDisclosedPartial,
            )],
            claim_narrow: Some(VisualInteractionClaimAutoNarrow {
                narrowed_to: M5VisualInteractionA11yClaim::IllustrationBoundaryDisclosedProjection,
                binding_dimension: M5VisualInteractionClaimDimension::IllustrationBoundaryClarity,
                trigger: M5VisualInteractionDowngradeTrigger::ProofStale,
                narrowed_label:
                    "This illustration boundary can only disclose a partial secondary-illustration boundary — shown as an illustration-boundary-disclosed projection that discloses the partial boundary alongside the last-known secondary placement, never presenting a decorative illustration as operational state, safety approval, or security truth"
                        .to_owned(),
                preserves_canonical_identity: true,
                preserves_truth_continuity: true,
            }),
            truth_preserved: true,
            rendering_surfaces: rendering_surfaces(),
            narrowing_disclosures: narrowed_surfaces(&[
                "interaction_identity",
                "semantic_role",
                "token_reference",
            ]),
            required_labels: all_required_labels(),
            consumer_surfaces: base_consumers(&[
                M5VisualInteractionConsumerSurface::OnboardingUi,
                M5VisualInteractionConsumerSurface::HelpUi,
            ]),
            source_refs: vec![
                "UX Style Guide §11.3 — Illustration boundaries".to_owned(),
                MOTION_LAYER_ICONOGRAPHY_A11Y_DOC_REF.to_owned(),
            ],
            observed_at: "2026-07-13T00:00:00Z".to_owned(),
            evidence_refs: ev("illustration-boundary-disclosed-partial"),
        },
    ]
}
