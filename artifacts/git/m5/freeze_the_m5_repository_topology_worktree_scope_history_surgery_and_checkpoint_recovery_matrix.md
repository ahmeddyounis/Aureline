# M5 Repository-Topology and History-Surgery Matrix

- Packet: `m5-git-topology-history-matrix:frozen:0001`
- Schema: `schemas/git/freeze-the-m5-repository-topology-worktree-scope-history-surgery-and-checkpoint-recovery-matrix.schema.json`
- Support export: `artifacts/git/m5/freeze_the_m5_repository_topology_worktree_scope_history_surgery_and_checkpoint_recovery_matrix/support_export.json`
- Contract doc: `docs/git/m5/freeze_the_m5_repository_topology_worktree_scope_history_surgery_and_checkpoint_recovery_matrix.md`
- Fixtures: `fixtures/git/m5/freeze_the_m5_repository_topology_worktree_scope_history_surgery_and_checkpoint_recovery_matrix/`

## Coverage

The matrix freezes one shared topology and history-surgery vocabulary for every
claimed M5 Git and source-acquisition surface.

- **Topology classes** (8): sparse checkout, promisor partial clone, shallow
  history, submodule, nested independent repo, worktree root, Git-LFS pointer
  boundary, and generated/vendor root. Each row binds a controlled degraded
  vocabulary, a mutation scope, the preview that must precede a mutation, and
  the recovery class that stays reachable. Sparse/partial/shallow stay
  active-root mutable with previews; submodule and nested roots require explicit
  child targeting and multi-root previews; pointer-only assets stay metadata-only
  until hydrated; generated/vendor roots stay read-only.
- **Session objects** (5): conflict session, sequence-edit session, recovery
  checkpoint, publish ref-update proposal, and stash shelf entry. Each row
  references the canonical risky-VCS record kind by id rather than redefining it,
  so every surface reads the same object. Mutating sessions require a checkpoint
  or another reachable recovery before they run.
- **Degraded vocabulary** (7): omitted, unfetched, uninitialized, pointer-only,
  dirty-child, reflog-only fallback, and stale-provider-overlay. Every state
  narrows a coverage claim — it is never reduced to a single badge — and the
  reflog-only fallback is always visible before a destructive operation.
- **Risky operations** (12): merge, rebase, interactive rebase, cherry-pick,
  revert, reset, stash apply/pop/drop, branch-from-stash, publish, and
  force-push-with-lease. Every operation carries an explicit preview class and a
  reachable recovery class, with recovery truth shown before execution.

## Guardrails proven

- Topology truth is not reduced to badges: the matrix controls actual mutation,
  preview, recovery, and export behavior.
- Provider overlays never overwrite local Git truth; a stale overlay narrows the
  claim and local truth wins.
- Recovery checkpoints or reflog-only fallbacks stay visible before any
  destructive operation.
- Provider overlay, AI context, search, review, support/export, and CLI surfaces
  can all express the same topology and recovery vocabulary.
- Sparse, partial, shallow, submodule, nested, LFS, and worktree states stay
  distinct product states rather than collapsing into one "incomplete" answer.

## Freeze posture

Frozen as canonical M5 truth with a 720-hour review SLO and automatic narrowing
on stale review. Raw paths, raw object bytes, raw branch names, raw
patch/reflog/stash bodies, raw provider payloads, and credentials never cross the
support boundary.
