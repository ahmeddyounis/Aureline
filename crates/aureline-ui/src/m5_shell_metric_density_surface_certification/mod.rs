//! M05-1163 surface certification over the frozen M5 shell-metric / minimum-size / density-mode /
//! responsive-geometry / collapse-priority shell-geometry matrix.
//!
//! Where the freeze matrix ([`crate::m5_shell_metric_density_matrix`]) defines the five governed
//! shell-geometry families, the M05-1157..1159 implement lanes narrow each one, the M05-1160 monitor-remap
//! lane proves mixed-DPI restore, the M05-1161 shared-consumer lane aligns their grammar across surfaces,
//! and the M05-1162 accessibility lane
//! ([`crate::m5_shell_metric_density_accessibility_parity_and_narrowing_when_shell_metric_density_or_adaptive_geometry_truth_is_stale`])
//! proves keyboard / screen-reader / high-zoom / high-contrast / snapped-width / CLI-export parity and
//! per-family auto-narrowing, this closing capstone *certifies* that the shared shell-geometry truth holds
//! on every claimed M5 desktop operating profile — and auto-narrows any profile that cannot sustain it.
//!
//! It is keyed on the claimed **profile** a user, reviewer, or support engineer reads a shell-metric,
//! minimum-size, density, responsive, or collapse surface through (a live, first-party trusted geometry
//! surface; a reviewable geometry structure; an unverified density-mode surface; an unverified
//! responsive-geometry surface; and a disclosed collapse-priority surface), not on geometry family or
//! implement lane. Each [`ShellGeometryProfileCertificationRow`] certifies one profile across nine truth
//! axes — visual, keyboard, screen-reader, high-zoom-reflow, high-contrast, snapped-width, CLI/export,
//! degraded-state, and shell-geometry-component-truth behavior — and either passes (green), auto-narrows
//! its geometry claim to the weakest supported ceiling (yellow), or is blocked (red) when a degraded axis
//! is hidden behind a fresh trusted claim inherited from a healthier profile.
//!
//! The invariant is: **a degraded axis must produce a visible claim narrowing**. A profile that keeps a
//! `TrustedGeometrySurface` / `ReviewableGeometrySurface` claim while one of its truth axes is not current
//! is over-claiming and blocks; a profile that discloses the reduction by narrowing its claim (with a bound
//! reason and a frozen downgrade trigger) is honestly yellow. Only a live, first-party trusted geometry
//! profile may certify a `TrustedGeometrySurface` claim — a reviewable, unverified-density,
//! unverified-responsive, or disclosed-collapse profile that keeps a trusted claim is over-reaching and
//! blocks. The always-on CLI/export axis must always stay certified so support and automation can
//! reconstruct the canonical zone metric, minimum size, density mode, responsive class, and registry
//! reference from the same geometry the user saw.
//!
//! The B138 hard invariants are enforced per row: no profile may let density or collapse change command
//! meaning, focus order, or trust visibility, let an extension or embedded surface set a private fracturing
//! width, shrink a hit target below the supported minimum, hide a primary workflow behind an overlay-only
//! fallback, or let a zone starve the main workspace. A profile that breaches any invariant blocks (red).
//!
//! Every row cites exactly one canonical shell-geometry proof bundle
//! ([`SHELL_METRIC_DENSITY_CERT_CANONICAL_BUNDLE_REF`]) — the frozen shell-metric-density matrix proof —
//! rather than cloning per-profile evidence. The packet is metadata-only: raw pixel geometry, absolute
//! window coordinates, z-index integers, credentials, secrets, and endpoint refs never cross this boundary.
//!
//! The boundary schema is
//! [`schemas/shell/m5-shell-metric-density-surface-certification.schema.json`](../../../../schemas/shell/m5-shell-metric-density-surface-certification.schema.json).
//! The contract doc is
//! [`docs/design-system/m5_shell_metric_density_surface_certification.md`](../../../../docs/design-system/m5_shell_metric_density_surface_certification.md).

#[cfg(test)]
mod tests;

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

use crate::m5_shell_metric_density_accessibility_parity_and_narrowing_when_shell_metric_density_or_adaptive_geometry_truth_is_stale as a11y;
use crate::m5_shell_metric_density_matrix as matrix;
use a11y::M5ShellGeometryA11yClaim;
use matrix::{M5ShellGeometryDowngradeTrigger, M5ShellGeometryFamily};

/// Schema version stamped on the M05-1163 certification packet.
pub const SHELL_METRIC_DENSITY_CERT_SCHEMA_VERSION: u32 = 1;

/// Stable record-kind tag carried by [`ShellGeometryProfileCertificationPacket`].
pub const SHELL_METRIC_DENSITY_CERT_RECORD_KIND: &str =
    "m5_shell_metric_density_surface_certification_packet";

/// Stable record-kind tag carried by each [`ShellGeometryProfileCertificationRow`].
pub const SHELL_METRIC_DENSITY_CERT_ROW_RECORD_KIND: &str =
    "m5_shell_metric_density_surface_certification_row";

/// Repo-relative path of the boundary schema.
pub const SHELL_METRIC_DENSITY_CERT_SCHEMA_REF: &str =
    "schemas/shell/m5-shell-metric-density-surface-certification.schema.json";

/// Repo-relative path of the contract doc.
pub const SHELL_METRIC_DENSITY_CERT_DOC_REF: &str =
    "docs/design-system/m5_shell_metric_density_surface_certification.md";

/// Repo-relative path of the frozen shell-geometry matrix schema the certified profiles render.
pub const SHELL_METRIC_DENSITY_CERT_MATRIX_REF: &str =
    matrix::M5_SHELL_METRIC_DENSITY_MATRIX_SCHEMA_REF;

/// The one canonical shell-geometry proof bundle every certified profile cites as its first-resolved
/// geometry truth. All five profiles point back to it rather than cloning per-profile evidence.
pub const SHELL_METRIC_DENSITY_CERT_CANONICAL_BUNDLE_REF: &str =
    matrix::M5_SHELL_METRIC_DENSITY_ARTIFACT_REF;

/// The M05-1162 accessibility support export the certification builds on. Recorded as a supporting
/// evidence ref on every row.
pub const SHELL_METRIC_DENSITY_CERT_A11Y_BUNDLE_REF: &str =
    a11y::SHELL_METRIC_DENSITY_A11Y_ARTIFACT_REF;

/// Repo-relative path of the checked support-export artifact (the `include_str!` canonical).
pub const SHELL_METRIC_DENSITY_CERT_ARTIFACT_REF: &str =
    "artifacts/release/m5-shell-metric-density-surface-certification/support_export.json";

/// Repo-relative path of the checked machine-readable matrix CSV.
pub const SHELL_METRIC_DENSITY_CERT_CSV_REF: &str =
    "artifacts/release/m5-shell-metric-density-surface-certification/matrix.csv";

/// Repo-relative path of the checked Markdown report.
pub const SHELL_METRIC_DENSITY_CERT_REPORT_REF: &str =
    "artifacts/release/m5-shell-metric-density-surface-certification.md";

/// Stable packet id for the checked-in certification bundle.
pub const SHELL_METRIC_DENSITY_CERT_PACKET_ID: &str =
    "m5-shell-metric-density-surface-certification:stable:0001";

/// The five claimed M5 desktop shell-geometry operating profiles this capstone certifies. Keyed on the
/// profile a user, reviewer, or support engineer reads a shell-metric, minimum-size, density, responsive,
/// or collapse surface through, not on the reusable geometry family it renders. Only a live, first-party
/// trusted geometry profile may certify a trusted geometry surface.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ShellGeometryCertifiedProfile {
    /// A live, first-party, fully-current geometry surface — a minimum-honoring, registry-bound shell
    /// rendering the trusted, workspace-dominant geometry exactly right now.
    LiveTrustedGeometrySurface,
    /// A reviewable geometry structure: a self-sufficient, inspectable zone-metric / registry reference a
    /// user can review, never itself an authoritative, live-rendering geometry surface.
    ReviewableGeometryStructure,
    /// A density-mode surface whose presentation-only safety cannot be confirmed; the claim narrows to a
    /// density-mode-unverified projection with the last-known information architecture preserved, never a
    /// density change shown as safe when it may rearrange focus order or trust visibility.
    UnverifiedDensityModeSurface,
    /// A responsive-geometry surface whose recovery-state preservation cannot be confirmed; the claim
    /// narrows to a responsive-geometry-unverified projection that keeps task identity and recovery-critical
    /// state inspectable, never a snapped-width collapse shown as recovery-safe when it may drop state.
    UnverifiedResponsiveGeometrySurface,
    /// A collapse-priority surface that can only disclose a partial collapse boundary; the claim narrows to
    /// a collapse-priority-disclosed projection disclosing the partial boundary, never a private width shown
    /// as workspace-dominant when the shell may fracture.
    DisclosedCollapsePrioritySurface,
}

impl M5ShellGeometryCertifiedProfile {
    /// Every certified profile, in declaration order.
    pub const ALL: [M5ShellGeometryCertifiedProfile; 5] = [
        M5ShellGeometryCertifiedProfile::LiveTrustedGeometrySurface,
        M5ShellGeometryCertifiedProfile::ReviewableGeometryStructure,
        M5ShellGeometryCertifiedProfile::UnverifiedDensityModeSurface,
        M5ShellGeometryCertifiedProfile::UnverifiedResponsiveGeometrySurface,
        M5ShellGeometryCertifiedProfile::DisclosedCollapsePrioritySurface,
    ];

    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LiveTrustedGeometrySurface => "live_trusted_geometry_surface",
            Self::ReviewableGeometryStructure => "reviewable_geometry_structure",
            Self::UnverifiedDensityModeSurface => "unverified_density_mode_surface",
            Self::UnverifiedResponsiveGeometrySurface => "unverified_responsive_geometry_surface",
            Self::DisclosedCollapsePrioritySurface => "disclosed_collapse_priority_surface",
        }
    }

    /// True only for the live, first-party trusted geometry surface profile. A trusted geometry surface may
    /// be certified on this profile alone; every other profile is at most a reviewable geometry structure or
    /// a narrowed projection.
    pub const fn is_live_trusted_geometry_surface(self) -> bool {
        matches!(self, Self::LiveTrustedGeometrySurface)
    }
}

/// The nine truth axes a certified profile is scored on. These are exactly the parity dimensions the spec
/// requires verifying — visual, keyboard, screen-reader, high-zoom reflow, high-contrast, snapped-width,
/// CLI/export, degraded-state, and shell-geometry-component-truth behavior. The CLI/export axis is
/// always-on and must stay certified for every profile.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ShellGeometryCertificationAxis {
    /// Visual parity: canonical zone metric, minimum size, density mode, responsive class, and registry
    /// reference are shown on the primary surface without relying on a private width or an off-screen zone
    /// alone.
    Visual,
    /// Keyboard-reach parity: the same geometry truth and its bound controls are reachable and operable
    /// without a pointer, never hover-only.
    Keyboard,
    /// Screen-reader parity: the same truth is announced non-visually, never relying on a private width, an
    /// off-screen zone, or an unlabeled control alone.
    ScreenReader,
    /// High-zoom reflow parity: the same truth reflows legibly at 200-400% zoom rather than clipping the
    /// zone metric, registry reference, density mode, or minimum size.
    HighZoomReflow,
    /// High-contrast parity: the same truth stays legible and operable in high-contrast mode, never
    /// dropping the zone metric, registry reference, or minimum hit target.
    HighContrast,
    /// Snapped-width parity: the same truth preserves task identity and recovery-critical state under
    /// snapped and narrow widths, never collapsing a primary workflow into an overlay-only fallback.
    SnappedWidth,
    /// CLI / export parity (always-on): the certified profile state is reconstructable as
    /// text / JSON / Markdown for support and automation.
    CliExport,
    /// Degraded-state parity: a stale shell metric, an unconfirmed density change, an unconfirmed
    /// responsive collapse, or a partially-disclosed collapse boundary honestly downgrades a
    /// `TrustedGeometrySurface` / `ReviewableGeometrySurface` claim rather than reading as a fresh,
    /// authoritative geometry surface.
    DegradedState,
    /// Shell-geometry-component-truth parity: canonical zone metric, minimum size, density mode, responsive
    /// class, and registry reference stay explicit and never let density or collapse change command
    /// meaning, focus order, or trust visibility, let an extension or embedded surface set a private
    /// fracturing width, shrink a hit target below the supported minimum, hide a primary workflow behind an
    /// overlay-only fallback, or let a zone starve the main workspace.
    ShellGeometryComponentTruth,
}

impl ShellGeometryCertificationAxis {
    /// Every certification axis, in declaration order.
    pub const ALL: [ShellGeometryCertificationAxis; 9] = [
        ShellGeometryCertificationAxis::Visual,
        ShellGeometryCertificationAxis::Keyboard,
        ShellGeometryCertificationAxis::ScreenReader,
        ShellGeometryCertificationAxis::HighZoomReflow,
        ShellGeometryCertificationAxis::HighContrast,
        ShellGeometryCertificationAxis::SnappedWidth,
        ShellGeometryCertificationAxis::CliExport,
        ShellGeometryCertificationAxis::DegradedState,
        ShellGeometryCertificationAxis::ShellGeometryComponentTruth,
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
            Self::HighContrast => "high_contrast",
            Self::SnappedWidth => "snapped_width",
            Self::CliExport => "cli_export",
            Self::DegradedState => "degraded_state",
            Self::ShellGeometryComponentTruth => "shell_geometry_component_truth",
        }
    }
}

/// The certification state of one truth axis on one profile.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ShellGeometryAxisCertificationState {
    /// Green: parity is current; the axis fully certifies.
    Certified,
    /// Yellow: parity is not current, but the reduction is disclosed and binds to a visible claim
    /// narrowing.
    DisclosedNarrowed,
    /// Red: parity is not current and the profile hides it behind a trusted claim inherited from a
    /// healthier profile.
    UndisclosedDrift,
}

impl ShellGeometryAxisCertificationState {
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
pub enum ShellGeometryProfileClaimStatus {
    /// Full standing: every axis certified, every invariant held, claimed geometry tier delivered.
    Green,
    /// Disclosed narrowing: an axis is not current and the claim narrows visibly.
    Yellow,
    /// Blocked: a degraded axis hides behind a full claim, a hard invariant breaks, CLI/export parity
    /// drops, a non-live profile claims a trusted geometry surface, or the narrowing is inconsistent.
    Red,
}

impl ShellGeometryProfileClaimStatus {
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

/// The five B138 hard invariants carried on every certified profile. All five must hold — a breach blocks
/// the profile (red). Each field is `true` only when the profile *breaks* the invariant, so a clean
/// profile carries all-false.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct ShellGeometryCertGuardrails {
    /// True if the profile lets a density change or responsive collapse change command meaning, focus
    /// order, or trust visibility. Must be false.
    pub lets_density_or_collapse_change_command_focus_or_trust: bool,
    /// True if the profile lets an extension or embedded surface set a private fracturing width. Must be
    /// false.
    pub lets_an_extension_or_embedded_surface_set_a_private_fracturing_width: bool,
    /// True if the profile shrinks a hit target below the supported minimum. Must be false.
    pub shrinks_a_hit_target_below_the_supported_minimum: bool,
    /// True if the profile hides a primary workflow behind an overlay-only fallback. Must be false.
    pub hides_a_primary_workflow_behind_an_overlay_only_fallback: bool,
    /// True if the profile lets a zone starve the main workspace below its minimum. Must be false.
    pub lets_a_zone_starve_the_main_workspace: bool,
}

impl ShellGeometryCertGuardrails {
    /// A clean profile: every invariant held.
    pub const CLEAN: Self = Self {
        lets_density_or_collapse_change_command_focus_or_trust: false,
        lets_an_extension_or_embedded_surface_set_a_private_fracturing_width: false,
        shrinks_a_hit_target_below_the_supported_minimum: false,
        hides_a_primary_workflow_behind_an_overlay_only_fallback: false,
        lets_a_zone_starve_the_main_workspace: false,
    };

    /// True when every invariant holds (no field is set).
    pub const fn all_held(&self) -> bool {
        !self.lets_density_or_collapse_change_command_focus_or_trust
            && !self.lets_an_extension_or_embedded_surface_set_a_private_fracturing_width
            && !self.shrinks_a_hit_target_below_the_supported_minimum
            && !self.hides_a_primary_workflow_behind_an_overlay_only_fallback
            && !self.lets_a_zone_starve_the_main_workspace
    }
}

/// The copy / export parity a certified profile preserves. The CLI/export axis certifies only when this
/// offers text / JSON / Markdown reconstruction and prohibits a raw-payload-only export.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ShellGeometryCertExportParity {
    /// The copy formats the profile offers (must include text / json / markdown).
    #[serde(default)]
    pub formats: Vec<String>,
    /// The zone-metric / minimum-size / density-mode / responsive-class / registry-reference fields the
    /// profile preserves in export.
    #[serde(default)]
    pub export_fields: Vec<String>,
    /// True when a raw-payload-only export is prohibited.
    pub raw_payload_only_prohibited: bool,
}

impl ShellGeometryCertExportParity {
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
pub struct ShellGeometryAxisOutcome {
    /// The truth axis this outcome scores.
    pub axis: ShellGeometryCertificationAxis,
    /// The certification state of the axis.
    pub state: ShellGeometryAxisCertificationState,
    /// The parity note recorded for this axis (always present).
    pub parity_note: String,
    /// The narrowing reason; present iff the axis is not certified.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub narrowing_reason: Option<String>,
    /// The frozen downgrade trigger; present iff the axis is disclosed-narrowed.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub downgrade_trigger: Option<M5ShellGeometryDowngradeTrigger>,
}

impl ShellGeometryAxisOutcome {
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
            ShellGeometryAxisCertificationState::Certified => {
                self.narrowing_reason.is_none() && self.downgrade_trigger.is_none()
            }
            ShellGeometryAxisCertificationState::DisclosedNarrowed => {
                let reason_ok = self
                    .narrowing_reason
                    .as_deref()
                    .is_some_and(|r| !r.trim().is_empty() && !label_is_generic(r));
                reason_ok && self.downgrade_trigger.is_some()
            }
            ShellGeometryAxisCertificationState::UndisclosedDrift => {
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
pub struct ShellGeometryClaimAutoNarrow {
    /// The axis whose degraded parity forced the narrowing.
    pub binding_axis: ShellGeometryCertificationAxis,
    /// The claim the profile would deliver at full parity.
    pub from_claim: M5ShellGeometryA11yClaim,
    /// The weakest supported claim the profile is certified down to.
    pub to_claim: M5ShellGeometryA11yClaim,
    /// The visible, non-generic disclosure label.
    pub visible_label: String,
}

/// One certified M5 desktop shell-geometry profile.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ShellGeometryProfileCertificationRow {
    /// Record kind; must equal [`SHELL_METRIC_DENSITY_CERT_ROW_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`SHELL_METRIC_DENSITY_CERT_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable row id.
    pub row_id: String,
    /// The certified profile.
    pub profile: M5ShellGeometryCertifiedProfile,
    /// The geometry claim ceiling the profile asserts.
    pub claimed_claim: M5ShellGeometryA11yClaim,
    /// The weakest supported claim the profile is certified down to. Must be no stronger than
    /// `claimed_claim`.
    pub certified_claim: M5ShellGeometryA11yClaim,
    /// The frozen geometry families this profile renders (at least one).
    #[serde(default)]
    pub consumed_families: Vec<M5ShellGeometryFamily>,
    /// One outcome per [`ShellGeometryCertificationAxis`], each axis appearing once.
    #[serde(default)]
    pub axis_outcomes: Vec<ShellGeometryAxisOutcome>,
    /// The B138 hard invariants; all must hold.
    pub guardrails: ShellGeometryCertGuardrails,
    /// The visible claim narrowing; present iff `certified_claim` is weaker than `claimed_claim`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub claim_auto_narrow: Option<ShellGeometryClaimAutoNarrow>,
    /// The one canonical shell-geometry proof bundle this profile cites. Must equal
    /// [`SHELL_METRIC_DENSITY_CERT_CANONICAL_BUNDLE_REF`].
    pub canonical_bundle_ref: String,
    /// The derived verdict. Recomputed and compared on validation.
    pub derived_status: ShellGeometryProfileClaimStatus,
    /// The copy / export parity of the certified profile state.
    pub export_parity: ShellGeometryCertExportParity,
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

impl ShellGeometryProfileCertificationRow {
    /// The outcome for a given axis, if present.
    pub fn axis(&self, axis: ShellGeometryCertificationAxis) -> Option<&ShellGeometryAxisOutcome> {
        self.axis_outcomes.iter().find(|o| o.axis == axis)
    }

    /// Whether every axis appears exactly once.
    pub fn covers_all_axes(&self) -> bool {
        let seen: BTreeSet<ShellGeometryCertificationAxis> =
            self.axis_outcomes.iter().map(|o| o.axis).collect();
        seen.len() == self.axis_outcomes.len()
            && ShellGeometryCertificationAxis::ALL
                .iter()
                .all(|a| seen.contains(a))
    }

    /// Whether every axis outcome is internally well-formed.
    pub fn axis_outcomes_well_formed(&self) -> bool {
        self.axis_outcomes
            .iter()
            .all(ShellGeometryAxisOutcome::well_formed)
    }

    /// True when the profile narrows its geometry claim below what it asserts.
    pub fn is_claim_narrowed(&self) -> bool {
        self.certified_claim.capability_rank() < self.claimed_claim.capability_rank()
    }

    /// The axes disclosed as narrowed (yellow).
    pub fn narrowed_axes(&self) -> Vec<ShellGeometryCertificationAxis> {
        self.axis_outcomes
            .iter()
            .filter(|o| o.state == ShellGeometryAxisCertificationState::DisclosedNarrowed)
            .map(|o| o.axis)
            .collect()
    }

    /// Derives the profile verdict from its axes, invariants, and claim narrowing. This is the heart of
    /// the capstone: a degraded axis must produce a visible claim narrowing, only a live first-party
    /// profile may certify a trusted geometry surface, every hard invariant must hold, CLI/export parity
    /// must always certify, and the narrowing must be consistent.
    pub fn derive_status(&self) -> ShellGeometryProfileClaimStatus {
        // Structural prerequisites: malformed rows can never certify.
        if !self.covers_all_axes()
            || !self.axis_outcomes_well_formed()
            || self.canonical_bundle_ref != SHELL_METRIC_DENSITY_CERT_CANONICAL_BUNDLE_REF
            || self.consumed_families.is_empty()
            || !self.export_parity.is_complete()
        {
            return ShellGeometryProfileClaimStatus::Red;
        }

        // Every B138 hard invariant must hold.
        if !self.guardrails.all_held() {
            return ShellGeometryProfileClaimStatus::Red;
        }

        // Certification may only narrow the claim, never strengthen it.
        if self.certified_claim.capability_rank() > self.claimed_claim.capability_rank() {
            return ShellGeometryProfileClaimStatus::Red;
        }

        // Only a live first-party profile may certify a trusted geometry surface.
        if self.certified_claim.asserts_trusted_surface()
            && !self.profile.is_live_trusted_geometry_surface()
        {
            return ShellGeometryProfileClaimStatus::Red;
        }

        // The always-on CLI/export axis must stay certified.
        match self.axis(ShellGeometryCertificationAxis::CliExport) {
            Some(o) if o.state == ShellGeometryAxisCertificationState::Certified => {}
            _ => return ShellGeometryProfileClaimStatus::Red,
        }

        // Any undisclosed drift blocks outright.
        if self
            .axis_outcomes
            .iter()
            .any(|o| o.state == ShellGeometryAxisCertificationState::UndisclosedDrift)
        {
            return ShellGeometryProfileClaimStatus::Red;
        }

        let narrowed = self.narrowed_axes();
        let claim_narrowed = self.is_claim_narrowed();

        match (&self.claim_auto_narrow, claim_narrowed) {
            // Spurious narrowing structure without a claim reduction.
            (Some(_), false) => return ShellGeometryProfileClaimStatus::Red,
            // A claim reduction with no disclosed narrowing structure.
            (None, true) => return ShellGeometryProfileClaimStatus::Red,
            (Some(narrow), true) => {
                if narrow.from_claim != self.claimed_claim
                    || narrow.to_claim != self.certified_claim
                    || !narrowed.contains(&narrow.binding_axis)
                    || narrow.binding_axis.is_always_on()
                    || narrow.visible_label.trim().is_empty()
                    || label_is_generic(&narrow.visible_label)
                {
                    return ShellGeometryProfileClaimStatus::Red;
                }
            }
            (None, false) => {}
        }

        if claim_narrowed {
            // A disclosed, consistently-bound narrowing.
            return ShellGeometryProfileClaimStatus::Yellow;
        }

        // Claim not narrowed: a degraded axis retained behind a full claim is a hidden overclaim
        // inheriting a healthier profile's truth.
        if !narrowed.is_empty() {
            return ShellGeometryProfileClaimStatus::Red;
        }

        ShellGeometryProfileClaimStatus::Green
    }

    /// Whether the stored `derived_status` matches a fresh recomputation.
    pub fn status_is_fresh(&self) -> bool {
        self.derived_status == self.derive_status()
    }

    /// Whether the row's identity and evidence fields are complete.
    pub fn is_complete(&self) -> bool {
        self.record_kind == SHELL_METRIC_DENSITY_CERT_ROW_RECORD_KIND
            && self.schema_version == SHELL_METRIC_DENSITY_CERT_SCHEMA_VERSION
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

/// Rolled-up summary of an M05-1163 certification packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ShellGeometryProfileCertificationSummary {
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

/// Constructor input for [`ShellGeometryProfileCertificationPacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ShellGeometryProfileCertificationPacketInput {
    pub packet_id: String,
    pub as_of: String,
    pub matrix_ref: String,
    pub canonical_bundle_ref: String,
    pub rows: Vec<ShellGeometryProfileCertificationRow>,
}

/// Checked-in M05-1163 certification packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ShellGeometryProfileCertificationPacket {
    pub schema_version: u32,
    pub record_kind: String,
    pub packet_id: String,
    pub as_of: String,
    pub matrix_ref: String,
    pub canonical_bundle_ref: String,
    #[serde(default)]
    pub rows: Vec<ShellGeometryProfileCertificationRow>,
    pub summary: ShellGeometryProfileCertificationSummary,
}

impl ShellGeometryProfileCertificationPacket {
    /// Builds a packet, stamping the record kind, schema version, and computed summary.
    pub fn new(input: ShellGeometryProfileCertificationPacketInput) -> Self {
        let mut packet = Self {
            schema_version: SHELL_METRIC_DENSITY_CERT_SCHEMA_VERSION,
            record_kind: SHELL_METRIC_DENSITY_CERT_RECORD_KIND.to_owned(),
            packet_id: input.packet_id,
            as_of: input.as_of,
            matrix_ref: input.matrix_ref,
            canonical_bundle_ref: input.canonical_bundle_ref,
            rows: input.rows,
            summary: ShellGeometryProfileCertificationSummary {
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
    pub fn represented_profiles(&self) -> BTreeSet<M5ShellGeometryCertifiedProfile> {
        self.rows.iter().map(|r| r.profile).collect()
    }

    /// Geometry families rendered by some certified profile in this packet.
    pub fn represented_families(&self) -> BTreeSet<M5ShellGeometryFamily> {
        self.rows
            .iter()
            .flat_map(|r| r.consumed_families.iter().copied())
            .collect()
    }

    /// Whether every certified profile appears exactly once.
    pub fn all_profiles_present(&self) -> bool {
        let profiles = self.represented_profiles();
        profiles.len() == self.rows.len()
            && M5ShellGeometryCertifiedProfile::ALL
                .iter()
                .all(|s| profiles.contains(s))
    }

    /// Whether every frozen geometry family is certified on at least one profile — proof the full matrix
    /// runs across the claimed consumers.
    pub fn all_families_covered(&self) -> bool {
        let families = self.represented_families();
        M5ShellGeometryFamily::ALL
            .iter()
            .all(|f| families.contains(f))
    }

    /// Whether a CLI/export axis is certified on every row.
    pub fn all_rows_export_parity_certified(&self) -> bool {
        self.rows.iter().all(|r| {
            r.axis(ShellGeometryCertificationAxis::CliExport)
                .is_some_and(|o| o.state == ShellGeometryAxisCertificationState::Certified)
                && r.export_parity.is_complete()
        })
    }

    /// Computes summary fields from the packet contents.
    pub fn computed_summary(&self) -> ShellGeometryProfileCertificationSummary {
        let profiles = self.represented_profiles();
        let green = self
            .rows
            .iter()
            .filter(|r| r.derived_status == ShellGeometryProfileClaimStatus::Green)
            .count();
        let yellow = self
            .rows
            .iter()
            .filter(|r| r.derived_status == ShellGeometryProfileClaimStatus::Yellow)
            .count();
        let red = self
            .rows
            .iter()
            .filter(|r| r.derived_status == ShellGeometryProfileClaimStatus::Red)
            .count();
        let all_publishable = self.rows.iter().all(|r| r.derived_status.is_publishable());
        let all_fresh = self
            .rows
            .iter()
            .all(ShellGeometryProfileCertificationRow::status_is_fresh);
        let all_profiles = self.all_profiles_present();
        let all_families = self.all_families_covered();

        ShellGeometryProfileCertificationSummary {
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
                .all(|r| r.canonical_bundle_ref == SHELL_METRIC_DENSITY_CERT_CANONICAL_BUNDLE_REF),
            all_rows_export_parity_certified: self.all_rows_export_parity_certified(),
            all_guardrails_held: self.rows.iter().all(|r| r.guardrails.all_held()),
            every_axis_covered_on_every_row: self
                .rows
                .iter()
                .all(ShellGeometryProfileCertificationRow::covers_all_axes),
            narrowed_profile_count: self.rows.iter().filter(|r| r.is_claim_narrowed()).count(),
            report_clean: all_publishable && all_fresh && all_profiles && all_families,
        }
    }

    /// Validates the packet and returns every contract violation.
    pub fn validate(&self) -> Vec<ShellGeometryCertificationViolation> {
        let mut violations = Vec::new();

        if self.schema_version != SHELL_METRIC_DENSITY_CERT_SCHEMA_VERSION {
            violations.push(ShellGeometryCertificationViolation::SchemaVersion {
                expected: SHELL_METRIC_DENSITY_CERT_SCHEMA_VERSION,
                actual: self.schema_version,
            });
        }
        if self.record_kind != SHELL_METRIC_DENSITY_CERT_RECORD_KIND {
            violations.push(ShellGeometryCertificationViolation::RecordKind {
                expected: SHELL_METRIC_DENSITY_CERT_RECORD_KIND.to_owned(),
                actual: self.record_kind.clone(),
            });
        }
        if self.packet_id.trim().is_empty()
            || self.as_of.trim().is_empty()
            || self.matrix_ref.trim().is_empty()
        {
            violations.push(ShellGeometryCertificationViolation::MissingIdentity);
        }
        if self.canonical_bundle_ref != SHELL_METRIC_DENSITY_CERT_CANONICAL_BUNDLE_REF {
            violations.push(ShellGeometryCertificationViolation::WrongCanonicalBundle);
        }

        let mut row_ids = BTreeSet::new();
        for row in &self.rows {
            if !row_ids.insert(row.row_id.clone()) {
                violations.push(ShellGeometryCertificationViolation::DuplicateId {
                    id: row.row_id.clone(),
                });
            }

            if !row.is_complete() {
                violations.push(ShellGeometryCertificationViolation::IncompleteRow {
                    id: row.row_id.clone(),
                });
            }

            if !row.covers_all_axes() {
                violations.push(
                    ShellGeometryCertificationViolation::AxisCoverageIncomplete {
                        id: row.row_id.clone(),
                    },
                );
            }

            if !row.axis_outcomes_well_formed() {
                violations.push(ShellGeometryCertificationViolation::MalformedAxisOutcome {
                    id: row.row_id.clone(),
                });
            }

            if row.canonical_bundle_ref != SHELL_METRIC_DENSITY_CERT_CANONICAL_BUNDLE_REF {
                violations.push(
                    ShellGeometryCertificationViolation::RowMissingCanonicalBundle {
                        id: row.row_id.clone(),
                    },
                );
            }

            // Every B138 hard invariant must hold.
            if !row.guardrails.all_held() {
                violations.push(ShellGeometryCertificationViolation::GuardrailViolated {
                    id: row.row_id.clone(),
                });
            }

            // Only a live first-party profile may certify a trusted geometry surface.
            if row.certified_claim.asserts_trusted_surface()
                && !row.profile.is_live_trusted_geometry_surface()
            {
                violations.push(
                    ShellGeometryCertificationViolation::NonLiveProfileClaimsTrustedSurface {
                        id: row.row_id.clone(),
                    },
                );
            }

            // CLI/export parity is always-on.
            if !row.export_parity.is_complete()
                || row
                    .axis(ShellGeometryCertificationAxis::CliExport)
                    .is_none_or_state_not_certified()
            {
                violations.push(
                    ShellGeometryCertificationViolation::ExportParityNotCertified {
                        id: row.row_id.clone(),
                    },
                );
            }

            // Certification may never strengthen a claim.
            if row.certified_claim.capability_rank() > row.claimed_claim.capability_rank() {
                violations.push(
                    ShellGeometryCertificationViolation::CertifiedClaimExceedsClaim {
                        id: row.row_id.clone(),
                    },
                );
            }

            // The stored verdict must match a fresh recomputation.
            if !row.status_is_fresh() {
                violations.push(ShellGeometryCertificationViolation::StatusDerivationStale {
                    id: row.row_id.clone(),
                });
            }

            // A blocked (red) profile must not ship in a clean packet.
            if row.derived_status == ShellGeometryProfileClaimStatus::Red {
                violations.push(ShellGeometryCertificationViolation::ProfileBlocked {
                    id: row.row_id.clone(),
                });
            }
        }

        // Every claimed profile must be certified exactly once.
        if !self.all_profiles_present() {
            violations.push(ShellGeometryCertificationViolation::ProfileCoverageIncomplete);
        }

        // Every frozen geometry family must be certified on some profile.
        if !self.all_families_covered() {
            violations.push(ShellGeometryCertificationViolation::FamilyCoverageIncomplete);
        }

        if self.summary != self.computed_summary() {
            violations.push(ShellGeometryCertificationViolation::SummaryMismatch);
        }

        if json_contains_forbidden_material(
            &serde_json::to_value(self).expect("certification packet serializes"),
        ) {
            violations.push(ShellGeometryCertificationViolation::RawGeometryMaterialInExport);
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
        out.push_str("# M5 Shell-Metric-Density Surface Certification\n\n");
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- As of: `{}`\n", self.as_of));
        out.push_str(&format!(
            "- Canonical bundle: `{}`\n",
            self.canonical_bundle_ref
        ));
        out.push_str(&format!(
            "- Profiles: {} / {} certified ({} green, {} yellow, {} red)\n",
            self.summary.profile_count,
            M5ShellGeometryCertifiedProfile::ALL.len(),
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
pub fn current_m5_shell_metric_density_surface_certification_export(
) -> Result<ShellGeometryProfileCertificationPacket, ShellGeometryCertificationArtifactError> {
    let packet: ShellGeometryProfileCertificationPacket = serde_json::from_str(include_str!(
        concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../artifacts/release/m5-shell-metric-density-surface-certification/support_export.json"
        )
    ))
    .map_err(ShellGeometryCertificationArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(ShellGeometryCertificationArtifactError::Validation(
            violations,
        ))
    }
}

/// Errors emitted when reading the checked-in certification export.
#[derive(Debug)]
pub enum ShellGeometryCertificationArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<ShellGeometryCertificationViolation>),
}

impl fmt::Display for ShellGeometryCertificationArtifactError {
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

impl Error for ShellGeometryCertificationArtifactError {}

/// Validation failure for M05-1163 certification packets.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ShellGeometryCertificationViolation {
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
    RawGeometryMaterialInExport,
}

impl fmt::Display for ShellGeometryCertificationViolation {
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
                    "packet does not cite the canonical shell-geometry proof bundle"
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
                    "row {id} does not cite the one canonical shell-geometry proof bundle"
                )
            }
            Self::GuardrailViolated { id } => {
                write!(
                    f,
                    "row {id} breaks a B138 hard invariant: density or collapse changing command meaning, \
focus order, or trust visibility; an extension or embedded surface setting a private fracturing width; a \
hit target shrinking below the supported minimum; a primary workflow hidden behind an overlay-only \
fallback; or a zone starving the main workspace"
                )
            }
            Self::NonLiveProfileClaimsTrustedSurface { id } => {
                write!(
                    f,
                    "row {id} certifies a trusted geometry surface on a non-live first-party profile"
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
a hard invariant broke, CLI/export parity dropped, a non-live profile claimed a trusted geometry \
surface, or the narrowing is inconsistent"
                )
            }
            Self::ProfileCoverageIncomplete => {
                write!(
                    f,
                    "not every claimed M5 shell-geometry profile is certified exactly once"
                )
            }
            Self::FamilyCoverageIncomplete => {
                write!(
                    f,
                    "not every frozen shell-geometry family is certified on some profile"
                )
            }
            Self::SummaryMismatch => write!(f, "computed summary does not match stored summary"),
            Self::RawGeometryMaterialInExport => {
                write!(
                    f,
                    "export contains raw pixel geometry, absolute window coordinates, a z-index integer, a credential, or secret material"
                )
            }
        }
    }
}

impl Error for ShellGeometryCertificationViolation {}

/// Small extension so the export-parity check reads cleanly.
trait AxisOutcomeOptionExt {
    fn is_none_or_state_not_certified(&self) -> bool;
}

impl AxisOutcomeOptionExt for Option<&ShellGeometryAxisOutcome> {
    fn is_none_or_state_not_certified(&self) -> bool {
        match self {
            None => true,
            Some(o) => o.state != ShellGeometryAxisCertificationState::Certified,
        }
    }
}

/// Whether a label is a generic non-answer rather than a precise disclosure. Includes the shell-geometry
/// generics the spec forbids collapsing distinct shell-metric, minimum-size, density, responsive, and
/// collapse truth into (whole-label matches so a full sentence naming a concrete zone, metric, or registry
/// reference is not flagged).
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
            | "geometry"
            | "metric"
            | "density"
            | "responsive"
            | "collapse"
            | "width"
            | "zone"
            | "sidebar"
            | "inspector"
            | "panel"
            | "workspace"
            | "compact"
            | "standard"
            | "expanded"
            | "minimum"
            | "hit target"
            | "size metric"
            | "density mode"
            | "responsive class"
            | "registry reference"
            | "shell metric"
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

/// Builds the canonical, checked-in M05-1163 certification packet. Certifies all five claimed M5 desktop
/// shell-geometry profiles: two deliver their claim (green) and three auto-narrow a not-current truth axis
/// to a weaker geometry ceiling (yellow). No profile hides drift or breaks a hard invariant (red).
pub fn seeded_m5_shell_metric_density_surface_certification_packet(
) -> ShellGeometryProfileCertificationPacket {
    ShellGeometryProfileCertificationPacket::new(ShellGeometryProfileCertificationPacketInput {
        packet_id: SHELL_METRIC_DENSITY_CERT_PACKET_ID.to_owned(),
        as_of: "2026-07-13T00:00:00Z".to_owned(),
        matrix_ref: SHELL_METRIC_DENSITY_CERT_MATRIX_REF.to_owned(),
        canonical_bundle_ref: SHELL_METRIC_DENSITY_CERT_CANONICAL_BUNDLE_REF.to_owned(),
        rows: seeded_rows(),
    })
}

fn seed_evidence(id: &str) -> Vec<String> {
    vec![
        format!("evidence:shell-metric-density-surface-certification:{id}"),
        SHELL_METRIC_DENSITY_CERT_A11Y_BUNDLE_REF.to_owned(),
    ]
}

fn seed_export_parity(fields: &[&str]) -> ShellGeometryCertExportParity {
    ShellGeometryCertExportParity {
        formats: vec!["text".to_owned(), "json".to_owned(), "markdown".to_owned()],
        export_fields: fields.iter().map(|f| (*f).to_owned()).collect(),
        raw_payload_only_prohibited: true,
    }
}

fn seed_certified_note(axis: ShellGeometryCertificationAxis) -> &'static str {
    match axis {
        ShellGeometryCertificationAxis::Visual => {
            "canonical zone metric, minimum size, density mode, responsive class, and registry reference shown on-surface without a private width or an off-screen zone alone"
        }
        ShellGeometryCertificationAxis::Keyboard => {
            "the same geometry role, registry reference, and bound controls are keyboard-reachable, never hover-only"
        }
        ShellGeometryCertificationAxis::ScreenReader => {
            "the same shell-geometry truth is announced non-visually, never a private-width / off-screen-zone / unlabeled-control-only cue"
        }
        ShellGeometryCertificationAxis::HighZoomReflow => {
            "the same truth reflows legibly at 200-400% zoom without clipping the zone metric, registry reference, density mode, or minimum size"
        }
        ShellGeometryCertificationAxis::HighContrast => {
            "the same truth stays legible and operable in high-contrast mode without dropping the zone metric, registry reference, or minimum hit target"
        }
        ShellGeometryCertificationAxis::SnappedWidth => {
            "the same truth preserves task identity and recovery-critical state under snapped and narrow widths, never collapsing a primary workflow into an overlay-only fallback"
        }
        ShellGeometryCertificationAxis::CliExport => {
            "profile state exports as text / JSON / Markdown for support replay"
        }
        ShellGeometryCertificationAxis::DegradedState => {
            "a stale shell metric, an unconfirmed density change, an unconfirmed responsive collapse, or a partially-disclosed collapse boundary honestly downgrades the TrustedGeometrySurface/ReviewableGeometrySurface claim rather than reading as a fresh authoritative geometry surface"
        }
        ShellGeometryCertificationAxis::ShellGeometryComponentTruth => {
            "canonical zone metric, minimum size, density mode, responsive class, and registry reference stay explicit and never let density or collapse change command meaning, focus order, or trust visibility, let an extension or embedded surface set a private fracturing width, shrink a hit target below the supported minimum, hide a primary workflow behind an overlay-only fallback, or let a zone starve the main workspace"
        }
    }
}

fn seed_certified(axis: ShellGeometryCertificationAxis) -> ShellGeometryAxisOutcome {
    ShellGeometryAxisOutcome {
        axis,
        state: ShellGeometryAxisCertificationState::Certified,
        parity_note: seed_certified_note(axis).to_owned(),
        narrowing_reason: None,
        downgrade_trigger: None,
    }
}

fn seed_narrowed(
    axis: ShellGeometryCertificationAxis,
    note: &str,
    reason: &str,
    trigger: M5ShellGeometryDowngradeTrigger,
) -> ShellGeometryAxisOutcome {
    ShellGeometryAxisOutcome {
        axis,
        state: ShellGeometryAxisCertificationState::DisclosedNarrowed,
        parity_note: note.to_owned(),
        narrowing_reason: Some(reason.to_owned()),
        downgrade_trigger: Some(trigger),
    }
}

fn seed_all_certified() -> Vec<ShellGeometryAxisOutcome> {
    ShellGeometryCertificationAxis::ALL
        .iter()
        .copied()
        .map(seed_certified)
        .collect()
}

fn seed_certified_except(
    axis: ShellGeometryCertificationAxis,
    outcome: ShellGeometryAxisOutcome,
) -> Vec<ShellGeometryAxisOutcome> {
    ShellGeometryCertificationAxis::ALL
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
    profile: M5ShellGeometryCertifiedProfile,
    claimed_claim: M5ShellGeometryA11yClaim,
    certified_claim: M5ShellGeometryA11yClaim,
    consumed_families: &[M5ShellGeometryFamily],
    axis_outcomes: Vec<ShellGeometryAxisOutcome>,
    claim_auto_narrow: Option<ShellGeometryClaimAutoNarrow>,
    export_fields: &[&str],
    compatibility_notes: &[&str],
) -> ShellGeometryProfileCertificationRow {
    let mut row = ShellGeometryProfileCertificationRow {
        record_kind: SHELL_METRIC_DENSITY_CERT_ROW_RECORD_KIND.to_owned(),
        schema_version: SHELL_METRIC_DENSITY_CERT_SCHEMA_VERSION,
        row_id: row_id.to_owned(),
        profile,
        claimed_claim,
        certified_claim,
        consumed_families: consumed_families.to_vec(),
        axis_outcomes,
        guardrails: ShellGeometryCertGuardrails::CLEAN,
        claim_auto_narrow,
        canonical_bundle_ref: SHELL_METRIC_DENSITY_CERT_CANONICAL_BUNDLE_REF.to_owned(),
        derived_status: ShellGeometryProfileClaimStatus::Green,
        export_parity: seed_export_parity(export_fields),
        compatibility_notes: compatibility_notes
            .iter()
            .map(|n| (*n).to_owned())
            .collect(),
        source_refs: vec![
            SHELL_METRIC_DENSITY_CERT_MATRIX_REF.to_owned(),
            SHELL_METRIC_DENSITY_CERT_SCHEMA_REF.to_owned(),
        ],
        observed_at: "2026-07-13T00:00:00Z".to_owned(),
        evidence_refs: seed_evidence(row_id),
    };
    row.derived_status = row.derive_status();
    row
}

fn seed_narrow(
    binding_axis: ShellGeometryCertificationAxis,
    from_claim: M5ShellGeometryA11yClaim,
    to_claim: M5ShellGeometryA11yClaim,
    label: &str,
) -> ShellGeometryClaimAutoNarrow {
    ShellGeometryClaimAutoNarrow {
        binding_axis,
        from_claim,
        to_claim,
        visible_label: label.to_owned(),
    }
}

fn seeded_rows() -> Vec<ShellGeometryProfileCertificationRow> {
    use M5ShellGeometryA11yClaim::*;
    use M5ShellGeometryCertifiedProfile as P;
    use M5ShellGeometryDowngradeTrigger as Trig;
    use M5ShellGeometryFamily::*;
    use ShellGeometryCertificationAxis as Ax;

    vec![
        // --- Green: full parity, claim delivered ---------------------------
        seed_row(
            "cert:live-trusted-geometry-surface",
            P::LiveTrustedGeometrySurface,
            TrustedGeometrySurface,
            TrustedGeometrySurface,
            &[MinimumSize],
            seed_all_certified(),
            None,
            &["profile", "claimed_claim", "certified_claim", "status", "size_metric"],
            &[
                "local profile: the shell keeps every hit target at or above its supported minimum (tab width, resize handle, icon-only target) rather than shrinking a control below the supported minimum",
                "the trusted geometry surface keeps the main workspace dominant across compact / standard / expanded window classes and every density mode",
                "keyboard / screen-reader / high-zoom / high-contrast / snapped-width reach preserved for the rendered shell surface, including mixed-DPI monitor remap",
                "shell-geometry-component-truth: a live first-party geometry surface is the only profile that certifies a trusted geometry surface",
            ],
        ),
        seed_row(
            "cert:reviewable-geometry-structure",
            P::ReviewableGeometryStructure,
            ReviewableGeometrySurface,
            ReviewableGeometrySurface,
            &[ShellMetric],
            seed_all_certified(),
            None,
            &["profile", "claimed_claim", "certified_claim", "status", "registry_reference"],
            &[
                "managed profile: the shell-zone metric stays bound to the single shell-metric registry with its default / minimum / recommended size honored rather than a private width copied by hand across packages",
                "the reviewable geometry structure keeps its title / context bar, rail, sidebar, main workspace, inspector, panel, and status metrics inspectable rather than a private-width or off-screen-zone cue",
                "text / JSON / Markdown reconstruction certified so support can replay the reviewable geometry structure",
                "shell-geometry-component-truth: a reviewable geometry structure never certifies a live trusted, authoritative geometry claim",
            ],
        ),
        // --- Yellow: an axis is not current; the claim narrows visibly ------
        seed_row(
            "cert:unverified-density-mode-surface",
            P::UnverifiedDensityModeSurface,
            ReviewableGeometrySurface,
            DensityModeUnverifiedProjection,
            &[DensityMode],
            seed_certified_except(
                Ax::DegradedState,
                seed_narrowed(
                    Ax::DegradedState,
                    "the density mode's presentation-only safety cannot be confirmed so a safe density change cannot be certified",
                    "The density mode's presentation-only safety cannot be confirmed, so the ReviewableGeometrySurface claim narrows to a density-mode-unverified projection and the shell keeps the last-known information architecture explicit rather than presenting a density change as safe when it may rearrange focus order or trust visibility",
                    Trig::DensityChangedCommandOrFocusOrTrust,
                ),
            ),
            Some(seed_narrow(
                Ax::DegradedState,
                ReviewableGeometrySurface,
                DensityModeUnverifiedProjection,
                "Density mode unverified: the presentation-only safety cannot be confirmed so the last-known information architecture, focus order, and trust visibility stay explicit and the density change never rearranges command meaning",
            )),
            &["profile", "claimed_claim", "certified_claim", "status", "binding_axis"],
            &[
                "remote profile: the shell keeps its last-known information architecture, focus order, and trust visibility explicit and marks the density mode as unverified rather than presenting a density change as presentation-only when it may rearrange information architecture",
                "the density surface keeps its Comfortable / Standard / Compact row, control, and spacing scale legible while the presentation-only safety is disclosed as unverified",
                "degraded-state: ReviewableGeometrySurface narrows to a density-mode-unverified projection (auto-narrowed)",
                "shell-geometry-component-truth: a density change never changes command meaning, focus order, or trust visibility",
            ],
        ),
        seed_row(
            "cert:unverified-responsive-geometry-surface",
            P::UnverifiedResponsiveGeometrySurface,
            ReviewableGeometrySurface,
            ResponsiveGeometryUnverifiedProjection,
            &[ResponsiveGeometry],
            seed_certified_except(
                Ax::SnappedWidth,
                seed_narrowed(
                    Ax::SnappedWidth,
                    "the responsive window class's recovery-state preservation cannot be confirmed under snapped or narrow widths so a recovery-safe collapse cannot be certified",
                    "The responsive window class's recovery-state preservation cannot be confirmed under snapped or narrow widths, so the ReviewableGeometrySurface claim narrows to a responsive-geometry-unverified projection and the shell keeps task identity and recovery-critical state inspectable rather than presenting a collapse as recovery-safe when it may drop recovery-critical state",
                    Trig::ResponsiveCollapseDroppedRecoveryState,
                ),
            ),
            Some(seed_narrow(
                Ax::SnappedWidth,
                ReviewableGeometrySurface,
                ResponsiveGeometryUnverifiedProjection,
                "Responsive geometry unverified: recovery-state preservation under snapped and narrow widths cannot be confirmed so task identity and recovery-critical state stay inspectable and no primary workflow is hidden behind an overlay-only fallback",
            )),
            &["profile", "claimed_claim", "certified_claim", "status", "binding_axis"],
            &[
                "snapped-width profile: the shell keeps its task identity and recovery-critical state inspectable and marks the responsive collapse as unverified rather than presenting a snapped-width collapse as recovery-safe",
                "the responsive surface keeps its Compact 1024-1279 / Standard 1280-1599 / Expanded 1600+ window class legible while the recovery-state preservation is disclosed as unverified",
                "snapped-width: ReviewableGeometrySurface narrows to a responsive-geometry-unverified projection (auto-narrowed)",
                "shell-geometry-component-truth: responsive compact / standard / expanded collapse preserves task identity and recovery-critical state rather than dropping it under a snapped width",
            ],
        ),
        seed_row(
            "cert:disclosed-collapse-priority-surface",
            P::DisclosedCollapsePrioritySurface,
            ReviewableGeometrySurface,
            CollapsePriorityDisclosedProjection,
            &[CollapsePriority],
            seed_certified_except(
                Ax::ShellGeometryComponentTruth,
                seed_narrowed(
                    Ax::ShellGeometryComponentTruth,
                    "the collapse boundary can only be partially disclosed so a fully-proven no-fracture collapse priority cannot be certified",
                    "The collapse boundary can only be partially disclosed, so the ReviewableGeometrySurface claim narrows to a collapse-priority-disclosed projection and the shell discloses the partial collapse boundary inspectably rather than letting an extension or embedded surface set a private width that fractures layout or starves the main workspace",
                    Trig::ProofStale,
                ),
            ),
            Some(seed_narrow(
                Ax::ShellGeometryComponentTruth,
                ReviewableGeometrySurface,
                CollapsePriorityDisclosedProjection,
                "Collapse priority disclosed partial: the collapse boundary is only partially proven so it is disclosed inspectably and no private width fractures layout or starves the main workspace",
            )),
            &["profile", "claimed_claim", "certified_claim", "status", "binding_axis"],
            &[
                "embedded / extension profile: the shell discloses its partial collapse boundary and keeps the declared adaptive-collapse priority order rather than letting an extension or embedded surface invent a private width that fractures the shell",
                "the collapse surface keeps its ranked collapse targets and protected main-workspace dominance legible while the collapse boundary is disclosed as partial",
                "shell-geometry-component-truth: ReviewableGeometrySurface narrows to a collapse-priority-disclosed projection (auto-narrowed)",
                "shell-geometry-component-truth: no zone starves the main workspace and no primary workflow is hidden behind an overlay-only fallback",
            ],
        ),
    ]
}
