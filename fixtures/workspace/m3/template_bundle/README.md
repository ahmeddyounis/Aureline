# Workspace-template bundle alpha fixtures

These JSON fixtures are the canonical reviewable inputs for the alpha
`workspace_template_bundle_alpha_record` contract. They are consumed by:

- the Rust integration test
  `crates/aureline-workspace/tests/template_bundle_alpha.rs`, which projects
  each bundle and asserts the acceptance states;
- the headless validator `ci/check_template_bundle_alpha.py`, which validates
  every fixture against
  `schemas/workspace/template_bundle.schema.json` and cross-checks the closed
  source / support / runtime / bypass vocabulary;
- the Start Center alpha consumer in
  `crates/aureline-shell/src/start_center/template_bundles/mod.rs`, which
  reads the first-party fixture to render a deterministic bundle row.

Each fixture exercises one acceptance case:

| Fixture | Acceptance case |
| --- | --- |
| `first_party_local_starter.json` | First-party signed local starter; no network egress; minimum bypass. |
| `community_uncertified_starter.json` | Community publisher; signer continuity unreviewed; trust notes required. |
| `managed_cloud_starter.json` | Managed-cloud target; managed-workspace provisioning, broker handle, and bypass continuity preserved. |

Adding a new fixture is additive. Update the validator and integration test
together if you change the closed vocabulary.
