# Route-Explorer, Component-Tree, and App-Topology View Fixtures

These fixtures are valid, export-safe app-topology packets that exercise the
downgrade behavior the canonical support export keeps green. Each keeps every
canonical row present, the review and consumer-projection invariants satisfied,
and proof freshness valid — the difference is which node is narrowed and why.
They are regenerated from the canonical builder via
`cargo run -p aureline-templates --example dump_app_topology_views`.

## origin_unknown_blocked.json

The authored dashboard route's origin can no longer be resolved, so its origin
narrows to `origin_unknown`, its freshness to `freshness_unknown`, its derivation
to `derivation_unknown`, and its downgrade banner to `origin_unknown_banner`. The
node is withdrawn from confident display and gains the `origin_unknown` downgrade
trigger. It is labeled and blocked rather than hidden or presented as authored
truth. The generated, managed-zone, heuristic, runtime-only, and unresolved rows
are unchanged.

## derivation_degraded_withheld.json

The authored-in-generated-zone `UserCard` component's derivation could not be
verified, so its derivation narrows to `derivation_degraded`, it gains a
derivation banner, it is withdrawn from confident display, and it gains the
`derivation_degraded` downgrade trigger. A degraded derivation is never presented
as exact structure. The authored, generated, heuristic, runtime-only, and
unresolved rows are unchanged.
