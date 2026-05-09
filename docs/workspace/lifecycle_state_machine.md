# Workspace lifecycle state machine

This document defines the canonical workspace lifecycle vocabulary used by
Aureline to report **readiness** honestly during workspace open, indexing, and
degraded operation.

The workspace lifecycle state machine is the source of truth for:

- status/readiness labels rendered in the shell (for example, `Partially Ready`);
- exportable snapshots and transition frames used for diagnostics and support; and
- deciding when the workspace should remain interactive for basic editing even
  while background work is still warming.

## States

The lifecycle is a single state machine with the following states (all are
snake-case when serialized):

- `discovered`: A candidate workspace target has been identified but no open has
  been initiated.
- `trust_evaluating`: The workspace is awaiting a trust decision (for example,
  restricted vs trusted).
- `opening`: The workspace is being opened and initial services are starting.
- `partially_ready`: The shell is interactive and basic editing/navigation are
  available, but one or more readiness gates are not yet satisfied.
- `ready`: The workspace is fully ready according to the readiness gates below.
- `degraded`: The workspace remains interactive but one or more critical gates
  have faulted (for example, watcher fallback polling or a trust downgrade).
- `closing`: A close has been requested and shutdown/teardown work is underway.
- `closed`: The workspace is closed and no longer interactive.

## Readiness gates

The `ready` state is derived from a small set of explicit gates. The current
implementation uses:

- `trust_state == trusted`
- `watcher_health == healthy`
- `hot_index_ready == true` (workspace file index is complete)
- `command_graph_ready == true` (command/registry projections needed for local navigation)

The `partially_ready` state is **not** a generic loading label: it is used when
the workspace is interactive but at least one gate remains unsatisfied.

The `degraded` state is entered when the machine observes an explicit fault that
would make “ready” dishonest (for example, `watcher_health` transitions to
`degraded`, `fallback_polling`, or `unavailable`, or trust is downgraded to
restricted).

## Interactivity contract

Consumers MUST treat `partially_ready`, `ready`, and `degraded` as interactive
states. The lifecycle machine exists to make background work visible without
blocking basic editing.

## Exported diagnostics

The shell emits JSON diagnostics for the current workspace lifecycle:

- `.logs/workspace/workspace_lifecycle_snapshot.json` (latest snapshot)
- `.logs/workspace/workspace_lifecycle_transitions.jsonl` (newline-delimited transition frames)

These logs are intended for support capture and proof artifacts; they are not a
stable external API contract yet.

