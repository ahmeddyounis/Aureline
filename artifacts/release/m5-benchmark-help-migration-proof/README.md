# M5 Benchmark / Help / Migration Component Proof

Record kind: `m5_benchmark_help_migration_component_proof`

This proof packet binds the reusable component families from
`artifacts/design/m5-benchmark-help-migration-component-matrix.md` to their
schemas and first-consumer fixtures. Release review uses this packet to confirm
that benchmark evidence, About/service-health, support-package, importer-diff,
and community-handoff surfaces do not clone local field lists or private status
wording.

## Packet Members

- Matrix: `artifacts/design/m5-benchmark-help-migration-component-matrix.md`
- Benchmark evidence schema: `schemas/ui/m5-benchmark-evidence-card.schema.json`
- About/service-health schema: `schemas/ui/m5-about-service-health-card.schema.json`
- Support package schema: `schemas/ui/m5-support-package-card.schema.json`
- Importer diff schema: `schemas/ui/m5-importer-diff-row.schema.json`
- Community handoff schema: `schemas/ui/m5-community-handoff-tile.schema.json`
- Fixtures: `fixtures/ui/m5-benchmark-help-migration-components/`
- Proof packet: `artifacts/release/m5-benchmark-help-migration-proof/proof_packet.json`
- Support projection: `artifacts/release/m5-benchmark-help-migration-proof/support_export.json`

## Release Gate

The M5 release gate narrows any first consumer below execution-ready when it:

- drops workflow, budget, corpus/hardware or capture source, freshness, or
  downgrade state from benchmark evidence;
- renders cached service health as live reachability;
- treats a local-only saved support package as submitted support;
- hides bridge-required import rows or checkpoint/restore context; or
- opens a community-owned destination without ownership, visibility, auth,
  commitment, and local fallback labels.

Validate the schemas and fixtures with the command in
`fixtures/ui/m5-benchmark-help-migration-components/README.md`.

