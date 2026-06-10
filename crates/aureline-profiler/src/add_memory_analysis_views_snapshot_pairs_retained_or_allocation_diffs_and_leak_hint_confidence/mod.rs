//! Add memory-analysis views, snapshot pairs, retained or allocation diffs, and leak-hint confidence.
//!
//! This module materializes the typed records that keep memory-analysis surfaces
//! honest about what snapshots are compared, how retained and allocation diffs are
//! computed, and how leak-hint confidence is communicated. The records and closed
//! vocabularies here mirror the boundary schema at
//! `/schemas/perf/add-memory-analysis-views-snapshot-pairs-retained-or-allocation-diffs-and-leak-hint-confidence.schema.json`
//! and reuse the capture-class, provenance, and mapping-quality axes already frozen in
//! `/docs/performance/profiling_trace_replay_contract.md`.
//!
//! The module exposes:
//!
//! - the [`MemoryAnalysisViewRow`] record that binds view identity, kind, snapshot
//!   pair ref, mapping quality, and degraded-state label so memory views never hide
//!   incomplete comparison data behind aggregate charts;
//! - the [`SnapshotPairRow`] record that carries baseline and comparison snapshot
//!   refs, snapshot kind, comparison basis, and mapping quality so users always know
//!   what is being compared and on what basis;
//! - the [`RetainedDiffRow`] record that carries type or class identity, baseline
//!   and comparison retained bytes, delta, and mapping quality so retained-size
//!   changes are attributable and honest about fidelity;
//! - the [`AllocationDiffRow`] record that carries type or class identity, baseline
//!   and comparison allocation counts and bytes, deltas, and mapping quality so
//!   allocation changes are attributable and honest about fidelity;
//! - the [`LeakHintRow`] record that carries type or class identity, confidence
//!   level, retained bytes, instance count, rationale, and mapping quality so leak
//!   hints do not overstate certainty when evidence is weak;
//! - the [`MemoryAnalysisQualificationPacket`] checked-in artifact that downstream
//!   docs, help, support, and CI surfaces ingest instead of cloning status text.
//!
//! Raw payload bytes, raw command lines, secrets, and ambient credentials MUST NOT
//! appear on any record carried here.

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

/// Schema version stamped on every memory-analysis qualification packet carried by
/// this module. Bumped only on breaking payload changes; additive-optional fields
/// do not bump this value.
pub const MEMORY_ANALYSIS_QUALIFICATION_SCHEMA_VERSION: u32 = 1;

/// Stable record-kind tag for [`MemoryAnalysisQualificationPacket`].
pub const MEMORY_ANALYSIS_QUALIFICATION_RECORD_KIND: &str =
    "add_memory_analysis_views_snapshot_pairs_retained_or_allocation_diffs_and_leak_hint_confidence";

/// Repo-relative path to the checked-in memory-analysis qualification packet JSON.
pub const MEMORY_ANALYSIS_QUALIFICATION_PACKET_PATH: &str =
    "artifacts/perf/m5/add-memory-analysis-views-snapshot-pairs-retained-or-allocation-diffs-and-leak-hint-confidence.json";

/// Embedded checked-in qualification packet JSON.
pub const MEMORY_ANALYSIS_QUALIFICATION_PACKET_JSON: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../artifacts/perf/m5/add-memory-analysis-views-snapshot-pairs-retained-or-allocation-diffs-and-leak-hint-confidence.json"
));

/// Qualification label shown on promoted memory-analysis surfaces.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MemoryAnalysisQualificationLabel {
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

impl MemoryAnalysisQualificationLabel {
    /// Returns true when the label is a stable claim.
    pub const fn is_stable(self) -> bool {
        matches!(self, Self::Stable)
    }
}

/// Memory-analysis surface family governed by this packet.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MemoryAnalysisSurfaceKind {
    /// Retained-size view.
    RetainedSizeView,
    /// Allocation-count view.
    AllocationCountView,
    /// Class histogram view.
    ClassHistogramView,
    /// Instance list view.
    InstanceListView,
    /// Dominator tree view.
    DominatorTreeView,
    /// Diff view (retained or allocation diffs).
    DiffView,
    /// Leak-hint inspector.
    LeakHintView,
    /// Snapshot-pair browser.
    SnapshotPairBrowser,
    /// Export review surface for memory-analysis evidence.
    ExportReview,
    /// Support export surface for memory-analysis evidence.
    SupportExport,
}

/// Kind of memory-analysis view.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MemoryAnalysisViewKind {
    /// Retained-size breakdown by type or package.
    RetainedSizeView,
    /// Allocation-count breakdown by type or package.
    AllocationCountView,
    /// Class histogram with instance counts and sizes.
    ClassHistogramView,
    /// Instance list for a selected type or package.
    InstanceListView,
    /// Dominator tree showing object retention paths.
    DominatorTreeView,
    /// Diff view comparing two snapshots.
    DiffView,
    /// Leak-hint summary view.
    LeakHintView,
}

/// Kind of snapshot captured for memory analysis.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SnapshotKind {
    /// Full heap dump.
    HeapDump,
    /// Allocation trace with call stacks.
    AllocationTrace,
    /// Live object set snapshot.
    LiveObjectSet,
}

/// Basis on which two snapshots are compared.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ComparisonBasis {
    /// Compare retained sizes.
    RetainedDiff,
    /// Compare allocation counts and bytes.
    AllocationDiff,
    /// Compare shallow sizes.
    ShallowDiff,
    /// Compare object counts.
    ObjectCountDiff,
}

/// Confidence level for a leak hint.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LeakHintConfidence {
    /// Strong evidence of a leak.
    High,
    /// Moderate evidence of a leak.
    Medium,
    /// Weak evidence of a leak.
    Low,
    /// Insufficient evidence to claim a leak.
    Uncertain,
}

impl LeakHintConfidence {
    /// Returns true when the confidence is high or medium.
    pub const fn is_actionable(self) -> bool {
        matches!(self, Self::High | Self::Medium)
    }
}

/// Mapping-quality state for symbol-to-type or snapshot-to-snapshot resolution.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MemoryMappingQualityLabel {
    /// Exact symbol and type location.
    Exact,
    /// Approximate match; may be nearest type or package.
    Approximate,
    /// Partial mapping; some generated or obfuscated types.
    Partial,
    /// No mapping available.
    Unavailable,
    /// Mapping is stale relative to current build.
    Stale,
    /// Mapping mismatches the current build.
    Mismatched,
}

/// One memory-analysis view row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MemoryAnalysisViewRow {
    /// Stable view row id.
    pub view_id: String,
    /// Human-readable title.
    pub title: String,
    /// View kind.
    pub view_kind: MemoryAnalysisViewKind,
    /// Snapshot pair ref when applicable.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub snapshot_pair_ref: Option<String>,
    /// Mapping quality state.
    pub mapping_quality: MemoryMappingQualityLabel,
    /// True when the view shows its mapping quality.
    pub shows_mapping_quality: bool,
    /// True when the view shows a degraded-state label.
    pub shows_degraded_label: bool,
}

/// One snapshot-pair row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SnapshotPairRow {
    /// Stable pair row id.
    pub pair_id: String,
    /// Human-readable title.
    pub title: String,
    /// Baseline snapshot ref.
    pub baseline_snapshot_ref: String,
    /// Comparison snapshot ref.
    pub comparison_snapshot_ref: String,
    /// Snapshot kind.
    pub snapshot_kind: SnapshotKind,
    /// Comparison basis.
    pub comparison_basis: ComparisonBasis,
    /// Mapping quality state.
    pub mapping_quality: MemoryMappingQualityLabel,
    /// True when the pair shows its comparison basis.
    pub shows_comparison_basis: bool,
    /// True when the pair shows a degraded-state label.
    pub shows_degraded_label: bool,
}

/// One retained-diff row.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RetainedDiffRow {
    /// Stable diff row id.
    pub diff_id: String,
    /// Human-readable title.
    pub title: String,
    /// Snapshot pair ref.
    pub pair_ref: String,
    /// Type path or class name.
    pub type_path_or_class: String,
    /// Baseline retained bytes.
    pub baseline_retained_bytes: u64,
    /// Comparison retained bytes.
    pub comparison_retained_bytes: u64,
    /// Delta in bytes.
    pub delta_bytes: i64,
    /// Delta as a percentage.
    pub delta_percent: f64,
    /// Mapping quality state.
    pub mapping_quality: MemoryMappingQualityLabel,
    /// True when the diff shows its mapping quality.
    pub shows_mapping_quality: bool,
}

/// One allocation-diff row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AllocationDiffRow {
    /// Stable diff row id.
    pub diff_id: String,
    /// Human-readable title.
    pub title: String,
    /// Snapshot pair ref.
    pub pair_ref: String,
    /// Type path or class name.
    pub type_path_or_class: String,
    /// Baseline allocation count.
    pub baseline_alloc_count: u64,
    /// Comparison allocation count.
    pub comparison_alloc_count: u64,
    /// Baseline allocation bytes.
    pub baseline_alloc_bytes: u64,
    /// Comparison allocation bytes.
    pub comparison_alloc_bytes: u64,
    /// Delta in count.
    pub delta_count: i64,
    /// Delta in bytes.
    pub delta_bytes: i64,
    /// Mapping quality state.
    pub mapping_quality: MemoryMappingQualityLabel,
    /// True when the diff shows its mapping quality.
    pub shows_mapping_quality: bool,
}

/// One leak-hint row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LeakHintRow {
    /// Stable hint row id.
    pub hint_id: String,
    /// Human-readable title.
    pub title: String,
    /// Snapshot pair ref.
    pub pair_ref: String,
    /// Type path or class name.
    pub type_path_or_class: String,
    /// Confidence level.
    pub confidence: LeakHintConfidence,
    /// Retained bytes associated with the hint.
    pub retained_bytes: u64,
    /// Instance count associated with the hint.
    pub instance_count: u64,
    /// Leak rationale.
    pub leak_rationale: String,
    /// True when the hint shows its confidence level.
    pub shows_confidence: bool,
    /// Mapping quality state.
    pub mapping_quality: MemoryMappingQualityLabel,
}

/// Checked-in proof bundle for one surface row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MemoryAnalysisQualificationProof {
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
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MemoryAnalysisQualificationSummary {
    /// Total number of memory-analysis view rows.
    pub view_count: usize,
    /// Total number of snapshot-pair rows.
    pub snapshot_pair_count: usize,
    /// Total number of retained-diff rows.
    pub retained_diff_count: usize,
    /// Total number of allocation-diff rows.
    pub allocation_diff_count: usize,
    /// Total number of leak-hint rows.
    pub leak_hint_count: usize,
    /// Number of surfaces claiming stable.
    pub stable_count: usize,
    /// Number of surfaces below stable.
    pub below_stable_count: usize,
    /// True when every row has a non-empty disclosure ref if below stable.
    pub all_below_stable_have_disclosure: bool,
}

/// Guard set for a memory-analysis surface qualification row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MemoryAnalysisSurfaceGuardSet {
    /// Memory-analysis views are visible.
    pub views_visible: bool,
    /// Snapshot pairs are visible.
    pub snapshot_pairs_visible: bool,
    /// Retained diffs are visible.
    pub retained_diffs_visible: bool,
    /// Allocation diffs are visible.
    pub allocation_diffs_visible: bool,
    /// Leak hints are visible.
    pub leak_hints_visible: bool,
    /// Mapping quality is visible.
    pub mapping_quality_visible: bool,
    /// Export posture is visible.
    pub export_posture_visible: bool,
    /// Degraded-state label is visible when applicable.
    pub degraded_state_label_visible: bool,
}

/// One surface qualification row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MemoryAnalysisSurfaceQualificationRow {
    /// Surface id.
    pub surface_id: String,
    /// Surface title.
    pub title: String,
    /// Surface kind.
    pub surface_kind: MemoryAnalysisSurfaceKind,
    /// True when the surface is present in the promoted build.
    pub promoted_build_surface: bool,
    /// Claim label.
    pub claim_label: MemoryAnalysisQualificationLabel,
    /// Displayed label (may differ from claim when narrowed).
    pub displayed_label: String,
    /// Qualification proof bundle.
    pub qualification_packet: MemoryAnalysisQualificationProof,
    /// Guard set.
    pub guards: MemoryAnalysisSurfaceGuardSet,
    /// True when the surface downgrades if required guards are missing.
    pub downgrade_if_missing: bool,
    /// Rationale string.
    pub rationale: String,
}

/// The checked-in memory-analysis qualification packet.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MemoryAnalysisQualificationPacket {
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
    pub surfaces: Vec<MemoryAnalysisSurfaceQualificationRow>,
    /// Memory-analysis view rows.
    pub views: Vec<MemoryAnalysisViewRow>,
    /// Snapshot-pair rows.
    pub snapshot_pairs: Vec<SnapshotPairRow>,
    /// Retained-diff rows.
    pub retained_diffs: Vec<RetainedDiffRow>,
    /// Allocation-diff rows.
    pub allocation_diffs: Vec<AllocationDiffRow>,
    /// Leak-hint rows.
    pub leak_hints: Vec<LeakHintRow>,
    /// Summary.
    pub summary: MemoryAnalysisQualificationSummary,
}

impl MemoryAnalysisQualificationPacket {
    /// Computes the summary from current rows.
    pub fn computed_summary(&self) -> MemoryAnalysisQualificationSummary {
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

        MemoryAnalysisQualificationSummary {
            view_count: self.views.len(),
            snapshot_pair_count: self.snapshot_pairs.len(),
            retained_diff_count: self.retained_diffs.len(),
            allocation_diff_count: self.allocation_diffs.len(),
            leak_hint_count: self.leak_hints.len(),
            stable_count,
            below_stable_count,
            all_below_stable_have_disclosure,
        }
    }

    /// Validates the packet and returns any violations.
    pub fn validate(&self) -> Vec<MemoryAnalysisQualificationViolation> {
        let mut violations = Vec::new();

        if self.schema_version != MEMORY_ANALYSIS_QUALIFICATION_SCHEMA_VERSION {
            violations.push(MemoryAnalysisQualificationViolation::SchemaVersion {
                expected: MEMORY_ANALYSIS_QUALIFICATION_SCHEMA_VERSION,
                actual: self.schema_version,
            });
        }

        if self.record_kind != MEMORY_ANALYSIS_QUALIFICATION_RECORD_KIND {
            violations.push(MemoryAnalysisQualificationViolation::RecordKind {
                expected: MEMORY_ANALYSIS_QUALIFICATION_RECORD_KIND.to_owned(),
                actual: self.record_kind.clone(),
            });
        }

        let mut surface_ids = BTreeSet::new();
        for surface in &self.surfaces {
            if !surface_ids.insert(surface.surface_id.clone()) {
                violations.push(MemoryAnalysisQualificationViolation::DuplicateId {
                    kind: MemoryAnalysisQualificationViolationKind::Surface,
                    id: surface.surface_id.clone(),
                });
            }
            if surface.promoted_build_surface
                && surface.claim_label.is_stable()
                && (!surface.guards.views_visible
                    || !surface.guards.snapshot_pairs_visible
                    || !surface.guards.retained_diffs_visible
                    || !surface.guards.allocation_diffs_visible
                    || !surface.guards.leak_hints_visible
                    || !surface.guards.mapping_quality_visible)
            {
                violations.push(MemoryAnalysisQualificationViolation::IncompleteGuardSet {
                    surface_id: surface.surface_id.clone(),
                });
            }
        }

        let mut view_ids = BTreeSet::new();
        for view in &self.views {
            if !view_ids.insert(view.view_id.clone()) {
                violations.push(MemoryAnalysisQualificationViolation::DuplicateId {
                    kind: MemoryAnalysisQualificationViolationKind::View,
                    id: view.view_id.clone(),
                });
            }
            if view.view_id.trim().is_empty() || view.title.trim().is_empty() {
                violations.push(MemoryAnalysisQualificationViolation::IncompleteView {
                    view_id: view.view_id.clone(),
                });
            }
            if !view.shows_mapping_quality {
                violations.push(
                    MemoryAnalysisQualificationViolation::ViewMissingMappingQuality {
                        view_id: view.view_id.clone(),
                    },
                );
            }
            if !view.shows_degraded_label {
                violations.push(
                    MemoryAnalysisQualificationViolation::ViewMissingDegradedLabel {
                        view_id: view.view_id.clone(),
                    },
                );
            }
        }

        let mut pair_ids = BTreeSet::new();
        for pair in &self.snapshot_pairs {
            if !pair_ids.insert(pair.pair_id.clone()) {
                violations.push(MemoryAnalysisQualificationViolation::DuplicateId {
                    kind: MemoryAnalysisQualificationViolationKind::SnapshotPair,
                    id: pair.pair_id.clone(),
                });
            }
            if pair.pair_id.trim().is_empty()
                || pair.title.trim().is_empty()
                || pair.baseline_snapshot_ref.trim().is_empty()
                || pair.comparison_snapshot_ref.trim().is_empty()
            {
                violations.push(
                    MemoryAnalysisQualificationViolation::IncompleteSnapshotPair {
                        pair_id: pair.pair_id.clone(),
                    },
                );
            }
            if !pair.shows_comparison_basis {
                violations.push(
                    MemoryAnalysisQualificationViolation::SnapshotPairMissingComparisonBasis {
                        pair_id: pair.pair_id.clone(),
                    },
                );
            }
            if !pair.shows_degraded_label {
                violations.push(
                    MemoryAnalysisQualificationViolation::SnapshotPairMissingDegradedLabel {
                        pair_id: pair.pair_id.clone(),
                    },
                );
            }
        }

        let mut diff_ids = BTreeSet::new();
        for diff in &self.retained_diffs {
            if !diff_ids.insert(diff.diff_id.clone()) {
                violations.push(MemoryAnalysisQualificationViolation::DuplicateId {
                    kind: MemoryAnalysisQualificationViolationKind::RetainedDiff,
                    id: diff.diff_id.clone(),
                });
            }
            if diff.diff_id.trim().is_empty()
                || diff.title.trim().is_empty()
                || diff.pair_ref.trim().is_empty()
                || diff.type_path_or_class.trim().is_empty()
            {
                violations.push(
                    MemoryAnalysisQualificationViolation::IncompleteRetainedDiff {
                        diff_id: diff.diff_id.clone(),
                    },
                );
            }
            if !diff.shows_mapping_quality {
                violations.push(
                    MemoryAnalysisQualificationViolation::RetainedDiffMissingMappingQuality {
                        diff_id: diff.diff_id.clone(),
                    },
                );
            }
        }

        let mut alloc_diff_ids = BTreeSet::new();
        for diff in &self.allocation_diffs {
            if !alloc_diff_ids.insert(diff.diff_id.clone()) {
                violations.push(MemoryAnalysisQualificationViolation::DuplicateId {
                    kind: MemoryAnalysisQualificationViolationKind::AllocationDiff,
                    id: diff.diff_id.clone(),
                });
            }
            if diff.diff_id.trim().is_empty()
                || diff.title.trim().is_empty()
                || diff.pair_ref.trim().is_empty()
                || diff.type_path_or_class.trim().is_empty()
            {
                violations.push(
                    MemoryAnalysisQualificationViolation::IncompleteAllocationDiff {
                        diff_id: diff.diff_id.clone(),
                    },
                );
            }
            if !diff.shows_mapping_quality {
                violations.push(
                    MemoryAnalysisQualificationViolation::AllocationDiffMissingMappingQuality {
                        diff_id: diff.diff_id.clone(),
                    },
                );
            }
        }

        let mut hint_ids = BTreeSet::new();
        for hint in &self.leak_hints {
            if !hint_ids.insert(hint.hint_id.clone()) {
                violations.push(MemoryAnalysisQualificationViolation::DuplicateId {
                    kind: MemoryAnalysisQualificationViolationKind::LeakHint,
                    id: hint.hint_id.clone(),
                });
            }
            if hint.hint_id.trim().is_empty()
                || hint.title.trim().is_empty()
                || hint.pair_ref.trim().is_empty()
                || hint.type_path_or_class.trim().is_empty()
                || hint.leak_rationale.trim().is_empty()
            {
                violations.push(MemoryAnalysisQualificationViolation::IncompleteLeakHint {
                    hint_id: hint.hint_id.clone(),
                });
            }
            if !hint.shows_confidence {
                violations.push(
                    MemoryAnalysisQualificationViolation::LeakHintMissingConfidence {
                        hint_id: hint.hint_id.clone(),
                    },
                );
            }
        }

        // Cross-reference: every view snapshot_pair_ref must point to a known snapshot pair.
        let pair_id_set: BTreeSet<String> = self
            .snapshot_pairs
            .iter()
            .map(|p| p.pair_id.clone())
            .collect();
        for view in &self.views {
            if let Some(ref pair_ref) = view.snapshot_pair_ref {
                if !pair_id_set.contains(pair_ref) {
                    violations.push(
                        MemoryAnalysisQualificationViolation::ViewSnapshotPairRefUnknown {
                            view_id: view.view_id.clone(),
                            pair_ref: pair_ref.clone(),
                        },
                    );
                }
            }
        }

        // Cross-reference: every retained diff must point to a known snapshot pair.
        for diff in &self.retained_diffs {
            if !pair_id_set.contains(&diff.pair_ref) {
                violations.push(
                    MemoryAnalysisQualificationViolation::RetainedDiffPairRefUnknown {
                        diff_id: diff.diff_id.clone(),
                        pair_ref: diff.pair_ref.clone(),
                    },
                );
            }
        }

        // Cross-reference: every allocation diff must point to a known snapshot pair.
        for diff in &self.allocation_diffs {
            if !pair_id_set.contains(&diff.pair_ref) {
                violations.push(
                    MemoryAnalysisQualificationViolation::AllocationDiffPairRefUnknown {
                        diff_id: diff.diff_id.clone(),
                        pair_ref: diff.pair_ref.clone(),
                    },
                );
            }
        }

        // Cross-reference: every leak hint must point to a known snapshot pair.
        for hint in &self.leak_hints {
            if !pair_id_set.contains(&hint.pair_ref) {
                violations.push(
                    MemoryAnalysisQualificationViolation::LeakHintPairRefUnknown {
                        hint_id: hint.hint_id.clone(),
                        pair_ref: hint.pair_ref.clone(),
                    },
                );
            }
        }

        if self.summary != self.computed_summary() {
            violations.push(MemoryAnalysisQualificationViolation::SummaryMismatch);
        }

        violations
    }
}

/// Loads the checked-in memory-analysis qualification packet.
///
/// # Errors
///
/// Returns the underlying JSON parse error when the embedded artifact no longer
/// matches the typed model.
pub fn current_memory_analysis_qualification(
) -> Result<MemoryAnalysisQualificationPacket, serde_json::Error> {
    serde_json::from_str(MEMORY_ANALYSIS_QUALIFICATION_PACKET_JSON)
}

/// Identity family used when reporting duplicate ids.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MemoryAnalysisQualificationViolationKind {
    /// Surface rows.
    Surface,
    /// Memory-analysis view rows.
    View,
    /// Snapshot-pair rows.
    SnapshotPair,
    /// Retained-diff rows.
    RetainedDiff,
    /// Allocation-diff rows.
    AllocationDiff,
    /// Leak-hint rows.
    LeakHint,
}

impl fmt::Display for MemoryAnalysisQualificationViolationKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Surface => write!(f, "surface"),
            Self::View => write!(f, "view"),
            Self::SnapshotPair => write!(f, "snapshot_pair"),
            Self::RetainedDiff => write!(f, "retained_diff"),
            Self::AllocationDiff => write!(f, "allocation_diff"),
            Self::LeakHint => write!(f, "leak_hint"),
        }
    }
}

/// Validation failure for memory-analysis qualification packets.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MemoryAnalysisQualificationViolation {
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
        kind: MemoryAnalysisQualificationViolationKind,
        /// Duplicate id.
        id: String,
    },
    /// A surface with a stable claim has an incomplete guard set.
    IncompleteGuardSet {
        /// Surface id.
        surface_id: String,
    },
    /// A memory-analysis view row is incomplete.
    IncompleteView {
        /// View id.
        view_id: String,
    },
    /// A memory-analysis view row must show its mapping quality.
    ViewMissingMappingQuality {
        /// View id.
        view_id: String,
    },
    /// A memory-analysis view row must show a degraded-state label.
    ViewMissingDegradedLabel {
        /// View id.
        view_id: String,
    },
    /// A snapshot-pair row is incomplete.
    IncompleteSnapshotPair {
        /// Pair id.
        pair_id: String,
    },
    /// A snapshot-pair row must show its comparison basis.
    SnapshotPairMissingComparisonBasis {
        /// Pair id.
        pair_id: String,
    },
    /// A snapshot-pair row must show a degraded-state label.
    SnapshotPairMissingDegradedLabel {
        /// Pair id.
        pair_id: String,
    },
    /// A retained-diff row is incomplete.
    IncompleteRetainedDiff {
        /// Diff id.
        diff_id: String,
    },
    /// A retained-diff row must show its mapping quality.
    RetainedDiffMissingMappingQuality {
        /// Diff id.
        diff_id: String,
    },
    /// An allocation-diff row is incomplete.
    IncompleteAllocationDiff {
        /// Diff id.
        diff_id: String,
    },
    /// An allocation-diff row must show its mapping quality.
    AllocationDiffMissingMappingQuality {
        /// Diff id.
        diff_id: String,
    },
    /// A leak-hint row is incomplete.
    IncompleteLeakHint {
        /// Hint id.
        hint_id: String,
    },
    /// A leak-hint row must show its confidence level.
    LeakHintMissingConfidence {
        /// Hint id.
        hint_id: String,
    },
    /// A view references an unknown snapshot pair.
    ViewSnapshotPairRefUnknown {
        /// View id.
        view_id: String,
        /// Unknown pair ref.
        pair_ref: String,
    },
    /// A retained diff references an unknown snapshot pair.
    RetainedDiffPairRefUnknown {
        /// Diff id.
        diff_id: String,
        /// Unknown pair ref.
        pair_ref: String,
    },
    /// An allocation diff references an unknown snapshot pair.
    AllocationDiffPairRefUnknown {
        /// Diff id.
        diff_id: String,
        /// Unknown pair ref.
        pair_ref: String,
    },
    /// A leak hint references an unknown snapshot pair.
    LeakHintPairRefUnknown {
        /// Hint id.
        hint_id: String,
        /// Unknown pair ref.
        pair_ref: String,
    },
    /// Computed summary does not match the stored summary.
    SummaryMismatch,
}

impl fmt::Display for MemoryAnalysisQualificationViolation {
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
            Self::IncompleteView { view_id } => {
                write!(f, "incomplete memory-analysis view row: {view_id}")
            }
            Self::ViewMissingMappingQuality { view_id } => {
                write!(
                    f,
                    "memory-analysis view {view_id} must show its mapping quality"
                )
            }
            Self::ViewMissingDegradedLabel { view_id } => {
                write!(
                    f,
                    "memory-analysis view {view_id} must show a degraded-state label"
                )
            }
            Self::IncompleteSnapshotPair { pair_id } => {
                write!(f, "incomplete snapshot-pair row: {pair_id}")
            }
            Self::SnapshotPairMissingComparisonBasis { pair_id } => {
                write!(f, "snapshot pair {pair_id} must show its comparison basis")
            }
            Self::SnapshotPairMissingDegradedLabel { pair_id } => {
                write!(
                    f,
                    "snapshot pair {pair_id} must show a degraded-state label"
                )
            }
            Self::IncompleteRetainedDiff { diff_id } => {
                write!(f, "incomplete retained-diff row: {diff_id}")
            }
            Self::RetainedDiffMissingMappingQuality { diff_id } => {
                write!(f, "retained diff {diff_id} must show its mapping quality")
            }
            Self::IncompleteAllocationDiff { diff_id } => {
                write!(f, "incomplete allocation-diff row: {diff_id}")
            }
            Self::AllocationDiffMissingMappingQuality { diff_id } => {
                write!(f, "allocation diff {diff_id} must show its mapping quality")
            }
            Self::IncompleteLeakHint { hint_id } => {
                write!(f, "incomplete leak-hint row: {hint_id}")
            }
            Self::LeakHintMissingConfidence { hint_id } => {
                write!(f, "leak hint {hint_id} must show its confidence level")
            }
            Self::ViewSnapshotPairRefUnknown { view_id, pair_ref } => {
                write!(
                    f,
                    "view {view_id} references unknown snapshot pair {pair_ref}"
                )
            }
            Self::RetainedDiffPairRefUnknown { diff_id, pair_ref } => {
                write!(
                    f,
                    "retained diff {diff_id} references unknown snapshot pair {pair_ref}"
                )
            }
            Self::AllocationDiffPairRefUnknown { diff_id, pair_ref } => {
                write!(
                    f,
                    "allocation diff {diff_id} references unknown snapshot pair {pair_ref}"
                )
            }
            Self::LeakHintPairRefUnknown { hint_id, pair_ref } => {
                write!(
                    f,
                    "leak hint {hint_id} references unknown snapshot pair {pair_ref}"
                )
            }
            Self::SummaryMismatch => {
                write!(f, "computed summary does not match stored summary")
            }
        }
    }
}

impl Error for MemoryAnalysisQualificationViolation {}
