//! Command palette query-session state and grouped result projections.
//!
//! The command palette is a governed shell surface: it projects from the
//! canonical command registry, keybinding resolver output, workspace file
//! index, and lexical-search truth instead of inventing palette-local lookup
//! tables.

pub mod discoverability;
pub mod preview;
pub mod query_session;
pub mod results_view;

pub use discoverability::{
    materialize_alpha_palette_query, materialize_alpha_palette_support_export,
    materialize_command_deep_link_review, materialize_invocation_session_for_review,
    AlphaFileCandidate, AlphaPaletteActionFooter, AlphaPaletteDiscoverabilitySnapshot,
    AlphaPaletteFooterAction, AlphaPalettePreviewPane, AlphaPaletteProviderSummary,
    AlphaPaletteQueryInputs, AlphaPaletteResultRow, AlphaPaletteRowKind, AlphaPaletteSupportExport,
    AlphaRecentActionCandidate, AlphaSymbolCandidate, CommandDeepLinkReviewRecord,
    ALPHA_DISCOVERABILITY_LANE_CAP,
};
pub use query_session::{
    CommandPaletteCommit, CommandPaletteState, PaletteItemKey, PaletteProviderClass,
    PaletteProviderStateClass, PaletteRankingSourceClass, QuickOpenCommandRow, QuickOpenLexicalRow,
    QuickOpenQuerySession, QuickOpenRecentTarget, QuickOpenSnapshot, QuickOpenSnapshotRow,
    QuickOpenSnapshotSource, QuickOpenSourceClass, QuickOpenSourceState,
    WorkspaceSearchSurfaceCard, WorkspaceSearchSurfaceCardItem, WorkspaceSearchSurfaceCardRow,
    WorkspaceSearchSurfaceLineageHint, WorkspaceSearchSurfaceState,
};
