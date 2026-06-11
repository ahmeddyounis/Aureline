# M5 desktop and handoff qualification audit

Generated from the seeded audit in
[`crate::m5_desktop_conformance`](../../../../crates/aureline-shell/src/m5_desktop_conformance/mod.rs).
Regenerate with:

```sh
cargo run -q -p aureline-shell --bin aureline_shell_m5_desktop_conformance -- report-md > \
  artifacts/ux/m5/desktop-conformance/m5_desktop_conformance_audit.md
```

- Report id: `shell:m5_desktop_conformance:audit:v1`
- Source schema ref: `schemas/platform/m5-surface-desktop-qualification.schema.json`
- Claimed desktop profiles: `macos`, `windows`, `linux`
- Registered M5 surfaces: `11`
- High-salience surfaces: `8`
- Marketed surfaces: `11`
- Desktop rows checked: `99`
- Blocking findings: `0`
- Narrowable marketed rows: `0`
- Status: **clean**
- Generated at: `2026-06-11T00:00:00Z`

## Per-row coverage

| Scenario row | Qualified | Narrowed | Unqualified | Missing evidence |
| ------------ | --------: | -------: | ----------: | ---------------: |
| Multi-window | 11 | 0 | 0 | 0 |
| Multi-monitor | 11 | 0 | 0 | 0 |
| Mixed-DPI | 11 | 0 | 0 | 0 |
| Suspend/resume | 11 | 0 | 0 | 0 |
| Battery saver | 11 | 0 | 0 | 0 |
| Thermal pressure | 10 | 1 | 0 | 0 |
| Deep link | 11 | 0 | 0 | 0 |
| File association | 10 | 1 | 0 | 0 |
| System-open return | 11 | 0 | 0 | 0 |

## Findings summary

| Class | Count |
| ----- | ----: |
| `unqualified_local_platform_path` | 0 |
| `missing_evidence` | 0 |
| `missing_evidence_pack` | 0 |
| `reopen_target_lost` | 0 |
| `layout_continuity_broken` | 0 |
| `interruption_unsafe` | 0 |
| `placeholder_misleading` | 0 |
| `authority_context_lost` | 0 |
| `background_not_throttled` | 0 |
| `handoff_reason_dropped` | 0 |
| `boundary_cue_hidden` | 0 |
| `stale_evidence_on_marketed_row` | 0 |
| `dimension_drift` | 0 |
| `missing_narrowing_reason` | 0 |
| `missing_projection` | 0 |
| `descriptor_missing_reopen_anchor` | 0 |
| `missing_continuity_note` | 0 |
| `missing_claimed_profiles` | 0 |
| `surface_not_on_platform_conformance` | 0 |

## Reopen anchor index

| Surface family | Surface | Reopen anchor |
| -------------- | ------- | ------------- |
| Companion surface | `surface:companion.surface` | `reopen:anchor:companion:surface` |
| Result-grid row | `surface:data_api.result_grid_row` | `reopen:anchor:data_api:result_grid_row` |
| Docs / browser pane | `surface:docs_browser.pane` | `reopen:anchor:docs_browser:pane` |
| Incident packet | `surface:incident.packet` | `reopen:anchor:incident:packet` |
| Notebook cell chrome | `surface:notebook.cell_chrome` | `reopen:anchor:notebook:cell_chrome` |
| Offboarding surface | `surface:offboarding.surface` | `reopen:anchor:offboarding:surface` |
| Preview-route badge | `surface:preview.route_badge` | `reopen:anchor:preview:route_badge` |
| Profiler panel | `surface:profiler.capture_panel` | `reopen:anchor:profiler:capture_panel` |
| Pipeline card | `surface:review.pipeline_card` | `reopen:anchor:review:pipeline_card` |
| Sync status surface | `surface:sync.status_surface` | `reopen:anchor:sync:status_surface` |
| Trace panel | `surface:trace.replay_panel` | `reopen:anchor:trace:replay_panel` |

## Per-surface rows

### `surface:companion.surface` (companion_surface, beta)

- Descriptor revision: `surface-rev:companion.surface:2026.06.01-01`
- Semantic salience: `trust_bearing`
- Reopen anchor: `reopen:anchor:companion:surface`
- Claimed profiles: `macos`, `windows`, `linux`
- Marketed on desktop rows: `yes`
- High-salience: `yes`

| Scenario row | Status | Reopen | Layout | Interruption | Placeholder | Authority | Background | Handoff | Boundary | Freshness | Narrowing reason |
| ------------ | ------ | ------ | ------ | ------------ | ----------- | --------- | ---------- | ------- | -------- | --------- | ---------------- |
| Multi-window | `qualified` | `exact_target_preserved` | `preserved` | `safe` | `honest` | `-` | `-` | `-` | `present` | `fresh` | - |
| Multi-monitor | `qualified` | `exact_target_preserved` | `preserved` | `safe` | `honest` | `-` | `-` | `-` | `present` | `fresh` | - |
| Mixed-DPI | `qualified` | `exact_target_preserved` | `preserved` | `safe` | `honest` | `-` | `-` | `-` | `present` | `fresh` | - |
| Suspend/resume | `qualified` | `exact_target_preserved` | `preserved` | `safe` | `honest` | `-` | `-` | `-` | `present` | `fresh` | - |
| Battery saver | `qualified` | `exact_target_preserved` | `preserved` | `safe` | `honest` | `-` | `throttled_before_corruption` | `-` | `present` | `fresh` | - |
| Thermal pressure | `declared_capture_gap` | `-` | `-` | `-` | `-` | `-` | `-` | `-` | `-` | `-` | companion_provider_owns_its_own_thermal_budget_so_the_throttle_capture_is_provider_attributed |
| Deep link | `qualified` | `exact_target_preserved` | `preserved` | `safe` | `honest` | `preserved` | `-` | `preserved` | `present` | `fresh` | - |
| File association | `qualified` | `exact_target_preserved` | `preserved` | `safe` | `honest` | `preserved` | `-` | `preserved` | `present` | `fresh` | - |
| System-open return | `qualified` | `exact_target_preserved` | `preserved` | `safe` | `honest` | `preserved` | `-` | `preserved` | `present` | `fresh` | - |

Findings: none.

### `surface:data_api.result_grid_row` (result_grid_row, beta)

- Descriptor revision: `surface-rev:data_api.result_grid_row:2026.06.01-01`
- Semantic salience: `lifecycle_bearing`
- Reopen anchor: `reopen:anchor:data_api:result_grid_row`
- Claimed profiles: `macos`, `windows`, `linux`
- Marketed on desktop rows: `yes`
- High-salience: `yes`

| Scenario row | Status | Reopen | Layout | Interruption | Placeholder | Authority | Background | Handoff | Boundary | Freshness | Narrowing reason |
| ------------ | ------ | ------ | ------ | ------------ | ----------- | --------- | ---------- | ------- | -------- | --------- | ---------------- |
| Multi-window | `qualified` | `exact_target_preserved` | `preserved` | `safe` | `honest` | `-` | `-` | `-` | `present` | `fresh` | - |
| Multi-monitor | `qualified` | `exact_target_preserved` | `preserved` | `safe` | `honest` | `-` | `-` | `-` | `present` | `fresh` | - |
| Mixed-DPI | `qualified` | `exact_target_preserved` | `preserved` | `safe` | `honest` | `-` | `-` | `-` | `present` | `fresh` | - |
| Suspend/resume | `qualified` | `exact_target_preserved` | `preserved` | `safe` | `honest` | `-` | `-` | `-` | `present` | `fresh` | - |
| Battery saver | `qualified` | `exact_target_preserved` | `preserved` | `safe` | `honest` | `-` | `throttled_before_corruption` | `-` | `present` | `fresh` | - |
| Thermal pressure | `qualified` | `exact_target_preserved` | `preserved` | `safe` | `honest` | `-` | `throttled_before_corruption` | `-` | `present` | `fresh` | - |
| Deep link | `qualified` | `exact_target_preserved` | `preserved` | `safe` | `honest` | `preserved` | `-` | `preserved` | `present` | `fresh` | - |
| File association | `qualified` | `exact_target_preserved` | `preserved` | `safe` | `honest` | `preserved` | `-` | `preserved` | `present` | `fresh` | - |
| System-open return | `qualified` | `exact_target_preserved` | `preserved` | `safe` | `honest` | `preserved` | `-` | `preserved` | `present` | `fresh` | - |

Findings: none.

### `surface:docs_browser.pane` (docs_browser_pane, beta)

- Descriptor revision: `surface-rev:docs_browser.pane:2026.06.01-01`
- Semantic salience: `informational`
- Reopen anchor: `reopen:anchor:docs_browser:pane`
- Claimed profiles: `macos`, `windows`, `linux`
- Marketed on desktop rows: `yes`
- High-salience: `no`

| Scenario row | Status | Reopen | Layout | Interruption | Placeholder | Authority | Background | Handoff | Boundary | Freshness | Narrowing reason |
| ------------ | ------ | ------ | ------ | ------------ | ----------- | --------- | ---------- | ------- | -------- | --------- | ---------------- |
| Multi-window | `qualified` | `exact_target_preserved` | `preserved` | `safe` | `honest` | `-` | `-` | `-` | `-` | `fresh` | - |
| Multi-monitor | `qualified` | `exact_target_preserved` | `preserved` | `safe` | `honest` | `-` | `-` | `-` | `-` | `fresh` | - |
| Mixed-DPI | `qualified` | `exact_target_preserved` | `preserved` | `safe` | `honest` | `-` | `-` | `-` | `-` | `fresh` | - |
| Suspend/resume | `qualified` | `exact_target_preserved` | `preserved` | `safe` | `honest` | `-` | `-` | `-` | `-` | `fresh` | - |
| Battery saver | `qualified` | `exact_target_preserved` | `preserved` | `safe` | `honest` | `-` | `throttled_before_corruption` | `-` | `-` | `fresh` | - |
| Thermal pressure | `qualified` | `exact_target_preserved` | `preserved` | `safe` | `honest` | `-` | `throttled_before_corruption` | `-` | `-` | `fresh` | - |
| Deep link | `qualified` | `exact_target_preserved` | `preserved` | `safe` | `honest` | `not_applicable` | `-` | `preserved` | `-` | `fresh` | - |
| File association | `declared_capture_gap` | `-` | `-` | `-` | `-` | `-` | `-` | `-` | `-` | `-` | embedded_provider_owns_os_file_handler_registration_so_the_association_capture_is_provider_attributed |
| System-open return | `qualified` | `exact_target_preserved` | `preserved` | `safe` | `honest` | `not_applicable` | `-` | `preserved` | `-` | `fresh` | - |

Findings: none.

### `surface:incident.packet` (incident_packet, beta)

- Descriptor revision: `surface-rev:incident.packet:2026.06.01-01`
- Semantic salience: `severity_bearing`
- Reopen anchor: `reopen:anchor:incident:packet`
- Claimed profiles: `macos`, `windows`, `linux`
- Marketed on desktop rows: `yes`
- High-salience: `yes`

| Scenario row | Status | Reopen | Layout | Interruption | Placeholder | Authority | Background | Handoff | Boundary | Freshness | Narrowing reason |
| ------------ | ------ | ------ | ------ | ------------ | ----------- | --------- | ---------- | ------- | -------- | --------- | ---------------- |
| Multi-window | `qualified` | `exact_target_preserved` | `preserved` | `safe` | `honest` | `-` | `-` | `-` | `present` | `fresh` | - |
| Multi-monitor | `qualified` | `exact_target_preserved` | `preserved` | `safe` | `honest` | `-` | `-` | `-` | `present` | `fresh` | - |
| Mixed-DPI | `qualified` | `exact_target_preserved` | `preserved` | `safe` | `honest` | `-` | `-` | `-` | `present` | `fresh` | - |
| Suspend/resume | `qualified` | `exact_target_preserved` | `preserved` | `safe` | `honest` | `-` | `-` | `-` | `present` | `fresh` | - |
| Battery saver | `qualified` | `exact_target_preserved` | `preserved` | `safe` | `honest` | `-` | `throttled_before_corruption` | `-` | `present` | `fresh` | - |
| Thermal pressure | `qualified` | `exact_target_preserved` | `preserved` | `safe` | `honest` | `-` | `throttled_before_corruption` | `-` | `present` | `fresh` | - |
| Deep link | `qualified` | `exact_target_preserved` | `preserved` | `safe` | `honest` | `preserved` | `-` | `preserved` | `present` | `fresh` | - |
| File association | `qualified` | `exact_target_preserved` | `preserved` | `safe` | `honest` | `preserved` | `-` | `preserved` | `present` | `fresh` | - |
| System-open return | `qualified` | `exact_target_preserved` | `preserved` | `safe` | `honest` | `preserved` | `-` | `preserved` | `present` | `fresh` | - |

Findings: none.

### `surface:notebook.cell_chrome` (notebook_cell_chrome, beta)

- Descriptor revision: `surface-rev:notebook.cell_chrome:2026.06.01-01`
- Semantic salience: `lifecycle_bearing`
- Reopen anchor: `reopen:anchor:notebook:cell_chrome`
- Claimed profiles: `macos`, `windows`, `linux`
- Marketed on desktop rows: `yes`
- High-salience: `yes`

| Scenario row | Status | Reopen | Layout | Interruption | Placeholder | Authority | Background | Handoff | Boundary | Freshness | Narrowing reason |
| ------------ | ------ | ------ | ------ | ------------ | ----------- | --------- | ---------- | ------- | -------- | --------- | ---------------- |
| Multi-window | `qualified` | `exact_target_preserved` | `preserved` | `safe` | `honest` | `-` | `-` | `-` | `present` | `fresh` | - |
| Multi-monitor | `qualified` | `exact_target_preserved` | `preserved` | `safe` | `honest` | `-` | `-` | `-` | `present` | `fresh` | - |
| Mixed-DPI | `qualified` | `exact_target_preserved` | `preserved` | `safe` | `honest` | `-` | `-` | `-` | `present` | `fresh` | - |
| Suspend/resume | `qualified` | `exact_target_preserved` | `preserved` | `safe` | `honest` | `-` | `-` | `-` | `present` | `fresh` | - |
| Battery saver | `qualified` | `exact_target_preserved` | `preserved` | `safe` | `honest` | `-` | `throttled_before_corruption` | `-` | `present` | `fresh` | - |
| Thermal pressure | `qualified` | `exact_target_preserved` | `preserved` | `safe` | `honest` | `-` | `throttled_before_corruption` | `-` | `present` | `fresh` | - |
| Deep link | `qualified` | `exact_target_preserved` | `preserved` | `safe` | `honest` | `preserved` | `-` | `preserved` | `present` | `fresh` | - |
| File association | `qualified` | `exact_target_preserved` | `preserved` | `safe` | `honest` | `preserved` | `-` | `preserved` | `present` | `fresh` | - |
| System-open return | `qualified` | `exact_target_preserved` | `preserved` | `safe` | `honest` | `preserved` | `-` | `preserved` | `present` | `fresh` | - |

Findings: none.

### `surface:offboarding.surface` (offboarding_surface, beta)

- Descriptor revision: `surface-rev:offboarding.surface:2026.06.01-01`
- Semantic salience: `severity_bearing`
- Reopen anchor: `reopen:anchor:offboarding:surface`
- Claimed profiles: `macos`, `windows`, `linux`
- Marketed on desktop rows: `yes`
- High-salience: `yes`

| Scenario row | Status | Reopen | Layout | Interruption | Placeholder | Authority | Background | Handoff | Boundary | Freshness | Narrowing reason |
| ------------ | ------ | ------ | ------ | ------------ | ----------- | --------- | ---------- | ------- | -------- | --------- | ---------------- |
| Multi-window | `qualified` | `exact_target_preserved` | `preserved` | `safe` | `honest` | `-` | `-` | `-` | `present` | `fresh` | - |
| Multi-monitor | `qualified` | `exact_target_preserved` | `preserved` | `safe` | `honest` | `-` | `-` | `-` | `present` | `fresh` | - |
| Mixed-DPI | `qualified` | `exact_target_preserved` | `preserved` | `safe` | `honest` | `-` | `-` | `-` | `present` | `fresh` | - |
| Suspend/resume | `qualified` | `exact_target_preserved` | `preserved` | `safe` | `honest` | `-` | `-` | `-` | `present` | `fresh` | - |
| Battery saver | `qualified` | `exact_target_preserved` | `preserved` | `safe` | `honest` | `-` | `throttled_before_corruption` | `-` | `present` | `fresh` | - |
| Thermal pressure | `qualified` | `exact_target_preserved` | `preserved` | `safe` | `honest` | `-` | `throttled_before_corruption` | `-` | `present` | `fresh` | - |
| Deep link | `qualified` | `exact_target_preserved` | `preserved` | `safe` | `honest` | `preserved` | `-` | `preserved` | `present` | `fresh` | - |
| File association | `qualified` | `exact_target_preserved` | `preserved` | `safe` | `honest` | `preserved` | `-` | `preserved` | `present` | `fresh` | - |
| System-open return | `qualified` | `exact_target_preserved` | `preserved` | `safe` | `honest` | `preserved` | `-` | `preserved` | `present` | `fresh` | - |

Findings: none.

### `surface:preview.route_badge` (preview_route_badge, beta)

- Descriptor revision: `surface-rev:preview.route_badge:2026.06.01-01`
- Semantic salience: `lifecycle_bearing`
- Reopen anchor: `reopen:anchor:preview:route_badge`
- Claimed profiles: `macos`, `windows`, `linux`
- Marketed on desktop rows: `yes`
- High-salience: `yes`

| Scenario row | Status | Reopen | Layout | Interruption | Placeholder | Authority | Background | Handoff | Boundary | Freshness | Narrowing reason |
| ------------ | ------ | ------ | ------ | ------------ | ----------- | --------- | ---------- | ------- | -------- | --------- | ---------------- |
| Multi-window | `qualified` | `exact_target_preserved` | `preserved` | `safe` | `honest` | `-` | `-` | `-` | `present` | `fresh` | - |
| Multi-monitor | `qualified` | `exact_target_preserved` | `preserved` | `safe` | `honest` | `-` | `-` | `-` | `present` | `fresh` | - |
| Mixed-DPI | `qualified` | `exact_target_preserved` | `preserved` | `safe` | `honest` | `-` | `-` | `-` | `present` | `fresh` | - |
| Suspend/resume | `qualified` | `exact_target_preserved` | `preserved` | `safe` | `honest` | `-` | `-` | `-` | `present` | `fresh` | - |
| Battery saver | `qualified` | `exact_target_preserved` | `preserved` | `safe` | `honest` | `-` | `throttled_before_corruption` | `-` | `present` | `fresh` | - |
| Thermal pressure | `qualified` | `exact_target_preserved` | `preserved` | `safe` | `honest` | `-` | `throttled_before_corruption` | `-` | `present` | `fresh` | - |
| Deep link | `qualified` | `exact_target_preserved` | `preserved` | `safe` | `honest` | `preserved` | `-` | `preserved` | `present` | `fresh` | - |
| File association | `qualified` | `exact_target_preserved` | `preserved` | `safe` | `honest` | `preserved` | `-` | `preserved` | `present` | `fresh` | - |
| System-open return | `qualified` | `exact_target_preserved` | `preserved` | `safe` | `honest` | `preserved` | `-` | `preserved` | `present` | `fresh` | - |

Findings: none.

### `surface:profiler.capture_panel` (profiler_panel, beta)

- Descriptor revision: `surface-rev:profiler.capture_panel:2026.06.01-01`
- Semantic salience: `informational`
- Reopen anchor: `reopen:anchor:profiler:capture_panel`
- Claimed profiles: `macos`, `windows`, `linux`
- Marketed on desktop rows: `yes`
- High-salience: `no`

| Scenario row | Status | Reopen | Layout | Interruption | Placeholder | Authority | Background | Handoff | Boundary | Freshness | Narrowing reason |
| ------------ | ------ | ------ | ------ | ------------ | ----------- | --------- | ---------- | ------- | -------- | --------- | ---------------- |
| Multi-window | `qualified` | `exact_target_preserved` | `preserved` | `safe` | `honest` | `-` | `-` | `-` | `-` | `fresh` | - |
| Multi-monitor | `qualified` | `exact_target_preserved` | `preserved` | `safe` | `honest` | `-` | `-` | `-` | `-` | `fresh` | - |
| Mixed-DPI | `qualified` | `exact_target_preserved` | `preserved` | `safe` | `honest` | `-` | `-` | `-` | `-` | `fresh` | - |
| Suspend/resume | `qualified` | `exact_target_preserved` | `preserved` | `safe` | `honest` | `-` | `-` | `-` | `-` | `fresh` | - |
| Battery saver | `qualified` | `exact_target_preserved` | `preserved` | `safe` | `honest` | `-` | `throttled_before_corruption` | `-` | `-` | `fresh` | - |
| Thermal pressure | `qualified` | `exact_target_preserved` | `preserved` | `safe` | `honest` | `-` | `throttled_before_corruption` | `-` | `-` | `fresh` | - |
| Deep link | `qualified` | `exact_target_preserved` | `preserved` | `safe` | `honest` | `not_applicable` | `-` | `preserved` | `-` | `fresh` | - |
| File association | `qualified` | `exact_target_preserved` | `preserved` | `safe` | `honest` | `not_applicable` | `-` | `preserved` | `-` | `fresh` | - |
| System-open return | `qualified` | `exact_target_preserved` | `preserved` | `safe` | `honest` | `not_applicable` | `-` | `preserved` | `-` | `fresh` | - |

Findings: none.

### `surface:review.pipeline_card` (pipeline_card, beta)

- Descriptor revision: `surface-rev:review.pipeline_card:2026.06.01-01`
- Semantic salience: `severity_bearing`
- Reopen anchor: `reopen:anchor:review:pipeline_card`
- Claimed profiles: `macos`, `windows`, `linux`
- Marketed on desktop rows: `yes`
- High-salience: `yes`

| Scenario row | Status | Reopen | Layout | Interruption | Placeholder | Authority | Background | Handoff | Boundary | Freshness | Narrowing reason |
| ------------ | ------ | ------ | ------ | ------------ | ----------- | --------- | ---------- | ------- | -------- | --------- | ---------------- |
| Multi-window | `qualified` | `exact_target_preserved` | `preserved` | `safe` | `honest` | `-` | `-` | `-` | `present` | `fresh` | - |
| Multi-monitor | `qualified` | `exact_target_preserved` | `preserved` | `safe` | `honest` | `-` | `-` | `-` | `present` | `fresh` | - |
| Mixed-DPI | `qualified` | `exact_target_preserved` | `preserved` | `safe` | `honest` | `-` | `-` | `-` | `present` | `fresh` | - |
| Suspend/resume | `qualified` | `exact_target_preserved` | `preserved` | `safe` | `honest` | `-` | `-` | `-` | `present` | `fresh` | - |
| Battery saver | `qualified` | `exact_target_preserved` | `preserved` | `safe` | `honest` | `-` | `throttled_before_corruption` | `-` | `present` | `fresh` | - |
| Thermal pressure | `qualified` | `exact_target_preserved` | `preserved` | `safe` | `honest` | `-` | `throttled_before_corruption` | `-` | `present` | `fresh` | - |
| Deep link | `qualified` | `exact_target_preserved` | `preserved` | `safe` | `honest` | `preserved` | `-` | `preserved` | `present` | `fresh` | - |
| File association | `qualified` | `exact_target_preserved` | `preserved` | `safe` | `honest` | `preserved` | `-` | `preserved` | `present` | `fresh` | - |
| System-open return | `qualified` | `exact_target_preserved` | `preserved` | `safe` | `honest` | `preserved` | `-` | `preserved` | `present` | `fresh` | - |

Findings: none.

### `surface:sync.status_surface` (sync_status_surface, beta)

- Descriptor revision: `surface-rev:sync.status_surface:2026.06.01-01`
- Semantic salience: `severity_bearing`
- Reopen anchor: `reopen:anchor:sync:status_surface`
- Claimed profiles: `macos`, `windows`, `linux`
- Marketed on desktop rows: `yes`
- High-salience: `yes`

| Scenario row | Status | Reopen | Layout | Interruption | Placeholder | Authority | Background | Handoff | Boundary | Freshness | Narrowing reason |
| ------------ | ------ | ------ | ------ | ------------ | ----------- | --------- | ---------- | ------- | -------- | --------- | ---------------- |
| Multi-window | `qualified` | `exact_target_preserved` | `preserved` | `safe` | `honest` | `-` | `-` | `-` | `present` | `fresh` | - |
| Multi-monitor | `qualified` | `exact_target_preserved` | `preserved` | `safe` | `honest` | `-` | `-` | `-` | `present` | `fresh` | - |
| Mixed-DPI | `qualified` | `exact_target_preserved` | `preserved` | `safe` | `honest` | `-` | `-` | `-` | `present` | `fresh` | - |
| Suspend/resume | `qualified` | `exact_target_preserved` | `preserved` | `safe` | `honest` | `-` | `-` | `-` | `present` | `fresh` | - |
| Battery saver | `qualified` | `exact_target_preserved` | `preserved` | `safe` | `honest` | `-` | `throttled_before_corruption` | `-` | `present` | `fresh` | - |
| Thermal pressure | `qualified` | `exact_target_preserved` | `preserved` | `safe` | `honest` | `-` | `throttled_before_corruption` | `-` | `present` | `fresh` | - |
| Deep link | `qualified` | `exact_target_preserved` | `preserved` | `safe` | `honest` | `preserved` | `-` | `preserved` | `present` | `fresh` | - |
| File association | `qualified` | `exact_target_preserved` | `preserved` | `safe` | `honest` | `preserved` | `-` | `preserved` | `present` | `fresh` | - |
| System-open return | `qualified` | `exact_target_preserved` | `preserved` | `safe` | `honest` | `preserved` | `-` | `preserved` | `present` | `fresh` | - |

Findings: none.

### `surface:trace.replay_panel` (trace_panel, beta)

- Descriptor revision: `surface-rev:trace.replay_panel:2026.06.01-01`
- Semantic salience: `informational`
- Reopen anchor: `reopen:anchor:trace:replay_panel`
- Claimed profiles: `macos`, `windows`, `linux`
- Marketed on desktop rows: `yes`
- High-salience: `no`

| Scenario row | Status | Reopen | Layout | Interruption | Placeholder | Authority | Background | Handoff | Boundary | Freshness | Narrowing reason |
| ------------ | ------ | ------ | ------ | ------------ | ----------- | --------- | ---------- | ------- | -------- | --------- | ---------------- |
| Multi-window | `qualified` | `exact_target_preserved` | `preserved` | `safe` | `honest` | `-` | `-` | `-` | `-` | `fresh` | - |
| Multi-monitor | `qualified` | `exact_target_preserved` | `preserved` | `safe` | `honest` | `-` | `-` | `-` | `-` | `fresh` | - |
| Mixed-DPI | `qualified` | `exact_target_preserved` | `preserved` | `safe` | `honest` | `-` | `-` | `-` | `-` | `fresh` | - |
| Suspend/resume | `qualified` | `exact_target_preserved` | `preserved` | `safe` | `honest` | `-` | `-` | `-` | `-` | `fresh` | - |
| Battery saver | `qualified` | `exact_target_preserved` | `preserved` | `safe` | `honest` | `-` | `throttled_before_corruption` | `-` | `-` | `fresh` | - |
| Thermal pressure | `qualified` | `exact_target_preserved` | `preserved` | `safe` | `honest` | `-` | `throttled_before_corruption` | `-` | `-` | `fresh` | - |
| Deep link | `qualified` | `exact_target_preserved` | `preserved` | `safe` | `honest` | `not_applicable` | `-` | `preserved` | `-` | `fresh` | - |
| File association | `qualified` | `exact_target_preserved` | `preserved` | `safe` | `honest` | `not_applicable` | `-` | `preserved` | `-` | `fresh` | - |
| System-open return | `qualified` | `exact_target_preserved` | `preserved` | `safe` | `honest` | `not_applicable` | `-` | `preserved` | `-` | `fresh` | - |

Findings: none.

## Verification

```sh
cargo run -q -p aureline-shell --bin aureline_shell_m5_desktop_conformance -- validate
cargo test -p aureline-shell --test m5_desktop_conformance_fixtures
python3 tools/ci/m5/desktop_conformance_check.py
```
