//! Stable monorepo hot-set indexing, warming-state, and graceful-degradation
//! truth packet for large-repo search rows.
//!
//! This module is the search-owned contract that finalizes the M4 monorepo
//! stable lane: every governed `monorepo archetype × indexing lane` row has
//! one packet that proves how the lane warms, when partial-index truth must
//! remain visible, which graceful-degradation class is in force, and that the
//! foreground responsiveness invariants (edit input unblocked, first useful
//! quick-open row within budget) hold while broader indexing is still
//! materializing.
//!
//! The packet is intentionally metadata-only — it carries no raw query text,
//! raw source bodies, secrets, ambient credentials, or provider payloads — and
//! binds the closed vocabularies that the search shell, docs/help, CLI or
//! headless inspector, support export, and the release proof index must read
//! verbatim. Surfaces MUST NOT collapse `hot_set_ready`, `partial_index`,
//! `stale_shard_served`, `known_paths_fallback`, or `index_unavailable`
//! disclosures into a generic loading spinner or success badge; the validator
//! refuses to certify when a projection drops the readiness vocabulary,
//! degradation class, or disclosure ref.

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

use crate::hot_set::SearchReadinessState;

/// Stable record-kind tag for [`MonorepoHotSetTruthPacket`].
pub const MONOREPO_HOT_SET_TRUTH_PACKET_RECORD_KIND: &str =
    "monorepo_hot_set_truth_stable_packet";

/// Stable record-kind tag for [`MonorepoHotSetTruthSupportExport`].
pub const MONOREPO_HOT_SET_TRUTH_SUPPORT_EXPORT_RECORD_KIND: &str =
    "monorepo_hot_set_truth_support_export";

/// Integer schema version for stable monorepo hot-set truth packets.
pub const MONOREPO_HOT_SET_TRUTH_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the boundary schema.
pub const MONOREPO_HOT_SET_TRUTH_SCHEMA_REF: &str =
    "schemas/search/monorepo_hot_set_truth.schema.json";

/// Repo-relative path of the reviewer doc.
pub const MONOREPO_HOT_SET_TRUTH_DOC_REF: &str =
    "docs/search/m4/finalize-monorepo-hot-set-indexing-warming-states-and.md";

/// Repo-relative path of the human-readable reviewer artifact.
pub const MONOREPO_HOT_SET_TRUTH_ARTIFACT_DOC_REF: &str =
    "artifacts/search/m4/finalize-monorepo-hot-set-indexing-warming-states-and.md";

/// Repo-relative path of the protected fixture corpus directory.
pub const MONOREPO_HOT_SET_TRUTH_FIXTURE_DIR: &str =
    "fixtures/search/m4/monorepo_hot_set_truth";

/// Repo-relative path of the checked-in stable monorepo truth packet.
pub const MONOREPO_HOT_SET_TRUTH_PACKET_ARTIFACT_REF: &str =
    "artifacts/search/m4/monorepo_hot_set_truth_packet.json";

/// Closed monorepo archetype that owns one hot-set truth row.
///
/// Archetypes name the shape of the workspace being indexed, not the
/// language family — the latency-truth packet already covers archetype ×
/// surface latency budgets, so the monorepo packet pins the orthogonal
/// scale-and-shape axis.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MonorepoArchetypeClass {
    /// Single-root repo under the small workspace threshold.
    SmallSingleRoot,
    /// Single-root mid-size repo (≈5k–50k tracked files).
    MediumSingleRoot,
    /// Single-root large monorepo (≈50k–500k tracked files).
    LargeSingleRoot,
    /// Multi-root polyglot workspace with several worksets.
    PolyglotMultiRoot,
    /// Monorepo dominated by generated artifacts.
    GeneratedArtifactDominant,
    /// Monorepo over the very-large size threshold (500k+ tracked files).
    VeryLargeMonorepo,
}

impl MonorepoArchetypeClass {
    /// Every governed monorepo archetype, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::SmallSingleRoot,
        Self::MediumSingleRoot,
        Self::LargeSingleRoot,
        Self::PolyglotMultiRoot,
        Self::GeneratedArtifactDominant,
        Self::VeryLargeMonorepo,
    ];

    /// Stable token used in fixtures, schemas, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SmallSingleRoot => "small_single_root",
            Self::MediumSingleRoot => "medium_single_root",
            Self::LargeSingleRoot => "large_single_root",
            Self::PolyglotMultiRoot => "polyglot_multi_root",
            Self::GeneratedArtifactDominant => "generated_artifact_dominant",
            Self::VeryLargeMonorepo => "very_large_monorepo",
        }
    }
}

/// Indexing lane the monorepo truth row certifies.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum IndexingLaneClass {
    /// Filename / quick-open index lane.
    FilenameIndex,
    /// Workspace-relative path index lane.
    PathIndex,
    /// Symbol index lane.
    SymbolIndex,
    /// Full-text index lane.
    TextIndex,
}

impl IndexingLaneClass {
    /// Every governed lane, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::FilenameIndex,
        Self::PathIndex,
        Self::SymbolIndex,
        Self::TextIndex,
    ];

    /// Stable token used in fixtures, schemas, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::FilenameIndex => "filename_index",
            Self::PathIndex => "path_index",
            Self::SymbolIndex => "symbol_index",
            Self::TextIndex => "text_index",
        }
    }
}

/// Closed graceful-degradation class attached to a monorepo truth row.
///
/// `NoDegradation` means the lane reached full coverage for the declared
/// scope. All other classes MUST carry a labeled disclosure ref and MUST
/// remain visible across consumer projections.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum GracefulDegradationClass {
    /// Full coverage for the declared scope; nothing is degraded.
    NoDegradation,
    /// Only the hot set is ready; cold paths still indexing.
    HotSetOnly,
    /// Coverage is partial against the declared scope and labeled.
    PartialIndexDeclared,
    /// A stale shard is being served while a fresher version warms.
    StaleShardServed,
    /// No hot inputs match; the lane falls back to known catalog paths.
    KnownPathsFallback,
    /// The lane is intentionally paused under resource pressure.
    PausedForResourcePressure,
    /// The lane is unavailable; the row is intentionally narrowed.
    IndexUnavailableDisclosed,
}

impl GracefulDegradationClass {
    /// Stable token used in fixtures, schemas, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NoDegradation => "no_degradation",
            Self::HotSetOnly => "hot_set_only",
            Self::PartialIndexDeclared => "partial_index_declared",
            Self::StaleShardServed => "stale_shard_served",
            Self::KnownPathsFallback => "known_paths_fallback",
            Self::PausedForResourcePressure => "paused_for_resource_pressure",
            Self::IndexUnavailableDisclosed => "index_unavailable_disclosed",
        }
    }

    /// True when the class requires a visible disclosure ref on the row.
    pub const fn requires_disclosure(self) -> bool {
        !matches!(self, Self::NoDegradation)
    }
}

/// Closed promotion state for [`MonorepoHotSetTruthPacket`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MonorepoTruthPromotionState {
    /// Packet certifies a stable claim for every declared archetype × lane row.
    Stable,
    /// Packet must remain narrowed below stable until a recorded gap closes.
    NarrowedBelowStable,
    /// Packet has a blocker finding and cannot publish on stable surfaces.
    BlocksStable,
}

impl MonorepoTruthPromotionState {
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
pub enum MonorepoTruthFindingSeverity {
    /// Informational finding.
    Info,
    /// Reviewable finding that narrows the row below stable.
    Warning,
    /// Blocker that prevents stable publication.
    Blocker,
}

/// Closed validation-finding vocabulary for [`MonorepoHotSetTruthPacket`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MonorepoTruthFindingKind {
    /// Record kind does not match the schema.
    WrongRecordKind,
    /// Schema version does not match the frozen schema.
    WrongSchemaVersion,
    /// Required identity field is empty.
    MissingIdentity,
    /// Required archetype × lane row is missing.
    MissingArchetypeOrLaneRow,
    /// A row reported a non-`fully_indexed` readiness state with no warming transition.
    MissingWarmingTransition,
    /// A row reported a degradation class but no disclosure ref.
    DegradationNotLabeled,
    /// A row reported a degradation class that disagrees with the readiness state.
    DegradationReadinessMismatch,
    /// A row reported the foreground edit input is blocked behind index warm-up.
    EditInputBlockedByWarmup,
    /// A row reported the first useful quick-open row missed its budget.
    FirstUsefulRowBudgetMissed,
    /// A row reported a hot-set coverage estimate that exceeds the declared scope.
    HotSetCoverageOverDeclared,
    /// A row's planner-version, lane id, or workspace id drifts across projections.
    LaneIdentityDrift,
    /// A required consumer projection is missing for this packet.
    MissingConsumerProjection,
    /// A consumer projection remints or drops monorepo hot-set truth.
    ConsumerProjectionDrift,
    /// A consumer projection collapses the readiness vocabulary.
    ReadinessVocabularyCollapsed,
    /// A consumer projection drops degradation labels.
    DegradationVocabularyDropped,
    /// Packet admits raw query text, raw bodies, secrets, or private weights.
    RawBoundaryMaterialPresent,
    /// Stored promotion state disagrees with derived findings.
    PromotionStateMismatch,
}

impl MonorepoTruthFindingKind {
    /// Stable token used in fixtures, schemas, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WrongRecordKind => "wrong_record_kind",
            Self::WrongSchemaVersion => "wrong_schema_version",
            Self::MissingIdentity => "missing_identity",
            Self::MissingArchetypeOrLaneRow => "missing_archetype_or_lane_row",
            Self::MissingWarmingTransition => "missing_warming_transition",
            Self::DegradationNotLabeled => "degradation_not_labeled",
            Self::DegradationReadinessMismatch => "degradation_readiness_mismatch",
            Self::EditInputBlockedByWarmup => "edit_input_blocked_by_warmup",
            Self::FirstUsefulRowBudgetMissed => "first_useful_row_budget_missed",
            Self::HotSetCoverageOverDeclared => "hot_set_coverage_over_declared",
            Self::LaneIdentityDrift => "lane_identity_drift",
            Self::MissingConsumerProjection => "missing_consumer_projection",
            Self::ConsumerProjectionDrift => "consumer_projection_drift",
            Self::ReadinessVocabularyCollapsed => "readiness_vocabulary_collapsed",
            Self::DegradationVocabularyDropped => "degradation_vocabulary_dropped",
            Self::RawBoundaryMaterialPresent => "raw_boundary_material_present",
            Self::PromotionStateMismatch => "promotion_state_mismatch",
        }
    }
}

/// Consumer surface that must inherit the monorepo packet's truth verbatim.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MonorepoConsumerSurface {
    /// Search shell quick-open, file, symbol, and text search panes.
    SearchShell,
    /// Docs/help surface explaining warming states and graceful degradation.
    DocsHelp,
    /// CLI or headless inspection surface.
    CliHeadless,
    /// Support export bundle.
    SupportExport,
    /// Release proof index entry.
    ReleaseProofIndex,
}

impl MonorepoConsumerSurface {
    /// Every required consumer surface, in declaration order.
    pub const REQUIRED: [Self; 5] = [
        Self::SearchShell,
        Self::DocsHelp,
        Self::CliHeadless,
        Self::SupportExport,
        Self::ReleaseProofIndex,
    ];

    /// Stable token used in fixtures, schemas, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SearchShell => "search_shell",
            Self::DocsHelp => "docs_help",
            Self::CliHeadless => "cli_headless",
            Self::SupportExport => "support_export",
            Self::ReleaseProofIndex => "release_proof_index",
        }
    }
}

/// One warming-state transition captured for a monorepo truth row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WarmingTransition {
    /// Readiness state the lane left.
    pub from_state: SearchReadinessState,
    /// Readiness state the lane entered.
    pub to_state: SearchReadinessState,
    /// Milliseconds since session open when the transition fired.
    pub elapsed_ms: u32,
    /// True when this transition emitted a first useful row to the user.
    pub emits_first_useful_row: bool,
}

/// Foreground responsiveness invariants for a monorepo truth row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ResponsivenessInvariants {
    /// True when edit input remains unblocked while indexing continues.
    pub edit_input_unblocked: bool,
    /// True when quick-open emits its first useful row within budget.
    pub quick_open_first_useful_within_budget: bool,
    /// True when the full-index work is deferred with explicit disclosure.
    pub full_index_deferred_with_disclosure: bool,
    /// Observed first-useful-row latency in milliseconds.
    pub first_useful_row_ms_observed: u32,
    /// Published budget for first-useful-row latency in milliseconds.
    pub first_useful_row_ms_budget: u32,
}

impl ResponsivenessInvariants {
    fn first_useful_meets_budget(&self) -> bool {
        self.first_useful_row_ms_budget == 0
            || self.first_useful_row_ms_observed <= self.first_useful_row_ms_budget
    }
}

/// Hot-set coverage estimate for one row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HotSetCoverageEstimate {
    /// Estimated number of files reachable by the hot set.
    pub hot_set_file_count: u32,
    /// Total tracked files in the declared scope.
    pub declared_scope_file_count: u32,
    /// Estimated number of cold (deferred) paths.
    pub deferred_cold_path_count: u32,
}

impl HotSetCoverageEstimate {
    fn coverage_over_declared(&self) -> bool {
        self.declared_scope_file_count > 0 && self.hot_set_file_count > self.declared_scope_file_count
    }
}

/// One row in the monorepo hot-set truth packet bound to an archetype × lane.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MonorepoTruthRow {
    /// Stable row id within the packet.
    pub row_id: String,
    /// Monorepo archetype that owns this row.
    pub archetype: MonorepoArchetypeClass,
    /// Indexing lane this row certifies.
    pub lane: IndexingLaneClass,
    /// Stable workspace identity for the row.
    pub workspace_id: String,
    /// Planner version that produced the row.
    pub planner_version: String,
    /// Workset or scope ref used by the captured session.
    pub scope_ref: String,
    /// Readiness state for the row at the captured time.
    pub readiness_state: SearchReadinessState,
    /// Graceful-degradation class active on the row.
    pub degradation: GracefulDegradationClass,
    /// Repo-relative disclosure ref shown when the row is degraded.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub degradation_disclosure_ref: Option<String>,
    /// Captured warming-state transitions ordered by elapsed time.
    #[serde(default)]
    pub warming_transitions: Vec<WarmingTransition>,
    /// Hot-set coverage estimate for the row.
    pub hot_set_coverage: HotSetCoverageEstimate,
    /// Foreground responsiveness invariants for the row.
    pub responsiveness: ResponsivenessInvariants,
    /// True when raw private query text, source bodies, and secrets are excluded.
    pub raw_boundary_material_excluded: bool,
    /// Capture timestamp for this row.
    pub captured_at: String,
}

impl MonorepoTruthRow {
    fn readiness_requires_visible_warming(&self) -> bool {
        !matches!(self.readiness_state, SearchReadinessState::FullyIndexed)
    }

    fn degradation_matches_readiness(&self) -> bool {
        matches!(
            (self.readiness_state, self.degradation),
            (SearchReadinessState::FullyIndexed, GracefulDegradationClass::NoDegradation)
                | (SearchReadinessState::HotSetReady, GracefulDegradationClass::HotSetOnly)
                | (
                    SearchReadinessState::PartialIndex | SearchReadinessState::WarmIndex,
                    GracefulDegradationClass::PartialIndexDeclared
                        | GracefulDegradationClass::KnownPathsFallback
                        | GracefulDegradationClass::HotSetOnly,
                )
                | (SearchReadinessState::StaleIndex, GracefulDegradationClass::StaleShardServed)
                | (
                    SearchReadinessState::Reindexing,
                    GracefulDegradationClass::PartialIndexDeclared
                        | GracefulDegradationClass::PausedForResourcePressure,
                )
                | (
                    SearchReadinessState::IndexUnavailable,
                    GracefulDegradationClass::IndexUnavailableDisclosed
                        | GracefulDegradationClass::KnownPathsFallback,
                )
                | (
                    SearchReadinessState::NotIndexed,
                    GracefulDegradationClass::KnownPathsFallback
                        | GracefulDegradationClass::IndexUnavailableDisclosed
                        | GracefulDegradationClass::PausedForResourcePressure,
                )
        )
    }
}

/// Consumer projection proving a surface reads this packet verbatim.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MonorepoConsumerProjection {
    /// Consumer surface class.
    pub consumer_surface: MonorepoConsumerSurface,
    /// Stable projection ref.
    pub projection_ref: String,
    /// Packet id consumed by the projection.
    pub monorepo_packet_id_ref: String,
    /// Rendered-at timestamp.
    pub rendered_at: String,
    /// True when the surface preserves the same packet id.
    pub preserves_same_packet: bool,
    /// True when the surface preserves lane × archetype identity.
    pub preserves_lane_identity: bool,
    /// True when readiness-state vocabulary is quoted verbatim.
    pub preserves_readiness_vocabulary: bool,
    /// True when graceful-degradation labels are quoted verbatim.
    pub preserves_degradation_labels: bool,
    /// True when foreground responsiveness invariants are shown to the user.
    pub preserves_responsiveness_invariants: bool,
    /// True when JSON export is available from the projection.
    pub supports_json_export: bool,
    /// True when raw private material is excluded.
    pub raw_private_material_excluded: bool,
    /// True when ambient authority/credentials are excluded.
    pub ambient_authority_excluded: bool,
}

impl MonorepoConsumerProjection {
    fn preserves_truth_for(&self, packet_id: &str) -> bool {
        self.monorepo_packet_id_ref == packet_id
            && self.preserves_same_packet
            && self.preserves_lane_identity
            && self.preserves_readiness_vocabulary
            && self.preserves_degradation_labels
            && self.preserves_responsiveness_invariants
            && self.supports_json_export
            && self.raw_private_material_excluded
            && self.ambient_authority_excluded
            && !self.projection_ref.trim().is_empty()
    }
}

/// One validation finding emitted by the monorepo packet validator.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MonorepoTruthValidationFinding {
    /// Closed finding kind.
    pub finding_kind: MonorepoTruthFindingKind,
    /// Finding severity.
    pub severity: MonorepoTruthFindingSeverity,
    /// Short support-safe summary.
    pub summary: String,
}

impl MonorepoTruthValidationFinding {
    fn new(
        finding_kind: MonorepoTruthFindingKind,
        severity: MonorepoTruthFindingSeverity,
        summary: impl Into<String>,
    ) -> Self {
        Self {
            finding_kind,
            severity,
            summary: summary.into(),
        }
    }
}

/// Constructor input for [`MonorepoHotSetTruthPacket::materialize`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MonorepoHotSetTruthPacketInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Claimed workflow or surface id.
    pub workflow_or_surface_id: String,
    /// Capture timestamp for the packet as a whole.
    pub generated_at: String,
    /// Monorepo archetypes the packet covers.
    #[serde(default)]
    pub covered_archetypes: Vec<MonorepoArchetypeClass>,
    /// Indexing lanes the packet covers.
    #[serde(default)]
    pub covered_lanes: Vec<IndexingLaneClass>,
    /// Truth rows, one per declared archetype × lane.
    #[serde(default)]
    pub rows: Vec<MonorepoTruthRow>,
    /// Consumer projections preserving this packet.
    #[serde(default)]
    pub consumer_projections: Vec<MonorepoConsumerProjection>,
    /// Source contracts (docs/schema/fixtures) consumed by the packet.
    #[serde(default)]
    pub source_contract_refs: Vec<String>,
}

/// Search-owned packet for monorepo hot-set indexing, warming-state, and
/// graceful-degradation truth on certified large-repo archetypes.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MonorepoHotSetTruthPacket {
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
    /// Monorepo archetypes the packet covers.
    #[serde(default)]
    pub covered_archetypes: Vec<MonorepoArchetypeClass>,
    /// Indexing lanes the packet covers.
    #[serde(default)]
    pub covered_lanes: Vec<IndexingLaneClass>,
    /// Truth rows.
    #[serde(default)]
    pub rows: Vec<MonorepoTruthRow>,
    /// Consumer projections preserving this packet.
    #[serde(default)]
    pub consumer_projections: Vec<MonorepoConsumerProjection>,
    /// Source contract refs consumed by the packet.
    #[serde(default)]
    pub source_contract_refs: Vec<String>,
    /// Derived promotion state.
    pub promotion_state: MonorepoTruthPromotionState,
    /// Validation findings captured at materialization.
    #[serde(default)]
    pub validation_findings: Vec<MonorepoTruthValidationFinding>,
}

impl MonorepoHotSetTruthPacket {
    /// Materializes a packet and records derived validation findings.
    pub fn materialize(input: MonorepoHotSetTruthPacketInput) -> Self {
        let mut packet = Self {
            record_kind: MONOREPO_HOT_SET_TRUTH_PACKET_RECORD_KIND.to_owned(),
            schema_version: MONOREPO_HOT_SET_TRUTH_SCHEMA_VERSION,
            packet_id: input.packet_id,
            workflow_or_surface_id: input.workflow_or_surface_id,
            generated_at: input.generated_at,
            covered_archetypes: input.covered_archetypes,
            covered_lanes: input.covered_lanes,
            rows: input.rows,
            consumer_projections: input.consumer_projections,
            source_contract_refs: input.source_contract_refs,
            promotion_state: MonorepoTruthPromotionState::Stable,
            validation_findings: Vec::new(),
        };
        let findings = packet.derived_findings(false);
        packet.promotion_state = promotion_state_for_findings(&findings);
        packet.validation_findings = findings;
        packet
    }

    /// Re-validates the packet against stable monorepo-truth invariants.
    pub fn validate(&self) -> Vec<MonorepoTruthValidationFinding> {
        self.derived_findings(true)
    }

    /// Returns true when this packet has no blocker-level finding.
    pub fn is_stable(&self) -> bool {
        !self
            .validate()
            .iter()
            .any(|finding| finding.severity == MonorepoTruthFindingSeverity::Blocker)
    }

    /// Returns true when a consumer projection preserves this packet.
    pub fn has_projection_for(&self, surface: MonorepoConsumerSurface) -> bool {
        self.consumer_projections.iter().any(|projection| {
            projection.consumer_surface == surface && projection.preserves_truth_for(&self.packet_id)
        })
    }

    /// Returns the unique archetype tokens observed across rows.
    pub fn archetype_tokens(&self) -> Vec<&'static str> {
        let mut set = BTreeSet::new();
        for row in &self.rows {
            set.insert(row.archetype);
        }
        set.into_iter().map(MonorepoArchetypeClass::as_str).collect()
    }

    /// Returns the unique lane tokens observed across rows.
    pub fn lane_tokens(&self) -> Vec<&'static str> {
        let mut set = BTreeSet::new();
        for row in &self.rows {
            set.insert(row.lane);
        }
        set.into_iter().map(IndexingLaneClass::as_str).collect()
    }

    /// Returns the unique degradation-class tokens observed across rows.
    pub fn degradation_tokens(&self) -> Vec<&'static str> {
        let mut set = BTreeSet::new();
        for row in &self.rows {
            set.insert(row.degradation);
        }
        set.into_iter().map(GracefulDegradationClass::as_str).collect()
    }

    /// Returns the unique readiness-state tokens observed across rows.
    pub fn readiness_state_tokens(&self) -> Vec<&'static str> {
        let mut set = BTreeSet::new();
        for row in &self.rows {
            set.insert(row.readiness_state);
        }
        set.into_iter().map(SearchReadinessState::as_str).collect()
    }

    /// Builds a support export wrapping the exact packet shown to product surfaces.
    pub fn support_export(
        &self,
        export_id: impl Into<String>,
        exported_at: impl Into<String>,
    ) -> MonorepoHotSetTruthSupportExport {
        MonorepoHotSetTruthSupportExport {
            record_kind: MONOREPO_HOT_SET_TRUTH_SUPPORT_EXPORT_RECORD_KIND.to_owned(),
            schema_version: MONOREPO_HOT_SET_TRUTH_SCHEMA_VERSION,
            export_id: export_id.into(),
            monorepo_packet_id_ref: self.packet_id.clone(),
            exported_at: exported_at.into(),
            raw_private_material_excluded: true,
            ambient_authority_excluded: true,
            monorepo_packet: self.clone(),
        }
    }

    fn derived_findings(&self, include_record_fields: bool) -> Vec<MonorepoTruthValidationFinding> {
        let mut findings = Vec::new();

        if include_record_fields && self.record_kind != MONOREPO_HOT_SET_TRUTH_PACKET_RECORD_KIND {
            findings.push(MonorepoTruthValidationFinding::new(
                MonorepoTruthFindingKind::WrongRecordKind,
                MonorepoTruthFindingSeverity::Blocker,
                "monorepo hot-set truth packet has the wrong record kind",
            ));
        }
        if include_record_fields && self.schema_version != MONOREPO_HOT_SET_TRUTH_SCHEMA_VERSION {
            findings.push(MonorepoTruthValidationFinding::new(
                MonorepoTruthFindingKind::WrongSchemaVersion,
                MonorepoTruthFindingSeverity::Blocker,
                "monorepo hot-set truth packet has the wrong schema version",
            ));
        }
        if self.packet_id.trim().is_empty()
            || self.workflow_or_surface_id.trim().is_empty()
            || self.generated_at.trim().is_empty()
        {
            findings.push(MonorepoTruthValidationFinding::new(
                MonorepoTruthFindingKind::MissingIdentity,
                MonorepoTruthFindingSeverity::Blocker,
                "packet, workflow, and timestamp refs are required",
            ));
        }

        if self.covered_archetypes.is_empty() || self.covered_lanes.is_empty() {
            findings.push(MonorepoTruthValidationFinding::new(
                MonorepoTruthFindingKind::MissingArchetypeOrLaneRow,
                MonorepoTruthFindingSeverity::Blocker,
                "packet must declare covered archetypes and lanes",
            ));
        }

        for archetype in &self.covered_archetypes {
            for lane in &self.covered_lanes {
                let present = self
                    .rows
                    .iter()
                    .any(|row| row.archetype == *archetype && row.lane == *lane);
                if !present {
                    findings.push(MonorepoTruthValidationFinding::new(
                        MonorepoTruthFindingKind::MissingArchetypeOrLaneRow,
                        MonorepoTruthFindingSeverity::Blocker,
                        format!(
                            "no row covers archetype {} on lane {}",
                            archetype.as_str(),
                            lane.as_str()
                        ),
                    ));
                }
            }
        }

        for row in &self.rows {
            if row.row_id.trim().is_empty()
                || row.workspace_id.trim().is_empty()
                || row.planner_version.trim().is_empty()
                || row.scope_ref.trim().is_empty()
                || row.captured_at.trim().is_empty()
            {
                findings.push(MonorepoTruthValidationFinding::new(
                    MonorepoTruthFindingKind::MissingIdentity,
                    MonorepoTruthFindingSeverity::Blocker,
                    format!(
                        "row {} identity, workspace, scope, planner, or timestamp is empty",
                        row.row_id
                    ),
                ));
            }
            if row.readiness_requires_visible_warming() && row.warming_transitions.is_empty() {
                findings.push(MonorepoTruthValidationFinding::new(
                    MonorepoTruthFindingKind::MissingWarmingTransition,
                    MonorepoTruthFindingSeverity::Blocker,
                    format!(
                        "row {} reports {} but records no warming transition",
                        row.row_id,
                        row.readiness_state.as_str()
                    ),
                ));
            }
            if row.degradation.requires_disclosure() && row.degradation_disclosure_ref.is_none() {
                findings.push(MonorepoTruthValidationFinding::new(
                    MonorepoTruthFindingKind::DegradationNotLabeled,
                    MonorepoTruthFindingSeverity::Blocker,
                    format!(
                        "row {} has degradation {} with no disclosure ref",
                        row.row_id,
                        row.degradation.as_str()
                    ),
                ));
            }
            if let Some(reference) = row.degradation_disclosure_ref.as_deref() {
                if reference.trim().is_empty() {
                    findings.push(MonorepoTruthValidationFinding::new(
                        MonorepoTruthFindingKind::DegradationNotLabeled,
                        MonorepoTruthFindingSeverity::Blocker,
                        format!("row {} declares an empty disclosure ref", row.row_id),
                    ));
                }
            }
            if !row.degradation_matches_readiness() {
                findings.push(MonorepoTruthValidationFinding::new(
                    MonorepoTruthFindingKind::DegradationReadinessMismatch,
                    MonorepoTruthFindingSeverity::Blocker,
                    format!(
                        "row {} readiness {} disagrees with degradation {}",
                        row.row_id,
                        row.readiness_state.as_str(),
                        row.degradation.as_str()
                    ),
                ));
            }
            if !row.responsiveness.edit_input_unblocked {
                findings.push(MonorepoTruthValidationFinding::new(
                    MonorepoTruthFindingKind::EditInputBlockedByWarmup,
                    MonorepoTruthFindingSeverity::Blocker,
                    format!(
                        "row {} reports edit input is blocked behind index warm-up",
                        row.row_id
                    ),
                ));
            }
            if !row.responsiveness.first_useful_meets_budget()
                || !row.responsiveness.quick_open_first_useful_within_budget
            {
                findings.push(MonorepoTruthValidationFinding::new(
                    MonorepoTruthFindingKind::FirstUsefulRowBudgetMissed,
                    MonorepoTruthFindingSeverity::Blocker,
                    format!(
                        "row {} observed first-useful-row {}ms exceeds budget {}ms",
                        row.row_id,
                        row.responsiveness.first_useful_row_ms_observed,
                        row.responsiveness.first_useful_row_ms_budget
                    ),
                ));
            }
            if row.hot_set_coverage.coverage_over_declared() {
                findings.push(MonorepoTruthValidationFinding::new(
                    MonorepoTruthFindingKind::HotSetCoverageOverDeclared,
                    MonorepoTruthFindingSeverity::Blocker,
                    format!(
                        "row {} reports hot-set coverage {} that exceeds declared scope {}",
                        row.row_id,
                        row.hot_set_coverage.hot_set_file_count,
                        row.hot_set_coverage.declared_scope_file_count
                    ),
                ));
            }
            if !row.raw_boundary_material_excluded {
                findings.push(MonorepoTruthValidationFinding::new(
                    MonorepoTruthFindingKind::RawBoundaryMaterialPresent,
                    MonorepoTruthFindingSeverity::Blocker,
                    format!(
                        "row {} admits raw query text, source bodies, or private weights",
                        row.row_id
                    ),
                ));
            }
        }

        for required_surface in MonorepoConsumerSurface::REQUIRED {
            if !self.has_projection_for(required_surface) {
                findings.push(MonorepoTruthValidationFinding::new(
                    MonorepoTruthFindingKind::MissingConsumerProjection,
                    MonorepoTruthFindingSeverity::Blocker,
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
                findings.push(MonorepoTruthValidationFinding::new(
                    MonorepoTruthFindingKind::ConsumerProjectionDrift,
                    MonorepoTruthFindingSeverity::Blocker,
                    format!(
                        "projection {} does not preserve monorepo hot-set truth",
                        projection.projection_ref
                    ),
                ));
            }
            if !projection.preserves_readiness_vocabulary {
                findings.push(MonorepoTruthValidationFinding::new(
                    MonorepoTruthFindingKind::ReadinessVocabularyCollapsed,
                    MonorepoTruthFindingSeverity::Blocker,
                    format!(
                        "projection {} collapses the readiness vocabulary",
                        projection.projection_ref
                    ),
                ));
            }
            if !projection.preserves_degradation_labels {
                findings.push(MonorepoTruthValidationFinding::new(
                    MonorepoTruthFindingKind::DegradationVocabularyDropped,
                    MonorepoTruthFindingSeverity::Blocker,
                    format!(
                        "projection {} drops the graceful-degradation vocabulary",
                        projection.projection_ref
                    ),
                ));
            }
            if !projection.preserves_lane_identity {
                findings.push(MonorepoTruthValidationFinding::new(
                    MonorepoTruthFindingKind::LaneIdentityDrift,
                    MonorepoTruthFindingSeverity::Blocker,
                    format!(
                        "projection {} drops lane × archetype identity",
                        projection.projection_ref
                    ),
                ));
            }
        }

        if include_record_fields {
            let mut without_promotion = findings.clone();
            without_promotion.retain(|finding| {
                finding.finding_kind != MonorepoTruthFindingKind::PromotionStateMismatch
            });
            let derived = promotion_state_for_findings(&without_promotion);
            if self.promotion_state != derived {
                findings.push(MonorepoTruthValidationFinding::new(
                    MonorepoTruthFindingKind::PromotionStateMismatch,
                    MonorepoTruthFindingSeverity::Blocker,
                    "stored promotion state does not match derived findings",
                ));
            }
        }

        findings
    }
}

fn promotion_state_for_findings(
    findings: &[MonorepoTruthValidationFinding],
) -> MonorepoTruthPromotionState {
    if findings
        .iter()
        .any(|finding| finding.severity == MonorepoTruthFindingSeverity::Blocker)
    {
        MonorepoTruthPromotionState::BlocksStable
    } else if findings
        .iter()
        .any(|finding| finding.severity == MonorepoTruthFindingSeverity::Warning)
    {
        MonorepoTruthPromotionState::NarrowedBelowStable
    } else {
        MonorepoTruthPromotionState::Stable
    }
}

/// Support-export wrapper that preserves the product monorepo packet verbatim.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MonorepoHotSetTruthSupportExport {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Stable export id.
    pub export_id: String,
    /// Packet id preserved by the export.
    pub monorepo_packet_id_ref: String,
    /// Export timestamp.
    pub exported_at: String,
    /// True when raw private material is excluded.
    pub raw_private_material_excluded: bool,
    /// True when ambient credentials/authority are excluded.
    pub ambient_authority_excluded: bool,
    /// Exact product packet preserved by the export.
    pub monorepo_packet: MonorepoHotSetTruthPacket,
}

impl MonorepoHotSetTruthSupportExport {
    /// Returns true when the export preserves the same packet id safely.
    pub fn is_export_safe(&self) -> bool {
        self.record_kind == MONOREPO_HOT_SET_TRUTH_SUPPORT_EXPORT_RECORD_KIND
            && self.schema_version == MONOREPO_HOT_SET_TRUTH_SCHEMA_VERSION
            && self.monorepo_packet_id_ref == self.monorepo_packet.packet_id
            && self.raw_private_material_excluded
            && self.ambient_authority_excluded
            && self.monorepo_packet.validate().is_empty()
    }
}

/// Errors emitted when reading the checked-in stable monorepo truth packet.
#[derive(Debug)]
pub enum MonorepoHotSetTruthArtifactError {
    /// Packet failed to parse.
    Packet(serde_json::Error),
    /// Packet failed validation.
    Validation(Vec<MonorepoTruthValidationFinding>),
}

impl fmt::Display for MonorepoHotSetTruthArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Packet(error) => write!(
                formatter,
                "monorepo hot-set truth packet parse failed: {error}"
            ),
            Self::Validation(findings) => {
                let tokens = findings
                    .iter()
                    .map(|finding| finding.finding_kind.as_str())
                    .collect::<Vec<_>>()
                    .join(",");
                write!(
                    formatter,
                    "monorepo hot-set truth packet failed validation: {tokens}"
                )
            }
        }
    }
}

impl Error for MonorepoHotSetTruthArtifactError {}

/// Returns the checked-in stable monorepo hot-set truth packet.
///
/// # Errors
///
/// Returns an artifact error if the checked-in packet does not parse or validate.
pub fn current_stable_monorepo_hot_set_truth_packet(
) -> Result<MonorepoHotSetTruthPacket, MonorepoHotSetTruthArtifactError> {
    let packet: MonorepoHotSetTruthPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/search/m4/monorepo_hot_set_truth_packet.json"
    )))
    .map_err(MonorepoHotSetTruthArtifactError::Packet)?;
    let findings = packet.validate();
    if findings.is_empty() {
        Ok(packet)
    } else {
        Err(MonorepoHotSetTruthArtifactError::Validation(findings))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_coverage() -> HotSetCoverageEstimate {
        HotSetCoverageEstimate {
            hot_set_file_count: 128,
            declared_scope_file_count: 250_000,
            deferred_cold_path_count: 249_872,
        }
    }

    fn sample_responsiveness() -> ResponsivenessInvariants {
        ResponsivenessInvariants {
            edit_input_unblocked: true,
            quick_open_first_useful_within_budget: true,
            full_index_deferred_with_disclosure: true,
            first_useful_row_ms_observed: 18,
            first_useful_row_ms_budget: 50,
        }
    }

    fn sample_row() -> MonorepoTruthRow {
        MonorepoTruthRow {
            row_id: "row:large_single_root:filename_index".to_owned(),
            archetype: MonorepoArchetypeClass::LargeSingleRoot,
            lane: IndexingLaneClass::FilenameIndex,
            workspace_id: "workspace:m4:monorepo:large_single_root".to_owned(),
            planner_version: "search-planner-stable".to_owned(),
            scope_ref: "scope:certified_archetype:large_single_root".to_owned(),
            readiness_state: SearchReadinessState::HotSetReady,
            degradation: GracefulDegradationClass::HotSetOnly,
            degradation_disclosure_ref: Some(
                "docs/search/m4/finalize-monorepo-hot-set-indexing-warming-states-and.md#hot-set-only"
                    .to_owned(),
            ),
            warming_transitions: vec![WarmingTransition {
                from_state: SearchReadinessState::NotIndexed,
                to_state: SearchReadinessState::HotSetReady,
                elapsed_ms: 14,
                emits_first_useful_row: true,
            }],
            hot_set_coverage: sample_coverage(),
            responsiveness: sample_responsiveness(),
            raw_boundary_material_excluded: true,
            captured_at: "2026-05-26T12:00:00Z".to_owned(),
        }
    }

    fn sample_projection(surface: MonorepoConsumerSurface) -> MonorepoConsumerProjection {
        MonorepoConsumerProjection {
            consumer_surface: surface,
            projection_ref: format!("projection:{}", surface.as_str()),
            monorepo_packet_id_ref: "packet:m4:monorepo_hot_set_truth".to_owned(),
            rendered_at: "2026-05-26T12:00:01Z".to_owned(),
            preserves_same_packet: true,
            preserves_lane_identity: true,
            preserves_readiness_vocabulary: true,
            preserves_degradation_labels: true,
            preserves_responsiveness_invariants: true,
            supports_json_export: true,
            raw_private_material_excluded: true,
            ambient_authority_excluded: true,
        }
    }

    fn baseline_input() -> MonorepoHotSetTruthPacketInput {
        MonorepoHotSetTruthPacketInput {
            packet_id: "packet:m4:monorepo_hot_set_truth".to_owned(),
            workflow_or_surface_id: "workflow.search.monorepo_hot_set_truth".to_owned(),
            generated_at: "2026-05-26T12:00:00Z".to_owned(),
            covered_archetypes: vec![MonorepoArchetypeClass::LargeSingleRoot],
            covered_lanes: vec![IndexingLaneClass::FilenameIndex],
            rows: vec![sample_row()],
            consumer_projections: MonorepoConsumerSurface::REQUIRED
                .iter()
                .copied()
                .map(sample_projection)
                .collect(),
            source_contract_refs: vec![MONOREPO_HOT_SET_TRUTH_DOC_REF.to_owned()],
        }
    }

    #[test]
    fn closed_tokens_are_pinned() {
        assert_eq!(
            MonorepoArchetypeClass::VeryLargeMonorepo.as_str(),
            "very_large_monorepo"
        );
        assert_eq!(IndexingLaneClass::SymbolIndex.as_str(), "symbol_index");
        assert_eq!(
            GracefulDegradationClass::KnownPathsFallback.as_str(),
            "known_paths_fallback"
        );
        assert_eq!(
            MonorepoTruthFindingKind::EditInputBlockedByWarmup.as_str(),
            "edit_input_blocked_by_warmup"
        );
        assert_eq!(
            MonorepoTruthPromotionState::BlocksStable.as_str(),
            "blocks_stable"
        );
    }

    #[test]
    fn baseline_input_materializes_stable() {
        let packet = MonorepoHotSetTruthPacket::materialize(baseline_input());
        assert_eq!(packet.promotion_state, MonorepoTruthPromotionState::Stable);
        assert!(packet.validation_findings.is_empty());
        assert!(packet.is_stable());
    }

    #[test]
    fn missing_warming_transition_blocks_stable() {
        let mut row = sample_row();
        row.warming_transitions.clear();
        let mut input = baseline_input();
        input.rows = vec![row];
        let packet = MonorepoHotSetTruthPacket::materialize(input);
        assert_eq!(
            packet.promotion_state,
            MonorepoTruthPromotionState::BlocksStable
        );
        assert!(packet
            .validation_findings
            .iter()
            .any(|finding| finding.finding_kind == MonorepoTruthFindingKind::MissingWarmingTransition));
    }

    #[test]
    fn degradation_without_disclosure_blocks_stable() {
        let mut row = sample_row();
        row.degradation_disclosure_ref = None;
        let mut input = baseline_input();
        input.rows = vec![row];
        let packet = MonorepoHotSetTruthPacket::materialize(input);
        assert_eq!(
            packet.promotion_state,
            MonorepoTruthPromotionState::BlocksStable
        );
        assert!(packet
            .validation_findings
            .iter()
            .any(|finding| finding.finding_kind == MonorepoTruthFindingKind::DegradationNotLabeled));
    }

    #[test]
    fn edit_input_blocked_blocks_stable() {
        let mut row = sample_row();
        row.responsiveness.edit_input_unblocked = false;
        let mut input = baseline_input();
        input.rows = vec![row];
        let packet = MonorepoHotSetTruthPacket::materialize(input);
        assert_eq!(
            packet.promotion_state,
            MonorepoTruthPromotionState::BlocksStable
        );
        assert!(packet
            .validation_findings
            .iter()
            .any(|finding| finding.finding_kind == MonorepoTruthFindingKind::EditInputBlockedByWarmup));
    }

    #[test]
    fn first_useful_row_breach_blocks_stable() {
        let mut row = sample_row();
        row.responsiveness.first_useful_row_ms_observed = 999;
        let mut input = baseline_input();
        input.rows = vec![row];
        let packet = MonorepoHotSetTruthPacket::materialize(input);
        assert_eq!(
            packet.promotion_state,
            MonorepoTruthPromotionState::BlocksStable
        );
        assert!(packet.validation_findings.iter().any(|finding| {
            finding.finding_kind == MonorepoTruthFindingKind::FirstUsefulRowBudgetMissed
        }));
    }

    #[test]
    fn missing_consumer_projection_blocks_stable() {
        let mut input = baseline_input();
        input.consumer_projections =
            vec![sample_projection(MonorepoConsumerSurface::SearchShell)];
        let packet = MonorepoHotSetTruthPacket::materialize(input);
        assert_eq!(
            packet.promotion_state,
            MonorepoTruthPromotionState::BlocksStable
        );
        assert!(packet.validation_findings.iter().any(|finding| {
            finding.finding_kind == MonorepoTruthFindingKind::MissingConsumerProjection
        }));
    }

    #[test]
    fn projection_drops_degradation_blocks_stable() {
        let mut input = baseline_input();
        input.consumer_projections = MonorepoConsumerSurface::REQUIRED
            .iter()
            .copied()
            .map(|surface| {
                let mut projection = sample_projection(surface);
                if surface == MonorepoConsumerSurface::DocsHelp {
                    projection.preserves_degradation_labels = false;
                }
                projection
            })
            .collect();
        let packet = MonorepoHotSetTruthPacket::materialize(input);
        assert_eq!(
            packet.promotion_state,
            MonorepoTruthPromotionState::BlocksStable
        );
        assert!(packet.validation_findings.iter().any(|finding| {
            finding.finding_kind == MonorepoTruthFindingKind::DegradationVocabularyDropped
        }));
    }
}
