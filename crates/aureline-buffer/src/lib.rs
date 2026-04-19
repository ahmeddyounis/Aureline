//! Editor buffer core.
//!
//! Hosts the piece-tree storage, selection/multicursor model, save coordination,
//! large-file mode, and the undo/redo history. Downstream editor surfaces read
//! from and write to buffers exclusively through this crate.

#![doc(html_root_url = "https://docs.rs/aureline-buffer/0.0.0")]
