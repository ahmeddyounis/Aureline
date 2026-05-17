//! Beta ranking-reason and operator-truth packets for search-owned rows.
//!
//! This module joins the row-level `Why this result?` explanation with the
//! indexed-readiness and partial-index drill state that governs beta graph,
//! AI, review, and support projections. It is intentionally metadata-only:
//! packets carry opaque refs, controlled vocabulary, and short reviewable
//! summaries instead of raw query text, raw source bodies, private rank
//! weights, provider payloads, credentials, or ambient authority.

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

use crate::hybrid_retrieval::{
    RetrievalInspectorPacket, RetrievalInspectorRow, RetrievalLaneClass, RetrievalReadinessClass,
    RetrievalReasonClass,
};
use crate::planner::{PlannerPathReadiness, PlannerResultTruthClass};

/// Stable record-kind tag for [`SearchOperatorTruthPacket`].
pub const SEARCH_OPERATOR_TRUTH_PACKET_RECORD_KIND: &str = "search_operator_truth_beta_packet";

/// Stable record-kind tag for [`PartialIndexDrillPacket`].
pub const PARTIAL_INDEX_DRILL_PACKET_RECORD_KIND: &str = "partial_index_drill_beta_packet";

/// Stable record-kind tag for [`SearchOperatorTruthSupportExport`].
pub const SEARCH_OPERATOR_TRUTH_SUPPORT_EXPORT_RECORD_KIND: &str =
    "search_operator_truth_support_export";

/// Schema version for search operator-truth beta packets.
pub const SEARCH_OPERATOR_TRUTH_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the boundary schema.
pub const SEARCH_OPERATOR_TRUTH_SCHEMA_REF: &str =
    "schemas/search/search_operator_truth_packet.schema.json";

/// Repo-relative path of the reviewer doc.
pub const SEARCH_OPERATOR_TRUTH_DOC_REF: &str = "docs/search/m3/ranking_reason_beta.md";

/// Repo-relative path of the protected fixture corpus directory.
pub const SEARCH_OPERATOR_TRUTH_FIXTURE_DIR: &str = "fixtures/search/m3/operator_truth_packets";

/// Repo-relative path of the checked-in beta operator-truth packet.
pub const SEARCH_OPERATOR_TRUTH_PACKET_ARTIFACT_REF: &str =
    "artifacts/search/m3/operator_truth_packets/search_operator_truth_beta_packet.json";

/// Consumer surface that must inherit the search-owned operator-truth packet.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SearchOperatorConsumerSurface {
    /// Search result pane or row inspector.
    SearchResults,
    /// Graph overlay, topology, or graph-backed result inspector.
    GraphOverlay,
    /// AI context picker or context inspector.
    AiContext,
    /// Review workspace or review-assist evidence lane.
    ReviewWorkspace,
    /// Support export or issue-report packet.
    SupportExport,
    /// CLI or headless inspection surface.
    CliHeadless,
}

impl SearchOperatorConsumerSurface {
    /// Stable token used in fixtures, schemas, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SearchResults => "search_results",
            Self::GraphOverlay => "graph_overlay",
            Self::AiContext => "ai_context",
            Self::ReviewWorkspace => "review_workspace",
            Self::SupportExport => "support_export",
            Self::CliHeadless => "cli_headless",
        }
    }
}

/// Closed ranking-reason vocabulary exposed by the beta packet.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RankingReasonSignal {
    /// Exact lexical or graph name match.
    ExactNameMatch,
    /// Prefix lexical match.
    LexicalPrefixMatch,
    /// Substring lexical match.
    LexicalSubstringMatch,
    /// Workspace path match.
    LexicalPathMatch,
    /// Semantic vector similarity contributed to ranking.
    VectorSemanticSimilarity,
    /// Graph neighborhood expansion contributed to ranking.
    GraphExpansion,
    /// Graph exact entity match contributed to ranking.
    GraphExactEntity,
    /// Recent file, edit, or hot-set signal boosted the row.
    RecencyBoost,
    /// Partial, warming, or stale index state affected the row.
    PartialIndex,
    /// Local fallback answered because a stronger lane was unavailable.
    LocalFallback,
    /// Remote or managed route affected ranking and was disclosed.
    RemoteRoute,
    /// Policy or trust posture limited ranking.
    PolicyLimited,
    /// Hidden scope or withheld rows affected the result set.
    HiddenScope,
    /// Stale shard or stale graph evidence affected the result set.
    StaleShard,
}

impl RankingReasonSignal {
    /// Stable token used in fixtures, schemas, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ExactNameMatch => "exact_name_match",
            Self::LexicalPrefixMatch => "lexical_prefix_match",
            Self::LexicalSubstringMatch => "lexical_substring_match",
            Self::LexicalPathMatch => "lexical_path_match",
            Self::VectorSemanticSimilarity => "vector_semantic_similarity",
            Self::GraphExpansion => "graph_expansion",
            Self::GraphExactEntity => "graph_exact_entity",
            Self::RecencyBoost => "recency_boost",
            Self::PartialIndex => "partial_index",
            Self::LocalFallback => "local_fallback",
            Self::RemoteRoute => "remote_route",
            Self::PolicyLimited => "policy_limited",
            Self::HiddenScope => "hidden_scope",
            Self::StaleShard => "stale_shard",
        }
    }

    fn from_retrieval_reason(reason: RetrievalReasonClass) -> Self {
        match reason {
            RetrievalReasonClass::LexicalExactMatch => Self::ExactNameMatch,
            RetrievalReasonClass::LexicalPrefixMatch => Self::LexicalPrefixMatch,
            RetrievalReasonClass::LexicalSubstringMatch => Self::LexicalSubstringMatch,
            RetrievalReasonClass::LexicalPathMatch => Self::LexicalPathMatch,
            RetrievalReasonClass::VectorSemanticSimilarity => Self::VectorSemanticSimilarity,
            RetrievalReasonClass::GraphExpansion => Self::GraphExpansion,
            RetrievalReasonClass::GraphExactEntity => Self::GraphExactEntity,
            RetrievalReasonClass::RecencyBoost => Self::RecencyBoost,
            RetrievalReasonClass::PartialIndex => Self::PartialIndex,
            RetrievalReasonClass::LocalFallback => Self::LocalFallback,
            RetrievalReasonClass::RemoteRoute => Self::RemoteRoute,
            RetrievalReasonClass::PolicyLimited => Self::PolicyLimited,
        }
    }
}

/// State exercised by a reusable partial-index drill.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PartialIndexDrillState {
    /// Lane was current for the declared scope.
    Current,
    /// Lane had partial declared-scope coverage.
    PartialIndex,
    /// At least one indexed shard or graph slice was stale.
    StaleShard,
    /// Rows existed outside the active workset, sparse scope, or policy view.
    HiddenScope,
    /// Drill detected a failing indexed lane.
    Failing,
    /// Lane was still warming.
    Warming,
}

impl PartialIndexDrillState {
    /// Stable token used in fixtures, schemas, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Current => "current",
            Self::PartialIndex => "partial_index",
            Self::StaleShard => "stale_shard",
            Self::HiddenScope => "hidden_scope",
            Self::Failing => "failing",
            Self::Warming => "warming",
        }
    }

    /// True when affected rows must be visibly downgraded.
    pub const fn requires_row_downgrade(self) -> bool {
        !matches!(self, Self::Current)
    }
}

/// Downgrade state applied to a row or beta packet.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SearchOperatorDowngradeState {
    /// No downgrade is required.
    None,
    /// Partial index or graph coverage narrows the row claim.
    YellowPartialIndex,
    /// Stale index or graph evidence narrows the row claim.
    YellowStaleShard,
    /// Hidden scope narrows the row claim.
    YellowHiddenScope,
    /// Search fell back to non-graph search truth only.
    FallbackSearchOnly,
    /// The row blocks beta promotion until corrected.
    RedBlocksBetaPromotion,
}

impl SearchOperatorDowngradeState {
    /// Stable token used in fixtures, schemas, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::None => "none",
            Self::YellowPartialIndex => "yellow_partial_index",
            Self::YellowStaleShard => "yellow_stale_shard",
            Self::YellowHiddenScope => "yellow_hidden_scope",
            Self::FallbackSearchOnly => "fallback_search_only",
            Self::RedBlocksBetaPromotion => "red_blocks_beta_promotion",
        }
    }

    /// True when this downgrade blocks beta promotion.
    pub const fn blocks_promotion(self) -> bool {
        matches!(self, Self::RedBlocksBetaPromotion)
    }
}

/// Promotion state derived from packet validation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SearchOperatorPromotionState {
    /// Packet is eligible for the claimed beta row.
    Promotable,
    /// Packet is usable but carries reviewable caveats.
    NeedsReview,
    /// Packet blocks promotion until corrected.
    Blocked,
}

impl SearchOperatorPromotionState {
    /// Stable token used in fixtures, schemas, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Promotable => "promotable",
            Self::NeedsReview => "needs_review",
            Self::Blocked => "blocked",
        }
    }
}

/// Severity attached to one operator-truth validation finding.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SearchOperatorTruthFindingSeverity {
    /// Informational finding.
    Info,
    /// Reviewable finding that narrows promotion confidence.
    Warning,
    /// Finding that blocks beta promotion.
    Blocker,
}

/// Closed validation finding vocabulary for [`SearchOperatorTruthPacket`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SearchOperatorTruthFindingKind {
    /// Record kind does not match the schema.
    WrongRecordKind,
    /// Schema version does not match the frozen beta schema.
    WrongSchemaVersion,
    /// Required identity field is empty.
    MissingIdentity,
    /// A row has no ranking reasons.
    MissingRankingReason,
    /// A row has no readiness label.
    MissingReadinessLabel,
    /// A degraded row has no partial-truth cause.
    MissingPartialTruthCause,
    /// A row has no source packet ref.
    MissingSourcePacketRef,
    /// The embedded partial-index drill is missing or not reusable.
    PartialIndexDrillNotReusable,
    /// A partial, stale, hidden, warming, or failing drill row was not downgraded.
    PartialIndexRowNotDowngraded,
    /// A required consumer surface does not preserve the packet.
    MissingConsumerProjection,
    /// A consumer projection remints or drops operator truth.
    ConsumerProjectionDrift,
    /// The upstream retrieval packet was not promotable.
    RetrievalPacketInvalid,
    /// Packet contains raw boundary material or private weights.
    RawBoundaryMaterialPresent,
    /// Stored promotion state disagrees with derived validation findings.
    PromotionStateMismatch,
}

impl SearchOperatorTruthFindingKind {
    /// Stable token used in fixtures, schemas, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WrongRecordKind => "wrong_record_kind",
            Self::WrongSchemaVersion => "wrong_schema_version",
            Self::MissingIdentity => "missing_identity",
            Self::MissingRankingReason => "missing_ranking_reason",
            Self::MissingReadinessLabel => "missing_readiness_label",
            Self::MissingPartialTruthCause => "missing_partial_truth_cause",
            Self::MissingSourcePacketRef => "missing_source_packet_ref",
            Self::PartialIndexDrillNotReusable => "partial_index_drill_not_reusable",
            Self::PartialIndexRowNotDowngraded => "partial_index_row_not_downgraded",
            Self::MissingConsumerProjection => "missing_consumer_projection",
            Self::ConsumerProjectionDrift => "consumer_projection_drift",
            Self::RetrievalPacketInvalid => "retrieval_packet_invalid",
            Self::RawBoundaryMaterialPresent => "raw_boundary_material_present",
            Self::PromotionStateMismatch => "promotion_state_mismatch",
        }
    }
}

/// One validation finding emitted by the operator-truth validator.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SearchOperatorTruthFinding {
    /// Closed finding kind.
    pub finding_kind: SearchOperatorTruthFindingKind,
    /// Finding severity.
    pub severity: SearchOperatorTruthFindingSeverity,
    /// Short support-safe summary.
    pub summary: String,
}

impl SearchOperatorTruthFinding {
    fn new(
        finding_kind: SearchOperatorTruthFindingKind,
        severity: SearchOperatorTruthFindingSeverity,
        summary: impl Into<String>,
    ) -> Self {
        Self {
            finding_kind,
            severity,
            summary: summary.into(),
        }
    }
}

/// One row in a partial-index drill packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PartialIndexDrillRow {
    /// Stable drill row id.
    pub row_id: String,
    /// Search result, graph cue, or retrieval row affected by this drill row.
    pub affected_result_ref: String,
    /// Drill state observed for this row.
    pub state: PartialIndexDrillState,
    /// Visible downgrade applied to the affected row.
    pub downgrade_state: SearchOperatorDowngradeState,
    /// Stable reason tokens explaining the state.
    #[serde(default)]
    pub reason_tokens: Vec<String>,
    /// Actions narrowed or blocked by this drill row.
    #[serde(default)]
    pub blocked_actions: Vec<String>,
}

impl PartialIndexDrillRow {
    fn is_downgraded_when_required(&self) -> bool {
        if !self.state.requires_row_downgrade() {
            return self.downgrade_state == SearchOperatorDowngradeState::None;
        }
        self.downgrade_state != SearchOperatorDowngradeState::None
            && !self.blocked_actions.is_empty()
    }
}

/// Reusable partial-index drill packet embedded in operator-truth packets.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PartialIndexDrillPacket {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Stable drill id.
    pub drill_id: String,
    /// Drill capture timestamp.
    pub generated_at: String,
    /// Indexed-state, graph-cue, retrieval, or support rows consumed by the drill.
    #[serde(default)]
    pub source_lane_state_refs: Vec<String>,
    /// Per-row drill findings and downgrades.
    #[serde(default)]
    pub rows: Vec<PartialIndexDrillRow>,
    /// Reusable support packet or artifact ref.
    pub support_packet_ref: String,
    /// True when another consumer can reuse the packet without private debug tooling.
    pub reusable_support_packet: bool,
    /// True when raw source, query, provider, and credential material is excluded.
    pub raw_private_material_excluded: bool,
}

impl PartialIndexDrillPacket {
    /// Returns stable state tokens covered by this drill packet.
    pub fn covered_state_tokens(&self) -> Vec<&'static str> {
        let mut states = BTreeSet::new();
        for row in &self.rows {
            states.insert(row.state);
        }
        states
            .into_iter()
            .map(PartialIndexDrillState::as_str)
            .collect()
    }

    /// Returns true when every degraded drill row has a visible downgrade.
    pub fn all_degraded_rows_downgraded(&self) -> bool {
        !self.rows.is_empty()
            && self
                .rows
                .iter()
                .all(PartialIndexDrillRow::is_downgraded_when_required)
    }

    /// Returns true when the drill can be reused by support, AI, and review surfaces.
    pub fn is_reusable(&self) -> bool {
        self.record_kind == PARTIAL_INDEX_DRILL_PACKET_RECORD_KIND
            && self.schema_version == SEARCH_OPERATOR_TRUTH_SCHEMA_VERSION
            && !self.drill_id.trim().is_empty()
            && !self.support_packet_ref.trim().is_empty()
            && self.reusable_support_packet
            && self.raw_private_material_excluded
            && self.all_degraded_rows_downgraded()
    }
}

/// One search row whose ranking and readiness are inspectable.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SearchOperatorTruthRow {
    /// Stable row id inside the packet.
    pub row_id: String,
    /// Product result row or graph/AI context row that rendered this truth.
    pub rendered_result_ref: String,
    /// Canonical target ref.
    pub target_ref: String,
    /// Display label safe for support and docs.
    pub display_title: String,
    /// Result truth class for the row.
    pub result_truth_class: PlannerResultTruthClass,
    /// Readiness state attached to the row.
    pub readiness_state: PlannerPathReadiness,
    /// Visible readiness label shown by product and export surfaces.
    pub readiness_label: String,
    /// Ordered ranking reasons.
    #[serde(default)]
    pub ranking_reasons: Vec<RankingReasonSignal>,
    /// Partial-truth causes attached to the row.
    #[serde(default)]
    pub partial_truth_causes: Vec<String>,
    /// Scope or workset boundary shown with the row.
    pub scope_label: String,
    /// Freshness label shown with the row.
    pub freshness_label: String,
    /// Source packets this row joins back to.
    #[serde(default)]
    pub source_packet_refs: Vec<String>,
    /// Downgrade applied to this row.
    pub downgrade_state: SearchOperatorDowngradeState,
    /// True when the row blocks beta promotion.
    pub row_blocks_beta_promotion: bool,
    /// Short `Why this result?` summary.
    pub why_this_result: String,
    /// Actions still admitted for this row.
    #[serde(default)]
    pub safe_actions: Vec<String>,
    /// Actions narrowed or blocked for this row.
    #[serde(default)]
    pub blocked_actions: Vec<String>,
    /// True when private weights and debug-only scoring inputs are excluded.
    pub private_debug_weights_excluded: bool,
}

impl SearchOperatorTruthRow {
    /// Builds a row from a retrieval-inspector row.
    pub fn from_retrieval_row(
        row: &RetrievalInspectorRow,
        retrieval_packet_id: impl Into<String>,
        scope_label: impl Into<String>,
        freshness_label: impl Into<String>,
    ) -> Self {
        let retrieval_packet_id = retrieval_packet_id.into();
        let readiness_state = readiness_from_retrieval(row.readiness);
        let partial_truth_causes = partial_truth_causes_for_retrieval_row(row);
        let downgrade_state = downgrade_for_retrieval(row.readiness, &partial_truth_causes);
        let blocked_actions = if downgrade_state == SearchOperatorDowngradeState::None {
            Vec::new()
        } else {
            vec!["broad_rename".to_owned(), "cross_root_apply".to_owned()]
        };
        Self {
            row_id: row.row_id.clone(),
            rendered_result_ref: row.rendered_result_ref.clone(),
            target_ref: row.canonical_id.clone(),
            display_title: row.display_title.clone(),
            result_truth_class: truth_class_from_retrieval_lane(row.selected_lane_class),
            readiness_state,
            readiness_label: row.readiness.as_str().to_owned(),
            ranking_reasons: ranking_reasons_for_retrieval_row(row),
            partial_truth_causes,
            scope_label: scope_label.into(),
            freshness_label: freshness_label.into(),
            source_packet_refs: vec![retrieval_packet_id],
            downgrade_state,
            row_blocks_beta_promotion: downgrade_state.blocks_promotion(),
            why_this_result: row.explanation.clone(),
            safe_actions: vec!["open_result".to_owned(), "why_this_result".to_owned()],
            blocked_actions,
            private_debug_weights_excluded: true,
        }
    }
}

/// Consumer projection proving a surface reads the same operator-truth packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SearchOperatorTruthProjection {
    /// Consumer surface class.
    pub consumer_surface: SearchOperatorConsumerSurface,
    /// Stable projection ref.
    pub projection_ref: String,
    /// Operator-truth packet id consumed by the projection.
    pub operator_truth_packet_id_ref: String,
    /// Render timestamp.
    pub rendered_at: String,
    /// True when the surface preserves the same packet id.
    pub preserves_same_packet: bool,
    /// True when ranking reasons are quoted from the packet.
    pub preserves_ranking_reasons: bool,
    /// True when readiness labels are quoted from the packet.
    pub preserves_readiness_label: bool,
    /// True when the partial-index drill ref is reachable from the projection.
    pub preserves_partial_index_drill_ref: bool,
    /// True when JSON export is available from the projection.
    pub supports_json_export: bool,
    /// True when raw private material is excluded.
    pub raw_private_material_excluded: bool,
    /// True when ambient credentials or authority are excluded.
    pub ambient_authority_excluded: bool,
}

impl SearchOperatorTruthProjection {
    fn preserves_truth_for(&self, packet_id: &str) -> bool {
        self.operator_truth_packet_id_ref == packet_id
            && self.preserves_same_packet
            && self.preserves_ranking_reasons
            && self.preserves_readiness_label
            && self.preserves_partial_index_drill_ref
            && self.supports_json_export
            && self.raw_private_material_excluded
            && self.ambient_authority_excluded
            && !self.projection_ref.trim().is_empty()
    }
}

/// Constructor input for [`SearchOperatorTruthPacket::materialize`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SearchOperatorTruthPacketInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Claimed workflow or surface id.
    pub workflow_or_surface_id: String,
    /// Query session that produced the rows.
    pub query_session_id_ref: String,
    /// Planner pass that produced the rows.
    pub planner_pass_id_ref: String,
    /// Retrieval packet consumed by this packet, when any.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub retrieval_packet_id_ref: Option<String>,
    /// Promotion token copied from the upstream retrieval packet.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub retrieval_promotion_state_token: Option<String>,
    /// Validation finding tokens copied from the upstream retrieval packet.
    #[serde(default)]
    pub retrieval_validation_finding_tokens: Vec<String>,
    /// Graph cue packets inherited by graph, AI, or review surfaces.
    #[serde(default)]
    pub graph_cue_packet_refs: Vec<String>,
    /// Reusable partial-index drill packet.
    pub partial_index_drill: PartialIndexDrillPacket,
    /// Inspectable rows.
    #[serde(default)]
    pub rows: Vec<SearchOperatorTruthRow>,
    /// Consumer projections preserving this packet.
    #[serde(default)]
    pub consumer_projections: Vec<SearchOperatorTruthProjection>,
    /// Source contract refs consumed by the packet.
    #[serde(default)]
    pub source_contract_refs: Vec<String>,
    /// Packet generation timestamp.
    pub generated_at: String,
}

/// Search-owned packet for ranking reasons, readiness, partial-index drill, and projections.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SearchOperatorTruthPacket {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Claimed workflow or surface id.
    pub workflow_or_surface_id: String,
    /// Query session that produced the rows.
    pub query_session_id_ref: String,
    /// Planner pass that produced the rows.
    pub planner_pass_id_ref: String,
    /// Retrieval packet consumed by this packet, when any.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub retrieval_packet_id_ref: Option<String>,
    /// Promotion token copied from the upstream retrieval packet.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub retrieval_promotion_state_token: Option<String>,
    /// Validation finding tokens copied from the upstream retrieval packet.
    #[serde(default)]
    pub retrieval_validation_finding_tokens: Vec<String>,
    /// Graph cue packets inherited by graph, AI, or review surfaces.
    #[serde(default)]
    pub graph_cue_packet_refs: Vec<String>,
    /// Reusable partial-index drill packet.
    pub partial_index_drill: PartialIndexDrillPacket,
    /// Inspectable rows.
    #[serde(default)]
    pub rows: Vec<SearchOperatorTruthRow>,
    /// Consumer projections preserving this packet.
    #[serde(default)]
    pub consumer_projections: Vec<SearchOperatorTruthProjection>,
    /// Source contract refs consumed by the packet.
    #[serde(default)]
    pub source_contract_refs: Vec<String>,
    /// Packet generation timestamp.
    pub generated_at: String,
    /// Derived promotion state.
    pub promotion_state: SearchOperatorPromotionState,
    /// Validation findings captured when the packet was materialized.
    #[serde(default)]
    pub validation_findings: Vec<SearchOperatorTruthFinding>,
}

impl SearchOperatorTruthPacket {
    /// Materializes a packet and records derived validation findings.
    pub fn materialize(input: SearchOperatorTruthPacketInput) -> Self {
        let mut packet = Self {
            record_kind: SEARCH_OPERATOR_TRUTH_PACKET_RECORD_KIND.to_owned(),
            schema_version: SEARCH_OPERATOR_TRUTH_SCHEMA_VERSION,
            packet_id: input.packet_id,
            workflow_or_surface_id: input.workflow_or_surface_id,
            query_session_id_ref: input.query_session_id_ref,
            planner_pass_id_ref: input.planner_pass_id_ref,
            retrieval_packet_id_ref: input.retrieval_packet_id_ref,
            retrieval_promotion_state_token: input.retrieval_promotion_state_token,
            retrieval_validation_finding_tokens: input.retrieval_validation_finding_tokens,
            graph_cue_packet_refs: input.graph_cue_packet_refs,
            partial_index_drill: input.partial_index_drill,
            rows: input.rows,
            consumer_projections: input.consumer_projections,
            source_contract_refs: input.source_contract_refs,
            generated_at: input.generated_at,
            promotion_state: SearchOperatorPromotionState::Promotable,
            validation_findings: Vec::new(),
        };
        let findings = packet.derived_findings(false);
        packet.promotion_state = promotion_state_for_findings(&findings, &packet.rows);
        packet.validation_findings = findings;
        packet
    }

    /// Builds operator-truth input from an existing retrieval packet.
    pub fn input_from_retrieval_packet(
        packet_id: impl Into<String>,
        workflow_or_surface_id: impl Into<String>,
        generated_at: impl Into<String>,
        retrieval_packet: &RetrievalInspectorPacket,
        partial_index_drill: PartialIndexDrillPacket,
        consumer_projections: Vec<SearchOperatorTruthProjection>,
        source_contract_refs: Vec<String>,
    ) -> SearchOperatorTruthPacketInput {
        let packet_id = packet_id.into();
        SearchOperatorTruthPacketInput {
            packet_id,
            workflow_or_surface_id: workflow_or_surface_id.into(),
            query_session_id_ref: retrieval_packet.query_session_id_ref.clone(),
            planner_pass_id_ref: retrieval_packet.planner_pass_id_ref.clone(),
            retrieval_packet_id_ref: Some(retrieval_packet.packet_id.clone()),
            retrieval_promotion_state_token: Some(
                retrieval_packet.promotion_state.as_str().to_owned(),
            ),
            retrieval_validation_finding_tokens: retrieval_packet
                .validate()
                .iter()
                .map(|finding| finding.finding_kind.as_str().to_owned())
                .collect(),
            graph_cue_packet_refs: retrieval_packet
                .rows
                .iter()
                .flat_map(|row| {
                    row.contributions
                        .iter()
                        .filter_map(|contribution| contribution.graph_epoch.clone())
                })
                .collect::<BTreeSet<_>>()
                .into_iter()
                .collect(),
            partial_index_drill,
            rows: retrieval_packet
                .rows
                .iter()
                .map(|row| {
                    SearchOperatorTruthRow::from_retrieval_row(
                        row,
                        &retrieval_packet.packet_id,
                        "Selected workset",
                        "mixed retrieval freshness",
                    )
                })
                .collect(),
            consumer_projections,
            source_contract_refs,
            generated_at: generated_at.into(),
        }
    }

    /// Re-validates the packet against beta operator-truth invariants.
    pub fn validate(&self) -> Vec<SearchOperatorTruthFinding> {
        self.derived_findings(true)
    }

    /// Returns true when this packet has no blocker-level finding.
    pub fn is_promotable(&self) -> bool {
        !self
            .validate()
            .iter()
            .any(|finding| finding.severity == SearchOperatorTruthFindingSeverity::Blocker)
    }

    /// Returns true when a consumer projection preserves this packet.
    pub fn has_projection_for(&self, surface: SearchOperatorConsumerSurface) -> bool {
        self.consumer_projections.iter().any(|projection| {
            projection.consumer_surface == surface
                && projection.preserves_truth_for(&self.packet_id)
        })
    }

    /// Returns stable ranking-reason tokens present in visible rows.
    pub fn ranking_reason_tokens(&self) -> Vec<&'static str> {
        let mut tokens = BTreeSet::new();
        for row in &self.rows {
            for reason in &row.ranking_reasons {
                tokens.insert(*reason);
            }
        }
        tokens
            .into_iter()
            .map(RankingReasonSignal::as_str)
            .collect()
    }

    /// Builds a support export that embeds the exact product packet.
    pub fn support_export(
        &self,
        export_id: impl Into<String>,
        exported_at: impl Into<String>,
    ) -> SearchOperatorTruthSupportExport {
        SearchOperatorTruthSupportExport {
            record_kind: SEARCH_OPERATOR_TRUTH_SUPPORT_EXPORT_RECORD_KIND.to_owned(),
            schema_version: SEARCH_OPERATOR_TRUTH_SCHEMA_VERSION,
            export_id: export_id.into(),
            operator_truth_packet_id_ref: self.packet_id.clone(),
            exported_at: exported_at.into(),
            raw_private_material_excluded: true,
            ambient_authority_excluded: true,
            operator_truth_packet: self.clone(),
        }
    }

    fn derived_findings(&self, include_record_fields: bool) -> Vec<SearchOperatorTruthFinding> {
        let mut findings = Vec::new();

        if include_record_fields && self.record_kind != SEARCH_OPERATOR_TRUTH_PACKET_RECORD_KIND {
            findings.push(SearchOperatorTruthFinding::new(
                SearchOperatorTruthFindingKind::WrongRecordKind,
                SearchOperatorTruthFindingSeverity::Blocker,
                "search operator-truth packet has the wrong record kind",
            ));
        }
        if include_record_fields && self.schema_version != SEARCH_OPERATOR_TRUTH_SCHEMA_VERSION {
            findings.push(SearchOperatorTruthFinding::new(
                SearchOperatorTruthFindingKind::WrongSchemaVersion,
                SearchOperatorTruthFindingSeverity::Blocker,
                "search operator-truth packet has the wrong schema version",
            ));
        }
        if self.packet_id.trim().is_empty()
            || self.workflow_or_surface_id.trim().is_empty()
            || self.query_session_id_ref.trim().is_empty()
            || self.planner_pass_id_ref.trim().is_empty()
            || self.generated_at.trim().is_empty()
        {
            findings.push(SearchOperatorTruthFinding::new(
                SearchOperatorTruthFindingKind::MissingIdentity,
                SearchOperatorTruthFindingSeverity::Blocker,
                "packet, workflow, query-session, planner-pass, and timestamp refs are required",
            ));
        }

        if !self.partial_index_drill.is_reusable() {
            findings.push(SearchOperatorTruthFinding::new(
                SearchOperatorTruthFindingKind::PartialIndexDrillNotReusable,
                SearchOperatorTruthFindingSeverity::Blocker,
                "partial-index drill must be reusable and metadata-safe",
            ));
        }
        for row in &self.partial_index_drill.rows {
            if !row.is_downgraded_when_required() {
                findings.push(SearchOperatorTruthFinding::new(
                    SearchOperatorTruthFindingKind::PartialIndexRowNotDowngraded,
                    SearchOperatorTruthFindingSeverity::Blocker,
                    format!(
                        "partial-index drill row {} is not visibly downgraded",
                        row.row_id
                    ),
                ));
            }
        }

        if self.rows.is_empty() {
            findings.push(SearchOperatorTruthFinding::new(
                SearchOperatorTruthFindingKind::MissingRankingReason,
                SearchOperatorTruthFindingSeverity::Blocker,
                "operator-truth packets must include at least one row",
            ));
        }
        for row in &self.rows {
            if row.row_id.trim().is_empty()
                || row.rendered_result_ref.trim().is_empty()
                || row.target_ref.trim().is_empty()
                || row.display_title.trim().is_empty()
            {
                findings.push(SearchOperatorTruthFinding::new(
                    SearchOperatorTruthFindingKind::MissingIdentity,
                    SearchOperatorTruthFindingSeverity::Blocker,
                    "row identity, rendered-result ref, target ref, and display title are required",
                ));
            }
            if row.ranking_reasons.is_empty() {
                findings.push(SearchOperatorTruthFinding::new(
                    SearchOperatorTruthFindingKind::MissingRankingReason,
                    SearchOperatorTruthFindingSeverity::Blocker,
                    format!("row {} has no ranking reasons", row.row_id),
                ));
            }
            if row.readiness_label.trim().is_empty() {
                findings.push(SearchOperatorTruthFinding::new(
                    SearchOperatorTruthFindingKind::MissingReadinessLabel,
                    SearchOperatorTruthFindingSeverity::Blocker,
                    format!("row {} has no readiness label", row.row_id),
                ));
            }
            if row.readiness_state != PlannerPathReadiness::Ready
                && row.partial_truth_causes.is_empty()
            {
                findings.push(SearchOperatorTruthFinding::new(
                    SearchOperatorTruthFindingKind::MissingPartialTruthCause,
                    SearchOperatorTruthFindingSeverity::Warning,
                    format!(
                        "row {} is degraded without partial-truth causes",
                        row.row_id
                    ),
                ));
            }
            if row.source_packet_refs.is_empty() {
                findings.push(SearchOperatorTruthFinding::new(
                    SearchOperatorTruthFindingKind::MissingSourcePacketRef,
                    SearchOperatorTruthFindingSeverity::Blocker,
                    format!("row {} has no source packet refs", row.row_id),
                ));
            }
            if !row.private_debug_weights_excluded {
                findings.push(SearchOperatorTruthFinding::new(
                    SearchOperatorTruthFindingKind::RawBoundaryMaterialPresent,
                    SearchOperatorTruthFindingSeverity::Blocker,
                    format!("row {} admits private debug weights", row.row_id),
                ));
            }
            if row.downgrade_state != SearchOperatorDowngradeState::None
                && row.blocked_actions.is_empty()
            {
                findings.push(SearchOperatorTruthFinding::new(
                    SearchOperatorTruthFindingKind::PartialIndexRowNotDowngraded,
                    SearchOperatorTruthFindingSeverity::Blocker,
                    format!("row {} has a downgrade without blocked actions", row.row_id),
                ));
            }
        }

        for required_surface in [
            SearchOperatorConsumerSurface::SearchResults,
            SearchOperatorConsumerSurface::GraphOverlay,
            SearchOperatorConsumerSurface::AiContext,
            SearchOperatorConsumerSurface::ReviewWorkspace,
            SearchOperatorConsumerSurface::SupportExport,
        ] {
            if !self.has_projection_for(required_surface) {
                findings.push(SearchOperatorTruthFinding::new(
                    SearchOperatorTruthFindingKind::MissingConsumerProjection,
                    SearchOperatorTruthFindingSeverity::Blocker,
                    format!(
                        "packet {} is missing a preserved {} projection",
                        self.packet_id,
                        required_surface.as_str()
                    ),
                ));
            }
        }
        for projection in &self.consumer_projections {
            if !projection.preserves_truth_for(&self.packet_id) {
                findings.push(SearchOperatorTruthFinding::new(
                    SearchOperatorTruthFindingKind::ConsumerProjectionDrift,
                    SearchOperatorTruthFindingSeverity::Blocker,
                    format!(
                        "projection {} does not preserve operator truth",
                        projection.projection_ref
                    ),
                ));
            }
        }

        if self.retrieval_packet_id_ref.is_some()
            && (self.retrieval_promotion_state_token.as_deref() != Some("promotable")
                || !self.retrieval_validation_finding_tokens.is_empty())
        {
            findings.push(SearchOperatorTruthFinding::new(
                SearchOperatorTruthFindingKind::RetrievalPacketInvalid,
                SearchOperatorTruthFindingSeverity::Blocker,
                "upstream retrieval packet must be promotable before operator truth can promote",
            ));
        }

        if include_record_fields {
            let mut without_promotion = findings.clone();
            without_promotion.retain(|finding| {
                finding.finding_kind != SearchOperatorTruthFindingKind::PromotionStateMismatch
            });
            let derived_promotion = promotion_state_for_findings(&without_promotion, &self.rows);
            if self.promotion_state != derived_promotion {
                findings.push(SearchOperatorTruthFinding::new(
                    SearchOperatorTruthFindingKind::PromotionStateMismatch,
                    SearchOperatorTruthFindingSeverity::Blocker,
                    "stored promotion state does not match derived validation findings",
                ));
            }
        }

        findings
    }
}

/// Support-export wrapper that preserves the product operator-truth packet verbatim.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SearchOperatorTruthSupportExport {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Stable support export id.
    pub export_id: String,
    /// Operator-truth packet id preserved by the export.
    pub operator_truth_packet_id_ref: String,
    /// Export timestamp.
    pub exported_at: String,
    /// True when raw private material is excluded.
    pub raw_private_material_excluded: bool,
    /// True when ambient credentials or authority are excluded.
    pub ambient_authority_excluded: bool,
    /// Exact operator-truth packet shown by product surfaces.
    pub operator_truth_packet: SearchOperatorTruthPacket,
}

impl SearchOperatorTruthSupportExport {
    /// Returns true when the export preserves the same packet id safely.
    pub fn is_export_safe(&self) -> bool {
        self.record_kind == SEARCH_OPERATOR_TRUTH_SUPPORT_EXPORT_RECORD_KIND
            && self.schema_version == SEARCH_OPERATOR_TRUTH_SCHEMA_VERSION
            && self.operator_truth_packet_id_ref == self.operator_truth_packet.packet_id
            && self.raw_private_material_excluded
            && self.ambient_authority_excluded
            && self.operator_truth_packet.validate().is_empty()
    }
}

/// Errors emitted when reading the checked-in beta operator-truth packet.
#[derive(Debug)]
pub enum SearchOperatorTruthArtifactError {
    /// Packet failed to parse.
    Packet(serde_json::Error),
    /// Packet failed validation.
    Validation(Vec<SearchOperatorTruthFinding>),
}

impl fmt::Display for SearchOperatorTruthArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Packet(error) => write!(
                formatter,
                "search operator-truth packet parse failed: {error}"
            ),
            Self::Validation(findings) => {
                let tokens = findings
                    .iter()
                    .map(|finding| finding.finding_kind.as_str())
                    .collect::<Vec<_>>()
                    .join(",");
                write!(
                    formatter,
                    "search operator-truth packet failed validation: {tokens}"
                )
            }
        }
    }
}

impl Error for SearchOperatorTruthArtifactError {}

/// Returns the checked-in beta operator-truth packet.
///
/// # Errors
///
/// Returns an artifact error if the checked-in packet does not parse or validate.
pub fn current_beta_search_operator_truth_packet(
) -> Result<SearchOperatorTruthPacket, SearchOperatorTruthArtifactError> {
    let packet: SearchOperatorTruthPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/search/m3/operator_truth_packets/search_operator_truth_beta_packet.json"
    )))
    .map_err(SearchOperatorTruthArtifactError::Packet)?;
    let findings = packet.validate();
    if findings.is_empty() {
        Ok(packet)
    } else {
        Err(SearchOperatorTruthArtifactError::Validation(findings))
    }
}

fn ranking_reasons_for_retrieval_row(row: &RetrievalInspectorRow) -> Vec<RankingReasonSignal> {
    let mut reasons = Vec::new();
    for contribution in &row.contributions {
        for reason in &contribution.ranking_reasons {
            let mapped = RankingReasonSignal::from_retrieval_reason(*reason);
            if !reasons.contains(&mapped) {
                reasons.push(mapped);
            }
        }
    }
    if row.readiness.requires_visible_caveat_for_operator_truth()
        && !reasons.contains(&RankingReasonSignal::PartialIndex)
    {
        reasons.push(RankingReasonSignal::PartialIndex);
    }
    reasons
}

fn partial_truth_causes_for_retrieval_row(row: &RetrievalInspectorRow) -> Vec<String> {
    let mut causes = Vec::new();
    for contribution in &row.contributions {
        for cause in &contribution.partial_truth_causes {
            if !causes.contains(cause) {
                causes.push(cause.clone());
            }
        }
    }
    causes
}

fn readiness_from_retrieval(readiness: RetrievalReadinessClass) -> PlannerPathReadiness {
    match readiness {
        RetrievalReadinessClass::Ready => PlannerPathReadiness::Ready,
        RetrievalReadinessClass::HotSetReady => PlannerPathReadiness::HotSetReady,
        RetrievalReadinessClass::Warming => PlannerPathReadiness::Warming,
        RetrievalReadinessClass::Partial => PlannerPathReadiness::Partial,
        RetrievalReadinessClass::Stale => PlannerPathReadiness::Stale,
        RetrievalReadinessClass::Unavailable => PlannerPathReadiness::Unavailable,
        RetrievalReadinessClass::OutOfScope => PlannerPathReadiness::OutOfScope,
    }
}

fn truth_class_from_retrieval_lane(lane: RetrievalLaneClass) -> PlannerResultTruthClass {
    match lane {
        RetrievalLaneClass::Lexical => PlannerResultTruthClass::Exact,
        RetrievalLaneClass::Vector | RetrievalLaneClass::Fused => PlannerResultTruthClass::Hybrid,
        RetrievalLaneClass::Graph => PlannerResultTruthClass::GraphBacked,
    }
}

fn downgrade_for_retrieval(
    readiness: RetrievalReadinessClass,
    partial_truth_causes: &[String],
) -> SearchOperatorDowngradeState {
    if partial_truth_causes
        .iter()
        .any(|cause| cause.contains("stale"))
        || readiness == RetrievalReadinessClass::Stale
    {
        return SearchOperatorDowngradeState::YellowStaleShard;
    }
    if partial_truth_causes
        .iter()
        .any(|cause| cause.contains("hidden") || cause.contains("scope"))
        || readiness == RetrievalReadinessClass::OutOfScope
    {
        return SearchOperatorDowngradeState::YellowHiddenScope;
    }
    match readiness {
        RetrievalReadinessClass::Ready => SearchOperatorDowngradeState::None,
        RetrievalReadinessClass::HotSetReady
        | RetrievalReadinessClass::Warming
        | RetrievalReadinessClass::Partial => SearchOperatorDowngradeState::YellowPartialIndex,
        RetrievalReadinessClass::Stale => SearchOperatorDowngradeState::YellowStaleShard,
        RetrievalReadinessClass::Unavailable => {
            SearchOperatorDowngradeState::RedBlocksBetaPromotion
        }
        RetrievalReadinessClass::OutOfScope => SearchOperatorDowngradeState::YellowHiddenScope,
    }
}

fn promotion_state_for_findings(
    findings: &[SearchOperatorTruthFinding],
    rows: &[SearchOperatorTruthRow],
) -> SearchOperatorPromotionState {
    if findings
        .iter()
        .any(|finding| finding.severity == SearchOperatorTruthFindingSeverity::Blocker)
        || rows.iter().any(|row| row.row_blocks_beta_promotion)
    {
        SearchOperatorPromotionState::Blocked
    } else if findings
        .iter()
        .any(|finding| finding.severity == SearchOperatorTruthFindingSeverity::Warning)
        || rows
            .iter()
            .any(|row| row.downgrade_state != SearchOperatorDowngradeState::None)
    {
        SearchOperatorPromotionState::NeedsReview
    } else {
        SearchOperatorPromotionState::Promotable
    }
}

trait RetrievalReadinessOperatorExt {
    fn requires_visible_caveat_for_operator_truth(self) -> bool;
}

impl RetrievalReadinessOperatorExt for RetrievalReadinessClass {
    fn requires_visible_caveat_for_operator_truth(self) -> bool {
        matches!(
            self,
            Self::HotSetReady | Self::Warming | Self::Partial | Self::Stale | Self::OutOfScope
        )
    }
}
