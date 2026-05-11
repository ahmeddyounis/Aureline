# Proof packet: skew and drift smoke lane

Purpose: anchor proof captures for the unattended skew / drift smoke
matrix that proves side-by-side channel coexistence, state / schema
migration direction, helper / agent attach drift, unknown-window
probe holds, and rollback to a prior coordinated artifact set all
stay honest on the claimed compatibility rows.

Reviewer entry point:
[`/artifacts/compat/m1_skew_drift_report.md`](../../compat/m1_skew_drift_report.md).

Canonical sources (non-exhaustive):

- `fixtures/skew/m1_channel_and_schema_cases.yaml` — smoke rows, the
  compatibility row + skew register case each row projects against,
  per-row install-topology / state-root / state-migration / helper-
  agent refs, recovery rung, and named failure drills.
- `tests/smoke/skew_drift/run_skew_drift_smoke.py` — unattended
  runner that replays the matrix and emits the durable JSON capture.
- `artifacts/compat/qualification_matrix_seed.yaml` — claim-bearing
  compatibility rows the smoke rows must resolve against.
- `artifacts/compat/version_skew_register.yaml` — skew register cases
  the smoke rows cite verbatim.
- `artifacts/compat/skew_windows.yaml` — supported windows, upgrade
  order, rollback order, and out-of-window posture per boundary
  family; the lane refuses unknown boundary families.
- `artifacts/release/install_topology_matrix.yaml` — install-profile
  cards the side-by-side and rollback rows must resolve against.
- `artifacts/release/state_root_map.yaml` — durable state-root rows
  with per-channel owning_channels; cross-channel state sharing on a
  side-by-side row is a named failure.
- `artifacts/milestones/m1/dogfood_matrix.yaml` — dogfood rows the
  smoke rows inherit coverage from.

Live runtime consumers (read-only):

- `artifacts/build/build_identity.json` — exact-build identity that
  the capture embeds for cross-artifact traceability.

Validation captures:

- `artifacts/milestones/m1/captures/skew_drift_smoke_validation_capture.json`

Refresh: re-run the validation lane after a change to the
qualification matrix, the version-skew register, the skew-windows
declaration, the install-topology matrix, the state-root map, or the
dogfood matrix.

Closure rule: the lane stays open until the latest capture lands
under the governed proof root and every row reports PASS for
compatibility-row resolution, skew-register case resolution, state-
root channel ownership, additive-vs-blocked migration direction,
helper / agent attach skew-state alignment, closed-vocabulary outcome
and promotion labels, and the named failure drill.
