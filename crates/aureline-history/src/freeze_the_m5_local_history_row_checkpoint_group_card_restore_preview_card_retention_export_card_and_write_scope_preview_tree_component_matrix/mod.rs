//! Frozen M5 local-history-row, checkpoint-group-card, restore-preview-card,
//! retention/export-card, write-scope-preview-tree, restore-granularity-selector,
//! and history-export-manifest component matrix.
//!
//! This module locks Aureline's reusable local-history and write-scope components
//! into one export-safe packet. Every mutation- and recovery-facing subcomponent M5
//! claims that still drifts too easily by editor local-history timeline, checkpoint
//! inspector, restore-review sheet, refactor preview, AI-apply review, recovery
//! center, or support-desk surface — the local-history row, the checkpoint-group
//! card, the restore-preview card, the retention/export card, the write-scope
//! preview tree, the restore-granularity selector, and the history-export manifest —
//! is named once here and constrained by the same timestamp, actor/source,
//! file-or-object identity, branch/worktree context, mutation class, checkpoint
//! lineage, restore granularity, generated-or-managed-file caveat, and
//! export/redaction rules regardless of the surface family that renders it.
//!
//! What this matrix freezes is the stable vocabulary for the *components*
//! themselves: the component families, the snapshot origins, actor classes, and
//! capture fidelities, the checkpoint lineage classes and mutation classes, the
//! restore granularities and drift states, the retention postures and
//! export/redaction postures, the write-scope classes and managed-file caveats, the
//! restore-selection modes, the export-manifest classes, the deployment lines every
//! component must survive, the non-visual accessibility routes, and the mandatory
//! labels every component must be able to show. It does not re-architect
//! mutation-journal storage, Git history, or repair-transaction engines that already
//! own those records — it is the shared local-history / write-scope contract layered
//! on top of them.
//!
//! The matrix is the single source of truth for whether a claimed M5 editor,
//! refactor, AI, import, repair, review, or support surface may publish a snapshot,
//! actor, checkpoint, restore-scope, write-scope, retention, or export claim. Editor,
//! checkpoint, restore, refactor, AI-apply, recovery, and support consumers all read
//! this packet so one local-history row names when a snapshot was captured and by
//! whom, one checkpoint-group card names its lineage and mutation class, one
//! restore-preview card names the granularity and drift it will restore into, one
//! retention/export card names how long history is kept and how it redacts on export,
//! one write-scope preview tree names how wide an apply reaches and which
//! generated-or-managed files it touches, one restore-granularity selector names the
//! selectable apply scope, and one history-export manifest names what an export
//! bundle contains and how it is redacted. No M5 lane invents a second history
//! grammar, masks an actor or timestamp, hides a generated-or-managed-file caveat, or
//! bypasses the restore-scope review.
//!
//! The controlled vocabularies are frozen in one self-describing
//! [`M5LocalHistoryWriteScopeComponentVocabularySet`] rather than minted per surface.
//! Raw snapshot bodies, restored file contents, pasted paths, credentials, and
//! private endpoints stay outside the support boundary.

mod seed;
#[cfg(test)]
mod tests;

pub use seed::{
    seeded_m5_local_history_write_scope_component_matrix,
    seeded_m5_local_history_write_scope_component_matrix_history_export_manifest_preview_narrowed,
    seeded_m5_local_history_write_scope_component_matrix_write_scope_preview_tree_beta_narrowed,
    M5_LOCAL_HISTORY_WRITE_SCOPE_COMPONENT_MATRIX_PACKET_ID,
};

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

/// Stable record-kind tag carried by [`M5LocalHistoryWriteScopeComponentMatrixPacket`].
pub const M5_LOCAL_HISTORY_WRITE_SCOPE_COMPONENT_MATRIX_RECORD_KIND: &str =
    "freeze_m5_local_history_row_checkpoint_group_card_restore_preview_card_retention_export_card_write_scope_preview_tree_restore_granularity_selector_and_history_export_manifest_component_matrix";

/// Schema version for M5 local-history / write-scope component-matrix records.
pub const M5_LOCAL_HISTORY_WRITE_SCOPE_COMPONENT_MATRIX_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the local-history / write-scope component boundary schema.
pub const M5_LOCAL_HISTORY_WRITE_SCOPE_COMPONENT_SCHEMA_REF: &str =
    "schemas/ui/m5-local-history-write-scope-component-matrix.schema.json";

/// Repo-relative path of the contract doc.
pub const M5_LOCAL_HISTORY_WRITE_SCOPE_COMPONENT_DOC_REF: &str =
    "docs/recovery/m5_local_history_write_scope_component_matrix.md";

/// Repo-relative path of the local-history-entry contract this matrix binds against.
pub const M5_LOCAL_HISTORY_WRITE_SCOPE_COMPONENT_HISTORY_ENTRY_REF: &str =
    "schemas/recovery/local_history_entry.schema.json";

/// Repo-relative path of the checkpoint-inventory contract this matrix binds against.
pub const M5_LOCAL_HISTORY_WRITE_SCOPE_COMPONENT_CHECKPOINT_REF: &str =
    "schemas/recovery/checkpoint_inventory.schema.json";

/// Repo-relative path of the restore-preview contract this matrix binds against.
pub const M5_LOCAL_HISTORY_WRITE_SCOPE_COMPONENT_RESTORE_PREVIEW_REF: &str =
    "schemas/recovery/restore_preview.schema.json";

/// Repo-relative path of the retention-card contract this matrix binds against.
pub const M5_LOCAL_HISTORY_WRITE_SCOPE_COMPONENT_RETENTION_REF: &str =
    "schemas/recovery/local_history_retention_card.schema.json";

/// Repo-relative path of the write-boundary contract this matrix binds against.
pub const M5_LOCAL_HISTORY_WRITE_SCOPE_COMPONENT_WRITE_BOUNDARY_REF: &str =
    "schemas/generated/write-boundary.schema.json";

/// Repo-relative path of the protected fixture directory.
pub const M5_LOCAL_HISTORY_WRITE_SCOPE_COMPONENT_FIXTURE_DIR: &str =
    "fixtures/ui/m5-local-history-write-scope-components";

/// Repo-relative path of the checked support-export artifact.
pub const M5_LOCAL_HISTORY_WRITE_SCOPE_COMPONENT_ARTIFACT_REF: &str =
    "artifacts/release/m5-local-history-write-scope-component-proof/support_export.json";

/// Repo-relative path of the checked machine-readable matrix CSV.
pub const M5_LOCAL_HISTORY_WRITE_SCOPE_COMPONENT_CSV_REF: &str =
    "artifacts/release/m5-local-history-write-scope-component-proof/matrix.csv";

/// Repo-relative path of the checked Markdown design report.
pub const M5_LOCAL_HISTORY_WRITE_SCOPE_COMPONENT_REPORT_REF: &str =
    "artifacts/design/m5-local-history-write-scope-component-matrix.md";

/// One of the seven governed local-history / write-scope component families this
/// matrix freezes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5LocalHistoryWriteScopeComponentFamily {
    /// A local-history row carrying a snapshot origin, actor, and capture fidelity.
    LocalHistoryRow,
    /// A checkpoint-group card carrying checkpoint lineage and its mutation class.
    CheckpointGroupCard,
    /// A restore-preview card carrying restore granularity and drift state.
    RestorePreviewCard,
    /// A retention/export card carrying retention posture and export redaction.
    RetentionExportCard,
    /// A write-scope preview tree carrying the write-scope class and managed-file
    /// caveat.
    WriteScopePreviewTree,
    /// A restore-granularity selector carrying the selectable apply scope.
    RestoreGranularitySelector,
    /// A history-export manifest carrying the export class and its redaction.
    HistoryExportManifest,
}

impl M5LocalHistoryWriteScopeComponentFamily {
    /// Every governed component family, in declaration order.
    pub const ALL: [Self; 7] = [
        Self::LocalHistoryRow,
        Self::CheckpointGroupCard,
        Self::RestorePreviewCard,
        Self::RetentionExportCard,
        Self::WriteScopePreviewTree,
        Self::RestoreGranularitySelector,
        Self::HistoryExportManifest,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LocalHistoryRow => "local_history_row",
            Self::CheckpointGroupCard => "checkpoint_group_card",
            Self::RestorePreviewCard => "restore_preview_card",
            Self::RetentionExportCard => "retention_export_card",
            Self::WriteScopePreviewTree => "write_scope_preview_tree",
            Self::RestoreGranularitySelector => "restore_granularity_selector",
            Self::HistoryExportManifest => "history_export_manifest",
        }
    }

    /// `true` when this family is a local-history row and must therefore declare its
    /// snapshot origins, actor classes, and capture fidelities.
    pub const fn is_local_history_row(self) -> bool {
        matches!(self, Self::LocalHistoryRow)
    }

    /// `true` when this family is a checkpoint-group card and must therefore declare
    /// its checkpoint lineage classes and mutation classes.
    pub const fn is_checkpoint_group_card(self) -> bool {
        matches!(self, Self::CheckpointGroupCard)
    }

    /// `true` when this family is a restore-preview card and must therefore declare
    /// its restore granularities and drift states.
    pub const fn is_restore_preview_card(self) -> bool {
        matches!(self, Self::RestorePreviewCard)
    }

    /// `true` when this family is a retention/export card and must therefore declare
    /// its retention postures and export-redaction postures.
    pub const fn is_retention_export_card(self) -> bool {
        matches!(self, Self::RetentionExportCard)
    }

    /// `true` when this family is a write-scope preview tree and must therefore
    /// declare its write-scope classes and managed-file caveats.
    pub const fn is_write_scope_preview_tree(self) -> bool {
        matches!(self, Self::WriteScopePreviewTree)
    }

    /// `true` when this family is a restore-granularity selector and must therefore
    /// declare its restore-selection modes.
    pub const fn is_restore_granularity_selector(self) -> bool {
        matches!(self, Self::RestoreGranularitySelector)
    }

    /// `true` when this family is a history-export manifest and must therefore
    /// declare its export-manifest classes.
    pub const fn is_history_export_manifest(self) -> bool {
        matches!(self, Self::HistoryExportManifest)
    }
}

/// Controlled snapshot origin — what produced a local-history snapshot, so a row
/// never leaves who or what created the checkpoint implicit.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5SnapshotOrigin {
    /// An explicit manual save.
    ManualSave,
    /// A periodic autosave.
    Autosave,
    /// A formatter run.
    FormatterRun,
    /// A refactor apply.
    RefactorApply,
    /// An AI apply.
    AiApply,
    /// An external import / sync.
    ExternalImport,
}

impl M5SnapshotOrigin {
    /// Every snapshot origin, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::ManualSave,
        Self::Autosave,
        Self::FormatterRun,
        Self::RefactorApply,
        Self::AiApply,
        Self::ExternalImport,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ManualSave => "manual_save",
            Self::Autosave => "autosave",
            Self::FormatterRun => "formatter_run",
            Self::RefactorApply => "refactor_apply",
            Self::AiApply => "ai_apply",
            Self::ExternalImport => "external_import",
        }
    }
}

/// Controlled actor class — who or what authored a snapshot, so a row never leaves
/// the actor lineage implicit.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5HistoryActorClass {
    /// The local user.
    LocalUser,
    /// A pair-programming participant.
    PairParticipant,
    /// An AI agent.
    AiAgent,
    /// An automation task.
    AutomationTask,
    /// An import bridge.
    ImportBridge,
    /// An unknown / unattributed actor.
    UnknownActor,
}

impl M5HistoryActorClass {
    /// Every actor class, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::LocalUser,
        Self::PairParticipant,
        Self::AiAgent,
        Self::AutomationTask,
        Self::ImportBridge,
        Self::UnknownActor,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LocalUser => "local_user",
            Self::PairParticipant => "pair_participant",
            Self::AiAgent => "ai_agent",
            Self::AutomationTask => "automation_task",
            Self::ImportBridge => "import_bridge",
            Self::UnknownActor => "unknown_actor",
        }
    }
}

/// Controlled capture fidelity — how much of a snapshot was actually captured, so a
/// metadata-only capture is never shown as a full-body snapshot that could restore.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5CaptureFidelity {
    /// A full-body snapshot.
    FullBodySnapshot,
    /// A metadata-only capture (no body).
    MetadataOnly,
    /// A diff-only capture.
    DiffOnly,
    /// A pointer / content-address reference.
    PointerReference,
    /// An external reference held elsewhere.
    ExternalReference,
    /// A redacted capture with omitted regions.
    RedactedCapture,
}

impl M5CaptureFidelity {
    /// Every capture fidelity, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::FullBodySnapshot,
        Self::MetadataOnly,
        Self::DiffOnly,
        Self::PointerReference,
        Self::ExternalReference,
        Self::RedactedCapture,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::FullBodySnapshot => "full_body_snapshot",
            Self::MetadataOnly => "metadata_only",
            Self::DiffOnly => "diff_only",
            Self::PointerReference => "pointer_reference",
            Self::ExternalReference => "external_reference",
            Self::RedactedCapture => "redacted_capture",
        }
    }
}

/// Controlled checkpoint lineage class — how a checkpoint-group was formed, so a card
/// never collapses a grouped transaction or session-restore point into a single edit.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5CheckpointLineageClass {
    /// A single-action checkpoint.
    SingleAction,
    /// A grouped transaction of several actions.
    GroupedTransaction,
    /// A session-restore point.
    SessionRestorePoint,
    /// A named milestone tag.
    MilestoneTag,
    /// A rollback point.
    RollbackPoint,
    /// An imported checkpoint.
    ImportedCheckpoint,
}

impl M5CheckpointLineageClass {
    /// Every checkpoint lineage class, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::SingleAction,
        Self::GroupedTransaction,
        Self::SessionRestorePoint,
        Self::MilestoneTag,
        Self::RollbackPoint,
        Self::ImportedCheckpoint,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SingleAction => "single_action",
            Self::GroupedTransaction => "grouped_transaction",
            Self::SessionRestorePoint => "session_restore_point",
            Self::MilestoneTag => "milestone_tag",
            Self::RollbackPoint => "rollback_point",
            Self::ImportedCheckpoint => "imported_checkpoint",
        }
    }
}

/// Controlled mutation class — what kind of change a checkpoint captured, so a card
/// never leaves the mutation class behind a checkpoint implicit.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5MutationClass {
    /// A plain text edit.
    TextEdit,
    /// A multi-file refactor.
    MultiFileRefactor,
    /// A generated-artifact write.
    GeneratedArtifact,
    /// A dependency change.
    DependencyChange,
    /// A repair transaction.
    RepairTransaction,
    /// A config migration.
    ConfigMigration,
}

impl M5MutationClass {
    /// Every mutation class, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::TextEdit,
        Self::MultiFileRefactor,
        Self::GeneratedArtifact,
        Self::DependencyChange,
        Self::RepairTransaction,
        Self::ConfigMigration,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::TextEdit => "text_edit",
            Self::MultiFileRefactor => "multi_file_refactor",
            Self::GeneratedArtifact => "generated_artifact",
            Self::DependencyChange => "dependency_change",
            Self::RepairTransaction => "repair_transaction",
            Self::ConfigMigration => "config_migration",
        }
    }
}

/// Controlled restore granularity — how much a restore-preview card will restore, so
/// a partial or manual restore is never shown as a whole-snapshot restore.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5RestoreGranularity {
    /// The whole snapshot.
    WholeSnapshot,
    /// Per-file restore.
    PerFile,
    /// Per-hunk restore.
    PerHunk,
    /// Per-symbol restore.
    PerSymbol,
    /// The current selection only.
    SelectionOnly,
    /// A manual merge.
    ManualMerge,
}

impl M5RestoreGranularity {
    /// Every restore granularity, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::WholeSnapshot,
        Self::PerFile,
        Self::PerHunk,
        Self::PerSymbol,
        Self::SelectionOnly,
        Self::ManualMerge,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WholeSnapshot => "whole_snapshot",
            Self::PerFile => "per_file",
            Self::PerHunk => "per_hunk",
            Self::PerSymbol => "per_symbol",
            Self::SelectionOnly => "selection_only",
            Self::ManualMerge => "manual_merge",
        }
    }
}

/// Controlled restore drift state — how the target diverged since capture, so a
/// restore-preview card never restores over local edits or a moved / deleted file
/// silently.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5RestoreDriftState {
    /// A clean apply.
    CleanApply,
    /// Local edits are present.
    LocalEditsPresent,
    /// The source was moved.
    SourceMoved,
    /// The source was deleted.
    SourceDeleted,
    /// External drift is present.
    ExternalDrift,
    /// A conflict is pending resolution.
    ConflictPending,
}

impl M5RestoreDriftState {
    /// Every restore drift state, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::CleanApply,
        Self::LocalEditsPresent,
        Self::SourceMoved,
        Self::SourceDeleted,
        Self::ExternalDrift,
        Self::ConflictPending,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::CleanApply => "clean_apply",
            Self::LocalEditsPresent => "local_edits_present",
            Self::SourceMoved => "source_moved",
            Self::SourceDeleted => "source_deleted",
            Self::ExternalDrift => "external_drift",
            Self::ConflictPending => "conflict_pending",
        }
    }
}

/// Controlled retention posture — how long local history is kept, so a
/// retention/export card never shows a purge-pending or expired history as retained.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5RetentionPosture {
    /// Session-only.
    SessionOnly,
    /// Workspace-retained.
    WorkspaceRetained,
    /// Account-synced.
    AccountSynced,
    /// Policy-pinned.
    PolicyPinned,
    /// Purge-pending.
    PurgePending,
    /// Expired and purged.
    ExpiredPurged,
}

impl M5RetentionPosture {
    /// Every retention posture, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::SessionOnly,
        Self::WorkspaceRetained,
        Self::AccountSynced,
        Self::PolicyPinned,
        Self::PurgePending,
        Self::ExpiredPurged,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SessionOnly => "session_only",
            Self::WorkspaceRetained => "workspace_retained",
            Self::AccountSynced => "account_synced",
            Self::PolicyPinned => "policy_pinned",
            Self::PurgePending => "purge_pending",
            Self::ExpiredPurged => "expired_purged",
        }
    }
}

/// Controlled export-redaction posture — how a history export redacts, so a
/// retention/export card or manifest never shows a redacted export as a full export.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ExportRedactionPosture {
    /// Full metadata export.
    FullMetadata,
    /// Paths redacted.
    PathsRedacted,
    /// Bodies omitted.
    BodiesOmitted,
    /// Credentials scrubbed.
    CredentialsScrubbed,
    /// Restricted by policy.
    PolicyRestricted,
    /// Export blocked.
    ExportBlocked,
}

impl M5ExportRedactionPosture {
    /// Every export-redaction posture, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::FullMetadata,
        Self::PathsRedacted,
        Self::BodiesOmitted,
        Self::CredentialsScrubbed,
        Self::PolicyRestricted,
        Self::ExportBlocked,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::FullMetadata => "full_metadata",
            Self::PathsRedacted => "paths_redacted",
            Self::BodiesOmitted => "bodies_omitted",
            Self::CredentialsScrubbed => "credentials_scrubbed",
            Self::PolicyRestricted => "policy_restricted",
            Self::ExportBlocked => "export_blocked",
        }
    }
}

/// Controlled write-scope class — how wide a write-scope preview tree reaches, so a
/// preview never understates the blast radius of a multi-file or out-of-workspace
/// apply.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5WriteScopeClass {
    /// A single file.
    SingleFile,
    /// Several files.
    MultiFile,
    /// A whole directory.
    WholeDirectory,
    /// Across packages.
    CrossPackage,
    /// A generated tree.
    GeneratedTree,
    /// Out of the workspace.
    OutOfWorkspace,
}

impl M5WriteScopeClass {
    /// Every write-scope class, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::SingleFile,
        Self::MultiFile,
        Self::WholeDirectory,
        Self::CrossPackage,
        Self::GeneratedTree,
        Self::OutOfWorkspace,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SingleFile => "single_file",
            Self::MultiFile => "multi_file",
            Self::WholeDirectory => "whole_directory",
            Self::CrossPackage => "cross_package",
            Self::GeneratedTree => "generated_tree",
            Self::OutOfWorkspace => "out_of_workspace",
        }
    }
}

/// Controlled managed-file caveat — how a file in a write-scope is managed, so a
/// preview never restores or applies over a generated, managed, or protected file
/// without saying so.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ManagedFileCaveat {
    /// An unmanaged plain file.
    Unmanaged,
    /// A generated file.
    GeneratedFile,
    /// A managed lockfile.
    ManagedLockfile,
    /// A vendored dependency.
    VendoredDependency,
    /// A protected read-only path.
    ProtectedReadonly,
    /// An ignored path.
    IgnoredPath,
}

impl M5ManagedFileCaveat {
    /// Every managed-file caveat, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::Unmanaged,
        Self::GeneratedFile,
        Self::ManagedLockfile,
        Self::VendoredDependency,
        Self::ProtectedReadonly,
        Self::IgnoredPath,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Unmanaged => "unmanaged",
            Self::GeneratedFile => "generated_file",
            Self::ManagedLockfile => "managed_lockfile",
            Self::VendoredDependency => "vendored_dependency",
            Self::ProtectedReadonly => "protected_readonly",
            Self::IgnoredPath => "ignored_path",
        }
    }
}

/// Controlled restore-selection mode — the selectable apply scope a
/// restore-granularity selector offers, so scope narrowing is a first-class choice,
/// never collapsed into an all-or-nothing apply.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5RestoreSelectionMode {
    /// Apply all changes.
    AllChanges,
    /// Choose files.
    ChooseFiles,
    /// Choose hunks.
    ChooseHunks,
    /// Choose symbols.
    ChooseSymbols,
    /// Exclude generated files.
    ExcludeGenerated,
    /// Dry-run only.
    DryRunOnly,
}

impl M5RestoreSelectionMode {
    /// Every restore-selection mode, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::AllChanges,
        Self::ChooseFiles,
        Self::ChooseHunks,
        Self::ChooseSymbols,
        Self::ExcludeGenerated,
        Self::DryRunOnly,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::AllChanges => "all_changes",
            Self::ChooseFiles => "choose_files",
            Self::ChooseHunks => "choose_hunks",
            Self::ChooseSymbols => "choose_symbols",
            Self::ExcludeGenerated => "exclude_generated",
            Self::DryRunOnly => "dry_run_only",
        }
    }
}

/// Controlled export-manifest class — what a history-export manifest bundles, so a
/// support, audit, or migration export is never mislabelled as a plain share.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ExportManifestClass {
    /// A support bundle.
    SupportBundle,
    /// A recovery-evidence bundle.
    RecoveryEvidence,
    /// An audit trail.
    AuditTrail,
    /// A migration session.
    MigrationSession,
    /// An offline mirror.
    OfflineMirror,
    /// A redacted share.
    RedactedShare,
}

impl M5ExportManifestClass {
    /// Every export-manifest class, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::SupportBundle,
        Self::RecoveryEvidence,
        Self::AuditTrail,
        Self::MigrationSession,
        Self::OfflineMirror,
        Self::RedactedShare,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SupportBundle => "support_bundle",
            Self::RecoveryEvidence => "recovery_evidence",
            Self::AuditTrail => "audit_trail",
            Self::MigrationSession => "migration_session",
            Self::OfflineMirror => "offline_mirror",
            Self::RedactedShare => "redacted_share",
        }
    }
}

/// Claimed M5 mutation / recovery surface family that renders / consumes a
/// local-history or write-scope component. No component may invent a parallel surface
/// taxonomy.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5HistorySurfaceFamily {
    /// The editor local-history timeline surface.
    EditorLocalHistory,
    /// The checkpoint-inspector surface.
    CheckpointInspector,
    /// The restore-review-sheet surface.
    RestoreReviewSheet,
    /// The refactor-preview surface.
    RefactorPreview,
    /// The AI-apply-review surface.
    AiApplyReview,
    /// The recovery-center surface.
    RecoveryCenter,
    /// The support-desk surface.
    SupportDesk,
}

impl M5HistorySurfaceFamily {
    /// Every surface family, in declaration order.
    pub const ALL: [Self; 7] = [
        Self::EditorLocalHistory,
        Self::CheckpointInspector,
        Self::RestoreReviewSheet,
        Self::RefactorPreview,
        Self::AiApplyReview,
        Self::RecoveryCenter,
        Self::SupportDesk,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::EditorLocalHistory => "editor_local_history",
            Self::CheckpointInspector => "checkpoint_inspector",
            Self::RestoreReviewSheet => "restore_review_sheet",
            Self::RefactorPreview => "refactor_preview",
            Self::AiApplyReview => "ai_apply_review",
            Self::RecoveryCenter => "recovery_center",
            Self::SupportDesk => "support_desk",
        }
    }
}

/// Deployment line a component must survive with the same truth, so a component's
/// actor, scope, or retention truth never silently narrows or widens between
/// deployment shapes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5HistoryDeploymentLine {
    /// The local open-source line.
    LocalOss,
    /// The self-hosted line.
    SelfHosted,
    /// The managed line.
    Managed,
    /// The air-gapped line.
    AirGapped,
    /// The mirror / offline line.
    MirrorOffline,
}

impl M5HistoryDeploymentLine {
    /// Every deployment line, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::LocalOss,
        Self::SelfHosted,
        Self::Managed,
        Self::AirGapped,
        Self::MirrorOffline,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LocalOss => "local_oss",
            Self::SelfHosted => "self_hosted",
            Self::Managed => "managed",
            Self::AirGapped => "air_gapped",
            Self::MirrorOffline => "mirror_offline",
        }
    }
}

/// Mutation / recovery subsystem that consumes a component's projection.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5HistoryConsumerSurface {
    /// The editor-timeline UI.
    EditorTimelineUi,
    /// The checkpoint-inspector UI.
    CheckpointInspectorUi,
    /// The restore-review UI.
    RestoreReviewUi,
    /// The refactor-preview UI.
    RefactorPreviewUi,
    /// The AI-apply-review UI.
    AiApplyReviewUi,
    /// The recovery-center UI.
    RecoveryCenterUi,
    /// The support export.
    SupportExport,
    /// The CLI inspect / headless surface.
    CliInspect,
    /// The general product UI.
    ProductUi,
}

impl M5HistoryConsumerSurface {
    /// Every consumer surface, in declaration order.
    pub const ALL: [Self; 9] = [
        Self::EditorTimelineUi,
        Self::CheckpointInspectorUi,
        Self::RestoreReviewUi,
        Self::RefactorPreviewUi,
        Self::AiApplyReviewUi,
        Self::RecoveryCenterUi,
        Self::SupportExport,
        Self::CliInspect,
        Self::ProductUi,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::EditorTimelineUi => "editor_timeline_ui",
            Self::CheckpointInspectorUi => "checkpoint_inspector_ui",
            Self::RestoreReviewUi => "restore_review_ui",
            Self::RefactorPreviewUi => "refactor_preview_ui",
            Self::AiApplyReviewUi => "ai_apply_review_ui",
            Self::RecoveryCenterUi => "recovery_center_ui",
            Self::SupportExport => "support_export",
            Self::CliInspect => "cli_inspect",
            Self::ProductUi => "product_ui",
        }
    }
}

/// Non-visual / accessibility route every component must offer so no history truth is
/// hover-only, pointer-only, or visually encoded alone.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5HistoryAccessibilityRoute {
    /// Reachable and operable by keyboard focus.
    KeyboardFocusable,
    /// Announced to a screen reader.
    ScreenReaderAnnounced,
    /// Reachable without pointer hover.
    NonHoverReachable,
    /// Pointer interaction is optional, never required.
    PointerOptional,
    /// Legible in high-contrast / reduced-motion modes.
    HighContrastSafe,
    /// Present in the support / export packet.
    SupportExportable,
}

impl M5HistoryAccessibilityRoute {
    /// Every accessibility route, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::KeyboardFocusable,
        Self::ScreenReaderAnnounced,
        Self::NonHoverReachable,
        Self::PointerOptional,
        Self::HighContrastSafe,
        Self::SupportExportable,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::KeyboardFocusable => "keyboard_focusable",
            Self::ScreenReaderAnnounced => "screen_reader_announced",
            Self::NonHoverReachable => "non_hover_reachable",
            Self::PointerOptional => "pointer_optional",
            Self::HighContrastSafe => "high_contrast_safe",
            Self::SupportExportable => "support_exportable",
        }
    }
}

/// Mandatory label a claimed local-history / write-scope component must be able to
/// show. The first three are hard requirements on every component; the remaining
/// three close the acceptance-criteria ambiguity about timestamp/actor,
/// file/object identity, and restore/apply scope or export redaction.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5HistoryRequiredLabel {
    /// The component's stable identity / what history object it represents.
    Identity,
    /// The component's current typed state.
    State,
    /// The non-visual keyboard route to the component.
    KeyboardRoute,
    /// The snapshot timestamp and the actor behind the component.
    TimestampAndActor,
    /// The file or object identity the component acts on.
    FileOrObjectIdentity,
    /// The restore / apply scope or export-redaction posture behind the component.
    ScopeOrRedaction,
}

impl M5HistoryRequiredLabel {
    /// Every declared label, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::Identity,
        Self::State,
        Self::KeyboardRoute,
        Self::TimestampAndActor,
        Self::FileOrObjectIdentity,
        Self::ScopeOrRedaction,
    ];

    /// The three labels every claimed component must be able to show.
    pub const MANDATORY: [Self; 3] = [Self::Identity, Self::State, Self::KeyboardRoute];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Identity => "identity",
            Self::State => "state",
            Self::KeyboardRoute => "keyboard_route",
            Self::TimestampAndActor => "timestamp_and_actor",
            Self::FileOrObjectIdentity => "file_or_object_identity",
            Self::ScopeOrRedaction => "scope_or_redaction",
        }
    }
}

/// Qualification class for an M5 local-history / write-scope component row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5HistoryQualificationClass {
    /// Component qualifies for the Stable claim.
    Stable,
    /// Component is narrowed to Beta.
    Beta,
    /// Component is narrowed to Preview.
    Preview,
    /// Component is experimental and not claimed.
    Experimental,
    /// Component is unavailable on this build.
    Unavailable,
    /// Component is held pending upstream resolution.
    Held,
}

impl M5HistoryQualificationClass {
    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Stable => "stable",
            Self::Beta => "beta",
            Self::Preview => "preview",
            Self::Experimental => "experimental",
            Self::Unavailable => "unavailable",
            Self::Held => "held",
        }
    }

    /// Whether the component may carry a public Stable claim.
    pub const fn is_stable(self) -> bool {
        matches!(self, Self::Stable)
    }
}

/// Downgrade trigger that narrows a local-history / write-scope component below its
/// claim.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5HistoryDowngradeTrigger {
    /// A row left its snapshot timestamp or actor unstated.
    TimestampOrActorUnstated,
    /// A row masked its capture fidelity (e.g. metadata-only shown as full-body).
    CaptureFidelityMasked,
    /// A component left the file or object identity unstated.
    FileOrObjectIdentityUnstated,
    /// A component masked the branch or worktree context.
    BranchOrWorktreeContextMasked,
    /// A checkpoint-group card left its checkpoint lineage unstated.
    CheckpointLineageUnstated,
    /// A checkpoint-group card masked its mutation class.
    MutationClassMasked,
    /// A restore-preview card collapsed a partial restore into a whole-snapshot
    /// restore.
    RestoreGranularityCollapsed,
    /// A restore-preview card hid the restore drift state.
    RestoreDriftHidden,
    /// A component hid a generated-or-managed-file caveat.
    GeneratedOrManagedCaveatHidden,
    /// A retention/export card left retention or redaction undisclosed.
    RetentionOrRedactionUndisclosed,
    /// A write-scope preview tree understated the write scope.
    WriteScopeUnderstated,
    /// The proof packet has gone stale.
    ProofStale,
}

impl M5HistoryDowngradeTrigger {
    /// Every trigger, in declaration order.
    pub const ALL: [Self; 12] = [
        Self::TimestampOrActorUnstated,
        Self::CaptureFidelityMasked,
        Self::FileOrObjectIdentityUnstated,
        Self::BranchOrWorktreeContextMasked,
        Self::CheckpointLineageUnstated,
        Self::MutationClassMasked,
        Self::RestoreGranularityCollapsed,
        Self::RestoreDriftHidden,
        Self::GeneratedOrManagedCaveatHidden,
        Self::RetentionOrRedactionUndisclosed,
        Self::WriteScopeUnderstated,
        Self::ProofStale,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::TimestampOrActorUnstated => "timestamp_or_actor_unstated",
            Self::CaptureFidelityMasked => "capture_fidelity_masked",
            Self::FileOrObjectIdentityUnstated => "file_or_object_identity_unstated",
            Self::BranchOrWorktreeContextMasked => "branch_or_worktree_context_masked",
            Self::CheckpointLineageUnstated => "checkpoint_lineage_unstated",
            Self::MutationClassMasked => "mutation_class_masked",
            Self::RestoreGranularityCollapsed => "restore_granularity_collapsed",
            Self::RestoreDriftHidden => "restore_drift_hidden",
            Self::GeneratedOrManagedCaveatHidden => "generated_or_managed_caveat_hidden",
            Self::RetentionOrRedactionUndisclosed => "retention_or_redaction_undisclosed",
            Self::WriteScopeUnderstated => "write_scope_understated",
            Self::ProofStale => "proof_stale",
        }
    }
}

/// One row in the matrix: one governed local-history / write-scope component family
/// bound to the surface-specific truth it must project.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5LocalHistoryWriteScopeComponentRow {
    /// Governed component family.
    pub component_family: M5LocalHistoryWriteScopeComponentFamily,
    /// Qualification class earned by this component.
    pub qualification: M5HistoryQualificationClass,
    /// Owner role accountable for keeping this component governed.
    pub owner_role: String,
    /// Human-readable scope summary.
    pub scope_summary: String,
    /// Claimed M5 mutation / recovery surface families that render / consume this
    /// component.
    pub surface_families: Vec<M5HistorySurfaceFamily>,
    /// Deployment lines this component keeps the same truth across.
    pub deployment_lines: Vec<M5HistoryDeploymentLine>,
    /// Mandatory labels this component must be able to show (must include the three
    /// [`M5HistoryRequiredLabel::MANDATORY`] labels).
    pub required_labels: Vec<M5HistoryRequiredLabel>,
    /// Snapshot origins this component names (local-history-row only).
    pub snapshot_origins: Vec<M5SnapshotOrigin>,
    /// Actor classes this component distinguishes (local-history-row only).
    pub actor_classes: Vec<M5HistoryActorClass>,
    /// Capture fidelities this component distinguishes (local-history-row only).
    pub capture_fidelities: Vec<M5CaptureFidelity>,
    /// Checkpoint lineage classes this component names (checkpoint-group-card only).
    pub checkpoint_lineage_classes: Vec<M5CheckpointLineageClass>,
    /// Mutation classes this component names (checkpoint-group-card only).
    pub mutation_classes: Vec<M5MutationClass>,
    /// Restore granularities this component distinguishes (restore-preview-card only).
    pub restore_granularities: Vec<M5RestoreGranularity>,
    /// Restore drift states this component discloses (restore-preview-card only).
    pub restore_drift_states: Vec<M5RestoreDriftState>,
    /// Retention postures this component distinguishes (retention-export-card only).
    pub retention_postures: Vec<M5RetentionPosture>,
    /// Export-redaction postures this component distinguishes (retention-export-card
    /// and history-export-manifest).
    pub export_redaction_postures: Vec<M5ExportRedactionPosture>,
    /// Write-scope classes this component distinguishes (write-scope-preview-tree
    /// only).
    pub write_scope_classes: Vec<M5WriteScopeClass>,
    /// Managed-file caveats this component discloses (write-scope-preview-tree only).
    pub managed_file_caveats: Vec<M5ManagedFileCaveat>,
    /// Restore-selection modes this component offers (restore-granularity-selector
    /// only).
    pub restore_selection_modes: Vec<M5RestoreSelectionMode>,
    /// Export-manifest classes this component distinguishes (history-export-manifest
    /// only).
    pub export_manifest_classes: Vec<M5ExportManifestClass>,
    /// Non-visual accessibility routes this component offers.
    pub accessibility_routes: Vec<M5HistoryAccessibilityRoute>,
    /// Mutation / recovery subsystems that consume this component's projection.
    pub consumer_surfaces: Vec<M5HistoryConsumerSurface>,
    /// Downgrade triggers that apply to this component.
    pub downgrade_triggers: Vec<M5HistoryDowngradeTrigger>,
    /// Proof packet refs that keep this component current.
    pub required_proof_packet_refs: Vec<String>,
    /// Source contract refs consumed by this component.
    pub source_contract_refs: Vec<String>,
    /// Hard invariant: this component never masks its snapshot actor or timestamp.
    /// MUST be `false`.
    pub masks_actor_or_timestamp: bool,
    /// Hard invariant: this component never hides a generated-or-managed-file caveat.
    /// MUST be `false`.
    pub hides_generated_or_managed_caveat: bool,
    /// Hard invariant: this component never invents a private history grammar. MUST
    /// be `false`.
    pub invents_private_history_grammar: bool,
    /// Hard invariant: this component never bypasses the restore-scope review. MUST
    /// be `false`.
    pub bypasses_restore_scope_review: bool,
}

impl M5LocalHistoryWriteScopeComponentRow {
    /// `true` when the row declares all mandatory labels.
    fn declares_mandatory_labels(&self) -> bool {
        let present: BTreeSet<M5HistoryRequiredLabel> =
            self.required_labels.iter().copied().collect();
        M5HistoryRequiredLabel::MANDATORY
            .iter()
            .all(|label| present.contains(label))
    }

    /// `true` when the row's hard invariants hold.
    fn honours_invariants(&self) -> bool {
        !self.masks_actor_or_timestamp
            && !self.hides_generated_or_managed_caveat
            && !self.invents_private_history_grammar
            && !self.bypasses_restore_scope_review
    }
}

/// Self-describing controlled-vocabulary set frozen by the matrix.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5LocalHistoryWriteScopeComponentVocabularySet {
    /// Component-family tokens.
    pub component_families: Vec<String>,
    /// Snapshot-origin tokens.
    pub snapshot_origins: Vec<String>,
    /// Actor-class tokens.
    pub actor_classes: Vec<String>,
    /// Capture-fidelity tokens.
    pub capture_fidelities: Vec<String>,
    /// Checkpoint-lineage-class tokens.
    pub checkpoint_lineage_classes: Vec<String>,
    /// Mutation-class tokens.
    pub mutation_classes: Vec<String>,
    /// Restore-granularity tokens.
    pub restore_granularities: Vec<String>,
    /// Restore-drift-state tokens.
    pub restore_drift_states: Vec<String>,
    /// Retention-posture tokens.
    pub retention_postures: Vec<String>,
    /// Export-redaction-posture tokens.
    pub export_redaction_postures: Vec<String>,
    /// Write-scope-class tokens.
    pub write_scope_classes: Vec<String>,
    /// Managed-file-caveat tokens.
    pub managed_file_caveats: Vec<String>,
    /// Restore-selection-mode tokens.
    pub restore_selection_modes: Vec<String>,
    /// Export-manifest-class tokens.
    pub export_manifest_classes: Vec<String>,
    /// Surface-family tokens.
    pub surface_families: Vec<String>,
    /// Deployment-line tokens.
    pub deployment_lines: Vec<String>,
    /// Consumer-surface tokens.
    pub consumer_surfaces: Vec<String>,
    /// Accessibility-route tokens.
    pub accessibility_routes: Vec<String>,
    /// Required-label tokens.
    pub required_labels: Vec<String>,
}

impl M5LocalHistoryWriteScopeComponentVocabularySet {
    /// Builds the canonical vocabulary set from the typed `ALL` arrays.
    pub fn canonical() -> Self {
        Self {
            component_families: tokens(&M5LocalHistoryWriteScopeComponentFamily::ALL, |v| {
                v.as_str()
            }),
            snapshot_origins: tokens(&M5SnapshotOrigin::ALL, |v| v.as_str()),
            actor_classes: tokens(&M5HistoryActorClass::ALL, |v| v.as_str()),
            capture_fidelities: tokens(&M5CaptureFidelity::ALL, |v| v.as_str()),
            checkpoint_lineage_classes: tokens(&M5CheckpointLineageClass::ALL, |v| v.as_str()),
            mutation_classes: tokens(&M5MutationClass::ALL, |v| v.as_str()),
            restore_granularities: tokens(&M5RestoreGranularity::ALL, |v| v.as_str()),
            restore_drift_states: tokens(&M5RestoreDriftState::ALL, |v| v.as_str()),
            retention_postures: tokens(&M5RetentionPosture::ALL, |v| v.as_str()),
            export_redaction_postures: tokens(&M5ExportRedactionPosture::ALL, |v| v.as_str()),
            write_scope_classes: tokens(&M5WriteScopeClass::ALL, |v| v.as_str()),
            managed_file_caveats: tokens(&M5ManagedFileCaveat::ALL, |v| v.as_str()),
            restore_selection_modes: tokens(&M5RestoreSelectionMode::ALL, |v| v.as_str()),
            export_manifest_classes: tokens(&M5ExportManifestClass::ALL, |v| v.as_str()),
            surface_families: tokens(&M5HistorySurfaceFamily::ALL, |v| v.as_str()),
            deployment_lines: tokens(&M5HistoryDeploymentLine::ALL, |v| v.as_str()),
            consumer_surfaces: tokens(&M5HistoryConsumerSurface::ALL, |v| v.as_str()),
            accessibility_routes: tokens(&M5HistoryAccessibilityRoute::ALL, |v| v.as_str()),
            required_labels: tokens(&M5HistoryRequiredLabel::ALL, |v| v.as_str()),
        }
    }

    /// Returns true when this set matches the canonical token lists exactly.
    pub fn matches_canonical(&self) -> bool {
        *self == Self::canonical()
    }
}

fn tokens<T: Copy>(items: &[T], to_token: impl Fn(T) -> &'static str) -> Vec<String> {
    items.iter().map(|v| to_token(*v).to_owned()).collect()
}

/// Governance-review block; every flag is a hard invariant.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5LocalHistoryWriteScopeComponentGovernanceReview {
    /// The local-history row shows its snapshot timestamp and actor.
    pub local_history_row_shows_timestamp_and_actor: bool,
    /// The checkpoint-group card shows its lineage and mutation class.
    pub checkpoint_group_card_shows_lineage_and_mutation_class: bool,
    /// The restore-preview card shows its granularity and drift.
    pub restore_preview_card_shows_granularity_and_drift: bool,
    /// The retention/export card shows its retention and redaction posture.
    pub retention_export_card_shows_retention_and_redaction: bool,
    /// The write-scope preview tree shows its scope and managed-file caveat.
    pub write_scope_preview_tree_shows_scope_and_managed_caveat: bool,
    /// The restore-granularity selector shows its selectable apply scope.
    pub restore_granularity_selector_shows_selection_modes: bool,
    /// The history-export manifest shows its class and redaction.
    pub history_export_manifest_shows_class_and_redaction: bool,
    /// Generated or managed files are never silently restored or applied over.
    pub generated_or_managed_files_never_silently_restored: bool,
    /// A partial or manual restore is never shown as a whole-snapshot restore.
    pub partial_restore_never_shown_as_whole_snapshot: bool,
    /// The branch or worktree context is always explicit.
    pub branch_or_worktree_context_always_explicit: bool,
    /// The export-redaction posture is always explicit.
    pub export_redaction_posture_always_explicit: bool,
    /// No component invents a second history grammar.
    pub no_component_invents_second_history_grammar: bool,
    /// Every component keeps the same truth across every deployment line.
    pub every_component_declares_deployment_lines: bool,
    /// Every component declares a non-visual accessibility route.
    pub every_component_declares_accessibility_route: bool,
    /// Later M5 rows cannot invent parallel history vocabulary.
    pub later_rows_cannot_invent_parallel_vocabulary: bool,
}

/// Consumer projection block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5LocalHistoryWriteScopeComponentConsumerProjection {
    /// Editor and recovery surfaces consume the shared history vocabulary.
    pub editor_and_recovery_surfaces_consume_history_vocabulary: bool,
    /// Restore surfaces consume the granularity vocabulary.
    pub restore_surfaces_consume_granularity_vocabulary: bool,
    /// Write-scope surfaces consume the managed-file-caveat vocabulary.
    pub write_scope_surfaces_consume_managed_caveat_vocabulary: bool,
    /// Export surfaces consume the redaction vocabulary.
    pub export_surfaces_consume_redaction_vocabulary: bool,
    /// Support / export reads a single canonical history source.
    pub support_export_reads_single_source: bool,
    /// Refactor and AI-apply surfaces read a single canonical history source.
    pub refactor_and_ai_surfaces_read_single_source: bool,
}

/// Proof freshness block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5LocalHistoryWriteScopeComponentProofFreshness {
    /// Proof-freshness SLO in hours.
    pub proof_freshness_slo_hours: u32,
    /// RFC 3339 timestamp of the last proof refresh.
    pub last_proof_refresh: String,
    /// True when stale proof automatically narrows the component.
    pub auto_narrow_on_stale: bool,
}

/// Release and support parity posture for the local-history / write-scope component
/// lane.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5LocalHistoryWriteScopeComponentReleasePosture {
    /// Ref of the supporting proof packet for the lane.
    pub proof_packet_ref: String,
    /// Ref of the supporting recovery audit for the lane.
    pub recovery_audit_ref: String,
    /// True when support/export parity is required for every component.
    pub support_export_parity_required: bool,
    /// True when accessibility parity is required for every component.
    pub accessibility_parity_required: bool,
}

/// Constructor input for [`M5LocalHistoryWriteScopeComponentMatrixPacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M5LocalHistoryWriteScopeComponentMatrixPacketInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable matrix label.
    pub matrix_label: String,
    /// Component rows.
    pub component_rows: Vec<M5LocalHistoryWriteScopeComponentRow>,
    /// Frozen controlled-vocabulary set.
    pub vocabulary_set: M5LocalHistoryWriteScopeComponentVocabularySet,
    /// Governance-review block.
    pub governance_review: M5LocalHistoryWriteScopeComponentGovernanceReview,
    /// Consumer projection block.
    pub consumer_projection: M5LocalHistoryWriteScopeComponentConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: M5LocalHistoryWriteScopeComponentProofFreshness,
    /// Release and support parity posture.
    pub release_posture: M5LocalHistoryWriteScopeComponentReleasePosture,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

/// Export-safe frozen M5 local-history / write-scope component matrix packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5LocalHistoryWriteScopeComponentMatrixPacket {
    /// Record kind; must equal
    /// [`M5_LOCAL_HISTORY_WRITE_SCOPE_COMPONENT_MATRIX_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal
    /// [`M5_LOCAL_HISTORY_WRITE_SCOPE_COMPONENT_MATRIX_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable matrix label.
    pub matrix_label: String,
    /// Component rows.
    pub component_rows: Vec<M5LocalHistoryWriteScopeComponentRow>,
    /// Frozen controlled-vocabulary set.
    pub vocabulary_set: M5LocalHistoryWriteScopeComponentVocabularySet,
    /// Governance-review block.
    pub governance_review: M5LocalHistoryWriteScopeComponentGovernanceReview,
    /// Consumer projection block.
    pub consumer_projection: M5LocalHistoryWriteScopeComponentConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: M5LocalHistoryWriteScopeComponentProofFreshness,
    /// Release and support parity posture.
    pub release_posture: M5LocalHistoryWriteScopeComponentReleasePosture,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

impl M5LocalHistoryWriteScopeComponentMatrixPacket {
    /// Builds an M5 local-history / write-scope component matrix packet from
    /// stable-lane input.
    pub fn new(input: M5LocalHistoryWriteScopeComponentMatrixPacketInput) -> Self {
        Self {
            record_kind: M5_LOCAL_HISTORY_WRITE_SCOPE_COMPONENT_MATRIX_RECORD_KIND.to_owned(),
            schema_version: M5_LOCAL_HISTORY_WRITE_SCOPE_COMPONENT_MATRIX_SCHEMA_VERSION,
            packet_id: input.packet_id,
            matrix_label: input.matrix_label,
            component_rows: input.component_rows,
            vocabulary_set: input.vocabulary_set,
            governance_review: input.governance_review,
            consumer_projection: input.consumer_projection,
            proof_freshness: input.proof_freshness,
            release_posture: input.release_posture,
            source_contract_refs: input.source_contract_refs,
            redaction_class_token: input.redaction_class_token,
            minted_at: input.minted_at,
        }
    }

    /// Validates the M5 local-history / write-scope component matrix invariants.
    pub fn validate(&self) -> Vec<M5LocalHistoryWriteScopeComponentMatrixViolation> {
        let mut violations = Vec::new();

        if self.record_kind != M5_LOCAL_HISTORY_WRITE_SCOPE_COMPONENT_MATRIX_RECORD_KIND {
            violations.push(M5LocalHistoryWriteScopeComponentMatrixViolation::WrongRecordKind);
        }
        if self.schema_version != M5_LOCAL_HISTORY_WRITE_SCOPE_COMPONENT_MATRIX_SCHEMA_VERSION {
            violations.push(M5LocalHistoryWriteScopeComponentMatrixViolation::WrongSchemaVersion);
        }
        if self.packet_id.trim().is_empty()
            || self.matrix_label.trim().is_empty()
            || self.redaction_class_token.trim().is_empty()
            || self.minted_at.trim().is_empty()
        {
            violations.push(M5LocalHistoryWriteScopeComponentMatrixViolation::MissingIdentity);
        }

        validate_source_contracts(self, &mut violations);
        validate_vocabulary_set(self, &mut violations);
        validate_component_rows(self, &mut violations);
        validate_governance_review(self, &mut violations);
        validate_consumer_projection(self, &mut violations);
        validate_proof_freshness(self, &mut violations);
        validate_release_posture(self, &mut violations);

        if json_contains_forbidden_material(
            &serde_json::to_value(self)
                .expect("m5 local-history write-scope component matrix packet serializes"),
        ) {
            violations.push(M5LocalHistoryWriteScopeComponentMatrixViolation::RawMaterialInExport);
        }

        violations
    }

    /// Deterministic export-safe JSON.
    ///
    /// # Panics
    ///
    /// Panics only if serializing this metadata-only packet fails.
    pub fn export_safe_json(&self) -> String {
        serde_json::to_string_pretty(self)
            .expect("m5 local-history write-scope component matrix packet serializes")
    }

    /// Deterministic, machine-readable matrix CSV: one row per governed component.
    pub fn render_matrix_csv(&self) -> String {
        let mut out = String::new();
        out.push_str(
            "component_family,qualification,owner,surface_families,deployment_lines,required_labels,consumer_surfaces,downgrade_triggers\n",
        );
        for row in &self.component_rows {
            out.push_str(&format!(
                "{},{},{},{},{},{},{},{}\n",
                row.component_family.as_str(),
                row.qualification.as_str(),
                csv_field(&row.owner_role),
                join_tokens(&row.surface_families, |v| v.as_str()),
                join_tokens(&row.deployment_lines, |v| v.as_str()),
                join_tokens(&row.required_labels, |v| v.as_str()),
                join_tokens(&row.consumer_surfaces, |v| v.as_str()),
                join_tokens(&row.downgrade_triggers, |v| v.as_str()),
            ));
        }
        out
    }

    /// Deterministic Markdown report for support, docs, or review handoff.
    pub fn render_markdown_summary(&self) -> String {
        let stable_components = self
            .component_rows
            .iter()
            .filter(|row| row.qualification.is_stable())
            .count();
        let mut out = String::new();
        out.push_str(
            "# M5 Local-History-Row, Checkpoint-Group-Card, Restore-Preview-Card, Retention/Export-Card, Write-Scope-Preview-Tree, Restore-Granularity-Selector, and History-Export-Manifest Component Matrix\n\n",
        );
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- Label: `{}`\n", self.matrix_label));
        out.push_str(&format!(
            "- Component families: {} ({} stable)\n",
            self.component_rows.len(),
            stable_components
        ));
        out.push_str(&format!(
            "- Snapshot origins: {}\n",
            self.vocabulary_set.snapshot_origins.join(", ")
        ));
        out.push_str(&format!(
            "- Restore granularities: {}\n",
            self.vocabulary_set.restore_granularities.join(", ")
        ));
        out.push_str(&format!(
            "- Proof freshness SLO: {} hours (last refresh: {})\n",
            self.proof_freshness.proof_freshness_slo_hours, self.proof_freshness.last_proof_refresh
        ));
        out.push_str("\n## Component families\n\n");
        for row in &self.component_rows {
            out.push_str(&format!(
                "- **{}**: `{}`\n",
                row.component_family.as_str(),
                row.qualification.as_str()
            ));
            out.push_str(&format!("  - Owner: {}\n", row.owner_role));
            out.push_str(&format!("  - Scope: {}\n", row.scope_summary));
            out.push_str(&format!(
                "  - Required labels: {}\n",
                row.required_labels
                    .iter()
                    .map(|v| v.as_str())
                    .collect::<Vec<_>>()
                    .join(", ")
            ));
            out.push_str(&format!(
                "  - Accessibility routes: {}\n",
                row.accessibility_routes
                    .iter()
                    .map(|v| v.as_str())
                    .collect::<Vec<_>>()
                    .join(", ")
            ));
        }
        out
    }
}

/// Errors emitted when reading the checked-in M5 local-history matrix export.
#[derive(Debug)]
pub enum M5LocalHistoryWriteScopeComponentMatrixArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<M5LocalHistoryWriteScopeComponentMatrixViolation>),
}

impl fmt::Display for M5LocalHistoryWriteScopeComponentMatrixArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(
                    formatter,
                    "m5 local-history write-scope component matrix export parse failed: {error}"
                )
            }
            Self::Validation(violations) => {
                let tokens = violations
                    .iter()
                    .map(|violation| violation.as_str())
                    .collect::<Vec<_>>()
                    .join(",");
                write!(
                    formatter,
                    "m5 local-history write-scope component matrix export failed validation: {tokens}"
                )
            }
        }
    }
}

impl Error for M5LocalHistoryWriteScopeComponentMatrixArtifactError {}

/// Validation failures emitted by
/// [`M5LocalHistoryWriteScopeComponentMatrixPacket::validate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum M5LocalHistoryWriteScopeComponentMatrixViolation {
    /// Packet record kind is wrong.
    WrongRecordKind,
    /// Packet schema version is wrong.
    WrongSchemaVersion,
    /// Required identity field is missing.
    MissingIdentity,
    /// Source contract refs are incomplete.
    MissingSourceContracts,
    /// The frozen vocabulary set drifted from the canonical token lists.
    VocabularySetDrift,
    /// A required governed component family is missing from the matrix.
    RequiredComponentMissing,
    /// A component row is incomplete.
    ComponentRowIncomplete,
    /// A component row omits one of the mandatory labels.
    MandatoryLabelMissing,
    /// A local-history-row component declares no snapshot origins.
    SnapshotOriginMissing,
    /// A local-history-row component declares no actor classes.
    ActorClassMissing,
    /// A local-history-row component declares no capture fidelities.
    CaptureFidelityMissing,
    /// A checkpoint-group-card component declares no checkpoint lineage classes.
    CheckpointLineageMissing,
    /// A checkpoint-group-card component declares no mutation classes.
    MutationClassMissing,
    /// A restore-preview-card component declares no restore granularities.
    RestoreGranularityMissing,
    /// A restore-preview-card component declares no restore drift states.
    RestoreDriftStateMissing,
    /// A retention-export-card component declares no retention postures.
    RetentionPostureMissing,
    /// A retention-export-card component declares no export-redaction postures.
    ExportRedactionPostureMissing,
    /// A write-scope-preview-tree component declares no write-scope classes.
    WriteScopeClassMissing,
    /// A write-scope-preview-tree component declares no managed-file caveats.
    ManagedFileCaveatMissing,
    /// A restore-granularity-selector component declares no restore-selection modes.
    RestoreSelectionModeMissing,
    /// A history-export-manifest component declares no export-manifest classes.
    ExportManifestClassMissing,
    /// A component declares no surface families.
    SurfaceFamilyMissing,
    /// A component declares no deployment lines.
    DeploymentLineMissing,
    /// A component declares no accessibility routes.
    AccessibilityRouteMissing,
    /// A component declares no consumer surfaces.
    ConsumerSurfacesMissing,
    /// A component declares no downgrade triggers.
    DowngradeTriggersMissing,
    /// A component claiming Stable is missing required proof packet refs.
    StableComponentMissingProof,
    /// A component violates a hard invariant (masked actor/timestamp, hidden
    /// generated/managed caveat, private history grammar, or bypassed restore-scope
    /// review).
    ComponentInvariantViolated,
    /// Governance review does not satisfy required invariants.
    GovernanceReviewIncomplete,
    /// Consumer projection does not satisfy required invariants.
    ConsumerProjectionIncomplete,
    /// Proof freshness block is incomplete.
    ProofFreshnessIncomplete,
    /// Release/support parity posture is incomplete.
    ReleasePostureIncomplete,
    /// Export contains raw sensitive material.
    RawMaterialInExport,
}

impl M5LocalHistoryWriteScopeComponentMatrixViolation {
    /// Stable token used in tests and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WrongRecordKind => "wrong_record_kind",
            Self::WrongSchemaVersion => "wrong_schema_version",
            Self::MissingIdentity => "missing_identity",
            Self::MissingSourceContracts => "missing_source_contracts",
            Self::VocabularySetDrift => "vocabulary_set_drift",
            Self::RequiredComponentMissing => "required_component_missing",
            Self::ComponentRowIncomplete => "component_row_incomplete",
            Self::MandatoryLabelMissing => "mandatory_label_missing",
            Self::SnapshotOriginMissing => "snapshot_origin_missing",
            Self::ActorClassMissing => "actor_class_missing",
            Self::CaptureFidelityMissing => "capture_fidelity_missing",
            Self::CheckpointLineageMissing => "checkpoint_lineage_missing",
            Self::MutationClassMissing => "mutation_class_missing",
            Self::RestoreGranularityMissing => "restore_granularity_missing",
            Self::RestoreDriftStateMissing => "restore_drift_state_missing",
            Self::RetentionPostureMissing => "retention_posture_missing",
            Self::ExportRedactionPostureMissing => "export_redaction_posture_missing",
            Self::WriteScopeClassMissing => "write_scope_class_missing",
            Self::ManagedFileCaveatMissing => "managed_file_caveat_missing",
            Self::RestoreSelectionModeMissing => "restore_selection_mode_missing",
            Self::ExportManifestClassMissing => "export_manifest_class_missing",
            Self::SurfaceFamilyMissing => "surface_family_missing",
            Self::DeploymentLineMissing => "deployment_line_missing",
            Self::AccessibilityRouteMissing => "accessibility_route_missing",
            Self::ConsumerSurfacesMissing => "consumer_surfaces_missing",
            Self::DowngradeTriggersMissing => "downgrade_triggers_missing",
            Self::StableComponentMissingProof => "stable_component_missing_proof",
            Self::ComponentInvariantViolated => "component_invariant_violated",
            Self::GovernanceReviewIncomplete => "governance_review_incomplete",
            Self::ConsumerProjectionIncomplete => "consumer_projection_incomplete",
            Self::ProofFreshnessIncomplete => "proof_freshness_incomplete",
            Self::ReleasePostureIncomplete => "release_posture_incomplete",
            Self::RawMaterialInExport => "raw_material_in_export",
        }
    }
}

/// Reads and validates the checked-in stable M5 local-history matrix export.
pub fn current_stable_m5_local_history_write_scope_component_matrix_export() -> Result<
    M5LocalHistoryWriteScopeComponentMatrixPacket,
    M5LocalHistoryWriteScopeComponentMatrixArtifactError,
> {
    let packet: M5LocalHistoryWriteScopeComponentMatrixPacket =
        serde_json::from_str(include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../artifacts/release/m5-local-history-write-scope-component-proof/support_export.json"
        )))
        .map_err(M5LocalHistoryWriteScopeComponentMatrixArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(M5LocalHistoryWriteScopeComponentMatrixArtifactError::Validation(violations))
    }
}

fn validate_source_contracts(
    packet: &M5LocalHistoryWriteScopeComponentMatrixPacket,
    violations: &mut Vec<M5LocalHistoryWriteScopeComponentMatrixViolation>,
) {
    let refs: BTreeSet<&str> = packet
        .source_contract_refs
        .iter()
        .map(String::as_str)
        .collect();
    for required in [
        M5_LOCAL_HISTORY_WRITE_SCOPE_COMPONENT_SCHEMA_REF,
        M5_LOCAL_HISTORY_WRITE_SCOPE_COMPONENT_DOC_REF,
        M5_LOCAL_HISTORY_WRITE_SCOPE_COMPONENT_HISTORY_ENTRY_REF,
        M5_LOCAL_HISTORY_WRITE_SCOPE_COMPONENT_CHECKPOINT_REF,
        M5_LOCAL_HISTORY_WRITE_SCOPE_COMPONENT_RESTORE_PREVIEW_REF,
        M5_LOCAL_HISTORY_WRITE_SCOPE_COMPONENT_RETENTION_REF,
        M5_LOCAL_HISTORY_WRITE_SCOPE_COMPONENT_WRITE_BOUNDARY_REF,
    ] {
        if !refs.contains(required) {
            violations
                .push(M5LocalHistoryWriteScopeComponentMatrixViolation::MissingSourceContracts);
            return;
        }
    }
}

fn validate_vocabulary_set(
    packet: &M5LocalHistoryWriteScopeComponentMatrixPacket,
    violations: &mut Vec<M5LocalHistoryWriteScopeComponentMatrixViolation>,
) {
    if !packet.vocabulary_set.matches_canonical() {
        violations.push(M5LocalHistoryWriteScopeComponentMatrixViolation::VocabularySetDrift);
    }
}

fn validate_component_rows(
    packet: &M5LocalHistoryWriteScopeComponentMatrixPacket,
    violations: &mut Vec<M5LocalHistoryWriteScopeComponentMatrixViolation>,
) {
    let present: BTreeSet<M5LocalHistoryWriteScopeComponentFamily> = packet
        .component_rows
        .iter()
        .map(|row| row.component_family)
        .collect();
    for required in M5LocalHistoryWriteScopeComponentFamily::ALL {
        if !present.contains(&required) {
            violations
                .push(M5LocalHistoryWriteScopeComponentMatrixViolation::RequiredComponentMissing);
            return;
        }
    }

    for row in &packet.component_rows {
        let family = row.component_family;
        if row.owner_role.trim().is_empty()
            || row.scope_summary.trim().is_empty()
            || row.source_contract_refs.is_empty()
            || row.required_labels.is_empty()
        {
            violations
                .push(M5LocalHistoryWriteScopeComponentMatrixViolation::ComponentRowIncomplete);
        }
        if !row.declares_mandatory_labels() {
            violations
                .push(M5LocalHistoryWriteScopeComponentMatrixViolation::MandatoryLabelMissing);
        }
        if family.is_local_history_row() && row.snapshot_origins.is_empty() {
            violations
                .push(M5LocalHistoryWriteScopeComponentMatrixViolation::SnapshotOriginMissing);
        }
        if family.is_local_history_row() && row.actor_classes.is_empty() {
            violations.push(M5LocalHistoryWriteScopeComponentMatrixViolation::ActorClassMissing);
        }
        if family.is_local_history_row() && row.capture_fidelities.is_empty() {
            violations
                .push(M5LocalHistoryWriteScopeComponentMatrixViolation::CaptureFidelityMissing);
        }
        if family.is_checkpoint_group_card() && row.checkpoint_lineage_classes.is_empty() {
            violations
                .push(M5LocalHistoryWriteScopeComponentMatrixViolation::CheckpointLineageMissing);
        }
        if family.is_checkpoint_group_card() && row.mutation_classes.is_empty() {
            violations.push(M5LocalHistoryWriteScopeComponentMatrixViolation::MutationClassMissing);
        }
        if family.is_restore_preview_card() && row.restore_granularities.is_empty() {
            violations
                .push(M5LocalHistoryWriteScopeComponentMatrixViolation::RestoreGranularityMissing);
        }
        if family.is_restore_preview_card() && row.restore_drift_states.is_empty() {
            violations
                .push(M5LocalHistoryWriteScopeComponentMatrixViolation::RestoreDriftStateMissing);
        }
        if family.is_retention_export_card() && row.retention_postures.is_empty() {
            violations
                .push(M5LocalHistoryWriteScopeComponentMatrixViolation::RetentionPostureMissing);
        }
        if family.is_retention_export_card() && row.export_redaction_postures.is_empty() {
            violations.push(
                M5LocalHistoryWriteScopeComponentMatrixViolation::ExportRedactionPostureMissing,
            );
        }
        if family.is_write_scope_preview_tree() && row.write_scope_classes.is_empty() {
            violations
                .push(M5LocalHistoryWriteScopeComponentMatrixViolation::WriteScopeClassMissing);
        }
        if family.is_write_scope_preview_tree() && row.managed_file_caveats.is_empty() {
            violations
                .push(M5LocalHistoryWriteScopeComponentMatrixViolation::ManagedFileCaveatMissing);
        }
        if family.is_restore_granularity_selector() && row.restore_selection_modes.is_empty() {
            violations.push(
                M5LocalHistoryWriteScopeComponentMatrixViolation::RestoreSelectionModeMissing,
            );
        }
        if family.is_history_export_manifest() && row.export_manifest_classes.is_empty() {
            violations
                .push(M5LocalHistoryWriteScopeComponentMatrixViolation::ExportManifestClassMissing);
        }
        if row.surface_families.is_empty() {
            violations.push(M5LocalHistoryWriteScopeComponentMatrixViolation::SurfaceFamilyMissing);
        }
        if row.deployment_lines.is_empty() {
            violations
                .push(M5LocalHistoryWriteScopeComponentMatrixViolation::DeploymentLineMissing);
        }
        if row.accessibility_routes.is_empty() {
            violations
                .push(M5LocalHistoryWriteScopeComponentMatrixViolation::AccessibilityRouteMissing);
        }
        if row.consumer_surfaces.is_empty() {
            violations
                .push(M5LocalHistoryWriteScopeComponentMatrixViolation::ConsumerSurfacesMissing);
        }
        if row.downgrade_triggers.is_empty() {
            violations
                .push(M5LocalHistoryWriteScopeComponentMatrixViolation::DowngradeTriggersMissing);
        }
        if row.qualification.is_stable() && row.required_proof_packet_refs.is_empty() {
            violations.push(
                M5LocalHistoryWriteScopeComponentMatrixViolation::StableComponentMissingProof,
            );
        }
        if !row.honours_invariants() {
            violations
                .push(M5LocalHistoryWriteScopeComponentMatrixViolation::ComponentInvariantViolated);
        }
    }
}

fn validate_governance_review(
    packet: &M5LocalHistoryWriteScopeComponentMatrixPacket,
    violations: &mut Vec<M5LocalHistoryWriteScopeComponentMatrixViolation>,
) {
    let review = &packet.governance_review;
    for ok in [
        review.local_history_row_shows_timestamp_and_actor,
        review.checkpoint_group_card_shows_lineage_and_mutation_class,
        review.restore_preview_card_shows_granularity_and_drift,
        review.retention_export_card_shows_retention_and_redaction,
        review.write_scope_preview_tree_shows_scope_and_managed_caveat,
        review.restore_granularity_selector_shows_selection_modes,
        review.history_export_manifest_shows_class_and_redaction,
        review.generated_or_managed_files_never_silently_restored,
        review.partial_restore_never_shown_as_whole_snapshot,
        review.branch_or_worktree_context_always_explicit,
        review.export_redaction_posture_always_explicit,
        review.no_component_invents_second_history_grammar,
        review.every_component_declares_deployment_lines,
        review.every_component_declares_accessibility_route,
        review.later_rows_cannot_invent_parallel_vocabulary,
    ] {
        if !ok {
            violations
                .push(M5LocalHistoryWriteScopeComponentMatrixViolation::GovernanceReviewIncomplete);
            return;
        }
    }
}

fn validate_consumer_projection(
    packet: &M5LocalHistoryWriteScopeComponentMatrixPacket,
    violations: &mut Vec<M5LocalHistoryWriteScopeComponentMatrixViolation>,
) {
    let projection = &packet.consumer_projection;
    for ok in [
        projection.editor_and_recovery_surfaces_consume_history_vocabulary,
        projection.restore_surfaces_consume_granularity_vocabulary,
        projection.write_scope_surfaces_consume_managed_caveat_vocabulary,
        projection.export_surfaces_consume_redaction_vocabulary,
        projection.support_export_reads_single_source,
        projection.refactor_and_ai_surfaces_read_single_source,
    ] {
        if !ok {
            violations.push(
                M5LocalHistoryWriteScopeComponentMatrixViolation::ConsumerProjectionIncomplete,
            );
            return;
        }
    }
}

fn validate_proof_freshness(
    packet: &M5LocalHistoryWriteScopeComponentMatrixPacket,
    violations: &mut Vec<M5LocalHistoryWriteScopeComponentMatrixViolation>,
) {
    if packet.proof_freshness.proof_freshness_slo_hours == 0
        || packet.proof_freshness.last_proof_refresh.trim().is_empty()
    {
        violations.push(M5LocalHistoryWriteScopeComponentMatrixViolation::ProofFreshnessIncomplete);
    }
}

fn validate_release_posture(
    packet: &M5LocalHistoryWriteScopeComponentMatrixPacket,
    violations: &mut Vec<M5LocalHistoryWriteScopeComponentMatrixViolation>,
) {
    let posture = &packet.release_posture;
    if posture.proof_packet_ref.trim().is_empty()
        || posture.recovery_audit_ref.trim().is_empty()
        || !posture.support_export_parity_required
        || !posture.accessibility_parity_required
    {
        violations.push(M5LocalHistoryWriteScopeComponentMatrixViolation::ReleasePostureIncomplete);
    }
}

/// Joins tokens for a CSV cell with a `|` separator so a single cell never
/// introduces a stray comma.
fn join_tokens<T, F>(items: &[T], to_token: F) -> String
where
    F: Fn(&T) -> &'static str,
{
    items.iter().map(to_token).collect::<Vec<_>>().join("|")
}

/// Quotes a free-text CSV field when it contains a comma or quote.
fn csv_field(value: &str) -> String {
    if value.contains(',') || value.contains('"') || value.contains('\n') {
        format!("\"{}\"", value.replace('"', "\"\""))
    } else {
        value.to_owned()
    }
}

/// Heuristic that rejects obviously forbidden material in export-safe JSON.
fn json_contains_forbidden_material(value: &serde_json::Value) -> bool {
    match value {
        serde_json::Value::String(s) => {
            let lower = s.to_lowercase();
            lower.contains("api_key")
                || lower.contains("password")
                || lower.contains("secret")
                || lower.contains("bearer ")
                || lower.contains("://")
        }
        serde_json::Value::Array(arr) => arr.iter().any(json_contains_forbidden_material),
        serde_json::Value::Object(map) => map.values().any(json_contains_forbidden_material),
        _ => false,
    }
}
