# Forward-read, back-read, round-trip, and migration-diff suites for M5 artifact families

The M5 reader/writer compatibility suite proves the durable and semi-durable M5 artifact families the docs treat as stable or beta public contracts are reader/writer compatible across versions, as repeatable fixtures rather than one-time release-note prose.

## Scope

Every durable M5 artifact family the JSON Schema catalog (`artifacts/contracts/m5-json-schema-catalog.json`) publishes is covered: workspace/state, evidence/support, appearance, learning, diagnostic, and replay-oriented families. The suite reuses the catalog's family list and the publication matrix's reader/writer posture rather than re-deriving them.

## What each family suite covers

- forward-read, back-read, and round-trip across a prior and a current version,
- a migration-diff report proving the prior-to-current change is additive-only,
- unknown-field preservation and additive-field tolerance,
- downgrade narrowing for an artifact at an unsupported newer version, and
- a compare-only fallback for families with a compare-only posture.

## Guardrails

A producer-side schema change is never signed off without reader/writer compatibility proof on the prior version, and migration tooling never rewrites a user-owned artifact without backup/compare-first behavior. Compare-only families are a passing, documented state, not a forced write-back.

## Authoritative artifacts

- Suite catalog: `artifacts/contracts/m5-reader-writer-compat-suite.json`
- Fixture corpus: `fixtures/contracts/m5-compat/`
- Migration-diff reports: `artifacts/contracts/m5-migration-diff-reports/`
- Operator report: `artifacts/contracts/m5-reader-writer-compat.md`
- Validator: `tools/validate_m5_reader_writer_compat_suite.py`
