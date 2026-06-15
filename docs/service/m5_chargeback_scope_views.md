# Chargeback scope views

The chargeback-scope view set is the canonical, inspectable chargeback surface for
Aureline's optional managed lanes. Where the commercial-control-plane matrix
(`docs/service/m5_commercial_control_plane.md`) freezes the per-lane entitlement and
metering contract and the usage-and-forecast views (`docs/service/m5_usage_forecast_views.md`)
render the month-to-date number a customer sees, this set answers a different question:
**who owns the cost, and is it charged directly or inherited from a broader scope.** It is
owned by the `aureline-service` crate (`crates/aureline-service/src/m5_chargeback_scope_views/`),
checked in at `artifacts/service/m5-chargeback-scope-views.json`, and bounded by
`schemas/service/m5-chargeback-scope-views.schema.json`.

## What it freezes

- **One view per service family.** The AI gateway, settings sync, the companion relay, the
  registry/mirror surface, support ingest, and the managed workspace each carry a view that
  names the meter unit and aggregation window, the control-plane lane it projects, the
  CSV/JSON export-parity guarantee, and the non-empty local-safe baseline.
- **One cost truth per offered scope.** Each view keeps a `ScopeCostTruth` per chargeback
  scope, so personal, workspace, team, and organization never collapse into one ambiguous
  owner bucket. Every truth carries an opaque owner identity and separates a `direct` cost
  line from an `inherited` one.
- **One scope switcher across the views.** The switcher holds the active scope and asserts
  that a switch preserves the active scope, the inherited-versus-direct separation, and each
  scope's owner identity, and never collapses the scopes.
- **One binding per surface.** The account chargeback surface, service-health diagnostics,
  Help/About, the support/admin export, and the release center each resolve through the
  views, projecting the effective claim, rendering the inherited-versus-direct separation,
  and rendering the local-safe baseline.

## Invariants

- Personal, workspace, team, and organization scopes never collapse into one total: every
  view keeps at least two distinct scopes (`no_collapsed_scope_total`).
- Inherited versus direct is inspectable in the product, not portal-only: each scope truth
  carries a direct line and an inherited line whose `inherited_from` names the recomputed
  parent scope, or whose value is explicitly suppressed at the inheritance-chain root rather
  than implied as a hidden zero.
- No number crosses the boundary bare: every cost line binds its value to the unit, as-of
  time, and scope owner via `value_presentation` and never carries a raw number
  (`carries_raw_number` is always `false`).
- Every view exports at confirmed CSV/JSON parity, so the per-scope direct and inherited
  lines carry the same fields in both formats.
- The local core is never blocked: every view keeps a non-empty local-safe baseline, and a
  stale meter labels its measurements stale, narrows the marketed claim, and never drops a
  scope.
- A view's effective marketed claim is recomputed from the active managed state's cap, so a
  removed seat, an org switch, a grace window, and a sign-in failure each narrow with their
  own typed state and recovery cue — never one generic account error.

## How to consume it

Call `current_stable_chargeback_scope_view_set()` to read and validate the checked-in set;
call `ChargebackScopeViewSet::view_for_family(family)` to resolve the view for a service
family, `ChargebackScopeViewSet::switch_scope(scope)` to move the active scope,
`ChargebackScopeViewSet::apply_managed_state(state)` to exercise narrowing,
`ChargebackScopeViewSet::export_safe_csv()` / `export_safe_json()` for the CSV/JSON exports,
and `ChargebackScopeViewSet::cross_check_against_control_plane()` to confirm each view
projects its control-plane lane. The reviewer contract is
`docs/m5/implement-chargeback-scope-switchers-and-personal-versus-workspace-versus-team-versus-org-cost-truth-with-inherited-direct-separation-and-csv-json-exports.md`.
