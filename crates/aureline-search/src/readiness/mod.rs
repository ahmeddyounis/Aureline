//! Canonical indexed-state honesty vocabulary for search-adjacent surfaces.
//!
//! This module is the single search-owned contract for derived index state
//! labels that need to travel through status chrome, search result panes,
//! graph/docs disclosures, and support artifacts. It intentionally sits above
//! the lower-level planner and lexical readiness enums: those contracts still
//! describe where a row came from, while this contract describes what a user
//! is allowed to believe about freshness and completeness right now.

use serde::{Deserialize, Serialize};

use crate::lexical::ReadinessClass;
use crate::planner::{PlannerFreshnessClass, PlannerPathReadiness};

/// Schema version for [`IndexedLaneState`] records.
pub const INDEXED_LANE_STATE_SCHEMA_VERSION: u32 = 1;

/// Schema version for [`IndexedStateSupportArtifact`] records.
pub const INDEXED_STATE_SUPPORT_ARTIFACT_SCHEMA_VERSION: u32 = 1;

/// Closed vocabulary for the visible state of indexed derived data.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum IndexedStateClass {
    /// Data is current for the declared scope.
    Current,
    /// Work is still warming and may not have produced complete rows yet.
    Warming,
    /// Some in-scope rows are available, but declared coverage is incomplete.
    Partial,
    /// A cached snapshot is being served within an accepted freshness window.
    Cached,
    /// Data is known to be out of date and must be refreshed or rebuilt.
    Stale,
    /// Background indexing or graph/doc refresh is intentionally paused.
    Paused,
    /// The lane cannot currently answer.
    Unavailable,
}

impl IndexedStateClass {
    /// Stable token used in records, fixtures, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Current => "current",
            Self::Warming => "warming",
            Self::Partial => "partial",
            Self::Cached => "cached",
            Self::Stale => "stale",
            Self::Paused => "paused",
            Self::Unavailable => "unavailable",
        }
    }

    /// Human-readable label rendered by status and result-pane surfaces.
    pub const fn label(self) -> &'static str {
        match self {
            Self::Current => "Current",
            Self::Warming => "Warming",
            Self::Partial => "Partial",
            Self::Cached => "Cached",
            Self::Stale => "Stale",
            Self::Paused => "Paused",
            Self::Unavailable => "Unavailable",
        }
    }

    /// True when a current/full-scope claim must be narrowed.
    pub const fn narrows_current_claim(self) -> bool {
        !matches!(self, Self::Current)
    }

    /// True when result rows must carry a visible state caveat.
    pub const fn requires_result_caveat(self) -> bool {
        matches!(
            self,
            Self::Warming | Self::Partial | Self::Cached | Self::Stale | Self::Paused
        )
    }

    /// Projects lexical readiness into the shared indexed-state vocabulary.
    pub const fn from_lexical_readiness(readiness: ReadinessClass) -> Self {
        match readiness {
            ReadinessClass::Ready => Self::Current,
            ReadinessClass::HotSetReady | ReadinessClass::Partial => Self::Partial,
            ReadinessClass::Warming => Self::Warming,
            ReadinessClass::Stale => Self::Stale,
            ReadinessClass::Unavailable | ReadinessClass::OutOfScope => Self::Unavailable,
        }
    }

    /// Projects planner readiness and freshness into the shared vocabulary.
    pub const fn from_planner_readiness(
        readiness: PlannerPathReadiness,
        freshness: PlannerFreshnessClass,
    ) -> Self {
        match freshness {
            PlannerFreshnessClass::StaleCached => return Self::Stale,
            PlannerFreshnessClass::WarmCached | PlannerFreshnessClass::Imported => {
                if matches!(
                    readiness,
                    PlannerPathReadiness::Ready | PlannerPathReadiness::HotSetReady
                ) {
                    return Self::Cached;
                }
            }
            PlannerFreshnessClass::AuthoritativeLive | PlannerFreshnessClass::Unknown => {}
        }

        match readiness {
            PlannerPathReadiness::Ready => Self::Current,
            PlannerPathReadiness::HotSetReady | PlannerPathReadiness::Partial => Self::Partial,
            PlannerPathReadiness::Warming => Self::Warming,
            PlannerPathReadiness::Stale => Self::Stale,
            PlannerPathReadiness::Unavailable | PlannerPathReadiness::OutOfScope => {
                Self::Unavailable
            }
        }
    }

    fn default_freshness_label(self) -> &'static str {
        match self {
            Self::Current => "current",
            Self::Warming => "warming",
            Self::Partial => "partial coverage",
            Self::Cached => "cached snapshot",
            Self::Stale => "stale snapshot",
            Self::Paused => "paused",
            Self::Unavailable => "unavailable",
        }
    }

    fn default_safe_actions(self) -> &'static [&'static str] {
        match self {
            Self::Current => &["open_results", "export_state"],
            Self::Warming => &["open_available_results", "view_index_status"],
            Self::Partial => &["open_available_results", "widen_scope_or_wait"],
            Self::Cached => &["open_cached_results", "refresh_index"],
            Self::Stale => &["open_with_stale_badge", "rebuild_index"],
            Self::Paused => &["open_available_results", "resume_indexing"],
            Self::Unavailable => &["repair_or_rebuild_index"],
        }
    }

    fn default_blocked_actions(self) -> &'static [&'static str] {
        match self {
            Self::Current => &[],
            Self::Warming | Self::Partial | Self::Cached | Self::Stale | Self::Paused => {
                &["broad_rename", "cross_root_apply"]
            }
            Self::Unavailable => &["broad_rename", "cross_root_apply", "result_replay"],
        }
    }

    fn default_honesty_note(self, lane: IndexedLaneKind) -> String {
        match self {
            Self::Current => format!("{} data is current for the declared scope.", lane.label()),
            Self::Warming => format!(
                "{} data is warming; available rows may be incomplete until background work catches up.",
                lane.label()
            ),
            Self::Partial => format!(
                "{} data is partial; visible rows cover only the declared subset.",
                lane.label()
            ),
            Self::Cached => format!(
                "{} data is cached; the surface must keep the cached label visible.",
                lane.label()
            ),
            Self::Stale => format!(
                "{} data is stale; the surface must not present it as current.",
                lane.label()
            ),
            Self::Paused => format!(
                "{} refresh is paused; available rows remain usable with a paused label.",
                lane.label()
            ),
            Self::Unavailable => format!(
                "{} data is unavailable; dependent actions must stay narrowed.",
                lane.label()
            ),
        }
    }
}

/// Indexed data lane that shares the honesty vocabulary.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum IndexedLaneKind {
    /// Search and quick-open index state.
    Search,
    /// Semantic graph or graph-backed navigation state.
    Graph,
    /// Documentation and help index state.
    Docs,
}

impl IndexedLaneKind {
    /// Stable token used in records, fixtures, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Search => "search",
            Self::Graph => "graph",
            Self::Docs => "docs",
        }
    }

    /// Human-readable lane label.
    pub const fn label(self) -> &'static str {
        match self {
            Self::Search => "Search",
            Self::Graph => "Graph",
            Self::Docs => "Docs",
        }
    }
}

/// Reason a lane is not current.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum IndexedStateReason {
    /// Workspace or index opened cold and is still warming.
    ColdStart,
    /// Only the bounded hot set is ready.
    HotSetOnly,
    /// Active scope or workset coverage is incomplete.
    ScopeIncomplete,
    /// A cached snapshot is being served.
    CachedSnapshot,
    /// Accepted freshness window has expired.
    FreshnessExpired,
    /// Refresh was paused to protect battery or thermal state.
    PausedForBattery,
    /// Refresh was paused to protect foreground typing or save latency.
    PausedForForegroundLatency,
    /// User or policy explicitly paused the lane.
    PausedByUser,
    /// Worker or provider cannot currently serve the lane.
    WorkerUnavailable,
    /// Graph data is still warming or rebuilding.
    GraphWarming,
    /// Documentation pack is served from cache.
    DocsPackCached,
    /// Active policy or trust posture limits the lane.
    PolicyLimited,
}

impl IndexedStateReason {
    /// Stable token used in records, fixtures, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ColdStart => "cold_start",
            Self::HotSetOnly => "hot_set_only",
            Self::ScopeIncomplete => "scope_incomplete",
            Self::CachedSnapshot => "cached_snapshot",
            Self::FreshnessExpired => "freshness_expired",
            Self::PausedForBattery => "paused_for_battery",
            Self::PausedForForegroundLatency => "paused_for_foreground_latency",
            Self::PausedByUser => "paused_by_user",
            Self::WorkerUnavailable => "worker_unavailable",
            Self::GraphWarming => "graph_warming",
            Self::DocsPackCached => "docs_pack_cached",
            Self::PolicyLimited => "policy_limited",
        }
    }
}

/// Inputs used to materialize one indexed lane state.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct IndexedLaneStateInput {
    /// Workspace identity for the state record.
    pub workspace_id: String,
    /// Lane described by this record.
    pub lane: IndexedLaneKind,
    /// Canonical indexed state.
    pub state: IndexedStateClass,
    /// Human-readable scope or workset label.
    pub scope_label: String,
    /// Monotonic or fixture timestamp for export parity.
    pub observed_at: String,
    /// Human-readable freshness label for the lane.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub freshness_label: Option<String>,
    /// Index epoch, shard epoch, or cache epoch used by the lane.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub index_epoch: Option<String>,
    /// Graph epoch used by graph-backed answers, when any.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub graph_epoch: Option<String>,
    /// Last time the lane was current, when known.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_current_at: Option<String>,
    /// Reasons explaining why the lane is not current.
    #[serde(default)]
    pub reasons: Vec<IndexedStateReason>,
    /// Human-readable lane details that remain usable.
    #[serde(default)]
    pub current_subset: Vec<String>,
    /// Human-readable affected sublanes.
    #[serde(default)]
    pub affected_lanes: Vec<String>,
    /// Safe actions that remain available in this state.
    #[serde(default)]
    pub safe_actions: Vec<String>,
    /// Actions that must stay narrowed or blocked.
    #[serde(default)]
    pub blocked_actions: Vec<String>,
    /// Source records the lane state was projected from.
    #[serde(default)]
    pub source_record_refs: Vec<String>,
}

/// Canonical state record for one indexed data lane.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct IndexedLaneState {
    /// Stable record-kind tag for support and fixture exports.
    pub record_kind: String,
    /// Integer schema version for this alpha record.
    pub schema_version: u32,
    /// Workspace identity for the state record.
    pub workspace_id: String,
    /// Lane described by this record.
    pub lane: IndexedLaneKind,
    /// Stable lane token.
    pub lane_token: String,
    /// Canonical indexed state.
    pub state: IndexedStateClass,
    /// Stable indexed-state token.
    pub state_token: String,
    /// Human-readable indexed-state label.
    pub state_label: String,
    /// Human-readable scope or workset label.
    pub scope_label: String,
    /// Human-readable freshness label for the lane.
    pub freshness_label: String,
    /// Stable reason tokens.
    pub reason_tokens: Vec<String>,
    /// Human-readable lane details that remain usable.
    pub current_subset: Vec<String>,
    /// Human-readable affected sublanes.
    pub affected_lanes: Vec<String>,
    /// Safe actions that remain available in this state.
    pub safe_actions: Vec<String>,
    /// Actions that must stay narrowed or blocked.
    pub blocked_actions: Vec<String>,
    /// True when a current/full-scope claim must be narrowed.
    pub current_claim_narrowed: bool,
    /// True when result rows must carry a state caveat.
    pub result_rows_require_caveat: bool,
    /// Human-readable note that keeps the degraded state explicit.
    pub honesty_note: String,
    /// Index epoch, shard epoch, or cache epoch used by the lane.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub index_epoch: Option<String>,
    /// Graph epoch used by graph-backed answers, when any.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub graph_epoch: Option<String>,
    /// Last time the lane was current, when known.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_current_at: Option<String>,
    /// Monotonic or fixture timestamp for export parity.
    pub observed_at: String,
    /// Source records the lane state was projected from.
    pub source_record_refs: Vec<String>,
}

impl IndexedLaneState {
    /// Stable record-kind tag carried in serialized lane state records.
    pub const RECORD_KIND: &'static str = "indexed_lane_state";

    /// Materializes one canonical lane state from caller-owned inputs.
    pub fn materialize(input: IndexedLaneStateInput) -> Self {
        let reason_tokens = sorted_unique_reason_tokens(&input.reasons);
        let safe_actions = if input.safe_actions.is_empty() {
            input
                .state
                .default_safe_actions()
                .iter()
                .map(|value| (*value).to_string())
                .collect()
        } else {
            sorted_unique_strings(input.safe_actions)
        };
        let blocked_actions = if input.blocked_actions.is_empty() {
            input
                .state
                .default_blocked_actions()
                .iter()
                .map(|value| (*value).to_string())
                .collect()
        } else {
            sorted_unique_strings(input.blocked_actions)
        };
        let freshness_label = input
            .freshness_label
            .unwrap_or_else(|| input.state.default_freshness_label().to_string());

        Self {
            record_kind: Self::RECORD_KIND.to_string(),
            schema_version: INDEXED_LANE_STATE_SCHEMA_VERSION,
            workspace_id: input.workspace_id,
            lane: input.lane,
            lane_token: input.lane.as_str().to_string(),
            state: input.state,
            state_token: input.state.as_str().to_string(),
            state_label: input.state.label().to_string(),
            scope_label: input.scope_label,
            freshness_label,
            reason_tokens,
            current_subset: sorted_unique_strings(input.current_subset),
            affected_lanes: sorted_unique_strings(input.affected_lanes),
            safe_actions,
            blocked_actions,
            current_claim_narrowed: input.state.narrows_current_claim(),
            result_rows_require_caveat: input.state.requires_result_caveat(),
            honesty_note: input.state.default_honesty_note(input.lane),
            index_epoch: input.index_epoch,
            graph_epoch: input.graph_epoch,
            last_current_at: input.last_current_at,
            observed_at: input.observed_at,
            source_record_refs: sorted_unique_strings(input.source_record_refs),
        }
    }

    /// True when this state would be unsafe to present as fully current.
    pub const fn forbids_full_current_claim(&self) -> bool {
        self.current_claim_narrowed
    }
}

/// Support-export row for one indexed lane state.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct IndexedLaneSupportRow {
    /// Lane token copied from [`IndexedLaneState`].
    pub lane_token: String,
    /// State token copied from [`IndexedLaneState`].
    pub state_token: String,
    /// Scope label copied from [`IndexedLaneState`].
    pub scope_label: String,
    /// Freshness label copied from [`IndexedLaneState`].
    pub freshness_label: String,
    /// Stable reason tokens.
    pub reason_tokens: Vec<String>,
    /// True when a current/full-scope claim has been narrowed.
    pub current_claim_narrowed: bool,
    /// Actions that stayed blocked in the exported evidence.
    pub blocked_actions: Vec<String>,
    /// Source records the support row can join back to.
    pub source_record_refs: Vec<String>,
}

/// Redaction-safe support artifact for indexed-state truth.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct IndexedStateSupportArtifact {
    /// Stable record-kind tag for support and fixture exports.
    pub record_kind: String,
    /// Integer schema version for this alpha artifact.
    pub schema_version: u32,
    /// Stable support artifact identity.
    pub artifact_id: String,
    /// Workspace identity for the artifact.
    pub workspace_id: String,
    /// Generation timestamp for the artifact.
    pub generated_at: String,
    /// Lane state rows included in the artifact.
    pub lane_states: Vec<IndexedLaneSupportRow>,
    /// True because the artifact carries metadata only.
    pub raw_private_material_excluded: bool,
}

impl IndexedStateSupportArtifact {
    /// Stable record-kind tag carried in serialized support artifacts.
    pub const RECORD_KIND: &'static str = "indexed_state_support_artifact";

    /// Builds a redaction-safe support artifact from canonical lane states.
    pub fn from_lane_states(
        artifact_id: impl Into<String>,
        generated_at: impl Into<String>,
        states: &[IndexedLaneState],
    ) -> Self {
        let workspace_id = states
            .first()
            .map(|state| state.workspace_id.clone())
            .unwrap_or_default();
        let lane_states = states
            .iter()
            .map(|state| IndexedLaneSupportRow {
                lane_token: state.lane_token.clone(),
                state_token: state.state_token.clone(),
                scope_label: state.scope_label.clone(),
                freshness_label: state.freshness_label.clone(),
                reason_tokens: state.reason_tokens.clone(),
                current_claim_narrowed: state.current_claim_narrowed,
                blocked_actions: state.blocked_actions.clone(),
                source_record_refs: state.source_record_refs.clone(),
            })
            .collect();

        Self {
            record_kind: Self::RECORD_KIND.to_string(),
            schema_version: INDEXED_STATE_SUPPORT_ARTIFACT_SCHEMA_VERSION,
            artifact_id: artifact_id.into(),
            workspace_id,
            generated_at: generated_at.into(),
            lane_states,
            raw_private_material_excluded: true,
        }
    }

    /// Returns lane tokens whose support rows still look like unsafe claims.
    pub fn unsafe_current_claim_lanes(&self) -> Vec<&str> {
        self.lane_states
            .iter()
            .filter(|row| row.state_token != IndexedStateClass::Current.as_str())
            .filter(|row| !row.current_claim_narrowed || row.blocked_actions.is_empty())
            .map(|row| row.lane_token.as_str())
            .collect()
    }
}

fn sorted_unique_reason_tokens(reasons: &[IndexedStateReason]) -> Vec<String> {
    let mut tokens: Vec<String> = reasons
        .iter()
        .map(|reason| reason.as_str().to_string())
        .collect();
    tokens.sort();
    tokens.dedup();
    tokens
}

fn sorted_unique_strings(mut values: Vec<String>) -> Vec<String> {
    values.retain(|value| !value.trim().is_empty());
    values.iter_mut().for_each(|value| {
        if value.trim() != value {
            *value = value.trim().to_string();
        }
    });
    values.sort();
    values.dedup();
    values
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn planner_warm_cached_projects_to_cached_state() {
        assert_eq!(
            IndexedStateClass::from_planner_readiness(
                PlannerPathReadiness::Ready,
                PlannerFreshnessClass::WarmCached,
            ),
            IndexedStateClass::Cached
        );
    }

    #[test]
    fn partial_state_blocks_full_current_claim() {
        let state = IndexedLaneState::materialize(IndexedLaneStateInput {
            workspace_id: "ws-test".to_string(),
            lane: IndexedLaneKind::Search,
            state: IndexedStateClass::Partial,
            scope_label: "Selected workset".to_string(),
            observed_at: "mono:1".to_string(),
            freshness_label: None,
            index_epoch: Some("idx:1".to_string()),
            graph_epoch: None,
            last_current_at: None,
            reasons: vec![IndexedStateReason::HotSetOnly],
            current_subset: Vec::new(),
            affected_lanes: Vec::new(),
            safe_actions: Vec::new(),
            blocked_actions: Vec::new(),
            source_record_refs: Vec::new(),
        });
        assert!(state.forbids_full_current_claim());
        assert_eq!(state.state_token, "partial");
        assert!(!state.blocked_actions.is_empty());
    }
}
