# M5 Resolution-Memory-Row and Finding-Lifecycle-Transition Registries

- Packet: `m5-ai-review-resolution-memory-and-finding-lifecycle-registries:stable:0001`
- Label: `M5 resolution-memory-row and finding-lifecycle-transition registries recording one durable resolution-memory row per AI review finding-state transition — one typed field per section: the finding's lifecycle state (open, dismissed, suppressed, published, outdated, reopened), the actor / source and rationale class captured on the transition without shaming language or anthropomorphic copy, the timestamp, the reopen action, and any provider destination or local-draft relation — each bound to one object-class identity, so a dismissed finding never collapses into the same generic hidden state as a suppressed one and a stale finding never keeps looking current after diff or instruction drift, with canonical / accessible / audit resolution-form coverage, and a machine-readable finding-lifecycle transition (published-transition-joined, outdated-transition-joined, or reopened-transition-joined) that joins each published, outdated, or reopened state back to the original finding and diff scope so later review and support exports can reconstruct the full lifecycle, and preserves the same finding packet — stable ID, actor / source, rationale class, and destination or local-draft relation — in local history and support / export so the lifecycle stays available after restart, export, and support capture without implying provider commitment where none exists, across review, AI, provider, and support / export surfaces`
- Consumer surfaces: 6
- Report sections: open, dismissed, suppressed, published, outdated, reopened, resolution_state_unclassified
- Resolution forms: canonical_object, accessible_summary, audit_record
- Proof freshness SLO: 720 hours (last audit: 2026-07-15T00:00:00Z)

## Consumer surfaces

- **review_detail**: `stable`
  - Owner: Review-detail owner
  - Scope: Review detail resolves an AI review finding to one durable resolution-memory row — its lifecycle state (open, dismissed, suppressed, published, outdated, reopened), the actor / source and rationale class captured on the transition, the timestamp, the reopen action, and any provider destination or local-draft relation — from the shared registry and proves the finding-lifecycle transition that joins each published, outdated, or reopened state back to the original finding; a row that collapses a transition into an unclassified state and a transition that hides the join back to the original finding degrade honestly instead of letting a dismissal, suppression, or staleness decision disappear into UI state
  - Correction-report entries: 2 / claim-history-diff entries: 2
- **ai_review_panel**: `stable`
  - Owner: AI-review-panel owner
  - Scope: The AI review panel resolves the dismiss, suppress, publish, and reopen transitions and the finding-lifecycle transition (a reopened state joined back to prior lineage) while keeping the actor / source and rationale class visible; a resolution-memory row that would collapse a dismissed / suppressed transition into a generic hidden state and a resolution-form gap on a transition are caught before the finding's history becomes unreconstructable
  - Correction-report entries: 2 / claim-history-diff entries: 2
- **support_export_packet**: `stable`
  - Owner: Support owner
  - Scope: Support / export resolves the same finding packet — stable ID, actor / source, rationale class, and destination or local-draft relation — keeping the lifecycle available outside the live provider UI without implying provider commitment, and reports the finding-lifecycle transition outcome; a row that is a hand-copied per-entry assumption and a transition on an unclassified lifecycle scope degrade honestly
  - Correction-report entries: 2 / claim-history-diff entries: 1
- **finding_row**: `stable`
  - Owner: Finding-row owner
  - Scope: The finding row resolves the rationale-class field and the outdated-transition-joined state bound to the registry so a stale finding surfaces as an explicit outdated state joined back to its original finding and diff scope rather than keeping it looking current after drift; an unstated registry token on a resolution-memory row is caught before its history can drift
  - Correction-report entries: 2 / claim-history-diff entries: 1
- **resolution_memory_ledger**: `stable`
  - Owner: Resolution-memory-ledger owner
  - Scope: The resolution memory ledger renders the same resolved resolution-memory-row and finding-lifecycle-transition truth the resolvers produced across the canonical, accessible, and audit resolution forms rather than a hand-copied history table; the finding's stable ID, actor / source, and rationale class stay inspectable off-renderer so a user can always reconstruct exactly what happened to a finding through dismiss, suppress, publish, outdated, and reopen transitions
  - Correction-report entries: 1 / claim-history-diff entries: 1
- **provider_publish_review**: `stable`
  - Owner: Provider-publish-review owner
  - Scope: The provider publish-review feed carries the same resolved resolution-memory-row and finding-lifecycle-transition truth, so a hand-copied constant, an unstated registry token, a transition collapsing into a generic hidden state, or a published / outdated / reopened state severed from its original finding is visible in evidence — the state joined back to its finding and diff scope — rather than hidden behind a screenshot
  - Correction-report entries: 1 / claim-history-diff entries: 1
