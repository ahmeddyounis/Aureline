<!--
SPDX-FileCopyrightText: 2026 Aureline contributors
SPDX-License-Identifier: Apache-2.0
-->

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
- Apply revalidates repository/worktree identity, branch and HEAD lineage,
  selected-path status, index bytes, and worktree bytes against the in-memory
  evidence reviewed by the user. Any drift blocks without invoking a mutating
  Git command and requires a fresh preview.
- Stage, unstage, and discard consume the exact reviewed binary patch rather
  than rereading path contents during mutation, so a last-moment worktree race
  cannot silently stage or discard unreviewed bytes.
- Raw patch/checkpoint evidence is deliberately not serialized. An exported or
  deserialized preview is an inspection record, not portable apply authority.
- Git subprocesses use a minimal environment with system/global configuration,
  prompts, file-system monitors, submodule recursion, and external diff helpers
  disabled. Failure records retain only export-safe classes and exit status.
- Untracked files admitted for staging receive a binary-safe no-index diff so a
  preview cannot silently stage bytes that were absent from review.
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
