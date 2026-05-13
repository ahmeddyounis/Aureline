# Git Publish Review Alpha

The publish review alpha gives the daily Git loop a narrow, inspectable push
path. A publish preview resolves one local ref, one remote ref, the acting
origin, and the route class before any network mutation runs. Failed publishes
retain a reopenable review packet and local recovery path.

## Contract

- `aureline-git` owns `git_publish_preview`, `git_publish_result`,
  `git_publish_activity_record`, `git_publish_support_export_record`,
  `git_publish_journal_record`, and `git_publish_failure_recovery_record`.
- Publish previews identify the local source ref, remote name, remote target ref,
  route class, client origin, redacted remote URL posture, and provider overlay
  state before execution.
- Normal push blocks when the last-known remote target contains commits missing
  from the local source. Force publish is limited to `--force-with-lease` and
  requires explicit guardrail acknowledgement plus the expected remote object id.
- Apply compares the current local source commit with the previewed source. If
  the local source changed after review, no push runs and the user must reopen
  publish review.
- Failed publish results keep the original preview ref, retry command, export
  packet ref, and local-state-preserved flag. The activity and support rows stay
  durable instead of collapsing into a lost modal.
- This lane is local Git publish only. `merge_queue_supported` is `false`, and
  provider overlay state is labeled `not_configured_alpha`.

## Records

- `git_publish_preview`: route/origin labels, source/target ref review,
  divergence, force guardrails, failure recovery, activity row, and
  support-export row.
- `git_publish_result`: outcome state, route and target copied from the preview,
  journal record, durable activity row, support-export row, and recovery record.
- `git_publish_failure_recovery_record`: original preview ref, reopen/retry
  commands, export packet ref, provider write state, and local preservation cue.
- `git_publish_journal_record`: actor, command id, source class, publish mode,
  route ref, target ref, external-effect summary, and recovery class.

## Inspection

Preview a publish using the current upstream:

```sh
cargo run -p aureline-git --bin aureline_git_publish -- --root .
```

Preview an explicit remote and target:

```sh
cargo run -p aureline-git --bin aureline_git_publish -- --root . --remote origin --target-branch main
```

Apply after preview inspection:

```sh
cargo run -p aureline-git --bin aureline_git_publish -- --root . --remote origin --target-branch main --apply
```

Preview a guarded force-with-lease publish:

```sh
cargo run -p aureline-git --bin aureline_git_publish -- --root . --mode force-with-lease --ack-force-review --expected-remote-oid <oid>
```

Protected fixture cases live under `fixtures/git/publish_review_alpha/` and are
covered by:

```sh
cargo test -p aureline-git --test publish_review_alpha
```
