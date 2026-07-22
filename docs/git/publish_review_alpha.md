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
  from the local source. Failure to calculate divergence also blocks instead of
  being interpreted as zero behind. Force publish is limited to `--force-with-lease` and
  requires explicit guardrail acknowledgement plus the expected remote object id.
- Apply compares the current local source commit with the previewed source. If
  the local source changed after review, no push runs and the user must reopen
  publish review.
- Apply also revalidates repository/worktree identity, source `HEAD`, the
  last-known remote-tracking object, configured remote identity, and the exact
  single push URL plus repository-config evidence. Route/config drift or
  multiple push destinations block before any network mutation.
- Direct publication admits only absolute/explicit relative local paths,
  `file://`, `https://`, `ssh://`, `git+ssh://`, `ssh+git://`, and Git scp-style
  `[user@]host:path` URLs (including host-without-user form). `git://`, plain
  `http://`, arbitrary `<helper>::` transports, URL rewrites, protocol
  overrides, proxy routing, and custom receive-pack configuration are blocked.
- Push invokes the reviewed URL directly, never the mutable remote name. Its
  refspec uses the reviewed immutable local object id, not a mutable local ref,
  and fixes `--receive-pack=git-receive-pack`. A second resolution check blocks
  chained URL rewrites, and the publish subprocess admits only the reviewed
  transport family (`file`, HTTPS, or SSH); every other protocol remains denied
  even if URL parsing is bypassed.
- Git subprocesses clear ambient environment and retain only `PATH`, explicit
  locale/hardening variables, required Windows runtime variables, and
  `SSH_AUTH_SOCK` only for a preview-admitted SSH transport. Branch, commit,
  status, HTTPS, file, and local-path Git commands receive no agent socket.
  SSH-agent auth therefore remains available for
  noninteractive pushes, while credential helpers, askpass, prompts, and
  caller-supplied Git/SSH command overrides remain disabled. SSH transport uses
  a constant batch-mode command with SSH config, keyfile, password,
  keyboard-interactive, GSSAPI, and host-based authentication disabled, leaving
  the reviewed agent socket as its credential source. It requires an existing
  trusted host key and neither prompts for nor rewrites host-key trust. The
  route review labels SSH-agent, unavailable-agent, remote-URL userinfo,
  unsupported-transport, local-filesystem, and no-helper auth postures explicitly; agent-source drift
  blocks apply.
- Raw route operands and apply authority remain process-local and one-shot.
  Exported, deserialized, cross-service, replayed, or publicly tampered
  previews are inspection records and cannot publish.
- Authority expires after ten minutes and is capped by entry count, per-entry
  bytes, and total retained bytes with deterministic oldest eviction. Git
  subprocesses also have closed stdin, bounded output, and a kill-on-timeout
  supervisor.
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
covered by the command below. They distinguish stale-route blocking (no push
attempt) from a real receive-side failure after the reviewed push starts; both
preserve the local commit, while only the latter reports `failed`.

```sh
cargo test -p aureline-git --test publish_review_alpha
```
