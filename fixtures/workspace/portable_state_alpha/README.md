# Workspace portable-state alpha fixtures

These protected fixtures exercise the workspace-specific portable-state
package body validated by
[`/schemas/workspace/portable_state_alpha.schema.json`](../../../schemas/workspace/portable_state_alpha.schema.json).

The package body composes with the existing portable-state manifest,
portable-profile, and pane-tree schemas. It does not inline live
authority, raw paths, credentials, source bytes, task commands, terminal
scrollback, or machine state roots.

**Coverage contract**

- every package separates workspace authority, window topology, profile
  defaults, local session context, and machine-local hints;
- at least one fixture contains live, context-only, and
  placeholder-only pane restore postures with stable pane ids;
- every export names the redaction rules and machine-local exclusions
  that explain why secrets, approval tickets, delegated credentials,
  live handles, and state roots are absent;
- display-topology changes must preserve visible bounds and stable pane
  ids while recording any compatible or layout-only fallback.

**Index**

| Fixture | Key coverage |
|---|---|
| [`core_round_trip_changed_display.json`](./core_round_trip_changed_display.json) | Core class separation, linked profile artifacts, live/context/placeholder pane posture, machine-local exclusions, and changed-display topology adjustment. |
