//! Schema-shaped autosave-journal record types.
//!
//! These types mirror the boundary vocabulary frozen in
//! `schemas/recovery/autosave_journal_entry.schema.json`. They intentionally
//! avoid embedding raw file bodies, raw paths, credentials, or unstructured
//! crash dumps; recoverable bytes are stored separately and referenced by
//! opaque ids.

use serde::{Deserialize, Serialize};

/// Schema version for autosave-journal records.
pub type AutosaveJournalSchemaVersion = u32;

/// Closed object-class vocabulary for autosave journal recovery.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ObjectClass {
    CanonicalFile,
    VirtualBuffer,
    UntitledBuffer,
    GeneratedArtifact,
    ManagedMirror,
    ReadOnlyFile,
    NotebookCell,
    StructuredArtifact,
    UnsupportedObject,
}

/// Relationship between the capture target and the current object identity.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum IdentityRelation {
    ExactObjectIdentity,
    AliasCanonicalDrift,
    SamePathDifferentObject,
    CurrentObjectMissing,
    VirtualOnly,
    IdentityUnknown,
}

/// Strongest base-on-disk token class available at capture time.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TokenClass {
    FileIdGeneration,
    InodeMtimeSizeHash,
    RemoteEtagRevision,
    ContentHashOnly,
    MissingOrNotApplicable,
    Unknown,
}

/// Confidence level for the base-on-disk token.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TokenConfidenceClass {
    Strong,
    CompatibleFallback,
    Weak,
    Unknown,
}

/// External-change posture for the capture target.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ExternalChangeState {
    NoExternalChangeKnown,
    ExternalChangeDetected,
    ExternalChangeUnknown,
    TargetMissing,
    TargetReadOnly,
}

/// Closed encoding-label vocabulary for autosave recovery.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EncodingLabelClass {
    Utf8,
    Utf8Sig,
    Utf16Le,
    Utf16Be,
    Latin1,
    Windows1252,
    ShiftJis,
    BinaryOrNotText,
    Unknown,
}

/// Closed newline-mode vocabulary for autosave recovery.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum NewlineMode {
    Lf,
    Crlf,
    Cr,
    Mixed,
    BinaryOrNotText,
    Unknown,
}

/// Decoder posture describing how bytes were interpreted.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DecoderPosture {
    ExactDecode,
    LossyDecodeRawPreserved,
    BinarySnapshot,
    MetadataOnly,
    Unknown,
}

/// Final-newline posture captured at journal time.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FinalNewlineState {
    Present,
    Absent,
    MixedOrUnknown,
    NotApplicable,
}

/// Actor class that initiated the capture.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ActorClass {
    UserKeystroke,
    UserCommand,
    MultiCursorCommand,
    Formatter,
    SaveParticipant,
    AiOrToolApply,
    ExtensionSurface,
    RemoteSession,
    ExternalChangeDetector,
    CrashRecoveryJournal,
    AutomationRecipeRunner,
    Unknown,
}

/// Source class indicating where the action originated.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SourceClass {
    HumanLocal,
    HumanRemoteSession,
    MachineLocal,
    MachineRemoteAgent,
    AiLocalModel,
    AiHostedProvider,
    ExtensionHost,
    ReplayedCapture,
    ExternalObservation,
    PolicyDriven,
    Unknown,
}

/// Surface class where the capture originated.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SurfaceClass {
    EditorTyping,
    SavePipeline,
    NotebookEditor,
    StructuredArtifactEditor,
    GeneratedViewer,
    ExternalChangeReview,
    CrashRestoreReview,
    ExtensionSurface,
    RemoteSession,
    AutomationSurface,
    Unknown,
}

/// Capture class describing what was captured.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CaptureClass {
    DirtyTextDelta,
    FullBufferSnapshot,
    CursorViewState,
    SaveCoordinationMetadata,
    GroupedManifest,
    MetadataOnlyStub,
    EvidenceOnlyCorruptFrame,
}

/// Capture mode describing how the capture is represented.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CaptureMode {
    ContentAddressedSnapshot,
    JournalDeltaChain,
    MetadataPlusReferenceOnly,
    GroupManifestOnly,
    EvidenceOnly,
}

/// Reason a body capture was omitted.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CaptureOmissionReason {
    NotOmitted,
    OmittedTooLarge,
    OmittedBinaryClass,
    OmittedGeneratedArtifactUseLineage,
    OmittedReadOnlyOrManagedTarget,
    OmittedPolicyRedacted,
    OmittedIntegrityFailed,
    OmittedUnsupportedObjectClass,
}

/// Checksum algorithm used by the integrity record.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ChecksumAlgorithm {
    Blake3,
    Unknown,
}

/// Frame integrity state for a capture.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FrameIntegrityState {
    Verified,
    ChecksumMismatch,
    TruncatedFrame,
    SchemaIncompatible,
    MissingFrame,
    Unverifiable,
}

/// Replay-integrity posture for a capture.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ReplayIntegrityPosture {
    ReplayAllowed,
    ReplayAllowedUntilLastGoodFrame,
    InspectOnly,
    EvidenceOnly,
    Blocked,
}

/// Replay posture for a capture based on object class and policy gates.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ReplayPostureClass {
    RestoreAllowed,
    RestoreRequiresReview,
    InspectOnly,
    OpenWithoutReplayDefault,
    BlockedGeneratedRequiresSource,
    BlockedReadOnlyTarget,
    BlockedManagedMirror,
    BlockedPolicy,
    EscalateSafeMode,
}

/// Guided choice vocabulary for replay flows.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum GuidedChoiceClass {
    Restore,
    InspectOnly,
    DiscardJournal,
    OpenWithoutReplay,
    EscalateToSafeMode,
}

/// Downgrade reasons recorded on replay posture.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DowngradeReasonClass {
    NotDowngraded,
    ChecksumMismatch,
    TruncatedFrame,
    SchemaIncompatible,
    PolicyRedactedBody,
    PolicyBlocksBodyRestore,
    IncompatibleObjectClass,
    GeneratedTargetRequiresSource,
    ReadOnlyTarget,
    ManagedMirror,
    SamePathDifferentObject,
    BaseOnDiskTokenMismatch,
    CurrentObjectMissing,
    RepeatedCrashLoop,
    SuspectExtension,
    UserChoseOpenWithoutReplay,
}

/// Retention class describing how long records are retained by default.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RetentionClass {
    ActiveReplayWindow,
    RetainedUntilUserDecision,
    RetainedBySupportOrEvidenceRef,
    RetainedByPolicyWindow,
    RetainedByUserPin,
    EvidenceOnlyAfterFailedReplay,
    ResetTombstoneOnly,
    ExpiredStubOnly,
}

/// Support bundle inclusion state for recovery records.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SupportBundleInclusionState {
    ExcludedByDefault,
    MetadataRefAllowed,
    ReviewRequiredBeforeExport,
    ExportedInSupportBundle,
    Prohibited,
}

/// Identity record for a captured object.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ObjectIdentityRecord {
    pub logical_document_id: String,
    pub object_ref: String,
    pub object_class: ObjectClass,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub presentation_hint: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filesystem_identity_ref: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub canonical_identity_ref: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub branch_worktree_ref: Option<String>,
    pub identity_relation: IdentityRelation,
    pub identity_notes: String,
}

/// Base-on-disk token captured alongside a journal entry.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BaseOnDiskTokenRecord {
    pub token_class: TokenClass,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token_ref: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub observed_revision_ref: Option<String>,
    pub token_confidence: TokenConfidenceClass,
    pub compare_before_write_required: bool,
    pub external_change_state: ExternalChangeState,
}

/// Text-format details captured for a journal entry.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TextFormatRecord {
    pub encoding_label: EncodingLabelClass,
    pub bom_policy: String,
    pub newline_mode: NewlineMode,
    pub decoder_posture: DecoderPosture,
    pub final_newline_state: FinalNewlineState,
    pub large_file_mode: bool,
    pub format_notes: String,
}

/// Actor/surface metadata captured for a journal entry.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ActorSurfaceRecord {
    pub actor_class: ActorClass,
    pub source_class: SourceClass,
    pub surface_class: SurfaceClass,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub command_ref: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub session_ref: Option<String>,
    pub actor_display: String,
}

/// Capture descriptor that points at stored body objects.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CaptureDescriptorRecord {
    pub capture_class: CaptureClass,
    pub capture_mode: CaptureMode,
    pub body_available: bool,
    pub body_object_refs: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dirty_range_summary_ref: Option<String>,
    pub group_member_refs: Vec<String>,
    pub omission_reason: CaptureOmissionReason,
    pub capture_notes: String,
}

/// Integrity metadata for a capture.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct IntegrityRecord {
    pub checksum_algorithm: ChecksumAlgorithm,
    pub checksum_ref: String,
    pub frame_integrity_state: FrameIntegrityState,
    pub replay_integrity_posture: ReplayIntegrityPosture,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_good_frame_ref: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failed_frame_ref: Option<String>,
    pub corruption_evidence_refs: Vec<String>,
    pub integrity_notes: String,
}

/// Replay posture for a journal entry.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReplayPostureRecord {
    pub object_class_replay_posture: ReplayPostureClass,
    pub recommended_choice_class: GuidedChoiceClass,
    pub blocked_choice_classes: Vec<GuidedChoiceClass>,
    pub downgrade_reason_classes: Vec<DowngradeReasonClass>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub new_local_history_checkpoint_on_restore: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub new_checkpoint_ref: Option<String>,
    pub open_without_replay_retains_journal: bool,
    pub replay_notes: String,
}

/// Retention posture for a journal entry.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RetentionPostureRecord {
    pub retention_class: RetentionClass,
    pub local_only_default: bool,
    pub ordinary_cache_clear_excluded: bool,
    pub settings_reset_excluded: bool,
    pub local_history_clear_excluded: bool,
    pub journal_reset_required_for_delete: bool,
    pub export_before_reset: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expiry_policy_ref: Option<String>,
    pub pin_refs: Vec<String>,
}

/// Support export posture for a journal entry.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SupportExportRecord {
    pub support_bundle_inclusion_state: SupportBundleInclusionState,
    pub redaction_class: String,
    pub support_export_refs: Vec<String>,
    pub export_notes: String,
}

/// Schema-shaped autosave journal entry record.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AutosaveJournalEntryRecord {
    pub record_kind: String,
    pub autosave_journal_schema_version: AutosaveJournalSchemaVersion,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fixture_metadata: Option<serde_json::Value>,
    pub journal_entry_id: String,
    pub journal_id: String,
    pub workspace_ref: String,
    pub object_identity: ObjectIdentityRecord,
    pub base_on_disk_token: BaseOnDiskTokenRecord,
    pub text_format: TextFormatRecord,
    pub actor_surface: ActorSurfaceRecord,
    pub capture_descriptor: CaptureDescriptorRecord,
    pub integrity: IntegrityRecord,
    pub replay_posture: ReplayPostureRecord,
    pub retention_posture: RetentionPostureRecord,
    pub support_export: SupportExportRecord,
    pub emitted_at: String,
}

impl AutosaveJournalEntryRecord {
    /// Creates a new autosave journal entry record.
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        journal_entry_id: String,
        journal_id: String,
        workspace_ref: String,
        object_identity: ObjectIdentityRecord,
        base_on_disk_token: BaseOnDiskTokenRecord,
        text_format: TextFormatRecord,
        actor_surface: ActorSurfaceRecord,
        capture_descriptor: CaptureDescriptorRecord,
        integrity: IntegrityRecord,
        replay_posture: ReplayPostureRecord,
        retention_posture: RetentionPostureRecord,
        support_export: SupportExportRecord,
        emitted_at: String,
    ) -> Self {
        Self {
            record_kind: "autosave_journal_entry".to_string(),
            autosave_journal_schema_version: 1,
            fixture_metadata: None,
            journal_entry_id,
            journal_id,
            workspace_ref,
            object_identity,
            base_on_disk_token,
            text_format,
            actor_surface,
            capture_descriptor,
            integrity,
            replay_posture,
            retention_posture,
            support_export,
            emitted_at,
        }
    }
}
