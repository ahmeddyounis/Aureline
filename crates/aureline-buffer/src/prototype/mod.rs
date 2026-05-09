//! Compatibility alias for the former `prototype` module path.
//!
//! The canonical buffer engine lives under [`crate::piece_tree`]. This module
//! re-exports the same implementation so existing imports continue to compile
//! while the repository transitions away from prototype naming.

pub use crate::piece_tree::{buffer, class, hooks, line_index};
