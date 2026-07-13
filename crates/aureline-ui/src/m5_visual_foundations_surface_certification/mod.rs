//! M05-1147 surface certification over the frozen M5 color-system / semantic-theme-token /
//! syntax-token / diff-token / chart-token / typography / spacing-sizing-radii-elevation / hit-target
//! visual-foundation matrix.
//!
//! Where the freeze matrix ([`crate::m5_visual_foundation_matrix`]) defines the eight governed
//! visual-foundation families, the M05-1141..1144 implement lanes narrow each one, the M05-1145
//! shared consumer lane aligns their vocabulary across surfaces, and the M05-1146 accessibility lane
//! ([`crate::m5_visual_foundations_accessibility_parity_and_narrowing_when_visual_foundation_truth_is_stale`])
//! proves high-contrast / high-zoom / reduced-motion / CLI-export parity and per-family auto-narrowing,
//! this closing capstone *certifies* that the shared visual-foundation truth holds on every claimed M5
//! shell / editor / review / data / docs operating profile — and auto-narrows any profile that cannot
//! sustain it.
//!
//! It is keyed on the claimed **profile** a user, reviewer, or support engineer reads a color, token,
//! typography, or geometry surface through (a live, first-party trusted visual surface; a reviewable
//! geometry structure; a stale-contrast color surface; an unpaired theme-token surface; a
//! colliding-diff surface; a color-only chart surface; a drifting-typography surface; and an
//! undisclosed hit-target surface), not on foundation family or implement lane. Each
//! [`VisualFoundationProfileCertificationRow`] certifies one profile across eight truth axes —
//! visual, keyboard, screen-reader, high-zoom-reflow, reduced-motion, CLI/export, degraded-state, and
//! visual-foundation-component-truth behavior — and either passes (green), auto-narrows its visual
//! claim to the weakest supported ceiling (yellow), or is blocked (red) when a degraded axis is hidden
//! behind a fresh trusted claim inherited from a healthier profile.
//!
//! The invariant is: **a degraded axis must produce a visible claim narrowing**. A profile that keeps
//! a `TrustedVisualSurface` / `ReviewableVisualSurface` claim while one of its truth axes is not
//! current is over-claiming and blocks; a profile that discloses the reduction by narrowing its claim
//! (with a bound reason and a frozen downgrade trigger) is honestly yellow. Only a live, first-party
//! trusted visual profile may certify a `TrustedVisualSurface` claim — a reviewable, stale-contrast,
//! unpaired, colliding, color-only, drifting, or undisclosed profile that keeps a trusted claim is
//! over-reaching and blocks. The always-on CLI/export axis must always stay certified so support and
//! automation can reconstruct the canonical foundation identity, semantic role, token reference, theme
//! variant, contrast pairing, and geometry baseline from the same foundation the user saw.
//!
//! The B136 hard invariants are enforced per row: no profile may collapse status or trust meaning into
//! a color-only cue, let a syntax or diff palette collide with diagnostics, shrink a hit target below
//! its supported minimum, let chart meaning depend on color alone, or fork local spacing or elevation
//! from the shared geometry. A profile that breaches any invariant blocks (red).
//!
//! Every row cites exactly one canonical visual-foundation proof bundle
//! ([`VISUAL_FOUNDATION_CERT_CANONICAL_BUNDLE_REF`]) — the frozen visual-foundation matrix proof —
//! rather than cloning per-profile evidence. The packet is metadata-only: raw hex values, font blobs,
//! credentials, secrets, and endpoint refs never cross this boundary.
//!
//! The boundary schema is
//! [`schemas/design-system/m5-visual-foundations-surface-certification.schema.json`](../../../../schemas/design-system/m5-visual-foundations-surface-certification.schema.json).
//! The contract doc is
//! [`docs/design-system/m5_visual_foundations_surface_certification.md`](../../../../docs/design-system/m5_visual_foundations_surface_certification.md).

#[cfg(test)]
mod tests;

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

use crate::m5_visual_foundation_matrix as matrix;
use crate::m5_visual_foundations_accessibility_parity_and_narrowing_when_visual_foundation_truth_is_stale as a11y;
use a11y::M5VisualFoundationA11yClaim;
use matrix::{M5VisualFoundationDowngradeTrigger, M5VisualFoundationFamily};

/// Schema version stamped on the M05-1147 certification packet.
pub const VISUAL_FOUNDATION_CERT_SCHEMA_VERSION: u32 = 1;

/// Stable record-kind tag carried by [`VisualFoundationProfileCertificationPacket`].
pub const VISUAL_FOUNDATION_CERT_RECORD_KIND: &str =
    "m5_visual_foundations_surface_certification_packet";

/// Stable record-kind tag carried by each [`VisualFoundationProfileCertificationRow`].
pub const VISUAL_FOUNDATION_CERT_ROW_RECORD_KIND: &str =
    "m5_visual_foundations_surface_certification_row";

/// Repo-relative path of the boundary schema.
pub const VISUAL_FOUNDATION_CERT_SCHEMA_REF: &str =
    "schemas/design-system/m5-visual-foundations-surface-certification.schema.json";

/// Repo-relative path of the contract doc.
pub const VISUAL_FOUNDATION_CERT_DOC_REF: &str =
    "docs/design-system/m5_visual_foundations_surface_certification.md";

/// Repo-relative path of the frozen visual-foundation matrix schema the certified profiles render.
pub const VISUAL_FOUNDATION_CERT_MATRIX_REF: &str = matrix::M5_VISUAL_FOUNDATION_MATRIX_SCHEMA_REF;

/// The one canonical visual-foundation proof bundle every certified profile cites as its
/// first-resolved foundation truth. All eight profiles point back to it rather than cloning
/// per-profile evidence.
pub const VISUAL_FOUNDATION_CERT_CANONICAL_BUNDLE_REF: &str =
    matrix::M5_VISUAL_FOUNDATION_ARTIFACT_REF;

/// The M05-1146 accessibility support export the certification builds on. Recorded as a supporting
/// evidence ref on every row.
pub const VISUAL_FOUNDATION_CERT_A11Y_BUNDLE_REF: &str = a11y::VISUAL_FOUNDATION_A11Y_ARTIFACT_REF;

/// Repo-relative path of the checked support-export artifact (the `include_str!` canonical).
pub const VISUAL_FOUNDATION_CERT_ARTIFACT_REF: &str =
    "artifacts/release/m5-visual-foundations-surface-certification/support_export.json";

/// Repo-relative path of the checked machine-readable matrix CSV.
pub const VISUAL_FOUNDATION_CERT_CSV_REF: &str =
    "artifacts/release/m5-visual-foundations-surface-certification/matrix.csv";

/// Repo-relative path of the checked Markdown report.
pub const VISUAL_FOUNDATION_CERT_REPORT_REF: &str =
    "artifacts/release/m5-visual-foundations-surface-certification.md";

/// Stable packet id for the checked-in certification bundle.
pub const VISUAL_FOUNDATION_CERT_PACKET_ID: &str =
    "m5-visual-foundations-surface-certification:stable:0001";

/// The eight claimed M5 shell / editor / review / data / docs visual operating profiles this capstone
/// certifies. Keyed on the profile a user, reviewer, or support engineer reads a color, token,
/// typography, or geometry surface through, not on the reusable foundation family it renders. Only a
/// live, first-party trusted visual profile may certify a trusted visual surface.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5VisualFoundationCertifiedProfile {
    /// A live, first-party, fully-current visual surface — a syntax / semantic palette rendering the
    /// trusted, contrast-proven, diagnostics-separated foundation exactly right now.
    LiveTrustedVisualSurface,
    /// A reviewable geometry structure: a self-sufficient, inspectable spacing / sizing / radii /
    /// elevation surface a user can review, never itself an authoritative, decision-driving visual
    /// surface.
    ReviewableGeometryStructure,
    /// A color-system surface whose contrast evidence is stale; the claim narrows to a
    /// contrast-unverified projection with the last-known canonical role preserved, never a fresh,
    /// color-only status shown as authoritative.
    StaleContrastColorSurface,
    /// A semantic-theme-token surface whose dark / light / high-contrast pair cannot be confirmed; the
    /// claim narrows to a theme-pair-unverified projection that keeps the last-known token role
    /// explicit, never a single-mode token shown as fully paired.
    UnpairedThemeTokenSurface,
    /// A diff-token surface whose diagnostics separation cannot be confirmed; the claim narrows to a
    /// semantic-separation-unverified projection that keeps the add / remove / context meaning
    /// inspectable, never a diff palette that collides with diagnostics.
    CollidingDiffSurface,
    /// A chart-token surface whose non-color encoding is unconfirmed; the claim narrows to a
    /// chart-encoding-unverified projection that keeps the legend / pattern / marker cue disclosed,
    /// never a chart whose meaning depends on color alone.
    ColorOnlyChartSurface,
    /// A typography surface whose readability evidence is stale; the claim narrows to a
    /// text-readability-unverified projection that keeps the type scale and tabular numerals legible,
    /// never a drifted scale shown as fully readable.
    DriftingTypographySurface,
    /// A hit-target / geometry-baseline surface that can only disclose a partial baseline; the claim
    /// narrows to a geometry-baseline-disclosed projection disclosing the partial minimum-target
    /// baseline.
    UndisclosedHitTargetSurface,
}

impl M5VisualFoundationCertifiedProfile {
    /// Every certified profile, in declaration order.
    pub const ALL: [M5VisualFoundationCertifiedProfile; 8] = [
        M5VisualFoundationCertifiedProfile::LiveTrustedVisualSurface,
        M5VisualFoundationCertifiedProfile::ReviewableGeometryStructure,
        M5VisualFoundationCertifiedProfile::StaleContrastColorSurface,
        M5VisualFoundationCertifiedProfile::UnpairedThemeTokenSurface,
        M5VisualFoundationCertifiedProfile::CollidingDiffSurface,
        M5VisualFoundationCertifiedProfile::ColorOnlyChartSurface,
        M5VisualFoundationCertifiedProfile::DriftingTypographySurface,
        M5VisualFoundationCertifiedProfile::UndisclosedHitTargetSurface,
    ];

    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LiveTrustedVisualSurface => "live_trusted_visual_surface",
            Self::ReviewableGeometryStructure => "reviewable_geometry_structure",
            Self::StaleContrastColorSurface => "stale_contrast_color_surface",
            Self::UnpairedThemeTokenSurface => "unpaired_theme_token_surface",
            Self::CollidingDiffSurface => "colliding_diff_surface",
            Self::ColorOnlyChartSurface => "color_only_chart_surface",
            Self::DriftingTypographySurface => "drifting_typography_surface",
            Self::UndisclosedHitTargetSurface => "undisclosed_hit_target_surface",
        }
    }

    /// True only for the live, first-party trusted visual surface profile. A trusted visual surface may
    /// be certified on this profile alone; every other profile is at most a reviewable visual structure
    /// or a narrowed projection.
    pub const fn is_live_trusted_visual_surface(self) -> bool {
        matches!(self, Self::LiveTrustedVisualSurface)
    }
}

/// The eight truth axes a certified profile is scored on. These are exactly the parity dimensions the
/// spec requires verifying — visual, keyboard, screen-reader, high-zoom reflow, reduced-motion,
/// CLI/export, degraded-state, and visual-foundation-component-truth behavior. The CLI/export axis is
/// always-on and must stay certified for every profile.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum VisualFoundationCertificationAxis {
    /// Visual parity: canonical role, semantic meaning, token reference, theme variant, and contrast
    /// pairing are shown on the primary surface without relying on color alone.
    Visual,
    /// Keyboard-reach parity: the same foundation truth and its bound controls are reachable and
    /// operable without a pointer, never hover-only.
    Keyboard,
    /// Screen-reader parity: the same truth is announced non-visually, never relying on color, motion,
    /// or a chrome glyph alone.
    ScreenReader,
    /// High-zoom reflow parity: the same truth reflows legibly at high zoom rather than clipping the
    /// role, token reference, type scale, or geometry baseline.
    HighZoomReflow,
    /// Reduced-motion parity: the same truth is legible and usable with reduced motion, never
    /// motion-only.
    ReducedMotion,
    /// CLI / export parity (always-on): the certified profile state is reconstructable as
    /// text / JSON / Markdown for support and automation.
    CliExport,
    /// Degraded-state parity: stale contrast evidence, an incomplete theme pair, unconfirmed semantic
    /// separation, unconfirmed chart encoding, stale readability evidence, or a partially-disclosed
    /// geometry baseline honestly downgrades a `TrustedVisualSurface` / `ReviewableVisualSurface` claim
    /// rather than reading as a fresh, authoritative visual surface.
    DegradedState,
    /// Visual-foundation-component-truth parity: canonical role, semantic meaning, token reference,
    /// theme variant, contrast pairing, non-color cue, type scale, and geometry baseline stay explicit
    /// and never collapse status or trust meaning into color alone, let a syntax or diff palette
    /// collide with diagnostics, shrink a hit target below its supported minimum, let chart meaning
    /// depend on color alone, or fork local spacing or elevation from the shared geometry.
    VisualFoundationComponentTruth,
}

impl VisualFoundationCertificationAxis {
    /// Every certification axis, in declaration order.
    pub const ALL: [VisualFoundationCertificationAxis; 8] = [
        VisualFoundationCertificationAxis::Visual,
        VisualFoundationCertificationAxis::Keyboard,
        VisualFoundationCertificationAxis::ScreenReader,
        VisualFoundationCertificationAxis::HighZoomReflow,
        VisualFoundationCertificationAxis::ReducedMotion,
        VisualFoundationCertificationAxis::CliExport,
        VisualFoundationCertificationAxis::DegradedState,
        VisualFoundationCertificationAxis::VisualFoundationComponentTruth,
    ];

    /// The always-on CLI/export axis that must stay certified on every row.
    pub const fn is_always_on(self) -> bool {
        matches!(self, Self::CliExport)
    }

    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Visual => "visual",
            Self::Keyboard => "keyboard",
            Self::ScreenReader => "screen_reader",
            Self::HighZoomReflow => "high_zoom_reflow",
            Self::ReducedMotion => "reduced_motion",
            Self::CliExport => "cli_export",
            Self::DegradedState => "degraded_state",
            Self::VisualFoundationComponentTruth => "visual_foundation_component_truth",
        }
    }
}

/// The certification state of one truth axis on one profile.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum VisualFoundationAxisCertificationState {
    /// Green: parity is current; the axis fully certifies.
    Certified,
    /// Yellow: parity is not current, but the reduction is disclosed and binds to a visible claim
    /// narrowing.
    DisclosedNarrowed,
    /// Red: parity is not current and the profile hides it behind a trusted claim inherited from a
    /// healthier profile.
    UndisclosedDrift,
}

impl VisualFoundationAxisCertificationState {
    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Certified => "certified",
            Self::DisclosedNarrowed => "disclosed_narrowed",
            Self::UndisclosedDrift => "undisclosed_drift",
        }
    }
}

/// The derived certification verdict for a whole profile. Never asserted by the author — always
/// recomputed from the axis outcomes, guardrails, and claim narrowing.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum VisualFoundationProfileClaimStatus {
    /// Full standing: every axis certified, every invariant held, claimed visual tier delivered.
    Green,
    /// Disclosed narrowing: an axis is not current and the claim narrows visibly.
    Yellow,
    /// Blocked: a degraded axis hides behind a full claim, a hard invariant breaks, CLI/export parity
    /// drops, a non-live profile claims a trusted visual surface, or the narrowing is inconsistent.
    Red,
}

impl VisualFoundationProfileClaimStatus {
    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Green => "green",
            Self::Yellow => "yellow",
            Self::Red => "red",
        }
    }

    /// True when the profile is publishable as certified (green or disclosed yellow); red profiles
    /// block the release.
    pub const fn is_publishable(self) -> bool {
        !matches!(self, Self::Red)
    }
}

/// The five B136 hard invariants carried on every certified profile. All five must hold — a breach
/// blocks the profile (red). Each field is `true` only when the profile *breaks* the invariant, so a
/// clean profile carries all-false.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct VisualFoundationCertGuardrails {
    /// True if the profile collapses status or trust meaning into a color-only cue. Must be false.
    pub collapses_status_or_trust_into_color_only: bool,
    /// True if the profile lets a syntax or diff palette collide with diagnostics. Must be false.
    pub lets_syntax_or_diff_palette_collide_with_diagnostics: bool,
    /// True if the profile shrinks a hit target below its supported minimum. Must be false.
    pub shrinks_hit_target_below_supported_minimum: bool,
    /// True if the profile lets chart meaning depend on color alone. Must be false.
    pub lets_chart_meaning_depend_on_color_alone: bool,
    /// True if the profile forks local spacing or elevation from the shared geometry. Must be false.
    pub forks_local_spacing_or_elevation_from_shared_geometry: bool,
}

impl VisualFoundationCertGuardrails {
    /// A clean profile: every invariant held.
    pub const CLEAN: Self = Self {
        collapses_status_or_trust_into_color_only: false,
        lets_syntax_or_diff_palette_collide_with_diagnostics: false,
        shrinks_hit_target_below_supported_minimum: false,
        lets_chart_meaning_depend_on_color_alone: false,
        forks_local_spacing_or_elevation_from_shared_geometry: false,
    };

    /// True when every invariant holds (no field is set).
    pub const fn all_held(&self) -> bool {
        !self.collapses_status_or_trust_into_color_only
            && !self.lets_syntax_or_diff_palette_collide_with_diagnostics
            && !self.shrinks_hit_target_below_supported_minimum
            && !self.lets_chart_meaning_depend_on_color_alone
            && !self.forks_local_spacing_or_elevation_from_shared_geometry
    }
}

/// The copy / export parity a certified profile preserves. The CLI/export axis certifies only when
/// this offers text / JSON / Markdown reconstruction and prohibits a raw-payload-only export.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct VisualFoundationCertExportParity {
    /// The copy formats the profile offers (must include text / json / markdown).
    #[serde(default)]
    pub formats: Vec<String>,
    /// The canonical-role / semantic-meaning / token-reference / theme-variant / contrast-pairing /
    /// geometry-baseline fields the profile preserves in export.
    #[serde(default)]
    pub export_fields: Vec<String>,
    /// True when a raw-payload-only export is prohibited.
    pub raw_payload_only_prohibited: bool,
}

impl VisualFoundationCertExportParity {
    /// Whether the parity offers text / JSON / Markdown copy and prohibits a raw-payload-only export.
    pub fn is_complete(&self) -> bool {
        let has = |f: &str| self.formats.iter().any(|v| v == f);
        has("text")
            && has("json")
            && has("markdown")
            && !self.export_fields.is_empty()
            && self.raw_payload_only_prohibited
    }
}

/// One axis outcome on one certified profile.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct VisualFoundationAxisOutcome {
    /// The truth axis this outcome scores.
    pub axis: VisualFoundationCertificationAxis,
    /// The certification state of the axis.
    pub state: VisualFoundationAxisCertificationState,
    /// The parity note recorded for this axis (always present).
    pub parity_note: String,
    /// The narrowing reason; present iff the axis is not certified.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub narrowing_reason: Option<String>,
    /// The frozen downgrade trigger; present iff the axis is disclosed-narrowed.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub downgrade_trigger: Option<M5VisualFoundationDowngradeTrigger>,
}

impl VisualFoundationAxisOutcome {
    /// Whether the outcome's optional fields are consistent with its state.
    ///
    /// - `Certified` carries neither a narrowing reason nor a trigger.
    /// - `DisclosedNarrowed` carries a non-generic reason *and* a frozen trigger.
    /// - `UndisclosedDrift` carries a reason describing the hidden drift but no visible trigger (that
    ///   is exactly what makes it undisclosed).
    pub fn well_formed(&self) -> bool {
        if self.parity_note.trim().is_empty() {
            return false;
        }
        match self.state {
            VisualFoundationAxisCertificationState::Certified => {
                self.narrowing_reason.is_none() && self.downgrade_trigger.is_none()
            }
            VisualFoundationAxisCertificationState::DisclosedNarrowed => {
                let reason_ok = self
                    .narrowing_reason
                    .as_deref()
                    .is_some_and(|r| !r.trim().is_empty() && !label_is_generic(r));
                reason_ok && self.downgrade_trigger.is_some()
            }
            VisualFoundationAxisCertificationState::UndisclosedDrift => {
                self.narrowing_reason
                    .as_deref()
                    .is_some_and(|r| !r.trim().is_empty())
                    && self.downgrade_trigger.is_none()
            }
        }
    }
}

/// The visible claim narrowing a profile applies when a truth axis is not current. Present iff the
/// certified claim is strictly weaker than the claimed one.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct VisualFoundationClaimAutoNarrow {
    /// The axis whose degraded parity forced the narrowing.
    pub binding_axis: VisualFoundationCertificationAxis,
    /// The claim the profile would deliver at full parity.
    pub from_claim: M5VisualFoundationA11yClaim,
    /// The weakest supported claim the profile is certified down to.
    pub to_claim: M5VisualFoundationA11yClaim,
    /// The visible, non-generic disclosure label.
    pub visible_label: String,
}

/// One certified M5 shell / editor / review / data / docs visual profile.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct VisualFoundationProfileCertificationRow {
    /// Record kind; must equal [`VISUAL_FOUNDATION_CERT_ROW_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`VISUAL_FOUNDATION_CERT_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable row id.
    pub row_id: String,
    /// The certified profile.
    pub profile: M5VisualFoundationCertifiedProfile,
    /// The visual claim ceiling the profile asserts.
    pub claimed_claim: M5VisualFoundationA11yClaim,
    /// The weakest supported claim the profile is certified down to. Must be no stronger than
    /// `claimed_claim`.
    pub certified_claim: M5VisualFoundationA11yClaim,
    /// The frozen foundation families this profile renders (at least one).
    #[serde(default)]
    pub consumed_families: Vec<M5VisualFoundationFamily>,
    /// One outcome per [`VisualFoundationCertificationAxis`], each axis appearing once.
    #[serde(default)]
    pub axis_outcomes: Vec<VisualFoundationAxisOutcome>,
    /// The B136 hard invariants; all must hold.
    pub guardrails: VisualFoundationCertGuardrails,
    /// The visible claim narrowing; present iff `certified_claim` is weaker than `claimed_claim`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub claim_auto_narrow: Option<VisualFoundationClaimAutoNarrow>,
    /// The one canonical visual-foundation proof bundle this profile cites. Must equal
    /// [`VISUAL_FOUNDATION_CERT_CANONICAL_BUNDLE_REF`].
    pub canonical_bundle_ref: String,
    /// The derived verdict. Recomputed and compared on validation.
    pub derived_status: VisualFoundationProfileClaimStatus,
    /// The copy / export parity of the certified profile state.
    pub export_parity: VisualFoundationCertExportParity,
    /// The compatibility notes captured for this profile.
    #[serde(default)]
    pub compatibility_notes: Vec<String>,
    /// Source contract refs backing this row.
    #[serde(default)]
    pub source_refs: Vec<String>,
    /// ISO 8601 UTC timestamp the certification was observed.
    pub observed_at: String,
    /// Evidence packet refs backing this row.
    #[serde(default)]
    pub evidence_refs: Vec<String>,
}

impl VisualFoundationProfileCertificationRow {
    /// The outcome for a given axis, if present.
    pub fn axis(
        &self,
        axis: VisualFoundationCertificationAxis,
    ) -> Option<&VisualFoundationAxisOutcome> {
        self.axis_outcomes.iter().find(|o| o.axis == axis)
    }

    /// Whether every axis appears exactly once.
    pub fn covers_all_axes(&self) -> bool {
        let seen: BTreeSet<VisualFoundationCertificationAxis> =
            self.axis_outcomes.iter().map(|o| o.axis).collect();
        seen.len() == self.axis_outcomes.len()
            && VisualFoundationCertificationAxis::ALL
                .iter()
                .all(|a| seen.contains(a))
    }

    /// Whether every axis outcome is internally well-formed.
    pub fn axis_outcomes_well_formed(&self) -> bool {
        self.axis_outcomes
            .iter()
            .all(VisualFoundationAxisOutcome::well_formed)
    }

    /// True when the profile narrows its visual claim below what it asserts.
    pub fn is_claim_narrowed(&self) -> bool {
        self.certified_claim.capability_rank() < self.claimed_claim.capability_rank()
    }

    /// The axes disclosed as narrowed (yellow).
    pub fn narrowed_axes(&self) -> Vec<VisualFoundationCertificationAxis> {
        self.axis_outcomes
            .iter()
            .filter(|o| o.state == VisualFoundationAxisCertificationState::DisclosedNarrowed)
            .map(|o| o.axis)
            .collect()
    }

    /// Derives the profile verdict from its axes, invariants, and claim narrowing. This is the heart
    /// of the capstone: a degraded axis must produce a visible claim narrowing, only a live
    /// first-party profile may certify a trusted visual surface, every hard invariant must hold,
    /// CLI/export parity must always certify, and the narrowing must be consistent.
    pub fn derive_status(&self) -> VisualFoundationProfileClaimStatus {
        // Structural prerequisites: malformed rows can never certify.
        if !self.covers_all_axes()
            || !self.axis_outcomes_well_formed()
            || self.canonical_bundle_ref != VISUAL_FOUNDATION_CERT_CANONICAL_BUNDLE_REF
            || self.consumed_families.is_empty()
            || !self.export_parity.is_complete()
        {
            return VisualFoundationProfileClaimStatus::Red;
        }

        // Every B136 hard invariant must hold.
        if !self.guardrails.all_held() {
            return VisualFoundationProfileClaimStatus::Red;
        }

        // Certification may only narrow the claim, never strengthen it.
        if self.certified_claim.capability_rank() > self.claimed_claim.capability_rank() {
            return VisualFoundationProfileClaimStatus::Red;
        }

        // Only a live first-party profile may certify a trusted visual surface.
        if self.certified_claim.asserts_trusted_surface()
            && !self.profile.is_live_trusted_visual_surface()
        {
            return VisualFoundationProfileClaimStatus::Red;
        }

        // The always-on CLI/export axis must stay certified.
        match self.axis(VisualFoundationCertificationAxis::CliExport) {
            Some(o) if o.state == VisualFoundationAxisCertificationState::Certified => {}
            _ => return VisualFoundationProfileClaimStatus::Red,
        }

        // Any undisclosed drift blocks outright.
        if self
            .axis_outcomes
            .iter()
            .any(|o| o.state == VisualFoundationAxisCertificationState::UndisclosedDrift)
        {
            return VisualFoundationProfileClaimStatus::Red;
        }

        let narrowed = self.narrowed_axes();
        let claim_narrowed = self.is_claim_narrowed();

        match (&self.claim_auto_narrow, claim_narrowed) {
            // Spurious narrowing structure without a claim reduction.
            (Some(_), false) => return VisualFoundationProfileClaimStatus::Red,
            // A claim reduction with no disclosed narrowing structure.
            (None, true) => return VisualFoundationProfileClaimStatus::Red,
            (Some(narrow), true) => {
                if narrow.from_claim != self.claimed_claim
                    || narrow.to_claim != self.certified_claim
                    || !narrowed.contains(&narrow.binding_axis)
                    || narrow.binding_axis.is_always_on()
                    || narrow.visible_label.trim().is_empty()
                    || label_is_generic(&narrow.visible_label)
                {
                    return VisualFoundationProfileClaimStatus::Red;
                }
            }
            (None, false) => {}
        }

        if claim_narrowed {
            // A disclosed, consistently-bound narrowing.
            return VisualFoundationProfileClaimStatus::Yellow;
        }

        // Claim not narrowed: a degraded axis retained behind a full claim is a hidden overclaim
        // inheriting a healthier profile's truth.
        if !narrowed.is_empty() {
            return VisualFoundationProfileClaimStatus::Red;
        }

        VisualFoundationProfileClaimStatus::Green
    }

    /// Whether the stored `derived_status` matches a fresh recomputation.
    pub fn status_is_fresh(&self) -> bool {
        self.derived_status == self.derive_status()
    }

    /// Whether the row's identity and evidence fields are complete.
    pub fn is_complete(&self) -> bool {
        self.record_kind == VISUAL_FOUNDATION_CERT_ROW_RECORD_KIND
            && self.schema_version == VISUAL_FOUNDATION_CERT_SCHEMA_VERSION
            && !self.row_id.trim().is_empty()
            && !self.canonical_bundle_ref.trim().is_empty()
            && !self.consumed_families.is_empty()
            && !self.observed_at.trim().is_empty()
            && !self.evidence_refs.is_empty()
            && self.evidence_refs.iter().all(|r| !r.trim().is_empty())
            && !self.compatibility_notes.is_empty()
    }

    /// Deterministic governed chip line for this row.
    pub fn chip_tokens(&self) -> String {
        format!(
            "profile={profile} claimed={claimed} certified={certified} status={status} \
narrowed_axes={narrowed}",
            profile = self.profile.as_str(),
            claimed = self.claimed_claim.as_str(),
            certified = self.certified_claim.as_str(),
            status = self.derived_status.as_str(),
            narrowed = self.narrowed_axes().len(),
        )
    }
}

/// Rolled-up summary of an M05-1147 certification packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct VisualFoundationProfileCertificationSummary {
    pub row_count: usize,
    pub profile_count: usize,
    pub green_row_count: usize,
    pub yellow_row_count: usize,
    pub red_row_count: usize,
    pub all_profiles_present: bool,
    pub all_families_covered: bool,
    pub all_rows_publishable: bool,
    pub all_status_fresh: bool,
    pub all_rows_cite_canonical_bundle: bool,
    pub all_rows_export_parity_certified: bool,
    pub all_guardrails_held: bool,
    pub every_axis_covered_on_every_row: bool,
    pub narrowed_profile_count: usize,
    pub report_clean: bool,
}

/// Constructor input for [`VisualFoundationProfileCertificationPacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VisualFoundationProfileCertificationPacketInput {
    pub packet_id: String,
    pub as_of: String,
    pub matrix_ref: String,
    pub canonical_bundle_ref: String,
    pub rows: Vec<VisualFoundationProfileCertificationRow>,
}

/// Checked-in M05-1147 certification packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct VisualFoundationProfileCertificationPacket {
    pub schema_version: u32,
    pub record_kind: String,
    pub packet_id: String,
    pub as_of: String,
    pub matrix_ref: String,
    pub canonical_bundle_ref: String,
    #[serde(default)]
    pub rows: Vec<VisualFoundationProfileCertificationRow>,
    pub summary: VisualFoundationProfileCertificationSummary,
}

impl VisualFoundationProfileCertificationPacket {
    /// Builds a packet, stamping the record kind, schema version, and computed summary.
    pub fn new(input: VisualFoundationProfileCertificationPacketInput) -> Self {
        let mut packet = Self {
            schema_version: VISUAL_FOUNDATION_CERT_SCHEMA_VERSION,
            record_kind: VISUAL_FOUNDATION_CERT_RECORD_KIND.to_owned(),
            packet_id: input.packet_id,
            as_of: input.as_of,
            matrix_ref: input.matrix_ref,
            canonical_bundle_ref: input.canonical_bundle_ref,
            rows: input.rows,
            summary: VisualFoundationProfileCertificationSummary {
                row_count: 0,
                profile_count: 0,
                green_row_count: 0,
                yellow_row_count: 0,
                red_row_count: 0,
                all_profiles_present: false,
                all_families_covered: false,
                all_rows_publishable: false,
                all_status_fresh: false,
                all_rows_cite_canonical_bundle: false,
                all_rows_export_parity_certified: false,
                all_guardrails_held: false,
                every_axis_covered_on_every_row: false,
                narrowed_profile_count: 0,
                report_clean: false,
            },
        };
        packet.summary = packet.computed_summary();
        packet
    }

    /// Profiles represented by some row in this packet.
    pub fn represented_profiles(&self) -> BTreeSet<M5VisualFoundationCertifiedProfile> {
        self.rows.iter().map(|r| r.profile).collect()
    }

    /// Foundation families rendered by some certified profile in this packet.
    pub fn represented_families(&self) -> BTreeSet<M5VisualFoundationFamily> {
        self.rows
            .iter()
            .flat_map(|r| r.consumed_families.iter().copied())
            .collect()
    }

    /// Whether every certified profile appears exactly once.
    pub fn all_profiles_present(&self) -> bool {
        let profiles = self.represented_profiles();
        profiles.len() == self.rows.len()
            && M5VisualFoundationCertifiedProfile::ALL
                .iter()
                .all(|s| profiles.contains(s))
    }

    /// Whether every frozen foundation family is certified on at least one profile — proof the full
    /// matrix runs across the claimed consumers.
    pub fn all_families_covered(&self) -> bool {
        let families = self.represented_families();
        M5VisualFoundationFamily::ALL
            .iter()
            .all(|f| families.contains(f))
    }

    /// Whether a CLI/export axis is certified on every row.
    pub fn all_rows_export_parity_certified(&self) -> bool {
        self.rows.iter().all(|r| {
            r.axis(VisualFoundationCertificationAxis::CliExport)
                .is_some_and(|o| o.state == VisualFoundationAxisCertificationState::Certified)
                && r.export_parity.is_complete()
        })
    }

    /// Computes summary fields from the packet contents.
    pub fn computed_summary(&self) -> VisualFoundationProfileCertificationSummary {
        let profiles = self.represented_profiles();
        let green = self
            .rows
            .iter()
            .filter(|r| r.derived_status == VisualFoundationProfileClaimStatus::Green)
            .count();
        let yellow = self
            .rows
            .iter()
            .filter(|r| r.derived_status == VisualFoundationProfileClaimStatus::Yellow)
            .count();
        let red = self
            .rows
            .iter()
            .filter(|r| r.derived_status == VisualFoundationProfileClaimStatus::Red)
            .count();
        let all_publishable = self.rows.iter().all(|r| r.derived_status.is_publishable());
        let all_fresh = self
            .rows
            .iter()
            .all(VisualFoundationProfileCertificationRow::status_is_fresh);
        let all_profiles = self.all_profiles_present();
        let all_families = self.all_families_covered();

        VisualFoundationProfileCertificationSummary {
            row_count: self.rows.len(),
            profile_count: profiles.len(),
            green_row_count: green,
            yellow_row_count: yellow,
            red_row_count: red,
            all_profiles_present: all_profiles,
            all_families_covered: all_families,
            all_rows_publishable: all_publishable,
            all_status_fresh: all_fresh,
            all_rows_cite_canonical_bundle: self
                .rows
                .iter()
                .all(|r| r.canonical_bundle_ref == VISUAL_FOUNDATION_CERT_CANONICAL_BUNDLE_REF),
            all_rows_export_parity_certified: self.all_rows_export_parity_certified(),
            all_guardrails_held: self.rows.iter().all(|r| r.guardrails.all_held()),
            every_axis_covered_on_every_row: self
                .rows
                .iter()
                .all(VisualFoundationProfileCertificationRow::covers_all_axes),
            narrowed_profile_count: self.rows.iter().filter(|r| r.is_claim_narrowed()).count(),
            report_clean: all_publishable && all_fresh && all_profiles && all_families,
        }
    }

    /// Validates the packet and returns every contract violation.
    pub fn validate(&self) -> Vec<VisualFoundationCertificationViolation> {
        let mut violations = Vec::new();

        if self.schema_version != VISUAL_FOUNDATION_CERT_SCHEMA_VERSION {
            violations.push(VisualFoundationCertificationViolation::SchemaVersion {
                expected: VISUAL_FOUNDATION_CERT_SCHEMA_VERSION,
                actual: self.schema_version,
            });
        }
        if self.record_kind != VISUAL_FOUNDATION_CERT_RECORD_KIND {
            violations.push(VisualFoundationCertificationViolation::RecordKind {
                expected: VISUAL_FOUNDATION_CERT_RECORD_KIND.to_owned(),
                actual: self.record_kind.clone(),
            });
        }
        if self.packet_id.trim().is_empty()
            || self.as_of.trim().is_empty()
            || self.matrix_ref.trim().is_empty()
        {
            violations.push(VisualFoundationCertificationViolation::MissingIdentity);
        }
        if self.canonical_bundle_ref != VISUAL_FOUNDATION_CERT_CANONICAL_BUNDLE_REF {
            violations.push(VisualFoundationCertificationViolation::WrongCanonicalBundle);
        }

        let mut row_ids = BTreeSet::new();
        for row in &self.rows {
            if !row_ids.insert(row.row_id.clone()) {
                violations.push(VisualFoundationCertificationViolation::DuplicateId {
                    id: row.row_id.clone(),
                });
            }

            if !row.is_complete() {
                violations.push(VisualFoundationCertificationViolation::IncompleteRow {
                    id: row.row_id.clone(),
                });
            }

            if !row.covers_all_axes() {
                violations.push(
                    VisualFoundationCertificationViolation::AxisCoverageIncomplete {
                        id: row.row_id.clone(),
                    },
                );
            }

            if !row.axis_outcomes_well_formed() {
                violations.push(
                    VisualFoundationCertificationViolation::MalformedAxisOutcome {
                        id: row.row_id.clone(),
                    },
                );
            }

            if row.canonical_bundle_ref != VISUAL_FOUNDATION_CERT_CANONICAL_BUNDLE_REF {
                violations.push(
                    VisualFoundationCertificationViolation::RowMissingCanonicalBundle {
                        id: row.row_id.clone(),
                    },
                );
            }

            // Every B136 hard invariant must hold.
            if !row.guardrails.all_held() {
                violations.push(VisualFoundationCertificationViolation::GuardrailViolated {
                    id: row.row_id.clone(),
                });
            }

            // Only a live first-party profile may certify a trusted visual surface.
            if row.certified_claim.asserts_trusted_surface()
                && !row.profile.is_live_trusted_visual_surface()
            {
                violations.push(
                    VisualFoundationCertificationViolation::NonLiveProfileClaimsTrustedSurface {
                        id: row.row_id.clone(),
                    },
                );
            }

            // CLI/export parity is always-on.
            if !row.export_parity.is_complete()
                || row
                    .axis(VisualFoundationCertificationAxis::CliExport)
                    .is_none_or_state_not_certified()
            {
                violations.push(
                    VisualFoundationCertificationViolation::ExportParityNotCertified {
                        id: row.row_id.clone(),
                    },
                );
            }

            // Certification may never strengthen a claim.
            if row.certified_claim.capability_rank() > row.claimed_claim.capability_rank() {
                violations.push(
                    VisualFoundationCertificationViolation::CertifiedClaimExceedsClaim {
                        id: row.row_id.clone(),
                    },
                );
            }

            // The stored verdict must match a fresh recomputation.
            if !row.status_is_fresh() {
                violations.push(
                    VisualFoundationCertificationViolation::StatusDerivationStale {
                        id: row.row_id.clone(),
                    },
                );
            }

            // A blocked (red) profile must not ship in a clean packet.
            if row.derived_status == VisualFoundationProfileClaimStatus::Red {
                violations.push(VisualFoundationCertificationViolation::ProfileBlocked {
                    id: row.row_id.clone(),
                });
            }
        }

        // Every claimed profile must be certified exactly once.
        if !self.all_profiles_present() {
            violations.push(VisualFoundationCertificationViolation::ProfileCoverageIncomplete);
        }

        // Every frozen foundation family must be certified on some profile.
        if !self.all_families_covered() {
            violations.push(VisualFoundationCertificationViolation::FamilyCoverageIncomplete);
        }

        if self.summary != self.computed_summary() {
            violations.push(VisualFoundationCertificationViolation::SummaryMismatch);
        }

        if json_contains_forbidden_material(
            &serde_json::to_value(self).expect("certification packet serializes"),
        ) {
            violations.push(VisualFoundationCertificationViolation::RawFoundationMaterialInExport);
        }

        violations
    }

    /// Deterministic export-safe JSON.
    ///
    /// # Panics
    ///
    /// Panics only if serializing this metadata-only packet fails.
    pub fn export_safe_json(&self) -> String {
        serde_json::to_string_pretty(self).expect("certification packet serializes")
    }

    /// Deterministic CSV of the certification rows for release / support handoff.
    pub fn render_matrix_csv(&self) -> String {
        let mut out = String::from(
            "row_id,profile,claimed_claim,certified_claim,status,narrowed_axes,binding_axis\n",
        );
        for row in &self.rows {
            let binding = row
                .claim_auto_narrow
                .as_ref()
                .map(|n| n.binding_axis.as_str())
                .unwrap_or("none");
            out.push_str(&format!(
                "{id},{profile},{claimed},{certified},{status},{narrowed},{binding}\n",
                id = row.row_id,
                profile = row.profile.as_str(),
                claimed = row.claimed_claim.as_str(),
                certified = row.certified_claim.as_str(),
                status = row.derived_status.as_str(),
                narrowed = row.narrowed_axes().len(),
                binding = binding,
            ));
        }
        out
    }

    /// Deterministic Markdown report for support, docs, or release handoff.
    pub fn render_markdown_summary(&self) -> String {
        let mut out = String::new();
        out.push_str("# M5 Visual-Foundations Surface Certification\n\n");
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- As of: `{}`\n", self.as_of));
        out.push_str(&format!(
            "- Canonical bundle: `{}`\n",
            self.canonical_bundle_ref
        ));
        out.push_str(&format!(
            "- Profiles: {} / {} certified ({} green, {} yellow, {} red)\n",
            self.summary.profile_count,
            M5VisualFoundationCertifiedProfile::ALL.len(),
            self.summary.green_row_count,
            self.summary.yellow_row_count,
            self.summary.red_row_count,
        ));
        out.push_str(&format!(
            "- Families covered: {}\n",
            self.summary.all_families_covered
        ));
        out.push_str(&format!(
            "- Invariants held: {}\n",
            self.summary.all_guardrails_held
        ));
        out.push_str(&format!(
            "- Auto-narrowed profiles: {}\n",
            self.summary.narrowed_profile_count,
        ));
        out.push_str(&format!("- Report clean: {}\n", self.summary.report_clean));
        out.push_str("\n## Profiles\n\n");
        for row in &self.rows {
            out.push_str(&format!("- **{}** — {}\n", row.row_id, row.chip_tokens()));
        }
        out
    }
}

/// Reads and validates the checked-in certification export.
pub fn current_m5_visual_foundations_surface_certification_export(
) -> Result<VisualFoundationProfileCertificationPacket, VisualFoundationCertificationArtifactError>
{
    let packet: VisualFoundationProfileCertificationPacket = serde_json::from_str(include_str!(
        concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../artifacts/release/m5-visual-foundations-surface-certification/support_export.json"
        )
    ))
    .map_err(VisualFoundationCertificationArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(VisualFoundationCertificationArtifactError::Validation(
            violations,
        ))
    }
}

/// Errors emitted when reading the checked-in certification export.
#[derive(Debug)]
pub enum VisualFoundationCertificationArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<VisualFoundationCertificationViolation>),
}

impl fmt::Display for VisualFoundationCertificationArtifactError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(f, "certification export parse failed: {error}")
            }
            Self::Validation(violations) => {
                write!(
                    f,
                    "certification export failed validation: {} violation(s)",
                    violations.len()
                )
            }
        }
    }
}

impl Error for VisualFoundationCertificationArtifactError {}

/// Validation failure for M05-1147 certification packets.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum VisualFoundationCertificationViolation {
    SchemaVersion { expected: u32, actual: u32 },
    RecordKind { expected: String, actual: String },
    MissingIdentity,
    WrongCanonicalBundle,
    DuplicateId { id: String },
    IncompleteRow { id: String },
    AxisCoverageIncomplete { id: String },
    MalformedAxisOutcome { id: String },
    RowMissingCanonicalBundle { id: String },
    GuardrailViolated { id: String },
    NonLiveProfileClaimsTrustedSurface { id: String },
    ExportParityNotCertified { id: String },
    CertifiedClaimExceedsClaim { id: String },
    StatusDerivationStale { id: String },
    ProfileBlocked { id: String },
    ProfileCoverageIncomplete,
    FamilyCoverageIncomplete,
    SummaryMismatch,
    RawFoundationMaterialInExport,
}

impl fmt::Display for VisualFoundationCertificationViolation {
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
            Self::WrongCanonicalBundle => {
                write!(
                    f,
                    "packet does not cite the canonical visual-foundation proof bundle"
                )
            }
            Self::DuplicateId { id } => write!(f, "duplicate row id: {id}"),
            Self::IncompleteRow { id } => write!(f, "incomplete certification row: {id}"),
            Self::AxisCoverageIncomplete { id } => {
                write!(
                    f,
                    "row {id} does not score every certification axis exactly once"
                )
            }
            Self::MalformedAxisOutcome { id } => {
                write!(
                    f,
                    "row {id} has an axis outcome whose disclosure fields disagree with its state"
                )
            }
            Self::RowMissingCanonicalBundle { id } => {
                write!(
                    f,
                    "row {id} does not cite the one canonical visual-foundation proof bundle"
                )
            }
            Self::GuardrailViolated { id } => {
                write!(
                    f,
                    "row {id} breaks a B136 hard invariant: status/trust color-only collapse, a \
syntax / diff palette colliding with diagnostics, a hit target shrunk below its supported minimum, \
chart meaning depending on color alone, or local spacing / elevation forked from the shared geometry"
                )
            }
            Self::NonLiveProfileClaimsTrustedSurface { id } => {
                write!(
                    f,
                    "row {id} certifies a trusted visual surface on a non-live first-party profile"
                )
            }
            Self::ExportParityNotCertified { id } => {
                write!(
                    f,
                    "row {id} drops always-on CLI/export parity (text / JSON / Markdown reconstruction)"
                )
            }
            Self::CertifiedClaimExceedsClaim { id } => {
                write!(
                    f,
                    "row {id} certifies a claim stronger than the claimed one"
                )
            }
            Self::StatusDerivationStale { id } => {
                write!(
                    f,
                    "row {id} stored status disagrees with a fresh derivation"
                )
            }
            Self::ProfileBlocked { id } => {
                write!(
                    f,
                    "row {id} is blocked (red): a degraded axis is hidden behind a fresh trusted \
claim, a hard invariant broke, CLI/export parity dropped, a non-live profile claimed a trusted \
visual surface, or the narrowing is inconsistent"
                )
            }
            Self::ProfileCoverageIncomplete => {
                write!(
                    f,
                    "not every claimed M5 visual-foundation profile is certified exactly once"
                )
            }
            Self::FamilyCoverageIncomplete => {
                write!(
                    f,
                    "not every frozen visual-foundation family is certified on some profile"
                )
            }
            Self::SummaryMismatch => write!(f, "computed summary does not match stored summary"),
            Self::RawFoundationMaterialInExport => {
                write!(
                    f,
                    "export contains a raw hex value, font blob, credential, or secret material"
                )
            }
        }
    }
}

impl Error for VisualFoundationCertificationViolation {}

/// Small extension so the export-parity check reads cleanly.
trait AxisOutcomeOptionExt {
    fn is_none_or_state_not_certified(&self) -> bool;
}

impl AxisOutcomeOptionExt for Option<&VisualFoundationAxisOutcome> {
    fn is_none_or_state_not_certified(&self) -> bool {
        match self {
            None => true,
            Some(o) => o.state != VisualFoundationAxisCertificationState::Certified,
        }
    }
}

/// Whether a label is a generic non-answer rather than a precise disclosure. Includes the
/// visual-foundation generics the spec forbids collapsing distinct color, token, typography, and
/// geometry truth into (whole-label matches so a full sentence naming a concrete role, token, or
/// geometry baseline is not flagged).
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
            | "something went wrong"
            | "degraded"
            | "narrowed"
            | "reduced"
            | "stale"
            | "unverified"
            | "offline"
            | "warning"
            | "blocked"
            | "pending"
            | "loading"
            | "partial"
            | "cached"
            | "trusted"
            | "reviewable"
            | "color"
            | "colour"
            | "theme"
            | "token"
            | "syntax"
            | "diff"
            | "chart"
            | "typography"
            | "geometry"
            | "spacing"
            | "hit target"
            | "contrast"
            | "semantic role"
            | "token reference"
            | "theme variant"
            | "density"
            | "more"
            | "…"
            | "..."
            | "overflow"
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

// --------------------------------------------------------------------------
// Seed builder — the one source of truth shared by the tests and the on-disk
// support export so both stay byte-aligned.
// --------------------------------------------------------------------------

/// Builds the canonical, checked-in M05-1147 certification packet. Certifies all eight claimed M5
/// shell / editor / review / data / docs visual profiles: two deliver their claim (green) and six
/// auto-narrow a not-current truth axis to a weaker visual ceiling (yellow). No profile hides drift
/// or breaks a hard invariant (red).
pub fn seeded_m5_visual_foundations_surface_certification_packet(
) -> VisualFoundationProfileCertificationPacket {
    VisualFoundationProfileCertificationPacket::new(
        VisualFoundationProfileCertificationPacketInput {
            packet_id: VISUAL_FOUNDATION_CERT_PACKET_ID.to_owned(),
            as_of: "2026-07-13T00:00:00Z".to_owned(),
            matrix_ref: VISUAL_FOUNDATION_CERT_MATRIX_REF.to_owned(),
            canonical_bundle_ref: VISUAL_FOUNDATION_CERT_CANONICAL_BUNDLE_REF.to_owned(),
            rows: seeded_rows(),
        },
    )
}

fn seed_evidence(id: &str) -> Vec<String> {
    vec![
        format!("evidence:visual-foundations-surface-certification:{id}"),
        VISUAL_FOUNDATION_CERT_A11Y_BUNDLE_REF.to_owned(),
    ]
}

fn seed_export_parity(fields: &[&str]) -> VisualFoundationCertExportParity {
    VisualFoundationCertExportParity {
        formats: vec!["text".to_owned(), "json".to_owned(), "markdown".to_owned()],
        export_fields: fields.iter().map(|f| (*f).to_owned()).collect(),
        raw_payload_only_prohibited: true,
    }
}

fn seed_certified_note(axis: VisualFoundationCertificationAxis) -> &'static str {
    match axis {
        VisualFoundationCertificationAxis::Visual => {
            "canonical role, semantic meaning, token reference, theme variant, and contrast pairing shown on-surface without color alone"
        }
        VisualFoundationCertificationAxis::Keyboard => {
            "the same foundation role, token reference, and bound controls are keyboard-reachable, never hover-only"
        }
        VisualFoundationCertificationAxis::ScreenReader => {
            "the same visual-foundation truth is announced non-visually, never color/motion/glyph-only"
        }
        VisualFoundationCertificationAxis::HighZoomReflow => {
            "the same truth reflows legibly at high zoom without clipping the role, token reference, type scale, or geometry baseline"
        }
        VisualFoundationCertificationAxis::ReducedMotion => {
            "the same truth stays legible and usable with reduced motion, never motion-only"
        }
        VisualFoundationCertificationAxis::CliExport => {
            "profile state exports as text / JSON / Markdown for support replay"
        }
        VisualFoundationCertificationAxis::DegradedState => {
            "stale contrast evidence, an incomplete theme pair, unconfirmed semantic separation, unconfirmed chart encoding, stale readability evidence, or a partially-disclosed geometry baseline honestly downgrades the TrustedVisualSurface/ReviewableVisualSurface claim rather than reading as a fresh authoritative visual surface"
        }
        VisualFoundationCertificationAxis::VisualFoundationComponentTruth => {
            "canonical role, semantic meaning, token reference, theme variant, contrast pairing, non-color cue, type scale, and geometry baseline stay explicit and never collapse status or trust meaning into color alone, let a syntax or diff palette collide with diagnostics, shrink a hit target below its supported minimum, let chart meaning depend on color alone, or fork local spacing or elevation from the shared geometry"
        }
    }
}

fn seed_certified(axis: VisualFoundationCertificationAxis) -> VisualFoundationAxisOutcome {
    VisualFoundationAxisOutcome {
        axis,
        state: VisualFoundationAxisCertificationState::Certified,
        parity_note: seed_certified_note(axis).to_owned(),
        narrowing_reason: None,
        downgrade_trigger: None,
    }
}

fn seed_narrowed(
    axis: VisualFoundationCertificationAxis,
    note: &str,
    reason: &str,
    trigger: M5VisualFoundationDowngradeTrigger,
) -> VisualFoundationAxisOutcome {
    VisualFoundationAxisOutcome {
        axis,
        state: VisualFoundationAxisCertificationState::DisclosedNarrowed,
        parity_note: note.to_owned(),
        narrowing_reason: Some(reason.to_owned()),
        downgrade_trigger: Some(trigger),
    }
}

fn seed_all_certified() -> Vec<VisualFoundationAxisOutcome> {
    VisualFoundationCertificationAxis::ALL
        .iter()
        .copied()
        .map(seed_certified)
        .collect()
}

fn seed_certified_except(
    axis: VisualFoundationCertificationAxis,
    outcome: VisualFoundationAxisOutcome,
) -> Vec<VisualFoundationAxisOutcome> {
    VisualFoundationCertificationAxis::ALL
        .iter()
        .copied()
        .map(|a| {
            if a == axis {
                outcome.clone()
            } else {
                seed_certified(a)
            }
        })
        .collect()
}

#[allow(clippy::too_many_arguments)]
fn seed_row(
    row_id: &str,
    profile: M5VisualFoundationCertifiedProfile,
    claimed_claim: M5VisualFoundationA11yClaim,
    certified_claim: M5VisualFoundationA11yClaim,
    consumed_families: &[M5VisualFoundationFamily],
    axis_outcomes: Vec<VisualFoundationAxisOutcome>,
    claim_auto_narrow: Option<VisualFoundationClaimAutoNarrow>,
    export_fields: &[&str],
    compatibility_notes: &[&str],
) -> VisualFoundationProfileCertificationRow {
    let mut row = VisualFoundationProfileCertificationRow {
        record_kind: VISUAL_FOUNDATION_CERT_ROW_RECORD_KIND.to_owned(),
        schema_version: VISUAL_FOUNDATION_CERT_SCHEMA_VERSION,
        row_id: row_id.to_owned(),
        profile,
        claimed_claim,
        certified_claim,
        consumed_families: consumed_families.to_vec(),
        axis_outcomes,
        guardrails: VisualFoundationCertGuardrails::CLEAN,
        claim_auto_narrow,
        canonical_bundle_ref: VISUAL_FOUNDATION_CERT_CANONICAL_BUNDLE_REF.to_owned(),
        derived_status: VisualFoundationProfileClaimStatus::Green,
        export_parity: seed_export_parity(export_fields),
        compatibility_notes: compatibility_notes
            .iter()
            .map(|n| (*n).to_owned())
            .collect(),
        source_refs: vec![
            VISUAL_FOUNDATION_CERT_MATRIX_REF.to_owned(),
            VISUAL_FOUNDATION_CERT_SCHEMA_REF.to_owned(),
        ],
        observed_at: "2026-07-13T00:00:00Z".to_owned(),
        evidence_refs: seed_evidence(row_id),
    };
    row.derived_status = row.derive_status();
    row
}

fn seed_narrow(
    binding_axis: VisualFoundationCertificationAxis,
    from_claim: M5VisualFoundationA11yClaim,
    to_claim: M5VisualFoundationA11yClaim,
    label: &str,
) -> VisualFoundationClaimAutoNarrow {
    VisualFoundationClaimAutoNarrow {
        binding_axis,
        from_claim,
        to_claim,
        visible_label: label.to_owned(),
    }
}

fn seeded_rows() -> Vec<VisualFoundationProfileCertificationRow> {
    use M5VisualFoundationA11yClaim::*;
    use M5VisualFoundationCertifiedProfile as P;
    use M5VisualFoundationDowngradeTrigger as Trig;
    use M5VisualFoundationFamily::*;
    use VisualFoundationCertificationAxis as Ax;

    vec![
        // --- Green: full parity, claim delivered ---------------------------
        seed_row(
            "cert:live-trusted-visual-surface",
            P::LiveTrustedVisualSurface,
            TrustedVisualSurface,
            TrustedVisualSurface,
            &[SyntaxToken],
            seed_all_certified(),
            None,
            &["profile", "claimed_claim", "certified_claim", "status", "semantic_role"],
            &[
                "local profile: the syntax palette names its canonical scope roles and keeps them distinct from diagnostics, contrast-proven across dark / light / high-contrast",
                "the trusted visual surface pairs every color role with a non-color cue rather than a color-only status or trust signal",
                "keyboard / screen-reader / high-zoom / reduced-motion reach preserved for the rendered token surface",
                "visual-foundation-component-truth: a live first-party visual surface is the only profile that certifies a trusted visual surface",
            ],
        ),
        seed_row(
            "cert:reviewable-geometry-structure",
            P::ReviewableGeometryStructure,
            ReviewableVisualSurface,
            ReviewableVisualSurface,
            &[SpacingSizingRadiiElevation],
            seed_all_certified(),
            None,
            &["profile", "claimed_claim", "certified_claim", "status", "density_context"],
            &[
                "managed profile: the spacing / sizing / radii / elevation geometry stays density-aware and machine-readable rather than a private per-surface layout fork",
                "the geometry surface keeps its elevation hierarchy and minimum-target baseline inspectable rather than a color-only or motion-only cue",
                "text / JSON / Markdown reconstruction certified so support can replay the reviewable geometry",
                "visual-foundation-component-truth: a reviewable geometry structure never certifies a live trusted, authoritative visual claim",
            ],
        ),
        // --- Yellow: an axis is not current; the claim narrows visibly ------
        seed_row(
            "cert:stale-contrast-color-surface",
            P::StaleContrastColorSurface,
            ReviewableVisualSurface,
            ContrastUnverifiedProjection,
            &[ColorSystem],
            seed_certified_except(
                Ax::DegradedState,
                seed_narrowed(
                    Ax::DegradedState,
                    "the color system's contrast evidence is stale so a fresh, contrast-proven color meaning cannot be certified",
                    "The color system's contrast evidence is stale, so the ReviewableVisualSurface claim narrows to a contrast-unverified projection and the palette preserves its last-known canonical role and non-color cue rather than presenting a fresh, color-only status as authoritative",
                    Trig::StatusOrTrustCollapsedToColorOnly,
                ),
            ),
            Some(seed_narrow(
                Ax::DegradedState,
                ReviewableVisualSurface,
                ContrastUnverifiedProjection,
                "Contrast unverified: the contrast evidence is stale so the last-known canonical role and non-color cue are preserved and the palette never reads as a fresh, color-only status",
            )),
            &["profile", "claimed_claim", "certified_claim", "status", "binding_axis"],
            &[
                "remote profile: the palette preserves its last-known canonical role and non-color cue and marks the contrast as unverified rather than presenting a stale, color-only status as authoritative",
                "the color surface keeps every status role paired with a non-color cue while the contrast evidence is disclosed as stale",
                "degraded-state: ReviewableVisualSurface narrows to a contrast-unverified projection (auto-narrowed)",
                "visual-foundation-component-truth: status or trust meaning never collapses into a color-only cue",
            ],
        ),
        seed_row(
            "cert:unpaired-theme-token-surface",
            P::UnpairedThemeTokenSurface,
            ReviewableVisualSurface,
            ThemePairUnverifiedProjection,
            &[SemanticThemeToken],
            seed_certified_except(
                Ax::DegradedState,
                seed_narrowed(
                    Ax::DegradedState,
                    "the semantic theme token's dark / light / high-contrast pair cannot be confirmed so a fully-paired, mode-stable token cannot be certified",
                    "The semantic theme token's dark / light / high-contrast pair cannot be confirmed, so the ReviewableVisualSurface claim narrows to a theme-pair-unverified projection and the token keeps its last-known role explicit rather than presenting a single-mode token as fully paired",
                    Trig::ThemePairIncomplete,
                ),
            ),
            Some(seed_narrow(
                Ax::DegradedState,
                ReviewableVisualSurface,
                ThemePairUnverifiedProjection,
                "Theme pair unverified: the dark / light / high-contrast pair cannot be confirmed so the last-known token role stays explicit and the token never reads as fully paired",
            )),
            &["profile", "claimed_claim", "certified_claim", "status", "binding_axis"],
            &[
                "mirrored profile: the token keeps its last-known semantic role and theme variant explicit and marks the pair as unverified rather than presenting a single-mode token as fully paired",
                "the theme-token surface keeps its role stable across the modes it can prove while the pair is disclosed as unverified",
                "degraded-state: ReviewableVisualSurface narrows to a theme-pair-unverified projection (auto-narrowed)",
                "visual-foundation-component-truth: a single-mode token is never shown as a fully paired, mode-stable role",
            ],
        ),
        seed_row(
            "cert:colliding-diff-surface",
            P::CollidingDiffSurface,
            ReviewableVisualSurface,
            SemanticSeparationUnverifiedProjection,
            &[DiffToken],
            seed_certified_except(
                Ax::VisualFoundationComponentTruth,
                seed_narrowed(
                    Ax::VisualFoundationComponentTruth,
                    "the diff token's diagnostics separation cannot be confirmed so a diagnostics-separated add / remove / context palette cannot be certified",
                    "The diff token's diagnostics separation cannot be confirmed, so the ReviewableVisualSurface claim narrows to a semantic-separation-unverified projection and the palette keeps its add / remove / context meaning inspectable rather than letting the diff palette collide with diagnostics",
                    Trig::SyntaxOrDiffPaletteCollidedWithDiagnostics,
                ),
            ),
            Some(seed_narrow(
                Ax::VisualFoundationComponentTruth,
                ReviewableVisualSurface,
                SemanticSeparationUnverifiedProjection,
                "Semantic separation unverified: the diagnostics separation cannot be confirmed so the add / remove / context meaning stays inspectable and the diff palette never collides with diagnostics",
            )),
            &["profile", "claimed_claim", "certified_claim", "status", "binding_axis"],
            &[
                "review profile: the palette keeps its add / remove / context meaning and non-color cue inspectable and marks the separation as unverified rather than colliding with diagnostics color",
                "the diff surface keeps its moved-block and historical-vs-current emphasis legible while the diagnostics separation is disclosed as unverified",
                "visual-foundation-component-truth: ReviewableVisualSurface narrows to a semantic-separation-unverified projection (auto-narrowed)",
                "visual-foundation-component-truth: a syntax or diff palette never collides with diagnostics",
            ],
        ),
        seed_row(
            "cert:color-only-chart-surface",
            P::ColorOnlyChartSurface,
            ReviewableVisualSurface,
            ChartEncodingUnverifiedProjection,
            &[ChartToken],
            seed_certified_except(
                Ax::VisualFoundationComponentTruth,
                seed_narrowed(
                    Ax::VisualFoundationComponentTruth,
                    "the chart token's non-color encoding is unconfirmed so a legend / pattern / marker-encoded chart cannot be certified",
                    "The chart token's non-color encoding is unconfirmed, so the ReviewableVisualSurface claim narrows to a chart-encoding-unverified projection and the chart keeps its legend / pattern / marker cue disclosed rather than letting chart meaning depend on color alone",
                    Trig::ChartMeaningDependedOnColorAlone,
                ),
            ),
            Some(seed_narrow(
                Ax::VisualFoundationComponentTruth,
                ReviewableVisualSurface,
                ChartEncodingUnverifiedProjection,
                "Chart encoding unverified: the non-color encoding is unconfirmed so the legend / pattern / marker cue stays disclosed and chart meaning never depends on color alone",
            )),
            &["profile", "claimed_claim", "certified_claim", "status", "binding_axis"],
            &[
                "data profile: the chart keeps its legend / pattern / marker cue disclosed and marks the encoding as unverified rather than letting series meaning depend on color alone",
                "the chart surface keeps its series and legend parity legible while the non-color encoding is disclosed as unverified",
                "visual-foundation-component-truth: ReviewableVisualSurface narrows to a chart-encoding-unverified projection (auto-narrowed)",
                "visual-foundation-component-truth: chart meaning never depends on color alone",
            ],
        ),
        seed_row(
            "cert:drifting-typography-surface",
            P::DriftingTypographySurface,
            ReviewableVisualSurface,
            TextReadabilityUnverifiedProjection,
            &[Typography],
            seed_certified_except(
                Ax::DegradedState,
                seed_narrowed(
                    Ax::DegradedState,
                    "the typography scale's readability evidence is stale so a fully-readable, scale-stable type surface cannot be certified",
                    "The typography scale's readability evidence is stale, so the ReviewableVisualSurface claim narrows to a text-readability-unverified projection and the surface keeps its type scale, line-height, and tabular numerals legible rather than presenting a drifted scale as fully readable",
                    Trig::TypographyScaleDrifted,
                ),
            ),
            Some(seed_narrow(
                Ax::DegradedState,
                ReviewableVisualSurface,
                TextReadabilityUnverifiedProjection,
                "Text readability unverified: the readability evidence is stale so the type scale, line-height, and tabular numerals stay legible and the surface never reads as a fully-readable, scale-stable type surface",
            )),
            &["profile", "claimed_claim", "certified_claim", "status", "binding_axis"],
            &[
                "docs profile: the surface keeps its type scale, line-height, and tabular numerals legible and marks the readability as unverified rather than presenting a drifted scale as fully readable",
                "the typography surface keeps its title / body / label / code hierarchy and code / UI font stacks stable while the readability evidence is disclosed as stale",
                "degraded-state: ReviewableVisualSurface narrows to a text-readability-unverified projection (auto-narrowed)",
                "visual-foundation-component-truth: a drifted type scale is never shown as fully readable and scale-stable",
            ],
        ),
        seed_row(
            "cert:undisclosed-hit-target-surface",
            P::UndisclosedHitTargetSurface,
            ReviewableVisualSurface,
            GeometryBaselineDisclosedProjection,
            &[HitTarget],
            seed_certified_except(
                Ax::VisualFoundationComponentTruth,
                seed_narrowed(
                    Ax::VisualFoundationComponentTruth,
                    "the hit-target baseline can only be partially disclosed so a fully-proven minimum-target geometry cannot be certified",
                    "The hit-target baseline can only be partially disclosed, so the ReviewableVisualSurface claim narrows to a geometry-baseline-disclosed projection and the surface discloses the partial minimum-target baseline inspectably rather than shrinking a hit target below its supported minimum under compact density",
                    Trig::ProofStale,
                ),
            ),
            Some(seed_narrow(
                Ax::VisualFoundationComponentTruth,
                ReviewableVisualSurface,
                GeometryBaselineDisclosedProjection,
                "Geometry baseline disclosed partial: the minimum-target baseline is only partially proven so it is disclosed inspectably and no hit target is shrunk below its supported minimum under compact density",
            )),
            &["profile", "claimed_claim", "certified_claim", "status", "binding_axis"],
            &[
                "accessibility-sensitive profile: the surface discloses its partial minimum-target baseline and keeps its interactive-control and resize-handle minima inspectable rather than shrinking a hit target below its supported minimum",
                "the hit-target surface keeps its density-aware minima legible while the geometry baseline is disclosed as partial",
                "visual-foundation-component-truth: ReviewableVisualSurface narrows to a geometry-baseline-disclosed projection (auto-narrowed)",
                "visual-foundation-component-truth: a hit target is never shrunk below its supported minimum under compact density",
            ],
        ),
    ]
}
