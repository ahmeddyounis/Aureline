# Fixture: cross-process trace join

## Scenario

A caller in the desktop process invokes a method on a service that
runs in a separate supervisor-managed process. The call traverses the
length-prefixed byte-stream transport (Unix-domain socket / named
pipe). A trace collector ingests spans from both sides and expects to
join them into one trace.

## Envelope fields exercised

- `trace.trace_id` is identical on the request, the server-side span,
  and every response / progress / event frame the server emits in
  response.
- `trace.span_id` differs per side; `trace.parent_span_id` on the
  service span points at the caller's `span_id`.
- `trace.flags` are preserved across the seam unchanged (sampled and
  debug bits must cross).
- `baggage` is dropped at the seam unless the connector policy
  whitelists specific keys; the default carries zero baggage.

## Hooks exercised

- `rpc_request_send` on the caller side.
- `rpc_request_receive` on the server side, with attributes that let
  the collector join the two spans.
- `rpc_response_dispatch` on the server side; the caller's receive
  hook (transport-internal) closes the caller span.

## Expected observable outcomes

- A single trace in the collector spans both processes. The
  W3C-tracecontext-compatible shape means an external collector
  ingests the traces without a translation layer.
- Dropped baggage does NOT silently break the trace; trace fields are
  a separate concern from baggage.
- If the handshake downgraded the encoding or capabilities,
  `rpc_capability_intersection` has already fired on both sides; the
  trace join still succeeds.

## ADR sections motivating this fixture

- Request metadata — Trace IDs.
- Remote proxy cost — same envelope over the seam; trace unchanged.
- Protected-hot-path hooks — `rpc_request_send`,
  `rpc_request_receive`, `rpc_response_dispatch`.
