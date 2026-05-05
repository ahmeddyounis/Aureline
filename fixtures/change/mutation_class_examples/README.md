# Mutation-class example fixtures

This directory contains short, worked examples that exercise the
mutation-class matrix and reversal-policy contract.

Source of truth:

- `docs/change/mutation_class_matrix.md`
- `artifacts/change/mutation_classes.yaml`

These fixtures are intentionally small and descriptive rather than full
`mutation_journal_entry` payloads. They exist so reviewers (and future
automation) can compare mutation families mechanically across editor,
refactor, AI, generated-state, and external-effect flows.

All fixture files share the same top-level shape:

- `record_kind: mutation_class_example`
- `mutation_class_examples_schema_version: 1`
- `example_id` (stable id)
- `mutation_class` (one of the closed vocabulary ids)
- `scenario_summary`
- `expected` (preview / approval / journal / reversal expectations)

