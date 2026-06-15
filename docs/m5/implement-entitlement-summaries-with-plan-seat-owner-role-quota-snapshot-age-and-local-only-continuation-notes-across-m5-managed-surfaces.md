# Entitlement summaries with plan, seat owner, role, quota snapshot age, and local-only continuation

Reviewer contract for the canonical entitlement-summary set that renders the current
managed state across the M5 managed surfaces: the plan, the role, the seat owner, the
org/tenant scope, the entitlement label, the quota-snapshot age, and the local-only
continuation notes. This row is a depth-lane proof governed by the canonical M5 evidence
index
(`docs/m5/certify_the_full_m5_train_narrow_stale_rows_and_publish_the_canonical_evidence_index.md`).

## Canonical artifacts

- Truth packet: `artifacts/service/m5-entitlement-summary.json`
- Boundary schema: `schemas/service/m5-entitlement-summary.schema.json`
- Human-readable rendering: `artifacts/m5/implement-entitlement-summaries-with-plan-seat-owner-role-quota-snapshot-age-and-local-only-continuation-notes-across-m5-managed-surfaces.md`
- Overview companion: `docs/service/m5_entitlement_summary.md`
- Fixture corpus: `fixtures/service/m5-entitlement-summary/`
- Owning crate module: `crates/aureline-service/src/m5_entitlement_summary/`

## Reuses the frozen control-plane vocabulary

The summaries reuse the closed vocabularies already frozen by the commercial-control-plane
matrix (`docs/service/m5_commercial_control_plane.md`) — the managed-state class, the
entitlement-state class, the scope-owner class, the marketed-claim class, the meter-unit and
aggregation-window classes, and the posture-origin class — rather than minting a parallel
synonym set. The new tokens are only the account-context vocabulary the matrix did not carry:
the plan tier, the account role, the snapshot freshness, the degradation kind, and the
summary surface.

## The summaries

One summary per managed state, covering the full ten-token vocabulary:

- `entitlement_summary.signed_in` — team plan, active; full managed claim.
- `entitlement_summary.local_only` — no managed account; no plan, role, or quota snapshot.
- `entitlement_summary.reauth_required` — the sign-in family; narrowed.
- `entitlement_summary.managed_blocked` — expired and held by policy; explicit block.
- `entitlement_summary.grace_period` — grace window; export-before-suspend.
- `entitlement_summary.seat_removed` — seat reclaimed; explicit block, cited to the seat.
- `entitlement_summary.plan_downgrade` — narrowed to the plan floor.
- `entitlement_summary.org_switched` — managed scope rebinding to a new org.
- `entitlement_summary.forecast_threshold` — approaching the budget; narrowed with a warning.
- `entitlement_summary.meter_stale` — metered number labeled stale; narrowed, never blocked.

## What the set proves

- **Local-use continuation is never implicit.** Every summary carries a non-empty
  `local_only_continuation`, so a surface always says what stays usable locally — opening,
  editing, saving, searching, local Git, and already-authorized local automation.
- **Seat loss and expiry degrade to explicit managed-blocked states.** A removed seat is
  cited to the `seat` posture origin and an expired entitlement (`entitlement_expired`) to
  the `policy` origin — both `managed_blocked_explicit` — and both stay distinct from the
  reauthentication (sign-in) family at the `account` origin. The degradation and posture
  origin are recomputed from the managed state, so a forged generic-error summary is a
  validation failure.
- **No number without unit, as-of time, and scope owner.** A `quota_snapshot` descriptor
  carries the meter unit, aggregation window, scope owner, as-of time, and freshness class,
  with `carries_raw_number` always `false`, so the snapshot age is legible without exposing
  a raw spend or quota number. A stale meter (`meter_stale`) labels the snapshot
  `freshness_stale`, narrows to `managed_narrowed`, and never collapses to `local_safe_only`.
- **The friendly plan name never hides the truth.** The plan tier, role, seat owner scope,
  account scope, and snapshot freshness are separate typed fields; the friendly plan label
  can never stand in for the entitlement freshness or owner scope.
- **One packet, many surfaces.** The account/seat surface, diagnostics, the support/admin
  packet, Help/About, and the managed-feature entry points each bind to the set and project
  the effective claim — never a stronger one — and render the local-only continuation.

## Regeneration

`canonical_stable_entitlement_summary_set` builds the set;
`current_stable_entitlement_summary_set` reads and validates the checked-in packet. Drift
between a stored value and the recomputation is a test failure in
`crates/aureline-service/src/m5_entitlement_summary/tests.rs`. Regenerate the artifact with
`cargo run -p aureline-service --example dump_m5_entitlement_summary`.
