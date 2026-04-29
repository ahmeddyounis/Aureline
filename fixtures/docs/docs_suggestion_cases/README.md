# Docs suggestion cases

Worked fixtures for
[`/docs/docs/docs_suggestion_and_validation_budget_policy.md`](../../../docs/docs/docs_suggestion_and_validation_budget_policy.md).

Each fixture is a `docs_suggestion_record` and uses
[`/schemas/docs/docs_suggestion.schema.json`](../../../schemas/docs/docs_suggestion.schema.json).

## Fixtures

- [`broken_link.yaml`](./broken_link.yaml) - proven broken docs link
  with a review diff and current link-check evidence.
- [`renamed_command.yaml`](./renamed_command.yaml) - command docs
  update caused by command descriptor rename evidence.
- [`stale_screenshot.yaml`](./stale_screenshot.yaml) - suspected stale
  screenshot capped at draft-only until the surface capture is
  revalidated.
- [`missing_migration_note.yaml`](./missing_migration_note.yaml) -
  migration-note prompt blocked from apply until compatibility and
  release-owner review.
- [`unverifiable_benchmark_copy.yaml`](./unverifiable_benchmark_copy.yaml)
  - benchmark copy review blocked because the cited benchmark evidence
  is missing or stale.

## Required Axes

Every case includes:

- cited source refs;
- a validation-budget row ref;
- freshness and version-match state;
- stale-detection state;
- draft/apply posture; and
- no-marketing-lift gate state.
