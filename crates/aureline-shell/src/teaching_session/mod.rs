//! Teaching/classroom sessions as a thin, reversible layer over learning mode.
//!
//! A teaching or classroom session lets a presenter run a guided walkthrough —
//! tours, exercise packs, glossary cards, and speaker notes — over the *same*
//! learning-mode objects the user already has, **without** building a parallel
//! collaboration product, a hidden progress model, or a cohort/grading flow. The
//! exit-gate contract is that the layer stays role-aware, cited, non-mutating by
//! default, honest about offline/cached docs-pack states, and reversible: it
//! always restores the prior workspace layout and focus on exit, leave, or crash
//! recovery.
//!
//! The module is split the same way the rest of the shell's governed-session
//! objects are:
//!
//! - [`session`] holds the canonical [`TeachingSession`] boundary object —
//!   session id, kind, lifecycle, local role, replay policy, retention class,
//!   exercise-pack refs, the segment list (each citing a learning-mode object
//!   and carrying its docs-pack availability state), the participant list, and
//!   the restore checkpoint — plus [`restore_from_checkpoint`], which proves the
//!   prior environment returns identically under every restore trigger.
//! - [`affordances`] projects one session into the role-aware control
//!   affordances each seat actually sees, proving teaching roles stay separate
//!   from terminal/debug control and that limited / low-bandwidth clients join
//!   safely as observers or note-takers without broken or misleading controls.
//! - [`corpus`] is the mint-from-truth seed corpus, support export, and
//!   validation that the checked-in fixtures and the headless inspector share.
//!
//! The canonical boundary object is frozen at
//! `schemas/help/teaching_session.schema.json` and documented in
//! `docs/help/m3/teaching_and_classroom_conformance.md`.

pub mod affordances;
pub mod corpus;
pub mod session;

pub use affordances::{
    project_affordances, AffordanceKind, ParticipantAffordanceView, TeachingAffordanceProjection,
    TeachingControlAffordance, LOCAL_PARTICIPANT_ID, TEACHING_AFFORDANCE_PROJECTION_RECORD_KIND,
};
pub use corpus::{
    seeded_teaching_classroom_corpus, validate_teaching_classroom_corpus,
    TeachingClassroomCorpus, TeachingClassroomCorpusSummary, TeachingClassroomSessionCase,
    TeachingClassroomSupportExport, TeachingClassroomSupportExportRow,
    TEACHING_CLASSROOM_CORPUS_RECORD_KIND, TEACHING_CLASSROOM_SESSION_CASE_RECORD_KIND,
    TEACHING_CLASSROOM_SUPPORT_EXPORT_RECORD_KIND, TEACHING_CLASSROOM_SUPPORT_EXPORT_ROW_RECORD_KIND,
};
pub use session::{
    restore_from_checkpoint, ClientClass, DemonstratedAction, DemonstrationKind, DocsPackState,
    ReplayPolicy, RestoreCheckpoint, RestoreTrigger, RetentionClass, SegmentKind, SessionKind,
    SessionLifecycleState, TeachingParticipant, TeachingRestoreOutcome, TeachingRole,
    TeachingSegment, TeachingSession, TeachingSessionBuilder, TEACHING_RESTORE_OUTCOME_RECORD_KIND,
    TEACHING_SESSION_BETA_SCHEMA_VERSION, TEACHING_SESSION_BETA_SHARED_CONTRACT_REF,
    TEACHING_SESSION_RECORD_KIND,
};

#[cfg(test)]
mod tests;
