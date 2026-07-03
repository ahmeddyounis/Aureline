//! Reusable trace timelines, heap/allocation compare panels, and profile-compare cards.
//!
//! This module is the M05-798 component contract that layers the frozen
//! `trace_timeline` and `heap_profile_compare_card` families of the M5
//! profiler/topology matrix onto checked-in fixtures. It keeps synchronized
//! clock/capture origin, lane partiality, and imported-versus-live truth
//! explicit on timelines, and keeps baseline identity, environment deltas,
//! threshold/waiver state, and likely-confounder notes explicit on heap and
//! profile compare cards before any regression is claimed.
//!
//! It reuses the shared controlled-vocabulary types and disclosure structs from
//! [`crate::implement_profile_session_cards_flamegraph_icicle_views_and_call_tree_rows`]
//! so the compare/confounder vocabulary survives exports, AI explanations, and
//! incident handoff surfaces unchanged.

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

use crate::implement_profile_session_cards_flamegraph_icicle_views_and_call_tree_rows::{
    ArtifactOrigin, AutoNarrowingContract, BaselineDisclosure, BaselineEnvironmentState,
    ComponentConsumerProjection, ComponentConsumerSurface, CopyExportProjection, CostScope,
    ExecutionOrigin, ProfileCostViewMode, ReducedCapabilityBanner, SourceNavigationDisclosure,
    SupportExportJoin, SymbolSummary, ThreadProcessContext, UiCaptureMode, UiMappingQuality,
    UiThresholdState,
};

/// Schema version stamped on the M05-798 component packet.
pub const TRACE_HEAP_COMPARE_COMPONENT_SCHEMA_VERSION: u32 = 1;

/// Stable record-kind tag for [`TraceHeapCompareComponentPacket`].
pub const TRACE_HEAP_COMPARE_COMPONENT_RECORD_KIND: &str = "m5_trace_heap_compare_component_packet";

/// Repo-relative path to the checked-in M05-798 packet.
pub const TRACE_HEAP_COMPARE_COMPONENT_PACKET_PATH: &str =
    "artifacts/perf/m5/m5-trace-heap-compare-components.json";

/// Embedded checked-in M05-798 packet JSON.
pub const TRACE_HEAP_COMPARE_COMPONENT_PACKET_JSON: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../artifacts/perf/m5/m5-trace-heap-compare-components.json"
));

/// Clock/synchronization basis disclosed by a trace timeline.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ClockSyncBasis {
    MonotonicSingleProcess,
    SynchronizedMultiProcess,
    ImportedClockDomain,
    PartialClockCorrelation,
    Unknown,
}

impl ClockSyncBasis {
    /// Returns true when the timeline must disclose that cross-lane timing is
    /// only approximate.
    pub const fn is_degraded(self) -> bool {
        matches!(
            self,
            Self::ImportedClockDomain | Self::PartialClockCorrelation | Self::Unknown
        )
    }
}

/// Process/thread lane summary shown by a trace timeline.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LaneSummary {
    #[serde(default)]
    pub process_refs: Vec<String>,
    #[serde(default)]
    pub thread_refs: Vec<String>,
    pub event_lane_count: u32,
    pub hidden_lane_count: u32,
}

/// Chronology/reverse-step capability disclosed by a trace timeline.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ChronologyCapability {
    pub reverse_step_available: bool,
    pub replay_available: bool,
    #[serde(default)]
    pub disabled_reason_refs: Vec<String>,
}

/// Reusable trace timeline.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TraceTimeline {
    pub record_kind: String,
    pub schema_version: u32,
    pub timeline_id: String,
    pub trace_ref: String,
    pub session_ref: String,
    pub capture_mode: UiCaptureMode,
    pub artifact_origin: ArtifactOrigin,
    pub execution_origin: ExecutionOrigin,
    pub lane_summary: LaneSummary,
    pub clock_sync_basis: ClockSyncBasis,
    pub mapping_quality: UiMappingQuality,
    pub selected_range_ref: String,
    #[serde(default)]
    pub bookmark_refs: Vec<String>,
    pub partiality_note_ref: String,
    #[serde(default)]
    pub packet_refs: Vec<String>,
    pub textual_fallback_ref: String,
    pub chronology_capability: ChronologyCapability,
    #[serde(default)]
    pub source_refs: Vec<String>,
    #[serde(default)]
    pub consumer_surfaces: Vec<ComponentConsumerSurface>,
    pub copy_export: CopyExportProjection,
    pub reduced_capability_banner: ReducedCapabilityBanner,
    pub support_export_join: SupportExportJoin,
    pub auto_narrowing_contract: AutoNarrowingContract,
}

/// Reusable heap/allocation and profile compare card.
///
/// Shares the flamegraph/profile-cost mapping grammar (`m5_heap_profile_compare_card`
/// records validate against `schemas/ui/m5-flamegraph-view.schema.json`), but the
/// compare card omits the flamegraph-only navigation fields and foregrounds the
/// baseline disclosure instead.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HeapProfileCompareCard {
    pub record_kind: String,
    pub schema_version: u32,
    pub view_id: String,
    pub profile_ref: String,
    pub session_ref: String,
    pub view_mode: ProfileCostViewMode,
    pub artifact_origin: ArtifactOrigin,
    pub thread_process_context: ThreadProcessContext,
    pub mapping_quality: UiMappingQuality,
    pub symbol_summary: SymbolSummary,
    pub focus_ref: String,
    pub cost_scope: CostScope,
    pub call_tree_available: bool,
    pub source_navigation: SourceNavigationDisclosure,
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

impl HeapProfileCompareCard {
    /// Returns true when this compare card is drawn from a non-live artifact and
    /// therefore may not claim a regression until it is narrowed.
    pub fn is_imported_artifact(&self) -> bool {
        matches!(
            self.artifact_origin,
            ArtifactOrigin::ImportedArtifact
                | ArtifactOrigin::SupportBundle
                | ArtifactOrigin::CachedReplay
        )
    }

    /// Returns true when the card foregrounds baseline identity, environment
    /// deltas, threshold state, waiver state, and confounder notes.
    pub fn baseline_disclosed(&self) -> bool {
        !self.baseline.baseline_ref.trim().is_empty()
            && !self.baseline.waiver_ref.trim().is_empty()
            && !self.baseline.confounder_refs.is_empty()
    }

    /// Returns true when the card claims a regression.
    pub fn claims_regression(&self) -> bool {
        self.baseline.threshold_state == UiThresholdState::Regression
    }

    /// Returns true when the card may honestly claim a regression: baseline
    /// identity, comparable environment, environment deltas, and confounders are
    /// all visible.
    pub fn regression_claim_supported(&self) -> bool {
        self.baseline_disclosed()
            && matches!(
                self.baseline.environment_state,
                BaselineEnvironmentState::Comparable
                    | BaselineEnvironmentState::ComparableWithDeltas
            )
            && !self.baseline.environment_delta_refs.is_empty()
    }
}

/// Rolled-up summary of an M05-798 component packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TraceHeapCompareComponentSummary {
    pub trace_timeline_count: usize,
    pub heap_compare_card_count: usize,
    pub consumer_projection_count: usize,
    pub trace_viewer_consumer_present: bool,
    pub profile_compare_consumer_present: bool,
    pub imported_and_live_both_present: bool,
    pub all_components_preserve_mapping_quality: bool,
    pub all_components_have_copy_export: bool,
    pub all_compare_cards_disclose_baseline: bool,
}

/// Checked-in M05-798 component packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TraceHeapCompareComponentPacket {
    pub schema_version: u32,
    pub record_kind: String,
    pub packet_id: String,
    pub as_of: String,
    pub matrix_ref: String,
    #[serde(default)]
    pub trace_timelines: Vec<TraceTimeline>,
    #[serde(default)]
    pub heap_compare_cards: Vec<HeapProfileCompareCard>,
    #[serde(default)]
    pub consumer_projection_rows: Vec<ComponentConsumerProjection>,
    pub summary: TraceHeapCompareComponentSummary,
}

impl TraceHeapCompareComponentPacket {
    /// Computes summary fields from the packet contents.
    pub fn computed_summary(&self) -> TraceHeapCompareComponentSummary {
        let mut consumers = BTreeSet::new();
        for row in &self.consumer_projection_rows {
            consumers.insert(row.consumer_surface);
        }
        for timeline in &self.trace_timelines {
            consumers.extend(timeline.consumer_surfaces.iter().copied());
        }
        for card in &self.heap_compare_cards {
            consumers.extend(card.consumer_surfaces.iter().copied());
        }

        let mut has_live = false;
        let mut has_imported = false;
        for origin in self
            .trace_timelines
            .iter()
            .map(|t| t.artifact_origin)
            .chain(self.heap_compare_cards.iter().map(|c| c.artifact_origin))
        {
            if origin.is_live() {
                has_live = true;
            } else {
                has_imported = true;
            }
        }

        let all_components_preserve_mapping_quality = self
            .trace_timelines
            .iter()
            .all(|t| t.mapping_quality != UiMappingQuality::NotApplicable)
            && self
                .heap_compare_cards
                .iter()
                .all(|c| c.mapping_quality != UiMappingQuality::NotApplicable);

        let all_components_have_copy_export = self
            .trace_timelines
            .iter()
            .all(|t| has_copy_export(&t.copy_export))
            && self
                .heap_compare_cards
                .iter()
                .all(|c| has_copy_export(&c.copy_export));

        let all_compare_cards_disclose_baseline = self
            .heap_compare_cards
            .iter()
            .all(HeapProfileCompareCard::baseline_disclosed);

        TraceHeapCompareComponentSummary {
            trace_timeline_count: self.trace_timelines.len(),
            heap_compare_card_count: self.heap_compare_cards.len(),
            consumer_projection_count: self.consumer_projection_rows.len(),
            trace_viewer_consumer_present: consumers
                .contains(&ComponentConsumerSurface::TraceViewer),
            profile_compare_consumer_present: consumers
                .contains(&ComponentConsumerSurface::ProfileCompare),
            imported_and_live_both_present: has_live && has_imported,
            all_components_preserve_mapping_quality,
            all_components_have_copy_export,
            all_compare_cards_disclose_baseline,
        }
    }

    /// Validates the packet and returns every contract violation.
    pub fn validate(&self) -> Vec<TraceHeapCompareComponentViolation> {
        let mut violations = Vec::new();

        if self.schema_version != TRACE_HEAP_COMPARE_COMPONENT_SCHEMA_VERSION {
            violations.push(TraceHeapCompareComponentViolation::SchemaVersion {
                expected: TRACE_HEAP_COMPARE_COMPONENT_SCHEMA_VERSION,
                actual: self.schema_version,
            });
        }
        if self.record_kind != TRACE_HEAP_COMPARE_COMPONENT_RECORD_KIND {
            violations.push(TraceHeapCompareComponentViolation::RecordKind {
                expected: TRACE_HEAP_COMPARE_COMPONENT_RECORD_KIND.to_owned(),
                actual: self.record_kind.clone(),
            });
        }

        let mut timeline_ids = BTreeSet::new();
        for timeline in &self.trace_timelines {
            if !timeline_ids.insert(timeline.timeline_id.clone()) {
                violations.push(TraceHeapCompareComponentViolation::DuplicateId {
                    kind: "trace_timeline",
                    id: timeline.timeline_id.clone(),
                });
            }
            if timeline.record_kind != "m5_trace_timeline"
                || timeline.schema_version != 1
                || timeline.trace_ref.trim().is_empty()
                || timeline.session_ref.trim().is_empty()
                || timeline.lane_summary.process_refs.is_empty()
                || timeline.lane_summary.thread_refs.is_empty()
                || timeline.selected_range_ref.trim().is_empty()
                || timeline.partiality_note_ref.trim().is_empty()
                || timeline.textual_fallback_ref.trim().is_empty()
            {
                violations.push(
                    TraceHeapCompareComponentViolation::IncompleteTraceTimeline {
                        id: timeline.timeline_id.clone(),
                    },
                );
            }
            if timeline.mapping_quality == UiMappingQuality::NotApplicable {
                violations.push(
                    TraceHeapCompareComponentViolation::MissingTimelineMappingQuality {
                        id: timeline.timeline_id.clone(),
                    },
                );
            }
            // Imported-versus-live truth must survive into the export.
            if !timeline
                .copy_export
                .export_fields
                .iter()
                .any(|f| f == "artifact_origin")
                || !timeline
                    .copy_export
                    .export_fields
                    .iter()
                    .any(|f| f == "clock_sync_basis")
            {
                violations.push(
                    TraceHeapCompareComponentViolation::TimelineOriginNotExported {
                        id: timeline.timeline_id.clone(),
                    },
                );
            }
            if !timeline
                .consumer_surfaces
                .contains(&ComponentConsumerSurface::TraceViewer)
                || timeline.consumer_surfaces.len() < 2
            {
                violations.push(TraceHeapCompareComponentViolation::MissingConsumerParity {
                    id: timeline.timeline_id.clone(),
                });
            }
        }

        let mut card_ids = BTreeSet::new();
        for card in &self.heap_compare_cards {
            if !card_ids.insert(card.view_id.clone()) {
                violations.push(TraceHeapCompareComponentViolation::DuplicateId {
                    kind: "heap_compare_card",
                    id: card.view_id.clone(),
                });
            }
            if card.record_kind != "m5_heap_profile_compare_card"
                || card.schema_version != 1
                || !matches!(
                    card.view_mode,
                    ProfileCostViewMode::HeapDiff
                        | ProfileCostViewMode::AllocationDiff
                        | ProfileCostViewMode::RetainedSizeDiff
                )
                || card.profile_ref.trim().is_empty()
                || card.session_ref.trim().is_empty()
                || card.focus_ref.trim().is_empty()
            {
                violations.push(
                    TraceHeapCompareComponentViolation::IncompleteHeapCompareCard {
                        id: card.view_id.clone(),
                    },
                );
            }
            if card.mapping_quality == UiMappingQuality::NotApplicable {
                violations.push(
                    TraceHeapCompareComponentViolation::MissingCardMappingQuality {
                        id: card.view_id.clone(),
                    },
                );
            }
            // AC1: a regression claim requires baseline identity, comparable
            // environment, and environment deltas to be visible first.
            if !card.baseline_disclosed()
                || (card.claims_regression() && !card.regression_claim_supported())
            {
                violations.push(
                    TraceHeapCompareComponentViolation::BaselineNotDisclosedBeforeRegression {
                        id: card.view_id.clone(),
                    },
                );
            }
            // AC2: imported/support/cached compare cards may not claim a
            // regression and must render as narrowed truth.
            if card.is_imported_artifact()
                && (card.claims_regression()
                    || card.reduced_capability_banner.capability_state == "full")
            {
                violations.push(
                    TraceHeapCompareComponentViolation::ImportedCompareNotNarrowed {
                        id: card.view_id.clone(),
                    },
                );
            }
            // AC3: the compare/confounder vocabulary must survive the export.
            if !card
                .copy_export
                .export_fields
                .iter()
                .any(|f| f.contains("confounder"))
                || !card
                    .copy_export
                    .export_fields
                    .iter()
                    .any(|f| f.contains("threshold_state"))
            {
                violations.push(
                    TraceHeapCompareComponentViolation::CompareVocabularyNotExported {
                        id: card.view_id.clone(),
                    },
                );
            }
            if !card
                .consumer_surfaces
                .contains(&ComponentConsumerSurface::ProfileCompare)
                || card.consumer_surfaces.len() < 2
            {
                violations.push(TraceHeapCompareComponentViolation::MissingConsumerParity {
                    id: card.view_id.clone(),
                });
            }
        }

        if self.summary != self.computed_summary() {
            violations.push(TraceHeapCompareComponentViolation::SummaryMismatch);
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

/// Loads the checked-in M05-798 packet.
pub fn current_trace_heap_compare_component_packet(
) -> Result<TraceHeapCompareComponentPacket, serde_json::Error> {
    serde_json::from_str(TRACE_HEAP_COMPARE_COMPONENT_PACKET_JSON)
}

/// Validation failure for M05-798 component packets.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TraceHeapCompareComponentViolation {
    SchemaVersion { expected: u32, actual: u32 },
    RecordKind { expected: String, actual: String },
    DuplicateId { kind: &'static str, id: String },
    IncompleteTraceTimeline { id: String },
    MissingTimelineMappingQuality { id: String },
    TimelineOriginNotExported { id: String },
    IncompleteHeapCompareCard { id: String },
    MissingCardMappingQuality { id: String },
    BaselineNotDisclosedBeforeRegression { id: String },
    ImportedCompareNotNarrowed { id: String },
    CompareVocabularyNotExported { id: String },
    MissingConsumerParity { id: String },
    SummaryMismatch,
}

impl fmt::Display for TraceHeapCompareComponentViolation {
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
            Self::IncompleteTraceTimeline { id } => write!(f, "incomplete trace timeline: {id}"),
            Self::MissingTimelineMappingQuality { id } => {
                write!(f, "trace timeline {id} is missing mapping quality")
            }
            Self::TimelineOriginNotExported { id } => {
                write!(
                    f,
                    "trace timeline {id} does not export artifact origin / clock basis"
                )
            }
            Self::IncompleteHeapCompareCard { id } => {
                write!(f, "incomplete heap/profile compare card: {id}")
            }
            Self::MissingCardMappingQuality { id } => {
                write!(f, "compare card {id} is missing mapping quality")
            }
            Self::BaselineNotDisclosedBeforeRegression { id } => {
                write!(
                    f,
                    "compare card {id} claims a regression without full baseline disclosure"
                )
            }
            Self::ImportedCompareNotNarrowed { id } => {
                write!(
                    f,
                    "imported compare card {id} claims regression or is not narrowed"
                )
            }
            Self::CompareVocabularyNotExported { id } => {
                write!(
                    f,
                    "compare card {id} drops confounder/threshold vocabulary from export"
                )
            }
            Self::MissingConsumerParity { id } => {
                write!(
                    f,
                    "component {id} is missing required plus secondary consumer parity"
                )
            }
            Self::SummaryMismatch => write!(f, "computed summary does not match stored summary"),
        }
    }
}

impl Error for TraceHeapCompareComponentViolation {}
