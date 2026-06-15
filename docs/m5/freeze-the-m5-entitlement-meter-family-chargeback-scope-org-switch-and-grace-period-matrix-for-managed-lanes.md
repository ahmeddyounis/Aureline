# Managed-lane entitlement, meter-family, chargeback-scope, org-switch, and grace-period matrix

Reviewer contract for the canonical commercial-control-plane matrix that maps each
claimed managed lane to its service family, meter family, meter unit, aggregation window,
as-of-time requirement, scope owner, chargeback scopes, fail posture, forecast confidence,
grace-period rights, export guarantee, and the local-safe baseline it always preserves. It
also locks the ten-token user-visible managed-state vocabulary. This row is a depth-lane
proof governed by the canonical M5 evidence index
(`docs/m5/certify_the_full_m5_train_narrow_stale_rows_and_publish_the_canonical_evidence_index.md`).

## Canonical artifacts

- Truth packet: `artifacts/service/m5-commercial-control-plane.json`
- Boundary schema: `schemas/service/m5-commercial-control-plane.schema.json`
- Human-readable rendering: `artifacts/m5/freeze-the-m5-entitlement-meter-family-chargeback-scope-org-switch-and-grace-period-matrix-for-managed-lanes.md`
- Overview companion: `docs/service/m5_commercial_control_plane.md`
- Fixture corpus: `fixtures/service/m5-commercial-control-plane/`
- Owning crate module: `crates/aureline-service/src/m5_commercial_control_plane/`

## Reuses the frozen service vocabulary

The matrix reuses the closed vocabularies already frozen in the metering and operating-mode
contracts (`docs/service/metering_and_chargeback_contract.md`,
`docs/service/operating_mode_and_capacity_contract.md`) and the account/seat/exit contract
(`docs/managed/account_seat_plan_and_exit_contract.md`) rather than minting a parallel
synonym set: the service-family and meter-family classes, the meter-unit and
aggregation-window classes, the entitlement-state and forecast-confidence classes, the
fail-posture class, the export-parity class, and the posture-origin class. The
user-visible managed-state vocabulary maps onto those frozen entitlement and posture
classes so the control-plane vocabulary never drifts away from what a surface renders.

## The lanes

One lane per claimed managed lane, covering all six service families and all six meter
families exactly once:

- `managed_lane.ai_gateway` — managed AI broker, tokens, per organization.
- `managed_lane.settings_sync` — managed settings sync, bytes stored, per workspace.
- `managed_lane.companion_relay` — managed relay, participant minutes, per workspace.
- `managed_lane.registry_mirror` — managed registry/mirror, download count, per organization.
- `managed_lane.support_ingest` — managed support ingest, support-bundle count, per tenant.
- `managed_lane.managed_workspace` — managed workspace control plane, workspace hours, per
  organization.

## The managed-state vocabulary

The matrix freezes exactly these ten tokens, each bound to a frozen entitlement state and
posture origin, each carrying the marketed-claim cap it imposes and the distinctness rule
it must honour: `signed_in`, `local_only`, `reauth_required`, `managed_blocked`,
`grace_period`, `seat_removed`, `plan_downgrade`, `org_switched`, `forecast_threshold`,
and `meter_stale`.

## What the matrix proves

- **Local core is never blocked.** Every lane carries a non-empty local-safe baseline and
  every managed-state row guarantees it. A metering or rating path that is stale or
  unavailable narrows the managed action but never opening, editing, saving, searching,
  local Git, or already-authorized local automation. A stale meter (`meter_stale`) narrows
  to `managed_narrowed`, never `local_safe_only`.
- **The marketed claim narrows automatically.** A lane's effective marketed claim is
  recomputed from the active managed state's cap, so a removed seat, an org switch, a
  grace window, a plan downgrade, an exhausted forecast, or a managed block narrows the
  marketed claim instead of leaving it an optimistic constant. A stored value that does not
  match the recomputation is a validation failure.
- **Distinct states never collapse.** `seat_removed`, `org_switched`, `grace_period`, and a
  sign-in/`reauth_required` failure each list the others in `must_not_collapse_with`, so a
  surface can never draw one generic account error over four different conditions.
- **No number without unit, as-of time, and scope owner.** Every lane requires an as-of
  time and a scope owner, the chargeback scopes stay distinct (personal, workspace, and
  organization never collapse into one total), and a forecast may not render under an
  unauthoritative state.
- **One packet, many consumers.** Account surfaces, diagnostics, Help/About, support/admin
  packets, and claim/public-truth automation each bind to the matrix and project the lane's
  effective claim — never a stronger one.

## Regeneration

`canonical_stable_commercial_control_plane_matrix` builds the matrix;
`current_stable_commercial_control_plane_matrix` reads and validates the checked-in packet.
Drift between a stored value and the recomputation is a test failure in
`crates/aureline-service/src/m5_commercial_control_plane/tests.rs`. Regenerate the artifact
with `cargo run -p aureline-service --example dump_m5_commercial_control_plane -- canonical`.
