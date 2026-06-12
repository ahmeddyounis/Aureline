# M5 workset-scope descriptor packet

This document describes the canonical packet that carries the **M5 workset-scope
descriptor** — the single active scope snapshot every M5 code-understanding surface binds to
so slice boundaries stay visible and durable. Where the
[graph-governance matrix](m5-graph-governance.md) freezes *which depth claim* each lane may
publish, this packet answers the prior question every graph-backed surface must answer first:
**what slice of the workspace am I actually looking at — the full workspace, a sparse slice,
a named workset, or a policy-limited view?**

It is the user-facing companion to the governed artifact at
`artifacts/graph/m5/m5-workset-scope.json` and the typed model in the `aureline-graph` crate
(`m5_workset_scope`).

## What this packet covers

The packet reuses the stable scope vocabulary from the graph explainers rather than minting
separate workset hints for search, explainers, review, onboarding, or AI context, so every
surface narrows from one shared model. It carries three things:

### 1. One active scope snapshot

The `active_snapshot` is the replay anchor. It records a stable `snapshot_id`, the date it was
`taken_as_of`, and the canonical workset-scope `descriptor` it bounds:

- **`scope_id`** and **`scope_class`** — the stable id and class of the active slice (for
  example a `named_workset`).
- **`included_roots_or_repos`** — the workspace roots or repository labels in scope.
- **`scope_mode`** — `full` when the descriptor covers the whole workspace, `sparse` when it
  covers a slice or hot set.
- **`scope_source`** — `local` when the scope came from local workspace state, `managed` when
  it came from managed or provider state.
- **`hidden_result_count`** — results hidden by the workset, policy, or unloaded-scope
  boundary.
- **`index_coverage`** — `covered_node_count`, `covered_edge_count`, `not_loaded_count`, and a
  compact `coverage_state` token, so the user can tell how much of the workspace remains out
  of scope.

A **full** scope hides and unloads nothing: a non-zero `hidden_result_count` or
`not_loaded_count` on a full scope is a validation failure, because that would be a slice
masquerading as the whole workspace.

### 2. Explicit and suggested scope-change actions

`scope_change_actions` make widening and narrowing explicit and reviewable. Each action
records a `direction` (`widen` or `narrow`), an `actuation` (`explicit` user action or
`suggested`), a `target_scope_id`, a `requires_review` flag, and a `summary`.

The **no-silent-broadening** invariant: a `widen` action — and any `suggested` action —
must set `requires_review: true`. A suggestion may exist, but a graph-backed feature may not
silently broaden beyond the active slice; scope widening always happens through an explicit,
reviewable action. The packet must offer at least one explicit widen action.

### 3. One consumer binding per M5 code-understanding surface

`consumer_bindings` carry the scope descriptor into every surface that could otherwise leave
its slice boundary implicit:

1. **`docs_recall`** — documentation recall over the workspace knowledge pack.
2. **`topology_view`** — topology node/edge views.
3. **`architecture_explainer`** — the generated architecture explainer.
4. **`review_explanation`** — review explanation of changed code.
5. **`onboarding_tour`** — the onboarding tour.
6. **`ai_context_assembly`** — AI context assembly.

Each binding records the active `snapshot_id` and `scope_id` it renders, an
`implies_full_workspace` flag, a `consumer_ref` to the surface artifact that ingests the
binding, and a `note`.

Two invariants hold across the bindings:

- **No slice masquerading as the whole workspace.** `implies_full_workspace` may only be
  `true` when the active descriptor is `full`. An explainer, impact card, or AI-context
  binding never implies whole-workspace knowledge while it is bound to a sparse slice.
- **Replayable scope.** Every binding is stamped with the active `snapshot_id` and `scope_id`,
  so a later support export or replay can reconstruct exactly which scope the user queried
  instead of guessing from result content.

## How the packet narrows surfaces

The typed model recomputes the summary counts and validates the snapshot, actions, and
bindings. `export_projection` produces the redaction-safe scope index that downstream
surfaces — release evidence, help/service-health, docs badges, and support exports — render
instead of restating the active slice by hand. The packet binds upstream to the canonical
graph-depth governance matrix (`governance_matrix_ref`) and the scope-provenance truth packet
(`source_packet_ref`) it extends, so the shared scope model has one provenance root.

## Guardrails

- A widen action or a suggestion that is not reviewable is a validation failure
  (`SilentBroadening`).
- A binding that claims whole-workspace knowledge over a sparse slice is a validation failure
  (`FullWorkspaceClaimOverSlice`).
- A binding not stamped with the active snapshot id or scope id is a validation failure
  (`SnapshotBindingMismatch`, `ScopeIdMismatch`).
- Every consumer surface must carry exactly one binding (`MissingSurfaceBinding`,
  `DuplicateSurfaceBinding`), so no surface leaves its scope boundary implicit.

## Out of scope

This packet does not broaden the scope model into unrelated v1 search surfaces until those
surfaces ingest the canonical descriptor.
