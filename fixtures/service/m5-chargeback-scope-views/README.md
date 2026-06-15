# Fixtures: chargeback-scope view set

This directory carries the fixture metadata for the frozen chargeback-scope view set.

The canonical set is checked in at:

`artifacts/service/m5-chargeback-scope-views.json`

Its boundary schema is:

`schemas/service/m5-chargeback-scope-views.schema.json`

## Coverage

- The set freezes exactly one view per claimed service family — the AI gateway, settings
  sync, the companion relay, the registry/mirror surface, support ingest, and the managed
  workspace — and a single scope switcher across them.
- Each view keeps one cost truth per offered chargeback scope, and personal, workspace,
  team, and organization never collapse into one ambiguous owner total. Across the six
  views the scope vocabulary exercised is personal, workspace, team, organization, and
  tenant.
- Each scope truth separates a **direct** cost line from an **inherited** one. The inherited
  line names the broader parent scope it rolls up from (`inherited_from`); the broadest
  scope in a view is the inheritance-chain root and explicitly marks its inherited line not
  applicable rather than a hidden zero.
- Five surface bindings — account chargeback, service-health diagnostics, Help/About,
  support/admin export, and the release center — each resolve through real view ids.

## What the corpus proves

- **Personal and org never collapse into one bucket.** Every view carries at least two
  distinct scopes (`no_collapsed_scope_total`), and the four headline scopes — personal,
  workspace, team, and organization — each appear in at least one view.
- **Inherited versus direct is inspectable, not portal-only.** Each scope truth carries a
  `direct` line (`attribution_basis: direct`) and an `inherited` line
  (`attribution_basis: inherited`); the inherited line names its recomputed parent scope or
  suppresses the value at the chain root, so the attribution is shown in the product.
- **No number crosses the boundary bare.** Every cost line sets `carries_raw_number` to
  `false`; a shown value is bound to the unit, as-of time, and scope owner via
  `value_presentation`, and the chain-root inherited line is `suppressed_no_managed_number`.
- **CSV and JSON export at parity.** Each `export_parity` asserts `csv`, `json`, and
  `parity_confirmed`; `ChargebackScopeViewSet::export_safe_csv` emits one row per view,
  scope, and attribution basis, carrying the same fields as the JSON packet.
- **Switching scope preserves the truth.** The switcher asserts `preserves_active_scope`,
  `preserves_inherited_direct_separation`, `preserves_owner_identity`, and
  `never_collapses_scopes`; `ChargebackScopeViewSet::switch_scope` moves the active scope
  without dropping a scope, a separation, or an owner identity.
- **A stale meter never blocks the local core.** The `support_ingest` view labels its
  measurements `freshness_stale` and still keeps a non-empty local-safe baseline; narrowing
  states never collapse a chargeback view away from its scopes.
- **Loss conditions stay distinct.** A view's `effective_marketed_claim` is recomputed from
  the active managed state's cap, so a removed seat, an org switch, a grace window, and a
  sign-in failure each narrow with their own typed state and recovery cue — never one
  generic account error.
- **The views project the control plane.** Each view's `lane_ref` resolves to a
  commercial-control-plane lane whose service family, meter family, meter unit, aggregation
  window, service ids, and applicable managed states match, and whose chargeback scope
  offers are all present among the view's scopes — proving the chargeback surface is a real
  consumer of the matrix rather than a parallel spreadsheet.

## Regeneration

The set is built and validated by `canonical_stable_chargeback_scope_view_set`, which
recomputes every view's effective claim, the per-scope inheritance chain, and the inspection
block; any drift between a stored value and the recomputation is a test failure in
`crates/aureline-service/src/m5_chargeback_scope_views/tests.rs`. Regenerate the checked-in
artifact deterministically with:

```text
cargo run -p aureline-service --example dump_m5_chargeback_scope_views -- canonical \
  > artifacts/service/m5-chargeback-scope-views.json
```

The per-scope CSV export can be dumped for review with:

```text
cargo run -p aureline-service --example dump_m5_chargeback_scope_views -- csv
```
