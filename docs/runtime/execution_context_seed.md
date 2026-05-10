# Execution-context seed: object model and resolver

This document is the reviewer-facing landing page for the M1
execution-context seed. It points at the canonical Rust object model, the
resolver seed that mints contexts for the terminal, task, and debug-prep
lanes through one API, and the failure drill that proves conflicting
inputs settle on a single source of truth.

## What this seed owns

- one inspectable [`ExecutionContext`](../../crates/aureline-runtime/src/execution_context/mod.rs)
  Rust object that carries:
  - `invocation_subject` (command id, surface, actor, workspace, profile),
  - `target_identity` (target class, canonical id, working directory,
    reachability state, local-vs-managed boundary marker),
  - `toolchain_identity` (class, id, resolved version, activation strategy,
    degraded-fallback flag),
  - `environment_capsule_ref` (capsule id, hash, schema version, drift
    state),
  - `policy_and_trust` (trust state, identity mode, policy epoch),
  - `workset_scope_class`, `cache_disposition`,
  - `provenance` (resolver version, recording timestamp, confidence level,
    per-input precedence decisions),
  - `degraded_fields` (visible honesty markers).
- one [`ExecutionContextResolver`](../../crates/aureline-runtime/src/execution_context/mod.rs)
  seed that mints contexts for terminal, task, and debug-prep lanes through
  the same API. The resolver is deterministic: same inputs and same
  monotonic timestamps produce the same record.

## Cross-tool boundary

The Rust object model is the schema of record. The cross-tool boundary
schema lives at
[`schemas/runtime/execution_context.schema.json`](../../schemas/runtime/execution_context.schema.json)
and follows the same `record_kind` and `schema_version` vocabulary so a
support export round-trips through both shapes.

The seed Rust object intentionally covers a subset of the boundary
schema's fields. Adding fields is additive-minor and does not bump the
schema version; widening a vocabulary is additive-minor; repurposing a
field is breaking and requires a new decision row.

## Precedence

Every field that accepts multiple input sources resolves under one
deterministic precedence:

1. `explicit_override` (highest) — the per-call override the caller passed
   on the [`ExecutionContextRequest`](../../crates/aureline-runtime/src/execution_context/mod.rs).
2. `surface_requested` — the value the invoking surface declared (e.g. the
   terminal pane requesting an SSH target).
3. `workspace_default` — the workspace authority's default (target class,
   working directory, scope class).
4. `resolver_fallback` (lowest) — used only when no other source supplies
   a value.

The resolver records the winning source in
`provenance.input_decisions[]`, even when no conflict occurred. When more
than one source contributes a different value, the losing sources appear
in `conflicting_sources` so a `why this target?` inspector can quote
exactly which input survived without re-deriving the precedence locally.

## Failure drill

The failure drill exercises conflicting `cwd` and `target_class` inputs
end-to-end:

- fixture: [`fixtures/runtime/execution_context_seed_cases/conflicting_inputs.json`](../../fixtures/runtime/execution_context_seed_cases/conflicting_inputs.json)
- unit coverage:
  `aureline_runtime::execution_context::tests::explicit_override_wins_over_surface_request_and_records_conflict`
  and
  `fixture_failure_drill_matches_resolver_output`
- terminal-pane consumer coverage:
  `crates/aureline-runtime/tests/terminal_consumer.rs::explicit_override_to_remote_target_lights_the_boundary_cue_on_the_session_header`

The terminal-pane consumer test proves the resolved object lights the
local-vs-managed boundary cue on the [`SessionHeader`](../../crates/aureline-terminal/src/pty_host/mod.rs)
when the resolver settles on a remote target — the same row a tab strip,
status mirror, support packet, and restore prompt all read.

## Protected walk

1. Open a terminal session — the bottom-panel terminal pane consumes a
   resolved [`ExecutionContext`](../../crates/aureline-runtime/src/execution_context/mod.rs)
   and quotes its `execution_context_id` on the [`SessionHeader`](../../crates/aureline-terminal/src/pty_host/mod.rs).
2. Resolve a task seed and a debug-prep seed through the same resolver —
   the integration test
   `crates/aureline-runtime/tests/terminal_consumer.rs::task_and_debug_seeds_resolve_through_the_same_resolver_and_match_terminal_context_shape`
   asserts the same record shape, schema version, scope class, identity
   mode, and trust state on every lane.
3. Inspect the shared model fields — every lane carries the same
   `target_identity`, `toolchain_identity`, `environment_capsule_ref`,
   `policy_and_trust`, and `provenance.input_decisions` shape; only the
   `surface` and `toolchain_class` differ.

## How to verify

```
cargo test -p aureline-runtime
```

The `aureline-runtime` crate runs:

- six unit tests covering nominal terminal/task/debug-prep resolution,
  the pending-trust degraded-field path, the explicit-override precedence
  drill, the surface-request precedence drill, and a fixture-driven
  conflicting-inputs replay,
- three integration tests under `tests/terminal_consumer.rs` that wire
  the resolver into [`aureline_terminal::pty_host::PtyHost`](../../crates/aureline-terminal/src/pty_host/mod.rs)
  and prove the session header quotes the resolved `execution_context_id`,
  cwd, trust posture, and target badge.

## Out of scope (M1)

- Full activator-decision orchestration (env-manager shims, venv
  activation, devcontainer build). The seed records the activation
  strategy class only.
- Full target reachability orchestration (warming, network probes,
  policy-blocked re-resolution).
- Full capsule materialisation. The seed quotes a reference and a hash;
  it does not materialise a capsule body.
- M2 task / debug depth, remote attach breadth, or hosted-account flows
  beyond the seed vocabulary required in M1.
