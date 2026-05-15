//! Editor buffer core.
//!
//! Hosts the piece-tree storage, selection/multicursor model, save
//! coordination, large-file mode, and the undo/redo history.
//! Downstream editor surfaces read from and write to buffers
//! exclusively through this crate.
//!
//! The canonical engine is a piece-tree buffer with versioned snapshots and a
//! grouped undo/redo journal. The public API is stable enough for editor views
//! and bench harnesses; the internals will evolve (balanced piece index,
//! structural snapshots, durable journals) without changing the surface.

#![doc(html_root_url = "https://docs.rs/aureline-buffer/0.0.0")]

pub mod content_integrity;
pub mod piece_tree;
pub mod prototype;

pub use content_integrity::{
    project_editor_content_integrity, EditorContentIntegrityEvent,
    EditorContentIntegrityProjection, EDITOR_CONTENT_INTEGRITY_PROJECTION_RECORD_KIND,
    EDITOR_CONTENT_INTEGRITY_PROJECTION_SCHEMA_VERSION,
};
pub use piece_tree::buffer::{
    Buffer, BufferConfig, BufferError, CheckpointHandle, CommittedInfo, JournalEntry, JournalView,
    RevisionId, Snapshot, SnapshotId, Transaction, TransactionId, TransactionSpec, UndoGroupId,
    UndoOutcome,
};
pub use piece_tree::class::{CompensationPosture, UndoClass};
pub use piece_tree::hooks::HookCounters;
pub use piece_tree::line_index::{LineIndex, LineSpan};
