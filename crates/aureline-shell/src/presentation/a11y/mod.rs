//! Accessibility, reduced-motion, and local/remote/shared boundary conformance
//! for the presentation overlay surfaces.
//!
//! A claimed presentation surface must pass the **same accessibility and
//! boundary-truth expectations as the rest of the shell** — not a softer
//! "presentation looks fine" bar. This module turns that expectation into a
//! governed, inspectable truth packet rather than a post-hoc manual signoff: it
//! proves keyboard order, visible focus, reduced-motion behavior, screen-reader
//! reachability, high-zoom support, and accessible labels across the presenter
//! bar, agenda / waypoint rail, spotlight inset, speaker-notes tray, audience
//! strip, breakaway banner, and provenance strip, and keeps the explicit local /
//! remote / shared boundary labels visible through the overlay and into its
//! diagnostics / export packets.
//!
//! - [`conformance`] holds the data model — the
//!   [`PresentationSurfaceTag`](conformance::PresentationSurfaceTag) surfaces, the
//!   per-dimension [`SurfaceConformance`](conformance::SurfaceConformance), the
//!   [`BoundaryPosture`](conformance::BoundaryPosture), the
//!   [`PresentationA11yClass`](conformance::PresentationA11yClass) fidelity
//!   vocabulary (mapped onto the shell's
//!   [`SupportState`](crate::a11y::tree_contract::SupportState) /
//!   [`RoleConfidence`](crate::a11y::tree_contract::RoleConfidence)), the
//!   [`PresentationAccessibilityReport`](conformance::PresentationAccessibilityReport)
//!   packet, [`project_accessibility_report`](conformance::project_accessibility_report),
//!   and [`PresentationAccessibilityReport::validate`](conformance::PresentationAccessibilityReport::validate),
//!   which re-derives every accessibility and boundary invariant.
//! - [`corpus`] is the mint-from-truth seed corpus, support export, and
//!   validation that the checked-in fixtures and headless inspectors share.
//!
//! The canonical session and overlay objects this module inspects live in
//! [`crate::presentation_mode`]; the thin overlay/navigation binding is
//! [`crate::presentation::binding`]. The support-export boundary schema is
//! [`schemas/presentation/accessibility-and-boundary-report.schema.json`](../../../../../schemas/presentation/accessibility-and-boundary-report.schema.json);
//! the human-readable contract is `docs/ux/presentation-accessibility.md` and the
//! coverage matrix is
//! `artifacts/presentation/accessibility-and-boundary-report.md`.

pub mod conformance;
pub mod corpus;

pub use conformance::{
    project_accessibility_report, AccessibilityProjectionInputs, BoundaryPosture, HighZoomReflow,
    PresentationA11yClass, PresentationA11ySupportExport, PresentationA11ySupportExportRow,
    PresentationA11yViolation, PresentationAccessibilityReport, PresentationSurfaceTag,
    SurfaceConformance, ZoomTier, PRESENTATION_ACCESSIBILITY_AND_BOUNDARY_REPORT_REF,
    PRESENTATION_ACCESSIBILITY_DOC_REF, PRESENTATION_A11Y_FIXTURE_DIR,
    PRESENTATION_A11Y_REPORT_RECORD_KIND, PRESENTATION_A11Y_SUPPORT_EXPORT_RECORD_KIND,
    PRESENTATION_A11Y_SUPPORT_EXPORT_ROW_RECORD_KIND, PRESENTATION_A11Y_SURFACE_RECORD_KIND,
};
pub use corpus::{
    presentation_a11y_support_export, seeded_presentation_a11y_corpus,
    validate_presentation_a11y_corpus, A11yCase, A11yCorpusError, A11yCorpusSummary,
    PresentationA11yCorpus, PRESENTATION_A11Y_CASE_RECORD_KIND, PRESENTATION_A11Y_CORPUS_RECORD_KIND,
};

#[cfg(test)]
mod tests;
