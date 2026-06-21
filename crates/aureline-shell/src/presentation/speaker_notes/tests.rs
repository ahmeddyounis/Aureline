//! Unit tests for the speaker-note sharing governance model.

use super::corpus::{
    seeded_speaker_note_sharing_corpus, speaker_note_sharing_support_export,
    validate_speaker_note_sharing_corpus, SpeakerNoteSharingCorpus,
};
use super::sharing::{
    project_audience_note_disclosures, promote_note_to_shared, GovernedSpeakerNote, NoteCitation,
    NoteCitationKind, NotePromotionError, NotePromotionRequest, NoteRetentionPosture,
    NoteSharedStateVisibility, SharedRetentionChoice, SpeakerNoteSharingViolation,
};
use crate::presentation_mode::{SpeakerNote, SpeakerNoteScope};

fn local(note_id: &str, body: &str) -> GovernedSpeakerNote {
    let note = SpeakerNote::local(note_id, "wp:1", body)
        .with_citations(vec!["file:src/lib.rs".to_owned()]);
    GovernedSpeakerNote::local(
        note,
        vec![NoteCitation::new("file:src/lib.rs", NoteCitationKind::File)],
    )
}

fn explicit_request(decision: &str) -> NotePromotionRequest {
    NotePromotionRequest {
        explicit_acknowledgement: true,
        retention_choice: SharedRetentionChoice::RetainInSessionStore,
        share_decision_ref: decision.to_owned(),
        promoted_at: "2026-06-20T09:30:00Z".to_owned(),
    }
}

#[test]
fn a_fresh_governed_note_is_local_and_private() {
    let note = local("note:1", "private prompt");
    assert!(note.is_local());
    assert!(!note.is_shared());
    assert_eq!(note.note.scope, SpeakerNoteScope::Local);
    assert!(!note.note.shared_promotion_explicit);
    assert_eq!(
        note.retention,
        NoteRetentionPosture::LocalOnlyNotRetainedRemotely
    );
    assert_eq!(note.shared_state, NoteSharedStateVisibility::LocalNotShared);
    assert!(note.is_consistent());
    assert!(!note.audience_visible());
}

#[test]
fn promotion_requires_explicit_acknowledgement() {
    let note = local("note:1", "private prompt");
    let mut request = explicit_request("decision:1");
    request.explicit_acknowledgement = false;
    let err = promote_note_to_shared(&note, request).unwrap_err();
    assert_eq!(err, NotePromotionError::NotExplicitlyAcknowledged);
}

#[test]
fn promotion_requires_a_share_decision_ref() {
    let note = local("note:1", "private prompt");
    let mut request = explicit_request("");
    request.share_decision_ref = "   ".to_owned();
    let err = promote_note_to_shared(&note, request).unwrap_err();
    assert_eq!(err, NotePromotionError::MissingShareDecision);
}

#[test]
fn promotion_records_an_auditable_share() {
    let note = local("note:1", "to share");
    let (shared, record) =
        promote_note_to_shared(&note, explicit_request("decision:1")).expect("explicit promotion");

    assert!(shared.is_shared());
    assert_eq!(shared.note.scope, SpeakerNoteScope::Shared);
    assert!(shared.note.shared_promotion_explicit);
    assert_eq!(
        shared.retention,
        NoteRetentionPosture::SharedRetainedInSessionStore
    );
    assert_eq!(
        shared.shared_state,
        NoteSharedStateVisibility::SharedExplicitlyPromoted
    );
    assert!(shared.is_consistent());
    assert!(shared.audience_visible());

    assert!(record.is_well_formed());
    assert_eq!(record.from_scope, SpeakerNoteScope::Local);
    assert_eq!(record.to_scope, SpeakerNoteScope::Shared);
    assert!(record.explicit_acknowledgement);
    assert!(record.body_remained_unexported);
}

#[test]
fn a_shared_note_cannot_be_promoted_again() {
    let note = local("note:1", "to share");
    let (shared, _) =
        promote_note_to_shared(&note, explicit_request("decision:1")).expect("first promotion");
    let err = promote_note_to_shared(&shared, explicit_request("decision:2")).unwrap_err();
    assert_eq!(err, NotePromotionError::AlreadyShared);
}

#[test]
fn retention_choice_controls_the_recorded_posture() {
    let note = local("note:1", "to share");
    let mut request = explicit_request("decision:1");
    request.retention_choice = SharedRetentionChoice::DisabledByPolicy;
    let (shared, record) = promote_note_to_shared(&note, request).expect("explicit promotion");
    assert_eq!(
        shared.retention,
        NoteRetentionPosture::SharedRetentionDisabledByPolicy
    );
    assert_eq!(
        record.retention,
        NoteRetentionPosture::SharedRetentionDisabledByPolicy
    );
}

#[test]
fn audience_projection_drops_local_notes() {
    let private = local("note:private", "never shown");
    let to_share = local("note:shared", "deliberately shared");
    let (shared, _) =
        promote_note_to_shared(&to_share, explicit_request("decision:1")).expect("promotion");

    let disclosures = project_audience_note_disclosures(&[private, shared]);
    assert_eq!(disclosures.len(), 1);
    assert_eq!(disclosures[0].note_id, "note:shared");
    assert_eq!(disclosures[0].scope, SpeakerNoteScope::Shared);
    assert!(!disclosures.iter().any(|d| d.note_id == "note:private"));
}

#[test]
fn sharing_is_never_inferred_from_co_presence() {
    // The audience disclosure of a session full of viewers is still empty while
    // every note is local: co-presence does not promote a note.
    let notes = vec![local("note:1", "a"), local("note:2", "b")];
    assert!(project_audience_note_disclosures(&notes).is_empty());
}

#[test]
fn citations_must_align_with_the_note_refs() {
    let aligned = local("note:1", "body");
    assert!(aligned.citations_align_with_note());

    // A typed citation that the note does not carry breaks alignment.
    let mut drifting = aligned.clone();
    drifting
        .citations
        .push(NoteCitation::new("doc:orphan", NoteCitationKind::Doc));
    assert!(!drifting.citations_align_with_note());
    assert!(!drifting.is_consistent());
}

#[test]
fn seeded_corpus_validates_and_round_trips() {
    let corpus = seeded_speaker_note_sharing_corpus();
    validate_speaker_note_sharing_corpus(&corpus).expect("seeded corpus must validate");

    let json = serde_json::to_string(&corpus).unwrap();
    let parsed: SpeakerNoteSharingCorpus = serde_json::from_str(&json).unwrap();
    assert_eq!(parsed, corpus);

    assert!(corpus.summary.promotion_demonstrated);
    assert!(corpus.summary.all_notes_consistent);
    assert!(corpus.summary.all_local_notes_default_private);
    assert!(corpus.summary.all_shared_notes_explicitly_promoted);
    assert!(corpus.summary.no_local_note_in_any_audience_disclosure);
    // Every citation kind is exercised by the corpus.
    assert_eq!(corpus.summary.citation_kinds_covered.len(), 4);
}

#[test]
fn support_export_validates_and_excludes_note_bodies() {
    let corpus = seeded_speaker_note_sharing_corpus();
    let export = speaker_note_sharing_support_export(
        "support-export:presentation-speaker-note-sharing:001",
        "2026-06-20T00:00:00Z",
        &corpus,
    );
    assert!(export.validate().is_empty(), "{:?}", export.validate());
    assert!(export.raw_note_bodies_excluded);
    assert!(export.shared_rows_explicitly_promoted);
    assert!(export.no_local_note_audience_visible);
    assert_eq!(export.rows.len() as u32, corpus.summary.note_count);

    // No raw note body or next-step cue ever leaks into the support export.
    let export_json = serde_json::to_string(&export).unwrap();
    for note in corpus.all_notes() {
        if let Some(body) = &note.note.body_label {
            assert!(
                !export_json.contains(body),
                "support export leaked a note body for {}",
                note.note.note_id
            );
        }
    }
    assert!(!export_json.contains("body_label"));
    assert!(!export_json.contains("next_step_cue_label"));
}

#[test]
fn validate_flags_a_local_note_marked_audience_visible() {
    // Hand-build a deliberately corrupt export row to prove validation catches a
    // local note that claims audience visibility.
    let corpus = seeded_speaker_note_sharing_corpus();
    let mut export = speaker_note_sharing_support_export("x", "t", &corpus);
    let local_row = export
        .rows
        .iter_mut()
        .find(|row| row.scope == SpeakerNoteScope::Local)
        .expect("corpus has a local note");
    local_row.audience_visible = true;
    let violations = export.validate();
    assert!(violations.contains(&SpeakerNoteSharingViolation::LocalNoteAudienceVisible));
}

#[test]
fn checked_in_fixtures_match_the_seed_projection() {
    let corpus = seeded_speaker_note_sharing_corpus();
    let fixture = include_str!(
        "../../../../../fixtures/presentation/speaker-note-sharing/speaker-note-sharing-corpus.json"
    );
    let parsed: SpeakerNoteSharingCorpus = serde_json::from_str(fixture).expect("fixture parses");
    assert_eq!(
        parsed, corpus,
        "fixtures/presentation/speaker-note-sharing drifted from the seed corpus; \
         regenerate with the dump_presentation_speaker_note_sharing example"
    );
}
