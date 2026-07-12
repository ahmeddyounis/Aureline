# M5 Text-Field and Search-Field Controls

- Packet: `m5-text-field-search-field-controls:stable:0001`
- Label: `M5 text-field and search-field controls with permanent labels, specific validation copy, focus-visible treatment, and reveal/clear/submit and retention/privacy truth aligned across forms, settings, search, entry, support, and product surfaces`
- Consumer surfaces: 6
- Field label modes: persistent_label, floating_label, label_plus_placeholder, aria_label_only, placeholder_only_disallowed, label_unresolved
- Search retention postures: live_not_retained, history_private, cached_results_disclosed, provider_backed_remote, export_sensitive, retention_unknown
- Proof freshness SLO: 720 hours (last refresh: 2026-07-12T00:00:00Z)

## Consumer surfaces

- **forms_ui**: `stable`
  - Owner: Forms surface owner
  - Scope: The forms surface renders a text field with a permanent label and a search field with a search icon, clear affordance, and submit model; both degrade honestly when the label is placeholder-only
  - Text-field examples: 2 / search-field examples: 2
- **settings_ui**: `stable`
  - Owner: Settings surface owner
  - Scope: The settings surface keeps read-only and locked text fields distinct rather than behind generic disabled chrome, and discloses a cached search's retention cue; both degrade honestly when a lock hides behind disabled or a privacy cue is missing
  - Text-field examples: 3 / search-field examples: 2
- **search_ui**: `stable`
  - Owner: Search surface owner
  - Scope: The search surface keeps a provider-backed query's scope disclosed and its clear affordance present, and keeps text-field validation copy specific; both degrade honestly when validation copy is vague, the clear affordance is missing, or the search icon cue is missing
  - Text-field examples: 2 / search-field examples: 3
- **entry_ui**: `stable`
  - Owner: Start-center entry owner
  - Scope: The start-center entry surface offers a sensitive text field with a reveal control and a private-history search with a resolved submit model; both degrade honestly when the reveal control is missing or the submit model is unresolved
  - Text-field examples: 2 / search-field examples: 2
- **support_export**: `stable`
  - Owner: Support/export owner
  - Scope: The support export carries the same resolved label, validation, retention, and submit truth, so an unstated command binding or a blocked search hidden behind disabled is visible in evidence rather than hidden behind generic chrome
  - Text-field examples: 2 / search-field examples: 2
- **product_ui**: `stable`
  - Owner: In-product control owner
  - Scope: In-product surfaces reuse the same permanent-label, validation-anchor, and draft-continuity grammar a user sees in forms and settings, always offering the command-backed detail path and degrading honestly when draft continuity is lost, a validation anchor is lost, or the trace path is missing
  - Text-field examples: 3 / search-field examples: 3
