# Skew and drift smoke report

Reviewer-facing landing page for the unattended skew / drift smoke
lane. The lane replays
[`fixtures/skew/m1_channel_and_schema_cases.yaml`](../../fixtures/skew/m1_channel_and_schema_cases.yaml)
against the canonical qualification matrix, version-skew register,
skew-windows declaration, install-topology matrix, state-root map,
and dogfood matrix, then emits a durable JSON capture under
`artifacts/milestones/m1/captures/`.

The lane proves that side-by-side channel coexistence, state / schema
migration, helper / agent attach drift, unknown-window probe holds,
and rollback to a prior coordinated artifact set all surface as
explicit, vocabulary-bounded truth states instead of partial silent
failures.

## How to run

```bash
python3 tests/smoke/skew_drift/run_skew_drift_smoke.py --repo-root .
```

The runner reads the matrix above, joins it against the canonical
sources, and exits non-zero on any check failure. The deterministic
JSON capture lands at
`artifacts/milestones/m1/captures/skew_drift_smoke_validation_capture.json`.

To force a named failure drill on a specific row:

```bash
python3 tests/smoke/skew_drift/run_skew_drift_smoke.py \
    --repo-root . \
    --force-drill <smoke_row_id>:<drill_id>
```

The runner exits `0` only when the row's `failure_drill.expected_check_id`
is reproduced.

## Smoke rows

| Smoke row | Surface | Skew state | Outcome label | Promotion |
| --- | --- | --- | --- | --- |
| `aureline.compat.smoke.side_by_side_stable_preview_coexist` | `side_by_side_install` | `compatible` | `side_by_side_coexist_ok` | `promote` |
| `aureline.compat.smoke.state_migration_old_to_new_additive` | `state_schema_migration` | `repairable` | `upgrade_applied_compatible` | `ship_narrowed_claim` |
| `aureline.compat.smoke.state_migration_new_to_old_blocked` | `state_schema_migration` | `blocked` | `incompatible_restore` | `no_go` |
| `aureline.compat.smoke.helper_agent_attach_skewed_client_degraded` | `helper_agent_attach` | `degraded` | `attach_degraded_review_only` | `ship_narrowed_claim` |
| `aureline.compat.smoke.helper_agent_attach_unknown_probe_required` | `helper_agent_attach` | `unknown_requires_probe` | `probe_required` | `pending_probe` |
| `aureline.compat.smoke.rollback_prior_channel_build_compatible` | `downgrade_upgrade_rollback` | `compatible` | `rollback_applied` | `promote` |

Required surface-class coverage (`side_by_side_install`,
`state_schema_migration`, `helper_agent_attach`,
`downgrade_upgrade_rollback`) is enforced by the runner.

## Named failure drills

| Drill | Reproduces | Forced input |
| --- | --- | --- |
| `rewrite_side_by_side_state_root_to_cross_channel` | `side_by_side.state_root_owning_channel_collision` | swap Preview state-root id with a Stable state-root id |
| `rewrite_migration_to_silent_downgrade` | `state_migration.silent_fidelity_downgrade` | rewrite `expected_fidelity_label` from `compatible` to `exact` under repairable skew |
| `drop_named_recovery_rung` | `state_migration.recovery_rung_missing` | drop the named recovery rung from a blocked / degraded migration row |
| `rewrite_skew_state_to_compatible_under_degraded_attach` | `helper_agent_attach.skew_state_not_aligned_with_outcome` | rewrite `skew_state_class` to `compatible` under a `review-only` attach outcome |
| `rewrite_outcome_label_to_unknown_vocab` | `skew_drift.outcome_label_unknown_class` / `skew_drift.promotion_decision_unknown_class` | rewrite a vocabulary-bound label to a free-text value |

Each drill is reproducible by the runner against pure data; the lane
fails loudly on real regressions instead of leaving the row silent.

## Source map

| Artifact | Path |
| --- | --- |
| Reviewer landing page (this file) | `artifacts/compat/m1_skew_drift_report.md` |
| Smoke matrix | `fixtures/skew/m1_channel_and_schema_cases.yaml` |
| Unattended runner | `tests/smoke/skew_drift/run_skew_drift_smoke.py` |
| Runner README | `tests/smoke/skew_drift/README.md` |
| Owning proof packet | `artifacts/milestones/m1/proof_packets/skew_drift_smoke.md` |
| Latest capture | `artifacts/milestones/m1/captures/skew_drift_smoke_validation_capture.json` |

The lane is registered in
[`artifacts/milestones/m1/artifact_index.yaml`](../milestones/m1/artifact_index.yaml)
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
