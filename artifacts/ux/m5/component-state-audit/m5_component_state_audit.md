# M5 component-state and design-token inheritance audit

Generated from the seeded audit in
[`crate::m5_component_registry`](../../../../crates/aureline-shell/src/m5_component_registry/mod.rs).
Regenerate with:

```sh
cargo run -q -p aureline-shell --bin aureline_shell_m5_component_state -- report-md > \
  artifacts/ux/m5/component-state-audit/m5_component_state_audit.md
```

- Report id: `shell:m5_component_state:audit:v1`
- Source schema ref: `schemas/ux/m5-component-state.schema.json`
- Registered M5 surfaces: `10`
- High-salience surfaces: `7`
- State bindings checked: `90`
- Blocking findings: `0`
- Status: **clean**
- Generated at: `2026-06-11T00:00:00Z`

## Per-state coverage

| State | Inherited | Narrowed | Unregistered | Unknown token |
| ----- | --------: | -------: | -----------: | ------------: |
| Loading | 10 | 0 | 0 | 0 |
| Cached | 10 | 0 | 0 | 0 |
| Stale | 10 | 0 | 0 | 0 |
| Partial | 9 | 1 | 0 | 0 |
| Policy-blocked | 10 | 0 | 0 | 0 |
| Degraded | 9 | 1 | 0 | 0 |
| Preview-only | 10 | 0 | 0 | 0 |
| Sync-pending | 6 | 4 | 0 | 0 |
| Boundary-handoff | 8 | 2 | 0 | 0 |

## Findings summary

| Class | Count |
| ----- | ----: |
| `unknown_token_gap` | 0 |
| `unregistered_local_state` | 0 |
| `token_group_drift` | 0 |
| `token_ref_drift` | 0 |
| `style_provenance_drift` | 0 |
| `cue_policy_drift` | 0 |
| `color_only_cue` | 0 |
| `hardcoded_theme_value` | 0 |
| `unresolved_token_fallback` | 0 |
| `missing_registry_anchor` | 0 |
| `override_drift` | 0 |
| `missing_narrowing_reason` | 0 |
| `missing_projection` | 0 |
| `descriptor_missing_registry_anchor` | 0 |
| `missing_accessibility_note` | 0 |
| `missing_non_color_cue_policy` | 0 |
| `surface_not_registered` | 0 |

## Registry anchor index

| Surface family | Surface | Registry anchor |
| -------------- | ------- | --------------- |
| Companion surface | `surface:companion.surface` | `registry:anchor:companion:surface` |
| Result-grid row | `surface:data_api.result_grid_row` | `registry:anchor:data_api:result_grid_row` |
| Docs / browser pane | `surface:docs_browser.pane` | `registry:anchor:docs_browser:pane` |
| Notebook cell chrome | `surface:notebook.cell_chrome` | `registry:anchor:notebook:cell_chrome` |
| Offboarding surface | `surface:offboarding.surface` | `registry:anchor:offboarding:surface` |
| Preview-route badge | `surface:preview.route_badge` | `registry:anchor:preview:route_badge` |
| Profiler panel | `surface:profiler.capture_panel` | `registry:anchor:profiler:capture_panel` |
| Pipeline card | `surface:review.pipeline_card` | `registry:anchor:review:pipeline_card` |
| Sync status surface | `surface:sync.status_surface` | `registry:anchor:sync:status_surface` |
| Trace panel | `surface:trace.replay_panel` | `registry:anchor:trace:replay_panel` |

## Per-surface rows

### `surface:companion.surface` (companion_surface, beta)

- Descriptor revision: `surface-rev:companion.surface:2026.06.01-01`
- Token group: `companion_presence_tokens`
- Style provenance: `provider_backed`
- Semantic salience: `trust_bearing`
- Cue policy: `non_color_cue_required`
- Registry anchor: `registry:anchor:companion:surface`
- High-salience: `yes`

| State | Status | Token ref | Cue | Provenance | Narrowing reason |
| ----- | ------ | --------- | --- | ---------- | ---------------- |
| Loading | `inherited` | `token:companion_presence_tokens:loading` | `shape_or_pattern` | `provider_backed` | - |
| Cached | `inherited` | `token:companion_presence_tokens:cached` | `text_label` | `provider_backed` | - |
| Stale | `inherited` | `token:companion_presence_tokens:stale` | `icon_and_text` | `provider_backed` | - |
| Partial | `inherited` | `token:companion_presence_tokens:partial` | `icon_and_text` | `provider_backed` | - |
| Policy-blocked | `inherited` | `token:companion_presence_tokens:policy_blocked` | `icon_and_text` | `provider_backed` | - |
| Degraded | `declared_inheritance_gap` | `-` | `-` | `-` | companion_provider_reports_degraded_link_without_a_shell_token_hook |
| Preview-only | `inherited` | `token:companion_presence_tokens:preview_only` | `text_label` | `provider_backed` | - |
| Sync-pending | `inherited` | `token:companion_presence_tokens:sync_pending` | `icon_and_text` | `provider_backed` | - |
| Boundary-handoff | `inherited` | `token:companion_presence_tokens:boundary_handoff` | `icon_and_text` | `provider_backed` | - |

Findings: none.

### `surface:data_api.result_grid_row` (result_grid_row, beta)

- Descriptor revision: `surface-rev:data_api.result_grid_row:2026.06.01-01`
- Token group: `data_density_tokens`
- Style provenance: `shell_token_inherited`
- Semantic salience: `lifecycle_bearing`
- Cue policy: `non_color_cue_required`
- Registry anchor: `registry:anchor:data_api:result_grid_row`
- High-salience: `yes`

| State | Status | Token ref | Cue | Provenance | Narrowing reason |
| ----- | ------ | --------- | --- | ---------- | ---------------- |
| Loading | `inherited` | `token:data_density_tokens:loading` | `shape_or_pattern` | `shell_token_inherited` | - |
| Cached | `inherited` | `token:data_density_tokens:cached` | `icon_and_text` | `shell_token_inherited` | - |
| Stale | `inherited` | `token:data_density_tokens:stale` | `icon_and_text` | `shell_token_inherited` | - |
| Partial | `inherited` | `token:data_density_tokens:partial` | `icon_and_text` | `shell_token_inherited` | - |
| Policy-blocked | `inherited` | `token:data_density_tokens:policy_blocked` | `icon_and_text` | `shell_token_inherited` | - |
| Degraded | `inherited` | `token:data_density_tokens:degraded` | `icon_and_text` | `shell_token_inherited` | - |
| Preview-only | `inherited` | `token:data_density_tokens:preview_only` | `text_label` | `shell_token_inherited` | - |
| Sync-pending | `inherited` | `token:data_density_tokens:sync_pending` | `icon_and_text` | `shell_token_inherited` | - |
| Boundary-handoff | `not_applicable` | `-` | `-` | `-` | result_rows_do_not_cross_an_embedded_boundary |

Findings: none.

### `surface:docs_browser.pane` (docs_browser_pane, beta)

- Descriptor revision: `surface-rev:docs_browser.pane:2026.06.01-01`
- Token group: `embedded_surface_tokens`
- Style provenance: `extension_contributed`
- Semantic salience: `informational`
- Cue policy: `color_allowed`
- Registry anchor: `registry:anchor:docs_browser:pane`
- High-salience: `no`

| State | Status | Token ref | Cue | Provenance | Narrowing reason |
| ----- | ------ | --------- | --- | ---------- | ---------------- |
| Loading | `inherited` | `token:embedded_surface_tokens:loading` | `shape_or_pattern` | `extension_contributed` | - |
| Cached | `inherited` | `token:embedded_surface_tokens:cached` | `text_label` | `extension_contributed` | - |
| Stale | `inherited` | `token:embedded_surface_tokens:stale` | `text_label` | `extension_contributed` | - |
| Partial | `declared_inheritance_gap` | `-` | `-` | `-` | embedded_provider_renders_partial_content_without_a_shell_token_hook |
| Policy-blocked | `inherited` | `token:embedded_surface_tokens:policy_blocked` | `icon_and_text` | `extension_contributed` | - |
| Degraded | `inherited` | `token:embedded_surface_tokens:degraded` | `text_label` | `extension_contributed` | - |
| Preview-only | `inherited` | `token:embedded_surface_tokens:preview_only` | `text_label` | `extension_contributed` | - |
| Sync-pending | `not_applicable` | `-` | `-` | `-` | browsed_content_is_not_synced_by_the_workspace |
| Boundary-handoff | `inherited` | `token:embedded_surface_tokens:boundary_handoff` | `icon_and_text` | `extension_contributed` | - |

Findings: none.

### `surface:notebook.cell_chrome` (notebook_cell_chrome, beta)

- Descriptor revision: `surface-rev:notebook.cell_chrome:2026.06.01-01`
- Token group: `surface_chrome_tokens`
- Style provenance: `shell_token_inherited`
- Semantic salience: `lifecycle_bearing`
- Cue policy: `non_color_cue_required`
- Registry anchor: `registry:anchor:notebook:cell_chrome`
- High-salience: `yes`

| State | Status | Token ref | Cue | Provenance | Narrowing reason |
| ----- | ------ | --------- | --- | ---------- | ---------------- |
| Loading | `inherited` | `token:surface_chrome_tokens:loading` | `icon_and_text` | `shell_token_inherited` | - |
| Cached | `inherited` | `token:surface_chrome_tokens:cached` | `text_label` | `shell_token_inherited` | - |
| Stale | `inherited` | `token:surface_chrome_tokens:stale` | `icon_and_text` | `shell_token_inherited` | - |
| Partial | `inherited` | `token:surface_chrome_tokens:partial` | `icon_and_text` | `shell_token_inherited` | - |
| Policy-blocked | `inherited` | `token:surface_chrome_tokens:policy_blocked` | `icon_and_text` | `shell_token_inherited` | - |
| Degraded | `inherited` | `token:surface_chrome_tokens:degraded` | `icon_and_text` | `shell_token_inherited` | - |
| Preview-only | `inherited` | `token:surface_chrome_tokens:preview_only` | `shape_or_pattern` | `shell_token_inherited` | - |
| Sync-pending | `not_applicable` | `-` | `-` | `-` | notebook_cells_sync_through_document_not_cell_chrome |
| Boundary-handoff | `inherited` | `token:surface_chrome_tokens:boundary_handoff` | `icon_and_text` | `shell_token_inherited` | - |

Findings: none.

### `surface:offboarding.surface` (offboarding_surface, beta)

- Descriptor revision: `surface-rev:offboarding.surface:2026.06.01-01`
- Token group: `lifecycle_state_tokens`
- Style provenance: `shell_token_inherited`
- Semantic salience: `severity_bearing`
- Cue policy: `non_color_cue_required`
- Registry anchor: `registry:anchor:offboarding:surface`
- High-salience: `yes`

| State | Status | Token ref | Cue | Provenance | Narrowing reason |
| ----- | ------ | --------- | --- | ---------- | ---------------- |
| Loading | `inherited` | `token:lifecycle_state_tokens:loading` | `shape_or_pattern` | `shell_token_inherited` | - |
| Cached | `inherited` | `token:lifecycle_state_tokens:cached` | `text_label` | `shell_token_inherited` | - |
| Stale | `inherited` | `token:lifecycle_state_tokens:stale` | `icon_and_text` | `shell_token_inherited` | - |
| Partial | `inherited` | `token:lifecycle_state_tokens:partial` | `icon_and_text` | `shell_token_inherited` | - |
| Policy-blocked | `inherited` | `token:lifecycle_state_tokens:policy_blocked` | `icon_and_text` | `shell_token_inherited` | - |
| Degraded | `inherited` | `token:lifecycle_state_tokens:degraded` | `icon_and_text` | `shell_token_inherited` | - |
| Preview-only | `inherited` | `token:lifecycle_state_tokens:preview_only` | `icon_and_text` | `shell_token_inherited` | - |
| Sync-pending | `inherited` | `token:lifecycle_state_tokens:sync_pending` | `icon_and_text` | `shell_token_inherited` | - |
| Boundary-handoff | `inherited` | `token:lifecycle_state_tokens:boundary_handoff` | `icon_and_text` | `shell_token_inherited` | - |

Findings: none.

### `surface:preview.route_badge` (preview_route_badge, beta)

- Descriptor revision: `surface-rev:preview.route_badge:2026.06.01-01`
- Token group: `preview_badge_tokens`
- Style provenance: `shell_token_inherited`
- Semantic salience: `lifecycle_bearing`
- Cue policy: `non_color_cue_required`
- Registry anchor: `registry:anchor:preview:route_badge`
- High-salience: `yes`

| State | Status | Token ref | Cue | Provenance | Narrowing reason |
| ----- | ------ | --------- | --- | ---------- | ---------------- |
| Loading | `inherited` | `token:preview_badge_tokens:loading` | `shape_or_pattern` | `shell_token_inherited` | - |
| Cached | `inherited` | `token:preview_badge_tokens:cached` | `text_label` | `shell_token_inherited` | - |
| Stale | `inherited` | `token:preview_badge_tokens:stale` | `icon_and_text` | `shell_token_inherited` | - |
| Partial | `inherited` | `token:preview_badge_tokens:partial` | `text_label` | `shell_token_inherited` | - |
| Policy-blocked | `inherited` | `token:preview_badge_tokens:policy_blocked` | `icon_and_text` | `shell_token_inherited` | - |
| Degraded | `inherited` | `token:preview_badge_tokens:degraded` | `icon_and_text` | `shell_token_inherited` | - |
| Preview-only | `inherited` | `token:preview_badge_tokens:preview_only` | `icon_and_text` | `shell_token_inherited` | - |
| Sync-pending | `inherited` | `token:preview_badge_tokens:sync_pending` | `icon_and_text` | `shell_token_inherited` | - |
| Boundary-handoff | `inherited` | `token:preview_badge_tokens:boundary_handoff` | `icon_and_text` | `shell_token_inherited` | - |

Findings: none.

### `surface:profiler.capture_panel` (profiler_panel, beta)

- Descriptor revision: `surface-rev:profiler.capture_panel:2026.06.01-01`
- Token group: `diagnostic_state_tokens`
- Style provenance: `shell_token_inherited`
- Semantic salience: `informational`
- Cue policy: `color_allowed`
- Registry anchor: `registry:anchor:profiler:capture_panel`
- High-salience: `no`

| State | Status | Token ref | Cue | Provenance | Narrowing reason |
| ----- | ------ | --------- | --- | ---------- | ---------------- |
| Loading | `inherited` | `token:diagnostic_state_tokens:loading` | `shape_or_pattern` | `shell_token_inherited` | - |
| Cached | `inherited` | `token:diagnostic_state_tokens:cached` | `text_label` | `shell_token_inherited` | - |
| Stale | `inherited` | `token:diagnostic_state_tokens:stale` | `text_label` | `shell_token_inherited` | - |
| Partial | `inherited` | `token:diagnostic_state_tokens:partial` | `text_label` | `shell_token_inherited` | - |
| Policy-blocked | `inherited` | `token:diagnostic_state_tokens:policy_blocked` | `icon_and_text` | `shell_token_inherited` | - |
| Degraded | `inherited` | `token:diagnostic_state_tokens:degraded` | `text_label` | `shell_token_inherited` | - |
| Preview-only | `inherited` | `token:diagnostic_state_tokens:preview_only` | `text_label` | `shell_token_inherited` | - |
| Sync-pending | `not_applicable` | `-` | `-` | `-` | captures_are_local_until_explicitly_exported |
| Boundary-handoff | `not_applicable` | `-` | `-` | `-` | profiler_capture_does_not_hand_off_across_a_boundary |

Findings: none.

### `surface:review.pipeline_card` (pipeline_card, beta)

- Descriptor revision: `surface-rev:review.pipeline_card:2026.06.01-01`
- Token group: `pipeline_status_tokens`
- Style provenance: `shell_token_inherited`
- Semantic salience: `severity_bearing`
- Cue policy: `non_color_cue_required`
- Registry anchor: `registry:anchor:review:pipeline_card`
- High-salience: `yes`

| State | Status | Token ref | Cue | Provenance | Narrowing reason |
| ----- | ------ | --------- | --- | ---------- | ---------------- |
| Loading | `inherited` | `token:pipeline_status_tokens:loading` | `shape_or_pattern` | `shell_token_inherited` | - |
| Cached | `inherited` | `token:pipeline_status_tokens:cached` | `text_label` | `shell_token_inherited` | - |
| Stale | `inherited` | `token:pipeline_status_tokens:stale` | `icon_and_text` | `shell_token_inherited` | - |
| Partial | `inherited` | `token:pipeline_status_tokens:partial` | `icon_and_text` | `shell_token_inherited` | - |
| Policy-blocked | `inherited` | `token:pipeline_status_tokens:policy_blocked` | `icon_and_text` | `shell_token_inherited` | - |
| Degraded | `inherited` | `token:pipeline_status_tokens:degraded` | `icon_and_text` | `shell_token_inherited` | - |
| Preview-only | `inherited` | `token:pipeline_status_tokens:preview_only` | `icon_and_text` | `shell_token_inherited` | - |
| Sync-pending | `inherited` | `token:pipeline_status_tokens:sync_pending` | `icon_and_text` | `shell_token_inherited` | - |
| Boundary-handoff | `inherited` | `token:pipeline_status_tokens:boundary_handoff` | `icon_and_text` | `shell_token_inherited` | - |

Findings: none.

### `surface:sync.status_surface` (sync_status_surface, beta)

- Descriptor revision: `surface-rev:sync.status_surface:2026.06.01-01`
- Token group: `sync_status_tokens`
- Style provenance: `shell_token_inherited`
- Semantic salience: `severity_bearing`
- Cue policy: `non_color_cue_required`
- Registry anchor: `registry:anchor:sync:status_surface`
- High-salience: `yes`

| State | Status | Token ref | Cue | Provenance | Narrowing reason |
| ----- | ------ | --------- | --- | ---------- | ---------------- |
| Loading | `inherited` | `token:sync_status_tokens:loading` | `shape_or_pattern` | `shell_token_inherited` | - |
| Cached | `inherited` | `token:sync_status_tokens:cached` | `text_label` | `shell_token_inherited` | - |
| Stale | `inherited` | `token:sync_status_tokens:stale` | `icon_and_text` | `shell_token_inherited` | - |
| Partial | `inherited` | `token:sync_status_tokens:partial` | `icon_and_text` | `shell_token_inherited` | - |
| Policy-blocked | `inherited` | `token:sync_status_tokens:policy_blocked` | `icon_and_text` | `shell_token_inherited` | - |
| Degraded | `inherited` | `token:sync_status_tokens:degraded` | `icon_and_text` | `shell_token_inherited` | - |
| Preview-only | `inherited` | `token:sync_status_tokens:preview_only` | `text_label` | `shell_token_inherited` | - |
| Sync-pending | `inherited` | `token:sync_status_tokens:sync_pending` | `icon_and_text` | `shell_token_inherited` | - |
| Boundary-handoff | `inherited` | `token:sync_status_tokens:boundary_handoff` | `icon_and_text` | `shell_token_inherited` | - |

Findings: none.

### `surface:trace.replay_panel` (trace_panel, beta)

- Descriptor revision: `surface-rev:trace.replay_panel:2026.06.01-01`
- Token group: `diagnostic_state_tokens`
- Style provenance: `shell_token_inherited`
- Semantic salience: `informational`
- Cue policy: `color_allowed`
- Registry anchor: `registry:anchor:trace:replay_panel`
- High-salience: `no`

| State | Status | Token ref | Cue | Provenance | Narrowing reason |
| ----- | ------ | --------- | --- | ---------- | ---------------- |
| Loading | `inherited` | `token:diagnostic_state_tokens:loading` | `shape_or_pattern` | `shell_token_inherited` | - |
| Cached | `inherited` | `token:diagnostic_state_tokens:cached` | `text_label` | `shell_token_inherited` | - |
| Stale | `inherited` | `token:diagnostic_state_tokens:stale` | `text_label` | `shell_token_inherited` | - |
| Partial | `inherited` | `token:diagnostic_state_tokens:partial` | `text_label` | `shell_token_inherited` | - |
| Policy-blocked | `inherited` | `token:diagnostic_state_tokens:policy_blocked` | `icon_and_text` | `shell_token_inherited` | - |
| Degraded | `inherited` | `token:diagnostic_state_tokens:degraded` | `text_label` | `shell_token_inherited` | - |
| Preview-only | `inherited` | `token:diagnostic_state_tokens:preview_only` | `text_label` | `shell_token_inherited` | - |
| Sync-pending | `not_applicable` | `-` | `-` | `-` | trace_sessions_are_local_until_exported |
| Boundary-handoff | `inherited` | `token:diagnostic_state_tokens:boundary_handoff` | `icon_and_text` | `shell_token_inherited` | - |

Findings: none.

## Verification

```sh
cargo run -q -p aureline-shell --bin aureline_shell_m5_component_state -- validate
cargo test -p aureline-shell --test m5_component_state_fixtures
python3 tools/ci/m5/component_state_check.py
```
