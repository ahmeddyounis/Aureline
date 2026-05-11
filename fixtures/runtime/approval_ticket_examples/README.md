# Runtime approval-ticket example fixtures

These fixtures are short, reviewable scenarios that anchor the runtime
approval-ticket vocabulary defined in
[`/schemas/runtime/approval_ticket.schema.json`](../../../schemas/runtime/approval_ticket.schema.json)
and the seed at
[`/artifacts/runtime/m1_runtime_approval_ticket_seed.yaml`](../../../artifacts/runtime/m1_runtime_approval_ticket_seed.yaml).

They are not a test suite. They are typed reviewable payloads that the
[runtime approval-ticket seed validation lane](../../../tests/governance/m1_runtime_approval_ticket_seed_lane/run_m1_runtime_approval_ticket_seed_lane.py)
parses, schema-validates, and replays as the first named live
consumer.

**Scope rules**

- Every fixture is a single `runtime_approval_ticket_record`. Raw
  secret bytes, raw URLs, raw delegated-token bodies, raw policy
  payloads, and raw evidence bodies never appear; opaque refs cross
  the boundary.
- Every fixture validates against the row schema and exercises a
  named authority class (`local_mutation`, `external_mutation`,
  `credential_projection`, `privileged_attach`).
- Fixtures describe observable, auditable approval moments — not
  internal implementation details.

**Index**

| Fixture | Primary authority class | What it freezes |
|---|---|---|
| [`local_mutation_destructive.json`](./local_mutation_destructive.json) | `local_mutation` | Single-use, preview-pinned, rollback-checkpoint-pinned destructive local edit. |
| [`external_mutation_provider_publish.json`](./external_mutation_provider_publish.json) | `external_mutation` | Provider-plane publish admitted under a runtime ticket via `inner_provider_approval_ticket_refs`. |
| [`credential_projection_build_sandbox.json`](./credential_projection_build_sandbox.json) | `credential_projection` | Bounded-reuse credential handle projected into a build sandbox under a named projection mode. |
| [`privileged_attach_language_server.json`](./privileged_attach_language_server.json) | `privileged_attach` | Privileged debugger attach with `step_up_required_flag = true`. |
| [`local_mutation_extension_apply.json`](./local_mutation_extension_apply.json) | `local_mutation` | Extension-requested reversible local edit; shell remains the issuer and carries `requesting_surface_ref`. |
