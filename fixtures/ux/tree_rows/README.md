# Structural tree row fixtures

Seed corpus for the contract frozen in
[`/docs/ux/tree_row_contract.md`](../../../docs/ux/tree_row_contract.md)
and the schema at
[`/schemas/ux/tree_row.schema.json`](../../../schemas/ux/tree_row.schema.json).

Each fixture is a single JSON document validating as a
`tree_surface_snapshot_record`. The nested rows and placeholders cover
the shared row anatomy, readiness placeholders, hidden-scope
disclosures, and identity/recovery states used by file trees, outlines,
component trees, runtime trees, and support/export projections.

View-level interaction fixtures live in
[`/fixtures/ux/tree_view_cases/`](../tree_view_cases/) and validate as
`tree_view_snapshot_record` against the same schema family.

Every fixture:

- uses only the closed tree surface, node kind, readiness, badge,
  hidden-scope, mutability, selection-sync, match, identity, and
  recovery vocabularies from the contract;
- separates row identity from labels and tree position;
- keeps partial or hidden scope explicit instead of rendering blank
  space as certainty;
- carries recovery actions for moved, missing, generated, cached,
  imported, unsupported, stale, and failed/degraded states; and
- names the contract sections it exercises under
  `__fixture__.contract_sections`.

## Cases

| Fixture | Surface | Scenario axis |
|---|---|---|
| [`file_tree_hot_set_partial_hidden_scope.json`](./file_tree_hot_set_partial_hidden_scope.json) | `file_tree` | Hot-set file discovery remains usable while ignored and outside-loaded scopes are disclosed. |
| [`outline_stale_cached_rebuilding.json`](./outline_stale_cached_rebuilding.json) | `outline_tree` | Cached outline rows remain inspectable while the structural provider rebuilds. |
| [`component_tree_runtime_partial_mapping.json`](./component_tree_runtime_partial_mapping.json) | `component_tree` | Runtime/component rows disclose approximate and runtime-only source mapping before reveal. |
| [`dependency_tree_paused_failed_degraded.json`](./dependency_tree_paused_failed_degraded.json) | `dependency_tree` | Discovering, paused, and failed/degraded placeholders stay distinct while known dependency rows remain visible. |
| [`moved_missing_generated_recovery.json`](./moved_missing_generated_recovery.json) | `file_tree` | Moved, missing, generated, imported, cached, and unsupported nodes keep identity and recovery posture visible. |
