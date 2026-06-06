//! Collaboration contracts shared by shell, companion, support, and export flows.
//!
//! This crate owns typed session-envelope artifacts for collaboration surfaces
//! that are visible before the full collaboration engine is promoted. The first
//! module qualifies session roles, invite/admission state, observer joins,
//! presenter/follow behavior, retention/export/delete truth, and downgrade
//! behavior for any M4-exposed collaboration-adjacent lane. It also owns the
//! shared terminal/debugger control-plane qualification packet that keeps
//! mutating control separate from presence and follow state.

#![doc(html_root_url = "https://docs.rs/aureline-collab/0.0.0")]

pub mod session_role_admission_and_retention_qualification;
pub mod shared_terminal_debug_control_plane;

pub use session_role_admission_and_retention_qualification::{
    current_session_role_admission_and_retention_qualification, AdmissionState, ClientBoundary,
    CollaborationLaneKind, ConsentTrigger, DowngradeClass, ExportDeleteRight, FollowState,
    GuestScope, LocalContinuityPosture, PresenterRole, QualificationLabel, RetentionMode,
    SessionEnvelopeRecord, SessionLifecycleState, SessionRoleAdmissionAndRetentionQualification,
    SessionRoleQualificationViolation, SessionRoleRequested, StableProofPacket,
};
pub use shared_terminal_debug_control_plane::{
    current_shared_terminal_debug_control_plane, SharedControlClientBoundary,
    SharedControlFollowState, SharedControlGrantRecord, SharedControlGrantScope,
    SharedControlGrantState, SharedControlGuardrail, SharedControlLineageEvent,
    SharedControlLineageEventKind, SharedControlQualificationLabel, SharedControlRestorePosture,
    SharedControlRetentionProfile, SharedControlSummary, SharedControlSurfaceKind,
    SharedControlViolation, SharedTerminalDebugControlPlane, SHARED_CONTROL_PACKET_PATH,
    SHARED_CONTROL_RECORD_KIND, SHARED_CONTROL_SCHEMA_VERSION,
};
