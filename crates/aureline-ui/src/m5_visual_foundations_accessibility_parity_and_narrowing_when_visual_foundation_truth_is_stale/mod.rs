//! High-contrast / high-zoom / reduced-motion / CLI / export parity, and honest automatic claim
//! narrowing for the M5 color-system / semantic-theme-token / syntax-token / diff-token / chart-token /
//! typography / spacing-sizing-radii-elevation / hit-target visual-foundation families.
//!
//! This module is the M05-1146 accessibility-and-auto-narrowing capstone over the frozen M5 visual
//! foundation matrix ([`crate::m5_visual_foundation_matrix`]). Where the freeze matrix defines the eight
//! governed visual-foundation families, and the 1141-1144 implementation lanes resolve their per-surface
//! token, typography, and geometry truth, this lane certifies — per foundation family — that
//! color / token / typography / geometry claims stay **high-contrast-proven, high-zoom-legible,
//! reduced-motion-safe, CLI/export-safe, and self-narrowing** rather than presenting a stale-contrast
//! palette, an incomplete theme pair, a syntax / diff palette that may collide with diagnostics, a
//! chart whose meaning depends on color alone, a drifted type scale, or a partial geometry / hit-target
//! baseline as still a stable, trusted visual surface:
//!
//! - **High-contrast / high-zoom / reduced-motion / CLI reach.** Every family exposes a
//!   keyboard-reachable, screen-reader-announced, high-contrast-legible, high-zoom-reflowing,
//!   reduced-motion-safe, and CLI/headless-reachable path into the same foundation identity, semantic
//!   role, token reference, theme variant, density context, and non-color cue the rendered surface shows —
//!   never a hue-only status color, a diagnostics-colliding diff palette, a color-only chart, or a
//!   motion-only affordance that strands assistive-tech or headless-CLI users. Structure-heavy families
//!   (the syntax palette's scopes, the diff palette's add / remove / context bands, the chart palette's
//!   series / legend) additionally bind their structured layout to a flat list / textual / legend path.
//! - **Export parity.** The support / release / CLI export reconstructs each family's meaning from typed
//!   tokens and opaque refs **without a raw payload**, preserving the same identity, semantic role, token
//!   reference, theme variant, density context, and non-color cue shown in-product so support, help, and
//!   release proof can reconstruct which visual-foundation truth class was active without leaking a raw
//!   hex value, font blob, or renderer-only screenshot.
//! - **Honest auto-narrowing.** When a color system's contrast evidence is stale, a semantic theme
//!   token's dark / light / high-contrast pair is incomplete, a syntax / diff palette's diagnostics
//!   separation cannot be confirmed, a chart's non-color encoding is unconfirmed, a typography scale's
//!   readability evidence is stale, or a geometry / hit-target baseline can only be partially disclosed,
//!   the family's claim auto-narrows from `trusted_visual_surface` / `reviewable_visual_surface` to a
//!   contrast-unverified / theme-pair-unverified / semantic-separation-unverified / chart-encoding-unverified
//!   / text-readability-unverified / geometry-baseline-disclosed projection, discloses the narrowing with a
//!   precise trigger and binding dimension, and preserves the canonical foundation identity / last-known
//!   token reference. The underlying color / token / typography / geometry truth is never dropped opaquely.
//!   A family with every dimension intact must NOT carry a spurious narrowing, and a stale-contrast /
//!   incomplete-theme-pair / diagnostics-colliding / color-only-chart / drifted-type-scale state can never
//!   keep a trusted, stable visual claim — status or trust meaning is never collapsed into color alone, and
//!   chart meaning never depends on color alone.
//! - **Cross-surface disclosure.** The same narrowed state surfaces in the shell UI, the editor UI, the
//!   review UI, the data UI, the docs UI, the settings UI, the CLI export, the support export, and the
//!   product UI so product, help, and release publication stay aligned on downgrade behavior rather than
//!   drifting in copy — a trusted-looking foundation can never outrun the contrast / theme-pair / separation
//!   / encoding / readability / geometry evidence it is being viewed away from.
//!
//! Each [`VisualFoundationAccessibilityRow`] keys on one
//! [`crate::m5_visual_foundation_matrix::M5VisualFoundationFamily`] and reuses that frozen family
//! vocabulary plus the frozen [`M5VisualFoundationRequiredLabel`], [`M5VisualFoundationDowngradeTrigger`],
//! and shared [`M5VisualFoundationConsumerSurface`] consumer surfaces rather than minting parallel
//! synonyms, so the certified labels stay byte-identical to the matrix and the sibling foundation packets.
//!
//! The packet is metadata-only: raw hex values, font blobs, credentials, secrets, and endpoint refs never
//! cross this boundary; the packet carries only typed class tokens, opaque foundation refs, booleans, and
//! controlled labels so support, release, and diagnostics exports can reconstruct exactly which
//! visual-foundation truth class was active without leaking sensitive material or a raw payload.

#[cfg(test)]
mod tests;

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

// Reused frozen visual-foundation vocabulary — the capstone certifies the freeze matrix's families,
// required labels, downgrade triggers, and consumer surfaces rather than mint parallel ones.
use crate::m5_visual_foundation_matrix::{
    M5VisualFoundationConsumerSurface, M5VisualFoundationDowngradeTrigger,
    M5VisualFoundationFamily, M5VisualFoundationRequiredLabel,
    M5_VISUAL_FOUNDATION_MATRIX_SCHEMA_REF,
};

/// Schema version stamped on the M05-1146 visual-foundations accessibility parity packet.
pub const VISUAL_FOUNDATION_A11Y_SCHEMA_VERSION: u32 = 1;

/// Stable record-kind tag carried by [`VisualFoundationAccessibilityPacket`].
pub const VISUAL_FOUNDATION_A11Y_RECORD_KIND: &str =
    "m5_visual_foundations_accessibility_parity_packet";

/// Stable record-kind tag carried by each [`VisualFoundationAccessibilityRow`].
pub const VISUAL_FOUNDATION_A11Y_ROW_RECORD_KIND: &str =
    "m5_visual_foundations_accessibility_parity_row";

/// Repo-relative path of the boundary schema.
pub const VISUAL_FOUNDATION_A11Y_SCHEMA_REF: &str =
    "schemas/design-system/m5-visual-foundations-accessibility-parity.schema.json";

/// Repo-relative path of the contract doc.
pub const VISUAL_FOUNDATION_A11Y_DOC_REF: &str =
    "docs/design-system/m5_visual_foundations_accessibility_parity.md";

/// Repo-relative path of the frozen visual-foundation matrix this lane certifies.
pub const VISUAL_FOUNDATION_A11Y_MATRIX_REF: &str = M5_VISUAL_FOUNDATION_MATRIX_SCHEMA_REF;

/// Repo-relative path of the protected fixture directory.
pub const VISUAL_FOUNDATION_A11Y_FIXTURE_DIR: &str =
    "fixtures/ui/m5-visual-foundations-accessibility-parity";

/// Repo-relative path of the checked support-export artifact (the `include_str!` canonical).
pub const VISUAL_FOUNDATION_A11Y_ARTIFACT_REF: &str =
    "artifacts/release/m5-visual-foundations-accessibility-parity/support_export.json";

/// Repo-relative path of the checked machine-readable matrix CSV.
pub const VISUAL_FOUNDATION_A11Y_CSV_REF: &str =
    "artifacts/release/m5-visual-foundations-accessibility-parity/matrix.csv";

/// Repo-relative path of the checked Markdown report.
pub const VISUAL_FOUNDATION_A11Y_REPORT_REF: &str =
    "artifacts/release/m5-visual-foundations-accessibility-parity.md";

/// The reusable foundation families that render a dense, structured surface (the syntax palette's scopes,
/// the diff palette's add / remove / context bands, the chart palette's series and legend) and therefore
/// MUST bind their structured layout to an equivalent flat list / textual / legend path so the structure is
/// navigable non-visually.
const fn family_is_structure_heavy(family: M5VisualFoundationFamily) -> bool {
    matches!(
        family,
        M5VisualFoundationFamily::SyntaxToken
            | M5VisualFoundationFamily::DiffToken
            | M5VisualFoundationFamily::ChartToken
    )
}

/// The visual-foundation-truth dimension whose weakening a family primarily discloses. Every row must model
/// at least this dimension so its key weakening axis is covered.
const fn family_primary_dimension(
    family: M5VisualFoundationFamily,
) -> M5VisualFoundationClaimDimension {
    match family {
        M5VisualFoundationFamily::ColorSystem => {
            M5VisualFoundationClaimDimension::ColorContrastClarity
        }
        M5VisualFoundationFamily::SemanticThemeToken => {
            M5VisualFoundationClaimDimension::ThemePairParityClarity
        }
        M5VisualFoundationFamily::SyntaxToken => {
            M5VisualFoundationClaimDimension::SyntaxSeparationClarity
        }
        M5VisualFoundationFamily::DiffToken => {
            M5VisualFoundationClaimDimension::DiffSeparationClarity
        }
        M5VisualFoundationFamily::ChartToken => {
            M5VisualFoundationClaimDimension::ChartEncodingClarity
        }
        M5VisualFoundationFamily::Typography => {
            M5VisualFoundationClaimDimension::TextReadabilityClarity
        }
        M5VisualFoundationFamily::SpacingSizingRadiiElevation => {
            M5VisualFoundationClaimDimension::GeometryBaselineClarity
        }
        M5VisualFoundationFamily::HitTarget => {
            M5VisualFoundationClaimDimension::HitTargetMinimumClarity
        }
    }
}

/// A rendered fallback modality for a visual-foundation family.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5VisualFoundationFallbackModality {
    /// A rich, structured (scopes / bands / series+legend) projection.
    Structured,
    /// A flat list projection.
    List,
    /// A textual / label-first projection.
    Textual,
    /// A CLI / headless text projection.
    Cli,
}

impl M5VisualFoundationFallbackModality {
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

/// A rendering-surface capability tier. Distinct from the semantic consumer surface: the same foundation may
/// render at desktop-full capability or narrow to a companion, read-only browser, headless CLI, docs export,
/// or support export.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5VisualFoundationRenderingSurface {
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

impl M5VisualFoundationRenderingSurface {
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

/// High-contrast / high-zoom / reduced-motion / CLI reach for a foundation's non-visual path.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum VisualFoundationNonVisualReachState {
    /// Fully traversable and labeled with no loss.
    ReachableAndLabeled,
    /// Reachable and labeled, but with a disclosed reduction (yellow).
    DisclosedReducedButReachable,
    /// A view-only / hover-only / hue-only surface that traps keyboard / assistive-tech / headless-CLI
    /// users (red).
    ViewOnlyTrap,
}

impl VisualFoundationNonVisualReachState {
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

/// Whether an export-safe summary preserves the foundation meaning without leaking a raw payload.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum VisualFoundationExportSummaryState {
    /// The foundation meaning reconstructs from the metadata summary without a raw payload.
    ReconstructableWithoutRawPayload,
    /// Partial capture, but disclosed (yellow).
    DisclosedPartialCapture,
    /// The export can only carry meaning by dumping a raw payload (red).
    RequiresRawPayload,
}

impl VisualFoundationExportSummaryState {
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
pub enum VisualFoundationNarrowingDisclosureState {
    /// Full label and summary parity with the desktop surface.
    ParityPreserved,
    /// Reduced interactivity, disclosed with preserved labels (yellow).
    DisclosedNarrowed,
    /// Interactivity, state, or tokens dropped without disclosure (red).
    SilentlyDropped,
}

impl VisualFoundationNarrowingDisclosureState {
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

/// The visual-foundation claim ceiling a family asserts: how strong a trusted / stable posture it lets a
/// surface present. Auto-narrowing lowers this ceiling when a contrast / theme-pair / separation / encoding
/// / readability / geometry dimension weakens so a stale-contrast palette, an incomplete theme pair, a
/// diagnostics-colliding syntax / diff palette, a color-only chart, a drifted type scale, or a partial
/// geometry / hit-target baseline can never keep an old `TrustedVisualSurface` or `ReviewableVisualSurface`
/// label — status or trust meaning is never collapsed into color alone.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5VisualFoundationA11yClaim {
    /// Trusted visual surface: a fully current, contrast-proven, theme-paired, separation-clean,
    /// encoding-honest, readability-stable, geometry-stable foundation — the strongest claim, a visual
    /// foundation Aureline can present as exactly trusted and stable right now.
    TrustedVisualSurface,
    /// Reviewable visual surface: a self-sufficient, inspectable read-only foundation projection (a static
    /// token / geometry reference a user can inspect) that is not itself an authoritative, live-rendering
    /// surface.
    ReviewableVisualSurface,
    /// Contrast-unverified projection: the color system's contrast evidence is stale; the foundation stays a
    /// contrast-unverified projection with its last-known semantic role and non-color cue preserved, never a
    /// fresh, hue-only status color shown as authoritative.
    ContrastUnverifiedProjection,
    /// Theme-pair-unverified projection: the semantic theme token's dark / light / high-contrast pair is
    /// incomplete; the foundation stays a theme-pair-unverified projection that keeps the last-known token
    /// role explicit, never an incomplete theme pair shown as fully stable.
    ThemePairUnverifiedProjection,
    /// Semantic-separation-unverified projection: a syntax / diff palette's diagnostics separation cannot be
    /// confirmed; the foundation stays a semantic-separation-unverified projection that keeps the scope /
    /// band role inspectable, never a syntax or diff palette shown as diagnostics-clean.
    SemanticSeparationUnverifiedProjection,
    /// Chart-encoding-unverified projection: a chart palette's non-color encoding is unconfirmed; the
    /// foundation stays a chart-encoding-unverified projection that keeps the legend / marker / pattern
    /// channel inspectable, never a chart shown as decodable when its meaning may depend on color alone.
    ChartEncodingUnverifiedProjection,
    /// Text-readability-unverified projection: a typography scale's readability evidence is stale; the
    /// foundation stays a text-readability-unverified projection that preserves the last-known type scale
    /// and font stack, never a drifted type scale shown as readability-stable.
    TextReadabilityUnverifiedProjection,
    /// Geometry-baseline-disclosed projection: a geometry / hit-target baseline can only be partially
    /// disclosed; the foundation stays a geometry-baseline-disclosed projection that discloses the partial
    /// density / hit-target baseline, never a full geometry baseline shown as complete.
    GeometryBaselineDisclosedProjection,
}

impl M5VisualFoundationA11yClaim {
    /// Every claim tier, strongest first.
    pub const ALL: [Self; 8] = [
        Self::TrustedVisualSurface,
        Self::ReviewableVisualSurface,
        Self::ContrastUnverifiedProjection,
        Self::ThemePairUnverifiedProjection,
        Self::SemanticSeparationUnverifiedProjection,
        Self::ChartEncodingUnverifiedProjection,
        Self::TextReadabilityUnverifiedProjection,
        Self::GeometryBaselineDisclosedProjection,
    ];

    /// Capability rank; a higher rank asserts a stronger posture. Narrowing lowers rank.
    pub const fn capability_rank(self) -> u8 {
        match self {
            Self::TrustedVisualSurface => 7,
            Self::ReviewableVisualSurface => 6,
            Self::ContrastUnverifiedProjection => 5,
            Self::ThemePairUnverifiedProjection => 4,
            Self::SemanticSeparationUnverifiedProjection => 3,
            Self::ChartEncodingUnverifiedProjection => 2,
            Self::TextReadabilityUnverifiedProjection => 1,
            Self::GeometryBaselineDisclosedProjection => 0,
        }
    }

    /// Returns true when this claim asserts a fully trusted, stable visual surface.
    pub const fn asserts_trusted_surface(self) -> bool {
        matches!(self, Self::TrustedVisualSurface)
    }

    /// Returns true when this claim asserts a fully self-sufficient (trusted or reviewable) surface.
    pub const fn asserts_self_sufficient_surface(self) -> bool {
        matches!(
            self,
            Self::TrustedVisualSurface | Self::ReviewableVisualSurface
        )
    }

    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::TrustedVisualSurface => "trusted_visual_surface",
            Self::ReviewableVisualSurface => "reviewable_visual_surface",
            Self::ContrastUnverifiedProjection => "contrast_unverified_projection",
            Self::ThemePairUnverifiedProjection => "theme_pair_unverified_projection",
            Self::SemanticSeparationUnverifiedProjection => {
                "semantic_separation_unverified_projection"
            }
            Self::ChartEncodingUnverifiedProjection => "chart_encoding_unverified_projection",
            Self::TextReadabilityUnverifiedProjection => "text_readability_unverified_projection",
            Self::GeometryBaselineDisclosedProjection => "geometry_baseline_disclosed_projection",
        }
    }
}

/// The contrast / theme-pair / separation / encoding / readability / geometry dimension whose state governs
/// how far a foundation may claim to be a fully trusted, stable visual surface. The dimensions map 1:1 to
/// the eight frozen foundation families so every family carries an honest narrowing path.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5VisualFoundationClaimDimension {
    /// Color-contrast clarity: does the color system keep its status / trust meaning paired with a
    /// non-color cue and contrast-proven across dark / light / high-contrast rather than hue alone
    /// (color-system)?
    ColorContrastClarity,
    /// Theme-pair parity clarity: does the semantic theme token keep a complete dark / light / high-contrast
    /// pair with a stable role rather than an inlined raw hex (semantic-theme-token)?
    ThemePairParityClarity,
    /// Syntax-separation clarity: does the syntax palette keep its scopes distinct from the diagnostics
    /// palette (syntax-token)?
    SyntaxSeparationClarity,
    /// Diff-separation clarity: does the diff palette keep add / remove / context bands distinct from the
    /// diagnostics palette (diff-token)?
    DiffSeparationClarity,
    /// Chart-encoding clarity: does the chart palette keep a non-color encoding (legend / marker / pattern)
    /// rather than depending on color alone (chart-token)?
    ChartEncodingClarity,
    /// Text-readability clarity: does the typography scale keep a stable type scale, line-height, tabular
    /// numerals, and font stack rather than drifting (typography)?
    TextReadabilityClarity,
    /// Geometry-baseline clarity: does the spacing / sizing / radii / elevation geometry stay density-aware
    /// and machine-readable rather than a local fork (spacing-sizing-radii-elevation)?
    GeometryBaselineClarity,
    /// Hit-target-minimum clarity: does the hit-target rule keep controls at or above the supported minimum
    /// under compact density (hit-target)?
    HitTargetMinimumClarity,
}

impl M5VisualFoundationClaimDimension {
    /// Every dimension, in declaration order.
    pub const ALL: [Self; 8] = [
        Self::ColorContrastClarity,
        Self::ThemePairParityClarity,
        Self::SyntaxSeparationClarity,
        Self::DiffSeparationClarity,
        Self::ChartEncodingClarity,
        Self::TextReadabilityClarity,
        Self::GeometryBaselineClarity,
        Self::HitTargetMinimumClarity,
    ];

    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ColorContrastClarity => "color_contrast_clarity",
            Self::ThemePairParityClarity => "theme_pair_parity_clarity",
            Self::SyntaxSeparationClarity => "syntax_separation_clarity",
            Self::DiffSeparationClarity => "diff_separation_clarity",
            Self::ChartEncodingClarity => "chart_encoding_clarity",
            Self::TextReadabilityClarity => "text_readability_clarity",
            Self::GeometryBaselineClarity => "geometry_baseline_clarity",
            Self::HitTargetMinimumClarity => "hit_target_minimum_clarity",
        }
    }
}

/// The observed condition of one visual-foundation-truth dimension. Anything weaker than
/// [`Self::FullyQualified`] imposes a narrowing ceiling on the foundation's claim. The stale / incomplete /
/// unconfirmed states the lane must auto-narrow on as *weakened evidence* — a stale contrast, an incomplete
/// theme pair, an unconfirmed diagnostics separation, an unconfirmed chart encoding, and a stale text
/// readability — are the states that [`Self::cannot_be_shown_trusted`] flags. A partial geometry / hit-target
/// baseline disclosure is an honest disclosed-absence operation (a partial density / hit-target baseline
/// shown honestly with an inspectable note), not a truth overstatement, so it is deliberately excluded there.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5VisualFoundationConditionState {
    /// Fully current, contrast-proven, theme-paired, separation-clean, encoding-honest, readability-stable,
    /// geometry-complete — imposes no ceiling.
    FullyQualified,
    /// The color system's contrast evidence is stale — claim drops to a contrast-unverified projection.
    ContrastEvidenceStale,
    /// The semantic theme token's dark / light / high-contrast pair is incomplete — claim drops to a
    /// theme-pair-unverified projection.
    ThemePairEvidenceIncomplete,
    /// The syntax / diff palette's diagnostics separation cannot be confirmed — claim drops to a
    /// semantic-separation-unverified projection.
    SemanticSeparationUnconfirmed,
    /// The chart palette's non-color encoding is unconfirmed — claim drops to a chart-encoding-unverified
    /// projection.
    ChartEncodingUnconfirmed,
    /// The typography scale's readability evidence is stale — claim drops to a text-readability-unverified
    /// projection.
    TextReadabilityStale,
    /// The geometry / hit-target baseline can only be partially disclosed — claim drops to a
    /// geometry-baseline-disclosed projection.
    GeometryBaselineDisclosedPartial,
}

impl M5VisualFoundationConditionState {
    /// Every condition state, in declaration order.
    pub const ALL: [Self; 7] = [
        Self::FullyQualified,
        Self::ContrastEvidenceStale,
        Self::ThemePairEvidenceIncomplete,
        Self::SemanticSeparationUnconfirmed,
        Self::ChartEncodingUnconfirmed,
        Self::TextReadabilityStale,
        Self::GeometryBaselineDisclosedPartial,
    ];

    /// Returns true when the dimension is weaker than fully qualified and therefore imposes a narrowing
    /// ceiling.
    pub const fn is_weak(self) -> bool {
        !matches!(self, Self::FullyQualified)
    }

    /// Returns true when the condition reflects weakened evidence that cannot be shown as a fully trusted,
    /// stable visual surface and must never be shown as such. A partial geometry / hit-target baseline
    /// disclosure is an honest disclosed-absence operation (a partial density / hit-target baseline shown
    /// honestly with an inspectable note), not a truth overstatement, so it is deliberately excluded here.
    pub const fn cannot_be_shown_trusted(self) -> bool {
        matches!(
            self,
            Self::ContrastEvidenceStale
                | Self::ThemePairEvidenceIncomplete
                | Self::SemanticSeparationUnconfirmed
                | Self::ChartEncodingUnconfirmed
                | Self::TextReadabilityStale
        )
    }

    /// The strongest claim this condition state permits.
    pub const fn permitted_ceiling(self) -> M5VisualFoundationA11yClaim {
        match self {
            Self::FullyQualified => M5VisualFoundationA11yClaim::TrustedVisualSurface,
            Self::ContrastEvidenceStale => {
                M5VisualFoundationA11yClaim::ContrastUnverifiedProjection
            }
            Self::ThemePairEvidenceIncomplete => {
                M5VisualFoundationA11yClaim::ThemePairUnverifiedProjection
            }
            Self::SemanticSeparationUnconfirmed => {
                M5VisualFoundationA11yClaim::SemanticSeparationUnverifiedProjection
            }
            Self::ChartEncodingUnconfirmed => {
                M5VisualFoundationA11yClaim::ChartEncodingUnverifiedProjection
            }
            Self::TextReadabilityStale => {
                M5VisualFoundationA11yClaim::TextReadabilityUnverifiedProjection
            }
            Self::GeometryBaselineDisclosedPartial => {
                M5VisualFoundationA11yClaim::GeometryBaselineDisclosedProjection
            }
        }
    }

    /// The frozen downgrade trigger this condition names when its weakness binds a narrowing. Each state
    /// maps to the on-topic frozen trigger the freeze matrix already governs, so the certified reason stays
    /// byte-identical to the matrix.
    pub const fn default_trigger(self) -> M5VisualFoundationDowngradeTrigger {
        match self {
            // The fully-qualified baseline never narrows; kept for exhaustiveness.
            Self::FullyQualified => M5VisualFoundationDowngradeTrigger::ProofStale,
            Self::ContrastEvidenceStale => {
                M5VisualFoundationDowngradeTrigger::StatusOrTrustCollapsedToColorOnly
            }
            Self::ThemePairEvidenceIncomplete => {
                M5VisualFoundationDowngradeTrigger::ThemePairIncomplete
            }
            Self::SemanticSeparationUnconfirmed => {
                M5VisualFoundationDowngradeTrigger::SyntaxOrDiffPaletteCollidedWithDiagnostics
            }
            Self::ChartEncodingUnconfirmed => {
                M5VisualFoundationDowngradeTrigger::ChartMeaningDependedOnColorAlone
            }
            Self::TextReadabilityStale => {
                M5VisualFoundationDowngradeTrigger::TypographyScaleDrifted
            }
            Self::GeometryBaselineDisclosedPartial => {
                M5VisualFoundationDowngradeTrigger::ProofStale
            }
        }
    }

    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::FullyQualified => "fully_qualified",
            Self::ContrastEvidenceStale => "contrast_evidence_stale",
            Self::ThemePairEvidenceIncomplete => "theme_pair_evidence_incomplete",
            Self::SemanticSeparationUnconfirmed => "semantic_separation_unconfirmed",
            Self::ChartEncodingUnconfirmed => "chart_encoding_unconfirmed",
            Self::TextReadabilityStale => "text_readability_stale",
            Self::GeometryBaselineDisclosedPartial => "geometry_baseline_disclosed_partial",
        }
    }
}

/// One visual-foundation-truth dimension's observed condition on a foundation.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct VisualFoundationClaimConditionEntry {
    /// Which dimension this entry describes.
    pub dimension: M5VisualFoundationClaimDimension,
    /// The observed condition state of the dimension.
    pub state: M5VisualFoundationConditionState,
}

/// An honest claim auto-narrow block. When a visual-foundation-truth dimension weakens, the foundation's
/// claim lowers to the permitted ceiling, names the binding dimension and frozen trigger, and preserves the
/// canonical foundation identity / last-known token reference rather than silently dropping it — the
/// underlying color / token / typography / geometry truth is never erased opaquely.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct VisualFoundationClaimAutoNarrow {
    /// The claim the foundation is narrowed to.
    pub narrowed_to: M5VisualFoundationA11yClaim,
    /// The dimension whose weakness bound the narrowing (the one imposing the strongest ceiling
    /// constraint).
    pub binding_dimension: M5VisualFoundationClaimDimension,
    /// The frozen downgrade trigger (reused vocabulary) the narrowing names.
    pub trigger: M5VisualFoundationDowngradeTrigger,
    /// A precise, non-generic label safe to render.
    pub narrowed_label: String,
    /// The canonical foundation identity and last-known token reference are preserved rather than dropped;
    /// must hold.
    pub preserves_canonical_identity: bool,
    /// The underlying color / token / typography / geometry truth is preserved (never dropped) across the
    /// narrowing; must hold so contrast-unverified, theme-pair-unverified, semantic-separation-unverified,
    /// chart-encoding-unverified, text-readability-unverified, and geometry-baseline-disclosed states never
    /// fail opaquely.
    pub preserves_truth_continuity: bool,
}

impl VisualFoundationClaimAutoNarrow {
    /// Whether the auto-narrow block is honest: it preserves canonical identity and color / token /
    /// typography / geometry truth and carries a precise, non-generic label.
    pub fn is_honest(&self) -> bool {
        self.preserves_canonical_identity
            && self.preserves_truth_continuity
            && !label_is_generic(&self.narrowed_label)
    }
}

/// Copy / export parity for a foundation's accessible fallback: the same truth must be copyable as
/// text / JSON / Markdown, and a raw payload is never the only export.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct VisualFoundationCopyExportParity {
    /// The copy / export formats offered (must include text, json, markdown).
    #[serde(default)]
    pub formats: Vec<String>,
    /// The named export fields the summary carries.
    #[serde(default)]
    pub export_fields: Vec<String>,
    /// A raw payload is never the only export; must always hold.
    pub raw_payload_only_prohibited: bool,
}

impl VisualFoundationCopyExportParity {
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
pub struct VisualFoundationRenderingNarrowingDisclosure {
    /// The rendering surface being narrowed.
    pub rendering_surface: M5VisualFoundationRenderingSurface,
    /// How the surface discloses its reduced interactivity.
    pub state: VisualFoundationNarrowingDisclosureState,
    /// The labels preserved across the narrowing.
    #[serde(default)]
    pub preserved_labels: Vec<String>,
    /// The interactions reduced on the narrowed surface.
    #[serde(default)]
    pub reduced_interactions: Vec<String>,
}

/// Derived qualification status for a visual-foundation accessibility row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum VisualFoundationAccessibilityStatus {
    /// Full high-contrast / high-zoom / reduced-motion / CLI / export parity with no narrowing (green).
    Parity,
    /// Reduced but fully disclosed, reachable, and honestly auto-narrowed (yellow).
    NarrowedDisclosed,
    /// Strands assistive tech, needs a raw payload, over-claims trusted, or drops state silently (red).
    Stranded,
}

impl VisualFoundationAccessibilityStatus {
    /// Stable token recorded in the summary / CSV.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Parity => "parity",
            Self::NarrowedDisclosed => "narrowed_disclosed",
            Self::Stranded => "stranded",
        }
    }
}

/// Accessibility / auto-narrowing parity row for one visual-foundation family.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct VisualFoundationAccessibilityRow {
    /// Record kind; must equal [`VISUAL_FOUNDATION_A11Y_ROW_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`VISUAL_FOUNDATION_A11Y_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable row id.
    pub row_id: String,
    /// The frozen foundation family this row certifies.
    pub foundation_family: M5VisualFoundationFamily,
    /// Ref to the frozen canonical per-domain schema this row certifies.
    pub source_family_schema_ref: String,
    /// Opaque ref to the foundation this row represents; stays visible on every surface, so this is never
    /// empty.
    pub foundation_context_ref: String,
    /// Rendered modalities offered; a structure-heavy family must also offer a non-visual (list / textual /
    /// CLI) path.
    #[serde(default)]
    pub fallback_modalities: Vec<M5VisualFoundationFallbackModality>,
    /// The non-visual / CLI path reaches the same canonical identity, semantic role, token reference, theme
    /// variant, density context, and non-color cue as the rendered foundation; must hold.
    pub reaches_canonical_truth: bool,
    /// Keyboard reach into the non-visual path.
    pub keyboard_reach: VisualFoundationNonVisualReachState,
    /// Screen-reader reach into the non-visual path.
    pub screen_reader_reach: VisualFoundationNonVisualReachState,
    /// High-contrast / high-zoom (reflow / magnification) legibility of the non-visual path.
    pub high_zoom_reach: VisualFoundationNonVisualReachState,
    /// Reduced-motion behavior of the non-visual path.
    pub reduced_motion_reach: VisualFoundationNonVisualReachState,
    /// CLI / headless reach into the non-visual path.
    pub cli_reach: VisualFoundationNonVisualReachState,
    /// Whether the export-safe summary preserves foundation meaning.
    pub export_summary: VisualFoundationExportSummaryState,
    /// Ref to the export-safe summary object for this foundation.
    pub export_summary_ref: String,
    /// The copy / export parity of the accessible fallback.
    pub copy_export: VisualFoundationCopyExportParity,
    /// The full claim this family asserts when every dimension is intact.
    pub full_ready_claim: M5VisualFoundationA11yClaim,
    /// The observed condition of each modeled visual-foundation-truth dimension.
    #[serde(default)]
    pub claim_conditions: Vec<VisualFoundationClaimConditionEntry>,
    /// The honest auto-narrow block, present only when some dimension weakens below the family's full claim.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub claim_narrow: Option<VisualFoundationClaimAutoNarrow>,
    /// Whether the underlying color / token / typography / geometry truth is preserved on this foundation
    /// regardless of narrowing; must hold so every unverified projection never fails opaquely.
    pub truth_preserved: bool,
    /// Rendering surfaces this foundation is certified on.
    #[serde(default)]
    pub rendering_surfaces: Vec<M5VisualFoundationRenderingSurface>,
    /// Per-surface narrowing disclosures.
    #[serde(default)]
    pub narrowing_disclosures: Vec<VisualFoundationRenderingNarrowingDisclosure>,
    /// The required labels the accessible fallback preserves (reused vocabulary).
    #[serde(default)]
    pub required_labels: Vec<M5VisualFoundationRequiredLabel>,
    /// Semantic consumer surfaces this foundation is embedded in (reused vocabulary).
    #[serde(default)]
    pub consumer_surfaces: Vec<M5VisualFoundationConsumerSurface>,
    /// Source contract refs backing this row.
    #[serde(default)]
    pub source_refs: Vec<String>,
    /// ISO 8601 UTC timestamp the accessibility posture was observed.
    pub observed_at: String,
    /// Evidence packet refs backing this row.
    #[serde(default)]
    pub evidence_refs: Vec<String>,
}

impl VisualFoundationAccessibilityRow {
    /// Returns true when this family renders a dense, structured surface and must bind to a flat non-visual
    /// path.
    pub const fn is_structure_heavy(&self) -> bool {
        family_is_structure_heavy(self.foundation_family)
    }

    /// Returns true when at least one non-visual (list / textual / CLI) fallback modality is offered.
    pub fn has_non_visual_fallback(&self) -> bool {
        self.fallback_modalities.iter().any(|m| m.is_non_visual())
    }

    /// The condition state observed for one dimension, or `FullyQualified` when the row does not model that
    /// dimension.
    pub fn condition_for(
        &self,
        dimension: M5VisualFoundationClaimDimension,
    ) -> M5VisualFoundationConditionState {
        self.claim_conditions
            .iter()
            .find(|c| c.dimension == dimension)
            .map(|c| c.state)
            .unwrap_or(M5VisualFoundationConditionState::FullyQualified)
    }

    /// Whether any modeled dimension is weaker than fully qualified.
    pub fn has_weak_dimension(&self) -> bool {
        self.claim_conditions.iter().any(|c| c.state.is_weak())
    }

    /// The strongest claim permitted after applying every modeled dimension's ceiling, capped at the
    /// family's full claim.
    pub fn permitted_claim(&self) -> M5VisualFoundationA11yClaim {
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
    pub fn binding_condition(&self) -> Option<&VisualFoundationClaimConditionEntry> {
        let mut binding: Option<(&VisualFoundationClaimConditionEntry, u8)> = None;
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
    pub fn binding_dimension(&self) -> Option<M5VisualFoundationClaimDimension> {
        self.binding_condition().map(|c| c.dimension)
    }

    /// The claim this foundation effectively asserts after narrowing.
    pub fn effective_claim(&self) -> M5VisualFoundationA11yClaim {
        match &self.claim_narrow {
            Some(narrow) => narrow.narrowed_to,
            None => self.full_ready_claim,
        }
    }

    /// AC / auto-narrowing honesty: a stale-contrast palette, an incomplete theme pair, a
    /// diagnostics-colliding syntax / diff palette, a color-only chart, a drifted type scale, or a partial
    /// geometry / hit-target baseline can no longer keep an old `TrustedVisualSurface` /
    /// `ReviewableVisualSurface` label. The effective claim never exceeds the permitted ceiling; when a
    /// dimension narrows below the full claim, an honest narrow block is present, narrows to exactly the
    /// permitted ceiling, binds to the ceiling-imposing dimension with its frozen trigger, and preserves
    /// canonical identity and truth. When nothing narrows, no spurious narrow block is present.
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

    /// AC / trusted honesty: a stale-contrast / incomplete-theme-pair / diagnostics-colliding /
    /// color-only-chart / drifted-type-scale state never keeps a trusted claim — status or trust meaning is
    /// never collapsed into color alone. When such a state is modeled, the effective claim must not assert
    /// `TrustedVisualSurface`.
    pub fn trusted_honesty_holds(&self) -> bool {
        let has_unprovable_state = self
            .claim_conditions
            .iter()
            .any(|c| c.state.cannot_be_shown_trusted());
        !(has_unprovable_state && self.effective_claim().asserts_trusted_surface())
    }

    /// AC / assistive-tech reach: accessibility and export surfaces reach the same canonical truth — no
    /// keyboard / screen-reader / high-contrast / high-zoom / reduced-motion / CLI trap, a structure-heavy
    /// family offers a non-visual fallback, and the export reconstructs meaning without a raw payload.
    pub fn reaches_canonical_truth_via_at(&self) -> bool {
        self.reaches_canonical_truth
            && !self.foundation_context_ref.trim().is_empty()
            && self.keyboard_reach.never_traps()
            && self.screen_reader_reach.never_traps()
            && self.high_zoom_reach.never_traps()
            && self.reduced_motion_reach.never_traps()
            && self.cli_reach.never_traps()
            && (!self.is_structure_heavy() || self.has_non_visual_fallback())
    }

    /// The export preserves the foundation meaning without leaking a raw payload.
    pub fn export_preserves_meaning(&self) -> bool {
        self.export_summary.never_requires_raw_payload()
            && !self.export_summary_ref.trim().is_empty()
            && self.copy_export.is_complete()
    }

    /// AC / no-loss: every unverified projection preserves the underlying color / token / typography /
    /// geometry truth. The row must assert `truth_preserved`, and any narrow block must preserve truth
    /// continuity too.
    pub fn preserves_truth_continuity(&self) -> bool {
        self.truth_preserved
            && self
                .claim_narrow
                .as_ref()
                .map(|n| n.preserves_truth_continuity)
                .unwrap_or(true)
    }

    /// Whether any axis is in a disclosed-reduction (yellow) state or the foundation carries an honest claim
    /// narrow.
    pub fn is_reduced(&self) -> bool {
        self.claim_narrow.is_some()
            || self.keyboard_reach.is_disclosed_reduction()
            || self.screen_reader_reach.is_disclosed_reduction()
            || self.high_zoom_reach.is_disclosed_reduction()
            || self.reduced_motion_reach.is_disclosed_reduction()
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
        let primary = family_primary_dimension(self.foundation_family);
        self.claim_conditions.iter().any(|c| c.dimension == primary)
    }

    /// Whether every mandatory required label is preserved on the accessible fallback.
    pub fn preserves_mandatory_labels(&self) -> bool {
        M5VisualFoundationRequiredLabel::MANDATORY
            .iter()
            .all(|label| self.required_labels.contains(label))
    }

    /// Derived qualification status.
    pub fn status(&self) -> VisualFoundationAccessibilityStatus {
        if !self.claim_is_honest()
            || !self.trusted_honesty_holds()
            || !self.reaches_canonical_truth_via_at()
            || !self.export_preserves_meaning()
            || !self.preserves_truth_continuity()
            || !self.narrowing_disclosed()
            || !self.models_primary_dimension()
            || !self.preserves_mandatory_labels()
        {
            return VisualFoundationAccessibilityStatus::Stranded;
        }
        if self.is_reduced() {
            VisualFoundationAccessibilityStatus::NarrowedDisclosed
        } else {
            VisualFoundationAccessibilityStatus::Parity
        }
    }

    /// Whether the row's identity and evidence fields are complete.
    pub fn is_complete(&self) -> bool {
        self.record_kind == VISUAL_FOUNDATION_A11Y_ROW_RECORD_KIND
            && self.schema_version == VISUAL_FOUNDATION_A11Y_SCHEMA_VERSION
            && !self.row_id.trim().is_empty()
            && !self.source_family_schema_ref.trim().is_empty()
            && !self.foundation_context_ref.trim().is_empty()
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
high_zoom={high_zoom} reduced_motion={reduced_motion} cli={cli} export={export} \
full_claim={full} effective_claim={effective} status={status}",
            family = self.foundation_family.as_str(),
            keyboard = self.keyboard_reach.as_str(),
            screen_reader = self.screen_reader_reach.as_str(),
            high_zoom = self.high_zoom_reach.as_str(),
            reduced_motion = self.reduced_motion_reach.as_str(),
            cli = self.cli_reach.as_str(),
            export = self.export_summary.as_str(),
            full = self.full_ready_claim.as_str(),
            effective = self.effective_claim().as_str(),
            status = self.status().as_str(),
        )
    }
}

/// Rolled-up summary of an M05-1146 visual-foundations accessibility parity packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct VisualFoundationAccessibilitySummary {
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

/// Constructor input for [`VisualFoundationAccessibilityPacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VisualFoundationAccessibilityPacketInput {
    pub packet_id: String,
    pub as_of: String,
    pub matrix_ref: String,
    pub rows: Vec<VisualFoundationAccessibilityRow>,
}

/// Checked-in M05-1146 visual-foundations accessibility parity packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct VisualFoundationAccessibilityPacket {
    pub schema_version: u32,
    pub record_kind: String,
    pub packet_id: String,
    pub as_of: String,
    pub matrix_ref: String,
    #[serde(default)]
    pub rows: Vec<VisualFoundationAccessibilityRow>,
    pub summary: VisualFoundationAccessibilitySummary,
}

impl VisualFoundationAccessibilityPacket {
    /// Builds a packet, stamping the record kind, schema version, and computed summary.
    pub fn new(input: VisualFoundationAccessibilityPacketInput) -> Self {
        let mut packet = Self {
            schema_version: VISUAL_FOUNDATION_A11Y_SCHEMA_VERSION,
            record_kind: VISUAL_FOUNDATION_A11Y_RECORD_KIND.to_owned(),
            packet_id: input.packet_id,
            as_of: input.as_of,
            matrix_ref: input.matrix_ref,
            rows: input.rows,
            summary: VisualFoundationAccessibilitySummary {
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
    pub fn represented_families(&self) -> BTreeSet<M5VisualFoundationFamily> {
        self.rows.iter().map(|r| r.foundation_family).collect()
    }

    /// Dimensions exercised by some row's claim conditions.
    pub fn exercised_dimensions(&self) -> BTreeSet<M5VisualFoundationClaimDimension> {
        self.rows
            .iter()
            .flat_map(|r| r.claim_conditions.iter().map(|c| c.dimension))
            .collect()
    }

    /// Condition states exercised by some row's claim conditions.
    pub fn exercised_condition_states(&self) -> BTreeSet<M5VisualFoundationConditionState> {
        self.rows
            .iter()
            .flat_map(|r| r.claim_conditions.iter().map(|c| c.state))
            .collect()
    }

    /// Claim tiers that appear as an effective claim across the rows.
    pub fn represented_effective_claims(&self) -> BTreeSet<M5VisualFoundationA11yClaim> {
        self.rows.iter().map(|r| r.effective_claim()).collect()
    }

    /// Consumer surfaces ingesting some row in this packet.
    pub fn represented_consumer_surfaces(&self) -> BTreeSet<M5VisualFoundationConsumerSurface> {
        self.rows
            .iter()
            .flat_map(|r| r.consumer_surfaces.iter().copied())
            .collect()
    }

    /// Computes summary fields from the packet contents.
    pub fn computed_summary(&self) -> VisualFoundationAccessibilitySummary {
        let mut rendering = BTreeSet::new();
        let mut consumers: BTreeSet<M5VisualFoundationConsumerSurface> = BTreeSet::new();
        for row in &self.rows {
            rendering.extend(row.rendering_surfaces.iter().copied());
            consumers.extend(row.consumer_surfaces.iter().copied());
        }

        let structure_heavy: Vec<&VisualFoundationAccessibilityRow> = self
            .rows
            .iter()
            .filter(|row| row.is_structure_heavy())
            .collect();

        let mut green = 0;
        let mut yellow = 0;
        let mut red = 0;
        for row in &self.rows {
            match row.status() {
                VisualFoundationAccessibilityStatus::Parity => green += 1,
                VisualFoundationAccessibilityStatus::NarrowedDisclosed => yellow += 1,
                VisualFoundationAccessibilityStatus::Stranded => red += 1,
            }
        }

        VisualFoundationAccessibilitySummary {
            row_count: self.rows.len(),
            family_count: self.represented_families().len(),
            structure_heavy_family_count: structure_heavy.len(),
            all_structure_heavy_have_non_visual_fallback: structure_heavy
                .iter()
                .all(|row| row.has_non_visual_fallback()),
            all_reach_canonical_truth_via_at: self
                .rows
                .iter()
                .all(VisualFoundationAccessibilityRow::reaches_canonical_truth_via_at),
            all_claims_honest: self
                .rows
                .iter()
                .all(VisualFoundationAccessibilityRow::claim_is_honest),
            all_trusted_honesty_holds: self
                .rows
                .iter()
                .all(VisualFoundationAccessibilityRow::trusted_honesty_holds),
            all_export_summaries_preserve_meaning: self
                .rows
                .iter()
                .all(VisualFoundationAccessibilityRow::export_preserves_meaning),
            all_truth_preserved: self
                .rows
                .iter()
                .all(VisualFoundationAccessibilityRow::preserves_truth_continuity),
            all_narrowing_disclosed: self
                .rows
                .iter()
                .all(VisualFoundationAccessibilityRow::narrowing_disclosed),
            green_count: green,
            yellow_count: yellow,
            red_count: red,
            rendering_surface_count: rendering.len(),
            consumer_surface_count: consumers.len(),
        }
    }

    /// Validates the packet and returns every contract violation.
    pub fn validate(&self) -> Vec<VisualFoundationAccessibilityViolation> {
        let mut violations = Vec::new();

        if self.schema_version != VISUAL_FOUNDATION_A11Y_SCHEMA_VERSION {
            violations.push(VisualFoundationAccessibilityViolation::SchemaVersion {
                expected: VISUAL_FOUNDATION_A11Y_SCHEMA_VERSION,
                actual: self.schema_version,
            });
        }
        if self.record_kind != VISUAL_FOUNDATION_A11Y_RECORD_KIND {
            violations.push(VisualFoundationAccessibilityViolation::RecordKind {
                expected: VISUAL_FOUNDATION_A11Y_RECORD_KIND.to_owned(),
                actual: self.record_kind.clone(),
            });
        }
        if self.packet_id.trim().is_empty()
            || self.as_of.trim().is_empty()
            || self.matrix_ref.trim().is_empty()
        {
            violations.push(VisualFoundationAccessibilityViolation::MissingIdentity);
        }

        let mut row_ids = BTreeSet::new();
        let mut seen_families = BTreeSet::new();
        let mut has_unprovable_row = false;
        for row in &self.rows {
            if !row_ids.insert(row.row_id.clone()) {
                violations.push(VisualFoundationAccessibilityViolation::DuplicateId {
                    id: row.row_id.clone(),
                });
            }
            seen_families.insert(row.foundation_family);
            if row
                .claim_conditions
                .iter()
                .any(|c| c.state.cannot_be_shown_trusted())
            {
                has_unprovable_row = true;
            }

            if !row.is_complete() {
                violations.push(VisualFoundationAccessibilityViolation::IncompleteRow {
                    id: row.row_id.clone(),
                });
            }

            // Each row must model its family's primary weakening dimension.
            if !row.models_primary_dimension() {
                violations.push(
                    VisualFoundationAccessibilityViolation::MissingPrimaryDimension {
                        id: row.row_id.clone(),
                        dimension: family_primary_dimension(row.foundation_family),
                    },
                );
            }

            // Each row must preserve every mandatory foundation label.
            if !row.preserves_mandatory_labels() {
                violations.push(
                    VisualFoundationAccessibilityViolation::MissingMandatoryLabel {
                        id: row.row_id.clone(),
                    },
                );
            }

            // A structure-heavy family must render a structured projection *and* a non-visual path.
            if row.is_structure_heavy()
                && !row
                    .fallback_modalities
                    .contains(&M5VisualFoundationFallbackModality::Structured)
            {
                violations.push(
                    VisualFoundationAccessibilityViolation::StructureHeavyMissingStructured {
                        id: row.row_id.clone(),
                    },
                );
            }

            // AC: claim never over-asserts a trusted / reviewable surface for a weakened one.
            if !row.claim_is_honest() {
                violations.push(VisualFoundationAccessibilityViolation::ClaimOverAsserted {
                    id: row.row_id.clone(),
                });
            }

            // AC / trusted honesty: a stale-contrast / incomplete-theme-pair / diagnostics-colliding /
            // color-only-chart / drifted-type-scale state never keeps a trusted claim.
            if !row.trusted_honesty_holds() {
                violations.push(
                    VisualFoundationAccessibilityViolation::WeakStateShownAsTrusted {
                        id: row.row_id.clone(),
                    },
                );
            }

            // AC: assistive-tech / CLI reach the same canonical truth.
            if !row.reaches_canonical_truth_via_at() {
                violations.push(
                    VisualFoundationAccessibilityViolation::AssistiveTechStranded {
                        id: row.row_id.clone(),
                    },
                );
            }

            // AC: export preserves meaning without leaking a raw payload.
            if !row.export_preserves_meaning() {
                violations.push(
                    VisualFoundationAccessibilityViolation::ExportRequiresRawPayload {
                        id: row.row_id.clone(),
                    },
                );
            }

            // AC / no-loss: weakened states preserve color / token / typography / geometry truth.
            if !row.preserves_truth_continuity() {
                violations.push(VisualFoundationAccessibilityViolation::TruthDropped {
                    id: row.row_id.clone(),
                });
            }

            // Narrowing disclosed on every narrowed rendering surface.
            if !row.narrowing_disclosed() {
                violations.push(
                    VisualFoundationAccessibilityViolation::NarrowingDropsContextSilently {
                        id: row.row_id.clone(),
                    },
                );
            }

            // Consumer parity: at least two consumer surfaces ingest the row.
            if row.consumer_surfaces.len() < 2 {
                violations.push(
                    VisualFoundationAccessibilityViolation::MissingConsumerParity {
                        id: row.row_id.clone(),
                    },
                );
            }

            // No red rows may ship.
            if row.status() == VisualFoundationAccessibilityStatus::Stranded {
                violations.push(VisualFoundationAccessibilityViolation::StrandedRow {
                    id: row.row_id.clone(),
                });
            }
        }

        // Coverage: every frozen family is certified at least once.
        for family in M5VisualFoundationFamily::ALL {
            if !seen_families.contains(&family) {
                violations
                    .push(VisualFoundationAccessibilityViolation::MissingFamilyCoverage { family });
            }
        }

        // Coverage: every weakening dimension is exercised somewhere.
        let exercised = self.exercised_dimensions();
        for dimension in M5VisualFoundationClaimDimension::ALL {
            if !exercised.contains(&dimension) {
                violations.push(
                    VisualFoundationAccessibilityViolation::MissingDimensionCoverage { dimension },
                );
            }
        }

        // Coverage: every condition state (the fully-qualified baseline plus each spec narrowing axis) is
        // exercised somewhere, so the full narrowing spectrum is proven end-to-end.
        let states = self.exercised_condition_states();
        for state in M5VisualFoundationConditionState::ALL {
            if !states.contains(&state) {
                violations.push(
                    VisualFoundationAccessibilityViolation::MissingConditionStateCoverage { state },
                );
            }
        }

        // Coverage: every claim tier appears as an effective claim, so the full narrowing spectrum
        // (trusted → … → geometry-baseline-disclosed) is proven end-to-end.
        let effective = self.represented_effective_claims();
        for claim in M5VisualFoundationA11yClaim::ALL {
            if !effective.contains(&claim) {
                violations.push(
                    VisualFoundationAccessibilityViolation::MissingClaimTierCoverage { claim },
                );
            }
        }

        // Trusted honesty must be proven with at least one stale-contrast / incomplete-theme-pair /
        // diagnostics-colliding / color-only-chart / drifted-type-scale row in the packet, so the
        // "cannot-prove never shown as trusted" guarantee is exercised end-to-end.
        if !has_unprovable_row {
            violations.push(VisualFoundationAccessibilityViolation::TrustedHonestyUnproven);
        }

        // Cross-surface: the same narrowed state must reach the shell, editor, review, data, docs, settings,
        // CLI-export, support-export, and product surfaces — so every consumer surface is exercised at least
        // once across the packet.
        let consumers = self.represented_consumer_surfaces();
        for surface in M5VisualFoundationConsumerSurface::ALL {
            if !consumers.contains(&surface) {
                violations.push(
                    VisualFoundationAccessibilityViolation::MissingConsumerSurfaceCoverage {
                        surface,
                    },
                );
            }
        }

        if self.summary != self.computed_summary() {
            violations.push(VisualFoundationAccessibilityViolation::SummaryMismatch);
        }

        if json_contains_forbidden_material(
            &serde_json::to_value(self)
                .expect("visual-foundations accessibility parity packet serializes"),
        ) {
            violations.push(VisualFoundationAccessibilityViolation::RawFoundationMaterialInExport);
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
            .expect("visual-foundations accessibility parity packet serializes")
    }

    /// Deterministic CSV of the certified rows for support / release handoff.
    pub fn render_matrix_csv(&self) -> String {
        let mut out = String::from(
            "row_id,foundation_family,keyboard_reach,screen_reader_reach,high_zoom_reach,reduced_motion_reach,cli_reach,export_summary,full_claim,effective_claim,status\n",
        );
        for row in &self.rows {
            out.push_str(&format!(
                "{id},{family},{keyboard},{screen_reader},{high_zoom},{reduced_motion},{cli},{export},{full},{effective},{status}\n",
                id = row.row_id,
                family = row.foundation_family.as_str(),
                keyboard = row.keyboard_reach.as_str(),
                screen_reader = row.screen_reader_reach.as_str(),
                high_zoom = row.high_zoom_reach.as_str(),
                reduced_motion = row.reduced_motion_reach.as_str(),
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
        out.push_str("# M5 Visual-Foundation Accessibility & Auto-Narrowing\n\n");
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- As of: `{}`\n", self.as_of));
        out.push_str(&format!(
            "- Families: {} certified across {} / {} frozen families\n",
            self.summary.family_count,
            self.represented_families().len(),
            M5VisualFoundationFamily::ALL.len(),
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
                row.foundation_family.as_str(),
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

/// Reads and validates the checked-in visual-foundations accessibility parity export.
pub fn current_m5_visual_foundations_a11y_export(
) -> Result<VisualFoundationAccessibilityPacket, VisualFoundationAccessibilityArtifactError> {
    let packet: VisualFoundationAccessibilityPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/release/m5-visual-foundations-accessibility-parity/support_export.json"
    )))
    .map_err(VisualFoundationAccessibilityArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(VisualFoundationAccessibilityArtifactError::Validation(
            violations,
        ))
    }
}

/// Errors emitted when reading the checked-in visual-foundations accessibility parity export.
#[derive(Debug)]
pub enum VisualFoundationAccessibilityArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<VisualFoundationAccessibilityViolation>),
}

impl fmt::Display for VisualFoundationAccessibilityArtifactError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(
                    f,
                    "visual-foundations accessibility parity export parse failed: {error}"
                )
            }
            Self::Validation(violations) => {
                write!(
                    f,
                    "visual-foundations accessibility parity export failed validation: {} violation(s)",
                    violations.len()
                )
            }
        }
    }
}

impl Error for VisualFoundationAccessibilityArtifactError {}

/// Validation failure for M05-1146 visual-foundations accessibility parity packets.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum VisualFoundationAccessibilityViolation {
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
        dimension: M5VisualFoundationClaimDimension,
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
        family: M5VisualFoundationFamily,
    },
    MissingDimensionCoverage {
        dimension: M5VisualFoundationClaimDimension,
    },
    MissingConditionStateCoverage {
        state: M5VisualFoundationConditionState,
    },
    MissingClaimTierCoverage {
        claim: M5VisualFoundationA11yClaim,
    },
    TrustedHonestyUnproven,
    MissingConsumerSurfaceCoverage {
        surface: M5VisualFoundationConsumerSurface,
    },
    SummaryMismatch,
    RawFoundationMaterialInExport,
}

impl VisualFoundationAccessibilityViolation {
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
            Self::RawFoundationMaterialInExport => "raw_foundation_material_in_export",
        }
    }
}

impl fmt::Display for VisualFoundationAccessibilityViolation {
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
                write!(f, "row {id} drops a mandatory foundation label")
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
                    "row {id} shows a stale-contrast / incomplete-theme-pair / diagnostics-colliding / color-only-chart / drifted-type-scale state as a trusted visual surface"
                )
            }
            Self::AssistiveTechStranded { id } => {
                write!(
                    f,
                    "row {id} strands keyboard / assistive-tech / high-contrast / high-zoom / reduced-motion / CLI users from the canonical truth"
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
                    "row {id} does not preserve color / token / typography / geometry truth across narrowing"
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
                    "foundation family {family:?} is not certified in the packet"
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
                    "no stale-contrast / incomplete-theme-pair / diagnostics-colliding / color-only-chart / drifted-type-scale row is present to prove the trusted-honesty guarantee"
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
            Self::RawFoundationMaterialInExport => {
                write!(f, "export contains raw foundation material")
            }
        }
    }
}

impl Error for VisualFoundationAccessibilityViolation {}

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
pub const VISUAL_FOUNDATION_A11Y_PACKET_ID: &str =
    "m5-visual-foundations-accessibility-parity:stable:0001";

/// Builds the canonical, checked-in visual-foundations accessibility parity packet. This is the one source
/// of truth shared by the tests and the on-disk support export so both stay byte-aligned.
pub fn seeded_m5_visual_foundations_a11y_packet() -> VisualFoundationAccessibilityPacket {
    VisualFoundationAccessibilityPacket::new(VisualFoundationAccessibilityPacketInput {
        packet_id: VISUAL_FOUNDATION_A11Y_PACKET_ID.to_owned(),
        as_of: "2026-07-13T00:00:00Z".to_owned(),
        matrix_ref: VISUAL_FOUNDATION_A11Y_MATRIX_REF.to_owned(),
        rows: seeded_rows(),
    })
}

fn ev(id: &str) -> Vec<String> {
    vec![format!("evidence:visual-foundations-a11y:{id}")]
}

fn all_required_labels() -> Vec<M5VisualFoundationRequiredLabel> {
    M5VisualFoundationRequiredLabel::ALL.to_vec()
}

fn copy_export(fields: &[&str]) -> VisualFoundationCopyExportParity {
    VisualFoundationCopyExportParity {
        formats: vec!["text".to_owned(), "json".to_owned(), "markdown".to_owned()],
        export_fields: fields.iter().map(|f| (*f).to_owned()).collect(),
        raw_payload_only_prohibited: true,
    }
}

fn condition(
    dimension: M5VisualFoundationClaimDimension,
    state: M5VisualFoundationConditionState,
) -> VisualFoundationClaimConditionEntry {
    VisualFoundationClaimConditionEntry { dimension, state }
}

/// The two consumer surfaces every row ships to at minimum — support / release export and the general
/// product UI — so the narrowed state always reaches headless field triage.
fn base_consumers(
    extra: &[M5VisualFoundationConsumerSurface],
) -> Vec<M5VisualFoundationConsumerSurface> {
    let mut out = vec![
        M5VisualFoundationConsumerSurface::SupportExport,
        M5VisualFoundationConsumerSurface::ProductUi,
    ];
    out.extend_from_slice(extra);
    out
}

/// Disclosures for the CLI-headless and support-export surfaces. A green (full parity) row keeps full label
/// and summary parity on the narrower surfaces; a narrowed row discloses the reduced interactions it drops
/// there.
fn surface_disclosures(
    labels: &[&str],
    state: VisualFoundationNarrowingDisclosureState,
) -> Vec<VisualFoundationRenderingNarrowingDisclosure> {
    let preserved: Vec<String> = labels.iter().map(|l| (*l).to_owned()).collect();
    vec![
        VisualFoundationRenderingNarrowingDisclosure {
            rendering_surface: M5VisualFoundationRenderingSurface::CliHeadless,
            state,
            preserved_labels: preserved.clone(),
            reduced_interactions: vec!["pointer_interaction".to_owned()],
        },
        VisualFoundationRenderingNarrowingDisclosure {
            rendering_surface: M5VisualFoundationRenderingSurface::SupportExport,
            state,
            preserved_labels: preserved,
            reduced_interactions: vec!["live_hover_affordance".to_owned()],
        },
    ]
}

/// Disclosures for a full-parity (green) row: the narrower surfaces preserve full label and summary parity.
fn parity_surfaces(labels: &[&str]) -> Vec<VisualFoundationRenderingNarrowingDisclosure> {
    surface_disclosures(
        labels,
        VisualFoundationNarrowingDisclosureState::ParityPreserved,
    )
}

/// Disclosures for a narrowed (yellow) row: the narrower surfaces disclose their reduced interactions while
/// preserving labels.
fn narrowed_surfaces(labels: &[&str]) -> Vec<VisualFoundationRenderingNarrowingDisclosure> {
    surface_disclosures(
        labels,
        VisualFoundationNarrowingDisclosureState::DisclosedNarrowed,
    )
}

fn rendering_surfaces() -> Vec<M5VisualFoundationRenderingSurface> {
    vec![
        M5VisualFoundationRenderingSurface::DesktopFull,
        M5VisualFoundationRenderingSurface::CliHeadless,
        M5VisualFoundationRenderingSurface::SupportExport,
    ]
}

fn non_visual_modalities() -> Vec<M5VisualFoundationFallbackModality> {
    vec![
        M5VisualFoundationFallbackModality::List,
        M5VisualFoundationFallbackModality::Textual,
        M5VisualFoundationFallbackModality::Cli,
    ]
}

fn structured_modalities() -> Vec<M5VisualFoundationFallbackModality> {
    vec![
        M5VisualFoundationFallbackModality::Structured,
        M5VisualFoundationFallbackModality::List,
        M5VisualFoundationFallbackModality::Textual,
        M5VisualFoundationFallbackModality::Cli,
    ]
}

const REACHABLE: VisualFoundationNonVisualReachState =
    VisualFoundationNonVisualReachState::ReachableAndLabeled;
const REDUCED: VisualFoundationNonVisualReachState =
    VisualFoundationNonVisualReachState::DisclosedReducedButReachable;

fn seeded_rows() -> Vec<VisualFoundationAccessibilityRow> {
    vec![
        // Syntax token (scopes stay distinct from diagnostics) — the syntax palette keeps its keyword /
        // string / comment / identifier scopes distinct from the diagnostics palette, so it is a trusted
        // visual surface reachable on every surface with no narrowing (green). Structure-heavy: its scope
        // set binds to a flat list / textual path.
        VisualFoundationAccessibilityRow {
            record_kind: VISUAL_FOUNDATION_A11Y_ROW_RECORD_KIND.to_owned(),
            schema_version: VISUAL_FOUNDATION_A11Y_SCHEMA_VERSION,
            row_id: "a11y:syntax-token-scopes-distinct-from-diagnostics".to_owned(),
            foundation_family: M5VisualFoundationFamily::SyntaxToken,
            source_family_schema_ref: M5VisualFoundationFamily::SyntaxToken
                .canonical_domain_schema_ref()
                .to_owned(),
            foundation_context_ref: "editor:syntax-token:0001".to_owned(),
            fallback_modalities: structured_modalities(),
            reaches_canonical_truth: true,
            keyboard_reach: REACHABLE,
            screen_reader_reach: REACHABLE,
            high_zoom_reach: REACHABLE,
            reduced_motion_reach: REACHABLE,
            cli_reach: REACHABLE,
            export_summary: VisualFoundationExportSummaryState::ReconstructableWithoutRawPayload,
            export_summary_ref: "summary:syntax-token-scopes-distinct-from-diagnostics:a11y"
                .to_owned(),
            copy_export: copy_export(&[
                "foundation_identity",
                "semantic_role",
                "token_reference",
                "diagnostics_separation",
            ]),
            full_ready_claim: M5VisualFoundationA11yClaim::TrustedVisualSurface,
            claim_conditions: vec![condition(
                M5VisualFoundationClaimDimension::SyntaxSeparationClarity,
                M5VisualFoundationConditionState::FullyQualified,
            )],
            claim_narrow: None,
            truth_preserved: true,
            rendering_surfaces: rendering_surfaces(),
            narrowing_disclosures: parity_surfaces(&[
                "foundation_identity",
                "semantic_role",
                "token_reference",
            ]),
            required_labels: all_required_labels(),
            consumer_surfaces: base_consumers(&[
                M5VisualFoundationConsumerSurface::EditorUi,
                M5VisualFoundationConsumerSurface::ReviewUi,
            ]),
            source_refs: vec![
                "UX Style Guide §9 — Syntax token guidance".to_owned(),
                VISUAL_FOUNDATION_A11Y_DOC_REF.to_owned(),
            ],
            observed_at: "2026-07-13T00:00:00Z".to_owned(),
            evidence_refs: ev("syntax-token-scopes-distinct-from-diagnostics"),
        },
        // Spacing / sizing / radii / elevation geometry (density-aware and machine-readable) — the geometry
        // stays density-aware and machine-readable rather than a local fork, so it is a self-sufficient
        // reviewable visual surface a user can inspect, but its narrower non-visual traversal discloses a
        // reduced high-zoom reflow walk (yellow).
        VisualFoundationAccessibilityRow {
            record_kind: VISUAL_FOUNDATION_A11Y_ROW_RECORD_KIND.to_owned(),
            schema_version: VISUAL_FOUNDATION_A11Y_SCHEMA_VERSION,
            row_id: "a11y:spacing-sizing-radii-elevation-geometry-baseline-stable".to_owned(),
            foundation_family: M5VisualFoundationFamily::SpacingSizingRadiiElevation,
            source_family_schema_ref: M5VisualFoundationFamily::SpacingSizingRadiiElevation
                .canonical_domain_schema_ref()
                .to_owned(),
            foundation_context_ref: "shell:spacing-sizing-radii-elevation:0002".to_owned(),
            fallback_modalities: non_visual_modalities(),
            reaches_canonical_truth: true,
            keyboard_reach: REACHABLE,
            screen_reader_reach: REACHABLE,
            high_zoom_reach: REDUCED,
            reduced_motion_reach: REACHABLE,
            cli_reach: REACHABLE,
            export_summary: VisualFoundationExportSummaryState::ReconstructableWithoutRawPayload,
            export_summary_ref:
                "summary:spacing-sizing-radii-elevation-geometry-baseline-stable:a11y".to_owned(),
            copy_export: copy_export(&[
                "foundation_identity",
                "semantic_role",
                "token_reference",
                "density_context",
            ]),
            full_ready_claim: M5VisualFoundationA11yClaim::ReviewableVisualSurface,
            claim_conditions: vec![condition(
                M5VisualFoundationClaimDimension::GeometryBaselineClarity,
                M5VisualFoundationConditionState::FullyQualified,
            )],
            claim_narrow: None,
            truth_preserved: true,
            rendering_surfaces: rendering_surfaces(),
            narrowing_disclosures: narrowed_surfaces(&[
                "foundation_identity",
                "semantic_role",
                "density_context",
            ]),
            required_labels: all_required_labels(),
            consumer_surfaces: base_consumers(&[
                M5VisualFoundationConsumerSurface::ShellUi,
                M5VisualFoundationConsumerSurface::DataUi,
            ]),
            source_refs: vec![
                "UI/UX Spec §8.4 — Spacing / sizing / shape / elevation".to_owned(),
                VISUAL_FOUNDATION_A11Y_DOC_REF.to_owned(),
            ],
            observed_at: "2026-07-13T00:00:00Z".to_owned(),
            evidence_refs: ev("spacing-sizing-radii-elevation-geometry-baseline-stable"),
        },
        // Color system (contrast evidence stale) — the color system's contrast evidence is stale, so it
        // auto-narrows to a contrast-unverified projection that keeps the last-known semantic role and
        // non-color cue visible without relying on hue alone, never a fresh, authoritative status color
        // (yellow).
        VisualFoundationAccessibilityRow {
            record_kind: VISUAL_FOUNDATION_A11Y_ROW_RECORD_KIND.to_owned(),
            schema_version: VISUAL_FOUNDATION_A11Y_SCHEMA_VERSION,
            row_id: "a11y:color-system-contrast-evidence-stale".to_owned(),
            foundation_family: M5VisualFoundationFamily::ColorSystem,
            source_family_schema_ref: M5VisualFoundationFamily::ColorSystem
                .canonical_domain_schema_ref()
                .to_owned(),
            foundation_context_ref: "shell:color-system:0003".to_owned(),
            fallback_modalities: non_visual_modalities(),
            reaches_canonical_truth: true,
            keyboard_reach: REACHABLE,
            screen_reader_reach: REACHABLE,
            high_zoom_reach: REACHABLE,
            reduced_motion_reach: REACHABLE,
            cli_reach: REACHABLE,
            export_summary: VisualFoundationExportSummaryState::ReconstructableWithoutRawPayload,
            export_summary_ref: "summary:color-system-contrast-evidence-stale:a11y".to_owned(),
            copy_export: copy_export(&[
                "foundation_identity",
                "semantic_role",
                "non_color_cue",
                "last_known_contrast_pairing",
            ]),
            full_ready_claim: M5VisualFoundationA11yClaim::TrustedVisualSurface,
            claim_conditions: vec![condition(
                M5VisualFoundationClaimDimension::ColorContrastClarity,
                M5VisualFoundationConditionState::ContrastEvidenceStale,
            )],
            claim_narrow: Some(VisualFoundationClaimAutoNarrow {
                narrowed_to: M5VisualFoundationA11yClaim::ContrastUnverifiedProjection,
                binding_dimension: M5VisualFoundationClaimDimension::ColorContrastClarity,
                trigger: M5VisualFoundationDowngradeTrigger::StatusOrTrustCollapsedToColorOnly,
                narrowed_label:
                    "This color system's contrast evidence is stale or unresolved — shown as a contrast-unverified projection that keeps the last-known semantic role and its non-color cue visible without relying on hue alone, never presenting a stale palette as a fresh, authoritative status color"
                        .to_owned(),
                preserves_canonical_identity: true,
                preserves_truth_continuity: true,
            }),
            truth_preserved: true,
            rendering_surfaces: rendering_surfaces(),
            narrowing_disclosures: narrowed_surfaces(&[
                "foundation_identity",
                "semantic_role",
                "non_color_cue",
            ]),
            required_labels: all_required_labels(),
            consumer_surfaces: base_consumers(&[
                M5VisualFoundationConsumerSurface::ShellUi,
                M5VisualFoundationConsumerSurface::SettingsUi,
            ]),
            source_refs: vec![
                "UI/UX Spec §8.2 — Palette / color tokens".to_owned(),
                VISUAL_FOUNDATION_A11Y_DOC_REF.to_owned(),
            ],
            observed_at: "2026-07-13T00:00:00Z".to_owned(),
            evidence_refs: ev("color-system-contrast-evidence-stale"),
        },
        // Semantic theme token (theme pair incomplete) — the token's dark / light / high-contrast pair
        // cannot be confirmed complete, so it auto-narrows to a theme-pair-unverified projection that keeps
        // the last-known stable token role explicit, never an incomplete theme pair shown as fully stable
        // (yellow).
        VisualFoundationAccessibilityRow {
            record_kind: VISUAL_FOUNDATION_A11Y_ROW_RECORD_KIND.to_owned(),
            schema_version: VISUAL_FOUNDATION_A11Y_SCHEMA_VERSION,
            row_id: "a11y:semantic-theme-token-theme-pair-incomplete".to_owned(),
            foundation_family: M5VisualFoundationFamily::SemanticThemeToken,
            source_family_schema_ref: M5VisualFoundationFamily::SemanticThemeToken
                .canonical_domain_schema_ref()
                .to_owned(),
            foundation_context_ref: "shell:semantic-theme-token:0004".to_owned(),
            fallback_modalities: non_visual_modalities(),
            reaches_canonical_truth: true,
            keyboard_reach: REACHABLE,
            screen_reader_reach: REACHABLE,
            high_zoom_reach: REACHABLE,
            reduced_motion_reach: REACHABLE,
            cli_reach: REACHABLE,
            export_summary: VisualFoundationExportSummaryState::ReconstructableWithoutRawPayload,
            export_summary_ref: "summary:semantic-theme-token-theme-pair-incomplete:a11y"
                .to_owned(),
            copy_export: copy_export(&[
                "foundation_identity",
                "semantic_role",
                "token_reference",
                "last_known_theme_variant",
            ]),
            full_ready_claim: M5VisualFoundationA11yClaim::TrustedVisualSurface,
            claim_conditions: vec![condition(
                M5VisualFoundationClaimDimension::ThemePairParityClarity,
                M5VisualFoundationConditionState::ThemePairEvidenceIncomplete,
            )],
            claim_narrow: Some(VisualFoundationClaimAutoNarrow {
                narrowed_to: M5VisualFoundationA11yClaim::ThemePairUnverifiedProjection,
                binding_dimension: M5VisualFoundationClaimDimension::ThemePairParityClarity,
                trigger: M5VisualFoundationDowngradeTrigger::ThemePairIncomplete,
                narrowed_label:
                    "This semantic theme token cannot confirm a complete dark / light / high-contrast pair — shown as a theme-pair-unverified projection that keeps the last-known stable token role and theme variant explicit, never presenting an incomplete theme pair as fully stable"
                        .to_owned(),
                preserves_canonical_identity: true,
                preserves_truth_continuity: true,
            }),
            truth_preserved: true,
            rendering_surfaces: rendering_surfaces(),
            narrowing_disclosures: narrowed_surfaces(&[
                "foundation_identity",
                "semantic_role",
                "token_reference",
            ]),
            required_labels: all_required_labels(),
            consumer_surfaces: base_consumers(&[
                M5VisualFoundationConsumerSurface::ShellUi,
                M5VisualFoundationConsumerSurface::EditorUi,
            ]),
            source_refs: vec![
                "UI/UX Spec §8.1 — Theme philosophy / semantic tokens".to_owned(),
                VISUAL_FOUNDATION_A11Y_DOC_REF.to_owned(),
            ],
            observed_at: "2026-07-13T00:00:00Z".to_owned(),
            evidence_refs: ev("semantic-theme-token-theme-pair-incomplete"),
        },
        // Diff token (diagnostics separation unconfirmed) — structure-heavy (add / remove / context bands);
        // the diff palette's separation from the diagnostics palette cannot be confirmed, so it auto-narrows
        // to a semantic-separation-unverified projection that keeps the band role inspectable, never a diff
        // palette shown as diagnostics-clean (yellow).
        VisualFoundationAccessibilityRow {
            record_kind: VISUAL_FOUNDATION_A11Y_ROW_RECORD_KIND.to_owned(),
            schema_version: VISUAL_FOUNDATION_A11Y_SCHEMA_VERSION,
            row_id: "a11y:diff-token-diagnostics-separation-unconfirmed".to_owned(),
            foundation_family: M5VisualFoundationFamily::DiffToken,
            source_family_schema_ref: M5VisualFoundationFamily::DiffToken
                .canonical_domain_schema_ref()
                .to_owned(),
            foundation_context_ref: "review:diff-token:0005".to_owned(),
            fallback_modalities: structured_modalities(),
            reaches_canonical_truth: true,
            keyboard_reach: REACHABLE,
            screen_reader_reach: REACHABLE,
            high_zoom_reach: REACHABLE,
            reduced_motion_reach: REACHABLE,
            cli_reach: REACHABLE,
            export_summary: VisualFoundationExportSummaryState::ReconstructableWithoutRawPayload,
            export_summary_ref: "summary:diff-token-diagnostics-separation-unconfirmed:a11y"
                .to_owned(),
            copy_export: copy_export(&[
                "foundation_identity",
                "semantic_role",
                "token_reference",
                "diagnostics_separation",
            ]),
            full_ready_claim: M5VisualFoundationA11yClaim::TrustedVisualSurface,
            claim_conditions: vec![condition(
                M5VisualFoundationClaimDimension::DiffSeparationClarity,
                M5VisualFoundationConditionState::SemanticSeparationUnconfirmed,
            )],
            claim_narrow: Some(VisualFoundationClaimAutoNarrow {
                narrowed_to: M5VisualFoundationA11yClaim::SemanticSeparationUnverifiedProjection,
                binding_dimension: M5VisualFoundationClaimDimension::DiffSeparationClarity,
                trigger:
                    M5VisualFoundationDowngradeTrigger::SyntaxOrDiffPaletteCollidedWithDiagnostics,
                narrowed_label:
                    "This diff palette cannot confirm its separation from the diagnostics palette — shown as a semantic-separation-unverified projection that keeps the add / remove / context band role inspectable, never presenting a diff palette as diagnostics-clean when a collision cannot be ruled out"
                        .to_owned(),
                preserves_canonical_identity: true,
                preserves_truth_continuity: true,
            }),
            truth_preserved: true,
            rendering_surfaces: rendering_surfaces(),
            narrowing_disclosures: narrowed_surfaces(&[
                "foundation_identity",
                "semantic_role",
                "token_reference",
            ]),
            required_labels: all_required_labels(),
            consumer_surfaces: base_consumers(&[
                M5VisualFoundationConsumerSurface::ReviewUi,
                M5VisualFoundationConsumerSurface::EditorUi,
            ]),
            source_refs: vec![
                "UX Style Guide §9 — Diff token guidance".to_owned(),
                VISUAL_FOUNDATION_A11Y_DOC_REF.to_owned(),
            ],
            observed_at: "2026-07-13T00:00:00Z".to_owned(),
            evidence_refs: ev("diff-token-diagnostics-separation-unconfirmed"),
        },
        // Chart token (non-color encoding unconfirmed) — structure-heavy (series / legend); the chart
        // palette's non-color encoding cannot be confirmed, so it auto-narrows to a chart-encoding-unverified
        // projection that keeps the legend / marker / pattern channel inspectable, never a chart shown as
        // decodable when its meaning may depend on color alone (yellow). Its animated affordance narrows the
        // reduced-motion path to a disclosed reduction.
        VisualFoundationAccessibilityRow {
            record_kind: VISUAL_FOUNDATION_A11Y_ROW_RECORD_KIND.to_owned(),
            schema_version: VISUAL_FOUNDATION_A11Y_SCHEMA_VERSION,
            row_id: "a11y:chart-token-non-color-encoding-unconfirmed".to_owned(),
            foundation_family: M5VisualFoundationFamily::ChartToken,
            source_family_schema_ref: M5VisualFoundationFamily::ChartToken
                .canonical_domain_schema_ref()
                .to_owned(),
            foundation_context_ref: "data:chart-token:0006".to_owned(),
            fallback_modalities: structured_modalities(),
            reaches_canonical_truth: true,
            keyboard_reach: REACHABLE,
            screen_reader_reach: REACHABLE,
            high_zoom_reach: REACHABLE,
            reduced_motion_reach: REDUCED,
            cli_reach: REACHABLE,
            export_summary: VisualFoundationExportSummaryState::ReconstructableWithoutRawPayload,
            export_summary_ref: "summary:chart-token-non-color-encoding-unconfirmed:a11y"
                .to_owned(),
            copy_export: copy_export(&[
                "foundation_identity",
                "semantic_role",
                "token_reference",
                "non_color_cue",
            ]),
            full_ready_claim: M5VisualFoundationA11yClaim::TrustedVisualSurface,
            claim_conditions: vec![condition(
                M5VisualFoundationClaimDimension::ChartEncodingClarity,
                M5VisualFoundationConditionState::ChartEncodingUnconfirmed,
            )],
            claim_narrow: Some(VisualFoundationClaimAutoNarrow {
                narrowed_to: M5VisualFoundationA11yClaim::ChartEncodingUnverifiedProjection,
                binding_dimension: M5VisualFoundationClaimDimension::ChartEncodingClarity,
                trigger: M5VisualFoundationDowngradeTrigger::ChartMeaningDependedOnColorAlone,
                narrowed_label:
                    "This chart palette cannot confirm a non-color encoding — shown as a chart-encoding-unverified projection that keeps the legend / marker / pattern channel inspectable, never presenting a chart as decodable when its meaning may depend on color alone"
                        .to_owned(),
                preserves_canonical_identity: true,
                preserves_truth_continuity: true,
            }),
            truth_preserved: true,
            rendering_surfaces: rendering_surfaces(),
            narrowing_disclosures: narrowed_surfaces(&[
                "foundation_identity",
                "semantic_role",
                "non_color_cue",
            ]),
            required_labels: all_required_labels(),
            consumer_surfaces: base_consumers(&[
                M5VisualFoundationConsumerSurface::DataUi,
                M5VisualFoundationConsumerSurface::DocsUi,
            ]),
            source_refs: vec![
                "UX Style Guide §9 — Chart token guidance".to_owned(),
                VISUAL_FOUNDATION_A11Y_DOC_REF.to_owned(),
            ],
            observed_at: "2026-07-13T00:00:00Z".to_owned(),
            evidence_refs: ev("chart-token-non-color-encoding-unconfirmed"),
        },
        // Typography (readability evidence stale) — the type scale's readability evidence is stale, so it
        // auto-narrows to a text-readability-unverified projection that preserves the last-known type scale,
        // line-height, tabular numerals, and font stack, never a drifted type scale shown as
        // readability-stable (yellow). Its dense reflow narrows the high-zoom legibility to a disclosed
        // reduction.
        VisualFoundationAccessibilityRow {
            record_kind: VISUAL_FOUNDATION_A11Y_ROW_RECORD_KIND.to_owned(),
            schema_version: VISUAL_FOUNDATION_A11Y_SCHEMA_VERSION,
            row_id: "a11y:typography-readability-evidence-stale".to_owned(),
            foundation_family: M5VisualFoundationFamily::Typography,
            source_family_schema_ref: M5VisualFoundationFamily::Typography
                .canonical_domain_schema_ref()
                .to_owned(),
            foundation_context_ref: "docs:typography:0007".to_owned(),
            fallback_modalities: non_visual_modalities(),
            reaches_canonical_truth: true,
            keyboard_reach: REACHABLE,
            screen_reader_reach: REACHABLE,
            high_zoom_reach: REDUCED,
            reduced_motion_reach: REACHABLE,
            cli_reach: REACHABLE,
            export_summary: VisualFoundationExportSummaryState::ReconstructableWithoutRawPayload,
            export_summary_ref: "summary:typography-readability-evidence-stale:a11y".to_owned(),
            copy_export: copy_export(&[
                "foundation_identity",
                "semantic_role",
                "token_reference",
                "last_known_type_scale",
            ]),
            full_ready_claim: M5VisualFoundationA11yClaim::TrustedVisualSurface,
            claim_conditions: vec![condition(
                M5VisualFoundationClaimDimension::TextReadabilityClarity,
                M5VisualFoundationConditionState::TextReadabilityStale,
            )],
            claim_narrow: Some(VisualFoundationClaimAutoNarrow {
                narrowed_to: M5VisualFoundationA11yClaim::TextReadabilityUnverifiedProjection,
                binding_dimension: M5VisualFoundationClaimDimension::TextReadabilityClarity,
                trigger: M5VisualFoundationDowngradeTrigger::TypographyScaleDrifted,
                narrowed_label:
                    "This typography scale's readability evidence is stale — shown as a text-readability-unverified projection that preserves the last-known type scale, line-height, tabular numerals, and font stack, never presenting a drifted type scale as readability-stable"
                        .to_owned(),
                preserves_canonical_identity: true,
                preserves_truth_continuity: true,
            }),
            truth_preserved: true,
            rendering_surfaces: rendering_surfaces(),
            narrowing_disclosures: narrowed_surfaces(&[
                "foundation_identity",
                "semantic_role",
                "token_reference",
            ]),
            required_labels: all_required_labels(),
            consumer_surfaces: base_consumers(&[
                M5VisualFoundationConsumerSurface::DocsUi,
                M5VisualFoundationConsumerSurface::EditorUi,
            ]),
            source_refs: vec![
                "UI/UX Spec §8.3 — Typography".to_owned(),
                VISUAL_FOUNDATION_A11Y_DOC_REF.to_owned(),
            ],
            observed_at: "2026-07-13T00:00:00Z".to_owned(),
            evidence_refs: ev("typography-readability-evidence-stale"),
        },
        // Hit target (baseline disclosed partial) — the hit-target rule can only disclose a partial density /
        // minimum-target baseline, so it auto-narrows to a geometry-baseline-disclosed projection that
        // discloses the partial baseline alongside the last-known minimum, never hiding the reduced density
        // behind a complete-baseline claim (yellow). A partial baseline disclosure is an honest
        // disclosed-absence operation, not a trusted overstatement.
        VisualFoundationAccessibilityRow {
            record_kind: VISUAL_FOUNDATION_A11Y_ROW_RECORD_KIND.to_owned(),
            schema_version: VISUAL_FOUNDATION_A11Y_SCHEMA_VERSION,
            row_id: "a11y:hit-target-baseline-disclosed-partial".to_owned(),
            foundation_family: M5VisualFoundationFamily::HitTarget,
            source_family_schema_ref: M5VisualFoundationFamily::HitTarget
                .canonical_domain_schema_ref()
                .to_owned(),
            foundation_context_ref: "settings:hit-target:0008".to_owned(),
            fallback_modalities: non_visual_modalities(),
            reaches_canonical_truth: true,
            keyboard_reach: REACHABLE,
            screen_reader_reach: REACHABLE,
            high_zoom_reach: REACHABLE,
            reduced_motion_reach: REACHABLE,
            cli_reach: REACHABLE,
            export_summary: VisualFoundationExportSummaryState::ReconstructableWithoutRawPayload,
            export_summary_ref: "summary:hit-target-baseline-disclosed-partial:a11y".to_owned(),
            copy_export: copy_export(&[
                "foundation_identity",
                "semantic_role",
                "token_reference",
                "partial_or_disclosed_note",
            ]),
            full_ready_claim: M5VisualFoundationA11yClaim::TrustedVisualSurface,
            claim_conditions: vec![condition(
                M5VisualFoundationClaimDimension::HitTargetMinimumClarity,
                M5VisualFoundationConditionState::GeometryBaselineDisclosedPartial,
            )],
            claim_narrow: Some(VisualFoundationClaimAutoNarrow {
                narrowed_to: M5VisualFoundationA11yClaim::GeometryBaselineDisclosedProjection,
                binding_dimension: M5VisualFoundationClaimDimension::HitTargetMinimumClarity,
                trigger: M5VisualFoundationDowngradeTrigger::ProofStale,
                narrowed_label:
                    "This hit-target rule can only disclose a partial density / minimum-target baseline — shown as a geometry-baseline-disclosed projection that discloses the partial baseline alongside the last-known supported minimum, never hiding the reduced density behind a complete-baseline claim"
                        .to_owned(),
                preserves_canonical_identity: true,
                preserves_truth_continuity: true,
            }),
            truth_preserved: true,
            rendering_surfaces: rendering_surfaces(),
            narrowing_disclosures: narrowed_surfaces(&[
                "foundation_identity",
                "semantic_role",
                "token_reference",
            ]),
            required_labels: all_required_labels(),
            consumer_surfaces: base_consumers(&[
                M5VisualFoundationConsumerSurface::SettingsUi,
                M5VisualFoundationConsumerSurface::CliExport,
            ]),
            source_refs: vec![
                "UX Style Guide §7 — Minimum hit-target".to_owned(),
                VISUAL_FOUNDATION_A11Y_DOC_REF.to_owned(),
            ],
            observed_at: "2026-07-13T00:00:00Z".to_owned(),
            evidence_refs: ev("hit-target-baseline-disclosed-partial"),
        },
    ]
}
