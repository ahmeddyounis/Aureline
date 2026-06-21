//! Speaker-note objects with local-only defaults, explicit share promotion,
//! citation refs, retention / export posture, and no accidental audience leakage.
//!
//! The canonical [`SpeakerNote`](crate::presentation_mode::SpeakerNote) boundary
//! object — a presenter-only prompt attached to one waypoint, local by default —
//! lives in [`crate::presentation_mode`]. This module is the governance layer the
//! spec calls for: it makes the *consequences* of a note's scope visible and
//! auditable without turning notes into mandatory shared collaboration data.
//!
//! - [`sharing`] wraps a note in a [`GovernedSpeakerNote`] with typed
//!   [`NoteCitation`]s and a visible retention / shared-state / export posture;
//!   [`promote_note_to_shared`] is the only local → shared path and emits a
//!   reopenable [`SpeakerNoteShareRecord`]; [`project_audience_note_disclosures`]
//!   proves a follower surface only ever sees a deliberately shared note; and
//!   [`SpeakerNoteSharingExport`] is the support / diagnostics projection that
//!   records scope honestly while carrying no note body.
//! - [`corpus`] is the mint-from-truth seed corpus, support export, and
//!   validation that the checked-in fixtures and headless inspectors share.
//!
//! The support-export boundary schema is
//! [`schemas/presentation/speaker-note-export.schema.json`](../../../../../schemas/presentation/speaker-note-export.schema.json);
//! the human-readable privacy contract is
//! `docs/privacy/presentation-speaker-notes.md`.

pub mod corpus;
pub mod sharing;

pub use corpus::{
    seeded_speaker_note_sharing_corpus, speaker_note_sharing_support_export,
    validate_speaker_note_sharing_corpus, SpeakerNoteSharingCase, SpeakerNoteSharingCorpus,
    SpeakerNoteSharingCorpusError, SpeakerNoteSharingSummary,
    SPEAKER_NOTE_SHARING_CASE_RECORD_KIND, SPEAKER_NOTE_SHARING_CORPUS_RECORD_KIND,
};
pub use sharing::{
    project_audience_note_disclosures, promote_note_to_shared, AudienceNoteDisclosure,
    GovernedSpeakerNote, NoteBodyExportPosture, NoteCitation, NoteCitationKind, NotePromotionError,
    NotePromotionRequest, NoteRetentionPosture, NoteSharedStateVisibility, SharedRetentionChoice,
    SpeakerNote, SpeakerNoteScope, SpeakerNoteShareRecord, SpeakerNoteSharingDiagnosticsRow,
    SpeakerNoteSharingExport, SpeakerNoteSharingViolation, SPEAKER_NOTE_EXPORT_SCHEMA_REF,
    SPEAKER_NOTE_PRIVACY_DOC_REF, SPEAKER_NOTE_SHARE_RECORD_KIND,
    SPEAKER_NOTE_SHARING_DIAGNOSTICS_ROW_RECORD_KIND, SPEAKER_NOTE_SHARING_EXPORT_RECORD_KIND,
};

#[cfg(test)]
mod tests;
