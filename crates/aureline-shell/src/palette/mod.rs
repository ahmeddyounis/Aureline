//! Command palette query-session state and grouped result projections.
//!
//! The command palette is a governed shell surface: it projects from the
//! canonical command registry, keybinding resolver output, workspace file
//! index, and lexical-search truth instead of inventing palette-local lookup
//! tables.

pub mod diagnostics_beta;
pub mod discoverability;
pub mod preview;
pub mod query_session;
pub mod results_view;

pub use diagnostics_beta::{
    seeded_beta_command_palette_diagnostics_pack,
    seeded_beta_command_palette_diagnostics_support_export,
    seeded_beta_palette_parity_examples_artifact, validate_beta_command_palette_diagnostics_pack,
    validate_beta_command_palette_diagnostics_support_export,
    validate_beta_palette_parity_examples_artifact, BetaCommandPaletteDeepLinkReviewSummary,
    BetaCommandPaletteDiagnosticRow, BetaCommandPaletteDiagnosticsPack,
    BetaCommandPaletteDiagnosticsSummary, BetaCommandPaletteDiagnosticsSupportExport,
    BetaPaletteHistoryPolicy, BetaPaletteParityExample, BetaPaletteParityExamplesArtifact,
    BetaPaletteParitySharedTruth, BetaPaletteWarmOpenBudget, COMMAND_PALETTE_DIAGNOSTICS_PACK_ID,
    COMMAND_PALETTE_DIAGNOSTICS_RECORD_KIND, COMMAND_PALETTE_DIAGNOSTICS_SCHEMA_VERSION,
    COMMAND_PALETTE_DIAGNOSTICS_SUPPORT_EXPORT_ID,
    COMMAND_PALETTE_DIAGNOSTICS_SUPPORT_EXPORT_RECORD_KIND, PALETTE_PARITY_EXAMPLES_ARTIFACT_ID,
    PALETTE_PARITY_EXAMPLES_RECORD_KIND,
};
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
    PaletteProviderStateClass, PaletteRankingSourceClass, QuickOpenCommandRow, QuickOpenDocsRow,
    QuickOpenLexicalRow, QuickOpenQuerySession, QuickOpenRecentTarget, QuickOpenSnapshot,
    QuickOpenSnapshotRow, QuickOpenSnapshotSource, QuickOpenSourceClass, QuickOpenSourceState,
    WorkspaceSearchSurfaceCard, WorkspaceSearchSurfaceCardItem, WorkspaceSearchSurfaceCardRow,
    WorkspaceSearchSurfaceLineageHint, WorkspaceSearchSurfaceState,
};
