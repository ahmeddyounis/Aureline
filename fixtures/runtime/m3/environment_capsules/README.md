# Environment-capsule beta fixtures

These fixtures pin the beta capsule resolver contract: devcontainer / Nix /
Compose body parsing, the closed precedence ladder
(devcontainer > compose > nix > node/python), the typed confidence labels
(`imported`, `heuristic`, `unsupported`), and the typed drift evaluator
(`in_sync`, `stale_inputs`, `manually_diverged`, `unknown_lineage`).

The integration test that replays these fixtures lives at
[`/crates/aureline-runtime/tests/capsule_resolver_beta.rs`](../../../../crates/aureline-runtime/tests/capsule_resolver_beta.rs).

| Fixture | What it proves |
| --- | --- |
| `devcontainer_only_case.json` | Single devcontainer.json parses cleanly with imported confidence. |
| `devcontainer_with_compose_case.json` | When devcontainer references a sibling compose file, devcontainer wins precedence and compose is marked overridden_by_higher_precedence rather than silently merged. |
| `compose_only_case.json` | A standalone docker-compose.yml is parsed into the compose source class with imported confidence. |
| `nix_flake_case.json` | A flake.nix is recognised and digested but flagged unsupported because the beta contract does not embed a Nix evaluator. |
| `conflict_devcontainer_nix_compose_case.json` | When all three declarative input families coexist, devcontainer wins precedence and the conflict notes record the override. |
| `empty_workspace_case.json` | Workspaces with no declarative inputs mark capsule lineage unknown and refuse to mint a primary source. |
| `drift_after_edit_case.json` | Editing the devcontainer body in place advances the source-set digest and the drift evaluator returns `stale_inputs`. |
| `source_added_drift_case.json` | Adding a new declarative input to a workspace returns `manually_diverged` with the new source listed under added_sources. |
| `beta_source_coverage.json` | Canonical coverage manifest the runtime emits — round-trips through serde. |

Workspaces under `workspaces/` are checked-in samples consumed verbatim by the
integration test. Editing the bodies will change content digests; rerun the
test suite after any change.
