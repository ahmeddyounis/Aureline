//! Workspace search foundations.
//!
//! This crate is the canonical home for the workspace lexical-search shell:
//! the runtime path that backs filename- and path-search rows in the live
//! shell while M1 still has no semantic graph.
//!
//! The vocabulary here is intentionally narrow:
//!
//! - [`lexical::SourceClass`] names the lane that produced a row
//!   (filename vs. path) so downstream surfaces never imply semantic depth.
//! - [`lexical::ReadinessClass`] names whether the lane is ready, warming,
//!   partial, or unavailable — sourced from the upstream workspace lifecycle
//!   ([`aureline_workspace::WorkspaceReadinessInputs`]) and the readiness
//!   labels published by the reactive store
//!   ([`aureline_reactive_state::ReadinessLabel`]). Surfaces MUST surface the
//!   same token; they MUST NOT collapse `warming` and `partial` into a generic
//!   "loading" badge.
//! - [`lexical::ScopeClass`] mirrors the
//!   [`aureline_workspace::ScopeClass`] so the search shell projects scope
//!   chips through the same vocabulary the workset surface uses.
//!
//! Higher layers (the shell `search_shell` module) convert this vocabulary
//! into chrome and persistable diagnostics; this crate only owns the
//! identity, ranking, and partiality truth.

#![doc(html_root_url = "https://docs.rs/aureline-search/0.0.0")]

pub mod lexical;
pub mod results;
pub mod scope;

pub use lexical::{
    LexicalIndexInputs, LexicalIndexState, LexicalQuery, LexicalSearchResults, LexicalShell,
    LexicalShellSnapshot, MatchKind, ReadinessClass, ResultGroup, ResultRow, ScopeClass,
    SourceClass, MAX_RESULTS_PER_GROUP,
};

pub use results::{
    build_lexical_identity, derive_lexical_ranking_reasons, derive_partiality_class,
    project_lexical_partiality, RankingReasonClass, ResultIdentity, ResultPartialityClass,
};

pub use scope::{
    glob_matches_relative_path, ScopeFilterOutcome, ScopePatternKind, ScopePatternRecord,
    ScopePresentationState, WorkspaceSearchScope, WorkspaceSearchScopeMetadata,
};

pub use aureline_workspace::{
    GeneratedArtifactClass, LineageFreshnessClass, LineageHintRecord,
};
