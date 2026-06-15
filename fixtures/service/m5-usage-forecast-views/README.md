# Fixtures: usage-and-forecast view set

This directory carries the fixture metadata for the frozen usage-and-forecast view set.

The canonical set is checked in at:

`artifacts/service/m5-usage-forecast-views.json`

Its boundary schema is:

`schemas/service/m5-usage-forecast-views.schema.json`

## Coverage

- The set freezes exactly one view per claimed service family — the AI gateway, settings
  sync, the companion relay, the registry/mirror surface, support ingest, and the managed
  workspace — so unlike families never merge into one opaque total.
- Each view names its meter unit, the month-to-date measurement descriptor (unit,
  aggregation window, scope owner, as-of time, freshness, and value presentation), the
  forecast threshold status and a banner that explains what changes next, the CSV/JSON
  export-parity guarantee, the distinct chargeback scopes, the control-plane lane it
  projects, and the non-empty local-safe baseline.
- The six views together exercise the full threshold-status vocabulary — `within_budget`,
  `approaching_threshold`, `threshold_crossed`, `budget_exhausted`, `forecast_unavailable`,
  and `meter_stale_unconfirmed`.
- Five surface bindings — account/usage, service-health diagnostics, Help/About,
  support/admin export, and the release center — each resolve through real view ids.

## What the corpus proves

- **No number crosses the boundary bare.** Every `measurement` sets `carries_raw_number`
  to `false` and `value_presentation` to `month_to_date_bound_to_unit_as_of_scope`, so a
  month-to-date value is never shown without its unit, as-of time, and scope owner.
- **A forecast banner explains what changes next.** Every `forecast_banner` carries a
  non-empty `what_changes_next` sentence recomputed from its threshold status, rather than
  relying on a warning color alone.
- **Every view exports at CSV/JSON parity.** Each `export_parity` asserts `csv`, `json`,
  and `parity_confirmed`, so usage and forecast export the same fields.
- **Unlike service families never merge.** One view per family with its own meter unit, and
  no opaque cross-family total (`no_collapsed_family_total`).
- **A stale meter never blocks the local core.** The `support_ingest` view labels its
  measurement `freshness_stale` under `meter_stale_unconfirmed`, narrows the marketed claim,
  and never collapses to `local_safe_only`; every view keeps a non-empty local-safe baseline.
- **The views project the control plane.** Each view's `lane_ref` resolves to a
  commercial-control-plane lane whose service family, meter family, meter unit, aggregation
  window, scope owner, service ids, and applicable managed states match — proving the usage
  surface is a real consumer of the matrix rather than a parallel spreadsheet.

## Regeneration

The set is built and validated by `canonical_stable_usage_forecast_view_set`, which
recomputes every view's effective claim, forecast banner, and the inspection block; any
drift between a stored value and the recomputation is a test failure in
`crates/aureline-service/src/m5_usage_forecast_views/tests.rs`. Regenerate the checked-in
artifact deterministically with:

```text
cargo run -p aureline-service --example dump_m5_usage_forecast_views -- canonical \
  > artifacts/service/m5-usage-forecast-views.json
```
