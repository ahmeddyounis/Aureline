# Finalized Service-Health Destination Truth

About, Help, service health, release notes, migration notices, issue/report
templates, community handoff, CLI/headless output, diagnostics, and support
export now consume one descriptor:

- Schema: `schemas/help/service-health-destination.schema.json`
- Canonical fixture:
  `fixtures/help/m4/finalize-service-health-destination-truth/canonical_descriptor.json`
- Typed consumer:
  `aureline_service_health::finalize_service_health_destination_truth`
- Headless inspector:
  `cargo run -q -p aureline-service-health --bin aureline_service_health_destination_truth -- validation`
- Reusable component matrix:
  `artifacts/design/m5-benchmark-help-migration-component-matrix.md`
- About/service-health card schema:
  `schemas/ui/m5-about-service-health-card.schema.json`

## Contract State Vocabulary

Every service-health card uses the stable `service_contract_state` vocabulary:

- `ready`
- `degraded`
- `local_only`
- `stale`
- `contract_mismatch`
- `policy_blocked`
- `unavailable`

Cards name the service family, boundary class, affected workflows, last-checked
time, freshness label, scoped outage statement, local-only continuity note, and
diagnostics action. A partial hosted outage cannot mark the entire product
unavailable when local editing, local docs, diagnostics, or installed extensions
remain usable.

## Destination Trust Classes

Every handoff destination is labeled before exit with:

- `public`
- `official_authenticated`
- `community`
- `vendor_managed`
- `local_only`

Each class records the visibility boundary, auth expectation, data-exit
boundary, issue-template support, browser-blocked fallback, and offline
fallback. Community destinations are explicitly not official support.
Vendor-managed destinations are outside Aureline governance. Local-only
destinations do not leave the machine.

## Offline Continuity

The descriptor includes drills for offline, mirrored, browser-blocked, degraded
service, and partial-service outage scenarios. Each drill asserts that cached or
stale labels remain visible, destination classes stay accurate before exit,
local-only continuity is shown, support save-later works, and no upload occurs
implicitly.

## Support Export

Support export stays local-first. The saved packet starts as `local_only`, can be
inspected before submit, and can only leave through an explicit submit action.
Cached or offline descriptors never claim live vendor reachability.

## Component Consumption

Help/About summary cards, service-health banners, and service-health status
cards consume the shared About/service-health card fields from the M5 component
matrix rather than cloning local status text. The card must preserve
`service_contract_state`, `source_trust_class`, `freshness_state`,
`local_continuity_state`, and `downgrade_state` into text, JSON, and Markdown
copy/export. A cached or stale service-health source renders
`cached_service_health` or `stale_cache`; it may not be promoted to live
reachability by a first consumer.
