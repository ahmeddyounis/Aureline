# Usage and forecast views

The usage-and-forecast view set is the canonical, inspectable customer-visible usage
surface for Aureline's optional managed lanes. Where the commercial-control-plane matrix
(`docs/service/m5_commercial_control_plane.md`) freezes the per-lane entitlement and
metering contract and the entitlement summaries (`docs/service/m5_entitlement_summary.md`)
render the account-context view, this set renders the usage and forecast surface a customer
sees for each lane. It is owned by the `aureline-service` crate
(`crates/aureline-service/src/m5_usage_forecast_views/`), checked in at
`artifacts/service/m5-usage-forecast-views.json`, and bounded by
`schemas/service/m5-usage-forecast-views.schema.json`.

## What it freezes

- **One view per service family.** The AI gateway, settings sync, the companion relay, the
  registry/mirror surface, support ingest, and the managed workspace each carry a view that
  names the meter unit, the month-to-date measurement descriptor (unit, aggregation window,
  scope owner, as-of time, freshness, and value presentation), the forecast threshold status
  and a banner that explains what changes next, the CSV/JSON export-parity guarantee, the
  distinct chargeback scopes, the control-plane lane it projects, and the non-empty
  local-safe baseline.
- **One binding per surface.** The account/usage surface, service-health diagnostics,
  Help/About, the support/admin export, and the release center each resolve through the
  views rather than retyping their state, projecting the effective claim, rendering the
  local-safe baseline, and explaining what changes next.

## Invariants

- No number crosses the boundary bare: every measurement binds its month-to-date value to
  the unit, as-of time, and scope owner via `value_presentation` and never carries a raw
  number (`carries_raw_number` is always `false`).
- A forecast banner explains what changes next: each banner is recomputed from its threshold
  status and carries a non-empty `what_changes_next` sentence; a banner that drifts from its
  status fails validation.
- Every view exports at confirmed CSV/JSON parity, so usage and forecast export the same
  fields in both formats.
- Unlike service families never merge: one view per family with its own meter unit and no
  opaque cross-family total.
- The local core is never blocked: every view keeps a non-empty local-safe baseline, and a
  stale meter labels its measurement stale, narrows the marketed claim, and never collapses
  to local-safe-only.
- A view's effective marketed claim is recomputed from the active managed state's cap, so a
  removed seat, an org switch, a grace window, and a sign-in failure each narrow with their
  own typed state and recovery cue — never one generic account error.

## How to consume it

Call `current_stable_usage_forecast_view_set()` to read and validate the checked-in set;
call `UsageForecastViewSet::view_for_family(family)` to resolve the view for a service
family, `UsageForecastViewSet::apply_managed_state(state)` to exercise narrowing, and
`UsageForecastViewSet::cross_check_against_control_plane()` to confirm each view projects
its control-plane lane. The reviewer contract is
`docs/m5/ship-usage-and-forecast-views-with-meter-units-as-of-time-owner-scope-threshold-banners-and-export-parity-for-ai-sync-relay-registry-and-workspace-services.md`.
