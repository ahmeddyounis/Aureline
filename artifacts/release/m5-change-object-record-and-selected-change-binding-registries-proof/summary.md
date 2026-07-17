# M5 Change-Object-Record and Selected-Change-Binding Registries

- Packet: `m5-change-object-record-and-selected-change-binding-registries:stable:0001`
- Label: `M5 change-object-record and selected-change-binding registries emitting one durable machine-readable change-object record per non-trivial multi-file change — one typed field per record section: the stable change identity, whether it is a bounded working-set patch or a side-branch work unit, the selected worktree ID and base commit or dirty-tree fingerprint, the intent class and affected path set, and the validation plan and checkpoint lineage — each bound to one selected worktree / base identity, so a change object never drops its selected worktree / base identity and ambient branch state never reads as a reviewed, landing-ready change, with canonical / accessible / audit resolution-form coverage, and a machine-readable selected-change binding (surfaced before a broad refactor, migration / import, scaffold / update flow, or provider-backed mutation) that names the selected change object and explicit worktree identity a broad-scope flow must pass — so no broad mutation runs against ambient branch state and stack membership is never inferred from branch names alone — rather than a green summary across Git, patch-stack / queue, review, provider-landing, help / docs, and support / export surfaces`
- Consumer surfaces: 6
- Scope selectors: changed_files_scope, pull_request_scope, base_head_range_scope, worktree_uncommitted_scope, full_tree_scope, saved_pack_snapshot_scope, pack_scope_unclassified
- Resolution forms: canonical_object, accessible_summary, audit_record
- Proof freshness SLO: 720 hours (last audit: 2026-07-15T00:00:00Z)

## Consumer surfaces

- **change_object_detail**: `stable`
  - Owner: Change-object-detail owner
  - Scope: The change-object detail resolves a non-trivial multi-file change to one typed change-object record — its stable identity, whether it is a bounded working-set patch or a side-branch work unit, the selected worktree ID and base commit or dirty-tree fingerprint, the intent class and affected path set, and the validation plan and checkpoint lineage — from the shared registry and proves the selected-change binding gating any broad-scope flow for that change; a record missing its selected worktree / base identity and a selected-change binding that would let a broad flow begin from ambient branch state degrade honestly instead of leaving ambient branch state to read as a reviewed, landing-ready change
  - Review-pack-record entries: 2 / review-pack-result entries: 2
- **patch_stack_queue**: `stable`
  - Owner: Patch-stack-queue owner
  - Scope: The patch-stack / queue resolves the selected-change binding a broad-scope flow must pass and separately names each stack-membership source — declared in the change object, declared locally, inferred from a branch name, or stale or broken — before any broad mutation runs; a change binding an unbound broad flow into a clean start and a stack membership inferred from a branch name alone are caught before a green summary can hide them, so no broad refactor, migration / import, scaffold / update flow, or provider-backed mutation runs against ambient branch state and no stack member is silently reordered
  - Review-pack-record entries: 2 / review-pack-result entries: 2
- **support_export_packet**: `stable`
  - Owner: Support owner
  - Scope: Support resolves the change-object record's selected worktree / base identity while keeping the intent class, affected path set, validation plan, and checkpoint lineage bound to the export, and reports the selected-change binding state; a record that is a hand-copied per-entry assumption and a selected-change binding on an unclassified broad-flow binding degrade honestly so the change identity and checkpoint lineage are never dropped on export or reopen
  - Review-pack-record entries: 2 / review-pack-result entries: 1
- **stack_edit_review_sheet**: `stable`
  - Owner: Stack-edit-review-sheet owner
  - Scope: The stack-edit / review sheet resolves the linked worktree / base identity and the stack-membership-source state — declared in the change object, declared locally, inferred from a branch name, or stale or broken — bound to the registry so the four membership sources can no longer be flattened into one generic badge; an unstated selected worktree / base identity on a record is caught before it can drift
  - Review-pack-record entries: 2 / review-pack-result entries: 1
- **provider_merge_queue**: `stable`
  - Owner: Provider-merge-queue owner
  - Scope: The provider merge queue renders the same resolved change-object record and selected-change-binding truth the resolvers produced across the canonical, accessible, and audit resolution forms rather than a hand-copied sheet; the queue-eligible / queue-blocked / protected-branch-blocked landing state and the validation freshness stay inspectable off-renderer so ambient branch state never reads as a reviewed landing candidate
  - Review-pack-record entries: 1 / review-pack-result entries: 1
- **help_docs**: `stable`
  - Owner: Help-docs owner
  - Scope: The help / docs feed carries the same resolved change-object record and selected-change-binding truth, so a dropped selected worktree / base identity, an unstated change-object kind, ambient branch state masquerading as a selected change, or a stale-validation record shown as landing-ready is visible in evidence — a landing-state change, a stack-membership-source change, or a cleanup-safety change — rather than hidden behind a green summary
  - Review-pack-record entries: 1 / review-pack-result entries: 1
