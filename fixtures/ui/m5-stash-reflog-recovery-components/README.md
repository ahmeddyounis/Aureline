# M5 stash / reflog-recovery component fixtures

Protected fixtures for the implemented Git recovery components (task M05-958):
stash entries and reflog-recovery banners. Each fixture is a complete, valid
`git_stash_reflog_recovery_component_truth` packet exercising a narrowed scenario.
All fixtures validate clean against both the typed `validate` and
`schemas/ui/m5-stash-reflog-recovery-component.schema.json`.

- `untracked_stash_scope.json` — a stash entry that swept in untracked files keeps
  its untracked/staged scope explicit, so a restore never surprises the user; the
  `stash@{n}` shorthand is never the only meaning-bearing label.
- `expiring_recovery_banner.json` — the interactive-rebase recovery banner narrows
  to an expiring-soon reflog point but stays reachable across Git history, review,
  and help/support surfaces with a concrete recovery destination.

Regenerate with the canonical export and summary via:

    GEN_STASH_REFLOG_RECOVERY_ARTIFACTS=1 cargo test -p aureline-git --lib \
      implement_stash_entries_and_reflog_recovery_banners::tests::generate_artifacts

Do not hand-edit; the generator is the source of truth.
