# Remote attach rerun-last-test cases

Worked fixtures for the attach→resolve→rerun-last-test→normalized-events flow.
These cases exist to prove remote mode changes placement and route truth only;
command semantics, execution-context truth, and normalized task-event vocabulary
remain identical.

Canonical refs:

- Remote attach contract:
  [`/docs/remote/attach_tunnel_port_forward_contract.md`](../../../docs/remote/attach_tunnel_port_forward_contract.md)
  and
  [`/schemas/remote/attach_session.schema.json`](../../../schemas/remote/attach_session.schema.json)
- Execution context snapshot packet:
  [`/docs/execution/context_inspector_packet.md`](../../../docs/execution/context_inspector_packet.md)
  and
  [`/schemas/execution/context_snapshot.schema.json`](../../../schemas/execution/context_snapshot.schema.json)
- Normalized task events:
  [`/schemas/tooling/task_event_envelope.schema.json`](../../../schemas/tooling/task_event_envelope.schema.json)
  and
  [`/schemas/execution/normalized_task_event.schema.json`](../../../schemas/execution/normalized_task_event.schema.json)

Scope rules:

- Fixtures use opaque refs for targets, routes, actors, tickets, contexts,
  events, artifacts, and tests. Raw URLs, hostnames, ports, paths, command
  lines, environment bodies, and secret material do not appear.
- Missing capability, narrowed authority, witness mismatch, and artifact
  retention downgrades must be visible as typed state and/or typed denial; no
  fixture permits silent widening or silent fallback.
- Milestone slugs must not appear in any fixture field.

## Index

| Fixture | Primary coverage |
|---|---|
| `active_rerun_last_test_ok.yaml` | Active attach; rerun-last-test succeeds; normalized events cite the remote execution context. |
| `reconnect_after_partial_attach_blocks_rerun.yaml` | Transport drop during attach; read-only preserved; rerun-last-test is blocked/deferred with explicit disclosure. |
| `missing_remote_capability_denied.yaml` | Attach admitted but `test_run` capability not admitted; rerun-last-test denied with missing-capability disclosure. |
| `downgraded_artifact_availability.yaml` | Rerun succeeds but raw-payload/artifact retention is downgraded; result links preserve replay truth. |
| `target_mismatch_requires_reapproval.yaml` | Target witness mismatch/retarget review required; rerun-last-test fails closed until reapproval. |

