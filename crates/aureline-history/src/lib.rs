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
pub mod mutation_journal;

mod storage;

pub use checkpoints::{
    LocalHistoryEntryRecord, LocalHistoryGroupRecord, LocalHistoryStore, RetentionScopeClass,
};
pub use mutation_journal::{
    ActorClass, ActorRef, CheckpointDurabilityClass, CheckpointKind, CheckpointRef,
    DurableVsDisposable, MutationGroupRecord, MutationJournalEntryRecord, MutationJournalStore,
    RedactionClass, ReversalClass, ScopeClass, ScopeRef, SideEffectSummary, SourceClass,
    TargetKind, TargetRef,
};
pub use storage::{HistoryError, HistoryStorageRoot, IdSource};
