//! Ranking-reason cards for search rows.
//!
//! Quick open and symbol search render compact rows, but the row chrome is not
//! the support contract. This module projects canonical row identity and
//! ranking-reason tokens into a small card that can back `Why this result?`
//! UI, CLI/debug output, and support-bundle metadata.

use serde::{Deserialize, Serialize};

use aureline_search::{
    PlannedResultSet, PlannedSearchResult, PlannerPathReadiness, SearchQuerySession,
};

use crate::palette::{QuickOpenSnapshot, QuickOpenSnapshotRow};

/// Schema version for [`RankingReasonCard`].
pub const RANKING_REASON_CARD_SCHEMA_VERSION: u32 = 1;

/// Schema version for [`RankingReasonSupportExport`].
pub const RANKING_REASON_SUPPORT_EXPORT_SCHEMA_VERSION: u32 = 1;

const MAX_DOMINANT_SIGNALS: usize = 3;

/// One dominant signal shown in a ranking-reason card.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RankingReasonSignal {
    /// Stable reason token from the search/planner vocabulary.
    pub signal_class: String,
    /// Short display label for the signal.
    pub label: String,
    /// Support-safe explanation of what the signal means.
    pub summary: String,
    /// Non-numeric rank bucket, never a private score or weight.
    pub weight_bucket: String,
}

/// Structured `Why this result?` card for a quick-open or symbol-search row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RankingReasonCard {
    /// Stable record-kind tag for card exports.
    pub record_kind: String,
    /// Integer schema version for this card.
    pub schema_version: u32,
    /// Stable row result ID quoted from the owning result row.
    pub result_id: String,
    /// Search surface that rendered the row.
    pub surface: String,
    /// Query-session ref when the row came from the planner contract.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub query_session_id_ref: Option<String>,
    /// Result-set ref when the row came from the planner contract.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub result_set_id_ref: Option<String>,
    /// Planner-pass ref when the row came from the planner contract.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub planner_pass_id_ref: Option<String>,
    /// Row kind token from the owning surface.
    pub row_kind_token: String,
    /// Primary row title cached for card display.
    pub title: String,
    /// Support-safe target ref: command ID, relative path, symbol ref, or canonical ID.
    pub target_ref: String,
    /// Source or data-path token that answered the row.
    pub source_class_token: String,
    /// Readiness token observed for the row.
    pub readiness_state: String,
    /// Result truth token observed for the row.
    pub result_truth_class: String,
    /// Row-level partiality token.
    pub partiality_class: String,
    /// Full ordered ranking-reason token list.
    pub ranking_reason_classes: Vec<String>,
    /// Top ranking reasons rendered by the compact card.
    pub dominant_signals: Vec<RankingReasonSignal>,
    /// Partial-truth causes attached to this row or its answering lane.
    #[serde(default)]
    pub partial_truth_causes: Vec<String>,
    /// Omitted, fallback, or unavailable-source notes expressed as safe tokens.
    #[serde(default)]
    pub omitted_source_notes: Vec<String>,
    /// Redaction posture for support-bundle inclusion.
    pub export_visibility_class: String,
}

impl RankingReasonCard {
    /// Stable record-kind tag carried in serialized cards.
    pub const RECORD_KIND: &'static str = "search_ranking_reason_card";

    /// Builds a card for one row in a quick-open snapshot.
    pub fn from_quick_open_row(snapshot: &QuickOpenSnapshot, row: &QuickOpenSnapshotRow) -> Self {
        let mut partial_truth_causes = row.partial_truth_causes.clone();
        if row.source_class_token.starts_with("lexical_") {
            for cause in &snapshot.lexical_partial_truth_causes {
                if !partial_truth_causes.contains(cause) {
                    partial_truth_causes.push(cause.clone());
                }
            }
        }
        let mut omitted_source_notes = Vec::new();
        if row.source_class_token.starts_with("lexical_") && !partial_truth_causes.is_empty() {
            omitted_source_notes.push("lexical_lane_partial".to_string());
        }
        if row
            .citation_anchor_availability_token
            .as_deref()
            .is_some_and(|token| token != "exact_anchor_available")
        {
            omitted_source_notes.push("citation_anchor_unavailable".to_string());
        }
        let target_ref = row
            .command_id
            .as_deref()
            .or(row.open_anchor_ref.as_deref())
            .or(row.canonical_ref.as_deref())
            .or(row.relative_path.as_deref())
            .unwrap_or(row.result_id.as_str())
            .to_string();
        let ranking_reason_classes = row.ranking_reason_classes.clone();
        Self {
            record_kind: Self::RECORD_KIND.to_string(),
            schema_version: RANKING_REASON_CARD_SCHEMA_VERSION,
            result_id: row.result_id.clone(),
            surface: "quick_open".to_string(),
            query_session_id_ref: None,
            result_set_id_ref: None,
            planner_pass_id_ref: None,
            row_kind_token: row.row_kind_token.clone(),
            title: row.display_label.clone(),
            target_ref,
            source_class_token: row.source_class_token.clone(),
            readiness_state: row.source_state_token.clone(),
            result_truth_class: row.result_truth_class.clone(),
            partiality_class: row.partiality_class.clone(),
            dominant_signals: dominant_signals(&ranking_reason_classes),
            ranking_reason_classes,
            partial_truth_causes,
            omitted_source_notes,
            export_visibility_class: "metadata_safe_default".to_string(),
        }
    }

    /// Builds a card for one planner-fused search row.
    pub fn from_planned_result(
        query_session: &SearchQuerySession,
        result_set: &PlannedResultSet,
        row: &PlannedSearchResult,
    ) -> Self {
        let ranking_reason_classes = row
            .ranking_reason_tokens()
            .into_iter()
            .map(str::to_string)
            .collect::<Vec<_>>();
        let mut omitted_source_notes = Vec::new();
        if let Some(reason) = row.explanation.fallback_reason {
            omitted_source_notes.push(format!("fallback_reason:{}", reason.as_str()));
        }
        if row.explanation.degraded_by_missing_graph_or_language {
            omitted_source_notes.push("missing_graph_or_language_path".to_string());
        }
        let target_ref = row
            .symbol_ref
            .as_deref()
            .or(row.relative_path.as_deref())
            .unwrap_or(row.canonical_id.as_str())
            .to_string();
        Self {
            record_kind: Self::RECORD_KIND.to_string(),
            schema_version: RANKING_REASON_CARD_SCHEMA_VERSION,
            result_id: row.result_id.clone(),
            surface: result_set.surface.as_str().to_string(),
            query_session_id_ref: Some(query_session.query_session_id.clone()),
            result_set_id_ref: Some(result_set.result_set_id.clone()),
            planner_pass_id_ref: Some(result_set.planner_pass_id_ref.clone()),
            row_kind_token: row.target_kind.as_str().to_string(),
            title: row.title.clone(),
            target_ref,
            source_class_token: row.answered_by.as_str().to_string(),
            readiness_state: row.readiness_state.as_str().to_string(),
            result_truth_class: row.truth_class.as_str().to_string(),
            partiality_class: partiality_for_planner_readiness(row.readiness_state).to_string(),
            dominant_signals: dominant_signals(&ranking_reason_classes),
            ranking_reason_classes,
            partial_truth_causes: row.partial_truth_causes.clone(),
            omitted_source_notes,
            export_visibility_class: "metadata_safe_default".to_string(),
        }
    }
}

/// Support-export wrapper for ranking-reason cards.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RankingReasonSupportExport {
    /// Stable record-kind tag for support exports.
    pub record_kind: String,
    /// Integer schema version for this support artifact.
    pub schema_version: u32,
    /// Stable artifact ID used by support-bundle manifests.
    pub support_artifact_id: String,
    /// Monotonic or fixture timestamp for export parity.
    pub generated_at: String,
    /// Redaction posture applied to this artifact.
    pub redaction_class: String,
    /// Query sessions referenced by the cards.
    pub query_session_refs: Vec<String>,
    /// Result IDs referenced by the cards.
    pub result_ids: Vec<String>,
    /// Cards included in the support artifact.
    pub cards: Vec<RankingReasonCard>,
}

impl RankingReasonSupportExport {
    /// Stable record-kind tag carried in serialized support exports.
    pub const RECORD_KIND: &'static str = "search_ranking_reason_support_export";

    /// Builds a support-export artifact from structured cards.
    pub fn from_cards(
        support_artifact_id: impl Into<String>,
        generated_at: impl Into<String>,
        redaction_class: impl Into<String>,
        cards: Vec<RankingReasonCard>,
    ) -> Self {
        let mut query_session_refs = Vec::new();
        let mut result_ids = Vec::new();
        for card in &cards {
            if let Some(query_session_id) = &card.query_session_id_ref {
                if !query_session_refs.contains(query_session_id) {
                    query_session_refs.push(query_session_id.clone());
                }
            }
            if !result_ids.contains(&card.result_id) {
                result_ids.push(card.result_id.clone());
            }
        }
        Self {
            record_kind: Self::RECORD_KIND.to_string(),
            schema_version: RANKING_REASON_SUPPORT_EXPORT_SCHEMA_VERSION,
            support_artifact_id: support_artifact_id.into(),
            generated_at: generated_at.into(),
            redaction_class: redaction_class.into(),
            query_session_refs,
            result_ids,
            cards,
        }
    }
}

/// Builds ranking-reason cards for every row in a quick-open snapshot.
pub fn ranking_reason_cards_for_quick_open_snapshot(
    snapshot: &QuickOpenSnapshot,
) -> Vec<RankingReasonCard> {
    snapshot
        .rows
        .iter()
        .map(|row| RankingReasonCard::from_quick_open_row(snapshot, row))
        .collect()
}

/// Builds ranking-reason cards for every row in a planner result set.
pub fn ranking_reason_cards_for_planned_result_set(
    query_session: &SearchQuerySession,
    result_set: &PlannedResultSet,
) -> Vec<RankingReasonCard> {
    result_set
        .rows
        .iter()
        .map(|row| RankingReasonCard::from_planned_result(query_session, result_set, row))
        .collect()
}

fn dominant_signals(tokens: &[String]) -> Vec<RankingReasonSignal> {
    tokens
        .iter()
        .take(MAX_DOMINANT_SIGNALS)
        .map(|token| signal_for_token(token))
        .collect()
}

fn signal_for_token(token: &str) -> RankingReasonSignal {
    let (label, summary, weight_bucket) = match token {
        "exact_name_match" | "lexical_exact_match" => (
            "Exact name",
            "The target name matched the query exactly.",
            "primary_signal",
        ),
        "exact_path_match" => (
            "Exact path",
            "The workspace-relative path matched the query exactly.",
            "primary_signal",
        ),
        "lexical_prefix_match" => (
            "Prefix match",
            "The target name started with the query.",
            "primary_signal",
        ),
        "lexical_substring_match" | "lexical_fuzzy_match" => (
            "Lexical match",
            "The target name or path contained the query.",
            "primary_signal",
        ),
        "lexical_path_match" => (
            "Path match",
            "The path matched when the basename did not.",
            "primary_signal",
        ),
        "recent_file_bias" | "recent_edit_bias" => (
            "Recent",
            "Recent navigation or edit history influenced the row order.",
            "recency_bias",
        ),
        "hot_set_bias" => (
            "Hot set",
            "The row came from the foreground hot set while broader indexing continued.",
            "responsiveness_bias",
        ),
        "palette_command_canonical" => (
            "Command registry",
            "The row came from the canonical command registry.",
            "primary_signal",
        ),
        "structural_symbol_match" => (
            "Structural symbol",
            "A syntax or outline provider matched the symbol.",
            "primary_signal",
        ),
        "structural_fallback" => (
            "Structural fallback",
            "Structural data answered while graph proof was unavailable.",
            "fallback_signal",
        ),
        "symbol_kind_prior" => (
            "Symbol kind",
            "The symbol kind matched the target class expected by this search.",
            "tie_breaker",
        ),
        "graph_exact_symbol" => (
            "Graph symbol",
            "The graph provided an exact symbol answer.",
            "primary_signal",
        ),
        "graph_neighbourhood_hop" => (
            "Graph neighborhood",
            "A related graph node influenced the row order.",
            "semantic_signal",
        ),
        "cached_snapshot_hit" => (
            "Cached snapshot",
            "A cached snapshot answered this row with freshness disclosure.",
            "fallback_signal",
        ),
        "generated_artifact_deprioritized" => (
            "Generated artifact",
            "Generated-artifact lineage changed the row posture.",
            "suppression_signal",
        ),
        "partial_index" => (
            "Partial index",
            "The answering source was partial or still warming.",
            "partiality_signal",
        ),
        "graph_unavailable" => (
            "Graph unavailable",
            "The graph path could not answer and a fallback was used.",
            "fallback_signal",
        ),
        "language_unavailable" => (
            "Language unavailable",
            "The language path could not answer and a fallback was used.",
            "fallback_signal",
        ),
        "docs_anchor_match" => (
            "Docs anchor",
            "The result matched a documentation or help citation anchor.",
            "primary_signal",
        ),
        "docs_symbol_linked_reference" => (
            "Symbol-linked docs",
            "The documentation row is linked to the requested symbol or command.",
            "primary_signal",
        ),
        "docs_source_precedence" => (
            "Docs precedence",
            "Project documentation precedence influenced the row order.",
            "anchor_boost",
        ),
        "citation_available" => (
            "Citation available",
            "Citation anchors are available for inspection.",
            "anchor_boost",
        ),
        "citation_missing" => (
            "Citation missing",
            "The row is useful but its exact citation anchor is missing.",
            "partiality_signal",
        ),
        "stale_example_signal" => (
            "Stale example",
            "A stale example or failing snippet signal affected this row.",
            "partiality_signal",
        ),
        _ => (
            "Ranking signal",
            "A canonical search ranking signal influenced the row.",
            "detail_signal",
        ),
    };
    RankingReasonSignal {
        signal_class: token.to_string(),
        label: label.to_string(),
        summary: summary.to_string(),
        weight_bucket: weight_bucket.to_string(),
    }
}

fn partiality_for_planner_readiness(readiness: PlannerPathReadiness) -> &'static str {
    match readiness {
        PlannerPathReadiness::Ready => "authoritative",
        PlannerPathReadiness::HotSetReady | PlannerPathReadiness::Partial => "partial",
        PlannerPathReadiness::Warming => "warming",
        PlannerPathReadiness::Stale => "stale",
        PlannerPathReadiness::Unavailable | PlannerPathReadiness::OutOfScope => "unavailable",
    }
}
