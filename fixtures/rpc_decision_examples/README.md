# RPC decision-example fixtures

These fixtures are short, reviewable scenarios that anchor the
envelope fields, error taxonomy, and protected-hot-path hook names
defined in
[ADR 0004](../../docs/adr/0004-rpc-transport-and-schema-toolchain.md)
to concrete inputs and observable outcomes. They are not a test suite;
they are the vocabulary the shell spike, the supervisor prototype, the
VFS / editor / telemetry lanes, and the benchmark lab use when they
instrument a hook or a code path.

**Scope rules**

- Every fixture names the envelope fields or hooks it exercises, the
  transport or service surface it stresses, and the observable outcome
  instrumentation should capture.
- Fixtures never assert latency numbers; the benchmark lab owns
  budgets. Fixtures only describe *what* to measure, not *how fast*.
- Fixtures never encode wire bytes. They describe the logical envelope
  contents; the `aureline/bin/1` encoding is derived from the Rust
  types in `crates/aureline-rpc`, not from this corpus.
- A new fixture MUST hit at least one protected-hot-path hook or one
  frozen envelope field and MUST cite the ADR section that motivates
  it.

**Index**

| Fixture                                                                           | Primary hooks / fields                                                 | Surface stressed                                                          |
|-----------------------------------------------------------------------------------|------------------------------------------------------------------------|---------------------------------------------------------------------------|
| [`in_process_local_call.md`](./in_process_local_call.md)                          | `rpc_request_send`, `rpc_request_receive`, `rpc_response_dispatch`     | Happy-path unary call over in-process channels                            |
| [`deadline_expiry.md`](./deadline_expiry.md)                                      | `rpc_deadline_expired`, `deadline_ns`, `error_class: deadline_exceeded`| Transport-side deadline enforcement; cancel emission                      |
| [`caller_initiated_cancel.md`](./caller_initiated_cancel.md)                      | `rpc_cancel_observed`, cancel frame, `error_class: cancelled`          | First-class cancel frame; idempotent cancel                               |
| [`event_stream_gap_detection.md`](./event_stream_gap_detection.md)                | `event_stream_gap_detected`, `sequence`, `delivery_mode`               | Per-subscription monotonic `sequence` invariant                           |
| [`cross_process_trace_join.md`](./cross_process_trace_join.md)                    | `trace_context`, `rpc_request_send`, `rpc_request_receive`             | Remote proxy seam; trace_id preserved across process                      |
| [`capability_negotiation_missing_method.md`](./capability_negotiation_missing_method.md) | `rpc_handshake_complete`, `rpc_capability_intersection`, `error_class: unavailable` | Handshake intersection fails closed on missing method          |
| [`policy_denial.md`](./policy_denial.md)                                          | `workspace_scope`, `actor_class`, `error_class: policy`                | Service rejects a scope-mismatched or unauthorised actor                  |
| [`at_least_once_idempotency_dedupe.md`](./at_least_once_idempotency_dedupe.md)    | `event_stream_dedupe_hit`, `idempotency_key`, `delivery_mode: at_least_once` | Consumer-side dedupe contract                                        |
| [`remote_proxy_seam.md`](./remote_proxy_seam.md)                                  | `baggage` drop, `trace_context` preserved, `rpc_connection_drop`       | Tunnelled envelope; baggage whitelisting default                          |
| [`error_class_routing.md`](./error_class_routing.md)                              | `error_class`, `retry_hint`, `error_payload.code`                      | Frozen taxonomy routes to exit code, retry, repair                        |
