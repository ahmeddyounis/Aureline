//! Presentation overlays bound onto the existing pane-and-navigation system.
//!
//! The canonical presentation object model — the [`PresentationSession`], its
//! waypoints, speaker notes, and the reversible
//! [`PresentationOverlay`](crate::presentation_mode::PresentationOverlay)
//! projection — lives in [`crate::presentation_mode`]. This module is the thin
//! binding layer the spec calls for: it places those overlay surfaces (presenter
//! bar, agenda / waypoint rail, spotlight frame, zoom presets, speaker-notes
//! tray, audience strip, and breakaway banner) **on top of Aureline's existing
//! shell zones and navigation provenance** rather than building a second
//! workspace shell.
//!
//! - [`binding`] projects a session and a live
//!   [`ZoneRegistryLayout`](crate::layout::zone_registry::ZoneRegistryLayout)
//!   into a [`PresentationOverlayNavigationBinding`]: an inspectable truth packet
//!   that proves every surface attaches to a canonical
//!   [`ShellZoneId`](crate::layout::zone_registry::ShellZoneId), the spotlight is
//!   a strict inset within the main workspace pane, source provenance stays
//!   visible, every actionable surface is command-backed and keyboard reachable,
//!   and entering checkpoints a layout that exit / cancel / crash recovery all
//!   restore.
//! - [`corpus`] is the mint-from-truth seed corpus, support export, and
//!   validation that the checked-in fixtures and headless inspectors share.
//! - [`speaker_notes`] governs the speaker-note objects that ride the
//!   speaker-notes tray: local-only defaults, explicit share promotion, typed
//!   citation refs, retention / export posture, and the audience projection that
//!   keeps a private note off any follower surface.
//!
//! The geometry primitive that decides which zone each surface rides — and that
//! it never replaces a pane — is
//! [`crate::layout::presentation_overlays`]. The human-readable contract is
//! `docs/ux/presentation-overlays-and-navigation.md`.

pub mod binding;
pub mod corpus;
pub mod speaker_notes;

pub use binding::{
    project_overlay_navigation_binding, LayoutCheckpointBinding, NavigationProvenanceBinding,
    OverlayPlacementBinding, OverlayRect, OverlaySurfaceTag, PresentationBindingViolation,
    PresentationOverlayNavigationBinding, ShellZoneTag,
    PRESENTATION_OVERLAYS_AND_NAVIGATION_DOC_REF, PRESENTATION_OVERLAY_BINDING_RECORD_KIND,
    PRESENTATION_OVERLAY_FIXTURE_DIR,
};
pub use corpus::{
    seeded_overlay_navigation_corpus, validate_overlay_navigation_corpus,
    OverlayBindingCorpusError, PresentationOverlayBindingCase, PresentationOverlayBindingCorpus,
    PresentationOverlayBindingSummary, PresentationOverlayBindingSupportExport,
    PresentationOverlayBindingSupportExportRow, PRESENTATION_OVERLAY_BINDING_CASE_RECORD_KIND,
    PRESENTATION_OVERLAY_BINDING_CORPUS_RECORD_KIND,
    PRESENTATION_OVERLAY_BINDING_SUPPORT_EXPORT_RECORD_KIND,
    PRESENTATION_OVERLAY_BINDING_SUPPORT_EXPORT_ROW_RECORD_KIND,
};

#[cfg(test)]
mod tests;
