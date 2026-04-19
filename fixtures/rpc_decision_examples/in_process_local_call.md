# Fixture: in-process local call

## Scenario

A local caller invokes a unary method (for example,
`vfs.read_metadata`) on a service that shares the process. The call
carries a finite deadline, a workspace scope, and an `actor_class` of
`user`. The service returns a typed `Ok` payload within the deadline.

## Envelope fields exercised

- `frame_kind: request` then `frame_kind: response`.
- `envelope_schema_version = 1`.
- `contract_version` matches an active entry in the service manifest.
- `workspace_scope` is the caller's workspace id (not `global`).
- `deadline_ns` is non-zero; set by the caller on the connection's
  monotonic clock.
- `cancellation_channel` is allocated by the client transport and
  echoed on every frame for this call.
- `actor_class = user`.
- `result = Ok(...)` with `terminal = true`.

## Hooks exercised

- `rpc_request_send` — client transport writes the request envelope.
- `rpc_request_receive` — server-side dispatcher decodes and routes.
- `rpc_response_dispatch` — service writes a terminal `Ok` response.

## Expected observable outcomes

- One `rpc.request` span per call, with attributes: method id,
  contract version, workspace scope, actor class, no error class.
- `server_hint_ns` is absent or zero on a unary happy-path response.
- `rpc_progress_emit` does NOT fire: unary methods never emit a
  `Progress` frame.
- `rpc_cancel_observed` and `rpc_deadline_expired` do NOT fire.

## ADR sections motivating this fixture

- Request / response envelope — frozen field vocabulary.
- Protected-hot-path hooks — send / receive / dispatch trio.
- Tradeoff table, **Hot-path performance** — in-process happy path is
  the canonical measurement target for envelope overhead.
