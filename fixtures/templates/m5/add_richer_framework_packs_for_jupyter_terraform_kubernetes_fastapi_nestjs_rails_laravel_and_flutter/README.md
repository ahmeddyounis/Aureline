# Richer Framework-Pack Lane Catalog Fixtures

These fixtures are valid, export-safe richer framework-pack lane packets that
exercise the downgrade behavior the canonical support export keeps green. Each
keeps every canonical lane row present, the review and consumer-projection
invariants satisfied, and proof freshness valid — the difference is which row is
narrowed and why. They are regenerated from the canonical builder via
`cargo run -p aureline-templates --example dump_richer_framework_packs`.

## health_degraded_withheld.json

The community Nest pack's archetype health check fails, so its health narrows to
`degraded`, it keeps a downgrade banner, it is withdrawn from offer, and it gains
the `archetype_health_degraded` downgrade trigger. A degraded archetype is never
offered as healthy. The Jupyter, Terraform, Kubernetes, FastAPI, Rails, Laravel,
and Flutter rows are unchanged.

## generator_version_yanked_blocked.json

The first-party Terraform pack's pinned generator version is yanked, so its
freshness narrows to `stale`, its downgrade banner narrows to `freshness_banner`,
it is withdrawn from offer, and it gains the `generator_version_yanked` downgrade
trigger. A pack whose generator version is yanked is never offered as current.
The remaining lane rows are unchanged.
