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
    ActorClass, ActorRef, CheckpointDurabilityClass, CheckpointKind, CheckpointRef,
    DurableVsDisposable, MutationGroupRecord, MutationJournalEntryRecord, MutationJournalStore,
    RedactionClass, ReversalClass, ScopeClass, ScopeRef, SideEffectSummary, SourceClass,
    TargetKind, TargetRef,
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
