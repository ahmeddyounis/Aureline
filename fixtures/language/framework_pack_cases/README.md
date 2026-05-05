# Framework-pack worked fixtures

These YAML fixtures exercise the framework-pack contract frozen in:

- [`/docs/language/framework_pack_contract.md`](../../../docs/language/framework_pack_contract.md)

and the boundary schema at:

- [`/schemas/language/framework_pack_descriptor.schema.json`](../../../schemas/language/framework_pack_descriptor.schema.json)

Each fixture is a single `framework_pack_descriptor_record`.

The corpus uses only opaque ids, typed vocabulary, filesystem-identity records,
and reviewable summaries. No fixture carries raw manifest bodies, raw lockfile
bodies, raw source text, raw build outputs, raw source-map bytes, raw command
lines, raw URLs, or raw secret material.

## Cases

| Fixture | Scenario it freezes |
|---|---|
| `route_config_test_overlays.yaml` | Pack declares route/config/test overlay families and binds them to shared graph/search/build/docs/execution contracts. |
| `mixed_generated_and_authored_source.yaml` | Pack claims a generated-source overlay and commits to generated-artifact lineage and row-level hints for derived artifacts. |
| `package_manager_identity_drift.yaml` | Pack discloses package-manager/workspace-manager identity drift (multiple plausible roots/managers) and fails closed to an explicit review-required posture. |

