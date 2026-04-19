# Fixture: deadline expiry

## Scenario

A caller dispatches a unary method with a deadline that is shorter
than the service's wall-clock cost on this input. The transport
observes the deadline and terminates the call with a typed
`deadline_exceeded` error before the service returns a terminal
response.

## Envelope fields exercised

- Request `deadline_ns` is finite and in the past by the time the
  transport notices expiry.
- Response `result = Err(error_payload)` with
  `error_payload.class = deadline_exceeded` and a stable
  per-class `code`.
- `error_payload.retry` is caller-controlled (the taxonomy allows the
  caller to decide whether to retry with a longer deadline).
- `error_payload.span_context` points at the producer span so the
  support bundle can re-enter the trace.

## Hooks exercised

- `rpc_deadline_expired` — transport fires the deadline-driven cancel
  on the inflight request.
- `rpc_cancel_observed` — the service dispatcher observes the
  transport-emitted cancel (deadlines act as implicit cancellations).
- `rpc_error_classified` — error envelope written with class
  `deadline_exceeded`.

## Expected observable outcomes

- The transport does NOT rewrite `deadline_exceeded` into
  `unavailable`: the class is always observable.
- The service either finishes and returns its typed terminal
  (race-loss case) or observes the cancel and returns
  `deadline_exceeded`; the response frame tells the truth about which
  happened and never folds them.
- Fan-out children inherit the (now-expired) deadline; they do not
  receive an extended one.

## ADR sections motivating this fixture

- Request metadata — Deadlines.
- Error taxonomy — `deadline_exceeded` is always observable.
- Protected-hot-path hooks — `rpc_deadline_expired`,
  `rpc_cancel_observed`.
