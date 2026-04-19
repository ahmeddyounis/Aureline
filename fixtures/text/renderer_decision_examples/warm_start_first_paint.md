# Fixture: warm-start first paint

## Scenario

The desktop process was launched recently enough that its binary and
common data pages are resident in the OS page cache. The user
relaunches the application; the session restores its last workspace
and reopens a small editor pane on a primary monitor.

## Hooks exercised

- `warm_start_to_first_paint` — measured from process launch to the
  first non-blank frame being presented.
- `first_paint` — fires for the first frame of every surface in the
  restored session.
- `frame_submit` — first GPU submission for the primary surface.

## Stack elements stressed

- winit-class windowing boot path and swap-chain creation.
- Shape-cache warmth: frequently-used glyphs for the editor theme
  should be present in the on-disk cache layer so the first frame
  does not wait on cold shaping for common clusters.
- Atlas-per-scale-bucket initialisation on the target monitor's
  scale.
- Accessibility-tree publication: the first `accessibility_tree_update`
  is expected alongside or immediately after `first_paint`.

## Expected observable outcomes

- `warm_start_to_first_paint` records a single latency sample per
  launch against the claimed hardware matrix row.
- `first_paint` reports one entry per top-level surface restored.
- `accessibility_tree_update` publishes an initial tree that covers
  the visible surfaces; glyphs visible without tree nodes are a
  defect, not a late-tree optimisation.
- `degraded_renderer_banner` MUST NOT fire on the claimed GPU targets
  during a warm start.

## ADR sections motivating this fixture

- Protected-hot-path hook list — `warm_start_to_first_paint`,
  `first_paint`, `frame_submit`.
- Accessibility bridge — tree publication coincident with first
  paint.
- Benchmark-measurement expectations — reproducibility pack records
  GPU target, driver, and display configuration at measurement time.
