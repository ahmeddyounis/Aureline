# M5 Git-History Surface Certification

Closing certification capstone (workstream **B113**) over the twelve shared
Git-history and risky-mutation components frozen by the
`freeze_the_m5_git_history_sequence_component_matrix` lane, implemented by the
identity / stash-recovery / sequence-edit / mutation-review lanes, adopted by the
shared consumers in
`add_shared_history_sidebar_review_workspace_command_help_support_and_export_consumers_...`,
and proven across assistive, headless, and exported forms by the
`implement_keyboard_screen_reader_cli_export_parity_and_automatic_narrowing_...`
accessibility lane.

Where the implement lanes ship the components and the consumer / accessibility lanes
prove ref / worktree / recovery / verb parity, this lane certifies the **release
claim**: on every claimed M5 Git-history surface the same controlled component truth
is presented with no hidden ref / worktree / recovery drift.

## Certified surfaces (8)

`history_sidebar`, `risky_mutation_sheet`, `review_workspace`, `help_git_surface`,
`support_export`, `exported_recovery_packet`, `cli_headless`, `diagnostics`.

## Certified components (12)

`commit_graph_header`, `history_graph_row`, `branch_comparison_chip`,
`worktree_row`, `stash_entry`, `reflog_recovery_banner`, `rebase_todo_row`,
`sequence_editor_header`, `cherry_pick_revert_review_sheet`,
`patch_apply_review_sheet`, `conflict_checkpoint_card`, `force_push_review_dialog`.

## Certification axes (6)

The four always-on parity axes — `visual`, `keyboard`, `screen_reader`,
`cli_export` — plus `degraded_state` (narrows a claim when repo topology, checkpoint
availability, or provider-linked recovery weakens) and `local_recovery_provenance`
(the separation axis: a certified surface never implies its provider-linked review
state is fresh or its recovery checkpoint is reachable).

## Claim tiers (5, strongest first)

`recoverable_in_product` > `locally_recoverable` > `partial_history_only` >
`reflog_only_recovery` > `local_continue_only` — reused from the accessibility lane's
`GitHistoryClaimTier`.

## Status derivation

- **Red** `parity_blocked` — component truth was flattened (exact ref / worktree
  identity, dirty / shallow / partial topology, stash contents, sequence-edit intent,
  patch / apply target, approval invalidation, or recovery destination dropped).
  This is the **delta** of the capstone: certification may narrow a claim but may
  never drop the component's meaning.
- **Yellow** `narrowed_parity` — the certified claim dropped below the claimed claim,
  or an axis narrowed, with an honest fallback and disclosed downgrade trigger.
- **Green** `certified_parity` — certified claim equals claimed claim, no axis
  narrows, component truth preserved.

## Acceptance criteria

- **AC1** — every claimed surface presents the same controlled component truth in
  keyboard, screen-reader, CLI, and export form (parity fields present, six axes
  scored).
- **AC2** — release evidence demonstrates component parity rather than relying only on
  earlier workflow-level history-surgery rows: a certified claim may never exceed the
  claim it certifies (`certified_claim_exceeds_claimed`), and the auto-narrow
  automation narrows stale provider-linked recovery to `locally_recoverable` while
  keeping the `local_recovery_provenance` axis explicit.

## Artifacts

- Schema: `schemas/ui/m5-git-history-surface-certification.schema.json`
- Release proof:
  `artifacts/release/m5-git-history-surface-certification-proof/{support_export.json,matrix.csv,report.md}`
- Fixtures: `fixtures/ui/m5-git-history-surface-certification/`

Regenerate the checked-in proof and fixtures with
`GEN_GIT_HISTORY_CERTIFICATION_ARTIFACTS=1 cargo test -p aureline-git --lib regenerate_git_history_certification_artifacts`.
