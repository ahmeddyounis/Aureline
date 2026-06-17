# Repository-Topology and History-Surgery Matrix (contract)

This document is the human-readable contract for the frozen repository-topology,
worktree-scope, history-surgery, and checkpoint-recovery matrix. The
machine-readable boundary is the schema and the checked support export:

- Schema: `schemas/git/freeze-the-m5-repository-topology-worktree-scope-history-surgery-and-checkpoint-recovery-matrix.schema.json`
- Support export: `artifacts/git/m5/freeze_the_m5_repository_topology_worktree_scope_history_surgery_and_checkpoint_recovery_matrix/support_export.json`
- Typed model: `crates/aureline-git/src/freeze_the_m5_repository_topology_worktree_scope_history_surgery_and_checkpoint_recovery_matrix/`

## Purpose

Sparse checkouts, partial clones, shallow history, submodules, nested repos, LFS
pointers, worktrees, and risky history mutations are distinct product states.
This matrix makes those states — and the recovery truth that protects them — a
single governed product object rather than implicit CLI knowledge split across
entry sheets, provider overlays, and fallbacks.

The matrix is the single source of truth for whether a Git or source-acquisition
surface may claim complete coverage, target a root for mutation, or run a risky
history operation. It references the canonical topology, conflict-session,
sequence-edit, recovery-checkpoint, stash, and ref-update contracts by id instead
of redefining them, so provider overlay, AI context, search, review, CLI, and
support/export flows all read one vocabulary.

## What the matrix governs

### Topology classes

Each class binds a controlled degraded vocabulary, the mutation scope it permits,
the preview that must precede a mutation, the recovery class that stays reachable,
and the consumer surfaces that must project it.

| Class | Degraded states | Mutation scope | Preview | Recovery |
| --- | --- | --- | --- | --- |
| Sparse checkout root | omitted | active root only | scope-widen preview | checkpoint before mutation |
| Promisor partial clone | unfetched | active root only | fetch/deepen preview | checkpoint before mutation |
| Shallow history root | unfetched | active root only | fetch/deepen preview | checkpoint before mutation |
| Submodule root | uninitialized, dirty-child | child root only | multi-root preview | checkpoint before mutation |
| Nested independent repo | dirty-child | child root only | multi-root preview | checkpoint before mutation |
| Worktree root | dirty-child | active root only | diff preview | checkpoint before mutation |
| LFS pointer boundary | pointer-only | metadata only | fetch/deepen preview (hydrate) | none (no mutation) |
| Generated / vendor root | omitted | mutation denied | none (read-only) | none (no mutation) |

### Session objects

Each session object references its canonical risky-VCS record kind by id:

- **Conflict session** — `review_risky_vcs_conflict_session_object`
- **Sequence-edit session** — `review_risky_vcs_sequence_edit_session_object`
- **Recovery checkpoint** — `review_risky_vcs_recovery_checkpoint_object`
- **Publish ref-update proposal** — `review_risky_vcs_ref_update_proposal_object`
- **Stash shelf entry** — `review_risky_vcs_stash_entry_object`

Every mutating session must require a checkpoint or expose another reachable
recovery before it runs; the recovery checkpoint is itself the restore surface.

### Degraded vocabulary

`omitted`, `unfetched`, `uninitialized`, `pointer_only`, `dirty_child`,
`reflog_only_fallback`, and `stale_provider_overlay`. Each state narrows a
coverage claim (it is never reduced to a single badge), and the reflog-only
fallback must be visible before any destructive operation.

### Risky operations

Merge, rebase, interactive rebase, cherry-pick, revert, reset, stash
apply/pop/drop, branch-from-stash, publish, and force-push-with-lease each carry
an explicit preview class and a reachable recovery class, with recovery truth
shown before execution and explicit-target selection required.

## Invariants enforced

The typed `validate` mirrors the schema and adds the cross-row rules a schema
cannot express:

- Every frozen topology class, session object, degraded state, and risky
  operation is present exactly once.
- A topology row that permits mutation must carry both a mutation preview and a
  reachable recovery class.
- A session row's `canonical_record_kind` must match its canonical object.
- Every degraded state must narrow a claim; the reflog-only fallback must be
  visible before destructive operations.
- Provider overlay, AI context, search, review, support/export, and CLI must all
  be able to express the vocabulary.
- The export carries no raw boundary material.

## Out of scope

This lane does not add new hosted review/provider breadth. It freezes the
topology and recovery truth that the broader M5 Git depth surfaces build on.
