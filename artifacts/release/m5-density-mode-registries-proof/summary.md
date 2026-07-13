# M5 Density-Mode Registries

- Packet: `m5-density-mode-registries:stable:0001`
- Label: `M5 density-mode registries with canonical comfortable / standard / compact row and control heights, tab / chip spacing, panel padding, and gutter spacing tokens, list / tree / table / tab / panel / editor / inspector surface-element coverage, profile-scope persistence with explained local overrides, and registry-bound tracing across shell, editor, review, notebook, data, and support surfaces`
- Consumer surfaces: 6
- Density modes: comfortable, standard, compact, mode_unclassified
- Surface elements: list, tree, table, tab, panel, editor, inspector
- Proof freshness SLO: 720 hours (last refresh: 2026-07-13T00:00:00Z)

## Consumer surfaces

- **shell_ui**: `stable`
  - Owner: Shell surface owner
  - Scope: The shell resolves the comfortable density scale from the shared registry and persists the choice at profile scope; a private per-widget scale and a silent provider-driven density switch degrade honestly instead of reading as a clean pass
  - Density-scale entries: 2 / persistence entries: 2
- **editor_ui**: `stable`
  - Owner: Editor surface owner
  - Scope: The editor resolves the standard density scale and keeps control hit targets above their 28 px minimum at high zoom; a control height below the supported minimum degrades honestly, and an explained presentation-viewer override is allowed
  - Density-scale entries: 2 / persistence entries: 1
- **review_ui**: `stable`
  - Owner: Review surface owner
  - Scope: The review surface resolves the compact density scale and keeps command meaning and focus order unchanged; a density change that would rearrange information architecture and an unexplained local override both degrade honestly, while an accessibility-viewer override is allowed
  - Density-scale entries: 2 / persistence entries: 2
- **data_ui**: `stable`
  - Owner: Data surface owner
  - Scope: The data surface resolves the compact density scale and keeps the density change presentation-only; a density change that would alter command / focus / trust and an unclassified persistence scope both degrade honestly instead of fracturing the layout
  - Density-scale entries: 2 / persistence entries: 1
- **settings_ui**: `stable`
  - Owner: Settings surface owner
  - Scope: The settings surface resolves the standard density scale across every surface element and persists the choice at profile scope; a density scale that omits the inspector element degrades honestly instead of claiming full coverage
  - Density-scale entries: 2 / persistence entries: 1
- **support_export**: `stable`
  - Owner: Support/export owner
  - Scope: The support export carries the same resolved density-scale and persistence truth, so a private scale or an unstated registry token is visible in evidence rather than hidden behind a screenshot
  - Density-scale entries: 2 / persistence entries: 1
