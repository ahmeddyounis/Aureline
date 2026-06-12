# Infrastructure Surface Qualification

This document defines the canonical infrastructure qualification packet that
ties the checked-in source-intelligence, target-context, live-resource,
plan-viewer, provider-overlay, and incident/support parity packets into one
auto-narrowing infrastructure posture source.

The machine-readable schema is
[`/schemas/infra/infrastructure-surface-qualification.schema.json`](../../schemas/infra/infrastructure-surface-qualification.schema.json).
The Rust validation model is in
[`/crates/aureline-infra`](../../crates/aureline-infra/src/infrastructure_surface_qualification/mod.rs).
Fixtures live in
[`/fixtures/infra/infrastructure-surface-qualification`](../../fixtures/infra/infrastructure-surface-qualification).

## Qualification Rule

A claimed infrastructure or incident-adjacent surface family remains
`stable_qualified` only when all of the following are true:

- the row names the exact upstream packet refs that back the claim;
- every required proof class is present and current:
  `relationship_graph`, `target_context`, `live_counterpart`,
  `plan_validation`, `handoff_boundary`, `wrong_target_drill`,
  `stale_live_overlay_drill`, and `export_parity` where the surface requires
  them;
- the displayed posture is derived from proof coverage rather than asserted
  manually;
- missing relationship proof narrows to `file_only`, missing target/live/plan
  proof narrows to `inspect_only`, missing handoff-boundary proof narrows to
  `handoff_only`, and missing export parity blocks the surface;
- docs/help, Help / About, support playbooks, and public-truth consumers all
  cite the same evidence-index entries instead of restating infrastructure
  maturity independently.

## Claimed Surface Families

- **Source intelligence** keeps the infrastructure truth-layer vocabulary and
  object model explicit for Terraform, Kubernetes, devcontainer, CI, and
  policy artifacts.
- **Live counterpart graph** keeps `show live counterpart`, ownership,
  impact, and reopen-safe graph flows bound to the same object and relation ids.
- **Plan and validation** keeps plan, diff, dry-run, admission, and policy
  viewers labeled as planned truth with tool identity and target context.
- **Live resource context** keeps target strips, separate truth modes, and
  wrong-target or stale-live drills explicit on live inspection surfaces.
- **Provider overlay handoff** keeps provider-owned overlays explicit and
  preserves vendor-console handoff reason, target identity, and return anchor.
- **Incident/support parity** keeps incident workspace and support export
  reopen parity tied to the same graph slice and handoff lineage.
- **Public evidence index** is the canonical row set that downstream docs/help,
  support, and public-truth consumers reuse directly.

## Fixture Meaning

- `qualified_surface_packet.json` mirrors the checked support-export packet and
  proves all seven claimed surface families remain fully qualified.
- `missing_relationship_proof_packet.json` intentionally fails validation by
  removing relationship proof from the source-intelligence row without letting
  the displayed posture narrow.
- `stale_public_index_packet.json` intentionally fails validation by marking
  the public evidence-index row stale without narrowing its displayed posture.
- `missing_consumer_binding_packet.json` intentionally fails validation by
  dropping one required downstream consumer from the shared evidence index.

## Canonical Consumer Refs

The shared evidence index is cited directly by:

- [`/docs/help/infrastructure-surface-qualification.md`](../help/infrastructure-surface-qualification.md)
- [`/docs/help/help_about_truth_source.md`](../help/help_about_truth_source.md)
- [`/artifacts/infra/infrastructure-surface-qualification/support_export.json`](../../artifacts/infra/infrastructure-surface-qualification/support_export.json)

These refs intentionally share the same row set and narrowing reasons so a
surface cannot inherit a greener infrastructure claim from adjacent copy.
