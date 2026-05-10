use std::fs::{create_dir_all, OpenOptions};
use std::io::Write;
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

use serde::{Deserialize, Serialize};

use super::records::{
    ActorClass, ActorSurfaceRecord, AutosaveJournalEntryRecord, BaseOnDiskTokenRecord,
    CaptureClass, CaptureDescriptorRecord, CaptureMode, CaptureOmissionReason, ChecksumAlgorithm,
    DecoderPosture, EncodingLabelClass, ExternalChangeState, FinalNewlineState,
    FrameIntegrityState, GuidedChoiceClass, IdentityRelation, IntegrityRecord, NewlineMode,
    ObjectClass, ObjectIdentityRecord, ReplayIntegrityPosture, ReplayPostureClass,
    ReplayPostureRecord, RetentionClass, RetentionPostureRecord, SourceClass, SupportBundleInclusionState,
    SupportExportRecord, SurfaceClass, TextFormatRecord, TokenClass, TokenConfidenceClass,
    DowngradeReasonClass,
};

/// Error returned when crash-journal persistence fails.
#[derive(Debug)]
pub enum CrashJournalError {
    Io(std::io::Error),
    Json(serde_json::Error),
    MissingBody(String),
}

impl std::fmt::Display for CrashJournalError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(err) => write!(f, "crash journal io error: {err}"),
            Self::Json(err) => write!(f, "crash journal json error: {err}"),
            Self::MissingBody(detail) => write!(f, "crash journal body missing: {detail}"),
        }
    }
}

impl std::error::Error for CrashJournalError {}

impl From<std::io::Error> for CrashJournalError {
    fn from(value: std::io::Error) -> Self {
        Self::Io(value)
    }
}

impl From<serde_json::Error> for CrashJournalError {
    fn from(value: serde_json::Error) -> Self {
        Self::Json(value)
    }
}

/// Identifier source used by crash-journal stores.
#[derive(Debug, Clone)]
pub struct IdSource {
    prefix: &'static str,
    next_seq: u64,
}

impl IdSource {
    /// Creates a new id source with a stable prefix.
    pub const fn new(prefix: &'static str) -> Self {
        Self {
            prefix,
            next_seq: 1,
        }
    }

    /// Mints a new opaque id.
    pub fn mint(&mut self) -> String {
        let seq = self.next_seq;
        self.next_seq = self.next_seq.saturating_add(1);
        let stamp = unix_nanos();
        format!("{prefix}-{stamp:020}-{seq:06}", prefix = self.prefix)
    }
}

fn unix_nanos() -> u128 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_nanos()
}

/// Prototype store for autosave journal entries and body objects.
#[derive(Debug, Clone)]
pub struct CrashJournalStore {
    root: PathBuf,
    entry_ids: IdSource,
}

/// Input for capturing one minimal autosave-journal entry.
#[derive(Debug, Clone)]
pub struct CrashJournalCaptureInput {
    /// Journal identity grouping entries for one workspace authority.
    pub journal_id: String,
    /// Opaque workspace ref carried on the journal entry.
    pub workspace_ref: String,
    /// Stable logical-document id for the captured buffer.
    pub logical_document_id: String,
    /// Stable object ref used for restore identity and dedupe.
    pub object_ref: String,
    /// Object-class vocabulary for the captured buffer.
    pub object_class: ObjectClass,
    /// Optional display hint such as a basename.
    pub presentation_hint: Option<String>,
    /// Producer-local monotonic timestamp for the capture.
    pub emitted_at: String,
    /// Buffer bytes captured for recovery replay.
    pub bytes: Vec<u8>,
}

impl CrashJournalStore {
    /// Creates a crash-journal store rooted at `root_dir/crash_journal`.
    pub fn new(root_dir: impl AsRef<Path>) -> Self {
        let root = root_dir.as_ref().join("crash_journal");
        Self {
            root,
            entry_ids: IdSource::new("j"),
        }
    }

    /// Returns the on-disk root path for the crash journal store.
    pub fn root_path(&self) -> &Path {
        &self.root
    }

    /// Mints a new `journal_entry_id` value suitable for autosave records.
    pub fn mint_journal_entry_id(&mut self) -> String {
        self.entry_ids.mint()
    }

    /// Writes a content-addressed body object and returns its opaque ref + checksum hex.
    pub fn write_body_object(&self, bytes: &[u8]) -> Result<(String, String), CrashJournalError> {
        let checksum = blake3::hash(bytes).to_hex().to_string();
        let object_ref = format!("journal-body:{checksum}");
        let path = self.root.join("bodies").join(format!("{checksum}.bin"));
        write_content_addressed_blob(&path, bytes)?;
        Ok((object_ref, checksum))
    }

    /// Persists an autosave journal entry record.
    pub fn write_entry(&self, entry: &AutosaveJournalEntryRecord) -> Result<PathBuf, CrashJournalError> {
        let path = self
            .root
            .join("entries")
            .join(format!("{}.json", sanitize_id(&entry.journal_entry_id)));
        write_new_json(&path, entry)?;
        Ok(path)
    }

    /// Captures a minimal full-buffer snapshot entry and persists it.
    pub fn capture_minimal_full_snapshot(
        &mut self,
        input: CrashJournalCaptureInput,
    ) -> Result<AutosaveJournalEntryRecord, CrashJournalError> {
        let journal_entry_id = self.mint_journal_entry_id();
        let (body_object_ref, checksum_hex) = self.write_body_object(&input.bytes)?;

        let object_identity = ObjectIdentityRecord {
            logical_document_id: input.logical_document_id,
            object_ref: input.object_ref,
            object_class: input.object_class,
            presentation_hint: input.presentation_hint,
            filesystem_identity_ref: None,
            canonical_identity_ref: None,
            branch_worktree_ref: None,
            identity_relation: IdentityRelation::IdentityUnknown,
            identity_notes: "captured from live buffer".to_string(),
        };

        let base_on_disk_token = BaseOnDiskTokenRecord {
            token_class: TokenClass::MissingOrNotApplicable,
            token_ref: None,
            observed_revision_ref: None,
            token_confidence: TokenConfidenceClass::Unknown,
            compare_before_write_required: false,
            external_change_state: ExternalChangeState::ExternalChangeUnknown,
        };

        let text_format = TextFormatRecord {
            encoding_label: EncodingLabelClass::Utf8,
            bom_policy: "utf8_default".to_string(),
            newline_mode: NewlineMode::Unknown,
            decoder_posture: DecoderPosture::ExactDecode,
            final_newline_state: FinalNewlineState::MixedOrUnknown,
            large_file_mode: false,
            format_notes: "buffer snapshot".to_string(),
        };

        let actor_surface = ActorSurfaceRecord {
            actor_class: ActorClass::CrashRecoveryJournal,
            source_class: SourceClass::MachineLocal,
            surface_class: SurfaceClass::SavePipeline,
            command_ref: None,
            session_ref: None,
            actor_display: "autosave_journal".to_string(),
        };

        let capture_descriptor = CaptureDescriptorRecord {
            capture_class: CaptureClass::FullBufferSnapshot,
            capture_mode: CaptureMode::ContentAddressedSnapshot,
            body_available: true,
            body_object_refs: vec![body_object_ref],
            dirty_range_summary_ref: None,
            group_member_refs: Vec::new(),
            omission_reason: CaptureOmissionReason::NotOmitted,
            capture_notes: "full buffer snapshot".to_string(),
        };

        let integrity = IntegrityRecord {
            checksum_algorithm: ChecksumAlgorithm::Blake3,
            checksum_ref: checksum_hex,
            frame_integrity_state: FrameIntegrityState::Verified,
            replay_integrity_posture: ReplayIntegrityPosture::ReplayAllowed,
            last_good_frame_ref: None,
            failed_frame_ref: None,
            corruption_evidence_refs: Vec::new(),
            integrity_notes: "body stored content-addressed".to_string(),
        };

        let replay_posture = ReplayPostureRecord {
            object_class_replay_posture: ReplayPostureClass::RestoreRequiresReview,
            recommended_choice_class: GuidedChoiceClass::InspectOnly,
            blocked_choice_classes: vec![GuidedChoiceClass::Restore],
            downgrade_reason_classes: vec![DowngradeReasonClass::NotDowngraded],
            new_local_history_checkpoint_on_restore: None,
            new_checkpoint_ref: None,
            open_without_replay_retains_journal: true,
            replay_notes: "prototype capture: restore requires explicit review surface".to_string(),
        };

        let retention_posture = RetentionPostureRecord {
            retention_class: RetentionClass::ActiveReplayWindow,
            local_only_default: true,
            ordinary_cache_clear_excluded: true,
            settings_reset_excluded: true,
            local_history_clear_excluded: true,
            journal_reset_required_for_delete: true,
            export_before_reset: "export_optional_redaction_applied".to_string(),
            expiry_policy_ref: None,
            pin_refs: Vec::new(),
        };

        let support_export = SupportExportRecord {
            support_bundle_inclusion_state: SupportBundleInclusionState::MetadataRefAllowed,
            redaction_class: "metadata_safe_default".to_string(),
            support_export_refs: Vec::new(),
            export_notes: "body excluded from support export by default".to_string(),
        };

        let entry = AutosaveJournalEntryRecord::new(
            journal_entry_id,
            input.journal_id,
            input.workspace_ref,
            object_identity,
            base_on_disk_token,
            text_format,
            actor_surface,
            capture_descriptor,
            integrity,
            replay_posture,
            retention_posture,
            support_export,
            input.emitted_at,
        );

        let _ = self.write_entry(&entry)?;
        Ok(entry)
    }

    /// Loads all autosave journal entries currently present on disk.
    pub fn load_entries(&self) -> Result<Vec<AutosaveJournalEntryRecord>, CrashJournalError> {
        let entries_dir = self.root.join("entries");
        let mut out = Vec::new();
        let Ok(dir) = std::fs::read_dir(&entries_dir) else {
            return Ok(out);
        };
        for entry in dir {
            let entry = match entry {
                Ok(value) => value,
                Err(_) => continue,
            };
            let path = entry.path();
            if path.extension().and_then(|s| s.to_str()) != Some("json") {
                continue;
            }
            let bytes = match std::fs::read(&path) {
                Ok(bytes) => bytes,
                Err(_) => continue,
            };
            let record: AutosaveJournalEntryRecord = match serde_json::from_slice(&bytes) {
                Ok(record) => record,
                Err(_) => continue,
            };
            out.push(record);
        }
        out.sort_by(|a, b| a.emitted_at.cmp(&b.emitted_at));
        Ok(out)
    }

    /// Reads a body object referenced by a `journal-body:<checksum>` ref.
    pub fn read_body_object(&self, body_object_ref: &str) -> Result<Vec<u8>, CrashJournalError> {
        let checksum = body_object_ref
            .strip_prefix("journal-body:")
            .ok_or_else(|| CrashJournalError::MissingBody(body_object_ref.to_string()))?;
        let path = self.root.join("bodies").join(format!("{checksum}.bin"));
        let bytes = std::fs::read(&path)
            .map_err(|_| CrashJournalError::MissingBody(body_object_ref.to_string()))?;
        Ok(bytes)
    }
}

fn sanitize_id(value: &str) -> String {
    value
        .chars()
        .map(|ch| match ch {
            ':' | '/' | '\\' | ' ' | '\t' | '\n' | '\r' => '_',
            other => other,
        })
        .collect()
}

fn write_new_json<T: Serialize>(path: &Path, value: &T) -> Result<(), CrashJournalError> {
    if let Some(parent) = path.parent() {
        create_dir_all(parent)?;
    }
    let json = serde_json::to_string_pretty(value)?;
    write_new_file(path, json.as_bytes())?;
    Ok(())
}

fn write_new_blob(path: &Path, bytes: &[u8]) -> Result<(), CrashJournalError> {
    if let Some(parent) = path.parent() {
        create_dir_all(parent)?;
    }
    write_new_file(path, bytes)?;
    Ok(())
}

fn write_new_file(path: &Path, bytes: &[u8]) -> Result<(), CrashJournalError> {
    let mut file = OpenOptions::new().write(true).create_new(true).open(path)?;
    file.write_all(bytes)?;
    file.sync_all()?;
    Ok(())
}

fn write_content_addressed_blob(path: &Path, bytes: &[u8]) -> Result<(), CrashJournalError> {
    if let Some(parent) = path.parent() {
        create_dir_all(parent)?;
    }
    match OpenOptions::new().write(true).create_new(true).open(path) {
        Ok(mut file) => {
            file.write_all(bytes)?;
            file.sync_all()?;
            Ok(())
        }
        Err(err) if err.kind() == std::io::ErrorKind::AlreadyExists => Ok(()),
        Err(err) => Err(CrashJournalError::Io(err)),
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
struct StoreProbeRecord {
    record_kind: String,
    emitted_at: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn writes_body_objects_content_addressed() {
        let dir = tempfile::tempdir().expect("tempdir");
        let store = CrashJournalStore::new(dir.path());
        let (ref1, checksum1) = store.write_body_object(b"hello").expect("write body");
        let (ref2, checksum2) = store.write_body_object(b"hello").expect("write body");
        assert_eq!(ref1, ref2);
        assert_eq!(checksum1, checksum2);
    }

    #[test]
    fn writes_entry_as_new_file() {
        let dir = tempfile::tempdir().expect("tempdir");
        let mut store = CrashJournalStore::new(dir.path());
        let record = StoreProbeRecord {
            record_kind: "probe".to_string(),
            emitted_at: "mono:0".to_string(),
        };
        let path = store
            .root
            .join("entries")
            .join(format!("{}.json", store.mint_journal_entry_id()));
        write_new_json(&path, &record).expect("write json");
        let second = write_new_json(&path, &record);
        assert!(
            second.is_err(),
            "expected create_new write to refuse overwriting existing entry"
        );
    }
}
