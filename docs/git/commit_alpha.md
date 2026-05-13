# Git Commit Alpha

The commit alpha gives the local daily Git loop a reviewable in-product commit
path. A commit preview resolves the author identity, names exactly what is in
the Git index, blocks ambiguous amend or squash requests, and records whether
the resulting commit is only local or ready for a later publish review.

## Contract

- `aureline-git` owns `git_commit_preview`, `git_commit_result`,
  `git_commit_activity_record`, `git_commit_support_export_record`,
  `git_commit_journal_record`, and `git_commit_publish_readiness_record`.
- Normal commits are created from the current Git index. Unstaged, untracked,
  and conflicted paths remain visible as outside the commit scope.
- Author identity is resolved before apply from explicit input or local Git
  config. Missing or invalid author state blocks the commit before Git mutates
  history.
- Commit apply compares the current index tree with the previewed tree. If the
  staged scope drifted after review, apply is blocked and the user must reopen
  commit review.
- Amend and squash modes require explicit guardrail acknowledgement. Amend
  discloses the preflight `HEAD` and reflog recovery route. Squash creates an
  autosquash marker commit and keeps publish-later state blocked until sequence
  review.
- Commit results emit mutation-journal shaped lineage, activity rows, support
  export rows, and publish-later readiness without publishing to any remote.

## Records

- `git_commit_preview`: author, message validation, staged scope, history
  guardrail, publish-later preview, activity row, and support-export row.
- `git_commit_result`: outcome state, local commit oid, committed targets,
  journal record, activity row, support-export row, and publish-later result.
- `git_commit_publish_readiness_record`: local-only commit posture, upstream
  state, queue state, provider overlay state, blockers, and next review command.
- `git_commit_journal_record`: actor, author, source class, commit mode, scope,
  target refs, recovery class, local commit oid, and side-effect summary.

## Inspection

Preview a normal commit:

```sh
cargo run -p aureline-git --bin aureline_git_commit -- --message "commit staged work" --root .
```

Apply after preview inspection:

```sh
cargo run -p aureline-git --bin aureline_git_commit -- --message "commit staged work" --root . --apply
```

Preview an amend with explicit guardrail acknowledgement:

```sh
cargo run -p aureline-git --bin aureline_git_commit -- --mode amend --ack-history-guardrail --message "update current commit" --root .
```

Create an autosquash marker with explicit target and acknowledgement:

```sh
cargo run -p aureline-git --bin aureline_git_commit -- --mode squash --squash-target HEAD~1 --ack-history-guardrail --message "follow-up details" --root . --apply
```

Protected fixture cases live under `fixtures/git/commit_alpha/` and are covered
by:

```sh
cargo test -p aureline-git --test commit_alpha
```

