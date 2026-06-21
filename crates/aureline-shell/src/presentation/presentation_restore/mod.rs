//! Presentation-session restore checkpoints, layout fidelity, and honest crash /
//! interrupted-resume recovery.
//!
//! Entering presentation mode checkpoints the prior layout and selection context;
//! exit, cancel, crash recovery, and interrupted resume all replay that
//! checkpoint so the user is never stranded in an improvised layout. This module
//! turns that restore into a governed truth packet that classifies its own
//! fidelity with the same vocabulary durable shell contexts use, degrades
//! honestly when targets are gone, and never silently re-runs an action or
//! re-acquires expired authority.
//!
//! - [`restore`] holds the data model — the [`PresentationRestoreClass`] fidelity
//!   vocabulary (mapped onto the durable-shell
//!   [`RestoreClass`](aureline_recovery::session_restore::records::RestoreClass)),
//!   the honest [`WaypointAvailability`] / [`RestoreDegradeTrigger`] degrade
//!   states, the [`PresentationRestoreReport`] packet, the projection functions,
//!   and [`PresentationRestoreReport::validate`], which re-derives every honesty
//!   invariant.
//! - [`corpus`] is the mint-from-truth seed corpus, support export, and
//!   validation that the checked-in fixtures and headless inspectors share.
//!
//! The canonical session and checkpoint objects this module restores live in
//! [`crate::presentation_mode`]. The support-export boundary schema is
//! [`schemas/presentation/restore-report.schema.json`](../../../../../schemas/presentation/restore-report.schema.json);
//! the human-readable contract is `docs/help/presentation-restore-and-recovery.md`
//! and the restore / crash coverage matrix is
//! `artifacts/presentation/restore-and-crash-matrix.md`.

pub mod corpus;
pub mod restore;

pub use corpus::{
    presentation_restore_support_export, seeded_presentation_restore_corpus,
    validate_presentation_restore_corpus, PresentationRestoreCorpus, RestoreCase,
    RestoreCorpusError, RestoreCorpusSummary, PRESENTATION_RESTORE_CASE_RECORD_KIND,
    PRESENTATION_RESTORE_CORPUS_RECORD_KIND,
};
pub use restore::{
    project_evidence_only_report, project_no_restore_report, project_restore_report,
    PresentationRestoreClass, PresentationRestoreLifecycle, PresentationRestoreReport,
    PresentationRestoreSupportExport, PresentationRestoreSupportExportRow,
    PresentationRestoreTrigger, PresentationRestoreViolation, RestoreDegradeTrigger,
    RestoreProjectionInputs, WaypointAvailability, WaypointDegrade, WaypointRestoreState,
    PRESENTATION_RESTORE_AND_CRASH_MATRIX_REF, PRESENTATION_RESTORE_AND_RECOVERY_DOC_REF,
    PRESENTATION_RESTORE_FIXTURE_DIR, PRESENTATION_RESTORE_REPORT_RECORD_KIND,
    PRESENTATION_RESTORE_SUPPORT_EXPORT_RECORD_KIND,
    PRESENTATION_RESTORE_SUPPORT_EXPORT_ROW_RECORD_KIND, PRESENTATION_WAYPOINT_RESTORE_RECORD_KIND,
};

#[cfg(test)]
mod tests;
