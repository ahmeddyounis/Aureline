//! Profile launcher, attach sheets, capture-mode descriptors, and storage-location truth.
//!
//! This crate owns the typed records that keep profile launch and attach surfaces,
//! capture-mode descriptors, and storage-location truth attributable and inspectable.
//! It exposes one canonical
//! [`materialize_profile_launcher_and_attach_sheets_capture_mode_descriptors_and_storage_location_truth`]
//! module that pins the launcher, attach-sheet, capture-mode, and storage-location
//! contracts every profiler UI, support export, and release reviewer reads.
//!
//! The reviewer-facing contract is at
//! [`/docs/performance/m5/materialize-profile-launcher-and-attach-sheets-capture-mode-descriptors-and-storage-location-truth.md`](../../../docs/performance/m5/materialize-profile-launcher-and-attach-sheets-capture-mode-descriptors-and-storage-location-truth.md).
//! The cross-tool boundary schema is at
//! [`/schemas/perf/materialize-profile-launcher-and-attach-sheets-capture-mode-descriptors-and-storage-location-truth.schema.json`](../../../schemas/perf/materialize-profile-launcher-and-attach-sheets-capture-mode-descriptors-and-storage-location-truth.schema.json).
//! The checked-in stable packet is at
//! [`/artifacts/perf/m5/materialize-profile-launcher-and-attach-sheets-capture-mode-descriptors-and-storage-location-truth.json`](../../../artifacts/perf/m5/materialize-profile-launcher-and-attach-sheets-capture-mode-descriptors-and-storage-location-truth.json).
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

#![doc(html_root_url = "https://docs.rs/aureline-profiler/0.0.0")]

pub mod materialize_profile_launcher_and_attach_sheets_capture_mode_descriptors_and_storage_location_truth;
pub mod ship_the_hotspot_workspace_with_flamegraph_call_tree_mapping_quality_labels_and_source_navigation;

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
