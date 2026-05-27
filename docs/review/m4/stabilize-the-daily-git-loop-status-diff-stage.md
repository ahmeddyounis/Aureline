# Stabilize the daily Git loop — review documentation

## Overview

The daily Git loop module (`stabilize_the_daily_git_loop_status_diff_stage`) provides the first stable, preview-first contract for the core Git operations that Aureline users perform every day. It is designed to be trustworthy enough that target personas can replace incumbent tools on normal workdays.

## Design principles

1. **Explicit targeting** — Every operation names the exact repository and worktree it acts on. Nested repos, submodules, and linked worktrees never collide.
2. **Preview-first mutations** — Stage, unstage, commit, amend, and all stash transitions produce a preview before apply. No surprise mutations.
3. **Stable stash/shelf vocabulary** — One [`StashShelfEntry`] object owns a stash across its entire lifecycle. UI, CLI, support export, and migration help all speak the same vocabulary.
4. **Truthful content labeling** — Omitted, unfetched, uninitialized, or pointer-only content is labeled explicitly rather than appearing as normal absence.
5. **Attributable and reversible** — Every mutation carries an actor ref and offers a recovery checkpoint ref.

## Module structure

- `DailyLoopBackend` / `SystemDailyLoopBackend` — subprocess contract.
- `DailyLoopService` — orchestrates snapshot, preview, and apply.
- `DailyLoopRequest` — unified request with `RepoTarget`, `WorktreeTarget`, and operation kind.
- `DailyLoopSnapshot` — canonical read-only result for status, diff, blame, history, stash-list.
- `DailyLoopPreview` / `DailyLoopResult` — mutation pipeline.
- `StashShelfEntry` — durable stash/shelf object.
- `BlameLineRecord` / `HistoryCommitRecord` — content-provenance rows.

## Record kinds

| Record | Kind | Schema version |
|---|---|---|
| Snapshot | `git_daily_loop_snapshot` | 1 |
| Preview | `git_daily_loop_preview` | 1 |
| Result | `git_daily_loop_result` | 1 |
| Activity | `git_daily_loop_activity_record` | 1 |
| Support export | `git_daily_loop_support_export_record` | 1 |
| Journal | `git_daily_loop_journal_record` | 1 |
| Stash/shelf entry | `git_stash_shelf_entry_record` | 1 |
| Blame line | `git_blame_line_record` | 1 |
| History commit | `git_history_commit_record` | 1 |

## Integration touchpoints

- `crates/aureline-git` — canonical implementation.
- `crates/aureline-review` — diff and blame consumers.
- `crates/aureline-workspace` — repo/worktree target resolution.
- `crates/aureline-provider` — provider-linked publish continuity.

## Fixtures

Canonical fixtures live under `fixtures/git/m4/daily_loop_beta/`:
- `status_attached_dirty.yaml`
- `status_not_a_repository.yaml`
- `stash_list_with_stash.yaml`
- `history_with_commits.yaml`
- `commit_preview_blocked_no_message.yaml`
- `stage_preview_ready.yaml`

## Acceptance criteria

- [x] Implementation is checked in under `crates/aureline-git/src/stabilize_the_daily_git_loop_status_diff_stage/`.
- [x] CLI binary `aureline_git_daily_loop` is wired and runnable.
- [x] Stash/shelf entry objects survive restart and keep source repo/worktree provenance.
- [x] Daily-loop fixtures prove distinguishable paths in parent and child repos.
- [x] Omitted or unfetched content is labeled truthfully.
- [x] Any surface still lacking stable qualification is not labeled as Stable in product copy.
