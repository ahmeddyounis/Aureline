//! The speaker-note sharing governance model: local-only defaults, explicit
//! promotion, retention / export / shared-state posture, audience projection,
//! and the support-safe diagnostics export.
//!
//! The canonical [`SpeakerNote`] boundary object already lives in
//! [`crate::presentation_mode`]: a presenter-only prompt attached to one
//! waypoint that defaults to [`SpeakerNoteScope::Local`] and only becomes shared
//! through an explicit, separately recorded promotion. This module is the thin
//! governance layer the spec calls for. It wraps that object in a
//! [`GovernedSpeakerNote`] that makes the consequences of scope *visible and
//! auditable*:
//!
//! - typed [`NoteCitation`]s preserve the file / symbol / doc / graph objects a
//!   note points at, instead of an opaque ref list;
//! - a [`NoteRetentionPosture`], [`NoteSharedStateVisibility`], and the absolute
//!   [`NoteBodyExportPosture`] surface where a note is kept and how it may leave
//!   local-only state — never inferred, always recorded;
//! - [`promote_note_to_shared`] is the *only* path from local to shared, and it
//!   requires an explicit acknowledgement and emits a reopenable
//!   [`SpeakerNoteShareRecord`]; sharing is never inferred from follow state or
//!   co-presence;
//! - [`project_audience_note_disclosures`] proves an audience / follower surface
//!   can only ever see a deliberately shared note — a local note is dropped by
//!   construction, so it can never leak;
//! - [`SpeakerNoteSharingExport`] is the support / diagnostics projection: it
//!   records scope and posture honestly but carries **no note body**, so a raw
//!   presenter prompt cannot escape through a support packet.
//!
//! The boundary schema for the support export is
//! [`schemas/presentation/speaker-note-export.schema.json`](../../../../../schemas/presentation/speaker-note-export.schema.json);
//! the privacy contract is `docs/privacy/presentation-speaker-notes.md`.

use serde::{Deserialize, Serialize};

pub use crate::presentation_mode::{SpeakerNote, SpeakerNoteScope};
use crate::presentation_mode::{
    PRESENTATION_MODE_BETA_SCHEMA_VERSION, PRESENTATION_MODE_BETA_SHARED_CONTRACT_REF,
};

/// Stable record kind for [`SpeakerNoteShareRecord`] payloads.
pub const SPEAKER_NOTE_SHARE_RECORD_KIND: &str = "presentation_speaker_note_share_record";

/// Stable record kind for [`SpeakerNoteSharingDiagnosticsRow`] payloads.
pub const SPEAKER_NOTE_SHARING_DIAGNOSTICS_ROW_RECORD_KIND: &str =
    "presentation_speaker_note_sharing_diagnostics_row";

/// Stable record kind for [`SpeakerNoteSharingExport`] payloads.
pub const SPEAKER_NOTE_SHARING_EXPORT_RECORD_KIND: &str =
    "presentation_speaker_note_sharing_export";

/// Repo-relative path of the support-export boundary schema this lane mirrors.
pub const SPEAKER_NOTE_EXPORT_SCHEMA_REF: &str =
    "schemas/presentation/speaker-note-export.schema.json";

/// Repo-relative path of the human-readable speaker-note privacy contract.
pub const SPEAKER_NOTE_PRIVACY_DOC_REF: &str = "docs/privacy/presentation-speaker-notes.md";

/// The kind of object a speaker-note citation points at. Preserved so a note
/// keeps its links to the file, symbol, doc, or graph object it was written
/// against, rather than an untyped ref.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum NoteCitationKind {
    /// A file in the workspace.
    File,
    /// A symbol anchor within a file.
    Symbol,
    /// A docs / knowledge object.
    Doc,
    /// A topology / dependency graph object.
    Graph,
}

impl NoteCitationKind {
    /// Every citation kind, in declaration order.
    pub const ALL: [Self; 4] = [Self::File, Self::Symbol, Self::Doc, Self::Graph];

    /// Stable token recorded in records and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::File => "file",
            Self::Symbol => "symbol",
            Self::Doc => "doc",
            Self::Graph => "graph",
        }
    }
}

/// One typed citation a speaker note preserves: a stable target ref plus the
/// kind of object it addresses. The `target_ref` is a stable id, never a file
/// body or symbol source.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NoteCitation {
    /// Stable id of the cited object (also present in the note's `citation_refs`).
    pub target_ref: String,
    /// The kind of object the citation addresses.
    pub kind: NoteCitationKind,
}

impl NoteCitation {
    /// Builds a typed citation.
    pub fn new(target_ref: impl Into<String>, kind: NoteCitationKind) -> Self {
        Self {
            target_ref: target_ref.into(),
            kind,
        }
    }
}

/// Where a note is retained, made visible whenever a note leaves local-only
/// state. A local note is never retained off the machine; a shared note records
/// the store it is retained in (or that policy disabled retention).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum NoteRetentionPosture {
    /// Local-only: the note never leaves the local machine and is not retained
    /// in any shared store.
    LocalOnlyNotRetainedRemotely,
    /// Shared and retained in the optional shared-session store for the session
    /// lifetime, then dropped.
    SharedRetainedInSessionStore,
    /// Shared but retention is disabled by policy: the note is delivered live and
    /// never persisted.
    SharedRetentionDisabledByPolicy,
}

impl NoteRetentionPosture {
    /// Stable token recorded in records and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LocalOnlyNotRetainedRemotely => "local_only_not_retained_remotely",
            Self::SharedRetainedInSessionStore => "shared_retained_in_session_store",
            Self::SharedRetentionDisabledByPolicy => "shared_retention_disabled_by_policy",
        }
    }

    /// Whether this posture keeps the note strictly local.
    pub const fn is_local_only(self) -> bool {
        matches!(self, Self::LocalOnlyNotRetainedRemotely)
    }
}

/// Whether a note is in shared state, and that the share was explicit. A note is
/// only ever [`Self::SharedExplicitlyPromoted`] through [`promote_note_to_shared`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum NoteSharedStateVisibility {
    /// The note is local and not shared with anyone.
    LocalNotShared,
    /// The note was explicitly promoted to a shared scope and is auditable.
    SharedExplicitlyPromoted,
}

impl NoteSharedStateVisibility {
    /// Stable token recorded in records and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LocalNotShared => "local_not_shared",
            Self::SharedExplicitlyPromoted => "shared_explicitly_promoted",
        }
    }

    /// Whether the note is currently shared.
    pub const fn is_shared(self) -> bool {
        matches!(self, Self::SharedExplicitlyPromoted)
    }
}

/// The export posture of a note body. There is exactly one honest value: a raw
/// note body never enters a support, diagnostics, or telemetry export. The enum
/// keeps the absolute invariant inspectable in every record rather than implied.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum NoteBodyExportPosture {
    /// The raw note body is never exported; only its presence and posture are.
    BodyNeverExported,
}

impl NoteBodyExportPosture {
    /// Stable token recorded in records and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::BodyNeverExported => "body_never_exported",
        }
    }
}

/// A governed speaker note: the canonical [`SpeakerNote`] plus the visible,
/// auditable posture that governs where it may go. The posture is consistent
/// with the note's scope by construction (see [`GovernedSpeakerNote::is_consistent`]).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GovernedSpeakerNote {
    /// The canonical boundary note object.
    pub note: SpeakerNote,
    /// Typed citations the note preserves (a typed view of `note.citation_refs`).
    pub citations: Vec<NoteCitation>,
    /// Where the note is retained.
    pub retention: NoteRetentionPosture,
    /// Whether the note is in shared state.
    pub shared_state: NoteSharedStateVisibility,
    /// The note-body export posture (always [`NoteBodyExportPosture::BodyNeverExported`]).
    pub body_export_posture: NoteBodyExportPosture,
}

impl GovernedSpeakerNote {
    /// Builds a local-only governed note. Local is the default for every note;
    /// the retention, shared-state, and export postures are fixed to their
    /// private values so a fresh note cannot start out shared.
    pub fn local(note: SpeakerNote, citations: Vec<NoteCitation>) -> Self {
        Self {
            note: SpeakerNote {
                scope: SpeakerNoteScope::Local,
                shared_promotion_explicit: false,
                ..note
            },
            citations,
            retention: NoteRetentionPosture::LocalOnlyNotRetainedRemotely,
            shared_state: NoteSharedStateVisibility::LocalNotShared,
            body_export_posture: NoteBodyExportPosture::BodyNeverExported,
        }
    }

    /// Whether the note is local-only.
    pub fn is_local(&self) -> bool {
        self.note.scope.is_local_only()
    }

    /// Whether the note is in a deliberately shared state.
    pub fn is_shared(&self) -> bool {
        !self.is_local()
    }

    /// Whether the note carries a presenter-facing body.
    pub fn has_body(&self) -> bool {
        self.note.has_body()
    }

    /// Whether the typed citations align with the note's `citation_refs`: each
    /// typed citation references a ref the note carries, no ref is cited twice,
    /// and every ref on the note is typed. Keeps the typed view a faithful
    /// projection of the canonical refs rather than a second source of truth.
    pub fn citations_align_with_note(&self) -> bool {
        if self.citations.len() != self.note.citation_refs.len() {
            return false;
        }
        let mut seen = std::collections::BTreeSet::new();
        for citation in &self.citations {
            if !self.note.citation_refs.contains(&citation.target_ref) {
                return false;
            }
            if !seen.insert(citation.target_ref.as_str()) {
                return false;
            }
        }
        true
    }

    /// Whether a note that has left local-only state surfaces its disclosure
    /// posture honestly, and a local note keeps the private posture. This is the
    /// core "retention / export / shared-state posture is visible" invariant:
    ///
    /// - a [`SpeakerNoteScope::Local`] note carries no promotion marker, a
    ///   local-only retention posture, and the not-shared visibility;
    /// - a [`SpeakerNoteScope::Shared`] note carries the explicit promotion
    ///   marker, a shared retention posture, and the shared visibility.
    ///
    /// Either way the body-export posture stays [`NoteBodyExportPosture::BodyNeverExported`].
    pub fn is_consistent(&self) -> bool {
        if self.body_export_posture != NoteBodyExportPosture::BodyNeverExported {
            return false;
        }
        if !self.citations_align_with_note() {
            return false;
        }
        match self.note.scope {
            SpeakerNoteScope::Local => {
                !self.note.shared_promotion_explicit
                    && self.retention.is_local_only()
                    && self.shared_state == NoteSharedStateVisibility::LocalNotShared
            }
            SpeakerNoteScope::Shared => {
                self.note.shared_promotion_explicit
                    && !self.retention.is_local_only()
                    && self.shared_state == NoteSharedStateVisibility::SharedExplicitlyPromoted
            }
        }
    }

    /// Whether this note may surface on an audience / follower surface. Only a
    /// deliberately shared note is eligible; a local note is never audience
    /// visible, so it cannot leak through a follower surface.
    pub fn audience_visible(&self) -> bool {
        self.is_shared() && self.shared_state.is_shared()
    }
}

/// The retention to record when promoting a note to shared state.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SharedRetentionChoice {
    /// Retain the shared note in the session store for the session lifetime.
    RetainInSessionStore,
    /// Do not persist the shared note; deliver it live only.
    DisabledByPolicy,
}

impl SharedRetentionChoice {
    /// The retention posture this choice records on the promoted note.
    pub const fn retention_posture(self) -> NoteRetentionPosture {
        match self {
            Self::RetainInSessionStore => NoteRetentionPosture::SharedRetainedInSessionStore,
            Self::DisabledByPolicy => NoteRetentionPosture::SharedRetentionDisabledByPolicy,
        }
    }
}

/// An explicit request to promote a local note to shared state. Sharing is never
/// inferred: the request must carry an explicit acknowledgement and a reopenable
/// share-decision ref. It deliberately takes no follow-state or co-presence
/// input, so co-presence can never imply a share.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NotePromotionRequest {
    /// The presenter explicitly acknowledged the note will leave local-only
    /// state. Must be `true` or the promotion fails closed.
    pub explicit_acknowledgement: bool,
    /// The retention to record on the shared note.
    pub retention_choice: SharedRetentionChoice,
    /// Reopenable ref of the share decision (e.g. the confirmation event).
    pub share_decision_ref: String,
    /// RFC 3339 timestamp the promotion was recorded.
    pub promoted_at: String,
}

/// A reopenable record of an explicit local → shared promotion. It makes the
/// share decision auditable without ever carrying the note body.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SpeakerNoteShareRecord {
    /// Record kind; must equal [`SPEAKER_NOTE_SHARE_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Shared contract ref.
    pub shared_contract_ref: String,
    /// The promoted note's id.
    pub note_id: String,
    /// The waypoint the note is attached to.
    pub linked_waypoint_ref: String,
    /// Scope before the promotion (always [`SpeakerNoteScope::Local`]).
    pub from_scope: SpeakerNoteScope,
    /// Scope after the promotion (always [`SpeakerNoteScope::Shared`]).
    pub to_scope: SpeakerNoteScope,
    /// The retention recorded for the shared note.
    pub retention: NoteRetentionPosture,
    /// Always `true`: the promotion was explicitly acknowledged.
    pub explicit_acknowledgement: bool,
    /// Reopenable ref of the share decision.
    pub share_decision_ref: String,
    /// Always `true`: the body did not enter any export as part of the share.
    pub body_remained_unexported: bool,
    /// RFC 3339 timestamp the promotion was recorded.
    pub promoted_at: String,
}

impl SpeakerNoteShareRecord {
    /// Whether the record is well-formed: it carries the canonical kind, a
    /// local → shared transition, an explicit acknowledgement, the unexported
    /// guarantee, and a non-empty decision ref and timestamp.
    pub fn is_well_formed(&self) -> bool {
        self.record_kind == SPEAKER_NOTE_SHARE_RECORD_KIND
            && self.schema_version == PRESENTATION_MODE_BETA_SCHEMA_VERSION
            && self.from_scope == SpeakerNoteScope::Local
            && self.to_scope == SpeakerNoteScope::Shared
            && self.explicit_acknowledgement
            && self.body_remained_unexported
            && !self.note_id.trim().is_empty()
            && !self.linked_waypoint_ref.trim().is_empty()
            && !self.share_decision_ref.trim().is_empty()
            && !self.promoted_at.trim().is_empty()
    }
}

/// Why a promotion was refused. The promotion path fails closed: an
/// unacknowledged or malformed request never produces a shared note.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NotePromotionError {
    /// The request did not carry an explicit acknowledgement.
    NotExplicitlyAcknowledged,
    /// The note is already shared; there is nothing to promote.
    AlreadyShared,
    /// The share-decision ref or timestamp was empty.
    MissingShareDecision,
}

impl NotePromotionError {
    /// Stable token used in tests and diagnostics.
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::NotExplicitlyAcknowledged => "not_explicitly_acknowledged",
            Self::AlreadyShared => "already_shared",
            Self::MissingShareDecision => "missing_share_decision",
        }
    }
}

/// Promotes a local note to shared state. This is the only path from local to
/// shared. It fails closed unless the request carries an explicit acknowledgement
/// and a reopenable share decision, and it emits a [`SpeakerNoteShareRecord`]
/// alongside the promoted note so the share stays auditable. Sharing is never
/// inferred from follow state or co-presence — the request carries no such input.
pub fn promote_note_to_shared(
    governed: &GovernedSpeakerNote,
    request: NotePromotionRequest,
) -> Result<(GovernedSpeakerNote, SpeakerNoteShareRecord), NotePromotionError> {
    if governed.is_shared() {
        return Err(NotePromotionError::AlreadyShared);
    }
    if !request.explicit_acknowledgement {
        return Err(NotePromotionError::NotExplicitlyAcknowledged);
    }
    if request.share_decision_ref.trim().is_empty() || request.promoted_at.trim().is_empty() {
        return Err(NotePromotionError::MissingShareDecision);
    }

    let retention = request.retention_choice.retention_posture();
    let promoted = GovernedSpeakerNote {
        note: SpeakerNote {
            scope: SpeakerNoteScope::Shared,
            shared_promotion_explicit: true,
            ..governed.note.clone()
        },
        citations: governed.citations.clone(),
        retention,
        shared_state: NoteSharedStateVisibility::SharedExplicitlyPromoted,
        body_export_posture: NoteBodyExportPosture::BodyNeverExported,
    };
    let record = SpeakerNoteShareRecord {
        record_kind: SPEAKER_NOTE_SHARE_RECORD_KIND.to_owned(),
        schema_version: PRESENTATION_MODE_BETA_SCHEMA_VERSION,
        shared_contract_ref: PRESENTATION_MODE_BETA_SHARED_CONTRACT_REF.to_owned(),
        note_id: governed.note.note_id.clone(),
        linked_waypoint_ref: governed.note.linked_waypoint_ref.clone(),
        from_scope: SpeakerNoteScope::Local,
        to_scope: SpeakerNoteScope::Shared,
        retention,
        explicit_acknowledgement: true,
        share_decision_ref: request.share_decision_ref,
        body_remained_unexported: true,
        promoted_at: request.promoted_at,
    };
    Ok((promoted, record))
}

/// What an audience / follower surface is allowed to learn about a note. Only a
/// shared note ever produces a disclosure, and even then the body text is not
/// carried here: the renderer reads a shared body from the canonical note, while
/// this metadata record stays export-safe. A local note produces no disclosure.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AudienceNoteDisclosure {
    /// The shared note's id.
    pub note_id: String,
    /// The waypoint the note is attached to.
    pub linked_waypoint_ref: String,
    /// Scope of the disclosed note; always [`SpeakerNoteScope::Shared`].
    pub scope: SpeakerNoteScope,
    /// Whether a shared body is available to the audience for this note.
    pub shared_body_available_to_audience: bool,
}

/// Projects the notes an audience / follower surface may see. Only deliberately
/// shared notes are included; local notes are dropped by construction, so a
/// private note can never render on a follower surface. The result carries no
/// note body, only export-safe metadata.
pub fn project_audience_note_disclosures(
    notes: &[GovernedSpeakerNote],
) -> Vec<AudienceNoteDisclosure> {
    notes
        .iter()
        .filter(|governed| governed.audience_visible())
        .map(|governed| AudienceNoteDisclosure {
            note_id: governed.note.note_id.clone(),
            linked_waypoint_ref: governed.note.linked_waypoint_ref.clone(),
            scope: governed.note.scope,
            shared_body_available_to_audience: governed.has_body(),
        })
        .collect()
}

/// One support-safe diagnostics row for a governed note. Carries the note's id,
/// waypoint, scope, posture, citation kinds, and presence flags — never the body
/// or any citation source. Diagnostics and support packets read these rows so a
/// note's scope is recorded honestly without ever leaking the prompt.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SpeakerNoteSharingDiagnosticsRow {
    /// Record kind; must equal [`SPEAKER_NOTE_SHARING_DIAGNOSTICS_ROW_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Shared contract ref.
    pub shared_contract_ref: String,
    /// The note's id.
    pub note_id: String,
    /// The waypoint the note is attached to.
    pub linked_waypoint_ref: String,
    /// The note's scope.
    pub scope: SpeakerNoteScope,
    /// Whether the note carries a body (presence only — never the text).
    pub has_body: bool,
    /// Whether the share was explicit (always `false` for a local note).
    pub shared_promotion_explicit: bool,
    /// The note's retention posture.
    pub retention: NoteRetentionPosture,
    /// The note's shared-state visibility.
    pub shared_state: NoteSharedStateVisibility,
    /// The note-body export posture (always body-never-exported).
    pub body_export_posture: NoteBodyExportPosture,
    /// Number of typed citations the note preserves.
    pub citation_count: u32,
    /// Distinct citation kinds the note preserves.
    pub citation_kinds: Vec<NoteCitationKind>,
    /// Whether this note is eligible to surface on an audience / follower surface.
    pub audience_visible: bool,
}

impl SpeakerNoteSharingDiagnosticsRow {
    /// Projects a governed note into a support-safe diagnostics row.
    pub fn from_governed(governed: &GovernedSpeakerNote) -> Self {
        let mut kinds: Vec<NoteCitationKind> = governed.citations.iter().map(|c| c.kind).collect();
        kinds.sort();
        kinds.dedup();
        Self {
            record_kind: SPEAKER_NOTE_SHARING_DIAGNOSTICS_ROW_RECORD_KIND.to_owned(),
            schema_version: PRESENTATION_MODE_BETA_SCHEMA_VERSION,
            shared_contract_ref: PRESENTATION_MODE_BETA_SHARED_CONTRACT_REF.to_owned(),
            note_id: governed.note.note_id.clone(),
            linked_waypoint_ref: governed.note.linked_waypoint_ref.clone(),
            scope: governed.note.scope,
            has_body: governed.has_body(),
            shared_promotion_explicit: governed.note.shared_promotion_explicit,
            retention: governed.retention,
            shared_state: governed.shared_state,
            body_export_posture: governed.body_export_posture,
            citation_count: governed.citations.len() as u32,
            citation_kinds: kinds,
            audience_visible: governed.audience_visible(),
        }
    }
}

/// The support / diagnostics export for speaker-note sharing posture: one row per
/// governed note, plus the guardrail flags that make the export auditable. Raw
/// note bodies and citation sources never cross this boundary, so the packet is
/// safe for support, diagnostics, and telemetry surfaces.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SpeakerNoteSharingExport {
    /// Record kind; must equal [`SPEAKER_NOTE_SHARING_EXPORT_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Shared contract ref.
    pub shared_contract_ref: String,
    /// Stable export id.
    pub export_id: String,
    /// RFC 3339 mint timestamp.
    pub generated_at: String,
    /// Support-safe per-note rows.
    pub rows: Vec<SpeakerNoteSharingDiagnosticsRow>,
    /// Always `true`: raw note bodies are excluded from this export.
    pub raw_note_bodies_excluded: bool,
    /// Always `true`: every shared row carries an explicit promotion marker.
    pub shared_rows_explicitly_promoted: bool,
    /// Always `true`: no local note is reported as audience visible.
    pub no_local_note_audience_visible: bool,
}

impl SpeakerNoteSharingExport {
    /// Projects governed notes into a support-safe export.
    pub fn from_notes(
        export_id: impl Into<String>,
        generated_at: impl Into<String>,
        notes: &[GovernedSpeakerNote],
    ) -> Self {
        let rows: Vec<SpeakerNoteSharingDiagnosticsRow> = notes
            .iter()
            .map(SpeakerNoteSharingDiagnosticsRow::from_governed)
            .collect();
        let shared_rows_explicitly_promoted = rows
            .iter()
            .filter(|row| row.scope == SpeakerNoteScope::Shared)
            .all(|row| row.shared_promotion_explicit);
        let no_local_note_audience_visible = rows
            .iter()
            .filter(|row| row.scope == SpeakerNoteScope::Local)
            .all(|row| !row.audience_visible);
        Self {
            record_kind: SPEAKER_NOTE_SHARING_EXPORT_RECORD_KIND.to_owned(),
            schema_version: PRESENTATION_MODE_BETA_SCHEMA_VERSION,
            shared_contract_ref: PRESENTATION_MODE_BETA_SHARED_CONTRACT_REF.to_owned(),
            export_id: export_id.into(),
            generated_at: generated_at.into(),
            rows,
            raw_note_bodies_excluded: true,
            shared_rows_explicitly_promoted,
            no_local_note_audience_visible,
        }
    }

    /// Validates the export's privacy and structural invariants.
    pub fn validate(&self) -> Vec<SpeakerNoteSharingViolation> {
        let mut violations = Vec::new();
        if self.record_kind != SPEAKER_NOTE_SHARING_EXPORT_RECORD_KIND {
            violations.push(SpeakerNoteSharingViolation::WrongRecordKind);
        }
        if self.schema_version != PRESENTATION_MODE_BETA_SCHEMA_VERSION {
            violations.push(SpeakerNoteSharingViolation::WrongSchemaVersion);
        }
        if self.export_id.trim().is_empty() || self.generated_at.trim().is_empty() {
            violations.push(SpeakerNoteSharingViolation::MissingIdentity);
        }
        if !self.raw_note_bodies_excluded {
            violations.push(SpeakerNoteSharingViolation::RawBodiesNotExcluded);
        }

        for row in &self.rows {
            if row.body_export_posture != NoteBodyExportPosture::BodyNeverExported {
                violations.push(SpeakerNoteSharingViolation::RowBodyExportable);
            }
            match row.scope {
                SpeakerNoteScope::Local => {
                    if row.shared_promotion_explicit
                        || !row.retention.is_local_only()
                        || row.shared_state != NoteSharedStateVisibility::LocalNotShared
                    {
                        violations.push(SpeakerNoteSharingViolation::LocalRowPostureInconsistent);
                    }
                    if row.audience_visible {
                        violations.push(SpeakerNoteSharingViolation::LocalNoteAudienceVisible);
                    }
                }
                SpeakerNoteScope::Shared => {
                    if !row.shared_promotion_explicit
                        || row.retention.is_local_only()
                        || row.shared_state != NoteSharedStateVisibility::SharedExplicitlyPromoted
                    {
                        violations.push(SpeakerNoteSharingViolation::SharedRowNotExplicit);
                    }
                }
            }
        }

        if json_contains_note_body(
            &serde_json::to_value(self).expect("speaker-note sharing export serializes"),
        ) {
            violations.push(SpeakerNoteSharingViolation::RawBoundaryMaterialInExport);
        }
        violations
    }

    /// Deterministic export-safe JSON.
    ///
    /// # Panics
    ///
    /// Panics only if serializing this metadata-only packet fails.
    pub fn export_safe_json(&self) -> String {
        serde_json::to_string_pretty(self).expect("speaker-note sharing export serializes")
    }
}

/// Validation failures emitted by [`SpeakerNoteSharingExport::validate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SpeakerNoteSharingViolation {
    /// Export record kind is wrong.
    WrongRecordKind,
    /// Export schema version is wrong.
    WrongSchemaVersion,
    /// A required identity field is missing.
    MissingIdentity,
    /// The raw-bodies-excluded flag is not set.
    RawBodiesNotExcluded,
    /// A row claims a body could be exported.
    RowBodyExportable,
    /// A local row carries an inconsistent (non-private) posture.
    LocalRowPostureInconsistent,
    /// A local note is reported as audience visible.
    LocalNoteAudienceVisible,
    /// A shared row lacks an explicit promotion / shared posture.
    SharedRowNotExplicit,
    /// The export contains a forbidden raw note body field.
    RawBoundaryMaterialInExport,
}

impl SpeakerNoteSharingViolation {
    /// Stable token used in tests and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WrongRecordKind => "wrong_record_kind",
            Self::WrongSchemaVersion => "wrong_schema_version",
            Self::MissingIdentity => "missing_identity",
            Self::RawBodiesNotExcluded => "raw_bodies_not_excluded",
            Self::RowBodyExportable => "row_body_exportable",
            Self::LocalRowPostureInconsistent => "local_row_posture_inconsistent",
            Self::LocalNoteAudienceVisible => "local_note_audience_visible",
            Self::SharedRowNotExplicit => "shared_row_not_explicit",
            Self::RawBoundaryMaterialInExport => "raw_boundary_material_in_export",
        }
    }
}

/// Whether a serialized export carries a forbidden note-body field. A support
/// export must never carry `body_label` or `next_step_cue_label`; the
/// metadata-only rows carry presence flags instead.
fn json_contains_note_body(value: &serde_json::Value) -> bool {
    match value {
        serde_json::Value::Object(map) => {
            if map.contains_key("body_label") || map.contains_key("next_step_cue_label") {
                return true;
            }
            map.values().any(json_contains_note_body)
        }
        serde_json::Value::Array(items) => items.iter().any(json_contains_note_body),
        _ => false,
    }
}
