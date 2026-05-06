# Workspace target-materialization and staging fixtures

These fixtures anchor the vocabulary frozen in:

- [`/docs/workspace/materialization_and_staging_policy.md`](../../../docs/workspace/materialization_and_staging_policy.md)

And validate against:

- [`/schemas/workspace/materialization_class.schema.json`](../../../schemas/workspace/materialization_class.schema.json)

**Scope rules**

- Each fixture encodes exactly one record from the schema (either a
  `materialization_class_definition_record` or a
  `target_materialization_disclosure_record`).
- Location disclosure uses redaction-safe labels only. Raw absolute paths
  never appear.
- These are policy examples only. They do not implement file copy,
  archive extraction, or on-disk promotion mechanics.

