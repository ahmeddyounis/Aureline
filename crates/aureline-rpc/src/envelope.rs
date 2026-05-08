//! Envelope types for request, response, event, and cancel frames.
//!
//! These are the frozen vocabulary carried across every internal RPC
//! call. Payload bytes are opaque at this boundary; per-method shapes
//! are described in the service manifest. See ADR 0004 §Request /
//! response envelope and §Event-stream envelope.

use std::collections::BTreeMap;

use crate::errors::{CancelReason, ErrorPayload};
use crate::trace::TraceContext;

/// Current envelope schema version. Bumped only on breaking envelope
/// changes; additive optional envelope fields do not bump this.
pub const ENVELOPE_SCHEMA_VERSION: u32 = 1;

/// Scope carried by every envelope: either a workspace id (stable
/// across renames) or the literal `Global`.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum WorkspaceScope {
    Workspace(String),
    Global,
}

impl WorkspaceScope {
    pub fn workspace(id: impl Into<String>) -> Self {
        Self::Workspace(id.into())
    }

    pub fn is_global(&self) -> bool {
        matches!(self, Self::Global)
    }

    pub fn workspace_id(&self) -> Option<&str> {
        match self {
            Self::Workspace(id) => Some(id.as_str()),
            Self::Global => None,
        }
    }
}

/// Originator classification used by policy, audit, and tracing.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ActorClass {
    User,
    Command,
    Recipe,
    Extension,
    Ai,
    System,
    Remote,
}

impl ActorClass {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::User => "user",
            Self::Command => "command",
            Self::Recipe => "recipe",
            Self::Extension => "extension",
            Self::Ai => "ai",
            Self::System => "system",
            Self::Remote => "remote",
        }
    }
}

/// Delivery mode declared at subscribe time and frozen for the life of
/// the subscription.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DeliveryMode {
    ExactlyOnce,
    AtLeastOnce,
    BestEffort,
}

impl DeliveryMode {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::ExactlyOnce => "exactly_once",
            Self::AtLeastOnce => "at_least_once",
            Self::BestEffort => "best_effort",
        }
    }

    /// Whether producers in this mode MUST carry an
    /// `idempotency_key`. Mirrors the envelope JSON Schema rule.
    pub fn requires_idempotency_key(&self) -> bool {
        matches!(self, Self::AtLeastOnce)
    }
}

/// Frame discriminator for observability and schema validation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum FrameKind {
    Request,
    Response,
    Event,
    Cancel,
}

impl FrameKind {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Request => "request",
            Self::Response => "response",
            Self::Event => "event",
            Self::Cancel => "cancel",
        }
    }
}

/// Fully qualified `service.method` identifier.
///
/// The constructor enforces the dotted-lowercase pattern shared with
/// the envelope JSON Schema.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct MethodName(String);

impl MethodName {
    pub fn new(raw: impl Into<String>) -> Result<Self, &'static str> {
        let s = raw.into();
        if !is_dotted_lowercase(&s) {
            return Err("method name must match service.method dotted-lowercase");
        }
        Ok(Self(s))
    }

    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }

    pub fn service_prefix(&self) -> &str {
        self.0.split_once('.').map(|(s, _)| s).unwrap_or("")
    }
}

/// Semver contract version for a method, for example `1.0.0`.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ContractVersion(String);

impl ContractVersion {
    pub fn new(raw: impl Into<String>) -> Result<Self, &'static str> {
        let s = raw.into();
        if !is_semver(&s) {
            return Err("contract version must be MAJOR.MINOR.PATCH[-PRERELEASE]");
        }
        Ok(Self(s))
    }

    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}

/// Idempotency key carried on selected requests and on every
/// `AtLeastOnce` event.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct IdempotencyKey(String);

impl IdempotencyKey {
    pub fn new(raw: impl Into<String>) -> Result<Self, &'static str> {
        let s = raw.into();
        if s.is_empty() {
            return Err("idempotency key must not be empty");
        }
        if s.len() > 128 {
            return Err("idempotency key must be at most 128 bytes");
        }
        Ok(Self(s))
    }

    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}

/// Baggage map carried on requests and events. Values are bounded to
/// 256 bytes each; the map is bounded to 32 entries.
pub type Baggage = BTreeMap<String, String>;

pub const BAGGAGE_MAX_ENTRIES: usize = 32;
pub const BAGGAGE_MAX_VALUE_BYTES: usize = 256;

/// Producer identity on every event envelope.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ProducerId {
    pub service: String,
    pub instance: String,
}

/// Request envelope.
#[derive(Debug, Clone)]
pub struct RequestEnvelope {
    pub frame_kind: FrameKind,
    pub envelope_schema_version: u32,
    pub request_id: u64,
    pub method: MethodName,
    pub contract_version: ContractVersion,
    pub trace: TraceContext,
    pub workspace_scope: WorkspaceScope,
    /// Absolute deadline on the connection clock, in nanoseconds. Zero
    /// means "no deadline" and is only legal for unbounded
    /// subscriptions.
    pub deadline_ns: u64,
    pub cancellation_channel: u64,
    pub idempotency_key: Option<IdempotencyKey>,
    pub baggage: Baggage,
    pub actor_class: ActorClass,
    pub payload: Vec<u8>,
}

impl RequestEnvelope {
    /// Construct a request envelope with all required fields. The
    /// constructor fixes `frame_kind` and `envelope_schema_version` so
    /// the only way to produce an invalid shape is to mutate the
    /// struct after construction.
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        request_id: u64,
        method: MethodName,
        contract_version: ContractVersion,
        trace: TraceContext,
        workspace_scope: WorkspaceScope,
        deadline_ns: u64,
        cancellation_channel: u64,
        actor_class: ActorClass,
        payload: Vec<u8>,
    ) -> Self {
        Self {
            frame_kind: FrameKind::Request,
            envelope_schema_version: ENVELOPE_SCHEMA_VERSION,
            request_id,
            method,
            contract_version,
            trace,
            workspace_scope,
            deadline_ns,
            cancellation_channel,
            idempotency_key: None,
            baggage: Baggage::new(),
            actor_class,
            payload,
        }
    }
}

/// Response result variants.
#[derive(Debug, Clone)]
pub enum ResponseResult {
    Ok(Vec<u8>),
    Err(ErrorPayload),
    /// Non-terminal streamed progress chunk.
    Progress(Vec<u8>),
}

/// Response envelope. `terminal = true` marks the last frame; progress
/// frames carry `terminal = false`.
#[derive(Debug, Clone)]
pub struct ResponseEnvelope {
    pub frame_kind: FrameKind,
    pub envelope_schema_version: u32,
    pub request_id: u64,
    pub trace: TraceContext,
    pub contract_version: ContractVersion,
    pub result: ResponseResult,
    pub terminal: bool,
    pub server_hint_ns: Option<u64>,
}

impl ResponseEnvelope {
    pub fn ok(
        request_id: u64,
        trace: TraceContext,
        contract_version: ContractVersion,
        payload: Vec<u8>,
    ) -> Self {
        Self {
            frame_kind: FrameKind::Response,
            envelope_schema_version: ENVELOPE_SCHEMA_VERSION,
            request_id,
            trace,
            contract_version,
            result: ResponseResult::Ok(payload),
            terminal: true,
            server_hint_ns: None,
        }
    }

    pub fn err(
        request_id: u64,
        trace: TraceContext,
        contract_version: ContractVersion,
        err: ErrorPayload,
    ) -> Self {
        Self {
            frame_kind: FrameKind::Response,
            envelope_schema_version: ENVELOPE_SCHEMA_VERSION,
            request_id,
            trace,
            contract_version,
            result: ResponseResult::Err(err),
            terminal: true,
            server_hint_ns: None,
        }
    }

    pub fn progress(
        request_id: u64,
        trace: TraceContext,
        contract_version: ContractVersion,
        chunk: Vec<u8>,
    ) -> Self {
        Self {
            frame_kind: FrameKind::Response,
            envelope_schema_version: ENVELOPE_SCHEMA_VERSION,
            request_id,
            trace,
            contract_version,
            result: ResponseResult::Progress(chunk),
            terminal: false,
            server_hint_ns: None,
        }
    }
}

/// Event envelope. Event streams ride on the same connection as
/// requests; the envelope shares the request vocabulary plus
/// stream-specific fields.
#[derive(Debug, Clone)]
pub struct EventEnvelope {
    pub frame_kind: FrameKind,
    pub envelope_schema_version: u32,
    pub subscription_id: u64,
    pub sequence: u64,
    pub kind: String,
    pub trace: TraceContext,
    pub workspace_scope: WorkspaceScope,
    pub schema_version: u32,
    pub producer: ProducerId,
    pub idempotency_key: Option<IdempotencyKey>,
    pub delivery_mode: DeliveryMode,
    pub payload: Vec<u8>,
}

impl EventEnvelope {
    /// Construct an event envelope. Panics in debug if the invariants
    /// shared with the JSON Schema are violated (for example, an
    /// `AtLeastOnce` producer without an `idempotency_key`). The
    /// release build returns an `Err` instead of panicking so callers
    /// always get attribution.
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        subscription_id: u64,
        sequence: u64,
        kind: impl Into<String>,
        trace: TraceContext,
        workspace_scope: WorkspaceScope,
        schema_version: u32,
        producer: ProducerId,
        idempotency_key: Option<IdempotencyKey>,
        delivery_mode: DeliveryMode,
        payload: Vec<u8>,
    ) -> Result<Self, &'static str> {
        if schema_version == 0 {
            return Err("event schema_version must be >= 1");
        }
        if delivery_mode.requires_idempotency_key() && idempotency_key.is_none() {
            return Err("at_least_once producers MUST carry idempotency_key");
        }
        Ok(Self {
            frame_kind: FrameKind::Event,
            envelope_schema_version: ENVELOPE_SCHEMA_VERSION,
            subscription_id,
            sequence,
            kind: kind.into(),
            trace,
            workspace_scope,
            schema_version,
            producer,
            idempotency_key,
            delivery_mode,
            payload,
        })
    }
}

/// Cancel frame carried on the request's cancellation channel.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct CancelFrame {
    pub frame_kind: FrameKind,
    pub envelope_schema_version: u32,
    pub cancellation_channel: u64,
    pub reason: CancelReason,
}

impl CancelFrame {
    pub fn new(cancellation_channel: u64, reason: CancelReason) -> Self {
        Self {
            frame_kind: FrameKind::Cancel,
            envelope_schema_version: ENVELOPE_SCHEMA_VERSION,
            cancellation_channel,
            reason,
        }
    }
}

fn is_dotted_lowercase(s: &str) -> bool {
    if s.is_empty() {
        return false;
    }
    let mut segments = s.split('.');
    let Some(first) = segments.next() else {
        return false;
    };
    if !is_lower_ident(first) {
        return false;
    }
    let mut had_dot = false;
    for seg in segments {
        had_dot = true;
        if !is_lower_ident(seg) {
            return false;
        }
    }
    had_dot
}

fn is_lower_ident(seg: &str) -> bool {
    let mut chars = seg.chars();
    match chars.next() {
        Some(c) if c.is_ascii_lowercase() => {}
        _ => return false,
    }
    for c in chars {
        if !(c.is_ascii_lowercase() || c.is_ascii_digit() || c == '_') {
            return false;
        }
    }
    true
}

fn is_semver(s: &str) -> bool {
    // MAJOR.MINOR.PATCH with optional -PRERELEASE, matches the JSON
    // Schema pattern. Prerelease is a loose charset (alphanumerics, dot,
    // dash) matching the schema.
    let (core, pre) = match s.split_once('-') {
        Some((c, p)) => (c, Some(p)),
        None => (s, None),
    };
    let mut parts = core.split('.');
    let (Some(a), Some(b), Some(c), None) =
        (parts.next(), parts.next(), parts.next(), parts.next())
    else {
        return false;
    };
    if !is_ascii_digits(a) || !is_ascii_digits(b) || !is_ascii_digits(c) {
        return false;
    }
    match pre {
        None => true,
        Some(p) => {
            if p.is_empty() {
                return false;
            }
            p.chars()
                .all(|ch| ch.is_ascii_alphanumeric() || ch == '.' || ch == '-')
        }
    }
}

fn is_ascii_digits(s: &str) -> bool {
    !s.is_empty() && s.chars().all(|c| c.is_ascii_digit())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn method_name_accepts_dotted_lowercase() {
        assert!(MethodName::new("vfs.read_metadata").is_ok());
        assert!(MethodName::new("editor.reflow_with_shape_cache").is_ok());
    }

    #[test]
    fn method_name_rejects_bad_shapes() {
        assert!(MethodName::new("").is_err());
        assert!(MethodName::new("vfs").is_err()); // missing dot
        assert!(MethodName::new("Vfs.read").is_err()); // uppercase
        assert!(MethodName::new("vfs.").is_err()); // empty tail
        assert!(MethodName::new(".read").is_err()); // empty head
    }

    #[test]
    fn contract_version_semver_accepts() {
        assert!(ContractVersion::new("1.0.0").is_ok());
        assert!(ContractVersion::new("2.1.3").is_ok());
        assert!(ContractVersion::new("1.0.0-beta.1").is_ok());
    }

    #[test]
    fn contract_version_semver_rejects() {
        assert!(ContractVersion::new("1.0").is_err());
        assert!(ContractVersion::new("1.0.0.0").is_err());
        assert!(ContractVersion::new("v1.0.0").is_err());
        assert!(ContractVersion::new("1.0.0-").is_err());
    }

    #[test]
    fn idempotency_key_bounds() {
        assert!(IdempotencyKey::new("").is_err());
        assert!(IdempotencyKey::new("ok").is_ok());
        let long = "x".repeat(129);
        assert!(IdempotencyKey::new(long).is_err());
    }

    #[test]
    fn at_least_once_event_requires_idempotency_key() {
        let trace = TraceContext::new_root(1, 1, 0);
        let producer = ProducerId {
            service: "editor".into(),
            instance: "host-1".into(),
        };
        let res = EventEnvelope::new(
            1,
            0,
            "BufferSnapshotDelta",
            trace,
            WorkspaceScope::workspace("ws"),
            1,
            producer.clone(),
            None,
            DeliveryMode::AtLeastOnce,
            vec![],
        );
        assert!(res.is_err());

        let key = IdempotencyKey::new("dedupe-1").unwrap();
        let ok = EventEnvelope::new(
            1,
            0,
            "BufferSnapshotDelta",
            trace,
            WorkspaceScope::workspace("ws"),
            1,
            producer,
            Some(key),
            DeliveryMode::AtLeastOnce,
            vec![],
        );
        assert!(ok.is_ok());
    }
}
