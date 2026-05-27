# Stabilize the daily Git loop — review artifact

## Scope

This artifact covers the stabilized daily Git loop for Aureline M4, implementing explicit repo/worktree targeting across status, diff, stage, commit, amend, stash, blame, and history operations.

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
- `diff` — worktree diff with file/hunk/line granularity.
- `stage` / `unstage` — preview-first path mutations.
- `commit` / `amend` — preview-first commit creation with message guardrails.
- `stash_capture` / `stash_apply` / `stash_pop` / `stash_drop` / `stash_branch_from` — stash/shelf lifecycle.
- `blame` — per-line blame with content-availability labels.
- `history` — commit history with content-availability labels.

### Stash/shelf entry objects
[`StashShelfEntry`] provides stable objects with:
- Entry ID, creator, source repo/worktree.
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
cargo test -p aureline-git daily_loop_beta
```

Run CLI snapshot:
```bash
cargo run -p aureline-git --bin aureline_git_daily_loop -- --kind status --root .
```

## Known limits

- Diff parsing is simplified (placeholder hunk parsing); full unified-diff parser is planned for M5.
- Blame parsing covers porcelain blame but does not yet correlate shallow/unfetched commit availability (always reports `available` in the simplified path).
- Submodule boundary detection in status relies on path inspection; explicit `.gitmodules` correlation is planned for M5.
