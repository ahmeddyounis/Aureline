# Skew and drift smoke

Unattended proof lane that replays the smoke rows in
[`fixtures/skew/m1_channel_and_schema_cases.yaml`](../../../fixtures/skew/m1_channel_and_schema_cases.yaml)
against the canonical qualification matrix, version-skew register,
skew-windows declaration, install-topology matrix, state-root map,
and the dogfood matrix.

The lane is deliberately runnable on CI / nightly without a graphical
display: it only consumes the pure data joined out of the canonical
sources.

## What the lane proves

For every row in the matrix the runner asserts:

- **Compatibility row is real** — the row's `compatibility_row_ref`
  resolves to a row in
  `artifacts/compat/qualification_matrix_seed.yaml`.
- **Skew register case is real** — the row's
  `version_skew_register_ref` resolves to a `skew_case_id` in
  `artifacts/compat/version_skew_register.yaml`.
- **Boundary family is in vocabulary** — `boundary_family` is a row
  in `artifacts/compat/skew_windows.yaml`'s closed vocabulary.
- **Side-by-side channels never collide on state** — Preview rows
  reference Preview-owned state roots and Stable rows reference
  Stable-owned state roots; cross-channel state sharing is a named
  failure (`side_by_side.state_root_owning_channel_collision`).
- **State / schema migration is honest** — repairable skew never
  claims `exact` fidelity; blocked skew always declares the
  `blocked_new_to_old` direction; degraded / blocked migrations always
  point at a concrete recovery rung.
- **Helper / agent attach drift is honest** — a `review-only` attach
  outcome is degraded skew, never compatible
  (`helper_agent_attach.skew_state_not_aligned_with_outcome`).
- **Closed vocabularies are honored** — `surface_class`,
  `skew_state_class`, `outcome_label_class`,
  `promotion_decision_class`, and `boundary_family` are all bounded
  to the matrix vocabulary.
- **Every required surface class is exercised** — `side_by_side_install`,
  `state_schema_migration`, `helper_agent_attach`, and
  `downgrade_upgrade_rollback` each have at least one row.
- **Failure drills are reproducible** — every row names one drill in
  `failure_drill_id_vocabulary` plus the precise `expected_check_id`
  the runner reproduces when the drill is forced with `--force-drill`.

## Run

```bash
python3 tests/smoke/skew_drift/run_skew_drift_smoke.py --repo-root .
```

The runner emits a deterministic JSON capture to
`artifacts/milestones/m1/captures/skew_drift_smoke_validation_capture.json`
and exits non-zero on any check failure.

### Force a named failure drill

```bash
python3 tests/smoke/skew_drift/run_skew_drift_smoke.py \
    --repo-root . \
    --force-drill <smoke_row_id>:<drill_id>
```

In `--force-drill` mode the runner exits `0` only when the row's
declared `expected_check_id` is reproduced by the forced input. Use
this to prove the lane fails loudly on real regressions.

Optional flags:

- `--matrix <path>` — point at an alternate matrix file.
- `--report <path>` — change the capture output path.
- `--build-identity <path>` — change which build identity record is
  embedded in the capture (defaults to
  `artifacts/build/build_identity.json`).

## Where the evidence lives

| Artifact | Path |
| --- | --- |
| Reviewer landing page | `artifacts/compat/m1_skew_drift_report.md` |
| Smoke matrix | `fixtures/skew/m1_channel_and_schema_cases.yaml` |
| Latest capture | `artifacts/milestones/m1/captures/skew_drift_smoke_validation_capture.json` |
| Owning proof packet | `artifacts/milestones/m1/proof_packets/skew_drift_smoke.md` |

The lane is registered in
[`artifacts/milestones/m1/artifact_index.yaml`](../../../artifacts/milestones/m1/artifact_index.yaml)
under `proof_lanes.skew_drift_smoke` so reviewers can find the latest
capture, owner, and validation-lane reference without searching ad
hoc folders.

## Refresh policy

Re-run the lane (and refresh the capture) when any of the following
change:

- `artifacts/compat/qualification_matrix_seed.yaml`
- `artifacts/compat/version_skew_register.yaml`
- `artifacts/compat/skew_windows.yaml`
- `artifacts/release/install_topology_matrix.yaml`
- `artifacts/release/state_root_map.yaml`
- `artifacts/milestones/m1/dogfood_matrix.yaml`
