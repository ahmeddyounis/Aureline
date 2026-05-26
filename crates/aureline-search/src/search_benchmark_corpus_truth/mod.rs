//! Stable search benchmark corpus, ranking evaluation, and
//! certified-archetype query-pack truth packet for the M4 stable lane.
//!
//! This module is the search-owned contract that binds the benchmark
//! corpus identity, the published ranking-evaluation metrics, and the
//! certified-archetype query packs into one packet that the search
//! shell, docs/help, CLI/headless inspector, support export, the
//! release proof index, and the benchmark lab all read instead of
//! reinventing corpus posture locally.
//!
//! Every row pins a closed `benchmark_corpus_class`,
//! `query_pack_class`, `retention_policy_class`, `provenance_class`,
//! `confidence_class`, and `evaluation_downgrade_state` plus a stable
//! `corpus_id_ref`, `query_pack_id`, per-metric capture ref, and (when
//! downgraded) a `disclosure_ref` so no consumer can promote a row
//! whose corpus is redacted, whose imported baseline lacks
//! provenance, whose observed metric regressed against the published
//! baseline without a waiver, or whose retention policy is missing.
//!
//! The packet is intentionally metadata-only — it never admits raw
//! query text, raw source bodies, raw corpus payloads, secrets, or
//! ambient credentials. Surfaces MUST preserve the metric, query-pack,
//! corpus-provenance, and downgrade-state vocabularies verbatim;
//! collapsing any of them blocks the stable claim until the projection
//! is repaired.

use std::collections::{BTreeMap, BTreeSet};
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

use crate::quick_open_latency_truth::CertifiedArchetypeClass;

/// Stable record-kind tag for [`SearchBenchmarkCorpusTruthPacket`].
pub const SEARCH_BENCHMARK_CORPUS_TRUTH_PACKET_RECORD_KIND: &str =
    "search_benchmark_corpus_truth_stable_packet";

/// Stable record-kind tag for [`SearchBenchmarkCorpusTruthSupportExport`].
pub const SEARCH_BENCHMARK_CORPUS_TRUTH_SUPPORT_EXPORT_RECORD_KIND: &str =
    "search_benchmark_corpus_truth_support_export";

/// Integer schema version for the stable search-benchmark-corpus truth packet.
pub const SEARCH_BENCHMARK_CORPUS_TRUTH_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the boundary schema.
pub const SEARCH_BENCHMARK_CORPUS_TRUTH_SCHEMA_REF: &str =
    "schemas/search/search_benchmark_corpus_truth.schema.json";

/// Repo-relative path of the reviewer doc.
pub const SEARCH_BENCHMARK_CORPUS_TRUTH_DOC_REF: &str =
    "docs/search/m4/finalize-search-benchmark-corpus-ranking-evaluation-and-certified.md";

/// Repo-relative path of the protected fixture corpus directory.
pub const SEARCH_BENCHMARK_CORPUS_TRUTH_FIXTURE_DIR: &str =
    "fixtures/search/m4/search_benchmark_corpus_truth";

/// Repo-relative path of the checked-in stable packet.
pub const SEARCH_BENCHMARK_CORPUS_TRUTH_PACKET_ARTIFACT_REF: &str =
    "artifacts/search/m4/search_benchmark_corpus_truth_packet.json";

/// Repo-relative path of the human-readable reviewer artifact.
pub const SEARCH_BENCHMARK_CORPUS_TRUTH_ARTIFACT_DOC_REF: &str =
    "artifacts/search/m4/finalize-search-benchmark-corpus-ranking-evaluation-and-certified.md";

/// Closed benchmark-corpus vocabulary the packet certifies. Every row
/// binds to exactly one corpus class so consumers cannot quietly mix
/// docs-lookup recall numbers into file-lookup rankings.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BenchmarkCorpusClass {
    /// File and path lookup corpus.
    FileLookupCorpus,
    /// Symbol and structural-navigation corpus.
    SymbolNavigationCorpus,
    /// Command-palette and action-search corpus.
    CommandPaletteCorpus,
    /// Docs-lookup corpus.
    DocsLookupCorpus,
    /// Semantic-recall (vector) corpus.
    SemanticRecallCorpus,
    /// Hybrid (lexical + vector + graph) retrieval corpus.
    HybridRetrievalCorpus,
}

impl BenchmarkCorpusClass {
    /// Every governed benchmark-corpus class, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::FileLookupCorpus,
        Self::SymbolNavigationCorpus,
        Self::CommandPaletteCorpus,
        Self::DocsLookupCorpus,
        Self::SemanticRecallCorpus,
        Self::HybridRetrievalCorpus,
    ];

    /// Stable token used in fixtures, schemas, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::FileLookupCorpus => "file_lookup_corpus",
            Self::SymbolNavigationCorpus => "symbol_navigation_corpus",
            Self::CommandPaletteCorpus => "command_palette_corpus",
            Self::DocsLookupCorpus => "docs_lookup_corpus",
            Self::SemanticRecallCorpus => "semantic_recall_corpus",
            Self::HybridRetrievalCorpus => "hybrid_retrieval_corpus",
        }
    }
}

/// Closed query-pack vocabulary. The packet pins the kind of pack each
/// row certifies so the support export and benchmark lab cannot quietly
/// fold edge-case regressions into the golden pack's metrics.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum QueryPackClass {
    /// Certified gold-standard pack (the M4 stable claim).
    GoldenQueryPack,
    /// Regression coverage pack.
    RegressionQueryPack,
    /// Boundary/edge-case coverage pack.
    EdgeCaseQueryPack,
    /// Imported external-corpus pack (requires explicit provenance).
    ImportedCorpusPack,
}

impl QueryPackClass {
    /// Every governed query-pack class, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::GoldenQueryPack,
        Self::RegressionQueryPack,
        Self::EdgeCaseQueryPack,
        Self::ImportedCorpusPack,
    ];

    /// Stable token used in fixtures, schemas, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::GoldenQueryPack => "golden_query_pack",
            Self::RegressionQueryPack => "regression_query_pack",
            Self::EdgeCaseQueryPack => "edge_case_query_pack",
            Self::ImportedCorpusPack => "imported_corpus_pack",
        }
    }

    /// True when this pack class is built from an external corpus and
    /// MUST therefore carry imported-corpus provenance.
    pub const fn requires_imported_provenance(self) -> bool {
        matches!(self, Self::ImportedCorpusPack)
    }
}

/// Closed ranking-metric vocabulary captured per row. Higher-is-better
/// metrics are expressed in basis points (10000 = 1.0); the latency
/// metric is expressed in milliseconds.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum RankingMetricClass {
    /// Normalized discounted cumulative gain at 10.
    #[serde(rename = "ndcg_at_10")]
    NdcgAt10,
    /// Mean reciprocal rank.
    #[serde(rename = "mrr")]
    Mrr,
    /// Recall at 50.
    #[serde(rename = "recall_at_50")]
    RecallAt50,
    /// Precision at 5.
    #[serde(rename = "precision_at_5")]
    PrecisionAt5,
    /// First-useful-row latency in milliseconds.
    #[serde(rename = "first_useful_row_latency_ms")]
    FirstUsefulRowLatencyMs,
}

impl RankingMetricClass {
    /// Every governed ranking metric, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::NdcgAt10,
        Self::Mrr,
        Self::RecallAt50,
        Self::PrecisionAt5,
        Self::FirstUsefulRowLatencyMs,
    ];

    /// Stable token used in fixtures, schemas, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NdcgAt10 => "ndcg_at_10",
            Self::Mrr => "mrr",
            Self::RecallAt50 => "recall_at_50",
            Self::PrecisionAt5 => "precision_at_5",
            Self::FirstUsefulRowLatencyMs => "first_useful_row_latency_ms",
        }
    }

    /// True when greater observed values are better for this metric.
    pub const fn higher_is_better(self) -> bool {
        match self {
            Self::NdcgAt10 | Self::Mrr | Self::RecallAt50 | Self::PrecisionAt5 => true,
            Self::FirstUsefulRowLatencyMs => false,
        }
    }

    /// True when an observed value regressed against the published baseline.
    pub const fn observation_regressed(self, baseline: u32, observed: u32) -> bool {
        if self.higher_is_better() {
            observed < baseline
        } else {
            observed > baseline
        }
    }
}

/// Closed retention-policy vocabulary for the benchmark corpus material.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RetentionPolicyClass {
    /// Corpus material never leaves the laptop running the benchmark.
    LocalOnly,
    /// Corpus material stays inside the tenant boundary.
    TenantOnly,
    /// Corpus material lives in the shared internal benchmark lab.
    SharedInternal,
    /// Corpus material is published externally under a license.
    PublishedExternal,
}

impl RetentionPolicyClass {
    /// Stable token used in fixtures, schemas, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LocalOnly => "local_only",
            Self::TenantOnly => "tenant_only",
            Self::SharedInternal => "shared_internal",
            Self::PublishedExternal => "published_external",
        }
    }
}

/// Closed provenance vocabulary for the benchmark corpus material.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ProvenanceClass {
    /// Corpus is internally authored by the search team.
    InternallyAuthored,
    /// Corpus is imported from an external benchmark.
    ImportedExternal,
    /// Corpus is community-contributed and reviewed.
    CommunityContributed,
    /// Corpus is synthesized from a seeded generator.
    SynthesizedSeed,
}

impl ProvenanceClass {
    /// Stable token used in fixtures, schemas, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::InternallyAuthored => "internally_authored",
            Self::ImportedExternal => "imported_external",
            Self::CommunityContributed => "community_contributed",
            Self::SynthesizedSeed => "synthesized_seed",
        }
    }

    /// True when this provenance class records an imported external corpus.
    pub const fn is_imported(self) -> bool {
        matches!(self, Self::ImportedExternal)
    }
}

/// Closed confidence-class vocabulary for one row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CorpusConfidenceClass {
    /// Sample is statistically significant and metrics are reproducible.
    HighConfidence,
    /// Sample is reasonable but not fully significant.
    MediumConfidence,
    /// Sample is too small to certify stable; the row narrows below stable.
    LowConfidence,
}

impl CorpusConfidenceClass {
    /// Stable token used in fixtures, schemas, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::HighConfidence => "high_confidence",
            Self::MediumConfidence => "medium_confidence",
            Self::LowConfidence => "low_confidence",
        }
    }
}

/// Closed evaluation-downgrade-state vocabulary applied to a row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EvaluationDowngradeState {
    /// No downgrade: row meets all published baselines.
    None,
    /// Observed values are below baseline; relies on an explicit waiver.
    NarrowedBelowBaseline,
    /// Row inherits an imported baseline (not internally produced).
    ImportedBaseline,
    /// Regression detected against baseline.
    RegressionDetected,
    /// Corpus material is redacted; only metrics are surfaced.
    CorpusRedacted,
}

impl EvaluationDowngradeState {
    /// Stable token used in fixtures, schemas, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::None => "none",
            Self::NarrowedBelowBaseline => "narrowed_below_baseline",
            Self::ImportedBaseline => "imported_baseline",
            Self::RegressionDetected => "regression_detected",
            Self::CorpusRedacted => "corpus_redacted",
        }
    }

    /// True when this state must remain visibly disclosed.
    pub const fn requires_explicit_disclosure(self) -> bool {
        !matches!(self, Self::None)
    }
}

/// Stable promotion state derived from packet validation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CorpusPromotionState {
    /// Packet certifies a stable claim across all required rows.
    Stable,
    /// Packet narrows below stable until a recorded gap closes.
    NarrowedBelowStable,
    /// Packet has a blocker finding and cannot publish on stable surfaces.
    BlocksStable,
}

impl CorpusPromotionState {
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
pub enum CorpusFindingSeverity {
    /// Informational finding.
    Info,
    /// Reviewable finding that narrows the packet below stable.
    Warning,
    /// Blocker finding that prevents stable publication.
    Blocker,
}

/// Closed validation-finding vocabulary for the benchmark-corpus packet.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CorpusFindingKind {
    /// Record kind does not match the schema.
    WrongRecordKind,
    /// Schema version does not match the frozen schema.
    WrongSchemaVersion,
    /// Required identity field is empty.
    MissingIdentity,
    /// Required archetype × corpus row is missing.
    MissingArchetypeOrCorpusRow,
    /// A row references a corpus_id that the packet does not define.
    CorpusIdDrift,
    /// A row reports a metric with no published baseline.
    MissingPublishedBaseline,
    /// A row reports a metric without an observed value.
    MissingObservedMetric,
    /// A row reports an observed metric that regressed against the baseline without a waiver.
    MetricRegressionWithoutWaiver,
    /// A row reports a waiver without a ref.
    WaiverWithoutRef,
    /// A row reports a metric without a benchmark capture ref.
    MissingCaptureRef,
    /// A row reports a downgrade state but no disclosure ref.
    MissingDisclosureRef,
    /// A corpus is defined without a retention policy class.
    MissingRetentionPolicy,
    /// An imported_corpus_pack row references a corpus without imported_external provenance.
    ImportedCorpusWithoutProvenance,
    /// Packet admits raw query text, raw bodies, secrets, or private weights.
    RawQueryMaterialPresent,
    /// A required consumer projection is missing for this packet.
    MissingConsumerProjection,
    /// A consumer projection remints or drops corpus truth.
    ConsumerProjectionDrift,
    /// A projection collapses the metric vocabulary.
    MetricVocabularyCollapsed,
    /// A projection collapses the query-pack vocabulary.
    QueryPackVocabularyCollapsed,
    /// A projection collapses the corpus provenance vocabulary.
    CorpusProvenanceCollapsed,
    /// A projection collapses the evaluation-downgrade vocabulary.
    DowngradeStateCollapsed,
    /// Stored promotion state disagrees with derived findings.
    PromotionStateMismatch,
}

impl CorpusFindingKind {
    /// Stable token used in fixtures, schemas, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WrongRecordKind => "wrong_record_kind",
            Self::WrongSchemaVersion => "wrong_schema_version",
            Self::MissingIdentity => "missing_identity",
            Self::MissingArchetypeOrCorpusRow => "missing_archetype_or_corpus_row",
            Self::CorpusIdDrift => "corpus_id_drift",
            Self::MissingPublishedBaseline => "missing_published_baseline",
            Self::MissingObservedMetric => "missing_observed_metric",
            Self::MetricRegressionWithoutWaiver => "metric_regression_without_waiver",
            Self::WaiverWithoutRef => "waiver_without_ref",
            Self::MissingCaptureRef => "missing_capture_ref",
            Self::MissingDisclosureRef => "missing_disclosure_ref",
            Self::MissingRetentionPolicy => "missing_retention_policy",
            Self::ImportedCorpusWithoutProvenance => "imported_corpus_without_provenance",
            Self::RawQueryMaterialPresent => "raw_query_material_present",
            Self::MissingConsumerProjection => "missing_consumer_projection",
            Self::ConsumerProjectionDrift => "consumer_projection_drift",
            Self::MetricVocabularyCollapsed => "metric_vocabulary_collapsed",
            Self::QueryPackVocabularyCollapsed => "query_pack_vocabulary_collapsed",
            Self::CorpusProvenanceCollapsed => "corpus_provenance_collapsed",
            Self::DowngradeStateCollapsed => "downgrade_state_collapsed",
            Self::PromotionStateMismatch => "promotion_state_mismatch",
        }
    }
}

/// Consumer surface that must inherit the benchmark-corpus packet verbatim.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CorpusConsumerSurface {
    /// Search shell quick-open, file, symbol, command, and benchmark-lab panes.
    SearchShell,
    /// Docs/help surface explaining the corpus and ranking budgets.
    DocsHelp,
    /// CLI or headless inspection surface.
    CliHeadless,
    /// Support export bundle.
    SupportExport,
    /// Release proof index entry.
    ReleaseProofIndex,
    /// Benchmark-lab dashboard and capture surface.
    BenchmarkLab,
}

impl CorpusConsumerSurface {
    /// Every required consumer surface, in declaration order.
    pub const REQUIRED: [Self; 6] = [
        Self::SearchShell,
        Self::DocsHelp,
        Self::CliHeadless,
        Self::SupportExport,
        Self::ReleaseProofIndex,
        Self::BenchmarkLab,
    ];

    /// Stable token used in fixtures, schemas, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SearchShell => "search_shell",
            Self::DocsHelp => "docs_help",
            Self::CliHeadless => "cli_headless",
            Self::SupportExport => "support_export",
            Self::ReleaseProofIndex => "release_proof_index",
            Self::BenchmarkLab => "benchmark_lab",
        }
    }
}

/// One validation finding emitted by the corpus packet validator.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CorpusValidationFinding {
    /// Closed finding kind.
    pub finding_kind: CorpusFindingKind,
    /// Finding severity.
    pub severity: CorpusFindingSeverity,
    /// Short support-safe summary.
    pub summary: String,
}

impl CorpusValidationFinding {
    fn new(
        finding_kind: CorpusFindingKind,
        severity: CorpusFindingSeverity,
        summary: impl Into<String>,
    ) -> Self {
        Self {
            finding_kind,
            severity,
            summary: summary.into(),
        }
    }
}

/// One ranking-metric observation captured for a row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RankingMetricObservation {
    /// Closed metric class.
    pub metric_class: RankingMetricClass,
    /// Published baseline (basis points for ratio metrics; milliseconds for latency).
    pub published_baseline: u32,
    /// Observed value at the same scaling as the baseline.
    pub observed_value: u32,
    /// Sample size for this metric capture.
    pub sample_size: u32,
    /// Repo-relative benchmark-lab capture ref.
    pub capture_ref: String,
    /// Optional waiver ref when the metric is intentionally regressed.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub waiver_ref: Option<String>,
}

impl RankingMetricObservation {
    fn regressed(&self) -> bool {
        self.metric_class
            .observation_regressed(self.published_baseline, self.observed_value)
    }
}

/// One benchmark corpus definition referenced by rows.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BenchmarkCorpusDefinition {
    /// Stable corpus id; rows reference this via `corpus_id_ref`.
    pub corpus_id: String,
    /// Corpus class.
    pub corpus_class: BenchmarkCorpusClass,
    /// Certified archetype this corpus belongs to.
    pub archetype: CertifiedArchetypeClass,
    /// Retention policy applied to the corpus material.
    pub retention_policy: RetentionPolicyClass,
    /// Provenance of the corpus material.
    pub provenance: ProvenanceClass,
    /// Number of queries in the corpus.
    pub sample_size: u32,
    /// Repo-relative source-of-truth ref where the corpus lives.
    pub source_ref: String,
    /// True when raw query text and source bodies are excluded from the corpus payload.
    pub raw_query_material_excluded: bool,
    /// Optional provenance ref when the corpus is imported_external.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub imported_provenance_ref: Option<String>,
}

/// One certified-archetype query-pack row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CertifiedArchetypePackRow {
    /// Stable row id within the packet.
    pub row_id: String,
    /// Certified archetype that owns the row.
    pub archetype: CertifiedArchetypeClass,
    /// Benchmark-corpus class certified by the row.
    pub corpus_class: BenchmarkCorpusClass,
    /// Query-pack class certified by the row.
    pub query_pack_class: QueryPackClass,
    /// Stable query-pack id (the certified pack itself).
    pub query_pack_id: String,
    /// Stable corpus id referenced by the row.
    pub corpus_id_ref: String,
    /// Number of queries in the pack.
    pub query_count: u32,
    /// Per-metric ranking-evaluation observations.
    #[serde(default)]
    pub metrics: Vec<RankingMetricObservation>,
    /// Confidence class for the row's evaluation.
    pub confidence_class: CorpusConfidenceClass,
    /// Evaluation downgrade state for the row.
    pub downgrade_state: EvaluationDowngradeState,
    /// Repo-relative disclosure ref shown when the row is downgraded.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub disclosure_ref: Option<String>,
    /// True when raw query text, source bodies, and secrets are excluded.
    pub raw_query_material_excluded: bool,
    /// Capture timestamp for the row.
    pub captured_at: String,
}

/// Consumer projection proving a surface reads this packet verbatim.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CorpusConsumerProjection {
    /// Consumer surface class.
    pub consumer_surface: CorpusConsumerSurface,
    /// Stable projection ref.
    pub projection_ref: String,
    /// Corpus packet id consumed by the projection.
    pub corpus_packet_id_ref: String,
    /// Rendered-at timestamp.
    pub rendered_at: String,
    /// True when the surface preserves the same packet id.
    pub preserves_same_packet: bool,
    /// True when the metric vocabulary is preserved verbatim.
    pub preserves_metric_vocabulary: bool,
    /// True when the query-pack vocabulary is preserved verbatim.
    pub preserves_query_pack_vocabulary: bool,
    /// True when the corpus provenance vocabulary is preserved verbatim.
    pub preserves_corpus_provenance: bool,
    /// True when the downgrade-state vocabulary is preserved verbatim.
    pub preserves_downgrade_state: bool,
    /// True when JSON export is available from the projection.
    pub supports_json_export: bool,
    /// True when raw private material is excluded.
    pub raw_private_material_excluded: bool,
    /// True when ambient authority/credentials are excluded.
    pub ambient_authority_excluded: bool,
}

impl CorpusConsumerProjection {
    fn preserves_truth_for(&self, packet_id: &str) -> bool {
        self.corpus_packet_id_ref == packet_id
            && self.preserves_same_packet
            && self.preserves_metric_vocabulary
            && self.preserves_query_pack_vocabulary
            && self.preserves_corpus_provenance
            && self.preserves_downgrade_state
            && self.supports_json_export
            && self.raw_private_material_excluded
            && self.ambient_authority_excluded
            && !self.projection_ref.trim().is_empty()
    }
}

/// Constructor input for [`SearchBenchmarkCorpusTruthPacket::materialize`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SearchBenchmarkCorpusTruthPacketInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Claimed workflow or surface id.
    pub workflow_or_surface_id: String,
    /// Capture timestamp for the packet.
    pub generated_at: String,
    /// Certified archetypes the packet covers.
    #[serde(default)]
    pub covered_archetypes: Vec<CertifiedArchetypeClass>,
    /// Benchmark corpus classes the packet covers.
    #[serde(default)]
    pub covered_corpus_classes: Vec<BenchmarkCorpusClass>,
    /// Benchmark corpus definitions referenced by rows.
    #[serde(default)]
    pub corpora: Vec<BenchmarkCorpusDefinition>,
    /// Certified-archetype query-pack rows.
    #[serde(default)]
    pub rows: Vec<CertifiedArchetypePackRow>,
    /// Consumer projections preserving this packet.
    #[serde(default)]
    pub consumer_projections: Vec<CorpusConsumerProjection>,
    /// Source contracts (docs/schema/fixtures) consumed by the packet.
    #[serde(default)]
    pub source_contract_refs: Vec<String>,
}

/// Search-owned packet for benchmark corpus identity, ranking evaluation,
/// and certified-archetype query packs on the M4 stable lane.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SearchBenchmarkCorpusTruthPacket {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Claimed workflow or surface id.
    pub workflow_or_surface_id: String,
    /// Packet capture timestamp.
    pub generated_at: String,
    /// Certified archetypes the packet covers.
    #[serde(default)]
    pub covered_archetypes: Vec<CertifiedArchetypeClass>,
    /// Benchmark corpus classes the packet covers.
    #[serde(default)]
    pub covered_corpus_classes: Vec<BenchmarkCorpusClass>,
    /// Benchmark corpus definitions referenced by rows.
    #[serde(default)]
    pub corpora: Vec<BenchmarkCorpusDefinition>,
    /// Certified-archetype query-pack rows.
    #[serde(default)]
    pub rows: Vec<CertifiedArchetypePackRow>,
    /// Consumer projections preserving this packet.
    #[serde(default)]
    pub consumer_projections: Vec<CorpusConsumerProjection>,
    /// Source contract refs consumed by the packet.
    #[serde(default)]
    pub source_contract_refs: Vec<String>,
    /// Derived promotion state.
    pub promotion_state: CorpusPromotionState,
    /// Validation findings captured at materialization.
    #[serde(default)]
    pub validation_findings: Vec<CorpusValidationFinding>,
}

impl SearchBenchmarkCorpusTruthPacket {
    /// Materializes a packet and records derived validation findings.
    pub fn materialize(input: SearchBenchmarkCorpusTruthPacketInput) -> Self {
        let mut packet = Self {
            record_kind: SEARCH_BENCHMARK_CORPUS_TRUTH_PACKET_RECORD_KIND.to_owned(),
            schema_version: SEARCH_BENCHMARK_CORPUS_TRUTH_SCHEMA_VERSION,
            packet_id: input.packet_id,
            workflow_or_surface_id: input.workflow_or_surface_id,
            generated_at: input.generated_at,
            covered_archetypes: input.covered_archetypes,
            covered_corpus_classes: input.covered_corpus_classes,
            corpora: input.corpora,
            rows: input.rows,
            consumer_projections: input.consumer_projections,
            source_contract_refs: input.source_contract_refs,
            promotion_state: CorpusPromotionState::Stable,
            validation_findings: Vec::new(),
        };
        let findings = packet.derived_findings(false);
        packet.promotion_state = promotion_state_for_findings(&findings);
        packet.validation_findings = findings;
        packet
    }

    /// Re-validates the packet against stable benchmark-corpus invariants.
    pub fn validate(&self) -> Vec<CorpusValidationFinding> {
        self.derived_findings(true)
    }

    /// Returns true when this packet has no blocker-level finding.
    pub fn is_stable(&self) -> bool {
        !self
            .validate()
            .iter()
            .any(|finding| finding.severity == CorpusFindingSeverity::Blocker)
    }

    /// Returns true when a consumer projection preserves this packet.
    pub fn has_projection_for(&self, surface: CorpusConsumerSurface) -> bool {
        self.consumer_projections.iter().any(|projection| {
            projection.consumer_surface == surface
                && projection.preserves_truth_for(&self.packet_id)
        })
    }

    /// Returns the unique archetype tokens observed across rows.
    pub fn archetype_tokens(&self) -> Vec<&'static str> {
        let mut set = BTreeSet::new();
        for row in &self.rows {
            set.insert(row.archetype);
        }
        set.into_iter()
            .map(CertifiedArchetypeClass::as_str)
            .collect()
    }

    /// Returns the unique corpus-class tokens observed across rows.
    pub fn corpus_class_tokens(&self) -> Vec<&'static str> {
        let mut set = BTreeSet::new();
        for row in &self.rows {
            set.insert(row.corpus_class);
        }
        set.into_iter().map(BenchmarkCorpusClass::as_str).collect()
    }

    /// Returns the unique query-pack tokens observed across rows.
    pub fn query_pack_tokens(&self) -> Vec<&'static str> {
        let mut set = BTreeSet::new();
        for row in &self.rows {
            set.insert(row.query_pack_class);
        }
        set.into_iter().map(QueryPackClass::as_str).collect()
    }

    /// Returns the unique metric-class tokens observed across rows.
    pub fn metric_tokens(&self) -> Vec<&'static str> {
        let mut set = BTreeSet::new();
        for row in &self.rows {
            for metric in &row.metrics {
                set.insert(metric.metric_class);
            }
        }
        set.into_iter().map(RankingMetricClass::as_str).collect()
    }

    /// Returns the unique retention-policy tokens observed across corpora.
    pub fn retention_policy_tokens(&self) -> Vec<&'static str> {
        let mut set = BTreeSet::new();
        for corpus in &self.corpora {
            set.insert(corpus.retention_policy);
        }
        set.into_iter().map(RetentionPolicyClass::as_str).collect()
    }

    /// Returns the unique provenance tokens observed across corpora.
    pub fn provenance_tokens(&self) -> Vec<&'static str> {
        let mut set = BTreeSet::new();
        for corpus in &self.corpora {
            set.insert(corpus.provenance);
        }
        set.into_iter().map(ProvenanceClass::as_str).collect()
    }

    /// Returns the unique downgrade-state tokens observed across rows.
    pub fn downgrade_state_tokens(&self) -> Vec<&'static str> {
        let mut set = BTreeSet::new();
        for row in &self.rows {
            set.insert(row.downgrade_state);
        }
        set.into_iter()
            .map(EvaluationDowngradeState::as_str)
            .collect()
    }

    /// Builds a support export wrapping the exact packet shown to product surfaces.
    pub fn support_export(
        &self,
        export_id: impl Into<String>,
        exported_at: impl Into<String>,
    ) -> SearchBenchmarkCorpusTruthSupportExport {
        SearchBenchmarkCorpusTruthSupportExport {
            record_kind: SEARCH_BENCHMARK_CORPUS_TRUTH_SUPPORT_EXPORT_RECORD_KIND.to_owned(),
            schema_version: SEARCH_BENCHMARK_CORPUS_TRUTH_SCHEMA_VERSION,
            export_id: export_id.into(),
            corpus_packet_id_ref: self.packet_id.clone(),
            exported_at: exported_at.into(),
            raw_private_material_excluded: true,
            ambient_authority_excluded: true,
            corpus_packet: self.clone(),
        }
    }

    fn derived_findings(&self, include_record_fields: bool) -> Vec<CorpusValidationFinding> {
        let mut findings = Vec::new();

        if include_record_fields
            && self.record_kind != SEARCH_BENCHMARK_CORPUS_TRUTH_PACKET_RECORD_KIND
        {
            findings.push(CorpusValidationFinding::new(
                CorpusFindingKind::WrongRecordKind,
                CorpusFindingSeverity::Blocker,
                "benchmark-corpus packet has the wrong record kind",
            ));
        }
        if include_record_fields
            && self.schema_version != SEARCH_BENCHMARK_CORPUS_TRUTH_SCHEMA_VERSION
        {
            findings.push(CorpusValidationFinding::new(
                CorpusFindingKind::WrongSchemaVersion,
                CorpusFindingSeverity::Blocker,
                "benchmark-corpus packet has the wrong schema version",
            ));
        }
        if self.packet_id.trim().is_empty()
            || self.workflow_or_surface_id.trim().is_empty()
            || self.generated_at.trim().is_empty()
        {
            findings.push(CorpusValidationFinding::new(
                CorpusFindingKind::MissingIdentity,
                CorpusFindingSeverity::Blocker,
                "packet, workflow, and timestamp refs are required",
            ));
        }
        if self.covered_archetypes.is_empty() || self.covered_corpus_classes.is_empty() {
            findings.push(CorpusValidationFinding::new(
                CorpusFindingKind::MissingArchetypeOrCorpusRow,
                CorpusFindingSeverity::Blocker,
                "packet must declare covered archetypes and corpus classes",
            ));
        }

        for archetype in &self.covered_archetypes {
            for corpus_class in &self.covered_corpus_classes {
                let present = self
                    .rows
                    .iter()
                    .any(|row| row.archetype == *archetype && row.corpus_class == *corpus_class);
                if !present {
                    findings.push(CorpusValidationFinding::new(
                        CorpusFindingKind::MissingArchetypeOrCorpusRow,
                        CorpusFindingSeverity::Blocker,
                        format!(
                            "no row covers archetype {} on corpus class {}",
                            archetype.as_str(),
                            corpus_class.as_str()
                        ),
                    ));
                }
            }
        }

        let mut corpus_lookup: BTreeMap<&str, &BenchmarkCorpusDefinition> = BTreeMap::new();
        for corpus in &self.corpora {
            if corpus.corpus_id.trim().is_empty()
                || corpus.source_ref.trim().is_empty()
                || corpus.sample_size == 0
            {
                findings.push(CorpusValidationFinding::new(
                    CorpusFindingKind::MissingIdentity,
                    CorpusFindingSeverity::Blocker,
                    format!(
                        "corpus {} identity, source ref, or sample size is empty",
                        corpus.corpus_id
                    ),
                ));
            }
            corpus_lookup.insert(corpus.corpus_id.as_str(), corpus);
            if !corpus.raw_query_material_excluded {
                findings.push(CorpusValidationFinding::new(
                    CorpusFindingKind::RawQueryMaterialPresent,
                    CorpusFindingSeverity::Blocker,
                    format!(
                        "corpus {} admits raw query text, source bodies, or secrets",
                        corpus.corpus_id
                    ),
                ));
            }
            if corpus.provenance.is_imported() && corpus.imported_provenance_ref.is_none() {
                findings.push(CorpusValidationFinding::new(
                    CorpusFindingKind::ImportedCorpusWithoutProvenance,
                    CorpusFindingSeverity::Blocker,
                    format!(
                        "imported corpus {} has no provenance ref",
                        corpus.corpus_id
                    ),
                ));
            }
        }

        // covered_corpus_classes must each have at least one corpus defined for each covered archetype.
        for archetype in &self.covered_archetypes {
            for corpus_class in &self.covered_corpus_classes {
                let has_corpus = self.corpora.iter().any(|corpus| {
                    corpus.archetype == *archetype && corpus.corpus_class == *corpus_class
                });
                if !has_corpus {
                    findings.push(CorpusValidationFinding::new(
                        CorpusFindingKind::MissingArchetypeOrCorpusRow,
                        CorpusFindingSeverity::Blocker,
                        format!(
                            "no corpus defined for archetype {} on corpus class {}",
                            archetype.as_str(),
                            corpus_class.as_str()
                        ),
                    ));
                }
            }
        }

        for row in &self.rows {
            if row.row_id.trim().is_empty()
                || row.query_pack_id.trim().is_empty()
                || row.corpus_id_ref.trim().is_empty()
                || row.captured_at.trim().is_empty()
            {
                findings.push(CorpusValidationFinding::new(
                    CorpusFindingKind::MissingIdentity,
                    CorpusFindingSeverity::Blocker,
                    format!(
                        "row {} identity, pack, corpus ref, or timestamp is empty",
                        row.row_id
                    ),
                ));
            }
            if !row.raw_query_material_excluded {
                findings.push(CorpusValidationFinding::new(
                    CorpusFindingKind::RawQueryMaterialPresent,
                    CorpusFindingSeverity::Blocker,
                    format!(
                        "row {} admits raw query text, source bodies, or secrets",
                        row.row_id
                    ),
                ));
            }
            if row.query_count == 0 {
                findings.push(CorpusValidationFinding::new(
                    CorpusFindingKind::MissingIdentity,
                    CorpusFindingSeverity::Blocker,
                    format!("row {} declares zero queries", row.row_id),
                ));
            }
            if row.downgrade_state.requires_explicit_disclosure() && row.disclosure_ref.is_none() {
                findings.push(CorpusValidationFinding::new(
                    CorpusFindingKind::MissingDisclosureRef,
                    CorpusFindingSeverity::Blocker,
                    format!(
                        "row {} has downgrade state {} without a disclosure ref",
                        row.row_id,
                        row.downgrade_state.as_str()
                    ),
                ));
            }

            let corpus = corpus_lookup.get(row.corpus_id_ref.as_str()).copied();
            if corpus.is_none() {
                findings.push(CorpusValidationFinding::new(
                    CorpusFindingKind::CorpusIdDrift,
                    CorpusFindingSeverity::Blocker,
                    format!(
                        "row {} references corpus {} that is not defined in this packet",
                        row.row_id, row.corpus_id_ref
                    ),
                ));
            }

            if let Some(corpus) = corpus {
                if corpus.archetype != row.archetype || corpus.corpus_class != row.corpus_class {
                    findings.push(CorpusValidationFinding::new(
                        CorpusFindingKind::CorpusIdDrift,
                        CorpusFindingSeverity::Blocker,
                        format!(
                            "row {} corpus binding disagrees with corpus {} identity",
                            row.row_id, corpus.corpus_id
                        ),
                    ));
                }
                if matches!(corpus.retention_policy, RetentionPolicyClass::LocalOnly)
                    && !matches!(row.query_pack_class, QueryPackClass::GoldenQueryPack | QueryPackClass::RegressionQueryPack | QueryPackClass::EdgeCaseQueryPack | QueryPackClass::ImportedCorpusPack)
                {
                    // exhaustive across the closed vocabulary; this branch protects future additions.
                    findings.push(CorpusValidationFinding::new(
                        CorpusFindingKind::MissingRetentionPolicy,
                        CorpusFindingSeverity::Blocker,
                        format!(
                            "row {} has an unrecognized query-pack class against a local-only corpus",
                            row.row_id
                        ),
                    ));
                }
                if row.query_pack_class.requires_imported_provenance()
                    && !corpus.provenance.is_imported()
                {
                    findings.push(CorpusValidationFinding::new(
                        CorpusFindingKind::ImportedCorpusWithoutProvenance,
                        CorpusFindingSeverity::Blocker,
                        format!(
                            "row {} is an imported_corpus_pack but corpus {} is not imported_external",
                            row.row_id, corpus.corpus_id
                        ),
                    ));
                }
            }

            if row.metrics.is_empty() {
                findings.push(CorpusValidationFinding::new(
                    CorpusFindingKind::MissingObservedMetric,
                    CorpusFindingSeverity::Blocker,
                    format!("row {} declares no ranking-metric observations", row.row_id),
                ));
            }

            for metric in &row.metrics {
                if metric.sample_size == 0 {
                    findings.push(CorpusValidationFinding::new(
                        CorpusFindingKind::MissingObservedMetric,
                        CorpusFindingSeverity::Blocker,
                        format!(
                            "row {} metric {} has no observed sample size",
                            row.row_id,
                            metric.metric_class.as_str()
                        ),
                    ));
                }
                if metric.published_baseline == 0 {
                    findings.push(CorpusValidationFinding::new(
                        CorpusFindingKind::MissingPublishedBaseline,
                        CorpusFindingSeverity::Blocker,
                        format!(
                            "row {} metric {} has no published baseline",
                            row.row_id,
                            metric.metric_class.as_str()
                        ),
                    ));
                }
                if metric.capture_ref.trim().is_empty() {
                    findings.push(CorpusValidationFinding::new(
                        CorpusFindingKind::MissingCaptureRef,
                        CorpusFindingSeverity::Blocker,
                        format!(
                            "row {} metric {} has no benchmark capture ref",
                            row.row_id,
                            metric.metric_class.as_str()
                        ),
                    ));
                }
                if metric.waiver_ref.as_deref().is_some_and(str::is_empty) {
                    findings.push(CorpusValidationFinding::new(
                        CorpusFindingKind::WaiverWithoutRef,
                        CorpusFindingSeverity::Blocker,
                        format!(
                            "row {} metric {} declares a waiver without a ref",
                            row.row_id,
                            metric.metric_class.as_str()
                        ),
                    ));
                }
                if metric.regressed() && metric.waiver_ref.is_none() {
                    findings.push(CorpusValidationFinding::new(
                        CorpusFindingKind::MetricRegressionWithoutWaiver,
                        CorpusFindingSeverity::Blocker,
                        format!(
                            "row {} metric {} observed {} regressed against baseline {} without waiver",
                            row.row_id,
                            metric.metric_class.as_str(),
                            metric.observed_value,
                            metric.published_baseline
                        ),
                    ));
                }
                if metric.regressed() && metric.waiver_ref.is_some() {
                    findings.push(CorpusValidationFinding::new(
                        CorpusFindingKind::MetricRegressionWithoutWaiver,
                        CorpusFindingSeverity::Warning,
                        format!(
                            "row {} metric {} relies on waiver {}",
                            row.row_id,
                            metric.metric_class.as_str(),
                            metric
                                .waiver_ref
                                .as_deref()
                                .unwrap_or("<missing>")
                        ),
                    ));
                }
            }

            if matches!(row.confidence_class, CorpusConfidenceClass::LowConfidence) {
                findings.push(CorpusValidationFinding::new(
                    CorpusFindingKind::MissingObservedMetric,
                    CorpusFindingSeverity::Warning,
                    format!(
                        "row {} confidence is low; narrowing below stable until sample size grows",
                        row.row_id
                    ),
                ));
            }
        }

        for required_surface in CorpusConsumerSurface::REQUIRED {
            if !self.has_projection_for(required_surface) {
                findings.push(CorpusValidationFinding::new(
                    CorpusFindingKind::MissingConsumerProjection,
                    CorpusFindingSeverity::Blocker,
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
                findings.push(CorpusValidationFinding::new(
                    CorpusFindingKind::ConsumerProjectionDrift,
                    CorpusFindingSeverity::Blocker,
                    format!(
                        "projection {} does not preserve corpus truth",
                        projection.projection_ref
                    ),
                ));
            }
            if !projection.preserves_metric_vocabulary {
                findings.push(CorpusValidationFinding::new(
                    CorpusFindingKind::MetricVocabularyCollapsed,
                    CorpusFindingSeverity::Blocker,
                    format!(
                        "projection {} collapses the metric vocabulary",
                        projection.projection_ref
                    ),
                ));
            }
            if !projection.preserves_query_pack_vocabulary {
                findings.push(CorpusValidationFinding::new(
                    CorpusFindingKind::QueryPackVocabularyCollapsed,
                    CorpusFindingSeverity::Blocker,
                    format!(
                        "projection {} collapses the query-pack vocabulary",
                        projection.projection_ref
                    ),
                ));
            }
            if !projection.preserves_corpus_provenance {
                findings.push(CorpusValidationFinding::new(
                    CorpusFindingKind::CorpusProvenanceCollapsed,
                    CorpusFindingSeverity::Blocker,
                    format!(
                        "projection {} collapses the corpus provenance vocabulary",
                        projection.projection_ref
                    ),
                ));
            }
            if !projection.preserves_downgrade_state {
                findings.push(CorpusValidationFinding::new(
                    CorpusFindingKind::DowngradeStateCollapsed,
                    CorpusFindingSeverity::Blocker,
                    format!(
                        "projection {} collapses the downgrade-state vocabulary",
                        projection.projection_ref
                    ),
                ));
            }
        }

        if include_record_fields {
            let mut without_promotion = findings.clone();
            without_promotion.retain(|finding| {
                finding.finding_kind != CorpusFindingKind::PromotionStateMismatch
            });
            let derived = promotion_state_for_findings(&without_promotion);
            if self.promotion_state != derived {
                findings.push(CorpusValidationFinding::new(
                    CorpusFindingKind::PromotionStateMismatch,
                    CorpusFindingSeverity::Blocker,
                    "stored promotion state does not match derived findings",
                ));
            }
        }

        findings
    }
}

fn promotion_state_for_findings(
    findings: &[CorpusValidationFinding],
) -> CorpusPromotionState {
    if findings
        .iter()
        .any(|finding| finding.severity == CorpusFindingSeverity::Blocker)
    {
        CorpusPromotionState::BlocksStable
    } else if findings
        .iter()
        .any(|finding| finding.severity == CorpusFindingSeverity::Warning)
    {
        CorpusPromotionState::NarrowedBelowStable
    } else {
        CorpusPromotionState::Stable
    }
}

/// Support-export wrapper that preserves the product benchmark-corpus packet verbatim.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SearchBenchmarkCorpusTruthSupportExport {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Stable export id.
    pub export_id: String,
    /// Packet id preserved by the export.
    pub corpus_packet_id_ref: String,
    /// Export timestamp.
    pub exported_at: String,
    /// True when raw private material is excluded.
    pub raw_private_material_excluded: bool,
    /// True when ambient credentials/authority are excluded.
    pub ambient_authority_excluded: bool,
    /// Exact product packet preserved by the export.
    pub corpus_packet: SearchBenchmarkCorpusTruthPacket,
}

impl SearchBenchmarkCorpusTruthSupportExport {
    /// Returns true when the export preserves the same packet id safely.
    pub fn is_export_safe(&self) -> bool {
        self.record_kind == SEARCH_BENCHMARK_CORPUS_TRUTH_SUPPORT_EXPORT_RECORD_KIND
            && self.schema_version == SEARCH_BENCHMARK_CORPUS_TRUTH_SCHEMA_VERSION
            && self.corpus_packet_id_ref == self.corpus_packet.packet_id
            && self.raw_private_material_excluded
            && self.ambient_authority_excluded
            && self.corpus_packet.validate().is_empty()
    }
}

/// Errors emitted when reading the checked-in stable benchmark-corpus packet.
#[derive(Debug)]
pub enum SearchBenchmarkCorpusTruthArtifactError {
    /// Packet failed to parse.
    Packet(serde_json::Error),
    /// Packet failed validation.
    Validation(Vec<CorpusValidationFinding>),
}

impl fmt::Display for SearchBenchmarkCorpusTruthArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Packet(error) => write!(
                formatter,
                "search-benchmark-corpus packet parse failed: {error}"
            ),
            Self::Validation(findings) => {
                let tokens = findings
                    .iter()
                    .map(|finding| finding.finding_kind.as_str())
                    .collect::<Vec<_>>()
                    .join(",");
                write!(
                    formatter,
                    "search-benchmark-corpus packet failed validation: {tokens}"
                )
            }
        }
    }
}

impl Error for SearchBenchmarkCorpusTruthArtifactError {}

/// Returns the checked-in stable benchmark-corpus truth packet.
///
/// # Errors
///
/// Returns an artifact error if the checked-in packet does not parse or validate.
pub fn current_stable_search_benchmark_corpus_truth_packet(
) -> Result<SearchBenchmarkCorpusTruthPacket, SearchBenchmarkCorpusTruthArtifactError> {
    let packet: SearchBenchmarkCorpusTruthPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/search/m4/search_benchmark_corpus_truth_packet.json"
    )))
    .map_err(SearchBenchmarkCorpusTruthArtifactError::Packet)?;
    let findings = packet.validate();
    if findings.is_empty() {
        Ok(packet)
    } else {
        Err(SearchBenchmarkCorpusTruthArtifactError::Validation(findings))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_metric() -> RankingMetricObservation {
        RankingMetricObservation {
            metric_class: RankingMetricClass::NdcgAt10,
            published_baseline: 8000,
            observed_value: 8200,
            sample_size: 512,
            capture_ref: "benchmarks/search/ranking/rust_workspace/golden.json".to_owned(),
            waiver_ref: None,
        }
    }

    fn sample_corpus() -> BenchmarkCorpusDefinition {
        BenchmarkCorpusDefinition {
            corpus_id: "corpus:rust_workspace:file_lookup".to_owned(),
            corpus_class: BenchmarkCorpusClass::FileLookupCorpus,
            archetype: CertifiedArchetypeClass::RustWorkspace,
            retention_policy: RetentionPolicyClass::SharedInternal,
            provenance: ProvenanceClass::InternallyAuthored,
            sample_size: 512,
            source_ref: "benchmarks/search/corpora/rust_workspace/file_lookup.json".to_owned(),
            raw_query_material_excluded: true,
            imported_provenance_ref: None,
        }
    }

    fn sample_row() -> CertifiedArchetypePackRow {
        CertifiedArchetypePackRow {
            row_id: "row:rust_workspace:file_lookup:golden".to_owned(),
            archetype: CertifiedArchetypeClass::RustWorkspace,
            corpus_class: BenchmarkCorpusClass::FileLookupCorpus,
            query_pack_class: QueryPackClass::GoldenQueryPack,
            query_pack_id: "pack:rust_workspace:file_lookup:golden".to_owned(),
            corpus_id_ref: "corpus:rust_workspace:file_lookup".to_owned(),
            query_count: 512,
            metrics: vec![sample_metric()],
            confidence_class: CorpusConfidenceClass::HighConfidence,
            downgrade_state: EvaluationDowngradeState::None,
            disclosure_ref: None,
            raw_query_material_excluded: true,
            captured_at: "2026-05-26T12:00:00Z".to_owned(),
        }
    }

    fn sample_projection(surface: CorpusConsumerSurface) -> CorpusConsumerProjection {
        CorpusConsumerProjection {
            consumer_surface: surface,
            projection_ref: format!("projection:{}", surface.as_str()),
            corpus_packet_id_ref: "packet:m4:search_benchmark_corpus_truth".to_owned(),
            rendered_at: "2026-05-26T12:00:01Z".to_owned(),
            preserves_same_packet: true,
            preserves_metric_vocabulary: true,
            preserves_query_pack_vocabulary: true,
            preserves_corpus_provenance: true,
            preserves_downgrade_state: true,
            supports_json_export: true,
            raw_private_material_excluded: true,
            ambient_authority_excluded: true,
        }
    }

    fn sample_input() -> SearchBenchmarkCorpusTruthPacketInput {
        SearchBenchmarkCorpusTruthPacketInput {
            packet_id: "packet:m4:search_benchmark_corpus_truth".to_owned(),
            workflow_or_surface_id: "workflow.search.benchmark_corpus_truth".to_owned(),
            generated_at: "2026-05-26T12:00:00Z".to_owned(),
            covered_archetypes: vec![CertifiedArchetypeClass::RustWorkspace],
            covered_corpus_classes: vec![BenchmarkCorpusClass::FileLookupCorpus],
            corpora: vec![sample_corpus()],
            rows: vec![sample_row()],
            consumer_projections: CorpusConsumerSurface::REQUIRED
                .iter()
                .copied()
                .map(sample_projection)
                .collect(),
            source_contract_refs: vec![SEARCH_BENCHMARK_CORPUS_TRUTH_DOC_REF.to_owned()],
        }
    }

    #[test]
    fn closed_tokens_are_pinned() {
        assert_eq!(
            BenchmarkCorpusClass::HybridRetrievalCorpus.as_str(),
            "hybrid_retrieval_corpus"
        );
        assert_eq!(QueryPackClass::ImportedCorpusPack.as_str(), "imported_corpus_pack");
        assert_eq!(RankingMetricClass::NdcgAt10.as_str(), "ndcg_at_10");
        assert_eq!(RetentionPolicyClass::TenantOnly.as_str(), "tenant_only");
        assert_eq!(ProvenanceClass::ImportedExternal.as_str(), "imported_external");
        assert_eq!(
            EvaluationDowngradeState::RegressionDetected.as_str(),
            "regression_detected"
        );
        assert_eq!(CorpusPromotionState::BlocksStable.as_str(), "blocks_stable");
        assert_eq!(
            CorpusFindingKind::MetricRegressionWithoutWaiver.as_str(),
            "metric_regression_without_waiver"
        );
    }

    #[test]
    fn baseline_materialization_is_stable() {
        let packet = SearchBenchmarkCorpusTruthPacket::materialize(sample_input());
        assert_eq!(packet.promotion_state, CorpusPromotionState::Stable);
        assert!(packet.validation_findings.is_empty());
        assert!(packet.is_stable());
        assert!(packet
            .support_export("support:m4:bench", "2026-05-26T12:00:10Z")
            .is_export_safe());
    }

    #[test]
    fn metric_regression_without_waiver_blocks_promotion() {
        let mut input = sample_input();
        input.rows[0].metrics[0].observed_value = 5000;
        let packet = SearchBenchmarkCorpusTruthPacket::materialize(input);
        assert_eq!(packet.promotion_state, CorpusPromotionState::BlocksStable);
        assert!(packet
            .validation_findings
            .iter()
            .any(|finding| finding.finding_kind
                == CorpusFindingKind::MetricRegressionWithoutWaiver
                && finding.severity == CorpusFindingSeverity::Blocker));
    }

    #[test]
    fn metric_regression_with_waiver_narrows_below_stable() {
        let mut input = sample_input();
        input.rows[0].metrics[0].observed_value = 5000;
        input.rows[0].metrics[0].waiver_ref =
            Some("artifacts/release/m4/waivers/rust_workspace_file_lookup.json".to_owned());
        let packet = SearchBenchmarkCorpusTruthPacket::materialize(input);
        assert_eq!(packet.promotion_state, CorpusPromotionState::NarrowedBelowStable);
    }

    #[test]
    fn imported_pack_without_provenance_blocks_promotion() {
        let mut input = sample_input();
        input.rows[0].query_pack_class = QueryPackClass::ImportedCorpusPack;
        // corpus provenance still InternallyAuthored -> blocks
        let packet = SearchBenchmarkCorpusTruthPacket::materialize(input);
        assert_eq!(packet.promotion_state, CorpusPromotionState::BlocksStable);
        assert!(packet.validation_findings.iter().any(|finding| {
            finding.finding_kind == CorpusFindingKind::ImportedCorpusWithoutProvenance
        }));
    }

    #[test]
    fn corpus_id_drift_blocks_promotion() {
        let mut input = sample_input();
        input.rows[0].corpus_id_ref = "corpus:unknown".to_owned();
        let packet = SearchBenchmarkCorpusTruthPacket::materialize(input);
        assert_eq!(packet.promotion_state, CorpusPromotionState::BlocksStable);
        assert!(packet
            .validation_findings
            .iter()
            .any(|finding| finding.finding_kind == CorpusFindingKind::CorpusIdDrift));
    }

    #[test]
    fn projection_drop_blocks_promotion() {
        let mut input = sample_input();
        input
            .consumer_projections
            .retain(|projection| projection.consumer_surface != CorpusConsumerSurface::BenchmarkLab);
        let packet = SearchBenchmarkCorpusTruthPacket::materialize(input);
        assert_eq!(packet.promotion_state, CorpusPromotionState::BlocksStable);
        assert!(packet
            .validation_findings
            .iter()
            .any(|finding| finding.finding_kind == CorpusFindingKind::MissingConsumerProjection));
    }

    #[test]
    fn collapsed_metric_vocabulary_blocks_promotion() {
        let mut input = sample_input();
        for projection in &mut input.consumer_projections {
            if projection.consumer_surface == CorpusConsumerSurface::DocsHelp {
                projection.preserves_metric_vocabulary = false;
            }
        }
        let packet = SearchBenchmarkCorpusTruthPacket::materialize(input);
        assert_eq!(packet.promotion_state, CorpusPromotionState::BlocksStable);
        assert!(packet
            .validation_findings
            .iter()
            .any(|finding| finding.finding_kind == CorpusFindingKind::MetricVocabularyCollapsed));
    }

    #[test]
    fn corpus_redacted_without_disclosure_blocks_promotion() {
        let mut input = sample_input();
        input.rows[0].downgrade_state = EvaluationDowngradeState::CorpusRedacted;
        input.rows[0].disclosure_ref = None;
        let packet = SearchBenchmarkCorpusTruthPacket::materialize(input);
        assert_eq!(packet.promotion_state, CorpusPromotionState::BlocksStable);
        assert!(packet
            .validation_findings
            .iter()
            .any(|finding| finding.finding_kind == CorpusFindingKind::MissingDisclosureRef));
    }
}
