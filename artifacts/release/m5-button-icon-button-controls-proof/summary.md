# M5 Button and Icon-Button Controls

- Packet: `m5-button-icon-button-controls:stable:0001`
- Label: `M5 button and icon-button controls with primary/secondary/quiet/destructive/ghost emphasis, loading-attribution-preserving states, locked/degraded distinctness, and canonical command parity aligned across forms, settings, review, entry, support, and product surfaces`
- Consumer surfaces: 6
- Button emphases: primary, secondary, quiet, destructive, ghost, link
- Loading behaviors: not_loading, label_preserved_spinner_leading, label_preserved_spinner_trailing, inline_progress_label_kept, width_reserved_label_kept, behavior_unknown
- Proof freshness SLO: 720 hours (last refresh: 2026-07-12T00:00:00Z)

## Consumer surfaces

- **forms_ui**: `stable`
  - Owner: Forms surface owner
  - Scope: The forms surface names one permanent action label and stable primary/secondary emphasis, preserves the label and width while a submit or save is in flight, and offers a labeled icon button with command parity; both degrade honestly when the label is unstated or a loading button loses attribution
  - Button examples: 4 / icon-button examples: 2
- **settings_ui**: `stable`
  - Owner: Settings surface owner
  - Scope: The settings surface keeps quiet and ghost emphasis distinct in disabled and locked states, showing locked semantics distinctly rather than behind generic disabled chrome, and keeps icon tooltip parity; both degrade honestly when a lock hides behind disabled or tooltip parity drifts
  - Button examples: 3 / icon-button examples: 2
- **review_ui**: `stable`
  - Owner: Review surface owner
  - Scope: The review sheet keeps destructive triggers appropriately risky and always labeled, reusing the shared emphasis grammar rather than a feature-local style fork, and never leaves an icon-only destructive action unlabeled; both degrade honestly when a style is forked or a destructive icon goes unlabeled
  - Button examples: 2 / icon-button examples: 2
- **entry_ui**: `stable`
  - Owner: Start-center entry owner
  - Scope: The start center reuses the same primary emphasis and named icon affordances a user sees elsewhere, never encoding emphasis by color alone and never inventing a brand-only affordance; both degrade honestly when emphasis is color-only or a brand-only affordance is invented
  - Button examples: 2 / icon-button examples: 2
- **support_export**: `stable`
  - Owner: Support/export owner
  - Scope: The support export carries the same resolved action and command truth, so an unstated command binding, a broken command parity, or a missing canonical command ID is visible in evidence rather than hidden behind generic chrome
  - Button examples: 2 / icon-button examples: 2
- **product_ui**: `stable`
  - Owner: In-product action owner
  - Scope: In-product surfaces reuse the same action label, emphasis, and command grammar a user sees in forms and settings, always offering the command-backed detail path and degrading honestly when the trace path is missing
  - Button examples: 2 / icon-button examples: 2
