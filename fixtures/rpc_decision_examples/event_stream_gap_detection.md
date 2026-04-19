# Fixture: event-stream gap detection

## Scenario

A consumer has subscribed to an event kind (for example,
`BufferSnapshotDelta`) with `delivery_mode = exactly_once`. The
producer emits events at monotonically increasing `sequence` values.
The consumer's dispatcher observes a jump (for example, ..., 42, 45,
...) that violates the contiguity invariant.

## Envelope fields exercised

- `frame_kind: event`, `subscription_id` stable across frames.
- `sequence` is a per-subscription monotonic counter; consumers
  gap-detect on this field.
- `delivery_mode` is frozen at subscribe time; changing it mid-stream
  is not legal.
- `producer.instance` lets the consumer distinguish a producer
  restart from a true gap.

## Hooks exercised

- `event_stream_consume` — every decoded event fires this hook.
- `event_stream_gap_detected` — fires on the non-contiguous sequence.
- `rpc_connection_drop` — may fire if the gap coincides with a
  reconnection; the consumer correlates the two signals.

## Expected observable outcomes

- The gap is surfaced as a typed observation, not swallowed. The
  subscription is NOT silently reset on a gap detection in
  `exactly_once` mode; downstream logic decides the recovery posture
  (replay from last known sequence, rebuild state, fail closed).
- If `producer.instance` also changed, the gap is correlated with a
  producer restart in the observability stream; recovery posture may
  differ from a same-instance gap.
- `at_least_once` consumers do NOT gap-detect on `sequence` alone;
  they dedupe on `idempotency_key` and treat `sequence` as advisory.

## ADR sections motivating this fixture

- Event-stream envelope — per-subscription monotonic `sequence`.
- Protected-hot-path hooks — `event_stream_gap_detected`.
- Versioning — `delivery_mode` frozen at subscribe time.
