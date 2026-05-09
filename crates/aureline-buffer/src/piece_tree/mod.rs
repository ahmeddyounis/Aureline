//! Piece-tree buffer core with grouped undo/redo.
//!
//! This module is the canonical text-storage engine for the editor. It follows
//! the contract frozen in `docs/adr/0003-buffer-undo-large-file.md`: piece-tree
//! storage, versioned snapshots, grouped undo/redo, and a stable undo-class
//! taxonomy.
//!
//! The current implementation is deliberately minimal — the piece list is a
//! linear `Vec` and snapshots materialise full bytes. A balanced piece index,
//! large-file backing store, durable journals, and more efficient structural
//! snapshots are tracked as follow-ups in `prototypes/buffer/README.md`.

pub mod buffer;
pub mod class;
pub mod hooks;
pub mod line_index;
