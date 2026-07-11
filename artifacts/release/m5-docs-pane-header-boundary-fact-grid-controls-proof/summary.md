# M5 Docs-Pane-Header and Boundary-Fact-Grid Controls

- Packet: `m5-docs-pane-header-boundary-fact-grid-controls:stable:0001`
- Label: `M5 docs-pane-header and boundary-fact-grid controls with source class, version/pack identity, owner/origin, open-externally, find-in-page, and data-boundary truth`
- Consumer surfaces: 5
- Source classes: project_local, first_party_hosted, mirrored_vendor, extension_contributed, browser_handoff_required, source_unknown
- Proof freshness SLO: 168 hours (last refresh: 2026-07-10T00:00:00Z)

## Consumer surfaces

- **docs_browser_ui**: `stable`
  - Owner: Docs browser owner
  - Scope: The docs browser renders one docs-pane header per pane naming the source class, owner/origin, version, and last-updated state, so a user reads whether they are looking at project-local or mirrored vendor material without leaving the pane
  - Header examples: 2 / grid examples: 1
- **embedded_webview_ui**: `stable`
  - Owner: Embedded webview owner
  - Scope: Extension-contributed docs render inside an embedded webview whose header names the contributing extension and capability limits, and whose boundary-fact grid never masquerades as an approval or policy-authority surface
  - Header examples: 1 / grid examples: 2
- **marketplace_ui**: `stable`
  - Owner: Marketplace docs owner
  - Scope: Marketplace listing docs distinguish browser-handoff-required content from an undistinguishable source, degrading honestly when the source class cannot be told or the data boundary is unstated
  - Header examples: 2 / grid examples: 1
- **support_export**: `stable`
  - Owner: Support/export owner
  - Scope: The support export carries the same resolved header and grid truth, so a required browser handoff that is not exposed or an unexplained reading-trust claim is visible in evidence rather than hidden
  - Header examples: 1 / grid examples: 1
- **product_ui**: `stable`
  - Owner: In-product help owner
  - Scope: In-product help panes reuse the same source-class and data-boundary vocabulary a user sees in the docs browser, degrading honestly when the owner/origin is undisclosed or the reading posture is unstated rather than inventing local prose
  - Header examples: 2 / grid examples: 1
