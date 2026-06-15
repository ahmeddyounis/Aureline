# Usage and forecast views with meter units, as-of time, owner scope, threshold banners, and export parity

Reviewer contract for the canonical usage-and-forecast view set: the customer-visible
usage surface for each claimed managed lane — the AI gateway, settings sync, the companion
relay, the registry/mirror surface, support ingest, and the managed workspace. This row is
a depth-lane proof governed by the canonical M5 evidence index
(`docs/m5/certify_the_full_m5_train_narrow_stale_rows_and_publish_the_canonical_evidence_index.md`).

## Canonical artifacts

- Truth packet: `artifacts/service/m5-usage-forecast-views.json`
- Boundary schema: `schemas/service/m5-usage-forecast-views.schema.json`
- Human-readable rendering: `artifacts/m5/ship-usage-and-forecast-views-with-meter-units-as-of-time-owner-scope-threshold-banners-and-export-parity-for-ai-sync-relay-registry-and-workspace-services.md`
- Overview companion: `docs/service/m5_usage_forecast_views.md`
- Fixture corpus: `fixtures/service/m5-usage-forecast-views/`
- Owning crate module: `crates/aureline-service/src/m5_usage_forecast_views/`

## Projects the frozen control-plane matrix

Each view reuses the closed vocabularies already frozen by the commercial-control-plane
matrix (`docs/service/m5_commercial_control_plane.md`) — the service-family, meter-family,
service-id, meter-unit, aggregation-window, scope-owner, forecast-confidence,
managed-state, and marketed-claim classes — plus the snapshot-freshness vocabulary from
the entitlement summaries (`docs/service/m5_entitlement_summary.md`), rather than minting a
parallel synonym set. Each view's `lane_ref` resolves to a control-plane lane, and
`UsageForecastViewSet::cross_check_against_control_plane` confirms the view's service
family, meter family, meter unit, aggregation window, scope owner, service ids, and
applicable managed states match the lane. The new tokens are only the usage-surface
vocabulary the matrix did not carry: the threshold status, the banner severity, the value
presentation, and the usage-forecast surface.

## The views

One view per service family:

- `usage_forecast.ai_gateway` — tokens, per organization; forecast approaching the threshold.
- `usage_forecast.settings_sync` — bytes stored, per workspace; within budget.
- `usage_forecast.companion_relay` — participant minutes, per workspace; budget exhausted.
- `usage_forecast.registry_mirror` — download count, per organization; no forecast available.
- `usage_forecast.support_ingest` — support-bundle count, per tenant; meter stale, labeled.
- `usage_forecast.managed_workspace` — workspace hours, per organization; threshold crossed.

## What the set proves

- **No number without unit, as-of time, and scope owner.** Every `measurement` carries the
  meter unit, aggregation window, scope owner, as-of time, and freshness, with
  `value_presentation` bound to `month_to_date_bound_to_unit_as_of_scope` and
  `carries_raw_number` always `false`, so the month-to-date value is legible without
  exposing a raw spend or quota number.
- **A forecast banner explains what changes next.** Each `forecast_banner` is recomputed
  from its threshold status and carries a non-empty `what_changes_next` sentence (for
  example, "at the threshold, new managed-broker work pauses while local editing, search,
  and Git continue") rather than relying on a warning color alone. A stored banner that
  drifts from its status is a validation failure.
- **Export parity across CSV and JSON.** Each `export_parity` asserts `csv`, `json`, and
  `parity_confirmed`, so the usage and forecast export the same fields, unit, as-of time,
  and scope owner in both formats.
- **Unlike service families never merge.** There is one view per family with its own meter
  unit, and the set carries no opaque cross-family total (`no_collapsed_family_total`).
- **Local core is never blocked.** Every view keeps a non-empty `local_safe_baseline`, so a
  stale or unavailable metering path narrows the managed usage view but never local editing,
  search, Git, or already-authorized local automation. A stale meter labels its measurement
  `freshness_stale`, narrows the marketed claim, and never collapses to `local_safe_only`.
- **Loss conditions stay distinct.** A view's `effective_marketed_claim` is recomputed from
  the active managed state's cap, so a removed seat, an org switch, a grace window, and a
  sign-in failure each narrow the marketed usage claim with their own typed state and
  recovery cue — never one generic account error.
- **One packet, many surfaces.** The account/usage surface, service-health diagnostics,
  Help/About, the support/admin export, and the release center each bind to the set and
  project the effective claim — never a stronger one — render the local-safe baseline, and
  explain what changes next.

## Regeneration

`canonical_stable_usage_forecast_view_set` builds the set;
`current_stable_usage_forecast_view_set` reads and validates the checked-in packet. Drift
between a stored value and the recomputation is a test failure in
`crates/aureline-service/src/m5_usage_forecast_views/tests.rs`. Regenerate the artifact with
`cargo run -p aureline-service --example dump_m5_usage_forecast_views -- canonical`.
