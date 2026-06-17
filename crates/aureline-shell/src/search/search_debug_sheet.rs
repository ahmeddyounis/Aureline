//! Query-debug sheets for the search surfaces.
//!
//! This module is the desktop and replay/debug consumer of the
//! [`RankingExplainabilityPacket`]. It projects a compact, inspectable
//! per-surface query-debug sheet that reuses the packet's `Why this result?`
//! explain sheets and withheld/policy-hidden omitted-candidate rows verbatim, so
//! the palette, sidebars, docs results, graph-backed results, and saved-query
//! reopen panes — and a CLI/headless inspect dump — read one explanation object
//! instead of reconstructing ranking truth from rendered row text.
//!
//! The projection mints no new explanation: it carries the same durable result
//! ids, explainer-state and fact-label tokens, prose reason lines, omission
//! reasons, and scope counts, and it never lifts literal query text out of the
//! metadata-only packet.

use aureline_search::{
    ExplainSurfaceClass, ExplainabilityConsumerClass, RankingExplainabilityPacket,
};
use serde::{Deserialize, Serialize};

/// Stable record-kind tag for [`SearchDebugSheetSet`].
pub const SEARCH_DEBUG_SHEET_SET_RECORD_KIND: &str = "search_debug_sheet_set";

/// Schema version for [`SearchDebugSheetSet`].
pub const SEARCH_DEBUG_SHEET_SET_SCHEMA_VERSION: u32 = 1;

/// One visible `Why this result?` row in a query-debug sheet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SearchDebugExplainRow {
    /// Durable, surface-independent result identity.
    pub result_id: String,
    /// Display title preserved verbatim.
    pub display_title: String,
    /// Explainer-state token (promoted, suppressed, tied, partial_index).
    pub explain_state_token: String,
    /// Fact-label token (exact, context_promoted, semantic, partial_index, …).
    pub fact_label_token: String,
    /// User-visible headline.
    pub headline: String,
    /// Prose reason lines, reused verbatim.
    pub reason_lines: Vec<String>,
    /// Optional partiality caveat.
    pub caveat_note: Option<String>,
}

/// One `Why withheld?` row in a query-debug sheet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SearchDebugOmissionRow {
    /// Explainer-state token (withheld_latency, policy_hidden, partial_index).
    pub explain_state_token: String,
    /// Fact-label token for the omission.
    pub fact_label_token: String,
    /// Number of candidates this row accounts for.
    pub omitted_count: u64,
    /// User-visible omission reason.
    pub omission_reason: String,
    /// Source stratum the omitted candidates came from.
    pub source_stratum_token: String,
    /// Optional recovery hint.
    pub recovery_hint: Option<String>,
}

/// One surface's query-debug sheet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SearchDebugSheet {
    /// Surface token reused from the packet.
    pub surface: String,
    /// Human-readable surface label.
    pub surface_label: String,
    /// Durable query-session id the surface answered from.
    pub query_session_id: String,
    /// Deterministic query hash; never the raw query text.
    pub query_hash: String,
    /// Visible explain rows, in result order.
    pub explain_rows: Vec<SearchDebugExplainRow>,
    /// Withheld / policy-hidden / partial-index omitted rows.
    pub omitted_rows: Vec<SearchDebugOmissionRow>,
    /// Visible row count carried from the packet's scope counters.
    pub visible_count: u64,
    /// Rows hidden by policy on this surface.
    pub hidden_by_policy_count: u64,
    /// Rows withheld by a latency budget on this surface.
    pub withheld_by_latency_count: u64,
}

/// Desktop / replay-debug projection of the explainability packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SearchDebugSheetSet {
    /// Stable record-kind discriminator.
    pub record_kind: String,
    /// Integer schema version.
    pub schema_version: u32,
    /// Packet id ingested verbatim.
    pub ingested_packet_id: String,
    /// Export consent posture carried from the packet.
    pub export_consent_token: String,
    /// True when literal query text is present (gated by consent).
    pub literal_query_text_included: bool,
    /// Per-surface query-debug sheets, in packet order.
    pub sheets: Vec<SearchDebugSheet>,
}

impl SearchDebugSheetSet {
    /// Returns the query-debug sheet for one surface, if present.
    pub fn sheet_for(&self, surface: ExplainSurfaceClass) -> Option<&SearchDebugSheet> {
        self.sheets
            .iter()
            .find(|sheet| sheet.surface == surface.as_str())
    }

    /// True when the projection reuses the packet for every claimed surface
    /// without lifting literal query text out of a metadata-only packet.
    pub fn reuses_explainability_packet(&self) -> bool {
        self.sheets.len() == ExplainSurfaceClass::ALL.len()
            && self.sheets.iter().all(|sheet| {
                !sheet.query_session_id.is_empty()
                    && !sheet.query_hash.is_empty()
                    && !sheet.explain_rows.is_empty()
            })
            && !self.literal_query_text_included
    }
}

/// Projects the per-surface query-debug sheets from the explainability packet.
///
/// The projection reuses the durable result ids, explainer-state and fact-label
/// tokens, prose reason lines, and omission reasons verbatim; it mints no new
/// explanation, so the desktop and replay/debug surfaces share one truth with
/// the support-export consumer.
pub fn project_search_debug_sheets(packet: &RankingExplainabilityPacket) -> SearchDebugSheetSet {
    let sheets = packet
        .surfaces
        .iter()
        .map(|surface| SearchDebugSheet {
            surface: surface.surface.as_str().to_string(),
            surface_label: surface.surface_label.clone(),
            query_session_id: surface.query_session_id_ref.clone(),
            query_hash: surface.query_hash.clone(),
            explain_rows: surface
                .explain_sheets
                .iter()
                .map(|sheet| SearchDebugExplainRow {
                    result_id: sheet.result_id.clone(),
                    display_title: sheet.display_title.clone(),
                    explain_state_token: sheet.explain_state.as_str().to_string(),
                    fact_label_token: sheet.ranking_reason.fact_label.as_str().to_string(),
                    headline: sheet.headline.clone(),
                    reason_lines: sheet.reason_lines.clone(),
                    caveat_note: sheet.caveat_note.clone(),
                })
                .collect(),
            omitted_rows: surface
                .omitted_candidates
                .iter()
                .map(|omitted| SearchDebugOmissionRow {
                    explain_state_token: omitted.explain_state.as_str().to_string(),
                    fact_label_token: omitted.fact_label.as_str().to_string(),
                    omitted_count: omitted.omitted_count,
                    omission_reason: omitted.omission_reason.clone(),
                    source_stratum_token: omitted.source_stratum.as_str().to_string(),
                    recovery_hint: omitted.recovery_hint.clone(),
                })
                .collect(),
            visible_count: surface.scope_counters.visible_rows,
            hidden_by_policy_count: surface.scope_counters.hidden_by_policy_rows,
            withheld_by_latency_count: surface.scope_counters.omitted_by_latency_budget_rows,
        })
        .collect();

    SearchDebugSheetSet {
        record_kind: SEARCH_DEBUG_SHEET_SET_RECORD_KIND.to_string(),
        schema_version: SEARCH_DEBUG_SHEET_SET_SCHEMA_VERSION,
        ingested_packet_id: packet.packet_id.clone(),
        export_consent_token: packet.export_consent.as_str().to_string(),
        literal_query_text_included: packet.literal_query_text_included,
        sheets,
    }
}

/// True when the packet names the product UI and replay/debug first consumers
/// that reuse the explanation object without lifting literal query text.
pub fn query_debug_is_reuse_consumer(packet: &RankingExplainabilityPacket) -> bool {
    let reuses = |consumer: ExplainabilityConsumerClass| {
        packet.consumer_projections.iter().any(|projection| {
            projection.consumer == consumer
                && projection.preserves_explain_sheets
                && projection.preserves_omitted_candidates
                && projection.preserves_counts_and_hashes
                && !projection.includes_literal_query_text
        })
    };
    reuses(ExplainabilityConsumerClass::ProductUi)
        && reuses(ExplainabilityConsumerClass::ReplayDebug)
}

#[cfg(test)]
mod tests {
    use super::*;
    use aureline_search::{
        seeded_partial_index_stale_ranking_explainability_packet,
        seeded_ranking_explainability_packet, ExportConsentClass,
    };

    #[test]
    fn projects_a_debug_sheet_for_every_surface() {
        let packet = seeded_ranking_explainability_packet();
        let set = project_search_debug_sheets(&packet);
        assert_eq!(set.record_kind, SEARCH_DEBUG_SHEET_SET_RECORD_KIND);
        assert_eq!(set.ingested_packet_id, packet.packet_id);
        assert!(set.reuses_explainability_packet());
        for surface in ExplainSurfaceClass::ALL {
            assert!(set.sheet_for(surface).is_some(), "missing {surface:?}");
        }
        assert!(query_debug_is_reuse_consumer(&packet));
    }

    #[test]
    fn surfaces_withheld_and_policy_hidden_omissions() {
        let packet = seeded_ranking_explainability_packet();
        let set = project_search_debug_sheets(&packet);
        let graph = set
            .sheet_for(ExplainSurfaceClass::GraphBackedResults)
            .unwrap();
        assert!(graph.hidden_by_policy_count > 0);
        assert!(graph
            .omitted_rows
            .iter()
            .any(|row| row.explain_state_token == "policy_hidden"));

        let palette = set.sheet_for(ExplainSurfaceClass::Palette).unwrap();
        assert!(palette.withheld_by_latency_count > 0);
        assert!(palette
            .omitted_rows
            .iter()
            .any(|row| row.explain_state_token == "withheld_latency"));
    }

    #[test]
    fn reuses_durable_result_ids_and_reason_lines() {
        let packet = seeded_ranking_explainability_packet();
        let set = project_search_debug_sheets(&packet);
        for surface in &packet.surfaces {
            let sheet = set.sheet_for(surface.surface).unwrap();
            for (row, source) in sheet.explain_rows.iter().zip(surface.explain_sheets.iter()) {
                assert_eq!(row.result_id, source.result_id);
                assert_eq!(row.reason_lines, source.reason_lines);
            }
        }
        assert_eq!(
            set.export_consent_token,
            ExportConsentClass::MetadataOnly.as_str()
        );
        assert!(!set.literal_query_text_included);
    }

    #[test]
    fn degraded_packet_projects_strictly_more_omissions() {
        let canonical = project_search_debug_sheets(&seeded_ranking_explainability_packet());
        let degraded = project_search_debug_sheets(
            &seeded_partial_index_stale_ranking_explainability_packet(),
        );
        let canonical_palette = canonical.sheet_for(ExplainSurfaceClass::Palette).unwrap();
        let degraded_palette = degraded.sheet_for(ExplainSurfaceClass::Palette).unwrap();
        assert!(
            degraded_palette.withheld_by_latency_count
                > canonical_palette.withheld_by_latency_count
        );
        assert!(degraded_palette.omitted_rows.len() > canonical_palette.omitted_rows.len());
    }
}
