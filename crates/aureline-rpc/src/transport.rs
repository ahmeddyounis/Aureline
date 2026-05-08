//! In-process transport that demonstrates the end-to-end contract.
//!
//! This transport is the prototype seam for the ADR 0004 vocabulary.
//! It handles handshake-driven capability intersection, unary
//! request/response, deadline enforcement, first-class cancel frames,
//! and event-stream publish/consume — all against the same envelope
//! types that the length-prefixed byte-stream transport will ship
//! with. The framing / socket plumbing lands in a follow-up; this
//! transport proves the contract compiles and enforces the rules.

use std::collections::{HashMap, HashSet};
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Arc, Mutex};

use crate::envelope::{
    CancelFrame, DeliveryMode, EventEnvelope, RequestEnvelope, ResponseEnvelope, WorkspaceScope,
};
use crate::errors::{ErrorClass, ErrorPayload, RetryHint};
use crate::hooks::{HookId, HookRegistry};
use crate::manifest::{Idempotency, MethodEntry, MethodManifest};

/// Service-side handler closure. Returns either a terminal payload or
/// a typed error; deadline and cancellation are enforced by the
/// transport around the handler.
pub type ServiceHandler =
    Arc<dyn Fn(&RequestEnvelope) -> Result<Vec<u8>, ErrorPayload> + Send + Sync>;

/// Registration wrapper coupling a method entry in the manifest with
/// its handler.
pub struct ServiceRegistration {
    pub entry: MethodEntry,
    pub handler: ServiceHandler,
}

/// Capability-negotiation outcome for a connection.
#[derive(Debug, Clone)]
struct Capabilities {
    /// Methods both sides support.
    supported_methods: HashSet<String>,
}

/// Shared per-connection cancellation state.
#[derive(Debug, Default)]
struct CancelRegistry {
    /// Cancelled channels observed on this connection. Cancels are
    /// idempotent; repeated cancels do not produce multiple effects.
    cancelled: Mutex<HashSet<u64>>,
}

impl CancelRegistry {
    fn mark(&self, channel: u64) -> bool {
        let mut guard = self.cancelled.lock().expect("cancel registry poisoned");
        guard.insert(channel)
    }

    fn is_cancelled(&self, channel: u64) -> bool {
        self.cancelled
            .lock()
            .expect("cancel registry poisoned")
            .contains(&channel)
    }
}

/// Subscription state on the consumer side.
#[derive(Debug)]
struct SubscriptionState {
    delivery_mode: DeliveryMode,
    /// Sequence of the last delivered event. `None` before the first
    /// delivery.
    last_sequence: Option<u64>,
    /// Dedupe set for `AtLeastOnce` subscriptions. Bounded only by the
    /// documented retention window in a real implementation; the
    /// prototype keeps the full set because the test scope is small.
    dedupe_keys: HashSet<String>,
}

/// In-process transport. Owns:
/// - the server-side manifest and handlers,
/// - the client-side advertised manifest (what the caller is willing
///   to invoke),
/// - the capability intersection chosen at handshake time,
/// - a cancel registry shared across calls,
/// - subscription state,
/// - the hook observation registry.
pub struct InProcessTransport {
    server_manifest: MethodManifest,
    handlers: HashMap<String, ServiceHandler>,
    caller_manifest: MethodManifest,
    capabilities: Option<Capabilities>,
    cancel_registry: Arc<CancelRegistry>,
    subscriptions: Mutex<HashMap<u64, SubscriptionState>>,
    subscription_counter: AtomicU64,
    hooks: HookRegistry,
}

impl InProcessTransport {
    pub fn new(
        server_manifest: MethodManifest,
        caller_manifest: MethodManifest,
        hooks: HookRegistry,
    ) -> Self {
        let mut handlers = HashMap::new();
        for (name, _) in server_manifest.methods.iter() {
            handlers.insert(name.clone(), default_unimplemented_handler());
        }
        Self {
            server_manifest,
            handlers,
            caller_manifest,
            capabilities: None,
            cancel_registry: Arc::new(CancelRegistry::default()),
            subscriptions: Mutex::new(HashMap::new()),
            subscription_counter: AtomicU64::new(1),
            hooks,
        }
    }

    pub fn register(&mut self, registration: ServiceRegistration) {
        let name = registration.entry.name.as_str().to_string();
        // Refuse registrations for methods not advertised in the
        // manifest: the service and the manifest must agree.
        assert!(
            self.server_manifest.methods.contains_key(&name),
            "registering handler for method not in manifest: {name}"
        );
        self.handlers.insert(name, registration.handler);
    }

    /// Perform the capability-negotiation handshake. Chooses the
    /// intersection of supported methods; emits
    /// `RpcHandshakeComplete` and optionally
    /// `RpcCapabilityIntersection`.
    pub fn handshake(&mut self) {
        let intersection: HashSet<String> = self
            .server_manifest
            .intersect_methods(&self.caller_manifest)
            .into_iter()
            .map(|s| s.to_string())
            .collect();
        let advertised = self.caller_manifest.methods.len();
        let intersected = intersection.len() < advertised
            || intersection.len() < self.server_manifest.methods.len();
        self.capabilities = Some(Capabilities {
            supported_methods: intersection,
        });
        self.hooks.emit(
            HookId::RpcHandshakeComplete,
            vec![
                ("server".into(), self.server_manifest.service.clone()),
                ("digest".into(), self.server_manifest.digest.0.clone()),
            ],
        );
        if intersected {
            self.hooks.emit_bare(HookId::RpcCapabilityIntersection);
        }
    }

    pub fn hooks(&self) -> &HookRegistry {
        &self.hooks
    }

    pub fn cancel(&self, frame: CancelFrame) {
        let newly_marked = self.cancel_registry.mark(frame.cancellation_channel);
        // Emit `rpc_cancel_observed` at most once per cancellation
        // channel; repeated cancel frames after the first are
        // idempotent and observability-silent.
        if newly_marked {
            self.hooks.emit(
                HookId::RpcCancelObserved,
                vec![("reason".into(), frame.reason.as_str().to_string())],
            );
        }
    }

    /// Dispatch a unary request. Enforces capability intersection,
    /// deadline (via a caller-supplied "now" in nanoseconds on the
    /// connection clock), cancellation, workspace-scope and
    /// actor-class policy, and idempotency requirements before
    /// invoking the handler.
    pub fn call(&self, now_ns: u64, req: RequestEnvelope) -> ResponseEnvelope {
        self.hooks.emit(
            HookId::RpcRequestSend,
            vec![
                ("method".into(), req.method.as_str().to_string()),
                (
                    "contract_version".into(),
                    req.contract_version.as_str().to_string(),
                ),
            ],
        );

        // Cancel observed before dispatch: respond `cancelled`.
        if self.cancel_registry.is_cancelled(req.cancellation_channel) {
            let err = ErrorPayload::new(
                ErrorClass::Cancelled,
                "rpc.caller_cancelled",
                "request cancelled before dispatch",
                RetryHint::No,
            )
            .with_span(req.trace);
            return self.dispatch_err(req, err);
        }

        // Deadline already elapsed: respond `deadline_exceeded`.
        if req.deadline_ns != 0 && now_ns >= req.deadline_ns {
            self.hooks.emit_bare(HookId::RpcDeadlineExpired);
            let err = ErrorPayload::new(
                ErrorClass::DeadlineExceeded,
                "rpc.deadline_expired",
                "deadline elapsed before dispatch",
                RetryHint::No,
            )
            .with_span(req.trace);
            return self.dispatch_err(req, err);
        }

        // Handshake must have run.
        let Some(caps) = self.capabilities.as_ref() else {
            let err = ErrorPayload::new(
                ErrorClass::Unavailable,
                "rpc.handshake_not_completed",
                "capability negotiation has not completed on this connection",
                RetryHint::After { after_ms: 10 },
            );
            return self.dispatch_err(req, err);
        };

        // Capability intersection: method not supported by one side.
        if !caps.supported_methods.contains(req.method.as_str()) {
            let err = ErrorPayload::new(
                ErrorClass::Unavailable,
                "rpc.method_unavailable",
                "method is not in the capability intersection for this connection",
                RetryHint::After { after_ms: 100 },
            );
            return self.dispatch_err(req, err);
        }

        let entry = match self.server_manifest.method(req.method.as_str()) {
            Some(e) => e,
            None => {
                let err = ErrorPayload::new(
                    ErrorClass::Internal,
                    "rpc.method_missing_from_manifest",
                    "method advertised but absent from the service manifest",
                    RetryHint::No,
                );
                return self.dispatch_err(req, err);
            }
        };

        // Actor-class policy.
        if !entry.actor_classes.contains(&req.actor_class) {
            let err = ErrorPayload::new(
                ErrorClass::Policy,
                "rpc.actor_class_denied",
                format!(
                    "actor class {} not authorised for {}",
                    req.actor_class.as_str(),
                    req.method.as_str()
                ),
                RetryHint::No,
            );
            return self.dispatch_err(req, err);
        }

        // Workspace-scope policy.
        if !scope_matches(entry, &req.workspace_scope) {
            let err = ErrorPayload::new(
                ErrorClass::Policy,
                "rpc.scope_mismatch",
                format!(
                    "workspace scope does not match method scope requirement for {}",
                    req.method.as_str()
                ),
                RetryHint::No,
            );
            return self.dispatch_err(req, err);
        }

        // Idempotency policy.
        if entry.idempotency == Idempotency::Required && req.idempotency_key.is_none() {
            let err = ErrorPayload::new(
                ErrorClass::Local,
                "rpc.idempotency_key_required",
                format!("method {} requires an idempotency_key", req.method.as_str()),
                RetryHint::No,
            );
            return self.dispatch_err(req, err);
        }

        // Deadline-required policy.
        if entry.deadline_required && req.deadline_ns == 0 {
            let err = ErrorPayload::new(
                ErrorClass::Local,
                "rpc.deadline_required",
                format!("method {} requires a finite deadline", req.method.as_str()),
                RetryHint::No,
            );
            return self.dispatch_err(req, err);
        }

        self.hooks.emit(
            HookId::RpcRequestReceive,
            vec![("method".into(), req.method.as_str().to_string())],
        );

        let handler = match self.handlers.get(req.method.as_str()) {
            Some(h) => h.clone(),
            None => {
                let err = ErrorPayload::new(
                    ErrorClass::Internal,
                    "rpc.handler_missing",
                    "method advertised but no handler registered",
                    RetryHint::No,
                );
                return self.dispatch_err(req, err);
            }
        };

        // The prototype invokes the handler inline; a real transport
        // would run it on a service-owned task pool and watch for
        // cancel / deadline mid-flight. Cancel/deadline between
        // dispatch and terminal is left to the next iteration: the
        // hook vocabulary covers it regardless.
        let outcome = (handler)(&req);

        // Re-check cancel post-handler: if the caller cancelled
        // mid-flight, tell the truth about which happened.
        if self.cancel_registry.is_cancelled(req.cancellation_channel) {
            // The handler may have still finished; honour its result,
            // but if it errored we attribute to cancel.
            if outcome.is_err() {
                let err = ErrorPayload::new(
                    ErrorClass::Cancelled,
                    "rpc.caller_cancelled",
                    "request cancelled mid-flight",
                    RetryHint::No,
                )
                .with_span(req.trace);
                return self.dispatch_err(req, err);
            }
        }

        match outcome {
            Ok(payload) => {
                let resp = ResponseEnvelope::ok(
                    req.request_id,
                    req.trace.child(req.request_id.wrapping_add(1)),
                    req.contract_version.clone(),
                    payload,
                );
                self.hooks.emit(
                    HookId::RpcResponseDispatch,
                    vec![("method".into(), req.method.as_str().to_string())],
                );
                resp
            }
            Err(err) => self.dispatch_err(req, err),
        }
    }

    fn dispatch_err(&self, req: RequestEnvelope, err: ErrorPayload) -> ResponseEnvelope {
        self.hooks.emit(
            HookId::RpcErrorClassified,
            vec![
                ("method".into(), req.method.as_str().to_string()),
                ("class".into(), err.class.as_str().to_string()),
                ("code".into(), err.code.clone()),
            ],
        );
        let resp = ResponseEnvelope::err(
            req.request_id,
            req.trace.child(req.request_id.wrapping_add(1)),
            req.contract_version,
            err,
        );
        self.hooks.emit_bare(HookId::RpcResponseDispatch);
        resp
    }

    /// Allocate a fresh subscription id on this connection.
    pub fn subscribe(&self, delivery_mode: DeliveryMode) -> u64 {
        let id = self.subscription_counter.fetch_add(1, Ordering::Relaxed);
        let state = SubscriptionState {
            delivery_mode,
            last_sequence: None,
            dedupe_keys: HashSet::new(),
        };
        self.subscriptions
            .lock()
            .expect("subscriptions poisoned")
            .insert(id, state);
        id
    }

    /// Publish an event to a subscription. Emits `EventStreamPublish`
    /// and, on the consumer side,
    /// `EventStreamConsume`/`EventStreamGapDetected`/
    /// `EventStreamDedupeHit` as appropriate. Returns whether the
    /// consumer-side side effect (delivery) happened.
    pub fn publish_event(&self, event: EventEnvelope) -> bool {
        self.hooks.emit(
            HookId::EventStreamPublish,
            vec![
                ("kind".into(), event.kind.clone()),
                ("sequence".into(), event.sequence.to_string()),
            ],
        );
        let mut guard = self.subscriptions.lock().expect("subscriptions poisoned");
        let state = match guard.get_mut(&event.subscription_id) {
            Some(s) => s,
            None => return false,
        };
        if state.delivery_mode != event.delivery_mode {
            // A producer that changes delivery mode mid-stream is a
            // contract violation; surface and drop.
            return false;
        }
        // AtLeastOnce: dedupe on idempotency_key.
        if let DeliveryMode::AtLeastOnce = state.delivery_mode {
            if let Some(key) = event.idempotency_key.as_ref() {
                let inserted = state.dedupe_keys.insert(key.as_str().to_string());
                if !inserted {
                    self.hooks.emit_bare(HookId::EventStreamDedupeHit);
                    return false;
                }
            }
        }
        // ExactlyOnce: gap-detect on sequence.
        if let DeliveryMode::ExactlyOnce = state.delivery_mode {
            if let Some(prev) = state.last_sequence {
                if event.sequence != prev.wrapping_add(1) {
                    self.hooks.emit(
                        HookId::EventStreamGapDetected,
                        vec![
                            ("expected".into(), prev.wrapping_add(1).to_string()),
                            ("observed".into(), event.sequence.to_string()),
                        ],
                    );
                }
            }
        }
        state.last_sequence = Some(event.sequence);
        self.hooks.emit(
            HookId::EventStreamConsume,
            vec![("kind".into(), event.kind.clone())],
        );
        true
    }

    /// Record a connection drop. In the in-process transport this is
    /// synthetic; a real transport hooks it on the socket-close path.
    pub fn drop_connection(&self) {
        self.hooks.emit_bare(HookId::RpcConnectionDrop);
    }
}

fn scope_matches(entry: &MethodEntry, scope: &WorkspaceScope) -> bool {
    use crate::manifest::ScopeKind;
    match entry.scope {
        ScopeKind::Workspace => matches!(scope, WorkspaceScope::Workspace(_)),
        ScopeKind::Global => scope.is_global(),
        ScopeKind::Either => true,
    }
}

fn default_unimplemented_handler() -> ServiceHandler {
    Arc::new(|req: &RequestEnvelope| {
        Err(ErrorPayload::new(
            ErrorClass::Internal,
            "rpc.handler_unregistered",
            format!("no handler registered for {}", req.method.as_str()),
            RetryHint::No,
        ))
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::envelope::{
        ActorClass, ContractVersion, IdempotencyKey, MethodName, ProducerId, ResponseResult,
    };
    use crate::errors::CancelReason;
    use crate::manifest::{ManifestDigest, MethodKind, ScopeKind};
    use crate::trace::TraceContext;

    fn cv(s: &str) -> ContractVersion {
        ContractVersion::new(s).expect("contract version")
    }

    fn basic_server_manifest() -> MethodManifest {
        let entry = MethodEntry {
            name: MethodName::new("vfs.read_metadata").unwrap(),
            kind: MethodKind::Unary,
            scope: ScopeKind::Workspace,
            actor_classes: vec![ActorClass::User, ActorClass::Command],
            contract_versions: vec![cv("1.0.0")],
            default_deadline_ns: 50_000_000,
            max_deadline_ns: 500_000_000,
            deadline_required: true,
            idempotency: Idempotency::NotApplicable,
            error_classes: vec![ErrorClass::Local, ErrorClass::Policy],
        };
        MethodManifest::new("vfs", cv("1.0.0"), ManifestDigest("sha256:0000".into()))
            .with_method(entry)
    }

    fn caller_manifest_matching() -> MethodManifest {
        basic_server_manifest()
    }

    fn new_transport() -> InProcessTransport {
        let server = basic_server_manifest();
        let caller = caller_manifest_matching();
        let entry = server.method("vfs.read_metadata").unwrap().clone();
        let mut t = InProcessTransport::new(server, caller, HookRegistry::new());
        t.register(ServiceRegistration {
            entry,
            handler: Arc::new(|req: &RequestEnvelope| {
                Ok(format!("hello {}", req.method.as_str()).into_bytes())
            }),
        });
        t.handshake();
        t
    }

    fn base_request(
        method: &str,
        trace: TraceContext,
        scope: WorkspaceScope,
        actor: ActorClass,
        deadline_ns: u64,
    ) -> RequestEnvelope {
        RequestEnvelope::new(
            1,
            MethodName::new(method).unwrap(),
            cv("1.0.0"),
            trace,
            scope,
            deadline_ns,
            42,
            actor,
            Vec::new(),
        )
    }

    #[test]
    fn unary_happy_path_dispatches_and_hooks_fire() {
        let t = new_transport();
        let trace = TraceContext::new_root(0x1111, 1, 0);
        let req = base_request(
            "vfs.read_metadata",
            trace,
            WorkspaceScope::workspace("ws"),
            ActorClass::User,
            1_000_000_000,
        );
        let resp = t.call(100, req);
        assert!(matches!(resp.result, ResponseResult::Ok(_)));
        assert!(resp.terminal);
        assert!(t.hooks().count(HookId::RpcRequestSend) >= 1);
        assert!(t.hooks().count(HookId::RpcRequestReceive) >= 1);
        assert!(t.hooks().count(HookId::RpcResponseDispatch) >= 1);
        // Happy path never classifies an error.
        assert_eq!(t.hooks().count(HookId::RpcErrorClassified), 0);
    }

    #[test]
    fn deadline_expiry_produces_deadline_exceeded() {
        let t = new_transport();
        let trace = TraceContext::new_root(0x2222, 2, 0);
        let req = base_request(
            "vfs.read_metadata",
            trace,
            WorkspaceScope::workspace("ws"),
            ActorClass::User,
            100,
        );
        let resp = t.call(500, req); // now > deadline
        match resp.result {
            ResponseResult::Err(err) => {
                assert_eq!(err.class, ErrorClass::DeadlineExceeded);
                assert_eq!(err.code, "rpc.deadline_expired");
            }
            _ => panic!("expected deadline_exceeded"),
        }
        assert!(t.hooks().count(HookId::RpcDeadlineExpired) >= 1);
    }

    #[test]
    fn caller_initiated_cancel_is_observed_once() {
        let t = new_transport();
        let trace = TraceContext::new_root(0x3333, 3, 0);
        let req = base_request(
            "vfs.read_metadata",
            trace,
            WorkspaceScope::workspace("ws"),
            ActorClass::User,
            1_000_000_000,
        );
        let cancel = CancelFrame::new(req.cancellation_channel, CancelReason::CallerInitiated);
        t.cancel(cancel);
        t.cancel(CancelFrame::new(
            req.cancellation_channel,
            CancelReason::CallerInitiated,
        ));
        // Repeated cancel: rpc_cancel_observed fires once.
        assert_eq!(t.hooks().count(HookId::RpcCancelObserved), 1);
        let resp = t.call(100, req);
        match resp.result {
            ResponseResult::Err(err) => assert_eq!(err.class, ErrorClass::Cancelled),
            _ => panic!("expected cancelled"),
        }
    }

    #[test]
    fn capability_intersection_rejects_unknown_method() {
        // Caller advertises a method the server does not know.
        let server = basic_server_manifest();
        let mut caller = basic_server_manifest();
        caller = caller.with_method(MethodEntry {
            name: MethodName::new("editor.reflow_with_shape_cache").unwrap(),
            kind: MethodKind::Unary,
            scope: ScopeKind::Workspace,
            actor_classes: vec![ActorClass::User],
            contract_versions: vec![cv("1.0.0")],
            default_deadline_ns: 0,
            max_deadline_ns: 0,
            deadline_required: false,
            idempotency: Idempotency::NotApplicable,
            error_classes: vec![ErrorClass::Local],
        });
        let mut t = InProcessTransport::new(server, caller, HookRegistry::new());
        t.handshake();
        // Intersection fires because caller advertised one extra.
        assert!(t.hooks().count(HookId::RpcCapabilityIntersection) >= 1);

        let trace = TraceContext::new_root(0x4444, 4, 0);
        let req = base_request(
            "editor.reflow_with_shape_cache",
            trace,
            WorkspaceScope::workspace("ws"),
            ActorClass::User,
            1_000_000_000,
        );
        let resp = t.call(100, req);
        match resp.result {
            ResponseResult::Err(err) => {
                assert_eq!(err.class, ErrorClass::Unavailable);
                assert_eq!(err.code, "rpc.method_unavailable");
            }
            _ => panic!("expected unavailable"),
        }
    }

    #[test]
    fn policy_denial_fires_for_wrong_actor_class() {
        let t = new_transport();
        let trace = TraceContext::new_root(0x5555, 5, 0);
        let req = base_request(
            "vfs.read_metadata",
            trace,
            WorkspaceScope::workspace("ws"),
            ActorClass::Ai,
            1_000_000_000,
        );
        let resp = t.call(100, req);
        match resp.result {
            ResponseResult::Err(err) => {
                assert_eq!(err.class, ErrorClass::Policy);
                assert_eq!(err.code, "rpc.actor_class_denied");
            }
            _ => panic!("expected policy denial"),
        }
    }

    #[test]
    fn policy_denial_fires_for_scope_mismatch() {
        let t = new_transport();
        let trace = TraceContext::new_root(0x6666, 6, 0);
        let req = base_request(
            "vfs.read_metadata",
            trace,
            WorkspaceScope::Global, // method is Workspace-scoped
            ActorClass::User,
            1_000_000_000,
        );
        let resp = t.call(100, req);
        match resp.result {
            ResponseResult::Err(err) => {
                assert_eq!(err.class, ErrorClass::Policy);
                assert_eq!(err.code, "rpc.scope_mismatch");
            }
            _ => panic!("expected scope mismatch"),
        }
    }

    #[test]
    fn deadline_required_is_enforced_for_methods_that_demand_it() {
        let t = new_transport();
        let trace = TraceContext::new_root(0x7777, 7, 0);
        let req = base_request(
            "vfs.read_metadata",
            trace,
            WorkspaceScope::workspace("ws"),
            ActorClass::User,
            0,
        );
        let resp = t.call(100, req);
        match resp.result {
            ResponseResult::Err(err) => {
                assert_eq!(err.class, ErrorClass::Local);
                assert_eq!(err.code, "rpc.deadline_required");
            }
            _ => panic!("expected local error"),
        }
    }

    #[test]
    fn at_least_once_event_stream_dedupes_on_key() {
        let t = new_transport();
        let sub = t.subscribe(DeliveryMode::AtLeastOnce);
        let trace = TraceContext::new_root(0x8888, 8, 0);
        let producer = ProducerId {
            service: "editor".into(),
            instance: "host-1".into(),
        };
        let key = IdempotencyKey::new("evt-1").unwrap();
        let evt = EventEnvelope::new(
            sub,
            1,
            "BufferSnapshotDelta",
            trace,
            WorkspaceScope::workspace("ws"),
            1,
            producer.clone(),
            Some(key.clone()),
            DeliveryMode::AtLeastOnce,
            vec![1, 2, 3],
        )
        .unwrap();
        assert!(t.publish_event(evt.clone()));
        // Retry delivers the same logical event a second time.
        assert!(!t.publish_event(evt));
        assert_eq!(t.hooks().count(HookId::EventStreamDedupeHit), 1);
        assert_eq!(t.hooks().count(HookId::EventStreamConsume), 1);
    }

    #[test]
    fn exactly_once_event_stream_detects_sequence_gap() {
        let t = new_transport();
        let sub = t.subscribe(DeliveryMode::ExactlyOnce);
        let trace = TraceContext::new_root(0x9999, 9, 0);
        let producer = ProducerId {
            service: "editor".into(),
            instance: "host-1".into(),
        };
        for seq in [1u64, 2u64, 5u64] {
            let evt = EventEnvelope::new(
                sub,
                seq,
                "BufferSnapshotDelta",
                trace,
                WorkspaceScope::workspace("ws"),
                1,
                producer.clone(),
                None,
                DeliveryMode::ExactlyOnce,
                vec![],
            )
            .unwrap();
            t.publish_event(evt);
        }
        assert_eq!(t.hooks().count(HookId::EventStreamGapDetected), 1);
        assert_eq!(t.hooks().count(HookId::EventStreamConsume), 3);
    }

    #[test]
    fn trace_child_preserves_trace_id_across_response() {
        let t = new_transport();
        let root = TraceContext::new_root(0xAAAA, 10, 0);
        let req = base_request(
            "vfs.read_metadata",
            root,
            WorkspaceScope::workspace("ws"),
            ActorClass::User,
            1_000_000_000,
        );
        let resp = t.call(100, req);
        assert_eq!(resp.trace.trace_id, root.trace_id);
        // Response carries a child span id (not identical to root).
        assert_ne!(resp.trace.span_id, root.span_id);
    }

    #[test]
    fn connection_drop_emits_hook() {
        let t = new_transport();
        t.drop_connection();
        assert!(t.hooks().count(HookId::RpcConnectionDrop) >= 1);
    }
}
