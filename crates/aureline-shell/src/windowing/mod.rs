//! Platform windowing adapters for the native desktop shell.
//!
//! This module is reserved for window creation and event-loop integration
//! helpers that sit below [`crate::bootstrap`] and above the OS bindings.

pub(crate) mod display_safety;
pub(crate) mod winit_softbuffer;
pub(crate) mod winit_window;
