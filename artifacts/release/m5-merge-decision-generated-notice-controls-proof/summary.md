# Merge Decision Rows & Generated-Artifact Notices

- Packet: `merge-generated-controls:stable:0001`
- Surface: `Merge decision rows and generated-artifact notices`
- Merge decision rows: 5 (4 non-ordinary conflict classes)
- Generated-artifact notices: 3 (2 stale or diverged)
- Proof freshness SLO: 168 hours (last refresh: 2026-07-08T00:00:00Z)

## Merge decision rows

- **src/main.rs:fn run** [`artifact:src/main.rs`]: ordinary_line_merge conflict (both modified) — recommends accept_current
- **generated/api_client.rs** [`artifact:generated/api_client.rs`]: generated_artifact_conflict conflict (both regenerated from divergent sources) — recommends regenerate_from_source
- **package/Cargo.lock:[serde]** [`artifact:package/Cargo.lock`]: lockfile_conflict conflict (both changed pinned version) — recommends regenerate_from_source
- **package/Cargo.toml:[dependencies]** [`artifact:package/Cargo.toml`]: manifest_conflict conflict (both edited dependency set) — recommends manual
- **policy/ownership.yaml:reviewers** [`artifact:policy/ownership.yaml`]: policy_owned_conflict conflict (both edited a policy-owned field) — recommends manual

## Generated-artifact notices

- **generated API client** [`artifact:generated/api_client.rs`]: diverged (v3 (generated 2026-06-01T00:00:00Z)) — from `generated from openapi/spec.yaml`, restriction regenerate_only
- **dependency lockfile** [`artifact:package/Cargo.lock`]: stale (generated 2026-05-20T00:00:00Z) — from `generated from package/Cargo.toml`, restriction regenerate_only
- **generated JSON schema** [`artifact:generated/schema.json`]: up_to_date (v7 (generated 2026-07-01T00:00:00Z)) — from `generated from schema/model.rs`, restriction compare_only
