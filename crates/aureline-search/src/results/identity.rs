//! Result identity, ranking-reason vocabulary, and row-level partiality.
//!
//! See the module docs in `super` for the contract these types own. This file
//! defines the closed vocabularies and the deterministic builders the lexical
//! query path uses to attach an identity to every row it materializes.

use serde::{Deserialize, Serialize};

use crate::lexical::index::ReadinessClass;
use crate::lexical::query::MatchKind;
use crate::lexical::source::SourceClass;

/// Closed vocabulary for *why* a row appeared / ranked where it did.
///
/// The taxonomy maps 1:1 onto the lexical match-kind buckets plus a small set
/// of cross-cutting reasons (generated-artifact deprioritization, partial
/// coverage caveats). Surfaces MUST surface these tokens directly; the
/// chrome must not re-derive a different reason vocabulary.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RankingReasonClass {
    /// Row matched because the workspace-relative basename equalled the query.
    ExactBasenameMatch,
    /// Row matched because the basename starts with the query (case-insensitive).
    PrefixBasenameMatch,
    /// Row matched because the basename contains the query (case-insensitive)
    /// but did not match exactly or as a prefix.
    SubstringBasenameMatch,
    /// Row matched only because the workspace-relative path contains the
    /// query — the basename did not match.
    SubstringPathMatch,
    /// Row carries a generated-artifact lineage hint, so surfaces should
    /// route edits to the canonical source rather than treating this row as
    /// the primary edit target. Always paired with one of the match-kind
    /// reasons above.
    GeneratedArtifactDeprioritized,
    /// Row was surfaced while the upstream provider was still warming /
    /// partial. The row is real, but the surrounding result set is not yet
    /// authoritative. Always paired with one of the match-kind reasons above.
    PartialCoverageCaveat,
}

impl RankingReasonClass {
    /// Stable token used in records, fixtures, and snapshots.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ExactBasenameMatch => "exact_basename_match",
            Self::PrefixBasenameMatch => "prefix_basename_match",
            Self::SubstringBasenameMatch => "substring_basename_match",
            Self::SubstringPathMatch => "substring_path_match",
            Self::GeneratedArtifactDeprioritized => "generated_artifact_deprioritized",
            Self::PartialCoverageCaveat => "partial_coverage_caveat",
        }
    }

    /// Map a [`MatchKind`] to its primary ranking reason.
    pub const fn from_match_kind(kind: MatchKind) -> Self {
        match kind {
            MatchKind::ExactBasename => Self::ExactBasenameMatch,
            MatchKind::PrefixBasename => Self::PrefixBasenameMatch,
            MatchKind::SubstringBasename => Self::SubstringBasenameMatch,
            MatchKind::SubstringPath => Self::SubstringPathMatch,
        }
    }
}

/// Row-level partiality vocabulary.
///
/// The class travels on every row so a warming / partial / stale row keeps
/// its caveat after the result set is sorted, paginated, or projected through
/// the quick-open or search-shell surfaces. The vocabulary is a strict subset
/// of [`ReadinessClass`] — `Unavailable` is included so support exports can
/// describe a row that was visible at capture time but came from a
/// now-unavailable provider.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ResultPartialityClass {
    /// Row came from a `ready` provider; no caveat is required.
    Authoritative,
    /// Row came from a provider that is still warming up. Surfaces MUST
    /// surface the warming caveat on the row, not just on the chrome chip.
    Warming,
    /// Row came from a provider with rows but incomplete coverage.
    Partial,
    /// Row came from a cached snapshot that has not been re-validated.
    Stale,
    /// Row came from a provider that is no longer answering. Used for
    /// captured snapshots and support exports; visible rows in a live
    /// session never carry this class.
    Unavailable,
}

impl ResultPartialityClass {
    /// Stable token used in records, fixtures, and snapshots.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Authoritative => "authoritative",
            Self::Warming => "warming",
            Self::Partial => "partial",
            Self::Stale => "stale",
            Self::Unavailable => "unavailable",
        }
    }

    /// Short label suitable for a row chip / badge ("Warming", "Partial",
    /// etc.). The chrome MUST NOT collapse `warming` and `partial` into one
    /// label; the contract requires both tokens to be visible to the user.
    pub const fn row_badge(self) -> &'static str {
        match self {
            Self::Authoritative => "Authoritative",
            Self::Warming => "Warming",
            Self::Partial => "Partial",
            Self::Stale => "Stale",
            Self::Unavailable => "Unavailable",
        }
    }

    /// True when the row carries a partial / warming / stale caveat that the
    /// chrome MUST surface alongside the row.
    pub const fn is_partial(self) -> bool {
        !matches!(self, Self::Authoritative)
    }
}

/// Project an upstream [`ReadinessClass`] onto the row-level partiality
/// vocabulary.
pub const fn derive_partiality_class(readiness: ReadinessClass) -> ResultPartialityClass {
    match readiness {
        ReadinessClass::Ready => ResultPartialityClass::Authoritative,
        ReadinessClass::Warming => ResultPartialityClass::Warming,
        ReadinessClass::Partial => ResultPartialityClass::Partial,
        ReadinessClass::Stale => ResultPartialityClass::Stale,
        // The lexical query path never emits visible rows for `Unavailable` or
        // `OutOfScope` readiness (it returns an empty group set). The mapping
        // is provided so support exports can describe a captured row whose
        // provider has since gone unavailable.
        ReadinessClass::Unavailable | ReadinessClass::OutOfScope => {
            ResultPartialityClass::Unavailable
        }
    }
}

/// Convenience: project the partiality class for a lexical row given the
/// active index readiness. Identical to [`derive_partiality_class`] today,
/// but provided as a named entry point so quick open and the search shell
/// can converge on one call site.
pub const fn project_lexical_partiality(readiness: ReadinessClass) -> ResultPartialityClass {
    derive_partiality_class(readiness)
}

/// Stable identity for a search result row.
///
/// The [`Self::result_id`] is a deterministic URN-style string built from the
/// workspace id, the source class, and the workspace-relative path. Two rows
/// for the same path but different lanes (filename vs. path) MUST receive
/// distinct ids so a quick-open dedup pass can preserve both rows when needed.
///
/// Surfaces MUST quote `result_id` directly when persisting selection,
/// exporting a support bundle, or reopening a row from a deep link. The id
/// is opaque to the chrome — it does not parse the workspace id or path back
/// out of the URN.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ResultIdentity {
    pub result_id: String,
    pub workspace_id: String,
    pub relative_path: String,
    pub source_class: SourceClass,
    pub match_kind: MatchKind,
    pub ranking_reasons: Vec<RankingReasonClass>,
    pub partiality_class: ResultPartialityClass,
}

impl ResultIdentity {
    /// Stable token vocabulary the URN scheme uses. Bumping this constant is
    /// a breaking change to every persisted result_id and every fixture, so
    /// reviewers can spot drift in a single grep.
    pub const SCHEME: &'static str = "wsearch";

    /// True when the row should be rendered with a partiality / warming /
    /// stale caveat directly on the row.
    pub fn must_show_row_caveat(&self) -> bool {
        self.partiality_class.is_partial()
    }

    /// Stable token list for the row's ranking reasons.
    pub fn ranking_reason_tokens(&self) -> Vec<&'static str> {
        self.ranking_reasons.iter().map(|r| r.as_str()).collect()
    }
}

/// Build the ranking-reason list for a lexical row.
///
/// The list is ordered: the match-kind primary reason comes first, followed
/// by the generated-artifact reason (if applicable) and the partial-coverage
/// caveat (if the active readiness is not `Ready`). The ordering is part of
/// the contract — fixtures rely on it.
pub fn derive_lexical_ranking_reasons(
    match_kind: MatchKind,
    has_generated_artifact_hint: bool,
    readiness: ReadinessClass,
) -> Vec<RankingReasonClass> {
    let mut reasons = Vec::with_capacity(3);
    reasons.push(RankingReasonClass::from_match_kind(match_kind));
    if has_generated_artifact_hint {
        reasons.push(RankingReasonClass::GeneratedArtifactDeprioritized);
    }
    if !matches!(readiness, ReadinessClass::Ready) {
        reasons.push(RankingReasonClass::PartialCoverageCaveat);
    }
    reasons
}

/// Build a [`ResultIdentity`] for a lexical row.
///
/// The id is a URN of the form
/// `wsearch:{workspace_id}:{source_class_token}:{relative_path}` with the
/// workspace id and relative path normalized so callers do not have to
/// reason about case or stray whitespace. Two rows that disagree on
/// workspace, source class, or path receive distinct ids; two materializations
/// of the same row in two ranking passes receive identical ids.
pub fn build_lexical_identity(
    workspace_id: &str,
    relative_path: &str,
    source_class: SourceClass,
    match_kind: MatchKind,
    has_generated_artifact_hint: bool,
    readiness: ReadinessClass,
) -> ResultIdentity {
    let normalized_workspace = workspace_id.trim();
    let normalized_path = relative_path.trim();
    let result_id = format!(
        "{}:{}:{}:{}",
        ResultIdentity::SCHEME,
        normalized_workspace,
        source_class.as_str(),
        normalized_path,
    );
    ResultIdentity {
        result_id,
        workspace_id: normalized_workspace.to_string(),
        relative_path: normalized_path.to_string(),
        source_class,
        match_kind,
        ranking_reasons: derive_lexical_ranking_reasons(
            match_kind,
            has_generated_artifact_hint,
            readiness,
        ),
        partiality_class: derive_partiality_class(readiness),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ready_row_has_authoritative_partiality_and_no_caveat_reason() {
        let identity = build_lexical_identity(
            "ws-test",
            "src/main.rs",
            SourceClass::LexicalFilename,
            MatchKind::ExactBasename,
            false,
            ReadinessClass::Ready,
        );
        assert_eq!(
            identity.partiality_class,
            ResultPartialityClass::Authoritative
        );
        assert_eq!(
            identity.ranking_reasons,
            vec![RankingReasonClass::ExactBasenameMatch]
        );
        assert!(!identity.must_show_row_caveat());
        assert_eq!(
            identity.result_id,
            "wsearch:ws-test:lexical_filename:src/main.rs"
        );
    }

    #[test]
    fn warming_row_carries_partial_caveat_reason_after_match_kind() {
        let identity = build_lexical_identity(
            "ws-test",
            "src/main.rs",
            SourceClass::LexicalFilename,
            MatchKind::PrefixBasename,
            false,
            ReadinessClass::Warming,
        );
        assert_eq!(identity.partiality_class, ResultPartialityClass::Warming);
        assert_eq!(
            identity.ranking_reasons,
            vec![
                RankingReasonClass::PrefixBasenameMatch,
                RankingReasonClass::PartialCoverageCaveat,
            ]
        );
        assert!(identity.must_show_row_caveat());
    }

    #[test]
    fn generated_artifact_appends_deprioritization_reason() {
        let identity = build_lexical_identity(
            "ws-test",
            "Cargo.lock",
            SourceClass::LexicalFilename,
            MatchKind::SubstringBasename,
            true,
            ReadinessClass::Ready,
        );
        assert_eq!(
            identity.ranking_reasons,
            vec![
                RankingReasonClass::SubstringBasenameMatch,
                RankingReasonClass::GeneratedArtifactDeprioritized,
            ]
        );
    }

    #[test]
    fn generated_artifact_on_partial_provider_carries_both_caveats() {
        let identity = build_lexical_identity(
            "ws-test",
            "Cargo.lock",
            SourceClass::LexicalFilename,
            MatchKind::SubstringBasename,
            true,
            ReadinessClass::Partial,
        );
        assert_eq!(
            identity.ranking_reasons,
            vec![
                RankingReasonClass::SubstringBasenameMatch,
                RankingReasonClass::GeneratedArtifactDeprioritized,
                RankingReasonClass::PartialCoverageCaveat,
            ]
        );
        assert_eq!(identity.partiality_class, ResultPartialityClass::Partial);
    }

    #[test]
    fn filename_and_path_lanes_get_distinct_result_ids_for_same_path() {
        let filename_identity = build_lexical_identity(
            "ws-test",
            "src/widgets/button.rs",
            SourceClass::LexicalFilename,
            MatchKind::SubstringBasename,
            false,
            ReadinessClass::Ready,
        );
        let path_identity = build_lexical_identity(
            "ws-test",
            "src/widgets/button.rs",
            SourceClass::LexicalPath,
            MatchKind::SubstringPath,
            false,
            ReadinessClass::Ready,
        );
        assert_ne!(filename_identity.result_id, path_identity.result_id);
        assert!(filename_identity.result_id.contains("lexical_filename"));
        assert!(path_identity.result_id.contains("lexical_path"));
    }

    #[test]
    fn unavailable_readiness_projects_unavailable_partiality_for_export_replay() {
        let class = derive_partiality_class(ReadinessClass::Unavailable);
        assert_eq!(class, ResultPartialityClass::Unavailable);
        let class = derive_partiality_class(ReadinessClass::OutOfScope);
        assert_eq!(class, ResultPartialityClass::Unavailable);
    }

    #[test]
    fn project_lexical_partiality_matches_derive_partiality_class() {
        for readiness in [
            ReadinessClass::Ready,
            ReadinessClass::Warming,
            ReadinessClass::Partial,
            ReadinessClass::Stale,
            ReadinessClass::Unavailable,
            ReadinessClass::OutOfScope,
        ] {
            assert_eq!(
                project_lexical_partiality(readiness),
                derive_partiality_class(readiness)
            );
        }
    }

    #[test]
    fn ranking_reason_class_round_trips_through_serde() {
        for reason in [
            RankingReasonClass::ExactBasenameMatch,
            RankingReasonClass::PrefixBasenameMatch,
            RankingReasonClass::SubstringBasenameMatch,
            RankingReasonClass::SubstringPathMatch,
            RankingReasonClass::GeneratedArtifactDeprioritized,
            RankingReasonClass::PartialCoverageCaveat,
        ] {
            let token = reason.as_str();
            let json = serde_json::to_string(&reason).expect("serialize");
            assert_eq!(json, format!("\"{token}\""));
            let parsed: RankingReasonClass = serde_json::from_str(&json).expect("deserialize");
            assert_eq!(parsed, reason);
        }
    }
}
