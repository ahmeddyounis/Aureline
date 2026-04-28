# Editor gutter fixtures

Worked examples for
[`docs/ux/editor_gutter_contract.md`](../../../docs/ux/editor_gutter_contract.md)
and the boundary schema at
[`schemas/ux/editor_gutter_lane.schema.json`](../../../schemas/ux/editor_gutter_lane.schema.json).

Each JSON file is either a `gutter_lane_catalog_record` or a
`gutter_line_case_record`. The corpus records input signals, expected
lane placement, collapsed or rejected cues, accessible-name
requirements, high-contrast non-color channels, keyboard command refs,
and no-jitter source-column measurements.

## Cases

| Fixture | Scenario axis |
| --- | --- |
| [`gutter_lane_catalog.json`](./gutter_lane_catalog.json) | Canonical lane order, signal admission, precedence rules, hit-target floor, and no-jitter policy. |
| [`dense_conflict_line.json`](./dense_conflict_line.json) | One line carries breakpoints, diagnostics, VCS/review, fold, coverage, blame, and collaboration cues without losing priority or detail. |
| [`collapsed_fold_hidden_diagnostics.json`](./collapsed_fold_hidden_diagnostics.json) | A collapsed fold remains actionable and names hidden diagnostics before supplemental cues. |
| [`debug_stop_breakpoint_diagnostic.json`](./debug_stop_breakpoint_diagnostic.json) | Current debug frame, breakpoint, diagnostic, fold, and failed-test state resolve without breakpoint/diagnostic overlap. |
| [`narrow_width_fallback.json`](./narrow_width_fallback.json) | Compact gutter preserves breakpoint, folded, and diagnostic perceivability while collapsing supplemental cues and rejecting ambient noise. |
