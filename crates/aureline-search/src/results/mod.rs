//! Search result identity, ranking reasons, and row-level partiality labels.
//!
//! The lexical query in [`crate::lexical::query`] decides *which* rows match
//! and *how* they sort. This module decides *how those rows are explained* to
//! the user and to support exports without losing truth across surfaces:
//!
//! - [`ResultIdentity`] attaches a stable [`ResultIdentity::result_id`] to
//!   every row so the same row is reproducible from quick open, search shell,
//!   explorer reveal, support exports, and CLI replay even when ranking
//!   reorders the row list.
//! - [`RankingReasonClass`] is the closed vocabulary that records *why* a row
//!   ranked where it did. Surfaces MUST quote tokens from this enum rather
//!   than coining ad-hoc copy strings.
//! - [`ResultPartialityClass`] travels on each row so a row that came from a
//!   warming, partial, or stale provider keeps its caveat after sorting,
//!   pagination, deduplication, or projection through the quick-open and
//!   search-shell surfaces.
//!
//! The vocabulary is intentionally narrow: M1 only ships the lexical lanes,
//! so the ranking-reason and partiality vocabularies stay pinned to the
//! lexical match-kind taxonomy plus the lifecycle readiness vocabulary.
//! Future semantic / symbol / graph lanes own their own ranking-reason
//! tokens; this module never relabels a lexical row as semantic just because
//! a richer surface ships alongside.

pub mod identity;

pub use identity::{
    build_lexical_identity, derive_lexical_ranking_reasons, derive_partiality_class,
    project_lexical_partiality, RankingReasonClass, ResultIdentity, ResultPartialityClass,
};
