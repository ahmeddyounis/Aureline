# Stabilize the daily Git loop — review documentation

## Overview

The daily Git loop module (`stabilize_the_daily_git_loop_status_diff_stage`) provides a bounded beta contract for core local Git operations. Stage, unstage, commit, and amend have canonical preview/apply authority; the remaining gaps are disclosed below and are not a Stable or full incumbent-replacement claim.

## Design principles

1. **Explicit targeting** — Every operation names the exact repository and worktree it acts on. Nested repos, submodules, and linked worktrees never collide.
2. **Preview-first mutations** — Stage, unstage, commit, and amend produce an in-process preview before apply. Stash transitions currently produce an inspect-only blocked preview until the adapter has exact checkpoint and stale-evidence authority.
3. **Stable stash/shelf vocabulary** — One [`StashShelfEntry`] object owns a stash across its entire lifecycle. UI, CLI, support export, and migration help all speak the same vocabulary.
4. **Truthful content labeling** — Omitted, unfetched, uninitialized, or pointer-only content is labeled explicitly rather than appearing as normal absence.
5. **Attributable and recoverable** — Admitted stage, unstage, commit, and amend operations reuse the canonical mutation/commit authority. A serialized or changed preview is never portable apply authority.

## Module structure

- `DailyLoopBackend` / `SystemDailyLoopBackend` — subprocess contract. The system backend reuses the hardened Git environment, bounded output capture, timeout, and process-tree termination posture.
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
| Support export | `git_daily_loop_support_export_record` | 2 |
| Journal | `git_daily_loop_journal_record` | 1 |
| Stash/shelf entry | `git_stash_shelf_entry_record` | 1 |
| Blame line | `git_blame_line_record` | 1 |
| History commit | `git_history_commit_record` | 1 |

## Integration touchpoints

- `crates/aureline-git` — canonical implementation.
- `crates/aureline-review` — diff and blame consumers.
- `crates/aureline-workspace` — repo/worktree target resolution.
- `crates/aureline-provider` — provider-linked publish continuity.

## Apply and export boundaries

- Stage and unstage delegate to the canonical mutation review service. Apply
  consumes the exact reviewed patch and revalidates repository, worktree, HEAD,
  index, worktree, and selected-path evidence. Drift blocks without mutation.
- Commit and amend delegate to the canonical commit review service and its
  single-process preview-authority store. Deserialized or tampered daily-loop
  previews block. Apply also requires the exact actor ref bound into review;
  callers cannot reattribute an existing preview at apply time.
- Requests are rejected before backend execution when command identity, roots,
  refs, timestamps, path normalization, path count, or aggregate path bytes are
  outside the bounded adapter contract.
- Support-export v2 replaces the full `DailyLoopTarget` with three
  domain-separated digests and topology booleans/classes. It exports affected
  path count but never absolute roots, Git-directory paths, display labels,
  branch/ref names, affected path bodies, outcome detail, or raw backend error
  text. See
  [`daily_loop_support_export_v1_to_v2.md`](../../migration/git/daily_loop_support_export_v1_to_v2.md).

## Fixtures

Canonical fixtures live under `fixtures/git/m4/daily_loop_beta/`:
- `status_attached_dirty.yaml`
- `status_not_a_repository.yaml`
- `stash_list_with_stash.yaml`
- `history_with_commits.yaml`
- `commit_preview_blocked_no_message.yaml`
- `stage_preview_ready.yaml`
- `support_export_redacted_v2.json`

## Acceptance criteria

- [x] Implementation is checked in under `crates/aureline-git/src/stabilize_the_daily_git_loop_status_diff_stage/`.
- [x] CLI binary `aureline_git_daily_loop` is wired and runnable.
- [x] Stage/unstage stale-evidence and non-portable-preview tests block without mutation.
- [x] Linked-worktree tests preserve a common repository identity and an exact worktree identity.
- [x] Support-export v2 fixtures omit raw target, path, branch, label, and failure-detail fields.
- [ ] Stash apply/pop/drop/branch transitions remain inspect-only until exact checkpoint authority exists.
- [ ] Diff hunk parsing and shallow/sparse/submodule content qualification remain follow-up work and are not claimed as complete here. A non-empty diff is returned as `partial_omitted`; the adapter never invents placeholder path or hunk rows.
- [x] Any surface still lacking stable qualification is not labeled as Stable in product copy.
