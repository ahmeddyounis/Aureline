# Repo-topology beta known limits

This document records the explicit limits of the beta repo-topology
lane covered by the conformance corpus at
[`fixtures/workspace/m3/repo_topology_corpus/`](../../../fixtures/workspace/m3/repo_topology_corpus/).
It is the authoritative list of surfaces that MAY appear in the IDE
but MUST NOT claim beta conformance until the listed dependency
lands.

The beta truth contract is published in
[`repo_topology_beta.md`](./repo_topology_beta.md); the
conformance evidence is published in
[`artifacts/workspace/m3/repo_topology_conformance.md`](../../../artifacts/workspace/m3/repo_topology_conformance.md).

## In scope for the beta corpus

- Single primary roots with sparse / workset-narrowed coverage,
  shallow history, partial-clone (`blob:none`) with a reachable
  promisor, and any combination of the above.
- Monorepo roots with uninitialized submodules (read-only until
  init) and initialized submodules with drift (mutation routes to
  the child root).
- Submodule roots opened directly (mutation routes to the child root).
- Nested-independent repository boundaries discovered inside a
  parent checkout (mutation requires a switch-root).
- Linked Git worktrees over a shared object store (mutation routes
  to the worktree child root; widen-scope still required for broader
  coverage).
- Git LFS hydration postures: pointer-only,
  partially-hydrated (per-asset hydration mixed), and fully
  hydrated. Pointer-only and partially-hydrated states embed pointer
  metadata only; fully-hydrated state allows hydrated-bytes export.
- Policy-blocked fetch / deepen / export, surfaced through the
  `policy_blocked` blocker, the `policy_blocked` mutation target,
  and the `blocked_by_policy` body-export posture.
- Support-export packets that preserve every descriptor and the
  projection without leaking raw paths, credentials, raw remote URLs,
  raw object bytes, raw blob content, raw commit messages, or raw
  LFS pointer bodies.

## Out of scope (do NOT claim beta)

- Provider-specific LFS hosting (Git LFS server orchestration, lock
  arbitration across providers, batch-API negotiation). The pointer
  / hydrated split is recognized; hosting it is not in scope.
- Recursive monorepo orchestration (nested submodules of submodules
  with cross-root rebases, merge-queue execution across nested
  roots). The corpus exercises one level of submodule and one level
  of nested-independent boundary; deeper recursion is explicitly
  deferred.
- Automatic widening of workset scope, history depth, partial-clone
  filter, submodule init, or LFS hydration without an explicit user
  affordance. The corpus protects against silent widening by design.
- Conflict resolution across topology widenings (e.g. resolving a
  rebase that would require a deepened history, an initialized
  submodule, or hydrated LFS bytes during the resolution). The
  `read_only_until_initialized`, `read_only_until_hydrated`, and
  `policy_blocked` mutation targets are the supported wedges until
  that lane lands.
- Hosted history rewrite or filter-repo style operations that
  rewrite topology metadata server-side. The corpus exercises local
  topology truth; provider-side rewrite remains read-only block-reason
  consumer.
- Cross-repository sharing of pointer-only LFS pools or partial-clone
  promisor remotes. The descriptor binds one root at a time; multi-root
  sharing of object stores is allowed only via the linked-worktree
  case in this corpus.
- Time-travel debugging across deepened history or hydrated LFS
  states. Deepen / hydrate / init are forward-only affordances in the
  beta contract.
- Migration of generated artifacts (templates, prebuilds) across a
  topology widening. The corpus pins the projection; migration of
  generated state across widenings is owned by other M3 lanes.

## Negative invariants the corpus guards

The corpus' negative drills exist to make these regressions visible
without manual review:

- A fetch/depth descriptor MUST bind the active root via
  `repo_root_descriptor_ref`; a sibling-root reference MUST fail
  projection. (`negative.fetch_depth_root_mismatch`)
- An LFS hydration descriptor MUST bind the active root via
  `repo_root_descriptor_ref`; a sibling-root reference MUST fail
  projection. (`negative.lfs_hydration_root_mismatch`)
- A submodule link MUST bind the active root as its
  `parent_repo_root_descriptor_ref`; a sibling parent MUST fail
  projection. (`negative.submodule_link_parent_mismatch`)
- Raw paths, raw remote URLs, raw object bytes, raw blob content,
  and raw LFS pointer bodies MUST NOT cross the support-export
  boundary. The runner scans every positive fixture for these tokens
  and fails the drill before projection if any appear.

A drift in any of these invariants fails the conformance harness; the
corresponding drill report points at the exact regression class.

## Verification

```
cargo test -p aureline-qe --test repo_topology_conformance
```

The harness exits non-zero with a per-drill failure list when the
corpus regresses. Treat any failure as a beta release blocker for the
workspace-topology lane.
