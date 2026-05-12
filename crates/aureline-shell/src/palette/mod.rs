//! Command palette query-session state and grouped result projections.
//!
//! The command palette is a governed shell surface: it projects from the
//! canonical command registry, keybinding resolver output, workspace file
//! index, and lexical-search truth instead of inventing palette-local lookup
//! tables.

pub mod preview;
pub mod query_session;
pub mod results_view;

pub use query_session::{
    CommandPaletteCommit, CommandPaletteState, PaletteItemKey, PaletteProviderClass,
    PaletteProviderStateClass, PaletteRankingSourceClass, QuickOpenCommandRow, QuickOpenLexicalRow,
    QuickOpenQuerySession, QuickOpenRecentTarget, QuickOpenSnapshot, QuickOpenSnapshotRow,
    QuickOpenSnapshotSource, QuickOpenSourceClass, QuickOpenSourceState,
    WorkspaceSearchSurfaceCard, WorkspaceSearchSurfaceCardItem, WorkspaceSearchSurfaceCardRow,
    WorkspaceSearchSurfaceLineageHint, WorkspaceSearchSurfaceState,
};
