//! Lexical filename/path search providers.
//!
//! The lexical shell has one job: answer "which workspace files match this
//! query string" using filename and path matching only, while staying honest
//! about the index's readiness and the active scope.
//!
//! Identity model
//! --------------
//!
//! - The index is keyed by workspace-relative path (forward-slash
//!   normalized). Two providers consume the same path list:
//!   [`SourceClass::LexicalFilename`] matches against the basename only,
//!   [`SourceClass::LexicalPath`] matches against the full relative path
//!   (and falls back to no-op when the basename match already covered the row).
//! - Readiness is the union of the upstream workspace lifecycle readiness
//!   and the index's own scan completion. The shell consumes the merged
//!   [`ReadinessClass`] so the shown badge cannot drift from the lifecycle
//!   truth.
//! - Scope is the workset/sparse-slice scope class the workspace already
//!   owns; this crate does not invent a parallel scope vocabulary.

pub mod index;
pub mod query;
pub mod scope;
pub mod shell;
pub mod source;

pub use index::{LexicalIndexInputs, LexicalIndexState, ReadinessClass};
pub use query::{
    LexicalQuery, LexicalSearchResults, MatchKind, ResultGroup, ResultRow,
    MAX_RESULTS_PER_GROUP,
};
pub use scope::ScopeClass;
pub use shell::{LexicalShell, LexicalShellSnapshot};
pub use source::SourceClass;
