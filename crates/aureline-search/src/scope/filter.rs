//! Pattern-based scope filter for workspace-relative paths.
//!
//! Patterns mirror the workset-artifact include/exclude vocabulary
//! ([`aureline_workspace::PatternEntry`]). The semantics match what the
//! workset switcher exposes:
//!
//! - **Excludes always win.** A path that matches any exclude pattern is
//!   outside the active scope, even if it also matches an include.
//! - **Includes are conjunctive at the lane level, disjunctive across the
//!   list.** When at least one include pattern is present, a path must match
//!   one of the includes (after the exclude check) to be in scope.
//! - **No includes means "every non-excluded path is in scope."** Worksets
//!   that only carry excludes still narrow scope correctly.
//!
//! The glob vocabulary is intentionally narrow: `**` (any path, including
//! empty), `*` (any run of non-`/` characters within one segment), and
//! literal segments. This is enough for the M1 workset/slice surfaces; a
//! richer matcher belongs in a future scope-pattern crate, not here.

use serde::{Deserialize, Serialize};

use aureline_workspace::{PatternEntry, PatternKind};

/// Stable include / exclude classification for one pattern entry.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ScopePatternKind {
    Include,
    Exclude,
}

impl ScopePatternKind {
    /// Stable token used in records, fixtures, and snapshots.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Include => "include",
            Self::Exclude => "exclude",
        }
    }

    /// Project from the canonical workspace [`PatternKind`].
    pub const fn from_workspace(kind: PatternKind) -> Self {
        match kind {
            PatternKind::Include => Self::Include,
            PatternKind::Exclude => Self::Exclude,
        }
    }
}

/// One pattern entry consumed by the scope filter.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ScopePatternRecord {
    pub kind: ScopePatternKind,
    pub pattern: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub applies_to_root_ref: Option<String>,
}

impl ScopePatternRecord {
    /// Project from the canonical workspace [`PatternEntry`]. Surfaces MUST
    /// use this conversion rather than re-deriving the pattern shape.
    pub fn from_workspace(entry: &PatternEntry) -> Self {
        Self {
            kind: ScopePatternKind::from_workspace(entry.pattern_kind),
            pattern: entry.pattern.clone(),
            applies_to_root_ref: entry.applies_to_root_ref.clone(),
        }
    }
}

/// Outcome of partitioning a file list against an active scope.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ScopeFilterOutcome {
    /// Workspace-relative paths that survive the active scope filter. The
    /// search shell builds its index from this list; the chrome surfaces
    /// these as the visible / loaded rows.
    pub in_scope: Vec<String>,
    /// Workspace-relative paths that were dropped by the active scope. The
    /// chrome uses this list to mark cross-repo / outside-scope rows when a
    /// surface is asked to project the failed-scope explanation.
    pub out_of_scope: Vec<String>,
    /// Total file count before scope filtering (== in_scope + out_of_scope).
    /// Surfaces use this as the "what would I see at full_workspace?"
    /// disclosure feeding the scope-truth chip's `all_matching_in_workspace`
    /// counter.
    pub all_workspace_count: u64,
    /// Total file count after scope filtering (== in_scope.len()).
    pub in_scope_count: u64,
}

impl ScopeFilterOutcome {
    /// True when the active scope drops at least one workspace file.
    pub fn is_narrowed(&self) -> bool {
        self.in_scope_count < self.all_workspace_count
    }
}

/// Apply a closed include/exclude rule list to one workspace-relative path.
///
/// Returns true when the path is in scope under those rules. The function is
/// pure and deterministic: surfaces use it directly and never invent their
/// own glob semantics on top.
pub fn glob_matches_relative_path(
    relative_path: &str,
    patterns: &[ScopePatternRecord],
) -> bool {
    let mut had_include = false;
    let mut include_hit = false;
    for entry in patterns {
        match entry.kind {
            ScopePatternKind::Exclude => {
                if glob_matches(&entry.pattern, relative_path) {
                    return false;
                }
            }
            ScopePatternKind::Include => {
                had_include = true;
                if glob_matches(&entry.pattern, relative_path) {
                    include_hit = true;
                }
            }
        }
    }
    if had_include {
        include_hit
    } else {
        true
    }
}

fn glob_matches(pattern: &str, path: &str) -> bool {
    let pattern_segments: Vec<&str> = pattern.split('/').collect();
    let path_segments: Vec<&str> = path.split('/').collect();
    glob_match_segments(&pattern_segments, &path_segments)
}

fn glob_match_segments(pattern: &[&str], path: &[&str]) -> bool {
    if pattern.is_empty() {
        return path.is_empty();
    }
    if pattern[0] == "**" {
        if pattern.len() == 1 {
            return true;
        }
        for take in 0..=path.len() {
            if glob_match_segments(&pattern[1..], &path[take..]) {
                return true;
            }
        }
        return false;
    }
    if path.is_empty() {
        return false;
    }
    if !segment_matches(pattern[0], path[0]) {
        return false;
    }
    glob_match_segments(&pattern[1..], &path[1..])
}

fn segment_matches(pattern: &str, segment: &str) -> bool {
    if !pattern.contains('*') {
        return pattern == segment;
    }
    let pattern_chars: Vec<char> = pattern.chars().collect();
    let segment_chars: Vec<char> = segment.chars().collect();
    segment_glob(&pattern_chars, &segment_chars)
}

fn segment_glob(pattern: &[char], segment: &[char]) -> bool {
    if pattern.is_empty() {
        return segment.is_empty();
    }
    if pattern[0] == '*' {
        if pattern.len() == 1 {
            return true;
        }
        for take in 0..=segment.len() {
            if segment_glob(&pattern[1..], &segment[take..]) {
                return true;
            }
        }
        return false;
    }
    if segment.is_empty() {
        return false;
    }
    if pattern[0] != segment[0] {
        return false;
    }
    segment_glob(&pattern[1..], &segment[1..])
}

#[cfg(test)]
mod tests {
    use super::*;

    fn pattern(kind: ScopePatternKind, pat: &str) -> ScopePatternRecord {
        ScopePatternRecord {
            kind,
            pattern: pat.to_string(),
            applies_to_root_ref: None,
        }
    }

    #[test]
    fn no_patterns_keeps_every_path_in_scope() {
        assert!(glob_matches_relative_path("src/main.rs", &[]));
    }

    #[test]
    fn exclude_drops_matching_path() {
        let patterns = vec![pattern(ScopePatternKind::Exclude, "**/vendor/**")];
        assert!(!glob_matches_relative_path("apps/vendor/lib.js", &patterns));
        assert!(glob_matches_relative_path("apps/web/main.rs", &patterns));
    }

    #[test]
    fn include_requires_match() {
        let patterns = vec![pattern(ScopePatternKind::Include, "apps/web/**")];
        assert!(glob_matches_relative_path("apps/web/main.tsx", &patterns));
        assert!(!glob_matches_relative_path("apps/api/handler.rs", &patterns));
    }

    #[test]
    fn exclude_wins_over_include() {
        let patterns = vec![
            pattern(ScopePatternKind::Include, "apps/web/**"),
            pattern(ScopePatternKind::Exclude, "apps/web/public/vendor/**"),
        ];
        assert!(glob_matches_relative_path("apps/web/main.tsx", &patterns));
        assert!(!glob_matches_relative_path(
            "apps/web/public/vendor/jquery.js",
            &patterns
        ));
    }

    #[test]
    fn star_matches_within_segment_only() {
        let patterns = vec![pattern(ScopePatternKind::Include, "*.md")];
        assert!(glob_matches_relative_path("README.md", &patterns));
        assert!(!glob_matches_relative_path("docs/notes.md", &patterns));
    }

    #[test]
    fn double_star_in_middle_matches_zero_segments() {
        let patterns = vec![pattern(ScopePatternKind::Include, "src/**/lib.rs")];
        assert!(glob_matches_relative_path("src/lib.rs", &patterns));
        assert!(glob_matches_relative_path("src/foo/lib.rs", &patterns));
        assert!(glob_matches_relative_path("src/a/b/lib.rs", &patterns));
        assert!(!glob_matches_relative_path("src/lib_extra.rs", &patterns));
    }
}
