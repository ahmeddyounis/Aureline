# Commercial-control-plane matrix

The commercial-control-plane matrix is the canonical, inspectable truth for Aureline's
optional managed lanes. It is owned by the `aureline-service` crate
(`crates/aureline-service/src/m5_commercial_control_plane/`), checked in at
`artifacts/service/m5-commercial-control-plane.json`, and bounded by
`schemas/service/m5-commercial-control-plane.schema.json`.

## What it freezes

- **One lane per claimed managed lane.** The AI gateway, settings sync, the companion
  relay, the registry/mirror surface, support ingest, and the managed workspace each carry
  one lane row that names its service family, meter family, meter unit, aggregation window,
  as-of-time requirement, scope owner, distinct chargeback scopes, fail posture, forecast
  confidence, grace-period rights, export guarantee, the managed-only actions that pause,
  and the non-empty local-safe baseline that always continues.
- **The user-visible managed-state vocabulary.** Ten tokens — `signed_in`, `local_only`,
  `reauth_required`, `managed_blocked`, `grace_period`, `seat_removed`, `plan_downgrade`,
  `org_switched`, `forecast_threshold`, and `meter_stale` — each bound to a frozen
  entitlement state, a posture origin, a marketed-claim cap, and the distinctness rule that
  keeps a seat loss, an org switch, a grace window, and a sign-in failure from collapsing
  into one generic account error.
- **One binding per consumer surface.** Account surfaces, diagnostics, Help/About,
  support/admin packets, and claim/public-truth automation each resolve through the matrix
  rather than retyping its state.

## Invariants

- A lane's effective marketed claim is recomputed from the active managed state's cap; the
  stored value must equal the recomputation. `managed_blocked`, `seat_removed`, and
  `local_only` cap to `local_safe_only`; the remaining narrowing states cap to
  `managed_narrowed`; `signed_in` imposes no cap.
- Every lane keeps a non-empty local-safe baseline and every managed-state row guarantees
  it, so a stale or unavailable metering path narrows a managed action but never the local
  core. `meter_stale` narrows to `managed_narrowed`, never `local_safe_only`.
- Usage is never shown without its unit, as-of time, and scope owner, and chargeback scopes
  stay distinct.

## How to consume it

Call `current_stable_commercial_control_plane_matrix()` to read and validate the checked-in
matrix; call `CommercialControlPlaneMatrix::apply_managed_state(state)` to narrow every
applicable lane for one active managed state. The reviewer contract is
`docs/m5/freeze-the-m5-entitlement-meter-family-chargeback-scope-org-switch-and-grace-period-matrix-for-managed-lanes.md`.
