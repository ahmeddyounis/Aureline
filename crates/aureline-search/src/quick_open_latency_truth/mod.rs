//! Stable quick-open, file/symbol/command-search latency and partial-index
//! truth packets for certified workspace archetypes.
//!
//! This module is the search-owned contract for the M4 stable lane: each
//! claimed certified archetype has one packet that proves quick-open,
//! file-search, symbol-search, and command-palette latency budgets and
//! partial-index truth before useful-before-ready answers ship to product
//! surfaces. The packet is intentionally metadata-only — it carries no raw
//! query text, raw source bodies, provider payloads, secrets, or private
//! numeric rank weights — and binds the durable query-session identity,
//! readiness transitions, and consumer projections that quick-open, file
//! search, symbol search, command palette, support export, and the
//! CLI/headless inspector must all read instead of inventing local copies.
//!
//! The packet is consumed by the search shell, the docs/help surface, the
//! support export, and the release proof index. Materialization derives a
//! promotion state from the validation findings; surfaces preserve the same
//! `query_session_id` across row virtualization, preview-pane open/close,
//! and ranking-refinement passes so support export, AI context packets, CLI
//! output, and replay fixtures all talk about the same search event.

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

use crate::query_session::SearchSurface;

/// Stable record-kind tag for [`QuickOpenLatencyTruthPacket`].
pub const QUICK_OPEN_LATENCY_TRUTH_PACKET_RECORD_KIND: &str =
    "quick_open_latency_truth_stable_packet";

/// Stable record-kind tag for [`QuickOpenLatencyTruthSupportExport`].
pub const QUICK_OPEN_LATENCY_TRUTH_SUPPORT_EXPORT_RECORD_KIND: &str =
    "quick_open_latency_truth_support_export";

/// Integer schema version for stable quick-open latency-truth packets.
pub const QUICK_OPEN_LATENCY_TRUTH_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the boundary schema.
pub const QUICK_OPEN_LATENCY_TRUTH_SCHEMA_REF: &str =
    "schemas/search/quick_open_latency_truth.schema.json";

/// Repo-relative path of the reviewer doc.
pub const QUICK_OPEN_LATENCY_TRUTH_DOC_REF: &str =
    "docs/search/m4/finalize-quick-open-file-symbol-command-search-latency.md";

/// Repo-relative path of the protected fixture corpus directory.
pub const QUICK_OPEN_LATENCY_TRUTH_FIXTURE_DIR: &str =
    "fixtures/search/m4/quick_open_latency_truth";

/// Repo-relative path of the checked-in stable latency-truth packet.
pub const QUICK_OPEN_LATENCY_TRUTH_PACKET_ARTIFACT_REF: &str =
    "artifacts/search/m4/quick_open_latency_truth_packet.json";

/// Repo-relative path of the human-readable reviewer artifact.
pub const QUICK_OPEN_LATENCY_TRUTH_ARTIFACT_DOC_REF: &str =
    "artifacts/search/m4/finalize-quick-open-file-symbol-command-search-latency.md";

/// Certified workspace archetype that owns one latency truth row.
///
/// The list mirrors the certified-archetype scorecard register so search
/// latency rows bind to the same archetype identity the release-control lane
/// already publishes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CertifiedArchetypeClass {
    /// TypeScript or JavaScript web app or service.
    TypescriptJavascriptWeb,
    /// Python service or data app.
    PythonServiceOrDataApp,
    /// Rust workspace.
    RustWorkspace,
    /// Go service or monorepo slice.
    GoServiceOrMonorepoSlice,
    /// Java or Kotlin service.
    JavaOrKotlinService,
    /// C or C++ native project.
    COrCppNativeProject,
}

impl CertifiedArchetypeClass {
    /// Every certified archetype, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::TypescriptJavascriptWeb,
        Self::PythonServiceOrDataApp,
        Self::RustWorkspace,
        Self::GoServiceOrMonorepoSlice,
        Self::JavaOrKotlinService,
        Self::COrCppNativeProject,
    ];

    /// Stable token used in fixtures, schemas, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::TypescriptJavascriptWeb => "typescript_javascript_web",
            Self::PythonServiceOrDataApp => "python_service_or_data_app",
            Self::RustWorkspace => "rust_workspace",
            Self::GoServiceOrMonorepoSlice => "go_service_or_monorepo_slice",
            Self::JavaOrKotlinService => "java_or_kotlin_service",
            Self::COrCppNativeProject => "c_or_cpp_native_project",
        }
    }
}

/// Latency surface that must publish budgets and partial-index truth.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LatencySurface {
    /// Quick-open jump surface for files, recent places, and symbols.
    QuickOpen,
    /// Full workspace file and text search.
    FileSearch,
    /// Symbol and structural-navigation search.
    SymbolSearch,
    /// Command palette and action search.
    CommandPalette,
}

impl LatencySurface {
    /// Every governed surface, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::QuickOpen,
        Self::FileSearch,
        Self::SymbolSearch,
        Self::CommandPalette,
    ];

    /// Stable token used in fixtures, schemas, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::QuickOpen => "quick_open",
            Self::FileSearch => "file_search",
            Self::SymbolSearch => "symbol_search",
            Self::CommandPalette => "command_palette",
        }
    }

    /// Projects this latency surface into a query-session [`SearchSurface`], when one is defined.
    pub const fn matching_query_session_surface(self) -> Option<SearchSurface> {
        match self {
            Self::QuickOpen => Some(SearchSurface::QuickOpen),
            Self::FileSearch => Some(SearchSurface::FileSearch),
            Self::SymbolSearch => Some(SearchSurface::SymbolSearch),
            Self::CommandPalette => None,
        }
    }
}

/// Session readiness state shown to product, AI, support, and replay surfaces.
///
/// Stable search rows MUST keep these states visible and exportable; they MUST
/// NOT collapse `warming`, `partial`, `blocked`, `withheld`, `provider_limited`,
/// or `policy_limited` into a generic loading spinner or success badge.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SessionReadinessState {
    /// Session is fully warm for the claimed scope.
    Ready,
    /// Hot-set rows are ready while cold indexing continues.
    HotSetReady,
    /// Session is still warming its indexes.
    Warming,
    /// Session answered with partial coverage and labeled the gap.
    Partial,
    /// Session is blocked behind a dependency or capability.
    Blocked,
    /// Session was withheld by trust, license, or admin policy.
    Withheld,
    /// Provider-side capacity or rate limited the session.
    ProviderLimited,
    /// Policy or workset narrowed the answer below the claimed scope.
    PolicyLimited,
}

impl SessionReadinessState {
    /// Stable token used in fixtures, schemas, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Ready => "ready",
            Self::HotSetReady => "hot_set_ready",
            Self::Warming => "warming",
            Self::Partial => "partial",
            Self::Blocked => "blocked",
            Self::Withheld => "withheld",
            Self::ProviderLimited => "provider_limited",
            Self::PolicyLimited => "policy_limited",
        }
    }

    /// True when this state must remain visibly disclosed instead of being
    /// flattened into a generic loading or success badge.
    pub const fn requires_explicit_disclosure(self) -> bool {
        !matches!(self, Self::Ready)
    }

    /// True when state implies the row admits partial truth and therefore
    /// blocks destructive workspace-wide actions.
    pub const fn implies_partial_truth(self) -> bool {
        matches!(
            self,
            Self::HotSetReady
                | Self::Warming
                | Self::Partial
                | Self::Blocked
                | Self::Withheld
                | Self::ProviderLimited
                | Self::PolicyLimited
        )
    }
}

/// Partial-index truth state attached to a latency row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PartialIndexTruthClass {
    /// Index is fully built for the declared scope.
    FullyIndexed,
    /// Only hot-set rows are ready; cold paths are still warming.
    HotSetOnly,
    /// Index is partial against the declared scope; the gap is labeled.
    PartialIndex,
    /// Index has a stale shard that has not yet been refreshed.
    StaleShard,
    /// Index is unavailable and the row is intentionally narrowed.
    IndexUnavailable,
}

impl PartialIndexTruthClass {
    /// Stable token used in fixtures, schemas, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::FullyIndexed => "fully_indexed",
            Self::HotSetOnly => "hot_set_only",
            Self::PartialIndex => "partial_index",
            Self::StaleShard => "stale_shard",
            Self::IndexUnavailable => "index_unavailable",
        }
    }

    /// True when this class requires a visible downgrade label on the row.
    pub const fn requires_visible_downgrade(self) -> bool {
        !matches!(self, Self::FullyIndexed)
    }
}

/// Stable promotion state derived from packet validation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LatencyPromotionState {
    /// Packet certifies stable claim for the archetype + surface row.
    Stable,
    /// Packet must remain narrowed below stable until a recorded gap closes.
    NarrowedBelowStable,
    /// Packet has a blocker finding and cannot publish on stable surfaces.
    BlocksStable,
}

impl LatencyPromotionState {
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
pub enum LatencyFindingSeverity {
    /// Informational finding.
    Info,
    /// Reviewable finding that narrows the row below stable.
    Warning,
    /// Blocker that prevents stable publication.
    Blocker,
}

/// Closed validation-finding vocabulary for [`QuickOpenLatencyTruthPacket`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LatencyFindingKind {
    /// Record kind does not match the schema.
    WrongRecordKind,
    /// Schema version does not match the frozen schema.
    WrongSchemaVersion,
    /// Required identity field is empty.
    MissingIdentity,
    /// Required archetype × surface row is missing.
    MissingArchetypeOrSurfaceRow,
    /// Observed latency exceeded the published p50 budget.
    ObservedP50ExceededBudget,
    /// Observed latency exceeded the published p95 budget.
    ObservedP95ExceededBudget,
    /// A row reported an observed latency but no published budget.
    MissingPublishedBudget,
    /// A row reported a budget but no observed latency.
    MissingObservedLatency,
    /// A row reported no benchmark capture ref.
    MissingBenchmarkCaptureRef,
    /// A row reported a budget waiver without a ref.
    WaiverWithoutRef,
    /// A row reported a partial-index downgrade without a labeled disclosure.
    PartialIndexNotLabeled,
    /// A row reported a non-ready readiness state without a recorded transition.
    MissingReadinessTransition,
    /// A row reported a state that must remain visible but the projection collapses it.
    SessionStateCollapsed,
    /// A row reported a session id that drifts across projections.
    SessionIdNotPropagated,
    /// A required consumer projection is missing for this packet.
    MissingConsumerProjection,
    /// A consumer projection remints or drops latency truth.
    ConsumerProjectionDrift,
    /// Packet admits raw query text, raw bodies, secrets, or private weights.
    RawBoundaryMaterialPresent,
    /// Stored promotion state disagrees with derived findings.
    PromotionStateMismatch,
}

impl LatencyFindingKind {
    /// Stable token used in fixtures, schemas, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WrongRecordKind => "wrong_record_kind",
            Self::WrongSchemaVersion => "wrong_schema_version",
            Self::MissingIdentity => "missing_identity",
            Self::MissingArchetypeOrSurfaceRow => "missing_archetype_or_surface_row",
            Self::ObservedP50ExceededBudget => "observed_p50_exceeded_budget",
            Self::ObservedP95ExceededBudget => "observed_p95_exceeded_budget",
            Self::MissingPublishedBudget => "missing_published_budget",
            Self::MissingObservedLatency => "missing_observed_latency",
            Self::MissingBenchmarkCaptureRef => "missing_benchmark_capture_ref",
            Self::WaiverWithoutRef => "waiver_without_ref",
            Self::PartialIndexNotLabeled => "partial_index_not_labeled",
            Self::MissingReadinessTransition => "missing_readiness_transition",
            Self::SessionStateCollapsed => "session_state_collapsed",
            Self::SessionIdNotPropagated => "session_id_not_propagated",
            Self::MissingConsumerProjection => "missing_consumer_projection",
            Self::ConsumerProjectionDrift => "consumer_projection_drift",
            Self::RawBoundaryMaterialPresent => "raw_boundary_material_present",
            Self::PromotionStateMismatch => "promotion_state_mismatch",
        }
    }
}

/// Consumer surface that must inherit the latency packet's truth verbatim.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LatencyConsumerSurface {
    /// Search shell quick-open, file, symbol, and command-search panes.
    SearchShell,
    /// Docs/help surface explaining latency budgets and downgrade state.
    DocsHelp,
    /// CLI or headless inspection surface.
    CliHeadless,
    /// Support export bundle.
    SupportExport,
    /// Release proof index entry.
    ReleaseProofIndex,
}

impl LatencyConsumerSurface {
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

/// One validation finding emitted by the latency packet validator.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LatencyValidationFinding {
    /// Closed finding kind.
    pub finding_kind: LatencyFindingKind,
    /// Finding severity.
    pub severity: LatencyFindingSeverity,
    /// Short support-safe summary.
    pub summary: String,
}

impl LatencyValidationFinding {
    fn new(
        finding_kind: LatencyFindingKind,
        severity: LatencyFindingSeverity,
        summary: impl Into<String>,
    ) -> Self {
        Self {
            finding_kind,
            severity,
            summary: summary.into(),
        }
    }
}

/// Latency budget for one archetype × surface row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LatencyBudget {
    /// Published p50 budget in milliseconds.
    pub p50_ms_budget: u32,
    /// Published p95 budget in milliseconds.
    pub p95_ms_budget: u32,
    /// Optional waiver ref when budgets are intentionally narrowed or tightened.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub waiver_ref: Option<String>,
}

/// Observed latency capture for one archetype × surface row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LatencyObservation {
    /// Observed p50 in milliseconds.
    pub p50_ms_observed: u32,
    /// Observed p95 in milliseconds.
    pub p95_ms_observed: u32,
    /// Repo-relative benchmark-lab capture ref.
    pub benchmark_capture_ref: String,
    /// Sample size captured.
    pub sample_size: u32,
}

/// Session readiness transition observed for one latency row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SessionReadinessTransition {
    /// Previous state (typically the open state of the session).
    pub from_state: SessionReadinessState,
    /// State entered by the session.
    pub to_state: SessionReadinessState,
    /// Milliseconds since session open when the transition fired.
    pub elapsed_ms: u32,
    /// Whether the transition resulted in a user-visible first useful row.
    pub first_useful_row_emitted: bool,
}

impl SessionReadinessTransition {
    /// True when this transition crosses into a state that must remain visible.
    pub fn requires_explicit_disclosure(&self) -> bool {
        self.to_state.requires_explicit_disclosure()
    }
}

/// One row in the latency truth packet bound to an archetype × surface.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LatencyTruthRow {
    /// Stable row id within the packet.
    pub row_id: String,
    /// Certified archetype that owns this row.
    pub archetype: CertifiedArchetypeClass,
    /// Surface this row certifies for the archetype.
    pub surface: LatencySurface,
    /// Durable query-session id captured for this row.
    pub query_session_id: String,
    /// Planner version that produced the row.
    pub planner_version: String,
    /// Workset or scope id used by the captured session.
    pub scope_ref: String,
    /// Published budget for the row.
    pub budget: LatencyBudget,
    /// Observed latency capture for the row.
    pub observation: LatencyObservation,
    /// Session readiness states the row keeps visible.
    #[serde(default)]
    pub visible_readiness_states: Vec<SessionReadinessState>,
    /// Captured readiness transitions ordered by elapsed time.
    #[serde(default)]
    pub readiness_transitions: Vec<SessionReadinessTransition>,
    /// Partial-index truth class for the row.
    pub partial_index_truth: PartialIndexTruthClass,
    /// Repo-relative disclosure ref shown when the row is downgraded.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub partial_index_disclosure_ref: Option<String>,
    /// True when raw private query text, source bodies, and secrets are excluded.
    pub raw_boundary_material_excluded: bool,
    /// Capture timestamp for this row.
    pub captured_at: String,
}

impl LatencyTruthRow {
    fn budgets_met(&self) -> bool {
        self.observation.p50_ms_observed <= self.budget.p50_ms_budget
            && self.observation.p95_ms_observed <= self.budget.p95_ms_budget
    }
}

/// Consumer projection proving a surface reads this packet verbatim.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LatencyConsumerProjection {
    /// Consumer surface class.
    pub consumer_surface: LatencyConsumerSurface,
    /// Stable projection ref.
    pub projection_ref: String,
    /// Latency packet id consumed by the projection.
    pub latency_packet_id_ref: String,
    /// Rendered-at timestamp.
    pub rendered_at: String,
    /// True when the surface preserves the same packet id.
    pub preserves_same_packet: bool,
    /// True when the surface preserves the same query-session ids.
    pub preserves_query_session_ids: bool,
    /// True when readiness-state vocabulary is quoted from the packet.
    pub preserves_readiness_states: bool,
    /// True when partial-index labels are quoted from the packet.
    pub preserves_partial_index_labels: bool,
    /// True when JSON export is available from the projection.
    pub supports_json_export: bool,
    /// True when raw private material is excluded.
    pub raw_private_material_excluded: bool,
    /// True when ambient authority/credentials are excluded.
    pub ambient_authority_excluded: bool,
}

impl LatencyConsumerProjection {
    fn preserves_truth_for(&self, packet_id: &str) -> bool {
        self.latency_packet_id_ref == packet_id
            && self.preserves_same_packet
            && self.preserves_query_session_ids
            && self.preserves_readiness_states
            && self.preserves_partial_index_labels
            && self.supports_json_export
            && self.raw_private_material_excluded
            && self.ambient_authority_excluded
            && !self.projection_ref.trim().is_empty()
    }
}

/// Constructor input for [`QuickOpenLatencyTruthPacket::materialize`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct QuickOpenLatencyTruthPacketInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Claimed workflow or surface id.
    pub workflow_or_surface_id: String,
    /// Capture timestamp for the packet as a whole.
    pub generated_at: String,
    /// Certified archetypes the packet covers.
    #[serde(default)]
    pub covered_archetypes: Vec<CertifiedArchetypeClass>,
    /// Surfaces the packet covers.
    #[serde(default)]
    pub covered_surfaces: Vec<LatencySurface>,
    /// Latency rows, one per declared archetype × surface.
    #[serde(default)]
    pub rows: Vec<LatencyTruthRow>,
    /// Consumer projections preserving this packet.
    #[serde(default)]
    pub consumer_projections: Vec<LatencyConsumerProjection>,
    /// Source contracts (docs/schema/fixtures) consumed by the packet.
    #[serde(default)]
    pub source_contract_refs: Vec<String>,
}

/// Search-owned packet for quick-open / file / symbol / command-palette
/// latency, readiness, and partial-index truth on certified archetypes.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct QuickOpenLatencyTruthPacket {
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
    /// Surfaces the packet covers.
    #[serde(default)]
    pub covered_surfaces: Vec<LatencySurface>,
    /// Latency rows.
    #[serde(default)]
    pub rows: Vec<LatencyTruthRow>,
    /// Consumer projections preserving this packet.
    #[serde(default)]
    pub consumer_projections: Vec<LatencyConsumerProjection>,
    /// Source contract refs consumed by the packet.
    #[serde(default)]
    pub source_contract_refs: Vec<String>,
    /// Derived promotion state.
    pub promotion_state: LatencyPromotionState,
    /// Validation findings captured at materialization.
    #[serde(default)]
    pub validation_findings: Vec<LatencyValidationFinding>,
}

impl QuickOpenLatencyTruthPacket {
    /// Materializes a packet and records derived validation findings.
    pub fn materialize(input: QuickOpenLatencyTruthPacketInput) -> Self {
        let mut packet = Self {
            record_kind: QUICK_OPEN_LATENCY_TRUTH_PACKET_RECORD_KIND.to_owned(),
            schema_version: QUICK_OPEN_LATENCY_TRUTH_SCHEMA_VERSION,
            packet_id: input.packet_id,
            workflow_or_surface_id: input.workflow_or_surface_id,
            generated_at: input.generated_at,
            covered_archetypes: input.covered_archetypes,
            covered_surfaces: input.covered_surfaces,
            rows: input.rows,
            consumer_projections: input.consumer_projections,
            source_contract_refs: input.source_contract_refs,
            promotion_state: LatencyPromotionState::Stable,
            validation_findings: Vec::new(),
        };
        let findings = packet.derived_findings(false);
        packet.promotion_state = promotion_state_for_findings(&findings);
        packet.validation_findings = findings;
        packet
    }

    /// Re-validates the packet against stable latency-truth invariants.
    pub fn validate(&self) -> Vec<LatencyValidationFinding> {
        self.derived_findings(true)
    }

    /// Returns true when this packet has no blocker-level finding.
    pub fn is_stable(&self) -> bool {
        !self
            .validate()
            .iter()
            .any(|finding| finding.severity == LatencyFindingSeverity::Blocker)
    }

    /// Returns true when a consumer projection preserves this packet.
    pub fn has_projection_for(&self, surface: LatencyConsumerSurface) -> bool {
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

    /// Returns the unique surface tokens observed across rows.
    pub fn surface_tokens(&self) -> Vec<&'static str> {
        let mut set = BTreeSet::new();
        for row in &self.rows {
            set.insert(row.surface);
        }
        set.into_iter().map(LatencySurface::as_str).collect()
    }

    /// Returns the unique partial-index truth tokens observed across rows.
    pub fn partial_index_truth_tokens(&self) -> Vec<&'static str> {
        let mut set = BTreeSet::new();
        for row in &self.rows {
            set.insert(row.partial_index_truth);
        }
        set.into_iter()
            .map(PartialIndexTruthClass::as_str)
            .collect()
    }

    /// Returns the unique session-readiness-state tokens kept visible across rows.
    pub fn visible_readiness_state_tokens(&self) -> Vec<&'static str> {
        let mut set = BTreeSet::new();
        for row in &self.rows {
            for state in &row.visible_readiness_states {
                set.insert(*state);
            }
        }
        set.into_iter().map(SessionReadinessState::as_str).collect()
    }

    /// Builds a support export wrapping the exact packet shown to product surfaces.
    pub fn support_export(
        &self,
        export_id: impl Into<String>,
        exported_at: impl Into<String>,
    ) -> QuickOpenLatencyTruthSupportExport {
        QuickOpenLatencyTruthSupportExport {
            record_kind: QUICK_OPEN_LATENCY_TRUTH_SUPPORT_EXPORT_RECORD_KIND.to_owned(),
            schema_version: QUICK_OPEN_LATENCY_TRUTH_SCHEMA_VERSION,
            export_id: export_id.into(),
            latency_packet_id_ref: self.packet_id.clone(),
            exported_at: exported_at.into(),
            raw_private_material_excluded: true,
            ambient_authority_excluded: true,
            latency_packet: self.clone(),
        }
    }

    fn derived_findings(&self, include_record_fields: bool) -> Vec<LatencyValidationFinding> {
        let mut findings = Vec::new();

        if include_record_fields && self.record_kind != QUICK_OPEN_LATENCY_TRUTH_PACKET_RECORD_KIND
        {
            findings.push(LatencyValidationFinding::new(
                LatencyFindingKind::WrongRecordKind,
                LatencyFindingSeverity::Blocker,
                "quick-open latency-truth packet has the wrong record kind",
            ));
        }
        if include_record_fields && self.schema_version != QUICK_OPEN_LATENCY_TRUTH_SCHEMA_VERSION {
            findings.push(LatencyValidationFinding::new(
                LatencyFindingKind::WrongSchemaVersion,
                LatencyFindingSeverity::Blocker,
                "quick-open latency-truth packet has the wrong schema version",
            ));
        }
        if self.packet_id.trim().is_empty()
            || self.workflow_or_surface_id.trim().is_empty()
            || self.generated_at.trim().is_empty()
        {
            findings.push(LatencyValidationFinding::new(
                LatencyFindingKind::MissingIdentity,
                LatencyFindingSeverity::Blocker,
                "packet, workflow, and timestamp refs are required",
            ));
        }

        if self.covered_archetypes.is_empty() || self.covered_surfaces.is_empty() {
            findings.push(LatencyValidationFinding::new(
                LatencyFindingKind::MissingArchetypeOrSurfaceRow,
                LatencyFindingSeverity::Blocker,
                "packet must declare covered archetypes and surfaces",
            ));
        }

        for archetype in &self.covered_archetypes {
            for surface in &self.covered_surfaces {
                let present = self
                    .rows
                    .iter()
                    .any(|row| row.archetype == *archetype && row.surface == *surface);
                if !present {
                    findings.push(LatencyValidationFinding::new(
                        LatencyFindingKind::MissingArchetypeOrSurfaceRow,
                        LatencyFindingSeverity::Blocker,
                        format!(
                            "no row covers archetype {} on surface {}",
                            archetype.as_str(),
                            surface.as_str()
                        ),
                    ));
                }
            }
        }

        let mut session_ids_by_surface: std::collections::BTreeMap<
            (LatencySurface, String),
            BTreeSet<String>,
        > = std::collections::BTreeMap::new();

        for row in &self.rows {
            if row.row_id.trim().is_empty()
                || row.query_session_id.trim().is_empty()
                || row.planner_version.trim().is_empty()
                || row.scope_ref.trim().is_empty()
                || row.captured_at.trim().is_empty()
            {
                findings.push(LatencyValidationFinding::new(
                    LatencyFindingKind::MissingIdentity,
                    LatencyFindingSeverity::Blocker,
                    format!(
                        "row {} identity, session, scope, planner, or timestamp is empty",
                        row.row_id
                    ),
                ));
            }
            if row.budget.p50_ms_budget == 0 || row.budget.p95_ms_budget == 0 {
                findings.push(LatencyValidationFinding::new(
                    LatencyFindingKind::MissingPublishedBudget,
                    LatencyFindingSeverity::Blocker,
                    format!("row {} has no published p50/p95 budget", row.row_id),
                ));
            }
            if row.budget.waiver_ref.as_deref().is_some_and(str::is_empty) {
                findings.push(LatencyValidationFinding::new(
                    LatencyFindingKind::WaiverWithoutRef,
                    LatencyFindingSeverity::Blocker,
                    format!("row {} declares a budget waiver without a ref", row.row_id),
                ));
            }
            if row.observation.sample_size == 0 {
                findings.push(LatencyValidationFinding::new(
                    LatencyFindingKind::MissingObservedLatency,
                    LatencyFindingSeverity::Blocker,
                    format!("row {} has no observed sample size", row.row_id),
                ));
            }
            if row.observation.benchmark_capture_ref.trim().is_empty() {
                findings.push(LatencyValidationFinding::new(
                    LatencyFindingKind::MissingBenchmarkCaptureRef,
                    LatencyFindingSeverity::Blocker,
                    format!("row {} has no benchmark capture ref", row.row_id),
                ));
            }
            if row.budget.p50_ms_budget > 0
                && row.observation.p50_ms_observed > row.budget.p50_ms_budget
                && row.budget.waiver_ref.is_none()
            {
                findings.push(LatencyValidationFinding::new(
                    LatencyFindingKind::ObservedP50ExceededBudget,
                    LatencyFindingSeverity::Blocker,
                    format!(
                        "row {} observed p50 {}ms exceeds published budget {}ms without waiver",
                        row.row_id, row.observation.p50_ms_observed, row.budget.p50_ms_budget
                    ),
                ));
            }
            if row.budget.p95_ms_budget > 0
                && row.observation.p95_ms_observed > row.budget.p95_ms_budget
                && row.budget.waiver_ref.is_none()
            {
                findings.push(LatencyValidationFinding::new(
                    LatencyFindingKind::ObservedP95ExceededBudget,
                    LatencyFindingSeverity::Blocker,
                    format!(
                        "row {} observed p95 {}ms exceeds published budget {}ms without waiver",
                        row.row_id, row.observation.p95_ms_observed, row.budget.p95_ms_budget
                    ),
                ));
            }
            if row.partial_index_truth.requires_visible_downgrade()
                && row.partial_index_disclosure_ref.is_none()
            {
                findings.push(LatencyValidationFinding::new(
                    LatencyFindingKind::PartialIndexNotLabeled,
                    LatencyFindingSeverity::Blocker,
                    format!(
                        "row {} has partial-index state {} without a disclosure ref",
                        row.row_id,
                        row.partial_index_truth.as_str()
                    ),
                ));
            }
            if row
                .visible_readiness_states
                .iter()
                .any(|state| state.requires_explicit_disclosure())
                && row.readiness_transitions.is_empty()
            {
                findings.push(LatencyValidationFinding::new(
                    LatencyFindingKind::MissingReadinessTransition,
                    LatencyFindingSeverity::Blocker,
                    format!(
                        "row {} keeps a non-ready state visible but records no readiness transition",
                        row.row_id
                    ),
                ));
            }
            if !row.raw_boundary_material_excluded {
                findings.push(LatencyValidationFinding::new(
                    LatencyFindingKind::RawBoundaryMaterialPresent,
                    LatencyFindingSeverity::Blocker,
                    format!(
                        "row {} admits raw query text, source bodies, or private weights",
                        row.row_id
                    ),
                ));
            }
            if !row.budgets_met() && row.budget.waiver_ref.is_some() {
                // tracked as Warning so the row still narrows below stable
                findings.push(LatencyValidationFinding::new(
                    LatencyFindingKind::ObservedP95ExceededBudget,
                    LatencyFindingSeverity::Warning,
                    format!(
                        "row {} relies on waiver {}",
                        row.row_id,
                        row.budget.waiver_ref.as_deref().unwrap_or("<missing>"),
                    ),
                ));
            }
            session_ids_by_surface
                .entry((row.surface, row.scope_ref.clone()))
                .or_default()
                .insert(row.query_session_id.clone());
        }

        for required_surface in LatencyConsumerSurface::REQUIRED {
            if !self.has_projection_for(required_surface) {
                findings.push(LatencyValidationFinding::new(
                    LatencyFindingKind::MissingConsumerProjection,
                    LatencyFindingSeverity::Blocker,
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
                findings.push(LatencyValidationFinding::new(
                    LatencyFindingKind::ConsumerProjectionDrift,
                    LatencyFindingSeverity::Blocker,
                    format!(
                        "projection {} does not preserve latency truth",
                        projection.projection_ref
                    ),
                ));
            }
            if !projection.preserves_query_session_ids {
                findings.push(LatencyValidationFinding::new(
                    LatencyFindingKind::SessionIdNotPropagated,
                    LatencyFindingSeverity::Blocker,
                    format!(
                        "projection {} drops query-session id propagation",
                        projection.projection_ref
                    ),
                ));
            }
            if !projection.preserves_readiness_states {
                findings.push(LatencyValidationFinding::new(
                    LatencyFindingKind::SessionStateCollapsed,
                    LatencyFindingSeverity::Blocker,
                    format!(
                        "projection {} collapses session readiness states",
                        projection.projection_ref
                    ),
                ));
            }
        }

        if include_record_fields {
            let mut without_promotion = findings.clone();
            without_promotion.retain(|finding| {
                finding.finding_kind != LatencyFindingKind::PromotionStateMismatch
            });
            let derived = promotion_state_for_findings(&without_promotion);
            if self.promotion_state != derived {
                findings.push(LatencyValidationFinding::new(
                    LatencyFindingKind::PromotionStateMismatch,
                    LatencyFindingSeverity::Blocker,
                    "stored promotion state does not match derived findings",
                ));
            }
        }

        findings
    }
}

fn promotion_state_for_findings(findings: &[LatencyValidationFinding]) -> LatencyPromotionState {
    if findings
        .iter()
        .any(|finding| finding.severity == LatencyFindingSeverity::Blocker)
    {
        LatencyPromotionState::BlocksStable
    } else if findings
        .iter()
        .any(|finding| finding.severity == LatencyFindingSeverity::Warning)
    {
        LatencyPromotionState::NarrowedBelowStable
    } else {
        LatencyPromotionState::Stable
    }
}

/// Support-export wrapper that preserves the product latency packet verbatim.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct QuickOpenLatencyTruthSupportExport {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Stable export id.
    pub export_id: String,
    /// Packet id preserved by the export.
    pub latency_packet_id_ref: String,
    /// Export timestamp.
    pub exported_at: String,
    /// True when raw private material is excluded.
    pub raw_private_material_excluded: bool,
    /// True when ambient credentials/authority are excluded.
    pub ambient_authority_excluded: bool,
    /// Exact product packet preserved by the export.
    pub latency_packet: QuickOpenLatencyTruthPacket,
}

impl QuickOpenLatencyTruthSupportExport {
    /// Returns true when the export preserves the same packet id safely.
    pub fn is_export_safe(&self) -> bool {
        self.record_kind == QUICK_OPEN_LATENCY_TRUTH_SUPPORT_EXPORT_RECORD_KIND
            && self.schema_version == QUICK_OPEN_LATENCY_TRUTH_SCHEMA_VERSION
            && self.latency_packet_id_ref == self.latency_packet.packet_id
            && self.raw_private_material_excluded
            && self.ambient_authority_excluded
            && self.latency_packet.validate().is_empty()
    }
}

/// Errors emitted when reading the checked-in stable latency-truth packet.
#[derive(Debug)]
pub enum QuickOpenLatencyTruthArtifactError {
    /// Packet failed to parse.
    Packet(serde_json::Error),
    /// Packet failed validation.
    Validation(Vec<LatencyValidationFinding>),
}

impl fmt::Display for QuickOpenLatencyTruthArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Packet(error) => write!(
                formatter,
                "quick-open latency-truth packet parse failed: {error}"
            ),
            Self::Validation(findings) => {
                let tokens = findings
                    .iter()
                    .map(|finding| finding.finding_kind.as_str())
                    .collect::<Vec<_>>()
                    .join(",");
                write!(
                    formatter,
                    "quick-open latency-truth packet failed validation: {tokens}"
                )
            }
        }
    }
}

impl Error for QuickOpenLatencyTruthArtifactError {}

/// Returns the checked-in stable latency-truth packet.
///
/// # Errors
///
/// Returns an artifact error if the checked-in packet does not parse or validate.
pub fn current_stable_quick_open_latency_truth_packet(
) -> Result<QuickOpenLatencyTruthPacket, QuickOpenLatencyTruthArtifactError> {
    let packet: QuickOpenLatencyTruthPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/search/m4/quick_open_latency_truth_packet.json"
    )))
    .map_err(QuickOpenLatencyTruthArtifactError::Packet)?;
    let findings = packet.validate();
    if findings.is_empty() {
        Ok(packet)
    } else {
        Err(QuickOpenLatencyTruthArtifactError::Validation(findings))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_budget(p50: u32, p95: u32) -> LatencyBudget {
        LatencyBudget {
            p50_ms_budget: p50,
            p95_ms_budget: p95,
            waiver_ref: None,
        }
    }

    fn sample_observation(p50: u32, p95: u32) -> LatencyObservation {
        LatencyObservation {
            p50_ms_observed: p50,
            p95_ms_observed: p95,
            benchmark_capture_ref: "benchmarks/search/quick_open/rust_workspace.json".to_owned(),
            sample_size: 500,
        }
    }

    fn sample_row() -> LatencyTruthRow {
        LatencyTruthRow {
            row_id: "row:quick_open:rust_workspace".to_owned(),
            archetype: CertifiedArchetypeClass::RustWorkspace,
            surface: LatencySurface::QuickOpen,
            query_session_id: "search:session:m4:rust_workspace:quick_open".to_owned(),
            planner_version: "search-planner-stable".to_owned(),
            scope_ref: "scope:current_repo".to_owned(),
            budget: sample_budget(40, 120),
            observation: sample_observation(28, 110),
            visible_readiness_states: vec![
                SessionReadinessState::HotSetReady,
                SessionReadinessState::Ready,
            ],
            readiness_transitions: vec![SessionReadinessTransition {
                from_state: SessionReadinessState::Warming,
                to_state: SessionReadinessState::HotSetReady,
                elapsed_ms: 18,
                first_useful_row_emitted: true,
            }],
            partial_index_truth: PartialIndexTruthClass::HotSetOnly,
            partial_index_disclosure_ref: Some(
                "docs/search/m4/finalize-quick-open-file-symbol-command-search-latency.md#hot-set"
                    .to_owned(),
            ),
            raw_boundary_material_excluded: true,
            captured_at: "2026-05-26T12:00:00Z".to_owned(),
        }
    }

    fn sample_projection(surface: LatencyConsumerSurface) -> LatencyConsumerProjection {
        LatencyConsumerProjection {
            consumer_surface: surface,
            projection_ref: format!("projection:{}", surface.as_str()),
            latency_packet_id_ref: "packet:m4:quick_open_latency_truth".to_owned(),
            rendered_at: "2026-05-26T12:00:01Z".to_owned(),
            preserves_same_packet: true,
            preserves_query_session_ids: true,
            preserves_readiness_states: true,
            preserves_partial_index_labels: true,
            supports_json_export: true,
            raw_private_material_excluded: true,
            ambient_authority_excluded: true,
        }
    }

    #[test]
    fn closed_tokens_are_pinned() {
        assert_eq!(
            CertifiedArchetypeClass::RustWorkspace.as_str(),
            "rust_workspace"
        );
        assert_eq!(LatencySurface::CommandPalette.as_str(), "command_palette");
        assert_eq!(SessionReadinessState::Partial.as_str(), "partial");
        assert_eq!(PartialIndexTruthClass::StaleShard.as_str(), "stale_shard");
        assert_eq!(
            LatencyPromotionState::BlocksStable.as_str(),
            "blocks_stable"
        );
    }

    #[test]
    fn budget_exceeded_without_waiver_blocks_promotion() {
        let mut row = sample_row();
        row.observation.p95_ms_observed = 999;
        let input = QuickOpenLatencyTruthPacketInput {
            packet_id: "packet:m4:quick_open_latency_truth".to_owned(),
            workflow_or_surface_id: "workflow.search.quick_open_latency_truth".to_owned(),
            generated_at: "2026-05-26T12:00:00Z".to_owned(),
            covered_archetypes: vec![CertifiedArchetypeClass::RustWorkspace],
            covered_surfaces: vec![LatencySurface::QuickOpen],
            rows: vec![row],
            consumer_projections: LatencyConsumerSurface::REQUIRED
                .iter()
                .copied()
                .map(sample_projection)
                .collect(),
            source_contract_refs: vec![QUICK_OPEN_LATENCY_TRUTH_DOC_REF.to_owned()],
        };
        let packet = QuickOpenLatencyTruthPacket::materialize(input);
        assert_eq!(packet.promotion_state, LatencyPromotionState::BlocksStable);
        assert!(packet
            .validation_findings
            .iter()
            .any(|finding| finding.finding_kind == LatencyFindingKind::ObservedP95ExceededBudget));
    }

    #[test]
    fn partial_index_state_without_disclosure_blocks_promotion() {
        let mut row = sample_row();
        row.partial_index_truth = PartialIndexTruthClass::PartialIndex;
        row.partial_index_disclosure_ref = None;
        let input = QuickOpenLatencyTruthPacketInput {
            packet_id: "packet:m4:quick_open_latency_truth".to_owned(),
            workflow_or_surface_id: "workflow.search.quick_open_latency_truth".to_owned(),
            generated_at: "2026-05-26T12:00:00Z".to_owned(),
            covered_archetypes: vec![CertifiedArchetypeClass::RustWorkspace],
            covered_surfaces: vec![LatencySurface::QuickOpen],
            rows: vec![row],
            consumer_projections: LatencyConsumerSurface::REQUIRED
                .iter()
                .copied()
                .map(sample_projection)
                .collect(),
            source_contract_refs: vec![QUICK_OPEN_LATENCY_TRUTH_DOC_REF.to_owned()],
        };
        let packet = QuickOpenLatencyTruthPacket::materialize(input);
        assert_eq!(packet.promotion_state, LatencyPromotionState::BlocksStable);
        assert!(packet
            .validation_findings
            .iter()
            .any(|finding| finding.finding_kind == LatencyFindingKind::PartialIndexNotLabeled));
    }

    #[test]
    fn missing_consumer_projection_blocks_promotion() {
        let input = QuickOpenLatencyTruthPacketInput {
            packet_id: "packet:m4:quick_open_latency_truth".to_owned(),
            workflow_or_surface_id: "workflow.search.quick_open_latency_truth".to_owned(),
            generated_at: "2026-05-26T12:00:00Z".to_owned(),
            covered_archetypes: vec![CertifiedArchetypeClass::RustWorkspace],
            covered_surfaces: vec![LatencySurface::QuickOpen],
            rows: vec![sample_row()],
            consumer_projections: vec![sample_projection(LatencyConsumerSurface::SearchShell)],
            source_contract_refs: vec![QUICK_OPEN_LATENCY_TRUTH_DOC_REF.to_owned()],
        };
        let packet = QuickOpenLatencyTruthPacket::materialize(input);
        assert_eq!(packet.promotion_state, LatencyPromotionState::BlocksStable);
        assert!(packet
            .validation_findings
            .iter()
            .any(|finding| finding.finding_kind == LatencyFindingKind::MissingConsumerProjection));
    }

    #[test]
    fn session_state_collapse_blocks_promotion() {
        let input = QuickOpenLatencyTruthPacketInput {
            packet_id: "packet:m4:quick_open_latency_truth".to_owned(),
            workflow_or_surface_id: "workflow.search.quick_open_latency_truth".to_owned(),
            generated_at: "2026-05-26T12:00:00Z".to_owned(),
            covered_archetypes: vec![CertifiedArchetypeClass::RustWorkspace],
            covered_surfaces: vec![LatencySurface::QuickOpen],
            rows: vec![sample_row()],
            consumer_projections: LatencyConsumerSurface::REQUIRED
                .iter()
                .copied()
                .map(|surface| {
                    let mut projection = sample_projection(surface);
                    if surface == LatencyConsumerSurface::DocsHelp {
                        projection.preserves_readiness_states = false;
                    }
                    projection
                })
                .collect(),
            source_contract_refs: vec![QUICK_OPEN_LATENCY_TRUTH_DOC_REF.to_owned()],
        };
        let packet = QuickOpenLatencyTruthPacket::materialize(input);
        assert_eq!(packet.promotion_state, LatencyPromotionState::BlocksStable);
        assert!(packet
            .validation_findings
            .iter()
            .any(|finding| finding.finding_kind == LatencyFindingKind::SessionStateCollapsed));
    }
}
