# Repository-topology beta conformance and recovery-drill corpus

This corpus is the failure / recovery drill harness for the four
repo-topology beta descriptors and their cross-surface projection
(`RepoRootDescriptor`, `FetchDepthDescriptor`, `SubmoduleLink`,
`LfsHydrationDescriptor`, `RepoTopologyBetaProjection`).

It complements the smaller scenario set under
[`/fixtures/workspace/m3/repo_topology_and_partial_clone/`](../repo_topology_and_partial_clone/)
with explicit drills for sparse / workset narrowing, shallow history,
partial-clone (promisor) gaps, uninitialized and drifted submodules,
nested-independent boundaries, linked worktrees, pointer-only and
partially-hydrated Git LFS assets, policy blocks on both fetch and
export, and the negative invariants that prove the projection refuses
to assemble when a fetch/depth, submodule, or LFS descriptor binds a
sibling root.

Every drill is loaded by the conformance harness at
[`crates/aureline-qe/src/repo_topology/`](../../../../crates/aureline-qe/src/repo_topology/).
Positive drills MUST parse and project, and their projection MUST
match every `expected_*` field in `manifest.json`. Negative drills
MUST fail projection with an error whose message contains
`expected_failure_substring`.

Schemas:

- [`/schemas/workspace/repo_root_descriptor.schema.json`](../../../../schemas/workspace/repo_root_descriptor.schema.json)
- [`/schemas/workspace/fetch_depth_descriptor.schema.json`](../../../../schemas/workspace/fetch_depth_descriptor.schema.json)
- [`/schemas/workspace/submodule_link.schema.json`](../../../../schemas/workspace/submodule_link.schema.json)
- [`/schemas/workspace/lfs_hydration_descriptor.schema.json`](../../../../schemas/workspace/lfs_hydration_descriptor.schema.json)

Reviewer guidance: [`docs/workspace/m3/repo_topology_beta.md`](../../../../docs/workspace/m3/repo_topology_beta.md).
Known limits: [`docs/workspace/m3/repo_topology_known_limits.md`](../../../../docs/workspace/m3/repo_topology_known_limits.md).
Conformance artifact: [`artifacts/workspace/m3/repo_topology_conformance.md`](../../../../artifacts/workspace/m3/repo_topology_conformance.md).

## Coverage axes

| Axis | Drill ids |
| --- | --- |
| Fully present local truth — no blockers | `primary.full_local_truth`, `lfs.fully_hydrated_export_allowed`, `submodule.initialized_drift_child_route`, `submodule.root_opened_directly_child_route` |
| Sparse / workset-narrowed coverage | `sparse.workset_narrow_widen_required`, `worktree.linked_sparse_child_route` |
| Shallow history | `shallow.deepen_required`, `partial_clone.shallow_combo`, `policy.blocked_fetch_denies_widen` |
| Partial-clone / promisor object gaps | `partial_clone.promisor_fetch_required`, `partial_clone.shallow_combo` |
| Uninitialized submodule | `submodule.uninitialized_read_only` |
| Initialized submodule with drift | `submodule.initialized_drift_child_route` |
| Submodule root opened directly | `submodule.root_opened_directly_child_route` |
| Nested independent boundary | `nested.independent_switch_root` |
| Linked worktree (child root) | `worktree.linked_sparse_child_route` |
| Pointer-only LFS / metadata-only export | `lfs.pointer_only_metadata_export` |
| Partially hydrated LFS / hydrate required | `lfs.partially_hydrated_still_blocked` |
| Fully hydrated LFS / hydrated export | `lfs.fully_hydrated_export_allowed` |
| Policy-blocked fetch / deepen | `policy.blocked_fetch_denies_widen` |
| Policy-blocked export / `blocked_by_policy` posture | `policy.blocked_export_metadata_only` |
| Negative — fetch/depth descriptor binds sibling root | `negative.fetch_depth_root_mismatch` |
| Negative — LFS hydration descriptor binds sibling root | `negative.lfs_hydration_root_mismatch` |
| Negative — submodule link parent ≠ active root | `negative.submodule_link_parent_mismatch` |

## Running the corpus

```
cargo test -p aureline-qe --test repo_topology_conformance
```

The crate also exposes the corpus loader + projection assertions as a
library, so other harnesses (workspace UI checks, AI evidence
packets, support-export parity reviews, migration replays) can quote
the same drill matrix without re-parsing the fixtures.

## Redaction guarantees

Every fixture is metadata-safe: only opaque refs and typed labels
cross the boundary. Raw absolute paths, raw remote URLs, raw object
bytes, raw blob content, raw commit messages, and raw LFS pointer
bodies never appear. Removing any positive or negative drill without
a replacement fixture is a breaking contract change for the
`workspace.repo_topology_corpus.beta` corpus.
