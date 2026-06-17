# Fixtures: environment status strips

This directory contains fixture metadata for the `m5_environment_status_strips` packet.

The canonical full corpus is checked in at:

`artifacts/support/m5/m5-execution-context-explainability.json`

It is the one authoritative environment-status-strip registry; the typed model and fail-closed status
gate live in the `aureline-runtime` crate (`m5_environment_status_strips`).

## Coverage

- All nine run-capable surfaces — `run`, `test`, `debug`, `notebook`, `request`, `database`, `preview`,
  `pipeline`, and `incident` — carry exactly one status strip, each projecting the execution-context
  resolver truth by reference and carrying a one-step explain entry plus the equivalent CLI / headless
  object id.
- The five context facets (`interpreter`, `sdk`, `shell`, `container`, `remote_target`) and the three
  freshness states (`fresh`, `stale`, `unknown`) are each exercised across the shown facets.
- The five status classes (`resolved`, `stale`, `blocked`, `remote_drift`, `conflicting`) are each
  exercised, and the published presentation covers `resolved` (`run`, `test`, `pipeline`), `flagged`
  (`debug`, `notebook`, `request`, `preview`, `incident`), and `blocked` (`database`).
- The four downgrade reasons — `stale_context` (the stale/unknown-facet strips), `blocked_environment`
  (`database`), `remote_drift` (`request`), and `conflicting_context` (`preview`) — are each exercised,
  and the five resolution paths — `refresh_target`, `reconnect_remote`, `resolve_conflict`,
  `unblock_environment`, and `none` — are each exercised.
- The gate is exercised in every direction: three strips resolve cleanly with all facets current and the
  status resolved, proving the gate is not a blanket flag; stale and unknown facets flag their strips with
  a refresh; `request` flags a drifted remote; `preview` surfaces a conflict instead of one chip; and
  `database` blocks and warns before the downstream statement runs. Each strip's `presentation`,
  `downgrade_reasons`, `resolution_path`, and `blocked_before_run` flag equal the recomputed gate, so the
  desktop-shell, support-center, support-export, issue-report-packet, and cli-headless surfaces ingest one
  registry and a flagged or blocked strip cannot read as a clean current-target chip.
