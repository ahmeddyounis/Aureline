//! Shell chrome surfaces and canonical identity state.
//!
//! This module owns the shared state that title/context bar chrome, the native
//! window title, workspace status items, and support/export packets must
//! project from. Consumers should treat the state records as the source of
//! truth rather than recomputing identity tuples per surface.

pub mod title_context_bar;
