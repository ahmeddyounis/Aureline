# M5 Git-history identity component fixtures

Protected fixtures for the implemented Git-history identity/display components
(task M05-957): commit-graph headers, history-graph rows, branch-comparison chips,
and worktree rows. Each fixture is a complete, valid
`git_history_identity_component_working_context_truth` packet exercising a narrowed
scenario. All fixtures validate clean against both the typed `validate` and
`schemas/ui/m5-git-history-identity-component.schema.json`.

- `linked_worktree_separate_context.json` — the worktree row keeps its own separate
  working context: a linked worktree with uncommitted changes that never claims the
  current context and never flattens into one branch list.
- `shallow_partial_incomplete_history.json` — the history-graph row narrows to a
  partial checkout; the incomplete-history marker stays explicit so lazily-fetched
  objects are never mistaken for complete history.

Regenerate with the canonical export and summary via:

    GEN_GIT_HISTORY_IDENTITY_ARTIFACTS=1 cargo test -p aureline-git --lib \
      implement_commit_graph_headers generate_artifacts

Do not hand-edit; the generator is the source of truth.
