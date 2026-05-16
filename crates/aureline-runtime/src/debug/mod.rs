//! Beta debugger / DAP host supervision.
//!
//! This module owns the runtime supervision seed for debug-adapter sessions.
//! Sessions are keyed by stable session id, scoped to a workspace, root,
//! language, and execution context, and pass through one typed lifecycle
//! state machine that the shell, support export, and release evidence flows
//! consume without forking adapter-specific handling.
//!
//! Adapter crashes, protocol violations, or hangs degrade only the affected
//! session: the supervisor records a typed exit reason, applies the bounded
//! restart budget, and either schedules a reconnect or moves the session
//! into quarantine. Unrelated language hosts, terminal lanes, and other
//! debug sessions are untouched.
//!
//! Reviewer-facing landing page:
//! [`/docs/runtime/m3/debugger_host_beta.md`](../../../../docs/runtime/m3/debugger_host_beta.md).
//! Export schema:
//! [`/schemas/runtime/debug_session.schema.json`](../../../../schemas/runtime/debug_session.schema.json).

mod host;
mod records;

pub use host::{
    DapHostSupervisor, DapHostSupervisorConfig, DapHostSupervisorError, DebugAdapterCapabilityRequest,
    DebugAdapterCapabilityResponse, DebugAdapterNegotiationInput, DebugSessionLaunchSpec,
};
pub use records::{
    DebugAdapterCapabilityClass, DebugAdapterNegotiationOutcome, DebugAdapterTransportClass,
    DebugAdapterIdentity, DebugSessionEventClass, DebugSessionExitReasonClass,
    DebugSessionIdentity, DebugSessionLifecycleEvent, DebugSessionMode, DebugSessionRestartCause,
    DebugSessionSnapshot, DebugSessionStateClass, DebugSessionSupportPacket,
    DebugSessionTargetIdentity, DEBUG_SESSION_EVENT_RECORD_KIND, DEBUG_SESSION_LIFECYCLE_SCHEMA_VERSION,
    DEBUG_SESSION_RECORD_KIND, DEBUG_SESSION_SUPPORT_PACKET_RECORD_KIND,
};
