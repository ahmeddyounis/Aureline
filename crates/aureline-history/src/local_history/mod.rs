//! Actor-lineage and restore-checkpoint projection for local history.
//!
//! This module is the export-safe consumer shape over the lower-level
//! checkpoint and mutation-journal records. It intentionally carries entry,
//! group, mutation, and checkpoint refs instead of raw snapshot bodies so
//! history, recovery, Git, apply, and support surfaces can inspect lineage
//! without widening data export.

use serde::{Deserialize, Serialize};

use crate::checkpoints::{
    LocalHistoryEntryRecord, LocalHistoryGroupRecord, MutationJournalLinkActorClass,
    MutationJournalLinkKind, SnapshotClass,
};

/// Schema version for [`LocalHistoryAlphaPacket`] records.
pub const LOCAL_HISTORY_ALPHA_SCHEMA_VERSION: u32 = 1;

/// Actor-lineage classes required on the protected alpha rows.
pub const REQUIRED_ALPHA_LINEAGE_CLASSES: [ActorLineageClass; 6] = [
    ActorLineageClass::Typing,
    ActorLineageClass::Import,
    ActorLineageClass::GitMutation,
    ActorLineageClass::Formatter,
    ActorLineageClass::AiApply,
    ActorLineageClass::ReviewApply,
];

/// Export-safe packet consumed by timeline, recovery, Git, apply, and support surfaces.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LocalHistoryAlphaPacket {
    /// Stable discriminator for this packet.
    pub record_kind: String,
    /// Integer schema version for this alpha projection.
    pub local_history_alpha_schema_version: u32,
    /// Opaque packet id for support and fixture joins.
    pub packet_id: String,
    /// Producer timestamp for this projection.
    pub produced_at: String,
    /// Surface that produced or is consuming this projection.
    pub consumer_surface: LocalHistoryConsumerSurface,
    /// Actor-lineage rows visible to the consuming surface.
    pub actor_lineage_rows: Vec<ActorLineageRow>,
    /// Named restore checkpoints visible to recovery and support.
    pub restore_checkpoints: Vec<RestoreCheckpointAlpha>,
    /// Export posture shared by every row in this packet.
    pub export_safety: HistoryArtifactExportSafety,
}

impl LocalHistoryAlphaPacket {
    /// Creates an empty export-safe local-history alpha packet.
    pub fn new(
        packet_id: impl Into<String>,
        produced_at: impl Into<String>,
        consumer_surface: LocalHistoryConsumerSurface,
    ) -> Self {
        Self {
            record_kind: "local_history_alpha_packet".to_owned(),
            local_history_alpha_schema_version: LOCAL_HISTORY_ALPHA_SCHEMA_VERSION,
            packet_id: packet_id.into(),
            produced_at: produced_at.into(),
            consumer_surface,
            actor_lineage_rows: Vec::new(),
            restore_checkpoints: Vec::new(),
            export_safety: HistoryArtifactExportSafety::metadata_only_default(),
        }
    }

    /// Appends an actor-lineage row.
    pub fn with_actor_lineage_row(mut self, row: ActorLineageRow) -> Self {
        self.actor_lineage_rows.push(row);
        self
    }

    /// Appends a named restore checkpoint.
    pub fn with_restore_checkpoint(mut self, checkpoint: RestoreCheckpointAlpha) -> Self {
        self.restore_checkpoints.push(checkpoint);
        self
    }

    /// Returns required protected classes not present in this packet.
    pub fn missing_required_alpha_coverage(&self) -> Vec<ActorLineageClass> {
        REQUIRED_ALPHA_LINEAGE_CLASSES
            .iter()
            .copied()
            .filter(|required| {
                !self
                    .actor_lineage_rows
                    .iter()
                    .any(|row| row.actor_lineage_class == *required)
            })
            .collect()
    }

    /// Validates baseline export safety and restore-checkpoint visibility.
    pub fn validate(&self) -> Result<(), LocalHistoryAlphaValidationError> {
        if self.actor_lineage_rows.is_empty() {
            return Err(LocalHistoryAlphaValidationError::EmptyPacket {
                packet_id: self.packet_id.clone(),
            });
        }
        if self.export_safety.raw_snapshot_bodies_included
            || self.export_safety.body_object_refs_included
        {
            return Err(LocalHistoryAlphaValidationError::RawBodyExportEnabled {
                packet_id: self.packet_id.clone(),
            });
        }
        for row in &self.actor_lineage_rows {
            row.validate_export_safe()?;
        }
        for checkpoint in &self.restore_checkpoints {
            checkpoint.validate_export_safe()?;
        }
        Ok(())
    }
}

/// Surface consuming or producing a local-history actor-lineage projection.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LocalHistoryConsumerSurface {
    /// Editor save or dirty-buffer history surface.
    EditorSave,
    /// Local-history timeline or compare surface.
    HistoryTimeline,
    /// Git mutation review or recovery surface.
    GitMutationReview,
    /// Preview/apply/revert review surface.
    ReviewApply,
    /// Restore preview or recovery support surface.
    RestorePreview,
    /// Support-bundle or diagnostic export surface.
    SupportExport,
}

/// Protected actor-lineage class rendered on a history row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ActorLineageClass {
    /// Direct typing or multi-cursor text input.
    Typing,
    /// Paste, drag/drop, migration, vendor, or replay import.
    Import,
    /// Local Git mutation such as stage, unstage, discard, or restore checkpoint.
    GitMutation,
    /// Formatter or save-participant write.
    Formatter,
    /// AI model or provider apply.
    AiApply,
    /// Review/apply workflow such as bulk replace or structured patch apply.
    ReviewApply,
    /// Restore that creates a new checkpoint.
    RestoreCheckpoint,
    /// Automation, scaffold, recipe, or migration runner.
    Automation,
    /// Repair or decode-recovery runner.
    Repair,
    /// External change detector or reload observation.
    ExternalChange,
    /// Known actor class that does not fit a protected alpha bucket.
    Other,
}

impl ActorLineageClass {
    /// Returns the schema token for this lineage class.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Typing => "typing",
            Self::Import => "import",
            Self::GitMutation => "git_mutation",
            Self::Formatter => "formatter",
            Self::AiApply => "ai_apply",
            Self::ReviewApply => "review_apply",
            Self::RestoreCheckpoint => "restore_checkpoint",
            Self::Automation => "automation",
            Self::Repair => "repair",
            Self::ExternalChange => "external_change",
            Self::Other => "other",
        }
    }

    fn from_actor(actor_class: MutationJournalLinkActorClass) -> Self {
        match actor_class {
            MutationJournalLinkActorClass::UserKeystroke
            | MutationJournalLinkActorClass::MultiCursorCommand => Self::Typing,
            MutationJournalLinkActorClass::PasteOrDropImport
            | MutationJournalLinkActorClass::ReplayImport
            | MutationJournalLinkActorClass::VendorImport
            | MutationJournalLinkActorClass::SettingsImport
            | MutationJournalLinkActorClass::WorkspaceMigration => Self::Import,
            MutationJournalLinkActorClass::Formatter
            | MutationJournalLinkActorClass::SaveParticipant => Self::Formatter,
            MutationJournalLinkActorClass::AiApply => Self::AiApply,
            MutationJournalLinkActorClass::ReviewApply => Self::ReviewApply,
            MutationJournalLinkActorClass::RestoreRollbackRunner => Self::RestoreCheckpoint,
            MutationJournalLinkActorClass::AutomationRecipeRunner
            | MutationJournalLinkActorClass::Scaffolding
            | MutationJournalLinkActorClass::PreviewRegenerator => Self::Automation,
            MutationJournalLinkActorClass::DecodeRecovery => Self::Repair,
            MutationJournalLinkActorClass::ExternalChangeDetector
            | MutationJournalLinkActorClass::ExternalReload => Self::ExternalChange,
            MutationJournalLinkActorClass::UserCommand
            | MutationJournalLinkActorClass::RefactorEngine
            | MutationJournalLinkActorClass::CodeAction
            | MutationJournalLinkActorClass::BuildRunner
            | MutationJournalLinkActorClass::CodegenRunner => Self::Other,
        }
    }
}

/// Export mode for history evidence crossing a support or evidence boundary.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum HistoryExportMode {
    /// Export only structured metadata and durable ids.
    MetadataOnly,
    /// Export entry and group refs that can be resolved locally.
    EntryRefsOnly,
}

/// Shared export-safety posture for local-history alpha packets.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HistoryArtifactExportSafety {
    /// Default export mode used by support and evidence surfaces.
    pub export_mode: HistoryExportMode,
    /// True only after an explicit high-friction raw body export path.
    pub raw_snapshot_bodies_included: bool,
    /// True only when content-addressed object refs are intentionally included.
    pub body_object_refs_included: bool,
    /// Payload classes deliberately omitted from this packet.
    pub omitted_payload_classes: Vec<String>,
}

impl HistoryArtifactExportSafety {
    /// Returns the metadata-only default required for support-safe exports.
    pub fn metadata_only_default() -> Self {
        Self {
            export_mode: HistoryExportMode::MetadataOnly,
            raw_snapshot_bodies_included: false,
            body_object_refs_included: false,
            omitted_payload_classes: vec![
                "raw_file_snapshot".to_owned(),
                "content_addressed_body_object_ref".to_owned(),
                "raw_diff_body".to_owned(),
            ],
        }
    }
}

/// One export-safe actor-lineage row projected from local history or Git.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ActorLineageRow {
    /// Stable row id for the consuming surface.
    pub row_id: String,
    /// Compact label rendered by recovery and support surfaces.
    pub display_label: String,
    /// Protected actor-lineage bucket.
    pub actor_lineage_class: ActorLineageClass,
    /// Original actor-class token from the source record.
    pub actor_class: String,
    /// Original source-class token from the source record.
    pub source_class: String,
    /// Original reversal-class token from the source record.
    pub reversal_class: String,
    /// Original redaction-class token from the source record.
    pub redaction_class: String,
    /// Snapshot class associated with this row.
    pub snapshot_class: String,
    /// Capture mode for entry rows, omitted for group-only or Git-only rows.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub capture_mode: Option<String>,
    /// Omission reason for entry rows, omitted for group-only or Git-only rows.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub omission_reason: Option<String>,
    /// Opaque local-history entry refs cited by this row.
    pub local_history_entry_refs: Vec<String>,
    /// Opaque local-history group ref cited by this row.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub local_history_group_ref: Option<String>,
    /// Opaque mutation-journal entry or group ref cited by this row.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mutation_journal_ref: Option<String>,
    /// Canonical command id when the source surface provides one.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub command_id: Option<String>,
    /// Checkpoint refs visible to restore and support surfaces.
    pub checkpoint_refs: Vec<String>,
    /// Redaction-aware summary copied from the source surface.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub side_effect_summary: Option<String>,
    /// True when the original checkpoint has bytes available locally.
    pub body_available_locally: bool,
    /// True only if raw body refs were intentionally exported.
    pub raw_body_refs_exported: bool,
}

impl ActorLineageRow {
    /// Projects a local-history entry into an export-safe actor-lineage row.
    pub fn try_from_entry(
        row_id: impl Into<String>,
        display_label: impl Into<String>,
        command_id: Option<String>,
        entry: &LocalHistoryEntryRecord,
    ) -> Result<Self, LocalHistoryAlphaValidationError> {
        let actor_class = entry.mutation_journal_link.actor_class.ok_or_else(|| {
            LocalHistoryAlphaValidationError::MissingLineageField {
                record_ref: entry.entry_id.clone(),
                field_name: "mutation_journal_link.actor_class",
            }
        })?;
        let source_class = entry.mutation_journal_link.source_class.ok_or_else(|| {
            LocalHistoryAlphaValidationError::MissingLineageField {
                record_ref: entry.entry_id.clone(),
                field_name: "mutation_journal_link.source_class",
            }
        })?;
        let reversal_class = entry.mutation_journal_link.reversal_class.ok_or_else(|| {
            LocalHistoryAlphaValidationError::MissingLineageField {
                record_ref: entry.entry_id.clone(),
                field_name: "mutation_journal_link.reversal_class",
            }
        })?;
        let redaction_class = entry
            .mutation_journal_link
            .redaction_class
            .map(|class| class.as_str().to_owned())
            .unwrap_or_else(|| "metadata_only".to_owned());

        Ok(Self {
            row_id: row_id.into(),
            display_label: display_label.into(),
            actor_lineage_class: ActorLineageClass::from_actor(actor_class),
            actor_class: actor_class.as_str().to_owned(),
            source_class: source_class.as_str().to_owned(),
            reversal_class: reversal_class.as_str().to_owned(),
            redaction_class,
            snapshot_class: entry.snapshot_class.as_str().to_owned(),
            capture_mode: Some(entry.capture_descriptor.capture_mode.as_str().to_owned()),
            omission_reason: Some(entry.capture_descriptor.omission_reason.as_str().to_owned()),
            local_history_entry_refs: vec![entry.entry_id.clone()],
            local_history_group_ref: entry.group_id.clone(),
            mutation_journal_ref: mutation_ref(
                entry.mutation_journal_link.linked_kind,
                &entry.mutation_journal_link.linked_id,
            ),
            command_id,
            checkpoint_refs: vec![entry.entry_id.clone()],
            side_effect_summary: entry.side_effect_summary.clone(),
            body_available_locally: entry.capture_descriptor.body_available,
            raw_body_refs_exported: false,
        })
    }

    /// Projects a local-history group into an export-safe actor-lineage row.
    pub fn try_from_group(
        row_id: impl Into<String>,
        display_label: impl Into<String>,
        command_id: Option<String>,
        group: &LocalHistoryGroupRecord,
    ) -> Result<Self, LocalHistoryAlphaValidationError> {
        let actor_class = group.mutation_journal_link.actor_class.ok_or_else(|| {
            LocalHistoryAlphaValidationError::MissingLineageField {
                record_ref: group.group_id.clone(),
                field_name: "mutation_journal_link.actor_class",
            }
        })?;
        let source_class = group.mutation_journal_link.source_class.ok_or_else(|| {
            LocalHistoryAlphaValidationError::MissingLineageField {
                record_ref: group.group_id.clone(),
                field_name: "mutation_journal_link.source_class",
            }
        })?;
        let reversal_class = group.mutation_journal_link.reversal_class.ok_or_else(|| {
            LocalHistoryAlphaValidationError::MissingLineageField {
                record_ref: group.group_id.clone(),
                field_name: "mutation_journal_link.reversal_class",
            }
        })?;
        let redaction_class = group
            .mutation_journal_link
            .redaction_class
            .map(|class| class.as_str().to_owned())
            .unwrap_or_else(|| "metadata_only".to_owned());

        Ok(Self {
            row_id: row_id.into(),
            display_label: display_label.into(),
            actor_lineage_class: ActorLineageClass::from_actor(actor_class),
            actor_class: actor_class.as_str().to_owned(),
            source_class: source_class.as_str().to_owned(),
            reversal_class: reversal_class.as_str().to_owned(),
            redaction_class,
            snapshot_class: group.snapshot_class.as_str().to_owned(),
            capture_mode: None,
            omission_reason: None,
            local_history_entry_refs: group.member_entry_ids.clone(),
            local_history_group_ref: Some(group.group_id.clone()),
            mutation_journal_ref: mutation_ref(
                group.mutation_journal_link.linked_kind,
                &group.mutation_journal_link.linked_id,
            ),
            command_id,
            checkpoint_refs: vec![group.group_id.clone()],
            side_effect_summary: group.side_effect_summary.clone(),
            body_available_locally: false,
            raw_body_refs_exported: false,
        })
    }

    /// Projects a Git mutation result into the shared local-history lineage shape.
    pub fn from_git_mutation(input: GitMutationLineageInput) -> Self {
        let checkpoint_refs = non_empty_vec(input.checkpoint_ref);
        Self {
            row_id: input.row_id,
            display_label: input.display_label,
            actor_lineage_class: ActorLineageClass::GitMutation,
            actor_class: input.actor_class,
            source_class: input.source_class,
            reversal_class: input.reversal_class,
            redaction_class: input.redaction_class,
            snapshot_class: SnapshotClass::ExternalStateCheckpoint.as_str().to_owned(),
            capture_mode: None,
            omission_reason: None,
            local_history_entry_refs: Vec::new(),
            local_history_group_ref: None,
            mutation_journal_ref: Some(input.mutation_journal_ref),
            command_id: Some(input.command_id),
            checkpoint_refs,
            side_effect_summary: Some(input.side_effect_summary),
            body_available_locally: false,
            raw_body_refs_exported: false,
        }
    }

    /// Projects a review apply group when the caller only has phase records.
    pub fn from_review_apply(input: ReviewApplyLineageInput) -> Self {
        Self {
            row_id: input.row_id,
            display_label: input.display_label,
            actor_lineage_class: ActorLineageClass::ReviewApply,
            actor_class: "review_apply".to_owned(),
            source_class: "human_local".to_owned(),
            reversal_class: input.reversal_class,
            redaction_class: "code_adjacent".to_owned(),
            snapshot_class: SnapshotClass::WorkspaceMutationCheckpoint
                .as_str()
                .to_owned(),
            capture_mode: None,
            omission_reason: None,
            local_history_entry_refs: input.local_history_entry_refs,
            local_history_group_ref: Some(input.local_history_group_ref.clone()),
            mutation_journal_ref: Some(input.mutation_group_ref),
            command_id: Some(input.command_id),
            checkpoint_refs: vec![input.local_history_group_ref],
            side_effect_summary: Some(input.side_effect_summary),
            body_available_locally: true,
            raw_body_refs_exported: false,
        }
    }

    fn validate_export_safe(&self) -> Result<(), LocalHistoryAlphaValidationError> {
        if self.raw_body_refs_exported {
            return Err(LocalHistoryAlphaValidationError::RawBodyRefLeaked {
                row_id: self.row_id.clone(),
                leaked_ref: "raw_body_refs_exported".to_owned(),
            });
        }
        for reference in self
            .local_history_entry_refs
            .iter()
            .chain(self.checkpoint_refs.iter())
            .chain(self.mutation_journal_ref.iter())
        {
            if reference.starts_with("obj:") {
                return Err(LocalHistoryAlphaValidationError::RawBodyRefLeaked {
                    row_id: self.row_id.clone(),
                    leaked_ref: reference.clone(),
                });
            }
        }
        Ok(())
    }
}

/// Input used to project Git mutation results without depending on Git types.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GitMutationLineageInput {
    /// Stable lineage row id.
    pub row_id: String,
    /// Compact display label.
    pub display_label: String,
    /// Mutation-journal ref emitted by the Git lane.
    pub mutation_journal_ref: String,
    /// Canonical Git command id.
    pub command_id: String,
    /// Git actor class token.
    pub actor_class: String,
    /// Git source class token.
    pub source_class: String,
    /// Git reversal class token.
    pub reversal_class: String,
    /// Git redaction class token.
    pub redaction_class: String,
    /// Optional checkpoint ref bound to the Git mutation.
    pub checkpoint_ref: Option<String>,
    /// Redaction-aware summary from the Git result.
    pub side_effect_summary: String,
}

/// Input used to project review apply phase records.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReviewApplyLineageInput {
    /// Stable lineage row id.
    pub row_id: String,
    /// Compact display label.
    pub display_label: String,
    /// Local-history group checkpoint ref.
    pub local_history_group_ref: String,
    /// Per-target local-history entry refs.
    pub local_history_entry_refs: Vec<String>,
    /// Mutation-group ref emitted by the apply lane.
    pub mutation_group_ref: String,
    /// Canonical apply command id.
    pub command_id: String,
    /// Reversal class shown after apply.
    pub reversal_class: String,
    /// Redaction-aware summary from the apply lane.
    pub side_effect_summary: String,
}

/// Named restore checkpoint visible in recovery and support surfaces.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RestoreCheckpointAlpha {
    /// Stable checkpoint entry or group ref.
    pub checkpoint_ref: String,
    /// Human-readable checkpoint name rendered by recovery surfaces.
    pub checkpoint_name: String,
    /// Timestamp copied from the restore entry or group.
    pub created_at: String,
    /// Source entry restored by this checkpoint.
    pub restored_from_entry_ref: String,
    /// Restore-preview source entry ref.
    pub source_entry_ref: String,
    /// Body-availability class shown in restore preview.
    pub body_availability: String,
    /// Resulting snapshot class, always restore rollback for successful restores.
    pub resulting_snapshot_class: String,
    /// Command id that reopens or reruns the restore path.
    pub restore_command_id: String,
    /// Support-export row that cites this checkpoint.
    pub support_export_ref: String,
    /// True only if raw body refs were intentionally exported.
    pub raw_body_refs_exported: bool,
}

impl RestoreCheckpointAlpha {
    /// Projects a restore checkpoint from a local-history restore entry.
    pub fn try_from_restore_entry(
        checkpoint_name: impl Into<String>,
        support_export_ref: impl Into<String>,
        entry: &LocalHistoryEntryRecord,
    ) -> Result<Self, LocalHistoryAlphaValidationError> {
        if entry.snapshot_class != SnapshotClass::RestoreRollbackCheckpoint {
            return Err(LocalHistoryAlphaValidationError::NotRestoreCheckpoint {
                record_ref: entry.entry_id.clone(),
            });
        }
        let restore_of_ref = entry.restore_of_ref.as_ref().ok_or_else(|| {
            LocalHistoryAlphaValidationError::MissingLineageField {
                record_ref: entry.entry_id.clone(),
                field_name: "restore_of_ref",
            }
        })?;
        Ok(Self {
            checkpoint_ref: entry.entry_id.clone(),
            checkpoint_name: checkpoint_name.into(),
            created_at: entry.captured_at.clone(),
            restored_from_entry_ref: restore_of_ref.restored_from_entry_id.clone(),
            source_entry_ref: restore_of_ref.restore_preview.source_entry_ref.clone(),
            body_availability: restore_of_ref.restore_preview.body_availability.clone(),
            resulting_snapshot_class: restore_of_ref
                .restore_preview
                .resulting_snapshot_class
                .clone(),
            restore_command_id: "cmd:local_history.restore_checkpoint".to_owned(),
            support_export_ref: support_export_ref.into(),
            raw_body_refs_exported: false,
        })
    }

    fn validate_export_safe(&self) -> Result<(), LocalHistoryAlphaValidationError> {
        if self.checkpoint_name.trim().is_empty() {
            return Err(LocalHistoryAlphaValidationError::MissingCheckpointName {
                checkpoint_ref: self.checkpoint_ref.clone(),
            });
        }
        if self.raw_body_refs_exported {
            return Err(LocalHistoryAlphaValidationError::RawBodyRefLeaked {
                row_id: self.checkpoint_ref.clone(),
                leaked_ref: "raw_body_refs_exported".to_owned(),
            });
        }
        if self.resulting_snapshot_class != SnapshotClass::RestoreRollbackCheckpoint.as_str() {
            return Err(LocalHistoryAlphaValidationError::NotRestoreCheckpoint {
                record_ref: self.checkpoint_ref.clone(),
            });
        }
        if self.checkpoint_ref.starts_with("obj:")
            || self.restored_from_entry_ref.starts_with("obj:")
            || self.source_entry_ref.starts_with("obj:")
        {
            return Err(LocalHistoryAlphaValidationError::RawBodyRefLeaked {
                row_id: self.checkpoint_ref.clone(),
                leaked_ref: self.checkpoint_ref.clone(),
            });
        }
        Ok(())
    }
}

/// Validation error for local-history actor-lineage alpha packets.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LocalHistoryAlphaValidationError {
    /// Packet has no actor-lineage rows.
    EmptyPacket {
        /// Packet id that failed validation.
        packet_id: String,
    },
    /// A source record is missing required lineage.
    MissingLineageField {
        /// Entry, group, or checkpoint ref that failed validation.
        record_ref: String,
        /// Missing field name.
        field_name: &'static str,
    },
    /// A restore checkpoint lacks a human-readable name.
    MissingCheckpointName {
        /// Checkpoint ref that failed validation.
        checkpoint_ref: String,
    },
    /// The packet enabled raw body export by default.
    RawBodyExportEnabled {
        /// Packet id that failed validation.
        packet_id: String,
    },
    /// A row leaked a content-addressed body ref into the export projection.
    RawBodyRefLeaked {
        /// Row id that failed validation.
        row_id: String,
        /// Ref or flag that leaked raw body posture.
        leaked_ref: String,
    },
    /// A restore projection was requested from a non-restore entry.
    NotRestoreCheckpoint {
        /// Entry ref that failed validation.
        record_ref: String,
    },
}

impl std::fmt::Display for LocalHistoryAlphaValidationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::EmptyPacket { packet_id } => {
                write!(f, "local-history alpha packet {packet_id} has no rows")
            }
            Self::MissingLineageField {
                record_ref,
                field_name,
            } => write!(
                f,
                "local-history record {record_ref} is missing {field_name}"
            ),
            Self::MissingCheckpointName { checkpoint_ref } => {
                write!(f, "restore checkpoint {checkpoint_ref} has no name")
            }
            Self::RawBodyExportEnabled { packet_id } => write!(
                f,
                "local-history alpha packet {packet_id} enables raw body export"
            ),
            Self::RawBodyRefLeaked { row_id, leaked_ref } => {
                write!(f, "local-history row {row_id} leaked {leaked_ref}")
            }
            Self::NotRestoreCheckpoint { record_ref } => {
                write!(
                    f,
                    "local-history entry {record_ref} is not a restore checkpoint"
                )
            }
        }
    }
}

impl std::error::Error for LocalHistoryAlphaValidationError {}

fn mutation_ref(kind: MutationJournalLinkKind, linked_id: &str) -> Option<String> {
    if kind == MutationJournalLinkKind::NoMutationJournalEntryExternalCause
        || linked_id.trim().is_empty()
    {
        None
    } else {
        Some(linked_id.to_owned())
    }
}

fn non_empty_vec(value: Option<String>) -> Vec<String> {
    value
        .into_iter()
        .filter(|value| !value.is_empty())
        .collect()
}
