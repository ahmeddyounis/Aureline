//! Typed internal RPC transport, schema, and envelope for Aureline.
//!
//! See `docs/adr/0004-rpc-transport-and-schema-toolchain.md`.
//!
//! The Rust types in this crate are the schema of record for the
//! internal RPC surface; the machine-readable boundary exports in
//! `schemas/rpc/` are derived from the types and reviewed in lockstep
//! with them.
//!
//! This crate provides:
//!
//! - The frozen envelope vocabulary (request, response, event, cancel).
//! - The frozen error taxonomy with stable per-class codes and retry
//!   hints.
//! - The trace-context shape (W3C-tracecontext-compatible) that every
//!   envelope carries unchanged across the remote proxy seam.
//! - The protected-hot-path hook ids and an in-memory observation
//!   registry used by tests, the shell spike, and the benchmark lab.
//! - A small in-process transport that demonstrates the end-to-end
//!   contract (handshake, unary call, deadline enforcement, cancel,
//!   event-stream publish and consume) against the vocabulary above.

#![doc(html_root_url = "https://docs.rs/aureline-rpc/0.0.0")]

pub mod envelope;
pub mod errors;
pub mod hooks;
pub mod manifest;
pub mod trace;
pub mod transport;
pub mod wire;

pub use envelope::{
    ActorClass, Baggage, CancelFrame, ContractVersion, DeliveryMode, EventEnvelope, FrameKind,
    IdempotencyKey, MethodName, ProducerId, RequestEnvelope, ResponseEnvelope, ResponseResult,
    WorkspaceScope, ENVELOPE_SCHEMA_VERSION,
};
pub use errors::{CancelReason, ErrorClass, ErrorPayload, RetryHint};
pub use hooks::{HookId, HookObservation, HookRegistry};
pub use manifest::{
    EventEntry, Idempotency, ManifestDigest, MethodEntry, MethodKind, MethodManifest, ScopeKind,
};
pub use trace::TraceContext;
pub use transport::{InProcessTransport, ServiceHandler, ServiceRegistration};
pub use wire::{EncodingId, WireProtocolVersion, ENCODING_ID, WIRE_PROTOCOL_VERSION};
