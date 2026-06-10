//! Profile launcher, attach sheets, capture-mode descriptors, storage-location truth,
//! hotspot workspace, flamegraph, call tree, mapping-quality labels, source navigation,
//! shared trace viewer with synchronized event lanes, bookmarks, and textual fallback,
//! memory-analysis views, snapshot pairs, retained or allocation diffs, leak-hint
//! confidence, regression baseline store, baseline selection UX, comparable-environment
//! guards, profile-compare cards, threshold or waiver state, confounder disclosure,
//! coverage, profile, test, debug, and notebook evidence handoff bars with artifact
//! lineage, and justified replay backend with recording-mode banner, expiry, and cost
//! posture.
//!
//! This crate owns the typed records that keep profile launch and attach surfaces,
//! capture-mode descriptors, storage-location truth, hotspot surfaces, trace viewer
//! surfaces, memory-analysis surfaces, regression-baseline surfaces, and evidence-handoff
//! surfaces attributable and inspectable.
//!
//! It exposes the canonical
//! [`materialize_profile_launcher_and_attach_sheets_capture_mode_descriptors_and_storage_location_truth`]
//! module that pins the launcher, attach-sheet, capture-mode, and storage-location
//! contracts every profiler UI, support export, and release reviewer reads.
//!
//! This crate also exposes the
//! [`implement_the_first_justified_replay_backend_with_recording_mode_banner_expiry_and_cost_posture`]
//! module that pins the recording-mode banner, replay expiry, and cost posture contracts
//! every replay surface reads.
//!
//! This crate also exposes the
//! [`implement_profile_compare_cards_threshold_or_waiver_state_and_confounder_disclosure`]
//! module that pins the profile-compare card, threshold-state, waiver-state, and
//! confounder-disclosure contracts every comparison surface reads.
//!
//! The reviewer-facing contract is at
//! [`/docs/performance/m5/materialize-profile-launcher-and-attach-sheets-capture-mode-descriptors-and-storage-location-truth.md`](../../../docs/performance/m5/materialize-profile-launcher-and-attach-sheets-capture-mode-descriptors-and-storage-location-truth.md).
//! The cross-tool boundary schema is at
//! [`/schemas/perf/materialize-profile-launcher-and-attach-sheets-capture-mode-descriptors-and-storage-location-truth.schema.json`](../../../schemas/perf/materialize-profile-launcher-and-attach-sheets-capture-mode-descriptors-and-storage-location-truth.schema.json).
//! The checked-in stable packet is at
//! [`/artifacts/perf/m5/materialize-profile-launcher-and-attach-sheets-capture-mode-descriptors-and-storage-location-truth.json`](../../../artifacts/perf/m5/materialize-profile-launcher-and-attach-sheets-capture-mode-descriptors-and-storage-location-truth.json).
//!
//! This crate also exposes the
//! [`build_the_regression_baseline_store_baseline_selection_ux_and_comparable_environment_guards`]
//! module that pins the regression baseline store, baseline selection UX, comparable-environment
//! guards, and environment fingerprint contracts every regression and comparison surface reads.
//!
//! The reviewer-facing contract for regression baselines is at
//! [`/docs/performance/m5/build-the-regression-baseline-store-baseline-selection-ux-and-comparable-environment-guards.md`](../../../docs/performance/m5/build-the-regression-baseline-store-baseline-selection-ux-and-comparable-environment-guards.md).
//! The cross-tool boundary schema is at
//! [`/schemas/perf/build-the-regression-baseline-store-baseline-selection-ux-and-comparable-environment-guards.schema.json`](../../../schemas/perf/build-the-regression-baseline-store-baseline-selection-ux-and-comparable-environment-guards.schema.json).
//! The checked-in stable packet is at
//! [`/artifacts/perf/m5/build-the-regression-baseline-store-baseline-selection-ux-and-comparable-environment-guards.json`](../../../artifacts/perf/m5/build-the-regression-baseline-store-baseline-selection-ux-and-comparable-environment-guards.json).
//!
//! The reviewer-facing contract for profile-compare cards is at
//! [`/docs/performance/m5/implement-profile-compare-cards-threshold-or-waiver-state-and-confounder-disclosure.md`](../../../docs/performance/m5/implement-profile-compare-cards-threshold-or-waiver-state-and-confounder-disclosure.md).
//! The cross-tool boundary schema is at
//! [`/schemas/perf/implement-profile-compare-cards-threshold-or-waiver-state-and-confounder-disclosure.schema.json`](../../../schemas/perf/implement-profile-compare-cards-threshold-or-waiver-state-and-confounder-disclosure.schema.json).
//! The checked-in stable packet is at
//! [`/artifacts/perf/m5/implement-profile-compare-cards-threshold-or-waiver-state-and-confounder-disclosure.json`](../../../artifacts/perf/m5/implement-profile-compare-cards-threshold-or-waiver-state-and-confounder-disclosure.json).
//!
//! This crate also exposes the
//! [`ship_the_hotspot_workspace_with_flamegraph_call_tree_mapping_quality_labels_and_source_navigation`]
//! module that pins the hotspot workspace, flamegraph, call tree, mapping-quality,
//! and source-navigation contracts every profiler drilldown surface reads.
//!
//! The reviewer-facing contract for the hotspot workspace is at
//! [`/docs/performance/m5/ship-the-hotspot-workspace-with-flamegraph-call-tree-mapping-quality-labels-and-source-navigation.md`](../../../docs/performance/m5/ship-the-hotspot-workspace-with-flamegraph-call-tree-mapping-quality-labels-and-source-navigation.md).
//! The cross-tool boundary schema is at
//! [`/schemas/perf/ship-the-hotspot-workspace-with-flamegraph-call-tree-mapping-quality-labels-and-source-navigation.schema.json`](../../../schemas/perf/ship-the-hotspot-workspace-with-flamegraph-call-tree-mapping-quality-labels-and-source-navigation.schema.json).
//! The checked-in stable packet is at
//! [`/artifacts/perf/m5/ship-the-hotspot-workspace-with-flamegraph-call-tree-mapping-quality-labels-and-source-navigation.json`](../../../artifacts/perf/m5/ship-the-hotspot-workspace-with-flamegraph-call-tree-mapping-quality-labels-and-source-navigation.json).
//!
//! This crate also exposes the
//! [`implement_the_shared_trace_viewer_with_synchronized_event_lanes_bookmarks_and_textual_fallback`]
//! module that pins the shared trace viewer, synchronized event lanes, bookmarks,
//! and textual-fallback contracts every trace and replay surface reads.
//!
//! The reviewer-facing contract for the trace viewer is at
//! [`/docs/performance/m5/implement-the-shared-trace-viewer-with-synchronized-event-lanes-bookmarks-and-textual-fallback.md`](../../../docs/performance/m5/implement-the-shared-trace-viewer-with-synchronized-event-lanes-bookmarks-and-textual-fallback.md).
//! The cross-tool boundary schema is at
//! [`/schemas/perf/implement-the-shared-trace-viewer-with-synchronized-event-lanes-bookmarks-and-textual-fallback.schema.json`](../../../schemas/perf/implement-the-shared-trace-viewer-with-synchronized-event-lanes-bookmarks-and-textual-fallback.schema.json).
//! The checked-in stable packet is at
//! [`/artifacts/perf/m5/implement-the-shared-trace-viewer-with-synchronized-event-lanes-bookmarks-and-textual-fallback.json`](../../../artifacts/perf/m5/implement-the-shared-trace-viewer-with-synchronized-event-lanes-bookmarks-and-textual-fallback.json).
//!
//! This crate also exposes the
//! [`add_memory_analysis_views_snapshot_pairs_retained_or_allocation_diffs_and_leak_hint_confidence`]
//! module that pins the memory-analysis views, snapshot pairs, retained diffs,
//! allocation diffs, and leak-hint confidence contracts every memory-analysis surface
//! reads.
//!
//! The reviewer-facing contract for memory analysis is at
//! [`/docs/performance/m5/add-memory-analysis-views-snapshot-pairs-retained-or-allocation-diffs-and-leak-hint-confidence.md`](../../../docs/performance/m5/add-memory-analysis-views-snapshot-pairs-retained-or-allocation-diffs-and-leak-hint-confidence.md).
//! The cross-tool boundary schema is at
//! [`/schemas/perf/add-memory-analysis-views-snapshot-pairs-retained-or-allocation-diffs-and-leak-hint-confidence.schema.json`](../../../schemas/perf/add-memory-analysis-views-snapshot-pairs-retained-or-allocation-diffs-and-leak-hint-confidence.schema.json).
//! The checked-in stable packet is at
//! [`/artifacts/perf/m5/add-memory-analysis-views-snapshot-pairs-retained-or-allocation-diffs-and-leak-hint-confidence.json`](../../../artifacts/perf/m5/add-memory-analysis-views-snapshot-pairs-retained-or-allocation-diffs-and-leak-hint-confidence.json).
//!
//! This crate also exposes the
//! [`ship_coverage_profile_test_debug_and_notebook_evidence_handoff_bars_with_artifact_lineage`]
//! module that pins the coverage, profile, test, debug, and notebook evidence handoff bar,
//! artifact lineage, capture source, and save/share scope contracts every evidence surface
//! reads.
//!
//! The reviewer-facing contract for evidence handoff bars is at
//! [`/docs/performance/m5/ship-coverage-profile-test-debug-and-notebook-evidence-handoff-bars-with-artifact-lineage.md`](../../../docs/performance/m5/ship-coverage-profile-test-debug-and-notebook-evidence-handoff-bars-with-artifact-lineage.md).
//! The cross-tool boundary schema is at
//! [`/schemas/perf/ship-coverage-profile-test-debug-and-notebook-evidence-handoff-bars-with-artifact-lineage.schema.json`](../../../schemas/perf/ship-coverage-profile-test-debug-and-notebook-evidence-handoff-bars-with-artifact-lineage.schema.json).
//! The checked-in stable packet is at
//! [`/artifacts/perf/m5/ship-coverage-profile-test-debug-and-notebook-evidence-handoff-bars-with-artifact-lineage.json`](../../../artifacts/perf/m5/ship-coverage-profile-test-debug-and-notebook-evidence-handoff-bars-with-artifact-lineage.json).
//!
//! The reviewer-facing contract for the justified replay backend is at
//! [`/docs/performance/m5/implement-the-first-justified-replay-backend-with-recording-mode-banner-expiry-and-cost-posture.md`](../../../docs/performance/m5/implement-the-first-justified-replay-backend-with-recording-mode-banner-expiry-and-cost-posture.md).
//! The cross-tool boundary schema is at
//! [`/schemas/perf/implement-the-first-justified-replay-backend-with-recording-mode-banner-expiry-and-cost-posture.schema.json`](../../../schemas/perf/implement-the-first-justified-replay-backend-with-recording-mode-banner-expiry-and-cost-posture.schema.json).
//! The checked-in stable packet is at
//! [`/artifacts/perf/m5/implement-the-first-justified-replay-backend-with-recording-mode-banner-expiry-and-cost-posture.json`](../../../artifacts/perf/m5/implement-the-first-justified-replay-backend-with-recording-mode-banner-expiry-and-cost-posture.json).

#![doc(html_root_url = "https://docs.rs/aureline-profiler/0.0.0")]

pub mod add_memory_analysis_views_snapshot_pairs_retained_or_allocation_diffs_and_leak_hint_confidence;
pub mod build_the_regression_baseline_store_baseline_selection_ux_and_comparable_environment_guards;
pub mod implement_profile_compare_cards_threshold_or_waiver_state_and_confounder_disclosure;
pub mod implement_the_first_justified_replay_backend_with_recording_mode_banner_expiry_and_cost_posture;
pub mod implement_the_shared_trace_viewer_with_synchronized_event_lanes_bookmarks_and_textual_fallback;
pub mod materialize_profile_launcher_and_attach_sheets_capture_mode_descriptors_and_storage_location_truth;
pub mod ship_coverage_profile_test_debug_and_notebook_evidence_handoff_bars_with_artifact_lineage;
pub mod ship_the_hotspot_workspace_with_flamegraph_call_tree_mapping_quality_labels_and_source_navigation;

pub use build_the_regression_baseline_store_baseline_selection_ux_and_comparable_environment_guards::{
    current_regression_baseline_qualification, BaselineFreshness, BaselineSelectionKind,
    BaselineSelectionUxRow, BaselineStoreRow, ComparableEnvironmentGuardRow,
    EnvironmentFingerprintRow, EnvironmentMatchState, RegressionBaselineQualificationLabel,
    RegressionBaselineQualificationPacket, RegressionBaselineQualificationProof,
    RegressionBaselineQualificationSummary, RegressionBaselineQualificationViolation,
    RegressionBaselineQualificationViolationKind, RegressionBaselineSurfaceGuardSet,
    RegressionBaselineSurfaceKind, RegressionBaselineSurfaceQualificationRow,
    REGRESSION_BASELINE_QUALIFICATION_PACKET_JSON, REGRESSION_BASELINE_QUALIFICATION_PACKET_PATH,
    REGRESSION_BASELINE_QUALIFICATION_RECORD_KIND, REGRESSION_BASELINE_QUALIFICATION_SCHEMA_VERSION,
};

pub use add_memory_analysis_views_snapshot_pairs_retained_or_allocation_diffs_and_leak_hint_confidence::{
    current_memory_analysis_qualification, AllocationDiffRow, ComparisonBasis,
    LeakHintConfidence, LeakHintRow, MemoryAnalysisQualificationLabel,
    MemoryAnalysisQualificationPacket, MemoryAnalysisQualificationProof,
    MemoryAnalysisQualificationSummary, MemoryAnalysisQualificationViolation,
    MemoryAnalysisQualificationViolationKind, MemoryAnalysisSurfaceGuardSet,
    MemoryAnalysisSurfaceKind, MemoryAnalysisSurfaceQualificationRow,
    MemoryAnalysisViewKind, MemoryAnalysisViewRow, MemoryMappingQualityLabel,
    RetainedDiffRow, SnapshotKind, SnapshotPairRow,
    MEMORY_ANALYSIS_QUALIFICATION_PACKET_JSON, MEMORY_ANALYSIS_QUALIFICATION_PACKET_PATH,
    MEMORY_ANALYSIS_QUALIFICATION_RECORD_KIND, MEMORY_ANALYSIS_QUALIFICATION_SCHEMA_VERSION,
};

pub use implement_the_shared_trace_viewer_with_synchronized_event_lanes_bookmarks_and_textual_fallback::{
    current_trace_viewer_qualification, BookmarkRow, EventLaneKind, EventLaneRow,
    TextualFallbackContentKind, TextualFallbackRow, TraceMappingQualityLabel,
    TraceViewerQualificationLabel, TraceViewerQualificationPacket,
    TraceViewerQualificationProof, TraceViewerQualificationSummary,
    TraceViewerQualificationViolation, TraceViewerQualificationViolationKind,
    TraceViewerSurfaceGuardSet, TraceViewerSurfaceKind,
    TraceViewerSurfaceQualificationRow,
    TRACE_VIEWER_QUALIFICATION_PACKET_JSON, TRACE_VIEWER_QUALIFICATION_PACKET_PATH,
    TRACE_VIEWER_QUALIFICATION_RECORD_KIND, TRACE_VIEWER_QUALIFICATION_SCHEMA_VERSION,
};

pub use materialize_profile_launcher_and_attach_sheets_capture_mode_descriptors_and_storage_location_truth::{
    current_profile_launcher_qualification, AttachSheetKind, AttachSheetRow,
    CaptureModeClass, CaptureModeDescriptorRow, ProfileLauncherQualificationLabel,
    ProfileLauncherQualificationPacket, ProfileLauncherQualificationProof,
    ProfileLauncherQualificationSummary, ProfileLauncherQualificationViolation,
    ProfileLauncherQualificationViolationKind, ProfileLauncherRow, ProfileLauncherSurfaceGuardSet,
    ProfileLauncherSurfaceKind, ProfileLauncherSurfaceQualificationRow, StorageLocationClass,
    StorageLocationTruthLabel, StorageLocationTruthRow, PROFILE_LAUNCHER_QUALIFICATION_PACKET_JSON,
    PROFILE_LAUNCHER_QUALIFICATION_PACKET_PATH, PROFILE_LAUNCHER_QUALIFICATION_RECORD_KIND,
    PROFILE_LAUNCHER_QUALIFICATION_SCHEMA_VERSION,
};

pub use implement_profile_compare_cards_threshold_or_waiver_state_and_confounder_disclosure::{
    current_profile_compare_qualification, ComparisonKind, ConfounderDisclosureRow, ConfounderKind,
    ConfounderSeverity, ProfileCompareCardRow, ProfileCompareQualificationLabel,
    ProfileCompareQualificationPacket, ProfileCompareQualificationProof,
    ProfileCompareQualificationSummary, ProfileCompareQualificationViolation,
    ProfileCompareQualificationViolationKind, ProfileCompareSurfaceGuardSet,
    ProfileCompareSurfaceKind, ProfileCompareSurfaceQualificationRow, ThresholdState,
    ThresholdStateRow, WaiverStateRow, WaiverStatus, PROFILE_COMPARE_QUALIFICATION_PACKET_JSON,
    PROFILE_COMPARE_QUALIFICATION_PACKET_PATH, PROFILE_COMPARE_QUALIFICATION_RECORD_KIND,
    PROFILE_COMPARE_QUALIFICATION_SCHEMA_VERSION,
};

pub use ship_coverage_profile_test_debug_and_notebook_evidence_handoff_bars_with_artifact_lineage::{
    current_evidence_handoff_qualification, ArtifactLineageRow, CaptureSourceClass,
    CaptureSourceRow, EvidenceHandoffBarRow, EvidenceHandoffQualificationLabel,
    EvidenceHandoffQualificationPacket, EvidenceHandoffQualificationProof,
    EvidenceHandoffQualificationSummary, EvidenceHandoffQualificationViolation,
    EvidenceHandoffQualificationViolationKind, EvidenceHandoffSurfaceGuardSet,
    EvidenceHandoffSurfaceKind, EvidenceHandoffSurfaceQualificationRow, EvidenceKind,
    LineageState, SaveShareScopeKind, SaveShareScopeRow,
    EVIDENCE_HANDOFF_QUALIFICATION_PACKET_JSON, EVIDENCE_HANDOFF_QUALIFICATION_PACKET_PATH,
    EVIDENCE_HANDOFF_QUALIFICATION_RECORD_KIND, EVIDENCE_HANDOFF_QUALIFICATION_SCHEMA_VERSION,
};

pub use implement_the_first_justified_replay_backend_with_recording_mode_banner_expiry_and_cost_posture::{
    current_replay_qualification, CostPostureClass, RecordingModeBannerRow, RecordingModeState,
    ReplayCostPostureRow, ReplayExpiryRow, ReplayQualificationLabel, ReplayQualificationPacket,
    ReplayQualificationProof, ReplayQualificationSummary, ReplayQualificationViolation,
    ReplayQualificationViolationKind, ReplaySurfaceGuardSet, ReplaySurfaceKind,
    ReplaySurfaceQualificationRow, ExpiryStatus,
    REPLAY_QUALIFICATION_PACKET_JSON, REPLAY_QUALIFICATION_PACKET_PATH,
    REPLAY_QUALIFICATION_RECORD_KIND, REPLAY_QUALIFICATION_SCHEMA_VERSION,
};

pub use ship_the_hotspot_workspace_with_flamegraph_call_tree_mapping_quality_labels_and_source_navigation::{
    current_hotspot_workspace_qualification, CallTreeRow, FlamegraphRow,
    HotspotWorkspaceQualificationLabel, HotspotWorkspaceQualificationPacket,
    HotspotWorkspaceQualificationProof, HotspotWorkspaceQualificationSummary,
    HotspotWorkspaceQualificationViolation, HotspotWorkspaceQualificationViolationKind,
    HotspotWorkspaceSurfaceGuardSet, HotspotWorkspaceSurfaceKind,
    HotspotWorkspaceSurfaceQualificationRow, MappingQualityBadgeRow, MappingQualityLabel,
    ProfilePosture, SessionStripRow, SourceNavigationAction, SourceNavigationRow,
    HOTSPOT_WORKSPACE_QUALIFICATION_PACKET_JSON, HOTSPOT_WORKSPACE_QUALIFICATION_PACKET_PATH,
    HOTSPOT_WORKSPACE_QUALIFICATION_RECORD_KIND, HOTSPOT_WORKSPACE_QUALIFICATION_SCHEMA_VERSION,
};
