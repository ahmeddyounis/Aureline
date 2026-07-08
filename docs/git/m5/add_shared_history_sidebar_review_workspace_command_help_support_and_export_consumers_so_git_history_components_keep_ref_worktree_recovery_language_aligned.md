# Shared Git-History Component Consumers: Ref, Worktree, and Recovery Parity

Closing consumer-adoption lane (M05-961, batch B113) for the twelve reusable
Git-history and risky-mutation components frozen in
`freeze_the_m5_git_history_sequence_component_matrix` and implemented by the
commit-graph / history-graph / branch-comparison / worktree identity lane, the
stash-entry / reflog-recovery lane, the rebase-todo / sequence-editor lane, and the
cherry-pick / revert / patch-apply / conflict-checkpoint / force-push mutation-review
lane. It binds each shared component to the consumer surfaces that render it and
proves — by fixtures, not screenshots — that the same Git-history object presents the
same exact target ref, worktree scope, recovery destination, and primary verb
wherever it appears.

## Consumers

| Consumer | Surface |
| --- | --- |
| `history_sidebar` | Desktop history sidebar (commit graph, branches, worktrees, stash shelf) |
| `risky_mutation_sheet` | Risky-mutation review sheet (rebase, cherry-pick, revert, patch, force-push) |
| `review_workspace_banner` | Review-workspace banner layered over an in-flight review |
| `command_help` | Command-help / About surface |
| `support_bundle` | Support bundle |
| `exported_recovery_packet` | Exported recovery packet / evidence |

`command_help`, `support_bundle`, and `exported_recovery_packet` are Help/support/
export surfaces and must point at both the frozen component matrix and the canonical
component schema by id.

## Parity facets

For a given Git-history object, every consumer surface must present identical values
for four facets. A surface may narrow how much it shows, but it may never reword
these per surface:

- `ref_identity_label` — the exact target ref / commit identity.
- `worktree_scope_label` — the worktree / root scope.
- `recovery_destination_label` — the recovery checkpoint / destination.
- `primary_verb` — the primary Git verb (never collapsed).

## Render conditions and modes

Each binding carries a render `condition`. `aligned_local_truth` renders at full
parity; every other condition binds back to a frozen `GitHistoryDowngradeState` and
narrows the rendering through an explicit banner without rewording the parity facets.

| Condition | Render mode | Disclosure required |
| --- | --- | --- |
| `aligned_local_truth` | `full_parity` | none |
| `stale_provider_overlay` | `identity_narrowed` | narrow banner |
| `detached_or_missing_ref` | `identity_narrowed` | narrow banner + `ref_identity_note` |
| `dirty_or_conflicted_worktree` | `identity_narrowed` | narrow banner |
| `shallow_or_partial_topology` | `identity_narrowed` | narrow banner |
| `reflog_only_fallback` | `recovery_narrowed` | narrow banner + `recovery_note` |
| `approval_invalidated` | `recovery_narrowed` | narrow banner + `recovery_note` |
| `offline_local_only` | `local_continue_fallback` | narrow banner + `local_continue_note` |

## Guardrails (each must be false on every binding)

- `collapses_git_verb_into_ambiguous_confirm`
- `hides_exact_target_ref_or_worktree`
- `drops_conflict_or_recovery_state_after_mutation`
- `rewords_ref_worktree_recovery_labels_per_surface`
- `hides_local_only_recovery_when_provider_linked`

## Proof

- Parity is enforced by grouping bindings by `history_object_id` and requiring
  identical parity facet values (`parity_drift_across_surfaces`).
- Reuse is proven by requiring every one of the twelve components to be adopted by at
  least two distinct consumers (`git_history_component_reuse_unproven`), covering all
  six consumers and all twelve components.

## Artifacts

- Schema: `schemas/ui/m5-git-history-component-consumer.schema.json`
- Support export: `artifacts/release/m5-git-history-component-consumers-proof/support_export.json`
- Summary: `artifacts/release/m5-git-history-component-consumers-proof/summary.md`
- Fixtures: `fixtures/ui/m5-git-history-component-consumers/`

Regenerate the checked-in artifacts and fixtures with
`GEN_GIT_HISTORY_COMPONENT_CONSUMER_ARTIFACTS=1 cargo test -p aureline-git --lib regenerate_git_history_component_consumer_artifacts`,
then review the diff.
