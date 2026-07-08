# M5 Git-history sequence component fixtures

Protected fixtures for the frozen Git-history and risky-mutation component
matrix (task M05-956). Each fixture is a complete, valid
`m5-git-history-sequence-component-matrix` packet that exercises a narrowed
scenario. All fixtures validate clean against both the typed `validate` and
`schemas/ui/m5-git-history-sequence-component-matrix.schema.json`.

- `force_push_approval_invalidated.json` — the force-push review dialog narrowed
  to Preview with an invalidated approval in its downgrade vocabulary; the dialog
  still names both refs, the ref-update rollback, and the invalidated approvals
  before confirm.
- `stash_entry_reflog_only_recovery.json` — a stash entry whose recovery
  destination has fallen back to a reflog-only path; the entry keeps that
  fallback explicit before pop.

Regenerate with the canonical export and summary via:

    GEN_GIT_HISTORY_COMPONENT_ARTIFACTS=1 cargo test -p aureline-git --lib \
      freeze_the_m5_git_history_sequence_component_matrix::tests::generate_artifacts

Do not hand-edit; the generator is the source of truth.
