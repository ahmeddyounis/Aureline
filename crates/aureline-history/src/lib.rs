//! Local-history checkpoints and unified mutation-journal persistence.
//!
//! This crate provides the prototype persistence backend for two linked truth
//! sources:
//!
//! - Local-history checkpoints (a timeline of attributable snapshots).
//! - The unified mutation journal (one vocabulary for reversible mutations).
//!
//! The writer emits schema-shaped JSON records under a caller-provided storage
//! root. Shell and tool surfaces can then inspect local history and journal
//! lineage without parsing unstructured logs.

#![doc(html_root_url = "https://docs.rs/aureline-history/0.0.0")]

pub mod checkpoints;
pub mod local_history;
pub mod mutation_journal;

mod storage;

pub use checkpoints::{
    LocalHistoryEntryRecord, LocalHistoryGroupRecord, LocalHistoryStore, RestoreOfEntryRef,
    RetentionScopeClass,
};
pub use local_history::{
    ActorLineageClass, ActorLineageRow, GitMutationLineageInput, HistoryArtifactExportSafety,
    HistoryExportMode, LocalHistoryAlphaPacket, LocalHistoryAlphaValidationError,
    LocalHistoryConsumerSurface, RestoreCheckpointAlpha, ReviewApplyLineageInput,
};
pub use mutation_journal::{
    producers::{
        emit_ai_apply_record, emit_build_output_record, emit_formatter_record,
        emit_lockfile_record, emit_preview_record, emit_producer_record, emit_refactor_record,
        producer_binding, validate_producer_registry, MutationProducerBinding,
        MutationProducerClass, MutationProducerEmissionError, MutationProducerInput,
        MUTATION_PRODUCER_REGISTRY, REQUIRED_MUTATION_PRODUCER_CLASSES,
    },
    ActorClass, ActorRef, ApprovalRef, CheckpointDurabilityClass, CheckpointKind, CheckpointRef,
    DurableVsDisposable, MutationGroupRecord, MutationJournalEntryRecord, MutationJournalStore,
    PreviewKind, PreviewRef, RedactionClass, ReversalClass, ScopeClass, ScopeRef,
    SideEffectSummary, SourceClass, TargetKind, TargetRef, MUTATION_GROUP_RECORD_KIND,
    MUTATION_JOURNAL_ENTRY_RECORD_KIND,
};
pub use storage::{HistoryError, HistoryStorageRoot, IdSource};

/// Stable content-addressed object id (`obj:blake3:<hex>`) for `bytes`.
///
/// The shared shape lets non-storage callers (preview/apply/revert lifecycle,
/// checkpoint plan inspectors, diff projections) compute the same body digest
/// the [`LocalHistoryStore::write_body_object`] writer would mint without
/// persisting a blob first.
pub fn body_object_id(bytes: &[u8]) -> String {
    let digest = blake3::hash(bytes).to_hex().to_string();
    format!("obj:blake3:{digest}")
}
