//! Protected-hot-path hook ids and an in-memory observation registry.
//!
//! Hook ids are frozen vocabulary; lanes instrument them without
//! inventing synonyms. The registry in this module is an observability
//! aid for tests, the shell spike, and the benchmark lab — production
//! instrumentation attaches the same hook ids to the telemetry crate.
//! See ADR 0004 §Protected-hot-path hooks.

use std::sync::{Arc, Mutex};

/// Frozen hook vocabulary. Adding a new hook is a decision-row-level
/// change so every instrumented lane observes the same names.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum HookId {
    RpcHandshakeComplete,
    RpcCapabilityIntersection,
    RpcRequestSend,
    RpcRequestReceive,
    RpcResponseDispatch,
    RpcProgressEmit,
    RpcCancelObserved,
    RpcDeadlineExpired,
    RpcQueueSaturation,
    RpcErrorClassified,
    RpcIdleKeepalive,
    EventStreamPublish,
    EventStreamConsume,
    EventStreamGapDetected,
    EventStreamDedupeHit,
    RpcConnectionDrop,
}

impl HookId {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::RpcHandshakeComplete => "rpc_handshake_complete",
            Self::RpcCapabilityIntersection => "rpc_capability_intersection",
            Self::RpcRequestSend => "rpc_request_send",
            Self::RpcRequestReceive => "rpc_request_receive",
            Self::RpcResponseDispatch => "rpc_response_dispatch",
            Self::RpcProgressEmit => "rpc_progress_emit",
            Self::RpcCancelObserved => "rpc_cancel_observed",
            Self::RpcDeadlineExpired => "rpc_deadline_expired",
            Self::RpcQueueSaturation => "rpc_queue_saturation",
            Self::RpcErrorClassified => "rpc_error_classified",
            Self::RpcIdleKeepalive => "rpc_idle_keepalive",
            Self::EventStreamPublish => "event_stream_publish",
            Self::EventStreamConsume => "event_stream_consume",
            Self::EventStreamGapDetected => "event_stream_gap_detected",
            Self::EventStreamDedupeHit => "event_stream_dedupe_hit",
            Self::RpcConnectionDrop => "rpc_connection_drop",
        }
    }

    /// Whether this hook sits on a protected hot-path budget. Matches
    /// the ADR's §Protected-hot-path hooks table.
    pub fn is_hot_path(&self) -> bool {
        matches!(
            self,
            Self::RpcHandshakeComplete
                | Self::RpcRequestSend
                | Self::RpcRequestReceive
                | Self::RpcResponseDispatch
                | Self::RpcCancelObserved
                | Self::RpcDeadlineExpired
                | Self::RpcQueueSaturation
                | Self::EventStreamPublish
                | Self::EventStreamConsume
                | Self::EventStreamGapDetected
                | Self::RpcConnectionDrop
        )
    }
}

/// A single hook emission captured by the registry.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HookObservation {
    pub hook: HookId,
    pub attributes: Vec<(String, String)>,
}

/// Cloneable, thread-safe registry that stores hook emissions. Used by
/// tests and early instrumentation; production code will route the
/// same hook ids through `aureline-telemetry`.
#[derive(Debug, Clone, Default)]
pub struct HookRegistry {
    inner: Arc<Mutex<Vec<HookObservation>>>,
}

impl HookRegistry {
    pub fn new() -> Self {
        Self {
            inner: Arc::new(Mutex::new(Vec::new())),
        }
    }

    pub fn emit(&self, hook: HookId, attributes: Vec<(String, String)>) {
        let mut guard = self.inner.lock().expect("hook registry poisoned");
        guard.push(HookObservation { hook, attributes });
    }

    pub fn emit_bare(&self, hook: HookId) {
        self.emit(hook, Vec::new());
    }

    pub fn snapshot(&self) -> Vec<HookObservation> {
        self.inner.lock().expect("hook registry poisoned").clone()
    }

    pub fn count(&self, hook: HookId) -> usize {
        self.inner
            .lock()
            .expect("hook registry poisoned")
            .iter()
            .filter(|o| o.hook == hook)
            .count()
    }

    pub fn clear(&self) {
        self.inner.lock().expect("hook registry poisoned").clear();
    }
}
