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

use aureline_workspace::{detect_lineage, LineageHintRecord};

use super::index::{LexicalIndexState, ReadinessClass};
use super::source::SourceClass;
use crate::results::{build_lexical_identity, ResultIdentity};

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
    /// Generated-artifact lineage hint, when the row's relative path matches
    /// a rule in the workspace's generated-artifact catalog. Surfaces use
    /// this to label generated rows distinctly from canonical sources and
    /// to point users back at the source-canonical artifact when known.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub generated_artifact_hint: Option<LineageHintRecord>,
    /// Stable result identity, ranking reasons, and row-level partiality
    /// class. The identity travels on every row so quick open, the search
    /// shell, support exports, and CLI replay can quote the same `result_id`,
    /// the same ranking-reason vocabulary, and the same partiality caveat
    /// without re-deriving them from the rendered chrome.
    pub identity: ResultIdentity,
}

impl ResultRow {
    /// True when the row carries a generated-artifact lineage hint.
    pub fn has_generated_artifact_hint(&self) -> bool {
        self.generated_artifact_hint.is_some()
    }
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
    let workspace_id = index.workspace_id();
    let readiness = index.readiness();

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
            let lineage = detect_lineage(path);
            let identity = build_lexical_identity(
                workspace_id,
                path,
                SourceClass::LexicalFilename,
                kind,
                lineage.is_some(),
                readiness,
            );
            filename_rows.push(ResultRow {
                relative_path: path.clone(),
                source_class: SourceClass::LexicalFilename,
                match_kind: kind,
                generated_artifact_hint: lineage,
                identity,
            });
            continue;
        }

        if lower_path.contains(&normalized) {
            let lineage = detect_lineage(path);
            let identity = build_lexical_identity(
                workspace_id,
                path,
                SourceClass::LexicalPath,
                MatchKind::SubstringPath,
                lineage.is_some(),
                readiness,
            );
            path_rows.push(ResultRow {
                relative_path: path.clone(),
                source_class: SourceClass::LexicalPath,
                match_kind: MatchKind::SubstringPath,
                generated_artifact_hint: lineage,
                identity,
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
    fn generated_lockfile_row_carries_lineage_hint() {
        let index = ready_index_with(vec!["Cargo.lock", "Cargo.toml", "src/main.rs"]);
        let results = run_query(&index, &LexicalQuery::new("cargo"));
        let lockfile_row = results
            .groups
            .iter()
            .flat_map(|g| g.items.iter())
            .find(|row| row.relative_path == "Cargo.lock")
            .expect("Cargo.lock must surface");
        let hint = lockfile_row
            .generated_artifact_hint
            .as_ref()
            .expect("Cargo.lock must carry a lineage hint");
        assert_eq!(
            hint.generated_class,
            aureline_workspace::GeneratedArtifactClass::Lockfile
        );
        assert_eq!(
            hint.source_canonical_relative_path.as_deref(),
            Some("Cargo.toml")
        );
        // The canonical sibling itself must NOT carry a hint — the detector
        // never relabels a hand-authored source as generated.
        let toml_row = results
            .groups
            .iter()
            .flat_map(|g| g.items.iter())
            .find(|row| row.relative_path == "Cargo.toml")
            .expect("Cargo.toml must surface");
        assert!(toml_row.generated_artifact_hint.is_none());
    }

    #[test]
    fn ordinary_source_row_has_no_lineage_hint() {
        let index = ready_index_with(vec!["src/main.rs", "src/lib.rs"]);
        let results = run_query(&index, &LexicalQuery::new("main"));
        let row = results.groups[0]
            .items
            .iter()
            .find(|row| row.relative_path == "src/main.rs")
            .expect("src/main.rs must surface");
        assert!(row.generated_artifact_hint.is_none());
    }

    #[test]
    fn run_query_attaches_stable_identity_with_match_kind_reason() {
        let index = ready_index_with(vec!["src/main.rs"]);
        let results = run_query(&index, &LexicalQuery::new("main.rs"));
        let row = &results.groups[0].items[0];
        assert_eq!(
            row.identity.result_id,
            "wsearch:ws-test:lexical_filename:src/main.rs"
        );
        assert_eq!(
            row.identity.ranking_reasons,
            vec![crate::results::RankingReasonClass::ExactBasenameMatch]
        );
        assert_eq!(
            row.identity.partiality_class,
            crate::results::ResultPartialityClass::Authoritative
        );
        assert!(!row.identity.must_show_row_caveat());
    }

    #[test]
    fn run_query_carries_partial_caveat_on_warming_index() {
        let index = LexicalIndexState::for_fixture(
            "ws-test",
            "mono:1",
            WorkspaceLifecycleState::PartiallyReady,
            ReadinessLabel::Partial,
            vec!["src/main.rs".to_string()],
        );
        let results = run_query(&index, &LexicalQuery::new("main"));
        let row = &results.groups[0].items[0];
        assert!(row.identity.ranking_reasons.contains(
            &crate::results::RankingReasonClass::PartialCoverageCaveat
        ));
        assert_eq!(
            row.identity.partiality_class,
            crate::results::ResultPartialityClass::Partial
        );
        assert!(row.identity.must_show_row_caveat());
    }

    #[test]
    fn run_query_marks_generated_lockfile_as_deprioritized_in_ranking_reasons() {
        let index = ready_index_with(vec!["Cargo.lock", "Cargo.toml"]);
        let results = run_query(&index, &LexicalQuery::new("cargo"));
        let lockfile_row = results
            .groups
            .iter()
            .flat_map(|g| g.items.iter())
            .find(|r| r.relative_path == "Cargo.lock")
            .expect("Cargo.lock must surface");
        assert!(lockfile_row.identity.ranking_reasons.contains(
            &crate::results::RankingReasonClass::GeneratedArtifactDeprioritized
        ));
        let toml_row = results
            .groups
            .iter()
            .flat_map(|g| g.items.iter())
            .find(|r| r.relative_path == "Cargo.toml")
            .expect("Cargo.toml must surface");
        assert!(!toml_row.identity.ranking_reasons.contains(
            &crate::results::RankingReasonClass::GeneratedArtifactDeprioritized
        ));
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
