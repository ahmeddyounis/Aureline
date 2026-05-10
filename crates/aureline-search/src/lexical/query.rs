//! Lexical query, ranking, and grouped result projection.
//!
//! The query is a single string. The ranking model is intentionally tiny:
//!
//! - `match_kind` records the strongest reason the row matched
//!   (`exact_basename` < `prefix_basename` < `substring_basename` <
//!   `substring_path`).
//! - Inside one match-kind bucket, rows are ordered by case-insensitive
//!   path so the same query produces the same row order across runs (the
//!   shell tests rely on this determinism).
//!
//! Each row carries a [`SourceClass`] so the shell can render the lane that
//! produced it; rows never claim a higher-confidence lane than they earned.

use serde::{Deserialize, Serialize};

use super::index::{LexicalIndexState, ReadinessClass};
use super::source::SourceClass;

/// Maximum rows surfaced per group. Keeps the shell render cheap and the
/// fixture proofs deterministic.
pub const MAX_RESULTS_PER_GROUP: usize = 12;

/// One lexical query against the index.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LexicalQuery {
    pub query: String,
}

impl LexicalQuery {
    /// Build a new query, trimming surrounding whitespace.
    pub fn new(query: impl Into<String>) -> Self {
        Self {
            query: query.into(),
        }
    }

    /// Lower-case, trimmed query string used for matching.
    pub fn normalized(&self) -> String {
        self.query.trim().to_ascii_lowercase()
    }

    /// True when the query has no non-whitespace characters.
    pub fn is_empty(&self) -> bool {
        self.normalized().is_empty()
    }
}

/// Why a row matched. Strongest reason wins per row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MatchKind {
    ExactBasename,
    PrefixBasename,
    SubstringBasename,
    SubstringPath,
}

impl MatchKind {
    /// Stable token used in records, fixtures, and shell snapshots.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ExactBasename => "exact_basename",
            Self::PrefixBasename => "prefix_basename",
            Self::SubstringBasename => "substring_basename",
            Self::SubstringPath => "substring_path",
        }
    }

    /// Lower scores rank earlier inside a group.
    const fn rank(self) -> u8 {
        match self {
            Self::ExactBasename => 0,
            Self::PrefixBasename => 1,
            Self::SubstringBasename => 2,
            Self::SubstringPath => 3,
        }
    }

    /// Source class this match kind belongs to.
    pub const fn source_class(self) -> SourceClass {
        match self {
            Self::ExactBasename | Self::PrefixBasename | Self::SubstringBasename => {
                SourceClass::LexicalFilename
            }
            Self::SubstringPath => SourceClass::LexicalPath,
        }
    }
}

/// One row in a lexical search result group.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ResultRow {
    pub relative_path: String,
    pub source_class: SourceClass,
    pub match_kind: MatchKind,
}

/// One grouped lexical result section, rendered with a clear lane label.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ResultGroup {
    pub source_class: SourceClass,
    pub label: String,
    pub items: Vec<ResultRow>,
}

/// Materialized lexical result set with scope/readiness metadata.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LexicalSearchResults {
    pub query: String,
    pub readiness: ReadinessClass,
    pub partial_truth_causes: Vec<String>,
    pub groups: Vec<ResultGroup>,
    pub total_rows: usize,
}

/// Run a lexical query against an index snapshot.
///
/// Empty queries return an empty group set so the shell can render the
/// scope/readiness chip without surfacing every workspace file as a "match".
pub fn run_query(index: &LexicalIndexState, query: &LexicalQuery) -> LexicalSearchResults {
    let normalized = query.normalized();
    let causes: Vec<String> = index
        .partial_truth_causes()
        .iter()
        .map(|c| c.as_str().to_string())
        .collect();

    if normalized.is_empty() || matches!(index.readiness(), ReadinessClass::Unavailable) {
        return LexicalSearchResults {
            query: query.query.clone(),
            readiness: index.readiness(),
            partial_truth_causes: causes,
            groups: Vec::new(),
            total_rows: 0,
        };
    }

    let mut filename_rows: Vec<ResultRow> = Vec::new();
    let mut path_rows: Vec<ResultRow> = Vec::new();

    for path in index.files() {
        let lower_path = path.to_ascii_lowercase();
        let basename = path
            .rsplit_once('/')
            .map(|(_, name)| name)
            .unwrap_or(path.as_str());
        let lower_basename = basename.to_ascii_lowercase();

        let basename_match_kind = if lower_basename == normalized {
            Some(MatchKind::ExactBasename)
        } else if lower_basename.starts_with(&normalized) {
            Some(MatchKind::PrefixBasename)
        } else if lower_basename.contains(&normalized) {
            Some(MatchKind::SubstringBasename)
        } else {
            None
        };

        if let Some(kind) = basename_match_kind {
            filename_rows.push(ResultRow {
                relative_path: path.clone(),
                source_class: SourceClass::LexicalFilename,
                match_kind: kind,
            });
            continue;
        }

        if lower_path.contains(&normalized) {
            path_rows.push(ResultRow {
                relative_path: path.clone(),
                source_class: SourceClass::LexicalPath,
                match_kind: MatchKind::SubstringPath,
            });
        }
    }

    sort_rows(&mut filename_rows);
    sort_rows(&mut path_rows);
    filename_rows.truncate(MAX_RESULTS_PER_GROUP);
    path_rows.truncate(MAX_RESULTS_PER_GROUP);

    let mut groups: Vec<ResultGroup> = Vec::new();
    let mut total_rows = 0;
    if !filename_rows.is_empty() {
        total_rows += filename_rows.len();
        groups.push(ResultGroup {
            source_class: SourceClass::LexicalFilename,
            label: SourceClass::LexicalFilename.group_label().to_string(),
            items: filename_rows,
        });
    }
    if !path_rows.is_empty() {
        total_rows += path_rows.len();
        groups.push(ResultGroup {
            source_class: SourceClass::LexicalPath,
            label: SourceClass::LexicalPath.group_label().to_string(),
            items: path_rows,
        });
    }

    LexicalSearchResults {
        query: query.query.clone(),
        readiness: index.readiness(),
        partial_truth_causes: causes,
        groups,
        total_rows,
    }
}

fn sort_rows(rows: &mut [ResultRow]) {
    rows.sort_by(|a, b| {
        a.match_kind
            .rank()
            .cmp(&b.match_kind.rank())
            .then_with(|| {
                a.relative_path
                    .to_ascii_lowercase()
                    .cmp(&b.relative_path.to_ascii_lowercase())
            })
    });
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexical::index::LexicalIndexState;
    use aureline_reactive_state::ReadinessLabel;
    use aureline_workspace::WorkspaceLifecycleState;

    fn ready_index_with(files: Vec<&str>) -> LexicalIndexState {
        LexicalIndexState::for_fixture(
            "ws-test",
            "mono:1",
            WorkspaceLifecycleState::Ready,
            ReadinessLabel::Exact,
            files.into_iter().map(String::from).collect(),
        )
    }

    #[test]
    fn empty_query_returns_no_rows_but_keeps_readiness() {
        let index = ready_index_with(vec!["src/main.rs"]);
        let results = run_query(&index, &LexicalQuery::new(""));
        assert_eq!(results.readiness, ReadinessClass::Ready);
        assert!(results.groups.is_empty());
        assert_eq!(results.total_rows, 0);
    }

    #[test]
    fn exact_basename_outranks_substring_path() {
        let index = ready_index_with(vec!["src/main.rs", "src/maintenance/mod.rs"]);
        let results = run_query(&index, &LexicalQuery::new("main.rs"));
        assert_eq!(results.groups.len(), 1);
        let group = &results.groups[0];
        assert_eq!(group.source_class, SourceClass::LexicalFilename);
        assert_eq!(group.items[0].relative_path, "src/main.rs");
        assert_eq!(group.items[0].match_kind, MatchKind::ExactBasename);
    }

    #[test]
    fn path_lane_picks_up_directory_match() {
        let index = ready_index_with(vec!["src/widgets/button.rs", "tests/widgets_smoke.rs"]);
        let results = run_query(&index, &LexicalQuery::new("widgets"));
        let path_group = results
            .groups
            .iter()
            .find(|g| g.source_class == SourceClass::LexicalPath)
            .expect("path group must exist");
        assert!(path_group
            .items
            .iter()
            .any(|row| row.relative_path == "src/widgets/button.rs"));
    }

    #[test]
    fn unavailable_index_returns_no_rows_but_carries_causes() {
        let index = LexicalIndexState::for_fixture(
            "ws-test",
            "mono:1",
            WorkspaceLifecycleState::Degraded,
            ReadinessLabel::Unavailable,
            vec!["src/main.rs".to_string()],
        );
        let results = run_query(&index, &LexicalQuery::new("main"));
        assert_eq!(results.readiness, ReadinessClass::Unavailable);
        assert!(results.groups.is_empty());
        assert!(!results.partial_truth_causes.is_empty());
    }
}
