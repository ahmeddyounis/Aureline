# Repository-topology edge-case truth contract

This document freezes the shared repository-topology vocabulary every
Aureline workspace, search, navigation, blame, review, AI, execution,
export, and support surface reads when a repository is not a simple
fully materialized checkout. Sparse checkouts, named worksets, partial
clones, shallow histories, submodules, nested independent repositories,
and Git LFS pointer states are first-class truth states, not missing-file
exceptions.

The machine-readable boundaries are:

- [`/schemas/workspace/repo_topology_state.schema.json`](../../schemas/workspace/repo_topology_state.schema.json)
  - root, worktree, parent-child, omitted-path, depth, promisor,
  pinning, dirty-state, hydration-summary, safe-mutation, affordance,
  and export/support requirements.
- [`/schemas/workspace/object_availability_state.schema.json`](../../schemas/workspace/object_availability_state.schema.json)
  - per-object or per-path availability, fetch/depth/hydration state,
  claim permissions, wrong-root guards, and packet-preservation
  requirements.

Worked fixtures live under:

- [`/fixtures/workspace/repository_topology_cases/`](../../fixtures/workspace/repository_topology_cases/)

This contract composes existing scope, search, VCS, AI, and support
contracts. It does not redefine named-workset artifacts
([`scope_truth_packet.md`](scope_truth_packet.md)), branch/worktree rows
([`../vcs/git_state_and_worktree_contract.md`](../vcs/git_state_and_worktree_contract.md)),
search result truth
([`../search/search_readiness_vocabulary.md`](../search/search_readiness_vocabulary.md)),
or support bundles
([`../support/support_bundle_contract.md`](../support/support_bundle_contract.md)).
Those surfaces quote these topology records by id instead of flattening
their caveats into generic prose.

If this document disagrees with the PRD, technical architecture,
technical design, or UI/UX source documents, those documents win and this
contract, schemas, and fixtures update together.

## Why Freeze This Now

Large repositories fail in ways that look deceptively similar: a file may
be absent, intentionally omitted from the active slice, known by manifest
but not materialized, represented only by a promisor object, hidden behind
a shallow boundary, present only as a Git LFS pointer, or owned by a
different repository root. Treating those states as one "missing file"
label lets downstream flows overclaim coverage and mutate the wrong root.

This contract makes each state reconstructable:

- search can report "no result in the current slice" without claiming no
  result exists globally;
- blame can offer a deepen action when history stops at a shallow
  boundary;
- review can show a submodule gitlink pin without pretending the parent
  repository owns the child files;
- AI evidence packets can explain which context segments were omitted,
  not fetched, or pointer-only;
- support packets can reconstruct whether content was absent, omitted,
  not fetched, blocked by initialization, or blocked by hydration; and
- mutation flows can deny a parent-root write when the active object
  belongs to a submodule or nested independent repository.

## Scope

Frozen at this revision:

1. `repo_topology_state_record` - one root/worktree topology snapshot.
   It carries identity, parent-child linkage, omitted-path sets,
   history depth, promisor/fetch posture, pinned commit, dirty state,
   hydration summary, safe mutation scope, offered affordances, and
   packet-preservation requirements.
2. `repo_topology_audit_event_record` - a denial or transition event
   emitted when a topology state blocks or redirects an operation, most
   importantly wrong-target-root mutation attempts.
3. `object_availability_state_record` - one object/path/history/LFS/
   submodule availability statement with explicit claim permissions.
4. `object_availability_audit_event_record` - a fetch, deepen, init,
   hydrate, export, or denial event tied to one availability record.

Out of scope:

- implementing Git sparse-checkout, partial-clone, submodule, or LFS
  orchestration;
- defining the named-workset artifact format;
- defining branch/worktree registry rows;
- fetching, deepening, initializing, or hydrating content; and
- changing user-facing copy beyond the frozen label vocabulary below.

## Topology Classes

Every topology state names one or more `repo_topology_class` values.
Surfaces may group them visually, but they may not merge their meanings.

| Class | Meaning | Must not be reported as |
|---|---|---|
| `current_repo_root` | The active single repository root for a plain open. | A full workspace when other roots are present. |
| `workset_root` | A root included by a named workset. | A global repository unless the workset is the full workspace. |
| `sparse_checkout_root` | Git sparse-checkout or sparse IDE slice is active. | Missing files or empty search results. |
| `worktree_root` | A Git worktree with its own mutation target. | Another worktree or an ambient branch. |
| `partial_clone_promisor_root` | Object completeness depends on a promisor remote or partial-clone filter. | Full object availability. |
| `shallow_history_root` | History is depth-limited. | Full ancestry for blame/history. |
| `submodule_root` | A child repository pinned by a parent gitlink. | A normal parent directory. |
| `nested_independent_repo_root` | A repository found inside another checkout without submodule linkage. | Part of the parent repository. |
| `lfs_hydration_boundary` | At least one visible object is governed by Git LFS pointer/hydration state. | Normal file content. |

## Required Truth Fields

Every `repo_topology_state_record` carries these fields so support and
export consumers can reconstruct the state later:

| Field group | Required truth |
|---|---|
| Repo/worktree identity | `repo_identity`, `worktree_identity`, active root ref, object-store ref, head revision, and worktree kind. |
| Parent-child linkage | `parent_child_linkage` with linkage class, parent root, child root, optional path ref, and pinned commit when applicable. |
| Omitted-path set | `omitted_path_sets` with omission class, pattern or manifest ref, hidden count when known, and the label downstream surfaces must carry. |
| Depth boundary | `history_boundary` with shallow/full/unknown state, depth, boundary commit, and deepen affordance ref. |
| Promisor/fetch policy | `promisor_fetch_policy` with promisor state, filter class, remote ref, allowed fetch policy, and last-fetch freshness. |
| Pinned commit | `pinned_commit` with commit ref and pinning source; submodule gitlinks must preserve this. |
| Dirty state | `dirty_state` with cleanliness class and the scope over which cleanliness was verified. |
| Hydration state | `hydration_state_summary` and object availability refs for LFS or generated-object hydration caveats. |
| Safe mutation scope | `safe_mutation_scope` with the only root/slice/worktree a mutation may target and any denial reason. |
| Caveat preservation | `export_support_requirements` with the labels and reconstruction fields packets must keep. |

Every `object_availability_state_record` carries the per-object truth:

| Field group | Required truth |
|---|---|
| Locator | Object ref, root ref, topology-state ref, path/revision/object id refs when known. |
| Availability | Exact availability class and the claim booleans for content, history, blame, search, mutation, body export, and metadata export. |
| Fetch/depth/hydration | `fetch_state`, `depth_state`, `hydration_state`, and optional promisor, depth, submodule, or LFS detail blocks. |
| Root guard | Active root vs authoritative root and the denial reason when they differ. |
| Affordance | The exact widen, fetch, deepen, init, hydrate, inspect, or switch-root action a surface may offer. |
| Packet preservation | Export/support requirements naming whether body bytes, refs, labels, or omission reasons travel. |

## Honesty Labels And Affordances

The label set is intentionally small. A surface that needs more detail
adds typed fields beside the label; it does not invent another label.

| Honesty label | Use when | Allowed affordance |
|---|---|---|
| `outside_current_slice` | The path or result is omitted by the active workset, sparse checkout, or policy-limited slice. | `widen_workset_scope` or `open_sparse_coverage_inspector`; never silent widening. |
| `not_fetched` | A promisor/partial-clone object is known but unavailable locally. | `fetch_missing_objects` with fetch policy and approval state. |
| `shallow_boundary` | History or blame stops at the clone depth boundary. | `deepen_history` with depth/cost disclosure. |
| `submodule_not_initialized` | Parent knows a gitlink but the child checkout is absent or unavailable. | `init_submodule` or `open_child_repo_root` after initialization. |
| `nested_repo_boundary` | A nested `.git` root is independent of the parent. | `switch_target_root` or `open_child_repo_root`; parent mutation is denied. |
| `pointer_only` | Git LFS pointer text is present but the large object is not hydrated. | `hydrate_lfs_objects` or `inspect_pointer`; body export stays metadata-only. |
| `wrong_target_root` | The requested action targeted a parent or sibling root while the active object belongs elsewhere. | `switch_target_root`; mutation is denied until target identity changes. |
| `dirty_state_unknown` | Cleanliness was not verified across the relevant scope. | Refresh status or limit claim to the verified slice. |
| `unavailable` | A root, remote, index, or object cannot currently be reached. | Retry, reconnect, or export metadata-only depending on policy. |
| `policy_excluded` | Policy/trust excludes the content or root. | Request policy review where supported; never fetch around policy. |
| `generated_or_excluded` | Generated or excluded roots are intentionally outside editable source truth. | Open lineage or generated-artifact inspector, not direct mutation by default. |

Affordances are optional. When offered, they must cite a command id or
explicitly state that no command is available. Network-bearing actions
(`fetch_missing_objects`, `deepen_history`, `init_submodule`,
`hydrate_lfs_objects`) carry approval/cost posture in the schema.

## Root Targeting Rules

1. A path inside a submodule resolves to the submodule root for mutation.
   The parent repository may expose only the gitlink pin, dirtiness, and
   summary unless the child root is opened explicitly.
2. A path inside a nested independent repository resolves to that nested
   root. A parent-root command that attempts to mutate it denies with
   `wrong_target_root`.
3. Worktree-aware operations bind to one `worktree_identity`. Cross-
   worktree mutation requires a separate selected target and must not be
   inferred from filesystem nesting.
4. Sparse/workset operations may mutate only the declared safe mutation
   scope. Widening scope is a reviewable change of topology state, not an
   invisible search or refactor side effect.
5. Pointer-only LFS files are not treated as hydrated body content.
   Preview, export, and mutation controls disclose whether they operate
   on the pointer or hydrated object.

## Surface Contract

| Surface | Required behavior |
|---|---|
| Search | Carries topology labels in result packets and hidden-scope disclosures; no "no results" claim when rows are outside current slice or not fetched. |
| Navigation | Deep links preserve root identity, workset/slice id, and drift labels; jumps across roots show the boundary. |
| Blame/history | Missing ancestry from shallow/partial states renders `shallow_boundary` or `not_fetched` and offers deepen/fetch only when policy allows. |
| Review | Parent diffs for submodules show gitlink pin and child dirty/init state; child file review opens against the child root. |
| AI context | Evidence packets cite topology and object-availability refs for every omitted, pointer-only, unfetched, or wrong-root segment. |
| Execution/mutation | Commands resolve safe mutation scope before acting; wrong-root and read-only hydration states deny with typed reasons. |
| Export/support | Packets preserve labels, root refs, omitted-path sets, depth/promisor policy, pinning, dirty state, hydration state, and safe mutation scope. |

## Export And Support Packet Requirements

Topology caveats survive every packet boundary. A downstream packet may
redact raw paths, query text, or object bytes, but it still preserves the
typed reconstruction fields:

- `repo_identity` and `worktree_identity`;
- `parent_child_linkage`;
- `omitted_path_sets`;
- `history_boundary`;
- `promisor_fetch_policy`;
- `pinned_commit`;
- `dirty_state`;
- `hydration_state`;
- `safe_mutation_scope`;
- `honesty_labels`;
- `offered_affordances`; and
- body/metadata embedding state for object availability.

Packet producers use the `export_support_requirements` block in both
schemas. A support packet that cannot embed content still exports the
availability label and reference posture so a triager can distinguish:

- content absent from the repository;
- content outside the active slice;
- content known but not fetched;
- content blocked by shallow history;
- content blocked by submodule initialization;
- content blocked by LFS hydration; and
- content denied because the caller selected the wrong root.

## Fixture Coverage

The repository-topology fixture set covers:

- sparse/workset omitted paths with explicit widen affordance;
- partial-clone promisor objects with fetch affordance;
- shallow-history blame boundaries with deepen affordance;
- uninitialized submodules with pinned commit and init affordance;
- nested independent repository wrong-root mutation denial; and
- Git LFS pointer-only availability with hydrate affordance.

Removing one of those classes without a replacement fixture is a
breaking contract change.
