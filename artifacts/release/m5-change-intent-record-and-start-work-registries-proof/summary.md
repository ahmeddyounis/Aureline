# M5 Change-Intent-Record and Start-Work-Sheet Registries

- Packet: `m5-change-intent-record-and-start-work-registries:stable:0001`
- Label: `M5 change-intent-record and start-work-sheet registries emitting one durable machine-readable change-intent record per tracked work item — one typed field per record section: the canonical work-item identity and provider ownership, the linked workspace / root and branch / worktree refs, the optional linked review target and task / test preset refs, the actor lineage, and the local-versus-provider commit state — each bound to one provider ownership with its commit-state lineage, so a change intent never drops its work-item identity or provider ownership and no local-only draft or queued publish reads as a provider-committed update, with canonical / accessible / audit resolution-form coverage, and a machine-readable start-work sheet (create-new-linked-branch / worktree, link-existing-branch-or-review, provider-link mutation, or local-only alternative) that discloses each start-work side effect as a visible, typed disclosure — provider-committed, local-only draft, queued for publish, offline handoff packet, or stale relative to provider — rather than a green summary across work-item, start-work, linked-change, provider-handoff, help / docs, and support / export surfaces`
- Consumer surfaces: 6
- Scope selectors: changed_files_scope, pull_request_scope, base_head_range_scope, worktree_uncommitted_scope, full_tree_scope, saved_pack_snapshot_scope, pack_scope_unclassified
- Resolution forms: canonical_object, accessible_summary, audit_record
- Proof freshness SLO: 720 hours (last audit: 2026-07-15T00:00:00Z)

## Consumer surfaces

- **work_item_detail**: `stable`
  - Owner: Work-item-detail owner
  - Scope: Work-item detail resolves a tracked work item to one typed change-intent record — its canonical work-item identity and provider ownership, the linked workspace / root and branch / worktree refs, the local-versus-provider commit state, and its actor lineage — from the shared registry and proves the start-work side-effect disclosure for that item; a record missing its provider ownership and a start-work sheet that would let a local handoff read as a provider-committed update degrade honestly instead of leaving a local-only draft to read as an authoritative tracked-item update
  - Review-pack-record entries: 2 / review-pack-result entries: 2
- **start_work_sheet**: `stable`
  - Owner: Start-work-sheet owner
  - Scope: The start-work sheet resolves the local-versus-provider commit-state binding and separately discloses each start-work side effect — create-new-linked branch / worktree, link-existing-branch-or-review, the provider-link mutation, and the local-only alternative — before commit; a record widening a local-only draft into a provider-committed reading and a disclosure gap on a side effect are caught before a green summary can hide them, so start work can never silently create a branch, worktree, review draft, or provider link
  - Review-pack-record entries: 2 / review-pack-result entries: 2
- **support_export_packet**: `stable`
  - Owner: Support owner
  - Scope: Support resolves the change-intent record's provider ownership while keeping the local-versus-provider commit state and the queued-publish / offline-handoff attribution bound to the export, and reports the start-work side-effect disclosure; a record that is a hand-copied per-entry assumption and a start-work sheet on an unclassified disclosure binding degrade honestly so the work-item identity and actor lineage are never dropped on export or reopen
  - Review-pack-record entries: 2 / review-pack-result entries: 1
- **linked_change_panel**: `stable`
  - Owner: Linked-change-panel owner
  - Scope: The linked-change panel resolves the linked branch / worktree / review identity and the relation-source state — linked by provider, linked locally, suggested by Aureline, or stale or broken — bound to the registry so the four relation sources can no longer be flattened into one generic badge; an unstated provider ownership on a record is caught before it can drift
  - Review-pack-record entries: 2 / review-pack-result entries: 1
- **ready_for_review_handoff**: `stable`
  - Owner: Ready-for-review-handoff owner
  - Scope: The ready-for-review handoff renders the same resolved change-intent record and start-work-sheet truth the resolvers produced across the canonical, accessible, and audit resolution forms rather than a hand-copied sheet; the queued-for-publish / offline-handoff-packet / provider-unavailable state and the validation evidence stay inspectable off-renderer so a local handoff packet never reads as a provider-committed update
  - Review-pack-record entries: 1 / review-pack-result entries: 1
- **help_docs**: `stable`
  - Owner: Help-docs owner
  - Scope: The help / docs feed carries the same resolved change-intent record and start-work-sheet truth, so a dropped provider ownership, an unstated commit state, a local-only draft masquerading as provider-committed, or a stale-relative-to-provider record shown as current is visible in evidence — a commit-state change, a relation-source change, or a blocker-state change — rather than hidden behind a green summary
  - Review-pack-record entries: 1 / review-pack-result entries: 1
