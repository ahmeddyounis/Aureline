//! Seeded speaker-note sharing corpus, support export, and validation.
//!
//! Each case is a set of [`GovernedSpeakerNote`]s plus the
//! [`SpeakerNoteShareRecord`]s and [`AudienceNoteDisclosure`]s they imply. The
//! checked-in fixtures under `fixtures/presentation/speaker-note-sharing/` are a
//! literal projection of [`seeded_speaker_note_sharing_corpus`], so the JSON
//! cannot drift from the Rust types.
//!
//! The corpus deliberately covers a solo rehearsal where every note stays
//! local-only (with citations to a file, symbol, doc, and graph object), a
//! shared-workspace session where exactly one note is explicitly promoted while
//! the rest stay private, and a policy-constrained share where retention is
//! disabled — so local-default privacy, explicit promotion, retention posture,
//! and audience non-leakage are proven across scenarios rather than asserted.

use std::collections::BTreeSet;

use serde::{Deserialize, Serialize};

use crate::presentation_mode::{
    SpeakerNote, SpeakerNoteScope, PRESENTATION_MODE_BETA_SCHEMA_VERSION,
    PRESENTATION_MODE_BETA_SHARED_CONTRACT_REF,
};

use super::sharing::{
    project_audience_note_disclosures, promote_note_to_shared, AudienceNoteDisclosure,
    GovernedSpeakerNote, NoteCitation, NoteCitationKind, NotePromotionRequest,
    SharedRetentionChoice, SpeakerNoteShareRecord, SpeakerNoteSharingExport,
};

/// Stable record kind for [`SpeakerNoteSharingCase`] payloads.
pub const SPEAKER_NOTE_SHARING_CASE_RECORD_KIND: &str = "presentation_speaker_note_sharing_case";

/// Stable record kind for [`SpeakerNoteSharingCorpus`] payloads.
pub const SPEAKER_NOTE_SHARING_CORPUS_RECORD_KIND: &str =
    "presentation_speaker_note_sharing_corpus";

/// One seeded scenario: the governed notes plus the share records and audience
/// disclosures they imply.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SpeakerNoteSharingCase {
    /// Record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Shared contract ref.
    pub shared_contract_ref: String,
    /// Stable case id.
    pub case_id: String,
    /// Human-readable scenario label.
    pub scenario_label: String,
    /// The governed notes in this scenario.
    pub notes: Vec<GovernedSpeakerNote>,
    /// Share records for the notes promoted in this scenario.
    pub share_records: Vec<SpeakerNoteShareRecord>,
    /// What an audience / follower surface may see — shared notes only.
    pub audience_disclosures: Vec<AudienceNoteDisclosure>,
}

/// Aggregate coverage summary for the corpus.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SpeakerNoteSharingSummary {
    /// Number of cases.
    pub case_count: u32,
    /// Total governed notes across the corpus.
    pub note_count: u32,
    /// Local-only notes across the corpus.
    pub local_note_count: u32,
    /// Shared notes across the corpus.
    pub shared_note_count: u32,
    /// Distinct citation kinds preserved across the corpus.
    pub citation_kinds_covered: Vec<NoteCitationKind>,
    /// True when every governed note is consistent with its scope.
    pub all_notes_consistent: bool,
    /// True when every local note keeps the default-private posture.
    pub all_local_notes_default_private: bool,
    /// True when every shared note carries an explicit promotion marker.
    pub all_shared_notes_explicitly_promoted: bool,
    /// True when no audience disclosure across the corpus references a local note.
    pub no_local_note_in_any_audience_disclosure: bool,
    /// True when at least one case demonstrates an explicit promotion.
    pub promotion_demonstrated: bool,
}

/// The full seeded speaker-note sharing corpus.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SpeakerNoteSharingCorpus {
    /// Record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Shared contract ref.
    pub shared_contract_ref: String,
    /// Mint timestamp.
    pub generated_at: String,
    /// Coverage summary.
    pub summary: SpeakerNoteSharingSummary,
    /// Per-scenario cases.
    pub cases: Vec<SpeakerNoteSharingCase>,
}

impl SpeakerNoteSharingCorpus {
    /// Every governed note across the corpus, in case order.
    pub fn all_notes(&self) -> Vec<GovernedSpeakerNote> {
        self.cases
            .iter()
            .flat_map(|case| case.notes.iter().cloned())
            .collect()
    }
}

/// Errors emitted by [`validate_speaker_note_sharing_corpus`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SpeakerNoteSharingCorpusError {
    /// The corpus carried the wrong record kind or schema version.
    MalformedCorpus,
    /// A case carried the wrong record kind or schema version.
    MalformedCase {
        /// The offending case id.
        case_id: String,
    },
    /// A governed note's posture is inconsistent with its scope.
    NoteInconsistent {
        /// The offending case id.
        case_id: String,
        /// The offending note id.
        note_id: String,
    },
    /// A share record is malformed.
    ShareRecordMalformed {
        /// The offending case id.
        case_id: String,
        /// The offending note id.
        note_id: String,
    },
    /// A case's audience disclosures did not match the projection of its notes,
    /// or referenced a local note.
    AudienceDisclosureMismatch {
        /// The offending case id.
        case_id: String,
    },
    /// The summary did not match the cases it claims to summarize.
    SummaryMismatch,
    /// No case demonstrated an explicit promotion.
    PromotionNotDemonstrated,
}

/// Validate the seeded speaker-note sharing corpus.
pub fn validate_speaker_note_sharing_corpus(
    corpus: &SpeakerNoteSharingCorpus,
) -> Result<(), SpeakerNoteSharingCorpusError> {
    if corpus.record_kind != SPEAKER_NOTE_SHARING_CORPUS_RECORD_KIND
        || corpus.schema_version != PRESENTATION_MODE_BETA_SCHEMA_VERSION
    {
        return Err(SpeakerNoteSharingCorpusError::MalformedCorpus);
    }

    for case in &corpus.cases {
        if case.record_kind != SPEAKER_NOTE_SHARING_CASE_RECORD_KIND
            || case.schema_version != PRESENTATION_MODE_BETA_SCHEMA_VERSION
        {
            return Err(SpeakerNoteSharingCorpusError::MalformedCase {
                case_id: case.case_id.clone(),
            });
        }

        for note in &case.notes {
            if !note.is_consistent() {
                return Err(SpeakerNoteSharingCorpusError::NoteInconsistent {
                    case_id: case.case_id.clone(),
                    note_id: note.note.note_id.clone(),
                });
            }
        }

        for record in &case.share_records {
            let backs_shared_note = case.notes.iter().any(|note| {
                note.note.note_id == record.note_id && note.note.scope == SpeakerNoteScope::Shared
            });
            if !record.is_well_formed() || !backs_shared_note {
                return Err(SpeakerNoteSharingCorpusError::ShareRecordMalformed {
                    case_id: case.case_id.clone(),
                    note_id: record.note_id.clone(),
                });
            }
        }

        let expected_disclosures = project_audience_note_disclosures(&case.notes);
        let references_local_note = case.audience_disclosures.iter().any(|disclosure| {
            case.notes.iter().any(|note| {
                note.note.note_id == disclosure.note_id
                    && note.note.scope == SpeakerNoteScope::Local
            })
        });
        if case.audience_disclosures != expected_disclosures || references_local_note {
            return Err(SpeakerNoteSharingCorpusError::AudienceDisclosureMismatch {
                case_id: case.case_id.clone(),
            });
        }
    }

    let expected = summarize(&corpus.cases);
    if expected != corpus.summary {
        return Err(SpeakerNoteSharingCorpusError::SummaryMismatch);
    }
    if !corpus.summary.promotion_demonstrated {
        return Err(SpeakerNoteSharingCorpusError::PromotionNotDemonstrated);
    }
    Ok(())
}

/// Project a corpus into a support-safe export over every governed note.
pub fn speaker_note_sharing_support_export(
    export_id: impl Into<String>,
    generated_at: impl Into<String>,
    corpus: &SpeakerNoteSharingCorpus,
) -> SpeakerNoteSharingExport {
    SpeakerNoteSharingExport::from_notes(export_id, generated_at, &corpus.all_notes())
}

fn summarize(cases: &[SpeakerNoteSharingCase]) -> SpeakerNoteSharingSummary {
    let mut note_count = 0u32;
    let mut local_note_count = 0u32;
    let mut shared_note_count = 0u32;
    let mut citation_kinds: BTreeSet<NoteCitationKind> = BTreeSet::new();
    let mut all_notes_consistent = true;
    let mut all_local_notes_default_private = true;
    let mut all_shared_notes_explicitly_promoted = true;
    let mut no_local_note_in_any_audience_disclosure = true;
    let mut promotion_demonstrated = false;

    for case in cases {
        for note in &case.notes {
            note_count += 1;
            all_notes_consistent &= note.is_consistent();
            for citation in &note.citations {
                citation_kinds.insert(citation.kind);
            }
            match note.note.scope {
                SpeakerNoteScope::Local => {
                    local_note_count += 1;
                    if note.note.shared_promotion_explicit
                        || note.shared_state.is_shared()
                        || note.audience_visible()
                    {
                        all_local_notes_default_private = false;
                    }
                }
                SpeakerNoteScope::Shared => {
                    shared_note_count += 1;
                    if !note.note.shared_promotion_explicit {
                        all_shared_notes_explicitly_promoted = false;
                    }
                }
            }
        }
        if !case.share_records.is_empty() {
            promotion_demonstrated = true;
        }
        for disclosure in &case.audience_disclosures {
            let is_local = case.notes.iter().any(|note| {
                note.note.note_id == disclosure.note_id
                    && note.note.scope == SpeakerNoteScope::Local
            });
            if is_local {
                no_local_note_in_any_audience_disclosure = false;
            }
        }
    }

    SpeakerNoteSharingSummary {
        case_count: cases.len() as u32,
        note_count,
        local_note_count,
        shared_note_count,
        citation_kinds_covered: citation_kinds.into_iter().collect(),
        all_notes_consistent,
        all_local_notes_default_private,
        all_shared_notes_explicitly_promoted,
        no_local_note_in_any_audience_disclosure,
        promotion_demonstrated,
    }
}

// ---- builders -------------------------------------------------------------

fn local_note(
    note_id: &str,
    waypoint: &str,
    body: &str,
    citations: Vec<NoteCitation>,
) -> GovernedSpeakerNote {
    let refs: Vec<String> = citations.iter().map(|c| c.target_ref.clone()).collect();
    let note = SpeakerNote::local(note_id, waypoint, body).with_citations(refs);
    GovernedSpeakerNote::local(note, citations)
}

fn promote(
    governed: &GovernedSpeakerNote,
    retention: SharedRetentionChoice,
    decision_ref: &str,
) -> (GovernedSpeakerNote, SpeakerNoteShareRecord) {
    promote_note_to_shared(
        governed,
        NotePromotionRequest {
            explicit_acknowledgement: true,
            retention_choice: retention,
            share_decision_ref: decision_ref.to_owned(),
            promoted_at: "2026-06-20T09:30:00Z".to_owned(),
        },
    )
    .expect("seed promotion is explicit and well-formed")
}

fn case(
    case_id: &str,
    scenario: &str,
    notes: Vec<GovernedSpeakerNote>,
    share_records: Vec<SpeakerNoteShareRecord>,
) -> SpeakerNoteSharingCase {
    let audience_disclosures = project_audience_note_disclosures(&notes);
    SpeakerNoteSharingCase {
        record_kind: SPEAKER_NOTE_SHARING_CASE_RECORD_KIND.to_owned(),
        schema_version: PRESENTATION_MODE_BETA_SCHEMA_VERSION,
        shared_contract_ref: PRESENTATION_MODE_BETA_SHARED_CONTRACT_REF.to_owned(),
        case_id: case_id.to_owned(),
        scenario_label: scenario.to_owned(),
        notes,
        share_records,
        audience_disclosures,
    }
}

fn solo_rehearsal_case() -> SpeakerNoteSharingCase {
    let notes = vec![
        local_note(
            "presentation.note.solo.0001",
            "presentation.waypoint.solo.0001",
            "Open on the entry point; keep this beat short.",
            vec![
                NoteCitation::new(
                    "file:crates/aureline-shell/src/lib.rs",
                    NoteCitationKind::File,
                ),
                NoteCitation::new("symbol:fn run", NoteCitationKind::Symbol),
            ],
        ),
        local_note(
            "presentation.note.solo.0002",
            "presentation.waypoint.solo.0002",
            "Tie the design doc to the topology view before moving on.",
            vec![
                NoteCitation::new(
                    "doc:docs/ux/presentation-and-walkthrough-truth.md",
                    NoteCitationKind::Doc,
                ),
                NoteCitation::new("graph:topology:presentation_lane", NoteCitationKind::Graph),
            ],
        ),
    ];
    // No audience and no promotion: everything stays local-only.
    case(
        "speaker-note-case:solo-rehearsal-local-notes",
        "Solo rehearsal: every note stays local-only, with citations to a file, \
         a symbol, a doc, and a graph object. Nothing is shared and there is no \
         audience to leak to.",
        notes,
        Vec::new(),
    )
}

fn shared_workspace_case() -> SpeakerNoteSharingCase {
    let private = local_note(
        "presentation.note.shared.0001",
        "presentation.waypoint.shared.0001",
        "Private aside: skip the tangent if we are running long.",
        vec![NoteCitation::new(
            "file:crates/aureline-shell/src/presentation/binding.rs",
            NoteCitationKind::File,
        )],
    );
    let to_share = local_note(
        "presentation.note.shared.0002",
        "presentation.waypoint.shared.0002",
        "Shared prompt: this is the contract the audience should remember.",
        vec![NoteCitation::new(
            "doc:docs/privacy/presentation-speaker-notes.md",
            NoteCitationKind::Doc,
        )],
    );
    let (shared, record) = promote(
        &to_share,
        SharedRetentionChoice::RetainInSessionStore,
        "share-decision:presentation:shared:0002",
    );
    case(
        "speaker-note-case:shared-workspace-one-promoted",
        "Shared workspace: one note is explicitly promoted and retained in the \
         session store, while a private aside stays local. Only the promoted \
         note reaches the audience disclosure; the private aside cannot.",
        vec![private, shared],
        vec![record],
    )
}

fn policy_retention_disabled_case() -> SpeakerNoteSharingCase {
    let to_share = local_note(
        "presentation.note.policy.0001",
        "presentation.waypoint.policy.0001",
        "Shared prompt under a no-retention policy: delivered live only.",
        vec![NoteCitation::new(
            "symbol:struct SpeakerNoteShareRecord",
            NoteCitationKind::Symbol,
        )],
    );
    let (shared, record) = promote(
        &to_share,
        SharedRetentionChoice::DisabledByPolicy,
        "share-decision:presentation:policy:0001",
    );
    case(
        "speaker-note-case:policy-retention-disabled-share",
        "Policy-constrained share: the note is explicitly promoted but retention \
         is disabled by policy, so the shared-state and retention posture is \
         visible as deliver-live-only and never persisted.",
        vec![shared],
        vec![record],
    )
}

/// Build the full seeded speaker-note sharing corpus.
pub fn seeded_speaker_note_sharing_corpus() -> SpeakerNoteSharingCorpus {
    let cases = vec![
        solo_rehearsal_case(),
        shared_workspace_case(),
        policy_retention_disabled_case(),
    ];
    let summary = summarize(&cases);
    SpeakerNoteSharingCorpus {
        record_kind: SPEAKER_NOTE_SHARING_CORPUS_RECORD_KIND.to_owned(),
        schema_version: PRESENTATION_MODE_BETA_SCHEMA_VERSION,
        shared_contract_ref: PRESENTATION_MODE_BETA_SHARED_CONTRACT_REF.to_owned(),
        generated_at: "2026-06-20T00:00:00Z".to_owned(),
        summary,
        cases,
    }
}
