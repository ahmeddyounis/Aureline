# Repository-Topology Truth Stabilization Evidence

## Canonical Outputs

- Implementation: `crates/aureline-git/src/stabilize_repository_topology_truth/mod.rs`
- Schema: `schemas/review/repository-topology.schema.json`
- Review contract: `docs/review/m4/stabilize-repository-topology-truth.md`
- Fixture packet:
  `fixtures/review/m4/stabilize-repository-topology-truth/stable_cross_surface_topology_packet.json`
- Verification:
  `cargo test -p aureline-git --test stabilize_repository_topology_truth`

## Stable Claims

The packet prevents stable rows from collapsing topology caveats into
generic missing-file or no-result states. Search, Git graph, review,
blame, code actions, AI context, run/debug, and support export rows carry
the same descriptor refs, honesty labels, action classes, and explicit
active-versus-authoritative root refs.

Network-bearing repair actions are modeled as approval-bearing actions:
fetch, deepen, submodule initialize, and LFS hydrate. Widen, switch-root,
open-child-root, inspect-generated-lineage, and export-topology are
non-network actions unless a future command definition says otherwise.

## Fixture Coverage

The fixture covers sparse checkout/workset, partial clone/promisor
missing objects, shallow history, parent/child submodule targeting,
nested independent repository wrong-root denial, Git LFS pointer-only and
hydrated states, and generated/vendor exclusions. The support export
retains reconstruction fields while redacting raw paths and object bytes.
