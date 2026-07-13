# M5 Syntax-, Diff-, and Chart-Token Registries

- Packet: `m5-syntax-diff-and-chart-token-registries:stable:0001`
- Label: `M5 syntax-, diff-, and chart-token registries with diagnostics precedence over syntax color, moved-block-confidence and historical-vs-current-emphasis notes, legend / pattern / marker parity, and screenshot / PDF / support-packet / high-contrast export survival across the editor, review, notebook, data, docs, and support surfaces`
- Consumer surfaces: 6
- Syntax roles: keyword, string_literal, comment, identifier, distinct_from_diagnostic, syntax_diagnostic_collision_disallowed
- Diff roles: addition, removal, context, moved, distinct_from_diagnostic, diff_diagnostic_collision_disallowed
- Chart roles: categorical_series, sequential_scale, diverging_scale, paired_with_shape_or_label, accessible_contrast, chart_color_alone_disallowed
- Export channels: screenshot, pdf, support_packet, high_contrast, monochrome_print, channel_unknown
- Proof freshness SLO: 720 hours (last refresh: 2026-07-13T00:00:00Z)

## Consumer surfaces

- **editor_ui**: `stable`
  - Owner: Editor surface owner
  - Scope: The editor consumes the canonical keyword / string / identifier syntax scopes with diagnostics outranking syntax color; a diagnostics collision and a raw-color inlining degrade honestly instead of reading as a clean pass
  - Syntax: 5 / diff: 0 / chart: 0
- **review_ui**: `stable`
  - Owner: Review surface owner
  - Scope: The review surface keeps addition / removal / context diff regions distinct from diagnostics, pairs each with a label or pattern, and states historical-vs-current emphasis; a diagnostics collision and a cue-missing region degrade honestly
  - Syntax: 0 / diff: 5 / chart: 0
- **data_ui**: `stable`
  - Owner: Data surface owner
  - Scope: The data surface keeps categorical / sequential / diverging chart series distinguishable with legends, patterns, and markers at accessible contrast; a color-alone series, a legend-missing series, and a low-contrast series degrade honestly
  - Syntax: 0 / diff: 0 / chart: 6
- **docs_ui**: `stable`
  - Owner: Docs surface owner
  - Scope: The docs surface renders code, diffs, and charts with the same syntax identifier scope, diverging chart scale, and non-color cues so meaning survives when the page is exported
  - Syntax: 1 / diff: 0 / chart: 1
- **shell_ui**: `stable`
  - Owner: Shell / notebook surface owner
  - Scope: The shell and notebook surfaces render inline code comments and moved-block diffs with stated moved-block confidence and high-contrast survival; a precedence-losing scope, an export-losing view, and unstated moved / emphasis notes degrade honestly
  - Syntax: 3 / diff: 5 / chart: 0
- **support_export**: `stable`
  - Owner: Support/export owner
  - Scope: The support export carries the same resolved syntax, diff, and chart truth, so a raw-color regression, an unstated token, or an export-losing chart is visible in evidence rather than hidden behind hue
  - Syntax: 2 / diff: 2 / chart: 3
