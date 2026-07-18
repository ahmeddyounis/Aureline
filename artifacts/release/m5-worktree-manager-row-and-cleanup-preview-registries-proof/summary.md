# M5 Worktree-Manager-Row and Cleanup-Preview Registries

- Packet: `m5-worktree-manager-row-and-cleanup-preview-registries:stable:0001`
- Label: `M5 worktree-manager-row and cleanup-preview registries emitting one durable machine-readable worktree-manager / switcher row per alternate working context — one typed field per section: the real worktree path, the checked-out ref, the divergence from base, the dirty / uncommitted state, the running-task / open-editor presence, and the recovery / checkpoint lineage — with the cleanup preview of that worktree kept separate, so a user can distinguish active, orphaned, abandoned, and cleanup-ready worktrees and side branches without shelling out and no member is silently reordered, collapsed, or retargeted, with canonical / accessible / audit resolution-form coverage, and a machine-readable cleanup preview that never feels like `rm -rf and hope` — it names the affected running tasks, open editors, and uncommitted-change scope, the reflog / checkpoint recovery paths, and the export-safe evidence before removal, so an orphaned worktree or stale stack member stays blocked from removal for background agents and broad automation unless the user explicitly previews and confirms the cleanup, and nothing is deleted from ambient branch state — rather than a green summary across Git, patch-stack / queue, review, provider-landing, help / docs, and support / export surfaces`
- Consumer surfaces: 6
- Scope selectors: changed_files_scope, pull_request_scope, base_head_range_scope, worktree_uncommitted_scope, full_tree_scope, saved_pack_snapshot_scope, pack_scope_unclassified
- Resolution forms: canonical_object, accessible_summary, audit_record
- Proof freshness SLO: 720 hours (last audit: 2026-07-15T00:00:00Z)

## Consumer surfaces

- **change_object_detail**: `stable`
  - Owner: Change-object-detail owner
  - Scope: The change-object detail resolves the worktree-manager row for a selected change from the shared registry — the real worktree path, the checked-out ref, the divergence from base, the dirty / uncommitted state, any running task or open editor, and the recovery / checkpoint lineage — and keeps the cleanup preview of that worktree bound separately; a worktree-manager row missing its path or checked-out ref and a cleanup preview that would remove a worktree without naming its running tasks, open editors, uncommitted changes, or recovery checkpoints degrade honestly instead of deleting from ambient branch state
  - Review-pack-record entries: 2 / review-pack-result entries: 2
- **patch_stack_queue**: `stable`
  - Owner: Patch-stack-queue owner
  - Scope: The patch-stack / queue resolves the worktree-manager row for each stack member's working context — real path, checked-out ref, divergence, dirty state, and running-task / open-editor presence — so which worktree is active, which is orphaned or abandoned, and which is cleanup-ready are visible before any removal rather than after; a cleanup that would delete from ambient branch state and a stale stack member shown as safe-to-remove are caught before a green summary can hide them, so no member is silently reordered, collapsed, or retargeted and nothing is deleted without previewing running work
  - Review-pack-record entries: 2 / review-pack-result entries: 2
- **support_export_packet**: `stable`
  - Owner: Support owner
  - Scope: Support resolves each worktree-manager row's path, checked-out ref, divergence, and dirty state while keeping the cleanup preview bound separately to the export; a worktree-manager row that is a hand-copied per-entry assumption and a cleanup preview left unclassified degrade honestly so the affected work, its recovery paths, and its export-safe evidence are never dropped on export or reopen — a user can export the recovery bundle or defer the cleanup instead of losing the reflog / checkpoint recovery when a running task or open editor blocks the removal
  - Review-pack-record entries: 2 / review-pack-result entries: 1
- **stack_edit_review_sheet**: `stable`
  - Owner: Stack-edit-review-sheet owner
  - Scope: The stack-edit / review sheet renders the same resolved worktree-manager-row and cleanup-preview truth bound to the registry — real path, checked-out ref, divergence, dirty state, and running-task / open-editor presence — so a worktree's path, state, and recovery lineage can no longer be flattened into one generic badge; a worktree-manager row with an unstated path or checked-out ref is caught before Aureline removes anything but an explicitly previewed, cleanup-ready worktree
  - Review-pack-record entries: 2 / review-pack-result entries: 1
- **provider_merge_queue**: `stable`
  - Owner: Provider-merge-queue owner
  - Scope: The provider merge queue renders the same resolved worktree-manager-row and cleanup-preview truth the resolvers produced across the canonical, accessible, and audit resolution forms rather than a hand-copied sheet; the cleanup preview and the running-task / open-editor posture stay inspectable off-renderer so an orphaned worktree or stale stack member stays blocked from removal for background agents and broad automation unless the user explicitly previews and confirms the cleanup, and a stale stack member never reads as safe-to-delete
  - Review-pack-record entries: 1 / review-pack-result entries: 1
- **help_docs**: `stable`
  - Owner: Help-docs owner
  - Scope: The help / docs feed carries the same resolved worktree-manager-row and cleanup-preview truth, so a dropped worktree path, an unstated checked-out ref, a running task shown as absent, or a worktree removed by background automation is visible in evidence — a running-task warning, an uncommitted-change scope, or a reflog / checkpoint recovery path — rather than hidden behind a green summary
  - Review-pack-record entries: 1 / review-pack-result entries: 1
