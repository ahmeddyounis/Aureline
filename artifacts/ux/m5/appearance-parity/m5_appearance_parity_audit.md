# M5 appearance and density qualification audit

Generated from the seeded audit in
[`crate::m5_appearance_parity`](../../../../crates/aureline-shell/src/m5_appearance_parity/mod.rs).
Regenerate with:

```sh
cargo run -q -p aureline-shell --bin aureline_shell_m5_appearance_parity -- report-md > \
  artifacts/ux/m5/appearance-parity/m5_appearance_parity_audit.md
```

- Report id: `shell:m5_appearance_parity:audit:v1`
- Source schema ref: `schemas/ux/m5-appearance-qualification.schema.json`
- Registered M5 surfaces: `10`
- High-salience surfaces: `7`
- Marketed surfaces: `10`
- Appearance rows checked: `80`
- Blocking findings: `0`
- Narrowable marketed rows: `0`
- Status: **clean**
- Generated at: `2026-06-11T00:00:00Z`

## Per-row coverage

| Appearance row | Qualified | Narrowed | Unqualified | Missing evidence |
| -------------- | --------: | -------: | ----------: | ---------------: |
| Dark theme | 10 | 0 | 0 | 0 |
| Light theme | 10 | 0 | 0 | 0 |
| High-contrast theme | 10 | 0 | 0 | 0 |
| Compact density | 10 | 0 | 0 | 0 |
| Standard density | 10 | 0 | 0 | 0 |
| Comfortable density | 10 | 0 | 0 | 0 |
| Reduced motion | 9 | 1 | 0 | 0 |
| Live appearance change | 9 | 1 | 0 | 0 |

## Findings summary

| Class | Count |
| ----- | ----: |
| `unqualified_local_appearance` | 0 |
| `missing_evidence` | 0 |
| `missing_screenshot_pack` | 0 |
| `contrast_below_threshold` | 0 |
| `focus_not_visible` | 0 |
| `state_semantics_lost` | 0 |
| `reopen_target_lost` | 0 |
| `boundary_cue_hidden` | 0 |
| `motion_not_downgraded` | 0 |
| `keyboard_check_failed` | 0 |
| `screen_reader_check_failed` | 0 |
| `live_change_layout_corruption` | 0 |
| `live_change_focus_loss` | 0 |
| `live_change_state_corruption` | 0 |
| `stale_evidence_on_marketed_row` | 0 |
| `dimension_drift` | 0 |
| `missing_narrowing_reason` | 0 |
| `missing_projection` | 0 |
| `descriptor_missing_appearance_anchor` | 0 |
| `missing_accessibility_note` | 0 |
| `surface_not_on_appearance_session` | 0 |

## Appearance anchor index

| Surface family | Surface | Appearance anchor |
| -------------- | ------- | ----------------- |
| Companion surface | `surface:companion.surface` | `appearance:anchor:companion:surface` |
| Result-grid row | `surface:data_api.result_grid_row` | `appearance:anchor:data_api:result_grid_row` |
| Docs / browser pane | `surface:docs_browser.pane` | `appearance:anchor:docs_browser:pane` |
| Notebook cell chrome | `surface:notebook.cell_chrome` | `appearance:anchor:notebook:cell_chrome` |
| Offboarding surface | `surface:offboarding.surface` | `appearance:anchor:offboarding:surface` |
| Preview-route badge | `surface:preview.route_badge` | `appearance:anchor:preview:route_badge` |
| Profiler panel | `surface:profiler.capture_panel` | `appearance:anchor:profiler:capture_panel` |
| Pipeline card | `surface:review.pipeline_card` | `appearance:anchor:review:pipeline_card` |
| Sync status surface | `surface:sync.status_surface` | `appearance:anchor:sync:status_surface` |
| Trace panel | `surface:trace.replay_panel` | `appearance:anchor:trace:replay_panel` |

## Per-surface rows

### `surface:companion.surface` (companion_surface, beta)

- Descriptor revision: `surface-rev:companion.surface:2026.06.01-01`
- Semantic salience: `trust_bearing`
- Appearance anchor: `appearance:anchor:companion:surface`
- Marketed on desktop rows: `yes`
- High-salience: `yes`

| Appearance row | Status | Contrast | Focus | Motion | State | Reopen | Boundary | Freshness | Narrowing reason |
| -------------- | ------ | -------- | ----- | ------ | ----- | ------ | -------- | --------- | ---------------- |
| Dark theme | `qualified` | `meets_aa` | `visible_focus_ring` | `-` | `preserved` | `exact_target_preserved` | `present` | `fresh` | - |
| Light theme | `qualified` | `meets_aa` | `visible_focus_ring` | `-` | `preserved` | `exact_target_preserved` | `present` | `fresh` | - |
| High-contrast theme | `qualified` | `meets_aaa` | `visible_focus_ring` | `-` | `preserved` | `exact_target_preserved` | `present` | `fresh` | - |
| Compact density | `qualified` | `-` | `visible_focus_ring` | `-` | `preserved` | `exact_target_preserved` | `present` | `fresh` | - |
| Standard density | `qualified` | `-` | `visible_focus_ring` | `-` | `preserved` | `exact_target_preserved` | `present` | `fresh` | - |
| Comfortable density | `qualified` | `-` | `visible_focus_ring` | `-` | `preserved` | `exact_target_preserved` | `present` | `fresh` | - |
| Reduced motion | `declared_capture_gap` | `-` | `-` | `-` | `-` | `-` | `-` | `-` | companion_provider_drives_its_own_motion_so_the_reduced_motion_downgrade_is_provider_attributed |
| Live appearance change | `qualified` | `-` | `visible_focus_ring` | `-` | `preserved` | `exact_target_preserved` | `present` | `fresh` | - |

Findings: none.

### `surface:data_api.result_grid_row` (result_grid_row, beta)

- Descriptor revision: `surface-rev:data_api.result_grid_row:2026.06.01-01`
- Semantic salience: `lifecycle_bearing`
- Appearance anchor: `appearance:anchor:data_api:result_grid_row`
- Marketed on desktop rows: `yes`
- High-salience: `yes`

| Appearance row | Status | Contrast | Focus | Motion | State | Reopen | Boundary | Freshness | Narrowing reason |
| -------------- | ------ | -------- | ----- | ------ | ----- | ------ | -------- | --------- | ---------------- |
| Dark theme | `qualified` | `meets_aa` | `visible_focus_ring` | `-` | `preserved` | `exact_target_preserved` | `present` | `fresh` | - |
| Light theme | `qualified` | `meets_aa` | `visible_focus_ring` | `-` | `preserved` | `exact_target_preserved` | `present` | `fresh` | - |
| High-contrast theme | `qualified` | `meets_aaa` | `visible_focus_ring` | `-` | `preserved` | `exact_target_preserved` | `present` | `fresh` | - |
| Compact density | `qualified` | `-` | `visible_focus_ring` | `-` | `preserved` | `exact_target_preserved` | `present` | `fresh` | - |
| Standard density | `qualified` | `-` | `visible_focus_ring` | `-` | `preserved` | `exact_target_preserved` | `present` | `fresh` | - |
| Comfortable density | `qualified` | `-` | `visible_focus_ring` | `-` | `preserved` | `exact_target_preserved` | `present` | `fresh` | - |
| Reduced motion | `qualified` | `-` | `visible_focus_ring` | `reduced` | `preserved` | `exact_target_preserved` | `present` | `fresh` | - |
| Live appearance change | `qualified` | `-` | `visible_focus_ring` | `-` | `preserved` | `exact_target_preserved` | `present` | `fresh` | - |

Findings: none.

### `surface:docs_browser.pane` (docs_browser_pane, beta)

- Descriptor revision: `surface-rev:docs_browser.pane:2026.06.01-01`
- Semantic salience: `informational`
- Appearance anchor: `appearance:anchor:docs_browser:pane`
- Marketed on desktop rows: `yes`
- High-salience: `no`

| Appearance row | Status | Contrast | Focus | Motion | State | Reopen | Boundary | Freshness | Narrowing reason |
| -------------- | ------ | -------- | ----- | ------ | ----- | ------ | -------- | --------- | ---------------- |
| Dark theme | `qualified` | `meets_aa` | `visible_focus_ring` | `-` | `preserved` | `exact_target_preserved` | `-` | `fresh` | - |
| Light theme | `qualified` | `meets_aa` | `visible_focus_ring` | `-` | `preserved` | `exact_target_preserved` | `-` | `fresh` | - |
| High-contrast theme | `qualified` | `meets_aaa` | `visible_focus_ring` | `-` | `preserved` | `exact_target_preserved` | `-` | `fresh` | - |
| Compact density | `qualified` | `-` | `visible_focus_ring` | `-` | `preserved` | `exact_target_preserved` | `-` | `fresh` | - |
| Standard density | `qualified` | `-` | `visible_focus_ring` | `-` | `preserved` | `exact_target_preserved` | `-` | `fresh` | - |
| Comfortable density | `qualified` | `-` | `visible_focus_ring` | `-` | `preserved` | `exact_target_preserved` | `-` | `fresh` | - |
| Reduced motion | `qualified` | `-` | `visible_focus_ring` | `reduced` | `preserved` | `exact_target_preserved` | `-` | `fresh` | - |
| Live appearance change | `declared_capture_gap` | `-` | `-` | `-` | `-` | `-` | `-` | `-` | embedded_provider_repaints_on_its_own_cadence_so_live_change_capture_is_provider_attributed |

Findings: none.

### `surface:notebook.cell_chrome` (notebook_cell_chrome, beta)

- Descriptor revision: `surface-rev:notebook.cell_chrome:2026.06.01-01`
- Semantic salience: `lifecycle_bearing`
- Appearance anchor: `appearance:anchor:notebook:cell_chrome`
- Marketed on desktop rows: `yes`
- High-salience: `yes`

| Appearance row | Status | Contrast | Focus | Motion | State | Reopen | Boundary | Freshness | Narrowing reason |
| -------------- | ------ | -------- | ----- | ------ | ----- | ------ | -------- | --------- | ---------------- |
| Dark theme | `qualified` | `meets_aa` | `visible_focus_ring` | `-` | `preserved` | `exact_target_preserved` | `present` | `fresh` | - |
| Light theme | `qualified` | `meets_aa` | `visible_focus_ring` | `-` | `preserved` | `exact_target_preserved` | `present` | `fresh` | - |
| High-contrast theme | `qualified` | `meets_aaa` | `visible_focus_ring` | `-` | `preserved` | `exact_target_preserved` | `present` | `fresh` | - |
| Compact density | `qualified` | `-` | `visible_focus_ring` | `-` | `preserved` | `exact_target_preserved` | `present` | `fresh` | - |
| Standard density | `qualified` | `-` | `visible_focus_ring` | `-` | `preserved` | `exact_target_preserved` | `present` | `fresh` | - |
| Comfortable density | `qualified` | `-` | `visible_focus_ring` | `-` | `preserved` | `exact_target_preserved` | `present` | `fresh` | - |
| Reduced motion | `qualified` | `-` | `visible_focus_ring` | `reduced` | `preserved` | `exact_target_preserved` | `present` | `fresh` | - |
| Live appearance change | `qualified` | `-` | `visible_focus_ring` | `-` | `preserved` | `exact_target_preserved` | `present` | `fresh` | - |

Findings: none.

### `surface:offboarding.surface` (offboarding_surface, beta)

- Descriptor revision: `surface-rev:offboarding.surface:2026.06.01-01`
- Semantic salience: `severity_bearing`
- Appearance anchor: `appearance:anchor:offboarding:surface`
- Marketed on desktop rows: `yes`
- High-salience: `yes`

| Appearance row | Status | Contrast | Focus | Motion | State | Reopen | Boundary | Freshness | Narrowing reason |
| -------------- | ------ | -------- | ----- | ------ | ----- | ------ | -------- | --------- | ---------------- |
| Dark theme | `qualified` | `meets_aa` | `visible_focus_ring` | `-` | `preserved` | `not_applicable` | `present` | `fresh` | - |
| Light theme | `qualified` | `meets_aa` | `visible_focus_ring` | `-` | `preserved` | `not_applicable` | `present` | `fresh` | - |
| High-contrast theme | `qualified` | `meets_aaa` | `visible_focus_ring` | `-` | `preserved` | `not_applicable` | `present` | `fresh` | - |
| Compact density | `qualified` | `-` | `visible_focus_ring` | `-` | `preserved` | `not_applicable` | `present` | `fresh` | - |
| Standard density | `qualified` | `-` | `visible_focus_ring` | `-` | `preserved` | `not_applicable` | `present` | `fresh` | - |
| Comfortable density | `qualified` | `-` | `visible_focus_ring` | `-` | `preserved` | `not_applicable` | `present` | `fresh` | - |
| Reduced motion | `qualified` | `-` | `visible_focus_ring` | `reduced` | `preserved` | `not_applicable` | `present` | `fresh` | - |
| Live appearance change | `qualified` | `-` | `visible_focus_ring` | `-` | `preserved` | `not_applicable` | `present` | `fresh` | - |

Findings: none.

### `surface:preview.route_badge` (preview_route_badge, beta)

- Descriptor revision: `surface-rev:preview.route_badge:2026.06.01-01`
- Semantic salience: `lifecycle_bearing`
- Appearance anchor: `appearance:anchor:preview:route_badge`
- Marketed on desktop rows: `yes`
- High-salience: `yes`

| Appearance row | Status | Contrast | Focus | Motion | State | Reopen | Boundary | Freshness | Narrowing reason |
| -------------- | ------ | -------- | ----- | ------ | ----- | ------ | -------- | --------- | ---------------- |
| Dark theme | `qualified` | `meets_aa` | `visible_focus_ring` | `-` | `preserved` | `exact_target_preserved` | `present` | `fresh` | - |
| Light theme | `qualified` | `meets_aa` | `visible_focus_ring` | `-` | `preserved` | `exact_target_preserved` | `present` | `fresh` | - |
| High-contrast theme | `qualified` | `meets_aaa` | `visible_focus_ring` | `-` | `preserved` | `exact_target_preserved` | `present` | `fresh` | - |
| Compact density | `qualified` | `-` | `visible_focus_ring` | `-` | `preserved` | `exact_target_preserved` | `present` | `fresh` | - |
| Standard density | `qualified` | `-` | `visible_focus_ring` | `-` | `preserved` | `exact_target_preserved` | `present` | `fresh` | - |
| Comfortable density | `qualified` | `-` | `visible_focus_ring` | `-` | `preserved` | `exact_target_preserved` | `present` | `fresh` | - |
| Reduced motion | `qualified` | `-` | `visible_focus_ring` | `reduced` | `preserved` | `exact_target_preserved` | `present` | `fresh` | - |
| Live appearance change | `qualified` | `-` | `visible_focus_ring` | `-` | `preserved` | `exact_target_preserved` | `present` | `fresh` | - |

Findings: none.

### `surface:profiler.capture_panel` (profiler_panel, beta)

- Descriptor revision: `surface-rev:profiler.capture_panel:2026.06.01-01`
- Semantic salience: `informational`
- Appearance anchor: `appearance:anchor:profiler:capture_panel`
- Marketed on desktop rows: `yes`
- High-salience: `no`

| Appearance row | Status | Contrast | Focus | Motion | State | Reopen | Boundary | Freshness | Narrowing reason |
| -------------- | ------ | -------- | ----- | ------ | ----- | ------ | -------- | --------- | ---------------- |
| Dark theme | `qualified` | `meets_aa` | `visible_focus_ring` | `-` | `preserved` | `not_applicable` | `-` | `fresh` | - |
| Light theme | `qualified` | `meets_aa` | `visible_focus_ring` | `-` | `preserved` | `not_applicable` | `-` | `fresh` | - |
| High-contrast theme | `qualified` | `meets_aaa` | `visible_focus_ring` | `-` | `preserved` | `not_applicable` | `-` | `fresh` | - |
| Compact density | `qualified` | `-` | `visible_focus_ring` | `-` | `preserved` | `not_applicable` | `-` | `fresh` | - |
| Standard density | `qualified` | `-` | `visible_focus_ring` | `-` | `preserved` | `not_applicable` | `-` | `fresh` | - |
| Comfortable density | `qualified` | `-` | `visible_focus_ring` | `-` | `preserved` | `not_applicable` | `-` | `fresh` | - |
| Reduced motion | `qualified` | `-` | `visible_focus_ring` | `reduced` | `preserved` | `not_applicable` | `-` | `fresh` | - |
| Live appearance change | `qualified` | `-` | `visible_focus_ring` | `-` | `preserved` | `not_applicable` | `-` | `fresh` | - |

Findings: none.

### `surface:review.pipeline_card` (pipeline_card, beta)

- Descriptor revision: `surface-rev:review.pipeline_card:2026.06.01-01`
- Semantic salience: `severity_bearing`
- Appearance anchor: `appearance:anchor:review:pipeline_card`
- Marketed on desktop rows: `yes`
- High-salience: `yes`

| Appearance row | Status | Contrast | Focus | Motion | State | Reopen | Boundary | Freshness | Narrowing reason |
| -------------- | ------ | -------- | ----- | ------ | ----- | ------ | -------- | --------- | ---------------- |
| Dark theme | `qualified` | `meets_aa` | `visible_focus_ring` | `-` | `preserved` | `exact_target_preserved` | `present` | `fresh` | - |
| Light theme | `qualified` | `meets_aa` | `visible_focus_ring` | `-` | `preserved` | `exact_target_preserved` | `present` | `fresh` | - |
| High-contrast theme | `qualified` | `meets_aaa` | `visible_focus_ring` | `-` | `preserved` | `exact_target_preserved` | `present` | `fresh` | - |
| Compact density | `qualified` | `-` | `visible_focus_ring` | `-` | `preserved` | `exact_target_preserved` | `present` | `fresh` | - |
| Standard density | `qualified` | `-` | `visible_focus_ring` | `-` | `preserved` | `exact_target_preserved` | `present` | `fresh` | - |
| Comfortable density | `qualified` | `-` | `visible_focus_ring` | `-` | `preserved` | `exact_target_preserved` | `present` | `fresh` | - |
| Reduced motion | `qualified` | `-` | `visible_focus_ring` | `reduced` | `preserved` | `exact_target_preserved` | `present` | `fresh` | - |
| Live appearance change | `qualified` | `-` | `visible_focus_ring` | `-` | `preserved` | `exact_target_preserved` | `present` | `fresh` | - |

Findings: none.

### `surface:sync.status_surface` (sync_status_surface, beta)

- Descriptor revision: `surface-rev:sync.status_surface:2026.06.01-01`
- Semantic salience: `severity_bearing`
- Appearance anchor: `appearance:anchor:sync:status_surface`
- Marketed on desktop rows: `yes`
- High-salience: `yes`

| Appearance row | Status | Contrast | Focus | Motion | State | Reopen | Boundary | Freshness | Narrowing reason |
| -------------- | ------ | -------- | ----- | ------ | ----- | ------ | -------- | --------- | ---------------- |
| Dark theme | `qualified` | `meets_aa` | `visible_focus_ring` | `-` | `preserved` | `not_applicable` | `present` | `fresh` | - |
| Light theme | `qualified` | `meets_aa` | `visible_focus_ring` | `-` | `preserved` | `not_applicable` | `present` | `fresh` | - |
| High-contrast theme | `qualified` | `meets_aaa` | `visible_focus_ring` | `-` | `preserved` | `not_applicable` | `present` | `fresh` | - |
| Compact density | `qualified` | `-` | `visible_focus_ring` | `-` | `preserved` | `not_applicable` | `present` | `fresh` | - |
| Standard density | `qualified` | `-` | `visible_focus_ring` | `-` | `preserved` | `not_applicable` | `present` | `fresh` | - |
| Comfortable density | `qualified` | `-` | `visible_focus_ring` | `-` | `preserved` | `not_applicable` | `present` | `fresh` | - |
| Reduced motion | `qualified` | `-` | `visible_focus_ring` | `reduced` | `preserved` | `not_applicable` | `present` | `fresh` | - |
| Live appearance change | `qualified` | `-` | `visible_focus_ring` | `-` | `preserved` | `not_applicable` | `present` | `fresh` | - |

Findings: none.

### `surface:trace.replay_panel` (trace_panel, beta)

- Descriptor revision: `surface-rev:trace.replay_panel:2026.06.01-01`
- Semantic salience: `informational`
- Appearance anchor: `appearance:anchor:trace:replay_panel`
- Marketed on desktop rows: `yes`
- High-salience: `no`

| Appearance row | Status | Contrast | Focus | Motion | State | Reopen | Boundary | Freshness | Narrowing reason |
| -------------- | ------ | -------- | ----- | ------ | ----- | ------ | -------- | --------- | ---------------- |
| Dark theme | `qualified` | `meets_aa` | `visible_focus_ring` | `-` | `preserved` | `not_applicable` | `-` | `fresh` | - |
| Light theme | `qualified` | `meets_aa` | `visible_focus_ring` | `-` | `preserved` | `not_applicable` | `-` | `fresh` | - |
| High-contrast theme | `qualified` | `meets_aaa` | `visible_focus_ring` | `-` | `preserved` | `not_applicable` | `-` | `fresh` | - |
| Compact density | `qualified` | `-` | `visible_focus_ring` | `-` | `preserved` | `not_applicable` | `-` | `fresh` | - |
| Standard density | `qualified` | `-` | `visible_focus_ring` | `-` | `preserved` | `not_applicable` | `-` | `fresh` | - |
| Comfortable density | `qualified` | `-` | `visible_focus_ring` | `-` | `preserved` | `not_applicable` | `-` | `fresh` | - |
| Reduced motion | `qualified` | `-` | `visible_focus_ring` | `reduced` | `preserved` | `not_applicable` | `-` | `fresh` | - |
| Live appearance change | `qualified` | `-` | `visible_focus_ring` | `-` | `preserved` | `not_applicable` | `-` | `fresh` | - |

Findings: none.

## Verification

```sh
cargo run -q -p aureline-shell --bin aureline_shell_m5_appearance_parity -- validate
cargo test -p aureline-shell --test m5_appearance_parity_fixtures
python3 tools/ci/m5/appearance_parity_check.py
```
