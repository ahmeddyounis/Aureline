# Fixture: policy denial

## Scenario

An extension-class actor (`actor_class = extension`) invokes a
workspace-scoped VFS mutation against a workspace it does not own, or
an AI-class actor invokes a method whose manifest does not list `ai`
in `actor_classes`. The service rejects the request with a typed
`policy` error before touching any workspace state.

## Envelope fields exercised

- Request `actor_class` and `workspace_scope` are both populated.
- Response `result = Err(error_payload)` with
  `error_payload.class = policy` and a stable per-class `code` (for
  example, `vfs.path_denied` or `editor.actor_class_denied`).
- `error_payload.retry` is `reauth_required` when the denial is
  recoverable by elevating the actor; `no` otherwise.
- The denial MUST occur before any side-effectful work; the service
  is not permitted to "peek and then deny".

## Hooks exercised

- `rpc_request_receive` — service dispatcher decoded the envelope.
- `rpc_error_classified` — class `policy` recorded.
- `rpc_response_dispatch` — terminal denial response written.

## Expected observable outcomes

- The request is refused with a typed `policy` code, never a generic
  `internal` or `local`. The audit record can route the denial to the
  correct reviewer queue.
- The workspace state is observably unchanged (benchmark lab / audit
  verifies no fs writes, no cache invalidations, no side-effectful
  downstream calls).
- Same taxonomy word (`policy`) flows to the Appendix B.2 CLI exit
  code mapping.

## ADR sections motivating this fixture

- Request metadata — workspace scope and actor-class rules.
- Error taxonomy — `policy` class and retry posture.
- Method manifest — `actor_classes` declaration per method.
