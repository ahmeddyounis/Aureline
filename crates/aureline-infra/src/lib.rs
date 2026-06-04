//! Infrastructure target-context and control-plane boundary packets.
//!
//! This crate owns the qualification model that keeps infrastructure-facing
//! surfaces honest about target identity, connector class, action safety, and
//! vendor-console handoff posture. It does not implement live Kubernetes,
//! cloud, or console connectors; it validates the packet evidence those
//! surfaces must emit before any stable ops claim can be promoted.

#![doc(html_root_url = "https://docs.rs/aureline-infra/0.0.0")]

pub mod target_context_and_control_plane_boundary;

pub use target_context_and_control_plane_boundary::{
    validate_packet, ActionEnvelope, ActionKind, ActionPosture, BoundaryActionReview,
    ConnectorClass, ConnectorClassPolicy, ControlPlaneHandoff, EnvironmentCompleteness,
    EnvironmentContext, FreshnessLabel, InfraBoundaryFinding, InfraBoundaryFindingSeverity,
    InfraBoundaryPacket, InfraBoundaryValidationReport, QualificationPosture, ResourceLinkRow,
    StateClass, SurfaceBinding, SurfaceKind, TargetContextChip, TruthClass,
    CONTROL_PLANE_BOUNDARY_DOC_REF, CONTROL_PLANE_BOUNDARY_FIXTURE_DIR,
    CONTROL_PLANE_BOUNDARY_PACKET_RECORD_KIND, CONTROL_PLANE_BOUNDARY_SCHEMA_REF,
    CONTROL_PLANE_BOUNDARY_SCHEMA_VERSION,
};
