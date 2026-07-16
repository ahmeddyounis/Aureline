# M5 Review-Scope-Selector-State and Rerun/Outdated-Freshness Registries

- Packet: `m5-ai-review-scope-selector-and-rerun-state-registries:stable:0001`
- Label: `M5 review-scope-selector-state and rerun/outdated-freshness registries emitting one machine-readable scope-selector state per AI review run — one typed field per section: the analyzed review scope (selected diff, uncommitted changes, pull / merge request, base..head range, staged changes, saved review snapshot), the base / head context, the repo-instruction / enabled-check-pack source, the freshness and in-scope rerun action, and the retained-versus-re-resolved lineage — each bound to one object-class identity, so a finding never hides whether it came from selected lines, local uncommitted changes, or a hosted review object, with canonical / accessible / audit resolution-form coverage, and a machine-readable rerun-freshness diff (analyzed-diff-changed, base-head-context-shifted, or saved-snapshot-mismatch) that turns a changed diff scope or shifted base / head into a visible, typed freshness event marking the prior finding outdated / rerun-recommended rather than a silent mutation across review, AI, provider, and support / export surfaces`
- Consumer surfaces: 6
- Report sections: selected_diff, uncommitted_changes, pull_merge_request, base_head_range, staged_changes, saved_review_snapshot, review_scope_unclassified
- Resolution forms: canonical_object, accessible_summary, audit_record
- Proof freshness SLO: 720 hours (last audit: 2026-07-15T00:00:00Z)

## Consumer surfaces

- **review_detail**: `stable`
  - Owner: Review-detail owner
  - Scope: Review detail resolves an AI review run to one typed review-scope-selector state — the analyzed scope (selected diff), base / head context, repo-instruction / check-pack source, freshness, and in-scope rerun action — from the shared registry and proves the analyzed-diff-changed freshness diff for that finding; a scope state missing its base / head joins and a freshness diff that keeps a stale finding looking current degrade honestly instead of leaving a finding to read as fresh
  - Correction-report entries: 2 / claim-history-diff entries: 2
- **ai_review_panel**: `stable`
  - Owner: AI-review-panel owner
  - Scope: The AI review panel resolves the in-scope rerun action and the base-head-context-shifted freshness diff while keeping the active drift reason visible; a rerun widening its scope without preserved prior lineage and a resolution-form gap on a freshness diff are caught before a rerun can reintroduce a falsely-fresh finding
  - Correction-report entries: 2 / claim-history-diff entries: 2
- **support_export_packet**: `stable`
  - Owner: Support owner
  - Scope: Support / export resolves the repo-instruction / check-pack source field while keeping its scope claim matched to the analyzed diff and reports the rerun-freshness-diff outcome; a scope entry that is a hand-copied per-entry assumption and a diff on an unclassified freshness drift degrade honestly
  - Correction-report entries: 2 / claim-history-diff entries: 1
- **finding_row**: `stable`
  - Owner: Finding-row owner
  - Scope: The finding row resolves the analyzed-scope field and the saved-snapshot-mismatch freshness diff bound to the registry so a prior finding can no longer read as current once its saved review snapshot no longer matches the target; an unstated registry token on a scope entry is caught before it can drift
  - Correction-report entries: 2 / claim-history-diff entries: 1
- **review_scope_selector**: `stable`
  - Owner: Review-scope-selector owner
  - Scope: The review scope selector renders the same resolved review-scope-selector and rerun-freshness-diff truth the resolvers produced across the canonical, accessible, and audit resolution forms rather than a hand-copied scope table; the in-scope rerun action and the base-head-context-shifted freshness diff stay inspectable off-renderer so a rerun always re-resolves current scope before new output is shown
  - Correction-report entries: 1 / claim-history-diff entries: 1
- **provider_publish_review**: `stable`
  - Owner: Provider-publish-review owner
  - Scope: The provider publish-review feed carries the same resolved review-scope-selector and rerun-freshness-diff truth, so a hand-copied constant, an unstated registry token, a scope state widening beyond its selected diff without preserved lineage, or a stale finding shown as current is visible in evidence — an analyzed-diff change, a base / head-context shift, or a saved-snapshot mismatch — rather than hidden behind a screenshot
  - Correction-report entries: 1 / claim-history-diff entries: 1
