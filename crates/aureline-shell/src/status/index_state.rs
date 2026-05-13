//! Shell consumer for canonical indexed-state truth.
//!
//! The status item, result-pane notice, and support artifact in this module
//! are all projected from [`aureline_search::IndexedLaneState`]. The shell
//! keeps the owner crate's `warming`, `partial`, `cached`, `stale`, and
//! `paused` tokens visible instead of translating them into surface-local
//! labels.

use serde::{Deserialize, Serialize};

use aureline_search::{IndexedLaneState, IndexedStateClass, IndexedStateSupportArtifact};

use crate::state_cards::DegradedStateToken;

/// Stable record-kind tag for [`IndexStateStatusRecord`].
pub const INDEX_STATE_STATUS_RECORD_KIND: &str = "index_state_status_record";

/// Stable record-kind tag for [`IndexStateResultPaneRecord`].
pub const INDEX_STATE_RESULT_PANE_RECORD_KIND: &str = "index_state_result_pane_record";

/// Stable record-kind tag for [`IndexStateSurfaceBundle`].
pub const INDEX_STATE_SURFACE_BUNDLE_RECORD_KIND: &str = "index_state_surface_bundle";

const INDEX_STATE_STATUS_SCHEMA_VERSION: u32 = 1;
const INDEX_STATE_RESULT_PANE_SCHEMA_VERSION: u32 = 1;
const INDEX_STATE_SURFACE_BUNDLE_SCHEMA_VERSION: u32 = 1;

/// Status-bar row for one indexed lane.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct IndexStateStatusRecord {
    /// Stable record-kind tag for support and fixture exports.
    pub record_kind: String,
    /// Integer schema version for this alpha record.
    pub schema_version: u32,
    /// Stable status item identity.
    pub status_item_id: String,
    /// Workspace identity copied from the canonical lane state.
    pub workspace_id: String,
    /// Lane token copied from the canonical lane state.
    pub lane_token: String,
    /// State token copied from the canonical lane state.
    pub state_token: String,
    /// Human-readable current value.
    pub current_value_label: String,
    /// Human-readable explanation copied from the canonical lane state.
    pub explanation: String,
    /// Optional degraded token for the existing shell badge family.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub degraded_token: Option<String>,
    /// Command id that opens the detailed indexed-state view.
    pub primary_command_id: String,
    /// Surface ref opened by the primary command.
    pub opens_surface_ref: String,
    /// True when the status row must not be rendered as a current/full claim.
    pub current_claim_narrowed: bool,
    /// Truth source carried for support/debug joins.
    pub truth_source_ref: String,
}

/// Result-pane notice for one indexed lane.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct IndexStateResultPaneRecord {
    /// Stable record-kind tag for support and fixture exports.
    pub record_kind: String,
    /// Integer schema version for this alpha record.
    pub schema_version: u32,
    /// Workspace identity copied from the canonical lane state.
    pub workspace_id: String,
    /// Lane token copied from the canonical lane state.
    pub lane_token: String,
    /// State token copied from the canonical lane state.
    pub state_token: String,
    /// Banner label the results pane renders above affected rows.
    pub banner_label: String,
    /// Row caveat label rendered on affected rows.
    pub row_caveat_label: String,
    /// True when affected rows must show the caveat label.
    pub row_caveat_required: bool,
    /// Current subset copied from the canonical lane state.
    pub current_subset: Vec<String>,
    /// Actions that must stay narrowed or blocked in the result pane.
    pub blocked_actions: Vec<String>,
    /// Source records the notice was projected from.
    pub source_record_refs: Vec<String>,
}

/// Shell-facing bundle that proves status, result-pane, and support surfaces
/// all quote the same indexed-state token.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct IndexStateSurfaceBundle {
    /// Stable record-kind tag for support and fixture exports.
    pub record_kind: String,
    /// Integer schema version for this alpha record.
    pub schema_version: u32,
    /// Workspace identity copied from the canonical lane state.
    pub workspace_id: String,
    /// Lane token copied from the canonical lane state.
    pub lane_token: String,
    /// State token copied from the canonical lane state.
    pub state_token: String,
    /// Status-bar projection.
    pub status: IndexStateStatusRecord,
    /// Results-pane projection.
    pub result_pane: IndexStateResultPaneRecord,
    /// Redaction-safe support artifact projection.
    pub support_artifact: IndexedStateSupportArtifact,
}

impl IndexStateSurfaceBundle {
    /// Materializes all first consumer surfaces from one canonical lane state.
    pub fn from_lane_state(
        support_artifact_id: impl Into<String>,
        generated_at: impl Into<String>,
        state: &IndexedLaneState,
    ) -> Self {
        let support_artifact = IndexedStateSupportArtifact::from_lane_states(
            support_artifact_id,
            generated_at,
            std::slice::from_ref(state),
        );
        Self {
            record_kind: INDEX_STATE_SURFACE_BUNDLE_RECORD_KIND.to_string(),
            schema_version: INDEX_STATE_SURFACE_BUNDLE_SCHEMA_VERSION,
            workspace_id: state.workspace_id.clone(),
            lane_token: state.lane_token.clone(),
            state_token: state.state_token.clone(),
            status: IndexStateStatusRecord::from_lane_state(state),
            result_pane: IndexStateResultPaneRecord::from_lane_state(state),
            support_artifact,
        }
    }

    /// True when every surface quotes the same state token.
    pub fn has_single_state_token(&self) -> bool {
        self.status.state_token == self.state_token
            && self.result_pane.state_token == self.state_token
            && self
                .support_artifact
                .lane_states
                .iter()
                .all(|row| row.state_token == self.state_token)
    }
}

impl IndexStateStatusRecord {
    /// Materializes a status row from the canonical lane state.
    pub fn from_lane_state(state: &IndexedLaneState) -> Self {
        Self {
            record_kind: INDEX_STATE_STATUS_RECORD_KIND.to_string(),
            schema_version: INDEX_STATE_STATUS_SCHEMA_VERSION,
            status_item_id: format!("status.item.index_state.{}", state.lane_token),
            workspace_id: state.workspace_id.clone(),
            lane_token: state.lane_token.clone(),
            state_token: state.state_token.clone(),
            current_value_label: format!("{}: {}", state.lane.label(), state.state_label),
            explanation: state.honesty_note.clone(),
            degraded_token: degraded_token_for_state(state.state)
                .map(|token| token.token().to_string()),
            primary_command_id: "cmd:search.index_state.inspect".to_string(),
            opens_surface_ref: format!("surface.index_state.{}", state.lane_token),
            current_claim_narrowed: state.current_claim_narrowed,
            truth_source_ref: format!("indexed_lane_state:{}", state.lane_token),
        }
    }
}

impl IndexStateResultPaneRecord {
    /// Materializes a result-pane notice from the canonical lane state.
    pub fn from_lane_state(state: &IndexedLaneState) -> Self {
        Self {
            record_kind: INDEX_STATE_RESULT_PANE_RECORD_KIND.to_string(),
            schema_version: INDEX_STATE_RESULT_PANE_SCHEMA_VERSION,
            workspace_id: state.workspace_id.clone(),
            lane_token: state.lane_token.clone(),
            state_token: state.state_token.clone(),
            banner_label: format!(
                "{} {} {}",
                state.lane.label(),
                lane_state_verb(state),
                state.state.label()
            ),
            row_caveat_label: state.state.label().to_string(),
            row_caveat_required: state.result_rows_require_caveat,
            current_subset: state.current_subset.clone(),
            blocked_actions: state.blocked_actions.clone(),
            source_record_refs: state.source_record_refs.clone(),
        }
    }
}

fn degraded_token_for_state(state: IndexedStateClass) -> Option<DegradedStateToken> {
    match state {
        IndexedStateClass::Current => None,
        IndexedStateClass::Warming => Some(DegradedStateToken::Warming),
        IndexedStateClass::Partial => Some(DegradedStateToken::Partial),
        IndexedStateClass::Cached => Some(DegradedStateToken::Cached),
        IndexedStateClass::Stale => Some(DegradedStateToken::Stale),
        IndexedStateClass::Paused => Some(DegradedStateToken::Limited),
        IndexedStateClass::Unavailable => Some(DegradedStateToken::Offline),
    }
}

fn lane_state_verb(state: &IndexedLaneState) -> &'static str {
    if state.lane_token == "docs" {
        "are"
    } else {
        "is"
    }
}

#[cfg(test)]
mod tests {
    use aureline_search::{IndexedLaneKind, IndexedLaneStateInput, IndexedStateReason};

    use super::*;

    #[test]
    fn bundle_reuses_one_state_token() {
        let lane_state = IndexedLaneState::materialize(IndexedLaneStateInput {
            workspace_id: "ws-test".to_string(),
            lane: IndexedLaneKind::Search,
            state: IndexedStateClass::Paused,
            scope_label: "Selected workset".to_string(),
            observed_at: "mono:1".to_string(),
            freshness_label: None,
            index_epoch: Some("idx:paused".to_string()),
            graph_epoch: None,
            last_current_at: Some("mono:0".to_string()),
            reasons: vec![IndexedStateReason::PausedForForegroundLatency],
            current_subset: vec!["open files".to_string()],
            affected_lanes: vec!["semantic_graph".to_string()],
            safe_actions: Vec::new(),
            blocked_actions: Vec::new(),
            source_record_refs: Vec::new(),
        });
        let bundle = IndexStateSurfaceBundle::from_lane_state(
            "artifact:index-state:test",
            "mono:1",
            &lane_state,
        );
        assert!(bundle.has_single_state_token());
        assert_eq!(bundle.status.state_token, "paused");
        assert_eq!(bundle.result_pane.row_caveat_label, "Paused");
        assert_eq!(
            bundle.support_artifact.unsafe_current_claim_lanes(),
            Vec::<&str>::new()
        );
    }
}
