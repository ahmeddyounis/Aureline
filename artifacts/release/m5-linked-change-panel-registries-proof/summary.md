# M5 Linked-Change-Panel and Linked-Change-Relation Registries

- Packet: `m5-linked-change-panel-registries:stable:0001`
- Label: `M5 linked-change-panel and linked-change-relation registries emitting one reusable machine-readable linked-change panel per tracked work item — one typed field per panel section: the branch / worktree state, the hosted review state, the validation summary, the AI run / evidence refs, the incident / docs links, and the relation-source class — each bound to one relation source with its freshness lineage, so a linked change never drops its branch / worktree / review identity and no locally linked, suggested, or stale relation reads as a provider-authoritative link, with canonical / accessible / audit resolution-form coverage, and a machine-readable linked-change-relation object (linked by provider, linked locally, suggested by Aureline, stale relation, broken relation, or queued for publish) that keeps each relation state a visible, typed relation — so a stale or broken relation stays actionable rather than collapsing into missing context — across work-item detail, review detail, Git / worktree, linked-change, provider-handoff, help / docs, and support / export surfaces`
- Consumer surfaces: 6
- Scope selectors: changed_files_scope, pull_request_scope, base_head_range_scope, worktree_uncommitted_scope, full_tree_scope, saved_pack_snapshot_scope, pack_scope_unclassified
- Resolution forms: canonical_object, accessible_summary, audit_record
- Proof freshness SLO: 720 hours (last audit: 2026-07-15T00:00:00Z)

## Consumer surfaces

- **work_item_detail**: `stable`
  - Owner: Work-item-detail owner
  - Scope: Work-item detail resolves a tracked work item to one typed linked-change panel — its branch / worktree state, hosted review state, validation summary, AI run / evidence refs, incident / docs links, and the relation-source class each artifact carries — from the shared registry and proves the relation-source disclosure for that item; a panel missing its relation source and a relation that would let a locally linked or suggested change read as provider-authoritative degrade honestly instead of leaving a stale or broken relation to collapse into missing context
  - Review-pack-record entries: 2 / review-pack-result entries: 2
- **review_detail**: `stable`
  - Owner: Review-detail owner
  - Scope: Review detail resolves the same linked-change panel from the tracked item and shows the hosted review state, the validation summary, and the AI run / evidence refs bound to their relation source; a panel widening a locally linked or suggested relation into a provider-authoritative reading and a relation-source gap are caught before a green summary can hide them, so review detail renders the same linked-change truth as work-item detail without contradiction
  - Review-pack-record entries: 2 / review-pack-result entries: 2
- **support_export_packet**: `stable`
  - Owner: Support owner
  - Scope: Support resolves the linked-change panel's relation source while keeping the branch / worktree / review identity and the queued-for-publish / stale-or-broken relation attribution bound to the export, and reports the relation-source disclosure; a panel that is a hand-copied per-item assumption and a relation on an unclassified relation binding degrade honestly so the linked branch / worktree / review identity is never dropped on export or reopen
  - Review-pack-record entries: 2 / review-pack-result entries: 1
- **linked_change_panel**: `stable`
  - Owner: Linked-change-panel owner
  - Scope: The linked-change panel resolves the linked branch / worktree / review identity and the relation-source state — linked by provider, linked locally, suggested by Aureline, stale relation, broken relation, or queued for publish — bound to the registry so the relation sources can no longer be flattened into one generic badge, and a stale or broken relation stays visible and actionable instead of collapsing into missing context; an unstated relation source on a panel is caught before it can drift
  - Review-pack-record entries: 2 / review-pack-result entries: 1
- **ready_for_review_handoff**: `stable`
  - Owner: Ready-for-review-handoff owner
  - Scope: The ready-for-review handoff renders the same resolved linked-change panel and relation truth the resolvers produced across the canonical, accessible, and audit resolution forms rather than a hand-copied sheet; the queued-for-publish / stale-or-broken-relation state and the validation summary stay inspectable off-renderer so a locally linked or suggested relation never reads as a provider-authoritative link
  - Review-pack-record entries: 1 / review-pack-result entries: 1
- **help_docs**: `stable`
  - Owner: Help-docs owner
  - Scope: The help / docs feed carries the same resolved linked-change panel and relation truth, so a dropped relation source, an unstated branch / worktree / review identity, a locally linked relation masquerading as provider-authoritative, or a stale-or-broken relation shown as current is visible in evidence — a relation-source change or a freshness change — rather than hidden behind a green summary
  - Review-pack-record entries: 1 / review-pack-result entries: 1
