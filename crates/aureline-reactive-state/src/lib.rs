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
//! Six pieces sit behind this crate's public surface:
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
//! - [`harness`] — the frozen scenario table: nominal, warming →
//!   full, out-of-order delta / resync, upstream-input-stale,
//!   refresh-ordering (authority precedes derived), replay
//!   isolation, imported read-only, terminal unavailable, and
//!   backpressure switch. Each scenario emits one per-scenario
//!   artifact plus contributes to an aggregate.
//!
//! Known holes (no real transport, no real producer crates, no
//! scope-permission lattice, no ADR-0004 event wire, no signed
//! imported bundles) live in
//! [`prototypes/reactive_state/README.md`](https://github.com/ahmeddyounis/Aureline/blob/main/prototypes/reactive_state/README.md)
//! and are tracked as carry-forward items, not silent capabilities
//! of this prototype.

#![doc(html_root_url = "https://docs.rs/aureline-reactive-state/0.0.0")]

pub mod envelope;
pub mod harness;
pub mod hooks;
pub mod producers;
pub mod store;
pub mod trace;

pub use envelope::{
    AuthorityClass, BackpressureMode, CausedBy, Completeness, DerivationClass, FrameClass,
    Freshness, InputDigest, Invalidation, JsonValue, ProducerRef, ScopeClass, ScopeRef,
    StaleReason, SubscriptionEnvelope, TerminalReason, ViewClass, SUBSCRIPTION_SCHEMA_VERSION,
};
pub use hooks::HookCounters;
pub use producers::{
    derived_diagnostics, file_identity, graph_neighborhood, provider_overlay, shell_health,
    window, workspace, workspace_readiness,
};
pub use store::{
    freshness_is_downgrade, ConsumerProjection, Emission, Producer, ReactiveStore, SamplePayload,
    StoreError,
};
pub use trace::{trace_event_to_json, trace_to_json, ConsumerObservation, TraceEvent};
