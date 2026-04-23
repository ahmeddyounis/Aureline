# Fixture: loading — warming index with progressive partial results

## Scenario

The workspace has reached `first_useful_navigation_reached` but
`semantic_warmup_completed` has not yet fired. The editor,
buffer, save pipeline, and recovery journal are live. The search
panel displays progressive partial results as the index warms.
The shell renders the warming posture explicitly; nothing
implies the semantic index is authoritative.

## Row bindings

- **Loading-state rows exercised.**
  `loading:top_of_pane_progress_indicator`,
  `loading:progressive_partial_results`,
  `loading:skeleton_row`, `loading:inline_placeholder`.
- **Empty-state row (intersection).** `empty:not_ready_warming`
  and `empty:not_ready_partial`.
- **Startup-state intersection.**
  `startup_state:warming_startup` per
  [`entry_restore_truth_audit.md`](../../../docs/ux/entry_restore_truth_audit.md)
  §6.6.
- **Lifecycle row.** `lifecycle:workspace`, state
  `workspace.partially_ready`.
- **Controlled label.** `Partially ready`.
- **Truth class.** `derived_indexed_truth` on the search panel;
  `user_authored_durable_truth` on the editor (not affected by
  warm-up).
- **Degraded-state token.** `Warming` → composes to `Partial`
  on the search row while rows stream in.

## Required axes rendered

- **`area_purpose`.** Search panel: "Search results — workspace
  + open-file scope."
- **Preserved.** Editor interactivity; buffer edits; save
  pipeline; recovery journal; all keyboard-reachable chrome;
  partial search rows that are authoritative for open files.
- **Narrowed.** `index_not_authoritative`,
  `semantic_lookups_pending`, `extensions_not_yet_activated`.
- **Next-safe action hooks.** `open_minimal`, `set_up_later`,
  `continue_in_restricted_mode`.

## Loading pattern detail

- **`top_of_pane_progress_indicator`.** Pinned chip reading
  "Partially ready — warming semantic index" with an explicit
  cancel / repair affordance. Names what is warming and the
  expected ready signal.
- **`progressive_partial_results`.** Search rows stream in as
  they resolve. Partial rows carry a typed `result_truth_label`
  (e.g., `partial_index`); authoritative rows (open-file range)
  render without the partial chip.
- **`skeleton_row`.** Matches the final row anatomy (title,
  path, preview snippet). Collapses to the authoritative row
  without reflow.
- **`inline_placeholder`.** "Semantic lookup pending" per
  row when the row is present but its semantic metadata has
  not yet resolved.

## Placement

- **Failure tier.** None (loading, not failing).
- **Delivery surfaces.** `status_item` (warming chip), owning
  surface inline (skeleton rows, inline placeholders),
  `durable_job_row` mirror for the warm-up job.
- **Interruptibility tier.** `tier_durable` (the warm-up is
  long-running and mirrored to the activity centre).

## Recovery / support / measurement

- **Recovery-ladder rung.** `rung.cache_index_repair` (on
  repeated warm-up failure).
- **Support-packet family.** `performance_evidence_packet`.
- **Journey-trace class.** `startup_to_first_useful_chrome`,
  `open_edit_save`.
- **Protected metrics.** `ff.warm_start_to_first_paint`,
  `ff.first_paint`.

## Accessibility

- Screen-reader announces "Partially ready" on first render
  and "Ready" once `semantic_warmup_completed` fires. The
  status-item chip is reachable from the keyboard focus ring.
- Skeleton rows collapse without reordering above-the-fold
  authoritative rows.

## Forbidden copy and patterns on this path

- "Working..."
- "Loading..."
- "Please wait"
- "Index complete" while `semantic_warmup_completed` has not
  fired
- Whole-shell spinner occluding the palette, breadcrumbs, or
  activity rail
- Indeterminate spinner replacing the partial-result chip

## Expected observable outcomes

- The warming posture is rendered verbatim; no banner claims
  readiness before `semantic_warmup_completed`.
- Durable mirror exists (activity centre row) for the warm-up
  job.
- Partial rows carry a typed `result_truth_label` and above-
  the-fold authoritative rows do not reorder on arrival.
- `overclaims_readiness = false` is asserted on the rendered
  view.

## Fixture fields (seed)

```yaml
__fixture__:
  name: loading_warming_index
  taxonomy_rows:
    - loading:top_of_pane_progress_indicator
    - loading:progressive_partial_results
    - loading:skeleton_row
    - loading:inline_placeholder
    - empty:not_ready_warming
    - empty:not_ready_partial
  startup_state_intersection: startup_state:warming_startup
  controlled_label: "Partially ready"
  doc_section: docs/ux/state_and_recovery_taxonomy.md#6.1
running_build_identity_ref: build-identity-seed-state-loading-warming
overclaims_readiness: false
```
