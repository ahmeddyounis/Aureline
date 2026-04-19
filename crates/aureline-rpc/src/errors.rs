//! Frozen error taxonomy.
//!
//! Every typed error carries an [`ErrorClass`], a stable per-class
//! `code`, a human-readable `reason`, a [`RetryHint`], and an optional
//! producer [`TraceContext`] so the consumer can re-enter the trace.
//!
//! A class id MAY NOT be reused across concerns. A schema failure is
//! always `Local` (caller-side) or `Remote` (peer-side); it is never
//! `Internal`. See ADR 0004 §Error taxonomy.

use crate::trace::TraceContext;

/// Frozen error-class taxonomy.
///
/// Adding a new class is a decision-row-level change; adding a new
/// `code` inside an existing class is not.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ErrorClass {
    /// Caller-side contract violation or input error.
    Local,
    /// Peer-service logical error not caused by the caller.
    Remote,
    /// Denied by policy or workspace trust.
    Policy,
    /// Missing or misconfigured host environment.
    Environment,
    /// External provider / integration failure (LSP, DAP, cloud).
    Provider,
    /// Deadline elapsed before a terminal response.
    DeadlineExceeded,
    /// Caller-initiated cancellation honoured.
    Cancelled,
    /// Transport saturation, peer absent, handshake failure.
    Unavailable,
    /// Service bug or impossible-case assertion failure.
    Internal,
}

impl ErrorClass {
    /// Stable wire token for the class. These tokens also appear in
    /// the JSON Schema enum for `error_class`.
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Local => "local",
            Self::Remote => "remote",
            Self::Policy => "policy",
            Self::Environment => "environment",
            Self::Provider => "provider",
            Self::DeadlineExceeded => "deadline_exceeded",
            Self::Cancelled => "cancelled",
            Self::Unavailable => "unavailable",
            Self::Internal => "internal",
        }
    }
}

/// Per-error retry posture.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum RetryHint {
    /// The caller MUST NOT retry.
    No,
    /// The caller MAY retry after the given delay, in milliseconds.
    After { after_ms: u32 },
    /// The caller MUST reauthenticate before retrying.
    ReauthRequired,
}

/// Typed error body that rides inside [`crate::envelope::ResponseResult::Err`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ErrorPayload {
    pub class: ErrorClass,
    /// Per-class stable error code, dotted-lowercase
    /// (for example `vfs.path_denied`).
    pub code: String,
    pub reason: String,
    pub retry: RetryHint,
    /// Optional producer span context so consumers can re-enter the
    /// trace.
    pub span_context: Option<TraceContext>,
}

impl ErrorPayload {
    pub fn new(
        class: ErrorClass,
        code: impl Into<String>,
        reason: impl Into<String>,
        retry: RetryHint,
    ) -> Self {
        Self {
            class,
            code: code.into(),
            reason: reason.into(),
            retry,
            span_context: None,
        }
    }

    pub fn with_span(mut self, span: TraceContext) -> Self {
        self.span_context = Some(span);
        self
    }
}

/// Reason carried by a cancel frame. Repeated cancels on the same
/// cancellation channel are idempotent regardless of reason.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CancelReason {
    CallerInitiated,
    DeadlineExpired,
    PolicyDenied,
    Shutdown,
    ParentCancelled,
}

impl CancelReason {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::CallerInitiated => "caller_initiated",
            Self::DeadlineExpired => "deadline_expired",
            Self::PolicyDenied => "policy_denied",
            Self::Shutdown => "shutdown",
            Self::ParentCancelled => "parent_cancelled",
        }
    }
}
