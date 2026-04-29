# Search Explainability Panel Cases

Worked records for `schemas/search/search_explainability_panel.schema.json`.

- `partial_index.yaml` shows global search explaining partial indexing,
  source mix, recentness, and unknown counts from the same search panel.
- `hidden_by_filter.yaml` shows hidden-by-filter counts staying separate from
  visible and matching counts.
- `policy_limited.yaml` shows docs search exposing policy-blocked and
  policy-hidden counts without revealing hidden details.
- `provider_limited.yaml` shows graph-backed discovery disclosing an
  unavailable provider overlay while preserving graph fallback truth.
- `recentness_boosted.yaml` shows quick open explaining recentness and
  frequent-use ranking without exporting raw query material.

The fixtures are direct panel records with a `$schema` hint so future validators
can read them as boundary examples.
