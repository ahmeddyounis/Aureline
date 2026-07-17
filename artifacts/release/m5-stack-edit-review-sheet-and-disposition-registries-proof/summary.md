# M5 Stack-Edit-Review-Sheet and Disposition Registries

- Packet: `m5-stack-edit-review-sheet-and-disposition-registries:stable:0001`
- Label: `M5 stack-edit review sheet and disposition registries emitting one durable machine-readable review sheet per proposed reorder, split, squash, or restack — one typed field per section: the original order, the proposed order, the affected parent/child links, the stale validation or approval impact, and the resulting branch/worktree consequences — with the explicit continue, abort, export, and defer disposition of the proposed plan kept separate, so no stack edit executes without a review surface showing ordering, dependency, stale-evidence, and landing consequences and no member is silently reordered, collapsed, or retargeted, with canonical / accessible / audit resolution-form coverage, and a machine-readable disposition that preserves the proposed re-stack plan for retry or export when a provider write, a hosted approval, a local validation, or a policy boundary goes stale or blocks publish — so a local-only continue never reads as a provider-committed landing and nothing lands from ambient branch state — rather than a green summary across Git, patch-stack / queue, review, provider-landing, help / docs, and support / export surfaces`
- Consumer surfaces: 6
- Scope selectors: changed_files_scope, pull_request_scope, base_head_range_scope, worktree_uncommitted_scope, full_tree_scope, saved_pack_snapshot_scope, pack_scope_unclassified
- Resolution forms: canonical_object, accessible_summary, audit_record
- Proof freshness SLO: 720 hours (last audit: 2026-07-15T00:00:00Z)

## Consumer surfaces

- **change_object_detail**: `stable`
  - Owner: Change-object-detail owner
  - Scope: The change-object detail resolves the stack-edit review sheet for a proposed reorder, split, squash, or restack from the shared registry — the original order, the proposed order, the affected parent/child links, the stale validation or approval impact, and the resulting branch/worktree consequences — and keeps the explicit continue, abort, and export disposition of that plan bound separately; a review sheet missing its original-versus-proposed ordering and a disposition that would let a local-only continue read as a provider-committed landing degrade honestly instead of executing a stack edit without a review surface
  - Review-pack-record entries: 2 / review-pack-result entries: 2
- **patch_stack_queue**: `stable`
  - Owner: Patch-stack-queue owner
  - Scope: The patch-stack / queue resolves the reorder and restack review sheets in order — original order, proposed order, affected parent/child links, stale validation or approval impact, and resulting branch/worktree consequences — so which member a proposed edit reorders ahead of another, which checks the edit invalidates, and which members become queue-eligible, blocked, or no-longer-landing-candidates are visible before apply rather than after; a proposed reorder that would silently retarget a member and a squash that hides a stale approval are caught before a green summary can hide them, so no member is silently reordered, collapsed, or retargeted and nothing lands from ambient branch state
  - Review-pack-record entries: 2 / review-pack-result entries: 2
- **support_export_packet**: `stable`
  - Owner: Support owner
  - Scope: Support resolves each stack-edit review sheet's original-versus-proposed order, affected parent/child links, and stale validation or approval impact while keeping the continue, abort, and export disposition of the proposed plan bound separately to the export; a review sheet that is a hand-copied per-entry assumption and a disposition left unclassified degrade honestly so the proposed re-stack plan, its ordering, and its stale-evidence impact are never dropped on export or reopen — a user can export or defer the plan instead of losing it when a provider or policy boundary blocks publish
  - Review-pack-record entries: 2 / review-pack-result entries: 1
- **stack_edit_review_sheet**: `stable`
  - Owner: Stack-edit-review-sheet owner
  - Scope: The stack-edit / review sheet resolves the reorder, split, squash, and restack operations bound to the registry — original order, proposed order, affected parent/child links, stale validation or approval impact, and resulting branch/worktree consequences — so a proposed edit's parent-child consequence, stale-check impact, and target-branch disclosure can no longer be flattened into one generic badge; a review sheet with an unstated original-versus-proposed ordering is caught before any stack edit executes without a review surface
  - Review-pack-record entries: 2 / review-pack-result entries: 1
- **provider_merge_queue**: `stable`
  - Owner: Provider-merge-queue owner
  - Scope: The provider merge queue renders the same resolved stack-edit review and disposition truth the resolvers produced across the canonical, accessible, and audit resolution forms rather than a hand-copied sheet; the continue, abort, export, and defer disposition and the stale validation or approval impact stay inspectable off-renderer so a proposed re-stack that a provider or policy boundary blocks from publish preserves its plan for retry or export and a local-only continue never reads as a provider-committed landing
  - Review-pack-record entries: 1 / review-pack-result entries: 1
- **help_docs**: `stable`
  - Owner: Help-docs owner
  - Scope: The help / docs feed carries the same resolved stack-edit review and disposition truth, so a dropped original-versus-proposed ordering, an unstated parent/child consequence, a stale approval shown as still-current, or a proposed re-stack silently retargeted onto a different target branch is visible in evidence — a reorder consequence, a stale-check impact, or a target-branch disclosure — rather than hidden behind a green summary
  - Review-pack-record entries: 1 / review-pack-result entries: 1
