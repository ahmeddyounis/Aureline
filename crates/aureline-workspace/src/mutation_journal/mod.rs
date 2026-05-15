//! Workspace mutation-lineage envelopes and export-safe journal projection.
//!
//! This module composes the canonical mutation-journal record types from
//! `aureline-history` with the workspace generated-artifact detector. It gives
//! refactor, formatter, lockfile, build-output, preview-regeneration, and
//! AI-apply paths one support-export-safe envelope without re-minting actor, source,
//! reversal, checkpoint, or redaction vocabularies.

use std::fmt;

use serde::{Deserialize, Serialize};

use crate::generated_artifacts::{detect_lineage, GeneratedArtifactClass, LineageHintRecord};

pub use aureline_history::mutation_journal::{MutationGroupKind, MutationGroupResolution};
pub use aureline_history::{
    ActorClass as MutationActorClass, ActorRef as MutationActorRef,
    CheckpointDurabilityClass as MutationCheckpointDurabilityClass,
    CheckpointKind as MutationCheckpointKind, CheckpointRef as MutationCheckpointRef,
    DurableVsDisposable as MutationDurabilityClass, MutationGroupRecord,
    MutationJournalEntryRecord, RedactionClass as MutationRedactionClass,
    ReversalClass as MutationReversalClass, ScopeClass as MutationScopeClass,
    ScopeRef as MutationScopeRef, SideEffectSummary as MutationSideEffectSummary,
    SourceClass as MutationSourceClass, TargetKind as MutationTargetKind,
    TargetRef as MutationTargetRef,
};

/// Schema version for [`MutationLineageAlphaPacket`] records.
pub const MUTATION_JOURNAL_ALPHA_SCHEMA_VERSION: u32 = 1;

/// Protected mutation paths the alpha packet must prove.
pub const REQUIRED_MUTATION_LINEAGE_ALPHA_PATHS: [MutationPathClass; 6] = [
    MutationPathClass::Refactor,
    MutationPathClass::Formatter,
    MutationPathClass::Lockfile,
    MutationPathClass::BuildOutput,
    MutationPathClass::PreviewRegeneration,
    MutationPathClass::AiApply,
];

/// Stable class for the mutation path represented by an envelope row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MutationPathClass {
    /// Language refactor or rename engine mutation.
    Refactor,
    /// Formatter or save-participant formatting mutation.
    Formatter,
    /// Resolver-created lockfile refresh or write.
    Lockfile,
    /// Build runner writing compiler, bundler, or packager output.
    BuildOutput,
    /// Preview runtime regenerating a derived render snapshot or bundle.
    PreviewRegeneration,
    /// AI patch apply after a reviewed proposal.
    AiApply,
}

impl MutationPathClass {
    /// Returns the schema token for this path class.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Refactor => "refactor",
            Self::Formatter => "formatter",
            Self::Lockfile => "lockfile",
            Self::BuildOutput => "build_output",
            Self::PreviewRegeneration => "preview_regeneration",
            Self::AiApply => "ai_apply",
        }
    }

    fn requires_generated_artifact_cue(self) -> bool {
        matches!(
            self,
            Self::Lockfile | Self::BuildOutput | Self::PreviewRegeneration
        )
    }
}

/// Surface producing or consuming the alpha projection.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MutationLineageConsumerSurface {
    /// Editor save or format-on-save participant surface.
    EditorSave,
    /// Build runner or generated-output producer surface.
    BuildRunner,
    /// Preview runtime or source-first preview surface.
    PreviewRuntime,
    /// AI patch review/apply surface.
    AiApplyReview,
    /// Support bundle or diagnostic export surface.
    SupportExport,
}

/// Journal record kind cited by an alpha row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MutationJournalRecordKind {
    /// One `mutation_journal_entry` record.
    MutationJournalEntry,
    /// One `mutation_group_record` record.
    MutationGroupRecord,
}

impl MutationJournalRecordKind {
    /// Returns the canonical record-kind token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::MutationJournalEntry => "mutation_journal_entry",
            Self::MutationGroupRecord => "mutation_group_record",
        }
    }
}

/// Mutation-journal record carried by a workspace lineage envelope.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(
    tag = "journal_record_kind",
    content = "journal_record",
    rename_all = "snake_case"
)]
pub enum MutationJournalRecord {
    /// Entry record emitted by a single-target or single-step mutation.
    Entry(MutationJournalEntryRecord),
    /// Group record emitted by a named multi-step mutation.
    Group(MutationGroupRecord),
}

impl MutationJournalRecord {
    /// Returns the cited record kind.
    pub const fn record_kind(&self) -> MutationJournalRecordKind {
        match self {
            Self::Entry(_) => MutationJournalRecordKind::MutationJournalEntry,
            Self::Group(_) => MutationJournalRecordKind::MutationGroupRecord,
        }
    }

    /// Returns the stable journal ref carried by support and review surfaces.
    pub fn journal_ref(&self) -> &str {
        match self {
            Self::Entry(entry) => &entry.mutation_id,
            Self::Group(group) => &group.group_id,
        }
    }

    /// Returns the canonical command id recorded on the journal record.
    pub fn command_id(&self) -> &str {
        match self {
            Self::Entry(entry) => &entry.command_id,
            Self::Group(group) => &group.command_id,
        }
    }

    /// Returns the actor class token from the journal record.
    pub fn actor_class_token(&self) -> &'static str {
        match self {
            Self::Entry(entry) => entry.actor_class.as_str(),
            Self::Group(group) => group.actor_class.as_str(),
        }
    }

    /// Returns the source class token from the journal record.
    pub fn source_class_token(&self) -> &'static str {
        match self {
            Self::Entry(entry) => entry.source_class.as_str(),
            Self::Group(group) => group.source_class.as_str(),
        }
    }

    /// Returns the reversal class token from the journal record.
    pub fn reversal_class_token(&self) -> &'static str {
        match self {
            Self::Entry(entry) => entry.reversal_class.as_str(),
            Self::Group(group) => group.reversal_class.as_str(),
        }
    }

    /// Returns the redaction class token from the journal record.
    pub fn redaction_class_token(&self) -> &'static str {
        match self {
            Self::Entry(entry) => entry.redaction_class.as_str(),
            Self::Group(group) => group.redaction_class.as_str(),
        }
    }

    /// Returns the durable/disposable state-class token.
    pub fn durable_vs_disposable_token(&self) -> &'static str {
        match self {
            Self::Entry(entry) => durable_vs_disposable_token(entry.durable_vs_disposable),
            Self::Group(group) => durable_vs_disposable_token(group.durable_vs_disposable),
        }
    }

    /// Returns the undo class token for entry records.
    pub fn undo_class_token(&self) -> Option<&str> {
        match self {
            Self::Entry(entry) => Some(entry.undo_class.as_str()),
            Self::Group(_) => None,
        }
    }

    /// Returns the group-kind token for group records.
    pub fn group_kind_token(&self) -> Option<&'static str> {
        match self {
            Self::Entry(_) => None,
            Self::Group(group) => Some(group_kind_token(group.group_kind)),
        }
    }

    /// Returns the checkpoint ids cited by the journal record.
    pub fn checkpoint_ids(&self) -> Vec<String> {
        match self {
            Self::Entry(entry) => entry
                .checkpoint_refs
                .iter()
                .map(|checkpoint| checkpoint.checkpoint_id.clone())
                .collect(),
            Self::Group(group) => group
                .checkpoint_refs
                .iter()
                .map(|checkpoint| checkpoint.checkpoint_id.clone())
                .collect(),
        }
    }

    /// Returns the redaction-aware side-effect summary.
    pub fn side_effect_summary(&self) -> &str {
        match self {
            Self::Entry(entry) => &entry.side_effect_summary.summary,
            Self::Group(group) => &group.side_effect_summary.summary,
        }
    }
}

/// Reference to the preview that authorized or explained a mutation.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MutationPreviewRef {
    /// Stable preview id.
    pub preview_id: String,
    /// Preview class token from the producing surface.
    pub preview_kind: String,
}

/// Reference to an approval record that admitted the mutation.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MutationApprovalRef {
    /// Stable approval id.
    pub approval_id: String,
    /// Policy or review gate that required the approval.
    pub approval_policy: String,
}

/// Compact generated-artifact cue projected into mutation support rows.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MutationGeneratedArtifactCue {
    /// Workspace-relative path or opaque virtual artifact ref.
    pub generated_relative_path: String,
    /// Generated-artifact class token.
    pub generated_class: String,
    /// Source-canonical workspace-relative path when known.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source_canonical_relative_path: Option<String>,
    /// Stable producer id or runtime id.
    pub producer_id: String,
    /// Human-readable producer label.
    pub producer_label: String,
    /// Freshness token projected from the lineage detector or runtime.
    pub freshness_class: String,
    /// Rule or runtime id that produced this cue.
    pub rule_id: String,
    /// Default edit posture exposed by support and review surfaces.
    pub default_edit_posture: String,
}

impl MutationGeneratedArtifactCue {
    /// Projects a cue from the canonical generated-artifact lineage detector.
    pub fn from_lineage_hint(record: &LineageHintRecord) -> Self {
        Self {
            generated_relative_path: record.generated_relative_path.clone(),
            generated_class: record.generated_class.as_str().to_owned(),
            source_canonical_relative_path: record.source_canonical_relative_path.clone(),
            producer_id: record.producer_id.clone(),
            producer_label: record.producer_label.clone(),
            freshness_class: record.freshness_class.as_str().to_owned(),
            rule_id: record.rule_id.clone(),
            default_edit_posture: edit_posture_for_generated_class(record.generated_class)
                .to_owned(),
        }
    }

    /// Creates a cue for preview/runtime artifacts that do not have a
    /// workspace-relative filesystem path.
    pub fn preview_snapshot(
        artifact_ref: impl Into<String>,
        source_canonical_relative_path: impl Into<String>,
        producer_id: impl Into<String>,
        producer_label: impl Into<String>,
        freshness_class: impl Into<String>,
        rule_id: impl Into<String>,
    ) -> Self {
        Self {
            generated_relative_path: artifact_ref.into(),
            generated_class: "preview_render_snapshot".to_owned(),
            source_canonical_relative_path: Some(source_canonical_relative_path.into()),
            producer_id: producer_id.into(),
            producer_label: producer_label.into(),
            freshness_class: freshness_class.into(),
            rule_id: rule_id.into(),
            default_edit_posture: "inspect_read_only".to_owned(),
        }
    }

    /// True when a source-canonical path is available.
    pub fn has_source_canonical(&self) -> bool {
        self.source_canonical_relative_path.is_some()
    }
}

/// Runtime envelope emitted by one mutation path before export projection.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MutationLineageEnvelope {
    /// Stable envelope id used for row and support joins.
    pub envelope_id: String,
    /// Protected mutation path class.
    pub mutation_path_class: MutationPathClass,
    /// Canonical mutation-journal record emitted by the producer.
    pub journal_record: MutationJournalRecord,
    /// Workspace-relative target path when one exists.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target_relative_path: Option<String>,
    /// Generated-artifact cue when the target is derived.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub generated_artifact_cue: Option<MutationGeneratedArtifactCue>,
    /// Opaque generated-artifact lineage record ref when available.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub generated_artifact_lineage_ref: Option<String>,
    /// Preview record that backed the mutation.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub preview_ref: Option<MutationPreviewRef>,
    /// Approval record that admitted the mutation.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub approval_ref: Option<MutationApprovalRef>,
}

impl MutationLineageEnvelope {
    /// Creates an envelope from a mutation-journal entry.
    pub fn from_entry(
        envelope_id: impl Into<String>,
        mutation_path_class: MutationPathClass,
        entry: MutationJournalEntryRecord,
    ) -> Self {
        Self {
            envelope_id: envelope_id.into(),
            mutation_path_class,
            journal_record: MutationJournalRecord::Entry(entry),
            target_relative_path: None,
            generated_artifact_cue: None,
            generated_artifact_lineage_ref: None,
            preview_ref: None,
            approval_ref: None,
        }
    }

    /// Creates an envelope from a mutation-group record.
    pub fn from_group(
        envelope_id: impl Into<String>,
        mutation_path_class: MutationPathClass,
        group: MutationGroupRecord,
    ) -> Self {
        Self {
            envelope_id: envelope_id.into(),
            mutation_path_class,
            journal_record: MutationJournalRecord::Group(group),
            target_relative_path: None,
            generated_artifact_cue: None,
            generated_artifact_lineage_ref: None,
            preview_ref: None,
            approval_ref: None,
        }
    }

    /// Adds a workspace-relative target path and derives a generated cue when
    /// the default catalog can prove lineage for that path.
    pub fn with_target_relative_path(mut self, relative_path: impl Into<String>) -> Self {
        let relative_path = relative_path.into();
        self.generated_artifact_cue = detect_lineage(&relative_path)
            .as_ref()
            .map(MutationGeneratedArtifactCue::from_lineage_hint);
        self.target_relative_path = Some(relative_path);
        self
    }

    /// Adds or overrides the generated-artifact cue.
    pub fn with_generated_artifact_cue(mut self, cue: MutationGeneratedArtifactCue) -> Self {
        self.generated_artifact_cue = Some(cue);
        self
    }

    /// Adds an opaque generated-artifact lineage record ref.
    pub fn with_generated_artifact_lineage_ref(
        mut self,
        generated_artifact_lineage_ref: impl Into<String>,
    ) -> Self {
        self.generated_artifact_lineage_ref = Some(generated_artifact_lineage_ref.into());
        self
    }

    /// Adds the preview ref that authorized or explained the mutation.
    pub fn with_preview_ref(
        mut self,
        preview_id: impl Into<String>,
        preview_kind: impl Into<String>,
    ) -> Self {
        self.preview_ref = Some(MutationPreviewRef {
            preview_id: preview_id.into(),
            preview_kind: preview_kind.into(),
        });
        self
    }

    /// Adds the approval ref that admitted the mutation.
    pub fn with_approval_ref(
        mut self,
        approval_id: impl Into<String>,
        approval_policy: impl Into<String>,
    ) -> Self {
        self.approval_ref = Some(MutationApprovalRef {
            approval_id: approval_id.into(),
            approval_policy: approval_policy.into(),
        });
        self
    }
}

/// Export-safe packet consumed by support, review, and diagnostics surfaces.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MutationLineageAlphaPacket {
    /// Stable discriminator for this packet.
    pub record_kind: String,
    /// Integer schema version for this alpha projection.
    pub mutation_journal_alpha_schema_version: u32,
    /// Opaque packet id for support and fixture joins.
    pub packet_id: String,
    /// Producer timestamp for this projection.
    pub produced_at: String,
    /// Surface that produced or is consuming this projection.
    pub consumer_surface: MutationLineageConsumerSurface,
    /// Export-safe rows projected from mutation-lineage envelopes.
    pub mutation_lineage_rows: Vec<MutationLineageAlphaRow>,
    /// Export posture shared by every row in this packet.
    pub export_safety: MutationLineageExportSafety,
}

impl MutationLineageAlphaPacket {
    /// Builds a packet from already-emitted mutation-lineage envelopes.
    pub fn from_envelopes(
        packet_id: impl Into<String>,
        produced_at: impl Into<String>,
        consumer_surface: MutationLineageConsumerSurface,
        envelopes: &[MutationLineageEnvelope],
    ) -> Self {
        Self {
            record_kind: "mutation_lineage_alpha_packet".to_owned(),
            mutation_journal_alpha_schema_version: MUTATION_JOURNAL_ALPHA_SCHEMA_VERSION,
            packet_id: packet_id.into(),
            produced_at: produced_at.into(),
            consumer_surface,
            mutation_lineage_rows: envelopes
                .iter()
                .enumerate()
                .map(|(index, envelope)| MutationLineageAlphaRow::from_envelope(index, envelope))
                .collect(),
            export_safety: MutationLineageExportSafety::metadata_only_default(),
        }
    }

    /// Builds and validates a support-export projection.
    pub fn support_export(
        packet_id: impl Into<String>,
        produced_at: impl Into<String>,
        envelopes: &[MutationLineageEnvelope],
    ) -> Result<Self, MutationLineageAlphaValidationError> {
        let packet = Self::from_envelopes(
            packet_id,
            produced_at,
            MutationLineageConsumerSurface::SupportExport,
            envelopes,
        );
        packet.validate()?;
        Ok(packet)
    }

    /// Returns required protected paths not present in this packet.
    pub fn missing_required_alpha_coverage(&self) -> Vec<MutationPathClass> {
        REQUIRED_MUTATION_LINEAGE_ALPHA_PATHS
            .iter()
            .copied()
            .filter(|required| {
                !self
                    .mutation_lineage_rows
                    .iter()
                    .any(|row| row.mutation_path_class == *required)
            })
            .collect()
    }

    /// Validates protected coverage, generated cues, and export safety.
    pub fn validate(&self) -> Result<(), MutationLineageAlphaValidationError> {
        if self.mutation_lineage_rows.is_empty() {
            return Err(MutationLineageAlphaValidationError::EmptyPacket {
                packet_id: self.packet_id.clone(),
            });
        }
        if self.export_safety.raw_payloads_included || self.export_safety.body_object_refs_included
        {
            return Err(
                MutationLineageAlphaValidationError::RawPayloadExportEnabled {
                    packet_id: self.packet_id.clone(),
                },
            );
        }
        if !self
            .export_safety
            .omitted_payload_classes
            .iter()
            .any(|class| class == "raw_secret_material")
        {
            return Err(MutationLineageAlphaValidationError::MissingSecretOmission {
                packet_id: self.packet_id.clone(),
            });
        }
        let missing = self.missing_required_alpha_coverage();
        if !missing.is_empty() {
            return Err(
                MutationLineageAlphaValidationError::MissingRequiredCoverage {
                    packet_id: self.packet_id.clone(),
                    missing,
                },
            );
        }
        for row in &self.mutation_lineage_rows {
            row.validate_export_safe()?;
        }
        Ok(())
    }
}

/// One export-safe row projected from a mutation-lineage envelope.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MutationLineageAlphaRow {
    /// Stable row id for the consuming surface.
    pub row_id: String,
    /// Protected mutation path class.
    pub mutation_path_class: MutationPathClass,
    /// Cited journal record kind.
    pub journal_record_kind: MutationJournalRecordKind,
    /// Cited journal entry or group id.
    pub journal_ref: String,
    /// Canonical command id.
    pub command_id: String,
    /// Actor class token from the journal record.
    pub actor_class: String,
    /// Source class token from the journal record.
    pub source_class: String,
    /// Undo class token for entry rows.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub undo_class: Option<String>,
    /// Group kind token for group rows.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub group_kind: Option<String>,
    /// Reversal class token from the journal record.
    pub reversal_class: String,
    /// Redaction class token from the journal record.
    pub redaction_class: String,
    /// Durable/disposable state-class token from the journal record.
    pub durable_vs_disposable: String,
    /// Workspace-relative target path when one exists.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target_relative_path: Option<String>,
    /// Generated-artifact cue when the target is derived.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub generated_artifact_cue: Option<MutationGeneratedArtifactCue>,
    /// Opaque generated-artifact lineage record ref when available.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub generated_artifact_lineage_ref: Option<String>,
    /// Preview record that backed the mutation.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub preview_ref: Option<MutationPreviewRef>,
    /// Approval record that admitted the mutation.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub approval_ref: Option<MutationApprovalRef>,
    /// Checkpoint refs visible to restore and support surfaces.
    pub checkpoint_refs: Vec<String>,
    /// Redaction-aware summary copied from the journal record.
    pub side_effect_summary: String,
    /// True when the row is safe for support export under metadata-only defaults.
    pub support_export_safe: bool,
    /// True only after an explicit raw payload export path.
    pub raw_payload_exported: bool,
    /// Payload classes deliberately omitted from this row.
    pub omitted_payload_classes: Vec<String>,
}

impl MutationLineageAlphaRow {
    fn from_envelope(index: usize, envelope: &MutationLineageEnvelope) -> Self {
        Self {
            row_id: format!("mutation.lineage.row.{:04}", index + 1),
            mutation_path_class: envelope.mutation_path_class,
            journal_record_kind: envelope.journal_record.record_kind(),
            journal_ref: envelope.journal_record.journal_ref().to_owned(),
            command_id: envelope.journal_record.command_id().to_owned(),
            actor_class: envelope.journal_record.actor_class_token().to_owned(),
            source_class: envelope.journal_record.source_class_token().to_owned(),
            undo_class: envelope
                .journal_record
                .undo_class_token()
                .map(ToOwned::to_owned),
            group_kind: envelope
                .journal_record
                .group_kind_token()
                .map(ToOwned::to_owned),
            reversal_class: envelope.journal_record.reversal_class_token().to_owned(),
            redaction_class: envelope.journal_record.redaction_class_token().to_owned(),
            durable_vs_disposable: envelope
                .journal_record
                .durable_vs_disposable_token()
                .to_owned(),
            target_relative_path: envelope.target_relative_path.clone(),
            generated_artifact_cue: envelope.generated_artifact_cue.clone(),
            generated_artifact_lineage_ref: envelope.generated_artifact_lineage_ref.clone(),
            preview_ref: envelope.preview_ref.clone(),
            approval_ref: envelope.approval_ref.clone(),
            checkpoint_refs: envelope.journal_record.checkpoint_ids(),
            side_effect_summary: envelope.journal_record.side_effect_summary().to_owned(),
            support_export_safe: true,
            raw_payload_exported: false,
            omitted_payload_classes: MutationLineageExportSafety::metadata_only_default()
                .omitted_payload_classes,
        }
    }

    fn validate_export_safe(&self) -> Result<(), MutationLineageAlphaValidationError> {
        if !self.support_export_safe || self.raw_payload_exported {
            return Err(
                MutationLineageAlphaValidationError::RawPayloadExportEnabled {
                    packet_id: self.row_id.clone(),
                },
            );
        }
        if self.mutation_path_class.requires_generated_artifact_cue()
            && self.generated_artifact_cue.is_none()
        {
            return Err(MutationLineageAlphaValidationError::MissingGeneratedCue {
                row_id: self.row_id.clone(),
                mutation_path_class: self.mutation_path_class,
            });
        }
        for reference in self
            .checkpoint_refs
            .iter()
            .chain(std::iter::once(&self.journal_ref))
            .chain(self.generated_artifact_lineage_ref.iter())
        {
            if is_forbidden_export_ref(reference) {
                return Err(MutationLineageAlphaValidationError::RawPayloadRefLeaked {
                    row_id: self.row_id.clone(),
                    leaked_ref: reference.clone(),
                });
            }
        }
        if let Some(preview_ref) = &self.preview_ref {
            for reference in [&preview_ref.preview_id, &preview_ref.preview_kind] {
                if is_forbidden_export_ref(reference) {
                    return Err(MutationLineageAlphaValidationError::RawPayloadRefLeaked {
                        row_id: self.row_id.clone(),
                        leaked_ref: reference.clone(),
                    });
                }
            }
        }
        if let Some(approval_ref) = &self.approval_ref {
            for reference in [&approval_ref.approval_id, &approval_ref.approval_policy] {
                if is_forbidden_export_ref(reference) {
                    return Err(MutationLineageAlphaValidationError::RawPayloadRefLeaked {
                        row_id: self.row_id.clone(),
                        leaked_ref: reference.clone(),
                    });
                }
            }
        }
        Ok(())
    }
}

/// Shared export-safety posture for mutation-lineage alpha packets.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MutationLineageExportSafety {
    /// Default export mode used by support and evidence surfaces.
    pub export_mode: String,
    /// True only after an explicit high-friction raw payload export path.
    pub raw_payloads_included: bool,
    /// True only when content-addressed body refs are intentionally included.
    pub body_object_refs_included: bool,
    /// Payload classes deliberately omitted from this packet.
    pub omitted_payload_classes: Vec<String>,
}

impl MutationLineageExportSafety {
    /// Returns the metadata-only default required for support-safe exports.
    pub fn metadata_only_default() -> Self {
        Self {
            export_mode: "metadata_only".to_owned(),
            raw_payloads_included: false,
            body_object_refs_included: false,
            omitted_payload_classes: vec![
                "raw_file_content".to_owned(),
                "raw_diff_body".to_owned(),
                "raw_prompt_or_model_response".to_owned(),
                "raw_secret_material".to_owned(),
            ],
        }
    }
}

/// Validation error for mutation-lineage alpha packets.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MutationLineageAlphaValidationError {
    /// Packet has no rows.
    EmptyPacket {
        /// Packet id that failed validation.
        packet_id: String,
    },
    /// Packet does not cover one or more protected path classes.
    MissingRequiredCoverage {
        /// Packet id that failed validation.
        packet_id: String,
        /// Missing path classes.
        missing: Vec<MutationPathClass>,
    },
    /// Packet or row attempted to export raw payload material.
    RawPayloadExportEnabled {
        /// Packet id or row id that failed validation.
        packet_id: String,
    },
    /// Packet export posture did not explicitly omit raw secret material.
    MissingSecretOmission {
        /// Packet id that failed validation.
        packet_id: String,
    },
    /// Row leaked a raw body, object, or secret-like reference.
    RawPayloadRefLeaked {
        /// Row id that failed validation.
        row_id: String,
        /// Leaked reference.
        leaked_ref: String,
    },
    /// Derived artifact mutation did not carry a generated-artifact cue.
    MissingGeneratedCue {
        /// Row id that failed validation.
        row_id: String,
        /// Path class that requires the cue.
        mutation_path_class: MutationPathClass,
    },
}

impl fmt::Display for MutationLineageAlphaValidationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyPacket { packet_id } => {
                write!(f, "mutation-lineage packet {packet_id} has no rows")
            }
            Self::MissingRequiredCoverage { packet_id, missing } => write!(
                f,
                "mutation-lineage packet {packet_id} is missing required path coverage: {:?}",
                missing
            ),
            Self::RawPayloadExportEnabled { packet_id } => {
                write!(
                    f,
                    "mutation-lineage packet or row {packet_id} exports raw payloads"
                )
            }
            Self::MissingSecretOmission { packet_id } => write!(
                f,
                "mutation-lineage packet {packet_id} does not declare raw secret omission"
            ),
            Self::RawPayloadRefLeaked { row_id, leaked_ref } => write!(
                f,
                "mutation-lineage row {row_id} leaked raw payload reference {leaked_ref}"
            ),
            Self::MissingGeneratedCue {
                row_id,
                mutation_path_class,
            } => write!(
                f,
                "mutation-lineage row {row_id} for {} is missing a generated-artifact cue",
                mutation_path_class.as_str()
            ),
        }
    }
}

impl std::error::Error for MutationLineageAlphaValidationError {}

fn durable_vs_disposable_token(value: MutationDurabilityClass) -> &'static str {
    match value {
        MutationDurabilityClass::DurableUserAuthored => "durable_user_authored",
        MutationDurabilityClass::DurableWorkspaceAuthored => "durable_workspace_authored",
        MutationDurabilityClass::DisposableDerived => "disposable_derived",
    }
}

fn group_kind_token(value: MutationGroupKind) -> &'static str {
    match value {
        MutationGroupKind::MultiCursorKeystroke => "multi_cursor_keystroke",
        MutationGroupKind::RefactorSingleFile => "refactor_single_file",
        MutationGroupKind::RefactorMultiFile => "refactor_multi_file",
        MutationGroupKind::FormatOnSave => "format_on_save",
        MutationGroupKind::SaveParticipantGroup => "save_participant_group",
        MutationGroupKind::AiPatch => "ai_patch",
        MutationGroupKind::BulkReplace => "bulk_replace",
        MutationGroupKind::MultiFileRename => "multi_file_rename",
        MutationGroupKind::ScaffoldingRun => "scaffolding_run",
        MutationGroupKind::MigrationImport => "migration_import",
        MutationGroupKind::SettingsImport => "settings_import",
        MutationGroupKind::GeneratedArtifactRefresh => "generated_artifact_refresh",
        MutationGroupKind::PreviewRegeneration => "preview_regeneration",
        MutationGroupKind::ExternalReloadGroup => "external_reload_group",
        MutationGroupKind::ReviewApplyGroup => "review_apply_group",
    }
}

fn edit_posture_for_generated_class(class: GeneratedArtifactClass) -> &'static str {
    match class {
        GeneratedArtifactClass::Lockfile => "regenerate_from_source",
        GeneratedArtifactClass::BuildOutput => "inspect_read_only",
        GeneratedArtifactClass::GeneratedSourceSibling => "edit_canonical_source",
        GeneratedArtifactClass::VendoredSnapshot => "replace_by_upstream_snapshot",
    }
}

fn is_forbidden_export_ref(value: &str) -> bool {
    value.starts_with("obj:")
        || value.starts_with("raw:")
        || value.starts_with("secret:")
        || value.starts_with("token:")
}
