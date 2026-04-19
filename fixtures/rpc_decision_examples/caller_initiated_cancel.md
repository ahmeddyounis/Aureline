# Fixture: caller-initiated cancel

## Scenario

A caller dispatches a streaming method (for example, a long-running
code-search) and, before the server emits its terminal response, the
caller decides to abandon the work (the user hit Escape, a superseding
command displaced the first). The caller writes a `Cancel` frame on
the request's `cancellation_channel`. The server stops work and emits
a terminal `cancelled` error.

## Envelope fields exercised

- `frame_kind: cancel` with `reason: caller_initiated`.
- `cancellation_channel` matches the value on the originating request.
- Final response `result = Err(error_payload)` with
  `error_payload.class = cancelled` and `retry = no`.
- Intermediate `Progress` frames (if any) carry `terminal = false`;
  the cancel response is the first `terminal = true` frame.

## Hooks exercised

- `rpc_cancel_observed` — the service dispatcher observes the cancel.
- `rpc_response_dispatch` — the terminal `cancelled` response writes.
- `rpc_error_classified` — class `cancelled` recorded.

## Expected observable outcomes

- Repeated cancels on the same `cancellation_channel` are idempotent:
  `rpc_cancel_observed` fires at most once per request regardless of
  how many cancel frames the caller sent.
- If the server finished the work before observing the cancel
  (race-win case), the server's typed terminal result is returned and
  `error_class: cancelled` does NOT appear; the response again tells
  the truth.
- Fan-out children observe the same cancellation channel transitively
  and terminate with `cancelled` as well.

## ADR sections motivating this fixture

- Request metadata — Cancellation.
- Cancel frame — frozen reason vocabulary; idempotent cancel.
- Error taxonomy — `cancelled` is always observable.
