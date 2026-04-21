//! Piece-tree buffer prototype with grouped undo/redo.
//!
//! This is a prototype validating the contract frozen in
//! `docs/adr/0003-buffer-undo-large-file.md` early enough that later
//! editor, save, refactor, AI apply, and mutation-journal work can
//! instrument against concrete hook names and one undo-class
//! taxonomy rather than a moving target.
//!
//! It is not a production buffer engine. Known holes (balanced piece
//! index, large-file backing store, save pipeline, recovery journal,
//! coordinate translation, external-change handling, decode recovery)
//! are called out in `prototypes/buffer/README.md`.

pub mod buffer;
pub mod class;
pub mod hooks;
