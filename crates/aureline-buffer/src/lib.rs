//! Editor buffer core.
//!
//! Hosts the piece-tree storage, selection/multicursor model, save
//! coordination, large-file mode, and the undo/redo history.
//! Downstream editor surfaces read from and write to buffers
//! exclusively through this crate.
//!
//! The current landing is a prototype (see [`prototype`]) that
//! validates the contract frozen in
//! `docs/adr/0003-buffer-undo-large-file.md`. It exposes a piece-tree
//! buffer, grouped undo/redo transactions over the frozen undo-class
//! taxonomy, snapshots, checkpoints, and named hook counters. It is
//! the API surface later editor layers consume; the production
//! implementation swaps internals behind the same surface.

#![doc(html_root_url = "https://docs.rs/aureline-buffer/0.0.0")]

pub mod prototype;

pub use prototype::buffer::{
    Buffer, BufferConfig, BufferError, CheckpointHandle, CommittedInfo, JournalEntry, JournalView,
    Snapshot, SnapshotId, Transaction, TransactionId, TransactionSpec, UndoGroupId, UndoOutcome,
};
pub use prototype::class::{CompensationPosture, UndoClass};
pub use prototype::hooks::HookCounters;
