//! Chronology and reverse-step controls, history partiality cues, and import or export packets.
//!
//! This module materializes the typed records that keep chronology navigation,
//! reverse-step debugging, history coverage honesty, and packet import/export
//! surfaces attributable and inspectable. The records and closed vocabularies here
//! mirror the boundary schema at
//! `/schemas/perf/add-chronology-and-reverse-step-controls-history-partiality-cues-and-import-or-export-packets.schema.json`
//! and reuse the capture-class, provenance, mapping-quality, and environment-identity
//! axes already frozen in `/docs/performance/profiling_trace_replay_contract.md`.
//!
//! The module exposes:
//!
//! - the [`ChronologyControlRow`] record that binds control identity, kind, enabled
//!   state, and visibility flags so users always know which navigation verbs are
//!   available and why;
//! - the [`ReverseStepActionRow`] record that binds action identity, kind, target
//!   event ref, and mapping quality so reverse-step surfaces never promise steps
//!   they cannot deliver;
//! - the [`HistoryPartialityCueRow`] record that binds cue identity, kind, time
//!   range, severity, and explanation so incomplete, truncated, or filtered history
//!   is never shown as a complete canvas;
//! - the [`ImportExportPacketRow`] record that binds packet identity, direction,
//!   format, provenance, integrity hash, and content summary so imported or exported
//!   evidence is always traceable to its origin and build;
//! - the [`ChronologyQualificationPacket`] checked-in artifact that downstream docs,
//!   help, support, and CI surfaces ingest instead of cloning status text.
//!
//! Raw payload bytes, raw command lines, secrets, and ambient credentials MUST NOT
//! appear on any record carried here.

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

/// Schema version stamped on every chronology qualification packet carried by this module.
/// Bumped only on breaking payload changes; additive-optional fields do not bump this value.
pub const CHRONOLOGY_QUALIFICATION_SCHEMA_VERSION: u32 = 1;

/// Stable record-kind tag for [`ChronologyQualificationPacket`].
pub const CHRONOLOGY_QUALIFICATION_RECORD_KIND: &str =
    "add_chronology_and_reverse_step_controls_history_partiality_cues_and_import_or_export_packets";

/// Repo-relative path to the checked-in chronology qualification packet JSON.
pub const CHRONOLOGY_QUALIFICATION_PACKET_PATH: &str =
    "artifacts/perf/m5/add-chronology-and-reverse-step-controls-history-partiality-cues-and-import-or-export-packets.json";

/// Embedded checked-in qualification packet JSON.
pub const CHRONOLOGY_QUALIFICATION_PACKET_JSON: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../artifacts/perf/m5/add-chronology-and-reverse-step-controls-history-partiality-cues-and-import-or-export-packets.json"
));

/// Qualification label shown on promoted chronology surfaces.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ChronologyQualificationLabel {
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

impl ChronologyQualificationLabel {
    /// Returns true when the label is a stable claim.
    pub const fn is_stable(self) -> bool {
        matches!(self, Self::Stable)
    }
}

/// Chronology surface family governed by this packet.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ChronologySurfaceKind {
    /// Chronology control bar with play, pause, scrub, and speed controls.
    ChronologyControlBar,
    /// Reverse-step toolbar with reverse-continue, reverse-step-over, etc.
    ReverseStepToolbar,
    /// History partiality indicator showing incomplete or truncated coverage.
    HistoryPartialityIndicator,
    /// Import packet dialog for importing trace or profile evidence.
    ImportPacketDialog,
    /// Export packet dialog for exporting trace or profile evidence.
    ExportPacketDialog,
    /// Packet integrity review surface.
    PacketIntegrityReview,
}

/// Kind of chronology control.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ChronologyControlKind {
    /// Play forward from current position.
    Play,
    /// Pause at current position.
    Pause,
    /// Step forward one event or frame.
    StepForward,
    /// Step backward one event or frame.
    StepBackward,
    /// Jump to an absolute timestamp.
    JumpToTime,
    /// Adjust playback speed.
    SpeedControl,
    /// Scrub along the timeline.
    Scrub,
}

/// Kind of reverse-step action.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ReverseStepActionKind {
    /// Continue execution in reverse.
    ReverseContinue,
    /// Step over one function call in reverse.
    ReverseStepOver,
    /// Step into one function call in reverse.
    ReverseStepInto,
    /// Step out of the current function in reverse.
    ReverseStepOut,
    /// Run backwards to the cursor position.
    ReverseRunToCursor,
}

/// Kind of history partiality cue.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum HistoryPartialityCueKind {
    /// History is truncated at the start.
    TruncatedStart,
    /// History is truncated at the end.
    TruncatedEnd,
    /// Some events were filtered out.
    FilteredOut,
    /// Mapping is missing for some events.
    MissingMapping,
    /// Sampling gap caused missing events.
    SamplingGap,
    /// Events were redacted by policy.
    PolicyRedacted,
}

/// Severity of a history partiality cue.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PartialitySeverity {
    /// Informational; history is mostly complete.
    Info,
    /// Warning; a noticeable portion of history is missing.
    Warning,
    /// Critical; history is substantially incomplete or misleading.
    Critical,
}

impl PartialitySeverity {
    /// Returns true when the severity blocks a stable claim for the surface.
    pub const fn blocks_stable_claim(self) -> bool {
        matches!(self, Self::Critical)
    }

    /// Returns true when the severity should show a degraded-state label.
    pub const fn shows_degraded_label(self) -> bool {
        matches!(self, Self::Warning | Self::Critical)
    }
}

/// Direction of an import or export packet.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PacketDirection {
    /// Packet is being imported into the workspace.
    Import,
    /// Packet is being exported from the workspace.
    Export,
}

impl PacketDirection {
    /// Returns true when the direction is import.
    pub const fn is_import(self) -> bool {
        matches!(self, Self::Import)
    }

    /// Returns true when the direction is export.
    pub const fn is_export(self) -> bool {
        matches!(self, Self::Export)
    }
}

/// Format kind for an import or export packet.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PacketFormatKind {
    /// Trace bundle format.
    TraceBundle,
    /// Profile snapshot format.
    ProfileSnapshot,
    /// Regression baseline format.
    RegressionBaseline,
    /// Notebook archive format.
    NotebookArchive,
    /// Replay capture format.
    ReplayCapture,
}

/// One chronology control row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ChronologyControlRow {
    /// Stable control row id.
    pub control_id: String,
    /// Human-readable title.
    pub title: String,
    /// Control kind.
    pub control_kind: ChronologyControlKind,
    /// True when the control is enabled in the current context.
    pub enabled: bool,
    /// True when the control shows its enabled state.
    pub shows_enabled_state: bool,
    /// True when the control shows a degraded-state label when applicable.
    pub shows_degraded_label: bool,
}

/// One reverse-step action row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReverseStepActionRow {
    /// Stable action row id.
    pub action_id: String,
    /// Human-readable title.
    pub title: String,
    /// Action kind.
    pub action_kind: ReverseStepActionKind,
    /// True when the action is enabled in the current context.
    pub enabled: bool,
    /// Target event or frame ref.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target_event_ref: Option<String>,
    /// Mapping quality label for the target.
    pub mapping_quality: String,
    /// True when the action shows its enabled state.
    pub shows_enabled_state: bool,
    /// True when the action shows its mapping quality.
    pub shows_mapping_quality: bool,
    /// True when the action shows a degraded-state label when applicable.
    pub shows_degraded_label: bool,
}

/// One history partiality cue row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HistoryPartialityCueRow {
    /// Stable cue row id.
    pub cue_id: String,
    /// Human-readable title.
    pub title: String,
    /// Cue kind.
    pub cue_kind: HistoryPartialityCueKind,
    /// Start timestamp in nanoseconds when the partiality begins.
    pub time_range_start_ns: u64,
    /// End timestamp in nanoseconds when the partiality ends.
    pub time_range_end_ns: u64,
    /// Severity.
    pub severity: PartialitySeverity,
    /// True when the cue shows its severity.
    pub shows_severity: bool,
    /// True when the cue shows an explanation.
    pub shows_explanation: bool,
    /// Explanation text.
    pub explanation: String,
    /// True when the cue shows a degraded-state label when applicable.
    pub shows_degraded_label: bool,
}

/// One import or export packet row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ImportExportPacketRow {
    /// Stable packet row id.
    pub packet_row_id: String,
    /// Human-readable title.
    pub title: String,
    /// Packet direction.
    pub direction: PacketDirection,
    /// Format kind.
    pub format_kind: PacketFormatKind,
    /// Format version string.
    pub format_version: String,
    /// Provenance ref.
    pub provenance: String,
    /// Integrity hash or checksum.
    pub integrity_hash: String,
    /// Creation timestamp.
    pub created_at: String,
    /// Size in bytes.
    pub size_bytes: u64,
    /// Content summary.
    pub content_summary: String,
    /// True when the packet shows its provenance.
    pub shows_provenance: bool,
    /// True when the packet shows its integrity hash.
    pub shows_integrity_hash: bool,
    /// True when the packet shows its format version.
    pub shows_format_version: bool,
    /// True when the packet shows a degraded-state label when applicable.
    pub shows_degraded_label: bool,
}

/// Checked-in proof bundle for one surface row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ChronologyQualificationProof {
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

/// Summary projected onto help, release, and support surfaces.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ChronologyQualificationSummary {
    /// Total number of chronology control rows.
    pub chronology_control_count: usize,
    /// Total number of reverse-step action rows.
    pub reverse_step_action_count: usize,
    /// Total number of history partiality cue rows.
    pub history_partiality_cue_count: usize,
    /// Total number of import/export packet rows.
    pub import_export_packet_count: usize,
    /// Number of rows claiming stable.
    pub stable_count: usize,
    /// Number of rows below stable.
    pub below_stable_count: usize,
    /// True when every row has a non-empty disclosure ref if below stable.
    pub all_below_stable_have_disclosure: bool,
    /// Number of controls showing degraded labels.
    pub control_showing_degraded_count: usize,
    /// Number of reverse-step actions showing degraded labels.
    pub reverse_step_showing_degraded_count: usize,
    /// Number of partiality cues showing degraded labels.
    pub partiality_cue_showing_degraded_count: usize,
    /// Number of import/export packets showing degraded labels.
    pub packet_showing_degraded_count: usize,
}

/// Guard set for a chronology surface qualification row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ChronologySurfaceGuardSet {
    /// Chronology controls are visible.
    pub chronology_controls_visible: bool,
    /// Reverse-step actions are visible.
    pub reverse_step_actions_visible: bool,
    /// History partiality cues are visible.
    pub history_partiality_cues_visible: bool,
    /// Import/export packets are visible.
    pub import_export_packets_visible: bool,
    /// Mapping quality is visible.
    pub mapping_quality_visible: bool,
    /// Degraded-state label is visible when applicable.
    pub degraded_state_label_visible: bool,
    /// Integrity check is visible.
    pub integrity_check_visible: bool,
}

/// One surface qualification row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ChronologySurfaceQualificationRow {
    /// Surface id.
    pub surface_id: String,
    /// Surface title.
    pub title: String,
    /// Surface kind.
    pub surface_kind: ChronologySurfaceKind,
    /// True when the surface is present in the promoted build.
    pub promoted_build_surface: bool,
    /// Claim label.
    pub claim_label: ChronologyQualificationLabel,
    /// Displayed label (may differ from claim when narrowed).
    pub displayed_label: String,
    /// Qualification proof bundle.
    pub qualification_packet: ChronologyQualificationProof,
    /// Guard set.
    pub guards: ChronologySurfaceGuardSet,
    /// True when the surface downgrades if required guards are missing.
    pub downgrade_if_missing: bool,
    /// Rationale string.
    pub rationale: String,
}

/// The checked-in chronology qualification packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ChronologyQualificationPacket {
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
    pub surfaces: Vec<ChronologySurfaceQualificationRow>,
    /// Chronology control rows.
    pub chronology_controls: Vec<ChronologyControlRow>,
    /// Reverse-step action rows.
    pub reverse_step_actions: Vec<ReverseStepActionRow>,
    /// History partiality cue rows.
    pub history_partiality_cues: Vec<HistoryPartialityCueRow>,
    /// Import/export packet rows.
    pub import_export_packets: Vec<ImportExportPacketRow>,
    /// Summary.
    pub summary: ChronologyQualificationSummary,
}

impl ChronologyQualificationPacket {
    /// Computes the summary from current rows.
    pub fn computed_summary(&self) -> ChronologyQualificationSummary {
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
        let control_showing_degraded_count = self
            .chronology_controls
            .iter()
            .filter(|c| c.shows_degraded_label)
            .count();
        let reverse_step_showing_degraded_count = self
            .reverse_step_actions
            .iter()
            .filter(|a| a.shows_degraded_label)
            .count();
        let partiality_cue_showing_degraded_count = self
            .history_partiality_cues
            .iter()
            .filter(|c| c.shows_degraded_label)
            .count();
        let packet_showing_degraded_count = self
            .import_export_packets
            .iter()
            .filter(|p| p.shows_degraded_label)
            .count();

        ChronologyQualificationSummary {
            chronology_control_count: self.chronology_controls.len(),
            reverse_step_action_count: self.reverse_step_actions.len(),
            history_partiality_cue_count: self.history_partiality_cues.len(),
            import_export_packet_count: self.import_export_packets.len(),
            stable_count,
            below_stable_count,
            all_below_stable_have_disclosure,
            control_showing_degraded_count,
            reverse_step_showing_degraded_count,
            partiality_cue_showing_degraded_count,
            packet_showing_degraded_count,
        }
    }

    /// Validates the packet and returns any violations.
    pub fn validate(&self) -> Vec<ChronologyQualificationViolation> {
        let mut violations = Vec::new();

        if self.schema_version != CHRONOLOGY_QUALIFICATION_SCHEMA_VERSION {
            violations.push(ChronologyQualificationViolation::SchemaVersion {
                expected: CHRONOLOGY_QUALIFICATION_SCHEMA_VERSION,
                actual: self.schema_version,
            });
        }

        if self.record_kind != CHRONOLOGY_QUALIFICATION_RECORD_KIND {
            violations.push(ChronologyQualificationViolation::RecordKind {
                expected: CHRONOLOGY_QUALIFICATION_RECORD_KIND.to_owned(),
                actual: self.record_kind.clone(),
            });
        }

        let mut surface_ids = BTreeSet::new();
        for surface in &self.surfaces {
            if !surface_ids.insert(surface.surface_id.clone()) {
                violations.push(ChronologyQualificationViolation::DuplicateId {
                    kind: ChronologyQualificationViolationKind::Surface,
                    id: surface.surface_id.clone(),
                });
            }
            if surface.promoted_build_surface
                && surface.claim_label.is_stable()
                && (!surface.guards.chronology_controls_visible
                    || !surface.guards.reverse_step_actions_visible
                    || !surface.guards.history_partiality_cues_visible
                    || !surface.guards.import_export_packets_visible
                    || !surface.guards.mapping_quality_visible
                    || !surface.guards.degraded_state_label_visible
                    || !surface.guards.integrity_check_visible)
            {
                violations.push(ChronologyQualificationViolation::IncompleteGuardSet {
                    surface_id: surface.surface_id.clone(),
                });
            }
        }

        let mut control_ids = BTreeSet::new();
        for control in &self.chronology_controls {
            if !control_ids.insert(control.control_id.clone()) {
                violations.push(ChronologyQualificationViolation::DuplicateId {
                    kind: ChronologyQualificationViolationKind::ChronologyControl,
                    id: control.control_id.clone(),
                });
            }
            if control.control_id.trim().is_empty() || control.title.trim().is_empty() {
                violations.push(
                    ChronologyQualificationViolation::IncompleteChronologyControl {
                        control_id: control.control_id.clone(),
                    },
                );
            }
            if !control.shows_enabled_state {
                violations.push(
                    ChronologyQualificationViolation::ChronologyControlMissingEnabledState {
                        control_id: control.control_id.clone(),
                    },
                );
            }
            if !control.shows_degraded_label {
                violations.push(
                    ChronologyQualificationViolation::ChronologyControlMissingDegradedLabel {
                        control_id: control.control_id.clone(),
                    },
                );
            }
        }

        let mut action_ids = BTreeSet::new();
        for action in &self.reverse_step_actions {
            if !action_ids.insert(action.action_id.clone()) {
                violations.push(ChronologyQualificationViolation::DuplicateId {
                    kind: ChronologyQualificationViolationKind::ReverseStepAction,
                    id: action.action_id.clone(),
                });
            }
            if action.action_id.trim().is_empty() || action.title.trim().is_empty() {
                violations.push(
                    ChronologyQualificationViolation::IncompleteReverseStepAction {
                        action_id: action.action_id.clone(),
                    },
                );
            }
            if !action.shows_enabled_state {
                violations.push(
                    ChronologyQualificationViolation::ReverseStepActionMissingEnabledState {
                        action_id: action.action_id.clone(),
                    },
                );
            }
            if !action.shows_mapping_quality {
                violations.push(
                    ChronologyQualificationViolation::ReverseStepActionMissingMappingQuality {
                        action_id: action.action_id.clone(),
                    },
                );
            }
            if !action.shows_degraded_label {
                violations.push(
                    ChronologyQualificationViolation::ReverseStepActionMissingDegradedLabel {
                        action_id: action.action_id.clone(),
                    },
                );
            }
        }

        let mut cue_ids = BTreeSet::new();
        for cue in &self.history_partiality_cues {
            if !cue_ids.insert(cue.cue_id.clone()) {
                violations.push(ChronologyQualificationViolation::DuplicateId {
                    kind: ChronologyQualificationViolationKind::HistoryPartialityCue,
                    id: cue.cue_id.clone(),
                });
            }
            if cue.cue_id.trim().is_empty()
                || cue.title.trim().is_empty()
                || cue.explanation.trim().is_empty()
            {
                violations.push(
                    ChronologyQualificationViolation::IncompleteHistoryPartialityCue {
                        cue_id: cue.cue_id.clone(),
                    },
                );
            }
            if !cue.shows_severity {
                violations.push(
                    ChronologyQualificationViolation::HistoryPartialityCueMissingSeverity {
                        cue_id: cue.cue_id.clone(),
                    },
                );
            }
            if !cue.shows_explanation {
                violations.push(
                    ChronologyQualificationViolation::HistoryPartialityCueMissingExplanation {
                        cue_id: cue.cue_id.clone(),
                    },
                );
            }
            if cue.severity.shows_degraded_label() && !cue.shows_degraded_label {
                violations.push(
                    ChronologyQualificationViolation::HistoryPartialityCueMissingDegradedLabel {
                        cue_id: cue.cue_id.clone(),
                    },
                );
            }
        }

        let mut packet_row_ids = BTreeSet::new();
        for packet in &self.import_export_packets {
            if !packet_row_ids.insert(packet.packet_row_id.clone()) {
                violations.push(ChronologyQualificationViolation::DuplicateId {
                    kind: ChronologyQualificationViolationKind::ImportExportPacket,
                    id: packet.packet_row_id.clone(),
                });
            }
            if packet.packet_row_id.trim().is_empty()
                || packet.title.trim().is_empty()
                || packet.provenance.trim().is_empty()
                || packet.integrity_hash.trim().is_empty()
                || packet.content_summary.trim().is_empty()
            {
                violations.push(
                    ChronologyQualificationViolation::IncompleteImportExportPacket {
                        packet_row_id: packet.packet_row_id.clone(),
                    },
                );
            }
            if !packet.shows_provenance {
                violations.push(
                    ChronologyQualificationViolation::ImportExportPacketMissingProvenance {
                        packet_row_id: packet.packet_row_id.clone(),
                    },
                );
            }
            if !packet.shows_integrity_hash {
                violations.push(
                    ChronologyQualificationViolation::ImportExportPacketMissingIntegrityHash {
                        packet_row_id: packet.packet_row_id.clone(),
                    },
                );
            }
            if !packet.shows_format_version {
                violations.push(
                    ChronologyQualificationViolation::ImportExportPacketMissingFormatVersion {
                        packet_row_id: packet.packet_row_id.clone(),
                    },
                );
            }
        }

        if self.summary != self.computed_summary() {
            violations.push(ChronologyQualificationViolation::SummaryMismatch);
        }

        violations
    }
}

/// Loads the checked-in chronology qualification packet.
///
/// # Errors
///
/// Returns the underlying JSON parse error when the embedded artifact no longer
/// matches the typed model.
pub fn current_chronology_qualification() -> Result<ChronologyQualificationPacket, serde_json::Error>
{
    serde_json::from_str(CHRONOLOGY_QUALIFICATION_PACKET_JSON)
}

/// Identity family used when reporting duplicate ids.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ChronologyQualificationViolationKind {
    /// Surface rows.
    Surface,
    /// Chronology control rows.
    ChronologyControl,
    /// Reverse-step action rows.
    ReverseStepAction,
    /// History partiality cue rows.
    HistoryPartialityCue,
    /// Import/export packet rows.
    ImportExportPacket,
}

impl fmt::Display for ChronologyQualificationViolationKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Surface => write!(f, "surface"),
            Self::ChronologyControl => write!(f, "chronology_control"),
            Self::ReverseStepAction => write!(f, "reverse_step_action"),
            Self::HistoryPartialityCue => write!(f, "history_partiality_cue"),
            Self::ImportExportPacket => write!(f, "import_export_packet"),
        }
    }
}

/// Validation failure for chronology qualification packets.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ChronologyQualificationViolation {
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
        kind: ChronologyQualificationViolationKind,
        /// Duplicate id.
        id: String,
    },
    /// A surface with a stable claim has an incomplete guard set.
    IncompleteGuardSet {
        /// Surface id.
        surface_id: String,
    },
    /// A chronology control row is incomplete.
    IncompleteChronologyControl {
        /// Control id.
        control_id: String,
    },
    /// A chronology control row must show its enabled state.
    ChronologyControlMissingEnabledState {
        /// Control id.
        control_id: String,
    },
    /// A chronology control row must show a degraded-state label.
    ChronologyControlMissingDegradedLabel {
        /// Control id.
        control_id: String,
    },
    /// A reverse-step action row is incomplete.
    IncompleteReverseStepAction {
        /// Action id.
        action_id: String,
    },
    /// A reverse-step action row must show its enabled state.
    ReverseStepActionMissingEnabledState {
        /// Action id.
        action_id: String,
    },
    /// A reverse-step action row must show its mapping quality.
    ReverseStepActionMissingMappingQuality {
        /// Action id.
        action_id: String,
    },
    /// A reverse-step action row must show a degraded-state label.
    ReverseStepActionMissingDegradedLabel {
        /// Action id.
        action_id: String,
    },
    /// A history partiality cue row is incomplete.
    IncompleteHistoryPartialityCue {
        /// Cue id.
        cue_id: String,
    },
    /// A history partiality cue row must show its severity.
    HistoryPartialityCueMissingSeverity {
        /// Cue id.
        cue_id: String,
    },
    /// A history partiality cue row must show an explanation.
    HistoryPartialityCueMissingExplanation {
        /// Cue id.
        cue_id: String,
    },
    /// A history partiality cue row must show a degraded-state label when applicable.
    HistoryPartialityCueMissingDegradedLabel {
        /// Cue id.
        cue_id: String,
    },
    /// An import/export packet row is incomplete.
    IncompleteImportExportPacket {
        /// Packet row id.
        packet_row_id: String,
    },
    /// An import/export packet row must show its provenance.
    ImportExportPacketMissingProvenance {
        /// Packet row id.
        packet_row_id: String,
    },
    /// An import/export packet row must show its integrity hash.
    ImportExportPacketMissingIntegrityHash {
        /// Packet row id.
        packet_row_id: String,
    },
    /// An import/export packet row must show its format version.
    ImportExportPacketMissingFormatVersion {
        /// Packet row id.
        packet_row_id: String,
    },
    /// Computed summary does not match the stored summary.
    SummaryMismatch,
}

impl fmt::Display for ChronologyQualificationViolation {
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
            Self::IncompleteChronologyControl { control_id } => {
                write!(f, "incomplete chronology control row: {control_id}")
            }
            Self::ChronologyControlMissingEnabledState { control_id } => {
                write!(
                    f,
                    "chronology control {control_id} must show its enabled state"
                )
            }
            Self::ChronologyControlMissingDegradedLabel { control_id } => {
                write!(
                    f,
                    "chronology control {control_id} must show a degraded-state label"
                )
            }
            Self::IncompleteReverseStepAction { action_id } => {
                write!(f, "incomplete reverse-step action row: {action_id}")
            }
            Self::ReverseStepActionMissingEnabledState { action_id } => {
                write!(
                    f,
                    "reverse-step action {action_id} must show its enabled state"
                )
            }
            Self::ReverseStepActionMissingMappingQuality { action_id } => {
                write!(
                    f,
                    "reverse-step action {action_id} must show its mapping quality"
                )
            }
            Self::ReverseStepActionMissingDegradedLabel { action_id } => {
                write!(
                    f,
                    "reverse-step action {action_id} must show a degraded-state label"
                )
            }
            Self::IncompleteHistoryPartialityCue { cue_id } => {
                write!(f, "incomplete history partiality cue row: {cue_id}")
            }
            Self::HistoryPartialityCueMissingSeverity { cue_id } => {
                write!(f, "history partiality cue {cue_id} must show its severity")
            }
            Self::HistoryPartialityCueMissingExplanation { cue_id } => {
                write!(
                    f,
                    "history partiality cue {cue_id} must show an explanation"
                )
            }
            Self::HistoryPartialityCueMissingDegradedLabel { cue_id } => {
                write!(
                    f,
                    "history partiality cue {cue_id} must show a degraded-state label"
                )
            }
            Self::IncompleteImportExportPacket { packet_row_id } => {
                write!(f, "incomplete import/export packet row: {packet_row_id}")
            }
            Self::ImportExportPacketMissingProvenance { packet_row_id } => {
                write!(
                    f,
                    "import/export packet {packet_row_id} must show its provenance"
                )
            }
            Self::ImportExportPacketMissingIntegrityHash { packet_row_id } => {
                write!(
                    f,
                    "import/export packet {packet_row_id} must show its integrity hash"
                )
            }
            Self::ImportExportPacketMissingFormatVersion { packet_row_id } => {
                write!(
                    f,
                    "import/export packet {packet_row_id} must show its format version"
                )
            }
            Self::SummaryMismatch => {
                write!(f, "computed summary does not match stored summary")
            }
        }
    }
}

impl Error for ChronologyQualificationViolation {}
