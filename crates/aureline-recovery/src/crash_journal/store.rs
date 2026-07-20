use std::fs::{self, OpenOptions};
use std::io::{Read, Write};
use std::path::{Component, Path, PathBuf};
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::{SystemTime, UNIX_EPOCH};

use serde::{Deserialize, Serialize};

use super::records::{
    ActorClass, ActorSurfaceRecord, AutosaveJournalEntryRecord, BaseOnDiskTokenRecord,
    CaptureClass, CaptureDescriptorRecord, CaptureMode, CaptureOmissionReason, ChecksumAlgorithm,
    DecoderPosture, DowngradeReasonClass, EncodingLabelClass, ExternalChangeState,
    FinalNewlineState, FrameIntegrityState, GuidedChoiceClass, IdentityRelation, IntegrityRecord,
    NewlineMode, ObjectClass, ObjectIdentityRecord, ReplayIntegrityPosture, ReplayPostureClass,
    ReplayPostureRecord, RetentionClass, RetentionPostureRecord, SourceClass,
    SupportBundleInclusionState, SupportExportRecord, SurfaceClass, TextFormatRecord, TokenClass,
    TokenConfidenceClass,
};

const MAX_JOURNAL_RECORD_BYTES: usize = 2 * 1024 * 1024;
const MAX_JOURNAL_BODY_BYTES: usize = 64 * 1024 * 1024;
const MAX_JOURNAL_ENTRY_FILES: usize = 4_096;
const MAX_BODY_REFS_PER_ENTRY: usize = 16;
const MAX_FILENAME_ID_BYTES: usize = 160;
const MAX_OPAQUE_REF_BYTES: usize = 1_024;
const MAX_PRESENTATION_HINT_BYTES: usize = 256;

static TEMP_FILE_SEQUENCE: AtomicU64 = AtomicU64::new(1);

/// Error returned when crash-journal persistence fails.
#[derive(Debug)]
pub enum CrashJournalError {
    Io(std::io::Error),
    Json(serde_json::Error),
    MissingBody(String),
    InvalidInput(&'static str),
    TooLarge(&'static str),
    Integrity(&'static str),
    CorruptRecord(&'static str),
}

impl std::fmt::Display for CrashJournalError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(err) => write!(f, "crash journal io error: {err}"),
            Self::Json(err) => write!(f, "crash journal json error: {err}"),
            Self::MissingBody(detail) => write!(f, "crash journal body missing: {detail}"),
            Self::InvalidInput(detail) => write!(f, "crash journal input rejected: {detail}"),
            Self::TooLarge(detail) => write!(f, "crash journal storage bound exceeded: {detail}"),
            Self::Integrity(detail) => write!(f, "crash journal integrity failure: {detail}"),
            Self::CorruptRecord(detail) => write!(f, "crash journal record corrupt: {detail}"),
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
        if bytes.len() > MAX_JOURNAL_BODY_BYTES {
            return Err(CrashJournalError::TooLarge("dirty-buffer body object"));
        }
        let checksum = blake3::hash(bytes).to_hex().to_string();
        let object_ref = format!("journal-body:{checksum}");
        let path = self.root.join("bodies").join(format!("{checksum}.bin"));
        write_content_addressed_blob(&self.root, &path, bytes)?;
        Ok((object_ref, checksum))
    }

    /// Persists an autosave journal entry record.
    pub fn write_entry(
        &self,
        entry: &AutosaveJournalEntryRecord,
    ) -> Result<PathBuf, CrashJournalError> {
        validate_entry_for_storage(entry)?;
        if matches!(
            entry.integrity.frame_integrity_state,
            FrameIntegrityState::Verified
        ) {
            for body_ref in &entry.capture_descriptor.body_object_refs {
                let _ = self.read_body_object(body_ref)?;
            }
        }
        let path = self
            .root
            .join("entries")
            .join(format!("{}.json", entry.journal_entry_id));
        write_new_json(&self.root, &path, entry)?;
        Ok(path)
    }

    /// Captures a minimal full-buffer snapshot entry and persists it.
    pub fn capture_minimal_full_snapshot(
        &mut self,
        input: CrashJournalCaptureInput,
    ) -> Result<AutosaveJournalEntryRecord, CrashJournalError> {
        validate_capture_input(&input)?;
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
        self.load_entries_inner(None)
    }

    /// Loads entries bound to one exact opaque workspace authority.
    ///
    /// The requested authority is validated before any disk access. Filtering
    /// uses the validated `workspace_ref` embedded in each joined record; file
    /// names are never treated as workspace authority.
    pub fn load_entries_for_workspace(
        &self,
        workspace_ref: &str,
    ) -> Result<Vec<AutosaveJournalEntryRecord>, CrashJournalError> {
        validate_opaque_ref(workspace_ref, "workspace ref")?;
        self.load_entries_inner(Some(workspace_ref))
    }

    fn load_entries_inner(
        &self,
        workspace_ref: Option<&str>,
    ) -> Result<Vec<AutosaveJournalEntryRecord>, CrashJournalError> {
        let entries_dir = self.root.join("entries");
        let metadata = match fs::symlink_metadata(&entries_dir) {
            Ok(metadata) => metadata,
            Err(err) if err.kind() == std::io::ErrorKind::NotFound => return Ok(Vec::new()),
            Err(err) => return Err(CrashJournalError::Io(err)),
        };
        if metadata.file_type().is_symlink() || !metadata.is_dir() {
            return Err(CrashJournalError::CorruptRecord(
                "entries root is not a regular directory",
            ));
        }

        let mut paths = Vec::new();
        for entry in fs::read_dir(&entries_dir)? {
            let path = entry?.path();
            if path.extension().and_then(|s| s.to_str()) != Some("json") {
                continue;
            }
            paths.push(path);
            if paths.len() > MAX_JOURNAL_ENTRY_FILES {
                return Err(CrashJournalError::TooLarge("journal entry file count"));
            }
        }
        paths.sort();

        let mut out = Vec::with_capacity(paths.len());
        for path in paths {
            let bytes = read_bounded_regular_file(&path, MAX_JOURNAL_RECORD_BYTES).map_err(
                |err| match err {
                    CrashJournalError::Io(inner)
                        if inner.kind() == std::io::ErrorKind::NotFound =>
                    {
                        CrashJournalError::CorruptRecord("journal entry disappeared during read")
                    }
                    other => other,
                },
            )?;
            let record: AutosaveJournalEntryRecord = serde_json::from_slice(&bytes)
                .map_err(|_| CrashJournalError::CorruptRecord("journal entry JSON is invalid"))?;
            validate_entry_for_storage(&record)?;
            let expected_name = format!("{}.json", record.journal_entry_id);
            if path.file_name().and_then(|name| name.to_str()) != Some(expected_name.as_str()) {
                return Err(CrashJournalError::CorruptRecord(
                    "journal entry filename does not match embedded id",
                ));
            }
            if workspace_ref.map_or(true, |expected| record.workspace_ref == expected) {
                out.push(record);
            }
        }
        out.sort_by(|a, b| {
            a.emitted_at
                .cmp(&b.emitted_at)
                .then_with(|| a.journal_entry_id.cmp(&b.journal_entry_id))
        });
        Ok(out)
    }

    /// Reads a body object referenced by a `journal-body:<checksum>` ref.
    pub fn read_body_object(&self, body_object_ref: &str) -> Result<Vec<u8>, CrashJournalError> {
        let checksum = parse_body_object_ref(body_object_ref)?;
        let path = self.root.join("bodies").join(format!("{checksum}.bin"));
        let bytes =
            read_bounded_regular_file(&path, MAX_JOURNAL_BODY_BYTES).map_err(|err| match err {
                CrashJournalError::Io(inner) if inner.kind() == std::io::ErrorKind::NotFound => {
                    CrashJournalError::MissingBody("referenced object is unavailable".to_string())
                }
                other => other,
            })?;
        let actual = blake3::hash(&bytes).to_hex().to_string();
        if actual != checksum {
            return Err(CrashJournalError::Integrity(
                "dirty-buffer body digest does not match object ref",
            ));
        }
        Ok(bytes)
    }
}

fn validate_filename_id(value: &str) -> Result<(), CrashJournalError> {
    if value.is_empty()
        || value.len() > MAX_FILENAME_ID_BYTES
        || value == "."
        || value == ".."
        || !value
            .bytes()
            .all(|byte| byte.is_ascii_alphanumeric() || matches!(byte, b'-' | b'_'))
    {
        return Err(CrashJournalError::InvalidInput(
            "journal entry id is not a bounded filename-safe opaque id",
        ));
    }
    Ok(())
}

fn validate_opaque_ref(value: &str, detail: &'static str) -> Result<(), CrashJournalError> {
    if value.is_empty()
        || value.len() > MAX_OPAQUE_REF_BYTES
        || value == "."
        || value == ".."
        || value.contains('/')
        || value.contains('\\')
        || value.contains("://")
        || value.bytes().any(|byte| byte.is_ascii_control())
        || value.chars().any(char::is_whitespace)
    {
        return Err(CrashJournalError::InvalidInput(detail));
    }
    Ok(())
}

fn validate_short_text(value: &str, detail: &'static str) -> Result<(), CrashJournalError> {
    if value.is_empty() || value.len() > MAX_OPAQUE_REF_BYTES || value.bytes().any(|byte| byte == 0)
    {
        return Err(CrashJournalError::InvalidInput(detail));
    }
    Ok(())
}

fn validate_presentation_hint(value: &str) -> Result<(), CrashJournalError> {
    if value.len() > MAX_PRESENTATION_HINT_BYTES
        || value.contains('/')
        || value.contains('\\')
        || value.bytes().any(|byte| byte.is_ascii_control())
    {
        return Err(CrashJournalError::InvalidInput(
            "presentation hint is not a sanitized basename",
        ));
    }
    Ok(())
}

fn parse_body_object_ref(body_object_ref: &str) -> Result<&str, CrashJournalError> {
    let checksum =
        body_object_ref
            .strip_prefix("journal-body:")
            .ok_or(CrashJournalError::InvalidInput(
                "journal body ref has an unsupported scheme",
            ))?;
    if checksum.len() != 64
        || !checksum
            .bytes()
            .all(|byte| byte.is_ascii_digit() || matches!(byte, b'a'..=b'f'))
    {
        return Err(CrashJournalError::InvalidInput(
            "journal body ref has an invalid digest",
        ));
    }
    Ok(checksum)
}

fn validate_capture_input(input: &CrashJournalCaptureInput) -> Result<(), CrashJournalError> {
    validate_opaque_ref(&input.journal_id, "journal id is not a bounded opaque ref")?;
    validate_opaque_ref(
        &input.workspace_ref,
        "workspace ref is not a bounded opaque ref",
    )?;
    validate_opaque_ref(
        &input.logical_document_id,
        "logical document id is not a bounded opaque ref",
    )?;
    validate_opaque_ref(&input.object_ref, "object ref is not a bounded opaque ref")?;
    validate_short_text(&input.emitted_at, "capture timestamp is invalid")?;
    if let Some(hint) = &input.presentation_hint {
        validate_presentation_hint(hint)?;
    }
    if input.bytes.len() > MAX_JOURNAL_BODY_BYTES {
        return Err(CrashJournalError::TooLarge("dirty-buffer body object"));
    }
    Ok(())
}

fn validate_entry_for_storage(entry: &AutosaveJournalEntryRecord) -> Result<(), CrashJournalError> {
    if entry.record_kind != "autosave_journal_entry" || entry.autosave_journal_schema_version != 1 {
        return Err(CrashJournalError::CorruptRecord(
            "unsupported journal record kind or schema",
        ));
    }
    validate_filename_id(&entry.journal_entry_id)?;
    validate_opaque_ref(&entry.journal_id, "journal id is not a bounded opaque ref")?;
    validate_opaque_ref(
        &entry.workspace_ref,
        "workspace ref is not a bounded opaque ref",
    )?;
    validate_opaque_ref(
        &entry.object_identity.logical_document_id,
        "logical document id is not a bounded opaque ref",
    )?;
    validate_opaque_ref(
        &entry.object_identity.object_ref,
        "object ref is not a bounded opaque ref",
    )?;
    if let Some(hint) = &entry.object_identity.presentation_hint {
        validate_presentation_hint(hint)?;
    }
    validate_short_text(&entry.emitted_at, "journal timestamp is invalid")?;

    if entry.capture_descriptor.body_object_refs.len() > MAX_BODY_REFS_PER_ENTRY {
        return Err(CrashJournalError::CorruptRecord(
            "journal entry has too many body refs",
        ));
    }
    if entry.capture_descriptor.body_available
        != !entry.capture_descriptor.body_object_refs.is_empty()
    {
        return Err(CrashJournalError::CorruptRecord(
            "journal body availability disagrees with body refs",
        ));
    }
    for body_ref in &entry.capture_descriptor.body_object_refs {
        parse_body_object_ref(body_ref)?;
    }

    match entry.integrity.checksum_algorithm {
        ChecksumAlgorithm::Blake3 => {
            let checksum = &entry.integrity.checksum_ref;
            if checksum.len() != 64
                || !checksum
                    .bytes()
                    .all(|byte| byte.is_ascii_digit() || matches!(byte, b'a'..=b'f'))
            {
                return Err(CrashJournalError::CorruptRecord(
                    "journal integrity checksum is malformed",
                ));
            }
            if entry.capture_descriptor.body_available
                && !entry
                    .capture_descriptor
                    .body_object_refs
                    .iter()
                    .any(|body_ref| {
                        body_ref
                            .strip_prefix("journal-body:")
                            .is_some_and(|digest| digest == checksum)
                    })
            {
                return Err(CrashJournalError::CorruptRecord(
                    "journal checksum is not bound to a body ref",
                ));
            }
        }
        ChecksumAlgorithm::Unknown => {
            validate_short_text(
                &entry.integrity.checksum_ref,
                "unknown checksum ref is invalid",
            )?;
        }
    }

    if !entry.retention_posture.local_only_default
        || !entry.retention_posture.ordinary_cache_clear_excluded
        || !entry.retention_posture.settings_reset_excluded
        || !entry.retention_posture.local_history_clear_excluded
        || !entry.retention_posture.journal_reset_required_for_delete
    {
        return Err(CrashJournalError::CorruptRecord(
            "journal retention boundaries were widened",
        ));
    }
    Ok(())
}

fn write_new_json<T: Serialize>(
    root: &Path,
    path: &Path,
    value: &T,
) -> Result<(), CrashJournalError> {
    let json = serde_json::to_vec_pretty(value)?;
    if json.len() > MAX_JOURNAL_RECORD_BYTES {
        return Err(CrashJournalError::TooLarge("serialized journal record"));
    }
    publish_new_file(root, path, &json)
}

fn write_content_addressed_blob(
    root: &Path,
    path: &Path,
    bytes: &[u8],
) -> Result<(), CrashJournalError> {
    if bytes.len() > MAX_JOURNAL_BODY_BYTES {
        return Err(CrashJournalError::TooLarge("dirty-buffer body object"));
    }
    match publish_new_file(root, path, bytes) {
        Ok(()) => Ok(()),
        Err(CrashJournalError::Io(err)) if err.kind() == std::io::ErrorKind::AlreadyExists => {
            let existing = read_bounded_regular_file(path, MAX_JOURNAL_BODY_BYTES)?;
            let expected = path.file_stem().and_then(|stem| stem.to_str()).ok_or(
                CrashJournalError::Integrity("journal body filename is invalid"),
            )?;
            let actual = blake3::hash(&existing).to_hex().to_string();
            if actual != expected {
                return Err(CrashJournalError::Integrity(
                    "existing journal body does not match its object id",
                ));
            }
            Ok(())
        }
        Err(err) => Err(err),
    }
}

fn publish_new_file(root: &Path, path: &Path, bytes: &[u8]) -> Result<(), CrashJournalError> {
    let parent = path.parent().ok_or(CrashJournalError::InvalidInput(
        "journal file has no parent",
    ))?;
    ensure_safe_directory(root, parent)?;
    checked_relative(root, path)?;

    let file_name =
        path.file_name()
            .and_then(|name| name.to_str())
            .ok_or(CrashJournalError::InvalidInput(
                "journal filename is not UTF-8",
            ))?;
    let temporary = parent.join(format!(
        ".{file_name}.tmp-{}-{}-{}",
        std::process::id(),
        TEMP_FILE_SEQUENCE.fetch_add(1, Ordering::Relaxed),
        unix_nanos()
    ));
    let result = (|| -> Result<(), CrashJournalError> {
        let mut file = OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(&temporary)?;
        file.write_all(bytes)?;
        file.sync_all()?;
        fs::hard_link(&temporary, path)?;
        sync_directory(parent)?;
        Ok(())
    })();
    let _ = fs::remove_file(&temporary);
    result
}

fn checked_relative<'a>(root: &Path, path: &'a Path) -> Result<&'a Path, CrashJournalError> {
    let relative = path
        .strip_prefix(root)
        .map_err(|_| CrashJournalError::InvalidInput("journal path escapes store root"))?;
    if relative
        .components()
        .any(|component| !matches!(component, Component::Normal(_)))
    {
        return Err(CrashJournalError::InvalidInput(
            "journal path contains a non-normal component",
        ));
    }
    Ok(relative)
}

fn ensure_safe_directory(root: &Path, path: &Path) -> Result<(), CrashJournalError> {
    let relative = checked_relative(root, path)?;
    if root.as_os_str().is_empty()
        || root
            .components()
            .any(|component| matches!(component, Component::ParentDir))
    {
        return Err(CrashJournalError::InvalidInput(
            "invalid crash-journal storage root",
        ));
    }
    match fs::symlink_metadata(root) {
        Ok(metadata) if metadata.file_type().is_symlink() || !metadata.is_dir() => {
            return Err(CrashJournalError::InvalidInput(
                "crash-journal storage root is not a regular directory",
            ));
        }
        Ok(_) => {}
        Err(err) if err.kind() == std::io::ErrorKind::NotFound => {
            fs::create_dir_all(root)?;
            let metadata = fs::symlink_metadata(root)?;
            if metadata.file_type().is_symlink() || !metadata.is_dir() {
                return Err(CrashJournalError::InvalidInput(
                    "crash-journal root creation was redirected",
                ));
            }
        }
        Err(err) => return Err(CrashJournalError::Io(err)),
    }

    let mut current = root.to_path_buf();
    for component in relative.components() {
        let Component::Normal(component) = component else {
            return Err(CrashJournalError::InvalidInput(
                "journal directory contains a non-normal component",
            ));
        };
        current.push(component);
        match fs::symlink_metadata(&current) {
            Ok(metadata) if metadata.file_type().is_symlink() || !metadata.is_dir() => {
                return Err(CrashJournalError::InvalidInput(
                    "journal directory is not a regular directory",
                ));
            }
            Ok(_) => {}
            Err(err) if err.kind() == std::io::ErrorKind::NotFound => {
                match fs::create_dir(&current) {
                    Ok(()) => {}
                    Err(err) if err.kind() == std::io::ErrorKind::AlreadyExists => {
                        let metadata = fs::symlink_metadata(&current)?;
                        if metadata.file_type().is_symlink() || !metadata.is_dir() {
                            return Err(CrashJournalError::InvalidInput(
                                "journal directory was replaced during creation",
                            ));
                        }
                    }
                    Err(err) => return Err(CrashJournalError::Io(err)),
                }
            }
            Err(err) => return Err(CrashJournalError::Io(err)),
        }
    }
    Ok(())
}

fn read_bounded_regular_file(path: &Path, max_bytes: usize) -> Result<Vec<u8>, CrashJournalError> {
    let metadata = fs::symlink_metadata(path)?;
    if metadata.file_type().is_symlink() || !metadata.is_file() {
        return Err(CrashJournalError::CorruptRecord(
            "journal object is not a regular file",
        ));
    }
    if metadata.len() > max_bytes as u64 {
        return Err(CrashJournalError::TooLarge("journal object read"));
    }
    let file = fs::File::open(path)?;
    let opened_metadata = file.metadata()?;
    if !opened_metadata.is_file() || opened_metadata.len() > max_bytes as u64 {
        return Err(CrashJournalError::CorruptRecord(
            "journal object changed during read",
        ));
    }
    let mut bytes = Vec::with_capacity(opened_metadata.len() as usize);
    file.take(max_bytes as u64 + 1).read_to_end(&mut bytes)?;
    if bytes.len() > max_bytes {
        return Err(CrashJournalError::TooLarge("journal object read"));
    }
    Ok(bytes)
}

#[cfg(unix)]
fn sync_directory(path: &Path) -> Result<(), CrashJournalError> {
    fs::File::open(path)?.sync_all()?;
    Ok(())
}

#[cfg(not(unix))]
fn sync_directory(_path: &Path) -> Result<(), CrashJournalError> {
    Ok(())
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
struct StoreProbeRecord {
    record_kind: String,
    emitted_at: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    fn capture_input(workspace_ref: &str, suffix: &str, bytes: &[u8]) -> CrashJournalCaptureInput {
        CrashJournalCaptureInput {
            journal_id: format!("journal:{workspace_ref}"),
            workspace_ref: workspace_ref.to_string(),
            logical_document_id: format!("ld:{suffix}"),
            object_ref: format!("buffer:{suffix}"),
            object_class: ObjectClass::CanonicalFile,
            presentation_hint: Some(format!("{suffix}.txt")),
            emitted_at: format!("mono:{suffix}"),
            bytes: bytes.to_vec(),
        }
    }

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
        write_new_json(&store.root, &path, &record).expect("write json");
        let original = fs::read(&path).expect("read original");
        let second = write_new_json(&store.root, &path, &record);
        assert!(
            second.is_err(),
            "expected create_new write to refuse overwriting existing entry"
        );
        assert_eq!(fs::read(&path).expect("read preserved"), original);
    }

    #[test]
    fn workspace_scoped_load_filters_by_embedded_authority() {
        let dir = tempfile::tempdir().expect("tempdir");
        let mut store = CrashJournalStore::new(dir.path());
        let alpha = store
            .capture_minimal_full_snapshot(capture_input("ws-alpha", "alpha", b"alpha"))
            .expect("capture alpha");
        store
            .capture_minimal_full_snapshot(capture_input("ws-beta", "beta", b"beta"))
            .expect("capture beta");

        let scoped = store
            .load_entries_for_workspace("ws-alpha")
            .expect("scoped load");
        assert_eq!(scoped.len(), 1);
        assert_eq!(scoped[0].journal_entry_id, alpha.journal_entry_id);
        assert_eq!(scoped[0].workspace_ref, "ws-alpha");

        let invalid = store.load_entries_for_workspace("../../ws-alpha");
        assert!(matches!(invalid, Err(CrashJournalError::InvalidInput(_))));
    }

    #[test]
    fn entry_ids_cannot_escape_or_alias_storage_paths() {
        let dir = tempfile::tempdir().expect("tempdir");
        let mut store = CrashJournalStore::new(dir.path());
        let mut record = store
            .capture_minimal_full_snapshot(capture_input("ws-alpha", "safe", b"body"))
            .expect("capture seed");
        record.journal_entry_id = "../outside".to_string();

        let error = store
            .write_entry(&record)
            .expect_err("path-shaped id must be rejected");
        assert!(matches!(error, CrashJournalError::InvalidInput(_)));
        assert!(!dir.path().join("outside.json").exists());
    }

    #[test]
    fn body_reads_verify_digest_and_enforce_size_bound() {
        let dir = tempfile::tempdir().expect("tempdir");
        let store = CrashJournalStore::new(dir.path());
        let (body_ref, checksum) = store.write_body_object(b"original").expect("write body");
        let path = store.root.join("bodies").join(format!("{checksum}.bin"));
        fs::write(&path, b"tampered").expect("tamper body");

        assert!(matches!(
            store.read_body_object(&body_ref),
            Err(CrashJournalError::Integrity(_))
        ));

        let oversized_checksum = blake3::hash(b"oversized-placeholder").to_hex().to_string();
        let oversized_path = store
            .root
            .join("bodies")
            .join(format!("{oversized_checksum}.bin"));
        let file = OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(&oversized_path)
            .expect("create sparse body");
        file.set_len(MAX_JOURNAL_BODY_BYTES as u64 + 1)
            .expect("extend sparse body");
        let oversized_ref = format!("journal-body:{oversized_checksum}");
        assert!(matches!(
            store.read_body_object(&oversized_ref),
            Err(CrashJournalError::TooLarge(_))
        ));
    }

    #[test]
    fn existing_wrong_content_addressed_body_is_never_accepted_or_replaced() {
        let dir = tempfile::tempdir().expect("tempdir");
        let store = CrashJournalStore::new(dir.path());
        let wanted = b"wanted";
        let checksum = blake3::hash(wanted).to_hex().to_string();
        let bodies = store.root.join("bodies");
        ensure_safe_directory(&store.root, &bodies).expect("create bodies");
        let path = bodies.join(format!("{checksum}.bin"));
        fs::write(&path, b"attacker-content").expect("seed wrong body");

        assert!(matches!(
            store.write_body_object(wanted),
            Err(CrashJournalError::Integrity(_))
        ));
        assert_eq!(
            fs::read(&path).expect("read preserved attacker body"),
            b"attacker-content"
        );
    }

    #[cfg(unix)]
    #[test]
    fn symlinked_entry_and_body_files_fail_closed() {
        use std::os::unix::fs::symlink;

        let dir = tempfile::tempdir().expect("tempdir");
        let mut store = CrashJournalStore::new(dir.path());
        let record = store
            .capture_minimal_full_snapshot(capture_input("ws-alpha", "safe", b"body"))
            .expect("capture seed");
        let entry_path = store
            .root
            .join("entries")
            .join(format!("{}.json", record.journal_entry_id));
        fs::remove_file(&entry_path).expect("remove entry");
        let victim = dir.path().join("victim.json");
        fs::write(&victim, b"{}").expect("seed victim");
        symlink(&victim, &entry_path).expect("symlink entry");

        assert!(matches!(
            store.load_entries_for_workspace("ws-alpha"),
            Err(CrashJournalError::CorruptRecord(_))
        ));
    }
}
