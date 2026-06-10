# Template Registry, Framework-Pack, and Support-Class Matrix Fixtures

These fixtures are valid, export-safe matrix packets that exercise the downgrade
behavior the canonical support export keeps green. Each one keeps every lane
present, trust-review and consumer-projection invariants satisfied, and proof
freshness valid — the difference is which lanes are narrowed and why.

## framework_pack_support_class_narrowed.json

The framework-pack lane is narrowed from Beta to Preview because its support
class dropped below stable; the gallery and run surface show a current
narrowed-support label and downgrade cue (`support_class_narrowed`) rather than
hiding the pack or presenting bridge behavior as exact first-party truth. The
signed template registry, scaffold planner, and archetype health bundle remain
Stable.

## archetype_health_held.json

The archetype-health-bundle lane is held pending upstream archetype-health-matrix
graduation. Held lanes carry no Stable evidence obligation, use the
`not_applicable` evidence requirement and rollback posture, and offer no health
claim while held; the run and recovery surfaces show a held label instead of a
stale pass/fail bit. The signed template registry and scaffold planner remain
Stable and the framework pack remains Beta.
