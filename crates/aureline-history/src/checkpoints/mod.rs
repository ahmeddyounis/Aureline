//! Local-history checkpoint storage.
//!
//! Local history persists attributable snapshots and metadata stubs that
//! recovery, compare, and support surfaces can inspect.

use std::path::PathBuf;

use aureline_vfs::{AliasKind, IdentityRecord, NormalizationForm, TrustState};
use serde::{Deserialize, Serialize};

use crate::mutation_journal::{
    ActorClass as JournalActorClass, RedactionClass, ReversalClass as JournalReversalClass,
    SourceClass,
};
use crate::storage::{HistoryError, HistoryStorageRoot, IdSource};

/// Writer for local-history entries and groups plus their content-addressed bodies.
#[derive(Debug, Clone)]
pub struct LocalHistoryStore {
    storage: HistoryStorageRoot,
    root: PathBuf,
    objects_root: PathBuf,
    entry_ids: IdSource,
    group_ids: IdSource,
}

impl LocalHistoryStore {
    /// Creates a store rooted at `root/local_history`.
    pub fn new(storage: HistoryStorageRoot) -> Self {
        let root = storage.path().join("local_history");
        let objects_root = storage.path().join("objects");
        Self {
            storage,
            root,
            objects_root,
            entry_ids: IdSource::new("lh"),
            group_ids: IdSource::new("lhg"),
        }
    }

    /// Mints a new local-history entry id.
    pub fn mint_entry_id(&mut self) -> String {
        self.entry_ids.mint()
    }

    /// Mints a new local-history group id.
    pub fn mint_group_id(&mut self) -> String {
        self.group_ids.mint()
    }

    /// Returns the on-disk root that holds content-addressed body objects.
    ///
    /// Callers that need to rehydrate a previously-persisted body (for
    /// example a preview/apply/revert wedge restoring a checkpoint) can
    /// resolve `obj:blake3:<hex>` ids against this root.
    pub fn objects_root_path(&self) -> std::path::PathBuf {
        self.objects_root.clone()
    }

    /// Writes a captured body into the content-addressed object store.
    pub fn write_body_object(&self, bytes: &[u8]) -> Result<String, HistoryError> {
        let digest = blake3::hash(bytes).to_hex().to_string();
        let object_id = format!("obj:blake3:{digest}");
        let path = self.objects_root.join(format!("{digest}.blob"));
        if path.exists() {
            return Ok(object_id);
        }
        self.storage.write_new_blob(&path, bytes)?;
        Ok(object_id)
    }

    /// Persists a local-history entry record.
    pub fn write_entry(&self, entry: &LocalHistoryEntryRecord) -> Result<PathBuf, HistoryError> {
        let path = self
            .root
            .join("entries")
            .join(format!("{}.json", sanitize_id(&entry.entry_id)));
        self.storage.write_new_json(&path, entry)?;
        Ok(path)
    }

    /// Persists a local-history group record.
    pub fn write_group(&self, group: &LocalHistoryGroupRecord) -> Result<PathBuf, HistoryError> {
        let path = self
            .root
            .join("groups")
            .join(format!("{}.json", sanitize_id(&group.group_id)));
        self.storage.write_new_json(&path, group)?;
        Ok(path)
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

/// Schema-shaped filesystem-identity record exported into checkpoints and journals.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FilesystemIdentityRecord {
    pub record_kind: String,
    pub filesystem_identity_schema_version: u32,
    pub presentation_path: PresentationPathRecord,
    pub logical_workspace_identity: LogicalWorkspaceIdentityRecord,
    pub canonical_filesystem_object: CanonicalFilesystemObjectRecord,
    pub alias_set: AliasSetRecord,
}

impl FilesystemIdentityRecord {
    /// Converts a VFS [`IdentityRecord`] into a schema-shaped export record.
    pub fn from_vfs(identity: &IdentityRecord) -> Self {
        let aliases = identity
            .alias_set
            .aliases
            .iter()
            .map(|alias| AliasRecord {
                alias_uri: alias.alias_uri.to_string(),
                alias_kind: alias.alias_kind.as_str().to_owned(),
                resolution_chain: if alias.resolution_chain.is_empty() {
                    None
                } else {
                    Some(alias.resolution_chain.clone())
                },
            })
            .collect();

        Self {
            record_kind: "filesystem_identity_record".to_owned(),
            filesystem_identity_schema_version: 1,
            presentation_path: PresentationPathRecord {
                uri: identity.presentation_path.uri.to_string(),
                display_label: identity.presentation_path.display_label.clone(),
                root_badge: identity.presentation_path.root_badge.clone(),
            },
            logical_workspace_identity: LogicalWorkspaceIdentityRecord {
                workspace_id: identity.logical_workspace_identity.workspace_id.clone(),
                root_id: identity.logical_workspace_identity.root_id.clone(),
                logical_uri: identity.logical_workspace_identity.logical_uri.to_string(),
                trust_state: identity
                    .logical_workspace_identity
                    .trust_state
                    .as_str()
                    .to_owned(),
                policy_scope: identity.logical_workspace_identity.policy_scope.clone(),
            },
            canonical_filesystem_object: CanonicalFilesystemObjectRecord {
                canonical_uri: identity
                    .canonical_filesystem_object
                    .canonical_uri
                    .to_string(),
                normalization_form: normalization_form_string(
                    identity.canonical_filesystem_object.normalization_form,
                ),
                strongest_identity_token: IdentityTokenRecord {
                    kind: identity
                        .canonical_filesystem_object
                        .strongest_identity_token
                        .kind
                        .as_str()
                        .to_owned(),
                    value: identity
                        .canonical_filesystem_object
                        .strongest_identity_token
                        .value
                        .clone(),
                },
                fallback_identity_tokens: identity
                    .canonical_filesystem_object
                    .fallback_identity_tokens
                    .iter()
                    .map(|token| FallbackIdentityTokenRecord {
                        kind: token.kind.as_str().to_owned(),
                        value: token.value.clone(),
                    })
                    .collect(),
            },
            alias_set: AliasSetRecord { aliases },
        }
    }
}

fn normalization_form_string(form: NormalizationForm) -> String {
    form.as_str().to_owned()
}

/// Schema-shaped presentation-path record.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PresentationPathRecord {
    pub uri: String,
    pub display_label: String,
    pub root_badge: String,
}

/// Schema-shaped logical-workspace-identity record.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LogicalWorkspaceIdentityRecord {
    pub workspace_id: String,
    pub root_id: String,
    pub logical_uri: String,
    pub trust_state: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_scope: Option<String>,
}

/// Schema-shaped canonical-filesystem-object record.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CanonicalFilesystemObjectRecord {
    pub canonical_uri: String,
    pub normalization_form: String,
    pub strongest_identity_token: IdentityTokenRecord,
    pub fallback_identity_tokens: Vec<FallbackIdentityTokenRecord>,
}

/// Schema-shaped identity-token record.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct IdentityTokenRecord {
    pub kind: String,
    pub value: String,
}

/// Schema-shaped fallback-identity-token record.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FallbackIdentityTokenRecord {
    pub kind: String,
    pub value: String,
}

/// Schema-shaped alias-set record.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AliasSetRecord {
    pub aliases: Vec<AliasRecord>,
}

/// Schema-shaped alias record.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AliasRecord {
    pub alias_uri: String,
    pub alias_kind: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resolution_chain: Option<Vec<String>>,
}

/// Schema-shaped local-history entry record.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LocalHistoryEntryRecord {
    pub record_kind: String,
    pub local_history_schema_version: u32,
    pub entry_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_id: Option<String>,
    pub snapshot_class: SnapshotClass,
    pub captured_at: String,
    pub truth_source_class: TruthSourceClass,
    pub logical_document_identity: LogicalDocumentIdentity,
    pub branch_worktree_context: BranchWorktreeContext,
    pub capture_descriptor: CaptureDescriptor,
    pub mutation_journal_link: MutationJournalLink,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub restore_of_ref: Option<RestoreOfEntryRef>,
    pub retention_scope: RetentionScopeClass,
    pub local_only_posture: LocalOnlyPosture,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub side_effect_summary: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
}

/// Snapshot-class vocabulary for local-history entries.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SnapshotClass {
    EditSaveCheckpoint,
    WorkspaceMutationCheckpoint,
    AutomationAiCheckpoint,
    ExternalStateCheckpoint,
    RestoreRollbackCheckpoint,
    CaptureOmittedStub,
    PolicyRedactedStub,
}

impl SnapshotClass {
    /// Returns the schema token for this snapshot class.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::EditSaveCheckpoint => "edit_save_checkpoint",
            Self::WorkspaceMutationCheckpoint => "workspace_mutation_checkpoint",
            Self::AutomationAiCheckpoint => "automation_ai_checkpoint",
            Self::ExternalStateCheckpoint => "external_state_checkpoint",
            Self::RestoreRollbackCheckpoint => "restore_rollback_checkpoint",
            Self::CaptureOmittedStub => "capture_omitted_stub",
            Self::PolicyRedactedStub => "policy_redacted_stub",
        }
    }
}

/// Truth-source-class vocabulary rendered on the timeline.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TruthSourceClass {
    LocalHistory,
    AutosaveJournal,
    GitHistory,
    AutomationLineage,
    ExternalChangeRecord,
    SyncedOrProviderHistory,
    ReviewCheckpoint,
}

impl TruthSourceClass {
    /// Returns the schema token for this truth-source class.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LocalHistory => "local_history",
            Self::AutosaveJournal => "autosave_journal",
            Self::GitHistory => "git_history",
            Self::AutomationLineage => "automation_lineage",
            Self::ExternalChangeRecord => "external_change_record",
            Self::SyncedOrProviderHistory => "synced_or_provider_history",
            Self::ReviewCheckpoint => "review_checkpoint",
        }
    }
}

/// Logical-document identity carried on local-history entries.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LogicalDocumentIdentity {
    pub logical_document_id: String,
    pub current_filesystem_identity: FilesystemIdentityRecord,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub canonical_identity_drift: Option<CanonicalIdentityDrift>,
    pub rename_move_history: Vec<RenameMoveEvent>,
}

/// Drift class between the logical id and the current canonical object.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CanonicalIdentityDrift {
    NoDrift,
    RenameDetected,
    MoveDetected,
    RenameAndMoveDetected,
    DeviceInodeReuseDetected,
    ProviderObjectIdRotated,
    CanonicalIdentityUnknown,
}

/// Rename/move event carried on local-history entries.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RenameMoveEvent {
    pub kind: RenameMoveEventKind,
    pub at: String,
    pub previous_presentation_label: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub new_presentation_label: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mutation_journal_entry_ref: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub note: Option<String>,
}

/// Rename/move event kind.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RenameMoveEventKind {
    Created,
    Renamed,
    Moved,
    RenamedAndMoved,
    RestoredFromLocalHistory,
    ReattachedAfterIdentityDrift,
}

/// Branch/worktree context captured for the entry.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BranchWorktreeContext {
    pub context_class: BranchWorktreeContextClass,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub branch_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub worktree_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub base_commit_digest: Option<String>,
    pub parent_commit_digests: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub note: Option<String>,
}

/// Branch/worktree context class.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BranchWorktreeContextClass {
    NoVcsContext,
    GitBranch,
    GitWorktree,
    GitDetachedHead,
    GitRebaseInProgress,
    GitMergeInProgress,
    GitCherryPickInProgress,
    ExternalVcs,
    ReviewWorkspaceBranch,
}

/// Capture descriptor describing what bytes are available for compare/restore.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CaptureDescriptor {
    pub capture_mode: CaptureMode,
    pub omission_reason: CaptureOmissionReasonClass,
    pub body_available: bool,
    pub body_object_refs: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reference_digest: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bytes_estimated: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub omission_note: Option<String>,
}

/// Capture-mode vocabulary for local-history entries.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CaptureMode {
    ContentAddressedSnapshot,
    MetadataPlusReferenceOnly,
    GroupManifestOnly,
    ExternalCauseMetadataOnly,
}

impl CaptureMode {
    /// Returns the schema token for this capture mode.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ContentAddressedSnapshot => "content_addressed_snapshot",
            Self::MetadataPlusReferenceOnly => "metadata_plus_reference_only",
            Self::GroupManifestOnly => "group_manifest_only",
            Self::ExternalCauseMetadataOnly => "external_cause_metadata_only",
        }
    }
}

/// Body-capture omission reason class.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CaptureOmissionReasonClass {
    NotOmitted,
    OmittedTooLarge,
    OmittedBinaryClassMetadataOnly,
    OmittedGeneratedArtifactUseLineage,
    OmittedManagedExternalArtifact,
    OmittedExcludedPath,
    OmittedPolicyRedactedSecretAdjacent,
    OmittedQuotaExceeded,
    OmittedUnsupportedFilesystemSemantics,
}

impl CaptureOmissionReasonClass {
    /// Returns the schema token for this omission reason.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NotOmitted => "not_omitted",
            Self::OmittedTooLarge => "omitted_too_large",
            Self::OmittedBinaryClassMetadataOnly => "omitted_binary_class_metadata_only",
            Self::OmittedGeneratedArtifactUseLineage => "omitted_generated_artifact_use_lineage",
            Self::OmittedManagedExternalArtifact => "omitted_managed_external_artifact",
            Self::OmittedExcludedPath => "omitted_excluded_path",
            Self::OmittedPolicyRedactedSecretAdjacent => "omitted_policy_redacted_secret_adjacent",
            Self::OmittedQuotaExceeded => "omitted_quota_exceeded",
            Self::OmittedUnsupportedFilesystemSemantics => {
                "omitted_unsupported_filesystem_semantics"
            }
        }
    }
}

/// Link back into the mutation journal.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MutationJournalLink {
    pub linked_kind: MutationJournalLinkKind,
    pub linked_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub actor_class: Option<MutationJournalLinkActorClass>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_class: Option<SourceClass>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reversal_class: Option<MutationJournalLinkReversalClass>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub redaction_class: Option<RedactionClass>,
}

/// Actor-class vocabulary for local-history records.
///
/// The local-history timeline reuses the mutation-journal actor-class taxonomy
/// and extends it with a small set of timeline-only authoring actors (for
/// example, external-state observations and restore runners).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MutationJournalLinkActorClass {
    UserKeystroke,
    UserCommand,
    MultiCursorCommand,
    RefactorEngine,
    Formatter,
    SaveParticipant,
    AiApply,
    CodeAction,
    Scaffolding,
    SettingsImport,
    WorkspaceMigration,
    ExternalReload,
    DecodeRecovery,
    BuildRunner,
    CodegenRunner,
    PreviewRegenerator,
    ReviewApply,
    ReplayImport,
    VendorImport,
    PasteOrDropImport,
    AutomationRecipeRunner,
    ExternalChangeDetector,
    RestoreRollbackRunner,
}

impl MutationJournalLinkActorClass {
    /// Returns the schema token for this actor class.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::UserKeystroke => "user_keystroke",
            Self::UserCommand => "user_command",
            Self::MultiCursorCommand => "multi_cursor_command",
            Self::RefactorEngine => "refactor_engine",
            Self::Formatter => "formatter",
            Self::SaveParticipant => "save_participant",
            Self::AiApply => "ai_apply",
            Self::CodeAction => "code_action",
            Self::Scaffolding => "scaffolding",
            Self::SettingsImport => "settings_import",
            Self::WorkspaceMigration => "workspace_migration",
            Self::ExternalReload => "external_reload",
            Self::DecodeRecovery => "decode_recovery",
            Self::BuildRunner => "build_runner",
            Self::CodegenRunner => "codegen_runner",
            Self::PreviewRegenerator => "preview_regenerator",
            Self::ReviewApply => "review_apply",
            Self::ReplayImport => "replay_import",
            Self::VendorImport => "vendor_import",
            Self::PasteOrDropImport => "paste_or_drop_import",
            Self::AutomationRecipeRunner => "automation_recipe_runner",
            Self::ExternalChangeDetector => "external_change_detector",
            Self::RestoreRollbackRunner => "restore_rollback_runner",
        }
    }
}

impl From<JournalActorClass> for MutationJournalLinkActorClass {
    fn from(value: JournalActorClass) -> Self {
        match value {
            JournalActorClass::UserKeystroke => Self::UserKeystroke,
            JournalActorClass::UserCommand => Self::UserCommand,
            JournalActorClass::MultiCursorCommand => Self::MultiCursorCommand,
            JournalActorClass::RefactorEngine => Self::RefactorEngine,
            JournalActorClass::Formatter => Self::Formatter,
            JournalActorClass::SaveParticipant => Self::SaveParticipant,
            JournalActorClass::AiApply => Self::AiApply,
            JournalActorClass::CodeAction => Self::CodeAction,
            JournalActorClass::Scaffolding => Self::Scaffolding,
            JournalActorClass::SettingsImport => Self::SettingsImport,
            JournalActorClass::WorkspaceMigration => Self::WorkspaceMigration,
            JournalActorClass::ExternalReload => Self::ExternalReload,
            JournalActorClass::DecodeRecovery => Self::DecodeRecovery,
            JournalActorClass::BuildRunner => Self::BuildRunner,
            JournalActorClass::CodegenRunner => Self::CodegenRunner,
            JournalActorClass::PreviewRegenerator => Self::PreviewRegenerator,
            JournalActorClass::ReviewApply => Self::ReviewApply,
            JournalActorClass::ReplayImport => Self::ReplayImport,
            JournalActorClass::VendorImport => Self::VendorImport,
        }
    }
}

/// Reversal-class vocabulary for local-history records.
///
/// The local-history timeline reuses the mutation-journal reversal-class
/// taxonomy and adds `no_reversal_external_event` for observation-only entries.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MutationJournalLinkReversalClass {
    ExactUndo,
    CompensatingUndo,
    RegenerateOrRecompute,
    RestoreFromCheckpoint,
    AuditOnly,
    NoReversalExternalEvent,
}

impl MutationJournalLinkReversalClass {
    /// Returns the schema token for this reversal class.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ExactUndo => "exact_undo",
            Self::CompensatingUndo => "compensating_undo",
            Self::RegenerateOrRecompute => "regenerate_or_recompute",
            Self::RestoreFromCheckpoint => "restore_from_checkpoint",
            Self::AuditOnly => "audit_only",
            Self::NoReversalExternalEvent => "no_reversal_external_event",
        }
    }
}

impl From<JournalReversalClass> for MutationJournalLinkReversalClass {
    fn from(value: JournalReversalClass) -> Self {
        match value {
            JournalReversalClass::ExactUndo => Self::ExactUndo,
            JournalReversalClass::CompensatingUndo => Self::CompensatingUndo,
            JournalReversalClass::RegenerateOrRecompute => Self::RegenerateOrRecompute,
            JournalReversalClass::RestoreFromCheckpoint => Self::RestoreFromCheckpoint,
            JournalReversalClass::AuditOnly => Self::AuditOnly,
        }
    }
}

/// Mutation-journal link kind.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MutationJournalLinkKind {
    MutationJournalEntry,
    MutationGroupRecord,
    NoMutationJournalEntryExternalCause,
}

/// Retention scope class for local-history entries.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RetentionScopeClass {
    RetainedByPolicyWindow,
    RetainedByExplicitUserPin,
    RetainedByEvidenceReference,
    RetainedBySupportCaseReference,
    RetainedByReleaseReference,
    RetainedByReviewPackReference,
    StubOnlyAfterExpiry,
    StubOnlyAfterRedaction,
}

/// Local-only posture carried on local-history records.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LocalOnlyPosture {
    pub local_only_by_default: bool,
    pub sync_exclusion: SyncExclusionClass,
    pub ordinary_cache_clear_exclusion: bool,
}

/// Sync exclusion class carried on local-history records.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SyncExclusionClass {
    ExcludedFromSyncByDefault,
    OptInExportOnly,
    SupportBundleByEntryRefOnly,
}

impl LocalHistoryEntryRecord {
    /// Creates a schema-shaped local-history entry record.
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        entry_id: String,
        snapshot_class: SnapshotClass,
        captured_at: String,
        logical_document_identity: LogicalDocumentIdentity,
        capture_descriptor: CaptureDescriptor,
        mutation_journal_link: MutationJournalLink,
        retention_scope: RetentionScopeClass,
        side_effect_summary: Option<String>,
    ) -> Self {
        Self {
            record_kind: "local_history_entry".to_owned(),
            local_history_schema_version: 1,
            entry_id,
            group_id: None,
            snapshot_class,
            captured_at,
            truth_source_class: TruthSourceClass::LocalHistory,
            logical_document_identity,
            branch_worktree_context: BranchWorktreeContext {
                context_class: BranchWorktreeContextClass::NoVcsContext,
                branch_id: None,
                worktree_id: None,
                base_commit_digest: None,
                parent_commit_digests: Vec::new(),
                note: None,
            },
            capture_descriptor,
            mutation_journal_link,
            restore_of_ref: None,
            retention_scope,
            local_only_posture: LocalOnlyPosture {
                local_only_by_default: true,
                sync_exclusion: SyncExclusionClass::ExcludedFromSyncByDefault,
                ordinary_cache_clear_exclusion: true,
            },
            side_effect_summary,
            notes: None,
        }
    }

    /// Assigns a group id to this entry.
    pub fn with_group_id(mut self, group_id: String) -> Self {
        self.group_id = Some(group_id.clone());
        self
    }

    /// Attaches the source checkpoint restored by this entry.
    pub fn with_restore_of_ref(mut self, restore_of_ref: RestoreOfEntryRef) -> Self {
        self.restore_of_ref = Some(restore_of_ref);
        self
    }
}

/// Schema-shaped local-history group record.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LocalHistoryGroupRecord {
    pub record_kind: String,
    pub local_history_schema_version: u32,
    pub group_id: String,
    pub group_kind: LocalHistoryGroupKind,
    pub snapshot_class: SnapshotClass,
    pub opened_at: String,
    pub resolved_at: String,
    pub resolution: LocalHistoryGroupResolution,
    pub truth_source_class: TruthSourceClass,
    pub member_entry_ids: Vec<String>,
    pub mutation_journal_link: MutationJournalLink,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub restore_of_ref: Option<RestoreOfGroupRef>,
    pub retention_scope: RetentionScopeClass,
    pub local_only_posture: LocalOnlyPosture,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub side_effect_summary: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
}

/// Group-kind vocabulary for local-history group records.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LocalHistoryGroupKind {
    RefactorSingleFile,
    RefactorMultiFile,
    FormatOnSave,
    SaveParticipantGroup,
    AiPatch,
    BulkReplace,
    MultiFileRename,
    ScaffoldingRun,
    MigrationImport,
    SettingsImport,
    GeneratedArtifactRefresh,
    PreviewRegeneration,
    ExternalReloadGroup,
    ReviewApplyGroup,
    AutomationRecipeRun,
    PasteOrDropImportGroup,
    RestoreRollbackGroup,
}

/// Resolution vocabulary for local-history group records.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LocalHistoryGroupResolution {
    Applied,
    Aborted,
    Reverted,
    PartiallyAppliedAndRolledBack,
}

/// Restore-of reference carried on restore group records.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RestoreOfGroupRef {
    pub restored_from_group_id: String,
    pub restore_preview: RestorePreviewRequiredFields,
}

/// Entry-level reference to the checkpoint a restore replayed.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RestoreOfEntryRef {
    pub restored_from_entry_id: String,
    pub restore_preview: RestorePreviewRequiredFields,
}

/// Restore preview minimum fields for restore group records.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RestorePreviewRequiredFields {
    pub source_entry_ref: String,
    pub last_known_canonical_identity_ref: String,
    pub current_canonical_identity_ref: String,
    pub canonical_identity_drift: String,
    pub rename_move_chain_ref: String,
    pub body_availability: String,
    pub resulting_snapshot_class: String,
    pub new_checkpoint_entry_ref: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub note: Option<String>,
}

impl LocalHistoryGroupRecord {
    /// Creates a schema-shaped local-history group record.
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        group_id: String,
        group_kind: LocalHistoryGroupKind,
        snapshot_class: SnapshotClass,
        opened_at: String,
        resolved_at: String,
        resolution: LocalHistoryGroupResolution,
        member_entry_ids: Vec<String>,
        mutation_journal_link: MutationJournalLink,
        retention_scope: RetentionScopeClass,
        side_effect_summary: Option<String>,
    ) -> Self {
        Self {
            record_kind: "local_history_group_record".to_owned(),
            local_history_schema_version: 1,
            group_id,
            group_kind,
            snapshot_class,
            opened_at,
            resolved_at,
            resolution,
            truth_source_class: TruthSourceClass::LocalHistory,
            member_entry_ids,
            mutation_journal_link,
            restore_of_ref: None,
            retention_scope,
            local_only_posture: LocalOnlyPosture {
                local_only_by_default: true,
                sync_exclusion: SyncExclusionClass::ExcludedFromSyncByDefault,
                ordinary_cache_clear_exclusion: true,
            },
            side_effect_summary,
            notes: None,
        }
    }

    /// Attaches the source checkpoint restored by this group.
    pub fn with_restore_of_ref(mut self, restore_of_ref: RestoreOfGroupRef) -> Self {
        self.restore_of_ref = Some(restore_of_ref);
        self
    }
}

/// Builds a stable logical-document id for an identity record.
pub fn logical_document_id(identity: &IdentityRecord) -> String {
    let canonical = identity
        .canonical_filesystem_object
        .canonical_uri
        .to_string();
    let digest = blake3::hash(canonical.as_bytes()).to_hex().to_string();
    format!("ld:{digest}")
}

/// Builds a schema-shaped filesystem identity record from a VFS identity record.
pub fn filesystem_identity_record(identity: &IdentityRecord) -> FilesystemIdentityRecord {
    FilesystemIdentityRecord::from_vfs(identity)
}

/// Returns a best-effort actor class for a trust state.
pub fn actor_class_for_trust_state(trust_state: TrustState) -> JournalActorClass {
    match trust_state {
        TrustState::Trusted | TrustState::Restricted | TrustState::PendingEvaluation => {
            JournalActorClass::UserCommand
        }
    }
}

/// Returns a best-effort source class for a trust state.
pub fn source_class_for_trust_state(trust_state: TrustState) -> SourceClass {
    match trust_state {
        TrustState::Trusted | TrustState::Restricted | TrustState::PendingEvaluation => {
            SourceClass::HumanLocal
        }
    }
}

/// Returns a stable alias-kind string.
pub fn alias_kind_string(kind: AliasKind) -> &'static str {
    kind.as_str()
}
