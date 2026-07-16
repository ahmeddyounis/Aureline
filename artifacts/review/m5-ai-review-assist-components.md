# M5 AI Review Assist: Finding Row, Review Scope Selector, Publish-to-Review Sheet, and Resolution Memory Matrix

- Packet: `m5-ai-review-assist:stable:0001`
- Label: `M5 AI-review-finding-row, review-scope-selector, publish-to-review-sheet, and resolution-memory-row matrix`
- Object classes: 4 (4 stable)
- AI-review-assist roles: finding_classification, analyzed_scope_disclosure, publish_destination_disclosure, local_versus_provider_state, lifecycle_state_tracking, publish_export_fallback, resolution_memory_disclosure
- Classification stages: finding_produced, scope_resolved, publish_destination_selected, publish_or_export_resolved, resolution_recorded
- Proof freshness SLO: 720 hours (last audit: 2026-07-16T00:00:00Z)

## Object classes

- **ai_review_finding_row**: `stable` (publish_state: `local_draft`)
  - Owner: AI review finding-row owner (backup: Review-governance backup owner)
  - Canonical schema: `schemas/review/m5-ai-review-finding.schema.json`
  - Scope: One reusable AI review finding row shows its finding class, severity, and confidence, names the analyzed diff scope it was produced from, shows its lifecycle state (open, outdated, suppressed), links its durable resolution memory, and never auto-approves, auto-requests changes, or auto-merges from a finding
  - Publish destination: held as a local draft; no provider comment, suggested patch, or check annotation yet
  - Lifecycle state: open
  - Required labels: identity, finding_role, canonical_reference, lifecycle_state
  - Accessibility routes: keyboard_focusable, screen_reader_announced, high_zoom_reflow, high_contrast_safe, cli_exportable, support_packet_present
- **review_scope_selector**: `stable` (publish_state: `local_draft`)
  - Owner: Review scope-selector owner (backup: AI-review-governance backup owner)
  - Canonical schema: `schemas/review/m5-ai-review-scope-selector.schema.json`
  - Scope: One review scope selector names the analyzed diff range plus the repo instruction and enabled check source that bound it, flags scope drift, and offers a rerun-within-scope safe next step so findings never silently outlive the diff they were bound to
  - Publish destination: no publish destination; scope selection stays local until findings are published
  - Lifecycle state: rerun recommended when the diff or instruction source drifts from the analyzed scope
  - Required labels: identity, finding_role, canonical_reference, finding_class_badge
  - Accessibility routes: keyboard_focusable, screen_reader_announced, high_zoom_reflow, high_contrast_safe, cli_exportable, support_packet_present
- **publish_to_review_sheet**: `stable` (publish_state: `publish_now_provider_comment`)
  - Owner: Publish-to-review-sheet owner (backup: Provider-governance backup owner)
  - Canonical schema: `schemas/review/m5-ai-review-publish-sheet.schema.json`
  - Scope: One publish-to-review sheet shows the publish mode (local draft, publish now, open in provider), names the provider destination (comment, suggested patch, check annotation), shows local-draft-versus-provider-committed state before mutation, and offers a publish-or-export fallback so AI review output never publishes or merges implicitly
  - Publish destination: publish destination: a provider review comment on the connected provider
  - Lifecycle state: published
  - Required labels: identity, finding_role, canonical_reference, publish_destination
  - Accessibility routes: keyboard_focusable, screen_reader_announced, high_zoom_reflow, high_contrast_safe, cli_exportable, support_packet_present
- **resolution_memory_row**: `stable` (publish_state: `export_fallback_offline`)
  - Owner: Resolution-memory-row owner (backup: Support-governance backup owner)
  - Canonical schema: `schemas/review/m5-ai-review-resolution-memory.schema.json`
  - Scope: One resolution memory row shows the resolution state (dismissed, published, suppressed), shows finding freshness and outdated state, names a reopen-or-rerun path, and preserves the local draft and evidence when a publish fails so a finding's durable history stays provable and no stale finding resurfaces as current
  - Publish destination: no publish destination; the resolution is recorded locally and included in the export packet
  - Lifecycle state: dismissed, published, outdated, suppressed, or rerun recommended as the finding's durable state
  - Required labels: identity, finding_role, canonical_reference, lifecycle_state
  - Accessibility routes: keyboard_focusable, screen_reader_announced, high_zoom_reflow, high_contrast_safe, cli_exportable, support_packet_present
