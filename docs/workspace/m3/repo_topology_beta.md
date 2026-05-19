# Repository-topology beta truth

Beta layer on top of the alpha `repo_topology_state_record` and
`object_availability_state_record` model in
[`repository_topology_edge_case_contract.md`](../repository_topology_edge_case_contract.md).
The alpha layer froze the topology vocabulary (root kinds, honesty
labels, affordances, packet preservation). The beta layer binds that
vocabulary into four durable descriptors and one cross-surface
projection so workspace, search, graph, blame, review, AI context,
execution, publish, and support surfaces always agree on:

- the root identity, parent/child linkage, remote summary, trust, and
  policy posture of every claimed beta large-repo row;
- whether history is shallow, the object store has a partial-clone
  filter, or a promisor remote owns the missing objects;
- whether each submodule link is initialized, dirty, or drifted from
  its parent gitlink pin; and
- whether Git LFS assets are pointer-only, partially hydrated, or
  fully hydrated, and whether edit / preview / export will operate on
  the pointer or the hydrated body.

The exit-gate condition the surfaces guard together is the M03-222
anchor: **Aureline does not confuse omitted, unfetched, uninitialized,
pointer-only, generated, or excluded repository state with fully
present local truth on any claimed beta large-repo row.**

The machine-readable boundaries are:

- [`/schemas/workspace/repo_root_descriptor.schema.json`](../../../schemas/workspace/repo_root_descriptor.schema.json)
- [`/schemas/workspace/fetch_depth_descriptor.schema.json`](../../../schemas/workspace/fetch_depth_descriptor.schema.json)
- [`/schemas/workspace/submodule_link.schema.json`](../../../schemas/workspace/submodule_link.schema.json)
- [`/schemas/workspace/lfs_hydration_descriptor.schema.json`](../../../schemas/workspace/lfs_hydration_descriptor.schema.json)

The worked fixtures live under:

- [`/fixtures/workspace/m3/repo_topology_and_partial_clone/`](../../../fixtures/workspace/m3/repo_topology_and_partial_clone/)

The Rust types are exported from `aureline_workspace::repo_topology`.
The integration test
[`crates/aureline-workspace/tests/repo_topology_beta.rs`](../../../crates/aureline-workspace/tests/repo_topology_beta.rs)
replays every fixture and proves the closed acceptance states. The
beta layer composes the alpha topology and object-availability
contracts; if the two layers disagree, the alpha contract wins and the
beta schema / projection updates together.

## 1 Beta truth contract

Every beta-claimed large-repo row reads exactly one
`RepoTopologyBetaProjection`. The projection is derived from one
`RepoRootDescriptor` plus, when present, one `FetchDepthDescriptor`,
zero or more `SubmoduleLink`s, and one `LfsHydrationDescriptor`. Each
descriptor binds back to the alpha `repo_topology_state_record` by
`repo_topology_state_ref` so surfaces can re-resolve the underlying
honesty labels without re-flattening them.

The projection emits one closed answer per surface for:

1. `may_claim_full_coverage` — false whenever the active root is
   sparse, shallow, partial-clone, uninitialized, nested-independent,
   pointer-only, partially-hydrated, policy-blocked, or unavailable.
   A search, graph, blame, review, AI, publish, or migration surface
   that needs to claim "fully covered" MUST downgrade when this is
   false.
2. `full_coverage_blockers` — the typed reasons the claim is blocked.
   The vocabulary is frozen: `sparse_or_workset_narrowed`,
   `shallow_history_present`, `partial_clone_promisor_present`,
   `submodule_uninitialized`, `nested_independent_boundary`,
   `lfs_pointer_only_present`, `lfs_partially_hydrated`,
   `policy_blocked`, `unavailable_unknown`.
3. `required_affordances` — the typed actions a user must take before
   the surface may claim broader coverage. Mapped one-to-one from the
   blocker vocabulary to `widen_workset_scope`,
   `fetch_missing_objects`, `deepen_history`, `init_submodule`,
   `hydrate_lfs_objects`, and `switch_target_root`. When no blocker is
   present the value is `none_available`.
4. `mutation_target` — the only safe target for an edit/refactor/AI
   apply on the active row: `parent_root`, `child_root`,
   `switch_root_required`, `read_only_until_initialized`,
   `read_only_until_hydrated`, or `policy_blocked`.
5. `body_export_posture` — what packets and previews may embed:
   `hydrated_bytes_allowed`, `pointer_metadata_only`,
   `blocked_by_policy`, or `unavailable`.
6. `honesty_labels` — the alpha honesty vocabulary the surface must
   render alongside the row (`outside_current_slice`, `shallow_boundary`,
   `not_fetched`, `submodule_not_initialized`, `nested_repo_boundary`,
   `pointer_only`, `policy_excluded`, `unavailable`).

## 2 Descriptor binding

| Descriptor | Binds | Required when |
|---|---|---|
| `RepoRootDescriptor` | Root identity, kind (`primary`/`nested`/`submodule`/`worktree`), parent link, remote summary, trust posture, policy posture, completeness class, supported affordances. | Always — every projection MUST quote one. |
| `FetchDepthDescriptor` | Shallow/full history, partial-clone filter, promisor state, fetch policy, denial reason. | A root has `topology_classes` containing `shallow_history_root` or `partial_clone_promisor_root`, or its completeness is `shallow_history_present` / `partial_clone_promisor_present`. |
| `SubmoduleLink` | Pinned commit, init state, child dirty state, drift state, parent mutation posture for one parent ↔ child gitlink. | The parent root has any submodule link, or the active child root has `parent_link.linkage_class == submodule_child`. |
| `LfsHydrationDescriptor` | Hydration summary, asset buckets, hydrate policy/lock/cost posture, preview/export/edit posture. | The root has `topology_classes` containing `lfs_hydration_boundary` or `completeness_class` is `lfs_pointer_only_present` / `lfs_partially_hydrated`. |

The projection refuses to assemble if a fetch/depth, submodule, or LFS
record references a different `repo_root_descriptor_id` than the
active root. This guarantees that downstream surfaces never read a
descriptor that bound to a sibling or stale root.

## 3 Surface obligations

| Surface | Obligation |
|---|---|
| Workspace | Renders the root kind, completeness class, and parent link badges. Routes new sessions to the safe mutation target. |
| Search | Carries `outside_current_slice` and `not_fetched` honesty labels on row groups when blockers are present; never claims "no results" when coverage is partial. |
| Graph | Stops traversal at the same blockers and labels the boundary edge with the alpha honesty label. |
| Blame | Renders `shallow_boundary` and `not_fetched` and offers `deepen_history` / `fetch_missing_objects` only when policy allows. |
| Review | Shows the submodule gitlink pin, drift state, and child dirty state without claiming the parent owns the child files; child file review opens against the child root. |
| AI context | Evidence packets cite the projection by `repo_root_descriptor_ref` for every omitted, pointer-only, unfetched, or wrong-root segment. |
| Execution | Resolves the safe mutation target before acting; wrong-root and read-only hydration / initialization states deny with typed reasons. |
| Publish | Reads `body_export_posture` before packaging artifacts; pointer-only or policy-blocked bodies are exported as metadata only. |
| Support | Preserves every descriptor and the projection in support bundles; no raw paths, credentials, or object bytes travel. |
| Migration | Replays projections through the closed vocabulary; never widens scope silently while migrating between hosts. |

## 4 Mutation safety

The `mutation_target` vocabulary is the only path from a beta row to a
mutating action. A surface MUST refuse to apply a refactor, AI edit,
or publish action when the target is not `parent_root` or `child_root`,
and MUST present the matching affordance from `required_affordances`
when it asks the user how to widen coverage.

Specifically:

- `parent_root` — the active root accepts mutations directly.
- `child_root` — the request belongs to a submodule, nested-independent,
  or linked worktree; the surface opens the child root before mutating.
- `switch_root_required` — the active root is a nested independent
  repository; mutation is denied until the caller explicitly switches
  target via `switch_target_root`.
- `read_only_until_initialized` — at least one submodule link is
  uninitialized; mutation is denied until `init_submodule` runs.
- `read_only_until_hydrated` — at least one LFS asset is pointer-only;
  edits and exports operate on the pointer until `hydrate_lfs_objects`
  runs.
- `policy_blocked` — policy denies the mutation; the surface surfaces
  the policy ref and the matching review affordance instead of
  retrying.

## 5 Body export posture

`body_export_posture` is the contract publish, review, AI evidence,
and support packets read before they embed file bytes:

- `hydrated_bytes_allowed` — the asset is locally present and may be
  embedded subject to redaction class.
- `pointer_metadata_only` — pointer text, size band, and lock posture
  travel; the body does not. Surface MUST render
  `pointer_only` honesty.
- `blocked_by_policy` — policy denies the export. Surface records the
  policy ref instead of the body.
- `unavailable` — the body is not retrievable (offline-cache, hydrate
  failed). Surface records availability and offers retry or hydrate.

## 6 Fixture coverage

The beta fixture suite covers, at minimum:

- `primary_full_local_truth` — every claim allowed; mutation routes to
  the parent root.
- `primary_shallow_partial_clone_promisor` — shallow + partial clone +
  promisor; deepen + fetch required.
- `monorepo_with_uninitialized_submodule` — submodule link without
  init; read-only-until-init mutation target.
- `monorepo_lfs_pointer_only` — LFS pointer-only; pointer-metadata-only
  export and read-only-until-hydrate mutation target.
- `submodule_root_initialized` — initialized submodule root with
  drift; mutation routes to child root.
- `nested_independent_root` — nested boundary; mutation requires a
  switch-root.
- `linked_worktree_root` — sparse projection over a shared object
  store; widen-scope required before claiming broader coverage.

Removing any of these scenarios without a replacement fixture is a
breaking contract change.
