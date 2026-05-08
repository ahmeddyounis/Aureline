//! Command palette query-session state and grouped result projections.
//!
//! The command palette is a governed shell surface: it projects from the
//! canonical command registry, keybinding resolver output, and enablement
//! decisions instead of inventing a palette-local command truth table.

pub mod query_session;
pub mod results_view;

pub use query_session::{
    CommandPaletteCommit, CommandPaletteState, PaletteItemKey, PaletteProviderClass,
    PaletteProviderStateClass, PaletteRankingSourceClass,
};
