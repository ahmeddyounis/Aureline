//! Reactive store: the minimal object that routes frames from
//! producers to consumers under the ADR 0005 subscription
//! contract.
//!
//! The store is deliberately synchronous, deterministic, and
//! single-threaded. It is not a runtime; it is the contract
//! enforcer that proves the envelope vocabulary and the lifecycle
//! rules are actually observable. Every method here fires a hook
//! the ADR names, records a [`trace::TraceEvent`] the harness
//! emits, and refuses to silently paper over a rule violation.
//!
//! The store enforces:
//!
//! - Subscribe allocates a monotonic `subscription_id`.
//! - Snapshot frames reset `delta_seq` to 0 and bump
//!   `snapshot_epoch`.
//! - Delta frames strictly increase `delta_seq` inside the
//!   current `snapshot_epoch`. Out-of-order or duplicate deltas
//!   are rejected; the store demotes the projection by emitting
//!   a `resync_required` frame with `causality_lost` rather than
//!   silently dropping the gap.
//! - Derived frames MAY NOT claim `freshness = authoritative`.
//!   Attempting to do so is a contract violation the store flags
//!   verbatim in the trace.
//! - Replayed frames never advance the live `snapshot_epoch`.
//! - Imported frames never claim authority and never merge into
//!   authoritative state.
//! - Backpressure switches to `snapshot_required` force a
//!   `resync_required` frame before the next snapshot.
//!
//! Every frame the store emits is both routed to the subscribed
//! consumer (who validates the lifecycle against its own local
//! projection) and recorded on the trace. The emitted
//! [`envelope::SubscriptionEnvelope`] is what would ride the
//! ADR-0004 event-stream envelope on the wire.

use crate::envelope::{
    AuthorityClass, BackpressureMode, Completeness, DerivationClass, FrameClass, Freshness,
    Invalidation, JsonValue, ProducerRef, ScopeRef, StaleReason, SubscriptionEnvelope,
    TerminalReason, ViewClass, SUBSCRIPTION_SCHEMA_VERSION,
};
#[cfg(test)]
use crate::envelope::InputDigest;
use crate::hooks::HookCounters;
use crate::trace::{ConsumerObservation, TraceEvent};

/// Shape of the sample payload emitted by the prototype
/// producers. The production envelope's `payload` is typed per
/// query-family (ADR 0005); the prototype uses a small struct
/// so the committed fixtures stay reviewable.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct SamplePayload {
    pub summary: String,
    pub entry_count: u64,
    /// `partial` / `full` coverage numerator and denominator.
    /// The pair is emitted verbatim so consumers can render a
    /// progress badge.
    pub coverage_ready: u64,
    pub coverage_total: u64,
    /// Optional extra key/value string pairs that the scenario
    /// wants to pin into the frame body (for example, the
    /// upstream digest a derived diagnostics frame references).
    pub detail_lines: Vec<(String, String)>,
}

impl SamplePayload {
    pub fn to_json(&self) -> JsonValue {
        let mut obj: Vec<(&str, JsonValue)> = vec![
            ("summary", JsonValue::str(&self.summary)),
            ("entry_count", JsonValue::u(self.entry_count)),
            (
                "coverage",
                JsonValue::obj(vec![
                    ("ready", JsonValue::u(self.coverage_ready)),
                    ("total", JsonValue::u(self.coverage_total)),
                ]),
            ),
        ];
        if !self.detail_lines.is_empty() {
            let detail = JsonValue::Object(
                self.detail_lines
                    .iter()
                    .map(|(k, v)| (k.clone(), JsonValue::str(v)))
                    .collect(),
            );
            obj.push(("detail", detail));
        }
        JsonValue::obj(obj)
    }
}

/// A producer registration. The prototype treats a producer as
/// an opaque emit source named by `(query_family, scope_ref)`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Producer {
    pub query_family: String,
    pub scope_ref: ScopeRef,
    pub authority_class: AuthorityClass,
    pub derivation_class: DerivationClass,
    pub view_class: ViewClass,
    pub backpressure_mode: BackpressureMode,
    pub producer_refs: Vec<ProducerRef>,
}

/// Live producer state tracked by the store.
#[derive(Debug, Clone, PartialEq, Eq)]
struct ProducerState {
    producer: Producer,
    snapshot_epoch: u64,
    last_delta_seq: u64,
    last_freshness: Freshness,
    last_completeness: Completeness,
    /// `true` once a `resync_required` frame was emitted for the
    /// current epoch and the store is awaiting the next snapshot.
    awaiting_snapshot: bool,
    /// `true` once a `terminal` frame has been written. No
    /// further frames may be emitted on this subscription.
    terminated: bool,
    /// Active subscriber, if any. The prototype is single-
    /// consumer per subscription_id; that matches the ADR's
    /// unary "subscribe call" + event-stream model.
    subscription_id: Option<u64>,
}

/// Live subscriber projection. Consumers observe frames via
/// [`ReactiveStore::deliver`] and validate lifecycle invariants
/// locally.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct ConsumerProjection {
    pub subscription_id: u64,
    pub query_family: String,
    pub scope_ref: ScopeRef,
    pub backpressure_mode: BackpressureMode,
    pub last_snapshot_epoch: u64,
    pub last_delta_seq: u64,
    pub last_freshness: Freshness,
    pub last_completeness: Completeness,
    pub is_stale: bool,
    pub is_terminal: bool,
    pub ever_replayed: bool,
    pub ever_imported: bool,
}

/// An emission result: the envelope the producer wrote plus the
/// consumer observation the store routed the frame to.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Emission {
    pub envelope: SubscriptionEnvelope,
    pub consumer_observation: ConsumerObservation,
}

/// The reactive store.
#[derive(Debug)]
pub struct ReactiveStore {
    next_subscription_id: u64,
    producers: Vec<ProducerState>,
    consumers: Vec<ConsumerProjection>,
    pub hooks: HookCounters,
    pub trace: Vec<TraceEvent>,
    /// Monotonic tick, increments on every store call. Synthetic
    /// so artifacts stay byte-stable across hosts (no wall
    /// clock).
    tick: u64,
}

impl ReactiveStore {
    pub fn new() -> Self {
        Self {
            next_subscription_id: 100,
            producers: Vec::new(),
            consumers: Vec::new(),
            hooks: HookCounters::default(),
            trace: Vec::new(),
            tick: 0,
        }
    }

    fn advance_tick(&mut self) -> u64 {
        self.tick += 1;
        self.tick
    }

    /// Register a producer. The store remembers its authority /
    /// derivation / view / backpressure posture so every emit
    /// call can copy the right fields into the envelope without
    /// the caller having to re-state them.
    pub fn register_producer(&mut self, producer: Producer) {
        let state = ProducerState {
            producer,
            snapshot_epoch: 0,
            last_delta_seq: 0,
            last_freshness: Freshness::Warming,
            last_completeness: Completeness::Unloaded,
            awaiting_snapshot: true,
            terminated: false,
            subscription_id: None,
        };
        self.producers.push(state);
    }

    /// Subscribe to a registered producer. Allocates a
    /// `subscription_id`, records the declared backpressure
    /// mode, and fires the `subscription_subscribe` hot-path
    /// hook. The subscription is considered established when
    /// the first frame is emitted, not on ack alone (ADR 0005
    /// § Subscription lifecycle step 1).
    pub fn subscribe(
        &mut self,
        query_family: &str,
        scope_ref: &ScopeRef,
        backpressure_mode: BackpressureMode,
    ) -> Result<u64, StoreError> {
        let producer_idx = self.find_producer(query_family, scope_ref)?;
        if self.producers[producer_idx].subscription_id.is_some() {
            return Err(StoreError::AlreadySubscribed);
        }
        let subscription_id = self.next_subscription_id;
        self.next_subscription_id += 1;
        self.producers[producer_idx].subscription_id = Some(subscription_id);
        // Consumers declare the backpressure mode they can
        // accept; the producer records it so later switches are
        // observable.
        self.producers[producer_idx].producer.backpressure_mode = backpressure_mode;

        self.consumers.push(ConsumerProjection {
            subscription_id,
            query_family: query_family.to_owned(),
            scope_ref: scope_ref.clone(),
            backpressure_mode,
            last_snapshot_epoch: 0,
            last_delta_seq: 0,
            last_freshness: Freshness::Warming,
            last_completeness: Completeness::Unloaded,
            is_stale: false,
            is_terminal: false,
            ever_replayed: false,
            ever_imported: false,
        });

        self.hooks.subscription_subscribe += 1;
        let tick = self.advance_tick();
        self.trace.push(TraceEvent::subscribe(
            tick,
            subscription_id,
            query_family,
            scope_ref.clone(),
            backpressure_mode,
        ));

        Ok(subscription_id)
    }

    /// Emit a snapshot frame. Bumps `snapshot_epoch` and resets
    /// `delta_seq` to 0 per ADR 0005 § Snapshot frame.
    pub fn emit_snapshot(
        &mut self,
        subscription_id: u64,
        freshness: Freshness,
        completeness: Completeness,
        payload: SamplePayload,
        invalidation: Option<Invalidation>,
    ) -> Result<Emission, StoreError> {
        self.emit_snapshot_with_refs(
            subscription_id,
            freshness,
            completeness,
            payload,
            invalidation,
            None,
        )
    }

    /// Emit a snapshot frame, optionally replacing the
    /// producer's advertised `producer_refs` (for example, when
    /// a derived producer publishes a new `derivation_epoch` or
    /// a new `input_digests` set).
    pub fn emit_snapshot_with_refs(
        &mut self,
        subscription_id: u64,
        freshness: Freshness,
        completeness: Completeness,
        payload: SamplePayload,
        invalidation: Option<Invalidation>,
        new_producer_refs: Option<Vec<ProducerRef>>,
    ) -> Result<Emission, StoreError> {
        let idx = self.find_producer_by_subscription(subscription_id)?;
        self.assert_not_terminated(idx)?;

        // Enforce derived ≠ authoritative rule (ADR 0005
        // § Authoritative versus derived state).
        self.assert_derivation_freshness_legal(idx, freshness)?;

        if let Some(refs) = new_producer_refs {
            self.producers[idx].producer.producer_refs = refs;
        }
        let state = &mut self.producers[idx];
        state.snapshot_epoch += 1;
        state.last_delta_seq = 0;
        state.last_freshness = freshness;
        state.last_completeness = completeness;
        state.awaiting_snapshot = false;

        let envelope = self.build_envelope(
            idx,
            subscription_id,
            FrameClass::Snapshot,
            0,
            freshness,
            completeness,
            invalidation,
            None,
            Some(payload.to_json()),
        );
        self.hooks.subscription_snapshot_emit += 1;

        let obs = self.deliver(subscription_id, &envelope)?;
        let tick = self.advance_tick();
        self.trace.push(TraceEvent::frame_emit(
            tick,
            "subscription_snapshot_emit",
            envelope.clone(),
            obs.clone(),
        ));
        Ok(Emission {
            envelope,
            consumer_observation: obs,
        })
    }

    /// Emit a delta frame. Bumps `delta_seq` by 1. Rejects the
    /// emission if the subscription is awaiting a snapshot.
    pub fn emit_delta(
        &mut self,
        subscription_id: u64,
        freshness: Freshness,
        completeness: Completeness,
        payload: SamplePayload,
        invalidation: Option<Invalidation>,
    ) -> Result<Emission, StoreError> {
        let idx = self.find_producer_by_subscription(subscription_id)?;
        self.assert_not_terminated(idx)?;
        self.assert_derivation_freshness_legal(idx, freshness)?;
        if self.producers[idx].awaiting_snapshot {
            return Err(StoreError::DeltaBeforeSnapshot);
        }
        let state = &mut self.producers[idx];
        state.last_delta_seq += 1;
        state.last_freshness = freshness;
        state.last_completeness = completeness;
        let seq = state.last_delta_seq;

        let envelope = self.build_envelope(
            idx,
            subscription_id,
            FrameClass::Delta,
            seq,
            freshness,
            completeness,
            invalidation,
            None,
            Some(payload.to_json()),
        );
        self.hooks.subscription_delta_emit += 1;

        let obs = self.deliver(subscription_id, &envelope)?;
        self.hooks.subscription_delta_apply += 1;
        let tick = self.advance_tick();
        self.trace.push(TraceEvent::frame_emit(
            tick,
            "subscription_delta_emit",
            envelope.clone(),
            obs.clone(),
        ));
        Ok(Emission {
            envelope,
            consumer_observation: obs,
        })
    }

    /// Emit a `resync_required` frame. Demotes the subscription
    /// to `freshness = stale`; the next snapshot bumps
    /// `snapshot_epoch`.
    pub fn emit_resync_required(
        &mut self,
        subscription_id: u64,
        stale_reason: StaleReason,
        caused_by: Option<crate::envelope::CausedBy>,
        completeness: Completeness,
    ) -> Result<Emission, StoreError> {
        let idx = self.find_producer_by_subscription(subscription_id)?;
        self.assert_not_terminated(idx)?;
        let last_delta_seq = {
            let state = &mut self.producers[idx];
            state.awaiting_snapshot = true;
            state.last_freshness = Freshness::Stale;
            state.last_completeness = completeness;
            state.last_delta_seq
        };

        let envelope = self.build_envelope(
            idx,
            subscription_id,
            FrameClass::ResyncRequired,
            last_delta_seq,
            Freshness::Stale,
            completeness,
            Some(Invalidation {
                stale_reason,
                caused_by,
            }),
            None,
            None,
        );
        self.hooks.subscription_resync_required_emit += 1;

        let obs = self.deliver(subscription_id, &envelope)?;
        let tick = self.advance_tick();
        self.trace.push(TraceEvent::frame_emit(
            tick,
            "subscription_resync_required_emit",
            envelope.clone(),
            obs.clone(),
        ));
        Ok(Emission {
            envelope,
            consumer_observation: obs,
        })
    }

    /// Emit a terminal frame. No further frames may be emitted
    /// on this subscription after this call.
    pub fn emit_terminal(
        &mut self,
        subscription_id: u64,
        terminal_reason: TerminalReason,
        freshness: Freshness,
        completeness: Completeness,
        invalidation: Option<Invalidation>,
    ) -> Result<Emission, StoreError> {
        let idx = self.find_producer_by_subscription(subscription_id)?;
        self.assert_not_terminated(idx)?;
        let last_delta_seq = {
            let state = &mut self.producers[idx];
            state.terminated = true;
            state.last_freshness = freshness;
            state.last_completeness = completeness;
            state.last_delta_seq
        };

        let envelope = self.build_envelope(
            idx,
            subscription_id,
            FrameClass::Terminal,
            last_delta_seq,
            freshness,
            completeness,
            invalidation,
            Some(terminal_reason),
            None,
        );
        self.hooks.subscription_terminate += 1;

        let obs = self.deliver(subscription_id, &envelope)?;
        let tick = self.advance_tick();
        self.trace.push(TraceEvent::frame_emit(
            tick,
            "subscription_terminate",
            envelope.clone(),
            obs.clone(),
        ));
        Ok(Emission {
            envelope,
            consumer_observation: obs,
        })
    }

    /// Producer-side coalesce hook: fired when the producer
    /// collapses `count` deltas into one because the consumer's
    /// declared backpressure mode is `coalesced`.
    pub fn record_coalesce(&mut self, count: u64) {
        self.hooks.subscription_backpressure_coalesce += count;
        let tick = self.advance_tick();
        self.trace.push(TraceEvent::note(
            tick,
            "subscription_backpressure_coalesce",
            format!("producer coalesced {count} pending deltas"),
        ));
    }

    /// Consumer-side request: switch backpressure to
    /// `snapshot_required`. Forces a `resync_required` on the
    /// producer per ADR 0005 § Subscription envelope fields.
    pub fn request_snapshot_required_switch(
        &mut self,
        subscription_id: u64,
    ) -> Result<Emission, StoreError> {
        let idx = self.find_producer_by_subscription(subscription_id)?;
        self.assert_not_terminated(idx)?;
        self.producers[idx].producer.backpressure_mode = BackpressureMode::SnapshotRequired;
        if let Some(consumer) = self
            .consumers
            .iter_mut()
            .find(|c| c.subscription_id == subscription_id)
        {
            consumer.backpressure_mode = BackpressureMode::SnapshotRequired;
        }
        self.hooks.subscription_snapshot_required_switch += 1;
        let tick = self.advance_tick();
        self.trace.push(TraceEvent::note(
            tick,
            "subscription_snapshot_required_switch",
            "consumer requested backpressure_mode = snapshot_required".to_owned(),
        ));
        self.emit_resync_required(
            subscription_id,
            StaleReason::CausalityLost,
            Some(crate::envelope::CausedBy {
                note: Some(
                    "backpressure switched to snapshot_required; causal continuity abandoned"
                        .to_owned(),
                ),
                ..crate::envelope::CausedBy::default()
            }),
            Completeness::Partial,
        )
    }

    /// Detect a delta gap. Used by scenarios that script an
    /// out-of-order delivery. Emits `resync_required` with
    /// `causality_lost`.
    pub fn report_delta_gap(
        &mut self,
        subscription_id: u64,
        observed_seq: u64,
        expected_seq: u64,
    ) -> Result<Emission, StoreError> {
        let tick = self.advance_tick();
        self.trace.push(TraceEvent::note(
            tick,
            "delta_gap_detected",
            format!(
                "consumer observed delta_seq={observed_seq}, expected {expected_seq}; escalating"
            ),
        ));
        self.emit_resync_required(
            subscription_id,
            StaleReason::CausalityLost,
            Some(crate::envelope::CausedBy {
                note: Some(format!(
                    "delta gap: observed={observed_seq}, expected={expected_seq}"
                )),
                ..crate::envelope::CausedBy::default()
            }),
            Completeness::Partial,
        )
    }

    /// Attach an imported-history frame. The producer MUST be
    /// registered with `derivation_class = derived`; the frame
    /// carries `freshness = imported` and never advances the
    /// live snapshot_epoch.
    pub fn attach_imported_snapshot(
        &mut self,
        subscription_id: u64,
        payload: SamplePayload,
        source: String,
    ) -> Result<Emission, StoreError> {
        let idx = self.find_producer_by_subscription(subscription_id)?;
        self.assert_not_terminated(idx)?;
        // Imported frames MUST be derived and MUST carry a
        // `source`; the store enforces both.
        if matches!(
            self.producers[idx].producer.derivation_class,
            DerivationClass::Authoritative
        ) {
            return Err(StoreError::ImportedRequiresDerived);
        }
        // Imported frames are a separate snapshot lineage; they
        // do not bump the live snapshot_epoch. We emit them at
        // the current epoch / last_delta_seq but with freshness
        // imported.
        let (epoch_for_frame, seq_for_frame) = {
            let state = &self.producers[idx];
            (state.snapshot_epoch.max(1), state.last_delta_seq)
        };
        // Ensure every imported producer_ref carries a source.
        if !self.producers[idx]
            .producer
            .producer_refs
            .iter()
            .any(|r| r.source.is_some())
        {
            if let Some(first) = self.producers[idx].producer.producer_refs.first_mut() {
                first.source = Some(source.clone());
            }
        }

        let envelope = self.build_envelope_with_epoch(
            idx,
            subscription_id,
            FrameClass::Snapshot,
            epoch_for_frame,
            seq_for_frame,
            Freshness::Imported,
            Completeness::Full,
            Some(Invalidation {
                stale_reason: StaleReason::ImportedFromExternal,
                caused_by: Some(crate::envelope::CausedBy {
                    note: Some(format!("imported from: {source}")),
                    ..crate::envelope::CausedBy::default()
                }),
            }),
            None,
            Some(payload.to_json()),
        );
        self.hooks.subscription_imported_attach += 1;
        // Imported frame is non-authoritative — still a
        // freshness downgrade for observability.
        self.hooks.subscription_freshness_downgrade += 1;

        let obs = self.deliver(subscription_id, &envelope)?;
        let tick = self.advance_tick();
        self.trace.push(TraceEvent::frame_emit(
            tick,
            "subscription_imported_attach",
            envelope.clone(),
            obs.clone(),
        ));
        Ok(Emission {
            envelope,
            consumer_observation: obs,
        })
    }

    /// Begin a replay session on a subscription. Subsequent
    /// frames emitted via `emit_replayed_delta` carry
    /// `freshness = replayed` and do not advance the live
    /// `snapshot_epoch`.
    pub fn begin_replay(&mut self, subscription_id: u64) -> Result<(), StoreError> {
        let _idx = self.find_producer_by_subscription(subscription_id)?;
        self.hooks.subscription_replay_begin += 1;
        let tick = self.advance_tick();
        self.trace.push(TraceEvent::note(
            tick,
            "subscription_replay_begin",
            "replay session started; live epoch will not advance".to_owned(),
        ));
        Ok(())
    }

    /// Emit a replayed delta frame. Freshness is `replayed`; the
    /// frame does not advance the producer's live `delta_seq`
    /// counter (a fresh counter local to the replay session is
    /// used so the live projection is not corrupted).
    pub fn emit_replayed_delta(
        &mut self,
        subscription_id: u64,
        replay_seq: u64,
        payload: SamplePayload,
        source: String,
    ) -> Result<Emission, StoreError> {
        let idx = self.find_producer_by_subscription(subscription_id)?;
        self.assert_not_terminated(idx)?;
        let epoch_for_frame = self.producers[idx].snapshot_epoch.max(1);

        let envelope = self.build_envelope_with_epoch(
            idx,
            subscription_id,
            FrameClass::Delta,
            epoch_for_frame,
            replay_seq,
            Freshness::Replayed,
            Completeness::Full,
            Some(Invalidation {
                stale_reason: StaleReason::ReplayedFromBundle,
                caused_by: Some(crate::envelope::CausedBy {
                    note: Some(format!("replayed from: {source}")),
                    ..crate::envelope::CausedBy::default()
                }),
            }),
            None,
            Some(payload.to_json()),
        );
        self.hooks.subscription_delta_emit += 1;
        self.hooks.subscription_freshness_downgrade += 1;

        let obs = self.deliver_replayed(subscription_id, &envelope)?;
        let tick = self.advance_tick();
        self.trace.push(TraceEvent::frame_emit(
            tick,
            "subscription_delta_emit_replayed",
            envelope.clone(),
            obs.clone(),
        ));
        Ok(Emission {
            envelope,
            consumer_observation: obs,
        })
    }

    /// End a replay session. Does not emit a frame; it is an
    /// observability-only transition the support-export lane
    /// needs to close the replay window.
    pub fn end_replay(&mut self, subscription_id: u64) -> Result<(), StoreError> {
        let _idx = self.find_producer_by_subscription(subscription_id)?;
        self.hooks.subscription_replay_end += 1;
        let tick = self.advance_tick();
        self.trace.push(TraceEvent::note(
            tick,
            "subscription_replay_end",
            "replay session ended; live frames may resume".to_owned(),
        ));
        Ok(())
    }

    /// Snapshot of every consumer projection.
    pub fn consumers(&self) -> &[ConsumerProjection] {
        &self.consumers
    }

    // -----------------------------------------------------------------
    // Internals.
    // -----------------------------------------------------------------

    fn find_producer(&self, query_family: &str, scope_ref: &ScopeRef) -> Result<usize, StoreError> {
        self.producers
            .iter()
            .position(|p| {
                p.producer.query_family == query_family && &p.producer.scope_ref == scope_ref
            })
            .ok_or(StoreError::ProducerNotRegistered)
    }

    fn find_producer_by_subscription(&self, subscription_id: u64) -> Result<usize, StoreError> {
        self.producers
            .iter()
            .position(|p| p.subscription_id == Some(subscription_id))
            .ok_or(StoreError::SubscriptionNotFound)
    }

    fn assert_not_terminated(&self, idx: usize) -> Result<(), StoreError> {
        if self.producers[idx].terminated {
            Err(StoreError::AlreadyTerminated)
        } else {
            Ok(())
        }
    }

    fn assert_derivation_freshness_legal(
        &self,
        idx: usize,
        freshness: Freshness,
    ) -> Result<(), StoreError> {
        if matches!(
            self.producers[idx].producer.derivation_class,
            DerivationClass::Derived
        ) && matches!(freshness, Freshness::Authoritative)
        {
            return Err(StoreError::DerivedCannotClaimAuthoritative);
        }
        Ok(())
    }

    fn build_envelope(
        &self,
        idx: usize,
        subscription_id: u64,
        frame_class: FrameClass,
        delta_seq: u64,
        freshness: Freshness,
        completeness: Completeness,
        invalidation: Option<Invalidation>,
        terminal_reason: Option<TerminalReason>,
        payload: Option<JsonValue>,
    ) -> SubscriptionEnvelope {
        let epoch = self.producers[idx].snapshot_epoch;
        self.build_envelope_with_epoch(
            idx,
            subscription_id,
            frame_class,
            epoch,
            delta_seq,
            freshness,
            completeness,
            invalidation,
            terminal_reason,
            payload,
        )
    }

    #[allow(clippy::too_many_arguments)]
    fn build_envelope_with_epoch(
        &self,
        idx: usize,
        subscription_id: u64,
        frame_class: FrameClass,
        snapshot_epoch: u64,
        delta_seq: u64,
        freshness: Freshness,
        completeness: Completeness,
        invalidation: Option<Invalidation>,
        terminal_reason: Option<TerminalReason>,
        payload: Option<JsonValue>,
    ) -> SubscriptionEnvelope {
        let p = &self.producers[idx].producer;
        SubscriptionEnvelope {
            subscription_schema_version: SUBSCRIPTION_SCHEMA_VERSION,
            subscription_id,
            query_family: p.query_family.clone(),
            scope_ref: p.scope_ref.clone(),
            authority_class: p.authority_class,
            derivation_class: p.derivation_class,
            snapshot_epoch,
            delta_seq,
            frame_class,
            freshness,
            completeness,
            backpressure_mode: p.backpressure_mode,
            view_class: p.view_class,
            producer_refs: p.producer_refs.clone(),
            invalidation,
            terminal_reason,
            payload,
        }
    }

    fn deliver(
        &mut self,
        subscription_id: u64,
        envelope: &SubscriptionEnvelope,
    ) -> Result<ConsumerObservation, StoreError> {
        let consumer = self
            .consumers
            .iter_mut()
            .find(|c| c.subscription_id == subscription_id)
            .ok_or(StoreError::SubscriptionNotFound)?;
        let prior_freshness = consumer.last_freshness;
        let prior_completeness = consumer.last_completeness;

        let mut observation = ConsumerObservation::apply(envelope);

        // Lifecycle validation against local projection.
        match envelope.frame_class {
            FrameClass::Snapshot => {
                // Replayed / imported snapshots do NOT replace
                // live epoch; they are separate lineages.
                if matches!(envelope.freshness, Freshness::Replayed) {
                    consumer.ever_replayed = true;
                } else if matches!(envelope.freshness, Freshness::Imported) {
                    consumer.ever_imported = true;
                } else {
                    consumer.last_snapshot_epoch = envelope.snapshot_epoch;
                    consumer.last_delta_seq = 0;
                    consumer.is_stale = false;
                }
            }
            FrameClass::Delta => {
                if matches!(envelope.freshness, Freshness::Replayed) {
                    consumer.ever_replayed = true;
                    // Replay deltas never advance the live seq.
                    observation
                        .reviewer_notes
                        .push("replayed delta does not advance live delta_seq".to_owned());
                } else {
                    // Strict monotonicity within epoch.
                    let expected = consumer.last_delta_seq + 1;
                    if envelope.snapshot_epoch != consumer.last_snapshot_epoch {
                        observation.reviewer_notes.push(
                            "delta arrived on a different snapshot_epoch; projection is stale"
                                .to_owned(),
                        );
                        consumer.is_stale = true;
                    } else if envelope.delta_seq != expected {
                        observation.reviewer_notes.push(format!(
                            "out-of-order delta: expected {expected}, observed {}",
                            envelope.delta_seq
                        ));
                        consumer.is_stale = true;
                    } else {
                        consumer.last_delta_seq = envelope.delta_seq;
                    }
                }
            }
            FrameClass::ResyncRequired => {
                consumer.is_stale = true;
            }
            FrameClass::Terminal => {
                consumer.is_terminal = true;
            }
        }

        consumer.last_freshness = envelope.freshness;
        consumer.last_completeness = envelope.completeness;

        // Observability hook firings.
        if freshness_is_downgrade(prior_freshness, envelope.freshness) {
            self.hooks.subscription_freshness_downgrade += 1;
            observation.reviewer_notes.push(format!(
                "freshness downgrade: {} -> {}",
                prior_freshness.as_str(),
                envelope.freshness.as_str()
            ));
        }
        if prior_completeness != envelope.completeness {
            self.hooks.subscription_completeness_changed += 1;
            observation.reviewer_notes.push(format!(
                "completeness changed: {} -> {}",
                prior_completeness.as_str(),
                envelope.completeness.as_str()
            ));
        }

        Ok(observation)
    }

    fn deliver_replayed(
        &mut self,
        subscription_id: u64,
        envelope: &SubscriptionEnvelope,
    ) -> Result<ConsumerObservation, StoreError> {
        // Replayed delta: route through `deliver` so the consumer
        // observation is produced consistently, but the caller
        // has already enforced "does not advance live epoch".
        self.deliver(subscription_id, envelope)
    }
}

impl Default for ReactiveStore {
    fn default() -> Self {
        Self::new()
    }
}

/// Errors the store refuses to paper over. Each maps to an
/// ADR 0005 rule; no silent demotion.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StoreError {
    ProducerNotRegistered,
    SubscriptionNotFound,
    AlreadySubscribed,
    AlreadyTerminated,
    DeltaBeforeSnapshot,
    DerivedCannotClaimAuthoritative,
    ImportedRequiresDerived,
}

impl StoreError {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::ProducerNotRegistered => "producer_not_registered",
            Self::SubscriptionNotFound => "subscription_not_found",
            Self::AlreadySubscribed => "already_subscribed",
            Self::AlreadyTerminated => "already_terminated",
            Self::DeltaBeforeSnapshot => "delta_before_snapshot",
            Self::DerivedCannotClaimAuthoritative => "derived_cannot_claim_authoritative",
            Self::ImportedRequiresDerived => "imported_requires_derived",
        }
    }
}

impl std::fmt::Display for StoreError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::error::Error for StoreError {}

/// Freshness-lattice rule used by the observability-only
/// `subscription_freshness_downgrade` hook. Direction is "away
/// from authoritative". The lattice is partial; the prototype
/// flags any move away from `authoritative` as a downgrade.
pub fn freshness_is_downgrade(prior: Freshness, next: Freshness) -> bool {
    if prior == next {
        return false;
    }
    // Moving away from authoritative is a downgrade.
    if matches!(prior, Freshness::Authoritative) {
        return true;
    }
    // Moving from warming to stale / unavailable is a
    // downgrade; moving from warming to authoritative is an
    // upgrade.
    if matches!(prior, Freshness::Warming) && matches!(next, Freshness::Stale) {
        return true;
    }
    if matches!(prior, Freshness::Cached) && matches!(next, Freshness::Stale) {
        return true;
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::envelope::{ScopeClass, SUBSCRIPTION_SCHEMA_VERSION};

    fn ws_scope(id: &str) -> ScopeRef {
        ScopeRef {
            class: ScopeClass::Workspace,
            id: id.to_owned(),
        }
    }

    fn authoritative_producer() -> Producer {
        Producer {
            query_family: "vfs.tree".to_owned(),
            scope_ref: ws_scope("ws"),
            authority_class: AuthorityClass::WorkspaceVfs,
            derivation_class: DerivationClass::Authoritative,
            view_class: ViewClass::DurableLocalMaterialization,
            backpressure_mode: BackpressureMode::Realtime,
            producer_refs: vec![ProducerRef {
                producer_id: "aureline.vfs".to_owned(),
                producer_instance: "host/pid/boot".to_owned(),
                producer_version: None,
                input_digests: vec![],
                derivation_epoch: None,
                source: None,
            }],
        }
    }

    fn derived_producer() -> Producer {
        Producer {
            query_family: "language.diagnostics".to_owned(),
            scope_ref: ws_scope("ws"),
            authority_class: AuthorityClass::DerivedKnowledge,
            derivation_class: DerivationClass::Derived,
            view_class: ViewClass::DurableLocalMaterialization,
            backpressure_mode: BackpressureMode::Realtime,
            producer_refs: vec![ProducerRef {
                producer_id: "aureline.language".to_owned(),
                producer_instance: "host/pid/boot".to_owned(),
                producer_version: None,
                input_digests: vec![InputDigest {
                    name: "editor.buffer_snapshot@ws".to_owned(),
                    digest: "sha256:aa".to_owned(),
                }],
                derivation_epoch: Some(1),
                source: None,
            }],
        }
    }

    #[test]
    fn snapshot_then_delta_happy_path() {
        let mut store = ReactiveStore::new();
        store.register_producer(authoritative_producer());
        let sid = store
            .subscribe("vfs.tree", &ws_scope("ws"), BackpressureMode::Realtime)
            .unwrap();
        let snap = store
            .emit_snapshot(
                sid,
                Freshness::Authoritative,
                Completeness::Full,
                SamplePayload {
                    summary: "initial".to_owned(),
                    entry_count: 10,
                    coverage_ready: 1,
                    coverage_total: 1,
                    detail_lines: vec![],
                },
                None,
            )
            .unwrap();
        assert_eq!(
            snap.envelope.subscription_schema_version,
            SUBSCRIPTION_SCHEMA_VERSION
        );
        assert_eq!(snap.envelope.snapshot_epoch, 1);
        assert_eq!(snap.envelope.delta_seq, 0);
        assert_eq!(snap.envelope.frame_class.as_str(), "snapshot");

        let delta = store
            .emit_delta(
                sid,
                Freshness::Authoritative,
                Completeness::Full,
                SamplePayload {
                    summary: "+1 file".to_owned(),
                    entry_count: 11,
                    coverage_ready: 1,
                    coverage_total: 1,
                    detail_lines: vec![],
                },
                None,
            )
            .unwrap();
        assert_eq!(delta.envelope.snapshot_epoch, 1);
        assert_eq!(delta.envelope.delta_seq, 1);
        assert_eq!(store.consumers()[0].last_delta_seq, 1);
        assert!(!store.consumers()[0].is_stale);
    }

    #[test]
    fn derived_cannot_claim_authoritative() {
        let mut store = ReactiveStore::new();
        store.register_producer(derived_producer());
        let sid = store
            .subscribe(
                "language.diagnostics",
                &ws_scope("ws"),
                BackpressureMode::Realtime,
            )
            .unwrap();
        let err = store
            .emit_snapshot(
                sid,
                Freshness::Authoritative,
                Completeness::Full,
                SamplePayload::default(),
                None,
            )
            .unwrap_err();
        assert_eq!(err, StoreError::DerivedCannotClaimAuthoritative);
    }

    #[test]
    fn resync_required_demotes_projection() {
        let mut store = ReactiveStore::new();
        store.register_producer(authoritative_producer());
        let sid = store
            .subscribe("vfs.tree", &ws_scope("ws"), BackpressureMode::Realtime)
            .unwrap();
        store
            .emit_snapshot(
                sid,
                Freshness::Authoritative,
                Completeness::Full,
                SamplePayload::default(),
                None,
            )
            .unwrap();
        store
            .emit_resync_required(
                sid,
                StaleReason::WatcherDropped,
                None,
                Completeness::Partial,
            )
            .unwrap();
        assert!(store.consumers()[0].is_stale);
        // Next snapshot bumps epoch.
        let snap = store
            .emit_snapshot(
                sid,
                Freshness::Authoritative,
                Completeness::Full,
                SamplePayload::default(),
                None,
            )
            .unwrap();
        assert_eq!(snap.envelope.snapshot_epoch, 2);
        assert!(!store.consumers()[0].is_stale);
    }

    #[test]
    fn replayed_does_not_advance_live_epoch() {
        let mut store = ReactiveStore::new();
        let mut producer = authoritative_producer();
        producer.derivation_class = DerivationClass::Derived;
        producer.authority_class = AuthorityClass::DerivedKnowledge;
        store.register_producer(producer);
        let sid = store
            .subscribe("vfs.tree", &ws_scope("ws"), BackpressureMode::Realtime)
            .unwrap();
        store
            .emit_snapshot(
                sid,
                Freshness::Warming,
                Completeness::Partial,
                SamplePayload::default(),
                None,
            )
            .unwrap();
        let prior_epoch = store.consumers()[0].last_snapshot_epoch;
        store.begin_replay(sid).unwrap();
        store
            .emit_replayed_delta(
                sid,
                99,
                SamplePayload::default(),
                "replay-bundle:xyz".to_owned(),
            )
            .unwrap();
        store.end_replay(sid).unwrap();
        assert_eq!(store.consumers()[0].last_snapshot_epoch, prior_epoch);
        assert!(store.consumers()[0].ever_replayed);
    }

    #[test]
    fn imported_requires_derived_producer() {
        let mut store = ReactiveStore::new();
        store.register_producer(authoritative_producer());
        let sid = store
            .subscribe("vfs.tree", &ws_scope("ws"), BackpressureMode::Realtime)
            .unwrap();
        let err = store
            .attach_imported_snapshot(sid, SamplePayload::default(), "imported-lsif:x".to_owned())
            .unwrap_err();
        assert_eq!(err, StoreError::ImportedRequiresDerived);
    }
}
