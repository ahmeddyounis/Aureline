# M5 Ready-for-Review Handoff Sheet and Publish-Action Registries

- Packet: `m5-ready-for-review-handoff-registries:stable:0001`
- Label: `M5 ready-for-review-handoff-sheet and publish-action registries emitting one reusable machine-readable ready-for-review handoff sheet per tracked work item — one typed field per sheet section, summary-first: the changed scope, the checks / test state, the linked review object, the comment draft, the attached evidence, the provider mutation list, and the export-versus-publish actions — each bound to one commit state with its lineage, so a handoff never drops its changed scope / linked review / attached evidence and no local-only draft or queued publish reads as a provider-committed update, with canonical / accessible / audit resolution-form coverage, and a machine-readable publish-action object (publish now, queue for publish, or export a local packet) that keeps each publish outcome a visible, typed action — so a handoff never implies provider acceptance when the target is offline, policy-blocked, or only partially writable — across work-item detail, review detail, Git / worktree, provider-handoff, help / docs, and support / export surfaces`
- Consumer surfaces: 6
- Scope selectors: changed_files_scope, pull_request_scope, base_head_range_scope, worktree_uncommitted_scope, full_tree_scope, saved_pack_snapshot_scope, pack_scope_unclassified
- Resolution forms: canonical_object, accessible_summary, audit_record
- Proof freshness SLO: 720 hours (last audit: 2026-07-15T00:00:00Z)

## Consumer surfaces

- **work_item_detail**: `stable`
  - Owner: Work-item-detail owner
  - Scope: Work-item detail resolves a tracked work item to one typed ready-for-review handoff sheet — its changed scope, checks / test state, linked review object, comment draft, attached evidence, provider mutation list, and export-versus-publish actions, summary-first — from the shared registry and proves the commit-state disclosure for that item; a sheet dropping its changed scope or linked review and a publish-action that would let a local-only draft or queued publish read as a provider-committed update degrade honestly instead of implying provider acceptance the target has not given
  - Review-pack-record entries: 2 / review-pack-result entries: 2
- **review_detail**: `stable`
  - Owner: Review-detail owner
  - Scope: Review detail resolves the same ready-for-review handoff sheet from the tracked item and shows the checks / test state, the linked review object, and the attached evidence bound to their commit state; a sheet letting a queued or local publish read as provider-accepted and a summary-first-ordering gap are caught before a green summary can hide them, so review detail renders the same handoff truth as work-item detail without contradiction
  - Review-pack-record entries: 2 / review-pack-result entries: 2
- **support_export_packet**: `stable`
  - Owner: Support owner
  - Scope: Support resolves the handoff sheet's commit state while keeping the changed scope / linked review object / attached evidence and the queue-for-publish / export-local-packet attribution bound to the export, and reports the commit-state disclosure; a sheet that is a hand-copied per-item assumption and a publish-action on an unclassified binding degrade honestly so the changed scope, linked review, and attached evidence are never dropped on export or reopen
  - Review-pack-record entries: 2 / review-pack-result entries: 1
- **linked_change_panel**: `stable`
  - Owner: Linked-change-panel owner
  - Scope: The linked-change panel surface renders the same handoff sheet's linked review object and provider mutation list bound to their commit state — publish now, queue for publish, or export a local packet — from the registry so the publish modes can no longer be flattened into one generic action, and a target that is offline, policy-blocked, or only partially writable stays visible and actionable instead of implying provider acceptance; an unstated commit state on a sheet is caught before it can drift
  - Review-pack-record entries: 2 / review-pack-result entries: 1
- **ready_for_review_handoff**: `stable`
  - Owner: Ready-for-review-handoff owner
  - Scope: The ready-for-review handoff sheet renders the same resolved sheet and publish-action truth the resolvers produced across the canonical, accessible, and audit resolution forms rather than a hand-copied sheet, letting users compare publish-now, queue-for-publish, and export-local-packet outcomes from one sheet; the publish-action state and the checks / test state stay inspectable off-renderer so a local-only draft or queued publish never reads as a provider-committed update
  - Review-pack-record entries: 1 / review-pack-result entries: 1
- **help_docs**: `stable`
  - Owner: Help-docs owner
  - Scope: The help / docs feed carries the same resolved handoff sheet and publish-action truth, so a dropped evidence field, an unstated commit state, a local-only draft masquerading as a provider-committed update, or an offline / policy-blocked / partially-writable target shown as accepted is visible in evidence — a publish-mode change or a commit-state change — rather than hidden behind a green summary
  - Review-pack-record entries: 1 / review-pack-result entries: 1
