# Template, starter, and prebuild entry disclosure fixtures

This directory contains the deterministic drill corpus for M04-195 — stabilizing
template, starter, and prebuild entry disclosure with side-effect envelopes,
freshness truth, and open-without-starter parity.

## Fixture index

| Fixture | Entry kind | Resulting mode | Freshness | Side effects | Bypass count |
| --- | --- | --- | --- | --- | --- |
| `template_first_party_create_project.json` | template | create_project | fresh | none | 2 |
| `starter_community_create_service.json` | starter | create_service | near_expiry | egress, extensions, container | 2 |
| `prebuild_fresh_resume_live.json` | prebuild | resume_live_workspace | fresh | devcontainer attach | 2 |
| `prebuild_stale_start_snapshot.json` | prebuild | start_from_snapshot | stale | mirror egress, container | 3 |
| `prebuild_clone_fresh.json` | prebuild | clone_fresh | fresh | first-party egress | 2 |
| `template_open_without_starter.json` | template | open_without_starter | fresh | first-party egress, extensions | 2 |
| `starter_failure_partial_apply.json` | starter | create_project | fresh | community egress, extensions | 2 |
| `prebuild_managed_cloud.json` | prebuild | open_prebuild_with_setup_actions | near_expiry | managed workspace, credentials | 2 |
| `template_create_empty.json` | template | create_empty_workspace | fresh | none | 2 |
| `prebuild_expired_open_minimal.json` | prebuild | open_prebuild_minimal | expired | none (expired) | 2 |

## Regenerating

```sh
cargo run -q -p aureline-shell \
  --bin aureline_shell_stabilize_template_starter_prebuild_entry -- emit-fixtures \
  fixtures/ux/m4/stabilize-template-starter-prebuild-entry
```

The fixture-replay test fails if any checked-in fixture drifts from the in-code
corpus.
