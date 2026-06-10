# Framework-Pack Header, Freshness-Chip, and Banner Fixtures

These fixtures are valid, export-safe framework-pack packets that exercise the
downgrade behavior the canonical support export keeps green. Each keeps every
canonical row present, the review and consumer-projection invariants satisfied,
and proof freshness valid — the difference is which row is narrowed and why.
They are regenerated from the canonical builder via
`cargo run -p aureline-templates --example dump_framework_pack_headers`.

## provenance_unknown_blocked.json

The clean first-party pack's provenance can no longer be verified, so its header
narrows to `provenance_unknown`, its freshness to `freshness_unknown`, its
capability to `capability_unknown`, and its downgrade banner to
`provenance_unknown_banner`. The pack is withdrawn from offer and gains the
`provenance_unknown` downgrade trigger. It is labeled and blocked rather than
hidden or presented as first-party truth. The community, bridge, and mirror rows
are unchanged.

## capability_degraded_withheld.json

The community pack's capability could not be verified, so its capability narrows
to `capability_degraded`, it keeps its capability banner, it is withdrawn from
offer, and it gains the `capability_degraded` downgrade trigger. A degraded
capability is never offered as complete. The clean, bridge, and mirror rows are
unchanged.
