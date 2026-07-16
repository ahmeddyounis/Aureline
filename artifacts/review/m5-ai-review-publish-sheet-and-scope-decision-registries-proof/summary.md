# M5 Publish-to-Review-Sheet and Publish-Scope-Decision Registries

- Packet: `m5-ai-review-publish-sheet-and-scope-decision-registries:stable:0001`
- Label: `M5 publish-to-review-sheet and publish-scope-decision registries emitting one machine-readable publish-to-review sheet per outbound AI review publish action — one typed field per section: the target provider, the thread or check-run target, the outbound text preview, the review artifact class (comment, suggested patch, or provider-specific check annotation), the attribution state, the redaction note, and the publish / copy / export / cancel actions — each bound to one object-class identity, so an outbound action never publishes or merges implicitly and never hides whether output stays local, becomes a provider comment, a suggested patch, or a check annotation, with canonical / accessible / audit resolution-form coverage, and a machine-readable publish-scope-decision (publish-scope-allowed, publish-scope-downgraded, or publish-scope-blocked) that turns missing or narrowed provider scope into an explicit publish-state explanation with copy / export fallback rather than a generic publish failure, and preserves the same publish packet — attribution, destination, and redaction state — in local history and support / export so outbound review state stays auditable outside the live provider UI, across review, AI, provider, and support / export surfaces`
- Consumer surfaces: 6
- Report sections: local_draft, publish_now_provider_comment, publish_now_suggested_patch, publish_now_check_annotation, open_in_provider, export_fallback_offline, publish_mode_unclassified
- Resolution forms: canonical_object, accessible_summary, audit_record
- Proof freshness SLO: 720 hours (last audit: 2026-07-15T00:00:00Z)

## Consumer surfaces

- **review_detail**: `stable`
  - Owner: Review-detail owner
  - Scope: Review detail resolves an AI review finding to one typed publish-to-review sheet — the target provider, thread or check-run target, outbound text preview, review artifact class (comment, suggested patch, or provider-specific check annotation), attribution state, redaction note, and publish / copy / export / cancel actions — from the shared registry and proves the publish-scope-decision that makes the publish path allowed; a sheet missing its outbound destination or text preview and a scope decision that flattens a provider write failure into a generic error degrade honestly instead of letting an outbound action look ready
  - Correction-report entries: 2 / claim-history-diff entries: 2
- **ai_review_panel**: `stable`
  - Owner: AI-review-panel owner
  - Scope: The AI review panel resolves the publish, copy, export, and cancel actions and the publish-scope-decision (a blocked publish path) while keeping the active permission-scope reason visible; a publish sheet that would commit implicitly without an explicit outbound preview and a resolution-form gap on a scope decision are caught before anything becomes durable provider history
  - Correction-report entries: 2 / claim-history-diff entries: 2
- **support_export_packet**: `stable`
  - Owner: Support owner
  - Scope: Support / export resolves the same publish packet — destination, attribution, and redaction state — keeping it auditable outside the live provider UI, and reports the publish-scope-decision outcome; a sheet that is a hand-copied per-entry assumption and a decision on an unclassified publish scope degrade honestly
  - Correction-report entries: 2 / claim-history-diff entries: 1
- **finding_row**: `stable`
  - Owner: Finding-row owner
  - Scope: The finding row resolves the review-artifact-class field and the publish-scope-downgraded decision bound to the registry so a narrowed provider scope surfaces as an explicit publish-state explanation with copy / export fallback rather than a generic failure; an unstated registry token on a publish sheet is caught before it can drift
  - Correction-report entries: 2 / claim-history-diff entries: 1
- **publish_to_review_sheet**: `stable`
  - Owner: Publish-to-review-sheet owner
  - Scope: The publish-to-review sheet renders the same resolved publish-to-review-sheet and publish-scope-decision truth the resolvers produced across the canonical, accessible, and audit resolution forms rather than a hand-copied outbound table; the outbound destination and text preview stay inspectable off-renderer so a user always reviews exactly what leaves the client before it becomes durable provider history
  - Correction-report entries: 1 / claim-history-diff entries: 1
- **provider_publish_review**: `stable`
  - Owner: Provider-publish-review owner
  - Scope: The provider publish-review feed carries the same resolved publish-to-review-sheet and publish-scope-decision truth, so a hand-copied constant, an unstated registry token, an outbound action publishing without an explicit destination and preview, or a provider write failure flattened into a generic error is visible in evidence — publish allowed, downgraded, or blocked — rather than hidden behind a screenshot
  - Correction-report entries: 1 / claim-history-diff entries: 1
