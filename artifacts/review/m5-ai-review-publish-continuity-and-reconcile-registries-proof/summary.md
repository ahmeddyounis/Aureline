# M5 Publish-Later-Draft and Compare-Reconcile-Review Registries

- Packet: `m5-ai-review-publish-continuity-and-reconcile-registries:stable:0001`
- Label: `M5 publish-later-draft and compare-reconcile-review registries recording one durable local draft per AI review finding that targets a provider object — one typed field per section: its publish-continuity state (provider-write-missing, kept-local-draft, exported-fallback, copied-forward, publish-later-queued, reconnect-repair-pending), the remote object identity it targets, the expected freshness floor, the target scope, the intended actor, and the conflict policy — each bound to one object-class identity, so a finding kept local, exported, or copied forward never wears a provider-committed badge, with canonical / accessible / audit resolution-form coverage, and a machine-readable compare-reconcile-review (reconciled-publish-ready, target-diff-drift-reconcile, or provider-edit-race-reconcile) that forces compare / reconcile when provider-side edits race the local draft or the target diff drifted materially instead of a silent last-writer-wins publish, and preserves the same local-draft packet — remote object identity, freshness floor, target scope, intended actor, and conflict policy — in local history and support / export so a draft can be reopened safely after reconnect or auth repair, across review, AI, provider, and support / export surfaces`
- Consumer surfaces: 6
- Report sections: provider_write_missing, kept_local_draft, exported_fallback, copied_forward, publish_later_queued, reconnect_repair_pending, publish_continuity_unclassified
- Resolution forms: canonical_object, accessible_summary, audit_record
- Proof freshness SLO: 720 hours (last audit: 2026-07-15T00:00:00Z)

## Consumer surfaces

- **review_detail**: `stable`
  - Owner: Review-detail owner
  - Scope: Review detail resolves an AI review finding to one durable publish-later draft — its publish-continuity state (provider-write-missing, kept-local-draft, exported-fallback, copied-forward, publish-later-queued, reconnect-repair-pending), the remote object identity it targets, the expected freshness floor, the target scope, the intended actor, and the conflict policy — from the shared registry and proves the compare-reconcile-review that keeps the path publish-ready; a draft missing its remote target identity and a reconcile that would silently overwrite a drifted diff degrade honestly instead of letting a local draft read as provider-committed
  - Correction-report entries: 2 / claim-history-diff entries: 2
- **ai_review_panel**: `stable`
  - Owner: AI-review-panel owner
  - Scope: The AI review panel resolves the keep-local, export, and copy-forward continuity states and the compare-reconcile-review (a provider-edit-race reconcile) while keeping the conflict policy and freshness floor visible; a draft that would wear a provider-committed badge without an accepted provider mutation and a resolution-form gap on a reconcile decision are caught before anything reads as committed
  - Correction-report entries: 2 / claim-history-diff entries: 2
- **support_export_packet**: `stable`
  - Owner: Support owner
  - Scope: Support / export resolves the same local-draft packet — remote object identity, expected freshness floor, target scope, intended actor, and conflict policy — keeping it reopenable after reconnect or auth repair, and reports the compare-reconcile-review outcome; a draft that is a hand-copied per-entry assumption and a decision on an unclassified reconcile scope degrade honestly
  - Correction-report entries: 2 / claim-history-diff entries: 1
- **finding_row**: `stable`
  - Owner: Finding-row owner
  - Scope: The finding row resolves the target-scope field and the target-diff-drift-reconcile decision bound to the registry so a materially drifted target diff forces compare / reconcile rather than a silent last-writer-wins publish; an unstated registry token on a publish-later draft is caught before its target identity can drift
  - Correction-report entries: 2 / claim-history-diff entries: 1
- **pending_review_tray**: `stable`
  - Owner: Pending-review-tray owner
  - Scope: The pending-review tray renders the same resolved publish-later-draft and compare-reconcile-review truth the resolvers produced across the canonical, accessible, and audit resolution forms rather than a hand-copied pending table; the local draft's remote object identity, freshness floor, and conflict policy stay inspectable off-renderer so a user can reopen a deferred draft safely after reconnect or auth repair without any provider-committed badge appearing
  - Correction-report entries: 1 / claim-history-diff entries: 1
- **provider_publish_review**: `stable`
  - Owner: Provider-publish-review owner
  - Scope: The provider publish-review feed carries the same resolved publish-later-draft and compare-reconcile-review truth, so a hand-copied constant, an unstated registry token, a local draft wearing a provider-committed badge without an accepted mutation, or a material drift committed as a silent last-writer-wins publish is visible in evidence — publish-ready, target-diff-drift reconcile, or provider-edit-race reconcile — rather than hidden behind a screenshot
  - Correction-report entries: 1 / claim-history-diff entries: 1
