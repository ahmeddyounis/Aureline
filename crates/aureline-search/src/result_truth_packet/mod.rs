//! Stable result-identity, ranking-reason, action-binding, and export packets
//! for search-result rows.
//!
//! This module is the search-owned contract for the M4 stable lane that
//! ships `SearchResultRef`, `RankingReason`, `SearchActionBinding`, and
//! `SearchResultTruthPacket` objects to every consumer that previously
//! reconstructed search truth from transient row state. The packet is
//! intentionally metadata-only — it carries no raw query text, raw source
//! bodies, provider payloads, secrets, or private numeric rank weights — and
//! binds the durable result-identity refs, ranking-reason vocabulary,
//! action-binding fallback policy, scope counters, and consumer projections
//! that the search shell, docs/help, AI-context inspector, CLI/headless
//! emitter, support export, and the release proof index all read instead of
//! inventing local copies.
//!
//! The vocabulary mirrors the v24 contract:
//!
//! - [`SearchResultRef`] owns the stable result identity: `result_id`,
//!   `result_kind`, canonical object refs, anchor/span, source stratum,
//!   snapshot/commit/worktree ref, freshness, confidence, and dedupe
//!   lineage so open/peek/split/history/export actions remain attributable
//!   even after presentational changes.
//! - [`RankingReason`] resolves to a structured explanation object listing
//!   the closed fact-label (`exact`, `context_promoted`, `semantic`,
//!   `partial_index`, `withheld_latency`, `policy_hidden`), promoted
//!   signals, suppressed signals, tie-break class, withheld-candidate note,
//!   and partiality note. Result rows may keep the default chrome quiet,
//!   but operator/support views inspect the same explanation object.
//! - [`SearchActionBinding`] keeps action binding explicit: open target,
//!   alternate behaviors, required surface capabilities, fallback mode, and
//!   history policy cannot be inferred from whichever UI control launched
//!   the row.
//! - [`SearchResultTruthPacket`] preserves hidden/omitted counts, captured-vs-live
//!   status, and action-binding fallback policy so support and automation
//!   can tell whether the user saw a snapshot, a rerun, or a narrowed scope.

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

/// Stable record-kind tag for [`SearchResultTruthPacket`].
pub const SEARCH_RESULT_TRUTH_PACKET_RECORD_KIND: &str = "search_result_truth_stable_packet";

/// Stable record-kind tag for [`SearchResultTruthPacketSupportExport`].
pub const SEARCH_RESULT_TRUTH_PACKET_SUPPORT_EXPORT_RECORD_KIND: &str =
    "search_result_truth_support_export";

/// Integer schema version for the stable result-truth packet.
pub const SEARCH_RESULT_TRUTH_PACKET_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the boundary schema.
pub const SEARCH_RESULT_TRUTH_PACKET_SCHEMA_REF: &str =
    "schemas/search/search_result_truth_packet.schema.json";

/// Repo-relative path of the reviewer doc.
pub const SEARCH_RESULT_TRUTH_PACKET_DOC_REF: &str =
    "docs/search/m4/result-identity-ranking-reasons-and-export-packets.md";

/// Repo-relative path of the human-readable artifact narrative.
pub const SEARCH_RESULT_TRUTH_PACKET_ARTIFACT_DOC_REF: &str =
    "artifacts/search/m4/result-identity-ranking-reasons-and-export-packets.md";

/// Repo-relative path of the protected fixture corpus directory.
pub const SEARCH_RESULT_TRUTH_PACKET_FIXTURE_DIR: &str = "fixtures/search/m4/result_truth_packet";

/// Repo-relative path of the checked-in stable result-truth packet.
pub const SEARCH_RESULT_TRUTH_PACKET_ARTIFACT_REF: &str =
    "artifacts/search/m4/search_result_truth_packet.json";

/// Closed result-kind vocabulary attached to every [`SearchResultRef`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ResultKindClass {
    /// Workspace-relative file row.
    WorkspaceFile,
    /// Symbol, route, type, or member row.
    Symbol,
    /// Command-registry row.
    Command,
    /// Documentation, help, or citation anchor row.
    DocsAnchor,
    /// Recent target, history, or pinned-place row.
    RecentTarget,
    /// Graph entity or relation row.
    GraphEntity,
    /// Imported or remote-cache row not owned by the local workspace.
    ImportedArtifact,
}

impl ResultKindClass {
    /// Stable token used in fixtures, schemas, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WorkspaceFile => "workspace_file",
            Self::Symbol => "symbol",
            Self::Command => "command",
            Self::DocsAnchor => "docs_anchor",
            Self::RecentTarget => "recent_target",
            Self::GraphEntity => "graph_entity",
            Self::ImportedArtifact => "imported_artifact",
        }
    }
}

/// Closed source-stratum vocabulary for a contributing candidate.
///
/// Multiple strata may contribute to one deduplicated row; the packet
/// preserves every contributing stratum so support and AI consumers can
/// inspect why one visible row may represent multiple candidate sources.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SourceStratumClass {
    /// Lexical filename / basename lane.
    LexicalFilename,
    /// Lexical path lane.
    LexicalPath,
    /// Lexical substring or content lane.
    LexicalContent,
    /// Semantic / vector embedding lane.
    SemanticVector,
    /// Workspace structural / symbol lane.
    StructuralSymbol,
    /// Graph-backed entity lane.
    GraphEntity,
    /// Documentation index lane.
    DocsIndex,
    /// Command registry lane.
    CommandRegistry,
    /// Recents / history lane.
    RecentTargets,
    /// Imported or remote-cache lane.
    ImportedCache,
}

impl SourceStratumClass {
    /// Stable token used in fixtures, schemas, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LexicalFilename => "lexical_filename",
            Self::LexicalPath => "lexical_path",
            Self::LexicalContent => "lexical_content",
            Self::SemanticVector => "semantic_vector",
            Self::StructuralSymbol => "structural_symbol",
            Self::GraphEntity => "graph_entity",
            Self::DocsIndex => "docs_index",
            Self::CommandRegistry => "command_registry",
            Self::RecentTargets => "recent_targets",
            Self::ImportedCache => "imported_cache",
        }
    }
}

/// Closed freshness vocabulary for a result row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FreshnessClass {
    /// Row reflects the live workspace state at session capture time.
    Live,
    /// Row was captured from a snapshot and is being replayed.
    CapturedSnapshot,
    /// Row reflects a hot-set lane while cold paths are still warming.
    HotSetCurrent,
    /// Row was answered by a partial or warming index lane.
    PartialIndex,
    /// Row reflects a known-stale shard, snapshot, or graph slice.
    Stale,
    /// Freshness is unknown because the answering lane could not be polled.
    Unknown,
}

impl FreshnessClass {
    /// Stable token used in fixtures, schemas, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Live => "live",
            Self::CapturedSnapshot => "captured_snapshot",
            Self::HotSetCurrent => "hot_set_current",
            Self::PartialIndex => "partial_index",
            Self::Stale => "stale",
            Self::Unknown => "unknown",
        }
    }

    /// True when this freshness state requires a visible caveat on the row.
    pub const fn requires_visible_caveat(self) -> bool {
        !matches!(self, Self::Live)
    }
}

/// Closed confidence vocabulary for a result row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ConfidenceClass {
    /// Row is a high-confidence direct match.
    High,
    /// Row is a medium-confidence match: semantic similarity, prefix, or graph expansion.
    Medium,
    /// Row is a low-confidence heuristic match.
    Low,
    /// Row carries no confidence claim; the surface MUST NOT promote it as canonical.
    Heuristic,
}

impl ConfidenceClass {
    /// Stable token used in fixtures, schemas, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::High => "high",
            Self::Medium => "medium",
            Self::Low => "low",
            Self::Heuristic => "heuristic",
        }
    }
}

/// Closed fact-label vocabulary for a result row.
///
/// The vocabulary is the v24 promise to all consumers: `Exact`,
/// `ContextPromoted`, `Semantic`, `PartialIndex`, `WithheldLatency`, and
/// `PolicyHidden`. The packet must preserve every label across product,
/// CLI/headless, AI, and support consumers so withheld or blocked states
/// cannot silently drop while a row still renders.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FactLabelClass {
    /// Row is a direct exact match.
    Exact,
    /// Row was promoted by context (recents, pinned, hot-set bias).
    ContextPromoted,
    /// Row came from semantic / vector retrieval.
    Semantic,
    /// Row came from a partial or warming index lane.
    PartialIndex,
    /// Row was withheld because the answering lane exceeded its latency budget.
    WithheldLatency,
    /// Row was withheld or narrowed by trust/policy posture.
    PolicyHidden,
}

impl FactLabelClass {
    /// Every fact-label token, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::Exact,
        Self::ContextPromoted,
        Self::Semantic,
        Self::PartialIndex,
        Self::WithheldLatency,
        Self::PolicyHidden,
    ];

    /// Stable token used in fixtures, schemas, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Exact => "exact",
            Self::ContextPromoted => "context_promoted",
            Self::Semantic => "semantic",
            Self::PartialIndex => "partial_index",
            Self::WithheldLatency => "withheld_latency",
            Self::PolicyHidden => "policy_hidden",
        }
    }

    /// True when this label requires a visible caveat directly on the row.
    pub const fn requires_row_caveat(self) -> bool {
        !matches!(self, Self::Exact | Self::ContextPromoted)
    }
}

/// Closed ranking-signal vocabulary used by [`RankingReason`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RankingSignalClass {
    /// Exact lexical or structural name match.
    ExactNameMatch,
    /// Prefix lexical match.
    LexicalPrefix,
    /// Substring lexical match.
    LexicalSubstring,
    /// Path-only lexical match.
    LexicalPath,
    /// Semantic / vector similarity.
    SemanticVectorSimilarity,
    /// Graph neighborhood expansion.
    GraphExpansion,
    /// Recents / hot-set / pinned signal.
    RecencyOrHotSet,
    /// Generated-artifact deprioritization.
    GeneratedArtifactDeprioritization,
    /// Tie-break by canonical-source preference.
    CanonicalSourceTieBreak,
    /// Tie-break by snapshot/commit/worktree fidelity.
    SnapshotFidelityTieBreak,
}

impl RankingSignalClass {
    /// Stable token used in fixtures, schemas, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ExactNameMatch => "exact_name_match",
            Self::LexicalPrefix => "lexical_prefix",
            Self::LexicalSubstring => "lexical_substring",
            Self::LexicalPath => "lexical_path",
            Self::SemanticVectorSimilarity => "semantic_vector_similarity",
            Self::GraphExpansion => "graph_expansion",
            Self::RecencyOrHotSet => "recency_or_hot_set",
            Self::GeneratedArtifactDeprioritization => "generated_artifact_deprioritization",
            Self::CanonicalSourceTieBreak => "canonical_source_tie_break",
            Self::SnapshotFidelityTieBreak => "snapshot_fidelity_tie_break",
        }
    }
}

/// Closed tie-break class.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TieBreakClass {
    /// No tie-break was required.
    None,
    /// Tie broken by canonical-source preference (e.g., source over generated).
    CanonicalSource,
    /// Tie broken by snapshot/commit/worktree fidelity.
    SnapshotFidelity,
    /// Tie broken by recency.
    Recency,
    /// Tie broken deterministically by stable id.
    StableIdOrder,
}

impl TieBreakClass {
    /// Stable token used in fixtures, schemas, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::None => "none",
            Self::CanonicalSource => "canonical_source",
            Self::SnapshotFidelity => "snapshot_fidelity",
            Self::Recency => "recency",
            Self::StableIdOrder => "stable_id_order",
        }
    }
}

/// Closed fallback-mode vocabulary for [`SearchActionBinding`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ActionFallbackModeClass {
    /// No fallback is required; the primary action is the canonical one.
    Direct,
    /// Action opens a captured-snapshot replay when the live target is unavailable.
    OpenCapturedSnapshot,
    /// Action falls back to the canonical source when the imported/derived row is selected.
    RouteToCanonicalSource,
    /// Action reruns against the live workspace and replaces the snapshot.
    RerunLiveQuery,
    /// Action is narrowed by policy; the binding must explain what is admitted.
    PolicyNarrowed,
    /// Action is unavailable; the binding records the reason and disables the launcher.
    Unavailable,
}

impl ActionFallbackModeClass {
    /// Stable token used in fixtures, schemas, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Direct => "direct",
            Self::OpenCapturedSnapshot => "open_captured_snapshot",
            Self::RouteToCanonicalSource => "route_to_canonical_source",
            Self::RerunLiveQuery => "rerun_live_query",
            Self::PolicyNarrowed => "policy_narrowed",
            Self::Unavailable => "unavailable",
        }
    }
}

/// Closed history-policy vocabulary for [`SearchActionBinding`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum HistoryPolicyClass {
    /// Action records a new history entry.
    RecordHistoryEntry,
    /// Action reuses the existing history entry without forking.
    ReuseExistingEntry,
    /// Action is captured-snapshot replay; do not record a new live entry.
    SuppressForCapturedReplay,
    /// Policy forbids history recording for this surface.
    PolicyForbidsHistory,
}

impl HistoryPolicyClass {
    /// Stable token used in fixtures, schemas, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::RecordHistoryEntry => "record_history_entry",
            Self::ReuseExistingEntry => "reuse_existing_entry",
            Self::SuppressForCapturedReplay => "suppress_for_captured_replay",
            Self::PolicyForbidsHistory => "policy_forbids_history",
        }
    }
}

/// Closed captured-vs-live vocabulary.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CapturedVsLiveClass {
    /// Packet was produced by a live session.
    Live,
    /// Packet was produced by replaying a captured snapshot.
    CapturedSnapshot,
    /// Packet was produced by a rerun that replaced a prior snapshot.
    RerunReplacedSnapshot,
    /// Packet was narrowed below scope and the captured rows are not live.
    NarrowedScopeRerun,
}

impl CapturedVsLiveClass {
    /// Stable token used in fixtures, schemas, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Live => "live",
            Self::CapturedSnapshot => "captured_snapshot",
            Self::RerunReplacedSnapshot => "rerun_replaced_snapshot",
            Self::NarrowedScopeRerun => "narrowed_scope_rerun",
        }
    }
}

/// Closed promotion state for [`SearchResultTruthPacket`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SearchResultTruthPromotionState {
    /// Packet certifies a stable claim.
    Stable,
    /// Packet must remain narrowed below stable until a recorded gap closes.
    NarrowedBelowStable,
    /// Packet has a blocker finding and cannot publish on stable surfaces.
    BlocksStable,
}

impl SearchResultTruthPromotionState {
    /// Stable token used in fixtures, schemas, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Stable => "stable",
            Self::NarrowedBelowStable => "narrowed_below_stable",
            Self::BlocksStable => "blocks_stable",
        }
    }
}

/// Severity for one validation finding.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SearchResultTruthFindingSeverity {
    /// Informational finding.
    Info,
    /// Reviewable finding that narrows the packet below stable.
    Warning,
    /// Blocker that prevents stable publication.
    Blocker,
}

/// Closed validation-finding vocabulary for [`SearchResultTruthPacket`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SearchResultTruthFindingKind {
    /// Record kind does not match the schema.
    WrongRecordKind,
    /// Schema version does not match the frozen schema.
    WrongSchemaVersion,
    /// Required identity field is empty.
    MissingIdentity,
    /// A row has no result ref.
    MissingResultRef,
    /// A row has no ranking reason.
    MissingRankingReason,
    /// A row has no action binding.
    MissingActionBinding,
    /// A row drops one of the contributing source strata after dedup.
    DedupeDroppedSourceStratum,
    /// A row drops its canonical anchor.
    DedupeDroppedCanonicalAnchor,
    /// A row drops its action-binding fallback mode after dedup.
    DedupeDroppedFallbackMode,
    /// A row dropped one of the closed fact-label vocabulary entries while still rendering.
    FactLabelSilentlyDropped,
    /// A required consumer projection is missing.
    MissingConsumerProjection,
    /// A consumer projection remints or drops result truth.
    ConsumerProjectionDrift,
    /// A consumer projection drops the captured-vs-live status.
    CapturedVsLiveDropped,
    /// A consumer projection drops the hidden/omitted scope counts.
    HiddenOmittedCountsDropped,
    /// A row admits raw query text, raw bodies, secrets, or private weights.
    RawBoundaryMaterialPresent,
    /// Stored promotion state disagrees with derived findings.
    PromotionStateMismatch,
    /// Packet drops one of the closed fact-label tokens from its vocabulary cover.
    MissingFactLabelCoverage,
    /// A row with a non-`live` freshness drops its partiality note.
    MissingPartialityNote,
    /// A row with a withheld-candidate state drops its withheld-candidate note.
    MissingWithheldCandidateNote,
}

impl SearchResultTruthFindingKind {
    /// Stable token used in fixtures, schemas, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WrongRecordKind => "wrong_record_kind",
            Self::WrongSchemaVersion => "wrong_schema_version",
            Self::MissingIdentity => "missing_identity",
            Self::MissingResultRef => "missing_result_ref",
            Self::MissingRankingReason => "missing_ranking_reason",
            Self::MissingActionBinding => "missing_action_binding",
            Self::DedupeDroppedSourceStratum => "dedupe_dropped_source_stratum",
            Self::DedupeDroppedCanonicalAnchor => "dedupe_dropped_canonical_anchor",
            Self::DedupeDroppedFallbackMode => "dedupe_dropped_fallback_mode",
            Self::FactLabelSilentlyDropped => "fact_label_silently_dropped",
            Self::MissingConsumerProjection => "missing_consumer_projection",
            Self::ConsumerProjectionDrift => "consumer_projection_drift",
            Self::CapturedVsLiveDropped => "captured_vs_live_dropped",
            Self::HiddenOmittedCountsDropped => "hidden_omitted_counts_dropped",
            Self::RawBoundaryMaterialPresent => "raw_boundary_material_present",
            Self::PromotionStateMismatch => "promotion_state_mismatch",
            Self::MissingFactLabelCoverage => "missing_fact_label_coverage",
            Self::MissingPartialityNote => "missing_partiality_note",
            Self::MissingWithheldCandidateNote => "missing_withheld_candidate_note",
        }
    }
}

/// Consumer surface that must inherit the export packet verbatim.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SearchResultTruthConsumerSurface {
    /// Search shell quick-open, file, symbol, and command-search panes.
    SearchShell,
    /// Docs/help surface explaining ranking and partial-truth labels.
    DocsHelp,
    /// AI context inspector / picker.
    AiContextInspector,
    /// CLI or headless inspection surface.
    CliHeadless,
    /// Support export bundle.
    SupportExport,
    /// Release proof index entry.
    ReleaseProofIndex,
}

impl SearchResultTruthConsumerSurface {
    /// Every required consumer surface, in declaration order.
    pub const REQUIRED: [Self; 6] = [
        Self::SearchShell,
        Self::DocsHelp,
        Self::AiContextInspector,
        Self::CliHeadless,
        Self::SupportExport,
        Self::ReleaseProofIndex,
    ];

    /// Stable token used in fixtures, schemas, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SearchShell => "search_shell",
            Self::DocsHelp => "docs_help",
            Self::AiContextInspector => "ai_context_inspector",
            Self::CliHeadless => "cli_headless",
            Self::SupportExport => "support_export",
            Self::ReleaseProofIndex => "release_proof_index",
        }
    }
}

/// One validation finding emitted by the export-packet validator.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SearchResultTruthValidationFinding {
    /// Closed finding kind.
    pub finding_kind: SearchResultTruthFindingKind,
    /// Finding severity.
    pub severity: SearchResultTruthFindingSeverity,
    /// Short support-safe summary.
    pub summary: String,
}

impl SearchResultTruthValidationFinding {
    fn new(
        finding_kind: SearchResultTruthFindingKind,
        severity: SearchResultTruthFindingSeverity,
        summary: impl Into<String>,
    ) -> Self {
        Self {
            finding_kind,
            severity,
            summary: summary.into(),
        }
    }
}

/// A contributing candidate that participated in producing the deduplicated row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DedupeContributor {
    /// Contributing source stratum.
    pub source_stratum: SourceStratumClass,
    /// Canonical-anchor ref carried from this contributor.
    pub canonical_anchor_ref: String,
    /// Optional contributor result-id (when the stratum minted one).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub contributor_result_id: Option<String>,
}

impl DedupeContributor {
    fn is_valid(&self) -> bool {
        !self.canonical_anchor_ref.trim().is_empty()
    }
}

/// Stable result identity attached to every row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SearchResultRef {
    /// Stable result id (URN-style, deterministic across presentational changes).
    pub result_id: String,
    /// Result kind.
    pub result_kind: ResultKindClass,
    /// Canonical object refs (file, symbol, command, anchor) joined by this row.
    pub canonical_object_refs: Vec<String>,
    /// Anchor / span ref preserved across open/peek/split/history/export.
    pub anchor_or_span_ref: String,
    /// Snapshot / commit / worktree ref pinning the row to a fixed state.
    pub snapshot_or_commit_ref: String,
    /// Freshness class for the row.
    pub freshness: FreshnessClass,
    /// Confidence class for the row.
    pub confidence: ConfidenceClass,
    /// Source strata that contributed to this dedup row.
    #[serde(default)]
    pub dedupe_lineage: Vec<DedupeContributor>,
}

impl SearchResultRef {
    /// Returns true when every contributing source stratum still carries a canonical anchor.
    pub fn dedupe_lineage_is_complete(&self) -> bool {
        !self.dedupe_lineage.is_empty()
            && self.dedupe_lineage.iter().all(DedupeContributor::is_valid)
    }

    /// Returns the unique source-stratum tokens contributing to this row.
    pub fn contributing_stratum_tokens(&self) -> Vec<&'static str> {
        let mut set = BTreeSet::new();
        for contributor in &self.dedupe_lineage {
            set.insert(contributor.source_stratum);
        }
        set.into_iter().map(SourceStratumClass::as_str).collect()
    }
}

/// Structured ranking-reason explanation.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RankingReason {
    /// Fact label resolved for the row.
    pub fact_label: FactLabelClass,
    /// Ranking signals that promoted the row.
    #[serde(default)]
    pub promoted_signals: Vec<RankingSignalClass>,
    /// Ranking signals that were suppressed (e.g., generated-artifact deprioritization).
    #[serde(default)]
    pub suppressed_signals: Vec<RankingSignalClass>,
    /// Tie-break class applied when ranking the row.
    pub tie_break_class: TieBreakClass,
    /// Short note describing any withheld candidates the row collapses.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub withheld_candidate_note: Option<String>,
    /// Short note describing the row's partiality / freshness caveat.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub partiality_note: Option<String>,
}

impl RankingReason {
    /// Returns the unique signal tokens carried by the row.
    pub fn signal_tokens(&self) -> Vec<&'static str> {
        let mut set = BTreeSet::new();
        for signal in self
            .promoted_signals
            .iter()
            .chain(self.suppressed_signals.iter())
        {
            set.insert(*signal);
        }
        set.into_iter().map(RankingSignalClass::as_str).collect()
    }
}

/// Action binding pinned to a row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SearchActionBinding {
    /// Open-target ref this action launches.
    pub open_target_ref: String,
    /// Alternate behaviors (peek, split, reveal, share) admitted for this row.
    #[serde(default)]
    pub alternate_behaviors: Vec<String>,
    /// Surface capabilities required to launch the row.
    #[serde(default)]
    pub required_surface_capabilities: Vec<String>,
    /// Fallback mode for the row when the live target is unavailable.
    pub fallback_mode: ActionFallbackModeClass,
    /// History policy for the row.
    pub history_policy: HistoryPolicyClass,
}

impl SearchActionBinding {
    fn is_valid(&self) -> bool {
        !self.open_target_ref.trim().is_empty()
    }
}

/// One row in the export packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SearchResultTruthRow {
    /// Stable row id inside the packet.
    pub row_id: String,
    /// Stable result identity.
    pub result_ref: SearchResultRef,
    /// Structured ranking explanation.
    pub ranking_reason: RankingReason,
    /// Action binding pinned to the row.
    pub action_binding: SearchActionBinding,
    /// Display title preserved verbatim in product copy.
    pub display_title: String,
    /// True when raw query text, source bodies, secrets, and weights are excluded.
    pub raw_boundary_material_excluded: bool,
    /// Capture timestamp for this row.
    pub captured_at: String,
}

/// Scope counters carried on the export packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ScopeCounters {
    /// Rows currently visible in the result set.
    pub visible_rows: u64,
    /// Rows loaded by the active scope before viewport / group truncation.
    pub loaded_rows: u64,
    /// Rows the same query would match in the full workspace, when known.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub all_matching_rows: Option<u64>,
    /// Rows hidden by the active workset / slice / scope.
    pub hidden_by_current_scope_rows: u64,
    /// Rows hidden or blocked by policy.
    pub hidden_by_policy_rows: u64,
    /// Rows known only through a remote-cache boundary.
    pub hidden_by_remote_cache_rows: u64,
    /// Rows withheld because their answering lane exceeded its latency budget.
    pub omitted_by_latency_budget_rows: u64,
}

impl ScopeCounters {
    /// True when one or more rows are known to be hidden / omitted.
    pub const fn has_hidden_or_omitted_rows(&self) -> bool {
        self.hidden_by_current_scope_rows > 0
            || self.hidden_by_policy_rows > 0
            || self.hidden_by_remote_cache_rows > 0
            || self.omitted_by_latency_budget_rows > 0
    }
}

/// Consumer projection proving a surface reads the same export packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SearchResultTruthConsumerProjection {
    /// Consumer surface class.
    pub consumer_surface: SearchResultTruthConsumerSurface,
    /// Stable projection ref.
    pub projection_ref: String,
    /// Export packet id consumed by the projection.
    pub export_packet_id_ref: String,
    /// Render timestamp.
    pub rendered_at: String,
    /// True when the surface preserves the same packet id.
    pub preserves_same_packet: bool,
    /// True when the surface preserves the result_id and dedupe lineage.
    pub preserves_result_refs: bool,
    /// True when the surface preserves the full closed ranking-reason vocabulary.
    pub preserves_ranking_reasons: bool,
    /// True when the surface preserves the action-binding fallback mode and history policy.
    pub preserves_action_binding: bool,
    /// True when the surface preserves the captured-vs-live status verbatim.
    pub preserves_captured_vs_live: bool,
    /// True when the surface preserves the hidden / omitted counts verbatim.
    pub preserves_hidden_omitted_counts: bool,
    /// True when JSON export is available from the projection.
    pub supports_json_export: bool,
    /// True when raw private material is excluded.
    pub raw_private_material_excluded: bool,
    /// True when ambient authority/credentials are excluded.
    pub ambient_authority_excluded: bool,
}

impl SearchResultTruthConsumerProjection {
    fn preserves_truth_for(&self, packet_id: &str) -> bool {
        self.export_packet_id_ref == packet_id
            && self.preserves_same_packet
            && self.preserves_result_refs
            && self.preserves_ranking_reasons
            && self.preserves_action_binding
            && self.preserves_captured_vs_live
            && self.preserves_hidden_omitted_counts
            && self.supports_json_export
            && self.raw_private_material_excluded
            && self.ambient_authority_excluded
            && !self.projection_ref.trim().is_empty()
    }
}

/// Constructor input for [`SearchResultTruthPacket::materialize`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SearchResultTruthPacketInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Claimed workflow or surface id.
    pub workflow_or_surface_id: String,
    /// Query session that produced the rows.
    pub query_session_id_ref: String,
    /// Capture timestamp for the packet as a whole.
    pub generated_at: String,
    /// Captured-vs-live status.
    pub captured_vs_live: CapturedVsLiveClass,
    /// Scope counters carried on the packet.
    pub scope_counters: ScopeCounters,
    /// Fact labels covered by the packet (must include every label
    /// represented by a row in `rows`).
    #[serde(default)]
    pub covered_fact_labels: Vec<FactLabelClass>,
    /// Result rows.
    #[serde(default)]
    pub rows: Vec<SearchResultTruthRow>,
    /// Consumer projections preserving this packet.
    #[serde(default)]
    pub consumer_projections: Vec<SearchResultTruthConsumerProjection>,
    /// Source contract refs (docs / schema / fixtures) consumed by the packet.
    #[serde(default)]
    pub source_contract_refs: Vec<String>,
}

/// Search-owned packet for result identity, ranking reasons, action binding,
/// scope counters, and consumer projections.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SearchResultTruthPacket {
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
    /// Packet capture timestamp.
    pub generated_at: String,
    /// Captured-vs-live status.
    pub captured_vs_live: CapturedVsLiveClass,
    /// Scope counters carried on the packet.
    pub scope_counters: ScopeCounters,
    /// Fact labels covered by the packet.
    #[serde(default)]
    pub covered_fact_labels: Vec<FactLabelClass>,
    /// Result rows.
    #[serde(default)]
    pub rows: Vec<SearchResultTruthRow>,
    /// Consumer projections preserving this packet.
    #[serde(default)]
    pub consumer_projections: Vec<SearchResultTruthConsumerProjection>,
    /// Source contract refs consumed by the packet.
    #[serde(default)]
    pub source_contract_refs: Vec<String>,
    /// Derived promotion state.
    pub promotion_state: SearchResultTruthPromotionState,
    /// Validation findings captured at materialization.
    #[serde(default)]
    pub validation_findings: Vec<SearchResultTruthValidationFinding>,
}

impl SearchResultTruthPacket {
    /// Materialize a packet and record derived validation findings.
    pub fn materialize(input: SearchResultTruthPacketInput) -> Self {
        let mut packet = Self {
            record_kind: SEARCH_RESULT_TRUTH_PACKET_RECORD_KIND.to_owned(),
            schema_version: SEARCH_RESULT_TRUTH_PACKET_SCHEMA_VERSION,
            packet_id: input.packet_id,
            workflow_or_surface_id: input.workflow_or_surface_id,
            query_session_id_ref: input.query_session_id_ref,
            generated_at: input.generated_at,
            captured_vs_live: input.captured_vs_live,
            scope_counters: input.scope_counters,
            covered_fact_labels: input.covered_fact_labels,
            rows: input.rows,
            consumer_projections: input.consumer_projections,
            source_contract_refs: input.source_contract_refs,
            promotion_state: SearchResultTruthPromotionState::Stable,
            validation_findings: Vec::new(),
        };
        let findings = packet.derived_findings(false);
        packet.promotion_state = promotion_state_for_findings(&findings);
        packet.validation_findings = findings;
        packet
    }

    /// Re-validate the packet against stable result-truth invariants.
    pub fn validate(&self) -> Vec<SearchResultTruthValidationFinding> {
        self.derived_findings(true)
    }

    /// True when this packet has no blocker-level finding.
    pub fn is_stable(&self) -> bool {
        !self
            .validate()
            .iter()
            .any(|finding| finding.severity == SearchResultTruthFindingSeverity::Blocker)
    }

    /// Returns true when a consumer projection preserves this packet.
    pub fn has_projection_for(&self, surface: SearchResultTruthConsumerSurface) -> bool {
        self.consumer_projections.iter().any(|projection| {
            projection.consumer_surface == surface
                && projection.preserves_truth_for(&self.packet_id)
        })
    }

    /// Returns the unique fact-label tokens carried across rows.
    pub fn fact_label_tokens(&self) -> Vec<&'static str> {
        let mut set = BTreeSet::new();
        for row in &self.rows {
            set.insert(row.ranking_reason.fact_label);
        }
        set.into_iter().map(FactLabelClass::as_str).collect()
    }

    /// Returns the unique result-kind tokens carried across rows.
    pub fn result_kind_tokens(&self) -> Vec<&'static str> {
        let mut set = BTreeSet::new();
        for row in &self.rows {
            set.insert(row.result_ref.result_kind);
        }
        set.into_iter().map(ResultKindClass::as_str).collect()
    }

    /// Returns the unique source-stratum tokens carried across all row dedup lineages.
    pub fn contributing_stratum_tokens(&self) -> Vec<&'static str> {
        let mut set = BTreeSet::new();
        for row in &self.rows {
            for contributor in &row.result_ref.dedupe_lineage {
                set.insert(contributor.source_stratum);
            }
        }
        set.into_iter().map(SourceStratumClass::as_str).collect()
    }

    /// Build a support export wrapping the exact product packet.
    pub fn support_export(
        &self,
        export_id: impl Into<String>,
        exported_at: impl Into<String>,
    ) -> SearchResultTruthPacketSupportExport {
        SearchResultTruthPacketSupportExport {
            record_kind: SEARCH_RESULT_TRUTH_PACKET_SUPPORT_EXPORT_RECORD_KIND.to_owned(),
            schema_version: SEARCH_RESULT_TRUTH_PACKET_SCHEMA_VERSION,
            export_id: export_id.into(),
            export_packet_id_ref: self.packet_id.clone(),
            exported_at: exported_at.into(),
            raw_private_material_excluded: true,
            ambient_authority_excluded: true,
            export_packet: self.clone(),
        }
    }

    fn derived_findings(
        &self,
        include_record_fields: bool,
    ) -> Vec<SearchResultTruthValidationFinding> {
        let mut findings = Vec::new();

        if include_record_fields && self.record_kind != SEARCH_RESULT_TRUTH_PACKET_RECORD_KIND {
            findings.push(SearchResultTruthValidationFinding::new(
                SearchResultTruthFindingKind::WrongRecordKind,
                SearchResultTruthFindingSeverity::Blocker,
                "search result-truth packet has the wrong record kind",
            ));
        }
        if include_record_fields && self.schema_version != SEARCH_RESULT_TRUTH_PACKET_SCHEMA_VERSION
        {
            findings.push(SearchResultTruthValidationFinding::new(
                SearchResultTruthFindingKind::WrongSchemaVersion,
                SearchResultTruthFindingSeverity::Blocker,
                "search result-truth packet has the wrong schema version",
            ));
        }
        if self.packet_id.trim().is_empty()
            || self.workflow_or_surface_id.trim().is_empty()
            || self.query_session_id_ref.trim().is_empty()
            || self.generated_at.trim().is_empty()
        {
            findings.push(SearchResultTruthValidationFinding::new(
                SearchResultTruthFindingKind::MissingIdentity,
                SearchResultTruthFindingSeverity::Blocker,
                "packet, workflow, session, and timestamp refs are required",
            ));
        }

        if self.rows.is_empty() {
            findings.push(SearchResultTruthValidationFinding::new(
                SearchResultTruthFindingKind::MissingResultRef,
                SearchResultTruthFindingSeverity::Blocker,
                "packet must include at least one result row",
            ));
        }

        let row_label_set: BTreeSet<FactLabelClass> = self
            .rows
            .iter()
            .map(|row| row.ranking_reason.fact_label)
            .collect();
        let covered_label_set: BTreeSet<FactLabelClass> =
            self.covered_fact_labels.iter().copied().collect();
        for label in &row_label_set {
            if !covered_label_set.contains(label) {
                findings.push(SearchResultTruthValidationFinding::new(
                    SearchResultTruthFindingKind::FactLabelSilentlyDropped,
                    SearchResultTruthFindingSeverity::Blocker,
                    format!(
                        "row carries fact label {} but packet covered_fact_labels drops it",
                        label.as_str()
                    ),
                ));
            }
        }
        for label in &covered_label_set {
            if !row_label_set.contains(label) {
                findings.push(SearchResultTruthValidationFinding::new(
                    SearchResultTruthFindingKind::MissingFactLabelCoverage,
                    SearchResultTruthFindingSeverity::Blocker,
                    format!(
                        "packet declares fact label {} in coverage but no row carries it",
                        label.as_str()
                    ),
                ));
            }
        }

        for row in &self.rows {
            if row.row_id.trim().is_empty()
                || row.display_title.trim().is_empty()
                || row.captured_at.trim().is_empty()
            {
                findings.push(SearchResultTruthValidationFinding::new(
                    SearchResultTruthFindingKind::MissingIdentity,
                    SearchResultTruthFindingSeverity::Blocker,
                    format!(
                        "row {} identity, display title, or capture timestamp is empty",
                        row.row_id
                    ),
                ));
            }
            if row.result_ref.result_id.trim().is_empty()
                || row.result_ref.anchor_or_span_ref.trim().is_empty()
                || row.result_ref.snapshot_or_commit_ref.trim().is_empty()
                || row.result_ref.canonical_object_refs.is_empty()
            {
                findings.push(SearchResultTruthValidationFinding::new(
                    SearchResultTruthFindingKind::MissingResultRef,
                    SearchResultTruthFindingSeverity::Blocker,
                    format!(
                        "row {} result-ref is missing id, anchor, snapshot, or canonical refs",
                        row.row_id
                    ),
                ));
            }
            if !row.result_ref.dedupe_lineage_is_complete() {
                findings.push(SearchResultTruthValidationFinding::new(
                    SearchResultTruthFindingKind::DedupeDroppedSourceStratum,
                    SearchResultTruthFindingSeverity::Blocker,
                    format!(
                        "row {} drops a contributing source stratum or canonical anchor after dedup",
                        row.row_id
                    ),
                ));
            }
            if row
                .result_ref
                .dedupe_lineage
                .iter()
                .any(|contributor| contributor.canonical_anchor_ref.trim().is_empty())
            {
                findings.push(SearchResultTruthValidationFinding::new(
                    SearchResultTruthFindingKind::DedupeDroppedCanonicalAnchor,
                    SearchResultTruthFindingSeverity::Blocker,
                    format!(
                        "row {} has a dedup contributor with no canonical anchor",
                        row.row_id
                    ),
                ));
            }
            if row.ranking_reason.promoted_signals.is_empty()
                && row.ranking_reason.suppressed_signals.is_empty()
            {
                findings.push(SearchResultTruthValidationFinding::new(
                    SearchResultTruthFindingKind::MissingRankingReason,
                    SearchResultTruthFindingSeverity::Blocker,
                    format!(
                        "row {} has no promoted or suppressed ranking signals",
                        row.row_id
                    ),
                ));
            }
            if row.ranking_reason.fact_label.requires_row_caveat()
                && row.ranking_reason.partiality_note.is_none()
            {
                findings.push(SearchResultTruthValidationFinding::new(
                    SearchResultTruthFindingKind::MissingPartialityNote,
                    SearchResultTruthFindingSeverity::Blocker,
                    format!(
                        "row {} has fact label {} but no partiality note",
                        row.row_id,
                        row.ranking_reason.fact_label.as_str()
                    ),
                ));
            }
            if matches!(
                row.ranking_reason.fact_label,
                FactLabelClass::WithheldLatency | FactLabelClass::PolicyHidden
            ) && row.ranking_reason.withheld_candidate_note.is_none()
            {
                findings.push(SearchResultTruthValidationFinding::new(
                    SearchResultTruthFindingKind::MissingWithheldCandidateNote,
                    SearchResultTruthFindingSeverity::Blocker,
                    format!(
                        "row {} is withheld ({}) without a withheld-candidate note",
                        row.row_id,
                        row.ranking_reason.fact_label.as_str()
                    ),
                ));
            }
            if !row.action_binding.is_valid() {
                findings.push(SearchResultTruthValidationFinding::new(
                    SearchResultTruthFindingKind::MissingActionBinding,
                    SearchResultTruthFindingSeverity::Blocker,
                    format!("row {} action binding has no open target ref", row.row_id),
                ));
            }
            if matches!(
                row.action_binding.fallback_mode,
                ActionFallbackModeClass::Direct
            ) && matches!(
                row.ranking_reason.fact_label,
                FactLabelClass::WithheldLatency | FactLabelClass::PolicyHidden
            ) {
                findings.push(SearchResultTruthValidationFinding::new(
                    SearchResultTruthFindingKind::DedupeDroppedFallbackMode,
                    SearchResultTruthFindingSeverity::Blocker,
                    format!(
                        "row {} is withheld ({}) but its action-binding fallback mode collapses to direct",
                        row.row_id,
                        row.ranking_reason.fact_label.as_str()
                    ),
                ));
            }
            if !row.raw_boundary_material_excluded {
                findings.push(SearchResultTruthValidationFinding::new(
                    SearchResultTruthFindingKind::RawBoundaryMaterialPresent,
                    SearchResultTruthFindingSeverity::Blocker,
                    format!(
                        "row {} admits raw query text, source bodies, or private weights",
                        row.row_id
                    ),
                ));
            }
        }

        for required_surface in SearchResultTruthConsumerSurface::REQUIRED {
            if !self.has_projection_for(required_surface) {
                findings.push(SearchResultTruthValidationFinding::new(
                    SearchResultTruthFindingKind::MissingConsumerProjection,
                    SearchResultTruthFindingSeverity::Blocker,
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
                findings.push(SearchResultTruthValidationFinding::new(
                    SearchResultTruthFindingKind::ConsumerProjectionDrift,
                    SearchResultTruthFindingSeverity::Blocker,
                    format!(
                        "projection {} does not preserve search result truth",
                        projection.projection_ref
                    ),
                ));
            }
            if !projection.preserves_captured_vs_live {
                findings.push(SearchResultTruthValidationFinding::new(
                    SearchResultTruthFindingKind::CapturedVsLiveDropped,
                    SearchResultTruthFindingSeverity::Blocker,
                    format!(
                        "projection {} drops captured-vs-live status",
                        projection.projection_ref
                    ),
                ));
            }
            if !projection.preserves_hidden_omitted_counts {
                findings.push(SearchResultTruthValidationFinding::new(
                    SearchResultTruthFindingKind::HiddenOmittedCountsDropped,
                    SearchResultTruthFindingSeverity::Blocker,
                    format!(
                        "projection {} drops hidden/omitted scope counts",
                        projection.projection_ref
                    ),
                ));
            }
        }

        if include_record_fields {
            let mut without_promotion = findings.clone();
            without_promotion.retain(|finding| {
                finding.finding_kind != SearchResultTruthFindingKind::PromotionStateMismatch
            });
            let derived = promotion_state_for_findings(&without_promotion);
            if self.promotion_state != derived {
                findings.push(SearchResultTruthValidationFinding::new(
                    SearchResultTruthFindingKind::PromotionStateMismatch,
                    SearchResultTruthFindingSeverity::Blocker,
                    "stored promotion state does not match derived findings",
                ));
            }
        }

        findings
    }
}

fn promotion_state_for_findings(
    findings: &[SearchResultTruthValidationFinding],
) -> SearchResultTruthPromotionState {
    if findings
        .iter()
        .any(|finding| finding.severity == SearchResultTruthFindingSeverity::Blocker)
    {
        SearchResultTruthPromotionState::BlocksStable
    } else if findings
        .iter()
        .any(|finding| finding.severity == SearchResultTruthFindingSeverity::Warning)
    {
        SearchResultTruthPromotionState::NarrowedBelowStable
    } else {
        SearchResultTruthPromotionState::Stable
    }
}

/// Support-export wrapper that preserves the product export packet verbatim.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SearchResultTruthPacketSupportExport {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Stable export id.
    pub export_id: String,
    /// Export packet id preserved by the export.
    pub export_packet_id_ref: String,
    /// Export timestamp.
    pub exported_at: String,
    /// True when raw private material is excluded.
    pub raw_private_material_excluded: bool,
    /// True when ambient credentials/authority are excluded.
    pub ambient_authority_excluded: bool,
    /// Exact product packet preserved by the export.
    pub export_packet: SearchResultTruthPacket,
}

impl SearchResultTruthPacketSupportExport {
    /// True when the export preserves the same packet id safely.
    pub fn is_export_safe(&self) -> bool {
        self.record_kind == SEARCH_RESULT_TRUTH_PACKET_SUPPORT_EXPORT_RECORD_KIND
            && self.schema_version == SEARCH_RESULT_TRUTH_PACKET_SCHEMA_VERSION
            && self.export_packet_id_ref == self.export_packet.packet_id
            && self.raw_private_material_excluded
            && self.ambient_authority_excluded
            && self.export_packet.validate().is_empty()
    }
}

/// Errors emitted when reading the checked-in stable result-truth packet.
#[derive(Debug)]
pub enum SearchResultTruthArtifactError {
    /// Packet failed to parse.
    Packet(serde_json::Error),
    /// Packet failed validation.
    Validation(Vec<SearchResultTruthValidationFinding>),
}

impl fmt::Display for SearchResultTruthArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Packet(error) => {
                write!(
                    formatter,
                    "search result-truth packet parse failed: {error}"
                )
            }
            Self::Validation(findings) => {
                let tokens = findings
                    .iter()
                    .map(|finding| finding.finding_kind.as_str())
                    .collect::<Vec<_>>()
                    .join(",");
                write!(
                    formatter,
                    "search result-truth packet failed validation: {tokens}"
                )
            }
        }
    }
}

impl Error for SearchResultTruthArtifactError {}

/// Returns the checked-in stable result-truth packet.
///
/// # Errors
///
/// Returns an artifact error if the checked-in packet does not parse or validate.
pub fn current_stable_search_result_truth_packet(
) -> Result<SearchResultTruthPacket, SearchResultTruthArtifactError> {
    let packet: SearchResultTruthPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/search/m4/search_result_truth_packet.json"
    )))
    .map_err(SearchResultTruthArtifactError::Packet)?;
    let findings = packet.validate();
    if findings.is_empty() {
        Ok(packet)
    } else {
        Err(SearchResultTruthArtifactError::Validation(findings))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_result_ref(label: FactLabelClass) -> SearchResultRef {
        SearchResultRef {
            result_id: format!("search:result:test:{}", label.as_str()),
            result_kind: ResultKindClass::WorkspaceFile,
            canonical_object_refs: vec!["file:/src/main.rs".to_owned()],
            anchor_or_span_ref: "anchor:/src/main.rs#L1".to_owned(),
            snapshot_or_commit_ref: "snapshot:test-sha".to_owned(),
            freshness: FreshnessClass::Live,
            confidence: ConfidenceClass::High,
            dedupe_lineage: vec![DedupeContributor {
                source_stratum: SourceStratumClass::LexicalFilename,
                canonical_anchor_ref: "anchor:/src/main.rs#L1".to_owned(),
                contributor_result_id: Some("wsearch:test:lexical_filename:src/main.rs".to_owned()),
            }],
        }
    }

    fn sample_ranking_reason(label: FactLabelClass) -> RankingReason {
        RankingReason {
            fact_label: label,
            promoted_signals: vec![RankingSignalClass::ExactNameMatch],
            suppressed_signals: vec![],
            tie_break_class: TieBreakClass::None,
            withheld_candidate_note: match label {
                FactLabelClass::WithheldLatency | FactLabelClass::PolicyHidden => {
                    Some("disclosed".to_owned())
                }
                _ => None,
            },
            partiality_note: if label.requires_row_caveat() {
                Some("partial-truth disclosed".to_owned())
            } else {
                None
            },
        }
    }

    fn sample_action_binding(fallback: ActionFallbackModeClass) -> SearchActionBinding {
        SearchActionBinding {
            open_target_ref: "open:/src/main.rs".to_owned(),
            alternate_behaviors: vec!["peek".to_owned(), "split".to_owned()],
            required_surface_capabilities: vec!["open_target".to_owned()],
            fallback_mode: fallback,
            history_policy: HistoryPolicyClass::RecordHistoryEntry,
        }
    }

    fn sample_row(label: FactLabelClass) -> SearchResultTruthRow {
        let fallback = match label {
            FactLabelClass::WithheldLatency => ActionFallbackModeClass::OpenCapturedSnapshot,
            FactLabelClass::PolicyHidden => ActionFallbackModeClass::PolicyNarrowed,
            _ => ActionFallbackModeClass::Direct,
        };
        SearchResultTruthRow {
            row_id: format!("row:{}", label.as_str()),
            result_ref: sample_result_ref(label),
            ranking_reason: sample_ranking_reason(label),
            action_binding: sample_action_binding(fallback),
            display_title: format!("Sample row {}", label.as_str()),
            raw_boundary_material_excluded: true,
            captured_at: "2026-05-26T12:00:00Z".to_owned(),
        }
    }

    fn sample_projection(
        surface: SearchResultTruthConsumerSurface,
        packet_id: &str,
    ) -> SearchResultTruthConsumerProjection {
        SearchResultTruthConsumerProjection {
            consumer_surface: surface,
            projection_ref: format!("projection:{}", surface.as_str()),
            export_packet_id_ref: packet_id.to_owned(),
            rendered_at: "2026-05-26T12:00:01Z".to_owned(),
            preserves_same_packet: true,
            preserves_result_refs: true,
            preserves_ranking_reasons: true,
            preserves_action_binding: true,
            preserves_captured_vs_live: true,
            preserves_hidden_omitted_counts: true,
            supports_json_export: true,
            raw_private_material_excluded: true,
            ambient_authority_excluded: true,
        }
    }

    fn sample_scope_counters() -> ScopeCounters {
        ScopeCounters {
            visible_rows: 1,
            loaded_rows: 1,
            all_matching_rows: Some(1),
            hidden_by_current_scope_rows: 0,
            hidden_by_policy_rows: 0,
            hidden_by_remote_cache_rows: 0,
            omitted_by_latency_budget_rows: 0,
        }
    }

    fn baseline_input(packet_id: &str) -> SearchResultTruthPacketInput {
        SearchResultTruthPacketInput {
            packet_id: packet_id.to_owned(),
            workflow_or_surface_id: "workflow.search.result_truth.baseline".to_owned(),
            query_session_id_ref: "search:session:m4:result_truth".to_owned(),
            generated_at: "2026-05-26T12:00:00Z".to_owned(),
            captured_vs_live: CapturedVsLiveClass::Live,
            scope_counters: sample_scope_counters(),
            covered_fact_labels: vec![FactLabelClass::Exact],
            rows: vec![sample_row(FactLabelClass::Exact)],
            consumer_projections: SearchResultTruthConsumerSurface::REQUIRED
                .iter()
                .copied()
                .map(|surface| sample_projection(surface, packet_id))
                .collect(),
            source_contract_refs: vec![SEARCH_RESULT_TRUTH_PACKET_DOC_REF.to_owned()],
        }
    }

    #[test]
    fn closed_tokens_are_pinned() {
        assert_eq!(FactLabelClass::Exact.as_str(), "exact");
        assert_eq!(FactLabelClass::ContextPromoted.as_str(), "context_promoted");
        assert_eq!(FactLabelClass::Semantic.as_str(), "semantic");
        assert_eq!(FactLabelClass::PartialIndex.as_str(), "partial_index");
        assert_eq!(FactLabelClass::WithheldLatency.as_str(), "withheld_latency");
        assert_eq!(FactLabelClass::PolicyHidden.as_str(), "policy_hidden");
        assert_eq!(
            ActionFallbackModeClass::PolicyNarrowed.as_str(),
            "policy_narrowed"
        );
        assert_eq!(
            HistoryPolicyClass::SuppressForCapturedReplay.as_str(),
            "suppress_for_captured_replay"
        );
        assert_eq!(
            CapturedVsLiveClass::CapturedSnapshot.as_str(),
            "captured_snapshot"
        );
    }

    #[test]
    fn baseline_packet_certifies_stable() {
        let packet =
            SearchResultTruthPacket::materialize(baseline_input("packet:m4:result_truth:baseline"));
        assert_eq!(
            packet.promotion_state,
            SearchResultTruthPromotionState::Stable
        );
        assert!(packet.validation_findings.is_empty());
        assert_eq!(packet.fact_label_tokens(), vec!["exact"]);
    }

    #[test]
    fn missing_consumer_projection_blocks_stable() {
        let mut input = baseline_input("packet:m4:result_truth:missing_projection");
        input.consumer_projections.pop();
        let packet = SearchResultTruthPacket::materialize(input);
        assert_eq!(
            packet.promotion_state,
            SearchResultTruthPromotionState::BlocksStable
        );
        assert!(packet
            .validation_findings
            .iter()
            .any(|finding| finding.finding_kind
                == SearchResultTruthFindingKind::MissingConsumerProjection));
    }

    #[test]
    fn withheld_row_with_direct_fallback_blocks_stable() {
        let mut input = baseline_input("packet:m4:result_truth:withheld_drift");
        input.covered_fact_labels = vec![FactLabelClass::Exact, FactLabelClass::WithheldLatency];
        let mut withheld_row = sample_row(FactLabelClass::WithheldLatency);
        withheld_row.action_binding.fallback_mode = ActionFallbackModeClass::Direct;
        input.rows.push(withheld_row);
        let packet = SearchResultTruthPacket::materialize(input);
        assert_eq!(
            packet.promotion_state,
            SearchResultTruthPromotionState::BlocksStable
        );
        assert!(packet
            .validation_findings
            .iter()
            .any(|finding| finding.finding_kind
                == SearchResultTruthFindingKind::DedupeDroppedFallbackMode));
    }

    #[test]
    fn dropping_contributor_anchor_blocks_stable() {
        let mut input = baseline_input("packet:m4:result_truth:anchor_drift");
        if let Some(row) = input.rows.first_mut() {
            row.result_ref.dedupe_lineage[0]
                .canonical_anchor_ref
                .clear();
        }
        let packet = SearchResultTruthPacket::materialize(input);
        assert_eq!(
            packet.promotion_state,
            SearchResultTruthPromotionState::BlocksStable
        );
        assert!(packet
            .validation_findings
            .iter()
            .any(|finding| finding.finding_kind
                == SearchResultTruthFindingKind::DedupeDroppedSourceStratum
                || finding.finding_kind
                    == SearchResultTruthFindingKind::DedupeDroppedCanonicalAnchor));
    }

    #[test]
    fn projection_drops_captured_vs_live_blocks_stable() {
        let packet_id = "packet:m4:result_truth:captured_drift";
        let mut input = baseline_input(packet_id);
        if let Some(projection) = input.consumer_projections.iter_mut().find(|projection| {
            projection.consumer_surface == SearchResultTruthConsumerSurface::SupportExport
        }) {
            projection.preserves_captured_vs_live = false;
        }
        let packet = SearchResultTruthPacket::materialize(input);
        assert_eq!(
            packet.promotion_state,
            SearchResultTruthPromotionState::BlocksStable
        );
        assert!(packet
            .validation_findings
            .iter()
            .any(|finding| finding.finding_kind
                == SearchResultTruthFindingKind::CapturedVsLiveDropped));
    }
}
