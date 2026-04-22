# Workspace layout serialization fixtures

These fixtures are short, reviewable scenarios that anchor the
vocabulary frozen in
[`/docs/workspace/layout_serialization_contract.md`](../../../docs/workspace/layout_serialization_contract.md)
and validated by the schema at
[`/schemas/workspace/pane_tree.schema.json`](../../../schemas/workspace/pane_tree.schema.json).

Each fixture names the record kind it exercises, the boundary or phase
classes it covers, and the contract section it motivates.

**Scope rules**

- Fixtures validate against the single pane-tree schema; they do not
  encode runtime restore-engine internals, OS window-manager handles, or
  raw capability tickets.
- A new fixture MUST exercise at least one node kind, one restore phase,
  one placeholder reason, one live-surface guardrail, or one
  multi-window boundary rule, and MUST cite the contract section that
  motivates it.
- Stable IDs and monotonic timestamps are opaque and chosen for review
  clarity rather than to reflect real machine state.
- `workspace_authority_ref`, `profile_defaults_ref`, and
  `machine_display_hint_ref` are boundaries, not invitations to inline
  those records into the fixture.

**Index**

| Fixture | Record kind | Key classes exercised | Doc section |
|---|---|---|---|
| [`window_topology_snapshot_presentation_aux.json`](./window_topology_snapshot_presentation_aux.json) | `window_topology_snapshot_record` | shared `workspace_authority_ref` + auxiliary presentation window / split + tab-group + leaf nodes / focus chain / visible inspectors / follow + presentation state / monitor-affinity hint | §1 Boundary and portability model, §2 Multi-window ownership rules, §3 Pane-tree schema seed |
| [`layout_restore_provenance_missing_dependencies.json`](./layout_restore_provenance_missing_dependencies.json) | `layout_restore_provenance_record` | all five restore phases / `missing_extension` + `missing_remote` + `non_reentrant_live_surface` placeholders / display-topology adjustments / terminal + notebook no-rerun guardrails | §4 Restore phases, §5 Placeholder and degradation rules, §6 Live-surface no-rerun rules |

**Coverage contract**

The seed fixture set MUST keep:

- at least one snapshot record showing a shared workspace authority with
  window-local topology;
- at least one restore provenance record naming every restore phase;
- at least one example of a missing-dependency placeholder that retains
  evidence and surrounding layout truth;
- at least one example of a live surface preserved without command
  rerun.
