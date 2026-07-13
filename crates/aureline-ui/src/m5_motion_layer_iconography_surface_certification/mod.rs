//! M05-1155 surface certification over the frozen M5 motion-token / reduced-motion / opacity-scrim /
//! layer-order / portal-ownership / iconography / illustration-boundary visual-interaction matrix.
//!
//! Where the freeze matrix ([`crate::m5_motion_layer_iconography_matrix`]) defines the seven governed
//! visual-interaction families, the M05-1149..1152 implement lanes narrow each one, the M05-1153 shared
//! consumer lane aligns their grammar across surfaces, and the M05-1154 accessibility lane
//! ([`crate::m5_motion_layer_iconography_accessibility_parity_and_narrowing_when_motion_layer_or_icon_truth_is_stale`])
//! proves keyboard / screen-reader / high-zoom / reduced-motion / power-saver / thermal / CLI-export
//! parity and per-family auto-narrowing, this closing capstone *certifies* that the shared
//! visual-interaction truth holds on every claimed M5 desktop / dialog / onboarding / notification /
//! embedded operating profile — and auto-narrows any profile that cannot sustain it.
//!
//! It is keyed on the claimed **profile** a user, reviewer, or support engineer reads a motion, scrim,
//! layer, portal, icon, or illustration surface through (a live, first-party trusted interaction
//! surface; a reviewable layer structure; a stale-motion-timing surface; an unconfirmed reduced-motion
//! surface; an orientation-erasing scrim surface; a detached-portal surface; and an
//! impersonating-illustration surface), not on interaction family or implement lane. Each
//! [`VisualInteractionProfileCertificationRow`] certifies one profile across nine truth axes — visual,
//! keyboard, screen-reader, high-zoom-reflow, reduced-motion, power-thermal, CLI/export, degraded-state,
//! and visual-interaction-component-truth behavior — and either passes (green), auto-narrows its
//! interaction claim to the weakest supported ceiling (yellow), or is blocked (red) when a degraded axis
//! is hidden behind a fresh trusted claim inherited from a healthier profile.
//!
//! The invariant is: **a degraded axis must produce a visible claim narrowing**. A profile that keeps a
//! `TrustedInteractionSurface` / `ReviewableInteractionSurface` claim while one of its truth axes is not
//! current is over-claiming and blocks; a profile that discloses the reduction by narrowing its claim
//! (with a bound reason and a frozen downgrade trigger) is honestly yellow. Only a live, first-party
//! trusted interaction profile may certify a `TrustedInteractionSurface` claim — a reviewable,
//! stale-motion, unconfirmed, orientation-erasing, detached, or impersonating profile that keeps a
//! trusted claim is over-reaching and blocks. The always-on CLI/export axis must always stay certified
//! so support and automation can reconstruct the canonical interaction identity, semantic role, token
//! reference, motion profile, layer tier, and accessible fallback from the same interaction the user saw.
//!
//! The B137 hard invariants are enforced per row: no profile may delay protected input with motion, let
//! a scrim erase workspace orientation or contrast, let an overlay bypass the shared z-order, use an
//! unlabeled icon for an uncommon or destructive action, or let an illustration impersonate operational
//! or security truth. A profile that breaches any invariant blocks (red).
//!
//! Every row cites exactly one canonical visual-interaction proof bundle
//! ([`MOTION_LAYER_ICONOGRAPHY_CERT_CANONICAL_BUNDLE_REF`]) — the frozen motion-layer-iconography matrix
//! proof — rather than cloning per-profile evidence. The packet is metadata-only: raw duration curves,
//! z-index integers, glyph blobs, credentials, secrets, and endpoint refs never cross this boundary.
//!
//! The boundary schema is
//! [`schemas/design-system/m5-motion-layer-iconography-surface-certification.schema.json`](../../../../schemas/design-system/m5-motion-layer-iconography-surface-certification.schema.json).
//! The contract doc is
//! [`docs/design-system/m5_motion_layer_iconography_surface_certification.md`](../../../../docs/design-system/m5_motion_layer_iconography_surface_certification.md).

#[cfg(test)]
mod tests;

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

use crate::m5_motion_layer_iconography_accessibility_parity_and_narrowing_when_motion_layer_or_icon_truth_is_stale as a11y;
use crate::m5_motion_layer_iconography_matrix as matrix;
use a11y::M5VisualInteractionA11yClaim;
use matrix::{M5VisualInteractionDowngradeTrigger, M5VisualInteractionFamily};

/// Schema version stamped on the M05-1155 certification packet.
pub const MOTION_LAYER_ICONOGRAPHY_CERT_SCHEMA_VERSION: u32 = 1;

/// Stable record-kind tag carried by [`VisualInteractionProfileCertificationPacket`].
pub const MOTION_LAYER_ICONOGRAPHY_CERT_RECORD_KIND: &str =
    "m5_motion_layer_iconography_surface_certification_packet";

/// Stable record-kind tag carried by each [`VisualInteractionProfileCertificationRow`].
pub const MOTION_LAYER_ICONOGRAPHY_CERT_ROW_RECORD_KIND: &str =
    "m5_motion_layer_iconography_surface_certification_row";

/// Repo-relative path of the boundary schema.
pub const MOTION_LAYER_ICONOGRAPHY_CERT_SCHEMA_REF: &str =
    "schemas/design-system/m5-motion-layer-iconography-surface-certification.schema.json";

/// Repo-relative path of the contract doc.
pub const MOTION_LAYER_ICONOGRAPHY_CERT_DOC_REF: &str =
    "docs/design-system/m5_motion_layer_iconography_surface_certification.md";

/// Repo-relative path of the frozen motion-layer-iconography matrix schema the certified profiles render.
pub const MOTION_LAYER_ICONOGRAPHY_CERT_MATRIX_REF: &str =
    matrix::M5_MOTION_LAYER_ICONOGRAPHY_MATRIX_SCHEMA_REF;

/// The one canonical visual-interaction proof bundle every certified profile cites as its first-resolved
/// interaction truth. All seven profiles point back to it rather than cloning per-profile evidence.
pub const MOTION_LAYER_ICONOGRAPHY_CERT_CANONICAL_BUNDLE_REF: &str =
    matrix::M5_MOTION_LAYER_ICONOGRAPHY_ARTIFACT_REF;

/// The M05-1154 accessibility support export the certification builds on. Recorded as a supporting
/// evidence ref on every row.
pub const MOTION_LAYER_ICONOGRAPHY_CERT_A11Y_BUNDLE_REF: &str =
    a11y::MOTION_LAYER_ICONOGRAPHY_A11Y_ARTIFACT_REF;

/// Repo-relative path of the checked support-export artifact (the `include_str!` canonical).
pub const MOTION_LAYER_ICONOGRAPHY_CERT_ARTIFACT_REF: &str =
    "artifacts/release/m5-motion-layer-iconography-surface-certification/support_export.json";

/// Repo-relative path of the checked machine-readable matrix CSV.
pub const MOTION_LAYER_ICONOGRAPHY_CERT_CSV_REF: &str =
    "artifacts/release/m5-motion-layer-iconography-surface-certification/matrix.csv";

/// Repo-relative path of the checked Markdown report.
pub const MOTION_LAYER_ICONOGRAPHY_CERT_REPORT_REF: &str =
    "artifacts/release/m5-motion-layer-iconography-surface-certification.md";

/// Stable packet id for the checked-in certification bundle.
pub const MOTION_LAYER_ICONOGRAPHY_CERT_PACKET_ID: &str =
    "m5-motion-layer-iconography-surface-certification:stable:0001";

/// The seven claimed M5 desktop / dialog / onboarding / notification / embedded visual-interaction
/// operating profiles this capstone certifies. Keyed on the profile a user, reviewer, or support engineer
/// reads a motion, scrim, layer, portal, icon, or illustration surface through, not on the reusable
/// interaction family it renders. Only a live, first-party trusted interaction profile may certify a
/// trusted interaction surface.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5VisualInteractionCertifiedProfile {
    /// A live, first-party, fully-current interaction surface — a semantic, labeled iconography surface
    /// rendering the trusted, protected-path-safe, owning-surface-attached interaction exactly right now.
    LiveTrustedInteractionSurface,
    /// A reviewable layer structure: a self-sufficient, inspectable z-tier / portal-order surface a user
    /// can review, never itself an authoritative, live-rendering interaction surface.
    ReviewableLayerStructure,
    /// A motion-token surface whose protected-path timing evidence is stale; the claim narrows to a
    /// motion-timing-unverified projection with the last-known semantic role preserved, never a fresh,
    /// protected-path-delaying motion shown as authoritative.
    StaleMotionTimingSurface,
    /// A reduced-motion surface whose reduced-motion / power-saver / thermal clamp cannot be confirmed;
    /// the claim narrows to a reduced-motion-clamp-unverified projection that keeps the last-known static
    /// fallback explicit, never a motion-only cue shown as clamp-safe.
    UnconfirmedReducedMotionSurface,
    /// An opacity-scrim surface whose orientation / contrast preservation cannot be confirmed; the claim
    /// narrows to a scrim-orientation-unverified projection that keeps the workspace orientation cue
    /// inspectable, never a scrim shown as orientation-safe when it may erase context.
    OrientationErasingScrimSurface,
    /// A portal-ownership surface whose owning-surface attachment cannot be confirmed; the claim narrows
    /// to a portal-ownership-unverified projection that keeps the owning-surface reference and z-tier
    /// inspectable, never an overlay shown as attached when it may bypass the shared z-order.
    DetachedPortalSurface,
    /// An illustration-boundary surface that can only disclose a partial secondary-illustration boundary;
    /// the claim narrows to an illustration-boundary-disclosed projection disclosing the partial boundary.
    ImpersonatingIllustrationSurface,
}

impl M5VisualInteractionCertifiedProfile {
    /// Every certified profile, in declaration order.
    pub const ALL: [M5VisualInteractionCertifiedProfile; 7] = [
        M5VisualInteractionCertifiedProfile::LiveTrustedInteractionSurface,
        M5VisualInteractionCertifiedProfile::ReviewableLayerStructure,
        M5VisualInteractionCertifiedProfile::StaleMotionTimingSurface,
        M5VisualInteractionCertifiedProfile::UnconfirmedReducedMotionSurface,
        M5VisualInteractionCertifiedProfile::OrientationErasingScrimSurface,
        M5VisualInteractionCertifiedProfile::DetachedPortalSurface,
        M5VisualInteractionCertifiedProfile::ImpersonatingIllustrationSurface,
    ];

    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LiveTrustedInteractionSurface => "live_trusted_interaction_surface",
            Self::ReviewableLayerStructure => "reviewable_layer_structure",
            Self::StaleMotionTimingSurface => "stale_motion_timing_surface",
            Self::UnconfirmedReducedMotionSurface => "unconfirmed_reduced_motion_surface",
            Self::OrientationErasingScrimSurface => "orientation_erasing_scrim_surface",
            Self::DetachedPortalSurface => "detached_portal_surface",
            Self::ImpersonatingIllustrationSurface => "impersonating_illustration_surface",
        }
    }

    /// True only for the live, first-party trusted interaction surface profile. A trusted interaction
    /// surface may be certified on this profile alone; every other profile is at most a reviewable
    /// interaction structure or a narrowed projection.
    pub const fn is_live_trusted_interaction_surface(self) -> bool {
        matches!(self, Self::LiveTrustedInteractionSurface)
    }
}

/// The nine truth axes a certified profile is scored on. These are exactly the parity dimensions the spec
/// requires verifying — visual, keyboard, screen-reader, high-zoom reflow, reduced-motion, power-thermal,
/// CLI/export, degraded-state, and visual-interaction-component-truth behavior. The CLI/export axis is
/// always-on and must stay certified for every profile.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum VisualInteractionCertificationAxis {
    /// Visual parity: canonical role, semantic meaning, token reference, motion profile, layer tier, and
    /// accessible fallback are shown on the primary surface without relying on motion or decoration alone.
    Visual,
    /// Keyboard-reach parity: the same interaction truth and its bound controls are reachable and operable
    /// without a pointer, never hover-only.
    Keyboard,
    /// Screen-reader parity: the same truth is announced non-visually, never relying on motion, an
    /// unlabeled symbol, or a decoration alone.
    ScreenReader,
    /// High-zoom reflow parity: the same truth reflows legibly at high zoom rather than clipping the role,
    /// token reference, layer tier, or accessible fallback.
    HighZoomReflow,
    /// Reduced-motion parity: the same truth is legible and usable with reduced motion, never motion-only.
    ReducedMotion,
    /// Power-thermal parity: the same truth stays legible and usable under battery-saver and
    /// thermal-pressure clamps, never dependent on a motion the clamp suppresses.
    PowerThermal,
    /// CLI / export parity (always-on): the certified profile state is reconstructable as
    /// text / JSON / Markdown for support and automation.
    CliExport,
    /// Degraded-state parity: stale motion-timing evidence, an unconfirmed reduced-motion clamp,
    /// unconfirmed scrim orientation, an unconfirmed owning-surface attachment, or a partially-disclosed
    /// illustration boundary honestly downgrades a `TrustedInteractionSurface` / `ReviewableInteractionSurface`
    /// claim rather than reading as a fresh, authoritative interaction surface.
    DegradedState,
    /// Visual-interaction-component-truth parity: canonical role, semantic meaning, token reference,
    /// motion profile, layer tier, and accessible fallback stay explicit and never delay protected input
    /// with motion, let a scrim erase workspace orientation or contrast, let an overlay bypass the shared
    /// z-order, use an unlabeled icon for an uncommon or destructive action, or let an illustration
    /// impersonate operational or security truth.
    VisualInteractionComponentTruth,
}

impl VisualInteractionCertificationAxis {
    /// Every certification axis, in declaration order.
    pub const ALL: [VisualInteractionCertificationAxis; 9] = [
        VisualInteractionCertificationAxis::Visual,
        VisualInteractionCertificationAxis::Keyboard,
        VisualInteractionCertificationAxis::ScreenReader,
        VisualInteractionCertificationAxis::HighZoomReflow,
        VisualInteractionCertificationAxis::ReducedMotion,
        VisualInteractionCertificationAxis::PowerThermal,
        VisualInteractionCertificationAxis::CliExport,
        VisualInteractionCertificationAxis::DegradedState,
        VisualInteractionCertificationAxis::VisualInteractionComponentTruth,
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
            Self::PowerThermal => "power_thermal",
            Self::CliExport => "cli_export",
            Self::DegradedState => "degraded_state",
            Self::VisualInteractionComponentTruth => "visual_interaction_component_truth",
        }
    }
}

/// The certification state of one truth axis on one profile.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum VisualInteractionAxisCertificationState {
    /// Green: parity is current; the axis fully certifies.
    Certified,
    /// Yellow: parity is not current, but the reduction is disclosed and binds to a visible claim
    /// narrowing.
    DisclosedNarrowed,
    /// Red: parity is not current and the profile hides it behind a trusted claim inherited from a
    /// healthier profile.
    UndisclosedDrift,
}

impl VisualInteractionAxisCertificationState {
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
pub enum VisualInteractionProfileClaimStatus {
    /// Full standing: every axis certified, every invariant held, claimed interaction tier delivered.
    Green,
    /// Disclosed narrowing: an axis is not current and the claim narrows visibly.
    Yellow,
    /// Blocked: a degraded axis hides behind a full claim, a hard invariant breaks, CLI/export parity
    /// drops, a non-live profile claims a trusted interaction surface, or the narrowing is inconsistent.
    Red,
}

impl VisualInteractionProfileClaimStatus {
    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Green => "green",
            Self::Yellow => "yellow",
            Self::Red => "red",
        }
    }

    /// True when the profile is publishable as certified (green or disclosed yellow); red profiles block
    /// the release.
    pub const fn is_publishable(self) -> bool {
        !matches!(self, Self::Red)
    }
}

/// The five B137 hard invariants carried on every certified profile. All five must hold — a breach blocks
/// the profile (red). Each field is `true` only when the profile *breaks* the invariant, so a clean
/// profile carries all-false.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct VisualInteractionCertGuardrails {
    /// True if the profile delays protected input (menu, palette, typing-critical surface) with motion.
    /// Must be false.
    pub delays_protected_input_with_motion: bool,
    /// True if the profile lets a scrim erase workspace orientation or contrast. Must be false.
    pub lets_a_scrim_erase_workspace_orientation_or_contrast: bool,
    /// True if the profile lets an extension / private overlay bypass the shared z-order. Must be false.
    pub lets_an_overlay_bypass_the_shared_z_order: bool,
    /// True if the profile uses an unlabeled icon for an uncommon or destructive action. Must be false.
    pub uses_an_unlabeled_icon_for_an_uncommon_or_destructive_action: bool,
    /// True if the profile lets an illustration impersonate operational or security truth. Must be false.
    pub lets_an_illustration_impersonate_operational_or_security_truth: bool,
}

impl VisualInteractionCertGuardrails {
    /// A clean profile: every invariant held.
    pub const CLEAN: Self = Self {
        delays_protected_input_with_motion: false,
        lets_a_scrim_erase_workspace_orientation_or_contrast: false,
        lets_an_overlay_bypass_the_shared_z_order: false,
        uses_an_unlabeled_icon_for_an_uncommon_or_destructive_action: false,
        lets_an_illustration_impersonate_operational_or_security_truth: false,
    };

    /// True when every invariant holds (no field is set).
    pub const fn all_held(&self) -> bool {
        !self.delays_protected_input_with_motion
            && !self.lets_a_scrim_erase_workspace_orientation_or_contrast
            && !self.lets_an_overlay_bypass_the_shared_z_order
            && !self.uses_an_unlabeled_icon_for_an_uncommon_or_destructive_action
            && !self.lets_an_illustration_impersonate_operational_or_security_truth
    }
}

/// The copy / export parity a certified profile preserves. The CLI/export axis certifies only when this
/// offers text / JSON / Markdown reconstruction and prohibits a raw-payload-only export.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct VisualInteractionCertExportParity {
    /// The copy formats the profile offers (must include text / json / markdown).
    #[serde(default)]
    pub formats: Vec<String>,
    /// The canonical-role / semantic-meaning / token-reference / motion-profile / layer-tier /
    /// accessible-fallback fields the profile preserves in export.
    #[serde(default)]
    pub export_fields: Vec<String>,
    /// True when a raw-payload-only export is prohibited.
    pub raw_payload_only_prohibited: bool,
}

impl VisualInteractionCertExportParity {
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
pub struct VisualInteractionAxisOutcome {
    /// The truth axis this outcome scores.
    pub axis: VisualInteractionCertificationAxis,
    /// The certification state of the axis.
    pub state: VisualInteractionAxisCertificationState,
    /// The parity note recorded for this axis (always present).
    pub parity_note: String,
    /// The narrowing reason; present iff the axis is not certified.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub narrowing_reason: Option<String>,
    /// The frozen downgrade trigger; present iff the axis is disclosed-narrowed.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub downgrade_trigger: Option<M5VisualInteractionDowngradeTrigger>,
}

impl VisualInteractionAxisOutcome {
    /// Whether the outcome's optional fields are consistent with its state.
    ///
    /// - `Certified` carries neither a narrowing reason nor a trigger.
    /// - `DisclosedNarrowed` carries a non-generic reason *and* a frozen trigger.
    /// - `UndisclosedDrift` carries a reason describing the hidden drift but no visible trigger (that is
    ///   exactly what makes it undisclosed).
    pub fn well_formed(&self) -> bool {
        if self.parity_note.trim().is_empty() {
            return false;
        }
        match self.state {
            VisualInteractionAxisCertificationState::Certified => {
                self.narrowing_reason.is_none() && self.downgrade_trigger.is_none()
            }
            VisualInteractionAxisCertificationState::DisclosedNarrowed => {
                let reason_ok = self
                    .narrowing_reason
                    .as_deref()
                    .is_some_and(|r| !r.trim().is_empty() && !label_is_generic(r));
                reason_ok && self.downgrade_trigger.is_some()
            }
            VisualInteractionAxisCertificationState::UndisclosedDrift => {
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
pub struct VisualInteractionClaimAutoNarrow {
    /// The axis whose degraded parity forced the narrowing.
    pub binding_axis: VisualInteractionCertificationAxis,
    /// The claim the profile would deliver at full parity.
    pub from_claim: M5VisualInteractionA11yClaim,
    /// The weakest supported claim the profile is certified down to.
    pub to_claim: M5VisualInteractionA11yClaim,
    /// The visible, non-generic disclosure label.
    pub visible_label: String,
}

/// One certified M5 desktop / dialog / onboarding / notification / embedded interaction profile.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct VisualInteractionProfileCertificationRow {
    /// Record kind; must equal [`MOTION_LAYER_ICONOGRAPHY_CERT_ROW_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`MOTION_LAYER_ICONOGRAPHY_CERT_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable row id.
    pub row_id: String,
    /// The certified profile.
    pub profile: M5VisualInteractionCertifiedProfile,
    /// The interaction claim ceiling the profile asserts.
    pub claimed_claim: M5VisualInteractionA11yClaim,
    /// The weakest supported claim the profile is certified down to. Must be no stronger than
    /// `claimed_claim`.
    pub certified_claim: M5VisualInteractionA11yClaim,
    /// The frozen interaction families this profile renders (at least one).
    #[serde(default)]
    pub consumed_families: Vec<M5VisualInteractionFamily>,
    /// One outcome per [`VisualInteractionCertificationAxis`], each axis appearing once.
    #[serde(default)]
    pub axis_outcomes: Vec<VisualInteractionAxisOutcome>,
    /// The B137 hard invariants; all must hold.
    pub guardrails: VisualInteractionCertGuardrails,
    /// The visible claim narrowing; present iff `certified_claim` is weaker than `claimed_claim`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub claim_auto_narrow: Option<VisualInteractionClaimAutoNarrow>,
    /// The one canonical visual-interaction proof bundle this profile cites. Must equal
    /// [`MOTION_LAYER_ICONOGRAPHY_CERT_CANONICAL_BUNDLE_REF`].
    pub canonical_bundle_ref: String,
    /// The derived verdict. Recomputed and compared on validation.
    pub derived_status: VisualInteractionProfileClaimStatus,
    /// The copy / export parity of the certified profile state.
    pub export_parity: VisualInteractionCertExportParity,
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

impl VisualInteractionProfileCertificationRow {
    /// The outcome for a given axis, if present.
    pub fn axis(
        &self,
        axis: VisualInteractionCertificationAxis,
    ) -> Option<&VisualInteractionAxisOutcome> {
        self.axis_outcomes.iter().find(|o| o.axis == axis)
    }

    /// Whether every axis appears exactly once.
    pub fn covers_all_axes(&self) -> bool {
        let seen: BTreeSet<VisualInteractionCertificationAxis> =
            self.axis_outcomes.iter().map(|o| o.axis).collect();
        seen.len() == self.axis_outcomes.len()
            && VisualInteractionCertificationAxis::ALL
                .iter()
                .all(|a| seen.contains(a))
    }

    /// Whether every axis outcome is internally well-formed.
    pub fn axis_outcomes_well_formed(&self) -> bool {
        self.axis_outcomes
            .iter()
            .all(VisualInteractionAxisOutcome::well_formed)
    }

    /// True when the profile narrows its interaction claim below what it asserts.
    pub fn is_claim_narrowed(&self) -> bool {
        self.certified_claim.capability_rank() < self.claimed_claim.capability_rank()
    }

    /// The axes disclosed as narrowed (yellow).
    pub fn narrowed_axes(&self) -> Vec<VisualInteractionCertificationAxis> {
        self.axis_outcomes
            .iter()
            .filter(|o| o.state == VisualInteractionAxisCertificationState::DisclosedNarrowed)
            .map(|o| o.axis)
            .collect()
    }

    /// Derives the profile verdict from its axes, invariants, and claim narrowing. This is the heart of
    /// the capstone: a degraded axis must produce a visible claim narrowing, only a live first-party
    /// profile may certify a trusted interaction surface, every hard invariant must hold, CLI/export
    /// parity must always certify, and the narrowing must be consistent.
    pub fn derive_status(&self) -> VisualInteractionProfileClaimStatus {
        // Structural prerequisites: malformed rows can never certify.
        if !self.covers_all_axes()
            || !self.axis_outcomes_well_formed()
            || self.canonical_bundle_ref != MOTION_LAYER_ICONOGRAPHY_CERT_CANONICAL_BUNDLE_REF
            || self.consumed_families.is_empty()
            || !self.export_parity.is_complete()
        {
            return VisualInteractionProfileClaimStatus::Red;
        }

        // Every B137 hard invariant must hold.
        if !self.guardrails.all_held() {
            return VisualInteractionProfileClaimStatus::Red;
        }

        // Certification may only narrow the claim, never strengthen it.
        if self.certified_claim.capability_rank() > self.claimed_claim.capability_rank() {
            return VisualInteractionProfileClaimStatus::Red;
        }

        // Only a live first-party profile may certify a trusted interaction surface.
        if self.certified_claim.asserts_trusted_surface()
            && !self.profile.is_live_trusted_interaction_surface()
        {
            return VisualInteractionProfileClaimStatus::Red;
        }

        // The always-on CLI/export axis must stay certified.
        match self.axis(VisualInteractionCertificationAxis::CliExport) {
            Some(o) if o.state == VisualInteractionAxisCertificationState::Certified => {}
            _ => return VisualInteractionProfileClaimStatus::Red,
        }

        // Any undisclosed drift blocks outright.
        if self
            .axis_outcomes
            .iter()
            .any(|o| o.state == VisualInteractionAxisCertificationState::UndisclosedDrift)
        {
            return VisualInteractionProfileClaimStatus::Red;
        }

        let narrowed = self.narrowed_axes();
        let claim_narrowed = self.is_claim_narrowed();

        match (&self.claim_auto_narrow, claim_narrowed) {
            // Spurious narrowing structure without a claim reduction.
            (Some(_), false) => return VisualInteractionProfileClaimStatus::Red,
            // A claim reduction with no disclosed narrowing structure.
            (None, true) => return VisualInteractionProfileClaimStatus::Red,
            (Some(narrow), true) => {
                if narrow.from_claim != self.claimed_claim
                    || narrow.to_claim != self.certified_claim
                    || !narrowed.contains(&narrow.binding_axis)
                    || narrow.binding_axis.is_always_on()
                    || narrow.visible_label.trim().is_empty()
                    || label_is_generic(&narrow.visible_label)
                {
                    return VisualInteractionProfileClaimStatus::Red;
                }
            }
            (None, false) => {}
        }

        if claim_narrowed {
            // A disclosed, consistently-bound narrowing.
            return VisualInteractionProfileClaimStatus::Yellow;
        }

        // Claim not narrowed: a degraded axis retained behind a full claim is a hidden overclaim
        // inheriting a healthier profile's truth.
        if !narrowed.is_empty() {
            return VisualInteractionProfileClaimStatus::Red;
        }

        VisualInteractionProfileClaimStatus::Green
    }

    /// Whether the stored `derived_status` matches a fresh recomputation.
    pub fn status_is_fresh(&self) -> bool {
        self.derived_status == self.derive_status()
    }

    /// Whether the row's identity and evidence fields are complete.
    pub fn is_complete(&self) -> bool {
        self.record_kind == MOTION_LAYER_ICONOGRAPHY_CERT_ROW_RECORD_KIND
            && self.schema_version == MOTION_LAYER_ICONOGRAPHY_CERT_SCHEMA_VERSION
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

/// Rolled-up summary of an M05-1155 certification packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct VisualInteractionProfileCertificationSummary {
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

/// Constructor input for [`VisualInteractionProfileCertificationPacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VisualInteractionProfileCertificationPacketInput {
    pub packet_id: String,
    pub as_of: String,
    pub matrix_ref: String,
    pub canonical_bundle_ref: String,
    pub rows: Vec<VisualInteractionProfileCertificationRow>,
}

/// Checked-in M05-1155 certification packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct VisualInteractionProfileCertificationPacket {
    pub schema_version: u32,
    pub record_kind: String,
    pub packet_id: String,
    pub as_of: String,
    pub matrix_ref: String,
    pub canonical_bundle_ref: String,
    #[serde(default)]
    pub rows: Vec<VisualInteractionProfileCertificationRow>,
    pub summary: VisualInteractionProfileCertificationSummary,
}

impl VisualInteractionProfileCertificationPacket {
    /// Builds a packet, stamping the record kind, schema version, and computed summary.
    pub fn new(input: VisualInteractionProfileCertificationPacketInput) -> Self {
        let mut packet = Self {
            schema_version: MOTION_LAYER_ICONOGRAPHY_CERT_SCHEMA_VERSION,
            record_kind: MOTION_LAYER_ICONOGRAPHY_CERT_RECORD_KIND.to_owned(),
            packet_id: input.packet_id,
            as_of: input.as_of,
            matrix_ref: input.matrix_ref,
            canonical_bundle_ref: input.canonical_bundle_ref,
            rows: input.rows,
            summary: VisualInteractionProfileCertificationSummary {
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
    pub fn represented_profiles(&self) -> BTreeSet<M5VisualInteractionCertifiedProfile> {
        self.rows.iter().map(|r| r.profile).collect()
    }

    /// Interaction families rendered by some certified profile in this packet.
    pub fn represented_families(&self) -> BTreeSet<M5VisualInteractionFamily> {
        self.rows
            .iter()
            .flat_map(|r| r.consumed_families.iter().copied())
            .collect()
    }

    /// Whether every certified profile appears exactly once.
    pub fn all_profiles_present(&self) -> bool {
        let profiles = self.represented_profiles();
        profiles.len() == self.rows.len()
            && M5VisualInteractionCertifiedProfile::ALL
                .iter()
                .all(|s| profiles.contains(s))
    }

    /// Whether every frozen interaction family is certified on at least one profile — proof the full
    /// matrix runs across the claimed consumers.
    pub fn all_families_covered(&self) -> bool {
        let families = self.represented_families();
        M5VisualInteractionFamily::ALL
            .iter()
            .all(|f| families.contains(f))
    }

    /// Whether a CLI/export axis is certified on every row.
    pub fn all_rows_export_parity_certified(&self) -> bool {
        self.rows.iter().all(|r| {
            r.axis(VisualInteractionCertificationAxis::CliExport)
                .is_some_and(|o| o.state == VisualInteractionAxisCertificationState::Certified)
                && r.export_parity.is_complete()
        })
    }

    /// Computes summary fields from the packet contents.
    pub fn computed_summary(&self) -> VisualInteractionProfileCertificationSummary {
        let profiles = self.represented_profiles();
        let green = self
            .rows
            .iter()
            .filter(|r| r.derived_status == VisualInteractionProfileClaimStatus::Green)
            .count();
        let yellow = self
            .rows
            .iter()
            .filter(|r| r.derived_status == VisualInteractionProfileClaimStatus::Yellow)
            .count();
        let red = self
            .rows
            .iter()
            .filter(|r| r.derived_status == VisualInteractionProfileClaimStatus::Red)
            .count();
        let all_publishable = self.rows.iter().all(|r| r.derived_status.is_publishable());
        let all_fresh = self
            .rows
            .iter()
            .all(VisualInteractionProfileCertificationRow::status_is_fresh);
        let all_profiles = self.all_profiles_present();
        let all_families = self.all_families_covered();

        VisualInteractionProfileCertificationSummary {
            row_count: self.rows.len(),
            profile_count: profiles.len(),
            green_row_count: green,
            yellow_row_count: yellow,
            red_row_count: red,
            all_profiles_present: all_profiles,
            all_families_covered: all_families,
            all_rows_publishable: all_publishable,
            all_status_fresh: all_fresh,
            all_rows_cite_canonical_bundle: self.rows.iter().all(|r| {
                r.canonical_bundle_ref == MOTION_LAYER_ICONOGRAPHY_CERT_CANONICAL_BUNDLE_REF
            }),
            all_rows_export_parity_certified: self.all_rows_export_parity_certified(),
            all_guardrails_held: self.rows.iter().all(|r| r.guardrails.all_held()),
            every_axis_covered_on_every_row: self
                .rows
                .iter()
                .all(VisualInteractionProfileCertificationRow::covers_all_axes),
            narrowed_profile_count: self.rows.iter().filter(|r| r.is_claim_narrowed()).count(),
            report_clean: all_publishable && all_fresh && all_profiles && all_families,
        }
    }

    /// Validates the packet and returns every contract violation.
    pub fn validate(&self) -> Vec<VisualInteractionCertificationViolation> {
        let mut violations = Vec::new();

        if self.schema_version != MOTION_LAYER_ICONOGRAPHY_CERT_SCHEMA_VERSION {
            violations.push(VisualInteractionCertificationViolation::SchemaVersion {
                expected: MOTION_LAYER_ICONOGRAPHY_CERT_SCHEMA_VERSION,
                actual: self.schema_version,
            });
        }
        if self.record_kind != MOTION_LAYER_ICONOGRAPHY_CERT_RECORD_KIND {
            violations.push(VisualInteractionCertificationViolation::RecordKind {
                expected: MOTION_LAYER_ICONOGRAPHY_CERT_RECORD_KIND.to_owned(),
                actual: self.record_kind.clone(),
            });
        }
        if self.packet_id.trim().is_empty()
            || self.as_of.trim().is_empty()
            || self.matrix_ref.trim().is_empty()
        {
            violations.push(VisualInteractionCertificationViolation::MissingIdentity);
        }
        if self.canonical_bundle_ref != MOTION_LAYER_ICONOGRAPHY_CERT_CANONICAL_BUNDLE_REF {
            violations.push(VisualInteractionCertificationViolation::WrongCanonicalBundle);
        }

        let mut row_ids = BTreeSet::new();
        for row in &self.rows {
            if !row_ids.insert(row.row_id.clone()) {
                violations.push(VisualInteractionCertificationViolation::DuplicateId {
                    id: row.row_id.clone(),
                });
            }

            if !row.is_complete() {
                violations.push(VisualInteractionCertificationViolation::IncompleteRow {
                    id: row.row_id.clone(),
                });
            }

            if !row.covers_all_axes() {
                violations.push(
                    VisualInteractionCertificationViolation::AxisCoverageIncomplete {
                        id: row.row_id.clone(),
                    },
                );
            }

            if !row.axis_outcomes_well_formed() {
                violations.push(
                    VisualInteractionCertificationViolation::MalformedAxisOutcome {
                        id: row.row_id.clone(),
                    },
                );
            }

            if row.canonical_bundle_ref != MOTION_LAYER_ICONOGRAPHY_CERT_CANONICAL_BUNDLE_REF {
                violations.push(
                    VisualInteractionCertificationViolation::RowMissingCanonicalBundle {
                        id: row.row_id.clone(),
                    },
                );
            }

            // Every B137 hard invariant must hold.
            if !row.guardrails.all_held() {
                violations.push(VisualInteractionCertificationViolation::GuardrailViolated {
                    id: row.row_id.clone(),
                });
            }

            // Only a live first-party profile may certify a trusted interaction surface.
            if row.certified_claim.asserts_trusted_surface()
                && !row.profile.is_live_trusted_interaction_surface()
            {
                violations.push(
                    VisualInteractionCertificationViolation::NonLiveProfileClaimsTrustedSurface {
                        id: row.row_id.clone(),
                    },
                );
            }

            // CLI/export parity is always-on.
            if !row.export_parity.is_complete()
                || row
                    .axis(VisualInteractionCertificationAxis::CliExport)
                    .is_none_or_state_not_certified()
            {
                violations.push(
                    VisualInteractionCertificationViolation::ExportParityNotCertified {
                        id: row.row_id.clone(),
                    },
                );
            }

            // Certification may never strengthen a claim.
            if row.certified_claim.capability_rank() > row.claimed_claim.capability_rank() {
                violations.push(
                    VisualInteractionCertificationViolation::CertifiedClaimExceedsClaim {
                        id: row.row_id.clone(),
                    },
                );
            }

            // The stored verdict must match a fresh recomputation.
            if !row.status_is_fresh() {
                violations.push(
                    VisualInteractionCertificationViolation::StatusDerivationStale {
                        id: row.row_id.clone(),
                    },
                );
            }

            // A blocked (red) profile must not ship in a clean packet.
            if row.derived_status == VisualInteractionProfileClaimStatus::Red {
                violations.push(VisualInteractionCertificationViolation::ProfileBlocked {
                    id: row.row_id.clone(),
                });
            }
        }

        // Every claimed profile must be certified exactly once.
        if !self.all_profiles_present() {
            violations.push(VisualInteractionCertificationViolation::ProfileCoverageIncomplete);
        }

        // Every frozen interaction family must be certified on some profile.
        if !self.all_families_covered() {
            violations.push(VisualInteractionCertificationViolation::FamilyCoverageIncomplete);
        }

        if self.summary != self.computed_summary() {
            violations.push(VisualInteractionCertificationViolation::SummaryMismatch);
        }

        if json_contains_forbidden_material(
            &serde_json::to_value(self).expect("certification packet serializes"),
        ) {
            violations
                .push(VisualInteractionCertificationViolation::RawInteractionMaterialInExport);
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
        out.push_str("# M5 Motion-Layer-Iconography Surface Certification\n\n");
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- As of: `{}`\n", self.as_of));
        out.push_str(&format!(
            "- Canonical bundle: `{}`\n",
            self.canonical_bundle_ref
        ));
        out.push_str(&format!(
            "- Profiles: {} / {} certified ({} green, {} yellow, {} red)\n",
            self.summary.profile_count,
            M5VisualInteractionCertifiedProfile::ALL.len(),
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
pub fn current_m5_motion_layer_iconography_surface_certification_export(
) -> Result<VisualInteractionProfileCertificationPacket, VisualInteractionCertificationArtifactError>
{
    let packet: VisualInteractionProfileCertificationPacket = serde_json::from_str(include_str!(
        concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../artifacts/release/m5-motion-layer-iconography-surface-certification/support_export.json"
        )
    ))
    .map_err(VisualInteractionCertificationArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(VisualInteractionCertificationArtifactError::Validation(
            violations,
        ))
    }
}

/// Errors emitted when reading the checked-in certification export.
#[derive(Debug)]
pub enum VisualInteractionCertificationArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<VisualInteractionCertificationViolation>),
}

impl fmt::Display for VisualInteractionCertificationArtifactError {
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

impl Error for VisualInteractionCertificationArtifactError {}

/// Validation failure for M05-1155 certification packets.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum VisualInteractionCertificationViolation {
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
    RawInteractionMaterialInExport,
}

impl fmt::Display for VisualInteractionCertificationViolation {
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
                    "packet does not cite the canonical visual-interaction proof bundle"
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
                    "row {id} does not cite the one canonical visual-interaction proof bundle"
                )
            }
            Self::GuardrailViolated { id } => {
                write!(
                    f,
                    "row {id} breaks a B137 hard invariant: protected input delayed by motion, a scrim \
erasing workspace orientation or contrast, an overlay bypassing the shared z-order, an unlabeled icon \
for an uncommon or destructive action, or an illustration impersonating operational or security truth"
                )
            }
            Self::NonLiveProfileClaimsTrustedSurface { id } => {
                write!(
                    f,
                    "row {id} certifies a trusted interaction surface on a non-live first-party profile"
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
                    "row {id} is blocked (red): a degraded axis is hidden behind a fresh trusted claim, \
a hard invariant broke, CLI/export parity dropped, a non-live profile claimed a trusted interaction \
surface, or the narrowing is inconsistent"
                )
            }
            Self::ProfileCoverageIncomplete => {
                write!(
                    f,
                    "not every claimed M5 visual-interaction profile is certified exactly once"
                )
            }
            Self::FamilyCoverageIncomplete => {
                write!(
                    f,
                    "not every frozen visual-interaction family is certified on some profile"
                )
            }
            Self::SummaryMismatch => write!(f, "computed summary does not match stored summary"),
            Self::RawInteractionMaterialInExport => {
                write!(
                    f,
                    "export contains a raw duration curve, z-index integer, glyph blob, credential, or secret material"
                )
            }
        }
    }
}

impl Error for VisualInteractionCertificationViolation {}

/// Small extension so the export-parity check reads cleanly.
trait AxisOutcomeOptionExt {
    fn is_none_or_state_not_certified(&self) -> bool;
}

impl AxisOutcomeOptionExt for Option<&VisualInteractionAxisOutcome> {
    fn is_none_or_state_not_certified(&self) -> bool {
        match self {
            None => true,
            Some(o) => o.state != VisualInteractionAxisCertificationState::Certified,
        }
    }
}

/// Whether a label is a generic non-answer rather than a precise disclosure. Includes the
/// visual-interaction generics the spec forbids collapsing distinct motion, scrim, layer, portal, icon,
/// and illustration truth into (whole-label matches so a full sentence naming a concrete role, token, or
/// accessible fallback is not flagged).
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
            | "motion"
            | "scrim"
            | "overlay"
            | "layer"
            | "portal"
            | "icon"
            | "illustration"
            | "z-order"
            | "z order"
            | "opacity"
            | "semantic role"
            | "token reference"
            | "motion profile"
            | "layer tier"
            | "accessible fallback"
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

/// Builds the canonical, checked-in M05-1155 certification packet. Certifies all seven claimed M5
/// desktop / dialog / onboarding / notification / embedded visual-interaction profiles: two deliver their
/// claim (green) and five auto-narrow a not-current truth axis to a weaker interaction ceiling (yellow).
/// No profile hides drift or breaks a hard invariant (red).
pub fn seeded_m5_motion_layer_iconography_surface_certification_packet(
) -> VisualInteractionProfileCertificationPacket {
    VisualInteractionProfileCertificationPacket::new(
        VisualInteractionProfileCertificationPacketInput {
            packet_id: MOTION_LAYER_ICONOGRAPHY_CERT_PACKET_ID.to_owned(),
            as_of: "2026-07-13T00:00:00Z".to_owned(),
            matrix_ref: MOTION_LAYER_ICONOGRAPHY_CERT_MATRIX_REF.to_owned(),
            canonical_bundle_ref: MOTION_LAYER_ICONOGRAPHY_CERT_CANONICAL_BUNDLE_REF.to_owned(),
            rows: seeded_rows(),
        },
    )
}

fn seed_evidence(id: &str) -> Vec<String> {
    vec![
        format!("evidence:motion-layer-iconography-surface-certification:{id}"),
        MOTION_LAYER_ICONOGRAPHY_CERT_A11Y_BUNDLE_REF.to_owned(),
    ]
}

fn seed_export_parity(fields: &[&str]) -> VisualInteractionCertExportParity {
    VisualInteractionCertExportParity {
        formats: vec!["text".to_owned(), "json".to_owned(), "markdown".to_owned()],
        export_fields: fields.iter().map(|f| (*f).to_owned()).collect(),
        raw_payload_only_prohibited: true,
    }
}

fn seed_certified_note(axis: VisualInteractionCertificationAxis) -> &'static str {
    match axis {
        VisualInteractionCertificationAxis::Visual => {
            "canonical role, semantic meaning, token reference, motion profile, layer tier, and accessible fallback shown on-surface without motion or decoration alone"
        }
        VisualInteractionCertificationAxis::Keyboard => {
            "the same interaction role, token reference, and bound controls are keyboard-reachable, never hover-only"
        }
        VisualInteractionCertificationAxis::ScreenReader => {
            "the same visual-interaction truth is announced non-visually, never motion/decoration/unlabeled-symbol-only"
        }
        VisualInteractionCertificationAxis::HighZoomReflow => {
            "the same truth reflows legibly at high zoom without clipping the role, token reference, layer tier, or accessible fallback"
        }
        VisualInteractionCertificationAxis::ReducedMotion => {
            "the same truth stays legible and usable with reduced motion, never motion-only"
        }
        VisualInteractionCertificationAxis::PowerThermal => {
            "the same truth stays legible and usable under battery-saver and thermal-pressure clamps, never dependent on a motion the clamp suppresses"
        }
        VisualInteractionCertificationAxis::CliExport => {
            "profile state exports as text / JSON / Markdown for support replay"
        }
        VisualInteractionCertificationAxis::DegradedState => {
            "stale motion-timing evidence, an unconfirmed reduced-motion clamp, unconfirmed scrim orientation, an unconfirmed owning-surface attachment, or a partially-disclosed illustration boundary honestly downgrades the TrustedInteractionSurface/ReviewableInteractionSurface claim rather than reading as a fresh authoritative interaction surface"
        }
        VisualInteractionCertificationAxis::VisualInteractionComponentTruth => {
            "canonical role, semantic meaning, token reference, motion profile, layer tier, and accessible fallback stay explicit and never delay protected input with motion, let a scrim erase workspace orientation or contrast, let an overlay bypass the shared z-order, use an unlabeled icon for an uncommon or destructive action, or let an illustration impersonate operational or security truth"
        }
    }
}

fn seed_certified(axis: VisualInteractionCertificationAxis) -> VisualInteractionAxisOutcome {
    VisualInteractionAxisOutcome {
        axis,
        state: VisualInteractionAxisCertificationState::Certified,
        parity_note: seed_certified_note(axis).to_owned(),
        narrowing_reason: None,
        downgrade_trigger: None,
    }
}

fn seed_narrowed(
    axis: VisualInteractionCertificationAxis,
    note: &str,
    reason: &str,
    trigger: M5VisualInteractionDowngradeTrigger,
) -> VisualInteractionAxisOutcome {
    VisualInteractionAxisOutcome {
        axis,
        state: VisualInteractionAxisCertificationState::DisclosedNarrowed,
        parity_note: note.to_owned(),
        narrowing_reason: Some(reason.to_owned()),
        downgrade_trigger: Some(trigger),
    }
}

fn seed_all_certified() -> Vec<VisualInteractionAxisOutcome> {
    VisualInteractionCertificationAxis::ALL
        .iter()
        .copied()
        .map(seed_certified)
        .collect()
}

fn seed_certified_except(
    axis: VisualInteractionCertificationAxis,
    outcome: VisualInteractionAxisOutcome,
) -> Vec<VisualInteractionAxisOutcome> {
    VisualInteractionCertificationAxis::ALL
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
    profile: M5VisualInteractionCertifiedProfile,
    claimed_claim: M5VisualInteractionA11yClaim,
    certified_claim: M5VisualInteractionA11yClaim,
    consumed_families: &[M5VisualInteractionFamily],
    axis_outcomes: Vec<VisualInteractionAxisOutcome>,
    claim_auto_narrow: Option<VisualInteractionClaimAutoNarrow>,
    export_fields: &[&str],
    compatibility_notes: &[&str],
) -> VisualInteractionProfileCertificationRow {
    let mut row = VisualInteractionProfileCertificationRow {
        record_kind: MOTION_LAYER_ICONOGRAPHY_CERT_ROW_RECORD_KIND.to_owned(),
        schema_version: MOTION_LAYER_ICONOGRAPHY_CERT_SCHEMA_VERSION,
        row_id: row_id.to_owned(),
        profile,
        claimed_claim,
        certified_claim,
        consumed_families: consumed_families.to_vec(),
        axis_outcomes,
        guardrails: VisualInteractionCertGuardrails::CLEAN,
        claim_auto_narrow,
        canonical_bundle_ref: MOTION_LAYER_ICONOGRAPHY_CERT_CANONICAL_BUNDLE_REF.to_owned(),
        derived_status: VisualInteractionProfileClaimStatus::Green,
        export_parity: seed_export_parity(export_fields),
        compatibility_notes: compatibility_notes
            .iter()
            .map(|n| (*n).to_owned())
            .collect(),
        source_refs: vec![
            MOTION_LAYER_ICONOGRAPHY_CERT_MATRIX_REF.to_owned(),
            MOTION_LAYER_ICONOGRAPHY_CERT_SCHEMA_REF.to_owned(),
        ],
        observed_at: "2026-07-13T00:00:00Z".to_owned(),
        evidence_refs: seed_evidence(row_id),
    };
    row.derived_status = row.derive_status();
    row
}

fn seed_narrow(
    binding_axis: VisualInteractionCertificationAxis,
    from_claim: M5VisualInteractionA11yClaim,
    to_claim: M5VisualInteractionA11yClaim,
    label: &str,
) -> VisualInteractionClaimAutoNarrow {
    VisualInteractionClaimAutoNarrow {
        binding_axis,
        from_claim,
        to_claim,
        visible_label: label.to_owned(),
    }
}

fn seeded_rows() -> Vec<VisualInteractionProfileCertificationRow> {
    use M5VisualInteractionA11yClaim::*;
    use M5VisualInteractionCertifiedProfile as P;
    use M5VisualInteractionDowngradeTrigger as Trig;
    use M5VisualInteractionFamily::*;
    use VisualInteractionCertificationAxis as Ax;

    vec![
        // --- Green: full parity, claim delivered ---------------------------
        seed_row(
            "cert:live-trusted-interaction-surface",
            P::LiveTrustedInteractionSurface,
            TrustedInteractionSurface,
            TrustedInteractionSurface,
            &[Iconography],
            seed_all_certified(),
            None,
            &["profile", "claimed_claim", "certified_claim", "status", "semantic_role"],
            &[
                "local profile: the iconography surface names its canonical semantic icon classes and keeps each one labeled and distinct, never an unlabeled icon standing in for an uncommon or destructive action",
                "the trusted interaction surface pairs every icon and motion role with an accessible fallback rather than a motion-only or decoration-only signal",
                "keyboard / screen-reader / high-zoom / reduced-motion / power-thermal reach preserved for the rendered icon surface",
                "visual-interaction-component-truth: a live first-party interaction surface is the only profile that certifies a trusted interaction surface",
            ],
        ),
        seed_row(
            "cert:reviewable-layer-structure",
            P::ReviewableLayerStructure,
            ReviewableInteractionSurface,
            ReviewableInteractionSurface,
            &[LayerOrder],
            seed_all_certified(),
            None,
            &["profile", "claimed_claim", "certified_claim", "status", "layer_tier"],
            &[
                "managed profile: the z-order layer structure stays tier-ordered and machine-readable rather than a private always-on-top layer fork that bypasses the shared z-order",
                "the layer surface keeps its base / sticky / floating / menu / dialog / toast / critical tier order inspectable rather than a motion-only or decoration-only cue",
                "text / JSON / Markdown reconstruction certified so support can replay the reviewable layer structure",
                "visual-interaction-component-truth: a reviewable layer structure never certifies a live trusted, authoritative interaction claim",
            ],
        ),
        // --- Yellow: an axis is not current; the claim narrows visibly ------
        seed_row(
            "cert:stale-motion-timing-surface",
            P::StaleMotionTimingSurface,
            ReviewableInteractionSurface,
            MotionTimingUnverifiedProjection,
            &[MotionToken],
            seed_certified_except(
                Ax::DegradedState,
                seed_narrowed(
                    Ax::DegradedState,
                    "the motion token's protected-path timing evidence is stale so a fresh, protected-path-safe motion cannot be certified",
                    "The motion token's protected-path timing evidence is stale, so the ReviewableInteractionSurface claim narrows to a motion-timing-unverified projection and the interaction preserves its last-known semantic role and static fallback rather than presenting a fresh, protected-path-delaying motion as authoritative",
                    Trig::MotionDelayedProtectedInput,
                ),
            ),
            Some(seed_narrow(
                Ax::DegradedState,
                ReviewableInteractionSurface,
                MotionTimingUnverifiedProjection,
                "Motion timing unverified: the protected-path timing evidence is stale so the last-known semantic role and static fallback are preserved and the motion never delays protected input",
            )),
            &["profile", "claimed_claim", "certified_claim", "status", "binding_axis"],
            &[
                "remote profile: the interaction preserves its last-known semantic role and static fallback and marks the motion timing as unverified rather than presenting a stale, protected-path-delaying motion as authoritative",
                "the motion surface keeps every protected-path affordance reachable without waiting on motion while the timing evidence is disclosed as stale",
                "degraded-state: ReviewableInteractionSurface narrows to a motion-timing-unverified projection (auto-narrowed)",
                "visual-interaction-component-truth: protected input on menus, palette, and typing-critical surfaces is never delayed by motion",
            ],
        ),
        seed_row(
            "cert:unconfirmed-reduced-motion-surface",
            P::UnconfirmedReducedMotionSurface,
            ReviewableInteractionSurface,
            ReducedMotionClampUnverifiedProjection,
            &[ReducedMotion],
            seed_certified_except(
                Ax::PowerThermal,
                seed_narrowed(
                    Ax::PowerThermal,
                    "the reduced-motion / power-saver / thermal clamp cannot be confirmed so a clamp-safe, static-fallback interaction cannot be certified",
                    "The reduced-motion / power-saver / thermal clamp cannot be confirmed, so the ReviewableInteractionSurface claim narrows to a reduced-motion-clamp-unverified projection and the interaction keeps its last-known static fallback explicit rather than presenting a motion-only cue as clamp-safe",
                    Trig::MotionMeaningLostUnderReducedMotion,
                ),
            ),
            Some(seed_narrow(
                Ax::PowerThermal,
                ReviewableInteractionSurface,
                ReducedMotionClampUnverifiedProjection,
                "Reduced-motion clamp unverified: the battery-saver and thermal-pressure clamp cannot be confirmed so the last-known static fallback stays explicit and no meaning is conveyed by motion alone",
            )),
            &["profile", "claimed_claim", "certified_claim", "status", "binding_axis"],
            &[
                "power-constrained profile: the interaction keeps its last-known static fallback explicit and marks the clamp as unverified rather than presenting a motion-only cue as clamp-safe under battery-saver or thermal pressure",
                "the reduced-motion surface keeps its static fallback legible across the clamps it can prove while the power-saver / thermal clamp is disclosed as unverified",
                "power-thermal: ReviewableInteractionSurface narrows to a reduced-motion-clamp-unverified projection (auto-narrowed)",
                "visual-interaction-component-truth: reduced-motion, power-saver, and thermal clamps are respected rather than assumed",
            ],
        ),
        seed_row(
            "cert:orientation-erasing-scrim-surface",
            P::OrientationErasingScrimSurface,
            ReviewableInteractionSurface,
            ScrimOrientationUnverifiedProjection,
            &[OpacityScrim],
            seed_certified_except(
                Ax::DegradedState,
                seed_narrowed(
                    Ax::DegradedState,
                    "the opacity scrim's orientation / contrast preservation cannot be confirmed so an orientation-safe scrim cannot be certified",
                    "The opacity scrim's orientation / contrast preservation cannot be confirmed, so the ReviewableInteractionSurface claim narrows to a scrim-orientation-unverified projection and the interaction keeps the workspace orientation cue inspectable rather than presenting a scrim as orientation-safe when it may erase context",
                    Trig::ScrimErasedOrientationOrContrast,
                ),
            ),
            Some(seed_narrow(
                Ax::DegradedState,
                ReviewableInteractionSurface,
                ScrimOrientationUnverifiedProjection,
                "Scrim orientation unverified: the orientation / contrast preservation cannot be confirmed so the workspace orientation cue stays inspectable and the scrim never erases workspace orientation or contrast",
            )),
            &["profile", "claimed_claim", "certified_claim", "status", "binding_axis"],
            &[
                "mirrored profile: the interaction keeps its workspace orientation cue and contrast inspectable and marks the scrim orientation as unverified rather than presenting a scrim that may erase context as orientation-safe",
                "the scrim surface keeps its lightweight-versus-blocking overlay depth legible while the orientation preservation is disclosed as unverified",
                "degraded-state: ReviewableInteractionSurface narrows to a scrim-orientation-unverified projection (auto-narrowed)",
                "visual-interaction-component-truth: a scrim never erases workspace orientation or contrast",
            ],
        ),
        seed_row(
            "cert:detached-portal-surface",
            P::DetachedPortalSurface,
            ReviewableInteractionSurface,
            PortalOwnershipUnverifiedProjection,
            &[PortalOwnership],
            seed_certified_except(
                Ax::VisualInteractionComponentTruth,
                seed_narrowed(
                    Ax::VisualInteractionComponentTruth,
                    "the portal's owning-surface attachment cannot be confirmed so an owning-surface-attached overlay cannot be certified",
                    "The portal's owning-surface attachment cannot be confirmed, so the ReviewableInteractionSurface claim narrows to a portal-ownership-unverified projection and the interaction keeps the owning-surface reference and z-tier inspectable rather than presenting an overlay as attached when it may bypass the shared z-order",
                    Trig::PortalDetachedFromOwningSurface,
                ),
            ),
            Some(seed_narrow(
                Ax::VisualInteractionComponentTruth,
                ReviewableInteractionSurface,
                PortalOwnershipUnverifiedProjection,
                "Portal ownership unverified: the owning-surface attachment cannot be confirmed so the owning-surface reference and z-tier stay inspectable and the overlay never bypasses the shared z-order",
            )),
            &["profile", "claimed_claim", "certified_claim", "status", "binding_axis"],
            &[
                "managed-embedded profile: the interaction keeps its owning-surface reference and z-tier inspectable and marks the portal ownership as unverified rather than presenting a detached overlay as attached",
                "the portal surface keeps its restore-safe owning-surface attachment legible while the attachment is disclosed as unverified",
                "visual-interaction-component-truth: ReviewableInteractionSurface narrows to a portal-ownership-unverified projection (auto-narrowed)",
                "visual-interaction-component-truth: an extension or private overlay never bypasses the shared z-order model",
            ],
        ),
        seed_row(
            "cert:impersonating-illustration-surface",
            P::ImpersonatingIllustrationSurface,
            ReviewableInteractionSurface,
            IllustrationBoundaryDisclosedProjection,
            &[Illustration],
            seed_certified_except(
                Ax::VisualInteractionComponentTruth,
                seed_narrowed(
                    Ax::VisualInteractionComponentTruth,
                    "the illustration boundary can only be partially disclosed so a fully-proven secondary-illustration boundary cannot be certified",
                    "The illustration boundary can only be partially disclosed, so the ReviewableInteractionSurface claim narrows to an illustration-boundary-disclosed projection and the interaction discloses the partial secondary-illustration boundary inspectably rather than letting a decorative illustration impersonate operational or security truth",
                    Trig::ProofStale,
                ),
            ),
            Some(seed_narrow(
                Ax::VisualInteractionComponentTruth,
                ReviewableInteractionSurface,
                IllustrationBoundaryDisclosedProjection,
                "Illustration boundary disclosed partial: the secondary-illustration boundary is only partially proven so it is disclosed inspectably and no illustration impersonates operational or security truth",
            )),
            &["profile", "claimed_claim", "certified_claim", "status", "binding_axis"],
            &[
                "accessibility-sensitive profile: the interaction discloses its partial secondary-illustration boundary and keeps the illustration decorative rather than letting it impersonate operational state, safety approval, or security messaging",
                "the illustration surface keeps its secondary, non-anthropomorphic placement legible while the boundary is disclosed as partial",
                "visual-interaction-component-truth: ReviewableInteractionSurface narrows to an illustration-boundary-disclosed projection (auto-narrowed)",
                "visual-interaction-component-truth: an illustration never impersonates operational state, safety approval, or security messaging",
            ],
        ),
    ]
}
