# Shared Service-Health Feed

Desktop UI, CLI/headless output, About, Help, diagnostics, support export,
release notes, migration notices, and handoff flows now have one shared
service-health feed contract:

- Schema: `schemas/help/service-health-feed.schema.json`
- Typed contract: `aureline_service_health::service_health_feed`
- Canonical feed fixture:
  `fixtures/help/m4/stabilize-service-health-feed-objects-outage-scope/canonical_feed.json`
- Support-export projection:
  `fixtures/help/m4/stabilize-service-health-feed-objects-outage-scope/support_export_projection.json`

## Contract State Vocabulary

Every participating surface reads the same stable `contract_state` vocabulary:

- `ready`
- `degraded`
- `local_only`
- `stale`
- `contract_mismatch`
- `policy_blocked`
- `unavailable`
- `scheduled`
- `read_only`
- `drain`
- `migration`
- `failover`
- `reconciling`
- `resolved`

The first seven states cover live and degraded availability. The latter seven
preserve planned-window and post-window continuity states that maintenance and
failover surfaces already claim elsewhere.

## Required Item Fields

Every shared feed item carries:

- service family
- boundary class
- contract state
- outage scope
- affected workflows
- unaffected workflows when the outage is partial
- summary copy
- freshness source and label
- last-checked time
- diagnostics actions
- local-only continuity note
- participating surfaces

Cached data, mirrored notices, and offline bundles are explicit source classes.
Those classes never admit a live-reachability claim.

## Scope Rules

Partial outages must not collapse the whole product into a single red banner.
The feed therefore requires unaffected workflows to stay explicit when a single
family is degraded or unavailable. Local-core continuity is always preserved as
copyable/exportable text rather than screenshot-only UI.

## Surface Parity

The current stable codebase projects this feed from three places:

- `aureline_service_health::finalize_service_health_destination_truth` for
  Help/About and public-truth surfaces
- `aureline_shell::service_health::aggregator` for desktop and headless shell
  status
- `aureline_support::finalize_support_center_surfaces_performance_inspector_language_service`
  for diagnostics summaries and support-export parity

Each projection validates against the same shared feed invariants before it is
considered stable.
