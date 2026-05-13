//! Unified mutation-journal record writer.
//!
//! The mutation journal is the canonical lineage vocabulary shared by local
//! history, recovery, review, and support surfaces.

use std::path::PathBuf;

use serde::{Deserialize, Serialize};

use crate::storage::{HistoryError, HistoryStorageRoot, IdSource};

/// Writer for mutation-journal entries and group records.
#[derive(Debug, Clone)]
pub struct MutationJournalStore {
    storage: HistoryStorageRoot,
    root: PathBuf,
    entry_ids: IdSource,
    group_ids: IdSource,
}

impl MutationJournalStore {
    /// Creates a store rooted at `root/mutation_journal`.
    pub fn new(storage: HistoryStorageRoot) -> Self {
        let root = storage.path().join("mutation_journal");
        Self {
            storage,
            root,
            entry_ids: IdSource::new("m"),
            group_ids: IdSource::new("g"),
        }
    }

    /// Mints a new mutation id.
    pub fn mint_mutation_id(&mut self) -> String {
        self.entry_ids.mint()
    }

    /// Mints a new group id.
    pub fn mint_group_id(&mut self) -> String {
        self.group_ids.mint()
    }

    /// Persists a mutation-journal entry record.
    pub fn write_entry(&self, entry: &MutationJournalEntryRecord) -> Result<PathBuf, HistoryError> {
        let path = self
            .root
            .join("entries")
            .join(format!("{}.json", sanitize_id(&entry.mutation_id)));
        self.storage.write_new_json(&path, entry)?;
        Ok(path)
    }

    /// Persists a mutation-group record.
    pub fn write_group(&self, group: &MutationGroupRecord) -> Result<PathBuf, HistoryError> {
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

/// Schema-shaped mutation-journal entry record.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MutationJournalEntryRecord {
    pub record_kind: String,
    pub mutation_journal_schema_version: u32,
    pub mutation_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_id: Option<String>,
    pub command_id: String,
    pub actor_class: ActorClass,
    pub source_class: SourceClass,
    pub actor_ref: ActorRef,
    pub scope_ref: ScopeRef,
    pub target_refs: Vec<TargetRef>,
    pub started_at: String,
    pub committed_at: String,
    pub undo_class: String,
    pub reversal_class: ReversalClass,
    pub reversibility: Reversibility,
    pub checkpoint_refs: Vec<CheckpointRef>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_context: Option<PolicyContext>,
    pub redaction_class: RedactionClass,
    pub durable_vs_disposable: DurableVsDisposable,
    pub side_effect_summary: SideEffectSummary,
}

impl MutationJournalEntryRecord {
    /// Creates a schema-shaped mutation-journal entry record.
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        mutation_id: String,
        command_id: String,
        actor_class: ActorClass,
        source_class: SourceClass,
        actor_ref: ActorRef,
        scope_ref: ScopeRef,
        target_refs: Vec<TargetRef>,
        started_at: String,
        committed_at: String,
        undo_class: String,
        reversal_class: ReversalClass,
        redaction_class: RedactionClass,
        durable_vs_disposable: DurableVsDisposable,
        side_effect_summary: SideEffectSummary,
        checkpoint_refs: Vec<CheckpointRef>,
    ) -> Self {
        let reversible = !matches!(reversal_class, ReversalClass::AuditOnly);
        Self {
            record_kind: "mutation_journal_entry".to_owned(),
            mutation_journal_schema_version: 1,
            mutation_id,
            group_id: None,
            command_id,
            actor_class,
            source_class,
            actor_ref,
            scope_ref,
            target_refs,
            started_at: started_at.clone(),
            committed_at,
            undo_class,
            reversal_class,
            reversibility: Reversibility {
                reversible,
                declared_at_commit: true,
                downgrade_reason: None,
            },
            checkpoint_refs,
            policy_context: None,
            redaction_class,
            durable_vs_disposable,
            side_effect_summary,
        }
    }

    /// Assigns a group id to this entry.
    pub fn with_group_id(mut self, group_id: String) -> Self {
        self.group_id = Some(group_id);
        self
    }
}

/// Schema-shaped mutation-group record.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MutationGroupRecord {
    pub record_kind: String,
    pub mutation_journal_schema_version: u32,
    pub group_id: String,
    pub group_kind: MutationGroupKind,
    pub command_id: String,
    pub actor_class: ActorClass,
    pub source_class: SourceClass,
    pub actor_ref: ActorRef,
    pub scope_ref: ScopeRef,
    pub opened_at: String,
    pub resolved_at: String,
    pub resolution: MutationGroupResolution,
    pub member_mutation_ids: Vec<String>,
    pub reversal_class: ReversalClass,
    pub reversibility: Reversibility,
    pub redaction_class: RedactionClass,
    pub durable_vs_disposable: DurableVsDisposable,
    pub side_effect_summary: SideEffectSummary,
    pub checkpoint_refs: Vec<CheckpointRef>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_context: Option<PolicyContext>,
}

impl MutationGroupRecord {
    /// Creates a schema-shaped mutation-group record.
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        group_id: String,
        group_kind: MutationGroupKind,
        command_id: String,
        actor_class: ActorClass,
        source_class: SourceClass,
        actor_ref: ActorRef,
        scope_ref: ScopeRef,
        opened_at: String,
        resolved_at: String,
        resolution: MutationGroupResolution,
        member_mutation_ids: Vec<String>,
        reversal_class: ReversalClass,
        redaction_class: RedactionClass,
        durable_vs_disposable: DurableVsDisposable,
        side_effect_summary: SideEffectSummary,
        checkpoint_refs: Vec<CheckpointRef>,
    ) -> Self {
        let reversible = !matches!(reversal_class, ReversalClass::AuditOnly);
        Self {
            record_kind: "mutation_group_record".to_owned(),
            mutation_journal_schema_version: 1,
            group_id,
            group_kind,
            command_id,
            actor_class,
            source_class,
            actor_ref,
            scope_ref,
            opened_at: opened_at.clone(),
            resolved_at,
            resolution,
            member_mutation_ids,
            reversal_class,
            reversibility: Reversibility {
                reversible,
                declared_at_commit: true,
                downgrade_reason: None,
            },
            redaction_class,
            durable_vs_disposable,
            side_effect_summary,
            checkpoint_refs,
            policy_context: None,
        }
    }
}

/// Stable actor reference carried on journal entries.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ActorRef {
    pub display_name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stable_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role: Option<String>,
}

/// Scope within which a mutation applied.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ScopeRef {
    pub class: ScopeClass,
    pub id: String,
}

/// Scope-class vocabulary for journal entries.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ScopeClass {
    Workspace,
    Root,
    Workset,
    Slice,
    Window,
    Buffer,
    File,
    ReviewWorkspace,
    RemoteSession,
    CompanionSurface,
    SettingsScope,
}

/// One mutation target reference.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TargetRef {
    pub target_kind: TargetKind,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filesystem_identity: Option<crate::checkpoints::FilesystemIdentityRecord>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logical_ref: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub affected_range: Option<AffectedRange>,
}

/// Target-kind vocabulary for journal entries.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TargetKind {
    FilesystemObject,
    Buffer,
    WorkspaceSetting,
    WorkspaceManifest,
    TaskConfig,
    LaunchConfig,
    PolicyDocument,
    GeneratedArtifact,
    ExternalService,
}

/// Optional affected range inside a target.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AffectedRange {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub byte_start: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub byte_end: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line_start: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line_end: Option<u64>,
}

/// Actor-class vocabulary for journal entries.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ActorClass {
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
}

impl ActorClass {
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
        }
    }
}

/// Source-class vocabulary for journal entries.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SourceClass {
    HumanLocal,
    HumanRemoteSession,
    MachineLocal,
    MachineRemoteAgent,
    AiLocalModel,
    AiHostedProvider,
    ImportedExternal,
    ReplayedCapture,
    PolicyDriven,
}

impl SourceClass {
    /// Returns the schema token for this source class.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::HumanLocal => "human_local",
            Self::HumanRemoteSession => "human_remote_session",
            Self::MachineLocal => "machine_local",
            Self::MachineRemoteAgent => "machine_remote_agent",
            Self::AiLocalModel => "ai_local_model",
            Self::AiHostedProvider => "ai_hosted_provider",
            Self::ImportedExternal => "imported_external",
            Self::ReplayedCapture => "replayed_capture",
            Self::PolicyDriven => "policy_driven",
        }
    }
}

/// Reversal-class vocabulary for journal entries.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ReversalClass {
    ExactUndo,
    CompensatingUndo,
    RegenerateOrRecompute,
    RestoreFromCheckpoint,
    AuditOnly,
}

impl ReversalClass {
    /// Returns the schema token for this reversal class.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ExactUndo => "exact_undo",
            Self::CompensatingUndo => "compensating_undo",
            Self::RegenerateOrRecompute => "regenerate_or_recompute",
            Self::RestoreFromCheckpoint => "restore_from_checkpoint",
            Self::AuditOnly => "audit_only",
        }
    }
}

/// Redaction-class vocabulary for journal entries.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RedactionClass {
    MetadataOnly,
    EnvironmentAdjacent,
    CodeAdjacent,
    HighRisk,
}

impl RedactionClass {
    /// Returns the schema token for this redaction class.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::MetadataOnly => "metadata_only",
            Self::EnvironmentAdjacent => "environment_adjacent",
            Self::CodeAdjacent => "code_adjacent",
            Self::HighRisk => "high_risk",
        }
    }
}

/// Durable-vs-disposable classification for journal entries.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DurableVsDisposable {
    DurableUserAuthored,
    DurableWorkspaceAuthored,
    DisposableDerived,
}

/// Reversibility flags carried on journal entries and group records.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Reversibility {
    pub reversible: bool,
    pub declared_at_commit: bool,
    pub downgrade_reason: Option<String>,
}

/// Link to a checkpoint cited by a mutation-journal record.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CheckpointRef {
    pub checkpoint_kind: CheckpointKind,
    pub checkpoint_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub durability_class: Option<CheckpointDurabilityClass>,
}

/// Checkpoint-kind vocabulary for journal entries.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CheckpointKind {
    RecoveryJournal,
    SaveManifest,
    WorkspaceMigration,
    SettingsBackup,
    MutationGroupPreview,
    LocalHistorySnapshot,
    ReviewCheckpoint,
}

impl CheckpointKind {
    /// Returns the schema token for this checkpoint kind.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::RecoveryJournal => "recovery_journal",
            Self::SaveManifest => "save_manifest",
            Self::WorkspaceMigration => "workspace_migration",
            Self::SettingsBackup => "settings_backup",
            Self::MutationGroupPreview => "mutation_group_preview",
            Self::LocalHistorySnapshot => "local_history_snapshot",
            Self::ReviewCheckpoint => "review_checkpoint",
        }
    }
}

/// Durability-class vocabulary for checkpoint refs.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CheckpointDurabilityClass {
    Durable,
    Disposable,
}

/// Optional policy context snapshot.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PolicyContext {
    pub policy_epoch: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trust_state: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_scope: Option<String>,
}

/// Side-effect summary carried on journal entries.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SideEffectSummary {
    pub summary: String,
    pub external_targets: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bytes_written: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub files_touched: Option<u64>,
}

impl SideEffectSummary {
    /// Creates a side-effect summary with a short text label.
    pub fn new(summary: impl Into<String>) -> Self {
        Self {
            summary: summary.into(),
            external_targets: Vec::new(),
            bytes_written: None,
            files_touched: None,
        }
    }
}

/// Named group kinds for mutation-group records.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MutationGroupKind {
    MultiCursorKeystroke,
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
}

/// Resolution vocabulary for mutation-group records.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MutationGroupResolution {
    Applied,
    Aborted,
    Reverted,
    PartiallyAppliedAndRolledBack,
}
