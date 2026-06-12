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
pub mod plan_and_validation_viewers;
pub mod provider_overlay_and_vendor_console_handoff_continuity;
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
