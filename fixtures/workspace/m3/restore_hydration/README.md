# Restore Hydration Fixtures

These fixtures are restore-hydration request bundles consumed by the restore
hydrator in [`crates/aureline-workspace/src/restore_hydrator/`](../../../../crates/aureline-workspace/src/restore_hydrator/).
Each bundle pairs one or more remembered window-topology snapshots with the
runtime restore environment (which extensions, remotes, providers, permissions,
workspace authority, and displays exist now).

Boundary schema:
[`/schemas/workspace/window_topology_snapshot.schema.json`](../../../../schemas/workspace/window_topology_snapshot.schema.json).
Emitted layout-restore provenance reuses
[`/schemas/workspace/pane_tree.schema.json`](../../../../schemas/workspace/pane_tree.schema.json).

| Fixture | What it proves |
| --- | --- |
| `all_ready_request.json` | Baseline: every dependency present, saved display connected at matching scale, saved bounds on-screen, no live panes — restore class is `exact_restore` with no placeholders and no display adjustments. |
| `multi_window_degraded_request.json` | Degraded multi-window restore: vanished external display, off-screen bounds, fullscreen window, dead terminal process, missing preview extension, and unreachable notebook remote. Both window shells and pane trees are rebuilt first, bounds snap back on-screen, missing surfaces become placeholders, and no command/kernel/remote session is replayed. |

The hydrator turns each request into a `RestoreHydrationOutcome` whose per-window
`LayoutRestoreProvenanceRecord` carries the restore class, phase trace,
display adjustments, placeholder results, and live-surface no-rerun outcomes.
The `RestoreHydrationSummary` projects the same restore class, missing-dependency
class, and remaining-manual-action vocabulary into diagnostics and support export.
