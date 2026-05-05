# Migration-failure and partial-open fixtures

These fixtures are short, reviewable scenarios that anchor the
feature-scoped migration-failure and partial-open contract frozen in:

- [`/docs/state/feature_scoped_migration_failure_contract.md`](../../../docs/state/feature_scoped_migration_failure_contract.md)
- [`/schemas/state/migration_failure_state.schema.json`](../../../schemas/state/migration_failure_state.schema.json)
- [`/artifacts/state/unknown_field_preservation_rules.yaml`](../../../artifacts/state/unknown_field_preservation_rules.yaml)

Each fixture is one `migration_failure_state_record`. The record is the
typed, export-safe payload that workspace open, Project Doctor, repair
flows, and support exports use to explain:

- which artifact could not be interpreted;
- which unknown-field posture applied;
- whether the workspace remained open (partial-open);
- which feature families were disabled/degraded/read-only/compare-only;
- which next-safe actions are offered.

## Index

| Fixture | Result | Primary point |
|---|---|---|
| [`unknown_fields_preserved_tasks.yaml`](./unknown_fields_preserved_tasks.yaml) | workspace open; tasks runnable | unknown fields preserved on a human-edited tasks file |
| [`unsupported_schema_version_tasks.yaml`](./unsupported_schema_version_tasks.yaml) | partial open; tasks disabled | unsupported schema version disables the dependent feature only |
| [`lossy_downgrade_refused_lockfile.yaml`](./lossy_downgrade_refused_lockfile.yaml) | partial open; extension pinning disabled | generator-owned lockfile refuses unknown fields; no lossy downgrade |
| [`corruption_generated_artifact_compare_only.yaml`](./corruption_generated_artifact_compare_only.yaml) | partial open; compare/read-only | generated structured artifact corruption routes to compare-only/read-only |

