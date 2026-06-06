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
