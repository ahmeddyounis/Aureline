//! Presentation-mode overlays as a thin, reversible layer over existing work.
//!
//! Presentation mode lets a presenter guide attention through code, diffs,
//! topology maps, docs, and notebook content **without** building a parallel
//! product. It reuses the existing editor, diff, docs, graph, and notebook
//! surfaces and layers governed overlay chrome on top of them. The exit-gate
//! contract is that the layer stays thin and reversible: it never hides
//! mutation shortcuts, never widens collaboration authority, never claims
//! private data ownership, preserves source provenance, and always restores the
//! prior layout and focus on exit, cancel, or crash recovery.
//!
//! The module is split the same way the rest of the shell's governed-session
//! objects are:
//!
//! - [`session`] holds the canonical [`PresentationSession`] boundary object —
//!   session id, leader/follow state, waypoint list, layout preset, current
//!   focus ref, audience scope, speaker notes, and the restore checkpoint —
//!   plus [`restore_from_checkpoint`], which proves the prior environment comes
//!   back identically under every restore trigger.
//! - [`overlay`] projects one session into the six design-system presentation
//!   surfaces (presenter bar, waypoint rail, spotlight frame, speaker-notes
//!   tray, audience strip / follow chip, and breakaway banner) with
//!   keyboard-complete, attention-only [`KeyboardAction`]s.
//! - [`corpus`] is the mint-from-truth seed corpus, support export, and
//!   validation that the checked-in fixtures and the headless inspector share.
//!
//! The canonical boundary object is frozen at
//! `schemas/help/presentation_session.schema.json` and documented in
//! `docs/help/m3/presentation_mode_beta.md`.

pub mod corpus;
pub mod overlay;
pub mod session;

pub use corpus::{
    seeded_presentation_mode_corpus, validate_presentation_mode_corpus, PresentationModeCorpus,
    PresentationModeCorpusSummary, PresentationModeSupportExport, PresentationModeSupportExportRow,
    PresentationSessionCase, PRESENTATION_MODE_CORPUS_RECORD_KIND,
    PRESENTATION_MODE_SUPPORT_EXPORT_RECORD_KIND, PRESENTATION_MODE_SUPPORT_EXPORT_ROW_RECORD_KIND,
    PRESENTATION_SESSION_CASE_RECORD_KIND,
};
pub use overlay::{
    project_overlay, AudienceStrip, BreakawayBanner, FollowChip, KeyboardAction,
    PresentationOverlay, PresenterBar, ProvenanceStrip, RestoreAffordance, SpeakerNotesTray,
    SpeakerNotesTrayRow, SpotlightFrame, WaypointRail, WaypointRailRow, ZoomPreset,
    PRESENTATION_OVERLAY_RECORD_KIND,
};
pub use session::{
    restore_from_checkpoint, AudienceParticipant, AudienceScope, BoundaryLabel, FollowWaypoint,
    LayoutPreset, LeaderFollowState, ParticipantFollowState, ParticipantRole, PresentationSession,
    PresentationSessionBuilder, RestoreCheckpoint, RestoreOutcome, RestoreTrigger,
    SessionLifecycleState, SpeakerNote, SpeakerNoteScope, WalkthroughSurfaceKind,
    WaypointCompletionState, PRESENTATION_MODE_BETA_SCHEMA_VERSION,
    PRESENTATION_MODE_BETA_SHARED_CONTRACT_REF, PRESENTATION_RESTORE_OUTCOME_RECORD_KIND,
    PRESENTATION_SESSION_RECORD_KIND,
};

#[cfg(test)]
mod tests;
