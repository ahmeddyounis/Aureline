# Spike metric names — mapping to protected-path concepts

This note freezes the mapping between the ADR-0002 protected-hot-path
hook vocabulary and the protected-path journey-budget concepts the
benchmark lab and the journey harness measure against. It is the
short human-readable companion to the machine-readable boundary at
[`schemas/traces/spike_timing.schema.json`](../../schemas/traces/spike_timing.schema.json)
and the committed examples under
[`artifacts/traces/examples/`](../../artifacts/traces/examples/).

The hook names themselves come from
[ADR 0002 — Renderer text-stack and shaping-fallback](../adr/0002-renderer-text-stack-and-shaping-fallback.md)
§Protected-hot-path hook list. This document does not introduce new
hook names; it assigns each hook to one protected-path bucket so that
two hooks that share a bucket measure against the same journey
budget.

## Why a bucket rather than per-hook budgets

Protected-path budgets are stated in product terms (startup, input
response, render submission, placeholder file lifecycle) rather than
in widget-local terms (what the caret did, which atlas evicted a
glyph). A trace consumer that only knows hook names would have to
re-derive the grouping for every hook it sees. Freezing the mapping
here means the journey harness and the benchmark lab consume one
protected-path token per mark and route to one budget, without
re-deriving the rule from a table of hook names.

## Protected-path vocabulary

| `protected_path`        | What it measures                                                                                                 | Journey-budget concept                                      |
|-------------------------|------------------------------------------------------------------------------------------------------------------|-------------------------------------------------------------|
| `startup`               | Cold-to-warm process entry; the first non-blank frame on the claimed hardware matrix                             | Startup journey                                             |
| `first_useful_chrome`   | First state where placeholder chrome (title / status / sidebar) is live enough to read (reserved, not yet fired) | Startup journey (sub-phase)                                 |
| `first_paint`           | First frame emitted by any surface in a session, including on reopen from a hidden state                         | Per-surface first-paint budget                              |
| `input_to_paint`        | Input boundary to the frame that reflects the input                                                              | Input-response journey                                      |
| `render_submission`     | Compositor submits a frame to the GPU surface                                                                    | Per-frame submission budget                                 |
| `frame_budget`          | Steady-state per-frame budget under load (reserved, not yet fired)                                               | Frame-budget journey                                        |
| `placeholder_open`      | Placeholder file-open path exists but is not yet exercised by the M0 spike                                       | Placeholder-open journey (reserved)                         |
| `placeholder_edit`      | Placeholder file-edit path exists but is not yet exercised by the M0 spike                                       | Placeholder-edit journey (reserved)                         |
| `placeholder_save`      | Placeholder file-save path exists but is not yet exercised by the M0 spike                                       | Placeholder-save journey (reserved)                         |
| `fallback_resolution`   | Shaping fallback, atlas shard rebind, or atlas eviction                                                          | Fallback-resolution observability                           |
| `observability`         | Observability-only hooks not on a budgeted journey                                                               | None — observability only                                   |

The `first_useful_chrome`, `frame_budget`, and `placeholder_*` buckets
are reserved slots: they are admissible in
`schemas/traces/spike_timing.schema.json` so later lanes wire in
without a schema version bump, but the M0 spike does not fire hooks
in those buckets. The committed example traces under
`artifacts/traces/examples/` record them as zero counts.

## Hook → protected-path mapping

| Hook (ADR-0002 name)            | `protected_path`        | Protected hot-path | Notes                                                                                                        |
|---------------------------------|-------------------------|--------------------|--------------------------------------------------------------------------------------------------------------|
| `warm_start_to_first_paint`     | `startup`               | yes                | The single startup mark the spike always emits before any input action.                                      |
| `first_paint`                   | `first_paint`           | yes                | Paired with `warm_start_to_first_paint` at scene entry.                                                     |
| `scroll_frame`                  | `input_to_paint`        | yes                | Rides the text-and-decoration layer.                                                                         |
| `caret_move`                    | `input_to_paint`        | yes                | Rides the overlay layer; never re-rasters glyphs.                                                            |
| `selection_change`              | `input_to_paint`        | yes                | Rides the overlay layer.                                                                                     |
| `ime_composition_update`        | `input_to_paint`        | yes                | Rides the overlay layer.                                                                                     |
| `reflow_line_range`             | `input_to_paint`        | yes                | Text insertion and line reflow; may re-shape clusters.                                                       |
| `multi_monitor_scale_change`    | `input_to_paint`        | yes                | Scale / DPI bucket change; drops raster caches and re-shapes.                                                |
| `fallback_glyph_resolution`     | `fallback_resolution`   | yes                | Fires when a glyph resolves through fallback stage ≥ 2.                                                     |
| `atlas_shard_rebind`            | `fallback_resolution`   | yes                | Follows `multi_monitor_scale_change` when the atlas shard changes.                                           |
| `atlas_eviction`                | `fallback_resolution`   | no                 | Observability only — evictions do not gate release.                                                         |
| `frame_submit`                  | `render_submission`     | yes                | Compositor frame submission; closes every scene.                                                             |
| `degraded_renderer_banner`      | `observability`         | no                 | Fires only on software fallback or adapter loss; does not gate release.                                     |
| `accessibility_tree_update`     | `observability`         | yes                | On the hot-path list but not on a journey budget the M0 spike measures; buckets as observability for now.   |

The canonical code for this mapping is
`aureline_shell_spike::timing_trace::protected_path_for`. A later
lane that moves `accessibility_tree_update` out of the observability
bucket MUST update this table, the schema, and the code in one
change.

## Counter vocabulary

The trace record carries one counters block per emitted trace. The
fields below are the full set; the schema requires all of them even
when the spike's M0 wiring always records zero.

| Counter                            | Wired in M0 spike? | What it counts                                                                                         |
|------------------------------------|---------------------|--------------------------------------------------------------------------------------------------------|
| `total_marks`                      | yes                 | Marks in the trace.                                                                                    |
| `hot_path_marks`                   | yes                 | Marks whose hook is on ADR-0002's protected hot-path list.                                             |
| `damage_records`                   | yes                 | Damage records emitted by the render path during the trace.                                            |
| `paint_count_by_zone.*`            | yes                 | Damage records bucketed by shell zone (`editor_viewport`, `sidebar`, `title_bar`, `status_bar`).       |
| `paint_count_by_layer.*`           | yes                 | Damage records bucketed by ADR-0002 scene layer (`text_and_decoration`, `overlay`).                    |
| `invalidation_class_counts.*`      | yes                 | Marks bucketed by `protected_path`. Provisional buckets that are never fired remain at zero.           |
| `visible_pane_work`                | yes                 | Damage records against visible panes. In the M0 spike every pane is visible; equals `damage_records`.  |
| `hidden_pane_work`                 | provisional         | Damage records against hidden panes. The M0 spike has no hidden panes; always zero.                    |
| `frame_misses`                     | provisional         | Frames that missed their budget. The M0 spike submits one frame and never misses; always zero.         |
| `offscreen_suppression_eligible`   | provisional         | Damage records that could be suppressed because their rect falls off-screen. Provisional; always zero. |
| `fallback_glyph_resolutions`       | yes                 | Marks whose hook is `fallback_glyph_resolution`. The default fixture scene never fires this hook.      |

"Provisional" means the counter is wired to zero in the M0 spike and
is reserved for later lanes to fill in. A trace consumer that needs
one of these counters MUST read the schema, not the code, to know
which counters are real in a given build.

## Regeneration

Regenerate the committed example traces with:

```
cargo run --bin shell_spike -- --emit-timing-traces artifacts/traces/examples
```

The committed examples under
[`artifacts/traces/examples/`](../../artifacts/traces/examples/) were
captured on a Linux build host and therefore record
`host_os = linux` and `rustc_target_triple = unknown-linux-gnu`. A
regeneration on macOS or Windows will rewrite those two fields; the
other fields are host-independent and stay byte-stable across a
regenerate / diff cycle.
