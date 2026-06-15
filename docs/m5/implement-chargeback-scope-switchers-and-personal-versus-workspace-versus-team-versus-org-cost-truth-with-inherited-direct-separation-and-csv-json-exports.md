# Chargeback scope switchers and personal/workspace/team/org cost truth with inherited/direct separation and CSV/JSON exports

Reviewer contract for the canonical chargeback-scope view set: the customer-visible
chargeback surface for each claimed managed lane — the AI gateway, settings sync, the
companion relay, the registry/mirror surface, support ingest, and the managed workspace —
that exposes who owns the cost and whether it is charged directly or inherited from a
broader scope. This row is a depth-lane proof governed by the canonical M5 evidence index
(`docs/m5/certify_the_full_m5_train_narrow_stale_rows_and_publish_the_canonical_evidence_index.md`).

## Canonical artifacts

- Truth packet: `artifacts/service/m5-chargeback-scope-views.json`
- Boundary schema: `schemas/service/m5-chargeback-scope-views.schema.json`
- Human-readable rendering: `artifacts/m5/implement-chargeback-scope-switchers-and-personal-versus-workspace-versus-team-versus-org-cost-truth-with-inherited-direct-separation-and-csv-json-exports.md`
- Overview companion: `docs/service/m5_chargeback_scope_views.md`
- Fixture corpus: `fixtures/service/m5-chargeback-scope-views/`
- Owning crate module: `crates/aureline-service/src/m5_chargeback_scope_views/`

## Projects the frozen control-plane matrix

Each view reuses the closed vocabularies already frozen by the commercial-control-plane
matrix (`docs/service/m5_commercial_control_plane.md`) — the service-family, meter-family,
service-id, meter-unit, aggregation-window, scope-owner, managed-state, and marketed-claim
classes — plus the snapshot-freshness vocabulary from the entitlement summaries and the
value-presentation and export-parity packets from the usage-and-forecast views, rather than
minting a parallel synonym set. The scope-owner vocabulary gains one canonical token, `team`,
so personal, workspace, team, and organization are first-class chargeback scopes. Each
view's `lane_ref` resolves to a control-plane lane, and
`ChargebackScopeViewSet::cross_check_against_control_plane` confirms the view's service
family, meter family, meter unit, aggregation window, service ids, and applicable managed
states match the lane, and that every chargeback scope the lane offers is present among the
view's scopes. The new tokens are only the chargeback-surface vocabulary the matrix did not
carry: the attribution basis (direct or inherited) and the chargeback surfaces.

## The views and the scope switcher

One view per service family, each keeping one cost truth per offered scope:

- `chargeback_scope.ai_gateway` — tokens; personal, workspace, team, organization.
- `chargeback_scope.settings_sync` — bytes stored; personal, workspace, team, organization.
- `chargeback_scope.companion_relay` — participant minutes; workspace, team, organization.
- `chargeback_scope.registry_mirror` — download count; team, organization, tenant.
- `chargeback_scope.support_ingest` — support-bundle count; team, organization, tenant.
- `chargeback_scope.managed_workspace` — workspace hours; workspace, team, organization.

A single scope switcher (`chargeback_scope.switcher`) holds the active scope — organization
in the canonical set — across all six views.

## What the set proves

- **Personal, workspace, team, and org never collapse into one bucket.** Each view keeps at
  least two distinct scopes (`no_collapsed_scope_total`), and the four headline scopes each
  appear in at least one view; the scope vocabulary exercised is personal, workspace, team,
  organization, and tenant.
- **Inherited versus direct attribution is inspectable, not portal-only.** Each
  `ScopeCostTruth` carries a `direct` line (`attribution_basis: direct`) and an `inherited`
  line (`attribution_basis: inherited`). The inherited line names the recomputed parent
  scope it rolls up from in `inherited_from`; the broadest scope in a view is the
  inheritance-chain root and explicitly marks its inherited line
  `suppressed_no_managed_number` rather than implying a hidden zero. An inherited parent that
  drifts from the recomputed chain is a validation failure.
- **No number without unit, as-of time, and scope owner.** Every cost line carries the meter
  unit, aggregation window, scope owner, as-of time, and freshness, with `carries_raw_number`
  always `false`; a shown value binds to `month_to_date_bound_to_unit_as_of_scope`.
- **Export parity across CSV and JSON.** Each `export_parity` asserts `csv`, `json`, and
  `parity_confirmed`; `export_safe_csv` emits one row per view, scope, and attribution basis,
  carrying the same fields, unit, as-of time, scope owner, and inherited/direct separation as
  the JSON packet, so the chargeback explanation lives in the product.
- **Switching scope preserves the truth.** The switcher asserts `preserves_active_scope`,
  `preserves_inherited_direct_separation`, `preserves_owner_identity`, and
  `never_collapses_scopes`; `switch_scope` moves the active scope without dropping a scope, a
  separation, or an owner identity. A switch to a scope the switcher does not offer is a
  validation failure.
- **Local core is never blocked.** Every view keeps a non-empty `local_safe_baseline`, so a
  stale or unavailable metering path narrows the managed chargeback view but never local
  editing, search, Git, or already-authorized local automation. A stale meter labels its
  measurements `freshness_stale`, narrows the marketed claim, and never collapses to
  `local_safe_only`.
- **Loss conditions stay distinct.** A view's `effective_marketed_claim` is recomputed from
  the active managed state's cap, so a removed seat, an org switch, a grace window, and a
  sign-in failure each narrow the marketed chargeback claim with their own typed state and
  recovery cue — never one generic account error.
- **One packet, many surfaces.** The account chargeback surface, service-health diagnostics,
  Help/About, the support/admin export, and the release center each bind to the set and
  project the effective claim — never a stronger one — render the inherited-versus-direct
  separation, and render the local-safe baseline.

## Regeneration

`canonical_stable_chargeback_scope_view_set` builds the set;
`current_stable_chargeback_scope_view_set` reads and validates the checked-in packet. Drift
between a stored value and the recomputation is a test failure in
`crates/aureline-service/src/m5_chargeback_scope_views/tests.rs`. Regenerate the artifact with
`cargo run -p aureline-service --example dump_m5_chargeback_scope_views -- canonical`.
