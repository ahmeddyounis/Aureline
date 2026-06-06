# Repository-Topology Truth Fixtures

`stable_cross_surface_topology_packet.json` is the canonical fixture for
stable cross-surface repository-topology truth. It is parsed by
`crates/aureline-git/tests/stabilize_repository_topology_truth.rs`.

The fixture intentionally keeps path and object data as opaque refs. It
covers sparse/workset omission, partial-clone promisor fetch posture,
shallow-history deepen posture, submodule parent/child targeting, nested
independent repo wrong-root denial, Git LFS pointer-only and hydrated
states, generated/vendor exclusion, and support-export reconstruction.
