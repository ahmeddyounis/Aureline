# Repo-topology beta conformance evidence

This artifact is the release-consumable conformance evidence for the
M3 repo-topology beta lane. Every claimed beta sparse, multi-root,
submodule, partial-clone, or LFS-friendly row reads exactly one
`RepoTopologyBetaProjection`. Every projection is exercised by at
least one drill in
[`fixtures/workspace/m3/repo_topology_corpus/`](../../../fixtures/workspace/m3/repo_topology_corpus/);
the drills are executed by
[`crates/aureline-qe/src/repo_topology/`](../../../crates/aureline-qe/src/repo_topology/)
and replayed by
`cargo test -p aureline-qe --test repo_topology_conformance`.

The corpus is owned by the QE crate so the same fixture matrix can
gate desktop projections, CLI / headless mirrors, AI-context omission
rules, blame / search / review downgrades, publish posture decisions,
and support-export parity reviews from one shared truth.

The exit-gate condition the corpus guards is the M03-222 anchor:

> Aureline does not confuse omitted, unfetched, uninitialized,
> pointer-only, generated, or excluded repository state with fully
> present local truth on any claimed beta large-repo row.

## Coverage matrix

| Axis (Spec §) | Drill ids | Outcome anchored |
| --- | --- | --- |
| Fully present local truth — every claim allowed | `primary.full_local_truth`, `lfs.fully_hydrated_export_allowed`, `submodule.initialized_drift_child_route`, `submodule.root_opened_directly_child_route` | `may_claim_full_coverage = true`, `required_affordances = [none_available]`, `body_export_posture = hydrated_bytes_allowed`. |
| Sparse / workset narrowing — widen-scope required | `sparse.workset_narrow_widen_required`, `worktree.linked_sparse_child_route` | Blocker `sparse_or_workset_narrowed`, affordance `widen_workset_scope`, honesty label `outside_current_slice`. Search must downgrade per `surface_must_downgrade_claim`. |
| Shallow history — deepen required | `shallow.deepen_required`, `partial_clone.shallow_combo`, `policy.blocked_fetch_denies_widen` | Blocker `shallow_history_present`, affordance `deepen_history`, honesty label `shallow_boundary`. Blame may not claim full history coverage. |
| Partial-clone / promisor object gaps — fetch required | `partial_clone.promisor_fetch_required`, `partial_clone.shallow_combo` | Blocker `partial_clone_promisor_present`, affordance `fetch_missing_objects`, honesty label `not_fetched`. AI evidence packets must cite the projection by `repo_root_descriptor_ref`. |
| Uninitialized submodule — read-only until init | `submodule.uninitialized_read_only` | Blocker `submodule_uninitialized`, affordance `init_submodule`, mutation target `read_only_until_initialized`, honesty label `submodule_not_initialized`. |
| Initialized submodule with drift — child-root mutation route | `submodule.initialized_drift_child_route`, `submodule.root_opened_directly_child_route` | Mutation routes to `child_root`; parent coverage stays intact. |
| Nested-independent boundary — switch root required | `nested.independent_switch_root` | Blockers `nested_independent_boundary`, `unavailable_unknown` (untrusted trust posture). Mutation target `switch_root_required`. |
| Linked worktree — child-root mutation route | `worktree.linked_sparse_child_route` | Mutation routes to `child_root`; widen-scope still required for broader coverage. |
| LFS pointer-only — metadata-only export | `lfs.pointer_only_metadata_export` | Blocker `lfs_pointer_only_present`, affordance `hydrate_lfs_objects`, mutation target `read_only_until_hydrated`, body-export posture `pointer_metadata_only`. |
| LFS partially hydrated — still blocked | `lfs.partially_hydrated_still_blocked` | Blocker `lfs_partially_hydrated`, mutation target `read_only_until_hydrated`. |
| LFS fully hydrated — hydrated export allowed | `lfs.fully_hydrated_export_allowed` | `may_claim_full_coverage = true`, body-export posture `hydrated_bytes_allowed`. |
| Policy-blocked fetch / deepen | `policy.blocked_fetch_denies_widen` | Blocker `policy_blocked` is published alongside `shallow_history_present`; mutation target `policy_blocked`; honesty label `policy_excluded`. |
| Policy-blocked export | `policy.blocked_export_metadata_only` | Body-export posture `blocked_by_policy`; mutation target `policy_blocked`. |
| Negative — fetch/depth descriptor binds a sibling root | `negative.fetch_depth_root_mismatch` | Projection rejects with `fetch_depth descriptor references …`. |
| Negative — LFS hydration descriptor binds a sibling root | `negative.lfs_hydration_root_mismatch` | Projection rejects with `lfs_hydration descriptor references …`. |
| Negative — submodule link binds a sibling parent | `negative.submodule_link_parent_mismatch` | Projection rejects with `submodule link parent …`. |

The conformance harness also pins three transverse invariants across
the entire positive set:

- `corpus_covers_every_blocker_class` asserts at least one positive
  drill exercises each of `sparse_or_workset_narrowed`,
  `shallow_history_present`, `partial_clone_promisor_present`,
  `submodule_uninitialized`, `nested_independent_boundary`,
  `lfs_pointer_only_present`, `lfs_partially_hydrated`,
  `policy_blocked`, and `unavailable_unknown`.
- `corpus_covers_every_mutation_target` asserts at least one
  positive drill exercises each of `parent_root`, `child_root`,
  `switch_root_required`, `read_only_until_initialized`,
  `read_only_until_hydrated`, and `policy_blocked`.
- `corpus_covers_every_body_export_posture` asserts at least one
  positive drill exercises each of `hydrated_bytes_allowed`,
  `pointer_metadata_only`, and `blocked_by_policy`.

## Cross-surface projection check

The drills are read by every surface that quotes a repo-topology
record:

- Workspace: `repo_root_kind`, completeness class, parent-link
  badges, and the safe mutation target.
- Search / graph: `outside_current_slice`, `not_fetched`, and
  `shallow_boundary` honesty labels on row groups, plus
  `surface_must_downgrade_claim` when coverage is partial.
- Blame: `shallow_boundary` and `not_fetched` labels; the
  `deepen_history` / `fetch_missing_objects` affordances are only
  offered when policy allows.
- Review: submodule gitlink pin, drift state, and child dirty state
  routed via the `child_root` mutation target; uninitialized links
  stay read-only.
- AI evidence packets: `repo_root_descriptor_ref` for every omitted,
  pointer-only, unfetched, or wrong-root segment.
- Execution: mutation target is resolved before any apply; wrong-root
  and read-only states deny with typed reasons.
- Publish: `body_export_posture` is read before packaging; pointer-only
  and policy-blocked bodies travel as metadata.
- Support: every descriptor and the projection are preserved in the
  support packet; no raw paths, credentials, or object bytes travel.
- Migration: the closed vocabulary is replayed end-to-end, never
  widening scope silently while migrating between hosts.

The conformance harness asserts the projection truth for every
drill, so a drift in any surface's read path appears as a failure on
the corresponding drill rather than silently passing through.

## Replay

```
cargo test -p aureline-qe --test repo_topology_conformance
```

The corpus manifest at
`fixtures/workspace/m3/repo_topology_corpus/manifest.json` is the
canonical pass / fail input; CI consumers SHOULD treat any
`failures()` returned by `run_corpus_from_repo_root` as a beta release
blocker for the workspace-topology lane.

## Redaction guarantees

Every fixture is metadata-safe: only opaque refs and typed labels
cross the boundary. The runner additionally scans each positive
fixture for forbidden raw-export tokens
(`raw_path_export_allowed`, `raw_remote_url_export_allowed`,
`raw_object_bytes_export_allowed`, `raw_blob_body_export_allowed`,
`raw_pointer_body_export_allowed`); any occurrence fails the drill
before projection. This keeps the redaction contract on the corpus
itself, not only on individual surface read paths.

## Known limits

See
[`docs/workspace/m3/repo_topology_known_limits.md`](../../../docs/workspace/m3/repo_topology_known_limits.md)
for the explicit beta-out-of-scope items, including provider-side
filter-spec hosting, recursive monorepo orchestration, and conflict
resolution on widened scopes.
