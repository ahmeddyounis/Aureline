//! Shell-side badge projections shared across run-capable seed surfaces.
//!
//! The badges here are thin projections over the canonical execution-context
//! and provider/auth contracts. They give the bottom-panel terminal pane, the
//! task seed, the debug-prep seed, and the provider/auth entry point one
//! shared vocabulary for "what target am I about to act on, and which origin
//! is asking?" without any surface forking a private label set.

pub mod target_origin;
