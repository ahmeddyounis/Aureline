# Shared Service-Health Feed Support Evidence

This artifact records the stable supportability posture for the shared
service-health feed contract.

## Scope

The feed is the metadata-safe object projected by desktop UI, CLI/headless
status output, Help/About public-truth surfaces, diagnostics-center summaries,
and support/export packets when they describe:

- which service family is affected
- whether the effect is scoped or product-wide
- which workflows are affected or unaffected
- when the state was last checked
- whether the state came from live polling, cached data, mirrored notices, or
  offline bundles
- what local-only continuity remains available

## Stable Checks

- partial-service outages preserve at least one explicitly healthy lane
- cached, mirrored, and offline states keep visible stale labels
- maintenance and failover states remain part of the shared contract instead of
  becoming surface-local prose
- support-export projection carries the same feed fields shown in-product
- no participating surface may overclaim live reachability from non-live data

## Checked-In Outputs

- `schemas/help/service-health-feed.schema.json`
- `fixtures/help/m4/stabilize-service-health-feed-objects-outage-scope/canonical_feed.json`
- `fixtures/help/m4/stabilize-service-health-feed-objects-outage-scope/support_export_projection.json`
- `fixtures/help/m4/stabilize-service-health-feed-objects-outage-scope/validation_report.json`
