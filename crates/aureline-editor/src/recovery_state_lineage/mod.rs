//! Durable-state recovery lineage: the editor's governed, export-safe
//! projection that finalizes piece-tree buffer recovery, undo/redo grouping,
//! and local-history actor lineage into one record per recovery posture.
//!
//! Three live truth sources feed this projection, and it ingests each verbatim
//! rather than re-deriving an outcome:
//!
//! 1. **Piece-tree buffer recovery** — the dirty-buffer autosave journal entry
//!    ([`AutosaveJournalEntryRecord`] from `aureline-recovery`) carries the
//!    captured object identity, base-on-disk token, text-format posture,
//!    capture mode, integrity state, and the replay posture that a crash
//!    restore would follow.
//! 2. **Undo/redo grouping** — the buffer journal groups
//!    ([`aureline_buffer::JournalEntry`]) carry the frozen undo class, its
//!    compensation posture, the originator lane, and the human-readable label a
//!    named group must carry. They are observed as [`UndoGroupObservation`]s so
//!    the projection stays serializable for fixtures and replay.
//! 3. **Local-history actor lineage** — the export-safe actor-lineage packet
//!    ([`LocalHistoryAlphaPacket`] from `aureline-history`) carries who made
//!    each change and how it reverses, without raw snapshot bodies.
//!
//! The projection proves the four claims the stable line is anchored on:
//!
//! - **Source fidelity** — the open-time encoding / newline / final-newline
//!   posture can be proven to round-trip on restore.
//! - **Canonical-path truth** — the restore target is the same object the
//!   capture was taken from, or a compare-before-write guard stands between the
//!   restore and any wrong-target write.
//! - **Restore is no-rerun** — a restore applies stored bytes; it never
//!   silently re-runs the actions that produced them, and it never overwrites
//!   live state without a recovery checkpoint.
//! - **Lineage / export honesty** — every actor-lineage row is export-safe and
//!   no raw body leaks into the record.
//!
//! When the projection cannot prove a claim on the captured posture it
//! auto-narrows below Stable with a named [`RecoveryNarrowReason`] instead of
//! inheriting an adjacent green row. Correct protective postures (a
//! compare-before-write guard in place, a restore correctly held for review or
//! blocked, integrity failures that downgrade the replay posture) stay Stable —
//! the contract working as designed is a pass, not a gap.
//!
//! The record excludes raw source, raw snapshot bodies, raw patches, and
//! content-addressed body refs, so it is safe for support export.

use aureline_buffer::{CompensationPosture, JournalEntry};
use aureline_history::{ActorLineageRow, LocalHistoryAlphaPacket};
use aureline_recovery::crash_journal::{
    AutosaveJournalEntryRecord, CaptureMode, DecoderPosture, EncodingLabelClass, GuidedChoiceClass,
    IdentityRelation, NewlineMode, ReplayPostureClass,
};
use serde::{Deserialize, Serialize};

/// Schema version for the recovery-state lineage record.
pub const RECOVERY_STATE_LINEAGE_SCHEMA_VERSION: u32 = 1;

/// Schema reference for the recovery-state lineage record.
pub const RECOVERY_STATE_LINEAGE_SCHEMA_REF: &str =
    "schemas/editor/recovery_state_lineage.schema.json";

/// Stable record-kind tag for the recovery-state lineage record.
pub const RECOVERY_STATE_LINEAGE_RECORD_KIND: &str = "recovery_state_lineage_record";

// ---------------------------------------------------------------------------
// Undo/redo grouping observation.
// ---------------------------------------------------------------------------

/// Compensation posture of an undo group, mirroring the frozen buffer taxonomy.
///
/// A `Compensatable` group's inverse is a forward, legal transaction, so its
/// redo survives a divergent edit and it reverses by replaying the inverse
/// operation. An `OnlyRevertible` group depends on the pre-transaction
/// snapshot, so it reverses by restoring that snapshot — a byte restore, never
/// a re-run.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CompensationPostureClass {
    /// Inverse is a forward transaction; redo survives divergence.
    Compensatable,
    /// Inverse depends on a stored snapshot; divergence drops the redo entry.
    OnlyRevertible,
}

impl CompensationPostureClass {
    /// Returns the stable string vocabulary for this posture.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Compensatable => "compensatable",
            Self::OnlyRevertible => "only_revertible",
        }
    }
}

impl From<CompensationPosture> for CompensationPostureClass {
    fn from(posture: CompensationPosture) -> Self {
        match posture {
            CompensationPosture::Compensatable => Self::Compensatable,
            CompensationPosture::OnlyRevertible => Self::OnlyRevertible,
        }
    }
}

/// How an undo group's effect reverses.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum UndoRecoveryClass {
    /// Reverse by replaying the recorded inverse operations (compensatable).
    InverseReplay,
    /// Reverse by restoring the pre-transaction snapshot (only-revertible).
    SnapshotRestore,
}

impl UndoRecoveryClass {
    /// Returns the stable string vocabulary for this recovery class.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::InverseReplay => "inverse_replay",
            Self::SnapshotRestore => "snapshot_restore",
        }
    }

    const fn for_posture(posture: CompensationPostureClass) -> Self {
        match posture {
            CompensationPostureClass::Compensatable => Self::InverseReplay,
            CompensationPostureClass::OnlyRevertible => Self::SnapshotRestore,
        }
    }
}

/// A serializable observation of one committed undo group.
///
/// This is the projection's serializable mirror of an
/// [`aureline_buffer::JournalEntry`]; the editor populates it from the live
/// buffer journal with [`UndoGroupObservation::from_journal_entry`], and
/// fixtures / replay reconstruct it from JSON.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct UndoGroupObservation {
    /// Monotonic undo-group id from the buffer journal.
    pub undo_group_id: u64,
    /// Frozen undo-class id from the buffer taxonomy.
    pub class_id: String,
    /// Compensation posture for this group.
    pub compensation_posture: CompensationPostureClass,
    /// True when the class opens a named group that must carry a label.
    pub is_named_group: bool,
    /// Stable originator / actor-lane identifier.
    pub originator: String,
    /// Human-readable label; required for named groups.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    /// Number of operations coalesced into this group.
    pub operation_count: usize,
}

impl UndoGroupObservation {
    /// Observes a committed buffer journal entry as a serializable group.
    pub fn from_journal_entry(entry: &JournalEntry<'_>) -> Self {
        Self {
            undo_group_id: entry.undo_group_id().0,
            class_id: entry.class_id().to_owned(),
            compensation_posture: entry.compensation_posture().into(),
            is_named_group: entry.class().is_named_group(),
            originator: entry.originator().to_owned(),
            label: entry.label().map(ToOwned::to_owned),
            operation_count: entry.operation_count(),
        }
    }

    /// Returns true when the group satisfies the grouping-label contract.
    ///
    /// A named group must carry a non-empty human-readable label; any other
    /// class is always satisfied.
    fn grouping_integrity_ok(&self) -> bool {
        if self.is_named_group {
            self.label
                .as_deref()
                .map(|label| !label.trim().is_empty())
                .unwrap_or(false)
        } else {
            true
        }
    }
}

// ---------------------------------------------------------------------------
// Narrow reasons + stable qualification.
// ---------------------------------------------------------------------------

/// Named reason a recovery-state lineage record narrows below Stable.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RecoveryNarrowReason {
    /// The captured encoding / newline posture cannot be proven to round-trip.
    SourceFidelityUnprovable,
    /// The restore target identity drifted with no compare-before-write guard.
    CanonicalPathUnproven,
    /// A restore is recommended but no faithful body exists to restore, so the
    /// restore would have to reconstruct (re-run) the content.
    RestoreWouldRerun,
    /// A restore is recommended but creates no recovery checkpoint, risking
    /// loss of the live state it overwrites.
    DestructiveRestoreNoRecoveryPath,
    /// Frame integrity is unverified yet the replay posture still claims an
    /// unconditional restore.
    RecoveryIntegrityInconsistent,
    /// A named undo group is missing its required human-readable label.
    UndoGroupingContractViolation,
    /// An actor-lineage row or the packet is not export-safe.
    ActorLineageExportUnsafe,
}

impl RecoveryNarrowReason {
    /// Returns the stable string vocabulary for this narrow reason.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SourceFidelityUnprovable => "source_fidelity_unprovable",
            Self::CanonicalPathUnproven => "canonical_path_unproven",
            Self::RestoreWouldRerun => "restore_would_rerun",
            Self::DestructiveRestoreNoRecoveryPath => "destructive_restore_no_recovery_path",
            Self::RecoveryIntegrityInconsistent => "recovery_integrity_inconsistent",
            Self::UndoGroupingContractViolation => "undo_grouping_contract_violation",
            Self::ActorLineageExportUnsafe => "actor_lineage_export_unsafe",
        }
    }
}

/// Stable-qualification posture for a recovery-state lineage record.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RecoveryStableQualification {
    /// Whether the record proves the contract on the claimed posture.
    pub qualified: bool,
    /// Stable lifecycle label: `stable` or `narrowed_below_stable`.
    pub level: String,
    /// Named reasons the record narrowed below Stable, when not qualified.
    pub narrow_reasons: Vec<RecoveryNarrowReason>,
}

// ---------------------------------------------------------------------------
// Projected sub-records.
// ---------------------------------------------------------------------------

/// Piece-tree buffer recovery posture projected from the autosave journal entry.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BufferRecoverySummary {
    /// Autosave journal entry ref this recovery posture is projected from.
    pub journal_entry_ref: String,
    /// Owning autosave journal ref.
    pub journal_ref: String,
    /// Captured object class token.
    pub object_class: String,
    /// Capture mode token (snapshot, delta chain, metadata-only, etc.).
    pub capture_mode: String,
    /// Whether a faithful recoverable body is available locally.
    pub body_available: bool,
    /// Frame-integrity token from the integrity record.
    pub frame_integrity_state: String,
    /// Object-class replay-posture token.
    pub replay_posture_class: String,
    /// Recommended guided-choice token (restore, inspect, discard, etc.).
    pub recommended_choice: String,
    /// Downgrade-reason tokens recorded on the replay posture.
    pub downgrade_reasons: Vec<String>,
    /// Encoding label token captured at journal time.
    pub encoding_label: String,
    /// Newline-mode token captured at journal time.
    pub newline_mode: String,
    /// Final-newline token captured at journal time.
    pub final_newline_state: String,
    /// Decoder-posture token captured at journal time.
    pub decoder_posture: String,
    /// True when frame integrity verified cleanly.
    pub integrity_verified: bool,
    /// True when the open-time representation can be proven to round-trip.
    pub round_trip_provable: bool,
}

/// Canonical-path truth projected from the captured object identity and token.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CanonicalPathTruth {
    /// Identity-relation token between the capture and the current object.
    pub identity_relation: String,
    /// Base-on-disk token-class token.
    pub base_on_disk_token_class: String,
    /// Token-confidence token.
    pub token_confidence: String,
    /// External-change-state token.
    pub external_change_state: String,
    /// Whether the save path compares before writing.
    pub compare_before_write_required: bool,
    /// True when a wrong-target write is structurally guarded.
    pub wrong_target_write_guarded: bool,
}

/// Restore-no-rerun posture projected from the replay posture.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RestoreSafetyPosture {
    /// Whether a restore is recommended or allowed at all.
    pub restore_recommended: bool,
    /// Whether a faithful stored body exists to restore byte-for-byte.
    pub byte_restore_faithful: bool,
    /// Whether a restore creates a new local-history recovery checkpoint.
    pub restore_creates_new_checkpoint: bool,
    /// Whether declining the restore retains the journal (non-destructive).
    pub open_without_replay_retains_journal: bool,
    /// True when a restore is proven to apply stored bytes, never a re-run.
    pub no_rerun_guaranteed: bool,
}

/// One export-safe actor-lineage row projected from the local-history packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ActorLineageSummary {
    /// Stable row id from the source packet.
    pub row_id: String,
    /// Compact display label.
    pub display_label: String,
    /// Protected actor-lineage class token.
    pub actor_lineage_class: String,
    /// Original actor-class token.
    pub actor_class: String,
    /// Original source-class token.
    pub source_class: String,
    /// Original reversal-class token.
    pub reversal_class: String,
    /// Original redaction-class token.
    pub redaction_class: String,
    /// Whether the original checkpoint body is available locally.
    pub body_available_locally: bool,
    /// Checkpoint refs cited by this row (never raw body refs).
    pub checkpoint_refs: Vec<String>,
    /// Canonical command id when the source surface provides one.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub command_id: Option<String>,
    /// True when this row is export-safe (no raw body refs leaked).
    pub export_safe: bool,
}

impl ActorLineageSummary {
    fn from_row(row: &ActorLineageRow) -> Self {
        Self {
            row_id: row.row_id.clone(),
            display_label: row.display_label.clone(),
            actor_lineage_class: row.actor_lineage_class.as_str().to_owned(),
            actor_class: row.actor_class.clone(),
            source_class: row.source_class.clone(),
            reversal_class: row.reversal_class.clone(),
            redaction_class: row.redaction_class.clone(),
            body_available_locally: row.body_available_locally,
            checkpoint_refs: row.checkpoint_refs.clone(),
            command_id: row.command_id.clone(),
            export_safe: row_export_safe(row),
        }
    }
}

// ---------------------------------------------------------------------------
// Top-level record.
// ---------------------------------------------------------------------------

/// Governed, export-safe recovery-state lineage record.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RecoveryStateLineageRecord {
    /// Stable record-kind tag.
    pub record_kind: String,
    /// Schema version.
    pub recovery_state_lineage_schema_version: u32,
    /// Schema reference.
    pub schema_ref: String,
    /// Stable lineage id.
    pub lineage_id: String,
    /// Workspace ref the recovery state belongs to.
    pub workspace_ref: String,
    /// Piece-tree buffer recovery posture.
    pub buffer_recovery: BufferRecoverySummary,
    /// Canonical-path truth for the restore target.
    pub canonical_path_truth: CanonicalPathTruth,
    /// Restore-no-rerun posture.
    pub restore_safety: RestoreSafetyPosture,
    /// Undo/redo grouping lineage rows.
    pub undo_grouping: Vec<UndoGroupLineageEntry>,
    /// Local-history actor-lineage export mode.
    pub actor_lineage_export_mode: String,
    /// Local-history actor-lineage rows.
    pub actor_lineage: Vec<ActorLineageSummary>,
    /// Stable-qualification posture with named narrow reasons.
    pub stable_qualification: RecoveryStableQualification,
    /// Whether support export may include this record without raw bodies.
    pub raw_payload_excluded: bool,
    /// Human-readable summary.
    pub summary: String,
}

/// One projected undo-group row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct UndoGroupLineageEntry {
    /// Monotonic undo-group id.
    pub undo_group_id: u64,
    /// Frozen undo-class id.
    pub class_id: String,
    /// Compensation posture.
    pub compensation_posture: CompensationPostureClass,
    /// Whether this is a named group.
    pub is_named_group: bool,
    /// Originator / actor lane.
    pub originator: String,
    /// Human-readable label, when present.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    /// Coalesced operation count.
    pub operation_count: usize,
    /// Whether the recorded redo survives a divergent edit.
    pub redo_survives_divergence: bool,
    /// How this group reverses.
    pub recovery_action_class: UndoRecoveryClass,
    /// Whether the grouping-label contract is satisfied.
    pub grouping_integrity_ok: bool,
}

impl RecoveryStateLineageRecord {
    /// Returns true when the record is metadata-safe for support export.
    pub fn is_support_export_safe(&self) -> bool {
        self.raw_payload_excluded
            && self.schema_ref == RECOVERY_STATE_LINEAGE_SCHEMA_REF
            && self.record_kind == RECOVERY_STATE_LINEAGE_RECORD_KIND
            && self.actor_lineage.iter().all(|row| row.export_safe)
    }

    /// Returns true when the record proves the contract on the claimed posture.
    pub fn is_stable_qualified(&self) -> bool {
        self.stable_qualification.qualified
    }
}

// ---------------------------------------------------------------------------
// Projection.
// ---------------------------------------------------------------------------

/// Projects a governed recovery-state lineage record from the three live truth
/// sources: the dirty-buffer autosave journal entry, the buffer undo/redo
/// grouping observations, and the local-history actor-lineage packet.
///
/// The projection is deterministic and read-only. It never re-runs a
/// participant, mutates a buffer, or widens authority; it pins the recovery,
/// undo-grouping, and lineage contracts and auto-narrows below Stable with a
/// named reason when a claim cannot be proven on the captured posture.
pub fn project_recovery_state_lineage(
    lineage_id: impl Into<String>,
    buffer_recovery: &AutosaveJournalEntryRecord,
    undo_groups: &[UndoGroupObservation],
    local_history: &LocalHistoryAlphaPacket,
) -> RecoveryStateLineageRecord {
    let recovery = project_buffer_recovery(buffer_recovery);
    let canonical_path_truth = project_canonical_path_truth(buffer_recovery);
    let restore_safety = project_restore_safety(buffer_recovery, &recovery);

    let undo_grouping: Vec<UndoGroupLineageEntry> =
        undo_groups.iter().map(project_undo_group).collect();

    let actor_lineage: Vec<ActorLineageSummary> = local_history
        .actor_lineage_rows
        .iter()
        .map(ActorLineageSummary::from_row)
        .collect();
    let actor_lineage_export_mode = token(&local_history.export_safety.export_mode);
    let lineage_export_safe = local_history.validate().is_ok()
        && actor_lineage.iter().all(|row| row.export_safe)
        && !raw_body_export_enabled(local_history);

    // Evaluate narrow reasons in a fixed order so the record is deterministic.
    let mut narrow_reasons = Vec::new();
    if !recovery.round_trip_provable {
        narrow_reasons.push(RecoveryNarrowReason::SourceFidelityUnprovable);
    }
    if !canonical_path_truth.wrong_target_write_guarded {
        narrow_reasons.push(RecoveryNarrowReason::CanonicalPathUnproven);
    }
    if restore_safety.restore_recommended && !restore_safety.byte_restore_faithful {
        narrow_reasons.push(RecoveryNarrowReason::RestoreWouldRerun);
    }
    if restore_safety.restore_recommended && !restore_safety.restore_creates_new_checkpoint {
        narrow_reasons.push(RecoveryNarrowReason::DestructiveRestoreNoRecoveryPath);
    }
    if !recovery.integrity_verified
        && buffer_recovery.replay_posture.object_class_replay_posture
            == ReplayPostureClass::RestoreAllowed
    {
        narrow_reasons.push(RecoveryNarrowReason::RecoveryIntegrityInconsistent);
    }
    if undo_grouping
        .iter()
        .any(|group| !group.grouping_integrity_ok)
    {
        narrow_reasons.push(RecoveryNarrowReason::UndoGroupingContractViolation);
    }
    if !lineage_export_safe {
        narrow_reasons.push(RecoveryNarrowReason::ActorLineageExportUnsafe);
    }

    let qualified = narrow_reasons.is_empty();
    let stable_qualification = RecoveryStableQualification {
        qualified,
        level: if qualified {
            "stable".to_owned()
        } else {
            "narrowed_below_stable".to_owned()
        },
        narrow_reasons,
    };

    let summary = build_summary(
        &recovery,
        &stable_qualification,
        undo_grouping.len(),
        actor_lineage.len(),
    );

    RecoveryStateLineageRecord {
        record_kind: RECOVERY_STATE_LINEAGE_RECORD_KIND.to_owned(),
        recovery_state_lineage_schema_version: RECOVERY_STATE_LINEAGE_SCHEMA_VERSION,
        schema_ref: RECOVERY_STATE_LINEAGE_SCHEMA_REF.to_owned(),
        lineage_id: lineage_id.into(),
        workspace_ref: buffer_recovery.workspace_ref.clone(),
        buffer_recovery: recovery,
        canonical_path_truth,
        restore_safety,
        undo_grouping,
        actor_lineage_export_mode,
        actor_lineage,
        stable_qualification,
        raw_payload_excluded: true,
        summary,
    }
}

fn project_buffer_recovery(entry: &AutosaveJournalEntryRecord) -> BufferRecoverySummary {
    let integrity_verified = entry.integrity.frame_integrity_state
        == aureline_recovery::crash_journal::FrameIntegrityState::Verified;
    let body_available = entry.capture_descriptor.body_available;
    let round_trip_provable = round_trip_provable(
        entry.text_format.encoding_label,
        entry.text_format.newline_mode,
        entry.text_format.decoder_posture,
    );

    BufferRecoverySummary {
        journal_entry_ref: entry.journal_entry_id.clone(),
        journal_ref: entry.journal_id.clone(),
        object_class: token(&entry.object_identity.object_class),
        capture_mode: token(&entry.capture_descriptor.capture_mode),
        body_available,
        frame_integrity_state: token(&entry.integrity.frame_integrity_state),
        replay_posture_class: token(&entry.replay_posture.object_class_replay_posture),
        recommended_choice: token(&entry.replay_posture.recommended_choice_class),
        downgrade_reasons: entry
            .replay_posture
            .downgrade_reason_classes
            .iter()
            .map(token)
            .collect(),
        encoding_label: token(&entry.text_format.encoding_label),
        newline_mode: token(&entry.text_format.newline_mode),
        final_newline_state: token(&entry.text_format.final_newline_state),
        decoder_posture: token(&entry.text_format.decoder_posture),
        integrity_verified,
        round_trip_provable,
    }
}

fn project_canonical_path_truth(entry: &AutosaveJournalEntryRecord) -> CanonicalPathTruth {
    let compare_before_write_required = entry.base_on_disk_token.compare_before_write_required;
    let wrong_target_write_guarded = wrong_target_guarded(
        entry.object_identity.identity_relation,
        compare_before_write_required,
    );

    CanonicalPathTruth {
        identity_relation: token(&entry.object_identity.identity_relation),
        base_on_disk_token_class: token(&entry.base_on_disk_token.token_class),
        token_confidence: token(&entry.base_on_disk_token.token_confidence),
        external_change_state: token(&entry.base_on_disk_token.external_change_state),
        compare_before_write_required,
        wrong_target_write_guarded,
    }
}

fn project_restore_safety(
    entry: &AutosaveJournalEntryRecord,
    recovery: &BufferRecoverySummary,
) -> RestoreSafetyPosture {
    let restore_recommended = restore_recommended(
        entry.replay_posture.object_class_replay_posture,
        entry.replay_posture.recommended_choice_class,
    );
    let byte_restore_faithful = recovery.body_available
        && matches!(
            entry.capture_descriptor.capture_mode,
            CaptureMode::ContentAddressedSnapshot | CaptureMode::JournalDeltaChain
        );
    let restore_creates_new_checkpoint = entry
        .replay_posture
        .new_local_history_checkpoint_on_restore
        .unwrap_or(false);
    let open_without_replay_retains_journal =
        entry.replay_posture.open_without_replay_retains_journal;

    // A restore is no-rerun safe only when it either does not restore at all
    // (protective inspect/discard/open-without-replay), or it restores faithful
    // stored bytes and creates a recovery checkpoint so live state survives.
    let no_rerun_guaranteed =
        !restore_recommended || (byte_restore_faithful && restore_creates_new_checkpoint);

    RestoreSafetyPosture {
        restore_recommended,
        byte_restore_faithful,
        restore_creates_new_checkpoint,
        open_without_replay_retains_journal,
        no_rerun_guaranteed,
    }
}

fn project_undo_group(observation: &UndoGroupObservation) -> UndoGroupLineageEntry {
    UndoGroupLineageEntry {
        undo_group_id: observation.undo_group_id,
        class_id: observation.class_id.clone(),
        compensation_posture: observation.compensation_posture,
        is_named_group: observation.is_named_group,
        originator: observation.originator.clone(),
        label: observation.label.clone(),
        operation_count: observation.operation_count,
        redo_survives_divergence: observation.compensation_posture
            == CompensationPostureClass::Compensatable,
        recovery_action_class: UndoRecoveryClass::for_posture(observation.compensation_posture),
        grouping_integrity_ok: observation.grouping_integrity_ok(),
    }
}

// ---------------------------------------------------------------------------
// Derivations.
// ---------------------------------------------------------------------------

/// Returns true when the open-time representation can be proven to round-trip
/// on restore. An unknown encoding, unknown newline mode, or a decoder posture
/// that kept no faithful representation cannot be proven byte-faithful.
fn round_trip_provable(
    encoding: EncodingLabelClass,
    newline: NewlineMode,
    decoder: DecoderPosture,
) -> bool {
    if encoding == EncodingLabelClass::Unknown || newline == NewlineMode::Unknown {
        return false;
    }
    matches!(
        decoder,
        DecoderPosture::ExactDecode
            | DecoderPosture::LossyDecodeRawPreserved
            | DecoderPosture::BinarySnapshot
    )
}

/// Returns true when a wrong-target write is structurally guarded.
///
/// Exact identity and virtual-only buffers have no wrong-target risk. Any
/// identity drift (alias drift, same-path-different-object, missing object,
/// unknown identity) is only guarded when the save path compares before write.
fn wrong_target_guarded(relation: IdentityRelation, compare_before_write: bool) -> bool {
    match relation {
        IdentityRelation::ExactObjectIdentity | IdentityRelation::VirtualOnly => true,
        IdentityRelation::AliasCanonicalDrift
        | IdentityRelation::SamePathDifferentObject
        | IdentityRelation::CurrentObjectMissing
        | IdentityRelation::IdentityUnknown => compare_before_write,
    }
}

/// Returns true when a restore is recommended or allowed for this posture.
fn restore_recommended(posture: ReplayPostureClass, choice: GuidedChoiceClass) -> bool {
    matches!(choice, GuidedChoiceClass::Restore)
        || matches!(
            posture,
            ReplayPostureClass::RestoreAllowed | ReplayPostureClass::RestoreRequiresReview
        )
}

fn raw_body_export_enabled(packet: &LocalHistoryAlphaPacket) -> bool {
    packet.export_safety.raw_snapshot_bodies_included
        || packet.export_safety.body_object_refs_included
}

/// Returns true when an actor-lineage row exposes no raw body refs.
fn row_export_safe(row: &ActorLineageRow) -> bool {
    if row.raw_body_refs_exported {
        return false;
    }
    let cited = row
        .local_history_entry_refs
        .iter()
        .chain(row.checkpoint_refs.iter())
        .chain(row.mutation_journal_ref.iter())
        .chain(row.local_history_group_ref.iter());
    !cited
        .into_iter()
        .any(|reference| is_forbidden_export_ref(reference))
}

fn is_forbidden_export_ref(value: &str) -> bool {
    value.starts_with("obj:")
        || value.starts_with("raw:")
        || value.starts_with("secret:")
        || value.starts_with("token:")
}

/// Renders a serde unit-enum variant to its stable snake_case token.
fn token<T: Serialize>(value: &T) -> String {
    serde_json::to_value(value)
        .ok()
        .and_then(|value| value.as_str().map(str::to_owned))
        .unwrap_or_else(|| "unknown".to_owned())
}

fn build_summary(
    recovery: &BufferRecoverySummary,
    qualification: &RecoveryStableQualification,
    undo_groups: usize,
    actor_rows: usize,
) -> String {
    if qualification.qualified {
        format!(
            "Recovery lineage proven Stable: replay {replay}, {undo_groups} undo group(s), \
             {actor_rows} actor-lineage row(s).",
            replay = recovery.replay_posture_class,
        )
    } else {
        let reasons: Vec<&str> = qualification
            .narrow_reasons
            .iter()
            .map(|reason| reason.as_str())
            .collect();
        format!(
            "Recovery lineage narrowed below Stable (replay {replay}): {reasons}.",
            replay = recovery.replay_posture_class,
            reasons = reasons.join(", "),
        )
    }
}

/// Renders the export-safe human-readable lines for a recovery-state lineage
/// record.
///
/// This is the shared projection consumed by the editor recovery-status
/// surface, the headless CLI emitter, Help/About, and support export, so they
/// never clone status text from each other.
pub fn recovery_state_lineage_lines(record: &RecoveryStateLineageRecord) -> Vec<String> {
    let mut lines = Vec::new();
    lines.push(format!(
        "Recovery state lineage — {} ({})",
        record.buffer_recovery.replay_posture_class, record.stable_qualification.level
    ));
    lines.push(format!(
        "workspace={} journal_entry={}",
        record.workspace_ref, record.buffer_recovery.journal_entry_ref
    ));
    lines.push(format!(
        "object={} capture_mode={} body_available={} integrity={} round_trip_provable={}",
        record.buffer_recovery.object_class,
        record.buffer_recovery.capture_mode,
        record.buffer_recovery.body_available,
        record.buffer_recovery.frame_integrity_state,
        record.buffer_recovery.round_trip_provable,
    ));
    lines.push(format!(
        "encoding={} newline={} final_newline={} decoder={}",
        record.buffer_recovery.encoding_label,
        record.buffer_recovery.newline_mode,
        record.buffer_recovery.final_newline_state,
        record.buffer_recovery.decoder_posture,
    ));
    lines.push(format!(
        "identity={} compare_before_write={} wrong_target_guarded={}",
        record.canonical_path_truth.identity_relation,
        record.canonical_path_truth.compare_before_write_required,
        record.canonical_path_truth.wrong_target_write_guarded,
    ));
    lines.push(format!(
        "restore_recommended={} byte_restore_faithful={} creates_checkpoint={} no_rerun_guaranteed={}",
        record.restore_safety.restore_recommended,
        record.restore_safety.byte_restore_faithful,
        record.restore_safety.restore_creates_new_checkpoint,
        record.restore_safety.no_rerun_guaranteed,
    ));

    lines.push("Undo groups:".to_owned());
    for group in &record.undo_grouping {
        lines.push(format!(
            "  #{id} [{class}] {originator} — {label} | posture={posture} reverses={recovery} redo_survives={redo} ops={ops} grouping_ok={ok}",
            id = group.undo_group_id,
            class = group.class_id,
            originator = group.originator,
            label = group.label.as_deref().unwrap_or("(none)"),
            posture = group.compensation_posture.as_str(),
            recovery = group.recovery_action_class.as_str(),
            redo = group.redo_survives_divergence,
            ops = group.operation_count,
            ok = group.grouping_integrity_ok,
        ));
    }

    lines.push(format!(
        "Actor lineage ({} mode):",
        record.actor_lineage_export_mode
    ));
    for row in &record.actor_lineage {
        lines.push(format!(
            "  {id} [{class}] {label} — actor={actor} source={source} reversal={reversal} export_safe={safe}",
            id = row.row_id,
            class = row.actor_lineage_class,
            label = row.display_label,
            actor = row.actor_class,
            source = row.source_class,
            reversal = row.reversal_class,
            safe = row.export_safe,
        ));
    }

    if !record.stable_qualification.qualified {
        let reasons: Vec<&str> = record
            .stable_qualification
            .narrow_reasons
            .iter()
            .map(|reason| reason.as_str())
            .collect();
        lines.push(format!("Narrowed below Stable: {}", reasons.join(", ")));
    }

    lines.push(record.summary.clone());
    lines
}

#[cfg(test)]
mod tests;
