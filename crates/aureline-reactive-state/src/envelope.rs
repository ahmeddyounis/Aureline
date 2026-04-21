//! Frozen subscription-envelope vocabulary.
//!
//! Mirrors the enums, field set, and lifecycle rules in
//! `docs/adr/0005-subscription-envelope-and-invalidation-semantics.md`
//! and the boundary schema at
//! `schemas/runtime/subscription_envelope.schema.json`. The
//! prototype's Rust types are the schema of record per the ADR; the
//! JSON Schema is the cross-tool boundary. Every `as_str` here
//! matches the exact token emitted in JSON.

use std::fmt::Write as _;

/// Schema version for the subscription payload shape. Bumped only
/// on breaking payload changes; additive-optional fields do not
/// bump this value.
pub const SUBSCRIPTION_SCHEMA_VERSION: u32 = 1;

/// Lifecycle frame class. Consumers branch on this value to
/// decide how to apply the payload.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum FrameClass {
    Snapshot,
    Delta,
    ResyncRequired,
    Terminal,
}

impl FrameClass {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Snapshot => "snapshot",
            Self::Delta => "delta",
            Self::ResyncRequired => "resync_required",
            Self::Terminal => "terminal",
        }
    }
}

/// Frozen freshness vocabulary. Six values. Consumers surface the
/// label verbatim; support bundles quote it without translation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Freshness {
    Authoritative,
    Warming,
    Cached,
    Stale,
    Replayed,
    Imported,
}

impl Freshness {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Authoritative => "authoritative",
            Self::Warming => "warming",
            Self::Cached => "cached",
            Self::Stale => "stale",
            Self::Replayed => "replayed",
            Self::Imported => "imported",
        }
    }

    /// Whether a frame carrying this freshness MUST carry an
    /// `invalidation` object. The ADR ties the obligation to
    /// "freshness != authoritative"; `warming` frames MAY omit
    /// invalidation when they simply have nothing to report yet,
    /// but the schema enforces invalidation on
    /// `{cached, stale, replayed, imported}`.
    pub fn requires_invalidation(self) -> bool {
        matches!(
            self,
            Self::Cached | Self::Stale | Self::Replayed | Self::Imported
        )
    }
}

/// Frozen completeness vocabulary. Four values. Orthogonal to
/// freshness.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Completeness {
    Full,
    Partial,
    Unloaded,
    Unavailable,
}

impl Completeness {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Full => "full",
            Self::Partial => "partial",
            Self::Unloaded => "unloaded",
            Self::Unavailable => "unavailable",
        }
    }
}

/// Frozen backpressure-mode vocabulary.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BackpressureMode {
    Realtime,
    Coalesced,
    SnapshotRequired,
}

impl BackpressureMode {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Realtime => "realtime",
            Self::Coalesced => "coalesced",
            Self::SnapshotRequired => "snapshot_required",
        }
    }
}

/// Frozen authority-class vocabulary. Distinguishes where the
/// canonical truth lives so mutating affordances know whether to
/// re-consult the authoritative producer.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AuthorityClass {
    WorkspaceVfs,
    BufferEditor,
    DerivedKnowledge,
    Execution,
    PolicyEntitlement,
    ProviderOverlay,
}

impl AuthorityClass {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::WorkspaceVfs => "workspace_vfs",
            Self::BufferEditor => "buffer_editor",
            Self::DerivedKnowledge => "derived_knowledge",
            Self::Execution => "execution",
            Self::PolicyEntitlement => "policy_entitlement",
            Self::ProviderOverlay => "provider_overlay",
        }
    }
}

/// Frozen derivation-class vocabulary. Derived frames MAY NOT
/// claim freshness = authoritative.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DerivationClass {
    Authoritative,
    Derived,
}

impl DerivationClass {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Authoritative => "authoritative",
            Self::Derived => "derived",
        }
    }
}

/// Frozen materialized-view-class vocabulary.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ViewClass {
    EphemeralProjection,
    DurableLocalMaterialization,
    ExportableSnapshot,
    ManagedReplicatedView,
}

impl ViewClass {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::EphemeralProjection => "ephemeral_projection",
            Self::DurableLocalMaterialization => "durable_local_materialization",
            Self::ExportableSnapshot => "exportable_snapshot",
            Self::ManagedReplicatedView => "managed_replicated_view",
        }
    }
}

/// Frozen stale-reason vocabulary. Adding a code is additive-minor;
/// repurposing a code requires a new decision row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum StaleReason {
    ProducerRestart,
    AuthorityEpochRolled,
    PolicyEpochChanged,
    WatcherDropped,
    QueueSaturation,
    UpstreamInputStale,
    ExplicitRefreshRequested,
    CacheServed,
    ReplayedFromBundle,
    ImportedFromExternal,
    ScopeRemoved,
    CausalityLost,
}

impl StaleReason {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::ProducerRestart => "producer_restart",
            Self::AuthorityEpochRolled => "authority_epoch_rolled",
            Self::PolicyEpochChanged => "policy_epoch_changed",
            Self::WatcherDropped => "watcher_dropped",
            Self::QueueSaturation => "queue_saturation",
            Self::UpstreamInputStale => "upstream_input_stale",
            Self::ExplicitRefreshRequested => "explicit_refresh_requested",
            Self::CacheServed => "cache_served",
            Self::ReplayedFromBundle => "replayed_from_bundle",
            Self::ImportedFromExternal => "imported_from_external",
            Self::ScopeRemoved => "scope_removed",
            Self::CausalityLost => "causality_lost",
        }
    }
}

/// Frozen terminal-reason vocabulary.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TerminalReason {
    ConsumerCancelled,
    ProducerShutdown,
    ScopeRemoved,
    PolicyTerminated,
    Unavailable,
}

impl TerminalReason {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::ConsumerCancelled => "consumer_cancelled",
            Self::ProducerShutdown => "producer_shutdown",
            Self::ScopeRemoved => "scope_removed",
            Self::PolicyTerminated => "policy_terminated",
            Self::Unavailable => "unavailable",
        }
    }
}

/// Frozen scope-class vocabulary.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ScopeClass {
    Workspace,
    Window,
    ReviewWorkspace,
    RemoteSession,
    Tenant,
    CompanionSurface,
}

impl ScopeClass {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Workspace => "workspace",
            Self::Window => "window",
            Self::ReviewWorkspace => "review_workspace",
            Self::RemoteSession => "remote_session",
            Self::Tenant => "tenant",
            Self::CompanionSurface => "companion_surface",
        }
    }
}

/// Typed subscription scope. Unscoped ambient subscriptions are
/// forbidden on protected surfaces.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ScopeRef {
    pub class: ScopeClass,
    pub id: String,
}

/// One `(name, digest)` pair naming an input this frame was
/// derived from. Required on derived frames; empty on
/// authoritative frames.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InputDigest {
    pub name: String,
    pub digest: String,
}

/// One producer attribution row. `producer_instance` captures
/// host + pid + boot epoch so consumers can distinguish producer
/// restarts. `source` is required on `imported` / `replayed`
/// frames.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProducerRef {
    pub producer_id: String,
    pub producer_instance: String,
    pub producer_version: Option<String>,
    pub input_digests: Vec<InputDigest>,
    pub derivation_epoch: Option<u64>,
    pub source: Option<String>,
}

/// Optional attribution for an invalidation. Any combination of
/// fields may be populated; the presence of a `trace_id` anchors
/// the invalidation to the ADR-0004 trace that carried the cause.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct CausedBy {
    pub authority_epoch: Option<u64>,
    pub policy_epoch: Option<u64>,
    pub upstream_digest: Option<String>,
    pub trace_id: Option<String>,
    pub note: Option<String>,
}

impl CausedBy {
    pub fn is_empty(&self) -> bool {
        self.authority_epoch.is_none()
            && self.policy_epoch.is_none()
            && self.upstream_digest.is_none()
            && self.trace_id.is_none()
            && self.note.is_none()
    }
}

/// Invalidation body carried on non-authoritative frames and on
/// every `resync_required` frame.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Invalidation {
    pub stale_reason: StaleReason,
    pub caused_by: Option<CausedBy>,
}

/// Minimal JSON AST used for payload rendering. Object entries
/// preserve insertion order so emitted JSON is byte-stable across
/// hosts.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum JsonValue {
    Null,
    Bool(bool),
    U64(u64),
    I64(i64),
    Str(String),
    Array(Vec<JsonValue>),
    Object(Vec<(String, JsonValue)>),
}

impl JsonValue {
    pub fn obj(entries: Vec<(&str, JsonValue)>) -> Self {
        JsonValue::Object(
            entries
                .into_iter()
                .map(|(k, v)| (k.to_owned(), v))
                .collect(),
        )
    }

    pub fn str(s: impl Into<String>) -> Self {
        JsonValue::Str(s.into())
    }

    pub fn u(n: u64) -> Self {
        JsonValue::U64(n)
    }
}

/// One subscription frame. The shape matches the ADR envelope
/// and the boundary schema at
/// `schemas/runtime/subscription_envelope.schema.json`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SubscriptionEnvelope {
    pub subscription_schema_version: u32,
    pub subscription_id: u64,
    pub query_family: String,
    pub scope_ref: ScopeRef,
    pub authority_class: AuthorityClass,
    pub derivation_class: DerivationClass,
    pub snapshot_epoch: u64,
    pub delta_seq: u64,
    pub frame_class: FrameClass,
    pub freshness: Freshness,
    pub completeness: Completeness,
    pub backpressure_mode: BackpressureMode,
    pub view_class: ViewClass,
    pub producer_refs: Vec<ProducerRef>,
    pub invalidation: Option<Invalidation>,
    pub terminal_reason: Option<TerminalReason>,
    pub payload: Option<JsonValue>,
}

impl SubscriptionEnvelope {
    /// Render to canonical JSON. Field order matches the ADR
    /// table and the boundary schema; optional fields are omitted
    /// when absent. The renderer is hand-rolled so the committed
    /// artifact under `artifacts/state/invalidation_trace_examples/`
    /// stays byte-stable.
    pub fn to_json(&self) -> String {
        let mut out = String::new();
        write_envelope(&mut out, self, 0);
        out
    }
}

// ---------------------------------------------------------------------------
// Hand-rolled JSON emission.
// ---------------------------------------------------------------------------

pub(crate) fn write_envelope(out: &mut String, env: &SubscriptionEnvelope, indent: usize) {
    out.push('{');
    push_nl(out, indent + 1);
    write_kv_u64(
        out,
        "subscription_schema_version",
        env.subscription_schema_version as u64,
    );
    write_comma_nl(out, indent + 1);
    write_kv_u64(out, "subscription_id", env.subscription_id);
    write_comma_nl(out, indent + 1);
    write_kv_string(out, "query_family", &env.query_family);
    write_comma_nl(out, indent + 1);
    write_key(out, "scope_ref");
    write_scope_ref(out, &env.scope_ref, indent + 1);
    write_comma_nl(out, indent + 1);
    write_kv_string(out, "authority_class", env.authority_class.as_str());
    write_comma_nl(out, indent + 1);
    write_kv_string(out, "derivation_class", env.derivation_class.as_str());
    write_comma_nl(out, indent + 1);
    write_kv_u64(out, "snapshot_epoch", env.snapshot_epoch);
    write_comma_nl(out, indent + 1);
    write_kv_u64(out, "delta_seq", env.delta_seq);
    write_comma_nl(out, indent + 1);
    write_kv_string(out, "frame_class", env.frame_class.as_str());
    write_comma_nl(out, indent + 1);
    write_kv_string(out, "freshness", env.freshness.as_str());
    write_comma_nl(out, indent + 1);
    write_kv_string(out, "completeness", env.completeness.as_str());
    write_comma_nl(out, indent + 1);
    write_kv_string(out, "backpressure_mode", env.backpressure_mode.as_str());
    write_comma_nl(out, indent + 1);
    write_kv_string(out, "view_class", env.view_class.as_str());
    write_comma_nl(out, indent + 1);
    write_key(out, "producer_refs");
    write_producer_refs(out, &env.producer_refs, indent + 1);

    if let Some(inv) = &env.invalidation {
        write_comma_nl(out, indent + 1);
        write_key(out, "invalidation");
        write_invalidation(out, inv, indent + 1);
    }
    if let Some(tr) = env.terminal_reason {
        write_comma_nl(out, indent + 1);
        write_kv_string(out, "terminal_reason", tr.as_str());
    }
    if let Some(payload) = &env.payload {
        write_comma_nl(out, indent + 1);
        write_key(out, "payload");
        write_json_value(out, payload, indent + 1);
    }
    push_nl(out, indent);
    out.push('}');
}

fn write_scope_ref(out: &mut String, scope: &ScopeRef, indent: usize) {
    out.push('{');
    push_nl(out, indent + 1);
    write_kv_string(out, "class", scope.class.as_str());
    write_comma_nl(out, indent + 1);
    write_kv_string(out, "id", &scope.id);
    push_nl(out, indent);
    out.push('}');
}

fn write_producer_refs(out: &mut String, refs: &[ProducerRef], indent: usize) {
    out.push('[');
    for (i, r) in refs.iter().enumerate() {
        if i > 0 {
            out.push(',');
        }
        push_nl(out, indent + 1);
        write_producer_ref(out, r, indent + 1);
    }
    if !refs.is_empty() {
        push_nl(out, indent);
    }
    out.push(']');
}

fn write_producer_ref(out: &mut String, r: &ProducerRef, indent: usize) {
    out.push('{');
    push_nl(out, indent + 1);
    write_kv_string(out, "producer_id", &r.producer_id);
    write_comma_nl(out, indent + 1);
    write_kv_string(out, "producer_instance", &r.producer_instance);
    if let Some(v) = &r.producer_version {
        write_comma_nl(out, indent + 1);
        write_kv_string(out, "producer_version", v);
    }
    if let Some(e) = r.derivation_epoch {
        write_comma_nl(out, indent + 1);
        write_kv_u64(out, "derivation_epoch", e);
    }
    if let Some(s) = &r.source {
        write_comma_nl(out, indent + 1);
        write_kv_string(out, "source", s);
    }
    if !r.input_digests.is_empty() {
        write_comma_nl(out, indent + 1);
        write_key(out, "input_digests");
        out.push('[');
        for (i, d) in r.input_digests.iter().enumerate() {
            if i > 0 {
                out.push(',');
            }
            push_nl(out, indent + 2);
            out.push('{');
            push_nl(out, indent + 3);
            write_kv_string(out, "name", &d.name);
            write_comma_nl(out, indent + 3);
            write_kv_string(out, "digest", &d.digest);
            push_nl(out, indent + 2);
            out.push('}');
        }
        push_nl(out, indent + 1);
        out.push(']');
    }
    push_nl(out, indent);
    out.push('}');
}

fn write_invalidation(out: &mut String, inv: &Invalidation, indent: usize) {
    out.push('{');
    push_nl(out, indent + 1);
    write_kv_string(out, "stale_reason", inv.stale_reason.as_str());
    if let Some(caused) = &inv.caused_by {
        if !caused.is_empty() {
            write_comma_nl(out, indent + 1);
            write_key(out, "caused_by");
            write_caused_by(out, caused, indent + 1);
        }
    }
    push_nl(out, indent);
    out.push('}');
}

fn write_caused_by(out: &mut String, c: &CausedBy, indent: usize) {
    out.push('{');
    let mut first = true;
    if let Some(e) = c.authority_epoch {
        maybe_comma(out, indent + 1, &mut first);
        write_kv_u64(out, "authority_epoch", e);
    }
    if let Some(e) = c.policy_epoch {
        maybe_comma(out, indent + 1, &mut first);
        write_kv_u64(out, "policy_epoch", e);
    }
    if let Some(d) = &c.upstream_digest {
        maybe_comma(out, indent + 1, &mut first);
        write_kv_string(out, "upstream_digest", d);
    }
    if let Some(t) = &c.trace_id {
        maybe_comma(out, indent + 1, &mut first);
        write_kv_string(out, "trace_id", t);
    }
    if let Some(n) = &c.note {
        maybe_comma(out, indent + 1, &mut first);
        write_kv_string(out, "note", n);
    }
    if !first {
        push_nl(out, indent);
    }
    out.push('}');
}

fn maybe_comma(out: &mut String, indent: usize, first: &mut bool) {
    if *first {
        push_nl(out, indent);
        *first = false;
    } else {
        write_comma_nl(out, indent);
    }
}

pub(crate) fn write_json_value(out: &mut String, v: &JsonValue, indent: usize) {
    match v {
        JsonValue::Null => out.push_str("null"),
        JsonValue::Bool(b) => out.push_str(if *b { "true" } else { "false" }),
        JsonValue::U64(n) => {
            let _ = write!(out, "{n}");
        }
        JsonValue::I64(n) => {
            let _ = write!(out, "{n}");
        }
        JsonValue::Str(s) => write_string_literal(out, s),
        JsonValue::Array(items) => {
            out.push('[');
            for (i, item) in items.iter().enumerate() {
                if i > 0 {
                    out.push(',');
                }
                push_nl(out, indent + 1);
                write_json_value(out, item, indent + 1);
            }
            if !items.is_empty() {
                push_nl(out, indent);
            }
            out.push(']');
        }
        JsonValue::Object(entries) => {
            out.push('{');
            for (i, (k, val)) in entries.iter().enumerate() {
                if i > 0 {
                    out.push(',');
                }
                push_nl(out, indent + 1);
                write_key(out, k);
                write_json_value(out, val, indent + 1);
            }
            if !entries.is_empty() {
                push_nl(out, indent);
            }
            out.push('}');
        }
    }
}

pub(crate) fn write_key(out: &mut String, key: &str) {
    write_string_literal(out, key);
    out.push_str(": ");
}

pub(crate) fn write_kv_string(out: &mut String, key: &str, value: &str) {
    write_string_literal(out, key);
    out.push_str(": ");
    write_string_literal(out, value);
}

pub(crate) fn write_kv_u64(out: &mut String, key: &str, value: u64) {
    write_string_literal(out, key);
    out.push_str(": ");
    let _ = write!(out, "{value}");
}

pub(crate) fn write_string_literal(out: &mut String, s: &str) {
    out.push('"');
    for ch in s.chars() {
        match ch {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            '\t' => out.push_str("\\t"),
            '\x08' => out.push_str("\\b"),
            '\x0c' => out.push_str("\\f"),
            c if (c as u32) < 0x20 => {
                let _ = write!(out, "\\u{:04x}", c as u32);
            }
            c => out.push(c),
        }
    }
    out.push('"');
}

pub(crate) fn push_nl(out: &mut String, indent: usize) {
    out.push('\n');
    for _ in 0..indent {
        out.push_str("  ");
    }
}

pub(crate) fn write_comma_nl(out: &mut String, indent: usize) {
    out.push(',');
    push_nl(out, indent);
}

#[cfg(test)]
mod tests {
    use super::*;

    fn minimal_env() -> SubscriptionEnvelope {
        SubscriptionEnvelope {
            subscription_schema_version: SUBSCRIPTION_SCHEMA_VERSION,
            subscription_id: 7,
            query_family: "vfs.tree".to_owned(),
            scope_ref: ScopeRef {
                class: ScopeClass::Workspace,
                id: "ws-x".to_owned(),
            },
            authority_class: AuthorityClass::WorkspaceVfs,
            derivation_class: DerivationClass::Authoritative,
            snapshot_epoch: 1,
            delta_seq: 0,
            frame_class: FrameClass::Snapshot,
            freshness: Freshness::Authoritative,
            completeness: Completeness::Full,
            backpressure_mode: BackpressureMode::Realtime,
            view_class: ViewClass::DurableLocalMaterialization,
            producer_refs: vec![ProducerRef {
                producer_id: "aureline.vfs".to_owned(),
                producer_instance: "host/pid/boot".to_owned(),
                producer_version: None,
                input_digests: vec![],
                derivation_epoch: None,
                source: None,
            }],
            invalidation: None,
            terminal_reason: None,
            payload: Some(JsonValue::obj(vec![("entry_count", JsonValue::u(3))])),
        }
    }

    #[test]
    fn emits_frozen_keys_in_order() {
        let json = minimal_env().to_json();
        let idx_sid = json.find("\"subscription_id\"").unwrap();
        let idx_qf = json.find("\"query_family\"").unwrap();
        let idx_scope = json.find("\"scope_ref\"").unwrap();
        let idx_auth = json.find("\"authority_class\"").unwrap();
        let idx_frame = json.find("\"frame_class\"").unwrap();
        let idx_fresh = json.find("\"freshness\"").unwrap();
        let idx_payload = json.find("\"payload\"").unwrap();
        assert!(idx_sid < idx_qf);
        assert!(idx_qf < idx_scope);
        assert!(idx_scope < idx_auth);
        assert!(idx_auth < idx_frame);
        assert!(idx_frame < idx_fresh);
        assert!(idx_fresh < idx_payload);
    }

    #[test]
    fn omits_optional_fields_when_absent() {
        let json = minimal_env().to_json();
        assert!(!json.contains("\"invalidation\""));
        assert!(!json.contains("\"terminal_reason\""));
    }

    #[test]
    fn byte_stable_across_calls() {
        let env = minimal_env();
        let a = env.to_json();
        let b = env.to_json();
        assert_eq!(a, b);
    }

    #[test]
    fn stale_reason_vocabulary_is_complete() {
        let all = [
            StaleReason::ProducerRestart,
            StaleReason::AuthorityEpochRolled,
            StaleReason::PolicyEpochChanged,
            StaleReason::WatcherDropped,
            StaleReason::QueueSaturation,
            StaleReason::UpstreamInputStale,
            StaleReason::ExplicitRefreshRequested,
            StaleReason::CacheServed,
            StaleReason::ReplayedFromBundle,
            StaleReason::ImportedFromExternal,
            StaleReason::ScopeRemoved,
            StaleReason::CausalityLost,
        ];
        assert_eq!(all.len(), 12);
        for r in all {
            assert!(!r.as_str().is_empty());
        }
    }
}
