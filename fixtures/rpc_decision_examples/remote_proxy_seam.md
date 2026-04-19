# Fixture: remote proxy seam

## Scenario

The desktop frontend is connected to a remote workspace through a
tunnelled byte stream (SSH or a managed tunnel). A request issued
locally is routed by the remote connector across the seam and served
by a service running on the remote host. The connection later drops
and reconnects.

## Envelope fields exercised

- Same envelope as the in-process case crosses the seam unchanged.
- `trace.trace_id`, `trace.span_id`, `trace.flags` are preserved.
- `baggage` is dropped at the seam unless the connector policy
  whitelists explicit keys; the default is empty baggage on the
  remote side.
- `workspace_scope` is a remote-workspace id (not `global`); the
  remote service rejects a scope that does not match its authority
  domain with a typed `policy` error.
- `deadline_ns` is translated through the per-side monotonic-clock
  seed exchanged at handshake; the relative budget is preserved, not
  the absolute tick count.

## Hooks exercised

- `rpc_handshake_complete` — fires on both sides when the seam
  connects.
- `rpc_capability_intersection` — may fire when the two sides have
  different method manifests.
- `rpc_request_send` / `rpc_request_receive` — fire on each side of
  the seam per call.
- `rpc_connection_drop` — fires on the seam drop; the consumer
  surface chooses reconnection posture.

## Expected observable outcomes

- A consumer of the envelope on either side sees the same field
  vocabulary; the `aureline/bin/1` encoding is the same on both
  sides of the seam.
- A dropped connection terminates inflight calls with
  `error_class = unavailable` (not `internal`); subscriptions MUST
  be re-established by the consumer.
- Baggage whitelisting is auditable: a support bundle taken on both
  sides lists which keys crossed the seam and which were dropped.

## ADR sections motivating this fixture

- Remote proxy cost — one vocabulary across the seam.
- Request metadata — baggage drop default, trace preservation.
- Protected-hot-path hooks — `rpc_connection_drop`,
  `rpc_handshake_complete`.
