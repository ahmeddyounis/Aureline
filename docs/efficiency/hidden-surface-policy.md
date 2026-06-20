# Hidden-surface render suppression

Richer product surfaces — notebooks, traces, previews, docs/browser panes,
pipelines, and incident workspaces — keep background work alive: they paint,
animate, refresh rich content, and poll. When the user is no longer looking at
one, that work should stop instead of quietly draining battery, GPU, or
background budget. The hidden-surface policy makes that shedding explicit and
inspectable, and it proves the suppression never loses user-owned state.

It builds on the canonical [efficiency state](./efficiency-state.md): the same
typed `EfficiencyState`, `VisibilityState`, and frozen hidden-pane behaviour
vocabulary drive the per-surface decisions, so the low-power story stays one
object rather than per-surface prose.

## What the policy governs

`aureline_shell::efficiency::hidden_surfaces` models one decision per surface.
Each governed [`HiddenSurfaceClass`] declares the work channels it can keep
alive:

| Channel | Correctness-critical | Examples |
| --- | --- | --- |
| `paint` | no | committed render passes |
| `animation` | no | decorative or non-essential motion |
| `rich_refresh` | no | re-render of live preview/document content |
| `speculative_poll` | no | prefetch, background refresh, status polling |
| `correctness_poll` | yes | a running notebook cell, trace event capture, pipeline completion, an incident feed |

The split is the whole point. Decorative paint, animation, rich-preview
refresh, and speculative polling are **dropped to zero** while a surface is
hidden or off-screen. Correctness-critical channels are only **throttled to a
non-zero floor** — never silently dropped — so resuming the surface restores
truthful state.

## The three cases

A surface's `VisibilityState` resolves to one of three activities:

- **active** (`visible_focused`) — contributing to the active task; nothing is
  suppressed.
- **visible_inactive** (`visible_background`) — an inactive preview the user can
  still glance at; it keeps painting and keeps its correctness work, but
  decorative motion, rich refresh, and speculative polling throttle down (and
  stop entirely under `ProtectCore`).
- **hidden** (`occluded_window`, `hidden_tab`, `collapsed_split`,
  `detached_offscreen`) — paint, animation, rich refresh, and speculative
  polling are suppressed; the correctness channel throttles to a floor.

The correctness floor scales with the efficiency state: full cadence under
`Nominal`, halved under battery/thermal/recovery pressure, and the minimum of
one poll under `ProtectCore` — but always at least one when work was requested.

## Resume stays correct

Every class carries a [`ResumeContinuityContract`] that names exactly what is
restored and asserts the restore neither re-runs suppressed work nor corrupts a
private cache:

| Class | Restored on resume |
| --- | --- |
| `notebook` | Kernel session and last committed cell outputs; no cell re-runs. |
| `trace` | Buffered trace events; nothing dropped or replayed. |
| `preview` | Last truthful snapshot; refresh resumes without a surprise rebuild. |
| `docs_browser` | Loaded document and scroll position from cache; no navigation replayed. |
| `pipeline` | Run state reconciled from its source; the run never restarts. |
| `incident` | Buffered incident events replayed in order; the feed is never truncated. |

This is the guardrail: suppression may pause optional work, but it may not lose
user-owned state or create resume-time correctness errors.

## Audit, energy/thermal trace, and diagnostics

The per-surface decisions roll up into one
[`HiddenSurfaceSuppressionAudit`], which proves no hidden surface kept forbidden
work alive (`passes_policy`), that every suppressed surface resumes correctly
(`all_resumes_correct`), and attributes the saved work to specific surface
classes (`saved_by_class`).

Two surfaces project from that audit:

- [`HiddenSurfaceEnergyTrace`] — attributes saved paint, animation, refresh, and
  polling to each class and emits per-surface trace marks, so an energy/thermal
  trace can answer "what did hiding this pane save, and where did it come from?"
- [`HiddenSurfaceDiagnosticsProjection`] — an operator-facing view of the policy
  pass, the savings, and the preserved protected interactions.

The audit also projects into the coarse
`HiddenPaneRenderAudit`, so the per-class policy can never disagree with the
frozen hidden-pane render contract about whether a hidden pane painted.

## Sources of truth

- Code: `crates/aureline-shell/src/efficiency/hidden_surfaces/`
- Schema: `schemas/efficiency/hidden-surface-policy.schema.json`
- Fixtures: `fixtures/efficiency/hidden-pane-audits/`
- Conformance dump: `cargo run -p aureline-shell --example dump_hidden_surface_audits`

[`HiddenSurfaceClass`]: ../../crates/aureline-shell/src/efficiency/hidden_surfaces/mod.rs
[`ResumeContinuityContract`]: ../../crates/aureline-shell/src/efficiency/hidden_surfaces/mod.rs
[`HiddenSurfaceSuppressionAudit`]: ../../crates/aureline-shell/src/efficiency/hidden_surfaces/mod.rs
[`HiddenSurfaceEnergyTrace`]: ../../crates/aureline-shell/src/efficiency/hidden_surfaces/mod.rs
[`HiddenSurfaceDiagnosticsProjection`]: ../../crates/aureline-shell/src/efficiency/hidden_surfaces/mod.rs
