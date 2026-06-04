//! Connectivity state, deferred-intent, and reconciliation contracts.
//!
//! This crate owns the stable continuity model shared by managed, provider,
//! request-workspace, remote, shell, service-health, diagnostics, and support
//! export surfaces. It models connectivity as an explicit state machine and
//! requires every networked command to declare queueability and replay safety
//! before offline or reconnect behavior can be admitted.

#![doc(html_root_url = "https://docs.rs/aureline-continuity/0.0.0")]

pub mod connectivity_state_and_deferred_intent;

pub use connectivity_state_and_deferred_intent::{
    admit_deferred_intent, audit_connectivity_continuity_page, replay_decision,
    seeded_connectivity_continuity_page, validate_connectivity_continuity_page, ActorIdentity,
    AuthScopeSnapshot, CommandQueueabilityDeclaration, ConnectivityContinuityDefect,
    ConnectivityContinuityDefectKind, ConnectivityContinuityPage, ConnectivityState,
    DeferredIntent, DeferredIntentAction, DeferredIntentState, DriftDimension,
    DriftRevalidationSnapshot, ExpiryPolicy, IdempotencyKeyShape, LocalSafePromise,
    NetworkCommandDeclaration, OfflineReadClass, QueueAdmissionDecision, QueueAdmissionOutcome,
    QueueabilityClass, ReconciliationDecision, ReconciliationOwnerClass, ReconciliationReviewSheet,
    ReplayOutcome, ReplayRevalidationInput, ReplaySafetyClass, SensitivePayloadPosture,
    ServiceFamily, StaleLabelSemantics, SupportExportPacket, TargetIdentity,
    CONNECTIVITY_CONTINUITY_ARTIFACT_REF, CONNECTIVITY_CONTINUITY_DEFECT_RECORD_KIND,
    CONNECTIVITY_CONTINUITY_DOC_REF, CONNECTIVITY_CONTINUITY_PAGE_RECORD_KIND,
    CONNECTIVITY_CONTINUITY_SCHEMA_REF, CONNECTIVITY_CONTINUITY_SCHEMA_VERSION,
    CONNECTIVITY_CONTINUITY_SHARED_CONTRACT_REF, DEFERRED_INTENT_RECORD_KIND,
    NETWORK_COMMAND_DECLARATION_RECORD_KIND, RECONCILIATION_REVIEW_SHEET_RECORD_KIND,
    SUPPORT_EXPORT_PACKET_RECORD_KIND,
};
