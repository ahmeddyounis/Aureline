# Service-Health Destination Truth Proof Packet

This proof packet records the stable descriptor contract for About, Help,
service health, CLI/headless output, diagnostics, support export, release notes,
migration notices, issue/report templates, and community handoff.

Canonical sources:

- Schema:
  `schemas/help/service-health-destination.schema.json`
- Descriptor fixture:
  `fixtures/help/m4/finalize-service-health-destination-truth/canonical_descriptor.json`
- Support export projection:
  `fixtures/help/m4/finalize-service-health-destination-truth/support_export_projection.json`
- Validation report:
  `fixtures/help/m4/finalize-service-health-destination-truth/validation_report.json`
- Help doc:
  `docs/help/m4/finalize-service-health-destination-truth.md`
- Typed consumer:
  `aureline_service_health::finalize_service_health_destination_truth`

## What Is Proven

The descriptor publishes one stable `service_contract_state` vocabulary:
`ready`, `degraded`, `local_only`, `stale`, `contract_mismatch`,
`policy_blocked`, and `unavailable`.

It publishes one destination trust-class manifest:
`public`, `official_authenticated`, `community`, `vendor_managed`, and
`local_only`. Each manifest row requires visibility boundary, auth expectation,
data-exit boundary, issue-template support, browser-blocked fallback, offline
fallback, and pre-exit labeling.

Every required surface binding consumes the same descriptor ref, build identity
ref, freshness refs, service-health card refs, and destination refs. No binding
requires sign-in to read build, outage, freshness, or local-continuity facts. No
binding may overclaim live reachability when the descriptor is cached, mirrored,
offline, or stale.

The support export projection remains local-first. It starts as a `local_only`
save-later packet, requires inspection before submit, forbids implicit upload,
and names explicit submit actions for public, official-authenticated, and
vendor-managed paths.

## Drill Coverage

The checked-in descriptor includes drill rows for:

- offline local-only continuity
- mirrored release notes and migration notices
- browser-blocked community handoff
- degraded managed AI
- partial marketplace outage

Each drill requires cached/stale labels, preserved destination classes before
exit, visible local-only continuity, support save-later behavior, and no
implicit upload.

## Re-Verification

```sh
cargo test -p aureline-service-health --test finalize_service_health_destination_truth
cargo run -q -p aureline-service-health --bin aureline_service_health_destination_truth -- validation
```
