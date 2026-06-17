# Repository topology descriptors and first consumers

Large workspaces are frequently sparse, shallow, nested, split into worktrees,
or only partially hydrated. Search, review, blame, AI context, and run/debug
surfaces need explicit topology descriptors instead of ambient assumptions. This
contract makes repository topology a canonical, serde-serializable substrate that
every Git-adjacent surface reuses.

- Schema: [`schemas/git/topology.schema.json`](../../../schemas/git/topology.schema.json)
- Canonical map: [`artifacts/git/m5/git_topology/topology_first_consumers.json`](../../../artifacts/git/m5/git_topology/topology_first_consumers.json)
- Summary: [`artifacts/git/m5/git_topology.md`](../../../artifacts/git/m5/git_topology.md)
- Fixtures: [`fixtures/git/m5/topology-corpus/`](../../../fixtures/git/m5/topology-corpus)
- Code: `crates/aureline-git/src/topology/`

## Explicit descriptor fields

Each `git_topology_root_descriptor` records one repository root with explicit,
structured fields rather than opaque scope references:

| Field | Purpose |
|-------|---------|
| `filter_class` + `omitted_paths` | Checkout filter class and the omitted-path set for sparse/workset slices. |
| `depth_boundary` | History depth class and the commit refs at the shallow/graft boundary. |
| `repo_identity` | Parent/child repo identity: standalone, parent, submodule child (with gitlink and pin), or nested independent. |
| `worktree` | Worktree root (primary or linked) and its shared common directory. |
| `object_availability` | Whether referenced objects are hydrated, promisor-backed, or unfetched. |
| `lfs` | Git LFS object state: not applicable, pointer-only, partially hydrated, or hydrated. |
| `generated_vendor` | Generated/vendor origin and whether the root is editable source truth. |

The descriptors carry only redaction-safe refs; raw paths, raw object bytes, and
credentials never cross the boundary.

## One projection drives every consumer

`TopologyRootDescriptor::project` is the single, deterministic function that maps
a descriptor onto a consumer surface — Git status, review, blame, search scope,
AI context, and support/export. Because each `git_topology_surface_binding` is
*derived* from a descriptor, a local or provider overlay cannot quietly erase a
boundary, and the checked-in map's validation re-derives every binding to prove
the same descriptors drive every surface.

The projection is surface-aware: a sparse slice narrows the path-scoped surfaces
(search, status, review, AI context) but leaves blame complete; a shallow
boundary narrows the history surfaces (blame, AI context) but leaves the working
tree complete. Universal content states — pointer-only, unfetched, an
uninitialized submodule child, and generated/vendor content — narrow every
surface.

How those bindings reach the search-scope, AI-context, and mutation-review
surfaces — and how cross-root bulk mutation stays preview-first and opt-in — is
documented in [topology propagation](topology_propagation.md).

## Invariants

- A pointer-only or unfetched object never resolves to `complete` or
  `full_coverage_allowed`, never permits mutation, and never allows body export.
- Parent and child repo identity stays explicit. Targeting a child while another
  root is active is denied (`wrong_target_root` / `nested_root`,
  `denied_wrong_root`); the scopes never flatten into one bulk mutation.
- Content that cannot be edited (pointer-only, generated/vendor, uninitialized
  submodule child) never advertises a mutating scope.
- The support export retains the structured reconstruction fields after redaction
  and asserts that raw paths and object bytes are redacted.

Regenerate the canonical map, summary, and fixtures with:

```bash
cargo run -p aureline-git --example dump_git_topology_first_consumers \
  > artifacts/git/m5/git_topology/topology_first_consumers.json
cargo run -p aureline-git --example dump_git_topology_first_consumers -- --markdown \
  > artifacts/git/m5/git_topology.md
cargo run -p aureline-git --example dump_git_topology_first_consumers -- submodule \
  > fixtures/git/m5/topology-corpus/submodule_uninitialized_narrowed.json
cargo run -p aureline-git --example dump_git_topology_first_consumers -- lfs \
  > fixtures/git/m5/topology-corpus/lfs_pointer_only.json
```
