# M5 Change-Object-Record and Selected-Change-Binding Registries

- Packet: `m5-patch-stack-queue-and-member-landing-registries:stable:0001`
- Label: `M5 patch-stack / queue and member-landing registries emitting one durable machine-readable patch-stack record per stacked change — one typed field per stack section: the stack ID, the ordered member IDs, the parent/child relation, the landing order, the rebase epoch, the inherited blockers, and the current validation freshness — with each member card keeping local stack state, provider-linked review state, and queue/landing posture explicitly separate, so stack membership is visible metadata rather than a branch-naming convention and no member is silently reordered, collapsed, or retargeted, with canonical / accessible / audit resolution-form coverage, and a machine-readable member-landing posture that names which member blocks another, which checks are stale, and which members are ready, blocked, or not yet landing candidates — so ambient branch state never reads as a reviewed landing candidate and stack membership is never inferred from branch names alone — rather than a green summary across Git, patch-stack / queue, review, provider-landing, help / docs, and support / export surfaces`
- Consumer surfaces: 6
- Scope selectors: changed_files_scope, pull_request_scope, base_head_range_scope, worktree_uncommitted_scope, full_tree_scope, saved_pack_snapshot_scope, pack_scope_unclassified
- Resolution forms: canonical_object, accessible_summary, audit_record
- Proof freshness SLO: 720 hours (last audit: 2026-07-15T00:00:00Z)

## Consumer surfaces

- **change_object_detail**: `stable`
  - Owner: Change-object-detail owner
  - Scope: The change-object detail resolves the patch stack a change object belongs to — its stack ID, the ordered member IDs, the parent/child relation, the landing order, the rebase epoch, the inherited blockers, and the current validation freshness — from the shared registry, and keeps local stack state, provider-linked review state, and queue/landing posture explicitly separate; a stack row missing its stack ID or ordered member set and a member whose local stack state is shown as provider-accepted degrade honestly instead of letting a branch-naming convention read as reviewed stack membership
  - Review-pack-record entries: 2 / review-pack-result entries: 2
- **patch_stack_queue**: `stable`
  - Owner: Patch-stack-queue owner
  - Scope: The patch-stack / queue resolves every stack member card in order — stack ID, member ID, parent/child relation, landing order, rebase epoch, inherited blockers, and validation freshness — so which member blocks another, which checks are stale, and which members are ready, blocked, or not yet landing candidates are visible metadata rather than a branch-naming convention; a stack membership inferred from a branch name alone and a member silently reordered are caught before a green summary can hide them, so no member is silently reordered, collapsed, or retargeted and nothing lands from ambient branch state
  - Review-pack-record entries: 2 / review-pack-result entries: 2
- **support_export_packet**: `stable`
  - Owner: Support owner
  - Scope: Support resolves each stack member's stack ID, ordered position, parent/child relation, and validation freshness while keeping local stack state, provider-linked review state, and queue/landing posture bound separately to the export; a member card that is a hand-copied per-entry assumption and a member landing posture left unclassified degrade honestly so the stack identity, member order, and inherited blockers are never dropped on export or reopen
  - Review-pack-record entries: 2 / review-pack-result entries: 1
- **stack_edit_review_sheet**: `stable`
  - Owner: Stack-edit-review-sheet owner
  - Scope: The stack-edit / review sheet resolves the ordered member set and the parent/child relation bound to the registry so a member's landing order, rebase epoch, and inherited blockers can no longer be flattened into one generic badge; an unstated stack ID or ordered-member set on a stack row is caught before it can drift into an implicit branch-naming convention
  - Review-pack-record entries: 2 / review-pack-result entries: 1
- **provider_merge_queue**: `stable`
  - Owner: Provider-merge-queue owner
  - Scope: The provider merge queue renders the same resolved patch-stack and member-landing truth the resolvers produced across the canonical, accessible, and audit resolution forms rather than a hand-copied sheet; the queue-eligible / queue-blocked / protected-branch-blocked landing state and the validation freshness stay inspectable off-renderer so ambient branch state never reads as a reviewed landing candidate and provider-linked review state stays distinct from local stack state
  - Review-pack-record entries: 1 / review-pack-result entries: 1
- **help_docs**: `stable`
  - Owner: Help-docs owner
  - Scope: The help / docs feed carries the same resolved patch-stack and member-landing truth, so a dropped stack ID, an unstated ordered-member set, a stack membership inferred from a branch name, or a stale-validation member shown as landing-ready is visible in evidence — a landing-order change, a stack-membership-source change, or a validation-freshness change — rather than hidden behind a green summary
  - Review-pack-record entries: 1 / review-pack-result entries: 1
