# Beta Execution-Context Resolver

This document is the reviewer-facing landing page for the beta finalize layer
of the execution-context resolver. It pins the closed set of beta resolver
lanes, the [`TargetClass`](../../../crates/aureline-runtime/src/execution_context/mod.rs)
tokens each lane is allowed to mint, the consumer surfaces that resolve
through the lane, and the typed ticket-drift evaluation that invalidates a
stored ticket or preview the moment its binding disagrees with the freshly
resolved context.

The machine-readable boundary lives at
[`/schemas/execution/execution_context.schema.json`](../../../schemas/execution/execution_context.schema.json).
The canonical record and resolver still live in
[`/crates/aureline-runtime/src/execution_context/`](../../../crates/aureline-runtime/src/execution_context/);
the lane manifest and ticket-drift evaluator live in
[`/crates/aureline-runtime/src/execution_context/beta.rs`](../../../crates/aureline-runtime/src/execution_context/beta.rs).

The beta promise:

- terminal, task, test, debug, AI-tool, and request-workspace surfaces all
  resolve through **one** [`ExecutionContext`](../../../crates/aureline-runtime/src/execution_context/mod.rs)
  contract; any surface that cannot route through the contract MUST visibly
  declare why before it dispatches;
- resolved toolchain, runtime, target, policy, trust, and provenance truth is
  inspectable on the same record before execution starts;
- a typed [`evaluate_ticket_drift`](../../../crates/aureline-runtime/src/execution_context/beta.rs)
  evaluator invalidates any stored ticket or preview whose binding disagrees
  with the freshly resolved context, instead of silently dispatching against
  the wrong target;
- the beta support-export packet quotes the same coverage manifest, lane
  samples, and drift evaluations the runtime, shell, and reviewer surfaces
  read, so support evidence does not fork its own contract.

## Beta lanes

| Lane | Target classes | Boundary cue | Claimed surfaces |
| --- | --- | --- | --- |
| `local_host` | `local_host`, `notebook_kernel_local` | not required | terminal, task, test, debug, ai_tool_call |
| `remote_attach` | `ssh_remote`, `notebook_kernel_remote` | required | terminal, task, test, debug, ai_tool_call |
| `container` | `container_local`, `devcontainer` | required | terminal, task, test, debug, ai_tool_call |
| `request_workspace` | `remote_workspace_vm`, `managed_workspace`, `prebuild_runtime`, `ai_sandbox` | required | terminal, task, test, debug, ai_tool_call |

The canonical manifest is exposed by
[`ExecutionContextBetaCoverageManifest::canonical`](../../../crates/aureline-runtime/src/execution_context/beta.rs)
and checked in at
[`/fixtures/runtime/execution_context_beta/beta_lane_coverage.json`](../../../fixtures/runtime/execution_context_beta/beta_lane_coverage.json).
The reviewer fixture asserts that every [`TargetClass`](../../../crates/aureline-runtime/src/execution_context/mod.rs)
variant is represented by exactly one lane row, so a future widening of the
vocabulary without a lane update fails the build.

## Ticket-drift evaluation

A stored "ticket" is any cached approval, preview, or rerun snapshot the
runtime persists alongside a resolved context. The beta layer pins a closed
set of [`TicketDriftField`](../../../crates/aureline-runtime/src/execution_context/beta.rs)
tokens the evaluator compares between the stored
[`TicketDriftBinding`](../../../crates/aureline-runtime/src/execution_context/beta.rs)
and the freshly resolved [`ExecutionContext`](../../../crates/aureline-runtime/src/execution_context/mod.rs):

| Drift field | Meaning |
| --- | --- |
| `target_class` | Stored target class differs from the freshly resolved class |
| `canonical_target_id` | Canonical target id changed under the same class |
| `working_directory` | Resolved working directory changed |
| `toolchain_class` | Resolved toolchain class changed |
| `capsule_hash` | Capsule hash advanced past the stored hash |
| `capsule_drift_state` | Capsule drift regressed away from `in_sync` |
| `scope_class` | Workset scope class changed |
| `policy_epoch_advanced` | Policy epoch advanced past the stored epoch |
| `trust_state_regressed` | Trust regressed (trusted → restricted/pending or restricted → pending) |

When any row is present, the evaluator returns
[`TicketDriftOutcome::Invalidated`](../../../crates/aureline-runtime/src/execution_context/beta.rs)
and consumer chrome MUST re-authorise before dispatching. When the rows are
empty the outcome is `Fresh` and the stored ticket / preview is safe to
dispatch. Every evaluation record is export-safe and is replayed verbatim by
the support-export packet.

## Support export

The [`ExecutionContextBetaSupportExport`](../../../crates/aureline-runtime/src/execution_context/beta.rs)
packet projects the canonical lane coverage manifest, per-lane resolved-context
sample rows, and any ticket-drift evaluations the support flow attached. The
sample row quotes the lane token, surface, resolved
[`ExecutionContext::execution_context_id`](../../../crates/aureline-runtime/src/execution_context/mod.rs),
canonical target id, target class, boundary-cue posture, and the
`has_degraded_field` flag. Raw env values, raw command lines, and raw secrets
are out of scope.

## Failure-drill fixtures

Reviewer fixtures live under
[`/fixtures/runtime/execution_context_beta/`](../../../fixtures/runtime/execution_context_beta/)
and exercise four named drills:

- `local_lane.json` — local terminal seed resolves through the
  `local_host` lane with no boundary cue and no degraded fields.
- `remote_lane.json` — SSH remote task seed resolves through the
  `remote_attach` lane, lights the boundary cue, and records the
  `mixed_version_unchecked` posture.
- `container_lane.json` — devcontainer test seed resolves through the
  `container` lane with the containerised runtime toolchain.
- `request_workspace_lane.json` — managed-workspace task seed resolves
  through the `request_workspace` lane with restricted trust narrowing
  visible on the same record.
- `ticket_drift_invalidated.json` — a stored binding minted against the
  local-host lane is evaluated against a freshly resolved remote-attach
  context. The evaluator MUST return `invalidated` with `target_class`,
  `canonical_target_id`, and `working_directory` drift rows.
- `beta_lane_coverage.json` — canonical lane coverage manifest the runtime
  emits.

The integration test that replays these fixtures lives at
[`/crates/aureline-runtime/tests/execution_context_beta.rs`](../../../crates/aureline-runtime/tests/execution_context_beta.rs).
It also drives the resolver into the
[`aureline-terminal`](../../../crates/aureline-terminal) PTY host so the
shell-side projection lights the boundary cue when a stored local ticket
is invalidated against a fresh remote-attach context.

## Out of scope for this beta

- Full activator-decision orchestration (env-manager shims, venv activation,
  devcontainer build, oci image fetch). The beta lane carries the
  activation-strategy class only.
- M5 notebook-kernel launch and attach depth.
- Cross-workspace ticket import. Drift bindings are workspace-scoped.
- Launch-language breadth outside the four claimed beta lanes.

## How to verify

```
cargo test -p aureline-runtime execution_context::beta
cargo test -p aureline-runtime --test execution_context_beta
```

## Cross-references

- Execution-context seed contract —
  [`/docs/runtime/execution_context_seed.md`](../execution_context_seed.md)
- Execution-context alpha export —
  [`/docs/runtime/execution_context_alpha.md`](../execution_context_alpha.md)
- Beta task-event model — [`task_event_model_beta.md`](task_event_model_beta.md)
- Beta debugger / DAP host — [`debugger_host_beta.md`](debugger_host_beta.md)
- Beta run / debug profile model — [`run_debug_profiles_beta.md`](run_debug_profiles_beta.md)
