# Framework Generator-Run Fixtures

These fixtures are valid, export-safe generator-run packets that exercise the
downgrade behavior the canonical support export keeps green. Each keeps every
canonical row present, the review and consumer-projection invariants satisfied,
and proof freshness valid — the difference is which run is narrowed and why. They
are regenerated from the canonical builder via
`cargo run -p aureline-templates --example dump_framework_generators`.

## rollback_unavailable_blocked.json

The exact resource-scaffold run's rollback handle can no longer be captured, so
its rollback narrows to `rollback_unavailable`, its downgrade banner becomes
`rollback_unavailable_banner`, and the run is withdrawn from confident display and
gains the `rollback_unavailable` downgrade trigger. A run is never applied without
a way back: it is labeled and blocked rather than offered. The codemod, migration,
refactor, config, and bridged rows are unchanged.

## context_reuse_unavailable_labeled.json

The exact resource-scaffold run's warm execution context could not be reused, so
its reuse state narrows to `context_reuse_unavailable`, it gains a
`context_reuse_banner`, and it gains the `context_reuse_unavailable` downgrade
trigger. Falling back to a fresh context is honest, not a block — the run is
labeled rather than hidden and stays offered. The codemod, migration, refactor,
config, and bridged rows are unchanged.
