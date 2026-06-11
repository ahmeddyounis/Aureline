# Certified-Archetype Health-Bundle Fixtures

These fixtures are valid, export-safe health-bundle packets that exercise the
downgrade behavior the canonical support export keeps green. Each keeps every
canonical row present, the review and consumer-projection invariants satisfied,
and proof freshness valid — the difference is which bundle is narrowed and why.
They are regenerated from the canonical builder via
`cargo run -p aureline-templates --example dump_archetype_health_bundles`.

## health_unknown_blocked.json

The certified service bundle's health verdict can no longer be determined, so its
health narrows to `health_unknown`, its stack diagnostics to
`diagnostics_unavailable`, its fix-forward path to `fix_unavailable`, its downgrade
banner becomes `health_unknown_banner`, and the bundle is withdrawn from a confident
verdict and gains the `health_undeterminable` and `diagnostics_unavailable` downgrade
triggers. A verdict is never invented: it is labeled and blocked rather than
presented. The web-app, full-stack, CLI, library, and bridged rows are unchanged.

## fix_forward_unavailable_labeled.json

The certified web-app bundle's fix-forward guidance could not be produced, so its
fix state narrows to `fix_unavailable`, it gains a `fix_unavailable_banner`, and it
gains the `fix_guidance_unavailable` downgrade trigger. A missing fix-forward path is
honest, not a block — the bundle is labeled rather than hidden and stays offered. The
service, full-stack, CLI, library, and bridged rows are unchanged.
