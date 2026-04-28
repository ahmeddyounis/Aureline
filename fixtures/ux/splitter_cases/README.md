# Splitter fixtures

Seed corpus for the contract frozen in
[`/docs/ux/splitter_contract.md`](../../../docs/ux/splitter_contract.md)
and the boundary schema at
[`/schemas/ux/splitter_state.schema.json`](../../../schemas/ux/splitter_state.schema.json).

Each fixture is a single JSON document that exercises one splitter or
resizable-pane case without exposing raw paths, raw logs, raw command
lines, raw provider payloads, credentials, live authority handles, or
planning identifiers. Identity is carried through opaque refs and
redaction-aware labels.

## Cases

| Fixture | Record kind | Main proof |
| --- | --- | --- |
| [`editor_terminal_keyboard_resize.json`](./editor_terminal_keyboard_resize.json) | `splitter_control_record` | The editor/terminal splitter has a thin visible line, larger hit target, focus reinforcement, screen-reader region naming, fine/coarse keyboard resize, reset, and equalize routes. |
| [`durable_job_collapse_denied.json`](./durable_job_collapse_denied.json) | `pane_collapse_decision_record` | A bottom-panel collapse requested for neighboring space is denied because durable work and blocking problems would otherwise disappear silently. |
| [`inspector_collapsed_recoverable.json`](./inspector_collapsed_recoverable.json) | `pane_collapse_decision_record` | A non-critical inspector can collapse to a visible summary only when a stub, toggle, and command-backed reopen route remain available. |
| [`topology_density_restore_proportional.json`](./topology_density_restore_proportional.json) | `splitter_persistence_record` | A saved proportional layout survives monitor, zoom, and density changes by applying weights, clamping minimums, sheeting the inspector, and preserving recovery routes instead of trusting stale pixels. |

## Intended usage

- Shell and renderer code can assert hit-target floors, no-layout-shift
  reinforcement, and keyboard resize parity with pointer drag.
- Accessibility tests can verify named adjustable separators and
  controlled-region refs.
- Restore and persistence code can verify that proportions or named
  presets, not raw pixel positions, drive layout recovery.
- Support export code can verify collapse decisions without exporting
  workspace content.
