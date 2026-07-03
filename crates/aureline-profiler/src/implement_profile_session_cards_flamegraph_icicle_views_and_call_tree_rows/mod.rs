//! Reusable profile-session cards, flamegraph/icicle views, and call-tree rows.
//!
//! This module is the M05-797 component contract tying the M5 profiler UI
//! primitives to checked-in fixtures. It keeps capture mode, execution origin,
//! build/runtime identity, artifact origin, mapping quality, thread/process
//! context, zoom/filter state, and caller/callee navigation visible across the
//! hotspot workspace and secondary consumers.

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

/// Schema version stamped on the M05-797 component packet.
pub const PROFILE_HOTPATH_COMPONENT_SCHEMA_VERSION: u32 = 1;

/// Stable record-kind tag for [`ProfileHotpathComponentPacket`].
pub const PROFILE_HOTPATH_COMPONENT_RECORD_KIND: &str = "m5_profile_hotpath_component_packet";

/// Repo-relative path to the checked-in M05-797 packet.
pub const PROFILE_HOTPATH_COMPONENT_PACKET_PATH: &str =
    "artifacts/perf/m5/m5-profile-session-hotpath-components.json";

/// Embedded checked-in M05-797 packet JSON.
pub const PROFILE_HOTPATH_COMPONENT_PACKET_JSON: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../artifacts/perf/m5/m5-profile-session-hotpath-components.json"
));

/// Stable profile kind shown by session cards.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ProfileKind {
    Cpu,
    WallTime,
    Allocation,
    Heap,
    Trace,
    Replay,
}

/// Shared capture-mode vocabulary from the M5 profiler/topology matrix.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum UiCaptureMode {
    SampleCpu,
    InstrumentedCpu,
    Allocation,
    HeapSnapshot,
    WallTime,
    Trace,
    ReplayImport,
    ImportedProfile,
}

/// Shared artifact-origin vocabulary from the M5 profiler/topology matrix.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ArtifactOrigin {
    LiveCapture,
    ReplayCapture,
    ImportedArtifact,
    SupportBundle,
    CachedReplay,
    Unknown,
}

impl ArtifactOrigin {
    /// Returns true when the artifact was captured live in the current runtime.
    pub const fn is_live(self) -> bool {
        matches!(self, Self::LiveCapture)
    }
}

/// Shared execution-origin vocabulary from the M5 profiler/topology matrix.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ExecutionOrigin {
    LocalDesktop,
    SshRemote,
    ContainerWorkspace,
    ManagedWorkspace,
    BrowserRuntime,
    CiRunner,
    ImportedArtifact,
    SupportBundle,
    CliHeadless,
}

/// Shared mapping-quality vocabulary from the M5 profiler/topology matrix.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum UiMappingQuality {
    Exact,
    Symbolicated,
    SourceMapped,
    Partial,
    Heuristic,
    Missing,
    NotApplicable,
}

impl UiMappingQuality {
    /// Returns true when a source jump may be offered with visible quality state.
    pub const fn allows_source_navigation(self) -> bool {
        matches!(self, Self::Exact | Self::Symbolicated | Self::SourceMapped)
    }

    /// Returns true when the view must disclose degraded mapping quality.
    pub const fn is_degraded(self) -> bool {
        matches!(self, Self::Partial | Self::Heuristic | Self::Missing)
    }
}

/// Baseline environment state shown before compare/regression claims.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BaselineEnvironmentState {
    Comparable,
    ComparableWithDeltas,
    Incomparable,
    BaselineMissing,
    ThresholdPending,
    Waived,
}

/// Threshold state shown before compare/regression claims.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum UiThresholdState {
    NotApplicable,
    WithinThreshold,
    Regression,
    Improvement,
    ThresholdPending,
    Waived,
}

/// Surface using a reusable M05-797 component.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ComponentConsumerSurface {
    DesktopProfilerWorkspace,
    HotspotWorkspace,
    TraceViewer,
    HeapAnalysis,
    ProfileCompare,
    ReviewWorkspace,
    AiContextPanel,
    IncidentWorkspace,
    CliHeadless,
    SupportExport,
    ReleaseProof,
}

/// Flamegraph-family view mode.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ProfileCostViewMode {
    Flamegraph,
    Icicle,
    CallTree,
    HeavyView,
    HeapDiff,
    AllocationDiff,
    RetainedSizeDiff,
}

/// Metric used by a profile-cost view.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ProfileMetric {
    Samples,
    CpuTime,
    WallTime,
    Allocations,
    Bytes,
    RetainedBytes,
}

/// Whether the view is currently emphasizing self, inclusive, or both metrics.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MetricPresentation {
    SelfMetric,
    InclusiveMetric,
    SelfAndInclusive,
}

/// Symbolization state for a call-tree frame.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SymbolizationState {
    Symbolicated,
    SourceMapped,
    Partial,
    Missing,
}

impl SymbolizationState {
    /// Returns true when the row carries a useful symbolization disclosure.
    pub const fn is_disclosed(self) -> bool {
        !matches!(self, Self::Missing)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TargetIdentity {
    pub process_ref: String,
    pub config_ref: String,
    pub display_label: String,
    pub raw_command_line_exported: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BaselineDisclosure {
    pub baseline_ref: String,
    pub environment_state: BaselineEnvironmentState,
    #[serde(default)]
    pub environment_delta_refs: Vec<String>,
    pub threshold_state: UiThresholdState,
    pub waiver_ref: String,
    #[serde(default)]
    pub confounder_refs: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProfileSessionActions {
    pub compare_available: bool,
    pub export_available: bool,
    pub open_trace_available: bool,
    #[serde(default)]
    pub disabled_reason_refs: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CopyExportProjection {
    #[serde(default)]
    pub formats: Vec<String>,
    #[serde(default)]
    pub export_fields: Vec<String>,
    pub text: String,
    pub json: String,
    pub markdown: String,
    pub screenshot_only_prohibited: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReducedCapabilityBanner {
    pub banner_id: String,
    pub severity: String,
    pub capability_state: String,
    pub visible_label: String,
    #[serde(default)]
    pub missing_capabilities: Vec<String>,
    #[serde(default)]
    pub preserved_fields: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SupportExportJoin {
    pub join_id: String,
    pub schema_ref: String,
    #[serde(default)]
    pub joined_object_kinds: Vec<String>,
    pub raw_profile_samples_exported: bool,
    pub raw_trace_events_exported: bool,
    pub gui_cli_support_label_parity: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AutoNarrowingContract {
    #[serde(default)]
    pub narrow_on_missing_or_stale: Vec<String>,
    pub stale_or_missing_effect: String,
    pub policy_blocked_effect: String,
    pub degraded_state_reason_field: String,
    pub release_help_claim_ceiling: String,
}

/// Reusable profile-session card.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProfileSessionCard {
    pub record_kind: String,
    pub schema_version: u32,
    pub card_id: String,
    pub session_ref: String,
    pub profile_kind: ProfileKind,
    pub capture_mode: UiCaptureMode,
    pub artifact_origin: ArtifactOrigin,
    pub execution_origin: ExecutionOrigin,
    pub build_identity_ref: String,
    pub runtime_identity_ref: String,
    pub target: TargetIdentity,
    pub captured_at: String,
    pub duration_ms: u64,
    pub mapping_quality: UiMappingQuality,
    pub baseline: BaselineDisclosure,
    pub actions: ProfileSessionActions,
    #[serde(default)]
    pub source_refs: Vec<String>,
    #[serde(default)]
    pub consumer_surfaces: Vec<ComponentConsumerSurface>,
    pub copy_export: CopyExportProjection,
    pub reduced_capability_banner: ReducedCapabilityBanner,
    pub support_export_join: SupportExportJoin,
    pub auto_narrowing_contract: AutoNarrowingContract,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ThreadProcessContext {
    pub process_ref: String,
    #[serde(default)]
    pub thread_refs: Vec<String>,
    pub context_label: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ThreadProcessFilters {
    pub process_filter_ref: String,
    #[serde(default)]
    pub thread_filter_refs: Vec<String>,
    pub active_filter_label: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SymbolSummary {
    pub symbolicated_frame_count: u64,
    pub unmapped_frame_count: u64,
    pub source_map_ref: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CostScope {
    pub metric: ProfileMetric,
    pub scope_label: String,
    pub sample_count_disclosed: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ZoomState {
    pub root_ref: String,
    pub selected_frame_ref: String,
    pub depth_start: u32,
    pub depth_limit: u32,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SourceNavigationDisclosure {
    pub available: bool,
    pub disabled_reason_ref: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProfileViewActions {
    pub export_available: bool,
    pub open_raw_available: bool,
    #[serde(default)]
    pub disabled_reason_refs: Vec<String>,
}

/// Reusable flamegraph or icicle view.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProfileCostView {
    pub record_kind: String,
    pub schema_version: u32,
    pub view_id: String,
    pub profile_ref: String,
    pub session_ref: String,
    pub view_mode: ProfileCostViewMode,
    pub artifact_origin: ArtifactOrigin,
    pub thread_process_context: ThreadProcessContext,
    pub thread_process_filters: ThreadProcessFilters,
    pub mapping_quality: UiMappingQuality,
    pub symbol_summary: SymbolSummary,
    pub focus_ref: String,
    pub total_samples: u64,
    pub total_time_ms: u64,
    pub metric_presentation: MetricPresentation,
    pub cost_scope: CostScope,
    pub zoom_state: ZoomState,
    pub call_tree_available: bool,
    pub source_navigation: SourceNavigationDisclosure,
    pub actions: ProfileViewActions,
    pub baseline: BaselineDisclosure,
    pub textual_fallback_ref: String,
    #[serde(default)]
    pub source_refs: Vec<String>,
    #[serde(default)]
    pub consumer_surfaces: Vec<ComponentConsumerSurface>,
    pub copy_export: CopyExportProjection,
    pub reduced_capability_banner: ReducedCapabilityBanner,
    pub support_export_join: SupportExportJoin,
    pub auto_narrowing_contract: AutoNarrowingContract,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MetricValue {
    pub metric: ProfileMetric,
    pub value: u64,
    pub unit: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CallTreeNavigation {
    pub caller_navigation_available: bool,
    pub callee_navigation_available: bool,
    pub source_navigation: SourceNavigationDisclosure,
}

/// Reusable call-tree row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProfileCallTreeRow {
    pub record_kind: String,
    pub schema_version: u32,
    pub row_id: String,
    pub session_ref: String,
    pub frame_ref: String,
    pub function_name: String,
    pub frame_identity_ref: String,
    pub self_metric: MetricValue,
    pub inclusive_metric: MetricValue,
    pub file_ref: String,
    pub module_ref: String,
    pub service_ref: String,
    pub thread_ref: String,
    pub symbolization_state: SymbolizationState,
    pub mapping_quality: UiMappingQuality,
    #[serde(default)]
    pub caller_refs: Vec<String>,
    #[serde(default)]
    pub callee_refs: Vec<String>,
    pub navigation: CallTreeNavigation,
    #[serde(default)]
    pub source_refs: Vec<String>,
    #[serde(default)]
    pub consumer_surfaces: Vec<ComponentConsumerSurface>,
    pub copy_export: CopyExportProjection,
    pub support_export_join: SupportExportJoin,
    pub auto_narrowing_contract: AutoNarrowingContract,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ComponentConsumerProjection {
    pub consumer_surface: ComponentConsumerSurface,
    #[serde(default)]
    pub component_refs: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProfileHotpathComponentSummary {
    pub profile_session_card_count: usize,
    pub profile_cost_view_count: usize,
    pub call_tree_row_count: usize,
    pub consumer_projection_count: usize,
    pub hotspot_consumer_present: bool,
    pub secondary_consumer_present: bool,
    pub all_components_preserve_mapping_quality: bool,
    pub all_components_have_copy_export: bool,
}

/// Checked-in M05-797 component packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProfileHotpathComponentPacket {
    pub schema_version: u32,
    pub record_kind: String,
    pub packet_id: String,
    pub as_of: String,
    pub matrix_ref: String,
    #[serde(default)]
    pub profile_session_cards: Vec<ProfileSessionCard>,
    #[serde(default)]
    pub profile_cost_views: Vec<ProfileCostView>,
    #[serde(default)]
    pub call_tree_rows: Vec<ProfileCallTreeRow>,
    #[serde(default)]
    pub consumer_projection_rows: Vec<ComponentConsumerProjection>,
    pub summary: ProfileHotpathComponentSummary,
}

impl ProfileHotpathComponentPacket {
    /// Computes summary fields from the packet contents.
    pub fn computed_summary(&self) -> ProfileHotpathComponentSummary {
        let mut consumers = BTreeSet::new();
        for row in &self.consumer_projection_rows {
            consumers.insert(row.consumer_surface);
        }
        for card in &self.profile_session_cards {
            consumers.extend(card.consumer_surfaces.iter().copied());
        }
        for view in &self.profile_cost_views {
            consumers.extend(view.consumer_surfaces.iter().copied());
        }
        for row in &self.call_tree_rows {
            consumers.extend(row.consumer_surfaces.iter().copied());
        }

        let all_components_preserve_mapping_quality = self
            .profile_session_cards
            .iter()
            .all(|card| card.mapping_quality != UiMappingQuality::NotApplicable)
            && self
                .profile_cost_views
                .iter()
                .all(|view| view.mapping_quality != UiMappingQuality::NotApplicable)
            && self
                .call_tree_rows
                .iter()
                .all(|row| row.mapping_quality != UiMappingQuality::NotApplicable);

        let all_components_have_copy_export = self
            .profile_session_cards
            .iter()
            .all(|card| has_copy_export(&card.copy_export))
            && self
                .profile_cost_views
                .iter()
                .all(|view| has_copy_export(&view.copy_export))
            && self
                .call_tree_rows
                .iter()
                .all(|row| has_copy_export(&row.copy_export));

        ProfileHotpathComponentSummary {
            profile_session_card_count: self.profile_session_cards.len(),
            profile_cost_view_count: self.profile_cost_views.len(),
            call_tree_row_count: self.call_tree_rows.len(),
            consumer_projection_count: self.consumer_projection_rows.len(),
            hotspot_consumer_present: consumers
                .contains(&ComponentConsumerSurface::HotspotWorkspace),
            secondary_consumer_present: consumers
                .iter()
                .any(|surface| *surface != ComponentConsumerSurface::HotspotWorkspace),
            all_components_preserve_mapping_quality,
            all_components_have_copy_export,
        }
    }

    /// Validates the packet and returns every contract violation.
    pub fn validate(&self) -> Vec<ProfileHotpathComponentViolation> {
        let mut violations = Vec::new();

        if self.schema_version != PROFILE_HOTPATH_COMPONENT_SCHEMA_VERSION {
            violations.push(ProfileHotpathComponentViolation::SchemaVersion {
                expected: PROFILE_HOTPATH_COMPONENT_SCHEMA_VERSION,
                actual: self.schema_version,
            });
        }
        if self.record_kind != PROFILE_HOTPATH_COMPONENT_RECORD_KIND {
            violations.push(ProfileHotpathComponentViolation::RecordKind {
                expected: PROFILE_HOTPATH_COMPONENT_RECORD_KIND.to_owned(),
                actual: self.record_kind.clone(),
            });
        }

        let mut card_ids = BTreeSet::new();
        for card in &self.profile_session_cards {
            if !card_ids.insert(card.card_id.clone()) {
                violations.push(ProfileHotpathComponentViolation::DuplicateId {
                    kind: "profile_session_card",
                    id: card.card_id.clone(),
                });
            }
            if card.record_kind != "m5_profile_session_card"
                || card.schema_version != 1
                || card.session_ref.trim().is_empty()
                || card.build_identity_ref.trim().is_empty()
                || card.runtime_identity_ref.trim().is_empty()
                || card.target.process_ref.trim().is_empty()
                || card.target.config_ref.trim().is_empty()
                || card.captured_at.trim().is_empty()
                || card.duration_ms == 0
            {
                violations.push(
                    ProfileHotpathComponentViolation::IncompleteProfileSessionCard {
                        id: card.card_id.clone(),
                    },
                );
            }
            if card.target.raw_command_line_exported {
                violations.push(ProfileHotpathComponentViolation::RawCommandLineExported {
                    id: card.card_id.clone(),
                });
            }
            if !card.actions.compare_available || !card.actions.export_available {
                violations.push(
                    ProfileHotpathComponentViolation::MissingCompareOrExportAction {
                        id: card.card_id.clone(),
                    },
                );
            }
            if !card
                .consumer_surfaces
                .contains(&ComponentConsumerSurface::HotspotWorkspace)
                || card.consumer_surfaces.len() < 2
            {
                violations.push(ProfileHotpathComponentViolation::MissingConsumerParity {
                    id: card.card_id.clone(),
                });
            }
        }

        let mut view_ids = BTreeSet::new();
        for view in &self.profile_cost_views {
            if !view_ids.insert(view.view_id.clone()) {
                violations.push(ProfileHotpathComponentViolation::DuplicateId {
                    kind: "profile_cost_view",
                    id: view.view_id.clone(),
                });
            }
            if view.record_kind != "m5_flamegraph_view"
                || view.schema_version != 1
                || view.profile_ref.trim().is_empty()
                || view.session_ref.trim().is_empty()
                || view.thread_process_context.process_ref.trim().is_empty()
                || view.thread_process_context.thread_refs.is_empty()
                || view.thread_process_filters.thread_filter_refs.is_empty()
                || view.focus_ref.trim().is_empty()
                || view.total_samples == 0
                || view.zoom_state.root_ref.trim().is_empty()
                || view.zoom_state.depth_limit == 0
            {
                violations.push(
                    ProfileHotpathComponentViolation::IncompleteProfileCostView {
                        id: view.view_id.clone(),
                    },
                );
            }
            if matches!(
                view.view_mode,
                ProfileCostViewMode::Flamegraph | ProfileCostViewMode::Icicle
            ) && (!view.actions.export_available || !view.actions.open_raw_available)
            {
                violations.push(ProfileHotpathComponentViolation::MissingExportOrRawAction {
                    id: view.view_id.clone(),
                });
            }
            if !view
                .consumer_surfaces
                .contains(&ComponentConsumerSurface::HotspotWorkspace)
                || view.consumer_surfaces.len() < 2
            {
                violations.push(ProfileHotpathComponentViolation::MissingConsumerParity {
                    id: view.view_id.clone(),
                });
            }
        }

        let mut row_ids = BTreeSet::new();
        for row in &self.call_tree_rows {
            if !row_ids.insert(row.row_id.clone()) {
                violations.push(ProfileHotpathComponentViolation::DuplicateId {
                    kind: "call_tree_row",
                    id: row.row_id.clone(),
                });
            }
            if row.record_kind != "m5_call_tree_row"
                || row.schema_version != 1
                || row.frame_ref.trim().is_empty()
                || row.function_name.trim().is_empty()
                || row.frame_identity_ref.trim().is_empty()
                || row.inclusive_metric.value < row.self_metric.value
                || row.file_ref.trim().is_empty()
                || row.module_ref.trim().is_empty()
                || row.service_ref.trim().is_empty()
                || row.thread_ref.trim().is_empty()
            {
                violations.push(ProfileHotpathComponentViolation::IncompleteCallTreeRow {
                    id: row.row_id.clone(),
                });
            }
            if !row.symbolization_state.is_disclosed()
                || row.mapping_quality == UiMappingQuality::NotApplicable
            {
                violations.push(
                    ProfileHotpathComponentViolation::MissingSymbolizationOrMapping {
                        id: row.row_id.clone(),
                    },
                );
            }
            if !row.navigation.caller_navigation_available
                || !row.navigation.callee_navigation_available
                || row.caller_refs.is_empty()
                || row.callee_refs.is_empty()
            {
                violations.push(
                    ProfileHotpathComponentViolation::MissingCallerCalleeNavigation {
                        id: row.row_id.clone(),
                    },
                );
            }
        }

        if self.summary != self.computed_summary() {
            violations.push(ProfileHotpathComponentViolation::SummaryMismatch);
        }

        violations
    }
}

fn has_copy_export(copy_export: &CopyExportProjection) -> bool {
    copy_export.screenshot_only_prohibited
        && copy_export.formats.iter().any(|format| format == "text")
        && copy_export.formats.iter().any(|format| format == "json")
        && copy_export
            .formats
            .iter()
            .any(|format| format == "markdown")
        && !copy_export.export_fields.is_empty()
}

/// Loads the checked-in M05-797 packet.
pub fn current_profile_hotpath_component_packet(
) -> Result<ProfileHotpathComponentPacket, serde_json::Error> {
    serde_json::from_str(PROFILE_HOTPATH_COMPONENT_PACKET_JSON)
}

/// Validation failure for M05-797 component packets.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ProfileHotpathComponentViolation {
    SchemaVersion { expected: u32, actual: u32 },
    RecordKind { expected: String, actual: String },
    DuplicateId { kind: &'static str, id: String },
    IncompleteProfileSessionCard { id: String },
    RawCommandLineExported { id: String },
    MissingCompareOrExportAction { id: String },
    IncompleteProfileCostView { id: String },
    MissingExportOrRawAction { id: String },
    IncompleteCallTreeRow { id: String },
    MissingSymbolizationOrMapping { id: String },
    MissingCallerCalleeNavigation { id: String },
    MissingConsumerParity { id: String },
    SummaryMismatch,
}

impl fmt::Display for ProfileHotpathComponentViolation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SchemaVersion { expected, actual } => {
                write!(
                    f,
                    "schema version mismatch: expected {expected}, got {actual}"
                )
            }
            Self::RecordKind { expected, actual } => {
                write!(f, "record kind mismatch: expected {expected}, got {actual}")
            }
            Self::DuplicateId { kind, id } => write!(f, "duplicate {kind} id: {id}"),
            Self::IncompleteProfileSessionCard { id } => {
                write!(f, "incomplete profile-session card: {id}")
            }
            Self::RawCommandLineExported { id } => {
                write!(f, "profile-session card {id} exported a raw command line")
            }
            Self::MissingCompareOrExportAction { id } => {
                write!(
                    f,
                    "profile-session card {id} is missing compare/export action"
                )
            }
            Self::IncompleteProfileCostView { id } => {
                write!(f, "incomplete flamegraph/icicle view: {id}")
            }
            Self::MissingExportOrRawAction { id } => {
                write!(
                    f,
                    "profile-cost view {id} is missing export/open-raw action"
                )
            }
            Self::IncompleteCallTreeRow { id } => write!(f, "incomplete call-tree row: {id}"),
            Self::MissingSymbolizationOrMapping { id } => {
                write!(
                    f,
                    "call-tree row {id} is missing symbolization or mapping truth"
                )
            }
            Self::MissingCallerCalleeNavigation { id } => {
                write!(f, "call-tree row {id} is missing caller/callee navigation")
            }
            Self::MissingConsumerParity { id } => {
                write!(
                    f,
                    "component {id} is missing hotspot plus secondary consumer parity"
                )
            }
            Self::SummaryMismatch => write!(f, "computed summary does not match stored summary"),
        }
    }
}

impl Error for ProfileHotpathComponentViolation {}
