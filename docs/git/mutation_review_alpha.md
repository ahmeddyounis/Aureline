# Git Mutation Review Alpha

The Git mutation review alpha makes source-control mutations preview-first
instead of opaque row actions.

## Contract

- `aureline-git` owns `git_mutation_preview`, `git_mutation_result`,
  `git_mutation_activity_record`, `git_mutation_support_export_record`, and
  `git_mutation_journal_record`.
- Stage, unstage, and discard requests create a preview packet before any Git
  command mutates the index or worktree.
- Preview packets preserve the selected scope, path-truth refs, source status
  snapshot ref, diff-preview ref, and checkpoint posture. Apply may not
  recompute or widen the scope silently.
- Stage and unstage capture an index-state checkpoint. Discard captures a
  worktree patch checkpoint and is blocked for untracked files until the delete
  path has a byte checkpoint.
- Checkpoint restore is represented as the revert flow for this alpha lane. It
  restores the captured index or worktree state and emits its own result,
  activity, support-export, and journal records.
- Support exports omit raw patch bodies and raw command lines while retaining
  operation kind, phase, scope ref, checkpoint ref, mutation id, and evidence
  refs.

## Records

- `git_mutation_preview`: operation, scope, diff-preview metadata, checkpoint
  posture, launch source, activity row, and support-export row.
- `git_mutation_result`: outcome state, applied/blocked targets, checkpoint,
  mutation journal, activity row, support-export row, and restore command id.
- `git_mutation_activity_record`: durable activity-center row for preview,
  apply, block, failure, or restore.
- `git_mutation_support_export_record`: redaction-safe support/export view of
  the same operation lineage.
- `git_mutation_journal_record`: mutation id, actor/source class, scope,
  target refs, reversal class, checkpoint refs, and side-effect summary.

## Inspection

Preview a selected path:

```sh
cargo run -p aureline-git --bin aureline_git_mutation -- --kind stage --path src/lib.rs --root .
```

Apply after preview inspection:

```sh
cargo run -p aureline-git --bin aureline_git_mutation -- --kind stage --path src/lib.rs --root . --apply
```

Run a forward apply plus checkpoint restore drill in one process:

```sh
cargo run -p aureline-git --bin aureline_git_mutation -- --kind discard --path src/lib.rs --root . --revert-after-apply
```

Protected fixture cases live under `fixtures/git/mutation_review_alpha/` and
are covered by:

```sh
cargo test -p aureline-git --test mutation_review_alpha
```
