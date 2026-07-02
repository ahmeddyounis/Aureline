# M5 ambient shell-instrumentation stability

Generated from the seeded packet in
[`crate::m5_ambient_instrumentation_stability`](../../crates/aureline-shell/src/m5_ambient_instrumentation_stability/mod.rs).
Regenerate with:

```sh
cargo run -q -p aureline-shell --bin aureline_shell_m5_ambient_instrumentation_stability -- markdown > \
  artifacts/shell/m5-ambient-instrumentation-stability.md
```

- Packet id: `m5-ambient-instrumentation-stability:stable:0001`
- Source schema ref: `schemas/shell/m5-ambient-instrumentation-stability.schema.json`
- Certifies matrix packet: `m5-shell-primitives:stable:0001`
- Exact build: `build-id:aureline:stable:1.0.0:aarch64-apple-darwin:release:9f3c1a2`
- Release channel: `stable`
- Rows certified: 8
- Green: 4
- Yellow (auto-narrowed): 4
- Red (blocked): 0
- All rows publishable: `true`
- Blocking findings: 0
- Status: **clean**
- Generated at: `2026-06-30T00:00:00Z`

## Certification dimensions

- `counter_stability`
- `overflow_searchability`
- `grouped_summary`
- `stability_export`

## Certification rows

| Profile | Status | Qualification | Counter | Overflow | Grouped | Export | No-vanity-reflow | Waiver |
| ------- | ------ | ------------- | ------- | -------- | ------- | ------ | ---------------- | ------ |
| Standard desktop density | `green` | `stable` | `counter_spinner_summary_stable_no_reflow` | `overflow_items_palette_searchable_same_labels` | `multi_job_grouped_into_one_summary` | `stability_fixtures_and_export_reconstructable` | `true` | — |
| Compact / reduced-width density | `yellow` | `stable` | `disclosed_reduced_counter_detail` | `overflow_items_palette_searchable_same_labels` | `multi_job_grouped_into_one_summary` | `stability_fixtures_and_export_reconstructable` | `true` | — |
| Expanded / wide density | `green` | `stable` | `counter_spinner_summary_stable_no_reflow` | `overflow_items_palette_searchable_same_labels` | `multi_job_grouped_into_one_summary` | `stability_fixtures_and_export_reconstructable` | `true` | — |
| Multi-window / detached shell | `green` | `stable` | `counter_spinner_summary_stable_no_reflow` | `overflow_items_palette_searchable_same_labels` | `multi_job_grouped_into_one_summary` | `stability_fixtures_and_export_reconstructable` | `true` | — |
| High-zoom / large-text rendering | `yellow` | `stable` | `counter_spinner_summary_stable_no_reflow` | `disclosed_reduced_overflow_search_detail` | `multi_job_grouped_into_one_summary` | `stability_fixtures_and_export_reconstructable` | `true` | — |
| Reduced-motion rendering | `yellow` | `stable` | `counter_spinner_summary_stable_no_reflow` | `overflow_items_palette_searchable_same_labels` | `disclosed_coarse_grouping` | `stability_fixtures_and_export_reconstructable` | `true` | `waiver:reduced-motion-coarse-grouping:0001` |
| Degraded-network conditions | `yellow` | `stable` | `counter_spinner_summary_stable_no_reflow` | `overflow_items_palette_searchable_same_labels` | `multi_job_grouped_into_one_summary` | `disclosed_partial_capture` | `true` | — |
| Degraded-power / low-power conditions | `green` | `stable` | `counter_spinner_summary_stable_no_reflow` | `overflow_items_palette_searchable_same_labels` | `multi_job_grouped_into_one_summary` | `stability_fixtures_and_export_reconstructable` | `true` | — |

## Auto-narrowed rows

- `compact` (`yellow`) — Under the seeded compact profile a wide problem/background-work count abbreviates to a magnitude (for example `99+`) and a spinner label shortens to fit the reduced-width strip, while every counter keeps its stable placement, identity, and meaning; the reduction is disclosed and the row is narrowed below green.
- `high_zoom` (`yellow`) — Under the seeded high-zoom profile the status-menu overflow search shows a shorter explanation and groups low-priority results to fit the large-text layout, while every overflowed item stays discoverable from the palette/status search and keeps its original label; the reduction is disclosed and the row is narrowed below green.
- `reduced_motion` (`yellow`) — The reduced-motion profile folds distinct job classes into one summarized-work chip sooner than the standard threshold to avoid animating many primitives at once, while the summary stays meaningful and each job stays reachable; the coarse grouping is disclosed behind a waiver and never hides a job, so the row is narrowed below green while the reduction is in force.
- `degraded_network` (`yellow`) — Under the seeded degraded-network profile the support export reconstructs the status items, counters, and grouped summaries but discloses a partial capture of the low-priority overflow entries while the export queue is throttled; the partial capture is disclosed and the row is narrowed below green.

## Exact certification causes

- `compact` — `vanity_item_reflow` (disclosed: `true`) — Under this profile a counter's detail is disclosedly reduced (a wide count abbreviates to a magnitude, or a spinner label shortens) while the item keeps its stable placement, identity, and meaning; the reduction is disclosed and the row is narrowed below green.
- `high_zoom` — `hover_only_critical_truth` (disclosed: `true`) — Under this profile the overflow search shows a disclosedly reduced detail (a shorter explanation or a grouped result) while every overflowed item stays discoverable and keeps its original label; the reduction is disclosed and the row is narrowed below green.
- `reduced_motion` — `spinner_only_state` (disclosed: `true`) — Under this profile the grouping is disclosedly coarse (a summary folds distinct job classes into one chip sooner than the standard threshold) while the summary stays meaningful and each job stays reachable; the coarse grouping is disclosed and waivered, and the row is narrowed below green.
- `degraded_network` — `proof_stale` (disclosed: `true`) — The support export reconstructs the profile's ambient instrumentation and discloses a partial capture (some low-priority overflow entries are trimmed) while the reduction is disclosed and the row is narrowed below green.

## Active waivers

- `waiver:reduced-motion-coarse-grouping:0001` (`reduced_motion`, owner: Shell/status-bar owner, expires `2026-09-30T00:00:00Z`) — Under the seeded reduced-motion profile the shell folds distinct job classes into one summarized-work chip sooner than the standard threshold to avoid animating many primitives at once, while the summary stays meaningful, keeps its count, and each job stays reachable from the activity center. The coarse grouping is disclosed, never hides a job, and keeps the reopen path into durable history.

## Findings

Findings: none.

## Verification

```sh
cargo run -q -p aureline-shell --bin aureline_shell_m5_ambient_instrumentation_stability -- validate
cargo test -p aureline-shell --test m5_ambient_instrumentation_stability_fixtures
```
