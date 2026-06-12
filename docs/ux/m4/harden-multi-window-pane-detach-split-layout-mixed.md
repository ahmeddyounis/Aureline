# Multi-window, pane-detach, split-layout, mixed-DPI, and multi-monitor restore — contract

This is the reviewer-facing companion for the stable lane that hardens
**desktop window restore** to Aureline's durable truth model: one governed
record per window reopen that binds **authority / topology separation**, a
**versioned pane tree with stable pane IDs**, **skeleton-first / hydrate-second
restore**, **restore-no-rerun honesty**, **display-topology and downgrade
provenance**, and a **public claim ceiling** with an automatic
narrow-below-Stable verdict.

This lane sits one level above the startup warm-restore lane
([`/docs/ux/m4/harden_shell_startup_warm_restore_and_first_useful.md`](./harden_shell_startup_warm_restore_and_first_useful.md)).
Where that lane brings one window back to a first-useful state, this lane proves
that *every* reopened window — primary, secondary, review, inspector — recreates
its pane topology truthfully across pane detach, splits, mixed-DPI docks, and
multi-monitor change, and never silently reacquires the live authority behind a
terminal, debugger, notebook, query console, preview route, profiler capture,
incident workspace, or remote-backed pane.

Do not clone status text from this doc — ingest the canonical machine sources:

- Records / fixtures:
  [`/fixtures/ux/m4/harden-multi-window-pane-detach-split-layout-mixed/`](../../../fixtures/ux/m4/harden-multi-window-pane-detach-split-layout-mixed/)
- Schema:
  [`/schemas/ux/harden-multi-window-pane-detach-split-layout-mixed.schema.json`](../../../schemas/ux/harden-multi-window-pane-detach-split-layout-mixed.schema.json)
- Release-evidence packet:
  [`/artifacts/ux/m4/harden-multi-window-pane-detach-split-layout-mixed.md`](../../../artifacts/ux/m4/harden-multi-window-pane-detach-split-layout-mixed.md)
- Typed source: `aureline_shell::window_topology_restore_stable` (`model`, `corpus`)
- Headless emitter: `aureline_shell_window_topology_restore_stable`
- Replay + invariant gate:
  `crates/aureline-shell/tests/window_topology_restore_stable_fixtures.rs`

## Why one governed restore record

Restoring a desktop session fuses two separable problems that competitors
routinely conflate: rebuilding the **window topology** (which windows existed,
their pane trees, focus history, and visible surfaces) and resuming the
**session-scoped execution** behind a pane (a live terminal, a debug session, a
notebook kernel, a preview server, a remote-backed surface). When they are
fused, a restored window silently reacquires live authority — a terminal re-runs
a deploy, a debugger reattaches, a remote pane reconnects without consent. When
the display topology changed underneath the save — a monitor unplugged, a
mixed-DPI dock cycle, a wake-from-sleep bounds shift — the window can also reopen
off-screen, at the wrong scale, or collapse panes whose extension or remote
target is now absent.

This lane mints one governed `window_topology_restore_record` per window reopen.
It does **not** reinvent the pane-tree vocabulary, the topology-change classes,
or the restore adjustments: each record is a genuine projection of the live
windows workspace-management page (`aureline_shell::windows`) and the
restore-provenance contract. The record binds, for one window identity:

- **Authority / topology separation.** Workspace authority — dirty buffers,
  recovery journals, trust and policy, VFS identity, and attached execution
  contexts — stays centralized and shared across the windows that view one
  workspace. Pane-tree layout, focus history, zoom/follow state, and visible
  surfaces stay window-local. The record proves the two are not fused; a reopen
  whose authority is fused with its topology is minted but narrowed.
- **A versioned pane tree with stable pane IDs.** Split, move, float, pin, and
  close-pane mutate one versioned `PaneTree` keyed by stable pane IDs. The tree
  carries `pane_tree_schema_version`; a leaf without a slot, a slot outside the
  tree, a duplicate pane ID, or a zero-weight split is a hard error.
- **Skeleton-first / hydrate-second restore.** The pane structure is recreated
  first; session-scoped panes (terminal, debugger, notebook, query console,
  preview route, profiler capture, incident workspace, remote-backed, AI, test,
  pipeline) then hydrate into a truthful placeholder or reconnect state. A
  session-scoped pane that hydrates *live* — silently reacquiring authority —
  is rejected outright.
- **Restore-no-rerun honesty.** Every session-scoped pane that did not survive
  keeps its slot with an in-place placeholder card carrying a restore-no-rerun
  state (`transcript_restored`, `session_ended`, `reconnect_available`,
  `rerun_required`, `context_unavailable`). The card forbids command rerun and
  authority reacquire until an explicit user action and offers at least one
  recovery action.
- **Display-topology and downgrade provenance.** Monitor removal, safe-bounds
  change, scale change, wake/reconnect, and docking change are recorded with the
  adjustments applied to reconcile them. Any downgrade from Exact to Compatible,
  Layout-only, or placeholder-backed records its from/to fidelity and a named
  reason, so the layout change is explainable — not surprising — in diagnostics
  and support export without scraping localized UI copy.
- **A public claim ceiling** and **automatic narrowing**: a reopen that cannot
  prove a pillar, or whose lowest binding surface marker is below Stable, narrows
  below Stable with a named reason instead of inheriting an adjacent green row.

## The claimed-stable matrix

| Record | Posture | Claim | Surface marker | Fidelity | Demonstrates |
| --- | --- | --- | --- | --- | --- |
| `exact_single_window.json` | Single window, exact reopen | **stable** | stable | Exact | a vertical editor split reopened exactly; both editor panes re-read truthfully from durable buffer state with no side effect |
| `mixed_dpi_multi_monitor_compatible.json` | Mixed-DPI dock, compatible reopen | **stable** | stable | Compatible | a mixed-DPI dock cycle adjusts scale and bounds; layout intent preserved, panes hydrate live, downgrade recorded with provenance |
| `monitor_removed_placeholder_backed.json` | Monitor removed, placeholder-backed recovery | **stable** | stable | Placeholder-backed | an external display detaches and the terminal, pipeline, query console, notebook, preview route, docs, profiler, and incident runtimes do not survive; the panes keep their slots with restore-no-rerun placeholder cards instead of collapsing |
| `help_about_preview_surface.json` | Exact reopen, Help/About surface in preview | preview (narrowed) | preview | Exact | narrow-below-Stable by the lowest binding surface marker (`surface_not_yet_stable`) |
| `authority_topology_leak_drill.json` | Authority/topology fusion drill | beta (narrowed) | stable | Exact | the lane detects workspace authority fused with window topology and narrows with `authority_topology_not_separated` |

Coverage verdict: **3 Stable, 2 narrowed**. The narrowed rows each name a reason
(`surface_not_yet_stable`; `authority_topology_not_separated`) and drop below the
launch cutline instead of inheriting an adjacent green row.

## How the pillars are derived (not asserted)

The builder *computes* every pillar from the validated reopen, so a record can
never publish a claim wider than its proof:

- **Authority / topology separation** holds only when every required
  workspace-authority class is centralized and every window-local topology class
  is window-local. Fusing the two drops `authority_topology_separated`.
- **Pane-tree versioning** holds only when the schema version is current and the
  tree is structurally sound (every leaf has a slot, every slot is in the tree,
  pane IDs are unique, no zero-weight split).
- **Skeleton-first / hydrate-second** holds because a session-scoped pane may
  never hydrate live: it is either `skeleton_recreated`, hydrated as a
  re-readable surface, or preserved by a `placeholder_card`.
- **No silent rerun or reacquire** holds because every placeholder pane forbids
  command rerun and authority reacquire and carries a restore-no-rerun state and
  a recovery action; an exact reopen may carry no downgrade, and any non-exact
  reopen must carry one whose target fidelity matches the resulting fidelity.
- **Restore provenance is export-safe** because the fidelity, topology changes,
  adjustments, and downgrade reason are all carried as closed tokens, not prose.
- **Recovery chrome is reachable** when title context, restore details, command
  palette, keyboard focus, and the activity center all survive the reopen.

The **claim ceiling** is then checked against the derived pillars: a reopen may
not assert a pillar it cannot prove (a build error), and any pillar that is false
adds a named narrowing reason and drops the claim below Stable.

## Surfaces that ingest this record

The desktop restore-review surface, the CLI restore inspector, Help/About, the
diagnostics support export, and docs read this record verbatim. The same reopen
is reachable, keyboard-first, from the activity center, command palette, status
bar, and a menu command, across normal, high-contrast, and zoomed layouts, and
stays available without an account or managed services.

## Regenerating the fixtures

```sh
cargo run -q -p aureline-shell \
  --bin aureline_shell_window_topology_restore_stable -- emit-fixtures \
  fixtures/ux/m4/harden-multi-window-pane-detach-split-layout-mixed
```

The replay gate
(`crates/aureline-shell/tests/window_topology_restore_stable_fixtures.rs`) fails
if the on-disk JSON drifts from the in-code projection.
