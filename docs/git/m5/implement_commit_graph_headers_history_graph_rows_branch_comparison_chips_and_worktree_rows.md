# M5 Git-history identity components: commit-graph headers, history-graph rows, branch-comparison chips, and worktree rows

This lane (M05-957) narrows the four **identity/display** components frozen in the
[M5 Git-history and risky-mutation component matrix](./freeze_the_m5_git_history_sequence_component_matrix.md)
— `commit_graph_header`, `history_graph_row`, `branch_comparison_chip`, and
`worktree_row` — into an implemented, export-safe row contract so every claimed M5
Git surface can render repo/worktree/ref identity and topology state without copying
per-screen chrome.

- Rust module: `crates/aureline-git/src/implement_commit_graph_headers_history_graph_rows_branch_comparison_chips_and_worktree_rows/`
- Boundary schema: [`schemas/ui/m5-git-history-identity-component.schema.json`](../../../schemas/ui/m5-git-history-identity-component.schema.json)
- Checked support export: [`artifacts/release/m5-git-history-identity-components-proof/support_export.json`](../../../artifacts/release/m5-git-history-identity-components-proof/support_export.json)
- Protected fixtures: [`fixtures/ui/m5-git-history-identity-components/`](../../../fixtures/ui/m5-git-history-identity-components/)

## Goal

Make repository and worktree identity obvious *before* users compare, switch, or
mutate history. Every component carries repo identity, checked-out ref, divergence,
dirty state, shallow/partial/sparse markers, worktree path, and the associated
recovery/reflog availability — and it preserves separate working-context semantics
for each worktree rather than flattening them into one branch list.

## Core honesty axis: the working-context target

`GitWorkingContextTarget` is the axis that makes acceptance criteria testable. A
component targets exactly one of:

| Target | Claims current context? | Separate context? | Incomplete history? |
| --- | --- | --- | --- |
| `current_repo_worktree` | yes | no | no |
| `linked_worktree` | no | yes (never flattened) | no |
| `partial_or_shallow_checkout` | no | no | yes (marked) |
| `detached_or_bare_root` | no | no | no (but recovery kept explicit) |

The current-context claim is **derived** from the target, never asserted directly:
`resolve_git_history_identity_disclosure` computes what a component must disclose from
`(target, divergence, dirty_state, topology_completeness)`. A linked worktree or a
partial/shallow checkout therefore can never silently pretend it is the current repo
(`AmbiguousContextClaimed`), a separate working context always keeps its own note
(`SeparateWorktreeContextMissing`), a shallow/partial/sparse checkout is always marked
(`IncompleteHistoryMarkerMissing`), and any dirty, conflicted, divergent, or detached
context keeps its recovery/reflog availability explicit
(`RecoveryReflogAvailabilityMissing`).

## Reuse

- `M5GitHistoryComponent` and the four-component allow-list gate the row's `component`
  (`NonIdentityComponent` rejects any risky-mutation component here).
- `GitHistoryDowngradeState` (the shared matrix downgrade vocabulary) is reused for
  both per-row `downgrade_vocab` and packet-level `downgrade_triggers`, so downgrades
  read the same everywhere.
- `ComponentConsumerSurface` (the shared matrix consumer surfaces) is reused for
  `consumer_surfaces`.

## Acceptance criteria mapping

- **Git lists/detail panes no longer make multiple worktrees or divergent roots look
  like one ambiguous context** — `worktree_identity_never_flattened` +
  `WorkingContextCoverageMissing` (current + linked + partial/shallow must all appear)
  + `AmbiguousContextClaimed`.
- **Users can tell from the component itself whether a history action targets the
  current repo, another worktree, or a partial/shallow checkout** — the derived
  `working_context_target`, `SeparateWorktreeContextMissing`, and
  `IncompleteHistoryMarkerMissing` invariants.

## Regenerating artifacts

The checked export, Markdown summary, and narrowed fixtures are produced by the
`generate_artifacts` test, gated behind an env var so it is inert in CI:

```
GEN_GIT_HISTORY_IDENTITY_ARTIFACTS=1 cargo test -p aureline-git --lib \
  implement_commit_graph_headers generate_artifacts
```

`checked_export_matches_seed` asserts the checked JSON equals the in-Rust seed packet,
so the artifact can never drift from the contract.
