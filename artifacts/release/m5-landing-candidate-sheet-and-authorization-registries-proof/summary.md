# M5 Landing-Candidate-Sheet and Authorization Registries

- Packet: `m5-landing-candidate-sheet-and-authorization-registries:stable:0001`
- Label: `M5 landing-candidate sheet and authorization registries emitting one durable machine-readable reviewed landing candidate per proposed land — one typed field per section: the exact target branch, the merge strategy (a local squash plan, a local rebase plan, a merge-queue enqueue, or a review-ready export bundle), the required checks, the approval state, and the queue eligibility — with the explicit advance authorization of the reviewed candidate kept separate, so Aureline only lands from an explicit reviewed landing candidate and never from ambient branch state and no member is silently reordered, collapsed, or retargeted, with canonical / accessible / audit resolution-form coverage, the provider-authoritative-versus-local-estimate distinction preserved through queue-position ambiguity, stale-base invalidation, and protected-branch blocking, and a machine-readable authorization that keeps a protected branch blocked for background agents and broad automation unless the user explicitly advances the reviewed candidate through the correct command path — so a local estimate never reads as a provider-authoritative landing and nothing lands from ambient branch state — rather than a green summary across Git, patch-stack / queue, review, provider-landing, help / docs, and support / export surfaces`
- Consumer surfaces: 6
- Scope selectors: changed_files_scope, pull_request_scope, base_head_range_scope, worktree_uncommitted_scope, full_tree_scope, saved_pack_snapshot_scope, pack_scope_unclassified
- Resolution forms: canonical_object, accessible_summary, audit_record
- Proof freshness SLO: 720 hours (last audit: 2026-07-15T00:00:00Z)

## Consumer surfaces

- **change_object_detail**: `stable`
  - Owner: Change-object-detail owner
  - Scope: The change-object detail resolves the landing candidate for a proposed land from the shared registry — the exact target branch, the merge strategy (a local squash plan, a local rebase plan, a merge-queue enqueue, or a review-ready export bundle), the required checks, the approval state, and the queue eligibility — and keeps the explicit advance authorization of that candidate bound separately; a landing candidate missing its target branch or merge strategy and an authorization that would let a local estimate read as a provider-authoritative landing degrade honestly instead of landing from ambient branch state
  - Review-pack-record entries: 2 / review-pack-result entries: 2
- **patch_stack_queue**: `stable`
  - Owner: Patch-stack-queue owner
  - Scope: The patch-stack / queue resolves the landing candidate for each of its members in order — target branch, merge strategy, required checks, approval state, and queue eligibility — so which member is queue-eligible, which is queue-blocked or protected-branch-blocked, and which carries a stale base are visible before advancing rather than after; a candidate that would land from ambient branch state and a queue-position estimate shown as provider-authoritative are caught before a green summary can hide them, so no member is silently reordered, collapsed, or retargeted and nothing lands from ambient branch state
  - Review-pack-record entries: 2 / review-pack-result entries: 2
- **support_export_packet**: `stable`
  - Owner: Support owner
  - Scope: Support resolves each landing candidate's target branch, merge strategy, required checks, and approval state while keeping the advance authorization of the reviewed candidate bound separately to the export; a landing candidate that is a hand-copied per-entry assumption and an authorization left unclassified degrade honestly so the reviewed candidate, its target and strategy, and its queue eligibility are never dropped on export or reopen — a user can export the review-ready bundle or defer the land instead of losing the reviewed candidate when a provider or policy boundary blocks the land
  - Review-pack-record entries: 2 / review-pack-result entries: 1
- **stack_edit_review_sheet**: `stable`
  - Owner: Stack-edit-review-sheet owner
  - Scope: The stack-edit / review sheet renders the same resolved landing-candidate and authorization truth bound to the registry — target branch, merge strategy, required checks, approval state, and queue eligibility — so a candidate's target, strategy, and queue eligibility can no longer be flattened into one generic badge; a landing candidate with an unstated target branch or merge strategy is caught before Aureline lands from anything but an explicit reviewed candidate
  - Review-pack-record entries: 2 / review-pack-result entries: 1
- **provider_merge_queue**: `stable`
  - Owner: Provider-merge-queue owner
  - Scope: The provider merge queue renders the same resolved landing-candidate and authorization truth the resolvers produced across the canonical, accessible, and audit resolution forms rather than a hand-copied sheet; the advance authorization and the protected-branch posture stay inspectable off-renderer so a protected branch stays blocked for background agents and broad automation unless the user explicitly advances the reviewed candidate through the correct command path, and a local estimate never reads as a provider-authoritative landing
  - Review-pack-record entries: 1 / review-pack-result entries: 1
- **help_docs**: `stable`
  - Owner: Help-docs owner
  - Scope: The help / docs feed carries the same resolved landing-candidate and authorization truth, so a dropped target branch, an unstated merge strategy, a stale base shown as still-current, or a protected branch advanced by background automation is visible in evidence — a target-branch disclosure, a queue-eligibility state, or a protected-branch block — rather than hidden behind a green summary
  - Review-pack-record entries: 1 / review-pack-result entries: 1
