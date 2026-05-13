//! Semantic token registry and appearance primitives.
//!
//! First-party UI surfaces consume colors, geometry scales, and motion timings
//! by semantic token name rather than embedding raw literals. The token values
//! are sourced from the design-system ledgers under `artifacts/design/`.

pub mod color;
mod loaders;
pub mod registry;
pub mod state_semantics;
pub mod theme;

pub use color::ColorRgba;
pub use registry::{seeded_token_registry, TokenRegistry, TokenRegistryError};
pub use state_semantics::{
    alpha_state_semantics_registry, SemanticVisualTreatment, StateSemanticsError,
    StateSemanticsRegistry,
};
pub use theme::ThemeClass;
