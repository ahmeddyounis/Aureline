# Proof packet: execution-context seed lane

Purpose: anchor proof captures for the M1 execution-context object model
and resolver seed that mints contexts for the terminal, task, and
debug-prep lanes through one API. Surfaces (terminal pane, task seed,
debug-prep seed, provider/auth entry points, activity center, status
bar, support / export flows) read structured execution-context records
through this seed; they do not derive runtime truth from terminal state
alone.

Reviewer landing page: [`docs/runtime/execution_context_seed.md`](../../../docs/runtime/execution_context_seed.md).

Canonical sources:

- Crate: `crates/aureline-runtime/`
  - `src/lib.rs` — public re-exports
  - `src/execution_context/mod.rs` — object model, resolver seed,
    precedence, provenance, degraded-field vocabulary
- Cross-tool boundary schema:
  `schemas/runtime/execution_context.schema.json`
- Failure-drill fixture:
  `fixtures/runtime/execution_context_seed_cases/conflicting_inputs.json`
- Integration test (named consumer wiring):
  `crates/aureline-runtime/tests/terminal_consumer.rs`

Protected walk: open the bottom-panel terminal pane against a resolved
execution-context, then resolve task and debug-prep seed contexts through
the same resolver. Confirm every lane projects the same canonical record
shape (`schema_version`, `scope_class`, `identity_mode`, `trust_state`,
`target_class`, provenance row vocabulary). Evidence:
`crates/aureline-runtime/tests/terminal_consumer.rs::task_and_debug_seeds_resolve_through_the_same_resolver_and_match_terminal_context_shape`.

Failure drill: resolve an execution context with conflicting `cwd` and
`target_class` inputs (terminal pane requests local with one cwd while
the caller passes an explicit override that targets an SSH remote with a
different cwd). Confirm the canonical object records the winning source
on `provenance.input_decisions[]`, lists the losing sources in
`conflicting_sources`, and lights the local-vs-managed boundary cue on
the projected `SessionHeader`. Evidence:
`fixtures/runtime/execution_context_seed_cases/conflicting_inputs.json`,
`aureline_runtime::execution_context::tests::explicit_override_wins_over_surface_request_and_records_conflict`,
`crates/aureline-runtime/tests/terminal_consumer.rs::explicit_override_to_remote_target_lights_the_boundary_cue_on_the_session_header`.

Validation command:

```
cargo test -p aureline-runtime
```

Evidence storage:

- Crate sources: `crates/aureline-runtime/`
- Reviewer doc: `docs/runtime/execution_context_seed.md`
- Boundary schema: `schemas/runtime/execution_context.schema.json`
- Failure-drill fixture: `fixtures/runtime/execution_context_seed_cases/`
- Integration test: `crates/aureline-runtime/tests/terminal_consumer.rs`
