//! Shell consumer for canonical query-envelope state.
//!
//! Status rows in this module project [`aureline_reactive_state::QueryEnvelopeRecord`]
//! without inventing a parallel readiness vocabulary. The same record fields feed
//! live chrome, support exports, and benchmark trace references.

use serde::{Deserialize, Serialize};

use aureline_reactive_state::{QueryEnvelopeRecord, QueryEnvelopeState};

use crate::state_cards::DegradedStateToken;

/// Stable record-kind tag for [`QueryEnvelopeStatusRecord`].
pub const QUERY_ENVELOPE_STATUS_RECORD_KIND: &str = "query_envelope_status_record";

/// Stable record-kind tag for [`QueryEnvelopeSurfaceBundle`].
pub const QUERY_ENVELOPE_SURFACE_BUNDLE_RECORD_KIND: &str = "query_envelope_surface_bundle";

const QUERY_ENVELOPE_STATUS_SCHEMA_VERSION: u32 = 1;
const QUERY_ENVELOPE_SURFACE_BUNDLE_SCHEMA_VERSION: u32 = 1;

/// Status-bar row for one query-envelope frame.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct QueryEnvelopeStatusRecord {
    /// Stable record-kind tag for support and fixture exports.
    pub record_kind: String,
    /// Integer schema version for this alpha record.
    pub schema_version: u32,
    /// Stable status item identity.
    pub status_item_id: String,
    /// Consumer surface token copied from the canonical query envelope.
    pub consumer_surface: String,
    /// Query family token copied from the canonical query envelope.
    pub query_family: String,
    /// Scope class token copied from the canonical query envelope.
    pub scope_class: String,
    /// Scope identity copied from the canonical query envelope.
    pub scope_id: String,
    /// State token copied from the canonical query envelope.
    pub state_token: String,
    /// Human-readable current value.
    pub current_value_label: String,
    /// Human-readable explanation for the current query-envelope state.
    pub explanation: String,
    /// Optional degraded token for the existing shell badge family.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub degraded_token: Option<String>,
    /// Refresh reason token copied from the canonical query envelope when present.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub refresh_reason: Option<String>,
    /// Cancellation reason token copied from the canonical query envelope when present.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cancellation_reason: Option<String>,
    /// Failure reason copied from the canonical query envelope when present.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub failure_reason: Option<String>,
    /// Invalidation reason copied from the canonical query envelope when present.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub invalidation_reason: Option<String>,
    /// Command id that opens the detailed query-envelope view.
    pub primary_command_id: String,
    /// Surface ref opened by the primary command.
    pub opens_surface_ref: String,
    /// True when the status row must not be rendered as a current/full claim.
    pub current_claim_narrowed: bool,
    /// Truth source carried for support/debug joins.
    pub truth_source_ref: String,
}

/// Shell-facing bundle that references support and benchmark artifacts by id.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct QueryEnvelopeSurfaceBundle {
    /// Stable record-kind tag for support and fixture exports.
    pub record_kind: String,
    /// Integer schema version for this alpha record.
    pub schema_version: u32,
    /// Stable generation timestamp supplied by the caller.
    pub generated_at: String,
    /// Status rows projected from query-envelope records.
    pub status_rows: Vec<QueryEnvelopeStatusRecord>,
    /// Support artifact identity generated from the same query-envelope records.
    pub support_artifact_ref: String,
    /// Benchmark trace identity generated from the same query-envelope records.
    pub benchmark_trace_ref: String,
}

impl QueryEnvelopeSurfaceBundle {
    /// Materializes shell status rows from canonical query-envelope records.
    pub fn from_records(
        support_artifact_ref: impl Into<String>,
        benchmark_trace_ref: impl Into<String>,
        generated_at: impl Into<String>,
        records: &[QueryEnvelopeRecord],
    ) -> Self {
        Self {
            record_kind: QUERY_ENVELOPE_SURFACE_BUNDLE_RECORD_KIND.to_string(),
            schema_version: QUERY_ENVELOPE_SURFACE_BUNDLE_SCHEMA_VERSION,
            generated_at: generated_at.into(),
            status_rows: records
                .iter()
                .map(QueryEnvelopeStatusRecord::from_record)
                .collect(),
            support_artifact_ref: support_artifact_ref.into(),
            benchmark_trace_ref: benchmark_trace_ref.into(),
        }
    }

    /// Returns true when every status row uses the query-envelope state grammar.
    pub fn has_single_event_grammar(&self) -> bool {
        self.status_rows.iter().all(|row| {
            matches!(
                row.state_token.as_str(),
                "ready" | "warming" | "partial" | "stale" | "failed"
            )
        })
    }
}

impl QueryEnvelopeStatusRecord {
    /// Materializes a status row from one canonical query-envelope record.
    pub fn from_record(record: &QueryEnvelopeRecord) -> Self {
        let surface_ref = format!(
            "surface.query_envelope.{}.{}",
            record.consumer_surface, record.subscription_id
        );
        Self {
            record_kind: QUERY_ENVELOPE_STATUS_RECORD_KIND.to_string(),
            schema_version: QUERY_ENVELOPE_STATUS_SCHEMA_VERSION,
            status_item_id: format!(
                "status.item.query_envelope.{}.{}",
                record.consumer_surface, record.subscription_id
            ),
            consumer_surface: record.consumer_surface.clone(),
            query_family: record.query_family.clone(),
            scope_class: record.scope_class.clone(),
            scope_id: record.scope_id.clone(),
            state_token: record.state_token.clone(),
            current_value_label: format!(
                "{} query: {}",
                record.consumer_surface, record.state_label
            ),
            explanation: explanation_for_record(record),
            degraded_token: degraded_token_for_state(record.state)
                .map(|token| token.token().to_string()),
            refresh_reason: record.refresh_reason.clone(),
            cancellation_reason: record.cancellation_reason.clone(),
            failure_reason: record.failure_reason.clone(),
            invalidation_reason: record.invalidation_reason.clone(),
            primary_command_id: "cmd:query_envelope.inspect".to_string(),
            opens_surface_ref: surface_ref,
            current_claim_narrowed: record.state.narrows_current_claim(),
            truth_source_ref: record.query_envelope_id.clone(),
        }
    }
}

fn degraded_token_for_state(state: QueryEnvelopeState) -> Option<DegradedStateToken> {
    match state {
        QueryEnvelopeState::Ready => None,
        QueryEnvelopeState::Warming => Some(DegradedStateToken::Warming),
        QueryEnvelopeState::Partial => Some(DegradedStateToken::Partial),
        QueryEnvelopeState::Stale => Some(DegradedStateToken::Stale),
        QueryEnvelopeState::Failed => Some(DegradedStateToken::Offline),
    }
}

fn explanation_for_record(record: &QueryEnvelopeRecord) -> String {
    if let Some(reason) = &record.failure_reason {
        return format!("{} query failed: {reason}", record.consumer_surface);
    }
    if let Some(reason) = &record.cancellation_reason {
        return format!("{} query cancelled: {reason}", record.consumer_surface);
    }
    if let Some(reason) = &record.refresh_reason {
        return format!("{} query refreshed: {reason}", record.consumer_surface);
    }
    if let Some(reason) = &record.invalidation_reason {
        return format!("{} query invalidated: {reason}", record.consumer_surface);
    }
    format!(
        "{} query is {} with {}/{} coverage",
        record.consumer_surface, record.state_token, record.coverage_ready, record.coverage_total
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    fn record_with_reasons() -> QueryEnvelopeRecord {
        QueryEnvelopeRecord {
            record_kind: "query_envelope_alpha_record".to_string(),
            schema_version: 1,
            query_envelope_id: "query-envelope:101:1:0:resync_required".to_string(),
            subscription_id: 101,
            consumer_surface: "graph".to_string(),
            query_family: "graph.neighborhood".to_string(),
            scope_class: "workspace".to_string(),
            scope_id: "ws-alpha".to_string(),
            state: QueryEnvelopeState::Stale,
            state_token: "stale".to_string(),
            state_label: "Stale".to_string(),
            snapshot_epoch: 1,
            delta_seq: 0,
            frame_class: "resync_required".to_string(),
            freshness: "stale".to_string(),
            completeness: "partial".to_string(),
            backpressure_mode: "coalesced".to_string(),
            producer_id: "aureline.graph.query".to_string(),
            producer_version: Some("0.0.0".to_string()),
            invalidation_reason: Some("authority_epoch_rolled".to_string()),
            refresh_reason: Some("graph_epoch_advanced".to_string()),
            cancellation_reason: None,
            failure_reason: None,
            observed_at: "mono:query:0004".to_string(),
            result_count: 0,
            coverage_ready: 0,
            coverage_total: 8,
        }
    }

    #[test]
    fn status_row_quotes_reason_tokens() {
        let row = QueryEnvelopeStatusRecord::from_record(&record_with_reasons());
        assert_eq!(row.state_token, "stale");
        assert_eq!(row.degraded_token.as_deref(), Some("Stale"));
        assert_eq!(row.refresh_reason.as_deref(), Some("graph_epoch_advanced"));
        assert_eq!(
            row.invalidation_reason.as_deref(),
            Some("authority_epoch_rolled")
        );
        assert!(row.current_claim_narrowed);
        assert!(row.explanation.contains("graph_epoch_advanced"));
    }

    #[test]
    fn bundle_preserves_query_event_grammar() {
        let record = record_with_reasons();
        let bundle = QueryEnvelopeSurfaceBundle::from_records(
            "artifact:query-envelope:alpha",
            "trace:query-envelope:alpha",
            "mono:query:bundle",
            &[record],
        );
        assert!(bundle.has_single_event_grammar());
        assert_eq!(bundle.support_artifact_ref, "artifact:query-envelope:alpha");
        assert_eq!(bundle.benchmark_trace_ref, "trace:query-envelope:alpha");
        assert_eq!(
            bundle.status_rows[0].truth_source_ref,
            "query-envelope:101:1:0:resync_required"
        );
    }
}
