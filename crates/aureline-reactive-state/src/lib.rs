//! Reactive state and subscription-envelope prototype.
//!
//! This crate is a contract-first prototype for the reactive
//! subscription fabric frozen in
//! `docs/adr/0005-subscription-envelope-and-invalidation-semantics.md`
//! and the boundary schema at
//! `schemas/runtime/subscription_envelope.schema.json`. Its goal is
//! not performance: it is to make every envelope field, every
//! lifecycle step, every freshness / completeness / stale-reason /
//! terminal-reason token, every authority / derivation split, every
//! materialized-view class, and every protected-hot-path hook the
//! ADR names observable against a frozen scenario table so the
//! vocabulary cannot silently drift.
//!
//! Eight pieces sit behind this crate's public surface:
//!
//! - [`envelope`] — the frozen vocabulary enums, the envelope
//!   struct, and a hand-rolled canonical JSON renderer. The
//!   rendered JSON matches the boundary schema field by field.
//! - [`hooks`] — the protected-hot-path + observability hook
//!   counters the ADR names. Eight protected, five observability.
//! - [`store`] — the contract-enforcing [`ReactiveStore`]. Every
//!   public method fires a hook, records a trace event, and refuses
//!   to silently paper over a rule violation (derived ≠
//!   authoritative, imported MUST be derived, delta-before-snapshot,
//!   already-terminated, replay-does-not-advance-live-epoch).
//! - [`trace`] — the per-scenario invalidation-trace record shape
//!   emitted under `artifacts/state/invalidation_trace_examples/`.
//!   Synthetic monotonic ticks replace wall clock so artifacts stay
//!   byte-stable across hosts.
//! - [`producers`] — factory constructors for the four producer
//!   families the spec lists (shell health, workspace readiness,
//!   file identity, derived / materialized views) plus a provider-
//!   overlay factory used by the terminal-unavailable scenario.
//! - [`query_envelope`] — the search / graph / docs query
//!   projection layered over the subscription envelope. It gives
//!   live consumers, support exports, and benchmark traces one
//!   state grammar for loading, partial results, staleness,
//!   cancellation, and failure.
//! - [`harness`] — the frozen scenario table: nominal, warming →
//!   full, out-of-order delta / resync, upstream-input-stale,
//!   refresh-ordering (authority precedes derived), replay
//!   isolation, imported read-only, terminal unavailable, and
//!   backpressure switch. Each scenario emits one per-scenario
//!   artifact plus contributes to an aggregate.
//! - [`verification`] — snapshot-vs-delta parity helpers and
//!   invalidation-order audits extracted from the same scenario
//!   reports. These feed the verification packet and state
//!   artifact directories without forking the scenario table.
//!
//! Known holes (no real transport, no real producer crates, no
//! scope-permission lattice, no ADR-0004 event wire, no signed
//! imported bundles) live in
//! [`prototypes/reactive_state/README.md`](https://github.com/ahmeddyounis/Aureline/blob/main/prototypes/reactive_state/README.md)
//! and are tracked as carry-forward items, not silent capabilities
//! of this prototype.

#![doc(html_root_url = "https://docs.rs/aureline-reactive-state/0.0.0")]

pub mod envelope;
pub mod generated_lineage;
pub mod harness;
pub mod hooks;
pub mod m5_mutation_lineage;
pub mod mutation_journal;
pub mod producers;
pub mod query_envelope;
pub mod reactive_views;
pub mod runtime;
pub mod state_class_recovery;
pub mod store;
pub mod trace;
pub mod verification;

pub use envelope::{
    AuthorityClass, BackpressureMode, CausedBy, Completeness, DerivationClass, FrameClass,
    Freshness, InputDigest, Invalidation, JsonValue, ProducerRef, ScopeClass, ScopeRef,
    StaleReason, SubscriptionEnvelope, TerminalReason, ViewClass, SUBSCRIPTION_SCHEMA_VERSION,
};
pub use hooks::HookCounters;
pub use m5_mutation_lineage::{
    seeded_m5_mutation_lineage_fixtures, seeded_m5_mutation_lineage_packet,
    validate_m5_mutation_lineage_fixture, validate_m5_mutation_lineage_packet,
    ActorClass as M5MutationActorClass, ArtifactClass as M5MutationArtifactClass,
    AutomationInfluence as M5MutationAutomationInfluence,
    CheckpointClass as M5MutationCheckpointClass, CheckpointRef as M5MutationCheckpointRef,
    CheckpointRole as M5MutationCheckpointRole, GroupPhaseClass as M5MutationGroupPhaseClass,
    HistoryInspectorRow as M5MutationHistoryInspectorRow, M5MutationLineageFixture,
    M5MutationLineagePacket, MutationEntry as M5MutationEntry,
    MutationGroupRecord as M5MutationGroupRecord, MutationSurfaceClass as M5MutationSurfaceClass,
    PolicyInfluence as M5MutationPolicyInfluence, ReversalClass as M5MutationReversalClass,
    ScopeClass as M5MutationScopeClass, SourceClass as M5MutationSourceClass,
    SourceContractRefs as M5MutationSourceContractRefs,
    SupportExportManifestRow as M5MutationSupportExportManifestRow,
    ValidationReport as M5MutationLineageValidationReport,
    ValidationViolation as M5MutationLineageValidationViolation, M5_MUTATION_LINEAGE_DOC_REF,
    M5_MUTATION_LINEAGE_FIXTURE_DIR, M5_MUTATION_LINEAGE_FIXTURE_MANIFEST_REF,
    M5_MUTATION_LINEAGE_FIXTURE_RECORD_KIND, M5_MUTATION_LINEAGE_PACKET_RECORD_KIND,
    M5_MUTATION_LINEAGE_PACKET_REF, M5_MUTATION_LINEAGE_REPORT_REF, M5_MUTATION_LINEAGE_SCHEMA_REF,
    M5_MUTATION_LINEAGE_SCHEMA_VERSION,
};
pub use producers::{
    derived_diagnostics, file_identity, graph_neighborhood, provider_overlay, shell_health, window,
    workspace, workspace_readiness,
};
pub use query_envelope::{
    LiveQueryEnvelopeRuntime, QueryCancellationReason, QueryConsumerSurface,
    QueryEnvelopeBenchmarkFrame, QueryEnvelopeBenchmarkTrace, QueryEnvelopeObserver,
    QueryEnvelopePayload, QueryEnvelopeRecord, QueryEnvelopeState, QueryEnvelopeSubscriptionInput,
    QueryEnvelopeSubscriptionToken, QueryEnvelopeSupportArtifact, QueryEnvelopeSupportRow,
    QueryRefreshReason, QUERY_ENVELOPE_ALPHA_SCHEMA_VERSION,
    QUERY_ENVELOPE_BENCHMARK_TRACE_RECORD_KIND, QUERY_ENVELOPE_RECORD_KIND,
    QUERY_ENVELOPE_SUPPORT_ARTIFACT_RECORD_KIND,
};
pub use runtime::{
    open_workspace_readiness, readiness_from_envelope, readiness_label_counts, render_chip_line,
    republish_workspace_readiness, workspace_readiness_producer, LiveReactiveStore,
    LiveSubscriptionToken, ReadinessLabel, ReadinessObserver, ReadinessProjection,
    WatcherHealthPhase, WorkspaceLifecyclePhase, WorkspaceReadinessSnapshot,
};
pub use state_class_recovery::{
    seeded_state_class_recovery_fixtures, seeded_state_class_recovery_packet,
    validate_state_class_recovery_fixture, validate_state_class_recovery_packet,
    AuthorityClass as StateClassRecoveryAuthorityClass,
    BlockedCapabilityClass as StateClassRecoveryBlockedCapabilityClass,
    FailureMode as StateClassRecoveryFailureMode,
    PlaceholderActionClass as StateClassRecoveryPlaceholderActionClass,
    PlaceholderContinuityPlan as StateClassRecoveryPlaceholderContinuityPlan,
    PlaceholderKind as StateClassRecoveryPlaceholderKind,
    PreservedContextClass as StateClassRecoveryPreservedContextClass,
    RecoveryFamilyRow as StateClassRecoveryFamilyRow, RecoveryRoute as StateClassRecoveryRoute,
    SourceContractRefs as StateClassRecoverySourceContractRefs,
    StateClass as StateClassRecoveryStateClass, StateClassRecoveryFixture,
    StateClassRecoveryPacket, SurfaceFamily as StateClassRecoverySurfaceFamily,
    ValidationReport as StateClassRecoveryValidationReport,
    ValidationViolation as StateClassRecoveryValidationViolation, STATE_CLASS_RECOVERY_DOC_REF,
    STATE_CLASS_RECOVERY_FIXTURE_DIR, STATE_CLASS_RECOVERY_FIXTURE_MANIFEST_REF,
    STATE_CLASS_RECOVERY_FIXTURE_RECORD_KIND, STATE_CLASS_RECOVERY_PACKET_RECORD_KIND,
    STATE_CLASS_RECOVERY_PACKET_REF, STATE_CLASS_RECOVERY_REPORT_REF,
    STATE_CLASS_RECOVERY_SCHEMA_REF, STATE_CLASS_RECOVERY_SCHEMA_VERSION,
};
pub use store::{
    freshness_is_downgrade, ConsumerProjection, Emission, Producer, ReactiveStore, SamplePayload,
    StoreError,
};
pub use trace::{trace_event_to_json, trace_to_json, ConsumerObservation, TraceEvent};
pub use verification::{
    invalidation_order_audit_to_json, invalidation_order_audits_to_json,
    run_invalidation_order_audits, run_snapshot_delta_parity_cases, DiagnosticsSummaryView,
    InvalidationOrderAudit, InvalidationOrderStep, SnapshotDeltaParityCase,
    SnapshotDeltaParityStep, INVALIDATION_ORDER_AUDIT_SCHEMA_VERSION,
    SNAPSHOT_DELTA_PARITY_SCHEMA_VERSION,
};
