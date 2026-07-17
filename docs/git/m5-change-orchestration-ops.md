# M5 change-orchestration component matrix — operations contract

This document is the human-readable companion to the frozen M5 change-orchestration matrix. The
authoritative gate is the Rust validator in
`crates/aureline-ui/src/m5_change_object_patch_stack_and_landing_matrix/`; the checked-in schemas,
fixtures, dashboard, and release proof bundle are canonical for this lane. This doc names what the matrix
freezes so later implementation does not re-interpret stack, landing, or cleanup semantics per surface.

- Matrix schema: `schemas/change/m5-change-orchestration-component-matrix.schema.json`
- Support export: `artifacts/release/m5-change-orchestration-proof/support_export.json`
- Matrix CSV: `artifacts/release/m5-change-orchestration-proof/matrix.csv`
- Design report: `artifacts/design/m5-change-orchestration-component-matrix.md`
- Health dashboard: `dashboards/m5-change-orchestration-health.json`
- Narrowed fixtures: `fixtures/git/m5-change-orchestration/`
- Emitter (single mint-from-truth path): `cargo run -p aureline-ui --example dump_m5_change_orchestration_matrix -- <subcommand>`

## Governed object classes

Every non-trivial multi-file change binds to one explicit change object; the six classes are:

1. **change_object** (`schemas/change/m5-change-object.schema.json`) — the explicit change object a change
   binds to: selected worktree / base identity, working-set-patch vs side-branch-work-unit kind, stack
   membership, landing state, validation freshness.
2. **patch_stack_queue** (`schemas/ui/m5-patch-stack-queue.schema.json`) — the ordered patch stack / merge or
   landing queue: member order, queue eligibility, queue-blocked reason, stack dependency edges.
3. **stack_edit_review_sheet** (`schemas/ui/m5-stack-edit-review-sheet.schema.json`) — the sheet that edits
   and reviews a stack while keeping the four membership sources distinct and flagging a restack-required stack.
4. **landing_candidate_sheet** (`schemas/ui/m5-landing-candidate-sheet.schema.json`) — the reviewed landing
   candidate: validation freshness, protected-branch gate, landing target, rollback / export fallback.
5. **portable_shelf** (`schemas/change/m5-portable-shelf.schema.json`) — the portable shelf / bundle:
   export bundle contents, import / reopen lineage, shelf state, recovery checkpoint.
6. **worktree_cleanup_preview** (`schemas/ui/m5-worktree-manager-row.schema.json`) — the worktree-manager row /
   orphan-cleanup preview: cleanup target, running-work / open-editor / uncommitted-change / checkpoint preview.

## Frozen state vocabulary (`landing_state`)

`selected_change`, `stale_validation`, `restack_required`, `queue_eligible`, `queue_blocked`,
`protected_branch_blocked`, `orphaned`, `abandoned`, `exported`, `imported_reopened`. Only `queue_eligible` is
mechanically queue-eligible; every other state must not read as a reviewed, queue-eligible landing candidate.

## Stack membership source (never flattened)

`declared_in_change_object`, `declared_locally`, `inferred_from_branch_name`, `stale_or_broken_membership`.
An inferred-from-branch-name membership is a suggestion and is always labelled as such — membership is never
inferred from branch names alone.

## Consumer lanes

`change_object_detail`, `patch_stack_queue_panel` (`patch_stack_queue`), `stack_edit_review_sheet`,
`landing_candidate_sheet`, `worktree_manager_row` (`worktree_cleanup_preview`), `review_detail`,
`provider_merge_queue`, `support_export_packet`, `help_docs`. Surface families:
`git_surface`, `stack_queue_surface`, `review`, `provider_landing`, `support_export`, `help_docs`.

## Downgrade triggers

`stack_membership_inferred_from_branch_name_alone`, `cross_worktree_write_without_selected_change_object`,
`stack_members_silently_reordered`, `landed_from_ambient_branch_state`, `orphan_deleted_without_safety_preview`,
`selected_change_object_unstated`, `worktree_binding_unstated`, `stack_membership_source_unstated`,
`stack_order_unstated`, `landing_state_unstated`, `validation_freshness_unstated`,
`change_orchestration_matrix_stale`. A claimed class narrows automatically when its matrix row is missing or
its proof has gone stale.

## Hard guardrails (each is a per-row invariant that MUST be `false`)

1. Do not infer stack membership from branch names alone.
2. Do not mutate files in another worktree without an explicit selected change object and worktree binding.
3. Do not silently reorder, collapse, or retarget stack members.
4. Do not land from ambient branch state without a reviewed landing candidate.
5. Do not delete orphaned worktrees or stale stack members without previewing running tasks, open editors,
   uncommitted changes, recovery checkpoints, and export-safe evidence.

## Acceptance criteria

- **AC1** — A reviewed component matrix names every object, visible state, consumer lane, and downgrade
  trigger needed for explicit change-orchestration truth (the six rows above, the `landing_state` vocabulary,
  the consumer lanes, and the downgrade triggers, all frozen in one packet).
- **AC2** — Follow-on work can implement the same matrix without re-interpreting stack, landing, or cleanup
  semantics per surface, because the vocabulary, per-domain schemas, and hard guardrails are frozen here and
  bound back to the already-landed stable-proof-index, migration-task-row, and portable-bundle packets.
