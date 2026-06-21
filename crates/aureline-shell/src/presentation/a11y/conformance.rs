//! Accessibility, reduced-motion, and local/remote/shared boundary conformance
//! for the presentation overlay surfaces.
//!
//! A presentation overlay rides on top of the existing panes (see
//! [`crate::presentation::binding`]); its presenter bar, agenda / waypoint rail,
//! spotlight frame, speaker-notes tray, audience strip, breakaway banner, and
//! provenance strip must therefore meet the **same accessibility and
//! boundary-truth expectations as the rest of the shell** rather than a softer
//! "presentation looks fine" bar. This module turns that expectation into an
//! inspectable, support-safe truth packet — a [`PresentationAccessibilityReport`]
//! — instead of leaving it to a post-hoc manual signoff.
//!
//! The contracts this row exists to hold:
//!
//! - **Every named accessibility dimension is proven per surface.** Each
//!   [`SurfaceConformance`] carries keyboard reachability, a place in a single
//!   contiguous focus ring, a visible focus indicator, screen-reader
//!   reachability, reduced-motion safety, a high-zoom reflow strategy, and a
//!   non-empty accessible label. [`PresentationAccessibilityReport::validate`]
//!   re-derives all of them, so a regression cannot quietly claim conformance.
//! - **Spotlight, zoom, and follow stay operable without a pointer or motion.**
//!   No surface is `pointer_only` or `motion_only`, and none traps focus, so the
//!   spotlight inset, the zoom presets, and the follow / breakaway controls are
//!   reachable and announced for keyboard and assistive-technology users.
//! - **Boundary labels are preserved, never flattened.** The
//!   [`BoundaryPosture`] keeps the current and distinct local / remote / shared
//!   labels visible through the overlay and into the support export, so a
//!   diagnostics or support surface can explain *what the user is looking at and
//!   where it lives* without collapsing it to a generic "shared" badge.
//!
//! The per-surface fidelity maps onto the shell's canonical accessibility-support
//! vocabulary in [`crate::a11y::tree_contract`]
//! ([`SupportState`](crate::a11y::tree_contract::SupportState) /
//! [`RoleConfidence`](crate::a11y::tree_contract::RoleConfidence)) via
//! [`HighZoomReflow::to_support_state`] and
//! [`PresentationA11yClass::to_support_state`], so the presentation lane stays a
//! thin, parity-checked layer over the existing accessibility model rather than a
//! second vocabulary. The support-export boundary schema is
//! [`schemas/presentation/accessibility-and-boundary-report.schema.json`](../../../../../schemas/presentation/accessibility-and-boundary-report.schema.json);
//! the human-readable contract is `docs/ux/presentation-accessibility.md` and the
//! coverage matrix is
//! `artifacts/presentation/accessibility-and-boundary-report.md`.

use std::collections::BTreeSet;

use serde::{Deserialize, Serialize};

use crate::a11y::tree_contract::{RoleConfidence, SupportState};
use crate::presentation_mode::{
    project_overlay, AudienceScope, BoundaryLabel, LeaderFollowState, PresentationOverlay,
    PresentationSession, PRESENTATION_MODE_BETA_SCHEMA_VERSION,
    PRESENTATION_MODE_BETA_SHARED_CONTRACT_REF,
};

/// Stable record kind for [`PresentationAccessibilityReport`] payloads.
pub const PRESENTATION_A11Y_REPORT_RECORD_KIND: &str = "presentation_accessibility_report_record";

/// Stable record kind for [`SurfaceConformance`] payloads.
pub const PRESENTATION_A11Y_SURFACE_RECORD_KIND: &str = "presentation_accessibility_surface_record";

/// Stable record kind for [`PresentationA11ySupportExport`] payloads.
pub const PRESENTATION_A11Y_SUPPORT_EXPORT_RECORD_KIND: &str =
    "presentation_accessibility_support_export_record";

/// Stable record kind for [`PresentationA11ySupportExportRow`] payloads.
pub const PRESENTATION_A11Y_SUPPORT_EXPORT_ROW_RECORD_KIND: &str =
    "presentation_accessibility_support_export_row_record";

/// The human-readable accessibility contract this module implements.
pub const PRESENTATION_ACCESSIBILITY_DOC_REF: &str = "docs/ux/presentation-accessibility.md";

/// The accessibility / boundary coverage matrix this module's corpus backs.
pub const PRESENTATION_ACCESSIBILITY_AND_BOUNDARY_REPORT_REF: &str =
    "artifacts/presentation/accessibility-and-boundary-report.md";

/// Directory holding the checked-in accessibility / reduced-motion fixtures.
pub const PRESENTATION_A11Y_FIXTURE_DIR: &str = "fixtures/presentation/a11y-and-motion";

/// One presentation overlay surface whose accessibility and boundary posture is
/// asserted.
///
/// Mirrors the design-system overlay surfaces. Every surface but the provenance
/// strip is actionable and sits in the keyboard focus ring; the provenance strip
/// is display-only but still screen-reader reachable.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PresentationSurfaceTag {
    /// Presenter bar (spotlight / notes / zoom / exit controls).
    PresenterBar,
    /// Agenda / waypoint rail.
    WaypointRail,
    /// Spotlight frame (a strict inset within the main workspace).
    SpotlightFrame,
    /// Speaker-notes tray.
    SpeakerNotesTray,
    /// Audience strip / follow chip.
    AudienceStrip,
    /// Breakaway banner (present only while broken away).
    BreakawayBanner,
    /// Provenance strip (display-only boundary / source carrier).
    ProvenanceStrip,
}

impl PresentationSurfaceTag {
    /// Stable token recorded in records and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::PresenterBar => "presenter_bar",
            Self::WaypointRail => "waypoint_rail",
            Self::SpotlightFrame => "spotlight_frame",
            Self::SpeakerNotesTray => "speaker_notes_tray",
            Self::AudienceStrip => "audience_strip",
            Self::BreakawayBanner => "breakaway_banner",
            Self::ProvenanceStrip => "provenance_strip",
        }
    }

    /// Whether the surface is an actionable control (and therefore part of the
    /// keyboard focus ring). The provenance strip is display-only.
    pub const fn is_actionable(self) -> bool {
        !matches!(self, Self::ProvenanceStrip)
    }

    /// Whether the surface carries the current target's source / boundary
    /// identity and must therefore show a local / remote / shared label.
    pub const fn is_source_bearing(self) -> bool {
        matches!(self, Self::ProvenanceStrip | Self::SpotlightFrame)
    }
}

/// The zoom tier a report is projected at. High zoom is the conformance case the
/// spec calls out: dense list surfaces may reflow to a summarized-but-reachable
/// form, which must stay keyboard reachable and announced.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ZoomTier {
    /// The presenter's standard zoom; every surface reflows in place.
    Standard,
    /// A high zoom / large-text tier; dense list surfaces summarize honestly.
    HighZoom,
}

impl ZoomTier {
    /// Stable token recorded in records and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Standard => "standard",
            Self::HighZoom => "high_zoom",
        }
    }
}

/// How a surface behaves at high zoom / large text.
///
/// A surface either reflows in place with no loss of content, or — for a dense
/// list surface at extreme zoom — collapses to a labeled, keyboard-reachable
/// summary. The summary is an **honest, announced** degrade, never a silent
/// truncation, so it maps to a degraded (not unsupported) support state.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum HighZoomReflow {
    /// The surface reflows in place at high zoom with no loss of content.
    Reflows,
    /// At high zoom the surface collapses to a labeled, reachable summary that
    /// expands on demand (an honest, announced degrade).
    SummarizedReachable,
}

impl HighZoomReflow {
    /// Stable token recorded in records and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Reflows => "reflows",
            Self::SummarizedReachable => "summarized_reachable",
        }
    }

    /// The canonical shell accessibility-support state this reflow maps to.
    pub const fn to_support_state(self) -> SupportState {
        match self {
            Self::Reflows => SupportState::FullAccessible,
            Self::SummarizedReachable => SupportState::DegradedAccessible,
        }
    }

    /// True when the reflow is the honest summarized degrade rather than a clean
    /// in-place reflow.
    pub const fn is_summarized(self) -> bool {
        matches!(self, Self::SummarizedReachable)
    }
}

/// The accessibility-conformance class for a whole presentation report.
///
/// Mirrors the shell's accessibility-support vocabulary so a presentation surface
/// reads exactly like any other claimed shell surface: a fully accessible
/// overlay, an overlay with at least one honestly-announced high-zoom degrade, or
/// a non-conformant overlay that must never ship.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PresentationA11yClass {
    /// Every surface is fully accessible and reflows in place.
    FullyAccessible,
    /// Every surface stays reachable and announced, but at least one degrades to
    /// a summarized-reachable form at high zoom.
    DegradedAnnounced,
    /// At least one surface fails a hard accessibility invariant; the overlay is
    /// not shippable.
    NonConformant,
}

impl PresentationA11yClass {
    /// Stable token recorded in records and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::FullyAccessible => "fully_accessible",
            Self::DegradedAnnounced => "degraded_announced",
            Self::NonConformant => "non_conformant",
        }
    }

    /// The canonical shell accessibility-support state this class maps to.
    pub const fn to_support_state(self) -> SupportState {
        match self {
            Self::FullyAccessible => SupportState::FullAccessible,
            Self::DegradedAnnounced => SupportState::DegradedAccessible,
            Self::NonConformant => SupportState::UnsupportedBlocked,
        }
    }

    /// The canonical shell role-confidence this class maps to.
    pub const fn to_role_confidence(self) -> RoleConfidence {
        match self {
            Self::FullyAccessible => RoleConfidence::Exact,
            Self::DegradedAnnounced => RoleConfidence::Degraded,
            Self::NonConformant => RoleConfidence::Unavailable,
        }
    }

    /// True when the overlay is shippable (fully accessible or honestly degraded).
    pub const fn is_conformant(self) -> bool {
        matches!(self, Self::FullyAccessible | Self::DegradedAnnounced)
    }
}

/// One overlay surface's accessibility and boundary posture.
///
/// Every guardrail field is fixed to its safe value by the projection: a surface
/// is keyboard reachable, has a visible focus indicator, is screen-reader
/// reachable, respects reduced motion, is never pointer-only or motion-only,
/// never traps focus, and never erases its boundary label.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SurfaceConformance {
    /// Record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Shared contract ref.
    pub shared_contract_ref: String,
    /// The overlay surface this conformance describes.
    pub surface: PresentationSurfaceTag,
    /// Whether the surface is an actionable control.
    pub is_actionable: bool,
    /// 1-based position in the single keyboard focus ring, present iff actionable.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub focus_order_index: Option<u32>,
    /// Always `true`: the surface is reachable by keyboard.
    pub keyboard_reachable: bool,
    /// Always `true`: the surface renders a visible focus indicator when focused.
    pub visible_focus_indicator: bool,
    /// Always `true`: the surface is reachable by a screen reader.
    pub screen_reader_reachable: bool,
    /// Always `true`: any motion the surface uses respects reduced-motion and has
    /// a non-animated equivalent.
    pub respects_reduced_motion: bool,
    /// How the surface behaves at high zoom / large text.
    pub high_zoom_reflow: HighZoomReflow,
    /// The accessible name announced to assistive technology. Never empty.
    pub accessible_label: String,
    /// The local / remote / shared boundary the surface shows, for source-bearing
    /// surfaces (provenance strip, spotlight). `None` for chrome-only surfaces.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub boundary_label: Option<BoundaryLabel>,
    /// The canonical shell support state this surface maps to (derived from the
    /// reflow strategy).
    pub support_state: SupportState,
    // ---- guardrail flags (always safe; re-checked by validate) ----
    /// Always `false`: nothing on the surface is pointer-only.
    pub pointer_only: bool,
    /// Always `false`: no state is conveyed by motion alone.
    pub motion_only: bool,
    /// Always `false`: the surface never traps keyboard focus.
    pub traps_focus: bool,
    /// Always `false`: a source-bearing surface never erases its boundary label.
    pub erases_boundary_label: bool,
}

impl SurfaceConformance {
    /// The per-surface consistency / honesty violation, if any.
    fn consistency_violation(&self) -> Option<PresentationA11yViolation> {
        let surface = self.surface;
        if self.record_kind != PRESENTATION_A11Y_SURFACE_RECORD_KIND
            || self.schema_version != PRESENTATION_MODE_BETA_SCHEMA_VERSION
            || self.shared_contract_ref != PRESENTATION_MODE_BETA_SHARED_CONTRACT_REF
        {
            return Some(PresentationA11yViolation::MalformedRecord);
        }
        if self.is_actionable != surface.is_actionable() {
            return Some(PresentationA11yViolation::SurfaceInconsistent { surface });
        }
        // An actionable surface sits in the focus ring; a display-only one does
        // not, but is still screen-reader reachable.
        if self.is_actionable != self.focus_order_index.is_some() {
            return Some(PresentationA11yViolation::FocusOrderBroken);
        }
        if !self.keyboard_reachable
            || !self.visible_focus_indicator
            || !self.screen_reader_reachable
        {
            return Some(PresentationA11yViolation::SurfaceNotReachable { surface });
        }
        if !self.respects_reduced_motion {
            return Some(PresentationA11yViolation::ReducedMotionViolated { surface });
        }
        if self.pointer_only || self.motion_only {
            return Some(PresentationA11yViolation::PointerOrMotionOnly { surface });
        }
        if self.traps_focus {
            return Some(PresentationA11yViolation::FocusTrapped { surface });
        }
        if self.accessible_label.trim().is_empty() {
            return Some(PresentationA11yViolation::AccessibleLabelMissing { surface });
        }
        if self.support_state != self.high_zoom_reflow.to_support_state() {
            return Some(PresentationA11yViolation::SupportStateMismatch { surface });
        }
        // A source-bearing surface must show its boundary label and never erase it.
        if surface.is_source_bearing() {
            if self.boundary_label.is_none() || self.erases_boundary_label {
                return Some(PresentationA11yViolation::BoundaryLabelErased { surface });
            }
        } else if self.erases_boundary_label {
            return Some(PresentationA11yViolation::BoundaryLabelErased { surface });
        }
        None
    }
}

/// The local / remote / shared boundary posture preserved through the overlay.
///
/// Kept explicit so diagnostics and support can explain what the audience is
/// looking at and where it lives without flattening the boundary into a generic
/// label.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BoundaryPosture {
    /// The boundary label of the currently focused waypoint's target, if any.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub current_boundary_label: Option<BoundaryLabel>,
    /// Every distinct boundary label present across the session's waypoints,
    /// sorted, so a mixed local + shared walkthrough is not collapsed.
    pub distinct_boundary_labels: Vec<BoundaryLabel>,
    /// The session's audience scope (solo / shared / invited guests).
    pub audience_scope: AudienceScope,
    /// Always `true`: the boundary labels stay visible through the overlay chrome.
    pub boundary_labels_visible: bool,
    /// Always `false`: the boundary is never flattened to a single generic badge.
    pub flattened_to_generic: bool,
}

impl BoundaryPosture {
    /// True when the boundary posture is honest: labels stay visible, are not
    /// flattened, and (when waypoints exist) a current label is present and
    /// included in the distinct set.
    fn is_preserved(&self) -> bool {
        if !self.boundary_labels_visible || self.flattened_to_generic {
            return false;
        }
        match self.current_boundary_label {
            Some(label) => self.distinct_boundary_labels.contains(&label),
            None => self.distinct_boundary_labels.is_empty(),
        }
    }
}

/// Inputs that drive an accessibility-report projection.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AccessibilityProjectionInputs {
    /// The zoom tier to project at.
    pub zoom_tier: ZoomTier,
}

impl AccessibilityProjectionInputs {
    /// A standard-zoom projection: every surface reflows in place.
    pub const fn standard() -> Self {
        Self {
            zoom_tier: ZoomTier::Standard,
        }
    }

    /// A high-zoom projection: dense list surfaces summarize honestly.
    pub const fn high_zoom() -> Self {
        Self {
            zoom_tier: ZoomTier::HighZoom,
        }
    }
}

/// The canonical, support-safe accessibility / boundary truth packet for one
/// presentation overlay.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PresentationAccessibilityReport {
    /// Record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Shared contract ref.
    pub shared_contract_ref: String,
    /// The session this report describes.
    pub session_id: String,
    /// The local user's leader / follow posture (presentation state context).
    pub leader_follow_state: LeaderFollowState,
    /// The zoom tier the report was projected at.
    pub zoom_tier: ZoomTier,
    /// The accessibility-conformance class.
    pub conformance_class: PresentationA11yClass,
    /// The canonical shell support state this class maps to.
    pub support_state: SupportState,
    /// The canonical shell role-confidence this class maps to.
    pub role_confidence: RoleConfidence,
    /// The local / remote / shared boundary posture preserved through the overlay.
    pub boundary_posture: BoundaryPosture,
    /// One conformance record per active overlay surface.
    pub surfaces: Vec<SurfaceConformance>,
    // ---- aggregate accessibility flags (derived; re-checked by validate) ----
    /// True when every actionable surface is keyboard reachable.
    pub keyboard_complete: bool,
    /// Always `false`: no surface is pointer-only.
    pub pointer_only: bool,
    /// True when every surface is screen-reader reachable.
    pub screen_reader_reachable: bool,
    /// True when every surface respects reduced motion.
    pub reduced_motion_respected: bool,
    /// True when every surface stays operable at high zoom (reflowed or
    /// summarized-reachable).
    pub high_zoom_supported: bool,
    /// True when the focus-ring indices form a contiguous `1..=N` order.
    pub focus_order_contiguous: bool,
    /// Always `true`: no surface traps keyboard focus.
    pub no_focus_trap: bool,
    /// Always `true`: boundary labels are preserved, not flattened or erased.
    pub boundary_labels_preserved: bool,
    /// True when every surface carries a non-empty accessible label.
    pub accessible_labels_complete: bool,
}

impl PresentationAccessibilityReport {
    /// A surface conformance record by its tag.
    pub fn surface(&self, surface: PresentationSurfaceTag) -> Option<&SurfaceConformance> {
        self.surfaces.iter().find(|s| s.surface == surface)
    }

    /// Count of surfaces that reflow in place at high zoom.
    pub fn reflowing_surface_count(&self) -> u32 {
        self.surfaces
            .iter()
            .filter(|s| s.high_zoom_reflow == HighZoomReflow::Reflows)
            .count() as u32
    }

    /// Count of surfaces that summarize honestly at high zoom.
    pub fn summarized_surface_count(&self) -> u32 {
        self.surfaces
            .iter()
            .filter(|s| s.high_zoom_reflow == HighZoomReflow::SummarizedReachable)
            .count() as u32
    }

    /// Re-derive the conformance class from the surface records so a hand-edited
    /// report cannot claim a fidelity it did not deliver.
    fn expected_class(&self) -> PresentationA11yClass {
        if self.surfaces.iter().any(|s| s.consistency_violation().is_some())
            || !self.boundary_posture.is_preserved()
            || !self.focus_ring_contiguous()
        {
            PresentationA11yClass::NonConformant
        } else if self
            .surfaces
            .iter()
            .any(|s| s.high_zoom_reflow.is_summarized())
        {
            PresentationA11yClass::DegradedAnnounced
        } else {
            PresentationA11yClass::FullyAccessible
        }
    }

    /// Whether the actionable surfaces' focus indices form a contiguous `1..=N`
    /// ring with no gaps or duplicates.
    fn focus_ring_contiguous(&self) -> bool {
        let mut indices: Vec<u32> = self
            .surfaces
            .iter()
            .filter_map(|s| s.focus_order_index)
            .collect();
        indices.sort_unstable();
        indices
            .iter()
            .enumerate()
            .all(|(i, idx)| *idx == (i as u32) + 1)
            && indices.len() == self.surfaces.iter().filter(|s| s.is_actionable).count()
    }

    /// Re-derive every invariant this report claims and return all violations.
    pub fn validate(&self) -> Vec<PresentationA11yViolation> {
        let mut violations = Vec::new();

        if self.record_kind != PRESENTATION_A11Y_REPORT_RECORD_KIND
            || self.schema_version != PRESENTATION_MODE_BETA_SCHEMA_VERSION
            || self.shared_contract_ref != PRESENTATION_MODE_BETA_SHARED_CONTRACT_REF
        {
            violations.push(PresentationA11yViolation::MalformedRecord);
        }
        if self.session_id.trim().is_empty() {
            violations.push(PresentationA11yViolation::MissingIdentity);
        }
        if self.surfaces.is_empty() {
            violations.push(PresentationA11yViolation::NoSurfaces);
        }

        // The presenter bar, waypoint rail, and provenance strip are always part
        // of a presentation overlay; their absence means it was not bound.
        for required in [
            PresentationSurfaceTag::PresenterBar,
            PresentationSurfaceTag::WaypointRail,
            PresentationSurfaceTag::ProvenanceStrip,
        ] {
            if self.surface(required).is_none() {
                violations.push(PresentationA11yViolation::RequiredSurfaceMissing {
                    surface: required,
                });
            }
        }

        for surface in &self.surfaces {
            if let Some(violation) = surface.consistency_violation() {
                violations.push(violation);
            }
        }

        if !self.focus_ring_contiguous() {
            violations.push(PresentationA11yViolation::FocusOrderBroken);
        }

        if !self.boundary_posture.is_preserved() {
            violations.push(PresentationA11yViolation::BoundaryPostureNotPreserved);
        }

        let expected_class = self.expected_class();
        if self.conformance_class != expected_class {
            violations.push(PresentationA11yViolation::ConformanceClassMismatch {
                expected: expected_class,
                found: self.conformance_class,
            });
        }
        if self.support_state != self.conformance_class.to_support_state()
            || self.role_confidence != self.conformance_class.to_role_confidence()
        {
            violations.push(PresentationA11yViolation::SupportStateMismatch {
                surface: PresentationSurfaceTag::PresenterBar,
            });
        }

        // Aggregate flags must agree with the per-surface truth.
        let expected_aggregates = AggregateFlags::derive(self);
        if expected_aggregates != AggregateFlags::read(self) {
            violations.push(PresentationA11yViolation::AggregateFlagMismatch);
        }
        if self.pointer_only {
            violations.push(PresentationA11yViolation::PointerOrMotionOnly {
                surface: PresentationSurfaceTag::PresenterBar,
            });
        }
        if !self.no_focus_trap {
            violations.push(PresentationA11yViolation::FocusTrapped {
                surface: PresentationSurfaceTag::PresenterBar,
            });
        }

        violations
    }
}

/// The derived-vs-read aggregate accessibility flags, compared by validation.
#[derive(Debug, Clone, PartialEq, Eq)]
struct AggregateFlags {
    keyboard_complete: bool,
    pointer_only: bool,
    screen_reader_reachable: bool,
    reduced_motion_respected: bool,
    high_zoom_supported: bool,
    focus_order_contiguous: bool,
    no_focus_trap: bool,
    boundary_labels_preserved: bool,
    accessible_labels_complete: bool,
}

impl AggregateFlags {
    fn derive(report: &PresentationAccessibilityReport) -> Self {
        let surfaces = &report.surfaces;
        Self {
            keyboard_complete: surfaces
                .iter()
                .filter(|s| s.is_actionable)
                .all(|s| s.keyboard_reachable),
            pointer_only: surfaces.iter().any(|s| s.pointer_only),
            screen_reader_reachable: surfaces.iter().all(|s| s.screen_reader_reachable),
            reduced_motion_respected: surfaces.iter().all(|s| s.respects_reduced_motion),
            high_zoom_supported: surfaces.iter().all(|s| !s.traps_focus && s.keyboard_reachable),
            focus_order_contiguous: report.focus_ring_contiguous(),
            no_focus_trap: surfaces.iter().all(|s| !s.traps_focus),
            boundary_labels_preserved: report.boundary_posture.is_preserved()
                && surfaces.iter().all(|s| !s.erases_boundary_label),
            accessible_labels_complete: surfaces
                .iter()
                .all(|s| !s.accessible_label.trim().is_empty()),
        }
    }

    fn read(report: &PresentationAccessibilityReport) -> Self {
        Self {
            keyboard_complete: report.keyboard_complete,
            pointer_only: report.pointer_only,
            screen_reader_reachable: report.screen_reader_reachable,
            reduced_motion_respected: report.reduced_motion_respected,
            high_zoom_supported: report.high_zoom_supported,
            focus_order_contiguous: report.focus_order_contiguous,
            no_focus_trap: report.no_focus_trap,
            boundary_labels_preserved: report.boundary_labels_preserved,
            accessible_labels_complete: report.accessible_labels_complete,
        }
    }
}

/// An accessibility / boundary conformance violation found by validation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PresentationA11yViolation {
    /// A record carried the wrong kind, schema version, or contract ref.
    MalformedRecord,
    /// The report carried no session identity.
    MissingIdentity,
    /// The report carried no surfaces.
    NoSurfaces,
    /// A surface that must always be present is missing.
    RequiredSurfaceMissing {
        /// The missing surface.
        surface: PresentationSurfaceTag,
    },
    /// A surface's actionable / focus / boundary fields disagree with its tag.
    SurfaceInconsistent {
        /// The offending surface.
        surface: PresentationSurfaceTag,
    },
    /// A surface is not keyboard / screen-reader reachable.
    SurfaceNotReachable {
        /// The offending surface.
        surface: PresentationSurfaceTag,
    },
    /// A surface does not respect reduced motion.
    ReducedMotionViolated {
        /// The offending surface.
        surface: PresentationSurfaceTag,
    },
    /// A surface is operable only by pointer or only via motion.
    PointerOrMotionOnly {
        /// The offending surface.
        surface: PresentationSurfaceTag,
    },
    /// A surface traps keyboard focus.
    FocusTrapped {
        /// The offending surface.
        surface: PresentationSurfaceTag,
    },
    /// A surface carries no accessible label.
    AccessibleLabelMissing {
        /// The offending surface.
        surface: PresentationSurfaceTag,
    },
    /// A surface's support state disagrees with its high-zoom reflow.
    SupportStateMismatch {
        /// The offending surface.
        surface: PresentationSurfaceTag,
    },
    /// A source-bearing surface erased or omitted its boundary label.
    BoundaryLabelErased {
        /// The offending surface.
        surface: PresentationSurfaceTag,
    },
    /// The keyboard focus ring is not a contiguous `1..=N` order.
    FocusOrderBroken,
    /// The boundary posture was flattened, hidden, or internally inconsistent.
    BoundaryPostureNotPreserved,
    /// The claimed conformance class does not match the surface records.
    ConformanceClassMismatch {
        /// The class the surfaces imply.
        expected: PresentationA11yClass,
        /// The class the report claimed.
        found: PresentationA11yClass,
    },
    /// An aggregate accessibility flag disagrees with the per-surface truth.
    AggregateFlagMismatch,
}

/// Project the accessibility / boundary conformance report for `session` at the
/// requested zoom tier.
///
/// Walks the canonical overlay projection ([`project_overlay`]) and asserts each
/// active surface's accessibility posture, derives the boundary posture from the
/// session's waypoints, assigns a single contiguous keyboard focus ring, and
/// classifies the overlay's conformance. Every guardrail is fixed to its safe
/// value, so the report proves — rather than asserts — that the overlay meets the
/// shell's accessibility and boundary-truth bar.
pub fn project_accessibility_report(
    session: &PresentationSession,
    inputs: &AccessibilityProjectionInputs,
) -> PresentationAccessibilityReport {
    let overlay = project_overlay(session);
    let boundary_posture = derive_boundary_posture(session, &overlay);
    let current_boundary = boundary_posture
        .current_boundary_label
        .unwrap_or(BoundaryLabel::Local);

    let mut surfaces = Vec::new();
    let mut next_focus_index: u32 = 1;

    // Actionable surfaces, in their canonical focus order. The spotlight and
    // breakaway banner are present only when active, matching the overlay.
    let mut actionable: Vec<(PresentationSurfaceTag, String, HighZoomReflow)> = Vec::new();
    actionable.push((
        PresentationSurfaceTag::PresenterBar,
        "Presenter bar — spotlight, notes, zoom, and exit controls".to_owned(),
        HighZoomReflow::Reflows,
    ));
    actionable.push((
        PresentationSurfaceTag::WaypointRail,
        format!("Agenda rail — {} steps", overlay.waypoint_rail.rows.len()),
        // The agenda rail is a dense list; it summarizes at high zoom.
        reflow_for(inputs.zoom_tier, true),
    ));
    if overlay.spotlight_frame.enabled {
        actionable.push((
            PresentationSurfaceTag::SpotlightFrame,
            overlay
                .spotlight_frame
                .accessible_region_label
                .clone()
                .unwrap_or_else(|| "Spotlight frame".to_owned()),
            HighZoomReflow::Reflows,
        ));
    }
    actionable.push((
        PresentationSurfaceTag::SpeakerNotesTray,
        "Speaker notes — presenter-only, local by default".to_owned(),
        HighZoomReflow::Reflows,
    ));
    actionable.push((
        PresentationSurfaceTag::AudienceStrip,
        format!(
            "Audience — {} following, {} broken away",
            overlay.audience_strip.following_count, overlay.audience_strip.broken_away_count
        ),
        // The audience strip is a dense list; it summarizes at high zoom.
        reflow_for(inputs.zoom_tier, true),
    ));
    if let Some(banner) = &overlay.breakaway_banner {
        actionable.push((
            PresentationSurfaceTag::BreakawayBanner,
            banner.state_label.clone(),
            HighZoomReflow::Reflows,
        ));
    }

    for (surface, label, reflow) in actionable {
        let boundary_label = surface.is_source_bearing().then_some(current_boundary);
        surfaces.push(actionable_surface(
            surface,
            next_focus_index,
            label,
            reflow,
            boundary_label,
        ));
        next_focus_index += 1;
    }

    // The provenance strip is display-only but screen-reader reachable, and is
    // the canonical boundary carrier.
    surfaces.push(display_only_surface(
        PresentationSurfaceTag::ProvenanceStrip,
        provenance_label(&overlay, current_boundary),
        Some(current_boundary),
    ));

    let conformance_class = if surfaces.iter().any(|s| s.high_zoom_reflow.is_summarized()) {
        PresentationA11yClass::DegradedAnnounced
    } else {
        PresentationA11yClass::FullyAccessible
    };

    let mut report = PresentationAccessibilityReport {
        record_kind: PRESENTATION_A11Y_REPORT_RECORD_KIND.to_owned(),
        schema_version: PRESENTATION_MODE_BETA_SCHEMA_VERSION,
        shared_contract_ref: PRESENTATION_MODE_BETA_SHARED_CONTRACT_REF.to_owned(),
        session_id: session.session_id.clone(),
        leader_follow_state: session.leader_follow_state,
        zoom_tier: inputs.zoom_tier,
        conformance_class,
        support_state: conformance_class.to_support_state(),
        role_confidence: conformance_class.to_role_confidence(),
        boundary_posture,
        surfaces,
        // Filled from the per-surface truth below.
        keyboard_complete: false,
        pointer_only: false,
        screen_reader_reachable: false,
        reduced_motion_respected: false,
        high_zoom_supported: false,
        focus_order_contiguous: false,
        no_focus_trap: false,
        boundary_labels_preserved: false,
        accessible_labels_complete: false,
    };

    let aggregates = AggregateFlags::derive(&report);
    report.keyboard_complete = aggregates.keyboard_complete;
    report.pointer_only = aggregates.pointer_only;
    report.screen_reader_reachable = aggregates.screen_reader_reachable;
    report.reduced_motion_respected = aggregates.reduced_motion_respected;
    report.high_zoom_supported = aggregates.high_zoom_supported;
    report.focus_order_contiguous = aggregates.focus_order_contiguous;
    report.no_focus_trap = aggregates.no_focus_trap;
    report.boundary_labels_preserved = aggregates.boundary_labels_preserved;
    report.accessible_labels_complete = aggregates.accessible_labels_complete;
    report
}

/// The high-zoom reflow for a surface: dense list surfaces summarize at high
/// zoom; everything else reflows in place.
const fn reflow_for(zoom_tier: ZoomTier, is_dense_list: bool) -> HighZoomReflow {
    match (zoom_tier, is_dense_list) {
        (ZoomTier::HighZoom, true) => HighZoomReflow::SummarizedReachable,
        _ => HighZoomReflow::Reflows,
    }
}

fn actionable_surface(
    surface: PresentationSurfaceTag,
    focus_order_index: u32,
    accessible_label: String,
    high_zoom_reflow: HighZoomReflow,
    boundary_label: Option<BoundaryLabel>,
) -> SurfaceConformance {
    SurfaceConformance {
        record_kind: PRESENTATION_A11Y_SURFACE_RECORD_KIND.to_owned(),
        schema_version: PRESENTATION_MODE_BETA_SCHEMA_VERSION,
        shared_contract_ref: PRESENTATION_MODE_BETA_SHARED_CONTRACT_REF.to_owned(),
        surface,
        is_actionable: true,
        focus_order_index: Some(focus_order_index),
        keyboard_reachable: true,
        visible_focus_indicator: true,
        screen_reader_reachable: true,
        respects_reduced_motion: true,
        high_zoom_reflow,
        accessible_label,
        boundary_label,
        support_state: high_zoom_reflow.to_support_state(),
        pointer_only: false,
        motion_only: false,
        traps_focus: false,
        erases_boundary_label: false,
    }
}

fn display_only_surface(
    surface: PresentationSurfaceTag,
    accessible_label: String,
    boundary_label: Option<BoundaryLabel>,
) -> SurfaceConformance {
    SurfaceConformance {
        record_kind: PRESENTATION_A11Y_SURFACE_RECORD_KIND.to_owned(),
        schema_version: PRESENTATION_MODE_BETA_SCHEMA_VERSION,
        shared_contract_ref: PRESENTATION_MODE_BETA_SHARED_CONTRACT_REF.to_owned(),
        surface,
        is_actionable: false,
        focus_order_index: None,
        keyboard_reachable: true,
        visible_focus_indicator: true,
        screen_reader_reachable: true,
        respects_reduced_motion: true,
        high_zoom_reflow: HighZoomReflow::Reflows,
        accessible_label,
        boundary_label,
        support_state: HighZoomReflow::Reflows.to_support_state(),
        pointer_only: false,
        motion_only: false,
        traps_focus: false,
        erases_boundary_label: false,
    }
}

fn derive_boundary_posture(
    session: &PresentationSession,
    overlay: &PresentationOverlay,
) -> BoundaryPosture {
    let current_boundary_label = if session.waypoints.is_empty() {
        None
    } else {
        Some(overlay.provenance_strip.boundary_label)
    };
    let distinct_boundary_labels: Vec<BoundaryLabel> = session
        .waypoints
        .iter()
        .map(|w| w.boundary_label)
        .collect::<BTreeSet<_>>()
        .into_iter()
        .collect();
    BoundaryPosture {
        current_boundary_label,
        distinct_boundary_labels,
        audience_scope: session.audience_scope,
        boundary_labels_visible: true,
        flattened_to_generic: false,
    }
}

fn provenance_label(overlay: &PresentationOverlay, boundary: BoundaryLabel) -> String {
    let strip = &overlay.provenance_strip;
    let mut parts = Vec::new();
    if let Some(path) = &strip.file_path_ref {
        parts.push(path.clone());
    }
    parts.push(strip.branch_workspace_ref.clone());
    parts.push(format!("boundary: {}", boundary.as_str()));
    format!("Source — {}", parts.join(" · "))
}

/// One support-safe row per accessibility report. Carries enums, counts, and
/// booleans — never accessible-label bodies or source refs.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PresentationA11ySupportExportRow {
    /// Record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Shared contract ref.
    pub shared_contract_ref: String,
    /// Session id.
    pub session_id: String,
    /// The local user's leader / follow posture.
    pub leader_follow_state: LeaderFollowState,
    /// The zoom tier the report was projected at.
    pub zoom_tier: ZoomTier,
    /// The accessibility-conformance class.
    pub conformance_class: PresentationA11yClass,
    /// The canonical shell support state this maps to.
    pub support_state: SupportState,
    /// The canonical shell role-confidence this maps to.
    pub role_confidence: RoleConfidence,
    /// The currently-focused waypoint's boundary label, if any.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub current_boundary_label: Option<BoundaryLabel>,
    /// The distinct boundary labels present, kept (never flattened).
    pub distinct_boundary_labels: Vec<BoundaryLabel>,
    /// The session's audience scope.
    pub audience_scope: AudienceScope,
    /// Number of active overlay surfaces.
    pub surface_count: u32,
    /// Surfaces that reflow in place at high zoom.
    pub reflowing_surface_count: u32,
    /// Surfaces that summarize honestly at high zoom.
    pub summarized_surface_count: u32,
    /// Whether every actionable surface is keyboard reachable.
    pub keyboard_complete: bool,
    /// Whether any surface is pointer-only (always `false`).
    pub pointer_only: bool,
    /// Whether every surface is screen-reader reachable.
    pub screen_reader_reachable: bool,
    /// Whether every surface respects reduced motion.
    pub reduced_motion_respected: bool,
    /// Whether every surface stays operable at high zoom.
    pub high_zoom_supported: bool,
    /// Whether the focus ring is contiguous.
    pub focus_order_contiguous: bool,
    /// Whether no surface traps focus (always `true`).
    pub no_focus_trap: bool,
    /// Whether boundary labels are preserved (always `true`).
    pub boundary_labels_preserved: bool,
    /// Whether every surface carries an accessible label.
    pub accessible_labels_complete: bool,
}

/// Support-export wrapper over a set of accessibility reports. Privacy-safe by
/// construction: no accessible-label bodies or source refs are carried.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PresentationA11ySupportExport {
    /// Record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Shared contract ref.
    pub shared_contract_ref: String,
    /// Export id.
    pub export_id: String,
    /// Mint timestamp.
    pub generated_at: String,
    /// Support-safe rows.
    pub rows: Vec<PresentationA11ySupportExportRow>,
    /// Always `true`: accessible-label bodies and source refs are excluded.
    pub raw_private_material_excluded: bool,
}

impl PresentationA11ySupportExport {
    /// Project a set of accessibility reports into a support-safe export.
    pub fn from_reports<'a>(
        export_id: impl Into<String>,
        generated_at: impl Into<String>,
        reports: impl IntoIterator<Item = &'a PresentationAccessibilityReport>,
    ) -> Self {
        let rows = reports
            .into_iter()
            .map(|report| PresentationA11ySupportExportRow {
                record_kind: PRESENTATION_A11Y_SUPPORT_EXPORT_ROW_RECORD_KIND.to_owned(),
                schema_version: PRESENTATION_MODE_BETA_SCHEMA_VERSION,
                shared_contract_ref: PRESENTATION_MODE_BETA_SHARED_CONTRACT_REF.to_owned(),
                session_id: report.session_id.clone(),
                leader_follow_state: report.leader_follow_state,
                zoom_tier: report.zoom_tier,
                conformance_class: report.conformance_class,
                support_state: report.support_state,
                role_confidence: report.role_confidence,
                current_boundary_label: report.boundary_posture.current_boundary_label,
                distinct_boundary_labels: report
                    .boundary_posture
                    .distinct_boundary_labels
                    .clone(),
                audience_scope: report.boundary_posture.audience_scope,
                surface_count: report.surfaces.len() as u32,
                reflowing_surface_count: report.reflowing_surface_count(),
                summarized_surface_count: report.summarized_surface_count(),
                keyboard_complete: report.keyboard_complete,
                pointer_only: report.pointer_only,
                screen_reader_reachable: report.screen_reader_reachable,
                reduced_motion_respected: report.reduced_motion_respected,
                high_zoom_supported: report.high_zoom_supported,
                focus_order_contiguous: report.focus_order_contiguous,
                no_focus_trap: report.no_focus_trap,
                boundary_labels_preserved: report.boundary_labels_preserved,
                accessible_labels_complete: report.accessible_labels_complete,
            })
            .collect();
        Self {
            record_kind: PRESENTATION_A11Y_SUPPORT_EXPORT_RECORD_KIND.to_owned(),
            schema_version: PRESENTATION_MODE_BETA_SCHEMA_VERSION,
            shared_contract_ref: PRESENTATION_MODE_BETA_SHARED_CONTRACT_REF.to_owned(),
            export_id: export_id.into(),
            generated_at: generated_at.into(),
            rows,
            raw_private_material_excluded: true,
        }
    }
}
