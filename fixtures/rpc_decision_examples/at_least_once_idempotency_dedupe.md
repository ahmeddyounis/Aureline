# Fixture: at-least-once idempotency dedupe

## Scenario

A producer (for example, a task runner) publishes events with
`delivery_mode = at_least_once`. A network retry or a producer-side
replay causes the same logical event to be emitted twice, each
carrying the same `idempotency_key`. The consumer's dispatcher must
dedupe on the key within the documented retention window.

## Envelope fields exercised

- `frame_kind: event`, `delivery_mode: at_least_once`,
  `idempotency_key` is present (the envelope schema requires it when
  `delivery_mode = at_least_once`).
- `sequence` may differ across the two duplicate frames (a retry may
  re-number under producer control); `idempotency_key` does not.
- `producer.instance` may or may not change; the dedupe rule is
  independent of producer identity.

## Hooks exercised

- `event_stream_publish` — producer writes the event (possibly twice
  on retry).
- `event_stream_consume` — fires on first successful delivery.
- `event_stream_dedupe_hit` — fires when the consumer observes the
  duplicate and drops it.

## Expected observable outcomes

- The consumer's downstream side effect is executed exactly once.
- `event_stream_dedupe_hit` is an observability-only hook; it is not
  a hot-path budget holder and does not gate release.
- An `exactly_once` subscription that receives two events with the
  same `idempotency_key` is a producer contract violation: the
  consumer MAY surface a typed `remote` error.

## ADR sections motivating this fixture

- Request metadata — event-stream idempotency rule.
- Envelope schema — `at_least_once` producers MUST carry
  `idempotency_key`.
- Protected-hot-path hooks — `event_stream_dedupe_hit`.
