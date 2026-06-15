# Fixtures: commercial-control-plane matrix

This directory carries the fixture metadata for the frozen commercial-control-plane
matrix.

The canonical matrix is checked in at:

`artifacts/service/m5-commercial-control-plane.json`

Its boundary schema is:

`schemas/service/m5-commercial-control-plane.schema.json`

## Coverage

- The matrix freezes exactly one lane per claimed managed lane — `managed_lane.ai_gateway`,
  `managed_lane.settings_sync`, `managed_lane.companion_relay`,
  `managed_lane.registry_mirror`, `managed_lane.support_ingest`, and
  `managed_lane.managed_workspace`. Together they cover all six service families and all
  six meter families exactly once.
- Each lane names its meter unit, aggregation window, as-of-time requirement, scope owner,
  distinct chargeback scopes, fail posture, forecast confidence, grace-period rights,
  export guarantee, and a non-empty local-safe baseline.
- The ten-token managed-state vocabulary is frozen in full — `signed_in`, `local_only`,
  `reauth_required`, `managed_blocked`, `grace_period`, `seat_removed`, `plan_downgrade`,
  `org_switched`, `forecast_threshold`, and `meter_stale` — each bound to its entitlement
  state, posture origin, marketed-claim cap, and distinctness rule.
- Five consumer bindings — account surface, diagnostics, Help/About, support/admin packet,
  and claim/public-truth automation — each resolve through real lane ids.

## What the corpus proves

- **Local core is never blocked.** Every lane carries a non-empty `local_safe_baseline`
  and every managed-state row asserts `local_safe_guaranteed`. Even when a managed state
  collapses the marketed claim to `local_safe_only`, the baseline stays, so a stale meter
  or rating path never blocks local editing, search, Git, or local automation.
- **The marketed claim narrows automatically.** A lane's `effective_marketed_claim` is
  recomputed from the active managed state's cap. `managed_blocked`, `seat_removed`, and
  `local_only` cap every applicable lane to `local_safe_only`; `grace_period`,
  `reauth_required`, `plan_downgrade`, `org_switched`, `forecast_threshold`, and
  `meter_stale` cap to `managed_narrowed`; `signed_in` imposes no cap. The stored value
  must equal that recomputation or validation fails.
- **A stale meter does not strand managed work as lost.** `meter_stale` narrows to
  `managed_narrowed`, never `local_safe_only`, because the local core is unaffected by a
  stale metering path.
- **The four loss conditions stay distinct.** `seat_removed`, `org_switched`,
  `grace_period`, and `reauth_required` each list the other three in
  `must_not_collapse_with`, so a surface can never draw one generic account error over four
  different conditions.
- **No number without unit, as-of time, and scope owner.** Every lane requires an as-of
  time and names a scope owner; the `forecast_threshold` row forbids rendering a forecast
  under an unauthoritative state.

## Regeneration

The matrix is built and validated by `canonical_stable_commercial_control_plane_matrix`,
which recomputes every lane's effective marketed claim, narrowing reasons, and the
inspection block; any drift between a stored value and the recomputation is a test failure
in `crates/aureline-service/src/m5_commercial_control_plane/tests.rs`. Regenerate the
checked-in artifact deterministically with:

```text
cargo run -p aureline-service --example dump_m5_commercial_control_plane -- canonical \
  > artifacts/service/m5-commercial-control-plane.json
```

The dump example also prints the matrix narrowed by any single active managed state (for
example `-- managed_blocked`, `-- grace_period`, `-- seat_removed`, `-- meter_stale`).
