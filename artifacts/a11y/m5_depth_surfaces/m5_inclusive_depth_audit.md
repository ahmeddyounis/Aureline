# M5 accessibility and locale qualification audit

Generated from the seeded audit in
[`crate::m5_inclusive_depth`](../../../../crates/aureline-shell/src/m5_inclusive_depth/mod.rs).
Regenerate with:

```sh
cargo run -q -p aureline-shell --bin aureline_shell_m5_inclusive_depth -- report-md > \
  artifacts/a11y/m5_depth_surfaces/m5_inclusive_depth_audit.md
```

- Report id: `shell:m5_inclusive_depth:audit:v1`
- Source schema ref: `schemas/a11y/m5-depth-qualification.schema.json`
- Claimed locales: `en`, `ar`, `ja`, `de`
- Registered M5 surfaces: `11`
- High-salience surfaces: `7`
- Marketed surfaces: `11`
- Inclusive rows checked: `99`
- Blocking findings: `0`
- Narrowable marketed rows: `0`
- Status: **clean**
- Generated at: `2026-06-11T00:00:00Z`

## Per-row coverage

| Scenario row | Qualified | Narrowed | Unqualified | Missing evidence |
| ------------ | --------: | -------: | ----------: | ---------------: |
| Keyboard reachability | 11 | 0 | 0 | 0 |
| Screen-reader narration | 11 | 0 | 0 | 0 |
| High zoom | 11 | 0 | 0 | 0 |
| IME composition | 10 | 1 | 0 | 0 |
| Grapheme correctness | 11 | 0 | 0 | 0 |
| Bidi direction | 11 | 0 | 0 | 0 |
| Pseudolocalization | 11 | 0 | 0 | 0 |
| Locale fallback | 11 | 0 | 0 | 0 |
| Translated-help parity | 10 | 1 | 0 | 0 |

## Findings summary

| Class | Count |
| ----- | ----: |
| `unqualified_local_a11y_path` | 0 |
| `missing_evidence` | 0 |
| `missing_evidence_pack` | 0 |
| `keyboard_unreachable` | 0 |
| `narration_silent` | 0 |
| `narration_misannounced` | 0 |
| `focus_indicator_hidden` | 0 |
| `text_corrupted` | 0 |
| `ime_composition_broken` | 0 |
| `bidi_leaking` | 0 |
| `zoom_content_clipped` | 0 |
| `locale_parity_lost` | 0 |
| `suspicious_content_hidden` | 0 |
| `stale_evidence_on_marketed_row` | 0 |
| `dimension_drift` | 0 |
| `missing_narrowing_reason` | 0 |
| `missing_projection` | 0 |
| `descriptor_missing_locale_anchor` | 0 |
| `missing_inclusive_note` | 0 |
| `missing_claimed_locales` | 0 |
| `surface_not_on_inclusive_harness` | 0 |

## Locale anchor index

| Surface family | Surface | Locale anchor |
| -------------- | ------- | ------------- |
| Companion surface | `surface:companion.surface` | `a11y:anchor:companion:surface` |
| Query console | `surface:data_api.query_console` | `a11y:anchor:data_api:query_console` |
| Result-grid row | `surface:data_api.result_grid_row` | `a11y:anchor:data_api:result_grid_row` |
| Docs / help pane | `surface:docs_help.pane` | `a11y:anchor:docs_help:pane` |
| Glossary panel | `surface:learning.glossary_panel` | `a11y:anchor:learning:glossary_panel` |
| Guided tour | `surface:learning.guided_tour` | `a11y:anchor:learning:guided_tour` |
| Notebook cell | `surface:notebook.cell` | `a11y:anchor:notebook:cell` |
| Preview-route pane | `surface:preview.route_pane` | `a11y:anchor:preview:route_pane` |
| Profiler timeline | `surface:profiler.timeline` | `a11y:anchor:profiler:timeline` |
| Pipeline / log view | `surface:review.pipeline_log_view` | `a11y:anchor:review:pipeline_log_view` |
| Support packet | `surface:support.packet` | `a11y:anchor:support:packet` |

## Per-surface rows

### `surface:companion.surface` (companion_surface, beta)

- Descriptor revision: `surface-rev:companion.surface:2026.06.01-01`
- Semantic salience: `trust_bearing`
- Locale anchor: `a11y:anchor:companion:surface`
- Claimed locales: `en`, `ar`, `ja`, `de`
- Marketed on inclusive rows: `yes`
- High-salience: `yes`

| Scenario row | Status | Keyboard | Narration | Focus | Text | IME | Bidi | Zoom | Locale | Cue | Freshness | Narrowing reason |
| ------------ | ------ | -------- | --------- | ----- | ---- | --- | ---- | ---- | ------ | --- | --------- | ---------------- |
| Keyboard reachability | `qualified` | `reachable` | `narrated` | `visible` | `correct` | `-` | `-` | `-` | `-` | `present` | `fresh` | - |
| Screen-reader narration | `qualified` | `reachable` | `narrated` | `visible` | `correct` | `-` | `-` | `-` | `-` | `present` | `fresh` | - |
| High zoom | `qualified` | `reachable` | `narrated` | `visible` | `correct` | `-` | `-` | `reflowed` | `-` | `present` | `fresh` | - |
| IME composition | `declared_capture_gap` | `-` | `-` | `-` | `-` | `-` | `-` | `-` | `-` | `-` | `-` | companion_provider_owns_its_on_device_ime_so_the_composition_capture_is_provider_attributed |
| Grapheme correctness | `qualified` | `reachable` | `narrated` | `visible` | `correct` | `-` | `-` | `-` | `-` | `present` | `fresh` | - |
| Bidi direction | `qualified` | `reachable` | `narrated` | `visible` | `correct` | `-` | `isolated` | `-` | `-` | `present` | `fresh` | - |
| Pseudolocalization | `qualified` | `reachable` | `narrated` | `visible` | `correct` | `-` | `-` | `-` | `parity` | `present` | `fresh` | - |
| Locale fallback | `qualified` | `reachable` | `narrated` | `visible` | `correct` | `-` | `-` | `-` | `parity` | `present` | `fresh` | - |
| Translated-help parity | `qualified` | `reachable` | `narrated` | `visible` | `correct` | `-` | `-` | `-` | `parity` | `present` | `fresh` | - |

Findings: none.

### `surface:data_api.query_console` (query_console, beta)

- Descriptor revision: `surface-rev:data_api.query_console:2026.06.01-01`
- Semantic salience: `lifecycle_bearing`
- Locale anchor: `a11y:anchor:data_api:query_console`
- Claimed locales: `en`, `ar`, `ja`, `de`
- Marketed on inclusive rows: `yes`
- High-salience: `yes`

| Scenario row | Status | Keyboard | Narration | Focus | Text | IME | Bidi | Zoom | Locale | Cue | Freshness | Narrowing reason |
| ------------ | ------ | -------- | --------- | ----- | ---- | --- | ---- | ---- | ------ | --- | --------- | ---------------- |
| Keyboard reachability | `qualified` | `reachable` | `narrated` | `visible` | `correct` | `-` | `-` | `-` | `-` | `present` | `fresh` | - |
| Screen-reader narration | `qualified` | `reachable` | `narrated` | `visible` | `correct` | `-` | `-` | `-` | `-` | `present` | `fresh` | - |
| High zoom | `qualified` | `reachable` | `narrated` | `visible` | `correct` | `-` | `-` | `reflowed` | `-` | `present` | `fresh` | - |
| IME composition | `qualified` | `reachable` | `narrated` | `visible` | `correct` | `preserved` | `-` | `-` | `-` | `present` | `fresh` | - |
| Grapheme correctness | `qualified` | `reachable` | `narrated` | `visible` | `correct` | `-` | `-` | `-` | `-` | `present` | `fresh` | - |
| Bidi direction | `qualified` | `reachable` | `narrated` | `visible` | `correct` | `-` | `isolated` | `-` | `-` | `present` | `fresh` | - |
| Pseudolocalization | `qualified` | `reachable` | `narrated` | `visible` | `correct` | `-` | `-` | `-` | `parity` | `present` | `fresh` | - |
| Locale fallback | `qualified` | `reachable` | `narrated` | `visible` | `correct` | `-` | `-` | `-` | `parity` | `present` | `fresh` | - |
| Translated-help parity | `qualified` | `reachable` | `narrated` | `visible` | `correct` | `-` | `-` | `-` | `parity` | `present` | `fresh` | - |

Findings: none.

### `surface:data_api.result_grid_row` (result_grid_row, beta)

- Descriptor revision: `surface-rev:data_api.result_grid_row:2026.06.01-01`
- Semantic salience: `lifecycle_bearing`
- Locale anchor: `a11y:anchor:data_api:result_grid_row`
- Claimed locales: `en`, `ar`, `ja`, `de`
- Marketed on inclusive rows: `yes`
- High-salience: `yes`

| Scenario row | Status | Keyboard | Narration | Focus | Text | IME | Bidi | Zoom | Locale | Cue | Freshness | Narrowing reason |
| ------------ | ------ | -------- | --------- | ----- | ---- | --- | ---- | ---- | ------ | --- | --------- | ---------------- |
| Keyboard reachability | `qualified` | `reachable` | `narrated` | `visible` | `correct` | `-` | `-` | `-` | `-` | `present` | `fresh` | - |
| Screen-reader narration | `qualified` | `reachable` | `narrated` | `visible` | `correct` | `-` | `-` | `-` | `-` | `present` | `fresh` | - |
| High zoom | `qualified` | `reachable` | `narrated` | `visible` | `correct` | `-` | `-` | `reflowed` | `-` | `present` | `fresh` | - |
| IME composition | `qualified` | `reachable` | `narrated` | `visible` | `correct` | `preserved` | `-` | `-` | `-` | `present` | `fresh` | - |
| Grapheme correctness | `qualified` | `reachable` | `narrated` | `visible` | `correct` | `-` | `-` | `-` | `-` | `present` | `fresh` | - |
| Bidi direction | `qualified` | `reachable` | `narrated` | `visible` | `correct` | `-` | `isolated` | `-` | `-` | `present` | `fresh` | - |
| Pseudolocalization | `qualified` | `reachable` | `narrated` | `visible` | `correct` | `-` | `-` | `-` | `parity` | `present` | `fresh` | - |
| Locale fallback | `qualified` | `reachable` | `narrated` | `visible` | `correct` | `-` | `-` | `-` | `parity` | `present` | `fresh` | - |
| Translated-help parity | `qualified` | `reachable` | `narrated` | `visible` | `correct` | `-` | `-` | `-` | `parity` | `present` | `fresh` | - |

Findings: none.

### `surface:docs_help.pane` (docs_help_pane, beta)

- Descriptor revision: `surface-rev:docs_help.pane:2026.06.01-01`
- Semantic salience: `informational`
- Locale anchor: `a11y:anchor:docs_help:pane`
- Claimed locales: `en`, `ar`, `ja`, `de`
- Marketed on inclusive rows: `yes`
- High-salience: `no`

| Scenario row | Status | Keyboard | Narration | Focus | Text | IME | Bidi | Zoom | Locale | Cue | Freshness | Narrowing reason |
| ------------ | ------ | -------- | --------- | ----- | ---- | --- | ---- | ---- | ------ | --- | --------- | ---------------- |
| Keyboard reachability | `qualified` | `reachable` | `narrated` | `visible` | `correct` | `-` | `-` | `-` | `-` | `-` | `fresh` | - |
| Screen-reader narration | `qualified` | `reachable` | `narrated` | `visible` | `correct` | `-` | `-` | `-` | `-` | `-` | `fresh` | - |
| High zoom | `qualified` | `reachable` | `narrated` | `visible` | `correct` | `-` | `-` | `reflowed` | `-` | `-` | `fresh` | - |
| IME composition | `qualified` | `reachable` | `narrated` | `visible` | `correct` | `preserved` | `-` | `-` | `-` | `-` | `fresh` | - |
| Grapheme correctness | `qualified` | `reachable` | `narrated` | `visible` | `correct` | `-` | `-` | `-` | `-` | `-` | `fresh` | - |
| Bidi direction | `qualified` | `reachable` | `narrated` | `visible` | `correct` | `-` | `isolated` | `-` | `-` | `-` | `fresh` | - |
| Pseudolocalization | `qualified` | `reachable` | `narrated` | `visible` | `correct` | `-` | `-` | `-` | `parity` | `-` | `fresh` | - |
| Locale fallback | `qualified` | `reachable` | `narrated` | `visible` | `correct` | `-` | `-` | `-` | `parity` | `-` | `fresh` | - |
| Translated-help parity | `qualified` | `reachable` | `narrated` | `visible` | `correct` | `-` | `-` | `-` | `parity` | `-` | `fresh` | - |

Findings: none.

### `surface:learning.glossary_panel` (glossary_panel, beta)

- Descriptor revision: `surface-rev:learning.glossary_panel:2026.06.01-01`
- Semantic salience: `informational`
- Locale anchor: `a11y:anchor:learning:glossary_panel`
- Claimed locales: `en`, `ar`, `ja`, `de`
- Marketed on inclusive rows: `yes`
- High-salience: `no`

| Scenario row | Status | Keyboard | Narration | Focus | Text | IME | Bidi | Zoom | Locale | Cue | Freshness | Narrowing reason |
| ------------ | ------ | -------- | --------- | ----- | ---- | --- | ---- | ---- | ------ | --- | --------- | ---------------- |
| Keyboard reachability | `qualified` | `reachable` | `narrated` | `visible` | `correct` | `-` | `-` | `-` | `-` | `-` | `fresh` | - |
| Screen-reader narration | `qualified` | `reachable` | `narrated` | `visible` | `correct` | `-` | `-` | `-` | `-` | `-` | `fresh` | - |
| High zoom | `qualified` | `reachable` | `narrated` | `visible` | `correct` | `-` | `-` | `reflowed` | `-` | `-` | `fresh` | - |
| IME composition | `qualified` | `reachable` | `narrated` | `visible` | `correct` | `preserved` | `-` | `-` | `-` | `-` | `fresh` | - |
| Grapheme correctness | `qualified` | `reachable` | `narrated` | `visible` | `correct` | `-` | `-` | `-` | `-` | `-` | `fresh` | - |
| Bidi direction | `qualified` | `reachable` | `narrated` | `visible` | `correct` | `-` | `isolated` | `-` | `-` | `-` | `fresh` | - |
| Pseudolocalization | `qualified` | `reachable` | `narrated` | `visible` | `correct` | `-` | `-` | `-` | `parity` | `-` | `fresh` | - |
| Locale fallback | `qualified` | `reachable` | `narrated` | `visible` | `correct` | `-` | `-` | `-` | `parity` | `-` | `fresh` | - |
| Translated-help parity | `qualified` | `reachable` | `narrated` | `visible` | `correct` | `-` | `-` | `-` | `parity` | `-` | `fresh` | - |

Findings: none.

### `surface:learning.guided_tour` (guided_tour, beta)

- Descriptor revision: `surface-rev:learning.guided_tour:2026.06.01-01`
- Semantic salience: `informational`
- Locale anchor: `a11y:anchor:learning:guided_tour`
- Claimed locales: `en`, `ar`, `ja`, `de`
- Marketed on inclusive rows: `yes`
- High-salience: `no`

| Scenario row | Status | Keyboard | Narration | Focus | Text | IME | Bidi | Zoom | Locale | Cue | Freshness | Narrowing reason |
| ------------ | ------ | -------- | --------- | ----- | ---- | --- | ---- | ---- | ------ | --- | --------- | ---------------- |
| Keyboard reachability | `qualified` | `reachable` | `narrated` | `visible` | `correct` | `-` | `-` | `-` | `-` | `-` | `fresh` | - |
| Screen-reader narration | `qualified` | `reachable` | `narrated` | `visible` | `correct` | `-` | `-` | `-` | `-` | `-` | `fresh` | - |
| High zoom | `qualified` | `reachable` | `narrated` | `visible` | `correct` | `-` | `-` | `reflowed` | `-` | `-` | `fresh` | - |
| IME composition | `qualified` | `reachable` | `narrated` | `visible` | `correct` | `preserved` | `-` | `-` | `-` | `-` | `fresh` | - |
| Grapheme correctness | `qualified` | `reachable` | `narrated` | `visible` | `correct` | `-` | `-` | `-` | `-` | `-` | `fresh` | - |
| Bidi direction | `qualified` | `reachable` | `narrated` | `visible` | `correct` | `-` | `isolated` | `-` | `-` | `-` | `fresh` | - |
| Pseudolocalization | `qualified` | `reachable` | `narrated` | `visible` | `correct` | `-` | `-` | `-` | `parity` | `-` | `fresh` | - |
| Locale fallback | `qualified` | `reachable` | `narrated` | `visible` | `correct` | `-` | `-` | `-` | `parity` | `-` | `fresh` | - |
| Translated-help parity | `qualified` | `reachable` | `narrated` | `visible` | `correct` | `-` | `-` | `-` | `parity` | `-` | `fresh` | - |

Findings: none.

### `surface:notebook.cell` (notebook_cell, beta)

- Descriptor revision: `surface-rev:notebook.cell:2026.06.01-01`
- Semantic salience: `lifecycle_bearing`
- Locale anchor: `a11y:anchor:notebook:cell`
- Claimed locales: `en`, `ar`, `ja`, `de`
- Marketed on inclusive rows: `yes`
- High-salience: `yes`

| Scenario row | Status | Keyboard | Narration | Focus | Text | IME | Bidi | Zoom | Locale | Cue | Freshness | Narrowing reason |
| ------------ | ------ | -------- | --------- | ----- | ---- | --- | ---- | ---- | ------ | --- | --------- | ---------------- |
| Keyboard reachability | `qualified` | `reachable` | `narrated` | `visible` | `correct` | `-` | `-` | `-` | `-` | `present` | `fresh` | - |
| Screen-reader narration | `qualified` | `reachable` | `narrated` | `visible` | `correct` | `-` | `-` | `-` | `-` | `present` | `fresh` | - |
| High zoom | `qualified` | `reachable` | `narrated` | `visible` | `correct` | `-` | `-` | `reflowed` | `-` | `present` | `fresh` | - |
| IME composition | `qualified` | `reachable` | `narrated` | `visible` | `correct` | `preserved` | `-` | `-` | `-` | `present` | `fresh` | - |
| Grapheme correctness | `qualified` | `reachable` | `narrated` | `visible` | `correct` | `-` | `-` | `-` | `-` | `present` | `fresh` | - |
| Bidi direction | `qualified` | `reachable` | `narrated` | `visible` | `correct` | `-` | `isolated` | `-` | `-` | `present` | `fresh` | - |
| Pseudolocalization | `qualified` | `reachable` | `narrated` | `visible` | `correct` | `-` | `-` | `-` | `parity` | `present` | `fresh` | - |
| Locale fallback | `qualified` | `reachable` | `narrated` | `visible` | `correct` | `-` | `-` | `-` | `parity` | `present` | `fresh` | - |
| Translated-help parity | `qualified` | `reachable` | `narrated` | `visible` | `correct` | `-` | `-` | `-` | `parity` | `present` | `fresh` | - |

Findings: none.

### `surface:preview.route_pane` (preview_route_pane, beta)

- Descriptor revision: `surface-rev:preview.route_pane:2026.06.01-01`
- Semantic salience: `lifecycle_bearing`
- Locale anchor: `a11y:anchor:preview:route_pane`
- Claimed locales: `en`, `ar`, `ja`, `de`
- Marketed on inclusive rows: `yes`
- High-salience: `yes`

| Scenario row | Status | Keyboard | Narration | Focus | Text | IME | Bidi | Zoom | Locale | Cue | Freshness | Narrowing reason |
| ------------ | ------ | -------- | --------- | ----- | ---- | --- | ---- | ---- | ------ | --- | --------- | ---------------- |
| Keyboard reachability | `qualified` | `reachable` | `narrated` | `visible` | `correct` | `-` | `-` | `-` | `-` | `present` | `fresh` | - |
| Screen-reader narration | `qualified` | `reachable` | `narrated` | `visible` | `correct` | `-` | `-` | `-` | `-` | `present` | `fresh` | - |
| High zoom | `qualified` | `reachable` | `narrated` | `visible` | `correct` | `-` | `-` | `reflowed` | `-` | `present` | `fresh` | - |
| IME composition | `qualified` | `reachable` | `narrated` | `visible` | `correct` | `preserved` | `-` | `-` | `-` | `present` | `fresh` | - |
| Grapheme correctness | `qualified` | `reachable` | `narrated` | `visible` | `correct` | `-` | `-` | `-` | `-` | `present` | `fresh` | - |
| Bidi direction | `qualified` | `reachable` | `narrated` | `visible` | `correct` | `-` | `isolated` | `-` | `-` | `present` | `fresh` | - |
| Pseudolocalization | `qualified` | `reachable` | `narrated` | `visible` | `correct` | `-` | `-` | `-` | `parity` | `present` | `fresh` | - |
| Locale fallback | `qualified` | `reachable` | `narrated` | `visible` | `correct` | `-` | `-` | `-` | `parity` | `present` | `fresh` | - |
| Translated-help parity | `declared_capture_gap` | `-` | `-` | `-` | `-` | `-` | `-` | `-` | `-` | `-` | `-` | embedded_provider_owns_its_localized_help_so_the_translated_help_parity_capture_is_provider_attributed |

Findings: none.

### `surface:profiler.timeline` (profiler_timeline, beta)

- Descriptor revision: `surface-rev:profiler.timeline:2026.06.01-01`
- Semantic salience: `informational`
- Locale anchor: `a11y:anchor:profiler:timeline`
- Claimed locales: `en`, `ar`, `ja`, `de`
- Marketed on inclusive rows: `yes`
- High-salience: `no`

| Scenario row | Status | Keyboard | Narration | Focus | Text | IME | Bidi | Zoom | Locale | Cue | Freshness | Narrowing reason |
| ------------ | ------ | -------- | --------- | ----- | ---- | --- | ---- | ---- | ------ | --- | --------- | ---------------- |
| Keyboard reachability | `qualified` | `reachable` | `narrated` | `visible` | `correct` | `-` | `-` | `-` | `-` | `-` | `fresh` | - |
| Screen-reader narration | `qualified` | `reachable` | `narrated` | `visible` | `correct` | `-` | `-` | `-` | `-` | `-` | `fresh` | - |
| High zoom | `qualified` | `reachable` | `narrated` | `visible` | `correct` | `-` | `-` | `reflowed` | `-` | `-` | `fresh` | - |
| IME composition | `qualified` | `reachable` | `narrated` | `visible` | `correct` | `preserved` | `-` | `-` | `-` | `-` | `fresh` | - |
| Grapheme correctness | `qualified` | `reachable` | `narrated` | `visible` | `correct` | `-` | `-` | `-` | `-` | `-` | `fresh` | - |
| Bidi direction | `qualified` | `reachable` | `narrated` | `visible` | `correct` | `-` | `isolated` | `-` | `-` | `-` | `fresh` | - |
| Pseudolocalization | `qualified` | `reachable` | `narrated` | `visible` | `correct` | `-` | `-` | `-` | `parity` | `-` | `fresh` | - |
| Locale fallback | `qualified` | `reachable` | `narrated` | `visible` | `correct` | `-` | `-` | `-` | `parity` | `-` | `fresh` | - |
| Translated-help parity | `qualified` | `reachable` | `narrated` | `visible` | `correct` | `-` | `-` | `-` | `parity` | `-` | `fresh` | - |

Findings: none.

### `surface:review.pipeline_log_view` (pipeline_log_view, beta)

- Descriptor revision: `surface-rev:review.pipeline_log_view:2026.06.01-01`
- Semantic salience: `severity_bearing`
- Locale anchor: `a11y:anchor:review:pipeline_log_view`
- Claimed locales: `en`, `ar`, `ja`, `de`
- Marketed on inclusive rows: `yes`
- High-salience: `yes`

| Scenario row | Status | Keyboard | Narration | Focus | Text | IME | Bidi | Zoom | Locale | Cue | Freshness | Narrowing reason |
| ------------ | ------ | -------- | --------- | ----- | ---- | --- | ---- | ---- | ------ | --- | --------- | ---------------- |
| Keyboard reachability | `qualified` | `reachable` | `narrated` | `visible` | `correct` | `-` | `-` | `-` | `-` | `present` | `fresh` | - |
| Screen-reader narration | `qualified` | `reachable` | `narrated` | `visible` | `correct` | `-` | `-` | `-` | `-` | `present` | `fresh` | - |
| High zoom | `qualified` | `reachable` | `narrated` | `visible` | `correct` | `-` | `-` | `reflowed` | `-` | `present` | `fresh` | - |
| IME composition | `qualified` | `reachable` | `narrated` | `visible` | `correct` | `preserved` | `-` | `-` | `-` | `present` | `fresh` | - |
| Grapheme correctness | `qualified` | `reachable` | `narrated` | `visible` | `correct` | `-` | `-` | `-` | `-` | `present` | `fresh` | - |
| Bidi direction | `qualified` | `reachable` | `narrated` | `visible` | `correct` | `-` | `isolated` | `-` | `-` | `present` | `fresh` | - |
| Pseudolocalization | `qualified` | `reachable` | `narrated` | `visible` | `correct` | `-` | `-` | `-` | `parity` | `present` | `fresh` | - |
| Locale fallback | `qualified` | `reachable` | `narrated` | `visible` | `correct` | `-` | `-` | `-` | `parity` | `present` | `fresh` | - |
| Translated-help parity | `qualified` | `reachable` | `narrated` | `visible` | `correct` | `-` | `-` | `-` | `parity` | `present` | `fresh` | - |

Findings: none.

### `surface:support.packet` (support_packet, beta)

- Descriptor revision: `surface-rev:support.packet:2026.06.01-01`
- Semantic salience: `severity_bearing`
- Locale anchor: `a11y:anchor:support:packet`
- Claimed locales: `en`, `ar`, `ja`, `de`
- Marketed on inclusive rows: `yes`
- High-salience: `yes`

| Scenario row | Status | Keyboard | Narration | Focus | Text | IME | Bidi | Zoom | Locale | Cue | Freshness | Narrowing reason |
| ------------ | ------ | -------- | --------- | ----- | ---- | --- | ---- | ---- | ------ | --- | --------- | ---------------- |
| Keyboard reachability | `qualified` | `reachable` | `narrated` | `visible` | `correct` | `-` | `-` | `-` | `-` | `present` | `fresh` | - |
| Screen-reader narration | `qualified` | `reachable` | `narrated` | `visible` | `correct` | `-` | `-` | `-` | `-` | `present` | `fresh` | - |
| High zoom | `qualified` | `reachable` | `narrated` | `visible` | `correct` | `-` | `-` | `reflowed` | `-` | `present` | `fresh` | - |
| IME composition | `qualified` | `reachable` | `narrated` | `visible` | `correct` | `preserved` | `-` | `-` | `-` | `present` | `fresh` | - |
| Grapheme correctness | `qualified` | `reachable` | `narrated` | `visible` | `correct` | `-` | `-` | `-` | `-` | `present` | `fresh` | - |
| Bidi direction | `qualified` | `reachable` | `narrated` | `visible` | `correct` | `-` | `isolated` | `-` | `-` | `present` | `fresh` | - |
| Pseudolocalization | `qualified` | `reachable` | `narrated` | `visible` | `correct` | `-` | `-` | `-` | `parity` | `present` | `fresh` | - |
| Locale fallback | `qualified` | `reachable` | `narrated` | `visible` | `correct` | `-` | `-` | `-` | `parity` | `present` | `fresh` | - |
| Translated-help parity | `qualified` | `reachable` | `narrated` | `visible` | `correct` | `-` | `-` | `-` | `parity` | `present` | `fresh` | - |

Findings: none.

## Verification

```sh
cargo run -q -p aureline-shell --bin aureline_shell_m5_inclusive_depth -- validate
cargo test -p aureline-shell --test m5_inclusive_depth_fixtures
python3 tools/ci/m5/inclusive_depth_check.py
```
