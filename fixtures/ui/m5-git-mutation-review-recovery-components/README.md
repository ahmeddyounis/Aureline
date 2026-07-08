# M5 Git mutation-review / recovery component fixtures

Protected fixtures for the implemented Git risky-mutation review components (task
M05-960): cherry-pick/revert review sheets, patch-apply review sheets,
conflict-checkpoint cards, and force-push review dialogs. Each fixture is a
complete, valid `git_mutation_review_recovery_component_truth` packet exercising a
narrowed scenario. All fixtures validate clean against both the typed `validate`
and `schemas/ui/m5-git-mutation-review-recovery-component.schema.json`.

- `cherry_pick_conflict_checkpoint.json` — a cherry-pick that is expected to
  conflict; the blocker is disclosed on the sheet and the captured conflict is
  held at a reopenable checkpoint that preserves base/ours/theirs context, so
  recovery stays reachable after the risky mutation.
- `force_push_with_lease_recovery.json` — a lease-guarded force push that
  overwrites two remote-only commits and invalidates the hosted approval, yet
  names the exact tips, discloses the approval consequence, and stays recoverable
  from the remote-tracking reflog ref.

Regenerate the canonical export, summary, and these fixtures via:

    GEN_GIT_MUTATION_REVIEW_ARTIFACTS=1 cargo test -p aureline-git --lib \
      implement_mutation_review_sheets_conflict_checkpoints_and_force_push_dialogs::tests::generate_artifacts

Do not hand-edit; the generator is the source of truth.
