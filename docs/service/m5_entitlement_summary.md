# Entitlement summaries

The entitlement-summary set is the canonical, inspectable account-context view for
Aureline's optional managed lanes. Where the commercial-control-plane matrix
(`docs/service/m5_commercial_control_plane.md`) freezes the per-lane contract, this set
renders the view a surface shows a user. It is owned by the `aureline-service` crate
(`crates/aureline-service/src/m5_entitlement_summary/`), checked in at
`artifacts/service/m5-entitlement-summary.json`, and bounded by
`schemas/service/m5-entitlement-summary.schema.json`.

## What it freezes

- **One summary per managed state.** The ten-token vocabulary — `signed_in`,
  `local_only`, `reauth_required`, `managed_blocked`, `grace_period`, `seat_removed`,
  `plan_downgrade`, `org_switched`, `forecast_threshold`, and `meter_stale` — each carry a
  summary that names the plan tier and friendly plan label, the role, the seat owner scope
  and opaque ref, the org/tenant scope and opaque ref, the entitlement label and state, the
  posture origin and degradation, the marketed claim, an optional quota-snapshot descriptor,
  and the non-empty local-only continuation notes.
- **One binding per surface.** The account/seat surface, diagnostics, the support/admin
  packet, Help/About, and the managed-feature entry points each resolve through the
  summaries rather than retyping their state, projecting the effective claim and rendering
  the local-only continuation.

## Invariants

- A summary's `effective_marketed_claim`, `degradation`, and `posture_origin` are recomputed
  from its managed state; the stored value must equal the recomputation or validation fails.
- Every summary carries a non-empty `local_only_continuation`, so the local-use continuation
  is never implicit and a stale or unavailable managed path never blocks local editing,
  search, Git, or local automation.
- Seat loss and expiry degrade to an explicit managed-blocked state, never a generic
  sign-in error. A removed seat is cited to the `seat` posture origin and an expired
  entitlement to the `policy` origin — both `managed_blocked_explicit` — and both stay
  distinct from the reauthentication (sign-in) family at the `account` origin.
- A `quota_snapshot` descriptor carries the unit, aggregation window, scope owner, as-of
  time, and freshness class but never a raw number; the friendly plan label never stands in
  for the plan tier, owner scope, or snapshot freshness, which are separate typed fields.

## How to consume it

Call `current_stable_entitlement_summary_set()` to read and validate the checked-in set;
call `EntitlementSummarySet::summary_for_state(state)` to resolve the summary for a managed
state. The reviewer contract is
`docs/m5/implement-entitlement-summaries-with-plan-seat-owner-role-quota-snapshot-age-and-local-only-continuation-notes-across-m5-managed-surfaces.md`.
