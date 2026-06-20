# Generated-artifact regeneration-plan proof packet

The canonical regeneration-plan packet is implemented in
[`crates/aureline-generated/src/regeneration_plan/mod.rs`](../../crates/aureline-generated/src/regeneration_plan/mod.rs)
and serialized to
[`artifacts/generated/regeneration-plan-packet.json`](./regeneration-plan-packet.json).

It is the checked-in truth source for:

- the reviewer contract in
  [`docs/generated/regeneration-plan.md`](../../docs/generated/regeneration-plan.md)
- the boundary schema at
  [`schemas/generated/regeneration-plan.schema.json`](../../schemas/generated/regeneration-plan.schema.json)
- fixture replay in
  [`crates/aureline-generated/tests/regeneration_plan.rs`](../../crates/aureline-generated/tests/regeneration_plan.rs)
- the fixture corpus under
  [`fixtures/generated/regeneration-plan/`](../../fixtures/generated/regeneration-plan/)

## What the packet models

For each scenario, the packet carries one case — the regeneration request (the
target artifacts with their canonical-source refs, generator/runtime
requirements, observed precondition states, and declared side effects, plus a
rollback boundary) and the plan the engine reaches before execution. A
regeneration runs in full only when every target is ready; otherwise the plan is
labeled precisely — `ready_stale_inputs`, `partial`, `policy_limited`, or
`blocked` — and never masquerades as a clean rebuild.

## Cases

| Case | Class(es) | Readiness | Targets run | Rollback | Side effects |
| --- | --- | --- | --- | --- | --- |
| `regeneration-plan.scaffolded_project.ready` | scaffolded_project | `ready` | 1/1 | `fully_reversible` | local_compute |
| `regeneration-plan.notebook_output.ready_stale_inputs` | notebook_output | `ready_stale_inputs` | 1/1 | `fully_reversible` | local_compute |
| `regeneration-plan.framework_codegen.partial` | framework_codegen, request_artifact | `partial` | 1/2 | `fully_reversible` | local_compute |
| `regeneration-plan.preview_derivative.blocked_runtime` | preview_derivative | `blocked` | 0/1 | `fully_reversible` | local_compute |
| `regeneration-plan.request_artifact.policy_limited` | request_artifact | `policy_limited` | 0/1 | `fully_reversible` | local_compute |
| `regeneration-plan.scaffolded_project.undeclared_side_effect` | scaffolded_project | `policy_limited` | 0/1 | `partially_reversible` | local_compute+network_install |
| `regeneration-plan.framework_codegen.ready_declared_install` | framework_codegen | `ready` | 1/1 | `partially_reversible` | local_compute+network_install+tool_download |

## Invariants the packet freezes

1. Every regenerate action resolves to a typed plan carrying its target
   artifacts, canonical-source refs, generator/runtime requirements, side-effect
   boundary, and rollback boundary before execution — never a bare command.
2. Blocked, partial, stale-input, and policy-limited plans are labeled precisely
   by the readiness state and never masquerade as a clean success.
3. A regeneration never hides a networked install, tool download, secret use, or
   broad filesystem write: any undeclared sensitive side effect holds the target
   for disclosure instead of running silently.
4. The rollback coverage is computed from the side effects, so a regeneration
   that escapes the workspace checkpoint is reported as only partially
   reversible rather than implying a clean undo.
5. Every plan that is not fully ready carries why-blocked tokens and a recovery
   path, so a block is never reduced to a generic failure; the plan and result
   packets are preserved in support exports and release evidence.

## Surfaces

The packet binds one rendered surface per consumer, all checked to exist on
disk:

- `regenerate_plan_sheet` — `crates/aureline-vfs/src/save_conflict_suite/mod.rs`
- `help_regeneration_guide` — `crates/aureline-shell/src/help/mod.rs`
- `support_export` — `crates/aureline-support/src/generated_lineage/mod.rs`
- `release_evidence` — `crates/aureline-release/src/harden_docs_help_about_and_service_health_truth/mod.rs`
- `ai_context` — `crates/aureline-ai/src/context_inspector/mod.rs`

## Regenerating this artifact

```bash
cargo run -q -p aureline-generated --example dump_regeneration_plan -- packet \
  | python3 -c 'import json,sys; print(json.dumps(json.load(sys.stdin), indent=2, sort_keys=True))' \
  > artifacts/generated/regeneration-plan-packet.json
```
