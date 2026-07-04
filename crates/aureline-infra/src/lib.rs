//! Infrastructure target-context, source-intelligence, and control-plane packets.
//!
//! This crate owns the qualification model that keeps infrastructure-facing
//! surfaces honest about target identity, truth layers, relationship edges,
//! action safety, and vendor-console handoff posture. It does not implement
//! live Kubernetes, cloud, or console connectors; it validates the packet
//! evidence those surfaces must emit before any stable ops claim can be
//! promoted.

#![doc(html_root_url = "https://docs.rs/aureline-infra/0.0.0")]

pub mod cluster_context_and_live_resource;
pub mod freeze_the_m5_manifest_editor_schema_validator_resource_link_build_adapter_target_graph_and_fallback_confidence_component_matrix;
pub mod implement_the_m5_manifest_editor_schema_validator_and_target_context_apply_review_primitive;
pub mod infrastructure_surface_qualification;
pub mod plan_and_validation_viewers;
pub mod provider_overlay_and_vendor_console_handoff_continuity;
pub mod relation_graph_incident_support_parity;
pub mod source_intelligence_and_resource_relationships;
pub mod target_context_and_control_plane_boundary;

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
