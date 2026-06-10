//! Shared trace viewer with synchronized event lanes, bookmarks, and textual fallback.
//!
//! This module materializes the typed records that keep the trace viewer honest about
//! what events are shown, how lanes stay synchronized, what bookmarks are available,
//! and how textual fallback degrades when visual rendering is unavailable. The records
//! and closed vocabularies here mirror the boundary schema at
//! `/schemas/perf/implement-the-shared-trace-viewer-with-synchronized-event-lanes-bookmarks-and-textual-fallback.schema.json`
//! and reuse the capture-class, provenance, and mapping-quality axes already frozen in
//! `/docs/performance/profiling_trace_replay_contract.md`.
//!
//! The module exposes:
//!
//! - the [`EventLaneRow`] record that binds lane identity, kind, thread or stream ref,
//!   event count, time range, synchronized-with refs, and mapping quality so lanes never
//!   hide unresolved events behind pretty timelines;
//! - the [`BookmarkRow`] record that carries timestamp, lane reference, category, note,
//!   creator identity, and provenance flag so bookmarks are always attributable;
//! - the [`TextualFallbackRow`] record that carries content kind, raw lines, source event
//!   refs, and mapping quality so terminals and accessibility surfaces degrade honestly;
//! - the [`TraceViewerQualificationPacket`] checked-in artifact that downstream docs, help,
//!   support, and CI surfaces ingest instead of cloning status text.
//!
//! Raw payload bytes, raw command lines, secrets, and ambient credentials MUST NOT
//! appear on any record carried here.

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

/// Schema version stamped on every trace-viewer qualification packet carried by
/// this module. Bumped only on breaking payload changes; additive-optional fields
/// do not bump this value.
pub const TRACE_VIEWER_QUALIFICATION_SCHEMA_VERSION: u32 = 1;

/// Stable record-kind tag for [`TraceViewerQualificationPacket`].
pub const TRACE_VIEWER_QUALIFICATION_RECORD_KIND: &str =
    "implement_the_shared_trace_viewer_with_synchronized_event_lanes_bookmarks_and_textual_fallback";

/// Repo-relative path to the checked-in trace-viewer qualification packet JSON.
pub const TRACE_VIEWER_QUALIFICATION_PACKET_PATH: &str =
    "artifacts/perf/m5/implement-the-shared-trace-viewer-with-synchronized-event-lanes-bookmarks-and-textual-fallback.json";

/// Embedded checked-in qualification packet JSON.
pub const TRACE_VIEWER_QUALIFICATION_PACKET_JSON: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../artifacts/perf/m5/implement-the-shared-trace-viewer-with-synchronized-event-lanes-bookmarks-and-textual-fallback.json"
));

/// Qualification label shown on promoted trace-viewer surfaces.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TraceViewerQualificationLabel {
    /// Surface has current proof and may be called stable for its declared scope.
    Stable,
    /// Surface is visible but below stable.
    Preview,
    /// Surface is an experiment or internal lab.
    Labs,
    /// Surface may inspect metadata but must not execute or export live data.
    InspectOnly,
    /// Surface may import or view captured files only.
    ImportOnly,
}

impl TraceViewerQualificationLabel {
    /// Returns true when the label is a stable claim.
    pub const fn is_stable(self) -> bool {
        matches!(self, Self::Stable)
    }
}

/// Trace-viewer surface family governed by this packet.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TraceViewerSurfaceKind {
    /// Synchronized event lane view.
    EventLaneView,
    /// Bookmark panel or inspector.
    BookmarkPanel,
    /// Textual fallback view for terminal or accessibility.
    TextualFallbackView,
    /// Side-by-side trace comparison.
    TraceComparison,
    /// Export review surface for trace evidence.
    ExportReview,
    /// Support export surface for trace evidence.
    SupportExport,
}

/// Kind of event lane displayed in the trace viewer.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EventLaneKind {
    /// Lane bound to a single thread.
    ThreadLane,
    /// Lane bound to an async stream or channel.
    StreamLane,
    /// Lane grouping events by category.
    CategoryLane,
    /// Lane merging multiple sources.
    MergedLane,
}

/// Mapping-quality state for symbol-to-source or event-to-lane resolution.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TraceMappingQualityLabel {
    /// Exact symbol and source location.
    Exact,
    /// Approximate match; may be nearest symbol or line.
    Approximate,
    /// Partial mapping; some inlined or generated frames.
    Partial,
    /// No mapping available.
    Unavailable,
    /// Mapping is stale relative to current build.
    Stale,
    /// Mapping mismatches the current build.
    Mismatched,
}

/// Content kind for a textual fallback row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TextualFallbackContentKind {
    /// Structured event lines with timestamps and names.
    StructuredEvents,
    /// Raw span records with ids and ranges.
    RawSpans,
    /// Annotated log output.
    AnnotatedLog,
    /// Diff output from trace comparison.
    ComparisonDiff,
}

/// One event-lane row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EventLaneRow {
    /// Stable lane row id.
    pub lane_id: String,
    /// Human-readable title.
    pub title: String,
    /// Lane kind.
    pub lane_kind: EventLaneKind,
    /// Thread or stream ref.
    pub thread_or_stream_ref: String,
    /// Number of events in the lane.
    pub event_count: usize,
    /// Start timestamp in nanoseconds.
    pub start_ns: u64,
    /// End timestamp in nanoseconds.
    pub end_ns: u64,
    /// Lane ids this lane is synchronized with.
    #[serde(default)]
    pub synchronized_with: Vec<String>,
    /// Mapping quality state.
    pub mapping_quality: TraceMappingQualityLabel,
    /// True when the lane shows its mapping quality.
    pub shows_mapping_quality: bool,
    /// True when the lane shows a degraded-state label.
    pub shows_degraded_label: bool,
}

/// One bookmark row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BookmarkRow {
    /// Stable bookmark row id.
    pub bookmark_id: String,
    /// Human-readable title.
    pub title: String,
    /// Timestamp in nanoseconds.
    pub timestamp_ns: u64,
    /// Lane ref this bookmark is anchored to.
    pub lane_ref: String,
    /// Category label.
    pub category: String,
    /// Optional note.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub note: Option<String>,
    /// Creator identity.
    pub created_by: String,
    /// True when the bookmark shows its provenance.
    pub shows_provenance: bool,
}

/// One textual-fallback row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TextualFallbackRow {
    /// Stable fallback row id.
    pub fallback_id: String,
    /// Human-readable title.
    pub title: String,
    /// Content kind.
    pub content_kind: TextualFallbackContentKind,
    /// Raw text lines.
    #[serde(default)]
    pub raw_lines: Vec<String>,
    /// Source event or lane refs.
    #[serde(default)]
    pub source_event_refs: Vec<String>,
    /// Mapping quality state.
    pub mapping_quality: TraceMappingQualityLabel,
    /// True when the fallback shows its mapping quality.
    pub shows_mapping_quality: bool,
}

/// Checked-in proof bundle for one surface row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TraceViewerQualificationProof {
    /// Packet id.
    pub packet_id: String,
    /// Packet ref path.
    pub packet_ref: String,
    /// Proof index ref path.
    pub proof_index_ref: String,
    /// Captured-at timestamp.
    pub captured_at: String,
    /// Evidence refs.
    #[serde(default)]
    pub evidence_refs: Vec<String>,
}

/// Guard set for a trace-viewer surface qualification row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TraceViewerSurfaceGuardSet {
    /// Event lanes are visible.
    pub event_lanes_visible: bool,
    /// Bookmarks are visible.
    pub bookmarks_visible: bool,
    /// Textual fallback is visible.
    pub textual_fallback_visible: bool,
    /// Synchronization state is visible.
    pub synchronization_visible: bool,
    /// Mapping quality is visible.
    pub mapping_quality_visible: bool,
    /// Export posture is visible.
    pub export_posture_visible: bool,
    /// Degraded-state label is visible when applicable.
    pub degraded_state_label_visible: bool,
}

/// One surface qualification row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TraceViewerSurfaceQualificationRow {
    /// Surface id.
    pub surface_id: String,
    /// Surface title.
    pub title: String,
    /// Surface kind.
    pub surface_kind: TraceViewerSurfaceKind,
    /// True when the surface is present in the promoted build.
    pub promoted_build_surface: bool,
    /// Claim label.
    pub claim_label: TraceViewerQualificationLabel,
    /// Displayed label (may differ from claim when narrowed).
    pub displayed_label: String,
    /// Qualification proof bundle.
    pub qualification_packet: TraceViewerQualificationProof,
    /// Guard set.
    pub guards: TraceViewerSurfaceGuardSet,
    /// True when the surface downgrades if required guards are missing.
    pub downgrade_if_missing: bool,
    /// Rationale string.
    pub rationale: String,
}

/// Summary projected onto help, release, and support surfaces.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TraceViewerQualificationSummary {
    /// Total number of event-lane rows.
    pub event_lane_count: usize,
    /// Total number of bookmark rows.
    pub bookmark_count: usize,
    /// Total number of textual-fallback rows.
    pub textual_fallback_count: usize,
    /// Number of rows claiming stable.
    pub stable_count: usize,
    /// Number of rows below stable.
    pub below_stable_count: usize,
    /// True when every row has a non-empty disclosure ref if below stable.
    pub all_below_stable_have_disclosure: bool,
}

/// The checked-in trace-viewer qualification packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TraceViewerQualificationPacket {
    /// Schema version.
    pub schema_version: u32,
    /// Record kind.
    pub record_kind: String,
    /// Packet id.
    pub packet_id: String,
    /// As-of timestamp.
    pub as_of: String,
    /// Release doc ref.
    pub release_doc_ref: String,
    /// Help doc ref.
    pub help_doc_ref: String,
    /// Schema ref.
    pub schema_ref: String,
    /// Surface qualification rows.
    pub surfaces: Vec<TraceViewerSurfaceQualificationRow>,
    /// Event-lane rows.
    pub event_lanes: Vec<EventLaneRow>,
    /// Bookmark rows.
    pub bookmarks: Vec<BookmarkRow>,
    /// Textual-fallback rows.
    pub textual_fallbacks: Vec<TextualFallbackRow>,
    /// Summary.
    pub summary: TraceViewerQualificationSummary,
}

impl TraceViewerQualificationPacket {
    /// Computes the summary from current rows.
    pub fn computed_summary(&self) -> TraceViewerQualificationSummary {
        let stable_count = self
            .surfaces
            .iter()
            .filter(|s| s.claim_label.is_stable())
            .count();
        let below_stable_count = self.surfaces.len().saturating_sub(stable_count);
        let all_below_stable_have_disclosure = self
            .surfaces
            .iter()
            .filter(|s| !s.claim_label.is_stable())
            .all(|s| !s.rationale.is_empty());

        TraceViewerQualificationSummary {
            event_lane_count: self.event_lanes.len(),
            bookmark_count: self.bookmarks.len(),
            textual_fallback_count: self.textual_fallbacks.len(),
            stable_count,
            below_stable_count,
            all_below_stable_have_disclosure,
        }
    }

    /// Validates the packet and returns any violations.
    pub fn validate(&self) -> Vec<TraceViewerQualificationViolation> {
        let mut violations = Vec::new();

        if self.schema_version != TRACE_VIEWER_QUALIFICATION_SCHEMA_VERSION {
            violations.push(TraceViewerQualificationViolation::SchemaVersion {
                expected: TRACE_VIEWER_QUALIFICATION_SCHEMA_VERSION,
                actual: self.schema_version,
            });
        }

        if self.record_kind != TRACE_VIEWER_QUALIFICATION_RECORD_KIND {
            violations.push(TraceViewerQualificationViolation::RecordKind {
                expected: TRACE_VIEWER_QUALIFICATION_RECORD_KIND.to_owned(),
                actual: self.record_kind.clone(),
            });
        }

        let mut surface_ids = BTreeSet::new();
        for surface in &self.surfaces {
            if !surface_ids.insert(surface.surface_id.clone()) {
                violations.push(TraceViewerQualificationViolation::DuplicateId {
                    kind: TraceViewerQualificationViolationKind::Surface,
                    id: surface.surface_id.clone(),
                });
            }
            if surface.promoted_build_surface
                && surface.claim_label.is_stable()
                && (!surface.guards.event_lanes_visible
                    || !surface.guards.bookmarks_visible
                    || !surface.guards.textual_fallback_visible
                    || !surface.guards.synchronization_visible
                    || !surface.guards.mapping_quality_visible)
            {
                violations.push(TraceViewerQualificationViolation::IncompleteGuardSet {
                    surface_id: surface.surface_id.clone(),
                });
            }
        }

        let mut lane_ids = BTreeSet::new();
        for lane in &self.event_lanes {
            if !lane_ids.insert(lane.lane_id.clone()) {
                violations.push(TraceViewerQualificationViolation::DuplicateId {
                    kind: TraceViewerQualificationViolationKind::EventLane,
                    id: lane.lane_id.clone(),
                });
            }
            if lane.lane_id.trim().is_empty()
                || lane.title.trim().is_empty()
                || lane.thread_or_stream_ref.trim().is_empty()
            {
                violations.push(TraceViewerQualificationViolation::IncompleteEventLane {
                    lane_id: lane.lane_id.clone(),
                });
            }
            if !lane.shows_mapping_quality {
                violations.push(
                    TraceViewerQualificationViolation::EventLaneMissingMappingQuality {
                        lane_id: lane.lane_id.clone(),
                    },
                );
            }
            if !lane.shows_degraded_label {
                violations.push(
                    TraceViewerQualificationViolation::EventLaneMissingDegradedLabel {
                        lane_id: lane.lane_id.clone(),
                    },
                );
            }
        }

        let mut bookmark_ids = BTreeSet::new();
        for bookmark in &self.bookmarks {
            if !bookmark_ids.insert(bookmark.bookmark_id.clone()) {
                violations.push(TraceViewerQualificationViolation::DuplicateId {
                    kind: TraceViewerQualificationViolationKind::Bookmark,
                    id: bookmark.bookmark_id.clone(),
                });
            }
            if bookmark.bookmark_id.trim().is_empty()
                || bookmark.title.trim().is_empty()
                || bookmark.lane_ref.trim().is_empty()
                || bookmark.created_by.trim().is_empty()
            {
                violations.push(TraceViewerQualificationViolation::IncompleteBookmark {
                    bookmark_id: bookmark.bookmark_id.clone(),
                });
            }
            if !bookmark.shows_provenance {
                violations.push(
                    TraceViewerQualificationViolation::BookmarkMissingProvenance {
                        bookmark_id: bookmark.bookmark_id.clone(),
                    },
                );
            }
        }

        let mut fallback_ids = BTreeSet::new();
        for fallback in &self.textual_fallbacks {
            if !fallback_ids.insert(fallback.fallback_id.clone()) {
                violations.push(TraceViewerQualificationViolation::DuplicateId {
                    kind: TraceViewerQualificationViolationKind::TextualFallback,
                    id: fallback.fallback_id.clone(),
                });
            }
            if fallback.fallback_id.trim().is_empty()
                || fallback.title.trim().is_empty()
                || fallback.raw_lines.is_empty()
            {
                violations.push(
                    TraceViewerQualificationViolation::IncompleteTextualFallback {
                        fallback_id: fallback.fallback_id.clone(),
                    },
                );
            }
            if !fallback.shows_mapping_quality {
                violations.push(
                    TraceViewerQualificationViolation::TextualFallbackMissingMappingQuality {
                        fallback_id: fallback.fallback_id.clone(),
                    },
                );
            }
        }

        // Cross-reference: every bookmark must point to a known event lane.
        for bookmark in &self.bookmarks {
            if !lane_ids.contains(&bookmark.lane_ref) {
                violations.push(TraceViewerQualificationViolation::BookmarkLaneRefUnknown {
                    bookmark_id: bookmark.bookmark_id.clone(),
                    lane_ref: bookmark.lane_ref.clone(),
                });
            }
        }

        // Cross-reference: every textual fallback source_event_ref must point to a known lane.
        for fallback in &self.textual_fallbacks {
            for ref_id in &fallback.source_event_refs {
                if !lane_ids.contains(ref_id) {
                    violations.push(
                        TraceViewerQualificationViolation::TextualFallbackSourceRefUnknown {
                            fallback_id: fallback.fallback_id.clone(),
                            source_ref: ref_id.clone(),
                        },
                    );
                }
            }
        }

        if self.summary != self.computed_summary() {
            violations.push(TraceViewerQualificationViolation::SummaryMismatch);
        }

        violations
    }
}

/// Loads the checked-in trace-viewer qualification packet.
///
/// # Errors
///
/// Returns the underlying JSON parse error when the embedded artifact no longer
/// matches the typed model.
pub fn current_trace_viewer_qualification(
) -> Result<TraceViewerQualificationPacket, serde_json::Error> {
    serde_json::from_str(TRACE_VIEWER_QUALIFICATION_PACKET_JSON)
}

/// Identity family used when reporting duplicate ids.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TraceViewerQualificationViolationKind {
    /// Surface rows.
    Surface,
    /// Event-lane rows.
    EventLane,
    /// Bookmark rows.
    Bookmark,
    /// Textual-fallback rows.
    TextualFallback,
}

impl fmt::Display for TraceViewerQualificationViolationKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Surface => write!(f, "surface"),
            Self::EventLane => write!(f, "event_lane"),
            Self::Bookmark => write!(f, "bookmark"),
            Self::TextualFallback => write!(f, "textual_fallback"),
        }
    }
}

/// Validation failure for trace-viewer qualification packets.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TraceViewerQualificationViolation {
    /// Schema version does not match the model.
    SchemaVersion {
        /// Expected schema version.
        expected: u32,
        /// Actual schema version.
        actual: u32,
    },
    /// Record kind does not match the model.
    RecordKind {
        /// Expected record kind.
        expected: String,
        /// Actual record kind.
        actual: String,
    },
    /// IDs must be unique inside an object family.
    DuplicateId {
        /// Kind of object family.
        kind: TraceViewerQualificationViolationKind,
        /// Duplicate id.
        id: String,
    },
    /// A surface with a stable claim has an incomplete guard set.
    IncompleteGuardSet {
        /// Surface id.
        surface_id: String,
    },
    /// An event-lane row is incomplete.
    IncompleteEventLane {
        /// Lane id.
        lane_id: String,
    },
    /// An event-lane row must show its mapping quality.
    EventLaneMissingMappingQuality {
        /// Lane id.
        lane_id: String,
    },
    /// An event-lane row must show a degraded-state label.
    EventLaneMissingDegradedLabel {
        /// Lane id.
        lane_id: String,
    },
    /// A bookmark row is incomplete.
    IncompleteBookmark {
        /// Bookmark id.
        bookmark_id: String,
    },
    /// A bookmark row must show its provenance.
    BookmarkMissingProvenance {
        /// Bookmark id.
        bookmark_id: String,
    },
    /// A textual-fallback row is incomplete.
    IncompleteTextualFallback {
        /// Fallback id.
        fallback_id: String,
    },
    /// A textual-fallback row must show its mapping quality.
    TextualFallbackMissingMappingQuality {
        /// Fallback id.
        fallback_id: String,
    },
    /// A bookmark references an unknown event lane.
    BookmarkLaneRefUnknown {
        /// Bookmark id.
        bookmark_id: String,
        /// Unknown lane ref.
        lane_ref: String,
    },
    /// A textual fallback references an unknown source event or lane.
    TextualFallbackSourceRefUnknown {
        /// Fallback id.
        fallback_id: String,
        /// Unknown source ref.
        source_ref: String,
    },
    /// Computed summary does not match the stored summary.
    SummaryMismatch,
}

impl fmt::Display for TraceViewerQualificationViolation {
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
            Self::DuplicateId { kind, id } => {
                write!(f, "duplicate {kind} id: {id}")
            }
            Self::IncompleteGuardSet { surface_id } => {
                write!(
                    f,
                    "surface {surface_id} claims stable but guard set is incomplete"
                )
            }
            Self::IncompleteEventLane { lane_id } => {
                write!(f, "incomplete event-lane row: {lane_id}")
            }
            Self::EventLaneMissingMappingQuality { lane_id } => {
                write!(f, "event lane {lane_id} must show its mapping quality")
            }
            Self::EventLaneMissingDegradedLabel { lane_id } => {
                write!(f, "event lane {lane_id} must show a degraded-state label")
            }
            Self::IncompleteBookmark { bookmark_id } => {
                write!(f, "incomplete bookmark row: {bookmark_id}")
            }
            Self::BookmarkMissingProvenance { bookmark_id } => {
                write!(f, "bookmark {bookmark_id} must show its provenance")
            }
            Self::IncompleteTextualFallback { fallback_id } => {
                write!(f, "incomplete textual-fallback row: {fallback_id}")
            }
            Self::TextualFallbackMissingMappingQuality { fallback_id } => {
                write!(
                    f,
                    "textual fallback {fallback_id} must show its mapping quality"
                )
            }
            Self::BookmarkLaneRefUnknown {
                bookmark_id,
                lane_ref,
            } => {
                write!(
                    f,
                    "bookmark {bookmark_id} references unknown lane {lane_ref}"
                )
            }
            Self::TextualFallbackSourceRefUnknown {
                fallback_id,
                source_ref,
            } => {
                write!(
                    f,
                    "textual fallback {fallback_id} references unknown source {source_ref}"
                )
            }
            Self::SummaryMismatch => {
                write!(f, "computed summary does not match stored summary")
            }
        }
    }
}

impl Error for TraceViewerQualificationViolation {}
