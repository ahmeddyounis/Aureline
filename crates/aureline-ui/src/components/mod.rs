//! Shared component-state and focus-return contracts.
//!
//! This module is the cross-surface home for the closed component-state
//! vocabulary and the shared focus-return helper used by protected shell
//! surfaces.

pub mod focus_return;
pub mod state_registry;

pub use focus_return::FocusReturnStack;
pub use state_registry::{
    ComponentChromeStyle, ComponentStateClass, ComponentStateRegistry, ComponentStates,
    ComponentSurfaceTone, FocusRingStyle,
};
