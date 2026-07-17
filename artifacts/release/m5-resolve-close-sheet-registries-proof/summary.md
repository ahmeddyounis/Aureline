# M5 Resolve-or-Close Sheet and Resolution-Outcome Registries

- Packet: `m5-resolve-close-sheet-registries:stable:0001`
- Label: `M5 resolve-or-close-sheet and resolution-outcome registries emitting one reusable machine-readable resolve-or-close sheet per tracked work item — one typed field per sheet section: the current state, the requested terminal state, the unresolved blockers, the linked evidence, the permission scope, the confirm / reopen / export actions, and the final side-effect preview — each bound to one commit state with its lineage, so a resolve-or-close never drops its unresolved blockers / linked evidence / reopen-and-export continuity and no local-only resolution reads as a provider-accepted terminal state, with canonical / accessible / audit resolution-form coverage, and a machine-readable resolution-outcome object (resolved locally, provider updated, queued for publish, blocked by missing permission, or blocked by unresolved engineering state) that keeps each terminal outcome a visible, typed action — so a resolve-or-close never implies the provider accepted the terminal state when the target is offline, policy-blocked, or only partially writable — across work-item detail, review detail, Git / worktree, provider-handoff, help / docs, and support / export surfaces`
- Consumer surfaces: 6
- Scope selectors: changed_files_scope, pull_request_scope, base_head_range_scope, worktree_uncommitted_scope, full_tree_scope, saved_pack_snapshot_scope, pack_scope_unclassified
- Resolution forms: canonical_object, accessible_summary, audit_record
- Proof freshness SLO: 720 hours (last audit: 2026-07-15T00:00:00Z)

## Consumer surfaces

- **work_item_detail**: `stable`
  - Owner: Work-item-detail owner
  - Scope: Work-item detail resolves a tracked work item to one typed resolve-or-close sheet — its current state, requested terminal state, unresolved blockers, linked evidence, permission scope, and confirm / reopen / export actions, with the final side-effect preview — from the shared registry and proves the final-resolution authority for that item; a sheet dropping its unresolved blockers or linked evidence and a resolution-outcome that would let a local-only resolution read as a provider-accepted terminal state degrade honestly instead of implying the provider accepted a terminal state it has not
  - Review-pack-record entries: 2 / review-pack-result entries: 2
- **review_detail**: `stable`
  - Owner: Review-detail owner
  - Scope: Review detail resolves the same resolve-or-close sheet from the tracked item and shows the unresolved blockers, the linked evidence, and the permission scope bound to their commit state; a sheet letting a queued or local-only resolution read as provider-accepted and a dropped reopen / export path are caught before a green summary can hide them, so review detail renders the same resolution truth as work-item detail without contradiction
  - Review-pack-record entries: 2 / review-pack-result entries: 2
- **support_export_packet**: `stable`
  - Owner: Support owner
  - Scope: Support resolves the sheet's commit state while keeping the unresolved blockers / linked evidence / permission scope and the resolved-locally / provider-updated / queued-for-publish attribution bound to the export, and reports the final-resolution authority; a sheet that is a hand-copied per-item assumption and a resolution-outcome on an unclassified binding degrade honestly so the unresolved blockers, linked evidence, and reopen / export continuity are never dropped on export or reopen
  - Review-pack-record entries: 2 / review-pack-result entries: 1
- **linked_change_panel**: `stable`
  - Owner: Linked-change-panel owner
  - Scope: The linked-change panel surface renders the same sheet's linked evidence and permission scope bound to their commit state — resolved locally, provider updated, queued for publish, blocked by missing permission, or blocked by unresolved engineering state — from the registry so the resolution outcomes can no longer be flattened into one generic close, and a target that is offline, policy-blocked, or only partially writable stays visible and actionable instead of implying provider acceptance; an unstated commit state on a sheet is caught before it can drift
  - Review-pack-record entries: 2 / review-pack-result entries: 1
- **resolve_close_sheet**: `stable`
  - Owner: Resolve-or-close-sheet owner
  - Scope: The resolve-or-close sheet renders the same resolved sheet and resolution-outcome truth the resolvers produced across the canonical, accessible, and audit resolution forms rather than a hand-copied sheet, letting users compare resolved-locally, provider-updated, and queued-for-publish outcomes from one sheet while preserving reopen / export continuity; the resolution-outcome state and the unresolved-blocker state stay inspectable off-renderer so a local-only resolution never reads as a provider-accepted terminal state
  - Review-pack-record entries: 1 / review-pack-result entries: 1
- **help_docs**: `stable`
  - Owner: Help-docs owner
  - Scope: The help / docs feed carries the same resolved sheet and resolution-outcome truth, so a dropped evidence field, an unstated commit state, a local-only resolution masquerading as a provider-accepted terminal state, or an offline / policy-blocked / partially-writable target shown as accepted is visible in evidence — a resolution-mode change or a commit-state change — rather than hidden behind a green summary
  - Review-pack-record entries: 1 / review-pack-result entries: 1
