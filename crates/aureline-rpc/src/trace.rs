//! Trace context for every envelope.
//!
//! The shape is W3C-tracecontext-compatible so external collectors can
//! ingest traces without a translation layer. See ADR 0004 §Request
//! metadata — Trace IDs.

/// 8-bit trace flag bit positions. The integer value of the field is
/// the bitwise OR of whichever of these the producer set.
pub const TRACE_FLAG_SAMPLED: u8 = 1 << 0;
pub const TRACE_FLAG_DEBUG: u8 = 1 << 1;
pub const TRACE_FLAG_DO_NOT_RECORD: u8 = 1 << 2;

/// Trace fields carried by every envelope.
///
/// `parent_span_id` is `None` when the span is the root of its trace.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct TraceContext {
    pub trace_id: u128,
    pub span_id: u64,
    pub parent_span_id: Option<u64>,
    pub flags: u8,
}

impl TraceContext {
    /// Construct a root span on a new trace.
    pub fn new_root(trace_id: u128, span_id: u64, flags: u8) -> Self {
        Self {
            trace_id,
            span_id,
            parent_span_id: None,
            flags,
        }
    }

    /// Construct a child span under the current context. The new span
    /// inherits the trace id and flags; the caller supplies the new
    /// span id.
    pub fn child(&self, new_span_id: u64) -> Self {
        Self {
            trace_id: self.trace_id,
            span_id: new_span_id,
            parent_span_id: Some(self.span_id),
            flags: self.flags,
        }
    }

    pub fn is_sampled(&self) -> bool {
        self.flags & TRACE_FLAG_SAMPLED != 0
    }

    pub fn is_debug(&self) -> bool {
        self.flags & TRACE_FLAG_DEBUG != 0
    }

    pub fn do_not_record(&self) -> bool {
        self.flags & TRACE_FLAG_DO_NOT_RECORD != 0
    }
}
