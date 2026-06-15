# Fixtures: entitlement-summary set

This directory carries the fixture metadata for the frozen entitlement-summary set.

The canonical set is checked in at:

`artifacts/service/m5-entitlement-summary.json`

Its boundary schema is:

`schemas/service/m5-entitlement-summary.schema.json`

## Coverage

- The set freezes exactly one summary per managed state in the locked ten-token
  vocabulary — `signed_in`, `local_only`, `reauth_required`, `managed_blocked`,
  `grace_period`, `seat_removed`, `plan_downgrade`, `org_switched`,
  `forecast_threshold`, and `meter_stale`.
- Each summary names the plan tier and friendly plan label, the role, the seat owner
  scope and opaque ref, the org/tenant scope and opaque ref, the entitlement label and
  state, the posture origin and degradation recomputed from the state, the marketed
  claim, an optional quota-snapshot descriptor, and the non-empty local-only
  continuation notes.
- Five surface bindings — account surface, diagnostics, support/admin packet,
  Help/About, and feature entry points — each resolve through real summary ids.

## What the corpus proves

- **Local-use continuation is never implicit.** Every summary carries a non-empty
  `local_only_continuation`, so a surface always says what stays usable locally.
- **Seat loss and expiry degrade to explicit managed-blocked states, never a generic
  sign-in error.** A removed seat is cited to the `seat` posture origin and an expired
  entitlement (`entitlement_expired`) to the `policy` origin — both
  `managed_blocked_explicit` — and both stay distinct from the reauthentication
  (sign-in) family at the `account` origin. The posture origin and degradation are
  recomputed from the managed state, so a forged generic-error summary fails validation.
- **No metered number crosses the boundary bare.** A `quota_snapshot` descriptor carries
  the unit, aggregation window, scope owner, as-of time, and freshness class, with
  `carries_raw_number` always `false`, so the snapshot age is legible without exposing a
  raw spend or quota number. A local-only summary carries no managed snapshot at all.
- **A stale meter never blocks the local core.** The `meter_stale` summary labels its
  snapshot `freshness_stale`, narrows to `managed_narrowed`, and never collapses to
  `local_safe_only`.
- **The friendly plan name never hides the truth.** The plan tier, role, seat owner
  scope, account scope, and snapshot freshness are separate typed fields, so the
  friendly plan label can never stand in for them.

## Regeneration

The set is built and validated by `canonical_stable_entitlement_summary_set`, which
recomputes every summary's effective claim, degradation, and posture origin and the
inspection block; any drift between a stored value and the recomputation is a test
failure in `crates/aureline-service/src/m5_entitlement_summary/tests.rs`. Regenerate the
checked-in artifact deterministically with:

```text
cargo run -p aureline-service --example dump_m5_entitlement_summary \
  > artifacts/service/m5-entitlement-summary.json
```
