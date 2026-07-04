//! Infrastructure target-context, source-intelligence, and control-plane packets.
//!
//! This crate owns the qualification model that keeps infrastructure-facing
//! surfaces honest about target identity, truth layers, relationship edges,
//! action safety, and vendor-console handoff posture. It does not implement
//! live Kubernetes, cloud, or console connectors; it validates the packet
//! evidence those surfaces must emit before any stable ops claim can be
//! promoted.

#![doc(html_root_url = "https://docs.rs/aureline-infra/0.0.0")]

pub mod add_manifest_build_component_accessibility_keyboard_screen_reader_cli_export_parity_and_auto_narrowing;
pub mod add_shared_container_devcontainer_request_incident_support_and_ai_manifest_build_component_consumers;
pub mod cluster_context_and_live_resource;
pub mod freeze_the_m5_manifest_editor_schema_validator_resource_link_build_adapter_target_graph_and_fallback_confidence_component_matrix;
pub mod implement_the_m5_adapter_drift_banner_launcher_state_and_no_higher_confidence_overwrite_primitive;
pub mod implement_the_m5_adapter_source_badge_target_graph_capability_matrix_raw_event_and_fallback_confidence_primitive;
pub mod implement_the_m5_manifest_editor_schema_validator_and_target_context_apply_review_primitive;
pub mod implement_the_m5_resource_link_compare_card_explorer_and_drift_banner_primitive;
pub mod infrastructure_surface_qualification;
pub mod plan_and_validation_viewers;
pub mod provider_overlay_and_vendor_console_handoff_continuity;
pub mod relation_graph_incident_support_parity;
pub mod source_intelligence_and_resource_relationships;
pub mod target_context_and_control_plane_boundary;

pub use add_manifest_build_component_accessibility_keyboard_screen_reader_cli_export_parity_and_auto_narrowing::{
    current_m5_manifest_build_a11y_fallback_export, seeded_m5_manifest_build_a11y_fallback_packet,
    AccessibilityAutoNarrow, ClaimAffordanceState, ClaimTruthSignals,
    ComponentAccessibilityArtifactError, ComponentAccessibilityPacket,
    ComponentAccessibilityPacketInput, ComponentAccessibilityRow, ComponentAccessibilityStatus,
    ComponentAccessibilitySummary, ComponentAccessibilityViolation,
    CopyExportParity as A11yCopyExportParity, ExportSummaryState,
    M5ManifestBuildConsumerSurface as M5ManifestBuildA11yConsumerSurface,
    M5ManifestBuildFallbackModality, M5ManifestBuildRenderingSurface, NarrowedClaimTier,
    NarrowingDisclosureState, NonVisualReachState, RenderingNarrowingDisclosure,
    MANIFEST_BUILD_A11Y_FALLBACK_ARTIFACT_REF, MANIFEST_BUILD_A11Y_FALLBACK_COMPONENT_MATRIX_REF,
    MANIFEST_BUILD_A11Y_FALLBACK_CSV_REF, MANIFEST_BUILD_A11Y_FALLBACK_DOC_REF,
    MANIFEST_BUILD_A11Y_FALLBACK_FIXTURE_DIR, MANIFEST_BUILD_A11Y_FALLBACK_RECORD_KIND,
    MANIFEST_BUILD_A11Y_FALLBACK_REPORT_REF, MANIFEST_BUILD_A11Y_FALLBACK_ROW_RECORD_KIND,
    MANIFEST_BUILD_A11Y_FALLBACK_SCHEMA_REF, MANIFEST_BUILD_A11Y_FALLBACK_SCHEMA_VERSION,
};
pub use add_shared_container_devcontainer_request_incident_support_and_ai_manifest_build_component_consumers::{
    canonical_packet_ref_for, canonical_schema_ref_for,
    current_stable_m5_manifest_build_consumer_export, seeded_m5_manifest_build_consumer_packet,
    AuthorityMode as ConsumerAuthorityMode, ConsumerGroup, CopyExportParity,
    HandoffTarget as ConsumerHandoffTarget, LabelParityState, M5ManifestBuildConsumerSurface,
    ManifestBuildConsumerArtifactError, ManifestBuildConsumerPacket, ManifestBuildConsumerRow,
    ManifestBuildConsumerSummary, ManifestBuildConsumerViolation, ReducedCapabilityBanner,
    MANIFEST_BUILD_CONSUMER_ARTIFACT_JSON, MANIFEST_BUILD_CONSUMER_ARTIFACT_REF,
    MANIFEST_BUILD_CONSUMER_CSV_REF, MANIFEST_BUILD_CONSUMER_DOC_REF,
    MANIFEST_BUILD_CONSUMER_FIXTURE_DIR, MANIFEST_BUILD_CONSUMER_MATRIX_REF,
    MANIFEST_BUILD_CONSUMER_RECORD_KIND, MANIFEST_BUILD_CONSUMER_REPORT_REF,
    MANIFEST_BUILD_CONSUMER_ROW_RECORD_KIND, MANIFEST_BUILD_CONSUMER_SCHEMA_REF,
    MANIFEST_BUILD_CONSUMER_SCHEMA_VERSION,
};
pub use cluster_context_and_live_resource::{
    validate_packet as validate_cluster_live_resource_packet, ClusterContextStrip,
    ClusterLiveResourcePacket, ClusterLiveResourceValidationReport, ConsoleHandoffTruth,
    MutatingActionGate, OpsSurface, OpsSurfaceProjection, OpsToolKind, TruthMode, TruthModeView,
    CLUSTER_LIVE_RESOURCE_DOC_REF, CLUSTER_LIVE_RESOURCE_FIXTURE_DIR,
    CLUSTER_LIVE_RESOURCE_PACKET_RECORD_KIND, CLUSTER_LIVE_RESOURCE_SCHEMA_REF,
    CLUSTER_LIVE_RESOURCE_SCHEMA_VERSION,
};
pub use freeze_the_m5_manifest_editor_schema_validator_resource_link_build_adapter_target_graph_and_fallback_confidence_component_matrix::{
    current_m5_manifest_build_component_matrix_export, seeded_manifest_build_component_matrix,
    truth_mode_token, AdapterSourceBadgeDescriptor, CapabilityMatrixDescriptor, ComponentRow,
    DegradedState, FallbackConfidenceDrawerDescriptor, ManifestBuildComponentArtifactError,
    ManifestBuildComponentMatrix, ManifestBuildComponentMatrixInput,
    ManifestBuildComponentViolation, ManifestBuildConsumerProjection, ManifestBuildGuardrails,
    ManifestEditorHeaderDescriptor, RawEventDrawerDescriptor, ResourceExplorerRowDescriptor,
    ResourceLinkRowDescriptor, SchemaValidatorRowDescriptor, TargetContextChipGroupDescriptor,
    TargetGraphRowDescriptor, M5AdapterSourceKind, M5CapabilityState, M5DiscoveryConfidence,
    M5FallbackConfidenceState, M5FallbackReason, M5FallbackRecoveryRoute,
    M5ManifestBuildComponentFamily, M5ManifestBuildDowngradeTrigger, M5ManifestBuildRequiredLabel,
    M5ManifestEditPosture, M5RawEventChannel, M5ResourceFreshness, M5ResourceLinkClass,
    M5SchemaFreshness, M5SchemaValidationState, M5TargetGraphNodeKind,
    MANIFEST_BUILD_COMPONENT_MATRIX_ARTIFACT_REF, MANIFEST_BUILD_COMPONENT_MATRIX_DOC_REF,
    MANIFEST_BUILD_COMPONENT_MATRIX_FIXTURE_DIR, MANIFEST_BUILD_COMPONENT_MATRIX_RECORD_KIND,
    MANIFEST_BUILD_COMPONENT_MATRIX_SCHEMA_REF, MANIFEST_BUILD_COMPONENT_MATRIX_SCHEMA_VERSION,
    MANIFEST_BUILD_COMPONENT_MATRIX_SUMMARY_REF,
};
pub use implement_the_m5_adapter_drift_banner_launcher_state_and_no_higher_confidence_overwrite_primitive::{
    current_stable_m5_execution_confidence_export, resolve_execution_confidence,
    seeded_m5_execution_confidence_packet, M5AffordanceState, M5CapabilityDeltaKind,
    M5ExecutionActionKind, M5ExecutionConfidenceArtifactError, M5ExecutionConfidenceCase,
    M5ExecutionConfidenceConsumerProjection, M5ExecutionConfidenceGovernanceReview,
    M5ExecutionConfidenceInput, M5ExecutionConfidencePrimitivePacket,
    M5ExecutionConfidencePrimitivePacketInput, M5ExecutionConfidenceReleasePosture,
    M5ExecutionConfidenceResolutionError, M5ExecutionConfidenceViolation,
    M5ExecutionConfidenceVocabularySet, M5ExecutionExportField, M5ExecutionParitySurface,
    M5ExecutionSurfaceFamily, M5ExecutionSurfaceRow, M5ExecutionVerbInput, M5OverwriteVerdict,
    M5ResolvedAdapterDriftBanner, M5ResolvedExecutionConfidence, M5ResolvedExecutionLauncher,
    M5ResolvedLaunchAffordance, M5ResolvedOverwriteGuard, M5ResolvedParityConsumer,
    M5ResolvedVerbDelta, M5_EXECUTION_CONFIDENCE_ARTIFACT_REF,
    M5_EXECUTION_CONFIDENCE_BUILD_PRIMITIVE_REF, M5_EXECUTION_CONFIDENCE_COMPONENT_MATRIX_REF,
    M5_EXECUTION_CONFIDENCE_CSV_REF, M5_EXECUTION_CONFIDENCE_DOC_REF,
    M5_EXECUTION_CONFIDENCE_FIXTURE_DIR, M5_EXECUTION_CONFIDENCE_RECORD_KIND,
    M5_EXECUTION_CONFIDENCE_REPORT_REF, M5_EXECUTION_CONFIDENCE_SCHEMA_REF,
    M5_EXECUTION_CONFIDENCE_SCHEMA_VERSION,
};
pub use implement_the_m5_adapter_source_badge_target_graph_capability_matrix_raw_event_and_fallback_confidence_primitive::{
    current_stable_m5_build_confidence_export, resolve_build_confidence,
    seeded_m5_build_confidence_packet, M5BuildActionKind, M5BuildConfidenceArtifactError,
    M5BuildConfidenceCase, M5BuildConfidenceConsumerProjection, M5BuildConfidenceExportField,
    M5BuildConfidenceGovernanceReview, M5BuildConfidenceInput, M5BuildConfidencePrimitivePacket,
    M5BuildConfidencePrimitivePacketInput, M5BuildConfidenceReleasePosture,
    M5BuildConfidenceResolutionError, M5BuildConfidenceSurfaceFamily, M5BuildConfidenceSurfaceRow,
    M5BuildConfidenceViolation, M5BuildConfidenceVocabularySet, M5BuildVerb, M5CapabilityCell,
    M5ResolvedAdapterSourceBadge, M5ResolvedBuildConfidence, M5ResolvedCapabilityCell,
    M5ResolvedCapabilityMatrix, M5ResolvedFallbackConfidenceDrawer, M5ResolvedRawEventDrawer,
    M5ResolvedTargetGraphRow, M5TargetIdentity, M5_BUILD_CONFIDENCE_ARTIFACT_REF,
    M5_BUILD_CONFIDENCE_COMPONENT_MATRIX_REF, M5_BUILD_CONFIDENCE_CSV_REF,
    M5_BUILD_CONFIDENCE_DOC_REF, M5_BUILD_CONFIDENCE_FIXTURE_DIR,
    M5_BUILD_CONFIDENCE_RECORD_KIND, M5_BUILD_CONFIDENCE_REPORT_REF,
    M5_BUILD_CONFIDENCE_SCHEMA_REF, M5_BUILD_CONFIDENCE_SCHEMA_VERSION,
};
pub use implement_the_m5_manifest_editor_schema_validator_and_target_context_apply_review_primitive::{
    current_stable_m5_manifest_authoring_export, resolve_manifest_authoring,
    seeded_m5_manifest_authoring_packet, M5DryRunAvailability, M5ExecutionOrigin,
    M5ManifestAuthoringArtifactError, M5ManifestAuthoringCase,
    M5ManifestAuthoringConsumerProjection, M5ManifestAuthoringExportField,
    M5ManifestAuthoringGovernanceReview, M5ManifestAuthoringInput,
    M5ManifestAuthoringPrimitivePacket, M5ManifestAuthoringPrimitivePacketInput,
    M5ManifestAuthoringReleasePosture, M5ManifestAuthoringResolutionError,
    M5ManifestAuthoringSurfaceFamily, M5ManifestAuthoringSurfaceRow, M5ManifestAuthoringViolation,
    M5ManifestAuthoringVocabularySet, M5ManifestSourceType, M5MutationCounts,
    M5ResolvedApplyReviewBanner, M5ResolvedManifestAuthoring, M5ResolvedManifestHeader,
    M5ResolvedSchemaValidatorRow, M5ResolvedTargetContextChips, M5RollbackPosture,
    M5SchemaSourceKind, M5TargetContextChips, M5_MANIFEST_AUTHORING_ARTIFACT_REF,
    M5_MANIFEST_AUTHORING_COMPONENT_MATRIX_REF, M5_MANIFEST_AUTHORING_CSV_REF,
    M5_MANIFEST_AUTHORING_DOC_REF, M5_MANIFEST_AUTHORING_FIXTURE_DIR,
    M5_MANIFEST_AUTHORING_RECORD_KIND, M5_MANIFEST_AUTHORING_REPORT_REF,
    M5_MANIFEST_AUTHORING_SCHEMA_REF, M5_MANIFEST_AUTHORING_SCHEMA_VERSION,
};
pub use implement_the_m5_resource_link_compare_card_explorer_and_drift_banner_primitive::{
    current_stable_m5_live_resource_export, resolve_live_resource_navigation,
    seeded_m5_live_resource_packet, M5CompareVerdict, M5LiveResourceArtifactError,
    M5LiveResourceCase, M5LiveResourceConsumerProjection, M5LiveResourceExportField,
    M5LiveResourceGovernanceReview, M5LiveResourceInput, M5LiveResourcePrimitivePacket,
    M5LiveResourcePrimitivePacketInput, M5LiveResourceReleasePosture,
    M5LiveResourceResolutionError, M5LiveResourceSurfaceFamily, M5LiveResourceSurfaceRow,
    M5LiveResourceViolation, M5LiveResourceVocabularySet, M5PermissionPosture, M5ResolvedCompareCard,
    M5ResolvedDriftBanner, M5ResolvedLiveResource, M5ResolvedResourceExplorerRow,
    M5ResolvedResourceLinkRow, M5ResourceActionKind, M5ResourceHealth, M5ResourceIdentity,
    M5ResourceKind, M5_LIVE_RESOURCE_ARTIFACT_REF, M5_LIVE_RESOURCE_COMPONENT_MATRIX_REF,
    M5_LIVE_RESOURCE_CSV_REF, M5_LIVE_RESOURCE_DOC_REF, M5_LIVE_RESOURCE_FIXTURE_DIR,
    M5_LIVE_RESOURCE_RECORD_KIND, M5_LIVE_RESOURCE_REPORT_REF, M5_LIVE_RESOURCE_SCHEMA_REF,
    M5_LIVE_RESOURCE_SCHEMA_VERSION,
};
pub use infrastructure_surface_qualification::{
    current_infrastructure_surface_qualification_export,
    seeded_infrastructure_surface_qualification_packet, EvidenceCurrency,
    InfrastructureEvidenceConsumer, InfrastructureEvidenceConsumerBinding,
    InfrastructureEvidenceIndexEntry, InfrastructureNarrowReason, InfrastructureProofClass,
    InfrastructureSurface, InfrastructureSurfaceQualificationArtifactError,
    InfrastructureSurfaceQualificationPacket, InfrastructureSurfaceQualificationPacketInput,
    InfrastructureSurfaceQualificationViolation, InfrastructureSurfaceRow,
    InfrastructureSurfaceVerdict, INFRASTRUCTURE_SURFACE_QUALIFICATION_ARTIFACT_REF,
    INFRASTRUCTURE_SURFACE_QUALIFICATION_DOC_REF, INFRASTRUCTURE_SURFACE_QUALIFICATION_FIXTURE_DIR,
    INFRASTRUCTURE_SURFACE_QUALIFICATION_HELP_REF,
    INFRASTRUCTURE_SURFACE_QUALIFICATION_RECORD_KIND,
    INFRASTRUCTURE_SURFACE_QUALIFICATION_SCHEMA_REF,
    INFRASTRUCTURE_SURFACE_QUALIFICATION_SCHEMA_VERSION,
    INFRASTRUCTURE_SURFACE_QUALIFICATION_SUMMARY_REF,
};
pub use plan_and_validation_viewers::{
    seeded_plan_and_validation_viewer_packet, validate_plan_and_validation_viewer_packet,
    PlanAndValidationViewerPacket, PlanAndValidationViewerValidationReport,
    PlanValidationToolIdentity, PlanValidationViewerKind, PlanValidationViewerRecord,
    PlanValidationViewerResult, ViewerAuthoritySourceClass, ViewerConsumerJoin,
    ViewerConsumerSurface, ViewerFollowUpGate, PLAN_AND_VALIDATION_VIEWER_DOC_REF,
    PLAN_AND_VALIDATION_VIEWER_FIXTURE_DIR, PLAN_AND_VALIDATION_VIEWER_PACKET_RECORD_KIND,
    PLAN_AND_VALIDATION_VIEWER_SCHEMA_REF, PLAN_AND_VALIDATION_VIEWER_SCHEMA_VERSION,
};
pub use provider_overlay_and_vendor_console_handoff_continuity::{
    seeded_provider_overlay_handoff_packet, validate_provider_overlay_handoff_packet,
    OverlayContinuitySurface, OverlayContinuitySurfaceBinding, ProviderOverlayDisclosureRow,
    ProviderOverlayHandoffContinuityPacket, ProviderOverlayHandoffContinuityValidationReport,
    PROVIDER_OVERLAY_HANDOFF_ARTIFACT_REF, PROVIDER_OVERLAY_HANDOFF_DOC_REF,
    PROVIDER_OVERLAY_HANDOFF_FIXTURE_DIR, PROVIDER_OVERLAY_HANDOFF_PACKET_RECORD_KIND,
    PROVIDER_OVERLAY_HANDOFF_SCHEMA_REF, PROVIDER_OVERLAY_HANDOFF_SCHEMA_VERSION,
};
pub use relation_graph_incident_support_parity::{
    seeded_relation_graph_incident_support_parity_packet,
    validate_relation_graph_incident_support_parity_packet, ConnectorSkewState, ExecutionPlane,
    LocalityMismatchState, ParityDrillClass, ParityDrillResolution, RelationGraphConsumerBinding,
    RelationGraphHandoffLineage, RelationGraphIncidentSupportParityPacket,
    RelationGraphIncidentSupportParityValidationReport, RelationGraphParityDrill,
    RelationGraphParitySurface, RelationGraphSelection, StaleLiveOverlayState,
    RELATION_GRAPH_PARITY_ARTIFACT_REF, RELATION_GRAPH_PARITY_DOC_REF,
    RELATION_GRAPH_PARITY_FIXTURE_DIR, RELATION_GRAPH_PARITY_PACKET_RECORD_KIND,
    RELATION_GRAPH_PARITY_SCHEMA_REF, RELATION_GRAPH_PARITY_SCHEMA_VERSION,
};
pub use source_intelligence_and_resource_relationships::{
    seeded_source_intelligence_object_packet, validate_object_packet,
    validate_packet as validate_source_intelligence_relationship_packet, ConsoleHandoffPosture,
    DowngradeProfile, ExportFidelity, InfrastructureConsumerProjection,
    InfrastructureConsumerSurface, InfrastructureEnvironmentSliceExplanation, InfrastructureFamily,
    InfrastructureFamilyMatrixRow, InfrastructureJourneyKind, InfrastructureJourneyStatus,
    InfrastructureJourneySurface, InfrastructureObjectIdentity, InfrastructureObjectLineage,
    InfrastructureObjectRecord, InfrastructureObjectRelationRecord, InfrastructureRelationJourney,
    InfrastructureSurfaceView, LiveAccessPrerequisite, RelationEdgeBinding, RelationEdgeClass,
    SourceIntelligenceObjectPacket, SourceIntelligenceObjectPacketValidationReport,
    SourceIntelligenceRelationshipMatrixPacket,
    SourceIntelligenceRelationshipMatrixValidationReport, TargetContextField,
    TargetContextRequirement, TargetContextRequirementClass, TruthLayer, TruthLayerBinding,
    SOURCE_INTELLIGENCE_OBJECT_FIXTURE_DIR, SOURCE_INTELLIGENCE_OBJECT_PACKET_RECORD_KIND,
    SOURCE_INTELLIGENCE_OBJECT_SCHEMA_REF, SOURCE_INTELLIGENCE_OBJECT_SCHEMA_VERSION,
    SOURCE_INTELLIGENCE_RELATIONSHIP_DOC_REF, SOURCE_INTELLIGENCE_RELATIONSHIP_FIXTURE_DIR,
    SOURCE_INTELLIGENCE_RELATIONSHIP_PACKET_RECORD_KIND,
    SOURCE_INTELLIGENCE_RELATIONSHIP_SCHEMA_REF, SOURCE_INTELLIGENCE_RELATIONSHIP_SCHEMA_VERSION,
};
pub use target_context_and_control_plane_boundary::{
    validate_packet, ActionEnvelope, ActionKind, ActionPosture, BoundaryActionReview,
    ConnectorClass, ConnectorClassPolicy, ControlPlaneAuthorityBoundary, ControlPlaneBreadcrumb,
    ControlPlaneHandoff, ControlPlaneHandoffDestinationClass, ControlPlaneHandoffReason,
    ControlPlaneReturnAnchor, ControlPlaneReturnSurface, ControlPlaneTargetIdentity,
    EnvironmentCompleteness, EnvironmentContext, FreshnessLabel, InfraBoundaryFinding,
    InfraBoundaryFindingSeverity, InfraBoundaryPacket, InfraBoundaryValidationReport,
    QualificationPosture, ResourceLinkRow, StateClass, SurfaceBinding, SurfaceKind,
    TargetContextChip, TruthClass, CONTROL_PLANE_BOUNDARY_DOC_REF,
    CONTROL_PLANE_BOUNDARY_FIXTURE_DIR, CONTROL_PLANE_BOUNDARY_PACKET_RECORD_KIND,
    CONTROL_PLANE_BOUNDARY_SCHEMA_REF, CONTROL_PLANE_BOUNDARY_SCHEMA_VERSION,
};
