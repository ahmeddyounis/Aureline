# Dirty-region model and retained-frame behavior

This document describes the current dirty-region planning contract, how it is derived from queued damage events, and how the native shell retains and reuses the last useful frame to avoid blank flashes during incremental updates.

Companion packets and sources of truth:

- `docs/architecture/viewport_invalidation_and_composition_packet.md` (damage-class semantics and composition-layer rules)
- `artifacts/render/damage_classes.yaml` (machine-readable damage vocabulary)
- `artifacts/render/composition_layer_map.yaml` (machine-readable composition-layer roster)

## Coordinate space

Dirty rectangles are expressed in **physical pixels** (window client coordinate space). The rectangle type is `aureline_render::PixelRect` (x/y/width/height, all `u32`).

## Damage events and regions

Producers enqueue invalidation intent as `aureline_render::DamageEvent` records:

- `layer`: one composition-layer id (for example `render_layer.floating_surface`)
- `class`: one damage-class id (for example `render_damage.selection_overlay_only`)
- `region`: optional pixel-space region metadata

`region` uses `aureline_render::DamageRegion`:

- `Unspecified`: no concrete rectangle is available; consumers must assume a full-window repaint
- `Rect(PixelRect)`: one concrete pixel rectangle

The current implementation is deliberately conservative: any `Unspecified` region escalates the entire repaint plan to a full-window update.

## Planning rules

The planner is implemented in `crates/aureline-render/src/dirty_regions.rs` and produces a `DirtyRegionPlan` for one frame:

1. If the window bounds are empty, plan a full repaint (degenerate safety).
2. If any queued damage event has an unspecified region, plan a full repaint (conservative correctness).
3. Otherwise, collect the rectangular regions, clip them to the window bounds, and decide between:
   - `FullWindow` when the dirty coverage is “too large” or too fragmented
   - `Partial` when the dirty coverage is sufficiently bounded

Promotion thresholds (current):

- cap the number of dirty rectangles; promote to full when the cap is exceeded
- promote to full when the total dirty area exceeds a fixed fraction of the window area

These thresholds are intentionally simple and can be tightened once downstream surfaces consistently emit tighter damage regions.

## Retained frame behavior (native shell)

The GPU-backed native shell keeps a retained `0RGB` framebuffer in memory and updates it incrementally:

1. Drain a coalesced `CompositedFrame` from the `FrameScheduler`.
2. Compute a dirty-region plan from `DamageEvent` records.
3. If the plan is `FullWindow` (or the retained buffer was resized), rasterize the full shell into the retained buffer and upload the full texture.
4. If the plan is `Partial`, rasterize only the dirty clip rectangle into the retained buffer and upload only that dirty rectangle to the GPU texture.

This preserves the last useful pixels outside the dirty region, which prevents “blank” flashes on incremental updates and reduces avoidable full-surface uploads.

Current wiring lives in:

- `crates/aureline-shell/src/bootstrap/native_shell.rs` (retained buffer + clipped raster)
- `crates/aureline-render/src/backend/wgpu_blit.rs` (`render_0rgb_dirty` partial texture uploads)

## Practical examples (current shell)

The native shell emits localized damage for:

- Start Center row selection changes (`render_damage.selection_overlay_only`, scoped to the focused editor group)
- Command palette list/query updates (scoped to the command-palette panel rectangle)
- Placeholder “open file/tab” actions (`render_damage.text_reflow_local`, scoped to the focused editor group)

When geometry is not known (first paint, resize, overlay open/close, etc.), the shell falls back to full-window invalidation.

