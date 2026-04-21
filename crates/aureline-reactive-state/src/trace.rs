//! Invalidation trace records.
//!
//! Every store call appends one [`TraceEvent`] to a scenario's
//! trace. A scenario's trace is the artifact under
//! `artifacts/state/invalidation_trace_examples/` that answers
//! the ADR 0005 acceptance question "why is this projection
//! fresh, stale, partial, or recomputing?". The trace is
//! byte-stable: synthetic monotonic ticks replace wall clock,
//! and every string is computed deterministically from the
//! scenario script.

use crate::envelope::{
    write_envelope, write_json_value, write_key, write_kv_string, write_kv_u64,
    write_string_literal, BackpressureMode, JsonValue, ScopeRef, SubscriptionEnvelope,
};

/// A single row on the trace. Rows are either a
/// subscription-lifecycle event (subscribe, frame emit, frame
/// apply, terminate) or a supporting observation (coalesce,
/// replay begin / end, delta-gap detection).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TraceEvent {
    Subscribe {
        tick: u64,
        subscription_id: u64,
        query_family: String,
        scope_ref: ScopeRef,
        backpressure_mode: BackpressureMode,
    },
    FrameEmit {
        tick: u64,
        hook_id: &'static str,
        envelope: SubscriptionEnvelope,
        observation: ConsumerObservation,
    },
    Note {
        tick: u64,
        hook_id: &'static str,
        message: String,
    },
}

impl TraceEvent {
    pub fn subscribe(
        tick: u64,
        subscription_id: u64,
        query_family: &str,
        scope_ref: ScopeRef,
        backpressure_mode: BackpressureMode,
    ) -> Self {
        TraceEvent::Subscribe {
            tick,
            subscription_id,
            query_family: query_family.to_owned(),
            scope_ref,
            backpressure_mode,
        }
    }

    pub fn frame_emit(
        tick: u64,
        hook_id: &'static str,
        envelope: SubscriptionEnvelope,
        observation: ConsumerObservation,
    ) -> Self {
        TraceEvent::FrameEmit {
            tick,
            hook_id,
            envelope,
            observation,
        }
    }

    pub fn note(tick: u64, hook_id: &'static str, message: String) -> Self {
        TraceEvent::Note {
            tick,
            hook_id,
            message,
        }
    }

    pub fn tick(&self) -> u64 {
        match self {
            TraceEvent::Subscribe { tick, .. }
            | TraceEvent::FrameEmit { tick, .. }
            | TraceEvent::Note { tick, .. } => *tick,
        }
    }

    pub fn hook_id(&self) -> &'static str {
        match self {
            TraceEvent::Subscribe { .. } => "subscription_subscribe",
            TraceEvent::FrameEmit { hook_id, .. } => hook_id,
            TraceEvent::Note { hook_id, .. } => hook_id,
        }
    }
}

/// Consumer-side observation for one frame. Captured at the
/// point the consumer applies (or rejects) the frame so the
/// trace answers "what the consumer saw at this tick".
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct ConsumerObservation {
    pub frame_class: String,
    pub freshness: String,
    pub completeness: String,
    pub snapshot_epoch: u64,
    pub delta_seq: u64,
    pub stale_reason: Option<String>,
    pub terminal_reason: Option<String>,
    pub reviewer_notes: Vec<String>,
}

impl ConsumerObservation {
    pub fn apply(envelope: &SubscriptionEnvelope) -> Self {
        ConsumerObservation {
            frame_class: envelope.frame_class.as_str().to_owned(),
            freshness: envelope.freshness.as_str().to_owned(),
            completeness: envelope.completeness.as_str().to_owned(),
            snapshot_epoch: envelope.snapshot_epoch,
            delta_seq: envelope.delta_seq,
            stale_reason: envelope
                .invalidation
                .as_ref()
                .map(|i| i.stale_reason.as_str().to_owned()),
            terminal_reason: envelope.terminal_reason.map(|t| t.as_str().to_owned()),
            reviewer_notes: Vec::new(),
        }
    }
}

/// Render a trace event as a JSON object. Used by the harness
/// when emitting the per-scenario invalidation trace record.
pub fn trace_event_to_json(ev: &TraceEvent, indent: usize) -> String {
    let mut out = String::new();
    write_trace_event(&mut out, ev, indent);
    out
}

pub(crate) fn write_trace_event(out: &mut String, ev: &TraceEvent, indent: usize) {
    out.push('{');
    let ind = indent + 1;
    match ev {
        TraceEvent::Subscribe {
            tick,
            subscription_id,
            query_family,
            scope_ref,
            backpressure_mode,
        } => {
            newline_indent(out, ind);
            write_kv_string(out, "event_kind", "subscribe");
            comma_nl(out, ind);
            write_kv_u64(out, "tick", *tick);
            comma_nl(out, ind);
            write_kv_string(out, "hook_id", ev.hook_id());
            comma_nl(out, ind);
            write_kv_u64(out, "subscription_id", *subscription_id);
            comma_nl(out, ind);
            write_kv_string(out, "query_family", query_family);
            comma_nl(out, ind);
            write_key(out, "scope_ref");
            let scope_json = JsonValue::obj(vec![
                ("class", JsonValue::str(scope_ref.class.as_str())),
                ("id", JsonValue::str(&scope_ref.id)),
            ]);
            write_json_value(out, &scope_json, ind);
            comma_nl(out, ind);
            write_kv_string(out, "backpressure_mode", backpressure_mode.as_str());
            newline_indent(out, indent);
        }
        TraceEvent::FrameEmit {
            tick,
            hook_id,
            envelope,
            observation,
        } => {
            newline_indent(out, ind);
            write_kv_string(out, "event_kind", "frame_emit");
            comma_nl(out, ind);
            write_kv_u64(out, "tick", *tick);
            comma_nl(out, ind);
            write_kv_string(out, "hook_id", hook_id);
            comma_nl(out, ind);
            write_key(out, "envelope");
            write_envelope(out, envelope, ind);
            comma_nl(out, ind);
            write_key(out, "consumer_observation");
            write_consumer_observation(out, observation, ind);
            newline_indent(out, indent);
        }
        TraceEvent::Note {
            tick,
            hook_id,
            message,
        } => {
            newline_indent(out, ind);
            write_kv_string(out, "event_kind", "note");
            comma_nl(out, ind);
            write_kv_u64(out, "tick", *tick);
            comma_nl(out, ind);
            write_kv_string(out, "hook_id", hook_id);
            comma_nl(out, ind);
            write_kv_string(out, "message", message);
            newline_indent(out, indent);
        }
    }
    out.push('}');
}

fn write_consumer_observation(out: &mut String, obs: &ConsumerObservation, indent: usize) {
    out.push('{');
    let ind = indent + 1;
    newline_indent(out, ind);
    write_kv_string(out, "frame_class", &obs.frame_class);
    comma_nl(out, ind);
    write_kv_string(out, "freshness", &obs.freshness);
    comma_nl(out, ind);
    write_kv_string(out, "completeness", &obs.completeness);
    comma_nl(out, ind);
    write_kv_u64(out, "snapshot_epoch", obs.snapshot_epoch);
    comma_nl(out, ind);
    write_kv_u64(out, "delta_seq", obs.delta_seq);
    if let Some(reason) = &obs.stale_reason {
        comma_nl(out, ind);
        write_kv_string(out, "stale_reason", reason);
    }
    if let Some(reason) = &obs.terminal_reason {
        comma_nl(out, ind);
        write_kv_string(out, "terminal_reason", reason);
    }
    comma_nl(out, ind);
    write_key(out, "reviewer_notes");
    out.push('[');
    for (i, note) in obs.reviewer_notes.iter().enumerate() {
        if i > 0 {
            out.push(',');
        }
        newline_indent(out, ind + 1);
        write_string_literal(out, note);
    }
    if !obs.reviewer_notes.is_empty() {
        newline_indent(out, ind);
    }
    out.push(']');
    newline_indent(out, indent);
    out.push('}');
}

pub(crate) fn newline_indent(out: &mut String, indent: usize) {
    out.push('\n');
    for _ in 0..indent {
        out.push_str("  ");
    }
}

pub(crate) fn comma_nl(out: &mut String, indent: usize) {
    out.push(',');
    newline_indent(out, indent);
}

/// Utility consumers of the trace module use when they want to
/// render the full trace as a single JSON object (array of
/// events plus summary fields).
pub fn trace_to_json(tick_count: u64, events: &[TraceEvent]) -> String {
    let mut out = String::new();
    out.push('{');
    let ind = 1;
    newline_indent(&mut out, ind);
    write_kv_u64(&mut out, "total_ticks", tick_count);
    comma_nl(&mut out, ind);
    write_kv_u64(&mut out, "event_count", events.len() as u64);
    comma_nl(&mut out, ind);
    write_key(&mut out, "events");
    out.push('[');
    for (i, ev) in events.iter().enumerate() {
        if i > 0 {
            out.push(',');
        }
        newline_indent(&mut out, ind + 1);
        write_trace_event(&mut out, ev, ind + 1);
    }
    if !events.is_empty() {
        newline_indent(&mut out, ind);
    }
    out.push(']');
    newline_indent(&mut out, 0);
    out.push('}');
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::envelope::{
        AuthorityClass, Completeness, DerivationClass, FrameClass, Freshness, ProducerRef,
        ScopeClass, ViewClass, SUBSCRIPTION_SCHEMA_VERSION,
    };

    fn sample_env() -> SubscriptionEnvelope {
        SubscriptionEnvelope {
            subscription_schema_version: SUBSCRIPTION_SCHEMA_VERSION,
            subscription_id: 42,
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
            payload: None,
        }
    }

    #[test]
    fn frame_emit_trace_round_trips_ticks() {
        let env = sample_env();
        let ev = TraceEvent::frame_emit(
            1,
            "subscription_snapshot_emit",
            env.clone(),
            ConsumerObservation::apply(&env),
        );
        let json = trace_event_to_json(&ev, 0);
        assert!(json.contains("\"event_kind\": \"frame_emit\""));
        assert!(json.contains("\"tick\": 1"));
        assert!(json.contains("\"subscription_id\": 42"));
        assert!(json.contains("\"snapshot_epoch\": 1"));
    }
}
