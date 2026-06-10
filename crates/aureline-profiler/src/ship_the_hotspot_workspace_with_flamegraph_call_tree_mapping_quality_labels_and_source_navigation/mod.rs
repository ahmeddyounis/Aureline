//! Ship the hotspot workspace with flamegraph, call tree, mapping-quality labels, and source navigation.
//!
//! This module materializes the typed records that keep profiler hotspot surfaces
//! honest about what was captured, how symbols map to source, and what navigation
//! actions are safe. The records and closed vocabularies here mirror the boundary
//! schema at
//! `/schemas/perf/ship-the-hotspot-workspace-with-flamegraph-call-tree-mapping-quality-labels-and-source-navigation.schema.json`
//! and reuse the capture-class, provenance, mapping-quality, and source-navigation
//! axes already frozen in `/docs/performance/profiling_trace_replay_contract.md`.
//!
//! The module exposes:
//!
//! - the [`FlamegraphRow`] record that binds frame identity, self and inclusive
//!   metrics, depth, thread, mapping quality, and source ref so flamegraphs never
//!   hide unresolved frames behind pretty charts;
//! - the [`CallTreeRow`] record that carries function or frame identity, self and
//!   inclusive metrics, file/module/service, thread, mapping quality, and caller/callee
//!   refs so call trees support keyboard-first hotspot inspection;
//! - the [`SessionStripRow`] record that carries workload identity, build/runtime
//!   identity, capture mode, mapping quality state, capture time, duration, and
//!   profile posture so every hotspot view leads with attribution before charts;
//! - the [`MappingQualityBadgeRow`] record that carries mapping quality, unresolved
//!   frame count, and imported-symbol note so users never trust false precision;
//! - the [`SourceNavigationRow`] record that carries available actions, default
//!   action, source ref, line number, and mapping quality so source jumps degrade
//!   honestly when fidelity is incomplete;
//! - the [`HotspotWorkspaceQualificationPacket`] checked-in artifact that downstream
//!   docs, help, support, and CI surfaces ingest instead of cloning status text.
//!
//! Raw payload bytes, raw command lines, secrets, and ambient credentials MUST NOT
//! appear on any record carried here.

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

/// Schema version stamped on every hotspot-workspace qualification packet carried
/// by this module. Bumped only on breaking payload changes; additive-optional fields
/// do not bump this value.
pub const HOTSPOT_WORKSPACE_QUALIFICATION_SCHEMA_VERSION: u32 = 1;

/// Stable record-kind tag for [`HotspotWorkspaceQualificationPacket`].
pub const HOTSPOT_WORKSPACE_QUALIFICATION_RECORD_KIND: &str =
    "ship_the_hotspot_workspace_with_flamegraph_call_tree_mapping_quality_labels_and_source_navigation";

/// Repo-relative path to the checked-in hotspot-workspace qualification packet JSON.
pub const HOTSPOT_WORKSPACE_QUALIFICATION_PACKET_PATH: &str =
    "artifacts/perf/m5/ship-the-hotspot-workspace-with-flamegraph-call-tree-mapping-quality-labels-and-source-navigation.json";

/// Embedded checked-in qualification packet JSON.
pub const HOTSPOT_WORKSPACE_QUALIFICATION_PACKET_JSON: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../artifacts/perf/m5/ship-the-hotspot-workspace-with-flamegraph-call-tree-mapping-quality-labels-and-source-navigation.json"
));

/// Qualification label shown on promoted hotspot-workspace surfaces.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum HotspotWorkspaceQualificationLabel {
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

impl HotspotWorkspaceQualificationLabel {
    /// Returns true when the label is a stable claim.
    pub const fn is_stable(self) -> bool {
        matches!(self, Self::Stable)
    }
}

/// Hotspot-workspace surface family governed by this packet.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum HotspotWorkspaceSurfaceKind {
    /// Flamegraph or icicle view.
    FlamegraphView,
    /// Call tree view.
    CallTreeView,
    /// Session strip with workload, build, capture mode, and mapping quality.
    SessionStrip,
    /// Mapping-quality badge or inspector.
    MappingQualityBadge,
    /// Source-navigation actions from hotspots or frames.
    SourceNavigation,
    /// Export review surface for hotspot evidence.
    ExportReview,
    /// Support export surface for hotspot evidence.
    SupportExport,
}

/// Mapping-quality state for symbol-to-source resolution.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MappingQualityLabel {
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

impl MappingQualityLabel {
    /// Returns true when the label allows safe source navigation.
    pub const fn allows_source_navigation(self) -> bool {
        matches!(self, Self::Exact | Self::Approximate | Self::Partial)
    }
}

/// Profile posture describing how the evidence was produced or arrived.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ProfilePosture {
    /// Live capture from the current session.
    LiveCapture,
    /// Imported from an external file or bundle.
    ImportedArtifact,
    /// Cached local evidence from a prior session.
    CachedLocalEvidence,
    /// Stale prior result that may no longer reflect current state.
    StalePriorResult,
}

/// Available source-navigation action from a hotspot or frame.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SourceNavigationAction {
    /// Open the source file at the resolved line.
    OpenSource,
    /// Open the caller of the current frame.
    OpenCaller,
    /// Open a callee of the current frame.
    OpenCallee,
    /// Open the raw symbol or address view.
    OpenRaw,
}

/// One flamegraph row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FlamegraphRow {
    /// Stable frame id.
    pub frame_id: String,
    /// Human-readable title (function or symbol name).
    pub title: String,
    /// Self sample count or metric.
    pub self_samples: u64,
    /// Inclusive sample count or metric.
    pub inclusive_samples: u64,
    /// Depth in the flamegraph stack.
    pub depth: u32,
    /// Thread or process id.
    pub thread_id: String,
    /// Mapping quality for this frame.
    pub mapping_quality: MappingQualityLabel,
    /// Source ref when known.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source_ref: Option<String>,
    /// Module or binary ref.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub module_ref: Option<String>,
    /// True when the frame offers source navigation.
    pub has_source_navigation: bool,
    /// True when the frame shows its mapping quality.
    pub shows_mapping_quality: bool,
}

/// One call-tree row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CallTreeRow {
    /// Stable frame id.
    pub frame_id: String,
    /// Function or frame name.
    pub function_name: String,
    /// Self metric (samples, time, or allocations).
    pub self_metric: u64,
    /// Inclusive metric.
    pub inclusive_metric: u64,
    /// File, module, or service label.
    pub file_module_service: String,
    /// Thread or process id.
    pub thread_id: String,
    /// Mapping quality for this frame.
    pub mapping_quality: MappingQualityLabel,
    /// Source ref when known.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source_ref: Option<String>,
    /// Caller frame refs.
    #[serde(default)]
    pub caller_refs: Vec<String>,
    /// Callee frame refs.
    #[serde(default)]
    pub callee_refs: Vec<String>,
    /// True when the row shows its symbolization state.
    pub shows_symbolization_state: bool,
    /// True when the row offers caller/callee navigation.
    pub has_caller_callee_navigation: bool,
}

/// One session-strip row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SessionStripRow {
    /// Stable strip id.
    pub strip_id: String,
    /// Workload identity.
    pub workload_identity: String,
    /// Build/runtime identity ref.
    pub build_runtime_identity: String,
    /// Capture mode descriptor ref.
    pub capture_mode: String,
    /// Mapping quality state for the session.
    pub mapping_quality_state: MappingQualityLabel,
    /// Capture start time.
    pub capture_time: String,
    /// Duration in milliseconds.
    pub duration_ms: u64,
    /// Profile posture.
    pub profile_posture: ProfilePosture,
    /// True when the strip shows compare/export actions.
    pub shows_compare_export: bool,
    /// True when the strip shows a degraded-state label instead of optimistic text.
    pub shows_degraded_label: bool,
}

/// One mapping-quality badge row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MappingQualityBadgeRow {
    /// Stable badge id.
    pub badge_id: String,
    /// Context id (frame, session, or view).
    pub context_id: String,
    /// Mapping quality for the context.
    pub mapping_quality: MappingQualityLabel,
    /// Unresolved frame count when applicable.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub unresolved_frame_count: Option<u64>,
    /// Imported-symbol note when applicable.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub imported_symbol_note: Option<String>,
    /// True when the badge shows its mapping quality.
    pub shows_mapping_quality: bool,
}

/// One source-navigation row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SourceNavigationRow {
    /// Stable navigation id.
    pub navigation_id: String,
    /// Frame id this navigation belongs to.
    pub frame_id: String,
    /// Available actions.
    #[serde(default)]
    pub available_actions: Vec<SourceNavigationAction>,
    /// Default action.
    pub default_action: SourceNavigationAction,
    /// Source ref when known.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source_ref: Option<String>,
    /// Line number when known.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub line_number: Option<u64>,
    /// Mapping quality before navigation.
    pub mapping_quality: MappingQualityLabel,
    /// True when the navigation shows mapping quality before the jump.
    pub shows_mapping_quality_before_jump: bool,
}

/// Checked-in proof bundle for one surface row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HotspotWorkspaceQualificationProof {
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
pub struct HotspotWorkspaceQualificationSummary {
    /// Total number of flamegraph rows.
    pub flamegraph_row_count: usize,
    /// Total number of call-tree rows.
    pub call_tree_row_count: usize,
    /// Total number of session-strip rows.
    pub session_strip_count: usize,
    /// Total number of mapping-quality badge rows.
    pub mapping_quality_badge_count: usize,
    /// Total number of source-navigation rows.
    pub source_navigation_count: usize,
    /// Number of surfaces claiming stable.
    pub stable_count: usize,
    /// Number of surfaces below stable.
    pub below_stable_count: usize,
    /// True when every row below stable has a non-empty disclosure ref.
    pub all_below_stable_have_disclosure: bool,
}

/// Guard set for a hotspot-workspace surface qualification row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HotspotWorkspaceSurfaceGuardSet {
    /// Session strip is visible.
    pub session_strip_visible: bool,
    /// Mapping quality is visible.
    pub mapping_quality_visible: bool,
    /// Source navigation is visible.
    pub source_navigation_visible: bool,
    /// Flamegraph is visible.
    pub flamegraph_visible: bool,
    /// Call tree is visible.
    pub call_tree_visible: bool,
    /// Thread or process filter is visible.
    pub thread_filter_visible: bool,
    /// Export posture is visible.
    pub export_posture_visible: bool,
    /// Degraded-state label is visible when applicable.
    pub degraded_state_label_visible: bool,
}

/// One surface qualification row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HotspotWorkspaceSurfaceQualificationRow {
    /// Surface id.
    pub surface_id: String,
    /// Surface title.
    pub title: String,
    /// Surface kind.
    pub surface_kind: HotspotWorkspaceSurfaceKind,
    /// True when the surface is present in the promoted build.
    pub promoted_build_surface: bool,
    /// Claim label.
    pub claim_label: HotspotWorkspaceQualificationLabel,
    /// Displayed label (may differ from claim when narrowed).
    pub displayed_label: String,
    /// Qualification proof bundle.
    pub qualification_packet: HotspotWorkspaceQualificationProof,
    /// Guard set.
    pub guards: HotspotWorkspaceSurfaceGuardSet,
    /// True when the surface downgrades if required guards are missing.
    pub downgrade_if_missing: bool,
    /// Rationale string.
    pub rationale: String,
}

/// The checked-in hotspot-workspace qualification packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HotspotWorkspaceQualificationPacket {
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
    pub surfaces: Vec<HotspotWorkspaceSurfaceQualificationRow>,
    /// Flamegraph rows.
    pub flamegraph_rows: Vec<FlamegraphRow>,
    /// Call-tree rows.
    pub call_tree_rows: Vec<CallTreeRow>,
    /// Session-strip rows.
    pub session_strips: Vec<SessionStripRow>,
    /// Mapping-quality badge rows.
    pub mapping_quality_badges: Vec<MappingQualityBadgeRow>,
    /// Source-navigation rows.
    pub source_navigations: Vec<SourceNavigationRow>,
    /// Summary.
    pub summary: HotspotWorkspaceQualificationSummary,
}

impl HotspotWorkspaceQualificationPacket {
    /// Computes the summary from current rows.
    pub fn computed_summary(&self) -> HotspotWorkspaceQualificationSummary {
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

        HotspotWorkspaceQualificationSummary {
            flamegraph_row_count: self.flamegraph_rows.len(),
            call_tree_row_count: self.call_tree_rows.len(),
            session_strip_count: self.session_strips.len(),
            mapping_quality_badge_count: self.mapping_quality_badges.len(),
            source_navigation_count: self.source_navigations.len(),
            stable_count,
            below_stable_count,
            all_below_stable_have_disclosure,
        }
    }

    /// Validates the packet and returns any violations.
    pub fn validate(&self) -> Vec<HotspotWorkspaceQualificationViolation> {
        let mut violations = Vec::new();

        if self.schema_version != HOTSPOT_WORKSPACE_QUALIFICATION_SCHEMA_VERSION {
            violations.push(HotspotWorkspaceQualificationViolation::SchemaVersion {
                expected: HOTSPOT_WORKSPACE_QUALIFICATION_SCHEMA_VERSION,
                actual: self.schema_version,
            });
        }

        if self.record_kind != HOTSPOT_WORKSPACE_QUALIFICATION_RECORD_KIND {
            violations.push(HotspotWorkspaceQualificationViolation::RecordKind {
                expected: HOTSPOT_WORKSPACE_QUALIFICATION_RECORD_KIND.to_owned(),
                actual: self.record_kind.clone(),
            });
        }

        let mut surface_ids = BTreeSet::new();
        for surface in &self.surfaces {
            if !surface_ids.insert(surface.surface_id.clone()) {
                violations.push(HotspotWorkspaceQualificationViolation::DuplicateId {
                    kind: HotspotWorkspaceQualificationViolationKind::Surface,
                    id: surface.surface_id.clone(),
                });
            }
            if surface.promoted_build_surface
                && surface.claim_label.is_stable()
                && (!surface.guards.session_strip_visible
                    || !surface.guards.mapping_quality_visible
                    || !surface.guards.source_navigation_visible
                    || !surface.guards.flamegraph_visible
                    || !surface.guards.call_tree_visible)
            {
                violations.push(HotspotWorkspaceQualificationViolation::IncompleteGuardSet {
                    surface_id: surface.surface_id.clone(),
                });
            }
        }

        let mut frame_ids = BTreeSet::new();
        for row in &self.flamegraph_rows {
            if !frame_ids.insert(row.frame_id.clone()) {
                violations.push(HotspotWorkspaceQualificationViolation::DuplicateId {
                    kind: HotspotWorkspaceQualificationViolationKind::FlamegraphRow,
                    id: row.frame_id.clone(),
                });
            }
            if row.frame_id.trim().is_empty()
                || row.title.trim().is_empty()
                || row.thread_id.trim().is_empty()
            {
                violations.push(
                    HotspotWorkspaceQualificationViolation::IncompleteFlamegraphRow {
                        frame_id: row.frame_id.clone(),
                    },
                );
            }
            if !row.shows_mapping_quality {
                violations.push(
                    HotspotWorkspaceQualificationViolation::FlamegraphRowMissingMappingQuality {
                        frame_id: row.frame_id.clone(),
                    },
                );
            }
        }

        let mut call_tree_ids = BTreeSet::new();
        for row in &self.call_tree_rows {
            if !call_tree_ids.insert(row.frame_id.clone()) {
                violations.push(HotspotWorkspaceQualificationViolation::DuplicateId {
                    kind: HotspotWorkspaceQualificationViolationKind::CallTreeRow,
                    id: row.frame_id.clone(),
                });
            }
            if row.frame_id.trim().is_empty()
                || row.function_name.trim().is_empty()
                || row.file_module_service.trim().is_empty()
                || row.thread_id.trim().is_empty()
            {
                violations.push(
                    HotspotWorkspaceQualificationViolation::IncompleteCallTreeRow {
                        frame_id: row.frame_id.clone(),
                    },
                );
            }
            if !row.shows_symbolization_state {
                violations.push(
                    HotspotWorkspaceQualificationViolation::CallTreeRowMissingSymbolizationState {
                        frame_id: row.frame_id.clone(),
                    },
                );
            }
        }

        let mut strip_ids = BTreeSet::new();
        for row in &self.session_strips {
            if !strip_ids.insert(row.strip_id.clone()) {
                violations.push(HotspotWorkspaceQualificationViolation::DuplicateId {
                    kind: HotspotWorkspaceQualificationViolationKind::SessionStrip,
                    id: row.strip_id.clone(),
                });
            }
            if row.strip_id.trim().is_empty()
                || row.workload_identity.trim().is_empty()
                || row.build_runtime_identity.trim().is_empty()
                || row.capture_mode.trim().is_empty()
                || row.capture_time.trim().is_empty()
            {
                violations.push(
                    HotspotWorkspaceQualificationViolation::IncompleteSessionStrip {
                        strip_id: row.strip_id.clone(),
                    },
                );
            }
            if !row.shows_degraded_label {
                violations.push(
                    HotspotWorkspaceQualificationViolation::SessionStripMissingDegradedLabel {
                        strip_id: row.strip_id.clone(),
                    },
                );
            }
        }

        let mut badge_ids = BTreeSet::new();
        for row in &self.mapping_quality_badges {
            if !badge_ids.insert(row.badge_id.clone()) {
                violations.push(HotspotWorkspaceQualificationViolation::DuplicateId {
                    kind: HotspotWorkspaceQualificationViolationKind::MappingQualityBadge,
                    id: row.badge_id.clone(),
                });
            }
            if row.badge_id.trim().is_empty() || row.context_id.trim().is_empty() {
                violations.push(
                    HotspotWorkspaceQualificationViolation::IncompleteMappingQualityBadge {
                        badge_id: row.badge_id.clone(),
                    },
                );
            }
            if !row.shows_mapping_quality {
                violations.push(
                    HotspotWorkspaceQualificationViolation::MappingQualityBadgeMissingLabel {
                        badge_id: row.badge_id.clone(),
                    },
                );
            }
        }

        let mut navigation_ids = BTreeSet::new();
        for row in &self.source_navigations {
            if !navigation_ids.insert(row.navigation_id.clone()) {
                violations.push(HotspotWorkspaceQualificationViolation::DuplicateId {
                    kind: HotspotWorkspaceQualificationViolationKind::SourceNavigation,
                    id: row.navigation_id.clone(),
                });
            }
            if row.navigation_id.trim().is_empty() || row.frame_id.trim().is_empty() {
                violations.push(
                    HotspotWorkspaceQualificationViolation::IncompleteSourceNavigation {
                        navigation_id: row.navigation_id.clone(),
                    },
                );
            }
            if !row.shows_mapping_quality_before_jump {
                violations.push(
                    HotspotWorkspaceQualificationViolation::SourceNavigationMissingMappingQuality {
                        navigation_id: row.navigation_id.clone(),
                    },
                );
            }
        }

        // Cross-reference: every source navigation must point to a known flamegraph or call-tree frame.
        let frame_id_set: BTreeSet<String> = self
            .flamegraph_rows
            .iter()
            .map(|r| r.frame_id.clone())
            .chain(self.call_tree_rows.iter().map(|r| r.frame_id.clone()))
            .collect();
        for nav in &self.source_navigations {
            if !frame_id_set.contains(&nav.frame_id) {
                violations.push(
                    HotspotWorkspaceQualificationViolation::SourceNavigationFrameRefUnknown {
                        navigation_id: nav.navigation_id.clone(),
                        frame_id: nav.frame_id.clone(),
                    },
                );
            }
        }

        // Cross-reference: every mapping-quality badge must point to a known flamegraph or call-tree frame or session strip.
        let context_id_set: BTreeSet<String> = self
            .flamegraph_rows
            .iter()
            .map(|r| r.frame_id.clone())
            .chain(self.call_tree_rows.iter().map(|r| r.frame_id.clone()))
            .chain(self.session_strips.iter().map(|r| r.strip_id.clone()))
            .collect();
        for badge in &self.mapping_quality_badges {
            if !context_id_set.contains(&badge.context_id) {
                violations.push(
                    HotspotWorkspaceQualificationViolation::MappingQualityBadgeContextRefUnknown {
                        badge_id: badge.badge_id.clone(),
                        context_id: badge.context_id.clone(),
                    },
                );
            }
        }

        if self.summary != self.computed_summary() {
            violations.push(HotspotWorkspaceQualificationViolation::SummaryMismatch);
        }

        violations
    }
}

/// Loads the checked-in hotspot-workspace qualification packet.
///
/// # Errors
///
/// Returns the underlying JSON parse error when the embedded artifact no longer
/// matches the typed model.
pub fn current_hotspot_workspace_qualification(
) -> Result<HotspotWorkspaceQualificationPacket, serde_json::Error> {
    serde_json::from_str(HOTSPOT_WORKSPACE_QUALIFICATION_PACKET_JSON)
}

/// Identity family used when reporting duplicate ids.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HotspotWorkspaceQualificationViolationKind {
    /// Surface rows.
    Surface,
    /// Flamegraph rows.
    FlamegraphRow,
    /// Call-tree rows.
    CallTreeRow,
    /// Session-strip rows.
    SessionStrip,
    /// Mapping-quality badge rows.
    MappingQualityBadge,
    /// Source-navigation rows.
    SourceNavigation,
}

impl fmt::Display for HotspotWorkspaceQualificationViolationKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Surface => write!(f, "surface"),
            Self::FlamegraphRow => write!(f, "flamegraph_row"),
            Self::CallTreeRow => write!(f, "call_tree_row"),
            Self::SessionStrip => write!(f, "session_strip"),
            Self::MappingQualityBadge => write!(f, "mapping_quality_badge"),
            Self::SourceNavigation => write!(f, "source_navigation"),
        }
    }
}

/// Validation failure for hotspot-workspace qualification packets.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum HotspotWorkspaceQualificationViolation {
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
        kind: HotspotWorkspaceQualificationViolationKind,
        /// Duplicate id.
        id: String,
    },
    /// A surface with a stable claim has an incomplete guard set.
    IncompleteGuardSet {
        /// Surface id.
        surface_id: String,
    },
    /// A flamegraph row is incomplete.
    IncompleteFlamegraphRow {
        /// Frame id.
        frame_id: String,
    },
    /// A flamegraph row must show mapping quality.
    FlamegraphRowMissingMappingQuality {
        /// Frame id.
        frame_id: String,
    },
    /// A call-tree row is incomplete.
    IncompleteCallTreeRow {
        /// Frame id.
        frame_id: String,
    },
    /// A call-tree row must show symbolization state.
    CallTreeRowMissingSymbolizationState {
        /// Frame id.
        frame_id: String,
    },
    /// A session-strip row is incomplete.
    IncompleteSessionStrip {
        /// Strip id.
        strip_id: String,
    },
    /// A session-strip row must show a degraded-state label.
    SessionStripMissingDegradedLabel {
        /// Strip id.
        strip_id: String,
    },
    /// A mapping-quality badge row is incomplete.
    IncompleteMappingQualityBadge {
        /// Badge id.
        badge_id: String,
    },
    /// A mapping-quality badge row must show its mapping quality.
    MappingQualityBadgeMissingLabel {
        /// Badge id.
        badge_id: String,
    },
    /// A source-navigation row is incomplete.
    IncompleteSourceNavigation {
        /// Navigation id.
        navigation_id: String,
    },
    /// A source-navigation row must show mapping quality before the jump.
    SourceNavigationMissingMappingQuality {
        /// Navigation id.
        navigation_id: String,
    },
    /// A source navigation references an unknown frame.
    SourceNavigationFrameRefUnknown {
        /// Navigation id.
        navigation_id: String,
        /// Unknown frame id.
        frame_id: String,
    },
    /// A mapping-quality badge references an unknown context.
    MappingQualityBadgeContextRefUnknown {
        /// Badge id.
        badge_id: String,
        /// Unknown context id.
        context_id: String,
    },
    /// Computed summary does not match the stored summary.
    SummaryMismatch,
}

impl fmt::Display for HotspotWorkspaceQualificationViolation {
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
            Self::IncompleteFlamegraphRow { frame_id } => {
                write!(f, "incomplete flamegraph row: {frame_id}")
            }
            Self::FlamegraphRowMissingMappingQuality { frame_id } => {
                write!(f, "flamegraph row {frame_id} must show mapping quality")
            }
            Self::IncompleteCallTreeRow { frame_id } => {
                write!(f, "incomplete call-tree row: {frame_id}")
            }
            Self::CallTreeRowMissingSymbolizationState { frame_id } => {
                write!(f, "call-tree row {frame_id} must show symbolization state")
            }
            Self::IncompleteSessionStrip { strip_id } => {
                write!(f, "incomplete session-strip row: {strip_id}")
            }
            Self::SessionStripMissingDegradedLabel { strip_id } => {
                write!(
                    f,
                    "session strip {strip_id} must show a degraded-state label"
                )
            }
            Self::IncompleteMappingQualityBadge { badge_id } => {
                write!(f, "incomplete mapping-quality badge row: {badge_id}")
            }
            Self::MappingQualityBadgeMissingLabel { badge_id } => {
                write!(
                    f,
                    "mapping-quality badge {badge_id} must show its mapping quality"
                )
            }
            Self::IncompleteSourceNavigation { navigation_id } => {
                write!(f, "incomplete source-navigation row: {navigation_id}")
            }
            Self::SourceNavigationMissingMappingQuality { navigation_id } => {
                write!(
                    f,
                    "source-navigation row {navigation_id} must show mapping quality before jump"
                )
            }
            Self::SourceNavigationFrameRefUnknown {
                navigation_id,
                frame_id,
            } => {
                write!(
                    f,
                    "source navigation {navigation_id} references unknown frame {frame_id}"
                )
            }
            Self::MappingQualityBadgeContextRefUnknown {
                badge_id,
                context_id,
            } => {
                write!(
                    f,
                    "mapping-quality badge {badge_id} references unknown context {context_id}"
                )
            }
            Self::SummaryMismatch => {
                write!(f, "computed summary does not match stored summary")
            }
        }
    }
}

impl Error for HotspotWorkspaceQualificationViolation {}
