//! Mutation-journal producer registry and metadata-only emitters.
//!
//! The registry pins the six alpha producer surfaces that must emit the
//! canonical [`MutationJournalEntryRecord`] kind: refactor, formatter,
//! AI apply, build-output observation, lockfile observation, and preview
//! regeneration. The emitters build journal records from metadata, checkpoint
//! refs, preview or approval refs, generated-artifact lineage refs, and a
//! diff identity ref; they never accept or store raw mutation bytes.

use std::fmt;

use aureline_buffer::UndoClass;
use aureline_records::{validate_typed, RecordClassId, RecordRegistryError};

use super::{
    ActorClass, ActorRef, AiApplyLineage, ApprovalRef, CheckpointRef, DurableVsDisposable,
    MutationGroupKind, MutationJournalEntryRecord, PreviewKind, PreviewRef, RedactionClass,
    ReversalClass, ScopeRef, SideEffectSummary, SourceClass, TargetKind, TargetRef,
    MUTATION_JOURNAL_ENTRY_RECORD_KIND,
};

/// Producer surfaces that must write mutation-journal entries in the alpha.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum MutationProducerClass {
    /// Language refactor or rename engine.
    Refactor,
    /// Formatter or format-on-save participant.
    Formatter,
    /// AI patch apply after review.
    AiApply,
    /// Build runner observing generated output writes.
    BuildOutput,
    /// Resolver or package workflow observing lockfile writes.
    Lockfile,
    /// Preview runtime regenerating derived snapshots or bundles.
    Preview,
}

impl MutationProducerClass {
    /// Returns the stable producer-class token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Refactor => "refactor",
            Self::Formatter => "formatter",
            Self::AiApply => "ai_apply",
            Self::BuildOutput => "build_output",
            Self::Lockfile => "lockfile",
            Self::Preview => "preview",
        }
    }
}

/// The required alpha producer classes, in stable test and export order.
pub const REQUIRED_MUTATION_PRODUCER_CLASSES: [MutationProducerClass; 6] = [
    MutationProducerClass::Refactor,
    MutationProducerClass::Formatter,
    MutationProducerClass::AiApply,
    MutationProducerClass::BuildOutput,
    MutationProducerClass::Lockfile,
    MutationProducerClass::Preview,
];

/// Static producer binding used by emitters and coverage tests.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MutationProducerBinding {
    /// Producer surface being bound.
    pub producer_class: MutationProducerClass,
    /// Canonical journal record kind emitted by this producer.
    pub emitted_record_kind: &'static str,
    /// Canonical command id prefix expected for the producer.
    pub command_id_prefix: &'static str,
    /// Actor class written to the journal record.
    pub actor_class: ActorClass,
    /// Source class written to the journal record.
    pub source_class: SourceClass,
    /// Default target kind for the producer's primary mutation target.
    pub primary_target_kind: TargetKind,
    /// Undo-class token written to the journal record.
    pub undo_class: &'static str,
    /// Reversal class written to the journal record.
    pub reversal_class: ReversalClass,
    /// Redaction posture written to the journal record.
    pub redaction_class: RedactionClass,
    /// Durable/disposable state class written to the journal record.
    pub durable_vs_disposable: DurableVsDisposable,
    /// Group-kind hint when the surface normally belongs to a named group.
    pub group_kind_hint: Option<MutationGroupKind>,
    /// Preview kind required by this producer, when applicable.
    pub required_preview_kind: Option<PreviewKind>,
    /// True when an approval ref is mandatory.
    pub requires_approval_ref: bool,
    /// True when a generated-artifact lineage ref is mandatory.
    pub requires_generated_artifact_lineage_ref: bool,
}

/// Canonical producer registry for the six alpha mutation-journal surfaces.
pub const MUTATION_PRODUCER_REGISTRY: [MutationProducerBinding; 6] = [
    MutationProducerBinding {
        producer_class: MutationProducerClass::Refactor,
        emitted_record_kind: MUTATION_JOURNAL_ENTRY_RECORD_KIND,
        command_id_prefix: "language.refactor",
        actor_class: ActorClass::RefactorEngine,
        source_class: SourceClass::MachineLocal,
        primary_target_kind: TargetKind::Buffer,
        undo_class: UndoClass::RefactorMultiFile.class_id(),
        reversal_class: ReversalClass::CompensatingUndo,
        redaction_class: RedactionClass::CodeAdjacent,
        durable_vs_disposable: DurableVsDisposable::DurableUserAuthored,
        group_kind_hint: Some(MutationGroupKind::RefactorMultiFile),
        required_preview_kind: Some(PreviewKind::RefactorPreview),
        requires_approval_ref: false,
        requires_generated_artifact_lineage_ref: false,
    },
    MutationProducerBinding {
        producer_class: MutationProducerClass::Formatter,
        emitted_record_kind: MUTATION_JOURNAL_ENTRY_RECORD_KIND,
        command_id_prefix: "editor.format",
        actor_class: ActorClass::Formatter,
        source_class: SourceClass::MachineLocal,
        primary_target_kind: TargetKind::Buffer,
        undo_class: UndoClass::FormatterRun.class_id(),
        reversal_class: ReversalClass::CompensatingUndo,
        redaction_class: RedactionClass::CodeAdjacent,
        durable_vs_disposable: DurableVsDisposable::DurableUserAuthored,
        group_kind_hint: Some(MutationGroupKind::FormatOnSave),
        required_preview_kind: None,
        requires_approval_ref: false,
        requires_generated_artifact_lineage_ref: false,
    },
    MutationProducerBinding {
        producer_class: MutationProducerClass::AiApply,
        emitted_record_kind: MUTATION_JOURNAL_ENTRY_RECORD_KIND,
        command_id_prefix: "ai.apply",
        actor_class: ActorClass::AiApply,
        source_class: SourceClass::AiHostedProvider,
        primary_target_kind: TargetKind::Buffer,
        undo_class: UndoClass::MachineGeneratedChange.class_id(),
        reversal_class: ReversalClass::CompensatingUndo,
        redaction_class: RedactionClass::CodeAdjacent,
        durable_vs_disposable: DurableVsDisposable::DurableUserAuthored,
        group_kind_hint: Some(MutationGroupKind::AiPatch),
        required_preview_kind: Some(PreviewKind::AiPatchPreview),
        requires_approval_ref: true,
        requires_generated_artifact_lineage_ref: false,
    },
    MutationProducerBinding {
        producer_class: MutationProducerClass::BuildOutput,
        emitted_record_kind: MUTATION_JOURNAL_ENTRY_RECORD_KIND,
        command_id_prefix: "build",
        actor_class: ActorClass::BuildRunner,
        source_class: SourceClass::MachineLocal,
        primary_target_kind: TargetKind::GeneratedArtifact,
        undo_class: UndoClass::MachineGeneratedChange.class_id(),
        reversal_class: ReversalClass::RegenerateOrRecompute,
        redaction_class: RedactionClass::EnvironmentAdjacent,
        durable_vs_disposable: DurableVsDisposable::DisposableDerived,
        group_kind_hint: Some(MutationGroupKind::GeneratedArtifactRefresh),
        required_preview_kind: None,
        requires_approval_ref: false,
        requires_generated_artifact_lineage_ref: true,
    },
    MutationProducerBinding {
        producer_class: MutationProducerClass::Lockfile,
        emitted_record_kind: MUTATION_JOURNAL_ENTRY_RECORD_KIND,
        command_id_prefix: "package.lockfile",
        actor_class: ActorClass::CodegenRunner,
        source_class: SourceClass::MachineLocal,
        primary_target_kind: TargetKind::GeneratedArtifact,
        undo_class: UndoClass::MachineGeneratedChange.class_id(),
        reversal_class: ReversalClass::RegenerateOrRecompute,
        redaction_class: RedactionClass::EnvironmentAdjacent,
        durable_vs_disposable: DurableVsDisposable::DurableWorkspaceAuthored,
        group_kind_hint: Some(MutationGroupKind::GeneratedArtifactRefresh),
        required_preview_kind: None,
        requires_approval_ref: false,
        requires_generated_artifact_lineage_ref: true,
    },
    MutationProducerBinding {
        producer_class: MutationProducerClass::Preview,
        emitted_record_kind: MUTATION_JOURNAL_ENTRY_RECORD_KIND,
        command_id_prefix: "preview",
        actor_class: ActorClass::PreviewRegenerator,
        source_class: SourceClass::MachineLocal,
        primary_target_kind: TargetKind::GeneratedArtifact,
        undo_class: UndoClass::MachineGeneratedChange.class_id(),
        reversal_class: ReversalClass::RegenerateOrRecompute,
        redaction_class: RedactionClass::EnvironmentAdjacent,
        durable_vs_disposable: DurableVsDisposable::DisposableDerived,
        group_kind_hint: Some(MutationGroupKind::PreviewRegeneration),
        required_preview_kind: Some(PreviewKind::GeneratedArtifactRefreshPreview),
        requires_approval_ref: false,
        requires_generated_artifact_lineage_ref: true,
    },
];

/// Metadata used to emit one producer-owned journal entry.
#[derive(Debug, Clone, PartialEq)]
pub struct MutationProducerInput {
    /// Stable mutation id minted before commit.
    pub mutation_id: String,
    /// Optional group id for multi-file or reviewed apply paths.
    pub group_id: Option<String>,
    /// Canonical command id that committed the mutation.
    pub command_id: String,
    /// Actor reference for the producer or reviewed tool.
    pub actor_ref: ActorRef,
    /// Scope in which the mutation applied.
    pub scope_ref: ScopeRef,
    /// Mutation targets; raw target bodies are never accepted here.
    pub target_refs: Vec<TargetRef>,
    /// Producer-local start timestamp.
    pub started_at: String,
    /// Producer-local commit timestamp.
    pub committed_at: String,
    /// Metadata-only diff identity, not the raw diff body.
    pub diff_identity_ref: String,
    /// Redaction-aware summary shown to review and support surfaces.
    pub side_effect_summary: String,
    /// Number of bytes written, when known.
    pub bytes_written: Option<u64>,
    /// Number of files or artifacts touched, when known.
    pub files_touched: Option<u64>,
    /// Checkpoints or save manifests cited by the mutation.
    pub checkpoint_refs: Vec<CheckpointRef>,
    /// Preview lineage required for reviewed producer paths.
    pub preview_ref: Option<PreviewRef>,
    /// Approval lineage required for AI apply and policy-gated paths.
    pub approval_ref: Option<ApprovalRef>,
    /// AI evidence, route, spend, and taint-fence refs for AI apply.
    pub ai_apply_lineage: Option<AiApplyLineage>,
    /// Save-manifest ref emitted by the save pipeline, when present.
    pub save_manifest_ref: Option<String>,
    /// Generated-artifact lineage ref for derived targets, when present.
    pub generated_artifact_lineage_ref: Option<String>,
}

impl MutationProducerInput {
    /// Builds a minimal input with no raw mutation body.
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        mutation_id: impl Into<String>,
        command_id: impl Into<String>,
        actor_ref: ActorRef,
        scope_ref: ScopeRef,
        target_refs: Vec<TargetRef>,
        timestamp: impl Into<String>,
        diff_identity_ref: impl Into<String>,
        side_effect_summary: impl Into<String>,
    ) -> Self {
        let timestamp = timestamp.into();
        Self {
            mutation_id: mutation_id.into(),
            group_id: None,
            command_id: command_id.into(),
            actor_ref,
            scope_ref,
            target_refs,
            started_at: timestamp.clone(),
            committed_at: timestamp,
            diff_identity_ref: diff_identity_ref.into(),
            side_effect_summary: side_effect_summary.into(),
            bytes_written: None,
            files_touched: None,
            checkpoint_refs: Vec::new(),
            preview_ref: None,
            approval_ref: None,
            ai_apply_lineage: None,
            save_manifest_ref: None,
            generated_artifact_lineage_ref: None,
        }
    }
}

/// Errors returned when producer metadata cannot form a valid journal record.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MutationProducerEmissionError {
    /// The producer registry omitted one required class.
    MissingProducerClass {
        /// Required class that was absent.
        producer_class: MutationProducerClass,
    },
    /// The producer registry declared the same class more than once.
    DuplicateProducerClass {
        /// Duplicated class.
        producer_class: MutationProducerClass,
    },
    /// The emitted record kind is not registered as durable workspace state.
    RecordRegistry(RecordRegistryError),
    /// The emitter was given no target refs.
    MissingTargetRefs {
        /// Producer class being emitted.
        producer_class: MutationProducerClass,
    },
    /// The emitter was given no metadata-only diff identity.
    MissingDiffIdentityRef {
        /// Producer class being emitted.
        producer_class: MutationProducerClass,
    },
    /// The producer requires a preview ref.
    MissingPreviewRef {
        /// Producer class being emitted.
        producer_class: MutationProducerClass,
    },
    /// The producer supplied a preview ref with the wrong kind.
    WrongPreviewKind {
        /// Producer class being emitted.
        producer_class: MutationProducerClass,
        /// Required preview kind.
        expected: PreviewKind,
        /// Supplied preview kind.
        actual: Option<PreviewKind>,
    },
    /// The producer requires an approval ref.
    MissingApprovalRef {
        /// Producer class being emitted.
        producer_class: MutationProducerClass,
    },
    /// AI apply requires an evidence-packet ref.
    MissingAiEvidencePacketRef {
        /// Producer class being emitted.
        producer_class: MutationProducerClass,
    },
    /// AI apply requires a route-class token.
    MissingAiRouteClass {
        /// Producer class being emitted.
        producer_class: MutationProducerClass,
    },
    /// AI apply requires a spend-record ref.
    MissingAiSpendRecordRef {
        /// Producer class being emitted.
        producer_class: MutationProducerClass,
    },
    /// A non-AI producer supplied AI apply lineage.
    UnexpectedAiApplyLineage {
        /// Producer class being emitted.
        producer_class: MutationProducerClass,
    },
    /// The producer requires a generated-artifact lineage ref.
    MissingGeneratedArtifactLineageRef {
        /// Producer class being emitted.
        producer_class: MutationProducerClass,
    },
}

impl fmt::Display for MutationProducerEmissionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::MissingProducerClass { producer_class } => {
                write!(
                    f,
                    "producer class {} is not registered",
                    producer_class.as_str()
                )
            }
            Self::DuplicateProducerClass { producer_class } => {
                write!(
                    f,
                    "producer class {} is registered twice",
                    producer_class.as_str()
                )
            }
            Self::RecordRegistry(err) => write!(f, "{err}"),
            Self::MissingTargetRefs { producer_class } => {
                write!(
                    f,
                    "{} producer has no mutation targets",
                    producer_class.as_str()
                )
            }
            Self::MissingDiffIdentityRef { producer_class } => write!(
                f,
                "{} producer has no metadata-only diff identity ref",
                producer_class.as_str()
            ),
            Self::MissingPreviewRef { producer_class } => {
                write!(f, "{} producer has no preview ref", producer_class.as_str())
            }
            Self::WrongPreviewKind {
                producer_class,
                expected,
                actual,
            } => write!(
                f,
                "{} producer preview kind mismatch: expected {}, got {:?}",
                producer_class.as_str(),
                expected.as_str(),
                actual
            ),
            Self::MissingApprovalRef { producer_class } => {
                write!(
                    f,
                    "{} producer has no approval ref",
                    producer_class.as_str()
                )
            }
            Self::MissingAiEvidencePacketRef { producer_class } => write!(
                f,
                "{} producer has no AI evidence packet ref",
                producer_class.as_str()
            ),
            Self::MissingAiRouteClass { producer_class } => write!(
                f,
                "{} producer has no AI route class",
                producer_class.as_str()
            ),
            Self::MissingAiSpendRecordRef { producer_class } => write!(
                f,
                "{} producer has no AI spend record ref",
                producer_class.as_str()
            ),
            Self::UnexpectedAiApplyLineage { producer_class } => write!(
                f,
                "{} producer cannot carry AI apply lineage",
                producer_class.as_str()
            ),
            Self::MissingGeneratedArtifactLineageRef { producer_class } => write!(
                f,
                "{} producer has no generated-artifact lineage ref",
                producer_class.as_str()
            ),
        }
    }
}

impl std::error::Error for MutationProducerEmissionError {}

impl From<RecordRegistryError> for MutationProducerEmissionError {
    fn from(value: RecordRegistryError) -> Self {
        Self::RecordRegistry(value)
    }
}

/// Returns the binding for `producer_class`.
pub fn producer_binding(
    producer_class: MutationProducerClass,
) -> Option<&'static MutationProducerBinding> {
    MUTATION_PRODUCER_REGISTRY
        .iter()
        .find(|binding| binding.producer_class == producer_class)
}

/// Validates that every required producer class emits a registered record kind.
pub fn validate_producer_registry() -> Result<(), MutationProducerEmissionError> {
    let mut seen = Vec::new();
    for binding in &MUTATION_PRODUCER_REGISTRY {
        if seen.contains(&binding.producer_class) {
            return Err(MutationProducerEmissionError::DuplicateProducerClass {
                producer_class: binding.producer_class,
            });
        }
        seen.push(binding.producer_class);
        validate_typed(
            binding.emitted_record_kind,
            RecordClassId::DurableWorkspaceState,
        )?;
    }

    for producer_class in REQUIRED_MUTATION_PRODUCER_CLASSES {
        if !seen.contains(&producer_class) {
            return Err(MutationProducerEmissionError::MissingProducerClass { producer_class });
        }
    }
    Ok(())
}

/// Emits a refactor-engine mutation-journal entry.
pub fn emit_refactor_record(
    input: MutationProducerInput,
) -> Result<MutationJournalEntryRecord, MutationProducerEmissionError> {
    emit_producer_record(MutationProducerClass::Refactor, input)
}

/// Emits a formatter or format-on-save mutation-journal entry.
pub fn emit_formatter_record(
    input: MutationProducerInput,
) -> Result<MutationJournalEntryRecord, MutationProducerEmissionError> {
    emit_producer_record(MutationProducerClass::Formatter, input)
}

/// Emits an AI-apply mutation-journal entry with evidence and approval lineage.
pub fn emit_ai_apply_record(
    input: MutationProducerInput,
) -> Result<MutationJournalEntryRecord, MutationProducerEmissionError> {
    emit_producer_record(MutationProducerClass::AiApply, input)
}

/// Emits a build-output observation mutation-journal entry.
pub fn emit_build_output_record(
    input: MutationProducerInput,
) -> Result<MutationJournalEntryRecord, MutationProducerEmissionError> {
    emit_producer_record(MutationProducerClass::BuildOutput, input)
}

/// Emits a lockfile observation mutation-journal entry.
pub fn emit_lockfile_record(
    input: MutationProducerInput,
) -> Result<MutationJournalEntryRecord, MutationProducerEmissionError> {
    emit_producer_record(MutationProducerClass::Lockfile, input)
}

/// Emits a preview-regeneration mutation-journal entry.
pub fn emit_preview_record(
    input: MutationProducerInput,
) -> Result<MutationJournalEntryRecord, MutationProducerEmissionError> {
    emit_producer_record(MutationProducerClass::Preview, input)
}

/// Emits one producer-owned mutation-journal entry.
pub fn emit_producer_record(
    producer_class: MutationProducerClass,
    input: MutationProducerInput,
) -> Result<MutationJournalEntryRecord, MutationProducerEmissionError> {
    let binding = producer_binding(producer_class)
        .ok_or(MutationProducerEmissionError::MissingProducerClass { producer_class })?;
    validate_typed(
        binding.emitted_record_kind,
        RecordClassId::DurableWorkspaceState,
    )?;
    validate_input(binding, &input)?;

    let mut summary = SideEffectSummary::new(input.side_effect_summary);
    summary.bytes_written = input.bytes_written;
    summary.files_touched = input.files_touched;

    let mut entry = MutationJournalEntryRecord::new(
        input.mutation_id,
        input.command_id,
        binding.actor_class,
        binding.source_class,
        input.actor_ref,
        input.scope_ref,
        input.target_refs,
        input.started_at,
        input.committed_at,
        binding.undo_class.to_owned(),
        binding.reversal_class,
        binding.redaction_class,
        binding.durable_vs_disposable,
        summary,
        input.checkpoint_refs,
    )
    .with_diff_identity_ref(input.diff_identity_ref);

    if let Some(group_id) = input.group_id {
        entry = entry.with_group_id(group_id);
    }
    if let Some(preview_ref) = input.preview_ref {
        entry = entry.with_preview_ref(preview_ref);
    }
    if let Some(approval_ref) = input.approval_ref {
        entry = entry.with_approval_ref(approval_ref);
    }
    if let Some(ai_apply_lineage) = input.ai_apply_lineage {
        entry = entry.with_ai_apply_lineage(ai_apply_lineage);
    }
    if let Some(save_manifest_ref) = input.save_manifest_ref {
        entry = entry.with_save_manifest_ref(save_manifest_ref);
    }
    if let Some(generated_artifact_lineage_ref) = input.generated_artifact_lineage_ref {
        entry = entry.with_generated_artifact_lineage_ref(generated_artifact_lineage_ref);
    }

    Ok(entry)
}

fn validate_input(
    binding: &MutationProducerBinding,
    input: &MutationProducerInput,
) -> Result<(), MutationProducerEmissionError> {
    let producer_class = binding.producer_class;
    if input.target_refs.is_empty() {
        return Err(MutationProducerEmissionError::MissingTargetRefs { producer_class });
    }
    if input.diff_identity_ref.trim().is_empty() {
        return Err(MutationProducerEmissionError::MissingDiffIdentityRef { producer_class });
    }
    if let Some(expected) = binding.required_preview_kind {
        let actual = input
            .preview_ref
            .as_ref()
            .ok_or(MutationProducerEmissionError::MissingPreviewRef { producer_class })?
            .preview_kind;
        if actual != Some(expected) {
            return Err(MutationProducerEmissionError::WrongPreviewKind {
                producer_class,
                expected,
                actual,
            });
        }
    }
    if binding.requires_approval_ref && input.approval_ref.is_none() {
        return Err(MutationProducerEmissionError::MissingApprovalRef { producer_class });
    }
    match (producer_class, &input.ai_apply_lineage) {
        (MutationProducerClass::AiApply, Some(lineage)) => {
            if lineage.ai_evidence_packet_ref.trim().is_empty() {
                return Err(MutationProducerEmissionError::MissingAiEvidencePacketRef {
                    producer_class,
                });
            }
            if lineage.route_class.trim().is_empty() {
                return Err(MutationProducerEmissionError::MissingAiRouteClass { producer_class });
            }
            if lineage.spend_record_ref.trim().is_empty() {
                return Err(MutationProducerEmissionError::MissingAiSpendRecordRef {
                    producer_class,
                });
            }
        }
        (MutationProducerClass::AiApply, None) => {
            return Err(MutationProducerEmissionError::MissingAiEvidencePacketRef {
                producer_class,
            });
        }
        (_, Some(_)) => {
            return Err(MutationProducerEmissionError::UnexpectedAiApplyLineage { producer_class });
        }
        (_, None) => {}
    }
    if binding.requires_generated_artifact_lineage_ref
        && input
            .generated_artifact_lineage_ref
            .as_ref()
            .map_or(true, |value| value.trim().is_empty())
    {
        return Err(
            MutationProducerEmissionError::MissingGeneratedArtifactLineageRef { producer_class },
        );
    }
    Ok(())
}
