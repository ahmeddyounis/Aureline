# Stabilize the daily Git loop — review artifact

## Scope

This artifact covers the bounded M4 daily-loop adapter: explicit repo/worktree targeting for status, diff, stage, commit, amend, blame, and history, plus inspect-only stash vocabulary. It is not a claim that every listed operation is mutation-qualified.

## Implementation location

- `crates/aureline-git/src/stabilize_the_daily_git_loop_status_diff_stage/mod.rs`
- `crates/aureline-git/src/bin/aureline_git_daily_loop.rs`
- `crates/aureline-git/tests/daily_loop_beta.rs`
- `fixtures/git/m4/daily_loop_beta/`

## Key contracts

### Explicit targeting
Every request carries [`RepoTarget`] and [`WorktreeTarget`] so that:
- Parent repos, submodules, nested independent repos, sparse slices, shallow histories, and pointer-backed assets never resolve ambiguously.
- Identical-looking paths in parent and child repos remain distinguishable.

### Operations covered
- `status` — canonical snapshot with path statuses.
- `diff` — bounded worktree-diff presence observation. A non-empty diff is `partial_omitted`; file/hunk/line rows are not synthesized while parsing is unqualified.
- `stage` / `unstage` — exact-patch, stale-evidence-checked path mutations through the canonical mutation service.
- `commit` / `amend` — single-process-authority commit previews with message and history guardrails through the canonical commit service.
- `stash_capture` / `stash_apply` / `stash_pop` / `stash_drop` / `stash_branch_from` — inspect-only stash/shelf vocabulary. Mutation previews are blocked until exact checkpoint and stale-evidence authority exists.
- `blame` — per-line blame with content-availability labels.
- `history` — commit history with content-availability labels.

### Stash/shelf entry objects
[`StashShelfEntry`] provides stable objects with:
- Object-ID-derived entry ID, creator, source repo/worktree, and stash-commit
  timestamp. The moving `stash@{n}` selector and observation time are never
  used as durable stash identity or creation time.
- Included path scope (tokens, not raw paths).
- Untracked-state posture.
- Message, checkpoint refs.
- Explicit command classes: `cmd:git.stash.apply`, `cmd:git.stash.pop`, `cmd:git.stash.drop`, `cmd:git.stash.branch_from`.

### Content availability
History, blame, and diff rows label content as:
- `available` — present locally.
- `unfetched` — known but not fetched.
- `omitted_sparse` — sparse-checkout omitted.
- `omitted_shallow` — shallow-history omitted.
- `uninitialized_submodule` — submodule not initialized.
- `pointer_only` — LFS or similar pointer.
- `not_repository` — path is not inside a Git repo.

### Records emitted
- [`DailyLoopSnapshot`] — canonical read-only snapshot.
- [`DailyLoopPreview`] — preview before mutation.
- [`DailyLoopResult`] — result after mutation.
- [`DailyLoopActivityRecord`] — activity-center projection.
- [`DailyLoopSupportExportRecord`] — support-export projection.
- [`DailyLoopJournalRecord`] — journal projection.

## Verification

Run tests:
```bash
cargo test --locked -p aureline-git --test daily_loop_beta
```

Run CLI snapshot:
```bash
cargo run --locked -p aureline-git --bin aureline_git_daily_loop -- --kind status --root .
```

## Known limits

- Structured diff parsing is unavailable; non-empty diffs are labeled `partial_omitted` with no fabricated file or hunk rows. A bounded unified-diff parser is planned for M5.
- Blame uses line-porcelain records so every emitted line repeats and validates its own commit provenance, but it does not yet correlate shallow/unfetched commit availability (locally returned commits report `available`).
- Status does not yet qualify submodule membership and keeps the field false; explicit index-mode correlation is planned for M5.
- Stash mutation verbs are intentionally inspect-only. The adapter does not invoke `stash push`, `apply`, `pop`, `drop`, or `branch` from a daily-loop preview.
- Support-export schema v2 is the only exportable daily-loop support shape. V1 embedded a full target object and must not be forwarded; migration is documented in `docs/migration/git/daily_loop_support_export_v1_to_v2.md`.
- The system backend uses the shared hardened Git runner. Request path scope, Git output, retained patch evidence, and stdin are bounded; private stderr and backend error text are not projected into result/support reasons. Local observation and mutation commands deny network/file transports; only the separately reviewed publish posture admits its explicit local/file, SSH, or HTTPS destination.
