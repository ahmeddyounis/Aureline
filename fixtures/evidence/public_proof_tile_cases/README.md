# Public-proof tile fixtures

Worked fixtures for
[`/docs/ux/public_proof_tile_contract.md`](../../../docs/ux/public_proof_tile_contract.md)
and
[`/schemas/evidence/benchmark_tile.schema.json`](../../../schemas/evidence/benchmark_tile.schema.json).

Each JSON file is one `public_proof_tile_record`. Fixtures use opaque ids
and redaction-aware labels only; raw benchmark traces, logs,
screenshots, private repository names, absolute paths, unrestricted
provider URLs, account identifiers, and credentials stay outside this
boundary.

## Cases

| Fixture | Scenario axis |
| --- | --- |
| [`green_current_benchmark_tile.json`](./green_current_benchmark_tile.json) | Fresh benchmark proof with exact build, bundle, archetype, docs, and packet refs aligned. |
| [`stale_benchmark_tile.json`](./stale_benchmark_tile.json) | Last green benchmark proof downgraded after freshness expiry. |
| [`partial_migration_importer_diff_row.json`](./partial_migration_importer_diff_row.json) | Importer diff row with manual review preserved across export and support handoff. |
| [`unsupported_workflow_migration_handoff_tile.json`](./unsupported_workflow_migration_handoff_tile.json) | Migration handoff tile for an unsupported workflow gap with known-limit refs. |
| [`community_followup_tile.json`](./community_followup_tile.json) | Community-owned issue handoff with public/private boundary and packet preview state. |
