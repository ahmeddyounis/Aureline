//! External alpha search validation packet.
//!
//! This module closes the review loop between search result IDs, ranking
//! reason cards, palette discoverability, and launch-critical keyboard routes.
//! It owns no search ranking logic; it consumes the canonical row/card/audit
//! projections and produces one compact packet for review and support export.

use std::collections::{BTreeMap, BTreeSet};

use serde::{Deserialize, Serialize};

use crate::help::keyboard_gap_audit::AlphaKeyboardGapAudit;
use crate::palette::AlphaPaletteDiscoverabilitySnapshot;

use super::ranking_reason_card::RankingReasonCard;

/// Schema version for [`SearchAlphaValidationPacket`].
pub const SEARCH_ALPHA_VALIDATION_SCHEMA_VERSION: u32 = 1;

/// Support-safe validation packet for the alpha search lane.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SearchAlphaValidationPacket {
    /// Stable record-kind tag for packet exports.
    pub record_kind: String,
    /// Integer schema version for this packet.
    pub schema_version: u32,
    /// Stable packet identity used by milestone and support artifacts.
    pub packet_id: String,
    /// Monotonic or fixture timestamp for deterministic review captures.
    pub generated_at: String,
    /// Overall validation state for the reviewed search lane.
    pub validation_state: String,
    /// Source contract and artifact refs consumed by this packet.
    pub source_contract_refs: Vec<String>,
    /// Per-surface review rows derived from ranking reason cards.
    pub surface_reviews: Vec<SearchAlphaSurfaceReview>,
    /// Keyboard-path review derived from the alpha keyboard audit.
    pub keyboard_review: SearchAlphaKeyboardReview,
    /// Discoverability review derived from palette search snapshots.
    pub discoverability_review: SearchAlphaDiscoverabilityReview,
    /// Known-limit ids that narrow or explain remaining alpha gaps.
    pub known_limit_refs: Vec<String>,
    /// Structured validation findings.
    pub findings: Vec<SearchAlphaValidationFinding>,
}

impl SearchAlphaValidationPacket {
    /// Stable record-kind tag carried in serialized packets.
    pub const RECORD_KIND: &'static str = "search_alpha_validation_packet";

    /// Returns `true` when the packet satisfies the protected alpha search
    /// acceptance floor.
    pub fn passes_acceptance(&self) -> bool {
        self.validation_state != "blocked"
            && !self
                .findings
                .iter()
                .any(|finding| finding.severity == "error")
            && self
                .surface_reviews
                .iter()
                .any(|surface| surface.surface == "quick_open")
            && self
                .surface_reviews
                .iter()
                .any(|surface| surface.surface == "symbol_search")
            && self.keyboard_review.search_keyboard_state == "covered"
            && self.discoverability_review.discoverability_state == "covered"
    }
}

/// Per-surface result-card review row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SearchAlphaSurfaceReview {
    /// Search surface under review.
    pub surface: String,
    /// Result IDs reviewed on this surface.
    pub result_id_refs: Vec<String>,
    /// Source/data-path tokens observed across reviewed rows.
    pub source_class_tokens: Vec<String>,
    /// Readiness states observed across reviewed rows.
    pub readiness_states: Vec<String>,
    /// Result-truth classes observed across reviewed rows.
    pub result_truth_classes: Vec<String>,
    /// Row-level partiality classes observed across reviewed rows.
    pub partiality_classes: Vec<String>,
    /// Ordered ranking reason classes observed across reviewed rows.
    pub ranking_reason_classes: Vec<String>,
    /// Same-surface route used to inspect `Why this result?`.
    pub same_surface_explanation_route: String,
    /// Review outcome for this surface.
    pub review_state: String,
}

/// Keyboard review row for search explanation access.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SearchAlphaKeyboardReview {
    /// Record kind consumed from the upstream keyboard audit.
    pub audit_record_kind: String,
    /// Active preset ref used for route attribution.
    pub active_preset_ref: String,
    /// Surface id that proves palette/search diagnostics are keyboard reachable.
    pub reason_detail_surface_id: String,
    /// Keyboard route copied from the upstream audit row.
    pub keyboard_route: String,
    /// Focus-return state copied from the upstream audit row.
    pub focus_return_state: String,
    /// Command ids exposed on the reason-detail route.
    pub command_ids: Vec<String>,
    /// Non-search gaps still present in the consumed keyboard audit.
    pub non_search_gap_surface_ids: Vec<String>,
    /// Search-specific keyboard coverage state.
    pub search_keyboard_state: String,
}

/// Discoverability review row for palette-backed search entry.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SearchAlphaDiscoverabilityReview {
    /// Snapshot record kinds consumed by this review.
    pub source_record_kinds: Vec<String>,
    /// Row-kind tokens surfaced by the palette discoverability lane.
    pub row_kind_tokens: Vec<String>,
    /// Command ids surfaced by command-backed palette rows.
    pub command_ids: Vec<String>,
    /// Ranking reason classes surfaced by palette rows.
    pub ranking_reason_classes: Vec<String>,
    /// Provider state classes surfaced by palette providers.
    pub provider_state_classes: Vec<String>,
    /// Discoverability coverage state.
    pub discoverability_state: String,
}

/// Validation finding emitted by the search alpha packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SearchAlphaValidationFinding {
    /// Severity token: `info`, `warning`, or `error`.
    pub severity: String,
    /// Stable finding id.
    pub finding_id: String,
    /// Short support-safe finding summary.
    pub summary: String,
    /// Artifact, result, surface, or known-limit refs for the finding.
    pub refs: Vec<String>,
}

/// Builds the combined alpha search validation packet.
pub fn materialize_search_alpha_validation_packet(
    packet_id: impl Into<String>,
    generated_at: impl Into<String>,
    ranking_cards: &[RankingReasonCard],
    keyboard_audit: &AlphaKeyboardGapAudit,
    discoverability_snapshots: &[AlphaPaletteDiscoverabilitySnapshot],
    known_limit_refs: Vec<String>,
) -> SearchAlphaValidationPacket {
    let surface_reviews = surface_reviews(ranking_cards);
    let keyboard_review = keyboard_review(keyboard_audit);
    let discoverability_review = discoverability_review(discoverability_snapshots);
    let findings = validation_findings(
        ranking_cards,
        &surface_reviews,
        &keyboard_review,
        &discoverability_review,
        &known_limit_refs,
    );
    let validation_state = if findings.iter().any(|finding| finding.severity == "error") {
        "blocked"
    } else if known_limit_refs.is_empty() {
        "accepted"
    } else {
        "accepted_with_known_limits"
    };

    SearchAlphaValidationPacket {
        record_kind: SearchAlphaValidationPacket::RECORD_KIND.to_string(),
        schema_version: SEARCH_ALPHA_VALIDATION_SCHEMA_VERSION,
        packet_id: packet_id.into(),
        generated_at: generated_at.into(),
        validation_state: validation_state.to_string(),
        source_contract_refs: source_contract_refs(),
        surface_reviews,
        keyboard_review,
        discoverability_review,
        known_limit_refs,
        findings,
    }
}

fn source_contract_refs() -> Vec<String> {
    vec![
        "docs/search/query_planner_contract_seed.md".to_string(),
        "docs/search/result_identity_and_ranking.md".to_string(),
        "docs/search/search_explainability_contract.md".to_string(),
        "docs/search/search_readiness_vocabulary.md".to_string(),
        "docs/ux/alpha_discoverability.md".to_string(),
        "docs/accessibility/m2_keyboard_gap_audit.md".to_string(),
        "artifacts/benchmarks/m2_partial_index_drill.md".to_string(),
    ]
}

fn surface_reviews(ranking_cards: &[RankingReasonCard]) -> Vec<SearchAlphaSurfaceReview> {
    let mut grouped: BTreeMap<String, Vec<&RankingReasonCard>> = BTreeMap::new();
    for card in ranking_cards {
        grouped.entry(card.surface.clone()).or_default().push(card);
    }

    grouped
        .into_iter()
        .map(|(surface, cards)| {
            let mut result_id_refs = Vec::new();
            let mut source_class_tokens = BTreeSet::new();
            let mut readiness_states = BTreeSet::new();
            let mut result_truth_classes = BTreeSet::new();
            let mut partiality_classes = BTreeSet::new();
            let mut ranking_reason_classes = Vec::new();
            let mut seen_reasons = BTreeSet::new();
            let mut complete = true;

            for card in cards {
                if card.result_id.trim().is_empty()
                    || card.target_ref.trim().is_empty()
                    || card.ranking_reason_classes.is_empty()
                    || card.dominant_signals.is_empty()
                {
                    complete = false;
                }
                result_id_refs.push(card.result_id.clone());
                source_class_tokens.insert(card.source_class_token.clone());
                readiness_states.insert(card.readiness_state.clone());
                result_truth_classes.insert(card.result_truth_class.clone());
                partiality_classes.insert(card.partiality_class.clone());
                for reason in &card.ranking_reason_classes {
                    if seen_reasons.insert(reason.clone()) {
                        ranking_reason_classes.push(reason.clone());
                    }
                }
            }

            SearchAlphaSurfaceReview {
                surface,
                result_id_refs,
                source_class_tokens: source_class_tokens.into_iter().collect(),
                readiness_states: readiness_states.into_iter().collect(),
                result_truth_classes: result_truth_classes.into_iter().collect(),
                partiality_classes: partiality_classes.into_iter().collect(),
                ranking_reason_classes,
                same_surface_explanation_route: "selected_result.why_this_result_card".to_string(),
                review_state: if complete { "covered" } else { "incomplete" }.to_string(),
            }
        })
        .collect()
}

fn keyboard_review(audit: &AlphaKeyboardGapAudit) -> SearchAlphaKeyboardReview {
    let reason_row = audit
        .rows
        .iter()
        .find(|row| row.surface_id == "palette.command_diagnostics");

    let command_ids = reason_row
        .map(|row| {
            row.command_exposures
                .iter()
                .map(|exposure| exposure.command_id.clone())
                .collect::<Vec<_>>()
        })
        .unwrap_or_default();
    let non_search_gap_surface_ids = audit
        .remaining_gaps
        .iter()
        .map(|gap| gap.surface_id.clone())
        .filter(|surface_id| !is_search_keyboard_surface(surface_id))
        .collect::<Vec<_>>();
    let has_search_gap = audit
        .remaining_gaps
        .iter()
        .any(|gap| is_search_keyboard_surface(&gap.surface_id));
    let search_keyboard_state = match reason_row {
        Some(row)
            if row.coverage_state == "covered"
                && !row.keyboard_route.trim().is_empty()
                && !row.focus_return_state.trim().is_empty()
                && !has_search_gap =>
        {
            "covered"
        }
        _ => "incomplete",
    };

    SearchAlphaKeyboardReview {
        audit_record_kind: audit.record_kind.clone(),
        active_preset_ref: audit.active_preset_ref.clone(),
        reason_detail_surface_id: "palette.command_diagnostics".to_string(),
        keyboard_route: reason_row
            .map(|row| row.keyboard_route.clone())
            .unwrap_or_default(),
        focus_return_state: reason_row
            .map(|row| row.focus_return_state.clone())
            .unwrap_or_default(),
        command_ids,
        non_search_gap_surface_ids,
        search_keyboard_state: search_keyboard_state.to_string(),
    }
}

fn discoverability_review(
    snapshots: &[AlphaPaletteDiscoverabilitySnapshot],
) -> SearchAlphaDiscoverabilityReview {
    let mut source_record_kinds = BTreeSet::new();
    let mut row_kind_tokens = Vec::new();
    let mut seen_row_kinds = BTreeSet::new();
    let mut command_ids = BTreeSet::new();
    let mut ranking_reason_classes = Vec::new();
    let mut seen_reasons = BTreeSet::new();
    let mut provider_state_classes = BTreeSet::new();

    for snapshot in snapshots {
        source_record_kinds.insert(snapshot.record_kind.clone());
        for provider in &snapshot.providers {
            provider_state_classes.insert(provider.state_class.clone());
        }
        for row in &snapshot.rows {
            let row_kind = row.row_kind.as_str().to_string();
            if seen_row_kinds.insert(row_kind.clone()) {
                row_kind_tokens.push(row_kind);
            }
            if let Some(command_id) = &row.command_id {
                command_ids.insert(command_id.clone());
            }
            for reason in &row.ranking_reason_classes {
                if seen_reasons.insert(reason.clone()) {
                    ranking_reason_classes.push(reason.clone());
                }
            }
        }
    }

    let expected_kinds = ["command", "symbol", "file"];
    let discoverability_state = if expected_kinds
        .iter()
        .all(|kind| seen_row_kinds.contains(*kind))
        && !ranking_reason_classes.is_empty()
    {
        "covered"
    } else {
        "incomplete"
    };

    SearchAlphaDiscoverabilityReview {
        source_record_kinds: source_record_kinds.into_iter().collect(),
        row_kind_tokens,
        command_ids: command_ids.into_iter().collect(),
        ranking_reason_classes,
        provider_state_classes: provider_state_classes.into_iter().collect(),
        discoverability_state: discoverability_state.to_string(),
    }
}

fn validation_findings(
    ranking_cards: &[RankingReasonCard],
    surface_reviews: &[SearchAlphaSurfaceReview],
    keyboard_review: &SearchAlphaKeyboardReview,
    discoverability_review: &SearchAlphaDiscoverabilityReview,
    known_limit_refs: &[String],
) -> Vec<SearchAlphaValidationFinding> {
    let mut findings = Vec::new();

    if ranking_cards.is_empty() {
        findings.push(error(
            "search_alpha.no_ranking_cards",
            "No ranking-reason cards were supplied for the protected search lane.",
            Vec::new(),
        ));
    }

    for required_surface in ["quick_open", "symbol_search"] {
        if !surface_reviews
            .iter()
            .any(|review| review.surface == required_surface)
        {
            findings.push(error(
                "search_alpha.required_surface_missing",
                format!("Required search surface is missing: {required_surface}"),
                vec![required_surface.to_string()],
            ));
        }
    }

    for review in surface_reviews {
        if review.review_state != "covered" {
            findings.push(error(
                "search_alpha.surface_incomplete",
                format!(
                    "Search surface has incomplete explanation coverage: {}",
                    review.surface
                ),
                vec![review.surface.clone()],
            ));
        }
    }

    if keyboard_review.search_keyboard_state != "covered" {
        findings.push(error(
            "search_alpha.keyboard_incomplete",
            "The search explanation route is not keyboard covered.",
            vec![keyboard_review.reason_detail_surface_id.clone()],
        ));
    }

    if discoverability_review.discoverability_state != "covered" {
        findings.push(error(
            "search_alpha.discoverability_incomplete",
            "Palette discoverability does not cover command, symbol, and file rows.",
            discoverability_review.row_kind_tokens.clone(),
        ));
    }

    if known_limit_refs.is_empty() {
        findings.push(SearchAlphaValidationFinding {
            severity: "warning".to_string(),
            finding_id: "search_alpha.known_limit_ref_absent".to_string(),
            summary: "No known-limit ref narrows the synthetic alpha proof.".to_string(),
            refs: Vec::new(),
        });
    } else {
        findings.push(SearchAlphaValidationFinding {
            severity: "info".to_string(),
            finding_id: "search_alpha.known_limits_attached".to_string(),
            summary: "Remaining alpha search proof limits are recorded in the known-limits packet."
                .to_string(),
            refs: known_limit_refs.to_vec(),
        });
    }

    findings
}

fn error(
    finding_id: impl Into<String>,
    summary: impl Into<String>,
    refs: Vec<String>,
) -> SearchAlphaValidationFinding {
    SearchAlphaValidationFinding {
        severity: "error".to_string(),
        finding_id: finding_id.into(),
        summary: summary.into(),
        refs,
    }
}

fn is_search_keyboard_surface(surface_id: &str) -> bool {
    surface_id.contains("palette") || surface_id.contains("search") || surface_id.contains("quick")
}
