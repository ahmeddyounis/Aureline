# Recommendation-source row fixtures

Seed corpus for the recommendation-source ledger contract in
`docs/workspace/recommendation_source_ledger.md` and the schema at
`schemas/workspace/recommendation_source_row.schema.json`.

Each YAML file is a single `recommendation_source_row_record`. The cases
exercise:

- `archetype_recommendation_source_class` values (detected facts, bundle
  metadata, admin policy, prior user choice, import packet, template
  default, extension contribution, and heuristic inference); and
- `evidence_hook_class` values reserved for later first-useful-work
  instrumentation.

These fixtures intentionally use opaque placeholder refs; they are
schema-shape examples, not detector implementations.

