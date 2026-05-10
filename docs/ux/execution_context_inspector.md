# Execution-context inspector

This document is the reviewer-facing entry point for the M1 execution-context
inspector. It points at the canonical Rust projection in
[`crates/aureline-shell/src/runtime/context_inspector/mod.rs`](../../crates/aureline-shell/src/runtime/context_inspector/mod.rs),
the upstream object model in
[`crates/aureline-runtime/src/execution_context/mod.rs`](../../crates/aureline-runtime/src/execution_context/mod.rs),
and the failure drills under
[`fixtures/runtime/context_inspector_cases/`](../../fixtures/runtime/context_inspector_cases/).

The inspector is the protected-row surface a user opens when they need to
answer "which target, runtime, trust posture, and policy epoch is this run
using — and why?" without scanning logs. It is reachable from the terminal,
task seed, and debug-prep seed entries; the same snapshot type backs all
three.

## What this seed owns

- one inspectable
  [`ExecutionContextInspectorSnapshot`](../../crates/aureline-shell/src/runtime/context_inspector/mod.rs)
  Rust object that projects every
  [`aureline_runtime::ExecutionContext`](../../crates/aureline-runtime/src/execution_context/mod.rs)
  field into a labeled row, tagged with the resolver's winning input source
  and the conflicting sources that lost the precedence contest;
- a stable section layout — invocation subject, target, toolchain,
  environment capsule, policy and trust, scope, cache, provenance, honesty
  markers — that does not shrink when a context degrades;
- a stable action set — `copy_context`, `view_resolver_details`,
  `open_target_settings`, `return_to_invoking_surface` — the chrome routes
  through its existing command surface;
- typed honesty markers
  ([`InspectorMissingFieldReason`](../../crates/aureline-shell/src/runtime/context_inspector/mod.rs))
  for fields the resolver did not settle, fields the seed object
  intentionally covers as a subset of the boundary schema, and seed
  placeholders the chrome should label rather than omit.

The inspector is a thin projection. It does not own resolver vocabulary,
mint a separate cwd / target model, or re-derive provenance locally. The
upstream contract from
[`crates/aureline-runtime/src/execution_context/mod.rs`](../../crates/aureline-runtime/src/execution_context/mod.rs)
is the schema of record.

## Opening surfaces

The snapshot carries an
[`InspectorOpeningSurface`](../../crates/aureline-shell/src/runtime/context_inspector/mod.rs)
tag so the chrome can render the framing copy that matches the lane that
opened the inspector:

| Opening surface | Reached from | Notes |
|---|---|---|
| `terminal` | bottom-panel terminal pane | Default for terminal sessions. |
| `task` | task-seed channel header | Default for task lanes. |
| `debug_prep` | debug-prep stub | Default for debug-prep lanes. |
| `support_flow` | provider/auth, status bar, support / export entries | Used when a flow opens the inspector on a context resolved by another lane. |

The rows themselves do not change between surfaces — the snapshot is the
same record every consumer reads. Only the framing tag differs.

## Stable section layout

The inspector renders these sections in this order. A section is always
present; when its rows reduce to honesty markers (e.g. an empty degraded
list) the section still shows so a degraded snapshot is never silently
smaller than a green snapshot.

1. **Invocation** — command id, surface, actor, workspace, profile.
2. **Target** — target class, canonical id, working directory,
   reachability, local-vs-managed boundary cue. Each precedence-bearing row
   carries the winning input source and the conflicting sources that lost.
3. **Toolchain** — class, id, resolved version, activation strategy, and a
   degraded-fallback row when the resolver fell back.
4. **Environment capsule** — capsule id, hash, schema, drift state.
5. **Policy and trust** — trust state, identity mode, policy epoch.
6. **Scope** — workset scope class.
7. **Cache reuse** — cache disposition.
8. **Why this launch?** — provenance record id, resolver version, recorded
   timestamp, confidence level, and one row per recorded resolver decision
   so the user can quote `"target_class won by explicit_override; surface_requested
   and workspace_default lost"` verbatim without re-deriving precedence.
9. **Honesty markers** — one row per
   [`DegradedFieldRecord`](../../crates/aureline-runtime/src/execution_context/mod.rs)
   on the upstream context. When the resolver settles every field cleanly
   the section renders one explicit "None — every field resolved cleanly"
   row instead of shrinking out of existence.

## Honesty markers, never silent omission

The inspector never silently omits a field. When the resolver did not
settle a value, the row carries an
[`InspectorMissingFieldReason`](../../crates/aureline-shell/src/runtime/context_inspector/mod.rs):

- `resolver_unsettled` — no caller, surface, or workspace input contributed
  a value, and the resolver did not fall back. The chrome renders the row
  with a `Not settled by resolver` label.
- `prototype_limited_to_m1_seed` — the field is in the boundary schema but
  intentionally outside the M1 seed. The chrome renders the row with a
  `Reserved (M1 seed limit)` label.
- `seed_placeholder_awaiting_wiring` — the field exists upstream but the
  inspector seed does not yet project it. The chrome renders a
  `Seed placeholder` label so the gap is visible.

The snapshot's `honesty_marker_present` flag is `true` whenever any row
carries a missing-field reason or a degraded reason. The chrome MUST render
a visible honesty chip while that flag is set.

## Protected walk

1. Open a terminal session → click the title-context bar's "Inspect
   context" entry (or invoke the equivalent task / debug-prep seed entry).
2. Review the resolved target class, working directory, and toolchain
   rows; each carries the winning input source and any conflicting sources.
3. Read the **Why this launch?** section to confirm the resolver decision
   trail (precedence rows, provenance record id, resolver version, recorded
   timestamp).
4. Return to the invoking surface via the `return_to_invoking_surface`
   action without losing the open session, task, or debug-prep state.

## Failure drill

The fixture suite under
[`fixtures/runtime/context_inspector_cases/`](../../fixtures/runtime/context_inspector_cases/)
exercises the drill end to end:

- [`conflicting_inputs_remote_target.json`](../../fixtures/runtime/context_inspector_cases/conflicting_inputs_remote_target.json):
  the terminal pane requests a local target while the caller supplies an
  explicit remote override with a different cwd. The inspector quotes the
  override-resolved values, records the conflicting sources on the
  precedence row, and lights the local-vs-managed boundary cue.
- [`partially_resolved_terminal.json`](../../fixtures/runtime/context_inspector_cases/partially_resolved_terminal.json):
  a terminal request is opened against a workspace whose authority has not
  yet supplied a default working directory or profile and whose trust
  posture is pending evaluation. The inspector surfaces honesty-marker
  rows on every unsettled field rather than silently omitting them.

Both fixtures replay through the unit tests in
[`crates/aureline-shell/src/runtime/context_inspector/tests.rs`](../../crates/aureline-shell/src/runtime/context_inspector/tests.rs)
so the live shell projection and the recorded fixtures stay in lockstep.

## How to verify

```
cargo test -p aureline-shell context_inspector
```

This runs the unit tests for nominal terminal/task/debug-prep projections,
the explicit-override precedence drill, the pending-trust honesty-marker
drill, the missing-working-directory drill, the multi-degraded-field path,
the plaintext rendering, and the two fixture-driven failure drills.

## Out of scope (M1)

- full activator-decision orchestration (env-manager shims, venv
  activation, devcontainer build);
- full target reachability orchestration (warming, network probes,
  policy-blocked re-resolution);
- full capsule materialisation;
- M2 task / debug depth, remote attach breadth, or hosted-account flows
  beyond the seed vocabulary required in M1.

When any of these arrive they extend the upstream
[`aureline_runtime::ExecutionContext`](../../crates/aureline-runtime/src/execution_context/mod.rs)
contract first; the inspector picks up the new fields by widening its
section projection — never by forking a parallel copy of resolver truth.
